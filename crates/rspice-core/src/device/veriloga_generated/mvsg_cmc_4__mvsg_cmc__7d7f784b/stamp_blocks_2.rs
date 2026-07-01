#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_32(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign12910_e13527, assign12910_e13527_d_n2, assign12910_e13527_d_n3, assign12910_e13527_d_n4, assign12910_e13527_d_n5, assign12910_e13527_d_n7, assign12910_e13527_d_n14,) = {
    if ((locals.var_guard132 != 0.0) && (locals.var_guard164 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn133_calc_iq__qsout, locals.var_fn133_calc_iq__qsout_dn2, locals.var_fn133_calc_iq__qsout_dn3, locals.var_fn133_calc_iq__qsout_dn4, locals.var_fn133_calc_iq__qsout_dn5, locals.var_fn133_calc_iq__qsout_dn7, locals.var_fn133_calc_iq__qsout_dn14,)
    }
};
        locals.var_fn133_calc_iq__qsout = assign12910_e13527;
        locals.var_fn133_calc_iq__qsout_dn2 = assign12910_e13527_d_n2;
        locals.var_fn133_calc_iq__qsout_dn3 = assign12910_e13527_d_n3;
        locals.var_fn133_calc_iq__qsout_dn4 = assign12910_e13527_d_n4;
        locals.var_fn133_calc_iq__qsout_dn5 = assign12910_e13527_d_n5;
        locals.var_fn133_calc_iq__qsout_dn7 = assign12910_e13527_d_n7;
        locals.var_fn133_calc_iq__qsout_dn14 = assign12910_e13527_d_n14;

        let (assign12940_e13539, assign12940_e13539_d_n2, assign12940_e13539_d_n4, assign12940_e13539_d_n5, assign12940_e13539_d_n7, assign12940_e13539_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        (locals.var_fn133_calc_iq__qgsout, locals.var_fn133_calc_iq__qgsout_dn2, locals.var_fn133_calc_iq__qgsout_dn4, locals.var_fn133_calc_iq__qgsout_dn5, locals.var_fn133_calc_iq__qgsout_dn7, locals.var_fn133_calc_iq__qgsout_dn14,)
    } else {
        (locals.var_qgsfp1, locals.var_qgsfp1_dn2, locals.var_qgsfp1_dn4, locals.var_qgsfp1_dn5, locals.var_qgsfp1_dn7, locals.var_qgsfp1_dn14,)
    }
};
        locals.var_qgsfp1 = assign12940_e13539;
        locals.var_qgsfp1_dn2 = assign12940_e13539_d_n2;
        locals.var_qgsfp1_dn4 = assign12940_e13539_d_n4;
        locals.var_qgsfp1_dn5 = assign12940_e13539_d_n5;
        locals.var_qgsfp1_dn7 = assign12940_e13539_d_n7;
        locals.var_qgsfp1_dn14 = assign12940_e13539_d_n14;

        let (assign12950_e13543, assign12950_e13543_d_n2, assign12950_e13543_d_n4, assign12950_e13543_d_n5, assign12950_e13543_d_n7, assign12950_e13543_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        (locals.var_fn133_calc_iq__qgdout, locals.var_fn133_calc_iq__qgdout_dn2, locals.var_fn133_calc_iq__qgdout_dn4, locals.var_fn133_calc_iq__qgdout_dn5, locals.var_fn133_calc_iq__qgdout_dn7, locals.var_fn133_calc_iq__qgdout_dn14,)
    } else {
        (locals.var_qgdfp1, locals.var_qgdfp1_dn2, locals.var_qgdfp1_dn4, locals.var_qgdfp1_dn5, locals.var_qgdfp1_dn7, locals.var_qgdfp1_dn14,)
    }
};
        locals.var_qgdfp1 = assign12950_e13543;
        locals.var_qgdfp1_dn2 = assign12950_e13543_d_n2;
        locals.var_qgdfp1_dn4 = assign12950_e13543_d_n4;
        locals.var_qgdfp1_dn5 = assign12950_e13543_d_n5;
        locals.var_qgdfp1_dn7 = assign12950_e13543_d_n7;
        locals.var_qgdfp1_dn14 = assign12950_e13543_d_n14;

        let (assign12960_e13547, assign12960_e13547_d_n2, assign12960_e13547_d_n3, assign12960_e13547_d_n4, assign12960_e13547_d_n5, assign12960_e13547_d_n7, assign12960_e13547_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        (locals.var_fn133_calc_iq__qcout, locals.var_fn133_calc_iq__qcout_dn2, locals.var_fn133_calc_iq__qcout_dn3, locals.var_fn133_calc_iq__qcout_dn4, locals.var_fn133_calc_iq__qcout_dn5, locals.var_fn133_calc_iq__qcout_dn7, locals.var_fn133_calc_iq__qcout_dn14,)
    } else {
        (locals.var_qcfp1, locals.var_qcfp1_dn2, locals.var_qcfp1_dn3, locals.var_qcfp1_dn4, locals.var_qcfp1_dn5, locals.var_qcfp1_dn7, locals.var_qcfp1_dn14,)
    }
};
        locals.var_qcfp1 = assign12960_e13547;
        locals.var_qcfp1_dn2 = assign12960_e13547_d_n2;
        locals.var_qcfp1_dn3 = assign12960_e13547_d_n3;
        locals.var_qcfp1_dn4 = assign12960_e13547_d_n4;
        locals.var_qcfp1_dn5 = assign12960_e13547_d_n5;
        locals.var_qcfp1_dn7 = assign12960_e13547_d_n7;
        locals.var_qcfp1_dn14 = assign12960_e13547_d_n14;

        let (assign12970_e13551, assign12970_e13551_d_n2, assign12970_e13551_d_n3, assign12970_e13551_d_n4, assign12970_e13551_d_n5, assign12970_e13551_d_n7, assign12970_e13551_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        (locals.var_fn133_calc_iq__qbout, locals.var_fn133_calc_iq__qbout_dn2, locals.var_fn133_calc_iq__qbout_dn3, locals.var_fn133_calc_iq__qbout_dn4, locals.var_fn133_calc_iq__qbout_dn5, locals.var_fn133_calc_iq__qbout_dn7, locals.var_fn133_calc_iq__qbout_dn14,)
    } else {
        (locals.var_qbfp1, locals.var_qbfp1_dn2, locals.var_qbfp1_dn3, locals.var_qbfp1_dn4, locals.var_qbfp1_dn5, locals.var_qbfp1_dn7, locals.var_qbfp1_dn14,)
    }
};
        locals.var_qbfp1 = assign12970_e13551;
        locals.var_qbfp1_dn2 = assign12970_e13551_d_n2;
        locals.var_qbfp1_dn3 = assign12970_e13551_d_n3;
        locals.var_qbfp1_dn4 = assign12970_e13551_d_n4;
        locals.var_qbfp1_dn5 = assign12970_e13551_d_n5;
        locals.var_qbfp1_dn7 = assign12970_e13551_d_n7;
        locals.var_qbfp1_dn14 = assign12970_e13551_d_n14;

        let (assign12980_e13555, assign12980_e13555_d_n2, assign12980_e13555_d_n3, assign12980_e13555_d_n4, assign12980_e13555_d_n5, assign12980_e13555_d_n7, assign12980_e13555_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        (locals.var_fn133_calc_iq__qsout, locals.var_fn133_calc_iq__qsout_dn2, locals.var_fn133_calc_iq__qsout_dn3, locals.var_fn133_calc_iq__qsout_dn4, locals.var_fn133_calc_iq__qsout_dn5, locals.var_fn133_calc_iq__qsout_dn7, locals.var_fn133_calc_iq__qsout_dn14,)
    } else {
        (locals.var_qsfp1, locals.var_qsfp1_dn2, locals.var_qsfp1_dn3, locals.var_qsfp1_dn4, locals.var_qsfp1_dn5, locals.var_qsfp1_dn7, locals.var_qsfp1_dn14,)
    }
};
        locals.var_qsfp1 = assign12980_e13555;
        locals.var_qsfp1_dn2 = assign12980_e13555_d_n2;
        locals.var_qsfp1_dn3 = assign12980_e13555_d_n3;
        locals.var_qsfp1_dn4 = assign12980_e13555_d_n4;
        locals.var_qsfp1_dn5 = assign12980_e13555_d_n5;
        locals.var_qsfp1_dn7 = assign12980_e13555_d_n7;
        locals.var_qsfp1_dn14 = assign12980_e13555_d_n14;

        let assign13020_e13570: f64 = if p.p166 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard167 = assign13020_e13570;

        locals.var_qgsfps1 = 0.0;
        locals.var_qgsfps1_dn2 = 0.0;
        locals.var_qgsfps1_dn4 = 0.0;
        locals.var_qgsfps1_dn7 = 0.0;
        locals.var_qgsfps1_dn9 = 0.0;
        locals.var_qgsfps1_dn10 = 0.0;

        locals.var_qgdfps1 = 0.0;
        locals.var_qgdfps1_dn2 = 0.0;
        locals.var_qgdfps1_dn4 = 0.0;
        locals.var_qgdfps1_dn7 = 0.0;
        locals.var_qgdfps1_dn9 = 0.0;
        locals.var_qgdfps1_dn10 = 0.0;

        locals.var_qcfps1 = 0.0;
        locals.var_qcfps1_dn2 = 0.0;
        locals.var_qcfps1_dn3 = 0.0;
        locals.var_qcfps1_dn4 = 0.0;
        locals.var_qcfps1_dn7 = 0.0;
        locals.var_qcfps1_dn9 = 0.0;
        locals.var_qcfps1_dn10 = 0.0;

        locals.var_qbfps1 = 0.0;
        locals.var_qbfps1_dn2 = 0.0;
        locals.var_qbfps1_dn3 = 0.0;
        locals.var_qbfps1_dn4 = 0.0;
        locals.var_qbfps1_dn7 = 0.0;
        locals.var_qbfps1_dn9 = 0.0;
        locals.var_qbfps1_dn10 = 0.0;

        locals.var_qsfps1 = 0.0;
        locals.var_qsfps1_dn2 = 0.0;
        locals.var_qsfps1_dn3 = 0.0;
        locals.var_qsfps1_dn4 = 0.0;
        locals.var_qsfps1_dn7 = 0.0;
        locals.var_qsfps1_dn9 = 0.0;
        locals.var_qsfps1_dn10 = 0.0;

        let assign13110_e13581: f64 = if p.p79 > p.p354 { 1.0 } else { 0.0 };
        locals.var_guard168 = assign13110_e13581;

        let (assign13140_e13593, assign13140_e13593_d_n2, assign13140_e13593_d_n4, assign13140_e13593_d_n7, assign13140_e13593_d_n9, assign13140_e13593_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__qgsout, locals.var_fn169_calc_iq__qgsout_dn2, locals.var_fn169_calc_iq__qgsout_dn4, locals.var_fn169_calc_iq__qgsout_dn7, locals.var_fn169_calc_iq__qgsout_dn9, locals.var_fn169_calc_iq__qgsout_dn10,)
    }
};
        locals.var_fn169_calc_iq__qgsout = assign13140_e13593;
        locals.var_fn169_calc_iq__qgsout_dn2 = assign13140_e13593_d_n2;
        locals.var_fn169_calc_iq__qgsout_dn4 = assign13140_e13593_d_n4;
        locals.var_fn169_calc_iq__qgsout_dn7 = assign13140_e13593_d_n7;
        locals.var_fn169_calc_iq__qgsout_dn9 = assign13140_e13593_d_n9;
        locals.var_fn169_calc_iq__qgsout_dn10 = assign13140_e13593_d_n10;

        let (assign13150_e13597, assign13150_e13597_d_n2, assign13150_e13597_d_n4, assign13150_e13597_d_n7, assign13150_e13597_d_n9, assign13150_e13597_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__qgdout, locals.var_fn169_calc_iq__qgdout_dn2, locals.var_fn169_calc_iq__qgdout_dn4, locals.var_fn169_calc_iq__qgdout_dn7, locals.var_fn169_calc_iq__qgdout_dn9, locals.var_fn169_calc_iq__qgdout_dn10,)
    }
};
        locals.var_fn169_calc_iq__qgdout = assign13150_e13597;
        locals.var_fn169_calc_iq__qgdout_dn2 = assign13150_e13597_d_n2;
        locals.var_fn169_calc_iq__qgdout_dn4 = assign13150_e13597_d_n4;
        locals.var_fn169_calc_iq__qgdout_dn7 = assign13150_e13597_d_n7;
        locals.var_fn169_calc_iq__qgdout_dn9 = assign13150_e13597_d_n9;
        locals.var_fn169_calc_iq__qgdout_dn10 = assign13150_e13597_d_n10;

        let (assign13160_e13601, assign13160_e13601_d_n2, assign13160_e13601_d_n3, assign13160_e13601_d_n4, assign13160_e13601_d_n7, assign13160_e13601_d_n9, assign13160_e13601_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__qcout, locals.var_fn169_calc_iq__qcout_dn2, locals.var_fn169_calc_iq__qcout_dn3, locals.var_fn169_calc_iq__qcout_dn4, locals.var_fn169_calc_iq__qcout_dn7, locals.var_fn169_calc_iq__qcout_dn9, locals.var_fn169_calc_iq__qcout_dn10,)
    }
};
        locals.var_fn169_calc_iq__qcout = assign13160_e13601;
        locals.var_fn169_calc_iq__qcout_dn2 = assign13160_e13601_d_n2;
        locals.var_fn169_calc_iq__qcout_dn3 = assign13160_e13601_d_n3;
        locals.var_fn169_calc_iq__qcout_dn4 = assign13160_e13601_d_n4;
        locals.var_fn169_calc_iq__qcout_dn7 = assign13160_e13601_d_n7;
        locals.var_fn169_calc_iq__qcout_dn9 = assign13160_e13601_d_n9;
        locals.var_fn169_calc_iq__qcout_dn10 = assign13160_e13601_d_n10;

        let (assign13170_e13605, assign13170_e13605_d_n2, assign13170_e13605_d_n3, assign13170_e13605_d_n4, assign13170_e13605_d_n7, assign13170_e13605_d_n9, assign13170_e13605_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__qbout, locals.var_fn169_calc_iq__qbout_dn2, locals.var_fn169_calc_iq__qbout_dn3, locals.var_fn169_calc_iq__qbout_dn4, locals.var_fn169_calc_iq__qbout_dn7, locals.var_fn169_calc_iq__qbout_dn9, locals.var_fn169_calc_iq__qbout_dn10,)
    }
};
        locals.var_fn169_calc_iq__qbout = assign13170_e13605;
        locals.var_fn169_calc_iq__qbout_dn2 = assign13170_e13605_d_n2;
        locals.var_fn169_calc_iq__qbout_dn3 = assign13170_e13605_d_n3;
        locals.var_fn169_calc_iq__qbout_dn4 = assign13170_e13605_d_n4;
        locals.var_fn169_calc_iq__qbout_dn7 = assign13170_e13605_d_n7;
        locals.var_fn169_calc_iq__qbout_dn9 = assign13170_e13605_d_n9;
        locals.var_fn169_calc_iq__qbout_dn10 = assign13170_e13605_d_n10;

        let (assign13180_e13609, assign13180_e13609_d_n2, assign13180_e13609_d_n3, assign13180_e13609_d_n4, assign13180_e13609_d_n7, assign13180_e13609_d_n9, assign13180_e13609_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__qsout, locals.var_fn169_calc_iq__qsout_dn2, locals.var_fn169_calc_iq__qsout_dn3, locals.var_fn169_calc_iq__qsout_dn4, locals.var_fn169_calc_iq__qsout_dn7, locals.var_fn169_calc_iq__qsout_dn9, locals.var_fn169_calc_iq__qsout_dn10,)
    }
};
        locals.var_fn169_calc_iq__qsout = assign13180_e13609;
        locals.var_fn169_calc_iq__qsout_dn2 = assign13180_e13609_d_n2;
        locals.var_fn169_calc_iq__qsout_dn3 = assign13180_e13609_d_n3;
        locals.var_fn169_calc_iq__qsout_dn4 = assign13180_e13609_d_n4;
        locals.var_fn169_calc_iq__qsout_dn7 = assign13180_e13609_d_n7;
        locals.var_fn169_calc_iq__qsout_dn9 = assign13180_e13609_d_n9;
        locals.var_fn169_calc_iq__qsout_dn10 = assign13180_e13609_d_n10;

        let (assign13190_e13613, assign13190_e13613_d_n4, assign13190_e13613_d_n9, assign13190_e13613_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__vtdibl, locals.var_fn169_calc_iq__vtdibl_dn4, locals.var_fn169_calc_iq__vtdibl_dn9, locals.var_fn169_calc_iq__vtdibl_dn10,)
    }
};
        locals.var_fn169_calc_iq__vtdibl = assign13190_e13613;
        locals.var_fn169_calc_iq__vtdibl_dn4 = assign13190_e13613_d_n4;
        locals.var_fn169_calc_iq__vtdibl_dn9 = assign13190_e13613_d_n9;
        locals.var_fn169_calc_iq__vtdibl_dn10 = assign13190_e13613_d_n10;

        let (assign13200_e13617, assign13200_e13617_d_n2, assign13200_e13617_d_n3, assign13200_e13617_d_n4, assign13200_e13617_d_n7, assign13200_e13617_d_n9, assign13200_e13617_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__vdsat1, locals.var_fn169_calc_iq__vdsat1_dn2, locals.var_fn169_calc_iq__vdsat1_dn3, locals.var_fn169_calc_iq__vdsat1_dn4, locals.var_fn169_calc_iq__vdsat1_dn7, locals.var_fn169_calc_iq__vdsat1_dn9, locals.var_fn169_calc_iq__vdsat1_dn10,)
    }
};
        locals.var_fn169_calc_iq__vdsat1 = assign13200_e13617;
        locals.var_fn169_calc_iq__vdsat1_dn2 = assign13200_e13617_d_n2;
        locals.var_fn169_calc_iq__vdsat1_dn3 = assign13200_e13617_d_n3;
        locals.var_fn169_calc_iq__vdsat1_dn4 = assign13200_e13617_d_n4;
        locals.var_fn169_calc_iq__vdsat1_dn7 = assign13200_e13617_d_n7;
        locals.var_fn169_calc_iq__vdsat1_dn9 = assign13200_e13617_d_n9;
        locals.var_fn169_calc_iq__vdsat1_dn10 = assign13200_e13617_d_n10;

        let (assign13210_e13621, assign13210_e13621_d_n2, assign13210_e13621_d_n7, assign13210_e13621_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        (locals.var_vgsfps1, locals.var_vgsfps1_dn2, locals.var_vgsfps1_dn7, locals.var_vgsfps1_dn10,)
    } else {
        (locals.var_fn169_calc_iq__vgsin, locals.var_fn169_calc_iq__vgsin_dn2, locals.var_fn169_calc_iq__vgsin_dn7, locals.var_fn169_calc_iq__vgsin_dn10,)
    }
};
        locals.var_fn169_calc_iq__vgsin = assign13210_e13621;
        locals.var_fn169_calc_iq__vgsin_dn2 = assign13210_e13621_d_n2;
        locals.var_fn169_calc_iq__vgsin_dn7 = assign13210_e13621_d_n7;
        locals.var_fn169_calc_iq__vgsin_dn10 = assign13210_e13621_d_n10;

        let (assign13220_e13625, assign13220_e13625_d_n9, assign13220_e13625_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        (locals.var_vdsfps1, locals.var_vdsfps1_dn9, locals.var_vdsfps1_dn10,)
    } else {
        (locals.var_fn169_calc_iq__vdsin, locals.var_fn169_calc_iq__vdsin_dn9, locals.var_fn169_calc_iq__vdsin_dn10,)
    }
};
        locals.var_fn169_calc_iq__vdsin = assign13220_e13625;
        locals.var_fn169_calc_iq__vdsin_dn9 = assign13220_e13625_d_n9;
        locals.var_fn169_calc_iq__vdsin_dn10 = assign13220_e13625_d_n10;

        let (assign13230_e13629,) = {
    if (locals.var_guard168 != 0.0) {
        (p.p85,)
    } else {
        (locals.var_fn169_calc_iq__qcbflag,)
    }
};
        locals.var_fn169_calc_iq__qcbflag = assign13230_e13629;

        let (assign13240_e13633, assign13240_e13633_d_n2, assign13240_e13633_d_n7, assign13240_e13633_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        (locals.var_vcfps1, locals.var_vcfps1_dn2, locals.var_vcfps1_dn7, locals.var_vcfps1_dn10,)
    } else {
        (locals.var_fn169_calc_iq__vcin, locals.var_fn169_calc_iq__vcin_dn2, locals.var_fn169_calc_iq__vcin_dn7, locals.var_fn169_calc_iq__vcin_dn10,)
    }
};
        locals.var_fn169_calc_iq__vcin = assign13240_e13633;
        locals.var_fn169_calc_iq__vcin_dn2 = assign13240_e13633_d_n2;
        locals.var_fn169_calc_iq__vcin_dn7 = assign13240_e13633_d_n7;
        locals.var_fn169_calc_iq__vcin_dn10 = assign13240_e13633_d_n10;

        let (assign13250_e13637, assign13250_e13637_d_n3, assign13250_e13637_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        (locals.var_vbfps1, locals.var_vbfps1_dn3, locals.var_vbfps1_dn10,)
    } else {
        (locals.var_fn169_calc_iq__vbin, locals.var_fn169_calc_iq__vbin_dn3, locals.var_fn169_calc_iq__vbin_dn10,)
    }
};
        locals.var_fn169_calc_iq__vbin = assign13250_e13637;
        locals.var_fn169_calc_iq__vbin_dn3 = assign13250_e13637_d_n3;
        locals.var_fn169_calc_iq__vbin_dn10 = assign13250_e13637_d_n10;

        let (assign13260_e13641,) = {
    if (locals.var_guard168 != 0.0) {
        (p.p83,)
    } else {
        (locals.var_fn169_calc_iq__qgsflag,)
    }
};
        locals.var_fn169_calc_iq__qgsflag = assign13260_e13641;

        let (assign13270_e13645, assign13270_e13645_d_n4,) = {
    if (locals.var_guard168 != 0.0) {
        (locals.var_tdut, locals.var_tdut_dn4,)
    } else {
        (locals.var_fn169_calc_iq__tambin, locals.var_fn169_calc_iq__tambin_dn4,)
    }
};
        locals.var_fn169_calc_iq__tambin = assign13270_e13645;
        locals.var_fn169_calc_iq__tambin_dn4 = assign13270_e13645_d_n4;

        let (assign13280_e13649,) = {
    if (locals.var_guard168 != 0.0) {
        (locals.var_tnomk,)
    } else {
        (locals.var_fn169_calc_iq__tnomin,)
    }
};
        locals.var_fn169_calc_iq__tnomin = assign13280_e13649;

        let (assign13290_e13653, assign13290_e13653_d_n4,) = {
    if (locals.var_guard168 != 0.0) {
        (locals.var_phit, locals.var_phit_dn4,)
    } else {
        (locals.var_fn169_calc_iq__phitin, locals.var_fn169_calc_iq__phitin_dn4,)
    }
};
        locals.var_fn169_calc_iq__phitin = assign13290_e13653;
        locals.var_fn169_calc_iq__phitin_dn4 = assign13290_e13653_d_n4;

        let (assign13300_e13657,) = {
    if (locals.var_guard168 != 0.0) {
        (p.p0,)
    } else {
        (locals.var_fn169_calc_iq__w,)
    }
};
        locals.var_fn169_calc_iq__w = assign13300_e13657;

        let (assign13310_e13661,) = {
    if (locals.var_guard168 != 0.0) {
        (p.p79,)
    } else {
        (locals.var_fn169_calc_iq__lin,)
    }
};
        locals.var_fn169_calc_iq__lin = assign13310_e13661;

        let (assign13320_e13665, assign13320_e13665_d_n4,) = {
    if (locals.var_guard168 != 0.0) {
        (locals.var_cgfps1t, locals.var_cgfps1t_dn4,)
    } else {
        (locals.var_fn169_calc_iq__cgin, locals.var_fn169_calc_iq__cgin_dn4,)
    }
};
        locals.var_fn169_calc_iq__cgin = assign13320_e13665;
        locals.var_fn169_calc_iq__cgin_dn4 = assign13320_e13665_d_n4;

        let (assign13330_e13669,) = {
    if (locals.var_guard168 != 0.0) {
        (p.p84,)
    } else {
        (locals.var_fn169_calc_iq__cs,)
    }
};
        locals.var_fn169_calc_iq__cs = assign13330_e13669;

        let (assign13340_e13673, assign13340_e13673_d_n4,) = {
    if (locals.var_guard168 != 0.0) {
        (locals.var_ccfps1t, locals.var_ccfps1t_dn4,)
    } else {
        (locals.var_fn169_calc_iq__cc, locals.var_fn169_calc_iq__cc_dn4,)
    }
};
        locals.var_fn169_calc_iq__cc = assign13340_e13673;
        locals.var_fn169_calc_iq__cc_dn4 = assign13340_e13673_d_n4;

        let (assign13350_e13677, assign13350_e13677_d_n4,) = {
    if (locals.var_guard168 != 0.0) {
        (locals.var_cbfps1t, locals.var_cbfps1t_dn4,)
    } else {
        (locals.var_fn169_calc_iq__cb, locals.var_fn169_calc_iq__cb_dn4,)
    }
};
        locals.var_fn169_calc_iq__cb = assign13350_e13677;
        locals.var_fn169_calc_iq__cb_dn4 = assign13350_e13677_d_n4;

        let (assign13360_e13681,) = {
    if (locals.var_guard168 != 0.0) {
        (p.p80,)
    } else {
        (locals.var_fn169_calc_iq__vto,)
    }
};
        locals.var_fn169_calc_iq__vto = assign13360_e13681;

        let (assign13370_e13685,) = {
    if (locals.var_guard168 != 0.0) {
        (p.p94,)
    } else {
        (locals.var_fn169_calc_iq__ss,)
    }
};
        locals.var_fn169_calc_iq__ss = assign13370_e13685;

        let (assign13380_e13689,) = {
    if (locals.var_guard168 != 0.0) {
        (p.p93,)
    } else {
        (locals.var_fn169_calc_iq__delta1,)
    }
};
        locals.var_fn169_calc_iq__delta1 = assign13380_e13689;

        let (assign13390_e13693,) = {
    if (locals.var_guard168 != 0.0) {
        (0.0,)
    } else {
        (locals.var_fn169_calc_iq__delta2,)
    }
};
        locals.var_fn169_calc_iq__delta2 = assign13390_e13693;

        let (assign13400_e13697,) = {
    if (locals.var_guard168 != 0.0) {
        (p.p95,)
    } else {
        (locals.var_fn169_calc_iq__nd,)
    }
};
        locals.var_fn169_calc_iq__nd = assign13400_e13697;

        let (assign13410_e13701,) = {
    if (locals.var_guard168 != 0.0) {
        (p.p99,)
    } else {
        (locals.var_fn169_calc_iq__alpha,)
    }
};
        locals.var_fn169_calc_iq__alpha = assign13410_e13701;

        let (assign13420_e13705,) = {
    if (locals.var_guard168 != 0.0) {
        (p.p90,)
    } else {
        (locals.var_fn169_calc_iq__vel0,)
    }
};
        locals.var_fn169_calc_iq__vel0 = assign13420_e13705;

        let (assign13430_e13709,) = {
    if (locals.var_guard168 != 0.0) {
        (p.p91,)
    } else {
        (locals.var_fn169_calc_iq__mu0,)
    }
};
        locals.var_fn169_calc_iq__mu0 = assign13430_e13709;

        let (assign13440_e13713,) = {
    if (locals.var_guard168 != 0.0) {
        (p.p92,)
    } else {
        (locals.var_fn169_calc_iq__beta,)
    }
};
        locals.var_fn169_calc_iq__beta = assign13440_e13713;

        let (assign13450_e13717,) = {
    if (locals.var_guard168 != 0.0) {
        (p.p98,)
    } else {
        (locals.var_fn169_calc_iq__mtheta,)
    }
};
        locals.var_fn169_calc_iq__mtheta = assign13450_e13717;

        let (assign13460_e13721,) = {
    if (locals.var_guard168 != 0.0) {
        (p.p97,)
    } else {
        (locals.var_fn169_calc_iq__vtheta,)
    }
};
        locals.var_fn169_calc_iq__vtheta = assign13460_e13721;

        let (assign13470_e13725,) = {
    if (locals.var_guard168 != 0.0) {
        (p.p96,)
    } else {
        (locals.var_fn169_calc_iq__vtzeta,)
    }
};
        locals.var_fn169_calc_iq__vtzeta = assign13470_e13725;

        let (assign13480_e13729,) = {
    if (locals.var_guard168 != 0.0) {
        (p.p39,)
    } else {
        (locals.var_fn169_calc_iq__dibsat,)
    }
};
        locals.var_fn169_calc_iq__dibsat = assign13480_e13729;

        let (assign13490_e13733,) = {
    if (locals.var_guard168 != 0.0) {
        (p.p47,)
    } else {
        (locals.var_fn169_calc_iq__epsilon,)
    }
};
        locals.var_fn169_calc_iq__epsilon = assign13490_e13733;

    }

    pub(super) fn stamp_transient_block_33(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign13500_e13737,) = {
    if (locals.var_guard168 != 0.0) {
        (p.p45,)
    } else {
        (locals.var_fn169_calc_iq__vzeta,)
    }
};
        locals.var_fn169_calc_iq__vzeta = assign13500_e13737;

        let (assign13510_e13741,) = {
    if (locals.var_guard168 != 0.0) {
        (p.p42,)
    } else {
        (locals.var_fn169_calc_iq__lambda,)
    }
};
        locals.var_fn169_calc_iq__lambda = assign13510_e13741;

        let (assign13520_e13745,) = {
    if (locals.var_guard168 != 0.0) {
        (p.p2,)
    } else {
        (locals.var_fn169_calc_iq__ngf,)
    }
};
        locals.var_fn169_calc_iq__ngf = assign13520_e13745;

        let (assign13530_e13749,) = {
    if (locals.var_guard168 != 0.0) {
        (p.p6,)
    } else {
        (locals.var_fn169_calc_iq__type,)
    }
};
        locals.var_fn169_calc_iq__type = assign13530_e13749;

        let (assign13540_e13753,) = {
    if (locals.var_guard168 != 0.0) {
        (1.0,)
    } else {
        (locals.var_fn169_calc_iq__trapfracdl,)
    }
};
        locals.var_fn169_calc_iq__trapfracdl = assign13540_e13753;

        let (assign13550_e13757, assign13550_e13757_d_n4,) = {
    if (locals.var_guard168 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__alpha_phit, locals.var_fn169_calc_iq__alpha_phit_dn4,)
    }
};
        locals.var_fn169_calc_iq__alpha_phit = assign13550_e13757;
        locals.var_fn169_calc_iq__alpha_phit_dn4 = assign13550_e13757_d_n4;

        let (assign13560_e13761, assign13560_e13761_d_n9, assign13560_e13761_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        (0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__delta, locals.var_fn169_calc_iq__delta_dn9, locals.var_fn169_calc_iq__delta_dn10,)
    }
};
        locals.var_fn169_calc_iq__delta = assign13560_e13761;
        locals.var_fn169_calc_iq__delta_dn9 = assign13560_e13761_d_n9;
        locals.var_fn169_calc_iq__delta_dn10 = assign13560_e13761_d_n10;

        let (assign13570_e13765, assign13570_e13765_d_n4, assign13570_e13765_d_n9, assign13570_e13765_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__n, locals.var_fn169_calc_iq__n_dn4, locals.var_fn169_calc_iq__n_dn9, locals.var_fn169_calc_iq__n_dn10,)
    }
};
        locals.var_fn169_calc_iq__n = assign13570_e13765;
        locals.var_fn169_calc_iq__n_dn4 = assign13570_e13765_d_n4;
        locals.var_fn169_calc_iq__n_dn9 = assign13570_e13765_d_n9;
        locals.var_fn169_calc_iq__n_dn10 = assign13570_e13765_d_n10;

        let (assign13580_e13769, assign13580_e13769_d_n4,) = {
    if (locals.var_guard168 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__vtof, locals.var_fn169_calc_iq__vtof_dn4,)
    }
};
        locals.var_fn169_calc_iq__vtof = assign13580_e13769;
        locals.var_fn169_calc_iq__vtof_dn4 = assign13580_e13769_d_n4;

        let (assign13590_e13773, assign13590_e13773_d_n9, assign13590_e13773_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        (0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__vsatdibl, locals.var_fn169_calc_iq__vsatdibl_dn9, locals.var_fn169_calc_iq__vsatdibl_dn10,)
    }
};
        locals.var_fn169_calc_iq__vsatdibl = assign13590_e13773;
        locals.var_fn169_calc_iq__vsatdibl_dn9 = assign13590_e13773_d_n9;
        locals.var_fn169_calc_iq__vsatdibl_dn10 = assign13590_e13773_d_n10;

        let (assign13600_e13777, assign13600_e13777_d_n2, assign13600_e13777_d_n3, assign13600_e13777_d_n4, assign13600_e13777_d_n7, assign13600_e13777_d_n9, assign13600_e13777_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__ffs, locals.var_fn169_calc_iq__ffs_dn2, locals.var_fn169_calc_iq__ffs_dn3, locals.var_fn169_calc_iq__ffs_dn4, locals.var_fn169_calc_iq__ffs_dn7, locals.var_fn169_calc_iq__ffs_dn9, locals.var_fn169_calc_iq__ffs_dn10,)
    }
};
        locals.var_fn169_calc_iq__ffs = assign13600_e13777;
        locals.var_fn169_calc_iq__ffs_dn2 = assign13600_e13777_d_n2;
        locals.var_fn169_calc_iq__ffs_dn3 = assign13600_e13777_d_n3;
        locals.var_fn169_calc_iq__ffs_dn4 = assign13600_e13777_d_n4;
        locals.var_fn169_calc_iq__ffs_dn7 = assign13600_e13777_d_n7;
        locals.var_fn169_calc_iq__ffs_dn9 = assign13600_e13777_d_n9;
        locals.var_fn169_calc_iq__ffs_dn10 = assign13600_e13777_d_n10;

        let (assign13610_e13781, assign13610_e13781_d_n4, assign13610_e13781_d_n9, assign13610_e13781_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__two_n_phit, locals.var_fn169_calc_iq__two_n_phit_dn4, locals.var_fn169_calc_iq__two_n_phit_dn9, locals.var_fn169_calc_iq__two_n_phit_dn10,)
    }
};
        locals.var_fn169_calc_iq__two_n_phit = assign13610_e13781;
        locals.var_fn169_calc_iq__two_n_phit_dn4 = assign13610_e13781_d_n4;
        locals.var_fn169_calc_iq__two_n_phit_dn9 = assign13610_e13781_d_n9;
        locals.var_fn169_calc_iq__two_n_phit_dn10 = assign13610_e13781_d_n10;

        let (assign13620_e13785, assign13620_e13785_d_n4, assign13620_e13785_d_n9, assign13620_e13785_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__qref, locals.var_fn169_calc_iq__qref_dn4, locals.var_fn169_calc_iq__qref_dn9, locals.var_fn169_calc_iq__qref_dn10,)
    }
};
        locals.var_fn169_calc_iq__qref = assign13620_e13785;
        locals.var_fn169_calc_iq__qref_dn4 = assign13620_e13785_d_n4;
        locals.var_fn169_calc_iq__qref_dn9 = assign13620_e13785_d_n9;
        locals.var_fn169_calc_iq__qref_dn10 = assign13620_e13785_d_n10;

        let (assign13630_e13789, assign13630_e13789_d_n2, assign13630_e13789_d_n3, assign13630_e13789_d_n4, assign13630_e13789_d_n7, assign13630_e13789_d_n9, assign13630_e13789_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__etas, locals.var_fn169_calc_iq__etas_dn2, locals.var_fn169_calc_iq__etas_dn3, locals.var_fn169_calc_iq__etas_dn4, locals.var_fn169_calc_iq__etas_dn7, locals.var_fn169_calc_iq__etas_dn9, locals.var_fn169_calc_iq__etas_dn10,)
    }
};
        locals.var_fn169_calc_iq__etas = assign13630_e13789;
        locals.var_fn169_calc_iq__etas_dn2 = assign13630_e13789_d_n2;
        locals.var_fn169_calc_iq__etas_dn3 = assign13630_e13789_d_n3;
        locals.var_fn169_calc_iq__etas_dn4 = assign13630_e13789_d_n4;
        locals.var_fn169_calc_iq__etas_dn7 = assign13630_e13789_d_n7;
        locals.var_fn169_calc_iq__etas_dn9 = assign13630_e13789_d_n9;
        locals.var_fn169_calc_iq__etas_dn10 = assign13630_e13789_d_n10;

        let (assign13640_e13793, assign13640_e13793_d_n2, assign13640_e13793_d_n3, assign13640_e13793_d_n4, assign13640_e13793_d_n7, assign13640_e13793_d_n9, assign13640_e13793_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__qinvs, locals.var_fn169_calc_iq__qinvs_dn2, locals.var_fn169_calc_iq__qinvs_dn3, locals.var_fn169_calc_iq__qinvs_dn4, locals.var_fn169_calc_iq__qinvs_dn7, locals.var_fn169_calc_iq__qinvs_dn9, locals.var_fn169_calc_iq__qinvs_dn10,)
    }
};
        locals.var_fn169_calc_iq__qinvs = assign13640_e13793;
        locals.var_fn169_calc_iq__qinvs_dn2 = assign13640_e13793_d_n2;
        locals.var_fn169_calc_iq__qinvs_dn3 = assign13640_e13793_d_n3;
        locals.var_fn169_calc_iq__qinvs_dn4 = assign13640_e13793_d_n4;
        locals.var_fn169_calc_iq__qinvs_dn7 = assign13640_e13793_d_n7;
        locals.var_fn169_calc_iq__qinvs_dn9 = assign13640_e13793_d_n9;
        locals.var_fn169_calc_iq__qinvs_dn10 = assign13640_e13793_d_n10;

        let (assign13650_e13797, assign13650_e13797_d_n2, assign13650_e13797_d_n3, assign13650_e13797_d_n4, assign13650_e13797_d_n7, assign13650_e13797_d_n9, assign13650_e13797_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__muf, locals.var_fn169_calc_iq__muf_dn2, locals.var_fn169_calc_iq__muf_dn3, locals.var_fn169_calc_iq__muf_dn4, locals.var_fn169_calc_iq__muf_dn7, locals.var_fn169_calc_iq__muf_dn9, locals.var_fn169_calc_iq__muf_dn10,)
    }
};
        locals.var_fn169_calc_iq__muf = assign13650_e13797;
        locals.var_fn169_calc_iq__muf_dn2 = assign13650_e13797_d_n2;
        locals.var_fn169_calc_iq__muf_dn3 = assign13650_e13797_d_n3;
        locals.var_fn169_calc_iq__muf_dn4 = assign13650_e13797_d_n4;
        locals.var_fn169_calc_iq__muf_dn7 = assign13650_e13797_d_n7;
        locals.var_fn169_calc_iq__muf_dn9 = assign13650_e13797_d_n9;
        locals.var_fn169_calc_iq__muf_dn10 = assign13650_e13797_d_n10;

        let (assign13660_e13801, assign13660_e13801_d_n2, assign13660_e13801_d_n3, assign13660_e13801_d_n4, assign13660_e13801_d_n7, assign13660_e13801_d_n9, assign13660_e13801_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__vx, locals.var_fn169_calc_iq__vx_dn2, locals.var_fn169_calc_iq__vx_dn3, locals.var_fn169_calc_iq__vx_dn4, locals.var_fn169_calc_iq__vx_dn7, locals.var_fn169_calc_iq__vx_dn9, locals.var_fn169_calc_iq__vx_dn10,)
    }
};
        locals.var_fn169_calc_iq__vx = assign13660_e13801;
        locals.var_fn169_calc_iq__vx_dn2 = assign13660_e13801_d_n2;
        locals.var_fn169_calc_iq__vx_dn3 = assign13660_e13801_d_n3;
        locals.var_fn169_calc_iq__vx_dn4 = assign13660_e13801_d_n4;
        locals.var_fn169_calc_iq__vx_dn7 = assign13660_e13801_d_n7;
        locals.var_fn169_calc_iq__vx_dn9 = assign13660_e13801_d_n9;
        locals.var_fn169_calc_iq__vx_dn10 = assign13660_e13801_d_n10;

        let (assign13680_e13809, assign13680_e13809_d_n4,) = {
    if (locals.var_guard168 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__n0, locals.var_fn169_calc_iq__n0_dn4,)
    }
};
        locals.var_fn169_calc_iq__n0 = assign13680_e13809;
        locals.var_fn169_calc_iq__n0_dn4 = assign13680_e13809_d_n4;

        let (assign13690_e13813, assign13690_e13813_d_n2, assign13690_e13813_d_n4, assign13690_e13813_d_n7, assign13690_e13813_d_n9, assign13690_e13813_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__ffs0, locals.var_fn169_calc_iq__ffs0_dn2, locals.var_fn169_calc_iq__ffs0_dn4, locals.var_fn169_calc_iq__ffs0_dn7, locals.var_fn169_calc_iq__ffs0_dn9, locals.var_fn169_calc_iq__ffs0_dn10,)
    }
};
        locals.var_fn169_calc_iq__ffs0 = assign13690_e13813;
        locals.var_fn169_calc_iq__ffs0_dn2 = assign13690_e13813_d_n2;
        locals.var_fn169_calc_iq__ffs0_dn4 = assign13690_e13813_d_n4;
        locals.var_fn169_calc_iq__ffs0_dn7 = assign13690_e13813_d_n7;
        locals.var_fn169_calc_iq__ffs0_dn9 = assign13690_e13813_d_n9;
        locals.var_fn169_calc_iq__ffs0_dn10 = assign13690_e13813_d_n10;

        let (assign13700_e13817, assign13700_e13817_d_n4,) = {
    if (locals.var_guard168 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__two_n_phit0, locals.var_fn169_calc_iq__two_n_phit0_dn4,)
    }
};
        locals.var_fn169_calc_iq__two_n_phit0 = assign13700_e13817;
        locals.var_fn169_calc_iq__two_n_phit0_dn4 = assign13700_e13817_d_n4;

        let (assign13710_e13821, assign13710_e13821_d_n4,) = {
    if (locals.var_guard168 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__qref0, locals.var_fn169_calc_iq__qref0_dn4,)
    }
};
        locals.var_fn169_calc_iq__qref0 = assign13710_e13821;
        locals.var_fn169_calc_iq__qref0_dn4 = assign13710_e13821_d_n4;

        let (assign13720_e13825, assign13720_e13825_d_n2, assign13720_e13825_d_n4, assign13720_e13825_d_n7, assign13720_e13825_d_n9, assign13720_e13825_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__etas0, locals.var_fn169_calc_iq__etas0_dn2, locals.var_fn169_calc_iq__etas0_dn4, locals.var_fn169_calc_iq__etas0_dn7, locals.var_fn169_calc_iq__etas0_dn9, locals.var_fn169_calc_iq__etas0_dn10,)
    }
};
        locals.var_fn169_calc_iq__etas0 = assign13720_e13825;
        locals.var_fn169_calc_iq__etas0_dn2 = assign13720_e13825_d_n2;
        locals.var_fn169_calc_iq__etas0_dn4 = assign13720_e13825_d_n4;
        locals.var_fn169_calc_iq__etas0_dn7 = assign13720_e13825_d_n7;
        locals.var_fn169_calc_iq__etas0_dn9 = assign13720_e13825_d_n9;
        locals.var_fn169_calc_iq__etas0_dn10 = assign13720_e13825_d_n10;

        let (assign13730_e13829, assign13730_e13829_d_n2, assign13730_e13829_d_n4, assign13730_e13829_d_n7, assign13730_e13829_d_n9, assign13730_e13829_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__qinvs0, locals.var_fn169_calc_iq__qinvs0_dn2, locals.var_fn169_calc_iq__qinvs0_dn4, locals.var_fn169_calc_iq__qinvs0_dn7, locals.var_fn169_calc_iq__qinvs0_dn9, locals.var_fn169_calc_iq__qinvs0_dn10,)
    }
};
        locals.var_fn169_calc_iq__qinvs0 = assign13730_e13829;
        locals.var_fn169_calc_iq__qinvs0_dn2 = assign13730_e13829_d_n2;
        locals.var_fn169_calc_iq__qinvs0_dn4 = assign13730_e13829_d_n4;
        locals.var_fn169_calc_iq__qinvs0_dn7 = assign13730_e13829_d_n7;
        locals.var_fn169_calc_iq__qinvs0_dn9 = assign13730_e13829_d_n9;
        locals.var_fn169_calc_iq__qinvs0_dn10 = assign13730_e13829_d_n10;

        let (assign13740_e13833, assign13740_e13833_d_n4,) = {
    if (locals.var_guard168 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__muf0, locals.var_fn169_calc_iq__muf0_dn4,)
    }
};
        locals.var_fn169_calc_iq__muf0 = assign13740_e13833;
        locals.var_fn169_calc_iq__muf0_dn4 = assign13740_e13833_d_n4;

        let (assign13750_e13837, assign13750_e13837_d_n4,) = {
    if (locals.var_guard168 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__vx0, locals.var_fn169_calc_iq__vx0_dn4,)
    }
};
        locals.var_fn169_calc_iq__vx0 = assign13750_e13837;
        locals.var_fn169_calc_iq__vx0_dn4 = assign13750_e13837_d_n4;

        let (assign13760_e13841, assign13760_e13841_d_n4,) = {
    if (locals.var_guard168 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__tfacmobin, locals.var_fn169_calc_iq__tfacmobin_dn4,)
    }
};
        locals.var_fn169_calc_iq__tfacmobin = assign13760_e13841;
        locals.var_fn169_calc_iq__tfacmobin_dn4 = assign13760_e13841_d_n4;

        let (assign13770_e13845, assign13770_e13845_d_n2, assign13770_e13845_d_n3, assign13770_e13845_d_n4, assign13770_e13845_d_n7, assign13770_e13845_d_n9, assign13770_e13845_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__ff, locals.var_fn169_calc_iq__ff_dn2, locals.var_fn169_calc_iq__ff_dn3, locals.var_fn169_calc_iq__ff_dn4, locals.var_fn169_calc_iq__ff_dn7, locals.var_fn169_calc_iq__ff_dn9, locals.var_fn169_calc_iq__ff_dn10,)
    }
};
        locals.var_fn169_calc_iq__ff = assign13770_e13845;
        locals.var_fn169_calc_iq__ff_dn2 = assign13770_e13845_d_n2;
        locals.var_fn169_calc_iq__ff_dn3 = assign13770_e13845_d_n3;
        locals.var_fn169_calc_iq__ff_dn4 = assign13770_e13845_d_n4;
        locals.var_fn169_calc_iq__ff_dn7 = assign13770_e13845_d_n7;
        locals.var_fn169_calc_iq__ff_dn9 = assign13770_e13845_d_n9;
        locals.var_fn169_calc_iq__ff_dn10 = assign13770_e13845_d_n10;

        let (assign13780_e13849, assign13780_e13849_d_n2, assign13780_e13849_d_n3, assign13780_e13849_d_n4, assign13780_e13849_d_n7, assign13780_e13849_d_n9, assign13780_e13849_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__eta, locals.var_fn169_calc_iq__eta_dn2, locals.var_fn169_calc_iq__eta_dn3, locals.var_fn169_calc_iq__eta_dn4, locals.var_fn169_calc_iq__eta_dn7, locals.var_fn169_calc_iq__eta_dn9, locals.var_fn169_calc_iq__eta_dn10,)
    }
};
        locals.var_fn169_calc_iq__eta = assign13780_e13849;
        locals.var_fn169_calc_iq__eta_dn2 = assign13780_e13849_d_n2;
        locals.var_fn169_calc_iq__eta_dn3 = assign13780_e13849_d_n3;
        locals.var_fn169_calc_iq__eta_dn4 = assign13780_e13849_d_n4;
        locals.var_fn169_calc_iq__eta_dn7 = assign13780_e13849_d_n7;
        locals.var_fn169_calc_iq__eta_dn9 = assign13780_e13849_d_n9;
        locals.var_fn169_calc_iq__eta_dn10 = assign13780_e13849_d_n10;

        let (assign13790_e13853, assign13790_e13853_d_n2, assign13790_e13853_d_n3, assign13790_e13853_d_n4, assign13790_e13853_d_n7, assign13790_e13853_d_n9, assign13790_e13853_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__qinvv, locals.var_fn169_calc_iq__qinvv_dn2, locals.var_fn169_calc_iq__qinvv_dn3, locals.var_fn169_calc_iq__qinvv_dn4, locals.var_fn169_calc_iq__qinvv_dn7, locals.var_fn169_calc_iq__qinvv_dn9, locals.var_fn169_calc_iq__qinvv_dn10,)
    }
};
        locals.var_fn169_calc_iq__qinvv = assign13790_e13853;
        locals.var_fn169_calc_iq__qinvv_dn2 = assign13790_e13853_d_n2;
        locals.var_fn169_calc_iq__qinvv_dn3 = assign13790_e13853_d_n3;
        locals.var_fn169_calc_iq__qinvv_dn4 = assign13790_e13853_d_n4;
        locals.var_fn169_calc_iq__qinvv_dn7 = assign13790_e13853_d_n7;
        locals.var_fn169_calc_iq__qinvv_dn9 = assign13790_e13853_d_n9;
        locals.var_fn169_calc_iq__qinvv_dn10 = assign13790_e13853_d_n10;

        let (assign13800_e13857, assign13800_e13857_d_n2, assign13800_e13857_d_n4, assign13800_e13857_d_n7, assign13800_e13857_d_n9, assign13800_e13857_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__ff0, locals.var_fn169_calc_iq__ff0_dn2, locals.var_fn169_calc_iq__ff0_dn4, locals.var_fn169_calc_iq__ff0_dn7, locals.var_fn169_calc_iq__ff0_dn9, locals.var_fn169_calc_iq__ff0_dn10,)
    }
};
        locals.var_fn169_calc_iq__ff0 = assign13800_e13857;
        locals.var_fn169_calc_iq__ff0_dn2 = assign13800_e13857_d_n2;
        locals.var_fn169_calc_iq__ff0_dn4 = assign13800_e13857_d_n4;
        locals.var_fn169_calc_iq__ff0_dn7 = assign13800_e13857_d_n7;
        locals.var_fn169_calc_iq__ff0_dn9 = assign13800_e13857_d_n9;
        locals.var_fn169_calc_iq__ff0_dn10 = assign13800_e13857_d_n10;

        let (assign13810_e13861, assign13810_e13861_d_n2, assign13810_e13861_d_n4, assign13810_e13861_d_n7, assign13810_e13861_d_n9, assign13810_e13861_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__eta0, locals.var_fn169_calc_iq__eta0_dn2, locals.var_fn169_calc_iq__eta0_dn4, locals.var_fn169_calc_iq__eta0_dn7, locals.var_fn169_calc_iq__eta0_dn9, locals.var_fn169_calc_iq__eta0_dn10,)
    }
};
        locals.var_fn169_calc_iq__eta0 = assign13810_e13861;
        locals.var_fn169_calc_iq__eta0_dn2 = assign13810_e13861_d_n2;
        locals.var_fn169_calc_iq__eta0_dn4 = assign13810_e13861_d_n4;
        locals.var_fn169_calc_iq__eta0_dn7 = assign13810_e13861_d_n7;
        locals.var_fn169_calc_iq__eta0_dn9 = assign13810_e13861_d_n9;
        locals.var_fn169_calc_iq__eta0_dn10 = assign13810_e13861_d_n10;

        let (assign13820_e13865, assign13820_e13865_d_n2, assign13820_e13865_d_n4, assign13820_e13865_d_n7, assign13820_e13865_d_n9, assign13820_e13865_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__qinvv0, locals.var_fn169_calc_iq__qinvv0_dn2, locals.var_fn169_calc_iq__qinvv0_dn4, locals.var_fn169_calc_iq__qinvv0_dn7, locals.var_fn169_calc_iq__qinvv0_dn9, locals.var_fn169_calc_iq__qinvv0_dn10,)
    }
};
        locals.var_fn169_calc_iq__qinvv0 = assign13820_e13865;
        locals.var_fn169_calc_iq__qinvv0_dn2 = assign13820_e13865_d_n2;
        locals.var_fn169_calc_iq__qinvv0_dn4 = assign13820_e13865_d_n4;
        locals.var_fn169_calc_iq__qinvv0_dn7 = assign13820_e13865_d_n7;
        locals.var_fn169_calc_iq__qinvv0_dn9 = assign13820_e13865_d_n9;
        locals.var_fn169_calc_iq__qinvv0_dn10 = assign13820_e13865_d_n10;

        let (assign13830_e13869, assign13830_e13869_d_n2, assign13830_e13869_d_n3, assign13830_e13869_d_n4, assign13830_e13869_d_n7, assign13830_e13869_d_n9, assign13830_e13869_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__vdsats, locals.var_fn169_calc_iq__vdsats_dn2, locals.var_fn169_calc_iq__vdsats_dn3, locals.var_fn169_calc_iq__vdsats_dn4, locals.var_fn169_calc_iq__vdsats_dn7, locals.var_fn169_calc_iq__vdsats_dn9, locals.var_fn169_calc_iq__vdsats_dn10,)
    }
};
        locals.var_fn169_calc_iq__vdsats = assign13830_e13869;
        locals.var_fn169_calc_iq__vdsats_dn2 = assign13830_e13869_d_n2;
        locals.var_fn169_calc_iq__vdsats_dn3 = assign13830_e13869_d_n3;
        locals.var_fn169_calc_iq__vdsats_dn4 = assign13830_e13869_d_n4;
        locals.var_fn169_calc_iq__vdsats_dn7 = assign13830_e13869_d_n7;
        locals.var_fn169_calc_iq__vdsats_dn9 = assign13830_e13869_d_n9;
        locals.var_fn169_calc_iq__vdsats_dn10 = assign13830_e13869_d_n10;

        let (assign13840_e13873, assign13840_e13873_d_n2, assign13840_e13873_d_n3, assign13840_e13873_d_n4, assign13840_e13873_d_n7, assign13840_e13873_d_n9, assign13840_e13873_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__vdsats1, locals.var_fn169_calc_iq__vdsats1_dn2, locals.var_fn169_calc_iq__vdsats1_dn3, locals.var_fn169_calc_iq__vdsats1_dn4, locals.var_fn169_calc_iq__vdsats1_dn7, locals.var_fn169_calc_iq__vdsats1_dn9, locals.var_fn169_calc_iq__vdsats1_dn10,)
    }
};
        locals.var_fn169_calc_iq__vdsats1 = assign13840_e13873;
        locals.var_fn169_calc_iq__vdsats1_dn2 = assign13840_e13873_d_n2;
        locals.var_fn169_calc_iq__vdsats1_dn3 = assign13840_e13873_d_n3;
        locals.var_fn169_calc_iq__vdsats1_dn4 = assign13840_e13873_d_n4;
        locals.var_fn169_calc_iq__vdsats1_dn7 = assign13840_e13873_d_n7;
        locals.var_fn169_calc_iq__vdsats1_dn9 = assign13840_e13873_d_n9;
        locals.var_fn169_calc_iq__vdsats1_dn10 = assign13840_e13873_d_n10;

        let (assign13850_e13877, assign13850_e13877_d_n2, assign13850_e13877_d_n3, assign13850_e13877_d_n4, assign13850_e13877_d_n7, assign13850_e13877_d_n9, assign13850_e13877_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__vdsat, locals.var_fn169_calc_iq__vdsat_dn2, locals.var_fn169_calc_iq__vdsat_dn3, locals.var_fn169_calc_iq__vdsat_dn4, locals.var_fn169_calc_iq__vdsat_dn7, locals.var_fn169_calc_iq__vdsat_dn9, locals.var_fn169_calc_iq__vdsat_dn10,)
    }
};
        locals.var_fn169_calc_iq__vdsat = assign13850_e13877;
        locals.var_fn169_calc_iq__vdsat_dn2 = assign13850_e13877_d_n2;
        locals.var_fn169_calc_iq__vdsat_dn3 = assign13850_e13877_d_n3;
        locals.var_fn169_calc_iq__vdsat_dn4 = assign13850_e13877_d_n4;
        locals.var_fn169_calc_iq__vdsat_dn7 = assign13850_e13877_d_n7;
        locals.var_fn169_calc_iq__vdsat_dn9 = assign13850_e13877_d_n9;
        locals.var_fn169_calc_iq__vdsat_dn10 = assign13850_e13877_d_n10;

        let (assign13860_e13881, assign13860_e13881_d_n2, assign13860_e13881_d_n3, assign13860_e13881_d_n4, assign13860_e13881_d_n7, assign13860_e13881_d_n9, assign13860_e13881_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__fsd, locals.var_fn169_calc_iq__fsd_dn2, locals.var_fn169_calc_iq__fsd_dn3, locals.var_fn169_calc_iq__fsd_dn4, locals.var_fn169_calc_iq__fsd_dn7, locals.var_fn169_calc_iq__fsd_dn9, locals.var_fn169_calc_iq__fsd_dn10,)
    }
};
        locals.var_fn169_calc_iq__fsd = assign13860_e13881;
        locals.var_fn169_calc_iq__fsd_dn2 = assign13860_e13881_d_n2;
        locals.var_fn169_calc_iq__fsd_dn3 = assign13860_e13881_d_n3;
        locals.var_fn169_calc_iq__fsd_dn4 = assign13860_e13881_d_n4;
        locals.var_fn169_calc_iq__fsd_dn7 = assign13860_e13881_d_n7;
        locals.var_fn169_calc_iq__fsd_dn9 = assign13860_e13881_d_n9;
        locals.var_fn169_calc_iq__fsd_dn10 = assign13860_e13881_d_n10;

        let (assign13870_e13885, assign13870_e13885_d_n2, assign13870_e13885_d_n3, assign13870_e13885_d_n4, assign13870_e13885_d_n7, assign13870_e13885_d_n9, assign13870_e13885_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__vdx, locals.var_fn169_calc_iq__vdx_dn2, locals.var_fn169_calc_iq__vdx_dn3, locals.var_fn169_calc_iq__vdx_dn4, locals.var_fn169_calc_iq__vdx_dn7, locals.var_fn169_calc_iq__vdx_dn9, locals.var_fn169_calc_iq__vdx_dn10,)
    }
};
        locals.var_fn169_calc_iq__vdx = assign13870_e13885;
        locals.var_fn169_calc_iq__vdx_dn2 = assign13870_e13885_d_n2;
        locals.var_fn169_calc_iq__vdx_dn3 = assign13870_e13885_d_n3;
        locals.var_fn169_calc_iq__vdx_dn4 = assign13870_e13885_d_n4;
        locals.var_fn169_calc_iq__vdx_dn7 = assign13870_e13885_d_n7;
        locals.var_fn169_calc_iq__vdx_dn9 = assign13870_e13885_d_n9;
        locals.var_fn169_calc_iq__vdx_dn10 = assign13870_e13885_d_n10;

        let (assign13880_e13889, assign13880_e13889_d_n2, assign13880_e13889_d_n3, assign13880_e13889_d_n4, assign13880_e13889_d_n7, assign13880_e13889_d_n9, assign13880_e13889_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__fds, locals.var_fn169_calc_iq__fds_dn2, locals.var_fn169_calc_iq__fds_dn3, locals.var_fn169_calc_iq__fds_dn4, locals.var_fn169_calc_iq__fds_dn7, locals.var_fn169_calc_iq__fds_dn9, locals.var_fn169_calc_iq__fds_dn10,)
    }
};
        locals.var_fn169_calc_iq__fds = assign13880_e13889;
        locals.var_fn169_calc_iq__fds_dn2 = assign13880_e13889_d_n2;
        locals.var_fn169_calc_iq__fds_dn3 = assign13880_e13889_d_n3;
        locals.var_fn169_calc_iq__fds_dn4 = assign13880_e13889_d_n4;
        locals.var_fn169_calc_iq__fds_dn7 = assign13880_e13889_d_n7;
        locals.var_fn169_calc_iq__fds_dn9 = assign13880_e13889_d_n9;
        locals.var_fn169_calc_iq__fds_dn10 = assign13880_e13889_d_n10;

        let (assign13890_e13893, assign13890_e13893_d_n2, assign13890_e13893_d_n3, assign13890_e13893_d_n4, assign13890_e13893_d_n7, assign13890_e13893_d_n9, assign13890_e13893_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__vsx, locals.var_fn169_calc_iq__vsx_dn2, locals.var_fn169_calc_iq__vsx_dn3, locals.var_fn169_calc_iq__vsx_dn4, locals.var_fn169_calc_iq__vsx_dn7, locals.var_fn169_calc_iq__vsx_dn9, locals.var_fn169_calc_iq__vsx_dn10,)
    }
};
        locals.var_fn169_calc_iq__vsx = assign13890_e13893;
        locals.var_fn169_calc_iq__vsx_dn2 = assign13890_e13893_d_n2;
        locals.var_fn169_calc_iq__vsx_dn3 = assign13890_e13893_d_n3;
        locals.var_fn169_calc_iq__vsx_dn4 = assign13890_e13893_d_n4;
        locals.var_fn169_calc_iq__vsx_dn7 = assign13890_e13893_d_n7;
        locals.var_fn169_calc_iq__vsx_dn9 = assign13890_e13893_d_n9;
        locals.var_fn169_calc_iq__vsx_dn10 = assign13890_e13893_d_n10;

        let (assign13900_e13897, assign13900_e13897_d_n2, assign13900_e13897_d_n3, assign13900_e13897_d_n4, assign13900_e13897_d_n7, assign13900_e13897_d_n9, assign13900_e13897_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__ffd, locals.var_fn169_calc_iq__ffd_dn2, locals.var_fn169_calc_iq__ffd_dn3, locals.var_fn169_calc_iq__ffd_dn4, locals.var_fn169_calc_iq__ffd_dn7, locals.var_fn169_calc_iq__ffd_dn9, locals.var_fn169_calc_iq__ffd_dn10,)
    }
};
        locals.var_fn169_calc_iq__ffd = assign13900_e13897;
        locals.var_fn169_calc_iq__ffd_dn2 = assign13900_e13897_d_n2;
        locals.var_fn169_calc_iq__ffd_dn3 = assign13900_e13897_d_n3;
        locals.var_fn169_calc_iq__ffd_dn4 = assign13900_e13897_d_n4;
        locals.var_fn169_calc_iq__ffd_dn7 = assign13900_e13897_d_n7;
        locals.var_fn169_calc_iq__ffd_dn9 = assign13900_e13897_d_n9;
        locals.var_fn169_calc_iq__ffd_dn10 = assign13900_e13897_d_n10;

    }

    pub(super) fn stamp_transient_block_34(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign13910_e13901, assign13910_e13901_d_n2, assign13910_e13901_d_n3, assign13910_e13901_d_n4, assign13910_e13901_d_n7, assign13910_e13901_d_n9, assign13910_e13901_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__etad, locals.var_fn169_calc_iq__etad_dn2, locals.var_fn169_calc_iq__etad_dn3, locals.var_fn169_calc_iq__etad_dn4, locals.var_fn169_calc_iq__etad_dn7, locals.var_fn169_calc_iq__etad_dn9, locals.var_fn169_calc_iq__etad_dn10,)
    }
};
        locals.var_fn169_calc_iq__etad = assign13910_e13901;
        locals.var_fn169_calc_iq__etad_dn2 = assign13910_e13901_d_n2;
        locals.var_fn169_calc_iq__etad_dn3 = assign13910_e13901_d_n3;
        locals.var_fn169_calc_iq__etad_dn4 = assign13910_e13901_d_n4;
        locals.var_fn169_calc_iq__etad_dn7 = assign13910_e13901_d_n7;
        locals.var_fn169_calc_iq__etad_dn9 = assign13910_e13901_d_n9;
        locals.var_fn169_calc_iq__etad_dn10 = assign13910_e13901_d_n10;

        let (assign13920_e13905, assign13920_e13905_d_n2, assign13920_e13905_d_n3, assign13920_e13905_d_n4, assign13920_e13905_d_n7, assign13920_e13905_d_n9, assign13920_e13905_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__qinvd, locals.var_fn169_calc_iq__qinvd_dn2, locals.var_fn169_calc_iq__qinvd_dn3, locals.var_fn169_calc_iq__qinvd_dn4, locals.var_fn169_calc_iq__qinvd_dn7, locals.var_fn169_calc_iq__qinvd_dn9, locals.var_fn169_calc_iq__qinvd_dn10,)
    }
};
        locals.var_fn169_calc_iq__qinvd = assign13920_e13905;
        locals.var_fn169_calc_iq__qinvd_dn2 = assign13920_e13905_d_n2;
        locals.var_fn169_calc_iq__qinvd_dn3 = assign13920_e13905_d_n3;
        locals.var_fn169_calc_iq__qinvd_dn4 = assign13920_e13905_d_n4;
        locals.var_fn169_calc_iq__qinvd_dn7 = assign13920_e13905_d_n7;
        locals.var_fn169_calc_iq__qinvd_dn9 = assign13920_e13905_d_n9;
        locals.var_fn169_calc_iq__qinvd_dn10 = assign13920_e13905_d_n10;

        let (assign13930_e13909, assign13930_e13909_d_n2, assign13930_e13909_d_n3, assign13930_e13909_d_n4, assign13930_e13909_d_n7, assign13930_e13909_d_n9, assign13930_e13909_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__vdsc, locals.var_fn169_calc_iq__vdsc_dn2, locals.var_fn169_calc_iq__vdsc_dn3, locals.var_fn169_calc_iq__vdsc_dn4, locals.var_fn169_calc_iq__vdsc_dn7, locals.var_fn169_calc_iq__vdsc_dn9, locals.var_fn169_calc_iq__vdsc_dn10,)
    }
};
        locals.var_fn169_calc_iq__vdsc = assign13930_e13909;
        locals.var_fn169_calc_iq__vdsc_dn2 = assign13930_e13909_d_n2;
        locals.var_fn169_calc_iq__vdsc_dn3 = assign13930_e13909_d_n3;
        locals.var_fn169_calc_iq__vdsc_dn4 = assign13930_e13909_d_n4;
        locals.var_fn169_calc_iq__vdsc_dn7 = assign13930_e13909_d_n7;
        locals.var_fn169_calc_iq__vdsc_dn9 = assign13930_e13909_d_n9;
        locals.var_fn169_calc_iq__vdsc_dn10 = assign13930_e13909_d_n10;

        let (assign13960_e13921, assign13960_e13921_d_n4,) = {
    if (locals.var_guard168 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__vdsats0, locals.var_fn169_calc_iq__vdsats0_dn4,)
    }
};
        locals.var_fn169_calc_iq__vdsats0 = assign13960_e13921;
        locals.var_fn169_calc_iq__vdsats0_dn4 = assign13960_e13921_d_n4;

        let (assign13970_e13925, assign13970_e13925_d_n2, assign13970_e13925_d_n4, assign13970_e13925_d_n7, assign13970_e13925_d_n9, assign13970_e13925_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__vdsats10, locals.var_fn169_calc_iq__vdsats10_dn2, locals.var_fn169_calc_iq__vdsats10_dn4, locals.var_fn169_calc_iq__vdsats10_dn7, locals.var_fn169_calc_iq__vdsats10_dn9, locals.var_fn169_calc_iq__vdsats10_dn10,)
    }
};
        locals.var_fn169_calc_iq__vdsats10 = assign13970_e13925;
        locals.var_fn169_calc_iq__vdsats10_dn2 = assign13970_e13925_d_n2;
        locals.var_fn169_calc_iq__vdsats10_dn4 = assign13970_e13925_d_n4;
        locals.var_fn169_calc_iq__vdsats10_dn7 = assign13970_e13925_d_n7;
        locals.var_fn169_calc_iq__vdsats10_dn9 = assign13970_e13925_d_n9;
        locals.var_fn169_calc_iq__vdsats10_dn10 = assign13970_e13925_d_n10;

        let (assign13980_e13929, assign13980_e13929_d_n2, assign13980_e13929_d_n4, assign13980_e13929_d_n7, assign13980_e13929_d_n9, assign13980_e13929_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__vdsat10, locals.var_fn169_calc_iq__vdsat10_dn2, locals.var_fn169_calc_iq__vdsat10_dn4, locals.var_fn169_calc_iq__vdsat10_dn7, locals.var_fn169_calc_iq__vdsat10_dn9, locals.var_fn169_calc_iq__vdsat10_dn10,)
    }
};
        locals.var_fn169_calc_iq__vdsat10 = assign13980_e13929;
        locals.var_fn169_calc_iq__vdsat10_dn2 = assign13980_e13929_d_n2;
        locals.var_fn169_calc_iq__vdsat10_dn4 = assign13980_e13929_d_n4;
        locals.var_fn169_calc_iq__vdsat10_dn7 = assign13980_e13929_d_n7;
        locals.var_fn169_calc_iq__vdsat10_dn9 = assign13980_e13929_d_n9;
        locals.var_fn169_calc_iq__vdsat10_dn10 = assign13980_e13929_d_n10;

        let (assign13990_e13933, assign13990_e13933_d_n2, assign13990_e13933_d_n4, assign13990_e13933_d_n7, assign13990_e13933_d_n9, assign13990_e13933_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__fsd0, locals.var_fn169_calc_iq__fsd0_dn2, locals.var_fn169_calc_iq__fsd0_dn4, locals.var_fn169_calc_iq__fsd0_dn7, locals.var_fn169_calc_iq__fsd0_dn9, locals.var_fn169_calc_iq__fsd0_dn10,)
    }
};
        locals.var_fn169_calc_iq__fsd0 = assign13990_e13933;
        locals.var_fn169_calc_iq__fsd0_dn2 = assign13990_e13933_d_n2;
        locals.var_fn169_calc_iq__fsd0_dn4 = assign13990_e13933_d_n4;
        locals.var_fn169_calc_iq__fsd0_dn7 = assign13990_e13933_d_n7;
        locals.var_fn169_calc_iq__fsd0_dn9 = assign13990_e13933_d_n9;
        locals.var_fn169_calc_iq__fsd0_dn10 = assign13990_e13933_d_n10;

        let (assign14000_e13937, assign14000_e13937_d_n2, assign14000_e13937_d_n4, assign14000_e13937_d_n7, assign14000_e13937_d_n9, assign14000_e13937_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__vdx0, locals.var_fn169_calc_iq__vdx0_dn2, locals.var_fn169_calc_iq__vdx0_dn4, locals.var_fn169_calc_iq__vdx0_dn7, locals.var_fn169_calc_iq__vdx0_dn9, locals.var_fn169_calc_iq__vdx0_dn10,)
    }
};
        locals.var_fn169_calc_iq__vdx0 = assign14000_e13937;
        locals.var_fn169_calc_iq__vdx0_dn2 = assign14000_e13937_d_n2;
        locals.var_fn169_calc_iq__vdx0_dn4 = assign14000_e13937_d_n4;
        locals.var_fn169_calc_iq__vdx0_dn7 = assign14000_e13937_d_n7;
        locals.var_fn169_calc_iq__vdx0_dn9 = assign14000_e13937_d_n9;
        locals.var_fn169_calc_iq__vdx0_dn10 = assign14000_e13937_d_n10;

        let (assign14010_e13941, assign14010_e13941_d_n2, assign14010_e13941_d_n4, assign14010_e13941_d_n7, assign14010_e13941_d_n9, assign14010_e13941_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__fds0, locals.var_fn169_calc_iq__fds0_dn2, locals.var_fn169_calc_iq__fds0_dn4, locals.var_fn169_calc_iq__fds0_dn7, locals.var_fn169_calc_iq__fds0_dn9, locals.var_fn169_calc_iq__fds0_dn10,)
    }
};
        locals.var_fn169_calc_iq__fds0 = assign14010_e13941;
        locals.var_fn169_calc_iq__fds0_dn2 = assign14010_e13941_d_n2;
        locals.var_fn169_calc_iq__fds0_dn4 = assign14010_e13941_d_n4;
        locals.var_fn169_calc_iq__fds0_dn7 = assign14010_e13941_d_n7;
        locals.var_fn169_calc_iq__fds0_dn9 = assign14010_e13941_d_n9;
        locals.var_fn169_calc_iq__fds0_dn10 = assign14010_e13941_d_n10;

        let (assign14020_e13945, assign14020_e13945_d_n2, assign14020_e13945_d_n4, assign14020_e13945_d_n7, assign14020_e13945_d_n9, assign14020_e13945_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__vsx0, locals.var_fn169_calc_iq__vsx0_dn2, locals.var_fn169_calc_iq__vsx0_dn4, locals.var_fn169_calc_iq__vsx0_dn7, locals.var_fn169_calc_iq__vsx0_dn9, locals.var_fn169_calc_iq__vsx0_dn10,)
    }
};
        locals.var_fn169_calc_iq__vsx0 = assign14020_e13945;
        locals.var_fn169_calc_iq__vsx0_dn2 = assign14020_e13945_d_n2;
        locals.var_fn169_calc_iq__vsx0_dn4 = assign14020_e13945_d_n4;
        locals.var_fn169_calc_iq__vsx0_dn7 = assign14020_e13945_d_n7;
        locals.var_fn169_calc_iq__vsx0_dn9 = assign14020_e13945_d_n9;
        locals.var_fn169_calc_iq__vsx0_dn10 = assign14020_e13945_d_n10;

        let (assign14030_e13949, assign14030_e13949_d_n2, assign14030_e13949_d_n4, assign14030_e13949_d_n7, assign14030_e13949_d_n9, assign14030_e13949_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__ffd0, locals.var_fn169_calc_iq__ffd0_dn2, locals.var_fn169_calc_iq__ffd0_dn4, locals.var_fn169_calc_iq__ffd0_dn7, locals.var_fn169_calc_iq__ffd0_dn9, locals.var_fn169_calc_iq__ffd0_dn10,)
    }
};
        locals.var_fn169_calc_iq__ffd0 = assign14030_e13949;
        locals.var_fn169_calc_iq__ffd0_dn2 = assign14030_e13949_d_n2;
        locals.var_fn169_calc_iq__ffd0_dn4 = assign14030_e13949_d_n4;
        locals.var_fn169_calc_iq__ffd0_dn7 = assign14030_e13949_d_n7;
        locals.var_fn169_calc_iq__ffd0_dn9 = assign14030_e13949_d_n9;
        locals.var_fn169_calc_iq__ffd0_dn10 = assign14030_e13949_d_n10;

        let (assign14040_e13953, assign14040_e13953_d_n2, assign14040_e13953_d_n4, assign14040_e13953_d_n7, assign14040_e13953_d_n9, assign14040_e13953_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__etad0, locals.var_fn169_calc_iq__etad0_dn2, locals.var_fn169_calc_iq__etad0_dn4, locals.var_fn169_calc_iq__etad0_dn7, locals.var_fn169_calc_iq__etad0_dn9, locals.var_fn169_calc_iq__etad0_dn10,)
    }
};
        locals.var_fn169_calc_iq__etad0 = assign14040_e13953;
        locals.var_fn169_calc_iq__etad0_dn2 = assign14040_e13953_d_n2;
        locals.var_fn169_calc_iq__etad0_dn4 = assign14040_e13953_d_n4;
        locals.var_fn169_calc_iq__etad0_dn7 = assign14040_e13953_d_n7;
        locals.var_fn169_calc_iq__etad0_dn9 = assign14040_e13953_d_n9;
        locals.var_fn169_calc_iq__etad0_dn10 = assign14040_e13953_d_n10;

        let (assign14050_e13957, assign14050_e13957_d_n2, assign14050_e13957_d_n4, assign14050_e13957_d_n7, assign14050_e13957_d_n9, assign14050_e13957_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__qinvd0, locals.var_fn169_calc_iq__qinvd0_dn2, locals.var_fn169_calc_iq__qinvd0_dn4, locals.var_fn169_calc_iq__qinvd0_dn7, locals.var_fn169_calc_iq__qinvd0_dn9, locals.var_fn169_calc_iq__qinvd0_dn10,)
    }
};
        locals.var_fn169_calc_iq__qinvd0 = assign14050_e13957;
        locals.var_fn169_calc_iq__qinvd0_dn2 = assign14050_e13957_d_n2;
        locals.var_fn169_calc_iq__qinvd0_dn4 = assign14050_e13957_d_n4;
        locals.var_fn169_calc_iq__qinvd0_dn7 = assign14050_e13957_d_n7;
        locals.var_fn169_calc_iq__qinvd0_dn9 = assign14050_e13957_d_n9;
        locals.var_fn169_calc_iq__qinvd0_dn10 = assign14050_e13957_d_n10;

        let (assign14060_e13961, assign14060_e13961_d_n2, assign14060_e13961_d_n4, assign14060_e13961_d_n7, assign14060_e13961_d_n9, assign14060_e13961_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__qs2, locals.var_fn169_calc_iq__qs2_dn2, locals.var_fn169_calc_iq__qs2_dn4, locals.var_fn169_calc_iq__qs2_dn7, locals.var_fn169_calc_iq__qs2_dn9, locals.var_fn169_calc_iq__qs2_dn10,)
    }
};
        locals.var_fn169_calc_iq__qs2 = assign14060_e13961;
        locals.var_fn169_calc_iq__qs2_dn2 = assign14060_e13961_d_n2;
        locals.var_fn169_calc_iq__qs2_dn4 = assign14060_e13961_d_n4;
        locals.var_fn169_calc_iq__qs2_dn7 = assign14060_e13961_d_n7;
        locals.var_fn169_calc_iq__qs2_dn9 = assign14060_e13961_d_n9;
        locals.var_fn169_calc_iq__qs2_dn10 = assign14060_e13961_d_n10;

        let (assign14070_e13965, assign14070_e13965_d_n2, assign14070_e13965_d_n4, assign14070_e13965_d_n7, assign14070_e13965_d_n9, assign14070_e13965_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__qs3, locals.var_fn169_calc_iq__qs3_dn2, locals.var_fn169_calc_iq__qs3_dn4, locals.var_fn169_calc_iq__qs3_dn7, locals.var_fn169_calc_iq__qs3_dn9, locals.var_fn169_calc_iq__qs3_dn10,)
    }
};
        locals.var_fn169_calc_iq__qs3 = assign14070_e13965;
        locals.var_fn169_calc_iq__qs3_dn2 = assign14070_e13965_d_n2;
        locals.var_fn169_calc_iq__qs3_dn4 = assign14070_e13965_d_n4;
        locals.var_fn169_calc_iq__qs3_dn7 = assign14070_e13965_d_n7;
        locals.var_fn169_calc_iq__qs3_dn9 = assign14070_e13965_d_n9;
        locals.var_fn169_calc_iq__qs3_dn10 = assign14070_e13965_d_n10;

        let (assign14080_e13969, assign14080_e13969_d_n2, assign14080_e13969_d_n4, assign14080_e13969_d_n7, assign14080_e13969_d_n9, assign14080_e13969_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__qd2, locals.var_fn169_calc_iq__qd2_dn2, locals.var_fn169_calc_iq__qd2_dn4, locals.var_fn169_calc_iq__qd2_dn7, locals.var_fn169_calc_iq__qd2_dn9, locals.var_fn169_calc_iq__qd2_dn10,)
    }
};
        locals.var_fn169_calc_iq__qd2 = assign14080_e13969;
        locals.var_fn169_calc_iq__qd2_dn2 = assign14080_e13969_d_n2;
        locals.var_fn169_calc_iq__qd2_dn4 = assign14080_e13969_d_n4;
        locals.var_fn169_calc_iq__qd2_dn7 = assign14080_e13969_d_n7;
        locals.var_fn169_calc_iq__qd2_dn9 = assign14080_e13969_d_n9;
        locals.var_fn169_calc_iq__qd2_dn10 = assign14080_e13969_d_n10;

        let (assign14090_e13973, assign14090_e13973_d_n2, assign14090_e13973_d_n4, assign14090_e13973_d_n7, assign14090_e13973_d_n9, assign14090_e13973_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__qd3, locals.var_fn169_calc_iq__qd3_dn2, locals.var_fn169_calc_iq__qd3_dn4, locals.var_fn169_calc_iq__qd3_dn7, locals.var_fn169_calc_iq__qd3_dn9, locals.var_fn169_calc_iq__qd3_dn10,)
    }
};
        locals.var_fn169_calc_iq__qd3 = assign14090_e13973;
        locals.var_fn169_calc_iq__qd3_dn2 = assign14090_e13973_d_n2;
        locals.var_fn169_calc_iq__qd3_dn4 = assign14090_e13973_d_n4;
        locals.var_fn169_calc_iq__qd3_dn7 = assign14090_e13973_d_n7;
        locals.var_fn169_calc_iq__qd3_dn9 = assign14090_e13973_d_n9;
        locals.var_fn169_calc_iq__qd3_dn10 = assign14090_e13973_d_n10;

        let (assign14100_e13977, assign14100_e13977_d_n2, assign14100_e13977_d_n4, assign14100_e13977_d_n7, assign14100_e13977_d_n9, assign14100_e13977_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__qsqd, locals.var_fn169_calc_iq__qsqd_dn2, locals.var_fn169_calc_iq__qsqd_dn4, locals.var_fn169_calc_iq__qsqd_dn7, locals.var_fn169_calc_iq__qsqd_dn9, locals.var_fn169_calc_iq__qsqd_dn10,)
    }
};
        locals.var_fn169_calc_iq__qsqd = assign14100_e13977;
        locals.var_fn169_calc_iq__qsqd_dn2 = assign14100_e13977_d_n2;
        locals.var_fn169_calc_iq__qsqd_dn4 = assign14100_e13977_d_n4;
        locals.var_fn169_calc_iq__qsqd_dn7 = assign14100_e13977_d_n7;
        locals.var_fn169_calc_iq__qsqd_dn9 = assign14100_e13977_d_n9;
        locals.var_fn169_calc_iq__qsqd_dn10 = assign14100_e13977_d_n10;

        let (assign14110_e13981, assign14110_e13981_d_n2, assign14110_e13981_d_n4, assign14110_e13981_d_n7, assign14110_e13981_d_n9, assign14110_e13981_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__qinvdd, locals.var_fn169_calc_iq__qinvdd_dn2, locals.var_fn169_calc_iq__qinvdd_dn4, locals.var_fn169_calc_iq__qinvdd_dn7, locals.var_fn169_calc_iq__qinvdd_dn9, locals.var_fn169_calc_iq__qinvdd_dn10,)
    }
};
        locals.var_fn169_calc_iq__qinvdd = assign14110_e13981;
        locals.var_fn169_calc_iq__qinvdd_dn2 = assign14110_e13981_d_n2;
        locals.var_fn169_calc_iq__qinvdd_dn4 = assign14110_e13981_d_n4;
        locals.var_fn169_calc_iq__qinvdd_dn7 = assign14110_e13981_d_n7;
        locals.var_fn169_calc_iq__qinvdd_dn9 = assign14110_e13981_d_n9;
        locals.var_fn169_calc_iq__qinvdd_dn10 = assign14110_e13981_d_n10;

        let (assign14120_e13985, assign14120_e13985_d_n2, assign14120_e13985_d_n4, assign14120_e13985_d_n7, assign14120_e13985_d_n9, assign14120_e13985_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__qd1, locals.var_fn169_calc_iq__qd1_dn2, locals.var_fn169_calc_iq__qd1_dn4, locals.var_fn169_calc_iq__qd1_dn7, locals.var_fn169_calc_iq__qd1_dn9, locals.var_fn169_calc_iq__qd1_dn10,)
    }
};
        locals.var_fn169_calc_iq__qd1 = assign14120_e13985;
        locals.var_fn169_calc_iq__qd1_dn2 = assign14120_e13985_d_n2;
        locals.var_fn169_calc_iq__qd1_dn4 = assign14120_e13985_d_n4;
        locals.var_fn169_calc_iq__qd1_dn7 = assign14120_e13985_d_n7;
        locals.var_fn169_calc_iq__qd1_dn9 = assign14120_e13985_d_n9;
        locals.var_fn169_calc_iq__qd1_dn10 = assign14120_e13985_d_n10;

        let (assign14130_e13989, assign14130_e13989_d_n2, assign14130_e13989_d_n4, assign14130_e13989_d_n7, assign14130_e13989_d_n9, assign14130_e13989_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__qs, locals.var_fn169_calc_iq__qs_dn2, locals.var_fn169_calc_iq__qs_dn4, locals.var_fn169_calc_iq__qs_dn7, locals.var_fn169_calc_iq__qs_dn9, locals.var_fn169_calc_iq__qs_dn10,)
    }
};
        locals.var_fn169_calc_iq__qs = assign14130_e13989;
        locals.var_fn169_calc_iq__qs_dn2 = assign14130_e13989_d_n2;
        locals.var_fn169_calc_iq__qs_dn4 = assign14130_e13989_d_n4;
        locals.var_fn169_calc_iq__qs_dn7 = assign14130_e13989_d_n7;
        locals.var_fn169_calc_iq__qs_dn9 = assign14130_e13989_d_n9;
        locals.var_fn169_calc_iq__qs_dn10 = assign14130_e13989_d_n10;

        let (assign14140_e13993, assign14140_e13993_d_n2, assign14140_e13993_d_n4, assign14140_e13993_d_n7, assign14140_e13993_d_n9, assign14140_e13993_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__qd, locals.var_fn169_calc_iq__qd_dn2, locals.var_fn169_calc_iq__qd_dn4, locals.var_fn169_calc_iq__qd_dn7, locals.var_fn169_calc_iq__qd_dn9, locals.var_fn169_calc_iq__qd_dn10,)
    }
};
        locals.var_fn169_calc_iq__qd = assign14140_e13993;
        locals.var_fn169_calc_iq__qd_dn2 = assign14140_e13993_d_n2;
        locals.var_fn169_calc_iq__qd_dn4 = assign14140_e13993_d_n4;
        locals.var_fn169_calc_iq__qd_dn7 = assign14140_e13993_d_n7;
        locals.var_fn169_calc_iq__qd_dn9 = assign14140_e13993_d_n9;
        locals.var_fn169_calc_iq__qd_dn10 = assign14140_e13993_d_n10;

        let (assign14150_e13997, assign14150_e13997_d_n2, assign14150_e13997_d_n4, assign14150_e13997_d_n7, assign14150_e13997_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__etac, locals.var_fn169_calc_iq__etac_dn2, locals.var_fn169_calc_iq__etac_dn4, locals.var_fn169_calc_iq__etac_dn7, locals.var_fn169_calc_iq__etac_dn10,)
    }
};
        locals.var_fn169_calc_iq__etac = assign14150_e13997;
        locals.var_fn169_calc_iq__etac_dn2 = assign14150_e13997_d_n2;
        locals.var_fn169_calc_iq__etac_dn4 = assign14150_e13997_d_n4;
        locals.var_fn169_calc_iq__etac_dn7 = assign14150_e13997_d_n7;
        locals.var_fn169_calc_iq__etac_dn10 = assign14150_e13997_d_n10;

        let (assign14160_e14001, assign14160_e14001_d_n3, assign14160_e14001_d_n4, assign14160_e14001_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__etab, locals.var_fn169_calc_iq__etab_dn3, locals.var_fn169_calc_iq__etab_dn4, locals.var_fn169_calc_iq__etab_dn10,)
    }
};
        locals.var_fn169_calc_iq__etab = assign14160_e14001;
        locals.var_fn169_calc_iq__etab_dn3 = assign14160_e14001_d_n3;
        locals.var_fn169_calc_iq__etab_dn4 = assign14160_e14001_d_n4;
        locals.var_fn169_calc_iq__etab_dn10 = assign14160_e14001_d_n10;

        let (assign14170_e14005, assign14170_e14005_d_n2, assign14170_e14005_d_n4, assign14170_e14005_d_n7, assign14170_e14005_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__etags, locals.var_fn169_calc_iq__etags_dn2, locals.var_fn169_calc_iq__etags_dn4, locals.var_fn169_calc_iq__etags_dn7, locals.var_fn169_calc_iq__etags_dn10,)
    }
};
        locals.var_fn169_calc_iq__etags = assign14170_e14005;
        locals.var_fn169_calc_iq__etags_dn2 = assign14170_e14005_d_n2;
        locals.var_fn169_calc_iq__etags_dn4 = assign14170_e14005_d_n4;
        locals.var_fn169_calc_iq__etags_dn7 = assign14170_e14005_d_n7;
        locals.var_fn169_calc_iq__etags_dn10 = assign14170_e14005_d_n10;

        let (assign14180_e14009, assign14180_e14009_d_n2, assign14180_e14009_d_n3, assign14180_e14009_d_n4, assign14180_e14009_d_n7, assign14180_e14009_d_n9, assign14180_e14009_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__exparg, locals.var_fn169_calc_iq__exparg_dn2, locals.var_fn169_calc_iq__exparg_dn3, locals.var_fn169_calc_iq__exparg_dn4, locals.var_fn169_calc_iq__exparg_dn7, locals.var_fn169_calc_iq__exparg_dn9, locals.var_fn169_calc_iq__exparg_dn10,)
    }
};
        locals.var_fn169_calc_iq__exparg = assign14180_e14009;
        locals.var_fn169_calc_iq__exparg_dn2 = assign14180_e14009_d_n2;
        locals.var_fn169_calc_iq__exparg_dn3 = assign14180_e14009_d_n3;
        locals.var_fn169_calc_iq__exparg_dn4 = assign14180_e14009_d_n4;
        locals.var_fn169_calc_iq__exparg_dn7 = assign14180_e14009_d_n7;
        locals.var_fn169_calc_iq__exparg_dn9 = assign14180_e14009_d_n9;
        locals.var_fn169_calc_iq__exparg_dn10 = assign14180_e14009_d_n10;

        let (assign14190_e14013, assign14190_e14013_d_n2, assign14190_e14013_d_n3, assign14190_e14013_d_n4, assign14190_e14013_d_n7, assign14190_e14013_d_n9, assign14190_e14013_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__myarg, locals.var_fn169_calc_iq__myarg_dn2, locals.var_fn169_calc_iq__myarg_dn3, locals.var_fn169_calc_iq__myarg_dn4, locals.var_fn169_calc_iq__myarg_dn7, locals.var_fn169_calc_iq__myarg_dn9, locals.var_fn169_calc_iq__myarg_dn10,)
    }
};
        locals.var_fn169_calc_iq__myarg = assign14190_e14013;
        locals.var_fn169_calc_iq__myarg_dn2 = assign14190_e14013_d_n2;
        locals.var_fn169_calc_iq__myarg_dn3 = assign14190_e14013_d_n3;
        locals.var_fn169_calc_iq__myarg_dn4 = assign14190_e14013_d_n4;
        locals.var_fn169_calc_iq__myarg_dn7 = assign14190_e14013_d_n7;
        locals.var_fn169_calc_iq__myarg_dn9 = assign14190_e14013_d_n9;
        locals.var_fn169_calc_iq__myarg_dn10 = assign14190_e14013_d_n10;

        let (assign14200_e14017, assign14200_e14017_d_n9, assign14200_e14017_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        (0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__absvdsin, locals.var_fn169_calc_iq__absvdsin_dn9, locals.var_fn169_calc_iq__absvdsin_dn10,)
    }
};
        locals.var_fn169_calc_iq__absvdsin = assign14200_e14017;
        locals.var_fn169_calc_iq__absvdsin_dn9 = assign14200_e14017_d_n9;
        locals.var_fn169_calc_iq__absvdsin_dn10 = assign14200_e14017_d_n10;

        let (assign14210_e14021, assign14210_e14021_d_n2, assign14210_e14021_d_n7, assign14210_e14021_d_n9, assign14210_e14021_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__vgdin, locals.var_fn169_calc_iq__vgdin_dn2, locals.var_fn169_calc_iq__vgdin_dn7, locals.var_fn169_calc_iq__vgdin_dn9, locals.var_fn169_calc_iq__vgdin_dn10,)
    }
};
        locals.var_fn169_calc_iq__vgdin = assign14210_e14021;
        locals.var_fn169_calc_iq__vgdin_dn2 = assign14210_e14021_d_n2;
        locals.var_fn169_calc_iq__vgdin_dn7 = assign14210_e14021_d_n7;
        locals.var_fn169_calc_iq__vgdin_dn9 = assign14210_e14021_d_n9;
        locals.var_fn169_calc_iq__vgdin_dn10 = assign14210_e14021_d_n10;

        let (assign14220_e14025, assign14220_e14025_d_n2, assign14220_e14025_d_n4, assign14220_e14025_d_n7, assign14220_e14025_d_n9, assign14220_e14025_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__exparg0, locals.var_fn169_calc_iq__exparg0_dn2, locals.var_fn169_calc_iq__exparg0_dn4, locals.var_fn169_calc_iq__exparg0_dn7, locals.var_fn169_calc_iq__exparg0_dn9, locals.var_fn169_calc_iq__exparg0_dn10,)
    }
};
        locals.var_fn169_calc_iq__exparg0 = assign14220_e14025;
        locals.var_fn169_calc_iq__exparg0_dn2 = assign14220_e14025_d_n2;
        locals.var_fn169_calc_iq__exparg0_dn4 = assign14220_e14025_d_n4;
        locals.var_fn169_calc_iq__exparg0_dn7 = assign14220_e14025_d_n7;
        locals.var_fn169_calc_iq__exparg0_dn9 = assign14220_e14025_d_n9;
        locals.var_fn169_calc_iq__exparg0_dn10 = assign14220_e14025_d_n10;

        let (assign14230_e14029, assign14230_e14029_d_n4,) = {
    if (locals.var_guard168 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__myarg0, locals.var_fn169_calc_iq__myarg0_dn4,)
    }
};
        locals.var_fn169_calc_iq__myarg0 = assign14230_e14029;
        locals.var_fn169_calc_iq__myarg0_dn4 = assign14230_e14029_d_n4;

        let (assign14240_e14056, assign14240_e14056_d_n9, assign14240_e14056_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        let (assign14240_e14054, assign14240_e14054_d_n9, assign14240_e14054_d_n10,) = {
            if (p.p52 != 0.0) {
                let assign14240_e14038: f64 = (0.001 / p.p53);
                let assign14240_e14040: f64 = (assign14240_e14038 * locals.var_fn169_calc_iq__vdsin);
                let assign14240_e14041: f64 = (assign14240_e14040).tanh();
                let assign14240_e14042: f64 = (locals.var_fn169_calc_iq__vdsin * assign14240_e14041);
                (assign14240_e14042, ((locals.var_fn169_calc_iq__vdsin_dn9 * assign14240_e14041) + (locals.var_fn169_calc_iq__vdsin * ((assign14240_e14038 * locals.var_fn169_calc_iq__vdsin_dn9) / ((assign14240_e14040).cosh() * (assign14240_e14040).cosh())))), ((locals.var_fn169_calc_iq__vdsin_dn10 * assign14240_e14041) + (locals.var_fn169_calc_iq__vdsin * ((assign14240_e14038 * locals.var_fn169_calc_iq__vdsin_dn10) / ((assign14240_e14040).cosh() * (assign14240_e14040).cosh())))),)
            } else {
                let (assign14240_e14053, assign14240_e14053_d_n9, assign14240_e14053_d_n10,) = {
                    if (p.p52 == 0.0) {
                        let assign14240_e14048: f64 = (locals.var_fn169_calc_iq__vdsin * locals.var_fn169_calc_iq__vdsin);
                        let assign14240_e14050: f64 = (assign14240_e14048 + p.p53);
                        let assign14240_e14051: f64 = (assign14240_e14050).sqrt();
                        (assign14240_e14051, (((locals.var_fn169_calc_iq__vdsin_dn9 * locals.var_fn169_calc_iq__vdsin) + (locals.var_fn169_calc_iq__vdsin * locals.var_fn169_calc_iq__vdsin_dn9)) / (2.0 * assign14240_e14051)), (((locals.var_fn169_calc_iq__vdsin_dn10 * locals.var_fn169_calc_iq__vdsin) + (locals.var_fn169_calc_iq__vdsin * locals.var_fn169_calc_iq__vdsin_dn10)) / (2.0 * assign14240_e14051)),)
                    } else {
                        (0.0, 0.0, 0.0,)
                    }
                };
                (assign14240_e14053, assign14240_e14053_d_n9, assign14240_e14053_d_n10,)
            }
        };
        (assign14240_e14054, assign14240_e14054_d_n9, assign14240_e14054_d_n10,)
    } else {
        (locals.var_fn169_calc_iq__absvdsin, locals.var_fn169_calc_iq__absvdsin_dn9, locals.var_fn169_calc_iq__absvdsin_dn10,)
    }
};
        locals.var_fn169_calc_iq__absvdsin = assign14240_e14056;
        locals.var_fn169_calc_iq__absvdsin_dn9 = assign14240_e14056_d_n9;
        locals.var_fn169_calc_iq__absvdsin_dn10 = assign14240_e14056_d_n10;

        let (assign14250_e14062, assign14250_e14062_d_n2, assign14250_e14062_d_n7, assign14250_e14062_d_n9, assign14250_e14062_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        let assign14250_e14060: f64 = (locals.var_fn169_calc_iq__vgsin - locals.var_fn169_calc_iq__vdsin);
        (assign14250_e14060, locals.var_fn169_calc_iq__vgsin_dn2, locals.var_fn169_calc_iq__vgsin_dn7, (-locals.var_fn169_calc_iq__vdsin_dn9), (locals.var_fn169_calc_iq__vgsin_dn10 - locals.var_fn169_calc_iq__vdsin_dn10),)
    } else {
        (locals.var_fn169_calc_iq__vgdin, locals.var_fn169_calc_iq__vgdin_dn2, locals.var_fn169_calc_iq__vgdin_dn7, locals.var_fn169_calc_iq__vgdin_dn9, locals.var_fn169_calc_iq__vgdin_dn10,)
    }
};
        locals.var_fn169_calc_iq__vgdin = assign14250_e14062;
        locals.var_fn169_calc_iq__vgdin_dn2 = assign14250_e14062_d_n2;
        locals.var_fn169_calc_iq__vgdin_dn7 = assign14250_e14062_d_n7;
        locals.var_fn169_calc_iq__vgdin_dn9 = assign14250_e14062_d_n9;
        locals.var_fn169_calc_iq__vgdin_dn10 = assign14250_e14062_d_n10;

        let (assign14260_e14068, assign14260_e14068_d_n4,) = {
    if (locals.var_guard168 != 0.0) {
        let assign14260_e14066: f64 = (locals.var_fn169_calc_iq__alpha * locals.var_fn169_calc_iq__phitin);
        (assign14260_e14066, (locals.var_fn169_calc_iq__alpha * locals.var_fn169_calc_iq__phitin_dn4),)
    } else {
        (locals.var_fn169_calc_iq__alpha_phit, locals.var_fn169_calc_iq__alpha_phit_dn4,)
    }
};
        locals.var_fn169_calc_iq__alpha_phit = assign14260_e14068;
        locals.var_fn169_calc_iq__alpha_phit_dn4 = assign14260_e14068_d_n4;

        let (assign14270_e14080, assign14270_e14080_d_n4, assign14270_e14080_d_n9, assign14270_e14080_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        let assign14270_e14073: f64 = (2.302585092994046 * locals.var_fn169_calc_iq__phitin);
        let assign14270_e14074: f64 = (locals.var_fn169_calc_iq__ss / assign14270_e14073);
        let assign14270_e14077: f64 = (locals.var_fn169_calc_iq__nd * locals.var_fn169_calc_iq__absvdsin);
        let assign14270_e14078: f64 = (assign14270_e14074 + assign14270_e14077);
        (assign14270_e14078, (-((locals.var_fn169_calc_iq__ss * (2.302585092994046 * locals.var_fn169_calc_iq__phitin_dn4)) / (assign14270_e14073 * assign14270_e14073))), (locals.var_fn169_calc_iq__nd * locals.var_fn169_calc_iq__absvdsin_dn9), (locals.var_fn169_calc_iq__nd * locals.var_fn169_calc_iq__absvdsin_dn10),)
    } else {
        (locals.var_fn169_calc_iq__n, locals.var_fn169_calc_iq__n_dn4, locals.var_fn169_calc_iq__n_dn9, locals.var_fn169_calc_iq__n_dn10,)
    }
};
        locals.var_fn169_calc_iq__n = assign14270_e14080;
        locals.var_fn169_calc_iq__n_dn4 = assign14270_e14080_d_n4;
        locals.var_fn169_calc_iq__n_dn9 = assign14270_e14080_d_n9;
        locals.var_fn169_calc_iq__n_dn10 = assign14270_e14080_d_n10;

        let (assign14280_e14090, assign14280_e14090_d_n4,) = {
    if (locals.var_guard168 != 0.0) {
        let assign14280_e14086: f64 = (locals.var_fn169_calc_iq__tambin - locals.var_fn169_calc_iq__tnomin);
        let assign14280_e14087: f64 = (locals.var_fn169_calc_iq__vtzeta * assign14280_e14086);
        let assign14280_e14088: f64 = (locals.var_fn169_calc_iq__vto + assign14280_e14087);
        (assign14280_e14088, (locals.var_fn169_calc_iq__vtzeta * locals.var_fn169_calc_iq__tambin_dn4),)
    } else {
        (locals.var_fn169_calc_iq__vtof, locals.var_fn169_calc_iq__vtof_dn4,)
    }
};
        locals.var_fn169_calc_iq__vtof = assign14280_e14090;
        locals.var_fn169_calc_iq__vtof_dn4 = assign14280_e14090_d_n4;

    }

    pub(super) fn stamp_transient_block_35(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign14290_e14098, assign14290_e14098_d_n4,) = {
    if (locals.var_guard168 != 0.0) {
        let assign14290_e14094: f64 = (locals.var_fn169_calc_iq__tambin / locals.var_fn169_calc_iq__tnomin);
        let assign14290_e14096: f64 = (assign14290_e14094).powf(locals.var_fn169_calc_iq__epsilon);
        (assign14290_e14096, if 0.0 == 0.0 && ((locals.var_fn169_calc_iq__epsilon) as f64).is_finite() && ((locals.var_fn169_calc_iq__epsilon) as f64).fract() == 0.0 { if locals.var_fn169_calc_iq__epsilon == 0.0 { 0.0 } else { (locals.var_fn169_calc_iq__epsilon * ((assign14290_e14094).powf(locals.var_fn169_calc_iq__epsilon - 1.0) * (locals.var_fn169_calc_iq__tambin_dn4 / locals.var_fn169_calc_iq__tnomin))) } } else { (assign14290_e14096 * (locals.var_fn169_calc_iq__epsilon * ((locals.var_fn169_calc_iq__tambin_dn4 / locals.var_fn169_calc_iq__tnomin) / assign14290_e14094))) },)
    } else {
        (locals.var_fn169_calc_iq__tfacmobin, locals.var_fn169_calc_iq__tfacmobin_dn4,)
    }
};
        locals.var_fn169_calc_iq__tfacmobin = assign14290_e14098;
        locals.var_fn169_calc_iq__tfacmobin_dn4 = assign14290_e14098_d_n4;

        let assign14300_e14101: f64 = if locals.var_fn169_calc_iq__dibsat != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard170 = assign14300_e14101;

        let (assign14310_e14119, assign14310_e14119_d_n9, assign14310_e14119_d_n10,) = {
    if ((locals.var_guard168 != 0.0) && (locals.var_guard170 != 0.0)) {
        let assign14310_e14109: f64 = (locals.var_fn169_calc_iq__absvdsin / locals.var_fn169_calc_iq__dibsat);
        let assign14310_e14111: f64 = (assign14310_e14109).powf(locals.var_fn169_calc_iq__beta);
        let assign14310_e14112: f64 = (1.0 + assign14310_e14111);
        let assign14310_e14115: f64 = (1.0 / locals.var_fn169_calc_iq__beta);
        let assign14310_e14116: f64 = (assign14310_e14112).powf(assign14310_e14115);
        let assign14310_e14117: f64 = (locals.var_fn169_calc_iq__absvdsin / assign14310_e14116);
        (assign14310_e14117, (((locals.var_fn169_calc_iq__absvdsin_dn9 * assign14310_e14116) - (locals.var_fn169_calc_iq__absvdsin * if 0.0 == 0.0 && ((assign14310_e14115) as f64).is_finite() && ((assign14310_e14115) as f64).fract() == 0.0 { if assign14310_e14115 == 0.0 { 0.0 } else { (assign14310_e14115 * ((assign14310_e14112).powf(assign14310_e14115 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn169_calc_iq__beta) as f64).is_finite() && ((locals.var_fn169_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn169_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn169_calc_iq__beta * ((assign14310_e14109).powf(locals.var_fn169_calc_iq__beta - 1.0) * (locals.var_fn169_calc_iq__absvdsin_dn9 / locals.var_fn169_calc_iq__dibsat))) } } else { (assign14310_e14111 * (locals.var_fn169_calc_iq__beta * ((locals.var_fn169_calc_iq__absvdsin_dn9 / locals.var_fn169_calc_iq__dibsat) / assign14310_e14109))) })) } } else { (assign14310_e14116 * (assign14310_e14115 * (if 0.0 == 0.0 && ((locals.var_fn169_calc_iq__beta) as f64).is_finite() && ((locals.var_fn169_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn169_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn169_calc_iq__beta * ((assign14310_e14109).powf(locals.var_fn169_calc_iq__beta - 1.0) * (locals.var_fn169_calc_iq__absvdsin_dn9 / locals.var_fn169_calc_iq__dibsat))) } } else { (assign14310_e14111 * (locals.var_fn169_calc_iq__beta * ((locals.var_fn169_calc_iq__absvdsin_dn9 / locals.var_fn169_calc_iq__dibsat) / assign14310_e14109))) } / assign14310_e14112))) })) / (assign14310_e14116 * assign14310_e14116)), (((locals.var_fn169_calc_iq__absvdsin_dn10 * assign14310_e14116) - (locals.var_fn169_calc_iq__absvdsin * if 0.0 == 0.0 && ((assign14310_e14115) as f64).is_finite() && ((assign14310_e14115) as f64).fract() == 0.0 { if assign14310_e14115 == 0.0 { 0.0 } else { (assign14310_e14115 * ((assign14310_e14112).powf(assign14310_e14115 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn169_calc_iq__beta) as f64).is_finite() && ((locals.var_fn169_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn169_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn169_calc_iq__beta * ((assign14310_e14109).powf(locals.var_fn169_calc_iq__beta - 1.0) * (locals.var_fn169_calc_iq__absvdsin_dn10 / locals.var_fn169_calc_iq__dibsat))) } } else { (assign14310_e14111 * (locals.var_fn169_calc_iq__beta * ((locals.var_fn169_calc_iq__absvdsin_dn10 / locals.var_fn169_calc_iq__dibsat) / assign14310_e14109))) })) } } else { (assign14310_e14116 * (assign14310_e14115 * (if 0.0 == 0.0 && ((locals.var_fn169_calc_iq__beta) as f64).is_finite() && ((locals.var_fn169_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn169_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn169_calc_iq__beta * ((assign14310_e14109).powf(locals.var_fn169_calc_iq__beta - 1.0) * (locals.var_fn169_calc_iq__absvdsin_dn10 / locals.var_fn169_calc_iq__dibsat))) } } else { (assign14310_e14111 * (locals.var_fn169_calc_iq__beta * ((locals.var_fn169_calc_iq__absvdsin_dn10 / locals.var_fn169_calc_iq__dibsat) / assign14310_e14109))) } / assign14310_e14112))) })) / (assign14310_e14116 * assign14310_e14116)),)
    } else {
        (locals.var_fn169_calc_iq__vsatdibl, locals.var_fn169_calc_iq__vsatdibl_dn9, locals.var_fn169_calc_iq__vsatdibl_dn10,)
    }
};
        locals.var_fn169_calc_iq__vsatdibl = assign14310_e14119;
        locals.var_fn169_calc_iq__vsatdibl_dn9 = assign14310_e14119_d_n9;
        locals.var_fn169_calc_iq__vsatdibl_dn10 = assign14310_e14119_d_n10;

        let (assign14320_e14126, assign14320_e14126_d_n9, assign14320_e14126_d_n10,) = {
    if ((locals.var_guard168 != 0.0) && (locals.var_guard170 == 0.0)) {
        (0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__vsatdibl, locals.var_fn169_calc_iq__vsatdibl_dn9, locals.var_fn169_calc_iq__vsatdibl_dn10,)
    }
};
        locals.var_fn169_calc_iq__vsatdibl = assign14320_e14126;
        locals.var_fn169_calc_iq__vsatdibl_dn9 = assign14320_e14126_d_n9;
        locals.var_fn169_calc_iq__vsatdibl_dn10 = assign14320_e14126_d_n10;

        let (assign14330_e14136, assign14330_e14136_d_n9, assign14330_e14136_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        let assign14330_e14131: f64 = (locals.var_fn169_calc_iq__vsatdibl * locals.var_fn169_calc_iq__delta2);
        let assign14330_e14132: f64 = (locals.var_fn169_calc_iq__delta1 - assign14330_e14131);
        let assign14330_e14134: f64 = (assign14330_e14132 * locals.var_fn169_calc_iq__absvdsin);
        (assign14330_e14134, (((-(locals.var_fn169_calc_iq__vsatdibl_dn9 * locals.var_fn169_calc_iq__delta2)) * locals.var_fn169_calc_iq__absvdsin) + (assign14330_e14132 * locals.var_fn169_calc_iq__absvdsin_dn9)), (((-(locals.var_fn169_calc_iq__vsatdibl_dn10 * locals.var_fn169_calc_iq__delta2)) * locals.var_fn169_calc_iq__absvdsin) + (assign14330_e14132 * locals.var_fn169_calc_iq__absvdsin_dn10)),)
    } else {
        (locals.var_fn169_calc_iq__delta, locals.var_fn169_calc_iq__delta_dn9, locals.var_fn169_calc_iq__delta_dn10,)
    }
};
        locals.var_fn169_calc_iq__delta = assign14330_e14136;
        locals.var_fn169_calc_iq__delta_dn9 = assign14330_e14136_d_n9;
        locals.var_fn169_calc_iq__delta_dn10 = assign14330_e14136_d_n10;

        let (assign14340_e14142, assign14340_e14142_d_n4, assign14340_e14142_d_n9, assign14340_e14142_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        let assign14340_e14140: f64 = (locals.var_fn169_calc_iq__vtof - locals.var_fn169_calc_iq__delta);
        (assign14340_e14140, locals.var_fn169_calc_iq__vtof_dn4, (-locals.var_fn169_calc_iq__delta_dn9), (-locals.var_fn169_calc_iq__delta_dn10),)
    } else {
        (locals.var_fn169_calc_iq__vtdibl, locals.var_fn169_calc_iq__vtdibl_dn4, locals.var_fn169_calc_iq__vtdibl_dn9, locals.var_fn169_calc_iq__vtdibl_dn10,)
    }
};
        locals.var_fn169_calc_iq__vtdibl = assign14340_e14142;
        locals.var_fn169_calc_iq__vtdibl_dn4 = assign14340_e14142_d_n4;
        locals.var_fn169_calc_iq__vtdibl_dn9 = assign14340_e14142_d_n9;
        locals.var_fn169_calc_iq__vtdibl_dn10 = assign14340_e14142_d_n10;

        let (assign14350_e14150, assign14350_e14150_d_n4, assign14350_e14150_d_n9, assign14350_e14150_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        let assign14350_e14146: f64 = (2.0 * locals.var_fn169_calc_iq__n);
        let assign14350_e14148: f64 = (assign14350_e14146 * locals.var_fn169_calc_iq__phitin);
        (assign14350_e14148, (((2.0 * locals.var_fn169_calc_iq__n_dn4) * locals.var_fn169_calc_iq__phitin) + (assign14350_e14146 * locals.var_fn169_calc_iq__phitin_dn4)), ((2.0 * locals.var_fn169_calc_iq__n_dn9) * locals.var_fn169_calc_iq__phitin), ((2.0 * locals.var_fn169_calc_iq__n_dn10) * locals.var_fn169_calc_iq__phitin),)
    } else {
        (locals.var_fn169_calc_iq__two_n_phit, locals.var_fn169_calc_iq__two_n_phit_dn4, locals.var_fn169_calc_iq__two_n_phit_dn9, locals.var_fn169_calc_iq__two_n_phit_dn10,)
    }
};
        locals.var_fn169_calc_iq__two_n_phit = assign14350_e14150;
        locals.var_fn169_calc_iq__two_n_phit_dn4 = assign14350_e14150_d_n4;
        locals.var_fn169_calc_iq__two_n_phit_dn9 = assign14350_e14150_d_n9;
        locals.var_fn169_calc_iq__two_n_phit_dn10 = assign14350_e14150_d_n10;

        let (assign14360_e14156, assign14360_e14156_d_n4, assign14360_e14156_d_n9, assign14360_e14156_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        let assign14360_e14154: f64 = (locals.var_fn169_calc_iq__cgin * locals.var_fn169_calc_iq__two_n_phit);
        (assign14360_e14154, ((locals.var_fn169_calc_iq__cgin_dn4 * locals.var_fn169_calc_iq__two_n_phit) + (locals.var_fn169_calc_iq__cgin * locals.var_fn169_calc_iq__two_n_phit_dn4)), (locals.var_fn169_calc_iq__cgin * locals.var_fn169_calc_iq__two_n_phit_dn9), (locals.var_fn169_calc_iq__cgin * locals.var_fn169_calc_iq__two_n_phit_dn10),)
    } else {
        (locals.var_fn169_calc_iq__qref, locals.var_fn169_calc_iq__qref_dn4, locals.var_fn169_calc_iq__qref_dn9, locals.var_fn169_calc_iq__qref_dn10,)
    }
};
        locals.var_fn169_calc_iq__qref = assign14360_e14156;
        locals.var_fn169_calc_iq__qref_dn4 = assign14360_e14156_d_n4;
        locals.var_fn169_calc_iq__qref_dn9 = assign14360_e14156_d_n9;
        locals.var_fn169_calc_iq__qref_dn10 = assign14360_e14156_d_n10;

        let (assign14370_e14166, assign14370_e14166_d_n2, assign14370_e14166_d_n3, assign14370_e14166_d_n4, assign14370_e14166_d_n7, assign14370_e14166_d_n9, assign14370_e14166_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        let assign14370_e14161: f64 = (p.p51 * locals.var_fn169_calc_iq__alpha_phit);
        let assign14370_e14163: f64 = (assign14370_e14161 / 2.0);
        let assign14370_e14164: f64 = (locals.var_fn169_calc_iq__vtdibl - assign14370_e14163);
        (assign14370_e14164, 0.0, 0.0, (locals.var_fn169_calc_iq__vtdibl_dn4 - ((p.p51 * locals.var_fn169_calc_iq__alpha_phit_dn4) / 2.0)), 0.0, locals.var_fn169_calc_iq__vtdibl_dn9, locals.var_fn169_calc_iq__vtdibl_dn10,)
    } else {
        (locals.var_fn169_calc_iq__myarg, locals.var_fn169_calc_iq__myarg_dn2, locals.var_fn169_calc_iq__myarg_dn3, locals.var_fn169_calc_iq__myarg_dn4, locals.var_fn169_calc_iq__myarg_dn7, locals.var_fn169_calc_iq__myarg_dn9, locals.var_fn169_calc_iq__myarg_dn10,)
    }
};
        locals.var_fn169_calc_iq__myarg = assign14370_e14166;
        locals.var_fn169_calc_iq__myarg_dn2 = assign14370_e14166_d_n2;
        locals.var_fn169_calc_iq__myarg_dn3 = assign14370_e14166_d_n3;
        locals.var_fn169_calc_iq__myarg_dn4 = assign14370_e14166_d_n4;
        locals.var_fn169_calc_iq__myarg_dn7 = assign14370_e14166_d_n7;
        locals.var_fn169_calc_iq__myarg_dn9 = assign14370_e14166_d_n9;
        locals.var_fn169_calc_iq__myarg_dn10 = assign14370_e14166_d_n10;

        let (assign14380_e14217, assign14380_e14217_d_n2, assign14380_e14217_d_n3, assign14380_e14217_d_n4, assign14380_e14217_d_n7, assign14380_e14217_d_n9, assign14380_e14217_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        let (assign14380_e14211, assign14380_e14211_d_n2, assign14380_e14211_d_n7, assign14380_e14211_d_n9, assign14380_e14211_d_n10,) = {
            if (p.p52 != 0.0) {
                let assign14380_e14175: f64 = (locals.var_fn169_calc_iq__vgsin + locals.var_fn169_calc_iq__vgdin);
                let assign14380_e14178: f64 = (locals.var_fn169_calc_iq__vgsin - locals.var_fn169_calc_iq__vgdin);
                let assign14380_e14181: f64 = (0.001 / p.p53);
                let assign14380_e14184: f64 = (locals.var_fn169_calc_iq__vgsin - locals.var_fn169_calc_iq__vgdin);
                let assign14380_e14185: f64 = (assign14380_e14181 * assign14380_e14184);
                let assign14380_e14186: f64 = (assign14380_e14185).tanh();
                let assign14380_e14187: f64 = (assign14380_e14178 * assign14380_e14186);
                let assign14380_e14188: f64 = (assign14380_e14175 + assign14380_e14187);
                let assign14380_e14189: f64 = (0.5 * assign14380_e14188);
                (assign14380_e14189, (0.5 * ((locals.var_fn169_calc_iq__vgsin_dn2 + locals.var_fn169_calc_iq__vgdin_dn2) + (((locals.var_fn169_calc_iq__vgsin_dn2 - locals.var_fn169_calc_iq__vgdin_dn2) * assign14380_e14186) + (assign14380_e14178 * ((assign14380_e14181 * (locals.var_fn169_calc_iq__vgsin_dn2 - locals.var_fn169_calc_iq__vgdin_dn2)) / ((assign14380_e14185).cosh() * (assign14380_e14185).cosh())))))), (0.5 * ((locals.var_fn169_calc_iq__vgsin_dn7 + locals.var_fn169_calc_iq__vgdin_dn7) + (((locals.var_fn169_calc_iq__vgsin_dn7 - locals.var_fn169_calc_iq__vgdin_dn7) * assign14380_e14186) + (assign14380_e14178 * ((assign14380_e14181 * (locals.var_fn169_calc_iq__vgsin_dn7 - locals.var_fn169_calc_iq__vgdin_dn7)) / ((assign14380_e14185).cosh() * (assign14380_e14185).cosh())))))), (0.5 * (locals.var_fn169_calc_iq__vgdin_dn9 + (((-locals.var_fn169_calc_iq__vgdin_dn9) * assign14380_e14186) + (assign14380_e14178 * ((assign14380_e14181 * (-locals.var_fn169_calc_iq__vgdin_dn9)) / ((assign14380_e14185).cosh() * (assign14380_e14185).cosh())))))), (0.5 * ((locals.var_fn169_calc_iq__vgsin_dn10 + locals.var_fn169_calc_iq__vgdin_dn10) + (((locals.var_fn169_calc_iq__vgsin_dn10 - locals.var_fn169_calc_iq__vgdin_dn10) * assign14380_e14186) + (assign14380_e14178 * ((assign14380_e14181 * (locals.var_fn169_calc_iq__vgsin_dn10 - locals.var_fn169_calc_iq__vgdin_dn10)) / ((assign14380_e14185).cosh() * (assign14380_e14185).cosh())))))),)
            } else {
                let (assign14380_e14210, assign14380_e14210_d_n2, assign14380_e14210_d_n7, assign14380_e14210_d_n9, assign14380_e14210_d_n10,) = {
                    if (p.p52 == 0.0) {
                        let assign14380_e14196: f64 = (locals.var_fn169_calc_iq__vgsin + locals.var_fn169_calc_iq__vgdin);
                        let assign14380_e14199: f64 = (locals.var_fn169_calc_iq__vgsin - locals.var_fn169_calc_iq__vgdin);
                        let assign14380_e14202: f64 = (locals.var_fn169_calc_iq__vgsin - locals.var_fn169_calc_iq__vgdin);
                        let assign14380_e14203: f64 = (assign14380_e14199 * assign14380_e14202);
                        let assign14380_e14205: f64 = (assign14380_e14203 + p.p53);
                        let assign14380_e14206: f64 = (assign14380_e14205).sqrt();
                        let assign14380_e14207: f64 = (assign14380_e14196 + assign14380_e14206);
                        let assign14380_e14208: f64 = (0.5 * assign14380_e14207);
                        (assign14380_e14208, (0.5 * ((locals.var_fn169_calc_iq__vgsin_dn2 + locals.var_fn169_calc_iq__vgdin_dn2) + ((((locals.var_fn169_calc_iq__vgsin_dn2 - locals.var_fn169_calc_iq__vgdin_dn2) * assign14380_e14202) + (assign14380_e14199 * (locals.var_fn169_calc_iq__vgsin_dn2 - locals.var_fn169_calc_iq__vgdin_dn2))) / (2.0 * assign14380_e14206)))), (0.5 * ((locals.var_fn169_calc_iq__vgsin_dn7 + locals.var_fn169_calc_iq__vgdin_dn7) + ((((locals.var_fn169_calc_iq__vgsin_dn7 - locals.var_fn169_calc_iq__vgdin_dn7) * assign14380_e14202) + (assign14380_e14199 * (locals.var_fn169_calc_iq__vgsin_dn7 - locals.var_fn169_calc_iq__vgdin_dn7))) / (2.0 * assign14380_e14206)))), (0.5 * (locals.var_fn169_calc_iq__vgdin_dn9 + ((((-locals.var_fn169_calc_iq__vgdin_dn9) * assign14380_e14202) + (assign14380_e14199 * (-locals.var_fn169_calc_iq__vgdin_dn9))) / (2.0 * assign14380_e14206)))), (0.5 * ((locals.var_fn169_calc_iq__vgsin_dn10 + locals.var_fn169_calc_iq__vgdin_dn10) + ((((locals.var_fn169_calc_iq__vgsin_dn10 - locals.var_fn169_calc_iq__vgdin_dn10) * assign14380_e14202) + (assign14380_e14199 * (locals.var_fn169_calc_iq__vgsin_dn10 - locals.var_fn169_calc_iq__vgdin_dn10))) / (2.0 * assign14380_e14206)))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign14380_e14210, assign14380_e14210_d_n2, assign14380_e14210_d_n7, assign14380_e14210_d_n9, assign14380_e14210_d_n10,)
            }
        };
        let assign14380_e14213: f64 = (assign14380_e14211 - locals.var_fn169_calc_iq__myarg);
        let assign14380_e14215: f64 = (assign14380_e14213 / locals.var_fn169_calc_iq__alpha_phit);
        (assign14380_e14215, ((assign14380_e14211_d_n2 - locals.var_fn169_calc_iq__myarg_dn2) / locals.var_fn169_calc_iq__alpha_phit), ((-locals.var_fn169_calc_iq__myarg_dn3) / locals.var_fn169_calc_iq__alpha_phit), ((((-locals.var_fn169_calc_iq__myarg_dn4) * locals.var_fn169_calc_iq__alpha_phit) - (assign14380_e14213 * locals.var_fn169_calc_iq__alpha_phit_dn4)) / (locals.var_fn169_calc_iq__alpha_phit * locals.var_fn169_calc_iq__alpha_phit)), ((assign14380_e14211_d_n7 - locals.var_fn169_calc_iq__myarg_dn7) / locals.var_fn169_calc_iq__alpha_phit), ((assign14380_e14211_d_n9 - locals.var_fn169_calc_iq__myarg_dn9) / locals.var_fn169_calc_iq__alpha_phit), ((assign14380_e14211_d_n10 - locals.var_fn169_calc_iq__myarg_dn10) / locals.var_fn169_calc_iq__alpha_phit),)
    } else {
        (locals.var_fn169_calc_iq__exparg, locals.var_fn169_calc_iq__exparg_dn2, locals.var_fn169_calc_iq__exparg_dn3, locals.var_fn169_calc_iq__exparg_dn4, locals.var_fn169_calc_iq__exparg_dn7, locals.var_fn169_calc_iq__exparg_dn9, locals.var_fn169_calc_iq__exparg_dn10,)
    }
};
        locals.var_fn169_calc_iq__exparg = assign14380_e14217;
        locals.var_fn169_calc_iq__exparg_dn2 = assign14380_e14217_d_n2;
        locals.var_fn169_calc_iq__exparg_dn3 = assign14380_e14217_d_n3;
        locals.var_fn169_calc_iq__exparg_dn4 = assign14380_e14217_d_n4;
        locals.var_fn169_calc_iq__exparg_dn7 = assign14380_e14217_d_n7;
        locals.var_fn169_calc_iq__exparg_dn9 = assign14380_e14217_d_n9;
        locals.var_fn169_calc_iq__exparg_dn10 = assign14380_e14217_d_n10;

        let assign14390_e14220: f64 = if locals.var_fn169_calc_iq__exparg > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard171 = assign14390_e14220;

        let (assign14400_e14226, assign14400_e14226_d_n2, assign14400_e14226_d_n3, assign14400_e14226_d_n4, assign14400_e14226_d_n7, assign14400_e14226_d_n9, assign14400_e14226_d_n10,) = {
    if ((locals.var_guard168 != 0.0) && (locals.var_guard171 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__ff, locals.var_fn169_calc_iq__ff_dn2, locals.var_fn169_calc_iq__ff_dn3, locals.var_fn169_calc_iq__ff_dn4, locals.var_fn169_calc_iq__ff_dn7, locals.var_fn169_calc_iq__ff_dn9, locals.var_fn169_calc_iq__ff_dn10,)
    }
};
        locals.var_fn169_calc_iq__ff = assign14400_e14226;
        locals.var_fn169_calc_iq__ff_dn2 = assign14400_e14226_d_n2;
        locals.var_fn169_calc_iq__ff_dn3 = assign14400_e14226_d_n3;
        locals.var_fn169_calc_iq__ff_dn4 = assign14400_e14226_d_n4;
        locals.var_fn169_calc_iq__ff_dn7 = assign14400_e14226_d_n7;
        locals.var_fn169_calc_iq__ff_dn9 = assign14400_e14226_d_n9;
        locals.var_fn169_calc_iq__ff_dn10 = assign14400_e14226_d_n10;

        let assign14410_e14229: f64 = (-50.0);
        let assign14410_e14230: f64 = if locals.var_fn169_calc_iq__exparg < assign14410_e14229 { 1.0 } else { 0.0 };
        locals.var_guard172 = assign14410_e14230;

        let (assign14420_e14239, assign14420_e14239_d_n2, assign14420_e14239_d_n3, assign14420_e14239_d_n4, assign14420_e14239_d_n7, assign14420_e14239_d_n9, assign14420_e14239_d_n10,) = {
    if (((locals.var_guard168 != 0.0) && (locals.var_guard171 == 0.0)) && (locals.var_guard172 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__ff, locals.var_fn169_calc_iq__ff_dn2, locals.var_fn169_calc_iq__ff_dn3, locals.var_fn169_calc_iq__ff_dn4, locals.var_fn169_calc_iq__ff_dn7, locals.var_fn169_calc_iq__ff_dn9, locals.var_fn169_calc_iq__ff_dn10,)
    }
};
        locals.var_fn169_calc_iq__ff = assign14420_e14239;
        locals.var_fn169_calc_iq__ff_dn2 = assign14420_e14239_d_n2;
        locals.var_fn169_calc_iq__ff_dn3 = assign14420_e14239_d_n3;
        locals.var_fn169_calc_iq__ff_dn4 = assign14420_e14239_d_n4;
        locals.var_fn169_calc_iq__ff_dn7 = assign14420_e14239_d_n7;
        locals.var_fn169_calc_iq__ff_dn9 = assign14420_e14239_d_n9;
        locals.var_fn169_calc_iq__ff_dn10 = assign14420_e14239_d_n10;

        let (assign14430_e14254, assign14430_e14254_d_n2, assign14430_e14254_d_n3, assign14430_e14254_d_n4, assign14430_e14254_d_n7, assign14430_e14254_d_n9, assign14430_e14254_d_n10,) = {
    if (((locals.var_guard168 != 0.0) && (locals.var_guard171 == 0.0)) && (locals.var_guard172 == 0.0)) {
        let assign14430_e14250: f64 = (locals.var_fn169_calc_iq__exparg).exp();
        let assign14430_e14251: f64 = (1.0 + assign14430_e14250);
        let assign14430_e14252: f64 = (1.0 / assign14430_e14251);
        (assign14430_e14252, (-((assign14430_e14250 * locals.var_fn169_calc_iq__exparg_dn2) / (assign14430_e14251 * assign14430_e14251))), (-((assign14430_e14250 * locals.var_fn169_calc_iq__exparg_dn3) / (assign14430_e14251 * assign14430_e14251))), (-((assign14430_e14250 * locals.var_fn169_calc_iq__exparg_dn4) / (assign14430_e14251 * assign14430_e14251))), (-((assign14430_e14250 * locals.var_fn169_calc_iq__exparg_dn7) / (assign14430_e14251 * assign14430_e14251))), (-((assign14430_e14250 * locals.var_fn169_calc_iq__exparg_dn9) / (assign14430_e14251 * assign14430_e14251))), (-((assign14430_e14250 * locals.var_fn169_calc_iq__exparg_dn10) / (assign14430_e14251 * assign14430_e14251))),)
    } else {
        (locals.var_fn169_calc_iq__ff, locals.var_fn169_calc_iq__ff_dn2, locals.var_fn169_calc_iq__ff_dn3, locals.var_fn169_calc_iq__ff_dn4, locals.var_fn169_calc_iq__ff_dn7, locals.var_fn169_calc_iq__ff_dn9, locals.var_fn169_calc_iq__ff_dn10,)
    }
};
        locals.var_fn169_calc_iq__ff = assign14430_e14254;
        locals.var_fn169_calc_iq__ff_dn2 = assign14430_e14254_d_n2;
        locals.var_fn169_calc_iq__ff_dn3 = assign14430_e14254_d_n3;
        locals.var_fn169_calc_iq__ff_dn4 = assign14430_e14254_d_n4;
        locals.var_fn169_calc_iq__ff_dn7 = assign14430_e14254_d_n7;
        locals.var_fn169_calc_iq__ff_dn9 = assign14430_e14254_d_n9;
        locals.var_fn169_calc_iq__ff_dn10 = assign14430_e14254_d_n10;

        let (assign14440_e14313, assign14440_e14313_d_n2, assign14440_e14313_d_n3, assign14440_e14313_d_n4, assign14440_e14313_d_n7, assign14440_e14313_d_n9, assign14440_e14313_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        let (assign14440_e14299, assign14440_e14299_d_n2, assign14440_e14299_d_n7, assign14440_e14299_d_n9, assign14440_e14299_d_n10,) = {
            if (p.p52 != 0.0) {
                let assign14440_e14263: f64 = (locals.var_fn169_calc_iq__vgsin + locals.var_fn169_calc_iq__vgdin);
                let assign14440_e14266: f64 = (locals.var_fn169_calc_iq__vgsin - locals.var_fn169_calc_iq__vgdin);
                let assign14440_e14269: f64 = (0.001 / p.p53);
                let assign14440_e14272: f64 = (locals.var_fn169_calc_iq__vgsin - locals.var_fn169_calc_iq__vgdin);
                let assign14440_e14273: f64 = (assign14440_e14269 * assign14440_e14272);
                let assign14440_e14274: f64 = (assign14440_e14273).tanh();
                let assign14440_e14275: f64 = (assign14440_e14266 * assign14440_e14274);
                let assign14440_e14276: f64 = (assign14440_e14263 + assign14440_e14275);
                let assign14440_e14277: f64 = (0.5 * assign14440_e14276);
                (assign14440_e14277, (0.5 * ((locals.var_fn169_calc_iq__vgsin_dn2 + locals.var_fn169_calc_iq__vgdin_dn2) + (((locals.var_fn169_calc_iq__vgsin_dn2 - locals.var_fn169_calc_iq__vgdin_dn2) * assign14440_e14274) + (assign14440_e14266 * ((assign14440_e14269 * (locals.var_fn169_calc_iq__vgsin_dn2 - locals.var_fn169_calc_iq__vgdin_dn2)) / ((assign14440_e14273).cosh() * (assign14440_e14273).cosh())))))), (0.5 * ((locals.var_fn169_calc_iq__vgsin_dn7 + locals.var_fn169_calc_iq__vgdin_dn7) + (((locals.var_fn169_calc_iq__vgsin_dn7 - locals.var_fn169_calc_iq__vgdin_dn7) * assign14440_e14274) + (assign14440_e14266 * ((assign14440_e14269 * (locals.var_fn169_calc_iq__vgsin_dn7 - locals.var_fn169_calc_iq__vgdin_dn7)) / ((assign14440_e14273).cosh() * (assign14440_e14273).cosh())))))), (0.5 * (locals.var_fn169_calc_iq__vgdin_dn9 + (((-locals.var_fn169_calc_iq__vgdin_dn9) * assign14440_e14274) + (assign14440_e14266 * ((assign14440_e14269 * (-locals.var_fn169_calc_iq__vgdin_dn9)) / ((assign14440_e14273).cosh() * (assign14440_e14273).cosh())))))), (0.5 * ((locals.var_fn169_calc_iq__vgsin_dn10 + locals.var_fn169_calc_iq__vgdin_dn10) + (((locals.var_fn169_calc_iq__vgsin_dn10 - locals.var_fn169_calc_iq__vgdin_dn10) * assign14440_e14274) + (assign14440_e14266 * ((assign14440_e14269 * (locals.var_fn169_calc_iq__vgsin_dn10 - locals.var_fn169_calc_iq__vgdin_dn10)) / ((assign14440_e14273).cosh() * (assign14440_e14273).cosh())))))),)
            } else {
                let (assign14440_e14298, assign14440_e14298_d_n2, assign14440_e14298_d_n7, assign14440_e14298_d_n9, assign14440_e14298_d_n10,) = {
                    if (p.p52 == 0.0) {
                        let assign14440_e14284: f64 = (locals.var_fn169_calc_iq__vgsin + locals.var_fn169_calc_iq__vgdin);
                        let assign14440_e14287: f64 = (locals.var_fn169_calc_iq__vgsin - locals.var_fn169_calc_iq__vgdin);
                        let assign14440_e14290: f64 = (locals.var_fn169_calc_iq__vgsin - locals.var_fn169_calc_iq__vgdin);
                        let assign14440_e14291: f64 = (assign14440_e14287 * assign14440_e14290);
                        let assign14440_e14293: f64 = (assign14440_e14291 + p.p53);
                        let assign14440_e14294: f64 = (assign14440_e14293).sqrt();
                        let assign14440_e14295: f64 = (assign14440_e14284 + assign14440_e14294);
                        let assign14440_e14296: f64 = (0.5 * assign14440_e14295);
                        (assign14440_e14296, (0.5 * ((locals.var_fn169_calc_iq__vgsin_dn2 + locals.var_fn169_calc_iq__vgdin_dn2) + ((((locals.var_fn169_calc_iq__vgsin_dn2 - locals.var_fn169_calc_iq__vgdin_dn2) * assign14440_e14290) + (assign14440_e14287 * (locals.var_fn169_calc_iq__vgsin_dn2 - locals.var_fn169_calc_iq__vgdin_dn2))) / (2.0 * assign14440_e14294)))), (0.5 * ((locals.var_fn169_calc_iq__vgsin_dn7 + locals.var_fn169_calc_iq__vgdin_dn7) + ((((locals.var_fn169_calc_iq__vgsin_dn7 - locals.var_fn169_calc_iq__vgdin_dn7) * assign14440_e14290) + (assign14440_e14287 * (locals.var_fn169_calc_iq__vgsin_dn7 - locals.var_fn169_calc_iq__vgdin_dn7))) / (2.0 * assign14440_e14294)))), (0.5 * (locals.var_fn169_calc_iq__vgdin_dn9 + ((((-locals.var_fn169_calc_iq__vgdin_dn9) * assign14440_e14290) + (assign14440_e14287 * (-locals.var_fn169_calc_iq__vgdin_dn9))) / (2.0 * assign14440_e14294)))), (0.5 * ((locals.var_fn169_calc_iq__vgsin_dn10 + locals.var_fn169_calc_iq__vgdin_dn10) + ((((locals.var_fn169_calc_iq__vgsin_dn10 - locals.var_fn169_calc_iq__vgdin_dn10) * assign14440_e14290) + (assign14440_e14287 * (locals.var_fn169_calc_iq__vgsin_dn10 - locals.var_fn169_calc_iq__vgdin_dn10))) / (2.0 * assign14440_e14294)))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign14440_e14298, assign14440_e14298_d_n2, assign14440_e14298_d_n7, assign14440_e14298_d_n9, assign14440_e14298_d_n10,)
            }
        };
        let assign14440_e14303: f64 = (p.p51 * 0.1);
        let assign14440_e14305: f64 = (assign14440_e14303 * locals.var_fn169_calc_iq__alpha_phit);
        let assign14440_e14307: f64 = (assign14440_e14305 * locals.var_fn169_calc_iq__ff);
        let assign14440_e14308: f64 = (locals.var_fn169_calc_iq__vtdibl - assign14440_e14307);
        let assign14440_e14309: f64 = (assign14440_e14299 - assign14440_e14308);
        let assign14440_e14311: f64 = (assign14440_e14309 / locals.var_fn169_calc_iq__two_n_phit);
        (assign14440_e14311, ((assign14440_e14299_d_n2 - (-(assign14440_e14305 * locals.var_fn169_calc_iq__ff_dn2))) / locals.var_fn169_calc_iq__two_n_phit), ((-(-(assign14440_e14305 * locals.var_fn169_calc_iq__ff_dn3))) / locals.var_fn169_calc_iq__two_n_phit), ((((-(locals.var_fn169_calc_iq__vtdibl_dn4 - (((assign14440_e14303 * locals.var_fn169_calc_iq__alpha_phit_dn4) * locals.var_fn169_calc_iq__ff) + (assign14440_e14305 * locals.var_fn169_calc_iq__ff_dn4)))) * locals.var_fn169_calc_iq__two_n_phit) - (assign14440_e14309 * locals.var_fn169_calc_iq__two_n_phit_dn4)) / (locals.var_fn169_calc_iq__two_n_phit * locals.var_fn169_calc_iq__two_n_phit)), ((assign14440_e14299_d_n7 - (-(assign14440_e14305 * locals.var_fn169_calc_iq__ff_dn7))) / locals.var_fn169_calc_iq__two_n_phit), ((((assign14440_e14299_d_n9 - (locals.var_fn169_calc_iq__vtdibl_dn9 - (assign14440_e14305 * locals.var_fn169_calc_iq__ff_dn9))) * locals.var_fn169_calc_iq__two_n_phit) - (assign14440_e14309 * locals.var_fn169_calc_iq__two_n_phit_dn9)) / (locals.var_fn169_calc_iq__two_n_phit * locals.var_fn169_calc_iq__two_n_phit)), ((((assign14440_e14299_d_n10 - (locals.var_fn169_calc_iq__vtdibl_dn10 - (assign14440_e14305 * locals.var_fn169_calc_iq__ff_dn10))) * locals.var_fn169_calc_iq__two_n_phit) - (assign14440_e14309 * locals.var_fn169_calc_iq__two_n_phit_dn10)) / (locals.var_fn169_calc_iq__two_n_phit * locals.var_fn169_calc_iq__two_n_phit)),)
    } else {
        (locals.var_fn169_calc_iq__eta, locals.var_fn169_calc_iq__eta_dn2, locals.var_fn169_calc_iq__eta_dn3, locals.var_fn169_calc_iq__eta_dn4, locals.var_fn169_calc_iq__eta_dn7, locals.var_fn169_calc_iq__eta_dn9, locals.var_fn169_calc_iq__eta_dn10,)
    }
};
        locals.var_fn169_calc_iq__eta = assign14440_e14313;
        locals.var_fn169_calc_iq__eta_dn2 = assign14440_e14313_d_n2;
        locals.var_fn169_calc_iq__eta_dn3 = assign14440_e14313_d_n3;
        locals.var_fn169_calc_iq__eta_dn4 = assign14440_e14313_d_n4;
        locals.var_fn169_calc_iq__eta_dn7 = assign14440_e14313_d_n7;
        locals.var_fn169_calc_iq__eta_dn9 = assign14440_e14313_d_n9;
        locals.var_fn169_calc_iq__eta_dn10 = assign14440_e14313_d_n10;

        let assign14450_e14316: f64 = if locals.var_fn169_calc_iq__eta > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard173 = assign14450_e14316;

        let (assign14460_e14324, assign14460_e14324_d_n2, assign14460_e14324_d_n3, assign14460_e14324_d_n4, assign14460_e14324_d_n7, assign14460_e14324_d_n9, assign14460_e14324_d_n10,) = {
    if ((locals.var_guard168 != 0.0) && (locals.var_guard173 != 0.0)) {
        let assign14460_e14322: f64 = (locals.var_fn169_calc_iq__qref * locals.var_fn169_calc_iq__eta);
        (assign14460_e14322, (locals.var_fn169_calc_iq__qref * locals.var_fn169_calc_iq__eta_dn2), (locals.var_fn169_calc_iq__qref * locals.var_fn169_calc_iq__eta_dn3), ((locals.var_fn169_calc_iq__qref_dn4 * locals.var_fn169_calc_iq__eta) + (locals.var_fn169_calc_iq__qref * locals.var_fn169_calc_iq__eta_dn4)), (locals.var_fn169_calc_iq__qref * locals.var_fn169_calc_iq__eta_dn7), ((locals.var_fn169_calc_iq__qref_dn9 * locals.var_fn169_calc_iq__eta) + (locals.var_fn169_calc_iq__qref * locals.var_fn169_calc_iq__eta_dn9)), ((locals.var_fn169_calc_iq__qref_dn10 * locals.var_fn169_calc_iq__eta) + (locals.var_fn169_calc_iq__qref * locals.var_fn169_calc_iq__eta_dn10)),)
    } else {
        (locals.var_fn169_calc_iq__qinvv, locals.var_fn169_calc_iq__qinvv_dn2, locals.var_fn169_calc_iq__qinvv_dn3, locals.var_fn169_calc_iq__qinvv_dn4, locals.var_fn169_calc_iq__qinvv_dn7, locals.var_fn169_calc_iq__qinvv_dn9, locals.var_fn169_calc_iq__qinvv_dn10,)
    }
};
        locals.var_fn169_calc_iq__qinvv = assign14460_e14324;
        locals.var_fn169_calc_iq__qinvv_dn2 = assign14460_e14324_d_n2;
        locals.var_fn169_calc_iq__qinvv_dn3 = assign14460_e14324_d_n3;
        locals.var_fn169_calc_iq__qinvv_dn4 = assign14460_e14324_d_n4;
        locals.var_fn169_calc_iq__qinvv_dn7 = assign14460_e14324_d_n7;
        locals.var_fn169_calc_iq__qinvv_dn9 = assign14460_e14324_d_n9;
        locals.var_fn169_calc_iq__qinvv_dn10 = assign14460_e14324_d_n10;

        let assign14470_e14327: f64 = (-50.0);
        let assign14470_e14328: f64 = if locals.var_fn169_calc_iq__eta < assign14470_e14327 { 1.0 } else { 0.0 };
        locals.var_guard174 = assign14470_e14328;

        let (assign14480_e14340, assign14480_e14340_d_n2, assign14480_e14340_d_n3, assign14480_e14340_d_n4, assign14480_e14340_d_n7, assign14480_e14340_d_n9, assign14480_e14340_d_n10,) = {
    if (((locals.var_guard168 != 0.0) && (locals.var_guard173 == 0.0)) && (locals.var_guard174 != 0.0)) {
        let assign14480_e14337: f64 = (locals.var_fn169_calc_iq__eta).exp();
        let assign14480_e14338: f64 = (locals.var_fn169_calc_iq__qref * assign14480_e14337);
        (assign14480_e14338, (locals.var_fn169_calc_iq__qref * (assign14480_e14337 * locals.var_fn169_calc_iq__eta_dn2)), (locals.var_fn169_calc_iq__qref * (assign14480_e14337 * locals.var_fn169_calc_iq__eta_dn3)), ((locals.var_fn169_calc_iq__qref_dn4 * assign14480_e14337) + (locals.var_fn169_calc_iq__qref * (assign14480_e14337 * locals.var_fn169_calc_iq__eta_dn4))), (locals.var_fn169_calc_iq__qref * (assign14480_e14337 * locals.var_fn169_calc_iq__eta_dn7)), ((locals.var_fn169_calc_iq__qref_dn9 * assign14480_e14337) + (locals.var_fn169_calc_iq__qref * (assign14480_e14337 * locals.var_fn169_calc_iq__eta_dn9))), ((locals.var_fn169_calc_iq__qref_dn10 * assign14480_e14337) + (locals.var_fn169_calc_iq__qref * (assign14480_e14337 * locals.var_fn169_calc_iq__eta_dn10))),)
    } else {
        (locals.var_fn169_calc_iq__qinvv, locals.var_fn169_calc_iq__qinvv_dn2, locals.var_fn169_calc_iq__qinvv_dn3, locals.var_fn169_calc_iq__qinvv_dn4, locals.var_fn169_calc_iq__qinvv_dn7, locals.var_fn169_calc_iq__qinvv_dn9, locals.var_fn169_calc_iq__qinvv_dn10,)
    }
};
        locals.var_fn169_calc_iq__qinvv = assign14480_e14340;
        locals.var_fn169_calc_iq__qinvv_dn2 = assign14480_e14340_d_n2;
        locals.var_fn169_calc_iq__qinvv_dn3 = assign14480_e14340_d_n3;
        locals.var_fn169_calc_iq__qinvv_dn4 = assign14480_e14340_d_n4;
        locals.var_fn169_calc_iq__qinvv_dn7 = assign14480_e14340_d_n7;
        locals.var_fn169_calc_iq__qinvv_dn9 = assign14480_e14340_d_n9;
        locals.var_fn169_calc_iq__qinvv_dn10 = assign14480_e14340_d_n10;

        let (assign14490_e14356, assign14490_e14356_d_n2, assign14490_e14356_d_n3, assign14490_e14356_d_n4, assign14490_e14356_d_n7, assign14490_e14356_d_n9, assign14490_e14356_d_n10,) = {
    if (((locals.var_guard168 != 0.0) && (locals.var_guard173 == 0.0)) && (locals.var_guard174 == 0.0)) {
        let assign14490_e14351: f64 = (locals.var_fn169_calc_iq__eta).exp();
        let assign14490_e14352: f64 = (1.0 + assign14490_e14351);
        let assign14490_e14353: f64 = (assign14490_e14352).ln();
        let assign14490_e14354: f64 = (locals.var_fn169_calc_iq__qref * assign14490_e14353);
        (assign14490_e14354, (locals.var_fn169_calc_iq__qref * ((assign14490_e14351 * locals.var_fn169_calc_iq__eta_dn2) / assign14490_e14352)), (locals.var_fn169_calc_iq__qref * ((assign14490_e14351 * locals.var_fn169_calc_iq__eta_dn3) / assign14490_e14352)), ((locals.var_fn169_calc_iq__qref_dn4 * assign14490_e14353) + (locals.var_fn169_calc_iq__qref * ((assign14490_e14351 * locals.var_fn169_calc_iq__eta_dn4) / assign14490_e14352))), (locals.var_fn169_calc_iq__qref * ((assign14490_e14351 * locals.var_fn169_calc_iq__eta_dn7) / assign14490_e14352)), ((locals.var_fn169_calc_iq__qref_dn9 * assign14490_e14353) + (locals.var_fn169_calc_iq__qref * ((assign14490_e14351 * locals.var_fn169_calc_iq__eta_dn9) / assign14490_e14352))), ((locals.var_fn169_calc_iq__qref_dn10 * assign14490_e14353) + (locals.var_fn169_calc_iq__qref * ((assign14490_e14351 * locals.var_fn169_calc_iq__eta_dn10) / assign14490_e14352))),)
    } else {
        (locals.var_fn169_calc_iq__qinvv, locals.var_fn169_calc_iq__qinvv_dn2, locals.var_fn169_calc_iq__qinvv_dn3, locals.var_fn169_calc_iq__qinvv_dn4, locals.var_fn169_calc_iq__qinvv_dn7, locals.var_fn169_calc_iq__qinvv_dn9, locals.var_fn169_calc_iq__qinvv_dn10,)
    }
};
        locals.var_fn169_calc_iq__qinvv = assign14490_e14356;
        locals.var_fn169_calc_iq__qinvv_dn2 = assign14490_e14356_d_n2;
        locals.var_fn169_calc_iq__qinvv_dn3 = assign14490_e14356_d_n3;
        locals.var_fn169_calc_iq__qinvv_dn4 = assign14490_e14356_d_n4;
        locals.var_fn169_calc_iq__qinvv_dn7 = assign14490_e14356_d_n7;
        locals.var_fn169_calc_iq__qinvv_dn9 = assign14490_e14356_d_n9;
        locals.var_fn169_calc_iq__qinvv_dn10 = assign14490_e14356_d_n10;

        let (assign14500_e14370, assign14500_e14370_d_n2, assign14500_e14370_d_n3, assign14500_e14370_d_n4, assign14500_e14370_d_n7, assign14500_e14370_d_n9, assign14500_e14370_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        let assign14500_e14363: f64 = (locals.var_fn169_calc_iq__mtheta * locals.var_fn169_calc_iq__qinvv);
        let assign14500_e14365: f64 = (assign14500_e14363 / locals.var_fn169_calc_iq__cgin);
        let assign14500_e14366: f64 = (1.0 + assign14500_e14365);
        let assign14500_e14367: f64 = (locals.var_fn169_calc_iq__tfacmobin * assign14500_e14366);
        let assign14500_e14368: f64 = (locals.var_fn169_calc_iq__mu0 / assign14500_e14367);
        (assign14500_e14368, (-((locals.var_fn169_calc_iq__mu0 * (locals.var_fn169_calc_iq__tfacmobin * ((locals.var_fn169_calc_iq__mtheta * locals.var_fn169_calc_iq__qinvv_dn2) / locals.var_fn169_calc_iq__cgin))) / (assign14500_e14367 * assign14500_e14367))), (-((locals.var_fn169_calc_iq__mu0 * (locals.var_fn169_calc_iq__tfacmobin * ((locals.var_fn169_calc_iq__mtheta * locals.var_fn169_calc_iq__qinvv_dn3) / locals.var_fn169_calc_iq__cgin))) / (assign14500_e14367 * assign14500_e14367))), (-((locals.var_fn169_calc_iq__mu0 * ((locals.var_fn169_calc_iq__tfacmobin_dn4 * assign14500_e14366) + (locals.var_fn169_calc_iq__tfacmobin * ((((locals.var_fn169_calc_iq__mtheta * locals.var_fn169_calc_iq__qinvv_dn4) * locals.var_fn169_calc_iq__cgin) - (assign14500_e14363 * locals.var_fn169_calc_iq__cgin_dn4)) / (locals.var_fn169_calc_iq__cgin * locals.var_fn169_calc_iq__cgin))))) / (assign14500_e14367 * assign14500_e14367))), (-((locals.var_fn169_calc_iq__mu0 * (locals.var_fn169_calc_iq__tfacmobin * ((locals.var_fn169_calc_iq__mtheta * locals.var_fn169_calc_iq__qinvv_dn7) / locals.var_fn169_calc_iq__cgin))) / (assign14500_e14367 * assign14500_e14367))), (-((locals.var_fn169_calc_iq__mu0 * (locals.var_fn169_calc_iq__tfacmobin * ((locals.var_fn169_calc_iq__mtheta * locals.var_fn169_calc_iq__qinvv_dn9) / locals.var_fn169_calc_iq__cgin))) / (assign14500_e14367 * assign14500_e14367))), (-((locals.var_fn169_calc_iq__mu0 * (locals.var_fn169_calc_iq__tfacmobin * ((locals.var_fn169_calc_iq__mtheta * locals.var_fn169_calc_iq__qinvv_dn10) / locals.var_fn169_calc_iq__cgin))) / (assign14500_e14367 * assign14500_e14367))),)
    } else {
        (locals.var_fn169_calc_iq__muf, locals.var_fn169_calc_iq__muf_dn2, locals.var_fn169_calc_iq__muf_dn3, locals.var_fn169_calc_iq__muf_dn4, locals.var_fn169_calc_iq__muf_dn7, locals.var_fn169_calc_iq__muf_dn9, locals.var_fn169_calc_iq__muf_dn10,)
    }
};
        locals.var_fn169_calc_iq__muf = assign14500_e14370;
        locals.var_fn169_calc_iq__muf_dn2 = assign14500_e14370_d_n2;
        locals.var_fn169_calc_iq__muf_dn3 = assign14500_e14370_d_n3;
        locals.var_fn169_calc_iq__muf_dn4 = assign14500_e14370_d_n4;
        locals.var_fn169_calc_iq__muf_dn7 = assign14500_e14370_d_n7;
        locals.var_fn169_calc_iq__muf_dn9 = assign14500_e14370_d_n9;
        locals.var_fn169_calc_iq__muf_dn10 = assign14500_e14370_d_n10;

        let (assign14510_e14402, assign14510_e14402_d_n2, assign14510_e14402_d_n3, assign14510_e14402_d_n4, assign14510_e14402_d_n7, assign14510_e14402_d_n9, assign14510_e14402_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        let assign14510_e14376: f64 = (locals.var_fn169_calc_iq__vzeta * locals.var_fn169_calc_iq__tnomin);
        let assign14510_e14377: f64 = (1.0 + assign14510_e14376);
        let assign14510_e14381: f64 = (locals.var_fn169_calc_iq__vzeta * locals.var_fn169_calc_iq__tambin);
        let assign14510_e14382: f64 = (1.0 + assign14510_e14381);
        let assign14510_e14383: f64 = (assign14510_e14377 / assign14510_e14382);
        let assign14510_e14384: f64 = (locals.var_fn169_calc_iq__vel0 * assign14510_e14383);
        let assign14510_e14388: f64 = (locals.var_fn169_calc_iq__lambda * locals.var_fn169_calc_iq__absvdsin);
        let assign14510_e14390: f64 = (assign14510_e14388 / locals.var_fn169_calc_iq__lin);
        let assign14510_e14391: f64 = (1.0 + assign14510_e14390);
        let assign14510_e14392: f64 = (assign14510_e14384 * assign14510_e14391);
        let assign14510_e14396: f64 = (locals.var_fn169_calc_iq__vtheta * locals.var_fn169_calc_iq__qinvv);
        let assign14510_e14398: f64 = (assign14510_e14396 / locals.var_fn169_calc_iq__cgin);
        let assign14510_e14399: f64 = (1.0 + assign14510_e14398);
        let assign14510_e14400: f64 = (assign14510_e14392 / assign14510_e14399);
        (assign14510_e14400, (-((assign14510_e14392 * ((locals.var_fn169_calc_iq__vtheta * locals.var_fn169_calc_iq__qinvv_dn2) / locals.var_fn169_calc_iq__cgin)) / (assign14510_e14399 * assign14510_e14399))), (-((assign14510_e14392 * ((locals.var_fn169_calc_iq__vtheta * locals.var_fn169_calc_iq__qinvv_dn3) / locals.var_fn169_calc_iq__cgin)) / (assign14510_e14399 * assign14510_e14399))), (((((locals.var_fn169_calc_iq__vel0 * (-((assign14510_e14377 * (locals.var_fn169_calc_iq__vzeta * locals.var_fn169_calc_iq__tambin_dn4)) / (assign14510_e14382 * assign14510_e14382)))) * assign14510_e14391) * assign14510_e14399) - (assign14510_e14392 * ((((locals.var_fn169_calc_iq__vtheta * locals.var_fn169_calc_iq__qinvv_dn4) * locals.var_fn169_calc_iq__cgin) - (assign14510_e14396 * locals.var_fn169_calc_iq__cgin_dn4)) / (locals.var_fn169_calc_iq__cgin * locals.var_fn169_calc_iq__cgin)))) / (assign14510_e14399 * assign14510_e14399)), (-((assign14510_e14392 * ((locals.var_fn169_calc_iq__vtheta * locals.var_fn169_calc_iq__qinvv_dn7) / locals.var_fn169_calc_iq__cgin)) / (assign14510_e14399 * assign14510_e14399))), ((((assign14510_e14384 * ((locals.var_fn169_calc_iq__lambda * locals.var_fn169_calc_iq__absvdsin_dn9) / locals.var_fn169_calc_iq__lin)) * assign14510_e14399) - (assign14510_e14392 * ((locals.var_fn169_calc_iq__vtheta * locals.var_fn169_calc_iq__qinvv_dn9) / locals.var_fn169_calc_iq__cgin))) / (assign14510_e14399 * assign14510_e14399)), ((((assign14510_e14384 * ((locals.var_fn169_calc_iq__lambda * locals.var_fn169_calc_iq__absvdsin_dn10) / locals.var_fn169_calc_iq__lin)) * assign14510_e14399) - (assign14510_e14392 * ((locals.var_fn169_calc_iq__vtheta * locals.var_fn169_calc_iq__qinvv_dn10) / locals.var_fn169_calc_iq__cgin))) / (assign14510_e14399 * assign14510_e14399)),)
    } else {
        (locals.var_fn169_calc_iq__vx, locals.var_fn169_calc_iq__vx_dn2, locals.var_fn169_calc_iq__vx_dn3, locals.var_fn169_calc_iq__vx_dn4, locals.var_fn169_calc_iq__vx_dn7, locals.var_fn169_calc_iq__vx_dn9, locals.var_fn169_calc_iq__vx_dn10,)
    }
};
        locals.var_fn169_calc_iq__vx = assign14510_e14402;
        locals.var_fn169_calc_iq__vx_dn2 = assign14510_e14402_d_n2;
        locals.var_fn169_calc_iq__vx_dn3 = assign14510_e14402_d_n3;
        locals.var_fn169_calc_iq__vx_dn4 = assign14510_e14402_d_n4;
        locals.var_fn169_calc_iq__vx_dn7 = assign14510_e14402_d_n7;
        locals.var_fn169_calc_iq__vx_dn9 = assign14510_e14402_d_n9;
        locals.var_fn169_calc_iq__vx_dn10 = assign14510_e14402_d_n10;

        let (assign14530_e14428, assign14530_e14428_d_n2, assign14530_e14428_d_n3, assign14530_e14428_d_n4, assign14530_e14428_d_n7, assign14530_e14428_d_n9, assign14530_e14428_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        let assign14530_e14424: f64 = (locals.var_fn169_calc_iq__vx * locals.var_fn169_calc_iq__lin);
        let assign14530_e14426: f64 = (assign14530_e14424 / locals.var_fn169_calc_iq__muf);
        (assign14530_e14426, ((((locals.var_fn169_calc_iq__vx_dn2 * locals.var_fn169_calc_iq__lin) * locals.var_fn169_calc_iq__muf) - (assign14530_e14424 * locals.var_fn169_calc_iq__muf_dn2)) / (locals.var_fn169_calc_iq__muf * locals.var_fn169_calc_iq__muf)), ((((locals.var_fn169_calc_iq__vx_dn3 * locals.var_fn169_calc_iq__lin) * locals.var_fn169_calc_iq__muf) - (assign14530_e14424 * locals.var_fn169_calc_iq__muf_dn3)) / (locals.var_fn169_calc_iq__muf * locals.var_fn169_calc_iq__muf)), ((((locals.var_fn169_calc_iq__vx_dn4 * locals.var_fn169_calc_iq__lin) * locals.var_fn169_calc_iq__muf) - (assign14530_e14424 * locals.var_fn169_calc_iq__muf_dn4)) / (locals.var_fn169_calc_iq__muf * locals.var_fn169_calc_iq__muf)), ((((locals.var_fn169_calc_iq__vx_dn7 * locals.var_fn169_calc_iq__lin) * locals.var_fn169_calc_iq__muf) - (assign14530_e14424 * locals.var_fn169_calc_iq__muf_dn7)) / (locals.var_fn169_calc_iq__muf * locals.var_fn169_calc_iq__muf)), ((((locals.var_fn169_calc_iq__vx_dn9 * locals.var_fn169_calc_iq__lin) * locals.var_fn169_calc_iq__muf) - (assign14530_e14424 * locals.var_fn169_calc_iq__muf_dn9)) / (locals.var_fn169_calc_iq__muf * locals.var_fn169_calc_iq__muf)), ((((locals.var_fn169_calc_iq__vx_dn10 * locals.var_fn169_calc_iq__lin) * locals.var_fn169_calc_iq__muf) - (assign14530_e14424 * locals.var_fn169_calc_iq__muf_dn10)) / (locals.var_fn169_calc_iq__muf * locals.var_fn169_calc_iq__muf)),)
    } else {
        (locals.var_fn169_calc_iq__vdsats, locals.var_fn169_calc_iq__vdsats_dn2, locals.var_fn169_calc_iq__vdsats_dn3, locals.var_fn169_calc_iq__vdsats_dn4, locals.var_fn169_calc_iq__vdsats_dn7, locals.var_fn169_calc_iq__vdsats_dn9, locals.var_fn169_calc_iq__vdsats_dn10,)
    }
};
        locals.var_fn169_calc_iq__vdsats = assign14530_e14428;
        locals.var_fn169_calc_iq__vdsats_dn2 = assign14530_e14428_d_n2;
        locals.var_fn169_calc_iq__vdsats_dn3 = assign14530_e14428_d_n3;
        locals.var_fn169_calc_iq__vdsats_dn4 = assign14530_e14428_d_n4;
        locals.var_fn169_calc_iq__vdsats_dn7 = assign14530_e14428_d_n7;
        locals.var_fn169_calc_iq__vdsats_dn9 = assign14530_e14428_d_n9;
        locals.var_fn169_calc_iq__vdsats_dn10 = assign14530_e14428_d_n10;

        let (assign14540_e14445, assign14540_e14445_d_n2, assign14540_e14445_d_n3, assign14540_e14445_d_n4, assign14540_e14445_d_n7, assign14540_e14445_d_n9, assign14540_e14445_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        let assign14540_e14434: f64 = (2.0 * locals.var_fn169_calc_iq__qinvv);
        let assign14540_e14436: f64 = (assign14540_e14434 / locals.var_fn169_calc_iq__cgin);
        let assign14540_e14438: f64 = (assign14540_e14436 / locals.var_fn169_calc_iq__vdsats);
        let assign14540_e14439: f64 = (1.0 + assign14540_e14438);
        let assign14540_e14440: f64 = (assign14540_e14439).sqrt();
        let assign14540_e14441: f64 = (locals.var_fn169_calc_iq__vdsats * assign14540_e14440);
        let assign14540_e14443: f64 = (assign14540_e14441 - locals.var_fn169_calc_iq__vdsats);
        (assign14540_e14443, (((locals.var_fn169_calc_iq__vdsats_dn2 * assign14540_e14440) + (locals.var_fn169_calc_iq__vdsats * ((((((2.0 * locals.var_fn169_calc_iq__qinvv_dn2) / locals.var_fn169_calc_iq__cgin) * locals.var_fn169_calc_iq__vdsats) - (assign14540_e14436 * locals.var_fn169_calc_iq__vdsats_dn2)) / (locals.var_fn169_calc_iq__vdsats * locals.var_fn169_calc_iq__vdsats)) / (2.0 * assign14540_e14440)))) - locals.var_fn169_calc_iq__vdsats_dn2), (((locals.var_fn169_calc_iq__vdsats_dn3 * assign14540_e14440) + (locals.var_fn169_calc_iq__vdsats * ((((((2.0 * locals.var_fn169_calc_iq__qinvv_dn3) / locals.var_fn169_calc_iq__cgin) * locals.var_fn169_calc_iq__vdsats) - (assign14540_e14436 * locals.var_fn169_calc_iq__vdsats_dn3)) / (locals.var_fn169_calc_iq__vdsats * locals.var_fn169_calc_iq__vdsats)) / (2.0 * assign14540_e14440)))) - locals.var_fn169_calc_iq__vdsats_dn3), (((locals.var_fn169_calc_iq__vdsats_dn4 * assign14540_e14440) + (locals.var_fn169_calc_iq__vdsats * ((((((((2.0 * locals.var_fn169_calc_iq__qinvv_dn4) * locals.var_fn169_calc_iq__cgin) - (assign14540_e14434 * locals.var_fn169_calc_iq__cgin_dn4)) / (locals.var_fn169_calc_iq__cgin * locals.var_fn169_calc_iq__cgin)) * locals.var_fn169_calc_iq__vdsats) - (assign14540_e14436 * locals.var_fn169_calc_iq__vdsats_dn4)) / (locals.var_fn169_calc_iq__vdsats * locals.var_fn169_calc_iq__vdsats)) / (2.0 * assign14540_e14440)))) - locals.var_fn169_calc_iq__vdsats_dn4), (((locals.var_fn169_calc_iq__vdsats_dn7 * assign14540_e14440) + (locals.var_fn169_calc_iq__vdsats * ((((((2.0 * locals.var_fn169_calc_iq__qinvv_dn7) / locals.var_fn169_calc_iq__cgin) * locals.var_fn169_calc_iq__vdsats) - (assign14540_e14436 * locals.var_fn169_calc_iq__vdsats_dn7)) / (locals.var_fn169_calc_iq__vdsats * locals.var_fn169_calc_iq__vdsats)) / (2.0 * assign14540_e14440)))) - locals.var_fn169_calc_iq__vdsats_dn7), (((locals.var_fn169_calc_iq__vdsats_dn9 * assign14540_e14440) + (locals.var_fn169_calc_iq__vdsats * ((((((2.0 * locals.var_fn169_calc_iq__qinvv_dn9) / locals.var_fn169_calc_iq__cgin) * locals.var_fn169_calc_iq__vdsats) - (assign14540_e14436 * locals.var_fn169_calc_iq__vdsats_dn9)) / (locals.var_fn169_calc_iq__vdsats * locals.var_fn169_calc_iq__vdsats)) / (2.0 * assign14540_e14440)))) - locals.var_fn169_calc_iq__vdsats_dn9), (((locals.var_fn169_calc_iq__vdsats_dn10 * assign14540_e14440) + (locals.var_fn169_calc_iq__vdsats * ((((((2.0 * locals.var_fn169_calc_iq__qinvv_dn10) / locals.var_fn169_calc_iq__cgin) * locals.var_fn169_calc_iq__vdsats) - (assign14540_e14436 * locals.var_fn169_calc_iq__vdsats_dn10)) / (locals.var_fn169_calc_iq__vdsats * locals.var_fn169_calc_iq__vdsats)) / (2.0 * assign14540_e14440)))) - locals.var_fn169_calc_iq__vdsats_dn10),)
    } else {
        (locals.var_fn169_calc_iq__vdsats1, locals.var_fn169_calc_iq__vdsats1_dn2, locals.var_fn169_calc_iq__vdsats1_dn3, locals.var_fn169_calc_iq__vdsats1_dn4, locals.var_fn169_calc_iq__vdsats1_dn7, locals.var_fn169_calc_iq__vdsats1_dn9, locals.var_fn169_calc_iq__vdsats1_dn10,)
    }
};
        locals.var_fn169_calc_iq__vdsats1 = assign14540_e14445;
        locals.var_fn169_calc_iq__vdsats1_dn2 = assign14540_e14445_d_n2;
        locals.var_fn169_calc_iq__vdsats1_dn3 = assign14540_e14445_d_n3;
        locals.var_fn169_calc_iq__vdsats1_dn4 = assign14540_e14445_d_n4;
        locals.var_fn169_calc_iq__vdsats1_dn7 = assign14540_e14445_d_n7;
        locals.var_fn169_calc_iq__vdsats1_dn9 = assign14540_e14445_d_n9;
        locals.var_fn169_calc_iq__vdsats1_dn10 = assign14540_e14445_d_n10;

        let (assign14550_e14457, assign14550_e14457_d_n2, assign14550_e14457_d_n3, assign14550_e14457_d_n4, assign14550_e14457_d_n7, assign14550_e14457_d_n9, assign14550_e14457_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        let assign14550_e14450: f64 = (1.0 - locals.var_fn169_calc_iq__ff);
        let assign14550_e14451: f64 = (locals.var_fn169_calc_iq__vdsats * assign14550_e14450);
        let assign14550_e14454: f64 = (locals.var_fn169_calc_iq__two_n_phit * locals.var_fn169_calc_iq__ff);
        let assign14550_e14455: f64 = (assign14550_e14451 + assign14550_e14454);
        (assign14550_e14455, (((locals.var_fn169_calc_iq__vdsats_dn2 * assign14550_e14450) + (locals.var_fn169_calc_iq__vdsats * (-locals.var_fn169_calc_iq__ff_dn2))) + (locals.var_fn169_calc_iq__two_n_phit * locals.var_fn169_calc_iq__ff_dn2)), (((locals.var_fn169_calc_iq__vdsats_dn3 * assign14550_e14450) + (locals.var_fn169_calc_iq__vdsats * (-locals.var_fn169_calc_iq__ff_dn3))) + (locals.var_fn169_calc_iq__two_n_phit * locals.var_fn169_calc_iq__ff_dn3)), (((locals.var_fn169_calc_iq__vdsats_dn4 * assign14550_e14450) + (locals.var_fn169_calc_iq__vdsats * (-locals.var_fn169_calc_iq__ff_dn4))) + ((locals.var_fn169_calc_iq__two_n_phit_dn4 * locals.var_fn169_calc_iq__ff) + (locals.var_fn169_calc_iq__two_n_phit * locals.var_fn169_calc_iq__ff_dn4))), (((locals.var_fn169_calc_iq__vdsats_dn7 * assign14550_e14450) + (locals.var_fn169_calc_iq__vdsats * (-locals.var_fn169_calc_iq__ff_dn7))) + (locals.var_fn169_calc_iq__two_n_phit * locals.var_fn169_calc_iq__ff_dn7)), (((locals.var_fn169_calc_iq__vdsats_dn9 * assign14550_e14450) + (locals.var_fn169_calc_iq__vdsats * (-locals.var_fn169_calc_iq__ff_dn9))) + ((locals.var_fn169_calc_iq__two_n_phit_dn9 * locals.var_fn169_calc_iq__ff) + (locals.var_fn169_calc_iq__two_n_phit * locals.var_fn169_calc_iq__ff_dn9))), (((locals.var_fn169_calc_iq__vdsats_dn10 * assign14550_e14450) + (locals.var_fn169_calc_iq__vdsats * (-locals.var_fn169_calc_iq__ff_dn10))) + ((locals.var_fn169_calc_iq__two_n_phit_dn10 * locals.var_fn169_calc_iq__ff) + (locals.var_fn169_calc_iq__two_n_phit * locals.var_fn169_calc_iq__ff_dn10))),)
    } else {
        (locals.var_fn169_calc_iq__vdsat, locals.var_fn169_calc_iq__vdsat_dn2, locals.var_fn169_calc_iq__vdsat_dn3, locals.var_fn169_calc_iq__vdsat_dn4, locals.var_fn169_calc_iq__vdsat_dn7, locals.var_fn169_calc_iq__vdsat_dn9, locals.var_fn169_calc_iq__vdsat_dn10,)
    }
};
        locals.var_fn169_calc_iq__vdsat = assign14550_e14457;
        locals.var_fn169_calc_iq__vdsat_dn2 = assign14550_e14457_d_n2;
        locals.var_fn169_calc_iq__vdsat_dn3 = assign14550_e14457_d_n3;
        locals.var_fn169_calc_iq__vdsat_dn4 = assign14550_e14457_d_n4;
        locals.var_fn169_calc_iq__vdsat_dn7 = assign14550_e14457_d_n7;
        locals.var_fn169_calc_iq__vdsat_dn9 = assign14550_e14457_d_n9;
        locals.var_fn169_calc_iq__vdsat_dn10 = assign14550_e14457_d_n10;

        let (assign14560_e14469, assign14560_e14469_d_n2, assign14560_e14469_d_n3, assign14560_e14469_d_n4, assign14560_e14469_d_n7, assign14560_e14469_d_n9, assign14560_e14469_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        let assign14560_e14462: f64 = (1.0 - locals.var_fn169_calc_iq__ff);
        let assign14560_e14463: f64 = (locals.var_fn169_calc_iq__vdsats1 * assign14560_e14462);
        let assign14560_e14466: f64 = (locals.var_fn169_calc_iq__two_n_phit * locals.var_fn169_calc_iq__ff);
        let assign14560_e14467: f64 = (assign14560_e14463 + assign14560_e14466);
        (assign14560_e14467, (((locals.var_fn169_calc_iq__vdsats1_dn2 * assign14560_e14462) + (locals.var_fn169_calc_iq__vdsats1 * (-locals.var_fn169_calc_iq__ff_dn2))) + (locals.var_fn169_calc_iq__two_n_phit * locals.var_fn169_calc_iq__ff_dn2)), (((locals.var_fn169_calc_iq__vdsats1_dn3 * assign14560_e14462) + (locals.var_fn169_calc_iq__vdsats1 * (-locals.var_fn169_calc_iq__ff_dn3))) + (locals.var_fn169_calc_iq__two_n_phit * locals.var_fn169_calc_iq__ff_dn3)), (((locals.var_fn169_calc_iq__vdsats1_dn4 * assign14560_e14462) + (locals.var_fn169_calc_iq__vdsats1 * (-locals.var_fn169_calc_iq__ff_dn4))) + ((locals.var_fn169_calc_iq__two_n_phit_dn4 * locals.var_fn169_calc_iq__ff) + (locals.var_fn169_calc_iq__two_n_phit * locals.var_fn169_calc_iq__ff_dn4))), (((locals.var_fn169_calc_iq__vdsats1_dn7 * assign14560_e14462) + (locals.var_fn169_calc_iq__vdsats1 * (-locals.var_fn169_calc_iq__ff_dn7))) + (locals.var_fn169_calc_iq__two_n_phit * locals.var_fn169_calc_iq__ff_dn7)), (((locals.var_fn169_calc_iq__vdsats1_dn9 * assign14560_e14462) + (locals.var_fn169_calc_iq__vdsats1 * (-locals.var_fn169_calc_iq__ff_dn9))) + ((locals.var_fn169_calc_iq__two_n_phit_dn9 * locals.var_fn169_calc_iq__ff) + (locals.var_fn169_calc_iq__two_n_phit * locals.var_fn169_calc_iq__ff_dn9))), (((locals.var_fn169_calc_iq__vdsats1_dn10 * assign14560_e14462) + (locals.var_fn169_calc_iq__vdsats1 * (-locals.var_fn169_calc_iq__ff_dn10))) + ((locals.var_fn169_calc_iq__two_n_phit_dn10 * locals.var_fn169_calc_iq__ff) + (locals.var_fn169_calc_iq__two_n_phit * locals.var_fn169_calc_iq__ff_dn10))),)
    } else {
        (locals.var_fn169_calc_iq__vdsat1, locals.var_fn169_calc_iq__vdsat1_dn2, locals.var_fn169_calc_iq__vdsat1_dn3, locals.var_fn169_calc_iq__vdsat1_dn4, locals.var_fn169_calc_iq__vdsat1_dn7, locals.var_fn169_calc_iq__vdsat1_dn9, locals.var_fn169_calc_iq__vdsat1_dn10,)
    }
};
        locals.var_fn169_calc_iq__vdsat1 = assign14560_e14469;
        locals.var_fn169_calc_iq__vdsat1_dn2 = assign14560_e14469_d_n2;
        locals.var_fn169_calc_iq__vdsat1_dn3 = assign14560_e14469_d_n3;
        locals.var_fn169_calc_iq__vdsat1_dn4 = assign14560_e14469_d_n4;
        locals.var_fn169_calc_iq__vdsat1_dn7 = assign14560_e14469_d_n7;
        locals.var_fn169_calc_iq__vdsat1_dn9 = assign14560_e14469_d_n9;
        locals.var_fn169_calc_iq__vdsat1_dn10 = assign14560_e14469_d_n10;

        let (assign14570_e14538, assign14570_e14538_d_n2, assign14570_e14538_d_n3, assign14570_e14538_d_n4, assign14570_e14538_d_n7, assign14570_e14538_d_n9, assign14570_e14538_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        let (assign14570_e14528, assign14570_e14528_d_n2, assign14570_e14528_d_n3, assign14570_e14528_d_n4, assign14570_e14528_d_n7, assign14570_e14528_d_n9, assign14570_e14528_d_n10,) = {
            if (p.p52 != 0.0) {
                let assign14570_e14481: f64 = (locals.var_fn169_calc_iq__vdsin / locals.var_fn169_calc_iq__vdsat1);
                let assign14570_e14482: f64 = assign14570_e14481;
                let assign14570_e14486: f64 = (locals.var_fn169_calc_iq__vdsin / locals.var_fn169_calc_iq__vdsat1);
                let assign14570_e14487: f64 = (-assign14570_e14486);
                let assign14570_e14490: f64 = (0.001 / p.p53);
                let assign14570_e14494: f64 = (locals.var_fn169_calc_iq__vdsin / locals.var_fn169_calc_iq__vdsat1);
                let assign14570_e14495: f64 = (-assign14570_e14494);
                let assign14570_e14496: f64 = (assign14570_e14490 * assign14570_e14495);
                let assign14570_e14497: f64 = (assign14570_e14496).tanh();
                let assign14570_e14498: f64 = (assign14570_e14487 * assign14570_e14497);
                let assign14570_e14499: f64 = (assign14570_e14482 + assign14570_e14498);
                let assign14570_e14500: f64 = (0.5 * assign14570_e14499);
                (assign14570_e14500, (0.5 * ((-((locals.var_fn169_calc_iq__vdsin * locals.var_fn169_calc_iq__vdsat1_dn2) / (locals.var_fn169_calc_iq__vdsat1 * locals.var_fn169_calc_iq__vdsat1))) + (((-(-((locals.var_fn169_calc_iq__vdsin * locals.var_fn169_calc_iq__vdsat1_dn2) / (locals.var_fn169_calc_iq__vdsat1 * locals.var_fn169_calc_iq__vdsat1)))) * assign14570_e14497) + (assign14570_e14487 * ((assign14570_e14490 * (-(-((locals.var_fn169_calc_iq__vdsin * locals.var_fn169_calc_iq__vdsat1_dn2) / (locals.var_fn169_calc_iq__vdsat1 * locals.var_fn169_calc_iq__vdsat1))))) / ((assign14570_e14496).cosh() * (assign14570_e14496).cosh())))))), (0.5 * ((-((locals.var_fn169_calc_iq__vdsin * locals.var_fn169_calc_iq__vdsat1_dn3) / (locals.var_fn169_calc_iq__vdsat1 * locals.var_fn169_calc_iq__vdsat1))) + (((-(-((locals.var_fn169_calc_iq__vdsin * locals.var_fn169_calc_iq__vdsat1_dn3) / (locals.var_fn169_calc_iq__vdsat1 * locals.var_fn169_calc_iq__vdsat1)))) * assign14570_e14497) + (assign14570_e14487 * ((assign14570_e14490 * (-(-((locals.var_fn169_calc_iq__vdsin * locals.var_fn169_calc_iq__vdsat1_dn3) / (locals.var_fn169_calc_iq__vdsat1 * locals.var_fn169_calc_iq__vdsat1))))) / ((assign14570_e14496).cosh() * (assign14570_e14496).cosh())))))), (0.5 * ((-((locals.var_fn169_calc_iq__vdsin * locals.var_fn169_calc_iq__vdsat1_dn4) / (locals.var_fn169_calc_iq__vdsat1 * locals.var_fn169_calc_iq__vdsat1))) + (((-(-((locals.var_fn169_calc_iq__vdsin * locals.var_fn169_calc_iq__vdsat1_dn4) / (locals.var_fn169_calc_iq__vdsat1 * locals.var_fn169_calc_iq__vdsat1)))) * assign14570_e14497) + (assign14570_e14487 * ((assign14570_e14490 * (-(-((locals.var_fn169_calc_iq__vdsin * locals.var_fn169_calc_iq__vdsat1_dn4) / (locals.var_fn169_calc_iq__vdsat1 * locals.var_fn169_calc_iq__vdsat1))))) / ((assign14570_e14496).cosh() * (assign14570_e14496).cosh())))))), (0.5 * ((-((locals.var_fn169_calc_iq__vdsin * locals.var_fn169_calc_iq__vdsat1_dn7) / (locals.var_fn169_calc_iq__vdsat1 * locals.var_fn169_calc_iq__vdsat1))) + (((-(-((locals.var_fn169_calc_iq__vdsin * locals.var_fn169_calc_iq__vdsat1_dn7) / (locals.var_fn169_calc_iq__vdsat1 * locals.var_fn169_calc_iq__vdsat1)))) * assign14570_e14497) + (assign14570_e14487 * ((assign14570_e14490 * (-(-((locals.var_fn169_calc_iq__vdsin * locals.var_fn169_calc_iq__vdsat1_dn7) / (locals.var_fn169_calc_iq__vdsat1 * locals.var_fn169_calc_iq__vdsat1))))) / ((assign14570_e14496).cosh() * (assign14570_e14496).cosh())))))), (0.5 * ((((locals.var_fn169_calc_iq__vdsin_dn9 * locals.var_fn169_calc_iq__vdsat1) - (locals.var_fn169_calc_iq__vdsin * locals.var_fn169_calc_iq__vdsat1_dn9)) / (locals.var_fn169_calc_iq__vdsat1 * locals.var_fn169_calc_iq__vdsat1)) + (((-(((locals.var_fn169_calc_iq__vdsin_dn9 * locals.var_fn169_calc_iq__vdsat1) - (locals.var_fn169_calc_iq__vdsin * locals.var_fn169_calc_iq__vdsat1_dn9)) / (locals.var_fn169_calc_iq__vdsat1 * locals.var_fn169_calc_iq__vdsat1))) * assign14570_e14497) + (assign14570_e14487 * ((assign14570_e14490 * (-(((locals.var_fn169_calc_iq__vdsin_dn9 * locals.var_fn169_calc_iq__vdsat1) - (locals.var_fn169_calc_iq__vdsin * locals.var_fn169_calc_iq__vdsat1_dn9)) / (locals.var_fn169_calc_iq__vdsat1 * locals.var_fn169_calc_iq__vdsat1)))) / ((assign14570_e14496).cosh() * (assign14570_e14496).cosh())))))), (0.5 * ((((locals.var_fn169_calc_iq__vdsin_dn10 * locals.var_fn169_calc_iq__vdsat1) - (locals.var_fn169_calc_iq__vdsin * locals.var_fn169_calc_iq__vdsat1_dn10)) / (locals.var_fn169_calc_iq__vdsat1 * locals.var_fn169_calc_iq__vdsat1)) + (((-(((locals.var_fn169_calc_iq__vdsin_dn10 * locals.var_fn169_calc_iq__vdsat1) - (locals.var_fn169_calc_iq__vdsin * locals.var_fn169_calc_iq__vdsat1_dn10)) / (locals.var_fn169_calc_iq__vdsat1 * locals.var_fn169_calc_iq__vdsat1))) * assign14570_e14497) + (assign14570_e14487 * ((assign14570_e14490 * (-(((locals.var_fn169_calc_iq__vdsin_dn10 * locals.var_fn169_calc_iq__vdsat1) - (locals.var_fn169_calc_iq__vdsin * locals.var_fn169_calc_iq__vdsat1_dn10)) / (locals.var_fn169_calc_iq__vdsat1 * locals.var_fn169_calc_iq__vdsat1)))) / ((assign14570_e14496).cosh() * (assign14570_e14496).cosh())))))),)
            } else {
                let (assign14570_e14527, assign14570_e14527_d_n2, assign14570_e14527_d_n3, assign14570_e14527_d_n4, assign14570_e14527_d_n7, assign14570_e14527_d_n9, assign14570_e14527_d_n10,) = {
                    if (p.p52 == 0.0) {
                        let assign14570_e14508: f64 = (locals.var_fn169_calc_iq__vdsin / locals.var_fn169_calc_iq__vdsat1);
                        let assign14570_e14509: f64 = assign14570_e14508;
                        let assign14570_e14513: f64 = (locals.var_fn169_calc_iq__vdsin / locals.var_fn169_calc_iq__vdsat1);
                        let assign14570_e14514: f64 = (-assign14570_e14513);
                        let assign14570_e14518: f64 = (locals.var_fn169_calc_iq__vdsin / locals.var_fn169_calc_iq__vdsat1);
                        let assign14570_e14519: f64 = (-assign14570_e14518);
                        let assign14570_e14520: f64 = (assign14570_e14514 * assign14570_e14519);
                        let assign14570_e14522: f64 = (assign14570_e14520 + p.p53);
                        let assign14570_e14523: f64 = (assign14570_e14522).sqrt();
                        let assign14570_e14524: f64 = (assign14570_e14509 + assign14570_e14523);
                        let assign14570_e14525: f64 = (0.5 * assign14570_e14524);
                        (assign14570_e14525, (0.5 * ((-((locals.var_fn169_calc_iq__vdsin * locals.var_fn169_calc_iq__vdsat1_dn2) / (locals.var_fn169_calc_iq__vdsat1 * locals.var_fn169_calc_iq__vdsat1))) + ((((-(-((locals.var_fn169_calc_iq__vdsin * locals.var_fn169_calc_iq__vdsat1_dn2) / (locals.var_fn169_calc_iq__vdsat1 * locals.var_fn169_calc_iq__vdsat1)))) * assign14570_e14519) + (assign14570_e14514 * (-(-((locals.var_fn169_calc_iq__vdsin * locals.var_fn169_calc_iq__vdsat1_dn2) / (locals.var_fn169_calc_iq__vdsat1 * locals.var_fn169_calc_iq__vdsat1)))))) / (2.0 * assign14570_e14523)))), (0.5 * ((-((locals.var_fn169_calc_iq__vdsin * locals.var_fn169_calc_iq__vdsat1_dn3) / (locals.var_fn169_calc_iq__vdsat1 * locals.var_fn169_calc_iq__vdsat1))) + ((((-(-((locals.var_fn169_calc_iq__vdsin * locals.var_fn169_calc_iq__vdsat1_dn3) / (locals.var_fn169_calc_iq__vdsat1 * locals.var_fn169_calc_iq__vdsat1)))) * assign14570_e14519) + (assign14570_e14514 * (-(-((locals.var_fn169_calc_iq__vdsin * locals.var_fn169_calc_iq__vdsat1_dn3) / (locals.var_fn169_calc_iq__vdsat1 * locals.var_fn169_calc_iq__vdsat1)))))) / (2.0 * assign14570_e14523)))), (0.5 * ((-((locals.var_fn169_calc_iq__vdsin * locals.var_fn169_calc_iq__vdsat1_dn4) / (locals.var_fn169_calc_iq__vdsat1 * locals.var_fn169_calc_iq__vdsat1))) + ((((-(-((locals.var_fn169_calc_iq__vdsin * locals.var_fn169_calc_iq__vdsat1_dn4) / (locals.var_fn169_calc_iq__vdsat1 * locals.var_fn169_calc_iq__vdsat1)))) * assign14570_e14519) + (assign14570_e14514 * (-(-((locals.var_fn169_calc_iq__vdsin * locals.var_fn169_calc_iq__vdsat1_dn4) / (locals.var_fn169_calc_iq__vdsat1 * locals.var_fn169_calc_iq__vdsat1)))))) / (2.0 * assign14570_e14523)))), (0.5 * ((-((locals.var_fn169_calc_iq__vdsin * locals.var_fn169_calc_iq__vdsat1_dn7) / (locals.var_fn169_calc_iq__vdsat1 * locals.var_fn169_calc_iq__vdsat1))) + ((((-(-((locals.var_fn169_calc_iq__vdsin * locals.var_fn169_calc_iq__vdsat1_dn7) / (locals.var_fn169_calc_iq__vdsat1 * locals.var_fn169_calc_iq__vdsat1)))) * assign14570_e14519) + (assign14570_e14514 * (-(-((locals.var_fn169_calc_iq__vdsin * locals.var_fn169_calc_iq__vdsat1_dn7) / (locals.var_fn169_calc_iq__vdsat1 * locals.var_fn169_calc_iq__vdsat1)))))) / (2.0 * assign14570_e14523)))), (0.5 * ((((locals.var_fn169_calc_iq__vdsin_dn9 * locals.var_fn169_calc_iq__vdsat1) - (locals.var_fn169_calc_iq__vdsin * locals.var_fn169_calc_iq__vdsat1_dn9)) / (locals.var_fn169_calc_iq__vdsat1 * locals.var_fn169_calc_iq__vdsat1)) + ((((-(((locals.var_fn169_calc_iq__vdsin_dn9 * locals.var_fn169_calc_iq__vdsat1) - (locals.var_fn169_calc_iq__vdsin * locals.var_fn169_calc_iq__vdsat1_dn9)) / (locals.var_fn169_calc_iq__vdsat1 * locals.var_fn169_calc_iq__vdsat1))) * assign14570_e14519) + (assign14570_e14514 * (-(((locals.var_fn169_calc_iq__vdsin_dn9 * locals.var_fn169_calc_iq__vdsat1) - (locals.var_fn169_calc_iq__vdsin * locals.var_fn169_calc_iq__vdsat1_dn9)) / (locals.var_fn169_calc_iq__vdsat1 * locals.var_fn169_calc_iq__vdsat1))))) / (2.0 * assign14570_e14523)))), (0.5 * ((((locals.var_fn169_calc_iq__vdsin_dn10 * locals.var_fn169_calc_iq__vdsat1) - (locals.var_fn169_calc_iq__vdsin * locals.var_fn169_calc_iq__vdsat1_dn10)) / (locals.var_fn169_calc_iq__vdsat1 * locals.var_fn169_calc_iq__vdsat1)) + ((((-(((locals.var_fn169_calc_iq__vdsin_dn10 * locals.var_fn169_calc_iq__vdsat1) - (locals.var_fn169_calc_iq__vdsin * locals.var_fn169_calc_iq__vdsat1_dn10)) / (locals.var_fn169_calc_iq__vdsat1 * locals.var_fn169_calc_iq__vdsat1))) * assign14570_e14519) + (assign14570_e14514 * (-(((locals.var_fn169_calc_iq__vdsin_dn10 * locals.var_fn169_calc_iq__vdsat1) - (locals.var_fn169_calc_iq__vdsin * locals.var_fn169_calc_iq__vdsat1_dn10)) / (locals.var_fn169_calc_iq__vdsat1 * locals.var_fn169_calc_iq__vdsat1))))) / (2.0 * assign14570_e14523)))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign14570_e14527, assign14570_e14527_d_n2, assign14570_e14527_d_n3, assign14570_e14527_d_n4, assign14570_e14527_d_n7, assign14570_e14527_d_n9, assign14570_e14527_d_n10,)
            }
        };
        let assign14570_e14530: f64 = (assign14570_e14528).powf(locals.var_fn169_calc_iq__beta);
        let assign14570_e14531: f64 = (1.0 + assign14570_e14530);
        let assign14570_e14534: f64 = (1.0 / locals.var_fn169_calc_iq__beta);
        let assign14570_e14535: f64 = (assign14570_e14531).powf(assign14570_e14534);
        let assign14570_e14536: f64 = (1.0 / assign14570_e14535);
        (assign14570_e14536, (-(if 0.0 == 0.0 && ((assign14570_e14534) as f64).is_finite() && ((assign14570_e14534) as f64).fract() == 0.0 { if assign14570_e14534 == 0.0 { 0.0 } else { (assign14570_e14534 * ((assign14570_e14531).powf(assign14570_e14534 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn169_calc_iq__beta) as f64).is_finite() && ((locals.var_fn169_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn169_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn169_calc_iq__beta * ((assign14570_e14528).powf(locals.var_fn169_calc_iq__beta - 1.0) * assign14570_e14528_d_n2)) } } else { (assign14570_e14530 * (locals.var_fn169_calc_iq__beta * (assign14570_e14528_d_n2 / assign14570_e14528))) })) } } else { (assign14570_e14535 * (assign14570_e14534 * (if 0.0 == 0.0 && ((locals.var_fn169_calc_iq__beta) as f64).is_finite() && ((locals.var_fn169_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn169_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn169_calc_iq__beta * ((assign14570_e14528).powf(locals.var_fn169_calc_iq__beta - 1.0) * assign14570_e14528_d_n2)) } } else { (assign14570_e14530 * (locals.var_fn169_calc_iq__beta * (assign14570_e14528_d_n2 / assign14570_e14528))) } / assign14570_e14531))) } / (assign14570_e14535 * assign14570_e14535))), (-(if 0.0 == 0.0 && ((assign14570_e14534) as f64).is_finite() && ((assign14570_e14534) as f64).fract() == 0.0 { if assign14570_e14534 == 0.0 { 0.0 } else { (assign14570_e14534 * ((assign14570_e14531).powf(assign14570_e14534 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn169_calc_iq__beta) as f64).is_finite() && ((locals.var_fn169_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn169_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn169_calc_iq__beta * ((assign14570_e14528).powf(locals.var_fn169_calc_iq__beta - 1.0) * assign14570_e14528_d_n3)) } } else { (assign14570_e14530 * (locals.var_fn169_calc_iq__beta * (assign14570_e14528_d_n3 / assign14570_e14528))) })) } } else { (assign14570_e14535 * (assign14570_e14534 * (if 0.0 == 0.0 && ((locals.var_fn169_calc_iq__beta) as f64).is_finite() && ((locals.var_fn169_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn169_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn169_calc_iq__beta * ((assign14570_e14528).powf(locals.var_fn169_calc_iq__beta - 1.0) * assign14570_e14528_d_n3)) } } else { (assign14570_e14530 * (locals.var_fn169_calc_iq__beta * (assign14570_e14528_d_n3 / assign14570_e14528))) } / assign14570_e14531))) } / (assign14570_e14535 * assign14570_e14535))), (-(if 0.0 == 0.0 && ((assign14570_e14534) as f64).is_finite() && ((assign14570_e14534) as f64).fract() == 0.0 { if assign14570_e14534 == 0.0 { 0.0 } else { (assign14570_e14534 * ((assign14570_e14531).powf(assign14570_e14534 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn169_calc_iq__beta) as f64).is_finite() && ((locals.var_fn169_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn169_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn169_calc_iq__beta * ((assign14570_e14528).powf(locals.var_fn169_calc_iq__beta - 1.0) * assign14570_e14528_d_n4)) } } else { (assign14570_e14530 * (locals.var_fn169_calc_iq__beta * (assign14570_e14528_d_n4 / assign14570_e14528))) })) } } else { (assign14570_e14535 * (assign14570_e14534 * (if 0.0 == 0.0 && ((locals.var_fn169_calc_iq__beta) as f64).is_finite() && ((locals.var_fn169_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn169_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn169_calc_iq__beta * ((assign14570_e14528).powf(locals.var_fn169_calc_iq__beta - 1.0) * assign14570_e14528_d_n4)) } } else { (assign14570_e14530 * (locals.var_fn169_calc_iq__beta * (assign14570_e14528_d_n4 / assign14570_e14528))) } / assign14570_e14531))) } / (assign14570_e14535 * assign14570_e14535))), (-(if 0.0 == 0.0 && ((assign14570_e14534) as f64).is_finite() && ((assign14570_e14534) as f64).fract() == 0.0 { if assign14570_e14534 == 0.0 { 0.0 } else { (assign14570_e14534 * ((assign14570_e14531).powf(assign14570_e14534 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn169_calc_iq__beta) as f64).is_finite() && ((locals.var_fn169_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn169_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn169_calc_iq__beta * ((assign14570_e14528).powf(locals.var_fn169_calc_iq__beta - 1.0) * assign14570_e14528_d_n7)) } } else { (assign14570_e14530 * (locals.var_fn169_calc_iq__beta * (assign14570_e14528_d_n7 / assign14570_e14528))) })) } } else { (assign14570_e14535 * (assign14570_e14534 * (if 0.0 == 0.0 && ((locals.var_fn169_calc_iq__beta) as f64).is_finite() && ((locals.var_fn169_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn169_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn169_calc_iq__beta * ((assign14570_e14528).powf(locals.var_fn169_calc_iq__beta - 1.0) * assign14570_e14528_d_n7)) } } else { (assign14570_e14530 * (locals.var_fn169_calc_iq__beta * (assign14570_e14528_d_n7 / assign14570_e14528))) } / assign14570_e14531))) } / (assign14570_e14535 * assign14570_e14535))), (-(if 0.0 == 0.0 && ((assign14570_e14534) as f64).is_finite() && ((assign14570_e14534) as f64).fract() == 0.0 { if assign14570_e14534 == 0.0 { 0.0 } else { (assign14570_e14534 * ((assign14570_e14531).powf(assign14570_e14534 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn169_calc_iq__beta) as f64).is_finite() && ((locals.var_fn169_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn169_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn169_calc_iq__beta * ((assign14570_e14528).powf(locals.var_fn169_calc_iq__beta - 1.0) * assign14570_e14528_d_n9)) } } else { (assign14570_e14530 * (locals.var_fn169_calc_iq__beta * (assign14570_e14528_d_n9 / assign14570_e14528))) })) } } else { (assign14570_e14535 * (assign14570_e14534 * (if 0.0 == 0.0 && ((locals.var_fn169_calc_iq__beta) as f64).is_finite() && ((locals.var_fn169_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn169_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn169_calc_iq__beta * ((assign14570_e14528).powf(locals.var_fn169_calc_iq__beta - 1.0) * assign14570_e14528_d_n9)) } } else { (assign14570_e14530 * (locals.var_fn169_calc_iq__beta * (assign14570_e14528_d_n9 / assign14570_e14528))) } / assign14570_e14531))) } / (assign14570_e14535 * assign14570_e14535))), (-(if 0.0 == 0.0 && ((assign14570_e14534) as f64).is_finite() && ((assign14570_e14534) as f64).fract() == 0.0 { if assign14570_e14534 == 0.0 { 0.0 } else { (assign14570_e14534 * ((assign14570_e14531).powf(assign14570_e14534 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn169_calc_iq__beta) as f64).is_finite() && ((locals.var_fn169_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn169_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn169_calc_iq__beta * ((assign14570_e14528).powf(locals.var_fn169_calc_iq__beta - 1.0) * assign14570_e14528_d_n10)) } } else { (assign14570_e14530 * (locals.var_fn169_calc_iq__beta * (assign14570_e14528_d_n10 / assign14570_e14528))) })) } } else { (assign14570_e14535 * (assign14570_e14534 * (if 0.0 == 0.0 && ((locals.var_fn169_calc_iq__beta) as f64).is_finite() && ((locals.var_fn169_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn169_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn169_calc_iq__beta * ((assign14570_e14528).powf(locals.var_fn169_calc_iq__beta - 1.0) * assign14570_e14528_d_n10)) } } else { (assign14570_e14530 * (locals.var_fn169_calc_iq__beta * (assign14570_e14528_d_n10 / assign14570_e14528))) } / assign14570_e14531))) } / (assign14570_e14535 * assign14570_e14535))),)
    } else {
        (locals.var_fn169_calc_iq__fsd, locals.var_fn169_calc_iq__fsd_dn2, locals.var_fn169_calc_iq__fsd_dn3, locals.var_fn169_calc_iq__fsd_dn4, locals.var_fn169_calc_iq__fsd_dn7, locals.var_fn169_calc_iq__fsd_dn9, locals.var_fn169_calc_iq__fsd_dn10,)
    }
};
        locals.var_fn169_calc_iq__fsd = assign14570_e14538;
        locals.var_fn169_calc_iq__fsd_dn2 = assign14570_e14538_d_n2;
        locals.var_fn169_calc_iq__fsd_dn3 = assign14570_e14538_d_n3;
        locals.var_fn169_calc_iq__fsd_dn4 = assign14570_e14538_d_n4;
        locals.var_fn169_calc_iq__fsd_dn7 = assign14570_e14538_d_n7;
        locals.var_fn169_calc_iq__fsd_dn9 = assign14570_e14538_d_n9;
        locals.var_fn169_calc_iq__fsd_dn10 = assign14570_e14538_d_n10;

    }

    pub(super) fn stamp_transient_block_36(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign14580_e14544, assign14580_e14544_d_n2, assign14580_e14544_d_n3, assign14580_e14544_d_n4, assign14580_e14544_d_n7, assign14580_e14544_d_n9, assign14580_e14544_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        let assign14580_e14542: f64 = (locals.var_fn169_calc_iq__vdsin * locals.var_fn169_calc_iq__fsd);
        (assign14580_e14542, (locals.var_fn169_calc_iq__vdsin * locals.var_fn169_calc_iq__fsd_dn2), (locals.var_fn169_calc_iq__vdsin * locals.var_fn169_calc_iq__fsd_dn3), (locals.var_fn169_calc_iq__vdsin * locals.var_fn169_calc_iq__fsd_dn4), (locals.var_fn169_calc_iq__vdsin * locals.var_fn169_calc_iq__fsd_dn7), ((locals.var_fn169_calc_iq__vdsin_dn9 * locals.var_fn169_calc_iq__fsd) + (locals.var_fn169_calc_iq__vdsin * locals.var_fn169_calc_iq__fsd_dn9)), ((locals.var_fn169_calc_iq__vdsin_dn10 * locals.var_fn169_calc_iq__fsd) + (locals.var_fn169_calc_iq__vdsin * locals.var_fn169_calc_iq__fsd_dn10)),)
    } else {
        (locals.var_fn169_calc_iq__vdx, locals.var_fn169_calc_iq__vdx_dn2, locals.var_fn169_calc_iq__vdx_dn3, locals.var_fn169_calc_iq__vdx_dn4, locals.var_fn169_calc_iq__vdx_dn7, locals.var_fn169_calc_iq__vdx_dn9, locals.var_fn169_calc_iq__vdx_dn10,)
    }
};
        locals.var_fn169_calc_iq__vdx = assign14580_e14544;
        locals.var_fn169_calc_iq__vdx_dn2 = assign14580_e14544_d_n2;
        locals.var_fn169_calc_iq__vdx_dn3 = assign14580_e14544_d_n3;
        locals.var_fn169_calc_iq__vdx_dn4 = assign14580_e14544_d_n4;
        locals.var_fn169_calc_iq__vdx_dn7 = assign14580_e14544_d_n7;
        locals.var_fn169_calc_iq__vdx_dn9 = assign14580_e14544_d_n9;
        locals.var_fn169_calc_iq__vdx_dn10 = assign14580_e14544_d_n10;

        let (assign14590_e14619, assign14590_e14619_d_n2, assign14590_e14619_d_n3, assign14590_e14619_d_n4, assign14590_e14619_d_n7, assign14590_e14619_d_n9, assign14590_e14619_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        let (assign14590_e14609, assign14590_e14609_d_n2, assign14590_e14609_d_n3, assign14590_e14609_d_n4, assign14590_e14609_d_n7, assign14590_e14609_d_n9, assign14590_e14609_d_n10,) = {
            if (p.p52 != 0.0) {
                let assign14590_e14555: f64 = (-locals.var_fn169_calc_iq__vdsin);
                let assign14590_e14557: f64 = (assign14590_e14555 / locals.var_fn169_calc_iq__vdsat1);
                let assign14590_e14558: f64 = assign14590_e14557;
                let assign14590_e14561: f64 = (-locals.var_fn169_calc_iq__vdsin);
                let assign14590_e14563: f64 = (assign14590_e14561 / locals.var_fn169_calc_iq__vdsat1);
                let assign14590_e14564: f64 = (-assign14590_e14563);
                let assign14590_e14567: f64 = (0.001 / p.p53);
                let assign14590_e14570: f64 = (-locals.var_fn169_calc_iq__vdsin);
                let assign14590_e14572: f64 = (assign14590_e14570 / locals.var_fn169_calc_iq__vdsat1);
                let assign14590_e14573: f64 = (-assign14590_e14572);
                let assign14590_e14574: f64 = (assign14590_e14567 * assign14590_e14573);
                let assign14590_e14575: f64 = (assign14590_e14574).tanh();
                let assign14590_e14576: f64 = (assign14590_e14564 * assign14590_e14575);
                let assign14590_e14577: f64 = (assign14590_e14558 + assign14590_e14576);
                let assign14590_e14578: f64 = (0.5 * assign14590_e14577);
                (assign14590_e14578, (0.5 * ((-((assign14590_e14555 * locals.var_fn169_calc_iq__vdsat1_dn2) / (locals.var_fn169_calc_iq__vdsat1 * locals.var_fn169_calc_iq__vdsat1))) + (((-(-((assign14590_e14561 * locals.var_fn169_calc_iq__vdsat1_dn2) / (locals.var_fn169_calc_iq__vdsat1 * locals.var_fn169_calc_iq__vdsat1)))) * assign14590_e14575) + (assign14590_e14564 * ((assign14590_e14567 * (-(-((assign14590_e14570 * locals.var_fn169_calc_iq__vdsat1_dn2) / (locals.var_fn169_calc_iq__vdsat1 * locals.var_fn169_calc_iq__vdsat1))))) / ((assign14590_e14574).cosh() * (assign14590_e14574).cosh())))))), (0.5 * ((-((assign14590_e14555 * locals.var_fn169_calc_iq__vdsat1_dn3) / (locals.var_fn169_calc_iq__vdsat1 * locals.var_fn169_calc_iq__vdsat1))) + (((-(-((assign14590_e14561 * locals.var_fn169_calc_iq__vdsat1_dn3) / (locals.var_fn169_calc_iq__vdsat1 * locals.var_fn169_calc_iq__vdsat1)))) * assign14590_e14575) + (assign14590_e14564 * ((assign14590_e14567 * (-(-((assign14590_e14570 * locals.var_fn169_calc_iq__vdsat1_dn3) / (locals.var_fn169_calc_iq__vdsat1 * locals.var_fn169_calc_iq__vdsat1))))) / ((assign14590_e14574).cosh() * (assign14590_e14574).cosh())))))), (0.5 * ((-((assign14590_e14555 * locals.var_fn169_calc_iq__vdsat1_dn4) / (locals.var_fn169_calc_iq__vdsat1 * locals.var_fn169_calc_iq__vdsat1))) + (((-(-((assign14590_e14561 * locals.var_fn169_calc_iq__vdsat1_dn4) / (locals.var_fn169_calc_iq__vdsat1 * locals.var_fn169_calc_iq__vdsat1)))) * assign14590_e14575) + (assign14590_e14564 * ((assign14590_e14567 * (-(-((assign14590_e14570 * locals.var_fn169_calc_iq__vdsat1_dn4) / (locals.var_fn169_calc_iq__vdsat1 * locals.var_fn169_calc_iq__vdsat1))))) / ((assign14590_e14574).cosh() * (assign14590_e14574).cosh())))))), (0.5 * ((-((assign14590_e14555 * locals.var_fn169_calc_iq__vdsat1_dn7) / (locals.var_fn169_calc_iq__vdsat1 * locals.var_fn169_calc_iq__vdsat1))) + (((-(-((assign14590_e14561 * locals.var_fn169_calc_iq__vdsat1_dn7) / (locals.var_fn169_calc_iq__vdsat1 * locals.var_fn169_calc_iq__vdsat1)))) * assign14590_e14575) + (assign14590_e14564 * ((assign14590_e14567 * (-(-((assign14590_e14570 * locals.var_fn169_calc_iq__vdsat1_dn7) / (locals.var_fn169_calc_iq__vdsat1 * locals.var_fn169_calc_iq__vdsat1))))) / ((assign14590_e14574).cosh() * (assign14590_e14574).cosh())))))), (0.5 * (((((-locals.var_fn169_calc_iq__vdsin_dn9) * locals.var_fn169_calc_iq__vdsat1) - (assign14590_e14555 * locals.var_fn169_calc_iq__vdsat1_dn9)) / (locals.var_fn169_calc_iq__vdsat1 * locals.var_fn169_calc_iq__vdsat1)) + (((-((((-locals.var_fn169_calc_iq__vdsin_dn9) * locals.var_fn169_calc_iq__vdsat1) - (assign14590_e14561 * locals.var_fn169_calc_iq__vdsat1_dn9)) / (locals.var_fn169_calc_iq__vdsat1 * locals.var_fn169_calc_iq__vdsat1))) * assign14590_e14575) + (assign14590_e14564 * ((assign14590_e14567 * (-((((-locals.var_fn169_calc_iq__vdsin_dn9) * locals.var_fn169_calc_iq__vdsat1) - (assign14590_e14570 * locals.var_fn169_calc_iq__vdsat1_dn9)) / (locals.var_fn169_calc_iq__vdsat1 * locals.var_fn169_calc_iq__vdsat1)))) / ((assign14590_e14574).cosh() * (assign14590_e14574).cosh())))))), (0.5 * (((((-locals.var_fn169_calc_iq__vdsin_dn10) * locals.var_fn169_calc_iq__vdsat1) - (assign14590_e14555 * locals.var_fn169_calc_iq__vdsat1_dn10)) / (locals.var_fn169_calc_iq__vdsat1 * locals.var_fn169_calc_iq__vdsat1)) + (((-((((-locals.var_fn169_calc_iq__vdsin_dn10) * locals.var_fn169_calc_iq__vdsat1) - (assign14590_e14561 * locals.var_fn169_calc_iq__vdsat1_dn10)) / (locals.var_fn169_calc_iq__vdsat1 * locals.var_fn169_calc_iq__vdsat1))) * assign14590_e14575) + (assign14590_e14564 * ((assign14590_e14567 * (-((((-locals.var_fn169_calc_iq__vdsin_dn10) * locals.var_fn169_calc_iq__vdsat1) - (assign14590_e14570 * locals.var_fn169_calc_iq__vdsat1_dn10)) / (locals.var_fn169_calc_iq__vdsat1 * locals.var_fn169_calc_iq__vdsat1)))) / ((assign14590_e14574).cosh() * (assign14590_e14574).cosh())))))),)
            } else {
                let (assign14590_e14608, assign14590_e14608_d_n2, assign14590_e14608_d_n3, assign14590_e14608_d_n4, assign14590_e14608_d_n7, assign14590_e14608_d_n9, assign14590_e14608_d_n10,) = {
                    if (p.p52 == 0.0) {
                        let assign14590_e14585: f64 = (-locals.var_fn169_calc_iq__vdsin);
                        let assign14590_e14587: f64 = (assign14590_e14585 / locals.var_fn169_calc_iq__vdsat1);
                        let assign14590_e14588: f64 = assign14590_e14587;
                        let assign14590_e14591: f64 = (-locals.var_fn169_calc_iq__vdsin);
                        let assign14590_e14593: f64 = (assign14590_e14591 / locals.var_fn169_calc_iq__vdsat1);
                        let assign14590_e14594: f64 = (-assign14590_e14593);
                        let assign14590_e14597: f64 = (-locals.var_fn169_calc_iq__vdsin);
                        let assign14590_e14599: f64 = (assign14590_e14597 / locals.var_fn169_calc_iq__vdsat1);
                        let assign14590_e14600: f64 = (-assign14590_e14599);
                        let assign14590_e14601: f64 = (assign14590_e14594 * assign14590_e14600);
                        let assign14590_e14603: f64 = (assign14590_e14601 + p.p53);
                        let assign14590_e14604: f64 = (assign14590_e14603).sqrt();
                        let assign14590_e14605: f64 = (assign14590_e14588 + assign14590_e14604);
                        let assign14590_e14606: f64 = (0.5 * assign14590_e14605);
                        (assign14590_e14606, (0.5 * ((-((assign14590_e14585 * locals.var_fn169_calc_iq__vdsat1_dn2) / (locals.var_fn169_calc_iq__vdsat1 * locals.var_fn169_calc_iq__vdsat1))) + ((((-(-((assign14590_e14591 * locals.var_fn169_calc_iq__vdsat1_dn2) / (locals.var_fn169_calc_iq__vdsat1 * locals.var_fn169_calc_iq__vdsat1)))) * assign14590_e14600) + (assign14590_e14594 * (-(-((assign14590_e14597 * locals.var_fn169_calc_iq__vdsat1_dn2) / (locals.var_fn169_calc_iq__vdsat1 * locals.var_fn169_calc_iq__vdsat1)))))) / (2.0 * assign14590_e14604)))), (0.5 * ((-((assign14590_e14585 * locals.var_fn169_calc_iq__vdsat1_dn3) / (locals.var_fn169_calc_iq__vdsat1 * locals.var_fn169_calc_iq__vdsat1))) + ((((-(-((assign14590_e14591 * locals.var_fn169_calc_iq__vdsat1_dn3) / (locals.var_fn169_calc_iq__vdsat1 * locals.var_fn169_calc_iq__vdsat1)))) * assign14590_e14600) + (assign14590_e14594 * (-(-((assign14590_e14597 * locals.var_fn169_calc_iq__vdsat1_dn3) / (locals.var_fn169_calc_iq__vdsat1 * locals.var_fn169_calc_iq__vdsat1)))))) / (2.0 * assign14590_e14604)))), (0.5 * ((-((assign14590_e14585 * locals.var_fn169_calc_iq__vdsat1_dn4) / (locals.var_fn169_calc_iq__vdsat1 * locals.var_fn169_calc_iq__vdsat1))) + ((((-(-((assign14590_e14591 * locals.var_fn169_calc_iq__vdsat1_dn4) / (locals.var_fn169_calc_iq__vdsat1 * locals.var_fn169_calc_iq__vdsat1)))) * assign14590_e14600) + (assign14590_e14594 * (-(-((assign14590_e14597 * locals.var_fn169_calc_iq__vdsat1_dn4) / (locals.var_fn169_calc_iq__vdsat1 * locals.var_fn169_calc_iq__vdsat1)))))) / (2.0 * assign14590_e14604)))), (0.5 * ((-((assign14590_e14585 * locals.var_fn169_calc_iq__vdsat1_dn7) / (locals.var_fn169_calc_iq__vdsat1 * locals.var_fn169_calc_iq__vdsat1))) + ((((-(-((assign14590_e14591 * locals.var_fn169_calc_iq__vdsat1_dn7) / (locals.var_fn169_calc_iq__vdsat1 * locals.var_fn169_calc_iq__vdsat1)))) * assign14590_e14600) + (assign14590_e14594 * (-(-((assign14590_e14597 * locals.var_fn169_calc_iq__vdsat1_dn7) / (locals.var_fn169_calc_iq__vdsat1 * locals.var_fn169_calc_iq__vdsat1)))))) / (2.0 * assign14590_e14604)))), (0.5 * (((((-locals.var_fn169_calc_iq__vdsin_dn9) * locals.var_fn169_calc_iq__vdsat1) - (assign14590_e14585 * locals.var_fn169_calc_iq__vdsat1_dn9)) / (locals.var_fn169_calc_iq__vdsat1 * locals.var_fn169_calc_iq__vdsat1)) + ((((-((((-locals.var_fn169_calc_iq__vdsin_dn9) * locals.var_fn169_calc_iq__vdsat1) - (assign14590_e14591 * locals.var_fn169_calc_iq__vdsat1_dn9)) / (locals.var_fn169_calc_iq__vdsat1 * locals.var_fn169_calc_iq__vdsat1))) * assign14590_e14600) + (assign14590_e14594 * (-((((-locals.var_fn169_calc_iq__vdsin_dn9) * locals.var_fn169_calc_iq__vdsat1) - (assign14590_e14597 * locals.var_fn169_calc_iq__vdsat1_dn9)) / (locals.var_fn169_calc_iq__vdsat1 * locals.var_fn169_calc_iq__vdsat1))))) / (2.0 * assign14590_e14604)))), (0.5 * (((((-locals.var_fn169_calc_iq__vdsin_dn10) * locals.var_fn169_calc_iq__vdsat1) - (assign14590_e14585 * locals.var_fn169_calc_iq__vdsat1_dn10)) / (locals.var_fn169_calc_iq__vdsat1 * locals.var_fn169_calc_iq__vdsat1)) + ((((-((((-locals.var_fn169_calc_iq__vdsin_dn10) * locals.var_fn169_calc_iq__vdsat1) - (assign14590_e14591 * locals.var_fn169_calc_iq__vdsat1_dn10)) / (locals.var_fn169_calc_iq__vdsat1 * locals.var_fn169_calc_iq__vdsat1))) * assign14590_e14600) + (assign14590_e14594 * (-((((-locals.var_fn169_calc_iq__vdsin_dn10) * locals.var_fn169_calc_iq__vdsat1) - (assign14590_e14597 * locals.var_fn169_calc_iq__vdsat1_dn10)) / (locals.var_fn169_calc_iq__vdsat1 * locals.var_fn169_calc_iq__vdsat1))))) / (2.0 * assign14590_e14604)))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign14590_e14608, assign14590_e14608_d_n2, assign14590_e14608_d_n3, assign14590_e14608_d_n4, assign14590_e14608_d_n7, assign14590_e14608_d_n9, assign14590_e14608_d_n10,)
            }
        };
        let assign14590_e14611: f64 = (assign14590_e14609).powf(locals.var_fn169_calc_iq__beta);
        let assign14590_e14612: f64 = (1.0 + assign14590_e14611);
        let assign14590_e14615: f64 = (1.0 / locals.var_fn169_calc_iq__beta);
        let assign14590_e14616: f64 = (assign14590_e14612).powf(assign14590_e14615);
        let assign14590_e14617: f64 = (1.0 / assign14590_e14616);
        (assign14590_e14617, (-(if 0.0 == 0.0 && ((assign14590_e14615) as f64).is_finite() && ((assign14590_e14615) as f64).fract() == 0.0 { if assign14590_e14615 == 0.0 { 0.0 } else { (assign14590_e14615 * ((assign14590_e14612).powf(assign14590_e14615 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn169_calc_iq__beta) as f64).is_finite() && ((locals.var_fn169_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn169_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn169_calc_iq__beta * ((assign14590_e14609).powf(locals.var_fn169_calc_iq__beta - 1.0) * assign14590_e14609_d_n2)) } } else { (assign14590_e14611 * (locals.var_fn169_calc_iq__beta * (assign14590_e14609_d_n2 / assign14590_e14609))) })) } } else { (assign14590_e14616 * (assign14590_e14615 * (if 0.0 == 0.0 && ((locals.var_fn169_calc_iq__beta) as f64).is_finite() && ((locals.var_fn169_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn169_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn169_calc_iq__beta * ((assign14590_e14609).powf(locals.var_fn169_calc_iq__beta - 1.0) * assign14590_e14609_d_n2)) } } else { (assign14590_e14611 * (locals.var_fn169_calc_iq__beta * (assign14590_e14609_d_n2 / assign14590_e14609))) } / assign14590_e14612))) } / (assign14590_e14616 * assign14590_e14616))), (-(if 0.0 == 0.0 && ((assign14590_e14615) as f64).is_finite() && ((assign14590_e14615) as f64).fract() == 0.0 { if assign14590_e14615 == 0.0 { 0.0 } else { (assign14590_e14615 * ((assign14590_e14612).powf(assign14590_e14615 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn169_calc_iq__beta) as f64).is_finite() && ((locals.var_fn169_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn169_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn169_calc_iq__beta * ((assign14590_e14609).powf(locals.var_fn169_calc_iq__beta - 1.0) * assign14590_e14609_d_n3)) } } else { (assign14590_e14611 * (locals.var_fn169_calc_iq__beta * (assign14590_e14609_d_n3 / assign14590_e14609))) })) } } else { (assign14590_e14616 * (assign14590_e14615 * (if 0.0 == 0.0 && ((locals.var_fn169_calc_iq__beta) as f64).is_finite() && ((locals.var_fn169_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn169_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn169_calc_iq__beta * ((assign14590_e14609).powf(locals.var_fn169_calc_iq__beta - 1.0) * assign14590_e14609_d_n3)) } } else { (assign14590_e14611 * (locals.var_fn169_calc_iq__beta * (assign14590_e14609_d_n3 / assign14590_e14609))) } / assign14590_e14612))) } / (assign14590_e14616 * assign14590_e14616))), (-(if 0.0 == 0.0 && ((assign14590_e14615) as f64).is_finite() && ((assign14590_e14615) as f64).fract() == 0.0 { if assign14590_e14615 == 0.0 { 0.0 } else { (assign14590_e14615 * ((assign14590_e14612).powf(assign14590_e14615 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn169_calc_iq__beta) as f64).is_finite() && ((locals.var_fn169_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn169_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn169_calc_iq__beta * ((assign14590_e14609).powf(locals.var_fn169_calc_iq__beta - 1.0) * assign14590_e14609_d_n4)) } } else { (assign14590_e14611 * (locals.var_fn169_calc_iq__beta * (assign14590_e14609_d_n4 / assign14590_e14609))) })) } } else { (assign14590_e14616 * (assign14590_e14615 * (if 0.0 == 0.0 && ((locals.var_fn169_calc_iq__beta) as f64).is_finite() && ((locals.var_fn169_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn169_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn169_calc_iq__beta * ((assign14590_e14609).powf(locals.var_fn169_calc_iq__beta - 1.0) * assign14590_e14609_d_n4)) } } else { (assign14590_e14611 * (locals.var_fn169_calc_iq__beta * (assign14590_e14609_d_n4 / assign14590_e14609))) } / assign14590_e14612))) } / (assign14590_e14616 * assign14590_e14616))), (-(if 0.0 == 0.0 && ((assign14590_e14615) as f64).is_finite() && ((assign14590_e14615) as f64).fract() == 0.0 { if assign14590_e14615 == 0.0 { 0.0 } else { (assign14590_e14615 * ((assign14590_e14612).powf(assign14590_e14615 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn169_calc_iq__beta) as f64).is_finite() && ((locals.var_fn169_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn169_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn169_calc_iq__beta * ((assign14590_e14609).powf(locals.var_fn169_calc_iq__beta - 1.0) * assign14590_e14609_d_n7)) } } else { (assign14590_e14611 * (locals.var_fn169_calc_iq__beta * (assign14590_e14609_d_n7 / assign14590_e14609))) })) } } else { (assign14590_e14616 * (assign14590_e14615 * (if 0.0 == 0.0 && ((locals.var_fn169_calc_iq__beta) as f64).is_finite() && ((locals.var_fn169_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn169_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn169_calc_iq__beta * ((assign14590_e14609).powf(locals.var_fn169_calc_iq__beta - 1.0) * assign14590_e14609_d_n7)) } } else { (assign14590_e14611 * (locals.var_fn169_calc_iq__beta * (assign14590_e14609_d_n7 / assign14590_e14609))) } / assign14590_e14612))) } / (assign14590_e14616 * assign14590_e14616))), (-(if 0.0 == 0.0 && ((assign14590_e14615) as f64).is_finite() && ((assign14590_e14615) as f64).fract() == 0.0 { if assign14590_e14615 == 0.0 { 0.0 } else { (assign14590_e14615 * ((assign14590_e14612).powf(assign14590_e14615 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn169_calc_iq__beta) as f64).is_finite() && ((locals.var_fn169_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn169_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn169_calc_iq__beta * ((assign14590_e14609).powf(locals.var_fn169_calc_iq__beta - 1.0) * assign14590_e14609_d_n9)) } } else { (assign14590_e14611 * (locals.var_fn169_calc_iq__beta * (assign14590_e14609_d_n9 / assign14590_e14609))) })) } } else { (assign14590_e14616 * (assign14590_e14615 * (if 0.0 == 0.0 && ((locals.var_fn169_calc_iq__beta) as f64).is_finite() && ((locals.var_fn169_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn169_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn169_calc_iq__beta * ((assign14590_e14609).powf(locals.var_fn169_calc_iq__beta - 1.0) * assign14590_e14609_d_n9)) } } else { (assign14590_e14611 * (locals.var_fn169_calc_iq__beta * (assign14590_e14609_d_n9 / assign14590_e14609))) } / assign14590_e14612))) } / (assign14590_e14616 * assign14590_e14616))), (-(if 0.0 == 0.0 && ((assign14590_e14615) as f64).is_finite() && ((assign14590_e14615) as f64).fract() == 0.0 { if assign14590_e14615 == 0.0 { 0.0 } else { (assign14590_e14615 * ((assign14590_e14612).powf(assign14590_e14615 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn169_calc_iq__beta) as f64).is_finite() && ((locals.var_fn169_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn169_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn169_calc_iq__beta * ((assign14590_e14609).powf(locals.var_fn169_calc_iq__beta - 1.0) * assign14590_e14609_d_n10)) } } else { (assign14590_e14611 * (locals.var_fn169_calc_iq__beta * (assign14590_e14609_d_n10 / assign14590_e14609))) })) } } else { (assign14590_e14616 * (assign14590_e14615 * (if 0.0 == 0.0 && ((locals.var_fn169_calc_iq__beta) as f64).is_finite() && ((locals.var_fn169_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn169_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn169_calc_iq__beta * ((assign14590_e14609).powf(locals.var_fn169_calc_iq__beta - 1.0) * assign14590_e14609_d_n10)) } } else { (assign14590_e14611 * (locals.var_fn169_calc_iq__beta * (assign14590_e14609_d_n10 / assign14590_e14609))) } / assign14590_e14612))) } / (assign14590_e14616 * assign14590_e14616))),)
    } else {
        (locals.var_fn169_calc_iq__fds, locals.var_fn169_calc_iq__fds_dn2, locals.var_fn169_calc_iq__fds_dn3, locals.var_fn169_calc_iq__fds_dn4, locals.var_fn169_calc_iq__fds_dn7, locals.var_fn169_calc_iq__fds_dn9, locals.var_fn169_calc_iq__fds_dn10,)
    }
};
        locals.var_fn169_calc_iq__fds = assign14590_e14619;
        locals.var_fn169_calc_iq__fds_dn2 = assign14590_e14619_d_n2;
        locals.var_fn169_calc_iq__fds_dn3 = assign14590_e14619_d_n3;
        locals.var_fn169_calc_iq__fds_dn4 = assign14590_e14619_d_n4;
        locals.var_fn169_calc_iq__fds_dn7 = assign14590_e14619_d_n7;
        locals.var_fn169_calc_iq__fds_dn9 = assign14590_e14619_d_n9;
        locals.var_fn169_calc_iq__fds_dn10 = assign14590_e14619_d_n10;

        let (assign14600_e14626, assign14600_e14626_d_n2, assign14600_e14626_d_n3, assign14600_e14626_d_n4, assign14600_e14626_d_n7, assign14600_e14626_d_n9, assign14600_e14626_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        let assign14600_e14622: f64 = (-locals.var_fn169_calc_iq__vdsin);
        let assign14600_e14624: f64 = (assign14600_e14622 * locals.var_fn169_calc_iq__fds);
        (assign14600_e14624, (assign14600_e14622 * locals.var_fn169_calc_iq__fds_dn2), (assign14600_e14622 * locals.var_fn169_calc_iq__fds_dn3), (assign14600_e14622 * locals.var_fn169_calc_iq__fds_dn4), (assign14600_e14622 * locals.var_fn169_calc_iq__fds_dn7), (((-locals.var_fn169_calc_iq__vdsin_dn9) * locals.var_fn169_calc_iq__fds) + (assign14600_e14622 * locals.var_fn169_calc_iq__fds_dn9)), (((-locals.var_fn169_calc_iq__vdsin_dn10) * locals.var_fn169_calc_iq__fds) + (assign14600_e14622 * locals.var_fn169_calc_iq__fds_dn10)),)
    } else {
        (locals.var_fn169_calc_iq__vsx, locals.var_fn169_calc_iq__vsx_dn2, locals.var_fn169_calc_iq__vsx_dn3, locals.var_fn169_calc_iq__vsx_dn4, locals.var_fn169_calc_iq__vsx_dn7, locals.var_fn169_calc_iq__vsx_dn9, locals.var_fn169_calc_iq__vsx_dn10,)
    }
};
        locals.var_fn169_calc_iq__vsx = assign14600_e14626;
        locals.var_fn169_calc_iq__vsx_dn2 = assign14600_e14626_d_n2;
        locals.var_fn169_calc_iq__vsx_dn3 = assign14600_e14626_d_n3;
        locals.var_fn169_calc_iq__vsx_dn4 = assign14600_e14626_d_n4;
        locals.var_fn169_calc_iq__vsx_dn7 = assign14600_e14626_d_n7;
        locals.var_fn169_calc_iq__vsx_dn9 = assign14600_e14626_d_n9;
        locals.var_fn169_calc_iq__vsx_dn10 = assign14600_e14626_d_n10;

        let (assign14610_e14634, assign14610_e14634_d_n2, assign14610_e14634_d_n3, assign14610_e14634_d_n4, assign14610_e14634_d_n7, assign14610_e14634_d_n9, assign14610_e14634_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        let assign14610_e14630: f64 = (locals.var_fn169_calc_iq__vgsin - locals.var_fn169_calc_iq__myarg);
        let assign14610_e14632: f64 = (assign14610_e14630 / locals.var_fn169_calc_iq__alpha_phit);
        (assign14610_e14632, ((locals.var_fn169_calc_iq__vgsin_dn2 - locals.var_fn169_calc_iq__myarg_dn2) / locals.var_fn169_calc_iq__alpha_phit), ((-locals.var_fn169_calc_iq__myarg_dn3) / locals.var_fn169_calc_iq__alpha_phit), ((((-locals.var_fn169_calc_iq__myarg_dn4) * locals.var_fn169_calc_iq__alpha_phit) - (assign14610_e14630 * locals.var_fn169_calc_iq__alpha_phit_dn4)) / (locals.var_fn169_calc_iq__alpha_phit * locals.var_fn169_calc_iq__alpha_phit)), ((locals.var_fn169_calc_iq__vgsin_dn7 - locals.var_fn169_calc_iq__myarg_dn7) / locals.var_fn169_calc_iq__alpha_phit), ((-locals.var_fn169_calc_iq__myarg_dn9) / locals.var_fn169_calc_iq__alpha_phit), ((locals.var_fn169_calc_iq__vgsin_dn10 - locals.var_fn169_calc_iq__myarg_dn10) / locals.var_fn169_calc_iq__alpha_phit),)
    } else {
        (locals.var_fn169_calc_iq__exparg, locals.var_fn169_calc_iq__exparg_dn2, locals.var_fn169_calc_iq__exparg_dn3, locals.var_fn169_calc_iq__exparg_dn4, locals.var_fn169_calc_iq__exparg_dn7, locals.var_fn169_calc_iq__exparg_dn9, locals.var_fn169_calc_iq__exparg_dn10,)
    }
};
        locals.var_fn169_calc_iq__exparg = assign14610_e14634;
        locals.var_fn169_calc_iq__exparg_dn2 = assign14610_e14634_d_n2;
        locals.var_fn169_calc_iq__exparg_dn3 = assign14610_e14634_d_n3;
        locals.var_fn169_calc_iq__exparg_dn4 = assign14610_e14634_d_n4;
        locals.var_fn169_calc_iq__exparg_dn7 = assign14610_e14634_d_n7;
        locals.var_fn169_calc_iq__exparg_dn9 = assign14610_e14634_d_n9;
        locals.var_fn169_calc_iq__exparg_dn10 = assign14610_e14634_d_n10;

        let assign14620_e14637: f64 = if locals.var_fn169_calc_iq__exparg > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard175 = assign14620_e14637;

        let (assign14630_e14643, assign14630_e14643_d_n2, assign14630_e14643_d_n3, assign14630_e14643_d_n4, assign14630_e14643_d_n7, assign14630_e14643_d_n9, assign14630_e14643_d_n10,) = {
    if ((locals.var_guard168 != 0.0) && (locals.var_guard175 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__ffs, locals.var_fn169_calc_iq__ffs_dn2, locals.var_fn169_calc_iq__ffs_dn3, locals.var_fn169_calc_iq__ffs_dn4, locals.var_fn169_calc_iq__ffs_dn7, locals.var_fn169_calc_iq__ffs_dn9, locals.var_fn169_calc_iq__ffs_dn10,)
    }
};
        locals.var_fn169_calc_iq__ffs = assign14630_e14643;
        locals.var_fn169_calc_iq__ffs_dn2 = assign14630_e14643_d_n2;
        locals.var_fn169_calc_iq__ffs_dn3 = assign14630_e14643_d_n3;
        locals.var_fn169_calc_iq__ffs_dn4 = assign14630_e14643_d_n4;
        locals.var_fn169_calc_iq__ffs_dn7 = assign14630_e14643_d_n7;
        locals.var_fn169_calc_iq__ffs_dn9 = assign14630_e14643_d_n9;
        locals.var_fn169_calc_iq__ffs_dn10 = assign14630_e14643_d_n10;

        let assign14640_e14646: f64 = (-50.0);
        let assign14640_e14647: f64 = if locals.var_fn169_calc_iq__exparg < assign14640_e14646 { 1.0 } else { 0.0 };
        locals.var_guard176 = assign14640_e14647;

        let (assign14650_e14656, assign14650_e14656_d_n2, assign14650_e14656_d_n3, assign14650_e14656_d_n4, assign14650_e14656_d_n7, assign14650_e14656_d_n9, assign14650_e14656_d_n10,) = {
    if (((locals.var_guard168 != 0.0) && (locals.var_guard175 == 0.0)) && (locals.var_guard176 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__ffs, locals.var_fn169_calc_iq__ffs_dn2, locals.var_fn169_calc_iq__ffs_dn3, locals.var_fn169_calc_iq__ffs_dn4, locals.var_fn169_calc_iq__ffs_dn7, locals.var_fn169_calc_iq__ffs_dn9, locals.var_fn169_calc_iq__ffs_dn10,)
    }
};
        locals.var_fn169_calc_iq__ffs = assign14650_e14656;
        locals.var_fn169_calc_iq__ffs_dn2 = assign14650_e14656_d_n2;
        locals.var_fn169_calc_iq__ffs_dn3 = assign14650_e14656_d_n3;
        locals.var_fn169_calc_iq__ffs_dn4 = assign14650_e14656_d_n4;
        locals.var_fn169_calc_iq__ffs_dn7 = assign14650_e14656_d_n7;
        locals.var_fn169_calc_iq__ffs_dn9 = assign14650_e14656_d_n9;
        locals.var_fn169_calc_iq__ffs_dn10 = assign14650_e14656_d_n10;

        let (assign14660_e14671, assign14660_e14671_d_n2, assign14660_e14671_d_n3, assign14660_e14671_d_n4, assign14660_e14671_d_n7, assign14660_e14671_d_n9, assign14660_e14671_d_n10,) = {
    if (((locals.var_guard168 != 0.0) && (locals.var_guard175 == 0.0)) && (locals.var_guard176 == 0.0)) {
        let assign14660_e14667: f64 = (locals.var_fn169_calc_iq__exparg).exp();
        let assign14660_e14668: f64 = (1.0 + assign14660_e14667);
        let assign14660_e14669: f64 = (1.0 / assign14660_e14668);
        (assign14660_e14669, (-((assign14660_e14667 * locals.var_fn169_calc_iq__exparg_dn2) / (assign14660_e14668 * assign14660_e14668))), (-((assign14660_e14667 * locals.var_fn169_calc_iq__exparg_dn3) / (assign14660_e14668 * assign14660_e14668))), (-((assign14660_e14667 * locals.var_fn169_calc_iq__exparg_dn4) / (assign14660_e14668 * assign14660_e14668))), (-((assign14660_e14667 * locals.var_fn169_calc_iq__exparg_dn7) / (assign14660_e14668 * assign14660_e14668))), (-((assign14660_e14667 * locals.var_fn169_calc_iq__exparg_dn9) / (assign14660_e14668 * assign14660_e14668))), (-((assign14660_e14667 * locals.var_fn169_calc_iq__exparg_dn10) / (assign14660_e14668 * assign14660_e14668))),)
    } else {
        (locals.var_fn169_calc_iq__ffs, locals.var_fn169_calc_iq__ffs_dn2, locals.var_fn169_calc_iq__ffs_dn3, locals.var_fn169_calc_iq__ffs_dn4, locals.var_fn169_calc_iq__ffs_dn7, locals.var_fn169_calc_iq__ffs_dn9, locals.var_fn169_calc_iq__ffs_dn10,)
    }
};
        locals.var_fn169_calc_iq__ffs = assign14660_e14671;
        locals.var_fn169_calc_iq__ffs_dn2 = assign14660_e14671_d_n2;
        locals.var_fn169_calc_iq__ffs_dn3 = assign14660_e14671_d_n3;
        locals.var_fn169_calc_iq__ffs_dn4 = assign14660_e14671_d_n4;
        locals.var_fn169_calc_iq__ffs_dn7 = assign14660_e14671_d_n7;
        locals.var_fn169_calc_iq__ffs_dn9 = assign14660_e14671_d_n9;
        locals.var_fn169_calc_iq__ffs_dn10 = assign14660_e14671_d_n10;

        let (assign14670_e14689, assign14670_e14689_d_n2, assign14670_e14689_d_n3, assign14670_e14689_d_n4, assign14670_e14689_d_n7, assign14670_e14689_d_n9, assign14670_e14689_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        let assign14670_e14675: f64 = (locals.var_fn169_calc_iq__vgdin - locals.var_fn169_calc_iq__vsx);
        let assign14670_e14679: f64 = (p.p51 * 0.1);
        let assign14670_e14681: f64 = (assign14670_e14679 * locals.var_fn169_calc_iq__alpha_phit);
        let assign14670_e14683: f64 = (assign14670_e14681 * locals.var_fn169_calc_iq__ffs);
        let assign14670_e14684: f64 = (locals.var_fn169_calc_iq__vtdibl - assign14670_e14683);
        let assign14670_e14685: f64 = (assign14670_e14675 - assign14670_e14684);
        let assign14670_e14687: f64 = (assign14670_e14685 / locals.var_fn169_calc_iq__two_n_phit);
        (assign14670_e14687, (((locals.var_fn169_calc_iq__vgdin_dn2 - locals.var_fn169_calc_iq__vsx_dn2) - (-(assign14670_e14681 * locals.var_fn169_calc_iq__ffs_dn2))) / locals.var_fn169_calc_iq__two_n_phit), (((-locals.var_fn169_calc_iq__vsx_dn3) - (-(assign14670_e14681 * locals.var_fn169_calc_iq__ffs_dn3))) / locals.var_fn169_calc_iq__two_n_phit), (((((-locals.var_fn169_calc_iq__vsx_dn4) - (locals.var_fn169_calc_iq__vtdibl_dn4 - (((assign14670_e14679 * locals.var_fn169_calc_iq__alpha_phit_dn4) * locals.var_fn169_calc_iq__ffs) + (assign14670_e14681 * locals.var_fn169_calc_iq__ffs_dn4)))) * locals.var_fn169_calc_iq__two_n_phit) - (assign14670_e14685 * locals.var_fn169_calc_iq__two_n_phit_dn4)) / (locals.var_fn169_calc_iq__two_n_phit * locals.var_fn169_calc_iq__two_n_phit)), (((locals.var_fn169_calc_iq__vgdin_dn7 - locals.var_fn169_calc_iq__vsx_dn7) - (-(assign14670_e14681 * locals.var_fn169_calc_iq__ffs_dn7))) / locals.var_fn169_calc_iq__two_n_phit), (((((locals.var_fn169_calc_iq__vgdin_dn9 - locals.var_fn169_calc_iq__vsx_dn9) - (locals.var_fn169_calc_iq__vtdibl_dn9 - (assign14670_e14681 * locals.var_fn169_calc_iq__ffs_dn9))) * locals.var_fn169_calc_iq__two_n_phit) - (assign14670_e14685 * locals.var_fn169_calc_iq__two_n_phit_dn9)) / (locals.var_fn169_calc_iq__two_n_phit * locals.var_fn169_calc_iq__two_n_phit)), (((((locals.var_fn169_calc_iq__vgdin_dn10 - locals.var_fn169_calc_iq__vsx_dn10) - (locals.var_fn169_calc_iq__vtdibl_dn10 - (assign14670_e14681 * locals.var_fn169_calc_iq__ffs_dn10))) * locals.var_fn169_calc_iq__two_n_phit) - (assign14670_e14685 * locals.var_fn169_calc_iq__two_n_phit_dn10)) / (locals.var_fn169_calc_iq__two_n_phit * locals.var_fn169_calc_iq__two_n_phit)),)
    } else {
        (locals.var_fn169_calc_iq__etas, locals.var_fn169_calc_iq__etas_dn2, locals.var_fn169_calc_iq__etas_dn3, locals.var_fn169_calc_iq__etas_dn4, locals.var_fn169_calc_iq__etas_dn7, locals.var_fn169_calc_iq__etas_dn9, locals.var_fn169_calc_iq__etas_dn10,)
    }
};
        locals.var_fn169_calc_iq__etas = assign14670_e14689;
        locals.var_fn169_calc_iq__etas_dn2 = assign14670_e14689_d_n2;
        locals.var_fn169_calc_iq__etas_dn3 = assign14670_e14689_d_n3;
        locals.var_fn169_calc_iq__etas_dn4 = assign14670_e14689_d_n4;
        locals.var_fn169_calc_iq__etas_dn7 = assign14670_e14689_d_n7;
        locals.var_fn169_calc_iq__etas_dn9 = assign14670_e14689_d_n9;
        locals.var_fn169_calc_iq__etas_dn10 = assign14670_e14689_d_n10;

        let assign14680_e14692: f64 = if locals.var_fn169_calc_iq__etas > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard177 = assign14680_e14692;

        let (assign14690_e14700, assign14690_e14700_d_n2, assign14690_e14700_d_n3, assign14690_e14700_d_n4, assign14690_e14700_d_n7, assign14690_e14700_d_n9, assign14690_e14700_d_n10,) = {
    if ((locals.var_guard168 != 0.0) && (locals.var_guard177 != 0.0)) {
        let assign14690_e14698: f64 = (locals.var_fn169_calc_iq__qref * locals.var_fn169_calc_iq__etas);
        (assign14690_e14698, (locals.var_fn169_calc_iq__qref * locals.var_fn169_calc_iq__etas_dn2), (locals.var_fn169_calc_iq__qref * locals.var_fn169_calc_iq__etas_dn3), ((locals.var_fn169_calc_iq__qref_dn4 * locals.var_fn169_calc_iq__etas) + (locals.var_fn169_calc_iq__qref * locals.var_fn169_calc_iq__etas_dn4)), (locals.var_fn169_calc_iq__qref * locals.var_fn169_calc_iq__etas_dn7), ((locals.var_fn169_calc_iq__qref_dn9 * locals.var_fn169_calc_iq__etas) + (locals.var_fn169_calc_iq__qref * locals.var_fn169_calc_iq__etas_dn9)), ((locals.var_fn169_calc_iq__qref_dn10 * locals.var_fn169_calc_iq__etas) + (locals.var_fn169_calc_iq__qref * locals.var_fn169_calc_iq__etas_dn10)),)
    } else {
        (locals.var_fn169_calc_iq__qinvs, locals.var_fn169_calc_iq__qinvs_dn2, locals.var_fn169_calc_iq__qinvs_dn3, locals.var_fn169_calc_iq__qinvs_dn4, locals.var_fn169_calc_iq__qinvs_dn7, locals.var_fn169_calc_iq__qinvs_dn9, locals.var_fn169_calc_iq__qinvs_dn10,)
    }
};
        locals.var_fn169_calc_iq__qinvs = assign14690_e14700;
        locals.var_fn169_calc_iq__qinvs_dn2 = assign14690_e14700_d_n2;
        locals.var_fn169_calc_iq__qinvs_dn3 = assign14690_e14700_d_n3;
        locals.var_fn169_calc_iq__qinvs_dn4 = assign14690_e14700_d_n4;
        locals.var_fn169_calc_iq__qinvs_dn7 = assign14690_e14700_d_n7;
        locals.var_fn169_calc_iq__qinvs_dn9 = assign14690_e14700_d_n9;
        locals.var_fn169_calc_iq__qinvs_dn10 = assign14690_e14700_d_n10;

        let assign14700_e14703: f64 = (-50.0);
        let assign14700_e14704: f64 = if locals.var_fn169_calc_iq__etas < assign14700_e14703 { 1.0 } else { 0.0 };
        locals.var_guard178 = assign14700_e14704;

        let (assign14710_e14716, assign14710_e14716_d_n2, assign14710_e14716_d_n3, assign14710_e14716_d_n4, assign14710_e14716_d_n7, assign14710_e14716_d_n9, assign14710_e14716_d_n10,) = {
    if (((locals.var_guard168 != 0.0) && (locals.var_guard177 == 0.0)) && (locals.var_guard178 != 0.0)) {
        let assign14710_e14713: f64 = (locals.var_fn169_calc_iq__etas).exp();
        let assign14710_e14714: f64 = (locals.var_fn169_calc_iq__qref * assign14710_e14713);
        (assign14710_e14714, (locals.var_fn169_calc_iq__qref * (assign14710_e14713 * locals.var_fn169_calc_iq__etas_dn2)), (locals.var_fn169_calc_iq__qref * (assign14710_e14713 * locals.var_fn169_calc_iq__etas_dn3)), ((locals.var_fn169_calc_iq__qref_dn4 * assign14710_e14713) + (locals.var_fn169_calc_iq__qref * (assign14710_e14713 * locals.var_fn169_calc_iq__etas_dn4))), (locals.var_fn169_calc_iq__qref * (assign14710_e14713 * locals.var_fn169_calc_iq__etas_dn7)), ((locals.var_fn169_calc_iq__qref_dn9 * assign14710_e14713) + (locals.var_fn169_calc_iq__qref * (assign14710_e14713 * locals.var_fn169_calc_iq__etas_dn9))), ((locals.var_fn169_calc_iq__qref_dn10 * assign14710_e14713) + (locals.var_fn169_calc_iq__qref * (assign14710_e14713 * locals.var_fn169_calc_iq__etas_dn10))),)
    } else {
        (locals.var_fn169_calc_iq__qinvs, locals.var_fn169_calc_iq__qinvs_dn2, locals.var_fn169_calc_iq__qinvs_dn3, locals.var_fn169_calc_iq__qinvs_dn4, locals.var_fn169_calc_iq__qinvs_dn7, locals.var_fn169_calc_iq__qinvs_dn9, locals.var_fn169_calc_iq__qinvs_dn10,)
    }
};
        locals.var_fn169_calc_iq__qinvs = assign14710_e14716;
        locals.var_fn169_calc_iq__qinvs_dn2 = assign14710_e14716_d_n2;
        locals.var_fn169_calc_iq__qinvs_dn3 = assign14710_e14716_d_n3;
        locals.var_fn169_calc_iq__qinvs_dn4 = assign14710_e14716_d_n4;
        locals.var_fn169_calc_iq__qinvs_dn7 = assign14710_e14716_d_n7;
        locals.var_fn169_calc_iq__qinvs_dn9 = assign14710_e14716_d_n9;
        locals.var_fn169_calc_iq__qinvs_dn10 = assign14710_e14716_d_n10;

        let (assign14720_e14732, assign14720_e14732_d_n2, assign14720_e14732_d_n3, assign14720_e14732_d_n4, assign14720_e14732_d_n7, assign14720_e14732_d_n9, assign14720_e14732_d_n10,) = {
    if (((locals.var_guard168 != 0.0) && (locals.var_guard177 == 0.0)) && (locals.var_guard178 == 0.0)) {
        let assign14720_e14727: f64 = (locals.var_fn169_calc_iq__etas).exp();
        let assign14720_e14728: f64 = (1.0 + assign14720_e14727);
        let assign14720_e14729: f64 = (assign14720_e14728).ln();
        let assign14720_e14730: f64 = (locals.var_fn169_calc_iq__qref * assign14720_e14729);
        (assign14720_e14730, (locals.var_fn169_calc_iq__qref * ((assign14720_e14727 * locals.var_fn169_calc_iq__etas_dn2) / assign14720_e14728)), (locals.var_fn169_calc_iq__qref * ((assign14720_e14727 * locals.var_fn169_calc_iq__etas_dn3) / assign14720_e14728)), ((locals.var_fn169_calc_iq__qref_dn4 * assign14720_e14729) + (locals.var_fn169_calc_iq__qref * ((assign14720_e14727 * locals.var_fn169_calc_iq__etas_dn4) / assign14720_e14728))), (locals.var_fn169_calc_iq__qref * ((assign14720_e14727 * locals.var_fn169_calc_iq__etas_dn7) / assign14720_e14728)), ((locals.var_fn169_calc_iq__qref_dn9 * assign14720_e14729) + (locals.var_fn169_calc_iq__qref * ((assign14720_e14727 * locals.var_fn169_calc_iq__etas_dn9) / assign14720_e14728))), ((locals.var_fn169_calc_iq__qref_dn10 * assign14720_e14729) + (locals.var_fn169_calc_iq__qref * ((assign14720_e14727 * locals.var_fn169_calc_iq__etas_dn10) / assign14720_e14728))),)
    } else {
        (locals.var_fn169_calc_iq__qinvs, locals.var_fn169_calc_iq__qinvs_dn2, locals.var_fn169_calc_iq__qinvs_dn3, locals.var_fn169_calc_iq__qinvs_dn4, locals.var_fn169_calc_iq__qinvs_dn7, locals.var_fn169_calc_iq__qinvs_dn9, locals.var_fn169_calc_iq__qinvs_dn10,)
    }
};
        locals.var_fn169_calc_iq__qinvs = assign14720_e14732;
        locals.var_fn169_calc_iq__qinvs_dn2 = assign14720_e14732_d_n2;
        locals.var_fn169_calc_iq__qinvs_dn3 = assign14720_e14732_d_n3;
        locals.var_fn169_calc_iq__qinvs_dn4 = assign14720_e14732_d_n4;
        locals.var_fn169_calc_iq__qinvs_dn7 = assign14720_e14732_d_n7;
        locals.var_fn169_calc_iq__qinvs_dn9 = assign14720_e14732_d_n9;
        locals.var_fn169_calc_iq__qinvs_dn10 = assign14720_e14732_d_n10;

        let (assign14730_e14740, assign14730_e14740_d_n2, assign14730_e14740_d_n3, assign14730_e14740_d_n4, assign14730_e14740_d_n7, assign14730_e14740_d_n9, assign14730_e14740_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        let assign14730_e14736: f64 = (locals.var_fn169_calc_iq__vgdin - locals.var_fn169_calc_iq__myarg);
        let assign14730_e14738: f64 = (assign14730_e14736 / locals.var_fn169_calc_iq__alpha_phit);
        (assign14730_e14738, ((locals.var_fn169_calc_iq__vgdin_dn2 - locals.var_fn169_calc_iq__myarg_dn2) / locals.var_fn169_calc_iq__alpha_phit), ((-locals.var_fn169_calc_iq__myarg_dn3) / locals.var_fn169_calc_iq__alpha_phit), ((((-locals.var_fn169_calc_iq__myarg_dn4) * locals.var_fn169_calc_iq__alpha_phit) - (assign14730_e14736 * locals.var_fn169_calc_iq__alpha_phit_dn4)) / (locals.var_fn169_calc_iq__alpha_phit * locals.var_fn169_calc_iq__alpha_phit)), ((locals.var_fn169_calc_iq__vgdin_dn7 - locals.var_fn169_calc_iq__myarg_dn7) / locals.var_fn169_calc_iq__alpha_phit), ((locals.var_fn169_calc_iq__vgdin_dn9 - locals.var_fn169_calc_iq__myarg_dn9) / locals.var_fn169_calc_iq__alpha_phit), ((locals.var_fn169_calc_iq__vgdin_dn10 - locals.var_fn169_calc_iq__myarg_dn10) / locals.var_fn169_calc_iq__alpha_phit),)
    } else {
        (locals.var_fn169_calc_iq__exparg, locals.var_fn169_calc_iq__exparg_dn2, locals.var_fn169_calc_iq__exparg_dn3, locals.var_fn169_calc_iq__exparg_dn4, locals.var_fn169_calc_iq__exparg_dn7, locals.var_fn169_calc_iq__exparg_dn9, locals.var_fn169_calc_iq__exparg_dn10,)
    }
};
        locals.var_fn169_calc_iq__exparg = assign14730_e14740;
        locals.var_fn169_calc_iq__exparg_dn2 = assign14730_e14740_d_n2;
        locals.var_fn169_calc_iq__exparg_dn3 = assign14730_e14740_d_n3;
        locals.var_fn169_calc_iq__exparg_dn4 = assign14730_e14740_d_n4;
        locals.var_fn169_calc_iq__exparg_dn7 = assign14730_e14740_d_n7;
        locals.var_fn169_calc_iq__exparg_dn9 = assign14730_e14740_d_n9;
        locals.var_fn169_calc_iq__exparg_dn10 = assign14730_e14740_d_n10;

        let assign14740_e14743: f64 = if locals.var_fn169_calc_iq__exparg > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard179 = assign14740_e14743;

        let (assign14750_e14749, assign14750_e14749_d_n2, assign14750_e14749_d_n3, assign14750_e14749_d_n4, assign14750_e14749_d_n7, assign14750_e14749_d_n9, assign14750_e14749_d_n10,) = {
    if ((locals.var_guard168 != 0.0) && (locals.var_guard179 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__ffd, locals.var_fn169_calc_iq__ffd_dn2, locals.var_fn169_calc_iq__ffd_dn3, locals.var_fn169_calc_iq__ffd_dn4, locals.var_fn169_calc_iq__ffd_dn7, locals.var_fn169_calc_iq__ffd_dn9, locals.var_fn169_calc_iq__ffd_dn10,)
    }
};
        locals.var_fn169_calc_iq__ffd = assign14750_e14749;
        locals.var_fn169_calc_iq__ffd_dn2 = assign14750_e14749_d_n2;
        locals.var_fn169_calc_iq__ffd_dn3 = assign14750_e14749_d_n3;
        locals.var_fn169_calc_iq__ffd_dn4 = assign14750_e14749_d_n4;
        locals.var_fn169_calc_iq__ffd_dn7 = assign14750_e14749_d_n7;
        locals.var_fn169_calc_iq__ffd_dn9 = assign14750_e14749_d_n9;
        locals.var_fn169_calc_iq__ffd_dn10 = assign14750_e14749_d_n10;

        let assign14760_e14752: f64 = (-50.0);
        let assign14760_e14753: f64 = if locals.var_fn169_calc_iq__exparg < assign14760_e14752 { 1.0 } else { 0.0 };
        locals.var_guard180 = assign14760_e14753;

        let (assign14770_e14762, assign14770_e14762_d_n2, assign14770_e14762_d_n3, assign14770_e14762_d_n4, assign14770_e14762_d_n7, assign14770_e14762_d_n9, assign14770_e14762_d_n10,) = {
    if (((locals.var_guard168 != 0.0) && (locals.var_guard179 == 0.0)) && (locals.var_guard180 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__ffd, locals.var_fn169_calc_iq__ffd_dn2, locals.var_fn169_calc_iq__ffd_dn3, locals.var_fn169_calc_iq__ffd_dn4, locals.var_fn169_calc_iq__ffd_dn7, locals.var_fn169_calc_iq__ffd_dn9, locals.var_fn169_calc_iq__ffd_dn10,)
    }
};
        locals.var_fn169_calc_iq__ffd = assign14770_e14762;
        locals.var_fn169_calc_iq__ffd_dn2 = assign14770_e14762_d_n2;
        locals.var_fn169_calc_iq__ffd_dn3 = assign14770_e14762_d_n3;
        locals.var_fn169_calc_iq__ffd_dn4 = assign14770_e14762_d_n4;
        locals.var_fn169_calc_iq__ffd_dn7 = assign14770_e14762_d_n7;
        locals.var_fn169_calc_iq__ffd_dn9 = assign14770_e14762_d_n9;
        locals.var_fn169_calc_iq__ffd_dn10 = assign14770_e14762_d_n10;

        let (assign14780_e14777, assign14780_e14777_d_n2, assign14780_e14777_d_n3, assign14780_e14777_d_n4, assign14780_e14777_d_n7, assign14780_e14777_d_n9, assign14780_e14777_d_n10,) = {
    if (((locals.var_guard168 != 0.0) && (locals.var_guard179 == 0.0)) && (locals.var_guard180 == 0.0)) {
        let assign14780_e14773: f64 = (locals.var_fn169_calc_iq__exparg).exp();
        let assign14780_e14774: f64 = (1.0 + assign14780_e14773);
        let assign14780_e14775: f64 = (1.0 / assign14780_e14774);
        (assign14780_e14775, (-((assign14780_e14773 * locals.var_fn169_calc_iq__exparg_dn2) / (assign14780_e14774 * assign14780_e14774))), (-((assign14780_e14773 * locals.var_fn169_calc_iq__exparg_dn3) / (assign14780_e14774 * assign14780_e14774))), (-((assign14780_e14773 * locals.var_fn169_calc_iq__exparg_dn4) / (assign14780_e14774 * assign14780_e14774))), (-((assign14780_e14773 * locals.var_fn169_calc_iq__exparg_dn7) / (assign14780_e14774 * assign14780_e14774))), (-((assign14780_e14773 * locals.var_fn169_calc_iq__exparg_dn9) / (assign14780_e14774 * assign14780_e14774))), (-((assign14780_e14773 * locals.var_fn169_calc_iq__exparg_dn10) / (assign14780_e14774 * assign14780_e14774))),)
    } else {
        (locals.var_fn169_calc_iq__ffd, locals.var_fn169_calc_iq__ffd_dn2, locals.var_fn169_calc_iq__ffd_dn3, locals.var_fn169_calc_iq__ffd_dn4, locals.var_fn169_calc_iq__ffd_dn7, locals.var_fn169_calc_iq__ffd_dn9, locals.var_fn169_calc_iq__ffd_dn10,)
    }
};
        locals.var_fn169_calc_iq__ffd = assign14780_e14777;
        locals.var_fn169_calc_iq__ffd_dn2 = assign14780_e14777_d_n2;
        locals.var_fn169_calc_iq__ffd_dn3 = assign14780_e14777_d_n3;
        locals.var_fn169_calc_iq__ffd_dn4 = assign14780_e14777_d_n4;
        locals.var_fn169_calc_iq__ffd_dn7 = assign14780_e14777_d_n7;
        locals.var_fn169_calc_iq__ffd_dn9 = assign14780_e14777_d_n9;
        locals.var_fn169_calc_iq__ffd_dn10 = assign14780_e14777_d_n10;

        let (assign14790_e14795, assign14790_e14795_d_n2, assign14790_e14795_d_n3, assign14790_e14795_d_n4, assign14790_e14795_d_n7, assign14790_e14795_d_n9, assign14790_e14795_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        let assign14790_e14781: f64 = (locals.var_fn169_calc_iq__vgsin - locals.var_fn169_calc_iq__vdx);
        let assign14790_e14785: f64 = (p.p51 * 0.1);
        let assign14790_e14787: f64 = (assign14790_e14785 * locals.var_fn169_calc_iq__alpha_phit);
        let assign14790_e14789: f64 = (assign14790_e14787 * locals.var_fn169_calc_iq__ffd);
        let assign14790_e14790: f64 = (locals.var_fn169_calc_iq__vtdibl - assign14790_e14789);
        let assign14790_e14791: f64 = (assign14790_e14781 - assign14790_e14790);
        let assign14790_e14793: f64 = (assign14790_e14791 / locals.var_fn169_calc_iq__two_n_phit);
        (assign14790_e14793, (((locals.var_fn169_calc_iq__vgsin_dn2 - locals.var_fn169_calc_iq__vdx_dn2) - (-(assign14790_e14787 * locals.var_fn169_calc_iq__ffd_dn2))) / locals.var_fn169_calc_iq__two_n_phit), (((-locals.var_fn169_calc_iq__vdx_dn3) - (-(assign14790_e14787 * locals.var_fn169_calc_iq__ffd_dn3))) / locals.var_fn169_calc_iq__two_n_phit), (((((-locals.var_fn169_calc_iq__vdx_dn4) - (locals.var_fn169_calc_iq__vtdibl_dn4 - (((assign14790_e14785 * locals.var_fn169_calc_iq__alpha_phit_dn4) * locals.var_fn169_calc_iq__ffd) + (assign14790_e14787 * locals.var_fn169_calc_iq__ffd_dn4)))) * locals.var_fn169_calc_iq__two_n_phit) - (assign14790_e14791 * locals.var_fn169_calc_iq__two_n_phit_dn4)) / (locals.var_fn169_calc_iq__two_n_phit * locals.var_fn169_calc_iq__two_n_phit)), (((locals.var_fn169_calc_iq__vgsin_dn7 - locals.var_fn169_calc_iq__vdx_dn7) - (-(assign14790_e14787 * locals.var_fn169_calc_iq__ffd_dn7))) / locals.var_fn169_calc_iq__two_n_phit), (((((-locals.var_fn169_calc_iq__vdx_dn9) - (locals.var_fn169_calc_iq__vtdibl_dn9 - (assign14790_e14787 * locals.var_fn169_calc_iq__ffd_dn9))) * locals.var_fn169_calc_iq__two_n_phit) - (assign14790_e14791 * locals.var_fn169_calc_iq__two_n_phit_dn9)) / (locals.var_fn169_calc_iq__two_n_phit * locals.var_fn169_calc_iq__two_n_phit)), (((((locals.var_fn169_calc_iq__vgsin_dn10 - locals.var_fn169_calc_iq__vdx_dn10) - (locals.var_fn169_calc_iq__vtdibl_dn10 - (assign14790_e14787 * locals.var_fn169_calc_iq__ffd_dn10))) * locals.var_fn169_calc_iq__two_n_phit) - (assign14790_e14791 * locals.var_fn169_calc_iq__two_n_phit_dn10)) / (locals.var_fn169_calc_iq__two_n_phit * locals.var_fn169_calc_iq__two_n_phit)),)
    } else {
        (locals.var_fn169_calc_iq__etad, locals.var_fn169_calc_iq__etad_dn2, locals.var_fn169_calc_iq__etad_dn3, locals.var_fn169_calc_iq__etad_dn4, locals.var_fn169_calc_iq__etad_dn7, locals.var_fn169_calc_iq__etad_dn9, locals.var_fn169_calc_iq__etad_dn10,)
    }
};
        locals.var_fn169_calc_iq__etad = assign14790_e14795;
        locals.var_fn169_calc_iq__etad_dn2 = assign14790_e14795_d_n2;
        locals.var_fn169_calc_iq__etad_dn3 = assign14790_e14795_d_n3;
        locals.var_fn169_calc_iq__etad_dn4 = assign14790_e14795_d_n4;
        locals.var_fn169_calc_iq__etad_dn7 = assign14790_e14795_d_n7;
        locals.var_fn169_calc_iq__etad_dn9 = assign14790_e14795_d_n9;
        locals.var_fn169_calc_iq__etad_dn10 = assign14790_e14795_d_n10;

        let assign14800_e14798: f64 = if locals.var_fn169_calc_iq__etad > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard181 = assign14800_e14798;

        let (assign14810_e14806, assign14810_e14806_d_n2, assign14810_e14806_d_n3, assign14810_e14806_d_n4, assign14810_e14806_d_n7, assign14810_e14806_d_n9, assign14810_e14806_d_n10,) = {
    if ((locals.var_guard168 != 0.0) && (locals.var_guard181 != 0.0)) {
        let assign14810_e14804: f64 = (locals.var_fn169_calc_iq__qref * locals.var_fn169_calc_iq__etad);
        (assign14810_e14804, (locals.var_fn169_calc_iq__qref * locals.var_fn169_calc_iq__etad_dn2), (locals.var_fn169_calc_iq__qref * locals.var_fn169_calc_iq__etad_dn3), ((locals.var_fn169_calc_iq__qref_dn4 * locals.var_fn169_calc_iq__etad) + (locals.var_fn169_calc_iq__qref * locals.var_fn169_calc_iq__etad_dn4)), (locals.var_fn169_calc_iq__qref * locals.var_fn169_calc_iq__etad_dn7), ((locals.var_fn169_calc_iq__qref_dn9 * locals.var_fn169_calc_iq__etad) + (locals.var_fn169_calc_iq__qref * locals.var_fn169_calc_iq__etad_dn9)), ((locals.var_fn169_calc_iq__qref_dn10 * locals.var_fn169_calc_iq__etad) + (locals.var_fn169_calc_iq__qref * locals.var_fn169_calc_iq__etad_dn10)),)
    } else {
        (locals.var_fn169_calc_iq__qinvd, locals.var_fn169_calc_iq__qinvd_dn2, locals.var_fn169_calc_iq__qinvd_dn3, locals.var_fn169_calc_iq__qinvd_dn4, locals.var_fn169_calc_iq__qinvd_dn7, locals.var_fn169_calc_iq__qinvd_dn9, locals.var_fn169_calc_iq__qinvd_dn10,)
    }
};
        locals.var_fn169_calc_iq__qinvd = assign14810_e14806;
        locals.var_fn169_calc_iq__qinvd_dn2 = assign14810_e14806_d_n2;
        locals.var_fn169_calc_iq__qinvd_dn3 = assign14810_e14806_d_n3;
        locals.var_fn169_calc_iq__qinvd_dn4 = assign14810_e14806_d_n4;
        locals.var_fn169_calc_iq__qinvd_dn7 = assign14810_e14806_d_n7;
        locals.var_fn169_calc_iq__qinvd_dn9 = assign14810_e14806_d_n9;
        locals.var_fn169_calc_iq__qinvd_dn10 = assign14810_e14806_d_n10;

        let assign14820_e14809: f64 = (-50.0);
        let assign14820_e14810: f64 = if locals.var_fn169_calc_iq__etad < assign14820_e14809 { 1.0 } else { 0.0 };
        locals.var_guard182 = assign14820_e14810;

        let (assign14830_e14822, assign14830_e14822_d_n2, assign14830_e14822_d_n3, assign14830_e14822_d_n4, assign14830_e14822_d_n7, assign14830_e14822_d_n9, assign14830_e14822_d_n10,) = {
    if (((locals.var_guard168 != 0.0) && (locals.var_guard181 == 0.0)) && (locals.var_guard182 != 0.0)) {
        let assign14830_e14819: f64 = (locals.var_fn169_calc_iq__etad).exp();
        let assign14830_e14820: f64 = (locals.var_fn169_calc_iq__qref * assign14830_e14819);
        (assign14830_e14820, (locals.var_fn169_calc_iq__qref * (assign14830_e14819 * locals.var_fn169_calc_iq__etad_dn2)), (locals.var_fn169_calc_iq__qref * (assign14830_e14819 * locals.var_fn169_calc_iq__etad_dn3)), ((locals.var_fn169_calc_iq__qref_dn4 * assign14830_e14819) + (locals.var_fn169_calc_iq__qref * (assign14830_e14819 * locals.var_fn169_calc_iq__etad_dn4))), (locals.var_fn169_calc_iq__qref * (assign14830_e14819 * locals.var_fn169_calc_iq__etad_dn7)), ((locals.var_fn169_calc_iq__qref_dn9 * assign14830_e14819) + (locals.var_fn169_calc_iq__qref * (assign14830_e14819 * locals.var_fn169_calc_iq__etad_dn9))), ((locals.var_fn169_calc_iq__qref_dn10 * assign14830_e14819) + (locals.var_fn169_calc_iq__qref * (assign14830_e14819 * locals.var_fn169_calc_iq__etad_dn10))),)
    } else {
        (locals.var_fn169_calc_iq__qinvd, locals.var_fn169_calc_iq__qinvd_dn2, locals.var_fn169_calc_iq__qinvd_dn3, locals.var_fn169_calc_iq__qinvd_dn4, locals.var_fn169_calc_iq__qinvd_dn7, locals.var_fn169_calc_iq__qinvd_dn9, locals.var_fn169_calc_iq__qinvd_dn10,)
    }
};
        locals.var_fn169_calc_iq__qinvd = assign14830_e14822;
        locals.var_fn169_calc_iq__qinvd_dn2 = assign14830_e14822_d_n2;
        locals.var_fn169_calc_iq__qinvd_dn3 = assign14830_e14822_d_n3;
        locals.var_fn169_calc_iq__qinvd_dn4 = assign14830_e14822_d_n4;
        locals.var_fn169_calc_iq__qinvd_dn7 = assign14830_e14822_d_n7;
        locals.var_fn169_calc_iq__qinvd_dn9 = assign14830_e14822_d_n9;
        locals.var_fn169_calc_iq__qinvd_dn10 = assign14830_e14822_d_n10;

        let (assign14840_e14838, assign14840_e14838_d_n2, assign14840_e14838_d_n3, assign14840_e14838_d_n4, assign14840_e14838_d_n7, assign14840_e14838_d_n9, assign14840_e14838_d_n10,) = {
    if (((locals.var_guard168 != 0.0) && (locals.var_guard181 == 0.0)) && (locals.var_guard182 == 0.0)) {
        let assign14840_e14833: f64 = (locals.var_fn169_calc_iq__etad).exp();
        let assign14840_e14834: f64 = (1.0 + assign14840_e14833);
        let assign14840_e14835: f64 = (assign14840_e14834).ln();
        let assign14840_e14836: f64 = (locals.var_fn169_calc_iq__qref * assign14840_e14835);
        (assign14840_e14836, (locals.var_fn169_calc_iq__qref * ((assign14840_e14833 * locals.var_fn169_calc_iq__etad_dn2) / assign14840_e14834)), (locals.var_fn169_calc_iq__qref * ((assign14840_e14833 * locals.var_fn169_calc_iq__etad_dn3) / assign14840_e14834)), ((locals.var_fn169_calc_iq__qref_dn4 * assign14840_e14835) + (locals.var_fn169_calc_iq__qref * ((assign14840_e14833 * locals.var_fn169_calc_iq__etad_dn4) / assign14840_e14834))), (locals.var_fn169_calc_iq__qref * ((assign14840_e14833 * locals.var_fn169_calc_iq__etad_dn7) / assign14840_e14834)), ((locals.var_fn169_calc_iq__qref_dn9 * assign14840_e14835) + (locals.var_fn169_calc_iq__qref * ((assign14840_e14833 * locals.var_fn169_calc_iq__etad_dn9) / assign14840_e14834))), ((locals.var_fn169_calc_iq__qref_dn10 * assign14840_e14835) + (locals.var_fn169_calc_iq__qref * ((assign14840_e14833 * locals.var_fn169_calc_iq__etad_dn10) / assign14840_e14834))),)
    } else {
        (locals.var_fn169_calc_iq__qinvd, locals.var_fn169_calc_iq__qinvd_dn2, locals.var_fn169_calc_iq__qinvd_dn3, locals.var_fn169_calc_iq__qinvd_dn4, locals.var_fn169_calc_iq__qinvd_dn7, locals.var_fn169_calc_iq__qinvd_dn9, locals.var_fn169_calc_iq__qinvd_dn10,)
    }
};
        locals.var_fn169_calc_iq__qinvd = assign14840_e14838;
        locals.var_fn169_calc_iq__qinvd_dn2 = assign14840_e14838_d_n2;
        locals.var_fn169_calc_iq__qinvd_dn3 = assign14840_e14838_d_n3;
        locals.var_fn169_calc_iq__qinvd_dn4 = assign14840_e14838_d_n4;
        locals.var_fn169_calc_iq__qinvd_dn7 = assign14840_e14838_d_n7;
        locals.var_fn169_calc_iq__qinvd_dn9 = assign14840_e14838_d_n9;
        locals.var_fn169_calc_iq__qinvd_dn10 = assign14840_e14838_d_n10;

        let (assign14850_e14846, assign14850_e14846_d_n2, assign14850_e14846_d_n3, assign14850_e14846_d_n4, assign14850_e14846_d_n7, assign14850_e14846_d_n9, assign14850_e14846_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        let assign14850_e14842: f64 = (locals.var_fn169_calc_iq__qinvs - locals.var_fn169_calc_iq__qinvd);
        let assign14850_e14844: f64 = (assign14850_e14842 / locals.var_fn169_calc_iq__cgin);
        (assign14850_e14844, ((locals.var_fn169_calc_iq__qinvs_dn2 - locals.var_fn169_calc_iq__qinvd_dn2) / locals.var_fn169_calc_iq__cgin), ((locals.var_fn169_calc_iq__qinvs_dn3 - locals.var_fn169_calc_iq__qinvd_dn3) / locals.var_fn169_calc_iq__cgin), ((((locals.var_fn169_calc_iq__qinvs_dn4 - locals.var_fn169_calc_iq__qinvd_dn4) * locals.var_fn169_calc_iq__cgin) - (assign14850_e14842 * locals.var_fn169_calc_iq__cgin_dn4)) / (locals.var_fn169_calc_iq__cgin * locals.var_fn169_calc_iq__cgin)), ((locals.var_fn169_calc_iq__qinvs_dn7 - locals.var_fn169_calc_iq__qinvd_dn7) / locals.var_fn169_calc_iq__cgin), ((locals.var_fn169_calc_iq__qinvs_dn9 - locals.var_fn169_calc_iq__qinvd_dn9) / locals.var_fn169_calc_iq__cgin), ((locals.var_fn169_calc_iq__qinvs_dn10 - locals.var_fn169_calc_iq__qinvd_dn10) / locals.var_fn169_calc_iq__cgin),)
    } else {
        (locals.var_fn169_calc_iq__vdsc, locals.var_fn169_calc_iq__vdsc_dn2, locals.var_fn169_calc_iq__vdsc_dn3, locals.var_fn169_calc_iq__vdsc_dn4, locals.var_fn169_calc_iq__vdsc_dn7, locals.var_fn169_calc_iq__vdsc_dn9, locals.var_fn169_calc_iq__vdsc_dn10,)
    }
};
        locals.var_fn169_calc_iq__vdsc = assign14850_e14846;
        locals.var_fn169_calc_iq__vdsc_dn2 = assign14850_e14846_d_n2;
        locals.var_fn169_calc_iq__vdsc_dn3 = assign14850_e14846_d_n3;
        locals.var_fn169_calc_iq__vdsc_dn4 = assign14850_e14846_d_n4;
        locals.var_fn169_calc_iq__vdsc_dn7 = assign14850_e14846_d_n7;
        locals.var_fn169_calc_iq__vdsc_dn9 = assign14850_e14846_d_n9;
        locals.var_fn169_calc_iq__vdsc_dn10 = assign14850_e14846_d_n10;

        let (assign14860_e14852, assign14860_e14852_d_n2, assign14860_e14852_d_n3, assign14860_e14852_d_n4, assign14860_e14852_d_n7, assign14860_e14852_d_n9, assign14860_e14852_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        let assign14860_e14850: f64 = (locals.var_fn169_calc_iq__vdsc / locals.var_fn169_calc_iq__vdsat);
        (assign14860_e14850, (((locals.var_fn169_calc_iq__vdsc_dn2 * locals.var_fn169_calc_iq__vdsat) - (locals.var_fn169_calc_iq__vdsc * locals.var_fn169_calc_iq__vdsat_dn2)) / (locals.var_fn169_calc_iq__vdsat * locals.var_fn169_calc_iq__vdsat)), (((locals.var_fn169_calc_iq__vdsc_dn3 * locals.var_fn169_calc_iq__vdsat) - (locals.var_fn169_calc_iq__vdsc * locals.var_fn169_calc_iq__vdsat_dn3)) / (locals.var_fn169_calc_iq__vdsat * locals.var_fn169_calc_iq__vdsat)), (((locals.var_fn169_calc_iq__vdsc_dn4 * locals.var_fn169_calc_iq__vdsat) - (locals.var_fn169_calc_iq__vdsc * locals.var_fn169_calc_iq__vdsat_dn4)) / (locals.var_fn169_calc_iq__vdsat * locals.var_fn169_calc_iq__vdsat)), (((locals.var_fn169_calc_iq__vdsc_dn7 * locals.var_fn169_calc_iq__vdsat) - (locals.var_fn169_calc_iq__vdsc * locals.var_fn169_calc_iq__vdsat_dn7)) / (locals.var_fn169_calc_iq__vdsat * locals.var_fn169_calc_iq__vdsat)), (((locals.var_fn169_calc_iq__vdsc_dn9 * locals.var_fn169_calc_iq__vdsat) - (locals.var_fn169_calc_iq__vdsc * locals.var_fn169_calc_iq__vdsat_dn9)) / (locals.var_fn169_calc_iq__vdsat * locals.var_fn169_calc_iq__vdsat)), (((locals.var_fn169_calc_iq__vdsc_dn10 * locals.var_fn169_calc_iq__vdsat) - (locals.var_fn169_calc_iq__vdsc * locals.var_fn169_calc_iq__vdsat_dn10)) / (locals.var_fn169_calc_iq__vdsat * locals.var_fn169_calc_iq__vdsat)),)
    } else {
        (locals.var_fn169_calc_iq__myarg, locals.var_fn169_calc_iq__myarg_dn2, locals.var_fn169_calc_iq__myarg_dn3, locals.var_fn169_calc_iq__myarg_dn4, locals.var_fn169_calc_iq__myarg_dn7, locals.var_fn169_calc_iq__myarg_dn9, locals.var_fn169_calc_iq__myarg_dn10,)
    }
};
        locals.var_fn169_calc_iq__myarg = assign14860_e14852;
        locals.var_fn169_calc_iq__myarg_dn2 = assign14860_e14852_d_n2;
        locals.var_fn169_calc_iq__myarg_dn3 = assign14860_e14852_d_n3;
        locals.var_fn169_calc_iq__myarg_dn4 = assign14860_e14852_d_n4;
        locals.var_fn169_calc_iq__myarg_dn7 = assign14860_e14852_d_n7;
        locals.var_fn169_calc_iq__myarg_dn9 = assign14860_e14852_d_n9;
        locals.var_fn169_calc_iq__myarg_dn10 = assign14860_e14852_d_n10;

        let (assign14900_e14921, assign14900_e14921_d_n4,) = {
    if (locals.var_guard168 != 0.0) {
        let assign14900_e14918: f64 = (2.302585092994046 * locals.var_fn169_calc_iq__phitin);
        let assign14900_e14919: f64 = (locals.var_fn169_calc_iq__ss / assign14900_e14918);
        (assign14900_e14919, (-((locals.var_fn169_calc_iq__ss * (2.302585092994046 * locals.var_fn169_calc_iq__phitin_dn4)) / (assign14900_e14918 * assign14900_e14918))),)
    } else {
        (locals.var_fn169_calc_iq__n0, locals.var_fn169_calc_iq__n0_dn4,)
    }
};
        locals.var_fn169_calc_iq__n0 = assign14900_e14921;
        locals.var_fn169_calc_iq__n0_dn4 = assign14900_e14921_d_n4;

        let (assign14910_e14929, assign14910_e14929_d_n4,) = {
    if (locals.var_guard168 != 0.0) {
        let assign14910_e14925: f64 = (2.0 * locals.var_fn169_calc_iq__n0);
        let assign14910_e14927: f64 = (assign14910_e14925 * locals.var_fn169_calc_iq__phitin);
        (assign14910_e14927, (((2.0 * locals.var_fn169_calc_iq__n0_dn4) * locals.var_fn169_calc_iq__phitin) + (assign14910_e14925 * locals.var_fn169_calc_iq__phitin_dn4)),)
    } else {
        (locals.var_fn169_calc_iq__two_n_phit0, locals.var_fn169_calc_iq__two_n_phit0_dn4,)
    }
};
        locals.var_fn169_calc_iq__two_n_phit0 = assign14910_e14929;
        locals.var_fn169_calc_iq__two_n_phit0_dn4 = assign14910_e14929_d_n4;

        let (assign14920_e14935, assign14920_e14935_d_n4,) = {
    if (locals.var_guard168 != 0.0) {
        let assign14920_e14933: f64 = (locals.var_fn169_calc_iq__cgin * locals.var_fn169_calc_iq__two_n_phit0);
        (assign14920_e14933, ((locals.var_fn169_calc_iq__cgin_dn4 * locals.var_fn169_calc_iq__two_n_phit0) + (locals.var_fn169_calc_iq__cgin * locals.var_fn169_calc_iq__two_n_phit0_dn4)),)
    } else {
        (locals.var_fn169_calc_iq__qref0, locals.var_fn169_calc_iq__qref0_dn4,)
    }
};
        locals.var_fn169_calc_iq__qref0 = assign14920_e14935;
        locals.var_fn169_calc_iq__qref0_dn4 = assign14920_e14935_d_n4;

        let (assign14930_e14945, assign14930_e14945_d_n4,) = {
    if (locals.var_guard168 != 0.0) {
        let assign14930_e14940: f64 = (p.p51 * locals.var_fn169_calc_iq__alpha_phit);
        let assign14930_e14942: f64 = (assign14930_e14940 / 2.0);
        let assign14930_e14943: f64 = (locals.var_fn169_calc_iq__vtof - assign14930_e14942);
        (assign14930_e14943, (locals.var_fn169_calc_iq__vtof_dn4 - ((p.p51 * locals.var_fn169_calc_iq__alpha_phit_dn4) / 2.0)),)
    } else {
        (locals.var_fn169_calc_iq__myarg0, locals.var_fn169_calc_iq__myarg0_dn4,)
    }
};
        locals.var_fn169_calc_iq__myarg0 = assign14930_e14945;
        locals.var_fn169_calc_iq__myarg0_dn4 = assign14930_e14945_d_n4;

    }

    pub(super) fn stamp_transient_block_37(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign14940_e14996, assign14940_e14996_d_n2, assign14940_e14996_d_n4, assign14940_e14996_d_n7, assign14940_e14996_d_n9, assign14940_e14996_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        let (assign14940_e14990, assign14940_e14990_d_n2, assign14940_e14990_d_n7, assign14940_e14990_d_n9, assign14940_e14990_d_n10,) = {
            if (p.p52 != 0.0) {
                let assign14940_e14954: f64 = (locals.var_fn169_calc_iq__vgsin + locals.var_fn169_calc_iq__vgdin);
                let assign14940_e14957: f64 = (locals.var_fn169_calc_iq__vgsin - locals.var_fn169_calc_iq__vgdin);
                let assign14940_e14960: f64 = (0.001 / p.p53);
                let assign14940_e14963: f64 = (locals.var_fn169_calc_iq__vgsin - locals.var_fn169_calc_iq__vgdin);
                let assign14940_e14964: f64 = (assign14940_e14960 * assign14940_e14963);
                let assign14940_e14965: f64 = (assign14940_e14964).tanh();
                let assign14940_e14966: f64 = (assign14940_e14957 * assign14940_e14965);
                let assign14940_e14967: f64 = (assign14940_e14954 + assign14940_e14966);
                let assign14940_e14968: f64 = (0.5 * assign14940_e14967);
                (assign14940_e14968, (0.5 * ((locals.var_fn169_calc_iq__vgsin_dn2 + locals.var_fn169_calc_iq__vgdin_dn2) + (((locals.var_fn169_calc_iq__vgsin_dn2 - locals.var_fn169_calc_iq__vgdin_dn2) * assign14940_e14965) + (assign14940_e14957 * ((assign14940_e14960 * (locals.var_fn169_calc_iq__vgsin_dn2 - locals.var_fn169_calc_iq__vgdin_dn2)) / ((assign14940_e14964).cosh() * (assign14940_e14964).cosh())))))), (0.5 * ((locals.var_fn169_calc_iq__vgsin_dn7 + locals.var_fn169_calc_iq__vgdin_dn7) + (((locals.var_fn169_calc_iq__vgsin_dn7 - locals.var_fn169_calc_iq__vgdin_dn7) * assign14940_e14965) + (assign14940_e14957 * ((assign14940_e14960 * (locals.var_fn169_calc_iq__vgsin_dn7 - locals.var_fn169_calc_iq__vgdin_dn7)) / ((assign14940_e14964).cosh() * (assign14940_e14964).cosh())))))), (0.5 * (locals.var_fn169_calc_iq__vgdin_dn9 + (((-locals.var_fn169_calc_iq__vgdin_dn9) * assign14940_e14965) + (assign14940_e14957 * ((assign14940_e14960 * (-locals.var_fn169_calc_iq__vgdin_dn9)) / ((assign14940_e14964).cosh() * (assign14940_e14964).cosh())))))), (0.5 * ((locals.var_fn169_calc_iq__vgsin_dn10 + locals.var_fn169_calc_iq__vgdin_dn10) + (((locals.var_fn169_calc_iq__vgsin_dn10 - locals.var_fn169_calc_iq__vgdin_dn10) * assign14940_e14965) + (assign14940_e14957 * ((assign14940_e14960 * (locals.var_fn169_calc_iq__vgsin_dn10 - locals.var_fn169_calc_iq__vgdin_dn10)) / ((assign14940_e14964).cosh() * (assign14940_e14964).cosh())))))),)
            } else {
                let (assign14940_e14989, assign14940_e14989_d_n2, assign14940_e14989_d_n7, assign14940_e14989_d_n9, assign14940_e14989_d_n10,) = {
                    if (p.p52 == 0.0) {
                        let assign14940_e14975: f64 = (locals.var_fn169_calc_iq__vgsin + locals.var_fn169_calc_iq__vgdin);
                        let assign14940_e14978: f64 = (locals.var_fn169_calc_iq__vgsin - locals.var_fn169_calc_iq__vgdin);
                        let assign14940_e14981: f64 = (locals.var_fn169_calc_iq__vgsin - locals.var_fn169_calc_iq__vgdin);
                        let assign14940_e14982: f64 = (assign14940_e14978 * assign14940_e14981);
                        let assign14940_e14984: f64 = (assign14940_e14982 + p.p53);
                        let assign14940_e14985: f64 = (assign14940_e14984).sqrt();
                        let assign14940_e14986: f64 = (assign14940_e14975 + assign14940_e14985);
                        let assign14940_e14987: f64 = (0.5 * assign14940_e14986);
                        (assign14940_e14987, (0.5 * ((locals.var_fn169_calc_iq__vgsin_dn2 + locals.var_fn169_calc_iq__vgdin_dn2) + ((((locals.var_fn169_calc_iq__vgsin_dn2 - locals.var_fn169_calc_iq__vgdin_dn2) * assign14940_e14981) + (assign14940_e14978 * (locals.var_fn169_calc_iq__vgsin_dn2 - locals.var_fn169_calc_iq__vgdin_dn2))) / (2.0 * assign14940_e14985)))), (0.5 * ((locals.var_fn169_calc_iq__vgsin_dn7 + locals.var_fn169_calc_iq__vgdin_dn7) + ((((locals.var_fn169_calc_iq__vgsin_dn7 - locals.var_fn169_calc_iq__vgdin_dn7) * assign14940_e14981) + (assign14940_e14978 * (locals.var_fn169_calc_iq__vgsin_dn7 - locals.var_fn169_calc_iq__vgdin_dn7))) / (2.0 * assign14940_e14985)))), (0.5 * (locals.var_fn169_calc_iq__vgdin_dn9 + ((((-locals.var_fn169_calc_iq__vgdin_dn9) * assign14940_e14981) + (assign14940_e14978 * (-locals.var_fn169_calc_iq__vgdin_dn9))) / (2.0 * assign14940_e14985)))), (0.5 * ((locals.var_fn169_calc_iq__vgsin_dn10 + locals.var_fn169_calc_iq__vgdin_dn10) + ((((locals.var_fn169_calc_iq__vgsin_dn10 - locals.var_fn169_calc_iq__vgdin_dn10) * assign14940_e14981) + (assign14940_e14978 * (locals.var_fn169_calc_iq__vgsin_dn10 - locals.var_fn169_calc_iq__vgdin_dn10))) / (2.0 * assign14940_e14985)))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign14940_e14989, assign14940_e14989_d_n2, assign14940_e14989_d_n7, assign14940_e14989_d_n9, assign14940_e14989_d_n10,)
            }
        };
        let assign14940_e14992: f64 = (assign14940_e14990 - locals.var_fn169_calc_iq__myarg0);
        let assign14940_e14994: f64 = (assign14940_e14992 / locals.var_fn169_calc_iq__alpha_phit);
        (assign14940_e14994, (assign14940_e14990_d_n2 / locals.var_fn169_calc_iq__alpha_phit), ((((-locals.var_fn169_calc_iq__myarg0_dn4) * locals.var_fn169_calc_iq__alpha_phit) - (assign14940_e14992 * locals.var_fn169_calc_iq__alpha_phit_dn4)) / (locals.var_fn169_calc_iq__alpha_phit * locals.var_fn169_calc_iq__alpha_phit)), (assign14940_e14990_d_n7 / locals.var_fn169_calc_iq__alpha_phit), (assign14940_e14990_d_n9 / locals.var_fn169_calc_iq__alpha_phit), (assign14940_e14990_d_n10 / locals.var_fn169_calc_iq__alpha_phit),)
    } else {
        (locals.var_fn169_calc_iq__exparg0, locals.var_fn169_calc_iq__exparg0_dn2, locals.var_fn169_calc_iq__exparg0_dn4, locals.var_fn169_calc_iq__exparg0_dn7, locals.var_fn169_calc_iq__exparg0_dn9, locals.var_fn169_calc_iq__exparg0_dn10,)
    }
};
        locals.var_fn169_calc_iq__exparg0 = assign14940_e14996;
        locals.var_fn169_calc_iq__exparg0_dn2 = assign14940_e14996_d_n2;
        locals.var_fn169_calc_iq__exparg0_dn4 = assign14940_e14996_d_n4;
        locals.var_fn169_calc_iq__exparg0_dn7 = assign14940_e14996_d_n7;
        locals.var_fn169_calc_iq__exparg0_dn9 = assign14940_e14996_d_n9;
        locals.var_fn169_calc_iq__exparg0_dn10 = assign14940_e14996_d_n10;

        let assign14950_e14999: f64 = if locals.var_fn169_calc_iq__exparg0 > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard183 = assign14950_e14999;

        let (assign14960_e15005, assign14960_e15005_d_n2, assign14960_e15005_d_n4, assign14960_e15005_d_n7, assign14960_e15005_d_n9, assign14960_e15005_d_n10,) = {
    if ((locals.var_guard168 != 0.0) && (locals.var_guard183 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__ff0, locals.var_fn169_calc_iq__ff0_dn2, locals.var_fn169_calc_iq__ff0_dn4, locals.var_fn169_calc_iq__ff0_dn7, locals.var_fn169_calc_iq__ff0_dn9, locals.var_fn169_calc_iq__ff0_dn10,)
    }
};
        locals.var_fn169_calc_iq__ff0 = assign14960_e15005;
        locals.var_fn169_calc_iq__ff0_dn2 = assign14960_e15005_d_n2;
        locals.var_fn169_calc_iq__ff0_dn4 = assign14960_e15005_d_n4;
        locals.var_fn169_calc_iq__ff0_dn7 = assign14960_e15005_d_n7;
        locals.var_fn169_calc_iq__ff0_dn9 = assign14960_e15005_d_n9;
        locals.var_fn169_calc_iq__ff0_dn10 = assign14960_e15005_d_n10;

        let assign14970_e15008: f64 = (-50.0);
        let assign14970_e15009: f64 = if locals.var_fn169_calc_iq__exparg0 < assign14970_e15008 { 1.0 } else { 0.0 };
        locals.var_guard184 = assign14970_e15009;

        let (assign14980_e15018, assign14980_e15018_d_n2, assign14980_e15018_d_n4, assign14980_e15018_d_n7, assign14980_e15018_d_n9, assign14980_e15018_d_n10,) = {
    if (((locals.var_guard168 != 0.0) && (locals.var_guard183 == 0.0)) && (locals.var_guard184 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__ff0, locals.var_fn169_calc_iq__ff0_dn2, locals.var_fn169_calc_iq__ff0_dn4, locals.var_fn169_calc_iq__ff0_dn7, locals.var_fn169_calc_iq__ff0_dn9, locals.var_fn169_calc_iq__ff0_dn10,)
    }
};
        locals.var_fn169_calc_iq__ff0 = assign14980_e15018;
        locals.var_fn169_calc_iq__ff0_dn2 = assign14980_e15018_d_n2;
        locals.var_fn169_calc_iq__ff0_dn4 = assign14980_e15018_d_n4;
        locals.var_fn169_calc_iq__ff0_dn7 = assign14980_e15018_d_n7;
        locals.var_fn169_calc_iq__ff0_dn9 = assign14980_e15018_d_n9;
        locals.var_fn169_calc_iq__ff0_dn10 = assign14980_e15018_d_n10;

        let (assign14990_e15033, assign14990_e15033_d_n2, assign14990_e15033_d_n4, assign14990_e15033_d_n7, assign14990_e15033_d_n9, assign14990_e15033_d_n10,) = {
    if (((locals.var_guard168 != 0.0) && (locals.var_guard183 == 0.0)) && (locals.var_guard184 == 0.0)) {
        let assign14990_e15029: f64 = (locals.var_fn169_calc_iq__exparg0).exp();
        let assign14990_e15030: f64 = (1.0 + assign14990_e15029);
        let assign14990_e15031: f64 = (1.0 / assign14990_e15030);
        (assign14990_e15031, (-((assign14990_e15029 * locals.var_fn169_calc_iq__exparg0_dn2) / (assign14990_e15030 * assign14990_e15030))), (-((assign14990_e15029 * locals.var_fn169_calc_iq__exparg0_dn4) / (assign14990_e15030 * assign14990_e15030))), (-((assign14990_e15029 * locals.var_fn169_calc_iq__exparg0_dn7) / (assign14990_e15030 * assign14990_e15030))), (-((assign14990_e15029 * locals.var_fn169_calc_iq__exparg0_dn9) / (assign14990_e15030 * assign14990_e15030))), (-((assign14990_e15029 * locals.var_fn169_calc_iq__exparg0_dn10) / (assign14990_e15030 * assign14990_e15030))),)
    } else {
        (locals.var_fn169_calc_iq__ff0, locals.var_fn169_calc_iq__ff0_dn2, locals.var_fn169_calc_iq__ff0_dn4, locals.var_fn169_calc_iq__ff0_dn7, locals.var_fn169_calc_iq__ff0_dn9, locals.var_fn169_calc_iq__ff0_dn10,)
    }
};
        locals.var_fn169_calc_iq__ff0 = assign14990_e15033;
        locals.var_fn169_calc_iq__ff0_dn2 = assign14990_e15033_d_n2;
        locals.var_fn169_calc_iq__ff0_dn4 = assign14990_e15033_d_n4;
        locals.var_fn169_calc_iq__ff0_dn7 = assign14990_e15033_d_n7;
        locals.var_fn169_calc_iq__ff0_dn9 = assign14990_e15033_d_n9;
        locals.var_fn169_calc_iq__ff0_dn10 = assign14990_e15033_d_n10;

        let (assign15000_e15092, assign15000_e15092_d_n2, assign15000_e15092_d_n4, assign15000_e15092_d_n7, assign15000_e15092_d_n9, assign15000_e15092_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        let (assign15000_e15078, assign15000_e15078_d_n2, assign15000_e15078_d_n7, assign15000_e15078_d_n9, assign15000_e15078_d_n10,) = {
            if (p.p52 != 0.0) {
                let assign15000_e15042: f64 = (locals.var_fn169_calc_iq__vgsin + locals.var_fn169_calc_iq__vgdin);
                let assign15000_e15045: f64 = (locals.var_fn169_calc_iq__vgsin - locals.var_fn169_calc_iq__vgdin);
                let assign15000_e15048: f64 = (0.001 / p.p53);
                let assign15000_e15051: f64 = (locals.var_fn169_calc_iq__vgsin - locals.var_fn169_calc_iq__vgdin);
                let assign15000_e15052: f64 = (assign15000_e15048 * assign15000_e15051);
                let assign15000_e15053: f64 = (assign15000_e15052).tanh();
                let assign15000_e15054: f64 = (assign15000_e15045 * assign15000_e15053);
                let assign15000_e15055: f64 = (assign15000_e15042 + assign15000_e15054);
                let assign15000_e15056: f64 = (0.5 * assign15000_e15055);
                (assign15000_e15056, (0.5 * ((locals.var_fn169_calc_iq__vgsin_dn2 + locals.var_fn169_calc_iq__vgdin_dn2) + (((locals.var_fn169_calc_iq__vgsin_dn2 - locals.var_fn169_calc_iq__vgdin_dn2) * assign15000_e15053) + (assign15000_e15045 * ((assign15000_e15048 * (locals.var_fn169_calc_iq__vgsin_dn2 - locals.var_fn169_calc_iq__vgdin_dn2)) / ((assign15000_e15052).cosh() * (assign15000_e15052).cosh())))))), (0.5 * ((locals.var_fn169_calc_iq__vgsin_dn7 + locals.var_fn169_calc_iq__vgdin_dn7) + (((locals.var_fn169_calc_iq__vgsin_dn7 - locals.var_fn169_calc_iq__vgdin_dn7) * assign15000_e15053) + (assign15000_e15045 * ((assign15000_e15048 * (locals.var_fn169_calc_iq__vgsin_dn7 - locals.var_fn169_calc_iq__vgdin_dn7)) / ((assign15000_e15052).cosh() * (assign15000_e15052).cosh())))))), (0.5 * (locals.var_fn169_calc_iq__vgdin_dn9 + (((-locals.var_fn169_calc_iq__vgdin_dn9) * assign15000_e15053) + (assign15000_e15045 * ((assign15000_e15048 * (-locals.var_fn169_calc_iq__vgdin_dn9)) / ((assign15000_e15052).cosh() * (assign15000_e15052).cosh())))))), (0.5 * ((locals.var_fn169_calc_iq__vgsin_dn10 + locals.var_fn169_calc_iq__vgdin_dn10) + (((locals.var_fn169_calc_iq__vgsin_dn10 - locals.var_fn169_calc_iq__vgdin_dn10) * assign15000_e15053) + (assign15000_e15045 * ((assign15000_e15048 * (locals.var_fn169_calc_iq__vgsin_dn10 - locals.var_fn169_calc_iq__vgdin_dn10)) / ((assign15000_e15052).cosh() * (assign15000_e15052).cosh())))))),)
            } else {
                let (assign15000_e15077, assign15000_e15077_d_n2, assign15000_e15077_d_n7, assign15000_e15077_d_n9, assign15000_e15077_d_n10,) = {
                    if (p.p52 == 0.0) {
                        let assign15000_e15063: f64 = (locals.var_fn169_calc_iq__vgsin + locals.var_fn169_calc_iq__vgdin);
                        let assign15000_e15066: f64 = (locals.var_fn169_calc_iq__vgsin - locals.var_fn169_calc_iq__vgdin);
                        let assign15000_e15069: f64 = (locals.var_fn169_calc_iq__vgsin - locals.var_fn169_calc_iq__vgdin);
                        let assign15000_e15070: f64 = (assign15000_e15066 * assign15000_e15069);
                        let assign15000_e15072: f64 = (assign15000_e15070 + p.p53);
                        let assign15000_e15073: f64 = (assign15000_e15072).sqrt();
                        let assign15000_e15074: f64 = (assign15000_e15063 + assign15000_e15073);
                        let assign15000_e15075: f64 = (0.5 * assign15000_e15074);
                        (assign15000_e15075, (0.5 * ((locals.var_fn169_calc_iq__vgsin_dn2 + locals.var_fn169_calc_iq__vgdin_dn2) + ((((locals.var_fn169_calc_iq__vgsin_dn2 - locals.var_fn169_calc_iq__vgdin_dn2) * assign15000_e15069) + (assign15000_e15066 * (locals.var_fn169_calc_iq__vgsin_dn2 - locals.var_fn169_calc_iq__vgdin_dn2))) / (2.0 * assign15000_e15073)))), (0.5 * ((locals.var_fn169_calc_iq__vgsin_dn7 + locals.var_fn169_calc_iq__vgdin_dn7) + ((((locals.var_fn169_calc_iq__vgsin_dn7 - locals.var_fn169_calc_iq__vgdin_dn7) * assign15000_e15069) + (assign15000_e15066 * (locals.var_fn169_calc_iq__vgsin_dn7 - locals.var_fn169_calc_iq__vgdin_dn7))) / (2.0 * assign15000_e15073)))), (0.5 * (locals.var_fn169_calc_iq__vgdin_dn9 + ((((-locals.var_fn169_calc_iq__vgdin_dn9) * assign15000_e15069) + (assign15000_e15066 * (-locals.var_fn169_calc_iq__vgdin_dn9))) / (2.0 * assign15000_e15073)))), (0.5 * ((locals.var_fn169_calc_iq__vgsin_dn10 + locals.var_fn169_calc_iq__vgdin_dn10) + ((((locals.var_fn169_calc_iq__vgsin_dn10 - locals.var_fn169_calc_iq__vgdin_dn10) * assign15000_e15069) + (assign15000_e15066 * (locals.var_fn169_calc_iq__vgsin_dn10 - locals.var_fn169_calc_iq__vgdin_dn10))) / (2.0 * assign15000_e15073)))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign15000_e15077, assign15000_e15077_d_n2, assign15000_e15077_d_n7, assign15000_e15077_d_n9, assign15000_e15077_d_n10,)
            }
        };
        let assign15000_e15082: f64 = (p.p51 * 0.1);
        let assign15000_e15084: f64 = (assign15000_e15082 * locals.var_fn169_calc_iq__alpha_phit);
        let assign15000_e15086: f64 = (assign15000_e15084 * locals.var_fn169_calc_iq__ff0);
        let assign15000_e15087: f64 = (locals.var_fn169_calc_iq__vtof - assign15000_e15086);
        let assign15000_e15088: f64 = (assign15000_e15078 - assign15000_e15087);
        let assign15000_e15090: f64 = (assign15000_e15088 / locals.var_fn169_calc_iq__two_n_phit0);
        (assign15000_e15090, ((assign15000_e15078_d_n2 - (-(assign15000_e15084 * locals.var_fn169_calc_iq__ff0_dn2))) / locals.var_fn169_calc_iq__two_n_phit0), ((((-(locals.var_fn169_calc_iq__vtof_dn4 - (((assign15000_e15082 * locals.var_fn169_calc_iq__alpha_phit_dn4) * locals.var_fn169_calc_iq__ff0) + (assign15000_e15084 * locals.var_fn169_calc_iq__ff0_dn4)))) * locals.var_fn169_calc_iq__two_n_phit0) - (assign15000_e15088 * locals.var_fn169_calc_iq__two_n_phit0_dn4)) / (locals.var_fn169_calc_iq__two_n_phit0 * locals.var_fn169_calc_iq__two_n_phit0)), ((assign15000_e15078_d_n7 - (-(assign15000_e15084 * locals.var_fn169_calc_iq__ff0_dn7))) / locals.var_fn169_calc_iq__two_n_phit0), ((assign15000_e15078_d_n9 - (-(assign15000_e15084 * locals.var_fn169_calc_iq__ff0_dn9))) / locals.var_fn169_calc_iq__two_n_phit0), ((assign15000_e15078_d_n10 - (-(assign15000_e15084 * locals.var_fn169_calc_iq__ff0_dn10))) / locals.var_fn169_calc_iq__two_n_phit0),)
    } else {
        (locals.var_fn169_calc_iq__eta0, locals.var_fn169_calc_iq__eta0_dn2, locals.var_fn169_calc_iq__eta0_dn4, locals.var_fn169_calc_iq__eta0_dn7, locals.var_fn169_calc_iq__eta0_dn9, locals.var_fn169_calc_iq__eta0_dn10,)
    }
};
        locals.var_fn169_calc_iq__eta0 = assign15000_e15092;
        locals.var_fn169_calc_iq__eta0_dn2 = assign15000_e15092_d_n2;
        locals.var_fn169_calc_iq__eta0_dn4 = assign15000_e15092_d_n4;
        locals.var_fn169_calc_iq__eta0_dn7 = assign15000_e15092_d_n7;
        locals.var_fn169_calc_iq__eta0_dn9 = assign15000_e15092_d_n9;
        locals.var_fn169_calc_iq__eta0_dn10 = assign15000_e15092_d_n10;

        let assign15010_e15095: f64 = if locals.var_fn169_calc_iq__eta0 > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard185 = assign15010_e15095;

        let (assign15020_e15103, assign15020_e15103_d_n2, assign15020_e15103_d_n4, assign15020_e15103_d_n7, assign15020_e15103_d_n9, assign15020_e15103_d_n10,) = {
    if ((locals.var_guard168 != 0.0) && (locals.var_guard185 != 0.0)) {
        let assign15020_e15101: f64 = (locals.var_fn169_calc_iq__qref0 * locals.var_fn169_calc_iq__eta0);
        (assign15020_e15101, (locals.var_fn169_calc_iq__qref0 * locals.var_fn169_calc_iq__eta0_dn2), ((locals.var_fn169_calc_iq__qref0_dn4 * locals.var_fn169_calc_iq__eta0) + (locals.var_fn169_calc_iq__qref0 * locals.var_fn169_calc_iq__eta0_dn4)), (locals.var_fn169_calc_iq__qref0 * locals.var_fn169_calc_iq__eta0_dn7), (locals.var_fn169_calc_iq__qref0 * locals.var_fn169_calc_iq__eta0_dn9), (locals.var_fn169_calc_iq__qref0 * locals.var_fn169_calc_iq__eta0_dn10),)
    } else {
        (locals.var_fn169_calc_iq__qinvv0, locals.var_fn169_calc_iq__qinvv0_dn2, locals.var_fn169_calc_iq__qinvv0_dn4, locals.var_fn169_calc_iq__qinvv0_dn7, locals.var_fn169_calc_iq__qinvv0_dn9, locals.var_fn169_calc_iq__qinvv0_dn10,)
    }
};
        locals.var_fn169_calc_iq__qinvv0 = assign15020_e15103;
        locals.var_fn169_calc_iq__qinvv0_dn2 = assign15020_e15103_d_n2;
        locals.var_fn169_calc_iq__qinvv0_dn4 = assign15020_e15103_d_n4;
        locals.var_fn169_calc_iq__qinvv0_dn7 = assign15020_e15103_d_n7;
        locals.var_fn169_calc_iq__qinvv0_dn9 = assign15020_e15103_d_n9;
        locals.var_fn169_calc_iq__qinvv0_dn10 = assign15020_e15103_d_n10;

        let assign15030_e15106: f64 = (-50.0);
        let assign15030_e15107: f64 = if locals.var_fn169_calc_iq__eta0 < assign15030_e15106 { 1.0 } else { 0.0 };
        locals.var_guard186 = assign15030_e15107;

        let (assign15040_e15119, assign15040_e15119_d_n2, assign15040_e15119_d_n4, assign15040_e15119_d_n7, assign15040_e15119_d_n9, assign15040_e15119_d_n10,) = {
    if (((locals.var_guard168 != 0.0) && (locals.var_guard185 == 0.0)) && (locals.var_guard186 != 0.0)) {
        let assign15040_e15116: f64 = (locals.var_fn169_calc_iq__eta0).exp();
        let assign15040_e15117: f64 = (locals.var_fn169_calc_iq__qref0 * assign15040_e15116);
        (assign15040_e15117, (locals.var_fn169_calc_iq__qref0 * (assign15040_e15116 * locals.var_fn169_calc_iq__eta0_dn2)), ((locals.var_fn169_calc_iq__qref0_dn4 * assign15040_e15116) + (locals.var_fn169_calc_iq__qref0 * (assign15040_e15116 * locals.var_fn169_calc_iq__eta0_dn4))), (locals.var_fn169_calc_iq__qref0 * (assign15040_e15116 * locals.var_fn169_calc_iq__eta0_dn7)), (locals.var_fn169_calc_iq__qref0 * (assign15040_e15116 * locals.var_fn169_calc_iq__eta0_dn9)), (locals.var_fn169_calc_iq__qref0 * (assign15040_e15116 * locals.var_fn169_calc_iq__eta0_dn10)),)
    } else {
        (locals.var_fn169_calc_iq__qinvv0, locals.var_fn169_calc_iq__qinvv0_dn2, locals.var_fn169_calc_iq__qinvv0_dn4, locals.var_fn169_calc_iq__qinvv0_dn7, locals.var_fn169_calc_iq__qinvv0_dn9, locals.var_fn169_calc_iq__qinvv0_dn10,)
    }
};
        locals.var_fn169_calc_iq__qinvv0 = assign15040_e15119;
        locals.var_fn169_calc_iq__qinvv0_dn2 = assign15040_e15119_d_n2;
        locals.var_fn169_calc_iq__qinvv0_dn4 = assign15040_e15119_d_n4;
        locals.var_fn169_calc_iq__qinvv0_dn7 = assign15040_e15119_d_n7;
        locals.var_fn169_calc_iq__qinvv0_dn9 = assign15040_e15119_d_n9;
        locals.var_fn169_calc_iq__qinvv0_dn10 = assign15040_e15119_d_n10;

        let (assign15050_e15135, assign15050_e15135_d_n2, assign15050_e15135_d_n4, assign15050_e15135_d_n7, assign15050_e15135_d_n9, assign15050_e15135_d_n10,) = {
    if (((locals.var_guard168 != 0.0) && (locals.var_guard185 == 0.0)) && (locals.var_guard186 == 0.0)) {
        let assign15050_e15130: f64 = (locals.var_fn169_calc_iq__eta0).exp();
        let assign15050_e15131: f64 = (1.0 + assign15050_e15130);
        let assign15050_e15132: f64 = (assign15050_e15131).ln();
        let assign15050_e15133: f64 = (locals.var_fn169_calc_iq__qref0 * assign15050_e15132);
        (assign15050_e15133, (locals.var_fn169_calc_iq__qref0 * ((assign15050_e15130 * locals.var_fn169_calc_iq__eta0_dn2) / assign15050_e15131)), ((locals.var_fn169_calc_iq__qref0_dn4 * assign15050_e15132) + (locals.var_fn169_calc_iq__qref0 * ((assign15050_e15130 * locals.var_fn169_calc_iq__eta0_dn4) / assign15050_e15131))), (locals.var_fn169_calc_iq__qref0 * ((assign15050_e15130 * locals.var_fn169_calc_iq__eta0_dn7) / assign15050_e15131)), (locals.var_fn169_calc_iq__qref0 * ((assign15050_e15130 * locals.var_fn169_calc_iq__eta0_dn9) / assign15050_e15131)), (locals.var_fn169_calc_iq__qref0 * ((assign15050_e15130 * locals.var_fn169_calc_iq__eta0_dn10) / assign15050_e15131)),)
    } else {
        (locals.var_fn169_calc_iq__qinvv0, locals.var_fn169_calc_iq__qinvv0_dn2, locals.var_fn169_calc_iq__qinvv0_dn4, locals.var_fn169_calc_iq__qinvv0_dn7, locals.var_fn169_calc_iq__qinvv0_dn9, locals.var_fn169_calc_iq__qinvv0_dn10,)
    }
};
        locals.var_fn169_calc_iq__qinvv0 = assign15050_e15135;
        locals.var_fn169_calc_iq__qinvv0_dn2 = assign15050_e15135_d_n2;
        locals.var_fn169_calc_iq__qinvv0_dn4 = assign15050_e15135_d_n4;
        locals.var_fn169_calc_iq__qinvv0_dn7 = assign15050_e15135_d_n7;
        locals.var_fn169_calc_iq__qinvv0_dn9 = assign15050_e15135_d_n9;
        locals.var_fn169_calc_iq__qinvv0_dn10 = assign15050_e15135_d_n10;

        let (assign15060_e15141, assign15060_e15141_d_n4,) = {
    if (locals.var_guard168 != 0.0) {
        let assign15060_e15139: f64 = (locals.var_fn169_calc_iq__mu0 / locals.var_fn169_calc_iq__tfacmobin);
        (assign15060_e15139, (-((locals.var_fn169_calc_iq__mu0 * locals.var_fn169_calc_iq__tfacmobin_dn4) / (locals.var_fn169_calc_iq__tfacmobin * locals.var_fn169_calc_iq__tfacmobin))),)
    } else {
        (locals.var_fn169_calc_iq__muf0, locals.var_fn169_calc_iq__muf0_dn4,)
    }
};
        locals.var_fn169_calc_iq__muf0 = assign15060_e15141;
        locals.var_fn169_calc_iq__muf0_dn4 = assign15060_e15141_d_n4;

        let (assign15070_e15157, assign15070_e15157_d_n4,) = {
    if (locals.var_guard168 != 0.0) {
        let assign15070_e15147: f64 = (locals.var_fn169_calc_iq__vzeta * locals.var_fn169_calc_iq__tnomin);
        let assign15070_e15148: f64 = (1.0 + assign15070_e15147);
        let assign15070_e15152: f64 = (locals.var_fn169_calc_iq__vzeta * locals.var_fn169_calc_iq__tambin);
        let assign15070_e15153: f64 = (1.0 + assign15070_e15152);
        let assign15070_e15154: f64 = (assign15070_e15148 / assign15070_e15153);
        let assign15070_e15155: f64 = (locals.var_fn169_calc_iq__vel0 * assign15070_e15154);
        (assign15070_e15155, (locals.var_fn169_calc_iq__vel0 * (-((assign15070_e15148 * (locals.var_fn169_calc_iq__vzeta * locals.var_fn169_calc_iq__tambin_dn4)) / (assign15070_e15153 * assign15070_e15153)))),)
    } else {
        (locals.var_fn169_calc_iq__vx0, locals.var_fn169_calc_iq__vx0_dn4,)
    }
};
        locals.var_fn169_calc_iq__vx0 = assign15070_e15157;
        locals.var_fn169_calc_iq__vx0_dn4 = assign15070_e15157_d_n4;

        let (assign15080_e15165, assign15080_e15165_d_n4,) = {
    if (locals.var_guard168 != 0.0) {
        let assign15080_e15161: f64 = (locals.var_fn169_calc_iq__vx0 * locals.var_fn169_calc_iq__lin);
        let assign15080_e15163: f64 = (assign15080_e15161 / locals.var_fn169_calc_iq__muf0);
        (assign15080_e15163, ((((locals.var_fn169_calc_iq__vx0_dn4 * locals.var_fn169_calc_iq__lin) * locals.var_fn169_calc_iq__muf0) - (assign15080_e15161 * locals.var_fn169_calc_iq__muf0_dn4)) / (locals.var_fn169_calc_iq__muf0 * locals.var_fn169_calc_iq__muf0)),)
    } else {
        (locals.var_fn169_calc_iq__vdsats0, locals.var_fn169_calc_iq__vdsats0_dn4,)
    }
};
        locals.var_fn169_calc_iq__vdsats0 = assign15080_e15165;
        locals.var_fn169_calc_iq__vdsats0_dn4 = assign15080_e15165_d_n4;

        let (assign15090_e15182, assign15090_e15182_d_n2, assign15090_e15182_d_n4, assign15090_e15182_d_n7, assign15090_e15182_d_n9, assign15090_e15182_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        let assign15090_e15171: f64 = (2.0 * locals.var_fn169_calc_iq__qinvv0);
        let assign15090_e15173: f64 = (assign15090_e15171 / locals.var_fn169_calc_iq__cgin);
        let assign15090_e15175: f64 = (assign15090_e15173 / locals.var_fn169_calc_iq__vdsats0);
        let assign15090_e15176: f64 = (1.0 + assign15090_e15175);
        let assign15090_e15177: f64 = (assign15090_e15176).sqrt();
        let assign15090_e15178: f64 = (locals.var_fn169_calc_iq__vdsats0 * assign15090_e15177);
        let assign15090_e15180: f64 = (assign15090_e15178 - locals.var_fn169_calc_iq__vdsats0);
        (assign15090_e15180, (locals.var_fn169_calc_iq__vdsats0 * ((((2.0 * locals.var_fn169_calc_iq__qinvv0_dn2) / locals.var_fn169_calc_iq__cgin) / locals.var_fn169_calc_iq__vdsats0) / (2.0 * assign15090_e15177))), (((locals.var_fn169_calc_iq__vdsats0_dn4 * assign15090_e15177) + (locals.var_fn169_calc_iq__vdsats0 * ((((((((2.0 * locals.var_fn169_calc_iq__qinvv0_dn4) * locals.var_fn169_calc_iq__cgin) - (assign15090_e15171 * locals.var_fn169_calc_iq__cgin_dn4)) / (locals.var_fn169_calc_iq__cgin * locals.var_fn169_calc_iq__cgin)) * locals.var_fn169_calc_iq__vdsats0) - (assign15090_e15173 * locals.var_fn169_calc_iq__vdsats0_dn4)) / (locals.var_fn169_calc_iq__vdsats0 * locals.var_fn169_calc_iq__vdsats0)) / (2.0 * assign15090_e15177)))) - locals.var_fn169_calc_iq__vdsats0_dn4), (locals.var_fn169_calc_iq__vdsats0 * ((((2.0 * locals.var_fn169_calc_iq__qinvv0_dn7) / locals.var_fn169_calc_iq__cgin) / locals.var_fn169_calc_iq__vdsats0) / (2.0 * assign15090_e15177))), (locals.var_fn169_calc_iq__vdsats0 * ((((2.0 * locals.var_fn169_calc_iq__qinvv0_dn9) / locals.var_fn169_calc_iq__cgin) / locals.var_fn169_calc_iq__vdsats0) / (2.0 * assign15090_e15177))), (locals.var_fn169_calc_iq__vdsats0 * ((((2.0 * locals.var_fn169_calc_iq__qinvv0_dn10) / locals.var_fn169_calc_iq__cgin) / locals.var_fn169_calc_iq__vdsats0) / (2.0 * assign15090_e15177))),)
    } else {
        (locals.var_fn169_calc_iq__vdsats10, locals.var_fn169_calc_iq__vdsats10_dn2, locals.var_fn169_calc_iq__vdsats10_dn4, locals.var_fn169_calc_iq__vdsats10_dn7, locals.var_fn169_calc_iq__vdsats10_dn9, locals.var_fn169_calc_iq__vdsats10_dn10,)
    }
};
        locals.var_fn169_calc_iq__vdsats10 = assign15090_e15182;
        locals.var_fn169_calc_iq__vdsats10_dn2 = assign15090_e15182_d_n2;
        locals.var_fn169_calc_iq__vdsats10_dn4 = assign15090_e15182_d_n4;
        locals.var_fn169_calc_iq__vdsats10_dn7 = assign15090_e15182_d_n7;
        locals.var_fn169_calc_iq__vdsats10_dn9 = assign15090_e15182_d_n9;
        locals.var_fn169_calc_iq__vdsats10_dn10 = assign15090_e15182_d_n10;

        let (assign15100_e15194, assign15100_e15194_d_n2, assign15100_e15194_d_n4, assign15100_e15194_d_n7, assign15100_e15194_d_n9, assign15100_e15194_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        let assign15100_e15187: f64 = (1.0 - locals.var_fn169_calc_iq__ff0);
        let assign15100_e15188: f64 = (locals.var_fn169_calc_iq__vdsats10 * assign15100_e15187);
        let assign15100_e15191: f64 = (locals.var_fn169_calc_iq__two_n_phit0 * locals.var_fn169_calc_iq__ff0);
        let assign15100_e15192: f64 = (assign15100_e15188 + assign15100_e15191);
        (assign15100_e15192, (((locals.var_fn169_calc_iq__vdsats10_dn2 * assign15100_e15187) + (locals.var_fn169_calc_iq__vdsats10 * (-locals.var_fn169_calc_iq__ff0_dn2))) + (locals.var_fn169_calc_iq__two_n_phit0 * locals.var_fn169_calc_iq__ff0_dn2)), (((locals.var_fn169_calc_iq__vdsats10_dn4 * assign15100_e15187) + (locals.var_fn169_calc_iq__vdsats10 * (-locals.var_fn169_calc_iq__ff0_dn4))) + ((locals.var_fn169_calc_iq__two_n_phit0_dn4 * locals.var_fn169_calc_iq__ff0) + (locals.var_fn169_calc_iq__two_n_phit0 * locals.var_fn169_calc_iq__ff0_dn4))), (((locals.var_fn169_calc_iq__vdsats10_dn7 * assign15100_e15187) + (locals.var_fn169_calc_iq__vdsats10 * (-locals.var_fn169_calc_iq__ff0_dn7))) + (locals.var_fn169_calc_iq__two_n_phit0 * locals.var_fn169_calc_iq__ff0_dn7)), (((locals.var_fn169_calc_iq__vdsats10_dn9 * assign15100_e15187) + (locals.var_fn169_calc_iq__vdsats10 * (-locals.var_fn169_calc_iq__ff0_dn9))) + (locals.var_fn169_calc_iq__two_n_phit0 * locals.var_fn169_calc_iq__ff0_dn9)), (((locals.var_fn169_calc_iq__vdsats10_dn10 * assign15100_e15187) + (locals.var_fn169_calc_iq__vdsats10 * (-locals.var_fn169_calc_iq__ff0_dn10))) + (locals.var_fn169_calc_iq__two_n_phit0 * locals.var_fn169_calc_iq__ff0_dn10)),)
    } else {
        (locals.var_fn169_calc_iq__vdsat10, locals.var_fn169_calc_iq__vdsat10_dn2, locals.var_fn169_calc_iq__vdsat10_dn4, locals.var_fn169_calc_iq__vdsat10_dn7, locals.var_fn169_calc_iq__vdsat10_dn9, locals.var_fn169_calc_iq__vdsat10_dn10,)
    }
};
        locals.var_fn169_calc_iq__vdsat10 = assign15100_e15194;
        locals.var_fn169_calc_iq__vdsat10_dn2 = assign15100_e15194_d_n2;
        locals.var_fn169_calc_iq__vdsat10_dn4 = assign15100_e15194_d_n4;
        locals.var_fn169_calc_iq__vdsat10_dn7 = assign15100_e15194_d_n7;
        locals.var_fn169_calc_iq__vdsat10_dn9 = assign15100_e15194_d_n9;
        locals.var_fn169_calc_iq__vdsat10_dn10 = assign15100_e15194_d_n10;

        let (assign15110_e15263, assign15110_e15263_d_n2, assign15110_e15263_d_n4, assign15110_e15263_d_n7, assign15110_e15263_d_n9, assign15110_e15263_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        let (assign15110_e15253, assign15110_e15253_d_n2, assign15110_e15253_d_n4, assign15110_e15253_d_n7, assign15110_e15253_d_n9, assign15110_e15253_d_n10,) = {
            if (p.p52 != 0.0) {
                let assign15110_e15206: f64 = (locals.var_fn169_calc_iq__vdsin / locals.var_fn169_calc_iq__vdsat10);
                let assign15110_e15207: f64 = assign15110_e15206;
                let assign15110_e15211: f64 = (locals.var_fn169_calc_iq__vdsin / locals.var_fn169_calc_iq__vdsat10);
                let assign15110_e15212: f64 = (-assign15110_e15211);
                let assign15110_e15215: f64 = (0.001 / p.p53);
                let assign15110_e15219: f64 = (locals.var_fn169_calc_iq__vdsin / locals.var_fn169_calc_iq__vdsat10);
                let assign15110_e15220: f64 = (-assign15110_e15219);
                let assign15110_e15221: f64 = (assign15110_e15215 * assign15110_e15220);
                let assign15110_e15222: f64 = (assign15110_e15221).tanh();
                let assign15110_e15223: f64 = (assign15110_e15212 * assign15110_e15222);
                let assign15110_e15224: f64 = (assign15110_e15207 + assign15110_e15223);
                let assign15110_e15225: f64 = (0.5 * assign15110_e15224);
                (assign15110_e15225, (0.5 * ((-((locals.var_fn169_calc_iq__vdsin * locals.var_fn169_calc_iq__vdsat10_dn2) / (locals.var_fn169_calc_iq__vdsat10 * locals.var_fn169_calc_iq__vdsat10))) + (((-(-((locals.var_fn169_calc_iq__vdsin * locals.var_fn169_calc_iq__vdsat10_dn2) / (locals.var_fn169_calc_iq__vdsat10 * locals.var_fn169_calc_iq__vdsat10)))) * assign15110_e15222) + (assign15110_e15212 * ((assign15110_e15215 * (-(-((locals.var_fn169_calc_iq__vdsin * locals.var_fn169_calc_iq__vdsat10_dn2) / (locals.var_fn169_calc_iq__vdsat10 * locals.var_fn169_calc_iq__vdsat10))))) / ((assign15110_e15221).cosh() * (assign15110_e15221).cosh())))))), (0.5 * ((-((locals.var_fn169_calc_iq__vdsin * locals.var_fn169_calc_iq__vdsat10_dn4) / (locals.var_fn169_calc_iq__vdsat10 * locals.var_fn169_calc_iq__vdsat10))) + (((-(-((locals.var_fn169_calc_iq__vdsin * locals.var_fn169_calc_iq__vdsat10_dn4) / (locals.var_fn169_calc_iq__vdsat10 * locals.var_fn169_calc_iq__vdsat10)))) * assign15110_e15222) + (assign15110_e15212 * ((assign15110_e15215 * (-(-((locals.var_fn169_calc_iq__vdsin * locals.var_fn169_calc_iq__vdsat10_dn4) / (locals.var_fn169_calc_iq__vdsat10 * locals.var_fn169_calc_iq__vdsat10))))) / ((assign15110_e15221).cosh() * (assign15110_e15221).cosh())))))), (0.5 * ((-((locals.var_fn169_calc_iq__vdsin * locals.var_fn169_calc_iq__vdsat10_dn7) / (locals.var_fn169_calc_iq__vdsat10 * locals.var_fn169_calc_iq__vdsat10))) + (((-(-((locals.var_fn169_calc_iq__vdsin * locals.var_fn169_calc_iq__vdsat10_dn7) / (locals.var_fn169_calc_iq__vdsat10 * locals.var_fn169_calc_iq__vdsat10)))) * assign15110_e15222) + (assign15110_e15212 * ((assign15110_e15215 * (-(-((locals.var_fn169_calc_iq__vdsin * locals.var_fn169_calc_iq__vdsat10_dn7) / (locals.var_fn169_calc_iq__vdsat10 * locals.var_fn169_calc_iq__vdsat10))))) / ((assign15110_e15221).cosh() * (assign15110_e15221).cosh())))))), (0.5 * ((((locals.var_fn169_calc_iq__vdsin_dn9 * locals.var_fn169_calc_iq__vdsat10) - (locals.var_fn169_calc_iq__vdsin * locals.var_fn169_calc_iq__vdsat10_dn9)) / (locals.var_fn169_calc_iq__vdsat10 * locals.var_fn169_calc_iq__vdsat10)) + (((-(((locals.var_fn169_calc_iq__vdsin_dn9 * locals.var_fn169_calc_iq__vdsat10) - (locals.var_fn169_calc_iq__vdsin * locals.var_fn169_calc_iq__vdsat10_dn9)) / (locals.var_fn169_calc_iq__vdsat10 * locals.var_fn169_calc_iq__vdsat10))) * assign15110_e15222) + (assign15110_e15212 * ((assign15110_e15215 * (-(((locals.var_fn169_calc_iq__vdsin_dn9 * locals.var_fn169_calc_iq__vdsat10) - (locals.var_fn169_calc_iq__vdsin * locals.var_fn169_calc_iq__vdsat10_dn9)) / (locals.var_fn169_calc_iq__vdsat10 * locals.var_fn169_calc_iq__vdsat10)))) / ((assign15110_e15221).cosh() * (assign15110_e15221).cosh())))))), (0.5 * ((((locals.var_fn169_calc_iq__vdsin_dn10 * locals.var_fn169_calc_iq__vdsat10) - (locals.var_fn169_calc_iq__vdsin * locals.var_fn169_calc_iq__vdsat10_dn10)) / (locals.var_fn169_calc_iq__vdsat10 * locals.var_fn169_calc_iq__vdsat10)) + (((-(((locals.var_fn169_calc_iq__vdsin_dn10 * locals.var_fn169_calc_iq__vdsat10) - (locals.var_fn169_calc_iq__vdsin * locals.var_fn169_calc_iq__vdsat10_dn10)) / (locals.var_fn169_calc_iq__vdsat10 * locals.var_fn169_calc_iq__vdsat10))) * assign15110_e15222) + (assign15110_e15212 * ((assign15110_e15215 * (-(((locals.var_fn169_calc_iq__vdsin_dn10 * locals.var_fn169_calc_iq__vdsat10) - (locals.var_fn169_calc_iq__vdsin * locals.var_fn169_calc_iq__vdsat10_dn10)) / (locals.var_fn169_calc_iq__vdsat10 * locals.var_fn169_calc_iq__vdsat10)))) / ((assign15110_e15221).cosh() * (assign15110_e15221).cosh())))))),)
            } else {
                let (assign15110_e15252, assign15110_e15252_d_n2, assign15110_e15252_d_n4, assign15110_e15252_d_n7, assign15110_e15252_d_n9, assign15110_e15252_d_n10,) = {
                    if (p.p52 == 0.0) {
                        let assign15110_e15233: f64 = (locals.var_fn169_calc_iq__vdsin / locals.var_fn169_calc_iq__vdsat10);
                        let assign15110_e15234: f64 = assign15110_e15233;
                        let assign15110_e15238: f64 = (locals.var_fn169_calc_iq__vdsin / locals.var_fn169_calc_iq__vdsat10);
                        let assign15110_e15239: f64 = (-assign15110_e15238);
                        let assign15110_e15243: f64 = (locals.var_fn169_calc_iq__vdsin / locals.var_fn169_calc_iq__vdsat10);
                        let assign15110_e15244: f64 = (-assign15110_e15243);
                        let assign15110_e15245: f64 = (assign15110_e15239 * assign15110_e15244);
                        let assign15110_e15247: f64 = (assign15110_e15245 + p.p53);
                        let assign15110_e15248: f64 = (assign15110_e15247).sqrt();
                        let assign15110_e15249: f64 = (assign15110_e15234 + assign15110_e15248);
                        let assign15110_e15250: f64 = (0.5 * assign15110_e15249);
                        (assign15110_e15250, (0.5 * ((-((locals.var_fn169_calc_iq__vdsin * locals.var_fn169_calc_iq__vdsat10_dn2) / (locals.var_fn169_calc_iq__vdsat10 * locals.var_fn169_calc_iq__vdsat10))) + ((((-(-((locals.var_fn169_calc_iq__vdsin * locals.var_fn169_calc_iq__vdsat10_dn2) / (locals.var_fn169_calc_iq__vdsat10 * locals.var_fn169_calc_iq__vdsat10)))) * assign15110_e15244) + (assign15110_e15239 * (-(-((locals.var_fn169_calc_iq__vdsin * locals.var_fn169_calc_iq__vdsat10_dn2) / (locals.var_fn169_calc_iq__vdsat10 * locals.var_fn169_calc_iq__vdsat10)))))) / (2.0 * assign15110_e15248)))), (0.5 * ((-((locals.var_fn169_calc_iq__vdsin * locals.var_fn169_calc_iq__vdsat10_dn4) / (locals.var_fn169_calc_iq__vdsat10 * locals.var_fn169_calc_iq__vdsat10))) + ((((-(-((locals.var_fn169_calc_iq__vdsin * locals.var_fn169_calc_iq__vdsat10_dn4) / (locals.var_fn169_calc_iq__vdsat10 * locals.var_fn169_calc_iq__vdsat10)))) * assign15110_e15244) + (assign15110_e15239 * (-(-((locals.var_fn169_calc_iq__vdsin * locals.var_fn169_calc_iq__vdsat10_dn4) / (locals.var_fn169_calc_iq__vdsat10 * locals.var_fn169_calc_iq__vdsat10)))))) / (2.0 * assign15110_e15248)))), (0.5 * ((-((locals.var_fn169_calc_iq__vdsin * locals.var_fn169_calc_iq__vdsat10_dn7) / (locals.var_fn169_calc_iq__vdsat10 * locals.var_fn169_calc_iq__vdsat10))) + ((((-(-((locals.var_fn169_calc_iq__vdsin * locals.var_fn169_calc_iq__vdsat10_dn7) / (locals.var_fn169_calc_iq__vdsat10 * locals.var_fn169_calc_iq__vdsat10)))) * assign15110_e15244) + (assign15110_e15239 * (-(-((locals.var_fn169_calc_iq__vdsin * locals.var_fn169_calc_iq__vdsat10_dn7) / (locals.var_fn169_calc_iq__vdsat10 * locals.var_fn169_calc_iq__vdsat10)))))) / (2.0 * assign15110_e15248)))), (0.5 * ((((locals.var_fn169_calc_iq__vdsin_dn9 * locals.var_fn169_calc_iq__vdsat10) - (locals.var_fn169_calc_iq__vdsin * locals.var_fn169_calc_iq__vdsat10_dn9)) / (locals.var_fn169_calc_iq__vdsat10 * locals.var_fn169_calc_iq__vdsat10)) + ((((-(((locals.var_fn169_calc_iq__vdsin_dn9 * locals.var_fn169_calc_iq__vdsat10) - (locals.var_fn169_calc_iq__vdsin * locals.var_fn169_calc_iq__vdsat10_dn9)) / (locals.var_fn169_calc_iq__vdsat10 * locals.var_fn169_calc_iq__vdsat10))) * assign15110_e15244) + (assign15110_e15239 * (-(((locals.var_fn169_calc_iq__vdsin_dn9 * locals.var_fn169_calc_iq__vdsat10) - (locals.var_fn169_calc_iq__vdsin * locals.var_fn169_calc_iq__vdsat10_dn9)) / (locals.var_fn169_calc_iq__vdsat10 * locals.var_fn169_calc_iq__vdsat10))))) / (2.0 * assign15110_e15248)))), (0.5 * ((((locals.var_fn169_calc_iq__vdsin_dn10 * locals.var_fn169_calc_iq__vdsat10) - (locals.var_fn169_calc_iq__vdsin * locals.var_fn169_calc_iq__vdsat10_dn10)) / (locals.var_fn169_calc_iq__vdsat10 * locals.var_fn169_calc_iq__vdsat10)) + ((((-(((locals.var_fn169_calc_iq__vdsin_dn10 * locals.var_fn169_calc_iq__vdsat10) - (locals.var_fn169_calc_iq__vdsin * locals.var_fn169_calc_iq__vdsat10_dn10)) / (locals.var_fn169_calc_iq__vdsat10 * locals.var_fn169_calc_iq__vdsat10))) * assign15110_e15244) + (assign15110_e15239 * (-(((locals.var_fn169_calc_iq__vdsin_dn10 * locals.var_fn169_calc_iq__vdsat10) - (locals.var_fn169_calc_iq__vdsin * locals.var_fn169_calc_iq__vdsat10_dn10)) / (locals.var_fn169_calc_iq__vdsat10 * locals.var_fn169_calc_iq__vdsat10))))) / (2.0 * assign15110_e15248)))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign15110_e15252, assign15110_e15252_d_n2, assign15110_e15252_d_n4, assign15110_e15252_d_n7, assign15110_e15252_d_n9, assign15110_e15252_d_n10,)
            }
        };
        let assign15110_e15255: f64 = (assign15110_e15253).powf(locals.var_fn169_calc_iq__beta);
        let assign15110_e15256: f64 = (1.0 + assign15110_e15255);
        let assign15110_e15259: f64 = (1.0 / locals.var_fn169_calc_iq__beta);
        let assign15110_e15260: f64 = (assign15110_e15256).powf(assign15110_e15259);
        let assign15110_e15261: f64 = (1.0 / assign15110_e15260);
        (assign15110_e15261, (-(if 0.0 == 0.0 && ((assign15110_e15259) as f64).is_finite() && ((assign15110_e15259) as f64).fract() == 0.0 { if assign15110_e15259 == 0.0 { 0.0 } else { (assign15110_e15259 * ((assign15110_e15256).powf(assign15110_e15259 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn169_calc_iq__beta) as f64).is_finite() && ((locals.var_fn169_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn169_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn169_calc_iq__beta * ((assign15110_e15253).powf(locals.var_fn169_calc_iq__beta - 1.0) * assign15110_e15253_d_n2)) } } else { (assign15110_e15255 * (locals.var_fn169_calc_iq__beta * (assign15110_e15253_d_n2 / assign15110_e15253))) })) } } else { (assign15110_e15260 * (assign15110_e15259 * (if 0.0 == 0.0 && ((locals.var_fn169_calc_iq__beta) as f64).is_finite() && ((locals.var_fn169_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn169_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn169_calc_iq__beta * ((assign15110_e15253).powf(locals.var_fn169_calc_iq__beta - 1.0) * assign15110_e15253_d_n2)) } } else { (assign15110_e15255 * (locals.var_fn169_calc_iq__beta * (assign15110_e15253_d_n2 / assign15110_e15253))) } / assign15110_e15256))) } / (assign15110_e15260 * assign15110_e15260))), (-(if 0.0 == 0.0 && ((assign15110_e15259) as f64).is_finite() && ((assign15110_e15259) as f64).fract() == 0.0 { if assign15110_e15259 == 0.0 { 0.0 } else { (assign15110_e15259 * ((assign15110_e15256).powf(assign15110_e15259 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn169_calc_iq__beta) as f64).is_finite() && ((locals.var_fn169_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn169_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn169_calc_iq__beta * ((assign15110_e15253).powf(locals.var_fn169_calc_iq__beta - 1.0) * assign15110_e15253_d_n4)) } } else { (assign15110_e15255 * (locals.var_fn169_calc_iq__beta * (assign15110_e15253_d_n4 / assign15110_e15253))) })) } } else { (assign15110_e15260 * (assign15110_e15259 * (if 0.0 == 0.0 && ((locals.var_fn169_calc_iq__beta) as f64).is_finite() && ((locals.var_fn169_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn169_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn169_calc_iq__beta * ((assign15110_e15253).powf(locals.var_fn169_calc_iq__beta - 1.0) * assign15110_e15253_d_n4)) } } else { (assign15110_e15255 * (locals.var_fn169_calc_iq__beta * (assign15110_e15253_d_n4 / assign15110_e15253))) } / assign15110_e15256))) } / (assign15110_e15260 * assign15110_e15260))), (-(if 0.0 == 0.0 && ((assign15110_e15259) as f64).is_finite() && ((assign15110_e15259) as f64).fract() == 0.0 { if assign15110_e15259 == 0.0 { 0.0 } else { (assign15110_e15259 * ((assign15110_e15256).powf(assign15110_e15259 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn169_calc_iq__beta) as f64).is_finite() && ((locals.var_fn169_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn169_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn169_calc_iq__beta * ((assign15110_e15253).powf(locals.var_fn169_calc_iq__beta - 1.0) * assign15110_e15253_d_n7)) } } else { (assign15110_e15255 * (locals.var_fn169_calc_iq__beta * (assign15110_e15253_d_n7 / assign15110_e15253))) })) } } else { (assign15110_e15260 * (assign15110_e15259 * (if 0.0 == 0.0 && ((locals.var_fn169_calc_iq__beta) as f64).is_finite() && ((locals.var_fn169_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn169_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn169_calc_iq__beta * ((assign15110_e15253).powf(locals.var_fn169_calc_iq__beta - 1.0) * assign15110_e15253_d_n7)) } } else { (assign15110_e15255 * (locals.var_fn169_calc_iq__beta * (assign15110_e15253_d_n7 / assign15110_e15253))) } / assign15110_e15256))) } / (assign15110_e15260 * assign15110_e15260))), (-(if 0.0 == 0.0 && ((assign15110_e15259) as f64).is_finite() && ((assign15110_e15259) as f64).fract() == 0.0 { if assign15110_e15259 == 0.0 { 0.0 } else { (assign15110_e15259 * ((assign15110_e15256).powf(assign15110_e15259 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn169_calc_iq__beta) as f64).is_finite() && ((locals.var_fn169_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn169_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn169_calc_iq__beta * ((assign15110_e15253).powf(locals.var_fn169_calc_iq__beta - 1.0) * assign15110_e15253_d_n9)) } } else { (assign15110_e15255 * (locals.var_fn169_calc_iq__beta * (assign15110_e15253_d_n9 / assign15110_e15253))) })) } } else { (assign15110_e15260 * (assign15110_e15259 * (if 0.0 == 0.0 && ((locals.var_fn169_calc_iq__beta) as f64).is_finite() && ((locals.var_fn169_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn169_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn169_calc_iq__beta * ((assign15110_e15253).powf(locals.var_fn169_calc_iq__beta - 1.0) * assign15110_e15253_d_n9)) } } else { (assign15110_e15255 * (locals.var_fn169_calc_iq__beta * (assign15110_e15253_d_n9 / assign15110_e15253))) } / assign15110_e15256))) } / (assign15110_e15260 * assign15110_e15260))), (-(if 0.0 == 0.0 && ((assign15110_e15259) as f64).is_finite() && ((assign15110_e15259) as f64).fract() == 0.0 { if assign15110_e15259 == 0.0 { 0.0 } else { (assign15110_e15259 * ((assign15110_e15256).powf(assign15110_e15259 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn169_calc_iq__beta) as f64).is_finite() && ((locals.var_fn169_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn169_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn169_calc_iq__beta * ((assign15110_e15253).powf(locals.var_fn169_calc_iq__beta - 1.0) * assign15110_e15253_d_n10)) } } else { (assign15110_e15255 * (locals.var_fn169_calc_iq__beta * (assign15110_e15253_d_n10 / assign15110_e15253))) })) } } else { (assign15110_e15260 * (assign15110_e15259 * (if 0.0 == 0.0 && ((locals.var_fn169_calc_iq__beta) as f64).is_finite() && ((locals.var_fn169_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn169_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn169_calc_iq__beta * ((assign15110_e15253).powf(locals.var_fn169_calc_iq__beta - 1.0) * assign15110_e15253_d_n10)) } } else { (assign15110_e15255 * (locals.var_fn169_calc_iq__beta * (assign15110_e15253_d_n10 / assign15110_e15253))) } / assign15110_e15256))) } / (assign15110_e15260 * assign15110_e15260))),)
    } else {
        (locals.var_fn169_calc_iq__fsd0, locals.var_fn169_calc_iq__fsd0_dn2, locals.var_fn169_calc_iq__fsd0_dn4, locals.var_fn169_calc_iq__fsd0_dn7, locals.var_fn169_calc_iq__fsd0_dn9, locals.var_fn169_calc_iq__fsd0_dn10,)
    }
};
        locals.var_fn169_calc_iq__fsd0 = assign15110_e15263;
        locals.var_fn169_calc_iq__fsd0_dn2 = assign15110_e15263_d_n2;
        locals.var_fn169_calc_iq__fsd0_dn4 = assign15110_e15263_d_n4;
        locals.var_fn169_calc_iq__fsd0_dn7 = assign15110_e15263_d_n7;
        locals.var_fn169_calc_iq__fsd0_dn9 = assign15110_e15263_d_n9;
        locals.var_fn169_calc_iq__fsd0_dn10 = assign15110_e15263_d_n10;

        let (assign15120_e15269, assign15120_e15269_d_n2, assign15120_e15269_d_n4, assign15120_e15269_d_n7, assign15120_e15269_d_n9, assign15120_e15269_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        let assign15120_e15267: f64 = (locals.var_fn169_calc_iq__vdsin * locals.var_fn169_calc_iq__fsd0);
        (assign15120_e15267, (locals.var_fn169_calc_iq__vdsin * locals.var_fn169_calc_iq__fsd0_dn2), (locals.var_fn169_calc_iq__vdsin * locals.var_fn169_calc_iq__fsd0_dn4), (locals.var_fn169_calc_iq__vdsin * locals.var_fn169_calc_iq__fsd0_dn7), ((locals.var_fn169_calc_iq__vdsin_dn9 * locals.var_fn169_calc_iq__fsd0) + (locals.var_fn169_calc_iq__vdsin * locals.var_fn169_calc_iq__fsd0_dn9)), ((locals.var_fn169_calc_iq__vdsin_dn10 * locals.var_fn169_calc_iq__fsd0) + (locals.var_fn169_calc_iq__vdsin * locals.var_fn169_calc_iq__fsd0_dn10)),)
    } else {
        (locals.var_fn169_calc_iq__vdx0, locals.var_fn169_calc_iq__vdx0_dn2, locals.var_fn169_calc_iq__vdx0_dn4, locals.var_fn169_calc_iq__vdx0_dn7, locals.var_fn169_calc_iq__vdx0_dn9, locals.var_fn169_calc_iq__vdx0_dn10,)
    }
};
        locals.var_fn169_calc_iq__vdx0 = assign15120_e15269;
        locals.var_fn169_calc_iq__vdx0_dn2 = assign15120_e15269_d_n2;
        locals.var_fn169_calc_iq__vdx0_dn4 = assign15120_e15269_d_n4;
        locals.var_fn169_calc_iq__vdx0_dn7 = assign15120_e15269_d_n7;
        locals.var_fn169_calc_iq__vdx0_dn9 = assign15120_e15269_d_n9;
        locals.var_fn169_calc_iq__vdx0_dn10 = assign15120_e15269_d_n10;

        let (assign15130_e15344, assign15130_e15344_d_n2, assign15130_e15344_d_n4, assign15130_e15344_d_n7, assign15130_e15344_d_n9, assign15130_e15344_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        let (assign15130_e15334, assign15130_e15334_d_n2, assign15130_e15334_d_n4, assign15130_e15334_d_n7, assign15130_e15334_d_n9, assign15130_e15334_d_n10,) = {
            if (p.p52 != 0.0) {
                let assign15130_e15280: f64 = (-locals.var_fn169_calc_iq__vdsin);
                let assign15130_e15282: f64 = (assign15130_e15280 / locals.var_fn169_calc_iq__vdsat10);
                let assign15130_e15283: f64 = assign15130_e15282;
                let assign15130_e15286: f64 = (-locals.var_fn169_calc_iq__vdsin);
                let assign15130_e15288: f64 = (assign15130_e15286 / locals.var_fn169_calc_iq__vdsat10);
                let assign15130_e15289: f64 = (-assign15130_e15288);
                let assign15130_e15292: f64 = (0.001 / p.p53);
                let assign15130_e15295: f64 = (-locals.var_fn169_calc_iq__vdsin);
                let assign15130_e15297: f64 = (assign15130_e15295 / locals.var_fn169_calc_iq__vdsat10);
                let assign15130_e15298: f64 = (-assign15130_e15297);
                let assign15130_e15299: f64 = (assign15130_e15292 * assign15130_e15298);
                let assign15130_e15300: f64 = (assign15130_e15299).tanh();
                let assign15130_e15301: f64 = (assign15130_e15289 * assign15130_e15300);
                let assign15130_e15302: f64 = (assign15130_e15283 + assign15130_e15301);
                let assign15130_e15303: f64 = (0.5 * assign15130_e15302);
                (assign15130_e15303, (0.5 * ((-((assign15130_e15280 * locals.var_fn169_calc_iq__vdsat10_dn2) / (locals.var_fn169_calc_iq__vdsat10 * locals.var_fn169_calc_iq__vdsat10))) + (((-(-((assign15130_e15286 * locals.var_fn169_calc_iq__vdsat10_dn2) / (locals.var_fn169_calc_iq__vdsat10 * locals.var_fn169_calc_iq__vdsat10)))) * assign15130_e15300) + (assign15130_e15289 * ((assign15130_e15292 * (-(-((assign15130_e15295 * locals.var_fn169_calc_iq__vdsat10_dn2) / (locals.var_fn169_calc_iq__vdsat10 * locals.var_fn169_calc_iq__vdsat10))))) / ((assign15130_e15299).cosh() * (assign15130_e15299).cosh())))))), (0.5 * ((-((assign15130_e15280 * locals.var_fn169_calc_iq__vdsat10_dn4) / (locals.var_fn169_calc_iq__vdsat10 * locals.var_fn169_calc_iq__vdsat10))) + (((-(-((assign15130_e15286 * locals.var_fn169_calc_iq__vdsat10_dn4) / (locals.var_fn169_calc_iq__vdsat10 * locals.var_fn169_calc_iq__vdsat10)))) * assign15130_e15300) + (assign15130_e15289 * ((assign15130_e15292 * (-(-((assign15130_e15295 * locals.var_fn169_calc_iq__vdsat10_dn4) / (locals.var_fn169_calc_iq__vdsat10 * locals.var_fn169_calc_iq__vdsat10))))) / ((assign15130_e15299).cosh() * (assign15130_e15299).cosh())))))), (0.5 * ((-((assign15130_e15280 * locals.var_fn169_calc_iq__vdsat10_dn7) / (locals.var_fn169_calc_iq__vdsat10 * locals.var_fn169_calc_iq__vdsat10))) + (((-(-((assign15130_e15286 * locals.var_fn169_calc_iq__vdsat10_dn7) / (locals.var_fn169_calc_iq__vdsat10 * locals.var_fn169_calc_iq__vdsat10)))) * assign15130_e15300) + (assign15130_e15289 * ((assign15130_e15292 * (-(-((assign15130_e15295 * locals.var_fn169_calc_iq__vdsat10_dn7) / (locals.var_fn169_calc_iq__vdsat10 * locals.var_fn169_calc_iq__vdsat10))))) / ((assign15130_e15299).cosh() * (assign15130_e15299).cosh())))))), (0.5 * (((((-locals.var_fn169_calc_iq__vdsin_dn9) * locals.var_fn169_calc_iq__vdsat10) - (assign15130_e15280 * locals.var_fn169_calc_iq__vdsat10_dn9)) / (locals.var_fn169_calc_iq__vdsat10 * locals.var_fn169_calc_iq__vdsat10)) + (((-((((-locals.var_fn169_calc_iq__vdsin_dn9) * locals.var_fn169_calc_iq__vdsat10) - (assign15130_e15286 * locals.var_fn169_calc_iq__vdsat10_dn9)) / (locals.var_fn169_calc_iq__vdsat10 * locals.var_fn169_calc_iq__vdsat10))) * assign15130_e15300) + (assign15130_e15289 * ((assign15130_e15292 * (-((((-locals.var_fn169_calc_iq__vdsin_dn9) * locals.var_fn169_calc_iq__vdsat10) - (assign15130_e15295 * locals.var_fn169_calc_iq__vdsat10_dn9)) / (locals.var_fn169_calc_iq__vdsat10 * locals.var_fn169_calc_iq__vdsat10)))) / ((assign15130_e15299).cosh() * (assign15130_e15299).cosh())))))), (0.5 * (((((-locals.var_fn169_calc_iq__vdsin_dn10) * locals.var_fn169_calc_iq__vdsat10) - (assign15130_e15280 * locals.var_fn169_calc_iq__vdsat10_dn10)) / (locals.var_fn169_calc_iq__vdsat10 * locals.var_fn169_calc_iq__vdsat10)) + (((-((((-locals.var_fn169_calc_iq__vdsin_dn10) * locals.var_fn169_calc_iq__vdsat10) - (assign15130_e15286 * locals.var_fn169_calc_iq__vdsat10_dn10)) / (locals.var_fn169_calc_iq__vdsat10 * locals.var_fn169_calc_iq__vdsat10))) * assign15130_e15300) + (assign15130_e15289 * ((assign15130_e15292 * (-((((-locals.var_fn169_calc_iq__vdsin_dn10) * locals.var_fn169_calc_iq__vdsat10) - (assign15130_e15295 * locals.var_fn169_calc_iq__vdsat10_dn10)) / (locals.var_fn169_calc_iq__vdsat10 * locals.var_fn169_calc_iq__vdsat10)))) / ((assign15130_e15299).cosh() * (assign15130_e15299).cosh())))))),)
            } else {
                let (assign15130_e15333, assign15130_e15333_d_n2, assign15130_e15333_d_n4, assign15130_e15333_d_n7, assign15130_e15333_d_n9, assign15130_e15333_d_n10,) = {
                    if (p.p52 == 0.0) {
                        let assign15130_e15310: f64 = (-locals.var_fn169_calc_iq__vdsin);
                        let assign15130_e15312: f64 = (assign15130_e15310 / locals.var_fn169_calc_iq__vdsat10);
                        let assign15130_e15313: f64 = assign15130_e15312;
                        let assign15130_e15316: f64 = (-locals.var_fn169_calc_iq__vdsin);
                        let assign15130_e15318: f64 = (assign15130_e15316 / locals.var_fn169_calc_iq__vdsat10);
                        let assign15130_e15319: f64 = (-assign15130_e15318);
                        let assign15130_e15322: f64 = (-locals.var_fn169_calc_iq__vdsin);
                        let assign15130_e15324: f64 = (assign15130_e15322 / locals.var_fn169_calc_iq__vdsat10);
                        let assign15130_e15325: f64 = (-assign15130_e15324);
                        let assign15130_e15326: f64 = (assign15130_e15319 * assign15130_e15325);
                        let assign15130_e15328: f64 = (assign15130_e15326 + p.p53);
                        let assign15130_e15329: f64 = (assign15130_e15328).sqrt();
                        let assign15130_e15330: f64 = (assign15130_e15313 + assign15130_e15329);
                        let assign15130_e15331: f64 = (0.5 * assign15130_e15330);
                        (assign15130_e15331, (0.5 * ((-((assign15130_e15310 * locals.var_fn169_calc_iq__vdsat10_dn2) / (locals.var_fn169_calc_iq__vdsat10 * locals.var_fn169_calc_iq__vdsat10))) + ((((-(-((assign15130_e15316 * locals.var_fn169_calc_iq__vdsat10_dn2) / (locals.var_fn169_calc_iq__vdsat10 * locals.var_fn169_calc_iq__vdsat10)))) * assign15130_e15325) + (assign15130_e15319 * (-(-((assign15130_e15322 * locals.var_fn169_calc_iq__vdsat10_dn2) / (locals.var_fn169_calc_iq__vdsat10 * locals.var_fn169_calc_iq__vdsat10)))))) / (2.0 * assign15130_e15329)))), (0.5 * ((-((assign15130_e15310 * locals.var_fn169_calc_iq__vdsat10_dn4) / (locals.var_fn169_calc_iq__vdsat10 * locals.var_fn169_calc_iq__vdsat10))) + ((((-(-((assign15130_e15316 * locals.var_fn169_calc_iq__vdsat10_dn4) / (locals.var_fn169_calc_iq__vdsat10 * locals.var_fn169_calc_iq__vdsat10)))) * assign15130_e15325) + (assign15130_e15319 * (-(-((assign15130_e15322 * locals.var_fn169_calc_iq__vdsat10_dn4) / (locals.var_fn169_calc_iq__vdsat10 * locals.var_fn169_calc_iq__vdsat10)))))) / (2.0 * assign15130_e15329)))), (0.5 * ((-((assign15130_e15310 * locals.var_fn169_calc_iq__vdsat10_dn7) / (locals.var_fn169_calc_iq__vdsat10 * locals.var_fn169_calc_iq__vdsat10))) + ((((-(-((assign15130_e15316 * locals.var_fn169_calc_iq__vdsat10_dn7) / (locals.var_fn169_calc_iq__vdsat10 * locals.var_fn169_calc_iq__vdsat10)))) * assign15130_e15325) + (assign15130_e15319 * (-(-((assign15130_e15322 * locals.var_fn169_calc_iq__vdsat10_dn7) / (locals.var_fn169_calc_iq__vdsat10 * locals.var_fn169_calc_iq__vdsat10)))))) / (2.0 * assign15130_e15329)))), (0.5 * (((((-locals.var_fn169_calc_iq__vdsin_dn9) * locals.var_fn169_calc_iq__vdsat10) - (assign15130_e15310 * locals.var_fn169_calc_iq__vdsat10_dn9)) / (locals.var_fn169_calc_iq__vdsat10 * locals.var_fn169_calc_iq__vdsat10)) + ((((-((((-locals.var_fn169_calc_iq__vdsin_dn9) * locals.var_fn169_calc_iq__vdsat10) - (assign15130_e15316 * locals.var_fn169_calc_iq__vdsat10_dn9)) / (locals.var_fn169_calc_iq__vdsat10 * locals.var_fn169_calc_iq__vdsat10))) * assign15130_e15325) + (assign15130_e15319 * (-((((-locals.var_fn169_calc_iq__vdsin_dn9) * locals.var_fn169_calc_iq__vdsat10) - (assign15130_e15322 * locals.var_fn169_calc_iq__vdsat10_dn9)) / (locals.var_fn169_calc_iq__vdsat10 * locals.var_fn169_calc_iq__vdsat10))))) / (2.0 * assign15130_e15329)))), (0.5 * (((((-locals.var_fn169_calc_iq__vdsin_dn10) * locals.var_fn169_calc_iq__vdsat10) - (assign15130_e15310 * locals.var_fn169_calc_iq__vdsat10_dn10)) / (locals.var_fn169_calc_iq__vdsat10 * locals.var_fn169_calc_iq__vdsat10)) + ((((-((((-locals.var_fn169_calc_iq__vdsin_dn10) * locals.var_fn169_calc_iq__vdsat10) - (assign15130_e15316 * locals.var_fn169_calc_iq__vdsat10_dn10)) / (locals.var_fn169_calc_iq__vdsat10 * locals.var_fn169_calc_iq__vdsat10))) * assign15130_e15325) + (assign15130_e15319 * (-((((-locals.var_fn169_calc_iq__vdsin_dn10) * locals.var_fn169_calc_iq__vdsat10) - (assign15130_e15322 * locals.var_fn169_calc_iq__vdsat10_dn10)) / (locals.var_fn169_calc_iq__vdsat10 * locals.var_fn169_calc_iq__vdsat10))))) / (2.0 * assign15130_e15329)))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign15130_e15333, assign15130_e15333_d_n2, assign15130_e15333_d_n4, assign15130_e15333_d_n7, assign15130_e15333_d_n9, assign15130_e15333_d_n10,)
            }
        };
        let assign15130_e15336: f64 = (assign15130_e15334).powf(locals.var_fn169_calc_iq__beta);
        let assign15130_e15337: f64 = (1.0 + assign15130_e15336);
        let assign15130_e15340: f64 = (1.0 / locals.var_fn169_calc_iq__beta);
        let assign15130_e15341: f64 = (assign15130_e15337).powf(assign15130_e15340);
        let assign15130_e15342: f64 = (1.0 / assign15130_e15341);
        (assign15130_e15342, (-(if 0.0 == 0.0 && ((assign15130_e15340) as f64).is_finite() && ((assign15130_e15340) as f64).fract() == 0.0 { if assign15130_e15340 == 0.0 { 0.0 } else { (assign15130_e15340 * ((assign15130_e15337).powf(assign15130_e15340 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn169_calc_iq__beta) as f64).is_finite() && ((locals.var_fn169_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn169_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn169_calc_iq__beta * ((assign15130_e15334).powf(locals.var_fn169_calc_iq__beta - 1.0) * assign15130_e15334_d_n2)) } } else { (assign15130_e15336 * (locals.var_fn169_calc_iq__beta * (assign15130_e15334_d_n2 / assign15130_e15334))) })) } } else { (assign15130_e15341 * (assign15130_e15340 * (if 0.0 == 0.0 && ((locals.var_fn169_calc_iq__beta) as f64).is_finite() && ((locals.var_fn169_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn169_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn169_calc_iq__beta * ((assign15130_e15334).powf(locals.var_fn169_calc_iq__beta - 1.0) * assign15130_e15334_d_n2)) } } else { (assign15130_e15336 * (locals.var_fn169_calc_iq__beta * (assign15130_e15334_d_n2 / assign15130_e15334))) } / assign15130_e15337))) } / (assign15130_e15341 * assign15130_e15341))), (-(if 0.0 == 0.0 && ((assign15130_e15340) as f64).is_finite() && ((assign15130_e15340) as f64).fract() == 0.0 { if assign15130_e15340 == 0.0 { 0.0 } else { (assign15130_e15340 * ((assign15130_e15337).powf(assign15130_e15340 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn169_calc_iq__beta) as f64).is_finite() && ((locals.var_fn169_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn169_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn169_calc_iq__beta * ((assign15130_e15334).powf(locals.var_fn169_calc_iq__beta - 1.0) * assign15130_e15334_d_n4)) } } else { (assign15130_e15336 * (locals.var_fn169_calc_iq__beta * (assign15130_e15334_d_n4 / assign15130_e15334))) })) } } else { (assign15130_e15341 * (assign15130_e15340 * (if 0.0 == 0.0 && ((locals.var_fn169_calc_iq__beta) as f64).is_finite() && ((locals.var_fn169_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn169_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn169_calc_iq__beta * ((assign15130_e15334).powf(locals.var_fn169_calc_iq__beta - 1.0) * assign15130_e15334_d_n4)) } } else { (assign15130_e15336 * (locals.var_fn169_calc_iq__beta * (assign15130_e15334_d_n4 / assign15130_e15334))) } / assign15130_e15337))) } / (assign15130_e15341 * assign15130_e15341))), (-(if 0.0 == 0.0 && ((assign15130_e15340) as f64).is_finite() && ((assign15130_e15340) as f64).fract() == 0.0 { if assign15130_e15340 == 0.0 { 0.0 } else { (assign15130_e15340 * ((assign15130_e15337).powf(assign15130_e15340 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn169_calc_iq__beta) as f64).is_finite() && ((locals.var_fn169_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn169_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn169_calc_iq__beta * ((assign15130_e15334).powf(locals.var_fn169_calc_iq__beta - 1.0) * assign15130_e15334_d_n7)) } } else { (assign15130_e15336 * (locals.var_fn169_calc_iq__beta * (assign15130_e15334_d_n7 / assign15130_e15334))) })) } } else { (assign15130_e15341 * (assign15130_e15340 * (if 0.0 == 0.0 && ((locals.var_fn169_calc_iq__beta) as f64).is_finite() && ((locals.var_fn169_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn169_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn169_calc_iq__beta * ((assign15130_e15334).powf(locals.var_fn169_calc_iq__beta - 1.0) * assign15130_e15334_d_n7)) } } else { (assign15130_e15336 * (locals.var_fn169_calc_iq__beta * (assign15130_e15334_d_n7 / assign15130_e15334))) } / assign15130_e15337))) } / (assign15130_e15341 * assign15130_e15341))), (-(if 0.0 == 0.0 && ((assign15130_e15340) as f64).is_finite() && ((assign15130_e15340) as f64).fract() == 0.0 { if assign15130_e15340 == 0.0 { 0.0 } else { (assign15130_e15340 * ((assign15130_e15337).powf(assign15130_e15340 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn169_calc_iq__beta) as f64).is_finite() && ((locals.var_fn169_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn169_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn169_calc_iq__beta * ((assign15130_e15334).powf(locals.var_fn169_calc_iq__beta - 1.0) * assign15130_e15334_d_n9)) } } else { (assign15130_e15336 * (locals.var_fn169_calc_iq__beta * (assign15130_e15334_d_n9 / assign15130_e15334))) })) } } else { (assign15130_e15341 * (assign15130_e15340 * (if 0.0 == 0.0 && ((locals.var_fn169_calc_iq__beta) as f64).is_finite() && ((locals.var_fn169_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn169_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn169_calc_iq__beta * ((assign15130_e15334).powf(locals.var_fn169_calc_iq__beta - 1.0) * assign15130_e15334_d_n9)) } } else { (assign15130_e15336 * (locals.var_fn169_calc_iq__beta * (assign15130_e15334_d_n9 / assign15130_e15334))) } / assign15130_e15337))) } / (assign15130_e15341 * assign15130_e15341))), (-(if 0.0 == 0.0 && ((assign15130_e15340) as f64).is_finite() && ((assign15130_e15340) as f64).fract() == 0.0 { if assign15130_e15340 == 0.0 { 0.0 } else { (assign15130_e15340 * ((assign15130_e15337).powf(assign15130_e15340 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn169_calc_iq__beta) as f64).is_finite() && ((locals.var_fn169_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn169_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn169_calc_iq__beta * ((assign15130_e15334).powf(locals.var_fn169_calc_iq__beta - 1.0) * assign15130_e15334_d_n10)) } } else { (assign15130_e15336 * (locals.var_fn169_calc_iq__beta * (assign15130_e15334_d_n10 / assign15130_e15334))) })) } } else { (assign15130_e15341 * (assign15130_e15340 * (if 0.0 == 0.0 && ((locals.var_fn169_calc_iq__beta) as f64).is_finite() && ((locals.var_fn169_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn169_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn169_calc_iq__beta * ((assign15130_e15334).powf(locals.var_fn169_calc_iq__beta - 1.0) * assign15130_e15334_d_n10)) } } else { (assign15130_e15336 * (locals.var_fn169_calc_iq__beta * (assign15130_e15334_d_n10 / assign15130_e15334))) } / assign15130_e15337))) } / (assign15130_e15341 * assign15130_e15341))),)
    } else {
        (locals.var_fn169_calc_iq__fds0, locals.var_fn169_calc_iq__fds0_dn2, locals.var_fn169_calc_iq__fds0_dn4, locals.var_fn169_calc_iq__fds0_dn7, locals.var_fn169_calc_iq__fds0_dn9, locals.var_fn169_calc_iq__fds0_dn10,)
    }
};
        locals.var_fn169_calc_iq__fds0 = assign15130_e15344;
        locals.var_fn169_calc_iq__fds0_dn2 = assign15130_e15344_d_n2;
        locals.var_fn169_calc_iq__fds0_dn4 = assign15130_e15344_d_n4;
        locals.var_fn169_calc_iq__fds0_dn7 = assign15130_e15344_d_n7;
        locals.var_fn169_calc_iq__fds0_dn9 = assign15130_e15344_d_n9;
        locals.var_fn169_calc_iq__fds0_dn10 = assign15130_e15344_d_n10;

        let (assign15140_e15351, assign15140_e15351_d_n2, assign15140_e15351_d_n4, assign15140_e15351_d_n7, assign15140_e15351_d_n9, assign15140_e15351_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        let assign15140_e15347: f64 = (-locals.var_fn169_calc_iq__vdsin);
        let assign15140_e15349: f64 = (assign15140_e15347 * locals.var_fn169_calc_iq__fds0);
        (assign15140_e15349, (assign15140_e15347 * locals.var_fn169_calc_iq__fds0_dn2), (assign15140_e15347 * locals.var_fn169_calc_iq__fds0_dn4), (assign15140_e15347 * locals.var_fn169_calc_iq__fds0_dn7), (((-locals.var_fn169_calc_iq__vdsin_dn9) * locals.var_fn169_calc_iq__fds0) + (assign15140_e15347 * locals.var_fn169_calc_iq__fds0_dn9)), (((-locals.var_fn169_calc_iq__vdsin_dn10) * locals.var_fn169_calc_iq__fds0) + (assign15140_e15347 * locals.var_fn169_calc_iq__fds0_dn10)),)
    } else {
        (locals.var_fn169_calc_iq__vsx0, locals.var_fn169_calc_iq__vsx0_dn2, locals.var_fn169_calc_iq__vsx0_dn4, locals.var_fn169_calc_iq__vsx0_dn7, locals.var_fn169_calc_iq__vsx0_dn9, locals.var_fn169_calc_iq__vsx0_dn10,)
    }
};
        locals.var_fn169_calc_iq__vsx0 = assign15140_e15351;
        locals.var_fn169_calc_iq__vsx0_dn2 = assign15140_e15351_d_n2;
        locals.var_fn169_calc_iq__vsx0_dn4 = assign15140_e15351_d_n4;
        locals.var_fn169_calc_iq__vsx0_dn7 = assign15140_e15351_d_n7;
        locals.var_fn169_calc_iq__vsx0_dn9 = assign15140_e15351_d_n9;
        locals.var_fn169_calc_iq__vsx0_dn10 = assign15140_e15351_d_n10;

        let (assign15150_e15359, assign15150_e15359_d_n2, assign15150_e15359_d_n4, assign15150_e15359_d_n7, assign15150_e15359_d_n9, assign15150_e15359_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        let assign15150_e15355: f64 = (locals.var_fn169_calc_iq__vgsin - locals.var_fn169_calc_iq__myarg0);
        let assign15150_e15357: f64 = (assign15150_e15355 / locals.var_fn169_calc_iq__alpha_phit);
        (assign15150_e15357, (locals.var_fn169_calc_iq__vgsin_dn2 / locals.var_fn169_calc_iq__alpha_phit), ((((-locals.var_fn169_calc_iq__myarg0_dn4) * locals.var_fn169_calc_iq__alpha_phit) - (assign15150_e15355 * locals.var_fn169_calc_iq__alpha_phit_dn4)) / (locals.var_fn169_calc_iq__alpha_phit * locals.var_fn169_calc_iq__alpha_phit)), (locals.var_fn169_calc_iq__vgsin_dn7 / locals.var_fn169_calc_iq__alpha_phit), 0.0, (locals.var_fn169_calc_iq__vgsin_dn10 / locals.var_fn169_calc_iq__alpha_phit),)
    } else {
        (locals.var_fn169_calc_iq__exparg0, locals.var_fn169_calc_iq__exparg0_dn2, locals.var_fn169_calc_iq__exparg0_dn4, locals.var_fn169_calc_iq__exparg0_dn7, locals.var_fn169_calc_iq__exparg0_dn9, locals.var_fn169_calc_iq__exparg0_dn10,)
    }
};
        locals.var_fn169_calc_iq__exparg0 = assign15150_e15359;
        locals.var_fn169_calc_iq__exparg0_dn2 = assign15150_e15359_d_n2;
        locals.var_fn169_calc_iq__exparg0_dn4 = assign15150_e15359_d_n4;
        locals.var_fn169_calc_iq__exparg0_dn7 = assign15150_e15359_d_n7;
        locals.var_fn169_calc_iq__exparg0_dn9 = assign15150_e15359_d_n9;
        locals.var_fn169_calc_iq__exparg0_dn10 = assign15150_e15359_d_n10;

        let assign15160_e15362: f64 = if locals.var_fn169_calc_iq__exparg0 > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard187 = assign15160_e15362;

        let (assign15170_e15368, assign15170_e15368_d_n2, assign15170_e15368_d_n4, assign15170_e15368_d_n7, assign15170_e15368_d_n9, assign15170_e15368_d_n10,) = {
    if ((locals.var_guard168 != 0.0) && (locals.var_guard187 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__ffs0, locals.var_fn169_calc_iq__ffs0_dn2, locals.var_fn169_calc_iq__ffs0_dn4, locals.var_fn169_calc_iq__ffs0_dn7, locals.var_fn169_calc_iq__ffs0_dn9, locals.var_fn169_calc_iq__ffs0_dn10,)
    }
};
        locals.var_fn169_calc_iq__ffs0 = assign15170_e15368;
        locals.var_fn169_calc_iq__ffs0_dn2 = assign15170_e15368_d_n2;
        locals.var_fn169_calc_iq__ffs0_dn4 = assign15170_e15368_d_n4;
        locals.var_fn169_calc_iq__ffs0_dn7 = assign15170_e15368_d_n7;
        locals.var_fn169_calc_iq__ffs0_dn9 = assign15170_e15368_d_n9;
        locals.var_fn169_calc_iq__ffs0_dn10 = assign15170_e15368_d_n10;

        let assign15180_e15371: f64 = (-50.0);
        let assign15180_e15372: f64 = if locals.var_fn169_calc_iq__exparg0 < assign15180_e15371 { 1.0 } else { 0.0 };
        locals.var_guard188 = assign15180_e15372;

        let (assign15190_e15381, assign15190_e15381_d_n2, assign15190_e15381_d_n4, assign15190_e15381_d_n7, assign15190_e15381_d_n9, assign15190_e15381_d_n10,) = {
    if (((locals.var_guard168 != 0.0) && (locals.var_guard187 == 0.0)) && (locals.var_guard188 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__ffs0, locals.var_fn169_calc_iq__ffs0_dn2, locals.var_fn169_calc_iq__ffs0_dn4, locals.var_fn169_calc_iq__ffs0_dn7, locals.var_fn169_calc_iq__ffs0_dn9, locals.var_fn169_calc_iq__ffs0_dn10,)
    }
};
        locals.var_fn169_calc_iq__ffs0 = assign15190_e15381;
        locals.var_fn169_calc_iq__ffs0_dn2 = assign15190_e15381_d_n2;
        locals.var_fn169_calc_iq__ffs0_dn4 = assign15190_e15381_d_n4;
        locals.var_fn169_calc_iq__ffs0_dn7 = assign15190_e15381_d_n7;
        locals.var_fn169_calc_iq__ffs0_dn9 = assign15190_e15381_d_n9;
        locals.var_fn169_calc_iq__ffs0_dn10 = assign15190_e15381_d_n10;

        let (assign15200_e15396, assign15200_e15396_d_n2, assign15200_e15396_d_n4, assign15200_e15396_d_n7, assign15200_e15396_d_n9, assign15200_e15396_d_n10,) = {
    if (((locals.var_guard168 != 0.0) && (locals.var_guard187 == 0.0)) && (locals.var_guard188 == 0.0)) {
        let assign15200_e15392: f64 = (locals.var_fn169_calc_iq__exparg0).exp();
        let assign15200_e15393: f64 = (1.0 + assign15200_e15392);
        let assign15200_e15394: f64 = (1.0 / assign15200_e15393);
        (assign15200_e15394, (-((assign15200_e15392 * locals.var_fn169_calc_iq__exparg0_dn2) / (assign15200_e15393 * assign15200_e15393))), (-((assign15200_e15392 * locals.var_fn169_calc_iq__exparg0_dn4) / (assign15200_e15393 * assign15200_e15393))), (-((assign15200_e15392 * locals.var_fn169_calc_iq__exparg0_dn7) / (assign15200_e15393 * assign15200_e15393))), (-((assign15200_e15392 * locals.var_fn169_calc_iq__exparg0_dn9) / (assign15200_e15393 * assign15200_e15393))), (-((assign15200_e15392 * locals.var_fn169_calc_iq__exparg0_dn10) / (assign15200_e15393 * assign15200_e15393))),)
    } else {
        (locals.var_fn169_calc_iq__ffs0, locals.var_fn169_calc_iq__ffs0_dn2, locals.var_fn169_calc_iq__ffs0_dn4, locals.var_fn169_calc_iq__ffs0_dn7, locals.var_fn169_calc_iq__ffs0_dn9, locals.var_fn169_calc_iq__ffs0_dn10,)
    }
};
        locals.var_fn169_calc_iq__ffs0 = assign15200_e15396;
        locals.var_fn169_calc_iq__ffs0_dn2 = assign15200_e15396_d_n2;
        locals.var_fn169_calc_iq__ffs0_dn4 = assign15200_e15396_d_n4;
        locals.var_fn169_calc_iq__ffs0_dn7 = assign15200_e15396_d_n7;
        locals.var_fn169_calc_iq__ffs0_dn9 = assign15200_e15396_d_n9;
        locals.var_fn169_calc_iq__ffs0_dn10 = assign15200_e15396_d_n10;

    }

    pub(super) fn stamp_transient_block_38(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign15210_e15414, assign15210_e15414_d_n2, assign15210_e15414_d_n4, assign15210_e15414_d_n7, assign15210_e15414_d_n9, assign15210_e15414_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        let assign15210_e15400: f64 = (locals.var_fn169_calc_iq__vgdin - locals.var_fn169_calc_iq__vsx0);
        let assign15210_e15404: f64 = (p.p51 * 0.1);
        let assign15210_e15406: f64 = (assign15210_e15404 * locals.var_fn169_calc_iq__alpha_phit);
        let assign15210_e15408: f64 = (assign15210_e15406 * locals.var_fn169_calc_iq__ffs0);
        let assign15210_e15409: f64 = (locals.var_fn169_calc_iq__vtof - assign15210_e15408);
        let assign15210_e15410: f64 = (assign15210_e15400 - assign15210_e15409);
        let assign15210_e15412: f64 = (assign15210_e15410 / locals.var_fn169_calc_iq__two_n_phit0);
        (assign15210_e15412, (((locals.var_fn169_calc_iq__vgdin_dn2 - locals.var_fn169_calc_iq__vsx0_dn2) - (-(assign15210_e15406 * locals.var_fn169_calc_iq__ffs0_dn2))) / locals.var_fn169_calc_iq__two_n_phit0), (((((-locals.var_fn169_calc_iq__vsx0_dn4) - (locals.var_fn169_calc_iq__vtof_dn4 - (((assign15210_e15404 * locals.var_fn169_calc_iq__alpha_phit_dn4) * locals.var_fn169_calc_iq__ffs0) + (assign15210_e15406 * locals.var_fn169_calc_iq__ffs0_dn4)))) * locals.var_fn169_calc_iq__two_n_phit0) - (assign15210_e15410 * locals.var_fn169_calc_iq__two_n_phit0_dn4)) / (locals.var_fn169_calc_iq__two_n_phit0 * locals.var_fn169_calc_iq__two_n_phit0)), (((locals.var_fn169_calc_iq__vgdin_dn7 - locals.var_fn169_calc_iq__vsx0_dn7) - (-(assign15210_e15406 * locals.var_fn169_calc_iq__ffs0_dn7))) / locals.var_fn169_calc_iq__two_n_phit0), (((locals.var_fn169_calc_iq__vgdin_dn9 - locals.var_fn169_calc_iq__vsx0_dn9) - (-(assign15210_e15406 * locals.var_fn169_calc_iq__ffs0_dn9))) / locals.var_fn169_calc_iq__two_n_phit0), (((locals.var_fn169_calc_iq__vgdin_dn10 - locals.var_fn169_calc_iq__vsx0_dn10) - (-(assign15210_e15406 * locals.var_fn169_calc_iq__ffs0_dn10))) / locals.var_fn169_calc_iq__two_n_phit0),)
    } else {
        (locals.var_fn169_calc_iq__etas0, locals.var_fn169_calc_iq__etas0_dn2, locals.var_fn169_calc_iq__etas0_dn4, locals.var_fn169_calc_iq__etas0_dn7, locals.var_fn169_calc_iq__etas0_dn9, locals.var_fn169_calc_iq__etas0_dn10,)
    }
};
        locals.var_fn169_calc_iq__etas0 = assign15210_e15414;
        locals.var_fn169_calc_iq__etas0_dn2 = assign15210_e15414_d_n2;
        locals.var_fn169_calc_iq__etas0_dn4 = assign15210_e15414_d_n4;
        locals.var_fn169_calc_iq__etas0_dn7 = assign15210_e15414_d_n7;
        locals.var_fn169_calc_iq__etas0_dn9 = assign15210_e15414_d_n9;
        locals.var_fn169_calc_iq__etas0_dn10 = assign15210_e15414_d_n10;

        let assign15220_e15417: f64 = if locals.var_fn169_calc_iq__etas0 > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard189 = assign15220_e15417;

        let (assign15230_e15425, assign15230_e15425_d_n2, assign15230_e15425_d_n4, assign15230_e15425_d_n7, assign15230_e15425_d_n9, assign15230_e15425_d_n10,) = {
    if ((locals.var_guard168 != 0.0) && (locals.var_guard189 != 0.0)) {
        let assign15230_e15423: f64 = (locals.var_fn169_calc_iq__qref0 * locals.var_fn169_calc_iq__etas0);
        (assign15230_e15423, (locals.var_fn169_calc_iq__qref0 * locals.var_fn169_calc_iq__etas0_dn2), ((locals.var_fn169_calc_iq__qref0_dn4 * locals.var_fn169_calc_iq__etas0) + (locals.var_fn169_calc_iq__qref0 * locals.var_fn169_calc_iq__etas0_dn4)), (locals.var_fn169_calc_iq__qref0 * locals.var_fn169_calc_iq__etas0_dn7), (locals.var_fn169_calc_iq__qref0 * locals.var_fn169_calc_iq__etas0_dn9), (locals.var_fn169_calc_iq__qref0 * locals.var_fn169_calc_iq__etas0_dn10),)
    } else {
        (locals.var_fn169_calc_iq__qinvs0, locals.var_fn169_calc_iq__qinvs0_dn2, locals.var_fn169_calc_iq__qinvs0_dn4, locals.var_fn169_calc_iq__qinvs0_dn7, locals.var_fn169_calc_iq__qinvs0_dn9, locals.var_fn169_calc_iq__qinvs0_dn10,)
    }
};
        locals.var_fn169_calc_iq__qinvs0 = assign15230_e15425;
        locals.var_fn169_calc_iq__qinvs0_dn2 = assign15230_e15425_d_n2;
        locals.var_fn169_calc_iq__qinvs0_dn4 = assign15230_e15425_d_n4;
        locals.var_fn169_calc_iq__qinvs0_dn7 = assign15230_e15425_d_n7;
        locals.var_fn169_calc_iq__qinvs0_dn9 = assign15230_e15425_d_n9;
        locals.var_fn169_calc_iq__qinvs0_dn10 = assign15230_e15425_d_n10;

        let assign15240_e15428: f64 = (-50.0);
        let assign15240_e15429: f64 = if locals.var_fn169_calc_iq__etas0 < assign15240_e15428 { 1.0 } else { 0.0 };
        locals.var_guard190 = assign15240_e15429;

        let (assign15250_e15441, assign15250_e15441_d_n2, assign15250_e15441_d_n4, assign15250_e15441_d_n7, assign15250_e15441_d_n9, assign15250_e15441_d_n10,) = {
    if (((locals.var_guard168 != 0.0) && (locals.var_guard189 == 0.0)) && (locals.var_guard190 != 0.0)) {
        let assign15250_e15438: f64 = (locals.var_fn169_calc_iq__etas0).exp();
        let assign15250_e15439: f64 = (locals.var_fn169_calc_iq__qref0 * assign15250_e15438);
        (assign15250_e15439, (locals.var_fn169_calc_iq__qref0 * (assign15250_e15438 * locals.var_fn169_calc_iq__etas0_dn2)), ((locals.var_fn169_calc_iq__qref0_dn4 * assign15250_e15438) + (locals.var_fn169_calc_iq__qref0 * (assign15250_e15438 * locals.var_fn169_calc_iq__etas0_dn4))), (locals.var_fn169_calc_iq__qref0 * (assign15250_e15438 * locals.var_fn169_calc_iq__etas0_dn7)), (locals.var_fn169_calc_iq__qref0 * (assign15250_e15438 * locals.var_fn169_calc_iq__etas0_dn9)), (locals.var_fn169_calc_iq__qref0 * (assign15250_e15438 * locals.var_fn169_calc_iq__etas0_dn10)),)
    } else {
        (locals.var_fn169_calc_iq__qinvs0, locals.var_fn169_calc_iq__qinvs0_dn2, locals.var_fn169_calc_iq__qinvs0_dn4, locals.var_fn169_calc_iq__qinvs0_dn7, locals.var_fn169_calc_iq__qinvs0_dn9, locals.var_fn169_calc_iq__qinvs0_dn10,)
    }
};
        locals.var_fn169_calc_iq__qinvs0 = assign15250_e15441;
        locals.var_fn169_calc_iq__qinvs0_dn2 = assign15250_e15441_d_n2;
        locals.var_fn169_calc_iq__qinvs0_dn4 = assign15250_e15441_d_n4;
        locals.var_fn169_calc_iq__qinvs0_dn7 = assign15250_e15441_d_n7;
        locals.var_fn169_calc_iq__qinvs0_dn9 = assign15250_e15441_d_n9;
        locals.var_fn169_calc_iq__qinvs0_dn10 = assign15250_e15441_d_n10;

        let (assign15260_e15457, assign15260_e15457_d_n2, assign15260_e15457_d_n4, assign15260_e15457_d_n7, assign15260_e15457_d_n9, assign15260_e15457_d_n10,) = {
    if (((locals.var_guard168 != 0.0) && (locals.var_guard189 == 0.0)) && (locals.var_guard190 == 0.0)) {
        let assign15260_e15452: f64 = (locals.var_fn169_calc_iq__etas0).exp();
        let assign15260_e15453: f64 = (1.0 + assign15260_e15452);
        let assign15260_e15454: f64 = (assign15260_e15453).ln();
        let assign15260_e15455: f64 = (locals.var_fn169_calc_iq__qref0 * assign15260_e15454);
        (assign15260_e15455, (locals.var_fn169_calc_iq__qref0 * ((assign15260_e15452 * locals.var_fn169_calc_iq__etas0_dn2) / assign15260_e15453)), ((locals.var_fn169_calc_iq__qref0_dn4 * assign15260_e15454) + (locals.var_fn169_calc_iq__qref0 * ((assign15260_e15452 * locals.var_fn169_calc_iq__etas0_dn4) / assign15260_e15453))), (locals.var_fn169_calc_iq__qref0 * ((assign15260_e15452 * locals.var_fn169_calc_iq__etas0_dn7) / assign15260_e15453)), (locals.var_fn169_calc_iq__qref0 * ((assign15260_e15452 * locals.var_fn169_calc_iq__etas0_dn9) / assign15260_e15453)), (locals.var_fn169_calc_iq__qref0 * ((assign15260_e15452 * locals.var_fn169_calc_iq__etas0_dn10) / assign15260_e15453)),)
    } else {
        (locals.var_fn169_calc_iq__qinvs0, locals.var_fn169_calc_iq__qinvs0_dn2, locals.var_fn169_calc_iq__qinvs0_dn4, locals.var_fn169_calc_iq__qinvs0_dn7, locals.var_fn169_calc_iq__qinvs0_dn9, locals.var_fn169_calc_iq__qinvs0_dn10,)
    }
};
        locals.var_fn169_calc_iq__qinvs0 = assign15260_e15457;
        locals.var_fn169_calc_iq__qinvs0_dn2 = assign15260_e15457_d_n2;
        locals.var_fn169_calc_iq__qinvs0_dn4 = assign15260_e15457_d_n4;
        locals.var_fn169_calc_iq__qinvs0_dn7 = assign15260_e15457_d_n7;
        locals.var_fn169_calc_iq__qinvs0_dn9 = assign15260_e15457_d_n9;
        locals.var_fn169_calc_iq__qinvs0_dn10 = assign15260_e15457_d_n10;

        let (assign15270_e15465, assign15270_e15465_d_n2, assign15270_e15465_d_n4, assign15270_e15465_d_n7, assign15270_e15465_d_n9, assign15270_e15465_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        let assign15270_e15461: f64 = (locals.var_fn169_calc_iq__vgdin - locals.var_fn169_calc_iq__myarg0);
        let assign15270_e15463: f64 = (assign15270_e15461 / locals.var_fn169_calc_iq__alpha_phit);
        (assign15270_e15463, (locals.var_fn169_calc_iq__vgdin_dn2 / locals.var_fn169_calc_iq__alpha_phit), ((((-locals.var_fn169_calc_iq__myarg0_dn4) * locals.var_fn169_calc_iq__alpha_phit) - (assign15270_e15461 * locals.var_fn169_calc_iq__alpha_phit_dn4)) / (locals.var_fn169_calc_iq__alpha_phit * locals.var_fn169_calc_iq__alpha_phit)), (locals.var_fn169_calc_iq__vgdin_dn7 / locals.var_fn169_calc_iq__alpha_phit), (locals.var_fn169_calc_iq__vgdin_dn9 / locals.var_fn169_calc_iq__alpha_phit), (locals.var_fn169_calc_iq__vgdin_dn10 / locals.var_fn169_calc_iq__alpha_phit),)
    } else {
        (locals.var_fn169_calc_iq__exparg0, locals.var_fn169_calc_iq__exparg0_dn2, locals.var_fn169_calc_iq__exparg0_dn4, locals.var_fn169_calc_iq__exparg0_dn7, locals.var_fn169_calc_iq__exparg0_dn9, locals.var_fn169_calc_iq__exparg0_dn10,)
    }
};
        locals.var_fn169_calc_iq__exparg0 = assign15270_e15465;
        locals.var_fn169_calc_iq__exparg0_dn2 = assign15270_e15465_d_n2;
        locals.var_fn169_calc_iq__exparg0_dn4 = assign15270_e15465_d_n4;
        locals.var_fn169_calc_iq__exparg0_dn7 = assign15270_e15465_d_n7;
        locals.var_fn169_calc_iq__exparg0_dn9 = assign15270_e15465_d_n9;
        locals.var_fn169_calc_iq__exparg0_dn10 = assign15270_e15465_d_n10;

        let assign15280_e15468: f64 = if locals.var_fn169_calc_iq__exparg0 > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard191 = assign15280_e15468;

        let (assign15290_e15474, assign15290_e15474_d_n2, assign15290_e15474_d_n4, assign15290_e15474_d_n7, assign15290_e15474_d_n9, assign15290_e15474_d_n10,) = {
    if ((locals.var_guard168 != 0.0) && (locals.var_guard191 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__ffd0, locals.var_fn169_calc_iq__ffd0_dn2, locals.var_fn169_calc_iq__ffd0_dn4, locals.var_fn169_calc_iq__ffd0_dn7, locals.var_fn169_calc_iq__ffd0_dn9, locals.var_fn169_calc_iq__ffd0_dn10,)
    }
};
        locals.var_fn169_calc_iq__ffd0 = assign15290_e15474;
        locals.var_fn169_calc_iq__ffd0_dn2 = assign15290_e15474_d_n2;
        locals.var_fn169_calc_iq__ffd0_dn4 = assign15290_e15474_d_n4;
        locals.var_fn169_calc_iq__ffd0_dn7 = assign15290_e15474_d_n7;
        locals.var_fn169_calc_iq__ffd0_dn9 = assign15290_e15474_d_n9;
        locals.var_fn169_calc_iq__ffd0_dn10 = assign15290_e15474_d_n10;

        let assign15300_e15477: f64 = (-50.0);
        let assign15300_e15478: f64 = if locals.var_fn169_calc_iq__exparg0 < assign15300_e15477 { 1.0 } else { 0.0 };
        locals.var_guard192 = assign15300_e15478;

        let (assign15310_e15487, assign15310_e15487_d_n2, assign15310_e15487_d_n4, assign15310_e15487_d_n7, assign15310_e15487_d_n9, assign15310_e15487_d_n10,) = {
    if (((locals.var_guard168 != 0.0) && (locals.var_guard191 == 0.0)) && (locals.var_guard192 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__ffd0, locals.var_fn169_calc_iq__ffd0_dn2, locals.var_fn169_calc_iq__ffd0_dn4, locals.var_fn169_calc_iq__ffd0_dn7, locals.var_fn169_calc_iq__ffd0_dn9, locals.var_fn169_calc_iq__ffd0_dn10,)
    }
};
        locals.var_fn169_calc_iq__ffd0 = assign15310_e15487;
        locals.var_fn169_calc_iq__ffd0_dn2 = assign15310_e15487_d_n2;
        locals.var_fn169_calc_iq__ffd0_dn4 = assign15310_e15487_d_n4;
        locals.var_fn169_calc_iq__ffd0_dn7 = assign15310_e15487_d_n7;
        locals.var_fn169_calc_iq__ffd0_dn9 = assign15310_e15487_d_n9;
        locals.var_fn169_calc_iq__ffd0_dn10 = assign15310_e15487_d_n10;

        let (assign15320_e15502, assign15320_e15502_d_n2, assign15320_e15502_d_n4, assign15320_e15502_d_n7, assign15320_e15502_d_n9, assign15320_e15502_d_n10,) = {
    if (((locals.var_guard168 != 0.0) && (locals.var_guard191 == 0.0)) && (locals.var_guard192 == 0.0)) {
        let assign15320_e15498: f64 = (locals.var_fn169_calc_iq__exparg0).exp();
        let assign15320_e15499: f64 = (1.0 + assign15320_e15498);
        let assign15320_e15500: f64 = (1.0 / assign15320_e15499);
        (assign15320_e15500, (-((assign15320_e15498 * locals.var_fn169_calc_iq__exparg0_dn2) / (assign15320_e15499 * assign15320_e15499))), (-((assign15320_e15498 * locals.var_fn169_calc_iq__exparg0_dn4) / (assign15320_e15499 * assign15320_e15499))), (-((assign15320_e15498 * locals.var_fn169_calc_iq__exparg0_dn7) / (assign15320_e15499 * assign15320_e15499))), (-((assign15320_e15498 * locals.var_fn169_calc_iq__exparg0_dn9) / (assign15320_e15499 * assign15320_e15499))), (-((assign15320_e15498 * locals.var_fn169_calc_iq__exparg0_dn10) / (assign15320_e15499 * assign15320_e15499))),)
    } else {
        (locals.var_fn169_calc_iq__ffd0, locals.var_fn169_calc_iq__ffd0_dn2, locals.var_fn169_calc_iq__ffd0_dn4, locals.var_fn169_calc_iq__ffd0_dn7, locals.var_fn169_calc_iq__ffd0_dn9, locals.var_fn169_calc_iq__ffd0_dn10,)
    }
};
        locals.var_fn169_calc_iq__ffd0 = assign15320_e15502;
        locals.var_fn169_calc_iq__ffd0_dn2 = assign15320_e15502_d_n2;
        locals.var_fn169_calc_iq__ffd0_dn4 = assign15320_e15502_d_n4;
        locals.var_fn169_calc_iq__ffd0_dn7 = assign15320_e15502_d_n7;
        locals.var_fn169_calc_iq__ffd0_dn9 = assign15320_e15502_d_n9;
        locals.var_fn169_calc_iq__ffd0_dn10 = assign15320_e15502_d_n10;

        let (assign15330_e15520, assign15330_e15520_d_n2, assign15330_e15520_d_n4, assign15330_e15520_d_n7, assign15330_e15520_d_n9, assign15330_e15520_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        let assign15330_e15506: f64 = (locals.var_fn169_calc_iq__vgsin - locals.var_fn169_calc_iq__vdx0);
        let assign15330_e15510: f64 = (p.p51 * 0.1);
        let assign15330_e15512: f64 = (assign15330_e15510 * locals.var_fn169_calc_iq__alpha_phit);
        let assign15330_e15514: f64 = (assign15330_e15512 * locals.var_fn169_calc_iq__ffd0);
        let assign15330_e15515: f64 = (locals.var_fn169_calc_iq__vtof - assign15330_e15514);
        let assign15330_e15516: f64 = (assign15330_e15506 - assign15330_e15515);
        let assign15330_e15518: f64 = (assign15330_e15516 / locals.var_fn169_calc_iq__two_n_phit0);
        (assign15330_e15518, (((locals.var_fn169_calc_iq__vgsin_dn2 - locals.var_fn169_calc_iq__vdx0_dn2) - (-(assign15330_e15512 * locals.var_fn169_calc_iq__ffd0_dn2))) / locals.var_fn169_calc_iq__two_n_phit0), (((((-locals.var_fn169_calc_iq__vdx0_dn4) - (locals.var_fn169_calc_iq__vtof_dn4 - (((assign15330_e15510 * locals.var_fn169_calc_iq__alpha_phit_dn4) * locals.var_fn169_calc_iq__ffd0) + (assign15330_e15512 * locals.var_fn169_calc_iq__ffd0_dn4)))) * locals.var_fn169_calc_iq__two_n_phit0) - (assign15330_e15516 * locals.var_fn169_calc_iq__two_n_phit0_dn4)) / (locals.var_fn169_calc_iq__two_n_phit0 * locals.var_fn169_calc_iq__two_n_phit0)), (((locals.var_fn169_calc_iq__vgsin_dn7 - locals.var_fn169_calc_iq__vdx0_dn7) - (-(assign15330_e15512 * locals.var_fn169_calc_iq__ffd0_dn7))) / locals.var_fn169_calc_iq__two_n_phit0), (((-locals.var_fn169_calc_iq__vdx0_dn9) - (-(assign15330_e15512 * locals.var_fn169_calc_iq__ffd0_dn9))) / locals.var_fn169_calc_iq__two_n_phit0), (((locals.var_fn169_calc_iq__vgsin_dn10 - locals.var_fn169_calc_iq__vdx0_dn10) - (-(assign15330_e15512 * locals.var_fn169_calc_iq__ffd0_dn10))) / locals.var_fn169_calc_iq__two_n_phit0),)
    } else {
        (locals.var_fn169_calc_iq__etad0, locals.var_fn169_calc_iq__etad0_dn2, locals.var_fn169_calc_iq__etad0_dn4, locals.var_fn169_calc_iq__etad0_dn7, locals.var_fn169_calc_iq__etad0_dn9, locals.var_fn169_calc_iq__etad0_dn10,)
    }
};
        locals.var_fn169_calc_iq__etad0 = assign15330_e15520;
        locals.var_fn169_calc_iq__etad0_dn2 = assign15330_e15520_d_n2;
        locals.var_fn169_calc_iq__etad0_dn4 = assign15330_e15520_d_n4;
        locals.var_fn169_calc_iq__etad0_dn7 = assign15330_e15520_d_n7;
        locals.var_fn169_calc_iq__etad0_dn9 = assign15330_e15520_d_n9;
        locals.var_fn169_calc_iq__etad0_dn10 = assign15330_e15520_d_n10;

        let assign15340_e15523: f64 = if locals.var_fn169_calc_iq__etad0 > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard193 = assign15340_e15523;

        let (assign15350_e15531, assign15350_e15531_d_n2, assign15350_e15531_d_n4, assign15350_e15531_d_n7, assign15350_e15531_d_n9, assign15350_e15531_d_n10,) = {
    if ((locals.var_guard168 != 0.0) && (locals.var_guard193 != 0.0)) {
        let assign15350_e15529: f64 = (locals.var_fn169_calc_iq__qref0 * locals.var_fn169_calc_iq__etad0);
        (assign15350_e15529, (locals.var_fn169_calc_iq__qref0 * locals.var_fn169_calc_iq__etad0_dn2), ((locals.var_fn169_calc_iq__qref0_dn4 * locals.var_fn169_calc_iq__etad0) + (locals.var_fn169_calc_iq__qref0 * locals.var_fn169_calc_iq__etad0_dn4)), (locals.var_fn169_calc_iq__qref0 * locals.var_fn169_calc_iq__etad0_dn7), (locals.var_fn169_calc_iq__qref0 * locals.var_fn169_calc_iq__etad0_dn9), (locals.var_fn169_calc_iq__qref0 * locals.var_fn169_calc_iq__etad0_dn10),)
    } else {
        (locals.var_fn169_calc_iq__qinvd0, locals.var_fn169_calc_iq__qinvd0_dn2, locals.var_fn169_calc_iq__qinvd0_dn4, locals.var_fn169_calc_iq__qinvd0_dn7, locals.var_fn169_calc_iq__qinvd0_dn9, locals.var_fn169_calc_iq__qinvd0_dn10,)
    }
};
        locals.var_fn169_calc_iq__qinvd0 = assign15350_e15531;
        locals.var_fn169_calc_iq__qinvd0_dn2 = assign15350_e15531_d_n2;
        locals.var_fn169_calc_iq__qinvd0_dn4 = assign15350_e15531_d_n4;
        locals.var_fn169_calc_iq__qinvd0_dn7 = assign15350_e15531_d_n7;
        locals.var_fn169_calc_iq__qinvd0_dn9 = assign15350_e15531_d_n9;
        locals.var_fn169_calc_iq__qinvd0_dn10 = assign15350_e15531_d_n10;

        let assign15360_e15534: f64 = (-50.0);
        let assign15360_e15535: f64 = if locals.var_fn169_calc_iq__etad0 < assign15360_e15534 { 1.0 } else { 0.0 };
        locals.var_guard194 = assign15360_e15535;

        let (assign15370_e15547, assign15370_e15547_d_n2, assign15370_e15547_d_n4, assign15370_e15547_d_n7, assign15370_e15547_d_n9, assign15370_e15547_d_n10,) = {
    if (((locals.var_guard168 != 0.0) && (locals.var_guard193 == 0.0)) && (locals.var_guard194 != 0.0)) {
        let assign15370_e15544: f64 = (locals.var_fn169_calc_iq__etad0).exp();
        let assign15370_e15545: f64 = (locals.var_fn169_calc_iq__qref0 * assign15370_e15544);
        (assign15370_e15545, (locals.var_fn169_calc_iq__qref0 * (assign15370_e15544 * locals.var_fn169_calc_iq__etad0_dn2)), ((locals.var_fn169_calc_iq__qref0_dn4 * assign15370_e15544) + (locals.var_fn169_calc_iq__qref0 * (assign15370_e15544 * locals.var_fn169_calc_iq__etad0_dn4))), (locals.var_fn169_calc_iq__qref0 * (assign15370_e15544 * locals.var_fn169_calc_iq__etad0_dn7)), (locals.var_fn169_calc_iq__qref0 * (assign15370_e15544 * locals.var_fn169_calc_iq__etad0_dn9)), (locals.var_fn169_calc_iq__qref0 * (assign15370_e15544 * locals.var_fn169_calc_iq__etad0_dn10)),)
    } else {
        (locals.var_fn169_calc_iq__qinvd0, locals.var_fn169_calc_iq__qinvd0_dn2, locals.var_fn169_calc_iq__qinvd0_dn4, locals.var_fn169_calc_iq__qinvd0_dn7, locals.var_fn169_calc_iq__qinvd0_dn9, locals.var_fn169_calc_iq__qinvd0_dn10,)
    }
};
        locals.var_fn169_calc_iq__qinvd0 = assign15370_e15547;
        locals.var_fn169_calc_iq__qinvd0_dn2 = assign15370_e15547_d_n2;
        locals.var_fn169_calc_iq__qinvd0_dn4 = assign15370_e15547_d_n4;
        locals.var_fn169_calc_iq__qinvd0_dn7 = assign15370_e15547_d_n7;
        locals.var_fn169_calc_iq__qinvd0_dn9 = assign15370_e15547_d_n9;
        locals.var_fn169_calc_iq__qinvd0_dn10 = assign15370_e15547_d_n10;

        let (assign15380_e15563, assign15380_e15563_d_n2, assign15380_e15563_d_n4, assign15380_e15563_d_n7, assign15380_e15563_d_n9, assign15380_e15563_d_n10,) = {
    if (((locals.var_guard168 != 0.0) && (locals.var_guard193 == 0.0)) && (locals.var_guard194 == 0.0)) {
        let assign15380_e15558: f64 = (locals.var_fn169_calc_iq__etad0).exp();
        let assign15380_e15559: f64 = (1.0 + assign15380_e15558);
        let assign15380_e15560: f64 = (assign15380_e15559).ln();
        let assign15380_e15561: f64 = (locals.var_fn169_calc_iq__qref0 * assign15380_e15560);
        (assign15380_e15561, (locals.var_fn169_calc_iq__qref0 * ((assign15380_e15558 * locals.var_fn169_calc_iq__etad0_dn2) / assign15380_e15559)), ((locals.var_fn169_calc_iq__qref0_dn4 * assign15380_e15560) + (locals.var_fn169_calc_iq__qref0 * ((assign15380_e15558 * locals.var_fn169_calc_iq__etad0_dn4) / assign15380_e15559))), (locals.var_fn169_calc_iq__qref0 * ((assign15380_e15558 * locals.var_fn169_calc_iq__etad0_dn7) / assign15380_e15559)), (locals.var_fn169_calc_iq__qref0 * ((assign15380_e15558 * locals.var_fn169_calc_iq__etad0_dn9) / assign15380_e15559)), (locals.var_fn169_calc_iq__qref0 * ((assign15380_e15558 * locals.var_fn169_calc_iq__etad0_dn10) / assign15380_e15559)),)
    } else {
        (locals.var_fn169_calc_iq__qinvd0, locals.var_fn169_calc_iq__qinvd0_dn2, locals.var_fn169_calc_iq__qinvd0_dn4, locals.var_fn169_calc_iq__qinvd0_dn7, locals.var_fn169_calc_iq__qinvd0_dn9, locals.var_fn169_calc_iq__qinvd0_dn10,)
    }
};
        locals.var_fn169_calc_iq__qinvd0 = assign15380_e15563;
        locals.var_fn169_calc_iq__qinvd0_dn2 = assign15380_e15563_d_n2;
        locals.var_fn169_calc_iq__qinvd0_dn4 = assign15380_e15563_d_n4;
        locals.var_fn169_calc_iq__qinvd0_dn7 = assign15380_e15563_d_n7;
        locals.var_fn169_calc_iq__qinvd0_dn9 = assign15380_e15563_d_n9;
        locals.var_fn169_calc_iq__qinvd0_dn10 = assign15380_e15563_d_n10;

        let (assign15390_e15571, assign15390_e15571_d_n2, assign15390_e15571_d_n4, assign15390_e15571_d_n7, assign15390_e15571_d_n9, assign15390_e15571_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        let assign15390_e15567: f64 = (locals.var_fn169_calc_iq__qinvs0 * locals.var_fn169_calc_iq__qinvs0);
        let assign15390_e15569: f64 = (assign15390_e15567 + 1e-38);
        (assign15390_e15569, ((locals.var_fn169_calc_iq__qinvs0_dn2 * locals.var_fn169_calc_iq__qinvs0) + (locals.var_fn169_calc_iq__qinvs0 * locals.var_fn169_calc_iq__qinvs0_dn2)), ((locals.var_fn169_calc_iq__qinvs0_dn4 * locals.var_fn169_calc_iq__qinvs0) + (locals.var_fn169_calc_iq__qinvs0 * locals.var_fn169_calc_iq__qinvs0_dn4)), ((locals.var_fn169_calc_iq__qinvs0_dn7 * locals.var_fn169_calc_iq__qinvs0) + (locals.var_fn169_calc_iq__qinvs0 * locals.var_fn169_calc_iq__qinvs0_dn7)), ((locals.var_fn169_calc_iq__qinvs0_dn9 * locals.var_fn169_calc_iq__qinvs0) + (locals.var_fn169_calc_iq__qinvs0 * locals.var_fn169_calc_iq__qinvs0_dn9)), ((locals.var_fn169_calc_iq__qinvs0_dn10 * locals.var_fn169_calc_iq__qinvs0) + (locals.var_fn169_calc_iq__qinvs0 * locals.var_fn169_calc_iq__qinvs0_dn10)),)
    } else {
        (locals.var_fn169_calc_iq__qs2, locals.var_fn169_calc_iq__qs2_dn2, locals.var_fn169_calc_iq__qs2_dn4, locals.var_fn169_calc_iq__qs2_dn7, locals.var_fn169_calc_iq__qs2_dn9, locals.var_fn169_calc_iq__qs2_dn10,)
    }
};
        locals.var_fn169_calc_iq__qs2 = assign15390_e15571;
        locals.var_fn169_calc_iq__qs2_dn2 = assign15390_e15571_d_n2;
        locals.var_fn169_calc_iq__qs2_dn4 = assign15390_e15571_d_n4;
        locals.var_fn169_calc_iq__qs2_dn7 = assign15390_e15571_d_n7;
        locals.var_fn169_calc_iq__qs2_dn9 = assign15390_e15571_d_n9;
        locals.var_fn169_calc_iq__qs2_dn10 = assign15390_e15571_d_n10;

        let (assign15400_e15579, assign15400_e15579_d_n2, assign15400_e15579_d_n4, assign15400_e15579_d_n7, assign15400_e15579_d_n9, assign15400_e15579_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        let assign15400_e15575: f64 = (locals.var_fn169_calc_iq__qs2 * locals.var_fn169_calc_iq__qinvs0);
        let assign15400_e15577: f64 = (assign15400_e15575 + 1e-57);
        (assign15400_e15577, ((locals.var_fn169_calc_iq__qs2_dn2 * locals.var_fn169_calc_iq__qinvs0) + (locals.var_fn169_calc_iq__qs2 * locals.var_fn169_calc_iq__qinvs0_dn2)), ((locals.var_fn169_calc_iq__qs2_dn4 * locals.var_fn169_calc_iq__qinvs0) + (locals.var_fn169_calc_iq__qs2 * locals.var_fn169_calc_iq__qinvs0_dn4)), ((locals.var_fn169_calc_iq__qs2_dn7 * locals.var_fn169_calc_iq__qinvs0) + (locals.var_fn169_calc_iq__qs2 * locals.var_fn169_calc_iq__qinvs0_dn7)), ((locals.var_fn169_calc_iq__qs2_dn9 * locals.var_fn169_calc_iq__qinvs0) + (locals.var_fn169_calc_iq__qs2 * locals.var_fn169_calc_iq__qinvs0_dn9)), ((locals.var_fn169_calc_iq__qs2_dn10 * locals.var_fn169_calc_iq__qinvs0) + (locals.var_fn169_calc_iq__qs2 * locals.var_fn169_calc_iq__qinvs0_dn10)),)
    } else {
        (locals.var_fn169_calc_iq__qs3, locals.var_fn169_calc_iq__qs3_dn2, locals.var_fn169_calc_iq__qs3_dn4, locals.var_fn169_calc_iq__qs3_dn7, locals.var_fn169_calc_iq__qs3_dn9, locals.var_fn169_calc_iq__qs3_dn10,)
    }
};
        locals.var_fn169_calc_iq__qs3 = assign15400_e15579;
        locals.var_fn169_calc_iq__qs3_dn2 = assign15400_e15579_d_n2;
        locals.var_fn169_calc_iq__qs3_dn4 = assign15400_e15579_d_n4;
        locals.var_fn169_calc_iq__qs3_dn7 = assign15400_e15579_d_n7;
        locals.var_fn169_calc_iq__qs3_dn9 = assign15400_e15579_d_n9;
        locals.var_fn169_calc_iq__qs3_dn10 = assign15400_e15579_d_n10;

        let (assign15410_e15587, assign15410_e15587_d_n2, assign15410_e15587_d_n4, assign15410_e15587_d_n7, assign15410_e15587_d_n9, assign15410_e15587_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        let assign15410_e15583: f64 = (locals.var_fn169_calc_iq__qinvd0 * locals.var_fn169_calc_iq__qinvd0);
        let assign15410_e15585: f64 = (assign15410_e15583 + 1e-38);
        (assign15410_e15585, ((locals.var_fn169_calc_iq__qinvd0_dn2 * locals.var_fn169_calc_iq__qinvd0) + (locals.var_fn169_calc_iq__qinvd0 * locals.var_fn169_calc_iq__qinvd0_dn2)), ((locals.var_fn169_calc_iq__qinvd0_dn4 * locals.var_fn169_calc_iq__qinvd0) + (locals.var_fn169_calc_iq__qinvd0 * locals.var_fn169_calc_iq__qinvd0_dn4)), ((locals.var_fn169_calc_iq__qinvd0_dn7 * locals.var_fn169_calc_iq__qinvd0) + (locals.var_fn169_calc_iq__qinvd0 * locals.var_fn169_calc_iq__qinvd0_dn7)), ((locals.var_fn169_calc_iq__qinvd0_dn9 * locals.var_fn169_calc_iq__qinvd0) + (locals.var_fn169_calc_iq__qinvd0 * locals.var_fn169_calc_iq__qinvd0_dn9)), ((locals.var_fn169_calc_iq__qinvd0_dn10 * locals.var_fn169_calc_iq__qinvd0) + (locals.var_fn169_calc_iq__qinvd0 * locals.var_fn169_calc_iq__qinvd0_dn10)),)
    } else {
        (locals.var_fn169_calc_iq__qd2, locals.var_fn169_calc_iq__qd2_dn2, locals.var_fn169_calc_iq__qd2_dn4, locals.var_fn169_calc_iq__qd2_dn7, locals.var_fn169_calc_iq__qd2_dn9, locals.var_fn169_calc_iq__qd2_dn10,)
    }
};
        locals.var_fn169_calc_iq__qd2 = assign15410_e15587;
        locals.var_fn169_calc_iq__qd2_dn2 = assign15410_e15587_d_n2;
        locals.var_fn169_calc_iq__qd2_dn4 = assign15410_e15587_d_n4;
        locals.var_fn169_calc_iq__qd2_dn7 = assign15410_e15587_d_n7;
        locals.var_fn169_calc_iq__qd2_dn9 = assign15410_e15587_d_n9;
        locals.var_fn169_calc_iq__qd2_dn10 = assign15410_e15587_d_n10;

        let (assign15420_e15595, assign15420_e15595_d_n2, assign15420_e15595_d_n4, assign15420_e15595_d_n7, assign15420_e15595_d_n9, assign15420_e15595_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        let assign15420_e15591: f64 = (locals.var_fn169_calc_iq__qd2 * locals.var_fn169_calc_iq__qinvd0);
        let assign15420_e15593: f64 = (assign15420_e15591 + 1e-57);
        (assign15420_e15593, ((locals.var_fn169_calc_iq__qd2_dn2 * locals.var_fn169_calc_iq__qinvd0) + (locals.var_fn169_calc_iq__qd2 * locals.var_fn169_calc_iq__qinvd0_dn2)), ((locals.var_fn169_calc_iq__qd2_dn4 * locals.var_fn169_calc_iq__qinvd0) + (locals.var_fn169_calc_iq__qd2 * locals.var_fn169_calc_iq__qinvd0_dn4)), ((locals.var_fn169_calc_iq__qd2_dn7 * locals.var_fn169_calc_iq__qinvd0) + (locals.var_fn169_calc_iq__qd2 * locals.var_fn169_calc_iq__qinvd0_dn7)), ((locals.var_fn169_calc_iq__qd2_dn9 * locals.var_fn169_calc_iq__qinvd0) + (locals.var_fn169_calc_iq__qd2 * locals.var_fn169_calc_iq__qinvd0_dn9)), ((locals.var_fn169_calc_iq__qd2_dn10 * locals.var_fn169_calc_iq__qinvd0) + (locals.var_fn169_calc_iq__qd2 * locals.var_fn169_calc_iq__qinvd0_dn10)),)
    } else {
        (locals.var_fn169_calc_iq__qd3, locals.var_fn169_calc_iq__qd3_dn2, locals.var_fn169_calc_iq__qd3_dn4, locals.var_fn169_calc_iq__qd3_dn7, locals.var_fn169_calc_iq__qd3_dn9, locals.var_fn169_calc_iq__qd3_dn10,)
    }
};
        locals.var_fn169_calc_iq__qd3 = assign15420_e15595;
        locals.var_fn169_calc_iq__qd3_dn2 = assign15420_e15595_d_n2;
        locals.var_fn169_calc_iq__qd3_dn4 = assign15420_e15595_d_n4;
        locals.var_fn169_calc_iq__qd3_dn7 = assign15420_e15595_d_n7;
        locals.var_fn169_calc_iq__qd3_dn9 = assign15420_e15595_d_n9;
        locals.var_fn169_calc_iq__qd3_dn10 = assign15420_e15595_d_n10;

        let (assign15430_e15603, assign15430_e15603_d_n2, assign15430_e15603_d_n4, assign15430_e15603_d_n7, assign15430_e15603_d_n9, assign15430_e15603_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        let assign15430_e15599: f64 = (locals.var_fn169_calc_iq__qinvs0 * locals.var_fn169_calc_iq__qinvd0);
        let assign15430_e15601: f64 = (assign15430_e15599 + 1e-38);
        (assign15430_e15601, ((locals.var_fn169_calc_iq__qinvs0_dn2 * locals.var_fn169_calc_iq__qinvd0) + (locals.var_fn169_calc_iq__qinvs0 * locals.var_fn169_calc_iq__qinvd0_dn2)), ((locals.var_fn169_calc_iq__qinvs0_dn4 * locals.var_fn169_calc_iq__qinvd0) + (locals.var_fn169_calc_iq__qinvs0 * locals.var_fn169_calc_iq__qinvd0_dn4)), ((locals.var_fn169_calc_iq__qinvs0_dn7 * locals.var_fn169_calc_iq__qinvd0) + (locals.var_fn169_calc_iq__qinvs0 * locals.var_fn169_calc_iq__qinvd0_dn7)), ((locals.var_fn169_calc_iq__qinvs0_dn9 * locals.var_fn169_calc_iq__qinvd0) + (locals.var_fn169_calc_iq__qinvs0 * locals.var_fn169_calc_iq__qinvd0_dn9)), ((locals.var_fn169_calc_iq__qinvs0_dn10 * locals.var_fn169_calc_iq__qinvd0) + (locals.var_fn169_calc_iq__qinvs0 * locals.var_fn169_calc_iq__qinvd0_dn10)),)
    } else {
        (locals.var_fn169_calc_iq__qsqd, locals.var_fn169_calc_iq__qsqd_dn2, locals.var_fn169_calc_iq__qsqd_dn4, locals.var_fn169_calc_iq__qsqd_dn7, locals.var_fn169_calc_iq__qsqd_dn9, locals.var_fn169_calc_iq__qsqd_dn10,)
    }
};
        locals.var_fn169_calc_iq__qsqd = assign15430_e15603;
        locals.var_fn169_calc_iq__qsqd_dn2 = assign15430_e15603_d_n2;
        locals.var_fn169_calc_iq__qsqd_dn4 = assign15430_e15603_d_n4;
        locals.var_fn169_calc_iq__qsqd_dn7 = assign15430_e15603_d_n7;
        locals.var_fn169_calc_iq__qsqd_dn9 = assign15430_e15603_d_n9;
        locals.var_fn169_calc_iq__qsqd_dn10 = assign15430_e15603_d_n10;

        let (assign15440_e15621, assign15440_e15621_d_n2, assign15440_e15621_d_n4, assign15440_e15621_d_n7, assign15440_e15621_d_n9, assign15440_e15621_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        let assign15440_e15607: f64 = (2.0 / 3.0);
        let assign15440_e15610: f64 = (locals.var_fn169_calc_iq__qs2 + locals.var_fn169_calc_iq__qd2);
        let assign15440_e15612: f64 = (assign15440_e15610 + locals.var_fn169_calc_iq__qsqd);
        let assign15440_e15613: f64 = (assign15440_e15607 * assign15440_e15612);
        let assign15440_e15616: f64 = (locals.var_fn169_calc_iq__qinvs0 + locals.var_fn169_calc_iq__qinvd0);
        let assign15440_e15618: f64 = (assign15440_e15616 + 2e-19);
        let assign15440_e15619: f64 = (assign15440_e15613 / assign15440_e15618);
        (assign15440_e15619, ((((assign15440_e15607 * ((locals.var_fn169_calc_iq__qs2_dn2 + locals.var_fn169_calc_iq__qd2_dn2) + locals.var_fn169_calc_iq__qsqd_dn2)) * assign15440_e15618) - (assign15440_e15613 * (locals.var_fn169_calc_iq__qinvs0_dn2 + locals.var_fn169_calc_iq__qinvd0_dn2))) / (assign15440_e15618 * assign15440_e15618)), ((((assign15440_e15607 * ((locals.var_fn169_calc_iq__qs2_dn4 + locals.var_fn169_calc_iq__qd2_dn4) + locals.var_fn169_calc_iq__qsqd_dn4)) * assign15440_e15618) - (assign15440_e15613 * (locals.var_fn169_calc_iq__qinvs0_dn4 + locals.var_fn169_calc_iq__qinvd0_dn4))) / (assign15440_e15618 * assign15440_e15618)), ((((assign15440_e15607 * ((locals.var_fn169_calc_iq__qs2_dn7 + locals.var_fn169_calc_iq__qd2_dn7) + locals.var_fn169_calc_iq__qsqd_dn7)) * assign15440_e15618) - (assign15440_e15613 * (locals.var_fn169_calc_iq__qinvs0_dn7 + locals.var_fn169_calc_iq__qinvd0_dn7))) / (assign15440_e15618 * assign15440_e15618)), ((((assign15440_e15607 * ((locals.var_fn169_calc_iq__qs2_dn9 + locals.var_fn169_calc_iq__qd2_dn9) + locals.var_fn169_calc_iq__qsqd_dn9)) * assign15440_e15618) - (assign15440_e15613 * (locals.var_fn169_calc_iq__qinvs0_dn9 + locals.var_fn169_calc_iq__qinvd0_dn9))) / (assign15440_e15618 * assign15440_e15618)), ((((assign15440_e15607 * ((locals.var_fn169_calc_iq__qs2_dn10 + locals.var_fn169_calc_iq__qd2_dn10) + locals.var_fn169_calc_iq__qsqd_dn10)) * assign15440_e15618) - (assign15440_e15613 * (locals.var_fn169_calc_iq__qinvs0_dn10 + locals.var_fn169_calc_iq__qinvd0_dn10))) / (assign15440_e15618 * assign15440_e15618)),)
    } else {
        (locals.var_fn169_calc_iq__qinvdd, locals.var_fn169_calc_iq__qinvdd_dn2, locals.var_fn169_calc_iq__qinvdd_dn4, locals.var_fn169_calc_iq__qinvdd_dn7, locals.var_fn169_calc_iq__qinvdd_dn9, locals.var_fn169_calc_iq__qinvdd_dn10,)
    }
};
        locals.var_fn169_calc_iq__qinvdd = assign15440_e15621;
        locals.var_fn169_calc_iq__qinvdd_dn2 = assign15440_e15621_d_n2;
        locals.var_fn169_calc_iq__qinvdd_dn4 = assign15440_e15621_d_n4;
        locals.var_fn169_calc_iq__qinvdd_dn7 = assign15440_e15621_d_n7;
        locals.var_fn169_calc_iq__qinvdd_dn9 = assign15440_e15621_d_n9;
        locals.var_fn169_calc_iq__qinvdd_dn10 = assign15440_e15621_d_n10;

        let (assign15450_e15655, assign15450_e15655_d_n2, assign15450_e15655_d_n4, assign15450_e15655_d_n7, assign15450_e15655_d_n9, assign15450_e15655_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        let assign15450_e15626: f64 = (2.0 * locals.var_fn169_calc_iq__qs3);
        let assign15450_e15629: f64 = (3.0 * locals.var_fn169_calc_iq__qd3);
        let assign15450_e15630: f64 = (assign15450_e15626 + assign15450_e15629);
        let assign15450_e15633: f64 = (4.0 * locals.var_fn169_calc_iq__qs2);
        let assign15450_e15635: f64 = (assign15450_e15633 * locals.var_fn169_calc_iq__qinvd0);
        let assign15450_e15636: f64 = (assign15450_e15630 + assign15450_e15635);
        let assign15450_e15639: f64 = (6.0 * locals.var_fn169_calc_iq__qd2);
        let assign15450_e15641: f64 = (assign15450_e15639 * locals.var_fn169_calc_iq__qinvs0);
        let assign15450_e15642: f64 = (assign15450_e15636 + assign15450_e15641);
        let assign15450_e15643: f64 = (2.0 * assign15450_e15642);
        let assign15450_e15647: f64 = (locals.var_fn169_calc_iq__qs2 + locals.var_fn169_calc_iq__qd2);
        let assign15450_e15650: f64 = (2.0 * locals.var_fn169_calc_iq__qsqd);
        let assign15450_e15651: f64 = (assign15450_e15647 + assign15450_e15650);
        let assign15450_e15652: f64 = (15.0 * assign15450_e15651);
        let assign15450_e15653: f64 = (assign15450_e15643 / assign15450_e15652);
        (assign15450_e15653, ((((2.0 * ((((2.0 * locals.var_fn169_calc_iq__qs3_dn2) + (3.0 * locals.var_fn169_calc_iq__qd3_dn2)) + (((4.0 * locals.var_fn169_calc_iq__qs2_dn2) * locals.var_fn169_calc_iq__qinvd0) + (assign15450_e15633 * locals.var_fn169_calc_iq__qinvd0_dn2))) + (((6.0 * locals.var_fn169_calc_iq__qd2_dn2) * locals.var_fn169_calc_iq__qinvs0) + (assign15450_e15639 * locals.var_fn169_calc_iq__qinvs0_dn2)))) * assign15450_e15652) - (assign15450_e15643 * (15.0 * ((locals.var_fn169_calc_iq__qs2_dn2 + locals.var_fn169_calc_iq__qd2_dn2) + (2.0 * locals.var_fn169_calc_iq__qsqd_dn2))))) / (assign15450_e15652 * assign15450_e15652)), ((((2.0 * ((((2.0 * locals.var_fn169_calc_iq__qs3_dn4) + (3.0 * locals.var_fn169_calc_iq__qd3_dn4)) + (((4.0 * locals.var_fn169_calc_iq__qs2_dn4) * locals.var_fn169_calc_iq__qinvd0) + (assign15450_e15633 * locals.var_fn169_calc_iq__qinvd0_dn4))) + (((6.0 * locals.var_fn169_calc_iq__qd2_dn4) * locals.var_fn169_calc_iq__qinvs0) + (assign15450_e15639 * locals.var_fn169_calc_iq__qinvs0_dn4)))) * assign15450_e15652) - (assign15450_e15643 * (15.0 * ((locals.var_fn169_calc_iq__qs2_dn4 + locals.var_fn169_calc_iq__qd2_dn4) + (2.0 * locals.var_fn169_calc_iq__qsqd_dn4))))) / (assign15450_e15652 * assign15450_e15652)), ((((2.0 * ((((2.0 * locals.var_fn169_calc_iq__qs3_dn7) + (3.0 * locals.var_fn169_calc_iq__qd3_dn7)) + (((4.0 * locals.var_fn169_calc_iq__qs2_dn7) * locals.var_fn169_calc_iq__qinvd0) + (assign15450_e15633 * locals.var_fn169_calc_iq__qinvd0_dn7))) + (((6.0 * locals.var_fn169_calc_iq__qd2_dn7) * locals.var_fn169_calc_iq__qinvs0) + (assign15450_e15639 * locals.var_fn169_calc_iq__qinvs0_dn7)))) * assign15450_e15652) - (assign15450_e15643 * (15.0 * ((locals.var_fn169_calc_iq__qs2_dn7 + locals.var_fn169_calc_iq__qd2_dn7) + (2.0 * locals.var_fn169_calc_iq__qsqd_dn7))))) / (assign15450_e15652 * assign15450_e15652)), ((((2.0 * ((((2.0 * locals.var_fn169_calc_iq__qs3_dn9) + (3.0 * locals.var_fn169_calc_iq__qd3_dn9)) + (((4.0 * locals.var_fn169_calc_iq__qs2_dn9) * locals.var_fn169_calc_iq__qinvd0) + (assign15450_e15633 * locals.var_fn169_calc_iq__qinvd0_dn9))) + (((6.0 * locals.var_fn169_calc_iq__qd2_dn9) * locals.var_fn169_calc_iq__qinvs0) + (assign15450_e15639 * locals.var_fn169_calc_iq__qinvs0_dn9)))) * assign15450_e15652) - (assign15450_e15643 * (15.0 * ((locals.var_fn169_calc_iq__qs2_dn9 + locals.var_fn169_calc_iq__qd2_dn9) + (2.0 * locals.var_fn169_calc_iq__qsqd_dn9))))) / (assign15450_e15652 * assign15450_e15652)), ((((2.0 * ((((2.0 * locals.var_fn169_calc_iq__qs3_dn10) + (3.0 * locals.var_fn169_calc_iq__qd3_dn10)) + (((4.0 * locals.var_fn169_calc_iq__qs2_dn10) * locals.var_fn169_calc_iq__qinvd0) + (assign15450_e15633 * locals.var_fn169_calc_iq__qinvd0_dn10))) + (((6.0 * locals.var_fn169_calc_iq__qd2_dn10) * locals.var_fn169_calc_iq__qinvs0) + (assign15450_e15639 * locals.var_fn169_calc_iq__qinvs0_dn10)))) * assign15450_e15652) - (assign15450_e15643 * (15.0 * ((locals.var_fn169_calc_iq__qs2_dn10 + locals.var_fn169_calc_iq__qd2_dn10) + (2.0 * locals.var_fn169_calc_iq__qsqd_dn10))))) / (assign15450_e15652 * assign15450_e15652)),)
    } else {
        (locals.var_fn169_calc_iq__qd1, locals.var_fn169_calc_iq__qd1_dn2, locals.var_fn169_calc_iq__qd1_dn4, locals.var_fn169_calc_iq__qd1_dn7, locals.var_fn169_calc_iq__qd1_dn9, locals.var_fn169_calc_iq__qd1_dn10,)
    }
};
        locals.var_fn169_calc_iq__qd1 = assign15450_e15655;
        locals.var_fn169_calc_iq__qd1_dn2 = assign15450_e15655_d_n2;
        locals.var_fn169_calc_iq__qd1_dn4 = assign15450_e15655_d_n4;
        locals.var_fn169_calc_iq__qd1_dn7 = assign15450_e15655_d_n7;
        locals.var_fn169_calc_iq__qd1_dn9 = assign15450_e15655_d_n9;
        locals.var_fn169_calc_iq__qd1_dn10 = assign15450_e15655_d_n10;

        let (assign15460_e15661, assign15460_e15661_d_n2, assign15460_e15661_d_n4, assign15460_e15661_d_n7, assign15460_e15661_d_n9, assign15460_e15661_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        let assign15460_e15659: f64 = (locals.var_fn169_calc_iq__qinvdd - locals.var_fn169_calc_iq__qd1);
        (assign15460_e15659, (locals.var_fn169_calc_iq__qinvdd_dn2 - locals.var_fn169_calc_iq__qd1_dn2), (locals.var_fn169_calc_iq__qinvdd_dn4 - locals.var_fn169_calc_iq__qd1_dn4), (locals.var_fn169_calc_iq__qinvdd_dn7 - locals.var_fn169_calc_iq__qd1_dn7), (locals.var_fn169_calc_iq__qinvdd_dn9 - locals.var_fn169_calc_iq__qd1_dn9), (locals.var_fn169_calc_iq__qinvdd_dn10 - locals.var_fn169_calc_iq__qd1_dn10),)
    } else {
        (locals.var_fn169_calc_iq__qs, locals.var_fn169_calc_iq__qs_dn2, locals.var_fn169_calc_iq__qs_dn4, locals.var_fn169_calc_iq__qs_dn7, locals.var_fn169_calc_iq__qs_dn9, locals.var_fn169_calc_iq__qs_dn10,)
    }
};
        locals.var_fn169_calc_iq__qs = assign15460_e15661;
        locals.var_fn169_calc_iq__qs_dn2 = assign15460_e15661_d_n2;
        locals.var_fn169_calc_iq__qs_dn4 = assign15460_e15661_d_n4;
        locals.var_fn169_calc_iq__qs_dn7 = assign15460_e15661_d_n7;
        locals.var_fn169_calc_iq__qs_dn9 = assign15460_e15661_d_n9;
        locals.var_fn169_calc_iq__qs_dn10 = assign15460_e15661_d_n10;

        let (assign15470_e15665, assign15470_e15665_d_n2, assign15470_e15665_d_n4, assign15470_e15665_d_n7, assign15470_e15665_d_n9, assign15470_e15665_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        (locals.var_fn169_calc_iq__qd1, locals.var_fn169_calc_iq__qd1_dn2, locals.var_fn169_calc_iq__qd1_dn4, locals.var_fn169_calc_iq__qd1_dn7, locals.var_fn169_calc_iq__qd1_dn9, locals.var_fn169_calc_iq__qd1_dn10,)
    } else {
        (locals.var_fn169_calc_iq__qd, locals.var_fn169_calc_iq__qd_dn2, locals.var_fn169_calc_iq__qd_dn4, locals.var_fn169_calc_iq__qd_dn7, locals.var_fn169_calc_iq__qd_dn9, locals.var_fn169_calc_iq__qd_dn10,)
    }
};
        locals.var_fn169_calc_iq__qd = assign15470_e15665;
        locals.var_fn169_calc_iq__qd_dn2 = assign15470_e15665_d_n2;
        locals.var_fn169_calc_iq__qd_dn4 = assign15470_e15665_d_n4;
        locals.var_fn169_calc_iq__qd_dn7 = assign15470_e15665_d_n7;
        locals.var_fn169_calc_iq__qd_dn9 = assign15470_e15665_d_n9;
        locals.var_fn169_calc_iq__qd_dn10 = assign15470_e15665_d_n10;

        let (assign15480_e15679, assign15480_e15679_d_n2, assign15480_e15679_d_n4, assign15480_e15679_d_n7, assign15480_e15679_d_n9, assign15480_e15679_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        let assign15480_e15669: f64 = (locals.var_fn169_calc_iq__w * locals.var_fn169_calc_iq__ngf);
        let assign15480_e15671: f64 = (assign15480_e15669 * locals.var_fn169_calc_iq__lin);
        let assign15480_e15673: f64 = (assign15480_e15671 * locals.var_fn169_calc_iq__type);
        let assign15480_e15675: f64 = (assign15480_e15673 * locals.var_fn169_calc_iq__qs);
        let assign15480_e15677: f64 = (assign15480_e15675 * locals.var_fn169_calc_iq__trapfracdl);
        (assign15480_e15677, ((assign15480_e15673 * locals.var_fn169_calc_iq__qs_dn2) * locals.var_fn169_calc_iq__trapfracdl), ((assign15480_e15673 * locals.var_fn169_calc_iq__qs_dn4) * locals.var_fn169_calc_iq__trapfracdl), ((assign15480_e15673 * locals.var_fn169_calc_iq__qs_dn7) * locals.var_fn169_calc_iq__trapfracdl), ((assign15480_e15673 * locals.var_fn169_calc_iq__qs_dn9) * locals.var_fn169_calc_iq__trapfracdl), ((assign15480_e15673 * locals.var_fn169_calc_iq__qs_dn10) * locals.var_fn169_calc_iq__trapfracdl),)
    } else {
        (locals.var_fn169_calc_iq__qgsout, locals.var_fn169_calc_iq__qgsout_dn2, locals.var_fn169_calc_iq__qgsout_dn4, locals.var_fn169_calc_iq__qgsout_dn7, locals.var_fn169_calc_iq__qgsout_dn9, locals.var_fn169_calc_iq__qgsout_dn10,)
    }
};
        locals.var_fn169_calc_iq__qgsout = assign15480_e15679;
        locals.var_fn169_calc_iq__qgsout_dn2 = assign15480_e15679_d_n2;
        locals.var_fn169_calc_iq__qgsout_dn4 = assign15480_e15679_d_n4;
        locals.var_fn169_calc_iq__qgsout_dn7 = assign15480_e15679_d_n7;
        locals.var_fn169_calc_iq__qgsout_dn9 = assign15480_e15679_d_n9;
        locals.var_fn169_calc_iq__qgsout_dn10 = assign15480_e15679_d_n10;

        let (assign15490_e15693, assign15490_e15693_d_n2, assign15490_e15693_d_n4, assign15490_e15693_d_n7, assign15490_e15693_d_n9, assign15490_e15693_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        let assign15490_e15683: f64 = (locals.var_fn169_calc_iq__w * locals.var_fn169_calc_iq__ngf);
        let assign15490_e15685: f64 = (assign15490_e15683 * locals.var_fn169_calc_iq__lin);
        let assign15490_e15687: f64 = (assign15490_e15685 * locals.var_fn169_calc_iq__type);
        let assign15490_e15689: f64 = (assign15490_e15687 * locals.var_fn169_calc_iq__qd);
        let assign15490_e15691: f64 = (assign15490_e15689 * locals.var_fn169_calc_iq__trapfracdl);
        (assign15490_e15691, ((assign15490_e15687 * locals.var_fn169_calc_iq__qd_dn2) * locals.var_fn169_calc_iq__trapfracdl), ((assign15490_e15687 * locals.var_fn169_calc_iq__qd_dn4) * locals.var_fn169_calc_iq__trapfracdl), ((assign15490_e15687 * locals.var_fn169_calc_iq__qd_dn7) * locals.var_fn169_calc_iq__trapfracdl), ((assign15490_e15687 * locals.var_fn169_calc_iq__qd_dn9) * locals.var_fn169_calc_iq__trapfracdl), ((assign15490_e15687 * locals.var_fn169_calc_iq__qd_dn10) * locals.var_fn169_calc_iq__trapfracdl),)
    } else {
        (locals.var_fn169_calc_iq__qgdout, locals.var_fn169_calc_iq__qgdout_dn2, locals.var_fn169_calc_iq__qgdout_dn4, locals.var_fn169_calc_iq__qgdout_dn7, locals.var_fn169_calc_iq__qgdout_dn9, locals.var_fn169_calc_iq__qgdout_dn10,)
    }
};
        locals.var_fn169_calc_iq__qgdout = assign15490_e15693;
        locals.var_fn169_calc_iq__qgdout_dn2 = assign15490_e15693_d_n2;
        locals.var_fn169_calc_iq__qgdout_dn4 = assign15490_e15693_d_n4;
        locals.var_fn169_calc_iq__qgdout_dn7 = assign15490_e15693_d_n7;
        locals.var_fn169_calc_iq__qgdout_dn9 = assign15490_e15693_d_n9;
        locals.var_fn169_calc_iq__qgdout_dn10 = assign15490_e15693_d_n10;

        let assign15500_e15696: f64 = if locals.var_fn169_calc_iq__qcbflag == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard195 = assign15500_e15696;

        let (assign15510_e15712, assign15510_e15712_d_n2, assign15510_e15712_d_n4, assign15510_e15712_d_n7, assign15510_e15712_d_n10,) = {
    if ((locals.var_guard168 != 0.0) && (locals.var_guard195 != 0.0)) {
        let assign15510_e15704: f64 = (p.p51 * 0.5);
        let assign15510_e15706: f64 = (assign15510_e15704 * locals.var_fn169_calc_iq__alpha_phit);
        let assign15510_e15707: f64 = (locals.var_fn169_calc_iq__vtof - assign15510_e15706);
        let assign15510_e15708: f64 = (locals.var_fn169_calc_iq__vcin - assign15510_e15707);
        let assign15510_e15710: f64 = (assign15510_e15708 / locals.var_fn169_calc_iq__two_n_phit0);
        (assign15510_e15710, (locals.var_fn169_calc_iq__vcin_dn2 / locals.var_fn169_calc_iq__two_n_phit0), ((((-(locals.var_fn169_calc_iq__vtof_dn4 - (assign15510_e15704 * locals.var_fn169_calc_iq__alpha_phit_dn4))) * locals.var_fn169_calc_iq__two_n_phit0) - (assign15510_e15708 * locals.var_fn169_calc_iq__two_n_phit0_dn4)) / (locals.var_fn169_calc_iq__two_n_phit0 * locals.var_fn169_calc_iq__two_n_phit0)), (locals.var_fn169_calc_iq__vcin_dn7 / locals.var_fn169_calc_iq__two_n_phit0), (locals.var_fn169_calc_iq__vcin_dn10 / locals.var_fn169_calc_iq__two_n_phit0),)
    } else {
        (locals.var_fn169_calc_iq__etac, locals.var_fn169_calc_iq__etac_dn2, locals.var_fn169_calc_iq__etac_dn4, locals.var_fn169_calc_iq__etac_dn7, locals.var_fn169_calc_iq__etac_dn10,)
    }
};
        locals.var_fn169_calc_iq__etac = assign15510_e15712;
        locals.var_fn169_calc_iq__etac_dn2 = assign15510_e15712_d_n2;
        locals.var_fn169_calc_iq__etac_dn4 = assign15510_e15712_d_n4;
        locals.var_fn169_calc_iq__etac_dn7 = assign15510_e15712_d_n7;
        locals.var_fn169_calc_iq__etac_dn10 = assign15510_e15712_d_n10;

        let assign15520_e15715: f64 = if locals.var_fn169_calc_iq__etac > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard196 = assign15520_e15715;

        let (assign15530_e15723, assign15530_e15723_d_n2, assign15530_e15723_d_n3, assign15530_e15723_d_n4, assign15530_e15723_d_n7, assign15530_e15723_d_n9, assign15530_e15723_d_n10,) = {
    if (((locals.var_guard168 != 0.0) && (locals.var_guard195 != 0.0)) && (locals.var_guard196 != 0.0)) {
        (locals.var_fn169_calc_iq__etac, locals.var_fn169_calc_iq__etac_dn2, 0.0, locals.var_fn169_calc_iq__etac_dn4, locals.var_fn169_calc_iq__etac_dn7, 0.0, locals.var_fn169_calc_iq__etac_dn10,)
    } else {
        (locals.var_fn169_calc_iq__exparg, locals.var_fn169_calc_iq__exparg_dn2, locals.var_fn169_calc_iq__exparg_dn3, locals.var_fn169_calc_iq__exparg_dn4, locals.var_fn169_calc_iq__exparg_dn7, locals.var_fn169_calc_iq__exparg_dn9, locals.var_fn169_calc_iq__exparg_dn10,)
    }
};
        locals.var_fn169_calc_iq__exparg = assign15530_e15723;
        locals.var_fn169_calc_iq__exparg_dn2 = assign15530_e15723_d_n2;
        locals.var_fn169_calc_iq__exparg_dn3 = assign15530_e15723_d_n3;
        locals.var_fn169_calc_iq__exparg_dn4 = assign15530_e15723_d_n4;
        locals.var_fn169_calc_iq__exparg_dn7 = assign15530_e15723_d_n7;
        locals.var_fn169_calc_iq__exparg_dn9 = assign15530_e15723_d_n9;
        locals.var_fn169_calc_iq__exparg_dn10 = assign15530_e15723_d_n10;

        let assign15540_e15726: f64 = (-50.0);
        let assign15540_e15727: f64 = if locals.var_fn169_calc_iq__etac < assign15540_e15726 { 1.0 } else { 0.0 };
        locals.var_guard197 = assign15540_e15727;

        let (assign15550_e15739, assign15550_e15739_d_n2, assign15550_e15739_d_n3, assign15550_e15739_d_n4, assign15550_e15739_d_n7, assign15550_e15739_d_n9, assign15550_e15739_d_n10,) = {
    if ((((locals.var_guard168 != 0.0) && (locals.var_guard195 != 0.0)) && (locals.var_guard196 == 0.0)) && (locals.var_guard197 != 0.0)) {
        let assign15550_e15737: f64 = (locals.var_fn169_calc_iq__etac).exp();
        (assign15550_e15737, (assign15550_e15737 * locals.var_fn169_calc_iq__etac_dn2), 0.0, (assign15550_e15737 * locals.var_fn169_calc_iq__etac_dn4), (assign15550_e15737 * locals.var_fn169_calc_iq__etac_dn7), 0.0, (assign15550_e15737 * locals.var_fn169_calc_iq__etac_dn10),)
    } else {
        (locals.var_fn169_calc_iq__exparg, locals.var_fn169_calc_iq__exparg_dn2, locals.var_fn169_calc_iq__exparg_dn3, locals.var_fn169_calc_iq__exparg_dn4, locals.var_fn169_calc_iq__exparg_dn7, locals.var_fn169_calc_iq__exparg_dn9, locals.var_fn169_calc_iq__exparg_dn10,)
    }
};
        locals.var_fn169_calc_iq__exparg = assign15550_e15739;
        locals.var_fn169_calc_iq__exparg_dn2 = assign15550_e15739_d_n2;
        locals.var_fn169_calc_iq__exparg_dn3 = assign15550_e15739_d_n3;
        locals.var_fn169_calc_iq__exparg_dn4 = assign15550_e15739_d_n4;
        locals.var_fn169_calc_iq__exparg_dn7 = assign15550_e15739_d_n7;
        locals.var_fn169_calc_iq__exparg_dn9 = assign15550_e15739_d_n9;
        locals.var_fn169_calc_iq__exparg_dn10 = assign15550_e15739_d_n10;

        let (assign15560_e15755, assign15560_e15755_d_n2, assign15560_e15755_d_n3, assign15560_e15755_d_n4, assign15560_e15755_d_n7, assign15560_e15755_d_n9, assign15560_e15755_d_n10,) = {
    if ((((locals.var_guard168 != 0.0) && (locals.var_guard195 != 0.0)) && (locals.var_guard196 == 0.0)) && (locals.var_guard197 == 0.0)) {
        let assign15560_e15751: f64 = (locals.var_fn169_calc_iq__etac).exp();
        let assign15560_e15752: f64 = (1.0 + assign15560_e15751);
        let assign15560_e15753: f64 = (assign15560_e15752).ln();
        (assign15560_e15753, ((assign15560_e15751 * locals.var_fn169_calc_iq__etac_dn2) / assign15560_e15752), 0.0, ((assign15560_e15751 * locals.var_fn169_calc_iq__etac_dn4) / assign15560_e15752), ((assign15560_e15751 * locals.var_fn169_calc_iq__etac_dn7) / assign15560_e15752), 0.0, ((assign15560_e15751 * locals.var_fn169_calc_iq__etac_dn10) / assign15560_e15752),)
    } else {
        (locals.var_fn169_calc_iq__exparg, locals.var_fn169_calc_iq__exparg_dn2, locals.var_fn169_calc_iq__exparg_dn3, locals.var_fn169_calc_iq__exparg_dn4, locals.var_fn169_calc_iq__exparg_dn7, locals.var_fn169_calc_iq__exparg_dn9, locals.var_fn169_calc_iq__exparg_dn10,)
    }
};
        locals.var_fn169_calc_iq__exparg = assign15560_e15755;
        locals.var_fn169_calc_iq__exparg_dn2 = assign15560_e15755_d_n2;
        locals.var_fn169_calc_iq__exparg_dn3 = assign15560_e15755_d_n3;
        locals.var_fn169_calc_iq__exparg_dn4 = assign15560_e15755_d_n4;
        locals.var_fn169_calc_iq__exparg_dn7 = assign15560_e15755_d_n7;
        locals.var_fn169_calc_iq__exparg_dn9 = assign15560_e15755_d_n9;
        locals.var_fn169_calc_iq__exparg_dn10 = assign15560_e15755_d_n10;

    }

    pub(super) fn stamp_transient_block_39(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign15570_e15773, assign15570_e15773_d_n2, assign15570_e15773_d_n3, assign15570_e15773_d_n4, assign15570_e15773_d_n7, assign15570_e15773_d_n9, assign15570_e15773_d_n10,) = {
    if ((locals.var_guard168 != 0.0) && (locals.var_guard195 != 0.0)) {
        let assign15570_e15761: f64 = (locals.var_fn169_calc_iq__w * locals.var_fn169_calc_iq__ngf);
        let assign15570_e15763: f64 = (assign15570_e15761 * locals.var_fn169_calc_iq__type);
        let assign15570_e15765: f64 = (assign15570_e15763 * locals.var_fn169_calc_iq__cc);
        let assign15570_e15767: f64 = (assign15570_e15765 * locals.var_fn169_calc_iq__two_n_phit0);
        let assign15570_e15769: f64 = (assign15570_e15767 * locals.var_fn169_calc_iq__exparg);
        let assign15570_e15771: f64 = (assign15570_e15769 * locals.var_fn169_calc_iq__trapfracdl);
        (assign15570_e15771, ((assign15570_e15767 * locals.var_fn169_calc_iq__exparg_dn2) * locals.var_fn169_calc_iq__trapfracdl), ((assign15570_e15767 * locals.var_fn169_calc_iq__exparg_dn3) * locals.var_fn169_calc_iq__trapfracdl), ((((((assign15570_e15763 * locals.var_fn169_calc_iq__cc_dn4) * locals.var_fn169_calc_iq__two_n_phit0) + (assign15570_e15765 * locals.var_fn169_calc_iq__two_n_phit0_dn4)) * locals.var_fn169_calc_iq__exparg) + (assign15570_e15767 * locals.var_fn169_calc_iq__exparg_dn4)) * locals.var_fn169_calc_iq__trapfracdl), ((assign15570_e15767 * locals.var_fn169_calc_iq__exparg_dn7) * locals.var_fn169_calc_iq__trapfracdl), ((assign15570_e15767 * locals.var_fn169_calc_iq__exparg_dn9) * locals.var_fn169_calc_iq__trapfracdl), ((assign15570_e15767 * locals.var_fn169_calc_iq__exparg_dn10) * locals.var_fn169_calc_iq__trapfracdl),)
    } else {
        (locals.var_fn169_calc_iq__qcout, locals.var_fn169_calc_iq__qcout_dn2, locals.var_fn169_calc_iq__qcout_dn3, locals.var_fn169_calc_iq__qcout_dn4, locals.var_fn169_calc_iq__qcout_dn7, locals.var_fn169_calc_iq__qcout_dn9, locals.var_fn169_calc_iq__qcout_dn10,)
    }
};
        locals.var_fn169_calc_iq__qcout = assign15570_e15773;
        locals.var_fn169_calc_iq__qcout_dn2 = assign15570_e15773_d_n2;
        locals.var_fn169_calc_iq__qcout_dn3 = assign15570_e15773_d_n3;
        locals.var_fn169_calc_iq__qcout_dn4 = assign15570_e15773_d_n4;
        locals.var_fn169_calc_iq__qcout_dn7 = assign15570_e15773_d_n7;
        locals.var_fn169_calc_iq__qcout_dn9 = assign15570_e15773_d_n9;
        locals.var_fn169_calc_iq__qcout_dn10 = assign15570_e15773_d_n10;

        let (assign15580_e15789, assign15580_e15789_d_n3, assign15580_e15789_d_n4, assign15580_e15789_d_n10,) = {
    if ((locals.var_guard168 != 0.0) && (locals.var_guard195 != 0.0)) {
        let assign15580_e15781: f64 = (p.p51 * 0.5);
        let assign15580_e15783: f64 = (assign15580_e15781 * locals.var_fn169_calc_iq__alpha_phit);
        let assign15580_e15784: f64 = (locals.var_fn169_calc_iq__vtof - assign15580_e15783);
        let assign15580_e15785: f64 = (locals.var_fn169_calc_iq__vbin - assign15580_e15784);
        let assign15580_e15787: f64 = (assign15580_e15785 / locals.var_fn169_calc_iq__two_n_phit0);
        (assign15580_e15787, (locals.var_fn169_calc_iq__vbin_dn3 / locals.var_fn169_calc_iq__two_n_phit0), ((((-(locals.var_fn169_calc_iq__vtof_dn4 - (assign15580_e15781 * locals.var_fn169_calc_iq__alpha_phit_dn4))) * locals.var_fn169_calc_iq__two_n_phit0) - (assign15580_e15785 * locals.var_fn169_calc_iq__two_n_phit0_dn4)) / (locals.var_fn169_calc_iq__two_n_phit0 * locals.var_fn169_calc_iq__two_n_phit0)), (locals.var_fn169_calc_iq__vbin_dn10 / locals.var_fn169_calc_iq__two_n_phit0),)
    } else {
        (locals.var_fn169_calc_iq__etab, locals.var_fn169_calc_iq__etab_dn3, locals.var_fn169_calc_iq__etab_dn4, locals.var_fn169_calc_iq__etab_dn10,)
    }
};
        locals.var_fn169_calc_iq__etab = assign15580_e15789;
        locals.var_fn169_calc_iq__etab_dn3 = assign15580_e15789_d_n3;
        locals.var_fn169_calc_iq__etab_dn4 = assign15580_e15789_d_n4;
        locals.var_fn169_calc_iq__etab_dn10 = assign15580_e15789_d_n10;

        let assign15590_e15792: f64 = if locals.var_fn169_calc_iq__etab > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard198 = assign15590_e15792;

        let (assign15600_e15800, assign15600_e15800_d_n2, assign15600_e15800_d_n3, assign15600_e15800_d_n4, assign15600_e15800_d_n7, assign15600_e15800_d_n9, assign15600_e15800_d_n10,) = {
    if (((locals.var_guard168 != 0.0) && (locals.var_guard195 != 0.0)) && (locals.var_guard198 != 0.0)) {
        (locals.var_fn169_calc_iq__etab, 0.0, locals.var_fn169_calc_iq__etab_dn3, locals.var_fn169_calc_iq__etab_dn4, 0.0, 0.0, locals.var_fn169_calc_iq__etab_dn10,)
    } else {
        (locals.var_fn169_calc_iq__exparg, locals.var_fn169_calc_iq__exparg_dn2, locals.var_fn169_calc_iq__exparg_dn3, locals.var_fn169_calc_iq__exparg_dn4, locals.var_fn169_calc_iq__exparg_dn7, locals.var_fn169_calc_iq__exparg_dn9, locals.var_fn169_calc_iq__exparg_dn10,)
    }
};
        locals.var_fn169_calc_iq__exparg = assign15600_e15800;
        locals.var_fn169_calc_iq__exparg_dn2 = assign15600_e15800_d_n2;
        locals.var_fn169_calc_iq__exparg_dn3 = assign15600_e15800_d_n3;
        locals.var_fn169_calc_iq__exparg_dn4 = assign15600_e15800_d_n4;
        locals.var_fn169_calc_iq__exparg_dn7 = assign15600_e15800_d_n7;
        locals.var_fn169_calc_iq__exparg_dn9 = assign15600_e15800_d_n9;
        locals.var_fn169_calc_iq__exparg_dn10 = assign15600_e15800_d_n10;

        let assign15610_e15803: f64 = (-50.0);
        let assign15610_e15804: f64 = if locals.var_fn169_calc_iq__etab < assign15610_e15803 { 1.0 } else { 0.0 };
        locals.var_guard199 = assign15610_e15804;

        let (assign15620_e15816, assign15620_e15816_d_n2, assign15620_e15816_d_n3, assign15620_e15816_d_n4, assign15620_e15816_d_n7, assign15620_e15816_d_n9, assign15620_e15816_d_n10,) = {
    if ((((locals.var_guard168 != 0.0) && (locals.var_guard195 != 0.0)) && (locals.var_guard198 == 0.0)) && (locals.var_guard199 != 0.0)) {
        let assign15620_e15814: f64 = (locals.var_fn169_calc_iq__etab).exp();
        (assign15620_e15814, 0.0, (assign15620_e15814 * locals.var_fn169_calc_iq__etab_dn3), (assign15620_e15814 * locals.var_fn169_calc_iq__etab_dn4), 0.0, 0.0, (assign15620_e15814 * locals.var_fn169_calc_iq__etab_dn10),)
    } else {
        (locals.var_fn169_calc_iq__exparg, locals.var_fn169_calc_iq__exparg_dn2, locals.var_fn169_calc_iq__exparg_dn3, locals.var_fn169_calc_iq__exparg_dn4, locals.var_fn169_calc_iq__exparg_dn7, locals.var_fn169_calc_iq__exparg_dn9, locals.var_fn169_calc_iq__exparg_dn10,)
    }
};
        locals.var_fn169_calc_iq__exparg = assign15620_e15816;
        locals.var_fn169_calc_iq__exparg_dn2 = assign15620_e15816_d_n2;
        locals.var_fn169_calc_iq__exparg_dn3 = assign15620_e15816_d_n3;
        locals.var_fn169_calc_iq__exparg_dn4 = assign15620_e15816_d_n4;
        locals.var_fn169_calc_iq__exparg_dn7 = assign15620_e15816_d_n7;
        locals.var_fn169_calc_iq__exparg_dn9 = assign15620_e15816_d_n9;
        locals.var_fn169_calc_iq__exparg_dn10 = assign15620_e15816_d_n10;

        let (assign15630_e15832, assign15630_e15832_d_n2, assign15630_e15832_d_n3, assign15630_e15832_d_n4, assign15630_e15832_d_n7, assign15630_e15832_d_n9, assign15630_e15832_d_n10,) = {
    if ((((locals.var_guard168 != 0.0) && (locals.var_guard195 != 0.0)) && (locals.var_guard198 == 0.0)) && (locals.var_guard199 == 0.0)) {
        let assign15630_e15828: f64 = (locals.var_fn169_calc_iq__etab).exp();
        let assign15630_e15829: f64 = (1.0 + assign15630_e15828);
        let assign15630_e15830: f64 = (assign15630_e15829).ln();
        (assign15630_e15830, 0.0, ((assign15630_e15828 * locals.var_fn169_calc_iq__etab_dn3) / assign15630_e15829), ((assign15630_e15828 * locals.var_fn169_calc_iq__etab_dn4) / assign15630_e15829), 0.0, 0.0, ((assign15630_e15828 * locals.var_fn169_calc_iq__etab_dn10) / assign15630_e15829),)
    } else {
        (locals.var_fn169_calc_iq__exparg, locals.var_fn169_calc_iq__exparg_dn2, locals.var_fn169_calc_iq__exparg_dn3, locals.var_fn169_calc_iq__exparg_dn4, locals.var_fn169_calc_iq__exparg_dn7, locals.var_fn169_calc_iq__exparg_dn9, locals.var_fn169_calc_iq__exparg_dn10,)
    }
};
        locals.var_fn169_calc_iq__exparg = assign15630_e15832;
        locals.var_fn169_calc_iq__exparg_dn2 = assign15630_e15832_d_n2;
        locals.var_fn169_calc_iq__exparg_dn3 = assign15630_e15832_d_n3;
        locals.var_fn169_calc_iq__exparg_dn4 = assign15630_e15832_d_n4;
        locals.var_fn169_calc_iq__exparg_dn7 = assign15630_e15832_d_n7;
        locals.var_fn169_calc_iq__exparg_dn9 = assign15630_e15832_d_n9;
        locals.var_fn169_calc_iq__exparg_dn10 = assign15630_e15832_d_n10;

        let (assign15640_e15850, assign15640_e15850_d_n2, assign15640_e15850_d_n3, assign15640_e15850_d_n4, assign15640_e15850_d_n7, assign15640_e15850_d_n9, assign15640_e15850_d_n10,) = {
    if ((locals.var_guard168 != 0.0) && (locals.var_guard195 != 0.0)) {
        let assign15640_e15838: f64 = (locals.var_fn169_calc_iq__w * locals.var_fn169_calc_iq__ngf);
        let assign15640_e15840: f64 = (assign15640_e15838 * locals.var_fn169_calc_iq__type);
        let assign15640_e15842: f64 = (assign15640_e15840 * locals.var_fn169_calc_iq__cb);
        let assign15640_e15844: f64 = (assign15640_e15842 * locals.var_fn169_calc_iq__two_n_phit0);
        let assign15640_e15846: f64 = (assign15640_e15844 * locals.var_fn169_calc_iq__exparg);
        let assign15640_e15848: f64 = (assign15640_e15846 * locals.var_fn169_calc_iq__trapfracdl);
        (assign15640_e15848, ((assign15640_e15844 * locals.var_fn169_calc_iq__exparg_dn2) * locals.var_fn169_calc_iq__trapfracdl), ((assign15640_e15844 * locals.var_fn169_calc_iq__exparg_dn3) * locals.var_fn169_calc_iq__trapfracdl), ((((((assign15640_e15840 * locals.var_fn169_calc_iq__cb_dn4) * locals.var_fn169_calc_iq__two_n_phit0) + (assign15640_e15842 * locals.var_fn169_calc_iq__two_n_phit0_dn4)) * locals.var_fn169_calc_iq__exparg) + (assign15640_e15844 * locals.var_fn169_calc_iq__exparg_dn4)) * locals.var_fn169_calc_iq__trapfracdl), ((assign15640_e15844 * locals.var_fn169_calc_iq__exparg_dn7) * locals.var_fn169_calc_iq__trapfracdl), ((assign15640_e15844 * locals.var_fn169_calc_iq__exparg_dn9) * locals.var_fn169_calc_iq__trapfracdl), ((assign15640_e15844 * locals.var_fn169_calc_iq__exparg_dn10) * locals.var_fn169_calc_iq__trapfracdl),)
    } else {
        (locals.var_fn169_calc_iq__qbout, locals.var_fn169_calc_iq__qbout_dn2, locals.var_fn169_calc_iq__qbout_dn3, locals.var_fn169_calc_iq__qbout_dn4, locals.var_fn169_calc_iq__qbout_dn7, locals.var_fn169_calc_iq__qbout_dn9, locals.var_fn169_calc_iq__qbout_dn10,)
    }
};
        locals.var_fn169_calc_iq__qbout = assign15640_e15850;
        locals.var_fn169_calc_iq__qbout_dn2 = assign15640_e15850_d_n2;
        locals.var_fn169_calc_iq__qbout_dn3 = assign15640_e15850_d_n3;
        locals.var_fn169_calc_iq__qbout_dn4 = assign15640_e15850_d_n4;
        locals.var_fn169_calc_iq__qbout_dn7 = assign15640_e15850_d_n7;
        locals.var_fn169_calc_iq__qbout_dn9 = assign15640_e15850_d_n9;
        locals.var_fn169_calc_iq__qbout_dn10 = assign15640_e15850_d_n10;

        let (assign15650_e15857, assign15650_e15857_d_n2, assign15650_e15857_d_n3, assign15650_e15857_d_n4, assign15650_e15857_d_n7, assign15650_e15857_d_n9, assign15650_e15857_d_n10,) = {
    if ((locals.var_guard168 != 0.0) && (locals.var_guard195 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__qcout, locals.var_fn169_calc_iq__qcout_dn2, locals.var_fn169_calc_iq__qcout_dn3, locals.var_fn169_calc_iq__qcout_dn4, locals.var_fn169_calc_iq__qcout_dn7, locals.var_fn169_calc_iq__qcout_dn9, locals.var_fn169_calc_iq__qcout_dn10,)
    }
};
        locals.var_fn169_calc_iq__qcout = assign15650_e15857;
        locals.var_fn169_calc_iq__qcout_dn2 = assign15650_e15857_d_n2;
        locals.var_fn169_calc_iq__qcout_dn3 = assign15650_e15857_d_n3;
        locals.var_fn169_calc_iq__qcout_dn4 = assign15650_e15857_d_n4;
        locals.var_fn169_calc_iq__qcout_dn7 = assign15650_e15857_d_n7;
        locals.var_fn169_calc_iq__qcout_dn9 = assign15650_e15857_d_n9;
        locals.var_fn169_calc_iq__qcout_dn10 = assign15650_e15857_d_n10;

        let (assign15660_e15864, assign15660_e15864_d_n2, assign15660_e15864_d_n3, assign15660_e15864_d_n4, assign15660_e15864_d_n7, assign15660_e15864_d_n9, assign15660_e15864_d_n10,) = {
    if ((locals.var_guard168 != 0.0) && (locals.var_guard195 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__qbout, locals.var_fn169_calc_iq__qbout_dn2, locals.var_fn169_calc_iq__qbout_dn3, locals.var_fn169_calc_iq__qbout_dn4, locals.var_fn169_calc_iq__qbout_dn7, locals.var_fn169_calc_iq__qbout_dn9, locals.var_fn169_calc_iq__qbout_dn10,)
    }
};
        locals.var_fn169_calc_iq__qbout = assign15660_e15864;
        locals.var_fn169_calc_iq__qbout_dn2 = assign15660_e15864_d_n2;
        locals.var_fn169_calc_iq__qbout_dn3 = assign15660_e15864_d_n3;
        locals.var_fn169_calc_iq__qbout_dn4 = assign15660_e15864_d_n4;
        locals.var_fn169_calc_iq__qbout_dn7 = assign15660_e15864_d_n7;
        locals.var_fn169_calc_iq__qbout_dn9 = assign15660_e15864_d_n9;
        locals.var_fn169_calc_iq__qbout_dn10 = assign15660_e15864_d_n10;

        let assign15670_e15867: f64 = if locals.var_fn169_calc_iq__qgsflag == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard200 = assign15670_e15867;

        let (assign15680_e15883, assign15680_e15883_d_n2, assign15680_e15883_d_n4, assign15680_e15883_d_n7, assign15680_e15883_d_n10,) = {
    if ((locals.var_guard168 != 0.0) && (locals.var_guard200 != 0.0)) {
        let assign15680_e15875: f64 = (p.p51 * 0.5);
        let assign15680_e15877: f64 = (assign15680_e15875 * locals.var_fn169_calc_iq__alpha_phit);
        let assign15680_e15878: f64 = (locals.var_fn169_calc_iq__vtof - assign15680_e15877);
        let assign15680_e15879: f64 = (locals.var_fn169_calc_iq__vgsin - assign15680_e15878);
        let assign15680_e15881: f64 = (assign15680_e15879 / locals.var_fn169_calc_iq__two_n_phit0);
        (assign15680_e15881, (locals.var_fn169_calc_iq__vgsin_dn2 / locals.var_fn169_calc_iq__two_n_phit0), ((((-(locals.var_fn169_calc_iq__vtof_dn4 - (assign15680_e15875 * locals.var_fn169_calc_iq__alpha_phit_dn4))) * locals.var_fn169_calc_iq__two_n_phit0) - (assign15680_e15879 * locals.var_fn169_calc_iq__two_n_phit0_dn4)) / (locals.var_fn169_calc_iq__two_n_phit0 * locals.var_fn169_calc_iq__two_n_phit0)), (locals.var_fn169_calc_iq__vgsin_dn7 / locals.var_fn169_calc_iq__two_n_phit0), (locals.var_fn169_calc_iq__vgsin_dn10 / locals.var_fn169_calc_iq__two_n_phit0),)
    } else {
        (locals.var_fn169_calc_iq__etags, locals.var_fn169_calc_iq__etags_dn2, locals.var_fn169_calc_iq__etags_dn4, locals.var_fn169_calc_iq__etags_dn7, locals.var_fn169_calc_iq__etags_dn10,)
    }
};
        locals.var_fn169_calc_iq__etags = assign15680_e15883;
        locals.var_fn169_calc_iq__etags_dn2 = assign15680_e15883_d_n2;
        locals.var_fn169_calc_iq__etags_dn4 = assign15680_e15883_d_n4;
        locals.var_fn169_calc_iq__etags_dn7 = assign15680_e15883_d_n7;
        locals.var_fn169_calc_iq__etags_dn10 = assign15680_e15883_d_n10;

        let assign15690_e15886: f64 = if locals.var_fn169_calc_iq__etags > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard201 = assign15690_e15886;

        let (assign15700_e15894, assign15700_e15894_d_n2, assign15700_e15894_d_n3, assign15700_e15894_d_n4, assign15700_e15894_d_n7, assign15700_e15894_d_n9, assign15700_e15894_d_n10,) = {
    if (((locals.var_guard168 != 0.0) && (locals.var_guard200 != 0.0)) && (locals.var_guard201 != 0.0)) {
        (locals.var_fn169_calc_iq__etags, locals.var_fn169_calc_iq__etags_dn2, 0.0, locals.var_fn169_calc_iq__etags_dn4, locals.var_fn169_calc_iq__etags_dn7, 0.0, locals.var_fn169_calc_iq__etags_dn10,)
    } else {
        (locals.var_fn169_calc_iq__exparg, locals.var_fn169_calc_iq__exparg_dn2, locals.var_fn169_calc_iq__exparg_dn3, locals.var_fn169_calc_iq__exparg_dn4, locals.var_fn169_calc_iq__exparg_dn7, locals.var_fn169_calc_iq__exparg_dn9, locals.var_fn169_calc_iq__exparg_dn10,)
    }
};
        locals.var_fn169_calc_iq__exparg = assign15700_e15894;
        locals.var_fn169_calc_iq__exparg_dn2 = assign15700_e15894_d_n2;
        locals.var_fn169_calc_iq__exparg_dn3 = assign15700_e15894_d_n3;
        locals.var_fn169_calc_iq__exparg_dn4 = assign15700_e15894_d_n4;
        locals.var_fn169_calc_iq__exparg_dn7 = assign15700_e15894_d_n7;
        locals.var_fn169_calc_iq__exparg_dn9 = assign15700_e15894_d_n9;
        locals.var_fn169_calc_iq__exparg_dn10 = assign15700_e15894_d_n10;

        let assign15710_e15897: f64 = (-50.0);
        let assign15710_e15898: f64 = if locals.var_fn169_calc_iq__etags < assign15710_e15897 { 1.0 } else { 0.0 };
        locals.var_guard202 = assign15710_e15898;

        let (assign15720_e15910, assign15720_e15910_d_n2, assign15720_e15910_d_n3, assign15720_e15910_d_n4, assign15720_e15910_d_n7, assign15720_e15910_d_n9, assign15720_e15910_d_n10,) = {
    if ((((locals.var_guard168 != 0.0) && (locals.var_guard200 != 0.0)) && (locals.var_guard201 == 0.0)) && (locals.var_guard202 != 0.0)) {
        let assign15720_e15908: f64 = (locals.var_fn169_calc_iq__etags).exp();
        (assign15720_e15908, (assign15720_e15908 * locals.var_fn169_calc_iq__etags_dn2), 0.0, (assign15720_e15908 * locals.var_fn169_calc_iq__etags_dn4), (assign15720_e15908 * locals.var_fn169_calc_iq__etags_dn7), 0.0, (assign15720_e15908 * locals.var_fn169_calc_iq__etags_dn10),)
    } else {
        (locals.var_fn169_calc_iq__exparg, locals.var_fn169_calc_iq__exparg_dn2, locals.var_fn169_calc_iq__exparg_dn3, locals.var_fn169_calc_iq__exparg_dn4, locals.var_fn169_calc_iq__exparg_dn7, locals.var_fn169_calc_iq__exparg_dn9, locals.var_fn169_calc_iq__exparg_dn10,)
    }
};
        locals.var_fn169_calc_iq__exparg = assign15720_e15910;
        locals.var_fn169_calc_iq__exparg_dn2 = assign15720_e15910_d_n2;
        locals.var_fn169_calc_iq__exparg_dn3 = assign15720_e15910_d_n3;
        locals.var_fn169_calc_iq__exparg_dn4 = assign15720_e15910_d_n4;
        locals.var_fn169_calc_iq__exparg_dn7 = assign15720_e15910_d_n7;
        locals.var_fn169_calc_iq__exparg_dn9 = assign15720_e15910_d_n9;
        locals.var_fn169_calc_iq__exparg_dn10 = assign15720_e15910_d_n10;

        let (assign15730_e15926, assign15730_e15926_d_n2, assign15730_e15926_d_n3, assign15730_e15926_d_n4, assign15730_e15926_d_n7, assign15730_e15926_d_n9, assign15730_e15926_d_n10,) = {
    if ((((locals.var_guard168 != 0.0) && (locals.var_guard200 != 0.0)) && (locals.var_guard201 == 0.0)) && (locals.var_guard202 == 0.0)) {
        let assign15730_e15922: f64 = (locals.var_fn169_calc_iq__etags).exp();
        let assign15730_e15923: f64 = (1.0 + assign15730_e15922);
        let assign15730_e15924: f64 = (assign15730_e15923).ln();
        (assign15730_e15924, ((assign15730_e15922 * locals.var_fn169_calc_iq__etags_dn2) / assign15730_e15923), 0.0, ((assign15730_e15922 * locals.var_fn169_calc_iq__etags_dn4) / assign15730_e15923), ((assign15730_e15922 * locals.var_fn169_calc_iq__etags_dn7) / assign15730_e15923), 0.0, ((assign15730_e15922 * locals.var_fn169_calc_iq__etags_dn10) / assign15730_e15923),)
    } else {
        (locals.var_fn169_calc_iq__exparg, locals.var_fn169_calc_iq__exparg_dn2, locals.var_fn169_calc_iq__exparg_dn3, locals.var_fn169_calc_iq__exparg_dn4, locals.var_fn169_calc_iq__exparg_dn7, locals.var_fn169_calc_iq__exparg_dn9, locals.var_fn169_calc_iq__exparg_dn10,)
    }
};
        locals.var_fn169_calc_iq__exparg = assign15730_e15926;
        locals.var_fn169_calc_iq__exparg_dn2 = assign15730_e15926_d_n2;
        locals.var_fn169_calc_iq__exparg_dn3 = assign15730_e15926_d_n3;
        locals.var_fn169_calc_iq__exparg_dn4 = assign15730_e15926_d_n4;
        locals.var_fn169_calc_iq__exparg_dn7 = assign15730_e15926_d_n7;
        locals.var_fn169_calc_iq__exparg_dn9 = assign15730_e15926_d_n9;
        locals.var_fn169_calc_iq__exparg_dn10 = assign15730_e15926_d_n10;

        let (assign15740_e15944, assign15740_e15944_d_n2, assign15740_e15944_d_n3, assign15740_e15944_d_n4, assign15740_e15944_d_n7, assign15740_e15944_d_n9, assign15740_e15944_d_n10,) = {
    if ((locals.var_guard168 != 0.0) && (locals.var_guard200 != 0.0)) {
        let assign15740_e15932: f64 = (locals.var_fn169_calc_iq__w * locals.var_fn169_calc_iq__ngf);
        let assign15740_e15934: f64 = (assign15740_e15932 * locals.var_fn169_calc_iq__type);
        let assign15740_e15936: f64 = (assign15740_e15934 * locals.var_fn169_calc_iq__cs);
        let assign15740_e15938: f64 = (assign15740_e15936 * locals.var_fn169_calc_iq__two_n_phit0);
        let assign15740_e15940: f64 = (assign15740_e15938 * locals.var_fn169_calc_iq__exparg);
        let assign15740_e15942: f64 = (assign15740_e15940 * locals.var_fn169_calc_iq__trapfracdl);
        (assign15740_e15942, ((assign15740_e15938 * locals.var_fn169_calc_iq__exparg_dn2) * locals.var_fn169_calc_iq__trapfracdl), ((assign15740_e15938 * locals.var_fn169_calc_iq__exparg_dn3) * locals.var_fn169_calc_iq__trapfracdl), ((((assign15740_e15936 * locals.var_fn169_calc_iq__two_n_phit0_dn4) * locals.var_fn169_calc_iq__exparg) + (assign15740_e15938 * locals.var_fn169_calc_iq__exparg_dn4)) * locals.var_fn169_calc_iq__trapfracdl), ((assign15740_e15938 * locals.var_fn169_calc_iq__exparg_dn7) * locals.var_fn169_calc_iq__trapfracdl), ((assign15740_e15938 * locals.var_fn169_calc_iq__exparg_dn9) * locals.var_fn169_calc_iq__trapfracdl), ((assign15740_e15938 * locals.var_fn169_calc_iq__exparg_dn10) * locals.var_fn169_calc_iq__trapfracdl),)
    } else {
        (locals.var_fn169_calc_iq__qsout, locals.var_fn169_calc_iq__qsout_dn2, locals.var_fn169_calc_iq__qsout_dn3, locals.var_fn169_calc_iq__qsout_dn4, locals.var_fn169_calc_iq__qsout_dn7, locals.var_fn169_calc_iq__qsout_dn9, locals.var_fn169_calc_iq__qsout_dn10,)
    }
};
        locals.var_fn169_calc_iq__qsout = assign15740_e15944;
        locals.var_fn169_calc_iq__qsout_dn2 = assign15740_e15944_d_n2;
        locals.var_fn169_calc_iq__qsout_dn3 = assign15740_e15944_d_n3;
        locals.var_fn169_calc_iq__qsout_dn4 = assign15740_e15944_d_n4;
        locals.var_fn169_calc_iq__qsout_dn7 = assign15740_e15944_d_n7;
        locals.var_fn169_calc_iq__qsout_dn9 = assign15740_e15944_d_n9;
        locals.var_fn169_calc_iq__qsout_dn10 = assign15740_e15944_d_n10;

        let (assign15750_e15951, assign15750_e15951_d_n2, assign15750_e15951_d_n3, assign15750_e15951_d_n4, assign15750_e15951_d_n7, assign15750_e15951_d_n9, assign15750_e15951_d_n10,) = {
    if ((locals.var_guard168 != 0.0) && (locals.var_guard200 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn169_calc_iq__qsout, locals.var_fn169_calc_iq__qsout_dn2, locals.var_fn169_calc_iq__qsout_dn3, locals.var_fn169_calc_iq__qsout_dn4, locals.var_fn169_calc_iq__qsout_dn7, locals.var_fn169_calc_iq__qsout_dn9, locals.var_fn169_calc_iq__qsout_dn10,)
    }
};
        locals.var_fn169_calc_iq__qsout = assign15750_e15951;
        locals.var_fn169_calc_iq__qsout_dn2 = assign15750_e15951_d_n2;
        locals.var_fn169_calc_iq__qsout_dn3 = assign15750_e15951_d_n3;
        locals.var_fn169_calc_iq__qsout_dn4 = assign15750_e15951_d_n4;
        locals.var_fn169_calc_iq__qsout_dn7 = assign15750_e15951_d_n7;
        locals.var_fn169_calc_iq__qsout_dn9 = assign15750_e15951_d_n9;
        locals.var_fn169_calc_iq__qsout_dn10 = assign15750_e15951_d_n10;

        let (assign15780_e15963, assign15780_e15963_d_n2, assign15780_e15963_d_n4, assign15780_e15963_d_n7, assign15780_e15963_d_n9, assign15780_e15963_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        (locals.var_fn169_calc_iq__qgsout, locals.var_fn169_calc_iq__qgsout_dn2, locals.var_fn169_calc_iq__qgsout_dn4, locals.var_fn169_calc_iq__qgsout_dn7, locals.var_fn169_calc_iq__qgsout_dn9, locals.var_fn169_calc_iq__qgsout_dn10,)
    } else {
        (locals.var_qgsfps1, locals.var_qgsfps1_dn2, locals.var_qgsfps1_dn4, locals.var_qgsfps1_dn7, locals.var_qgsfps1_dn9, locals.var_qgsfps1_dn10,)
    }
};
        locals.var_qgsfps1 = assign15780_e15963;
        locals.var_qgsfps1_dn2 = assign15780_e15963_d_n2;
        locals.var_qgsfps1_dn4 = assign15780_e15963_d_n4;
        locals.var_qgsfps1_dn7 = assign15780_e15963_d_n7;
        locals.var_qgsfps1_dn9 = assign15780_e15963_d_n9;
        locals.var_qgsfps1_dn10 = assign15780_e15963_d_n10;

        let (assign15790_e15967, assign15790_e15967_d_n2, assign15790_e15967_d_n4, assign15790_e15967_d_n7, assign15790_e15967_d_n9, assign15790_e15967_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        (locals.var_fn169_calc_iq__qgdout, locals.var_fn169_calc_iq__qgdout_dn2, locals.var_fn169_calc_iq__qgdout_dn4, locals.var_fn169_calc_iq__qgdout_dn7, locals.var_fn169_calc_iq__qgdout_dn9, locals.var_fn169_calc_iq__qgdout_dn10,)
    } else {
        (locals.var_qgdfps1, locals.var_qgdfps1_dn2, locals.var_qgdfps1_dn4, locals.var_qgdfps1_dn7, locals.var_qgdfps1_dn9, locals.var_qgdfps1_dn10,)
    }
};
        locals.var_qgdfps1 = assign15790_e15967;
        locals.var_qgdfps1_dn2 = assign15790_e15967_d_n2;
        locals.var_qgdfps1_dn4 = assign15790_e15967_d_n4;
        locals.var_qgdfps1_dn7 = assign15790_e15967_d_n7;
        locals.var_qgdfps1_dn9 = assign15790_e15967_d_n9;
        locals.var_qgdfps1_dn10 = assign15790_e15967_d_n10;

        let (assign15800_e15971, assign15800_e15971_d_n2, assign15800_e15971_d_n3, assign15800_e15971_d_n4, assign15800_e15971_d_n7, assign15800_e15971_d_n9, assign15800_e15971_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        (locals.var_fn169_calc_iq__qcout, locals.var_fn169_calc_iq__qcout_dn2, locals.var_fn169_calc_iq__qcout_dn3, locals.var_fn169_calc_iq__qcout_dn4, locals.var_fn169_calc_iq__qcout_dn7, locals.var_fn169_calc_iq__qcout_dn9, locals.var_fn169_calc_iq__qcout_dn10,)
    } else {
        (locals.var_qcfps1, locals.var_qcfps1_dn2, locals.var_qcfps1_dn3, locals.var_qcfps1_dn4, locals.var_qcfps1_dn7, locals.var_qcfps1_dn9, locals.var_qcfps1_dn10,)
    }
};
        locals.var_qcfps1 = assign15800_e15971;
        locals.var_qcfps1_dn2 = assign15800_e15971_d_n2;
        locals.var_qcfps1_dn3 = assign15800_e15971_d_n3;
        locals.var_qcfps1_dn4 = assign15800_e15971_d_n4;
        locals.var_qcfps1_dn7 = assign15800_e15971_d_n7;
        locals.var_qcfps1_dn9 = assign15800_e15971_d_n9;
        locals.var_qcfps1_dn10 = assign15800_e15971_d_n10;

        let (assign15810_e15975, assign15810_e15975_d_n2, assign15810_e15975_d_n3, assign15810_e15975_d_n4, assign15810_e15975_d_n7, assign15810_e15975_d_n9, assign15810_e15975_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        (locals.var_fn169_calc_iq__qbout, locals.var_fn169_calc_iq__qbout_dn2, locals.var_fn169_calc_iq__qbout_dn3, locals.var_fn169_calc_iq__qbout_dn4, locals.var_fn169_calc_iq__qbout_dn7, locals.var_fn169_calc_iq__qbout_dn9, locals.var_fn169_calc_iq__qbout_dn10,)
    } else {
        (locals.var_qbfps1, locals.var_qbfps1_dn2, locals.var_qbfps1_dn3, locals.var_qbfps1_dn4, locals.var_qbfps1_dn7, locals.var_qbfps1_dn9, locals.var_qbfps1_dn10,)
    }
};
        locals.var_qbfps1 = assign15810_e15975;
        locals.var_qbfps1_dn2 = assign15810_e15975_d_n2;
        locals.var_qbfps1_dn3 = assign15810_e15975_d_n3;
        locals.var_qbfps1_dn4 = assign15810_e15975_d_n4;
        locals.var_qbfps1_dn7 = assign15810_e15975_d_n7;
        locals.var_qbfps1_dn9 = assign15810_e15975_d_n9;
        locals.var_qbfps1_dn10 = assign15810_e15975_d_n10;

        let (assign15820_e15979, assign15820_e15979_d_n2, assign15820_e15979_d_n3, assign15820_e15979_d_n4, assign15820_e15979_d_n7, assign15820_e15979_d_n9, assign15820_e15979_d_n10,) = {
    if (locals.var_guard168 != 0.0) {
        (locals.var_fn169_calc_iq__qsout, locals.var_fn169_calc_iq__qsout_dn2, locals.var_fn169_calc_iq__qsout_dn3, locals.var_fn169_calc_iq__qsout_dn4, locals.var_fn169_calc_iq__qsout_dn7, locals.var_fn169_calc_iq__qsout_dn9, locals.var_fn169_calc_iq__qsout_dn10,)
    } else {
        (locals.var_qsfps1, locals.var_qsfps1_dn2, locals.var_qsfps1_dn3, locals.var_qsfps1_dn4, locals.var_qsfps1_dn7, locals.var_qsfps1_dn9, locals.var_qsfps1_dn10,)
    }
};
        locals.var_qsfps1 = assign15820_e15979;
        locals.var_qsfps1_dn2 = assign15820_e15979_d_n2;
        locals.var_qsfps1_dn3 = assign15820_e15979_d_n3;
        locals.var_qsfps1_dn4 = assign15820_e15979_d_n4;
        locals.var_qsfps1_dn7 = assign15820_e15979_d_n7;
        locals.var_qsfps1_dn9 = assign15820_e15979_d_n9;
        locals.var_qsfps1_dn10 = assign15820_e15979_d_n10;

        let assign15860_e15994: f64 = if p.p78 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard203 = assign15860_e15994;

        locals.var_qgsfps2 = 0.0;
        locals.var_qgsfps2_dn2 = 0.0;
        locals.var_qgsfps2_dn4 = 0.0;
        locals.var_qgsfps2_dn7 = 0.0;
        locals.var_qgsfps2_dn10 = 0.0;
        locals.var_qgsfps2_dn11 = 0.0;

        locals.var_qgdfps2 = 0.0;
        locals.var_qgdfps2_dn2 = 0.0;
        locals.var_qgdfps2_dn4 = 0.0;
        locals.var_qgdfps2_dn7 = 0.0;
        locals.var_qgdfps2_dn10 = 0.0;
        locals.var_qgdfps2_dn11 = 0.0;

        locals.var_qcfps2 = 0.0;
        locals.var_qcfps2_dn2 = 0.0;
        locals.var_qcfps2_dn3 = 0.0;
        locals.var_qcfps2_dn4 = 0.0;
        locals.var_qcfps2_dn7 = 0.0;
        locals.var_qcfps2_dn10 = 0.0;
        locals.var_qcfps2_dn11 = 0.0;

        locals.var_qbfps2 = 0.0;
        locals.var_qbfps2_dn2 = 0.0;
        locals.var_qbfps2_dn3 = 0.0;
        locals.var_qbfps2_dn4 = 0.0;
        locals.var_qbfps2_dn7 = 0.0;
        locals.var_qbfps2_dn10 = 0.0;
        locals.var_qbfps2_dn11 = 0.0;

        locals.var_qsfps2 = 0.0;
        locals.var_qsfps2_dn2 = 0.0;
        locals.var_qsfps2_dn3 = 0.0;
        locals.var_qsfps2_dn4 = 0.0;
        locals.var_qsfps2_dn7 = 0.0;
        locals.var_qsfps2_dn10 = 0.0;
        locals.var_qsfps2_dn11 = 0.0;

        let assign15950_e16005: f64 = if p.p101 > p.p354 { 1.0 } else { 0.0 };
        locals.var_guard204 = assign15950_e16005;

        let (assign15980_e16017, assign15980_e16017_d_n2, assign15980_e16017_d_n4, assign15980_e16017_d_n7, assign15980_e16017_d_n10, assign15980_e16017_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__qgsout, locals.var_fn205_calc_iq__qgsout_dn2, locals.var_fn205_calc_iq__qgsout_dn4, locals.var_fn205_calc_iq__qgsout_dn7, locals.var_fn205_calc_iq__qgsout_dn10, locals.var_fn205_calc_iq__qgsout_dn11,)
    }
};
        locals.var_fn205_calc_iq__qgsout = assign15980_e16017;
        locals.var_fn205_calc_iq__qgsout_dn2 = assign15980_e16017_d_n2;
        locals.var_fn205_calc_iq__qgsout_dn4 = assign15980_e16017_d_n4;
        locals.var_fn205_calc_iq__qgsout_dn7 = assign15980_e16017_d_n7;
        locals.var_fn205_calc_iq__qgsout_dn10 = assign15980_e16017_d_n10;
        locals.var_fn205_calc_iq__qgsout_dn11 = assign15980_e16017_d_n11;

        let (assign15990_e16021, assign15990_e16021_d_n2, assign15990_e16021_d_n4, assign15990_e16021_d_n7, assign15990_e16021_d_n10, assign15990_e16021_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__qgdout, locals.var_fn205_calc_iq__qgdout_dn2, locals.var_fn205_calc_iq__qgdout_dn4, locals.var_fn205_calc_iq__qgdout_dn7, locals.var_fn205_calc_iq__qgdout_dn10, locals.var_fn205_calc_iq__qgdout_dn11,)
    }
};
        locals.var_fn205_calc_iq__qgdout = assign15990_e16021;
        locals.var_fn205_calc_iq__qgdout_dn2 = assign15990_e16021_d_n2;
        locals.var_fn205_calc_iq__qgdout_dn4 = assign15990_e16021_d_n4;
        locals.var_fn205_calc_iq__qgdout_dn7 = assign15990_e16021_d_n7;
        locals.var_fn205_calc_iq__qgdout_dn10 = assign15990_e16021_d_n10;
        locals.var_fn205_calc_iq__qgdout_dn11 = assign15990_e16021_d_n11;

        let (assign16000_e16025, assign16000_e16025_d_n2, assign16000_e16025_d_n3, assign16000_e16025_d_n4, assign16000_e16025_d_n7, assign16000_e16025_d_n10, assign16000_e16025_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__qcout, locals.var_fn205_calc_iq__qcout_dn2, locals.var_fn205_calc_iq__qcout_dn3, locals.var_fn205_calc_iq__qcout_dn4, locals.var_fn205_calc_iq__qcout_dn7, locals.var_fn205_calc_iq__qcout_dn10, locals.var_fn205_calc_iq__qcout_dn11,)
    }
};
        locals.var_fn205_calc_iq__qcout = assign16000_e16025;
        locals.var_fn205_calc_iq__qcout_dn2 = assign16000_e16025_d_n2;
        locals.var_fn205_calc_iq__qcout_dn3 = assign16000_e16025_d_n3;
        locals.var_fn205_calc_iq__qcout_dn4 = assign16000_e16025_d_n4;
        locals.var_fn205_calc_iq__qcout_dn7 = assign16000_e16025_d_n7;
        locals.var_fn205_calc_iq__qcout_dn10 = assign16000_e16025_d_n10;
        locals.var_fn205_calc_iq__qcout_dn11 = assign16000_e16025_d_n11;

        let (assign16010_e16029, assign16010_e16029_d_n2, assign16010_e16029_d_n3, assign16010_e16029_d_n4, assign16010_e16029_d_n7, assign16010_e16029_d_n10, assign16010_e16029_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__qbout, locals.var_fn205_calc_iq__qbout_dn2, locals.var_fn205_calc_iq__qbout_dn3, locals.var_fn205_calc_iq__qbout_dn4, locals.var_fn205_calc_iq__qbout_dn7, locals.var_fn205_calc_iq__qbout_dn10, locals.var_fn205_calc_iq__qbout_dn11,)
    }
};
        locals.var_fn205_calc_iq__qbout = assign16010_e16029;
        locals.var_fn205_calc_iq__qbout_dn2 = assign16010_e16029_d_n2;
        locals.var_fn205_calc_iq__qbout_dn3 = assign16010_e16029_d_n3;
        locals.var_fn205_calc_iq__qbout_dn4 = assign16010_e16029_d_n4;
        locals.var_fn205_calc_iq__qbout_dn7 = assign16010_e16029_d_n7;
        locals.var_fn205_calc_iq__qbout_dn10 = assign16010_e16029_d_n10;
        locals.var_fn205_calc_iq__qbout_dn11 = assign16010_e16029_d_n11;

        let (assign16020_e16033, assign16020_e16033_d_n2, assign16020_e16033_d_n3, assign16020_e16033_d_n4, assign16020_e16033_d_n7, assign16020_e16033_d_n10, assign16020_e16033_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__qsout, locals.var_fn205_calc_iq__qsout_dn2, locals.var_fn205_calc_iq__qsout_dn3, locals.var_fn205_calc_iq__qsout_dn4, locals.var_fn205_calc_iq__qsout_dn7, locals.var_fn205_calc_iq__qsout_dn10, locals.var_fn205_calc_iq__qsout_dn11,)
    }
};
        locals.var_fn205_calc_iq__qsout = assign16020_e16033;
        locals.var_fn205_calc_iq__qsout_dn2 = assign16020_e16033_d_n2;
        locals.var_fn205_calc_iq__qsout_dn3 = assign16020_e16033_d_n3;
        locals.var_fn205_calc_iq__qsout_dn4 = assign16020_e16033_d_n4;
        locals.var_fn205_calc_iq__qsout_dn7 = assign16020_e16033_d_n7;
        locals.var_fn205_calc_iq__qsout_dn10 = assign16020_e16033_d_n10;
        locals.var_fn205_calc_iq__qsout_dn11 = assign16020_e16033_d_n11;

        let (assign16030_e16037, assign16030_e16037_d_n4, assign16030_e16037_d_n10, assign16030_e16037_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__vtdibl, locals.var_fn205_calc_iq__vtdibl_dn4, locals.var_fn205_calc_iq__vtdibl_dn10, locals.var_fn205_calc_iq__vtdibl_dn11,)
    }
};
        locals.var_fn205_calc_iq__vtdibl = assign16030_e16037;
        locals.var_fn205_calc_iq__vtdibl_dn4 = assign16030_e16037_d_n4;
        locals.var_fn205_calc_iq__vtdibl_dn10 = assign16030_e16037_d_n10;
        locals.var_fn205_calc_iq__vtdibl_dn11 = assign16030_e16037_d_n11;

        let (assign16040_e16041, assign16040_e16041_d_n2, assign16040_e16041_d_n3, assign16040_e16041_d_n4, assign16040_e16041_d_n7, assign16040_e16041_d_n10, assign16040_e16041_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__vdsat1, locals.var_fn205_calc_iq__vdsat1_dn2, locals.var_fn205_calc_iq__vdsat1_dn3, locals.var_fn205_calc_iq__vdsat1_dn4, locals.var_fn205_calc_iq__vdsat1_dn7, locals.var_fn205_calc_iq__vdsat1_dn10, locals.var_fn205_calc_iq__vdsat1_dn11,)
    }
};
        locals.var_fn205_calc_iq__vdsat1 = assign16040_e16041;
        locals.var_fn205_calc_iq__vdsat1_dn2 = assign16040_e16041_d_n2;
        locals.var_fn205_calc_iq__vdsat1_dn3 = assign16040_e16041_d_n3;
        locals.var_fn205_calc_iq__vdsat1_dn4 = assign16040_e16041_d_n4;
        locals.var_fn205_calc_iq__vdsat1_dn7 = assign16040_e16041_d_n7;
        locals.var_fn205_calc_iq__vdsat1_dn10 = assign16040_e16041_d_n10;
        locals.var_fn205_calc_iq__vdsat1_dn11 = assign16040_e16041_d_n11;

        let (assign16050_e16045, assign16050_e16045_d_n2, assign16050_e16045_d_n7, assign16050_e16045_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        (locals.var_vgsfps2, locals.var_vgsfps2_dn2, locals.var_vgsfps2_dn7, locals.var_vgsfps2_dn11,)
    } else {
        (locals.var_fn205_calc_iq__vgsin, locals.var_fn205_calc_iq__vgsin_dn2, locals.var_fn205_calc_iq__vgsin_dn7, locals.var_fn205_calc_iq__vgsin_dn11,)
    }
};
        locals.var_fn205_calc_iq__vgsin = assign16050_e16045;
        locals.var_fn205_calc_iq__vgsin_dn2 = assign16050_e16045_d_n2;
        locals.var_fn205_calc_iq__vgsin_dn7 = assign16050_e16045_d_n7;
        locals.var_fn205_calc_iq__vgsin_dn11 = assign16050_e16045_d_n11;

        let (assign16060_e16049, assign16060_e16049_d_n10, assign16060_e16049_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        (locals.var_vdsfps2, locals.var_vdsfps2_dn10, locals.var_vdsfps2_dn11,)
    } else {
        (locals.var_fn205_calc_iq__vdsin, locals.var_fn205_calc_iq__vdsin_dn10, locals.var_fn205_calc_iq__vdsin_dn11,)
    }
};
        locals.var_fn205_calc_iq__vdsin = assign16060_e16049;
        locals.var_fn205_calc_iq__vdsin_dn10 = assign16060_e16049_d_n10;
        locals.var_fn205_calc_iq__vdsin_dn11 = assign16060_e16049_d_n11;

        let (assign16070_e16053,) = {
    if (locals.var_guard204 != 0.0) {
        (p.p107,)
    } else {
        (locals.var_fn205_calc_iq__qcbflag,)
    }
};
        locals.var_fn205_calc_iq__qcbflag = assign16070_e16053;

    }

    pub(super) fn stamp_transient_block_40(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign16080_e16057, assign16080_e16057_d_n2, assign16080_e16057_d_n7, assign16080_e16057_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        (locals.var_vcfps2, locals.var_vcfps2_dn2, locals.var_vcfps2_dn7, locals.var_vcfps2_dn11,)
    } else {
        (locals.var_fn205_calc_iq__vcin, locals.var_fn205_calc_iq__vcin_dn2, locals.var_fn205_calc_iq__vcin_dn7, locals.var_fn205_calc_iq__vcin_dn11,)
    }
};
        locals.var_fn205_calc_iq__vcin = assign16080_e16057;
        locals.var_fn205_calc_iq__vcin_dn2 = assign16080_e16057_d_n2;
        locals.var_fn205_calc_iq__vcin_dn7 = assign16080_e16057_d_n7;
        locals.var_fn205_calc_iq__vcin_dn11 = assign16080_e16057_d_n11;

        let (assign16090_e16061, assign16090_e16061_d_n3, assign16090_e16061_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        (locals.var_vbfps2, locals.var_vbfps2_dn3, locals.var_vbfps2_dn11,)
    } else {
        (locals.var_fn205_calc_iq__vbin, locals.var_fn205_calc_iq__vbin_dn3, locals.var_fn205_calc_iq__vbin_dn11,)
    }
};
        locals.var_fn205_calc_iq__vbin = assign16090_e16061;
        locals.var_fn205_calc_iq__vbin_dn3 = assign16090_e16061_d_n3;
        locals.var_fn205_calc_iq__vbin_dn11 = assign16090_e16061_d_n11;

        let (assign16100_e16065,) = {
    if (locals.var_guard204 != 0.0) {
        (p.p105,)
    } else {
        (locals.var_fn205_calc_iq__qgsflag,)
    }
};
        locals.var_fn205_calc_iq__qgsflag = assign16100_e16065;

        let (assign16110_e16069, assign16110_e16069_d_n4,) = {
    if (locals.var_guard204 != 0.0) {
        (locals.var_tdut, locals.var_tdut_dn4,)
    } else {
        (locals.var_fn205_calc_iq__tambin, locals.var_fn205_calc_iq__tambin_dn4,)
    }
};
        locals.var_fn205_calc_iq__tambin = assign16110_e16069;
        locals.var_fn205_calc_iq__tambin_dn4 = assign16110_e16069_d_n4;

        let (assign16120_e16073,) = {
    if (locals.var_guard204 != 0.0) {
        (locals.var_tnomk,)
    } else {
        (locals.var_fn205_calc_iq__tnomin,)
    }
};
        locals.var_fn205_calc_iq__tnomin = assign16120_e16073;

        let (assign16130_e16077, assign16130_e16077_d_n4,) = {
    if (locals.var_guard204 != 0.0) {
        (locals.var_phit, locals.var_phit_dn4,)
    } else {
        (locals.var_fn205_calc_iq__phitin, locals.var_fn205_calc_iq__phitin_dn4,)
    }
};
        locals.var_fn205_calc_iq__phitin = assign16130_e16077;
        locals.var_fn205_calc_iq__phitin_dn4 = assign16130_e16077_d_n4;

        let (assign16140_e16081,) = {
    if (locals.var_guard204 != 0.0) {
        (p.p0,)
    } else {
        (locals.var_fn205_calc_iq__w,)
    }
};
        locals.var_fn205_calc_iq__w = assign16140_e16081;

        let (assign16150_e16085,) = {
    if (locals.var_guard204 != 0.0) {
        (p.p101,)
    } else {
        (locals.var_fn205_calc_iq__lin,)
    }
};
        locals.var_fn205_calc_iq__lin = assign16150_e16085;

        let (assign16160_e16089, assign16160_e16089_d_n4,) = {
    if (locals.var_guard204 != 0.0) {
        (locals.var_cgfps2t, locals.var_cgfps2t_dn4,)
    } else {
        (locals.var_fn205_calc_iq__cgin, locals.var_fn205_calc_iq__cgin_dn4,)
    }
};
        locals.var_fn205_calc_iq__cgin = assign16160_e16089;
        locals.var_fn205_calc_iq__cgin_dn4 = assign16160_e16089_d_n4;

        let (assign16170_e16093,) = {
    if (locals.var_guard204 != 0.0) {
        (p.p106,)
    } else {
        (locals.var_fn205_calc_iq__cs,)
    }
};
        locals.var_fn205_calc_iq__cs = assign16170_e16093;

        let (assign16180_e16097, assign16180_e16097_d_n4,) = {
    if (locals.var_guard204 != 0.0) {
        (locals.var_ccfps2t, locals.var_ccfps2t_dn4,)
    } else {
        (locals.var_fn205_calc_iq__cc, locals.var_fn205_calc_iq__cc_dn4,)
    }
};
        locals.var_fn205_calc_iq__cc = assign16180_e16097;
        locals.var_fn205_calc_iq__cc_dn4 = assign16180_e16097_d_n4;

        let (assign16190_e16101, assign16190_e16101_d_n4,) = {
    if (locals.var_guard204 != 0.0) {
        (locals.var_cbfps2t, locals.var_cbfps2t_dn4,)
    } else {
        (locals.var_fn205_calc_iq__cb, locals.var_fn205_calc_iq__cb_dn4,)
    }
};
        locals.var_fn205_calc_iq__cb = assign16190_e16101;
        locals.var_fn205_calc_iq__cb_dn4 = assign16190_e16101_d_n4;

        let (assign16200_e16105,) = {
    if (locals.var_guard204 != 0.0) {
        (p.p102,)
    } else {
        (locals.var_fn205_calc_iq__vto,)
    }
};
        locals.var_fn205_calc_iq__vto = assign16200_e16105;

        let (assign16210_e16109,) = {
    if (locals.var_guard204 != 0.0) {
        (p.p116,)
    } else {
        (locals.var_fn205_calc_iq__ss,)
    }
};
        locals.var_fn205_calc_iq__ss = assign16210_e16109;

        let (assign16220_e16113,) = {
    if (locals.var_guard204 != 0.0) {
        (p.p115,)
    } else {
        (locals.var_fn205_calc_iq__delta1,)
    }
};
        locals.var_fn205_calc_iq__delta1 = assign16220_e16113;

        let (assign16230_e16117,) = {
    if (locals.var_guard204 != 0.0) {
        (0.0,)
    } else {
        (locals.var_fn205_calc_iq__delta2,)
    }
};
        locals.var_fn205_calc_iq__delta2 = assign16230_e16117;

        let (assign16240_e16121,) = {
    if (locals.var_guard204 != 0.0) {
        (p.p117,)
    } else {
        (locals.var_fn205_calc_iq__nd,)
    }
};
        locals.var_fn205_calc_iq__nd = assign16240_e16121;

        let (assign16250_e16125,) = {
    if (locals.var_guard204 != 0.0) {
        (p.p121,)
    } else {
        (locals.var_fn205_calc_iq__alpha,)
    }
};
        locals.var_fn205_calc_iq__alpha = assign16250_e16125;

        let (assign16260_e16129,) = {
    if (locals.var_guard204 != 0.0) {
        (p.p112,)
    } else {
        (locals.var_fn205_calc_iq__vel0,)
    }
};
        locals.var_fn205_calc_iq__vel0 = assign16260_e16129;

        let (assign16270_e16133,) = {
    if (locals.var_guard204 != 0.0) {
        (p.p113,)
    } else {
        (locals.var_fn205_calc_iq__mu0,)
    }
};
        locals.var_fn205_calc_iq__mu0 = assign16270_e16133;

        let (assign16280_e16137,) = {
    if (locals.var_guard204 != 0.0) {
        (p.p114,)
    } else {
        (locals.var_fn205_calc_iq__beta,)
    }
};
        locals.var_fn205_calc_iq__beta = assign16280_e16137;

        let (assign16290_e16141,) = {
    if (locals.var_guard204 != 0.0) {
        (p.p120,)
    } else {
        (locals.var_fn205_calc_iq__mtheta,)
    }
};
        locals.var_fn205_calc_iq__mtheta = assign16290_e16141;

        let (assign16300_e16145,) = {
    if (locals.var_guard204 != 0.0) {
        (p.p119,)
    } else {
        (locals.var_fn205_calc_iq__vtheta,)
    }
};
        locals.var_fn205_calc_iq__vtheta = assign16300_e16145;

        let (assign16310_e16149,) = {
    if (locals.var_guard204 != 0.0) {
        (p.p118,)
    } else {
        (locals.var_fn205_calc_iq__vtzeta,)
    }
};
        locals.var_fn205_calc_iq__vtzeta = assign16310_e16149;

        let (assign16320_e16153,) = {
    if (locals.var_guard204 != 0.0) {
        (p.p39,)
    } else {
        (locals.var_fn205_calc_iq__dibsat,)
    }
};
        locals.var_fn205_calc_iq__dibsat = assign16320_e16153;

        let (assign16330_e16157,) = {
    if (locals.var_guard204 != 0.0) {
        (p.p47,)
    } else {
        (locals.var_fn205_calc_iq__epsilon,)
    }
};
        locals.var_fn205_calc_iq__epsilon = assign16330_e16157;

        let (assign16340_e16161,) = {
    if (locals.var_guard204 != 0.0) {
        (p.p45,)
    } else {
        (locals.var_fn205_calc_iq__vzeta,)
    }
};
        locals.var_fn205_calc_iq__vzeta = assign16340_e16161;

        let (assign16350_e16165,) = {
    if (locals.var_guard204 != 0.0) {
        (p.p42,)
    } else {
        (locals.var_fn205_calc_iq__lambda,)
    }
};
        locals.var_fn205_calc_iq__lambda = assign16350_e16165;

        let (assign16360_e16169,) = {
    if (locals.var_guard204 != 0.0) {
        (p.p2,)
    } else {
        (locals.var_fn205_calc_iq__ngf,)
    }
};
        locals.var_fn205_calc_iq__ngf = assign16360_e16169;

        let (assign16370_e16173,) = {
    if (locals.var_guard204 != 0.0) {
        (p.p6,)
    } else {
        (locals.var_fn205_calc_iq__type,)
    }
};
        locals.var_fn205_calc_iq__type = assign16370_e16173;

        let (assign16380_e16177,) = {
    if (locals.var_guard204 != 0.0) {
        (1.0,)
    } else {
        (locals.var_fn205_calc_iq__trapfracdl,)
    }
};
        locals.var_fn205_calc_iq__trapfracdl = assign16380_e16177;

        let (assign16390_e16181, assign16390_e16181_d_n4,) = {
    if (locals.var_guard204 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__alpha_phit, locals.var_fn205_calc_iq__alpha_phit_dn4,)
    }
};
        locals.var_fn205_calc_iq__alpha_phit = assign16390_e16181;
        locals.var_fn205_calc_iq__alpha_phit_dn4 = assign16390_e16181_d_n4;

        let (assign16400_e16185, assign16400_e16185_d_n10, assign16400_e16185_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        (0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__delta, locals.var_fn205_calc_iq__delta_dn10, locals.var_fn205_calc_iq__delta_dn11,)
    }
};
        locals.var_fn205_calc_iq__delta = assign16400_e16185;
        locals.var_fn205_calc_iq__delta_dn10 = assign16400_e16185_d_n10;
        locals.var_fn205_calc_iq__delta_dn11 = assign16400_e16185_d_n11;

        let (assign16410_e16189, assign16410_e16189_d_n4, assign16410_e16189_d_n10, assign16410_e16189_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__n, locals.var_fn205_calc_iq__n_dn4, locals.var_fn205_calc_iq__n_dn10, locals.var_fn205_calc_iq__n_dn11,)
    }
};
        locals.var_fn205_calc_iq__n = assign16410_e16189;
        locals.var_fn205_calc_iq__n_dn4 = assign16410_e16189_d_n4;
        locals.var_fn205_calc_iq__n_dn10 = assign16410_e16189_d_n10;
        locals.var_fn205_calc_iq__n_dn11 = assign16410_e16189_d_n11;

        let (assign16420_e16193, assign16420_e16193_d_n4,) = {
    if (locals.var_guard204 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__vtof, locals.var_fn205_calc_iq__vtof_dn4,)
    }
};
        locals.var_fn205_calc_iq__vtof = assign16420_e16193;
        locals.var_fn205_calc_iq__vtof_dn4 = assign16420_e16193_d_n4;

        let (assign16430_e16197, assign16430_e16197_d_n10, assign16430_e16197_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        (0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__vsatdibl, locals.var_fn205_calc_iq__vsatdibl_dn10, locals.var_fn205_calc_iq__vsatdibl_dn11,)
    }
};
        locals.var_fn205_calc_iq__vsatdibl = assign16430_e16197;
        locals.var_fn205_calc_iq__vsatdibl_dn10 = assign16430_e16197_d_n10;
        locals.var_fn205_calc_iq__vsatdibl_dn11 = assign16430_e16197_d_n11;

        let (assign16440_e16201, assign16440_e16201_d_n2, assign16440_e16201_d_n3, assign16440_e16201_d_n4, assign16440_e16201_d_n7, assign16440_e16201_d_n10, assign16440_e16201_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__ffs, locals.var_fn205_calc_iq__ffs_dn2, locals.var_fn205_calc_iq__ffs_dn3, locals.var_fn205_calc_iq__ffs_dn4, locals.var_fn205_calc_iq__ffs_dn7, locals.var_fn205_calc_iq__ffs_dn10, locals.var_fn205_calc_iq__ffs_dn11,)
    }
};
        locals.var_fn205_calc_iq__ffs = assign16440_e16201;
        locals.var_fn205_calc_iq__ffs_dn2 = assign16440_e16201_d_n2;
        locals.var_fn205_calc_iq__ffs_dn3 = assign16440_e16201_d_n3;
        locals.var_fn205_calc_iq__ffs_dn4 = assign16440_e16201_d_n4;
        locals.var_fn205_calc_iq__ffs_dn7 = assign16440_e16201_d_n7;
        locals.var_fn205_calc_iq__ffs_dn10 = assign16440_e16201_d_n10;
        locals.var_fn205_calc_iq__ffs_dn11 = assign16440_e16201_d_n11;

        let (assign16450_e16205, assign16450_e16205_d_n4, assign16450_e16205_d_n10, assign16450_e16205_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__two_n_phit, locals.var_fn205_calc_iq__two_n_phit_dn4, locals.var_fn205_calc_iq__two_n_phit_dn10, locals.var_fn205_calc_iq__two_n_phit_dn11,)
    }
};
        locals.var_fn205_calc_iq__two_n_phit = assign16450_e16205;
        locals.var_fn205_calc_iq__two_n_phit_dn4 = assign16450_e16205_d_n4;
        locals.var_fn205_calc_iq__two_n_phit_dn10 = assign16450_e16205_d_n10;
        locals.var_fn205_calc_iq__two_n_phit_dn11 = assign16450_e16205_d_n11;

        let (assign16460_e16209, assign16460_e16209_d_n4, assign16460_e16209_d_n10, assign16460_e16209_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__qref, locals.var_fn205_calc_iq__qref_dn4, locals.var_fn205_calc_iq__qref_dn10, locals.var_fn205_calc_iq__qref_dn11,)
    }
};
        locals.var_fn205_calc_iq__qref = assign16460_e16209;
        locals.var_fn205_calc_iq__qref_dn4 = assign16460_e16209_d_n4;
        locals.var_fn205_calc_iq__qref_dn10 = assign16460_e16209_d_n10;
        locals.var_fn205_calc_iq__qref_dn11 = assign16460_e16209_d_n11;

        let (assign16470_e16213, assign16470_e16213_d_n2, assign16470_e16213_d_n3, assign16470_e16213_d_n4, assign16470_e16213_d_n7, assign16470_e16213_d_n10, assign16470_e16213_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__etas, locals.var_fn205_calc_iq__etas_dn2, locals.var_fn205_calc_iq__etas_dn3, locals.var_fn205_calc_iq__etas_dn4, locals.var_fn205_calc_iq__etas_dn7, locals.var_fn205_calc_iq__etas_dn10, locals.var_fn205_calc_iq__etas_dn11,)
    }
};
        locals.var_fn205_calc_iq__etas = assign16470_e16213;
        locals.var_fn205_calc_iq__etas_dn2 = assign16470_e16213_d_n2;
        locals.var_fn205_calc_iq__etas_dn3 = assign16470_e16213_d_n3;
        locals.var_fn205_calc_iq__etas_dn4 = assign16470_e16213_d_n4;
        locals.var_fn205_calc_iq__etas_dn7 = assign16470_e16213_d_n7;
        locals.var_fn205_calc_iq__etas_dn10 = assign16470_e16213_d_n10;
        locals.var_fn205_calc_iq__etas_dn11 = assign16470_e16213_d_n11;

        let (assign16480_e16217, assign16480_e16217_d_n2, assign16480_e16217_d_n3, assign16480_e16217_d_n4, assign16480_e16217_d_n7, assign16480_e16217_d_n10, assign16480_e16217_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__qinvs, locals.var_fn205_calc_iq__qinvs_dn2, locals.var_fn205_calc_iq__qinvs_dn3, locals.var_fn205_calc_iq__qinvs_dn4, locals.var_fn205_calc_iq__qinvs_dn7, locals.var_fn205_calc_iq__qinvs_dn10, locals.var_fn205_calc_iq__qinvs_dn11,)
    }
};
        locals.var_fn205_calc_iq__qinvs = assign16480_e16217;
        locals.var_fn205_calc_iq__qinvs_dn2 = assign16480_e16217_d_n2;
        locals.var_fn205_calc_iq__qinvs_dn3 = assign16480_e16217_d_n3;
        locals.var_fn205_calc_iq__qinvs_dn4 = assign16480_e16217_d_n4;
        locals.var_fn205_calc_iq__qinvs_dn7 = assign16480_e16217_d_n7;
        locals.var_fn205_calc_iq__qinvs_dn10 = assign16480_e16217_d_n10;
        locals.var_fn205_calc_iq__qinvs_dn11 = assign16480_e16217_d_n11;

        let (assign16490_e16221, assign16490_e16221_d_n2, assign16490_e16221_d_n3, assign16490_e16221_d_n4, assign16490_e16221_d_n7, assign16490_e16221_d_n10, assign16490_e16221_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__muf, locals.var_fn205_calc_iq__muf_dn2, locals.var_fn205_calc_iq__muf_dn3, locals.var_fn205_calc_iq__muf_dn4, locals.var_fn205_calc_iq__muf_dn7, locals.var_fn205_calc_iq__muf_dn10, locals.var_fn205_calc_iq__muf_dn11,)
    }
};
        locals.var_fn205_calc_iq__muf = assign16490_e16221;
        locals.var_fn205_calc_iq__muf_dn2 = assign16490_e16221_d_n2;
        locals.var_fn205_calc_iq__muf_dn3 = assign16490_e16221_d_n3;
        locals.var_fn205_calc_iq__muf_dn4 = assign16490_e16221_d_n4;
        locals.var_fn205_calc_iq__muf_dn7 = assign16490_e16221_d_n7;
        locals.var_fn205_calc_iq__muf_dn10 = assign16490_e16221_d_n10;
        locals.var_fn205_calc_iq__muf_dn11 = assign16490_e16221_d_n11;

        let (assign16500_e16225, assign16500_e16225_d_n2, assign16500_e16225_d_n3, assign16500_e16225_d_n4, assign16500_e16225_d_n7, assign16500_e16225_d_n10, assign16500_e16225_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__vx, locals.var_fn205_calc_iq__vx_dn2, locals.var_fn205_calc_iq__vx_dn3, locals.var_fn205_calc_iq__vx_dn4, locals.var_fn205_calc_iq__vx_dn7, locals.var_fn205_calc_iq__vx_dn10, locals.var_fn205_calc_iq__vx_dn11,)
    }
};
        locals.var_fn205_calc_iq__vx = assign16500_e16225;
        locals.var_fn205_calc_iq__vx_dn2 = assign16500_e16225_d_n2;
        locals.var_fn205_calc_iq__vx_dn3 = assign16500_e16225_d_n3;
        locals.var_fn205_calc_iq__vx_dn4 = assign16500_e16225_d_n4;
        locals.var_fn205_calc_iq__vx_dn7 = assign16500_e16225_d_n7;
        locals.var_fn205_calc_iq__vx_dn10 = assign16500_e16225_d_n10;
        locals.var_fn205_calc_iq__vx_dn11 = assign16500_e16225_d_n11;

        let (assign16520_e16233, assign16520_e16233_d_n4,) = {
    if (locals.var_guard204 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__n0, locals.var_fn205_calc_iq__n0_dn4,)
    }
};
        locals.var_fn205_calc_iq__n0 = assign16520_e16233;
        locals.var_fn205_calc_iq__n0_dn4 = assign16520_e16233_d_n4;

        let (assign16530_e16237, assign16530_e16237_d_n2, assign16530_e16237_d_n4, assign16530_e16237_d_n7, assign16530_e16237_d_n10, assign16530_e16237_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__ffs0, locals.var_fn205_calc_iq__ffs0_dn2, locals.var_fn205_calc_iq__ffs0_dn4, locals.var_fn205_calc_iq__ffs0_dn7, locals.var_fn205_calc_iq__ffs0_dn10, locals.var_fn205_calc_iq__ffs0_dn11,)
    }
};
        locals.var_fn205_calc_iq__ffs0 = assign16530_e16237;
        locals.var_fn205_calc_iq__ffs0_dn2 = assign16530_e16237_d_n2;
        locals.var_fn205_calc_iq__ffs0_dn4 = assign16530_e16237_d_n4;
        locals.var_fn205_calc_iq__ffs0_dn7 = assign16530_e16237_d_n7;
        locals.var_fn205_calc_iq__ffs0_dn10 = assign16530_e16237_d_n10;
        locals.var_fn205_calc_iq__ffs0_dn11 = assign16530_e16237_d_n11;

        let (assign16540_e16241, assign16540_e16241_d_n4,) = {
    if (locals.var_guard204 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__two_n_phit0, locals.var_fn205_calc_iq__two_n_phit0_dn4,)
    }
};
        locals.var_fn205_calc_iq__two_n_phit0 = assign16540_e16241;
        locals.var_fn205_calc_iq__two_n_phit0_dn4 = assign16540_e16241_d_n4;

        let (assign16550_e16245, assign16550_e16245_d_n4,) = {
    if (locals.var_guard204 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__qref0, locals.var_fn205_calc_iq__qref0_dn4,)
    }
};
        locals.var_fn205_calc_iq__qref0 = assign16550_e16245;
        locals.var_fn205_calc_iq__qref0_dn4 = assign16550_e16245_d_n4;

        let (assign16560_e16249, assign16560_e16249_d_n2, assign16560_e16249_d_n4, assign16560_e16249_d_n7, assign16560_e16249_d_n10, assign16560_e16249_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__etas0, locals.var_fn205_calc_iq__etas0_dn2, locals.var_fn205_calc_iq__etas0_dn4, locals.var_fn205_calc_iq__etas0_dn7, locals.var_fn205_calc_iq__etas0_dn10, locals.var_fn205_calc_iq__etas0_dn11,)
    }
};
        locals.var_fn205_calc_iq__etas0 = assign16560_e16249;
        locals.var_fn205_calc_iq__etas0_dn2 = assign16560_e16249_d_n2;
        locals.var_fn205_calc_iq__etas0_dn4 = assign16560_e16249_d_n4;
        locals.var_fn205_calc_iq__etas0_dn7 = assign16560_e16249_d_n7;
        locals.var_fn205_calc_iq__etas0_dn10 = assign16560_e16249_d_n10;
        locals.var_fn205_calc_iq__etas0_dn11 = assign16560_e16249_d_n11;

    }

    pub(super) fn stamp_transient_block_41(
        locals: &mut StampLocals,
    ) {
        let (assign16570_e16253, assign16570_e16253_d_n2, assign16570_e16253_d_n4, assign16570_e16253_d_n7, assign16570_e16253_d_n10, assign16570_e16253_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__qinvs0, locals.var_fn205_calc_iq__qinvs0_dn2, locals.var_fn205_calc_iq__qinvs0_dn4, locals.var_fn205_calc_iq__qinvs0_dn7, locals.var_fn205_calc_iq__qinvs0_dn10, locals.var_fn205_calc_iq__qinvs0_dn11,)
    }
};
        locals.var_fn205_calc_iq__qinvs0 = assign16570_e16253;
        locals.var_fn205_calc_iq__qinvs0_dn2 = assign16570_e16253_d_n2;
        locals.var_fn205_calc_iq__qinvs0_dn4 = assign16570_e16253_d_n4;
        locals.var_fn205_calc_iq__qinvs0_dn7 = assign16570_e16253_d_n7;
        locals.var_fn205_calc_iq__qinvs0_dn10 = assign16570_e16253_d_n10;
        locals.var_fn205_calc_iq__qinvs0_dn11 = assign16570_e16253_d_n11;

        let (assign16580_e16257, assign16580_e16257_d_n4,) = {
    if (locals.var_guard204 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__muf0, locals.var_fn205_calc_iq__muf0_dn4,)
    }
};
        locals.var_fn205_calc_iq__muf0 = assign16580_e16257;
        locals.var_fn205_calc_iq__muf0_dn4 = assign16580_e16257_d_n4;

        let (assign16590_e16261, assign16590_e16261_d_n4,) = {
    if (locals.var_guard204 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__vx0, locals.var_fn205_calc_iq__vx0_dn4,)
    }
};
        locals.var_fn205_calc_iq__vx0 = assign16590_e16261;
        locals.var_fn205_calc_iq__vx0_dn4 = assign16590_e16261_d_n4;

        let (assign16600_e16265, assign16600_e16265_d_n4,) = {
    if (locals.var_guard204 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__tfacmobin, locals.var_fn205_calc_iq__tfacmobin_dn4,)
    }
};
        locals.var_fn205_calc_iq__tfacmobin = assign16600_e16265;
        locals.var_fn205_calc_iq__tfacmobin_dn4 = assign16600_e16265_d_n4;

        let (assign16610_e16269, assign16610_e16269_d_n2, assign16610_e16269_d_n3, assign16610_e16269_d_n4, assign16610_e16269_d_n7, assign16610_e16269_d_n10, assign16610_e16269_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__ff, locals.var_fn205_calc_iq__ff_dn2, locals.var_fn205_calc_iq__ff_dn3, locals.var_fn205_calc_iq__ff_dn4, locals.var_fn205_calc_iq__ff_dn7, locals.var_fn205_calc_iq__ff_dn10, locals.var_fn205_calc_iq__ff_dn11,)
    }
};
        locals.var_fn205_calc_iq__ff = assign16610_e16269;
        locals.var_fn205_calc_iq__ff_dn2 = assign16610_e16269_d_n2;
        locals.var_fn205_calc_iq__ff_dn3 = assign16610_e16269_d_n3;
        locals.var_fn205_calc_iq__ff_dn4 = assign16610_e16269_d_n4;
        locals.var_fn205_calc_iq__ff_dn7 = assign16610_e16269_d_n7;
        locals.var_fn205_calc_iq__ff_dn10 = assign16610_e16269_d_n10;
        locals.var_fn205_calc_iq__ff_dn11 = assign16610_e16269_d_n11;

        let (assign16620_e16273, assign16620_e16273_d_n2, assign16620_e16273_d_n3, assign16620_e16273_d_n4, assign16620_e16273_d_n7, assign16620_e16273_d_n10, assign16620_e16273_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__eta, locals.var_fn205_calc_iq__eta_dn2, locals.var_fn205_calc_iq__eta_dn3, locals.var_fn205_calc_iq__eta_dn4, locals.var_fn205_calc_iq__eta_dn7, locals.var_fn205_calc_iq__eta_dn10, locals.var_fn205_calc_iq__eta_dn11,)
    }
};
        locals.var_fn205_calc_iq__eta = assign16620_e16273;
        locals.var_fn205_calc_iq__eta_dn2 = assign16620_e16273_d_n2;
        locals.var_fn205_calc_iq__eta_dn3 = assign16620_e16273_d_n3;
        locals.var_fn205_calc_iq__eta_dn4 = assign16620_e16273_d_n4;
        locals.var_fn205_calc_iq__eta_dn7 = assign16620_e16273_d_n7;
        locals.var_fn205_calc_iq__eta_dn10 = assign16620_e16273_d_n10;
        locals.var_fn205_calc_iq__eta_dn11 = assign16620_e16273_d_n11;

        let (assign16630_e16277, assign16630_e16277_d_n2, assign16630_e16277_d_n3, assign16630_e16277_d_n4, assign16630_e16277_d_n7, assign16630_e16277_d_n10, assign16630_e16277_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__qinvv, locals.var_fn205_calc_iq__qinvv_dn2, locals.var_fn205_calc_iq__qinvv_dn3, locals.var_fn205_calc_iq__qinvv_dn4, locals.var_fn205_calc_iq__qinvv_dn7, locals.var_fn205_calc_iq__qinvv_dn10, locals.var_fn205_calc_iq__qinvv_dn11,)
    }
};
        locals.var_fn205_calc_iq__qinvv = assign16630_e16277;
        locals.var_fn205_calc_iq__qinvv_dn2 = assign16630_e16277_d_n2;
        locals.var_fn205_calc_iq__qinvv_dn3 = assign16630_e16277_d_n3;
        locals.var_fn205_calc_iq__qinvv_dn4 = assign16630_e16277_d_n4;
        locals.var_fn205_calc_iq__qinvv_dn7 = assign16630_e16277_d_n7;
        locals.var_fn205_calc_iq__qinvv_dn10 = assign16630_e16277_d_n10;
        locals.var_fn205_calc_iq__qinvv_dn11 = assign16630_e16277_d_n11;

        let (assign16640_e16281, assign16640_e16281_d_n2, assign16640_e16281_d_n4, assign16640_e16281_d_n7, assign16640_e16281_d_n10, assign16640_e16281_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__ff0, locals.var_fn205_calc_iq__ff0_dn2, locals.var_fn205_calc_iq__ff0_dn4, locals.var_fn205_calc_iq__ff0_dn7, locals.var_fn205_calc_iq__ff0_dn10, locals.var_fn205_calc_iq__ff0_dn11,)
    }
};
        locals.var_fn205_calc_iq__ff0 = assign16640_e16281;
        locals.var_fn205_calc_iq__ff0_dn2 = assign16640_e16281_d_n2;
        locals.var_fn205_calc_iq__ff0_dn4 = assign16640_e16281_d_n4;
        locals.var_fn205_calc_iq__ff0_dn7 = assign16640_e16281_d_n7;
        locals.var_fn205_calc_iq__ff0_dn10 = assign16640_e16281_d_n10;
        locals.var_fn205_calc_iq__ff0_dn11 = assign16640_e16281_d_n11;

        let (assign16650_e16285, assign16650_e16285_d_n2, assign16650_e16285_d_n4, assign16650_e16285_d_n7, assign16650_e16285_d_n10, assign16650_e16285_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__eta0, locals.var_fn205_calc_iq__eta0_dn2, locals.var_fn205_calc_iq__eta0_dn4, locals.var_fn205_calc_iq__eta0_dn7, locals.var_fn205_calc_iq__eta0_dn10, locals.var_fn205_calc_iq__eta0_dn11,)
    }
};
        locals.var_fn205_calc_iq__eta0 = assign16650_e16285;
        locals.var_fn205_calc_iq__eta0_dn2 = assign16650_e16285_d_n2;
        locals.var_fn205_calc_iq__eta0_dn4 = assign16650_e16285_d_n4;
        locals.var_fn205_calc_iq__eta0_dn7 = assign16650_e16285_d_n7;
        locals.var_fn205_calc_iq__eta0_dn10 = assign16650_e16285_d_n10;
        locals.var_fn205_calc_iq__eta0_dn11 = assign16650_e16285_d_n11;

        let (assign16660_e16289, assign16660_e16289_d_n2, assign16660_e16289_d_n4, assign16660_e16289_d_n7, assign16660_e16289_d_n10, assign16660_e16289_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__qinvv0, locals.var_fn205_calc_iq__qinvv0_dn2, locals.var_fn205_calc_iq__qinvv0_dn4, locals.var_fn205_calc_iq__qinvv0_dn7, locals.var_fn205_calc_iq__qinvv0_dn10, locals.var_fn205_calc_iq__qinvv0_dn11,)
    }
};
        locals.var_fn205_calc_iq__qinvv0 = assign16660_e16289;
        locals.var_fn205_calc_iq__qinvv0_dn2 = assign16660_e16289_d_n2;
        locals.var_fn205_calc_iq__qinvv0_dn4 = assign16660_e16289_d_n4;
        locals.var_fn205_calc_iq__qinvv0_dn7 = assign16660_e16289_d_n7;
        locals.var_fn205_calc_iq__qinvv0_dn10 = assign16660_e16289_d_n10;
        locals.var_fn205_calc_iq__qinvv0_dn11 = assign16660_e16289_d_n11;

        let (assign16670_e16293, assign16670_e16293_d_n2, assign16670_e16293_d_n3, assign16670_e16293_d_n4, assign16670_e16293_d_n7, assign16670_e16293_d_n10, assign16670_e16293_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__vdsats, locals.var_fn205_calc_iq__vdsats_dn2, locals.var_fn205_calc_iq__vdsats_dn3, locals.var_fn205_calc_iq__vdsats_dn4, locals.var_fn205_calc_iq__vdsats_dn7, locals.var_fn205_calc_iq__vdsats_dn10, locals.var_fn205_calc_iq__vdsats_dn11,)
    }
};
        locals.var_fn205_calc_iq__vdsats = assign16670_e16293;
        locals.var_fn205_calc_iq__vdsats_dn2 = assign16670_e16293_d_n2;
        locals.var_fn205_calc_iq__vdsats_dn3 = assign16670_e16293_d_n3;
        locals.var_fn205_calc_iq__vdsats_dn4 = assign16670_e16293_d_n4;
        locals.var_fn205_calc_iq__vdsats_dn7 = assign16670_e16293_d_n7;
        locals.var_fn205_calc_iq__vdsats_dn10 = assign16670_e16293_d_n10;
        locals.var_fn205_calc_iq__vdsats_dn11 = assign16670_e16293_d_n11;

        let (assign16680_e16297, assign16680_e16297_d_n2, assign16680_e16297_d_n3, assign16680_e16297_d_n4, assign16680_e16297_d_n7, assign16680_e16297_d_n10, assign16680_e16297_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__vdsats1, locals.var_fn205_calc_iq__vdsats1_dn2, locals.var_fn205_calc_iq__vdsats1_dn3, locals.var_fn205_calc_iq__vdsats1_dn4, locals.var_fn205_calc_iq__vdsats1_dn7, locals.var_fn205_calc_iq__vdsats1_dn10, locals.var_fn205_calc_iq__vdsats1_dn11,)
    }
};
        locals.var_fn205_calc_iq__vdsats1 = assign16680_e16297;
        locals.var_fn205_calc_iq__vdsats1_dn2 = assign16680_e16297_d_n2;
        locals.var_fn205_calc_iq__vdsats1_dn3 = assign16680_e16297_d_n3;
        locals.var_fn205_calc_iq__vdsats1_dn4 = assign16680_e16297_d_n4;
        locals.var_fn205_calc_iq__vdsats1_dn7 = assign16680_e16297_d_n7;
        locals.var_fn205_calc_iq__vdsats1_dn10 = assign16680_e16297_d_n10;
        locals.var_fn205_calc_iq__vdsats1_dn11 = assign16680_e16297_d_n11;

        let (assign16690_e16301, assign16690_e16301_d_n2, assign16690_e16301_d_n3, assign16690_e16301_d_n4, assign16690_e16301_d_n7, assign16690_e16301_d_n10, assign16690_e16301_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__vdsat, locals.var_fn205_calc_iq__vdsat_dn2, locals.var_fn205_calc_iq__vdsat_dn3, locals.var_fn205_calc_iq__vdsat_dn4, locals.var_fn205_calc_iq__vdsat_dn7, locals.var_fn205_calc_iq__vdsat_dn10, locals.var_fn205_calc_iq__vdsat_dn11,)
    }
};
        locals.var_fn205_calc_iq__vdsat = assign16690_e16301;
        locals.var_fn205_calc_iq__vdsat_dn2 = assign16690_e16301_d_n2;
        locals.var_fn205_calc_iq__vdsat_dn3 = assign16690_e16301_d_n3;
        locals.var_fn205_calc_iq__vdsat_dn4 = assign16690_e16301_d_n4;
        locals.var_fn205_calc_iq__vdsat_dn7 = assign16690_e16301_d_n7;
        locals.var_fn205_calc_iq__vdsat_dn10 = assign16690_e16301_d_n10;
        locals.var_fn205_calc_iq__vdsat_dn11 = assign16690_e16301_d_n11;

        let (assign16700_e16305, assign16700_e16305_d_n2, assign16700_e16305_d_n3, assign16700_e16305_d_n4, assign16700_e16305_d_n7, assign16700_e16305_d_n10, assign16700_e16305_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__fsd, locals.var_fn205_calc_iq__fsd_dn2, locals.var_fn205_calc_iq__fsd_dn3, locals.var_fn205_calc_iq__fsd_dn4, locals.var_fn205_calc_iq__fsd_dn7, locals.var_fn205_calc_iq__fsd_dn10, locals.var_fn205_calc_iq__fsd_dn11,)
    }
};
        locals.var_fn205_calc_iq__fsd = assign16700_e16305;
        locals.var_fn205_calc_iq__fsd_dn2 = assign16700_e16305_d_n2;
        locals.var_fn205_calc_iq__fsd_dn3 = assign16700_e16305_d_n3;
        locals.var_fn205_calc_iq__fsd_dn4 = assign16700_e16305_d_n4;
        locals.var_fn205_calc_iq__fsd_dn7 = assign16700_e16305_d_n7;
        locals.var_fn205_calc_iq__fsd_dn10 = assign16700_e16305_d_n10;
        locals.var_fn205_calc_iq__fsd_dn11 = assign16700_e16305_d_n11;

        let (assign16710_e16309, assign16710_e16309_d_n2, assign16710_e16309_d_n3, assign16710_e16309_d_n4, assign16710_e16309_d_n7, assign16710_e16309_d_n10, assign16710_e16309_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__vdx, locals.var_fn205_calc_iq__vdx_dn2, locals.var_fn205_calc_iq__vdx_dn3, locals.var_fn205_calc_iq__vdx_dn4, locals.var_fn205_calc_iq__vdx_dn7, locals.var_fn205_calc_iq__vdx_dn10, locals.var_fn205_calc_iq__vdx_dn11,)
    }
};
        locals.var_fn205_calc_iq__vdx = assign16710_e16309;
        locals.var_fn205_calc_iq__vdx_dn2 = assign16710_e16309_d_n2;
        locals.var_fn205_calc_iq__vdx_dn3 = assign16710_e16309_d_n3;
        locals.var_fn205_calc_iq__vdx_dn4 = assign16710_e16309_d_n4;
        locals.var_fn205_calc_iq__vdx_dn7 = assign16710_e16309_d_n7;
        locals.var_fn205_calc_iq__vdx_dn10 = assign16710_e16309_d_n10;
        locals.var_fn205_calc_iq__vdx_dn11 = assign16710_e16309_d_n11;

        let (assign16720_e16313, assign16720_e16313_d_n2, assign16720_e16313_d_n3, assign16720_e16313_d_n4, assign16720_e16313_d_n7, assign16720_e16313_d_n10, assign16720_e16313_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__fds, locals.var_fn205_calc_iq__fds_dn2, locals.var_fn205_calc_iq__fds_dn3, locals.var_fn205_calc_iq__fds_dn4, locals.var_fn205_calc_iq__fds_dn7, locals.var_fn205_calc_iq__fds_dn10, locals.var_fn205_calc_iq__fds_dn11,)
    }
};
        locals.var_fn205_calc_iq__fds = assign16720_e16313;
        locals.var_fn205_calc_iq__fds_dn2 = assign16720_e16313_d_n2;
        locals.var_fn205_calc_iq__fds_dn3 = assign16720_e16313_d_n3;
        locals.var_fn205_calc_iq__fds_dn4 = assign16720_e16313_d_n4;
        locals.var_fn205_calc_iq__fds_dn7 = assign16720_e16313_d_n7;
        locals.var_fn205_calc_iq__fds_dn10 = assign16720_e16313_d_n10;
        locals.var_fn205_calc_iq__fds_dn11 = assign16720_e16313_d_n11;

        let (assign16730_e16317, assign16730_e16317_d_n2, assign16730_e16317_d_n3, assign16730_e16317_d_n4, assign16730_e16317_d_n7, assign16730_e16317_d_n10, assign16730_e16317_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__vsx, locals.var_fn205_calc_iq__vsx_dn2, locals.var_fn205_calc_iq__vsx_dn3, locals.var_fn205_calc_iq__vsx_dn4, locals.var_fn205_calc_iq__vsx_dn7, locals.var_fn205_calc_iq__vsx_dn10, locals.var_fn205_calc_iq__vsx_dn11,)
    }
};
        locals.var_fn205_calc_iq__vsx = assign16730_e16317;
        locals.var_fn205_calc_iq__vsx_dn2 = assign16730_e16317_d_n2;
        locals.var_fn205_calc_iq__vsx_dn3 = assign16730_e16317_d_n3;
        locals.var_fn205_calc_iq__vsx_dn4 = assign16730_e16317_d_n4;
        locals.var_fn205_calc_iq__vsx_dn7 = assign16730_e16317_d_n7;
        locals.var_fn205_calc_iq__vsx_dn10 = assign16730_e16317_d_n10;
        locals.var_fn205_calc_iq__vsx_dn11 = assign16730_e16317_d_n11;

        let (assign16740_e16321, assign16740_e16321_d_n2, assign16740_e16321_d_n3, assign16740_e16321_d_n4, assign16740_e16321_d_n7, assign16740_e16321_d_n10, assign16740_e16321_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__ffd, locals.var_fn205_calc_iq__ffd_dn2, locals.var_fn205_calc_iq__ffd_dn3, locals.var_fn205_calc_iq__ffd_dn4, locals.var_fn205_calc_iq__ffd_dn7, locals.var_fn205_calc_iq__ffd_dn10, locals.var_fn205_calc_iq__ffd_dn11,)
    }
};
        locals.var_fn205_calc_iq__ffd = assign16740_e16321;
        locals.var_fn205_calc_iq__ffd_dn2 = assign16740_e16321_d_n2;
        locals.var_fn205_calc_iq__ffd_dn3 = assign16740_e16321_d_n3;
        locals.var_fn205_calc_iq__ffd_dn4 = assign16740_e16321_d_n4;
        locals.var_fn205_calc_iq__ffd_dn7 = assign16740_e16321_d_n7;
        locals.var_fn205_calc_iq__ffd_dn10 = assign16740_e16321_d_n10;
        locals.var_fn205_calc_iq__ffd_dn11 = assign16740_e16321_d_n11;

        let (assign16750_e16325, assign16750_e16325_d_n2, assign16750_e16325_d_n3, assign16750_e16325_d_n4, assign16750_e16325_d_n7, assign16750_e16325_d_n10, assign16750_e16325_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__etad, locals.var_fn205_calc_iq__etad_dn2, locals.var_fn205_calc_iq__etad_dn3, locals.var_fn205_calc_iq__etad_dn4, locals.var_fn205_calc_iq__etad_dn7, locals.var_fn205_calc_iq__etad_dn10, locals.var_fn205_calc_iq__etad_dn11,)
    }
};
        locals.var_fn205_calc_iq__etad = assign16750_e16325;
        locals.var_fn205_calc_iq__etad_dn2 = assign16750_e16325_d_n2;
        locals.var_fn205_calc_iq__etad_dn3 = assign16750_e16325_d_n3;
        locals.var_fn205_calc_iq__etad_dn4 = assign16750_e16325_d_n4;
        locals.var_fn205_calc_iq__etad_dn7 = assign16750_e16325_d_n7;
        locals.var_fn205_calc_iq__etad_dn10 = assign16750_e16325_d_n10;
        locals.var_fn205_calc_iq__etad_dn11 = assign16750_e16325_d_n11;

        let (assign16760_e16329, assign16760_e16329_d_n2, assign16760_e16329_d_n3, assign16760_e16329_d_n4, assign16760_e16329_d_n7, assign16760_e16329_d_n10, assign16760_e16329_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__qinvd, locals.var_fn205_calc_iq__qinvd_dn2, locals.var_fn205_calc_iq__qinvd_dn3, locals.var_fn205_calc_iq__qinvd_dn4, locals.var_fn205_calc_iq__qinvd_dn7, locals.var_fn205_calc_iq__qinvd_dn10, locals.var_fn205_calc_iq__qinvd_dn11,)
    }
};
        locals.var_fn205_calc_iq__qinvd = assign16760_e16329;
        locals.var_fn205_calc_iq__qinvd_dn2 = assign16760_e16329_d_n2;
        locals.var_fn205_calc_iq__qinvd_dn3 = assign16760_e16329_d_n3;
        locals.var_fn205_calc_iq__qinvd_dn4 = assign16760_e16329_d_n4;
        locals.var_fn205_calc_iq__qinvd_dn7 = assign16760_e16329_d_n7;
        locals.var_fn205_calc_iq__qinvd_dn10 = assign16760_e16329_d_n10;
        locals.var_fn205_calc_iq__qinvd_dn11 = assign16760_e16329_d_n11;

        let (assign16770_e16333, assign16770_e16333_d_n2, assign16770_e16333_d_n3, assign16770_e16333_d_n4, assign16770_e16333_d_n7, assign16770_e16333_d_n10, assign16770_e16333_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__vdsc, locals.var_fn205_calc_iq__vdsc_dn2, locals.var_fn205_calc_iq__vdsc_dn3, locals.var_fn205_calc_iq__vdsc_dn4, locals.var_fn205_calc_iq__vdsc_dn7, locals.var_fn205_calc_iq__vdsc_dn10, locals.var_fn205_calc_iq__vdsc_dn11,)
    }
};
        locals.var_fn205_calc_iq__vdsc = assign16770_e16333;
        locals.var_fn205_calc_iq__vdsc_dn2 = assign16770_e16333_d_n2;
        locals.var_fn205_calc_iq__vdsc_dn3 = assign16770_e16333_d_n3;
        locals.var_fn205_calc_iq__vdsc_dn4 = assign16770_e16333_d_n4;
        locals.var_fn205_calc_iq__vdsc_dn7 = assign16770_e16333_d_n7;
        locals.var_fn205_calc_iq__vdsc_dn10 = assign16770_e16333_d_n10;
        locals.var_fn205_calc_iq__vdsc_dn11 = assign16770_e16333_d_n11;

        let (assign16800_e16345, assign16800_e16345_d_n4,) = {
    if (locals.var_guard204 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__vdsats0, locals.var_fn205_calc_iq__vdsats0_dn4,)
    }
};
        locals.var_fn205_calc_iq__vdsats0 = assign16800_e16345;
        locals.var_fn205_calc_iq__vdsats0_dn4 = assign16800_e16345_d_n4;

        let (assign16810_e16349, assign16810_e16349_d_n2, assign16810_e16349_d_n4, assign16810_e16349_d_n7, assign16810_e16349_d_n10, assign16810_e16349_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__vdsats10, locals.var_fn205_calc_iq__vdsats10_dn2, locals.var_fn205_calc_iq__vdsats10_dn4, locals.var_fn205_calc_iq__vdsats10_dn7, locals.var_fn205_calc_iq__vdsats10_dn10, locals.var_fn205_calc_iq__vdsats10_dn11,)
    }
};
        locals.var_fn205_calc_iq__vdsats10 = assign16810_e16349;
        locals.var_fn205_calc_iq__vdsats10_dn2 = assign16810_e16349_d_n2;
        locals.var_fn205_calc_iq__vdsats10_dn4 = assign16810_e16349_d_n4;
        locals.var_fn205_calc_iq__vdsats10_dn7 = assign16810_e16349_d_n7;
        locals.var_fn205_calc_iq__vdsats10_dn10 = assign16810_e16349_d_n10;
        locals.var_fn205_calc_iq__vdsats10_dn11 = assign16810_e16349_d_n11;

        let (assign16820_e16353, assign16820_e16353_d_n2, assign16820_e16353_d_n4, assign16820_e16353_d_n7, assign16820_e16353_d_n10, assign16820_e16353_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__vdsat10, locals.var_fn205_calc_iq__vdsat10_dn2, locals.var_fn205_calc_iq__vdsat10_dn4, locals.var_fn205_calc_iq__vdsat10_dn7, locals.var_fn205_calc_iq__vdsat10_dn10, locals.var_fn205_calc_iq__vdsat10_dn11,)
    }
};
        locals.var_fn205_calc_iq__vdsat10 = assign16820_e16353;
        locals.var_fn205_calc_iq__vdsat10_dn2 = assign16820_e16353_d_n2;
        locals.var_fn205_calc_iq__vdsat10_dn4 = assign16820_e16353_d_n4;
        locals.var_fn205_calc_iq__vdsat10_dn7 = assign16820_e16353_d_n7;
        locals.var_fn205_calc_iq__vdsat10_dn10 = assign16820_e16353_d_n10;
        locals.var_fn205_calc_iq__vdsat10_dn11 = assign16820_e16353_d_n11;

        let (assign16830_e16357, assign16830_e16357_d_n2, assign16830_e16357_d_n4, assign16830_e16357_d_n7, assign16830_e16357_d_n10, assign16830_e16357_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__fsd0, locals.var_fn205_calc_iq__fsd0_dn2, locals.var_fn205_calc_iq__fsd0_dn4, locals.var_fn205_calc_iq__fsd0_dn7, locals.var_fn205_calc_iq__fsd0_dn10, locals.var_fn205_calc_iq__fsd0_dn11,)
    }
};
        locals.var_fn205_calc_iq__fsd0 = assign16830_e16357;
        locals.var_fn205_calc_iq__fsd0_dn2 = assign16830_e16357_d_n2;
        locals.var_fn205_calc_iq__fsd0_dn4 = assign16830_e16357_d_n4;
        locals.var_fn205_calc_iq__fsd0_dn7 = assign16830_e16357_d_n7;
        locals.var_fn205_calc_iq__fsd0_dn10 = assign16830_e16357_d_n10;
        locals.var_fn205_calc_iq__fsd0_dn11 = assign16830_e16357_d_n11;

        let (assign16840_e16361, assign16840_e16361_d_n2, assign16840_e16361_d_n4, assign16840_e16361_d_n7, assign16840_e16361_d_n10, assign16840_e16361_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__vdx0, locals.var_fn205_calc_iq__vdx0_dn2, locals.var_fn205_calc_iq__vdx0_dn4, locals.var_fn205_calc_iq__vdx0_dn7, locals.var_fn205_calc_iq__vdx0_dn10, locals.var_fn205_calc_iq__vdx0_dn11,)
    }
};
        locals.var_fn205_calc_iq__vdx0 = assign16840_e16361;
        locals.var_fn205_calc_iq__vdx0_dn2 = assign16840_e16361_d_n2;
        locals.var_fn205_calc_iq__vdx0_dn4 = assign16840_e16361_d_n4;
        locals.var_fn205_calc_iq__vdx0_dn7 = assign16840_e16361_d_n7;
        locals.var_fn205_calc_iq__vdx0_dn10 = assign16840_e16361_d_n10;
        locals.var_fn205_calc_iq__vdx0_dn11 = assign16840_e16361_d_n11;

        let (assign16850_e16365, assign16850_e16365_d_n2, assign16850_e16365_d_n4, assign16850_e16365_d_n7, assign16850_e16365_d_n10, assign16850_e16365_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__fds0, locals.var_fn205_calc_iq__fds0_dn2, locals.var_fn205_calc_iq__fds0_dn4, locals.var_fn205_calc_iq__fds0_dn7, locals.var_fn205_calc_iq__fds0_dn10, locals.var_fn205_calc_iq__fds0_dn11,)
    }
};
        locals.var_fn205_calc_iq__fds0 = assign16850_e16365;
        locals.var_fn205_calc_iq__fds0_dn2 = assign16850_e16365_d_n2;
        locals.var_fn205_calc_iq__fds0_dn4 = assign16850_e16365_d_n4;
        locals.var_fn205_calc_iq__fds0_dn7 = assign16850_e16365_d_n7;
        locals.var_fn205_calc_iq__fds0_dn10 = assign16850_e16365_d_n10;
        locals.var_fn205_calc_iq__fds0_dn11 = assign16850_e16365_d_n11;

        let (assign16860_e16369, assign16860_e16369_d_n2, assign16860_e16369_d_n4, assign16860_e16369_d_n7, assign16860_e16369_d_n10, assign16860_e16369_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__vsx0, locals.var_fn205_calc_iq__vsx0_dn2, locals.var_fn205_calc_iq__vsx0_dn4, locals.var_fn205_calc_iq__vsx0_dn7, locals.var_fn205_calc_iq__vsx0_dn10, locals.var_fn205_calc_iq__vsx0_dn11,)
    }
};
        locals.var_fn205_calc_iq__vsx0 = assign16860_e16369;
        locals.var_fn205_calc_iq__vsx0_dn2 = assign16860_e16369_d_n2;
        locals.var_fn205_calc_iq__vsx0_dn4 = assign16860_e16369_d_n4;
        locals.var_fn205_calc_iq__vsx0_dn7 = assign16860_e16369_d_n7;
        locals.var_fn205_calc_iq__vsx0_dn10 = assign16860_e16369_d_n10;
        locals.var_fn205_calc_iq__vsx0_dn11 = assign16860_e16369_d_n11;

        let (assign16870_e16373, assign16870_e16373_d_n2, assign16870_e16373_d_n4, assign16870_e16373_d_n7, assign16870_e16373_d_n10, assign16870_e16373_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__ffd0, locals.var_fn205_calc_iq__ffd0_dn2, locals.var_fn205_calc_iq__ffd0_dn4, locals.var_fn205_calc_iq__ffd0_dn7, locals.var_fn205_calc_iq__ffd0_dn10, locals.var_fn205_calc_iq__ffd0_dn11,)
    }
};
        locals.var_fn205_calc_iq__ffd0 = assign16870_e16373;
        locals.var_fn205_calc_iq__ffd0_dn2 = assign16870_e16373_d_n2;
        locals.var_fn205_calc_iq__ffd0_dn4 = assign16870_e16373_d_n4;
        locals.var_fn205_calc_iq__ffd0_dn7 = assign16870_e16373_d_n7;
        locals.var_fn205_calc_iq__ffd0_dn10 = assign16870_e16373_d_n10;
        locals.var_fn205_calc_iq__ffd0_dn11 = assign16870_e16373_d_n11;

        let (assign16880_e16377, assign16880_e16377_d_n2, assign16880_e16377_d_n4, assign16880_e16377_d_n7, assign16880_e16377_d_n10, assign16880_e16377_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__etad0, locals.var_fn205_calc_iq__etad0_dn2, locals.var_fn205_calc_iq__etad0_dn4, locals.var_fn205_calc_iq__etad0_dn7, locals.var_fn205_calc_iq__etad0_dn10, locals.var_fn205_calc_iq__etad0_dn11,)
    }
};
        locals.var_fn205_calc_iq__etad0 = assign16880_e16377;
        locals.var_fn205_calc_iq__etad0_dn2 = assign16880_e16377_d_n2;
        locals.var_fn205_calc_iq__etad0_dn4 = assign16880_e16377_d_n4;
        locals.var_fn205_calc_iq__etad0_dn7 = assign16880_e16377_d_n7;
        locals.var_fn205_calc_iq__etad0_dn10 = assign16880_e16377_d_n10;
        locals.var_fn205_calc_iq__etad0_dn11 = assign16880_e16377_d_n11;

        let (assign16890_e16381, assign16890_e16381_d_n2, assign16890_e16381_d_n4, assign16890_e16381_d_n7, assign16890_e16381_d_n10, assign16890_e16381_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__qinvd0, locals.var_fn205_calc_iq__qinvd0_dn2, locals.var_fn205_calc_iq__qinvd0_dn4, locals.var_fn205_calc_iq__qinvd0_dn7, locals.var_fn205_calc_iq__qinvd0_dn10, locals.var_fn205_calc_iq__qinvd0_dn11,)
    }
};
        locals.var_fn205_calc_iq__qinvd0 = assign16890_e16381;
        locals.var_fn205_calc_iq__qinvd0_dn2 = assign16890_e16381_d_n2;
        locals.var_fn205_calc_iq__qinvd0_dn4 = assign16890_e16381_d_n4;
        locals.var_fn205_calc_iq__qinvd0_dn7 = assign16890_e16381_d_n7;
        locals.var_fn205_calc_iq__qinvd0_dn10 = assign16890_e16381_d_n10;
        locals.var_fn205_calc_iq__qinvd0_dn11 = assign16890_e16381_d_n11;

        let (assign16900_e16385, assign16900_e16385_d_n2, assign16900_e16385_d_n4, assign16900_e16385_d_n7, assign16900_e16385_d_n10, assign16900_e16385_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__qs2, locals.var_fn205_calc_iq__qs2_dn2, locals.var_fn205_calc_iq__qs2_dn4, locals.var_fn205_calc_iq__qs2_dn7, locals.var_fn205_calc_iq__qs2_dn10, locals.var_fn205_calc_iq__qs2_dn11,)
    }
};
        locals.var_fn205_calc_iq__qs2 = assign16900_e16385;
        locals.var_fn205_calc_iq__qs2_dn2 = assign16900_e16385_d_n2;
        locals.var_fn205_calc_iq__qs2_dn4 = assign16900_e16385_d_n4;
        locals.var_fn205_calc_iq__qs2_dn7 = assign16900_e16385_d_n7;
        locals.var_fn205_calc_iq__qs2_dn10 = assign16900_e16385_d_n10;
        locals.var_fn205_calc_iq__qs2_dn11 = assign16900_e16385_d_n11;

        let (assign16910_e16389, assign16910_e16389_d_n2, assign16910_e16389_d_n4, assign16910_e16389_d_n7, assign16910_e16389_d_n10, assign16910_e16389_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__qs3, locals.var_fn205_calc_iq__qs3_dn2, locals.var_fn205_calc_iq__qs3_dn4, locals.var_fn205_calc_iq__qs3_dn7, locals.var_fn205_calc_iq__qs3_dn10, locals.var_fn205_calc_iq__qs3_dn11,)
    }
};
        locals.var_fn205_calc_iq__qs3 = assign16910_e16389;
        locals.var_fn205_calc_iq__qs3_dn2 = assign16910_e16389_d_n2;
        locals.var_fn205_calc_iq__qs3_dn4 = assign16910_e16389_d_n4;
        locals.var_fn205_calc_iq__qs3_dn7 = assign16910_e16389_d_n7;
        locals.var_fn205_calc_iq__qs3_dn10 = assign16910_e16389_d_n10;
        locals.var_fn205_calc_iq__qs3_dn11 = assign16910_e16389_d_n11;

        let (assign16920_e16393, assign16920_e16393_d_n2, assign16920_e16393_d_n4, assign16920_e16393_d_n7, assign16920_e16393_d_n10, assign16920_e16393_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__qd2, locals.var_fn205_calc_iq__qd2_dn2, locals.var_fn205_calc_iq__qd2_dn4, locals.var_fn205_calc_iq__qd2_dn7, locals.var_fn205_calc_iq__qd2_dn10, locals.var_fn205_calc_iq__qd2_dn11,)
    }
};
        locals.var_fn205_calc_iq__qd2 = assign16920_e16393;
        locals.var_fn205_calc_iq__qd2_dn2 = assign16920_e16393_d_n2;
        locals.var_fn205_calc_iq__qd2_dn4 = assign16920_e16393_d_n4;
        locals.var_fn205_calc_iq__qd2_dn7 = assign16920_e16393_d_n7;
        locals.var_fn205_calc_iq__qd2_dn10 = assign16920_e16393_d_n10;
        locals.var_fn205_calc_iq__qd2_dn11 = assign16920_e16393_d_n11;

        let (assign16930_e16397, assign16930_e16397_d_n2, assign16930_e16397_d_n4, assign16930_e16397_d_n7, assign16930_e16397_d_n10, assign16930_e16397_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__qd3, locals.var_fn205_calc_iq__qd3_dn2, locals.var_fn205_calc_iq__qd3_dn4, locals.var_fn205_calc_iq__qd3_dn7, locals.var_fn205_calc_iq__qd3_dn10, locals.var_fn205_calc_iq__qd3_dn11,)
    }
};
        locals.var_fn205_calc_iq__qd3 = assign16930_e16397;
        locals.var_fn205_calc_iq__qd3_dn2 = assign16930_e16397_d_n2;
        locals.var_fn205_calc_iq__qd3_dn4 = assign16930_e16397_d_n4;
        locals.var_fn205_calc_iq__qd3_dn7 = assign16930_e16397_d_n7;
        locals.var_fn205_calc_iq__qd3_dn10 = assign16930_e16397_d_n10;
        locals.var_fn205_calc_iq__qd3_dn11 = assign16930_e16397_d_n11;

        let (assign16940_e16401, assign16940_e16401_d_n2, assign16940_e16401_d_n4, assign16940_e16401_d_n7, assign16940_e16401_d_n10, assign16940_e16401_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__qsqd, locals.var_fn205_calc_iq__qsqd_dn2, locals.var_fn205_calc_iq__qsqd_dn4, locals.var_fn205_calc_iq__qsqd_dn7, locals.var_fn205_calc_iq__qsqd_dn10, locals.var_fn205_calc_iq__qsqd_dn11,)
    }
};
        locals.var_fn205_calc_iq__qsqd = assign16940_e16401;
        locals.var_fn205_calc_iq__qsqd_dn2 = assign16940_e16401_d_n2;
        locals.var_fn205_calc_iq__qsqd_dn4 = assign16940_e16401_d_n4;
        locals.var_fn205_calc_iq__qsqd_dn7 = assign16940_e16401_d_n7;
        locals.var_fn205_calc_iq__qsqd_dn10 = assign16940_e16401_d_n10;
        locals.var_fn205_calc_iq__qsqd_dn11 = assign16940_e16401_d_n11;

    }

    pub(super) fn stamp_transient_block_42(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign16950_e16405, assign16950_e16405_d_n2, assign16950_e16405_d_n4, assign16950_e16405_d_n7, assign16950_e16405_d_n10, assign16950_e16405_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__qinvdd, locals.var_fn205_calc_iq__qinvdd_dn2, locals.var_fn205_calc_iq__qinvdd_dn4, locals.var_fn205_calc_iq__qinvdd_dn7, locals.var_fn205_calc_iq__qinvdd_dn10, locals.var_fn205_calc_iq__qinvdd_dn11,)
    }
};
        locals.var_fn205_calc_iq__qinvdd = assign16950_e16405;
        locals.var_fn205_calc_iq__qinvdd_dn2 = assign16950_e16405_d_n2;
        locals.var_fn205_calc_iq__qinvdd_dn4 = assign16950_e16405_d_n4;
        locals.var_fn205_calc_iq__qinvdd_dn7 = assign16950_e16405_d_n7;
        locals.var_fn205_calc_iq__qinvdd_dn10 = assign16950_e16405_d_n10;
        locals.var_fn205_calc_iq__qinvdd_dn11 = assign16950_e16405_d_n11;

        let (assign16960_e16409, assign16960_e16409_d_n2, assign16960_e16409_d_n4, assign16960_e16409_d_n7, assign16960_e16409_d_n10, assign16960_e16409_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__qd1, locals.var_fn205_calc_iq__qd1_dn2, locals.var_fn205_calc_iq__qd1_dn4, locals.var_fn205_calc_iq__qd1_dn7, locals.var_fn205_calc_iq__qd1_dn10, locals.var_fn205_calc_iq__qd1_dn11,)
    }
};
        locals.var_fn205_calc_iq__qd1 = assign16960_e16409;
        locals.var_fn205_calc_iq__qd1_dn2 = assign16960_e16409_d_n2;
        locals.var_fn205_calc_iq__qd1_dn4 = assign16960_e16409_d_n4;
        locals.var_fn205_calc_iq__qd1_dn7 = assign16960_e16409_d_n7;
        locals.var_fn205_calc_iq__qd1_dn10 = assign16960_e16409_d_n10;
        locals.var_fn205_calc_iq__qd1_dn11 = assign16960_e16409_d_n11;

        let (assign16970_e16413, assign16970_e16413_d_n2, assign16970_e16413_d_n4, assign16970_e16413_d_n7, assign16970_e16413_d_n10, assign16970_e16413_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__qs, locals.var_fn205_calc_iq__qs_dn2, locals.var_fn205_calc_iq__qs_dn4, locals.var_fn205_calc_iq__qs_dn7, locals.var_fn205_calc_iq__qs_dn10, locals.var_fn205_calc_iq__qs_dn11,)
    }
};
        locals.var_fn205_calc_iq__qs = assign16970_e16413;
        locals.var_fn205_calc_iq__qs_dn2 = assign16970_e16413_d_n2;
        locals.var_fn205_calc_iq__qs_dn4 = assign16970_e16413_d_n4;
        locals.var_fn205_calc_iq__qs_dn7 = assign16970_e16413_d_n7;
        locals.var_fn205_calc_iq__qs_dn10 = assign16970_e16413_d_n10;
        locals.var_fn205_calc_iq__qs_dn11 = assign16970_e16413_d_n11;

        let (assign16980_e16417, assign16980_e16417_d_n2, assign16980_e16417_d_n4, assign16980_e16417_d_n7, assign16980_e16417_d_n10, assign16980_e16417_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__qd, locals.var_fn205_calc_iq__qd_dn2, locals.var_fn205_calc_iq__qd_dn4, locals.var_fn205_calc_iq__qd_dn7, locals.var_fn205_calc_iq__qd_dn10, locals.var_fn205_calc_iq__qd_dn11,)
    }
};
        locals.var_fn205_calc_iq__qd = assign16980_e16417;
        locals.var_fn205_calc_iq__qd_dn2 = assign16980_e16417_d_n2;
        locals.var_fn205_calc_iq__qd_dn4 = assign16980_e16417_d_n4;
        locals.var_fn205_calc_iq__qd_dn7 = assign16980_e16417_d_n7;
        locals.var_fn205_calc_iq__qd_dn10 = assign16980_e16417_d_n10;
        locals.var_fn205_calc_iq__qd_dn11 = assign16980_e16417_d_n11;

        let (assign16990_e16421, assign16990_e16421_d_n2, assign16990_e16421_d_n4, assign16990_e16421_d_n7, assign16990_e16421_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__etac, locals.var_fn205_calc_iq__etac_dn2, locals.var_fn205_calc_iq__etac_dn4, locals.var_fn205_calc_iq__etac_dn7, locals.var_fn205_calc_iq__etac_dn11,)
    }
};
        locals.var_fn205_calc_iq__etac = assign16990_e16421;
        locals.var_fn205_calc_iq__etac_dn2 = assign16990_e16421_d_n2;
        locals.var_fn205_calc_iq__etac_dn4 = assign16990_e16421_d_n4;
        locals.var_fn205_calc_iq__etac_dn7 = assign16990_e16421_d_n7;
        locals.var_fn205_calc_iq__etac_dn11 = assign16990_e16421_d_n11;

        let (assign17000_e16425, assign17000_e16425_d_n3, assign17000_e16425_d_n4, assign17000_e16425_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__etab, locals.var_fn205_calc_iq__etab_dn3, locals.var_fn205_calc_iq__etab_dn4, locals.var_fn205_calc_iq__etab_dn11,)
    }
};
        locals.var_fn205_calc_iq__etab = assign17000_e16425;
        locals.var_fn205_calc_iq__etab_dn3 = assign17000_e16425_d_n3;
        locals.var_fn205_calc_iq__etab_dn4 = assign17000_e16425_d_n4;
        locals.var_fn205_calc_iq__etab_dn11 = assign17000_e16425_d_n11;

        let (assign17010_e16429, assign17010_e16429_d_n2, assign17010_e16429_d_n4, assign17010_e16429_d_n7, assign17010_e16429_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__etags, locals.var_fn205_calc_iq__etags_dn2, locals.var_fn205_calc_iq__etags_dn4, locals.var_fn205_calc_iq__etags_dn7, locals.var_fn205_calc_iq__etags_dn11,)
    }
};
        locals.var_fn205_calc_iq__etags = assign17010_e16429;
        locals.var_fn205_calc_iq__etags_dn2 = assign17010_e16429_d_n2;
        locals.var_fn205_calc_iq__etags_dn4 = assign17010_e16429_d_n4;
        locals.var_fn205_calc_iq__etags_dn7 = assign17010_e16429_d_n7;
        locals.var_fn205_calc_iq__etags_dn11 = assign17010_e16429_d_n11;

        let (assign17020_e16433, assign17020_e16433_d_n2, assign17020_e16433_d_n3, assign17020_e16433_d_n4, assign17020_e16433_d_n7, assign17020_e16433_d_n10, assign17020_e16433_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__exparg, locals.var_fn205_calc_iq__exparg_dn2, locals.var_fn205_calc_iq__exparg_dn3, locals.var_fn205_calc_iq__exparg_dn4, locals.var_fn205_calc_iq__exparg_dn7, locals.var_fn205_calc_iq__exparg_dn10, locals.var_fn205_calc_iq__exparg_dn11,)
    }
};
        locals.var_fn205_calc_iq__exparg = assign17020_e16433;
        locals.var_fn205_calc_iq__exparg_dn2 = assign17020_e16433_d_n2;
        locals.var_fn205_calc_iq__exparg_dn3 = assign17020_e16433_d_n3;
        locals.var_fn205_calc_iq__exparg_dn4 = assign17020_e16433_d_n4;
        locals.var_fn205_calc_iq__exparg_dn7 = assign17020_e16433_d_n7;
        locals.var_fn205_calc_iq__exparg_dn10 = assign17020_e16433_d_n10;
        locals.var_fn205_calc_iq__exparg_dn11 = assign17020_e16433_d_n11;

        let (assign17030_e16437, assign17030_e16437_d_n2, assign17030_e16437_d_n3, assign17030_e16437_d_n4, assign17030_e16437_d_n7, assign17030_e16437_d_n10, assign17030_e16437_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__myarg, locals.var_fn205_calc_iq__myarg_dn2, locals.var_fn205_calc_iq__myarg_dn3, locals.var_fn205_calc_iq__myarg_dn4, locals.var_fn205_calc_iq__myarg_dn7, locals.var_fn205_calc_iq__myarg_dn10, locals.var_fn205_calc_iq__myarg_dn11,)
    }
};
        locals.var_fn205_calc_iq__myarg = assign17030_e16437;
        locals.var_fn205_calc_iq__myarg_dn2 = assign17030_e16437_d_n2;
        locals.var_fn205_calc_iq__myarg_dn3 = assign17030_e16437_d_n3;
        locals.var_fn205_calc_iq__myarg_dn4 = assign17030_e16437_d_n4;
        locals.var_fn205_calc_iq__myarg_dn7 = assign17030_e16437_d_n7;
        locals.var_fn205_calc_iq__myarg_dn10 = assign17030_e16437_d_n10;
        locals.var_fn205_calc_iq__myarg_dn11 = assign17030_e16437_d_n11;

        let (assign17040_e16441, assign17040_e16441_d_n10, assign17040_e16441_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        (0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__absvdsin, locals.var_fn205_calc_iq__absvdsin_dn10, locals.var_fn205_calc_iq__absvdsin_dn11,)
    }
};
        locals.var_fn205_calc_iq__absvdsin = assign17040_e16441;
        locals.var_fn205_calc_iq__absvdsin_dn10 = assign17040_e16441_d_n10;
        locals.var_fn205_calc_iq__absvdsin_dn11 = assign17040_e16441_d_n11;

        let (assign17050_e16445, assign17050_e16445_d_n2, assign17050_e16445_d_n7, assign17050_e16445_d_n10, assign17050_e16445_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__vgdin, locals.var_fn205_calc_iq__vgdin_dn2, locals.var_fn205_calc_iq__vgdin_dn7, locals.var_fn205_calc_iq__vgdin_dn10, locals.var_fn205_calc_iq__vgdin_dn11,)
    }
};
        locals.var_fn205_calc_iq__vgdin = assign17050_e16445;
        locals.var_fn205_calc_iq__vgdin_dn2 = assign17050_e16445_d_n2;
        locals.var_fn205_calc_iq__vgdin_dn7 = assign17050_e16445_d_n7;
        locals.var_fn205_calc_iq__vgdin_dn10 = assign17050_e16445_d_n10;
        locals.var_fn205_calc_iq__vgdin_dn11 = assign17050_e16445_d_n11;

        let (assign17060_e16449, assign17060_e16449_d_n2, assign17060_e16449_d_n4, assign17060_e16449_d_n7, assign17060_e16449_d_n10, assign17060_e16449_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__exparg0, locals.var_fn205_calc_iq__exparg0_dn2, locals.var_fn205_calc_iq__exparg0_dn4, locals.var_fn205_calc_iq__exparg0_dn7, locals.var_fn205_calc_iq__exparg0_dn10, locals.var_fn205_calc_iq__exparg0_dn11,)
    }
};
        locals.var_fn205_calc_iq__exparg0 = assign17060_e16449;
        locals.var_fn205_calc_iq__exparg0_dn2 = assign17060_e16449_d_n2;
        locals.var_fn205_calc_iq__exparg0_dn4 = assign17060_e16449_d_n4;
        locals.var_fn205_calc_iq__exparg0_dn7 = assign17060_e16449_d_n7;
        locals.var_fn205_calc_iq__exparg0_dn10 = assign17060_e16449_d_n10;
        locals.var_fn205_calc_iq__exparg0_dn11 = assign17060_e16449_d_n11;

        let (assign17070_e16453, assign17070_e16453_d_n4,) = {
    if (locals.var_guard204 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__myarg0, locals.var_fn205_calc_iq__myarg0_dn4,)
    }
};
        locals.var_fn205_calc_iq__myarg0 = assign17070_e16453;
        locals.var_fn205_calc_iq__myarg0_dn4 = assign17070_e16453_d_n4;

        let (assign17080_e16480, assign17080_e16480_d_n10, assign17080_e16480_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        let (assign17080_e16478, assign17080_e16478_d_n10, assign17080_e16478_d_n11,) = {
            if (p.p52 != 0.0) {
                let assign17080_e16462: f64 = (0.001 / p.p53);
                let assign17080_e16464: f64 = (assign17080_e16462 * locals.var_fn205_calc_iq__vdsin);
                let assign17080_e16465: f64 = (assign17080_e16464).tanh();
                let assign17080_e16466: f64 = (locals.var_fn205_calc_iq__vdsin * assign17080_e16465);
                (assign17080_e16466, ((locals.var_fn205_calc_iq__vdsin_dn10 * assign17080_e16465) + (locals.var_fn205_calc_iq__vdsin * ((assign17080_e16462 * locals.var_fn205_calc_iq__vdsin_dn10) / ((assign17080_e16464).cosh() * (assign17080_e16464).cosh())))), ((locals.var_fn205_calc_iq__vdsin_dn11 * assign17080_e16465) + (locals.var_fn205_calc_iq__vdsin * ((assign17080_e16462 * locals.var_fn205_calc_iq__vdsin_dn11) / ((assign17080_e16464).cosh() * (assign17080_e16464).cosh())))),)
            } else {
                let (assign17080_e16477, assign17080_e16477_d_n10, assign17080_e16477_d_n11,) = {
                    if (p.p52 == 0.0) {
                        let assign17080_e16472: f64 = (locals.var_fn205_calc_iq__vdsin * locals.var_fn205_calc_iq__vdsin);
                        let assign17080_e16474: f64 = (assign17080_e16472 + p.p53);
                        let assign17080_e16475: f64 = (assign17080_e16474).sqrt();
                        (assign17080_e16475, (((locals.var_fn205_calc_iq__vdsin_dn10 * locals.var_fn205_calc_iq__vdsin) + (locals.var_fn205_calc_iq__vdsin * locals.var_fn205_calc_iq__vdsin_dn10)) / (2.0 * assign17080_e16475)), (((locals.var_fn205_calc_iq__vdsin_dn11 * locals.var_fn205_calc_iq__vdsin) + (locals.var_fn205_calc_iq__vdsin * locals.var_fn205_calc_iq__vdsin_dn11)) / (2.0 * assign17080_e16475)),)
                    } else {
                        (0.0, 0.0, 0.0,)
                    }
                };
                (assign17080_e16477, assign17080_e16477_d_n10, assign17080_e16477_d_n11,)
            }
        };
        (assign17080_e16478, assign17080_e16478_d_n10, assign17080_e16478_d_n11,)
    } else {
        (locals.var_fn205_calc_iq__absvdsin, locals.var_fn205_calc_iq__absvdsin_dn10, locals.var_fn205_calc_iq__absvdsin_dn11,)
    }
};
        locals.var_fn205_calc_iq__absvdsin = assign17080_e16480;
        locals.var_fn205_calc_iq__absvdsin_dn10 = assign17080_e16480_d_n10;
        locals.var_fn205_calc_iq__absvdsin_dn11 = assign17080_e16480_d_n11;

        let (assign17090_e16486, assign17090_e16486_d_n2, assign17090_e16486_d_n7, assign17090_e16486_d_n10, assign17090_e16486_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        let assign17090_e16484: f64 = (locals.var_fn205_calc_iq__vgsin - locals.var_fn205_calc_iq__vdsin);
        (assign17090_e16484, locals.var_fn205_calc_iq__vgsin_dn2, locals.var_fn205_calc_iq__vgsin_dn7, (-locals.var_fn205_calc_iq__vdsin_dn10), (locals.var_fn205_calc_iq__vgsin_dn11 - locals.var_fn205_calc_iq__vdsin_dn11),)
    } else {
        (locals.var_fn205_calc_iq__vgdin, locals.var_fn205_calc_iq__vgdin_dn2, locals.var_fn205_calc_iq__vgdin_dn7, locals.var_fn205_calc_iq__vgdin_dn10, locals.var_fn205_calc_iq__vgdin_dn11,)
    }
};
        locals.var_fn205_calc_iq__vgdin = assign17090_e16486;
        locals.var_fn205_calc_iq__vgdin_dn2 = assign17090_e16486_d_n2;
        locals.var_fn205_calc_iq__vgdin_dn7 = assign17090_e16486_d_n7;
        locals.var_fn205_calc_iq__vgdin_dn10 = assign17090_e16486_d_n10;
        locals.var_fn205_calc_iq__vgdin_dn11 = assign17090_e16486_d_n11;

        let (assign17100_e16492, assign17100_e16492_d_n4,) = {
    if (locals.var_guard204 != 0.0) {
        let assign17100_e16490: f64 = (locals.var_fn205_calc_iq__alpha * locals.var_fn205_calc_iq__phitin);
        (assign17100_e16490, (locals.var_fn205_calc_iq__alpha * locals.var_fn205_calc_iq__phitin_dn4),)
    } else {
        (locals.var_fn205_calc_iq__alpha_phit, locals.var_fn205_calc_iq__alpha_phit_dn4,)
    }
};
        locals.var_fn205_calc_iq__alpha_phit = assign17100_e16492;
        locals.var_fn205_calc_iq__alpha_phit_dn4 = assign17100_e16492_d_n4;

        let (assign17110_e16504, assign17110_e16504_d_n4, assign17110_e16504_d_n10, assign17110_e16504_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        let assign17110_e16497: f64 = (2.302585092994046 * locals.var_fn205_calc_iq__phitin);
        let assign17110_e16498: f64 = (locals.var_fn205_calc_iq__ss / assign17110_e16497);
        let assign17110_e16501: f64 = (locals.var_fn205_calc_iq__nd * locals.var_fn205_calc_iq__absvdsin);
        let assign17110_e16502: f64 = (assign17110_e16498 + assign17110_e16501);
        (assign17110_e16502, (-((locals.var_fn205_calc_iq__ss * (2.302585092994046 * locals.var_fn205_calc_iq__phitin_dn4)) / (assign17110_e16497 * assign17110_e16497))), (locals.var_fn205_calc_iq__nd * locals.var_fn205_calc_iq__absvdsin_dn10), (locals.var_fn205_calc_iq__nd * locals.var_fn205_calc_iq__absvdsin_dn11),)
    } else {
        (locals.var_fn205_calc_iq__n, locals.var_fn205_calc_iq__n_dn4, locals.var_fn205_calc_iq__n_dn10, locals.var_fn205_calc_iq__n_dn11,)
    }
};
        locals.var_fn205_calc_iq__n = assign17110_e16504;
        locals.var_fn205_calc_iq__n_dn4 = assign17110_e16504_d_n4;
        locals.var_fn205_calc_iq__n_dn10 = assign17110_e16504_d_n10;
        locals.var_fn205_calc_iq__n_dn11 = assign17110_e16504_d_n11;

        let (assign17120_e16514, assign17120_e16514_d_n4,) = {
    if (locals.var_guard204 != 0.0) {
        let assign17120_e16510: f64 = (locals.var_fn205_calc_iq__tambin - locals.var_fn205_calc_iq__tnomin);
        let assign17120_e16511: f64 = (locals.var_fn205_calc_iq__vtzeta * assign17120_e16510);
        let assign17120_e16512: f64 = (locals.var_fn205_calc_iq__vto + assign17120_e16511);
        (assign17120_e16512, (locals.var_fn205_calc_iq__vtzeta * locals.var_fn205_calc_iq__tambin_dn4),)
    } else {
        (locals.var_fn205_calc_iq__vtof, locals.var_fn205_calc_iq__vtof_dn4,)
    }
};
        locals.var_fn205_calc_iq__vtof = assign17120_e16514;
        locals.var_fn205_calc_iq__vtof_dn4 = assign17120_e16514_d_n4;

        let (assign17130_e16522, assign17130_e16522_d_n4,) = {
    if (locals.var_guard204 != 0.0) {
        let assign17130_e16518: f64 = (locals.var_fn205_calc_iq__tambin / locals.var_fn205_calc_iq__tnomin);
        let assign17130_e16520: f64 = (assign17130_e16518).powf(locals.var_fn205_calc_iq__epsilon);
        (assign17130_e16520, if 0.0 == 0.0 && ((locals.var_fn205_calc_iq__epsilon) as f64).is_finite() && ((locals.var_fn205_calc_iq__epsilon) as f64).fract() == 0.0 { if locals.var_fn205_calc_iq__epsilon == 0.0 { 0.0 } else { (locals.var_fn205_calc_iq__epsilon * ((assign17130_e16518).powf(locals.var_fn205_calc_iq__epsilon - 1.0) * (locals.var_fn205_calc_iq__tambin_dn4 / locals.var_fn205_calc_iq__tnomin))) } } else { (assign17130_e16520 * (locals.var_fn205_calc_iq__epsilon * ((locals.var_fn205_calc_iq__tambin_dn4 / locals.var_fn205_calc_iq__tnomin) / assign17130_e16518))) },)
    } else {
        (locals.var_fn205_calc_iq__tfacmobin, locals.var_fn205_calc_iq__tfacmobin_dn4,)
    }
};
        locals.var_fn205_calc_iq__tfacmobin = assign17130_e16522;
        locals.var_fn205_calc_iq__tfacmobin_dn4 = assign17130_e16522_d_n4;

        let assign17140_e16525: f64 = if locals.var_fn205_calc_iq__dibsat != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard206 = assign17140_e16525;

        let (assign17150_e16543, assign17150_e16543_d_n10, assign17150_e16543_d_n11,) = {
    if ((locals.var_guard204 != 0.0) && (locals.var_guard206 != 0.0)) {
        let assign17150_e16533: f64 = (locals.var_fn205_calc_iq__absvdsin / locals.var_fn205_calc_iq__dibsat);
        let assign17150_e16535: f64 = (assign17150_e16533).powf(locals.var_fn205_calc_iq__beta);
        let assign17150_e16536: f64 = (1.0 + assign17150_e16535);
        let assign17150_e16539: f64 = (1.0 / locals.var_fn205_calc_iq__beta);
        let assign17150_e16540: f64 = (assign17150_e16536).powf(assign17150_e16539);
        let assign17150_e16541: f64 = (locals.var_fn205_calc_iq__absvdsin / assign17150_e16540);
        (assign17150_e16541, (((locals.var_fn205_calc_iq__absvdsin_dn10 * assign17150_e16540) - (locals.var_fn205_calc_iq__absvdsin * if 0.0 == 0.0 && ((assign17150_e16539) as f64).is_finite() && ((assign17150_e16539) as f64).fract() == 0.0 { if assign17150_e16539 == 0.0 { 0.0 } else { (assign17150_e16539 * ((assign17150_e16536).powf(assign17150_e16539 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn205_calc_iq__beta) as f64).is_finite() && ((locals.var_fn205_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn205_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn205_calc_iq__beta * ((assign17150_e16533).powf(locals.var_fn205_calc_iq__beta - 1.0) * (locals.var_fn205_calc_iq__absvdsin_dn10 / locals.var_fn205_calc_iq__dibsat))) } } else { (assign17150_e16535 * (locals.var_fn205_calc_iq__beta * ((locals.var_fn205_calc_iq__absvdsin_dn10 / locals.var_fn205_calc_iq__dibsat) / assign17150_e16533))) })) } } else { (assign17150_e16540 * (assign17150_e16539 * (if 0.0 == 0.0 && ((locals.var_fn205_calc_iq__beta) as f64).is_finite() && ((locals.var_fn205_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn205_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn205_calc_iq__beta * ((assign17150_e16533).powf(locals.var_fn205_calc_iq__beta - 1.0) * (locals.var_fn205_calc_iq__absvdsin_dn10 / locals.var_fn205_calc_iq__dibsat))) } } else { (assign17150_e16535 * (locals.var_fn205_calc_iq__beta * ((locals.var_fn205_calc_iq__absvdsin_dn10 / locals.var_fn205_calc_iq__dibsat) / assign17150_e16533))) } / assign17150_e16536))) })) / (assign17150_e16540 * assign17150_e16540)), (((locals.var_fn205_calc_iq__absvdsin_dn11 * assign17150_e16540) - (locals.var_fn205_calc_iq__absvdsin * if 0.0 == 0.0 && ((assign17150_e16539) as f64).is_finite() && ((assign17150_e16539) as f64).fract() == 0.0 { if assign17150_e16539 == 0.0 { 0.0 } else { (assign17150_e16539 * ((assign17150_e16536).powf(assign17150_e16539 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn205_calc_iq__beta) as f64).is_finite() && ((locals.var_fn205_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn205_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn205_calc_iq__beta * ((assign17150_e16533).powf(locals.var_fn205_calc_iq__beta - 1.0) * (locals.var_fn205_calc_iq__absvdsin_dn11 / locals.var_fn205_calc_iq__dibsat))) } } else { (assign17150_e16535 * (locals.var_fn205_calc_iq__beta * ((locals.var_fn205_calc_iq__absvdsin_dn11 / locals.var_fn205_calc_iq__dibsat) / assign17150_e16533))) })) } } else { (assign17150_e16540 * (assign17150_e16539 * (if 0.0 == 0.0 && ((locals.var_fn205_calc_iq__beta) as f64).is_finite() && ((locals.var_fn205_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn205_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn205_calc_iq__beta * ((assign17150_e16533).powf(locals.var_fn205_calc_iq__beta - 1.0) * (locals.var_fn205_calc_iq__absvdsin_dn11 / locals.var_fn205_calc_iq__dibsat))) } } else { (assign17150_e16535 * (locals.var_fn205_calc_iq__beta * ((locals.var_fn205_calc_iq__absvdsin_dn11 / locals.var_fn205_calc_iq__dibsat) / assign17150_e16533))) } / assign17150_e16536))) })) / (assign17150_e16540 * assign17150_e16540)),)
    } else {
        (locals.var_fn205_calc_iq__vsatdibl, locals.var_fn205_calc_iq__vsatdibl_dn10, locals.var_fn205_calc_iq__vsatdibl_dn11,)
    }
};
        locals.var_fn205_calc_iq__vsatdibl = assign17150_e16543;
        locals.var_fn205_calc_iq__vsatdibl_dn10 = assign17150_e16543_d_n10;
        locals.var_fn205_calc_iq__vsatdibl_dn11 = assign17150_e16543_d_n11;

        let (assign17160_e16550, assign17160_e16550_d_n10, assign17160_e16550_d_n11,) = {
    if ((locals.var_guard204 != 0.0) && (locals.var_guard206 == 0.0)) {
        (0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__vsatdibl, locals.var_fn205_calc_iq__vsatdibl_dn10, locals.var_fn205_calc_iq__vsatdibl_dn11,)
    }
};
        locals.var_fn205_calc_iq__vsatdibl = assign17160_e16550;
        locals.var_fn205_calc_iq__vsatdibl_dn10 = assign17160_e16550_d_n10;
        locals.var_fn205_calc_iq__vsatdibl_dn11 = assign17160_e16550_d_n11;

        let (assign17170_e16560, assign17170_e16560_d_n10, assign17170_e16560_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        let assign17170_e16555: f64 = (locals.var_fn205_calc_iq__vsatdibl * locals.var_fn205_calc_iq__delta2);
        let assign17170_e16556: f64 = (locals.var_fn205_calc_iq__delta1 - assign17170_e16555);
        let assign17170_e16558: f64 = (assign17170_e16556 * locals.var_fn205_calc_iq__absvdsin);
        (assign17170_e16558, (((-(locals.var_fn205_calc_iq__vsatdibl_dn10 * locals.var_fn205_calc_iq__delta2)) * locals.var_fn205_calc_iq__absvdsin) + (assign17170_e16556 * locals.var_fn205_calc_iq__absvdsin_dn10)), (((-(locals.var_fn205_calc_iq__vsatdibl_dn11 * locals.var_fn205_calc_iq__delta2)) * locals.var_fn205_calc_iq__absvdsin) + (assign17170_e16556 * locals.var_fn205_calc_iq__absvdsin_dn11)),)
    } else {
        (locals.var_fn205_calc_iq__delta, locals.var_fn205_calc_iq__delta_dn10, locals.var_fn205_calc_iq__delta_dn11,)
    }
};
        locals.var_fn205_calc_iq__delta = assign17170_e16560;
        locals.var_fn205_calc_iq__delta_dn10 = assign17170_e16560_d_n10;
        locals.var_fn205_calc_iq__delta_dn11 = assign17170_e16560_d_n11;

        let (assign17180_e16566, assign17180_e16566_d_n4, assign17180_e16566_d_n10, assign17180_e16566_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        let assign17180_e16564: f64 = (locals.var_fn205_calc_iq__vtof - locals.var_fn205_calc_iq__delta);
        (assign17180_e16564, locals.var_fn205_calc_iq__vtof_dn4, (-locals.var_fn205_calc_iq__delta_dn10), (-locals.var_fn205_calc_iq__delta_dn11),)
    } else {
        (locals.var_fn205_calc_iq__vtdibl, locals.var_fn205_calc_iq__vtdibl_dn4, locals.var_fn205_calc_iq__vtdibl_dn10, locals.var_fn205_calc_iq__vtdibl_dn11,)
    }
};
        locals.var_fn205_calc_iq__vtdibl = assign17180_e16566;
        locals.var_fn205_calc_iq__vtdibl_dn4 = assign17180_e16566_d_n4;
        locals.var_fn205_calc_iq__vtdibl_dn10 = assign17180_e16566_d_n10;
        locals.var_fn205_calc_iq__vtdibl_dn11 = assign17180_e16566_d_n11;

        let (assign17190_e16574, assign17190_e16574_d_n4, assign17190_e16574_d_n10, assign17190_e16574_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        let assign17190_e16570: f64 = (2.0 * locals.var_fn205_calc_iq__n);
        let assign17190_e16572: f64 = (assign17190_e16570 * locals.var_fn205_calc_iq__phitin);
        (assign17190_e16572, (((2.0 * locals.var_fn205_calc_iq__n_dn4) * locals.var_fn205_calc_iq__phitin) + (assign17190_e16570 * locals.var_fn205_calc_iq__phitin_dn4)), ((2.0 * locals.var_fn205_calc_iq__n_dn10) * locals.var_fn205_calc_iq__phitin), ((2.0 * locals.var_fn205_calc_iq__n_dn11) * locals.var_fn205_calc_iq__phitin),)
    } else {
        (locals.var_fn205_calc_iq__two_n_phit, locals.var_fn205_calc_iq__two_n_phit_dn4, locals.var_fn205_calc_iq__two_n_phit_dn10, locals.var_fn205_calc_iq__two_n_phit_dn11,)
    }
};
        locals.var_fn205_calc_iq__two_n_phit = assign17190_e16574;
        locals.var_fn205_calc_iq__two_n_phit_dn4 = assign17190_e16574_d_n4;
        locals.var_fn205_calc_iq__two_n_phit_dn10 = assign17190_e16574_d_n10;
        locals.var_fn205_calc_iq__two_n_phit_dn11 = assign17190_e16574_d_n11;

        let (assign17200_e16580, assign17200_e16580_d_n4, assign17200_e16580_d_n10, assign17200_e16580_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        let assign17200_e16578: f64 = (locals.var_fn205_calc_iq__cgin * locals.var_fn205_calc_iq__two_n_phit);
        (assign17200_e16578, ((locals.var_fn205_calc_iq__cgin_dn4 * locals.var_fn205_calc_iq__two_n_phit) + (locals.var_fn205_calc_iq__cgin * locals.var_fn205_calc_iq__two_n_phit_dn4)), (locals.var_fn205_calc_iq__cgin * locals.var_fn205_calc_iq__two_n_phit_dn10), (locals.var_fn205_calc_iq__cgin * locals.var_fn205_calc_iq__two_n_phit_dn11),)
    } else {
        (locals.var_fn205_calc_iq__qref, locals.var_fn205_calc_iq__qref_dn4, locals.var_fn205_calc_iq__qref_dn10, locals.var_fn205_calc_iq__qref_dn11,)
    }
};
        locals.var_fn205_calc_iq__qref = assign17200_e16580;
        locals.var_fn205_calc_iq__qref_dn4 = assign17200_e16580_d_n4;
        locals.var_fn205_calc_iq__qref_dn10 = assign17200_e16580_d_n10;
        locals.var_fn205_calc_iq__qref_dn11 = assign17200_e16580_d_n11;

        let (assign17210_e16590, assign17210_e16590_d_n2, assign17210_e16590_d_n3, assign17210_e16590_d_n4, assign17210_e16590_d_n7, assign17210_e16590_d_n10, assign17210_e16590_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        let assign17210_e16585: f64 = (p.p51 * locals.var_fn205_calc_iq__alpha_phit);
        let assign17210_e16587: f64 = (assign17210_e16585 / 2.0);
        let assign17210_e16588: f64 = (locals.var_fn205_calc_iq__vtdibl - assign17210_e16587);
        (assign17210_e16588, 0.0, 0.0, (locals.var_fn205_calc_iq__vtdibl_dn4 - ((p.p51 * locals.var_fn205_calc_iq__alpha_phit_dn4) / 2.0)), 0.0, locals.var_fn205_calc_iq__vtdibl_dn10, locals.var_fn205_calc_iq__vtdibl_dn11,)
    } else {
        (locals.var_fn205_calc_iq__myarg, locals.var_fn205_calc_iq__myarg_dn2, locals.var_fn205_calc_iq__myarg_dn3, locals.var_fn205_calc_iq__myarg_dn4, locals.var_fn205_calc_iq__myarg_dn7, locals.var_fn205_calc_iq__myarg_dn10, locals.var_fn205_calc_iq__myarg_dn11,)
    }
};
        locals.var_fn205_calc_iq__myarg = assign17210_e16590;
        locals.var_fn205_calc_iq__myarg_dn2 = assign17210_e16590_d_n2;
        locals.var_fn205_calc_iq__myarg_dn3 = assign17210_e16590_d_n3;
        locals.var_fn205_calc_iq__myarg_dn4 = assign17210_e16590_d_n4;
        locals.var_fn205_calc_iq__myarg_dn7 = assign17210_e16590_d_n7;
        locals.var_fn205_calc_iq__myarg_dn10 = assign17210_e16590_d_n10;
        locals.var_fn205_calc_iq__myarg_dn11 = assign17210_e16590_d_n11;

        let (assign17220_e16641, assign17220_e16641_d_n2, assign17220_e16641_d_n3, assign17220_e16641_d_n4, assign17220_e16641_d_n7, assign17220_e16641_d_n10, assign17220_e16641_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        let (assign17220_e16635, assign17220_e16635_d_n2, assign17220_e16635_d_n7, assign17220_e16635_d_n10, assign17220_e16635_d_n11,) = {
            if (p.p52 != 0.0) {
                let assign17220_e16599: f64 = (locals.var_fn205_calc_iq__vgsin + locals.var_fn205_calc_iq__vgdin);
                let assign17220_e16602: f64 = (locals.var_fn205_calc_iq__vgsin - locals.var_fn205_calc_iq__vgdin);
                let assign17220_e16605: f64 = (0.001 / p.p53);
                let assign17220_e16608: f64 = (locals.var_fn205_calc_iq__vgsin - locals.var_fn205_calc_iq__vgdin);
                let assign17220_e16609: f64 = (assign17220_e16605 * assign17220_e16608);
                let assign17220_e16610: f64 = (assign17220_e16609).tanh();
                let assign17220_e16611: f64 = (assign17220_e16602 * assign17220_e16610);
                let assign17220_e16612: f64 = (assign17220_e16599 + assign17220_e16611);
                let assign17220_e16613: f64 = (0.5 * assign17220_e16612);
                (assign17220_e16613, (0.5 * ((locals.var_fn205_calc_iq__vgsin_dn2 + locals.var_fn205_calc_iq__vgdin_dn2) + (((locals.var_fn205_calc_iq__vgsin_dn2 - locals.var_fn205_calc_iq__vgdin_dn2) * assign17220_e16610) + (assign17220_e16602 * ((assign17220_e16605 * (locals.var_fn205_calc_iq__vgsin_dn2 - locals.var_fn205_calc_iq__vgdin_dn2)) / ((assign17220_e16609).cosh() * (assign17220_e16609).cosh())))))), (0.5 * ((locals.var_fn205_calc_iq__vgsin_dn7 + locals.var_fn205_calc_iq__vgdin_dn7) + (((locals.var_fn205_calc_iq__vgsin_dn7 - locals.var_fn205_calc_iq__vgdin_dn7) * assign17220_e16610) + (assign17220_e16602 * ((assign17220_e16605 * (locals.var_fn205_calc_iq__vgsin_dn7 - locals.var_fn205_calc_iq__vgdin_dn7)) / ((assign17220_e16609).cosh() * (assign17220_e16609).cosh())))))), (0.5 * (locals.var_fn205_calc_iq__vgdin_dn10 + (((-locals.var_fn205_calc_iq__vgdin_dn10) * assign17220_e16610) + (assign17220_e16602 * ((assign17220_e16605 * (-locals.var_fn205_calc_iq__vgdin_dn10)) / ((assign17220_e16609).cosh() * (assign17220_e16609).cosh())))))), (0.5 * ((locals.var_fn205_calc_iq__vgsin_dn11 + locals.var_fn205_calc_iq__vgdin_dn11) + (((locals.var_fn205_calc_iq__vgsin_dn11 - locals.var_fn205_calc_iq__vgdin_dn11) * assign17220_e16610) + (assign17220_e16602 * ((assign17220_e16605 * (locals.var_fn205_calc_iq__vgsin_dn11 - locals.var_fn205_calc_iq__vgdin_dn11)) / ((assign17220_e16609).cosh() * (assign17220_e16609).cosh())))))),)
            } else {
                let (assign17220_e16634, assign17220_e16634_d_n2, assign17220_e16634_d_n7, assign17220_e16634_d_n10, assign17220_e16634_d_n11,) = {
                    if (p.p52 == 0.0) {
                        let assign17220_e16620: f64 = (locals.var_fn205_calc_iq__vgsin + locals.var_fn205_calc_iq__vgdin);
                        let assign17220_e16623: f64 = (locals.var_fn205_calc_iq__vgsin - locals.var_fn205_calc_iq__vgdin);
                        let assign17220_e16626: f64 = (locals.var_fn205_calc_iq__vgsin - locals.var_fn205_calc_iq__vgdin);
                        let assign17220_e16627: f64 = (assign17220_e16623 * assign17220_e16626);
                        let assign17220_e16629: f64 = (assign17220_e16627 + p.p53);
                        let assign17220_e16630: f64 = (assign17220_e16629).sqrt();
                        let assign17220_e16631: f64 = (assign17220_e16620 + assign17220_e16630);
                        let assign17220_e16632: f64 = (0.5 * assign17220_e16631);
                        (assign17220_e16632, (0.5 * ((locals.var_fn205_calc_iq__vgsin_dn2 + locals.var_fn205_calc_iq__vgdin_dn2) + ((((locals.var_fn205_calc_iq__vgsin_dn2 - locals.var_fn205_calc_iq__vgdin_dn2) * assign17220_e16626) + (assign17220_e16623 * (locals.var_fn205_calc_iq__vgsin_dn2 - locals.var_fn205_calc_iq__vgdin_dn2))) / (2.0 * assign17220_e16630)))), (0.5 * ((locals.var_fn205_calc_iq__vgsin_dn7 + locals.var_fn205_calc_iq__vgdin_dn7) + ((((locals.var_fn205_calc_iq__vgsin_dn7 - locals.var_fn205_calc_iq__vgdin_dn7) * assign17220_e16626) + (assign17220_e16623 * (locals.var_fn205_calc_iq__vgsin_dn7 - locals.var_fn205_calc_iq__vgdin_dn7))) / (2.0 * assign17220_e16630)))), (0.5 * (locals.var_fn205_calc_iq__vgdin_dn10 + ((((-locals.var_fn205_calc_iq__vgdin_dn10) * assign17220_e16626) + (assign17220_e16623 * (-locals.var_fn205_calc_iq__vgdin_dn10))) / (2.0 * assign17220_e16630)))), (0.5 * ((locals.var_fn205_calc_iq__vgsin_dn11 + locals.var_fn205_calc_iq__vgdin_dn11) + ((((locals.var_fn205_calc_iq__vgsin_dn11 - locals.var_fn205_calc_iq__vgdin_dn11) * assign17220_e16626) + (assign17220_e16623 * (locals.var_fn205_calc_iq__vgsin_dn11 - locals.var_fn205_calc_iq__vgdin_dn11))) / (2.0 * assign17220_e16630)))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign17220_e16634, assign17220_e16634_d_n2, assign17220_e16634_d_n7, assign17220_e16634_d_n10, assign17220_e16634_d_n11,)
            }
        };
        let assign17220_e16637: f64 = (assign17220_e16635 - locals.var_fn205_calc_iq__myarg);
        let assign17220_e16639: f64 = (assign17220_e16637 / locals.var_fn205_calc_iq__alpha_phit);
        (assign17220_e16639, ((assign17220_e16635_d_n2 - locals.var_fn205_calc_iq__myarg_dn2) / locals.var_fn205_calc_iq__alpha_phit), ((-locals.var_fn205_calc_iq__myarg_dn3) / locals.var_fn205_calc_iq__alpha_phit), ((((-locals.var_fn205_calc_iq__myarg_dn4) * locals.var_fn205_calc_iq__alpha_phit) - (assign17220_e16637 * locals.var_fn205_calc_iq__alpha_phit_dn4)) / (locals.var_fn205_calc_iq__alpha_phit * locals.var_fn205_calc_iq__alpha_phit)), ((assign17220_e16635_d_n7 - locals.var_fn205_calc_iq__myarg_dn7) / locals.var_fn205_calc_iq__alpha_phit), ((assign17220_e16635_d_n10 - locals.var_fn205_calc_iq__myarg_dn10) / locals.var_fn205_calc_iq__alpha_phit), ((assign17220_e16635_d_n11 - locals.var_fn205_calc_iq__myarg_dn11) / locals.var_fn205_calc_iq__alpha_phit),)
    } else {
        (locals.var_fn205_calc_iq__exparg, locals.var_fn205_calc_iq__exparg_dn2, locals.var_fn205_calc_iq__exparg_dn3, locals.var_fn205_calc_iq__exparg_dn4, locals.var_fn205_calc_iq__exparg_dn7, locals.var_fn205_calc_iq__exparg_dn10, locals.var_fn205_calc_iq__exparg_dn11,)
    }
};
        locals.var_fn205_calc_iq__exparg = assign17220_e16641;
        locals.var_fn205_calc_iq__exparg_dn2 = assign17220_e16641_d_n2;
        locals.var_fn205_calc_iq__exparg_dn3 = assign17220_e16641_d_n3;
        locals.var_fn205_calc_iq__exparg_dn4 = assign17220_e16641_d_n4;
        locals.var_fn205_calc_iq__exparg_dn7 = assign17220_e16641_d_n7;
        locals.var_fn205_calc_iq__exparg_dn10 = assign17220_e16641_d_n10;
        locals.var_fn205_calc_iq__exparg_dn11 = assign17220_e16641_d_n11;

        let assign17230_e16644: f64 = if locals.var_fn205_calc_iq__exparg > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard207 = assign17230_e16644;

        let (assign17240_e16650, assign17240_e16650_d_n2, assign17240_e16650_d_n3, assign17240_e16650_d_n4, assign17240_e16650_d_n7, assign17240_e16650_d_n10, assign17240_e16650_d_n11,) = {
    if ((locals.var_guard204 != 0.0) && (locals.var_guard207 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__ff, locals.var_fn205_calc_iq__ff_dn2, locals.var_fn205_calc_iq__ff_dn3, locals.var_fn205_calc_iq__ff_dn4, locals.var_fn205_calc_iq__ff_dn7, locals.var_fn205_calc_iq__ff_dn10, locals.var_fn205_calc_iq__ff_dn11,)
    }
};
        locals.var_fn205_calc_iq__ff = assign17240_e16650;
        locals.var_fn205_calc_iq__ff_dn2 = assign17240_e16650_d_n2;
        locals.var_fn205_calc_iq__ff_dn3 = assign17240_e16650_d_n3;
        locals.var_fn205_calc_iq__ff_dn4 = assign17240_e16650_d_n4;
        locals.var_fn205_calc_iq__ff_dn7 = assign17240_e16650_d_n7;
        locals.var_fn205_calc_iq__ff_dn10 = assign17240_e16650_d_n10;
        locals.var_fn205_calc_iq__ff_dn11 = assign17240_e16650_d_n11;

        let assign17250_e16653: f64 = (-50.0);
        let assign17250_e16654: f64 = if locals.var_fn205_calc_iq__exparg < assign17250_e16653 { 1.0 } else { 0.0 };
        locals.var_guard208 = assign17250_e16654;

        let (assign17260_e16663, assign17260_e16663_d_n2, assign17260_e16663_d_n3, assign17260_e16663_d_n4, assign17260_e16663_d_n7, assign17260_e16663_d_n10, assign17260_e16663_d_n11,) = {
    if (((locals.var_guard204 != 0.0) && (locals.var_guard207 == 0.0)) && (locals.var_guard208 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__ff, locals.var_fn205_calc_iq__ff_dn2, locals.var_fn205_calc_iq__ff_dn3, locals.var_fn205_calc_iq__ff_dn4, locals.var_fn205_calc_iq__ff_dn7, locals.var_fn205_calc_iq__ff_dn10, locals.var_fn205_calc_iq__ff_dn11,)
    }
};
        locals.var_fn205_calc_iq__ff = assign17260_e16663;
        locals.var_fn205_calc_iq__ff_dn2 = assign17260_e16663_d_n2;
        locals.var_fn205_calc_iq__ff_dn3 = assign17260_e16663_d_n3;
        locals.var_fn205_calc_iq__ff_dn4 = assign17260_e16663_d_n4;
        locals.var_fn205_calc_iq__ff_dn7 = assign17260_e16663_d_n7;
        locals.var_fn205_calc_iq__ff_dn10 = assign17260_e16663_d_n10;
        locals.var_fn205_calc_iq__ff_dn11 = assign17260_e16663_d_n11;

        let (assign17270_e16678, assign17270_e16678_d_n2, assign17270_e16678_d_n3, assign17270_e16678_d_n4, assign17270_e16678_d_n7, assign17270_e16678_d_n10, assign17270_e16678_d_n11,) = {
    if (((locals.var_guard204 != 0.0) && (locals.var_guard207 == 0.0)) && (locals.var_guard208 == 0.0)) {
        let assign17270_e16674: f64 = (locals.var_fn205_calc_iq__exparg).exp();
        let assign17270_e16675: f64 = (1.0 + assign17270_e16674);
        let assign17270_e16676: f64 = (1.0 / assign17270_e16675);
        (assign17270_e16676, (-((assign17270_e16674 * locals.var_fn205_calc_iq__exparg_dn2) / (assign17270_e16675 * assign17270_e16675))), (-((assign17270_e16674 * locals.var_fn205_calc_iq__exparg_dn3) / (assign17270_e16675 * assign17270_e16675))), (-((assign17270_e16674 * locals.var_fn205_calc_iq__exparg_dn4) / (assign17270_e16675 * assign17270_e16675))), (-((assign17270_e16674 * locals.var_fn205_calc_iq__exparg_dn7) / (assign17270_e16675 * assign17270_e16675))), (-((assign17270_e16674 * locals.var_fn205_calc_iq__exparg_dn10) / (assign17270_e16675 * assign17270_e16675))), (-((assign17270_e16674 * locals.var_fn205_calc_iq__exparg_dn11) / (assign17270_e16675 * assign17270_e16675))),)
    } else {
        (locals.var_fn205_calc_iq__ff, locals.var_fn205_calc_iq__ff_dn2, locals.var_fn205_calc_iq__ff_dn3, locals.var_fn205_calc_iq__ff_dn4, locals.var_fn205_calc_iq__ff_dn7, locals.var_fn205_calc_iq__ff_dn10, locals.var_fn205_calc_iq__ff_dn11,)
    }
};
        locals.var_fn205_calc_iq__ff = assign17270_e16678;
        locals.var_fn205_calc_iq__ff_dn2 = assign17270_e16678_d_n2;
        locals.var_fn205_calc_iq__ff_dn3 = assign17270_e16678_d_n3;
        locals.var_fn205_calc_iq__ff_dn4 = assign17270_e16678_d_n4;
        locals.var_fn205_calc_iq__ff_dn7 = assign17270_e16678_d_n7;
        locals.var_fn205_calc_iq__ff_dn10 = assign17270_e16678_d_n10;
        locals.var_fn205_calc_iq__ff_dn11 = assign17270_e16678_d_n11;

    }

    pub(super) fn stamp_transient_block_43(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign17280_e16737, assign17280_e16737_d_n2, assign17280_e16737_d_n3, assign17280_e16737_d_n4, assign17280_e16737_d_n7, assign17280_e16737_d_n10, assign17280_e16737_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        let (assign17280_e16723, assign17280_e16723_d_n2, assign17280_e16723_d_n7, assign17280_e16723_d_n10, assign17280_e16723_d_n11,) = {
            if (p.p52 != 0.0) {
                let assign17280_e16687: f64 = (locals.var_fn205_calc_iq__vgsin + locals.var_fn205_calc_iq__vgdin);
                let assign17280_e16690: f64 = (locals.var_fn205_calc_iq__vgsin - locals.var_fn205_calc_iq__vgdin);
                let assign17280_e16693: f64 = (0.001 / p.p53);
                let assign17280_e16696: f64 = (locals.var_fn205_calc_iq__vgsin - locals.var_fn205_calc_iq__vgdin);
                let assign17280_e16697: f64 = (assign17280_e16693 * assign17280_e16696);
                let assign17280_e16698: f64 = (assign17280_e16697).tanh();
                let assign17280_e16699: f64 = (assign17280_e16690 * assign17280_e16698);
                let assign17280_e16700: f64 = (assign17280_e16687 + assign17280_e16699);
                let assign17280_e16701: f64 = (0.5 * assign17280_e16700);
                (assign17280_e16701, (0.5 * ((locals.var_fn205_calc_iq__vgsin_dn2 + locals.var_fn205_calc_iq__vgdin_dn2) + (((locals.var_fn205_calc_iq__vgsin_dn2 - locals.var_fn205_calc_iq__vgdin_dn2) * assign17280_e16698) + (assign17280_e16690 * ((assign17280_e16693 * (locals.var_fn205_calc_iq__vgsin_dn2 - locals.var_fn205_calc_iq__vgdin_dn2)) / ((assign17280_e16697).cosh() * (assign17280_e16697).cosh())))))), (0.5 * ((locals.var_fn205_calc_iq__vgsin_dn7 + locals.var_fn205_calc_iq__vgdin_dn7) + (((locals.var_fn205_calc_iq__vgsin_dn7 - locals.var_fn205_calc_iq__vgdin_dn7) * assign17280_e16698) + (assign17280_e16690 * ((assign17280_e16693 * (locals.var_fn205_calc_iq__vgsin_dn7 - locals.var_fn205_calc_iq__vgdin_dn7)) / ((assign17280_e16697).cosh() * (assign17280_e16697).cosh())))))), (0.5 * (locals.var_fn205_calc_iq__vgdin_dn10 + (((-locals.var_fn205_calc_iq__vgdin_dn10) * assign17280_e16698) + (assign17280_e16690 * ((assign17280_e16693 * (-locals.var_fn205_calc_iq__vgdin_dn10)) / ((assign17280_e16697).cosh() * (assign17280_e16697).cosh())))))), (0.5 * ((locals.var_fn205_calc_iq__vgsin_dn11 + locals.var_fn205_calc_iq__vgdin_dn11) + (((locals.var_fn205_calc_iq__vgsin_dn11 - locals.var_fn205_calc_iq__vgdin_dn11) * assign17280_e16698) + (assign17280_e16690 * ((assign17280_e16693 * (locals.var_fn205_calc_iq__vgsin_dn11 - locals.var_fn205_calc_iq__vgdin_dn11)) / ((assign17280_e16697).cosh() * (assign17280_e16697).cosh())))))),)
            } else {
                let (assign17280_e16722, assign17280_e16722_d_n2, assign17280_e16722_d_n7, assign17280_e16722_d_n10, assign17280_e16722_d_n11,) = {
                    if (p.p52 == 0.0) {
                        let assign17280_e16708: f64 = (locals.var_fn205_calc_iq__vgsin + locals.var_fn205_calc_iq__vgdin);
                        let assign17280_e16711: f64 = (locals.var_fn205_calc_iq__vgsin - locals.var_fn205_calc_iq__vgdin);
                        let assign17280_e16714: f64 = (locals.var_fn205_calc_iq__vgsin - locals.var_fn205_calc_iq__vgdin);
                        let assign17280_e16715: f64 = (assign17280_e16711 * assign17280_e16714);
                        let assign17280_e16717: f64 = (assign17280_e16715 + p.p53);
                        let assign17280_e16718: f64 = (assign17280_e16717).sqrt();
                        let assign17280_e16719: f64 = (assign17280_e16708 + assign17280_e16718);
                        let assign17280_e16720: f64 = (0.5 * assign17280_e16719);
                        (assign17280_e16720, (0.5 * ((locals.var_fn205_calc_iq__vgsin_dn2 + locals.var_fn205_calc_iq__vgdin_dn2) + ((((locals.var_fn205_calc_iq__vgsin_dn2 - locals.var_fn205_calc_iq__vgdin_dn2) * assign17280_e16714) + (assign17280_e16711 * (locals.var_fn205_calc_iq__vgsin_dn2 - locals.var_fn205_calc_iq__vgdin_dn2))) / (2.0 * assign17280_e16718)))), (0.5 * ((locals.var_fn205_calc_iq__vgsin_dn7 + locals.var_fn205_calc_iq__vgdin_dn7) + ((((locals.var_fn205_calc_iq__vgsin_dn7 - locals.var_fn205_calc_iq__vgdin_dn7) * assign17280_e16714) + (assign17280_e16711 * (locals.var_fn205_calc_iq__vgsin_dn7 - locals.var_fn205_calc_iq__vgdin_dn7))) / (2.0 * assign17280_e16718)))), (0.5 * (locals.var_fn205_calc_iq__vgdin_dn10 + ((((-locals.var_fn205_calc_iq__vgdin_dn10) * assign17280_e16714) + (assign17280_e16711 * (-locals.var_fn205_calc_iq__vgdin_dn10))) / (2.0 * assign17280_e16718)))), (0.5 * ((locals.var_fn205_calc_iq__vgsin_dn11 + locals.var_fn205_calc_iq__vgdin_dn11) + ((((locals.var_fn205_calc_iq__vgsin_dn11 - locals.var_fn205_calc_iq__vgdin_dn11) * assign17280_e16714) + (assign17280_e16711 * (locals.var_fn205_calc_iq__vgsin_dn11 - locals.var_fn205_calc_iq__vgdin_dn11))) / (2.0 * assign17280_e16718)))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign17280_e16722, assign17280_e16722_d_n2, assign17280_e16722_d_n7, assign17280_e16722_d_n10, assign17280_e16722_d_n11,)
            }
        };
        let assign17280_e16727: f64 = (p.p51 * 0.1);
        let assign17280_e16729: f64 = (assign17280_e16727 * locals.var_fn205_calc_iq__alpha_phit);
        let assign17280_e16731: f64 = (assign17280_e16729 * locals.var_fn205_calc_iq__ff);
        let assign17280_e16732: f64 = (locals.var_fn205_calc_iq__vtdibl - assign17280_e16731);
        let assign17280_e16733: f64 = (assign17280_e16723 - assign17280_e16732);
        let assign17280_e16735: f64 = (assign17280_e16733 / locals.var_fn205_calc_iq__two_n_phit);
        (assign17280_e16735, ((assign17280_e16723_d_n2 - (-(assign17280_e16729 * locals.var_fn205_calc_iq__ff_dn2))) / locals.var_fn205_calc_iq__two_n_phit), ((-(-(assign17280_e16729 * locals.var_fn205_calc_iq__ff_dn3))) / locals.var_fn205_calc_iq__two_n_phit), ((((-(locals.var_fn205_calc_iq__vtdibl_dn4 - (((assign17280_e16727 * locals.var_fn205_calc_iq__alpha_phit_dn4) * locals.var_fn205_calc_iq__ff) + (assign17280_e16729 * locals.var_fn205_calc_iq__ff_dn4)))) * locals.var_fn205_calc_iq__two_n_phit) - (assign17280_e16733 * locals.var_fn205_calc_iq__two_n_phit_dn4)) / (locals.var_fn205_calc_iq__two_n_phit * locals.var_fn205_calc_iq__two_n_phit)), ((assign17280_e16723_d_n7 - (-(assign17280_e16729 * locals.var_fn205_calc_iq__ff_dn7))) / locals.var_fn205_calc_iq__two_n_phit), ((((assign17280_e16723_d_n10 - (locals.var_fn205_calc_iq__vtdibl_dn10 - (assign17280_e16729 * locals.var_fn205_calc_iq__ff_dn10))) * locals.var_fn205_calc_iq__two_n_phit) - (assign17280_e16733 * locals.var_fn205_calc_iq__two_n_phit_dn10)) / (locals.var_fn205_calc_iq__two_n_phit * locals.var_fn205_calc_iq__two_n_phit)), ((((assign17280_e16723_d_n11 - (locals.var_fn205_calc_iq__vtdibl_dn11 - (assign17280_e16729 * locals.var_fn205_calc_iq__ff_dn11))) * locals.var_fn205_calc_iq__two_n_phit) - (assign17280_e16733 * locals.var_fn205_calc_iq__two_n_phit_dn11)) / (locals.var_fn205_calc_iq__two_n_phit * locals.var_fn205_calc_iq__two_n_phit)),)
    } else {
        (locals.var_fn205_calc_iq__eta, locals.var_fn205_calc_iq__eta_dn2, locals.var_fn205_calc_iq__eta_dn3, locals.var_fn205_calc_iq__eta_dn4, locals.var_fn205_calc_iq__eta_dn7, locals.var_fn205_calc_iq__eta_dn10, locals.var_fn205_calc_iq__eta_dn11,)
    }
};
        locals.var_fn205_calc_iq__eta = assign17280_e16737;
        locals.var_fn205_calc_iq__eta_dn2 = assign17280_e16737_d_n2;
        locals.var_fn205_calc_iq__eta_dn3 = assign17280_e16737_d_n3;
        locals.var_fn205_calc_iq__eta_dn4 = assign17280_e16737_d_n4;
        locals.var_fn205_calc_iq__eta_dn7 = assign17280_e16737_d_n7;
        locals.var_fn205_calc_iq__eta_dn10 = assign17280_e16737_d_n10;
        locals.var_fn205_calc_iq__eta_dn11 = assign17280_e16737_d_n11;

        let assign17290_e16740: f64 = if locals.var_fn205_calc_iq__eta > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard209 = assign17290_e16740;

        let (assign17300_e16748, assign17300_e16748_d_n2, assign17300_e16748_d_n3, assign17300_e16748_d_n4, assign17300_e16748_d_n7, assign17300_e16748_d_n10, assign17300_e16748_d_n11,) = {
    if ((locals.var_guard204 != 0.0) && (locals.var_guard209 != 0.0)) {
        let assign17300_e16746: f64 = (locals.var_fn205_calc_iq__qref * locals.var_fn205_calc_iq__eta);
        (assign17300_e16746, (locals.var_fn205_calc_iq__qref * locals.var_fn205_calc_iq__eta_dn2), (locals.var_fn205_calc_iq__qref * locals.var_fn205_calc_iq__eta_dn3), ((locals.var_fn205_calc_iq__qref_dn4 * locals.var_fn205_calc_iq__eta) + (locals.var_fn205_calc_iq__qref * locals.var_fn205_calc_iq__eta_dn4)), (locals.var_fn205_calc_iq__qref * locals.var_fn205_calc_iq__eta_dn7), ((locals.var_fn205_calc_iq__qref_dn10 * locals.var_fn205_calc_iq__eta) + (locals.var_fn205_calc_iq__qref * locals.var_fn205_calc_iq__eta_dn10)), ((locals.var_fn205_calc_iq__qref_dn11 * locals.var_fn205_calc_iq__eta) + (locals.var_fn205_calc_iq__qref * locals.var_fn205_calc_iq__eta_dn11)),)
    } else {
        (locals.var_fn205_calc_iq__qinvv, locals.var_fn205_calc_iq__qinvv_dn2, locals.var_fn205_calc_iq__qinvv_dn3, locals.var_fn205_calc_iq__qinvv_dn4, locals.var_fn205_calc_iq__qinvv_dn7, locals.var_fn205_calc_iq__qinvv_dn10, locals.var_fn205_calc_iq__qinvv_dn11,)
    }
};
        locals.var_fn205_calc_iq__qinvv = assign17300_e16748;
        locals.var_fn205_calc_iq__qinvv_dn2 = assign17300_e16748_d_n2;
        locals.var_fn205_calc_iq__qinvv_dn3 = assign17300_e16748_d_n3;
        locals.var_fn205_calc_iq__qinvv_dn4 = assign17300_e16748_d_n4;
        locals.var_fn205_calc_iq__qinvv_dn7 = assign17300_e16748_d_n7;
        locals.var_fn205_calc_iq__qinvv_dn10 = assign17300_e16748_d_n10;
        locals.var_fn205_calc_iq__qinvv_dn11 = assign17300_e16748_d_n11;

        let assign17310_e16751: f64 = (-50.0);
        let assign17310_e16752: f64 = if locals.var_fn205_calc_iq__eta < assign17310_e16751 { 1.0 } else { 0.0 };
        locals.var_guard210 = assign17310_e16752;

        let (assign17320_e16764, assign17320_e16764_d_n2, assign17320_e16764_d_n3, assign17320_e16764_d_n4, assign17320_e16764_d_n7, assign17320_e16764_d_n10, assign17320_e16764_d_n11,) = {
    if (((locals.var_guard204 != 0.0) && (locals.var_guard209 == 0.0)) && (locals.var_guard210 != 0.0)) {
        let assign17320_e16761: f64 = (locals.var_fn205_calc_iq__eta).exp();
        let assign17320_e16762: f64 = (locals.var_fn205_calc_iq__qref * assign17320_e16761);
        (assign17320_e16762, (locals.var_fn205_calc_iq__qref * (assign17320_e16761 * locals.var_fn205_calc_iq__eta_dn2)), (locals.var_fn205_calc_iq__qref * (assign17320_e16761 * locals.var_fn205_calc_iq__eta_dn3)), ((locals.var_fn205_calc_iq__qref_dn4 * assign17320_e16761) + (locals.var_fn205_calc_iq__qref * (assign17320_e16761 * locals.var_fn205_calc_iq__eta_dn4))), (locals.var_fn205_calc_iq__qref * (assign17320_e16761 * locals.var_fn205_calc_iq__eta_dn7)), ((locals.var_fn205_calc_iq__qref_dn10 * assign17320_e16761) + (locals.var_fn205_calc_iq__qref * (assign17320_e16761 * locals.var_fn205_calc_iq__eta_dn10))), ((locals.var_fn205_calc_iq__qref_dn11 * assign17320_e16761) + (locals.var_fn205_calc_iq__qref * (assign17320_e16761 * locals.var_fn205_calc_iq__eta_dn11))),)
    } else {
        (locals.var_fn205_calc_iq__qinvv, locals.var_fn205_calc_iq__qinvv_dn2, locals.var_fn205_calc_iq__qinvv_dn3, locals.var_fn205_calc_iq__qinvv_dn4, locals.var_fn205_calc_iq__qinvv_dn7, locals.var_fn205_calc_iq__qinvv_dn10, locals.var_fn205_calc_iq__qinvv_dn11,)
    }
};
        locals.var_fn205_calc_iq__qinvv = assign17320_e16764;
        locals.var_fn205_calc_iq__qinvv_dn2 = assign17320_e16764_d_n2;
        locals.var_fn205_calc_iq__qinvv_dn3 = assign17320_e16764_d_n3;
        locals.var_fn205_calc_iq__qinvv_dn4 = assign17320_e16764_d_n4;
        locals.var_fn205_calc_iq__qinvv_dn7 = assign17320_e16764_d_n7;
        locals.var_fn205_calc_iq__qinvv_dn10 = assign17320_e16764_d_n10;
        locals.var_fn205_calc_iq__qinvv_dn11 = assign17320_e16764_d_n11;

        let (assign17330_e16780, assign17330_e16780_d_n2, assign17330_e16780_d_n3, assign17330_e16780_d_n4, assign17330_e16780_d_n7, assign17330_e16780_d_n10, assign17330_e16780_d_n11,) = {
    if (((locals.var_guard204 != 0.0) && (locals.var_guard209 == 0.0)) && (locals.var_guard210 == 0.0)) {
        let assign17330_e16775: f64 = (locals.var_fn205_calc_iq__eta).exp();
        let assign17330_e16776: f64 = (1.0 + assign17330_e16775);
        let assign17330_e16777: f64 = (assign17330_e16776).ln();
        let assign17330_e16778: f64 = (locals.var_fn205_calc_iq__qref * assign17330_e16777);
        (assign17330_e16778, (locals.var_fn205_calc_iq__qref * ((assign17330_e16775 * locals.var_fn205_calc_iq__eta_dn2) / assign17330_e16776)), (locals.var_fn205_calc_iq__qref * ((assign17330_e16775 * locals.var_fn205_calc_iq__eta_dn3) / assign17330_e16776)), ((locals.var_fn205_calc_iq__qref_dn4 * assign17330_e16777) + (locals.var_fn205_calc_iq__qref * ((assign17330_e16775 * locals.var_fn205_calc_iq__eta_dn4) / assign17330_e16776))), (locals.var_fn205_calc_iq__qref * ((assign17330_e16775 * locals.var_fn205_calc_iq__eta_dn7) / assign17330_e16776)), ((locals.var_fn205_calc_iq__qref_dn10 * assign17330_e16777) + (locals.var_fn205_calc_iq__qref * ((assign17330_e16775 * locals.var_fn205_calc_iq__eta_dn10) / assign17330_e16776))), ((locals.var_fn205_calc_iq__qref_dn11 * assign17330_e16777) + (locals.var_fn205_calc_iq__qref * ((assign17330_e16775 * locals.var_fn205_calc_iq__eta_dn11) / assign17330_e16776))),)
    } else {
        (locals.var_fn205_calc_iq__qinvv, locals.var_fn205_calc_iq__qinvv_dn2, locals.var_fn205_calc_iq__qinvv_dn3, locals.var_fn205_calc_iq__qinvv_dn4, locals.var_fn205_calc_iq__qinvv_dn7, locals.var_fn205_calc_iq__qinvv_dn10, locals.var_fn205_calc_iq__qinvv_dn11,)
    }
};
        locals.var_fn205_calc_iq__qinvv = assign17330_e16780;
        locals.var_fn205_calc_iq__qinvv_dn2 = assign17330_e16780_d_n2;
        locals.var_fn205_calc_iq__qinvv_dn3 = assign17330_e16780_d_n3;
        locals.var_fn205_calc_iq__qinvv_dn4 = assign17330_e16780_d_n4;
        locals.var_fn205_calc_iq__qinvv_dn7 = assign17330_e16780_d_n7;
        locals.var_fn205_calc_iq__qinvv_dn10 = assign17330_e16780_d_n10;
        locals.var_fn205_calc_iq__qinvv_dn11 = assign17330_e16780_d_n11;

        let (assign17340_e16794, assign17340_e16794_d_n2, assign17340_e16794_d_n3, assign17340_e16794_d_n4, assign17340_e16794_d_n7, assign17340_e16794_d_n10, assign17340_e16794_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        let assign17340_e16787: f64 = (locals.var_fn205_calc_iq__mtheta * locals.var_fn205_calc_iq__qinvv);
        let assign17340_e16789: f64 = (assign17340_e16787 / locals.var_fn205_calc_iq__cgin);
        let assign17340_e16790: f64 = (1.0 + assign17340_e16789);
        let assign17340_e16791: f64 = (locals.var_fn205_calc_iq__tfacmobin * assign17340_e16790);
        let assign17340_e16792: f64 = (locals.var_fn205_calc_iq__mu0 / assign17340_e16791);
        (assign17340_e16792, (-((locals.var_fn205_calc_iq__mu0 * (locals.var_fn205_calc_iq__tfacmobin * ((locals.var_fn205_calc_iq__mtheta * locals.var_fn205_calc_iq__qinvv_dn2) / locals.var_fn205_calc_iq__cgin))) / (assign17340_e16791 * assign17340_e16791))), (-((locals.var_fn205_calc_iq__mu0 * (locals.var_fn205_calc_iq__tfacmobin * ((locals.var_fn205_calc_iq__mtheta * locals.var_fn205_calc_iq__qinvv_dn3) / locals.var_fn205_calc_iq__cgin))) / (assign17340_e16791 * assign17340_e16791))), (-((locals.var_fn205_calc_iq__mu0 * ((locals.var_fn205_calc_iq__tfacmobin_dn4 * assign17340_e16790) + (locals.var_fn205_calc_iq__tfacmobin * ((((locals.var_fn205_calc_iq__mtheta * locals.var_fn205_calc_iq__qinvv_dn4) * locals.var_fn205_calc_iq__cgin) - (assign17340_e16787 * locals.var_fn205_calc_iq__cgin_dn4)) / (locals.var_fn205_calc_iq__cgin * locals.var_fn205_calc_iq__cgin))))) / (assign17340_e16791 * assign17340_e16791))), (-((locals.var_fn205_calc_iq__mu0 * (locals.var_fn205_calc_iq__tfacmobin * ((locals.var_fn205_calc_iq__mtheta * locals.var_fn205_calc_iq__qinvv_dn7) / locals.var_fn205_calc_iq__cgin))) / (assign17340_e16791 * assign17340_e16791))), (-((locals.var_fn205_calc_iq__mu0 * (locals.var_fn205_calc_iq__tfacmobin * ((locals.var_fn205_calc_iq__mtheta * locals.var_fn205_calc_iq__qinvv_dn10) / locals.var_fn205_calc_iq__cgin))) / (assign17340_e16791 * assign17340_e16791))), (-((locals.var_fn205_calc_iq__mu0 * (locals.var_fn205_calc_iq__tfacmobin * ((locals.var_fn205_calc_iq__mtheta * locals.var_fn205_calc_iq__qinvv_dn11) / locals.var_fn205_calc_iq__cgin))) / (assign17340_e16791 * assign17340_e16791))),)
    } else {
        (locals.var_fn205_calc_iq__muf, locals.var_fn205_calc_iq__muf_dn2, locals.var_fn205_calc_iq__muf_dn3, locals.var_fn205_calc_iq__muf_dn4, locals.var_fn205_calc_iq__muf_dn7, locals.var_fn205_calc_iq__muf_dn10, locals.var_fn205_calc_iq__muf_dn11,)
    }
};
        locals.var_fn205_calc_iq__muf = assign17340_e16794;
        locals.var_fn205_calc_iq__muf_dn2 = assign17340_e16794_d_n2;
        locals.var_fn205_calc_iq__muf_dn3 = assign17340_e16794_d_n3;
        locals.var_fn205_calc_iq__muf_dn4 = assign17340_e16794_d_n4;
        locals.var_fn205_calc_iq__muf_dn7 = assign17340_e16794_d_n7;
        locals.var_fn205_calc_iq__muf_dn10 = assign17340_e16794_d_n10;
        locals.var_fn205_calc_iq__muf_dn11 = assign17340_e16794_d_n11;

        let (assign17350_e16826, assign17350_e16826_d_n2, assign17350_e16826_d_n3, assign17350_e16826_d_n4, assign17350_e16826_d_n7, assign17350_e16826_d_n10, assign17350_e16826_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        let assign17350_e16800: f64 = (locals.var_fn205_calc_iq__vzeta * locals.var_fn205_calc_iq__tnomin);
        let assign17350_e16801: f64 = (1.0 + assign17350_e16800);
        let assign17350_e16805: f64 = (locals.var_fn205_calc_iq__vzeta * locals.var_fn205_calc_iq__tambin);
        let assign17350_e16806: f64 = (1.0 + assign17350_e16805);
        let assign17350_e16807: f64 = (assign17350_e16801 / assign17350_e16806);
        let assign17350_e16808: f64 = (locals.var_fn205_calc_iq__vel0 * assign17350_e16807);
        let assign17350_e16812: f64 = (locals.var_fn205_calc_iq__lambda * locals.var_fn205_calc_iq__absvdsin);
        let assign17350_e16814: f64 = (assign17350_e16812 / locals.var_fn205_calc_iq__lin);
        let assign17350_e16815: f64 = (1.0 + assign17350_e16814);
        let assign17350_e16816: f64 = (assign17350_e16808 * assign17350_e16815);
        let assign17350_e16820: f64 = (locals.var_fn205_calc_iq__vtheta * locals.var_fn205_calc_iq__qinvv);
        let assign17350_e16822: f64 = (assign17350_e16820 / locals.var_fn205_calc_iq__cgin);
        let assign17350_e16823: f64 = (1.0 + assign17350_e16822);
        let assign17350_e16824: f64 = (assign17350_e16816 / assign17350_e16823);
        (assign17350_e16824, (-((assign17350_e16816 * ((locals.var_fn205_calc_iq__vtheta * locals.var_fn205_calc_iq__qinvv_dn2) / locals.var_fn205_calc_iq__cgin)) / (assign17350_e16823 * assign17350_e16823))), (-((assign17350_e16816 * ((locals.var_fn205_calc_iq__vtheta * locals.var_fn205_calc_iq__qinvv_dn3) / locals.var_fn205_calc_iq__cgin)) / (assign17350_e16823 * assign17350_e16823))), (((((locals.var_fn205_calc_iq__vel0 * (-((assign17350_e16801 * (locals.var_fn205_calc_iq__vzeta * locals.var_fn205_calc_iq__tambin_dn4)) / (assign17350_e16806 * assign17350_e16806)))) * assign17350_e16815) * assign17350_e16823) - (assign17350_e16816 * ((((locals.var_fn205_calc_iq__vtheta * locals.var_fn205_calc_iq__qinvv_dn4) * locals.var_fn205_calc_iq__cgin) - (assign17350_e16820 * locals.var_fn205_calc_iq__cgin_dn4)) / (locals.var_fn205_calc_iq__cgin * locals.var_fn205_calc_iq__cgin)))) / (assign17350_e16823 * assign17350_e16823)), (-((assign17350_e16816 * ((locals.var_fn205_calc_iq__vtheta * locals.var_fn205_calc_iq__qinvv_dn7) / locals.var_fn205_calc_iq__cgin)) / (assign17350_e16823 * assign17350_e16823))), ((((assign17350_e16808 * ((locals.var_fn205_calc_iq__lambda * locals.var_fn205_calc_iq__absvdsin_dn10) / locals.var_fn205_calc_iq__lin)) * assign17350_e16823) - (assign17350_e16816 * ((locals.var_fn205_calc_iq__vtheta * locals.var_fn205_calc_iq__qinvv_dn10) / locals.var_fn205_calc_iq__cgin))) / (assign17350_e16823 * assign17350_e16823)), ((((assign17350_e16808 * ((locals.var_fn205_calc_iq__lambda * locals.var_fn205_calc_iq__absvdsin_dn11) / locals.var_fn205_calc_iq__lin)) * assign17350_e16823) - (assign17350_e16816 * ((locals.var_fn205_calc_iq__vtheta * locals.var_fn205_calc_iq__qinvv_dn11) / locals.var_fn205_calc_iq__cgin))) / (assign17350_e16823 * assign17350_e16823)),)
    } else {
        (locals.var_fn205_calc_iq__vx, locals.var_fn205_calc_iq__vx_dn2, locals.var_fn205_calc_iq__vx_dn3, locals.var_fn205_calc_iq__vx_dn4, locals.var_fn205_calc_iq__vx_dn7, locals.var_fn205_calc_iq__vx_dn10, locals.var_fn205_calc_iq__vx_dn11,)
    }
};
        locals.var_fn205_calc_iq__vx = assign17350_e16826;
        locals.var_fn205_calc_iq__vx_dn2 = assign17350_e16826_d_n2;
        locals.var_fn205_calc_iq__vx_dn3 = assign17350_e16826_d_n3;
        locals.var_fn205_calc_iq__vx_dn4 = assign17350_e16826_d_n4;
        locals.var_fn205_calc_iq__vx_dn7 = assign17350_e16826_d_n7;
        locals.var_fn205_calc_iq__vx_dn10 = assign17350_e16826_d_n10;
        locals.var_fn205_calc_iq__vx_dn11 = assign17350_e16826_d_n11;

        let (assign17370_e16852, assign17370_e16852_d_n2, assign17370_e16852_d_n3, assign17370_e16852_d_n4, assign17370_e16852_d_n7, assign17370_e16852_d_n10, assign17370_e16852_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        let assign17370_e16848: f64 = (locals.var_fn205_calc_iq__vx * locals.var_fn205_calc_iq__lin);
        let assign17370_e16850: f64 = (assign17370_e16848 / locals.var_fn205_calc_iq__muf);
        (assign17370_e16850, ((((locals.var_fn205_calc_iq__vx_dn2 * locals.var_fn205_calc_iq__lin) * locals.var_fn205_calc_iq__muf) - (assign17370_e16848 * locals.var_fn205_calc_iq__muf_dn2)) / (locals.var_fn205_calc_iq__muf * locals.var_fn205_calc_iq__muf)), ((((locals.var_fn205_calc_iq__vx_dn3 * locals.var_fn205_calc_iq__lin) * locals.var_fn205_calc_iq__muf) - (assign17370_e16848 * locals.var_fn205_calc_iq__muf_dn3)) / (locals.var_fn205_calc_iq__muf * locals.var_fn205_calc_iq__muf)), ((((locals.var_fn205_calc_iq__vx_dn4 * locals.var_fn205_calc_iq__lin) * locals.var_fn205_calc_iq__muf) - (assign17370_e16848 * locals.var_fn205_calc_iq__muf_dn4)) / (locals.var_fn205_calc_iq__muf * locals.var_fn205_calc_iq__muf)), ((((locals.var_fn205_calc_iq__vx_dn7 * locals.var_fn205_calc_iq__lin) * locals.var_fn205_calc_iq__muf) - (assign17370_e16848 * locals.var_fn205_calc_iq__muf_dn7)) / (locals.var_fn205_calc_iq__muf * locals.var_fn205_calc_iq__muf)), ((((locals.var_fn205_calc_iq__vx_dn10 * locals.var_fn205_calc_iq__lin) * locals.var_fn205_calc_iq__muf) - (assign17370_e16848 * locals.var_fn205_calc_iq__muf_dn10)) / (locals.var_fn205_calc_iq__muf * locals.var_fn205_calc_iq__muf)), ((((locals.var_fn205_calc_iq__vx_dn11 * locals.var_fn205_calc_iq__lin) * locals.var_fn205_calc_iq__muf) - (assign17370_e16848 * locals.var_fn205_calc_iq__muf_dn11)) / (locals.var_fn205_calc_iq__muf * locals.var_fn205_calc_iq__muf)),)
    } else {
        (locals.var_fn205_calc_iq__vdsats, locals.var_fn205_calc_iq__vdsats_dn2, locals.var_fn205_calc_iq__vdsats_dn3, locals.var_fn205_calc_iq__vdsats_dn4, locals.var_fn205_calc_iq__vdsats_dn7, locals.var_fn205_calc_iq__vdsats_dn10, locals.var_fn205_calc_iq__vdsats_dn11,)
    }
};
        locals.var_fn205_calc_iq__vdsats = assign17370_e16852;
        locals.var_fn205_calc_iq__vdsats_dn2 = assign17370_e16852_d_n2;
        locals.var_fn205_calc_iq__vdsats_dn3 = assign17370_e16852_d_n3;
        locals.var_fn205_calc_iq__vdsats_dn4 = assign17370_e16852_d_n4;
        locals.var_fn205_calc_iq__vdsats_dn7 = assign17370_e16852_d_n7;
        locals.var_fn205_calc_iq__vdsats_dn10 = assign17370_e16852_d_n10;
        locals.var_fn205_calc_iq__vdsats_dn11 = assign17370_e16852_d_n11;

        let (assign17380_e16869, assign17380_e16869_d_n2, assign17380_e16869_d_n3, assign17380_e16869_d_n4, assign17380_e16869_d_n7, assign17380_e16869_d_n10, assign17380_e16869_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        let assign17380_e16858: f64 = (2.0 * locals.var_fn205_calc_iq__qinvv);
        let assign17380_e16860: f64 = (assign17380_e16858 / locals.var_fn205_calc_iq__cgin);
        let assign17380_e16862: f64 = (assign17380_e16860 / locals.var_fn205_calc_iq__vdsats);
        let assign17380_e16863: f64 = (1.0 + assign17380_e16862);
        let assign17380_e16864: f64 = (assign17380_e16863).sqrt();
        let assign17380_e16865: f64 = (locals.var_fn205_calc_iq__vdsats * assign17380_e16864);
        let assign17380_e16867: f64 = (assign17380_e16865 - locals.var_fn205_calc_iq__vdsats);
        (assign17380_e16867, (((locals.var_fn205_calc_iq__vdsats_dn2 * assign17380_e16864) + (locals.var_fn205_calc_iq__vdsats * ((((((2.0 * locals.var_fn205_calc_iq__qinvv_dn2) / locals.var_fn205_calc_iq__cgin) * locals.var_fn205_calc_iq__vdsats) - (assign17380_e16860 * locals.var_fn205_calc_iq__vdsats_dn2)) / (locals.var_fn205_calc_iq__vdsats * locals.var_fn205_calc_iq__vdsats)) / (2.0 * assign17380_e16864)))) - locals.var_fn205_calc_iq__vdsats_dn2), (((locals.var_fn205_calc_iq__vdsats_dn3 * assign17380_e16864) + (locals.var_fn205_calc_iq__vdsats * ((((((2.0 * locals.var_fn205_calc_iq__qinvv_dn3) / locals.var_fn205_calc_iq__cgin) * locals.var_fn205_calc_iq__vdsats) - (assign17380_e16860 * locals.var_fn205_calc_iq__vdsats_dn3)) / (locals.var_fn205_calc_iq__vdsats * locals.var_fn205_calc_iq__vdsats)) / (2.0 * assign17380_e16864)))) - locals.var_fn205_calc_iq__vdsats_dn3), (((locals.var_fn205_calc_iq__vdsats_dn4 * assign17380_e16864) + (locals.var_fn205_calc_iq__vdsats * ((((((((2.0 * locals.var_fn205_calc_iq__qinvv_dn4) * locals.var_fn205_calc_iq__cgin) - (assign17380_e16858 * locals.var_fn205_calc_iq__cgin_dn4)) / (locals.var_fn205_calc_iq__cgin * locals.var_fn205_calc_iq__cgin)) * locals.var_fn205_calc_iq__vdsats) - (assign17380_e16860 * locals.var_fn205_calc_iq__vdsats_dn4)) / (locals.var_fn205_calc_iq__vdsats * locals.var_fn205_calc_iq__vdsats)) / (2.0 * assign17380_e16864)))) - locals.var_fn205_calc_iq__vdsats_dn4), (((locals.var_fn205_calc_iq__vdsats_dn7 * assign17380_e16864) + (locals.var_fn205_calc_iq__vdsats * ((((((2.0 * locals.var_fn205_calc_iq__qinvv_dn7) / locals.var_fn205_calc_iq__cgin) * locals.var_fn205_calc_iq__vdsats) - (assign17380_e16860 * locals.var_fn205_calc_iq__vdsats_dn7)) / (locals.var_fn205_calc_iq__vdsats * locals.var_fn205_calc_iq__vdsats)) / (2.0 * assign17380_e16864)))) - locals.var_fn205_calc_iq__vdsats_dn7), (((locals.var_fn205_calc_iq__vdsats_dn10 * assign17380_e16864) + (locals.var_fn205_calc_iq__vdsats * ((((((2.0 * locals.var_fn205_calc_iq__qinvv_dn10) / locals.var_fn205_calc_iq__cgin) * locals.var_fn205_calc_iq__vdsats) - (assign17380_e16860 * locals.var_fn205_calc_iq__vdsats_dn10)) / (locals.var_fn205_calc_iq__vdsats * locals.var_fn205_calc_iq__vdsats)) / (2.0 * assign17380_e16864)))) - locals.var_fn205_calc_iq__vdsats_dn10), (((locals.var_fn205_calc_iq__vdsats_dn11 * assign17380_e16864) + (locals.var_fn205_calc_iq__vdsats * ((((((2.0 * locals.var_fn205_calc_iq__qinvv_dn11) / locals.var_fn205_calc_iq__cgin) * locals.var_fn205_calc_iq__vdsats) - (assign17380_e16860 * locals.var_fn205_calc_iq__vdsats_dn11)) / (locals.var_fn205_calc_iq__vdsats * locals.var_fn205_calc_iq__vdsats)) / (2.0 * assign17380_e16864)))) - locals.var_fn205_calc_iq__vdsats_dn11),)
    } else {
        (locals.var_fn205_calc_iq__vdsats1, locals.var_fn205_calc_iq__vdsats1_dn2, locals.var_fn205_calc_iq__vdsats1_dn3, locals.var_fn205_calc_iq__vdsats1_dn4, locals.var_fn205_calc_iq__vdsats1_dn7, locals.var_fn205_calc_iq__vdsats1_dn10, locals.var_fn205_calc_iq__vdsats1_dn11,)
    }
};
        locals.var_fn205_calc_iq__vdsats1 = assign17380_e16869;
        locals.var_fn205_calc_iq__vdsats1_dn2 = assign17380_e16869_d_n2;
        locals.var_fn205_calc_iq__vdsats1_dn3 = assign17380_e16869_d_n3;
        locals.var_fn205_calc_iq__vdsats1_dn4 = assign17380_e16869_d_n4;
        locals.var_fn205_calc_iq__vdsats1_dn7 = assign17380_e16869_d_n7;
        locals.var_fn205_calc_iq__vdsats1_dn10 = assign17380_e16869_d_n10;
        locals.var_fn205_calc_iq__vdsats1_dn11 = assign17380_e16869_d_n11;

        let (assign17390_e16881, assign17390_e16881_d_n2, assign17390_e16881_d_n3, assign17390_e16881_d_n4, assign17390_e16881_d_n7, assign17390_e16881_d_n10, assign17390_e16881_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        let assign17390_e16874: f64 = (1.0 - locals.var_fn205_calc_iq__ff);
        let assign17390_e16875: f64 = (locals.var_fn205_calc_iq__vdsats * assign17390_e16874);
        let assign17390_e16878: f64 = (locals.var_fn205_calc_iq__two_n_phit * locals.var_fn205_calc_iq__ff);
        let assign17390_e16879: f64 = (assign17390_e16875 + assign17390_e16878);
        (assign17390_e16879, (((locals.var_fn205_calc_iq__vdsats_dn2 * assign17390_e16874) + (locals.var_fn205_calc_iq__vdsats * (-locals.var_fn205_calc_iq__ff_dn2))) + (locals.var_fn205_calc_iq__two_n_phit * locals.var_fn205_calc_iq__ff_dn2)), (((locals.var_fn205_calc_iq__vdsats_dn3 * assign17390_e16874) + (locals.var_fn205_calc_iq__vdsats * (-locals.var_fn205_calc_iq__ff_dn3))) + (locals.var_fn205_calc_iq__two_n_phit * locals.var_fn205_calc_iq__ff_dn3)), (((locals.var_fn205_calc_iq__vdsats_dn4 * assign17390_e16874) + (locals.var_fn205_calc_iq__vdsats * (-locals.var_fn205_calc_iq__ff_dn4))) + ((locals.var_fn205_calc_iq__two_n_phit_dn4 * locals.var_fn205_calc_iq__ff) + (locals.var_fn205_calc_iq__two_n_phit * locals.var_fn205_calc_iq__ff_dn4))), (((locals.var_fn205_calc_iq__vdsats_dn7 * assign17390_e16874) + (locals.var_fn205_calc_iq__vdsats * (-locals.var_fn205_calc_iq__ff_dn7))) + (locals.var_fn205_calc_iq__two_n_phit * locals.var_fn205_calc_iq__ff_dn7)), (((locals.var_fn205_calc_iq__vdsats_dn10 * assign17390_e16874) + (locals.var_fn205_calc_iq__vdsats * (-locals.var_fn205_calc_iq__ff_dn10))) + ((locals.var_fn205_calc_iq__two_n_phit_dn10 * locals.var_fn205_calc_iq__ff) + (locals.var_fn205_calc_iq__two_n_phit * locals.var_fn205_calc_iq__ff_dn10))), (((locals.var_fn205_calc_iq__vdsats_dn11 * assign17390_e16874) + (locals.var_fn205_calc_iq__vdsats * (-locals.var_fn205_calc_iq__ff_dn11))) + ((locals.var_fn205_calc_iq__two_n_phit_dn11 * locals.var_fn205_calc_iq__ff) + (locals.var_fn205_calc_iq__two_n_phit * locals.var_fn205_calc_iq__ff_dn11))),)
    } else {
        (locals.var_fn205_calc_iq__vdsat, locals.var_fn205_calc_iq__vdsat_dn2, locals.var_fn205_calc_iq__vdsat_dn3, locals.var_fn205_calc_iq__vdsat_dn4, locals.var_fn205_calc_iq__vdsat_dn7, locals.var_fn205_calc_iq__vdsat_dn10, locals.var_fn205_calc_iq__vdsat_dn11,)
    }
};
        locals.var_fn205_calc_iq__vdsat = assign17390_e16881;
        locals.var_fn205_calc_iq__vdsat_dn2 = assign17390_e16881_d_n2;
        locals.var_fn205_calc_iq__vdsat_dn3 = assign17390_e16881_d_n3;
        locals.var_fn205_calc_iq__vdsat_dn4 = assign17390_e16881_d_n4;
        locals.var_fn205_calc_iq__vdsat_dn7 = assign17390_e16881_d_n7;
        locals.var_fn205_calc_iq__vdsat_dn10 = assign17390_e16881_d_n10;
        locals.var_fn205_calc_iq__vdsat_dn11 = assign17390_e16881_d_n11;

        let (assign17400_e16893, assign17400_e16893_d_n2, assign17400_e16893_d_n3, assign17400_e16893_d_n4, assign17400_e16893_d_n7, assign17400_e16893_d_n10, assign17400_e16893_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        let assign17400_e16886: f64 = (1.0 - locals.var_fn205_calc_iq__ff);
        let assign17400_e16887: f64 = (locals.var_fn205_calc_iq__vdsats1 * assign17400_e16886);
        let assign17400_e16890: f64 = (locals.var_fn205_calc_iq__two_n_phit * locals.var_fn205_calc_iq__ff);
        let assign17400_e16891: f64 = (assign17400_e16887 + assign17400_e16890);
        (assign17400_e16891, (((locals.var_fn205_calc_iq__vdsats1_dn2 * assign17400_e16886) + (locals.var_fn205_calc_iq__vdsats1 * (-locals.var_fn205_calc_iq__ff_dn2))) + (locals.var_fn205_calc_iq__two_n_phit * locals.var_fn205_calc_iq__ff_dn2)), (((locals.var_fn205_calc_iq__vdsats1_dn3 * assign17400_e16886) + (locals.var_fn205_calc_iq__vdsats1 * (-locals.var_fn205_calc_iq__ff_dn3))) + (locals.var_fn205_calc_iq__two_n_phit * locals.var_fn205_calc_iq__ff_dn3)), (((locals.var_fn205_calc_iq__vdsats1_dn4 * assign17400_e16886) + (locals.var_fn205_calc_iq__vdsats1 * (-locals.var_fn205_calc_iq__ff_dn4))) + ((locals.var_fn205_calc_iq__two_n_phit_dn4 * locals.var_fn205_calc_iq__ff) + (locals.var_fn205_calc_iq__two_n_phit * locals.var_fn205_calc_iq__ff_dn4))), (((locals.var_fn205_calc_iq__vdsats1_dn7 * assign17400_e16886) + (locals.var_fn205_calc_iq__vdsats1 * (-locals.var_fn205_calc_iq__ff_dn7))) + (locals.var_fn205_calc_iq__two_n_phit * locals.var_fn205_calc_iq__ff_dn7)), (((locals.var_fn205_calc_iq__vdsats1_dn10 * assign17400_e16886) + (locals.var_fn205_calc_iq__vdsats1 * (-locals.var_fn205_calc_iq__ff_dn10))) + ((locals.var_fn205_calc_iq__two_n_phit_dn10 * locals.var_fn205_calc_iq__ff) + (locals.var_fn205_calc_iq__two_n_phit * locals.var_fn205_calc_iq__ff_dn10))), (((locals.var_fn205_calc_iq__vdsats1_dn11 * assign17400_e16886) + (locals.var_fn205_calc_iq__vdsats1 * (-locals.var_fn205_calc_iq__ff_dn11))) + ((locals.var_fn205_calc_iq__two_n_phit_dn11 * locals.var_fn205_calc_iq__ff) + (locals.var_fn205_calc_iq__two_n_phit * locals.var_fn205_calc_iq__ff_dn11))),)
    } else {
        (locals.var_fn205_calc_iq__vdsat1, locals.var_fn205_calc_iq__vdsat1_dn2, locals.var_fn205_calc_iq__vdsat1_dn3, locals.var_fn205_calc_iq__vdsat1_dn4, locals.var_fn205_calc_iq__vdsat1_dn7, locals.var_fn205_calc_iq__vdsat1_dn10, locals.var_fn205_calc_iq__vdsat1_dn11,)
    }
};
        locals.var_fn205_calc_iq__vdsat1 = assign17400_e16893;
        locals.var_fn205_calc_iq__vdsat1_dn2 = assign17400_e16893_d_n2;
        locals.var_fn205_calc_iq__vdsat1_dn3 = assign17400_e16893_d_n3;
        locals.var_fn205_calc_iq__vdsat1_dn4 = assign17400_e16893_d_n4;
        locals.var_fn205_calc_iq__vdsat1_dn7 = assign17400_e16893_d_n7;
        locals.var_fn205_calc_iq__vdsat1_dn10 = assign17400_e16893_d_n10;
        locals.var_fn205_calc_iq__vdsat1_dn11 = assign17400_e16893_d_n11;

        let (assign17410_e16962, assign17410_e16962_d_n2, assign17410_e16962_d_n3, assign17410_e16962_d_n4, assign17410_e16962_d_n7, assign17410_e16962_d_n10, assign17410_e16962_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        let (assign17410_e16952, assign17410_e16952_d_n2, assign17410_e16952_d_n3, assign17410_e16952_d_n4, assign17410_e16952_d_n7, assign17410_e16952_d_n10, assign17410_e16952_d_n11,) = {
            if (p.p52 != 0.0) {
                let assign17410_e16905: f64 = (locals.var_fn205_calc_iq__vdsin / locals.var_fn205_calc_iq__vdsat1);
                let assign17410_e16906: f64 = assign17410_e16905;
                let assign17410_e16910: f64 = (locals.var_fn205_calc_iq__vdsin / locals.var_fn205_calc_iq__vdsat1);
                let assign17410_e16911: f64 = (-assign17410_e16910);
                let assign17410_e16914: f64 = (0.001 / p.p53);
                let assign17410_e16918: f64 = (locals.var_fn205_calc_iq__vdsin / locals.var_fn205_calc_iq__vdsat1);
                let assign17410_e16919: f64 = (-assign17410_e16918);
                let assign17410_e16920: f64 = (assign17410_e16914 * assign17410_e16919);
                let assign17410_e16921: f64 = (assign17410_e16920).tanh();
                let assign17410_e16922: f64 = (assign17410_e16911 * assign17410_e16921);
                let assign17410_e16923: f64 = (assign17410_e16906 + assign17410_e16922);
                let assign17410_e16924: f64 = (0.5 * assign17410_e16923);
                (assign17410_e16924, (0.5 * ((-((locals.var_fn205_calc_iq__vdsin * locals.var_fn205_calc_iq__vdsat1_dn2) / (locals.var_fn205_calc_iq__vdsat1 * locals.var_fn205_calc_iq__vdsat1))) + (((-(-((locals.var_fn205_calc_iq__vdsin * locals.var_fn205_calc_iq__vdsat1_dn2) / (locals.var_fn205_calc_iq__vdsat1 * locals.var_fn205_calc_iq__vdsat1)))) * assign17410_e16921) + (assign17410_e16911 * ((assign17410_e16914 * (-(-((locals.var_fn205_calc_iq__vdsin * locals.var_fn205_calc_iq__vdsat1_dn2) / (locals.var_fn205_calc_iq__vdsat1 * locals.var_fn205_calc_iq__vdsat1))))) / ((assign17410_e16920).cosh() * (assign17410_e16920).cosh())))))), (0.5 * ((-((locals.var_fn205_calc_iq__vdsin * locals.var_fn205_calc_iq__vdsat1_dn3) / (locals.var_fn205_calc_iq__vdsat1 * locals.var_fn205_calc_iq__vdsat1))) + (((-(-((locals.var_fn205_calc_iq__vdsin * locals.var_fn205_calc_iq__vdsat1_dn3) / (locals.var_fn205_calc_iq__vdsat1 * locals.var_fn205_calc_iq__vdsat1)))) * assign17410_e16921) + (assign17410_e16911 * ((assign17410_e16914 * (-(-((locals.var_fn205_calc_iq__vdsin * locals.var_fn205_calc_iq__vdsat1_dn3) / (locals.var_fn205_calc_iq__vdsat1 * locals.var_fn205_calc_iq__vdsat1))))) / ((assign17410_e16920).cosh() * (assign17410_e16920).cosh())))))), (0.5 * ((-((locals.var_fn205_calc_iq__vdsin * locals.var_fn205_calc_iq__vdsat1_dn4) / (locals.var_fn205_calc_iq__vdsat1 * locals.var_fn205_calc_iq__vdsat1))) + (((-(-((locals.var_fn205_calc_iq__vdsin * locals.var_fn205_calc_iq__vdsat1_dn4) / (locals.var_fn205_calc_iq__vdsat1 * locals.var_fn205_calc_iq__vdsat1)))) * assign17410_e16921) + (assign17410_e16911 * ((assign17410_e16914 * (-(-((locals.var_fn205_calc_iq__vdsin * locals.var_fn205_calc_iq__vdsat1_dn4) / (locals.var_fn205_calc_iq__vdsat1 * locals.var_fn205_calc_iq__vdsat1))))) / ((assign17410_e16920).cosh() * (assign17410_e16920).cosh())))))), (0.5 * ((-((locals.var_fn205_calc_iq__vdsin * locals.var_fn205_calc_iq__vdsat1_dn7) / (locals.var_fn205_calc_iq__vdsat1 * locals.var_fn205_calc_iq__vdsat1))) + (((-(-((locals.var_fn205_calc_iq__vdsin * locals.var_fn205_calc_iq__vdsat1_dn7) / (locals.var_fn205_calc_iq__vdsat1 * locals.var_fn205_calc_iq__vdsat1)))) * assign17410_e16921) + (assign17410_e16911 * ((assign17410_e16914 * (-(-((locals.var_fn205_calc_iq__vdsin * locals.var_fn205_calc_iq__vdsat1_dn7) / (locals.var_fn205_calc_iq__vdsat1 * locals.var_fn205_calc_iq__vdsat1))))) / ((assign17410_e16920).cosh() * (assign17410_e16920).cosh())))))), (0.5 * ((((locals.var_fn205_calc_iq__vdsin_dn10 * locals.var_fn205_calc_iq__vdsat1) - (locals.var_fn205_calc_iq__vdsin * locals.var_fn205_calc_iq__vdsat1_dn10)) / (locals.var_fn205_calc_iq__vdsat1 * locals.var_fn205_calc_iq__vdsat1)) + (((-(((locals.var_fn205_calc_iq__vdsin_dn10 * locals.var_fn205_calc_iq__vdsat1) - (locals.var_fn205_calc_iq__vdsin * locals.var_fn205_calc_iq__vdsat1_dn10)) / (locals.var_fn205_calc_iq__vdsat1 * locals.var_fn205_calc_iq__vdsat1))) * assign17410_e16921) + (assign17410_e16911 * ((assign17410_e16914 * (-(((locals.var_fn205_calc_iq__vdsin_dn10 * locals.var_fn205_calc_iq__vdsat1) - (locals.var_fn205_calc_iq__vdsin * locals.var_fn205_calc_iq__vdsat1_dn10)) / (locals.var_fn205_calc_iq__vdsat1 * locals.var_fn205_calc_iq__vdsat1)))) / ((assign17410_e16920).cosh() * (assign17410_e16920).cosh())))))), (0.5 * ((((locals.var_fn205_calc_iq__vdsin_dn11 * locals.var_fn205_calc_iq__vdsat1) - (locals.var_fn205_calc_iq__vdsin * locals.var_fn205_calc_iq__vdsat1_dn11)) / (locals.var_fn205_calc_iq__vdsat1 * locals.var_fn205_calc_iq__vdsat1)) + (((-(((locals.var_fn205_calc_iq__vdsin_dn11 * locals.var_fn205_calc_iq__vdsat1) - (locals.var_fn205_calc_iq__vdsin * locals.var_fn205_calc_iq__vdsat1_dn11)) / (locals.var_fn205_calc_iq__vdsat1 * locals.var_fn205_calc_iq__vdsat1))) * assign17410_e16921) + (assign17410_e16911 * ((assign17410_e16914 * (-(((locals.var_fn205_calc_iq__vdsin_dn11 * locals.var_fn205_calc_iq__vdsat1) - (locals.var_fn205_calc_iq__vdsin * locals.var_fn205_calc_iq__vdsat1_dn11)) / (locals.var_fn205_calc_iq__vdsat1 * locals.var_fn205_calc_iq__vdsat1)))) / ((assign17410_e16920).cosh() * (assign17410_e16920).cosh())))))),)
            } else {
                let (assign17410_e16951, assign17410_e16951_d_n2, assign17410_e16951_d_n3, assign17410_e16951_d_n4, assign17410_e16951_d_n7, assign17410_e16951_d_n10, assign17410_e16951_d_n11,) = {
                    if (p.p52 == 0.0) {
                        let assign17410_e16932: f64 = (locals.var_fn205_calc_iq__vdsin / locals.var_fn205_calc_iq__vdsat1);
                        let assign17410_e16933: f64 = assign17410_e16932;
                        let assign17410_e16937: f64 = (locals.var_fn205_calc_iq__vdsin / locals.var_fn205_calc_iq__vdsat1);
                        let assign17410_e16938: f64 = (-assign17410_e16937);
                        let assign17410_e16942: f64 = (locals.var_fn205_calc_iq__vdsin / locals.var_fn205_calc_iq__vdsat1);
                        let assign17410_e16943: f64 = (-assign17410_e16942);
                        let assign17410_e16944: f64 = (assign17410_e16938 * assign17410_e16943);
                        let assign17410_e16946: f64 = (assign17410_e16944 + p.p53);
                        let assign17410_e16947: f64 = (assign17410_e16946).sqrt();
                        let assign17410_e16948: f64 = (assign17410_e16933 + assign17410_e16947);
                        let assign17410_e16949: f64 = (0.5 * assign17410_e16948);
                        (assign17410_e16949, (0.5 * ((-((locals.var_fn205_calc_iq__vdsin * locals.var_fn205_calc_iq__vdsat1_dn2) / (locals.var_fn205_calc_iq__vdsat1 * locals.var_fn205_calc_iq__vdsat1))) + ((((-(-((locals.var_fn205_calc_iq__vdsin * locals.var_fn205_calc_iq__vdsat1_dn2) / (locals.var_fn205_calc_iq__vdsat1 * locals.var_fn205_calc_iq__vdsat1)))) * assign17410_e16943) + (assign17410_e16938 * (-(-((locals.var_fn205_calc_iq__vdsin * locals.var_fn205_calc_iq__vdsat1_dn2) / (locals.var_fn205_calc_iq__vdsat1 * locals.var_fn205_calc_iq__vdsat1)))))) / (2.0 * assign17410_e16947)))), (0.5 * ((-((locals.var_fn205_calc_iq__vdsin * locals.var_fn205_calc_iq__vdsat1_dn3) / (locals.var_fn205_calc_iq__vdsat1 * locals.var_fn205_calc_iq__vdsat1))) + ((((-(-((locals.var_fn205_calc_iq__vdsin * locals.var_fn205_calc_iq__vdsat1_dn3) / (locals.var_fn205_calc_iq__vdsat1 * locals.var_fn205_calc_iq__vdsat1)))) * assign17410_e16943) + (assign17410_e16938 * (-(-((locals.var_fn205_calc_iq__vdsin * locals.var_fn205_calc_iq__vdsat1_dn3) / (locals.var_fn205_calc_iq__vdsat1 * locals.var_fn205_calc_iq__vdsat1)))))) / (2.0 * assign17410_e16947)))), (0.5 * ((-((locals.var_fn205_calc_iq__vdsin * locals.var_fn205_calc_iq__vdsat1_dn4) / (locals.var_fn205_calc_iq__vdsat1 * locals.var_fn205_calc_iq__vdsat1))) + ((((-(-((locals.var_fn205_calc_iq__vdsin * locals.var_fn205_calc_iq__vdsat1_dn4) / (locals.var_fn205_calc_iq__vdsat1 * locals.var_fn205_calc_iq__vdsat1)))) * assign17410_e16943) + (assign17410_e16938 * (-(-((locals.var_fn205_calc_iq__vdsin * locals.var_fn205_calc_iq__vdsat1_dn4) / (locals.var_fn205_calc_iq__vdsat1 * locals.var_fn205_calc_iq__vdsat1)))))) / (2.0 * assign17410_e16947)))), (0.5 * ((-((locals.var_fn205_calc_iq__vdsin * locals.var_fn205_calc_iq__vdsat1_dn7) / (locals.var_fn205_calc_iq__vdsat1 * locals.var_fn205_calc_iq__vdsat1))) + ((((-(-((locals.var_fn205_calc_iq__vdsin * locals.var_fn205_calc_iq__vdsat1_dn7) / (locals.var_fn205_calc_iq__vdsat1 * locals.var_fn205_calc_iq__vdsat1)))) * assign17410_e16943) + (assign17410_e16938 * (-(-((locals.var_fn205_calc_iq__vdsin * locals.var_fn205_calc_iq__vdsat1_dn7) / (locals.var_fn205_calc_iq__vdsat1 * locals.var_fn205_calc_iq__vdsat1)))))) / (2.0 * assign17410_e16947)))), (0.5 * ((((locals.var_fn205_calc_iq__vdsin_dn10 * locals.var_fn205_calc_iq__vdsat1) - (locals.var_fn205_calc_iq__vdsin * locals.var_fn205_calc_iq__vdsat1_dn10)) / (locals.var_fn205_calc_iq__vdsat1 * locals.var_fn205_calc_iq__vdsat1)) + ((((-(((locals.var_fn205_calc_iq__vdsin_dn10 * locals.var_fn205_calc_iq__vdsat1) - (locals.var_fn205_calc_iq__vdsin * locals.var_fn205_calc_iq__vdsat1_dn10)) / (locals.var_fn205_calc_iq__vdsat1 * locals.var_fn205_calc_iq__vdsat1))) * assign17410_e16943) + (assign17410_e16938 * (-(((locals.var_fn205_calc_iq__vdsin_dn10 * locals.var_fn205_calc_iq__vdsat1) - (locals.var_fn205_calc_iq__vdsin * locals.var_fn205_calc_iq__vdsat1_dn10)) / (locals.var_fn205_calc_iq__vdsat1 * locals.var_fn205_calc_iq__vdsat1))))) / (2.0 * assign17410_e16947)))), (0.5 * ((((locals.var_fn205_calc_iq__vdsin_dn11 * locals.var_fn205_calc_iq__vdsat1) - (locals.var_fn205_calc_iq__vdsin * locals.var_fn205_calc_iq__vdsat1_dn11)) / (locals.var_fn205_calc_iq__vdsat1 * locals.var_fn205_calc_iq__vdsat1)) + ((((-(((locals.var_fn205_calc_iq__vdsin_dn11 * locals.var_fn205_calc_iq__vdsat1) - (locals.var_fn205_calc_iq__vdsin * locals.var_fn205_calc_iq__vdsat1_dn11)) / (locals.var_fn205_calc_iq__vdsat1 * locals.var_fn205_calc_iq__vdsat1))) * assign17410_e16943) + (assign17410_e16938 * (-(((locals.var_fn205_calc_iq__vdsin_dn11 * locals.var_fn205_calc_iq__vdsat1) - (locals.var_fn205_calc_iq__vdsin * locals.var_fn205_calc_iq__vdsat1_dn11)) / (locals.var_fn205_calc_iq__vdsat1 * locals.var_fn205_calc_iq__vdsat1))))) / (2.0 * assign17410_e16947)))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign17410_e16951, assign17410_e16951_d_n2, assign17410_e16951_d_n3, assign17410_e16951_d_n4, assign17410_e16951_d_n7, assign17410_e16951_d_n10, assign17410_e16951_d_n11,)
            }
        };
        let assign17410_e16954: f64 = (assign17410_e16952).powf(locals.var_fn205_calc_iq__beta);
        let assign17410_e16955: f64 = (1.0 + assign17410_e16954);
        let assign17410_e16958: f64 = (1.0 / locals.var_fn205_calc_iq__beta);
        let assign17410_e16959: f64 = (assign17410_e16955).powf(assign17410_e16958);
        let assign17410_e16960: f64 = (1.0 / assign17410_e16959);
        (assign17410_e16960, (-(if 0.0 == 0.0 && ((assign17410_e16958) as f64).is_finite() && ((assign17410_e16958) as f64).fract() == 0.0 { if assign17410_e16958 == 0.0 { 0.0 } else { (assign17410_e16958 * ((assign17410_e16955).powf(assign17410_e16958 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn205_calc_iq__beta) as f64).is_finite() && ((locals.var_fn205_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn205_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn205_calc_iq__beta * ((assign17410_e16952).powf(locals.var_fn205_calc_iq__beta - 1.0) * assign17410_e16952_d_n2)) } } else { (assign17410_e16954 * (locals.var_fn205_calc_iq__beta * (assign17410_e16952_d_n2 / assign17410_e16952))) })) } } else { (assign17410_e16959 * (assign17410_e16958 * (if 0.0 == 0.0 && ((locals.var_fn205_calc_iq__beta) as f64).is_finite() && ((locals.var_fn205_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn205_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn205_calc_iq__beta * ((assign17410_e16952).powf(locals.var_fn205_calc_iq__beta - 1.0) * assign17410_e16952_d_n2)) } } else { (assign17410_e16954 * (locals.var_fn205_calc_iq__beta * (assign17410_e16952_d_n2 / assign17410_e16952))) } / assign17410_e16955))) } / (assign17410_e16959 * assign17410_e16959))), (-(if 0.0 == 0.0 && ((assign17410_e16958) as f64).is_finite() && ((assign17410_e16958) as f64).fract() == 0.0 { if assign17410_e16958 == 0.0 { 0.0 } else { (assign17410_e16958 * ((assign17410_e16955).powf(assign17410_e16958 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn205_calc_iq__beta) as f64).is_finite() && ((locals.var_fn205_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn205_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn205_calc_iq__beta * ((assign17410_e16952).powf(locals.var_fn205_calc_iq__beta - 1.0) * assign17410_e16952_d_n3)) } } else { (assign17410_e16954 * (locals.var_fn205_calc_iq__beta * (assign17410_e16952_d_n3 / assign17410_e16952))) })) } } else { (assign17410_e16959 * (assign17410_e16958 * (if 0.0 == 0.0 && ((locals.var_fn205_calc_iq__beta) as f64).is_finite() && ((locals.var_fn205_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn205_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn205_calc_iq__beta * ((assign17410_e16952).powf(locals.var_fn205_calc_iq__beta - 1.0) * assign17410_e16952_d_n3)) } } else { (assign17410_e16954 * (locals.var_fn205_calc_iq__beta * (assign17410_e16952_d_n3 / assign17410_e16952))) } / assign17410_e16955))) } / (assign17410_e16959 * assign17410_e16959))), (-(if 0.0 == 0.0 && ((assign17410_e16958) as f64).is_finite() && ((assign17410_e16958) as f64).fract() == 0.0 { if assign17410_e16958 == 0.0 { 0.0 } else { (assign17410_e16958 * ((assign17410_e16955).powf(assign17410_e16958 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn205_calc_iq__beta) as f64).is_finite() && ((locals.var_fn205_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn205_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn205_calc_iq__beta * ((assign17410_e16952).powf(locals.var_fn205_calc_iq__beta - 1.0) * assign17410_e16952_d_n4)) } } else { (assign17410_e16954 * (locals.var_fn205_calc_iq__beta * (assign17410_e16952_d_n4 / assign17410_e16952))) })) } } else { (assign17410_e16959 * (assign17410_e16958 * (if 0.0 == 0.0 && ((locals.var_fn205_calc_iq__beta) as f64).is_finite() && ((locals.var_fn205_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn205_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn205_calc_iq__beta * ((assign17410_e16952).powf(locals.var_fn205_calc_iq__beta - 1.0) * assign17410_e16952_d_n4)) } } else { (assign17410_e16954 * (locals.var_fn205_calc_iq__beta * (assign17410_e16952_d_n4 / assign17410_e16952))) } / assign17410_e16955))) } / (assign17410_e16959 * assign17410_e16959))), (-(if 0.0 == 0.0 && ((assign17410_e16958) as f64).is_finite() && ((assign17410_e16958) as f64).fract() == 0.0 { if assign17410_e16958 == 0.0 { 0.0 } else { (assign17410_e16958 * ((assign17410_e16955).powf(assign17410_e16958 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn205_calc_iq__beta) as f64).is_finite() && ((locals.var_fn205_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn205_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn205_calc_iq__beta * ((assign17410_e16952).powf(locals.var_fn205_calc_iq__beta - 1.0) * assign17410_e16952_d_n7)) } } else { (assign17410_e16954 * (locals.var_fn205_calc_iq__beta * (assign17410_e16952_d_n7 / assign17410_e16952))) })) } } else { (assign17410_e16959 * (assign17410_e16958 * (if 0.0 == 0.0 && ((locals.var_fn205_calc_iq__beta) as f64).is_finite() && ((locals.var_fn205_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn205_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn205_calc_iq__beta * ((assign17410_e16952).powf(locals.var_fn205_calc_iq__beta - 1.0) * assign17410_e16952_d_n7)) } } else { (assign17410_e16954 * (locals.var_fn205_calc_iq__beta * (assign17410_e16952_d_n7 / assign17410_e16952))) } / assign17410_e16955))) } / (assign17410_e16959 * assign17410_e16959))), (-(if 0.0 == 0.0 && ((assign17410_e16958) as f64).is_finite() && ((assign17410_e16958) as f64).fract() == 0.0 { if assign17410_e16958 == 0.0 { 0.0 } else { (assign17410_e16958 * ((assign17410_e16955).powf(assign17410_e16958 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn205_calc_iq__beta) as f64).is_finite() && ((locals.var_fn205_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn205_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn205_calc_iq__beta * ((assign17410_e16952).powf(locals.var_fn205_calc_iq__beta - 1.0) * assign17410_e16952_d_n10)) } } else { (assign17410_e16954 * (locals.var_fn205_calc_iq__beta * (assign17410_e16952_d_n10 / assign17410_e16952))) })) } } else { (assign17410_e16959 * (assign17410_e16958 * (if 0.0 == 0.0 && ((locals.var_fn205_calc_iq__beta) as f64).is_finite() && ((locals.var_fn205_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn205_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn205_calc_iq__beta * ((assign17410_e16952).powf(locals.var_fn205_calc_iq__beta - 1.0) * assign17410_e16952_d_n10)) } } else { (assign17410_e16954 * (locals.var_fn205_calc_iq__beta * (assign17410_e16952_d_n10 / assign17410_e16952))) } / assign17410_e16955))) } / (assign17410_e16959 * assign17410_e16959))), (-(if 0.0 == 0.0 && ((assign17410_e16958) as f64).is_finite() && ((assign17410_e16958) as f64).fract() == 0.0 { if assign17410_e16958 == 0.0 { 0.0 } else { (assign17410_e16958 * ((assign17410_e16955).powf(assign17410_e16958 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn205_calc_iq__beta) as f64).is_finite() && ((locals.var_fn205_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn205_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn205_calc_iq__beta * ((assign17410_e16952).powf(locals.var_fn205_calc_iq__beta - 1.0) * assign17410_e16952_d_n11)) } } else { (assign17410_e16954 * (locals.var_fn205_calc_iq__beta * (assign17410_e16952_d_n11 / assign17410_e16952))) })) } } else { (assign17410_e16959 * (assign17410_e16958 * (if 0.0 == 0.0 && ((locals.var_fn205_calc_iq__beta) as f64).is_finite() && ((locals.var_fn205_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn205_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn205_calc_iq__beta * ((assign17410_e16952).powf(locals.var_fn205_calc_iq__beta - 1.0) * assign17410_e16952_d_n11)) } } else { (assign17410_e16954 * (locals.var_fn205_calc_iq__beta * (assign17410_e16952_d_n11 / assign17410_e16952))) } / assign17410_e16955))) } / (assign17410_e16959 * assign17410_e16959))),)
    } else {
        (locals.var_fn205_calc_iq__fsd, locals.var_fn205_calc_iq__fsd_dn2, locals.var_fn205_calc_iq__fsd_dn3, locals.var_fn205_calc_iq__fsd_dn4, locals.var_fn205_calc_iq__fsd_dn7, locals.var_fn205_calc_iq__fsd_dn10, locals.var_fn205_calc_iq__fsd_dn11,)
    }
};
        locals.var_fn205_calc_iq__fsd = assign17410_e16962;
        locals.var_fn205_calc_iq__fsd_dn2 = assign17410_e16962_d_n2;
        locals.var_fn205_calc_iq__fsd_dn3 = assign17410_e16962_d_n3;
        locals.var_fn205_calc_iq__fsd_dn4 = assign17410_e16962_d_n4;
        locals.var_fn205_calc_iq__fsd_dn7 = assign17410_e16962_d_n7;
        locals.var_fn205_calc_iq__fsd_dn10 = assign17410_e16962_d_n10;
        locals.var_fn205_calc_iq__fsd_dn11 = assign17410_e16962_d_n11;

        let (assign17420_e16968, assign17420_e16968_d_n2, assign17420_e16968_d_n3, assign17420_e16968_d_n4, assign17420_e16968_d_n7, assign17420_e16968_d_n10, assign17420_e16968_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        let assign17420_e16966: f64 = (locals.var_fn205_calc_iq__vdsin * locals.var_fn205_calc_iq__fsd);
        (assign17420_e16966, (locals.var_fn205_calc_iq__vdsin * locals.var_fn205_calc_iq__fsd_dn2), (locals.var_fn205_calc_iq__vdsin * locals.var_fn205_calc_iq__fsd_dn3), (locals.var_fn205_calc_iq__vdsin * locals.var_fn205_calc_iq__fsd_dn4), (locals.var_fn205_calc_iq__vdsin * locals.var_fn205_calc_iq__fsd_dn7), ((locals.var_fn205_calc_iq__vdsin_dn10 * locals.var_fn205_calc_iq__fsd) + (locals.var_fn205_calc_iq__vdsin * locals.var_fn205_calc_iq__fsd_dn10)), ((locals.var_fn205_calc_iq__vdsin_dn11 * locals.var_fn205_calc_iq__fsd) + (locals.var_fn205_calc_iq__vdsin * locals.var_fn205_calc_iq__fsd_dn11)),)
    } else {
        (locals.var_fn205_calc_iq__vdx, locals.var_fn205_calc_iq__vdx_dn2, locals.var_fn205_calc_iq__vdx_dn3, locals.var_fn205_calc_iq__vdx_dn4, locals.var_fn205_calc_iq__vdx_dn7, locals.var_fn205_calc_iq__vdx_dn10, locals.var_fn205_calc_iq__vdx_dn11,)
    }
};
        locals.var_fn205_calc_iq__vdx = assign17420_e16968;
        locals.var_fn205_calc_iq__vdx_dn2 = assign17420_e16968_d_n2;
        locals.var_fn205_calc_iq__vdx_dn3 = assign17420_e16968_d_n3;
        locals.var_fn205_calc_iq__vdx_dn4 = assign17420_e16968_d_n4;
        locals.var_fn205_calc_iq__vdx_dn7 = assign17420_e16968_d_n7;
        locals.var_fn205_calc_iq__vdx_dn10 = assign17420_e16968_d_n10;
        locals.var_fn205_calc_iq__vdx_dn11 = assign17420_e16968_d_n11;

        let (assign17430_e17043, assign17430_e17043_d_n2, assign17430_e17043_d_n3, assign17430_e17043_d_n4, assign17430_e17043_d_n7, assign17430_e17043_d_n10, assign17430_e17043_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        let (assign17430_e17033, assign17430_e17033_d_n2, assign17430_e17033_d_n3, assign17430_e17033_d_n4, assign17430_e17033_d_n7, assign17430_e17033_d_n10, assign17430_e17033_d_n11,) = {
            if (p.p52 != 0.0) {
                let assign17430_e16979: f64 = (-locals.var_fn205_calc_iq__vdsin);
                let assign17430_e16981: f64 = (assign17430_e16979 / locals.var_fn205_calc_iq__vdsat1);
                let assign17430_e16982: f64 = assign17430_e16981;
                let assign17430_e16985: f64 = (-locals.var_fn205_calc_iq__vdsin);
                let assign17430_e16987: f64 = (assign17430_e16985 / locals.var_fn205_calc_iq__vdsat1);
                let assign17430_e16988: f64 = (-assign17430_e16987);
                let assign17430_e16991: f64 = (0.001 / p.p53);
                let assign17430_e16994: f64 = (-locals.var_fn205_calc_iq__vdsin);
                let assign17430_e16996: f64 = (assign17430_e16994 / locals.var_fn205_calc_iq__vdsat1);
                let assign17430_e16997: f64 = (-assign17430_e16996);
                let assign17430_e16998: f64 = (assign17430_e16991 * assign17430_e16997);
                let assign17430_e16999: f64 = (assign17430_e16998).tanh();
                let assign17430_e17000: f64 = (assign17430_e16988 * assign17430_e16999);
                let assign17430_e17001: f64 = (assign17430_e16982 + assign17430_e17000);
                let assign17430_e17002: f64 = (0.5 * assign17430_e17001);
                (assign17430_e17002, (0.5 * ((-((assign17430_e16979 * locals.var_fn205_calc_iq__vdsat1_dn2) / (locals.var_fn205_calc_iq__vdsat1 * locals.var_fn205_calc_iq__vdsat1))) + (((-(-((assign17430_e16985 * locals.var_fn205_calc_iq__vdsat1_dn2) / (locals.var_fn205_calc_iq__vdsat1 * locals.var_fn205_calc_iq__vdsat1)))) * assign17430_e16999) + (assign17430_e16988 * ((assign17430_e16991 * (-(-((assign17430_e16994 * locals.var_fn205_calc_iq__vdsat1_dn2) / (locals.var_fn205_calc_iq__vdsat1 * locals.var_fn205_calc_iq__vdsat1))))) / ((assign17430_e16998).cosh() * (assign17430_e16998).cosh())))))), (0.5 * ((-((assign17430_e16979 * locals.var_fn205_calc_iq__vdsat1_dn3) / (locals.var_fn205_calc_iq__vdsat1 * locals.var_fn205_calc_iq__vdsat1))) + (((-(-((assign17430_e16985 * locals.var_fn205_calc_iq__vdsat1_dn3) / (locals.var_fn205_calc_iq__vdsat1 * locals.var_fn205_calc_iq__vdsat1)))) * assign17430_e16999) + (assign17430_e16988 * ((assign17430_e16991 * (-(-((assign17430_e16994 * locals.var_fn205_calc_iq__vdsat1_dn3) / (locals.var_fn205_calc_iq__vdsat1 * locals.var_fn205_calc_iq__vdsat1))))) / ((assign17430_e16998).cosh() * (assign17430_e16998).cosh())))))), (0.5 * ((-((assign17430_e16979 * locals.var_fn205_calc_iq__vdsat1_dn4) / (locals.var_fn205_calc_iq__vdsat1 * locals.var_fn205_calc_iq__vdsat1))) + (((-(-((assign17430_e16985 * locals.var_fn205_calc_iq__vdsat1_dn4) / (locals.var_fn205_calc_iq__vdsat1 * locals.var_fn205_calc_iq__vdsat1)))) * assign17430_e16999) + (assign17430_e16988 * ((assign17430_e16991 * (-(-((assign17430_e16994 * locals.var_fn205_calc_iq__vdsat1_dn4) / (locals.var_fn205_calc_iq__vdsat1 * locals.var_fn205_calc_iq__vdsat1))))) / ((assign17430_e16998).cosh() * (assign17430_e16998).cosh())))))), (0.5 * ((-((assign17430_e16979 * locals.var_fn205_calc_iq__vdsat1_dn7) / (locals.var_fn205_calc_iq__vdsat1 * locals.var_fn205_calc_iq__vdsat1))) + (((-(-((assign17430_e16985 * locals.var_fn205_calc_iq__vdsat1_dn7) / (locals.var_fn205_calc_iq__vdsat1 * locals.var_fn205_calc_iq__vdsat1)))) * assign17430_e16999) + (assign17430_e16988 * ((assign17430_e16991 * (-(-((assign17430_e16994 * locals.var_fn205_calc_iq__vdsat1_dn7) / (locals.var_fn205_calc_iq__vdsat1 * locals.var_fn205_calc_iq__vdsat1))))) / ((assign17430_e16998).cosh() * (assign17430_e16998).cosh())))))), (0.5 * (((((-locals.var_fn205_calc_iq__vdsin_dn10) * locals.var_fn205_calc_iq__vdsat1) - (assign17430_e16979 * locals.var_fn205_calc_iq__vdsat1_dn10)) / (locals.var_fn205_calc_iq__vdsat1 * locals.var_fn205_calc_iq__vdsat1)) + (((-((((-locals.var_fn205_calc_iq__vdsin_dn10) * locals.var_fn205_calc_iq__vdsat1) - (assign17430_e16985 * locals.var_fn205_calc_iq__vdsat1_dn10)) / (locals.var_fn205_calc_iq__vdsat1 * locals.var_fn205_calc_iq__vdsat1))) * assign17430_e16999) + (assign17430_e16988 * ((assign17430_e16991 * (-((((-locals.var_fn205_calc_iq__vdsin_dn10) * locals.var_fn205_calc_iq__vdsat1) - (assign17430_e16994 * locals.var_fn205_calc_iq__vdsat1_dn10)) / (locals.var_fn205_calc_iq__vdsat1 * locals.var_fn205_calc_iq__vdsat1)))) / ((assign17430_e16998).cosh() * (assign17430_e16998).cosh())))))), (0.5 * (((((-locals.var_fn205_calc_iq__vdsin_dn11) * locals.var_fn205_calc_iq__vdsat1) - (assign17430_e16979 * locals.var_fn205_calc_iq__vdsat1_dn11)) / (locals.var_fn205_calc_iq__vdsat1 * locals.var_fn205_calc_iq__vdsat1)) + (((-((((-locals.var_fn205_calc_iq__vdsin_dn11) * locals.var_fn205_calc_iq__vdsat1) - (assign17430_e16985 * locals.var_fn205_calc_iq__vdsat1_dn11)) / (locals.var_fn205_calc_iq__vdsat1 * locals.var_fn205_calc_iq__vdsat1))) * assign17430_e16999) + (assign17430_e16988 * ((assign17430_e16991 * (-((((-locals.var_fn205_calc_iq__vdsin_dn11) * locals.var_fn205_calc_iq__vdsat1) - (assign17430_e16994 * locals.var_fn205_calc_iq__vdsat1_dn11)) / (locals.var_fn205_calc_iq__vdsat1 * locals.var_fn205_calc_iq__vdsat1)))) / ((assign17430_e16998).cosh() * (assign17430_e16998).cosh())))))),)
            } else {
                let (assign17430_e17032, assign17430_e17032_d_n2, assign17430_e17032_d_n3, assign17430_e17032_d_n4, assign17430_e17032_d_n7, assign17430_e17032_d_n10, assign17430_e17032_d_n11,) = {
                    if (p.p52 == 0.0) {
                        let assign17430_e17009: f64 = (-locals.var_fn205_calc_iq__vdsin);
                        let assign17430_e17011: f64 = (assign17430_e17009 / locals.var_fn205_calc_iq__vdsat1);
                        let assign17430_e17012: f64 = assign17430_e17011;
                        let assign17430_e17015: f64 = (-locals.var_fn205_calc_iq__vdsin);
                        let assign17430_e17017: f64 = (assign17430_e17015 / locals.var_fn205_calc_iq__vdsat1);
                        let assign17430_e17018: f64 = (-assign17430_e17017);
                        let assign17430_e17021: f64 = (-locals.var_fn205_calc_iq__vdsin);
                        let assign17430_e17023: f64 = (assign17430_e17021 / locals.var_fn205_calc_iq__vdsat1);
                        let assign17430_e17024: f64 = (-assign17430_e17023);
                        let assign17430_e17025: f64 = (assign17430_e17018 * assign17430_e17024);
                        let assign17430_e17027: f64 = (assign17430_e17025 + p.p53);
                        let assign17430_e17028: f64 = (assign17430_e17027).sqrt();
                        let assign17430_e17029: f64 = (assign17430_e17012 + assign17430_e17028);
                        let assign17430_e17030: f64 = (0.5 * assign17430_e17029);
                        (assign17430_e17030, (0.5 * ((-((assign17430_e17009 * locals.var_fn205_calc_iq__vdsat1_dn2) / (locals.var_fn205_calc_iq__vdsat1 * locals.var_fn205_calc_iq__vdsat1))) + ((((-(-((assign17430_e17015 * locals.var_fn205_calc_iq__vdsat1_dn2) / (locals.var_fn205_calc_iq__vdsat1 * locals.var_fn205_calc_iq__vdsat1)))) * assign17430_e17024) + (assign17430_e17018 * (-(-((assign17430_e17021 * locals.var_fn205_calc_iq__vdsat1_dn2) / (locals.var_fn205_calc_iq__vdsat1 * locals.var_fn205_calc_iq__vdsat1)))))) / (2.0 * assign17430_e17028)))), (0.5 * ((-((assign17430_e17009 * locals.var_fn205_calc_iq__vdsat1_dn3) / (locals.var_fn205_calc_iq__vdsat1 * locals.var_fn205_calc_iq__vdsat1))) + ((((-(-((assign17430_e17015 * locals.var_fn205_calc_iq__vdsat1_dn3) / (locals.var_fn205_calc_iq__vdsat1 * locals.var_fn205_calc_iq__vdsat1)))) * assign17430_e17024) + (assign17430_e17018 * (-(-((assign17430_e17021 * locals.var_fn205_calc_iq__vdsat1_dn3) / (locals.var_fn205_calc_iq__vdsat1 * locals.var_fn205_calc_iq__vdsat1)))))) / (2.0 * assign17430_e17028)))), (0.5 * ((-((assign17430_e17009 * locals.var_fn205_calc_iq__vdsat1_dn4) / (locals.var_fn205_calc_iq__vdsat1 * locals.var_fn205_calc_iq__vdsat1))) + ((((-(-((assign17430_e17015 * locals.var_fn205_calc_iq__vdsat1_dn4) / (locals.var_fn205_calc_iq__vdsat1 * locals.var_fn205_calc_iq__vdsat1)))) * assign17430_e17024) + (assign17430_e17018 * (-(-((assign17430_e17021 * locals.var_fn205_calc_iq__vdsat1_dn4) / (locals.var_fn205_calc_iq__vdsat1 * locals.var_fn205_calc_iq__vdsat1)))))) / (2.0 * assign17430_e17028)))), (0.5 * ((-((assign17430_e17009 * locals.var_fn205_calc_iq__vdsat1_dn7) / (locals.var_fn205_calc_iq__vdsat1 * locals.var_fn205_calc_iq__vdsat1))) + ((((-(-((assign17430_e17015 * locals.var_fn205_calc_iq__vdsat1_dn7) / (locals.var_fn205_calc_iq__vdsat1 * locals.var_fn205_calc_iq__vdsat1)))) * assign17430_e17024) + (assign17430_e17018 * (-(-((assign17430_e17021 * locals.var_fn205_calc_iq__vdsat1_dn7) / (locals.var_fn205_calc_iq__vdsat1 * locals.var_fn205_calc_iq__vdsat1)))))) / (2.0 * assign17430_e17028)))), (0.5 * (((((-locals.var_fn205_calc_iq__vdsin_dn10) * locals.var_fn205_calc_iq__vdsat1) - (assign17430_e17009 * locals.var_fn205_calc_iq__vdsat1_dn10)) / (locals.var_fn205_calc_iq__vdsat1 * locals.var_fn205_calc_iq__vdsat1)) + ((((-((((-locals.var_fn205_calc_iq__vdsin_dn10) * locals.var_fn205_calc_iq__vdsat1) - (assign17430_e17015 * locals.var_fn205_calc_iq__vdsat1_dn10)) / (locals.var_fn205_calc_iq__vdsat1 * locals.var_fn205_calc_iq__vdsat1))) * assign17430_e17024) + (assign17430_e17018 * (-((((-locals.var_fn205_calc_iq__vdsin_dn10) * locals.var_fn205_calc_iq__vdsat1) - (assign17430_e17021 * locals.var_fn205_calc_iq__vdsat1_dn10)) / (locals.var_fn205_calc_iq__vdsat1 * locals.var_fn205_calc_iq__vdsat1))))) / (2.0 * assign17430_e17028)))), (0.5 * (((((-locals.var_fn205_calc_iq__vdsin_dn11) * locals.var_fn205_calc_iq__vdsat1) - (assign17430_e17009 * locals.var_fn205_calc_iq__vdsat1_dn11)) / (locals.var_fn205_calc_iq__vdsat1 * locals.var_fn205_calc_iq__vdsat1)) + ((((-((((-locals.var_fn205_calc_iq__vdsin_dn11) * locals.var_fn205_calc_iq__vdsat1) - (assign17430_e17015 * locals.var_fn205_calc_iq__vdsat1_dn11)) / (locals.var_fn205_calc_iq__vdsat1 * locals.var_fn205_calc_iq__vdsat1))) * assign17430_e17024) + (assign17430_e17018 * (-((((-locals.var_fn205_calc_iq__vdsin_dn11) * locals.var_fn205_calc_iq__vdsat1) - (assign17430_e17021 * locals.var_fn205_calc_iq__vdsat1_dn11)) / (locals.var_fn205_calc_iq__vdsat1 * locals.var_fn205_calc_iq__vdsat1))))) / (2.0 * assign17430_e17028)))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign17430_e17032, assign17430_e17032_d_n2, assign17430_e17032_d_n3, assign17430_e17032_d_n4, assign17430_e17032_d_n7, assign17430_e17032_d_n10, assign17430_e17032_d_n11,)
            }
        };
        let assign17430_e17035: f64 = (assign17430_e17033).powf(locals.var_fn205_calc_iq__beta);
        let assign17430_e17036: f64 = (1.0 + assign17430_e17035);
        let assign17430_e17039: f64 = (1.0 / locals.var_fn205_calc_iq__beta);
        let assign17430_e17040: f64 = (assign17430_e17036).powf(assign17430_e17039);
        let assign17430_e17041: f64 = (1.0 / assign17430_e17040);
        (assign17430_e17041, (-(if 0.0 == 0.0 && ((assign17430_e17039) as f64).is_finite() && ((assign17430_e17039) as f64).fract() == 0.0 { if assign17430_e17039 == 0.0 { 0.0 } else { (assign17430_e17039 * ((assign17430_e17036).powf(assign17430_e17039 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn205_calc_iq__beta) as f64).is_finite() && ((locals.var_fn205_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn205_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn205_calc_iq__beta * ((assign17430_e17033).powf(locals.var_fn205_calc_iq__beta - 1.0) * assign17430_e17033_d_n2)) } } else { (assign17430_e17035 * (locals.var_fn205_calc_iq__beta * (assign17430_e17033_d_n2 / assign17430_e17033))) })) } } else { (assign17430_e17040 * (assign17430_e17039 * (if 0.0 == 0.0 && ((locals.var_fn205_calc_iq__beta) as f64).is_finite() && ((locals.var_fn205_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn205_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn205_calc_iq__beta * ((assign17430_e17033).powf(locals.var_fn205_calc_iq__beta - 1.0) * assign17430_e17033_d_n2)) } } else { (assign17430_e17035 * (locals.var_fn205_calc_iq__beta * (assign17430_e17033_d_n2 / assign17430_e17033))) } / assign17430_e17036))) } / (assign17430_e17040 * assign17430_e17040))), (-(if 0.0 == 0.0 && ((assign17430_e17039) as f64).is_finite() && ((assign17430_e17039) as f64).fract() == 0.0 { if assign17430_e17039 == 0.0 { 0.0 } else { (assign17430_e17039 * ((assign17430_e17036).powf(assign17430_e17039 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn205_calc_iq__beta) as f64).is_finite() && ((locals.var_fn205_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn205_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn205_calc_iq__beta * ((assign17430_e17033).powf(locals.var_fn205_calc_iq__beta - 1.0) * assign17430_e17033_d_n3)) } } else { (assign17430_e17035 * (locals.var_fn205_calc_iq__beta * (assign17430_e17033_d_n3 / assign17430_e17033))) })) } } else { (assign17430_e17040 * (assign17430_e17039 * (if 0.0 == 0.0 && ((locals.var_fn205_calc_iq__beta) as f64).is_finite() && ((locals.var_fn205_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn205_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn205_calc_iq__beta * ((assign17430_e17033).powf(locals.var_fn205_calc_iq__beta - 1.0) * assign17430_e17033_d_n3)) } } else { (assign17430_e17035 * (locals.var_fn205_calc_iq__beta * (assign17430_e17033_d_n3 / assign17430_e17033))) } / assign17430_e17036))) } / (assign17430_e17040 * assign17430_e17040))), (-(if 0.0 == 0.0 && ((assign17430_e17039) as f64).is_finite() && ((assign17430_e17039) as f64).fract() == 0.0 { if assign17430_e17039 == 0.0 { 0.0 } else { (assign17430_e17039 * ((assign17430_e17036).powf(assign17430_e17039 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn205_calc_iq__beta) as f64).is_finite() && ((locals.var_fn205_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn205_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn205_calc_iq__beta * ((assign17430_e17033).powf(locals.var_fn205_calc_iq__beta - 1.0) * assign17430_e17033_d_n4)) } } else { (assign17430_e17035 * (locals.var_fn205_calc_iq__beta * (assign17430_e17033_d_n4 / assign17430_e17033))) })) } } else { (assign17430_e17040 * (assign17430_e17039 * (if 0.0 == 0.0 && ((locals.var_fn205_calc_iq__beta) as f64).is_finite() && ((locals.var_fn205_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn205_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn205_calc_iq__beta * ((assign17430_e17033).powf(locals.var_fn205_calc_iq__beta - 1.0) * assign17430_e17033_d_n4)) } } else { (assign17430_e17035 * (locals.var_fn205_calc_iq__beta * (assign17430_e17033_d_n4 / assign17430_e17033))) } / assign17430_e17036))) } / (assign17430_e17040 * assign17430_e17040))), (-(if 0.0 == 0.0 && ((assign17430_e17039) as f64).is_finite() && ((assign17430_e17039) as f64).fract() == 0.0 { if assign17430_e17039 == 0.0 { 0.0 } else { (assign17430_e17039 * ((assign17430_e17036).powf(assign17430_e17039 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn205_calc_iq__beta) as f64).is_finite() && ((locals.var_fn205_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn205_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn205_calc_iq__beta * ((assign17430_e17033).powf(locals.var_fn205_calc_iq__beta - 1.0) * assign17430_e17033_d_n7)) } } else { (assign17430_e17035 * (locals.var_fn205_calc_iq__beta * (assign17430_e17033_d_n7 / assign17430_e17033))) })) } } else { (assign17430_e17040 * (assign17430_e17039 * (if 0.0 == 0.0 && ((locals.var_fn205_calc_iq__beta) as f64).is_finite() && ((locals.var_fn205_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn205_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn205_calc_iq__beta * ((assign17430_e17033).powf(locals.var_fn205_calc_iq__beta - 1.0) * assign17430_e17033_d_n7)) } } else { (assign17430_e17035 * (locals.var_fn205_calc_iq__beta * (assign17430_e17033_d_n7 / assign17430_e17033))) } / assign17430_e17036))) } / (assign17430_e17040 * assign17430_e17040))), (-(if 0.0 == 0.0 && ((assign17430_e17039) as f64).is_finite() && ((assign17430_e17039) as f64).fract() == 0.0 { if assign17430_e17039 == 0.0 { 0.0 } else { (assign17430_e17039 * ((assign17430_e17036).powf(assign17430_e17039 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn205_calc_iq__beta) as f64).is_finite() && ((locals.var_fn205_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn205_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn205_calc_iq__beta * ((assign17430_e17033).powf(locals.var_fn205_calc_iq__beta - 1.0) * assign17430_e17033_d_n10)) } } else { (assign17430_e17035 * (locals.var_fn205_calc_iq__beta * (assign17430_e17033_d_n10 / assign17430_e17033))) })) } } else { (assign17430_e17040 * (assign17430_e17039 * (if 0.0 == 0.0 && ((locals.var_fn205_calc_iq__beta) as f64).is_finite() && ((locals.var_fn205_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn205_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn205_calc_iq__beta * ((assign17430_e17033).powf(locals.var_fn205_calc_iq__beta - 1.0) * assign17430_e17033_d_n10)) } } else { (assign17430_e17035 * (locals.var_fn205_calc_iq__beta * (assign17430_e17033_d_n10 / assign17430_e17033))) } / assign17430_e17036))) } / (assign17430_e17040 * assign17430_e17040))), (-(if 0.0 == 0.0 && ((assign17430_e17039) as f64).is_finite() && ((assign17430_e17039) as f64).fract() == 0.0 { if assign17430_e17039 == 0.0 { 0.0 } else { (assign17430_e17039 * ((assign17430_e17036).powf(assign17430_e17039 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn205_calc_iq__beta) as f64).is_finite() && ((locals.var_fn205_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn205_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn205_calc_iq__beta * ((assign17430_e17033).powf(locals.var_fn205_calc_iq__beta - 1.0) * assign17430_e17033_d_n11)) } } else { (assign17430_e17035 * (locals.var_fn205_calc_iq__beta * (assign17430_e17033_d_n11 / assign17430_e17033))) })) } } else { (assign17430_e17040 * (assign17430_e17039 * (if 0.0 == 0.0 && ((locals.var_fn205_calc_iq__beta) as f64).is_finite() && ((locals.var_fn205_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn205_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn205_calc_iq__beta * ((assign17430_e17033).powf(locals.var_fn205_calc_iq__beta - 1.0) * assign17430_e17033_d_n11)) } } else { (assign17430_e17035 * (locals.var_fn205_calc_iq__beta * (assign17430_e17033_d_n11 / assign17430_e17033))) } / assign17430_e17036))) } / (assign17430_e17040 * assign17430_e17040))),)
    } else {
        (locals.var_fn205_calc_iq__fds, locals.var_fn205_calc_iq__fds_dn2, locals.var_fn205_calc_iq__fds_dn3, locals.var_fn205_calc_iq__fds_dn4, locals.var_fn205_calc_iq__fds_dn7, locals.var_fn205_calc_iq__fds_dn10, locals.var_fn205_calc_iq__fds_dn11,)
    }
};
        locals.var_fn205_calc_iq__fds = assign17430_e17043;
        locals.var_fn205_calc_iq__fds_dn2 = assign17430_e17043_d_n2;
        locals.var_fn205_calc_iq__fds_dn3 = assign17430_e17043_d_n3;
        locals.var_fn205_calc_iq__fds_dn4 = assign17430_e17043_d_n4;
        locals.var_fn205_calc_iq__fds_dn7 = assign17430_e17043_d_n7;
        locals.var_fn205_calc_iq__fds_dn10 = assign17430_e17043_d_n10;
        locals.var_fn205_calc_iq__fds_dn11 = assign17430_e17043_d_n11;

        let (assign17440_e17050, assign17440_e17050_d_n2, assign17440_e17050_d_n3, assign17440_e17050_d_n4, assign17440_e17050_d_n7, assign17440_e17050_d_n10, assign17440_e17050_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        let assign17440_e17046: f64 = (-locals.var_fn205_calc_iq__vdsin);
        let assign17440_e17048: f64 = (assign17440_e17046 * locals.var_fn205_calc_iq__fds);
        (assign17440_e17048, (assign17440_e17046 * locals.var_fn205_calc_iq__fds_dn2), (assign17440_e17046 * locals.var_fn205_calc_iq__fds_dn3), (assign17440_e17046 * locals.var_fn205_calc_iq__fds_dn4), (assign17440_e17046 * locals.var_fn205_calc_iq__fds_dn7), (((-locals.var_fn205_calc_iq__vdsin_dn10) * locals.var_fn205_calc_iq__fds) + (assign17440_e17046 * locals.var_fn205_calc_iq__fds_dn10)), (((-locals.var_fn205_calc_iq__vdsin_dn11) * locals.var_fn205_calc_iq__fds) + (assign17440_e17046 * locals.var_fn205_calc_iq__fds_dn11)),)
    } else {
        (locals.var_fn205_calc_iq__vsx, locals.var_fn205_calc_iq__vsx_dn2, locals.var_fn205_calc_iq__vsx_dn3, locals.var_fn205_calc_iq__vsx_dn4, locals.var_fn205_calc_iq__vsx_dn7, locals.var_fn205_calc_iq__vsx_dn10, locals.var_fn205_calc_iq__vsx_dn11,)
    }
};
        locals.var_fn205_calc_iq__vsx = assign17440_e17050;
        locals.var_fn205_calc_iq__vsx_dn2 = assign17440_e17050_d_n2;
        locals.var_fn205_calc_iq__vsx_dn3 = assign17440_e17050_d_n3;
        locals.var_fn205_calc_iq__vsx_dn4 = assign17440_e17050_d_n4;
        locals.var_fn205_calc_iq__vsx_dn7 = assign17440_e17050_d_n7;
        locals.var_fn205_calc_iq__vsx_dn10 = assign17440_e17050_d_n10;
        locals.var_fn205_calc_iq__vsx_dn11 = assign17440_e17050_d_n11;

        let (assign17450_e17058, assign17450_e17058_d_n2, assign17450_e17058_d_n3, assign17450_e17058_d_n4, assign17450_e17058_d_n7, assign17450_e17058_d_n10, assign17450_e17058_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        let assign17450_e17054: f64 = (locals.var_fn205_calc_iq__vgsin - locals.var_fn205_calc_iq__myarg);
        let assign17450_e17056: f64 = (assign17450_e17054 / locals.var_fn205_calc_iq__alpha_phit);
        (assign17450_e17056, ((locals.var_fn205_calc_iq__vgsin_dn2 - locals.var_fn205_calc_iq__myarg_dn2) / locals.var_fn205_calc_iq__alpha_phit), ((-locals.var_fn205_calc_iq__myarg_dn3) / locals.var_fn205_calc_iq__alpha_phit), ((((-locals.var_fn205_calc_iq__myarg_dn4) * locals.var_fn205_calc_iq__alpha_phit) - (assign17450_e17054 * locals.var_fn205_calc_iq__alpha_phit_dn4)) / (locals.var_fn205_calc_iq__alpha_phit * locals.var_fn205_calc_iq__alpha_phit)), ((locals.var_fn205_calc_iq__vgsin_dn7 - locals.var_fn205_calc_iq__myarg_dn7) / locals.var_fn205_calc_iq__alpha_phit), ((-locals.var_fn205_calc_iq__myarg_dn10) / locals.var_fn205_calc_iq__alpha_phit), ((locals.var_fn205_calc_iq__vgsin_dn11 - locals.var_fn205_calc_iq__myarg_dn11) / locals.var_fn205_calc_iq__alpha_phit),)
    } else {
        (locals.var_fn205_calc_iq__exparg, locals.var_fn205_calc_iq__exparg_dn2, locals.var_fn205_calc_iq__exparg_dn3, locals.var_fn205_calc_iq__exparg_dn4, locals.var_fn205_calc_iq__exparg_dn7, locals.var_fn205_calc_iq__exparg_dn10, locals.var_fn205_calc_iq__exparg_dn11,)
    }
};
        locals.var_fn205_calc_iq__exparg = assign17450_e17058;
        locals.var_fn205_calc_iq__exparg_dn2 = assign17450_e17058_d_n2;
        locals.var_fn205_calc_iq__exparg_dn3 = assign17450_e17058_d_n3;
        locals.var_fn205_calc_iq__exparg_dn4 = assign17450_e17058_d_n4;
        locals.var_fn205_calc_iq__exparg_dn7 = assign17450_e17058_d_n7;
        locals.var_fn205_calc_iq__exparg_dn10 = assign17450_e17058_d_n10;
        locals.var_fn205_calc_iq__exparg_dn11 = assign17450_e17058_d_n11;

        let assign17460_e17061: f64 = if locals.var_fn205_calc_iq__exparg > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard211 = assign17460_e17061;

        let (assign17470_e17067, assign17470_e17067_d_n2, assign17470_e17067_d_n3, assign17470_e17067_d_n4, assign17470_e17067_d_n7, assign17470_e17067_d_n10, assign17470_e17067_d_n11,) = {
    if ((locals.var_guard204 != 0.0) && (locals.var_guard211 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__ffs, locals.var_fn205_calc_iq__ffs_dn2, locals.var_fn205_calc_iq__ffs_dn3, locals.var_fn205_calc_iq__ffs_dn4, locals.var_fn205_calc_iq__ffs_dn7, locals.var_fn205_calc_iq__ffs_dn10, locals.var_fn205_calc_iq__ffs_dn11,)
    }
};
        locals.var_fn205_calc_iq__ffs = assign17470_e17067;
        locals.var_fn205_calc_iq__ffs_dn2 = assign17470_e17067_d_n2;
        locals.var_fn205_calc_iq__ffs_dn3 = assign17470_e17067_d_n3;
        locals.var_fn205_calc_iq__ffs_dn4 = assign17470_e17067_d_n4;
        locals.var_fn205_calc_iq__ffs_dn7 = assign17470_e17067_d_n7;
        locals.var_fn205_calc_iq__ffs_dn10 = assign17470_e17067_d_n10;
        locals.var_fn205_calc_iq__ffs_dn11 = assign17470_e17067_d_n11;

        let assign17480_e17070: f64 = (-50.0);
        let assign17480_e17071: f64 = if locals.var_fn205_calc_iq__exparg < assign17480_e17070 { 1.0 } else { 0.0 };
        locals.var_guard212 = assign17480_e17071;

        let (assign17490_e17080, assign17490_e17080_d_n2, assign17490_e17080_d_n3, assign17490_e17080_d_n4, assign17490_e17080_d_n7, assign17490_e17080_d_n10, assign17490_e17080_d_n11,) = {
    if (((locals.var_guard204 != 0.0) && (locals.var_guard211 == 0.0)) && (locals.var_guard212 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__ffs, locals.var_fn205_calc_iq__ffs_dn2, locals.var_fn205_calc_iq__ffs_dn3, locals.var_fn205_calc_iq__ffs_dn4, locals.var_fn205_calc_iq__ffs_dn7, locals.var_fn205_calc_iq__ffs_dn10, locals.var_fn205_calc_iq__ffs_dn11,)
    }
};
        locals.var_fn205_calc_iq__ffs = assign17490_e17080;
        locals.var_fn205_calc_iq__ffs_dn2 = assign17490_e17080_d_n2;
        locals.var_fn205_calc_iq__ffs_dn3 = assign17490_e17080_d_n3;
        locals.var_fn205_calc_iq__ffs_dn4 = assign17490_e17080_d_n4;
        locals.var_fn205_calc_iq__ffs_dn7 = assign17490_e17080_d_n7;
        locals.var_fn205_calc_iq__ffs_dn10 = assign17490_e17080_d_n10;
        locals.var_fn205_calc_iq__ffs_dn11 = assign17490_e17080_d_n11;

        let (assign17500_e17095, assign17500_e17095_d_n2, assign17500_e17095_d_n3, assign17500_e17095_d_n4, assign17500_e17095_d_n7, assign17500_e17095_d_n10, assign17500_e17095_d_n11,) = {
    if (((locals.var_guard204 != 0.0) && (locals.var_guard211 == 0.0)) && (locals.var_guard212 == 0.0)) {
        let assign17500_e17091: f64 = (locals.var_fn205_calc_iq__exparg).exp();
        let assign17500_e17092: f64 = (1.0 + assign17500_e17091);
        let assign17500_e17093: f64 = (1.0 / assign17500_e17092);
        (assign17500_e17093, (-((assign17500_e17091 * locals.var_fn205_calc_iq__exparg_dn2) / (assign17500_e17092 * assign17500_e17092))), (-((assign17500_e17091 * locals.var_fn205_calc_iq__exparg_dn3) / (assign17500_e17092 * assign17500_e17092))), (-((assign17500_e17091 * locals.var_fn205_calc_iq__exparg_dn4) / (assign17500_e17092 * assign17500_e17092))), (-((assign17500_e17091 * locals.var_fn205_calc_iq__exparg_dn7) / (assign17500_e17092 * assign17500_e17092))), (-((assign17500_e17091 * locals.var_fn205_calc_iq__exparg_dn10) / (assign17500_e17092 * assign17500_e17092))), (-((assign17500_e17091 * locals.var_fn205_calc_iq__exparg_dn11) / (assign17500_e17092 * assign17500_e17092))),)
    } else {
        (locals.var_fn205_calc_iq__ffs, locals.var_fn205_calc_iq__ffs_dn2, locals.var_fn205_calc_iq__ffs_dn3, locals.var_fn205_calc_iq__ffs_dn4, locals.var_fn205_calc_iq__ffs_dn7, locals.var_fn205_calc_iq__ffs_dn10, locals.var_fn205_calc_iq__ffs_dn11,)
    }
};
        locals.var_fn205_calc_iq__ffs = assign17500_e17095;
        locals.var_fn205_calc_iq__ffs_dn2 = assign17500_e17095_d_n2;
        locals.var_fn205_calc_iq__ffs_dn3 = assign17500_e17095_d_n3;
        locals.var_fn205_calc_iq__ffs_dn4 = assign17500_e17095_d_n4;
        locals.var_fn205_calc_iq__ffs_dn7 = assign17500_e17095_d_n7;
        locals.var_fn205_calc_iq__ffs_dn10 = assign17500_e17095_d_n10;
        locals.var_fn205_calc_iq__ffs_dn11 = assign17500_e17095_d_n11;

        let (assign17510_e17113, assign17510_e17113_d_n2, assign17510_e17113_d_n3, assign17510_e17113_d_n4, assign17510_e17113_d_n7, assign17510_e17113_d_n10, assign17510_e17113_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        let assign17510_e17099: f64 = (locals.var_fn205_calc_iq__vgdin - locals.var_fn205_calc_iq__vsx);
        let assign17510_e17103: f64 = (p.p51 * 0.1);
        let assign17510_e17105: f64 = (assign17510_e17103 * locals.var_fn205_calc_iq__alpha_phit);
        let assign17510_e17107: f64 = (assign17510_e17105 * locals.var_fn205_calc_iq__ffs);
        let assign17510_e17108: f64 = (locals.var_fn205_calc_iq__vtdibl - assign17510_e17107);
        let assign17510_e17109: f64 = (assign17510_e17099 - assign17510_e17108);
        let assign17510_e17111: f64 = (assign17510_e17109 / locals.var_fn205_calc_iq__two_n_phit);
        (assign17510_e17111, (((locals.var_fn205_calc_iq__vgdin_dn2 - locals.var_fn205_calc_iq__vsx_dn2) - (-(assign17510_e17105 * locals.var_fn205_calc_iq__ffs_dn2))) / locals.var_fn205_calc_iq__two_n_phit), (((-locals.var_fn205_calc_iq__vsx_dn3) - (-(assign17510_e17105 * locals.var_fn205_calc_iq__ffs_dn3))) / locals.var_fn205_calc_iq__two_n_phit), (((((-locals.var_fn205_calc_iq__vsx_dn4) - (locals.var_fn205_calc_iq__vtdibl_dn4 - (((assign17510_e17103 * locals.var_fn205_calc_iq__alpha_phit_dn4) * locals.var_fn205_calc_iq__ffs) + (assign17510_e17105 * locals.var_fn205_calc_iq__ffs_dn4)))) * locals.var_fn205_calc_iq__two_n_phit) - (assign17510_e17109 * locals.var_fn205_calc_iq__two_n_phit_dn4)) / (locals.var_fn205_calc_iq__two_n_phit * locals.var_fn205_calc_iq__two_n_phit)), (((locals.var_fn205_calc_iq__vgdin_dn7 - locals.var_fn205_calc_iq__vsx_dn7) - (-(assign17510_e17105 * locals.var_fn205_calc_iq__ffs_dn7))) / locals.var_fn205_calc_iq__two_n_phit), (((((locals.var_fn205_calc_iq__vgdin_dn10 - locals.var_fn205_calc_iq__vsx_dn10) - (locals.var_fn205_calc_iq__vtdibl_dn10 - (assign17510_e17105 * locals.var_fn205_calc_iq__ffs_dn10))) * locals.var_fn205_calc_iq__two_n_phit) - (assign17510_e17109 * locals.var_fn205_calc_iq__two_n_phit_dn10)) / (locals.var_fn205_calc_iq__two_n_phit * locals.var_fn205_calc_iq__two_n_phit)), (((((locals.var_fn205_calc_iq__vgdin_dn11 - locals.var_fn205_calc_iq__vsx_dn11) - (locals.var_fn205_calc_iq__vtdibl_dn11 - (assign17510_e17105 * locals.var_fn205_calc_iq__ffs_dn11))) * locals.var_fn205_calc_iq__two_n_phit) - (assign17510_e17109 * locals.var_fn205_calc_iq__two_n_phit_dn11)) / (locals.var_fn205_calc_iq__two_n_phit * locals.var_fn205_calc_iq__two_n_phit)),)
    } else {
        (locals.var_fn205_calc_iq__etas, locals.var_fn205_calc_iq__etas_dn2, locals.var_fn205_calc_iq__etas_dn3, locals.var_fn205_calc_iq__etas_dn4, locals.var_fn205_calc_iq__etas_dn7, locals.var_fn205_calc_iq__etas_dn10, locals.var_fn205_calc_iq__etas_dn11,)
    }
};
        locals.var_fn205_calc_iq__etas = assign17510_e17113;
        locals.var_fn205_calc_iq__etas_dn2 = assign17510_e17113_d_n2;
        locals.var_fn205_calc_iq__etas_dn3 = assign17510_e17113_d_n3;
        locals.var_fn205_calc_iq__etas_dn4 = assign17510_e17113_d_n4;
        locals.var_fn205_calc_iq__etas_dn7 = assign17510_e17113_d_n7;
        locals.var_fn205_calc_iq__etas_dn10 = assign17510_e17113_d_n10;
        locals.var_fn205_calc_iq__etas_dn11 = assign17510_e17113_d_n11;

        let assign17520_e17116: f64 = if locals.var_fn205_calc_iq__etas > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard213 = assign17520_e17116;

        let (assign17530_e17124, assign17530_e17124_d_n2, assign17530_e17124_d_n3, assign17530_e17124_d_n4, assign17530_e17124_d_n7, assign17530_e17124_d_n10, assign17530_e17124_d_n11,) = {
    if ((locals.var_guard204 != 0.0) && (locals.var_guard213 != 0.0)) {
        let assign17530_e17122: f64 = (locals.var_fn205_calc_iq__qref * locals.var_fn205_calc_iq__etas);
        (assign17530_e17122, (locals.var_fn205_calc_iq__qref * locals.var_fn205_calc_iq__etas_dn2), (locals.var_fn205_calc_iq__qref * locals.var_fn205_calc_iq__etas_dn3), ((locals.var_fn205_calc_iq__qref_dn4 * locals.var_fn205_calc_iq__etas) + (locals.var_fn205_calc_iq__qref * locals.var_fn205_calc_iq__etas_dn4)), (locals.var_fn205_calc_iq__qref * locals.var_fn205_calc_iq__etas_dn7), ((locals.var_fn205_calc_iq__qref_dn10 * locals.var_fn205_calc_iq__etas) + (locals.var_fn205_calc_iq__qref * locals.var_fn205_calc_iq__etas_dn10)), ((locals.var_fn205_calc_iq__qref_dn11 * locals.var_fn205_calc_iq__etas) + (locals.var_fn205_calc_iq__qref * locals.var_fn205_calc_iq__etas_dn11)),)
    } else {
        (locals.var_fn205_calc_iq__qinvs, locals.var_fn205_calc_iq__qinvs_dn2, locals.var_fn205_calc_iq__qinvs_dn3, locals.var_fn205_calc_iq__qinvs_dn4, locals.var_fn205_calc_iq__qinvs_dn7, locals.var_fn205_calc_iq__qinvs_dn10, locals.var_fn205_calc_iq__qinvs_dn11,)
    }
};
        locals.var_fn205_calc_iq__qinvs = assign17530_e17124;
        locals.var_fn205_calc_iq__qinvs_dn2 = assign17530_e17124_d_n2;
        locals.var_fn205_calc_iq__qinvs_dn3 = assign17530_e17124_d_n3;
        locals.var_fn205_calc_iq__qinvs_dn4 = assign17530_e17124_d_n4;
        locals.var_fn205_calc_iq__qinvs_dn7 = assign17530_e17124_d_n7;
        locals.var_fn205_calc_iq__qinvs_dn10 = assign17530_e17124_d_n10;
        locals.var_fn205_calc_iq__qinvs_dn11 = assign17530_e17124_d_n11;

        let assign17540_e17127: f64 = (-50.0);
        let assign17540_e17128: f64 = if locals.var_fn205_calc_iq__etas < assign17540_e17127 { 1.0 } else { 0.0 };
        locals.var_guard214 = assign17540_e17128;

    }

    pub(super) fn stamp_transient_block_44(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign17550_e17140, assign17550_e17140_d_n2, assign17550_e17140_d_n3, assign17550_e17140_d_n4, assign17550_e17140_d_n7, assign17550_e17140_d_n10, assign17550_e17140_d_n11,) = {
    if (((locals.var_guard204 != 0.0) && (locals.var_guard213 == 0.0)) && (locals.var_guard214 != 0.0)) {
        let assign17550_e17137: f64 = (locals.var_fn205_calc_iq__etas).exp();
        let assign17550_e17138: f64 = (locals.var_fn205_calc_iq__qref * assign17550_e17137);
        (assign17550_e17138, (locals.var_fn205_calc_iq__qref * (assign17550_e17137 * locals.var_fn205_calc_iq__etas_dn2)), (locals.var_fn205_calc_iq__qref * (assign17550_e17137 * locals.var_fn205_calc_iq__etas_dn3)), ((locals.var_fn205_calc_iq__qref_dn4 * assign17550_e17137) + (locals.var_fn205_calc_iq__qref * (assign17550_e17137 * locals.var_fn205_calc_iq__etas_dn4))), (locals.var_fn205_calc_iq__qref * (assign17550_e17137 * locals.var_fn205_calc_iq__etas_dn7)), ((locals.var_fn205_calc_iq__qref_dn10 * assign17550_e17137) + (locals.var_fn205_calc_iq__qref * (assign17550_e17137 * locals.var_fn205_calc_iq__etas_dn10))), ((locals.var_fn205_calc_iq__qref_dn11 * assign17550_e17137) + (locals.var_fn205_calc_iq__qref * (assign17550_e17137 * locals.var_fn205_calc_iq__etas_dn11))),)
    } else {
        (locals.var_fn205_calc_iq__qinvs, locals.var_fn205_calc_iq__qinvs_dn2, locals.var_fn205_calc_iq__qinvs_dn3, locals.var_fn205_calc_iq__qinvs_dn4, locals.var_fn205_calc_iq__qinvs_dn7, locals.var_fn205_calc_iq__qinvs_dn10, locals.var_fn205_calc_iq__qinvs_dn11,)
    }
};
        locals.var_fn205_calc_iq__qinvs = assign17550_e17140;
        locals.var_fn205_calc_iq__qinvs_dn2 = assign17550_e17140_d_n2;
        locals.var_fn205_calc_iq__qinvs_dn3 = assign17550_e17140_d_n3;
        locals.var_fn205_calc_iq__qinvs_dn4 = assign17550_e17140_d_n4;
        locals.var_fn205_calc_iq__qinvs_dn7 = assign17550_e17140_d_n7;
        locals.var_fn205_calc_iq__qinvs_dn10 = assign17550_e17140_d_n10;
        locals.var_fn205_calc_iq__qinvs_dn11 = assign17550_e17140_d_n11;

        let (assign17560_e17156, assign17560_e17156_d_n2, assign17560_e17156_d_n3, assign17560_e17156_d_n4, assign17560_e17156_d_n7, assign17560_e17156_d_n10, assign17560_e17156_d_n11,) = {
    if (((locals.var_guard204 != 0.0) && (locals.var_guard213 == 0.0)) && (locals.var_guard214 == 0.0)) {
        let assign17560_e17151: f64 = (locals.var_fn205_calc_iq__etas).exp();
        let assign17560_e17152: f64 = (1.0 + assign17560_e17151);
        let assign17560_e17153: f64 = (assign17560_e17152).ln();
        let assign17560_e17154: f64 = (locals.var_fn205_calc_iq__qref * assign17560_e17153);
        (assign17560_e17154, (locals.var_fn205_calc_iq__qref * ((assign17560_e17151 * locals.var_fn205_calc_iq__etas_dn2) / assign17560_e17152)), (locals.var_fn205_calc_iq__qref * ((assign17560_e17151 * locals.var_fn205_calc_iq__etas_dn3) / assign17560_e17152)), ((locals.var_fn205_calc_iq__qref_dn4 * assign17560_e17153) + (locals.var_fn205_calc_iq__qref * ((assign17560_e17151 * locals.var_fn205_calc_iq__etas_dn4) / assign17560_e17152))), (locals.var_fn205_calc_iq__qref * ((assign17560_e17151 * locals.var_fn205_calc_iq__etas_dn7) / assign17560_e17152)), ((locals.var_fn205_calc_iq__qref_dn10 * assign17560_e17153) + (locals.var_fn205_calc_iq__qref * ((assign17560_e17151 * locals.var_fn205_calc_iq__etas_dn10) / assign17560_e17152))), ((locals.var_fn205_calc_iq__qref_dn11 * assign17560_e17153) + (locals.var_fn205_calc_iq__qref * ((assign17560_e17151 * locals.var_fn205_calc_iq__etas_dn11) / assign17560_e17152))),)
    } else {
        (locals.var_fn205_calc_iq__qinvs, locals.var_fn205_calc_iq__qinvs_dn2, locals.var_fn205_calc_iq__qinvs_dn3, locals.var_fn205_calc_iq__qinvs_dn4, locals.var_fn205_calc_iq__qinvs_dn7, locals.var_fn205_calc_iq__qinvs_dn10, locals.var_fn205_calc_iq__qinvs_dn11,)
    }
};
        locals.var_fn205_calc_iq__qinvs = assign17560_e17156;
        locals.var_fn205_calc_iq__qinvs_dn2 = assign17560_e17156_d_n2;
        locals.var_fn205_calc_iq__qinvs_dn3 = assign17560_e17156_d_n3;
        locals.var_fn205_calc_iq__qinvs_dn4 = assign17560_e17156_d_n4;
        locals.var_fn205_calc_iq__qinvs_dn7 = assign17560_e17156_d_n7;
        locals.var_fn205_calc_iq__qinvs_dn10 = assign17560_e17156_d_n10;
        locals.var_fn205_calc_iq__qinvs_dn11 = assign17560_e17156_d_n11;

        let (assign17570_e17164, assign17570_e17164_d_n2, assign17570_e17164_d_n3, assign17570_e17164_d_n4, assign17570_e17164_d_n7, assign17570_e17164_d_n10, assign17570_e17164_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        let assign17570_e17160: f64 = (locals.var_fn205_calc_iq__vgdin - locals.var_fn205_calc_iq__myarg);
        let assign17570_e17162: f64 = (assign17570_e17160 / locals.var_fn205_calc_iq__alpha_phit);
        (assign17570_e17162, ((locals.var_fn205_calc_iq__vgdin_dn2 - locals.var_fn205_calc_iq__myarg_dn2) / locals.var_fn205_calc_iq__alpha_phit), ((-locals.var_fn205_calc_iq__myarg_dn3) / locals.var_fn205_calc_iq__alpha_phit), ((((-locals.var_fn205_calc_iq__myarg_dn4) * locals.var_fn205_calc_iq__alpha_phit) - (assign17570_e17160 * locals.var_fn205_calc_iq__alpha_phit_dn4)) / (locals.var_fn205_calc_iq__alpha_phit * locals.var_fn205_calc_iq__alpha_phit)), ((locals.var_fn205_calc_iq__vgdin_dn7 - locals.var_fn205_calc_iq__myarg_dn7) / locals.var_fn205_calc_iq__alpha_phit), ((locals.var_fn205_calc_iq__vgdin_dn10 - locals.var_fn205_calc_iq__myarg_dn10) / locals.var_fn205_calc_iq__alpha_phit), ((locals.var_fn205_calc_iq__vgdin_dn11 - locals.var_fn205_calc_iq__myarg_dn11) / locals.var_fn205_calc_iq__alpha_phit),)
    } else {
        (locals.var_fn205_calc_iq__exparg, locals.var_fn205_calc_iq__exparg_dn2, locals.var_fn205_calc_iq__exparg_dn3, locals.var_fn205_calc_iq__exparg_dn4, locals.var_fn205_calc_iq__exparg_dn7, locals.var_fn205_calc_iq__exparg_dn10, locals.var_fn205_calc_iq__exparg_dn11,)
    }
};
        locals.var_fn205_calc_iq__exparg = assign17570_e17164;
        locals.var_fn205_calc_iq__exparg_dn2 = assign17570_e17164_d_n2;
        locals.var_fn205_calc_iq__exparg_dn3 = assign17570_e17164_d_n3;
        locals.var_fn205_calc_iq__exparg_dn4 = assign17570_e17164_d_n4;
        locals.var_fn205_calc_iq__exparg_dn7 = assign17570_e17164_d_n7;
        locals.var_fn205_calc_iq__exparg_dn10 = assign17570_e17164_d_n10;
        locals.var_fn205_calc_iq__exparg_dn11 = assign17570_e17164_d_n11;

        let assign17580_e17167: f64 = if locals.var_fn205_calc_iq__exparg > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard215 = assign17580_e17167;

        let (assign17590_e17173, assign17590_e17173_d_n2, assign17590_e17173_d_n3, assign17590_e17173_d_n4, assign17590_e17173_d_n7, assign17590_e17173_d_n10, assign17590_e17173_d_n11,) = {
    if ((locals.var_guard204 != 0.0) && (locals.var_guard215 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__ffd, locals.var_fn205_calc_iq__ffd_dn2, locals.var_fn205_calc_iq__ffd_dn3, locals.var_fn205_calc_iq__ffd_dn4, locals.var_fn205_calc_iq__ffd_dn7, locals.var_fn205_calc_iq__ffd_dn10, locals.var_fn205_calc_iq__ffd_dn11,)
    }
};
        locals.var_fn205_calc_iq__ffd = assign17590_e17173;
        locals.var_fn205_calc_iq__ffd_dn2 = assign17590_e17173_d_n2;
        locals.var_fn205_calc_iq__ffd_dn3 = assign17590_e17173_d_n3;
        locals.var_fn205_calc_iq__ffd_dn4 = assign17590_e17173_d_n4;
        locals.var_fn205_calc_iq__ffd_dn7 = assign17590_e17173_d_n7;
        locals.var_fn205_calc_iq__ffd_dn10 = assign17590_e17173_d_n10;
        locals.var_fn205_calc_iq__ffd_dn11 = assign17590_e17173_d_n11;

        let assign17600_e17176: f64 = (-50.0);
        let assign17600_e17177: f64 = if locals.var_fn205_calc_iq__exparg < assign17600_e17176 { 1.0 } else { 0.0 };
        locals.var_guard216 = assign17600_e17177;

        let (assign17610_e17186, assign17610_e17186_d_n2, assign17610_e17186_d_n3, assign17610_e17186_d_n4, assign17610_e17186_d_n7, assign17610_e17186_d_n10, assign17610_e17186_d_n11,) = {
    if (((locals.var_guard204 != 0.0) && (locals.var_guard215 == 0.0)) && (locals.var_guard216 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__ffd, locals.var_fn205_calc_iq__ffd_dn2, locals.var_fn205_calc_iq__ffd_dn3, locals.var_fn205_calc_iq__ffd_dn4, locals.var_fn205_calc_iq__ffd_dn7, locals.var_fn205_calc_iq__ffd_dn10, locals.var_fn205_calc_iq__ffd_dn11,)
    }
};
        locals.var_fn205_calc_iq__ffd = assign17610_e17186;
        locals.var_fn205_calc_iq__ffd_dn2 = assign17610_e17186_d_n2;
        locals.var_fn205_calc_iq__ffd_dn3 = assign17610_e17186_d_n3;
        locals.var_fn205_calc_iq__ffd_dn4 = assign17610_e17186_d_n4;
        locals.var_fn205_calc_iq__ffd_dn7 = assign17610_e17186_d_n7;
        locals.var_fn205_calc_iq__ffd_dn10 = assign17610_e17186_d_n10;
        locals.var_fn205_calc_iq__ffd_dn11 = assign17610_e17186_d_n11;

        let (assign17620_e17201, assign17620_e17201_d_n2, assign17620_e17201_d_n3, assign17620_e17201_d_n4, assign17620_e17201_d_n7, assign17620_e17201_d_n10, assign17620_e17201_d_n11,) = {
    if (((locals.var_guard204 != 0.0) && (locals.var_guard215 == 0.0)) && (locals.var_guard216 == 0.0)) {
        let assign17620_e17197: f64 = (locals.var_fn205_calc_iq__exparg).exp();
        let assign17620_e17198: f64 = (1.0 + assign17620_e17197);
        let assign17620_e17199: f64 = (1.0 / assign17620_e17198);
        (assign17620_e17199, (-((assign17620_e17197 * locals.var_fn205_calc_iq__exparg_dn2) / (assign17620_e17198 * assign17620_e17198))), (-((assign17620_e17197 * locals.var_fn205_calc_iq__exparg_dn3) / (assign17620_e17198 * assign17620_e17198))), (-((assign17620_e17197 * locals.var_fn205_calc_iq__exparg_dn4) / (assign17620_e17198 * assign17620_e17198))), (-((assign17620_e17197 * locals.var_fn205_calc_iq__exparg_dn7) / (assign17620_e17198 * assign17620_e17198))), (-((assign17620_e17197 * locals.var_fn205_calc_iq__exparg_dn10) / (assign17620_e17198 * assign17620_e17198))), (-((assign17620_e17197 * locals.var_fn205_calc_iq__exparg_dn11) / (assign17620_e17198 * assign17620_e17198))),)
    } else {
        (locals.var_fn205_calc_iq__ffd, locals.var_fn205_calc_iq__ffd_dn2, locals.var_fn205_calc_iq__ffd_dn3, locals.var_fn205_calc_iq__ffd_dn4, locals.var_fn205_calc_iq__ffd_dn7, locals.var_fn205_calc_iq__ffd_dn10, locals.var_fn205_calc_iq__ffd_dn11,)
    }
};
        locals.var_fn205_calc_iq__ffd = assign17620_e17201;
        locals.var_fn205_calc_iq__ffd_dn2 = assign17620_e17201_d_n2;
        locals.var_fn205_calc_iq__ffd_dn3 = assign17620_e17201_d_n3;
        locals.var_fn205_calc_iq__ffd_dn4 = assign17620_e17201_d_n4;
        locals.var_fn205_calc_iq__ffd_dn7 = assign17620_e17201_d_n7;
        locals.var_fn205_calc_iq__ffd_dn10 = assign17620_e17201_d_n10;
        locals.var_fn205_calc_iq__ffd_dn11 = assign17620_e17201_d_n11;

        let (assign17630_e17219, assign17630_e17219_d_n2, assign17630_e17219_d_n3, assign17630_e17219_d_n4, assign17630_e17219_d_n7, assign17630_e17219_d_n10, assign17630_e17219_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        let assign17630_e17205: f64 = (locals.var_fn205_calc_iq__vgsin - locals.var_fn205_calc_iq__vdx);
        let assign17630_e17209: f64 = (p.p51 * 0.1);
        let assign17630_e17211: f64 = (assign17630_e17209 * locals.var_fn205_calc_iq__alpha_phit);
        let assign17630_e17213: f64 = (assign17630_e17211 * locals.var_fn205_calc_iq__ffd);
        let assign17630_e17214: f64 = (locals.var_fn205_calc_iq__vtdibl - assign17630_e17213);
        let assign17630_e17215: f64 = (assign17630_e17205 - assign17630_e17214);
        let assign17630_e17217: f64 = (assign17630_e17215 / locals.var_fn205_calc_iq__two_n_phit);
        (assign17630_e17217, (((locals.var_fn205_calc_iq__vgsin_dn2 - locals.var_fn205_calc_iq__vdx_dn2) - (-(assign17630_e17211 * locals.var_fn205_calc_iq__ffd_dn2))) / locals.var_fn205_calc_iq__two_n_phit), (((-locals.var_fn205_calc_iq__vdx_dn3) - (-(assign17630_e17211 * locals.var_fn205_calc_iq__ffd_dn3))) / locals.var_fn205_calc_iq__two_n_phit), (((((-locals.var_fn205_calc_iq__vdx_dn4) - (locals.var_fn205_calc_iq__vtdibl_dn4 - (((assign17630_e17209 * locals.var_fn205_calc_iq__alpha_phit_dn4) * locals.var_fn205_calc_iq__ffd) + (assign17630_e17211 * locals.var_fn205_calc_iq__ffd_dn4)))) * locals.var_fn205_calc_iq__two_n_phit) - (assign17630_e17215 * locals.var_fn205_calc_iq__two_n_phit_dn4)) / (locals.var_fn205_calc_iq__two_n_phit * locals.var_fn205_calc_iq__two_n_phit)), (((locals.var_fn205_calc_iq__vgsin_dn7 - locals.var_fn205_calc_iq__vdx_dn7) - (-(assign17630_e17211 * locals.var_fn205_calc_iq__ffd_dn7))) / locals.var_fn205_calc_iq__two_n_phit), (((((-locals.var_fn205_calc_iq__vdx_dn10) - (locals.var_fn205_calc_iq__vtdibl_dn10 - (assign17630_e17211 * locals.var_fn205_calc_iq__ffd_dn10))) * locals.var_fn205_calc_iq__two_n_phit) - (assign17630_e17215 * locals.var_fn205_calc_iq__two_n_phit_dn10)) / (locals.var_fn205_calc_iq__two_n_phit * locals.var_fn205_calc_iq__two_n_phit)), (((((locals.var_fn205_calc_iq__vgsin_dn11 - locals.var_fn205_calc_iq__vdx_dn11) - (locals.var_fn205_calc_iq__vtdibl_dn11 - (assign17630_e17211 * locals.var_fn205_calc_iq__ffd_dn11))) * locals.var_fn205_calc_iq__two_n_phit) - (assign17630_e17215 * locals.var_fn205_calc_iq__two_n_phit_dn11)) / (locals.var_fn205_calc_iq__two_n_phit * locals.var_fn205_calc_iq__two_n_phit)),)
    } else {
        (locals.var_fn205_calc_iq__etad, locals.var_fn205_calc_iq__etad_dn2, locals.var_fn205_calc_iq__etad_dn3, locals.var_fn205_calc_iq__etad_dn4, locals.var_fn205_calc_iq__etad_dn7, locals.var_fn205_calc_iq__etad_dn10, locals.var_fn205_calc_iq__etad_dn11,)
    }
};
        locals.var_fn205_calc_iq__etad = assign17630_e17219;
        locals.var_fn205_calc_iq__etad_dn2 = assign17630_e17219_d_n2;
        locals.var_fn205_calc_iq__etad_dn3 = assign17630_e17219_d_n3;
        locals.var_fn205_calc_iq__etad_dn4 = assign17630_e17219_d_n4;
        locals.var_fn205_calc_iq__etad_dn7 = assign17630_e17219_d_n7;
        locals.var_fn205_calc_iq__etad_dn10 = assign17630_e17219_d_n10;
        locals.var_fn205_calc_iq__etad_dn11 = assign17630_e17219_d_n11;

        let assign17640_e17222: f64 = if locals.var_fn205_calc_iq__etad > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard217 = assign17640_e17222;

        let (assign17650_e17230, assign17650_e17230_d_n2, assign17650_e17230_d_n3, assign17650_e17230_d_n4, assign17650_e17230_d_n7, assign17650_e17230_d_n10, assign17650_e17230_d_n11,) = {
    if ((locals.var_guard204 != 0.0) && (locals.var_guard217 != 0.0)) {
        let assign17650_e17228: f64 = (locals.var_fn205_calc_iq__qref * locals.var_fn205_calc_iq__etad);
        (assign17650_e17228, (locals.var_fn205_calc_iq__qref * locals.var_fn205_calc_iq__etad_dn2), (locals.var_fn205_calc_iq__qref * locals.var_fn205_calc_iq__etad_dn3), ((locals.var_fn205_calc_iq__qref_dn4 * locals.var_fn205_calc_iq__etad) + (locals.var_fn205_calc_iq__qref * locals.var_fn205_calc_iq__etad_dn4)), (locals.var_fn205_calc_iq__qref * locals.var_fn205_calc_iq__etad_dn7), ((locals.var_fn205_calc_iq__qref_dn10 * locals.var_fn205_calc_iq__etad) + (locals.var_fn205_calc_iq__qref * locals.var_fn205_calc_iq__etad_dn10)), ((locals.var_fn205_calc_iq__qref_dn11 * locals.var_fn205_calc_iq__etad) + (locals.var_fn205_calc_iq__qref * locals.var_fn205_calc_iq__etad_dn11)),)
    } else {
        (locals.var_fn205_calc_iq__qinvd, locals.var_fn205_calc_iq__qinvd_dn2, locals.var_fn205_calc_iq__qinvd_dn3, locals.var_fn205_calc_iq__qinvd_dn4, locals.var_fn205_calc_iq__qinvd_dn7, locals.var_fn205_calc_iq__qinvd_dn10, locals.var_fn205_calc_iq__qinvd_dn11,)
    }
};
        locals.var_fn205_calc_iq__qinvd = assign17650_e17230;
        locals.var_fn205_calc_iq__qinvd_dn2 = assign17650_e17230_d_n2;
        locals.var_fn205_calc_iq__qinvd_dn3 = assign17650_e17230_d_n3;
        locals.var_fn205_calc_iq__qinvd_dn4 = assign17650_e17230_d_n4;
        locals.var_fn205_calc_iq__qinvd_dn7 = assign17650_e17230_d_n7;
        locals.var_fn205_calc_iq__qinvd_dn10 = assign17650_e17230_d_n10;
        locals.var_fn205_calc_iq__qinvd_dn11 = assign17650_e17230_d_n11;

        let assign17660_e17233: f64 = (-50.0);
        let assign17660_e17234: f64 = if locals.var_fn205_calc_iq__etad < assign17660_e17233 { 1.0 } else { 0.0 };
        locals.var_guard218 = assign17660_e17234;

        let (assign17670_e17246, assign17670_e17246_d_n2, assign17670_e17246_d_n3, assign17670_e17246_d_n4, assign17670_e17246_d_n7, assign17670_e17246_d_n10, assign17670_e17246_d_n11,) = {
    if (((locals.var_guard204 != 0.0) && (locals.var_guard217 == 0.0)) && (locals.var_guard218 != 0.0)) {
        let assign17670_e17243: f64 = (locals.var_fn205_calc_iq__etad).exp();
        let assign17670_e17244: f64 = (locals.var_fn205_calc_iq__qref * assign17670_e17243);
        (assign17670_e17244, (locals.var_fn205_calc_iq__qref * (assign17670_e17243 * locals.var_fn205_calc_iq__etad_dn2)), (locals.var_fn205_calc_iq__qref * (assign17670_e17243 * locals.var_fn205_calc_iq__etad_dn3)), ((locals.var_fn205_calc_iq__qref_dn4 * assign17670_e17243) + (locals.var_fn205_calc_iq__qref * (assign17670_e17243 * locals.var_fn205_calc_iq__etad_dn4))), (locals.var_fn205_calc_iq__qref * (assign17670_e17243 * locals.var_fn205_calc_iq__etad_dn7)), ((locals.var_fn205_calc_iq__qref_dn10 * assign17670_e17243) + (locals.var_fn205_calc_iq__qref * (assign17670_e17243 * locals.var_fn205_calc_iq__etad_dn10))), ((locals.var_fn205_calc_iq__qref_dn11 * assign17670_e17243) + (locals.var_fn205_calc_iq__qref * (assign17670_e17243 * locals.var_fn205_calc_iq__etad_dn11))),)
    } else {
        (locals.var_fn205_calc_iq__qinvd, locals.var_fn205_calc_iq__qinvd_dn2, locals.var_fn205_calc_iq__qinvd_dn3, locals.var_fn205_calc_iq__qinvd_dn4, locals.var_fn205_calc_iq__qinvd_dn7, locals.var_fn205_calc_iq__qinvd_dn10, locals.var_fn205_calc_iq__qinvd_dn11,)
    }
};
        locals.var_fn205_calc_iq__qinvd = assign17670_e17246;
        locals.var_fn205_calc_iq__qinvd_dn2 = assign17670_e17246_d_n2;
        locals.var_fn205_calc_iq__qinvd_dn3 = assign17670_e17246_d_n3;
        locals.var_fn205_calc_iq__qinvd_dn4 = assign17670_e17246_d_n4;
        locals.var_fn205_calc_iq__qinvd_dn7 = assign17670_e17246_d_n7;
        locals.var_fn205_calc_iq__qinvd_dn10 = assign17670_e17246_d_n10;
        locals.var_fn205_calc_iq__qinvd_dn11 = assign17670_e17246_d_n11;

        let (assign17680_e17262, assign17680_e17262_d_n2, assign17680_e17262_d_n3, assign17680_e17262_d_n4, assign17680_e17262_d_n7, assign17680_e17262_d_n10, assign17680_e17262_d_n11,) = {
    if (((locals.var_guard204 != 0.0) && (locals.var_guard217 == 0.0)) && (locals.var_guard218 == 0.0)) {
        let assign17680_e17257: f64 = (locals.var_fn205_calc_iq__etad).exp();
        let assign17680_e17258: f64 = (1.0 + assign17680_e17257);
        let assign17680_e17259: f64 = (assign17680_e17258).ln();
        let assign17680_e17260: f64 = (locals.var_fn205_calc_iq__qref * assign17680_e17259);
        (assign17680_e17260, (locals.var_fn205_calc_iq__qref * ((assign17680_e17257 * locals.var_fn205_calc_iq__etad_dn2) / assign17680_e17258)), (locals.var_fn205_calc_iq__qref * ((assign17680_e17257 * locals.var_fn205_calc_iq__etad_dn3) / assign17680_e17258)), ((locals.var_fn205_calc_iq__qref_dn4 * assign17680_e17259) + (locals.var_fn205_calc_iq__qref * ((assign17680_e17257 * locals.var_fn205_calc_iq__etad_dn4) / assign17680_e17258))), (locals.var_fn205_calc_iq__qref * ((assign17680_e17257 * locals.var_fn205_calc_iq__etad_dn7) / assign17680_e17258)), ((locals.var_fn205_calc_iq__qref_dn10 * assign17680_e17259) + (locals.var_fn205_calc_iq__qref * ((assign17680_e17257 * locals.var_fn205_calc_iq__etad_dn10) / assign17680_e17258))), ((locals.var_fn205_calc_iq__qref_dn11 * assign17680_e17259) + (locals.var_fn205_calc_iq__qref * ((assign17680_e17257 * locals.var_fn205_calc_iq__etad_dn11) / assign17680_e17258))),)
    } else {
        (locals.var_fn205_calc_iq__qinvd, locals.var_fn205_calc_iq__qinvd_dn2, locals.var_fn205_calc_iq__qinvd_dn3, locals.var_fn205_calc_iq__qinvd_dn4, locals.var_fn205_calc_iq__qinvd_dn7, locals.var_fn205_calc_iq__qinvd_dn10, locals.var_fn205_calc_iq__qinvd_dn11,)
    }
};
        locals.var_fn205_calc_iq__qinvd = assign17680_e17262;
        locals.var_fn205_calc_iq__qinvd_dn2 = assign17680_e17262_d_n2;
        locals.var_fn205_calc_iq__qinvd_dn3 = assign17680_e17262_d_n3;
        locals.var_fn205_calc_iq__qinvd_dn4 = assign17680_e17262_d_n4;
        locals.var_fn205_calc_iq__qinvd_dn7 = assign17680_e17262_d_n7;
        locals.var_fn205_calc_iq__qinvd_dn10 = assign17680_e17262_d_n10;
        locals.var_fn205_calc_iq__qinvd_dn11 = assign17680_e17262_d_n11;

        let (assign17690_e17270, assign17690_e17270_d_n2, assign17690_e17270_d_n3, assign17690_e17270_d_n4, assign17690_e17270_d_n7, assign17690_e17270_d_n10, assign17690_e17270_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        let assign17690_e17266: f64 = (locals.var_fn205_calc_iq__qinvs - locals.var_fn205_calc_iq__qinvd);
        let assign17690_e17268: f64 = (assign17690_e17266 / locals.var_fn205_calc_iq__cgin);
        (assign17690_e17268, ((locals.var_fn205_calc_iq__qinvs_dn2 - locals.var_fn205_calc_iq__qinvd_dn2) / locals.var_fn205_calc_iq__cgin), ((locals.var_fn205_calc_iq__qinvs_dn3 - locals.var_fn205_calc_iq__qinvd_dn3) / locals.var_fn205_calc_iq__cgin), ((((locals.var_fn205_calc_iq__qinvs_dn4 - locals.var_fn205_calc_iq__qinvd_dn4) * locals.var_fn205_calc_iq__cgin) - (assign17690_e17266 * locals.var_fn205_calc_iq__cgin_dn4)) / (locals.var_fn205_calc_iq__cgin * locals.var_fn205_calc_iq__cgin)), ((locals.var_fn205_calc_iq__qinvs_dn7 - locals.var_fn205_calc_iq__qinvd_dn7) / locals.var_fn205_calc_iq__cgin), ((locals.var_fn205_calc_iq__qinvs_dn10 - locals.var_fn205_calc_iq__qinvd_dn10) / locals.var_fn205_calc_iq__cgin), ((locals.var_fn205_calc_iq__qinvs_dn11 - locals.var_fn205_calc_iq__qinvd_dn11) / locals.var_fn205_calc_iq__cgin),)
    } else {
        (locals.var_fn205_calc_iq__vdsc, locals.var_fn205_calc_iq__vdsc_dn2, locals.var_fn205_calc_iq__vdsc_dn3, locals.var_fn205_calc_iq__vdsc_dn4, locals.var_fn205_calc_iq__vdsc_dn7, locals.var_fn205_calc_iq__vdsc_dn10, locals.var_fn205_calc_iq__vdsc_dn11,)
    }
};
        locals.var_fn205_calc_iq__vdsc = assign17690_e17270;
        locals.var_fn205_calc_iq__vdsc_dn2 = assign17690_e17270_d_n2;
        locals.var_fn205_calc_iq__vdsc_dn3 = assign17690_e17270_d_n3;
        locals.var_fn205_calc_iq__vdsc_dn4 = assign17690_e17270_d_n4;
        locals.var_fn205_calc_iq__vdsc_dn7 = assign17690_e17270_d_n7;
        locals.var_fn205_calc_iq__vdsc_dn10 = assign17690_e17270_d_n10;
        locals.var_fn205_calc_iq__vdsc_dn11 = assign17690_e17270_d_n11;

        let (assign17700_e17276, assign17700_e17276_d_n2, assign17700_e17276_d_n3, assign17700_e17276_d_n4, assign17700_e17276_d_n7, assign17700_e17276_d_n10, assign17700_e17276_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        let assign17700_e17274: f64 = (locals.var_fn205_calc_iq__vdsc / locals.var_fn205_calc_iq__vdsat);
        (assign17700_e17274, (((locals.var_fn205_calc_iq__vdsc_dn2 * locals.var_fn205_calc_iq__vdsat) - (locals.var_fn205_calc_iq__vdsc * locals.var_fn205_calc_iq__vdsat_dn2)) / (locals.var_fn205_calc_iq__vdsat * locals.var_fn205_calc_iq__vdsat)), (((locals.var_fn205_calc_iq__vdsc_dn3 * locals.var_fn205_calc_iq__vdsat) - (locals.var_fn205_calc_iq__vdsc * locals.var_fn205_calc_iq__vdsat_dn3)) / (locals.var_fn205_calc_iq__vdsat * locals.var_fn205_calc_iq__vdsat)), (((locals.var_fn205_calc_iq__vdsc_dn4 * locals.var_fn205_calc_iq__vdsat) - (locals.var_fn205_calc_iq__vdsc * locals.var_fn205_calc_iq__vdsat_dn4)) / (locals.var_fn205_calc_iq__vdsat * locals.var_fn205_calc_iq__vdsat)), (((locals.var_fn205_calc_iq__vdsc_dn7 * locals.var_fn205_calc_iq__vdsat) - (locals.var_fn205_calc_iq__vdsc * locals.var_fn205_calc_iq__vdsat_dn7)) / (locals.var_fn205_calc_iq__vdsat * locals.var_fn205_calc_iq__vdsat)), (((locals.var_fn205_calc_iq__vdsc_dn10 * locals.var_fn205_calc_iq__vdsat) - (locals.var_fn205_calc_iq__vdsc * locals.var_fn205_calc_iq__vdsat_dn10)) / (locals.var_fn205_calc_iq__vdsat * locals.var_fn205_calc_iq__vdsat)), (((locals.var_fn205_calc_iq__vdsc_dn11 * locals.var_fn205_calc_iq__vdsat) - (locals.var_fn205_calc_iq__vdsc * locals.var_fn205_calc_iq__vdsat_dn11)) / (locals.var_fn205_calc_iq__vdsat * locals.var_fn205_calc_iq__vdsat)),)
    } else {
        (locals.var_fn205_calc_iq__myarg, locals.var_fn205_calc_iq__myarg_dn2, locals.var_fn205_calc_iq__myarg_dn3, locals.var_fn205_calc_iq__myarg_dn4, locals.var_fn205_calc_iq__myarg_dn7, locals.var_fn205_calc_iq__myarg_dn10, locals.var_fn205_calc_iq__myarg_dn11,)
    }
};
        locals.var_fn205_calc_iq__myarg = assign17700_e17276;
        locals.var_fn205_calc_iq__myarg_dn2 = assign17700_e17276_d_n2;
        locals.var_fn205_calc_iq__myarg_dn3 = assign17700_e17276_d_n3;
        locals.var_fn205_calc_iq__myarg_dn4 = assign17700_e17276_d_n4;
        locals.var_fn205_calc_iq__myarg_dn7 = assign17700_e17276_d_n7;
        locals.var_fn205_calc_iq__myarg_dn10 = assign17700_e17276_d_n10;
        locals.var_fn205_calc_iq__myarg_dn11 = assign17700_e17276_d_n11;

        let (assign17740_e17345, assign17740_e17345_d_n4,) = {
    if (locals.var_guard204 != 0.0) {
        let assign17740_e17342: f64 = (2.302585092994046 * locals.var_fn205_calc_iq__phitin);
        let assign17740_e17343: f64 = (locals.var_fn205_calc_iq__ss / assign17740_e17342);
        (assign17740_e17343, (-((locals.var_fn205_calc_iq__ss * (2.302585092994046 * locals.var_fn205_calc_iq__phitin_dn4)) / (assign17740_e17342 * assign17740_e17342))),)
    } else {
        (locals.var_fn205_calc_iq__n0, locals.var_fn205_calc_iq__n0_dn4,)
    }
};
        locals.var_fn205_calc_iq__n0 = assign17740_e17345;
        locals.var_fn205_calc_iq__n0_dn4 = assign17740_e17345_d_n4;

        let (assign17750_e17353, assign17750_e17353_d_n4,) = {
    if (locals.var_guard204 != 0.0) {
        let assign17750_e17349: f64 = (2.0 * locals.var_fn205_calc_iq__n0);
        let assign17750_e17351: f64 = (assign17750_e17349 * locals.var_fn205_calc_iq__phitin);
        (assign17750_e17351, (((2.0 * locals.var_fn205_calc_iq__n0_dn4) * locals.var_fn205_calc_iq__phitin) + (assign17750_e17349 * locals.var_fn205_calc_iq__phitin_dn4)),)
    } else {
        (locals.var_fn205_calc_iq__two_n_phit0, locals.var_fn205_calc_iq__two_n_phit0_dn4,)
    }
};
        locals.var_fn205_calc_iq__two_n_phit0 = assign17750_e17353;
        locals.var_fn205_calc_iq__two_n_phit0_dn4 = assign17750_e17353_d_n4;

        let (assign17760_e17359, assign17760_e17359_d_n4,) = {
    if (locals.var_guard204 != 0.0) {
        let assign17760_e17357: f64 = (locals.var_fn205_calc_iq__cgin * locals.var_fn205_calc_iq__two_n_phit0);
        (assign17760_e17357, ((locals.var_fn205_calc_iq__cgin_dn4 * locals.var_fn205_calc_iq__two_n_phit0) + (locals.var_fn205_calc_iq__cgin * locals.var_fn205_calc_iq__two_n_phit0_dn4)),)
    } else {
        (locals.var_fn205_calc_iq__qref0, locals.var_fn205_calc_iq__qref0_dn4,)
    }
};
        locals.var_fn205_calc_iq__qref0 = assign17760_e17359;
        locals.var_fn205_calc_iq__qref0_dn4 = assign17760_e17359_d_n4;

        let (assign17770_e17369, assign17770_e17369_d_n4,) = {
    if (locals.var_guard204 != 0.0) {
        let assign17770_e17364: f64 = (p.p51 * locals.var_fn205_calc_iq__alpha_phit);
        let assign17770_e17366: f64 = (assign17770_e17364 / 2.0);
        let assign17770_e17367: f64 = (locals.var_fn205_calc_iq__vtof - assign17770_e17366);
        (assign17770_e17367, (locals.var_fn205_calc_iq__vtof_dn4 - ((p.p51 * locals.var_fn205_calc_iq__alpha_phit_dn4) / 2.0)),)
    } else {
        (locals.var_fn205_calc_iq__myarg0, locals.var_fn205_calc_iq__myarg0_dn4,)
    }
};
        locals.var_fn205_calc_iq__myarg0 = assign17770_e17369;
        locals.var_fn205_calc_iq__myarg0_dn4 = assign17770_e17369_d_n4;

        let (assign17780_e17420, assign17780_e17420_d_n2, assign17780_e17420_d_n4, assign17780_e17420_d_n7, assign17780_e17420_d_n10, assign17780_e17420_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        let (assign17780_e17414, assign17780_e17414_d_n2, assign17780_e17414_d_n7, assign17780_e17414_d_n10, assign17780_e17414_d_n11,) = {
            if (p.p52 != 0.0) {
                let assign17780_e17378: f64 = (locals.var_fn205_calc_iq__vgsin + locals.var_fn205_calc_iq__vgdin);
                let assign17780_e17381: f64 = (locals.var_fn205_calc_iq__vgsin - locals.var_fn205_calc_iq__vgdin);
                let assign17780_e17384: f64 = (0.001 / p.p53);
                let assign17780_e17387: f64 = (locals.var_fn205_calc_iq__vgsin - locals.var_fn205_calc_iq__vgdin);
                let assign17780_e17388: f64 = (assign17780_e17384 * assign17780_e17387);
                let assign17780_e17389: f64 = (assign17780_e17388).tanh();
                let assign17780_e17390: f64 = (assign17780_e17381 * assign17780_e17389);
                let assign17780_e17391: f64 = (assign17780_e17378 + assign17780_e17390);
                let assign17780_e17392: f64 = (0.5 * assign17780_e17391);
                (assign17780_e17392, (0.5 * ((locals.var_fn205_calc_iq__vgsin_dn2 + locals.var_fn205_calc_iq__vgdin_dn2) + (((locals.var_fn205_calc_iq__vgsin_dn2 - locals.var_fn205_calc_iq__vgdin_dn2) * assign17780_e17389) + (assign17780_e17381 * ((assign17780_e17384 * (locals.var_fn205_calc_iq__vgsin_dn2 - locals.var_fn205_calc_iq__vgdin_dn2)) / ((assign17780_e17388).cosh() * (assign17780_e17388).cosh())))))), (0.5 * ((locals.var_fn205_calc_iq__vgsin_dn7 + locals.var_fn205_calc_iq__vgdin_dn7) + (((locals.var_fn205_calc_iq__vgsin_dn7 - locals.var_fn205_calc_iq__vgdin_dn7) * assign17780_e17389) + (assign17780_e17381 * ((assign17780_e17384 * (locals.var_fn205_calc_iq__vgsin_dn7 - locals.var_fn205_calc_iq__vgdin_dn7)) / ((assign17780_e17388).cosh() * (assign17780_e17388).cosh())))))), (0.5 * (locals.var_fn205_calc_iq__vgdin_dn10 + (((-locals.var_fn205_calc_iq__vgdin_dn10) * assign17780_e17389) + (assign17780_e17381 * ((assign17780_e17384 * (-locals.var_fn205_calc_iq__vgdin_dn10)) / ((assign17780_e17388).cosh() * (assign17780_e17388).cosh())))))), (0.5 * ((locals.var_fn205_calc_iq__vgsin_dn11 + locals.var_fn205_calc_iq__vgdin_dn11) + (((locals.var_fn205_calc_iq__vgsin_dn11 - locals.var_fn205_calc_iq__vgdin_dn11) * assign17780_e17389) + (assign17780_e17381 * ((assign17780_e17384 * (locals.var_fn205_calc_iq__vgsin_dn11 - locals.var_fn205_calc_iq__vgdin_dn11)) / ((assign17780_e17388).cosh() * (assign17780_e17388).cosh())))))),)
            } else {
                let (assign17780_e17413, assign17780_e17413_d_n2, assign17780_e17413_d_n7, assign17780_e17413_d_n10, assign17780_e17413_d_n11,) = {
                    if (p.p52 == 0.0) {
                        let assign17780_e17399: f64 = (locals.var_fn205_calc_iq__vgsin + locals.var_fn205_calc_iq__vgdin);
                        let assign17780_e17402: f64 = (locals.var_fn205_calc_iq__vgsin - locals.var_fn205_calc_iq__vgdin);
                        let assign17780_e17405: f64 = (locals.var_fn205_calc_iq__vgsin - locals.var_fn205_calc_iq__vgdin);
                        let assign17780_e17406: f64 = (assign17780_e17402 * assign17780_e17405);
                        let assign17780_e17408: f64 = (assign17780_e17406 + p.p53);
                        let assign17780_e17409: f64 = (assign17780_e17408).sqrt();
                        let assign17780_e17410: f64 = (assign17780_e17399 + assign17780_e17409);
                        let assign17780_e17411: f64 = (0.5 * assign17780_e17410);
                        (assign17780_e17411, (0.5 * ((locals.var_fn205_calc_iq__vgsin_dn2 + locals.var_fn205_calc_iq__vgdin_dn2) + ((((locals.var_fn205_calc_iq__vgsin_dn2 - locals.var_fn205_calc_iq__vgdin_dn2) * assign17780_e17405) + (assign17780_e17402 * (locals.var_fn205_calc_iq__vgsin_dn2 - locals.var_fn205_calc_iq__vgdin_dn2))) / (2.0 * assign17780_e17409)))), (0.5 * ((locals.var_fn205_calc_iq__vgsin_dn7 + locals.var_fn205_calc_iq__vgdin_dn7) + ((((locals.var_fn205_calc_iq__vgsin_dn7 - locals.var_fn205_calc_iq__vgdin_dn7) * assign17780_e17405) + (assign17780_e17402 * (locals.var_fn205_calc_iq__vgsin_dn7 - locals.var_fn205_calc_iq__vgdin_dn7))) / (2.0 * assign17780_e17409)))), (0.5 * (locals.var_fn205_calc_iq__vgdin_dn10 + ((((-locals.var_fn205_calc_iq__vgdin_dn10) * assign17780_e17405) + (assign17780_e17402 * (-locals.var_fn205_calc_iq__vgdin_dn10))) / (2.0 * assign17780_e17409)))), (0.5 * ((locals.var_fn205_calc_iq__vgsin_dn11 + locals.var_fn205_calc_iq__vgdin_dn11) + ((((locals.var_fn205_calc_iq__vgsin_dn11 - locals.var_fn205_calc_iq__vgdin_dn11) * assign17780_e17405) + (assign17780_e17402 * (locals.var_fn205_calc_iq__vgsin_dn11 - locals.var_fn205_calc_iq__vgdin_dn11))) / (2.0 * assign17780_e17409)))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign17780_e17413, assign17780_e17413_d_n2, assign17780_e17413_d_n7, assign17780_e17413_d_n10, assign17780_e17413_d_n11,)
            }
        };
        let assign17780_e17416: f64 = (assign17780_e17414 - locals.var_fn205_calc_iq__myarg0);
        let assign17780_e17418: f64 = (assign17780_e17416 / locals.var_fn205_calc_iq__alpha_phit);
        (assign17780_e17418, (assign17780_e17414_d_n2 / locals.var_fn205_calc_iq__alpha_phit), ((((-locals.var_fn205_calc_iq__myarg0_dn4) * locals.var_fn205_calc_iq__alpha_phit) - (assign17780_e17416 * locals.var_fn205_calc_iq__alpha_phit_dn4)) / (locals.var_fn205_calc_iq__alpha_phit * locals.var_fn205_calc_iq__alpha_phit)), (assign17780_e17414_d_n7 / locals.var_fn205_calc_iq__alpha_phit), (assign17780_e17414_d_n10 / locals.var_fn205_calc_iq__alpha_phit), (assign17780_e17414_d_n11 / locals.var_fn205_calc_iq__alpha_phit),)
    } else {
        (locals.var_fn205_calc_iq__exparg0, locals.var_fn205_calc_iq__exparg0_dn2, locals.var_fn205_calc_iq__exparg0_dn4, locals.var_fn205_calc_iq__exparg0_dn7, locals.var_fn205_calc_iq__exparg0_dn10, locals.var_fn205_calc_iq__exparg0_dn11,)
    }
};
        locals.var_fn205_calc_iq__exparg0 = assign17780_e17420;
        locals.var_fn205_calc_iq__exparg0_dn2 = assign17780_e17420_d_n2;
        locals.var_fn205_calc_iq__exparg0_dn4 = assign17780_e17420_d_n4;
        locals.var_fn205_calc_iq__exparg0_dn7 = assign17780_e17420_d_n7;
        locals.var_fn205_calc_iq__exparg0_dn10 = assign17780_e17420_d_n10;
        locals.var_fn205_calc_iq__exparg0_dn11 = assign17780_e17420_d_n11;

        let assign17790_e17423: f64 = if locals.var_fn205_calc_iq__exparg0 > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard219 = assign17790_e17423;

        let (assign17800_e17429, assign17800_e17429_d_n2, assign17800_e17429_d_n4, assign17800_e17429_d_n7, assign17800_e17429_d_n10, assign17800_e17429_d_n11,) = {
    if ((locals.var_guard204 != 0.0) && (locals.var_guard219 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__ff0, locals.var_fn205_calc_iq__ff0_dn2, locals.var_fn205_calc_iq__ff0_dn4, locals.var_fn205_calc_iq__ff0_dn7, locals.var_fn205_calc_iq__ff0_dn10, locals.var_fn205_calc_iq__ff0_dn11,)
    }
};
        locals.var_fn205_calc_iq__ff0 = assign17800_e17429;
        locals.var_fn205_calc_iq__ff0_dn2 = assign17800_e17429_d_n2;
        locals.var_fn205_calc_iq__ff0_dn4 = assign17800_e17429_d_n4;
        locals.var_fn205_calc_iq__ff0_dn7 = assign17800_e17429_d_n7;
        locals.var_fn205_calc_iq__ff0_dn10 = assign17800_e17429_d_n10;
        locals.var_fn205_calc_iq__ff0_dn11 = assign17800_e17429_d_n11;

        let assign17810_e17432: f64 = (-50.0);
        let assign17810_e17433: f64 = if locals.var_fn205_calc_iq__exparg0 < assign17810_e17432 { 1.0 } else { 0.0 };
        locals.var_guard220 = assign17810_e17433;

        let (assign17820_e17442, assign17820_e17442_d_n2, assign17820_e17442_d_n4, assign17820_e17442_d_n7, assign17820_e17442_d_n10, assign17820_e17442_d_n11,) = {
    if (((locals.var_guard204 != 0.0) && (locals.var_guard219 == 0.0)) && (locals.var_guard220 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__ff0, locals.var_fn205_calc_iq__ff0_dn2, locals.var_fn205_calc_iq__ff0_dn4, locals.var_fn205_calc_iq__ff0_dn7, locals.var_fn205_calc_iq__ff0_dn10, locals.var_fn205_calc_iq__ff0_dn11,)
    }
};
        locals.var_fn205_calc_iq__ff0 = assign17820_e17442;
        locals.var_fn205_calc_iq__ff0_dn2 = assign17820_e17442_d_n2;
        locals.var_fn205_calc_iq__ff0_dn4 = assign17820_e17442_d_n4;
        locals.var_fn205_calc_iq__ff0_dn7 = assign17820_e17442_d_n7;
        locals.var_fn205_calc_iq__ff0_dn10 = assign17820_e17442_d_n10;
        locals.var_fn205_calc_iq__ff0_dn11 = assign17820_e17442_d_n11;

        let (assign17830_e17457, assign17830_e17457_d_n2, assign17830_e17457_d_n4, assign17830_e17457_d_n7, assign17830_e17457_d_n10, assign17830_e17457_d_n11,) = {
    if (((locals.var_guard204 != 0.0) && (locals.var_guard219 == 0.0)) && (locals.var_guard220 == 0.0)) {
        let assign17830_e17453: f64 = (locals.var_fn205_calc_iq__exparg0).exp();
        let assign17830_e17454: f64 = (1.0 + assign17830_e17453);
        let assign17830_e17455: f64 = (1.0 / assign17830_e17454);
        (assign17830_e17455, (-((assign17830_e17453 * locals.var_fn205_calc_iq__exparg0_dn2) / (assign17830_e17454 * assign17830_e17454))), (-((assign17830_e17453 * locals.var_fn205_calc_iq__exparg0_dn4) / (assign17830_e17454 * assign17830_e17454))), (-((assign17830_e17453 * locals.var_fn205_calc_iq__exparg0_dn7) / (assign17830_e17454 * assign17830_e17454))), (-((assign17830_e17453 * locals.var_fn205_calc_iq__exparg0_dn10) / (assign17830_e17454 * assign17830_e17454))), (-((assign17830_e17453 * locals.var_fn205_calc_iq__exparg0_dn11) / (assign17830_e17454 * assign17830_e17454))),)
    } else {
        (locals.var_fn205_calc_iq__ff0, locals.var_fn205_calc_iq__ff0_dn2, locals.var_fn205_calc_iq__ff0_dn4, locals.var_fn205_calc_iq__ff0_dn7, locals.var_fn205_calc_iq__ff0_dn10, locals.var_fn205_calc_iq__ff0_dn11,)
    }
};
        locals.var_fn205_calc_iq__ff0 = assign17830_e17457;
        locals.var_fn205_calc_iq__ff0_dn2 = assign17830_e17457_d_n2;
        locals.var_fn205_calc_iq__ff0_dn4 = assign17830_e17457_d_n4;
        locals.var_fn205_calc_iq__ff0_dn7 = assign17830_e17457_d_n7;
        locals.var_fn205_calc_iq__ff0_dn10 = assign17830_e17457_d_n10;
        locals.var_fn205_calc_iq__ff0_dn11 = assign17830_e17457_d_n11;

        let (assign17840_e17516, assign17840_e17516_d_n2, assign17840_e17516_d_n4, assign17840_e17516_d_n7, assign17840_e17516_d_n10, assign17840_e17516_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        let (assign17840_e17502, assign17840_e17502_d_n2, assign17840_e17502_d_n7, assign17840_e17502_d_n10, assign17840_e17502_d_n11,) = {
            if (p.p52 != 0.0) {
                let assign17840_e17466: f64 = (locals.var_fn205_calc_iq__vgsin + locals.var_fn205_calc_iq__vgdin);
                let assign17840_e17469: f64 = (locals.var_fn205_calc_iq__vgsin - locals.var_fn205_calc_iq__vgdin);
                let assign17840_e17472: f64 = (0.001 / p.p53);
                let assign17840_e17475: f64 = (locals.var_fn205_calc_iq__vgsin - locals.var_fn205_calc_iq__vgdin);
                let assign17840_e17476: f64 = (assign17840_e17472 * assign17840_e17475);
                let assign17840_e17477: f64 = (assign17840_e17476).tanh();
                let assign17840_e17478: f64 = (assign17840_e17469 * assign17840_e17477);
                let assign17840_e17479: f64 = (assign17840_e17466 + assign17840_e17478);
                let assign17840_e17480: f64 = (0.5 * assign17840_e17479);
                (assign17840_e17480, (0.5 * ((locals.var_fn205_calc_iq__vgsin_dn2 + locals.var_fn205_calc_iq__vgdin_dn2) + (((locals.var_fn205_calc_iq__vgsin_dn2 - locals.var_fn205_calc_iq__vgdin_dn2) * assign17840_e17477) + (assign17840_e17469 * ((assign17840_e17472 * (locals.var_fn205_calc_iq__vgsin_dn2 - locals.var_fn205_calc_iq__vgdin_dn2)) / ((assign17840_e17476).cosh() * (assign17840_e17476).cosh())))))), (0.5 * ((locals.var_fn205_calc_iq__vgsin_dn7 + locals.var_fn205_calc_iq__vgdin_dn7) + (((locals.var_fn205_calc_iq__vgsin_dn7 - locals.var_fn205_calc_iq__vgdin_dn7) * assign17840_e17477) + (assign17840_e17469 * ((assign17840_e17472 * (locals.var_fn205_calc_iq__vgsin_dn7 - locals.var_fn205_calc_iq__vgdin_dn7)) / ((assign17840_e17476).cosh() * (assign17840_e17476).cosh())))))), (0.5 * (locals.var_fn205_calc_iq__vgdin_dn10 + (((-locals.var_fn205_calc_iq__vgdin_dn10) * assign17840_e17477) + (assign17840_e17469 * ((assign17840_e17472 * (-locals.var_fn205_calc_iq__vgdin_dn10)) / ((assign17840_e17476).cosh() * (assign17840_e17476).cosh())))))), (0.5 * ((locals.var_fn205_calc_iq__vgsin_dn11 + locals.var_fn205_calc_iq__vgdin_dn11) + (((locals.var_fn205_calc_iq__vgsin_dn11 - locals.var_fn205_calc_iq__vgdin_dn11) * assign17840_e17477) + (assign17840_e17469 * ((assign17840_e17472 * (locals.var_fn205_calc_iq__vgsin_dn11 - locals.var_fn205_calc_iq__vgdin_dn11)) / ((assign17840_e17476).cosh() * (assign17840_e17476).cosh())))))),)
            } else {
                let (assign17840_e17501, assign17840_e17501_d_n2, assign17840_e17501_d_n7, assign17840_e17501_d_n10, assign17840_e17501_d_n11,) = {
                    if (p.p52 == 0.0) {
                        let assign17840_e17487: f64 = (locals.var_fn205_calc_iq__vgsin + locals.var_fn205_calc_iq__vgdin);
                        let assign17840_e17490: f64 = (locals.var_fn205_calc_iq__vgsin - locals.var_fn205_calc_iq__vgdin);
                        let assign17840_e17493: f64 = (locals.var_fn205_calc_iq__vgsin - locals.var_fn205_calc_iq__vgdin);
                        let assign17840_e17494: f64 = (assign17840_e17490 * assign17840_e17493);
                        let assign17840_e17496: f64 = (assign17840_e17494 + p.p53);
                        let assign17840_e17497: f64 = (assign17840_e17496).sqrt();
                        let assign17840_e17498: f64 = (assign17840_e17487 + assign17840_e17497);
                        let assign17840_e17499: f64 = (0.5 * assign17840_e17498);
                        (assign17840_e17499, (0.5 * ((locals.var_fn205_calc_iq__vgsin_dn2 + locals.var_fn205_calc_iq__vgdin_dn2) + ((((locals.var_fn205_calc_iq__vgsin_dn2 - locals.var_fn205_calc_iq__vgdin_dn2) * assign17840_e17493) + (assign17840_e17490 * (locals.var_fn205_calc_iq__vgsin_dn2 - locals.var_fn205_calc_iq__vgdin_dn2))) / (2.0 * assign17840_e17497)))), (0.5 * ((locals.var_fn205_calc_iq__vgsin_dn7 + locals.var_fn205_calc_iq__vgdin_dn7) + ((((locals.var_fn205_calc_iq__vgsin_dn7 - locals.var_fn205_calc_iq__vgdin_dn7) * assign17840_e17493) + (assign17840_e17490 * (locals.var_fn205_calc_iq__vgsin_dn7 - locals.var_fn205_calc_iq__vgdin_dn7))) / (2.0 * assign17840_e17497)))), (0.5 * (locals.var_fn205_calc_iq__vgdin_dn10 + ((((-locals.var_fn205_calc_iq__vgdin_dn10) * assign17840_e17493) + (assign17840_e17490 * (-locals.var_fn205_calc_iq__vgdin_dn10))) / (2.0 * assign17840_e17497)))), (0.5 * ((locals.var_fn205_calc_iq__vgsin_dn11 + locals.var_fn205_calc_iq__vgdin_dn11) + ((((locals.var_fn205_calc_iq__vgsin_dn11 - locals.var_fn205_calc_iq__vgdin_dn11) * assign17840_e17493) + (assign17840_e17490 * (locals.var_fn205_calc_iq__vgsin_dn11 - locals.var_fn205_calc_iq__vgdin_dn11))) / (2.0 * assign17840_e17497)))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign17840_e17501, assign17840_e17501_d_n2, assign17840_e17501_d_n7, assign17840_e17501_d_n10, assign17840_e17501_d_n11,)
            }
        };
        let assign17840_e17506: f64 = (p.p51 * 0.1);
        let assign17840_e17508: f64 = (assign17840_e17506 * locals.var_fn205_calc_iq__alpha_phit);
        let assign17840_e17510: f64 = (assign17840_e17508 * locals.var_fn205_calc_iq__ff0);
        let assign17840_e17511: f64 = (locals.var_fn205_calc_iq__vtof - assign17840_e17510);
        let assign17840_e17512: f64 = (assign17840_e17502 - assign17840_e17511);
        let assign17840_e17514: f64 = (assign17840_e17512 / locals.var_fn205_calc_iq__two_n_phit0);
        (assign17840_e17514, ((assign17840_e17502_d_n2 - (-(assign17840_e17508 * locals.var_fn205_calc_iq__ff0_dn2))) / locals.var_fn205_calc_iq__two_n_phit0), ((((-(locals.var_fn205_calc_iq__vtof_dn4 - (((assign17840_e17506 * locals.var_fn205_calc_iq__alpha_phit_dn4) * locals.var_fn205_calc_iq__ff0) + (assign17840_e17508 * locals.var_fn205_calc_iq__ff0_dn4)))) * locals.var_fn205_calc_iq__two_n_phit0) - (assign17840_e17512 * locals.var_fn205_calc_iq__two_n_phit0_dn4)) / (locals.var_fn205_calc_iq__two_n_phit0 * locals.var_fn205_calc_iq__two_n_phit0)), ((assign17840_e17502_d_n7 - (-(assign17840_e17508 * locals.var_fn205_calc_iq__ff0_dn7))) / locals.var_fn205_calc_iq__two_n_phit0), ((assign17840_e17502_d_n10 - (-(assign17840_e17508 * locals.var_fn205_calc_iq__ff0_dn10))) / locals.var_fn205_calc_iq__two_n_phit0), ((assign17840_e17502_d_n11 - (-(assign17840_e17508 * locals.var_fn205_calc_iq__ff0_dn11))) / locals.var_fn205_calc_iq__two_n_phit0),)
    } else {
        (locals.var_fn205_calc_iq__eta0, locals.var_fn205_calc_iq__eta0_dn2, locals.var_fn205_calc_iq__eta0_dn4, locals.var_fn205_calc_iq__eta0_dn7, locals.var_fn205_calc_iq__eta0_dn10, locals.var_fn205_calc_iq__eta0_dn11,)
    }
};
        locals.var_fn205_calc_iq__eta0 = assign17840_e17516;
        locals.var_fn205_calc_iq__eta0_dn2 = assign17840_e17516_d_n2;
        locals.var_fn205_calc_iq__eta0_dn4 = assign17840_e17516_d_n4;
        locals.var_fn205_calc_iq__eta0_dn7 = assign17840_e17516_d_n7;
        locals.var_fn205_calc_iq__eta0_dn10 = assign17840_e17516_d_n10;
        locals.var_fn205_calc_iq__eta0_dn11 = assign17840_e17516_d_n11;

        let assign17850_e17519: f64 = if locals.var_fn205_calc_iq__eta0 > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard221 = assign17850_e17519;

        let (assign17860_e17527, assign17860_e17527_d_n2, assign17860_e17527_d_n4, assign17860_e17527_d_n7, assign17860_e17527_d_n10, assign17860_e17527_d_n11,) = {
    if ((locals.var_guard204 != 0.0) && (locals.var_guard221 != 0.0)) {
        let assign17860_e17525: f64 = (locals.var_fn205_calc_iq__qref0 * locals.var_fn205_calc_iq__eta0);
        (assign17860_e17525, (locals.var_fn205_calc_iq__qref0 * locals.var_fn205_calc_iq__eta0_dn2), ((locals.var_fn205_calc_iq__qref0_dn4 * locals.var_fn205_calc_iq__eta0) + (locals.var_fn205_calc_iq__qref0 * locals.var_fn205_calc_iq__eta0_dn4)), (locals.var_fn205_calc_iq__qref0 * locals.var_fn205_calc_iq__eta0_dn7), (locals.var_fn205_calc_iq__qref0 * locals.var_fn205_calc_iq__eta0_dn10), (locals.var_fn205_calc_iq__qref0 * locals.var_fn205_calc_iq__eta0_dn11),)
    } else {
        (locals.var_fn205_calc_iq__qinvv0, locals.var_fn205_calc_iq__qinvv0_dn2, locals.var_fn205_calc_iq__qinvv0_dn4, locals.var_fn205_calc_iq__qinvv0_dn7, locals.var_fn205_calc_iq__qinvv0_dn10, locals.var_fn205_calc_iq__qinvv0_dn11,)
    }
};
        locals.var_fn205_calc_iq__qinvv0 = assign17860_e17527;
        locals.var_fn205_calc_iq__qinvv0_dn2 = assign17860_e17527_d_n2;
        locals.var_fn205_calc_iq__qinvv0_dn4 = assign17860_e17527_d_n4;
        locals.var_fn205_calc_iq__qinvv0_dn7 = assign17860_e17527_d_n7;
        locals.var_fn205_calc_iq__qinvv0_dn10 = assign17860_e17527_d_n10;
        locals.var_fn205_calc_iq__qinvv0_dn11 = assign17860_e17527_d_n11;

        let assign17870_e17530: f64 = (-50.0);
        let assign17870_e17531: f64 = if locals.var_fn205_calc_iq__eta0 < assign17870_e17530 { 1.0 } else { 0.0 };
        locals.var_guard222 = assign17870_e17531;

        let (assign17880_e17543, assign17880_e17543_d_n2, assign17880_e17543_d_n4, assign17880_e17543_d_n7, assign17880_e17543_d_n10, assign17880_e17543_d_n11,) = {
    if (((locals.var_guard204 != 0.0) && (locals.var_guard221 == 0.0)) && (locals.var_guard222 != 0.0)) {
        let assign17880_e17540: f64 = (locals.var_fn205_calc_iq__eta0).exp();
        let assign17880_e17541: f64 = (locals.var_fn205_calc_iq__qref0 * assign17880_e17540);
        (assign17880_e17541, (locals.var_fn205_calc_iq__qref0 * (assign17880_e17540 * locals.var_fn205_calc_iq__eta0_dn2)), ((locals.var_fn205_calc_iq__qref0_dn4 * assign17880_e17540) + (locals.var_fn205_calc_iq__qref0 * (assign17880_e17540 * locals.var_fn205_calc_iq__eta0_dn4))), (locals.var_fn205_calc_iq__qref0 * (assign17880_e17540 * locals.var_fn205_calc_iq__eta0_dn7)), (locals.var_fn205_calc_iq__qref0 * (assign17880_e17540 * locals.var_fn205_calc_iq__eta0_dn10)), (locals.var_fn205_calc_iq__qref0 * (assign17880_e17540 * locals.var_fn205_calc_iq__eta0_dn11)),)
    } else {
        (locals.var_fn205_calc_iq__qinvv0, locals.var_fn205_calc_iq__qinvv0_dn2, locals.var_fn205_calc_iq__qinvv0_dn4, locals.var_fn205_calc_iq__qinvv0_dn7, locals.var_fn205_calc_iq__qinvv0_dn10, locals.var_fn205_calc_iq__qinvv0_dn11,)
    }
};
        locals.var_fn205_calc_iq__qinvv0 = assign17880_e17543;
        locals.var_fn205_calc_iq__qinvv0_dn2 = assign17880_e17543_d_n2;
        locals.var_fn205_calc_iq__qinvv0_dn4 = assign17880_e17543_d_n4;
        locals.var_fn205_calc_iq__qinvv0_dn7 = assign17880_e17543_d_n7;
        locals.var_fn205_calc_iq__qinvv0_dn10 = assign17880_e17543_d_n10;
        locals.var_fn205_calc_iq__qinvv0_dn11 = assign17880_e17543_d_n11;

        let (assign17890_e17559, assign17890_e17559_d_n2, assign17890_e17559_d_n4, assign17890_e17559_d_n7, assign17890_e17559_d_n10, assign17890_e17559_d_n11,) = {
    if (((locals.var_guard204 != 0.0) && (locals.var_guard221 == 0.0)) && (locals.var_guard222 == 0.0)) {
        let assign17890_e17554: f64 = (locals.var_fn205_calc_iq__eta0).exp();
        let assign17890_e17555: f64 = (1.0 + assign17890_e17554);
        let assign17890_e17556: f64 = (assign17890_e17555).ln();
        let assign17890_e17557: f64 = (locals.var_fn205_calc_iq__qref0 * assign17890_e17556);
        (assign17890_e17557, (locals.var_fn205_calc_iq__qref0 * ((assign17890_e17554 * locals.var_fn205_calc_iq__eta0_dn2) / assign17890_e17555)), ((locals.var_fn205_calc_iq__qref0_dn4 * assign17890_e17556) + (locals.var_fn205_calc_iq__qref0 * ((assign17890_e17554 * locals.var_fn205_calc_iq__eta0_dn4) / assign17890_e17555))), (locals.var_fn205_calc_iq__qref0 * ((assign17890_e17554 * locals.var_fn205_calc_iq__eta0_dn7) / assign17890_e17555)), (locals.var_fn205_calc_iq__qref0 * ((assign17890_e17554 * locals.var_fn205_calc_iq__eta0_dn10) / assign17890_e17555)), (locals.var_fn205_calc_iq__qref0 * ((assign17890_e17554 * locals.var_fn205_calc_iq__eta0_dn11) / assign17890_e17555)),)
    } else {
        (locals.var_fn205_calc_iq__qinvv0, locals.var_fn205_calc_iq__qinvv0_dn2, locals.var_fn205_calc_iq__qinvv0_dn4, locals.var_fn205_calc_iq__qinvv0_dn7, locals.var_fn205_calc_iq__qinvv0_dn10, locals.var_fn205_calc_iq__qinvv0_dn11,)
    }
};
        locals.var_fn205_calc_iq__qinvv0 = assign17890_e17559;
        locals.var_fn205_calc_iq__qinvv0_dn2 = assign17890_e17559_d_n2;
        locals.var_fn205_calc_iq__qinvv0_dn4 = assign17890_e17559_d_n4;
        locals.var_fn205_calc_iq__qinvv0_dn7 = assign17890_e17559_d_n7;
        locals.var_fn205_calc_iq__qinvv0_dn10 = assign17890_e17559_d_n10;
        locals.var_fn205_calc_iq__qinvv0_dn11 = assign17890_e17559_d_n11;

        let (assign17900_e17565, assign17900_e17565_d_n4,) = {
    if (locals.var_guard204 != 0.0) {
        let assign17900_e17563: f64 = (locals.var_fn205_calc_iq__mu0 / locals.var_fn205_calc_iq__tfacmobin);
        (assign17900_e17563, (-((locals.var_fn205_calc_iq__mu0 * locals.var_fn205_calc_iq__tfacmobin_dn4) / (locals.var_fn205_calc_iq__tfacmobin * locals.var_fn205_calc_iq__tfacmobin))),)
    } else {
        (locals.var_fn205_calc_iq__muf0, locals.var_fn205_calc_iq__muf0_dn4,)
    }
};
        locals.var_fn205_calc_iq__muf0 = assign17900_e17565;
        locals.var_fn205_calc_iq__muf0_dn4 = assign17900_e17565_d_n4;

        let (assign17910_e17581, assign17910_e17581_d_n4,) = {
    if (locals.var_guard204 != 0.0) {
        let assign17910_e17571: f64 = (locals.var_fn205_calc_iq__vzeta * locals.var_fn205_calc_iq__tnomin);
        let assign17910_e17572: f64 = (1.0 + assign17910_e17571);
        let assign17910_e17576: f64 = (locals.var_fn205_calc_iq__vzeta * locals.var_fn205_calc_iq__tambin);
        let assign17910_e17577: f64 = (1.0 + assign17910_e17576);
        let assign17910_e17578: f64 = (assign17910_e17572 / assign17910_e17577);
        let assign17910_e17579: f64 = (locals.var_fn205_calc_iq__vel0 * assign17910_e17578);
        (assign17910_e17579, (locals.var_fn205_calc_iq__vel0 * (-((assign17910_e17572 * (locals.var_fn205_calc_iq__vzeta * locals.var_fn205_calc_iq__tambin_dn4)) / (assign17910_e17577 * assign17910_e17577)))),)
    } else {
        (locals.var_fn205_calc_iq__vx0, locals.var_fn205_calc_iq__vx0_dn4,)
    }
};
        locals.var_fn205_calc_iq__vx0 = assign17910_e17581;
        locals.var_fn205_calc_iq__vx0_dn4 = assign17910_e17581_d_n4;

    }

    pub(super) fn stamp_transient_block_45(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign17920_e17589, assign17920_e17589_d_n4,) = {
    if (locals.var_guard204 != 0.0) {
        let assign17920_e17585: f64 = (locals.var_fn205_calc_iq__vx0 * locals.var_fn205_calc_iq__lin);
        let assign17920_e17587: f64 = (assign17920_e17585 / locals.var_fn205_calc_iq__muf0);
        (assign17920_e17587, ((((locals.var_fn205_calc_iq__vx0_dn4 * locals.var_fn205_calc_iq__lin) * locals.var_fn205_calc_iq__muf0) - (assign17920_e17585 * locals.var_fn205_calc_iq__muf0_dn4)) / (locals.var_fn205_calc_iq__muf0 * locals.var_fn205_calc_iq__muf0)),)
    } else {
        (locals.var_fn205_calc_iq__vdsats0, locals.var_fn205_calc_iq__vdsats0_dn4,)
    }
};
        locals.var_fn205_calc_iq__vdsats0 = assign17920_e17589;
        locals.var_fn205_calc_iq__vdsats0_dn4 = assign17920_e17589_d_n4;

        let (assign17930_e17606, assign17930_e17606_d_n2, assign17930_e17606_d_n4, assign17930_e17606_d_n7, assign17930_e17606_d_n10, assign17930_e17606_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        let assign17930_e17595: f64 = (2.0 * locals.var_fn205_calc_iq__qinvv0);
        let assign17930_e17597: f64 = (assign17930_e17595 / locals.var_fn205_calc_iq__cgin);
        let assign17930_e17599: f64 = (assign17930_e17597 / locals.var_fn205_calc_iq__vdsats0);
        let assign17930_e17600: f64 = (1.0 + assign17930_e17599);
        let assign17930_e17601: f64 = (assign17930_e17600).sqrt();
        let assign17930_e17602: f64 = (locals.var_fn205_calc_iq__vdsats0 * assign17930_e17601);
        let assign17930_e17604: f64 = (assign17930_e17602 - locals.var_fn205_calc_iq__vdsats0);
        (assign17930_e17604, (locals.var_fn205_calc_iq__vdsats0 * ((((2.0 * locals.var_fn205_calc_iq__qinvv0_dn2) / locals.var_fn205_calc_iq__cgin) / locals.var_fn205_calc_iq__vdsats0) / (2.0 * assign17930_e17601))), (((locals.var_fn205_calc_iq__vdsats0_dn4 * assign17930_e17601) + (locals.var_fn205_calc_iq__vdsats0 * ((((((((2.0 * locals.var_fn205_calc_iq__qinvv0_dn4) * locals.var_fn205_calc_iq__cgin) - (assign17930_e17595 * locals.var_fn205_calc_iq__cgin_dn4)) / (locals.var_fn205_calc_iq__cgin * locals.var_fn205_calc_iq__cgin)) * locals.var_fn205_calc_iq__vdsats0) - (assign17930_e17597 * locals.var_fn205_calc_iq__vdsats0_dn4)) / (locals.var_fn205_calc_iq__vdsats0 * locals.var_fn205_calc_iq__vdsats0)) / (2.0 * assign17930_e17601)))) - locals.var_fn205_calc_iq__vdsats0_dn4), (locals.var_fn205_calc_iq__vdsats0 * ((((2.0 * locals.var_fn205_calc_iq__qinvv0_dn7) / locals.var_fn205_calc_iq__cgin) / locals.var_fn205_calc_iq__vdsats0) / (2.0 * assign17930_e17601))), (locals.var_fn205_calc_iq__vdsats0 * ((((2.0 * locals.var_fn205_calc_iq__qinvv0_dn10) / locals.var_fn205_calc_iq__cgin) / locals.var_fn205_calc_iq__vdsats0) / (2.0 * assign17930_e17601))), (locals.var_fn205_calc_iq__vdsats0 * ((((2.0 * locals.var_fn205_calc_iq__qinvv0_dn11) / locals.var_fn205_calc_iq__cgin) / locals.var_fn205_calc_iq__vdsats0) / (2.0 * assign17930_e17601))),)
    } else {
        (locals.var_fn205_calc_iq__vdsats10, locals.var_fn205_calc_iq__vdsats10_dn2, locals.var_fn205_calc_iq__vdsats10_dn4, locals.var_fn205_calc_iq__vdsats10_dn7, locals.var_fn205_calc_iq__vdsats10_dn10, locals.var_fn205_calc_iq__vdsats10_dn11,)
    }
};
        locals.var_fn205_calc_iq__vdsats10 = assign17930_e17606;
        locals.var_fn205_calc_iq__vdsats10_dn2 = assign17930_e17606_d_n2;
        locals.var_fn205_calc_iq__vdsats10_dn4 = assign17930_e17606_d_n4;
        locals.var_fn205_calc_iq__vdsats10_dn7 = assign17930_e17606_d_n7;
        locals.var_fn205_calc_iq__vdsats10_dn10 = assign17930_e17606_d_n10;
        locals.var_fn205_calc_iq__vdsats10_dn11 = assign17930_e17606_d_n11;

        let (assign17940_e17618, assign17940_e17618_d_n2, assign17940_e17618_d_n4, assign17940_e17618_d_n7, assign17940_e17618_d_n10, assign17940_e17618_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        let assign17940_e17611: f64 = (1.0 - locals.var_fn205_calc_iq__ff0);
        let assign17940_e17612: f64 = (locals.var_fn205_calc_iq__vdsats10 * assign17940_e17611);
        let assign17940_e17615: f64 = (locals.var_fn205_calc_iq__two_n_phit0 * locals.var_fn205_calc_iq__ff0);
        let assign17940_e17616: f64 = (assign17940_e17612 + assign17940_e17615);
        (assign17940_e17616, (((locals.var_fn205_calc_iq__vdsats10_dn2 * assign17940_e17611) + (locals.var_fn205_calc_iq__vdsats10 * (-locals.var_fn205_calc_iq__ff0_dn2))) + (locals.var_fn205_calc_iq__two_n_phit0 * locals.var_fn205_calc_iq__ff0_dn2)), (((locals.var_fn205_calc_iq__vdsats10_dn4 * assign17940_e17611) + (locals.var_fn205_calc_iq__vdsats10 * (-locals.var_fn205_calc_iq__ff0_dn4))) + ((locals.var_fn205_calc_iq__two_n_phit0_dn4 * locals.var_fn205_calc_iq__ff0) + (locals.var_fn205_calc_iq__two_n_phit0 * locals.var_fn205_calc_iq__ff0_dn4))), (((locals.var_fn205_calc_iq__vdsats10_dn7 * assign17940_e17611) + (locals.var_fn205_calc_iq__vdsats10 * (-locals.var_fn205_calc_iq__ff0_dn7))) + (locals.var_fn205_calc_iq__two_n_phit0 * locals.var_fn205_calc_iq__ff0_dn7)), (((locals.var_fn205_calc_iq__vdsats10_dn10 * assign17940_e17611) + (locals.var_fn205_calc_iq__vdsats10 * (-locals.var_fn205_calc_iq__ff0_dn10))) + (locals.var_fn205_calc_iq__two_n_phit0 * locals.var_fn205_calc_iq__ff0_dn10)), (((locals.var_fn205_calc_iq__vdsats10_dn11 * assign17940_e17611) + (locals.var_fn205_calc_iq__vdsats10 * (-locals.var_fn205_calc_iq__ff0_dn11))) + (locals.var_fn205_calc_iq__two_n_phit0 * locals.var_fn205_calc_iq__ff0_dn11)),)
    } else {
        (locals.var_fn205_calc_iq__vdsat10, locals.var_fn205_calc_iq__vdsat10_dn2, locals.var_fn205_calc_iq__vdsat10_dn4, locals.var_fn205_calc_iq__vdsat10_dn7, locals.var_fn205_calc_iq__vdsat10_dn10, locals.var_fn205_calc_iq__vdsat10_dn11,)
    }
};
        locals.var_fn205_calc_iq__vdsat10 = assign17940_e17618;
        locals.var_fn205_calc_iq__vdsat10_dn2 = assign17940_e17618_d_n2;
        locals.var_fn205_calc_iq__vdsat10_dn4 = assign17940_e17618_d_n4;
        locals.var_fn205_calc_iq__vdsat10_dn7 = assign17940_e17618_d_n7;
        locals.var_fn205_calc_iq__vdsat10_dn10 = assign17940_e17618_d_n10;
        locals.var_fn205_calc_iq__vdsat10_dn11 = assign17940_e17618_d_n11;

        let (assign17950_e17687, assign17950_e17687_d_n2, assign17950_e17687_d_n4, assign17950_e17687_d_n7, assign17950_e17687_d_n10, assign17950_e17687_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        let (assign17950_e17677, assign17950_e17677_d_n2, assign17950_e17677_d_n4, assign17950_e17677_d_n7, assign17950_e17677_d_n10, assign17950_e17677_d_n11,) = {
            if (p.p52 != 0.0) {
                let assign17950_e17630: f64 = (locals.var_fn205_calc_iq__vdsin / locals.var_fn205_calc_iq__vdsat10);
                let assign17950_e17631: f64 = assign17950_e17630;
                let assign17950_e17635: f64 = (locals.var_fn205_calc_iq__vdsin / locals.var_fn205_calc_iq__vdsat10);
                let assign17950_e17636: f64 = (-assign17950_e17635);
                let assign17950_e17639: f64 = (0.001 / p.p53);
                let assign17950_e17643: f64 = (locals.var_fn205_calc_iq__vdsin / locals.var_fn205_calc_iq__vdsat10);
                let assign17950_e17644: f64 = (-assign17950_e17643);
                let assign17950_e17645: f64 = (assign17950_e17639 * assign17950_e17644);
                let assign17950_e17646: f64 = (assign17950_e17645).tanh();
                let assign17950_e17647: f64 = (assign17950_e17636 * assign17950_e17646);
                let assign17950_e17648: f64 = (assign17950_e17631 + assign17950_e17647);
                let assign17950_e17649: f64 = (0.5 * assign17950_e17648);
                (assign17950_e17649, (0.5 * ((-((locals.var_fn205_calc_iq__vdsin * locals.var_fn205_calc_iq__vdsat10_dn2) / (locals.var_fn205_calc_iq__vdsat10 * locals.var_fn205_calc_iq__vdsat10))) + (((-(-((locals.var_fn205_calc_iq__vdsin * locals.var_fn205_calc_iq__vdsat10_dn2) / (locals.var_fn205_calc_iq__vdsat10 * locals.var_fn205_calc_iq__vdsat10)))) * assign17950_e17646) + (assign17950_e17636 * ((assign17950_e17639 * (-(-((locals.var_fn205_calc_iq__vdsin * locals.var_fn205_calc_iq__vdsat10_dn2) / (locals.var_fn205_calc_iq__vdsat10 * locals.var_fn205_calc_iq__vdsat10))))) / ((assign17950_e17645).cosh() * (assign17950_e17645).cosh())))))), (0.5 * ((-((locals.var_fn205_calc_iq__vdsin * locals.var_fn205_calc_iq__vdsat10_dn4) / (locals.var_fn205_calc_iq__vdsat10 * locals.var_fn205_calc_iq__vdsat10))) + (((-(-((locals.var_fn205_calc_iq__vdsin * locals.var_fn205_calc_iq__vdsat10_dn4) / (locals.var_fn205_calc_iq__vdsat10 * locals.var_fn205_calc_iq__vdsat10)))) * assign17950_e17646) + (assign17950_e17636 * ((assign17950_e17639 * (-(-((locals.var_fn205_calc_iq__vdsin * locals.var_fn205_calc_iq__vdsat10_dn4) / (locals.var_fn205_calc_iq__vdsat10 * locals.var_fn205_calc_iq__vdsat10))))) / ((assign17950_e17645).cosh() * (assign17950_e17645).cosh())))))), (0.5 * ((-((locals.var_fn205_calc_iq__vdsin * locals.var_fn205_calc_iq__vdsat10_dn7) / (locals.var_fn205_calc_iq__vdsat10 * locals.var_fn205_calc_iq__vdsat10))) + (((-(-((locals.var_fn205_calc_iq__vdsin * locals.var_fn205_calc_iq__vdsat10_dn7) / (locals.var_fn205_calc_iq__vdsat10 * locals.var_fn205_calc_iq__vdsat10)))) * assign17950_e17646) + (assign17950_e17636 * ((assign17950_e17639 * (-(-((locals.var_fn205_calc_iq__vdsin * locals.var_fn205_calc_iq__vdsat10_dn7) / (locals.var_fn205_calc_iq__vdsat10 * locals.var_fn205_calc_iq__vdsat10))))) / ((assign17950_e17645).cosh() * (assign17950_e17645).cosh())))))), (0.5 * ((((locals.var_fn205_calc_iq__vdsin_dn10 * locals.var_fn205_calc_iq__vdsat10) - (locals.var_fn205_calc_iq__vdsin * locals.var_fn205_calc_iq__vdsat10_dn10)) / (locals.var_fn205_calc_iq__vdsat10 * locals.var_fn205_calc_iq__vdsat10)) + (((-(((locals.var_fn205_calc_iq__vdsin_dn10 * locals.var_fn205_calc_iq__vdsat10) - (locals.var_fn205_calc_iq__vdsin * locals.var_fn205_calc_iq__vdsat10_dn10)) / (locals.var_fn205_calc_iq__vdsat10 * locals.var_fn205_calc_iq__vdsat10))) * assign17950_e17646) + (assign17950_e17636 * ((assign17950_e17639 * (-(((locals.var_fn205_calc_iq__vdsin_dn10 * locals.var_fn205_calc_iq__vdsat10) - (locals.var_fn205_calc_iq__vdsin * locals.var_fn205_calc_iq__vdsat10_dn10)) / (locals.var_fn205_calc_iq__vdsat10 * locals.var_fn205_calc_iq__vdsat10)))) / ((assign17950_e17645).cosh() * (assign17950_e17645).cosh())))))), (0.5 * ((((locals.var_fn205_calc_iq__vdsin_dn11 * locals.var_fn205_calc_iq__vdsat10) - (locals.var_fn205_calc_iq__vdsin * locals.var_fn205_calc_iq__vdsat10_dn11)) / (locals.var_fn205_calc_iq__vdsat10 * locals.var_fn205_calc_iq__vdsat10)) + (((-(((locals.var_fn205_calc_iq__vdsin_dn11 * locals.var_fn205_calc_iq__vdsat10) - (locals.var_fn205_calc_iq__vdsin * locals.var_fn205_calc_iq__vdsat10_dn11)) / (locals.var_fn205_calc_iq__vdsat10 * locals.var_fn205_calc_iq__vdsat10))) * assign17950_e17646) + (assign17950_e17636 * ((assign17950_e17639 * (-(((locals.var_fn205_calc_iq__vdsin_dn11 * locals.var_fn205_calc_iq__vdsat10) - (locals.var_fn205_calc_iq__vdsin * locals.var_fn205_calc_iq__vdsat10_dn11)) / (locals.var_fn205_calc_iq__vdsat10 * locals.var_fn205_calc_iq__vdsat10)))) / ((assign17950_e17645).cosh() * (assign17950_e17645).cosh())))))),)
            } else {
                let (assign17950_e17676, assign17950_e17676_d_n2, assign17950_e17676_d_n4, assign17950_e17676_d_n7, assign17950_e17676_d_n10, assign17950_e17676_d_n11,) = {
                    if (p.p52 == 0.0) {
                        let assign17950_e17657: f64 = (locals.var_fn205_calc_iq__vdsin / locals.var_fn205_calc_iq__vdsat10);
                        let assign17950_e17658: f64 = assign17950_e17657;
                        let assign17950_e17662: f64 = (locals.var_fn205_calc_iq__vdsin / locals.var_fn205_calc_iq__vdsat10);
                        let assign17950_e17663: f64 = (-assign17950_e17662);
                        let assign17950_e17667: f64 = (locals.var_fn205_calc_iq__vdsin / locals.var_fn205_calc_iq__vdsat10);
                        let assign17950_e17668: f64 = (-assign17950_e17667);
                        let assign17950_e17669: f64 = (assign17950_e17663 * assign17950_e17668);
                        let assign17950_e17671: f64 = (assign17950_e17669 + p.p53);
                        let assign17950_e17672: f64 = (assign17950_e17671).sqrt();
                        let assign17950_e17673: f64 = (assign17950_e17658 + assign17950_e17672);
                        let assign17950_e17674: f64 = (0.5 * assign17950_e17673);
                        (assign17950_e17674, (0.5 * ((-((locals.var_fn205_calc_iq__vdsin * locals.var_fn205_calc_iq__vdsat10_dn2) / (locals.var_fn205_calc_iq__vdsat10 * locals.var_fn205_calc_iq__vdsat10))) + ((((-(-((locals.var_fn205_calc_iq__vdsin * locals.var_fn205_calc_iq__vdsat10_dn2) / (locals.var_fn205_calc_iq__vdsat10 * locals.var_fn205_calc_iq__vdsat10)))) * assign17950_e17668) + (assign17950_e17663 * (-(-((locals.var_fn205_calc_iq__vdsin * locals.var_fn205_calc_iq__vdsat10_dn2) / (locals.var_fn205_calc_iq__vdsat10 * locals.var_fn205_calc_iq__vdsat10)))))) / (2.0 * assign17950_e17672)))), (0.5 * ((-((locals.var_fn205_calc_iq__vdsin * locals.var_fn205_calc_iq__vdsat10_dn4) / (locals.var_fn205_calc_iq__vdsat10 * locals.var_fn205_calc_iq__vdsat10))) + ((((-(-((locals.var_fn205_calc_iq__vdsin * locals.var_fn205_calc_iq__vdsat10_dn4) / (locals.var_fn205_calc_iq__vdsat10 * locals.var_fn205_calc_iq__vdsat10)))) * assign17950_e17668) + (assign17950_e17663 * (-(-((locals.var_fn205_calc_iq__vdsin * locals.var_fn205_calc_iq__vdsat10_dn4) / (locals.var_fn205_calc_iq__vdsat10 * locals.var_fn205_calc_iq__vdsat10)))))) / (2.0 * assign17950_e17672)))), (0.5 * ((-((locals.var_fn205_calc_iq__vdsin * locals.var_fn205_calc_iq__vdsat10_dn7) / (locals.var_fn205_calc_iq__vdsat10 * locals.var_fn205_calc_iq__vdsat10))) + ((((-(-((locals.var_fn205_calc_iq__vdsin * locals.var_fn205_calc_iq__vdsat10_dn7) / (locals.var_fn205_calc_iq__vdsat10 * locals.var_fn205_calc_iq__vdsat10)))) * assign17950_e17668) + (assign17950_e17663 * (-(-((locals.var_fn205_calc_iq__vdsin * locals.var_fn205_calc_iq__vdsat10_dn7) / (locals.var_fn205_calc_iq__vdsat10 * locals.var_fn205_calc_iq__vdsat10)))))) / (2.0 * assign17950_e17672)))), (0.5 * ((((locals.var_fn205_calc_iq__vdsin_dn10 * locals.var_fn205_calc_iq__vdsat10) - (locals.var_fn205_calc_iq__vdsin * locals.var_fn205_calc_iq__vdsat10_dn10)) / (locals.var_fn205_calc_iq__vdsat10 * locals.var_fn205_calc_iq__vdsat10)) + ((((-(((locals.var_fn205_calc_iq__vdsin_dn10 * locals.var_fn205_calc_iq__vdsat10) - (locals.var_fn205_calc_iq__vdsin * locals.var_fn205_calc_iq__vdsat10_dn10)) / (locals.var_fn205_calc_iq__vdsat10 * locals.var_fn205_calc_iq__vdsat10))) * assign17950_e17668) + (assign17950_e17663 * (-(((locals.var_fn205_calc_iq__vdsin_dn10 * locals.var_fn205_calc_iq__vdsat10) - (locals.var_fn205_calc_iq__vdsin * locals.var_fn205_calc_iq__vdsat10_dn10)) / (locals.var_fn205_calc_iq__vdsat10 * locals.var_fn205_calc_iq__vdsat10))))) / (2.0 * assign17950_e17672)))), (0.5 * ((((locals.var_fn205_calc_iq__vdsin_dn11 * locals.var_fn205_calc_iq__vdsat10) - (locals.var_fn205_calc_iq__vdsin * locals.var_fn205_calc_iq__vdsat10_dn11)) / (locals.var_fn205_calc_iq__vdsat10 * locals.var_fn205_calc_iq__vdsat10)) + ((((-(((locals.var_fn205_calc_iq__vdsin_dn11 * locals.var_fn205_calc_iq__vdsat10) - (locals.var_fn205_calc_iq__vdsin * locals.var_fn205_calc_iq__vdsat10_dn11)) / (locals.var_fn205_calc_iq__vdsat10 * locals.var_fn205_calc_iq__vdsat10))) * assign17950_e17668) + (assign17950_e17663 * (-(((locals.var_fn205_calc_iq__vdsin_dn11 * locals.var_fn205_calc_iq__vdsat10) - (locals.var_fn205_calc_iq__vdsin * locals.var_fn205_calc_iq__vdsat10_dn11)) / (locals.var_fn205_calc_iq__vdsat10 * locals.var_fn205_calc_iq__vdsat10))))) / (2.0 * assign17950_e17672)))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign17950_e17676, assign17950_e17676_d_n2, assign17950_e17676_d_n4, assign17950_e17676_d_n7, assign17950_e17676_d_n10, assign17950_e17676_d_n11,)
            }
        };
        let assign17950_e17679: f64 = (assign17950_e17677).powf(locals.var_fn205_calc_iq__beta);
        let assign17950_e17680: f64 = (1.0 + assign17950_e17679);
        let assign17950_e17683: f64 = (1.0 / locals.var_fn205_calc_iq__beta);
        let assign17950_e17684: f64 = (assign17950_e17680).powf(assign17950_e17683);
        let assign17950_e17685: f64 = (1.0 / assign17950_e17684);
        (assign17950_e17685, (-(if 0.0 == 0.0 && ((assign17950_e17683) as f64).is_finite() && ((assign17950_e17683) as f64).fract() == 0.0 { if assign17950_e17683 == 0.0 { 0.0 } else { (assign17950_e17683 * ((assign17950_e17680).powf(assign17950_e17683 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn205_calc_iq__beta) as f64).is_finite() && ((locals.var_fn205_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn205_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn205_calc_iq__beta * ((assign17950_e17677).powf(locals.var_fn205_calc_iq__beta - 1.0) * assign17950_e17677_d_n2)) } } else { (assign17950_e17679 * (locals.var_fn205_calc_iq__beta * (assign17950_e17677_d_n2 / assign17950_e17677))) })) } } else { (assign17950_e17684 * (assign17950_e17683 * (if 0.0 == 0.0 && ((locals.var_fn205_calc_iq__beta) as f64).is_finite() && ((locals.var_fn205_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn205_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn205_calc_iq__beta * ((assign17950_e17677).powf(locals.var_fn205_calc_iq__beta - 1.0) * assign17950_e17677_d_n2)) } } else { (assign17950_e17679 * (locals.var_fn205_calc_iq__beta * (assign17950_e17677_d_n2 / assign17950_e17677))) } / assign17950_e17680))) } / (assign17950_e17684 * assign17950_e17684))), (-(if 0.0 == 0.0 && ((assign17950_e17683) as f64).is_finite() && ((assign17950_e17683) as f64).fract() == 0.0 { if assign17950_e17683 == 0.0 { 0.0 } else { (assign17950_e17683 * ((assign17950_e17680).powf(assign17950_e17683 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn205_calc_iq__beta) as f64).is_finite() && ((locals.var_fn205_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn205_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn205_calc_iq__beta * ((assign17950_e17677).powf(locals.var_fn205_calc_iq__beta - 1.0) * assign17950_e17677_d_n4)) } } else { (assign17950_e17679 * (locals.var_fn205_calc_iq__beta * (assign17950_e17677_d_n4 / assign17950_e17677))) })) } } else { (assign17950_e17684 * (assign17950_e17683 * (if 0.0 == 0.0 && ((locals.var_fn205_calc_iq__beta) as f64).is_finite() && ((locals.var_fn205_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn205_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn205_calc_iq__beta * ((assign17950_e17677).powf(locals.var_fn205_calc_iq__beta - 1.0) * assign17950_e17677_d_n4)) } } else { (assign17950_e17679 * (locals.var_fn205_calc_iq__beta * (assign17950_e17677_d_n4 / assign17950_e17677))) } / assign17950_e17680))) } / (assign17950_e17684 * assign17950_e17684))), (-(if 0.0 == 0.0 && ((assign17950_e17683) as f64).is_finite() && ((assign17950_e17683) as f64).fract() == 0.0 { if assign17950_e17683 == 0.0 { 0.0 } else { (assign17950_e17683 * ((assign17950_e17680).powf(assign17950_e17683 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn205_calc_iq__beta) as f64).is_finite() && ((locals.var_fn205_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn205_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn205_calc_iq__beta * ((assign17950_e17677).powf(locals.var_fn205_calc_iq__beta - 1.0) * assign17950_e17677_d_n7)) } } else { (assign17950_e17679 * (locals.var_fn205_calc_iq__beta * (assign17950_e17677_d_n7 / assign17950_e17677))) })) } } else { (assign17950_e17684 * (assign17950_e17683 * (if 0.0 == 0.0 && ((locals.var_fn205_calc_iq__beta) as f64).is_finite() && ((locals.var_fn205_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn205_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn205_calc_iq__beta * ((assign17950_e17677).powf(locals.var_fn205_calc_iq__beta - 1.0) * assign17950_e17677_d_n7)) } } else { (assign17950_e17679 * (locals.var_fn205_calc_iq__beta * (assign17950_e17677_d_n7 / assign17950_e17677))) } / assign17950_e17680))) } / (assign17950_e17684 * assign17950_e17684))), (-(if 0.0 == 0.0 && ((assign17950_e17683) as f64).is_finite() && ((assign17950_e17683) as f64).fract() == 0.0 { if assign17950_e17683 == 0.0 { 0.0 } else { (assign17950_e17683 * ((assign17950_e17680).powf(assign17950_e17683 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn205_calc_iq__beta) as f64).is_finite() && ((locals.var_fn205_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn205_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn205_calc_iq__beta * ((assign17950_e17677).powf(locals.var_fn205_calc_iq__beta - 1.0) * assign17950_e17677_d_n10)) } } else { (assign17950_e17679 * (locals.var_fn205_calc_iq__beta * (assign17950_e17677_d_n10 / assign17950_e17677))) })) } } else { (assign17950_e17684 * (assign17950_e17683 * (if 0.0 == 0.0 && ((locals.var_fn205_calc_iq__beta) as f64).is_finite() && ((locals.var_fn205_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn205_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn205_calc_iq__beta * ((assign17950_e17677).powf(locals.var_fn205_calc_iq__beta - 1.0) * assign17950_e17677_d_n10)) } } else { (assign17950_e17679 * (locals.var_fn205_calc_iq__beta * (assign17950_e17677_d_n10 / assign17950_e17677))) } / assign17950_e17680))) } / (assign17950_e17684 * assign17950_e17684))), (-(if 0.0 == 0.0 && ((assign17950_e17683) as f64).is_finite() && ((assign17950_e17683) as f64).fract() == 0.0 { if assign17950_e17683 == 0.0 { 0.0 } else { (assign17950_e17683 * ((assign17950_e17680).powf(assign17950_e17683 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn205_calc_iq__beta) as f64).is_finite() && ((locals.var_fn205_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn205_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn205_calc_iq__beta * ((assign17950_e17677).powf(locals.var_fn205_calc_iq__beta - 1.0) * assign17950_e17677_d_n11)) } } else { (assign17950_e17679 * (locals.var_fn205_calc_iq__beta * (assign17950_e17677_d_n11 / assign17950_e17677))) })) } } else { (assign17950_e17684 * (assign17950_e17683 * (if 0.0 == 0.0 && ((locals.var_fn205_calc_iq__beta) as f64).is_finite() && ((locals.var_fn205_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn205_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn205_calc_iq__beta * ((assign17950_e17677).powf(locals.var_fn205_calc_iq__beta - 1.0) * assign17950_e17677_d_n11)) } } else { (assign17950_e17679 * (locals.var_fn205_calc_iq__beta * (assign17950_e17677_d_n11 / assign17950_e17677))) } / assign17950_e17680))) } / (assign17950_e17684 * assign17950_e17684))),)
    } else {
        (locals.var_fn205_calc_iq__fsd0, locals.var_fn205_calc_iq__fsd0_dn2, locals.var_fn205_calc_iq__fsd0_dn4, locals.var_fn205_calc_iq__fsd0_dn7, locals.var_fn205_calc_iq__fsd0_dn10, locals.var_fn205_calc_iq__fsd0_dn11,)
    }
};
        locals.var_fn205_calc_iq__fsd0 = assign17950_e17687;
        locals.var_fn205_calc_iq__fsd0_dn2 = assign17950_e17687_d_n2;
        locals.var_fn205_calc_iq__fsd0_dn4 = assign17950_e17687_d_n4;
        locals.var_fn205_calc_iq__fsd0_dn7 = assign17950_e17687_d_n7;
        locals.var_fn205_calc_iq__fsd0_dn10 = assign17950_e17687_d_n10;
        locals.var_fn205_calc_iq__fsd0_dn11 = assign17950_e17687_d_n11;

        let (assign17960_e17693, assign17960_e17693_d_n2, assign17960_e17693_d_n4, assign17960_e17693_d_n7, assign17960_e17693_d_n10, assign17960_e17693_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        let assign17960_e17691: f64 = (locals.var_fn205_calc_iq__vdsin * locals.var_fn205_calc_iq__fsd0);
        (assign17960_e17691, (locals.var_fn205_calc_iq__vdsin * locals.var_fn205_calc_iq__fsd0_dn2), (locals.var_fn205_calc_iq__vdsin * locals.var_fn205_calc_iq__fsd0_dn4), (locals.var_fn205_calc_iq__vdsin * locals.var_fn205_calc_iq__fsd0_dn7), ((locals.var_fn205_calc_iq__vdsin_dn10 * locals.var_fn205_calc_iq__fsd0) + (locals.var_fn205_calc_iq__vdsin * locals.var_fn205_calc_iq__fsd0_dn10)), ((locals.var_fn205_calc_iq__vdsin_dn11 * locals.var_fn205_calc_iq__fsd0) + (locals.var_fn205_calc_iq__vdsin * locals.var_fn205_calc_iq__fsd0_dn11)),)
    } else {
        (locals.var_fn205_calc_iq__vdx0, locals.var_fn205_calc_iq__vdx0_dn2, locals.var_fn205_calc_iq__vdx0_dn4, locals.var_fn205_calc_iq__vdx0_dn7, locals.var_fn205_calc_iq__vdx0_dn10, locals.var_fn205_calc_iq__vdx0_dn11,)
    }
};
        locals.var_fn205_calc_iq__vdx0 = assign17960_e17693;
        locals.var_fn205_calc_iq__vdx0_dn2 = assign17960_e17693_d_n2;
        locals.var_fn205_calc_iq__vdx0_dn4 = assign17960_e17693_d_n4;
        locals.var_fn205_calc_iq__vdx0_dn7 = assign17960_e17693_d_n7;
        locals.var_fn205_calc_iq__vdx0_dn10 = assign17960_e17693_d_n10;
        locals.var_fn205_calc_iq__vdx0_dn11 = assign17960_e17693_d_n11;

        let (assign17970_e17768, assign17970_e17768_d_n2, assign17970_e17768_d_n4, assign17970_e17768_d_n7, assign17970_e17768_d_n10, assign17970_e17768_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        let (assign17970_e17758, assign17970_e17758_d_n2, assign17970_e17758_d_n4, assign17970_e17758_d_n7, assign17970_e17758_d_n10, assign17970_e17758_d_n11,) = {
            if (p.p52 != 0.0) {
                let assign17970_e17704: f64 = (-locals.var_fn205_calc_iq__vdsin);
                let assign17970_e17706: f64 = (assign17970_e17704 / locals.var_fn205_calc_iq__vdsat10);
                let assign17970_e17707: f64 = assign17970_e17706;
                let assign17970_e17710: f64 = (-locals.var_fn205_calc_iq__vdsin);
                let assign17970_e17712: f64 = (assign17970_e17710 / locals.var_fn205_calc_iq__vdsat10);
                let assign17970_e17713: f64 = (-assign17970_e17712);
                let assign17970_e17716: f64 = (0.001 / p.p53);
                let assign17970_e17719: f64 = (-locals.var_fn205_calc_iq__vdsin);
                let assign17970_e17721: f64 = (assign17970_e17719 / locals.var_fn205_calc_iq__vdsat10);
                let assign17970_e17722: f64 = (-assign17970_e17721);
                let assign17970_e17723: f64 = (assign17970_e17716 * assign17970_e17722);
                let assign17970_e17724: f64 = (assign17970_e17723).tanh();
                let assign17970_e17725: f64 = (assign17970_e17713 * assign17970_e17724);
                let assign17970_e17726: f64 = (assign17970_e17707 + assign17970_e17725);
                let assign17970_e17727: f64 = (0.5 * assign17970_e17726);
                (assign17970_e17727, (0.5 * ((-((assign17970_e17704 * locals.var_fn205_calc_iq__vdsat10_dn2) / (locals.var_fn205_calc_iq__vdsat10 * locals.var_fn205_calc_iq__vdsat10))) + (((-(-((assign17970_e17710 * locals.var_fn205_calc_iq__vdsat10_dn2) / (locals.var_fn205_calc_iq__vdsat10 * locals.var_fn205_calc_iq__vdsat10)))) * assign17970_e17724) + (assign17970_e17713 * ((assign17970_e17716 * (-(-((assign17970_e17719 * locals.var_fn205_calc_iq__vdsat10_dn2) / (locals.var_fn205_calc_iq__vdsat10 * locals.var_fn205_calc_iq__vdsat10))))) / ((assign17970_e17723).cosh() * (assign17970_e17723).cosh())))))), (0.5 * ((-((assign17970_e17704 * locals.var_fn205_calc_iq__vdsat10_dn4) / (locals.var_fn205_calc_iq__vdsat10 * locals.var_fn205_calc_iq__vdsat10))) + (((-(-((assign17970_e17710 * locals.var_fn205_calc_iq__vdsat10_dn4) / (locals.var_fn205_calc_iq__vdsat10 * locals.var_fn205_calc_iq__vdsat10)))) * assign17970_e17724) + (assign17970_e17713 * ((assign17970_e17716 * (-(-((assign17970_e17719 * locals.var_fn205_calc_iq__vdsat10_dn4) / (locals.var_fn205_calc_iq__vdsat10 * locals.var_fn205_calc_iq__vdsat10))))) / ((assign17970_e17723).cosh() * (assign17970_e17723).cosh())))))), (0.5 * ((-((assign17970_e17704 * locals.var_fn205_calc_iq__vdsat10_dn7) / (locals.var_fn205_calc_iq__vdsat10 * locals.var_fn205_calc_iq__vdsat10))) + (((-(-((assign17970_e17710 * locals.var_fn205_calc_iq__vdsat10_dn7) / (locals.var_fn205_calc_iq__vdsat10 * locals.var_fn205_calc_iq__vdsat10)))) * assign17970_e17724) + (assign17970_e17713 * ((assign17970_e17716 * (-(-((assign17970_e17719 * locals.var_fn205_calc_iq__vdsat10_dn7) / (locals.var_fn205_calc_iq__vdsat10 * locals.var_fn205_calc_iq__vdsat10))))) / ((assign17970_e17723).cosh() * (assign17970_e17723).cosh())))))), (0.5 * (((((-locals.var_fn205_calc_iq__vdsin_dn10) * locals.var_fn205_calc_iq__vdsat10) - (assign17970_e17704 * locals.var_fn205_calc_iq__vdsat10_dn10)) / (locals.var_fn205_calc_iq__vdsat10 * locals.var_fn205_calc_iq__vdsat10)) + (((-((((-locals.var_fn205_calc_iq__vdsin_dn10) * locals.var_fn205_calc_iq__vdsat10) - (assign17970_e17710 * locals.var_fn205_calc_iq__vdsat10_dn10)) / (locals.var_fn205_calc_iq__vdsat10 * locals.var_fn205_calc_iq__vdsat10))) * assign17970_e17724) + (assign17970_e17713 * ((assign17970_e17716 * (-((((-locals.var_fn205_calc_iq__vdsin_dn10) * locals.var_fn205_calc_iq__vdsat10) - (assign17970_e17719 * locals.var_fn205_calc_iq__vdsat10_dn10)) / (locals.var_fn205_calc_iq__vdsat10 * locals.var_fn205_calc_iq__vdsat10)))) / ((assign17970_e17723).cosh() * (assign17970_e17723).cosh())))))), (0.5 * (((((-locals.var_fn205_calc_iq__vdsin_dn11) * locals.var_fn205_calc_iq__vdsat10) - (assign17970_e17704 * locals.var_fn205_calc_iq__vdsat10_dn11)) / (locals.var_fn205_calc_iq__vdsat10 * locals.var_fn205_calc_iq__vdsat10)) + (((-((((-locals.var_fn205_calc_iq__vdsin_dn11) * locals.var_fn205_calc_iq__vdsat10) - (assign17970_e17710 * locals.var_fn205_calc_iq__vdsat10_dn11)) / (locals.var_fn205_calc_iq__vdsat10 * locals.var_fn205_calc_iq__vdsat10))) * assign17970_e17724) + (assign17970_e17713 * ((assign17970_e17716 * (-((((-locals.var_fn205_calc_iq__vdsin_dn11) * locals.var_fn205_calc_iq__vdsat10) - (assign17970_e17719 * locals.var_fn205_calc_iq__vdsat10_dn11)) / (locals.var_fn205_calc_iq__vdsat10 * locals.var_fn205_calc_iq__vdsat10)))) / ((assign17970_e17723).cosh() * (assign17970_e17723).cosh())))))),)
            } else {
                let (assign17970_e17757, assign17970_e17757_d_n2, assign17970_e17757_d_n4, assign17970_e17757_d_n7, assign17970_e17757_d_n10, assign17970_e17757_d_n11,) = {
                    if (p.p52 == 0.0) {
                        let assign17970_e17734: f64 = (-locals.var_fn205_calc_iq__vdsin);
                        let assign17970_e17736: f64 = (assign17970_e17734 / locals.var_fn205_calc_iq__vdsat10);
                        let assign17970_e17737: f64 = assign17970_e17736;
                        let assign17970_e17740: f64 = (-locals.var_fn205_calc_iq__vdsin);
                        let assign17970_e17742: f64 = (assign17970_e17740 / locals.var_fn205_calc_iq__vdsat10);
                        let assign17970_e17743: f64 = (-assign17970_e17742);
                        let assign17970_e17746: f64 = (-locals.var_fn205_calc_iq__vdsin);
                        let assign17970_e17748: f64 = (assign17970_e17746 / locals.var_fn205_calc_iq__vdsat10);
                        let assign17970_e17749: f64 = (-assign17970_e17748);
                        let assign17970_e17750: f64 = (assign17970_e17743 * assign17970_e17749);
                        let assign17970_e17752: f64 = (assign17970_e17750 + p.p53);
                        let assign17970_e17753: f64 = (assign17970_e17752).sqrt();
                        let assign17970_e17754: f64 = (assign17970_e17737 + assign17970_e17753);
                        let assign17970_e17755: f64 = (0.5 * assign17970_e17754);
                        (assign17970_e17755, (0.5 * ((-((assign17970_e17734 * locals.var_fn205_calc_iq__vdsat10_dn2) / (locals.var_fn205_calc_iq__vdsat10 * locals.var_fn205_calc_iq__vdsat10))) + ((((-(-((assign17970_e17740 * locals.var_fn205_calc_iq__vdsat10_dn2) / (locals.var_fn205_calc_iq__vdsat10 * locals.var_fn205_calc_iq__vdsat10)))) * assign17970_e17749) + (assign17970_e17743 * (-(-((assign17970_e17746 * locals.var_fn205_calc_iq__vdsat10_dn2) / (locals.var_fn205_calc_iq__vdsat10 * locals.var_fn205_calc_iq__vdsat10)))))) / (2.0 * assign17970_e17753)))), (0.5 * ((-((assign17970_e17734 * locals.var_fn205_calc_iq__vdsat10_dn4) / (locals.var_fn205_calc_iq__vdsat10 * locals.var_fn205_calc_iq__vdsat10))) + ((((-(-((assign17970_e17740 * locals.var_fn205_calc_iq__vdsat10_dn4) / (locals.var_fn205_calc_iq__vdsat10 * locals.var_fn205_calc_iq__vdsat10)))) * assign17970_e17749) + (assign17970_e17743 * (-(-((assign17970_e17746 * locals.var_fn205_calc_iq__vdsat10_dn4) / (locals.var_fn205_calc_iq__vdsat10 * locals.var_fn205_calc_iq__vdsat10)))))) / (2.0 * assign17970_e17753)))), (0.5 * ((-((assign17970_e17734 * locals.var_fn205_calc_iq__vdsat10_dn7) / (locals.var_fn205_calc_iq__vdsat10 * locals.var_fn205_calc_iq__vdsat10))) + ((((-(-((assign17970_e17740 * locals.var_fn205_calc_iq__vdsat10_dn7) / (locals.var_fn205_calc_iq__vdsat10 * locals.var_fn205_calc_iq__vdsat10)))) * assign17970_e17749) + (assign17970_e17743 * (-(-((assign17970_e17746 * locals.var_fn205_calc_iq__vdsat10_dn7) / (locals.var_fn205_calc_iq__vdsat10 * locals.var_fn205_calc_iq__vdsat10)))))) / (2.0 * assign17970_e17753)))), (0.5 * (((((-locals.var_fn205_calc_iq__vdsin_dn10) * locals.var_fn205_calc_iq__vdsat10) - (assign17970_e17734 * locals.var_fn205_calc_iq__vdsat10_dn10)) / (locals.var_fn205_calc_iq__vdsat10 * locals.var_fn205_calc_iq__vdsat10)) + ((((-((((-locals.var_fn205_calc_iq__vdsin_dn10) * locals.var_fn205_calc_iq__vdsat10) - (assign17970_e17740 * locals.var_fn205_calc_iq__vdsat10_dn10)) / (locals.var_fn205_calc_iq__vdsat10 * locals.var_fn205_calc_iq__vdsat10))) * assign17970_e17749) + (assign17970_e17743 * (-((((-locals.var_fn205_calc_iq__vdsin_dn10) * locals.var_fn205_calc_iq__vdsat10) - (assign17970_e17746 * locals.var_fn205_calc_iq__vdsat10_dn10)) / (locals.var_fn205_calc_iq__vdsat10 * locals.var_fn205_calc_iq__vdsat10))))) / (2.0 * assign17970_e17753)))), (0.5 * (((((-locals.var_fn205_calc_iq__vdsin_dn11) * locals.var_fn205_calc_iq__vdsat10) - (assign17970_e17734 * locals.var_fn205_calc_iq__vdsat10_dn11)) / (locals.var_fn205_calc_iq__vdsat10 * locals.var_fn205_calc_iq__vdsat10)) + ((((-((((-locals.var_fn205_calc_iq__vdsin_dn11) * locals.var_fn205_calc_iq__vdsat10) - (assign17970_e17740 * locals.var_fn205_calc_iq__vdsat10_dn11)) / (locals.var_fn205_calc_iq__vdsat10 * locals.var_fn205_calc_iq__vdsat10))) * assign17970_e17749) + (assign17970_e17743 * (-((((-locals.var_fn205_calc_iq__vdsin_dn11) * locals.var_fn205_calc_iq__vdsat10) - (assign17970_e17746 * locals.var_fn205_calc_iq__vdsat10_dn11)) / (locals.var_fn205_calc_iq__vdsat10 * locals.var_fn205_calc_iq__vdsat10))))) / (2.0 * assign17970_e17753)))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign17970_e17757, assign17970_e17757_d_n2, assign17970_e17757_d_n4, assign17970_e17757_d_n7, assign17970_e17757_d_n10, assign17970_e17757_d_n11,)
            }
        };
        let assign17970_e17760: f64 = (assign17970_e17758).powf(locals.var_fn205_calc_iq__beta);
        let assign17970_e17761: f64 = (1.0 + assign17970_e17760);
        let assign17970_e17764: f64 = (1.0 / locals.var_fn205_calc_iq__beta);
        let assign17970_e17765: f64 = (assign17970_e17761).powf(assign17970_e17764);
        let assign17970_e17766: f64 = (1.0 / assign17970_e17765);
        (assign17970_e17766, (-(if 0.0 == 0.0 && ((assign17970_e17764) as f64).is_finite() && ((assign17970_e17764) as f64).fract() == 0.0 { if assign17970_e17764 == 0.0 { 0.0 } else { (assign17970_e17764 * ((assign17970_e17761).powf(assign17970_e17764 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn205_calc_iq__beta) as f64).is_finite() && ((locals.var_fn205_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn205_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn205_calc_iq__beta * ((assign17970_e17758).powf(locals.var_fn205_calc_iq__beta - 1.0) * assign17970_e17758_d_n2)) } } else { (assign17970_e17760 * (locals.var_fn205_calc_iq__beta * (assign17970_e17758_d_n2 / assign17970_e17758))) })) } } else { (assign17970_e17765 * (assign17970_e17764 * (if 0.0 == 0.0 && ((locals.var_fn205_calc_iq__beta) as f64).is_finite() && ((locals.var_fn205_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn205_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn205_calc_iq__beta * ((assign17970_e17758).powf(locals.var_fn205_calc_iq__beta - 1.0) * assign17970_e17758_d_n2)) } } else { (assign17970_e17760 * (locals.var_fn205_calc_iq__beta * (assign17970_e17758_d_n2 / assign17970_e17758))) } / assign17970_e17761))) } / (assign17970_e17765 * assign17970_e17765))), (-(if 0.0 == 0.0 && ((assign17970_e17764) as f64).is_finite() && ((assign17970_e17764) as f64).fract() == 0.0 { if assign17970_e17764 == 0.0 { 0.0 } else { (assign17970_e17764 * ((assign17970_e17761).powf(assign17970_e17764 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn205_calc_iq__beta) as f64).is_finite() && ((locals.var_fn205_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn205_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn205_calc_iq__beta * ((assign17970_e17758).powf(locals.var_fn205_calc_iq__beta - 1.0) * assign17970_e17758_d_n4)) } } else { (assign17970_e17760 * (locals.var_fn205_calc_iq__beta * (assign17970_e17758_d_n4 / assign17970_e17758))) })) } } else { (assign17970_e17765 * (assign17970_e17764 * (if 0.0 == 0.0 && ((locals.var_fn205_calc_iq__beta) as f64).is_finite() && ((locals.var_fn205_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn205_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn205_calc_iq__beta * ((assign17970_e17758).powf(locals.var_fn205_calc_iq__beta - 1.0) * assign17970_e17758_d_n4)) } } else { (assign17970_e17760 * (locals.var_fn205_calc_iq__beta * (assign17970_e17758_d_n4 / assign17970_e17758))) } / assign17970_e17761))) } / (assign17970_e17765 * assign17970_e17765))), (-(if 0.0 == 0.0 && ((assign17970_e17764) as f64).is_finite() && ((assign17970_e17764) as f64).fract() == 0.0 { if assign17970_e17764 == 0.0 { 0.0 } else { (assign17970_e17764 * ((assign17970_e17761).powf(assign17970_e17764 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn205_calc_iq__beta) as f64).is_finite() && ((locals.var_fn205_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn205_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn205_calc_iq__beta * ((assign17970_e17758).powf(locals.var_fn205_calc_iq__beta - 1.0) * assign17970_e17758_d_n7)) } } else { (assign17970_e17760 * (locals.var_fn205_calc_iq__beta * (assign17970_e17758_d_n7 / assign17970_e17758))) })) } } else { (assign17970_e17765 * (assign17970_e17764 * (if 0.0 == 0.0 && ((locals.var_fn205_calc_iq__beta) as f64).is_finite() && ((locals.var_fn205_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn205_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn205_calc_iq__beta * ((assign17970_e17758).powf(locals.var_fn205_calc_iq__beta - 1.0) * assign17970_e17758_d_n7)) } } else { (assign17970_e17760 * (locals.var_fn205_calc_iq__beta * (assign17970_e17758_d_n7 / assign17970_e17758))) } / assign17970_e17761))) } / (assign17970_e17765 * assign17970_e17765))), (-(if 0.0 == 0.0 && ((assign17970_e17764) as f64).is_finite() && ((assign17970_e17764) as f64).fract() == 0.0 { if assign17970_e17764 == 0.0 { 0.0 } else { (assign17970_e17764 * ((assign17970_e17761).powf(assign17970_e17764 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn205_calc_iq__beta) as f64).is_finite() && ((locals.var_fn205_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn205_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn205_calc_iq__beta * ((assign17970_e17758).powf(locals.var_fn205_calc_iq__beta - 1.0) * assign17970_e17758_d_n10)) } } else { (assign17970_e17760 * (locals.var_fn205_calc_iq__beta * (assign17970_e17758_d_n10 / assign17970_e17758))) })) } } else { (assign17970_e17765 * (assign17970_e17764 * (if 0.0 == 0.0 && ((locals.var_fn205_calc_iq__beta) as f64).is_finite() && ((locals.var_fn205_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn205_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn205_calc_iq__beta * ((assign17970_e17758).powf(locals.var_fn205_calc_iq__beta - 1.0) * assign17970_e17758_d_n10)) } } else { (assign17970_e17760 * (locals.var_fn205_calc_iq__beta * (assign17970_e17758_d_n10 / assign17970_e17758))) } / assign17970_e17761))) } / (assign17970_e17765 * assign17970_e17765))), (-(if 0.0 == 0.0 && ((assign17970_e17764) as f64).is_finite() && ((assign17970_e17764) as f64).fract() == 0.0 { if assign17970_e17764 == 0.0 { 0.0 } else { (assign17970_e17764 * ((assign17970_e17761).powf(assign17970_e17764 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn205_calc_iq__beta) as f64).is_finite() && ((locals.var_fn205_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn205_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn205_calc_iq__beta * ((assign17970_e17758).powf(locals.var_fn205_calc_iq__beta - 1.0) * assign17970_e17758_d_n11)) } } else { (assign17970_e17760 * (locals.var_fn205_calc_iq__beta * (assign17970_e17758_d_n11 / assign17970_e17758))) })) } } else { (assign17970_e17765 * (assign17970_e17764 * (if 0.0 == 0.0 && ((locals.var_fn205_calc_iq__beta) as f64).is_finite() && ((locals.var_fn205_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn205_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn205_calc_iq__beta * ((assign17970_e17758).powf(locals.var_fn205_calc_iq__beta - 1.0) * assign17970_e17758_d_n11)) } } else { (assign17970_e17760 * (locals.var_fn205_calc_iq__beta * (assign17970_e17758_d_n11 / assign17970_e17758))) } / assign17970_e17761))) } / (assign17970_e17765 * assign17970_e17765))),)
    } else {
        (locals.var_fn205_calc_iq__fds0, locals.var_fn205_calc_iq__fds0_dn2, locals.var_fn205_calc_iq__fds0_dn4, locals.var_fn205_calc_iq__fds0_dn7, locals.var_fn205_calc_iq__fds0_dn10, locals.var_fn205_calc_iq__fds0_dn11,)
    }
};
        locals.var_fn205_calc_iq__fds0 = assign17970_e17768;
        locals.var_fn205_calc_iq__fds0_dn2 = assign17970_e17768_d_n2;
        locals.var_fn205_calc_iq__fds0_dn4 = assign17970_e17768_d_n4;
        locals.var_fn205_calc_iq__fds0_dn7 = assign17970_e17768_d_n7;
        locals.var_fn205_calc_iq__fds0_dn10 = assign17970_e17768_d_n10;
        locals.var_fn205_calc_iq__fds0_dn11 = assign17970_e17768_d_n11;

        let (assign17980_e17775, assign17980_e17775_d_n2, assign17980_e17775_d_n4, assign17980_e17775_d_n7, assign17980_e17775_d_n10, assign17980_e17775_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        let assign17980_e17771: f64 = (-locals.var_fn205_calc_iq__vdsin);
        let assign17980_e17773: f64 = (assign17980_e17771 * locals.var_fn205_calc_iq__fds0);
        (assign17980_e17773, (assign17980_e17771 * locals.var_fn205_calc_iq__fds0_dn2), (assign17980_e17771 * locals.var_fn205_calc_iq__fds0_dn4), (assign17980_e17771 * locals.var_fn205_calc_iq__fds0_dn7), (((-locals.var_fn205_calc_iq__vdsin_dn10) * locals.var_fn205_calc_iq__fds0) + (assign17980_e17771 * locals.var_fn205_calc_iq__fds0_dn10)), (((-locals.var_fn205_calc_iq__vdsin_dn11) * locals.var_fn205_calc_iq__fds0) + (assign17980_e17771 * locals.var_fn205_calc_iq__fds0_dn11)),)
    } else {
        (locals.var_fn205_calc_iq__vsx0, locals.var_fn205_calc_iq__vsx0_dn2, locals.var_fn205_calc_iq__vsx0_dn4, locals.var_fn205_calc_iq__vsx0_dn7, locals.var_fn205_calc_iq__vsx0_dn10, locals.var_fn205_calc_iq__vsx0_dn11,)
    }
};
        locals.var_fn205_calc_iq__vsx0 = assign17980_e17775;
        locals.var_fn205_calc_iq__vsx0_dn2 = assign17980_e17775_d_n2;
        locals.var_fn205_calc_iq__vsx0_dn4 = assign17980_e17775_d_n4;
        locals.var_fn205_calc_iq__vsx0_dn7 = assign17980_e17775_d_n7;
        locals.var_fn205_calc_iq__vsx0_dn10 = assign17980_e17775_d_n10;
        locals.var_fn205_calc_iq__vsx0_dn11 = assign17980_e17775_d_n11;

        let (assign17990_e17783, assign17990_e17783_d_n2, assign17990_e17783_d_n4, assign17990_e17783_d_n7, assign17990_e17783_d_n10, assign17990_e17783_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        let assign17990_e17779: f64 = (locals.var_fn205_calc_iq__vgsin - locals.var_fn205_calc_iq__myarg0);
        let assign17990_e17781: f64 = (assign17990_e17779 / locals.var_fn205_calc_iq__alpha_phit);
        (assign17990_e17781, (locals.var_fn205_calc_iq__vgsin_dn2 / locals.var_fn205_calc_iq__alpha_phit), ((((-locals.var_fn205_calc_iq__myarg0_dn4) * locals.var_fn205_calc_iq__alpha_phit) - (assign17990_e17779 * locals.var_fn205_calc_iq__alpha_phit_dn4)) / (locals.var_fn205_calc_iq__alpha_phit * locals.var_fn205_calc_iq__alpha_phit)), (locals.var_fn205_calc_iq__vgsin_dn7 / locals.var_fn205_calc_iq__alpha_phit), 0.0, (locals.var_fn205_calc_iq__vgsin_dn11 / locals.var_fn205_calc_iq__alpha_phit),)
    } else {
        (locals.var_fn205_calc_iq__exparg0, locals.var_fn205_calc_iq__exparg0_dn2, locals.var_fn205_calc_iq__exparg0_dn4, locals.var_fn205_calc_iq__exparg0_dn7, locals.var_fn205_calc_iq__exparg0_dn10, locals.var_fn205_calc_iq__exparg0_dn11,)
    }
};
        locals.var_fn205_calc_iq__exparg0 = assign17990_e17783;
        locals.var_fn205_calc_iq__exparg0_dn2 = assign17990_e17783_d_n2;
        locals.var_fn205_calc_iq__exparg0_dn4 = assign17990_e17783_d_n4;
        locals.var_fn205_calc_iq__exparg0_dn7 = assign17990_e17783_d_n7;
        locals.var_fn205_calc_iq__exparg0_dn10 = assign17990_e17783_d_n10;
        locals.var_fn205_calc_iq__exparg0_dn11 = assign17990_e17783_d_n11;

        let assign18000_e17786: f64 = if locals.var_fn205_calc_iq__exparg0 > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard223 = assign18000_e17786;

        let (assign18010_e17792, assign18010_e17792_d_n2, assign18010_e17792_d_n4, assign18010_e17792_d_n7, assign18010_e17792_d_n10, assign18010_e17792_d_n11,) = {
    if ((locals.var_guard204 != 0.0) && (locals.var_guard223 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__ffs0, locals.var_fn205_calc_iq__ffs0_dn2, locals.var_fn205_calc_iq__ffs0_dn4, locals.var_fn205_calc_iq__ffs0_dn7, locals.var_fn205_calc_iq__ffs0_dn10, locals.var_fn205_calc_iq__ffs0_dn11,)
    }
};
        locals.var_fn205_calc_iq__ffs0 = assign18010_e17792;
        locals.var_fn205_calc_iq__ffs0_dn2 = assign18010_e17792_d_n2;
        locals.var_fn205_calc_iq__ffs0_dn4 = assign18010_e17792_d_n4;
        locals.var_fn205_calc_iq__ffs0_dn7 = assign18010_e17792_d_n7;
        locals.var_fn205_calc_iq__ffs0_dn10 = assign18010_e17792_d_n10;
        locals.var_fn205_calc_iq__ffs0_dn11 = assign18010_e17792_d_n11;

        let assign18020_e17795: f64 = (-50.0);
        let assign18020_e17796: f64 = if locals.var_fn205_calc_iq__exparg0 < assign18020_e17795 { 1.0 } else { 0.0 };
        locals.var_guard224 = assign18020_e17796;

        let (assign18030_e17805, assign18030_e17805_d_n2, assign18030_e17805_d_n4, assign18030_e17805_d_n7, assign18030_e17805_d_n10, assign18030_e17805_d_n11,) = {
    if (((locals.var_guard204 != 0.0) && (locals.var_guard223 == 0.0)) && (locals.var_guard224 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__ffs0, locals.var_fn205_calc_iq__ffs0_dn2, locals.var_fn205_calc_iq__ffs0_dn4, locals.var_fn205_calc_iq__ffs0_dn7, locals.var_fn205_calc_iq__ffs0_dn10, locals.var_fn205_calc_iq__ffs0_dn11,)
    }
};
        locals.var_fn205_calc_iq__ffs0 = assign18030_e17805;
        locals.var_fn205_calc_iq__ffs0_dn2 = assign18030_e17805_d_n2;
        locals.var_fn205_calc_iq__ffs0_dn4 = assign18030_e17805_d_n4;
        locals.var_fn205_calc_iq__ffs0_dn7 = assign18030_e17805_d_n7;
        locals.var_fn205_calc_iq__ffs0_dn10 = assign18030_e17805_d_n10;
        locals.var_fn205_calc_iq__ffs0_dn11 = assign18030_e17805_d_n11;

        let (assign18040_e17820, assign18040_e17820_d_n2, assign18040_e17820_d_n4, assign18040_e17820_d_n7, assign18040_e17820_d_n10, assign18040_e17820_d_n11,) = {
    if (((locals.var_guard204 != 0.0) && (locals.var_guard223 == 0.0)) && (locals.var_guard224 == 0.0)) {
        let assign18040_e17816: f64 = (locals.var_fn205_calc_iq__exparg0).exp();
        let assign18040_e17817: f64 = (1.0 + assign18040_e17816);
        let assign18040_e17818: f64 = (1.0 / assign18040_e17817);
        (assign18040_e17818, (-((assign18040_e17816 * locals.var_fn205_calc_iq__exparg0_dn2) / (assign18040_e17817 * assign18040_e17817))), (-((assign18040_e17816 * locals.var_fn205_calc_iq__exparg0_dn4) / (assign18040_e17817 * assign18040_e17817))), (-((assign18040_e17816 * locals.var_fn205_calc_iq__exparg0_dn7) / (assign18040_e17817 * assign18040_e17817))), (-((assign18040_e17816 * locals.var_fn205_calc_iq__exparg0_dn10) / (assign18040_e17817 * assign18040_e17817))), (-((assign18040_e17816 * locals.var_fn205_calc_iq__exparg0_dn11) / (assign18040_e17817 * assign18040_e17817))),)
    } else {
        (locals.var_fn205_calc_iq__ffs0, locals.var_fn205_calc_iq__ffs0_dn2, locals.var_fn205_calc_iq__ffs0_dn4, locals.var_fn205_calc_iq__ffs0_dn7, locals.var_fn205_calc_iq__ffs0_dn10, locals.var_fn205_calc_iq__ffs0_dn11,)
    }
};
        locals.var_fn205_calc_iq__ffs0 = assign18040_e17820;
        locals.var_fn205_calc_iq__ffs0_dn2 = assign18040_e17820_d_n2;
        locals.var_fn205_calc_iq__ffs0_dn4 = assign18040_e17820_d_n4;
        locals.var_fn205_calc_iq__ffs0_dn7 = assign18040_e17820_d_n7;
        locals.var_fn205_calc_iq__ffs0_dn10 = assign18040_e17820_d_n10;
        locals.var_fn205_calc_iq__ffs0_dn11 = assign18040_e17820_d_n11;

        let (assign18050_e17838, assign18050_e17838_d_n2, assign18050_e17838_d_n4, assign18050_e17838_d_n7, assign18050_e17838_d_n10, assign18050_e17838_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        let assign18050_e17824: f64 = (locals.var_fn205_calc_iq__vgdin - locals.var_fn205_calc_iq__vsx0);
        let assign18050_e17828: f64 = (p.p51 * 0.1);
        let assign18050_e17830: f64 = (assign18050_e17828 * locals.var_fn205_calc_iq__alpha_phit);
        let assign18050_e17832: f64 = (assign18050_e17830 * locals.var_fn205_calc_iq__ffs0);
        let assign18050_e17833: f64 = (locals.var_fn205_calc_iq__vtof - assign18050_e17832);
        let assign18050_e17834: f64 = (assign18050_e17824 - assign18050_e17833);
        let assign18050_e17836: f64 = (assign18050_e17834 / locals.var_fn205_calc_iq__two_n_phit0);
        (assign18050_e17836, (((locals.var_fn205_calc_iq__vgdin_dn2 - locals.var_fn205_calc_iq__vsx0_dn2) - (-(assign18050_e17830 * locals.var_fn205_calc_iq__ffs0_dn2))) / locals.var_fn205_calc_iq__two_n_phit0), (((((-locals.var_fn205_calc_iq__vsx0_dn4) - (locals.var_fn205_calc_iq__vtof_dn4 - (((assign18050_e17828 * locals.var_fn205_calc_iq__alpha_phit_dn4) * locals.var_fn205_calc_iq__ffs0) + (assign18050_e17830 * locals.var_fn205_calc_iq__ffs0_dn4)))) * locals.var_fn205_calc_iq__two_n_phit0) - (assign18050_e17834 * locals.var_fn205_calc_iq__two_n_phit0_dn4)) / (locals.var_fn205_calc_iq__two_n_phit0 * locals.var_fn205_calc_iq__two_n_phit0)), (((locals.var_fn205_calc_iq__vgdin_dn7 - locals.var_fn205_calc_iq__vsx0_dn7) - (-(assign18050_e17830 * locals.var_fn205_calc_iq__ffs0_dn7))) / locals.var_fn205_calc_iq__two_n_phit0), (((locals.var_fn205_calc_iq__vgdin_dn10 - locals.var_fn205_calc_iq__vsx0_dn10) - (-(assign18050_e17830 * locals.var_fn205_calc_iq__ffs0_dn10))) / locals.var_fn205_calc_iq__two_n_phit0), (((locals.var_fn205_calc_iq__vgdin_dn11 - locals.var_fn205_calc_iq__vsx0_dn11) - (-(assign18050_e17830 * locals.var_fn205_calc_iq__ffs0_dn11))) / locals.var_fn205_calc_iq__two_n_phit0),)
    } else {
        (locals.var_fn205_calc_iq__etas0, locals.var_fn205_calc_iq__etas0_dn2, locals.var_fn205_calc_iq__etas0_dn4, locals.var_fn205_calc_iq__etas0_dn7, locals.var_fn205_calc_iq__etas0_dn10, locals.var_fn205_calc_iq__etas0_dn11,)
    }
};
        locals.var_fn205_calc_iq__etas0 = assign18050_e17838;
        locals.var_fn205_calc_iq__etas0_dn2 = assign18050_e17838_d_n2;
        locals.var_fn205_calc_iq__etas0_dn4 = assign18050_e17838_d_n4;
        locals.var_fn205_calc_iq__etas0_dn7 = assign18050_e17838_d_n7;
        locals.var_fn205_calc_iq__etas0_dn10 = assign18050_e17838_d_n10;
        locals.var_fn205_calc_iq__etas0_dn11 = assign18050_e17838_d_n11;

        let assign18060_e17841: f64 = if locals.var_fn205_calc_iq__etas0 > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard225 = assign18060_e17841;

        let (assign18070_e17849, assign18070_e17849_d_n2, assign18070_e17849_d_n4, assign18070_e17849_d_n7, assign18070_e17849_d_n10, assign18070_e17849_d_n11,) = {
    if ((locals.var_guard204 != 0.0) && (locals.var_guard225 != 0.0)) {
        let assign18070_e17847: f64 = (locals.var_fn205_calc_iq__qref0 * locals.var_fn205_calc_iq__etas0);
        (assign18070_e17847, (locals.var_fn205_calc_iq__qref0 * locals.var_fn205_calc_iq__etas0_dn2), ((locals.var_fn205_calc_iq__qref0_dn4 * locals.var_fn205_calc_iq__etas0) + (locals.var_fn205_calc_iq__qref0 * locals.var_fn205_calc_iq__etas0_dn4)), (locals.var_fn205_calc_iq__qref0 * locals.var_fn205_calc_iq__etas0_dn7), (locals.var_fn205_calc_iq__qref0 * locals.var_fn205_calc_iq__etas0_dn10), (locals.var_fn205_calc_iq__qref0 * locals.var_fn205_calc_iq__etas0_dn11),)
    } else {
        (locals.var_fn205_calc_iq__qinvs0, locals.var_fn205_calc_iq__qinvs0_dn2, locals.var_fn205_calc_iq__qinvs0_dn4, locals.var_fn205_calc_iq__qinvs0_dn7, locals.var_fn205_calc_iq__qinvs0_dn10, locals.var_fn205_calc_iq__qinvs0_dn11,)
    }
};
        locals.var_fn205_calc_iq__qinvs0 = assign18070_e17849;
        locals.var_fn205_calc_iq__qinvs0_dn2 = assign18070_e17849_d_n2;
        locals.var_fn205_calc_iq__qinvs0_dn4 = assign18070_e17849_d_n4;
        locals.var_fn205_calc_iq__qinvs0_dn7 = assign18070_e17849_d_n7;
        locals.var_fn205_calc_iq__qinvs0_dn10 = assign18070_e17849_d_n10;
        locals.var_fn205_calc_iq__qinvs0_dn11 = assign18070_e17849_d_n11;

        let assign18080_e17852: f64 = (-50.0);
        let assign18080_e17853: f64 = if locals.var_fn205_calc_iq__etas0 < assign18080_e17852 { 1.0 } else { 0.0 };
        locals.var_guard226 = assign18080_e17853;

        let (assign18090_e17865, assign18090_e17865_d_n2, assign18090_e17865_d_n4, assign18090_e17865_d_n7, assign18090_e17865_d_n10, assign18090_e17865_d_n11,) = {
    if (((locals.var_guard204 != 0.0) && (locals.var_guard225 == 0.0)) && (locals.var_guard226 != 0.0)) {
        let assign18090_e17862: f64 = (locals.var_fn205_calc_iq__etas0).exp();
        let assign18090_e17863: f64 = (locals.var_fn205_calc_iq__qref0 * assign18090_e17862);
        (assign18090_e17863, (locals.var_fn205_calc_iq__qref0 * (assign18090_e17862 * locals.var_fn205_calc_iq__etas0_dn2)), ((locals.var_fn205_calc_iq__qref0_dn4 * assign18090_e17862) + (locals.var_fn205_calc_iq__qref0 * (assign18090_e17862 * locals.var_fn205_calc_iq__etas0_dn4))), (locals.var_fn205_calc_iq__qref0 * (assign18090_e17862 * locals.var_fn205_calc_iq__etas0_dn7)), (locals.var_fn205_calc_iq__qref0 * (assign18090_e17862 * locals.var_fn205_calc_iq__etas0_dn10)), (locals.var_fn205_calc_iq__qref0 * (assign18090_e17862 * locals.var_fn205_calc_iq__etas0_dn11)),)
    } else {
        (locals.var_fn205_calc_iq__qinvs0, locals.var_fn205_calc_iq__qinvs0_dn2, locals.var_fn205_calc_iq__qinvs0_dn4, locals.var_fn205_calc_iq__qinvs0_dn7, locals.var_fn205_calc_iq__qinvs0_dn10, locals.var_fn205_calc_iq__qinvs0_dn11,)
    }
};
        locals.var_fn205_calc_iq__qinvs0 = assign18090_e17865;
        locals.var_fn205_calc_iq__qinvs0_dn2 = assign18090_e17865_d_n2;
        locals.var_fn205_calc_iq__qinvs0_dn4 = assign18090_e17865_d_n4;
        locals.var_fn205_calc_iq__qinvs0_dn7 = assign18090_e17865_d_n7;
        locals.var_fn205_calc_iq__qinvs0_dn10 = assign18090_e17865_d_n10;
        locals.var_fn205_calc_iq__qinvs0_dn11 = assign18090_e17865_d_n11;

        let (assign18100_e17881, assign18100_e17881_d_n2, assign18100_e17881_d_n4, assign18100_e17881_d_n7, assign18100_e17881_d_n10, assign18100_e17881_d_n11,) = {
    if (((locals.var_guard204 != 0.0) && (locals.var_guard225 == 0.0)) && (locals.var_guard226 == 0.0)) {
        let assign18100_e17876: f64 = (locals.var_fn205_calc_iq__etas0).exp();
        let assign18100_e17877: f64 = (1.0 + assign18100_e17876);
        let assign18100_e17878: f64 = (assign18100_e17877).ln();
        let assign18100_e17879: f64 = (locals.var_fn205_calc_iq__qref0 * assign18100_e17878);
        (assign18100_e17879, (locals.var_fn205_calc_iq__qref0 * ((assign18100_e17876 * locals.var_fn205_calc_iq__etas0_dn2) / assign18100_e17877)), ((locals.var_fn205_calc_iq__qref0_dn4 * assign18100_e17878) + (locals.var_fn205_calc_iq__qref0 * ((assign18100_e17876 * locals.var_fn205_calc_iq__etas0_dn4) / assign18100_e17877))), (locals.var_fn205_calc_iq__qref0 * ((assign18100_e17876 * locals.var_fn205_calc_iq__etas0_dn7) / assign18100_e17877)), (locals.var_fn205_calc_iq__qref0 * ((assign18100_e17876 * locals.var_fn205_calc_iq__etas0_dn10) / assign18100_e17877)), (locals.var_fn205_calc_iq__qref0 * ((assign18100_e17876 * locals.var_fn205_calc_iq__etas0_dn11) / assign18100_e17877)),)
    } else {
        (locals.var_fn205_calc_iq__qinvs0, locals.var_fn205_calc_iq__qinvs0_dn2, locals.var_fn205_calc_iq__qinvs0_dn4, locals.var_fn205_calc_iq__qinvs0_dn7, locals.var_fn205_calc_iq__qinvs0_dn10, locals.var_fn205_calc_iq__qinvs0_dn11,)
    }
};
        locals.var_fn205_calc_iq__qinvs0 = assign18100_e17881;
        locals.var_fn205_calc_iq__qinvs0_dn2 = assign18100_e17881_d_n2;
        locals.var_fn205_calc_iq__qinvs0_dn4 = assign18100_e17881_d_n4;
        locals.var_fn205_calc_iq__qinvs0_dn7 = assign18100_e17881_d_n7;
        locals.var_fn205_calc_iq__qinvs0_dn10 = assign18100_e17881_d_n10;
        locals.var_fn205_calc_iq__qinvs0_dn11 = assign18100_e17881_d_n11;

        let (assign18110_e17889, assign18110_e17889_d_n2, assign18110_e17889_d_n4, assign18110_e17889_d_n7, assign18110_e17889_d_n10, assign18110_e17889_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        let assign18110_e17885: f64 = (locals.var_fn205_calc_iq__vgdin - locals.var_fn205_calc_iq__myarg0);
        let assign18110_e17887: f64 = (assign18110_e17885 / locals.var_fn205_calc_iq__alpha_phit);
        (assign18110_e17887, (locals.var_fn205_calc_iq__vgdin_dn2 / locals.var_fn205_calc_iq__alpha_phit), ((((-locals.var_fn205_calc_iq__myarg0_dn4) * locals.var_fn205_calc_iq__alpha_phit) - (assign18110_e17885 * locals.var_fn205_calc_iq__alpha_phit_dn4)) / (locals.var_fn205_calc_iq__alpha_phit * locals.var_fn205_calc_iq__alpha_phit)), (locals.var_fn205_calc_iq__vgdin_dn7 / locals.var_fn205_calc_iq__alpha_phit), (locals.var_fn205_calc_iq__vgdin_dn10 / locals.var_fn205_calc_iq__alpha_phit), (locals.var_fn205_calc_iq__vgdin_dn11 / locals.var_fn205_calc_iq__alpha_phit),)
    } else {
        (locals.var_fn205_calc_iq__exparg0, locals.var_fn205_calc_iq__exparg0_dn2, locals.var_fn205_calc_iq__exparg0_dn4, locals.var_fn205_calc_iq__exparg0_dn7, locals.var_fn205_calc_iq__exparg0_dn10, locals.var_fn205_calc_iq__exparg0_dn11,)
    }
};
        locals.var_fn205_calc_iq__exparg0 = assign18110_e17889;
        locals.var_fn205_calc_iq__exparg0_dn2 = assign18110_e17889_d_n2;
        locals.var_fn205_calc_iq__exparg0_dn4 = assign18110_e17889_d_n4;
        locals.var_fn205_calc_iq__exparg0_dn7 = assign18110_e17889_d_n7;
        locals.var_fn205_calc_iq__exparg0_dn10 = assign18110_e17889_d_n10;
        locals.var_fn205_calc_iq__exparg0_dn11 = assign18110_e17889_d_n11;

        let assign18120_e17892: f64 = if locals.var_fn205_calc_iq__exparg0 > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard227 = assign18120_e17892;

        let (assign18130_e17898, assign18130_e17898_d_n2, assign18130_e17898_d_n4, assign18130_e17898_d_n7, assign18130_e17898_d_n10, assign18130_e17898_d_n11,) = {
    if ((locals.var_guard204 != 0.0) && (locals.var_guard227 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__ffd0, locals.var_fn205_calc_iq__ffd0_dn2, locals.var_fn205_calc_iq__ffd0_dn4, locals.var_fn205_calc_iq__ffd0_dn7, locals.var_fn205_calc_iq__ffd0_dn10, locals.var_fn205_calc_iq__ffd0_dn11,)
    }
};
        locals.var_fn205_calc_iq__ffd0 = assign18130_e17898;
        locals.var_fn205_calc_iq__ffd0_dn2 = assign18130_e17898_d_n2;
        locals.var_fn205_calc_iq__ffd0_dn4 = assign18130_e17898_d_n4;
        locals.var_fn205_calc_iq__ffd0_dn7 = assign18130_e17898_d_n7;
        locals.var_fn205_calc_iq__ffd0_dn10 = assign18130_e17898_d_n10;
        locals.var_fn205_calc_iq__ffd0_dn11 = assign18130_e17898_d_n11;

        let assign18140_e17901: f64 = (-50.0);
        let assign18140_e17902: f64 = if locals.var_fn205_calc_iq__exparg0 < assign18140_e17901 { 1.0 } else { 0.0 };
        locals.var_guard228 = assign18140_e17902;

        let (assign18150_e17911, assign18150_e17911_d_n2, assign18150_e17911_d_n4, assign18150_e17911_d_n7, assign18150_e17911_d_n10, assign18150_e17911_d_n11,) = {
    if (((locals.var_guard204 != 0.0) && (locals.var_guard227 == 0.0)) && (locals.var_guard228 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__ffd0, locals.var_fn205_calc_iq__ffd0_dn2, locals.var_fn205_calc_iq__ffd0_dn4, locals.var_fn205_calc_iq__ffd0_dn7, locals.var_fn205_calc_iq__ffd0_dn10, locals.var_fn205_calc_iq__ffd0_dn11,)
    }
};
        locals.var_fn205_calc_iq__ffd0 = assign18150_e17911;
        locals.var_fn205_calc_iq__ffd0_dn2 = assign18150_e17911_d_n2;
        locals.var_fn205_calc_iq__ffd0_dn4 = assign18150_e17911_d_n4;
        locals.var_fn205_calc_iq__ffd0_dn7 = assign18150_e17911_d_n7;
        locals.var_fn205_calc_iq__ffd0_dn10 = assign18150_e17911_d_n10;
        locals.var_fn205_calc_iq__ffd0_dn11 = assign18150_e17911_d_n11;

        let (assign18160_e17926, assign18160_e17926_d_n2, assign18160_e17926_d_n4, assign18160_e17926_d_n7, assign18160_e17926_d_n10, assign18160_e17926_d_n11,) = {
    if (((locals.var_guard204 != 0.0) && (locals.var_guard227 == 0.0)) && (locals.var_guard228 == 0.0)) {
        let assign18160_e17922: f64 = (locals.var_fn205_calc_iq__exparg0).exp();
        let assign18160_e17923: f64 = (1.0 + assign18160_e17922);
        let assign18160_e17924: f64 = (1.0 / assign18160_e17923);
        (assign18160_e17924, (-((assign18160_e17922 * locals.var_fn205_calc_iq__exparg0_dn2) / (assign18160_e17923 * assign18160_e17923))), (-((assign18160_e17922 * locals.var_fn205_calc_iq__exparg0_dn4) / (assign18160_e17923 * assign18160_e17923))), (-((assign18160_e17922 * locals.var_fn205_calc_iq__exparg0_dn7) / (assign18160_e17923 * assign18160_e17923))), (-((assign18160_e17922 * locals.var_fn205_calc_iq__exparg0_dn10) / (assign18160_e17923 * assign18160_e17923))), (-((assign18160_e17922 * locals.var_fn205_calc_iq__exparg0_dn11) / (assign18160_e17923 * assign18160_e17923))),)
    } else {
        (locals.var_fn205_calc_iq__ffd0, locals.var_fn205_calc_iq__ffd0_dn2, locals.var_fn205_calc_iq__ffd0_dn4, locals.var_fn205_calc_iq__ffd0_dn7, locals.var_fn205_calc_iq__ffd0_dn10, locals.var_fn205_calc_iq__ffd0_dn11,)
    }
};
        locals.var_fn205_calc_iq__ffd0 = assign18160_e17926;
        locals.var_fn205_calc_iq__ffd0_dn2 = assign18160_e17926_d_n2;
        locals.var_fn205_calc_iq__ffd0_dn4 = assign18160_e17926_d_n4;
        locals.var_fn205_calc_iq__ffd0_dn7 = assign18160_e17926_d_n7;
        locals.var_fn205_calc_iq__ffd0_dn10 = assign18160_e17926_d_n10;
        locals.var_fn205_calc_iq__ffd0_dn11 = assign18160_e17926_d_n11;

        let (assign18170_e17944, assign18170_e17944_d_n2, assign18170_e17944_d_n4, assign18170_e17944_d_n7, assign18170_e17944_d_n10, assign18170_e17944_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        let assign18170_e17930: f64 = (locals.var_fn205_calc_iq__vgsin - locals.var_fn205_calc_iq__vdx0);
        let assign18170_e17934: f64 = (p.p51 * 0.1);
        let assign18170_e17936: f64 = (assign18170_e17934 * locals.var_fn205_calc_iq__alpha_phit);
        let assign18170_e17938: f64 = (assign18170_e17936 * locals.var_fn205_calc_iq__ffd0);
        let assign18170_e17939: f64 = (locals.var_fn205_calc_iq__vtof - assign18170_e17938);
        let assign18170_e17940: f64 = (assign18170_e17930 - assign18170_e17939);
        let assign18170_e17942: f64 = (assign18170_e17940 / locals.var_fn205_calc_iq__two_n_phit0);
        (assign18170_e17942, (((locals.var_fn205_calc_iq__vgsin_dn2 - locals.var_fn205_calc_iq__vdx0_dn2) - (-(assign18170_e17936 * locals.var_fn205_calc_iq__ffd0_dn2))) / locals.var_fn205_calc_iq__two_n_phit0), (((((-locals.var_fn205_calc_iq__vdx0_dn4) - (locals.var_fn205_calc_iq__vtof_dn4 - (((assign18170_e17934 * locals.var_fn205_calc_iq__alpha_phit_dn4) * locals.var_fn205_calc_iq__ffd0) + (assign18170_e17936 * locals.var_fn205_calc_iq__ffd0_dn4)))) * locals.var_fn205_calc_iq__two_n_phit0) - (assign18170_e17940 * locals.var_fn205_calc_iq__two_n_phit0_dn4)) / (locals.var_fn205_calc_iq__two_n_phit0 * locals.var_fn205_calc_iq__two_n_phit0)), (((locals.var_fn205_calc_iq__vgsin_dn7 - locals.var_fn205_calc_iq__vdx0_dn7) - (-(assign18170_e17936 * locals.var_fn205_calc_iq__ffd0_dn7))) / locals.var_fn205_calc_iq__two_n_phit0), (((-locals.var_fn205_calc_iq__vdx0_dn10) - (-(assign18170_e17936 * locals.var_fn205_calc_iq__ffd0_dn10))) / locals.var_fn205_calc_iq__two_n_phit0), (((locals.var_fn205_calc_iq__vgsin_dn11 - locals.var_fn205_calc_iq__vdx0_dn11) - (-(assign18170_e17936 * locals.var_fn205_calc_iq__ffd0_dn11))) / locals.var_fn205_calc_iq__two_n_phit0),)
    } else {
        (locals.var_fn205_calc_iq__etad0, locals.var_fn205_calc_iq__etad0_dn2, locals.var_fn205_calc_iq__etad0_dn4, locals.var_fn205_calc_iq__etad0_dn7, locals.var_fn205_calc_iq__etad0_dn10, locals.var_fn205_calc_iq__etad0_dn11,)
    }
};
        locals.var_fn205_calc_iq__etad0 = assign18170_e17944;
        locals.var_fn205_calc_iq__etad0_dn2 = assign18170_e17944_d_n2;
        locals.var_fn205_calc_iq__etad0_dn4 = assign18170_e17944_d_n4;
        locals.var_fn205_calc_iq__etad0_dn7 = assign18170_e17944_d_n7;
        locals.var_fn205_calc_iq__etad0_dn10 = assign18170_e17944_d_n10;
        locals.var_fn205_calc_iq__etad0_dn11 = assign18170_e17944_d_n11;

        let assign18180_e17947: f64 = if locals.var_fn205_calc_iq__etad0 > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard229 = assign18180_e17947;

        let (assign18190_e17955, assign18190_e17955_d_n2, assign18190_e17955_d_n4, assign18190_e17955_d_n7, assign18190_e17955_d_n10, assign18190_e17955_d_n11,) = {
    if ((locals.var_guard204 != 0.0) && (locals.var_guard229 != 0.0)) {
        let assign18190_e17953: f64 = (locals.var_fn205_calc_iq__qref0 * locals.var_fn205_calc_iq__etad0);
        (assign18190_e17953, (locals.var_fn205_calc_iq__qref0 * locals.var_fn205_calc_iq__etad0_dn2), ((locals.var_fn205_calc_iq__qref0_dn4 * locals.var_fn205_calc_iq__etad0) + (locals.var_fn205_calc_iq__qref0 * locals.var_fn205_calc_iq__etad0_dn4)), (locals.var_fn205_calc_iq__qref0 * locals.var_fn205_calc_iq__etad0_dn7), (locals.var_fn205_calc_iq__qref0 * locals.var_fn205_calc_iq__etad0_dn10), (locals.var_fn205_calc_iq__qref0 * locals.var_fn205_calc_iq__etad0_dn11),)
    } else {
        (locals.var_fn205_calc_iq__qinvd0, locals.var_fn205_calc_iq__qinvd0_dn2, locals.var_fn205_calc_iq__qinvd0_dn4, locals.var_fn205_calc_iq__qinvd0_dn7, locals.var_fn205_calc_iq__qinvd0_dn10, locals.var_fn205_calc_iq__qinvd0_dn11,)
    }
};
        locals.var_fn205_calc_iq__qinvd0 = assign18190_e17955;
        locals.var_fn205_calc_iq__qinvd0_dn2 = assign18190_e17955_d_n2;
        locals.var_fn205_calc_iq__qinvd0_dn4 = assign18190_e17955_d_n4;
        locals.var_fn205_calc_iq__qinvd0_dn7 = assign18190_e17955_d_n7;
        locals.var_fn205_calc_iq__qinvd0_dn10 = assign18190_e17955_d_n10;
        locals.var_fn205_calc_iq__qinvd0_dn11 = assign18190_e17955_d_n11;

        let assign18200_e17958: f64 = (-50.0);
        let assign18200_e17959: f64 = if locals.var_fn205_calc_iq__etad0 < assign18200_e17958 { 1.0 } else { 0.0 };
        locals.var_guard230 = assign18200_e17959;

        let (assign18210_e17971, assign18210_e17971_d_n2, assign18210_e17971_d_n4, assign18210_e17971_d_n7, assign18210_e17971_d_n10, assign18210_e17971_d_n11,) = {
    if (((locals.var_guard204 != 0.0) && (locals.var_guard229 == 0.0)) && (locals.var_guard230 != 0.0)) {
        let assign18210_e17968: f64 = (locals.var_fn205_calc_iq__etad0).exp();
        let assign18210_e17969: f64 = (locals.var_fn205_calc_iq__qref0 * assign18210_e17968);
        (assign18210_e17969, (locals.var_fn205_calc_iq__qref0 * (assign18210_e17968 * locals.var_fn205_calc_iq__etad0_dn2)), ((locals.var_fn205_calc_iq__qref0_dn4 * assign18210_e17968) + (locals.var_fn205_calc_iq__qref0 * (assign18210_e17968 * locals.var_fn205_calc_iq__etad0_dn4))), (locals.var_fn205_calc_iq__qref0 * (assign18210_e17968 * locals.var_fn205_calc_iq__etad0_dn7)), (locals.var_fn205_calc_iq__qref0 * (assign18210_e17968 * locals.var_fn205_calc_iq__etad0_dn10)), (locals.var_fn205_calc_iq__qref0 * (assign18210_e17968 * locals.var_fn205_calc_iq__etad0_dn11)),)
    } else {
        (locals.var_fn205_calc_iq__qinvd0, locals.var_fn205_calc_iq__qinvd0_dn2, locals.var_fn205_calc_iq__qinvd0_dn4, locals.var_fn205_calc_iq__qinvd0_dn7, locals.var_fn205_calc_iq__qinvd0_dn10, locals.var_fn205_calc_iq__qinvd0_dn11,)
    }
};
        locals.var_fn205_calc_iq__qinvd0 = assign18210_e17971;
        locals.var_fn205_calc_iq__qinvd0_dn2 = assign18210_e17971_d_n2;
        locals.var_fn205_calc_iq__qinvd0_dn4 = assign18210_e17971_d_n4;
        locals.var_fn205_calc_iq__qinvd0_dn7 = assign18210_e17971_d_n7;
        locals.var_fn205_calc_iq__qinvd0_dn10 = assign18210_e17971_d_n10;
        locals.var_fn205_calc_iq__qinvd0_dn11 = assign18210_e17971_d_n11;

        let (assign18220_e17987, assign18220_e17987_d_n2, assign18220_e17987_d_n4, assign18220_e17987_d_n7, assign18220_e17987_d_n10, assign18220_e17987_d_n11,) = {
    if (((locals.var_guard204 != 0.0) && (locals.var_guard229 == 0.0)) && (locals.var_guard230 == 0.0)) {
        let assign18220_e17982: f64 = (locals.var_fn205_calc_iq__etad0).exp();
        let assign18220_e17983: f64 = (1.0 + assign18220_e17982);
        let assign18220_e17984: f64 = (assign18220_e17983).ln();
        let assign18220_e17985: f64 = (locals.var_fn205_calc_iq__qref0 * assign18220_e17984);
        (assign18220_e17985, (locals.var_fn205_calc_iq__qref0 * ((assign18220_e17982 * locals.var_fn205_calc_iq__etad0_dn2) / assign18220_e17983)), ((locals.var_fn205_calc_iq__qref0_dn4 * assign18220_e17984) + (locals.var_fn205_calc_iq__qref0 * ((assign18220_e17982 * locals.var_fn205_calc_iq__etad0_dn4) / assign18220_e17983))), (locals.var_fn205_calc_iq__qref0 * ((assign18220_e17982 * locals.var_fn205_calc_iq__etad0_dn7) / assign18220_e17983)), (locals.var_fn205_calc_iq__qref0 * ((assign18220_e17982 * locals.var_fn205_calc_iq__etad0_dn10) / assign18220_e17983)), (locals.var_fn205_calc_iq__qref0 * ((assign18220_e17982 * locals.var_fn205_calc_iq__etad0_dn11) / assign18220_e17983)),)
    } else {
        (locals.var_fn205_calc_iq__qinvd0, locals.var_fn205_calc_iq__qinvd0_dn2, locals.var_fn205_calc_iq__qinvd0_dn4, locals.var_fn205_calc_iq__qinvd0_dn7, locals.var_fn205_calc_iq__qinvd0_dn10, locals.var_fn205_calc_iq__qinvd0_dn11,)
    }
};
        locals.var_fn205_calc_iq__qinvd0 = assign18220_e17987;
        locals.var_fn205_calc_iq__qinvd0_dn2 = assign18220_e17987_d_n2;
        locals.var_fn205_calc_iq__qinvd0_dn4 = assign18220_e17987_d_n4;
        locals.var_fn205_calc_iq__qinvd0_dn7 = assign18220_e17987_d_n7;
        locals.var_fn205_calc_iq__qinvd0_dn10 = assign18220_e17987_d_n10;
        locals.var_fn205_calc_iq__qinvd0_dn11 = assign18220_e17987_d_n11;

        let (assign18230_e17995, assign18230_e17995_d_n2, assign18230_e17995_d_n4, assign18230_e17995_d_n7, assign18230_e17995_d_n10, assign18230_e17995_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        let assign18230_e17991: f64 = (locals.var_fn205_calc_iq__qinvs0 * locals.var_fn205_calc_iq__qinvs0);
        let assign18230_e17993: f64 = (assign18230_e17991 + 1e-38);
        (assign18230_e17993, ((locals.var_fn205_calc_iq__qinvs0_dn2 * locals.var_fn205_calc_iq__qinvs0) + (locals.var_fn205_calc_iq__qinvs0 * locals.var_fn205_calc_iq__qinvs0_dn2)), ((locals.var_fn205_calc_iq__qinvs0_dn4 * locals.var_fn205_calc_iq__qinvs0) + (locals.var_fn205_calc_iq__qinvs0 * locals.var_fn205_calc_iq__qinvs0_dn4)), ((locals.var_fn205_calc_iq__qinvs0_dn7 * locals.var_fn205_calc_iq__qinvs0) + (locals.var_fn205_calc_iq__qinvs0 * locals.var_fn205_calc_iq__qinvs0_dn7)), ((locals.var_fn205_calc_iq__qinvs0_dn10 * locals.var_fn205_calc_iq__qinvs0) + (locals.var_fn205_calc_iq__qinvs0 * locals.var_fn205_calc_iq__qinvs0_dn10)), ((locals.var_fn205_calc_iq__qinvs0_dn11 * locals.var_fn205_calc_iq__qinvs0) + (locals.var_fn205_calc_iq__qinvs0 * locals.var_fn205_calc_iq__qinvs0_dn11)),)
    } else {
        (locals.var_fn205_calc_iq__qs2, locals.var_fn205_calc_iq__qs2_dn2, locals.var_fn205_calc_iq__qs2_dn4, locals.var_fn205_calc_iq__qs2_dn7, locals.var_fn205_calc_iq__qs2_dn10, locals.var_fn205_calc_iq__qs2_dn11,)
    }
};
        locals.var_fn205_calc_iq__qs2 = assign18230_e17995;
        locals.var_fn205_calc_iq__qs2_dn2 = assign18230_e17995_d_n2;
        locals.var_fn205_calc_iq__qs2_dn4 = assign18230_e17995_d_n4;
        locals.var_fn205_calc_iq__qs2_dn7 = assign18230_e17995_d_n7;
        locals.var_fn205_calc_iq__qs2_dn10 = assign18230_e17995_d_n10;
        locals.var_fn205_calc_iq__qs2_dn11 = assign18230_e17995_d_n11;

    }

    pub(super) fn stamp_transient_block_46(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign18240_e18003, assign18240_e18003_d_n2, assign18240_e18003_d_n4, assign18240_e18003_d_n7, assign18240_e18003_d_n10, assign18240_e18003_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        let assign18240_e17999: f64 = (locals.var_fn205_calc_iq__qs2 * locals.var_fn205_calc_iq__qinvs0);
        let assign18240_e18001: f64 = (assign18240_e17999 + 1e-57);
        (assign18240_e18001, ((locals.var_fn205_calc_iq__qs2_dn2 * locals.var_fn205_calc_iq__qinvs0) + (locals.var_fn205_calc_iq__qs2 * locals.var_fn205_calc_iq__qinvs0_dn2)), ((locals.var_fn205_calc_iq__qs2_dn4 * locals.var_fn205_calc_iq__qinvs0) + (locals.var_fn205_calc_iq__qs2 * locals.var_fn205_calc_iq__qinvs0_dn4)), ((locals.var_fn205_calc_iq__qs2_dn7 * locals.var_fn205_calc_iq__qinvs0) + (locals.var_fn205_calc_iq__qs2 * locals.var_fn205_calc_iq__qinvs0_dn7)), ((locals.var_fn205_calc_iq__qs2_dn10 * locals.var_fn205_calc_iq__qinvs0) + (locals.var_fn205_calc_iq__qs2 * locals.var_fn205_calc_iq__qinvs0_dn10)), ((locals.var_fn205_calc_iq__qs2_dn11 * locals.var_fn205_calc_iq__qinvs0) + (locals.var_fn205_calc_iq__qs2 * locals.var_fn205_calc_iq__qinvs0_dn11)),)
    } else {
        (locals.var_fn205_calc_iq__qs3, locals.var_fn205_calc_iq__qs3_dn2, locals.var_fn205_calc_iq__qs3_dn4, locals.var_fn205_calc_iq__qs3_dn7, locals.var_fn205_calc_iq__qs3_dn10, locals.var_fn205_calc_iq__qs3_dn11,)
    }
};
        locals.var_fn205_calc_iq__qs3 = assign18240_e18003;
        locals.var_fn205_calc_iq__qs3_dn2 = assign18240_e18003_d_n2;
        locals.var_fn205_calc_iq__qs3_dn4 = assign18240_e18003_d_n4;
        locals.var_fn205_calc_iq__qs3_dn7 = assign18240_e18003_d_n7;
        locals.var_fn205_calc_iq__qs3_dn10 = assign18240_e18003_d_n10;
        locals.var_fn205_calc_iq__qs3_dn11 = assign18240_e18003_d_n11;

        let (assign18250_e18011, assign18250_e18011_d_n2, assign18250_e18011_d_n4, assign18250_e18011_d_n7, assign18250_e18011_d_n10, assign18250_e18011_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        let assign18250_e18007: f64 = (locals.var_fn205_calc_iq__qinvd0 * locals.var_fn205_calc_iq__qinvd0);
        let assign18250_e18009: f64 = (assign18250_e18007 + 1e-38);
        (assign18250_e18009, ((locals.var_fn205_calc_iq__qinvd0_dn2 * locals.var_fn205_calc_iq__qinvd0) + (locals.var_fn205_calc_iq__qinvd0 * locals.var_fn205_calc_iq__qinvd0_dn2)), ((locals.var_fn205_calc_iq__qinvd0_dn4 * locals.var_fn205_calc_iq__qinvd0) + (locals.var_fn205_calc_iq__qinvd0 * locals.var_fn205_calc_iq__qinvd0_dn4)), ((locals.var_fn205_calc_iq__qinvd0_dn7 * locals.var_fn205_calc_iq__qinvd0) + (locals.var_fn205_calc_iq__qinvd0 * locals.var_fn205_calc_iq__qinvd0_dn7)), ((locals.var_fn205_calc_iq__qinvd0_dn10 * locals.var_fn205_calc_iq__qinvd0) + (locals.var_fn205_calc_iq__qinvd0 * locals.var_fn205_calc_iq__qinvd0_dn10)), ((locals.var_fn205_calc_iq__qinvd0_dn11 * locals.var_fn205_calc_iq__qinvd0) + (locals.var_fn205_calc_iq__qinvd0 * locals.var_fn205_calc_iq__qinvd0_dn11)),)
    } else {
        (locals.var_fn205_calc_iq__qd2, locals.var_fn205_calc_iq__qd2_dn2, locals.var_fn205_calc_iq__qd2_dn4, locals.var_fn205_calc_iq__qd2_dn7, locals.var_fn205_calc_iq__qd2_dn10, locals.var_fn205_calc_iq__qd2_dn11,)
    }
};
        locals.var_fn205_calc_iq__qd2 = assign18250_e18011;
        locals.var_fn205_calc_iq__qd2_dn2 = assign18250_e18011_d_n2;
        locals.var_fn205_calc_iq__qd2_dn4 = assign18250_e18011_d_n4;
        locals.var_fn205_calc_iq__qd2_dn7 = assign18250_e18011_d_n7;
        locals.var_fn205_calc_iq__qd2_dn10 = assign18250_e18011_d_n10;
        locals.var_fn205_calc_iq__qd2_dn11 = assign18250_e18011_d_n11;

        let (assign18260_e18019, assign18260_e18019_d_n2, assign18260_e18019_d_n4, assign18260_e18019_d_n7, assign18260_e18019_d_n10, assign18260_e18019_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        let assign18260_e18015: f64 = (locals.var_fn205_calc_iq__qd2 * locals.var_fn205_calc_iq__qinvd0);
        let assign18260_e18017: f64 = (assign18260_e18015 + 1e-57);
        (assign18260_e18017, ((locals.var_fn205_calc_iq__qd2_dn2 * locals.var_fn205_calc_iq__qinvd0) + (locals.var_fn205_calc_iq__qd2 * locals.var_fn205_calc_iq__qinvd0_dn2)), ((locals.var_fn205_calc_iq__qd2_dn4 * locals.var_fn205_calc_iq__qinvd0) + (locals.var_fn205_calc_iq__qd2 * locals.var_fn205_calc_iq__qinvd0_dn4)), ((locals.var_fn205_calc_iq__qd2_dn7 * locals.var_fn205_calc_iq__qinvd0) + (locals.var_fn205_calc_iq__qd2 * locals.var_fn205_calc_iq__qinvd0_dn7)), ((locals.var_fn205_calc_iq__qd2_dn10 * locals.var_fn205_calc_iq__qinvd0) + (locals.var_fn205_calc_iq__qd2 * locals.var_fn205_calc_iq__qinvd0_dn10)), ((locals.var_fn205_calc_iq__qd2_dn11 * locals.var_fn205_calc_iq__qinvd0) + (locals.var_fn205_calc_iq__qd2 * locals.var_fn205_calc_iq__qinvd0_dn11)),)
    } else {
        (locals.var_fn205_calc_iq__qd3, locals.var_fn205_calc_iq__qd3_dn2, locals.var_fn205_calc_iq__qd3_dn4, locals.var_fn205_calc_iq__qd3_dn7, locals.var_fn205_calc_iq__qd3_dn10, locals.var_fn205_calc_iq__qd3_dn11,)
    }
};
        locals.var_fn205_calc_iq__qd3 = assign18260_e18019;
        locals.var_fn205_calc_iq__qd3_dn2 = assign18260_e18019_d_n2;
        locals.var_fn205_calc_iq__qd3_dn4 = assign18260_e18019_d_n4;
        locals.var_fn205_calc_iq__qd3_dn7 = assign18260_e18019_d_n7;
        locals.var_fn205_calc_iq__qd3_dn10 = assign18260_e18019_d_n10;
        locals.var_fn205_calc_iq__qd3_dn11 = assign18260_e18019_d_n11;

        let (assign18270_e18027, assign18270_e18027_d_n2, assign18270_e18027_d_n4, assign18270_e18027_d_n7, assign18270_e18027_d_n10, assign18270_e18027_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        let assign18270_e18023: f64 = (locals.var_fn205_calc_iq__qinvs0 * locals.var_fn205_calc_iq__qinvd0);
        let assign18270_e18025: f64 = (assign18270_e18023 + 1e-38);
        (assign18270_e18025, ((locals.var_fn205_calc_iq__qinvs0_dn2 * locals.var_fn205_calc_iq__qinvd0) + (locals.var_fn205_calc_iq__qinvs0 * locals.var_fn205_calc_iq__qinvd0_dn2)), ((locals.var_fn205_calc_iq__qinvs0_dn4 * locals.var_fn205_calc_iq__qinvd0) + (locals.var_fn205_calc_iq__qinvs0 * locals.var_fn205_calc_iq__qinvd0_dn4)), ((locals.var_fn205_calc_iq__qinvs0_dn7 * locals.var_fn205_calc_iq__qinvd0) + (locals.var_fn205_calc_iq__qinvs0 * locals.var_fn205_calc_iq__qinvd0_dn7)), ((locals.var_fn205_calc_iq__qinvs0_dn10 * locals.var_fn205_calc_iq__qinvd0) + (locals.var_fn205_calc_iq__qinvs0 * locals.var_fn205_calc_iq__qinvd0_dn10)), ((locals.var_fn205_calc_iq__qinvs0_dn11 * locals.var_fn205_calc_iq__qinvd0) + (locals.var_fn205_calc_iq__qinvs0 * locals.var_fn205_calc_iq__qinvd0_dn11)),)
    } else {
        (locals.var_fn205_calc_iq__qsqd, locals.var_fn205_calc_iq__qsqd_dn2, locals.var_fn205_calc_iq__qsqd_dn4, locals.var_fn205_calc_iq__qsqd_dn7, locals.var_fn205_calc_iq__qsqd_dn10, locals.var_fn205_calc_iq__qsqd_dn11,)
    }
};
        locals.var_fn205_calc_iq__qsqd = assign18270_e18027;
        locals.var_fn205_calc_iq__qsqd_dn2 = assign18270_e18027_d_n2;
        locals.var_fn205_calc_iq__qsqd_dn4 = assign18270_e18027_d_n4;
        locals.var_fn205_calc_iq__qsqd_dn7 = assign18270_e18027_d_n7;
        locals.var_fn205_calc_iq__qsqd_dn10 = assign18270_e18027_d_n10;
        locals.var_fn205_calc_iq__qsqd_dn11 = assign18270_e18027_d_n11;

        let (assign18280_e18045, assign18280_e18045_d_n2, assign18280_e18045_d_n4, assign18280_e18045_d_n7, assign18280_e18045_d_n10, assign18280_e18045_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        let assign18280_e18031: f64 = (2.0 / 3.0);
        let assign18280_e18034: f64 = (locals.var_fn205_calc_iq__qs2 + locals.var_fn205_calc_iq__qd2);
        let assign18280_e18036: f64 = (assign18280_e18034 + locals.var_fn205_calc_iq__qsqd);
        let assign18280_e18037: f64 = (assign18280_e18031 * assign18280_e18036);
        let assign18280_e18040: f64 = (locals.var_fn205_calc_iq__qinvs0 + locals.var_fn205_calc_iq__qinvd0);
        let assign18280_e18042: f64 = (assign18280_e18040 + 2e-19);
        let assign18280_e18043: f64 = (assign18280_e18037 / assign18280_e18042);
        (assign18280_e18043, ((((assign18280_e18031 * ((locals.var_fn205_calc_iq__qs2_dn2 + locals.var_fn205_calc_iq__qd2_dn2) + locals.var_fn205_calc_iq__qsqd_dn2)) * assign18280_e18042) - (assign18280_e18037 * (locals.var_fn205_calc_iq__qinvs0_dn2 + locals.var_fn205_calc_iq__qinvd0_dn2))) / (assign18280_e18042 * assign18280_e18042)), ((((assign18280_e18031 * ((locals.var_fn205_calc_iq__qs2_dn4 + locals.var_fn205_calc_iq__qd2_dn4) + locals.var_fn205_calc_iq__qsqd_dn4)) * assign18280_e18042) - (assign18280_e18037 * (locals.var_fn205_calc_iq__qinvs0_dn4 + locals.var_fn205_calc_iq__qinvd0_dn4))) / (assign18280_e18042 * assign18280_e18042)), ((((assign18280_e18031 * ((locals.var_fn205_calc_iq__qs2_dn7 + locals.var_fn205_calc_iq__qd2_dn7) + locals.var_fn205_calc_iq__qsqd_dn7)) * assign18280_e18042) - (assign18280_e18037 * (locals.var_fn205_calc_iq__qinvs0_dn7 + locals.var_fn205_calc_iq__qinvd0_dn7))) / (assign18280_e18042 * assign18280_e18042)), ((((assign18280_e18031 * ((locals.var_fn205_calc_iq__qs2_dn10 + locals.var_fn205_calc_iq__qd2_dn10) + locals.var_fn205_calc_iq__qsqd_dn10)) * assign18280_e18042) - (assign18280_e18037 * (locals.var_fn205_calc_iq__qinvs0_dn10 + locals.var_fn205_calc_iq__qinvd0_dn10))) / (assign18280_e18042 * assign18280_e18042)), ((((assign18280_e18031 * ((locals.var_fn205_calc_iq__qs2_dn11 + locals.var_fn205_calc_iq__qd2_dn11) + locals.var_fn205_calc_iq__qsqd_dn11)) * assign18280_e18042) - (assign18280_e18037 * (locals.var_fn205_calc_iq__qinvs0_dn11 + locals.var_fn205_calc_iq__qinvd0_dn11))) / (assign18280_e18042 * assign18280_e18042)),)
    } else {
        (locals.var_fn205_calc_iq__qinvdd, locals.var_fn205_calc_iq__qinvdd_dn2, locals.var_fn205_calc_iq__qinvdd_dn4, locals.var_fn205_calc_iq__qinvdd_dn7, locals.var_fn205_calc_iq__qinvdd_dn10, locals.var_fn205_calc_iq__qinvdd_dn11,)
    }
};
        locals.var_fn205_calc_iq__qinvdd = assign18280_e18045;
        locals.var_fn205_calc_iq__qinvdd_dn2 = assign18280_e18045_d_n2;
        locals.var_fn205_calc_iq__qinvdd_dn4 = assign18280_e18045_d_n4;
        locals.var_fn205_calc_iq__qinvdd_dn7 = assign18280_e18045_d_n7;
        locals.var_fn205_calc_iq__qinvdd_dn10 = assign18280_e18045_d_n10;
        locals.var_fn205_calc_iq__qinvdd_dn11 = assign18280_e18045_d_n11;

        let (assign18290_e18079, assign18290_e18079_d_n2, assign18290_e18079_d_n4, assign18290_e18079_d_n7, assign18290_e18079_d_n10, assign18290_e18079_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        let assign18290_e18050: f64 = (2.0 * locals.var_fn205_calc_iq__qs3);
        let assign18290_e18053: f64 = (3.0 * locals.var_fn205_calc_iq__qd3);
        let assign18290_e18054: f64 = (assign18290_e18050 + assign18290_e18053);
        let assign18290_e18057: f64 = (4.0 * locals.var_fn205_calc_iq__qs2);
        let assign18290_e18059: f64 = (assign18290_e18057 * locals.var_fn205_calc_iq__qinvd0);
        let assign18290_e18060: f64 = (assign18290_e18054 + assign18290_e18059);
        let assign18290_e18063: f64 = (6.0 * locals.var_fn205_calc_iq__qd2);
        let assign18290_e18065: f64 = (assign18290_e18063 * locals.var_fn205_calc_iq__qinvs0);
        let assign18290_e18066: f64 = (assign18290_e18060 + assign18290_e18065);
        let assign18290_e18067: f64 = (2.0 * assign18290_e18066);
        let assign18290_e18071: f64 = (locals.var_fn205_calc_iq__qs2 + locals.var_fn205_calc_iq__qd2);
        let assign18290_e18074: f64 = (2.0 * locals.var_fn205_calc_iq__qsqd);
        let assign18290_e18075: f64 = (assign18290_e18071 + assign18290_e18074);
        let assign18290_e18076: f64 = (15.0 * assign18290_e18075);
        let assign18290_e18077: f64 = (assign18290_e18067 / assign18290_e18076);
        (assign18290_e18077, ((((2.0 * ((((2.0 * locals.var_fn205_calc_iq__qs3_dn2) + (3.0 * locals.var_fn205_calc_iq__qd3_dn2)) + (((4.0 * locals.var_fn205_calc_iq__qs2_dn2) * locals.var_fn205_calc_iq__qinvd0) + (assign18290_e18057 * locals.var_fn205_calc_iq__qinvd0_dn2))) + (((6.0 * locals.var_fn205_calc_iq__qd2_dn2) * locals.var_fn205_calc_iq__qinvs0) + (assign18290_e18063 * locals.var_fn205_calc_iq__qinvs0_dn2)))) * assign18290_e18076) - (assign18290_e18067 * (15.0 * ((locals.var_fn205_calc_iq__qs2_dn2 + locals.var_fn205_calc_iq__qd2_dn2) + (2.0 * locals.var_fn205_calc_iq__qsqd_dn2))))) / (assign18290_e18076 * assign18290_e18076)), ((((2.0 * ((((2.0 * locals.var_fn205_calc_iq__qs3_dn4) + (3.0 * locals.var_fn205_calc_iq__qd3_dn4)) + (((4.0 * locals.var_fn205_calc_iq__qs2_dn4) * locals.var_fn205_calc_iq__qinvd0) + (assign18290_e18057 * locals.var_fn205_calc_iq__qinvd0_dn4))) + (((6.0 * locals.var_fn205_calc_iq__qd2_dn4) * locals.var_fn205_calc_iq__qinvs0) + (assign18290_e18063 * locals.var_fn205_calc_iq__qinvs0_dn4)))) * assign18290_e18076) - (assign18290_e18067 * (15.0 * ((locals.var_fn205_calc_iq__qs2_dn4 + locals.var_fn205_calc_iq__qd2_dn4) + (2.0 * locals.var_fn205_calc_iq__qsqd_dn4))))) / (assign18290_e18076 * assign18290_e18076)), ((((2.0 * ((((2.0 * locals.var_fn205_calc_iq__qs3_dn7) + (3.0 * locals.var_fn205_calc_iq__qd3_dn7)) + (((4.0 * locals.var_fn205_calc_iq__qs2_dn7) * locals.var_fn205_calc_iq__qinvd0) + (assign18290_e18057 * locals.var_fn205_calc_iq__qinvd0_dn7))) + (((6.0 * locals.var_fn205_calc_iq__qd2_dn7) * locals.var_fn205_calc_iq__qinvs0) + (assign18290_e18063 * locals.var_fn205_calc_iq__qinvs0_dn7)))) * assign18290_e18076) - (assign18290_e18067 * (15.0 * ((locals.var_fn205_calc_iq__qs2_dn7 + locals.var_fn205_calc_iq__qd2_dn7) + (2.0 * locals.var_fn205_calc_iq__qsqd_dn7))))) / (assign18290_e18076 * assign18290_e18076)), ((((2.0 * ((((2.0 * locals.var_fn205_calc_iq__qs3_dn10) + (3.0 * locals.var_fn205_calc_iq__qd3_dn10)) + (((4.0 * locals.var_fn205_calc_iq__qs2_dn10) * locals.var_fn205_calc_iq__qinvd0) + (assign18290_e18057 * locals.var_fn205_calc_iq__qinvd0_dn10))) + (((6.0 * locals.var_fn205_calc_iq__qd2_dn10) * locals.var_fn205_calc_iq__qinvs0) + (assign18290_e18063 * locals.var_fn205_calc_iq__qinvs0_dn10)))) * assign18290_e18076) - (assign18290_e18067 * (15.0 * ((locals.var_fn205_calc_iq__qs2_dn10 + locals.var_fn205_calc_iq__qd2_dn10) + (2.0 * locals.var_fn205_calc_iq__qsqd_dn10))))) / (assign18290_e18076 * assign18290_e18076)), ((((2.0 * ((((2.0 * locals.var_fn205_calc_iq__qs3_dn11) + (3.0 * locals.var_fn205_calc_iq__qd3_dn11)) + (((4.0 * locals.var_fn205_calc_iq__qs2_dn11) * locals.var_fn205_calc_iq__qinvd0) + (assign18290_e18057 * locals.var_fn205_calc_iq__qinvd0_dn11))) + (((6.0 * locals.var_fn205_calc_iq__qd2_dn11) * locals.var_fn205_calc_iq__qinvs0) + (assign18290_e18063 * locals.var_fn205_calc_iq__qinvs0_dn11)))) * assign18290_e18076) - (assign18290_e18067 * (15.0 * ((locals.var_fn205_calc_iq__qs2_dn11 + locals.var_fn205_calc_iq__qd2_dn11) + (2.0 * locals.var_fn205_calc_iq__qsqd_dn11))))) / (assign18290_e18076 * assign18290_e18076)),)
    } else {
        (locals.var_fn205_calc_iq__qd1, locals.var_fn205_calc_iq__qd1_dn2, locals.var_fn205_calc_iq__qd1_dn4, locals.var_fn205_calc_iq__qd1_dn7, locals.var_fn205_calc_iq__qd1_dn10, locals.var_fn205_calc_iq__qd1_dn11,)
    }
};
        locals.var_fn205_calc_iq__qd1 = assign18290_e18079;
        locals.var_fn205_calc_iq__qd1_dn2 = assign18290_e18079_d_n2;
        locals.var_fn205_calc_iq__qd1_dn4 = assign18290_e18079_d_n4;
        locals.var_fn205_calc_iq__qd1_dn7 = assign18290_e18079_d_n7;
        locals.var_fn205_calc_iq__qd1_dn10 = assign18290_e18079_d_n10;
        locals.var_fn205_calc_iq__qd1_dn11 = assign18290_e18079_d_n11;

        let (assign18300_e18085, assign18300_e18085_d_n2, assign18300_e18085_d_n4, assign18300_e18085_d_n7, assign18300_e18085_d_n10, assign18300_e18085_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        let assign18300_e18083: f64 = (locals.var_fn205_calc_iq__qinvdd - locals.var_fn205_calc_iq__qd1);
        (assign18300_e18083, (locals.var_fn205_calc_iq__qinvdd_dn2 - locals.var_fn205_calc_iq__qd1_dn2), (locals.var_fn205_calc_iq__qinvdd_dn4 - locals.var_fn205_calc_iq__qd1_dn4), (locals.var_fn205_calc_iq__qinvdd_dn7 - locals.var_fn205_calc_iq__qd1_dn7), (locals.var_fn205_calc_iq__qinvdd_dn10 - locals.var_fn205_calc_iq__qd1_dn10), (locals.var_fn205_calc_iq__qinvdd_dn11 - locals.var_fn205_calc_iq__qd1_dn11),)
    } else {
        (locals.var_fn205_calc_iq__qs, locals.var_fn205_calc_iq__qs_dn2, locals.var_fn205_calc_iq__qs_dn4, locals.var_fn205_calc_iq__qs_dn7, locals.var_fn205_calc_iq__qs_dn10, locals.var_fn205_calc_iq__qs_dn11,)
    }
};
        locals.var_fn205_calc_iq__qs = assign18300_e18085;
        locals.var_fn205_calc_iq__qs_dn2 = assign18300_e18085_d_n2;
        locals.var_fn205_calc_iq__qs_dn4 = assign18300_e18085_d_n4;
        locals.var_fn205_calc_iq__qs_dn7 = assign18300_e18085_d_n7;
        locals.var_fn205_calc_iq__qs_dn10 = assign18300_e18085_d_n10;
        locals.var_fn205_calc_iq__qs_dn11 = assign18300_e18085_d_n11;

        let (assign18310_e18089, assign18310_e18089_d_n2, assign18310_e18089_d_n4, assign18310_e18089_d_n7, assign18310_e18089_d_n10, assign18310_e18089_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        (locals.var_fn205_calc_iq__qd1, locals.var_fn205_calc_iq__qd1_dn2, locals.var_fn205_calc_iq__qd1_dn4, locals.var_fn205_calc_iq__qd1_dn7, locals.var_fn205_calc_iq__qd1_dn10, locals.var_fn205_calc_iq__qd1_dn11,)
    } else {
        (locals.var_fn205_calc_iq__qd, locals.var_fn205_calc_iq__qd_dn2, locals.var_fn205_calc_iq__qd_dn4, locals.var_fn205_calc_iq__qd_dn7, locals.var_fn205_calc_iq__qd_dn10, locals.var_fn205_calc_iq__qd_dn11,)
    }
};
        locals.var_fn205_calc_iq__qd = assign18310_e18089;
        locals.var_fn205_calc_iq__qd_dn2 = assign18310_e18089_d_n2;
        locals.var_fn205_calc_iq__qd_dn4 = assign18310_e18089_d_n4;
        locals.var_fn205_calc_iq__qd_dn7 = assign18310_e18089_d_n7;
        locals.var_fn205_calc_iq__qd_dn10 = assign18310_e18089_d_n10;
        locals.var_fn205_calc_iq__qd_dn11 = assign18310_e18089_d_n11;

        let (assign18320_e18103, assign18320_e18103_d_n2, assign18320_e18103_d_n4, assign18320_e18103_d_n7, assign18320_e18103_d_n10, assign18320_e18103_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        let assign18320_e18093: f64 = (locals.var_fn205_calc_iq__w * locals.var_fn205_calc_iq__ngf);
        let assign18320_e18095: f64 = (assign18320_e18093 * locals.var_fn205_calc_iq__lin);
        let assign18320_e18097: f64 = (assign18320_e18095 * locals.var_fn205_calc_iq__type);
        let assign18320_e18099: f64 = (assign18320_e18097 * locals.var_fn205_calc_iq__qs);
        let assign18320_e18101: f64 = (assign18320_e18099 * locals.var_fn205_calc_iq__trapfracdl);
        (assign18320_e18101, ((assign18320_e18097 * locals.var_fn205_calc_iq__qs_dn2) * locals.var_fn205_calc_iq__trapfracdl), ((assign18320_e18097 * locals.var_fn205_calc_iq__qs_dn4) * locals.var_fn205_calc_iq__trapfracdl), ((assign18320_e18097 * locals.var_fn205_calc_iq__qs_dn7) * locals.var_fn205_calc_iq__trapfracdl), ((assign18320_e18097 * locals.var_fn205_calc_iq__qs_dn10) * locals.var_fn205_calc_iq__trapfracdl), ((assign18320_e18097 * locals.var_fn205_calc_iq__qs_dn11) * locals.var_fn205_calc_iq__trapfracdl),)
    } else {
        (locals.var_fn205_calc_iq__qgsout, locals.var_fn205_calc_iq__qgsout_dn2, locals.var_fn205_calc_iq__qgsout_dn4, locals.var_fn205_calc_iq__qgsout_dn7, locals.var_fn205_calc_iq__qgsout_dn10, locals.var_fn205_calc_iq__qgsout_dn11,)
    }
};
        locals.var_fn205_calc_iq__qgsout = assign18320_e18103;
        locals.var_fn205_calc_iq__qgsout_dn2 = assign18320_e18103_d_n2;
        locals.var_fn205_calc_iq__qgsout_dn4 = assign18320_e18103_d_n4;
        locals.var_fn205_calc_iq__qgsout_dn7 = assign18320_e18103_d_n7;
        locals.var_fn205_calc_iq__qgsout_dn10 = assign18320_e18103_d_n10;
        locals.var_fn205_calc_iq__qgsout_dn11 = assign18320_e18103_d_n11;

        let (assign18330_e18117, assign18330_e18117_d_n2, assign18330_e18117_d_n4, assign18330_e18117_d_n7, assign18330_e18117_d_n10, assign18330_e18117_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        let assign18330_e18107: f64 = (locals.var_fn205_calc_iq__w * locals.var_fn205_calc_iq__ngf);
        let assign18330_e18109: f64 = (assign18330_e18107 * locals.var_fn205_calc_iq__lin);
        let assign18330_e18111: f64 = (assign18330_e18109 * locals.var_fn205_calc_iq__type);
        let assign18330_e18113: f64 = (assign18330_e18111 * locals.var_fn205_calc_iq__qd);
        let assign18330_e18115: f64 = (assign18330_e18113 * locals.var_fn205_calc_iq__trapfracdl);
        (assign18330_e18115, ((assign18330_e18111 * locals.var_fn205_calc_iq__qd_dn2) * locals.var_fn205_calc_iq__trapfracdl), ((assign18330_e18111 * locals.var_fn205_calc_iq__qd_dn4) * locals.var_fn205_calc_iq__trapfracdl), ((assign18330_e18111 * locals.var_fn205_calc_iq__qd_dn7) * locals.var_fn205_calc_iq__trapfracdl), ((assign18330_e18111 * locals.var_fn205_calc_iq__qd_dn10) * locals.var_fn205_calc_iq__trapfracdl), ((assign18330_e18111 * locals.var_fn205_calc_iq__qd_dn11) * locals.var_fn205_calc_iq__trapfracdl),)
    } else {
        (locals.var_fn205_calc_iq__qgdout, locals.var_fn205_calc_iq__qgdout_dn2, locals.var_fn205_calc_iq__qgdout_dn4, locals.var_fn205_calc_iq__qgdout_dn7, locals.var_fn205_calc_iq__qgdout_dn10, locals.var_fn205_calc_iq__qgdout_dn11,)
    }
};
        locals.var_fn205_calc_iq__qgdout = assign18330_e18117;
        locals.var_fn205_calc_iq__qgdout_dn2 = assign18330_e18117_d_n2;
        locals.var_fn205_calc_iq__qgdout_dn4 = assign18330_e18117_d_n4;
        locals.var_fn205_calc_iq__qgdout_dn7 = assign18330_e18117_d_n7;
        locals.var_fn205_calc_iq__qgdout_dn10 = assign18330_e18117_d_n10;
        locals.var_fn205_calc_iq__qgdout_dn11 = assign18330_e18117_d_n11;

        let assign18340_e18120: f64 = if locals.var_fn205_calc_iq__qcbflag == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard231 = assign18340_e18120;

        let (assign18350_e18136, assign18350_e18136_d_n2, assign18350_e18136_d_n4, assign18350_e18136_d_n7, assign18350_e18136_d_n11,) = {
    if ((locals.var_guard204 != 0.0) && (locals.var_guard231 != 0.0)) {
        let assign18350_e18128: f64 = (p.p51 * 0.5);
        let assign18350_e18130: f64 = (assign18350_e18128 * locals.var_fn205_calc_iq__alpha_phit);
        let assign18350_e18131: f64 = (locals.var_fn205_calc_iq__vtof - assign18350_e18130);
        let assign18350_e18132: f64 = (locals.var_fn205_calc_iq__vcin - assign18350_e18131);
        let assign18350_e18134: f64 = (assign18350_e18132 / locals.var_fn205_calc_iq__two_n_phit0);
        (assign18350_e18134, (locals.var_fn205_calc_iq__vcin_dn2 / locals.var_fn205_calc_iq__two_n_phit0), ((((-(locals.var_fn205_calc_iq__vtof_dn4 - (assign18350_e18128 * locals.var_fn205_calc_iq__alpha_phit_dn4))) * locals.var_fn205_calc_iq__two_n_phit0) - (assign18350_e18132 * locals.var_fn205_calc_iq__two_n_phit0_dn4)) / (locals.var_fn205_calc_iq__two_n_phit0 * locals.var_fn205_calc_iq__two_n_phit0)), (locals.var_fn205_calc_iq__vcin_dn7 / locals.var_fn205_calc_iq__two_n_phit0), (locals.var_fn205_calc_iq__vcin_dn11 / locals.var_fn205_calc_iq__two_n_phit0),)
    } else {
        (locals.var_fn205_calc_iq__etac, locals.var_fn205_calc_iq__etac_dn2, locals.var_fn205_calc_iq__etac_dn4, locals.var_fn205_calc_iq__etac_dn7, locals.var_fn205_calc_iq__etac_dn11,)
    }
};
        locals.var_fn205_calc_iq__etac = assign18350_e18136;
        locals.var_fn205_calc_iq__etac_dn2 = assign18350_e18136_d_n2;
        locals.var_fn205_calc_iq__etac_dn4 = assign18350_e18136_d_n4;
        locals.var_fn205_calc_iq__etac_dn7 = assign18350_e18136_d_n7;
        locals.var_fn205_calc_iq__etac_dn11 = assign18350_e18136_d_n11;

        let assign18360_e18139: f64 = if locals.var_fn205_calc_iq__etac > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard232 = assign18360_e18139;

        let (assign18370_e18147, assign18370_e18147_d_n2, assign18370_e18147_d_n3, assign18370_e18147_d_n4, assign18370_e18147_d_n7, assign18370_e18147_d_n10, assign18370_e18147_d_n11,) = {
    if (((locals.var_guard204 != 0.0) && (locals.var_guard231 != 0.0)) && (locals.var_guard232 != 0.0)) {
        (locals.var_fn205_calc_iq__etac, locals.var_fn205_calc_iq__etac_dn2, 0.0, locals.var_fn205_calc_iq__etac_dn4, locals.var_fn205_calc_iq__etac_dn7, 0.0, locals.var_fn205_calc_iq__etac_dn11,)
    } else {
        (locals.var_fn205_calc_iq__exparg, locals.var_fn205_calc_iq__exparg_dn2, locals.var_fn205_calc_iq__exparg_dn3, locals.var_fn205_calc_iq__exparg_dn4, locals.var_fn205_calc_iq__exparg_dn7, locals.var_fn205_calc_iq__exparg_dn10, locals.var_fn205_calc_iq__exparg_dn11,)
    }
};
        locals.var_fn205_calc_iq__exparg = assign18370_e18147;
        locals.var_fn205_calc_iq__exparg_dn2 = assign18370_e18147_d_n2;
        locals.var_fn205_calc_iq__exparg_dn3 = assign18370_e18147_d_n3;
        locals.var_fn205_calc_iq__exparg_dn4 = assign18370_e18147_d_n4;
        locals.var_fn205_calc_iq__exparg_dn7 = assign18370_e18147_d_n7;
        locals.var_fn205_calc_iq__exparg_dn10 = assign18370_e18147_d_n10;
        locals.var_fn205_calc_iq__exparg_dn11 = assign18370_e18147_d_n11;

        let assign18380_e18150: f64 = (-50.0);
        let assign18380_e18151: f64 = if locals.var_fn205_calc_iq__etac < assign18380_e18150 { 1.0 } else { 0.0 };
        locals.var_guard233 = assign18380_e18151;

        let (assign18390_e18163, assign18390_e18163_d_n2, assign18390_e18163_d_n3, assign18390_e18163_d_n4, assign18390_e18163_d_n7, assign18390_e18163_d_n10, assign18390_e18163_d_n11,) = {
    if ((((locals.var_guard204 != 0.0) && (locals.var_guard231 != 0.0)) && (locals.var_guard232 == 0.0)) && (locals.var_guard233 != 0.0)) {
        let assign18390_e18161: f64 = (locals.var_fn205_calc_iq__etac).exp();
        (assign18390_e18161, (assign18390_e18161 * locals.var_fn205_calc_iq__etac_dn2), 0.0, (assign18390_e18161 * locals.var_fn205_calc_iq__etac_dn4), (assign18390_e18161 * locals.var_fn205_calc_iq__etac_dn7), 0.0, (assign18390_e18161 * locals.var_fn205_calc_iq__etac_dn11),)
    } else {
        (locals.var_fn205_calc_iq__exparg, locals.var_fn205_calc_iq__exparg_dn2, locals.var_fn205_calc_iq__exparg_dn3, locals.var_fn205_calc_iq__exparg_dn4, locals.var_fn205_calc_iq__exparg_dn7, locals.var_fn205_calc_iq__exparg_dn10, locals.var_fn205_calc_iq__exparg_dn11,)
    }
};
        locals.var_fn205_calc_iq__exparg = assign18390_e18163;
        locals.var_fn205_calc_iq__exparg_dn2 = assign18390_e18163_d_n2;
        locals.var_fn205_calc_iq__exparg_dn3 = assign18390_e18163_d_n3;
        locals.var_fn205_calc_iq__exparg_dn4 = assign18390_e18163_d_n4;
        locals.var_fn205_calc_iq__exparg_dn7 = assign18390_e18163_d_n7;
        locals.var_fn205_calc_iq__exparg_dn10 = assign18390_e18163_d_n10;
        locals.var_fn205_calc_iq__exparg_dn11 = assign18390_e18163_d_n11;

        let (assign18400_e18179, assign18400_e18179_d_n2, assign18400_e18179_d_n3, assign18400_e18179_d_n4, assign18400_e18179_d_n7, assign18400_e18179_d_n10, assign18400_e18179_d_n11,) = {
    if ((((locals.var_guard204 != 0.0) && (locals.var_guard231 != 0.0)) && (locals.var_guard232 == 0.0)) && (locals.var_guard233 == 0.0)) {
        let assign18400_e18175: f64 = (locals.var_fn205_calc_iq__etac).exp();
        let assign18400_e18176: f64 = (1.0 + assign18400_e18175);
        let assign18400_e18177: f64 = (assign18400_e18176).ln();
        (assign18400_e18177, ((assign18400_e18175 * locals.var_fn205_calc_iq__etac_dn2) / assign18400_e18176), 0.0, ((assign18400_e18175 * locals.var_fn205_calc_iq__etac_dn4) / assign18400_e18176), ((assign18400_e18175 * locals.var_fn205_calc_iq__etac_dn7) / assign18400_e18176), 0.0, ((assign18400_e18175 * locals.var_fn205_calc_iq__etac_dn11) / assign18400_e18176),)
    } else {
        (locals.var_fn205_calc_iq__exparg, locals.var_fn205_calc_iq__exparg_dn2, locals.var_fn205_calc_iq__exparg_dn3, locals.var_fn205_calc_iq__exparg_dn4, locals.var_fn205_calc_iq__exparg_dn7, locals.var_fn205_calc_iq__exparg_dn10, locals.var_fn205_calc_iq__exparg_dn11,)
    }
};
        locals.var_fn205_calc_iq__exparg = assign18400_e18179;
        locals.var_fn205_calc_iq__exparg_dn2 = assign18400_e18179_d_n2;
        locals.var_fn205_calc_iq__exparg_dn3 = assign18400_e18179_d_n3;
        locals.var_fn205_calc_iq__exparg_dn4 = assign18400_e18179_d_n4;
        locals.var_fn205_calc_iq__exparg_dn7 = assign18400_e18179_d_n7;
        locals.var_fn205_calc_iq__exparg_dn10 = assign18400_e18179_d_n10;
        locals.var_fn205_calc_iq__exparg_dn11 = assign18400_e18179_d_n11;

        let (assign18410_e18197, assign18410_e18197_d_n2, assign18410_e18197_d_n3, assign18410_e18197_d_n4, assign18410_e18197_d_n7, assign18410_e18197_d_n10, assign18410_e18197_d_n11,) = {
    if ((locals.var_guard204 != 0.0) && (locals.var_guard231 != 0.0)) {
        let assign18410_e18185: f64 = (locals.var_fn205_calc_iq__w * locals.var_fn205_calc_iq__ngf);
        let assign18410_e18187: f64 = (assign18410_e18185 * locals.var_fn205_calc_iq__type);
        let assign18410_e18189: f64 = (assign18410_e18187 * locals.var_fn205_calc_iq__cc);
        let assign18410_e18191: f64 = (assign18410_e18189 * locals.var_fn205_calc_iq__two_n_phit0);
        let assign18410_e18193: f64 = (assign18410_e18191 * locals.var_fn205_calc_iq__exparg);
        let assign18410_e18195: f64 = (assign18410_e18193 * locals.var_fn205_calc_iq__trapfracdl);
        (assign18410_e18195, ((assign18410_e18191 * locals.var_fn205_calc_iq__exparg_dn2) * locals.var_fn205_calc_iq__trapfracdl), ((assign18410_e18191 * locals.var_fn205_calc_iq__exparg_dn3) * locals.var_fn205_calc_iq__trapfracdl), ((((((assign18410_e18187 * locals.var_fn205_calc_iq__cc_dn4) * locals.var_fn205_calc_iq__two_n_phit0) + (assign18410_e18189 * locals.var_fn205_calc_iq__two_n_phit0_dn4)) * locals.var_fn205_calc_iq__exparg) + (assign18410_e18191 * locals.var_fn205_calc_iq__exparg_dn4)) * locals.var_fn205_calc_iq__trapfracdl), ((assign18410_e18191 * locals.var_fn205_calc_iq__exparg_dn7) * locals.var_fn205_calc_iq__trapfracdl), ((assign18410_e18191 * locals.var_fn205_calc_iq__exparg_dn10) * locals.var_fn205_calc_iq__trapfracdl), ((assign18410_e18191 * locals.var_fn205_calc_iq__exparg_dn11) * locals.var_fn205_calc_iq__trapfracdl),)
    } else {
        (locals.var_fn205_calc_iq__qcout, locals.var_fn205_calc_iq__qcout_dn2, locals.var_fn205_calc_iq__qcout_dn3, locals.var_fn205_calc_iq__qcout_dn4, locals.var_fn205_calc_iq__qcout_dn7, locals.var_fn205_calc_iq__qcout_dn10, locals.var_fn205_calc_iq__qcout_dn11,)
    }
};
        locals.var_fn205_calc_iq__qcout = assign18410_e18197;
        locals.var_fn205_calc_iq__qcout_dn2 = assign18410_e18197_d_n2;
        locals.var_fn205_calc_iq__qcout_dn3 = assign18410_e18197_d_n3;
        locals.var_fn205_calc_iq__qcout_dn4 = assign18410_e18197_d_n4;
        locals.var_fn205_calc_iq__qcout_dn7 = assign18410_e18197_d_n7;
        locals.var_fn205_calc_iq__qcout_dn10 = assign18410_e18197_d_n10;
        locals.var_fn205_calc_iq__qcout_dn11 = assign18410_e18197_d_n11;

        let (assign18420_e18213, assign18420_e18213_d_n3, assign18420_e18213_d_n4, assign18420_e18213_d_n11,) = {
    if ((locals.var_guard204 != 0.0) && (locals.var_guard231 != 0.0)) {
        let assign18420_e18205: f64 = (p.p51 * 0.5);
        let assign18420_e18207: f64 = (assign18420_e18205 * locals.var_fn205_calc_iq__alpha_phit);
        let assign18420_e18208: f64 = (locals.var_fn205_calc_iq__vtof - assign18420_e18207);
        let assign18420_e18209: f64 = (locals.var_fn205_calc_iq__vbin - assign18420_e18208);
        let assign18420_e18211: f64 = (assign18420_e18209 / locals.var_fn205_calc_iq__two_n_phit0);
        (assign18420_e18211, (locals.var_fn205_calc_iq__vbin_dn3 / locals.var_fn205_calc_iq__two_n_phit0), ((((-(locals.var_fn205_calc_iq__vtof_dn4 - (assign18420_e18205 * locals.var_fn205_calc_iq__alpha_phit_dn4))) * locals.var_fn205_calc_iq__two_n_phit0) - (assign18420_e18209 * locals.var_fn205_calc_iq__two_n_phit0_dn4)) / (locals.var_fn205_calc_iq__two_n_phit0 * locals.var_fn205_calc_iq__two_n_phit0)), (locals.var_fn205_calc_iq__vbin_dn11 / locals.var_fn205_calc_iq__two_n_phit0),)
    } else {
        (locals.var_fn205_calc_iq__etab, locals.var_fn205_calc_iq__etab_dn3, locals.var_fn205_calc_iq__etab_dn4, locals.var_fn205_calc_iq__etab_dn11,)
    }
};
        locals.var_fn205_calc_iq__etab = assign18420_e18213;
        locals.var_fn205_calc_iq__etab_dn3 = assign18420_e18213_d_n3;
        locals.var_fn205_calc_iq__etab_dn4 = assign18420_e18213_d_n4;
        locals.var_fn205_calc_iq__etab_dn11 = assign18420_e18213_d_n11;

        let assign18430_e18216: f64 = if locals.var_fn205_calc_iq__etab > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard234 = assign18430_e18216;

        let (assign18440_e18224, assign18440_e18224_d_n2, assign18440_e18224_d_n3, assign18440_e18224_d_n4, assign18440_e18224_d_n7, assign18440_e18224_d_n10, assign18440_e18224_d_n11,) = {
    if (((locals.var_guard204 != 0.0) && (locals.var_guard231 != 0.0)) && (locals.var_guard234 != 0.0)) {
        (locals.var_fn205_calc_iq__etab, 0.0, locals.var_fn205_calc_iq__etab_dn3, locals.var_fn205_calc_iq__etab_dn4, 0.0, 0.0, locals.var_fn205_calc_iq__etab_dn11,)
    } else {
        (locals.var_fn205_calc_iq__exparg, locals.var_fn205_calc_iq__exparg_dn2, locals.var_fn205_calc_iq__exparg_dn3, locals.var_fn205_calc_iq__exparg_dn4, locals.var_fn205_calc_iq__exparg_dn7, locals.var_fn205_calc_iq__exparg_dn10, locals.var_fn205_calc_iq__exparg_dn11,)
    }
};
        locals.var_fn205_calc_iq__exparg = assign18440_e18224;
        locals.var_fn205_calc_iq__exparg_dn2 = assign18440_e18224_d_n2;
        locals.var_fn205_calc_iq__exparg_dn3 = assign18440_e18224_d_n3;
        locals.var_fn205_calc_iq__exparg_dn4 = assign18440_e18224_d_n4;
        locals.var_fn205_calc_iq__exparg_dn7 = assign18440_e18224_d_n7;
        locals.var_fn205_calc_iq__exparg_dn10 = assign18440_e18224_d_n10;
        locals.var_fn205_calc_iq__exparg_dn11 = assign18440_e18224_d_n11;

        let assign18450_e18227: f64 = (-50.0);
        let assign18450_e18228: f64 = if locals.var_fn205_calc_iq__etab < assign18450_e18227 { 1.0 } else { 0.0 };
        locals.var_guard235 = assign18450_e18228;

        let (assign18460_e18240, assign18460_e18240_d_n2, assign18460_e18240_d_n3, assign18460_e18240_d_n4, assign18460_e18240_d_n7, assign18460_e18240_d_n10, assign18460_e18240_d_n11,) = {
    if ((((locals.var_guard204 != 0.0) && (locals.var_guard231 != 0.0)) && (locals.var_guard234 == 0.0)) && (locals.var_guard235 != 0.0)) {
        let assign18460_e18238: f64 = (locals.var_fn205_calc_iq__etab).exp();
        (assign18460_e18238, 0.0, (assign18460_e18238 * locals.var_fn205_calc_iq__etab_dn3), (assign18460_e18238 * locals.var_fn205_calc_iq__etab_dn4), 0.0, 0.0, (assign18460_e18238 * locals.var_fn205_calc_iq__etab_dn11),)
    } else {
        (locals.var_fn205_calc_iq__exparg, locals.var_fn205_calc_iq__exparg_dn2, locals.var_fn205_calc_iq__exparg_dn3, locals.var_fn205_calc_iq__exparg_dn4, locals.var_fn205_calc_iq__exparg_dn7, locals.var_fn205_calc_iq__exparg_dn10, locals.var_fn205_calc_iq__exparg_dn11,)
    }
};
        locals.var_fn205_calc_iq__exparg = assign18460_e18240;
        locals.var_fn205_calc_iq__exparg_dn2 = assign18460_e18240_d_n2;
        locals.var_fn205_calc_iq__exparg_dn3 = assign18460_e18240_d_n3;
        locals.var_fn205_calc_iq__exparg_dn4 = assign18460_e18240_d_n4;
        locals.var_fn205_calc_iq__exparg_dn7 = assign18460_e18240_d_n7;
        locals.var_fn205_calc_iq__exparg_dn10 = assign18460_e18240_d_n10;
        locals.var_fn205_calc_iq__exparg_dn11 = assign18460_e18240_d_n11;

        let (assign18470_e18256, assign18470_e18256_d_n2, assign18470_e18256_d_n3, assign18470_e18256_d_n4, assign18470_e18256_d_n7, assign18470_e18256_d_n10, assign18470_e18256_d_n11,) = {
    if ((((locals.var_guard204 != 0.0) && (locals.var_guard231 != 0.0)) && (locals.var_guard234 == 0.0)) && (locals.var_guard235 == 0.0)) {
        let assign18470_e18252: f64 = (locals.var_fn205_calc_iq__etab).exp();
        let assign18470_e18253: f64 = (1.0 + assign18470_e18252);
        let assign18470_e18254: f64 = (assign18470_e18253).ln();
        (assign18470_e18254, 0.0, ((assign18470_e18252 * locals.var_fn205_calc_iq__etab_dn3) / assign18470_e18253), ((assign18470_e18252 * locals.var_fn205_calc_iq__etab_dn4) / assign18470_e18253), 0.0, 0.0, ((assign18470_e18252 * locals.var_fn205_calc_iq__etab_dn11) / assign18470_e18253),)
    } else {
        (locals.var_fn205_calc_iq__exparg, locals.var_fn205_calc_iq__exparg_dn2, locals.var_fn205_calc_iq__exparg_dn3, locals.var_fn205_calc_iq__exparg_dn4, locals.var_fn205_calc_iq__exparg_dn7, locals.var_fn205_calc_iq__exparg_dn10, locals.var_fn205_calc_iq__exparg_dn11,)
    }
};
        locals.var_fn205_calc_iq__exparg = assign18470_e18256;
        locals.var_fn205_calc_iq__exparg_dn2 = assign18470_e18256_d_n2;
        locals.var_fn205_calc_iq__exparg_dn3 = assign18470_e18256_d_n3;
        locals.var_fn205_calc_iq__exparg_dn4 = assign18470_e18256_d_n4;
        locals.var_fn205_calc_iq__exparg_dn7 = assign18470_e18256_d_n7;
        locals.var_fn205_calc_iq__exparg_dn10 = assign18470_e18256_d_n10;
        locals.var_fn205_calc_iq__exparg_dn11 = assign18470_e18256_d_n11;

        let (assign18480_e18274, assign18480_e18274_d_n2, assign18480_e18274_d_n3, assign18480_e18274_d_n4, assign18480_e18274_d_n7, assign18480_e18274_d_n10, assign18480_e18274_d_n11,) = {
    if ((locals.var_guard204 != 0.0) && (locals.var_guard231 != 0.0)) {
        let assign18480_e18262: f64 = (locals.var_fn205_calc_iq__w * locals.var_fn205_calc_iq__ngf);
        let assign18480_e18264: f64 = (assign18480_e18262 * locals.var_fn205_calc_iq__type);
        let assign18480_e18266: f64 = (assign18480_e18264 * locals.var_fn205_calc_iq__cb);
        let assign18480_e18268: f64 = (assign18480_e18266 * locals.var_fn205_calc_iq__two_n_phit0);
        let assign18480_e18270: f64 = (assign18480_e18268 * locals.var_fn205_calc_iq__exparg);
        let assign18480_e18272: f64 = (assign18480_e18270 * locals.var_fn205_calc_iq__trapfracdl);
        (assign18480_e18272, ((assign18480_e18268 * locals.var_fn205_calc_iq__exparg_dn2) * locals.var_fn205_calc_iq__trapfracdl), ((assign18480_e18268 * locals.var_fn205_calc_iq__exparg_dn3) * locals.var_fn205_calc_iq__trapfracdl), ((((((assign18480_e18264 * locals.var_fn205_calc_iq__cb_dn4) * locals.var_fn205_calc_iq__two_n_phit0) + (assign18480_e18266 * locals.var_fn205_calc_iq__two_n_phit0_dn4)) * locals.var_fn205_calc_iq__exparg) + (assign18480_e18268 * locals.var_fn205_calc_iq__exparg_dn4)) * locals.var_fn205_calc_iq__trapfracdl), ((assign18480_e18268 * locals.var_fn205_calc_iq__exparg_dn7) * locals.var_fn205_calc_iq__trapfracdl), ((assign18480_e18268 * locals.var_fn205_calc_iq__exparg_dn10) * locals.var_fn205_calc_iq__trapfracdl), ((assign18480_e18268 * locals.var_fn205_calc_iq__exparg_dn11) * locals.var_fn205_calc_iq__trapfracdl),)
    } else {
        (locals.var_fn205_calc_iq__qbout, locals.var_fn205_calc_iq__qbout_dn2, locals.var_fn205_calc_iq__qbout_dn3, locals.var_fn205_calc_iq__qbout_dn4, locals.var_fn205_calc_iq__qbout_dn7, locals.var_fn205_calc_iq__qbout_dn10, locals.var_fn205_calc_iq__qbout_dn11,)
    }
};
        locals.var_fn205_calc_iq__qbout = assign18480_e18274;
        locals.var_fn205_calc_iq__qbout_dn2 = assign18480_e18274_d_n2;
        locals.var_fn205_calc_iq__qbout_dn3 = assign18480_e18274_d_n3;
        locals.var_fn205_calc_iq__qbout_dn4 = assign18480_e18274_d_n4;
        locals.var_fn205_calc_iq__qbout_dn7 = assign18480_e18274_d_n7;
        locals.var_fn205_calc_iq__qbout_dn10 = assign18480_e18274_d_n10;
        locals.var_fn205_calc_iq__qbout_dn11 = assign18480_e18274_d_n11;

        let (assign18490_e18281, assign18490_e18281_d_n2, assign18490_e18281_d_n3, assign18490_e18281_d_n4, assign18490_e18281_d_n7, assign18490_e18281_d_n10, assign18490_e18281_d_n11,) = {
    if ((locals.var_guard204 != 0.0) && (locals.var_guard231 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__qcout, locals.var_fn205_calc_iq__qcout_dn2, locals.var_fn205_calc_iq__qcout_dn3, locals.var_fn205_calc_iq__qcout_dn4, locals.var_fn205_calc_iq__qcout_dn7, locals.var_fn205_calc_iq__qcout_dn10, locals.var_fn205_calc_iq__qcout_dn11,)
    }
};
        locals.var_fn205_calc_iq__qcout = assign18490_e18281;
        locals.var_fn205_calc_iq__qcout_dn2 = assign18490_e18281_d_n2;
        locals.var_fn205_calc_iq__qcout_dn3 = assign18490_e18281_d_n3;
        locals.var_fn205_calc_iq__qcout_dn4 = assign18490_e18281_d_n4;
        locals.var_fn205_calc_iq__qcout_dn7 = assign18490_e18281_d_n7;
        locals.var_fn205_calc_iq__qcout_dn10 = assign18490_e18281_d_n10;
        locals.var_fn205_calc_iq__qcout_dn11 = assign18490_e18281_d_n11;

        let (assign18500_e18288, assign18500_e18288_d_n2, assign18500_e18288_d_n3, assign18500_e18288_d_n4, assign18500_e18288_d_n7, assign18500_e18288_d_n10, assign18500_e18288_d_n11,) = {
    if ((locals.var_guard204 != 0.0) && (locals.var_guard231 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__qbout, locals.var_fn205_calc_iq__qbout_dn2, locals.var_fn205_calc_iq__qbout_dn3, locals.var_fn205_calc_iq__qbout_dn4, locals.var_fn205_calc_iq__qbout_dn7, locals.var_fn205_calc_iq__qbout_dn10, locals.var_fn205_calc_iq__qbout_dn11,)
    }
};
        locals.var_fn205_calc_iq__qbout = assign18500_e18288;
        locals.var_fn205_calc_iq__qbout_dn2 = assign18500_e18288_d_n2;
        locals.var_fn205_calc_iq__qbout_dn3 = assign18500_e18288_d_n3;
        locals.var_fn205_calc_iq__qbout_dn4 = assign18500_e18288_d_n4;
        locals.var_fn205_calc_iq__qbout_dn7 = assign18500_e18288_d_n7;
        locals.var_fn205_calc_iq__qbout_dn10 = assign18500_e18288_d_n10;
        locals.var_fn205_calc_iq__qbout_dn11 = assign18500_e18288_d_n11;

        let assign18510_e18291: f64 = if locals.var_fn205_calc_iq__qgsflag == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard236 = assign18510_e18291;

        let (assign18520_e18307, assign18520_e18307_d_n2, assign18520_e18307_d_n4, assign18520_e18307_d_n7, assign18520_e18307_d_n11,) = {
    if ((locals.var_guard204 != 0.0) && (locals.var_guard236 != 0.0)) {
        let assign18520_e18299: f64 = (p.p51 * 0.5);
        let assign18520_e18301: f64 = (assign18520_e18299 * locals.var_fn205_calc_iq__alpha_phit);
        let assign18520_e18302: f64 = (locals.var_fn205_calc_iq__vtof - assign18520_e18301);
        let assign18520_e18303: f64 = (locals.var_fn205_calc_iq__vgsin - assign18520_e18302);
        let assign18520_e18305: f64 = (assign18520_e18303 / locals.var_fn205_calc_iq__two_n_phit0);
        (assign18520_e18305, (locals.var_fn205_calc_iq__vgsin_dn2 / locals.var_fn205_calc_iq__two_n_phit0), ((((-(locals.var_fn205_calc_iq__vtof_dn4 - (assign18520_e18299 * locals.var_fn205_calc_iq__alpha_phit_dn4))) * locals.var_fn205_calc_iq__two_n_phit0) - (assign18520_e18303 * locals.var_fn205_calc_iq__two_n_phit0_dn4)) / (locals.var_fn205_calc_iq__two_n_phit0 * locals.var_fn205_calc_iq__two_n_phit0)), (locals.var_fn205_calc_iq__vgsin_dn7 / locals.var_fn205_calc_iq__two_n_phit0), (locals.var_fn205_calc_iq__vgsin_dn11 / locals.var_fn205_calc_iq__two_n_phit0),)
    } else {
        (locals.var_fn205_calc_iq__etags, locals.var_fn205_calc_iq__etags_dn2, locals.var_fn205_calc_iq__etags_dn4, locals.var_fn205_calc_iq__etags_dn7, locals.var_fn205_calc_iq__etags_dn11,)
    }
};
        locals.var_fn205_calc_iq__etags = assign18520_e18307;
        locals.var_fn205_calc_iq__etags_dn2 = assign18520_e18307_d_n2;
        locals.var_fn205_calc_iq__etags_dn4 = assign18520_e18307_d_n4;
        locals.var_fn205_calc_iq__etags_dn7 = assign18520_e18307_d_n7;
        locals.var_fn205_calc_iq__etags_dn11 = assign18520_e18307_d_n11;

        let assign18530_e18310: f64 = if locals.var_fn205_calc_iq__etags > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard237 = assign18530_e18310;

        let (assign18540_e18318, assign18540_e18318_d_n2, assign18540_e18318_d_n3, assign18540_e18318_d_n4, assign18540_e18318_d_n7, assign18540_e18318_d_n10, assign18540_e18318_d_n11,) = {
    if (((locals.var_guard204 != 0.0) && (locals.var_guard236 != 0.0)) && (locals.var_guard237 != 0.0)) {
        (locals.var_fn205_calc_iq__etags, locals.var_fn205_calc_iq__etags_dn2, 0.0, locals.var_fn205_calc_iq__etags_dn4, locals.var_fn205_calc_iq__etags_dn7, 0.0, locals.var_fn205_calc_iq__etags_dn11,)
    } else {
        (locals.var_fn205_calc_iq__exparg, locals.var_fn205_calc_iq__exparg_dn2, locals.var_fn205_calc_iq__exparg_dn3, locals.var_fn205_calc_iq__exparg_dn4, locals.var_fn205_calc_iq__exparg_dn7, locals.var_fn205_calc_iq__exparg_dn10, locals.var_fn205_calc_iq__exparg_dn11,)
    }
};
        locals.var_fn205_calc_iq__exparg = assign18540_e18318;
        locals.var_fn205_calc_iq__exparg_dn2 = assign18540_e18318_d_n2;
        locals.var_fn205_calc_iq__exparg_dn3 = assign18540_e18318_d_n3;
        locals.var_fn205_calc_iq__exparg_dn4 = assign18540_e18318_d_n4;
        locals.var_fn205_calc_iq__exparg_dn7 = assign18540_e18318_d_n7;
        locals.var_fn205_calc_iq__exparg_dn10 = assign18540_e18318_d_n10;
        locals.var_fn205_calc_iq__exparg_dn11 = assign18540_e18318_d_n11;

        let assign18550_e18321: f64 = (-50.0);
        let assign18550_e18322: f64 = if locals.var_fn205_calc_iq__etags < assign18550_e18321 { 1.0 } else { 0.0 };
        locals.var_guard238 = assign18550_e18322;

        let (assign18560_e18334, assign18560_e18334_d_n2, assign18560_e18334_d_n3, assign18560_e18334_d_n4, assign18560_e18334_d_n7, assign18560_e18334_d_n10, assign18560_e18334_d_n11,) = {
    if ((((locals.var_guard204 != 0.0) && (locals.var_guard236 != 0.0)) && (locals.var_guard237 == 0.0)) && (locals.var_guard238 != 0.0)) {
        let assign18560_e18332: f64 = (locals.var_fn205_calc_iq__etags).exp();
        (assign18560_e18332, (assign18560_e18332 * locals.var_fn205_calc_iq__etags_dn2), 0.0, (assign18560_e18332 * locals.var_fn205_calc_iq__etags_dn4), (assign18560_e18332 * locals.var_fn205_calc_iq__etags_dn7), 0.0, (assign18560_e18332 * locals.var_fn205_calc_iq__etags_dn11),)
    } else {
        (locals.var_fn205_calc_iq__exparg, locals.var_fn205_calc_iq__exparg_dn2, locals.var_fn205_calc_iq__exparg_dn3, locals.var_fn205_calc_iq__exparg_dn4, locals.var_fn205_calc_iq__exparg_dn7, locals.var_fn205_calc_iq__exparg_dn10, locals.var_fn205_calc_iq__exparg_dn11,)
    }
};
        locals.var_fn205_calc_iq__exparg = assign18560_e18334;
        locals.var_fn205_calc_iq__exparg_dn2 = assign18560_e18334_d_n2;
        locals.var_fn205_calc_iq__exparg_dn3 = assign18560_e18334_d_n3;
        locals.var_fn205_calc_iq__exparg_dn4 = assign18560_e18334_d_n4;
        locals.var_fn205_calc_iq__exparg_dn7 = assign18560_e18334_d_n7;
        locals.var_fn205_calc_iq__exparg_dn10 = assign18560_e18334_d_n10;
        locals.var_fn205_calc_iq__exparg_dn11 = assign18560_e18334_d_n11;

        let (assign18570_e18350, assign18570_e18350_d_n2, assign18570_e18350_d_n3, assign18570_e18350_d_n4, assign18570_e18350_d_n7, assign18570_e18350_d_n10, assign18570_e18350_d_n11,) = {
    if ((((locals.var_guard204 != 0.0) && (locals.var_guard236 != 0.0)) && (locals.var_guard237 == 0.0)) && (locals.var_guard238 == 0.0)) {
        let assign18570_e18346: f64 = (locals.var_fn205_calc_iq__etags).exp();
        let assign18570_e18347: f64 = (1.0 + assign18570_e18346);
        let assign18570_e18348: f64 = (assign18570_e18347).ln();
        (assign18570_e18348, ((assign18570_e18346 * locals.var_fn205_calc_iq__etags_dn2) / assign18570_e18347), 0.0, ((assign18570_e18346 * locals.var_fn205_calc_iq__etags_dn4) / assign18570_e18347), ((assign18570_e18346 * locals.var_fn205_calc_iq__etags_dn7) / assign18570_e18347), 0.0, ((assign18570_e18346 * locals.var_fn205_calc_iq__etags_dn11) / assign18570_e18347),)
    } else {
        (locals.var_fn205_calc_iq__exparg, locals.var_fn205_calc_iq__exparg_dn2, locals.var_fn205_calc_iq__exparg_dn3, locals.var_fn205_calc_iq__exparg_dn4, locals.var_fn205_calc_iq__exparg_dn7, locals.var_fn205_calc_iq__exparg_dn10, locals.var_fn205_calc_iq__exparg_dn11,)
    }
};
        locals.var_fn205_calc_iq__exparg = assign18570_e18350;
        locals.var_fn205_calc_iq__exparg_dn2 = assign18570_e18350_d_n2;
        locals.var_fn205_calc_iq__exparg_dn3 = assign18570_e18350_d_n3;
        locals.var_fn205_calc_iq__exparg_dn4 = assign18570_e18350_d_n4;
        locals.var_fn205_calc_iq__exparg_dn7 = assign18570_e18350_d_n7;
        locals.var_fn205_calc_iq__exparg_dn10 = assign18570_e18350_d_n10;
        locals.var_fn205_calc_iq__exparg_dn11 = assign18570_e18350_d_n11;

        let (assign18580_e18368, assign18580_e18368_d_n2, assign18580_e18368_d_n3, assign18580_e18368_d_n4, assign18580_e18368_d_n7, assign18580_e18368_d_n10, assign18580_e18368_d_n11,) = {
    if ((locals.var_guard204 != 0.0) && (locals.var_guard236 != 0.0)) {
        let assign18580_e18356: f64 = (locals.var_fn205_calc_iq__w * locals.var_fn205_calc_iq__ngf);
        let assign18580_e18358: f64 = (assign18580_e18356 * locals.var_fn205_calc_iq__type);
        let assign18580_e18360: f64 = (assign18580_e18358 * locals.var_fn205_calc_iq__cs);
        let assign18580_e18362: f64 = (assign18580_e18360 * locals.var_fn205_calc_iq__two_n_phit0);
        let assign18580_e18364: f64 = (assign18580_e18362 * locals.var_fn205_calc_iq__exparg);
        let assign18580_e18366: f64 = (assign18580_e18364 * locals.var_fn205_calc_iq__trapfracdl);
        (assign18580_e18366, ((assign18580_e18362 * locals.var_fn205_calc_iq__exparg_dn2) * locals.var_fn205_calc_iq__trapfracdl), ((assign18580_e18362 * locals.var_fn205_calc_iq__exparg_dn3) * locals.var_fn205_calc_iq__trapfracdl), ((((assign18580_e18360 * locals.var_fn205_calc_iq__two_n_phit0_dn4) * locals.var_fn205_calc_iq__exparg) + (assign18580_e18362 * locals.var_fn205_calc_iq__exparg_dn4)) * locals.var_fn205_calc_iq__trapfracdl), ((assign18580_e18362 * locals.var_fn205_calc_iq__exparg_dn7) * locals.var_fn205_calc_iq__trapfracdl), ((assign18580_e18362 * locals.var_fn205_calc_iq__exparg_dn10) * locals.var_fn205_calc_iq__trapfracdl), ((assign18580_e18362 * locals.var_fn205_calc_iq__exparg_dn11) * locals.var_fn205_calc_iq__trapfracdl),)
    } else {
        (locals.var_fn205_calc_iq__qsout, locals.var_fn205_calc_iq__qsout_dn2, locals.var_fn205_calc_iq__qsout_dn3, locals.var_fn205_calc_iq__qsout_dn4, locals.var_fn205_calc_iq__qsout_dn7, locals.var_fn205_calc_iq__qsout_dn10, locals.var_fn205_calc_iq__qsout_dn11,)
    }
};
        locals.var_fn205_calc_iq__qsout = assign18580_e18368;
        locals.var_fn205_calc_iq__qsout_dn2 = assign18580_e18368_d_n2;
        locals.var_fn205_calc_iq__qsout_dn3 = assign18580_e18368_d_n3;
        locals.var_fn205_calc_iq__qsout_dn4 = assign18580_e18368_d_n4;
        locals.var_fn205_calc_iq__qsout_dn7 = assign18580_e18368_d_n7;
        locals.var_fn205_calc_iq__qsout_dn10 = assign18580_e18368_d_n10;
        locals.var_fn205_calc_iq__qsout_dn11 = assign18580_e18368_d_n11;

    }

    pub(super) fn stamp_transient_block_47(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign18590_e18375, assign18590_e18375_d_n2, assign18590_e18375_d_n3, assign18590_e18375_d_n4, assign18590_e18375_d_n7, assign18590_e18375_d_n10, assign18590_e18375_d_n11,) = {
    if ((locals.var_guard204 != 0.0) && (locals.var_guard236 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn205_calc_iq__qsout, locals.var_fn205_calc_iq__qsout_dn2, locals.var_fn205_calc_iq__qsout_dn3, locals.var_fn205_calc_iq__qsout_dn4, locals.var_fn205_calc_iq__qsout_dn7, locals.var_fn205_calc_iq__qsout_dn10, locals.var_fn205_calc_iq__qsout_dn11,)
    }
};
        locals.var_fn205_calc_iq__qsout = assign18590_e18375;
        locals.var_fn205_calc_iq__qsout_dn2 = assign18590_e18375_d_n2;
        locals.var_fn205_calc_iq__qsout_dn3 = assign18590_e18375_d_n3;
        locals.var_fn205_calc_iq__qsout_dn4 = assign18590_e18375_d_n4;
        locals.var_fn205_calc_iq__qsout_dn7 = assign18590_e18375_d_n7;
        locals.var_fn205_calc_iq__qsout_dn10 = assign18590_e18375_d_n10;
        locals.var_fn205_calc_iq__qsout_dn11 = assign18590_e18375_d_n11;

        let (assign18620_e18387, assign18620_e18387_d_n2, assign18620_e18387_d_n4, assign18620_e18387_d_n7, assign18620_e18387_d_n10, assign18620_e18387_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        (locals.var_fn205_calc_iq__qgsout, locals.var_fn205_calc_iq__qgsout_dn2, locals.var_fn205_calc_iq__qgsout_dn4, locals.var_fn205_calc_iq__qgsout_dn7, locals.var_fn205_calc_iq__qgsout_dn10, locals.var_fn205_calc_iq__qgsout_dn11,)
    } else {
        (locals.var_qgsfps2, locals.var_qgsfps2_dn2, locals.var_qgsfps2_dn4, locals.var_qgsfps2_dn7, locals.var_qgsfps2_dn10, locals.var_qgsfps2_dn11,)
    }
};
        locals.var_qgsfps2 = assign18620_e18387;
        locals.var_qgsfps2_dn2 = assign18620_e18387_d_n2;
        locals.var_qgsfps2_dn4 = assign18620_e18387_d_n4;
        locals.var_qgsfps2_dn7 = assign18620_e18387_d_n7;
        locals.var_qgsfps2_dn10 = assign18620_e18387_d_n10;
        locals.var_qgsfps2_dn11 = assign18620_e18387_d_n11;

        let (assign18630_e18391, assign18630_e18391_d_n2, assign18630_e18391_d_n4, assign18630_e18391_d_n7, assign18630_e18391_d_n10, assign18630_e18391_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        (locals.var_fn205_calc_iq__qgdout, locals.var_fn205_calc_iq__qgdout_dn2, locals.var_fn205_calc_iq__qgdout_dn4, locals.var_fn205_calc_iq__qgdout_dn7, locals.var_fn205_calc_iq__qgdout_dn10, locals.var_fn205_calc_iq__qgdout_dn11,)
    } else {
        (locals.var_qgdfps2, locals.var_qgdfps2_dn2, locals.var_qgdfps2_dn4, locals.var_qgdfps2_dn7, locals.var_qgdfps2_dn10, locals.var_qgdfps2_dn11,)
    }
};
        locals.var_qgdfps2 = assign18630_e18391;
        locals.var_qgdfps2_dn2 = assign18630_e18391_d_n2;
        locals.var_qgdfps2_dn4 = assign18630_e18391_d_n4;
        locals.var_qgdfps2_dn7 = assign18630_e18391_d_n7;
        locals.var_qgdfps2_dn10 = assign18630_e18391_d_n10;
        locals.var_qgdfps2_dn11 = assign18630_e18391_d_n11;

        let (assign18640_e18395, assign18640_e18395_d_n2, assign18640_e18395_d_n3, assign18640_e18395_d_n4, assign18640_e18395_d_n7, assign18640_e18395_d_n10, assign18640_e18395_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        (locals.var_fn205_calc_iq__qcout, locals.var_fn205_calc_iq__qcout_dn2, locals.var_fn205_calc_iq__qcout_dn3, locals.var_fn205_calc_iq__qcout_dn4, locals.var_fn205_calc_iq__qcout_dn7, locals.var_fn205_calc_iq__qcout_dn10, locals.var_fn205_calc_iq__qcout_dn11,)
    } else {
        (locals.var_qcfps2, locals.var_qcfps2_dn2, locals.var_qcfps2_dn3, locals.var_qcfps2_dn4, locals.var_qcfps2_dn7, locals.var_qcfps2_dn10, locals.var_qcfps2_dn11,)
    }
};
        locals.var_qcfps2 = assign18640_e18395;
        locals.var_qcfps2_dn2 = assign18640_e18395_d_n2;
        locals.var_qcfps2_dn3 = assign18640_e18395_d_n3;
        locals.var_qcfps2_dn4 = assign18640_e18395_d_n4;
        locals.var_qcfps2_dn7 = assign18640_e18395_d_n7;
        locals.var_qcfps2_dn10 = assign18640_e18395_d_n10;
        locals.var_qcfps2_dn11 = assign18640_e18395_d_n11;

        let (assign18650_e18399, assign18650_e18399_d_n2, assign18650_e18399_d_n3, assign18650_e18399_d_n4, assign18650_e18399_d_n7, assign18650_e18399_d_n10, assign18650_e18399_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        (locals.var_fn205_calc_iq__qbout, locals.var_fn205_calc_iq__qbout_dn2, locals.var_fn205_calc_iq__qbout_dn3, locals.var_fn205_calc_iq__qbout_dn4, locals.var_fn205_calc_iq__qbout_dn7, locals.var_fn205_calc_iq__qbout_dn10, locals.var_fn205_calc_iq__qbout_dn11,)
    } else {
        (locals.var_qbfps2, locals.var_qbfps2_dn2, locals.var_qbfps2_dn3, locals.var_qbfps2_dn4, locals.var_qbfps2_dn7, locals.var_qbfps2_dn10, locals.var_qbfps2_dn11,)
    }
};
        locals.var_qbfps2 = assign18650_e18399;
        locals.var_qbfps2_dn2 = assign18650_e18399_d_n2;
        locals.var_qbfps2_dn3 = assign18650_e18399_d_n3;
        locals.var_qbfps2_dn4 = assign18650_e18399_d_n4;
        locals.var_qbfps2_dn7 = assign18650_e18399_d_n7;
        locals.var_qbfps2_dn10 = assign18650_e18399_d_n10;
        locals.var_qbfps2_dn11 = assign18650_e18399_d_n11;

        let (assign18660_e18403, assign18660_e18403_d_n2, assign18660_e18403_d_n3, assign18660_e18403_d_n4, assign18660_e18403_d_n7, assign18660_e18403_d_n10, assign18660_e18403_d_n11,) = {
    if (locals.var_guard204 != 0.0) {
        (locals.var_fn205_calc_iq__qsout, locals.var_fn205_calc_iq__qsout_dn2, locals.var_fn205_calc_iq__qsout_dn3, locals.var_fn205_calc_iq__qsout_dn4, locals.var_fn205_calc_iq__qsout_dn7, locals.var_fn205_calc_iq__qsout_dn10, locals.var_fn205_calc_iq__qsout_dn11,)
    } else {
        (locals.var_qsfps2, locals.var_qsfps2_dn2, locals.var_qsfps2_dn3, locals.var_qsfps2_dn4, locals.var_qsfps2_dn7, locals.var_qsfps2_dn10, locals.var_qsfps2_dn11,)
    }
};
        locals.var_qsfps2 = assign18660_e18403;
        locals.var_qsfps2_dn2 = assign18660_e18403_d_n2;
        locals.var_qsfps2_dn3 = assign18660_e18403_d_n3;
        locals.var_qsfps2_dn4 = assign18660_e18403_d_n4;
        locals.var_qsfps2_dn7 = assign18660_e18403_d_n7;
        locals.var_qsfps2_dn10 = assign18660_e18403_d_n10;
        locals.var_qsfps2_dn11 = assign18660_e18403_d_n11;

        let assign18700_e18418: f64 = if p.p100 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard239 = assign18700_e18418;

        locals.var_qgsfps3 = 0.0;
        locals.var_qgsfps3_dn2 = 0.0;
        locals.var_qgsfps3_dn4 = 0.0;
        locals.var_qgsfps3_dn7 = 0.0;
        locals.var_qgsfps3_dn11 = 0.0;
        locals.var_qgsfps3_dn12 = 0.0;

        locals.var_qgdfps3 = 0.0;
        locals.var_qgdfps3_dn2 = 0.0;
        locals.var_qgdfps3_dn4 = 0.0;
        locals.var_qgdfps3_dn7 = 0.0;
        locals.var_qgdfps3_dn11 = 0.0;
        locals.var_qgdfps3_dn12 = 0.0;

        locals.var_qcfps3 = 0.0;
        locals.var_qcfps3_dn2 = 0.0;
        locals.var_qcfps3_dn3 = 0.0;
        locals.var_qcfps3_dn4 = 0.0;
        locals.var_qcfps3_dn7 = 0.0;
        locals.var_qcfps3_dn11 = 0.0;
        locals.var_qcfps3_dn12 = 0.0;

        locals.var_qbfps3 = 0.0;
        locals.var_qbfps3_dn2 = 0.0;
        locals.var_qbfps3_dn3 = 0.0;
        locals.var_qbfps3_dn4 = 0.0;
        locals.var_qbfps3_dn7 = 0.0;
        locals.var_qbfps3_dn11 = 0.0;
        locals.var_qbfps3_dn12 = 0.0;

        locals.var_qsfps3 = 0.0;
        locals.var_qsfps3_dn2 = 0.0;
        locals.var_qsfps3_dn3 = 0.0;
        locals.var_qsfps3_dn4 = 0.0;
        locals.var_qsfps3_dn7 = 0.0;
        locals.var_qsfps3_dn11 = 0.0;
        locals.var_qsfps3_dn12 = 0.0;

        let assign18790_e18429: f64 = if p.p123 > p.p354 { 1.0 } else { 0.0 };
        locals.var_guard240 = assign18790_e18429;

        let (assign18820_e18441, assign18820_e18441_d_n2, assign18820_e18441_d_n4, assign18820_e18441_d_n7, assign18820_e18441_d_n11, assign18820_e18441_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__qgsout, locals.var_fn241_calc_iq__qgsout_dn2, locals.var_fn241_calc_iq__qgsout_dn4, locals.var_fn241_calc_iq__qgsout_dn7, locals.var_fn241_calc_iq__qgsout_dn11, locals.var_fn241_calc_iq__qgsout_dn12,)
    }
};
        locals.var_fn241_calc_iq__qgsout = assign18820_e18441;
        locals.var_fn241_calc_iq__qgsout_dn2 = assign18820_e18441_d_n2;
        locals.var_fn241_calc_iq__qgsout_dn4 = assign18820_e18441_d_n4;
        locals.var_fn241_calc_iq__qgsout_dn7 = assign18820_e18441_d_n7;
        locals.var_fn241_calc_iq__qgsout_dn11 = assign18820_e18441_d_n11;
        locals.var_fn241_calc_iq__qgsout_dn12 = assign18820_e18441_d_n12;

        let (assign18830_e18445, assign18830_e18445_d_n2, assign18830_e18445_d_n4, assign18830_e18445_d_n7, assign18830_e18445_d_n11, assign18830_e18445_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__qgdout, locals.var_fn241_calc_iq__qgdout_dn2, locals.var_fn241_calc_iq__qgdout_dn4, locals.var_fn241_calc_iq__qgdout_dn7, locals.var_fn241_calc_iq__qgdout_dn11, locals.var_fn241_calc_iq__qgdout_dn12,)
    }
};
        locals.var_fn241_calc_iq__qgdout = assign18830_e18445;
        locals.var_fn241_calc_iq__qgdout_dn2 = assign18830_e18445_d_n2;
        locals.var_fn241_calc_iq__qgdout_dn4 = assign18830_e18445_d_n4;
        locals.var_fn241_calc_iq__qgdout_dn7 = assign18830_e18445_d_n7;
        locals.var_fn241_calc_iq__qgdout_dn11 = assign18830_e18445_d_n11;
        locals.var_fn241_calc_iq__qgdout_dn12 = assign18830_e18445_d_n12;

        let (assign18840_e18449, assign18840_e18449_d_n2, assign18840_e18449_d_n3, assign18840_e18449_d_n4, assign18840_e18449_d_n7, assign18840_e18449_d_n11, assign18840_e18449_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__qcout, locals.var_fn241_calc_iq__qcout_dn2, locals.var_fn241_calc_iq__qcout_dn3, locals.var_fn241_calc_iq__qcout_dn4, locals.var_fn241_calc_iq__qcout_dn7, locals.var_fn241_calc_iq__qcout_dn11, locals.var_fn241_calc_iq__qcout_dn12,)
    }
};
        locals.var_fn241_calc_iq__qcout = assign18840_e18449;
        locals.var_fn241_calc_iq__qcout_dn2 = assign18840_e18449_d_n2;
        locals.var_fn241_calc_iq__qcout_dn3 = assign18840_e18449_d_n3;
        locals.var_fn241_calc_iq__qcout_dn4 = assign18840_e18449_d_n4;
        locals.var_fn241_calc_iq__qcout_dn7 = assign18840_e18449_d_n7;
        locals.var_fn241_calc_iq__qcout_dn11 = assign18840_e18449_d_n11;
        locals.var_fn241_calc_iq__qcout_dn12 = assign18840_e18449_d_n12;

        let (assign18850_e18453, assign18850_e18453_d_n2, assign18850_e18453_d_n3, assign18850_e18453_d_n4, assign18850_e18453_d_n7, assign18850_e18453_d_n11, assign18850_e18453_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__qbout, locals.var_fn241_calc_iq__qbout_dn2, locals.var_fn241_calc_iq__qbout_dn3, locals.var_fn241_calc_iq__qbout_dn4, locals.var_fn241_calc_iq__qbout_dn7, locals.var_fn241_calc_iq__qbout_dn11, locals.var_fn241_calc_iq__qbout_dn12,)
    }
};
        locals.var_fn241_calc_iq__qbout = assign18850_e18453;
        locals.var_fn241_calc_iq__qbout_dn2 = assign18850_e18453_d_n2;
        locals.var_fn241_calc_iq__qbout_dn3 = assign18850_e18453_d_n3;
        locals.var_fn241_calc_iq__qbout_dn4 = assign18850_e18453_d_n4;
        locals.var_fn241_calc_iq__qbout_dn7 = assign18850_e18453_d_n7;
        locals.var_fn241_calc_iq__qbout_dn11 = assign18850_e18453_d_n11;
        locals.var_fn241_calc_iq__qbout_dn12 = assign18850_e18453_d_n12;

        let (assign18860_e18457, assign18860_e18457_d_n2, assign18860_e18457_d_n3, assign18860_e18457_d_n4, assign18860_e18457_d_n7, assign18860_e18457_d_n11, assign18860_e18457_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__qsout, locals.var_fn241_calc_iq__qsout_dn2, locals.var_fn241_calc_iq__qsout_dn3, locals.var_fn241_calc_iq__qsout_dn4, locals.var_fn241_calc_iq__qsout_dn7, locals.var_fn241_calc_iq__qsout_dn11, locals.var_fn241_calc_iq__qsout_dn12,)
    }
};
        locals.var_fn241_calc_iq__qsout = assign18860_e18457;
        locals.var_fn241_calc_iq__qsout_dn2 = assign18860_e18457_d_n2;
        locals.var_fn241_calc_iq__qsout_dn3 = assign18860_e18457_d_n3;
        locals.var_fn241_calc_iq__qsout_dn4 = assign18860_e18457_d_n4;
        locals.var_fn241_calc_iq__qsout_dn7 = assign18860_e18457_d_n7;
        locals.var_fn241_calc_iq__qsout_dn11 = assign18860_e18457_d_n11;
        locals.var_fn241_calc_iq__qsout_dn12 = assign18860_e18457_d_n12;

        let (assign18870_e18461, assign18870_e18461_d_n4, assign18870_e18461_d_n11, assign18870_e18461_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__vtdibl, locals.var_fn241_calc_iq__vtdibl_dn4, locals.var_fn241_calc_iq__vtdibl_dn11, locals.var_fn241_calc_iq__vtdibl_dn12,)
    }
};
        locals.var_fn241_calc_iq__vtdibl = assign18870_e18461;
        locals.var_fn241_calc_iq__vtdibl_dn4 = assign18870_e18461_d_n4;
        locals.var_fn241_calc_iq__vtdibl_dn11 = assign18870_e18461_d_n11;
        locals.var_fn241_calc_iq__vtdibl_dn12 = assign18870_e18461_d_n12;

        let (assign18880_e18465, assign18880_e18465_d_n2, assign18880_e18465_d_n3, assign18880_e18465_d_n4, assign18880_e18465_d_n7, assign18880_e18465_d_n11, assign18880_e18465_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__vdsat1, locals.var_fn241_calc_iq__vdsat1_dn2, locals.var_fn241_calc_iq__vdsat1_dn3, locals.var_fn241_calc_iq__vdsat1_dn4, locals.var_fn241_calc_iq__vdsat1_dn7, locals.var_fn241_calc_iq__vdsat1_dn11, locals.var_fn241_calc_iq__vdsat1_dn12,)
    }
};
        locals.var_fn241_calc_iq__vdsat1 = assign18880_e18465;
        locals.var_fn241_calc_iq__vdsat1_dn2 = assign18880_e18465_d_n2;
        locals.var_fn241_calc_iq__vdsat1_dn3 = assign18880_e18465_d_n3;
        locals.var_fn241_calc_iq__vdsat1_dn4 = assign18880_e18465_d_n4;
        locals.var_fn241_calc_iq__vdsat1_dn7 = assign18880_e18465_d_n7;
        locals.var_fn241_calc_iq__vdsat1_dn11 = assign18880_e18465_d_n11;
        locals.var_fn241_calc_iq__vdsat1_dn12 = assign18880_e18465_d_n12;

        let (assign18890_e18469, assign18890_e18469_d_n2, assign18890_e18469_d_n7, assign18890_e18469_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        (locals.var_vgsfps3, locals.var_vgsfps3_dn2, locals.var_vgsfps3_dn7, locals.var_vgsfps3_dn12,)
    } else {
        (locals.var_fn241_calc_iq__vgsin, locals.var_fn241_calc_iq__vgsin_dn2, locals.var_fn241_calc_iq__vgsin_dn7, locals.var_fn241_calc_iq__vgsin_dn12,)
    }
};
        locals.var_fn241_calc_iq__vgsin = assign18890_e18469;
        locals.var_fn241_calc_iq__vgsin_dn2 = assign18890_e18469_d_n2;
        locals.var_fn241_calc_iq__vgsin_dn7 = assign18890_e18469_d_n7;
        locals.var_fn241_calc_iq__vgsin_dn12 = assign18890_e18469_d_n12;

        let (assign18900_e18473, assign18900_e18473_d_n11, assign18900_e18473_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        (locals.var_vdsfps3, locals.var_vdsfps3_dn11, locals.var_vdsfps3_dn12,)
    } else {
        (locals.var_fn241_calc_iq__vdsin, locals.var_fn241_calc_iq__vdsin_dn11, locals.var_fn241_calc_iq__vdsin_dn12,)
    }
};
        locals.var_fn241_calc_iq__vdsin = assign18900_e18473;
        locals.var_fn241_calc_iq__vdsin_dn11 = assign18900_e18473_d_n11;
        locals.var_fn241_calc_iq__vdsin_dn12 = assign18900_e18473_d_n12;

        let (assign18910_e18477,) = {
    if (locals.var_guard240 != 0.0) {
        (p.p129,)
    } else {
        (locals.var_fn241_calc_iq__qcbflag,)
    }
};
        locals.var_fn241_calc_iq__qcbflag = assign18910_e18477;

        let (assign18920_e18481, assign18920_e18481_d_n2, assign18920_e18481_d_n7, assign18920_e18481_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        (locals.var_vcfps3, locals.var_vcfps3_dn2, locals.var_vcfps3_dn7, locals.var_vcfps3_dn12,)
    } else {
        (locals.var_fn241_calc_iq__vcin, locals.var_fn241_calc_iq__vcin_dn2, locals.var_fn241_calc_iq__vcin_dn7, locals.var_fn241_calc_iq__vcin_dn12,)
    }
};
        locals.var_fn241_calc_iq__vcin = assign18920_e18481;
        locals.var_fn241_calc_iq__vcin_dn2 = assign18920_e18481_d_n2;
        locals.var_fn241_calc_iq__vcin_dn7 = assign18920_e18481_d_n7;
        locals.var_fn241_calc_iq__vcin_dn12 = assign18920_e18481_d_n12;

        let (assign18930_e18485, assign18930_e18485_d_n3, assign18930_e18485_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        (locals.var_vbfps3, locals.var_vbfps3_dn3, locals.var_vbfps3_dn12,)
    } else {
        (locals.var_fn241_calc_iq__vbin, locals.var_fn241_calc_iq__vbin_dn3, locals.var_fn241_calc_iq__vbin_dn12,)
    }
};
        locals.var_fn241_calc_iq__vbin = assign18930_e18485;
        locals.var_fn241_calc_iq__vbin_dn3 = assign18930_e18485_d_n3;
        locals.var_fn241_calc_iq__vbin_dn12 = assign18930_e18485_d_n12;

        let (assign18940_e18489,) = {
    if (locals.var_guard240 != 0.0) {
        (p.p127,)
    } else {
        (locals.var_fn241_calc_iq__qgsflag,)
    }
};
        locals.var_fn241_calc_iq__qgsflag = assign18940_e18489;

        let (assign18950_e18493, assign18950_e18493_d_n4,) = {
    if (locals.var_guard240 != 0.0) {
        (locals.var_tdut, locals.var_tdut_dn4,)
    } else {
        (locals.var_fn241_calc_iq__tambin, locals.var_fn241_calc_iq__tambin_dn4,)
    }
};
        locals.var_fn241_calc_iq__tambin = assign18950_e18493;
        locals.var_fn241_calc_iq__tambin_dn4 = assign18950_e18493_d_n4;

        let (assign18960_e18497,) = {
    if (locals.var_guard240 != 0.0) {
        (locals.var_tnomk,)
    } else {
        (locals.var_fn241_calc_iq__tnomin,)
    }
};
        locals.var_fn241_calc_iq__tnomin = assign18960_e18497;

        let (assign18970_e18501, assign18970_e18501_d_n4,) = {
    if (locals.var_guard240 != 0.0) {
        (locals.var_phit, locals.var_phit_dn4,)
    } else {
        (locals.var_fn241_calc_iq__phitin, locals.var_fn241_calc_iq__phitin_dn4,)
    }
};
        locals.var_fn241_calc_iq__phitin = assign18970_e18501;
        locals.var_fn241_calc_iq__phitin_dn4 = assign18970_e18501_d_n4;

        let (assign18980_e18505,) = {
    if (locals.var_guard240 != 0.0) {
        (p.p0,)
    } else {
        (locals.var_fn241_calc_iq__w,)
    }
};
        locals.var_fn241_calc_iq__w = assign18980_e18505;

        let (assign18990_e18509,) = {
    if (locals.var_guard240 != 0.0) {
        (p.p123,)
    } else {
        (locals.var_fn241_calc_iq__lin,)
    }
};
        locals.var_fn241_calc_iq__lin = assign18990_e18509;

        let (assign19000_e18513, assign19000_e18513_d_n4,) = {
    if (locals.var_guard240 != 0.0) {
        (locals.var_cgfps3t, locals.var_cgfps3t_dn4,)
    } else {
        (locals.var_fn241_calc_iq__cgin, locals.var_fn241_calc_iq__cgin_dn4,)
    }
};
        locals.var_fn241_calc_iq__cgin = assign19000_e18513;
        locals.var_fn241_calc_iq__cgin_dn4 = assign19000_e18513_d_n4;

        let (assign19010_e18517,) = {
    if (locals.var_guard240 != 0.0) {
        (p.p128,)
    } else {
        (locals.var_fn241_calc_iq__cs,)
    }
};
        locals.var_fn241_calc_iq__cs = assign19010_e18517;

        let (assign19020_e18521, assign19020_e18521_d_n4,) = {
    if (locals.var_guard240 != 0.0) {
        (locals.var_ccfps3t, locals.var_ccfps3t_dn4,)
    } else {
        (locals.var_fn241_calc_iq__cc, locals.var_fn241_calc_iq__cc_dn4,)
    }
};
        locals.var_fn241_calc_iq__cc = assign19020_e18521;
        locals.var_fn241_calc_iq__cc_dn4 = assign19020_e18521_d_n4;

        let (assign19030_e18525, assign19030_e18525_d_n4,) = {
    if (locals.var_guard240 != 0.0) {
        (locals.var_cbfps3t, locals.var_cbfps3t_dn4,)
    } else {
        (locals.var_fn241_calc_iq__cb, locals.var_fn241_calc_iq__cb_dn4,)
    }
};
        locals.var_fn241_calc_iq__cb = assign19030_e18525;
        locals.var_fn241_calc_iq__cb_dn4 = assign19030_e18525_d_n4;

        let (assign19040_e18529,) = {
    if (locals.var_guard240 != 0.0) {
        (p.p124,)
    } else {
        (locals.var_fn241_calc_iq__vto,)
    }
};
        locals.var_fn241_calc_iq__vto = assign19040_e18529;

        let (assign19050_e18533,) = {
    if (locals.var_guard240 != 0.0) {
        (p.p138,)
    } else {
        (locals.var_fn241_calc_iq__ss,)
    }
};
        locals.var_fn241_calc_iq__ss = assign19050_e18533;

        let (assign19060_e18537,) = {
    if (locals.var_guard240 != 0.0) {
        (p.p137,)
    } else {
        (locals.var_fn241_calc_iq__delta1,)
    }
};
        locals.var_fn241_calc_iq__delta1 = assign19060_e18537;

        let (assign19070_e18541,) = {
    if (locals.var_guard240 != 0.0) {
        (0.0,)
    } else {
        (locals.var_fn241_calc_iq__delta2,)
    }
};
        locals.var_fn241_calc_iq__delta2 = assign19070_e18541;

        let (assign19080_e18545,) = {
    if (locals.var_guard240 != 0.0) {
        (p.p139,)
    } else {
        (locals.var_fn241_calc_iq__nd,)
    }
};
        locals.var_fn241_calc_iq__nd = assign19080_e18545;

        let (assign19090_e18549,) = {
    if (locals.var_guard240 != 0.0) {
        (p.p143,)
    } else {
        (locals.var_fn241_calc_iq__alpha,)
    }
};
        locals.var_fn241_calc_iq__alpha = assign19090_e18549;

        let (assign19100_e18553,) = {
    if (locals.var_guard240 != 0.0) {
        (p.p134,)
    } else {
        (locals.var_fn241_calc_iq__vel0,)
    }
};
        locals.var_fn241_calc_iq__vel0 = assign19100_e18553;

        let (assign19110_e18557,) = {
    if (locals.var_guard240 != 0.0) {
        (p.p135,)
    } else {
        (locals.var_fn241_calc_iq__mu0,)
    }
};
        locals.var_fn241_calc_iq__mu0 = assign19110_e18557;

        let (assign19120_e18561,) = {
    if (locals.var_guard240 != 0.0) {
        (p.p136,)
    } else {
        (locals.var_fn241_calc_iq__beta,)
    }
};
        locals.var_fn241_calc_iq__beta = assign19120_e18561;

        let (assign19130_e18565,) = {
    if (locals.var_guard240 != 0.0) {
        (p.p142,)
    } else {
        (locals.var_fn241_calc_iq__mtheta,)
    }
};
        locals.var_fn241_calc_iq__mtheta = assign19130_e18565;

        let (assign19140_e18569,) = {
    if (locals.var_guard240 != 0.0) {
        (p.p141,)
    } else {
        (locals.var_fn241_calc_iq__vtheta,)
    }
};
        locals.var_fn241_calc_iq__vtheta = assign19140_e18569;

        let (assign19150_e18573,) = {
    if (locals.var_guard240 != 0.0) {
        (p.p140,)
    } else {
        (locals.var_fn241_calc_iq__vtzeta,)
    }
};
        locals.var_fn241_calc_iq__vtzeta = assign19150_e18573;

        let (assign19160_e18577,) = {
    if (locals.var_guard240 != 0.0) {
        (p.p39,)
    } else {
        (locals.var_fn241_calc_iq__dibsat,)
    }
};
        locals.var_fn241_calc_iq__dibsat = assign19160_e18577;

        let (assign19170_e18581,) = {
    if (locals.var_guard240 != 0.0) {
        (p.p47,)
    } else {
        (locals.var_fn241_calc_iq__epsilon,)
    }
};
        locals.var_fn241_calc_iq__epsilon = assign19170_e18581;

    }
}
