#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_326(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign88040_e134211, assign88040_e134211_d_n0, assign88040_e134211_d_n2, assign88040_e134211_d_n4, assign88040_e134211_d_n5, assign88040_e134211_d_n6, assign88040_e134211_d_n7, assign88040_e134211_d_n8, assign88040_e134211_d_n9, assign88040_e134211_d_n10, assign88040_e134211_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2041 != 0.0)) && (locals.var_guard2042 != 0.0)) {
        let assign88040_e134209: f64 = (p.p334 - locals.var_wdep_func);
        (assign88040_e134209, (-locals.var_wdep_func_dn0), (-locals.var_wdep_func_dn2), (-locals.var_wdep_func_dn4), (-locals.var_wdep_func_dn5), (-locals.var_wdep_func_dn6), (-locals.var_wdep_func_dn7), (-locals.var_wdep_func_dn8), (-locals.var_wdep_func_dn9), (-locals.var_wdep_func_dn10), (-locals.var_wdep_func_dn13),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign88040_e134211;
        locals.var_t2_dn0 = assign88040_e134211_d_n0;
        locals.var_t2_dn2 = assign88040_e134211_d_n2;
        locals.var_t2_dn4 = assign88040_e134211_d_n4;
        locals.var_t2_dn5 = assign88040_e134211_d_n5;
        locals.var_t2_dn6 = assign88040_e134211_d_n6;
        locals.var_t2_dn7 = assign88040_e134211_d_n7;
        locals.var_t2_dn8 = assign88040_e134211_d_n8;
        locals.var_t2_dn9 = assign88040_e134211_d_n9;
        locals.var_t2_dn10 = assign88040_e134211_d_n10;
        locals.var_t2_dn13 = assign88040_e134211_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign88050_e134236, assign88050_e134236_d_n0, assign88050_e134236_d_n2, assign88050_e134236_d_n4, assign88050_e134236_d_n5, assign88050_e134236_d_n6, assign88050_e134236_d_n7, assign88050_e134236_d_n8, assign88050_e134236_d_n9, assign88050_e134236_d_n10, assign88050_e134236_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2041 != 0.0)) && (locals.var_guard2042 == 0.0)) {
        let assign88050_e134223: f64 = (locals.var_vdsi + p.p137);
        let assign88050_e134226: f64 = (locals.var_vdsi + p.p137);
        let assign88050_e134227: f64 = (assign88050_e134223 * assign88050_e134226);
        let assign88050_e134230: f64 = (4.0 * 0.1);
        let assign88050_e134232: f64 = (assign88050_e134230 * 0.1);
        let assign88050_e134233: f64 = (assign88050_e134227 + assign88050_e134232);
        let assign88050_e134234: f64 = (assign88050_e134233).sqrt();
        (assign88050_e134234, 0.0, 0.0, 0.0, (((locals.var_vdsi_dn5 * assign88050_e134226) + (assign88050_e134223 * locals.var_vdsi_dn5)) / (2.0 * assign88050_e134234)), 0.0, (((locals.var_vdsi_dn7 * assign88050_e134226) + (assign88050_e134223 * locals.var_vdsi_dn7)) / (2.0 * assign88050_e134234)), 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign88050_e134236;
        locals.var_tmf2_dn0 = assign88050_e134236_d_n0;
        locals.var_tmf2_dn2 = assign88050_e134236_d_n2;
        locals.var_tmf2_dn4 = assign88050_e134236_d_n4;
        locals.var_tmf2_dn5 = assign88050_e134236_d_n5;
        locals.var_tmf2_dn6 = assign88050_e134236_d_n6;
        locals.var_tmf2_dn7 = assign88050_e134236_d_n7;
        locals.var_tmf2_dn8 = assign88050_e134236_d_n8;
        locals.var_tmf2_dn9 = assign88050_e134236_d_n9;
        locals.var_tmf2_dn10 = assign88050_e134236_d_n10;
        locals.var_tmf2_dn13 = assign88050_e134236_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign88060_e134256, assign88060_e134256_d_n0, assign88060_e134256_d_n2, assign88060_e134256_d_n4, assign88060_e134256_d_n5, assign88060_e134256_d_n6, assign88060_e134256_d_n7, assign88060_e134256_d_n8, assign88060_e134256_d_n9, assign88060_e134256_d_n10, assign88060_e134256_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2041 != 0.0)) && (locals.var_guard2042 == 0.0)) {
        let assign88060_e134250: f64 = (locals.var_vdsi + p.p137);
        let assign88060_e134252: f64 = (assign88060_e134250 / locals.var_tmf2);
        let assign88060_e134253: f64 = (1.0 + assign88060_e134252);
        let assign88060_e134254: f64 = (0.5 * assign88060_e134253);
        (assign88060_e134254, (0.5 * (-((assign88060_e134250 * locals.var_tmf2_dn0) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign88060_e134250 * locals.var_tmf2_dn2) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign88060_e134250 * locals.var_tmf2_dn4) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (((locals.var_vdsi_dn5 * locals.var_tmf2) - (assign88060_e134250 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (-((assign88060_e134250 * locals.var_tmf2_dn6) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (((locals.var_vdsi_dn7 * locals.var_tmf2) - (assign88060_e134250 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (-((assign88060_e134250 * locals.var_tmf2_dn8) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign88060_e134250 * locals.var_tmf2_dn9) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign88060_e134250 * locals.var_tmf2_dn10) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign88060_e134250 * locals.var_tmf2_dn13) / (locals.var_tmf2 * locals.var_tmf2)))),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign88060_e134256;
        locals.var_t9_dn0 = assign88060_e134256_d_n0;
        locals.var_t9_dn2 = assign88060_e134256_d_n2;
        locals.var_t9_dn4 = assign88060_e134256_d_n4;
        locals.var_t9_dn5 = assign88060_e134256_d_n5;
        locals.var_t9_dn6 = assign88060_e134256_d_n6;
        locals.var_t9_dn7 = assign88060_e134256_d_n7;
        locals.var_t9_dn8 = assign88060_e134256_d_n8;
        locals.var_t9_dn9 = assign88060_e134256_d_n9;
        locals.var_t9_dn10 = assign88060_e134256_d_n10;
        locals.var_t9_dn13 = assign88060_e134256_d_n13;
        locals.var_t9_rv = 0.0;

        let (assign88070_e134274, assign88070_e134274_d_n0, assign88070_e134274_d_n2, assign88070_e134274_d_n4, assign88070_e134274_d_n5, assign88070_e134274_d_n6, assign88070_e134274_d_n7, assign88070_e134274_d_n8, assign88070_e134274_d_n9, assign88070_e134274_d_n10, assign88070_e134274_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2041 != 0.0)) && (locals.var_guard2042 == 0.0)) {
        let assign88070_e134269: f64 = (locals.var_vdsi + p.p137);
        let assign88070_e134271: f64 = (assign88070_e134269 + locals.var_tmf2);
        let assign88070_e134272: f64 = (0.5 * assign88070_e134271);
        (assign88070_e134272, (0.5 * locals.var_tmf2_dn0), (0.5 * locals.var_tmf2_dn2), (0.5 * locals.var_tmf2_dn4), (0.5 * (locals.var_vdsi_dn5 + locals.var_tmf2_dn5)), (0.5 * locals.var_tmf2_dn6), (0.5 * (locals.var_vdsi_dn7 + locals.var_tmf2_dn7)), (0.5 * locals.var_tmf2_dn8), (0.5 * locals.var_tmf2_dn9), (0.5 * locals.var_tmf2_dn10), (0.5 * locals.var_tmf2_dn13),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign88070_e134274;
        locals.var_t2_dn0 = assign88070_e134274_d_n0;
        locals.var_t2_dn2 = assign88070_e134274_d_n2;
        locals.var_t2_dn4 = assign88070_e134274_d_n4;
        locals.var_t2_dn5 = assign88070_e134274_d_n5;
        locals.var_t2_dn6 = assign88070_e134274_d_n6;
        locals.var_t2_dn7 = assign88070_e134274_d_n7;
        locals.var_t2_dn8 = assign88070_e134274_d_n8;
        locals.var_t2_dn9 = assign88070_e134274_d_n9;
        locals.var_t2_dn10 = assign88070_e134274_d_n10;
        locals.var_t2_dn13 = assign88070_e134274_d_n13;
        locals.var_t2_rv = 0.0;

        let assign88080_e134277: f64 = if locals.var_t2 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2043 = assign88080_e134277;
        locals.var_guard2043_rv = 0.0;

        let (assign88090_e134291, assign88090_e134291_d_n0, assign88090_e134291_d_n2, assign88090_e134291_d_n4, assign88090_e134291_d_n5, assign88090_e134291_d_n6, assign88090_e134291_d_n7, assign88090_e134291_d_n8, assign88090_e134291_d_n9, assign88090_e134291_d_n10, assign88090_e134291_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2041 != 0.0)) && (locals.var_guard2042 == 0.0)) && (locals.var_guard2043 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign88090_e134291;
        locals.var_t2_dn0 = assign88090_e134291_d_n0;
        locals.var_t2_dn2 = assign88090_e134291_d_n2;
        locals.var_t2_dn4 = assign88090_e134291_d_n4;
        locals.var_t2_dn5 = assign88090_e134291_d_n5;
        locals.var_t2_dn6 = assign88090_e134291_d_n6;
        locals.var_t2_dn7 = assign88090_e134291_d_n7;
        locals.var_t2_dn8 = assign88090_e134291_d_n8;
        locals.var_t2_dn9 = assign88090_e134291_d_n9;
        locals.var_t2_dn10 = assign88090_e134291_d_n10;
        locals.var_t2_dn13 = assign88090_e134291_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign88100_e134305, assign88100_e134305_d_n0, assign88100_e134305_d_n2, assign88100_e134305_d_n4, assign88100_e134305_d_n5, assign88100_e134305_d_n6, assign88100_e134305_d_n7, assign88100_e134305_d_n8, assign88100_e134305_d_n9, assign88100_e134305_d_n10, assign88100_e134305_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2041 != 0.0)) && (locals.var_guard2042 == 0.0)) && (locals.var_guard2043 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign88100_e134305;
        locals.var_t9_dn0 = assign88100_e134305_d_n0;
        locals.var_t9_dn2 = assign88100_e134305_d_n2;
        locals.var_t9_dn4 = assign88100_e134305_d_n4;
        locals.var_t9_dn5 = assign88100_e134305_d_n5;
        locals.var_t9_dn6 = assign88100_e134305_d_n6;
        locals.var_t9_dn7 = assign88100_e134305_d_n7;
        locals.var_t9_dn8 = assign88100_e134305_d_n8;
        locals.var_t9_dn9 = assign88100_e134305_d_n9;
        locals.var_t9_dn10 = assign88100_e134305_d_n10;
        locals.var_t9_dn13 = assign88100_e134305_d_n13;
        locals.var_t9_rv = 0.0;

        let (assign88110_e134322, assign88110_e134322_d_n0, assign88110_e134322_d_n2, assign88110_e134322_d_n4, assign88110_e134322_d_n5, assign88110_e134322_d_n6, assign88110_e134322_d_n7, assign88110_e134322_d_n8, assign88110_e134322_d_n9, assign88110_e134322_d_n10, assign88110_e134322_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2041 != 0.0)) && (locals.var_guard2042 == 0.0)) {
        let assign88110_e134317: f64 = (locals.var_kjunc * locals.var_t2);
        let assign88110_e134318: f64 = (assign88110_e134317).sqrt();
        let assign88110_e134320: f64 = (assign88110_e134318 * p.p432);
        (assign88110_e134320, ((((locals.var_kjunc_dn0 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn0)) / (2.0 * assign88110_e134318)) * p.p432), ((((locals.var_kjunc_dn2 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn2)) / (2.0 * assign88110_e134318)) * p.p432), ((((locals.var_kjunc_dn4 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn4)) / (2.0 * assign88110_e134318)) * p.p432), ((((locals.var_kjunc_dn5 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn5)) / (2.0 * assign88110_e134318)) * p.p432), ((((locals.var_kjunc_dn6 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn6)) / (2.0 * assign88110_e134318)) * p.p432), ((((locals.var_kjunc_dn7 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn7)) / (2.0 * assign88110_e134318)) * p.p432), ((((locals.var_kjunc_dn8 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn8)) / (2.0 * assign88110_e134318)) * p.p432), ((((locals.var_kjunc_dn9 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn9)) / (2.0 * assign88110_e134318)) * p.p432), ((((locals.var_kjunc_dn10 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn10)) / (2.0 * assign88110_e134318)) * p.p432), ((((locals.var_kjunc_dn13 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn13)) / (2.0 * assign88110_e134318)) * p.p432),)
    } else {
        (locals.var_wjunc0, locals.var_wjunc0_dn0, locals.var_wjunc0_dn2, locals.var_wjunc0_dn4, locals.var_wjunc0_dn5, locals.var_wjunc0_dn6, locals.var_wjunc0_dn7, locals.var_wjunc0_dn8, locals.var_wjunc0_dn9, locals.var_wjunc0_dn10, locals.var_wjunc0_dn13,)
    }
};
        locals.var_wjunc0 = assign88110_e134322;
        locals.var_wjunc0_dn0 = assign88110_e134322_d_n0;
        locals.var_wjunc0_dn2 = assign88110_e134322_d_n2;
        locals.var_wjunc0_dn4 = assign88110_e134322_d_n4;
        locals.var_wjunc0_dn5 = assign88110_e134322_d_n5;
        locals.var_wjunc0_dn6 = assign88110_e134322_d_n6;
        locals.var_wjunc0_dn7 = assign88110_e134322_d_n7;
        locals.var_wjunc0_dn8 = assign88110_e134322_d_n8;
        locals.var_wjunc0_dn9 = assign88110_e134322_d_n9;
        locals.var_wjunc0_dn10 = assign88110_e134322_d_n10;
        locals.var_wjunc0_dn13 = assign88110_e134322_d_n13;
        locals.var_wjunc0_rv = 0.0;

        let (assign88120_e134336, assign88120_e134336_d_n0, assign88120_e134336_d_n2, assign88120_e134336_d_n4, assign88120_e134336_d_n5, assign88120_e134336_d_n6, assign88120_e134336_d_n7, assign88120_e134336_d_n8, assign88120_e134336_d_n9, assign88120_e134336_d_n10, assign88120_e134336_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2041 != 0.0)) && (locals.var_guard2042 == 0.0)) {
        let assign88120_e134334: f64 = (p.p334 - locals.var_wjunc0);
        (assign88120_e134334, (-locals.var_wjunc0_dn0), (-locals.var_wjunc0_dn2), (-locals.var_wjunc0_dn4), (-locals.var_wjunc0_dn5), (-locals.var_wjunc0_dn6), (-locals.var_wjunc0_dn7), (-locals.var_wjunc0_dn8), (-locals.var_wjunc0_dn9), (-locals.var_wjunc0_dn10), (-locals.var_wjunc0_dn13),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign88120_e134336;
        locals.var_t2_dn0 = assign88120_e134336_d_n0;
        locals.var_t2_dn2 = assign88120_e134336_d_n2;
        locals.var_t2_dn4 = assign88120_e134336_d_n4;
        locals.var_t2_dn5 = assign88120_e134336_d_n5;
        locals.var_t2_dn6 = assign88120_e134336_d_n6;
        locals.var_t2_dn7 = assign88120_e134336_d_n7;
        locals.var_t2_dn8 = assign88120_e134336_d_n8;
        locals.var_t2_dn9 = assign88120_e134336_d_n9;
        locals.var_t2_dn10 = assign88120_e134336_d_n10;
        locals.var_t2_dn13 = assign88120_e134336_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign88130_e134358, assign88130_e134358_d_n0, assign88130_e134358_d_n2, assign88130_e134358_d_n4, assign88130_e134358_d_n5, assign88130_e134358_d_n6, assign88130_e134358_d_n7, assign88130_e134358_d_n8, assign88130_e134358_d_n9, assign88130_e134358_d_n10, assign88130_e134358_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2041 != 0.0)) {
        let assign88130_e134345: f64 = (locals.var_t2 * locals.var_t2);
        let assign88130_e134349: f64 = (p.p334 * 0.01);
        let assign88130_e134350: f64 = (4.0 * assign88130_e134349);
        let assign88130_e134353: f64 = (p.p334 * 0.01);
        let assign88130_e134354: f64 = (assign88130_e134350 * assign88130_e134353);
        let assign88130_e134355: f64 = (assign88130_e134345 + assign88130_e134354);
        let assign88130_e134356: f64 = (assign88130_e134355).sqrt();
        (assign88130_e134356, (((locals.var_t2_dn0 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn0)) / (2.0 * assign88130_e134356)), (((locals.var_t2_dn2 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn2)) / (2.0 * assign88130_e134356)), (((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4)) / (2.0 * assign88130_e134356)), (((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5)) / (2.0 * assign88130_e134356)), (((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6)) / (2.0 * assign88130_e134356)), (((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7)) / (2.0 * assign88130_e134356)), (((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8)) / (2.0 * assign88130_e134356)), (((locals.var_t2_dn9 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn9)) / (2.0 * assign88130_e134356)), (((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10)) / (2.0 * assign88130_e134356)), (((locals.var_t2_dn13 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn13)) / (2.0 * assign88130_e134356)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign88130_e134358;
        locals.var_tmf2_dn0 = assign88130_e134358_d_n0;
        locals.var_tmf2_dn2 = assign88130_e134358_d_n2;
        locals.var_tmf2_dn4 = assign88130_e134358_d_n4;
        locals.var_tmf2_dn5 = assign88130_e134358_d_n5;
        locals.var_tmf2_dn6 = assign88130_e134358_d_n6;
        locals.var_tmf2_dn7 = assign88130_e134358_d_n7;
        locals.var_tmf2_dn8 = assign88130_e134358_d_n8;
        locals.var_tmf2_dn9 = assign88130_e134358_d_n9;
        locals.var_tmf2_dn10 = assign88130_e134358_d_n10;
        locals.var_tmf2_dn13 = assign88130_e134358_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign88140_e134373, assign88140_e134373_d_n0, assign88140_e134373_d_n2, assign88140_e134373_d_n4, assign88140_e134373_d_n5, assign88140_e134373_d_n6, assign88140_e134373_d_n7, assign88140_e134373_d_n8, assign88140_e134373_d_n9, assign88140_e134373_d_n10, assign88140_e134373_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2041 != 0.0)) {
        let assign88140_e134369: f64 = (locals.var_t2 / locals.var_tmf2);
        let assign88140_e134370: f64 = (1.0 + assign88140_e134369);
        let assign88140_e134371: f64 = (0.5 * assign88140_e134370);
        (assign88140_e134371, (0.5 * (((locals.var_t2_dn0 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn2 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn4 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn5 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn6 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn7 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn8 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn9 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn10 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn13 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign88140_e134373;
        locals.var_t9_dn0 = assign88140_e134373_d_n0;
        locals.var_t9_dn2 = assign88140_e134373_d_n2;
        locals.var_t9_dn4 = assign88140_e134373_d_n4;
        locals.var_t9_dn5 = assign88140_e134373_d_n5;
        locals.var_t9_dn6 = assign88140_e134373_d_n6;
        locals.var_t9_dn7 = assign88140_e134373_d_n7;
        locals.var_t9_dn8 = assign88140_e134373_d_n8;
        locals.var_t9_dn9 = assign88140_e134373_d_n9;
        locals.var_t9_dn10 = assign88140_e134373_d_n10;
        locals.var_t9_dn13 = assign88140_e134373_d_n13;
        locals.var_t9_rv = 0.0;

        let (assign88150_e134386, assign88150_e134386_d_n0, assign88150_e134386_d_n2, assign88150_e134386_d_n4, assign88150_e134386_d_n5, assign88150_e134386_d_n6, assign88150_e134386_d_n7, assign88150_e134386_d_n8, assign88150_e134386_d_n9, assign88150_e134386_d_n10, assign88150_e134386_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2041 != 0.0)) {
        let assign88150_e134383: f64 = (locals.var_t2 + locals.var_tmf2);
        let assign88150_e134384: f64 = (0.5 * assign88150_e134383);
        (assign88150_e134384, (0.5 * (locals.var_t2_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_t2_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_t2_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_t2_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_t2_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_t2_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_t2_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_t2_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_t2_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_t2_dn13 + locals.var_tmf2_dn13)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign88150_e134386;
        locals.var_t2_dn0 = assign88150_e134386_d_n0;
        locals.var_t2_dn2 = assign88150_e134386_d_n2;
        locals.var_t2_dn4 = assign88150_e134386_d_n4;
        locals.var_t2_dn5 = assign88150_e134386_d_n5;
        locals.var_t2_dn6 = assign88150_e134386_d_n6;
        locals.var_t2_dn7 = assign88150_e134386_d_n7;
        locals.var_t2_dn8 = assign88150_e134386_d_n8;
        locals.var_t2_dn9 = assign88150_e134386_d_n9;
        locals.var_t2_dn10 = assign88150_e134386_d_n10;
        locals.var_t2_dn13 = assign88150_e134386_d_n13;
        locals.var_t2_rv = 0.0;

        let assign88160_e134389: f64 = if locals.var_t2 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2044 = assign88160_e134389;
        locals.var_guard2044_rv = 0.0;

        let (assign88170_e134400, assign88170_e134400_d_n0, assign88170_e134400_d_n2, assign88170_e134400_d_n4, assign88170_e134400_d_n5, assign88170_e134400_d_n6, assign88170_e134400_d_n7, assign88170_e134400_d_n8, assign88170_e134400_d_n9, assign88170_e134400_d_n10, assign88170_e134400_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2041 != 0.0)) && (locals.var_guard2044 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign88170_e134400;
        locals.var_t2_dn0 = assign88170_e134400_d_n0;
        locals.var_t2_dn2 = assign88170_e134400_d_n2;
        locals.var_t2_dn4 = assign88170_e134400_d_n4;
        locals.var_t2_dn5 = assign88170_e134400_d_n5;
        locals.var_t2_dn6 = assign88170_e134400_d_n6;
        locals.var_t2_dn7 = assign88170_e134400_d_n7;
        locals.var_t2_dn8 = assign88170_e134400_d_n8;
        locals.var_t2_dn9 = assign88170_e134400_d_n9;
        locals.var_t2_dn10 = assign88170_e134400_d_n10;
        locals.var_t2_dn13 = assign88170_e134400_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign88180_e134411, assign88180_e134411_d_n0, assign88180_e134411_d_n2, assign88180_e134411_d_n4, assign88180_e134411_d_n5, assign88180_e134411_d_n6, assign88180_e134411_d_n7, assign88180_e134411_d_n8, assign88180_e134411_d_n9, assign88180_e134411_d_n10, assign88180_e134411_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2041 != 0.0)) && (locals.var_guard2044 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign88180_e134411;
        locals.var_t9_dn0 = assign88180_e134411_d_n0;
        locals.var_t9_dn2 = assign88180_e134411_d_n2;
        locals.var_t9_dn4 = assign88180_e134411_d_n4;
        locals.var_t9_dn5 = assign88180_e134411_d_n5;
        locals.var_t9_dn6 = assign88180_e134411_d_n6;
        locals.var_t9_dn7 = assign88180_e134411_d_n7;
        locals.var_t9_dn8 = assign88180_e134411_d_n8;
        locals.var_t9_dn9 = assign88180_e134411_d_n9;
        locals.var_t9_dn10 = assign88180_e134411_d_n10;
        locals.var_t9_dn13 = assign88180_e134411_d_n13;
        locals.var_t9_rv = 0.0;

        let (assign88190_e134420, assign88190_e134420_d_n0, assign88190_e134420_d_n2, assign88190_e134420_d_n4, assign88190_e134420_d_n5, assign88190_e134420_d_n6, assign88190_e134420_d_n7, assign88190_e134420_d_n8, assign88190_e134420_d_n9, assign88190_e134420_d_n10, assign88190_e134420_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2041 != 0.0)) {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    } else {
        (locals.var_ddriftldc, locals.var_ddriftldc_dn0, locals.var_ddriftldc_dn2, locals.var_ddriftldc_dn4, locals.var_ddriftldc_dn5, locals.var_ddriftldc_dn6, locals.var_ddriftldc_dn7, locals.var_ddriftldc_dn8, locals.var_ddriftldc_dn9, locals.var_ddriftldc_dn10, locals.var_ddriftldc_dn13,)
    }
};
        locals.var_ddriftldc = assign88190_e134420;
        locals.var_ddriftldc_dn0 = assign88190_e134420_d_n0;
        locals.var_ddriftldc_dn2 = assign88190_e134420_d_n2;
        locals.var_ddriftldc_dn4 = assign88190_e134420_d_n4;
        locals.var_ddriftldc_dn5 = assign88190_e134420_d_n5;
        locals.var_ddriftldc_dn6 = assign88190_e134420_d_n6;
        locals.var_ddriftldc_dn7 = assign88190_e134420_d_n7;
        locals.var_ddriftldc_dn8 = assign88190_e134420_d_n8;
        locals.var_ddriftldc_dn9 = assign88190_e134420_d_n9;
        locals.var_ddriftldc_dn10 = assign88190_e134420_d_n10;
        locals.var_ddriftldc_dn13 = assign88190_e134420_d_n13;
        locals.var_ddriftldc_rv = 0.0;

        let (assign88200_e134437, assign88200_e134437_d_n0, assign88200_e134437_d_n2, assign88200_e134437_d_n4, assign88200_e134437_d_n5, assign88200_e134437_d_n6, assign88200_e134437_d_n7, assign88200_e134437_d_n8, assign88200_e134437_d_n9, assign88200_e134437_d_n10, assign88200_e134437_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2041 != 0.0)) {
        let assign88200_e134429: f64 = (locals.var_q_nsubld__blk2004 * locals.var_ddriftldc);
        let assign88200_e134431: f64 = (assign88200_e134429 * locals.var_ddriftldc);
        let assign88200_e134433: f64 = (assign88200_e134431 / 2.0);
        let assign88200_e134435: f64 = (assign88200_e134433 / 1.034943e-10);
        (assign88200_e134435, (((((locals.var_q_nsubld__blk2004 * locals.var_ddriftldc_dn0) * locals.var_ddriftldc) + (assign88200_e134429 * locals.var_ddriftldc_dn0)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2004 * locals.var_ddriftldc_dn2) * locals.var_ddriftldc) + (assign88200_e134429 * locals.var_ddriftldc_dn2)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2004 * locals.var_ddriftldc_dn4) * locals.var_ddriftldc) + (assign88200_e134429 * locals.var_ddriftldc_dn4)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2004 * locals.var_ddriftldc_dn5) * locals.var_ddriftldc) + (assign88200_e134429 * locals.var_ddriftldc_dn5)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2004 * locals.var_ddriftldc_dn6) * locals.var_ddriftldc) + (assign88200_e134429 * locals.var_ddriftldc_dn6)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2004 * locals.var_ddriftldc_dn7) * locals.var_ddriftldc) + (assign88200_e134429 * locals.var_ddriftldc_dn7)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2004 * locals.var_ddriftldc_dn8) * locals.var_ddriftldc) + (assign88200_e134429 * locals.var_ddriftldc_dn8)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2004 * locals.var_ddriftldc_dn9) * locals.var_ddriftldc) + (assign88200_e134429 * locals.var_ddriftldc_dn9)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2004 * locals.var_ddriftldc_dn10) * locals.var_ddriftldc) + (assign88200_e134429 * locals.var_ddriftldc_dn10)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2004 * locals.var_ddriftldc_dn13) * locals.var_ddriftldc) + (assign88200_e134429 * locals.var_ddriftldc_dn13)) / 2.0) / 1.034943e-10),)
    } else {
        (locals.var_dphi_sb, locals.var_dphi_sb_dn0, locals.var_dphi_sb_dn2, locals.var_dphi_sb_dn4, locals.var_dphi_sb_dn5, locals.var_dphi_sb_dn6, locals.var_dphi_sb_dn7, locals.var_dphi_sb_dn8, locals.var_dphi_sb_dn9, locals.var_dphi_sb_dn10, locals.var_dphi_sb_dn13,)
    }
};
        locals.var_dphi_sb = assign88200_e134437;
        locals.var_dphi_sb_dn0 = assign88200_e134437_d_n0;
        locals.var_dphi_sb_dn2 = assign88200_e134437_d_n2;
        locals.var_dphi_sb_dn4 = assign88200_e134437_d_n4;
        locals.var_dphi_sb_dn5 = assign88200_e134437_d_n5;
        locals.var_dphi_sb_dn6 = assign88200_e134437_d_n6;
        locals.var_dphi_sb_dn7 = assign88200_e134437_d_n7;
        locals.var_dphi_sb_dn8 = assign88200_e134437_d_n8;
        locals.var_dphi_sb_dn9 = assign88200_e134437_d_n9;
        locals.var_dphi_sb_dn10 = assign88200_e134437_d_n10;
        locals.var_dphi_sb_dn13 = assign88200_e134437_d_n13;
        locals.var_dphi_sb_rv = 0.0;

        let (assign88210_e134451, assign88210_e134451_d_n0, assign88210_e134451_d_n2, assign88210_e134451_d_n4, assign88210_e134451_d_n5, assign88210_e134451_d_n6, assign88210_e134451_d_n7, assign88210_e134451_d_n8, assign88210_e134451_d_n9, assign88210_e134451_d_n10, assign88210_e134451_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2041 != 0.0)) {
        let assign88210_e134446: f64 = (2.0 * locals.var_beta);
        let assign88210_e134448: f64 = (assign88210_e134446 * locals.var_dphi_sb);
        let assign88210_e134449: f64 = (assign88210_e134448).sqrt();
        (assign88210_e134449, ((((2.0 * locals.var_beta_dn0) * locals.var_dphi_sb) + (assign88210_e134446 * locals.var_dphi_sb_dn0)) / (2.0 * assign88210_e134449)), ((((2.0 * locals.var_beta_dn2) * locals.var_dphi_sb) + (assign88210_e134446 * locals.var_dphi_sb_dn2)) / (2.0 * assign88210_e134449)), ((((2.0 * locals.var_beta_dn4) * locals.var_dphi_sb) + (assign88210_e134446 * locals.var_dphi_sb_dn4)) / (2.0 * assign88210_e134449)), ((((2.0 * locals.var_beta_dn5) * locals.var_dphi_sb) + (assign88210_e134446 * locals.var_dphi_sb_dn5)) / (2.0 * assign88210_e134449)), ((((2.0 * locals.var_beta_dn6) * locals.var_dphi_sb) + (assign88210_e134446 * locals.var_dphi_sb_dn6)) / (2.0 * assign88210_e134449)), ((((2.0 * locals.var_beta_dn7) * locals.var_dphi_sb) + (assign88210_e134446 * locals.var_dphi_sb_dn7)) / (2.0 * assign88210_e134449)), ((((2.0 * locals.var_beta_dn8) * locals.var_dphi_sb) + (assign88210_e134446 * locals.var_dphi_sb_dn8)) / (2.0 * assign88210_e134449)), ((((2.0 * locals.var_beta_dn9) * locals.var_dphi_sb) + (assign88210_e134446 * locals.var_dphi_sb_dn9)) / (2.0 * assign88210_e134449)), ((((2.0 * locals.var_beta_dn10) * locals.var_dphi_sb) + (assign88210_e134446 * locals.var_dphi_sb_dn10)) / (2.0 * assign88210_e134449)), ((((2.0 * locals.var_beta_dn13) * locals.var_dphi_sb) + (assign88210_e134446 * locals.var_dphi_sb_dn13)) / (2.0 * assign88210_e134449)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign88210_e134451;
        locals.var_t0_dn0 = assign88210_e134451_d_n0;
        locals.var_t0_dn2 = assign88210_e134451_d_n2;
        locals.var_t0_dn4 = assign88210_e134451_d_n4;
        locals.var_t0_dn5 = assign88210_e134451_d_n5;
        locals.var_t0_dn6 = assign88210_e134451_d_n6;
        locals.var_t0_dn7 = assign88210_e134451_d_n7;
        locals.var_t0_dn8 = assign88210_e134451_d_n8;
        locals.var_t0_dn9 = assign88210_e134451_d_n9;
        locals.var_t0_dn10 = assign88210_e134451_d_n10;
        locals.var_t0_dn13 = assign88210_e134451_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign88220_e134467, assign88220_e134467_d_n0, assign88220_e134467_d_n2, assign88220_e134467_d_n4, assign88220_e134467_d_n5, assign88220_e134467_d_n6, assign88220_e134467_d_n7, assign88220_e134467_d_n8, assign88220_e134467_d_n9, assign88220_e134467_d_n10, assign88220_e134467_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2041 != 0.0)) {
        let assign88220_e134459: f64 = { let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign88220_e134461: f64 = (-locals.var_t0);
        let assign88220_e134462: f64 = { let limited_exp_arg = assign88220_e134461; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign88220_e134463: f64 = (assign88220_e134459 + assign88220_e134462);
        let assign88220_e134465: f64 = (assign88220_e134463 / 2.0);
        (assign88220_e134465, ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn0) + ({ let limited_exp_arg = assign88220_e134461; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn0))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn2) + ({ let limited_exp_arg = assign88220_e134461; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn2))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn4) + ({ let limited_exp_arg = assign88220_e134461; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn4))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn5) + ({ let limited_exp_arg = assign88220_e134461; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn5))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn6) + ({ let limited_exp_arg = assign88220_e134461; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn6))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn7) + ({ let limited_exp_arg = assign88220_e134461; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn7))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn8) + ({ let limited_exp_arg = assign88220_e134461; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn8))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn9) + ({ let limited_exp_arg = assign88220_e134461; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn9))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn10) + ({ let limited_exp_arg = assign88220_e134461; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn10))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn13) + ({ let limited_exp_arg = assign88220_e134461; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn13))) / 2.0),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign88220_e134467;
        locals.var_t1_dn0 = assign88220_e134467_d_n0;
        locals.var_t1_dn2 = assign88220_e134467_d_n2;
        locals.var_t1_dn4 = assign88220_e134467_d_n4;
        locals.var_t1_dn5 = assign88220_e134467_d_n5;
        locals.var_t1_dn6 = assign88220_e134467_d_n6;
        locals.var_t1_dn7 = assign88220_e134467_d_n7;
        locals.var_t1_dn8 = assign88220_e134467_d_n8;
        locals.var_t1_dn9 = assign88220_e134467_d_n9;
        locals.var_t1_dn10 = assign88220_e134467_d_n10;
        locals.var_t1_dn13 = assign88220_e134467_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign88230_e134479, assign88230_e134479_d_n0, assign88230_e134479_d_n2, assign88230_e134479_d_n4, assign88230_e134479_d_n5, assign88230_e134479_d_n6, assign88230_e134479_d_n7, assign88230_e134479_d_n8, assign88230_e134479_d_n9, assign88230_e134479_d_n10, assign88230_e134479_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2041 != 0.0)) {
        let assign88230_e134475: f64 = (locals.var_t1).ln();
        let assign88230_e134477: f64 = (assign88230_e134475 / locals.var_dphi_sb);
        (assign88230_e134477, ((((locals.var_t1_dn0 / locals.var_t1) * locals.var_dphi_sb) - (assign88230_e134475 * locals.var_dphi_sb_dn0)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn2 / locals.var_t1) * locals.var_dphi_sb) - (assign88230_e134475 * locals.var_dphi_sb_dn2)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn4 / locals.var_t1) * locals.var_dphi_sb) - (assign88230_e134475 * locals.var_dphi_sb_dn4)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn5 / locals.var_t1) * locals.var_dphi_sb) - (assign88230_e134475 * locals.var_dphi_sb_dn5)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn6 / locals.var_t1) * locals.var_dphi_sb) - (assign88230_e134475 * locals.var_dphi_sb_dn6)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn7 / locals.var_t1) * locals.var_dphi_sb) - (assign88230_e134475 * locals.var_dphi_sb_dn7)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn8 / locals.var_t1) * locals.var_dphi_sb) - (assign88230_e134475 * locals.var_dphi_sb_dn8)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn9 / locals.var_t1) * locals.var_dphi_sb) - (assign88230_e134475 * locals.var_dphi_sb_dn9)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn10 / locals.var_t1) * locals.var_dphi_sb) - (assign88230_e134475 * locals.var_dphi_sb_dn10)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn13 / locals.var_t1) * locals.var_dphi_sb) - (assign88230_e134475 * locals.var_dphi_sb_dn13)) / (locals.var_dphi_sb * locals.var_dphi_sb)),)
    } else {
        (locals.var_c_sb, locals.var_c_sb_dn0, locals.var_c_sb_dn2, locals.var_c_sb_dn4, locals.var_c_sb_dn5, locals.var_c_sb_dn6, locals.var_c_sb_dn7, locals.var_c_sb_dn8, locals.var_c_sb_dn9, locals.var_c_sb_dn10, locals.var_c_sb_dn13,)
    }
};
        locals.var_c_sb = assign88230_e134479;
        locals.var_c_sb_dn0 = assign88230_e134479_d_n0;
        locals.var_c_sb_dn2 = assign88230_e134479_d_n2;
        locals.var_c_sb_dn4 = assign88230_e134479_d_n4;
        locals.var_c_sb_dn5 = assign88230_e134479_d_n5;
        locals.var_c_sb_dn6 = assign88230_e134479_d_n6;
        locals.var_c_sb_dn7 = assign88230_e134479_d_n7;
        locals.var_c_sb_dn8 = assign88230_e134479_d_n8;
        locals.var_c_sb_dn9 = assign88230_e134479_d_n9;
        locals.var_c_sb_dn10 = assign88230_e134479_d_n10;
        locals.var_c_sb_dn13 = assign88230_e134479_d_n13;
        locals.var_c_sb_rv = 0.0;

        let (assign88240_e134490, assign88240_e134490_d_n0, assign88240_e134490_d_n2, assign88240_e134490_d_n4, assign88240_e134490_d_n5, assign88240_e134490_d_n6, assign88240_e134490_d_n7, assign88240_e134490_d_n8, assign88240_e134490_d_n9, assign88240_e134490_d_n10, assign88240_e134490_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2041 != 0.0)) {
        let assign88240_e134488: f64 = (locals.var_ps0ld + locals.var_vxbgmtcl);
        (assign88240_e134488, (locals.var_ps0ld_dn0 + locals.var_vxbgmtcl_dn0), (locals.var_ps0ld_dn2 + locals.var_vxbgmtcl_dn2), (locals.var_ps0ld_dn4 + locals.var_vxbgmtcl_dn4), (locals.var_ps0ld_dn5 + locals.var_vxbgmtcl_dn5), (locals.var_ps0ld_dn6 + locals.var_vxbgmtcl_dn6), (locals.var_ps0ld_dn7 + locals.var_vxbgmtcl_dn7), (locals.var_ps0ld_dn8 + locals.var_vxbgmtcl_dn8), (locals.var_ps0ld_dn9 + locals.var_vxbgmtcl_dn9), (locals.var_ps0ld_dn10 + locals.var_vxbgmtcl_dn10), (locals.var_ps0ld_dn13 + locals.var_vxbgmtcl_dn13),)
    } else {
        (locals.var_ps0ld_vxb, locals.var_ps0ld_vxb_dn0, locals.var_ps0ld_vxb_dn2, locals.var_ps0ld_vxb_dn4, locals.var_ps0ld_vxb_dn5, locals.var_ps0ld_vxb_dn6, locals.var_ps0ld_vxb_dn7, locals.var_ps0ld_vxb_dn8, locals.var_ps0ld_vxb_dn9, locals.var_ps0ld_vxb_dn10, locals.var_ps0ld_vxb_dn13,)
    }
};
        locals.var_ps0ld_vxb = assign88240_e134490;
        locals.var_ps0ld_vxb_dn0 = assign88240_e134490_d_n0;
        locals.var_ps0ld_vxb_dn2 = assign88240_e134490_d_n2;
        locals.var_ps0ld_vxb_dn4 = assign88240_e134490_d_n4;
        locals.var_ps0ld_vxb_dn5 = assign88240_e134490_d_n5;
        locals.var_ps0ld_vxb_dn6 = assign88240_e134490_d_n6;
        locals.var_ps0ld_vxb_dn7 = assign88240_e134490_d_n7;
        locals.var_ps0ld_vxb_dn8 = assign88240_e134490_d_n8;
        locals.var_ps0ld_vxb_dn9 = assign88240_e134490_d_n9;
        locals.var_ps0ld_vxb_dn10 = assign88240_e134490_d_n10;
        locals.var_ps0ld_vxb_dn13 = assign88240_e134490_d_n13;
        locals.var_ps0ld_vxb_rv = 0.0;

        let (assign88250_e134503, assign88250_e134503_d_n0, assign88250_e134503_d_n2, assign88250_e134503_d_n4, assign88250_e134503_d_n5, assign88250_e134503_d_n6, assign88250_e134503_d_n7, assign88250_e134503_d_n8, assign88250_e134503_d_n9, assign88250_e134503_d_n10, assign88250_e134503_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2041 != 0.0)) {
        let assign88250_e134500: f64 = (locals.var_ps0ld_vxb - locals.var_dphi_sb);
        let assign88250_e134501: f64 = (locals.var_c_sb * assign88250_e134500);
        (assign88250_e134501, ((locals.var_c_sb_dn0 * assign88250_e134500) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn0 - locals.var_dphi_sb_dn0))), ((locals.var_c_sb_dn2 * assign88250_e134500) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn2 - locals.var_dphi_sb_dn2))), ((locals.var_c_sb_dn4 * assign88250_e134500) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn4 - locals.var_dphi_sb_dn4))), ((locals.var_c_sb_dn5 * assign88250_e134500) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn5 - locals.var_dphi_sb_dn5))), ((locals.var_c_sb_dn6 * assign88250_e134500) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn6 - locals.var_dphi_sb_dn6))), ((locals.var_c_sb_dn7 * assign88250_e134500) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn7 - locals.var_dphi_sb_dn7))), ((locals.var_c_sb_dn8 * assign88250_e134500) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn8 - locals.var_dphi_sb_dn8))), ((locals.var_c_sb_dn9 * assign88250_e134500) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn9 - locals.var_dphi_sb_dn9))), ((locals.var_c_sb_dn10 * assign88250_e134500) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn10 - locals.var_dphi_sb_dn10))), ((locals.var_c_sb_dn13 * assign88250_e134500) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn13 - locals.var_dphi_sb_dn13))),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn4, locals.var_ty_dn5, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn8, locals.var_ty_dn9, locals.var_ty_dn10, locals.var_ty_dn13,)
    }
};
        locals.var_ty = assign88250_e134503;
        locals.var_ty_dn0 = assign88250_e134503_d_n0;
        locals.var_ty_dn2 = assign88250_e134503_d_n2;
        locals.var_ty_dn4 = assign88250_e134503_d_n4;
        locals.var_ty_dn5 = assign88250_e134503_d_n5;
        locals.var_ty_dn6 = assign88250_e134503_d_n6;
        locals.var_ty_dn7 = assign88250_e134503_d_n7;
        locals.var_ty_dn8 = assign88250_e134503_d_n8;
        locals.var_ty_dn9 = assign88250_e134503_d_n9;
        locals.var_ty_dn10 = assign88250_e134503_d_n10;
        locals.var_ty_dn13 = assign88250_e134503_d_n13;
        locals.var_ty_rv = 0.0;

        let assign88260_e134506: f64 = if locals.var_ty < 60.0 { 1.0 } else { 0.0 };
        locals.var_guard2045 = assign88260_e134506;
        locals.var_guard2045_rv = 0.0;

        let (assign88270_e134518, assign88270_e134518_d_n0, assign88270_e134518_d_n2, assign88270_e134518_d_n4, assign88270_e134518_d_n5, assign88270_e134518_d_n6, assign88270_e134518_d_n7, assign88270_e134518_d_n8, assign88270_e134518_d_n9, assign88270_e134518_d_n10, assign88270_e134518_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2041 != 0.0)) && (locals.var_guard2045 != 0.0)) {
        let assign88270_e134516: f64 = (locals.var_ty).exp();
        (assign88270_e134516, (assign88270_e134516 * locals.var_ty_dn0), (assign88270_e134516 * locals.var_ty_dn2), (assign88270_e134516 * locals.var_ty_dn4), (assign88270_e134516 * locals.var_ty_dn5), (assign88270_e134516 * locals.var_ty_dn6), (assign88270_e134516 * locals.var_ty_dn7), (assign88270_e134516 * locals.var_ty_dn8), (assign88270_e134516 * locals.var_ty_dn9), (assign88270_e134516 * locals.var_ty_dn10), (assign88270_e134516 * locals.var_ty_dn13),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign88270_e134518;
        locals.var_t1_dn0 = assign88270_e134518_d_n0;
        locals.var_t1_dn2 = assign88270_e134518_d_n2;
        locals.var_t1_dn4 = assign88270_e134518_d_n4;
        locals.var_t1_dn5 = assign88270_e134518_d_n5;
        locals.var_t1_dn6 = assign88270_e134518_d_n6;
        locals.var_t1_dn7 = assign88270_e134518_d_n7;
        locals.var_t1_dn8 = assign88270_e134518_d_n8;
        locals.var_t1_dn9 = assign88270_e134518_d_n9;
        locals.var_t1_dn10 = assign88270_e134518_d_n10;
        locals.var_t1_dn13 = assign88270_e134518_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign88280_e134533, assign88280_e134533_d_n0, assign88280_e134533_d_n2, assign88280_e134533_d_n4, assign88280_e134533_d_n5, assign88280_e134533_d_n6, assign88280_e134533_d_n7, assign88280_e134533_d_n8, assign88280_e134533_d_n9, assign88280_e134533_d_n10, assign88280_e134533_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2041 != 0.0)) && (locals.var_guard2045 != 0.0)) {
        let assign88280_e134528: f64 = (-locals.var_c_sb);
        let assign88280_e134530: f64 = (assign88280_e134528 * locals.var_dphi_sb);
        let assign88280_e134531: f64 = (assign88280_e134530).exp();
        (assign88280_e134531, (assign88280_e134531 * (((-locals.var_c_sb_dn0) * locals.var_dphi_sb) + (assign88280_e134528 * locals.var_dphi_sb_dn0))), (assign88280_e134531 * (((-locals.var_c_sb_dn2) * locals.var_dphi_sb) + (assign88280_e134528 * locals.var_dphi_sb_dn2))), (assign88280_e134531 * (((-locals.var_c_sb_dn4) * locals.var_dphi_sb) + (assign88280_e134528 * locals.var_dphi_sb_dn4))), (assign88280_e134531 * (((-locals.var_c_sb_dn5) * locals.var_dphi_sb) + (assign88280_e134528 * locals.var_dphi_sb_dn5))), (assign88280_e134531 * (((-locals.var_c_sb_dn6) * locals.var_dphi_sb) + (assign88280_e134528 * locals.var_dphi_sb_dn6))), (assign88280_e134531 * (((-locals.var_c_sb_dn7) * locals.var_dphi_sb) + (assign88280_e134528 * locals.var_dphi_sb_dn7))), (assign88280_e134531 * (((-locals.var_c_sb_dn8) * locals.var_dphi_sb) + (assign88280_e134528 * locals.var_dphi_sb_dn8))), (assign88280_e134531 * (((-locals.var_c_sb_dn9) * locals.var_dphi_sb) + (assign88280_e134528 * locals.var_dphi_sb_dn9))), (assign88280_e134531 * (((-locals.var_c_sb_dn10) * locals.var_dphi_sb) + (assign88280_e134528 * locals.var_dphi_sb_dn10))), (assign88280_e134531 * (((-locals.var_c_sb_dn13) * locals.var_dphi_sb) + (assign88280_e134528 * locals.var_dphi_sb_dn13))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign88280_e134533;
        locals.var_t0_dn0 = assign88280_e134533_d_n0;
        locals.var_t0_dn2 = assign88280_e134533_d_n2;
        locals.var_t0_dn4 = assign88280_e134533_d_n4;
        locals.var_t0_dn5 = assign88280_e134533_d_n5;
        locals.var_t0_dn6 = assign88280_e134533_d_n6;
        locals.var_t0_dn7 = assign88280_e134533_d_n7;
        locals.var_t0_dn8 = assign88280_e134533_d_n8;
        locals.var_t0_dn9 = assign88280_e134533_d_n9;
        locals.var_t0_dn10 = assign88280_e134533_d_n10;
        locals.var_t0_dn13 = assign88280_e134533_d_n13;
        locals.var_t0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_327(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign88290_e134546, assign88290_e134546_d_n0, assign88290_e134546_d_n2, assign88290_e134546_d_n4, assign88290_e134546_d_n5, assign88290_e134546_d_n6, assign88290_e134546_d_n7, assign88290_e134546_d_n8, assign88290_e134546_d_n9, assign88290_e134546_d_n10, assign88290_e134546_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2041 != 0.0)) && (locals.var_guard2045 != 0.0)) {
        let assign88290_e134544: f64 = (locals.var_t1 - locals.var_t0);
        (assign88290_e134544, (locals.var_t1_dn0 - locals.var_t0_dn0), (locals.var_t1_dn2 - locals.var_t0_dn2), (locals.var_t1_dn4 - locals.var_t0_dn4), (locals.var_t1_dn5 - locals.var_t0_dn5), (locals.var_t1_dn6 - locals.var_t0_dn6), (locals.var_t1_dn7 - locals.var_t0_dn7), (locals.var_t1_dn8 - locals.var_t0_dn8), (locals.var_t1_dn9 - locals.var_t0_dn9), (locals.var_t1_dn10 - locals.var_t0_dn10), (locals.var_t1_dn13 - locals.var_t0_dn13),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign88290_e134546;
        locals.var_t2_dn0 = assign88290_e134546_d_n0;
        locals.var_t2_dn2 = assign88290_e134546_d_n2;
        locals.var_t2_dn4 = assign88290_e134546_d_n4;
        locals.var_t2_dn5 = assign88290_e134546_d_n5;
        locals.var_t2_dn6 = assign88290_e134546_d_n6;
        locals.var_t2_dn7 = assign88290_e134546_d_n7;
        locals.var_t2_dn8 = assign88290_e134546_d_n8;
        locals.var_t2_dn9 = assign88290_e134546_d_n9;
        locals.var_t2_dn10 = assign88290_e134546_d_n10;
        locals.var_t2_dn13 = assign88290_e134546_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign88300_e134562, assign88300_e134562_d_n0, assign88300_e134562_d_n2, assign88300_e134562_d_n4, assign88300_e134562_d_n5, assign88300_e134562_d_n6, assign88300_e134562_d_n7, assign88300_e134562_d_n8, assign88300_e134562_d_n9, assign88300_e134562_d_n10, assign88300_e134562_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2041 != 0.0)) && (locals.var_guard2045 != 0.0)) {
        let assign88300_e134557: f64 = (1.0 + locals.var_t2);
        let assign88300_e134558: f64 = (assign88300_e134557).ln();
        let assign88300_e134560: f64 = (assign88300_e134558 / locals.var_c_sb);
        (assign88300_e134560, ((((locals.var_t2_dn0 / assign88300_e134557) * locals.var_c_sb) - (assign88300_e134558 * locals.var_c_sb_dn0)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn2 / assign88300_e134557) * locals.var_c_sb) - (assign88300_e134558 * locals.var_c_sb_dn2)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn4 / assign88300_e134557) * locals.var_c_sb) - (assign88300_e134558 * locals.var_c_sb_dn4)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn5 / assign88300_e134557) * locals.var_c_sb) - (assign88300_e134558 * locals.var_c_sb_dn5)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn6 / assign88300_e134557) * locals.var_c_sb) - (assign88300_e134558 * locals.var_c_sb_dn6)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn7 / assign88300_e134557) * locals.var_c_sb) - (assign88300_e134558 * locals.var_c_sb_dn7)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn8 / assign88300_e134557) * locals.var_c_sb) - (assign88300_e134558 * locals.var_c_sb_dn8)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn9 / assign88300_e134557) * locals.var_c_sb) - (assign88300_e134558 * locals.var_c_sb_dn9)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn10 / assign88300_e134557) * locals.var_c_sb) - (assign88300_e134558 * locals.var_c_sb_dn10)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn13 / assign88300_e134557) * locals.var_c_sb) - (assign88300_e134558 * locals.var_c_sb_dn13)) / (locals.var_c_sb * locals.var_c_sb)),)
    } else {
        (locals.var_phi_b, locals.var_phi_b_dn0, locals.var_phi_b_dn2, locals.var_phi_b_dn4, locals.var_phi_b_dn5, locals.var_phi_b_dn6, locals.var_phi_b_dn7, locals.var_phi_b_dn8, locals.var_phi_b_dn9, locals.var_phi_b_dn10, locals.var_phi_b_dn13,)
    }
};
        locals.var_phi_b = assign88300_e134562;
        locals.var_phi_b_dn0 = assign88300_e134562_d_n0;
        locals.var_phi_b_dn2 = assign88300_e134562_d_n2;
        locals.var_phi_b_dn4 = assign88300_e134562_d_n4;
        locals.var_phi_b_dn5 = assign88300_e134562_d_n5;
        locals.var_phi_b_dn6 = assign88300_e134562_d_n6;
        locals.var_phi_b_dn7 = assign88300_e134562_d_n7;
        locals.var_phi_b_dn8 = assign88300_e134562_d_n8;
        locals.var_phi_b_dn9 = assign88300_e134562_d_n9;
        locals.var_phi_b_dn10 = assign88300_e134562_d_n10;
        locals.var_phi_b_dn13 = assign88300_e134562_d_n13;
        locals.var_phi_b_rv = 0.0;

        let (assign88310_e134576, assign88310_e134576_d_n0, assign88310_e134576_d_n2, assign88310_e134576_d_n4, assign88310_e134576_d_n5, assign88310_e134576_d_n6, assign88310_e134576_d_n7, assign88310_e134576_d_n8, assign88310_e134576_d_n9, assign88310_e134576_d_n10, assign88310_e134576_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2041 != 0.0)) && (locals.var_guard2045 == 0.0)) {
        let assign88310_e134574: f64 = (locals.var_ps0ld_vxb - locals.var_dphi_sb);
        (assign88310_e134574, (locals.var_ps0ld_vxb_dn0 - locals.var_dphi_sb_dn0), (locals.var_ps0ld_vxb_dn2 - locals.var_dphi_sb_dn2), (locals.var_ps0ld_vxb_dn4 - locals.var_dphi_sb_dn4), (locals.var_ps0ld_vxb_dn5 - locals.var_dphi_sb_dn5), (locals.var_ps0ld_vxb_dn6 - locals.var_dphi_sb_dn6), (locals.var_ps0ld_vxb_dn7 - locals.var_dphi_sb_dn7), (locals.var_ps0ld_vxb_dn8 - locals.var_dphi_sb_dn8), (locals.var_ps0ld_vxb_dn9 - locals.var_dphi_sb_dn9), (locals.var_ps0ld_vxb_dn10 - locals.var_dphi_sb_dn10), (locals.var_ps0ld_vxb_dn13 - locals.var_dphi_sb_dn13),)
    } else {
        (locals.var_phi_b, locals.var_phi_b_dn0, locals.var_phi_b_dn2, locals.var_phi_b_dn4, locals.var_phi_b_dn5, locals.var_phi_b_dn6, locals.var_phi_b_dn7, locals.var_phi_b_dn8, locals.var_phi_b_dn9, locals.var_phi_b_dn10, locals.var_phi_b_dn13,)
    }
};
        locals.var_phi_b = assign88310_e134576;
        locals.var_phi_b_dn0 = assign88310_e134576_d_n0;
        locals.var_phi_b_dn2 = assign88310_e134576_d_n2;
        locals.var_phi_b_dn4 = assign88310_e134576_d_n4;
        locals.var_phi_b_dn5 = assign88310_e134576_d_n5;
        locals.var_phi_b_dn6 = assign88310_e134576_d_n6;
        locals.var_phi_b_dn7 = assign88310_e134576_d_n7;
        locals.var_phi_b_dn8 = assign88310_e134576_d_n8;
        locals.var_phi_b_dn9 = assign88310_e134576_d_n9;
        locals.var_phi_b_dn10 = assign88310_e134576_d_n10;
        locals.var_phi_b_dn13 = assign88310_e134576_d_n13;
        locals.var_phi_b_rv = 0.0;

        let (assign88320_e134587, assign88320_e134587_d_n0, assign88320_e134587_d_n2, assign88320_e134587_d_n4, assign88320_e134587_d_n5, assign88320_e134587_d_n6, assign88320_e134587_d_n7, assign88320_e134587_d_n8, assign88320_e134587_d_n9, assign88320_e134587_d_n10, assign88320_e134587_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2041 != 0.0)) {
        let assign88320_e134585: f64 = (locals.var_beta * locals.var_phi_b);
        (assign88320_e134585, ((locals.var_beta_dn0 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn0)), ((locals.var_beta_dn2 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn2)), ((locals.var_beta_dn4 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn4)), ((locals.var_beta_dn5 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn5)), ((locals.var_beta_dn6 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn6)), ((locals.var_beta_dn7 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn7)), ((locals.var_beta_dn8 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn8)), ((locals.var_beta_dn9 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn9)), ((locals.var_beta_dn10 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn10)), ((locals.var_beta_dn13 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn13)),)
    } else {
        (locals.var_chib, locals.var_chib_dn0, locals.var_chib_dn2, locals.var_chib_dn4, locals.var_chib_dn5, locals.var_chib_dn6, locals.var_chib_dn7, locals.var_chib_dn8, locals.var_chib_dn9, locals.var_chib_dn10, locals.var_chib_dn13,)
    }
};
        locals.var_chib = assign88320_e134587;
        locals.var_chib_dn0 = assign88320_e134587_d_n0;
        locals.var_chib_dn2 = assign88320_e134587_d_n2;
        locals.var_chib_dn4 = assign88320_e134587_d_n4;
        locals.var_chib_dn5 = assign88320_e134587_d_n5;
        locals.var_chib_dn6 = assign88320_e134587_d_n6;
        locals.var_chib_dn7 = assign88320_e134587_d_n7;
        locals.var_chib_dn8 = assign88320_e134587_d_n8;
        locals.var_chib_dn9 = assign88320_e134587_d_n9;
        locals.var_chib_dn10 = assign88320_e134587_d_n10;
        locals.var_chib_dn13 = assign88320_e134587_d_n13;
        locals.var_chib_rv = 0.0;

        let assign88330_e134591: f64 = (locals.var_chi / 100.0);
        let assign88330_e134596: f64 = if ((locals.var_chib > assign88330_e134591) && (locals.var_chib > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard2046 = assign88330_e134596;
        locals.var_guard2046_rv = 0.0;

        let (assign88340_e134609,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2041 != 0.0)) && (locals.var_guard2046 != 0.0)) {
        let assign88340_e134607: f64 = (locals.var_flg_fd_mode__blk2010 + 1.0);
        (assign88340_e134607,)
    } else {
        (locals.var_flg_fd_mode__blk2010,)
    }
};
        locals.var_flg_fd_mode__blk2010 = assign88340_e134609;
        locals.var_flg_fd_mode__blk2010_rv = 0.0;

        let (assign88350_e134620, assign88350_e134620_d_n0, assign88350_e134620_d_n2, assign88350_e134620_d_n4, assign88350_e134620_d_n5, assign88350_e134620_d_n6, assign88350_e134620_d_n7, assign88350_e134620_d_n8, assign88350_e134620_d_n9, assign88350_e134620_d_n10, assign88350_e134620_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2041 != 0.0)) && (locals.var_guard2046 != 0.0)) {
        (locals.var_chi_1, locals.var_chi_1_dn0, locals.var_chi_1_dn2, locals.var_chi_1_dn4, locals.var_chi_1_dn5, locals.var_chi_1_dn6, locals.var_chi_1_dn7, locals.var_chi_1_dn8, locals.var_chi_1_dn9, locals.var_chi_1_dn10, locals.var_chi_1_dn13,)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn13,)
    }
};
        locals.var_chi = assign88350_e134620;
        locals.var_chi_dn0 = assign88350_e134620_d_n0;
        locals.var_chi_dn2 = assign88350_e134620_d_n2;
        locals.var_chi_dn4 = assign88350_e134620_d_n4;
        locals.var_chi_dn5 = assign88350_e134620_d_n5;
        locals.var_chi_dn6 = assign88350_e134620_d_n6;
        locals.var_chi_dn7 = assign88350_e134620_d_n7;
        locals.var_chi_dn8 = assign88350_e134620_d_n8;
        locals.var_chi_dn9 = assign88350_e134620_d_n9;
        locals.var_chi_dn10 = assign88350_e134620_d_n10;
        locals.var_chi_dn13 = assign88350_e134620_d_n13;
        locals.var_chi_rv = 0.0;

        let (assign88360_e134631, assign88360_e134631_d_n0, assign88360_e134631_d_n2, assign88360_e134631_d_n4, assign88360_e134631_d_n5, assign88360_e134631_d_n6, assign88360_e134631_d_n7, assign88360_e134631_d_n8, assign88360_e134631_d_n9, assign88360_e134631_d_n10, assign88360_e134631_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) {
        let assign88360_e134627: f64 = (locals.var_chi * locals.var_beta_inv);
        let assign88360_e134629: f64 = (assign88360_e134627 - locals.var_vxbgmtcl);
        (assign88360_e134629, (((locals.var_chi_dn0 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn0)) - locals.var_vxbgmtcl_dn0), (((locals.var_chi_dn2 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn2)) - locals.var_vxbgmtcl_dn2), (((locals.var_chi_dn4 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn4)) - locals.var_vxbgmtcl_dn4), (((locals.var_chi_dn5 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn5)) - locals.var_vxbgmtcl_dn5), (((locals.var_chi_dn6 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn6)) - locals.var_vxbgmtcl_dn6), (((locals.var_chi_dn7 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn7)) - locals.var_vxbgmtcl_dn7), (((locals.var_chi_dn8 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn8)) - locals.var_vxbgmtcl_dn8), (((locals.var_chi_dn9 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn9)) - locals.var_vxbgmtcl_dn9), (((locals.var_chi_dn10 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn10)) - locals.var_vxbgmtcl_dn10), (((locals.var_chi_dn13 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn13)) - locals.var_vxbgmtcl_dn13),)
    } else {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn13,)
    }
};
        locals.var_ps0ld = assign88360_e134631;
        locals.var_ps0ld_dn0 = assign88360_e134631_d_n0;
        locals.var_ps0ld_dn2 = assign88360_e134631_d_n2;
        locals.var_ps0ld_dn4 = assign88360_e134631_d_n4;
        locals.var_ps0ld_dn5 = assign88360_e134631_d_n5;
        locals.var_ps0ld_dn6 = assign88360_e134631_d_n6;
        locals.var_ps0ld_dn7 = assign88360_e134631_d_n7;
        locals.var_ps0ld_dn8 = assign88360_e134631_d_n8;
        locals.var_ps0ld_dn9 = assign88360_e134631_d_n9;
        locals.var_ps0ld_dn10 = assign88360_e134631_d_n10;
        locals.var_ps0ld_dn13 = assign88360_e134631_d_n13;
        locals.var_ps0ld_rv = 0.0;

        let assign88370_e134633: f64 = (locals.var_chi).abs();
        let assign88370_e134635: f64 = if assign88370_e134633 > 1e-6 { 1.0 } else { 0.0 };
        locals.var_guard2047 = assign88370_e134635;
        locals.var_guard2047_rv = 0.0;

        let (assign88380_e134650, assign88380_e134650_d_n0, assign88380_e134650_d_n2, assign88380_e134650_d_n4, assign88380_e134650_d_n5, assign88380_e134650_d_n6, assign88380_e134650_d_n7, assign88380_e134650_d_n8, assign88380_e134650_d_n9, assign88380_e134650_d_n10, assign88380_e134650_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2047 != 0.0)) {
        let assign88380_e134644: f64 = (locals.var_chi - 1.0);
        let assign88380_e134646: f64 = (-locals.var_chi);
        let assign88380_e134647: f64 = (assign88380_e134646).exp();
        let assign88380_e134648: f64 = (assign88380_e134644 + assign88380_e134647);
        (assign88380_e134648, (locals.var_chi_dn0 + (assign88380_e134647 * (-locals.var_chi_dn0))), (locals.var_chi_dn2 + (assign88380_e134647 * (-locals.var_chi_dn2))), (locals.var_chi_dn4 + (assign88380_e134647 * (-locals.var_chi_dn4))), (locals.var_chi_dn5 + (assign88380_e134647 * (-locals.var_chi_dn5))), (locals.var_chi_dn6 + (assign88380_e134647 * (-locals.var_chi_dn6))), (locals.var_chi_dn7 + (assign88380_e134647 * (-locals.var_chi_dn7))), (locals.var_chi_dn8 + (assign88380_e134647 * (-locals.var_chi_dn8))), (locals.var_chi_dn9 + (assign88380_e134647 * (-locals.var_chi_dn9))), (locals.var_chi_dn10 + (assign88380_e134647 * (-locals.var_chi_dn10))), (locals.var_chi_dn13 + (assign88380_e134647 * (-locals.var_chi_dn13))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign88380_e134650;
        locals.var_t1_dn0 = assign88380_e134650_d_n0;
        locals.var_t1_dn2 = assign88380_e134650_d_n2;
        locals.var_t1_dn4 = assign88380_e134650_d_n4;
        locals.var_t1_dn5 = assign88380_e134650_d_n5;
        locals.var_t1_dn6 = assign88380_e134650_d_n6;
        locals.var_t1_dn7 = assign88380_e134650_d_n7;
        locals.var_t1_dn8 = assign88380_e134650_d_n8;
        locals.var_t1_dn9 = assign88380_e134650_d_n9;
        locals.var_t1_dn10 = assign88380_e134650_d_n10;
        locals.var_t1_dn13 = assign88380_e134650_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign88390_e134660, assign88390_e134660_d_n0, assign88390_e134660_d_n2, assign88390_e134660_d_n4, assign88390_e134660_d_n5, assign88390_e134660_d_n6, assign88390_e134660_d_n7, assign88390_e134660_d_n8, assign88390_e134660_d_n9, assign88390_e134660_d_n10, assign88390_e134660_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2047 != 0.0)) {
        let assign88390_e134658: f64 = (locals.var_t1).sqrt();
        (assign88390_e134658, (locals.var_t1_dn0 / (2.0 * assign88390_e134658)), (locals.var_t1_dn2 / (2.0 * assign88390_e134658)), (locals.var_t1_dn4 / (2.0 * assign88390_e134658)), (locals.var_t1_dn5 / (2.0 * assign88390_e134658)), (locals.var_t1_dn6 / (2.0 * assign88390_e134658)), (locals.var_t1_dn7 / (2.0 * assign88390_e134658)), (locals.var_t1_dn8 / (2.0 * assign88390_e134658)), (locals.var_t1_dn9 / (2.0 * assign88390_e134658)), (locals.var_t1_dn10 / (2.0 * assign88390_e134658)), (locals.var_t1_dn13 / (2.0 * assign88390_e134658)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign88390_e134660;
        locals.var_t2_dn0 = assign88390_e134660_d_n0;
        locals.var_t2_dn2 = assign88390_e134660_d_n2;
        locals.var_t2_dn4 = assign88390_e134660_d_n4;
        locals.var_t2_dn5 = assign88390_e134660_d_n5;
        locals.var_t2_dn6 = assign88390_e134660_d_n6;
        locals.var_t2_dn7 = assign88390_e134660_d_n7;
        locals.var_t2_dn8 = assign88390_e134660_d_n8;
        locals.var_t2_dn9 = assign88390_e134660_d_n9;
        locals.var_t2_dn10 = assign88390_e134660_d_n10;
        locals.var_t2_dn13 = assign88390_e134660_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign88410_e134691, assign88410_e134691_d_n0, assign88410_e134691_d_n2, assign88410_e134691_d_n4, assign88410_e134691_d_n5, assign88410_e134691_d_n6, assign88410_e134691_d_n7, assign88410_e134691_d_n8, assign88410_e134691_d_n9, assign88410_e134691_d_n10, assign88410_e134691_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2047 == 0.0)) {
        let assign88410_e134682: f64 = (0.7071067811865475 * locals.var_chi);
        let assign88410_e134686: f64 = (locals.var_chi * 0.3333333333333333);
        let assign88410_e134687: f64 = (1.0 - assign88410_e134686);
        let assign88410_e134688: f64 = (assign88410_e134687).sqrt();
        let assign88410_e134689: f64 = (assign88410_e134682 * assign88410_e134688);
        (assign88410_e134689, (((0.7071067811865475 * locals.var_chi_dn0) * assign88410_e134688) + (assign88410_e134682 * ((-(locals.var_chi_dn0 * 0.3333333333333333)) / (2.0 * assign88410_e134688)))), (((0.7071067811865475 * locals.var_chi_dn2) * assign88410_e134688) + (assign88410_e134682 * ((-(locals.var_chi_dn2 * 0.3333333333333333)) / (2.0 * assign88410_e134688)))), (((0.7071067811865475 * locals.var_chi_dn4) * assign88410_e134688) + (assign88410_e134682 * ((-(locals.var_chi_dn4 * 0.3333333333333333)) / (2.0 * assign88410_e134688)))), (((0.7071067811865475 * locals.var_chi_dn5) * assign88410_e134688) + (assign88410_e134682 * ((-(locals.var_chi_dn5 * 0.3333333333333333)) / (2.0 * assign88410_e134688)))), (((0.7071067811865475 * locals.var_chi_dn6) * assign88410_e134688) + (assign88410_e134682 * ((-(locals.var_chi_dn6 * 0.3333333333333333)) / (2.0 * assign88410_e134688)))), (((0.7071067811865475 * locals.var_chi_dn7) * assign88410_e134688) + (assign88410_e134682 * ((-(locals.var_chi_dn7 * 0.3333333333333333)) / (2.0 * assign88410_e134688)))), (((0.7071067811865475 * locals.var_chi_dn8) * assign88410_e134688) + (assign88410_e134682 * ((-(locals.var_chi_dn8 * 0.3333333333333333)) / (2.0 * assign88410_e134688)))), (((0.7071067811865475 * locals.var_chi_dn9) * assign88410_e134688) + (assign88410_e134682 * ((-(locals.var_chi_dn9 * 0.3333333333333333)) / (2.0 * assign88410_e134688)))), (((0.7071067811865475 * locals.var_chi_dn10) * assign88410_e134688) + (assign88410_e134682 * ((-(locals.var_chi_dn10 * 0.3333333333333333)) / (2.0 * assign88410_e134688)))), (((0.7071067811865475 * locals.var_chi_dn13) * assign88410_e134688) + (assign88410_e134682 * ((-(locals.var_chi_dn13 * 0.3333333333333333)) / (2.0 * assign88410_e134688)))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign88410_e134691;
        locals.var_t2_dn0 = assign88410_e134691_d_n0;
        locals.var_t2_dn2 = assign88410_e134691_d_n2;
        locals.var_t2_dn4 = assign88410_e134691_d_n4;
        locals.var_t2_dn5 = assign88410_e134691_d_n5;
        locals.var_t2_dn6 = assign88410_e134691_d_n6;
        locals.var_t2_dn7 = assign88410_e134691_d_n7;
        locals.var_t2_dn8 = assign88410_e134691_d_n8;
        locals.var_t2_dn9 = assign88410_e134691_d_n9;
        locals.var_t2_dn10 = assign88410_e134691_d_n10;
        locals.var_t2_dn13 = assign88410_e134691_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign88420_e134700, assign88420_e134700_d_n0, assign88420_e134700_d_n2, assign88420_e134700_d_n4, assign88420_e134700_d_n5, assign88420_e134700_d_n6, assign88420_e134700_d_n7, assign88420_e134700_d_n8, assign88420_e134700_d_n9, assign88420_e134700_d_n10, assign88420_e134700_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) {
        let assign88420_e134698: f64 = (locals.var_cnst0over_func * locals.var_t2);
        (assign88420_e134698, ((locals.var_cnst0over_func_dn0 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn0)), ((locals.var_cnst0over_func_dn2 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn2)), ((locals.var_cnst0over_func_dn4 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn4)), ((locals.var_cnst0over_func_dn5 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn5)), ((locals.var_cnst0over_func_dn6 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn6)), ((locals.var_cnst0over_func_dn7 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn7)), ((locals.var_cnst0over_func_dn8 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn8)), ((locals.var_cnst0over_func_dn9 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn9)), ((locals.var_cnst0over_func_dn10 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn10)), ((locals.var_cnst0over_func_dn13 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn13)),)
    } else {
        (locals.var_qbuld, locals.var_qbuld_dn0, locals.var_qbuld_dn2, locals.var_qbuld_dn4, locals.var_qbuld_dn5, locals.var_qbuld_dn6, locals.var_qbuld_dn7, locals.var_qbuld_dn8, locals.var_qbuld_dn9, locals.var_qbuld_dn10, locals.var_qbuld_dn13,)
    }
};
        locals.var_qbuld = assign88420_e134700;
        locals.var_qbuld_dn0 = assign88420_e134700_d_n0;
        locals.var_qbuld_dn2 = assign88420_e134700_d_n2;
        locals.var_qbuld_dn4 = assign88420_e134700_d_n4;
        locals.var_qbuld_dn5 = assign88420_e134700_d_n5;
        locals.var_qbuld_dn6 = assign88420_e134700_d_n6;
        locals.var_qbuld_dn7 = assign88420_e134700_d_n7;
        locals.var_qbuld_dn8 = assign88420_e134700_d_n8;
        locals.var_qbuld_dn9 = assign88420_e134700_d_n9;
        locals.var_qbuld_dn10 = assign88420_e134700_d_n10;
        locals.var_qbuld_dn13 = assign88420_e134700_d_n13;
        locals.var_qbuld_rv = 0.0;

        let (assign88430_e134711, assign88430_e134711_d_n0, assign88430_e134711_d_n2, assign88430_e134711_d_n4, assign88430_e134711_d_n5, assign88430_e134711_d_n6, assign88430_e134711_d_n7, assign88430_e134711_d_n8, assign88430_e134711_d_n9, assign88430_e134711_d_n10, assign88430_e134711_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) {
        let assign88430_e134708: f64 = (locals.var_vgpld - locals.var_ps0ld);
        let assign88430_e134709: f64 = (locals.var_cox0_func * assign88430_e134708);
        (assign88430_e134709, (locals.var_cox0_func * (-locals.var_ps0ld_dn0)), (locals.var_cox0_func * (locals.var_vgpld_dn2 - locals.var_ps0ld_dn2)), (locals.var_cox0_func * (-locals.var_ps0ld_dn4)), (locals.var_cox0_func * (-locals.var_ps0ld_dn5)), (locals.var_cox0_func * (locals.var_vgpld_dn6 - locals.var_ps0ld_dn6)), (locals.var_cox0_func * (locals.var_vgpld_dn7 - locals.var_ps0ld_dn7)), (locals.var_cox0_func * (locals.var_vgpld_dn8 - locals.var_ps0ld_dn8)), (locals.var_cox0_func * (-locals.var_ps0ld_dn9)), (locals.var_cox0_func * (-locals.var_ps0ld_dn10)), (locals.var_cox0_func * (-locals.var_ps0ld_dn13)),)
    } else {
        (locals.var_qsuld, locals.var_qsuld_dn0, locals.var_qsuld_dn2, locals.var_qsuld_dn4, locals.var_qsuld_dn5, locals.var_qsuld_dn6, locals.var_qsuld_dn7, locals.var_qsuld_dn8, locals.var_qsuld_dn9, locals.var_qsuld_dn10, locals.var_qsuld_dn13,)
    }
};
        locals.var_qsuld = assign88430_e134711;
        locals.var_qsuld_dn0 = assign88430_e134711_d_n0;
        locals.var_qsuld_dn2 = assign88430_e134711_d_n2;
        locals.var_qsuld_dn4 = assign88430_e134711_d_n4;
        locals.var_qsuld_dn5 = assign88430_e134711_d_n5;
        locals.var_qsuld_dn6 = assign88430_e134711_d_n6;
        locals.var_qsuld_dn7 = assign88430_e134711_d_n7;
        locals.var_qsuld_dn8 = assign88430_e134711_d_n8;
        locals.var_qsuld_dn9 = assign88430_e134711_d_n9;
        locals.var_qsuld_dn10 = assign88430_e134711_d_n10;
        locals.var_qsuld_dn13 = assign88430_e134711_d_n13;
        locals.var_qsuld_rv = 0.0;

        let (assign88440_e134720, assign88440_e134720_d_n0, assign88440_e134720_d_n2, assign88440_e134720_d_n4, assign88440_e134720_d_n5, assign88440_e134720_d_n6, assign88440_e134720_d_n7, assign88440_e134720_d_n8, assign88440_e134720_d_n9, assign88440_e134720_d_n10, assign88440_e134720_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) {
        let assign88440_e134718: f64 = (locals.var_qbuld / locals.var_q_nsubld__blk2004);
        (assign88440_e134718, (locals.var_qbuld_dn0 / locals.var_q_nsubld__blk2004), (locals.var_qbuld_dn2 / locals.var_q_nsubld__blk2004), (locals.var_qbuld_dn4 / locals.var_q_nsubld__blk2004), (locals.var_qbuld_dn5 / locals.var_q_nsubld__blk2004), (locals.var_qbuld_dn6 / locals.var_q_nsubld__blk2004), (locals.var_qbuld_dn7 / locals.var_q_nsubld__blk2004), (locals.var_qbuld_dn8 / locals.var_q_nsubld__blk2004), (locals.var_qbuld_dn9 / locals.var_q_nsubld__blk2004), (locals.var_qbuld_dn10 / locals.var_q_nsubld__blk2004), (locals.var_qbuld_dn13 / locals.var_q_nsubld__blk2004),)
    } else {
        (locals.var_wdld0__blk2048, locals.var_wdld0__blk2048_dn0, locals.var_wdld0__blk2048_dn2, locals.var_wdld0__blk2048_dn4, locals.var_wdld0__blk2048_dn5, locals.var_wdld0__blk2048_dn6, locals.var_wdld0__blk2048_dn7, locals.var_wdld0__blk2048_dn8, locals.var_wdld0__blk2048_dn9, locals.var_wdld0__blk2048_dn10, locals.var_wdld0__blk2048_dn13,)
    }
};
        locals.var_wdld0__blk2048 = assign88440_e134720;
        locals.var_wdld0__blk2048_dn0 = assign88440_e134720_d_n0;
        locals.var_wdld0__blk2048_dn2 = assign88440_e134720_d_n2;
        locals.var_wdld0__blk2048_dn4 = assign88440_e134720_d_n4;
        locals.var_wdld0__blk2048_dn5 = assign88440_e134720_d_n5;
        locals.var_wdld0__blk2048_dn6 = assign88440_e134720_d_n6;
        locals.var_wdld0__blk2048_dn7 = assign88440_e134720_d_n7;
        locals.var_wdld0__blk2048_dn8 = assign88440_e134720_d_n8;
        locals.var_wdld0__blk2048_dn9 = assign88440_e134720_d_n9;
        locals.var_wdld0__blk2048_dn10 = assign88440_e134720_d_n10;
        locals.var_wdld0__blk2048_dn13 = assign88440_e134720_d_n13;
        locals.var_wdld0__blk2048_rv = 0.0;

        let assign88450_e134723: f64 = if p.p33 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard2050 = assign88450_e134723;
        locals.var_guard2050_rv = 0.0;

        let assign88460_e134728: f64 = (locals.var_ddriftldc * 0.1);
        let assign88460_e134729: f64 = (locals.var_ddriftldc - assign88460_e134728);
        let assign88460_e134733: f64 = (locals.var_ddriftldc * 0.1);
        let assign88460_e134736: f64 = if ((locals.var_wdld0__blk2048 > assign88460_e134729) && (assign88460_e134733 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard2051 = assign88460_e134736;
        locals.var_guard2051_rv = 0.0;

        let (assign88470_e134753, assign88470_e134753_d_n0, assign88470_e134753_d_n2, assign88470_e134753_d_n4, assign88470_e134753_d_n5, assign88470_e134753_d_n6, assign88470_e134753_d_n7, assign88470_e134753_d_n8, assign88470_e134753_d_n9, assign88470_e134753_d_n10, assign88470_e134753_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2050 != 0.0)) && (locals.var_guard2051 != 0.0)) {
        let assign88470_e134747: f64 = (locals.var_wdld0__blk2048 - locals.var_ddriftldc);
        let assign88470_e134750: f64 = (locals.var_ddriftldc * 0.1);
        let assign88470_e134751: f64 = (assign88470_e134747 + assign88470_e134750);
        (assign88470_e134751, ((locals.var_wdld0__blk2048_dn0 - locals.var_ddriftldc_dn0) + (locals.var_ddriftldc_dn0 * 0.1)), ((locals.var_wdld0__blk2048_dn2 - locals.var_ddriftldc_dn2) + (locals.var_ddriftldc_dn2 * 0.1)), ((locals.var_wdld0__blk2048_dn4 - locals.var_ddriftldc_dn4) + (locals.var_ddriftldc_dn4 * 0.1)), ((locals.var_wdld0__blk2048_dn5 - locals.var_ddriftldc_dn5) + (locals.var_ddriftldc_dn5 * 0.1)), ((locals.var_wdld0__blk2048_dn6 - locals.var_ddriftldc_dn6) + (locals.var_ddriftldc_dn6 * 0.1)), ((locals.var_wdld0__blk2048_dn7 - locals.var_ddriftldc_dn7) + (locals.var_ddriftldc_dn7 * 0.1)), ((locals.var_wdld0__blk2048_dn8 - locals.var_ddriftldc_dn8) + (locals.var_ddriftldc_dn8 * 0.1)), ((locals.var_wdld0__blk2048_dn9 - locals.var_ddriftldc_dn9) + (locals.var_ddriftldc_dn9 * 0.1)), ((locals.var_wdld0__blk2048_dn10 - locals.var_ddriftldc_dn10) + (locals.var_ddriftldc_dn10 * 0.1)), ((locals.var_wdld0__blk2048_dn13 - locals.var_ddriftldc_dn13) + (locals.var_ddriftldc_dn13 * 0.1)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign88470_e134753;
        locals.var_tmf1_dn0 = assign88470_e134753_d_n0;
        locals.var_tmf1_dn2 = assign88470_e134753_d_n2;
        locals.var_tmf1_dn4 = assign88470_e134753_d_n4;
        locals.var_tmf1_dn5 = assign88470_e134753_d_n5;
        locals.var_tmf1_dn6 = assign88470_e134753_d_n6;
        locals.var_tmf1_dn7 = assign88470_e134753_d_n7;
        locals.var_tmf1_dn8 = assign88470_e134753_d_n8;
        locals.var_tmf1_dn9 = assign88470_e134753_d_n9;
        locals.var_tmf1_dn10 = assign88470_e134753_d_n10;
        locals.var_tmf1_dn13 = assign88470_e134753_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign88480_e134766, assign88480_e134766_d_n0, assign88480_e134766_d_n2, assign88480_e134766_d_n4, assign88480_e134766_d_n5, assign88480_e134766_d_n6, assign88480_e134766_d_n7, assign88480_e134766_d_n8, assign88480_e134766_d_n9, assign88480_e134766_d_n10, assign88480_e134766_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2050 != 0.0)) && (locals.var_guard2051 != 0.0)) {
        let assign88480_e134764: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign88480_e134764, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn13,)
    }
};
        locals.var_x2 = assign88480_e134766;
        locals.var_x2_dn0 = assign88480_e134766_d_n0;
        locals.var_x2_dn2 = assign88480_e134766_d_n2;
        locals.var_x2_dn4 = assign88480_e134766_d_n4;
        locals.var_x2_dn5 = assign88480_e134766_d_n5;
        locals.var_x2_dn6 = assign88480_e134766_d_n6;
        locals.var_x2_dn7 = assign88480_e134766_d_n7;
        locals.var_x2_dn8 = assign88480_e134766_d_n8;
        locals.var_x2_dn9 = assign88480_e134766_d_n9;
        locals.var_x2_dn10 = assign88480_e134766_d_n10;
        locals.var_x2_dn13 = assign88480_e134766_d_n13;
        locals.var_x2_rv = 0.0;

        let (assign88490_e134783, assign88490_e134783_d_n0, assign88490_e134783_d_n2, assign88490_e134783_d_n4, assign88490_e134783_d_n5, assign88490_e134783_d_n6, assign88490_e134783_d_n7, assign88490_e134783_d_n8, assign88490_e134783_d_n9, assign88490_e134783_d_n10, assign88490_e134783_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2050 != 0.0)) && (locals.var_guard2051 != 0.0)) {
        let assign88490_e134777: f64 = (locals.var_ddriftldc * 0.1);
        let assign88490_e134780: f64 = (locals.var_ddriftldc * 0.1);
        let assign88490_e134781: f64 = (assign88490_e134777 * assign88490_e134780);
        (assign88490_e134781, (((locals.var_ddriftldc_dn0 * 0.1) * assign88490_e134780) + (assign88490_e134777 * (locals.var_ddriftldc_dn0 * 0.1))), (((locals.var_ddriftldc_dn2 * 0.1) * assign88490_e134780) + (assign88490_e134777 * (locals.var_ddriftldc_dn2 * 0.1))), (((locals.var_ddriftldc_dn4 * 0.1) * assign88490_e134780) + (assign88490_e134777 * (locals.var_ddriftldc_dn4 * 0.1))), (((locals.var_ddriftldc_dn5 * 0.1) * assign88490_e134780) + (assign88490_e134777 * (locals.var_ddriftldc_dn5 * 0.1))), (((locals.var_ddriftldc_dn6 * 0.1) * assign88490_e134780) + (assign88490_e134777 * (locals.var_ddriftldc_dn6 * 0.1))), (((locals.var_ddriftldc_dn7 * 0.1) * assign88490_e134780) + (assign88490_e134777 * (locals.var_ddriftldc_dn7 * 0.1))), (((locals.var_ddriftldc_dn8 * 0.1) * assign88490_e134780) + (assign88490_e134777 * (locals.var_ddriftldc_dn8 * 0.1))), (((locals.var_ddriftldc_dn9 * 0.1) * assign88490_e134780) + (assign88490_e134777 * (locals.var_ddriftldc_dn9 * 0.1))), (((locals.var_ddriftldc_dn10 * 0.1) * assign88490_e134780) + (assign88490_e134777 * (locals.var_ddriftldc_dn10 * 0.1))), (((locals.var_ddriftldc_dn13 * 0.1) * assign88490_e134780) + (assign88490_e134777 * (locals.var_ddriftldc_dn13 * 0.1))),)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn13,)
    }
};
        locals.var_xmax2 = assign88490_e134783;
        locals.var_xmax2_dn0 = assign88490_e134783_d_n0;
        locals.var_xmax2_dn2 = assign88490_e134783_d_n2;
        locals.var_xmax2_dn4 = assign88490_e134783_d_n4;
        locals.var_xmax2_dn5 = assign88490_e134783_d_n5;
        locals.var_xmax2_dn6 = assign88490_e134783_d_n6;
        locals.var_xmax2_dn7 = assign88490_e134783_d_n7;
        locals.var_xmax2_dn8 = assign88490_e134783_d_n8;
        locals.var_xmax2_dn9 = assign88490_e134783_d_n9;
        locals.var_xmax2_dn10 = assign88490_e134783_d_n10;
        locals.var_xmax2_dn13 = assign88490_e134783_d_n13;
        locals.var_xmax2_rv = 0.0;

        let (assign88500_e134794, assign88500_e134794_d_n0, assign88500_e134794_d_n2, assign88500_e134794_d_n4, assign88500_e134794_d_n5, assign88500_e134794_d_n6, assign88500_e134794_d_n7, assign88500_e134794_d_n8, assign88500_e134794_d_n9, assign88500_e134794_d_n10, assign88500_e134794_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2050 != 0.0)) && (locals.var_guard2051 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign88500_e134794;
        locals.var_xp_dn0 = assign88500_e134794_d_n0;
        locals.var_xp_dn2 = assign88500_e134794_d_n2;
        locals.var_xp_dn4 = assign88500_e134794_d_n4;
        locals.var_xp_dn5 = assign88500_e134794_d_n5;
        locals.var_xp_dn6 = assign88500_e134794_d_n6;
        locals.var_xp_dn7 = assign88500_e134794_d_n7;
        locals.var_xp_dn8 = assign88500_e134794_d_n8;
        locals.var_xp_dn9 = assign88500_e134794_d_n9;
        locals.var_xp_dn10 = assign88500_e134794_d_n10;
        locals.var_xp_dn13 = assign88500_e134794_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign88510_e134805, assign88510_e134805_d_n0, assign88510_e134805_d_n2, assign88510_e134805_d_n4, assign88510_e134805_d_n5, assign88510_e134805_d_n6, assign88510_e134805_d_n7, assign88510_e134805_d_n8, assign88510_e134805_d_n9, assign88510_e134805_d_n10, assign88510_e134805_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2050 != 0.0)) && (locals.var_guard2051 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign88510_e134805;
        locals.var_xmp_dn0 = assign88510_e134805_d_n0;
        locals.var_xmp_dn2 = assign88510_e134805_d_n2;
        locals.var_xmp_dn4 = assign88510_e134805_d_n4;
        locals.var_xmp_dn5 = assign88510_e134805_d_n5;
        locals.var_xmp_dn6 = assign88510_e134805_d_n6;
        locals.var_xmp_dn7 = assign88510_e134805_d_n7;
        locals.var_xmp_dn8 = assign88510_e134805_d_n8;
        locals.var_xmp_dn9 = assign88510_e134805_d_n9;
        locals.var_xmp_dn10 = assign88510_e134805_d_n10;
        locals.var_xmp_dn13 = assign88510_e134805_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign88520_e134816,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2050 != 0.0)) && (locals.var_guard2051 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign88520_e134816;
        locals.var_m0_rv = 0.0;

        let (assign88530_e134827,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2050 != 0.0)) && (locals.var_guard2051 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign88530_e134827;
        locals.var_mm_rv = 0.0;

        let (assign88540_e134838, assign88540_e134838_d_n0, assign88540_e134838_d_n2, assign88540_e134838_d_n4, assign88540_e134838_d_n5, assign88540_e134838_d_n6, assign88540_e134838_d_n7, assign88540_e134838_d_n8, assign88540_e134838_d_n9, assign88540_e134838_d_n10, assign88540_e134838_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2050 != 0.0)) && (locals.var_guard2051 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign88540_e134838;
        locals.var_arg_dn0 = assign88540_e134838_d_n0;
        locals.var_arg_dn2 = assign88540_e134838_d_n2;
        locals.var_arg_dn4 = assign88540_e134838_d_n4;
        locals.var_arg_dn5 = assign88540_e134838_d_n5;
        locals.var_arg_dn6 = assign88540_e134838_d_n6;
        locals.var_arg_dn7 = assign88540_e134838_d_n7;
        locals.var_arg_dn8 = assign88540_e134838_d_n8;
        locals.var_arg_dn9 = assign88540_e134838_d_n9;
        locals.var_arg_dn10 = assign88540_e134838_d_n10;
        locals.var_arg_dn13 = assign88540_e134838_d_n13;
        locals.var_arg_rv = 0.0;

        let (assign88550_e134849, assign88550_e134849_d_n0, assign88550_e134849_d_n2, assign88550_e134849_d_n4, assign88550_e134849_d_n5, assign88550_e134849_d_n6, assign88550_e134849_d_n7, assign88550_e134849_d_n8, assign88550_e134849_d_n9, assign88550_e134849_d_n10, assign88550_e134849_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2050 != 0.0)) && (locals.var_guard2051 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign88550_e134849;
        locals.var_dnm_dn0 = assign88550_e134849_d_n0;
        locals.var_dnm_dn2 = assign88550_e134849_d_n2;
        locals.var_dnm_dn4 = assign88550_e134849_d_n4;
        locals.var_dnm_dn5 = assign88550_e134849_d_n5;
        locals.var_dnm_dn6 = assign88550_e134849_d_n6;
        locals.var_dnm_dn7 = assign88550_e134849_d_n7;
        locals.var_dnm_dn8 = assign88550_e134849_d_n8;
        locals.var_dnm_dn9 = assign88550_e134849_d_n9;
        locals.var_dnm_dn10 = assign88550_e134849_d_n10;
        locals.var_dnm_dn13 = assign88550_e134849_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign88560_e134862, assign88560_e134862_d_n0, assign88560_e134862_d_n2, assign88560_e134862_d_n4, assign88560_e134862_d_n5, assign88560_e134862_d_n6, assign88560_e134862_d_n7, assign88560_e134862_d_n8, assign88560_e134862_d_n9, assign88560_e134862_d_n10, assign88560_e134862_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2050 != 0.0)) && (locals.var_guard2051 != 0.0)) {
        let assign88560_e134860: f64 = (locals.var_xp * locals.var_x2);
        (assign88560_e134860, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign88560_e134862;
        locals.var_xp_dn0 = assign88560_e134862_d_n0;
        locals.var_xp_dn2 = assign88560_e134862_d_n2;
        locals.var_xp_dn4 = assign88560_e134862_d_n4;
        locals.var_xp_dn5 = assign88560_e134862_d_n5;
        locals.var_xp_dn6 = assign88560_e134862_d_n6;
        locals.var_xp_dn7 = assign88560_e134862_d_n7;
        locals.var_xp_dn8 = assign88560_e134862_d_n8;
        locals.var_xp_dn9 = assign88560_e134862_d_n9;
        locals.var_xp_dn10 = assign88560_e134862_d_n10;
        locals.var_xp_dn13 = assign88560_e134862_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign88570_e134875, assign88570_e134875_d_n0, assign88570_e134875_d_n2, assign88570_e134875_d_n4, assign88570_e134875_d_n5, assign88570_e134875_d_n6, assign88570_e134875_d_n7, assign88570_e134875_d_n8, assign88570_e134875_d_n9, assign88570_e134875_d_n10, assign88570_e134875_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2050 != 0.0)) && (locals.var_guard2051 != 0.0)) {
        let assign88570_e134873: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign88570_e134873, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign88570_e134875;
        locals.var_xmp_dn0 = assign88570_e134875_d_n0;
        locals.var_xmp_dn2 = assign88570_e134875_d_n2;
        locals.var_xmp_dn4 = assign88570_e134875_d_n4;
        locals.var_xmp_dn5 = assign88570_e134875_d_n5;
        locals.var_xmp_dn6 = assign88570_e134875_d_n6;
        locals.var_xmp_dn7 = assign88570_e134875_d_n7;
        locals.var_xmp_dn8 = assign88570_e134875_d_n8;
        locals.var_xmp_dn9 = assign88570_e134875_d_n9;
        locals.var_xmp_dn10 = assign88570_e134875_d_n10;
        locals.var_xmp_dn13 = assign88570_e134875_d_n13;
        locals.var_xmp_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_328(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign88580_e134888, assign88580_e134888_d_n0, assign88580_e134888_d_n2, assign88580_e134888_d_n4, assign88580_e134888_d_n5, assign88580_e134888_d_n6, assign88580_e134888_d_n7, assign88580_e134888_d_n8, assign88580_e134888_d_n9, assign88580_e134888_d_n10, assign88580_e134888_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2050 != 0.0)) && (locals.var_guard2051 != 0.0)) {
        let assign88580_e134886: f64 = (locals.var_xp * locals.var_x2);
        (assign88580_e134886, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign88580_e134888;
        locals.var_xp_dn0 = assign88580_e134888_d_n0;
        locals.var_xp_dn2 = assign88580_e134888_d_n2;
        locals.var_xp_dn4 = assign88580_e134888_d_n4;
        locals.var_xp_dn5 = assign88580_e134888_d_n5;
        locals.var_xp_dn6 = assign88580_e134888_d_n6;
        locals.var_xp_dn7 = assign88580_e134888_d_n7;
        locals.var_xp_dn8 = assign88580_e134888_d_n8;
        locals.var_xp_dn9 = assign88580_e134888_d_n9;
        locals.var_xp_dn10 = assign88580_e134888_d_n10;
        locals.var_xp_dn13 = assign88580_e134888_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign88590_e134901, assign88590_e134901_d_n0, assign88590_e134901_d_n2, assign88590_e134901_d_n4, assign88590_e134901_d_n5, assign88590_e134901_d_n6, assign88590_e134901_d_n7, assign88590_e134901_d_n8, assign88590_e134901_d_n9, assign88590_e134901_d_n10, assign88590_e134901_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2050 != 0.0)) && (locals.var_guard2051 != 0.0)) {
        let assign88590_e134899: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign88590_e134899, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign88590_e134901;
        locals.var_xmp_dn0 = assign88590_e134901_d_n0;
        locals.var_xmp_dn2 = assign88590_e134901_d_n2;
        locals.var_xmp_dn4 = assign88590_e134901_d_n4;
        locals.var_xmp_dn5 = assign88590_e134901_d_n5;
        locals.var_xmp_dn6 = assign88590_e134901_d_n6;
        locals.var_xmp_dn7 = assign88590_e134901_d_n7;
        locals.var_xmp_dn8 = assign88590_e134901_d_n8;
        locals.var_xmp_dn9 = assign88590_e134901_d_n9;
        locals.var_xmp_dn10 = assign88590_e134901_d_n10;
        locals.var_xmp_dn13 = assign88590_e134901_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign88600_e134914, assign88600_e134914_d_n0, assign88600_e134914_d_n2, assign88600_e134914_d_n4, assign88600_e134914_d_n5, assign88600_e134914_d_n6, assign88600_e134914_d_n7, assign88600_e134914_d_n8, assign88600_e134914_d_n9, assign88600_e134914_d_n10, assign88600_e134914_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2050 != 0.0)) && (locals.var_guard2051 != 0.0)) {
        let assign88600_e134912: f64 = (locals.var_xp + locals.var_xmp);
        (assign88600_e134912, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn13 + locals.var_xmp_dn13),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign88600_e134914;
        locals.var_arg_dn0 = assign88600_e134914_d_n0;
        locals.var_arg_dn2 = assign88600_e134914_d_n2;
        locals.var_arg_dn4 = assign88600_e134914_d_n4;
        locals.var_arg_dn5 = assign88600_e134914_d_n5;
        locals.var_arg_dn6 = assign88600_e134914_d_n6;
        locals.var_arg_dn7 = assign88600_e134914_d_n7;
        locals.var_arg_dn8 = assign88600_e134914_d_n8;
        locals.var_arg_dn9 = assign88600_e134914_d_n9;
        locals.var_arg_dn10 = assign88600_e134914_d_n10;
        locals.var_arg_dn13 = assign88600_e134914_d_n13;
        locals.var_arg_rv = 0.0;

        let (assign88610_e134925, assign88610_e134925_d_n0, assign88610_e134925_d_n2, assign88610_e134925_d_n4, assign88610_e134925_d_n5, assign88610_e134925_d_n6, assign88610_e134925_d_n7, assign88610_e134925_d_n8, assign88610_e134925_d_n9, assign88610_e134925_d_n10, assign88610_e134925_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2050 != 0.0)) && (locals.var_guard2051 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign88610_e134925;
        locals.var_dnm_dn0 = assign88610_e134925_d_n0;
        locals.var_dnm_dn2 = assign88610_e134925_d_n2;
        locals.var_dnm_dn4 = assign88610_e134925_d_n4;
        locals.var_dnm_dn5 = assign88610_e134925_d_n5;
        locals.var_dnm_dn6 = assign88610_e134925_d_n6;
        locals.var_dnm_dn7 = assign88610_e134925_d_n7;
        locals.var_dnm_dn8 = assign88610_e134925_d_n8;
        locals.var_dnm_dn9 = assign88610_e134925_d_n9;
        locals.var_dnm_dn10 = assign88610_e134925_d_n10;
        locals.var_dnm_dn13 = assign88610_e134925_d_n13;
        locals.var_dnm_rv = 0.0;

        let assign88620_e134940: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard2052 = assign88620_e134940;
        locals.var_guard2052_rv = 0.0;

        let assign88630_e134943: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard2053 = assign88630_e134943;
        locals.var_guard2053_rv = 0.0;

        let (assign88640_e134958,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2050 != 0.0)) && (locals.var_guard2051 != 0.0)) && (locals.var_guard2052 != 0.0)) && (locals.var_guard2053 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign88640_e134958;
        locals.var_mm_rv = 0.0;

        let assign88650_e134961: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard2054 = assign88650_e134961;
        locals.var_guard2054_rv = 0.0;

        let (assign88660_e134979,) = {
    if (((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2050 != 0.0)) && (locals.var_guard2051 != 0.0)) && (locals.var_guard2052 != 0.0)) && (locals.var_guard2053 == 0.0)) && (locals.var_guard2054 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign88660_e134979;
        locals.var_mm_rv = 0.0;

        let assign88670_e134982: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard2055 = assign88670_e134982;
        locals.var_guard2055_rv = 0.0;

        let (assign88680_e135003,) = {
    if ((((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2050 != 0.0)) && (locals.var_guard2051 != 0.0)) && (locals.var_guard2052 != 0.0)) && (locals.var_guard2053 == 0.0)) && (locals.var_guard2054 == 0.0)) && (locals.var_guard2055 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign88680_e135003;
        locals.var_mm_rv = 0.0;

        let assign88690_e135006: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard2056 = assign88690_e135006;
        locals.var_guard2056_rv = 0.0;

        let (assign88700_e135030,) = {
    if (((((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2050 != 0.0)) && (locals.var_guard2051 != 0.0)) && (locals.var_guard2052 != 0.0)) && (locals.var_guard2053 == 0.0)) && (locals.var_guard2054 == 0.0)) && (locals.var_guard2055 == 0.0)) && (locals.var_guard2056 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign88700_e135030;
        locals.var_mm_rv = 0.0;

        let (assign88710_e135043,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2050 != 0.0)) && (locals.var_guard2051 != 0.0)) && (locals.var_guard2052 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign88710_e135043;
        locals.var_m0_rv = 0.0;

        let mut assign88720_loop_guard: usize = 0;
        while {
            let assign88720_cond_e135057: f64 = if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2050 != 0.0)) && (locals.var_guard2051 != 0.0)) && (locals.var_guard2052 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign88720_cond_e135057 != 0.0
        } {
            assign88720_loop_guard += 1;
            assert!(assign88720_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign88720_body0_e135071, assign88720_body0_e135071_d_n0, assign88720_body0_e135071_d_n2, assign88720_body0_e135071_d_n4, assign88720_body0_e135071_d_n5, assign88720_body0_e135071_d_n6, assign88720_body0_e135071_d_n7, assign88720_body0_e135071_d_n8, assign88720_body0_e135071_d_n9, assign88720_body0_e135071_d_n10, assign88720_body0_e135071_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2050 != 0.0)) && (locals.var_guard2051 != 0.0)) && (locals.var_guard2052 != 0.0)) {
        let assign88720_body0_e135069: f64 = (locals.var_dnm).sqrt();
        (assign88720_body0_e135069, (locals.var_dnm_dn0 / (2.0 * assign88720_body0_e135069)), (locals.var_dnm_dn2 / (2.0 * assign88720_body0_e135069)), (locals.var_dnm_dn4 / (2.0 * assign88720_body0_e135069)), (locals.var_dnm_dn5 / (2.0 * assign88720_body0_e135069)), (locals.var_dnm_dn6 / (2.0 * assign88720_body0_e135069)), (locals.var_dnm_dn7 / (2.0 * assign88720_body0_e135069)), (locals.var_dnm_dn8 / (2.0 * assign88720_body0_e135069)), (locals.var_dnm_dn9 / (2.0 * assign88720_body0_e135069)), (locals.var_dnm_dn10 / (2.0 * assign88720_body0_e135069)), (locals.var_dnm_dn13 / (2.0 * assign88720_body0_e135069)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
            locals.var_dnm = assign88720_body0_e135071;
            locals.var_dnm_dn0 = assign88720_body0_e135071_d_n0;
            locals.var_dnm_dn2 = assign88720_body0_e135071_d_n2;
            locals.var_dnm_dn4 = assign88720_body0_e135071_d_n4;
            locals.var_dnm_dn5 = assign88720_body0_e135071_d_n5;
            locals.var_dnm_dn6 = assign88720_body0_e135071_d_n6;
            locals.var_dnm_dn7 = assign88720_body0_e135071_d_n7;
            locals.var_dnm_dn8 = assign88720_body0_e135071_d_n8;
            locals.var_dnm_dn9 = assign88720_body0_e135071_d_n9;
            locals.var_dnm_dn10 = assign88720_body0_e135071_d_n10;
            locals.var_dnm_dn13 = assign88720_body0_e135071_d_n13;
            locals.var_dnm_rv = 0.0;
            let (assign88720_body1_e135086,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2050 != 0.0)) && (locals.var_guard2051 != 0.0)) && (locals.var_guard2052 != 0.0)) {
        let assign88720_body1_e135084: f64 = (locals.var_m0 + 1.0);
        (assign88720_body1_e135084,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign88720_body1_e135086;
            locals.var_m0_rv = 0.0;
        }

        let (assign88730_e135111, assign88730_e135111_d_n0, assign88730_e135111_d_n2, assign88730_e135111_d_n4, assign88730_e135111_d_n5, assign88730_e135111_d_n6, assign88730_e135111_d_n7, assign88730_e135111_d_n8, assign88730_e135111_d_n9, assign88730_e135111_d_n10, assign88730_e135111_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2050 != 0.0)) && (locals.var_guard2051 != 0.0)) && (locals.var_guard2052 == 0.0)) {
        let (assign88730_e135109, assign88730_e135109_d_n0, assign88730_e135109_d_n2, assign88730_e135109_d_n4, assign88730_e135109_d_n5, assign88730_e135109_d_n6, assign88730_e135109_d_n7, assign88730_e135109_d_n8, assign88730_e135109_d_n9, assign88730_e135109_d_n10, assign88730_e135109_d_n13,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign88730_e135106: f64 = (2.0 * 2.0);
                let assign88730_e135107: f64 = (1.0 / assign88730_e135106);
                let assign88730_e135108: f64 = (locals.var_dnm).powf(assign88730_e135107);
                (assign88730_e135108, if 0.0 == 0.0 && ((assign88730_e135107) as f64).is_finite() && ((assign88730_e135107) as f64).fract() == 0.0 { if assign88730_e135107 == 0.0 { 0.0 } else { (assign88730_e135107 * ((locals.var_dnm).powf(assign88730_e135107 - 1.0) * locals.var_dnm_dn0)) } } else { (assign88730_e135108 * (assign88730_e135107 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign88730_e135107) as f64).is_finite() && ((assign88730_e135107) as f64).fract() == 0.0 { if assign88730_e135107 == 0.0 { 0.0 } else { (assign88730_e135107 * ((locals.var_dnm).powf(assign88730_e135107 - 1.0) * locals.var_dnm_dn2)) } } else { (assign88730_e135108 * (assign88730_e135107 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign88730_e135107) as f64).is_finite() && ((assign88730_e135107) as f64).fract() == 0.0 { if assign88730_e135107 == 0.0 { 0.0 } else { (assign88730_e135107 * ((locals.var_dnm).powf(assign88730_e135107 - 1.0) * locals.var_dnm_dn4)) } } else { (assign88730_e135108 * (assign88730_e135107 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign88730_e135107) as f64).is_finite() && ((assign88730_e135107) as f64).fract() == 0.0 { if assign88730_e135107 == 0.0 { 0.0 } else { (assign88730_e135107 * ((locals.var_dnm).powf(assign88730_e135107 - 1.0) * locals.var_dnm_dn5)) } } else { (assign88730_e135108 * (assign88730_e135107 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign88730_e135107) as f64).is_finite() && ((assign88730_e135107) as f64).fract() == 0.0 { if assign88730_e135107 == 0.0 { 0.0 } else { (assign88730_e135107 * ((locals.var_dnm).powf(assign88730_e135107 - 1.0) * locals.var_dnm_dn6)) } } else { (assign88730_e135108 * (assign88730_e135107 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign88730_e135107) as f64).is_finite() && ((assign88730_e135107) as f64).fract() == 0.0 { if assign88730_e135107 == 0.0 { 0.0 } else { (assign88730_e135107 * ((locals.var_dnm).powf(assign88730_e135107 - 1.0) * locals.var_dnm_dn7)) } } else { (assign88730_e135108 * (assign88730_e135107 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign88730_e135107) as f64).is_finite() && ((assign88730_e135107) as f64).fract() == 0.0 { if assign88730_e135107 == 0.0 { 0.0 } else { (assign88730_e135107 * ((locals.var_dnm).powf(assign88730_e135107 - 1.0) * locals.var_dnm_dn8)) } } else { (assign88730_e135108 * (assign88730_e135107 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign88730_e135107) as f64).is_finite() && ((assign88730_e135107) as f64).fract() == 0.0 { if assign88730_e135107 == 0.0 { 0.0 } else { (assign88730_e135107 * ((locals.var_dnm).powf(assign88730_e135107 - 1.0) * locals.var_dnm_dn9)) } } else { (assign88730_e135108 * (assign88730_e135107 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign88730_e135107) as f64).is_finite() && ((assign88730_e135107) as f64).fract() == 0.0 { if assign88730_e135107 == 0.0 { 0.0 } else { (assign88730_e135107 * ((locals.var_dnm).powf(assign88730_e135107 - 1.0) * locals.var_dnm_dn10)) } } else { (assign88730_e135108 * (assign88730_e135107 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign88730_e135107) as f64).is_finite() && ((assign88730_e135107) as f64).fract() == 0.0 { if assign88730_e135107 == 0.0 { 0.0 } else { (assign88730_e135107 * ((locals.var_dnm).powf(assign88730_e135107 - 1.0) * locals.var_dnm_dn13)) } } else { (assign88730_e135108 * (assign88730_e135107 * (locals.var_dnm_dn13 / locals.var_dnm))) },)
            }
        };
        (assign88730_e135109, assign88730_e135109_d_n0, assign88730_e135109_d_n2, assign88730_e135109_d_n4, assign88730_e135109_d_n5, assign88730_e135109_d_n6, assign88730_e135109_d_n7, assign88730_e135109_d_n8, assign88730_e135109_d_n9, assign88730_e135109_d_n10, assign88730_e135109_d_n13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign88730_e135111;
        locals.var_dnm_dn0 = assign88730_e135111_d_n0;
        locals.var_dnm_dn2 = assign88730_e135111_d_n2;
        locals.var_dnm_dn4 = assign88730_e135111_d_n4;
        locals.var_dnm_dn5 = assign88730_e135111_d_n5;
        locals.var_dnm_dn6 = assign88730_e135111_d_n6;
        locals.var_dnm_dn7 = assign88730_e135111_d_n7;
        locals.var_dnm_dn8 = assign88730_e135111_d_n8;
        locals.var_dnm_dn9 = assign88730_e135111_d_n9;
        locals.var_dnm_dn10 = assign88730_e135111_d_n10;
        locals.var_dnm_dn13 = assign88730_e135111_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign88740_e135124, assign88740_e135124_d_n0, assign88740_e135124_d_n2, assign88740_e135124_d_n4, assign88740_e135124_d_n5, assign88740_e135124_d_n6, assign88740_e135124_d_n7, assign88740_e135124_d_n8, assign88740_e135124_d_n9, assign88740_e135124_d_n10, assign88740_e135124_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2050 != 0.0)) && (locals.var_guard2051 != 0.0)) {
        let assign88740_e135122: f64 = (1.0 / locals.var_dnm);
        (assign88740_e135122, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn13 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign88740_e135124;
        locals.var_dnm_dn0 = assign88740_e135124_d_n0;
        locals.var_dnm_dn2 = assign88740_e135124_d_n2;
        locals.var_dnm_dn4 = assign88740_e135124_d_n4;
        locals.var_dnm_dn5 = assign88740_e135124_d_n5;
        locals.var_dnm_dn6 = assign88740_e135124_d_n6;
        locals.var_dnm_dn7 = assign88740_e135124_d_n7;
        locals.var_dnm_dn8 = assign88740_e135124_d_n8;
        locals.var_dnm_dn9 = assign88740_e135124_d_n9;
        locals.var_dnm_dn10 = assign88740_e135124_d_n10;
        locals.var_dnm_dn13 = assign88740_e135124_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign88750_e135141, assign88750_e135141_d_n0, assign88750_e135141_d_n2, assign88750_e135141_d_n4, assign88750_e135141_d_n5, assign88750_e135141_d_n6, assign88750_e135141_d_n7, assign88750_e135141_d_n8, assign88750_e135141_d_n9, assign88750_e135141_d_n10, assign88750_e135141_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2050 != 0.0)) && (locals.var_guard2051 != 0.0)) {
        let assign88750_e135136: f64 = (locals.var_ddriftldc * 0.1);
        let assign88750_e135137: f64 = (locals.var_tmf1 * assign88750_e135136);
        let assign88750_e135139: f64 = (assign88750_e135137 * locals.var_dnm);
        (assign88750_e135139, ((((locals.var_tmf1_dn0 * assign88750_e135136) + (locals.var_tmf1 * (locals.var_ddriftldc_dn0 * 0.1))) * locals.var_dnm) + (assign88750_e135137 * locals.var_dnm_dn0)), ((((locals.var_tmf1_dn2 * assign88750_e135136) + (locals.var_tmf1 * (locals.var_ddriftldc_dn2 * 0.1))) * locals.var_dnm) + (assign88750_e135137 * locals.var_dnm_dn2)), ((((locals.var_tmf1_dn4 * assign88750_e135136) + (locals.var_tmf1 * (locals.var_ddriftldc_dn4 * 0.1))) * locals.var_dnm) + (assign88750_e135137 * locals.var_dnm_dn4)), ((((locals.var_tmf1_dn5 * assign88750_e135136) + (locals.var_tmf1 * (locals.var_ddriftldc_dn5 * 0.1))) * locals.var_dnm) + (assign88750_e135137 * locals.var_dnm_dn5)), ((((locals.var_tmf1_dn6 * assign88750_e135136) + (locals.var_tmf1 * (locals.var_ddriftldc_dn6 * 0.1))) * locals.var_dnm) + (assign88750_e135137 * locals.var_dnm_dn6)), ((((locals.var_tmf1_dn7 * assign88750_e135136) + (locals.var_tmf1 * (locals.var_ddriftldc_dn7 * 0.1))) * locals.var_dnm) + (assign88750_e135137 * locals.var_dnm_dn7)), ((((locals.var_tmf1_dn8 * assign88750_e135136) + (locals.var_tmf1 * (locals.var_ddriftldc_dn8 * 0.1))) * locals.var_dnm) + (assign88750_e135137 * locals.var_dnm_dn8)), ((((locals.var_tmf1_dn9 * assign88750_e135136) + (locals.var_tmf1 * (locals.var_ddriftldc_dn9 * 0.1))) * locals.var_dnm) + (assign88750_e135137 * locals.var_dnm_dn9)), ((((locals.var_tmf1_dn10 * assign88750_e135136) + (locals.var_tmf1 * (locals.var_ddriftldc_dn10 * 0.1))) * locals.var_dnm) + (assign88750_e135137 * locals.var_dnm_dn10)), ((((locals.var_tmf1_dn13 * assign88750_e135136) + (locals.var_tmf1 * (locals.var_ddriftldc_dn13 * 0.1))) * locals.var_dnm) + (assign88750_e135137 * locals.var_dnm_dn13)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn13,)
    }
};
        locals.var_tmf0 = assign88750_e135141;
        locals.var_tmf0_dn0 = assign88750_e135141_d_n0;
        locals.var_tmf0_dn2 = assign88750_e135141_d_n2;
        locals.var_tmf0_dn4 = assign88750_e135141_d_n4;
        locals.var_tmf0_dn5 = assign88750_e135141_d_n5;
        locals.var_tmf0_dn6 = assign88750_e135141_d_n6;
        locals.var_tmf0_dn7 = assign88750_e135141_d_n7;
        locals.var_tmf0_dn8 = assign88750_e135141_d_n8;
        locals.var_tmf0_dn9 = assign88750_e135141_d_n9;
        locals.var_tmf0_dn10 = assign88750_e135141_d_n10;
        locals.var_tmf0_dn13 = assign88750_e135141_d_n13;
        locals.var_tmf0_rv = 0.0;

        let (assign88760_e135160, assign88760_e135160_d_n0, assign88760_e135160_d_n2, assign88760_e135160_d_n4, assign88760_e135160_d_n5, assign88760_e135160_d_n6, assign88760_e135160_d_n7, assign88760_e135160_d_n8, assign88760_e135160_d_n9, assign88760_e135160_d_n10, assign88760_e135160_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2050 != 0.0)) && (locals.var_guard2051 != 0.0)) {
        let assign88760_e135152: f64 = (locals.var_ddriftldc * 0.1);
        let assign88760_e135154: f64 = (assign88760_e135152 * locals.var_xmp);
        let assign88760_e135156: f64 = (assign88760_e135154 * locals.var_dnm);
        let assign88760_e135158: f64 = (assign88760_e135156 / locals.var_arg);
        (assign88760_e135158, ((((((((locals.var_ddriftldc_dn0 * 0.1) * locals.var_xmp) + (assign88760_e135152 * locals.var_xmp_dn0)) * locals.var_dnm) + (assign88760_e135154 * locals.var_dnm_dn0)) * locals.var_arg) - (assign88760_e135156 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((((locals.var_ddriftldc_dn2 * 0.1) * locals.var_xmp) + (assign88760_e135152 * locals.var_xmp_dn2)) * locals.var_dnm) + (assign88760_e135154 * locals.var_dnm_dn2)) * locals.var_arg) - (assign88760_e135156 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((((locals.var_ddriftldc_dn4 * 0.1) * locals.var_xmp) + (assign88760_e135152 * locals.var_xmp_dn4)) * locals.var_dnm) + (assign88760_e135154 * locals.var_dnm_dn4)) * locals.var_arg) - (assign88760_e135156 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((((locals.var_ddriftldc_dn5 * 0.1) * locals.var_xmp) + (assign88760_e135152 * locals.var_xmp_dn5)) * locals.var_dnm) + (assign88760_e135154 * locals.var_dnm_dn5)) * locals.var_arg) - (assign88760_e135156 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((((locals.var_ddriftldc_dn6 * 0.1) * locals.var_xmp) + (assign88760_e135152 * locals.var_xmp_dn6)) * locals.var_dnm) + (assign88760_e135154 * locals.var_dnm_dn6)) * locals.var_arg) - (assign88760_e135156 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((((locals.var_ddriftldc_dn7 * 0.1) * locals.var_xmp) + (assign88760_e135152 * locals.var_xmp_dn7)) * locals.var_dnm) + (assign88760_e135154 * locals.var_dnm_dn7)) * locals.var_arg) - (assign88760_e135156 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((((locals.var_ddriftldc_dn8 * 0.1) * locals.var_xmp) + (assign88760_e135152 * locals.var_xmp_dn8)) * locals.var_dnm) + (assign88760_e135154 * locals.var_dnm_dn8)) * locals.var_arg) - (assign88760_e135156 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((((locals.var_ddriftldc_dn9 * 0.1) * locals.var_xmp) + (assign88760_e135152 * locals.var_xmp_dn9)) * locals.var_dnm) + (assign88760_e135154 * locals.var_dnm_dn9)) * locals.var_arg) - (assign88760_e135156 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((((locals.var_ddriftldc_dn10 * 0.1) * locals.var_xmp) + (assign88760_e135152 * locals.var_xmp_dn10)) * locals.var_dnm) + (assign88760_e135154 * locals.var_dnm_dn10)) * locals.var_arg) - (assign88760_e135156 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((((locals.var_ddriftldc_dn13 * 0.1) * locals.var_xmp) + (assign88760_e135152 * locals.var_xmp_dn13)) * locals.var_dnm) + (assign88760_e135154 * locals.var_dnm_dn13)) * locals.var_arg) - (assign88760_e135156 * locals.var_arg_dn13)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign88760_e135160;
        locals.var_t0_dn0 = assign88760_e135160_d_n0;
        locals.var_t0_dn2 = assign88760_e135160_d_n2;
        locals.var_t0_dn4 = assign88760_e135160_d_n4;
        locals.var_t0_dn5 = assign88760_e135160_d_n5;
        locals.var_t0_dn6 = assign88760_e135160_d_n6;
        locals.var_t0_dn7 = assign88760_e135160_d_n7;
        locals.var_t0_dn8 = assign88760_e135160_d_n8;
        locals.var_t0_dn9 = assign88760_e135160_d_n9;
        locals.var_t0_dn10 = assign88760_e135160_d_n10;
        locals.var_t0_dn13 = assign88760_e135160_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign88770_e135177, assign88770_e135177_d_n0, assign88770_e135177_d_n2, assign88770_e135177_d_n4, assign88770_e135177_d_n5, assign88770_e135177_d_n6, assign88770_e135177_d_n7, assign88770_e135177_d_n8, assign88770_e135177_d_n9, assign88770_e135177_d_n10, assign88770_e135177_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2050 != 0.0)) && (locals.var_guard2051 != 0.0)) {
        let assign88770_e135172: f64 = (locals.var_ddriftldc * 0.1);
        let assign88770_e135173: f64 = (locals.var_ddriftldc - assign88770_e135172);
        let assign88770_e135175: f64 = (assign88770_e135173 + locals.var_tmf0);
        (assign88770_e135175, ((locals.var_ddriftldc_dn0 - (locals.var_ddriftldc_dn0 * 0.1)) + locals.var_tmf0_dn0), ((locals.var_ddriftldc_dn2 - (locals.var_ddriftldc_dn2 * 0.1)) + locals.var_tmf0_dn2), ((locals.var_ddriftldc_dn4 - (locals.var_ddriftldc_dn4 * 0.1)) + locals.var_tmf0_dn4), ((locals.var_ddriftldc_dn5 - (locals.var_ddriftldc_dn5 * 0.1)) + locals.var_tmf0_dn5), ((locals.var_ddriftldc_dn6 - (locals.var_ddriftldc_dn6 * 0.1)) + locals.var_tmf0_dn6), ((locals.var_ddriftldc_dn7 - (locals.var_ddriftldc_dn7 * 0.1)) + locals.var_tmf0_dn7), ((locals.var_ddriftldc_dn8 - (locals.var_ddriftldc_dn8 * 0.1)) + locals.var_tmf0_dn8), ((locals.var_ddriftldc_dn9 - (locals.var_ddriftldc_dn9 * 0.1)) + locals.var_tmf0_dn9), ((locals.var_ddriftldc_dn10 - (locals.var_ddriftldc_dn10 * 0.1)) + locals.var_tmf0_dn10), ((locals.var_ddriftldc_dn13 - (locals.var_ddriftldc_dn13 * 0.1)) + locals.var_tmf0_dn13),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign88770_e135177;
        locals.var_t1_dn0 = assign88770_e135177_d_n0;
        locals.var_t1_dn2 = assign88770_e135177_d_n2;
        locals.var_t1_dn4 = assign88770_e135177_d_n4;
        locals.var_t1_dn5 = assign88770_e135177_d_n5;
        locals.var_t1_dn6 = assign88770_e135177_d_n6;
        locals.var_t1_dn7 = assign88770_e135177_d_n7;
        locals.var_t1_dn8 = assign88770_e135177_d_n8;
        locals.var_t1_dn9 = assign88770_e135177_d_n9;
        locals.var_t1_dn10 = assign88770_e135177_d_n10;
        locals.var_t1_dn13 = assign88770_e135177_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign88780_e135188, assign88780_e135188_d_n0, assign88780_e135188_d_n2, assign88780_e135188_d_n4, assign88780_e135188_d_n5, assign88780_e135188_d_n6, assign88780_e135188_d_n7, assign88780_e135188_d_n8, assign88780_e135188_d_n9, assign88780_e135188_d_n10, assign88780_e135188_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2050 != 0.0)) && (locals.var_guard2051 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign88780_e135188;
        locals.var_t0_dn0 = assign88780_e135188_d_n0;
        locals.var_t0_dn2 = assign88780_e135188_d_n2;
        locals.var_t0_dn4 = assign88780_e135188_d_n4;
        locals.var_t0_dn5 = assign88780_e135188_d_n5;
        locals.var_t0_dn6 = assign88780_e135188_d_n6;
        locals.var_t0_dn7 = assign88780_e135188_d_n7;
        locals.var_t0_dn8 = assign88780_e135188_d_n8;
        locals.var_t0_dn9 = assign88780_e135188_d_n9;
        locals.var_t0_dn10 = assign88780_e135188_d_n10;
        locals.var_t0_dn13 = assign88780_e135188_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign88790_e135200, assign88790_e135200_d_n0, assign88790_e135200_d_n2, assign88790_e135200_d_n4, assign88790_e135200_d_n5, assign88790_e135200_d_n6, assign88790_e135200_d_n7, assign88790_e135200_d_n8, assign88790_e135200_d_n9, assign88790_e135200_d_n10, assign88790_e135200_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2050 != 0.0)) && (locals.var_guard2051 == 0.0)) {
        (locals.var_wdld0__blk2048, locals.var_wdld0__blk2048_dn0, locals.var_wdld0__blk2048_dn2, locals.var_wdld0__blk2048_dn4, locals.var_wdld0__blk2048_dn5, locals.var_wdld0__blk2048_dn6, locals.var_wdld0__blk2048_dn7, locals.var_wdld0__blk2048_dn8, locals.var_wdld0__blk2048_dn9, locals.var_wdld0__blk2048_dn10, locals.var_wdld0__blk2048_dn13,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign88790_e135200;
        locals.var_t1_dn0 = assign88790_e135200_d_n0;
        locals.var_t1_dn2 = assign88790_e135200_d_n2;
        locals.var_t1_dn4 = assign88790_e135200_d_n4;
        locals.var_t1_dn5 = assign88790_e135200_d_n5;
        locals.var_t1_dn6 = assign88790_e135200_d_n6;
        locals.var_t1_dn7 = assign88790_e135200_d_n7;
        locals.var_t1_dn8 = assign88790_e135200_d_n8;
        locals.var_t1_dn9 = assign88790_e135200_d_n9;
        locals.var_t1_dn10 = assign88790_e135200_d_n10;
        locals.var_t1_dn13 = assign88790_e135200_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign88800_e135212, assign88800_e135212_d_n0, assign88800_e135212_d_n2, assign88800_e135212_d_n4, assign88800_e135212_d_n5, assign88800_e135212_d_n6, assign88800_e135212_d_n7, assign88800_e135212_d_n8, assign88800_e135212_d_n9, assign88800_e135212_d_n10, assign88800_e135212_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2050 != 0.0)) && (locals.var_guard2051 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign88800_e135212;
        locals.var_t0_dn0 = assign88800_e135212_d_n0;
        locals.var_t0_dn2 = assign88800_e135212_d_n2;
        locals.var_t0_dn4 = assign88800_e135212_d_n4;
        locals.var_t0_dn5 = assign88800_e135212_d_n5;
        locals.var_t0_dn6 = assign88800_e135212_d_n6;
        locals.var_t0_dn7 = assign88800_e135212_d_n7;
        locals.var_t0_dn8 = assign88800_e135212_d_n8;
        locals.var_t0_dn9 = assign88800_e135212_d_n9;
        locals.var_t0_dn10 = assign88800_e135212_d_n10;
        locals.var_t0_dn13 = assign88800_e135212_d_n13;
        locals.var_t0_rv = 0.0;

        let assign88810_e135215: f64 = if locals.var_t0 < 1.0 { 1.0 } else { 0.0 };
        locals.var_guard2057 = assign88810_e135215;
        locals.var_guard2057_rv = 0.0;

        let (assign88820_e135228,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2050 != 0.0)) && (locals.var_guard2057 != 0.0)) {
        let assign88820_e135226: f64 = (locals.var_flg_fd_mode__blk2010 + 2.0);
        (assign88820_e135226,)
    } else {
        (locals.var_flg_fd_mode__blk2010,)
    }
};
        locals.var_flg_fd_mode__blk2010 = assign88820_e135228;
        locals.var_flg_fd_mode__blk2010_rv = 0.0;

        let (assign88830_e135243, assign88830_e135243_d_n0, assign88830_e135243_d_n2, assign88830_e135243_d_n4, assign88830_e135243_d_n5, assign88830_e135243_d_n6, assign88830_e135243_d_n7, assign88830_e135243_d_n8, assign88830_e135243_d_n9, assign88830_e135243_d_n10, assign88830_e135243_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2050 == 0.0)) {
        let (assign88830_e135241, assign88830_e135241_d_n0, assign88830_e135241_d_n2, assign88830_e135241_d_n4, assign88830_e135241_d_n5, assign88830_e135241_d_n6, assign88830_e135241_d_n7, assign88830_e135241_d_n8, assign88830_e135241_d_n9, assign88830_e135241_d_n10, assign88830_e135241_d_n13,) = {
            if (locals.var_wdld0__blk2048 <= locals.var_ddriftldc) {
                (locals.var_wdld0__blk2048, locals.var_wdld0__blk2048_dn0, locals.var_wdld0__blk2048_dn2, locals.var_wdld0__blk2048_dn4, locals.var_wdld0__blk2048_dn5, locals.var_wdld0__blk2048_dn6, locals.var_wdld0__blk2048_dn7, locals.var_wdld0__blk2048_dn8, locals.var_wdld0__blk2048_dn9, locals.var_wdld0__blk2048_dn10, locals.var_wdld0__blk2048_dn13,)
            } else {
                (locals.var_ddriftldc, locals.var_ddriftldc_dn0, locals.var_ddriftldc_dn2, locals.var_ddriftldc_dn4, locals.var_ddriftldc_dn5, locals.var_ddriftldc_dn6, locals.var_ddriftldc_dn7, locals.var_ddriftldc_dn8, locals.var_ddriftldc_dn9, locals.var_ddriftldc_dn10, locals.var_ddriftldc_dn13,)
            }
        };
        (assign88830_e135241, assign88830_e135241_d_n0, assign88830_e135241_d_n2, assign88830_e135241_d_n4, assign88830_e135241_d_n5, assign88830_e135241_d_n6, assign88830_e135241_d_n7, assign88830_e135241_d_n8, assign88830_e135241_d_n9, assign88830_e135241_d_n10, assign88830_e135241_d_n13,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign88830_e135243;
        locals.var_t1_dn0 = assign88830_e135243_d_n0;
        locals.var_t1_dn2 = assign88830_e135243_d_n2;
        locals.var_t1_dn4 = assign88830_e135243_d_n4;
        locals.var_t1_dn5 = assign88830_e135243_d_n5;
        locals.var_t1_dn6 = assign88830_e135243_d_n6;
        locals.var_t1_dn7 = assign88830_e135243_d_n7;
        locals.var_t1_dn8 = assign88830_e135243_d_n8;
        locals.var_t1_dn9 = assign88830_e135243_d_n9;
        locals.var_t1_dn10 = assign88830_e135243_d_n10;
        locals.var_t1_dn13 = assign88830_e135243_d_n13;
        locals.var_t1_rv = 0.0;

        let assign88840_e135246: f64 = if locals.var_wdld0__blk2048 >= locals.var_ddriftldc { 1.0 } else { 0.0 };
        locals.var_guard2058 = assign88840_e135246;
        locals.var_guard2058_rv = 0.0;

        let (assign88850_e135260,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2050 == 0.0)) && (locals.var_guard2058 != 0.0)) {
        let assign88850_e135258: f64 = (locals.var_flg_fd_mode__blk2010 + 2.0);
        (assign88850_e135258,)
    } else {
        (locals.var_flg_fd_mode__blk2010,)
    }
};
        locals.var_flg_fd_mode__blk2010 = assign88850_e135260;
        locals.var_flg_fd_mode__blk2010_rv = 0.0;

        let assign88860_e135263: f64 = if locals.var_flg_fd_mode__blk2010 >= 2.0 { 1.0 } else { 0.0 };
        locals.var_guard2059 = assign88860_e135263;
        locals.var_guard2059_rv = 0.0;

        let (assign88870_e135272, assign88870_e135272_d_n0, assign88870_e135272_d_n2, assign88870_e135272_d_n4, assign88870_e135272_d_n5, assign88870_e135272_d_n6, assign88870_e135272_d_n7, assign88870_e135272_d_n8, assign88870_e135272_d_n9, assign88870_e135272_d_n10, assign88870_e135272_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2059 != 0.0)) {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn13,)
    } else {
        (locals.var_ps0ld_bef1__blk2049, locals.var_ps0ld_bef1__blk2049_dn0, locals.var_ps0ld_bef1__blk2049_dn2, locals.var_ps0ld_bef1__blk2049_dn4, locals.var_ps0ld_bef1__blk2049_dn5, locals.var_ps0ld_bef1__blk2049_dn6, locals.var_ps0ld_bef1__blk2049_dn7, locals.var_ps0ld_bef1__blk2049_dn8, locals.var_ps0ld_bef1__blk2049_dn9, locals.var_ps0ld_bef1__blk2049_dn10, locals.var_ps0ld_bef1__blk2049_dn13,)
    }
};
        locals.var_ps0ld_bef1__blk2049 = assign88870_e135272;
        locals.var_ps0ld_bef1__blk2049_dn0 = assign88870_e135272_d_n0;
        locals.var_ps0ld_bef1__blk2049_dn2 = assign88870_e135272_d_n2;
        locals.var_ps0ld_bef1__blk2049_dn4 = assign88870_e135272_d_n4;
        locals.var_ps0ld_bef1__blk2049_dn5 = assign88870_e135272_d_n5;
        locals.var_ps0ld_bef1__blk2049_dn6 = assign88870_e135272_d_n6;
        locals.var_ps0ld_bef1__blk2049_dn7 = assign88870_e135272_d_n7;
        locals.var_ps0ld_bef1__blk2049_dn8 = assign88870_e135272_d_n8;
        locals.var_ps0ld_bef1__blk2049_dn9 = assign88870_e135272_d_n9;
        locals.var_ps0ld_bef1__blk2049_dn10 = assign88870_e135272_d_n10;
        locals.var_ps0ld_bef1__blk2049_dn13 = assign88870_e135272_d_n13;
        locals.var_ps0ld_bef1__blk2049_rv = 0.0;

        let (assign88880_e135283, assign88880_e135283_d_n0, assign88880_e135283_d_n2, assign88880_e135283_d_n4, assign88880_e135283_d_n5, assign88880_e135283_d_n6, assign88880_e135283_d_n7, assign88880_e135283_d_n8, assign88880_e135283_d_n9, assign88880_e135283_d_n10, assign88880_e135283_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2059 != 0.0)) {
        let assign88880_e135281: f64 = (locals.var_t1 * locals.var_q_nsubld__blk2004);
        (assign88880_e135281, (locals.var_t1_dn0 * locals.var_q_nsubld__blk2004), (locals.var_t1_dn2 * locals.var_q_nsubld__blk2004), (locals.var_t1_dn4 * locals.var_q_nsubld__blk2004), (locals.var_t1_dn5 * locals.var_q_nsubld__blk2004), (locals.var_t1_dn6 * locals.var_q_nsubld__blk2004), (locals.var_t1_dn7 * locals.var_q_nsubld__blk2004), (locals.var_t1_dn8 * locals.var_q_nsubld__blk2004), (locals.var_t1_dn9 * locals.var_q_nsubld__blk2004), (locals.var_t1_dn10 * locals.var_q_nsubld__blk2004), (locals.var_t1_dn13 * locals.var_q_nsubld__blk2004),)
    } else {
        (locals.var_qbuld, locals.var_qbuld_dn0, locals.var_qbuld_dn2, locals.var_qbuld_dn4, locals.var_qbuld_dn5, locals.var_qbuld_dn6, locals.var_qbuld_dn7, locals.var_qbuld_dn8, locals.var_qbuld_dn9, locals.var_qbuld_dn10, locals.var_qbuld_dn13,)
    }
};
        locals.var_qbuld = assign88880_e135283;
        locals.var_qbuld_dn0 = assign88880_e135283_d_n0;
        locals.var_qbuld_dn2 = assign88880_e135283_d_n2;
        locals.var_qbuld_dn4 = assign88880_e135283_d_n4;
        locals.var_qbuld_dn5 = assign88880_e135283_d_n5;
        locals.var_qbuld_dn6 = assign88880_e135283_d_n6;
        locals.var_qbuld_dn7 = assign88880_e135283_d_n7;
        locals.var_qbuld_dn8 = assign88880_e135283_d_n8;
        locals.var_qbuld_dn9 = assign88880_e135283_d_n9;
        locals.var_qbuld_dn10 = assign88880_e135283_d_n10;
        locals.var_qbuld_dn13 = assign88880_e135283_d_n13;
        locals.var_qbuld_rv = 0.0;

        let (assign88890_e135296, assign88890_e135296_d_n0, assign88890_e135296_d_n2, assign88890_e135296_d_n4, assign88890_e135296_d_n5, assign88890_e135296_d_n6, assign88890_e135296_d_n7, assign88890_e135296_d_n8, assign88890_e135296_d_n9, assign88890_e135296_d_n10, assign88890_e135296_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2059 != 0.0)) {
        let assign88890_e135293: f64 = (locals.var_qbuld / locals.var_cox0_func);
        let assign88890_e135294: f64 = (locals.var_vgpld - assign88890_e135293);
        (assign88890_e135294, (-(locals.var_qbuld_dn0 / locals.var_cox0_func)), (locals.var_vgpld_dn2 - (locals.var_qbuld_dn2 / locals.var_cox0_func)), (-(locals.var_qbuld_dn4 / locals.var_cox0_func)), (-(locals.var_qbuld_dn5 / locals.var_cox0_func)), (locals.var_vgpld_dn6 - (locals.var_qbuld_dn6 / locals.var_cox0_func)), (locals.var_vgpld_dn7 - (locals.var_qbuld_dn7 / locals.var_cox0_func)), (locals.var_vgpld_dn8 - (locals.var_qbuld_dn8 / locals.var_cox0_func)), (-(locals.var_qbuld_dn9 / locals.var_cox0_func)), (-(locals.var_qbuld_dn10 / locals.var_cox0_func)), (-(locals.var_qbuld_dn13 / locals.var_cox0_func)),)
    } else {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn13,)
    }
};
        locals.var_ps0ld = assign88890_e135296;
        locals.var_ps0ld_dn0 = assign88890_e135296_d_n0;
        locals.var_ps0ld_dn2 = assign88890_e135296_d_n2;
        locals.var_ps0ld_dn4 = assign88890_e135296_d_n4;
        locals.var_ps0ld_dn5 = assign88890_e135296_d_n5;
        locals.var_ps0ld_dn6 = assign88890_e135296_d_n6;
        locals.var_ps0ld_dn7 = assign88890_e135296_d_n7;
        locals.var_ps0ld_dn8 = assign88890_e135296_d_n8;
        locals.var_ps0ld_dn9 = assign88890_e135296_d_n9;
        locals.var_ps0ld_dn10 = assign88890_e135296_d_n10;
        locals.var_ps0ld_dn13 = assign88890_e135296_d_n13;
        locals.var_ps0ld_rv = 0.0;

        let assign88900_e135299: f64 = if p.p33 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard2060 = assign88900_e135299;
        locals.var_guard2060_rv = 0.0;

        let assign88910_e135303: f64 = (locals.var_ps0ld_bef1__blk2049 - 0.1);
        let assign88910_e135308: f64 = if ((locals.var_ps0ld > assign88910_e135303) && (0.1 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard2061 = assign88910_e135308;
        locals.var_guard2061_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_329(
        locals: &mut StampLocals,
    ) {
        let (assign88920_e135325, assign88920_e135325_d_n0, assign88920_e135325_d_n2, assign88920_e135325_d_n4, assign88920_e135325_d_n5, assign88920_e135325_d_n6, assign88920_e135325_d_n7, assign88920_e135325_d_n8, assign88920_e135325_d_n9, assign88920_e135325_d_n10, assign88920_e135325_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2059 != 0.0)) && (locals.var_guard2060 != 0.0)) && (locals.var_guard2061 != 0.0)) {
        let assign88920_e135321: f64 = (locals.var_ps0ld - locals.var_ps0ld_bef1__blk2049);
        let assign88920_e135323: f64 = (assign88920_e135321 + 0.1);
        (assign88920_e135323, (locals.var_ps0ld_dn0 - locals.var_ps0ld_bef1__blk2049_dn0), (locals.var_ps0ld_dn2 - locals.var_ps0ld_bef1__blk2049_dn2), (locals.var_ps0ld_dn4 - locals.var_ps0ld_bef1__blk2049_dn4), (locals.var_ps0ld_dn5 - locals.var_ps0ld_bef1__blk2049_dn5), (locals.var_ps0ld_dn6 - locals.var_ps0ld_bef1__blk2049_dn6), (locals.var_ps0ld_dn7 - locals.var_ps0ld_bef1__blk2049_dn7), (locals.var_ps0ld_dn8 - locals.var_ps0ld_bef1__blk2049_dn8), (locals.var_ps0ld_dn9 - locals.var_ps0ld_bef1__blk2049_dn9), (locals.var_ps0ld_dn10 - locals.var_ps0ld_bef1__blk2049_dn10), (locals.var_ps0ld_dn13 - locals.var_ps0ld_bef1__blk2049_dn13),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign88920_e135325;
        locals.var_tmf1_dn0 = assign88920_e135325_d_n0;
        locals.var_tmf1_dn2 = assign88920_e135325_d_n2;
        locals.var_tmf1_dn4 = assign88920_e135325_d_n4;
        locals.var_tmf1_dn5 = assign88920_e135325_d_n5;
        locals.var_tmf1_dn6 = assign88920_e135325_d_n6;
        locals.var_tmf1_dn7 = assign88920_e135325_d_n7;
        locals.var_tmf1_dn8 = assign88920_e135325_d_n8;
        locals.var_tmf1_dn9 = assign88920_e135325_d_n9;
        locals.var_tmf1_dn10 = assign88920_e135325_d_n10;
        locals.var_tmf1_dn13 = assign88920_e135325_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign88930_e135340, assign88930_e135340_d_n0, assign88930_e135340_d_n2, assign88930_e135340_d_n4, assign88930_e135340_d_n5, assign88930_e135340_d_n6, assign88930_e135340_d_n7, assign88930_e135340_d_n8, assign88930_e135340_d_n9, assign88930_e135340_d_n10, assign88930_e135340_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2059 != 0.0)) && (locals.var_guard2060 != 0.0)) && (locals.var_guard2061 != 0.0)) {
        let assign88930_e135338: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign88930_e135338, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn13,)
    }
};
        locals.var_x2 = assign88930_e135340;
        locals.var_x2_dn0 = assign88930_e135340_d_n0;
        locals.var_x2_dn2 = assign88930_e135340_d_n2;
        locals.var_x2_dn4 = assign88930_e135340_d_n4;
        locals.var_x2_dn5 = assign88930_e135340_d_n5;
        locals.var_x2_dn6 = assign88930_e135340_d_n6;
        locals.var_x2_dn7 = assign88930_e135340_d_n7;
        locals.var_x2_dn8 = assign88930_e135340_d_n8;
        locals.var_x2_dn9 = assign88930_e135340_d_n9;
        locals.var_x2_dn10 = assign88930_e135340_d_n10;
        locals.var_x2_dn13 = assign88930_e135340_d_n13;
        locals.var_x2_rv = 0.0;

        let (assign88940_e135355, assign88940_e135355_d_n0, assign88940_e135355_d_n2, assign88940_e135355_d_n4, assign88940_e135355_d_n5, assign88940_e135355_d_n6, assign88940_e135355_d_n7, assign88940_e135355_d_n8, assign88940_e135355_d_n9, assign88940_e135355_d_n10, assign88940_e135355_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2059 != 0.0)) && (locals.var_guard2060 != 0.0)) && (locals.var_guard2061 != 0.0)) {
        let assign88940_e135353: f64 = (0.1 * 0.1);
        (assign88940_e135353, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn13,)
    }
};
        locals.var_xmax2 = assign88940_e135355;
        locals.var_xmax2_dn0 = assign88940_e135355_d_n0;
        locals.var_xmax2_dn2 = assign88940_e135355_d_n2;
        locals.var_xmax2_dn4 = assign88940_e135355_d_n4;
        locals.var_xmax2_dn5 = assign88940_e135355_d_n5;
        locals.var_xmax2_dn6 = assign88940_e135355_d_n6;
        locals.var_xmax2_dn7 = assign88940_e135355_d_n7;
        locals.var_xmax2_dn8 = assign88940_e135355_d_n8;
        locals.var_xmax2_dn9 = assign88940_e135355_d_n9;
        locals.var_xmax2_dn10 = assign88940_e135355_d_n10;
        locals.var_xmax2_dn13 = assign88940_e135355_d_n13;
        locals.var_xmax2_rv = 0.0;

        let (assign88950_e135368, assign88950_e135368_d_n0, assign88950_e135368_d_n2, assign88950_e135368_d_n4, assign88950_e135368_d_n5, assign88950_e135368_d_n6, assign88950_e135368_d_n7, assign88950_e135368_d_n8, assign88950_e135368_d_n9, assign88950_e135368_d_n10, assign88950_e135368_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2059 != 0.0)) && (locals.var_guard2060 != 0.0)) && (locals.var_guard2061 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign88950_e135368;
        locals.var_xp_dn0 = assign88950_e135368_d_n0;
        locals.var_xp_dn2 = assign88950_e135368_d_n2;
        locals.var_xp_dn4 = assign88950_e135368_d_n4;
        locals.var_xp_dn5 = assign88950_e135368_d_n5;
        locals.var_xp_dn6 = assign88950_e135368_d_n6;
        locals.var_xp_dn7 = assign88950_e135368_d_n7;
        locals.var_xp_dn8 = assign88950_e135368_d_n8;
        locals.var_xp_dn9 = assign88950_e135368_d_n9;
        locals.var_xp_dn10 = assign88950_e135368_d_n10;
        locals.var_xp_dn13 = assign88950_e135368_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign88960_e135381, assign88960_e135381_d_n0, assign88960_e135381_d_n2, assign88960_e135381_d_n4, assign88960_e135381_d_n5, assign88960_e135381_d_n6, assign88960_e135381_d_n7, assign88960_e135381_d_n8, assign88960_e135381_d_n9, assign88960_e135381_d_n10, assign88960_e135381_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2059 != 0.0)) && (locals.var_guard2060 != 0.0)) && (locals.var_guard2061 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign88960_e135381;
        locals.var_xmp_dn0 = assign88960_e135381_d_n0;
        locals.var_xmp_dn2 = assign88960_e135381_d_n2;
        locals.var_xmp_dn4 = assign88960_e135381_d_n4;
        locals.var_xmp_dn5 = assign88960_e135381_d_n5;
        locals.var_xmp_dn6 = assign88960_e135381_d_n6;
        locals.var_xmp_dn7 = assign88960_e135381_d_n7;
        locals.var_xmp_dn8 = assign88960_e135381_d_n8;
        locals.var_xmp_dn9 = assign88960_e135381_d_n9;
        locals.var_xmp_dn10 = assign88960_e135381_d_n10;
        locals.var_xmp_dn13 = assign88960_e135381_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign88970_e135394,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2059 != 0.0)) && (locals.var_guard2060 != 0.0)) && (locals.var_guard2061 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign88970_e135394;
        locals.var_m0_rv = 0.0;

        let (assign88980_e135407,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2059 != 0.0)) && (locals.var_guard2060 != 0.0)) && (locals.var_guard2061 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign88980_e135407;
        locals.var_mm_rv = 0.0;

        let (assign88990_e135420, assign88990_e135420_d_n0, assign88990_e135420_d_n2, assign88990_e135420_d_n4, assign88990_e135420_d_n5, assign88990_e135420_d_n6, assign88990_e135420_d_n7, assign88990_e135420_d_n8, assign88990_e135420_d_n9, assign88990_e135420_d_n10, assign88990_e135420_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2059 != 0.0)) && (locals.var_guard2060 != 0.0)) && (locals.var_guard2061 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign88990_e135420;
        locals.var_arg_dn0 = assign88990_e135420_d_n0;
        locals.var_arg_dn2 = assign88990_e135420_d_n2;
        locals.var_arg_dn4 = assign88990_e135420_d_n4;
        locals.var_arg_dn5 = assign88990_e135420_d_n5;
        locals.var_arg_dn6 = assign88990_e135420_d_n6;
        locals.var_arg_dn7 = assign88990_e135420_d_n7;
        locals.var_arg_dn8 = assign88990_e135420_d_n8;
        locals.var_arg_dn9 = assign88990_e135420_d_n9;
        locals.var_arg_dn10 = assign88990_e135420_d_n10;
        locals.var_arg_dn13 = assign88990_e135420_d_n13;
        locals.var_arg_rv = 0.0;

        let (assign89000_e135433, assign89000_e135433_d_n0, assign89000_e135433_d_n2, assign89000_e135433_d_n4, assign89000_e135433_d_n5, assign89000_e135433_d_n6, assign89000_e135433_d_n7, assign89000_e135433_d_n8, assign89000_e135433_d_n9, assign89000_e135433_d_n10, assign89000_e135433_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2059 != 0.0)) && (locals.var_guard2060 != 0.0)) && (locals.var_guard2061 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign89000_e135433;
        locals.var_dnm_dn0 = assign89000_e135433_d_n0;
        locals.var_dnm_dn2 = assign89000_e135433_d_n2;
        locals.var_dnm_dn4 = assign89000_e135433_d_n4;
        locals.var_dnm_dn5 = assign89000_e135433_d_n5;
        locals.var_dnm_dn6 = assign89000_e135433_d_n6;
        locals.var_dnm_dn7 = assign89000_e135433_d_n7;
        locals.var_dnm_dn8 = assign89000_e135433_d_n8;
        locals.var_dnm_dn9 = assign89000_e135433_d_n9;
        locals.var_dnm_dn10 = assign89000_e135433_d_n10;
        locals.var_dnm_dn13 = assign89000_e135433_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign89010_e135448, assign89010_e135448_d_n0, assign89010_e135448_d_n2, assign89010_e135448_d_n4, assign89010_e135448_d_n5, assign89010_e135448_d_n6, assign89010_e135448_d_n7, assign89010_e135448_d_n8, assign89010_e135448_d_n9, assign89010_e135448_d_n10, assign89010_e135448_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2059 != 0.0)) && (locals.var_guard2060 != 0.0)) && (locals.var_guard2061 != 0.0)) {
        let assign89010_e135446: f64 = (locals.var_xp * locals.var_x2);
        (assign89010_e135446, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign89010_e135448;
        locals.var_xp_dn0 = assign89010_e135448_d_n0;
        locals.var_xp_dn2 = assign89010_e135448_d_n2;
        locals.var_xp_dn4 = assign89010_e135448_d_n4;
        locals.var_xp_dn5 = assign89010_e135448_d_n5;
        locals.var_xp_dn6 = assign89010_e135448_d_n6;
        locals.var_xp_dn7 = assign89010_e135448_d_n7;
        locals.var_xp_dn8 = assign89010_e135448_d_n8;
        locals.var_xp_dn9 = assign89010_e135448_d_n9;
        locals.var_xp_dn10 = assign89010_e135448_d_n10;
        locals.var_xp_dn13 = assign89010_e135448_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign89020_e135463, assign89020_e135463_d_n0, assign89020_e135463_d_n2, assign89020_e135463_d_n4, assign89020_e135463_d_n5, assign89020_e135463_d_n6, assign89020_e135463_d_n7, assign89020_e135463_d_n8, assign89020_e135463_d_n9, assign89020_e135463_d_n10, assign89020_e135463_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2059 != 0.0)) && (locals.var_guard2060 != 0.0)) && (locals.var_guard2061 != 0.0)) {
        let assign89020_e135461: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign89020_e135461, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign89020_e135463;
        locals.var_xmp_dn0 = assign89020_e135463_d_n0;
        locals.var_xmp_dn2 = assign89020_e135463_d_n2;
        locals.var_xmp_dn4 = assign89020_e135463_d_n4;
        locals.var_xmp_dn5 = assign89020_e135463_d_n5;
        locals.var_xmp_dn6 = assign89020_e135463_d_n6;
        locals.var_xmp_dn7 = assign89020_e135463_d_n7;
        locals.var_xmp_dn8 = assign89020_e135463_d_n8;
        locals.var_xmp_dn9 = assign89020_e135463_d_n9;
        locals.var_xmp_dn10 = assign89020_e135463_d_n10;
        locals.var_xmp_dn13 = assign89020_e135463_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign89030_e135478, assign89030_e135478_d_n0, assign89030_e135478_d_n2, assign89030_e135478_d_n4, assign89030_e135478_d_n5, assign89030_e135478_d_n6, assign89030_e135478_d_n7, assign89030_e135478_d_n8, assign89030_e135478_d_n9, assign89030_e135478_d_n10, assign89030_e135478_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2059 != 0.0)) && (locals.var_guard2060 != 0.0)) && (locals.var_guard2061 != 0.0)) {
        let assign89030_e135476: f64 = (locals.var_xp * locals.var_x2);
        (assign89030_e135476, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign89030_e135478;
        locals.var_xp_dn0 = assign89030_e135478_d_n0;
        locals.var_xp_dn2 = assign89030_e135478_d_n2;
        locals.var_xp_dn4 = assign89030_e135478_d_n4;
        locals.var_xp_dn5 = assign89030_e135478_d_n5;
        locals.var_xp_dn6 = assign89030_e135478_d_n6;
        locals.var_xp_dn7 = assign89030_e135478_d_n7;
        locals.var_xp_dn8 = assign89030_e135478_d_n8;
        locals.var_xp_dn9 = assign89030_e135478_d_n9;
        locals.var_xp_dn10 = assign89030_e135478_d_n10;
        locals.var_xp_dn13 = assign89030_e135478_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign89040_e135493, assign89040_e135493_d_n0, assign89040_e135493_d_n2, assign89040_e135493_d_n4, assign89040_e135493_d_n5, assign89040_e135493_d_n6, assign89040_e135493_d_n7, assign89040_e135493_d_n8, assign89040_e135493_d_n9, assign89040_e135493_d_n10, assign89040_e135493_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2059 != 0.0)) && (locals.var_guard2060 != 0.0)) && (locals.var_guard2061 != 0.0)) {
        let assign89040_e135491: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign89040_e135491, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign89040_e135493;
        locals.var_xmp_dn0 = assign89040_e135493_d_n0;
        locals.var_xmp_dn2 = assign89040_e135493_d_n2;
        locals.var_xmp_dn4 = assign89040_e135493_d_n4;
        locals.var_xmp_dn5 = assign89040_e135493_d_n5;
        locals.var_xmp_dn6 = assign89040_e135493_d_n6;
        locals.var_xmp_dn7 = assign89040_e135493_d_n7;
        locals.var_xmp_dn8 = assign89040_e135493_d_n8;
        locals.var_xmp_dn9 = assign89040_e135493_d_n9;
        locals.var_xmp_dn10 = assign89040_e135493_d_n10;
        locals.var_xmp_dn13 = assign89040_e135493_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign89050_e135508, assign89050_e135508_d_n0, assign89050_e135508_d_n2, assign89050_e135508_d_n4, assign89050_e135508_d_n5, assign89050_e135508_d_n6, assign89050_e135508_d_n7, assign89050_e135508_d_n8, assign89050_e135508_d_n9, assign89050_e135508_d_n10, assign89050_e135508_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2059 != 0.0)) && (locals.var_guard2060 != 0.0)) && (locals.var_guard2061 != 0.0)) {
        let assign89050_e135506: f64 = (locals.var_xp + locals.var_xmp);
        (assign89050_e135506, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn13 + locals.var_xmp_dn13),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign89050_e135508;
        locals.var_arg_dn0 = assign89050_e135508_d_n0;
        locals.var_arg_dn2 = assign89050_e135508_d_n2;
        locals.var_arg_dn4 = assign89050_e135508_d_n4;
        locals.var_arg_dn5 = assign89050_e135508_d_n5;
        locals.var_arg_dn6 = assign89050_e135508_d_n6;
        locals.var_arg_dn7 = assign89050_e135508_d_n7;
        locals.var_arg_dn8 = assign89050_e135508_d_n8;
        locals.var_arg_dn9 = assign89050_e135508_d_n9;
        locals.var_arg_dn10 = assign89050_e135508_d_n10;
        locals.var_arg_dn13 = assign89050_e135508_d_n13;
        locals.var_arg_rv = 0.0;

        let (assign89060_e135521, assign89060_e135521_d_n0, assign89060_e135521_d_n2, assign89060_e135521_d_n4, assign89060_e135521_d_n5, assign89060_e135521_d_n6, assign89060_e135521_d_n7, assign89060_e135521_d_n8, assign89060_e135521_d_n9, assign89060_e135521_d_n10, assign89060_e135521_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2059 != 0.0)) && (locals.var_guard2060 != 0.0)) && (locals.var_guard2061 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign89060_e135521;
        locals.var_dnm_dn0 = assign89060_e135521_d_n0;
        locals.var_dnm_dn2 = assign89060_e135521_d_n2;
        locals.var_dnm_dn4 = assign89060_e135521_d_n4;
        locals.var_dnm_dn5 = assign89060_e135521_d_n5;
        locals.var_dnm_dn6 = assign89060_e135521_d_n6;
        locals.var_dnm_dn7 = assign89060_e135521_d_n7;
        locals.var_dnm_dn8 = assign89060_e135521_d_n8;
        locals.var_dnm_dn9 = assign89060_e135521_d_n9;
        locals.var_dnm_dn10 = assign89060_e135521_d_n10;
        locals.var_dnm_dn13 = assign89060_e135521_d_n13;
        locals.var_dnm_rv = 0.0;

        let assign89070_e135536: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard2062 = assign89070_e135536;
        locals.var_guard2062_rv = 0.0;

        let assign89080_e135539: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard2063 = assign89080_e135539;
        locals.var_guard2063_rv = 0.0;

        let (assign89090_e135556,) = {
    if (((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2059 != 0.0)) && (locals.var_guard2060 != 0.0)) && (locals.var_guard2061 != 0.0)) && (locals.var_guard2062 != 0.0)) && (locals.var_guard2063 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign89090_e135556;
        locals.var_mm_rv = 0.0;

        let assign89100_e135559: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard2064 = assign89100_e135559;
        locals.var_guard2064_rv = 0.0;

        let (assign89110_e135579,) = {
    if ((((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2059 != 0.0)) && (locals.var_guard2060 != 0.0)) && (locals.var_guard2061 != 0.0)) && (locals.var_guard2062 != 0.0)) && (locals.var_guard2063 == 0.0)) && (locals.var_guard2064 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign89110_e135579;
        locals.var_mm_rv = 0.0;

        let assign89120_e135582: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard2065 = assign89120_e135582;
        locals.var_guard2065_rv = 0.0;

        let (assign89130_e135605,) = {
    if (((((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2059 != 0.0)) && (locals.var_guard2060 != 0.0)) && (locals.var_guard2061 != 0.0)) && (locals.var_guard2062 != 0.0)) && (locals.var_guard2063 == 0.0)) && (locals.var_guard2064 == 0.0)) && (locals.var_guard2065 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign89130_e135605;
        locals.var_mm_rv = 0.0;

        let assign89140_e135608: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard2066 = assign89140_e135608;
        locals.var_guard2066_rv = 0.0;

        let (assign89150_e135634,) = {
    if ((((((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2059 != 0.0)) && (locals.var_guard2060 != 0.0)) && (locals.var_guard2061 != 0.0)) && (locals.var_guard2062 != 0.0)) && (locals.var_guard2063 == 0.0)) && (locals.var_guard2064 == 0.0)) && (locals.var_guard2065 == 0.0)) && (locals.var_guard2066 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign89150_e135634;
        locals.var_mm_rv = 0.0;

        let (assign89160_e135649,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2059 != 0.0)) && (locals.var_guard2060 != 0.0)) && (locals.var_guard2061 != 0.0)) && (locals.var_guard2062 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign89160_e135649;
        locals.var_m0_rv = 0.0;

        let mut assign89170_loop_guard: usize = 0;
        while {
            let assign89170_cond_e135665: f64 = if (((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2059 != 0.0)) && (locals.var_guard2060 != 0.0)) && (locals.var_guard2061 != 0.0)) && (locals.var_guard2062 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign89170_cond_e135665 != 0.0
        } {
            assign89170_loop_guard += 1;
            assert!(assign89170_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign89170_body0_e135681, assign89170_body0_e135681_d_n0, assign89170_body0_e135681_d_n2, assign89170_body0_e135681_d_n4, assign89170_body0_e135681_d_n5, assign89170_body0_e135681_d_n6, assign89170_body0_e135681_d_n7, assign89170_body0_e135681_d_n8, assign89170_body0_e135681_d_n9, assign89170_body0_e135681_d_n10, assign89170_body0_e135681_d_n13,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2059 != 0.0)) && (locals.var_guard2060 != 0.0)) && (locals.var_guard2061 != 0.0)) && (locals.var_guard2062 != 0.0)) {
        let assign89170_body0_e135679: f64 = (locals.var_dnm).sqrt();
        (assign89170_body0_e135679, (locals.var_dnm_dn0 / (2.0 * assign89170_body0_e135679)), (locals.var_dnm_dn2 / (2.0 * assign89170_body0_e135679)), (locals.var_dnm_dn4 / (2.0 * assign89170_body0_e135679)), (locals.var_dnm_dn5 / (2.0 * assign89170_body0_e135679)), (locals.var_dnm_dn6 / (2.0 * assign89170_body0_e135679)), (locals.var_dnm_dn7 / (2.0 * assign89170_body0_e135679)), (locals.var_dnm_dn8 / (2.0 * assign89170_body0_e135679)), (locals.var_dnm_dn9 / (2.0 * assign89170_body0_e135679)), (locals.var_dnm_dn10 / (2.0 * assign89170_body0_e135679)), (locals.var_dnm_dn13 / (2.0 * assign89170_body0_e135679)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
            locals.var_dnm = assign89170_body0_e135681;
            locals.var_dnm_dn0 = assign89170_body0_e135681_d_n0;
            locals.var_dnm_dn2 = assign89170_body0_e135681_d_n2;
            locals.var_dnm_dn4 = assign89170_body0_e135681_d_n4;
            locals.var_dnm_dn5 = assign89170_body0_e135681_d_n5;
            locals.var_dnm_dn6 = assign89170_body0_e135681_d_n6;
            locals.var_dnm_dn7 = assign89170_body0_e135681_d_n7;
            locals.var_dnm_dn8 = assign89170_body0_e135681_d_n8;
            locals.var_dnm_dn9 = assign89170_body0_e135681_d_n9;
            locals.var_dnm_dn10 = assign89170_body0_e135681_d_n10;
            locals.var_dnm_dn13 = assign89170_body0_e135681_d_n13;
            locals.var_dnm_rv = 0.0;
            let (assign89170_body1_e135698,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2059 != 0.0)) && (locals.var_guard2060 != 0.0)) && (locals.var_guard2061 != 0.0)) && (locals.var_guard2062 != 0.0)) {
        let assign89170_body1_e135696: f64 = (locals.var_m0 + 1.0);
        (assign89170_body1_e135696,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign89170_body1_e135698;
            locals.var_m0_rv = 0.0;
        }

        let (assign89180_e135725, assign89180_e135725_d_n0, assign89180_e135725_d_n2, assign89180_e135725_d_n4, assign89180_e135725_d_n5, assign89180_e135725_d_n6, assign89180_e135725_d_n7, assign89180_e135725_d_n8, assign89180_e135725_d_n9, assign89180_e135725_d_n10, assign89180_e135725_d_n13,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2059 != 0.0)) && (locals.var_guard2060 != 0.0)) && (locals.var_guard2061 != 0.0)) && (locals.var_guard2062 == 0.0)) {
        let (assign89180_e135723, assign89180_e135723_d_n0, assign89180_e135723_d_n2, assign89180_e135723_d_n4, assign89180_e135723_d_n5, assign89180_e135723_d_n6, assign89180_e135723_d_n7, assign89180_e135723_d_n8, assign89180_e135723_d_n9, assign89180_e135723_d_n10, assign89180_e135723_d_n13,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign89180_e135720: f64 = (2.0 * 2.0);
                let assign89180_e135721: f64 = (1.0 / assign89180_e135720);
                let assign89180_e135722: f64 = (locals.var_dnm).powf(assign89180_e135721);
                (assign89180_e135722, if 0.0 == 0.0 && ((assign89180_e135721) as f64).is_finite() && ((assign89180_e135721) as f64).fract() == 0.0 { if assign89180_e135721 == 0.0 { 0.0 } else { (assign89180_e135721 * ((locals.var_dnm).powf(assign89180_e135721 - 1.0) * locals.var_dnm_dn0)) } } else { (assign89180_e135722 * (assign89180_e135721 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign89180_e135721) as f64).is_finite() && ((assign89180_e135721) as f64).fract() == 0.0 { if assign89180_e135721 == 0.0 { 0.0 } else { (assign89180_e135721 * ((locals.var_dnm).powf(assign89180_e135721 - 1.0) * locals.var_dnm_dn2)) } } else { (assign89180_e135722 * (assign89180_e135721 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign89180_e135721) as f64).is_finite() && ((assign89180_e135721) as f64).fract() == 0.0 { if assign89180_e135721 == 0.0 { 0.0 } else { (assign89180_e135721 * ((locals.var_dnm).powf(assign89180_e135721 - 1.0) * locals.var_dnm_dn4)) } } else { (assign89180_e135722 * (assign89180_e135721 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign89180_e135721) as f64).is_finite() && ((assign89180_e135721) as f64).fract() == 0.0 { if assign89180_e135721 == 0.0 { 0.0 } else { (assign89180_e135721 * ((locals.var_dnm).powf(assign89180_e135721 - 1.0) * locals.var_dnm_dn5)) } } else { (assign89180_e135722 * (assign89180_e135721 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign89180_e135721) as f64).is_finite() && ((assign89180_e135721) as f64).fract() == 0.0 { if assign89180_e135721 == 0.0 { 0.0 } else { (assign89180_e135721 * ((locals.var_dnm).powf(assign89180_e135721 - 1.0) * locals.var_dnm_dn6)) } } else { (assign89180_e135722 * (assign89180_e135721 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign89180_e135721) as f64).is_finite() && ((assign89180_e135721) as f64).fract() == 0.0 { if assign89180_e135721 == 0.0 { 0.0 } else { (assign89180_e135721 * ((locals.var_dnm).powf(assign89180_e135721 - 1.0) * locals.var_dnm_dn7)) } } else { (assign89180_e135722 * (assign89180_e135721 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign89180_e135721) as f64).is_finite() && ((assign89180_e135721) as f64).fract() == 0.0 { if assign89180_e135721 == 0.0 { 0.0 } else { (assign89180_e135721 * ((locals.var_dnm).powf(assign89180_e135721 - 1.0) * locals.var_dnm_dn8)) } } else { (assign89180_e135722 * (assign89180_e135721 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign89180_e135721) as f64).is_finite() && ((assign89180_e135721) as f64).fract() == 0.0 { if assign89180_e135721 == 0.0 { 0.0 } else { (assign89180_e135721 * ((locals.var_dnm).powf(assign89180_e135721 - 1.0) * locals.var_dnm_dn9)) } } else { (assign89180_e135722 * (assign89180_e135721 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign89180_e135721) as f64).is_finite() && ((assign89180_e135721) as f64).fract() == 0.0 { if assign89180_e135721 == 0.0 { 0.0 } else { (assign89180_e135721 * ((locals.var_dnm).powf(assign89180_e135721 - 1.0) * locals.var_dnm_dn10)) } } else { (assign89180_e135722 * (assign89180_e135721 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign89180_e135721) as f64).is_finite() && ((assign89180_e135721) as f64).fract() == 0.0 { if assign89180_e135721 == 0.0 { 0.0 } else { (assign89180_e135721 * ((locals.var_dnm).powf(assign89180_e135721 - 1.0) * locals.var_dnm_dn13)) } } else { (assign89180_e135722 * (assign89180_e135721 * (locals.var_dnm_dn13 / locals.var_dnm))) },)
            }
        };
        (assign89180_e135723, assign89180_e135723_d_n0, assign89180_e135723_d_n2, assign89180_e135723_d_n4, assign89180_e135723_d_n5, assign89180_e135723_d_n6, assign89180_e135723_d_n7, assign89180_e135723_d_n8, assign89180_e135723_d_n9, assign89180_e135723_d_n10, assign89180_e135723_d_n13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign89180_e135725;
        locals.var_dnm_dn0 = assign89180_e135725_d_n0;
        locals.var_dnm_dn2 = assign89180_e135725_d_n2;
        locals.var_dnm_dn4 = assign89180_e135725_d_n4;
        locals.var_dnm_dn5 = assign89180_e135725_d_n5;
        locals.var_dnm_dn6 = assign89180_e135725_d_n6;
        locals.var_dnm_dn7 = assign89180_e135725_d_n7;
        locals.var_dnm_dn8 = assign89180_e135725_d_n8;
        locals.var_dnm_dn9 = assign89180_e135725_d_n9;
        locals.var_dnm_dn10 = assign89180_e135725_d_n10;
        locals.var_dnm_dn13 = assign89180_e135725_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign89190_e135740, assign89190_e135740_d_n0, assign89190_e135740_d_n2, assign89190_e135740_d_n4, assign89190_e135740_d_n5, assign89190_e135740_d_n6, assign89190_e135740_d_n7, assign89190_e135740_d_n8, assign89190_e135740_d_n9, assign89190_e135740_d_n10, assign89190_e135740_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2059 != 0.0)) && (locals.var_guard2060 != 0.0)) && (locals.var_guard2061 != 0.0)) {
        let assign89190_e135738: f64 = (1.0 / locals.var_dnm);
        (assign89190_e135738, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn13 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign89190_e135740;
        locals.var_dnm_dn0 = assign89190_e135740_d_n0;
        locals.var_dnm_dn2 = assign89190_e135740_d_n2;
        locals.var_dnm_dn4 = assign89190_e135740_d_n4;
        locals.var_dnm_dn5 = assign89190_e135740_d_n5;
        locals.var_dnm_dn6 = assign89190_e135740_d_n6;
        locals.var_dnm_dn7 = assign89190_e135740_d_n7;
        locals.var_dnm_dn8 = assign89190_e135740_d_n8;
        locals.var_dnm_dn9 = assign89190_e135740_d_n9;
        locals.var_dnm_dn10 = assign89190_e135740_d_n10;
        locals.var_dnm_dn13 = assign89190_e135740_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign89200_e135757, assign89200_e135757_d_n0, assign89200_e135757_d_n2, assign89200_e135757_d_n4, assign89200_e135757_d_n5, assign89200_e135757_d_n6, assign89200_e135757_d_n7, assign89200_e135757_d_n8, assign89200_e135757_d_n9, assign89200_e135757_d_n10, assign89200_e135757_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2059 != 0.0)) && (locals.var_guard2060 != 0.0)) && (locals.var_guard2061 != 0.0)) {
        let assign89200_e135753: f64 = (locals.var_tmf1 * 0.1);
        let assign89200_e135755: f64 = (assign89200_e135753 * locals.var_dnm);
        (assign89200_e135755, (((locals.var_tmf1_dn0 * 0.1) * locals.var_dnm) + (assign89200_e135753 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * 0.1) * locals.var_dnm) + (assign89200_e135753 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn4 * 0.1) * locals.var_dnm) + (assign89200_e135753 * locals.var_dnm_dn4)), (((locals.var_tmf1_dn5 * 0.1) * locals.var_dnm) + (assign89200_e135753 * locals.var_dnm_dn5)), (((locals.var_tmf1_dn6 * 0.1) * locals.var_dnm) + (assign89200_e135753 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * 0.1) * locals.var_dnm) + (assign89200_e135753 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn8 * 0.1) * locals.var_dnm) + (assign89200_e135753 * locals.var_dnm_dn8)), (((locals.var_tmf1_dn9 * 0.1) * locals.var_dnm) + (assign89200_e135753 * locals.var_dnm_dn9)), (((locals.var_tmf1_dn10 * 0.1) * locals.var_dnm) + (assign89200_e135753 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn13 * 0.1) * locals.var_dnm) + (assign89200_e135753 * locals.var_dnm_dn13)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn13,)
    }
};
        locals.var_tmf0 = assign89200_e135757;
        locals.var_tmf0_dn0 = assign89200_e135757_d_n0;
        locals.var_tmf0_dn2 = assign89200_e135757_d_n2;
        locals.var_tmf0_dn4 = assign89200_e135757_d_n4;
        locals.var_tmf0_dn5 = assign89200_e135757_d_n5;
        locals.var_tmf0_dn6 = assign89200_e135757_d_n6;
        locals.var_tmf0_dn7 = assign89200_e135757_d_n7;
        locals.var_tmf0_dn8 = assign89200_e135757_d_n8;
        locals.var_tmf0_dn9 = assign89200_e135757_d_n9;
        locals.var_tmf0_dn10 = assign89200_e135757_d_n10;
        locals.var_tmf0_dn13 = assign89200_e135757_d_n13;
        locals.var_tmf0_rv = 0.0;

        let (assign89210_e135776, assign89210_e135776_d_n0, assign89210_e135776_d_n2, assign89210_e135776_d_n4, assign89210_e135776_d_n5, assign89210_e135776_d_n6, assign89210_e135776_d_n7, assign89210_e135776_d_n8, assign89210_e135776_d_n9, assign89210_e135776_d_n10, assign89210_e135776_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2059 != 0.0)) && (locals.var_guard2060 != 0.0)) && (locals.var_guard2061 != 0.0)) {
        let assign89210_e135770: f64 = (0.1 * locals.var_xmp);
        let assign89210_e135772: f64 = (assign89210_e135770 * locals.var_dnm);
        let assign89210_e135774: f64 = (assign89210_e135772 / locals.var_arg);
        (assign89210_e135774, ((((((0.1 * locals.var_xmp_dn0) * locals.var_dnm) + (assign89210_e135770 * locals.var_dnm_dn0)) * locals.var_arg) - (assign89210_e135772 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn2) * locals.var_dnm) + (assign89210_e135770 * locals.var_dnm_dn2)) * locals.var_arg) - (assign89210_e135772 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn4) * locals.var_dnm) + (assign89210_e135770 * locals.var_dnm_dn4)) * locals.var_arg) - (assign89210_e135772 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn5) * locals.var_dnm) + (assign89210_e135770 * locals.var_dnm_dn5)) * locals.var_arg) - (assign89210_e135772 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn6) * locals.var_dnm) + (assign89210_e135770 * locals.var_dnm_dn6)) * locals.var_arg) - (assign89210_e135772 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn7) * locals.var_dnm) + (assign89210_e135770 * locals.var_dnm_dn7)) * locals.var_arg) - (assign89210_e135772 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn8) * locals.var_dnm) + (assign89210_e135770 * locals.var_dnm_dn8)) * locals.var_arg) - (assign89210_e135772 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn9) * locals.var_dnm) + (assign89210_e135770 * locals.var_dnm_dn9)) * locals.var_arg) - (assign89210_e135772 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn10) * locals.var_dnm) + (assign89210_e135770 * locals.var_dnm_dn10)) * locals.var_arg) - (assign89210_e135772 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn13) * locals.var_dnm) + (assign89210_e135770 * locals.var_dnm_dn13)) * locals.var_arg) - (assign89210_e135772 * locals.var_arg_dn13)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign89210_e135776;
        locals.var_t0_dn0 = assign89210_e135776_d_n0;
        locals.var_t0_dn2 = assign89210_e135776_d_n2;
        locals.var_t0_dn4 = assign89210_e135776_d_n4;
        locals.var_t0_dn5 = assign89210_e135776_d_n5;
        locals.var_t0_dn6 = assign89210_e135776_d_n6;
        locals.var_t0_dn7 = assign89210_e135776_d_n7;
        locals.var_t0_dn8 = assign89210_e135776_d_n8;
        locals.var_t0_dn9 = assign89210_e135776_d_n9;
        locals.var_t0_dn10 = assign89210_e135776_d_n10;
        locals.var_t0_dn13 = assign89210_e135776_d_n13;
        locals.var_t0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_330(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign89220_e135793, assign89220_e135793_d_n0, assign89220_e135793_d_n2, assign89220_e135793_d_n4, assign89220_e135793_d_n5, assign89220_e135793_d_n6, assign89220_e135793_d_n7, assign89220_e135793_d_n8, assign89220_e135793_d_n9, assign89220_e135793_d_n10, assign89220_e135793_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2059 != 0.0)) && (locals.var_guard2060 != 0.0)) && (locals.var_guard2061 != 0.0)) {
        let assign89220_e135789: f64 = (locals.var_ps0ld_bef1__blk2049 - 0.1);
        let assign89220_e135791: f64 = (assign89220_e135789 + locals.var_tmf0);
        (assign89220_e135791, (locals.var_ps0ld_bef1__blk2049_dn0 + locals.var_tmf0_dn0), (locals.var_ps0ld_bef1__blk2049_dn2 + locals.var_tmf0_dn2), (locals.var_ps0ld_bef1__blk2049_dn4 + locals.var_tmf0_dn4), (locals.var_ps0ld_bef1__blk2049_dn5 + locals.var_tmf0_dn5), (locals.var_ps0ld_bef1__blk2049_dn6 + locals.var_tmf0_dn6), (locals.var_ps0ld_bef1__blk2049_dn7 + locals.var_tmf0_dn7), (locals.var_ps0ld_bef1__blk2049_dn8 + locals.var_tmf0_dn8), (locals.var_ps0ld_bef1__blk2049_dn9 + locals.var_tmf0_dn9), (locals.var_ps0ld_bef1__blk2049_dn10 + locals.var_tmf0_dn10), (locals.var_ps0ld_bef1__blk2049_dn13 + locals.var_tmf0_dn13),)
    } else {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn13,)
    }
};
        locals.var_ps0ld = assign89220_e135793;
        locals.var_ps0ld_dn0 = assign89220_e135793_d_n0;
        locals.var_ps0ld_dn2 = assign89220_e135793_d_n2;
        locals.var_ps0ld_dn4 = assign89220_e135793_d_n4;
        locals.var_ps0ld_dn5 = assign89220_e135793_d_n5;
        locals.var_ps0ld_dn6 = assign89220_e135793_d_n6;
        locals.var_ps0ld_dn7 = assign89220_e135793_d_n7;
        locals.var_ps0ld_dn8 = assign89220_e135793_d_n8;
        locals.var_ps0ld_dn9 = assign89220_e135793_d_n9;
        locals.var_ps0ld_dn10 = assign89220_e135793_d_n10;
        locals.var_ps0ld_dn13 = assign89220_e135793_d_n13;
        locals.var_ps0ld_rv = 0.0;

        let (assign89230_e135806, assign89230_e135806_d_n0, assign89230_e135806_d_n2, assign89230_e135806_d_n4, assign89230_e135806_d_n5, assign89230_e135806_d_n6, assign89230_e135806_d_n7, assign89230_e135806_d_n8, assign89230_e135806_d_n9, assign89230_e135806_d_n10, assign89230_e135806_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2059 != 0.0)) && (locals.var_guard2060 != 0.0)) && (locals.var_guard2061 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign89230_e135806;
        locals.var_t0_dn0 = assign89230_e135806_d_n0;
        locals.var_t0_dn2 = assign89230_e135806_d_n2;
        locals.var_t0_dn4 = assign89230_e135806_d_n4;
        locals.var_t0_dn5 = assign89230_e135806_d_n5;
        locals.var_t0_dn6 = assign89230_e135806_d_n6;
        locals.var_t0_dn7 = assign89230_e135806_d_n7;
        locals.var_t0_dn8 = assign89230_e135806_d_n8;
        locals.var_t0_dn9 = assign89230_e135806_d_n9;
        locals.var_t0_dn10 = assign89230_e135806_d_n10;
        locals.var_t0_dn13 = assign89230_e135806_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign89240_e135820, assign89240_e135820_d_n0, assign89240_e135820_d_n2, assign89240_e135820_d_n4, assign89240_e135820_d_n5, assign89240_e135820_d_n6, assign89240_e135820_d_n7, assign89240_e135820_d_n8, assign89240_e135820_d_n9, assign89240_e135820_d_n10, assign89240_e135820_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2059 != 0.0)) && (locals.var_guard2060 != 0.0)) && (locals.var_guard2061 == 0.0)) {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn13,)
    } else {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn13,)
    }
};
        locals.var_ps0ld = assign89240_e135820;
        locals.var_ps0ld_dn0 = assign89240_e135820_d_n0;
        locals.var_ps0ld_dn2 = assign89240_e135820_d_n2;
        locals.var_ps0ld_dn4 = assign89240_e135820_d_n4;
        locals.var_ps0ld_dn5 = assign89240_e135820_d_n5;
        locals.var_ps0ld_dn6 = assign89240_e135820_d_n6;
        locals.var_ps0ld_dn7 = assign89240_e135820_d_n7;
        locals.var_ps0ld_dn8 = assign89240_e135820_d_n8;
        locals.var_ps0ld_dn9 = assign89240_e135820_d_n9;
        locals.var_ps0ld_dn10 = assign89240_e135820_d_n10;
        locals.var_ps0ld_dn13 = assign89240_e135820_d_n13;
        locals.var_ps0ld_rv = 0.0;

        let (assign89250_e135834, assign89250_e135834_d_n0, assign89250_e135834_d_n2, assign89250_e135834_d_n4, assign89250_e135834_d_n5, assign89250_e135834_d_n6, assign89250_e135834_d_n7, assign89250_e135834_d_n8, assign89250_e135834_d_n9, assign89250_e135834_d_n10, assign89250_e135834_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2059 != 0.0)) && (locals.var_guard2060 != 0.0)) && (locals.var_guard2061 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign89250_e135834;
        locals.var_t0_dn0 = assign89250_e135834_d_n0;
        locals.var_t0_dn2 = assign89250_e135834_d_n2;
        locals.var_t0_dn4 = assign89250_e135834_d_n4;
        locals.var_t0_dn5 = assign89250_e135834_d_n5;
        locals.var_t0_dn6 = assign89250_e135834_d_n6;
        locals.var_t0_dn7 = assign89250_e135834_d_n7;
        locals.var_t0_dn8 = assign89250_e135834_d_n8;
        locals.var_t0_dn9 = assign89250_e135834_d_n9;
        locals.var_t0_dn10 = assign89250_e135834_d_n10;
        locals.var_t0_dn13 = assign89250_e135834_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign89260_e135851, assign89260_e135851_d_n0, assign89260_e135851_d_n2, assign89260_e135851_d_n4, assign89260_e135851_d_n5, assign89260_e135851_d_n6, assign89260_e135851_d_n7, assign89260_e135851_d_n8, assign89260_e135851_d_n9, assign89260_e135851_d_n10, assign89260_e135851_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2059 != 0.0)) && (locals.var_guard2060 == 0.0)) {
        let (assign89260_e135849, assign89260_e135849_d_n0, assign89260_e135849_d_n2, assign89260_e135849_d_n4, assign89260_e135849_d_n5, assign89260_e135849_d_n6, assign89260_e135849_d_n7, assign89260_e135849_d_n8, assign89260_e135849_d_n9, assign89260_e135849_d_n10, assign89260_e135849_d_n13,) = {
            if (locals.var_ps0ld <= locals.var_ps0ld_bef1__blk2049) {
                (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn13,)
            } else {
                (locals.var_ps0ld_bef1__blk2049, locals.var_ps0ld_bef1__blk2049_dn0, locals.var_ps0ld_bef1__blk2049_dn2, locals.var_ps0ld_bef1__blk2049_dn4, locals.var_ps0ld_bef1__blk2049_dn5, locals.var_ps0ld_bef1__blk2049_dn6, locals.var_ps0ld_bef1__blk2049_dn7, locals.var_ps0ld_bef1__blk2049_dn8, locals.var_ps0ld_bef1__blk2049_dn9, locals.var_ps0ld_bef1__blk2049_dn10, locals.var_ps0ld_bef1__blk2049_dn13,)
            }
        };
        (assign89260_e135849, assign89260_e135849_d_n0, assign89260_e135849_d_n2, assign89260_e135849_d_n4, assign89260_e135849_d_n5, assign89260_e135849_d_n6, assign89260_e135849_d_n7, assign89260_e135849_d_n8, assign89260_e135849_d_n9, assign89260_e135849_d_n10, assign89260_e135849_d_n13,)
    } else {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn13,)
    }
};
        locals.var_ps0ld = assign89260_e135851;
        locals.var_ps0ld_dn0 = assign89260_e135851_d_n0;
        locals.var_ps0ld_dn2 = assign89260_e135851_d_n2;
        locals.var_ps0ld_dn4 = assign89260_e135851_d_n4;
        locals.var_ps0ld_dn5 = assign89260_e135851_d_n5;
        locals.var_ps0ld_dn6 = assign89260_e135851_d_n6;
        locals.var_ps0ld_dn7 = assign89260_e135851_d_n7;
        locals.var_ps0ld_dn8 = assign89260_e135851_d_n8;
        locals.var_ps0ld_dn9 = assign89260_e135851_d_n9;
        locals.var_ps0ld_dn10 = assign89260_e135851_d_n10;
        locals.var_ps0ld_dn13 = assign89260_e135851_d_n13;
        locals.var_ps0ld_rv = 0.0;

        let (assign89270_e135858, assign89270_e135858_d_n0, assign89270_e135858_d_n2, assign89270_e135858_d_n4, assign89270_e135858_d_n5, assign89270_e135858_d_n6, assign89270_e135858_d_n7, assign89270_e135858_d_n8, assign89270_e135858_d_n9, assign89270_e135858_d_n10, assign89270_e135858_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn13,)
    } else {
        (locals.var_ps0ld_ini__blk2011, locals.var_ps0ld_ini__blk2011_dn0, locals.var_ps0ld_ini__blk2011_dn2, locals.var_ps0ld_ini__blk2011_dn4, locals.var_ps0ld_ini__blk2011_dn5, locals.var_ps0ld_ini__blk2011_dn6, locals.var_ps0ld_ini__blk2011_dn7, locals.var_ps0ld_ini__blk2011_dn8, locals.var_ps0ld_ini__blk2011_dn9, locals.var_ps0ld_ini__blk2011_dn10, locals.var_ps0ld_ini__blk2011_dn13,)
    }
};
        locals.var_ps0ld_ini__blk2011 = assign89270_e135858;
        locals.var_ps0ld_ini__blk2011_dn0 = assign89270_e135858_d_n0;
        locals.var_ps0ld_ini__blk2011_dn2 = assign89270_e135858_d_n2;
        locals.var_ps0ld_ini__blk2011_dn4 = assign89270_e135858_d_n4;
        locals.var_ps0ld_ini__blk2011_dn5 = assign89270_e135858_d_n5;
        locals.var_ps0ld_ini__blk2011_dn6 = assign89270_e135858_d_n6;
        locals.var_ps0ld_ini__blk2011_dn7 = assign89270_e135858_d_n7;
        locals.var_ps0ld_ini__blk2011_dn8 = assign89270_e135858_d_n8;
        locals.var_ps0ld_ini__blk2011_dn9 = assign89270_e135858_d_n9;
        locals.var_ps0ld_ini__blk2011_dn10 = assign89270_e135858_d_n10;
        locals.var_ps0ld_ini__blk2011_dn13 = assign89270_e135858_d_n13;
        locals.var_ps0ld_ini__blk2011_rv = 0.0;

        let assign89280_e135861: f64 = if p.p33 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard2067 = assign89280_e135861;
        locals.var_guard2067_rv = 0.0;

        let (assign89290_e135870,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_flg_conv,)
    }
};
        locals.var_flg_conv = assign89290_e135870;
        locals.var_flg_conv_rv = 0.0;

        let (assign89300_e135886, assign89300_e135886_d_n0, assign89300_e135886_d_n2, assign89300_e135886_d_n4, assign89300_e135886_d_n5, assign89300_e135886_d_n6, assign89300_e135886_d_n7, assign89300_e135886_d_n8, assign89300_e135886_d_n9, assign89300_e135886_d_n10, assign89300_e135886_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) {
        let assign89300_e135880: f64 = (1.034943e-10 / locals.var_q_nsubld__blk2004);
        let assign89300_e135882: f64 = (assign89300_e135880 * locals.var_beta_inv);
        let assign89300_e135883: f64 = (2.0 * assign89300_e135882);
        let assign89300_e135884: f64 = (assign89300_e135883).sqrt();
        (assign89300_e135884, ((2.0 * (assign89300_e135880 * locals.var_beta_inv_dn0)) / (2.0 * assign89300_e135884)), ((2.0 * (assign89300_e135880 * locals.var_beta_inv_dn2)) / (2.0 * assign89300_e135884)), ((2.0 * (assign89300_e135880 * locals.var_beta_inv_dn4)) / (2.0 * assign89300_e135884)), ((2.0 * (assign89300_e135880 * locals.var_beta_inv_dn5)) / (2.0 * assign89300_e135884)), ((2.0 * (assign89300_e135880 * locals.var_beta_inv_dn6)) / (2.0 * assign89300_e135884)), ((2.0 * (assign89300_e135880 * locals.var_beta_inv_dn7)) / (2.0 * assign89300_e135884)), ((2.0 * (assign89300_e135880 * locals.var_beta_inv_dn8)) / (2.0 * assign89300_e135884)), ((2.0 * (assign89300_e135880 * locals.var_beta_inv_dn9)) / (2.0 * assign89300_e135884)), ((2.0 * (assign89300_e135880 * locals.var_beta_inv_dn10)) / (2.0 * assign89300_e135884)), ((2.0 * (assign89300_e135880 * locals.var_beta_inv_dn13)) / (2.0 * assign89300_e135884)),)
    } else {
        (locals.var_c_w_ld, locals.var_c_w_ld_dn0, locals.var_c_w_ld_dn2, locals.var_c_w_ld_dn4, locals.var_c_w_ld_dn5, locals.var_c_w_ld_dn6, locals.var_c_w_ld_dn7, locals.var_c_w_ld_dn8, locals.var_c_w_ld_dn9, locals.var_c_w_ld_dn10, locals.var_c_w_ld_dn13,)
    }
};
        locals.var_c_w_ld = assign89300_e135886;
        locals.var_c_w_ld_dn0 = assign89300_e135886_d_n0;
        locals.var_c_w_ld_dn2 = assign89300_e135886_d_n2;
        locals.var_c_w_ld_dn4 = assign89300_e135886_d_n4;
        locals.var_c_w_ld_dn5 = assign89300_e135886_d_n5;
        locals.var_c_w_ld_dn6 = assign89300_e135886_d_n6;
        locals.var_c_w_ld_dn7 = assign89300_e135886_d_n7;
        locals.var_c_w_ld_dn8 = assign89300_e135886_d_n8;
        locals.var_c_w_ld_dn9 = assign89300_e135886_d_n9;
        locals.var_c_w_ld_dn10 = assign89300_e135886_d_n10;
        locals.var_c_w_ld_dn13 = assign89300_e135886_d_n13;
        locals.var_c_w_ld_rv = 0.0;

        let assign89310_e135889: f64 = if locals.var_wdep_func > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2068 = assign89310_e135889;
        locals.var_guard2068_rv = 0.0;

        let (assign89320_e135902, assign89320_e135902_d_n0, assign89320_e135902_d_n2, assign89320_e135902_d_n4, assign89320_e135902_d_n5, assign89320_e135902_d_n6, assign89320_e135902_d_n7, assign89320_e135902_d_n8, assign89320_e135902_d_n9, assign89320_e135902_d_n10, assign89320_e135902_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) && (locals.var_guard2068 != 0.0)) {
        let assign89320_e135900: f64 = (p.p334 - locals.var_wdep_func);
        (assign89320_e135900, (-locals.var_wdep_func_dn0), (-locals.var_wdep_func_dn2), (-locals.var_wdep_func_dn4), (-locals.var_wdep_func_dn5), (-locals.var_wdep_func_dn6), (-locals.var_wdep_func_dn7), (-locals.var_wdep_func_dn8), (-locals.var_wdep_func_dn9), (-locals.var_wdep_func_dn10), (-locals.var_wdep_func_dn13),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign89320_e135902;
        locals.var_t2_dn0 = assign89320_e135902_d_n0;
        locals.var_t2_dn2 = assign89320_e135902_d_n2;
        locals.var_t2_dn4 = assign89320_e135902_d_n4;
        locals.var_t2_dn5 = assign89320_e135902_d_n5;
        locals.var_t2_dn6 = assign89320_e135902_d_n6;
        locals.var_t2_dn7 = assign89320_e135902_d_n7;
        locals.var_t2_dn8 = assign89320_e135902_d_n8;
        locals.var_t2_dn9 = assign89320_e135902_d_n9;
        locals.var_t2_dn10 = assign89320_e135902_d_n10;
        locals.var_t2_dn13 = assign89320_e135902_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign89330_e135927, assign89330_e135927_d_n0, assign89330_e135927_d_n2, assign89330_e135927_d_n4, assign89330_e135927_d_n5, assign89330_e135927_d_n6, assign89330_e135927_d_n7, assign89330_e135927_d_n8, assign89330_e135927_d_n9, assign89330_e135927_d_n10, assign89330_e135927_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) && (locals.var_guard2068 == 0.0)) {
        let assign89330_e135914: f64 = (locals.var_vdsi + p.p137);
        let assign89330_e135917: f64 = (locals.var_vdsi + p.p137);
        let assign89330_e135918: f64 = (assign89330_e135914 * assign89330_e135917);
        let assign89330_e135921: f64 = (4.0 * 0.1);
        let assign89330_e135923: f64 = (assign89330_e135921 * 0.1);
        let assign89330_e135924: f64 = (assign89330_e135918 + assign89330_e135923);
        let assign89330_e135925: f64 = (assign89330_e135924).sqrt();
        (assign89330_e135925, 0.0, 0.0, 0.0, (((locals.var_vdsi_dn5 * assign89330_e135917) + (assign89330_e135914 * locals.var_vdsi_dn5)) / (2.0 * assign89330_e135925)), 0.0, (((locals.var_vdsi_dn7 * assign89330_e135917) + (assign89330_e135914 * locals.var_vdsi_dn7)) / (2.0 * assign89330_e135925)), 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign89330_e135927;
        locals.var_tmf2_dn0 = assign89330_e135927_d_n0;
        locals.var_tmf2_dn2 = assign89330_e135927_d_n2;
        locals.var_tmf2_dn4 = assign89330_e135927_d_n4;
        locals.var_tmf2_dn5 = assign89330_e135927_d_n5;
        locals.var_tmf2_dn6 = assign89330_e135927_d_n6;
        locals.var_tmf2_dn7 = assign89330_e135927_d_n7;
        locals.var_tmf2_dn8 = assign89330_e135927_d_n8;
        locals.var_tmf2_dn9 = assign89330_e135927_d_n9;
        locals.var_tmf2_dn10 = assign89330_e135927_d_n10;
        locals.var_tmf2_dn13 = assign89330_e135927_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign89340_e135947, assign89340_e135947_d_n0, assign89340_e135947_d_n2, assign89340_e135947_d_n4, assign89340_e135947_d_n5, assign89340_e135947_d_n6, assign89340_e135947_d_n7, assign89340_e135947_d_n8, assign89340_e135947_d_n9, assign89340_e135947_d_n10, assign89340_e135947_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) && (locals.var_guard2068 == 0.0)) {
        let assign89340_e135941: f64 = (locals.var_vdsi + p.p137);
        let assign89340_e135943: f64 = (assign89340_e135941 / locals.var_tmf2);
        let assign89340_e135944: f64 = (1.0 + assign89340_e135943);
        let assign89340_e135945: f64 = (0.5 * assign89340_e135944);
        (assign89340_e135945, (0.5 * (-((assign89340_e135941 * locals.var_tmf2_dn0) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign89340_e135941 * locals.var_tmf2_dn2) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign89340_e135941 * locals.var_tmf2_dn4) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (((locals.var_vdsi_dn5 * locals.var_tmf2) - (assign89340_e135941 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (-((assign89340_e135941 * locals.var_tmf2_dn6) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (((locals.var_vdsi_dn7 * locals.var_tmf2) - (assign89340_e135941 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (-((assign89340_e135941 * locals.var_tmf2_dn8) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign89340_e135941 * locals.var_tmf2_dn9) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign89340_e135941 * locals.var_tmf2_dn10) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign89340_e135941 * locals.var_tmf2_dn13) / (locals.var_tmf2 * locals.var_tmf2)))),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign89340_e135947;
        locals.var_t9_dn0 = assign89340_e135947_d_n0;
        locals.var_t9_dn2 = assign89340_e135947_d_n2;
        locals.var_t9_dn4 = assign89340_e135947_d_n4;
        locals.var_t9_dn5 = assign89340_e135947_d_n5;
        locals.var_t9_dn6 = assign89340_e135947_d_n6;
        locals.var_t9_dn7 = assign89340_e135947_d_n7;
        locals.var_t9_dn8 = assign89340_e135947_d_n8;
        locals.var_t9_dn9 = assign89340_e135947_d_n9;
        locals.var_t9_dn10 = assign89340_e135947_d_n10;
        locals.var_t9_dn13 = assign89340_e135947_d_n13;
        locals.var_t9_rv = 0.0;

        let (assign89350_e135965, assign89350_e135965_d_n0, assign89350_e135965_d_n2, assign89350_e135965_d_n4, assign89350_e135965_d_n5, assign89350_e135965_d_n6, assign89350_e135965_d_n7, assign89350_e135965_d_n8, assign89350_e135965_d_n9, assign89350_e135965_d_n10, assign89350_e135965_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) && (locals.var_guard2068 == 0.0)) {
        let assign89350_e135960: f64 = (locals.var_vdsi + p.p137);
        let assign89350_e135962: f64 = (assign89350_e135960 + locals.var_tmf2);
        let assign89350_e135963: f64 = (0.5 * assign89350_e135962);
        (assign89350_e135963, (0.5 * locals.var_tmf2_dn0), (0.5 * locals.var_tmf2_dn2), (0.5 * locals.var_tmf2_dn4), (0.5 * (locals.var_vdsi_dn5 + locals.var_tmf2_dn5)), (0.5 * locals.var_tmf2_dn6), (0.5 * (locals.var_vdsi_dn7 + locals.var_tmf2_dn7)), (0.5 * locals.var_tmf2_dn8), (0.5 * locals.var_tmf2_dn9), (0.5 * locals.var_tmf2_dn10), (0.5 * locals.var_tmf2_dn13),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign89350_e135965;
        locals.var_t2_dn0 = assign89350_e135965_d_n0;
        locals.var_t2_dn2 = assign89350_e135965_d_n2;
        locals.var_t2_dn4 = assign89350_e135965_d_n4;
        locals.var_t2_dn5 = assign89350_e135965_d_n5;
        locals.var_t2_dn6 = assign89350_e135965_d_n6;
        locals.var_t2_dn7 = assign89350_e135965_d_n7;
        locals.var_t2_dn8 = assign89350_e135965_d_n8;
        locals.var_t2_dn9 = assign89350_e135965_d_n9;
        locals.var_t2_dn10 = assign89350_e135965_d_n10;
        locals.var_t2_dn13 = assign89350_e135965_d_n13;
        locals.var_t2_rv = 0.0;

        let assign89360_e135968: f64 = if locals.var_t2 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2069 = assign89360_e135968;
        locals.var_guard2069_rv = 0.0;

        let (assign89370_e135982, assign89370_e135982_d_n0, assign89370_e135982_d_n2, assign89370_e135982_d_n4, assign89370_e135982_d_n5, assign89370_e135982_d_n6, assign89370_e135982_d_n7, assign89370_e135982_d_n8, assign89370_e135982_d_n9, assign89370_e135982_d_n10, assign89370_e135982_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) && (locals.var_guard2068 == 0.0)) && (locals.var_guard2069 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign89370_e135982;
        locals.var_t2_dn0 = assign89370_e135982_d_n0;
        locals.var_t2_dn2 = assign89370_e135982_d_n2;
        locals.var_t2_dn4 = assign89370_e135982_d_n4;
        locals.var_t2_dn5 = assign89370_e135982_d_n5;
        locals.var_t2_dn6 = assign89370_e135982_d_n6;
        locals.var_t2_dn7 = assign89370_e135982_d_n7;
        locals.var_t2_dn8 = assign89370_e135982_d_n8;
        locals.var_t2_dn9 = assign89370_e135982_d_n9;
        locals.var_t2_dn10 = assign89370_e135982_d_n10;
        locals.var_t2_dn13 = assign89370_e135982_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign89380_e135996, assign89380_e135996_d_n0, assign89380_e135996_d_n2, assign89380_e135996_d_n4, assign89380_e135996_d_n5, assign89380_e135996_d_n6, assign89380_e135996_d_n7, assign89380_e135996_d_n8, assign89380_e135996_d_n9, assign89380_e135996_d_n10, assign89380_e135996_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) && (locals.var_guard2068 == 0.0)) && (locals.var_guard2069 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign89380_e135996;
        locals.var_t9_dn0 = assign89380_e135996_d_n0;
        locals.var_t9_dn2 = assign89380_e135996_d_n2;
        locals.var_t9_dn4 = assign89380_e135996_d_n4;
        locals.var_t9_dn5 = assign89380_e135996_d_n5;
        locals.var_t9_dn6 = assign89380_e135996_d_n6;
        locals.var_t9_dn7 = assign89380_e135996_d_n7;
        locals.var_t9_dn8 = assign89380_e135996_d_n8;
        locals.var_t9_dn9 = assign89380_e135996_d_n9;
        locals.var_t9_dn10 = assign89380_e135996_d_n10;
        locals.var_t9_dn13 = assign89380_e135996_d_n13;
        locals.var_t9_rv = 0.0;

        let (assign89390_e136013, assign89390_e136013_d_n0, assign89390_e136013_d_n2, assign89390_e136013_d_n4, assign89390_e136013_d_n5, assign89390_e136013_d_n6, assign89390_e136013_d_n7, assign89390_e136013_d_n8, assign89390_e136013_d_n9, assign89390_e136013_d_n10, assign89390_e136013_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) && (locals.var_guard2068 == 0.0)) {
        let assign89390_e136008: f64 = (locals.var_kjunc * locals.var_t2);
        let assign89390_e136009: f64 = (assign89390_e136008).sqrt();
        let assign89390_e136011: f64 = (assign89390_e136009 * p.p432);
        (assign89390_e136011, ((((locals.var_kjunc_dn0 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn0)) / (2.0 * assign89390_e136009)) * p.p432), ((((locals.var_kjunc_dn2 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn2)) / (2.0 * assign89390_e136009)) * p.p432), ((((locals.var_kjunc_dn4 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn4)) / (2.0 * assign89390_e136009)) * p.p432), ((((locals.var_kjunc_dn5 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn5)) / (2.0 * assign89390_e136009)) * p.p432), ((((locals.var_kjunc_dn6 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn6)) / (2.0 * assign89390_e136009)) * p.p432), ((((locals.var_kjunc_dn7 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn7)) / (2.0 * assign89390_e136009)) * p.p432), ((((locals.var_kjunc_dn8 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn8)) / (2.0 * assign89390_e136009)) * p.p432), ((((locals.var_kjunc_dn9 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn9)) / (2.0 * assign89390_e136009)) * p.p432), ((((locals.var_kjunc_dn10 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn10)) / (2.0 * assign89390_e136009)) * p.p432), ((((locals.var_kjunc_dn13 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn13)) / (2.0 * assign89390_e136009)) * p.p432),)
    } else {
        (locals.var_wjunc0, locals.var_wjunc0_dn0, locals.var_wjunc0_dn2, locals.var_wjunc0_dn4, locals.var_wjunc0_dn5, locals.var_wjunc0_dn6, locals.var_wjunc0_dn7, locals.var_wjunc0_dn8, locals.var_wjunc0_dn9, locals.var_wjunc0_dn10, locals.var_wjunc0_dn13,)
    }
};
        locals.var_wjunc0 = assign89390_e136013;
        locals.var_wjunc0_dn0 = assign89390_e136013_d_n0;
        locals.var_wjunc0_dn2 = assign89390_e136013_d_n2;
        locals.var_wjunc0_dn4 = assign89390_e136013_d_n4;
        locals.var_wjunc0_dn5 = assign89390_e136013_d_n5;
        locals.var_wjunc0_dn6 = assign89390_e136013_d_n6;
        locals.var_wjunc0_dn7 = assign89390_e136013_d_n7;
        locals.var_wjunc0_dn8 = assign89390_e136013_d_n8;
        locals.var_wjunc0_dn9 = assign89390_e136013_d_n9;
        locals.var_wjunc0_dn10 = assign89390_e136013_d_n10;
        locals.var_wjunc0_dn13 = assign89390_e136013_d_n13;
        locals.var_wjunc0_rv = 0.0;

        let (assign89400_e136027, assign89400_e136027_d_n0, assign89400_e136027_d_n2, assign89400_e136027_d_n4, assign89400_e136027_d_n5, assign89400_e136027_d_n6, assign89400_e136027_d_n7, assign89400_e136027_d_n8, assign89400_e136027_d_n9, assign89400_e136027_d_n10, assign89400_e136027_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) && (locals.var_guard2068 == 0.0)) {
        let assign89400_e136025: f64 = (p.p334 - locals.var_wjunc0);
        (assign89400_e136025, (-locals.var_wjunc0_dn0), (-locals.var_wjunc0_dn2), (-locals.var_wjunc0_dn4), (-locals.var_wjunc0_dn5), (-locals.var_wjunc0_dn6), (-locals.var_wjunc0_dn7), (-locals.var_wjunc0_dn8), (-locals.var_wjunc0_dn9), (-locals.var_wjunc0_dn10), (-locals.var_wjunc0_dn13),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign89400_e136027;
        locals.var_t2_dn0 = assign89400_e136027_d_n0;
        locals.var_t2_dn2 = assign89400_e136027_d_n2;
        locals.var_t2_dn4 = assign89400_e136027_d_n4;
        locals.var_t2_dn5 = assign89400_e136027_d_n5;
        locals.var_t2_dn6 = assign89400_e136027_d_n6;
        locals.var_t2_dn7 = assign89400_e136027_d_n7;
        locals.var_t2_dn8 = assign89400_e136027_d_n8;
        locals.var_t2_dn9 = assign89400_e136027_d_n9;
        locals.var_t2_dn10 = assign89400_e136027_d_n10;
        locals.var_t2_dn13 = assign89400_e136027_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign89410_e136049, assign89410_e136049_d_n0, assign89410_e136049_d_n2, assign89410_e136049_d_n4, assign89410_e136049_d_n5, assign89410_e136049_d_n6, assign89410_e136049_d_n7, assign89410_e136049_d_n8, assign89410_e136049_d_n9, assign89410_e136049_d_n10, assign89410_e136049_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) {
        let assign89410_e136036: f64 = (locals.var_t2 * locals.var_t2);
        let assign89410_e136040: f64 = (p.p334 * 0.01);
        let assign89410_e136041: f64 = (4.0 * assign89410_e136040);
        let assign89410_e136044: f64 = (p.p334 * 0.01);
        let assign89410_e136045: f64 = (assign89410_e136041 * assign89410_e136044);
        let assign89410_e136046: f64 = (assign89410_e136036 + assign89410_e136045);
        let assign89410_e136047: f64 = (assign89410_e136046).sqrt();
        (assign89410_e136047, (((locals.var_t2_dn0 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn0)) / (2.0 * assign89410_e136047)), (((locals.var_t2_dn2 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn2)) / (2.0 * assign89410_e136047)), (((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4)) / (2.0 * assign89410_e136047)), (((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5)) / (2.0 * assign89410_e136047)), (((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6)) / (2.0 * assign89410_e136047)), (((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7)) / (2.0 * assign89410_e136047)), (((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8)) / (2.0 * assign89410_e136047)), (((locals.var_t2_dn9 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn9)) / (2.0 * assign89410_e136047)), (((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10)) / (2.0 * assign89410_e136047)), (((locals.var_t2_dn13 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn13)) / (2.0 * assign89410_e136047)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign89410_e136049;
        locals.var_tmf2_dn0 = assign89410_e136049_d_n0;
        locals.var_tmf2_dn2 = assign89410_e136049_d_n2;
        locals.var_tmf2_dn4 = assign89410_e136049_d_n4;
        locals.var_tmf2_dn5 = assign89410_e136049_d_n5;
        locals.var_tmf2_dn6 = assign89410_e136049_d_n6;
        locals.var_tmf2_dn7 = assign89410_e136049_d_n7;
        locals.var_tmf2_dn8 = assign89410_e136049_d_n8;
        locals.var_tmf2_dn9 = assign89410_e136049_d_n9;
        locals.var_tmf2_dn10 = assign89410_e136049_d_n10;
        locals.var_tmf2_dn13 = assign89410_e136049_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign89420_e136064, assign89420_e136064_d_n0, assign89420_e136064_d_n2, assign89420_e136064_d_n4, assign89420_e136064_d_n5, assign89420_e136064_d_n6, assign89420_e136064_d_n7, assign89420_e136064_d_n8, assign89420_e136064_d_n9, assign89420_e136064_d_n10, assign89420_e136064_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) {
        let assign89420_e136060: f64 = (locals.var_t2 / locals.var_tmf2);
        let assign89420_e136061: f64 = (1.0 + assign89420_e136060);
        let assign89420_e136062: f64 = (0.5 * assign89420_e136061);
        (assign89420_e136062, (0.5 * (((locals.var_t2_dn0 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn2 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn4 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn5 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn6 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn7 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn8 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn9 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn10 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn13 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign89420_e136064;
        locals.var_t9_dn0 = assign89420_e136064_d_n0;
        locals.var_t9_dn2 = assign89420_e136064_d_n2;
        locals.var_t9_dn4 = assign89420_e136064_d_n4;
        locals.var_t9_dn5 = assign89420_e136064_d_n5;
        locals.var_t9_dn6 = assign89420_e136064_d_n6;
        locals.var_t9_dn7 = assign89420_e136064_d_n7;
        locals.var_t9_dn8 = assign89420_e136064_d_n8;
        locals.var_t9_dn9 = assign89420_e136064_d_n9;
        locals.var_t9_dn10 = assign89420_e136064_d_n10;
        locals.var_t9_dn13 = assign89420_e136064_d_n13;
        locals.var_t9_rv = 0.0;

        let (assign89430_e136077, assign89430_e136077_d_n0, assign89430_e136077_d_n2, assign89430_e136077_d_n4, assign89430_e136077_d_n5, assign89430_e136077_d_n6, assign89430_e136077_d_n7, assign89430_e136077_d_n8, assign89430_e136077_d_n9, assign89430_e136077_d_n10, assign89430_e136077_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) {
        let assign89430_e136074: f64 = (locals.var_t2 + locals.var_tmf2);
        let assign89430_e136075: f64 = (0.5 * assign89430_e136074);
        (assign89430_e136075, (0.5 * (locals.var_t2_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_t2_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_t2_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_t2_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_t2_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_t2_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_t2_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_t2_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_t2_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_t2_dn13 + locals.var_tmf2_dn13)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign89430_e136077;
        locals.var_t2_dn0 = assign89430_e136077_d_n0;
        locals.var_t2_dn2 = assign89430_e136077_d_n2;
        locals.var_t2_dn4 = assign89430_e136077_d_n4;
        locals.var_t2_dn5 = assign89430_e136077_d_n5;
        locals.var_t2_dn6 = assign89430_e136077_d_n6;
        locals.var_t2_dn7 = assign89430_e136077_d_n7;
        locals.var_t2_dn8 = assign89430_e136077_d_n8;
        locals.var_t2_dn9 = assign89430_e136077_d_n9;
        locals.var_t2_dn10 = assign89430_e136077_d_n10;
        locals.var_t2_dn13 = assign89430_e136077_d_n13;
        locals.var_t2_rv = 0.0;

        let assign89440_e136080: f64 = if locals.var_t2 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2070 = assign89440_e136080;
        locals.var_guard2070_rv = 0.0;

        let (assign89450_e136091, assign89450_e136091_d_n0, assign89450_e136091_d_n2, assign89450_e136091_d_n4, assign89450_e136091_d_n5, assign89450_e136091_d_n6, assign89450_e136091_d_n7, assign89450_e136091_d_n8, assign89450_e136091_d_n9, assign89450_e136091_d_n10, assign89450_e136091_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) && (locals.var_guard2070 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign89450_e136091;
        locals.var_t2_dn0 = assign89450_e136091_d_n0;
        locals.var_t2_dn2 = assign89450_e136091_d_n2;
        locals.var_t2_dn4 = assign89450_e136091_d_n4;
        locals.var_t2_dn5 = assign89450_e136091_d_n5;
        locals.var_t2_dn6 = assign89450_e136091_d_n6;
        locals.var_t2_dn7 = assign89450_e136091_d_n7;
        locals.var_t2_dn8 = assign89450_e136091_d_n8;
        locals.var_t2_dn9 = assign89450_e136091_d_n9;
        locals.var_t2_dn10 = assign89450_e136091_d_n10;
        locals.var_t2_dn13 = assign89450_e136091_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign89460_e136102, assign89460_e136102_d_n0, assign89460_e136102_d_n2, assign89460_e136102_d_n4, assign89460_e136102_d_n5, assign89460_e136102_d_n6, assign89460_e136102_d_n7, assign89460_e136102_d_n8, assign89460_e136102_d_n9, assign89460_e136102_d_n10, assign89460_e136102_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) && (locals.var_guard2070 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign89460_e136102;
        locals.var_t9_dn0 = assign89460_e136102_d_n0;
        locals.var_t9_dn2 = assign89460_e136102_d_n2;
        locals.var_t9_dn4 = assign89460_e136102_d_n4;
        locals.var_t9_dn5 = assign89460_e136102_d_n5;
        locals.var_t9_dn6 = assign89460_e136102_d_n6;
        locals.var_t9_dn7 = assign89460_e136102_d_n7;
        locals.var_t9_dn8 = assign89460_e136102_d_n8;
        locals.var_t9_dn9 = assign89460_e136102_d_n9;
        locals.var_t9_dn10 = assign89460_e136102_d_n10;
        locals.var_t9_dn13 = assign89460_e136102_d_n13;
        locals.var_t9_rv = 0.0;

        let (assign89470_e136111, assign89470_e136111_d_n0, assign89470_e136111_d_n2, assign89470_e136111_d_n4, assign89470_e136111_d_n5, assign89470_e136111_d_n6, assign89470_e136111_d_n7, assign89470_e136111_d_n8, assign89470_e136111_d_n9, assign89470_e136111_d_n10, assign89470_e136111_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    } else {
        (locals.var_ddriftldc, locals.var_ddriftldc_dn0, locals.var_ddriftldc_dn2, locals.var_ddriftldc_dn4, locals.var_ddriftldc_dn5, locals.var_ddriftldc_dn6, locals.var_ddriftldc_dn7, locals.var_ddriftldc_dn8, locals.var_ddriftldc_dn9, locals.var_ddriftldc_dn10, locals.var_ddriftldc_dn13,)
    }
};
        locals.var_ddriftldc = assign89470_e136111;
        locals.var_ddriftldc_dn0 = assign89470_e136111_d_n0;
        locals.var_ddriftldc_dn2 = assign89470_e136111_d_n2;
        locals.var_ddriftldc_dn4 = assign89470_e136111_d_n4;
        locals.var_ddriftldc_dn5 = assign89470_e136111_d_n5;
        locals.var_ddriftldc_dn6 = assign89470_e136111_d_n6;
        locals.var_ddriftldc_dn7 = assign89470_e136111_d_n7;
        locals.var_ddriftldc_dn8 = assign89470_e136111_d_n8;
        locals.var_ddriftldc_dn9 = assign89470_e136111_d_n9;
        locals.var_ddriftldc_dn10 = assign89470_e136111_d_n10;
        locals.var_ddriftldc_dn13 = assign89470_e136111_d_n13;
        locals.var_ddriftldc_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_331(
        locals: &mut StampLocals,
    ) {
        let (assign89480_e136128, assign89480_e136128_d_n0, assign89480_e136128_d_n2, assign89480_e136128_d_n4, assign89480_e136128_d_n5, assign89480_e136128_d_n6, assign89480_e136128_d_n7, assign89480_e136128_d_n8, assign89480_e136128_d_n9, assign89480_e136128_d_n10, assign89480_e136128_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) {
        let assign89480_e136120: f64 = (locals.var_q_nsubld__blk2004 * locals.var_ddriftldc);
        let assign89480_e136122: f64 = (assign89480_e136120 * locals.var_ddriftldc);
        let assign89480_e136124: f64 = (assign89480_e136122 / 2.0);
        let assign89480_e136126: f64 = (assign89480_e136124 / 1.034943e-10);
        (assign89480_e136126, (((((locals.var_q_nsubld__blk2004 * locals.var_ddriftldc_dn0) * locals.var_ddriftldc) + (assign89480_e136120 * locals.var_ddriftldc_dn0)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2004 * locals.var_ddriftldc_dn2) * locals.var_ddriftldc) + (assign89480_e136120 * locals.var_ddriftldc_dn2)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2004 * locals.var_ddriftldc_dn4) * locals.var_ddriftldc) + (assign89480_e136120 * locals.var_ddriftldc_dn4)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2004 * locals.var_ddriftldc_dn5) * locals.var_ddriftldc) + (assign89480_e136120 * locals.var_ddriftldc_dn5)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2004 * locals.var_ddriftldc_dn6) * locals.var_ddriftldc) + (assign89480_e136120 * locals.var_ddriftldc_dn6)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2004 * locals.var_ddriftldc_dn7) * locals.var_ddriftldc) + (assign89480_e136120 * locals.var_ddriftldc_dn7)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2004 * locals.var_ddriftldc_dn8) * locals.var_ddriftldc) + (assign89480_e136120 * locals.var_ddriftldc_dn8)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2004 * locals.var_ddriftldc_dn9) * locals.var_ddriftldc) + (assign89480_e136120 * locals.var_ddriftldc_dn9)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2004 * locals.var_ddriftldc_dn10) * locals.var_ddriftldc) + (assign89480_e136120 * locals.var_ddriftldc_dn10)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2004 * locals.var_ddriftldc_dn13) * locals.var_ddriftldc) + (assign89480_e136120 * locals.var_ddriftldc_dn13)) / 2.0) / 1.034943e-10),)
    } else {
        (locals.var_dphi_sb, locals.var_dphi_sb_dn0, locals.var_dphi_sb_dn2, locals.var_dphi_sb_dn4, locals.var_dphi_sb_dn5, locals.var_dphi_sb_dn6, locals.var_dphi_sb_dn7, locals.var_dphi_sb_dn8, locals.var_dphi_sb_dn9, locals.var_dphi_sb_dn10, locals.var_dphi_sb_dn13,)
    }
};
        locals.var_dphi_sb = assign89480_e136128;
        locals.var_dphi_sb_dn0 = assign89480_e136128_d_n0;
        locals.var_dphi_sb_dn2 = assign89480_e136128_d_n2;
        locals.var_dphi_sb_dn4 = assign89480_e136128_d_n4;
        locals.var_dphi_sb_dn5 = assign89480_e136128_d_n5;
        locals.var_dphi_sb_dn6 = assign89480_e136128_d_n6;
        locals.var_dphi_sb_dn7 = assign89480_e136128_d_n7;
        locals.var_dphi_sb_dn8 = assign89480_e136128_d_n8;
        locals.var_dphi_sb_dn9 = assign89480_e136128_d_n9;
        locals.var_dphi_sb_dn10 = assign89480_e136128_d_n10;
        locals.var_dphi_sb_dn13 = assign89480_e136128_d_n13;
        locals.var_dphi_sb_rv = 0.0;

        let (assign89490_e136142, assign89490_e136142_d_n0, assign89490_e136142_d_n2, assign89490_e136142_d_n4, assign89490_e136142_d_n5, assign89490_e136142_d_n6, assign89490_e136142_d_n7, assign89490_e136142_d_n8, assign89490_e136142_d_n9, assign89490_e136142_d_n10, assign89490_e136142_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) {
        let assign89490_e136137: f64 = (2.0 * locals.var_beta);
        let assign89490_e136139: f64 = (assign89490_e136137 * locals.var_dphi_sb);
        let assign89490_e136140: f64 = (assign89490_e136139).sqrt();
        (assign89490_e136140, ((((2.0 * locals.var_beta_dn0) * locals.var_dphi_sb) + (assign89490_e136137 * locals.var_dphi_sb_dn0)) / (2.0 * assign89490_e136140)), ((((2.0 * locals.var_beta_dn2) * locals.var_dphi_sb) + (assign89490_e136137 * locals.var_dphi_sb_dn2)) / (2.0 * assign89490_e136140)), ((((2.0 * locals.var_beta_dn4) * locals.var_dphi_sb) + (assign89490_e136137 * locals.var_dphi_sb_dn4)) / (2.0 * assign89490_e136140)), ((((2.0 * locals.var_beta_dn5) * locals.var_dphi_sb) + (assign89490_e136137 * locals.var_dphi_sb_dn5)) / (2.0 * assign89490_e136140)), ((((2.0 * locals.var_beta_dn6) * locals.var_dphi_sb) + (assign89490_e136137 * locals.var_dphi_sb_dn6)) / (2.0 * assign89490_e136140)), ((((2.0 * locals.var_beta_dn7) * locals.var_dphi_sb) + (assign89490_e136137 * locals.var_dphi_sb_dn7)) / (2.0 * assign89490_e136140)), ((((2.0 * locals.var_beta_dn8) * locals.var_dphi_sb) + (assign89490_e136137 * locals.var_dphi_sb_dn8)) / (2.0 * assign89490_e136140)), ((((2.0 * locals.var_beta_dn9) * locals.var_dphi_sb) + (assign89490_e136137 * locals.var_dphi_sb_dn9)) / (2.0 * assign89490_e136140)), ((((2.0 * locals.var_beta_dn10) * locals.var_dphi_sb) + (assign89490_e136137 * locals.var_dphi_sb_dn10)) / (2.0 * assign89490_e136140)), ((((2.0 * locals.var_beta_dn13) * locals.var_dphi_sb) + (assign89490_e136137 * locals.var_dphi_sb_dn13)) / (2.0 * assign89490_e136140)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign89490_e136142;
        locals.var_t0_dn0 = assign89490_e136142_d_n0;
        locals.var_t0_dn2 = assign89490_e136142_d_n2;
        locals.var_t0_dn4 = assign89490_e136142_d_n4;
        locals.var_t0_dn5 = assign89490_e136142_d_n5;
        locals.var_t0_dn6 = assign89490_e136142_d_n6;
        locals.var_t0_dn7 = assign89490_e136142_d_n7;
        locals.var_t0_dn8 = assign89490_e136142_d_n8;
        locals.var_t0_dn9 = assign89490_e136142_d_n9;
        locals.var_t0_dn10 = assign89490_e136142_d_n10;
        locals.var_t0_dn13 = assign89490_e136142_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign89500_e136158, assign89500_e136158_d_n0, assign89500_e136158_d_n2, assign89500_e136158_d_n4, assign89500_e136158_d_n5, assign89500_e136158_d_n6, assign89500_e136158_d_n7, assign89500_e136158_d_n8, assign89500_e136158_d_n9, assign89500_e136158_d_n10, assign89500_e136158_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) {
        let assign89500_e136150: f64 = { let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign89500_e136152: f64 = (-locals.var_t0);
        let assign89500_e136153: f64 = { let limited_exp_arg = assign89500_e136152; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign89500_e136154: f64 = (assign89500_e136150 + assign89500_e136153);
        let assign89500_e136156: f64 = (assign89500_e136154 / 2.0);
        (assign89500_e136156, ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn0) + ({ let limited_exp_arg = assign89500_e136152; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn0))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn2) + ({ let limited_exp_arg = assign89500_e136152; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn2))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn4) + ({ let limited_exp_arg = assign89500_e136152; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn4))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn5) + ({ let limited_exp_arg = assign89500_e136152; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn5))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn6) + ({ let limited_exp_arg = assign89500_e136152; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn6))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn7) + ({ let limited_exp_arg = assign89500_e136152; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn7))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn8) + ({ let limited_exp_arg = assign89500_e136152; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn8))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn9) + ({ let limited_exp_arg = assign89500_e136152; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn9))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn10) + ({ let limited_exp_arg = assign89500_e136152; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn10))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn13) + ({ let limited_exp_arg = assign89500_e136152; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn13))) / 2.0),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign89500_e136158;
        locals.var_t1_dn0 = assign89500_e136158_d_n0;
        locals.var_t1_dn2 = assign89500_e136158_d_n2;
        locals.var_t1_dn4 = assign89500_e136158_d_n4;
        locals.var_t1_dn5 = assign89500_e136158_d_n5;
        locals.var_t1_dn6 = assign89500_e136158_d_n6;
        locals.var_t1_dn7 = assign89500_e136158_d_n7;
        locals.var_t1_dn8 = assign89500_e136158_d_n8;
        locals.var_t1_dn9 = assign89500_e136158_d_n9;
        locals.var_t1_dn10 = assign89500_e136158_d_n10;
        locals.var_t1_dn13 = assign89500_e136158_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign89510_e136170, assign89510_e136170_d_n0, assign89510_e136170_d_n2, assign89510_e136170_d_n4, assign89510_e136170_d_n5, assign89510_e136170_d_n6, assign89510_e136170_d_n7, assign89510_e136170_d_n8, assign89510_e136170_d_n9, assign89510_e136170_d_n10, assign89510_e136170_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) {
        let assign89510_e136166: f64 = (locals.var_t1).ln();
        let assign89510_e136168: f64 = (assign89510_e136166 / locals.var_dphi_sb);
        (assign89510_e136168, ((((locals.var_t1_dn0 / locals.var_t1) * locals.var_dphi_sb) - (assign89510_e136166 * locals.var_dphi_sb_dn0)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn2 / locals.var_t1) * locals.var_dphi_sb) - (assign89510_e136166 * locals.var_dphi_sb_dn2)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn4 / locals.var_t1) * locals.var_dphi_sb) - (assign89510_e136166 * locals.var_dphi_sb_dn4)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn5 / locals.var_t1) * locals.var_dphi_sb) - (assign89510_e136166 * locals.var_dphi_sb_dn5)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn6 / locals.var_t1) * locals.var_dphi_sb) - (assign89510_e136166 * locals.var_dphi_sb_dn6)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn7 / locals.var_t1) * locals.var_dphi_sb) - (assign89510_e136166 * locals.var_dphi_sb_dn7)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn8 / locals.var_t1) * locals.var_dphi_sb) - (assign89510_e136166 * locals.var_dphi_sb_dn8)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn9 / locals.var_t1) * locals.var_dphi_sb) - (assign89510_e136166 * locals.var_dphi_sb_dn9)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn10 / locals.var_t1) * locals.var_dphi_sb) - (assign89510_e136166 * locals.var_dphi_sb_dn10)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn13 / locals.var_t1) * locals.var_dphi_sb) - (assign89510_e136166 * locals.var_dphi_sb_dn13)) / (locals.var_dphi_sb * locals.var_dphi_sb)),)
    } else {
        (locals.var_c_sb, locals.var_c_sb_dn0, locals.var_c_sb_dn2, locals.var_c_sb_dn4, locals.var_c_sb_dn5, locals.var_c_sb_dn6, locals.var_c_sb_dn7, locals.var_c_sb_dn8, locals.var_c_sb_dn9, locals.var_c_sb_dn10, locals.var_c_sb_dn13,)
    }
};
        locals.var_c_sb = assign89510_e136170;
        locals.var_c_sb_dn0 = assign89510_e136170_d_n0;
        locals.var_c_sb_dn2 = assign89510_e136170_d_n2;
        locals.var_c_sb_dn4 = assign89510_e136170_d_n4;
        locals.var_c_sb_dn5 = assign89510_e136170_d_n5;
        locals.var_c_sb_dn6 = assign89510_e136170_d_n6;
        locals.var_c_sb_dn7 = assign89510_e136170_d_n7;
        locals.var_c_sb_dn8 = assign89510_e136170_d_n8;
        locals.var_c_sb_dn9 = assign89510_e136170_d_n9;
        locals.var_c_sb_dn10 = assign89510_e136170_d_n10;
        locals.var_c_sb_dn13 = assign89510_e136170_d_n13;
        locals.var_c_sb_rv = 0.0;

        let (assign89520_e136179,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_lp_s0,)
    }
};
        locals.var_lp_s0 = assign89520_e136179;
        locals.var_lp_s0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_332(
        locals: &mut StampLocals,
    ) {
        let mut assign89530_loop_guard: usize = 0;
        while {
            let assign89530_cond_e136189: f64 = (locals.var_lp_s0_max + 1.0);
            let assign89530_cond_e136191: f64 = if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) && (locals.var_lp_s0 <= assign89530_cond_e136189)) { 1.0 } else { 0.0 };
            assign89530_cond_e136191 != 0.0
        } {
            assign89530_loop_guard += 1;
            assert!(assign89530_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign89530_body3_e136227, assign89530_body3_e136227_d_n0, assign89530_body3_e136227_d_n2, assign89530_body3_e136227_d_n4, assign89530_body3_e136227_d_n5, assign89530_body3_e136227_d_n6, assign89530_body3_e136227_d_n7, assign89530_body3_e136227_d_n8, assign89530_body3_e136227_d_n9, assign89530_body3_e136227_d_n10, assign89530_body3_e136227_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) {
        let assign89530_body3_e136225: f64 = (locals.var_ps0ld + locals.var_vxbgmtcl);
        (assign89530_body3_e136225, (locals.var_ps0ld_dn0 + locals.var_vxbgmtcl_dn0), (locals.var_ps0ld_dn2 + locals.var_vxbgmtcl_dn2), (locals.var_ps0ld_dn4 + locals.var_vxbgmtcl_dn4), (locals.var_ps0ld_dn5 + locals.var_vxbgmtcl_dn5), (locals.var_ps0ld_dn6 + locals.var_vxbgmtcl_dn6), (locals.var_ps0ld_dn7 + locals.var_vxbgmtcl_dn7), (locals.var_ps0ld_dn8 + locals.var_vxbgmtcl_dn8), (locals.var_ps0ld_dn9 + locals.var_vxbgmtcl_dn9), (locals.var_ps0ld_dn10 + locals.var_vxbgmtcl_dn10), (locals.var_ps0ld_dn13 + locals.var_vxbgmtcl_dn13),)
    } else {
        (locals.var_ps0ld_vxb, locals.var_ps0ld_vxb_dn0, locals.var_ps0ld_vxb_dn2, locals.var_ps0ld_vxb_dn4, locals.var_ps0ld_vxb_dn5, locals.var_ps0ld_vxb_dn6, locals.var_ps0ld_vxb_dn7, locals.var_ps0ld_vxb_dn8, locals.var_ps0ld_vxb_dn9, locals.var_ps0ld_vxb_dn10, locals.var_ps0ld_vxb_dn13,)
    }
};
            locals.var_ps0ld_vxb = assign89530_body3_e136227;
            locals.var_ps0ld_vxb_dn0 = assign89530_body3_e136227_d_n0;
            locals.var_ps0ld_vxb_dn2 = assign89530_body3_e136227_d_n2;
            locals.var_ps0ld_vxb_dn4 = assign89530_body3_e136227_d_n4;
            locals.var_ps0ld_vxb_dn5 = assign89530_body3_e136227_d_n5;
            locals.var_ps0ld_vxb_dn6 = assign89530_body3_e136227_d_n6;
            locals.var_ps0ld_vxb_dn7 = assign89530_body3_e136227_d_n7;
            locals.var_ps0ld_vxb_dn8 = assign89530_body3_e136227_d_n8;
            locals.var_ps0ld_vxb_dn9 = assign89530_body3_e136227_d_n9;
            locals.var_ps0ld_vxb_dn10 = assign89530_body3_e136227_d_n10;
            locals.var_ps0ld_vxb_dn13 = assign89530_body3_e136227_d_n13;
            locals.var_ps0ld_vxb_rv = 0.0;
            let (assign89530_body4_e136238, assign89530_body4_e136238_d_n0, assign89530_body4_e136238_d_n2, assign89530_body4_e136238_d_n4, assign89530_body4_e136238_d_n5, assign89530_body4_e136238_d_n6, assign89530_body4_e136238_d_n7, assign89530_body4_e136238_d_n8, assign89530_body4_e136238_d_n9, assign89530_body4_e136238_d_n10, assign89530_body4_e136238_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) {
        let assign89530_body4_e136236: f64 = (locals.var_beta * locals.var_ps0ld_vxb);
        (assign89530_body4_e136236, ((locals.var_beta_dn0 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn0)), ((locals.var_beta_dn2 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn2)), ((locals.var_beta_dn4 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn4)), ((locals.var_beta_dn5 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn5)), ((locals.var_beta_dn6 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn6)), ((locals.var_beta_dn7 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn7)), ((locals.var_beta_dn8 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn8)), ((locals.var_beta_dn9 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn9)), ((locals.var_beta_dn10 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn10)), ((locals.var_beta_dn13 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn13)),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn13,)
    }
};
            locals.var_chi = assign89530_body4_e136238;
            locals.var_chi_dn0 = assign89530_body4_e136238_d_n0;
            locals.var_chi_dn2 = assign89530_body4_e136238_d_n2;
            locals.var_chi_dn4 = assign89530_body4_e136238_d_n4;
            locals.var_chi_dn5 = assign89530_body4_e136238_d_n5;
            locals.var_chi_dn6 = assign89530_body4_e136238_d_n6;
            locals.var_chi_dn7 = assign89530_body4_e136238_d_n7;
            locals.var_chi_dn8 = assign89530_body4_e136238_d_n8;
            locals.var_chi_dn9 = assign89530_body4_e136238_d_n9;
            locals.var_chi_dn10 = assign89530_body4_e136238_d_n10;
            locals.var_chi_dn13 = assign89530_body4_e136238_d_n13;
            locals.var_chi_rv = 0.0;
            let (assign89530_body5_e136251, assign89530_body5_e136251_d_n0, assign89530_body5_e136251_d_n2, assign89530_body5_e136251_d_n4, assign89530_body5_e136251_d_n5, assign89530_body5_e136251_d_n6, assign89530_body5_e136251_d_n7, assign89530_body5_e136251_d_n8, assign89530_body5_e136251_d_n9, assign89530_body5_e136251_d_n10, assign89530_body5_e136251_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) {
        let assign89530_body5_e136248: f64 = (locals.var_ps0ld_vxb - locals.var_dphi_sb);
        let assign89530_body5_e136249: f64 = (locals.var_c_sb * assign89530_body5_e136248);
        (assign89530_body5_e136249, ((locals.var_c_sb_dn0 * assign89530_body5_e136248) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn0 - locals.var_dphi_sb_dn0))), ((locals.var_c_sb_dn2 * assign89530_body5_e136248) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn2 - locals.var_dphi_sb_dn2))), ((locals.var_c_sb_dn4 * assign89530_body5_e136248) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn4 - locals.var_dphi_sb_dn4))), ((locals.var_c_sb_dn5 * assign89530_body5_e136248) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn5 - locals.var_dphi_sb_dn5))), ((locals.var_c_sb_dn6 * assign89530_body5_e136248) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn6 - locals.var_dphi_sb_dn6))), ((locals.var_c_sb_dn7 * assign89530_body5_e136248) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn7 - locals.var_dphi_sb_dn7))), ((locals.var_c_sb_dn8 * assign89530_body5_e136248) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn8 - locals.var_dphi_sb_dn8))), ((locals.var_c_sb_dn9 * assign89530_body5_e136248) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn9 - locals.var_dphi_sb_dn9))), ((locals.var_c_sb_dn10 * assign89530_body5_e136248) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn10 - locals.var_dphi_sb_dn10))), ((locals.var_c_sb_dn13 * assign89530_body5_e136248) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn13 - locals.var_dphi_sb_dn13))),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn4, locals.var_ty_dn5, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn8, locals.var_ty_dn9, locals.var_ty_dn10, locals.var_ty_dn13,)
    }
};
            locals.var_ty = assign89530_body5_e136251;
            locals.var_ty_dn0 = assign89530_body5_e136251_d_n0;
            locals.var_ty_dn2 = assign89530_body5_e136251_d_n2;
            locals.var_ty_dn4 = assign89530_body5_e136251_d_n4;
            locals.var_ty_dn5 = assign89530_body5_e136251_d_n5;
            locals.var_ty_dn6 = assign89530_body5_e136251_d_n6;
            locals.var_ty_dn7 = assign89530_body5_e136251_d_n7;
            locals.var_ty_dn8 = assign89530_body5_e136251_d_n8;
            locals.var_ty_dn9 = assign89530_body5_e136251_d_n9;
            locals.var_ty_dn10 = assign89530_body5_e136251_d_n10;
            locals.var_ty_dn13 = assign89530_body5_e136251_d_n13;
            locals.var_ty_rv = 0.0;
            let assign89530_body6_e136254: f64 = if locals.var_ty < 60.0 { 1.0 } else { 0.0 };
            locals.var_guard2072 = assign89530_body6_e136254;
            locals.var_guard2072_rv = 0.0;
            let (assign89530_body7_e136266, assign89530_body7_e136266_d_n0, assign89530_body7_e136266_d_n2, assign89530_body7_e136266_d_n4, assign89530_body7_e136266_d_n5, assign89530_body7_e136266_d_n6, assign89530_body7_e136266_d_n7, assign89530_body7_e136266_d_n8, assign89530_body7_e136266_d_n9, assign89530_body7_e136266_d_n10, assign89530_body7_e136266_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) && (locals.var_guard2072 != 0.0)) {
        let assign89530_body7_e136264: f64 = (locals.var_ty).exp();
        (assign89530_body7_e136264, (assign89530_body7_e136264 * locals.var_ty_dn0), (assign89530_body7_e136264 * locals.var_ty_dn2), (assign89530_body7_e136264 * locals.var_ty_dn4), (assign89530_body7_e136264 * locals.var_ty_dn5), (assign89530_body7_e136264 * locals.var_ty_dn6), (assign89530_body7_e136264 * locals.var_ty_dn7), (assign89530_body7_e136264 * locals.var_ty_dn8), (assign89530_body7_e136264 * locals.var_ty_dn9), (assign89530_body7_e136264 * locals.var_ty_dn10), (assign89530_body7_e136264 * locals.var_ty_dn13),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
            locals.var_t1 = assign89530_body7_e136266;
            locals.var_t1_dn0 = assign89530_body7_e136266_d_n0;
            locals.var_t1_dn2 = assign89530_body7_e136266_d_n2;
            locals.var_t1_dn4 = assign89530_body7_e136266_d_n4;
            locals.var_t1_dn5 = assign89530_body7_e136266_d_n5;
            locals.var_t1_dn6 = assign89530_body7_e136266_d_n6;
            locals.var_t1_dn7 = assign89530_body7_e136266_d_n7;
            locals.var_t1_dn8 = assign89530_body7_e136266_d_n8;
            locals.var_t1_dn9 = assign89530_body7_e136266_d_n9;
            locals.var_t1_dn10 = assign89530_body7_e136266_d_n10;
            locals.var_t1_dn13 = assign89530_body7_e136266_d_n13;
            locals.var_t1_rv = 0.0;
            let (assign89530_body8_e136281, assign89530_body8_e136281_d_n0, assign89530_body8_e136281_d_n2, assign89530_body8_e136281_d_n4, assign89530_body8_e136281_d_n5, assign89530_body8_e136281_d_n6, assign89530_body8_e136281_d_n7, assign89530_body8_e136281_d_n8, assign89530_body8_e136281_d_n9, assign89530_body8_e136281_d_n10, assign89530_body8_e136281_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) && (locals.var_guard2072 != 0.0)) {
        let assign89530_body8_e136276: f64 = (-locals.var_c_sb);
        let assign89530_body8_e136278: f64 = (assign89530_body8_e136276 * locals.var_dphi_sb);
        let assign89530_body8_e136279: f64 = (assign89530_body8_e136278).exp();
        (assign89530_body8_e136279, (assign89530_body8_e136279 * (((-locals.var_c_sb_dn0) * locals.var_dphi_sb) + (assign89530_body8_e136276 * locals.var_dphi_sb_dn0))), (assign89530_body8_e136279 * (((-locals.var_c_sb_dn2) * locals.var_dphi_sb) + (assign89530_body8_e136276 * locals.var_dphi_sb_dn2))), (assign89530_body8_e136279 * (((-locals.var_c_sb_dn4) * locals.var_dphi_sb) + (assign89530_body8_e136276 * locals.var_dphi_sb_dn4))), (assign89530_body8_e136279 * (((-locals.var_c_sb_dn5) * locals.var_dphi_sb) + (assign89530_body8_e136276 * locals.var_dphi_sb_dn5))), (assign89530_body8_e136279 * (((-locals.var_c_sb_dn6) * locals.var_dphi_sb) + (assign89530_body8_e136276 * locals.var_dphi_sb_dn6))), (assign89530_body8_e136279 * (((-locals.var_c_sb_dn7) * locals.var_dphi_sb) + (assign89530_body8_e136276 * locals.var_dphi_sb_dn7))), (assign89530_body8_e136279 * (((-locals.var_c_sb_dn8) * locals.var_dphi_sb) + (assign89530_body8_e136276 * locals.var_dphi_sb_dn8))), (assign89530_body8_e136279 * (((-locals.var_c_sb_dn9) * locals.var_dphi_sb) + (assign89530_body8_e136276 * locals.var_dphi_sb_dn9))), (assign89530_body8_e136279 * (((-locals.var_c_sb_dn10) * locals.var_dphi_sb) + (assign89530_body8_e136276 * locals.var_dphi_sb_dn10))), (assign89530_body8_e136279 * (((-locals.var_c_sb_dn13) * locals.var_dphi_sb) + (assign89530_body8_e136276 * locals.var_dphi_sb_dn13))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
            locals.var_t0 = assign89530_body8_e136281;
            locals.var_t0_dn0 = assign89530_body8_e136281_d_n0;
            locals.var_t0_dn2 = assign89530_body8_e136281_d_n2;
            locals.var_t0_dn4 = assign89530_body8_e136281_d_n4;
            locals.var_t0_dn5 = assign89530_body8_e136281_d_n5;
            locals.var_t0_dn6 = assign89530_body8_e136281_d_n6;
            locals.var_t0_dn7 = assign89530_body8_e136281_d_n7;
            locals.var_t0_dn8 = assign89530_body8_e136281_d_n8;
            locals.var_t0_dn9 = assign89530_body8_e136281_d_n9;
            locals.var_t0_dn10 = assign89530_body8_e136281_d_n10;
            locals.var_t0_dn13 = assign89530_body8_e136281_d_n13;
            locals.var_t0_rv = 0.0;
            let (assign89530_body9_e136294, assign89530_body9_e136294_d_n0, assign89530_body9_e136294_d_n2, assign89530_body9_e136294_d_n4, assign89530_body9_e136294_d_n5, assign89530_body9_e136294_d_n6, assign89530_body9_e136294_d_n7, assign89530_body9_e136294_d_n8, assign89530_body9_e136294_d_n9, assign89530_body9_e136294_d_n10, assign89530_body9_e136294_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) && (locals.var_guard2072 != 0.0)) {
        let assign89530_body9_e136292: f64 = (locals.var_t1 - locals.var_t0);
        (assign89530_body9_e136292, (locals.var_t1_dn0 - locals.var_t0_dn0), (locals.var_t1_dn2 - locals.var_t0_dn2), (locals.var_t1_dn4 - locals.var_t0_dn4), (locals.var_t1_dn5 - locals.var_t0_dn5), (locals.var_t1_dn6 - locals.var_t0_dn6), (locals.var_t1_dn7 - locals.var_t0_dn7), (locals.var_t1_dn8 - locals.var_t0_dn8), (locals.var_t1_dn9 - locals.var_t0_dn9), (locals.var_t1_dn10 - locals.var_t0_dn10), (locals.var_t1_dn13 - locals.var_t0_dn13),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
            locals.var_t2 = assign89530_body9_e136294;
            locals.var_t2_dn0 = assign89530_body9_e136294_d_n0;
            locals.var_t2_dn2 = assign89530_body9_e136294_d_n2;
            locals.var_t2_dn4 = assign89530_body9_e136294_d_n4;
            locals.var_t2_dn5 = assign89530_body9_e136294_d_n5;
            locals.var_t2_dn6 = assign89530_body9_e136294_d_n6;
            locals.var_t2_dn7 = assign89530_body9_e136294_d_n7;
            locals.var_t2_dn8 = assign89530_body9_e136294_d_n8;
            locals.var_t2_dn9 = assign89530_body9_e136294_d_n9;
            locals.var_t2_dn10 = assign89530_body9_e136294_d_n10;
            locals.var_t2_dn13 = assign89530_body9_e136294_d_n13;
            locals.var_t2_rv = 0.0;
            let (assign89530_body10_e136310, assign89530_body10_e136310_d_n0, assign89530_body10_e136310_d_n2, assign89530_body10_e136310_d_n4, assign89530_body10_e136310_d_n5, assign89530_body10_e136310_d_n6, assign89530_body10_e136310_d_n7, assign89530_body10_e136310_d_n8, assign89530_body10_e136310_d_n9, assign89530_body10_e136310_d_n10, assign89530_body10_e136310_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) && (locals.var_guard2072 != 0.0)) {
        let assign89530_body10_e136305: f64 = (1.0 + locals.var_t2);
        let assign89530_body10_e136306: f64 = (assign89530_body10_e136305).ln();
        let assign89530_body10_e136308: f64 = (assign89530_body10_e136306 / locals.var_c_sb);
        (assign89530_body10_e136308, ((((locals.var_t2_dn0 / assign89530_body10_e136305) * locals.var_c_sb) - (assign89530_body10_e136306 * locals.var_c_sb_dn0)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn2 / assign89530_body10_e136305) * locals.var_c_sb) - (assign89530_body10_e136306 * locals.var_c_sb_dn2)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn4 / assign89530_body10_e136305) * locals.var_c_sb) - (assign89530_body10_e136306 * locals.var_c_sb_dn4)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn5 / assign89530_body10_e136305) * locals.var_c_sb) - (assign89530_body10_e136306 * locals.var_c_sb_dn5)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn6 / assign89530_body10_e136305) * locals.var_c_sb) - (assign89530_body10_e136306 * locals.var_c_sb_dn6)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn7 / assign89530_body10_e136305) * locals.var_c_sb) - (assign89530_body10_e136306 * locals.var_c_sb_dn7)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn8 / assign89530_body10_e136305) * locals.var_c_sb) - (assign89530_body10_e136306 * locals.var_c_sb_dn8)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn9 / assign89530_body10_e136305) * locals.var_c_sb) - (assign89530_body10_e136306 * locals.var_c_sb_dn9)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn10 / assign89530_body10_e136305) * locals.var_c_sb) - (assign89530_body10_e136306 * locals.var_c_sb_dn10)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn13 / assign89530_body10_e136305) * locals.var_c_sb) - (assign89530_body10_e136306 * locals.var_c_sb_dn13)) / (locals.var_c_sb * locals.var_c_sb)),)
    } else {
        (locals.var_phi_b, locals.var_phi_b_dn0, locals.var_phi_b_dn2, locals.var_phi_b_dn4, locals.var_phi_b_dn5, locals.var_phi_b_dn6, locals.var_phi_b_dn7, locals.var_phi_b_dn8, locals.var_phi_b_dn9, locals.var_phi_b_dn10, locals.var_phi_b_dn13,)
    }
};
            locals.var_phi_b = assign89530_body10_e136310;
            locals.var_phi_b_dn0 = assign89530_body10_e136310_d_n0;
            locals.var_phi_b_dn2 = assign89530_body10_e136310_d_n2;
            locals.var_phi_b_dn4 = assign89530_body10_e136310_d_n4;
            locals.var_phi_b_dn5 = assign89530_body10_e136310_d_n5;
            locals.var_phi_b_dn6 = assign89530_body10_e136310_d_n6;
            locals.var_phi_b_dn7 = assign89530_body10_e136310_d_n7;
            locals.var_phi_b_dn8 = assign89530_body10_e136310_d_n8;
            locals.var_phi_b_dn9 = assign89530_body10_e136310_d_n9;
            locals.var_phi_b_dn10 = assign89530_body10_e136310_d_n10;
            locals.var_phi_b_dn13 = assign89530_body10_e136310_d_n13;
            locals.var_phi_b_rv = 0.0;
            let (assign89530_body11_e136325, assign89530_body11_e136325_d_n0, assign89530_body11_e136325_d_n2, assign89530_body11_e136325_d_n4, assign89530_body11_e136325_d_n5, assign89530_body11_e136325_d_n6, assign89530_body11_e136325_d_n7, assign89530_body11_e136325_d_n8, assign89530_body11_e136325_d_n9, assign89530_body11_e136325_d_n10, assign89530_body11_e136325_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) && (locals.var_guard2072 != 0.0)) {
        let assign89530_body11_e136322: f64 = (1.0 + locals.var_t2);
        let assign89530_body11_e136323: f64 = (locals.var_t1 / assign89530_body11_e136322);
        (assign89530_body11_e136323, (((locals.var_t1_dn0 * assign89530_body11_e136322) - (locals.var_t1 * locals.var_t2_dn0)) / (assign89530_body11_e136322 * assign89530_body11_e136322)), (((locals.var_t1_dn2 * assign89530_body11_e136322) - (locals.var_t1 * locals.var_t2_dn2)) / (assign89530_body11_e136322 * assign89530_body11_e136322)), (((locals.var_t1_dn4 * assign89530_body11_e136322) - (locals.var_t1 * locals.var_t2_dn4)) / (assign89530_body11_e136322 * assign89530_body11_e136322)), (((locals.var_t1_dn5 * assign89530_body11_e136322) - (locals.var_t1 * locals.var_t2_dn5)) / (assign89530_body11_e136322 * assign89530_body11_e136322)), (((locals.var_t1_dn6 * assign89530_body11_e136322) - (locals.var_t1 * locals.var_t2_dn6)) / (assign89530_body11_e136322 * assign89530_body11_e136322)), (((locals.var_t1_dn7 * assign89530_body11_e136322) - (locals.var_t1 * locals.var_t2_dn7)) / (assign89530_body11_e136322 * assign89530_body11_e136322)), (((locals.var_t1_dn8 * assign89530_body11_e136322) - (locals.var_t1 * locals.var_t2_dn8)) / (assign89530_body11_e136322 * assign89530_body11_e136322)), (((locals.var_t1_dn9 * assign89530_body11_e136322) - (locals.var_t1 * locals.var_t2_dn9)) / (assign89530_body11_e136322 * assign89530_body11_e136322)), (((locals.var_t1_dn10 * assign89530_body11_e136322) - (locals.var_t1 * locals.var_t2_dn10)) / (assign89530_body11_e136322 * assign89530_body11_e136322)), (((locals.var_t1_dn13 * assign89530_body11_e136322) - (locals.var_t1 * locals.var_t2_dn13)) / (assign89530_body11_e136322 * assign89530_body11_e136322)),)
    } else {
        (locals.var_phi_b_dpss, locals.var_phi_b_dpss_dn0, locals.var_phi_b_dpss_dn2, locals.var_phi_b_dpss_dn4, locals.var_phi_b_dpss_dn5, locals.var_phi_b_dpss_dn6, locals.var_phi_b_dpss_dn7, locals.var_phi_b_dpss_dn8, locals.var_phi_b_dpss_dn9, locals.var_phi_b_dpss_dn10, locals.var_phi_b_dpss_dn13,)
    }
};
            locals.var_phi_b_dpss = assign89530_body11_e136325;
            locals.var_phi_b_dpss_dn0 = assign89530_body11_e136325_d_n0;
            locals.var_phi_b_dpss_dn2 = assign89530_body11_e136325_d_n2;
            locals.var_phi_b_dpss_dn4 = assign89530_body11_e136325_d_n4;
            locals.var_phi_b_dpss_dn5 = assign89530_body11_e136325_d_n5;
            locals.var_phi_b_dpss_dn6 = assign89530_body11_e136325_d_n6;
            locals.var_phi_b_dpss_dn7 = assign89530_body11_e136325_d_n7;
            locals.var_phi_b_dpss_dn8 = assign89530_body11_e136325_d_n8;
            locals.var_phi_b_dpss_dn9 = assign89530_body11_e136325_d_n9;
            locals.var_phi_b_dpss_dn10 = assign89530_body11_e136325_d_n10;
            locals.var_phi_b_dpss_dn13 = assign89530_body11_e136325_d_n13;
            locals.var_phi_b_dpss_rv = 0.0;
            let (assign89530_body13_e136353, assign89530_body13_e136353_d_n0, assign89530_body13_e136353_d_n2, assign89530_body13_e136353_d_n4, assign89530_body13_e136353_d_n5, assign89530_body13_e136353_d_n6, assign89530_body13_e136353_d_n7, assign89530_body13_e136353_d_n8, assign89530_body13_e136353_d_n9, assign89530_body13_e136353_d_n10, assign89530_body13_e136353_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) && (locals.var_guard2072 == 0.0)) {
        let assign89530_body13_e136351: f64 = (locals.var_ps0ld_vxb - locals.var_dphi_sb);
        (assign89530_body13_e136351, (locals.var_ps0ld_vxb_dn0 - locals.var_dphi_sb_dn0), (locals.var_ps0ld_vxb_dn2 - locals.var_dphi_sb_dn2), (locals.var_ps0ld_vxb_dn4 - locals.var_dphi_sb_dn4), (locals.var_ps0ld_vxb_dn5 - locals.var_dphi_sb_dn5), (locals.var_ps0ld_vxb_dn6 - locals.var_dphi_sb_dn6), (locals.var_ps0ld_vxb_dn7 - locals.var_dphi_sb_dn7), (locals.var_ps0ld_vxb_dn8 - locals.var_dphi_sb_dn8), (locals.var_ps0ld_vxb_dn9 - locals.var_dphi_sb_dn9), (locals.var_ps0ld_vxb_dn10 - locals.var_dphi_sb_dn10), (locals.var_ps0ld_vxb_dn13 - locals.var_dphi_sb_dn13),)
    } else {
        (locals.var_phi_b, locals.var_phi_b_dn0, locals.var_phi_b_dn2, locals.var_phi_b_dn4, locals.var_phi_b_dn5, locals.var_phi_b_dn6, locals.var_phi_b_dn7, locals.var_phi_b_dn8, locals.var_phi_b_dn9, locals.var_phi_b_dn10, locals.var_phi_b_dn13,)
    }
};
            locals.var_phi_b = assign89530_body13_e136353;
            locals.var_phi_b_dn0 = assign89530_body13_e136353_d_n0;
            locals.var_phi_b_dn2 = assign89530_body13_e136353_d_n2;
            locals.var_phi_b_dn4 = assign89530_body13_e136353_d_n4;
            locals.var_phi_b_dn5 = assign89530_body13_e136353_d_n5;
            locals.var_phi_b_dn6 = assign89530_body13_e136353_d_n6;
            locals.var_phi_b_dn7 = assign89530_body13_e136353_d_n7;
            locals.var_phi_b_dn8 = assign89530_body13_e136353_d_n8;
            locals.var_phi_b_dn9 = assign89530_body13_e136353_d_n9;
            locals.var_phi_b_dn10 = assign89530_body13_e136353_d_n10;
            locals.var_phi_b_dn13 = assign89530_body13_e136353_d_n13;
            locals.var_phi_b_rv = 0.0;
            let (assign89530_body14_e136365, assign89530_body14_e136365_d_n0, assign89530_body14_e136365_d_n2, assign89530_body14_e136365_d_n4, assign89530_body14_e136365_d_n5, assign89530_body14_e136365_d_n6, assign89530_body14_e136365_d_n7, assign89530_body14_e136365_d_n8, assign89530_body14_e136365_d_n9, assign89530_body14_e136365_d_n10, assign89530_body14_e136365_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) && (locals.var_guard2072 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_phi_b_dpss, locals.var_phi_b_dpss_dn0, locals.var_phi_b_dpss_dn2, locals.var_phi_b_dpss_dn4, locals.var_phi_b_dpss_dn5, locals.var_phi_b_dpss_dn6, locals.var_phi_b_dpss_dn7, locals.var_phi_b_dpss_dn8, locals.var_phi_b_dpss_dn9, locals.var_phi_b_dpss_dn10, locals.var_phi_b_dpss_dn13,)
    }
};
            locals.var_phi_b_dpss = assign89530_body14_e136365;
            locals.var_phi_b_dpss_dn0 = assign89530_body14_e136365_d_n0;
            locals.var_phi_b_dpss_dn2 = assign89530_body14_e136365_d_n2;
            locals.var_phi_b_dpss_dn4 = assign89530_body14_e136365_d_n4;
            locals.var_phi_b_dpss_dn5 = assign89530_body14_e136365_d_n5;
            locals.var_phi_b_dpss_dn6 = assign89530_body14_e136365_d_n6;
            locals.var_phi_b_dpss_dn7 = assign89530_body14_e136365_d_n7;
            locals.var_phi_b_dpss_dn8 = assign89530_body14_e136365_d_n8;
            locals.var_phi_b_dpss_dn9 = assign89530_body14_e136365_d_n9;
            locals.var_phi_b_dpss_dn10 = assign89530_body14_e136365_d_n10;
            locals.var_phi_b_dpss_dn13 = assign89530_body14_e136365_d_n13;
            locals.var_phi_b_dpss_rv = 0.0;
            let (assign89530_body15_e136376, assign89530_body15_e136376_d_n0, assign89530_body15_e136376_d_n2, assign89530_body15_e136376_d_n4, assign89530_body15_e136376_d_n5, assign89530_body15_e136376_d_n6, assign89530_body15_e136376_d_n7, assign89530_body15_e136376_d_n8, assign89530_body15_e136376_d_n9, assign89530_body15_e136376_d_n10, assign89530_body15_e136376_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) {
        let assign89530_body15_e136374: f64 = (locals.var_beta * locals.var_phi_b);
        (assign89530_body15_e136374, ((locals.var_beta_dn0 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn0)), ((locals.var_beta_dn2 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn2)), ((locals.var_beta_dn4 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn4)), ((locals.var_beta_dn5 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn5)), ((locals.var_beta_dn6 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn6)), ((locals.var_beta_dn7 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn7)), ((locals.var_beta_dn8 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn8)), ((locals.var_beta_dn9 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn9)), ((locals.var_beta_dn10 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn10)), ((locals.var_beta_dn13 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn13)),)
    } else {
        (locals.var_chib, locals.var_chib_dn0, locals.var_chib_dn2, locals.var_chib_dn4, locals.var_chib_dn5, locals.var_chib_dn6, locals.var_chib_dn7, locals.var_chib_dn8, locals.var_chib_dn9, locals.var_chib_dn10, locals.var_chib_dn13,)
    }
};
            locals.var_chib = assign89530_body15_e136376;
            locals.var_chib_dn0 = assign89530_body15_e136376_d_n0;
            locals.var_chib_dn2 = assign89530_body15_e136376_d_n2;
            locals.var_chib_dn4 = assign89530_body15_e136376_d_n4;
            locals.var_chib_dn5 = assign89530_body15_e136376_d_n5;
            locals.var_chib_dn6 = assign89530_body15_e136376_d_n6;
            locals.var_chib_dn7 = assign89530_body15_e136376_d_n7;
            locals.var_chib_dn8 = assign89530_body15_e136376_d_n8;
            locals.var_chib_dn9 = assign89530_body15_e136376_d_n9;
            locals.var_chib_dn10 = assign89530_body15_e136376_d_n10;
            locals.var_chib_dn13 = assign89530_body15_e136376_d_n13;
            locals.var_chib_rv = 0.0;
            let assign89530_body16_e136379: f64 = if locals.var_chi < 0.0 { 1.0 } else { 0.0 };
            locals.var_guard2073 = assign89530_body16_e136379;
            locals.var_guard2073_rv = 0.0;
            let (assign89530_body18_e136404, assign89530_body18_e136404_d_n0, assign89530_body18_e136404_d_n2, assign89530_body18_e136404_d_n4, assign89530_body18_e136404_d_n5, assign89530_body18_e136404_d_n6, assign89530_body18_e136404_d_n7, assign89530_body18_e136404_d_n8, assign89530_body18_e136404_d_n9, assign89530_body18_e136404_d_n10, assign89530_body18_e136404_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) && (locals.var_guard2073 != 0.0)) {
        let assign89530_body18_e136402: f64 = (-0.7071067811865475);
        (assign89530_body18_e136402, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
            locals.var_t0 = assign89530_body18_e136404;
            locals.var_t0_dn0 = assign89530_body18_e136404_d_n0;
            locals.var_t0_dn2 = assign89530_body18_e136404_d_n2;
            locals.var_t0_dn4 = assign89530_body18_e136404_d_n4;
            locals.var_t0_dn5 = assign89530_body18_e136404_d_n5;
            locals.var_t0_dn6 = assign89530_body18_e136404_d_n6;
            locals.var_t0_dn7 = assign89530_body18_e136404_d_n7;
            locals.var_t0_dn8 = assign89530_body18_e136404_d_n8;
            locals.var_t0_dn9 = assign89530_body18_e136404_d_n9;
            locals.var_t0_dn10 = assign89530_body18_e136404_d_n10;
            locals.var_t0_dn13 = assign89530_body18_e136404_d_n13;
            locals.var_t0_rv = 0.0;
            let (assign89530_body19_e136417, assign89530_body19_e136417_d_n0, assign89530_body19_e136417_d_n2, assign89530_body19_e136417_d_n4, assign89530_body19_e136417_d_n5, assign89530_body19_e136417_d_n6, assign89530_body19_e136417_d_n7, assign89530_body19_e136417_d_n8, assign89530_body19_e136417_d_n9, assign89530_body19_e136417_d_n10, assign89530_body19_e136417_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) && (locals.var_guard2073 != 0.0)) {
        let assign89530_body19_e136415: f64 = (locals.var_chi * locals.var_t0);
        (assign89530_body19_e136415, ((locals.var_chi_dn0 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn0)), ((locals.var_chi_dn2 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn2)), ((locals.var_chi_dn4 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn4)), ((locals.var_chi_dn5 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn5)), ((locals.var_chi_dn6 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn6)), ((locals.var_chi_dn7 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn7)), ((locals.var_chi_dn8 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn8)), ((locals.var_chi_dn9 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn9)), ((locals.var_chi_dn10 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn10)), ((locals.var_chi_dn13 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn13)),)
    } else {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn4, locals.var_fb_dn5, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn8, locals.var_fb_dn9, locals.var_fb_dn10, locals.var_fb_dn13,)
    }
};
            locals.var_fb = assign89530_body19_e136417;
            locals.var_fb_dn0 = assign89530_body19_e136417_d_n0;
            locals.var_fb_dn2 = assign89530_body19_e136417_d_n2;
            locals.var_fb_dn4 = assign89530_body19_e136417_d_n4;
            locals.var_fb_dn5 = assign89530_body19_e136417_d_n5;
            locals.var_fb_dn6 = assign89530_body19_e136417_d_n6;
            locals.var_fb_dn7 = assign89530_body19_e136417_d_n7;
            locals.var_fb_dn8 = assign89530_body19_e136417_d_n8;
            locals.var_fb_dn9 = assign89530_body19_e136417_d_n9;
            locals.var_fb_dn10 = assign89530_body19_e136417_d_n10;
            locals.var_fb_dn13 = assign89530_body19_e136417_d_n13;
            locals.var_fb_rv = 0.0;
            let (assign89530_body20_e136430, assign89530_body20_e136430_d_n0, assign89530_body20_e136430_d_n2, assign89530_body20_e136430_d_n4, assign89530_body20_e136430_d_n5, assign89530_body20_e136430_d_n6, assign89530_body20_e136430_d_n7, assign89530_body20_e136430_d_n8, assign89530_body20_e136430_d_n9, assign89530_body20_e136430_d_n10, assign89530_body20_e136430_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) && (locals.var_guard2073 != 0.0)) {
        let assign89530_body20_e136428: f64 = (locals.var_beta * locals.var_t0);
        (assign89530_body20_e136428, ((locals.var_beta_dn0 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn0)), ((locals.var_beta_dn2 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn2)), ((locals.var_beta_dn4 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn4)), ((locals.var_beta_dn5 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn5)), ((locals.var_beta_dn6 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn6)), ((locals.var_beta_dn7 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn7)), ((locals.var_beta_dn8 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn8)), ((locals.var_beta_dn9 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn9)), ((locals.var_beta_dn10 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn10)), ((locals.var_beta_dn13 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn13)),)
    } else {
        (locals.var_fb_dpss, locals.var_fb_dpss_dn0, locals.var_fb_dpss_dn2, locals.var_fb_dpss_dn4, locals.var_fb_dpss_dn5, locals.var_fb_dpss_dn6, locals.var_fb_dpss_dn7, locals.var_fb_dpss_dn8, locals.var_fb_dpss_dn9, locals.var_fb_dpss_dn10, locals.var_fb_dpss_dn13,)
    }
};
            locals.var_fb_dpss = assign89530_body20_e136430;
            locals.var_fb_dpss_dn0 = assign89530_body20_e136430_d_n0;
            locals.var_fb_dpss_dn2 = assign89530_body20_e136430_d_n2;
            locals.var_fb_dpss_dn4 = assign89530_body20_e136430_d_n4;
            locals.var_fb_dpss_dn5 = assign89530_body20_e136430_d_n5;
            locals.var_fb_dpss_dn6 = assign89530_body20_e136430_d_n6;
            locals.var_fb_dpss_dn7 = assign89530_body20_e136430_d_n7;
            locals.var_fb_dpss_dn8 = assign89530_body20_e136430_d_n8;
            locals.var_fb_dpss_dn9 = assign89530_body20_e136430_d_n9;
            locals.var_fb_dpss_dn10 = assign89530_body20_e136430_d_n10;
            locals.var_fb_dpss_dn13 = assign89530_body20_e136430_d_n13;
            locals.var_fb_dpss_rv = 0.0;
            let assign89530_body21_e136433: f64 = if locals.var_chi < 1e-6 { 1.0 } else { 0.0 };
            locals.var_guard2074 = assign89530_body21_e136433;
            locals.var_guard2074_rv = 0.0;
            let (assign89530_body23_e136485, assign89530_body23_e136485_d_n0, assign89530_body23_e136485_d_n2, assign89530_body23_e136485_d_n4, assign89530_body23_e136485_d_n5, assign89530_body23_e136485_d_n6, assign89530_body23_e136485_d_n7, assign89530_body23_e136485_d_n8, assign89530_body23_e136485_d_n9, assign89530_body23_e136485_d_n10, assign89530_body23_e136485_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) && (locals.var_guard2073 == 0.0)) && (locals.var_guard2074 != 0.0)) {
        let assign89530_body23_e136463: f64 = (locals.var_chi * locals.var_chi);
        let assign89530_body23_e136465: f64 = (assign89530_body23_e136463 / 2.0);
        let assign89530_body23_e136469: f64 = (locals.var_chi / 3.0);
        let assign89530_body23_e136473: f64 = (locals.var_chi / 4.0);
        let assign89530_body23_e136477: f64 = (locals.var_chi / 5.0);
        let assign89530_body23_e136478: f64 = (1.0 - assign89530_body23_e136477);
        let assign89530_body23_e136479: f64 = (assign89530_body23_e136473 * assign89530_body23_e136478);
        let assign89530_body23_e136480: f64 = (1.0 - assign89530_body23_e136479);
        let assign89530_body23_e136481: f64 = (assign89530_body23_e136469 * assign89530_body23_e136480);
        let assign89530_body23_e136482: f64 = (1.0 - assign89530_body23_e136481);
        let assign89530_body23_e136483: f64 = (assign89530_body23_e136465 * assign89530_body23_e136482);
        (assign89530_body23_e136483, (((((locals.var_chi_dn0 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn0)) / 2.0) * assign89530_body23_e136482) + (assign89530_body23_e136465 * (-(((locals.var_chi_dn0 / 3.0) * assign89530_body23_e136480) + (assign89530_body23_e136469 * (-(((locals.var_chi_dn0 / 4.0) * assign89530_body23_e136478) + (assign89530_body23_e136473 * (-(locals.var_chi_dn0 / 5.0)))))))))), (((((locals.var_chi_dn2 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn2)) / 2.0) * assign89530_body23_e136482) + (assign89530_body23_e136465 * (-(((locals.var_chi_dn2 / 3.0) * assign89530_body23_e136480) + (assign89530_body23_e136469 * (-(((locals.var_chi_dn2 / 4.0) * assign89530_body23_e136478) + (assign89530_body23_e136473 * (-(locals.var_chi_dn2 / 5.0)))))))))), (((((locals.var_chi_dn4 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn4)) / 2.0) * assign89530_body23_e136482) + (assign89530_body23_e136465 * (-(((locals.var_chi_dn4 / 3.0) * assign89530_body23_e136480) + (assign89530_body23_e136469 * (-(((locals.var_chi_dn4 / 4.0) * assign89530_body23_e136478) + (assign89530_body23_e136473 * (-(locals.var_chi_dn4 / 5.0)))))))))), (((((locals.var_chi_dn5 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn5)) / 2.0) * assign89530_body23_e136482) + (assign89530_body23_e136465 * (-(((locals.var_chi_dn5 / 3.0) * assign89530_body23_e136480) + (assign89530_body23_e136469 * (-(((locals.var_chi_dn5 / 4.0) * assign89530_body23_e136478) + (assign89530_body23_e136473 * (-(locals.var_chi_dn5 / 5.0)))))))))), (((((locals.var_chi_dn6 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn6)) / 2.0) * assign89530_body23_e136482) + (assign89530_body23_e136465 * (-(((locals.var_chi_dn6 / 3.0) * assign89530_body23_e136480) + (assign89530_body23_e136469 * (-(((locals.var_chi_dn6 / 4.0) * assign89530_body23_e136478) + (assign89530_body23_e136473 * (-(locals.var_chi_dn6 / 5.0)))))))))), (((((locals.var_chi_dn7 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn7)) / 2.0) * assign89530_body23_e136482) + (assign89530_body23_e136465 * (-(((locals.var_chi_dn7 / 3.0) * assign89530_body23_e136480) + (assign89530_body23_e136469 * (-(((locals.var_chi_dn7 / 4.0) * assign89530_body23_e136478) + (assign89530_body23_e136473 * (-(locals.var_chi_dn7 / 5.0)))))))))), (((((locals.var_chi_dn8 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn8)) / 2.0) * assign89530_body23_e136482) + (assign89530_body23_e136465 * (-(((locals.var_chi_dn8 / 3.0) * assign89530_body23_e136480) + (assign89530_body23_e136469 * (-(((locals.var_chi_dn8 / 4.0) * assign89530_body23_e136478) + (assign89530_body23_e136473 * (-(locals.var_chi_dn8 / 5.0)))))))))), (((((locals.var_chi_dn9 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn9)) / 2.0) * assign89530_body23_e136482) + (assign89530_body23_e136465 * (-(((locals.var_chi_dn9 / 3.0) * assign89530_body23_e136480) + (assign89530_body23_e136469 * (-(((locals.var_chi_dn9 / 4.0) * assign89530_body23_e136478) + (assign89530_body23_e136473 * (-(locals.var_chi_dn9 / 5.0)))))))))), (((((locals.var_chi_dn10 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn10)) / 2.0) * assign89530_body23_e136482) + (assign89530_body23_e136465 * (-(((locals.var_chi_dn10 / 3.0) * assign89530_body23_e136480) + (assign89530_body23_e136469 * (-(((locals.var_chi_dn10 / 4.0) * assign89530_body23_e136478) + (assign89530_body23_e136473 * (-(locals.var_chi_dn10 / 5.0)))))))))), (((((locals.var_chi_dn13 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn13)) / 2.0) * assign89530_body23_e136482) + (assign89530_body23_e136465 * (-(((locals.var_chi_dn13 / 3.0) * assign89530_body23_e136480) + (assign89530_body23_e136469 * (-(((locals.var_chi_dn13 / 4.0) * assign89530_body23_e136478) + (assign89530_body23_e136473 * (-(locals.var_chi_dn13 / 5.0)))))))))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
            locals.var_t0 = assign89530_body23_e136485;
            locals.var_t0_dn0 = assign89530_body23_e136485_d_n0;
            locals.var_t0_dn2 = assign89530_body23_e136485_d_n2;
            locals.var_t0_dn4 = assign89530_body23_e136485_d_n4;
            locals.var_t0_dn5 = assign89530_body23_e136485_d_n5;
            locals.var_t0_dn6 = assign89530_body23_e136485_d_n6;
            locals.var_t0_dn7 = assign89530_body23_e136485_d_n7;
            locals.var_t0_dn8 = assign89530_body23_e136485_d_n8;
            locals.var_t0_dn9 = assign89530_body23_e136485_d_n9;
            locals.var_t0_dn10 = assign89530_body23_e136485_d_n10;
            locals.var_t0_dn13 = assign89530_body23_e136485_d_n13;
            locals.var_t0_rv = 0.0;
            let (assign89530_body24_e136517, assign89530_body24_e136517_d_n0, assign89530_body24_e136517_d_n2, assign89530_body24_e136517_d_n4, assign89530_body24_e136517_d_n5, assign89530_body24_e136517_d_n6, assign89530_body24_e136517_d_n7, assign89530_body24_e136517_d_n8, assign89530_body24_e136517_d_n9, assign89530_body24_e136517_d_n10, assign89530_body24_e136517_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) && (locals.var_guard2073 == 0.0)) && (locals.var_guard2074 != 0.0)) {
        let assign89530_body24_e136501: f64 = (locals.var_chi / 2.0);
        let assign89530_body24_e136505: f64 = (locals.var_chi / 3.0);
        let assign89530_body24_e136509: f64 = (locals.var_chi / 4.0);
        let assign89530_body24_e136510: f64 = (1.0 - assign89530_body24_e136509);
        let assign89530_body24_e136511: f64 = (assign89530_body24_e136505 * assign89530_body24_e136510);
        let assign89530_body24_e136512: f64 = (1.0 - assign89530_body24_e136511);
        let assign89530_body24_e136513: f64 = (assign89530_body24_e136501 * assign89530_body24_e136512);
        let assign89530_body24_e136514: f64 = (1.0 - assign89530_body24_e136513);
        let assign89530_body24_e136515: f64 = (locals.var_chi * assign89530_body24_e136514);
        (assign89530_body24_e136515, ((locals.var_chi_dn0 * assign89530_body24_e136514) + (locals.var_chi * (-(((locals.var_chi_dn0 / 2.0) * assign89530_body24_e136512) + (assign89530_body24_e136501 * (-(((locals.var_chi_dn0 / 3.0) * assign89530_body24_e136510) + (assign89530_body24_e136505 * (-(locals.var_chi_dn0 / 4.0)))))))))), ((locals.var_chi_dn2 * assign89530_body24_e136514) + (locals.var_chi * (-(((locals.var_chi_dn2 / 2.0) * assign89530_body24_e136512) + (assign89530_body24_e136501 * (-(((locals.var_chi_dn2 / 3.0) * assign89530_body24_e136510) + (assign89530_body24_e136505 * (-(locals.var_chi_dn2 / 4.0)))))))))), ((locals.var_chi_dn4 * assign89530_body24_e136514) + (locals.var_chi * (-(((locals.var_chi_dn4 / 2.0) * assign89530_body24_e136512) + (assign89530_body24_e136501 * (-(((locals.var_chi_dn4 / 3.0) * assign89530_body24_e136510) + (assign89530_body24_e136505 * (-(locals.var_chi_dn4 / 4.0)))))))))), ((locals.var_chi_dn5 * assign89530_body24_e136514) + (locals.var_chi * (-(((locals.var_chi_dn5 / 2.0) * assign89530_body24_e136512) + (assign89530_body24_e136501 * (-(((locals.var_chi_dn5 / 3.0) * assign89530_body24_e136510) + (assign89530_body24_e136505 * (-(locals.var_chi_dn5 / 4.0)))))))))), ((locals.var_chi_dn6 * assign89530_body24_e136514) + (locals.var_chi * (-(((locals.var_chi_dn6 / 2.0) * assign89530_body24_e136512) + (assign89530_body24_e136501 * (-(((locals.var_chi_dn6 / 3.0) * assign89530_body24_e136510) + (assign89530_body24_e136505 * (-(locals.var_chi_dn6 / 4.0)))))))))), ((locals.var_chi_dn7 * assign89530_body24_e136514) + (locals.var_chi * (-(((locals.var_chi_dn7 / 2.0) * assign89530_body24_e136512) + (assign89530_body24_e136501 * (-(((locals.var_chi_dn7 / 3.0) * assign89530_body24_e136510) + (assign89530_body24_e136505 * (-(locals.var_chi_dn7 / 4.0)))))))))), ((locals.var_chi_dn8 * assign89530_body24_e136514) + (locals.var_chi * (-(((locals.var_chi_dn8 / 2.0) * assign89530_body24_e136512) + (assign89530_body24_e136501 * (-(((locals.var_chi_dn8 / 3.0) * assign89530_body24_e136510) + (assign89530_body24_e136505 * (-(locals.var_chi_dn8 / 4.0)))))))))), ((locals.var_chi_dn9 * assign89530_body24_e136514) + (locals.var_chi * (-(((locals.var_chi_dn9 / 2.0) * assign89530_body24_e136512) + (assign89530_body24_e136501 * (-(((locals.var_chi_dn9 / 3.0) * assign89530_body24_e136510) + (assign89530_body24_e136505 * (-(locals.var_chi_dn9 / 4.0)))))))))), ((locals.var_chi_dn10 * assign89530_body24_e136514) + (locals.var_chi * (-(((locals.var_chi_dn10 / 2.0) * assign89530_body24_e136512) + (assign89530_body24_e136501 * (-(((locals.var_chi_dn10 / 3.0) * assign89530_body24_e136510) + (assign89530_body24_e136505 * (-(locals.var_chi_dn10 / 4.0)))))))))), ((locals.var_chi_dn13 * assign89530_body24_e136514) + (locals.var_chi * (-(((locals.var_chi_dn13 / 2.0) * assign89530_body24_e136512) + (assign89530_body24_e136501 * (-(((locals.var_chi_dn13 / 3.0) * assign89530_body24_e136510) + (assign89530_body24_e136505 * (-(locals.var_chi_dn13 / 4.0)))))))))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
            locals.var_t1 = assign89530_body24_e136517;
            locals.var_t1_dn0 = assign89530_body24_e136517_d_n0;
            locals.var_t1_dn2 = assign89530_body24_e136517_d_n2;
            locals.var_t1_dn4 = assign89530_body24_e136517_d_n4;
            locals.var_t1_dn5 = assign89530_body24_e136517_d_n5;
            locals.var_t1_dn6 = assign89530_body24_e136517_d_n6;
            locals.var_t1_dn7 = assign89530_body24_e136517_d_n7;
            locals.var_t1_dn8 = assign89530_body24_e136517_d_n8;
            locals.var_t1_dn9 = assign89530_body24_e136517_d_n9;
            locals.var_t1_dn10 = assign89530_body24_e136517_d_n10;
            locals.var_t1_dn13 = assign89530_body24_e136517_d_n13;
            locals.var_t1_rv = 0.0;
            let (assign89530_body25_e136553, assign89530_body25_e136553_d_n0, assign89530_body25_e136553_d_n2, assign89530_body25_e136553_d_n4, assign89530_body25_e136553_d_n5, assign89530_body25_e136553_d_n6, assign89530_body25_e136553_d_n7, assign89530_body25_e136553_d_n8, assign89530_body25_e136553_d_n9, assign89530_body25_e136553_d_n10, assign89530_body25_e136553_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) && (locals.var_guard2073 == 0.0)) && (locals.var_guard2074 != 0.0)) {
        let assign89530_body25_e136531: f64 = (locals.var_chib * locals.var_chib);
        let assign89530_body25_e136533: f64 = (assign89530_body25_e136531 / 2.0);
        let assign89530_body25_e136537: f64 = (locals.var_chib / 3.0);
        let assign89530_body25_e136541: f64 = (locals.var_chib / 4.0);
        let assign89530_body25_e136545: f64 = (locals.var_chib / 5.0);
        let assign89530_body25_e136546: f64 = (1.0 - assign89530_body25_e136545);
        let assign89530_body25_e136547: f64 = (assign89530_body25_e136541 * assign89530_body25_e136546);
        let assign89530_body25_e136548: f64 = (1.0 - assign89530_body25_e136547);
        let assign89530_body25_e136549: f64 = (assign89530_body25_e136537 * assign89530_body25_e136548);
        let assign89530_body25_e136550: f64 = (1.0 - assign89530_body25_e136549);
        let assign89530_body25_e136551: f64 = (assign89530_body25_e136533 * assign89530_body25_e136550);
        (assign89530_body25_e136551, (((((locals.var_chib_dn0 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn0)) / 2.0) * assign89530_body25_e136550) + (assign89530_body25_e136533 * (-(((locals.var_chib_dn0 / 3.0) * assign89530_body25_e136548) + (assign89530_body25_e136537 * (-(((locals.var_chib_dn0 / 4.0) * assign89530_body25_e136546) + (assign89530_body25_e136541 * (-(locals.var_chib_dn0 / 5.0)))))))))), (((((locals.var_chib_dn2 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn2)) / 2.0) * assign89530_body25_e136550) + (assign89530_body25_e136533 * (-(((locals.var_chib_dn2 / 3.0) * assign89530_body25_e136548) + (assign89530_body25_e136537 * (-(((locals.var_chib_dn2 / 4.0) * assign89530_body25_e136546) + (assign89530_body25_e136541 * (-(locals.var_chib_dn2 / 5.0)))))))))), (((((locals.var_chib_dn4 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn4)) / 2.0) * assign89530_body25_e136550) + (assign89530_body25_e136533 * (-(((locals.var_chib_dn4 / 3.0) * assign89530_body25_e136548) + (assign89530_body25_e136537 * (-(((locals.var_chib_dn4 / 4.0) * assign89530_body25_e136546) + (assign89530_body25_e136541 * (-(locals.var_chib_dn4 / 5.0)))))))))), (((((locals.var_chib_dn5 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn5)) / 2.0) * assign89530_body25_e136550) + (assign89530_body25_e136533 * (-(((locals.var_chib_dn5 / 3.0) * assign89530_body25_e136548) + (assign89530_body25_e136537 * (-(((locals.var_chib_dn5 / 4.0) * assign89530_body25_e136546) + (assign89530_body25_e136541 * (-(locals.var_chib_dn5 / 5.0)))))))))), (((((locals.var_chib_dn6 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn6)) / 2.0) * assign89530_body25_e136550) + (assign89530_body25_e136533 * (-(((locals.var_chib_dn6 / 3.0) * assign89530_body25_e136548) + (assign89530_body25_e136537 * (-(((locals.var_chib_dn6 / 4.0) * assign89530_body25_e136546) + (assign89530_body25_e136541 * (-(locals.var_chib_dn6 / 5.0)))))))))), (((((locals.var_chib_dn7 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn7)) / 2.0) * assign89530_body25_e136550) + (assign89530_body25_e136533 * (-(((locals.var_chib_dn7 / 3.0) * assign89530_body25_e136548) + (assign89530_body25_e136537 * (-(((locals.var_chib_dn7 / 4.0) * assign89530_body25_e136546) + (assign89530_body25_e136541 * (-(locals.var_chib_dn7 / 5.0)))))))))), (((((locals.var_chib_dn8 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn8)) / 2.0) * assign89530_body25_e136550) + (assign89530_body25_e136533 * (-(((locals.var_chib_dn8 / 3.0) * assign89530_body25_e136548) + (assign89530_body25_e136537 * (-(((locals.var_chib_dn8 / 4.0) * assign89530_body25_e136546) + (assign89530_body25_e136541 * (-(locals.var_chib_dn8 / 5.0)))))))))), (((((locals.var_chib_dn9 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn9)) / 2.0) * assign89530_body25_e136550) + (assign89530_body25_e136533 * (-(((locals.var_chib_dn9 / 3.0) * assign89530_body25_e136548) + (assign89530_body25_e136537 * (-(((locals.var_chib_dn9 / 4.0) * assign89530_body25_e136546) + (assign89530_body25_e136541 * (-(locals.var_chib_dn9 / 5.0)))))))))), (((((locals.var_chib_dn10 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn10)) / 2.0) * assign89530_body25_e136550) + (assign89530_body25_e136533 * (-(((locals.var_chib_dn10 / 3.0) * assign89530_body25_e136548) + (assign89530_body25_e136537 * (-(((locals.var_chib_dn10 / 4.0) * assign89530_body25_e136546) + (assign89530_body25_e136541 * (-(locals.var_chib_dn10 / 5.0)))))))))), (((((locals.var_chib_dn13 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn13)) / 2.0) * assign89530_body25_e136550) + (assign89530_body25_e136533 * (-(((locals.var_chib_dn13 / 3.0) * assign89530_body25_e136548) + (assign89530_body25_e136537 * (-(((locals.var_chib_dn13 / 4.0) * assign89530_body25_e136546) + (assign89530_body25_e136541 * (-(locals.var_chib_dn13 / 5.0)))))))))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
            locals.var_t2 = assign89530_body25_e136553;
            locals.var_t2_dn0 = assign89530_body25_e136553_d_n0;
            locals.var_t2_dn2 = assign89530_body25_e136553_d_n2;
            locals.var_t2_dn4 = assign89530_body25_e136553_d_n4;
            locals.var_t2_dn5 = assign89530_body25_e136553_d_n5;
            locals.var_t2_dn6 = assign89530_body25_e136553_d_n6;
            locals.var_t2_dn7 = assign89530_body25_e136553_d_n7;
            locals.var_t2_dn8 = assign89530_body25_e136553_d_n8;
            locals.var_t2_dn9 = assign89530_body25_e136553_d_n9;
            locals.var_t2_dn10 = assign89530_body25_e136553_d_n10;
            locals.var_t2_dn13 = assign89530_body25_e136553_d_n13;
            locals.var_t2_rv = 0.0;
            let (assign89530_body26_e136585, assign89530_body26_e136585_d_n0, assign89530_body26_e136585_d_n2, assign89530_body26_e136585_d_n4, assign89530_body26_e136585_d_n5, assign89530_body26_e136585_d_n6, assign89530_body26_e136585_d_n7, assign89530_body26_e136585_d_n8, assign89530_body26_e136585_d_n9, assign89530_body26_e136585_d_n10, assign89530_body26_e136585_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) && (locals.var_guard2073 == 0.0)) && (locals.var_guard2074 != 0.0)) {
        let assign89530_body26_e136569: f64 = (locals.var_chib / 2.0);
        let assign89530_body26_e136573: f64 = (locals.var_chib / 3.0);
        let assign89530_body26_e136577: f64 = (locals.var_chib / 4.0);
        let assign89530_body26_e136578: f64 = (1.0 - assign89530_body26_e136577);
        let assign89530_body26_e136579: f64 = (assign89530_body26_e136573 * assign89530_body26_e136578);
        let assign89530_body26_e136580: f64 = (1.0 - assign89530_body26_e136579);
        let assign89530_body26_e136581: f64 = (assign89530_body26_e136569 * assign89530_body26_e136580);
        let assign89530_body26_e136582: f64 = (1.0 - assign89530_body26_e136581);
        let assign89530_body26_e136583: f64 = (locals.var_chib * assign89530_body26_e136582);
        (assign89530_body26_e136583, ((locals.var_chib_dn0 * assign89530_body26_e136582) + (locals.var_chib * (-(((locals.var_chib_dn0 / 2.0) * assign89530_body26_e136580) + (assign89530_body26_e136569 * (-(((locals.var_chib_dn0 / 3.0) * assign89530_body26_e136578) + (assign89530_body26_e136573 * (-(locals.var_chib_dn0 / 4.0)))))))))), ((locals.var_chib_dn2 * assign89530_body26_e136582) + (locals.var_chib * (-(((locals.var_chib_dn2 / 2.0) * assign89530_body26_e136580) + (assign89530_body26_e136569 * (-(((locals.var_chib_dn2 / 3.0) * assign89530_body26_e136578) + (assign89530_body26_e136573 * (-(locals.var_chib_dn2 / 4.0)))))))))), ((locals.var_chib_dn4 * assign89530_body26_e136582) + (locals.var_chib * (-(((locals.var_chib_dn4 / 2.0) * assign89530_body26_e136580) + (assign89530_body26_e136569 * (-(((locals.var_chib_dn4 / 3.0) * assign89530_body26_e136578) + (assign89530_body26_e136573 * (-(locals.var_chib_dn4 / 4.0)))))))))), ((locals.var_chib_dn5 * assign89530_body26_e136582) + (locals.var_chib * (-(((locals.var_chib_dn5 / 2.0) * assign89530_body26_e136580) + (assign89530_body26_e136569 * (-(((locals.var_chib_dn5 / 3.0) * assign89530_body26_e136578) + (assign89530_body26_e136573 * (-(locals.var_chib_dn5 / 4.0)))))))))), ((locals.var_chib_dn6 * assign89530_body26_e136582) + (locals.var_chib * (-(((locals.var_chib_dn6 / 2.0) * assign89530_body26_e136580) + (assign89530_body26_e136569 * (-(((locals.var_chib_dn6 / 3.0) * assign89530_body26_e136578) + (assign89530_body26_e136573 * (-(locals.var_chib_dn6 / 4.0)))))))))), ((locals.var_chib_dn7 * assign89530_body26_e136582) + (locals.var_chib * (-(((locals.var_chib_dn7 / 2.0) * assign89530_body26_e136580) + (assign89530_body26_e136569 * (-(((locals.var_chib_dn7 / 3.0) * assign89530_body26_e136578) + (assign89530_body26_e136573 * (-(locals.var_chib_dn7 / 4.0)))))))))), ((locals.var_chib_dn8 * assign89530_body26_e136582) + (locals.var_chib * (-(((locals.var_chib_dn8 / 2.0) * assign89530_body26_e136580) + (assign89530_body26_e136569 * (-(((locals.var_chib_dn8 / 3.0) * assign89530_body26_e136578) + (assign89530_body26_e136573 * (-(locals.var_chib_dn8 / 4.0)))))))))), ((locals.var_chib_dn9 * assign89530_body26_e136582) + (locals.var_chib * (-(((locals.var_chib_dn9 / 2.0) * assign89530_body26_e136580) + (assign89530_body26_e136569 * (-(((locals.var_chib_dn9 / 3.0) * assign89530_body26_e136578) + (assign89530_body26_e136573 * (-(locals.var_chib_dn9 / 4.0)))))))))), ((locals.var_chib_dn10 * assign89530_body26_e136582) + (locals.var_chib * (-(((locals.var_chib_dn10 / 2.0) * assign89530_body26_e136580) + (assign89530_body26_e136569 * (-(((locals.var_chib_dn10 / 3.0) * assign89530_body26_e136578) + (assign89530_body26_e136573 * (-(locals.var_chib_dn10 / 4.0)))))))))), ((locals.var_chib_dn13 * assign89530_body26_e136582) + (locals.var_chib * (-(((locals.var_chib_dn13 / 2.0) * assign89530_body26_e136580) + (assign89530_body26_e136569 * (-(((locals.var_chib_dn13 / 3.0) * assign89530_body26_e136578) + (assign89530_body26_e136573 * (-(locals.var_chib_dn13 / 4.0)))))))))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
            locals.var_t3 = assign89530_body26_e136585;
            locals.var_t3_dn0 = assign89530_body26_e136585_d_n0;
            locals.var_t3_dn2 = assign89530_body26_e136585_d_n2;
            locals.var_t3_dn4 = assign89530_body26_e136585_d_n4;
            locals.var_t3_dn5 = assign89530_body26_e136585_d_n5;
            locals.var_t3_dn6 = assign89530_body26_e136585_d_n6;
            locals.var_t3_dn7 = assign89530_body26_e136585_d_n7;
            locals.var_t3_dn8 = assign89530_body26_e136585_d_n8;
            locals.var_t3_dn9 = assign89530_body26_e136585_d_n9;
            locals.var_t3_dn10 = assign89530_body26_e136585_d_n10;
            locals.var_t3_dn13 = assign89530_body26_e136585_d_n13;
            locals.var_t3_rv = 0.0;
            let (assign89530_body27_e136601, assign89530_body27_e136601_d_n0, assign89530_body27_e136601_d_n2, assign89530_body27_e136601_d_n4, assign89530_body27_e136601_d_n5, assign89530_body27_e136601_d_n6, assign89530_body27_e136601_d_n7, assign89530_body27_e136601_d_n8, assign89530_body27_e136601_d_n9, assign89530_body27_e136601_d_n10, assign89530_body27_e136601_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) && (locals.var_guard2073 == 0.0)) && (locals.var_guard2074 != 0.0)) {
        let assign89530_body27_e136599: f64 = (locals.var_t0 - locals.var_t2);
        (assign89530_body27_e136599, (locals.var_t0_dn0 - locals.var_t2_dn0), (locals.var_t0_dn2 - locals.var_t2_dn2), (locals.var_t0_dn4 - locals.var_t2_dn4), (locals.var_t0_dn5 - locals.var_t2_dn5), (locals.var_t0_dn6 - locals.var_t2_dn6), (locals.var_t0_dn7 - locals.var_t2_dn7), (locals.var_t0_dn8 - locals.var_t2_dn8), (locals.var_t0_dn9 - locals.var_t2_dn9), (locals.var_t0_dn10 - locals.var_t2_dn10), (locals.var_t0_dn13 - locals.var_t2_dn13),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
            locals.var_t4 = assign89530_body27_e136601;
            locals.var_t4_dn0 = assign89530_body27_e136601_d_n0;
            locals.var_t4_dn2 = assign89530_body27_e136601_d_n2;
            locals.var_t4_dn4 = assign89530_body27_e136601_d_n4;
            locals.var_t4_dn5 = assign89530_body27_e136601_d_n5;
            locals.var_t4_dn6 = assign89530_body27_e136601_d_n6;
            locals.var_t4_dn7 = assign89530_body27_e136601_d_n7;
            locals.var_t4_dn8 = assign89530_body27_e136601_d_n8;
            locals.var_t4_dn9 = assign89530_body27_e136601_d_n9;
            locals.var_t4_dn10 = assign89530_body27_e136601_d_n10;
            locals.var_t4_dn13 = assign89530_body27_e136601_d_n13;
            locals.var_t4_rv = 0.0;
            let assign89530_body28_e136604: f64 = if locals.var_t4 > 0.0 { 1.0 } else { 0.0 };
            locals.var_guard2075 = assign89530_body28_e136604;
            locals.var_guard2075_rv = 0.0;
            let (assign89530_body29_e136621, assign89530_body29_e136621_d_n0, assign89530_body29_e136621_d_n2, assign89530_body29_e136621_d_n4, assign89530_body29_e136621_d_n5, assign89530_body29_e136621_d_n6, assign89530_body29_e136621_d_n7, assign89530_body29_e136621_d_n8, assign89530_body29_e136621_d_n9, assign89530_body29_e136621_d_n10, assign89530_body29_e136621_d_n13,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) && (locals.var_guard2073 == 0.0)) && (locals.var_guard2074 != 0.0)) && (locals.var_guard2075 != 0.0)) {
        let assign89530_body29_e136619: f64 = (locals.var_t4).sqrt();
        (assign89530_body29_e136619, (locals.var_t4_dn0 / (2.0 * assign89530_body29_e136619)), (locals.var_t4_dn2 / (2.0 * assign89530_body29_e136619)), (locals.var_t4_dn4 / (2.0 * assign89530_body29_e136619)), (locals.var_t4_dn5 / (2.0 * assign89530_body29_e136619)), (locals.var_t4_dn6 / (2.0 * assign89530_body29_e136619)), (locals.var_t4_dn7 / (2.0 * assign89530_body29_e136619)), (locals.var_t4_dn8 / (2.0 * assign89530_body29_e136619)), (locals.var_t4_dn9 / (2.0 * assign89530_body29_e136619)), (locals.var_t4_dn10 / (2.0 * assign89530_body29_e136619)), (locals.var_t4_dn13 / (2.0 * assign89530_body29_e136619)),)
    } else {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn4, locals.var_fb_dn5, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn8, locals.var_fb_dn9, locals.var_fb_dn10, locals.var_fb_dn13,)
    }
};
            locals.var_fb = assign89530_body29_e136621;
            locals.var_fb_dn0 = assign89530_body29_e136621_d_n0;
            locals.var_fb_dn2 = assign89530_body29_e136621_d_n2;
            locals.var_fb_dn4 = assign89530_body29_e136621_d_n4;
            locals.var_fb_dn5 = assign89530_body29_e136621_d_n5;
            locals.var_fb_dn6 = assign89530_body29_e136621_d_n6;
            locals.var_fb_dn7 = assign89530_body29_e136621_d_n7;
            locals.var_fb_dn8 = assign89530_body29_e136621_d_n8;
            locals.var_fb_dn9 = assign89530_body29_e136621_d_n9;
            locals.var_fb_dn10 = assign89530_body29_e136621_d_n10;
            locals.var_fb_dn13 = assign89530_body29_e136621_d_n13;
            locals.var_fb_rv = 0.0;
            let (assign89530_body30_e136647, assign89530_body30_e136647_d_n0, assign89530_body30_e136647_d_n2, assign89530_body30_e136647_d_n4, assign89530_body30_e136647_d_n5, assign89530_body30_e136647_d_n6, assign89530_body30_e136647_d_n7, assign89530_body30_e136647_d_n8, assign89530_body30_e136647_d_n9, assign89530_body30_e136647_d_n10, assign89530_body30_e136647_d_n13,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) && (locals.var_guard2073 == 0.0)) && (locals.var_guard2074 != 0.0)) && (locals.var_guard2075 != 0.0)) {
        let assign89530_body30_e136637: f64 = (locals.var_beta * 0.5);
        let assign89530_body30_e136641: f64 = (locals.var_phi_b_dpss * locals.var_t3);
        let assign89530_body30_e136642: f64 = (locals.var_t1 - assign89530_body30_e136641);
        let assign89530_body30_e136643: f64 = (assign89530_body30_e136637 * assign89530_body30_e136642);
        let assign89530_body30_e136645: f64 = (assign89530_body30_e136643 / locals.var_fb);
        (assign89530_body30_e136645, ((((((locals.var_beta_dn0 * 0.5) * assign89530_body30_e136642) + (assign89530_body30_e136637 * (locals.var_t1_dn0 - ((locals.var_phi_b_dpss_dn0 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn0))))) * locals.var_fb) - (assign89530_body30_e136643 * locals.var_fb_dn0)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn2 * 0.5) * assign89530_body30_e136642) + (assign89530_body30_e136637 * (locals.var_t1_dn2 - ((locals.var_phi_b_dpss_dn2 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn2))))) * locals.var_fb) - (assign89530_body30_e136643 * locals.var_fb_dn2)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn4 * 0.5) * assign89530_body30_e136642) + (assign89530_body30_e136637 * (locals.var_t1_dn4 - ((locals.var_phi_b_dpss_dn4 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn4))))) * locals.var_fb) - (assign89530_body30_e136643 * locals.var_fb_dn4)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn5 * 0.5) * assign89530_body30_e136642) + (assign89530_body30_e136637 * (locals.var_t1_dn5 - ((locals.var_phi_b_dpss_dn5 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn5))))) * locals.var_fb) - (assign89530_body30_e136643 * locals.var_fb_dn5)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn6 * 0.5) * assign89530_body30_e136642) + (assign89530_body30_e136637 * (locals.var_t1_dn6 - ((locals.var_phi_b_dpss_dn6 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn6))))) * locals.var_fb) - (assign89530_body30_e136643 * locals.var_fb_dn6)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn7 * 0.5) * assign89530_body30_e136642) + (assign89530_body30_e136637 * (locals.var_t1_dn7 - ((locals.var_phi_b_dpss_dn7 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn7))))) * locals.var_fb) - (assign89530_body30_e136643 * locals.var_fb_dn7)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn8 * 0.5) * assign89530_body30_e136642) + (assign89530_body30_e136637 * (locals.var_t1_dn8 - ((locals.var_phi_b_dpss_dn8 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn8))))) * locals.var_fb) - (assign89530_body30_e136643 * locals.var_fb_dn8)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn9 * 0.5) * assign89530_body30_e136642) + (assign89530_body30_e136637 * (locals.var_t1_dn9 - ((locals.var_phi_b_dpss_dn9 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn9))))) * locals.var_fb) - (assign89530_body30_e136643 * locals.var_fb_dn9)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn10 * 0.5) * assign89530_body30_e136642) + (assign89530_body30_e136637 * (locals.var_t1_dn10 - ((locals.var_phi_b_dpss_dn10 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn10))))) * locals.var_fb) - (assign89530_body30_e136643 * locals.var_fb_dn10)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn13 * 0.5) * assign89530_body30_e136642) + (assign89530_body30_e136637 * (locals.var_t1_dn13 - ((locals.var_phi_b_dpss_dn13 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn13))))) * locals.var_fb) - (assign89530_body30_e136643 * locals.var_fb_dn13)) / (locals.var_fb * locals.var_fb)),)
    } else {
        (locals.var_fb_dpss, locals.var_fb_dpss_dn0, locals.var_fb_dpss_dn2, locals.var_fb_dpss_dn4, locals.var_fb_dpss_dn5, locals.var_fb_dpss_dn6, locals.var_fb_dpss_dn7, locals.var_fb_dpss_dn8, locals.var_fb_dpss_dn9, locals.var_fb_dpss_dn10, locals.var_fb_dpss_dn13,)
    }
};
            locals.var_fb_dpss = assign89530_body30_e136647;
            locals.var_fb_dpss_dn0 = assign89530_body30_e136647_d_n0;
            locals.var_fb_dpss_dn2 = assign89530_body30_e136647_d_n2;
            locals.var_fb_dpss_dn4 = assign89530_body30_e136647_d_n4;
            locals.var_fb_dpss_dn5 = assign89530_body30_e136647_d_n5;
            locals.var_fb_dpss_dn6 = assign89530_body30_e136647_d_n6;
            locals.var_fb_dpss_dn7 = assign89530_body30_e136647_d_n7;
            locals.var_fb_dpss_dn8 = assign89530_body30_e136647_d_n8;
            locals.var_fb_dpss_dn9 = assign89530_body30_e136647_d_n9;
            locals.var_fb_dpss_dn10 = assign89530_body30_e136647_d_n10;
            locals.var_fb_dpss_dn13 = assign89530_body30_e136647_d_n13;
            locals.var_fb_dpss_rv = 0.0;
            let (assign89530_body32_e136683, assign89530_body32_e136683_d_n0, assign89530_body32_e136683_d_n2, assign89530_body32_e136683_d_n4, assign89530_body32_e136683_d_n5, assign89530_body32_e136683_d_n6, assign89530_body32_e136683_d_n7, assign89530_body32_e136683_d_n8, assign89530_body32_e136683_d_n9, assign89530_body32_e136683_d_n10, assign89530_body32_e136683_d_n13,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) && (locals.var_guard2073 == 0.0)) && (locals.var_guard2074 != 0.0)) && (locals.var_guard2075 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn4, locals.var_fb_dn5, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn8, locals.var_fb_dn9, locals.var_fb_dn10, locals.var_fb_dn13,)
    }
};
            locals.var_fb = assign89530_body32_e136683;
            locals.var_fb_dn0 = assign89530_body32_e136683_d_n0;
            locals.var_fb_dn2 = assign89530_body32_e136683_d_n2;
            locals.var_fb_dn4 = assign89530_body32_e136683_d_n4;
            locals.var_fb_dn5 = assign89530_body32_e136683_d_n5;
            locals.var_fb_dn6 = assign89530_body32_e136683_d_n6;
            locals.var_fb_dn7 = assign89530_body32_e136683_d_n7;
            locals.var_fb_dn8 = assign89530_body32_e136683_d_n8;
            locals.var_fb_dn9 = assign89530_body32_e136683_d_n9;
            locals.var_fb_dn10 = assign89530_body32_e136683_d_n10;
            locals.var_fb_dn13 = assign89530_body32_e136683_d_n13;
            locals.var_fb_rv = 0.0;
            let (assign89530_body33_e136700, assign89530_body33_e136700_d_n0, assign89530_body33_e136700_d_n2, assign89530_body33_e136700_d_n4, assign89530_body33_e136700_d_n5, assign89530_body33_e136700_d_n6, assign89530_body33_e136700_d_n7, assign89530_body33_e136700_d_n8, assign89530_body33_e136700_d_n9, assign89530_body33_e136700_d_n10, assign89530_body33_e136700_d_n13,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) && (locals.var_guard2073 == 0.0)) && (locals.var_guard2074 != 0.0)) && (locals.var_guard2075 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fb_dpss, locals.var_fb_dpss_dn0, locals.var_fb_dpss_dn2, locals.var_fb_dpss_dn4, locals.var_fb_dpss_dn5, locals.var_fb_dpss_dn6, locals.var_fb_dpss_dn7, locals.var_fb_dpss_dn8, locals.var_fb_dpss_dn9, locals.var_fb_dpss_dn10, locals.var_fb_dpss_dn13,)
    }
};
            locals.var_fb_dpss = assign89530_body33_e136700;
            locals.var_fb_dpss_dn0 = assign89530_body33_e136700_d_n0;
            locals.var_fb_dpss_dn2 = assign89530_body33_e136700_d_n2;
            locals.var_fb_dpss_dn4 = assign89530_body33_e136700_d_n4;
            locals.var_fb_dpss_dn5 = assign89530_body33_e136700_d_n5;
            locals.var_fb_dpss_dn6 = assign89530_body33_e136700_d_n6;
            locals.var_fb_dpss_dn7 = assign89530_body33_e136700_d_n7;
            locals.var_fb_dpss_dn8 = assign89530_body33_e136700_d_n8;
            locals.var_fb_dpss_dn9 = assign89530_body33_e136700_d_n9;
            locals.var_fb_dpss_dn10 = assign89530_body33_e136700_d_n10;
            locals.var_fb_dpss_dn13 = assign89530_body33_e136700_d_n13;
            locals.var_fb_dpss_rv = 0.0;
            let (assign89530_body34_e136717, assign89530_body34_e136717_d_n0, assign89530_body34_e136717_d_n2, assign89530_body34_e136717_d_n4, assign89530_body34_e136717_d_n5, assign89530_body34_e136717_d_n6, assign89530_body34_e136717_d_n7, assign89530_body34_e136717_d_n8, assign89530_body34_e136717_d_n9, assign89530_body34_e136717_d_n10, assign89530_body34_e136717_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) && (locals.var_guard2073 == 0.0)) && (locals.var_guard2074 == 0.0)) {
        let assign89530_body34_e136714: f64 = (-locals.var_chi);
        let assign89530_body34_e136715: f64 = (assign89530_body34_e136714).exp();
        (assign89530_body34_e136715, (assign89530_body34_e136715 * (-locals.var_chi_dn0)), (assign89530_body34_e136715 * (-locals.var_chi_dn2)), (assign89530_body34_e136715 * (-locals.var_chi_dn4)), (assign89530_body34_e136715 * (-locals.var_chi_dn5)), (assign89530_body34_e136715 * (-locals.var_chi_dn6)), (assign89530_body34_e136715 * (-locals.var_chi_dn7)), (assign89530_body34_e136715 * (-locals.var_chi_dn8)), (assign89530_body34_e136715 * (-locals.var_chi_dn9)), (assign89530_body34_e136715 * (-locals.var_chi_dn10)), (assign89530_body34_e136715 * (-locals.var_chi_dn13)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
            locals.var_t0 = assign89530_body34_e136717;
            locals.var_t0_dn0 = assign89530_body34_e136717_d_n0;
            locals.var_t0_dn2 = assign89530_body34_e136717_d_n2;
            locals.var_t0_dn4 = assign89530_body34_e136717_d_n4;
            locals.var_t0_dn5 = assign89530_body34_e136717_d_n5;
            locals.var_t0_dn6 = assign89530_body34_e136717_d_n6;
            locals.var_t0_dn7 = assign89530_body34_e136717_d_n7;
            locals.var_t0_dn8 = assign89530_body34_e136717_d_n8;
            locals.var_t0_dn9 = assign89530_body34_e136717_d_n9;
            locals.var_t0_dn10 = assign89530_body34_e136717_d_n10;
            locals.var_t0_dn13 = assign89530_body34_e136717_d_n13;
            locals.var_t0_rv = 0.0;
            let (assign89530_body35_e136734, assign89530_body35_e136734_d_n0, assign89530_body35_e136734_d_n2, assign89530_body35_e136734_d_n4, assign89530_body35_e136734_d_n5, assign89530_body35_e136734_d_n6, assign89530_body35_e136734_d_n7, assign89530_body35_e136734_d_n8, assign89530_body35_e136734_d_n9, assign89530_body35_e136734_d_n10, assign89530_body35_e136734_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) && (locals.var_guard2073 == 0.0)) && (locals.var_guard2074 == 0.0)) {
        let assign89530_body35_e136731: f64 = (-locals.var_chib);
        let assign89530_body35_e136732: f64 = (assign89530_body35_e136731).exp();
        (assign89530_body35_e136732, (assign89530_body35_e136732 * (-locals.var_chib_dn0)), (assign89530_body35_e136732 * (-locals.var_chib_dn2)), (assign89530_body35_e136732 * (-locals.var_chib_dn4)), (assign89530_body35_e136732 * (-locals.var_chib_dn5)), (assign89530_body35_e136732 * (-locals.var_chib_dn6)), (assign89530_body35_e136732 * (-locals.var_chib_dn7)), (assign89530_body35_e136732 * (-locals.var_chib_dn8)), (assign89530_body35_e136732 * (-locals.var_chib_dn9)), (assign89530_body35_e136732 * (-locals.var_chib_dn10)), (assign89530_body35_e136732 * (-locals.var_chib_dn13)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
            locals.var_t1 = assign89530_body35_e136734;
            locals.var_t1_dn0 = assign89530_body35_e136734_d_n0;
            locals.var_t1_dn2 = assign89530_body35_e136734_d_n2;
            locals.var_t1_dn4 = assign89530_body35_e136734_d_n4;
            locals.var_t1_dn5 = assign89530_body35_e136734_d_n5;
            locals.var_t1_dn6 = assign89530_body35_e136734_d_n6;
            locals.var_t1_dn7 = assign89530_body35_e136734_d_n7;
            locals.var_t1_dn8 = assign89530_body35_e136734_d_n8;
            locals.var_t1_dn9 = assign89530_body35_e136734_d_n9;
            locals.var_t1_dn10 = assign89530_body35_e136734_d_n10;
            locals.var_t1_dn13 = assign89530_body35_e136734_d_n13;
            locals.var_t1_rv = 0.0;
            let (assign89530_body36_e136755, assign89530_body36_e136755_d_n0, assign89530_body36_e136755_d_n2, assign89530_body36_e136755_d_n4, assign89530_body36_e136755_d_n5, assign89530_body36_e136755_d_n6, assign89530_body36_e136755_d_n7, assign89530_body36_e136755_d_n8, assign89530_body36_e136755_d_n9, assign89530_body36_e136755_d_n10, assign89530_body36_e136755_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) && (locals.var_guard2073 == 0.0)) && (locals.var_guard2074 == 0.0)) {
        let assign89530_body36_e136749: f64 = (locals.var_chi - locals.var_chib);
        let assign89530_body36_e136752: f64 = (locals.var_t0 - locals.var_t1);
        let assign89530_body36_e136753: f64 = (assign89530_body36_e136749 + assign89530_body36_e136752);
        (assign89530_body36_e136753, ((locals.var_chi_dn0 - locals.var_chib_dn0) + (locals.var_t0_dn0 - locals.var_t1_dn0)), ((locals.var_chi_dn2 - locals.var_chib_dn2) + (locals.var_t0_dn2 - locals.var_t1_dn2)), ((locals.var_chi_dn4 - locals.var_chib_dn4) + (locals.var_t0_dn4 - locals.var_t1_dn4)), ((locals.var_chi_dn5 - locals.var_chib_dn5) + (locals.var_t0_dn5 - locals.var_t1_dn5)), ((locals.var_chi_dn6 - locals.var_chib_dn6) + (locals.var_t0_dn6 - locals.var_t1_dn6)), ((locals.var_chi_dn7 - locals.var_chib_dn7) + (locals.var_t0_dn7 - locals.var_t1_dn7)), ((locals.var_chi_dn8 - locals.var_chib_dn8) + (locals.var_t0_dn8 - locals.var_t1_dn8)), ((locals.var_chi_dn9 - locals.var_chib_dn9) + (locals.var_t0_dn9 - locals.var_t1_dn9)), ((locals.var_chi_dn10 - locals.var_chib_dn10) + (locals.var_t0_dn10 - locals.var_t1_dn10)), ((locals.var_chi_dn13 - locals.var_chib_dn13) + (locals.var_t0_dn13 - locals.var_t1_dn13)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
            locals.var_t4 = assign89530_body36_e136755;
            locals.var_t4_dn0 = assign89530_body36_e136755_d_n0;
            locals.var_t4_dn2 = assign89530_body36_e136755_d_n2;
            locals.var_t4_dn4 = assign89530_body36_e136755_d_n4;
            locals.var_t4_dn5 = assign89530_body36_e136755_d_n5;
            locals.var_t4_dn6 = assign89530_body36_e136755_d_n6;
            locals.var_t4_dn7 = assign89530_body36_e136755_d_n7;
            locals.var_t4_dn8 = assign89530_body36_e136755_d_n8;
            locals.var_t4_dn9 = assign89530_body36_e136755_d_n9;
            locals.var_t4_dn10 = assign89530_body36_e136755_d_n10;
            locals.var_t4_dn13 = assign89530_body36_e136755_d_n13;
            locals.var_t4_rv = 0.0;
            let assign89530_body37_e136758: f64 = if locals.var_t4 > 0.0 { 1.0 } else { 0.0 };
            locals.var_guard2076 = assign89530_body37_e136758;
            locals.var_guard2076_rv = 0.0;
            let (assign89530_body38_e136776, assign89530_body38_e136776_d_n0, assign89530_body38_e136776_d_n2, assign89530_body38_e136776_d_n4, assign89530_body38_e136776_d_n5, assign89530_body38_e136776_d_n6, assign89530_body38_e136776_d_n7, assign89530_body38_e136776_d_n8, assign89530_body38_e136776_d_n9, assign89530_body38_e136776_d_n10, assign89530_body38_e136776_d_n13,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) && (locals.var_guard2073 == 0.0)) && (locals.var_guard2074 == 0.0)) && (locals.var_guard2076 != 0.0)) {
        let assign89530_body38_e136774: f64 = (locals.var_t4).sqrt();
        (assign89530_body38_e136774, (locals.var_t4_dn0 / (2.0 * assign89530_body38_e136774)), (locals.var_t4_dn2 / (2.0 * assign89530_body38_e136774)), (locals.var_t4_dn4 / (2.0 * assign89530_body38_e136774)), (locals.var_t4_dn5 / (2.0 * assign89530_body38_e136774)), (locals.var_t4_dn6 / (2.0 * assign89530_body38_e136774)), (locals.var_t4_dn7 / (2.0 * assign89530_body38_e136774)), (locals.var_t4_dn8 / (2.0 * assign89530_body38_e136774)), (locals.var_t4_dn9 / (2.0 * assign89530_body38_e136774)), (locals.var_t4_dn10 / (2.0 * assign89530_body38_e136774)), (locals.var_t4_dn13 / (2.0 * assign89530_body38_e136774)),)
    } else {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn4, locals.var_fb_dn5, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn8, locals.var_fb_dn9, locals.var_fb_dn10, locals.var_fb_dn13,)
    }
};
            locals.var_fb = assign89530_body38_e136776;
            locals.var_fb_dn0 = assign89530_body38_e136776_d_n0;
            locals.var_fb_dn2 = assign89530_body38_e136776_d_n2;
            locals.var_fb_dn4 = assign89530_body38_e136776_d_n4;
            locals.var_fb_dn5 = assign89530_body38_e136776_d_n5;
            locals.var_fb_dn6 = assign89530_body38_e136776_d_n6;
            locals.var_fb_dn7 = assign89530_body38_e136776_d_n7;
            locals.var_fb_dn8 = assign89530_body38_e136776_d_n8;
            locals.var_fb_dn9 = assign89530_body38_e136776_d_n9;
            locals.var_fb_dn10 = assign89530_body38_e136776_d_n10;
            locals.var_fb_dn13 = assign89530_body38_e136776_d_n13;
            locals.var_fb_rv = 0.0;
            let (assign89530_body39_e136807, assign89530_body39_e136807_d_n0, assign89530_body39_e136807_d_n2, assign89530_body39_e136807_d_n4, assign89530_body39_e136807_d_n5, assign89530_body39_e136807_d_n6, assign89530_body39_e136807_d_n7, assign89530_body39_e136807_d_n8, assign89530_body39_e136807_d_n9, assign89530_body39_e136807_d_n10, assign89530_body39_e136807_d_n13,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) && (locals.var_guard2073 == 0.0)) && (locals.var_guard2074 == 0.0)) && (locals.var_guard2076 != 0.0)) {
        let assign89530_body39_e136793: f64 = (locals.var_beta * 0.5);
        let assign89530_body39_e136796: f64 = (1.0 - locals.var_t0);
        let assign89530_body39_e136800: f64 = (1.0 - locals.var_t1);
        let assign89530_body39_e136801: f64 = (locals.var_phi_b_dpss * assign89530_body39_e136800);
        let assign89530_body39_e136802: f64 = (assign89530_body39_e136796 - assign89530_body39_e136801);
        let assign89530_body39_e136803: f64 = (assign89530_body39_e136793 * assign89530_body39_e136802);
        let assign89530_body39_e136805: f64 = (assign89530_body39_e136803 / locals.var_fb);
        (assign89530_body39_e136805, ((((((locals.var_beta_dn0 * 0.5) * assign89530_body39_e136802) + (assign89530_body39_e136793 * ((-locals.var_t0_dn0) - ((locals.var_phi_b_dpss_dn0 * assign89530_body39_e136800) + (locals.var_phi_b_dpss * (-locals.var_t1_dn0)))))) * locals.var_fb) - (assign89530_body39_e136803 * locals.var_fb_dn0)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn2 * 0.5) * assign89530_body39_e136802) + (assign89530_body39_e136793 * ((-locals.var_t0_dn2) - ((locals.var_phi_b_dpss_dn2 * assign89530_body39_e136800) + (locals.var_phi_b_dpss * (-locals.var_t1_dn2)))))) * locals.var_fb) - (assign89530_body39_e136803 * locals.var_fb_dn2)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn4 * 0.5) * assign89530_body39_e136802) + (assign89530_body39_e136793 * ((-locals.var_t0_dn4) - ((locals.var_phi_b_dpss_dn4 * assign89530_body39_e136800) + (locals.var_phi_b_dpss * (-locals.var_t1_dn4)))))) * locals.var_fb) - (assign89530_body39_e136803 * locals.var_fb_dn4)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn5 * 0.5) * assign89530_body39_e136802) + (assign89530_body39_e136793 * ((-locals.var_t0_dn5) - ((locals.var_phi_b_dpss_dn5 * assign89530_body39_e136800) + (locals.var_phi_b_dpss * (-locals.var_t1_dn5)))))) * locals.var_fb) - (assign89530_body39_e136803 * locals.var_fb_dn5)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn6 * 0.5) * assign89530_body39_e136802) + (assign89530_body39_e136793 * ((-locals.var_t0_dn6) - ((locals.var_phi_b_dpss_dn6 * assign89530_body39_e136800) + (locals.var_phi_b_dpss * (-locals.var_t1_dn6)))))) * locals.var_fb) - (assign89530_body39_e136803 * locals.var_fb_dn6)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn7 * 0.5) * assign89530_body39_e136802) + (assign89530_body39_e136793 * ((-locals.var_t0_dn7) - ((locals.var_phi_b_dpss_dn7 * assign89530_body39_e136800) + (locals.var_phi_b_dpss * (-locals.var_t1_dn7)))))) * locals.var_fb) - (assign89530_body39_e136803 * locals.var_fb_dn7)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn8 * 0.5) * assign89530_body39_e136802) + (assign89530_body39_e136793 * ((-locals.var_t0_dn8) - ((locals.var_phi_b_dpss_dn8 * assign89530_body39_e136800) + (locals.var_phi_b_dpss * (-locals.var_t1_dn8)))))) * locals.var_fb) - (assign89530_body39_e136803 * locals.var_fb_dn8)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn9 * 0.5) * assign89530_body39_e136802) + (assign89530_body39_e136793 * ((-locals.var_t0_dn9) - ((locals.var_phi_b_dpss_dn9 * assign89530_body39_e136800) + (locals.var_phi_b_dpss * (-locals.var_t1_dn9)))))) * locals.var_fb) - (assign89530_body39_e136803 * locals.var_fb_dn9)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn10 * 0.5) * assign89530_body39_e136802) + (assign89530_body39_e136793 * ((-locals.var_t0_dn10) - ((locals.var_phi_b_dpss_dn10 * assign89530_body39_e136800) + (locals.var_phi_b_dpss * (-locals.var_t1_dn10)))))) * locals.var_fb) - (assign89530_body39_e136803 * locals.var_fb_dn10)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn13 * 0.5) * assign89530_body39_e136802) + (assign89530_body39_e136793 * ((-locals.var_t0_dn13) - ((locals.var_phi_b_dpss_dn13 * assign89530_body39_e136800) + (locals.var_phi_b_dpss * (-locals.var_t1_dn13)))))) * locals.var_fb) - (assign89530_body39_e136803 * locals.var_fb_dn13)) / (locals.var_fb * locals.var_fb)),)
    } else {
        (locals.var_fb_dpss, locals.var_fb_dpss_dn0, locals.var_fb_dpss_dn2, locals.var_fb_dpss_dn4, locals.var_fb_dpss_dn5, locals.var_fb_dpss_dn6, locals.var_fb_dpss_dn7, locals.var_fb_dpss_dn8, locals.var_fb_dpss_dn9, locals.var_fb_dpss_dn10, locals.var_fb_dpss_dn13,)
    }
};
            locals.var_fb_dpss = assign89530_body39_e136807;
            locals.var_fb_dpss_dn0 = assign89530_body39_e136807_d_n0;
            locals.var_fb_dpss_dn2 = assign89530_body39_e136807_d_n2;
            locals.var_fb_dpss_dn4 = assign89530_body39_e136807_d_n4;
            locals.var_fb_dpss_dn5 = assign89530_body39_e136807_d_n5;
            locals.var_fb_dpss_dn6 = assign89530_body39_e136807_d_n6;
            locals.var_fb_dpss_dn7 = assign89530_body39_e136807_d_n7;
            locals.var_fb_dpss_dn8 = assign89530_body39_e136807_d_n8;
            locals.var_fb_dpss_dn9 = assign89530_body39_e136807_d_n9;
            locals.var_fb_dpss_dn10 = assign89530_body39_e136807_d_n10;
            locals.var_fb_dpss_dn13 = assign89530_body39_e136807_d_n13;
            locals.var_fb_dpss_rv = 0.0;
            let (assign89530_body41_e136845, assign89530_body41_e136845_d_n0, assign89530_body41_e136845_d_n2, assign89530_body41_e136845_d_n4, assign89530_body41_e136845_d_n5, assign89530_body41_e136845_d_n6, assign89530_body41_e136845_d_n7, assign89530_body41_e136845_d_n8, assign89530_body41_e136845_d_n9, assign89530_body41_e136845_d_n10, assign89530_body41_e136845_d_n13,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) && (locals.var_guard2073 == 0.0)) && (locals.var_guard2074 == 0.0)) && (locals.var_guard2076 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn4, locals.var_fb_dn5, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn8, locals.var_fb_dn9, locals.var_fb_dn10, locals.var_fb_dn13,)
    }
};
            locals.var_fb = assign89530_body41_e136845;
            locals.var_fb_dn0 = assign89530_body41_e136845_d_n0;
            locals.var_fb_dn2 = assign89530_body41_e136845_d_n2;
            locals.var_fb_dn4 = assign89530_body41_e136845_d_n4;
            locals.var_fb_dn5 = assign89530_body41_e136845_d_n5;
            locals.var_fb_dn6 = assign89530_body41_e136845_d_n6;
            locals.var_fb_dn7 = assign89530_body41_e136845_d_n7;
            locals.var_fb_dn8 = assign89530_body41_e136845_d_n8;
            locals.var_fb_dn9 = assign89530_body41_e136845_d_n9;
            locals.var_fb_dn10 = assign89530_body41_e136845_d_n10;
            locals.var_fb_dn13 = assign89530_body41_e136845_d_n13;
            locals.var_fb_rv = 0.0;
            let (assign89530_body42_e136863, assign89530_body42_e136863_d_n0, assign89530_body42_e136863_d_n2, assign89530_body42_e136863_d_n4, assign89530_body42_e136863_d_n5, assign89530_body42_e136863_d_n6, assign89530_body42_e136863_d_n7, assign89530_body42_e136863_d_n8, assign89530_body42_e136863_d_n9, assign89530_body42_e136863_d_n10, assign89530_body42_e136863_d_n13,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) && (locals.var_guard2073 == 0.0)) && (locals.var_guard2074 == 0.0)) && (locals.var_guard2076 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fb_dpss, locals.var_fb_dpss_dn0, locals.var_fb_dpss_dn2, locals.var_fb_dpss_dn4, locals.var_fb_dpss_dn5, locals.var_fb_dpss_dn6, locals.var_fb_dpss_dn7, locals.var_fb_dpss_dn8, locals.var_fb_dpss_dn9, locals.var_fb_dpss_dn10, locals.var_fb_dpss_dn13,)
    }
};
            locals.var_fb_dpss = assign89530_body42_e136863;
            locals.var_fb_dpss_dn0 = assign89530_body42_e136863_d_n0;
            locals.var_fb_dpss_dn2 = assign89530_body42_e136863_d_n2;
            locals.var_fb_dpss_dn4 = assign89530_body42_e136863_d_n4;
            locals.var_fb_dpss_dn5 = assign89530_body42_e136863_d_n5;
            locals.var_fb_dpss_dn6 = assign89530_body42_e136863_d_n6;
            locals.var_fb_dpss_dn7 = assign89530_body42_e136863_d_n7;
            locals.var_fb_dpss_dn8 = assign89530_body42_e136863_d_n8;
            locals.var_fb_dpss_dn9 = assign89530_body42_e136863_d_n9;
            locals.var_fb_dpss_dn10 = assign89530_body42_e136863_d_n10;
            locals.var_fb_dpss_dn13 = assign89530_body42_e136863_d_n13;
            locals.var_fb_dpss_rv = 0.0;
            let assign89530_body43_e136866: f64 = if locals.var_chi < 0.0 { 1.0 } else { 0.0 };
            locals.var_guard2077 = assign89530_body43_e136866;
            locals.var_guard2077_rv = 0.0;
            let (assign89530_body45_e136890, assign89530_body45_e136890_d_n0, assign89530_body45_e136890_d_n2, assign89530_body45_e136890_d_n4, assign89530_body45_e136890_d_n5, assign89530_body45_e136890_d_n6, assign89530_body45_e136890_d_n7, assign89530_body45_e136890_d_n8, assign89530_body45_e136890_d_n9, assign89530_body45_e136890_d_n10, assign89530_body45_e136890_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) && (locals.var_guard2077 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fs01, locals.var_fs01_dn0, locals.var_fs01_dn2, locals.var_fs01_dn4, locals.var_fs01_dn5, locals.var_fs01_dn6, locals.var_fs01_dn7, locals.var_fs01_dn8, locals.var_fs01_dn9, locals.var_fs01_dn10, locals.var_fs01_dn13,)
    }
};
            locals.var_fs01 = assign89530_body45_e136890;
            locals.var_fs01_dn0 = assign89530_body45_e136890_d_n0;
            locals.var_fs01_dn2 = assign89530_body45_e136890_d_n2;
            locals.var_fs01_dn4 = assign89530_body45_e136890_d_n4;
            locals.var_fs01_dn5 = assign89530_body45_e136890_d_n5;
            locals.var_fs01_dn6 = assign89530_body45_e136890_d_n6;
            locals.var_fs01_dn7 = assign89530_body45_e136890_d_n7;
            locals.var_fs01_dn8 = assign89530_body45_e136890_d_n8;
            locals.var_fs01_dn9 = assign89530_body45_e136890_d_n9;
            locals.var_fs01_dn10 = assign89530_body45_e136890_d_n10;
            locals.var_fs01_dn13 = assign89530_body45_e136890_d_n13;
            locals.var_fs01_rv = 0.0;
            let (assign89530_body46_e136901, assign89530_body46_e136901_d_n0, assign89530_body46_e136901_d_n2, assign89530_body46_e136901_d_n4, assign89530_body46_e136901_d_n5, assign89530_body46_e136901_d_n6, assign89530_body46_e136901_d_n7, assign89530_body46_e136901_d_n8, assign89530_body46_e136901_d_n9, assign89530_body46_e136901_d_n10, assign89530_body46_e136901_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) && (locals.var_guard2077 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fs01_dps0, locals.var_fs01_dps0_dn0, locals.var_fs01_dps0_dn2, locals.var_fs01_dps0_dn4, locals.var_fs01_dps0_dn5, locals.var_fs01_dps0_dn6, locals.var_fs01_dps0_dn7, locals.var_fs01_dps0_dn8, locals.var_fs01_dps0_dn9, locals.var_fs01_dps0_dn10, locals.var_fs01_dps0_dn13,)
    }
};
            locals.var_fs01_dps0 = assign89530_body46_e136901;
            locals.var_fs01_dps0_dn0 = assign89530_body46_e136901_d_n0;
            locals.var_fs01_dps0_dn2 = assign89530_body46_e136901_d_n2;
            locals.var_fs01_dps0_dn4 = assign89530_body46_e136901_d_n4;
            locals.var_fs01_dps0_dn5 = assign89530_body46_e136901_d_n5;
            locals.var_fs01_dps0_dn6 = assign89530_body46_e136901_d_n6;
            locals.var_fs01_dps0_dn7 = assign89530_body46_e136901_d_n7;
            locals.var_fs01_dps0_dn8 = assign89530_body46_e136901_d_n8;
            locals.var_fs01_dps0_dn9 = assign89530_body46_e136901_d_n9;
            locals.var_fs01_dps0_dn10 = assign89530_body46_e136901_d_n10;
            locals.var_fs01_dps0_dn13 = assign89530_body46_e136901_d_n13;
            locals.var_fs01_dps0_rv = 0.0;
            let (assign89530_body47_e136913, assign89530_body47_e136913_d_n0, assign89530_body47_e136913_d_n2, assign89530_body47_e136913_d_n4, assign89530_body47_e136913_d_n5, assign89530_body47_e136913_d_n6, assign89530_body47_e136913_d_n7, assign89530_body47_e136913_d_n8, assign89530_body47_e136913_d_n9, assign89530_body47_e136913_d_n10, assign89530_body47_e136913_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) && (locals.var_guard2077 != 0.0)) {
        let assign89530_body47_e136911: f64 = (-locals.var_fb);
        (assign89530_body47_e136911, (-locals.var_fb_dn0), (-locals.var_fb_dn2), (-locals.var_fb_dn4), (-locals.var_fb_dn5), (-locals.var_fb_dn6), (-locals.var_fb_dn7), (-locals.var_fb_dn8), (-locals.var_fb_dn9), (-locals.var_fb_dn10), (-locals.var_fb_dn13),)
    } else {
        (locals.var_fs02, locals.var_fs02_dn0, locals.var_fs02_dn2, locals.var_fs02_dn4, locals.var_fs02_dn5, locals.var_fs02_dn6, locals.var_fs02_dn7, locals.var_fs02_dn8, locals.var_fs02_dn9, locals.var_fs02_dn10, locals.var_fs02_dn13,)
    }
};
            locals.var_fs02 = assign89530_body47_e136913;
            locals.var_fs02_dn0 = assign89530_body47_e136913_d_n0;
            locals.var_fs02_dn2 = assign89530_body47_e136913_d_n2;
            locals.var_fs02_dn4 = assign89530_body47_e136913_d_n4;
            locals.var_fs02_dn5 = assign89530_body47_e136913_d_n5;
            locals.var_fs02_dn6 = assign89530_body47_e136913_d_n6;
            locals.var_fs02_dn7 = assign89530_body47_e136913_d_n7;
            locals.var_fs02_dn8 = assign89530_body47_e136913_d_n8;
            locals.var_fs02_dn9 = assign89530_body47_e136913_d_n9;
            locals.var_fs02_dn10 = assign89530_body47_e136913_d_n10;
            locals.var_fs02_dn13 = assign89530_body47_e136913_d_n13;
            locals.var_fs02_rv = 0.0;
            let (assign89530_body48_e136925, assign89530_body48_e136925_d_n0, assign89530_body48_e136925_d_n2, assign89530_body48_e136925_d_n4, assign89530_body48_e136925_d_n5, assign89530_body48_e136925_d_n6, assign89530_body48_e136925_d_n7, assign89530_body48_e136925_d_n8, assign89530_body48_e136925_d_n9, assign89530_body48_e136925_d_n10, assign89530_body48_e136925_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) && (locals.var_guard2077 != 0.0)) {
        let assign89530_body48_e136923: f64 = (-locals.var_fb_dpss);
        (assign89530_body48_e136923, (-locals.var_fb_dpss_dn0), (-locals.var_fb_dpss_dn2), (-locals.var_fb_dpss_dn4), (-locals.var_fb_dpss_dn5), (-locals.var_fb_dpss_dn6), (-locals.var_fb_dpss_dn7), (-locals.var_fb_dpss_dn8), (-locals.var_fb_dpss_dn9), (-locals.var_fb_dpss_dn10), (-locals.var_fb_dpss_dn13),)
    } else {
        (locals.var_fs02_dps0, locals.var_fs02_dps0_dn0, locals.var_fs02_dps0_dn2, locals.var_fs02_dps0_dn4, locals.var_fs02_dps0_dn5, locals.var_fs02_dps0_dn6, locals.var_fs02_dps0_dn7, locals.var_fs02_dps0_dn8, locals.var_fs02_dps0_dn9, locals.var_fs02_dps0_dn10, locals.var_fs02_dps0_dn13,)
    }
};
            locals.var_fs02_dps0 = assign89530_body48_e136925;
            locals.var_fs02_dps0_dn0 = assign89530_body48_e136925_d_n0;
            locals.var_fs02_dps0_dn2 = assign89530_body48_e136925_d_n2;
            locals.var_fs02_dps0_dn4 = assign89530_body48_e136925_d_n4;
            locals.var_fs02_dps0_dn5 = assign89530_body48_e136925_d_n5;
            locals.var_fs02_dps0_dn6 = assign89530_body48_e136925_d_n6;
            locals.var_fs02_dps0_dn7 = assign89530_body48_e136925_d_n7;
            locals.var_fs02_dps0_dn8 = assign89530_body48_e136925_d_n8;
            locals.var_fs02_dps0_dn9 = assign89530_body48_e136925_d_n9;
            locals.var_fs02_dps0_dn10 = assign89530_body48_e136925_d_n10;
            locals.var_fs02_dps0_dn13 = assign89530_body48_e136925_d_n13;
            locals.var_fs02_dps0_rv = 0.0;
            let assign89530_body49_e136928: f64 = if locals.var_chi < 60.0 { 1.0 } else { 0.0 };
            locals.var_guard2078 = assign89530_body49_e136928;
            locals.var_guard2078_rv = 0.0;
            let assign89530_body50_e136931: f64 = if locals.var_chi < 5e-5 { 1.0 } else { 0.0 };
            locals.var_guard2079 = assign89530_body50_e136931;
            locals.var_guard2079_rv = 0.0;
            let (assign89530_body51_e136969, assign89530_body51_e136969_d_n0, assign89530_body51_e136969_d_n2, assign89530_body51_e136969_d_n4, assign89530_body51_e136969_d_n5, assign89530_body51_e136969_d_n6, assign89530_body51_e136969_d_n7, assign89530_body51_e136969_d_n8, assign89530_body51_e136969_d_n9, assign89530_body51_e136969_d_n10, assign89530_body51_e136969_d_n13,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) && (locals.var_guard2077 == 0.0)) && (locals.var_guard2078 != 0.0)) && (locals.var_guard2079 != 0.0)) {
        let assign89530_body51_e136947: f64 = (locals.var_chi * locals.var_chi);
        let assign89530_body51_e136949: f64 = (assign89530_body51_e136947 / 2.0);
        let assign89530_body51_e136953: f64 = (locals.var_chi / 3.0);
        let assign89530_body51_e136957: f64 = (locals.var_chi / 4.0);
        let assign89530_body51_e136961: f64 = (locals.var_chi / 5.0);
        let assign89530_body51_e136962: f64 = (1.0 + assign89530_body51_e136961);
        let assign89530_body51_e136963: f64 = (assign89530_body51_e136957 * assign89530_body51_e136962);
        let assign89530_body51_e136964: f64 = (1.0 + assign89530_body51_e136963);
        let assign89530_body51_e136965: f64 = (assign89530_body51_e136953 * assign89530_body51_e136964);
        let assign89530_body51_e136966: f64 = (1.0 + assign89530_body51_e136965);
        let assign89530_body51_e136967: f64 = (assign89530_body51_e136949 * assign89530_body51_e136966);
        (assign89530_body51_e136967, (((((locals.var_chi_dn0 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn0)) / 2.0) * assign89530_body51_e136966) + (assign89530_body51_e136949 * (((locals.var_chi_dn0 / 3.0) * assign89530_body51_e136964) + (assign89530_body51_e136953 * (((locals.var_chi_dn0 / 4.0) * assign89530_body51_e136962) + (assign89530_body51_e136957 * (locals.var_chi_dn0 / 5.0))))))), (((((locals.var_chi_dn2 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn2)) / 2.0) * assign89530_body51_e136966) + (assign89530_body51_e136949 * (((locals.var_chi_dn2 / 3.0) * assign89530_body51_e136964) + (assign89530_body51_e136953 * (((locals.var_chi_dn2 / 4.0) * assign89530_body51_e136962) + (assign89530_body51_e136957 * (locals.var_chi_dn2 / 5.0))))))), (((((locals.var_chi_dn4 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn4)) / 2.0) * assign89530_body51_e136966) + (assign89530_body51_e136949 * (((locals.var_chi_dn4 / 3.0) * assign89530_body51_e136964) + (assign89530_body51_e136953 * (((locals.var_chi_dn4 / 4.0) * assign89530_body51_e136962) + (assign89530_body51_e136957 * (locals.var_chi_dn4 / 5.0))))))), (((((locals.var_chi_dn5 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn5)) / 2.0) * assign89530_body51_e136966) + (assign89530_body51_e136949 * (((locals.var_chi_dn5 / 3.0) * assign89530_body51_e136964) + (assign89530_body51_e136953 * (((locals.var_chi_dn5 / 4.0) * assign89530_body51_e136962) + (assign89530_body51_e136957 * (locals.var_chi_dn5 / 5.0))))))), (((((locals.var_chi_dn6 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn6)) / 2.0) * assign89530_body51_e136966) + (assign89530_body51_e136949 * (((locals.var_chi_dn6 / 3.0) * assign89530_body51_e136964) + (assign89530_body51_e136953 * (((locals.var_chi_dn6 / 4.0) * assign89530_body51_e136962) + (assign89530_body51_e136957 * (locals.var_chi_dn6 / 5.0))))))), (((((locals.var_chi_dn7 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn7)) / 2.0) * assign89530_body51_e136966) + (assign89530_body51_e136949 * (((locals.var_chi_dn7 / 3.0) * assign89530_body51_e136964) + (assign89530_body51_e136953 * (((locals.var_chi_dn7 / 4.0) * assign89530_body51_e136962) + (assign89530_body51_e136957 * (locals.var_chi_dn7 / 5.0))))))), (((((locals.var_chi_dn8 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn8)) / 2.0) * assign89530_body51_e136966) + (assign89530_body51_e136949 * (((locals.var_chi_dn8 / 3.0) * assign89530_body51_e136964) + (assign89530_body51_e136953 * (((locals.var_chi_dn8 / 4.0) * assign89530_body51_e136962) + (assign89530_body51_e136957 * (locals.var_chi_dn8 / 5.0))))))), (((((locals.var_chi_dn9 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn9)) / 2.0) * assign89530_body51_e136966) + (assign89530_body51_e136949 * (((locals.var_chi_dn9 / 3.0) * assign89530_body51_e136964) + (assign89530_body51_e136953 * (((locals.var_chi_dn9 / 4.0) * assign89530_body51_e136962) + (assign89530_body51_e136957 * (locals.var_chi_dn9 / 5.0))))))), (((((locals.var_chi_dn10 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn10)) / 2.0) * assign89530_body51_e136966) + (assign89530_body51_e136949 * (((locals.var_chi_dn10 / 3.0) * assign89530_body51_e136964) + (assign89530_body51_e136953 * (((locals.var_chi_dn10 / 4.0) * assign89530_body51_e136962) + (assign89530_body51_e136957 * (locals.var_chi_dn10 / 5.0))))))), (((((locals.var_chi_dn13 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn13)) / 2.0) * assign89530_body51_e136966) + (assign89530_body51_e136949 * (((locals.var_chi_dn13 / 3.0) * assign89530_body51_e136964) + (assign89530_body51_e136953 * (((locals.var_chi_dn13 / 4.0) * assign89530_body51_e136962) + (assign89530_body51_e136957 * (locals.var_chi_dn13 / 5.0))))))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
            locals.var_t0 = assign89530_body51_e136969;
            locals.var_t0_dn0 = assign89530_body51_e136969_d_n0;
            locals.var_t0_dn2 = assign89530_body51_e136969_d_n2;
            locals.var_t0_dn4 = assign89530_body51_e136969_d_n4;
            locals.var_t0_dn5 = assign89530_body51_e136969_d_n5;
            locals.var_t0_dn6 = assign89530_body51_e136969_d_n6;
            locals.var_t0_dn7 = assign89530_body51_e136969_d_n7;
            locals.var_t0_dn8 = assign89530_body51_e136969_d_n8;
            locals.var_t0_dn9 = assign89530_body51_e136969_d_n9;
            locals.var_t0_dn10 = assign89530_body51_e136969_d_n10;
            locals.var_t0_dn13 = assign89530_body51_e136969_d_n13;
            locals.var_t0_rv = 0.0;
            let (assign89530_body52_e137003, assign89530_body52_e137003_d_n0, assign89530_body52_e137003_d_n2, assign89530_body52_e137003_d_n4, assign89530_body52_e137003_d_n5, assign89530_body52_e137003_d_n6, assign89530_body52_e137003_d_n7, assign89530_body52_e137003_d_n8, assign89530_body52_e137003_d_n9, assign89530_body52_e137003_d_n10, assign89530_body52_e137003_d_n13,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) && (locals.var_guard2077 == 0.0)) && (locals.var_guard2078 != 0.0)) && (locals.var_guard2079 != 0.0)) {
        let assign89530_body52_e136987: f64 = (locals.var_chi / 2.0);
        let assign89530_body52_e136991: f64 = (locals.var_chi / 3.0);
        let assign89530_body52_e136995: f64 = (locals.var_chi / 4.0);
        let assign89530_body52_e136996: f64 = (1.0 + assign89530_body52_e136995);
        let assign89530_body52_e136997: f64 = (assign89530_body52_e136991 * assign89530_body52_e136996);
        let assign89530_body52_e136998: f64 = (1.0 + assign89530_body52_e136997);
        let assign89530_body52_e136999: f64 = (assign89530_body52_e136987 * assign89530_body52_e136998);
        let assign89530_body52_e137000: f64 = (1.0 + assign89530_body52_e136999);
        let assign89530_body52_e137001: f64 = (locals.var_chi * assign89530_body52_e137000);
        (assign89530_body52_e137001, ((locals.var_chi_dn0 * assign89530_body52_e137000) + (locals.var_chi * (((locals.var_chi_dn0 / 2.0) * assign89530_body52_e136998) + (assign89530_body52_e136987 * (((locals.var_chi_dn0 / 3.0) * assign89530_body52_e136996) + (assign89530_body52_e136991 * (locals.var_chi_dn0 / 4.0))))))), ((locals.var_chi_dn2 * assign89530_body52_e137000) + (locals.var_chi * (((locals.var_chi_dn2 / 2.0) * assign89530_body52_e136998) + (assign89530_body52_e136987 * (((locals.var_chi_dn2 / 3.0) * assign89530_body52_e136996) + (assign89530_body52_e136991 * (locals.var_chi_dn2 / 4.0))))))), ((locals.var_chi_dn4 * assign89530_body52_e137000) + (locals.var_chi * (((locals.var_chi_dn4 / 2.0) * assign89530_body52_e136998) + (assign89530_body52_e136987 * (((locals.var_chi_dn4 / 3.0) * assign89530_body52_e136996) + (assign89530_body52_e136991 * (locals.var_chi_dn4 / 4.0))))))), ((locals.var_chi_dn5 * assign89530_body52_e137000) + (locals.var_chi * (((locals.var_chi_dn5 / 2.0) * assign89530_body52_e136998) + (assign89530_body52_e136987 * (((locals.var_chi_dn5 / 3.0) * assign89530_body52_e136996) + (assign89530_body52_e136991 * (locals.var_chi_dn5 / 4.0))))))), ((locals.var_chi_dn6 * assign89530_body52_e137000) + (locals.var_chi * (((locals.var_chi_dn6 / 2.0) * assign89530_body52_e136998) + (assign89530_body52_e136987 * (((locals.var_chi_dn6 / 3.0) * assign89530_body52_e136996) + (assign89530_body52_e136991 * (locals.var_chi_dn6 / 4.0))))))), ((locals.var_chi_dn7 * assign89530_body52_e137000) + (locals.var_chi * (((locals.var_chi_dn7 / 2.0) * assign89530_body52_e136998) + (assign89530_body52_e136987 * (((locals.var_chi_dn7 / 3.0) * assign89530_body52_e136996) + (assign89530_body52_e136991 * (locals.var_chi_dn7 / 4.0))))))), ((locals.var_chi_dn8 * assign89530_body52_e137000) + (locals.var_chi * (((locals.var_chi_dn8 / 2.0) * assign89530_body52_e136998) + (assign89530_body52_e136987 * (((locals.var_chi_dn8 / 3.0) * assign89530_body52_e136996) + (assign89530_body52_e136991 * (locals.var_chi_dn8 / 4.0))))))), ((locals.var_chi_dn9 * assign89530_body52_e137000) + (locals.var_chi * (((locals.var_chi_dn9 / 2.0) * assign89530_body52_e136998) + (assign89530_body52_e136987 * (((locals.var_chi_dn9 / 3.0) * assign89530_body52_e136996) + (assign89530_body52_e136991 * (locals.var_chi_dn9 / 4.0))))))), ((locals.var_chi_dn10 * assign89530_body52_e137000) + (locals.var_chi * (((locals.var_chi_dn10 / 2.0) * assign89530_body52_e136998) + (assign89530_body52_e136987 * (((locals.var_chi_dn10 / 3.0) * assign89530_body52_e136996) + (assign89530_body52_e136991 * (locals.var_chi_dn10 / 4.0))))))), ((locals.var_chi_dn13 * assign89530_body52_e137000) + (locals.var_chi * (((locals.var_chi_dn13 / 2.0) * assign89530_body52_e136998) + (assign89530_body52_e136987 * (((locals.var_chi_dn13 / 3.0) * assign89530_body52_e136996) + (assign89530_body52_e136991 * (locals.var_chi_dn13 / 4.0))))))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
            locals.var_t1 = assign89530_body52_e137003;
            locals.var_t1_dn0 = assign89530_body52_e137003_d_n0;
            locals.var_t1_dn2 = assign89530_body52_e137003_d_n2;
            locals.var_t1_dn4 = assign89530_body52_e137003_d_n4;
            locals.var_t1_dn5 = assign89530_body52_e137003_d_n5;
            locals.var_t1_dn6 = assign89530_body52_e137003_d_n6;
            locals.var_t1_dn7 = assign89530_body52_e137003_d_n7;
            locals.var_t1_dn8 = assign89530_body52_e137003_d_n8;
            locals.var_t1_dn9 = assign89530_body52_e137003_d_n9;
            locals.var_t1_dn10 = assign89530_body52_e137003_d_n10;
            locals.var_t1_dn13 = assign89530_body52_e137003_d_n13;
            locals.var_t1_rv = 0.0;
            let (assign89530_body53_e137021, assign89530_body53_e137021_d_n0, assign89530_body53_e137021_d_n2, assign89530_body53_e137021_d_n4, assign89530_body53_e137021_d_n5, assign89530_body53_e137021_d_n6, assign89530_body53_e137021_d_n7, assign89530_body53_e137021_d_n8, assign89530_body53_e137021_d_n9, assign89530_body53_e137021_d_n10, assign89530_body53_e137021_d_n13,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) && (locals.var_guard2077 == 0.0)) && (locals.var_guard2078 != 0.0)) && (locals.var_guard2079 != 0.0)) {
        let assign89530_body53_e137019: f64 = (locals.var_cfs1 * locals.var_t0);
        (assign89530_body53_e137019, ((locals.var_cfs1_dn0 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn0)), ((locals.var_cfs1_dn2 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn2)), ((locals.var_cfs1_dn4 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn4)), ((locals.var_cfs1_dn5 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn5)), ((locals.var_cfs1_dn6 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn6)), ((locals.var_cfs1_dn7 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn7)), ((locals.var_cfs1_dn8 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn8)), ((locals.var_cfs1_dn9 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn9)), ((locals.var_cfs1_dn10 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn10)), ((locals.var_cfs1_dn13 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn13)),)
    } else {
        (locals.var_fs01, locals.var_fs01_dn0, locals.var_fs01_dn2, locals.var_fs01_dn4, locals.var_fs01_dn5, locals.var_fs01_dn6, locals.var_fs01_dn7, locals.var_fs01_dn8, locals.var_fs01_dn9, locals.var_fs01_dn10, locals.var_fs01_dn13,)
    }
};
            locals.var_fs01 = assign89530_body53_e137021;
            locals.var_fs01_dn0 = assign89530_body53_e137021_d_n0;
            locals.var_fs01_dn2 = assign89530_body53_e137021_d_n2;
            locals.var_fs01_dn4 = assign89530_body53_e137021_d_n4;
            locals.var_fs01_dn5 = assign89530_body53_e137021_d_n5;
            locals.var_fs01_dn6 = assign89530_body53_e137021_d_n6;
            locals.var_fs01_dn7 = assign89530_body53_e137021_d_n7;
            locals.var_fs01_dn8 = assign89530_body53_e137021_d_n8;
            locals.var_fs01_dn9 = assign89530_body53_e137021_d_n9;
            locals.var_fs01_dn10 = assign89530_body53_e137021_d_n10;
            locals.var_fs01_dn13 = assign89530_body53_e137021_d_n13;
            locals.var_fs01_rv = 0.0;
            let (assign89530_body54_e137041, assign89530_body54_e137041_d_n0, assign89530_body54_e137041_d_n2, assign89530_body54_e137041_d_n4, assign89530_body54_e137041_d_n5, assign89530_body54_e137041_d_n6, assign89530_body54_e137041_d_n7, assign89530_body54_e137041_d_n8, assign89530_body54_e137041_d_n9, assign89530_body54_e137041_d_n10, assign89530_body54_e137041_d_n13,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) && (locals.var_guard2077 == 0.0)) && (locals.var_guard2078 != 0.0)) && (locals.var_guard2079 != 0.0)) {
        let assign89530_body54_e137037: f64 = (locals.var_cfs1 * locals.var_t1);
        let assign89530_body54_e137039: f64 = (assign89530_body54_e137037 * locals.var_beta);
        (assign89530_body54_e137039, ((((locals.var_cfs1_dn0 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn0)) * locals.var_beta) + (assign89530_body54_e137037 * locals.var_beta_dn0)), ((((locals.var_cfs1_dn2 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn2)) * locals.var_beta) + (assign89530_body54_e137037 * locals.var_beta_dn2)), ((((locals.var_cfs1_dn4 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn4)) * locals.var_beta) + (assign89530_body54_e137037 * locals.var_beta_dn4)), ((((locals.var_cfs1_dn5 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn5)) * locals.var_beta) + (assign89530_body54_e137037 * locals.var_beta_dn5)), ((((locals.var_cfs1_dn6 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn6)) * locals.var_beta) + (assign89530_body54_e137037 * locals.var_beta_dn6)), ((((locals.var_cfs1_dn7 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn7)) * locals.var_beta) + (assign89530_body54_e137037 * locals.var_beta_dn7)), ((((locals.var_cfs1_dn8 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn8)) * locals.var_beta) + (assign89530_body54_e137037 * locals.var_beta_dn8)), ((((locals.var_cfs1_dn9 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn9)) * locals.var_beta) + (assign89530_body54_e137037 * locals.var_beta_dn9)), ((((locals.var_cfs1_dn10 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn10)) * locals.var_beta) + (assign89530_body54_e137037 * locals.var_beta_dn10)), ((((locals.var_cfs1_dn13 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn13)) * locals.var_beta) + (assign89530_body54_e137037 * locals.var_beta_dn13)),)
    } else {
        (locals.var_fs01_dps0, locals.var_fs01_dps0_dn0, locals.var_fs01_dps0_dn2, locals.var_fs01_dps0_dn4, locals.var_fs01_dps0_dn5, locals.var_fs01_dps0_dn6, locals.var_fs01_dps0_dn7, locals.var_fs01_dps0_dn8, locals.var_fs01_dps0_dn9, locals.var_fs01_dps0_dn10, locals.var_fs01_dps0_dn13,)
    }
};
            locals.var_fs01_dps0 = assign89530_body54_e137041;
            locals.var_fs01_dps0_dn0 = assign89530_body54_e137041_d_n0;
            locals.var_fs01_dps0_dn2 = assign89530_body54_e137041_d_n2;
            locals.var_fs01_dps0_dn4 = assign89530_body54_e137041_d_n4;
            locals.var_fs01_dps0_dn5 = assign89530_body54_e137041_d_n5;
            locals.var_fs01_dps0_dn6 = assign89530_body54_e137041_d_n6;
            locals.var_fs01_dps0_dn7 = assign89530_body54_e137041_d_n7;
            locals.var_fs01_dps0_dn8 = assign89530_body54_e137041_d_n8;
            locals.var_fs01_dps0_dn9 = assign89530_body54_e137041_d_n9;
            locals.var_fs01_dps0_dn10 = assign89530_body54_e137041_d_n10;
            locals.var_fs01_dps0_dn13 = assign89530_body54_e137041_d_n13;
            locals.var_fs01_dps0_rv = 0.0;
            let (assign89530_body55_e137059, assign89530_body55_e137059_d_n0, assign89530_body55_e137059_d_n2, assign89530_body55_e137059_d_n4, assign89530_body55_e137059_d_n5, assign89530_body55_e137059_d_n6, assign89530_body55_e137059_d_n7, assign89530_body55_e137059_d_n8, assign89530_body55_e137059_d_n9, assign89530_body55_e137059_d_n10, assign89530_body55_e137059_d_n13,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) && (locals.var_guard2077 == 0.0)) && (locals.var_guard2078 != 0.0)) && (locals.var_guard2079 == 0.0)) {
        let assign89530_body55_e137057: f64 = (locals.var_chi).exp();
        (assign89530_body55_e137057, (assign89530_body55_e137057 * locals.var_chi_dn0), (assign89530_body55_e137057 * locals.var_chi_dn2), (assign89530_body55_e137057 * locals.var_chi_dn4), (assign89530_body55_e137057 * locals.var_chi_dn5), (assign89530_body55_e137057 * locals.var_chi_dn6), (assign89530_body55_e137057 * locals.var_chi_dn7), (assign89530_body55_e137057 * locals.var_chi_dn8), (assign89530_body55_e137057 * locals.var_chi_dn9), (assign89530_body55_e137057 * locals.var_chi_dn10), (assign89530_body55_e137057 * locals.var_chi_dn13),)
    } else {
        (locals.var_exp_chi, locals.var_exp_chi_dn0, locals.var_exp_chi_dn2, locals.var_exp_chi_dn4, locals.var_exp_chi_dn5, locals.var_exp_chi_dn6, locals.var_exp_chi_dn7, locals.var_exp_chi_dn8, locals.var_exp_chi_dn9, locals.var_exp_chi_dn10, locals.var_exp_chi_dn13,)
    }
};
            locals.var_exp_chi = assign89530_body55_e137059;
            locals.var_exp_chi_dn0 = assign89530_body55_e137059_d_n0;
            locals.var_exp_chi_dn2 = assign89530_body55_e137059_d_n2;
            locals.var_exp_chi_dn4 = assign89530_body55_e137059_d_n4;
            locals.var_exp_chi_dn5 = assign89530_body55_e137059_d_n5;
            locals.var_exp_chi_dn6 = assign89530_body55_e137059_d_n6;
            locals.var_exp_chi_dn7 = assign89530_body55_e137059_d_n7;
            locals.var_exp_chi_dn8 = assign89530_body55_e137059_d_n8;
            locals.var_exp_chi_dn9 = assign89530_body55_e137059_d_n9;
            locals.var_exp_chi_dn10 = assign89530_body55_e137059_d_n10;
            locals.var_exp_chi_dn13 = assign89530_body55_e137059_d_n13;
            locals.var_exp_chi_rv = 0.0;
            let (assign89530_body56_e137078, assign89530_body56_e137078_d_n0, assign89530_body56_e137078_d_n2, assign89530_body56_e137078_d_n4, assign89530_body56_e137078_d_n5, assign89530_body56_e137078_d_n6, assign89530_body56_e137078_d_n7, assign89530_body56_e137078_d_n8, assign89530_body56_e137078_d_n9, assign89530_body56_e137078_d_n10, assign89530_body56_e137078_d_n13,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) && (locals.var_guard2077 == 0.0)) && (locals.var_guard2078 != 0.0)) && (locals.var_guard2079 == 0.0)) {
        let assign89530_body56_e137076: f64 = (locals.var_exp_chi - 1.0);
        (assign89530_body56_e137076, locals.var_exp_chi_dn0, locals.var_exp_chi_dn2, locals.var_exp_chi_dn4, locals.var_exp_chi_dn5, locals.var_exp_chi_dn6, locals.var_exp_chi_dn7, locals.var_exp_chi_dn8, locals.var_exp_chi_dn9, locals.var_exp_chi_dn10, locals.var_exp_chi_dn13,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
            locals.var_t1 = assign89530_body56_e137078;
            locals.var_t1_dn0 = assign89530_body56_e137078_d_n0;
            locals.var_t1_dn2 = assign89530_body56_e137078_d_n2;
            locals.var_t1_dn4 = assign89530_body56_e137078_d_n4;
            locals.var_t1_dn5 = assign89530_body56_e137078_d_n5;
            locals.var_t1_dn6 = assign89530_body56_e137078_d_n6;
            locals.var_t1_dn7 = assign89530_body56_e137078_d_n7;
            locals.var_t1_dn8 = assign89530_body56_e137078_d_n8;
            locals.var_t1_dn9 = assign89530_body56_e137078_d_n9;
            locals.var_t1_dn10 = assign89530_body56_e137078_d_n10;
            locals.var_t1_dn13 = assign89530_body56_e137078_d_n13;
            locals.var_t1_rv = 0.0;
            let (assign89530_body57_e137099, assign89530_body57_e137099_d_n0, assign89530_body57_e137099_d_n2, assign89530_body57_e137099_d_n4, assign89530_body57_e137099_d_n5, assign89530_body57_e137099_d_n6, assign89530_body57_e137099_d_n7, assign89530_body57_e137099_d_n8, assign89530_body57_e137099_d_n9, assign89530_body57_e137099_d_n10, assign89530_body57_e137099_d_n13,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) && (locals.var_guard2077 == 0.0)) && (locals.var_guard2078 != 0.0)) && (locals.var_guard2079 == 0.0)) {
        let assign89530_body57_e137096: f64 = (locals.var_t1 - locals.var_chi);
        let assign89530_body57_e137097: f64 = (locals.var_cfs1 * assign89530_body57_e137096);
        (assign89530_body57_e137097, ((locals.var_cfs1_dn0 * assign89530_body57_e137096) + (locals.var_cfs1 * (locals.var_t1_dn0 - locals.var_chi_dn0))), ((locals.var_cfs1_dn2 * assign89530_body57_e137096) + (locals.var_cfs1 * (locals.var_t1_dn2 - locals.var_chi_dn2))), ((locals.var_cfs1_dn4 * assign89530_body57_e137096) + (locals.var_cfs1 * (locals.var_t1_dn4 - locals.var_chi_dn4))), ((locals.var_cfs1_dn5 * assign89530_body57_e137096) + (locals.var_cfs1 * (locals.var_t1_dn5 - locals.var_chi_dn5))), ((locals.var_cfs1_dn6 * assign89530_body57_e137096) + (locals.var_cfs1 * (locals.var_t1_dn6 - locals.var_chi_dn6))), ((locals.var_cfs1_dn7 * assign89530_body57_e137096) + (locals.var_cfs1 * (locals.var_t1_dn7 - locals.var_chi_dn7))), ((locals.var_cfs1_dn8 * assign89530_body57_e137096) + (locals.var_cfs1 * (locals.var_t1_dn8 - locals.var_chi_dn8))), ((locals.var_cfs1_dn9 * assign89530_body57_e137096) + (locals.var_cfs1 * (locals.var_t1_dn9 - locals.var_chi_dn9))), ((locals.var_cfs1_dn10 * assign89530_body57_e137096) + (locals.var_cfs1 * (locals.var_t1_dn10 - locals.var_chi_dn10))), ((locals.var_cfs1_dn13 * assign89530_body57_e137096) + (locals.var_cfs1 * (locals.var_t1_dn13 - locals.var_chi_dn13))),)
    } else {
        (locals.var_fs01, locals.var_fs01_dn0, locals.var_fs01_dn2, locals.var_fs01_dn4, locals.var_fs01_dn5, locals.var_fs01_dn6, locals.var_fs01_dn7, locals.var_fs01_dn8, locals.var_fs01_dn9, locals.var_fs01_dn10, locals.var_fs01_dn13,)
    }
};
            locals.var_fs01 = assign89530_body57_e137099;
            locals.var_fs01_dn0 = assign89530_body57_e137099_d_n0;
            locals.var_fs01_dn2 = assign89530_body57_e137099_d_n2;
            locals.var_fs01_dn4 = assign89530_body57_e137099_d_n4;
            locals.var_fs01_dn5 = assign89530_body57_e137099_d_n5;
            locals.var_fs01_dn6 = assign89530_body57_e137099_d_n6;
            locals.var_fs01_dn7 = assign89530_body57_e137099_d_n7;
            locals.var_fs01_dn8 = assign89530_body57_e137099_d_n8;
            locals.var_fs01_dn9 = assign89530_body57_e137099_d_n9;
            locals.var_fs01_dn10 = assign89530_body57_e137099_d_n10;
            locals.var_fs01_dn13 = assign89530_body57_e137099_d_n13;
            locals.var_fs01_rv = 0.0;
            let (assign89530_body58_e137120, assign89530_body58_e137120_d_n0, assign89530_body58_e137120_d_n2, assign89530_body58_e137120_d_n4, assign89530_body58_e137120_d_n5, assign89530_body58_e137120_d_n6, assign89530_body58_e137120_d_n7, assign89530_body58_e137120_d_n8, assign89530_body58_e137120_d_n9, assign89530_body58_e137120_d_n10, assign89530_body58_e137120_d_n13,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) && (locals.var_guard2077 == 0.0)) && (locals.var_guard2078 != 0.0)) && (locals.var_guard2079 == 0.0)) {
        let assign89530_body58_e137116: f64 = (locals.var_cfs1 * locals.var_beta);
        let assign89530_body58_e137118: f64 = (assign89530_body58_e137116 * locals.var_t1);
        (assign89530_body58_e137118, ((((locals.var_cfs1_dn0 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn0)) * locals.var_t1) + (assign89530_body58_e137116 * locals.var_t1_dn0)), ((((locals.var_cfs1_dn2 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn2)) * locals.var_t1) + (assign89530_body58_e137116 * locals.var_t1_dn2)), ((((locals.var_cfs1_dn4 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn4)) * locals.var_t1) + (assign89530_body58_e137116 * locals.var_t1_dn4)), ((((locals.var_cfs1_dn5 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn5)) * locals.var_t1) + (assign89530_body58_e137116 * locals.var_t1_dn5)), ((((locals.var_cfs1_dn6 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn6)) * locals.var_t1) + (assign89530_body58_e137116 * locals.var_t1_dn6)), ((((locals.var_cfs1_dn7 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn7)) * locals.var_t1) + (assign89530_body58_e137116 * locals.var_t1_dn7)), ((((locals.var_cfs1_dn8 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn8)) * locals.var_t1) + (assign89530_body58_e137116 * locals.var_t1_dn8)), ((((locals.var_cfs1_dn9 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn9)) * locals.var_t1) + (assign89530_body58_e137116 * locals.var_t1_dn9)), ((((locals.var_cfs1_dn10 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn10)) * locals.var_t1) + (assign89530_body58_e137116 * locals.var_t1_dn10)), ((((locals.var_cfs1_dn13 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn13)) * locals.var_t1) + (assign89530_body58_e137116 * locals.var_t1_dn13)),)
    } else {
        (locals.var_fs01_dps0, locals.var_fs01_dps0_dn0, locals.var_fs01_dps0_dn2, locals.var_fs01_dps0_dn4, locals.var_fs01_dps0_dn5, locals.var_fs01_dps0_dn6, locals.var_fs01_dps0_dn7, locals.var_fs01_dps0_dn8, locals.var_fs01_dps0_dn9, locals.var_fs01_dps0_dn10, locals.var_fs01_dps0_dn13,)
    }
};
            locals.var_fs01_dps0 = assign89530_body58_e137120;
            locals.var_fs01_dps0_dn0 = assign89530_body58_e137120_d_n0;
            locals.var_fs01_dps0_dn2 = assign89530_body58_e137120_d_n2;
            locals.var_fs01_dps0_dn4 = assign89530_body58_e137120_d_n4;
            locals.var_fs01_dps0_dn5 = assign89530_body58_e137120_d_n5;
            locals.var_fs01_dps0_dn6 = assign89530_body58_e137120_d_n6;
            locals.var_fs01_dps0_dn7 = assign89530_body58_e137120_d_n7;
            locals.var_fs01_dps0_dn8 = assign89530_body58_e137120_d_n8;
            locals.var_fs01_dps0_dn9 = assign89530_body58_e137120_d_n9;
            locals.var_fs01_dps0_dn10 = assign89530_body58_e137120_d_n10;
            locals.var_fs01_dps0_dn13 = assign89530_body58_e137120_d_n13;
            locals.var_fs01_dps0_rv = 0.0;
            let (assign89530_body60_e137155, assign89530_body60_e137155_d_n0, assign89530_body60_e137155_d_n2, assign89530_body60_e137155_d_n4, assign89530_body60_e137155_d_n5, assign89530_body60_e137155_d_n6, assign89530_body60_e137155_d_n7, assign89530_body60_e137155_d_n8, assign89530_body60_e137155_d_n9, assign89530_body60_e137155_d_n10, assign89530_body60_e137155_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) && (locals.var_guard2077 == 0.0)) && (locals.var_guard2078 == 0.0)) {
        let assign89530_body60_e137152: f64 = (locals.var_beta * locals.var_ps0ld);
        let assign89530_body60_e137153: f64 = (assign89530_body60_e137152).exp();
        (assign89530_body60_e137153, (assign89530_body60_e137153 * ((locals.var_beta_dn0 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn0))), (assign89530_body60_e137153 * ((locals.var_beta_dn2 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn2))), (assign89530_body60_e137153 * ((locals.var_beta_dn4 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn4))), (assign89530_body60_e137153 * ((locals.var_beta_dn5 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn5))), (assign89530_body60_e137153 * ((locals.var_beta_dn6 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn6))), (assign89530_body60_e137153 * ((locals.var_beta_dn7 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn7))), (assign89530_body60_e137153 * ((locals.var_beta_dn8 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn8))), (assign89530_body60_e137153 * ((locals.var_beta_dn9 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn9))), (assign89530_body60_e137153 * ((locals.var_beta_dn10 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn10))), (assign89530_body60_e137153 * ((locals.var_beta_dn13 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn13))),)
    } else {
        (locals.var_exp_bps0, locals.var_exp_bps0_dn0, locals.var_exp_bps0_dn2, locals.var_exp_bps0_dn4, locals.var_exp_bps0_dn5, locals.var_exp_bps0_dn6, locals.var_exp_bps0_dn7, locals.var_exp_bps0_dn8, locals.var_exp_bps0_dn9, locals.var_exp_bps0_dn10, locals.var_exp_bps0_dn13,)
    }
};
            locals.var_exp_bps0 = assign89530_body60_e137155;
            locals.var_exp_bps0_dn0 = assign89530_body60_e137155_d_n0;
            locals.var_exp_bps0_dn2 = assign89530_body60_e137155_d_n2;
            locals.var_exp_bps0_dn4 = assign89530_body60_e137155_d_n4;
            locals.var_exp_bps0_dn5 = assign89530_body60_e137155_d_n5;
            locals.var_exp_bps0_dn6 = assign89530_body60_e137155_d_n6;
            locals.var_exp_bps0_dn7 = assign89530_body60_e137155_d_n7;
            locals.var_exp_bps0_dn8 = assign89530_body60_e137155_d_n8;
            locals.var_exp_bps0_dn9 = assign89530_body60_e137155_d_n9;
            locals.var_exp_bps0_dn10 = assign89530_body60_e137155_d_n10;
            locals.var_exp_bps0_dn13 = assign89530_body60_e137155_d_n13;
            locals.var_exp_bps0_rv = 0.0;
            let (assign89530_body61_e137178, assign89530_body61_e137178_d_n0, assign89530_body61_e137178_d_n2, assign89530_body61_e137178_d_n4, assign89530_body61_e137178_d_n5, assign89530_body61_e137178_d_n6, assign89530_body61_e137178_d_n7, assign89530_body61_e137178_d_n8, assign89530_body61_e137178_d_n9, assign89530_body61_e137178_d_n10, assign89530_body61_e137178_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) && (locals.var_guard2077 == 0.0)) && (locals.var_guard2078 == 0.0)) {
        let assign89530_body61_e137173: f64 = (locals.var_chi + 1.0);
        let assign89530_body61_e137174: f64 = (locals.var_exp_bvbs * assign89530_body61_e137173);
        let assign89530_body61_e137175: f64 = (locals.var_exp_bps0 - assign89530_body61_e137174);
        let assign89530_body61_e137176: f64 = (locals.var_cnst1over * assign89530_body61_e137175);
        (assign89530_body61_e137176, ((locals.var_cnst1over_dn0 * assign89530_body61_e137175) + (locals.var_cnst1over * (locals.var_exp_bps0_dn0 - ((locals.var_exp_bvbs_dn0 * assign89530_body61_e137173) + (locals.var_exp_bvbs * locals.var_chi_dn0))))), ((locals.var_cnst1over_dn2 * assign89530_body61_e137175) + (locals.var_cnst1over * (locals.var_exp_bps0_dn2 - ((locals.var_exp_bvbs_dn2 * assign89530_body61_e137173) + (locals.var_exp_bvbs * locals.var_chi_dn2))))), ((locals.var_cnst1over_dn4 * assign89530_body61_e137175) + (locals.var_cnst1over * (locals.var_exp_bps0_dn4 - ((locals.var_exp_bvbs_dn4 * assign89530_body61_e137173) + (locals.var_exp_bvbs * locals.var_chi_dn4))))), ((locals.var_cnst1over_dn5 * assign89530_body61_e137175) + (locals.var_cnst1over * (locals.var_exp_bps0_dn5 - ((locals.var_exp_bvbs_dn5 * assign89530_body61_e137173) + (locals.var_exp_bvbs * locals.var_chi_dn5))))), ((locals.var_cnst1over_dn6 * assign89530_body61_e137175) + (locals.var_cnst1over * (locals.var_exp_bps0_dn6 - ((locals.var_exp_bvbs_dn6 * assign89530_body61_e137173) + (locals.var_exp_bvbs * locals.var_chi_dn6))))), ((locals.var_cnst1over_dn7 * assign89530_body61_e137175) + (locals.var_cnst1over * (locals.var_exp_bps0_dn7 - ((locals.var_exp_bvbs_dn7 * assign89530_body61_e137173) + (locals.var_exp_bvbs * locals.var_chi_dn7))))), ((locals.var_cnst1over_dn8 * assign89530_body61_e137175) + (locals.var_cnst1over * (locals.var_exp_bps0_dn8 - ((locals.var_exp_bvbs_dn8 * assign89530_body61_e137173) + (locals.var_exp_bvbs * locals.var_chi_dn8))))), ((locals.var_cnst1over_dn9 * assign89530_body61_e137175) + (locals.var_cnst1over * (locals.var_exp_bps0_dn9 - ((locals.var_exp_bvbs_dn9 * assign89530_body61_e137173) + (locals.var_exp_bvbs * locals.var_chi_dn9))))), ((locals.var_cnst1over_dn10 * assign89530_body61_e137175) + (locals.var_cnst1over * (locals.var_exp_bps0_dn10 - ((locals.var_exp_bvbs_dn10 * assign89530_body61_e137173) + (locals.var_exp_bvbs * locals.var_chi_dn10))))), ((locals.var_cnst1over_dn13 * assign89530_body61_e137175) + (locals.var_cnst1over * (locals.var_exp_bps0_dn13 - ((locals.var_exp_bvbs_dn13 * assign89530_body61_e137173) + (locals.var_exp_bvbs * locals.var_chi_dn13))))),)
    } else {
        (locals.var_fs01, locals.var_fs01_dn0, locals.var_fs01_dn2, locals.var_fs01_dn4, locals.var_fs01_dn5, locals.var_fs01_dn6, locals.var_fs01_dn7, locals.var_fs01_dn8, locals.var_fs01_dn9, locals.var_fs01_dn10, locals.var_fs01_dn13,)
    }
};
            locals.var_fs01 = assign89530_body61_e137178;
            locals.var_fs01_dn0 = assign89530_body61_e137178_d_n0;
            locals.var_fs01_dn2 = assign89530_body61_e137178_d_n2;
            locals.var_fs01_dn4 = assign89530_body61_e137178_d_n4;
            locals.var_fs01_dn5 = assign89530_body61_e137178_d_n5;
            locals.var_fs01_dn6 = assign89530_body61_e137178_d_n6;
            locals.var_fs01_dn7 = assign89530_body61_e137178_d_n7;
            locals.var_fs01_dn8 = assign89530_body61_e137178_d_n8;
            locals.var_fs01_dn9 = assign89530_body61_e137178_d_n9;
            locals.var_fs01_dn10 = assign89530_body61_e137178_d_n10;
            locals.var_fs01_dn13 = assign89530_body61_e137178_d_n13;
            locals.var_fs01_rv = 0.0;
            let (assign89530_body62_e137199, assign89530_body62_e137199_d_n0, assign89530_body62_e137199_d_n2, assign89530_body62_e137199_d_n4, assign89530_body62_e137199_d_n5, assign89530_body62_e137199_d_n6, assign89530_body62_e137199_d_n7, assign89530_body62_e137199_d_n8, assign89530_body62_e137199_d_n9, assign89530_body62_e137199_d_n10, assign89530_body62_e137199_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) && (locals.var_guard2077 == 0.0)) && (locals.var_guard2078 == 0.0)) {
        let assign89530_body62_e137193: f64 = (locals.var_cnst1over * locals.var_beta);
        let assign89530_body62_e137196: f64 = (locals.var_exp_bps0 - locals.var_exp_bvbs);
        let assign89530_body62_e137197: f64 = (assign89530_body62_e137193 * assign89530_body62_e137196);
        (assign89530_body62_e137197, ((((locals.var_cnst1over_dn0 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn0)) * assign89530_body62_e137196) + (assign89530_body62_e137193 * (locals.var_exp_bps0_dn0 - locals.var_exp_bvbs_dn0))), ((((locals.var_cnst1over_dn2 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn2)) * assign89530_body62_e137196) + (assign89530_body62_e137193 * (locals.var_exp_bps0_dn2 - locals.var_exp_bvbs_dn2))), ((((locals.var_cnst1over_dn4 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn4)) * assign89530_body62_e137196) + (assign89530_body62_e137193 * (locals.var_exp_bps0_dn4 - locals.var_exp_bvbs_dn4))), ((((locals.var_cnst1over_dn5 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn5)) * assign89530_body62_e137196) + (assign89530_body62_e137193 * (locals.var_exp_bps0_dn5 - locals.var_exp_bvbs_dn5))), ((((locals.var_cnst1over_dn6 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn6)) * assign89530_body62_e137196) + (assign89530_body62_e137193 * (locals.var_exp_bps0_dn6 - locals.var_exp_bvbs_dn6))), ((((locals.var_cnst1over_dn7 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn7)) * assign89530_body62_e137196) + (assign89530_body62_e137193 * (locals.var_exp_bps0_dn7 - locals.var_exp_bvbs_dn7))), ((((locals.var_cnst1over_dn8 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn8)) * assign89530_body62_e137196) + (assign89530_body62_e137193 * (locals.var_exp_bps0_dn8 - locals.var_exp_bvbs_dn8))), ((((locals.var_cnst1over_dn9 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn9)) * assign89530_body62_e137196) + (assign89530_body62_e137193 * (locals.var_exp_bps0_dn9 - locals.var_exp_bvbs_dn9))), ((((locals.var_cnst1over_dn10 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn10)) * assign89530_body62_e137196) + (assign89530_body62_e137193 * (locals.var_exp_bps0_dn10 - locals.var_exp_bvbs_dn10))), ((((locals.var_cnst1over_dn13 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn13)) * assign89530_body62_e137196) + (assign89530_body62_e137193 * (locals.var_exp_bps0_dn13 - locals.var_exp_bvbs_dn13))),)
    } else {
        (locals.var_fs01_dps0, locals.var_fs01_dps0_dn0, locals.var_fs01_dps0_dn2, locals.var_fs01_dps0_dn4, locals.var_fs01_dps0_dn5, locals.var_fs01_dps0_dn6, locals.var_fs01_dps0_dn7, locals.var_fs01_dps0_dn8, locals.var_fs01_dps0_dn9, locals.var_fs01_dps0_dn10, locals.var_fs01_dps0_dn13,)
    }
};
            locals.var_fs01_dps0 = assign89530_body62_e137199;
            locals.var_fs01_dps0_dn0 = assign89530_body62_e137199_d_n0;
            locals.var_fs01_dps0_dn2 = assign89530_body62_e137199_d_n2;
            locals.var_fs01_dps0_dn4 = assign89530_body62_e137199_d_n4;
            locals.var_fs01_dps0_dn5 = assign89530_body62_e137199_d_n5;
            locals.var_fs01_dps0_dn6 = assign89530_body62_e137199_d_n6;
            locals.var_fs01_dps0_dn7 = assign89530_body62_e137199_d_n7;
            locals.var_fs01_dps0_dn8 = assign89530_body62_e137199_d_n8;
            locals.var_fs01_dps0_dn9 = assign89530_body62_e137199_d_n9;
            locals.var_fs01_dps0_dn10 = assign89530_body62_e137199_d_n10;
            locals.var_fs01_dps0_dn13 = assign89530_body62_e137199_d_n13;
            locals.var_fs01_dps0_rv = 0.0;
            let assign89530_body63_e137202: f64 = if locals.var_fs01 > 0.0 { 1.0 } else { 0.0 };
            locals.var_guard2080 = assign89530_body63_e137202;
            locals.var_guard2080_rv = 0.0;
            let (assign89530_body64_e137221, assign89530_body64_e137221_d_n0, assign89530_body64_e137221_d_n2, assign89530_body64_e137221_d_n4, assign89530_body64_e137221_d_n5, assign89530_body64_e137221_d_n6, assign89530_body64_e137221_d_n7, assign89530_body64_e137221_d_n8, assign89530_body64_e137221_d_n9, assign89530_body64_e137221_d_n10, assign89530_body64_e137221_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) && (locals.var_guard2077 == 0.0)) && (locals.var_guard2080 != 0.0)) {
        let assign89530_body64_e137216: f64 = (locals.var_fb * locals.var_fb);
        let assign89530_body64_e137218: f64 = (assign89530_body64_e137216 + locals.var_fs01);
        let assign89530_body64_e137219: f64 = (assign89530_body64_e137218).sqrt();
        (assign89530_body64_e137219, ((((locals.var_fb_dn0 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn0)) + locals.var_fs01_dn0) / (2.0 * assign89530_body64_e137219)), ((((locals.var_fb_dn2 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn2)) + locals.var_fs01_dn2) / (2.0 * assign89530_body64_e137219)), ((((locals.var_fb_dn4 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn4)) + locals.var_fs01_dn4) / (2.0 * assign89530_body64_e137219)), ((((locals.var_fb_dn5 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn5)) + locals.var_fs01_dn5) / (2.0 * assign89530_body64_e137219)), ((((locals.var_fb_dn6 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn6)) + locals.var_fs01_dn6) / (2.0 * assign89530_body64_e137219)), ((((locals.var_fb_dn7 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn7)) + locals.var_fs01_dn7) / (2.0 * assign89530_body64_e137219)), ((((locals.var_fb_dn8 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn8)) + locals.var_fs01_dn8) / (2.0 * assign89530_body64_e137219)), ((((locals.var_fb_dn9 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn9)) + locals.var_fs01_dn9) / (2.0 * assign89530_body64_e137219)), ((((locals.var_fb_dn10 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn10)) + locals.var_fs01_dn10) / (2.0 * assign89530_body64_e137219)), ((((locals.var_fb_dn13 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn13)) + locals.var_fs01_dn13) / (2.0 * assign89530_body64_e137219)),)
    } else {
        (locals.var_fs02, locals.var_fs02_dn0, locals.var_fs02_dn2, locals.var_fs02_dn4, locals.var_fs02_dn5, locals.var_fs02_dn6, locals.var_fs02_dn7, locals.var_fs02_dn8, locals.var_fs02_dn9, locals.var_fs02_dn10, locals.var_fs02_dn13,)
    }
};
            locals.var_fs02 = assign89530_body64_e137221;
            locals.var_fs02_dn0 = assign89530_body64_e137221_d_n0;
            locals.var_fs02_dn2 = assign89530_body64_e137221_d_n2;
            locals.var_fs02_dn4 = assign89530_body64_e137221_d_n4;
            locals.var_fs02_dn5 = assign89530_body64_e137221_d_n5;
            locals.var_fs02_dn6 = assign89530_body64_e137221_d_n6;
            locals.var_fs02_dn7 = assign89530_body64_e137221_d_n7;
            locals.var_fs02_dn8 = assign89530_body64_e137221_d_n8;
            locals.var_fs02_dn9 = assign89530_body64_e137221_d_n9;
            locals.var_fs02_dn10 = assign89530_body64_e137221_d_n10;
            locals.var_fs02_dn13 = assign89530_body64_e137221_d_n13;
            locals.var_fs02_rv = 0.0;
            let (assign89530_body65_e137245, assign89530_body65_e137245_d_n0, assign89530_body65_e137245_d_n2, assign89530_body65_e137245_d_n4, assign89530_body65_e137245_d_n5, assign89530_body65_e137245_d_n6, assign89530_body65_e137245_d_n7, assign89530_body65_e137245_d_n8, assign89530_body65_e137245_d_n9, assign89530_body65_e137245_d_n10, assign89530_body65_e137245_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) && (locals.var_guard2077 == 0.0)) && (locals.var_guard2080 != 0.0)) {
        let assign89530_body65_e137236: f64 = (2.0 * locals.var_fb_dpss);
        let assign89530_body65_e137238: f64 = (assign89530_body65_e137236 * locals.var_fb);
        let assign89530_body65_e137240: f64 = (assign89530_body65_e137238 + locals.var_fs01_dps0);
        let assign89530_body65_e137241: f64 = (0.5 * assign89530_body65_e137240);
        let assign89530_body65_e137243: f64 = (assign89530_body65_e137241 / locals.var_fs02);
        (assign89530_body65_e137243, ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn0) * locals.var_fb) + (assign89530_body65_e137236 * locals.var_fb_dn0)) + locals.var_fs01_dps0_dn0)) * locals.var_fs02) - (assign89530_body65_e137241 * locals.var_fs02_dn0)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn2) * locals.var_fb) + (assign89530_body65_e137236 * locals.var_fb_dn2)) + locals.var_fs01_dps0_dn2)) * locals.var_fs02) - (assign89530_body65_e137241 * locals.var_fs02_dn2)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn4) * locals.var_fb) + (assign89530_body65_e137236 * locals.var_fb_dn4)) + locals.var_fs01_dps0_dn4)) * locals.var_fs02) - (assign89530_body65_e137241 * locals.var_fs02_dn4)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn5) * locals.var_fb) + (assign89530_body65_e137236 * locals.var_fb_dn5)) + locals.var_fs01_dps0_dn5)) * locals.var_fs02) - (assign89530_body65_e137241 * locals.var_fs02_dn5)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn6) * locals.var_fb) + (assign89530_body65_e137236 * locals.var_fb_dn6)) + locals.var_fs01_dps0_dn6)) * locals.var_fs02) - (assign89530_body65_e137241 * locals.var_fs02_dn6)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn7) * locals.var_fb) + (assign89530_body65_e137236 * locals.var_fb_dn7)) + locals.var_fs01_dps0_dn7)) * locals.var_fs02) - (assign89530_body65_e137241 * locals.var_fs02_dn7)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn8) * locals.var_fb) + (assign89530_body65_e137236 * locals.var_fb_dn8)) + locals.var_fs01_dps0_dn8)) * locals.var_fs02) - (assign89530_body65_e137241 * locals.var_fs02_dn8)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn9) * locals.var_fb) + (assign89530_body65_e137236 * locals.var_fb_dn9)) + locals.var_fs01_dps0_dn9)) * locals.var_fs02) - (assign89530_body65_e137241 * locals.var_fs02_dn9)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn10) * locals.var_fb) + (assign89530_body65_e137236 * locals.var_fb_dn10)) + locals.var_fs01_dps0_dn10)) * locals.var_fs02) - (assign89530_body65_e137241 * locals.var_fs02_dn10)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn13) * locals.var_fb) + (assign89530_body65_e137236 * locals.var_fb_dn13)) + locals.var_fs01_dps0_dn13)) * locals.var_fs02) - (assign89530_body65_e137241 * locals.var_fs02_dn13)) / (locals.var_fs02 * locals.var_fs02)),)
    } else {
        (locals.var_fs02_dps0, locals.var_fs02_dps0_dn0, locals.var_fs02_dps0_dn2, locals.var_fs02_dps0_dn4, locals.var_fs02_dps0_dn5, locals.var_fs02_dps0_dn6, locals.var_fs02_dps0_dn7, locals.var_fs02_dps0_dn8, locals.var_fs02_dps0_dn9, locals.var_fs02_dps0_dn10, locals.var_fs02_dps0_dn13,)
    }
};
            locals.var_fs02_dps0 = assign89530_body65_e137245;
            locals.var_fs02_dps0_dn0 = assign89530_body65_e137245_d_n0;
            locals.var_fs02_dps0_dn2 = assign89530_body65_e137245_d_n2;
            locals.var_fs02_dps0_dn4 = assign89530_body65_e137245_d_n4;
            locals.var_fs02_dps0_dn5 = assign89530_body65_e137245_d_n5;
            locals.var_fs02_dps0_dn6 = assign89530_body65_e137245_d_n6;
            locals.var_fs02_dps0_dn7 = assign89530_body65_e137245_d_n7;
            locals.var_fs02_dps0_dn8 = assign89530_body65_e137245_d_n8;
            locals.var_fs02_dps0_dn9 = assign89530_body65_e137245_d_n9;
            locals.var_fs02_dps0_dn10 = assign89530_body65_e137245_d_n10;
            locals.var_fs02_dps0_dn13 = assign89530_body65_e137245_d_n13;
            locals.var_fs02_dps0_rv = 0.0;
            let (assign89530_body67_e137277, assign89530_body67_e137277_d_n0, assign89530_body67_e137277_d_n2, assign89530_body67_e137277_d_n4, assign89530_body67_e137277_d_n5, assign89530_body67_e137277_d_n6, assign89530_body67_e137277_d_n7, assign89530_body67_e137277_d_n8, assign89530_body67_e137277_d_n9, assign89530_body67_e137277_d_n10, assign89530_body67_e137277_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) && (locals.var_guard2077 == 0.0)) && (locals.var_guard2080 == 0.0)) {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn4, locals.var_fb_dn5, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn8, locals.var_fb_dn9, locals.var_fb_dn10, locals.var_fb_dn13,)
    } else {
        (locals.var_fs02, locals.var_fs02_dn0, locals.var_fs02_dn2, locals.var_fs02_dn4, locals.var_fs02_dn5, locals.var_fs02_dn6, locals.var_fs02_dn7, locals.var_fs02_dn8, locals.var_fs02_dn9, locals.var_fs02_dn10, locals.var_fs02_dn13,)
    }
};
            locals.var_fs02 = assign89530_body67_e137277;
            locals.var_fs02_dn0 = assign89530_body67_e137277_d_n0;
            locals.var_fs02_dn2 = assign89530_body67_e137277_d_n2;
            locals.var_fs02_dn4 = assign89530_body67_e137277_d_n4;
            locals.var_fs02_dn5 = assign89530_body67_e137277_d_n5;
            locals.var_fs02_dn6 = assign89530_body67_e137277_d_n6;
            locals.var_fs02_dn7 = assign89530_body67_e137277_d_n7;
            locals.var_fs02_dn8 = assign89530_body67_e137277_d_n8;
            locals.var_fs02_dn9 = assign89530_body67_e137277_d_n9;
            locals.var_fs02_dn10 = assign89530_body67_e137277_d_n10;
            locals.var_fs02_dn13 = assign89530_body67_e137277_d_n13;
            locals.var_fs02_rv = 0.0;
            let (assign89530_body68_e137292, assign89530_body68_e137292_d_n0, assign89530_body68_e137292_d_n2, assign89530_body68_e137292_d_n4, assign89530_body68_e137292_d_n5, assign89530_body68_e137292_d_n6, assign89530_body68_e137292_d_n7, assign89530_body68_e137292_d_n8, assign89530_body68_e137292_d_n9, assign89530_body68_e137292_d_n10, assign89530_body68_e137292_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) && (locals.var_guard2077 == 0.0)) && (locals.var_guard2080 == 0.0)) {
        (locals.var_fb_dpss, locals.var_fb_dpss_dn0, locals.var_fb_dpss_dn2, locals.var_fb_dpss_dn4, locals.var_fb_dpss_dn5, locals.var_fb_dpss_dn6, locals.var_fb_dpss_dn7, locals.var_fb_dpss_dn8, locals.var_fb_dpss_dn9, locals.var_fb_dpss_dn10, locals.var_fb_dpss_dn13,)
    } else {
        (locals.var_fs02_dps0, locals.var_fs02_dps0_dn0, locals.var_fs02_dps0_dn2, locals.var_fs02_dps0_dn4, locals.var_fs02_dps0_dn5, locals.var_fs02_dps0_dn6, locals.var_fs02_dps0_dn7, locals.var_fs02_dps0_dn8, locals.var_fs02_dps0_dn9, locals.var_fs02_dps0_dn10, locals.var_fs02_dps0_dn13,)
    }
};
            locals.var_fs02_dps0 = assign89530_body68_e137292;
            locals.var_fs02_dps0_dn0 = assign89530_body68_e137292_d_n0;
            locals.var_fs02_dps0_dn2 = assign89530_body68_e137292_d_n2;
            locals.var_fs02_dps0_dn4 = assign89530_body68_e137292_d_n4;
            locals.var_fs02_dps0_dn5 = assign89530_body68_e137292_d_n5;
            locals.var_fs02_dps0_dn6 = assign89530_body68_e137292_d_n6;
            locals.var_fs02_dps0_dn7 = assign89530_body68_e137292_d_n7;
            locals.var_fs02_dps0_dn8 = assign89530_body68_e137292_d_n8;
            locals.var_fs02_dps0_dn9 = assign89530_body68_e137292_d_n9;
            locals.var_fs02_dps0_dn10 = assign89530_body68_e137292_d_n10;
            locals.var_fs02_dps0_dn13 = assign89530_body68_e137292_d_n13;
            locals.var_fs02_dps0_rv = 0.0;
            let (assign89530_body69_e137308, assign89530_body69_e137308_d_n0, assign89530_body69_e137308_d_n2, assign89530_body69_e137308_d_n4, assign89530_body69_e137308_d_n5, assign89530_body69_e137308_d_n6, assign89530_body69_e137308_d_n7, assign89530_body69_e137308_d_n8, assign89530_body69_e137308_d_n9, assign89530_body69_e137308_d_n10, assign89530_body69_e137308_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) {
        let assign89530_body69_e137300: f64 = (-locals.var_vgpld);
        let assign89530_body69_e137302: f64 = (assign89530_body69_e137300 + locals.var_ps0ld);
        let assign89530_body69_e137305: f64 = (locals.var_fac1 * locals.var_fs02);
        let assign89530_body69_e137306: f64 = (assign89530_body69_e137302 + assign89530_body69_e137305);
        (assign89530_body69_e137306, (locals.var_ps0ld_dn0 + ((locals.var_fac1_dn0 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn0))), (((-locals.var_vgpld_dn2) + locals.var_ps0ld_dn2) + ((locals.var_fac1_dn2 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn2))), (locals.var_ps0ld_dn4 + ((locals.var_fac1_dn4 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn4))), (locals.var_ps0ld_dn5 + ((locals.var_fac1_dn5 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn5))), (((-locals.var_vgpld_dn6) + locals.var_ps0ld_dn6) + ((locals.var_fac1_dn6 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn6))), (((-locals.var_vgpld_dn7) + locals.var_ps0ld_dn7) + ((locals.var_fac1_dn7 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn7))), (((-locals.var_vgpld_dn8) + locals.var_ps0ld_dn8) + ((locals.var_fac1_dn8 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn8))), (locals.var_ps0ld_dn9 + ((locals.var_fac1_dn9 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn9))), (locals.var_ps0ld_dn10 + ((locals.var_fac1_dn10 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn10))), (locals.var_ps0ld_dn13 + ((locals.var_fac1_dn13 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn13))),)
    } else {
        (locals.var_fs0, locals.var_fs0_dn0, locals.var_fs0_dn2, locals.var_fs0_dn4, locals.var_fs0_dn5, locals.var_fs0_dn6, locals.var_fs0_dn7, locals.var_fs0_dn8, locals.var_fs0_dn9, locals.var_fs0_dn10, locals.var_fs0_dn13,)
    }
};
            locals.var_fs0 = assign89530_body69_e137308;
            locals.var_fs0_dn0 = assign89530_body69_e137308_d_n0;
            locals.var_fs0_dn2 = assign89530_body69_e137308_d_n2;
            locals.var_fs0_dn4 = assign89530_body69_e137308_d_n4;
            locals.var_fs0_dn5 = assign89530_body69_e137308_d_n5;
            locals.var_fs0_dn6 = assign89530_body69_e137308_d_n6;
            locals.var_fs0_dn7 = assign89530_body69_e137308_d_n7;
            locals.var_fs0_dn8 = assign89530_body69_e137308_d_n8;
            locals.var_fs0_dn9 = assign89530_body69_e137308_d_n9;
            locals.var_fs0_dn10 = assign89530_body69_e137308_d_n10;
            locals.var_fs0_dn13 = assign89530_body69_e137308_d_n13;
            locals.var_fs0_rv = 0.0;
            let (assign89530_body70_e137321, assign89530_body70_e137321_d_n0, assign89530_body70_e137321_d_n2, assign89530_body70_e137321_d_n4, assign89530_body70_e137321_d_n5, assign89530_body70_e137321_d_n6, assign89530_body70_e137321_d_n7, assign89530_body70_e137321_d_n8, assign89530_body70_e137321_d_n9, assign89530_body70_e137321_d_n10, assign89530_body70_e137321_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) {
        let assign89530_body70_e137318: f64 = (locals.var_fac1 * locals.var_fs02_dps0);
        let assign89530_body70_e137319: f64 = (1.0 + assign89530_body70_e137318);
        (assign89530_body70_e137319, ((locals.var_fac1_dn0 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn0)), ((locals.var_fac1_dn2 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn2)), ((locals.var_fac1_dn4 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn4)), ((locals.var_fac1_dn5 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn5)), ((locals.var_fac1_dn6 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn6)), ((locals.var_fac1_dn7 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn7)), ((locals.var_fac1_dn8 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn8)), ((locals.var_fac1_dn9 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn9)), ((locals.var_fac1_dn10 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn10)), ((locals.var_fac1_dn13 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn13)),)
    } else {
        (locals.var_fs0_dps0, locals.var_fs0_dps0_dn0, locals.var_fs0_dps0_dn2, locals.var_fs0_dps0_dn4, locals.var_fs0_dps0_dn5, locals.var_fs0_dps0_dn6, locals.var_fs0_dps0_dn7, locals.var_fs0_dps0_dn8, locals.var_fs0_dps0_dn9, locals.var_fs0_dps0_dn10, locals.var_fs0_dps0_dn13,)
    }
};
            locals.var_fs0_dps0 = assign89530_body70_e137321;
            locals.var_fs0_dps0_dn0 = assign89530_body70_e137321_d_n0;
            locals.var_fs0_dps0_dn2 = assign89530_body70_e137321_d_n2;
            locals.var_fs0_dps0_dn4 = assign89530_body70_e137321_d_n4;
            locals.var_fs0_dps0_dn5 = assign89530_body70_e137321_d_n5;
            locals.var_fs0_dps0_dn6 = assign89530_body70_e137321_d_n6;
            locals.var_fs0_dps0_dn7 = assign89530_body70_e137321_d_n7;
            locals.var_fs0_dps0_dn8 = assign89530_body70_e137321_d_n8;
            locals.var_fs0_dps0_dn9 = assign89530_body70_e137321_d_n9;
            locals.var_fs0_dps0_dn10 = assign89530_body70_e137321_d_n10;
            locals.var_fs0_dps0_dn13 = assign89530_body70_e137321_d_n13;
            locals.var_fs0_dps0_rv = 0.0;
            let assign89530_body71_e137324: f64 = if locals.var_flg_conv == 1.0 { 1.0 } else { 0.0 };
            locals.var_guard2081 = assign89530_body71_e137324;
            locals.var_guard2081_rv = 0.0;
            let (assign89530_body72_e137337,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) && (locals.var_guard2081 != 0.0)) {
        let assign89530_body72_e137335: f64 = (locals.var_lp_s0_max + 1.0);
        (assign89530_body72_e137335,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign89530_body72_e137337;
            locals.var_lp_s0_rv = 0.0;
            let (assign89530_body73_e137352, assign89530_body73_e137352_d_n0, assign89530_body73_e137352_d_n2, assign89530_body73_e137352_d_n4, assign89530_body73_e137352_d_n5, assign89530_body73_e137352_d_n6, assign89530_body73_e137352_d_n7, assign89530_body73_e137352_d_n8, assign89530_body73_e137352_d_n9, assign89530_body73_e137352_d_n10, assign89530_body73_e137352_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) && (locals.var_guard2081 == 0.0)) {
        let assign89530_body73_e137348: f64 = (-locals.var_fs0);
        let assign89530_body73_e137350: f64 = (assign89530_body73_e137348 / locals.var_fs0_dps0);
        (assign89530_body73_e137350, ((((-locals.var_fs0_dn0) * locals.var_fs0_dps0) - (assign89530_body73_e137348 * locals.var_fs0_dps0_dn0)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn2) * locals.var_fs0_dps0) - (assign89530_body73_e137348 * locals.var_fs0_dps0_dn2)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn4) * locals.var_fs0_dps0) - (assign89530_body73_e137348 * locals.var_fs0_dps0_dn4)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn5) * locals.var_fs0_dps0) - (assign89530_body73_e137348 * locals.var_fs0_dps0_dn5)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn6) * locals.var_fs0_dps0) - (assign89530_body73_e137348 * locals.var_fs0_dps0_dn6)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn7) * locals.var_fs0_dps0) - (assign89530_body73_e137348 * locals.var_fs0_dps0_dn7)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn8) * locals.var_fs0_dps0) - (assign89530_body73_e137348 * locals.var_fs0_dps0_dn8)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn9) * locals.var_fs0_dps0) - (assign89530_body73_e137348 * locals.var_fs0_dps0_dn9)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn10) * locals.var_fs0_dps0) - (assign89530_body73_e137348 * locals.var_fs0_dps0_dn10)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn13) * locals.var_fs0_dps0) - (assign89530_body73_e137348 * locals.var_fs0_dps0_dn13)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)),)
    } else {
        (locals.var_dps0, locals.var_dps0_dn0, locals.var_dps0_dn2, locals.var_dps0_dn4, locals.var_dps0_dn5, locals.var_dps0_dn6, locals.var_dps0_dn7, locals.var_dps0_dn8, locals.var_dps0_dn9, locals.var_dps0_dn10, locals.var_dps0_dn13,)
    }
};
            locals.var_dps0 = assign89530_body73_e137352;
            locals.var_dps0_dn0 = assign89530_body73_e137352_d_n0;
            locals.var_dps0_dn2 = assign89530_body73_e137352_d_n2;
            locals.var_dps0_dn4 = assign89530_body73_e137352_d_n4;
            locals.var_dps0_dn5 = assign89530_body73_e137352_d_n5;
            locals.var_dps0_dn6 = assign89530_body73_e137352_d_n6;
            locals.var_dps0_dn7 = assign89530_body73_e137352_d_n7;
            locals.var_dps0_dn8 = assign89530_body73_e137352_d_n8;
            locals.var_dps0_dn9 = assign89530_body73_e137352_d_n9;
            locals.var_dps0_dn10 = assign89530_body73_e137352_d_n10;
            locals.var_dps0_dn13 = assign89530_body73_e137352_d_n13;
            locals.var_dps0_rv = 0.0;
            let (assign89530_body74_e137377, assign89530_body74_e137377_d_n0, assign89530_body74_e137377_d_n2, assign89530_body74_e137377_d_n4, assign89530_body74_e137377_d_n5, assign89530_body74_e137377_d_n6, assign89530_body74_e137377_d_n7, assign89530_body74_e137377_d_n8, assign89530_body74_e137377_d_n9, assign89530_body74_e137377_d_n10, assign89530_body74_e137377_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) && (locals.var_guard2081 == 0.0)) {
        let assign89530_body74_e137364: f64 = (0.5 * 0.1);
        let assign89530_body74_e137368: f64 = (locals.var_ps0ld).abs();
        let (assign89530_body74_e137373, assign89530_body74_e137373_d_n0, assign89530_body74_e137373_d_n2, assign89530_body74_e137373_d_n4, assign89530_body74_e137373_d_n5, assign89530_body74_e137373_d_n6, assign89530_body74_e137373_d_n7, assign89530_body74_e137373_d_n8, assign89530_body74_e137373_d_n9, assign89530_body74_e137373_d_n10, assign89530_body74_e137373_d_n13,) = {
            if (1.0 >= assign89530_body74_e137368) {
                (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign89530_body74_e137372: f64 = (locals.var_ps0ld).abs();
                (assign89530_body74_e137372, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn0 } else { (-locals.var_ps0ld_dn0) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn2 } else { (-locals.var_ps0ld_dn2) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn4 } else { (-locals.var_ps0ld_dn4) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn5 } else { (-locals.var_ps0ld_dn5) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn6 } else { (-locals.var_ps0ld_dn6) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn7 } else { (-locals.var_ps0ld_dn7) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn8 } else { (-locals.var_ps0ld_dn8) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn9 } else { (-locals.var_ps0ld_dn9) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn10 } else { (-locals.var_ps0ld_dn10) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn13 } else { (-locals.var_ps0ld_dn13) },)
            }
        };
        let assign89530_body74_e137374: f64 = (1.0 + assign89530_body74_e137373);
        let assign89530_body74_e137375: f64 = (assign89530_body74_e137364 * assign89530_body74_e137374);
        (assign89530_body74_e137375, (assign89530_body74_e137364 * assign89530_body74_e137373_d_n0), (assign89530_body74_e137364 * assign89530_body74_e137373_d_n2), (assign89530_body74_e137364 * assign89530_body74_e137373_d_n4), (assign89530_body74_e137364 * assign89530_body74_e137373_d_n5), (assign89530_body74_e137364 * assign89530_body74_e137373_d_n6), (assign89530_body74_e137364 * assign89530_body74_e137373_d_n7), (assign89530_body74_e137364 * assign89530_body74_e137373_d_n8), (assign89530_body74_e137364 * assign89530_body74_e137373_d_n9), (assign89530_body74_e137364 * assign89530_body74_e137373_d_n10), (assign89530_body74_e137364 * assign89530_body74_e137373_d_n13),)
    } else {
        (locals.var_dplim, locals.var_dplim_dn0, locals.var_dplim_dn2, locals.var_dplim_dn4, locals.var_dplim_dn5, locals.var_dplim_dn6, locals.var_dplim_dn7, locals.var_dplim_dn8, locals.var_dplim_dn9, locals.var_dplim_dn10, locals.var_dplim_dn13,)
    }
};
            locals.var_dplim = assign89530_body74_e137377;
            locals.var_dplim_dn0 = assign89530_body74_e137377_d_n0;
            locals.var_dplim_dn2 = assign89530_body74_e137377_d_n2;
            locals.var_dplim_dn4 = assign89530_body74_e137377_d_n4;
            locals.var_dplim_dn5 = assign89530_body74_e137377_d_n5;
            locals.var_dplim_dn6 = assign89530_body74_e137377_d_n6;
            locals.var_dplim_dn7 = assign89530_body74_e137377_d_n7;
            locals.var_dplim_dn8 = assign89530_body74_e137377_d_n8;
            locals.var_dplim_dn9 = assign89530_body74_e137377_d_n9;
            locals.var_dplim_dn10 = assign89530_body74_e137377_d_n10;
            locals.var_dplim_dn13 = assign89530_body74_e137377_d_n13;
            locals.var_dplim_rv = 0.0;
            let assign89530_body75_e137379: f64 = (locals.var_dps0).abs();
            let assign89530_body75_e137381: f64 = if assign89530_body75_e137379 > locals.var_dplim { 1.0 } else { 0.0 };
            locals.var_guard2082 = assign89530_body75_e137381;
            locals.var_guard2082_rv = 0.0;
            let (assign89530_body76_e137403, assign89530_body76_e137403_d_n0, assign89530_body76_e137403_d_n2, assign89530_body76_e137403_d_n4, assign89530_body76_e137403_d_n5, assign89530_body76_e137403_d_n6, assign89530_body76_e137403_d_n7, assign89530_body76_e137403_d_n8, assign89530_body76_e137403_d_n9, assign89530_body76_e137403_d_n10, assign89530_body76_e137403_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) && (locals.var_guard2081 == 0.0)) && (locals.var_guard2082 != 0.0)) {
        let (assign89530_body76_e137400,) = {
            if (locals.var_dps0 >= 0.0) {
                (1.0,)
            } else {
                let assign89530_body76_e137399: f64 = (-1.0);
                (assign89530_body76_e137399,)
            }
        };
        let assign89530_body76_e137401: f64 = (locals.var_dplim * assign89530_body76_e137400);
        (assign89530_body76_e137401, (locals.var_dplim_dn0 * assign89530_body76_e137400), (locals.var_dplim_dn2 * assign89530_body76_e137400), (locals.var_dplim_dn4 * assign89530_body76_e137400), (locals.var_dplim_dn5 * assign89530_body76_e137400), (locals.var_dplim_dn6 * assign89530_body76_e137400), (locals.var_dplim_dn7 * assign89530_body76_e137400), (locals.var_dplim_dn8 * assign89530_body76_e137400), (locals.var_dplim_dn9 * assign89530_body76_e137400), (locals.var_dplim_dn10 * assign89530_body76_e137400), (locals.var_dplim_dn13 * assign89530_body76_e137400),)
    } else {
        (locals.var_dps0, locals.var_dps0_dn0, locals.var_dps0_dn2, locals.var_dps0_dn4, locals.var_dps0_dn5, locals.var_dps0_dn6, locals.var_dps0_dn7, locals.var_dps0_dn8, locals.var_dps0_dn9, locals.var_dps0_dn10, locals.var_dps0_dn13,)
    }
};
            locals.var_dps0 = assign89530_body76_e137403;
            locals.var_dps0_dn0 = assign89530_body76_e137403_d_n0;
            locals.var_dps0_dn2 = assign89530_body76_e137403_d_n2;
            locals.var_dps0_dn4 = assign89530_body76_e137403_d_n4;
            locals.var_dps0_dn5 = assign89530_body76_e137403_d_n5;
            locals.var_dps0_dn6 = assign89530_body76_e137403_d_n6;
            locals.var_dps0_dn7 = assign89530_body76_e137403_d_n7;
            locals.var_dps0_dn8 = assign89530_body76_e137403_d_n8;
            locals.var_dps0_dn9 = assign89530_body76_e137403_d_n9;
            locals.var_dps0_dn10 = assign89530_body76_e137403_d_n10;
            locals.var_dps0_dn13 = assign89530_body76_e137403_d_n13;
            locals.var_dps0_rv = 0.0;
            let (assign89530_body77_e137417, assign89530_body77_e137417_d_n0, assign89530_body77_e137417_d_n2, assign89530_body77_e137417_d_n4, assign89530_body77_e137417_d_n5, assign89530_body77_e137417_d_n6, assign89530_body77_e137417_d_n7, assign89530_body77_e137417_d_n8, assign89530_body77_e137417_d_n9, assign89530_body77_e137417_d_n10, assign89530_body77_e137417_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) && (locals.var_guard2081 == 0.0)) {
        let assign89530_body77_e137415: f64 = (locals.var_ps0ld + locals.var_dps0);
        (assign89530_body77_e137415, (locals.var_ps0ld_dn0 + locals.var_dps0_dn0), (locals.var_ps0ld_dn2 + locals.var_dps0_dn2), (locals.var_ps0ld_dn4 + locals.var_dps0_dn4), (locals.var_ps0ld_dn5 + locals.var_dps0_dn5), (locals.var_ps0ld_dn6 + locals.var_dps0_dn6), (locals.var_ps0ld_dn7 + locals.var_dps0_dn7), (locals.var_ps0ld_dn8 + locals.var_dps0_dn8), (locals.var_ps0ld_dn9 + locals.var_dps0_dn9), (locals.var_ps0ld_dn10 + locals.var_dps0_dn10), (locals.var_ps0ld_dn13 + locals.var_dps0_dn13),)
    } else {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn13,)
    }
};
            locals.var_ps0ld = assign89530_body77_e137417;
            locals.var_ps0ld_dn0 = assign89530_body77_e137417_d_n0;
            locals.var_ps0ld_dn2 = assign89530_body77_e137417_d_n2;
            locals.var_ps0ld_dn4 = assign89530_body77_e137417_d_n4;
            locals.var_ps0ld_dn5 = assign89530_body77_e137417_d_n5;
            locals.var_ps0ld_dn6 = assign89530_body77_e137417_d_n6;
            locals.var_ps0ld_dn7 = assign89530_body77_e137417_d_n7;
            locals.var_ps0ld_dn8 = assign89530_body77_e137417_d_n8;
            locals.var_ps0ld_dn9 = assign89530_body77_e137417_d_n9;
            locals.var_ps0ld_dn10 = assign89530_body77_e137417_d_n10;
            locals.var_ps0ld_dn13 = assign89530_body77_e137417_d_n13;
            locals.var_ps0ld_rv = 0.0;
            let assign89530_body78_e137419: f64 = (locals.var_dps0).abs();
            let assign89530_body78_e137423: f64 = (locals.var_fs0).abs();
            let assign89530_body78_e137426: f64 = if ((assign89530_body78_e137419 <= 1e-12) && (assign89530_body78_e137423 <= 1e-8)) { 1.0 } else { 0.0 };
            locals.var_guard2083 = assign89530_body78_e137426;
            locals.var_guard2083_rv = 0.0;
            let (assign89530_body79_e137440,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) && (locals.var_guard2081 == 0.0)) && (locals.var_guard2083 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_conv,)
    }
};
            locals.var_flg_conv = assign89530_body79_e137440;
            locals.var_flg_conv_rv = 0.0;
            let (assign89530_body80_e137451,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) {
        let assign89530_body80_e137449: f64 = (locals.var_lp_s0 + 1.0);
        (assign89530_body80_e137449,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign89530_body80_e137451;
            locals.var_lp_s0_rv = 0.0;
        }

    }

    pub(super) fn stamp_reactive_block_333(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign89550_e137465, assign89550_e137465_d_n0, assign89550_e137465_d_n2, assign89550_e137465_d_n4, assign89550_e137465_d_n5, assign89550_e137465_d_n6, assign89550_e137465_d_n7, assign89550_e137465_d_n8, assign89550_e137465_d_n9, assign89550_e137465_d_n10, assign89550_e137465_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) {
        let assign89550_e137463: f64 = (locals.var_c_w_ld * locals.var_fb);
        (assign89550_e137463, ((locals.var_c_w_ld_dn0 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn0)), ((locals.var_c_w_ld_dn2 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn2)), ((locals.var_c_w_ld_dn4 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn4)), ((locals.var_c_w_ld_dn5 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn5)), ((locals.var_c_w_ld_dn6 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn6)), ((locals.var_c_w_ld_dn7 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn7)), ((locals.var_c_w_ld_dn8 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn8)), ((locals.var_c_w_ld_dn9 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn9)), ((locals.var_c_w_ld_dn10 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn10)), ((locals.var_c_w_ld_dn13 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn13)),)
    } else {
        (locals.var_wdld__blk2002, locals.var_wdld__blk2002_dn0, locals.var_wdld__blk2002_dn2, locals.var_wdld__blk2002_dn4, locals.var_wdld__blk2002_dn5, locals.var_wdld__blk2002_dn6, locals.var_wdld__blk2002_dn7, locals.var_wdld__blk2002_dn8, locals.var_wdld__blk2002_dn9, locals.var_wdld__blk2002_dn10, locals.var_wdld__blk2002_dn13,)
    }
};
        locals.var_wdld__blk2002 = assign89550_e137465;
        locals.var_wdld__blk2002_dn0 = assign89550_e137465_d_n0;
        locals.var_wdld__blk2002_dn2 = assign89550_e137465_d_n2;
        locals.var_wdld__blk2002_dn4 = assign89550_e137465_d_n4;
        locals.var_wdld__blk2002_dn5 = assign89550_e137465_d_n5;
        locals.var_wdld__blk2002_dn6 = assign89550_e137465_d_n6;
        locals.var_wdld__blk2002_dn7 = assign89550_e137465_d_n7;
        locals.var_wdld__blk2002_dn8 = assign89550_e137465_d_n8;
        locals.var_wdld__blk2002_dn9 = assign89550_e137465_d_n9;
        locals.var_wdld__blk2002_dn10 = assign89550_e137465_d_n10;
        locals.var_wdld__blk2002_dn13 = assign89550_e137465_d_n13;
        locals.var_wdld__blk2002_rv = 0.0;

        let (assign89560_e137476, assign89560_e137476_d_n0, assign89560_e137476_d_n2, assign89560_e137476_d_n4, assign89560_e137476_d_n5, assign89560_e137476_d_n6, assign89560_e137476_d_n7, assign89560_e137476_d_n8, assign89560_e137476_d_n9, assign89560_e137476_d_n10, assign89560_e137476_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) {
        let assign89560_e137474: f64 = (locals.var_q_nsubld__blk2004 * locals.var_wdld__blk2002);
        (assign89560_e137474, (locals.var_q_nsubld__blk2004 * locals.var_wdld__blk2002_dn0), (locals.var_q_nsubld__blk2004 * locals.var_wdld__blk2002_dn2), (locals.var_q_nsubld__blk2004 * locals.var_wdld__blk2002_dn4), (locals.var_q_nsubld__blk2004 * locals.var_wdld__blk2002_dn5), (locals.var_q_nsubld__blk2004 * locals.var_wdld__blk2002_dn6), (locals.var_q_nsubld__blk2004 * locals.var_wdld__blk2002_dn7), (locals.var_q_nsubld__blk2004 * locals.var_wdld__blk2002_dn8), (locals.var_q_nsubld__blk2004 * locals.var_wdld__blk2002_dn9), (locals.var_q_nsubld__blk2004 * locals.var_wdld__blk2002_dn10), (locals.var_q_nsubld__blk2004 * locals.var_wdld__blk2002_dn13),)
    } else {
        (locals.var_q_dep_ld__blk2003, locals.var_q_dep_ld__blk2003_dn0, locals.var_q_dep_ld__blk2003_dn2, locals.var_q_dep_ld__blk2003_dn4, locals.var_q_dep_ld__blk2003_dn5, locals.var_q_dep_ld__blk2003_dn6, locals.var_q_dep_ld__blk2003_dn7, locals.var_q_dep_ld__blk2003_dn8, locals.var_q_dep_ld__blk2003_dn9, locals.var_q_dep_ld__blk2003_dn10, locals.var_q_dep_ld__blk2003_dn13,)
    }
};
        locals.var_q_dep_ld__blk2003 = assign89560_e137476;
        locals.var_q_dep_ld__blk2003_dn0 = assign89560_e137476_d_n0;
        locals.var_q_dep_ld__blk2003_dn2 = assign89560_e137476_d_n2;
        locals.var_q_dep_ld__blk2003_dn4 = assign89560_e137476_d_n4;
        locals.var_q_dep_ld__blk2003_dn5 = assign89560_e137476_d_n5;
        locals.var_q_dep_ld__blk2003_dn6 = assign89560_e137476_d_n6;
        locals.var_q_dep_ld__blk2003_dn7 = assign89560_e137476_d_n7;
        locals.var_q_dep_ld__blk2003_dn8 = assign89560_e137476_d_n8;
        locals.var_q_dep_ld__blk2003_dn9 = assign89560_e137476_d_n9;
        locals.var_q_dep_ld__blk2003_dn10 = assign89560_e137476_d_n10;
        locals.var_q_dep_ld__blk2003_dn13 = assign89560_e137476_d_n13;
        locals.var_q_dep_ld__blk2003_rv = 0.0;

        let (assign89570_e137491, assign89570_e137491_d_n0, assign89570_e137491_d_n2, assign89570_e137491_d_n4, assign89570_e137491_d_n5, assign89570_e137491_d_n6, assign89570_e137491_d_n7, assign89570_e137491_d_n8, assign89570_e137491_d_n9, assign89570_e137491_d_n10, assign89570_e137491_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) {
        let assign89570_e137485: f64 = (locals.var_q_dep_ld__blk2003 / locals.var_cnst0over_func);
        let assign89570_e137488: f64 = (10.0 * 2.220446049250313e-16);
        let assign89570_e137489: f64 = (assign89570_e137485 + assign89570_e137488);
        (assign89570_e137489, (((locals.var_q_dep_ld__blk2003_dn0 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk2003 * locals.var_cnst0over_func_dn0)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk2003_dn2 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk2003 * locals.var_cnst0over_func_dn2)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk2003_dn4 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk2003 * locals.var_cnst0over_func_dn4)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk2003_dn5 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk2003 * locals.var_cnst0over_func_dn5)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk2003_dn6 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk2003 * locals.var_cnst0over_func_dn6)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk2003_dn7 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk2003 * locals.var_cnst0over_func_dn7)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk2003_dn8 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk2003 * locals.var_cnst0over_func_dn8)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk2003_dn9 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk2003 * locals.var_cnst0over_func_dn9)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk2003_dn10 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk2003 * locals.var_cnst0over_func_dn10)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk2003_dn13 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk2003 * locals.var_cnst0over_func_dn13)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)),)
    } else {
        (locals.var_xi0p12, locals.var_xi0p12_dn0, locals.var_xi0p12_dn2, locals.var_xi0p12_dn4, locals.var_xi0p12_dn5, locals.var_xi0p12_dn6, locals.var_xi0p12_dn7, locals.var_xi0p12_dn8, locals.var_xi0p12_dn9, locals.var_xi0p12_dn10, locals.var_xi0p12_dn13,)
    }
};
        locals.var_xi0p12 = assign89570_e137491;
        locals.var_xi0p12_dn0 = assign89570_e137491_d_n0;
        locals.var_xi0p12_dn2 = assign89570_e137491_d_n2;
        locals.var_xi0p12_dn4 = assign89570_e137491_d_n4;
        locals.var_xi0p12_dn5 = assign89570_e137491_d_n5;
        locals.var_xi0p12_dn6 = assign89570_e137491_d_n6;
        locals.var_xi0p12_dn7 = assign89570_e137491_d_n7;
        locals.var_xi0p12_dn8 = assign89570_e137491_d_n8;
        locals.var_xi0p12_dn9 = assign89570_e137491_d_n9;
        locals.var_xi0p12_dn10 = assign89570_e137491_d_n10;
        locals.var_xi0p12_dn13 = assign89570_e137491_d_n13;
        locals.var_xi0p12_rv = 0.0;

        let (assign89580_e137502, assign89580_e137502_d_n0, assign89580_e137502_d_n2, assign89580_e137502_d_n4, assign89580_e137502_d_n5, assign89580_e137502_d_n6, assign89580_e137502_d_n7, assign89580_e137502_d_n8, assign89580_e137502_d_n9, assign89580_e137502_d_n10, assign89580_e137502_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) {
        let assign89580_e137500: f64 = (locals.var_cnst0over_func * locals.var_xi0p12);
        (assign89580_e137500, ((locals.var_cnst0over_func_dn0 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn0)), ((locals.var_cnst0over_func_dn2 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn2)), ((locals.var_cnst0over_func_dn4 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn4)), ((locals.var_cnst0over_func_dn5 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn5)), ((locals.var_cnst0over_func_dn6 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn6)), ((locals.var_cnst0over_func_dn7 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn7)), ((locals.var_cnst0over_func_dn8 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn8)), ((locals.var_cnst0over_func_dn9 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn9)), ((locals.var_cnst0over_func_dn10 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn10)), ((locals.var_cnst0over_func_dn13 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn13)),)
    } else {
        (locals.var_qbuld, locals.var_qbuld_dn0, locals.var_qbuld_dn2, locals.var_qbuld_dn4, locals.var_qbuld_dn5, locals.var_qbuld_dn6, locals.var_qbuld_dn7, locals.var_qbuld_dn8, locals.var_qbuld_dn9, locals.var_qbuld_dn10, locals.var_qbuld_dn13,)
    }
};
        locals.var_qbuld = assign89580_e137502;
        locals.var_qbuld_dn0 = assign89580_e137502_d_n0;
        locals.var_qbuld_dn2 = assign89580_e137502_d_n2;
        locals.var_qbuld_dn4 = assign89580_e137502_d_n4;
        locals.var_qbuld_dn5 = assign89580_e137502_d_n5;
        locals.var_qbuld_dn6 = assign89580_e137502_d_n6;
        locals.var_qbuld_dn7 = assign89580_e137502_d_n7;
        locals.var_qbuld_dn8 = assign89580_e137502_d_n8;
        locals.var_qbuld_dn9 = assign89580_e137502_d_n9;
        locals.var_qbuld_dn10 = assign89580_e137502_d_n10;
        locals.var_qbuld_dn13 = assign89580_e137502_d_n13;
        locals.var_qbuld_rv = 0.0;

        let (assign89590_e137515, assign89590_e137515_d_n0, assign89590_e137515_d_n2, assign89590_e137515_d_n4, assign89590_e137515_d_n5, assign89590_e137515_d_n6, assign89590_e137515_d_n7, assign89590_e137515_d_n8, assign89590_e137515_d_n9, assign89590_e137515_d_n10, assign89590_e137515_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) {
        let assign89590_e137512: f64 = (locals.var_fs02 + locals.var_xi0p12);
        let assign89590_e137513: f64 = (1.0 / assign89590_e137512);
        (assign89590_e137513, (-((locals.var_fs02_dn0 + locals.var_xi0p12_dn0) / (assign89590_e137512 * assign89590_e137512))), (-((locals.var_fs02_dn2 + locals.var_xi0p12_dn2) / (assign89590_e137512 * assign89590_e137512))), (-((locals.var_fs02_dn4 + locals.var_xi0p12_dn4) / (assign89590_e137512 * assign89590_e137512))), (-((locals.var_fs02_dn5 + locals.var_xi0p12_dn5) / (assign89590_e137512 * assign89590_e137512))), (-((locals.var_fs02_dn6 + locals.var_xi0p12_dn6) / (assign89590_e137512 * assign89590_e137512))), (-((locals.var_fs02_dn7 + locals.var_xi0p12_dn7) / (assign89590_e137512 * assign89590_e137512))), (-((locals.var_fs02_dn8 + locals.var_xi0p12_dn8) / (assign89590_e137512 * assign89590_e137512))), (-((locals.var_fs02_dn9 + locals.var_xi0p12_dn9) / (assign89590_e137512 * assign89590_e137512))), (-((locals.var_fs02_dn10 + locals.var_xi0p12_dn10) / (assign89590_e137512 * assign89590_e137512))), (-((locals.var_fs02_dn13 + locals.var_xi0p12_dn13) / (assign89590_e137512 * assign89590_e137512))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign89590_e137515;
        locals.var_t1_dn0 = assign89590_e137515_d_n0;
        locals.var_t1_dn2 = assign89590_e137515_d_n2;
        locals.var_t1_dn4 = assign89590_e137515_d_n4;
        locals.var_t1_dn5 = assign89590_e137515_d_n5;
        locals.var_t1_dn6 = assign89590_e137515_d_n6;
        locals.var_t1_dn7 = assign89590_e137515_d_n7;
        locals.var_t1_dn8 = assign89590_e137515_d_n8;
        locals.var_t1_dn9 = assign89590_e137515_d_n9;
        locals.var_t1_dn10 = assign89590_e137515_d_n10;
        locals.var_t1_dn13 = assign89590_e137515_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign89600_e137528, assign89600_e137528_d_n0, assign89600_e137528_d_n2, assign89600_e137528_d_n4, assign89600_e137528_d_n5, assign89600_e137528_d_n6, assign89600_e137528_d_n7, assign89600_e137528_d_n8, assign89600_e137528_d_n9, assign89600_e137528_d_n10, assign89600_e137528_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) {
        let assign89600_e137524: f64 = (locals.var_cnst0over_func * locals.var_fs01);
        let assign89600_e137526: f64 = (assign89600_e137524 * locals.var_t1);
        (assign89600_e137526, ((((locals.var_cnst0over_func_dn0 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn0)) * locals.var_t1) + (assign89600_e137524 * locals.var_t1_dn0)), ((((locals.var_cnst0over_func_dn2 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn2)) * locals.var_t1) + (assign89600_e137524 * locals.var_t1_dn2)), ((((locals.var_cnst0over_func_dn4 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn4)) * locals.var_t1) + (assign89600_e137524 * locals.var_t1_dn4)), ((((locals.var_cnst0over_func_dn5 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn5)) * locals.var_t1) + (assign89600_e137524 * locals.var_t1_dn5)), ((((locals.var_cnst0over_func_dn6 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn6)) * locals.var_t1) + (assign89600_e137524 * locals.var_t1_dn6)), ((((locals.var_cnst0over_func_dn7 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn7)) * locals.var_t1) + (assign89600_e137524 * locals.var_t1_dn7)), ((((locals.var_cnst0over_func_dn8 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn8)) * locals.var_t1) + (assign89600_e137524 * locals.var_t1_dn8)), ((((locals.var_cnst0over_func_dn9 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn9)) * locals.var_t1) + (assign89600_e137524 * locals.var_t1_dn9)), ((((locals.var_cnst0over_func_dn10 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn10)) * locals.var_t1) + (assign89600_e137524 * locals.var_t1_dn10)), ((((locals.var_cnst0over_func_dn13 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn13)) * locals.var_t1) + (assign89600_e137524 * locals.var_t1_dn13)),)
    } else {
        (locals.var_qiuld, locals.var_qiuld_dn0, locals.var_qiuld_dn2, locals.var_qiuld_dn4, locals.var_qiuld_dn5, locals.var_qiuld_dn6, locals.var_qiuld_dn7, locals.var_qiuld_dn8, locals.var_qiuld_dn9, locals.var_qiuld_dn10, locals.var_qiuld_dn13,)
    }
};
        locals.var_qiuld = assign89600_e137528;
        locals.var_qiuld_dn0 = assign89600_e137528_d_n0;
        locals.var_qiuld_dn2 = assign89600_e137528_d_n2;
        locals.var_qiuld_dn4 = assign89600_e137528_d_n4;
        locals.var_qiuld_dn5 = assign89600_e137528_d_n5;
        locals.var_qiuld_dn6 = assign89600_e137528_d_n6;
        locals.var_qiuld_dn7 = assign89600_e137528_d_n7;
        locals.var_qiuld_dn8 = assign89600_e137528_d_n8;
        locals.var_qiuld_dn9 = assign89600_e137528_d_n9;
        locals.var_qiuld_dn10 = assign89600_e137528_d_n10;
        locals.var_qiuld_dn13 = assign89600_e137528_d_n13;
        locals.var_qiuld_rv = 0.0;

        let (assign89610_e137539, assign89610_e137539_d_n0, assign89610_e137539_d_n2, assign89610_e137539_d_n4, assign89610_e137539_d_n5, assign89610_e137539_d_n6, assign89610_e137539_d_n7, assign89610_e137539_d_n8, assign89610_e137539_d_n9, assign89610_e137539_d_n10, assign89610_e137539_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) {
        let assign89610_e137537: f64 = (locals.var_qbuld + locals.var_qiuld);
        (assign89610_e137537, (locals.var_qbuld_dn0 + locals.var_qiuld_dn0), (locals.var_qbuld_dn2 + locals.var_qiuld_dn2), (locals.var_qbuld_dn4 + locals.var_qiuld_dn4), (locals.var_qbuld_dn5 + locals.var_qiuld_dn5), (locals.var_qbuld_dn6 + locals.var_qiuld_dn6), (locals.var_qbuld_dn7 + locals.var_qiuld_dn7), (locals.var_qbuld_dn8 + locals.var_qiuld_dn8), (locals.var_qbuld_dn9 + locals.var_qiuld_dn9), (locals.var_qbuld_dn10 + locals.var_qiuld_dn10), (locals.var_qbuld_dn13 + locals.var_qiuld_dn13),)
    } else {
        (locals.var_qsuld, locals.var_qsuld_dn0, locals.var_qsuld_dn2, locals.var_qsuld_dn4, locals.var_qsuld_dn5, locals.var_qsuld_dn6, locals.var_qsuld_dn7, locals.var_qsuld_dn8, locals.var_qsuld_dn9, locals.var_qsuld_dn10, locals.var_qsuld_dn13,)
    }
};
        locals.var_qsuld = assign89610_e137539;
        locals.var_qsuld_dn0 = assign89610_e137539_d_n0;
        locals.var_qsuld_dn2 = assign89610_e137539_d_n2;
        locals.var_qsuld_dn4 = assign89610_e137539_d_n4;
        locals.var_qsuld_dn5 = assign89610_e137539_d_n5;
        locals.var_qsuld_dn6 = assign89610_e137539_d_n6;
        locals.var_qsuld_dn7 = assign89610_e137539_d_n7;
        locals.var_qsuld_dn8 = assign89610_e137539_d_n8;
        locals.var_qsuld_dn9 = assign89610_e137539_d_n9;
        locals.var_qsuld_dn10 = assign89610_e137539_d_n10;
        locals.var_qsuld_dn13 = assign89610_e137539_d_n13;
        locals.var_qsuld_rv = 0.0;

        let assign89620_e137542: f64 = if p.p33 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard2085 = assign89620_e137542;
        locals.var_guard2085_rv = 0.0;

        let (assign89630_e137552, assign89630_e137552_d_n0, assign89630_e137552_d_n2, assign89630_e137552_d_n4, assign89630_e137552_d_n5, assign89630_e137552_d_n6, assign89630_e137552_d_n7, assign89630_e137552_d_n8, assign89630_e137552_d_n9, assign89630_e137552_d_n10, assign89630_e137552_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) {
        let assign89630_e137548: f64 = (-locals.var_vxbgmtcl);
        let assign89630_e137549: f64 = (locals.var_beta * assign89630_e137548);
        let assign89630_e137550: f64 = (assign89630_e137549).exp();
        (assign89630_e137550, (assign89630_e137550 * ((locals.var_beta_dn0 * assign89630_e137548) + (locals.var_beta * (-locals.var_vxbgmtcl_dn0)))), (assign89630_e137550 * ((locals.var_beta_dn2 * assign89630_e137548) + (locals.var_beta * (-locals.var_vxbgmtcl_dn2)))), (assign89630_e137550 * ((locals.var_beta_dn4 * assign89630_e137548) + (locals.var_beta * (-locals.var_vxbgmtcl_dn4)))), (assign89630_e137550 * ((locals.var_beta_dn5 * assign89630_e137548) + (locals.var_beta * (-locals.var_vxbgmtcl_dn5)))), (assign89630_e137550 * ((locals.var_beta_dn6 * assign89630_e137548) + (locals.var_beta * (-locals.var_vxbgmtcl_dn6)))), (assign89630_e137550 * ((locals.var_beta_dn7 * assign89630_e137548) + (locals.var_beta * (-locals.var_vxbgmtcl_dn7)))), (assign89630_e137550 * ((locals.var_beta_dn8 * assign89630_e137548) + (locals.var_beta * (-locals.var_vxbgmtcl_dn8)))), (assign89630_e137550 * ((locals.var_beta_dn9 * assign89630_e137548) + (locals.var_beta * (-locals.var_vxbgmtcl_dn9)))), (assign89630_e137550 * ((locals.var_beta_dn10 * assign89630_e137548) + (locals.var_beta * (-locals.var_vxbgmtcl_dn10)))), (assign89630_e137550 * ((locals.var_beta_dn13 * assign89630_e137548) + (locals.var_beta * (-locals.var_vxbgmtcl_dn13)))),)
    } else {
        (locals.var_exp_bvbs, locals.var_exp_bvbs_dn0, locals.var_exp_bvbs_dn2, locals.var_exp_bvbs_dn4, locals.var_exp_bvbs_dn5, locals.var_exp_bvbs_dn6, locals.var_exp_bvbs_dn7, locals.var_exp_bvbs_dn8, locals.var_exp_bvbs_dn9, locals.var_exp_bvbs_dn10, locals.var_exp_bvbs_dn13,)
    }
};
        locals.var_exp_bvbs = assign89630_e137552;
        locals.var_exp_bvbs_dn0 = assign89630_e137552_d_n0;
        locals.var_exp_bvbs_dn2 = assign89630_e137552_d_n2;
        locals.var_exp_bvbs_dn4 = assign89630_e137552_d_n4;
        locals.var_exp_bvbs_dn5 = assign89630_e137552_d_n5;
        locals.var_exp_bvbs_dn6 = assign89630_e137552_d_n6;
        locals.var_exp_bvbs_dn7 = assign89630_e137552_d_n7;
        locals.var_exp_bvbs_dn8 = assign89630_e137552_d_n8;
        locals.var_exp_bvbs_dn9 = assign89630_e137552_d_n9;
        locals.var_exp_bvbs_dn10 = assign89630_e137552_d_n10;
        locals.var_exp_bvbs_dn13 = assign89630_e137552_d_n13;
        locals.var_exp_bvbs_rv = 0.0;

        let (assign89640_e137560, assign89640_e137560_d_n0, assign89640_e137560_d_n2, assign89640_e137560_d_n4, assign89640_e137560_d_n5, assign89640_e137560_d_n6, assign89640_e137560_d_n7, assign89640_e137560_d_n8, assign89640_e137560_d_n9, assign89640_e137560_d_n10, assign89640_e137560_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) {
        let assign89640_e137558: f64 = (locals.var_nin / locals.var_nover_func);
        (assign89640_e137558, (locals.var_nin_dn0 / locals.var_nover_func), (locals.var_nin_dn2 / locals.var_nover_func), (locals.var_nin_dn4 / locals.var_nover_func), (locals.var_nin_dn5 / locals.var_nover_func), (locals.var_nin_dn6 / locals.var_nover_func), (locals.var_nin_dn7 / locals.var_nover_func), (locals.var_nin_dn8 / locals.var_nover_func), (locals.var_nin_dn9 / locals.var_nover_func), (locals.var_nin_dn10 / locals.var_nover_func), (locals.var_nin_dn13 / locals.var_nover_func),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign89640_e137560;
        locals.var_t0_dn0 = assign89640_e137560_d_n0;
        locals.var_t0_dn2 = assign89640_e137560_d_n2;
        locals.var_t0_dn4 = assign89640_e137560_d_n4;
        locals.var_t0_dn5 = assign89640_e137560_d_n5;
        locals.var_t0_dn6 = assign89640_e137560_d_n6;
        locals.var_t0_dn7 = assign89640_e137560_d_n7;
        locals.var_t0_dn8 = assign89640_e137560_d_n8;
        locals.var_t0_dn9 = assign89640_e137560_d_n9;
        locals.var_t0_dn10 = assign89640_e137560_d_n10;
        locals.var_t0_dn13 = assign89640_e137560_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign89650_e137568, assign89650_e137568_d_n0, assign89650_e137568_d_n2, assign89650_e137568_d_n4, assign89650_e137568_d_n5, assign89650_e137568_d_n6, assign89650_e137568_d_n7, assign89650_e137568_d_n8, assign89650_e137568_d_n9, assign89650_e137568_d_n10, assign89650_e137568_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) {
        let assign89650_e137566: f64 = (locals.var_t0 * locals.var_t0);
        (assign89650_e137566, ((locals.var_t0_dn0 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn0)), ((locals.var_t0_dn2 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn2)), ((locals.var_t0_dn4 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn4)), ((locals.var_t0_dn5 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn5)), ((locals.var_t0_dn6 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn6)), ((locals.var_t0_dn7 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn7)), ((locals.var_t0_dn8 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn8)), ((locals.var_t0_dn9 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn9)), ((locals.var_t0_dn10 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn10)), ((locals.var_t0_dn13 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn13)),)
    } else {
        (locals.var_cnst1over, locals.var_cnst1over_dn0, locals.var_cnst1over_dn2, locals.var_cnst1over_dn4, locals.var_cnst1over_dn5, locals.var_cnst1over_dn6, locals.var_cnst1over_dn7, locals.var_cnst1over_dn8, locals.var_cnst1over_dn9, locals.var_cnst1over_dn10, locals.var_cnst1over_dn13,)
    }
};
        locals.var_cnst1over = assign89650_e137568;
        locals.var_cnst1over_dn0 = assign89650_e137568_d_n0;
        locals.var_cnst1over_dn2 = assign89650_e137568_d_n2;
        locals.var_cnst1over_dn4 = assign89650_e137568_d_n4;
        locals.var_cnst1over_dn5 = assign89650_e137568_d_n5;
        locals.var_cnst1over_dn6 = assign89650_e137568_d_n6;
        locals.var_cnst1over_dn7 = assign89650_e137568_d_n7;
        locals.var_cnst1over_dn8 = assign89650_e137568_d_n8;
        locals.var_cnst1over_dn9 = assign89650_e137568_d_n9;
        locals.var_cnst1over_dn10 = assign89650_e137568_d_n10;
        locals.var_cnst1over_dn13 = assign89650_e137568_d_n13;
        locals.var_cnst1over_rv = 0.0;

        let (assign89660_e137576, assign89660_e137576_d_n0, assign89660_e137576_d_n2, assign89660_e137576_d_n4, assign89660_e137576_d_n5, assign89660_e137576_d_n6, assign89660_e137576_d_n7, assign89660_e137576_d_n8, assign89660_e137576_d_n9, assign89660_e137576_d_n10, assign89660_e137576_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) {
        let assign89660_e137574: f64 = (locals.var_cnst1over * locals.var_exp_bvbs);
        (assign89660_e137574, ((locals.var_cnst1over_dn0 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn0)), ((locals.var_cnst1over_dn2 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn2)), ((locals.var_cnst1over_dn4 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn4)), ((locals.var_cnst1over_dn5 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn5)), ((locals.var_cnst1over_dn6 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn6)), ((locals.var_cnst1over_dn7 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn7)), ((locals.var_cnst1over_dn8 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn8)), ((locals.var_cnst1over_dn9 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn9)), ((locals.var_cnst1over_dn10 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn10)), ((locals.var_cnst1over_dn13 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn13)),)
    } else {
        (locals.var_cfs1, locals.var_cfs1_dn0, locals.var_cfs1_dn2, locals.var_cfs1_dn4, locals.var_cfs1_dn5, locals.var_cfs1_dn6, locals.var_cfs1_dn7, locals.var_cfs1_dn8, locals.var_cfs1_dn9, locals.var_cfs1_dn10, locals.var_cfs1_dn13,)
    }
};
        locals.var_cfs1 = assign89660_e137576;
        locals.var_cfs1_dn0 = assign89660_e137576_d_n0;
        locals.var_cfs1_dn2 = assign89660_e137576_d_n2;
        locals.var_cfs1_dn4 = assign89660_e137576_d_n4;
        locals.var_cfs1_dn5 = assign89660_e137576_d_n5;
        locals.var_cfs1_dn6 = assign89660_e137576_d_n6;
        locals.var_cfs1_dn7 = assign89660_e137576_d_n7;
        locals.var_cfs1_dn8 = assign89660_e137576_d_n8;
        locals.var_cfs1_dn9 = assign89660_e137576_d_n9;
        locals.var_cfs1_dn10 = assign89660_e137576_d_n10;
        locals.var_cfs1_dn13 = assign89660_e137576_d_n13;
        locals.var_cfs1_rv = 0.0;

        let (assign89670_e137582, assign89670_e137582_d_n0, assign89670_e137582_d_n2, assign89670_e137582_d_n4, assign89670_e137582_d_n5, assign89670_e137582_d_n6, assign89670_e137582_d_n7, assign89670_e137582_d_n8, assign89670_e137582_d_n9, assign89670_e137582_d_n10, assign89670_e137582_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) {
        (locals.var_ps0ld_ini__blk2011, locals.var_ps0ld_ini__blk2011_dn0, locals.var_ps0ld_ini__blk2011_dn2, locals.var_ps0ld_ini__blk2011_dn4, locals.var_ps0ld_ini__blk2011_dn5, locals.var_ps0ld_ini__blk2011_dn6, locals.var_ps0ld_ini__blk2011_dn7, locals.var_ps0ld_ini__blk2011_dn8, locals.var_ps0ld_ini__blk2011_dn9, locals.var_ps0ld_ini__blk2011_dn10, locals.var_ps0ld_ini__blk2011_dn13,)
    } else {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn13,)
    }
};
        locals.var_ps0ld = assign89670_e137582;
        locals.var_ps0ld_dn0 = assign89670_e137582_d_n0;
        locals.var_ps0ld_dn2 = assign89670_e137582_d_n2;
        locals.var_ps0ld_dn4 = assign89670_e137582_d_n4;
        locals.var_ps0ld_dn5 = assign89670_e137582_d_n5;
        locals.var_ps0ld_dn6 = assign89670_e137582_d_n6;
        locals.var_ps0ld_dn7 = assign89670_e137582_d_n7;
        locals.var_ps0ld_dn8 = assign89670_e137582_d_n8;
        locals.var_ps0ld_dn9 = assign89670_e137582_d_n9;
        locals.var_ps0ld_dn10 = assign89670_e137582_d_n10;
        locals.var_ps0ld_dn13 = assign89670_e137582_d_n13;
        locals.var_ps0ld_rv = 0.0;

        let (assign89680_e137588,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_flg_conv,)
    }
};
        locals.var_flg_conv = assign89680_e137588;
        locals.var_flg_conv_rv = 0.0;

        let (assign89690_e137601, assign89690_e137601_d_n0, assign89690_e137601_d_n2, assign89690_e137601_d_n4, assign89690_e137601_d_n5, assign89690_e137601_d_n6, assign89690_e137601_d_n7, assign89690_e137601_d_n8, assign89690_e137601_d_n9, assign89690_e137601_d_n10, assign89690_e137601_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) {
        let assign89690_e137595: f64 = (1.034943e-10 / locals.var_q_nsubld__blk2004);
        let assign89690_e137597: f64 = (assign89690_e137595 * locals.var_beta_inv);
        let assign89690_e137598: f64 = (2.0 * assign89690_e137597);
        let assign89690_e137599: f64 = (assign89690_e137598).sqrt();
        (assign89690_e137599, ((2.0 * (assign89690_e137595 * locals.var_beta_inv_dn0)) / (2.0 * assign89690_e137599)), ((2.0 * (assign89690_e137595 * locals.var_beta_inv_dn2)) / (2.0 * assign89690_e137599)), ((2.0 * (assign89690_e137595 * locals.var_beta_inv_dn4)) / (2.0 * assign89690_e137599)), ((2.0 * (assign89690_e137595 * locals.var_beta_inv_dn5)) / (2.0 * assign89690_e137599)), ((2.0 * (assign89690_e137595 * locals.var_beta_inv_dn6)) / (2.0 * assign89690_e137599)), ((2.0 * (assign89690_e137595 * locals.var_beta_inv_dn7)) / (2.0 * assign89690_e137599)), ((2.0 * (assign89690_e137595 * locals.var_beta_inv_dn8)) / (2.0 * assign89690_e137599)), ((2.0 * (assign89690_e137595 * locals.var_beta_inv_dn9)) / (2.0 * assign89690_e137599)), ((2.0 * (assign89690_e137595 * locals.var_beta_inv_dn10)) / (2.0 * assign89690_e137599)), ((2.0 * (assign89690_e137595 * locals.var_beta_inv_dn13)) / (2.0 * assign89690_e137599)),)
    } else {
        (locals.var_c_w_ld, locals.var_c_w_ld_dn0, locals.var_c_w_ld_dn2, locals.var_c_w_ld_dn4, locals.var_c_w_ld_dn5, locals.var_c_w_ld_dn6, locals.var_c_w_ld_dn7, locals.var_c_w_ld_dn8, locals.var_c_w_ld_dn9, locals.var_c_w_ld_dn10, locals.var_c_w_ld_dn13,)
    }
};
        locals.var_c_w_ld = assign89690_e137601;
        locals.var_c_w_ld_dn0 = assign89690_e137601_d_n0;
        locals.var_c_w_ld_dn2 = assign89690_e137601_d_n2;
        locals.var_c_w_ld_dn4 = assign89690_e137601_d_n4;
        locals.var_c_w_ld_dn5 = assign89690_e137601_d_n5;
        locals.var_c_w_ld_dn6 = assign89690_e137601_d_n6;
        locals.var_c_w_ld_dn7 = assign89690_e137601_d_n7;
        locals.var_c_w_ld_dn8 = assign89690_e137601_d_n8;
        locals.var_c_w_ld_dn9 = assign89690_e137601_d_n9;
        locals.var_c_w_ld_dn10 = assign89690_e137601_d_n10;
        locals.var_c_w_ld_dn13 = assign89690_e137601_d_n13;
        locals.var_c_w_ld_rv = 0.0;

        let assign89700_e137604: f64 = if locals.var_wdep_func > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2086 = assign89700_e137604;
        locals.var_guard2086_rv = 0.0;

        let (assign89710_e137614, assign89710_e137614_d_n0, assign89710_e137614_d_n2, assign89710_e137614_d_n4, assign89710_e137614_d_n5, assign89710_e137614_d_n6, assign89710_e137614_d_n7, assign89710_e137614_d_n8, assign89710_e137614_d_n9, assign89710_e137614_d_n10, assign89710_e137614_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) && (locals.var_guard2086 != 0.0)) {
        let assign89710_e137612: f64 = (p.p334 - locals.var_wdep_func);
        (assign89710_e137612, (-locals.var_wdep_func_dn0), (-locals.var_wdep_func_dn2), (-locals.var_wdep_func_dn4), (-locals.var_wdep_func_dn5), (-locals.var_wdep_func_dn6), (-locals.var_wdep_func_dn7), (-locals.var_wdep_func_dn8), (-locals.var_wdep_func_dn9), (-locals.var_wdep_func_dn10), (-locals.var_wdep_func_dn13),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign89710_e137614;
        locals.var_t2_dn0 = assign89710_e137614_d_n0;
        locals.var_t2_dn2 = assign89710_e137614_d_n2;
        locals.var_t2_dn4 = assign89710_e137614_d_n4;
        locals.var_t2_dn5 = assign89710_e137614_d_n5;
        locals.var_t2_dn6 = assign89710_e137614_d_n6;
        locals.var_t2_dn7 = assign89710_e137614_d_n7;
        locals.var_t2_dn8 = assign89710_e137614_d_n8;
        locals.var_t2_dn9 = assign89710_e137614_d_n9;
        locals.var_t2_dn10 = assign89710_e137614_d_n10;
        locals.var_t2_dn13 = assign89710_e137614_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign89720_e137636, assign89720_e137636_d_n0, assign89720_e137636_d_n2, assign89720_e137636_d_n4, assign89720_e137636_d_n5, assign89720_e137636_d_n6, assign89720_e137636_d_n7, assign89720_e137636_d_n8, assign89720_e137636_d_n9, assign89720_e137636_d_n10, assign89720_e137636_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) && (locals.var_guard2086 == 0.0)) {
        let assign89720_e137623: f64 = (locals.var_vdsi + p.p137);
        let assign89720_e137626: f64 = (locals.var_vdsi + p.p137);
        let assign89720_e137627: f64 = (assign89720_e137623 * assign89720_e137626);
        let assign89720_e137630: f64 = (4.0 * 0.1);
        let assign89720_e137632: f64 = (assign89720_e137630 * 0.1);
        let assign89720_e137633: f64 = (assign89720_e137627 + assign89720_e137632);
        let assign89720_e137634: f64 = (assign89720_e137633).sqrt();
        (assign89720_e137634, 0.0, 0.0, 0.0, (((locals.var_vdsi_dn5 * assign89720_e137626) + (assign89720_e137623 * locals.var_vdsi_dn5)) / (2.0 * assign89720_e137634)), 0.0, (((locals.var_vdsi_dn7 * assign89720_e137626) + (assign89720_e137623 * locals.var_vdsi_dn7)) / (2.0 * assign89720_e137634)), 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign89720_e137636;
        locals.var_tmf2_dn0 = assign89720_e137636_d_n0;
        locals.var_tmf2_dn2 = assign89720_e137636_d_n2;
        locals.var_tmf2_dn4 = assign89720_e137636_d_n4;
        locals.var_tmf2_dn5 = assign89720_e137636_d_n5;
        locals.var_tmf2_dn6 = assign89720_e137636_d_n6;
        locals.var_tmf2_dn7 = assign89720_e137636_d_n7;
        locals.var_tmf2_dn8 = assign89720_e137636_d_n8;
        locals.var_tmf2_dn9 = assign89720_e137636_d_n9;
        locals.var_tmf2_dn10 = assign89720_e137636_d_n10;
        locals.var_tmf2_dn13 = assign89720_e137636_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign89730_e137653, assign89730_e137653_d_n0, assign89730_e137653_d_n2, assign89730_e137653_d_n4, assign89730_e137653_d_n5, assign89730_e137653_d_n6, assign89730_e137653_d_n7, assign89730_e137653_d_n8, assign89730_e137653_d_n9, assign89730_e137653_d_n10, assign89730_e137653_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) && (locals.var_guard2086 == 0.0)) {
        let assign89730_e137647: f64 = (locals.var_vdsi + p.p137);
        let assign89730_e137649: f64 = (assign89730_e137647 / locals.var_tmf2);
        let assign89730_e137650: f64 = (1.0 + assign89730_e137649);
        let assign89730_e137651: f64 = (0.5 * assign89730_e137650);
        (assign89730_e137651, (0.5 * (-((assign89730_e137647 * locals.var_tmf2_dn0) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign89730_e137647 * locals.var_tmf2_dn2) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign89730_e137647 * locals.var_tmf2_dn4) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (((locals.var_vdsi_dn5 * locals.var_tmf2) - (assign89730_e137647 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (-((assign89730_e137647 * locals.var_tmf2_dn6) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (((locals.var_vdsi_dn7 * locals.var_tmf2) - (assign89730_e137647 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (-((assign89730_e137647 * locals.var_tmf2_dn8) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign89730_e137647 * locals.var_tmf2_dn9) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign89730_e137647 * locals.var_tmf2_dn10) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign89730_e137647 * locals.var_tmf2_dn13) / (locals.var_tmf2 * locals.var_tmf2)))),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign89730_e137653;
        locals.var_t9_dn0 = assign89730_e137653_d_n0;
        locals.var_t9_dn2 = assign89730_e137653_d_n2;
        locals.var_t9_dn4 = assign89730_e137653_d_n4;
        locals.var_t9_dn5 = assign89730_e137653_d_n5;
        locals.var_t9_dn6 = assign89730_e137653_d_n6;
        locals.var_t9_dn7 = assign89730_e137653_d_n7;
        locals.var_t9_dn8 = assign89730_e137653_d_n8;
        locals.var_t9_dn9 = assign89730_e137653_d_n9;
        locals.var_t9_dn10 = assign89730_e137653_d_n10;
        locals.var_t9_dn13 = assign89730_e137653_d_n13;
        locals.var_t9_rv = 0.0;

        let (assign89740_e137668, assign89740_e137668_d_n0, assign89740_e137668_d_n2, assign89740_e137668_d_n4, assign89740_e137668_d_n5, assign89740_e137668_d_n6, assign89740_e137668_d_n7, assign89740_e137668_d_n8, assign89740_e137668_d_n9, assign89740_e137668_d_n10, assign89740_e137668_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) && (locals.var_guard2086 == 0.0)) {
        let assign89740_e137663: f64 = (locals.var_vdsi + p.p137);
        let assign89740_e137665: f64 = (assign89740_e137663 + locals.var_tmf2);
        let assign89740_e137666: f64 = (0.5 * assign89740_e137665);
        (assign89740_e137666, (0.5 * locals.var_tmf2_dn0), (0.5 * locals.var_tmf2_dn2), (0.5 * locals.var_tmf2_dn4), (0.5 * (locals.var_vdsi_dn5 + locals.var_tmf2_dn5)), (0.5 * locals.var_tmf2_dn6), (0.5 * (locals.var_vdsi_dn7 + locals.var_tmf2_dn7)), (0.5 * locals.var_tmf2_dn8), (0.5 * locals.var_tmf2_dn9), (0.5 * locals.var_tmf2_dn10), (0.5 * locals.var_tmf2_dn13),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign89740_e137668;
        locals.var_t2_dn0 = assign89740_e137668_d_n0;
        locals.var_t2_dn2 = assign89740_e137668_d_n2;
        locals.var_t2_dn4 = assign89740_e137668_d_n4;
        locals.var_t2_dn5 = assign89740_e137668_d_n5;
        locals.var_t2_dn6 = assign89740_e137668_d_n6;
        locals.var_t2_dn7 = assign89740_e137668_d_n7;
        locals.var_t2_dn8 = assign89740_e137668_d_n8;
        locals.var_t2_dn9 = assign89740_e137668_d_n9;
        locals.var_t2_dn10 = assign89740_e137668_d_n10;
        locals.var_t2_dn13 = assign89740_e137668_d_n13;
        locals.var_t2_rv = 0.0;

        let assign89750_e137671: f64 = if locals.var_t2 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2087 = assign89750_e137671;
        locals.var_guard2087_rv = 0.0;

        let (assign89760_e137682, assign89760_e137682_d_n0, assign89760_e137682_d_n2, assign89760_e137682_d_n4, assign89760_e137682_d_n5, assign89760_e137682_d_n6, assign89760_e137682_d_n7, assign89760_e137682_d_n8, assign89760_e137682_d_n9, assign89760_e137682_d_n10, assign89760_e137682_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) && (locals.var_guard2086 == 0.0)) && (locals.var_guard2087 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign89760_e137682;
        locals.var_t2_dn0 = assign89760_e137682_d_n0;
        locals.var_t2_dn2 = assign89760_e137682_d_n2;
        locals.var_t2_dn4 = assign89760_e137682_d_n4;
        locals.var_t2_dn5 = assign89760_e137682_d_n5;
        locals.var_t2_dn6 = assign89760_e137682_d_n6;
        locals.var_t2_dn7 = assign89760_e137682_d_n7;
        locals.var_t2_dn8 = assign89760_e137682_d_n8;
        locals.var_t2_dn9 = assign89760_e137682_d_n9;
        locals.var_t2_dn10 = assign89760_e137682_d_n10;
        locals.var_t2_dn13 = assign89760_e137682_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign89770_e137693, assign89770_e137693_d_n0, assign89770_e137693_d_n2, assign89770_e137693_d_n4, assign89770_e137693_d_n5, assign89770_e137693_d_n6, assign89770_e137693_d_n7, assign89770_e137693_d_n8, assign89770_e137693_d_n9, assign89770_e137693_d_n10, assign89770_e137693_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) && (locals.var_guard2086 == 0.0)) && (locals.var_guard2087 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign89770_e137693;
        locals.var_t9_dn0 = assign89770_e137693_d_n0;
        locals.var_t9_dn2 = assign89770_e137693_d_n2;
        locals.var_t9_dn4 = assign89770_e137693_d_n4;
        locals.var_t9_dn5 = assign89770_e137693_d_n5;
        locals.var_t9_dn6 = assign89770_e137693_d_n6;
        locals.var_t9_dn7 = assign89770_e137693_d_n7;
        locals.var_t9_dn8 = assign89770_e137693_d_n8;
        locals.var_t9_dn9 = assign89770_e137693_d_n9;
        locals.var_t9_dn10 = assign89770_e137693_d_n10;
        locals.var_t9_dn13 = assign89770_e137693_d_n13;
        locals.var_t9_rv = 0.0;

        let (assign89780_e137707, assign89780_e137707_d_n0, assign89780_e137707_d_n2, assign89780_e137707_d_n4, assign89780_e137707_d_n5, assign89780_e137707_d_n6, assign89780_e137707_d_n7, assign89780_e137707_d_n8, assign89780_e137707_d_n9, assign89780_e137707_d_n10, assign89780_e137707_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) && (locals.var_guard2086 == 0.0)) {
        let assign89780_e137702: f64 = (locals.var_kjunc * locals.var_t2);
        let assign89780_e137703: f64 = (assign89780_e137702).sqrt();
        let assign89780_e137705: f64 = (assign89780_e137703 * p.p432);
        (assign89780_e137705, ((((locals.var_kjunc_dn0 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn0)) / (2.0 * assign89780_e137703)) * p.p432), ((((locals.var_kjunc_dn2 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn2)) / (2.0 * assign89780_e137703)) * p.p432), ((((locals.var_kjunc_dn4 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn4)) / (2.0 * assign89780_e137703)) * p.p432), ((((locals.var_kjunc_dn5 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn5)) / (2.0 * assign89780_e137703)) * p.p432), ((((locals.var_kjunc_dn6 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn6)) / (2.0 * assign89780_e137703)) * p.p432), ((((locals.var_kjunc_dn7 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn7)) / (2.0 * assign89780_e137703)) * p.p432), ((((locals.var_kjunc_dn8 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn8)) / (2.0 * assign89780_e137703)) * p.p432), ((((locals.var_kjunc_dn9 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn9)) / (2.0 * assign89780_e137703)) * p.p432), ((((locals.var_kjunc_dn10 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn10)) / (2.0 * assign89780_e137703)) * p.p432), ((((locals.var_kjunc_dn13 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn13)) / (2.0 * assign89780_e137703)) * p.p432),)
    } else {
        (locals.var_wjunc0, locals.var_wjunc0_dn0, locals.var_wjunc0_dn2, locals.var_wjunc0_dn4, locals.var_wjunc0_dn5, locals.var_wjunc0_dn6, locals.var_wjunc0_dn7, locals.var_wjunc0_dn8, locals.var_wjunc0_dn9, locals.var_wjunc0_dn10, locals.var_wjunc0_dn13,)
    }
};
        locals.var_wjunc0 = assign89780_e137707;
        locals.var_wjunc0_dn0 = assign89780_e137707_d_n0;
        locals.var_wjunc0_dn2 = assign89780_e137707_d_n2;
        locals.var_wjunc0_dn4 = assign89780_e137707_d_n4;
        locals.var_wjunc0_dn5 = assign89780_e137707_d_n5;
        locals.var_wjunc0_dn6 = assign89780_e137707_d_n6;
        locals.var_wjunc0_dn7 = assign89780_e137707_d_n7;
        locals.var_wjunc0_dn8 = assign89780_e137707_d_n8;
        locals.var_wjunc0_dn9 = assign89780_e137707_d_n9;
        locals.var_wjunc0_dn10 = assign89780_e137707_d_n10;
        locals.var_wjunc0_dn13 = assign89780_e137707_d_n13;
        locals.var_wjunc0_rv = 0.0;

        let (assign89790_e137718, assign89790_e137718_d_n0, assign89790_e137718_d_n2, assign89790_e137718_d_n4, assign89790_e137718_d_n5, assign89790_e137718_d_n6, assign89790_e137718_d_n7, assign89790_e137718_d_n8, assign89790_e137718_d_n9, assign89790_e137718_d_n10, assign89790_e137718_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) && (locals.var_guard2086 == 0.0)) {
        let assign89790_e137716: f64 = (p.p334 - locals.var_wjunc0);
        (assign89790_e137716, (-locals.var_wjunc0_dn0), (-locals.var_wjunc0_dn2), (-locals.var_wjunc0_dn4), (-locals.var_wjunc0_dn5), (-locals.var_wjunc0_dn6), (-locals.var_wjunc0_dn7), (-locals.var_wjunc0_dn8), (-locals.var_wjunc0_dn9), (-locals.var_wjunc0_dn10), (-locals.var_wjunc0_dn13),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign89790_e137718;
        locals.var_t2_dn0 = assign89790_e137718_d_n0;
        locals.var_t2_dn2 = assign89790_e137718_d_n2;
        locals.var_t2_dn4 = assign89790_e137718_d_n4;
        locals.var_t2_dn5 = assign89790_e137718_d_n5;
        locals.var_t2_dn6 = assign89790_e137718_d_n6;
        locals.var_t2_dn7 = assign89790_e137718_d_n7;
        locals.var_t2_dn8 = assign89790_e137718_d_n8;
        locals.var_t2_dn9 = assign89790_e137718_d_n9;
        locals.var_t2_dn10 = assign89790_e137718_d_n10;
        locals.var_t2_dn13 = assign89790_e137718_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign89800_e137737, assign89800_e137737_d_n0, assign89800_e137737_d_n2, assign89800_e137737_d_n4, assign89800_e137737_d_n5, assign89800_e137737_d_n6, assign89800_e137737_d_n7, assign89800_e137737_d_n8, assign89800_e137737_d_n9, assign89800_e137737_d_n10, assign89800_e137737_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) {
        let assign89800_e137724: f64 = (locals.var_t2 * locals.var_t2);
        let assign89800_e137728: f64 = (p.p334 * 0.01);
        let assign89800_e137729: f64 = (4.0 * assign89800_e137728);
        let assign89800_e137732: f64 = (p.p334 * 0.01);
        let assign89800_e137733: f64 = (assign89800_e137729 * assign89800_e137732);
        let assign89800_e137734: f64 = (assign89800_e137724 + assign89800_e137733);
        let assign89800_e137735: f64 = (assign89800_e137734).sqrt();
        (assign89800_e137735, (((locals.var_t2_dn0 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn0)) / (2.0 * assign89800_e137735)), (((locals.var_t2_dn2 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn2)) / (2.0 * assign89800_e137735)), (((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4)) / (2.0 * assign89800_e137735)), (((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5)) / (2.0 * assign89800_e137735)), (((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6)) / (2.0 * assign89800_e137735)), (((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7)) / (2.0 * assign89800_e137735)), (((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8)) / (2.0 * assign89800_e137735)), (((locals.var_t2_dn9 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn9)) / (2.0 * assign89800_e137735)), (((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10)) / (2.0 * assign89800_e137735)), (((locals.var_t2_dn13 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn13)) / (2.0 * assign89800_e137735)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign89800_e137737;
        locals.var_tmf2_dn0 = assign89800_e137737_d_n0;
        locals.var_tmf2_dn2 = assign89800_e137737_d_n2;
        locals.var_tmf2_dn4 = assign89800_e137737_d_n4;
        locals.var_tmf2_dn5 = assign89800_e137737_d_n5;
        locals.var_tmf2_dn6 = assign89800_e137737_d_n6;
        locals.var_tmf2_dn7 = assign89800_e137737_d_n7;
        locals.var_tmf2_dn8 = assign89800_e137737_d_n8;
        locals.var_tmf2_dn9 = assign89800_e137737_d_n9;
        locals.var_tmf2_dn10 = assign89800_e137737_d_n10;
        locals.var_tmf2_dn13 = assign89800_e137737_d_n13;
        locals.var_tmf2_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_334(
        locals: &mut StampLocals,
    ) {
        let (assign89810_e137749, assign89810_e137749_d_n0, assign89810_e137749_d_n2, assign89810_e137749_d_n4, assign89810_e137749_d_n5, assign89810_e137749_d_n6, assign89810_e137749_d_n7, assign89810_e137749_d_n8, assign89810_e137749_d_n9, assign89810_e137749_d_n10, assign89810_e137749_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) {
        let assign89810_e137745: f64 = (locals.var_t2 / locals.var_tmf2);
        let assign89810_e137746: f64 = (1.0 + assign89810_e137745);
        let assign89810_e137747: f64 = (0.5 * assign89810_e137746);
        (assign89810_e137747, (0.5 * (((locals.var_t2_dn0 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn2 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn4 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn5 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn6 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn7 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn8 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn9 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn10 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn13 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign89810_e137749;
        locals.var_t9_dn0 = assign89810_e137749_d_n0;
        locals.var_t9_dn2 = assign89810_e137749_d_n2;
        locals.var_t9_dn4 = assign89810_e137749_d_n4;
        locals.var_t9_dn5 = assign89810_e137749_d_n5;
        locals.var_t9_dn6 = assign89810_e137749_d_n6;
        locals.var_t9_dn7 = assign89810_e137749_d_n7;
        locals.var_t9_dn8 = assign89810_e137749_d_n8;
        locals.var_t9_dn9 = assign89810_e137749_d_n9;
        locals.var_t9_dn10 = assign89810_e137749_d_n10;
        locals.var_t9_dn13 = assign89810_e137749_d_n13;
        locals.var_t9_rv = 0.0;

        let (assign89820_e137759, assign89820_e137759_d_n0, assign89820_e137759_d_n2, assign89820_e137759_d_n4, assign89820_e137759_d_n5, assign89820_e137759_d_n6, assign89820_e137759_d_n7, assign89820_e137759_d_n8, assign89820_e137759_d_n9, assign89820_e137759_d_n10, assign89820_e137759_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) {
        let assign89820_e137756: f64 = (locals.var_t2 + locals.var_tmf2);
        let assign89820_e137757: f64 = (0.5 * assign89820_e137756);
        (assign89820_e137757, (0.5 * (locals.var_t2_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_t2_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_t2_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_t2_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_t2_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_t2_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_t2_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_t2_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_t2_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_t2_dn13 + locals.var_tmf2_dn13)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign89820_e137759;
        locals.var_t2_dn0 = assign89820_e137759_d_n0;
        locals.var_t2_dn2 = assign89820_e137759_d_n2;
        locals.var_t2_dn4 = assign89820_e137759_d_n4;
        locals.var_t2_dn5 = assign89820_e137759_d_n5;
        locals.var_t2_dn6 = assign89820_e137759_d_n6;
        locals.var_t2_dn7 = assign89820_e137759_d_n7;
        locals.var_t2_dn8 = assign89820_e137759_d_n8;
        locals.var_t2_dn9 = assign89820_e137759_d_n9;
        locals.var_t2_dn10 = assign89820_e137759_d_n10;
        locals.var_t2_dn13 = assign89820_e137759_d_n13;
        locals.var_t2_rv = 0.0;

        let assign89830_e137762: f64 = if locals.var_t2 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2088 = assign89830_e137762;
        locals.var_guard2088_rv = 0.0;

        let (assign89840_e137770, assign89840_e137770_d_n0, assign89840_e137770_d_n2, assign89840_e137770_d_n4, assign89840_e137770_d_n5, assign89840_e137770_d_n6, assign89840_e137770_d_n7, assign89840_e137770_d_n8, assign89840_e137770_d_n9, assign89840_e137770_d_n10, assign89840_e137770_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) && (locals.var_guard2088 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign89840_e137770;
        locals.var_t2_dn0 = assign89840_e137770_d_n0;
        locals.var_t2_dn2 = assign89840_e137770_d_n2;
        locals.var_t2_dn4 = assign89840_e137770_d_n4;
        locals.var_t2_dn5 = assign89840_e137770_d_n5;
        locals.var_t2_dn6 = assign89840_e137770_d_n6;
        locals.var_t2_dn7 = assign89840_e137770_d_n7;
        locals.var_t2_dn8 = assign89840_e137770_d_n8;
        locals.var_t2_dn9 = assign89840_e137770_d_n9;
        locals.var_t2_dn10 = assign89840_e137770_d_n10;
        locals.var_t2_dn13 = assign89840_e137770_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign89850_e137778, assign89850_e137778_d_n0, assign89850_e137778_d_n2, assign89850_e137778_d_n4, assign89850_e137778_d_n5, assign89850_e137778_d_n6, assign89850_e137778_d_n7, assign89850_e137778_d_n8, assign89850_e137778_d_n9, assign89850_e137778_d_n10, assign89850_e137778_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) && (locals.var_guard2088 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign89850_e137778;
        locals.var_t9_dn0 = assign89850_e137778_d_n0;
        locals.var_t9_dn2 = assign89850_e137778_d_n2;
        locals.var_t9_dn4 = assign89850_e137778_d_n4;
        locals.var_t9_dn5 = assign89850_e137778_d_n5;
        locals.var_t9_dn6 = assign89850_e137778_d_n6;
        locals.var_t9_dn7 = assign89850_e137778_d_n7;
        locals.var_t9_dn8 = assign89850_e137778_d_n8;
        locals.var_t9_dn9 = assign89850_e137778_d_n9;
        locals.var_t9_dn10 = assign89850_e137778_d_n10;
        locals.var_t9_dn13 = assign89850_e137778_d_n13;
        locals.var_t9_rv = 0.0;

        let (assign89860_e137784, assign89860_e137784_d_n0, assign89860_e137784_d_n2, assign89860_e137784_d_n4, assign89860_e137784_d_n5, assign89860_e137784_d_n6, assign89860_e137784_d_n7, assign89860_e137784_d_n8, assign89860_e137784_d_n9, assign89860_e137784_d_n10, assign89860_e137784_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    } else {
        (locals.var_ddriftldc, locals.var_ddriftldc_dn0, locals.var_ddriftldc_dn2, locals.var_ddriftldc_dn4, locals.var_ddriftldc_dn5, locals.var_ddriftldc_dn6, locals.var_ddriftldc_dn7, locals.var_ddriftldc_dn8, locals.var_ddriftldc_dn9, locals.var_ddriftldc_dn10, locals.var_ddriftldc_dn13,)
    }
};
        locals.var_ddriftldc = assign89860_e137784;
        locals.var_ddriftldc_dn0 = assign89860_e137784_d_n0;
        locals.var_ddriftldc_dn2 = assign89860_e137784_d_n2;
        locals.var_ddriftldc_dn4 = assign89860_e137784_d_n4;
        locals.var_ddriftldc_dn5 = assign89860_e137784_d_n5;
        locals.var_ddriftldc_dn6 = assign89860_e137784_d_n6;
        locals.var_ddriftldc_dn7 = assign89860_e137784_d_n7;
        locals.var_ddriftldc_dn8 = assign89860_e137784_d_n8;
        locals.var_ddriftldc_dn9 = assign89860_e137784_d_n9;
        locals.var_ddriftldc_dn10 = assign89860_e137784_d_n10;
        locals.var_ddriftldc_dn13 = assign89860_e137784_d_n13;
        locals.var_ddriftldc_rv = 0.0;

        let (assign89870_e137798, assign89870_e137798_d_n0, assign89870_e137798_d_n2, assign89870_e137798_d_n4, assign89870_e137798_d_n5, assign89870_e137798_d_n6, assign89870_e137798_d_n7, assign89870_e137798_d_n8, assign89870_e137798_d_n9, assign89870_e137798_d_n10, assign89870_e137798_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) {
        let assign89870_e137790: f64 = (locals.var_q_nsubld__blk2004 * locals.var_ddriftldc);
        let assign89870_e137792: f64 = (assign89870_e137790 * locals.var_ddriftldc);
        let assign89870_e137794: f64 = (assign89870_e137792 / 2.0);
        let assign89870_e137796: f64 = (assign89870_e137794 / 1.034943e-10);
        (assign89870_e137796, (((((locals.var_q_nsubld__blk2004 * locals.var_ddriftldc_dn0) * locals.var_ddriftldc) + (assign89870_e137790 * locals.var_ddriftldc_dn0)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2004 * locals.var_ddriftldc_dn2) * locals.var_ddriftldc) + (assign89870_e137790 * locals.var_ddriftldc_dn2)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2004 * locals.var_ddriftldc_dn4) * locals.var_ddriftldc) + (assign89870_e137790 * locals.var_ddriftldc_dn4)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2004 * locals.var_ddriftldc_dn5) * locals.var_ddriftldc) + (assign89870_e137790 * locals.var_ddriftldc_dn5)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2004 * locals.var_ddriftldc_dn6) * locals.var_ddriftldc) + (assign89870_e137790 * locals.var_ddriftldc_dn6)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2004 * locals.var_ddriftldc_dn7) * locals.var_ddriftldc) + (assign89870_e137790 * locals.var_ddriftldc_dn7)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2004 * locals.var_ddriftldc_dn8) * locals.var_ddriftldc) + (assign89870_e137790 * locals.var_ddriftldc_dn8)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2004 * locals.var_ddriftldc_dn9) * locals.var_ddriftldc) + (assign89870_e137790 * locals.var_ddriftldc_dn9)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2004 * locals.var_ddriftldc_dn10) * locals.var_ddriftldc) + (assign89870_e137790 * locals.var_ddriftldc_dn10)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2004 * locals.var_ddriftldc_dn13) * locals.var_ddriftldc) + (assign89870_e137790 * locals.var_ddriftldc_dn13)) / 2.0) / 1.034943e-10),)
    } else {
        (locals.var_dphi_sb, locals.var_dphi_sb_dn0, locals.var_dphi_sb_dn2, locals.var_dphi_sb_dn4, locals.var_dphi_sb_dn5, locals.var_dphi_sb_dn6, locals.var_dphi_sb_dn7, locals.var_dphi_sb_dn8, locals.var_dphi_sb_dn9, locals.var_dphi_sb_dn10, locals.var_dphi_sb_dn13,)
    }
};
        locals.var_dphi_sb = assign89870_e137798;
        locals.var_dphi_sb_dn0 = assign89870_e137798_d_n0;
        locals.var_dphi_sb_dn2 = assign89870_e137798_d_n2;
        locals.var_dphi_sb_dn4 = assign89870_e137798_d_n4;
        locals.var_dphi_sb_dn5 = assign89870_e137798_d_n5;
        locals.var_dphi_sb_dn6 = assign89870_e137798_d_n6;
        locals.var_dphi_sb_dn7 = assign89870_e137798_d_n7;
        locals.var_dphi_sb_dn8 = assign89870_e137798_d_n8;
        locals.var_dphi_sb_dn9 = assign89870_e137798_d_n9;
        locals.var_dphi_sb_dn10 = assign89870_e137798_d_n10;
        locals.var_dphi_sb_dn13 = assign89870_e137798_d_n13;
        locals.var_dphi_sb_rv = 0.0;

        let (assign89880_e137809, assign89880_e137809_d_n0, assign89880_e137809_d_n2, assign89880_e137809_d_n4, assign89880_e137809_d_n5, assign89880_e137809_d_n6, assign89880_e137809_d_n7, assign89880_e137809_d_n8, assign89880_e137809_d_n9, assign89880_e137809_d_n10, assign89880_e137809_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) {
        let assign89880_e137804: f64 = (2.0 * locals.var_beta);
        let assign89880_e137806: f64 = (assign89880_e137804 * locals.var_dphi_sb);
        let assign89880_e137807: f64 = (assign89880_e137806).sqrt();
        (assign89880_e137807, ((((2.0 * locals.var_beta_dn0) * locals.var_dphi_sb) + (assign89880_e137804 * locals.var_dphi_sb_dn0)) / (2.0 * assign89880_e137807)), ((((2.0 * locals.var_beta_dn2) * locals.var_dphi_sb) + (assign89880_e137804 * locals.var_dphi_sb_dn2)) / (2.0 * assign89880_e137807)), ((((2.0 * locals.var_beta_dn4) * locals.var_dphi_sb) + (assign89880_e137804 * locals.var_dphi_sb_dn4)) / (2.0 * assign89880_e137807)), ((((2.0 * locals.var_beta_dn5) * locals.var_dphi_sb) + (assign89880_e137804 * locals.var_dphi_sb_dn5)) / (2.0 * assign89880_e137807)), ((((2.0 * locals.var_beta_dn6) * locals.var_dphi_sb) + (assign89880_e137804 * locals.var_dphi_sb_dn6)) / (2.0 * assign89880_e137807)), ((((2.0 * locals.var_beta_dn7) * locals.var_dphi_sb) + (assign89880_e137804 * locals.var_dphi_sb_dn7)) / (2.0 * assign89880_e137807)), ((((2.0 * locals.var_beta_dn8) * locals.var_dphi_sb) + (assign89880_e137804 * locals.var_dphi_sb_dn8)) / (2.0 * assign89880_e137807)), ((((2.0 * locals.var_beta_dn9) * locals.var_dphi_sb) + (assign89880_e137804 * locals.var_dphi_sb_dn9)) / (2.0 * assign89880_e137807)), ((((2.0 * locals.var_beta_dn10) * locals.var_dphi_sb) + (assign89880_e137804 * locals.var_dphi_sb_dn10)) / (2.0 * assign89880_e137807)), ((((2.0 * locals.var_beta_dn13) * locals.var_dphi_sb) + (assign89880_e137804 * locals.var_dphi_sb_dn13)) / (2.0 * assign89880_e137807)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign89880_e137809;
        locals.var_t0_dn0 = assign89880_e137809_d_n0;
        locals.var_t0_dn2 = assign89880_e137809_d_n2;
        locals.var_t0_dn4 = assign89880_e137809_d_n4;
        locals.var_t0_dn5 = assign89880_e137809_d_n5;
        locals.var_t0_dn6 = assign89880_e137809_d_n6;
        locals.var_t0_dn7 = assign89880_e137809_d_n7;
        locals.var_t0_dn8 = assign89880_e137809_d_n8;
        locals.var_t0_dn9 = assign89880_e137809_d_n9;
        locals.var_t0_dn10 = assign89880_e137809_d_n10;
        locals.var_t0_dn13 = assign89880_e137809_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign89890_e137822, assign89890_e137822_d_n0, assign89890_e137822_d_n2, assign89890_e137822_d_n4, assign89890_e137822_d_n5, assign89890_e137822_d_n6, assign89890_e137822_d_n7, assign89890_e137822_d_n8, assign89890_e137822_d_n9, assign89890_e137822_d_n10, assign89890_e137822_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) {
        let assign89890_e137814: f64 = { let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign89890_e137816: f64 = (-locals.var_t0);
        let assign89890_e137817: f64 = { let limited_exp_arg = assign89890_e137816; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign89890_e137818: f64 = (assign89890_e137814 + assign89890_e137817);
        let assign89890_e137820: f64 = (assign89890_e137818 / 2.0);
        (assign89890_e137820, ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn0) + ({ let limited_exp_arg = assign89890_e137816; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn0))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn2) + ({ let limited_exp_arg = assign89890_e137816; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn2))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn4) + ({ let limited_exp_arg = assign89890_e137816; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn4))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn5) + ({ let limited_exp_arg = assign89890_e137816; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn5))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn6) + ({ let limited_exp_arg = assign89890_e137816; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn6))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn7) + ({ let limited_exp_arg = assign89890_e137816; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn7))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn8) + ({ let limited_exp_arg = assign89890_e137816; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn8))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn9) + ({ let limited_exp_arg = assign89890_e137816; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn9))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn10) + ({ let limited_exp_arg = assign89890_e137816; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn10))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn13) + ({ let limited_exp_arg = assign89890_e137816; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn13))) / 2.0),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign89890_e137822;
        locals.var_t1_dn0 = assign89890_e137822_d_n0;
        locals.var_t1_dn2 = assign89890_e137822_d_n2;
        locals.var_t1_dn4 = assign89890_e137822_d_n4;
        locals.var_t1_dn5 = assign89890_e137822_d_n5;
        locals.var_t1_dn6 = assign89890_e137822_d_n6;
        locals.var_t1_dn7 = assign89890_e137822_d_n7;
        locals.var_t1_dn8 = assign89890_e137822_d_n8;
        locals.var_t1_dn9 = assign89890_e137822_d_n9;
        locals.var_t1_dn10 = assign89890_e137822_d_n10;
        locals.var_t1_dn13 = assign89890_e137822_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign89900_e137831, assign89900_e137831_d_n0, assign89900_e137831_d_n2, assign89900_e137831_d_n4, assign89900_e137831_d_n5, assign89900_e137831_d_n6, assign89900_e137831_d_n7, assign89900_e137831_d_n8, assign89900_e137831_d_n9, assign89900_e137831_d_n10, assign89900_e137831_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) {
        let assign89900_e137827: f64 = (locals.var_t1).ln();
        let assign89900_e137829: f64 = (assign89900_e137827 / locals.var_dphi_sb);
        (assign89900_e137829, ((((locals.var_t1_dn0 / locals.var_t1) * locals.var_dphi_sb) - (assign89900_e137827 * locals.var_dphi_sb_dn0)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn2 / locals.var_t1) * locals.var_dphi_sb) - (assign89900_e137827 * locals.var_dphi_sb_dn2)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn4 / locals.var_t1) * locals.var_dphi_sb) - (assign89900_e137827 * locals.var_dphi_sb_dn4)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn5 / locals.var_t1) * locals.var_dphi_sb) - (assign89900_e137827 * locals.var_dphi_sb_dn5)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn6 / locals.var_t1) * locals.var_dphi_sb) - (assign89900_e137827 * locals.var_dphi_sb_dn6)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn7 / locals.var_t1) * locals.var_dphi_sb) - (assign89900_e137827 * locals.var_dphi_sb_dn7)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn8 / locals.var_t1) * locals.var_dphi_sb) - (assign89900_e137827 * locals.var_dphi_sb_dn8)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn9 / locals.var_t1) * locals.var_dphi_sb) - (assign89900_e137827 * locals.var_dphi_sb_dn9)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn10 / locals.var_t1) * locals.var_dphi_sb) - (assign89900_e137827 * locals.var_dphi_sb_dn10)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn13 / locals.var_t1) * locals.var_dphi_sb) - (assign89900_e137827 * locals.var_dphi_sb_dn13)) / (locals.var_dphi_sb * locals.var_dphi_sb)),)
    } else {
        (locals.var_c_sb, locals.var_c_sb_dn0, locals.var_c_sb_dn2, locals.var_c_sb_dn4, locals.var_c_sb_dn5, locals.var_c_sb_dn6, locals.var_c_sb_dn7, locals.var_c_sb_dn8, locals.var_c_sb_dn9, locals.var_c_sb_dn10, locals.var_c_sb_dn13,)
    }
};
        locals.var_c_sb = assign89900_e137831;
        locals.var_c_sb_dn0 = assign89900_e137831_d_n0;
        locals.var_c_sb_dn2 = assign89900_e137831_d_n2;
        locals.var_c_sb_dn4 = assign89900_e137831_d_n4;
        locals.var_c_sb_dn5 = assign89900_e137831_d_n5;
        locals.var_c_sb_dn6 = assign89900_e137831_d_n6;
        locals.var_c_sb_dn7 = assign89900_e137831_d_n7;
        locals.var_c_sb_dn8 = assign89900_e137831_d_n8;
        locals.var_c_sb_dn9 = assign89900_e137831_d_n9;
        locals.var_c_sb_dn10 = assign89900_e137831_d_n10;
        locals.var_c_sb_dn13 = assign89900_e137831_d_n13;
        locals.var_c_sb_rv = 0.0;

        let (assign89910_e137837,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_lp_s0,)
    }
};
        locals.var_lp_s0 = assign89910_e137837;
        locals.var_lp_s0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_335(
        locals: &mut StampLocals,
    ) {
        let mut assign89920_loop_guard: usize = 0;
        while {
            let assign89920_cond_e137844: f64 = (locals.var_lp_s0_max + 1.0);
            let assign89920_cond_e137846: f64 = if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) && (locals.var_lp_s0 <= assign89920_cond_e137844)) { 1.0 } else { 0.0 };
            assign89920_cond_e137846 != 0.0
        } {
            assign89920_loop_guard += 1;
            assert!(assign89920_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign89920_body3_e137873, assign89920_body3_e137873_d_n0, assign89920_body3_e137873_d_n2, assign89920_body3_e137873_d_n4, assign89920_body3_e137873_d_n5, assign89920_body3_e137873_d_n6, assign89920_body3_e137873_d_n7, assign89920_body3_e137873_d_n8, assign89920_body3_e137873_d_n9, assign89920_body3_e137873_d_n10, assign89920_body3_e137873_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) {
        let assign89920_body3_e137871: f64 = (locals.var_ps0ld + locals.var_vxbgmtcl);
        (assign89920_body3_e137871, (locals.var_ps0ld_dn0 + locals.var_vxbgmtcl_dn0), (locals.var_ps0ld_dn2 + locals.var_vxbgmtcl_dn2), (locals.var_ps0ld_dn4 + locals.var_vxbgmtcl_dn4), (locals.var_ps0ld_dn5 + locals.var_vxbgmtcl_dn5), (locals.var_ps0ld_dn6 + locals.var_vxbgmtcl_dn6), (locals.var_ps0ld_dn7 + locals.var_vxbgmtcl_dn7), (locals.var_ps0ld_dn8 + locals.var_vxbgmtcl_dn8), (locals.var_ps0ld_dn9 + locals.var_vxbgmtcl_dn9), (locals.var_ps0ld_dn10 + locals.var_vxbgmtcl_dn10), (locals.var_ps0ld_dn13 + locals.var_vxbgmtcl_dn13),)
    } else {
        (locals.var_ps0ld_vxb, locals.var_ps0ld_vxb_dn0, locals.var_ps0ld_vxb_dn2, locals.var_ps0ld_vxb_dn4, locals.var_ps0ld_vxb_dn5, locals.var_ps0ld_vxb_dn6, locals.var_ps0ld_vxb_dn7, locals.var_ps0ld_vxb_dn8, locals.var_ps0ld_vxb_dn9, locals.var_ps0ld_vxb_dn10, locals.var_ps0ld_vxb_dn13,)
    }
};
            locals.var_ps0ld_vxb = assign89920_body3_e137873;
            locals.var_ps0ld_vxb_dn0 = assign89920_body3_e137873_d_n0;
            locals.var_ps0ld_vxb_dn2 = assign89920_body3_e137873_d_n2;
            locals.var_ps0ld_vxb_dn4 = assign89920_body3_e137873_d_n4;
            locals.var_ps0ld_vxb_dn5 = assign89920_body3_e137873_d_n5;
            locals.var_ps0ld_vxb_dn6 = assign89920_body3_e137873_d_n6;
            locals.var_ps0ld_vxb_dn7 = assign89920_body3_e137873_d_n7;
            locals.var_ps0ld_vxb_dn8 = assign89920_body3_e137873_d_n8;
            locals.var_ps0ld_vxb_dn9 = assign89920_body3_e137873_d_n9;
            locals.var_ps0ld_vxb_dn10 = assign89920_body3_e137873_d_n10;
            locals.var_ps0ld_vxb_dn13 = assign89920_body3_e137873_d_n13;
            locals.var_ps0ld_vxb_rv = 0.0;
            let (assign89920_body4_e137881, assign89920_body4_e137881_d_n0, assign89920_body4_e137881_d_n2, assign89920_body4_e137881_d_n4, assign89920_body4_e137881_d_n5, assign89920_body4_e137881_d_n6, assign89920_body4_e137881_d_n7, assign89920_body4_e137881_d_n8, assign89920_body4_e137881_d_n9, assign89920_body4_e137881_d_n10, assign89920_body4_e137881_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) {
        let assign89920_body4_e137879: f64 = (locals.var_beta * locals.var_ps0ld_vxb);
        (assign89920_body4_e137879, ((locals.var_beta_dn0 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn0)), ((locals.var_beta_dn2 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn2)), ((locals.var_beta_dn4 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn4)), ((locals.var_beta_dn5 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn5)), ((locals.var_beta_dn6 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn6)), ((locals.var_beta_dn7 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn7)), ((locals.var_beta_dn8 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn8)), ((locals.var_beta_dn9 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn9)), ((locals.var_beta_dn10 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn10)), ((locals.var_beta_dn13 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn13)),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn13,)
    }
};
            locals.var_chi = assign89920_body4_e137881;
            locals.var_chi_dn0 = assign89920_body4_e137881_d_n0;
            locals.var_chi_dn2 = assign89920_body4_e137881_d_n2;
            locals.var_chi_dn4 = assign89920_body4_e137881_d_n4;
            locals.var_chi_dn5 = assign89920_body4_e137881_d_n5;
            locals.var_chi_dn6 = assign89920_body4_e137881_d_n6;
            locals.var_chi_dn7 = assign89920_body4_e137881_d_n7;
            locals.var_chi_dn8 = assign89920_body4_e137881_d_n8;
            locals.var_chi_dn9 = assign89920_body4_e137881_d_n9;
            locals.var_chi_dn10 = assign89920_body4_e137881_d_n10;
            locals.var_chi_dn13 = assign89920_body4_e137881_d_n13;
            locals.var_chi_rv = 0.0;
            let (assign89920_body5_e137891, assign89920_body5_e137891_d_n0, assign89920_body5_e137891_d_n2, assign89920_body5_e137891_d_n4, assign89920_body5_e137891_d_n5, assign89920_body5_e137891_d_n6, assign89920_body5_e137891_d_n7, assign89920_body5_e137891_d_n8, assign89920_body5_e137891_d_n9, assign89920_body5_e137891_d_n10, assign89920_body5_e137891_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) {
        let assign89920_body5_e137888: f64 = (locals.var_ps0ld_vxb - locals.var_dphi_sb);
        let assign89920_body5_e137889: f64 = (locals.var_c_sb * assign89920_body5_e137888);
        (assign89920_body5_e137889, ((locals.var_c_sb_dn0 * assign89920_body5_e137888) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn0 - locals.var_dphi_sb_dn0))), ((locals.var_c_sb_dn2 * assign89920_body5_e137888) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn2 - locals.var_dphi_sb_dn2))), ((locals.var_c_sb_dn4 * assign89920_body5_e137888) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn4 - locals.var_dphi_sb_dn4))), ((locals.var_c_sb_dn5 * assign89920_body5_e137888) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn5 - locals.var_dphi_sb_dn5))), ((locals.var_c_sb_dn6 * assign89920_body5_e137888) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn6 - locals.var_dphi_sb_dn6))), ((locals.var_c_sb_dn7 * assign89920_body5_e137888) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn7 - locals.var_dphi_sb_dn7))), ((locals.var_c_sb_dn8 * assign89920_body5_e137888) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn8 - locals.var_dphi_sb_dn8))), ((locals.var_c_sb_dn9 * assign89920_body5_e137888) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn9 - locals.var_dphi_sb_dn9))), ((locals.var_c_sb_dn10 * assign89920_body5_e137888) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn10 - locals.var_dphi_sb_dn10))), ((locals.var_c_sb_dn13 * assign89920_body5_e137888) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn13 - locals.var_dphi_sb_dn13))),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn4, locals.var_ty_dn5, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn8, locals.var_ty_dn9, locals.var_ty_dn10, locals.var_ty_dn13,)
    }
};
            locals.var_ty = assign89920_body5_e137891;
            locals.var_ty_dn0 = assign89920_body5_e137891_d_n0;
            locals.var_ty_dn2 = assign89920_body5_e137891_d_n2;
            locals.var_ty_dn4 = assign89920_body5_e137891_d_n4;
            locals.var_ty_dn5 = assign89920_body5_e137891_d_n5;
            locals.var_ty_dn6 = assign89920_body5_e137891_d_n6;
            locals.var_ty_dn7 = assign89920_body5_e137891_d_n7;
            locals.var_ty_dn8 = assign89920_body5_e137891_d_n8;
            locals.var_ty_dn9 = assign89920_body5_e137891_d_n9;
            locals.var_ty_dn10 = assign89920_body5_e137891_d_n10;
            locals.var_ty_dn13 = assign89920_body5_e137891_d_n13;
            locals.var_ty_rv = 0.0;
            let assign89920_body6_e137894: f64 = if locals.var_ty < 60.0 { 1.0 } else { 0.0 };
            locals.var_guard2090 = assign89920_body6_e137894;
            locals.var_guard2090_rv = 0.0;
            let (assign89920_body7_e137903, assign89920_body7_e137903_d_n0, assign89920_body7_e137903_d_n2, assign89920_body7_e137903_d_n4, assign89920_body7_e137903_d_n5, assign89920_body7_e137903_d_n6, assign89920_body7_e137903_d_n7, assign89920_body7_e137903_d_n8, assign89920_body7_e137903_d_n9, assign89920_body7_e137903_d_n10, assign89920_body7_e137903_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) && (locals.var_guard2090 != 0.0)) {
        let assign89920_body7_e137901: f64 = (locals.var_ty).exp();
        (assign89920_body7_e137901, (assign89920_body7_e137901 * locals.var_ty_dn0), (assign89920_body7_e137901 * locals.var_ty_dn2), (assign89920_body7_e137901 * locals.var_ty_dn4), (assign89920_body7_e137901 * locals.var_ty_dn5), (assign89920_body7_e137901 * locals.var_ty_dn6), (assign89920_body7_e137901 * locals.var_ty_dn7), (assign89920_body7_e137901 * locals.var_ty_dn8), (assign89920_body7_e137901 * locals.var_ty_dn9), (assign89920_body7_e137901 * locals.var_ty_dn10), (assign89920_body7_e137901 * locals.var_ty_dn13),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
            locals.var_t1 = assign89920_body7_e137903;
            locals.var_t1_dn0 = assign89920_body7_e137903_d_n0;
            locals.var_t1_dn2 = assign89920_body7_e137903_d_n2;
            locals.var_t1_dn4 = assign89920_body7_e137903_d_n4;
            locals.var_t1_dn5 = assign89920_body7_e137903_d_n5;
            locals.var_t1_dn6 = assign89920_body7_e137903_d_n6;
            locals.var_t1_dn7 = assign89920_body7_e137903_d_n7;
            locals.var_t1_dn8 = assign89920_body7_e137903_d_n8;
            locals.var_t1_dn9 = assign89920_body7_e137903_d_n9;
            locals.var_t1_dn10 = assign89920_body7_e137903_d_n10;
            locals.var_t1_dn13 = assign89920_body7_e137903_d_n13;
            locals.var_t1_rv = 0.0;
            let (assign89920_body8_e137915, assign89920_body8_e137915_d_n0, assign89920_body8_e137915_d_n2, assign89920_body8_e137915_d_n4, assign89920_body8_e137915_d_n5, assign89920_body8_e137915_d_n6, assign89920_body8_e137915_d_n7, assign89920_body8_e137915_d_n8, assign89920_body8_e137915_d_n9, assign89920_body8_e137915_d_n10, assign89920_body8_e137915_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) && (locals.var_guard2090 != 0.0)) {
        let assign89920_body8_e137910: f64 = (-locals.var_c_sb);
        let assign89920_body8_e137912: f64 = (assign89920_body8_e137910 * locals.var_dphi_sb);
        let assign89920_body8_e137913: f64 = (assign89920_body8_e137912).exp();
        (assign89920_body8_e137913, (assign89920_body8_e137913 * (((-locals.var_c_sb_dn0) * locals.var_dphi_sb) + (assign89920_body8_e137910 * locals.var_dphi_sb_dn0))), (assign89920_body8_e137913 * (((-locals.var_c_sb_dn2) * locals.var_dphi_sb) + (assign89920_body8_e137910 * locals.var_dphi_sb_dn2))), (assign89920_body8_e137913 * (((-locals.var_c_sb_dn4) * locals.var_dphi_sb) + (assign89920_body8_e137910 * locals.var_dphi_sb_dn4))), (assign89920_body8_e137913 * (((-locals.var_c_sb_dn5) * locals.var_dphi_sb) + (assign89920_body8_e137910 * locals.var_dphi_sb_dn5))), (assign89920_body8_e137913 * (((-locals.var_c_sb_dn6) * locals.var_dphi_sb) + (assign89920_body8_e137910 * locals.var_dphi_sb_dn6))), (assign89920_body8_e137913 * (((-locals.var_c_sb_dn7) * locals.var_dphi_sb) + (assign89920_body8_e137910 * locals.var_dphi_sb_dn7))), (assign89920_body8_e137913 * (((-locals.var_c_sb_dn8) * locals.var_dphi_sb) + (assign89920_body8_e137910 * locals.var_dphi_sb_dn8))), (assign89920_body8_e137913 * (((-locals.var_c_sb_dn9) * locals.var_dphi_sb) + (assign89920_body8_e137910 * locals.var_dphi_sb_dn9))), (assign89920_body8_e137913 * (((-locals.var_c_sb_dn10) * locals.var_dphi_sb) + (assign89920_body8_e137910 * locals.var_dphi_sb_dn10))), (assign89920_body8_e137913 * (((-locals.var_c_sb_dn13) * locals.var_dphi_sb) + (assign89920_body8_e137910 * locals.var_dphi_sb_dn13))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
            locals.var_t0 = assign89920_body8_e137915;
            locals.var_t0_dn0 = assign89920_body8_e137915_d_n0;
            locals.var_t0_dn2 = assign89920_body8_e137915_d_n2;
            locals.var_t0_dn4 = assign89920_body8_e137915_d_n4;
            locals.var_t0_dn5 = assign89920_body8_e137915_d_n5;
            locals.var_t0_dn6 = assign89920_body8_e137915_d_n6;
            locals.var_t0_dn7 = assign89920_body8_e137915_d_n7;
            locals.var_t0_dn8 = assign89920_body8_e137915_d_n8;
            locals.var_t0_dn9 = assign89920_body8_e137915_d_n9;
            locals.var_t0_dn10 = assign89920_body8_e137915_d_n10;
            locals.var_t0_dn13 = assign89920_body8_e137915_d_n13;
            locals.var_t0_rv = 0.0;
            let (assign89920_body9_e137925, assign89920_body9_e137925_d_n0, assign89920_body9_e137925_d_n2, assign89920_body9_e137925_d_n4, assign89920_body9_e137925_d_n5, assign89920_body9_e137925_d_n6, assign89920_body9_e137925_d_n7, assign89920_body9_e137925_d_n8, assign89920_body9_e137925_d_n9, assign89920_body9_e137925_d_n10, assign89920_body9_e137925_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) && (locals.var_guard2090 != 0.0)) {
        let assign89920_body9_e137923: f64 = (locals.var_t1 - locals.var_t0);
        (assign89920_body9_e137923, (locals.var_t1_dn0 - locals.var_t0_dn0), (locals.var_t1_dn2 - locals.var_t0_dn2), (locals.var_t1_dn4 - locals.var_t0_dn4), (locals.var_t1_dn5 - locals.var_t0_dn5), (locals.var_t1_dn6 - locals.var_t0_dn6), (locals.var_t1_dn7 - locals.var_t0_dn7), (locals.var_t1_dn8 - locals.var_t0_dn8), (locals.var_t1_dn9 - locals.var_t0_dn9), (locals.var_t1_dn10 - locals.var_t0_dn10), (locals.var_t1_dn13 - locals.var_t0_dn13),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
            locals.var_t2 = assign89920_body9_e137925;
            locals.var_t2_dn0 = assign89920_body9_e137925_d_n0;
            locals.var_t2_dn2 = assign89920_body9_e137925_d_n2;
            locals.var_t2_dn4 = assign89920_body9_e137925_d_n4;
            locals.var_t2_dn5 = assign89920_body9_e137925_d_n5;
            locals.var_t2_dn6 = assign89920_body9_e137925_d_n6;
            locals.var_t2_dn7 = assign89920_body9_e137925_d_n7;
            locals.var_t2_dn8 = assign89920_body9_e137925_d_n8;
            locals.var_t2_dn9 = assign89920_body9_e137925_d_n9;
            locals.var_t2_dn10 = assign89920_body9_e137925_d_n10;
            locals.var_t2_dn13 = assign89920_body9_e137925_d_n13;
            locals.var_t2_rv = 0.0;
            let (assign89920_body10_e137938, assign89920_body10_e137938_d_n0, assign89920_body10_e137938_d_n2, assign89920_body10_e137938_d_n4, assign89920_body10_e137938_d_n5, assign89920_body10_e137938_d_n6, assign89920_body10_e137938_d_n7, assign89920_body10_e137938_d_n8, assign89920_body10_e137938_d_n9, assign89920_body10_e137938_d_n10, assign89920_body10_e137938_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) && (locals.var_guard2090 != 0.0)) {
        let assign89920_body10_e137933: f64 = (1.0 + locals.var_t2);
        let assign89920_body10_e137934: f64 = (assign89920_body10_e137933).ln();
        let assign89920_body10_e137936: f64 = (assign89920_body10_e137934 / locals.var_c_sb);
        (assign89920_body10_e137936, ((((locals.var_t2_dn0 / assign89920_body10_e137933) * locals.var_c_sb) - (assign89920_body10_e137934 * locals.var_c_sb_dn0)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn2 / assign89920_body10_e137933) * locals.var_c_sb) - (assign89920_body10_e137934 * locals.var_c_sb_dn2)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn4 / assign89920_body10_e137933) * locals.var_c_sb) - (assign89920_body10_e137934 * locals.var_c_sb_dn4)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn5 / assign89920_body10_e137933) * locals.var_c_sb) - (assign89920_body10_e137934 * locals.var_c_sb_dn5)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn6 / assign89920_body10_e137933) * locals.var_c_sb) - (assign89920_body10_e137934 * locals.var_c_sb_dn6)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn7 / assign89920_body10_e137933) * locals.var_c_sb) - (assign89920_body10_e137934 * locals.var_c_sb_dn7)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn8 / assign89920_body10_e137933) * locals.var_c_sb) - (assign89920_body10_e137934 * locals.var_c_sb_dn8)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn9 / assign89920_body10_e137933) * locals.var_c_sb) - (assign89920_body10_e137934 * locals.var_c_sb_dn9)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn10 / assign89920_body10_e137933) * locals.var_c_sb) - (assign89920_body10_e137934 * locals.var_c_sb_dn10)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn13 / assign89920_body10_e137933) * locals.var_c_sb) - (assign89920_body10_e137934 * locals.var_c_sb_dn13)) / (locals.var_c_sb * locals.var_c_sb)),)
    } else {
        (locals.var_phi_b, locals.var_phi_b_dn0, locals.var_phi_b_dn2, locals.var_phi_b_dn4, locals.var_phi_b_dn5, locals.var_phi_b_dn6, locals.var_phi_b_dn7, locals.var_phi_b_dn8, locals.var_phi_b_dn9, locals.var_phi_b_dn10, locals.var_phi_b_dn13,)
    }
};
            locals.var_phi_b = assign89920_body10_e137938;
            locals.var_phi_b_dn0 = assign89920_body10_e137938_d_n0;
            locals.var_phi_b_dn2 = assign89920_body10_e137938_d_n2;
            locals.var_phi_b_dn4 = assign89920_body10_e137938_d_n4;
            locals.var_phi_b_dn5 = assign89920_body10_e137938_d_n5;
            locals.var_phi_b_dn6 = assign89920_body10_e137938_d_n6;
            locals.var_phi_b_dn7 = assign89920_body10_e137938_d_n7;
            locals.var_phi_b_dn8 = assign89920_body10_e137938_d_n8;
            locals.var_phi_b_dn9 = assign89920_body10_e137938_d_n9;
            locals.var_phi_b_dn10 = assign89920_body10_e137938_d_n10;
            locals.var_phi_b_dn13 = assign89920_body10_e137938_d_n13;
            locals.var_phi_b_rv = 0.0;
            let (assign89920_body11_e137950, assign89920_body11_e137950_d_n0, assign89920_body11_e137950_d_n2, assign89920_body11_e137950_d_n4, assign89920_body11_e137950_d_n5, assign89920_body11_e137950_d_n6, assign89920_body11_e137950_d_n7, assign89920_body11_e137950_d_n8, assign89920_body11_e137950_d_n9, assign89920_body11_e137950_d_n10, assign89920_body11_e137950_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) && (locals.var_guard2090 != 0.0)) {
        let assign89920_body11_e137947: f64 = (1.0 + locals.var_t2);
        let assign89920_body11_e137948: f64 = (locals.var_t1 / assign89920_body11_e137947);
        (assign89920_body11_e137948, (((locals.var_t1_dn0 * assign89920_body11_e137947) - (locals.var_t1 * locals.var_t2_dn0)) / (assign89920_body11_e137947 * assign89920_body11_e137947)), (((locals.var_t1_dn2 * assign89920_body11_e137947) - (locals.var_t1 * locals.var_t2_dn2)) / (assign89920_body11_e137947 * assign89920_body11_e137947)), (((locals.var_t1_dn4 * assign89920_body11_e137947) - (locals.var_t1 * locals.var_t2_dn4)) / (assign89920_body11_e137947 * assign89920_body11_e137947)), (((locals.var_t1_dn5 * assign89920_body11_e137947) - (locals.var_t1 * locals.var_t2_dn5)) / (assign89920_body11_e137947 * assign89920_body11_e137947)), (((locals.var_t1_dn6 * assign89920_body11_e137947) - (locals.var_t1 * locals.var_t2_dn6)) / (assign89920_body11_e137947 * assign89920_body11_e137947)), (((locals.var_t1_dn7 * assign89920_body11_e137947) - (locals.var_t1 * locals.var_t2_dn7)) / (assign89920_body11_e137947 * assign89920_body11_e137947)), (((locals.var_t1_dn8 * assign89920_body11_e137947) - (locals.var_t1 * locals.var_t2_dn8)) / (assign89920_body11_e137947 * assign89920_body11_e137947)), (((locals.var_t1_dn9 * assign89920_body11_e137947) - (locals.var_t1 * locals.var_t2_dn9)) / (assign89920_body11_e137947 * assign89920_body11_e137947)), (((locals.var_t1_dn10 * assign89920_body11_e137947) - (locals.var_t1 * locals.var_t2_dn10)) / (assign89920_body11_e137947 * assign89920_body11_e137947)), (((locals.var_t1_dn13 * assign89920_body11_e137947) - (locals.var_t1 * locals.var_t2_dn13)) / (assign89920_body11_e137947 * assign89920_body11_e137947)),)
    } else {
        (locals.var_phi_b_dpss, locals.var_phi_b_dpss_dn0, locals.var_phi_b_dpss_dn2, locals.var_phi_b_dpss_dn4, locals.var_phi_b_dpss_dn5, locals.var_phi_b_dpss_dn6, locals.var_phi_b_dpss_dn7, locals.var_phi_b_dpss_dn8, locals.var_phi_b_dpss_dn9, locals.var_phi_b_dpss_dn10, locals.var_phi_b_dpss_dn13,)
    }
};
            locals.var_phi_b_dpss = assign89920_body11_e137950;
            locals.var_phi_b_dpss_dn0 = assign89920_body11_e137950_d_n0;
            locals.var_phi_b_dpss_dn2 = assign89920_body11_e137950_d_n2;
            locals.var_phi_b_dpss_dn4 = assign89920_body11_e137950_d_n4;
            locals.var_phi_b_dpss_dn5 = assign89920_body11_e137950_d_n5;
            locals.var_phi_b_dpss_dn6 = assign89920_body11_e137950_d_n6;
            locals.var_phi_b_dpss_dn7 = assign89920_body11_e137950_d_n7;
            locals.var_phi_b_dpss_dn8 = assign89920_body11_e137950_d_n8;
            locals.var_phi_b_dpss_dn9 = assign89920_body11_e137950_d_n9;
            locals.var_phi_b_dpss_dn10 = assign89920_body11_e137950_d_n10;
            locals.var_phi_b_dpss_dn13 = assign89920_body11_e137950_d_n13;
            locals.var_phi_b_dpss_rv = 0.0;
            let (assign89920_body12_e137961, assign89920_body12_e137961_d_n0, assign89920_body12_e137961_d_n2, assign89920_body12_e137961_d_n4, assign89920_body12_e137961_d_n5, assign89920_body12_e137961_d_n6, assign89920_body12_e137961_d_n7, assign89920_body12_e137961_d_n8, assign89920_body12_e137961_d_n9, assign89920_body12_e137961_d_n10, assign89920_body12_e137961_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) && (locals.var_guard2090 == 0.0)) {
        let assign89920_body12_e137959: f64 = (locals.var_ps0ld_vxb - locals.var_dphi_sb);
        (assign89920_body12_e137959, (locals.var_ps0ld_vxb_dn0 - locals.var_dphi_sb_dn0), (locals.var_ps0ld_vxb_dn2 - locals.var_dphi_sb_dn2), (locals.var_ps0ld_vxb_dn4 - locals.var_dphi_sb_dn4), (locals.var_ps0ld_vxb_dn5 - locals.var_dphi_sb_dn5), (locals.var_ps0ld_vxb_dn6 - locals.var_dphi_sb_dn6), (locals.var_ps0ld_vxb_dn7 - locals.var_dphi_sb_dn7), (locals.var_ps0ld_vxb_dn8 - locals.var_dphi_sb_dn8), (locals.var_ps0ld_vxb_dn9 - locals.var_dphi_sb_dn9), (locals.var_ps0ld_vxb_dn10 - locals.var_dphi_sb_dn10), (locals.var_ps0ld_vxb_dn13 - locals.var_dphi_sb_dn13),)
    } else {
        (locals.var_phi_b, locals.var_phi_b_dn0, locals.var_phi_b_dn2, locals.var_phi_b_dn4, locals.var_phi_b_dn5, locals.var_phi_b_dn6, locals.var_phi_b_dn7, locals.var_phi_b_dn8, locals.var_phi_b_dn9, locals.var_phi_b_dn10, locals.var_phi_b_dn13,)
    }
};
            locals.var_phi_b = assign89920_body12_e137961;
            locals.var_phi_b_dn0 = assign89920_body12_e137961_d_n0;
            locals.var_phi_b_dn2 = assign89920_body12_e137961_d_n2;
            locals.var_phi_b_dn4 = assign89920_body12_e137961_d_n4;
            locals.var_phi_b_dn5 = assign89920_body12_e137961_d_n5;
            locals.var_phi_b_dn6 = assign89920_body12_e137961_d_n6;
            locals.var_phi_b_dn7 = assign89920_body12_e137961_d_n7;
            locals.var_phi_b_dn8 = assign89920_body12_e137961_d_n8;
            locals.var_phi_b_dn9 = assign89920_body12_e137961_d_n9;
            locals.var_phi_b_dn10 = assign89920_body12_e137961_d_n10;
            locals.var_phi_b_dn13 = assign89920_body12_e137961_d_n13;
            locals.var_phi_b_rv = 0.0;
            let (assign89920_body13_e137970, assign89920_body13_e137970_d_n0, assign89920_body13_e137970_d_n2, assign89920_body13_e137970_d_n4, assign89920_body13_e137970_d_n5, assign89920_body13_e137970_d_n6, assign89920_body13_e137970_d_n7, assign89920_body13_e137970_d_n8, assign89920_body13_e137970_d_n9, assign89920_body13_e137970_d_n10, assign89920_body13_e137970_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) && (locals.var_guard2090 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_phi_b_dpss, locals.var_phi_b_dpss_dn0, locals.var_phi_b_dpss_dn2, locals.var_phi_b_dpss_dn4, locals.var_phi_b_dpss_dn5, locals.var_phi_b_dpss_dn6, locals.var_phi_b_dpss_dn7, locals.var_phi_b_dpss_dn8, locals.var_phi_b_dpss_dn9, locals.var_phi_b_dpss_dn10, locals.var_phi_b_dpss_dn13,)
    }
};
            locals.var_phi_b_dpss = assign89920_body13_e137970;
            locals.var_phi_b_dpss_dn0 = assign89920_body13_e137970_d_n0;
            locals.var_phi_b_dpss_dn2 = assign89920_body13_e137970_d_n2;
            locals.var_phi_b_dpss_dn4 = assign89920_body13_e137970_d_n4;
            locals.var_phi_b_dpss_dn5 = assign89920_body13_e137970_d_n5;
            locals.var_phi_b_dpss_dn6 = assign89920_body13_e137970_d_n6;
            locals.var_phi_b_dpss_dn7 = assign89920_body13_e137970_d_n7;
            locals.var_phi_b_dpss_dn8 = assign89920_body13_e137970_d_n8;
            locals.var_phi_b_dpss_dn9 = assign89920_body13_e137970_d_n9;
            locals.var_phi_b_dpss_dn10 = assign89920_body13_e137970_d_n10;
            locals.var_phi_b_dpss_dn13 = assign89920_body13_e137970_d_n13;
            locals.var_phi_b_dpss_rv = 0.0;
            let (assign89920_body14_e137978, assign89920_body14_e137978_d_n0, assign89920_body14_e137978_d_n2, assign89920_body14_e137978_d_n4, assign89920_body14_e137978_d_n5, assign89920_body14_e137978_d_n6, assign89920_body14_e137978_d_n7, assign89920_body14_e137978_d_n8, assign89920_body14_e137978_d_n9, assign89920_body14_e137978_d_n10, assign89920_body14_e137978_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) {
        let assign89920_body14_e137976: f64 = (locals.var_beta * locals.var_phi_b);
        (assign89920_body14_e137976, ((locals.var_beta_dn0 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn0)), ((locals.var_beta_dn2 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn2)), ((locals.var_beta_dn4 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn4)), ((locals.var_beta_dn5 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn5)), ((locals.var_beta_dn6 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn6)), ((locals.var_beta_dn7 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn7)), ((locals.var_beta_dn8 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn8)), ((locals.var_beta_dn9 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn9)), ((locals.var_beta_dn10 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn10)), ((locals.var_beta_dn13 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn13)),)
    } else {
        (locals.var_chib, locals.var_chib_dn0, locals.var_chib_dn2, locals.var_chib_dn4, locals.var_chib_dn5, locals.var_chib_dn6, locals.var_chib_dn7, locals.var_chib_dn8, locals.var_chib_dn9, locals.var_chib_dn10, locals.var_chib_dn13,)
    }
};
            locals.var_chib = assign89920_body14_e137978;
            locals.var_chib_dn0 = assign89920_body14_e137978_d_n0;
            locals.var_chib_dn2 = assign89920_body14_e137978_d_n2;
            locals.var_chib_dn4 = assign89920_body14_e137978_d_n4;
            locals.var_chib_dn5 = assign89920_body14_e137978_d_n5;
            locals.var_chib_dn6 = assign89920_body14_e137978_d_n6;
            locals.var_chib_dn7 = assign89920_body14_e137978_d_n7;
            locals.var_chib_dn8 = assign89920_body14_e137978_d_n8;
            locals.var_chib_dn9 = assign89920_body14_e137978_d_n9;
            locals.var_chib_dn10 = assign89920_body14_e137978_d_n10;
            locals.var_chib_dn13 = assign89920_body14_e137978_d_n13;
            locals.var_chib_rv = 0.0;
            let assign89920_body15_e137980: f64 = (locals.var_chi).abs();
            let assign89920_body15_e137982: f64 = if assign89920_body15_e137980 < 1e-6 { 1.0 } else { 0.0 };
            locals.var_guard2091 = assign89920_body15_e137982;
            locals.var_guard2091_rv = 0.0;
            let (assign89920_body17_e138028, assign89920_body17_e138028_d_n0, assign89920_body17_e138028_d_n2, assign89920_body17_e138028_d_n4, assign89920_body17_e138028_d_n5, assign89920_body17_e138028_d_n6, assign89920_body17_e138028_d_n7, assign89920_body17_e138028_d_n8, assign89920_body17_e138028_d_n9, assign89920_body17_e138028_d_n10, assign89920_body17_e138028_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) && (locals.var_guard2091 != 0.0)) {
        let assign89920_body17_e138006: f64 = (locals.var_chi * locals.var_chi);
        let assign89920_body17_e138008: f64 = (assign89920_body17_e138006 / 2.0);
        let assign89920_body17_e138012: f64 = (locals.var_chi / 3.0);
        let assign89920_body17_e138016: f64 = (locals.var_chi / 4.0);
        let assign89920_body17_e138020: f64 = (locals.var_chi / 5.0);
        let assign89920_body17_e138021: f64 = (1.0 - assign89920_body17_e138020);
        let assign89920_body17_e138022: f64 = (assign89920_body17_e138016 * assign89920_body17_e138021);
        let assign89920_body17_e138023: f64 = (1.0 - assign89920_body17_e138022);
        let assign89920_body17_e138024: f64 = (assign89920_body17_e138012 * assign89920_body17_e138023);
        let assign89920_body17_e138025: f64 = (1.0 - assign89920_body17_e138024);
        let assign89920_body17_e138026: f64 = (assign89920_body17_e138008 * assign89920_body17_e138025);
        (assign89920_body17_e138026, (((((locals.var_chi_dn0 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn0)) / 2.0) * assign89920_body17_e138025) + (assign89920_body17_e138008 * (-(((locals.var_chi_dn0 / 3.0) * assign89920_body17_e138023) + (assign89920_body17_e138012 * (-(((locals.var_chi_dn0 / 4.0) * assign89920_body17_e138021) + (assign89920_body17_e138016 * (-(locals.var_chi_dn0 / 5.0)))))))))), (((((locals.var_chi_dn2 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn2)) / 2.0) * assign89920_body17_e138025) + (assign89920_body17_e138008 * (-(((locals.var_chi_dn2 / 3.0) * assign89920_body17_e138023) + (assign89920_body17_e138012 * (-(((locals.var_chi_dn2 / 4.0) * assign89920_body17_e138021) + (assign89920_body17_e138016 * (-(locals.var_chi_dn2 / 5.0)))))))))), (((((locals.var_chi_dn4 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn4)) / 2.0) * assign89920_body17_e138025) + (assign89920_body17_e138008 * (-(((locals.var_chi_dn4 / 3.0) * assign89920_body17_e138023) + (assign89920_body17_e138012 * (-(((locals.var_chi_dn4 / 4.0) * assign89920_body17_e138021) + (assign89920_body17_e138016 * (-(locals.var_chi_dn4 / 5.0)))))))))), (((((locals.var_chi_dn5 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn5)) / 2.0) * assign89920_body17_e138025) + (assign89920_body17_e138008 * (-(((locals.var_chi_dn5 / 3.0) * assign89920_body17_e138023) + (assign89920_body17_e138012 * (-(((locals.var_chi_dn5 / 4.0) * assign89920_body17_e138021) + (assign89920_body17_e138016 * (-(locals.var_chi_dn5 / 5.0)))))))))), (((((locals.var_chi_dn6 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn6)) / 2.0) * assign89920_body17_e138025) + (assign89920_body17_e138008 * (-(((locals.var_chi_dn6 / 3.0) * assign89920_body17_e138023) + (assign89920_body17_e138012 * (-(((locals.var_chi_dn6 / 4.0) * assign89920_body17_e138021) + (assign89920_body17_e138016 * (-(locals.var_chi_dn6 / 5.0)))))))))), (((((locals.var_chi_dn7 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn7)) / 2.0) * assign89920_body17_e138025) + (assign89920_body17_e138008 * (-(((locals.var_chi_dn7 / 3.0) * assign89920_body17_e138023) + (assign89920_body17_e138012 * (-(((locals.var_chi_dn7 / 4.0) * assign89920_body17_e138021) + (assign89920_body17_e138016 * (-(locals.var_chi_dn7 / 5.0)))))))))), (((((locals.var_chi_dn8 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn8)) / 2.0) * assign89920_body17_e138025) + (assign89920_body17_e138008 * (-(((locals.var_chi_dn8 / 3.0) * assign89920_body17_e138023) + (assign89920_body17_e138012 * (-(((locals.var_chi_dn8 / 4.0) * assign89920_body17_e138021) + (assign89920_body17_e138016 * (-(locals.var_chi_dn8 / 5.0)))))))))), (((((locals.var_chi_dn9 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn9)) / 2.0) * assign89920_body17_e138025) + (assign89920_body17_e138008 * (-(((locals.var_chi_dn9 / 3.0) * assign89920_body17_e138023) + (assign89920_body17_e138012 * (-(((locals.var_chi_dn9 / 4.0) * assign89920_body17_e138021) + (assign89920_body17_e138016 * (-(locals.var_chi_dn9 / 5.0)))))))))), (((((locals.var_chi_dn10 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn10)) / 2.0) * assign89920_body17_e138025) + (assign89920_body17_e138008 * (-(((locals.var_chi_dn10 / 3.0) * assign89920_body17_e138023) + (assign89920_body17_e138012 * (-(((locals.var_chi_dn10 / 4.0) * assign89920_body17_e138021) + (assign89920_body17_e138016 * (-(locals.var_chi_dn10 / 5.0)))))))))), (((((locals.var_chi_dn13 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn13)) / 2.0) * assign89920_body17_e138025) + (assign89920_body17_e138008 * (-(((locals.var_chi_dn13 / 3.0) * assign89920_body17_e138023) + (assign89920_body17_e138012 * (-(((locals.var_chi_dn13 / 4.0) * assign89920_body17_e138021) + (assign89920_body17_e138016 * (-(locals.var_chi_dn13 / 5.0)))))))))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
            locals.var_t0 = assign89920_body17_e138028;
            locals.var_t0_dn0 = assign89920_body17_e138028_d_n0;
            locals.var_t0_dn2 = assign89920_body17_e138028_d_n2;
            locals.var_t0_dn4 = assign89920_body17_e138028_d_n4;
            locals.var_t0_dn5 = assign89920_body17_e138028_d_n5;
            locals.var_t0_dn6 = assign89920_body17_e138028_d_n6;
            locals.var_t0_dn7 = assign89920_body17_e138028_d_n7;
            locals.var_t0_dn8 = assign89920_body17_e138028_d_n8;
            locals.var_t0_dn9 = assign89920_body17_e138028_d_n9;
            locals.var_t0_dn10 = assign89920_body17_e138028_d_n10;
            locals.var_t0_dn13 = assign89920_body17_e138028_d_n13;
            locals.var_t0_rv = 0.0;
            let (assign89920_body18_e138054, assign89920_body18_e138054_d_n0, assign89920_body18_e138054_d_n2, assign89920_body18_e138054_d_n4, assign89920_body18_e138054_d_n5, assign89920_body18_e138054_d_n6, assign89920_body18_e138054_d_n7, assign89920_body18_e138054_d_n8, assign89920_body18_e138054_d_n9, assign89920_body18_e138054_d_n10, assign89920_body18_e138054_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) && (locals.var_guard2091 != 0.0)) {
        let assign89920_body18_e138038: f64 = (locals.var_chi / 2.0);
        let assign89920_body18_e138042: f64 = (locals.var_chi / 3.0);
        let assign89920_body18_e138046: f64 = (locals.var_chi / 4.0);
        let assign89920_body18_e138047: f64 = (1.0 - assign89920_body18_e138046);
        let assign89920_body18_e138048: f64 = (assign89920_body18_e138042 * assign89920_body18_e138047);
        let assign89920_body18_e138049: f64 = (1.0 - assign89920_body18_e138048);
        let assign89920_body18_e138050: f64 = (assign89920_body18_e138038 * assign89920_body18_e138049);
        let assign89920_body18_e138051: f64 = (1.0 - assign89920_body18_e138050);
        let assign89920_body18_e138052: f64 = (locals.var_chi * assign89920_body18_e138051);
        (assign89920_body18_e138052, ((locals.var_chi_dn0 * assign89920_body18_e138051) + (locals.var_chi * (-(((locals.var_chi_dn0 / 2.0) * assign89920_body18_e138049) + (assign89920_body18_e138038 * (-(((locals.var_chi_dn0 / 3.0) * assign89920_body18_e138047) + (assign89920_body18_e138042 * (-(locals.var_chi_dn0 / 4.0)))))))))), ((locals.var_chi_dn2 * assign89920_body18_e138051) + (locals.var_chi * (-(((locals.var_chi_dn2 / 2.0) * assign89920_body18_e138049) + (assign89920_body18_e138038 * (-(((locals.var_chi_dn2 / 3.0) * assign89920_body18_e138047) + (assign89920_body18_e138042 * (-(locals.var_chi_dn2 / 4.0)))))))))), ((locals.var_chi_dn4 * assign89920_body18_e138051) + (locals.var_chi * (-(((locals.var_chi_dn4 / 2.0) * assign89920_body18_e138049) + (assign89920_body18_e138038 * (-(((locals.var_chi_dn4 / 3.0) * assign89920_body18_e138047) + (assign89920_body18_e138042 * (-(locals.var_chi_dn4 / 4.0)))))))))), ((locals.var_chi_dn5 * assign89920_body18_e138051) + (locals.var_chi * (-(((locals.var_chi_dn5 / 2.0) * assign89920_body18_e138049) + (assign89920_body18_e138038 * (-(((locals.var_chi_dn5 / 3.0) * assign89920_body18_e138047) + (assign89920_body18_e138042 * (-(locals.var_chi_dn5 / 4.0)))))))))), ((locals.var_chi_dn6 * assign89920_body18_e138051) + (locals.var_chi * (-(((locals.var_chi_dn6 / 2.0) * assign89920_body18_e138049) + (assign89920_body18_e138038 * (-(((locals.var_chi_dn6 / 3.0) * assign89920_body18_e138047) + (assign89920_body18_e138042 * (-(locals.var_chi_dn6 / 4.0)))))))))), ((locals.var_chi_dn7 * assign89920_body18_e138051) + (locals.var_chi * (-(((locals.var_chi_dn7 / 2.0) * assign89920_body18_e138049) + (assign89920_body18_e138038 * (-(((locals.var_chi_dn7 / 3.0) * assign89920_body18_e138047) + (assign89920_body18_e138042 * (-(locals.var_chi_dn7 / 4.0)))))))))), ((locals.var_chi_dn8 * assign89920_body18_e138051) + (locals.var_chi * (-(((locals.var_chi_dn8 / 2.0) * assign89920_body18_e138049) + (assign89920_body18_e138038 * (-(((locals.var_chi_dn8 / 3.0) * assign89920_body18_e138047) + (assign89920_body18_e138042 * (-(locals.var_chi_dn8 / 4.0)))))))))), ((locals.var_chi_dn9 * assign89920_body18_e138051) + (locals.var_chi * (-(((locals.var_chi_dn9 / 2.0) * assign89920_body18_e138049) + (assign89920_body18_e138038 * (-(((locals.var_chi_dn9 / 3.0) * assign89920_body18_e138047) + (assign89920_body18_e138042 * (-(locals.var_chi_dn9 / 4.0)))))))))), ((locals.var_chi_dn10 * assign89920_body18_e138051) + (locals.var_chi * (-(((locals.var_chi_dn10 / 2.0) * assign89920_body18_e138049) + (assign89920_body18_e138038 * (-(((locals.var_chi_dn10 / 3.0) * assign89920_body18_e138047) + (assign89920_body18_e138042 * (-(locals.var_chi_dn10 / 4.0)))))))))), ((locals.var_chi_dn13 * assign89920_body18_e138051) + (locals.var_chi * (-(((locals.var_chi_dn13 / 2.0) * assign89920_body18_e138049) + (assign89920_body18_e138038 * (-(((locals.var_chi_dn13 / 3.0) * assign89920_body18_e138047) + (assign89920_body18_e138042 * (-(locals.var_chi_dn13 / 4.0)))))))))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
            locals.var_t1 = assign89920_body18_e138054;
            locals.var_t1_dn0 = assign89920_body18_e138054_d_n0;
            locals.var_t1_dn2 = assign89920_body18_e138054_d_n2;
            locals.var_t1_dn4 = assign89920_body18_e138054_d_n4;
            locals.var_t1_dn5 = assign89920_body18_e138054_d_n5;
            locals.var_t1_dn6 = assign89920_body18_e138054_d_n6;
            locals.var_t1_dn7 = assign89920_body18_e138054_d_n7;
            locals.var_t1_dn8 = assign89920_body18_e138054_d_n8;
            locals.var_t1_dn9 = assign89920_body18_e138054_d_n9;
            locals.var_t1_dn10 = assign89920_body18_e138054_d_n10;
            locals.var_t1_dn13 = assign89920_body18_e138054_d_n13;
            locals.var_t1_rv = 0.0;
            let (assign89920_body19_e138084, assign89920_body19_e138084_d_n0, assign89920_body19_e138084_d_n2, assign89920_body19_e138084_d_n4, assign89920_body19_e138084_d_n5, assign89920_body19_e138084_d_n6, assign89920_body19_e138084_d_n7, assign89920_body19_e138084_d_n8, assign89920_body19_e138084_d_n9, assign89920_body19_e138084_d_n10, assign89920_body19_e138084_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) && (locals.var_guard2091 != 0.0)) {
        let assign89920_body19_e138062: f64 = (locals.var_chib * locals.var_chib);
        let assign89920_body19_e138064: f64 = (assign89920_body19_e138062 / 2.0);
        let assign89920_body19_e138068: f64 = (locals.var_chib / 3.0);
        let assign89920_body19_e138072: f64 = (locals.var_chib / 4.0);
        let assign89920_body19_e138076: f64 = (locals.var_chib / 5.0);
        let assign89920_body19_e138077: f64 = (1.0 - assign89920_body19_e138076);
        let assign89920_body19_e138078: f64 = (assign89920_body19_e138072 * assign89920_body19_e138077);
        let assign89920_body19_e138079: f64 = (1.0 - assign89920_body19_e138078);
        let assign89920_body19_e138080: f64 = (assign89920_body19_e138068 * assign89920_body19_e138079);
        let assign89920_body19_e138081: f64 = (1.0 - assign89920_body19_e138080);
        let assign89920_body19_e138082: f64 = (assign89920_body19_e138064 * assign89920_body19_e138081);
        (assign89920_body19_e138082, (((((locals.var_chib_dn0 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn0)) / 2.0) * assign89920_body19_e138081) + (assign89920_body19_e138064 * (-(((locals.var_chib_dn0 / 3.0) * assign89920_body19_e138079) + (assign89920_body19_e138068 * (-(((locals.var_chib_dn0 / 4.0) * assign89920_body19_e138077) + (assign89920_body19_e138072 * (-(locals.var_chib_dn0 / 5.0)))))))))), (((((locals.var_chib_dn2 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn2)) / 2.0) * assign89920_body19_e138081) + (assign89920_body19_e138064 * (-(((locals.var_chib_dn2 / 3.0) * assign89920_body19_e138079) + (assign89920_body19_e138068 * (-(((locals.var_chib_dn2 / 4.0) * assign89920_body19_e138077) + (assign89920_body19_e138072 * (-(locals.var_chib_dn2 / 5.0)))))))))), (((((locals.var_chib_dn4 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn4)) / 2.0) * assign89920_body19_e138081) + (assign89920_body19_e138064 * (-(((locals.var_chib_dn4 / 3.0) * assign89920_body19_e138079) + (assign89920_body19_e138068 * (-(((locals.var_chib_dn4 / 4.0) * assign89920_body19_e138077) + (assign89920_body19_e138072 * (-(locals.var_chib_dn4 / 5.0)))))))))), (((((locals.var_chib_dn5 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn5)) / 2.0) * assign89920_body19_e138081) + (assign89920_body19_e138064 * (-(((locals.var_chib_dn5 / 3.0) * assign89920_body19_e138079) + (assign89920_body19_e138068 * (-(((locals.var_chib_dn5 / 4.0) * assign89920_body19_e138077) + (assign89920_body19_e138072 * (-(locals.var_chib_dn5 / 5.0)))))))))), (((((locals.var_chib_dn6 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn6)) / 2.0) * assign89920_body19_e138081) + (assign89920_body19_e138064 * (-(((locals.var_chib_dn6 / 3.0) * assign89920_body19_e138079) + (assign89920_body19_e138068 * (-(((locals.var_chib_dn6 / 4.0) * assign89920_body19_e138077) + (assign89920_body19_e138072 * (-(locals.var_chib_dn6 / 5.0)))))))))), (((((locals.var_chib_dn7 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn7)) / 2.0) * assign89920_body19_e138081) + (assign89920_body19_e138064 * (-(((locals.var_chib_dn7 / 3.0) * assign89920_body19_e138079) + (assign89920_body19_e138068 * (-(((locals.var_chib_dn7 / 4.0) * assign89920_body19_e138077) + (assign89920_body19_e138072 * (-(locals.var_chib_dn7 / 5.0)))))))))), (((((locals.var_chib_dn8 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn8)) / 2.0) * assign89920_body19_e138081) + (assign89920_body19_e138064 * (-(((locals.var_chib_dn8 / 3.0) * assign89920_body19_e138079) + (assign89920_body19_e138068 * (-(((locals.var_chib_dn8 / 4.0) * assign89920_body19_e138077) + (assign89920_body19_e138072 * (-(locals.var_chib_dn8 / 5.0)))))))))), (((((locals.var_chib_dn9 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn9)) / 2.0) * assign89920_body19_e138081) + (assign89920_body19_e138064 * (-(((locals.var_chib_dn9 / 3.0) * assign89920_body19_e138079) + (assign89920_body19_e138068 * (-(((locals.var_chib_dn9 / 4.0) * assign89920_body19_e138077) + (assign89920_body19_e138072 * (-(locals.var_chib_dn9 / 5.0)))))))))), (((((locals.var_chib_dn10 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn10)) / 2.0) * assign89920_body19_e138081) + (assign89920_body19_e138064 * (-(((locals.var_chib_dn10 / 3.0) * assign89920_body19_e138079) + (assign89920_body19_e138068 * (-(((locals.var_chib_dn10 / 4.0) * assign89920_body19_e138077) + (assign89920_body19_e138072 * (-(locals.var_chib_dn10 / 5.0)))))))))), (((((locals.var_chib_dn13 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn13)) / 2.0) * assign89920_body19_e138081) + (assign89920_body19_e138064 * (-(((locals.var_chib_dn13 / 3.0) * assign89920_body19_e138079) + (assign89920_body19_e138068 * (-(((locals.var_chib_dn13 / 4.0) * assign89920_body19_e138077) + (assign89920_body19_e138072 * (-(locals.var_chib_dn13 / 5.0)))))))))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
            locals.var_t2 = assign89920_body19_e138084;
            locals.var_t2_dn0 = assign89920_body19_e138084_d_n0;
            locals.var_t2_dn2 = assign89920_body19_e138084_d_n2;
            locals.var_t2_dn4 = assign89920_body19_e138084_d_n4;
            locals.var_t2_dn5 = assign89920_body19_e138084_d_n5;
            locals.var_t2_dn6 = assign89920_body19_e138084_d_n6;
            locals.var_t2_dn7 = assign89920_body19_e138084_d_n7;
            locals.var_t2_dn8 = assign89920_body19_e138084_d_n8;
            locals.var_t2_dn9 = assign89920_body19_e138084_d_n9;
            locals.var_t2_dn10 = assign89920_body19_e138084_d_n10;
            locals.var_t2_dn13 = assign89920_body19_e138084_d_n13;
            locals.var_t2_rv = 0.0;
            let (assign89920_body20_e138110, assign89920_body20_e138110_d_n0, assign89920_body20_e138110_d_n2, assign89920_body20_e138110_d_n4, assign89920_body20_e138110_d_n5, assign89920_body20_e138110_d_n6, assign89920_body20_e138110_d_n7, assign89920_body20_e138110_d_n8, assign89920_body20_e138110_d_n9, assign89920_body20_e138110_d_n10, assign89920_body20_e138110_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) && (locals.var_guard2091 != 0.0)) {
        let assign89920_body20_e138094: f64 = (locals.var_chib / 2.0);
        let assign89920_body20_e138098: f64 = (locals.var_chib / 3.0);
        let assign89920_body20_e138102: f64 = (locals.var_chib / 4.0);
        let assign89920_body20_e138103: f64 = (1.0 - assign89920_body20_e138102);
        let assign89920_body20_e138104: f64 = (assign89920_body20_e138098 * assign89920_body20_e138103);
        let assign89920_body20_e138105: f64 = (1.0 - assign89920_body20_e138104);
        let assign89920_body20_e138106: f64 = (assign89920_body20_e138094 * assign89920_body20_e138105);
        let assign89920_body20_e138107: f64 = (1.0 - assign89920_body20_e138106);
        let assign89920_body20_e138108: f64 = (locals.var_chib * assign89920_body20_e138107);
        (assign89920_body20_e138108, ((locals.var_chib_dn0 * assign89920_body20_e138107) + (locals.var_chib * (-(((locals.var_chib_dn0 / 2.0) * assign89920_body20_e138105) + (assign89920_body20_e138094 * (-(((locals.var_chib_dn0 / 3.0) * assign89920_body20_e138103) + (assign89920_body20_e138098 * (-(locals.var_chib_dn0 / 4.0)))))))))), ((locals.var_chib_dn2 * assign89920_body20_e138107) + (locals.var_chib * (-(((locals.var_chib_dn2 / 2.0) * assign89920_body20_e138105) + (assign89920_body20_e138094 * (-(((locals.var_chib_dn2 / 3.0) * assign89920_body20_e138103) + (assign89920_body20_e138098 * (-(locals.var_chib_dn2 / 4.0)))))))))), ((locals.var_chib_dn4 * assign89920_body20_e138107) + (locals.var_chib * (-(((locals.var_chib_dn4 / 2.0) * assign89920_body20_e138105) + (assign89920_body20_e138094 * (-(((locals.var_chib_dn4 / 3.0) * assign89920_body20_e138103) + (assign89920_body20_e138098 * (-(locals.var_chib_dn4 / 4.0)))))))))), ((locals.var_chib_dn5 * assign89920_body20_e138107) + (locals.var_chib * (-(((locals.var_chib_dn5 / 2.0) * assign89920_body20_e138105) + (assign89920_body20_e138094 * (-(((locals.var_chib_dn5 / 3.0) * assign89920_body20_e138103) + (assign89920_body20_e138098 * (-(locals.var_chib_dn5 / 4.0)))))))))), ((locals.var_chib_dn6 * assign89920_body20_e138107) + (locals.var_chib * (-(((locals.var_chib_dn6 / 2.0) * assign89920_body20_e138105) + (assign89920_body20_e138094 * (-(((locals.var_chib_dn6 / 3.0) * assign89920_body20_e138103) + (assign89920_body20_e138098 * (-(locals.var_chib_dn6 / 4.0)))))))))), ((locals.var_chib_dn7 * assign89920_body20_e138107) + (locals.var_chib * (-(((locals.var_chib_dn7 / 2.0) * assign89920_body20_e138105) + (assign89920_body20_e138094 * (-(((locals.var_chib_dn7 / 3.0) * assign89920_body20_e138103) + (assign89920_body20_e138098 * (-(locals.var_chib_dn7 / 4.0)))))))))), ((locals.var_chib_dn8 * assign89920_body20_e138107) + (locals.var_chib * (-(((locals.var_chib_dn8 / 2.0) * assign89920_body20_e138105) + (assign89920_body20_e138094 * (-(((locals.var_chib_dn8 / 3.0) * assign89920_body20_e138103) + (assign89920_body20_e138098 * (-(locals.var_chib_dn8 / 4.0)))))))))), ((locals.var_chib_dn9 * assign89920_body20_e138107) + (locals.var_chib * (-(((locals.var_chib_dn9 / 2.0) * assign89920_body20_e138105) + (assign89920_body20_e138094 * (-(((locals.var_chib_dn9 / 3.0) * assign89920_body20_e138103) + (assign89920_body20_e138098 * (-(locals.var_chib_dn9 / 4.0)))))))))), ((locals.var_chib_dn10 * assign89920_body20_e138107) + (locals.var_chib * (-(((locals.var_chib_dn10 / 2.0) * assign89920_body20_e138105) + (assign89920_body20_e138094 * (-(((locals.var_chib_dn10 / 3.0) * assign89920_body20_e138103) + (assign89920_body20_e138098 * (-(locals.var_chib_dn10 / 4.0)))))))))), ((locals.var_chib_dn13 * assign89920_body20_e138107) + (locals.var_chib * (-(((locals.var_chib_dn13 / 2.0) * assign89920_body20_e138105) + (assign89920_body20_e138094 * (-(((locals.var_chib_dn13 / 3.0) * assign89920_body20_e138103) + (assign89920_body20_e138098 * (-(locals.var_chib_dn13 / 4.0)))))))))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
            locals.var_t3 = assign89920_body20_e138110;
            locals.var_t3_dn0 = assign89920_body20_e138110_d_n0;
            locals.var_t3_dn2 = assign89920_body20_e138110_d_n2;
            locals.var_t3_dn4 = assign89920_body20_e138110_d_n4;
            locals.var_t3_dn5 = assign89920_body20_e138110_d_n5;
            locals.var_t3_dn6 = assign89920_body20_e138110_d_n6;
            locals.var_t3_dn7 = assign89920_body20_e138110_d_n7;
            locals.var_t3_dn8 = assign89920_body20_e138110_d_n8;
            locals.var_t3_dn9 = assign89920_body20_e138110_d_n9;
            locals.var_t3_dn10 = assign89920_body20_e138110_d_n10;
            locals.var_t3_dn13 = assign89920_body20_e138110_d_n13;
            locals.var_t3_rv = 0.0;
            let (assign89920_body21_e138120, assign89920_body21_e138120_d_n0, assign89920_body21_e138120_d_n2, assign89920_body21_e138120_d_n4, assign89920_body21_e138120_d_n5, assign89920_body21_e138120_d_n6, assign89920_body21_e138120_d_n7, assign89920_body21_e138120_d_n8, assign89920_body21_e138120_d_n9, assign89920_body21_e138120_d_n10, assign89920_body21_e138120_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) && (locals.var_guard2091 != 0.0)) {
        let assign89920_body21_e138118: f64 = (locals.var_t0 - locals.var_t2);
        (assign89920_body21_e138118, (locals.var_t0_dn0 - locals.var_t2_dn0), (locals.var_t0_dn2 - locals.var_t2_dn2), (locals.var_t0_dn4 - locals.var_t2_dn4), (locals.var_t0_dn5 - locals.var_t2_dn5), (locals.var_t0_dn6 - locals.var_t2_dn6), (locals.var_t0_dn7 - locals.var_t2_dn7), (locals.var_t0_dn8 - locals.var_t2_dn8), (locals.var_t0_dn9 - locals.var_t2_dn9), (locals.var_t0_dn10 - locals.var_t2_dn10), (locals.var_t0_dn13 - locals.var_t2_dn13),)
    } else {
        (locals.var_fbsq__blk2012, locals.var_fbsq__blk2012_dn0, locals.var_fbsq__blk2012_dn2, locals.var_fbsq__blk2012_dn4, locals.var_fbsq__blk2012_dn5, locals.var_fbsq__blk2012_dn6, locals.var_fbsq__blk2012_dn7, locals.var_fbsq__blk2012_dn8, locals.var_fbsq__blk2012_dn9, locals.var_fbsq__blk2012_dn10, locals.var_fbsq__blk2012_dn13,)
    }
};
            locals.var_fbsq__blk2012 = assign89920_body21_e138120;
            locals.var_fbsq__blk2012_dn0 = assign89920_body21_e138120_d_n0;
            locals.var_fbsq__blk2012_dn2 = assign89920_body21_e138120_d_n2;
            locals.var_fbsq__blk2012_dn4 = assign89920_body21_e138120_d_n4;
            locals.var_fbsq__blk2012_dn5 = assign89920_body21_e138120_d_n5;
            locals.var_fbsq__blk2012_dn6 = assign89920_body21_e138120_d_n6;
            locals.var_fbsq__blk2012_dn7 = assign89920_body21_e138120_d_n7;
            locals.var_fbsq__blk2012_dn8 = assign89920_body21_e138120_d_n8;
            locals.var_fbsq__blk2012_dn9 = assign89920_body21_e138120_d_n9;
            locals.var_fbsq__blk2012_dn10 = assign89920_body21_e138120_d_n10;
            locals.var_fbsq__blk2012_dn13 = assign89920_body21_e138120_d_n13;
            locals.var_fbsq__blk2012_rv = 0.0;
            let (assign89920_body22_e138134, assign89920_body22_e138134_d_n0, assign89920_body22_e138134_d_n2, assign89920_body22_e138134_d_n4, assign89920_body22_e138134_d_n5, assign89920_body22_e138134_d_n6, assign89920_body22_e138134_d_n7, assign89920_body22_e138134_d_n8, assign89920_body22_e138134_d_n9, assign89920_body22_e138134_d_n10, assign89920_body22_e138134_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) && (locals.var_guard2091 != 0.0)) {
        let assign89920_body22_e138130: f64 = (locals.var_phi_b_dpss * locals.var_t3);
        let assign89920_body22_e138131: f64 = (locals.var_t1 - assign89920_body22_e138130);
        let assign89920_body22_e138132: f64 = (locals.var_beta * assign89920_body22_e138131);
        (assign89920_body22_e138132, ((locals.var_beta_dn0 * assign89920_body22_e138131) + (locals.var_beta * (locals.var_t1_dn0 - ((locals.var_phi_b_dpss_dn0 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn0))))), ((locals.var_beta_dn2 * assign89920_body22_e138131) + (locals.var_beta * (locals.var_t1_dn2 - ((locals.var_phi_b_dpss_dn2 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn2))))), ((locals.var_beta_dn4 * assign89920_body22_e138131) + (locals.var_beta * (locals.var_t1_dn4 - ((locals.var_phi_b_dpss_dn4 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn4))))), ((locals.var_beta_dn5 * assign89920_body22_e138131) + (locals.var_beta * (locals.var_t1_dn5 - ((locals.var_phi_b_dpss_dn5 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn5))))), ((locals.var_beta_dn6 * assign89920_body22_e138131) + (locals.var_beta * (locals.var_t1_dn6 - ((locals.var_phi_b_dpss_dn6 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn6))))), ((locals.var_beta_dn7 * assign89920_body22_e138131) + (locals.var_beta * (locals.var_t1_dn7 - ((locals.var_phi_b_dpss_dn7 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn7))))), ((locals.var_beta_dn8 * assign89920_body22_e138131) + (locals.var_beta * (locals.var_t1_dn8 - ((locals.var_phi_b_dpss_dn8 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn8))))), ((locals.var_beta_dn9 * assign89920_body22_e138131) + (locals.var_beta * (locals.var_t1_dn9 - ((locals.var_phi_b_dpss_dn9 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn9))))), ((locals.var_beta_dn10 * assign89920_body22_e138131) + (locals.var_beta * (locals.var_t1_dn10 - ((locals.var_phi_b_dpss_dn10 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn10))))), ((locals.var_beta_dn13 * assign89920_body22_e138131) + (locals.var_beta * (locals.var_t1_dn13 - ((locals.var_phi_b_dpss_dn13 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn13))))),)
    } else {
        (locals.var_fbsq_dpss__blk2013, locals.var_fbsq_dpss__blk2013_dn0, locals.var_fbsq_dpss__blk2013_dn2, locals.var_fbsq_dpss__blk2013_dn4, locals.var_fbsq_dpss__blk2013_dn5, locals.var_fbsq_dpss__blk2013_dn6, locals.var_fbsq_dpss__blk2013_dn7, locals.var_fbsq_dpss__blk2013_dn8, locals.var_fbsq_dpss__blk2013_dn9, locals.var_fbsq_dpss__blk2013_dn10, locals.var_fbsq_dpss__blk2013_dn13,)
    }
};
            locals.var_fbsq_dpss__blk2013 = assign89920_body22_e138134;
            locals.var_fbsq_dpss__blk2013_dn0 = assign89920_body22_e138134_d_n0;
            locals.var_fbsq_dpss__blk2013_dn2 = assign89920_body22_e138134_d_n2;
            locals.var_fbsq_dpss__blk2013_dn4 = assign89920_body22_e138134_d_n4;
            locals.var_fbsq_dpss__blk2013_dn5 = assign89920_body22_e138134_d_n5;
            locals.var_fbsq_dpss__blk2013_dn6 = assign89920_body22_e138134_d_n6;
            locals.var_fbsq_dpss__blk2013_dn7 = assign89920_body22_e138134_d_n7;
            locals.var_fbsq_dpss__blk2013_dn8 = assign89920_body22_e138134_d_n8;
            locals.var_fbsq_dpss__blk2013_dn9 = assign89920_body22_e138134_d_n9;
            locals.var_fbsq_dpss__blk2013_dn10 = assign89920_body22_e138134_d_n10;
            locals.var_fbsq_dpss__blk2013_dn13 = assign89920_body22_e138134_d_n13;
            locals.var_fbsq_dpss__blk2013_rv = 0.0;
            let (assign89920_body24_e138162, assign89920_body24_e138162_d_n0, assign89920_body24_e138162_d_n2, assign89920_body24_e138162_d_n4, assign89920_body24_e138162_d_n5, assign89920_body24_e138162_d_n6, assign89920_body24_e138162_d_n7, assign89920_body24_e138162_d_n8, assign89920_body24_e138162_d_n9, assign89920_body24_e138162_d_n10, assign89920_body24_e138162_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) && (locals.var_guard2091 == 0.0)) {
        let assign89920_body24_e138159: f64 = (-locals.var_chi);
        let assign89920_body24_e138160: f64 = (assign89920_body24_e138159).exp();
        (assign89920_body24_e138160, (assign89920_body24_e138160 * (-locals.var_chi_dn0)), (assign89920_body24_e138160 * (-locals.var_chi_dn2)), (assign89920_body24_e138160 * (-locals.var_chi_dn4)), (assign89920_body24_e138160 * (-locals.var_chi_dn5)), (assign89920_body24_e138160 * (-locals.var_chi_dn6)), (assign89920_body24_e138160 * (-locals.var_chi_dn7)), (assign89920_body24_e138160 * (-locals.var_chi_dn8)), (assign89920_body24_e138160 * (-locals.var_chi_dn9)), (assign89920_body24_e138160 * (-locals.var_chi_dn10)), (assign89920_body24_e138160 * (-locals.var_chi_dn13)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
            locals.var_t0 = assign89920_body24_e138162;
            locals.var_t0_dn0 = assign89920_body24_e138162_d_n0;
            locals.var_t0_dn2 = assign89920_body24_e138162_d_n2;
            locals.var_t0_dn4 = assign89920_body24_e138162_d_n4;
            locals.var_t0_dn5 = assign89920_body24_e138162_d_n5;
            locals.var_t0_dn6 = assign89920_body24_e138162_d_n6;
            locals.var_t0_dn7 = assign89920_body24_e138162_d_n7;
            locals.var_t0_dn8 = assign89920_body24_e138162_d_n8;
            locals.var_t0_dn9 = assign89920_body24_e138162_d_n9;
            locals.var_t0_dn10 = assign89920_body24_e138162_d_n10;
            locals.var_t0_dn13 = assign89920_body24_e138162_d_n13;
            locals.var_t0_rv = 0.0;
            let (assign89920_body25_e138173, assign89920_body25_e138173_d_n0, assign89920_body25_e138173_d_n2, assign89920_body25_e138173_d_n4, assign89920_body25_e138173_d_n5, assign89920_body25_e138173_d_n6, assign89920_body25_e138173_d_n7, assign89920_body25_e138173_d_n8, assign89920_body25_e138173_d_n9, assign89920_body25_e138173_d_n10, assign89920_body25_e138173_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) && (locals.var_guard2091 == 0.0)) {
        let assign89920_body25_e138170: f64 = (-locals.var_chib);
        let assign89920_body25_e138171: f64 = (assign89920_body25_e138170).exp();
        (assign89920_body25_e138171, (assign89920_body25_e138171 * (-locals.var_chib_dn0)), (assign89920_body25_e138171 * (-locals.var_chib_dn2)), (assign89920_body25_e138171 * (-locals.var_chib_dn4)), (assign89920_body25_e138171 * (-locals.var_chib_dn5)), (assign89920_body25_e138171 * (-locals.var_chib_dn6)), (assign89920_body25_e138171 * (-locals.var_chib_dn7)), (assign89920_body25_e138171 * (-locals.var_chib_dn8)), (assign89920_body25_e138171 * (-locals.var_chib_dn9)), (assign89920_body25_e138171 * (-locals.var_chib_dn10)), (assign89920_body25_e138171 * (-locals.var_chib_dn13)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
            locals.var_t1 = assign89920_body25_e138173;
            locals.var_t1_dn0 = assign89920_body25_e138173_d_n0;
            locals.var_t1_dn2 = assign89920_body25_e138173_d_n2;
            locals.var_t1_dn4 = assign89920_body25_e138173_d_n4;
            locals.var_t1_dn5 = assign89920_body25_e138173_d_n5;
            locals.var_t1_dn6 = assign89920_body25_e138173_d_n6;
            locals.var_t1_dn7 = assign89920_body25_e138173_d_n7;
            locals.var_t1_dn8 = assign89920_body25_e138173_d_n8;
            locals.var_t1_dn9 = assign89920_body25_e138173_d_n9;
            locals.var_t1_dn10 = assign89920_body25_e138173_d_n10;
            locals.var_t1_dn13 = assign89920_body25_e138173_d_n13;
            locals.var_t1_rv = 0.0;
            let (assign89920_body26_e138188, assign89920_body26_e138188_d_n0, assign89920_body26_e138188_d_n2, assign89920_body26_e138188_d_n4, assign89920_body26_e138188_d_n5, assign89920_body26_e138188_d_n6, assign89920_body26_e138188_d_n7, assign89920_body26_e138188_d_n8, assign89920_body26_e138188_d_n9, assign89920_body26_e138188_d_n10, assign89920_body26_e138188_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) && (locals.var_guard2091 == 0.0)) {
        let assign89920_body26_e138182: f64 = (locals.var_chi - locals.var_chib);
        let assign89920_body26_e138185: f64 = (locals.var_t0 - locals.var_t1);
        let assign89920_body26_e138186: f64 = (assign89920_body26_e138182 + assign89920_body26_e138185);
        (assign89920_body26_e138186, ((locals.var_chi_dn0 - locals.var_chib_dn0) + (locals.var_t0_dn0 - locals.var_t1_dn0)), ((locals.var_chi_dn2 - locals.var_chib_dn2) + (locals.var_t0_dn2 - locals.var_t1_dn2)), ((locals.var_chi_dn4 - locals.var_chib_dn4) + (locals.var_t0_dn4 - locals.var_t1_dn4)), ((locals.var_chi_dn5 - locals.var_chib_dn5) + (locals.var_t0_dn5 - locals.var_t1_dn5)), ((locals.var_chi_dn6 - locals.var_chib_dn6) + (locals.var_t0_dn6 - locals.var_t1_dn6)), ((locals.var_chi_dn7 - locals.var_chib_dn7) + (locals.var_t0_dn7 - locals.var_t1_dn7)), ((locals.var_chi_dn8 - locals.var_chib_dn8) + (locals.var_t0_dn8 - locals.var_t1_dn8)), ((locals.var_chi_dn9 - locals.var_chib_dn9) + (locals.var_t0_dn9 - locals.var_t1_dn9)), ((locals.var_chi_dn10 - locals.var_chib_dn10) + (locals.var_t0_dn10 - locals.var_t1_dn10)), ((locals.var_chi_dn13 - locals.var_chib_dn13) + (locals.var_t0_dn13 - locals.var_t1_dn13)),)
    } else {
        (locals.var_fbsq__blk2012, locals.var_fbsq__blk2012_dn0, locals.var_fbsq__blk2012_dn2, locals.var_fbsq__blk2012_dn4, locals.var_fbsq__blk2012_dn5, locals.var_fbsq__blk2012_dn6, locals.var_fbsq__blk2012_dn7, locals.var_fbsq__blk2012_dn8, locals.var_fbsq__blk2012_dn9, locals.var_fbsq__blk2012_dn10, locals.var_fbsq__blk2012_dn13,)
    }
};
            locals.var_fbsq__blk2012 = assign89920_body26_e138188;
            locals.var_fbsq__blk2012_dn0 = assign89920_body26_e138188_d_n0;
            locals.var_fbsq__blk2012_dn2 = assign89920_body26_e138188_d_n2;
            locals.var_fbsq__blk2012_dn4 = assign89920_body26_e138188_d_n4;
            locals.var_fbsq__blk2012_dn5 = assign89920_body26_e138188_d_n5;
            locals.var_fbsq__blk2012_dn6 = assign89920_body26_e138188_d_n6;
            locals.var_fbsq__blk2012_dn7 = assign89920_body26_e138188_d_n7;
            locals.var_fbsq__blk2012_dn8 = assign89920_body26_e138188_d_n8;
            locals.var_fbsq__blk2012_dn9 = assign89920_body26_e138188_d_n9;
            locals.var_fbsq__blk2012_dn10 = assign89920_body26_e138188_d_n10;
            locals.var_fbsq__blk2012_dn13 = assign89920_body26_e138188_d_n13;
            locals.var_fbsq__blk2012_rv = 0.0;
            let (assign89920_body27_e138207, assign89920_body27_e138207_d_n0, assign89920_body27_e138207_d_n2, assign89920_body27_e138207_d_n4, assign89920_body27_e138207_d_n5, assign89920_body27_e138207_d_n6, assign89920_body27_e138207_d_n7, assign89920_body27_e138207_d_n8, assign89920_body27_e138207_d_n9, assign89920_body27_e138207_d_n10, assign89920_body27_e138207_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) && (locals.var_guard2091 == 0.0)) {
        let assign89920_body27_e138198: f64 = (1.0 - locals.var_t0);
        let assign89920_body27_e138202: f64 = (1.0 - locals.var_t1);
        let assign89920_body27_e138203: f64 = (locals.var_phi_b_dpss * assign89920_body27_e138202);
        let assign89920_body27_e138204: f64 = (assign89920_body27_e138198 - assign89920_body27_e138203);
        let assign89920_body27_e138205: f64 = (locals.var_beta * assign89920_body27_e138204);
        (assign89920_body27_e138205, ((locals.var_beta_dn0 * assign89920_body27_e138204) + (locals.var_beta * ((-locals.var_t0_dn0) - ((locals.var_phi_b_dpss_dn0 * assign89920_body27_e138202) + (locals.var_phi_b_dpss * (-locals.var_t1_dn0)))))), ((locals.var_beta_dn2 * assign89920_body27_e138204) + (locals.var_beta * ((-locals.var_t0_dn2) - ((locals.var_phi_b_dpss_dn2 * assign89920_body27_e138202) + (locals.var_phi_b_dpss * (-locals.var_t1_dn2)))))), ((locals.var_beta_dn4 * assign89920_body27_e138204) + (locals.var_beta * ((-locals.var_t0_dn4) - ((locals.var_phi_b_dpss_dn4 * assign89920_body27_e138202) + (locals.var_phi_b_dpss * (-locals.var_t1_dn4)))))), ((locals.var_beta_dn5 * assign89920_body27_e138204) + (locals.var_beta * ((-locals.var_t0_dn5) - ((locals.var_phi_b_dpss_dn5 * assign89920_body27_e138202) + (locals.var_phi_b_dpss * (-locals.var_t1_dn5)))))), ((locals.var_beta_dn6 * assign89920_body27_e138204) + (locals.var_beta * ((-locals.var_t0_dn6) - ((locals.var_phi_b_dpss_dn6 * assign89920_body27_e138202) + (locals.var_phi_b_dpss * (-locals.var_t1_dn6)))))), ((locals.var_beta_dn7 * assign89920_body27_e138204) + (locals.var_beta * ((-locals.var_t0_dn7) - ((locals.var_phi_b_dpss_dn7 * assign89920_body27_e138202) + (locals.var_phi_b_dpss * (-locals.var_t1_dn7)))))), ((locals.var_beta_dn8 * assign89920_body27_e138204) + (locals.var_beta * ((-locals.var_t0_dn8) - ((locals.var_phi_b_dpss_dn8 * assign89920_body27_e138202) + (locals.var_phi_b_dpss * (-locals.var_t1_dn8)))))), ((locals.var_beta_dn9 * assign89920_body27_e138204) + (locals.var_beta * ((-locals.var_t0_dn9) - ((locals.var_phi_b_dpss_dn9 * assign89920_body27_e138202) + (locals.var_phi_b_dpss * (-locals.var_t1_dn9)))))), ((locals.var_beta_dn10 * assign89920_body27_e138204) + (locals.var_beta * ((-locals.var_t0_dn10) - ((locals.var_phi_b_dpss_dn10 * assign89920_body27_e138202) + (locals.var_phi_b_dpss * (-locals.var_t1_dn10)))))), ((locals.var_beta_dn13 * assign89920_body27_e138204) + (locals.var_beta * ((-locals.var_t0_dn13) - ((locals.var_phi_b_dpss_dn13 * assign89920_body27_e138202) + (locals.var_phi_b_dpss * (-locals.var_t1_dn13)))))),)
    } else {
        (locals.var_fbsq_dpss__blk2013, locals.var_fbsq_dpss__blk2013_dn0, locals.var_fbsq_dpss__blk2013_dn2, locals.var_fbsq_dpss__blk2013_dn4, locals.var_fbsq_dpss__blk2013_dn5, locals.var_fbsq_dpss__blk2013_dn6, locals.var_fbsq_dpss__blk2013_dn7, locals.var_fbsq_dpss__blk2013_dn8, locals.var_fbsq_dpss__blk2013_dn9, locals.var_fbsq_dpss__blk2013_dn10, locals.var_fbsq_dpss__blk2013_dn13,)
    }
};
            locals.var_fbsq_dpss__blk2013 = assign89920_body27_e138207;
            locals.var_fbsq_dpss__blk2013_dn0 = assign89920_body27_e138207_d_n0;
            locals.var_fbsq_dpss__blk2013_dn2 = assign89920_body27_e138207_d_n2;
            locals.var_fbsq_dpss__blk2013_dn4 = assign89920_body27_e138207_d_n4;
            locals.var_fbsq_dpss__blk2013_dn5 = assign89920_body27_e138207_d_n5;
            locals.var_fbsq_dpss__blk2013_dn6 = assign89920_body27_e138207_d_n6;
            locals.var_fbsq_dpss__blk2013_dn7 = assign89920_body27_e138207_d_n7;
            locals.var_fbsq_dpss__blk2013_dn8 = assign89920_body27_e138207_d_n8;
            locals.var_fbsq_dpss__blk2013_dn9 = assign89920_body27_e138207_d_n9;
            locals.var_fbsq_dpss__blk2013_dn10 = assign89920_body27_e138207_d_n10;
            locals.var_fbsq_dpss__blk2013_dn13 = assign89920_body27_e138207_d_n13;
            locals.var_fbsq_dpss__blk2013_rv = 0.0;
            let assign89920_body28_e138209: f64 = (locals.var_chi).abs();
            let assign89920_body28_e138211: f64 = if assign89920_body28_e138209 < 5e-5 { 1.0 } else { 0.0 };
            locals.var_guard2092 = assign89920_body28_e138211;
            locals.var_guard2092_rv = 0.0;
            let (assign89920_body29_e138241, assign89920_body29_e138241_d_n0, assign89920_body29_e138241_d_n2, assign89920_body29_e138241_d_n4, assign89920_body29_e138241_d_n5, assign89920_body29_e138241_d_n6, assign89920_body29_e138241_d_n7, assign89920_body29_e138241_d_n8, assign89920_body29_e138241_d_n9, assign89920_body29_e138241_d_n10, assign89920_body29_e138241_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) && (locals.var_guard2092 != 0.0)) {
        let assign89920_body29_e138219: f64 = (locals.var_chi * locals.var_chi);
        let assign89920_body29_e138221: f64 = (assign89920_body29_e138219 / 2.0);
        let assign89920_body29_e138225: f64 = (locals.var_chi / 3.0);
        let assign89920_body29_e138229: f64 = (locals.var_chi / 4.0);
        let assign89920_body29_e138233: f64 = (locals.var_chi / 5.0);
        let assign89920_body29_e138234: f64 = (1.0 + assign89920_body29_e138233);
        let assign89920_body29_e138235: f64 = (assign89920_body29_e138229 * assign89920_body29_e138234);
        let assign89920_body29_e138236: f64 = (1.0 + assign89920_body29_e138235);
        let assign89920_body29_e138237: f64 = (assign89920_body29_e138225 * assign89920_body29_e138236);
        let assign89920_body29_e138238: f64 = (1.0 + assign89920_body29_e138237);
        let assign89920_body29_e138239: f64 = (assign89920_body29_e138221 * assign89920_body29_e138238);
        (assign89920_body29_e138239, (((((locals.var_chi_dn0 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn0)) / 2.0) * assign89920_body29_e138238) + (assign89920_body29_e138221 * (((locals.var_chi_dn0 / 3.0) * assign89920_body29_e138236) + (assign89920_body29_e138225 * (((locals.var_chi_dn0 / 4.0) * assign89920_body29_e138234) + (assign89920_body29_e138229 * (locals.var_chi_dn0 / 5.0))))))), (((((locals.var_chi_dn2 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn2)) / 2.0) * assign89920_body29_e138238) + (assign89920_body29_e138221 * (((locals.var_chi_dn2 / 3.0) * assign89920_body29_e138236) + (assign89920_body29_e138225 * (((locals.var_chi_dn2 / 4.0) * assign89920_body29_e138234) + (assign89920_body29_e138229 * (locals.var_chi_dn2 / 5.0))))))), (((((locals.var_chi_dn4 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn4)) / 2.0) * assign89920_body29_e138238) + (assign89920_body29_e138221 * (((locals.var_chi_dn4 / 3.0) * assign89920_body29_e138236) + (assign89920_body29_e138225 * (((locals.var_chi_dn4 / 4.0) * assign89920_body29_e138234) + (assign89920_body29_e138229 * (locals.var_chi_dn4 / 5.0))))))), (((((locals.var_chi_dn5 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn5)) / 2.0) * assign89920_body29_e138238) + (assign89920_body29_e138221 * (((locals.var_chi_dn5 / 3.0) * assign89920_body29_e138236) + (assign89920_body29_e138225 * (((locals.var_chi_dn5 / 4.0) * assign89920_body29_e138234) + (assign89920_body29_e138229 * (locals.var_chi_dn5 / 5.0))))))), (((((locals.var_chi_dn6 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn6)) / 2.0) * assign89920_body29_e138238) + (assign89920_body29_e138221 * (((locals.var_chi_dn6 / 3.0) * assign89920_body29_e138236) + (assign89920_body29_e138225 * (((locals.var_chi_dn6 / 4.0) * assign89920_body29_e138234) + (assign89920_body29_e138229 * (locals.var_chi_dn6 / 5.0))))))), (((((locals.var_chi_dn7 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn7)) / 2.0) * assign89920_body29_e138238) + (assign89920_body29_e138221 * (((locals.var_chi_dn7 / 3.0) * assign89920_body29_e138236) + (assign89920_body29_e138225 * (((locals.var_chi_dn7 / 4.0) * assign89920_body29_e138234) + (assign89920_body29_e138229 * (locals.var_chi_dn7 / 5.0))))))), (((((locals.var_chi_dn8 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn8)) / 2.0) * assign89920_body29_e138238) + (assign89920_body29_e138221 * (((locals.var_chi_dn8 / 3.0) * assign89920_body29_e138236) + (assign89920_body29_e138225 * (((locals.var_chi_dn8 / 4.0) * assign89920_body29_e138234) + (assign89920_body29_e138229 * (locals.var_chi_dn8 / 5.0))))))), (((((locals.var_chi_dn9 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn9)) / 2.0) * assign89920_body29_e138238) + (assign89920_body29_e138221 * (((locals.var_chi_dn9 / 3.0) * assign89920_body29_e138236) + (assign89920_body29_e138225 * (((locals.var_chi_dn9 / 4.0) * assign89920_body29_e138234) + (assign89920_body29_e138229 * (locals.var_chi_dn9 / 5.0))))))), (((((locals.var_chi_dn10 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn10)) / 2.0) * assign89920_body29_e138238) + (assign89920_body29_e138221 * (((locals.var_chi_dn10 / 3.0) * assign89920_body29_e138236) + (assign89920_body29_e138225 * (((locals.var_chi_dn10 / 4.0) * assign89920_body29_e138234) + (assign89920_body29_e138229 * (locals.var_chi_dn10 / 5.0))))))), (((((locals.var_chi_dn13 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn13)) / 2.0) * assign89920_body29_e138238) + (assign89920_body29_e138221 * (((locals.var_chi_dn13 / 3.0) * assign89920_body29_e138236) + (assign89920_body29_e138225 * (((locals.var_chi_dn13 / 4.0) * assign89920_body29_e138234) + (assign89920_body29_e138229 * (locals.var_chi_dn13 / 5.0))))))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
            locals.var_t0 = assign89920_body29_e138241;
            locals.var_t0_dn0 = assign89920_body29_e138241_d_n0;
            locals.var_t0_dn2 = assign89920_body29_e138241_d_n2;
            locals.var_t0_dn4 = assign89920_body29_e138241_d_n4;
            locals.var_t0_dn5 = assign89920_body29_e138241_d_n5;
            locals.var_t0_dn6 = assign89920_body29_e138241_d_n6;
            locals.var_t0_dn7 = assign89920_body29_e138241_d_n7;
            locals.var_t0_dn8 = assign89920_body29_e138241_d_n8;
            locals.var_t0_dn9 = assign89920_body29_e138241_d_n9;
            locals.var_t0_dn10 = assign89920_body29_e138241_d_n10;
            locals.var_t0_dn13 = assign89920_body29_e138241_d_n13;
            locals.var_t0_rv = 0.0;
            let (assign89920_body30_e138267, assign89920_body30_e138267_d_n0, assign89920_body30_e138267_d_n2, assign89920_body30_e138267_d_n4, assign89920_body30_e138267_d_n5, assign89920_body30_e138267_d_n6, assign89920_body30_e138267_d_n7, assign89920_body30_e138267_d_n8, assign89920_body30_e138267_d_n9, assign89920_body30_e138267_d_n10, assign89920_body30_e138267_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) && (locals.var_guard2092 != 0.0)) {
        let assign89920_body30_e138251: f64 = (locals.var_chi / 2.0);
        let assign89920_body30_e138255: f64 = (locals.var_chi / 3.0);
        let assign89920_body30_e138259: f64 = (locals.var_chi / 4.0);
        let assign89920_body30_e138260: f64 = (1.0 + assign89920_body30_e138259);
        let assign89920_body30_e138261: f64 = (assign89920_body30_e138255 * assign89920_body30_e138260);
        let assign89920_body30_e138262: f64 = (1.0 + assign89920_body30_e138261);
        let assign89920_body30_e138263: f64 = (assign89920_body30_e138251 * assign89920_body30_e138262);
        let assign89920_body30_e138264: f64 = (1.0 + assign89920_body30_e138263);
        let assign89920_body30_e138265: f64 = (locals.var_chi * assign89920_body30_e138264);
        (assign89920_body30_e138265, ((locals.var_chi_dn0 * assign89920_body30_e138264) + (locals.var_chi * (((locals.var_chi_dn0 / 2.0) * assign89920_body30_e138262) + (assign89920_body30_e138251 * (((locals.var_chi_dn0 / 3.0) * assign89920_body30_e138260) + (assign89920_body30_e138255 * (locals.var_chi_dn0 / 4.0))))))), ((locals.var_chi_dn2 * assign89920_body30_e138264) + (locals.var_chi * (((locals.var_chi_dn2 / 2.0) * assign89920_body30_e138262) + (assign89920_body30_e138251 * (((locals.var_chi_dn2 / 3.0) * assign89920_body30_e138260) + (assign89920_body30_e138255 * (locals.var_chi_dn2 / 4.0))))))), ((locals.var_chi_dn4 * assign89920_body30_e138264) + (locals.var_chi * (((locals.var_chi_dn4 / 2.0) * assign89920_body30_e138262) + (assign89920_body30_e138251 * (((locals.var_chi_dn4 / 3.0) * assign89920_body30_e138260) + (assign89920_body30_e138255 * (locals.var_chi_dn4 / 4.0))))))), ((locals.var_chi_dn5 * assign89920_body30_e138264) + (locals.var_chi * (((locals.var_chi_dn5 / 2.0) * assign89920_body30_e138262) + (assign89920_body30_e138251 * (((locals.var_chi_dn5 / 3.0) * assign89920_body30_e138260) + (assign89920_body30_e138255 * (locals.var_chi_dn5 / 4.0))))))), ((locals.var_chi_dn6 * assign89920_body30_e138264) + (locals.var_chi * (((locals.var_chi_dn6 / 2.0) * assign89920_body30_e138262) + (assign89920_body30_e138251 * (((locals.var_chi_dn6 / 3.0) * assign89920_body30_e138260) + (assign89920_body30_e138255 * (locals.var_chi_dn6 / 4.0))))))), ((locals.var_chi_dn7 * assign89920_body30_e138264) + (locals.var_chi * (((locals.var_chi_dn7 / 2.0) * assign89920_body30_e138262) + (assign89920_body30_e138251 * (((locals.var_chi_dn7 / 3.0) * assign89920_body30_e138260) + (assign89920_body30_e138255 * (locals.var_chi_dn7 / 4.0))))))), ((locals.var_chi_dn8 * assign89920_body30_e138264) + (locals.var_chi * (((locals.var_chi_dn8 / 2.0) * assign89920_body30_e138262) + (assign89920_body30_e138251 * (((locals.var_chi_dn8 / 3.0) * assign89920_body30_e138260) + (assign89920_body30_e138255 * (locals.var_chi_dn8 / 4.0))))))), ((locals.var_chi_dn9 * assign89920_body30_e138264) + (locals.var_chi * (((locals.var_chi_dn9 / 2.0) * assign89920_body30_e138262) + (assign89920_body30_e138251 * (((locals.var_chi_dn9 / 3.0) * assign89920_body30_e138260) + (assign89920_body30_e138255 * (locals.var_chi_dn9 / 4.0))))))), ((locals.var_chi_dn10 * assign89920_body30_e138264) + (locals.var_chi * (((locals.var_chi_dn10 / 2.0) * assign89920_body30_e138262) + (assign89920_body30_e138251 * (((locals.var_chi_dn10 / 3.0) * assign89920_body30_e138260) + (assign89920_body30_e138255 * (locals.var_chi_dn10 / 4.0))))))), ((locals.var_chi_dn13 * assign89920_body30_e138264) + (locals.var_chi * (((locals.var_chi_dn13 / 2.0) * assign89920_body30_e138262) + (assign89920_body30_e138251 * (((locals.var_chi_dn13 / 3.0) * assign89920_body30_e138260) + (assign89920_body30_e138255 * (locals.var_chi_dn13 / 4.0))))))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
            locals.var_t1 = assign89920_body30_e138267;
            locals.var_t1_dn0 = assign89920_body30_e138267_d_n0;
            locals.var_t1_dn2 = assign89920_body30_e138267_d_n2;
            locals.var_t1_dn4 = assign89920_body30_e138267_d_n4;
            locals.var_t1_dn5 = assign89920_body30_e138267_d_n5;
            locals.var_t1_dn6 = assign89920_body30_e138267_d_n6;
            locals.var_t1_dn7 = assign89920_body30_e138267_d_n7;
            locals.var_t1_dn8 = assign89920_body30_e138267_d_n8;
            locals.var_t1_dn9 = assign89920_body30_e138267_d_n9;
            locals.var_t1_dn10 = assign89920_body30_e138267_d_n10;
            locals.var_t1_dn13 = assign89920_body30_e138267_d_n13;
            locals.var_t1_rv = 0.0;
            let (assign89920_body31_e138277, assign89920_body31_e138277_d_n0, assign89920_body31_e138277_d_n2, assign89920_body31_e138277_d_n4, assign89920_body31_e138277_d_n5, assign89920_body31_e138277_d_n6, assign89920_body31_e138277_d_n7, assign89920_body31_e138277_d_n8, assign89920_body31_e138277_d_n9, assign89920_body31_e138277_d_n10, assign89920_body31_e138277_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) && (locals.var_guard2092 != 0.0)) {
        let assign89920_body31_e138275: f64 = (locals.var_cfs1 * locals.var_t0);
        (assign89920_body31_e138275, ((locals.var_cfs1_dn0 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn0)), ((locals.var_cfs1_dn2 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn2)), ((locals.var_cfs1_dn4 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn4)), ((locals.var_cfs1_dn5 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn5)), ((locals.var_cfs1_dn6 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn6)), ((locals.var_cfs1_dn7 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn7)), ((locals.var_cfs1_dn8 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn8)), ((locals.var_cfs1_dn9 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn9)), ((locals.var_cfs1_dn10 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn10)), ((locals.var_cfs1_dn13 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn13)),)
    } else {
        (locals.var_fs01, locals.var_fs01_dn0, locals.var_fs01_dn2, locals.var_fs01_dn4, locals.var_fs01_dn5, locals.var_fs01_dn6, locals.var_fs01_dn7, locals.var_fs01_dn8, locals.var_fs01_dn9, locals.var_fs01_dn10, locals.var_fs01_dn13,)
    }
};
            locals.var_fs01 = assign89920_body31_e138277;
            locals.var_fs01_dn0 = assign89920_body31_e138277_d_n0;
            locals.var_fs01_dn2 = assign89920_body31_e138277_d_n2;
            locals.var_fs01_dn4 = assign89920_body31_e138277_d_n4;
            locals.var_fs01_dn5 = assign89920_body31_e138277_d_n5;
            locals.var_fs01_dn6 = assign89920_body31_e138277_d_n6;
            locals.var_fs01_dn7 = assign89920_body31_e138277_d_n7;
            locals.var_fs01_dn8 = assign89920_body31_e138277_d_n8;
            locals.var_fs01_dn9 = assign89920_body31_e138277_d_n9;
            locals.var_fs01_dn10 = assign89920_body31_e138277_d_n10;
            locals.var_fs01_dn13 = assign89920_body31_e138277_d_n13;
            locals.var_fs01_rv = 0.0;
            let (assign89920_body32_e138289, assign89920_body32_e138289_d_n0, assign89920_body32_e138289_d_n2, assign89920_body32_e138289_d_n4, assign89920_body32_e138289_d_n5, assign89920_body32_e138289_d_n6, assign89920_body32_e138289_d_n7, assign89920_body32_e138289_d_n8, assign89920_body32_e138289_d_n9, assign89920_body32_e138289_d_n10, assign89920_body32_e138289_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) && (locals.var_guard2092 != 0.0)) {
        let assign89920_body32_e138285: f64 = (locals.var_cfs1 * locals.var_t1);
        let assign89920_body32_e138287: f64 = (assign89920_body32_e138285 * locals.var_beta);
        (assign89920_body32_e138287, ((((locals.var_cfs1_dn0 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn0)) * locals.var_beta) + (assign89920_body32_e138285 * locals.var_beta_dn0)), ((((locals.var_cfs1_dn2 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn2)) * locals.var_beta) + (assign89920_body32_e138285 * locals.var_beta_dn2)), ((((locals.var_cfs1_dn4 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn4)) * locals.var_beta) + (assign89920_body32_e138285 * locals.var_beta_dn4)), ((((locals.var_cfs1_dn5 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn5)) * locals.var_beta) + (assign89920_body32_e138285 * locals.var_beta_dn5)), ((((locals.var_cfs1_dn6 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn6)) * locals.var_beta) + (assign89920_body32_e138285 * locals.var_beta_dn6)), ((((locals.var_cfs1_dn7 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn7)) * locals.var_beta) + (assign89920_body32_e138285 * locals.var_beta_dn7)), ((((locals.var_cfs1_dn8 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn8)) * locals.var_beta) + (assign89920_body32_e138285 * locals.var_beta_dn8)), ((((locals.var_cfs1_dn9 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn9)) * locals.var_beta) + (assign89920_body32_e138285 * locals.var_beta_dn9)), ((((locals.var_cfs1_dn10 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn10)) * locals.var_beta) + (assign89920_body32_e138285 * locals.var_beta_dn10)), ((((locals.var_cfs1_dn13 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn13)) * locals.var_beta) + (assign89920_body32_e138285 * locals.var_beta_dn13)),)
    } else {
        (locals.var_fs01_dps0, locals.var_fs01_dps0_dn0, locals.var_fs01_dps0_dn2, locals.var_fs01_dps0_dn4, locals.var_fs01_dps0_dn5, locals.var_fs01_dps0_dn6, locals.var_fs01_dps0_dn7, locals.var_fs01_dps0_dn8, locals.var_fs01_dps0_dn9, locals.var_fs01_dps0_dn10, locals.var_fs01_dps0_dn13,)
    }
};
            locals.var_fs01_dps0 = assign89920_body32_e138289;
            locals.var_fs01_dps0_dn0 = assign89920_body32_e138289_d_n0;
            locals.var_fs01_dps0_dn2 = assign89920_body32_e138289_d_n2;
            locals.var_fs01_dps0_dn4 = assign89920_body32_e138289_d_n4;
            locals.var_fs01_dps0_dn5 = assign89920_body32_e138289_d_n5;
            locals.var_fs01_dps0_dn6 = assign89920_body32_e138289_d_n6;
            locals.var_fs01_dps0_dn7 = assign89920_body32_e138289_d_n7;
            locals.var_fs01_dps0_dn8 = assign89920_body32_e138289_d_n8;
            locals.var_fs01_dps0_dn9 = assign89920_body32_e138289_d_n9;
            locals.var_fs01_dps0_dn10 = assign89920_body32_e138289_d_n10;
            locals.var_fs01_dps0_dn13 = assign89920_body32_e138289_d_n13;
            locals.var_fs01_dps0_rv = 0.0;
            let assign89920_body33_e138291: f64 = (locals.var_chi).abs();
            let assign89920_body33_e138293: f64 = if assign89920_body33_e138291 < 60.0 { 1.0 } else { 0.0 };
            locals.var_guard2093 = assign89920_body33_e138293;
            locals.var_guard2093_rv = 0.0;
            let (assign89920_body35_e138324, assign89920_body35_e138324_d_n0, assign89920_body35_e138324_d_n2, assign89920_body35_e138324_d_n4, assign89920_body35_e138324_d_n5, assign89920_body35_e138324_d_n6, assign89920_body35_e138324_d_n7, assign89920_body35_e138324_d_n8, assign89920_body35_e138324_d_n9, assign89920_body35_e138324_d_n10, assign89920_body35_e138324_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) && (locals.var_guard2092 == 0.0)) && (locals.var_guard2093 != 0.0)) {
        let assign89920_body35_e138322: f64 = (locals.var_chi).exp();
        (assign89920_body35_e138322, (assign89920_body35_e138322 * locals.var_chi_dn0), (assign89920_body35_e138322 * locals.var_chi_dn2), (assign89920_body35_e138322 * locals.var_chi_dn4), (assign89920_body35_e138322 * locals.var_chi_dn5), (assign89920_body35_e138322 * locals.var_chi_dn6), (assign89920_body35_e138322 * locals.var_chi_dn7), (assign89920_body35_e138322 * locals.var_chi_dn8), (assign89920_body35_e138322 * locals.var_chi_dn9), (assign89920_body35_e138322 * locals.var_chi_dn10), (assign89920_body35_e138322 * locals.var_chi_dn13),)
    } else {
        (locals.var_exp_chi, locals.var_exp_chi_dn0, locals.var_exp_chi_dn2, locals.var_exp_chi_dn4, locals.var_exp_chi_dn5, locals.var_exp_chi_dn6, locals.var_exp_chi_dn7, locals.var_exp_chi_dn8, locals.var_exp_chi_dn9, locals.var_exp_chi_dn10, locals.var_exp_chi_dn13,)
    }
};
            locals.var_exp_chi = assign89920_body35_e138324;
            locals.var_exp_chi_dn0 = assign89920_body35_e138324_d_n0;
            locals.var_exp_chi_dn2 = assign89920_body35_e138324_d_n2;
            locals.var_exp_chi_dn4 = assign89920_body35_e138324_d_n4;
            locals.var_exp_chi_dn5 = assign89920_body35_e138324_d_n5;
            locals.var_exp_chi_dn6 = assign89920_body35_e138324_d_n6;
            locals.var_exp_chi_dn7 = assign89920_body35_e138324_d_n7;
            locals.var_exp_chi_dn8 = assign89920_body35_e138324_d_n8;
            locals.var_exp_chi_dn9 = assign89920_body35_e138324_d_n9;
            locals.var_exp_chi_dn10 = assign89920_body35_e138324_d_n10;
            locals.var_exp_chi_dn13 = assign89920_body35_e138324_d_n13;
            locals.var_exp_chi_rv = 0.0;
            let (assign89920_body36_e138337, assign89920_body36_e138337_d_n0, assign89920_body36_e138337_d_n2, assign89920_body36_e138337_d_n4, assign89920_body36_e138337_d_n5, assign89920_body36_e138337_d_n6, assign89920_body36_e138337_d_n7, assign89920_body36_e138337_d_n8, assign89920_body36_e138337_d_n9, assign89920_body36_e138337_d_n10, assign89920_body36_e138337_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) && (locals.var_guard2092 == 0.0)) && (locals.var_guard2093 != 0.0)) {
        let assign89920_body36_e138335: f64 = (locals.var_exp_chi - 1.0);
        (assign89920_body36_e138335, locals.var_exp_chi_dn0, locals.var_exp_chi_dn2, locals.var_exp_chi_dn4, locals.var_exp_chi_dn5, locals.var_exp_chi_dn6, locals.var_exp_chi_dn7, locals.var_exp_chi_dn8, locals.var_exp_chi_dn9, locals.var_exp_chi_dn10, locals.var_exp_chi_dn13,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
            locals.var_t1 = assign89920_body36_e138337;
            locals.var_t1_dn0 = assign89920_body36_e138337_d_n0;
            locals.var_t1_dn2 = assign89920_body36_e138337_d_n2;
            locals.var_t1_dn4 = assign89920_body36_e138337_d_n4;
            locals.var_t1_dn5 = assign89920_body36_e138337_d_n5;
            locals.var_t1_dn6 = assign89920_body36_e138337_d_n6;
            locals.var_t1_dn7 = assign89920_body36_e138337_d_n7;
            locals.var_t1_dn8 = assign89920_body36_e138337_d_n8;
            locals.var_t1_dn9 = assign89920_body36_e138337_d_n9;
            locals.var_t1_dn10 = assign89920_body36_e138337_d_n10;
            locals.var_t1_dn13 = assign89920_body36_e138337_d_n13;
            locals.var_t1_rv = 0.0;
            let (assign89920_body37_e138352, assign89920_body37_e138352_d_n0, assign89920_body37_e138352_d_n2, assign89920_body37_e138352_d_n4, assign89920_body37_e138352_d_n5, assign89920_body37_e138352_d_n6, assign89920_body37_e138352_d_n7, assign89920_body37_e138352_d_n8, assign89920_body37_e138352_d_n9, assign89920_body37_e138352_d_n10, assign89920_body37_e138352_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) && (locals.var_guard2092 == 0.0)) && (locals.var_guard2093 != 0.0)) {
        let assign89920_body37_e138349: f64 = (locals.var_t1 - locals.var_chi);
        let assign89920_body37_e138350: f64 = (locals.var_cfs1 * assign89920_body37_e138349);
        (assign89920_body37_e138350, ((locals.var_cfs1_dn0 * assign89920_body37_e138349) + (locals.var_cfs1 * (locals.var_t1_dn0 - locals.var_chi_dn0))), ((locals.var_cfs1_dn2 * assign89920_body37_e138349) + (locals.var_cfs1 * (locals.var_t1_dn2 - locals.var_chi_dn2))), ((locals.var_cfs1_dn4 * assign89920_body37_e138349) + (locals.var_cfs1 * (locals.var_t1_dn4 - locals.var_chi_dn4))), ((locals.var_cfs1_dn5 * assign89920_body37_e138349) + (locals.var_cfs1 * (locals.var_t1_dn5 - locals.var_chi_dn5))), ((locals.var_cfs1_dn6 * assign89920_body37_e138349) + (locals.var_cfs1 * (locals.var_t1_dn6 - locals.var_chi_dn6))), ((locals.var_cfs1_dn7 * assign89920_body37_e138349) + (locals.var_cfs1 * (locals.var_t1_dn7 - locals.var_chi_dn7))), ((locals.var_cfs1_dn8 * assign89920_body37_e138349) + (locals.var_cfs1 * (locals.var_t1_dn8 - locals.var_chi_dn8))), ((locals.var_cfs1_dn9 * assign89920_body37_e138349) + (locals.var_cfs1 * (locals.var_t1_dn9 - locals.var_chi_dn9))), ((locals.var_cfs1_dn10 * assign89920_body37_e138349) + (locals.var_cfs1 * (locals.var_t1_dn10 - locals.var_chi_dn10))), ((locals.var_cfs1_dn13 * assign89920_body37_e138349) + (locals.var_cfs1 * (locals.var_t1_dn13 - locals.var_chi_dn13))),)
    } else {
        (locals.var_fs01, locals.var_fs01_dn0, locals.var_fs01_dn2, locals.var_fs01_dn4, locals.var_fs01_dn5, locals.var_fs01_dn6, locals.var_fs01_dn7, locals.var_fs01_dn8, locals.var_fs01_dn9, locals.var_fs01_dn10, locals.var_fs01_dn13,)
    }
};
            locals.var_fs01 = assign89920_body37_e138352;
            locals.var_fs01_dn0 = assign89920_body37_e138352_d_n0;
            locals.var_fs01_dn2 = assign89920_body37_e138352_d_n2;
            locals.var_fs01_dn4 = assign89920_body37_e138352_d_n4;
            locals.var_fs01_dn5 = assign89920_body37_e138352_d_n5;
            locals.var_fs01_dn6 = assign89920_body37_e138352_d_n6;
            locals.var_fs01_dn7 = assign89920_body37_e138352_d_n7;
            locals.var_fs01_dn8 = assign89920_body37_e138352_d_n8;
            locals.var_fs01_dn9 = assign89920_body37_e138352_d_n9;
            locals.var_fs01_dn10 = assign89920_body37_e138352_d_n10;
            locals.var_fs01_dn13 = assign89920_body37_e138352_d_n13;
            locals.var_fs01_rv = 0.0;
            let (assign89920_body38_e138367, assign89920_body38_e138367_d_n0, assign89920_body38_e138367_d_n2, assign89920_body38_e138367_d_n4, assign89920_body38_e138367_d_n5, assign89920_body38_e138367_d_n6, assign89920_body38_e138367_d_n7, assign89920_body38_e138367_d_n8, assign89920_body38_e138367_d_n9, assign89920_body38_e138367_d_n10, assign89920_body38_e138367_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) && (locals.var_guard2092 == 0.0)) && (locals.var_guard2093 != 0.0)) {
        let assign89920_body38_e138363: f64 = (locals.var_cfs1 * locals.var_beta);
        let assign89920_body38_e138365: f64 = (assign89920_body38_e138363 * locals.var_t1);
        (assign89920_body38_e138365, ((((locals.var_cfs1_dn0 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn0)) * locals.var_t1) + (assign89920_body38_e138363 * locals.var_t1_dn0)), ((((locals.var_cfs1_dn2 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn2)) * locals.var_t1) + (assign89920_body38_e138363 * locals.var_t1_dn2)), ((((locals.var_cfs1_dn4 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn4)) * locals.var_t1) + (assign89920_body38_e138363 * locals.var_t1_dn4)), ((((locals.var_cfs1_dn5 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn5)) * locals.var_t1) + (assign89920_body38_e138363 * locals.var_t1_dn5)), ((((locals.var_cfs1_dn6 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn6)) * locals.var_t1) + (assign89920_body38_e138363 * locals.var_t1_dn6)), ((((locals.var_cfs1_dn7 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn7)) * locals.var_t1) + (assign89920_body38_e138363 * locals.var_t1_dn7)), ((((locals.var_cfs1_dn8 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn8)) * locals.var_t1) + (assign89920_body38_e138363 * locals.var_t1_dn8)), ((((locals.var_cfs1_dn9 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn9)) * locals.var_t1) + (assign89920_body38_e138363 * locals.var_t1_dn9)), ((((locals.var_cfs1_dn10 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn10)) * locals.var_t1) + (assign89920_body38_e138363 * locals.var_t1_dn10)), ((((locals.var_cfs1_dn13 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn13)) * locals.var_t1) + (assign89920_body38_e138363 * locals.var_t1_dn13)),)
    } else {
        (locals.var_fs01_dps0, locals.var_fs01_dps0_dn0, locals.var_fs01_dps0_dn2, locals.var_fs01_dps0_dn4, locals.var_fs01_dps0_dn5, locals.var_fs01_dps0_dn6, locals.var_fs01_dps0_dn7, locals.var_fs01_dps0_dn8, locals.var_fs01_dps0_dn9, locals.var_fs01_dps0_dn10, locals.var_fs01_dps0_dn13,)
    }
};
            locals.var_fs01_dps0 = assign89920_body38_e138367;
            locals.var_fs01_dps0_dn0 = assign89920_body38_e138367_d_n0;
            locals.var_fs01_dps0_dn2 = assign89920_body38_e138367_d_n2;
            locals.var_fs01_dps0_dn4 = assign89920_body38_e138367_d_n4;
            locals.var_fs01_dps0_dn5 = assign89920_body38_e138367_d_n5;
            locals.var_fs01_dps0_dn6 = assign89920_body38_e138367_d_n6;
            locals.var_fs01_dps0_dn7 = assign89920_body38_e138367_d_n7;
            locals.var_fs01_dps0_dn8 = assign89920_body38_e138367_d_n8;
            locals.var_fs01_dps0_dn9 = assign89920_body38_e138367_d_n9;
            locals.var_fs01_dps0_dn10 = assign89920_body38_e138367_d_n10;
            locals.var_fs01_dps0_dn13 = assign89920_body38_e138367_d_n13;
            locals.var_fs01_dps0_rv = 0.0;
            let (assign89920_body40_e138402, assign89920_body40_e138402_d_n0, assign89920_body40_e138402_d_n2, assign89920_body40_e138402_d_n4, assign89920_body40_e138402_d_n5, assign89920_body40_e138402_d_n6, assign89920_body40_e138402_d_n7, assign89920_body40_e138402_d_n8, assign89920_body40_e138402_d_n9, assign89920_body40_e138402_d_n10, assign89920_body40_e138402_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) && (locals.var_guard2092 == 0.0)) && (locals.var_guard2093 == 0.0)) {
        let assign89920_body40_e138399: f64 = (locals.var_beta * locals.var_ps0ld);
        let assign89920_body40_e138400: f64 = (assign89920_body40_e138399).exp();
        (assign89920_body40_e138400, (assign89920_body40_e138400 * ((locals.var_beta_dn0 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn0))), (assign89920_body40_e138400 * ((locals.var_beta_dn2 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn2))), (assign89920_body40_e138400 * ((locals.var_beta_dn4 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn4))), (assign89920_body40_e138400 * ((locals.var_beta_dn5 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn5))), (assign89920_body40_e138400 * ((locals.var_beta_dn6 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn6))), (assign89920_body40_e138400 * ((locals.var_beta_dn7 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn7))), (assign89920_body40_e138400 * ((locals.var_beta_dn8 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn8))), (assign89920_body40_e138400 * ((locals.var_beta_dn9 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn9))), (assign89920_body40_e138400 * ((locals.var_beta_dn10 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn10))), (assign89920_body40_e138400 * ((locals.var_beta_dn13 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn13))),)
    } else {
        (locals.var_exp_bps0, locals.var_exp_bps0_dn0, locals.var_exp_bps0_dn2, locals.var_exp_bps0_dn4, locals.var_exp_bps0_dn5, locals.var_exp_bps0_dn6, locals.var_exp_bps0_dn7, locals.var_exp_bps0_dn8, locals.var_exp_bps0_dn9, locals.var_exp_bps0_dn10, locals.var_exp_bps0_dn13,)
    }
};
            locals.var_exp_bps0 = assign89920_body40_e138402;
            locals.var_exp_bps0_dn0 = assign89920_body40_e138402_d_n0;
            locals.var_exp_bps0_dn2 = assign89920_body40_e138402_d_n2;
            locals.var_exp_bps0_dn4 = assign89920_body40_e138402_d_n4;
            locals.var_exp_bps0_dn5 = assign89920_body40_e138402_d_n5;
            locals.var_exp_bps0_dn6 = assign89920_body40_e138402_d_n6;
            locals.var_exp_bps0_dn7 = assign89920_body40_e138402_d_n7;
            locals.var_exp_bps0_dn8 = assign89920_body40_e138402_d_n8;
            locals.var_exp_bps0_dn9 = assign89920_body40_e138402_d_n9;
            locals.var_exp_bps0_dn10 = assign89920_body40_e138402_d_n10;
            locals.var_exp_bps0_dn13 = assign89920_body40_e138402_d_n13;
            locals.var_exp_bps0_rv = 0.0;
            let (assign89920_body41_e138422, assign89920_body41_e138422_d_n0, assign89920_body41_e138422_d_n2, assign89920_body41_e138422_d_n4, assign89920_body41_e138422_d_n5, assign89920_body41_e138422_d_n6, assign89920_body41_e138422_d_n7, assign89920_body41_e138422_d_n8, assign89920_body41_e138422_d_n9, assign89920_body41_e138422_d_n10, assign89920_body41_e138422_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) && (locals.var_guard2092 == 0.0)) && (locals.var_guard2093 == 0.0)) {
        let assign89920_body41_e138417: f64 = (locals.var_chi + 1.0);
        let assign89920_body41_e138418: f64 = (locals.var_exp_bvbs * assign89920_body41_e138417);
        let assign89920_body41_e138419: f64 = (locals.var_exp_bps0 - assign89920_body41_e138418);
        let assign89920_body41_e138420: f64 = (locals.var_cnst1over * assign89920_body41_e138419);
        (assign89920_body41_e138420, ((locals.var_cnst1over_dn0 * assign89920_body41_e138419) + (locals.var_cnst1over * (locals.var_exp_bps0_dn0 - ((locals.var_exp_bvbs_dn0 * assign89920_body41_e138417) + (locals.var_exp_bvbs * locals.var_chi_dn0))))), ((locals.var_cnst1over_dn2 * assign89920_body41_e138419) + (locals.var_cnst1over * (locals.var_exp_bps0_dn2 - ((locals.var_exp_bvbs_dn2 * assign89920_body41_e138417) + (locals.var_exp_bvbs * locals.var_chi_dn2))))), ((locals.var_cnst1over_dn4 * assign89920_body41_e138419) + (locals.var_cnst1over * (locals.var_exp_bps0_dn4 - ((locals.var_exp_bvbs_dn4 * assign89920_body41_e138417) + (locals.var_exp_bvbs * locals.var_chi_dn4))))), ((locals.var_cnst1over_dn5 * assign89920_body41_e138419) + (locals.var_cnst1over * (locals.var_exp_bps0_dn5 - ((locals.var_exp_bvbs_dn5 * assign89920_body41_e138417) + (locals.var_exp_bvbs * locals.var_chi_dn5))))), ((locals.var_cnst1over_dn6 * assign89920_body41_e138419) + (locals.var_cnst1over * (locals.var_exp_bps0_dn6 - ((locals.var_exp_bvbs_dn6 * assign89920_body41_e138417) + (locals.var_exp_bvbs * locals.var_chi_dn6))))), ((locals.var_cnst1over_dn7 * assign89920_body41_e138419) + (locals.var_cnst1over * (locals.var_exp_bps0_dn7 - ((locals.var_exp_bvbs_dn7 * assign89920_body41_e138417) + (locals.var_exp_bvbs * locals.var_chi_dn7))))), ((locals.var_cnst1over_dn8 * assign89920_body41_e138419) + (locals.var_cnst1over * (locals.var_exp_bps0_dn8 - ((locals.var_exp_bvbs_dn8 * assign89920_body41_e138417) + (locals.var_exp_bvbs * locals.var_chi_dn8))))), ((locals.var_cnst1over_dn9 * assign89920_body41_e138419) + (locals.var_cnst1over * (locals.var_exp_bps0_dn9 - ((locals.var_exp_bvbs_dn9 * assign89920_body41_e138417) + (locals.var_exp_bvbs * locals.var_chi_dn9))))), ((locals.var_cnst1over_dn10 * assign89920_body41_e138419) + (locals.var_cnst1over * (locals.var_exp_bps0_dn10 - ((locals.var_exp_bvbs_dn10 * assign89920_body41_e138417) + (locals.var_exp_bvbs * locals.var_chi_dn10))))), ((locals.var_cnst1over_dn13 * assign89920_body41_e138419) + (locals.var_cnst1over * (locals.var_exp_bps0_dn13 - ((locals.var_exp_bvbs_dn13 * assign89920_body41_e138417) + (locals.var_exp_bvbs * locals.var_chi_dn13))))),)
    } else {
        (locals.var_fs01, locals.var_fs01_dn0, locals.var_fs01_dn2, locals.var_fs01_dn4, locals.var_fs01_dn5, locals.var_fs01_dn6, locals.var_fs01_dn7, locals.var_fs01_dn8, locals.var_fs01_dn9, locals.var_fs01_dn10, locals.var_fs01_dn13,)
    }
};
            locals.var_fs01 = assign89920_body41_e138422;
            locals.var_fs01_dn0 = assign89920_body41_e138422_d_n0;
            locals.var_fs01_dn2 = assign89920_body41_e138422_d_n2;
            locals.var_fs01_dn4 = assign89920_body41_e138422_d_n4;
            locals.var_fs01_dn5 = assign89920_body41_e138422_d_n5;
            locals.var_fs01_dn6 = assign89920_body41_e138422_d_n6;
            locals.var_fs01_dn7 = assign89920_body41_e138422_d_n7;
            locals.var_fs01_dn8 = assign89920_body41_e138422_d_n8;
            locals.var_fs01_dn9 = assign89920_body41_e138422_d_n9;
            locals.var_fs01_dn10 = assign89920_body41_e138422_d_n10;
            locals.var_fs01_dn13 = assign89920_body41_e138422_d_n13;
            locals.var_fs01_rv = 0.0;
            let (assign89920_body42_e138440, assign89920_body42_e138440_d_n0, assign89920_body42_e138440_d_n2, assign89920_body42_e138440_d_n4, assign89920_body42_e138440_d_n5, assign89920_body42_e138440_d_n6, assign89920_body42_e138440_d_n7, assign89920_body42_e138440_d_n8, assign89920_body42_e138440_d_n9, assign89920_body42_e138440_d_n10, assign89920_body42_e138440_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) && (locals.var_guard2092 == 0.0)) && (locals.var_guard2093 == 0.0)) {
        let assign89920_body42_e138434: f64 = (locals.var_cnst1over * locals.var_beta);
        let assign89920_body42_e138437: f64 = (locals.var_exp_bps0 - locals.var_exp_bvbs);
        let assign89920_body42_e138438: f64 = (assign89920_body42_e138434 * assign89920_body42_e138437);
        (assign89920_body42_e138438, ((((locals.var_cnst1over_dn0 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn0)) * assign89920_body42_e138437) + (assign89920_body42_e138434 * (locals.var_exp_bps0_dn0 - locals.var_exp_bvbs_dn0))), ((((locals.var_cnst1over_dn2 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn2)) * assign89920_body42_e138437) + (assign89920_body42_e138434 * (locals.var_exp_bps0_dn2 - locals.var_exp_bvbs_dn2))), ((((locals.var_cnst1over_dn4 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn4)) * assign89920_body42_e138437) + (assign89920_body42_e138434 * (locals.var_exp_bps0_dn4 - locals.var_exp_bvbs_dn4))), ((((locals.var_cnst1over_dn5 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn5)) * assign89920_body42_e138437) + (assign89920_body42_e138434 * (locals.var_exp_bps0_dn5 - locals.var_exp_bvbs_dn5))), ((((locals.var_cnst1over_dn6 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn6)) * assign89920_body42_e138437) + (assign89920_body42_e138434 * (locals.var_exp_bps0_dn6 - locals.var_exp_bvbs_dn6))), ((((locals.var_cnst1over_dn7 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn7)) * assign89920_body42_e138437) + (assign89920_body42_e138434 * (locals.var_exp_bps0_dn7 - locals.var_exp_bvbs_dn7))), ((((locals.var_cnst1over_dn8 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn8)) * assign89920_body42_e138437) + (assign89920_body42_e138434 * (locals.var_exp_bps0_dn8 - locals.var_exp_bvbs_dn8))), ((((locals.var_cnst1over_dn9 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn9)) * assign89920_body42_e138437) + (assign89920_body42_e138434 * (locals.var_exp_bps0_dn9 - locals.var_exp_bvbs_dn9))), ((((locals.var_cnst1over_dn10 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn10)) * assign89920_body42_e138437) + (assign89920_body42_e138434 * (locals.var_exp_bps0_dn10 - locals.var_exp_bvbs_dn10))), ((((locals.var_cnst1over_dn13 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn13)) * assign89920_body42_e138437) + (assign89920_body42_e138434 * (locals.var_exp_bps0_dn13 - locals.var_exp_bvbs_dn13))),)
    } else {
        (locals.var_fs01_dps0, locals.var_fs01_dps0_dn0, locals.var_fs01_dps0_dn2, locals.var_fs01_dps0_dn4, locals.var_fs01_dps0_dn5, locals.var_fs01_dps0_dn6, locals.var_fs01_dps0_dn7, locals.var_fs01_dps0_dn8, locals.var_fs01_dps0_dn9, locals.var_fs01_dps0_dn10, locals.var_fs01_dps0_dn13,)
    }
};
            locals.var_fs01_dps0 = assign89920_body42_e138440;
            locals.var_fs01_dps0_dn0 = assign89920_body42_e138440_d_n0;
            locals.var_fs01_dps0_dn2 = assign89920_body42_e138440_d_n2;
            locals.var_fs01_dps0_dn4 = assign89920_body42_e138440_d_n4;
            locals.var_fs01_dps0_dn5 = assign89920_body42_e138440_d_n5;
            locals.var_fs01_dps0_dn6 = assign89920_body42_e138440_d_n6;
            locals.var_fs01_dps0_dn7 = assign89920_body42_e138440_d_n7;
            locals.var_fs01_dps0_dn8 = assign89920_body42_e138440_d_n8;
            locals.var_fs01_dps0_dn9 = assign89920_body42_e138440_d_n9;
            locals.var_fs01_dps0_dn10 = assign89920_body42_e138440_d_n10;
            locals.var_fs01_dps0_dn13 = assign89920_body42_e138440_d_n13;
            locals.var_fs01_dps0_rv = 0.0;
            let assign89920_body43_e138443: f64 = if locals.var_fs01 > 0.0 { 1.0 } else { 0.0 };
            locals.var_guard2094 = assign89920_body43_e138443;
            locals.var_guard2094_rv = 0.0;
            let (assign89920_body44_e138454, assign89920_body44_e138454_d_n0, assign89920_body44_e138454_d_n2, assign89920_body44_e138454_d_n4, assign89920_body44_e138454_d_n5, assign89920_body44_e138454_d_n6, assign89920_body44_e138454_d_n7, assign89920_body44_e138454_d_n8, assign89920_body44_e138454_d_n9, assign89920_body44_e138454_d_n10, assign89920_body44_e138454_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) && (locals.var_guard2094 != 0.0)) {
        let assign89920_body44_e138451: f64 = (locals.var_fbsq__blk2012 + locals.var_fs01);
        let assign89920_body44_e138452: f64 = (assign89920_body44_e138451).sqrt();
        (assign89920_body44_e138452, ((locals.var_fbsq__blk2012_dn0 + locals.var_fs01_dn0) / (2.0 * assign89920_body44_e138452)), ((locals.var_fbsq__blk2012_dn2 + locals.var_fs01_dn2) / (2.0 * assign89920_body44_e138452)), ((locals.var_fbsq__blk2012_dn4 + locals.var_fs01_dn4) / (2.0 * assign89920_body44_e138452)), ((locals.var_fbsq__blk2012_dn5 + locals.var_fs01_dn5) / (2.0 * assign89920_body44_e138452)), ((locals.var_fbsq__blk2012_dn6 + locals.var_fs01_dn6) / (2.0 * assign89920_body44_e138452)), ((locals.var_fbsq__blk2012_dn7 + locals.var_fs01_dn7) / (2.0 * assign89920_body44_e138452)), ((locals.var_fbsq__blk2012_dn8 + locals.var_fs01_dn8) / (2.0 * assign89920_body44_e138452)), ((locals.var_fbsq__blk2012_dn9 + locals.var_fs01_dn9) / (2.0 * assign89920_body44_e138452)), ((locals.var_fbsq__blk2012_dn10 + locals.var_fs01_dn10) / (2.0 * assign89920_body44_e138452)), ((locals.var_fbsq__blk2012_dn13 + locals.var_fs01_dn13) / (2.0 * assign89920_body44_e138452)),)
    } else {
        (locals.var_fs02, locals.var_fs02_dn0, locals.var_fs02_dn2, locals.var_fs02_dn4, locals.var_fs02_dn5, locals.var_fs02_dn6, locals.var_fs02_dn7, locals.var_fs02_dn8, locals.var_fs02_dn9, locals.var_fs02_dn10, locals.var_fs02_dn13,)
    }
};
            locals.var_fs02 = assign89920_body44_e138454;
            locals.var_fs02_dn0 = assign89920_body44_e138454_d_n0;
            locals.var_fs02_dn2 = assign89920_body44_e138454_d_n2;
            locals.var_fs02_dn4 = assign89920_body44_e138454_d_n4;
            locals.var_fs02_dn5 = assign89920_body44_e138454_d_n5;
            locals.var_fs02_dn6 = assign89920_body44_e138454_d_n6;
            locals.var_fs02_dn7 = assign89920_body44_e138454_d_n7;
            locals.var_fs02_dn8 = assign89920_body44_e138454_d_n8;
            locals.var_fs02_dn9 = assign89920_body44_e138454_d_n9;
            locals.var_fs02_dn10 = assign89920_body44_e138454_d_n10;
            locals.var_fs02_dn13 = assign89920_body44_e138454_d_n13;
            locals.var_fs02_rv = 0.0;
            let (assign89920_body45_e138468, assign89920_body45_e138468_d_n0, assign89920_body45_e138468_d_n2, assign89920_body45_e138468_d_n4, assign89920_body45_e138468_d_n5, assign89920_body45_e138468_d_n6, assign89920_body45_e138468_d_n7, assign89920_body45_e138468_d_n8, assign89920_body45_e138468_d_n9, assign89920_body45_e138468_d_n10, assign89920_body45_e138468_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) && (locals.var_guard2094 != 0.0)) {
        let assign89920_body45_e138463: f64 = (locals.var_fbsq_dpss__blk2013 + locals.var_fs01_dps0);
        let assign89920_body45_e138464: f64 = (0.5 * assign89920_body45_e138463);
        let assign89920_body45_e138466: f64 = (assign89920_body45_e138464 / locals.var_fs02);
        (assign89920_body45_e138466, ((((0.5 * (locals.var_fbsq_dpss__blk2013_dn0 + locals.var_fs01_dps0_dn0)) * locals.var_fs02) - (assign89920_body45_e138464 * locals.var_fs02_dn0)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * (locals.var_fbsq_dpss__blk2013_dn2 + locals.var_fs01_dps0_dn2)) * locals.var_fs02) - (assign89920_body45_e138464 * locals.var_fs02_dn2)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * (locals.var_fbsq_dpss__blk2013_dn4 + locals.var_fs01_dps0_dn4)) * locals.var_fs02) - (assign89920_body45_e138464 * locals.var_fs02_dn4)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * (locals.var_fbsq_dpss__blk2013_dn5 + locals.var_fs01_dps0_dn5)) * locals.var_fs02) - (assign89920_body45_e138464 * locals.var_fs02_dn5)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * (locals.var_fbsq_dpss__blk2013_dn6 + locals.var_fs01_dps0_dn6)) * locals.var_fs02) - (assign89920_body45_e138464 * locals.var_fs02_dn6)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * (locals.var_fbsq_dpss__blk2013_dn7 + locals.var_fs01_dps0_dn7)) * locals.var_fs02) - (assign89920_body45_e138464 * locals.var_fs02_dn7)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * (locals.var_fbsq_dpss__blk2013_dn8 + locals.var_fs01_dps0_dn8)) * locals.var_fs02) - (assign89920_body45_e138464 * locals.var_fs02_dn8)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * (locals.var_fbsq_dpss__blk2013_dn9 + locals.var_fs01_dps0_dn9)) * locals.var_fs02) - (assign89920_body45_e138464 * locals.var_fs02_dn9)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * (locals.var_fbsq_dpss__blk2013_dn10 + locals.var_fs01_dps0_dn10)) * locals.var_fs02) - (assign89920_body45_e138464 * locals.var_fs02_dn10)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * (locals.var_fbsq_dpss__blk2013_dn13 + locals.var_fs01_dps0_dn13)) * locals.var_fs02) - (assign89920_body45_e138464 * locals.var_fs02_dn13)) / (locals.var_fs02 * locals.var_fs02)),)
    } else {
        (locals.var_fs02_dps0, locals.var_fs02_dps0_dn0, locals.var_fs02_dps0_dn2, locals.var_fs02_dps0_dn4, locals.var_fs02_dps0_dn5, locals.var_fs02_dps0_dn6, locals.var_fs02_dps0_dn7, locals.var_fs02_dps0_dn8, locals.var_fs02_dps0_dn9, locals.var_fs02_dps0_dn10, locals.var_fs02_dps0_dn13,)
    }
};
            locals.var_fs02_dps0 = assign89920_body45_e138468;
            locals.var_fs02_dps0_dn0 = assign89920_body45_e138468_d_n0;
            locals.var_fs02_dps0_dn2 = assign89920_body45_e138468_d_n2;
            locals.var_fs02_dps0_dn4 = assign89920_body45_e138468_d_n4;
            locals.var_fs02_dps0_dn5 = assign89920_body45_e138468_d_n5;
            locals.var_fs02_dps0_dn6 = assign89920_body45_e138468_d_n6;
            locals.var_fs02_dps0_dn7 = assign89920_body45_e138468_d_n7;
            locals.var_fs02_dps0_dn8 = assign89920_body45_e138468_d_n8;
            locals.var_fs02_dps0_dn9 = assign89920_body45_e138468_d_n9;
            locals.var_fs02_dps0_dn10 = assign89920_body45_e138468_d_n10;
            locals.var_fs02_dps0_dn13 = assign89920_body45_e138468_d_n13;
            locals.var_fs02_dps0_rv = 0.0;
            let assign89920_body46_e138471: f64 = if locals.var_fbsq__blk2012 > 0.0 { 1.0 } else { 0.0 };
            locals.var_guard2095 = assign89920_body46_e138471;
            locals.var_guard2095_rv = 0.0;
            let (assign89920_body47_e138483, assign89920_body47_e138483_d_n0, assign89920_body47_e138483_d_n2, assign89920_body47_e138483_d_n4, assign89920_body47_e138483_d_n5, assign89920_body47_e138483_d_n6, assign89920_body47_e138483_d_n7, assign89920_body47_e138483_d_n8, assign89920_body47_e138483_d_n9, assign89920_body47_e138483_d_n10, assign89920_body47_e138483_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) && (locals.var_guard2094 == 0.0)) && (locals.var_guard2095 != 0.0)) {
        let assign89920_body47_e138481: f64 = (locals.var_fbsq__blk2012).sqrt();
        (assign89920_body47_e138481, (locals.var_fbsq__blk2012_dn0 / (2.0 * assign89920_body47_e138481)), (locals.var_fbsq__blk2012_dn2 / (2.0 * assign89920_body47_e138481)), (locals.var_fbsq__blk2012_dn4 / (2.0 * assign89920_body47_e138481)), (locals.var_fbsq__blk2012_dn5 / (2.0 * assign89920_body47_e138481)), (locals.var_fbsq__blk2012_dn6 / (2.0 * assign89920_body47_e138481)), (locals.var_fbsq__blk2012_dn7 / (2.0 * assign89920_body47_e138481)), (locals.var_fbsq__blk2012_dn8 / (2.0 * assign89920_body47_e138481)), (locals.var_fbsq__blk2012_dn9 / (2.0 * assign89920_body47_e138481)), (locals.var_fbsq__blk2012_dn10 / (2.0 * assign89920_body47_e138481)), (locals.var_fbsq__blk2012_dn13 / (2.0 * assign89920_body47_e138481)),)
    } else {
        (locals.var_fs02, locals.var_fs02_dn0, locals.var_fs02_dn2, locals.var_fs02_dn4, locals.var_fs02_dn5, locals.var_fs02_dn6, locals.var_fs02_dn7, locals.var_fs02_dn8, locals.var_fs02_dn9, locals.var_fs02_dn10, locals.var_fs02_dn13,)
    }
};
            locals.var_fs02 = assign89920_body47_e138483;
            locals.var_fs02_dn0 = assign89920_body47_e138483_d_n0;
            locals.var_fs02_dn2 = assign89920_body47_e138483_d_n2;
            locals.var_fs02_dn4 = assign89920_body47_e138483_d_n4;
            locals.var_fs02_dn5 = assign89920_body47_e138483_d_n5;
            locals.var_fs02_dn6 = assign89920_body47_e138483_d_n6;
            locals.var_fs02_dn7 = assign89920_body47_e138483_d_n7;
            locals.var_fs02_dn8 = assign89920_body47_e138483_d_n8;
            locals.var_fs02_dn9 = assign89920_body47_e138483_d_n9;
            locals.var_fs02_dn10 = assign89920_body47_e138483_d_n10;
            locals.var_fs02_dn13 = assign89920_body47_e138483_d_n13;
            locals.var_fs02_rv = 0.0;
            let (assign89920_body48_e138498, assign89920_body48_e138498_d_n0, assign89920_body48_e138498_d_n2, assign89920_body48_e138498_d_n4, assign89920_body48_e138498_d_n5, assign89920_body48_e138498_d_n6, assign89920_body48_e138498_d_n7, assign89920_body48_e138498_d_n8, assign89920_body48_e138498_d_n9, assign89920_body48_e138498_d_n10, assign89920_body48_e138498_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) && (locals.var_guard2094 == 0.0)) && (locals.var_guard2095 != 0.0)) {
        let assign89920_body48_e138494: f64 = (0.5 * locals.var_fbsq_dpss__blk2013);
        let assign89920_body48_e138496: f64 = (assign89920_body48_e138494 / locals.var_fs02);
        (assign89920_body48_e138496, ((((0.5 * locals.var_fbsq_dpss__blk2013_dn0) * locals.var_fs02) - (assign89920_body48_e138494 * locals.var_fs02_dn0)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * locals.var_fbsq_dpss__blk2013_dn2) * locals.var_fs02) - (assign89920_body48_e138494 * locals.var_fs02_dn2)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * locals.var_fbsq_dpss__blk2013_dn4) * locals.var_fs02) - (assign89920_body48_e138494 * locals.var_fs02_dn4)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * locals.var_fbsq_dpss__blk2013_dn5) * locals.var_fs02) - (assign89920_body48_e138494 * locals.var_fs02_dn5)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * locals.var_fbsq_dpss__blk2013_dn6) * locals.var_fs02) - (assign89920_body48_e138494 * locals.var_fs02_dn6)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * locals.var_fbsq_dpss__blk2013_dn7) * locals.var_fs02) - (assign89920_body48_e138494 * locals.var_fs02_dn7)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * locals.var_fbsq_dpss__blk2013_dn8) * locals.var_fs02) - (assign89920_body48_e138494 * locals.var_fs02_dn8)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * locals.var_fbsq_dpss__blk2013_dn9) * locals.var_fs02) - (assign89920_body48_e138494 * locals.var_fs02_dn9)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * locals.var_fbsq_dpss__blk2013_dn10) * locals.var_fs02) - (assign89920_body48_e138494 * locals.var_fs02_dn10)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * locals.var_fbsq_dpss__blk2013_dn13) * locals.var_fs02) - (assign89920_body48_e138494 * locals.var_fs02_dn13)) / (locals.var_fs02 * locals.var_fs02)),)
    } else {
        (locals.var_fs02_dps0, locals.var_fs02_dps0_dn0, locals.var_fs02_dps0_dn2, locals.var_fs02_dps0_dn4, locals.var_fs02_dps0_dn5, locals.var_fs02_dps0_dn6, locals.var_fs02_dps0_dn7, locals.var_fs02_dps0_dn8, locals.var_fs02_dps0_dn9, locals.var_fs02_dps0_dn10, locals.var_fs02_dps0_dn13,)
    }
};
            locals.var_fs02_dps0 = assign89920_body48_e138498;
            locals.var_fs02_dps0_dn0 = assign89920_body48_e138498_d_n0;
            locals.var_fs02_dps0_dn2 = assign89920_body48_e138498_d_n2;
            locals.var_fs02_dps0_dn4 = assign89920_body48_e138498_d_n4;
            locals.var_fs02_dps0_dn5 = assign89920_body48_e138498_d_n5;
            locals.var_fs02_dps0_dn6 = assign89920_body48_e138498_d_n6;
            locals.var_fs02_dps0_dn7 = assign89920_body48_e138498_d_n7;
            locals.var_fs02_dps0_dn8 = assign89920_body48_e138498_d_n8;
            locals.var_fs02_dps0_dn9 = assign89920_body48_e138498_d_n9;
            locals.var_fs02_dps0_dn10 = assign89920_body48_e138498_d_n10;
            locals.var_fs02_dps0_dn13 = assign89920_body48_e138498_d_n13;
            locals.var_fs02_dps0_rv = 0.0;
            let (assign89920_body49_e138510, assign89920_body49_e138510_d_n0, assign89920_body49_e138510_d_n2, assign89920_body49_e138510_d_n4, assign89920_body49_e138510_d_n5, assign89920_body49_e138510_d_n6, assign89920_body49_e138510_d_n7, assign89920_body49_e138510_d_n8, assign89920_body49_e138510_d_n9, assign89920_body49_e138510_d_n10, assign89920_body49_e138510_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) && (locals.var_guard2094 == 0.0)) && (locals.var_guard2095 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fs02, locals.var_fs02_dn0, locals.var_fs02_dn2, locals.var_fs02_dn4, locals.var_fs02_dn5, locals.var_fs02_dn6, locals.var_fs02_dn7, locals.var_fs02_dn8, locals.var_fs02_dn9, locals.var_fs02_dn10, locals.var_fs02_dn13,)
    }
};
            locals.var_fs02 = assign89920_body49_e138510;
            locals.var_fs02_dn0 = assign89920_body49_e138510_d_n0;
            locals.var_fs02_dn2 = assign89920_body49_e138510_d_n2;
            locals.var_fs02_dn4 = assign89920_body49_e138510_d_n4;
            locals.var_fs02_dn5 = assign89920_body49_e138510_d_n5;
            locals.var_fs02_dn6 = assign89920_body49_e138510_d_n6;
            locals.var_fs02_dn7 = assign89920_body49_e138510_d_n7;
            locals.var_fs02_dn8 = assign89920_body49_e138510_d_n8;
            locals.var_fs02_dn9 = assign89920_body49_e138510_d_n9;
            locals.var_fs02_dn10 = assign89920_body49_e138510_d_n10;
            locals.var_fs02_dn13 = assign89920_body49_e138510_d_n13;
            locals.var_fs02_rv = 0.0;
            let (assign89920_body50_e138522, assign89920_body50_e138522_d_n0, assign89920_body50_e138522_d_n2, assign89920_body50_e138522_d_n4, assign89920_body50_e138522_d_n5, assign89920_body50_e138522_d_n6, assign89920_body50_e138522_d_n7, assign89920_body50_e138522_d_n8, assign89920_body50_e138522_d_n9, assign89920_body50_e138522_d_n10, assign89920_body50_e138522_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) && (locals.var_guard2094 == 0.0)) && (locals.var_guard2095 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fs02_dps0, locals.var_fs02_dps0_dn0, locals.var_fs02_dps0_dn2, locals.var_fs02_dps0_dn4, locals.var_fs02_dps0_dn5, locals.var_fs02_dps0_dn6, locals.var_fs02_dps0_dn7, locals.var_fs02_dps0_dn8, locals.var_fs02_dps0_dn9, locals.var_fs02_dps0_dn10, locals.var_fs02_dps0_dn13,)
    }
};
            locals.var_fs02_dps0 = assign89920_body50_e138522;
            locals.var_fs02_dps0_dn0 = assign89920_body50_e138522_d_n0;
            locals.var_fs02_dps0_dn2 = assign89920_body50_e138522_d_n2;
            locals.var_fs02_dps0_dn4 = assign89920_body50_e138522_d_n4;
            locals.var_fs02_dps0_dn5 = assign89920_body50_e138522_d_n5;
            locals.var_fs02_dps0_dn6 = assign89920_body50_e138522_d_n6;
            locals.var_fs02_dps0_dn7 = assign89920_body50_e138522_d_n7;
            locals.var_fs02_dps0_dn8 = assign89920_body50_e138522_d_n8;
            locals.var_fs02_dps0_dn9 = assign89920_body50_e138522_d_n9;
            locals.var_fs02_dps0_dn10 = assign89920_body50_e138522_d_n10;
            locals.var_fs02_dps0_dn13 = assign89920_body50_e138522_d_n13;
            locals.var_fs02_dps0_rv = 0.0;
            let (assign89920_body51_e138536, assign89920_body51_e138536_d_n0, assign89920_body51_e138536_d_n2, assign89920_body51_e138536_d_n4, assign89920_body51_e138536_d_n5, assign89920_body51_e138536_d_n6, assign89920_body51_e138536_d_n7, assign89920_body51_e138536_d_n8, assign89920_body51_e138536_d_n9, assign89920_body51_e138536_d_n10, assign89920_body51_e138536_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) {
        let (assign89920_body51_e138532,) = {
            if (locals.var_chi >= 0.0) {
                (1.0,)
            } else {
                let assign89920_body51_e138531: f64 = (-1.0);
                (assign89920_body51_e138531,)
            }
        };
        let assign89920_body51_e138534: f64 = (assign89920_body51_e138532 * locals.var_fs02);
        (assign89920_body51_e138534, (assign89920_body51_e138532 * locals.var_fs02_dn0), (assign89920_body51_e138532 * locals.var_fs02_dn2), (assign89920_body51_e138532 * locals.var_fs02_dn4), (assign89920_body51_e138532 * locals.var_fs02_dn5), (assign89920_body51_e138532 * locals.var_fs02_dn6), (assign89920_body51_e138532 * locals.var_fs02_dn7), (assign89920_body51_e138532 * locals.var_fs02_dn8), (assign89920_body51_e138532 * locals.var_fs02_dn9), (assign89920_body51_e138532 * locals.var_fs02_dn10), (assign89920_body51_e138532 * locals.var_fs02_dn13),)
    } else {
        (locals.var_fs02, locals.var_fs02_dn0, locals.var_fs02_dn2, locals.var_fs02_dn4, locals.var_fs02_dn5, locals.var_fs02_dn6, locals.var_fs02_dn7, locals.var_fs02_dn8, locals.var_fs02_dn9, locals.var_fs02_dn10, locals.var_fs02_dn13,)
    }
};
            locals.var_fs02 = assign89920_body51_e138536;
            locals.var_fs02_dn0 = assign89920_body51_e138536_d_n0;
            locals.var_fs02_dn2 = assign89920_body51_e138536_d_n2;
            locals.var_fs02_dn4 = assign89920_body51_e138536_d_n4;
            locals.var_fs02_dn5 = assign89920_body51_e138536_d_n5;
            locals.var_fs02_dn6 = assign89920_body51_e138536_d_n6;
            locals.var_fs02_dn7 = assign89920_body51_e138536_d_n7;
            locals.var_fs02_dn8 = assign89920_body51_e138536_d_n8;
            locals.var_fs02_dn9 = assign89920_body51_e138536_d_n9;
            locals.var_fs02_dn10 = assign89920_body51_e138536_d_n10;
            locals.var_fs02_dn13 = assign89920_body51_e138536_d_n13;
            locals.var_fs02_rv = 0.0;
            let (assign89920_body52_e138550, assign89920_body52_e138550_d_n0, assign89920_body52_e138550_d_n2, assign89920_body52_e138550_d_n4, assign89920_body52_e138550_d_n5, assign89920_body52_e138550_d_n6, assign89920_body52_e138550_d_n7, assign89920_body52_e138550_d_n8, assign89920_body52_e138550_d_n9, assign89920_body52_e138550_d_n10, assign89920_body52_e138550_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) {
        let (assign89920_body52_e138546,) = {
            if (locals.var_chi >= 0.0) {
                (1.0,)
            } else {
                let assign89920_body52_e138545: f64 = (-1.0);
                (assign89920_body52_e138545,)
            }
        };
        let assign89920_body52_e138548: f64 = (assign89920_body52_e138546 * locals.var_fs02_dps0);
        (assign89920_body52_e138548, (assign89920_body52_e138546 * locals.var_fs02_dps0_dn0), (assign89920_body52_e138546 * locals.var_fs02_dps0_dn2), (assign89920_body52_e138546 * locals.var_fs02_dps0_dn4), (assign89920_body52_e138546 * locals.var_fs02_dps0_dn5), (assign89920_body52_e138546 * locals.var_fs02_dps0_dn6), (assign89920_body52_e138546 * locals.var_fs02_dps0_dn7), (assign89920_body52_e138546 * locals.var_fs02_dps0_dn8), (assign89920_body52_e138546 * locals.var_fs02_dps0_dn9), (assign89920_body52_e138546 * locals.var_fs02_dps0_dn10), (assign89920_body52_e138546 * locals.var_fs02_dps0_dn13),)
    } else {
        (locals.var_fs02_dps0, locals.var_fs02_dps0_dn0, locals.var_fs02_dps0_dn2, locals.var_fs02_dps0_dn4, locals.var_fs02_dps0_dn5, locals.var_fs02_dps0_dn6, locals.var_fs02_dps0_dn7, locals.var_fs02_dps0_dn8, locals.var_fs02_dps0_dn9, locals.var_fs02_dps0_dn10, locals.var_fs02_dps0_dn13,)
    }
};
            locals.var_fs02_dps0 = assign89920_body52_e138550;
            locals.var_fs02_dps0_dn0 = assign89920_body52_e138550_d_n0;
            locals.var_fs02_dps0_dn2 = assign89920_body52_e138550_d_n2;
            locals.var_fs02_dps0_dn4 = assign89920_body52_e138550_d_n4;
            locals.var_fs02_dps0_dn5 = assign89920_body52_e138550_d_n5;
            locals.var_fs02_dps0_dn6 = assign89920_body52_e138550_d_n6;
            locals.var_fs02_dps0_dn7 = assign89920_body52_e138550_d_n7;
            locals.var_fs02_dps0_dn8 = assign89920_body52_e138550_d_n8;
            locals.var_fs02_dps0_dn9 = assign89920_body52_e138550_d_n9;
            locals.var_fs02_dps0_dn10 = assign89920_body52_e138550_d_n10;
            locals.var_fs02_dps0_dn13 = assign89920_body52_e138550_d_n13;
            locals.var_fs02_dps0_rv = 0.0;
            let (assign89920_body53_e138563, assign89920_body53_e138563_d_n0, assign89920_body53_e138563_d_n2, assign89920_body53_e138563_d_n4, assign89920_body53_e138563_d_n5, assign89920_body53_e138563_d_n6, assign89920_body53_e138563_d_n7, assign89920_body53_e138563_d_n8, assign89920_body53_e138563_d_n9, assign89920_body53_e138563_d_n10, assign89920_body53_e138563_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) {
        let assign89920_body53_e138555: f64 = (-locals.var_vgpld);
        let assign89920_body53_e138557: f64 = (assign89920_body53_e138555 + locals.var_ps0ld);
        let assign89920_body53_e138560: f64 = (locals.var_fac1 * locals.var_fs02);
        let assign89920_body53_e138561: f64 = (assign89920_body53_e138557 + assign89920_body53_e138560);
        (assign89920_body53_e138561, (locals.var_ps0ld_dn0 + ((locals.var_fac1_dn0 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn0))), (((-locals.var_vgpld_dn2) + locals.var_ps0ld_dn2) + ((locals.var_fac1_dn2 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn2))), (locals.var_ps0ld_dn4 + ((locals.var_fac1_dn4 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn4))), (locals.var_ps0ld_dn5 + ((locals.var_fac1_dn5 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn5))), (((-locals.var_vgpld_dn6) + locals.var_ps0ld_dn6) + ((locals.var_fac1_dn6 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn6))), (((-locals.var_vgpld_dn7) + locals.var_ps0ld_dn7) + ((locals.var_fac1_dn7 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn7))), (((-locals.var_vgpld_dn8) + locals.var_ps0ld_dn8) + ((locals.var_fac1_dn8 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn8))), (locals.var_ps0ld_dn9 + ((locals.var_fac1_dn9 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn9))), (locals.var_ps0ld_dn10 + ((locals.var_fac1_dn10 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn10))), (locals.var_ps0ld_dn13 + ((locals.var_fac1_dn13 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn13))),)
    } else {
        (locals.var_fs0, locals.var_fs0_dn0, locals.var_fs0_dn2, locals.var_fs0_dn4, locals.var_fs0_dn5, locals.var_fs0_dn6, locals.var_fs0_dn7, locals.var_fs0_dn8, locals.var_fs0_dn9, locals.var_fs0_dn10, locals.var_fs0_dn13,)
    }
};
            locals.var_fs0 = assign89920_body53_e138563;
            locals.var_fs0_dn0 = assign89920_body53_e138563_d_n0;
            locals.var_fs0_dn2 = assign89920_body53_e138563_d_n2;
            locals.var_fs0_dn4 = assign89920_body53_e138563_d_n4;
            locals.var_fs0_dn5 = assign89920_body53_e138563_d_n5;
            locals.var_fs0_dn6 = assign89920_body53_e138563_d_n6;
            locals.var_fs0_dn7 = assign89920_body53_e138563_d_n7;
            locals.var_fs0_dn8 = assign89920_body53_e138563_d_n8;
            locals.var_fs0_dn9 = assign89920_body53_e138563_d_n9;
            locals.var_fs0_dn10 = assign89920_body53_e138563_d_n10;
            locals.var_fs0_dn13 = assign89920_body53_e138563_d_n13;
            locals.var_fs0_rv = 0.0;
            let (assign89920_body54_e138573, assign89920_body54_e138573_d_n0, assign89920_body54_e138573_d_n2, assign89920_body54_e138573_d_n4, assign89920_body54_e138573_d_n5, assign89920_body54_e138573_d_n6, assign89920_body54_e138573_d_n7, assign89920_body54_e138573_d_n8, assign89920_body54_e138573_d_n9, assign89920_body54_e138573_d_n10, assign89920_body54_e138573_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) {
        let assign89920_body54_e138570: f64 = (locals.var_fac1 * locals.var_fs02_dps0);
        let assign89920_body54_e138571: f64 = (1.0 + assign89920_body54_e138570);
        (assign89920_body54_e138571, ((locals.var_fac1_dn0 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn0)), ((locals.var_fac1_dn2 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn2)), ((locals.var_fac1_dn4 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn4)), ((locals.var_fac1_dn5 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn5)), ((locals.var_fac1_dn6 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn6)), ((locals.var_fac1_dn7 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn7)), ((locals.var_fac1_dn8 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn8)), ((locals.var_fac1_dn9 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn9)), ((locals.var_fac1_dn10 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn10)), ((locals.var_fac1_dn13 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn13)),)
    } else {
        (locals.var_fs0_dps0, locals.var_fs0_dps0_dn0, locals.var_fs0_dps0_dn2, locals.var_fs0_dps0_dn4, locals.var_fs0_dps0_dn5, locals.var_fs0_dps0_dn6, locals.var_fs0_dps0_dn7, locals.var_fs0_dps0_dn8, locals.var_fs0_dps0_dn9, locals.var_fs0_dps0_dn10, locals.var_fs0_dps0_dn13,)
    }
};
            locals.var_fs0_dps0 = assign89920_body54_e138573;
            locals.var_fs0_dps0_dn0 = assign89920_body54_e138573_d_n0;
            locals.var_fs0_dps0_dn2 = assign89920_body54_e138573_d_n2;
            locals.var_fs0_dps0_dn4 = assign89920_body54_e138573_d_n4;
            locals.var_fs0_dps0_dn5 = assign89920_body54_e138573_d_n5;
            locals.var_fs0_dps0_dn6 = assign89920_body54_e138573_d_n6;
            locals.var_fs0_dps0_dn7 = assign89920_body54_e138573_d_n7;
            locals.var_fs0_dps0_dn8 = assign89920_body54_e138573_d_n8;
            locals.var_fs0_dps0_dn9 = assign89920_body54_e138573_d_n9;
            locals.var_fs0_dps0_dn10 = assign89920_body54_e138573_d_n10;
            locals.var_fs0_dps0_dn13 = assign89920_body54_e138573_d_n13;
            locals.var_fs0_dps0_rv = 0.0;
            let assign89920_body55_e138576: f64 = if locals.var_flg_conv > 0.0 { 1.0 } else { 0.0 };
            locals.var_guard2096 = assign89920_body55_e138576;
            locals.var_guard2096_rv = 0.0;
            let (assign89920_body56_e138586,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) && (locals.var_guard2096 != 0.0)) {
        let assign89920_body56_e138584: f64 = (locals.var_lp_s0_max + 1.0);
        (assign89920_body56_e138584,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign89920_body56_e138586;
            locals.var_lp_s0_rv = 0.0;
            let (assign89920_body57_e138598, assign89920_body57_e138598_d_n0, assign89920_body57_e138598_d_n2, assign89920_body57_e138598_d_n4, assign89920_body57_e138598_d_n5, assign89920_body57_e138598_d_n6, assign89920_body57_e138598_d_n7, assign89920_body57_e138598_d_n8, assign89920_body57_e138598_d_n9, assign89920_body57_e138598_d_n10, assign89920_body57_e138598_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) && (locals.var_guard2096 == 0.0)) {
        let assign89920_body57_e138594: f64 = (-locals.var_fs0);
        let assign89920_body57_e138596: f64 = (assign89920_body57_e138594 / locals.var_fs0_dps0);
        (assign89920_body57_e138596, ((((-locals.var_fs0_dn0) * locals.var_fs0_dps0) - (assign89920_body57_e138594 * locals.var_fs0_dps0_dn0)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn2) * locals.var_fs0_dps0) - (assign89920_body57_e138594 * locals.var_fs0_dps0_dn2)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn4) * locals.var_fs0_dps0) - (assign89920_body57_e138594 * locals.var_fs0_dps0_dn4)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn5) * locals.var_fs0_dps0) - (assign89920_body57_e138594 * locals.var_fs0_dps0_dn5)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn6) * locals.var_fs0_dps0) - (assign89920_body57_e138594 * locals.var_fs0_dps0_dn6)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn7) * locals.var_fs0_dps0) - (assign89920_body57_e138594 * locals.var_fs0_dps0_dn7)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn8) * locals.var_fs0_dps0) - (assign89920_body57_e138594 * locals.var_fs0_dps0_dn8)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn9) * locals.var_fs0_dps0) - (assign89920_body57_e138594 * locals.var_fs0_dps0_dn9)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn10) * locals.var_fs0_dps0) - (assign89920_body57_e138594 * locals.var_fs0_dps0_dn10)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn13) * locals.var_fs0_dps0) - (assign89920_body57_e138594 * locals.var_fs0_dps0_dn13)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)),)
    } else {
        (locals.var_dps0, locals.var_dps0_dn0, locals.var_dps0_dn2, locals.var_dps0_dn4, locals.var_dps0_dn5, locals.var_dps0_dn6, locals.var_dps0_dn7, locals.var_dps0_dn8, locals.var_dps0_dn9, locals.var_dps0_dn10, locals.var_dps0_dn13,)
    }
};
            locals.var_dps0 = assign89920_body57_e138598;
            locals.var_dps0_dn0 = assign89920_body57_e138598_d_n0;
            locals.var_dps0_dn2 = assign89920_body57_e138598_d_n2;
            locals.var_dps0_dn4 = assign89920_body57_e138598_d_n4;
            locals.var_dps0_dn5 = assign89920_body57_e138598_d_n5;
            locals.var_dps0_dn6 = assign89920_body57_e138598_d_n6;
            locals.var_dps0_dn7 = assign89920_body57_e138598_d_n7;
            locals.var_dps0_dn8 = assign89920_body57_e138598_d_n8;
            locals.var_dps0_dn9 = assign89920_body57_e138598_d_n9;
            locals.var_dps0_dn10 = assign89920_body57_e138598_d_n10;
            locals.var_dps0_dn13 = assign89920_body57_e138598_d_n13;
            locals.var_dps0_rv = 0.0;
            let (assign89920_body58_e138620, assign89920_body58_e138620_d_n0, assign89920_body58_e138620_d_n2, assign89920_body58_e138620_d_n4, assign89920_body58_e138620_d_n5, assign89920_body58_e138620_d_n6, assign89920_body58_e138620_d_n7, assign89920_body58_e138620_d_n8, assign89920_body58_e138620_d_n9, assign89920_body58_e138620_d_n10, assign89920_body58_e138620_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) && (locals.var_guard2096 == 0.0)) {
        let assign89920_body58_e138607: f64 = (0.5 * 0.1);
        let assign89920_body58_e138611: f64 = (locals.var_ps0ld).abs();
        let (assign89920_body58_e138616, assign89920_body58_e138616_d_n0, assign89920_body58_e138616_d_n2, assign89920_body58_e138616_d_n4, assign89920_body58_e138616_d_n5, assign89920_body58_e138616_d_n6, assign89920_body58_e138616_d_n7, assign89920_body58_e138616_d_n8, assign89920_body58_e138616_d_n9, assign89920_body58_e138616_d_n10, assign89920_body58_e138616_d_n13,) = {
            if (1.0 >= assign89920_body58_e138611) {
                (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign89920_body58_e138615: f64 = (locals.var_ps0ld).abs();
                (assign89920_body58_e138615, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn0 } else { (-locals.var_ps0ld_dn0) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn2 } else { (-locals.var_ps0ld_dn2) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn4 } else { (-locals.var_ps0ld_dn4) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn5 } else { (-locals.var_ps0ld_dn5) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn6 } else { (-locals.var_ps0ld_dn6) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn7 } else { (-locals.var_ps0ld_dn7) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn8 } else { (-locals.var_ps0ld_dn8) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn9 } else { (-locals.var_ps0ld_dn9) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn10 } else { (-locals.var_ps0ld_dn10) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn13 } else { (-locals.var_ps0ld_dn13) },)
            }
        };
        let assign89920_body58_e138617: f64 = (1.0 + assign89920_body58_e138616);
        let assign89920_body58_e138618: f64 = (assign89920_body58_e138607 * assign89920_body58_e138617);
        (assign89920_body58_e138618, (assign89920_body58_e138607 * assign89920_body58_e138616_d_n0), (assign89920_body58_e138607 * assign89920_body58_e138616_d_n2), (assign89920_body58_e138607 * assign89920_body58_e138616_d_n4), (assign89920_body58_e138607 * assign89920_body58_e138616_d_n5), (assign89920_body58_e138607 * assign89920_body58_e138616_d_n6), (assign89920_body58_e138607 * assign89920_body58_e138616_d_n7), (assign89920_body58_e138607 * assign89920_body58_e138616_d_n8), (assign89920_body58_e138607 * assign89920_body58_e138616_d_n9), (assign89920_body58_e138607 * assign89920_body58_e138616_d_n10), (assign89920_body58_e138607 * assign89920_body58_e138616_d_n13),)
    } else {
        (locals.var_dplim, locals.var_dplim_dn0, locals.var_dplim_dn2, locals.var_dplim_dn4, locals.var_dplim_dn5, locals.var_dplim_dn6, locals.var_dplim_dn7, locals.var_dplim_dn8, locals.var_dplim_dn9, locals.var_dplim_dn10, locals.var_dplim_dn13,)
    }
};
            locals.var_dplim = assign89920_body58_e138620;
            locals.var_dplim_dn0 = assign89920_body58_e138620_d_n0;
            locals.var_dplim_dn2 = assign89920_body58_e138620_d_n2;
            locals.var_dplim_dn4 = assign89920_body58_e138620_d_n4;
            locals.var_dplim_dn5 = assign89920_body58_e138620_d_n5;
            locals.var_dplim_dn6 = assign89920_body58_e138620_d_n6;
            locals.var_dplim_dn7 = assign89920_body58_e138620_d_n7;
            locals.var_dplim_dn8 = assign89920_body58_e138620_d_n8;
            locals.var_dplim_dn9 = assign89920_body58_e138620_d_n9;
            locals.var_dplim_dn10 = assign89920_body58_e138620_d_n10;
            locals.var_dplim_dn13 = assign89920_body58_e138620_d_n13;
            locals.var_dplim_rv = 0.0;
            let assign89920_body59_e138622: f64 = (locals.var_dps0).abs();
            let assign89920_body59_e138624: f64 = if assign89920_body59_e138622 > locals.var_dplim { 1.0 } else { 0.0 };
            locals.var_guard2097 = assign89920_body59_e138624;
            locals.var_guard2097_rv = 0.0;
            let (assign89920_body60_e138643, assign89920_body60_e138643_d_n0, assign89920_body60_e138643_d_n2, assign89920_body60_e138643_d_n4, assign89920_body60_e138643_d_n5, assign89920_body60_e138643_d_n6, assign89920_body60_e138643_d_n7, assign89920_body60_e138643_d_n8, assign89920_body60_e138643_d_n9, assign89920_body60_e138643_d_n10, assign89920_body60_e138643_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) && (locals.var_guard2096 == 0.0)) && (locals.var_guard2097 != 0.0)) {
        let (assign89920_body60_e138640,) = {
            if (locals.var_dps0 >= 0.0) {
                (1.0,)
            } else {
                let assign89920_body60_e138639: f64 = (-1.0);
                (assign89920_body60_e138639,)
            }
        };
        let assign89920_body60_e138641: f64 = (locals.var_dplim * assign89920_body60_e138640);
        (assign89920_body60_e138641, (locals.var_dplim_dn0 * assign89920_body60_e138640), (locals.var_dplim_dn2 * assign89920_body60_e138640), (locals.var_dplim_dn4 * assign89920_body60_e138640), (locals.var_dplim_dn5 * assign89920_body60_e138640), (locals.var_dplim_dn6 * assign89920_body60_e138640), (locals.var_dplim_dn7 * assign89920_body60_e138640), (locals.var_dplim_dn8 * assign89920_body60_e138640), (locals.var_dplim_dn9 * assign89920_body60_e138640), (locals.var_dplim_dn10 * assign89920_body60_e138640), (locals.var_dplim_dn13 * assign89920_body60_e138640),)
    } else {
        (locals.var_dps0, locals.var_dps0_dn0, locals.var_dps0_dn2, locals.var_dps0_dn4, locals.var_dps0_dn5, locals.var_dps0_dn6, locals.var_dps0_dn7, locals.var_dps0_dn8, locals.var_dps0_dn9, locals.var_dps0_dn10, locals.var_dps0_dn13,)
    }
};
            locals.var_dps0 = assign89920_body60_e138643;
            locals.var_dps0_dn0 = assign89920_body60_e138643_d_n0;
            locals.var_dps0_dn2 = assign89920_body60_e138643_d_n2;
            locals.var_dps0_dn4 = assign89920_body60_e138643_d_n4;
            locals.var_dps0_dn5 = assign89920_body60_e138643_d_n5;
            locals.var_dps0_dn6 = assign89920_body60_e138643_d_n6;
            locals.var_dps0_dn7 = assign89920_body60_e138643_d_n7;
            locals.var_dps0_dn8 = assign89920_body60_e138643_d_n8;
            locals.var_dps0_dn9 = assign89920_body60_e138643_d_n9;
            locals.var_dps0_dn10 = assign89920_body60_e138643_d_n10;
            locals.var_dps0_dn13 = assign89920_body60_e138643_d_n13;
            locals.var_dps0_rv = 0.0;
            let (assign89920_body61_e138654, assign89920_body61_e138654_d_n0, assign89920_body61_e138654_d_n2, assign89920_body61_e138654_d_n4, assign89920_body61_e138654_d_n5, assign89920_body61_e138654_d_n6, assign89920_body61_e138654_d_n7, assign89920_body61_e138654_d_n8, assign89920_body61_e138654_d_n9, assign89920_body61_e138654_d_n10, assign89920_body61_e138654_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) && (locals.var_guard2096 == 0.0)) {
        let assign89920_body61_e138652: f64 = (locals.var_ps0ld + locals.var_dps0);
        (assign89920_body61_e138652, (locals.var_ps0ld_dn0 + locals.var_dps0_dn0), (locals.var_ps0ld_dn2 + locals.var_dps0_dn2), (locals.var_ps0ld_dn4 + locals.var_dps0_dn4), (locals.var_ps0ld_dn5 + locals.var_dps0_dn5), (locals.var_ps0ld_dn6 + locals.var_dps0_dn6), (locals.var_ps0ld_dn7 + locals.var_dps0_dn7), (locals.var_ps0ld_dn8 + locals.var_dps0_dn8), (locals.var_ps0ld_dn9 + locals.var_dps0_dn9), (locals.var_ps0ld_dn10 + locals.var_dps0_dn10), (locals.var_ps0ld_dn13 + locals.var_dps0_dn13),)
    } else {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn13,)
    }
};
            locals.var_ps0ld = assign89920_body61_e138654;
            locals.var_ps0ld_dn0 = assign89920_body61_e138654_d_n0;
            locals.var_ps0ld_dn2 = assign89920_body61_e138654_d_n2;
            locals.var_ps0ld_dn4 = assign89920_body61_e138654_d_n4;
            locals.var_ps0ld_dn5 = assign89920_body61_e138654_d_n5;
            locals.var_ps0ld_dn6 = assign89920_body61_e138654_d_n6;
            locals.var_ps0ld_dn7 = assign89920_body61_e138654_d_n7;
            locals.var_ps0ld_dn8 = assign89920_body61_e138654_d_n8;
            locals.var_ps0ld_dn9 = assign89920_body61_e138654_d_n9;
            locals.var_ps0ld_dn10 = assign89920_body61_e138654_d_n10;
            locals.var_ps0ld_dn13 = assign89920_body61_e138654_d_n13;
            locals.var_ps0ld_rv = 0.0;
            let assign89920_body62_e138656: f64 = (locals.var_dps0).abs();
            let assign89920_body62_e138660: f64 = (locals.var_fs0).abs();
            let assign89920_body62_e138663: f64 = if ((assign89920_body62_e138656 <= 1e-12) && (assign89920_body62_e138660 <= 1e-8)) { 1.0 } else { 0.0 };
            locals.var_guard2098 = assign89920_body62_e138663;
            locals.var_guard2098_rv = 0.0;
            let (assign89920_body63_e138676,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) && (locals.var_guard2096 == 0.0)) && (locals.var_guard2098 != 0.0)) {
        let assign89920_body63_e138674: f64 = (locals.var_flg_conv + 2.0);
        (assign89920_body63_e138674,)
    } else {
        (locals.var_flg_conv,)
    }
};
            locals.var_flg_conv = assign89920_body63_e138676;
            locals.var_flg_conv_rv = 0.0;
            let (assign89920_body64_e138684,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) {
        let assign89920_body64_e138682: f64 = (locals.var_lp_s0 + 1.0);
        (assign89920_body64_e138682,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign89920_body64_e138684;
            locals.var_lp_s0_rv = 0.0;
        }

    }

    pub(super) fn stamp_reactive_block_336(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign89940_e138707, assign89940_e138707_d_n0, assign89940_e138707_d_n2, assign89940_e138707_d_n4, assign89940_e138707_d_n5, assign89940_e138707_d_n6, assign89940_e138707_d_n7, assign89940_e138707_d_n8, assign89940_e138707_d_n9, assign89940_e138707_d_n10, assign89940_e138707_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) {
        let (assign89940_e138705, assign89940_e138705_d_n0, assign89940_e138705_d_n2, assign89940_e138705_d_n4, assign89940_e138705_d_n5, assign89940_e138705_d_n6, assign89940_e138705_d_n7, assign89940_e138705_d_n8, assign89940_e138705_d_n9, assign89940_e138705_d_n10, assign89940_e138705_d_n13,) = {
            if (locals.var_fbsq__blk2012 >= 0.0) {
                let (assign89940_e138700,) = {
                    if (locals.var_chi >= 0.0) {
                        (1.0,)
                    } else {
                        let assign89940_e138699: f64 = (-1.0);
                        (assign89940_e138699,)
                    }
                };
                let assign89940_e138702: f64 = (locals.var_fbsq__blk2012).sqrt();
                let assign89940_e138703: f64 = (assign89940_e138700 * assign89940_e138702);
                (assign89940_e138703, (assign89940_e138700 * (locals.var_fbsq__blk2012_dn0 / (2.0 * assign89940_e138702))), (assign89940_e138700 * (locals.var_fbsq__blk2012_dn2 / (2.0 * assign89940_e138702))), (assign89940_e138700 * (locals.var_fbsq__blk2012_dn4 / (2.0 * assign89940_e138702))), (assign89940_e138700 * (locals.var_fbsq__blk2012_dn5 / (2.0 * assign89940_e138702))), (assign89940_e138700 * (locals.var_fbsq__blk2012_dn6 / (2.0 * assign89940_e138702))), (assign89940_e138700 * (locals.var_fbsq__blk2012_dn7 / (2.0 * assign89940_e138702))), (assign89940_e138700 * (locals.var_fbsq__blk2012_dn8 / (2.0 * assign89940_e138702))), (assign89940_e138700 * (locals.var_fbsq__blk2012_dn9 / (2.0 * assign89940_e138702))), (assign89940_e138700 * (locals.var_fbsq__blk2012_dn10 / (2.0 * assign89940_e138702))), (assign89940_e138700 * (locals.var_fbsq__blk2012_dn13 / (2.0 * assign89940_e138702))),)
            } else {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign89940_e138705, assign89940_e138705_d_n0, assign89940_e138705_d_n2, assign89940_e138705_d_n4, assign89940_e138705_d_n5, assign89940_e138705_d_n6, assign89940_e138705_d_n7, assign89940_e138705_d_n8, assign89940_e138705_d_n9, assign89940_e138705_d_n10, assign89940_e138705_d_n13,)
    } else {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn4, locals.var_fb_dn5, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn8, locals.var_fb_dn9, locals.var_fb_dn10, locals.var_fb_dn13,)
    }
};
        locals.var_fb = assign89940_e138707;
        locals.var_fb_dn0 = assign89940_e138707_d_n0;
        locals.var_fb_dn2 = assign89940_e138707_d_n2;
        locals.var_fb_dn4 = assign89940_e138707_d_n4;
        locals.var_fb_dn5 = assign89940_e138707_d_n5;
        locals.var_fb_dn6 = assign89940_e138707_d_n6;
        locals.var_fb_dn7 = assign89940_e138707_d_n7;
        locals.var_fb_dn8 = assign89940_e138707_d_n8;
        locals.var_fb_dn9 = assign89940_e138707_d_n9;
        locals.var_fb_dn10 = assign89940_e138707_d_n10;
        locals.var_fb_dn13 = assign89940_e138707_d_n13;
        locals.var_fb_rv = 0.0;

        let (assign89950_e138715, assign89950_e138715_d_n0, assign89950_e138715_d_n2, assign89950_e138715_d_n4, assign89950_e138715_d_n5, assign89950_e138715_d_n6, assign89950_e138715_d_n7, assign89950_e138715_d_n8, assign89950_e138715_d_n9, assign89950_e138715_d_n10, assign89950_e138715_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) {
        let assign89950_e138713: f64 = (locals.var_c_w_ld * locals.var_fb);
        (assign89950_e138713, ((locals.var_c_w_ld_dn0 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn0)), ((locals.var_c_w_ld_dn2 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn2)), ((locals.var_c_w_ld_dn4 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn4)), ((locals.var_c_w_ld_dn5 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn5)), ((locals.var_c_w_ld_dn6 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn6)), ((locals.var_c_w_ld_dn7 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn7)), ((locals.var_c_w_ld_dn8 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn8)), ((locals.var_c_w_ld_dn9 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn9)), ((locals.var_c_w_ld_dn10 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn10)), ((locals.var_c_w_ld_dn13 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn13)),)
    } else {
        (locals.var_wdld__blk2002, locals.var_wdld__blk2002_dn0, locals.var_wdld__blk2002_dn2, locals.var_wdld__blk2002_dn4, locals.var_wdld__blk2002_dn5, locals.var_wdld__blk2002_dn6, locals.var_wdld__blk2002_dn7, locals.var_wdld__blk2002_dn8, locals.var_wdld__blk2002_dn9, locals.var_wdld__blk2002_dn10, locals.var_wdld__blk2002_dn13,)
    }
};
        locals.var_wdld__blk2002 = assign89950_e138715;
        locals.var_wdld__blk2002_dn0 = assign89950_e138715_d_n0;
        locals.var_wdld__blk2002_dn2 = assign89950_e138715_d_n2;
        locals.var_wdld__blk2002_dn4 = assign89950_e138715_d_n4;
        locals.var_wdld__blk2002_dn5 = assign89950_e138715_d_n5;
        locals.var_wdld__blk2002_dn6 = assign89950_e138715_d_n6;
        locals.var_wdld__blk2002_dn7 = assign89950_e138715_d_n7;
        locals.var_wdld__blk2002_dn8 = assign89950_e138715_d_n8;
        locals.var_wdld__blk2002_dn9 = assign89950_e138715_d_n9;
        locals.var_wdld__blk2002_dn10 = assign89950_e138715_d_n10;
        locals.var_wdld__blk2002_dn13 = assign89950_e138715_d_n13;
        locals.var_wdld__blk2002_rv = 0.0;

        let (assign89960_e138723, assign89960_e138723_d_n0, assign89960_e138723_d_n2, assign89960_e138723_d_n4, assign89960_e138723_d_n5, assign89960_e138723_d_n6, assign89960_e138723_d_n7, assign89960_e138723_d_n8, assign89960_e138723_d_n9, assign89960_e138723_d_n10, assign89960_e138723_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) {
        let assign89960_e138721: f64 = (locals.var_q_nsubld__blk2004 * locals.var_wdld__blk2002);
        (assign89960_e138721, (locals.var_q_nsubld__blk2004 * locals.var_wdld__blk2002_dn0), (locals.var_q_nsubld__blk2004 * locals.var_wdld__blk2002_dn2), (locals.var_q_nsubld__blk2004 * locals.var_wdld__blk2002_dn4), (locals.var_q_nsubld__blk2004 * locals.var_wdld__blk2002_dn5), (locals.var_q_nsubld__blk2004 * locals.var_wdld__blk2002_dn6), (locals.var_q_nsubld__blk2004 * locals.var_wdld__blk2002_dn7), (locals.var_q_nsubld__blk2004 * locals.var_wdld__blk2002_dn8), (locals.var_q_nsubld__blk2004 * locals.var_wdld__blk2002_dn9), (locals.var_q_nsubld__blk2004 * locals.var_wdld__blk2002_dn10), (locals.var_q_nsubld__blk2004 * locals.var_wdld__blk2002_dn13),)
    } else {
        (locals.var_q_dep_ld__blk2003, locals.var_q_dep_ld__blk2003_dn0, locals.var_q_dep_ld__blk2003_dn2, locals.var_q_dep_ld__blk2003_dn4, locals.var_q_dep_ld__blk2003_dn5, locals.var_q_dep_ld__blk2003_dn6, locals.var_q_dep_ld__blk2003_dn7, locals.var_q_dep_ld__blk2003_dn8, locals.var_q_dep_ld__blk2003_dn9, locals.var_q_dep_ld__blk2003_dn10, locals.var_q_dep_ld__blk2003_dn13,)
    }
};
        locals.var_q_dep_ld__blk2003 = assign89960_e138723;
        locals.var_q_dep_ld__blk2003_dn0 = assign89960_e138723_d_n0;
        locals.var_q_dep_ld__blk2003_dn2 = assign89960_e138723_d_n2;
        locals.var_q_dep_ld__blk2003_dn4 = assign89960_e138723_d_n4;
        locals.var_q_dep_ld__blk2003_dn5 = assign89960_e138723_d_n5;
        locals.var_q_dep_ld__blk2003_dn6 = assign89960_e138723_d_n6;
        locals.var_q_dep_ld__blk2003_dn7 = assign89960_e138723_d_n7;
        locals.var_q_dep_ld__blk2003_dn8 = assign89960_e138723_d_n8;
        locals.var_q_dep_ld__blk2003_dn9 = assign89960_e138723_d_n9;
        locals.var_q_dep_ld__blk2003_dn10 = assign89960_e138723_d_n10;
        locals.var_q_dep_ld__blk2003_dn13 = assign89960_e138723_d_n13;
        locals.var_q_dep_ld__blk2003_rv = 0.0;

        let (assign89970_e138735, assign89970_e138735_d_n0, assign89970_e138735_d_n2, assign89970_e138735_d_n4, assign89970_e138735_d_n5, assign89970_e138735_d_n6, assign89970_e138735_d_n7, assign89970_e138735_d_n8, assign89970_e138735_d_n9, assign89970_e138735_d_n10, assign89970_e138735_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) {
        let assign89970_e138729: f64 = (locals.var_q_dep_ld__blk2003 / locals.var_cnst0over_func);
        let assign89970_e138732: f64 = (10.0 * 2.220446049250313e-16);
        let assign89970_e138733: f64 = (assign89970_e138729 + assign89970_e138732);
        (assign89970_e138733, (((locals.var_q_dep_ld__blk2003_dn0 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk2003 * locals.var_cnst0over_func_dn0)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk2003_dn2 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk2003 * locals.var_cnst0over_func_dn2)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk2003_dn4 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk2003 * locals.var_cnst0over_func_dn4)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk2003_dn5 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk2003 * locals.var_cnst0over_func_dn5)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk2003_dn6 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk2003 * locals.var_cnst0over_func_dn6)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk2003_dn7 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk2003 * locals.var_cnst0over_func_dn7)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk2003_dn8 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk2003 * locals.var_cnst0over_func_dn8)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk2003_dn9 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk2003 * locals.var_cnst0over_func_dn9)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk2003_dn10 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk2003 * locals.var_cnst0over_func_dn10)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk2003_dn13 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk2003 * locals.var_cnst0over_func_dn13)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)),)
    } else {
        (locals.var_xi0p12, locals.var_xi0p12_dn0, locals.var_xi0p12_dn2, locals.var_xi0p12_dn4, locals.var_xi0p12_dn5, locals.var_xi0p12_dn6, locals.var_xi0p12_dn7, locals.var_xi0p12_dn8, locals.var_xi0p12_dn9, locals.var_xi0p12_dn10, locals.var_xi0p12_dn13,)
    }
};
        locals.var_xi0p12 = assign89970_e138735;
        locals.var_xi0p12_dn0 = assign89970_e138735_d_n0;
        locals.var_xi0p12_dn2 = assign89970_e138735_d_n2;
        locals.var_xi0p12_dn4 = assign89970_e138735_d_n4;
        locals.var_xi0p12_dn5 = assign89970_e138735_d_n5;
        locals.var_xi0p12_dn6 = assign89970_e138735_d_n6;
        locals.var_xi0p12_dn7 = assign89970_e138735_d_n7;
        locals.var_xi0p12_dn8 = assign89970_e138735_d_n8;
        locals.var_xi0p12_dn9 = assign89970_e138735_d_n9;
        locals.var_xi0p12_dn10 = assign89970_e138735_d_n10;
        locals.var_xi0p12_dn13 = assign89970_e138735_d_n13;
        locals.var_xi0p12_rv = 0.0;

        let (assign89980_e138743, assign89980_e138743_d_n0, assign89980_e138743_d_n2, assign89980_e138743_d_n4, assign89980_e138743_d_n5, assign89980_e138743_d_n6, assign89980_e138743_d_n7, assign89980_e138743_d_n8, assign89980_e138743_d_n9, assign89980_e138743_d_n10, assign89980_e138743_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) {
        let assign89980_e138741: f64 = (locals.var_cnst0over_func * locals.var_xi0p12);
        (assign89980_e138741, ((locals.var_cnst0over_func_dn0 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn0)), ((locals.var_cnst0over_func_dn2 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn2)), ((locals.var_cnst0over_func_dn4 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn4)), ((locals.var_cnst0over_func_dn5 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn5)), ((locals.var_cnst0over_func_dn6 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn6)), ((locals.var_cnst0over_func_dn7 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn7)), ((locals.var_cnst0over_func_dn8 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn8)), ((locals.var_cnst0over_func_dn9 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn9)), ((locals.var_cnst0over_func_dn10 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn10)), ((locals.var_cnst0over_func_dn13 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn13)),)
    } else {
        (locals.var_qbuld, locals.var_qbuld_dn0, locals.var_qbuld_dn2, locals.var_qbuld_dn4, locals.var_qbuld_dn5, locals.var_qbuld_dn6, locals.var_qbuld_dn7, locals.var_qbuld_dn8, locals.var_qbuld_dn9, locals.var_qbuld_dn10, locals.var_qbuld_dn13,)
    }
};
        locals.var_qbuld = assign89980_e138743;
        locals.var_qbuld_dn0 = assign89980_e138743_d_n0;
        locals.var_qbuld_dn2 = assign89980_e138743_d_n2;
        locals.var_qbuld_dn4 = assign89980_e138743_d_n4;
        locals.var_qbuld_dn5 = assign89980_e138743_d_n5;
        locals.var_qbuld_dn6 = assign89980_e138743_d_n6;
        locals.var_qbuld_dn7 = assign89980_e138743_d_n7;
        locals.var_qbuld_dn8 = assign89980_e138743_d_n8;
        locals.var_qbuld_dn9 = assign89980_e138743_d_n9;
        locals.var_qbuld_dn10 = assign89980_e138743_d_n10;
        locals.var_qbuld_dn13 = assign89980_e138743_d_n13;
        locals.var_qbuld_rv = 0.0;

        let (assign89990_e138753, assign89990_e138753_d_n0, assign89990_e138753_d_n2, assign89990_e138753_d_n4, assign89990_e138753_d_n5, assign89990_e138753_d_n6, assign89990_e138753_d_n7, assign89990_e138753_d_n8, assign89990_e138753_d_n9, assign89990_e138753_d_n10, assign89990_e138753_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) {
        let assign89990_e138750: f64 = (locals.var_fs02 + locals.var_xi0p12);
        let assign89990_e138751: f64 = (1.0 / assign89990_e138750);
        (assign89990_e138751, (-((locals.var_fs02_dn0 + locals.var_xi0p12_dn0) / (assign89990_e138750 * assign89990_e138750))), (-((locals.var_fs02_dn2 + locals.var_xi0p12_dn2) / (assign89990_e138750 * assign89990_e138750))), (-((locals.var_fs02_dn4 + locals.var_xi0p12_dn4) / (assign89990_e138750 * assign89990_e138750))), (-((locals.var_fs02_dn5 + locals.var_xi0p12_dn5) / (assign89990_e138750 * assign89990_e138750))), (-((locals.var_fs02_dn6 + locals.var_xi0p12_dn6) / (assign89990_e138750 * assign89990_e138750))), (-((locals.var_fs02_dn7 + locals.var_xi0p12_dn7) / (assign89990_e138750 * assign89990_e138750))), (-((locals.var_fs02_dn8 + locals.var_xi0p12_dn8) / (assign89990_e138750 * assign89990_e138750))), (-((locals.var_fs02_dn9 + locals.var_xi0p12_dn9) / (assign89990_e138750 * assign89990_e138750))), (-((locals.var_fs02_dn10 + locals.var_xi0p12_dn10) / (assign89990_e138750 * assign89990_e138750))), (-((locals.var_fs02_dn13 + locals.var_xi0p12_dn13) / (assign89990_e138750 * assign89990_e138750))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign89990_e138753;
        locals.var_t1_dn0 = assign89990_e138753_d_n0;
        locals.var_t1_dn2 = assign89990_e138753_d_n2;
        locals.var_t1_dn4 = assign89990_e138753_d_n4;
        locals.var_t1_dn5 = assign89990_e138753_d_n5;
        locals.var_t1_dn6 = assign89990_e138753_d_n6;
        locals.var_t1_dn7 = assign89990_e138753_d_n7;
        locals.var_t1_dn8 = assign89990_e138753_d_n8;
        locals.var_t1_dn9 = assign89990_e138753_d_n9;
        locals.var_t1_dn10 = assign89990_e138753_d_n10;
        locals.var_t1_dn13 = assign89990_e138753_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign90000_e138763, assign90000_e138763_d_n0, assign90000_e138763_d_n2, assign90000_e138763_d_n4, assign90000_e138763_d_n5, assign90000_e138763_d_n6, assign90000_e138763_d_n7, assign90000_e138763_d_n8, assign90000_e138763_d_n9, assign90000_e138763_d_n10, assign90000_e138763_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) {
        let assign90000_e138759: f64 = (locals.var_cnst0over_func * locals.var_fs01);
        let assign90000_e138761: f64 = (assign90000_e138759 * locals.var_t1);
        (assign90000_e138761, ((((locals.var_cnst0over_func_dn0 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn0)) * locals.var_t1) + (assign90000_e138759 * locals.var_t1_dn0)), ((((locals.var_cnst0over_func_dn2 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn2)) * locals.var_t1) + (assign90000_e138759 * locals.var_t1_dn2)), ((((locals.var_cnst0over_func_dn4 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn4)) * locals.var_t1) + (assign90000_e138759 * locals.var_t1_dn4)), ((((locals.var_cnst0over_func_dn5 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn5)) * locals.var_t1) + (assign90000_e138759 * locals.var_t1_dn5)), ((((locals.var_cnst0over_func_dn6 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn6)) * locals.var_t1) + (assign90000_e138759 * locals.var_t1_dn6)), ((((locals.var_cnst0over_func_dn7 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn7)) * locals.var_t1) + (assign90000_e138759 * locals.var_t1_dn7)), ((((locals.var_cnst0over_func_dn8 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn8)) * locals.var_t1) + (assign90000_e138759 * locals.var_t1_dn8)), ((((locals.var_cnst0over_func_dn9 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn9)) * locals.var_t1) + (assign90000_e138759 * locals.var_t1_dn9)), ((((locals.var_cnst0over_func_dn10 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn10)) * locals.var_t1) + (assign90000_e138759 * locals.var_t1_dn10)), ((((locals.var_cnst0over_func_dn13 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn13)) * locals.var_t1) + (assign90000_e138759 * locals.var_t1_dn13)),)
    } else {
        (locals.var_qiuld, locals.var_qiuld_dn0, locals.var_qiuld_dn2, locals.var_qiuld_dn4, locals.var_qiuld_dn5, locals.var_qiuld_dn6, locals.var_qiuld_dn7, locals.var_qiuld_dn8, locals.var_qiuld_dn9, locals.var_qiuld_dn10, locals.var_qiuld_dn13,)
    }
};
        locals.var_qiuld = assign90000_e138763;
        locals.var_qiuld_dn0 = assign90000_e138763_d_n0;
        locals.var_qiuld_dn2 = assign90000_e138763_d_n2;
        locals.var_qiuld_dn4 = assign90000_e138763_d_n4;
        locals.var_qiuld_dn5 = assign90000_e138763_d_n5;
        locals.var_qiuld_dn6 = assign90000_e138763_d_n6;
        locals.var_qiuld_dn7 = assign90000_e138763_d_n7;
        locals.var_qiuld_dn8 = assign90000_e138763_d_n8;
        locals.var_qiuld_dn9 = assign90000_e138763_d_n9;
        locals.var_qiuld_dn10 = assign90000_e138763_d_n10;
        locals.var_qiuld_dn13 = assign90000_e138763_d_n13;
        locals.var_qiuld_rv = 0.0;

        let (assign90010_e138771, assign90010_e138771_d_n0, assign90010_e138771_d_n2, assign90010_e138771_d_n4, assign90010_e138771_d_n5, assign90010_e138771_d_n6, assign90010_e138771_d_n7, assign90010_e138771_d_n8, assign90010_e138771_d_n9, assign90010_e138771_d_n10, assign90010_e138771_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) {
        let assign90010_e138769: f64 = (locals.var_qbuld + locals.var_qiuld);
        (assign90010_e138769, (locals.var_qbuld_dn0 + locals.var_qiuld_dn0), (locals.var_qbuld_dn2 + locals.var_qiuld_dn2), (locals.var_qbuld_dn4 + locals.var_qiuld_dn4), (locals.var_qbuld_dn5 + locals.var_qiuld_dn5), (locals.var_qbuld_dn6 + locals.var_qiuld_dn6), (locals.var_qbuld_dn7 + locals.var_qiuld_dn7), (locals.var_qbuld_dn8 + locals.var_qiuld_dn8), (locals.var_qbuld_dn9 + locals.var_qiuld_dn9), (locals.var_qbuld_dn10 + locals.var_qiuld_dn10), (locals.var_qbuld_dn13 + locals.var_qiuld_dn13),)
    } else {
        (locals.var_qsuld, locals.var_qsuld_dn0, locals.var_qsuld_dn2, locals.var_qsuld_dn4, locals.var_qsuld_dn5, locals.var_qsuld_dn6, locals.var_qsuld_dn7, locals.var_qsuld_dn8, locals.var_qsuld_dn9, locals.var_qsuld_dn10, locals.var_qsuld_dn13,)
    }
};
        locals.var_qsuld = assign90010_e138771;
        locals.var_qsuld_dn0 = assign90010_e138771_d_n0;
        locals.var_qsuld_dn2 = assign90010_e138771_d_n2;
        locals.var_qsuld_dn4 = assign90010_e138771_d_n4;
        locals.var_qsuld_dn5 = assign90010_e138771_d_n5;
        locals.var_qsuld_dn6 = assign90010_e138771_d_n6;
        locals.var_qsuld_dn7 = assign90010_e138771_d_n7;
        locals.var_qsuld_dn8 = assign90010_e138771_d_n8;
        locals.var_qsuld_dn9 = assign90010_e138771_d_n9;
        locals.var_qsuld_dn10 = assign90010_e138771_d_n10;
        locals.var_qsuld_dn13 = assign90010_e138771_d_n13;
        locals.var_qsuld_rv = 0.0;

        let (assign90020_e138777, assign90020_e138777_d_n0, assign90020_e138777_d_n2, assign90020_e138777_d_n4, assign90020_e138777_d_n5, assign90020_e138777_d_n6, assign90020_e138777_d_n7, assign90020_e138777_d_n8, assign90020_e138777_d_n9, assign90020_e138777_d_n10, assign90020_e138777_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign90020_e138775: f64 = (locals.var_qsuld - locals.var_qbuld);
        (assign90020_e138775, (locals.var_qsuld_dn0 - locals.var_qbuld_dn0), (locals.var_qsuld_dn2 - locals.var_qbuld_dn2), (locals.var_qsuld_dn4 - locals.var_qbuld_dn4), (locals.var_qsuld_dn5 - locals.var_qbuld_dn5), (locals.var_qsuld_dn6 - locals.var_qbuld_dn6), (locals.var_qsuld_dn7 - locals.var_qbuld_dn7), (locals.var_qsuld_dn8 - locals.var_qbuld_dn8), (locals.var_qsuld_dn9 - locals.var_qbuld_dn9), (locals.var_qsuld_dn10 - locals.var_qbuld_dn10), (locals.var_qsuld_dn13 - locals.var_qbuld_dn13),)
    } else {
        (locals.var_qiuld, locals.var_qiuld_dn0, locals.var_qiuld_dn2, locals.var_qiuld_dn4, locals.var_qiuld_dn5, locals.var_qiuld_dn6, locals.var_qiuld_dn7, locals.var_qiuld_dn8, locals.var_qiuld_dn9, locals.var_qiuld_dn10, locals.var_qiuld_dn13,)
    }
};
        locals.var_qiuld = assign90020_e138777;
        locals.var_qiuld_dn0 = assign90020_e138777_d_n0;
        locals.var_qiuld_dn2 = assign90020_e138777_d_n2;
        locals.var_qiuld_dn4 = assign90020_e138777_d_n4;
        locals.var_qiuld_dn5 = assign90020_e138777_d_n5;
        locals.var_qiuld_dn6 = assign90020_e138777_d_n6;
        locals.var_qiuld_dn7 = assign90020_e138777_d_n7;
        locals.var_qiuld_dn8 = assign90020_e138777_d_n8;
        locals.var_qiuld_dn9 = assign90020_e138777_d_n9;
        locals.var_qiuld_dn10 = assign90020_e138777_d_n10;
        locals.var_qiuld_dn13 = assign90020_e138777_d_n13;
        locals.var_qiuld_rv = 0.0;

        let assign90030_e138780: f64 = if locals.var_lover_func < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2100 = assign90030_e138780;
        locals.var_guard2100_rv = 0.0;

        let (assign90040_e138787, assign90040_e138787_d_n0, assign90040_e138787_d_n2, assign90040_e138787_d_n4, assign90040_e138787_d_n5, assign90040_e138787_d_n6, assign90040_e138787_d_n7, assign90040_e138787_d_n8, assign90040_e138787_d_n9, assign90040_e138787_d_n10, assign90040_e138787_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2100 != 0.0)) {
        let assign90040_e138785: f64 = (-locals.var_lover_func);
        (assign90040_e138785, (-locals.var_lover_func_dn0), (-locals.var_lover_func_dn2), (-locals.var_lover_func_dn4), (-locals.var_lover_func_dn5), (-locals.var_lover_func_dn6), (-locals.var_lover_func_dn7), (-locals.var_lover_func_dn8), (-locals.var_lover_func_dn9), (-locals.var_lover_func_dn10), (-locals.var_lover_func_dn13),)
    } else {
        (locals.var_lover_func, locals.var_lover_func_dn0, locals.var_lover_func_dn2, locals.var_lover_func_dn4, locals.var_lover_func_dn5, locals.var_lover_func_dn6, locals.var_lover_func_dn7, locals.var_lover_func_dn8, locals.var_lover_func_dn9, locals.var_lover_func_dn10, locals.var_lover_func_dn13,)
    }
};
        locals.var_lover_func = assign90040_e138787;
        locals.var_lover_func_dn0 = assign90040_e138787_d_n0;
        locals.var_lover_func_dn2 = assign90040_e138787_d_n2;
        locals.var_lover_func_dn4 = assign90040_e138787_d_n4;
        locals.var_lover_func_dn5 = assign90040_e138787_d_n5;
        locals.var_lover_func_dn6 = assign90040_e138787_d_n6;
        locals.var_lover_func_dn7 = assign90040_e138787_d_n7;
        locals.var_lover_func_dn8 = assign90040_e138787_d_n8;
        locals.var_lover_func_dn9 = assign90040_e138787_d_n9;
        locals.var_lover_func_dn10 = assign90040_e138787_d_n10;
        locals.var_lover_func_dn13 = assign90040_e138787_d_n13;
        locals.var_lover_func_rv = 0.0;

        let assign90050_e138790: f64 = if p.p55 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2101 = assign90050_e138790;
        locals.var_guard2101_rv = 0.0;

        let assign90060_e138793: f64 = if p.p50 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2102 = assign90060_e138793;
        locals.var_guard2102_rv = 0.0;

        let (assign90070_e138804, assign90070_e138804_d_n0, assign90070_e138804_d_n2, assign90070_e138804_d_n4, assign90070_e138804_d_n5, assign90070_e138804_d_n6, assign90070_e138804_d_n7, assign90070_e138804_d_n8, assign90070_e138804_d_n9, assign90070_e138804_d_n10, assign90070_e138804_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2100 != 0.0)) && (locals.var_guard2101 != 0.0)) && (locals.var_guard2102 != 0.0)) {
        let assign90070_e138802: f64 = (-locals.var_ps0ld);
        (assign90070_e138802, (-locals.var_ps0ld_dn0), (-locals.var_ps0ld_dn2), (-locals.var_ps0ld_dn4), (-locals.var_ps0ld_dn5), (-locals.var_ps0ld_dn6), (-locals.var_ps0ld_dn7), (-locals.var_ps0ld_dn8), (-locals.var_ps0ld_dn9), (-locals.var_ps0ld_dn10), (-locals.var_ps0ld_dn13),)
    } else {
        (locals.var_vx__blk2005, locals.var_vx__blk2005_dn0, locals.var_vx__blk2005_dn2, locals.var_vx__blk2005_dn4, locals.var_vx__blk2005_dn5, locals.var_vx__blk2005_dn6, locals.var_vx__blk2005_dn7, locals.var_vx__blk2005_dn8, locals.var_vx__blk2005_dn9, locals.var_vx__blk2005_dn10, locals.var_vx__blk2005_dn13,)
    }
};
        locals.var_vx__blk2005 = assign90070_e138804;
        locals.var_vx__blk2005_dn0 = assign90070_e138804_d_n0;
        locals.var_vx__blk2005_dn2 = assign90070_e138804_d_n2;
        locals.var_vx__blk2005_dn4 = assign90070_e138804_d_n4;
        locals.var_vx__blk2005_dn5 = assign90070_e138804_d_n5;
        locals.var_vx__blk2005_dn6 = assign90070_e138804_d_n6;
        locals.var_vx__blk2005_dn7 = assign90070_e138804_d_n7;
        locals.var_vx__blk2005_dn8 = assign90070_e138804_d_n8;
        locals.var_vx__blk2005_dn9 = assign90070_e138804_d_n9;
        locals.var_vx__blk2005_dn10 = assign90070_e138804_d_n10;
        locals.var_vx__blk2005_dn13 = assign90070_e138804_d_n13;
        locals.var_vx__blk2005_rv = 0.0;

        let (assign90080_e138815, assign90080_e138815_d_n0, assign90080_e138815_d_n2, assign90080_e138815_d_n4, assign90080_e138815_d_n5, assign90080_e138815_d_n6, assign90080_e138815_d_n7, assign90080_e138815_d_n8, assign90080_e138815_d_n9, assign90080_e138815_d_n10, assign90080_e138815_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2100 != 0.0)) && (locals.var_guard2101 != 0.0)) && (locals.var_guard2102 == 0.0)) {
        (locals.var_vxbgmt, locals.var_vxbgmt_dn0, locals.var_vxbgmt_dn2, locals.var_vxbgmt_dn4, locals.var_vxbgmt_dn5, locals.var_vxbgmt_dn6, locals.var_vxbgmt_dn7, locals.var_vxbgmt_dn8, locals.var_vxbgmt_dn9, locals.var_vxbgmt_dn10, locals.var_vxbgmt_dn13,)
    } else {
        (locals.var_vx__blk2005, locals.var_vx__blk2005_dn0, locals.var_vx__blk2005_dn2, locals.var_vx__blk2005_dn4, locals.var_vx__blk2005_dn5, locals.var_vx__blk2005_dn6, locals.var_vx__blk2005_dn7, locals.var_vx__blk2005_dn8, locals.var_vx__blk2005_dn9, locals.var_vx__blk2005_dn10, locals.var_vx__blk2005_dn13,)
    }
};
        locals.var_vx__blk2005 = assign90080_e138815;
        locals.var_vx__blk2005_dn0 = assign90080_e138815_d_n0;
        locals.var_vx__blk2005_dn2 = assign90080_e138815_d_n2;
        locals.var_vx__blk2005_dn4 = assign90080_e138815_d_n4;
        locals.var_vx__blk2005_dn5 = assign90080_e138815_d_n5;
        locals.var_vx__blk2005_dn6 = assign90080_e138815_d_n6;
        locals.var_vx__blk2005_dn7 = assign90080_e138815_d_n7;
        locals.var_vx__blk2005_dn8 = assign90080_e138815_d_n8;
        locals.var_vx__blk2005_dn9 = assign90080_e138815_d_n9;
        locals.var_vx__blk2005_dn10 = assign90080_e138815_d_n10;
        locals.var_vx__blk2005_dn13 = assign90080_e138815_d_n13;
        locals.var_vx__blk2005_rv = 0.0;

        let (assign90090_e138836, assign90090_e138836_d_n0, assign90090_e138836_d_n2, assign90090_e138836_d_n4, assign90090_e138836_d_n5, assign90090_e138836_d_n6, assign90090_e138836_d_n7, assign90090_e138836_d_n8, assign90090_e138836_d_n9, assign90090_e138836_d_n10, assign90090_e138836_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2100 != 0.0)) && (locals.var_guard2101 != 0.0)) {
        let assign90090_e138823: f64 = (locals.var_vx__blk2005 + p.p137);
        let assign90090_e138826: f64 = (locals.var_vx__blk2005 + p.p137);
        let assign90090_e138827: f64 = (assign90090_e138823 * assign90090_e138826);
        let assign90090_e138830: f64 = (4.0 * 0.1);
        let assign90090_e138832: f64 = (assign90090_e138830 * 0.1);
        let assign90090_e138833: f64 = (assign90090_e138827 + assign90090_e138832);
        let assign90090_e138834: f64 = (assign90090_e138833).sqrt();
        (assign90090_e138834, (((locals.var_vx__blk2005_dn0 * assign90090_e138826) + (assign90090_e138823 * locals.var_vx__blk2005_dn0)) / (2.0 * assign90090_e138834)), (((locals.var_vx__blk2005_dn2 * assign90090_e138826) + (assign90090_e138823 * locals.var_vx__blk2005_dn2)) / (2.0 * assign90090_e138834)), (((locals.var_vx__blk2005_dn4 * assign90090_e138826) + (assign90090_e138823 * locals.var_vx__blk2005_dn4)) / (2.0 * assign90090_e138834)), (((locals.var_vx__blk2005_dn5 * assign90090_e138826) + (assign90090_e138823 * locals.var_vx__blk2005_dn5)) / (2.0 * assign90090_e138834)), (((locals.var_vx__blk2005_dn6 * assign90090_e138826) + (assign90090_e138823 * locals.var_vx__blk2005_dn6)) / (2.0 * assign90090_e138834)), (((locals.var_vx__blk2005_dn7 * assign90090_e138826) + (assign90090_e138823 * locals.var_vx__blk2005_dn7)) / (2.0 * assign90090_e138834)), (((locals.var_vx__blk2005_dn8 * assign90090_e138826) + (assign90090_e138823 * locals.var_vx__blk2005_dn8)) / (2.0 * assign90090_e138834)), (((locals.var_vx__blk2005_dn9 * assign90090_e138826) + (assign90090_e138823 * locals.var_vx__blk2005_dn9)) / (2.0 * assign90090_e138834)), (((locals.var_vx__blk2005_dn10 * assign90090_e138826) + (assign90090_e138823 * locals.var_vx__blk2005_dn10)) / (2.0 * assign90090_e138834)), (((locals.var_vx__blk2005_dn13 * assign90090_e138826) + (assign90090_e138823 * locals.var_vx__blk2005_dn13)) / (2.0 * assign90090_e138834)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign90090_e138836;
        locals.var_tmf2_dn0 = assign90090_e138836_d_n0;
        locals.var_tmf2_dn2 = assign90090_e138836_d_n2;
        locals.var_tmf2_dn4 = assign90090_e138836_d_n4;
        locals.var_tmf2_dn5 = assign90090_e138836_d_n5;
        locals.var_tmf2_dn6 = assign90090_e138836_d_n6;
        locals.var_tmf2_dn7 = assign90090_e138836_d_n7;
        locals.var_tmf2_dn8 = assign90090_e138836_d_n8;
        locals.var_tmf2_dn9 = assign90090_e138836_d_n9;
        locals.var_tmf2_dn10 = assign90090_e138836_d_n10;
        locals.var_tmf2_dn13 = assign90090_e138836_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign90100_e138852, assign90100_e138852_d_n0, assign90100_e138852_d_n2, assign90100_e138852_d_n4, assign90100_e138852_d_n5, assign90100_e138852_d_n6, assign90100_e138852_d_n7, assign90100_e138852_d_n8, assign90100_e138852_d_n9, assign90100_e138852_d_n10, assign90100_e138852_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2100 != 0.0)) && (locals.var_guard2101 != 0.0)) {
        let assign90100_e138846: f64 = (locals.var_vx__blk2005 + p.p137);
        let assign90100_e138848: f64 = (assign90100_e138846 / locals.var_tmf2);
        let assign90100_e138849: f64 = (1.0 + assign90100_e138848);
        let assign90100_e138850: f64 = (0.5 * assign90100_e138849);
        (assign90100_e138850, (0.5 * (((locals.var_vx__blk2005_dn0 * locals.var_tmf2) - (assign90100_e138846 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vx__blk2005_dn2 * locals.var_tmf2) - (assign90100_e138846 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vx__blk2005_dn4 * locals.var_tmf2) - (assign90100_e138846 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vx__blk2005_dn5 * locals.var_tmf2) - (assign90100_e138846 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vx__blk2005_dn6 * locals.var_tmf2) - (assign90100_e138846 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vx__blk2005_dn7 * locals.var_tmf2) - (assign90100_e138846 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vx__blk2005_dn8 * locals.var_tmf2) - (assign90100_e138846 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vx__blk2005_dn9 * locals.var_tmf2) - (assign90100_e138846 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vx__blk2005_dn10 * locals.var_tmf2) - (assign90100_e138846 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vx__blk2005_dn13 * locals.var_tmf2) - (assign90100_e138846 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign90100_e138852;
        locals.var_t9_dn0 = assign90100_e138852_d_n0;
        locals.var_t9_dn2 = assign90100_e138852_d_n2;
        locals.var_t9_dn4 = assign90100_e138852_d_n4;
        locals.var_t9_dn5 = assign90100_e138852_d_n5;
        locals.var_t9_dn6 = assign90100_e138852_d_n6;
        locals.var_t9_dn7 = assign90100_e138852_d_n7;
        locals.var_t9_dn8 = assign90100_e138852_d_n8;
        locals.var_t9_dn9 = assign90100_e138852_d_n9;
        locals.var_t9_dn10 = assign90100_e138852_d_n10;
        locals.var_t9_dn13 = assign90100_e138852_d_n13;
        locals.var_t9_rv = 0.0;

        let (assign90110_e138866, assign90110_e138866_d_n0, assign90110_e138866_d_n2, assign90110_e138866_d_n4, assign90110_e138866_d_n5, assign90110_e138866_d_n6, assign90110_e138866_d_n7, assign90110_e138866_d_n8, assign90110_e138866_d_n9, assign90110_e138866_d_n10, assign90110_e138866_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2100 != 0.0)) && (locals.var_guard2101 != 0.0)) {
        let assign90110_e138861: f64 = (locals.var_vx__blk2005 + p.p137);
        let assign90110_e138863: f64 = (assign90110_e138861 + locals.var_tmf2);
        let assign90110_e138864: f64 = (0.5 * assign90110_e138863);
        (assign90110_e138864, (0.5 * (locals.var_vx__blk2005_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_vx__blk2005_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_vx__blk2005_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_vx__blk2005_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_vx__blk2005_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_vx__blk2005_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_vx__blk2005_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_vx__blk2005_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_vx__blk2005_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_vx__blk2005_dn13 + locals.var_tmf2_dn13)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign90110_e138866;
        locals.var_t2_dn0 = assign90110_e138866_d_n0;
        locals.var_t2_dn2 = assign90110_e138866_d_n2;
        locals.var_t2_dn4 = assign90110_e138866_d_n4;
        locals.var_t2_dn5 = assign90110_e138866_d_n5;
        locals.var_t2_dn6 = assign90110_e138866_d_n6;
        locals.var_t2_dn7 = assign90110_e138866_d_n7;
        locals.var_t2_dn8 = assign90110_e138866_d_n8;
        locals.var_t2_dn9 = assign90110_e138866_d_n9;
        locals.var_t2_dn10 = assign90110_e138866_d_n10;
        locals.var_t2_dn13 = assign90110_e138866_d_n13;
        locals.var_t2_rv = 0.0;

        let assign90120_e138869: f64 = if locals.var_t2 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2103 = assign90120_e138869;
        locals.var_guard2103_rv = 0.0;

        let (assign90130_e138879, assign90130_e138879_d_n0, assign90130_e138879_d_n2, assign90130_e138879_d_n4, assign90130_e138879_d_n5, assign90130_e138879_d_n6, assign90130_e138879_d_n7, assign90130_e138879_d_n8, assign90130_e138879_d_n9, assign90130_e138879_d_n10, assign90130_e138879_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2100 != 0.0)) && (locals.var_guard2101 != 0.0)) && (locals.var_guard2103 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign90130_e138879;
        locals.var_t2_dn0 = assign90130_e138879_d_n0;
        locals.var_t2_dn2 = assign90130_e138879_d_n2;
        locals.var_t2_dn4 = assign90130_e138879_d_n4;
        locals.var_t2_dn5 = assign90130_e138879_d_n5;
        locals.var_t2_dn6 = assign90130_e138879_d_n6;
        locals.var_t2_dn7 = assign90130_e138879_d_n7;
        locals.var_t2_dn8 = assign90130_e138879_d_n8;
        locals.var_t2_dn9 = assign90130_e138879_d_n9;
        locals.var_t2_dn10 = assign90130_e138879_d_n10;
        locals.var_t2_dn13 = assign90130_e138879_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign90140_e138889, assign90140_e138889_d_n0, assign90140_e138889_d_n2, assign90140_e138889_d_n4, assign90140_e138889_d_n5, assign90140_e138889_d_n6, assign90140_e138889_d_n7, assign90140_e138889_d_n8, assign90140_e138889_d_n9, assign90140_e138889_d_n10, assign90140_e138889_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2100 != 0.0)) && (locals.var_guard2101 != 0.0)) && (locals.var_guard2103 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign90140_e138889;
        locals.var_t9_dn0 = assign90140_e138889_d_n0;
        locals.var_t9_dn2 = assign90140_e138889_d_n2;
        locals.var_t9_dn4 = assign90140_e138889_d_n4;
        locals.var_t9_dn5 = assign90140_e138889_d_n5;
        locals.var_t9_dn6 = assign90140_e138889_d_n6;
        locals.var_t9_dn7 = assign90140_e138889_d_n7;
        locals.var_t9_dn8 = assign90140_e138889_d_n8;
        locals.var_t9_dn9 = assign90140_e138889_d_n9;
        locals.var_t9_dn10 = assign90140_e138889_d_n10;
        locals.var_t9_dn13 = assign90140_e138889_d_n13;
        locals.var_t9_rv = 0.0;

        let (assign90150_e138902, assign90150_e138902_d_n0, assign90150_e138902_d_n2, assign90150_e138902_d_n4, assign90150_e138902_d_n5, assign90150_e138902_d_n6, assign90150_e138902_d_n7, assign90150_e138902_d_n8, assign90150_e138902_d_n9, assign90150_e138902_d_n10, assign90150_e138902_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2100 != 0.0)) && (locals.var_guard2101 != 0.0)) {
        let assign90150_e138897: f64 = (locals.var_kjunc * locals.var_t2);
        let assign90150_e138898: f64 = (assign90150_e138897).sqrt();
        let assign90150_e138900: f64 = (assign90150_e138898 * p.p432);
        (assign90150_e138900, ((((locals.var_kjunc_dn0 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn0)) / (2.0 * assign90150_e138898)) * p.p432), ((((locals.var_kjunc_dn2 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn2)) / (2.0 * assign90150_e138898)) * p.p432), ((((locals.var_kjunc_dn4 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn4)) / (2.0 * assign90150_e138898)) * p.p432), ((((locals.var_kjunc_dn5 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn5)) / (2.0 * assign90150_e138898)) * p.p432), ((((locals.var_kjunc_dn6 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn6)) / (2.0 * assign90150_e138898)) * p.p432), ((((locals.var_kjunc_dn7 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn7)) / (2.0 * assign90150_e138898)) * p.p432), ((((locals.var_kjunc_dn8 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn8)) / (2.0 * assign90150_e138898)) * p.p432), ((((locals.var_kjunc_dn9 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn9)) / (2.0 * assign90150_e138898)) * p.p432), ((((locals.var_kjunc_dn10 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn10)) / (2.0 * assign90150_e138898)) * p.p432), ((((locals.var_kjunc_dn13 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn13)) / (2.0 * assign90150_e138898)) * p.p432),)
    } else {
        (locals.var_wjunc0, locals.var_wjunc0_dn0, locals.var_wjunc0_dn2, locals.var_wjunc0_dn4, locals.var_wjunc0_dn5, locals.var_wjunc0_dn6, locals.var_wjunc0_dn7, locals.var_wjunc0_dn8, locals.var_wjunc0_dn9, locals.var_wjunc0_dn10, locals.var_wjunc0_dn13,)
    }
};
        locals.var_wjunc0 = assign90150_e138902;
        locals.var_wjunc0_dn0 = assign90150_e138902_d_n0;
        locals.var_wjunc0_dn2 = assign90150_e138902_d_n2;
        locals.var_wjunc0_dn4 = assign90150_e138902_d_n4;
        locals.var_wjunc0_dn5 = assign90150_e138902_d_n5;
        locals.var_wjunc0_dn6 = assign90150_e138902_d_n6;
        locals.var_wjunc0_dn7 = assign90150_e138902_d_n7;
        locals.var_wjunc0_dn8 = assign90150_e138902_d_n8;
        locals.var_wjunc0_dn9 = assign90150_e138902_d_n9;
        locals.var_wjunc0_dn10 = assign90150_e138902_d_n10;
        locals.var_wjunc0_dn13 = assign90150_e138902_d_n13;
        locals.var_wjunc0_rv = 0.0;

        let (assign90160_e138916, assign90160_e138916_d_n0, assign90160_e138916_d_n2, assign90160_e138916_d_n4, assign90160_e138916_d_n5, assign90160_e138916_d_n6, assign90160_e138916_d_n7, assign90160_e138916_d_n8, assign90160_e138916_d_n9, assign90160_e138916_d_n10, assign90160_e138916_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2100 != 0.0)) && (locals.var_guard2101 != 0.0)) {
        let assign90160_e138910: f64 = (locals.var_lover_func - locals.var_wjunc0);
        let assign90160_e138913: f64 = (0.1 * locals.var_lover_func);
        let assign90160_e138914: f64 = (assign90160_e138910 - assign90160_e138913);
        (assign90160_e138914, ((locals.var_lover_func_dn0 - locals.var_wjunc0_dn0) - (0.1 * locals.var_lover_func_dn0)), ((locals.var_lover_func_dn2 - locals.var_wjunc0_dn2) - (0.1 * locals.var_lover_func_dn2)), ((locals.var_lover_func_dn4 - locals.var_wjunc0_dn4) - (0.1 * locals.var_lover_func_dn4)), ((locals.var_lover_func_dn5 - locals.var_wjunc0_dn5) - (0.1 * locals.var_lover_func_dn5)), ((locals.var_lover_func_dn6 - locals.var_wjunc0_dn6) - (0.1 * locals.var_lover_func_dn6)), ((locals.var_lover_func_dn7 - locals.var_wjunc0_dn7) - (0.1 * locals.var_lover_func_dn7)), ((locals.var_lover_func_dn8 - locals.var_wjunc0_dn8) - (0.1 * locals.var_lover_func_dn8)), ((locals.var_lover_func_dn9 - locals.var_wjunc0_dn9) - (0.1 * locals.var_lover_func_dn9)), ((locals.var_lover_func_dn10 - locals.var_wjunc0_dn10) - (0.1 * locals.var_lover_func_dn10)), ((locals.var_lover_func_dn13 - locals.var_wjunc0_dn13) - (0.1 * locals.var_lover_func_dn13)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign90160_e138916;
        locals.var_tmf1_dn0 = assign90160_e138916_d_n0;
        locals.var_tmf1_dn2 = assign90160_e138916_d_n2;
        locals.var_tmf1_dn4 = assign90160_e138916_d_n4;
        locals.var_tmf1_dn5 = assign90160_e138916_d_n5;
        locals.var_tmf1_dn6 = assign90160_e138916_d_n6;
        locals.var_tmf1_dn7 = assign90160_e138916_d_n7;
        locals.var_tmf1_dn8 = assign90160_e138916_d_n8;
        locals.var_tmf1_dn9 = assign90160_e138916_d_n9;
        locals.var_tmf1_dn10 = assign90160_e138916_d_n10;
        locals.var_tmf1_dn13 = assign90160_e138916_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign90170_e138930, assign90170_e138930_d_n0, assign90170_e138930_d_n2, assign90170_e138930_d_n4, assign90170_e138930_d_n5, assign90170_e138930_d_n6, assign90170_e138930_d_n7, assign90170_e138930_d_n8, assign90170_e138930_d_n9, assign90170_e138930_d_n10, assign90170_e138930_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2100 != 0.0)) && (locals.var_guard2101 != 0.0)) {
        let assign90170_e138924: f64 = (4.0 * locals.var_lover_func);
        let assign90170_e138927: f64 = (0.1 * locals.var_lover_func);
        let assign90170_e138928: f64 = (assign90170_e138924 * assign90170_e138927);
        (assign90170_e138928, (((4.0 * locals.var_lover_func_dn0) * assign90170_e138927) + (assign90170_e138924 * (0.1 * locals.var_lover_func_dn0))), (((4.0 * locals.var_lover_func_dn2) * assign90170_e138927) + (assign90170_e138924 * (0.1 * locals.var_lover_func_dn2))), (((4.0 * locals.var_lover_func_dn4) * assign90170_e138927) + (assign90170_e138924 * (0.1 * locals.var_lover_func_dn4))), (((4.0 * locals.var_lover_func_dn5) * assign90170_e138927) + (assign90170_e138924 * (0.1 * locals.var_lover_func_dn5))), (((4.0 * locals.var_lover_func_dn6) * assign90170_e138927) + (assign90170_e138924 * (0.1 * locals.var_lover_func_dn6))), (((4.0 * locals.var_lover_func_dn7) * assign90170_e138927) + (assign90170_e138924 * (0.1 * locals.var_lover_func_dn7))), (((4.0 * locals.var_lover_func_dn8) * assign90170_e138927) + (assign90170_e138924 * (0.1 * locals.var_lover_func_dn8))), (((4.0 * locals.var_lover_func_dn9) * assign90170_e138927) + (assign90170_e138924 * (0.1 * locals.var_lover_func_dn9))), (((4.0 * locals.var_lover_func_dn10) * assign90170_e138927) + (assign90170_e138924 * (0.1 * locals.var_lover_func_dn10))), (((4.0 * locals.var_lover_func_dn13) * assign90170_e138927) + (assign90170_e138924 * (0.1 * locals.var_lover_func_dn13))),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign90170_e138930;
        locals.var_tmf2_dn0 = assign90170_e138930_d_n0;
        locals.var_tmf2_dn2 = assign90170_e138930_d_n2;
        locals.var_tmf2_dn4 = assign90170_e138930_d_n4;
        locals.var_tmf2_dn5 = assign90170_e138930_d_n5;
        locals.var_tmf2_dn6 = assign90170_e138930_d_n6;
        locals.var_tmf2_dn7 = assign90170_e138930_d_n7;
        locals.var_tmf2_dn8 = assign90170_e138930_d_n8;
        locals.var_tmf2_dn9 = assign90170_e138930_d_n9;
        locals.var_tmf2_dn10 = assign90170_e138930_d_n10;
        locals.var_tmf2_dn13 = assign90170_e138930_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign90180_e138944, assign90180_e138944_d_n0, assign90180_e138944_d_n2, assign90180_e138944_d_n4, assign90180_e138944_d_n5, assign90180_e138944_d_n6, assign90180_e138944_d_n7, assign90180_e138944_d_n8, assign90180_e138944_d_n9, assign90180_e138944_d_n10, assign90180_e138944_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2100 != 0.0)) && (locals.var_guard2101 != 0.0)) {
        let (assign90180_e138942, assign90180_e138942_d_n0, assign90180_e138942_d_n2, assign90180_e138942_d_n4, assign90180_e138942_d_n5, assign90180_e138942_d_n6, assign90180_e138942_d_n7, assign90180_e138942_d_n8, assign90180_e138942_d_n9, assign90180_e138942_d_n10, assign90180_e138942_d_n13,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
            } else {
                let assign90180_e138941: f64 = (-locals.var_tmf2);
                (assign90180_e138941, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn13),)
            }
        };
        (assign90180_e138942, assign90180_e138942_d_n0, assign90180_e138942_d_n2, assign90180_e138942_d_n4, assign90180_e138942_d_n5, assign90180_e138942_d_n6, assign90180_e138942_d_n7, assign90180_e138942_d_n8, assign90180_e138942_d_n9, assign90180_e138942_d_n10, assign90180_e138942_d_n13,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign90180_e138944;
        locals.var_tmf2_dn0 = assign90180_e138944_d_n0;
        locals.var_tmf2_dn2 = assign90180_e138944_d_n2;
        locals.var_tmf2_dn4 = assign90180_e138944_d_n4;
        locals.var_tmf2_dn5 = assign90180_e138944_d_n5;
        locals.var_tmf2_dn6 = assign90180_e138944_d_n6;
        locals.var_tmf2_dn7 = assign90180_e138944_d_n7;
        locals.var_tmf2_dn8 = assign90180_e138944_d_n8;
        locals.var_tmf2_dn9 = assign90180_e138944_d_n9;
        locals.var_tmf2_dn10 = assign90180_e138944_d_n10;
        locals.var_tmf2_dn13 = assign90180_e138944_d_n13;
        locals.var_tmf2_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_337(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign90190_e138957, assign90190_e138957_d_n0, assign90190_e138957_d_n2, assign90190_e138957_d_n4, assign90190_e138957_d_n5, assign90190_e138957_d_n6, assign90190_e138957_d_n7, assign90190_e138957_d_n8, assign90190_e138957_d_n9, assign90190_e138957_d_n10, assign90190_e138957_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2100 != 0.0)) && (locals.var_guard2101 != 0.0)) {
        let assign90190_e138952: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign90190_e138954: f64 = (assign90190_e138952 + locals.var_tmf2);
        let assign90190_e138955: f64 = (assign90190_e138954).sqrt();
        (assign90190_e138955, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign90190_e138955)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign90190_e138955)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign90190_e138955)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign90190_e138955)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign90190_e138955)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign90190_e138955)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign90190_e138955)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign90190_e138955)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign90190_e138955)), ((((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)) + locals.var_tmf2_dn13) / (2.0 * assign90190_e138955)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign90190_e138957;
        locals.var_tmf2_dn0 = assign90190_e138957_d_n0;
        locals.var_tmf2_dn2 = assign90190_e138957_d_n2;
        locals.var_tmf2_dn4 = assign90190_e138957_d_n4;
        locals.var_tmf2_dn5 = assign90190_e138957_d_n5;
        locals.var_tmf2_dn6 = assign90190_e138957_d_n6;
        locals.var_tmf2_dn7 = assign90190_e138957_d_n7;
        locals.var_tmf2_dn8 = assign90190_e138957_d_n8;
        locals.var_tmf2_dn9 = assign90190_e138957_d_n9;
        locals.var_tmf2_dn10 = assign90190_e138957_d_n10;
        locals.var_tmf2_dn13 = assign90190_e138957_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign90200_e138971, assign90200_e138971_d_n0, assign90200_e138971_d_n2, assign90200_e138971_d_n4, assign90200_e138971_d_n5, assign90200_e138971_d_n6, assign90200_e138971_d_n7, assign90200_e138971_d_n8, assign90200_e138971_d_n9, assign90200_e138971_d_n10, assign90200_e138971_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2100 != 0.0)) && (locals.var_guard2101 != 0.0)) {
        let assign90200_e138967: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign90200_e138968: f64 = (1.0 + assign90200_e138967);
        let assign90200_e138969: f64 = (0.5 * assign90200_e138968);
        (assign90200_e138969, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn13 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign90200_e138971;
        locals.var_t0_dn0 = assign90200_e138971_d_n0;
        locals.var_t0_dn2 = assign90200_e138971_d_n2;
        locals.var_t0_dn4 = assign90200_e138971_d_n4;
        locals.var_t0_dn5 = assign90200_e138971_d_n5;
        locals.var_t0_dn6 = assign90200_e138971_d_n6;
        locals.var_t0_dn7 = assign90200_e138971_d_n7;
        locals.var_t0_dn8 = assign90200_e138971_d_n8;
        locals.var_t0_dn9 = assign90200_e138971_d_n9;
        locals.var_t0_dn10 = assign90200_e138971_d_n10;
        locals.var_t0_dn13 = assign90200_e138971_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign90210_e138985, assign90210_e138985_d_n0, assign90210_e138985_d_n2, assign90210_e138985_d_n4, assign90210_e138985_d_n5, assign90210_e138985_d_n6, assign90210_e138985_d_n7, assign90210_e138985_d_n8, assign90210_e138985_d_n9, assign90210_e138985_d_n10, assign90210_e138985_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2100 != 0.0)) && (locals.var_guard2101 != 0.0)) {
        let assign90210_e138981: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign90210_e138982: f64 = (0.5 * assign90210_e138981);
        let assign90210_e138983: f64 = (locals.var_lover_func - assign90210_e138982);
        (assign90210_e138983, (locals.var_lover_func_dn0 - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (locals.var_lover_func_dn2 - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (locals.var_lover_func_dn4 - (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), (locals.var_lover_func_dn5 - (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), (locals.var_lover_func_dn6 - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (locals.var_lover_func_dn7 - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (locals.var_lover_func_dn8 - (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), (locals.var_lover_func_dn9 - (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), (locals.var_lover_func_dn10 - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (locals.var_lover_func_dn13 - (0.5 * (locals.var_tmf1_dn13 + locals.var_tmf2_dn13))),)
    } else {
        (locals.var_wjuncld, locals.var_wjuncld_dn0, locals.var_wjuncld_dn2, locals.var_wjuncld_dn4, locals.var_wjuncld_dn5, locals.var_wjuncld_dn6, locals.var_wjuncld_dn7, locals.var_wjuncld_dn8, locals.var_wjuncld_dn9, locals.var_wjuncld_dn10, locals.var_wjuncld_dn13,)
    }
};
        locals.var_wjuncld = assign90210_e138985;
        locals.var_wjuncld_dn0 = assign90210_e138985_d_n0;
        locals.var_wjuncld_dn2 = assign90210_e138985_d_n2;
        locals.var_wjuncld_dn4 = assign90210_e138985_d_n4;
        locals.var_wjuncld_dn5 = assign90210_e138985_d_n5;
        locals.var_wjuncld_dn6 = assign90210_e138985_d_n6;
        locals.var_wjuncld_dn7 = assign90210_e138985_d_n7;
        locals.var_wjuncld_dn8 = assign90210_e138985_d_n8;
        locals.var_wjuncld_dn9 = assign90210_e138985_d_n9;
        locals.var_wjuncld_dn10 = assign90210_e138985_d_n10;
        locals.var_wjuncld_dn13 = assign90210_e138985_d_n13;
        locals.var_wjuncld_rv = 0.0;

        let (assign90220_e138995, assign90220_e138995_d_n0, assign90220_e138995_d_n2, assign90220_e138995_d_n4, assign90220_e138995_d_n5, assign90220_e138995_d_n6, assign90220_e138995_d_n7, assign90220_e138995_d_n8, assign90220_e138995_d_n9, assign90220_e138995_d_n10, assign90220_e138995_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2100 != 0.0)) && (locals.var_guard2101 != 0.0)) {
        let assign90220_e138993: f64 = (locals.var_lover_func - locals.var_wjuncld);
        (assign90220_e138993, (locals.var_lover_func_dn0 - locals.var_wjuncld_dn0), (locals.var_lover_func_dn2 - locals.var_wjuncld_dn2), (locals.var_lover_func_dn4 - locals.var_wjuncld_dn4), (locals.var_lover_func_dn5 - locals.var_wjuncld_dn5), (locals.var_lover_func_dn6 - locals.var_wjuncld_dn6), (locals.var_lover_func_dn7 - locals.var_wjuncld_dn7), (locals.var_lover_func_dn8 - locals.var_wjuncld_dn8), (locals.var_lover_func_dn9 - locals.var_wjuncld_dn9), (locals.var_lover_func_dn10 - locals.var_wjuncld_dn10), (locals.var_lover_func_dn13 - locals.var_wjuncld_dn13),)
    } else {
        (locals.var_lover_func, locals.var_lover_func_dn0, locals.var_lover_func_dn2, locals.var_lover_func_dn4, locals.var_lover_func_dn5, locals.var_lover_func_dn6, locals.var_lover_func_dn7, locals.var_lover_func_dn8, locals.var_lover_func_dn9, locals.var_lover_func_dn10, locals.var_lover_func_dn13,)
    }
};
        locals.var_lover_func = assign90220_e138995;
        locals.var_lover_func_dn0 = assign90220_e138995_d_n0;
        locals.var_lover_func_dn2 = assign90220_e138995_d_n2;
        locals.var_lover_func_dn4 = assign90220_e138995_d_n4;
        locals.var_lover_func_dn5 = assign90220_e138995_d_n5;
        locals.var_lover_func_dn6 = assign90220_e138995_d_n6;
        locals.var_lover_func_dn7 = assign90220_e138995_d_n7;
        locals.var_lover_func_dn8 = assign90220_e138995_d_n8;
        locals.var_lover_func_dn9 = assign90220_e138995_d_n9;
        locals.var_lover_func_dn10 = assign90220_e138995_d_n10;
        locals.var_lover_func_dn13 = assign90220_e138995_d_n13;
        locals.var_lover_func_rv = 0.0;

        let assign90230_e138998: f64 = if 4.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard2104 = assign90230_e138998;
        locals.var_guard2104_rv = 0.0;

        let assign90240_e139001: f64 = if 4.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard2105 = assign90240_e139001;
        locals.var_guard2105_rv = 0.0;

        let assign90250_e139004: f64 = if 4.0 == 3.0 { 1.0 } else { 0.0 };
        locals.var_guard2106 = assign90250_e139004;
        locals.var_guard2106_rv = 0.0;

        let assign90260_e139007: f64 = if 4.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard2107 = assign90260_e139007;
        locals.var_guard2107_rv = 0.0;

        let assign90270_e139010: f64 = if p.p55 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard2108 = assign90270_e139010;
        locals.var_guard2108_rv = 0.0;

        let (assign90280_e139020, assign90280_e139020_d_n0, assign90280_e139020_d_n2, assign90280_e139020_d_n4, assign90280_e139020_d_n5, assign90280_e139020_d_n6, assign90280_e139020_d_n7, assign90280_e139020_d_n8, assign90280_e139020_d_n9, assign90280_e139020_d_n10, assign90280_e139020_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2104 != 0.0)) && (locals.var_guard2108 != 0.0)) {
        let assign90280_e139018: f64 = (locals.var_weffcv_nf * locals.var_lover_func);
        (assign90280_e139018, (locals.var_weffcv_nf * locals.var_lover_func_dn0), (locals.var_weffcv_nf * locals.var_lover_func_dn2), (locals.var_weffcv_nf * locals.var_lover_func_dn4), (locals.var_weffcv_nf * locals.var_lover_func_dn5), (locals.var_weffcv_nf * locals.var_lover_func_dn6), (locals.var_weffcv_nf * locals.var_lover_func_dn7), (locals.var_weffcv_nf * locals.var_lover_func_dn8), (locals.var_weffcv_nf * locals.var_lover_func_dn9), (locals.var_weffcv_nf * locals.var_lover_func_dn10), (locals.var_weffcv_nf * locals.var_lover_func_dn13),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign90280_e139020;
        locals.var_t4_dn0 = assign90280_e139020_d_n0;
        locals.var_t4_dn2 = assign90280_e139020_d_n2;
        locals.var_t4_dn4 = assign90280_e139020_d_n4;
        locals.var_t4_dn5 = assign90280_e139020_d_n5;
        locals.var_t4_dn6 = assign90280_e139020_d_n6;
        locals.var_t4_dn7 = assign90280_e139020_d_n7;
        locals.var_t4_dn8 = assign90280_e139020_d_n8;
        locals.var_t4_dn9 = assign90280_e139020_d_n9;
        locals.var_t4_dn10 = assign90280_e139020_d_n10;
        locals.var_t4_dn13 = assign90280_e139020_d_n13;
        locals.var_t4_rv = 0.0;

        let (assign90290_e139035, assign90290_e139035_d_n0, assign90290_e139035_d_n2, assign90290_e139035_d_n4, assign90290_e139035_d_n5, assign90290_e139035_d_n6, assign90290_e139035_d_n7, assign90290_e139035_d_n8, assign90290_e139035_d_n9, assign90290_e139035_d_n10, assign90290_e139035_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2104 != 0.0)) && (locals.var_guard2108 == 0.0)) {
        let assign90290_e139029: f64 = (locals.var_weffcv_nf * locals.var_lover_func);
        let assign90290_e139032: f64 = (1.0 - locals.var_uc_cvdsover);
        let assign90290_e139033: f64 = (assign90290_e139029 * assign90290_e139032);
        (assign90290_e139033, ((locals.var_weffcv_nf * locals.var_lover_func_dn0) * assign90290_e139032), ((locals.var_weffcv_nf * locals.var_lover_func_dn2) * assign90290_e139032), ((locals.var_weffcv_nf * locals.var_lover_func_dn4) * assign90290_e139032), ((locals.var_weffcv_nf * locals.var_lover_func_dn5) * assign90290_e139032), ((locals.var_weffcv_nf * locals.var_lover_func_dn6) * assign90290_e139032), ((locals.var_weffcv_nf * locals.var_lover_func_dn7) * assign90290_e139032), ((locals.var_weffcv_nf * locals.var_lover_func_dn8) * assign90290_e139032), ((locals.var_weffcv_nf * locals.var_lover_func_dn9) * assign90290_e139032), ((locals.var_weffcv_nf * locals.var_lover_func_dn10) * assign90290_e139032), ((locals.var_weffcv_nf * locals.var_lover_func_dn13) * assign90290_e139032),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign90290_e139035;
        locals.var_t4_dn0 = assign90290_e139035_d_n0;
        locals.var_t4_dn2 = assign90290_e139035_d_n2;
        locals.var_t4_dn4 = assign90290_e139035_d_n4;
        locals.var_t4_dn5 = assign90290_e139035_d_n5;
        locals.var_t4_dn6 = assign90290_e139035_d_n6;
        locals.var_t4_dn7 = assign90290_e139035_d_n7;
        locals.var_t4_dn8 = assign90290_e139035_d_n8;
        locals.var_t4_dn9 = assign90290_e139035_d_n9;
        locals.var_t4_dn10 = assign90290_e139035_d_n10;
        locals.var_t4_dn13 = assign90290_e139035_d_n13;
        locals.var_t4_rv = 0.0;

        let (assign90300_e139043, assign90300_e139043_d_n0, assign90300_e139043_d_n2, assign90300_e139043_d_n4, assign90300_e139043_d_n5, assign90300_e139043_d_n6, assign90300_e139043_d_n7, assign90300_e139043_d_n8, assign90300_e139043_d_n9, assign90300_e139043_d_n10, assign90300_e139043_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2104 != 0.0)) {
        let assign90300_e139041: f64 = (locals.var_t4 * locals.var_qsuld);
        (assign90300_e139041, ((locals.var_t4_dn0 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn0)), ((locals.var_t4_dn2 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn2)), ((locals.var_t4_dn4 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn4)), ((locals.var_t4_dn5 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn5)), ((locals.var_t4_dn6 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn6)), ((locals.var_t4_dn7 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn7)), ((locals.var_t4_dn8 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn8)), ((locals.var_t4_dn9 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn9)), ((locals.var_t4_dn10 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn10)), ((locals.var_t4_dn13 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn13)),)
    } else {
        (locals.var_qovs, locals.var_qovs_dn0, locals.var_qovs_dn2, locals.var_qovs_dn4, locals.var_qovs_dn5, locals.var_qovs_dn6, locals.var_qovs_dn7, locals.var_qovs_dn8, locals.var_qovs_dn9, locals.var_qovs_dn10, locals.var_qovs_dn13,)
    }
};
        locals.var_qovs = assign90300_e139043;
        locals.var_qovs_dn0 = assign90300_e139043_d_n0;
        locals.var_qovs_dn2 = assign90300_e139043_d_n2;
        locals.var_qovs_dn4 = assign90300_e139043_d_n4;
        locals.var_qovs_dn5 = assign90300_e139043_d_n5;
        locals.var_qovs_dn6 = assign90300_e139043_d_n6;
        locals.var_qovs_dn7 = assign90300_e139043_d_n7;
        locals.var_qovs_dn8 = assign90300_e139043_d_n8;
        locals.var_qovs_dn9 = assign90300_e139043_d_n9;
        locals.var_qovs_dn10 = assign90300_e139043_d_n10;
        locals.var_qovs_dn13 = assign90300_e139043_d_n13;
        locals.var_qovs_rv = 0.0;

        let (assign90310_e139051, assign90310_e139051_d_n0, assign90310_e139051_d_n2, assign90310_e139051_d_n4, assign90310_e139051_d_n5, assign90310_e139051_d_n6, assign90310_e139051_d_n7, assign90310_e139051_d_n8, assign90310_e139051_d_n9, assign90310_e139051_d_n10, assign90310_e139051_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2104 != 0.0)) {
        let assign90310_e139049: f64 = (locals.var_t4 * locals.var_qbuld);
        (assign90310_e139049, ((locals.var_t4_dn0 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn0)), ((locals.var_t4_dn2 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn2)), ((locals.var_t4_dn4 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn4)), ((locals.var_t4_dn5 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn5)), ((locals.var_t4_dn6 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn6)), ((locals.var_t4_dn7 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn7)), ((locals.var_t4_dn8 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn8)), ((locals.var_t4_dn9 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn9)), ((locals.var_t4_dn10 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn10)), ((locals.var_t4_dn13 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn13)),)
    } else {
        (locals.var_qbsld, locals.var_qbsld_dn0, locals.var_qbsld_dn2, locals.var_qbsld_dn4, locals.var_qbsld_dn5, locals.var_qbsld_dn6, locals.var_qbsld_dn7, locals.var_qbsld_dn8, locals.var_qbsld_dn9, locals.var_qbsld_dn10, locals.var_qbsld_dn13,)
    }
};
        locals.var_qbsld = assign90310_e139051;
        locals.var_qbsld_dn0 = assign90310_e139051_d_n0;
        locals.var_qbsld_dn2 = assign90310_e139051_d_n2;
        locals.var_qbsld_dn4 = assign90310_e139051_d_n4;
        locals.var_qbsld_dn5 = assign90310_e139051_d_n5;
        locals.var_qbsld_dn6 = assign90310_e139051_d_n6;
        locals.var_qbsld_dn7 = assign90310_e139051_d_n7;
        locals.var_qbsld_dn8 = assign90310_e139051_d_n8;
        locals.var_qbsld_dn9 = assign90310_e139051_d_n9;
        locals.var_qbsld_dn10 = assign90310_e139051_d_n10;
        locals.var_qbsld_dn13 = assign90310_e139051_d_n13;
        locals.var_qbsld_rv = 0.0;

        let (assign90340_e139076, assign90340_e139076_d_n0, assign90340_e139076_d_n2, assign90340_e139076_d_n4, assign90340_e139076_d_n5, assign90340_e139076_d_n6, assign90340_e139076_d_n7, assign90340_e139076_d_n8, assign90340_e139076_d_n9, assign90340_e139076_d_n10, assign90340_e139076_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && ((locals.var_guard2105 != 0.0) && (locals.var_guard2104 == 0.0))) {
        let assign90340_e139072: f64 = (locals.var_weffcv_nf * locals.var_lover_func);
        let assign90340_e139074: f64 = (assign90340_e139072 * locals.var_uc_cvdsover);
        (assign90340_e139074, ((locals.var_weffcv_nf * locals.var_lover_func_dn0) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn2) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn4) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn5) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn6) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn7) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn8) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn9) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn10) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn13) * locals.var_uc_cvdsover),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign90340_e139076;
        locals.var_t4_dn0 = assign90340_e139076_d_n0;
        locals.var_t4_dn2 = assign90340_e139076_d_n2;
        locals.var_t4_dn4 = assign90340_e139076_d_n4;
        locals.var_t4_dn5 = assign90340_e139076_d_n5;
        locals.var_t4_dn6 = assign90340_e139076_d_n6;
        locals.var_t4_dn7 = assign90340_e139076_d_n7;
        locals.var_t4_dn8 = assign90340_e139076_d_n8;
        locals.var_t4_dn9 = assign90340_e139076_d_n9;
        locals.var_t4_dn10 = assign90340_e139076_d_n10;
        locals.var_t4_dn13 = assign90340_e139076_d_n13;
        locals.var_t4_rv = 0.0;

        let (assign90350_e139087, assign90350_e139087_d_n0, assign90350_e139087_d_n2, assign90350_e139087_d_n4, assign90350_e139087_d_n5, assign90350_e139087_d_n6, assign90350_e139087_d_n7, assign90350_e139087_d_n8, assign90350_e139087_d_n9, assign90350_e139087_d_n10, assign90350_e139087_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && ((locals.var_guard2105 != 0.0) && (locals.var_guard2104 == 0.0))) {
        let assign90350_e139085: f64 = (locals.var_t4 * locals.var_qsuld);
        (assign90350_e139085, ((locals.var_t4_dn0 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn0)), ((locals.var_t4_dn2 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn2)), ((locals.var_t4_dn4 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn4)), ((locals.var_t4_dn5 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn5)), ((locals.var_t4_dn6 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn6)), ((locals.var_t4_dn7 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn7)), ((locals.var_t4_dn8 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn8)), ((locals.var_t4_dn9 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn9)), ((locals.var_t4_dn10 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn10)), ((locals.var_t4_dn13 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn13)),)
    } else {
        (locals.var_qovsext, locals.var_qovsext_dn0, locals.var_qovsext_dn2, locals.var_qovsext_dn4, locals.var_qovsext_dn5, locals.var_qovsext_dn6, locals.var_qovsext_dn7, locals.var_qovsext_dn8, locals.var_qovsext_dn9, locals.var_qovsext_dn10, locals.var_qovsext_dn13,)
    }
};
        locals.var_qovsext = assign90350_e139087;
        locals.var_qovsext_dn0 = assign90350_e139087_d_n0;
        locals.var_qovsext_dn2 = assign90350_e139087_d_n2;
        locals.var_qovsext_dn4 = assign90350_e139087_d_n4;
        locals.var_qovsext_dn5 = assign90350_e139087_d_n5;
        locals.var_qovsext_dn6 = assign90350_e139087_d_n6;
        locals.var_qovsext_dn7 = assign90350_e139087_d_n7;
        locals.var_qovsext_dn8 = assign90350_e139087_d_n8;
        locals.var_qovsext_dn9 = assign90350_e139087_d_n9;
        locals.var_qovsext_dn10 = assign90350_e139087_d_n10;
        locals.var_qovsext_dn13 = assign90350_e139087_d_n13;
        locals.var_qovsext_rv = 0.0;

        let (assign90360_e139098, assign90360_e139098_d_n0, assign90360_e139098_d_n2, assign90360_e139098_d_n4, assign90360_e139098_d_n5, assign90360_e139098_d_n6, assign90360_e139098_d_n7, assign90360_e139098_d_n8, assign90360_e139098_d_n9, assign90360_e139098_d_n10, assign90360_e139098_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && ((locals.var_guard2105 != 0.0) && (locals.var_guard2104 == 0.0))) {
        let assign90360_e139096: f64 = (locals.var_t4 * locals.var_qbuld);
        (assign90360_e139096, ((locals.var_t4_dn0 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn0)), ((locals.var_t4_dn2 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn2)), ((locals.var_t4_dn4 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn4)), ((locals.var_t4_dn5 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn5)), ((locals.var_t4_dn6 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn6)), ((locals.var_t4_dn7 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn7)), ((locals.var_t4_dn8 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn8)), ((locals.var_t4_dn9 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn9)), ((locals.var_t4_dn10 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn10)), ((locals.var_t4_dn13 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn13)),)
    } else {
        (locals.var_qbsldext, locals.var_qbsldext_dn0, locals.var_qbsldext_dn2, locals.var_qbsldext_dn4, locals.var_qbsldext_dn5, locals.var_qbsldext_dn6, locals.var_qbsldext_dn7, locals.var_qbsldext_dn8, locals.var_qbsldext_dn9, locals.var_qbsldext_dn10, locals.var_qbsldext_dn13,)
    }
};
        locals.var_qbsldext = assign90360_e139098;
        locals.var_qbsldext_dn0 = assign90360_e139098_d_n0;
        locals.var_qbsldext_dn2 = assign90360_e139098_d_n2;
        locals.var_qbsldext_dn4 = assign90360_e139098_d_n4;
        locals.var_qbsldext_dn5 = assign90360_e139098_d_n5;
        locals.var_qbsldext_dn6 = assign90360_e139098_d_n6;
        locals.var_qbsldext_dn7 = assign90360_e139098_d_n7;
        locals.var_qbsldext_dn8 = assign90360_e139098_d_n8;
        locals.var_qbsldext_dn9 = assign90360_e139098_d_n9;
        locals.var_qbsldext_dn10 = assign90360_e139098_d_n10;
        locals.var_qbsldext_dn13 = assign90360_e139098_d_n13;
        locals.var_qbsldext_rv = 0.0;

        let assign90370_e139101: f64 = if p.p55 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard2109 = assign90370_e139101;
        locals.var_guard2109_rv = 0.0;

        let (assign90380_e139116, assign90380_e139116_d_n0, assign90380_e139116_d_n2, assign90380_e139116_d_n4, assign90380_e139116_d_n5, assign90380_e139116_d_n6, assign90380_e139116_d_n7, assign90380_e139116_d_n8, assign90380_e139116_d_n9, assign90380_e139116_d_n10, assign90380_e139116_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && ((locals.var_guard2106 != 0.0) && (!((locals.var_guard2104 != 0.0) || (locals.var_guard2105 != 0.0))))) && (locals.var_guard2109 != 0.0)) {
        let assign90380_e139114: f64 = (locals.var_weffcv_nf * locals.var_lover_func);
        (assign90380_e139114, (locals.var_weffcv_nf * locals.var_lover_func_dn0), (locals.var_weffcv_nf * locals.var_lover_func_dn2), (locals.var_weffcv_nf * locals.var_lover_func_dn4), (locals.var_weffcv_nf * locals.var_lover_func_dn5), (locals.var_weffcv_nf * locals.var_lover_func_dn6), (locals.var_weffcv_nf * locals.var_lover_func_dn7), (locals.var_weffcv_nf * locals.var_lover_func_dn8), (locals.var_weffcv_nf * locals.var_lover_func_dn9), (locals.var_weffcv_nf * locals.var_lover_func_dn10), (locals.var_weffcv_nf * locals.var_lover_func_dn13),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign90380_e139116;
        locals.var_t4_dn0 = assign90380_e139116_d_n0;
        locals.var_t4_dn2 = assign90380_e139116_d_n2;
        locals.var_t4_dn4 = assign90380_e139116_d_n4;
        locals.var_t4_dn5 = assign90380_e139116_d_n5;
        locals.var_t4_dn6 = assign90380_e139116_d_n6;
        locals.var_t4_dn7 = assign90380_e139116_d_n7;
        locals.var_t4_dn8 = assign90380_e139116_d_n8;
        locals.var_t4_dn9 = assign90380_e139116_d_n9;
        locals.var_t4_dn10 = assign90380_e139116_d_n10;
        locals.var_t4_dn13 = assign90380_e139116_d_n13;
        locals.var_t4_rv = 0.0;

        let (assign90390_e139136, assign90390_e139136_d_n0, assign90390_e139136_d_n2, assign90390_e139136_d_n4, assign90390_e139136_d_n5, assign90390_e139136_d_n6, assign90390_e139136_d_n7, assign90390_e139136_d_n8, assign90390_e139136_d_n9, assign90390_e139136_d_n10, assign90390_e139136_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && ((locals.var_guard2106 != 0.0) && (!((locals.var_guard2104 != 0.0) || (locals.var_guard2105 != 0.0))))) && (locals.var_guard2109 == 0.0)) {
        let assign90390_e139130: f64 = (locals.var_weffcv_nf * locals.var_lover_func);
        let assign90390_e139133: f64 = (1.0 - locals.var_uc_cvdsover);
        let assign90390_e139134: f64 = (assign90390_e139130 * assign90390_e139133);
        (assign90390_e139134, ((locals.var_weffcv_nf * locals.var_lover_func_dn0) * assign90390_e139133), ((locals.var_weffcv_nf * locals.var_lover_func_dn2) * assign90390_e139133), ((locals.var_weffcv_nf * locals.var_lover_func_dn4) * assign90390_e139133), ((locals.var_weffcv_nf * locals.var_lover_func_dn5) * assign90390_e139133), ((locals.var_weffcv_nf * locals.var_lover_func_dn6) * assign90390_e139133), ((locals.var_weffcv_nf * locals.var_lover_func_dn7) * assign90390_e139133), ((locals.var_weffcv_nf * locals.var_lover_func_dn8) * assign90390_e139133), ((locals.var_weffcv_nf * locals.var_lover_func_dn9) * assign90390_e139133), ((locals.var_weffcv_nf * locals.var_lover_func_dn10) * assign90390_e139133), ((locals.var_weffcv_nf * locals.var_lover_func_dn13) * assign90390_e139133),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign90390_e139136;
        locals.var_t4_dn0 = assign90390_e139136_d_n0;
        locals.var_t4_dn2 = assign90390_e139136_d_n2;
        locals.var_t4_dn4 = assign90390_e139136_d_n4;
        locals.var_t4_dn5 = assign90390_e139136_d_n5;
        locals.var_t4_dn6 = assign90390_e139136_d_n6;
        locals.var_t4_dn7 = assign90390_e139136_d_n7;
        locals.var_t4_dn8 = assign90390_e139136_d_n8;
        locals.var_t4_dn9 = assign90390_e139136_d_n9;
        locals.var_t4_dn10 = assign90390_e139136_d_n10;
        locals.var_t4_dn13 = assign90390_e139136_d_n13;
        locals.var_t4_rv = 0.0;

        let (assign90400_e139147, assign90400_e139147_d_n0, assign90400_e139147_d_n2, assign90400_e139147_d_n4, assign90400_e139147_d_n5, assign90400_e139147_d_n6, assign90400_e139147_d_n7, assign90400_e139147_d_n8, assign90400_e139147_d_n9, assign90400_e139147_d_n10, assign90400_e139147_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && ((locals.var_guard2106 != 0.0) && (!((locals.var_guard2104 != 0.0) || (locals.var_guard2105 != 0.0))))) {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn13,)
    } else {
        (locals.var_rd_ps0ld, locals.var_rd_ps0ld_dn0, locals.var_rd_ps0ld_dn2, locals.var_rd_ps0ld_dn4, locals.var_rd_ps0ld_dn5, locals.var_rd_ps0ld_dn6, locals.var_rd_ps0ld_dn7, locals.var_rd_ps0ld_dn8, locals.var_rd_ps0ld_dn9, locals.var_rd_ps0ld_dn10, locals.var_rd_ps0ld_dn13,)
    }
};
        locals.var_rd_ps0ld = assign90400_e139147;
        locals.var_rd_ps0ld_dn0 = assign90400_e139147_d_n0;
        locals.var_rd_ps0ld_dn2 = assign90400_e139147_d_n2;
        locals.var_rd_ps0ld_dn4 = assign90400_e139147_d_n4;
        locals.var_rd_ps0ld_dn5 = assign90400_e139147_d_n5;
        locals.var_rd_ps0ld_dn6 = assign90400_e139147_d_n6;
        locals.var_rd_ps0ld_dn7 = assign90400_e139147_d_n7;
        locals.var_rd_ps0ld_dn8 = assign90400_e139147_d_n8;
        locals.var_rd_ps0ld_dn9 = assign90400_e139147_d_n9;
        locals.var_rd_ps0ld_dn10 = assign90400_e139147_d_n10;
        locals.var_rd_ps0ld_dn13 = assign90400_e139147_d_n13;
        locals.var_rd_ps0ld_rv = 0.0;

        let assign90410_e139150: f64 = if p.p430 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2110 = assign90410_e139150;
        locals.var_guard2110_rv = 0.0;

        let (assign90420_e139163, assign90420_e139163_d_n0, assign90420_e139163_d_n2, assign90420_e139163_d_n4, assign90420_e139163_d_n5, assign90420_e139163_d_n6, assign90420_e139163_d_n7, assign90420_e139163_d_n8, assign90420_e139163_d_n9, assign90420_e139163_d_n10, assign90420_e139163_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && ((locals.var_guard2106 != 0.0) && (!((locals.var_guard2104 != 0.0) || (locals.var_guard2105 != 0.0))))) && (locals.var_guard2110 != 0.0)) {
        (locals.var_qbuld, locals.var_qbuld_dn0, locals.var_qbuld_dn2, locals.var_qbuld_dn4, locals.var_qbuld_dn5, locals.var_qbuld_dn6, locals.var_qbuld_dn7, locals.var_qbuld_dn8, locals.var_qbuld_dn9, locals.var_qbuld_dn10, locals.var_qbuld_dn13,)
    } else {
        (locals.var_rd_qbuld, locals.var_rd_qbuld_dn0, locals.var_rd_qbuld_dn2, locals.var_rd_qbuld_dn4, locals.var_rd_qbuld_dn5, locals.var_rd_qbuld_dn6, locals.var_rd_qbuld_dn7, locals.var_rd_qbuld_dn8, locals.var_rd_qbuld_dn9, locals.var_rd_qbuld_dn10, locals.var_rd_qbuld_dn13,)
    }
};
        locals.var_rd_qbuld = assign90420_e139163;
        locals.var_rd_qbuld_dn0 = assign90420_e139163_d_n0;
        locals.var_rd_qbuld_dn2 = assign90420_e139163_d_n2;
        locals.var_rd_qbuld_dn4 = assign90420_e139163_d_n4;
        locals.var_rd_qbuld_dn5 = assign90420_e139163_d_n5;
        locals.var_rd_qbuld_dn6 = assign90420_e139163_d_n6;
        locals.var_rd_qbuld_dn7 = assign90420_e139163_d_n7;
        locals.var_rd_qbuld_dn8 = assign90420_e139163_d_n8;
        locals.var_rd_qbuld_dn9 = assign90420_e139163_d_n9;
        locals.var_rd_qbuld_dn10 = assign90420_e139163_d_n10;
        locals.var_rd_qbuld_dn13 = assign90420_e139163_d_n13;
        locals.var_rd_qbuld_rv = 0.0;

        let (assign90430_e139176, assign90430_e139176_d_n0, assign90430_e139176_d_n2, assign90430_e139176_d_n4, assign90430_e139176_d_n5, assign90430_e139176_d_n6, assign90430_e139176_d_n7, assign90430_e139176_d_n8, assign90430_e139176_d_n9, assign90430_e139176_d_n10, assign90430_e139176_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && ((locals.var_guard2106 != 0.0) && (!((locals.var_guard2104 != 0.0) || (locals.var_guard2105 != 0.0))))) {
        let assign90430_e139174: f64 = (locals.var_t4 * locals.var_qsuld);
        (assign90430_e139174, ((locals.var_t4_dn0 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn0)), ((locals.var_t4_dn2 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn2)), ((locals.var_t4_dn4 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn4)), ((locals.var_t4_dn5 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn5)), ((locals.var_t4_dn6 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn6)), ((locals.var_t4_dn7 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn7)), ((locals.var_t4_dn8 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn8)), ((locals.var_t4_dn9 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn9)), ((locals.var_t4_dn10 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn10)), ((locals.var_t4_dn13 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn13)),)
    } else {
        (locals.var_qovd, locals.var_qovd_dn0, locals.var_qovd_dn2, locals.var_qovd_dn4, locals.var_qovd_dn5, locals.var_qovd_dn6, locals.var_qovd_dn7, locals.var_qovd_dn8, locals.var_qovd_dn9, locals.var_qovd_dn10, locals.var_qovd_dn13,)
    }
};
        locals.var_qovd = assign90430_e139176;
        locals.var_qovd_dn0 = assign90430_e139176_d_n0;
        locals.var_qovd_dn2 = assign90430_e139176_d_n2;
        locals.var_qovd_dn4 = assign90430_e139176_d_n4;
        locals.var_qovd_dn5 = assign90430_e139176_d_n5;
        locals.var_qovd_dn6 = assign90430_e139176_d_n6;
        locals.var_qovd_dn7 = assign90430_e139176_d_n7;
        locals.var_qovd_dn8 = assign90430_e139176_d_n8;
        locals.var_qovd_dn9 = assign90430_e139176_d_n9;
        locals.var_qovd_dn10 = assign90430_e139176_d_n10;
        locals.var_qovd_dn13 = assign90430_e139176_d_n13;
        locals.var_qovd_rv = 0.0;

        let (assign90440_e139189, assign90440_e139189_d_n0, assign90440_e139189_d_n2, assign90440_e139189_d_n4, assign90440_e139189_d_n5, assign90440_e139189_d_n6, assign90440_e139189_d_n7, assign90440_e139189_d_n8, assign90440_e139189_d_n9, assign90440_e139189_d_n10, assign90440_e139189_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && ((locals.var_guard2106 != 0.0) && (!((locals.var_guard2104 != 0.0) || (locals.var_guard2105 != 0.0))))) {
        let assign90440_e139187: f64 = (locals.var_t4 * locals.var_qbuld);
        (assign90440_e139187, ((locals.var_t4_dn0 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn0)), ((locals.var_t4_dn2 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn2)), ((locals.var_t4_dn4 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn4)), ((locals.var_t4_dn5 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn5)), ((locals.var_t4_dn6 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn6)), ((locals.var_t4_dn7 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn7)), ((locals.var_t4_dn8 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn8)), ((locals.var_t4_dn9 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn9)), ((locals.var_t4_dn10 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn10)), ((locals.var_t4_dn13 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn13)),)
    } else {
        (locals.var_qbdld, locals.var_qbdld_dn0, locals.var_qbdld_dn2, locals.var_qbdld_dn4, locals.var_qbdld_dn5, locals.var_qbdld_dn6, locals.var_qbdld_dn7, locals.var_qbdld_dn8, locals.var_qbdld_dn9, locals.var_qbdld_dn10, locals.var_qbdld_dn13,)
    }
};
        locals.var_qbdld = assign90440_e139189;
        locals.var_qbdld_dn0 = assign90440_e139189_d_n0;
        locals.var_qbdld_dn2 = assign90440_e139189_d_n2;
        locals.var_qbdld_dn4 = assign90440_e139189_d_n4;
        locals.var_qbdld_dn5 = assign90440_e139189_d_n5;
        locals.var_qbdld_dn6 = assign90440_e139189_d_n6;
        locals.var_qbdld_dn7 = assign90440_e139189_d_n7;
        locals.var_qbdld_dn8 = assign90440_e139189_d_n8;
        locals.var_qbdld_dn9 = assign90440_e139189_d_n9;
        locals.var_qbdld_dn10 = assign90440_e139189_d_n10;
        locals.var_qbdld_dn13 = assign90440_e139189_d_n13;
        locals.var_qbdld_rv = 0.0;

        let (assign90450_e139200, assign90450_e139200_d_n0, assign90450_e139200_d_n2, assign90450_e139200_d_n4, assign90450_e139200_d_n5, assign90450_e139200_d_n6, assign90450_e139200_d_n7, assign90450_e139200_d_n8, assign90450_e139200_d_n9, assign90450_e139200_d_n10, assign90450_e139200_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && ((locals.var_guard2106 != 0.0) && (!((locals.var_guard2104 != 0.0) || (locals.var_guard2105 != 0.0))))) {
        (locals.var_qbdld, locals.var_qbdld_dn0, locals.var_qbdld_dn2, locals.var_qbdld_dn4, locals.var_qbdld_dn5, locals.var_qbdld_dn6, locals.var_qbdld_dn7, locals.var_qbdld_dn8, locals.var_qbdld_dn9, locals.var_qbdld_dn10, locals.var_qbdld_dn13,)
    } else {
        (locals.var_qbd_qs, locals.var_qbd_qs_dn0, locals.var_qbd_qs_dn2, locals.var_qbd_qs_dn4, locals.var_qbd_qs_dn5, locals.var_qbd_qs_dn6, locals.var_qbd_qs_dn7, locals.var_qbd_qs_dn8, locals.var_qbd_qs_dn9, locals.var_qbd_qs_dn10, locals.var_qbd_qs_dn13,)
    }
};
        locals.var_qbd_qs = assign90450_e139200;
        locals.var_qbd_qs_dn0 = assign90450_e139200_d_n0;
        locals.var_qbd_qs_dn2 = assign90450_e139200_d_n2;
        locals.var_qbd_qs_dn4 = assign90450_e139200_d_n4;
        locals.var_qbd_qs_dn5 = assign90450_e139200_d_n5;
        locals.var_qbd_qs_dn6 = assign90450_e139200_d_n6;
        locals.var_qbd_qs_dn7 = assign90450_e139200_d_n7;
        locals.var_qbd_qs_dn8 = assign90450_e139200_d_n8;
        locals.var_qbd_qs_dn9 = assign90450_e139200_d_n9;
        locals.var_qbd_qs_dn10 = assign90450_e139200_d_n10;
        locals.var_qbd_qs_dn13 = assign90450_e139200_d_n13;
        locals.var_qbd_qs_rv = 0.0;

        let (assign90460_e139217, assign90460_e139217_d_n0, assign90460_e139217_d_n2, assign90460_e139217_d_n4, assign90460_e139217_d_n5, assign90460_e139217_d_n6, assign90460_e139217_d_n7, assign90460_e139217_d_n8, assign90460_e139217_d_n9, assign90460_e139217_d_n10, assign90460_e139217_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && ((locals.var_guard2107 != 0.0) && (!(((locals.var_guard2104 != 0.0) || (locals.var_guard2105 != 0.0)) || (locals.var_guard2106 != 0.0))))) {
        let assign90460_e139213: f64 = (locals.var_weffcv_nf * locals.var_lover_func);
        let assign90460_e139215: f64 = (assign90460_e139213 * locals.var_uc_cvdsover);
        (assign90460_e139215, ((locals.var_weffcv_nf * locals.var_lover_func_dn0) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn2) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn4) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn5) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn6) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn7) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn8) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn9) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn10) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn13) * locals.var_uc_cvdsover),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign90460_e139217;
        locals.var_t4_dn0 = assign90460_e139217_d_n0;
        locals.var_t4_dn2 = assign90460_e139217_d_n2;
        locals.var_t4_dn4 = assign90460_e139217_d_n4;
        locals.var_t4_dn5 = assign90460_e139217_d_n5;
        locals.var_t4_dn6 = assign90460_e139217_d_n6;
        locals.var_t4_dn7 = assign90460_e139217_d_n7;
        locals.var_t4_dn8 = assign90460_e139217_d_n8;
        locals.var_t4_dn9 = assign90460_e139217_d_n9;
        locals.var_t4_dn10 = assign90460_e139217_d_n10;
        locals.var_t4_dn13 = assign90460_e139217_d_n13;
        locals.var_t4_rv = 0.0;

        let (assign90470_e139232, assign90470_e139232_d_n0, assign90470_e139232_d_n2, assign90470_e139232_d_n4, assign90470_e139232_d_n5, assign90470_e139232_d_n6, assign90470_e139232_d_n7, assign90470_e139232_d_n8, assign90470_e139232_d_n9, assign90470_e139232_d_n10, assign90470_e139232_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && ((locals.var_guard2107 != 0.0) && (!(((locals.var_guard2104 != 0.0) || (locals.var_guard2105 != 0.0)) || (locals.var_guard2106 != 0.0))))) {
        let assign90470_e139230: f64 = (locals.var_t4 * locals.var_qsuld);
        (assign90470_e139230, ((locals.var_t4_dn0 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn0)), ((locals.var_t4_dn2 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn2)), ((locals.var_t4_dn4 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn4)), ((locals.var_t4_dn5 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn5)), ((locals.var_t4_dn6 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn6)), ((locals.var_t4_dn7 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn7)), ((locals.var_t4_dn8 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn8)), ((locals.var_t4_dn9 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn9)), ((locals.var_t4_dn10 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn10)), ((locals.var_t4_dn13 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn13)),)
    } else {
        (locals.var_qovdext, locals.var_qovdext_dn0, locals.var_qovdext_dn2, locals.var_qovdext_dn4, locals.var_qovdext_dn5, locals.var_qovdext_dn6, locals.var_qovdext_dn7, locals.var_qovdext_dn8, locals.var_qovdext_dn9, locals.var_qovdext_dn10, locals.var_qovdext_dn13,)
    }
};
        locals.var_qovdext = assign90470_e139232;
        locals.var_qovdext_dn0 = assign90470_e139232_d_n0;
        locals.var_qovdext_dn2 = assign90470_e139232_d_n2;
        locals.var_qovdext_dn4 = assign90470_e139232_d_n4;
        locals.var_qovdext_dn5 = assign90470_e139232_d_n5;
        locals.var_qovdext_dn6 = assign90470_e139232_d_n6;
        locals.var_qovdext_dn7 = assign90470_e139232_d_n7;
        locals.var_qovdext_dn8 = assign90470_e139232_d_n8;
        locals.var_qovdext_dn9 = assign90470_e139232_d_n9;
        locals.var_qovdext_dn10 = assign90470_e139232_d_n10;
        locals.var_qovdext_dn13 = assign90470_e139232_d_n13;
        locals.var_qovdext_rv = 0.0;

        let (assign90480_e139247, assign90480_e139247_d_n0, assign90480_e139247_d_n2, assign90480_e139247_d_n4, assign90480_e139247_d_n5, assign90480_e139247_d_n6, assign90480_e139247_d_n7, assign90480_e139247_d_n8, assign90480_e139247_d_n9, assign90480_e139247_d_n10, assign90480_e139247_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && ((locals.var_guard2107 != 0.0) && (!(((locals.var_guard2104 != 0.0) || (locals.var_guard2105 != 0.0)) || (locals.var_guard2106 != 0.0))))) {
        let assign90480_e139245: f64 = (locals.var_t4 * locals.var_qbuld);
        (assign90480_e139245, ((locals.var_t4_dn0 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn0)), ((locals.var_t4_dn2 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn2)), ((locals.var_t4_dn4 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn4)), ((locals.var_t4_dn5 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn5)), ((locals.var_t4_dn6 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn6)), ((locals.var_t4_dn7 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn7)), ((locals.var_t4_dn8 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn8)), ((locals.var_t4_dn9 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn9)), ((locals.var_t4_dn10 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn10)), ((locals.var_t4_dn13 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn13)),)
    } else {
        (locals.var_qbdldext, locals.var_qbdldext_dn0, locals.var_qbdldext_dn2, locals.var_qbdldext_dn4, locals.var_qbdldext_dn5, locals.var_qbdldext_dn6, locals.var_qbdldext_dn7, locals.var_qbdldext_dn8, locals.var_qbdldext_dn9, locals.var_qbdldext_dn10, locals.var_qbdldext_dn13,)
    }
};
        locals.var_qbdldext = assign90480_e139247;
        locals.var_qbdldext_dn0 = assign90480_e139247_d_n0;
        locals.var_qbdldext_dn2 = assign90480_e139247_d_n2;
        locals.var_qbdldext_dn4 = assign90480_e139247_d_n4;
        locals.var_qbdldext_dn5 = assign90480_e139247_d_n5;
        locals.var_qbdldext_dn6 = assign90480_e139247_d_n6;
        locals.var_qbdldext_dn7 = assign90480_e139247_d_n7;
        locals.var_qbdldext_dn8 = assign90480_e139247_d_n8;
        locals.var_qbdldext_dn9 = assign90480_e139247_d_n9;
        locals.var_qbdldext_dn10 = assign90480_e139247_d_n10;
        locals.var_qbdldext_dn13 = assign90480_e139247_d_n13;
        locals.var_qbdldext_rv = 0.0;

        let assign90490_e139250: f64 = if p.p430 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2111 = assign90490_e139250;
        locals.var_guard2111_rv = 0.0;

        let (assign90500_e139254,) = {
    if (locals.var_guard2111 != 0.0) {
        (1.0,)
    } else {
        (locals.var_flg_never_reach_vfbover,)
    }
};
        locals.var_flg_never_reach_vfbover = assign90500_e139254;
        locals.var_flg_never_reach_vfbover_rv = 0.0;

        let assign90510_e139265: f64 = if (((p.p35 == 1.0) && (p.p63 > 0.0)) && (locals.var_uc_nover > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard2112 = assign90510_e139265;
        locals.var_guard2112_rv = 0.0;

        let (assign90520_e139273, assign90520_e139273_d_n2, assign90520_e139273_d_n6, assign90520_e139273_d_n7, assign90520_e139273_d_n8,) = {
    if ((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) {
        let assign90520_e139271: f64 = (locals.var_vgsi - locals.var_vbsi);
        (assign90520_e139271, 0.0, locals.var_vgsi_dn6, (locals.var_vgsi_dn7 - locals.var_vbsi_dn7), (-locals.var_vbsi_dn8),)
    } else {
        (locals.var_vgbgmt, locals.var_vgbgmt_dn2, locals.var_vgbgmt_dn6, locals.var_vgbgmt_dn7, locals.var_vgbgmt_dn8,)
    }
};
        locals.var_vgbgmt = assign90520_e139273;
        locals.var_vgbgmt_dn2 = assign90520_e139273_d_n2;
        locals.var_vgbgmt_dn6 = assign90520_e139273_d_n6;
        locals.var_vgbgmt_dn7 = assign90520_e139273_d_n7;
        locals.var_vgbgmt_dn8 = assign90520_e139273_d_n8;
        locals.var_vgbgmt_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_338(
        locals: &mut StampLocals,
    ) {
        let (assign90530_e139281, assign90530_e139281_d_n0, assign90530_e139281_d_n2, assign90530_e139281_d_n4, assign90530_e139281_d_n5, assign90530_e139281_d_n6, assign90530_e139281_d_n7, assign90530_e139281_d_n8, assign90530_e139281_d_n9, assign90530_e139281_d_n10, assign90530_e139281_d_n13,) = {
    if ((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) {
        let assign90530_e139279: f64 = (locals.var_vdsi - locals.var_vbsi);
        (assign90530_e139279, 0.0, 0.0, 0.0, locals.var_vdsi_dn5, 0.0, (locals.var_vdsi_dn7 - locals.var_vbsi_dn7), (-locals.var_vbsi_dn8), 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vxbgmt, locals.var_vxbgmt_dn0, locals.var_vxbgmt_dn2, locals.var_vxbgmt_dn4, locals.var_vxbgmt_dn5, locals.var_vxbgmt_dn6, locals.var_vxbgmt_dn7, locals.var_vxbgmt_dn8, locals.var_vxbgmt_dn9, locals.var_vxbgmt_dn10, locals.var_vxbgmt_dn13,)
    }
};
        locals.var_vxbgmt = assign90530_e139281;
        locals.var_vxbgmt_dn0 = assign90530_e139281_d_n0;
        locals.var_vxbgmt_dn2 = assign90530_e139281_d_n2;
        locals.var_vxbgmt_dn4 = assign90530_e139281_d_n4;
        locals.var_vxbgmt_dn5 = assign90530_e139281_d_n5;
        locals.var_vxbgmt_dn6 = assign90530_e139281_d_n6;
        locals.var_vxbgmt_dn7 = assign90530_e139281_d_n7;
        locals.var_vxbgmt_dn8 = assign90530_e139281_d_n8;
        locals.var_vxbgmt_dn9 = assign90530_e139281_d_n9;
        locals.var_vxbgmt_dn10 = assign90530_e139281_d_n10;
        locals.var_vxbgmt_dn13 = assign90530_e139281_d_n13;
        locals.var_vxbgmt_rv = 0.0;

        let (assign90540_e139287,) = {
    if ((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) {
        (locals.var_uc_nover,)
    } else {
        (locals.var_nover_func,)
    }
};
        locals.var_nover_func = assign90540_e139287;
        locals.var_nover_func_rv = 0.0;

        let (assign90550_e139293, assign90550_e139293_d_n0, assign90550_e139293_d_n2, assign90550_e139293_d_n4, assign90550_e139293_d_n5, assign90550_e139293_d_n6, assign90550_e139293_d_n7, assign90550_e139293_d_n8, assign90550_e139293_d_n9, assign90550_e139293_d_n10, assign90550_e139293_d_n13,) = {
    if ((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_lover_func, locals.var_lover_func_dn0, locals.var_lover_func_dn2, locals.var_lover_func_dn4, locals.var_lover_func_dn5, locals.var_lover_func_dn6, locals.var_lover_func_dn7, locals.var_lover_func_dn8, locals.var_lover_func_dn9, locals.var_lover_func_dn10, locals.var_lover_func_dn13,)
    }
};
        locals.var_lover_func = assign90550_e139293;
        locals.var_lover_func_dn0 = assign90550_e139293_d_n0;
        locals.var_lover_func_dn2 = assign90550_e139293_d_n2;
        locals.var_lover_func_dn4 = assign90550_e139293_d_n4;
        locals.var_lover_func_dn5 = assign90550_e139293_d_n5;
        locals.var_lover_func_dn6 = assign90550_e139293_d_n6;
        locals.var_lover_func_dn7 = assign90550_e139293_d_n7;
        locals.var_lover_func_dn8 = assign90550_e139293_d_n8;
        locals.var_lover_func_dn9 = assign90550_e139293_d_n9;
        locals.var_lover_func_dn10 = assign90550_e139293_d_n10;
        locals.var_lover_func_dn13 = assign90550_e139293_d_n13;
        locals.var_lover_func_rv = 0.0;

        let (assign90560_e139299, assign90560_e139299_d_n0, assign90560_e139299_d_n2, assign90560_e139299_d_n4, assign90560_e139299_d_n5, assign90560_e139299_d_n6, assign90560_e139299_d_n7, assign90560_e139299_d_n8, assign90560_e139299_d_n9, assign90560_e139299_d_n10, assign90560_e139299_d_n13,) = {
    if ((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) {
        (locals.var_wdep, locals.var_wdep_dn0, locals.var_wdep_dn2, locals.var_wdep_dn4, locals.var_wdep_dn5, locals.var_wdep_dn6, locals.var_wdep_dn7, locals.var_wdep_dn8, locals.var_wdep_dn9, locals.var_wdep_dn10, locals.var_wdep_dn13,)
    } else {
        (locals.var_wdep_func, locals.var_wdep_func_dn0, locals.var_wdep_func_dn2, locals.var_wdep_func_dn4, locals.var_wdep_func_dn5, locals.var_wdep_func_dn6, locals.var_wdep_func_dn7, locals.var_wdep_func_dn8, locals.var_wdep_func_dn9, locals.var_wdep_func_dn10, locals.var_wdep_func_dn13,)
    }
};
        locals.var_wdep_func = assign90560_e139299;
        locals.var_wdep_func_dn0 = assign90560_e139299_d_n0;
        locals.var_wdep_func_dn2 = assign90560_e139299_d_n2;
        locals.var_wdep_func_dn4 = assign90560_e139299_d_n4;
        locals.var_wdep_func_dn5 = assign90560_e139299_d_n5;
        locals.var_wdep_func_dn6 = assign90560_e139299_d_n6;
        locals.var_wdep_func_dn7 = assign90560_e139299_d_n7;
        locals.var_wdep_func_dn8 = assign90560_e139299_d_n8;
        locals.var_wdep_func_dn9 = assign90560_e139299_d_n9;
        locals.var_wdep_func_dn10 = assign90560_e139299_d_n10;
        locals.var_wdep_func_dn13 = assign90560_e139299_d_n13;
        locals.var_wdep_func_rv = 0.0;

        let (assign90570_e139305, assign90570_e139305_d_n0, assign90570_e139305_d_n2, assign90570_e139305_d_n4, assign90570_e139305_d_n5, assign90570_e139305_d_n6, assign90570_e139305_d_n7, assign90570_e139305_d_n8, assign90570_e139305_d_n9, assign90570_e139305_d_n10, assign90570_e139305_d_n13,) = {
    if ((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) {
        (locals.var_cnst0over, locals.var_cnst0over_dn0, locals.var_cnst0over_dn2, locals.var_cnst0over_dn4, locals.var_cnst0over_dn5, locals.var_cnst0over_dn6, locals.var_cnst0over_dn7, locals.var_cnst0over_dn8, locals.var_cnst0over_dn9, locals.var_cnst0over_dn10, locals.var_cnst0over_dn13,)
    } else {
        (locals.var_cnst0over_func, locals.var_cnst0over_func_dn0, locals.var_cnst0over_func_dn2, locals.var_cnst0over_func_dn4, locals.var_cnst0over_func_dn5, locals.var_cnst0over_func_dn6, locals.var_cnst0over_func_dn7, locals.var_cnst0over_func_dn8, locals.var_cnst0over_func_dn9, locals.var_cnst0over_func_dn10, locals.var_cnst0over_func_dn13,)
    }
};
        locals.var_cnst0over_func = assign90570_e139305;
        locals.var_cnst0over_func_dn0 = assign90570_e139305_d_n0;
        locals.var_cnst0over_func_dn2 = assign90570_e139305_d_n2;
        locals.var_cnst0over_func_dn4 = assign90570_e139305_d_n4;
        locals.var_cnst0over_func_dn5 = assign90570_e139305_d_n5;
        locals.var_cnst0over_func_dn6 = assign90570_e139305_d_n6;
        locals.var_cnst0over_func_dn7 = assign90570_e139305_d_n7;
        locals.var_cnst0over_func_dn8 = assign90570_e139305_d_n8;
        locals.var_cnst0over_func_dn9 = assign90570_e139305_d_n9;
        locals.var_cnst0over_func_dn10 = assign90570_e139305_d_n10;
        locals.var_cnst0over_func_dn13 = assign90570_e139305_d_n13;
        locals.var_cnst0over_func_rv = 0.0;

        let (assign90580_e139311,) = {
    if ((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) {
        (locals.var_coxb0,)
    } else {
        (locals.var_cox0_func,)
    }
};
        locals.var_cox0_func = assign90580_e139311;
        locals.var_cox0_func_rv = 0.0;

        let (assign90590_e139317, assign90590_e139317_d_n0, assign90590_e139317_d_n2, assign90590_e139317_d_n4, assign90590_e139317_d_n5, assign90590_e139317_d_n6, assign90590_e139317_d_n7, assign90590_e139317_d_n8, assign90590_e139317_d_n9, assign90590_e139317_d_n10, assign90590_e139317_d_n13,) = {
    if ((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) {
        (0.4, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vbs_bnd_over__blk2120, locals.var_vbs_bnd_over__blk2120_dn0, locals.var_vbs_bnd_over__blk2120_dn2, locals.var_vbs_bnd_over__blk2120_dn4, locals.var_vbs_bnd_over__blk2120_dn5, locals.var_vbs_bnd_over__blk2120_dn6, locals.var_vbs_bnd_over__blk2120_dn7, locals.var_vbs_bnd_over__blk2120_dn8, locals.var_vbs_bnd_over__blk2120_dn9, locals.var_vbs_bnd_over__blk2120_dn10, locals.var_vbs_bnd_over__blk2120_dn13,)
    }
};
        locals.var_vbs_bnd_over__blk2120 = assign90590_e139317;
        locals.var_vbs_bnd_over__blk2120_dn0 = assign90590_e139317_d_n0;
        locals.var_vbs_bnd_over__blk2120_dn2 = assign90590_e139317_d_n2;
        locals.var_vbs_bnd_over__blk2120_dn4 = assign90590_e139317_d_n4;
        locals.var_vbs_bnd_over__blk2120_dn5 = assign90590_e139317_d_n5;
        locals.var_vbs_bnd_over__blk2120_dn6 = assign90590_e139317_d_n6;
        locals.var_vbs_bnd_over__blk2120_dn7 = assign90590_e139317_d_n7;
        locals.var_vbs_bnd_over__blk2120_dn8 = assign90590_e139317_d_n8;
        locals.var_vbs_bnd_over__blk2120_dn9 = assign90590_e139317_d_n9;
        locals.var_vbs_bnd_over__blk2120_dn10 = assign90590_e139317_d_n10;
        locals.var_vbs_bnd_over__blk2120_dn13 = assign90590_e139317_d_n13;
        locals.var_vbs_bnd_over__blk2120_rv = 0.0;

        let (assign90610_e139329,) = {
    if ((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_flg_fd_mode__blk2121,)
    }
};
        locals.var_flg_fd_mode__blk2121 = assign90610_e139329;
        locals.var_flg_fd_mode__blk2121_rv = 0.0;

        let (assign90620_e139335, assign90620_e139335_d_n0, assign90620_e139335_d_n2, assign90620_e139335_d_n4, assign90620_e139335_d_n5, assign90620_e139335_d_n6, assign90620_e139335_d_n7, assign90620_e139335_d_n8, assign90620_e139335_d_n9, assign90620_e139335_d_n10, assign90620_e139335_d_n13,) = {
    if ((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn4, locals.var_fb_dn5, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn8, locals.var_fb_dn9, locals.var_fb_dn10, locals.var_fb_dn13,)
    }
};
        locals.var_fb = assign90620_e139335;
        locals.var_fb_dn0 = assign90620_e139335_d_n0;
        locals.var_fb_dn2 = assign90620_e139335_d_n2;
        locals.var_fb_dn4 = assign90620_e139335_d_n4;
        locals.var_fb_dn5 = assign90620_e139335_d_n5;
        locals.var_fb_dn6 = assign90620_e139335_d_n6;
        locals.var_fb_dn7 = assign90620_e139335_d_n7;
        locals.var_fb_dn8 = assign90620_e139335_d_n8;
        locals.var_fb_dn9 = assign90620_e139335_d_n9;
        locals.var_fb_dn10 = assign90620_e139335_d_n10;
        locals.var_fb_dn13 = assign90620_e139335_d_n13;
        locals.var_fb_rv = 0.0;

        let (assign90630_e139341, assign90630_e139341_d_n0, assign90630_e139341_d_n2, assign90630_e139341_d_n4, assign90630_e139341_d_n5, assign90630_e139341_d_n6, assign90630_e139341_d_n7, assign90630_e139341_d_n8, assign90630_e139341_d_n9, assign90630_e139341_d_n10, assign90630_e139341_d_n13,) = {
    if ((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fs01, locals.var_fs01_dn0, locals.var_fs01_dn2, locals.var_fs01_dn4, locals.var_fs01_dn5, locals.var_fs01_dn6, locals.var_fs01_dn7, locals.var_fs01_dn8, locals.var_fs01_dn9, locals.var_fs01_dn10, locals.var_fs01_dn13,)
    }
};
        locals.var_fs01 = assign90630_e139341;
        locals.var_fs01_dn0 = assign90630_e139341_d_n0;
        locals.var_fs01_dn2 = assign90630_e139341_d_n2;
        locals.var_fs01_dn4 = assign90630_e139341_d_n4;
        locals.var_fs01_dn5 = assign90630_e139341_d_n5;
        locals.var_fs01_dn6 = assign90630_e139341_d_n6;
        locals.var_fs01_dn7 = assign90630_e139341_d_n7;
        locals.var_fs01_dn8 = assign90630_e139341_d_n8;
        locals.var_fs01_dn9 = assign90630_e139341_d_n9;
        locals.var_fs01_dn10 = assign90630_e139341_d_n10;
        locals.var_fs01_dn13 = assign90630_e139341_d_n13;
        locals.var_fs01_rv = 0.0;

        let (assign90640_e139347, assign90640_e139347_d_n0, assign90640_e139347_d_n2, assign90640_e139347_d_n4, assign90640_e139347_d_n5, assign90640_e139347_d_n6, assign90640_e139347_d_n7, assign90640_e139347_d_n8, assign90640_e139347_d_n9, assign90640_e139347_d_n10, assign90640_e139347_d_n13,) = {
    if ((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fs02, locals.var_fs02_dn0, locals.var_fs02_dn2, locals.var_fs02_dn4, locals.var_fs02_dn5, locals.var_fs02_dn6, locals.var_fs02_dn7, locals.var_fs02_dn8, locals.var_fs02_dn9, locals.var_fs02_dn10, locals.var_fs02_dn13,)
    }
};
        locals.var_fs02 = assign90640_e139347;
        locals.var_fs02_dn0 = assign90640_e139347_d_n0;
        locals.var_fs02_dn2 = assign90640_e139347_d_n2;
        locals.var_fs02_dn4 = assign90640_e139347_d_n4;
        locals.var_fs02_dn5 = assign90640_e139347_d_n5;
        locals.var_fs02_dn6 = assign90640_e139347_d_n6;
        locals.var_fs02_dn7 = assign90640_e139347_d_n7;
        locals.var_fs02_dn8 = assign90640_e139347_d_n8;
        locals.var_fs02_dn9 = assign90640_e139347_d_n9;
        locals.var_fs02_dn10 = assign90640_e139347_d_n10;
        locals.var_fs02_dn13 = assign90640_e139347_d_n13;
        locals.var_fs02_rv = 0.0;

        let (assign90650_e139353, assign90650_e139353_d_n0, assign90650_e139353_d_n2, assign90650_e139353_d_n4, assign90650_e139353_d_n5, assign90650_e139353_d_n6, assign90650_e139353_d_n7, assign90650_e139353_d_n8, assign90650_e139353_d_n9, assign90650_e139353_d_n10, assign90650_e139353_d_n13,) = {
    if ((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fs0, locals.var_fs0_dn0, locals.var_fs0_dn2, locals.var_fs0_dn4, locals.var_fs0_dn5, locals.var_fs0_dn6, locals.var_fs0_dn7, locals.var_fs0_dn8, locals.var_fs0_dn9, locals.var_fs0_dn10, locals.var_fs0_dn13,)
    }
};
        locals.var_fs0 = assign90650_e139353;
        locals.var_fs0_dn0 = assign90650_e139353_d_n0;
        locals.var_fs0_dn2 = assign90650_e139353_d_n2;
        locals.var_fs0_dn4 = assign90650_e139353_d_n4;
        locals.var_fs0_dn5 = assign90650_e139353_d_n5;
        locals.var_fs0_dn6 = assign90650_e139353_d_n6;
        locals.var_fs0_dn7 = assign90650_e139353_d_n7;
        locals.var_fs0_dn8 = assign90650_e139353_d_n8;
        locals.var_fs0_dn9 = assign90650_e139353_d_n9;
        locals.var_fs0_dn10 = assign90650_e139353_d_n10;
        locals.var_fs0_dn13 = assign90650_e139353_d_n13;
        locals.var_fs0_rv = 0.0;

        let (assign90660_e139359, assign90660_e139359_d_n0, assign90660_e139359_d_n2, assign90660_e139359_d_n4, assign90660_e139359_d_n5, assign90660_e139359_d_n6, assign90660_e139359_d_n7, assign90660_e139359_d_n8, assign90660_e139359_d_n9, assign90660_e139359_d_n10, assign90660_e139359_d_n13,) = {
    if ((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dps0, locals.var_dps0_dn0, locals.var_dps0_dn2, locals.var_dps0_dn4, locals.var_dps0_dn5, locals.var_dps0_dn6, locals.var_dps0_dn7, locals.var_dps0_dn8, locals.var_dps0_dn9, locals.var_dps0_dn10, locals.var_dps0_dn13,)
    }
};
        locals.var_dps0 = assign90660_e139359;
        locals.var_dps0_dn0 = assign90660_e139359_d_n0;
        locals.var_dps0_dn2 = assign90660_e139359_d_n2;
        locals.var_dps0_dn4 = assign90660_e139359_d_n4;
        locals.var_dps0_dn5 = assign90660_e139359_d_n5;
        locals.var_dps0_dn6 = assign90660_e139359_d_n6;
        locals.var_dps0_dn7 = assign90660_e139359_d_n7;
        locals.var_dps0_dn8 = assign90660_e139359_d_n8;
        locals.var_dps0_dn9 = assign90660_e139359_d_n9;
        locals.var_dps0_dn10 = assign90660_e139359_d_n10;
        locals.var_dps0_dn13 = assign90660_e139359_d_n13;
        locals.var_dps0_rv = 0.0;

        let (assign90670_e139365, assign90670_e139365_d_n0, assign90670_e139365_d_n2, assign90670_e139365_d_n4, assign90670_e139365_d_n5, assign90670_e139365_d_n6, assign90670_e139365_d_n7, assign90670_e139365_d_n8, assign90670_e139365_d_n9, assign90670_e139365_d_n10, assign90670_e139365_d_n13,) = {
    if ((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fs0_dps0, locals.var_fs0_dps0_dn0, locals.var_fs0_dps0_dn2, locals.var_fs0_dps0_dn4, locals.var_fs0_dps0_dn5, locals.var_fs0_dps0_dn6, locals.var_fs0_dps0_dn7, locals.var_fs0_dps0_dn8, locals.var_fs0_dps0_dn9, locals.var_fs0_dps0_dn10, locals.var_fs0_dps0_dn13,)
    }
};
        locals.var_fs0_dps0 = assign90670_e139365;
        locals.var_fs0_dps0_dn0 = assign90670_e139365_d_n0;
        locals.var_fs0_dps0_dn2 = assign90670_e139365_d_n2;
        locals.var_fs0_dps0_dn4 = assign90670_e139365_d_n4;
        locals.var_fs0_dps0_dn5 = assign90670_e139365_d_n5;
        locals.var_fs0_dps0_dn6 = assign90670_e139365_d_n6;
        locals.var_fs0_dps0_dn7 = assign90670_e139365_d_n7;
        locals.var_fs0_dps0_dn8 = assign90670_e139365_d_n8;
        locals.var_fs0_dps0_dn9 = assign90670_e139365_d_n9;
        locals.var_fs0_dps0_dn10 = assign90670_e139365_d_n10;
        locals.var_fs0_dps0_dn13 = assign90670_e139365_d_n13;
        locals.var_fs0_dps0_rv = 0.0;

        let (assign90680_e139371, assign90680_e139371_d_n0, assign90680_e139371_d_n2, assign90680_e139371_d_n4, assign90680_e139371_d_n5, assign90680_e139371_d_n6, assign90680_e139371_d_n7, assign90680_e139371_d_n8, assign90680_e139371_d_n9, assign90680_e139371_d_n10, assign90680_e139371_d_n13,) = {
    if ((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fs02_dps0, locals.var_fs02_dps0_dn0, locals.var_fs02_dps0_dn2, locals.var_fs02_dps0_dn4, locals.var_fs02_dps0_dn5, locals.var_fs02_dps0_dn6, locals.var_fs02_dps0_dn7, locals.var_fs02_dps0_dn8, locals.var_fs02_dps0_dn9, locals.var_fs02_dps0_dn10, locals.var_fs02_dps0_dn13,)
    }
};
        locals.var_fs02_dps0 = assign90680_e139371;
        locals.var_fs02_dps0_dn0 = assign90680_e139371_d_n0;
        locals.var_fs02_dps0_dn2 = assign90680_e139371_d_n2;
        locals.var_fs02_dps0_dn4 = assign90680_e139371_d_n4;
        locals.var_fs02_dps0_dn5 = assign90680_e139371_d_n5;
        locals.var_fs02_dps0_dn6 = assign90680_e139371_d_n6;
        locals.var_fs02_dps0_dn7 = assign90680_e139371_d_n7;
        locals.var_fs02_dps0_dn8 = assign90680_e139371_d_n8;
        locals.var_fs02_dps0_dn9 = assign90680_e139371_d_n9;
        locals.var_fs02_dps0_dn10 = assign90680_e139371_d_n10;
        locals.var_fs02_dps0_dn13 = assign90680_e139371_d_n13;
        locals.var_fs02_dps0_rv = 0.0;

        let (assign90690_e139377, assign90690_e139377_d_n0, assign90690_e139377_d_n2, assign90690_e139377_d_n4, assign90690_e139377_d_n5, assign90690_e139377_d_n6, assign90690_e139377_d_n7, assign90690_e139377_d_n8, assign90690_e139377_d_n9, assign90690_e139377_d_n10, assign90690_e139377_d_n13,) = {
    if ((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fb_dpss, locals.var_fb_dpss_dn0, locals.var_fb_dpss_dn2, locals.var_fb_dpss_dn4, locals.var_fb_dpss_dn5, locals.var_fb_dpss_dn6, locals.var_fb_dpss_dn7, locals.var_fb_dpss_dn8, locals.var_fb_dpss_dn9, locals.var_fb_dpss_dn10, locals.var_fb_dpss_dn13,)
    }
};
        locals.var_fb_dpss = assign90690_e139377;
        locals.var_fb_dpss_dn0 = assign90690_e139377_d_n0;
        locals.var_fb_dpss_dn2 = assign90690_e139377_d_n2;
        locals.var_fb_dpss_dn4 = assign90690_e139377_d_n4;
        locals.var_fb_dpss_dn5 = assign90690_e139377_d_n5;
        locals.var_fb_dpss_dn6 = assign90690_e139377_d_n6;
        locals.var_fb_dpss_dn7 = assign90690_e139377_d_n7;
        locals.var_fb_dpss_dn8 = assign90690_e139377_d_n8;
        locals.var_fb_dpss_dn9 = assign90690_e139377_d_n9;
        locals.var_fb_dpss_dn10 = assign90690_e139377_d_n10;
        locals.var_fb_dpss_dn13 = assign90690_e139377_d_n13;
        locals.var_fb_dpss_rv = 0.0;

        let (assign90700_e139383, assign90700_e139383_d_n0, assign90700_e139383_d_n2, assign90700_e139383_d_n4, assign90700_e139383_d_n5, assign90700_e139383_d_n6, assign90700_e139383_d_n7, assign90700_e139383_d_n8, assign90700_e139383_d_n9, assign90700_e139383_d_n10, assign90700_e139383_d_n13,) = {
    if ((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fs01_dps0, locals.var_fs01_dps0_dn0, locals.var_fs01_dps0_dn2, locals.var_fs01_dps0_dn4, locals.var_fs01_dps0_dn5, locals.var_fs01_dps0_dn6, locals.var_fs01_dps0_dn7, locals.var_fs01_dps0_dn8, locals.var_fs01_dps0_dn9, locals.var_fs01_dps0_dn10, locals.var_fs01_dps0_dn13,)
    }
};
        locals.var_fs01_dps0 = assign90700_e139383;
        locals.var_fs01_dps0_dn0 = assign90700_e139383_d_n0;
        locals.var_fs01_dps0_dn2 = assign90700_e139383_d_n2;
        locals.var_fs01_dps0_dn4 = assign90700_e139383_d_n4;
        locals.var_fs01_dps0_dn5 = assign90700_e139383_d_n5;
        locals.var_fs01_dps0_dn6 = assign90700_e139383_d_n6;
        locals.var_fs01_dps0_dn7 = assign90700_e139383_d_n7;
        locals.var_fs01_dps0_dn8 = assign90700_e139383_d_n8;
        locals.var_fs01_dps0_dn9 = assign90700_e139383_d_n9;
        locals.var_fs01_dps0_dn10 = assign90700_e139383_d_n10;
        locals.var_fs01_dps0_dn13 = assign90700_e139383_d_n13;
        locals.var_fs01_dps0_rv = 0.0;

        let (assign90710_e139389, assign90710_e139389_d_n0, assign90710_e139389_d_n2, assign90710_e139389_d_n4, assign90710_e139389_d_n5, assign90710_e139389_d_n6, assign90710_e139389_d_n7, assign90710_e139389_d_n8, assign90710_e139389_d_n9, assign90710_e139389_d_n10, assign90710_e139389_d_n13,) = {
    if ((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_chi_1, locals.var_chi_1_dn0, locals.var_chi_1_dn2, locals.var_chi_1_dn4, locals.var_chi_1_dn5, locals.var_chi_1_dn6, locals.var_chi_1_dn7, locals.var_chi_1_dn8, locals.var_chi_1_dn9, locals.var_chi_1_dn10, locals.var_chi_1_dn13,)
    }
};
        locals.var_chi_1 = assign90710_e139389;
        locals.var_chi_1_dn0 = assign90710_e139389_d_n0;
        locals.var_chi_1_dn2 = assign90710_e139389_d_n2;
        locals.var_chi_1_dn4 = assign90710_e139389_d_n4;
        locals.var_chi_1_dn5 = assign90710_e139389_d_n5;
        locals.var_chi_1_dn6 = assign90710_e139389_d_n6;
        locals.var_chi_1_dn7 = assign90710_e139389_d_n7;
        locals.var_chi_1_dn8 = assign90710_e139389_d_n8;
        locals.var_chi_1_dn9 = assign90710_e139389_d_n9;
        locals.var_chi_1_dn10 = assign90710_e139389_d_n10;
        locals.var_chi_1_dn13 = assign90710_e139389_d_n13;
        locals.var_chi_1_rv = 0.0;

        let (assign90720_e139395, assign90720_e139395_d_n0, assign90720_e139395_d_n2, assign90720_e139395_d_n4, assign90720_e139395_d_n5, assign90720_e139395_d_n6, assign90720_e139395_d_n7, assign90720_e139395_d_n8, assign90720_e139395_d_n9, assign90720_e139395_d_n10, assign90720_e139395_d_n13,) = {
    if ((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_chi_a, locals.var_chi_a_dn0, locals.var_chi_a_dn2, locals.var_chi_a_dn4, locals.var_chi_a_dn5, locals.var_chi_a_dn6, locals.var_chi_a_dn7, locals.var_chi_a_dn8, locals.var_chi_a_dn9, locals.var_chi_a_dn10, locals.var_chi_a_dn13,)
    }
};
        locals.var_chi_a = assign90720_e139395;
        locals.var_chi_a_dn0 = assign90720_e139395_d_n0;
        locals.var_chi_a_dn2 = assign90720_e139395_d_n2;
        locals.var_chi_a_dn4 = assign90720_e139395_d_n4;
        locals.var_chi_a_dn5 = assign90720_e139395_d_n5;
        locals.var_chi_a_dn6 = assign90720_e139395_d_n6;
        locals.var_chi_a_dn7 = assign90720_e139395_d_n7;
        locals.var_chi_a_dn8 = assign90720_e139395_d_n8;
        locals.var_chi_a_dn9 = assign90720_e139395_d_n9;
        locals.var_chi_a_dn10 = assign90720_e139395_d_n10;
        locals.var_chi_a_dn13 = assign90720_e139395_d_n13;
        locals.var_chi_a_rv = 0.0;

        let (assign90730_e139401, assign90730_e139401_d_n0, assign90730_e139401_d_n2, assign90730_e139401_d_n4, assign90730_e139401_d_n5, assign90730_e139401_d_n6, assign90730_e139401_d_n7, assign90730_e139401_d_n8, assign90730_e139401_d_n9, assign90730_e139401_d_n10, assign90730_e139401_d_n13,) = {
    if ((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_chi_b, locals.var_chi_b_dn0, locals.var_chi_b_dn2, locals.var_chi_b_dn4, locals.var_chi_b_dn5, locals.var_chi_b_dn6, locals.var_chi_b_dn7, locals.var_chi_b_dn8, locals.var_chi_b_dn9, locals.var_chi_b_dn10, locals.var_chi_b_dn13,)
    }
};
        locals.var_chi_b = assign90730_e139401;
        locals.var_chi_b_dn0 = assign90730_e139401_d_n0;
        locals.var_chi_b_dn2 = assign90730_e139401_d_n2;
        locals.var_chi_b_dn4 = assign90730_e139401_d_n4;
        locals.var_chi_b_dn5 = assign90730_e139401_d_n5;
        locals.var_chi_b_dn6 = assign90730_e139401_d_n6;
        locals.var_chi_b_dn7 = assign90730_e139401_d_n7;
        locals.var_chi_b_dn8 = assign90730_e139401_d_n8;
        locals.var_chi_b_dn9 = assign90730_e139401_d_n9;
        locals.var_chi_b_dn10 = assign90730_e139401_d_n10;
        locals.var_chi_b_dn13 = assign90730_e139401_d_n13;
        locals.var_chi_b_rv = 0.0;

        let (assign90740_e139408,) = {
    if ((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) {
        let assign90740_e139406: f64 = (-1.0);
        (assign90740_e139406,)
    } else {
        (locals.var_flg_conv,)
    }
};
        locals.var_flg_conv = assign90740_e139408;
        locals.var_flg_conv_rv = 0.0;

        let (assign90750_e139414, assign90750_e139414_d_n0, assign90750_e139414_d_n2, assign90750_e139414_d_n4, assign90750_e139414_d_n5, assign90750_e139414_d_n6, assign90750_e139414_d_n7, assign90750_e139414_d_n8, assign90750_e139414_d_n9, assign90750_e139414_d_n10, assign90750_e139414_d_n13,) = {
    if ((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ps0ld_ini__blk2122, locals.var_ps0ld_ini__blk2122_dn0, locals.var_ps0ld_ini__blk2122_dn2, locals.var_ps0ld_ini__blk2122_dn4, locals.var_ps0ld_ini__blk2122_dn5, locals.var_ps0ld_ini__blk2122_dn6, locals.var_ps0ld_ini__blk2122_dn7, locals.var_ps0ld_ini__blk2122_dn8, locals.var_ps0ld_ini__blk2122_dn9, locals.var_ps0ld_ini__blk2122_dn10, locals.var_ps0ld_ini__blk2122_dn13,)
    }
};
        locals.var_ps0ld_ini__blk2122 = assign90750_e139414;
        locals.var_ps0ld_ini__blk2122_dn0 = assign90750_e139414_d_n0;
        locals.var_ps0ld_ini__blk2122_dn2 = assign90750_e139414_d_n2;
        locals.var_ps0ld_ini__blk2122_dn4 = assign90750_e139414_d_n4;
        locals.var_ps0ld_ini__blk2122_dn5 = assign90750_e139414_d_n5;
        locals.var_ps0ld_ini__blk2122_dn6 = assign90750_e139414_d_n6;
        locals.var_ps0ld_ini__blk2122_dn7 = assign90750_e139414_d_n7;
        locals.var_ps0ld_ini__blk2122_dn8 = assign90750_e139414_d_n8;
        locals.var_ps0ld_ini__blk2122_dn9 = assign90750_e139414_d_n9;
        locals.var_ps0ld_ini__blk2122_dn10 = assign90750_e139414_d_n10;
        locals.var_ps0ld_ini__blk2122_dn13 = assign90750_e139414_d_n13;
        locals.var_ps0ld_ini__blk2122_rv = 0.0;

        let (assign90760_e139420, assign90760_e139420_d_n0, assign90760_e139420_d_n2, assign90760_e139420_d_n4, assign90760_e139420_d_n5, assign90760_e139420_d_n6, assign90760_e139420_d_n7, assign90760_e139420_d_n8, assign90760_e139420_d_n9, assign90760_e139420_d_n10, assign90760_e139420_d_n13,) = {
    if ((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fbsq__blk2123, locals.var_fbsq__blk2123_dn0, locals.var_fbsq__blk2123_dn2, locals.var_fbsq__blk2123_dn4, locals.var_fbsq__blk2123_dn5, locals.var_fbsq__blk2123_dn6, locals.var_fbsq__blk2123_dn7, locals.var_fbsq__blk2123_dn8, locals.var_fbsq__blk2123_dn9, locals.var_fbsq__blk2123_dn10, locals.var_fbsq__blk2123_dn13,)
    }
};
        locals.var_fbsq__blk2123 = assign90760_e139420;
        locals.var_fbsq__blk2123_dn0 = assign90760_e139420_d_n0;
        locals.var_fbsq__blk2123_dn2 = assign90760_e139420_d_n2;
        locals.var_fbsq__blk2123_dn4 = assign90760_e139420_d_n4;
        locals.var_fbsq__blk2123_dn5 = assign90760_e139420_d_n5;
        locals.var_fbsq__blk2123_dn6 = assign90760_e139420_d_n6;
        locals.var_fbsq__blk2123_dn7 = assign90760_e139420_d_n7;
        locals.var_fbsq__blk2123_dn8 = assign90760_e139420_d_n8;
        locals.var_fbsq__blk2123_dn9 = assign90760_e139420_d_n9;
        locals.var_fbsq__blk2123_dn10 = assign90760_e139420_d_n10;
        locals.var_fbsq__blk2123_dn13 = assign90760_e139420_d_n13;
        locals.var_fbsq__blk2123_rv = 0.0;

        let (assign90770_e139433, assign90770_e139433_d_n0, assign90770_e139433_d_n2, assign90770_e139433_d_n4, assign90770_e139433_d_n5, assign90770_e139433_d_n6, assign90770_e139433_d_n7, assign90770_e139433_d_n8, assign90770_e139433_d_n9, assign90770_e139433_d_n10, assign90770_e139433_d_n13,) = {
    if ((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) {
        let assign90770_e139426: f64 = (2.0 * locals.var_beta_inv);
        let assign90770_e139429: f64 = (locals.var_nover_func / locals.var_nin);
        let assign90770_e139430: f64 = (assign90770_e139429).ln();
        let assign90770_e139431: f64 = (assign90770_e139426 * assign90770_e139430);
        (assign90770_e139431, (((2.0 * locals.var_beta_inv_dn0) * assign90770_e139430) + (assign90770_e139426 * ((-((locals.var_nover_func * locals.var_nin_dn0) / (locals.var_nin * locals.var_nin))) / assign90770_e139429))), (((2.0 * locals.var_beta_inv_dn2) * assign90770_e139430) + (assign90770_e139426 * ((-((locals.var_nover_func * locals.var_nin_dn2) / (locals.var_nin * locals.var_nin))) / assign90770_e139429))), (((2.0 * locals.var_beta_inv_dn4) * assign90770_e139430) + (assign90770_e139426 * ((-((locals.var_nover_func * locals.var_nin_dn4) / (locals.var_nin * locals.var_nin))) / assign90770_e139429))), (((2.0 * locals.var_beta_inv_dn5) * assign90770_e139430) + (assign90770_e139426 * ((-((locals.var_nover_func * locals.var_nin_dn5) / (locals.var_nin * locals.var_nin))) / assign90770_e139429))), (((2.0 * locals.var_beta_inv_dn6) * assign90770_e139430) + (assign90770_e139426 * ((-((locals.var_nover_func * locals.var_nin_dn6) / (locals.var_nin * locals.var_nin))) / assign90770_e139429))), (((2.0 * locals.var_beta_inv_dn7) * assign90770_e139430) + (assign90770_e139426 * ((-((locals.var_nover_func * locals.var_nin_dn7) / (locals.var_nin * locals.var_nin))) / assign90770_e139429))), (((2.0 * locals.var_beta_inv_dn8) * assign90770_e139430) + (assign90770_e139426 * ((-((locals.var_nover_func * locals.var_nin_dn8) / (locals.var_nin * locals.var_nin))) / assign90770_e139429))), (((2.0 * locals.var_beta_inv_dn9) * assign90770_e139430) + (assign90770_e139426 * ((-((locals.var_nover_func * locals.var_nin_dn9) / (locals.var_nin * locals.var_nin))) / assign90770_e139429))), (((2.0 * locals.var_beta_inv_dn10) * assign90770_e139430) + (assign90770_e139426 * ((-((locals.var_nover_func * locals.var_nin_dn10) / (locals.var_nin * locals.var_nin))) / assign90770_e139429))), (((2.0 * locals.var_beta_inv_dn13) * assign90770_e139430) + (assign90770_e139426 * ((-((locals.var_nover_func * locals.var_nin_dn13) / (locals.var_nin * locals.var_nin))) / assign90770_e139429))),)
    } else {
        (locals.var_pb2over__blk2118, locals.var_pb2over__blk2118_dn0, locals.var_pb2over__blk2118_dn2, locals.var_pb2over__blk2118_dn4, locals.var_pb2over__blk2118_dn5, locals.var_pb2over__blk2118_dn6, locals.var_pb2over__blk2118_dn7, locals.var_pb2over__blk2118_dn8, locals.var_pb2over__blk2118_dn9, locals.var_pb2over__blk2118_dn10, locals.var_pb2over__blk2118_dn13,)
    }
};
        locals.var_pb2over__blk2118 = assign90770_e139433;
        locals.var_pb2over__blk2118_dn0 = assign90770_e139433_d_n0;
        locals.var_pb2over__blk2118_dn2 = assign90770_e139433_d_n2;
        locals.var_pb2over__blk2118_dn4 = assign90770_e139433_d_n4;
        locals.var_pb2over__blk2118_dn5 = assign90770_e139433_d_n5;
        locals.var_pb2over__blk2118_dn6 = assign90770_e139433_d_n6;
        locals.var_pb2over__blk2118_dn7 = assign90770_e139433_d_n7;
        locals.var_pb2over__blk2118_dn8 = assign90770_e139433_d_n8;
        locals.var_pb2over__blk2118_dn9 = assign90770_e139433_d_n9;
        locals.var_pb2over__blk2118_dn10 = assign90770_e139433_d_n10;
        locals.var_pb2over__blk2118_dn13 = assign90770_e139433_d_n13;
        locals.var_pb2over__blk2118_rv = 0.0;

        let (assign90780_e139443, assign90780_e139443_d_n0, assign90780_e139443_d_n2, assign90780_e139443_d_n4, assign90780_e139443_d_n5, assign90780_e139443_d_n6, assign90780_e139443_d_n7, assign90780_e139443_d_n8, assign90780_e139443_d_n9, assign90780_e139443_d_n10, assign90780_e139443_d_n13,) = {
    if ((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) {
        let assign90780_e139439: f64 = (0.8 - locals.var_pb2over__blk2118);
        let assign90780_e139441: f64 = (assign90780_e139439 - 0.1);
        (assign90780_e139441, (-locals.var_pb2over__blk2118_dn0), (-locals.var_pb2over__blk2118_dn2), (-locals.var_pb2over__blk2118_dn4), (-locals.var_pb2over__blk2118_dn5), (-locals.var_pb2over__blk2118_dn6), (-locals.var_pb2over__blk2118_dn7), (-locals.var_pb2over__blk2118_dn8), (-locals.var_pb2over__blk2118_dn9), (-locals.var_pb2over__blk2118_dn10), (-locals.var_pb2over__blk2118_dn13),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign90780_e139443;
        locals.var_tmf1_dn0 = assign90780_e139443_d_n0;
        locals.var_tmf1_dn2 = assign90780_e139443_d_n2;
        locals.var_tmf1_dn4 = assign90780_e139443_d_n4;
        locals.var_tmf1_dn5 = assign90780_e139443_d_n5;
        locals.var_tmf1_dn6 = assign90780_e139443_d_n6;
        locals.var_tmf1_dn7 = assign90780_e139443_d_n7;
        locals.var_tmf1_dn8 = assign90780_e139443_d_n8;
        locals.var_tmf1_dn9 = assign90780_e139443_d_n9;
        locals.var_tmf1_dn10 = assign90780_e139443_d_n10;
        locals.var_tmf1_dn13 = assign90780_e139443_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign90790_e139453, assign90790_e139453_d_n0, assign90790_e139453_d_n2, assign90790_e139453_d_n4, assign90790_e139453_d_n5, assign90790_e139453_d_n6, assign90790_e139453_d_n7, assign90790_e139453_d_n8, assign90790_e139453_d_n9, assign90790_e139453_d_n10, assign90790_e139453_d_n13,) = {
    if ((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) {
        let assign90790_e139449: f64 = (4.0 * 0.8);
        let assign90790_e139451: f64 = (assign90790_e139449 * 0.1);
        (assign90790_e139451, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign90790_e139453;
        locals.var_tmf2_dn0 = assign90790_e139453_d_n0;
        locals.var_tmf2_dn2 = assign90790_e139453_d_n2;
        locals.var_tmf2_dn4 = assign90790_e139453_d_n4;
        locals.var_tmf2_dn5 = assign90790_e139453_d_n5;
        locals.var_tmf2_dn6 = assign90790_e139453_d_n6;
        locals.var_tmf2_dn7 = assign90790_e139453_d_n7;
        locals.var_tmf2_dn8 = assign90790_e139453_d_n8;
        locals.var_tmf2_dn9 = assign90790_e139453_d_n9;
        locals.var_tmf2_dn10 = assign90790_e139453_d_n10;
        locals.var_tmf2_dn13 = assign90790_e139453_d_n13;
        locals.var_tmf2_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_339(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        locals: &mut StampLocals,
    ) {
        let (assign90800_e139465, assign90800_e139465_d_n0, assign90800_e139465_d_n2, assign90800_e139465_d_n4, assign90800_e139465_d_n5, assign90800_e139465_d_n6, assign90800_e139465_d_n7, assign90800_e139465_d_n8, assign90800_e139465_d_n9, assign90800_e139465_d_n10, assign90800_e139465_d_n13,) = {
    if ((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) {
        let (assign90800_e139463, assign90800_e139463_d_n0, assign90800_e139463_d_n2, assign90800_e139463_d_n4, assign90800_e139463_d_n5, assign90800_e139463_d_n6, assign90800_e139463_d_n7, assign90800_e139463_d_n8, assign90800_e139463_d_n9, assign90800_e139463_d_n10, assign90800_e139463_d_n13,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
            } else {
                let assign90800_e139462: f64 = (-locals.var_tmf2);
                (assign90800_e139462, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn13),)
            }
        };
        (assign90800_e139463, assign90800_e139463_d_n0, assign90800_e139463_d_n2, assign90800_e139463_d_n4, assign90800_e139463_d_n5, assign90800_e139463_d_n6, assign90800_e139463_d_n7, assign90800_e139463_d_n8, assign90800_e139463_d_n9, assign90800_e139463_d_n10, assign90800_e139463_d_n13,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign90800_e139465;
        locals.var_tmf2_dn0 = assign90800_e139465_d_n0;
        locals.var_tmf2_dn2 = assign90800_e139465_d_n2;
        locals.var_tmf2_dn4 = assign90800_e139465_d_n4;
        locals.var_tmf2_dn5 = assign90800_e139465_d_n5;
        locals.var_tmf2_dn6 = assign90800_e139465_d_n6;
        locals.var_tmf2_dn7 = assign90800_e139465_d_n7;
        locals.var_tmf2_dn8 = assign90800_e139465_d_n8;
        locals.var_tmf2_dn9 = assign90800_e139465_d_n9;
        locals.var_tmf2_dn10 = assign90800_e139465_d_n10;
        locals.var_tmf2_dn13 = assign90800_e139465_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign90810_e139476, assign90810_e139476_d_n0, assign90810_e139476_d_n2, assign90810_e139476_d_n4, assign90810_e139476_d_n5, assign90810_e139476_d_n6, assign90810_e139476_d_n7, assign90810_e139476_d_n8, assign90810_e139476_d_n9, assign90810_e139476_d_n10, assign90810_e139476_d_n13,) = {
    if ((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) {
        let assign90810_e139471: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign90810_e139473: f64 = (assign90810_e139471 + locals.var_tmf2);
        let assign90810_e139474: f64 = (assign90810_e139473).sqrt();
        (assign90810_e139474, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign90810_e139474)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign90810_e139474)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign90810_e139474)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign90810_e139474)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign90810_e139474)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign90810_e139474)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign90810_e139474)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign90810_e139474)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign90810_e139474)), ((((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)) + locals.var_tmf2_dn13) / (2.0 * assign90810_e139474)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign90810_e139476;
        locals.var_tmf2_dn0 = assign90810_e139476_d_n0;
        locals.var_tmf2_dn2 = assign90810_e139476_d_n2;
        locals.var_tmf2_dn4 = assign90810_e139476_d_n4;
        locals.var_tmf2_dn5 = assign90810_e139476_d_n5;
        locals.var_tmf2_dn6 = assign90810_e139476_d_n6;
        locals.var_tmf2_dn7 = assign90810_e139476_d_n7;
        locals.var_tmf2_dn8 = assign90810_e139476_d_n8;
        locals.var_tmf2_dn9 = assign90810_e139476_d_n9;
        locals.var_tmf2_dn10 = assign90810_e139476_d_n10;
        locals.var_tmf2_dn13 = assign90810_e139476_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign90820_e139488, assign90820_e139488_d_n0, assign90820_e139488_d_n2, assign90820_e139488_d_n4, assign90820_e139488_d_n5, assign90820_e139488_d_n6, assign90820_e139488_d_n7, assign90820_e139488_d_n8, assign90820_e139488_d_n9, assign90820_e139488_d_n10, assign90820_e139488_d_n13,) = {
    if ((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) {
        let assign90820_e139484: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign90820_e139485: f64 = (1.0 + assign90820_e139484);
        let assign90820_e139486: f64 = (0.5 * assign90820_e139485);
        (assign90820_e139486, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn13 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign90820_e139488;
        locals.var_t0_dn0 = assign90820_e139488_d_n0;
        locals.var_t0_dn2 = assign90820_e139488_d_n2;
        locals.var_t0_dn4 = assign90820_e139488_d_n4;
        locals.var_t0_dn5 = assign90820_e139488_d_n5;
        locals.var_t0_dn6 = assign90820_e139488_d_n6;
        locals.var_t0_dn7 = assign90820_e139488_d_n7;
        locals.var_t0_dn8 = assign90820_e139488_d_n8;
        locals.var_t0_dn9 = assign90820_e139488_d_n9;
        locals.var_t0_dn10 = assign90820_e139488_d_n10;
        locals.var_t0_dn13 = assign90820_e139488_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign90830_e139500, assign90830_e139500_d_n0, assign90830_e139500_d_n2, assign90830_e139500_d_n4, assign90830_e139500_d_n5, assign90830_e139500_d_n6, assign90830_e139500_d_n7, assign90830_e139500_d_n8, assign90830_e139500_d_n9, assign90830_e139500_d_n10, assign90830_e139500_d_n13,) = {
    if ((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) {
        let assign90830_e139496: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign90830_e139497: f64 = (0.5 * assign90830_e139496);
        let assign90830_e139498: f64 = (0.8 - assign90830_e139497);
        (assign90830_e139498, (-(0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (-(0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (-(0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), (-(0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), (-(0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (-(0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (-(0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), (-(0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), (-(0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (-(0.5 * (locals.var_tmf1_dn13 + locals.var_tmf2_dn13))),)
    } else {
        (locals.var_vbs_max_over__blk2119, locals.var_vbs_max_over__blk2119_dn0, locals.var_vbs_max_over__blk2119_dn2, locals.var_vbs_max_over__blk2119_dn4, locals.var_vbs_max_over__blk2119_dn5, locals.var_vbs_max_over__blk2119_dn6, locals.var_vbs_max_over__blk2119_dn7, locals.var_vbs_max_over__blk2119_dn8, locals.var_vbs_max_over__blk2119_dn9, locals.var_vbs_max_over__blk2119_dn10, locals.var_vbs_max_over__blk2119_dn13,)
    }
};
        locals.var_vbs_max_over__blk2119 = assign90830_e139500;
        locals.var_vbs_max_over__blk2119_dn0 = assign90830_e139500_d_n0;
        locals.var_vbs_max_over__blk2119_dn2 = assign90830_e139500_d_n2;
        locals.var_vbs_max_over__blk2119_dn4 = assign90830_e139500_d_n4;
        locals.var_vbs_max_over__blk2119_dn5 = assign90830_e139500_d_n5;
        locals.var_vbs_max_over__blk2119_dn6 = assign90830_e139500_d_n6;
        locals.var_vbs_max_over__blk2119_dn7 = assign90830_e139500_d_n7;
        locals.var_vbs_max_over__blk2119_dn8 = assign90830_e139500_d_n8;
        locals.var_vbs_max_over__blk2119_dn9 = assign90830_e139500_d_n9;
        locals.var_vbs_max_over__blk2119_dn10 = assign90830_e139500_d_n10;
        locals.var_vbs_max_over__blk2119_dn13 = assign90830_e139500_d_n13;
        locals.var_vbs_max_over__blk2119_rv = 0.0;

        let assign90840_e139504: f64 = (locals.var_vbs_max_over__blk2119 * 0.5);
        let assign90840_e139505: f64 = if locals.var_vbs_bnd_over__blk2120 > assign90840_e139504 { 1.0 } else { 0.0 };
        locals.var_guard2125 = assign90840_e139505;
        locals.var_guard2125_rv = 0.0;

        let (assign90850_e139515, assign90850_e139515_d_n0, assign90850_e139515_d_n2, assign90850_e139515_d_n4, assign90850_e139515_d_n5, assign90850_e139515_d_n6, assign90850_e139515_d_n7, assign90850_e139515_d_n8, assign90850_e139515_d_n9, assign90850_e139515_d_n10, assign90850_e139515_d_n13,) = {
    if (((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2125 != 0.0)) {
        let assign90850_e139513: f64 = (0.5 * locals.var_vbs_max_over__blk2119);
        (assign90850_e139513, (0.5 * locals.var_vbs_max_over__blk2119_dn0), (0.5 * locals.var_vbs_max_over__blk2119_dn2), (0.5 * locals.var_vbs_max_over__blk2119_dn4), (0.5 * locals.var_vbs_max_over__blk2119_dn5), (0.5 * locals.var_vbs_max_over__blk2119_dn6), (0.5 * locals.var_vbs_max_over__blk2119_dn7), (0.5 * locals.var_vbs_max_over__blk2119_dn8), (0.5 * locals.var_vbs_max_over__blk2119_dn9), (0.5 * locals.var_vbs_max_over__blk2119_dn10), (0.5 * locals.var_vbs_max_over__blk2119_dn13),)
    } else {
        (locals.var_vbs_bnd_over__blk2120, locals.var_vbs_bnd_over__blk2120_dn0, locals.var_vbs_bnd_over__blk2120_dn2, locals.var_vbs_bnd_over__blk2120_dn4, locals.var_vbs_bnd_over__blk2120_dn5, locals.var_vbs_bnd_over__blk2120_dn6, locals.var_vbs_bnd_over__blk2120_dn7, locals.var_vbs_bnd_over__blk2120_dn8, locals.var_vbs_bnd_over__blk2120_dn9, locals.var_vbs_bnd_over__blk2120_dn10, locals.var_vbs_bnd_over__blk2120_dn13,)
    }
};
        locals.var_vbs_bnd_over__blk2120 = assign90850_e139515;
        locals.var_vbs_bnd_over__blk2120_dn0 = assign90850_e139515_d_n0;
        locals.var_vbs_bnd_over__blk2120_dn2 = assign90850_e139515_d_n2;
        locals.var_vbs_bnd_over__blk2120_dn4 = assign90850_e139515_d_n4;
        locals.var_vbs_bnd_over__blk2120_dn5 = assign90850_e139515_d_n5;
        locals.var_vbs_bnd_over__blk2120_dn6 = assign90850_e139515_d_n6;
        locals.var_vbs_bnd_over__blk2120_dn7 = assign90850_e139515_d_n7;
        locals.var_vbs_bnd_over__blk2120_dn8 = assign90850_e139515_d_n8;
        locals.var_vbs_bnd_over__blk2120_dn9 = assign90850_e139515_d_n9;
        locals.var_vbs_bnd_over__blk2120_dn10 = assign90850_e139515_d_n10;
        locals.var_vbs_bnd_over__blk2120_dn13 = assign90850_e139515_d_n13;
        locals.var_vbs_bnd_over__blk2120_rv = 0.0;

        let assign90860_e139517: f64 = if param_given[338] { 1.0 } else { 0.0 };
        locals.var_guard2126 = assign90860_e139517;
        locals.var_guard2126_rv = 0.0;

        let (assign90870_e139525, assign90870_e139525_d_n0, assign90870_e139525_d_n2, assign90870_e139525_d_n4, assign90870_e139525_d_n5, assign90870_e139525_d_n6, assign90870_e139525_d_n7, assign90870_e139525_d_n8, assign90870_e139525_d_n9, assign90870_e139525_d_n10, assign90870_e139525_d_n13,) = {
    if (((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2126 != 0.0)) {
        (p.p338, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vbs_max_over__blk2119, locals.var_vbs_max_over__blk2119_dn0, locals.var_vbs_max_over__blk2119_dn2, locals.var_vbs_max_over__blk2119_dn4, locals.var_vbs_max_over__blk2119_dn5, locals.var_vbs_max_over__blk2119_dn6, locals.var_vbs_max_over__blk2119_dn7, locals.var_vbs_max_over__blk2119_dn8, locals.var_vbs_max_over__blk2119_dn9, locals.var_vbs_max_over__blk2119_dn10, locals.var_vbs_max_over__blk2119_dn13,)
    }
};
        locals.var_vbs_max_over__blk2119 = assign90870_e139525;
        locals.var_vbs_max_over__blk2119_dn0 = assign90870_e139525_d_n0;
        locals.var_vbs_max_over__blk2119_dn2 = assign90870_e139525_d_n2;
        locals.var_vbs_max_over__blk2119_dn4 = assign90870_e139525_d_n4;
        locals.var_vbs_max_over__blk2119_dn5 = assign90870_e139525_d_n5;
        locals.var_vbs_max_over__blk2119_dn6 = assign90870_e139525_d_n6;
        locals.var_vbs_max_over__blk2119_dn7 = assign90870_e139525_d_n7;
        locals.var_vbs_max_over__blk2119_dn8 = assign90870_e139525_d_n8;
        locals.var_vbs_max_over__blk2119_dn9 = assign90870_e139525_d_n9;
        locals.var_vbs_max_over__blk2119_dn10 = assign90870_e139525_d_n10;
        locals.var_vbs_max_over__blk2119_dn13 = assign90870_e139525_d_n13;
        locals.var_vbs_max_over__blk2119_rv = 0.0;

        let assign90880_e139527: f64 = if param_given[339] { 1.0 } else { 0.0 };
        locals.var_guard2127 = assign90880_e139527;
        locals.var_guard2127_rv = 0.0;

        let (assign90890_e139535, assign90890_e139535_d_n0, assign90890_e139535_d_n2, assign90890_e139535_d_n4, assign90890_e139535_d_n5, assign90890_e139535_d_n6, assign90890_e139535_d_n7, assign90890_e139535_d_n8, assign90890_e139535_d_n9, assign90890_e139535_d_n10, assign90890_e139535_d_n13,) = {
    if (((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2127 != 0.0)) {
        (p.p339, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vbs_bnd_over__blk2120, locals.var_vbs_bnd_over__blk2120_dn0, locals.var_vbs_bnd_over__blk2120_dn2, locals.var_vbs_bnd_over__blk2120_dn4, locals.var_vbs_bnd_over__blk2120_dn5, locals.var_vbs_bnd_over__blk2120_dn6, locals.var_vbs_bnd_over__blk2120_dn7, locals.var_vbs_bnd_over__blk2120_dn8, locals.var_vbs_bnd_over__blk2120_dn9, locals.var_vbs_bnd_over__blk2120_dn10, locals.var_vbs_bnd_over__blk2120_dn13,)
    }
};
        locals.var_vbs_bnd_over__blk2120 = assign90890_e139535;
        locals.var_vbs_bnd_over__blk2120_dn0 = assign90890_e139535_d_n0;
        locals.var_vbs_bnd_over__blk2120_dn2 = assign90890_e139535_d_n2;
        locals.var_vbs_bnd_over__blk2120_dn4 = assign90890_e139535_d_n4;
        locals.var_vbs_bnd_over__blk2120_dn5 = assign90890_e139535_d_n5;
        locals.var_vbs_bnd_over__blk2120_dn6 = assign90890_e139535_d_n6;
        locals.var_vbs_bnd_over__blk2120_dn7 = assign90890_e139535_d_n7;
        locals.var_vbs_bnd_over__blk2120_dn8 = assign90890_e139535_d_n8;
        locals.var_vbs_bnd_over__blk2120_dn9 = assign90890_e139535_d_n9;
        locals.var_vbs_bnd_over__blk2120_dn10 = assign90890_e139535_d_n10;
        locals.var_vbs_bnd_over__blk2120_dn13 = assign90890_e139535_d_n13;
        locals.var_vbs_bnd_over__blk2120_rv = 0.0;

        let assign90900_e139537: f64 = if param_given[338] { 1.0 } else { 0.0 };
        locals.var_guard2128 = assign90900_e139537;
        locals.var_guard2128_rv = 0.0;

        let (assign90910_e139550, assign90910_e139550_d_n0, assign90910_e139550_d_n2, assign90910_e139550_d_n4, assign90910_e139550_d_n5, assign90910_e139550_d_n6, assign90910_e139550_d_n7, assign90910_e139550_d_n8, assign90910_e139550_d_n9, assign90910_e139550_d_n10, assign90910_e139550_d_n13,) = {
    if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2127 == 0.0)) && (locals.var_guard2128 != 0.0)) {
        let assign90910_e139548: f64 = (0.5 * locals.var_vbs_max_over__blk2119);
        (assign90910_e139548, (0.5 * locals.var_vbs_max_over__blk2119_dn0), (0.5 * locals.var_vbs_max_over__blk2119_dn2), (0.5 * locals.var_vbs_max_over__blk2119_dn4), (0.5 * locals.var_vbs_max_over__blk2119_dn5), (0.5 * locals.var_vbs_max_over__blk2119_dn6), (0.5 * locals.var_vbs_max_over__blk2119_dn7), (0.5 * locals.var_vbs_max_over__blk2119_dn8), (0.5 * locals.var_vbs_max_over__blk2119_dn9), (0.5 * locals.var_vbs_max_over__blk2119_dn10), (0.5 * locals.var_vbs_max_over__blk2119_dn13),)
    } else {
        (locals.var_vbs_bnd_over__blk2120, locals.var_vbs_bnd_over__blk2120_dn0, locals.var_vbs_bnd_over__blk2120_dn2, locals.var_vbs_bnd_over__blk2120_dn4, locals.var_vbs_bnd_over__blk2120_dn5, locals.var_vbs_bnd_over__blk2120_dn6, locals.var_vbs_bnd_over__blk2120_dn7, locals.var_vbs_bnd_over__blk2120_dn8, locals.var_vbs_bnd_over__blk2120_dn9, locals.var_vbs_bnd_over__blk2120_dn10, locals.var_vbs_bnd_over__blk2120_dn13,)
    }
};
        locals.var_vbs_bnd_over__blk2120 = assign90910_e139550;
        locals.var_vbs_bnd_over__blk2120_dn0 = assign90910_e139550_d_n0;
        locals.var_vbs_bnd_over__blk2120_dn2 = assign90910_e139550_d_n2;
        locals.var_vbs_bnd_over__blk2120_dn4 = assign90910_e139550_d_n4;
        locals.var_vbs_bnd_over__blk2120_dn5 = assign90910_e139550_d_n5;
        locals.var_vbs_bnd_over__blk2120_dn6 = assign90910_e139550_d_n6;
        locals.var_vbs_bnd_over__blk2120_dn7 = assign90910_e139550_d_n7;
        locals.var_vbs_bnd_over__blk2120_dn8 = assign90910_e139550_d_n8;
        locals.var_vbs_bnd_over__blk2120_dn9 = assign90910_e139550_d_n9;
        locals.var_vbs_bnd_over__blk2120_dn10 = assign90910_e139550_d_n10;
        locals.var_vbs_bnd_over__blk2120_dn13 = assign90910_e139550_d_n13;
        locals.var_vbs_bnd_over__blk2120_rv = 0.0;

        let assign90920_e139554: f64 = (locals.var_vbs_max_over__blk2119 * 0.5);
        let assign90920_e139555: f64 = if locals.var_vbs_bnd_over__blk2120 > assign90920_e139554 { 1.0 } else { 0.0 };
        locals.var_guard2129 = assign90920_e139555;
        locals.var_guard2129_rv = 0.0;

        let (assign90930_e139565, assign90930_e139565_d_n0, assign90930_e139565_d_n2, assign90930_e139565_d_n4, assign90930_e139565_d_n5, assign90930_e139565_d_n6, assign90930_e139565_d_n7, assign90930_e139565_d_n8, assign90930_e139565_d_n9, assign90930_e139565_d_n10, assign90930_e139565_d_n13,) = {
    if (((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2129 != 0.0)) {
        let assign90930_e139563: f64 = (0.5 * locals.var_vbs_max_over__blk2119);
        (assign90930_e139563, (0.5 * locals.var_vbs_max_over__blk2119_dn0), (0.5 * locals.var_vbs_max_over__blk2119_dn2), (0.5 * locals.var_vbs_max_over__blk2119_dn4), (0.5 * locals.var_vbs_max_over__blk2119_dn5), (0.5 * locals.var_vbs_max_over__blk2119_dn6), (0.5 * locals.var_vbs_max_over__blk2119_dn7), (0.5 * locals.var_vbs_max_over__blk2119_dn8), (0.5 * locals.var_vbs_max_over__blk2119_dn9), (0.5 * locals.var_vbs_max_over__blk2119_dn10), (0.5 * locals.var_vbs_max_over__blk2119_dn13),)
    } else {
        (locals.var_vbs_bnd_over__blk2120, locals.var_vbs_bnd_over__blk2120_dn0, locals.var_vbs_bnd_over__blk2120_dn2, locals.var_vbs_bnd_over__blk2120_dn4, locals.var_vbs_bnd_over__blk2120_dn5, locals.var_vbs_bnd_over__blk2120_dn6, locals.var_vbs_bnd_over__blk2120_dn7, locals.var_vbs_bnd_over__blk2120_dn8, locals.var_vbs_bnd_over__blk2120_dn9, locals.var_vbs_bnd_over__blk2120_dn10, locals.var_vbs_bnd_over__blk2120_dn13,)
    }
};
        locals.var_vbs_bnd_over__blk2120 = assign90930_e139565;
        locals.var_vbs_bnd_over__blk2120_dn0 = assign90930_e139565_d_n0;
        locals.var_vbs_bnd_over__blk2120_dn2 = assign90930_e139565_d_n2;
        locals.var_vbs_bnd_over__blk2120_dn4 = assign90930_e139565_d_n4;
        locals.var_vbs_bnd_over__blk2120_dn5 = assign90930_e139565_d_n5;
        locals.var_vbs_bnd_over__blk2120_dn6 = assign90930_e139565_d_n6;
        locals.var_vbs_bnd_over__blk2120_dn7 = assign90930_e139565_d_n7;
        locals.var_vbs_bnd_over__blk2120_dn8 = assign90930_e139565_d_n8;
        locals.var_vbs_bnd_over__blk2120_dn9 = assign90930_e139565_d_n9;
        locals.var_vbs_bnd_over__blk2120_dn10 = assign90930_e139565_d_n10;
        locals.var_vbs_bnd_over__blk2120_dn13 = assign90930_e139565_d_n13;
        locals.var_vbs_bnd_over__blk2120_rv = 0.0;

        let assign90940_e139568: f64 = if p.p38 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard2130 = assign90940_e139568;
        locals.var_guard2130_rv = 0.0;

        let (assign90950_e139577, assign90950_e139577_d_n0, assign90950_e139577_d_n2, assign90950_e139577_d_n4, assign90950_e139577_d_n5, assign90950_e139577_d_n6, assign90950_e139577_d_n7, assign90950_e139577_d_n8, assign90950_e139577_d_n9, assign90950_e139577_d_n10, assign90950_e139577_d_n13,) = {
    if (((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2130 != 0.0)) {
        let assign90950_e139575: f64 = (-locals.var_vxbgmt);
        (assign90950_e139575, (-locals.var_vxbgmt_dn0), (-locals.var_vxbgmt_dn2), (-locals.var_vxbgmt_dn4), (-locals.var_vxbgmt_dn5), (-locals.var_vxbgmt_dn6), (-locals.var_vxbgmt_dn7), (-locals.var_vxbgmt_dn8), (-locals.var_vxbgmt_dn9), (-locals.var_vxbgmt_dn10), (-locals.var_vxbgmt_dn13),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign90950_e139577;
        locals.var_t0_dn0 = assign90950_e139577_d_n0;
        locals.var_t0_dn2 = assign90950_e139577_d_n2;
        locals.var_t0_dn4 = assign90950_e139577_d_n4;
        locals.var_t0_dn5 = assign90950_e139577_d_n5;
        locals.var_t0_dn6 = assign90950_e139577_d_n6;
        locals.var_t0_dn7 = assign90950_e139577_d_n7;
        locals.var_t0_dn8 = assign90950_e139577_d_n8;
        locals.var_t0_dn9 = assign90950_e139577_d_n9;
        locals.var_t0_dn10 = assign90950_e139577_d_n10;
        locals.var_t0_dn13 = assign90950_e139577_d_n13;
        locals.var_t0_rv = 0.0;

        let assign90960_e139580: f64 = if locals.var_t0 > locals.var_vbs_bnd_over__blk2120 { 1.0 } else { 0.0 };
        locals.var_guard2131 = assign90960_e139580;
        locals.var_guard2131_rv = 0.0;

        let (assign90970_e139592, assign90970_e139592_d_n0, assign90970_e139592_d_n2, assign90970_e139592_d_n4, assign90970_e139592_d_n5, assign90970_e139592_d_n6, assign90970_e139592_d_n7, assign90970_e139592_d_n8, assign90970_e139592_d_n9, assign90970_e139592_d_n10, assign90970_e139592_d_n13,) = {
    if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2130 != 0.0)) && (locals.var_guard2131 != 0.0)) {
        let assign90970_e139590: f64 = (locals.var_t0 - locals.var_vbs_bnd_over__blk2120);
        (assign90970_e139590, (locals.var_t0_dn0 - locals.var_vbs_bnd_over__blk2120_dn0), (locals.var_t0_dn2 - locals.var_vbs_bnd_over__blk2120_dn2), (locals.var_t0_dn4 - locals.var_vbs_bnd_over__blk2120_dn4), (locals.var_t0_dn5 - locals.var_vbs_bnd_over__blk2120_dn5), (locals.var_t0_dn6 - locals.var_vbs_bnd_over__blk2120_dn6), (locals.var_t0_dn7 - locals.var_vbs_bnd_over__blk2120_dn7), (locals.var_t0_dn8 - locals.var_vbs_bnd_over__blk2120_dn8), (locals.var_t0_dn9 - locals.var_vbs_bnd_over__blk2120_dn9), (locals.var_t0_dn10 - locals.var_vbs_bnd_over__blk2120_dn10), (locals.var_t0_dn13 - locals.var_vbs_bnd_over__blk2120_dn13),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign90970_e139592;
        locals.var_t1_dn0 = assign90970_e139592_d_n0;
        locals.var_t1_dn2 = assign90970_e139592_d_n2;
        locals.var_t1_dn4 = assign90970_e139592_d_n4;
        locals.var_t1_dn5 = assign90970_e139592_d_n5;
        locals.var_t1_dn6 = assign90970_e139592_d_n6;
        locals.var_t1_dn7 = assign90970_e139592_d_n7;
        locals.var_t1_dn8 = assign90970_e139592_d_n8;
        locals.var_t1_dn9 = assign90970_e139592_d_n9;
        locals.var_t1_dn10 = assign90970_e139592_d_n10;
        locals.var_t1_dn13 = assign90970_e139592_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign90980_e139604, assign90980_e139604_d_n0, assign90980_e139604_d_n2, assign90980_e139604_d_n4, assign90980_e139604_d_n5, assign90980_e139604_d_n6, assign90980_e139604_d_n7, assign90980_e139604_d_n8, assign90980_e139604_d_n9, assign90980_e139604_d_n10, assign90980_e139604_d_n13,) = {
    if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2130 != 0.0)) && (locals.var_guard2131 != 0.0)) {
        let assign90980_e139602: f64 = (locals.var_vbs_max_over__blk2119 - locals.var_vbs_bnd_over__blk2120);
        (assign90980_e139602, (locals.var_vbs_max_over__blk2119_dn0 - locals.var_vbs_bnd_over__blk2120_dn0), (locals.var_vbs_max_over__blk2119_dn2 - locals.var_vbs_bnd_over__blk2120_dn2), (locals.var_vbs_max_over__blk2119_dn4 - locals.var_vbs_bnd_over__blk2120_dn4), (locals.var_vbs_max_over__blk2119_dn5 - locals.var_vbs_bnd_over__blk2120_dn5), (locals.var_vbs_max_over__blk2119_dn6 - locals.var_vbs_bnd_over__blk2120_dn6), (locals.var_vbs_max_over__blk2119_dn7 - locals.var_vbs_bnd_over__blk2120_dn7), (locals.var_vbs_max_over__blk2119_dn8 - locals.var_vbs_bnd_over__blk2120_dn8), (locals.var_vbs_max_over__blk2119_dn9 - locals.var_vbs_bnd_over__blk2120_dn9), (locals.var_vbs_max_over__blk2119_dn10 - locals.var_vbs_bnd_over__blk2120_dn10), (locals.var_vbs_max_over__blk2119_dn13 - locals.var_vbs_bnd_over__blk2120_dn13),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign90980_e139604;
        locals.var_t2_dn0 = assign90980_e139604_d_n0;
        locals.var_t2_dn2 = assign90980_e139604_d_n2;
        locals.var_t2_dn4 = assign90980_e139604_d_n4;
        locals.var_t2_dn5 = assign90980_e139604_d_n5;
        locals.var_t2_dn6 = assign90980_e139604_d_n6;
        locals.var_t2_dn7 = assign90980_e139604_d_n7;
        locals.var_t2_dn8 = assign90980_e139604_d_n8;
        locals.var_t2_dn9 = assign90980_e139604_d_n9;
        locals.var_t2_dn10 = assign90980_e139604_d_n10;
        locals.var_t2_dn13 = assign90980_e139604_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign90990_e139616, assign90990_e139616_d_n0, assign90990_e139616_d_n2, assign90990_e139616_d_n4, assign90990_e139616_d_n5, assign90990_e139616_d_n6, assign90990_e139616_d_n7, assign90990_e139616_d_n8, assign90990_e139616_d_n9, assign90990_e139616_d_n10, assign90990_e139616_d_n13,) = {
    if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2130 != 0.0)) && (locals.var_guard2131 != 0.0)) {
        let assign90990_e139614: f64 = (locals.var_t1 / locals.var_t2);
        (assign90990_e139614, (((locals.var_t1_dn0 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn0)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn2 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn2)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn4 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn4)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn5 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn5)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn6 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn6)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn7 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn7)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn8 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn8)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn9 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn9)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn10 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn10)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn13 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn13)) / (locals.var_t2 * locals.var_t2)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign90990_e139616;
        locals.var_tmf1_dn0 = assign90990_e139616_d_n0;
        locals.var_tmf1_dn2 = assign90990_e139616_d_n2;
        locals.var_tmf1_dn4 = assign90990_e139616_d_n4;
        locals.var_tmf1_dn5 = assign90990_e139616_d_n5;
        locals.var_tmf1_dn6 = assign90990_e139616_d_n6;
        locals.var_tmf1_dn7 = assign90990_e139616_d_n7;
        locals.var_tmf1_dn8 = assign90990_e139616_d_n8;
        locals.var_tmf1_dn9 = assign90990_e139616_d_n9;
        locals.var_tmf1_dn10 = assign90990_e139616_d_n10;
        locals.var_tmf1_dn13 = assign90990_e139616_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign91000_e139628, assign91000_e139628_d_n0, assign91000_e139628_d_n2, assign91000_e139628_d_n4, assign91000_e139628_d_n5, assign91000_e139628_d_n6, assign91000_e139628_d_n7, assign91000_e139628_d_n8, assign91000_e139628_d_n9, assign91000_e139628_d_n10, assign91000_e139628_d_n13,) = {
    if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2130 != 0.0)) && (locals.var_guard2131 != 0.0)) {
        let assign91000_e139626: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign91000_e139626, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign91000_e139628;
        locals.var_tmf2_dn0 = assign91000_e139628_d_n0;
        locals.var_tmf2_dn2 = assign91000_e139628_d_n2;
        locals.var_tmf2_dn4 = assign91000_e139628_d_n4;
        locals.var_tmf2_dn5 = assign91000_e139628_d_n5;
        locals.var_tmf2_dn6 = assign91000_e139628_d_n6;
        locals.var_tmf2_dn7 = assign91000_e139628_d_n7;
        locals.var_tmf2_dn8 = assign91000_e139628_d_n8;
        locals.var_tmf2_dn9 = assign91000_e139628_d_n9;
        locals.var_tmf2_dn10 = assign91000_e139628_d_n10;
        locals.var_tmf2_dn13 = assign91000_e139628_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign91010_e139640, assign91010_e139640_d_n0, assign91010_e139640_d_n2, assign91010_e139640_d_n4, assign91010_e139640_d_n5, assign91010_e139640_d_n6, assign91010_e139640_d_n7, assign91010_e139640_d_n8, assign91010_e139640_d_n9, assign91010_e139640_d_n10, assign91010_e139640_d_n13,) = {
    if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2130 != 0.0)) && (locals.var_guard2131 != 0.0)) {
        let assign91010_e139638: f64 = (locals.var_tmf2 * locals.var_tmf1);
        (assign91010_e139638, ((locals.var_tmf2_dn0 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn0)), ((locals.var_tmf2_dn2 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn2)), ((locals.var_tmf2_dn4 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn4)), ((locals.var_tmf2_dn5 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn5)), ((locals.var_tmf2_dn6 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn6)), ((locals.var_tmf2_dn7 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn7)), ((locals.var_tmf2_dn8 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn8)), ((locals.var_tmf2_dn9 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn9)), ((locals.var_tmf2_dn10 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn10)), ((locals.var_tmf2_dn13 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn13)),)
    } else {
        (locals.var_tmf3, locals.var_tmf3_dn0, locals.var_tmf3_dn2, locals.var_tmf3_dn4, locals.var_tmf3_dn5, locals.var_tmf3_dn6, locals.var_tmf3_dn7, locals.var_tmf3_dn8, locals.var_tmf3_dn9, locals.var_tmf3_dn10, locals.var_tmf3_dn13,)
    }
};
        locals.var_tmf3 = assign91010_e139640;
        locals.var_tmf3_dn0 = assign91010_e139640_d_n0;
        locals.var_tmf3_dn2 = assign91010_e139640_d_n2;
        locals.var_tmf3_dn4 = assign91010_e139640_d_n4;
        locals.var_tmf3_dn5 = assign91010_e139640_d_n5;
        locals.var_tmf3_dn6 = assign91010_e139640_d_n6;
        locals.var_tmf3_dn7 = assign91010_e139640_d_n7;
        locals.var_tmf3_dn8 = assign91010_e139640_d_n8;
        locals.var_tmf3_dn9 = assign91010_e139640_d_n9;
        locals.var_tmf3_dn10 = assign91010_e139640_d_n10;
        locals.var_tmf3_dn13 = assign91010_e139640_d_n13;
        locals.var_tmf3_rv = 0.0;

        let (assign91020_e139652, assign91020_e139652_d_n0, assign91020_e139652_d_n2, assign91020_e139652_d_n4, assign91020_e139652_d_n5, assign91020_e139652_d_n6, assign91020_e139652_d_n7, assign91020_e139652_d_n8, assign91020_e139652_d_n9, assign91020_e139652_d_n10, assign91020_e139652_d_n13,) = {
    if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2130 != 0.0)) && (locals.var_guard2131 != 0.0)) {
        let assign91020_e139650: f64 = (locals.var_tmf2 * locals.var_tmf2);
        (assign91020_e139650, ((locals.var_tmf2_dn0 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn0)), ((locals.var_tmf2_dn2 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn2)), ((locals.var_tmf2_dn4 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn4)), ((locals.var_tmf2_dn5 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn5)), ((locals.var_tmf2_dn6 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn6)), ((locals.var_tmf2_dn7 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn7)), ((locals.var_tmf2_dn8 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn8)), ((locals.var_tmf2_dn9 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn9)), ((locals.var_tmf2_dn10 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn10)), ((locals.var_tmf2_dn13 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn13)),)
    } else {
        (locals.var_tmf4, locals.var_tmf4_dn0, locals.var_tmf4_dn2, locals.var_tmf4_dn4, locals.var_tmf4_dn5, locals.var_tmf4_dn6, locals.var_tmf4_dn7, locals.var_tmf4_dn8, locals.var_tmf4_dn9, locals.var_tmf4_dn10, locals.var_tmf4_dn13,)
    }
};
        locals.var_tmf4 = assign91020_e139652;
        locals.var_tmf4_dn0 = assign91020_e139652_d_n0;
        locals.var_tmf4_dn2 = assign91020_e139652_d_n2;
        locals.var_tmf4_dn4 = assign91020_e139652_d_n4;
        locals.var_tmf4_dn5 = assign91020_e139652_d_n5;
        locals.var_tmf4_dn6 = assign91020_e139652_d_n6;
        locals.var_tmf4_dn7 = assign91020_e139652_d_n7;
        locals.var_tmf4_dn8 = assign91020_e139652_d_n8;
        locals.var_tmf4_dn9 = assign91020_e139652_d_n9;
        locals.var_tmf4_dn10 = assign91020_e139652_d_n10;
        locals.var_tmf4_dn13 = assign91020_e139652_d_n13;
        locals.var_tmf4_rv = 0.0;

        let (assign91030_e139672, assign91030_e139672_d_n0, assign91030_e139672_d_n2, assign91030_e139672_d_n4, assign91030_e139672_d_n5, assign91030_e139672_d_n6, assign91030_e139672_d_n7, assign91030_e139672_d_n8, assign91030_e139672_d_n9, assign91030_e139672_d_n10, assign91030_e139672_d_n13,) = {
    if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2130 != 0.0)) && (locals.var_guard2131 != 0.0)) {
        let assign91030_e139663: f64 = (1.0 + locals.var_tmf1);
        let assign91030_e139665: f64 = (assign91030_e139663 + locals.var_tmf2);
        let assign91030_e139667: f64 = (assign91030_e139665 + locals.var_tmf3);
        let assign91030_e139669: f64 = (assign91030_e139667 + locals.var_tmf4);
        let assign91030_e139670: f64 = (1.0 / assign91030_e139669);
        (assign91030_e139670, (-((((locals.var_tmf1_dn0 + locals.var_tmf2_dn0) + locals.var_tmf3_dn0) + locals.var_tmf4_dn0) / (assign91030_e139669 * assign91030_e139669))), (-((((locals.var_tmf1_dn2 + locals.var_tmf2_dn2) + locals.var_tmf3_dn2) + locals.var_tmf4_dn2) / (assign91030_e139669 * assign91030_e139669))), (-((((locals.var_tmf1_dn4 + locals.var_tmf2_dn4) + locals.var_tmf3_dn4) + locals.var_tmf4_dn4) / (assign91030_e139669 * assign91030_e139669))), (-((((locals.var_tmf1_dn5 + locals.var_tmf2_dn5) + locals.var_tmf3_dn5) + locals.var_tmf4_dn5) / (assign91030_e139669 * assign91030_e139669))), (-((((locals.var_tmf1_dn6 + locals.var_tmf2_dn6) + locals.var_tmf3_dn6) + locals.var_tmf4_dn6) / (assign91030_e139669 * assign91030_e139669))), (-((((locals.var_tmf1_dn7 + locals.var_tmf2_dn7) + locals.var_tmf3_dn7) + locals.var_tmf4_dn7) / (assign91030_e139669 * assign91030_e139669))), (-((((locals.var_tmf1_dn8 + locals.var_tmf2_dn8) + locals.var_tmf3_dn8) + locals.var_tmf4_dn8) / (assign91030_e139669 * assign91030_e139669))), (-((((locals.var_tmf1_dn9 + locals.var_tmf2_dn9) + locals.var_tmf3_dn9) + locals.var_tmf4_dn9) / (assign91030_e139669 * assign91030_e139669))), (-((((locals.var_tmf1_dn10 + locals.var_tmf2_dn10) + locals.var_tmf3_dn10) + locals.var_tmf4_dn10) / (assign91030_e139669 * assign91030_e139669))), (-((((locals.var_tmf1_dn13 + locals.var_tmf2_dn13) + locals.var_tmf3_dn13) + locals.var_tmf4_dn13) / (assign91030_e139669 * assign91030_e139669))),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn13,)
    }
};
        locals.var_tmf0 = assign91030_e139672;
        locals.var_tmf0_dn0 = assign91030_e139672_d_n0;
        locals.var_tmf0_dn2 = assign91030_e139672_d_n2;
        locals.var_tmf0_dn4 = assign91030_e139672_d_n4;
        locals.var_tmf0_dn5 = assign91030_e139672_d_n5;
        locals.var_tmf0_dn6 = assign91030_e139672_d_n6;
        locals.var_tmf0_dn7 = assign91030_e139672_d_n7;
        locals.var_tmf0_dn8 = assign91030_e139672_d_n8;
        locals.var_tmf0_dn9 = assign91030_e139672_d_n9;
        locals.var_tmf0_dn10 = assign91030_e139672_d_n10;
        locals.var_tmf0_dn13 = assign91030_e139672_d_n13;
        locals.var_tmf0_rv = 0.0;

        let (assign91040_e139699, assign91040_e139699_d_n0, assign91040_e139699_d_n2, assign91040_e139699_d_n4, assign91040_e139699_d_n5, assign91040_e139699_d_n6, assign91040_e139699_d_n7, assign91040_e139699_d_n8, assign91040_e139699_d_n9, assign91040_e139699_d_n10, assign91040_e139699_d_n13,) = {
    if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2130 != 0.0)) && (locals.var_guard2131 != 0.0)) {
        let assign91040_e139683: f64 = (2.0 * locals.var_tmf1);
        let assign91040_e139684: f64 = (1.0 + assign91040_e139683);
        let assign91040_e139687: f64 = (3.0 * locals.var_tmf2);
        let assign91040_e139688: f64 = (assign91040_e139684 + assign91040_e139687);
        let assign91040_e139691: f64 = (4.0 * locals.var_tmf3);
        let assign91040_e139692: f64 = (assign91040_e139688 + assign91040_e139691);
        let assign91040_e139693: f64 = (-assign91040_e139692);
        let assign91040_e139695: f64 = (assign91040_e139693 * locals.var_tmf0);
        let assign91040_e139697: f64 = (assign91040_e139695 * locals.var_tmf0);
        (assign91040_e139697, (((((-(((2.0 * locals.var_tmf1_dn0) + (3.0 * locals.var_tmf2_dn0)) + (4.0 * locals.var_tmf3_dn0))) * locals.var_tmf0) + (assign91040_e139693 * locals.var_tmf0_dn0)) * locals.var_tmf0) + (assign91040_e139695 * locals.var_tmf0_dn0)), (((((-(((2.0 * locals.var_tmf1_dn2) + (3.0 * locals.var_tmf2_dn2)) + (4.0 * locals.var_tmf3_dn2))) * locals.var_tmf0) + (assign91040_e139693 * locals.var_tmf0_dn2)) * locals.var_tmf0) + (assign91040_e139695 * locals.var_tmf0_dn2)), (((((-(((2.0 * locals.var_tmf1_dn4) + (3.0 * locals.var_tmf2_dn4)) + (4.0 * locals.var_tmf3_dn4))) * locals.var_tmf0) + (assign91040_e139693 * locals.var_tmf0_dn4)) * locals.var_tmf0) + (assign91040_e139695 * locals.var_tmf0_dn4)), (((((-(((2.0 * locals.var_tmf1_dn5) + (3.0 * locals.var_tmf2_dn5)) + (4.0 * locals.var_tmf3_dn5))) * locals.var_tmf0) + (assign91040_e139693 * locals.var_tmf0_dn5)) * locals.var_tmf0) + (assign91040_e139695 * locals.var_tmf0_dn5)), (((((-(((2.0 * locals.var_tmf1_dn6) + (3.0 * locals.var_tmf2_dn6)) + (4.0 * locals.var_tmf3_dn6))) * locals.var_tmf0) + (assign91040_e139693 * locals.var_tmf0_dn6)) * locals.var_tmf0) + (assign91040_e139695 * locals.var_tmf0_dn6)), (((((-(((2.0 * locals.var_tmf1_dn7) + (3.0 * locals.var_tmf2_dn7)) + (4.0 * locals.var_tmf3_dn7))) * locals.var_tmf0) + (assign91040_e139693 * locals.var_tmf0_dn7)) * locals.var_tmf0) + (assign91040_e139695 * locals.var_tmf0_dn7)), (((((-(((2.0 * locals.var_tmf1_dn8) + (3.0 * locals.var_tmf2_dn8)) + (4.0 * locals.var_tmf3_dn8))) * locals.var_tmf0) + (assign91040_e139693 * locals.var_tmf0_dn8)) * locals.var_tmf0) + (assign91040_e139695 * locals.var_tmf0_dn8)), (((((-(((2.0 * locals.var_tmf1_dn9) + (3.0 * locals.var_tmf2_dn9)) + (4.0 * locals.var_tmf3_dn9))) * locals.var_tmf0) + (assign91040_e139693 * locals.var_tmf0_dn9)) * locals.var_tmf0) + (assign91040_e139695 * locals.var_tmf0_dn9)), (((((-(((2.0 * locals.var_tmf1_dn10) + (3.0 * locals.var_tmf2_dn10)) + (4.0 * locals.var_tmf3_dn10))) * locals.var_tmf0) + (assign91040_e139693 * locals.var_tmf0_dn10)) * locals.var_tmf0) + (assign91040_e139695 * locals.var_tmf0_dn10)), (((((-(((2.0 * locals.var_tmf1_dn13) + (3.0 * locals.var_tmf2_dn13)) + (4.0 * locals.var_tmf3_dn13))) * locals.var_tmf0) + (assign91040_e139693 * locals.var_tmf0_dn13)) * locals.var_tmf0) + (assign91040_e139695 * locals.var_tmf0_dn13)),)
    } else {
        (locals.var_t11, locals.var_t11_dn0, locals.var_t11_dn2, locals.var_t11_dn4, locals.var_t11_dn5, locals.var_t11_dn6, locals.var_t11_dn7, locals.var_t11_dn8, locals.var_t11_dn9, locals.var_t11_dn10, locals.var_t11_dn13,)
    }
};
        locals.var_t11 = assign91040_e139699;
        locals.var_t11_dn0 = assign91040_e139699_d_n0;
        locals.var_t11_dn2 = assign91040_e139699_d_n2;
        locals.var_t11_dn4 = assign91040_e139699_d_n4;
        locals.var_t11_dn5 = assign91040_e139699_d_n5;
        locals.var_t11_dn6 = assign91040_e139699_d_n6;
        locals.var_t11_dn7 = assign91040_e139699_d_n7;
        locals.var_t11_dn8 = assign91040_e139699_d_n8;
        locals.var_t11_dn9 = assign91040_e139699_d_n9;
        locals.var_t11_dn10 = assign91040_e139699_d_n10;
        locals.var_t11_dn13 = assign91040_e139699_d_n13;
        locals.var_t11_rv = 0.0;

        let (assign91050_e139713, assign91050_e139713_d_n0, assign91050_e139713_d_n2, assign91050_e139713_d_n4, assign91050_e139713_d_n5, assign91050_e139713_d_n6, assign91050_e139713_d_n7, assign91050_e139713_d_n8, assign91050_e139713_d_n9, assign91050_e139713_d_n10, assign91050_e139713_d_n13,) = {
    if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2130 != 0.0)) && (locals.var_guard2131 != 0.0)) {
        let assign91050_e139710: f64 = (1.0 - locals.var_tmf0);
        let assign91050_e139711: f64 = (locals.var_t2 * assign91050_e139710);
        (assign91050_e139711, ((locals.var_t2_dn0 * assign91050_e139710) + (locals.var_t2 * (-locals.var_tmf0_dn0))), ((locals.var_t2_dn2 * assign91050_e139710) + (locals.var_t2 * (-locals.var_tmf0_dn2))), ((locals.var_t2_dn4 * assign91050_e139710) + (locals.var_t2 * (-locals.var_tmf0_dn4))), ((locals.var_t2_dn5 * assign91050_e139710) + (locals.var_t2 * (-locals.var_tmf0_dn5))), ((locals.var_t2_dn6 * assign91050_e139710) + (locals.var_t2 * (-locals.var_tmf0_dn6))), ((locals.var_t2_dn7 * assign91050_e139710) + (locals.var_t2 * (-locals.var_tmf0_dn7))), ((locals.var_t2_dn8 * assign91050_e139710) + (locals.var_t2 * (-locals.var_tmf0_dn8))), ((locals.var_t2_dn9 * assign91050_e139710) + (locals.var_t2 * (-locals.var_tmf0_dn9))), ((locals.var_t2_dn10 * assign91050_e139710) + (locals.var_t2 * (-locals.var_tmf0_dn10))), ((locals.var_t2_dn13 * assign91050_e139710) + (locals.var_t2 * (-locals.var_tmf0_dn13))),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn4, locals.var_ty_dn5, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn8, locals.var_ty_dn9, locals.var_ty_dn10, locals.var_ty_dn13,)
    }
};
        locals.var_ty = assign91050_e139713;
        locals.var_ty_dn0 = assign91050_e139713_d_n0;
        locals.var_ty_dn2 = assign91050_e139713_d_n2;
        locals.var_ty_dn4 = assign91050_e139713_d_n4;
        locals.var_ty_dn5 = assign91050_e139713_d_n5;
        locals.var_ty_dn6 = assign91050_e139713_d_n6;
        locals.var_ty_dn7 = assign91050_e139713_d_n7;
        locals.var_ty_dn8 = assign91050_e139713_d_n8;
        locals.var_ty_dn9 = assign91050_e139713_d_n9;
        locals.var_ty_dn10 = assign91050_e139713_d_n10;
        locals.var_ty_dn13 = assign91050_e139713_d_n13;
        locals.var_ty_rv = 0.0;

        let (assign91060_e139729, assign91060_e139729_d_n0, assign91060_e139729_d_n2, assign91060_e139729_d_n4, assign91060_e139729_d_n5, assign91060_e139729_d_n6, assign91060_e139729_d_n7, assign91060_e139729_d_n8, assign91060_e139729_d_n9, assign91060_e139729_d_n10, assign91060_e139729_d_n13,) = {
    if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2130 != 0.0)) && (locals.var_guard2131 != 0.0)) {
        let assign91060_e139723: f64 = (1.0 - locals.var_tmf0);
        let assign91060_e139726: f64 = (locals.var_tmf1 * locals.var_t11);
        let assign91060_e139727: f64 = (assign91060_e139723 + assign91060_e139726);
        (assign91060_e139727, ((-locals.var_tmf0_dn0) + ((locals.var_tmf1_dn0 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn0))), ((-locals.var_tmf0_dn2) + ((locals.var_tmf1_dn2 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn2))), ((-locals.var_tmf0_dn4) + ((locals.var_tmf1_dn4 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn4))), ((-locals.var_tmf0_dn5) + ((locals.var_tmf1_dn5 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn5))), ((-locals.var_tmf0_dn6) + ((locals.var_tmf1_dn6 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn6))), ((-locals.var_tmf0_dn7) + ((locals.var_tmf1_dn7 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn7))), ((-locals.var_tmf0_dn8) + ((locals.var_tmf1_dn8 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn8))), ((-locals.var_tmf0_dn9) + ((locals.var_tmf1_dn9 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn9))), ((-locals.var_tmf0_dn10) + ((locals.var_tmf1_dn10 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn10))), ((-locals.var_tmf0_dn13) + ((locals.var_tmf1_dn13 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn13))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign91060_e139729;
        locals.var_t0_dn0 = assign91060_e139729_d_n0;
        locals.var_t0_dn2 = assign91060_e139729_d_n2;
        locals.var_t0_dn4 = assign91060_e139729_d_n4;
        locals.var_t0_dn5 = assign91060_e139729_d_n5;
        locals.var_t0_dn6 = assign91060_e139729_d_n6;
        locals.var_t0_dn7 = assign91060_e139729_d_n7;
        locals.var_t0_dn8 = assign91060_e139729_d_n8;
        locals.var_t0_dn9 = assign91060_e139729_d_n9;
        locals.var_t0_dn10 = assign91060_e139729_d_n10;
        locals.var_t0_dn13 = assign91060_e139729_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign91070_e139740, assign91070_e139740_d_n0, assign91070_e139740_d_n2, assign91070_e139740_d_n4, assign91070_e139740_d_n5, assign91070_e139740_d_n6, assign91070_e139740_d_n7, assign91070_e139740_d_n8, assign91070_e139740_d_n9, assign91070_e139740_d_n10, assign91070_e139740_d_n13,) = {
    if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2130 != 0.0)) && (locals.var_guard2131 != 0.0)) {
        let assign91070_e139738: f64 = (-locals.var_t11);
        (assign91070_e139738, (-locals.var_t11_dn0), (-locals.var_t11_dn2), (-locals.var_t11_dn4), (-locals.var_t11_dn5), (-locals.var_t11_dn6), (-locals.var_t11_dn7), (-locals.var_t11_dn8), (-locals.var_t11_dn9), (-locals.var_t11_dn10), (-locals.var_t11_dn13),)
    } else {
        (locals.var_t11, locals.var_t11_dn0, locals.var_t11_dn2, locals.var_t11_dn4, locals.var_t11_dn5, locals.var_t11_dn6, locals.var_t11_dn7, locals.var_t11_dn8, locals.var_t11_dn9, locals.var_t11_dn10, locals.var_t11_dn13,)
    }
};
        locals.var_t11 = assign91070_e139740;
        locals.var_t11_dn0 = assign91070_e139740_d_n0;
        locals.var_t11_dn2 = assign91070_e139740_d_n2;
        locals.var_t11_dn4 = assign91070_e139740_d_n4;
        locals.var_t11_dn5 = assign91070_e139740_d_n5;
        locals.var_t11_dn6 = assign91070_e139740_d_n6;
        locals.var_t11_dn7 = assign91070_e139740_d_n7;
        locals.var_t11_dn8 = assign91070_e139740_d_n8;
        locals.var_t11_dn9 = assign91070_e139740_d_n9;
        locals.var_t11_dn10 = assign91070_e139740_d_n10;
        locals.var_t11_dn13 = assign91070_e139740_d_n13;
        locals.var_t11_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_340(
        locals: &mut StampLocals,
    ) {
        let (assign91080_e139752, assign91080_e139752_d_n0, assign91080_e139752_d_n2, assign91080_e139752_d_n4, assign91080_e139752_d_n5, assign91080_e139752_d_n6, assign91080_e139752_d_n7, assign91080_e139752_d_n8, assign91080_e139752_d_n9, assign91080_e139752_d_n10, assign91080_e139752_d_n13,) = {
    if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2130 != 0.0)) && (locals.var_guard2131 != 0.0)) {
        let assign91080_e139750: f64 = (locals.var_vbs_bnd_over__blk2120 + locals.var_ty);
        (assign91080_e139750, (locals.var_vbs_bnd_over__blk2120_dn0 + locals.var_ty_dn0), (locals.var_vbs_bnd_over__blk2120_dn2 + locals.var_ty_dn2), (locals.var_vbs_bnd_over__blk2120_dn4 + locals.var_ty_dn4), (locals.var_vbs_bnd_over__blk2120_dn5 + locals.var_ty_dn5), (locals.var_vbs_bnd_over__blk2120_dn6 + locals.var_ty_dn6), (locals.var_vbs_bnd_over__blk2120_dn7 + locals.var_ty_dn7), (locals.var_vbs_bnd_over__blk2120_dn8 + locals.var_ty_dn8), (locals.var_vbs_bnd_over__blk2120_dn9 + locals.var_ty_dn9), (locals.var_vbs_bnd_over__blk2120_dn10 + locals.var_ty_dn10), (locals.var_vbs_bnd_over__blk2120_dn13 + locals.var_ty_dn13),)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn13,)
    }
};
        locals.var_t10 = assign91080_e139752;
        locals.var_t10_dn0 = assign91080_e139752_d_n0;
        locals.var_t10_dn2 = assign91080_e139752_d_n2;
        locals.var_t10_dn4 = assign91080_e139752_d_n4;
        locals.var_t10_dn5 = assign91080_e139752_d_n5;
        locals.var_t10_dn6 = assign91080_e139752_d_n6;
        locals.var_t10_dn7 = assign91080_e139752_d_n7;
        locals.var_t10_dn8 = assign91080_e139752_d_n8;
        locals.var_t10_dn9 = assign91080_e139752_d_n9;
        locals.var_t10_dn10 = assign91080_e139752_d_n10;
        locals.var_t10_dn13 = assign91080_e139752_d_n13;
        locals.var_t10_rv = 0.0;

        let (assign91090_e139763, assign91090_e139763_d_n0, assign91090_e139763_d_n2, assign91090_e139763_d_n4, assign91090_e139763_d_n5, assign91090_e139763_d_n6, assign91090_e139763_d_n7, assign91090_e139763_d_n8, assign91090_e139763_d_n9, assign91090_e139763_d_n10, assign91090_e139763_d_n13,) = {
    if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2130 != 0.0)) && (locals.var_guard2131 == 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn13,)
    }
};
        locals.var_t10 = assign91090_e139763;
        locals.var_t10_dn0 = assign91090_e139763_d_n0;
        locals.var_t10_dn2 = assign91090_e139763_d_n2;
        locals.var_t10_dn4 = assign91090_e139763_d_n4;
        locals.var_t10_dn5 = assign91090_e139763_d_n5;
        locals.var_t10_dn6 = assign91090_e139763_d_n6;
        locals.var_t10_dn7 = assign91090_e139763_d_n7;
        locals.var_t10_dn8 = assign91090_e139763_d_n8;
        locals.var_t10_dn9 = assign91090_e139763_d_n9;
        locals.var_t10_dn10 = assign91090_e139763_d_n10;
        locals.var_t10_dn13 = assign91090_e139763_d_n13;
        locals.var_t10_rv = 0.0;

        let (assign91100_e139772, assign91100_e139772_d_n0, assign91100_e139772_d_n2, assign91100_e139772_d_n4, assign91100_e139772_d_n5, assign91100_e139772_d_n6, assign91100_e139772_d_n7, assign91100_e139772_d_n8, assign91100_e139772_d_n9, assign91100_e139772_d_n10, assign91100_e139772_d_n13,) = {
    if (((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2130 != 0.0)) {
        let assign91100_e139770: f64 = (-locals.var_t10);
        (assign91100_e139770, (-locals.var_t10_dn0), (-locals.var_t10_dn2), (-locals.var_t10_dn4), (-locals.var_t10_dn5), (-locals.var_t10_dn6), (-locals.var_t10_dn7), (-locals.var_t10_dn8), (-locals.var_t10_dn9), (-locals.var_t10_dn10), (-locals.var_t10_dn13),)
    } else {
        (locals.var_vxbgmtcl, locals.var_vxbgmtcl_dn0, locals.var_vxbgmtcl_dn2, locals.var_vxbgmtcl_dn4, locals.var_vxbgmtcl_dn5, locals.var_vxbgmtcl_dn6, locals.var_vxbgmtcl_dn7, locals.var_vxbgmtcl_dn8, locals.var_vxbgmtcl_dn9, locals.var_vxbgmtcl_dn10, locals.var_vxbgmtcl_dn13,)
    }
};
        locals.var_vxbgmtcl = assign91100_e139772;
        locals.var_vxbgmtcl_dn0 = assign91100_e139772_d_n0;
        locals.var_vxbgmtcl_dn2 = assign91100_e139772_d_n2;
        locals.var_vxbgmtcl_dn4 = assign91100_e139772_d_n4;
        locals.var_vxbgmtcl_dn5 = assign91100_e139772_d_n5;
        locals.var_vxbgmtcl_dn6 = assign91100_e139772_d_n6;
        locals.var_vxbgmtcl_dn7 = assign91100_e139772_d_n7;
        locals.var_vxbgmtcl_dn8 = assign91100_e139772_d_n8;
        locals.var_vxbgmtcl_dn9 = assign91100_e139772_d_n9;
        locals.var_vxbgmtcl_dn10 = assign91100_e139772_d_n10;
        locals.var_vxbgmtcl_dn13 = assign91100_e139772_d_n13;
        locals.var_vxbgmtcl_rv = 0.0;

        let (assign91110_e139781, assign91110_e139781_d_n0, assign91110_e139781_d_n2, assign91110_e139781_d_n4, assign91110_e139781_d_n5, assign91110_e139781_d_n6, assign91110_e139781_d_n7, assign91110_e139781_d_n8, assign91110_e139781_d_n9, assign91110_e139781_d_n10, assign91110_e139781_d_n13,) = {
    if (((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2130 == 0.0)) {
        (locals.var_vxbgmt, locals.var_vxbgmt_dn0, locals.var_vxbgmt_dn2, locals.var_vxbgmt_dn4, locals.var_vxbgmt_dn5, locals.var_vxbgmt_dn6, locals.var_vxbgmt_dn7, locals.var_vxbgmt_dn8, locals.var_vxbgmt_dn9, locals.var_vxbgmt_dn10, locals.var_vxbgmt_dn13,)
    } else {
        (locals.var_vxbgmtcl, locals.var_vxbgmtcl_dn0, locals.var_vxbgmtcl_dn2, locals.var_vxbgmtcl_dn4, locals.var_vxbgmtcl_dn5, locals.var_vxbgmtcl_dn6, locals.var_vxbgmtcl_dn7, locals.var_vxbgmtcl_dn8, locals.var_vxbgmtcl_dn9, locals.var_vxbgmtcl_dn10, locals.var_vxbgmtcl_dn13,)
    }
};
        locals.var_vxbgmtcl = assign91110_e139781;
        locals.var_vxbgmtcl_dn0 = assign91110_e139781_d_n0;
        locals.var_vxbgmtcl_dn2 = assign91110_e139781_d_n2;
        locals.var_vxbgmtcl_dn4 = assign91110_e139781_d_n4;
        locals.var_vxbgmtcl_dn5 = assign91110_e139781_d_n5;
        locals.var_vxbgmtcl_dn6 = assign91110_e139781_d_n6;
        locals.var_vxbgmtcl_dn7 = assign91110_e139781_d_n7;
        locals.var_vxbgmtcl_dn8 = assign91110_e139781_d_n8;
        locals.var_vxbgmtcl_dn9 = assign91110_e139781_d_n9;
        locals.var_vxbgmtcl_dn10 = assign91110_e139781_d_n10;
        locals.var_vxbgmtcl_dn13 = assign91110_e139781_d_n13;
        locals.var_vxbgmtcl_rv = 0.0;

        let (assign91120_e139789, assign91120_e139789_d_n0, assign91120_e139789_d_n2, assign91120_e139789_d_n4, assign91120_e139789_d_n5, assign91120_e139789_d_n6, assign91120_e139789_d_n7, assign91120_e139789_d_n8, assign91120_e139789_d_n9, assign91120_e139789_d_n10, assign91120_e139789_d_n13,) = {
    if ((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) {
        let assign91120_e139787: f64 = (locals.var_cnst0over_func / locals.var_cox0_func);
        (assign91120_e139787, (locals.var_cnst0over_func_dn0 / locals.var_cox0_func), (locals.var_cnst0over_func_dn2 / locals.var_cox0_func), (locals.var_cnst0over_func_dn4 / locals.var_cox0_func), (locals.var_cnst0over_func_dn5 / locals.var_cox0_func), (locals.var_cnst0over_func_dn6 / locals.var_cox0_func), (locals.var_cnst0over_func_dn7 / locals.var_cox0_func), (locals.var_cnst0over_func_dn8 / locals.var_cox0_func), (locals.var_cnst0over_func_dn9 / locals.var_cox0_func), (locals.var_cnst0over_func_dn10 / locals.var_cox0_func), (locals.var_cnst0over_func_dn13 / locals.var_cox0_func),)
    } else {
        (locals.var_fac1, locals.var_fac1_dn0, locals.var_fac1_dn2, locals.var_fac1_dn4, locals.var_fac1_dn5, locals.var_fac1_dn6, locals.var_fac1_dn7, locals.var_fac1_dn8, locals.var_fac1_dn9, locals.var_fac1_dn10, locals.var_fac1_dn13,)
    }
};
        locals.var_fac1 = assign91120_e139789;
        locals.var_fac1_dn0 = assign91120_e139789_d_n0;
        locals.var_fac1_dn2 = assign91120_e139789_d_n2;
        locals.var_fac1_dn4 = assign91120_e139789_d_n4;
        locals.var_fac1_dn5 = assign91120_e139789_d_n5;
        locals.var_fac1_dn6 = assign91120_e139789_d_n6;
        locals.var_fac1_dn7 = assign91120_e139789_d_n7;
        locals.var_fac1_dn8 = assign91120_e139789_d_n8;
        locals.var_fac1_dn9 = assign91120_e139789_d_n9;
        locals.var_fac1_dn10 = assign91120_e139789_d_n10;
        locals.var_fac1_dn13 = assign91120_e139789_d_n13;
        locals.var_fac1_rv = 0.0;

        let (assign91130_e139797, assign91130_e139797_d_n0, assign91130_e139797_d_n2, assign91130_e139797_d_n4, assign91130_e139797_d_n5, assign91130_e139797_d_n6, assign91130_e139797_d_n7, assign91130_e139797_d_n8, assign91130_e139797_d_n9, assign91130_e139797_d_n10, assign91130_e139797_d_n13,) = {
    if ((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) {
        let assign91130_e139795: f64 = (locals.var_fac1 * locals.var_fac1);
        (assign91130_e139795, ((locals.var_fac1_dn0 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn0)), ((locals.var_fac1_dn2 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn2)), ((locals.var_fac1_dn4 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn4)), ((locals.var_fac1_dn5 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn5)), ((locals.var_fac1_dn6 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn6)), ((locals.var_fac1_dn7 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn7)), ((locals.var_fac1_dn8 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn8)), ((locals.var_fac1_dn9 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn9)), ((locals.var_fac1_dn10 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn10)), ((locals.var_fac1_dn13 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn13)),)
    } else {
        (locals.var_fac1p2, locals.var_fac1p2_dn0, locals.var_fac1p2_dn2, locals.var_fac1p2_dn4, locals.var_fac1p2_dn5, locals.var_fac1p2_dn6, locals.var_fac1p2_dn7, locals.var_fac1p2_dn8, locals.var_fac1p2_dn9, locals.var_fac1p2_dn10, locals.var_fac1p2_dn13,)
    }
};
        locals.var_fac1p2 = assign91130_e139797;
        locals.var_fac1p2_dn0 = assign91130_e139797_d_n0;
        locals.var_fac1p2_dn2 = assign91130_e139797_d_n2;
        locals.var_fac1p2_dn4 = assign91130_e139797_d_n4;
        locals.var_fac1p2_dn5 = assign91130_e139797_d_n5;
        locals.var_fac1p2_dn6 = assign91130_e139797_d_n6;
        locals.var_fac1p2_dn7 = assign91130_e139797_d_n7;
        locals.var_fac1p2_dn8 = assign91130_e139797_d_n8;
        locals.var_fac1p2_dn9 = assign91130_e139797_d_n9;
        locals.var_fac1p2_dn10 = assign91130_e139797_d_n10;
        locals.var_fac1p2_dn13 = assign91130_e139797_d_n13;
        locals.var_fac1p2_rv = 0.0;

        let (assign91140_e139806, assign91140_e139806_d_n2, assign91140_e139806_d_n6, assign91140_e139806_d_n7, assign91140_e139806_d_n8,) = {
    if ((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) {
        let assign91140_e139802: f64 = (-locals.var_vgbgmt);
        let assign91140_e139804: f64 = (assign91140_e139802 + locals.var_uc_vfbover);
        (assign91140_e139804, (-locals.var_vgbgmt_dn2), (-locals.var_vgbgmt_dn6), (-locals.var_vgbgmt_dn7), (-locals.var_vgbgmt_dn8),)
    } else {
        (locals.var_vgpld, locals.var_vgpld_dn2, locals.var_vgpld_dn6, locals.var_vgpld_dn7, locals.var_vgpld_dn8,)
    }
};
        locals.var_vgpld = assign91140_e139806;
        locals.var_vgpld_dn2 = assign91140_e139806_d_n2;
        locals.var_vgpld_dn6 = assign91140_e139806_d_n6;
        locals.var_vgpld_dn7 = assign91140_e139806_d_n7;
        locals.var_vgpld_dn8 = assign91140_e139806_d_n8;
        locals.var_vgpld_rv = 0.0;

        let (assign91150_e139817, assign91150_e139817_d_n0, assign91150_e139817_d_n2, assign91150_e139817_d_n4, assign91150_e139817_d_n5, assign91150_e139817_d_n6, assign91150_e139817_d_n7, assign91150_e139817_d_n8, assign91150_e139817_d_n9, assign91150_e139817_d_n10, assign91150_e139817_d_n13,) = {
    if ((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) {
        let assign91150_e139811: f64 = (-locals.var_vxbgmtcl);
        let assign91150_e139814: f64 = (10.0 * 2.220446049250313e-16);
        let assign91150_e139815: f64 = (assign91150_e139811 + assign91150_e139814);
        (assign91150_e139815, (-locals.var_vxbgmtcl_dn0), (-locals.var_vxbgmtcl_dn2), (-locals.var_vxbgmtcl_dn4), (-locals.var_vxbgmtcl_dn5), (-locals.var_vxbgmtcl_dn6), (-locals.var_vxbgmtcl_dn7), (-locals.var_vxbgmtcl_dn8), (-locals.var_vxbgmtcl_dn9), (-locals.var_vxbgmtcl_dn10), (-locals.var_vxbgmtcl_dn13),)
    } else {
        (locals.var_vgb_fb_ld, locals.var_vgb_fb_ld_dn0, locals.var_vgb_fb_ld_dn2, locals.var_vgb_fb_ld_dn4, locals.var_vgb_fb_ld_dn5, locals.var_vgb_fb_ld_dn6, locals.var_vgb_fb_ld_dn7, locals.var_vgb_fb_ld_dn8, locals.var_vgb_fb_ld_dn9, locals.var_vgb_fb_ld_dn10, locals.var_vgb_fb_ld_dn13,)
    }
};
        locals.var_vgb_fb_ld = assign91150_e139817;
        locals.var_vgb_fb_ld_dn0 = assign91150_e139817_d_n0;
        locals.var_vgb_fb_ld_dn2 = assign91150_e139817_d_n2;
        locals.var_vgb_fb_ld_dn4 = assign91150_e139817_d_n4;
        locals.var_vgb_fb_ld_dn5 = assign91150_e139817_d_n5;
        locals.var_vgb_fb_ld_dn6 = assign91150_e139817_d_n6;
        locals.var_vgb_fb_ld_dn7 = assign91150_e139817_d_n7;
        locals.var_vgb_fb_ld_dn8 = assign91150_e139817_d_n8;
        locals.var_vgb_fb_ld_dn9 = assign91150_e139817_d_n9;
        locals.var_vgb_fb_ld_dn10 = assign91150_e139817_d_n10;
        locals.var_vgb_fb_ld_dn13 = assign91150_e139817_d_n13;
        locals.var_vgb_fb_ld_rv = 0.0;

        let (assign91160_e139823, assign91160_e139823_d_n0, assign91160_e139823_d_n2, assign91160_e139823_d_n4, assign91160_e139823_d_n5, assign91160_e139823_d_n6, assign91160_e139823_d_n7, assign91160_e139823_d_n8, assign91160_e139823_d_n9, assign91160_e139823_d_n10, assign91160_e139823_d_n13,) = {
    if ((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_q_dep_ld__blk2114, locals.var_q_dep_ld__blk2114_dn0, locals.var_q_dep_ld__blk2114_dn2, locals.var_q_dep_ld__blk2114_dn4, locals.var_q_dep_ld__blk2114_dn5, locals.var_q_dep_ld__blk2114_dn6, locals.var_q_dep_ld__blk2114_dn7, locals.var_q_dep_ld__blk2114_dn8, locals.var_q_dep_ld__blk2114_dn9, locals.var_q_dep_ld__blk2114_dn10, locals.var_q_dep_ld__blk2114_dn13,)
    }
};
        locals.var_q_dep_ld__blk2114 = assign91160_e139823;
        locals.var_q_dep_ld__blk2114_dn0 = assign91160_e139823_d_n0;
        locals.var_q_dep_ld__blk2114_dn2 = assign91160_e139823_d_n2;
        locals.var_q_dep_ld__blk2114_dn4 = assign91160_e139823_d_n4;
        locals.var_q_dep_ld__blk2114_dn5 = assign91160_e139823_d_n5;
        locals.var_q_dep_ld__blk2114_dn6 = assign91160_e139823_d_n6;
        locals.var_q_dep_ld__blk2114_dn7 = assign91160_e139823_d_n7;
        locals.var_q_dep_ld__blk2114_dn8 = assign91160_e139823_d_n8;
        locals.var_q_dep_ld__blk2114_dn9 = assign91160_e139823_d_n9;
        locals.var_q_dep_ld__blk2114_dn10 = assign91160_e139823_d_n10;
        locals.var_q_dep_ld__blk2114_dn13 = assign91160_e139823_d_n13;
        locals.var_q_dep_ld__blk2114_rv = 0.0;

        let (assign91170_e139831,) = {
    if ((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) {
        let assign91170_e139829: f64 = (1.6021918e-19 * locals.var_nover_func);
        (assign91170_e139829,)
    } else {
        (locals.var_q_nsubld__blk2115,)
    }
};
        locals.var_q_nsubld__blk2115 = assign91170_e139831;
        locals.var_q_nsubld__blk2115_rv = 0.0;

        let (assign91180_e139839, assign91180_e139839_d_n0, assign91180_e139839_d_n2, assign91180_e139839_d_n4, assign91180_e139839_d_n5, assign91180_e139839_d_n6, assign91180_e139839_d_n7, assign91180_e139839_d_n8, assign91180_e139839_d_n9, assign91180_e139839_d_n10, assign91180_e139839_d_n13,) = {
    if ((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) {
        let assign91180_e139837: f64 = (locals.var_nin / locals.var_nover_func);
        (assign91180_e139837, (locals.var_nin_dn0 / locals.var_nover_func), (locals.var_nin_dn2 / locals.var_nover_func), (locals.var_nin_dn4 / locals.var_nover_func), (locals.var_nin_dn5 / locals.var_nover_func), (locals.var_nin_dn6 / locals.var_nover_func), (locals.var_nin_dn7 / locals.var_nover_func), (locals.var_nin_dn8 / locals.var_nover_func), (locals.var_nin_dn9 / locals.var_nover_func), (locals.var_nin_dn10 / locals.var_nover_func), (locals.var_nin_dn13 / locals.var_nover_func),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign91180_e139839;
        locals.var_t0_dn0 = assign91180_e139839_d_n0;
        locals.var_t0_dn2 = assign91180_e139839_d_n2;
        locals.var_t0_dn4 = assign91180_e139839_d_n4;
        locals.var_t0_dn5 = assign91180_e139839_d_n5;
        locals.var_t0_dn6 = assign91180_e139839_d_n6;
        locals.var_t0_dn7 = assign91180_e139839_d_n7;
        locals.var_t0_dn8 = assign91180_e139839_d_n8;
        locals.var_t0_dn9 = assign91180_e139839_d_n9;
        locals.var_t0_dn10 = assign91180_e139839_d_n10;
        locals.var_t0_dn13 = assign91180_e139839_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign91190_e139847, assign91190_e139847_d_n0, assign91190_e139847_d_n2, assign91190_e139847_d_n4, assign91190_e139847_d_n5, assign91190_e139847_d_n6, assign91190_e139847_d_n7, assign91190_e139847_d_n8, assign91190_e139847_d_n9, assign91190_e139847_d_n10, assign91190_e139847_d_n13,) = {
    if ((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) {
        let assign91190_e139845: f64 = (locals.var_t0 * locals.var_t0);
        (assign91190_e139845, ((locals.var_t0_dn0 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn0)), ((locals.var_t0_dn2 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn2)), ((locals.var_t0_dn4 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn4)), ((locals.var_t0_dn5 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn5)), ((locals.var_t0_dn6 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn6)), ((locals.var_t0_dn7 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn7)), ((locals.var_t0_dn8 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn8)), ((locals.var_t0_dn9 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn9)), ((locals.var_t0_dn10 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn10)), ((locals.var_t0_dn13 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn13)),)
    } else {
        (locals.var_cnst1over, locals.var_cnst1over_dn0, locals.var_cnst1over_dn2, locals.var_cnst1over_dn4, locals.var_cnst1over_dn5, locals.var_cnst1over_dn6, locals.var_cnst1over_dn7, locals.var_cnst1over_dn8, locals.var_cnst1over_dn9, locals.var_cnst1over_dn10, locals.var_cnst1over_dn13,)
    }
};
        locals.var_cnst1over = assign91190_e139847;
        locals.var_cnst1over_dn0 = assign91190_e139847_d_n0;
        locals.var_cnst1over_dn2 = assign91190_e139847_d_n2;
        locals.var_cnst1over_dn4 = assign91190_e139847_d_n4;
        locals.var_cnst1over_dn5 = assign91190_e139847_d_n5;
        locals.var_cnst1over_dn6 = assign91190_e139847_d_n6;
        locals.var_cnst1over_dn7 = assign91190_e139847_d_n7;
        locals.var_cnst1over_dn8 = assign91190_e139847_d_n8;
        locals.var_cnst1over_dn9 = assign91190_e139847_d_n9;
        locals.var_cnst1over_dn10 = assign91190_e139847_d_n10;
        locals.var_cnst1over_dn13 = assign91190_e139847_d_n13;
        locals.var_cnst1over_rv = 0.0;

        let assign91200_e139850: f64 = (-locals.var_vxbgmtcl);
        let assign91200_e139851: f64 = (locals.var_beta * assign91200_e139850);
        let assign91200_e139853: f64 = if assign91200_e139851 >= 500.0 { 1.0 } else { 0.0 };
        locals.var_guard2132 = assign91200_e139853;
        locals.var_guard2132_rv = 0.0;

        let (assign91210_e139870, assign91210_e139870_d_n0, assign91210_e139870_d_n2, assign91210_e139870_d_n4, assign91210_e139870_d_n5, assign91210_e139870_d_n6, assign91210_e139870_d_n7, assign91210_e139870_d_n8, assign91210_e139870_d_n9, assign91210_e139870_d_n10, assign91210_e139870_d_n13,) = {
    if (((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2132 != 0.0)) {
        let assign91210_e139863: f64 = (-locals.var_vxbgmtcl);
        let assign91210_e139864: f64 = (locals.var_beta * assign91210_e139863);
        let assign91210_e139865: f64 = (1.0 + assign91210_e139864);
        let assign91210_e139867: f64 = (assign91210_e139865 - 500.0);
        let assign91210_e139868: f64 = (1.403592217853e217 * assign91210_e139867);
        (assign91210_e139868, (1.403592217853e217 * ((locals.var_beta_dn0 * assign91210_e139863) + (locals.var_beta * (-locals.var_vxbgmtcl_dn0)))), (1.403592217853e217 * ((locals.var_beta_dn2 * assign91210_e139863) + (locals.var_beta * (-locals.var_vxbgmtcl_dn2)))), (1.403592217853e217 * ((locals.var_beta_dn4 * assign91210_e139863) + (locals.var_beta * (-locals.var_vxbgmtcl_dn4)))), (1.403592217853e217 * ((locals.var_beta_dn5 * assign91210_e139863) + (locals.var_beta * (-locals.var_vxbgmtcl_dn5)))), (1.403592217853e217 * ((locals.var_beta_dn6 * assign91210_e139863) + (locals.var_beta * (-locals.var_vxbgmtcl_dn6)))), (1.403592217853e217 * ((locals.var_beta_dn7 * assign91210_e139863) + (locals.var_beta * (-locals.var_vxbgmtcl_dn7)))), (1.403592217853e217 * ((locals.var_beta_dn8 * assign91210_e139863) + (locals.var_beta * (-locals.var_vxbgmtcl_dn8)))), (1.403592217853e217 * ((locals.var_beta_dn9 * assign91210_e139863) + (locals.var_beta * (-locals.var_vxbgmtcl_dn9)))), (1.403592217853e217 * ((locals.var_beta_dn10 * assign91210_e139863) + (locals.var_beta * (-locals.var_vxbgmtcl_dn10)))), (1.403592217853e217 * ((locals.var_beta_dn13 * assign91210_e139863) + (locals.var_beta * (-locals.var_vxbgmtcl_dn13)))),)
    } else {
        (locals.var_exp_bvbs, locals.var_exp_bvbs_dn0, locals.var_exp_bvbs_dn2, locals.var_exp_bvbs_dn4, locals.var_exp_bvbs_dn5, locals.var_exp_bvbs_dn6, locals.var_exp_bvbs_dn7, locals.var_exp_bvbs_dn8, locals.var_exp_bvbs_dn9, locals.var_exp_bvbs_dn10, locals.var_exp_bvbs_dn13,)
    }
};
        locals.var_exp_bvbs = assign91210_e139870;
        locals.var_exp_bvbs_dn0 = assign91210_e139870_d_n0;
        locals.var_exp_bvbs_dn2 = assign91210_e139870_d_n2;
        locals.var_exp_bvbs_dn4 = assign91210_e139870_d_n4;
        locals.var_exp_bvbs_dn5 = assign91210_e139870_d_n5;
        locals.var_exp_bvbs_dn6 = assign91210_e139870_d_n6;
        locals.var_exp_bvbs_dn7 = assign91210_e139870_d_n7;
        locals.var_exp_bvbs_dn8 = assign91210_e139870_d_n8;
        locals.var_exp_bvbs_dn9 = assign91210_e139870_d_n9;
        locals.var_exp_bvbs_dn10 = assign91210_e139870_d_n10;
        locals.var_exp_bvbs_dn13 = assign91210_e139870_d_n13;
        locals.var_exp_bvbs_rv = 0.0;

        let (assign91220_e139878, assign91220_e139878_d_n0, assign91220_e139878_d_n2, assign91220_e139878_d_n4, assign91220_e139878_d_n5, assign91220_e139878_d_n6, assign91220_e139878_d_n7, assign91220_e139878_d_n8, assign91220_e139878_d_n9, assign91220_e139878_d_n10, assign91220_e139878_d_n13,) = {
    if (((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2132 != 0.0)) {
        (1.403592217853e217, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign91220_e139878;
        locals.var_t0_dn0 = assign91220_e139878_d_n0;
        locals.var_t0_dn2 = assign91220_e139878_d_n2;
        locals.var_t0_dn4 = assign91220_e139878_d_n4;
        locals.var_t0_dn5 = assign91220_e139878_d_n5;
        locals.var_t0_dn6 = assign91220_e139878_d_n6;
        locals.var_t0_dn7 = assign91220_e139878_d_n7;
        locals.var_t0_dn8 = assign91220_e139878_d_n8;
        locals.var_t0_dn9 = assign91220_e139878_d_n9;
        locals.var_t0_dn10 = assign91220_e139878_d_n10;
        locals.var_t0_dn13 = assign91220_e139878_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign91230_e139890, assign91230_e139890_d_n0, assign91230_e139890_d_n2, assign91230_e139890_d_n4, assign91230_e139890_d_n5, assign91230_e139890_d_n6, assign91230_e139890_d_n7, assign91230_e139890_d_n8, assign91230_e139890_d_n9, assign91230_e139890_d_n10, assign91230_e139890_d_n13,) = {
    if (((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2132 == 0.0)) {
        let assign91230_e139887: f64 = (-locals.var_vxbgmtcl);
        let assign91230_e139888: f64 = (locals.var_beta * assign91230_e139887);
        (assign91230_e139888, ((locals.var_beta_dn0 * assign91230_e139887) + (locals.var_beta * (-locals.var_vxbgmtcl_dn0))), ((locals.var_beta_dn2 * assign91230_e139887) + (locals.var_beta * (-locals.var_vxbgmtcl_dn2))), ((locals.var_beta_dn4 * assign91230_e139887) + (locals.var_beta * (-locals.var_vxbgmtcl_dn4))), ((locals.var_beta_dn5 * assign91230_e139887) + (locals.var_beta * (-locals.var_vxbgmtcl_dn5))), ((locals.var_beta_dn6 * assign91230_e139887) + (locals.var_beta * (-locals.var_vxbgmtcl_dn6))), ((locals.var_beta_dn7 * assign91230_e139887) + (locals.var_beta * (-locals.var_vxbgmtcl_dn7))), ((locals.var_beta_dn8 * assign91230_e139887) + (locals.var_beta * (-locals.var_vxbgmtcl_dn8))), ((locals.var_beta_dn9 * assign91230_e139887) + (locals.var_beta * (-locals.var_vxbgmtcl_dn9))), ((locals.var_beta_dn10 * assign91230_e139887) + (locals.var_beta * (-locals.var_vxbgmtcl_dn10))), ((locals.var_beta_dn13 * assign91230_e139887) + (locals.var_beta * (-locals.var_vxbgmtcl_dn13))),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign91230_e139890;
        locals.var_tmf1_dn0 = assign91230_e139890_d_n0;
        locals.var_tmf1_dn2 = assign91230_e139890_d_n2;
        locals.var_tmf1_dn4 = assign91230_e139890_d_n4;
        locals.var_tmf1_dn5 = assign91230_e139890_d_n5;
        locals.var_tmf1_dn6 = assign91230_e139890_d_n6;
        locals.var_tmf1_dn7 = assign91230_e139890_d_n7;
        locals.var_tmf1_dn8 = assign91230_e139890_d_n8;
        locals.var_tmf1_dn9 = assign91230_e139890_d_n9;
        locals.var_tmf1_dn10 = assign91230_e139890_d_n10;
        locals.var_tmf1_dn13 = assign91230_e139890_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign91240_e139899, assign91240_e139899_d_n0, assign91240_e139899_d_n2, assign91240_e139899_d_n4, assign91240_e139899_d_n5, assign91240_e139899_d_n6, assign91240_e139899_d_n7, assign91240_e139899_d_n8, assign91240_e139899_d_n9, assign91240_e139899_d_n10, assign91240_e139899_d_n13,) = {
    if (((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2132 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_exp_bvbs, locals.var_exp_bvbs_dn0, locals.var_exp_bvbs_dn2, locals.var_exp_bvbs_dn4, locals.var_exp_bvbs_dn5, locals.var_exp_bvbs_dn6, locals.var_exp_bvbs_dn7, locals.var_exp_bvbs_dn8, locals.var_exp_bvbs_dn9, locals.var_exp_bvbs_dn10, locals.var_exp_bvbs_dn13,)
    }
};
        locals.var_exp_bvbs = assign91240_e139899;
        locals.var_exp_bvbs_dn0 = assign91240_e139899_d_n0;
        locals.var_exp_bvbs_dn2 = assign91240_e139899_d_n2;
        locals.var_exp_bvbs_dn4 = assign91240_e139899_d_n4;
        locals.var_exp_bvbs_dn5 = assign91240_e139899_d_n5;
        locals.var_exp_bvbs_dn6 = assign91240_e139899_d_n6;
        locals.var_exp_bvbs_dn7 = assign91240_e139899_d_n7;
        locals.var_exp_bvbs_dn8 = assign91240_e139899_d_n8;
        locals.var_exp_bvbs_dn9 = assign91240_e139899_d_n9;
        locals.var_exp_bvbs_dn10 = assign91240_e139899_d_n10;
        locals.var_exp_bvbs_dn13 = assign91240_e139899_d_n13;
        locals.var_exp_bvbs_rv = 0.0;

        let mut assign91250_loop_guard: usize = 0;
        while {
            let assign91250_cond_e139909: f64 = if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2132 == 0.0)) && (locals.var_tmf1 >= 60.0)) { 1.0 } else { 0.0 };
            assign91250_cond_e139909 != 0.0
        } {
            assign91250_loop_guard += 1;
            assert!(assign91250_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign91250_body0_e139920, assign91250_body0_e139920_d_n0, assign91250_body0_e139920_d_n2, assign91250_body0_e139920_d_n4, assign91250_body0_e139920_d_n5, assign91250_body0_e139920_d_n6, assign91250_body0_e139920_d_n7, assign91250_body0_e139920_d_n8, assign91250_body0_e139920_d_n9, assign91250_body0_e139920_d_n10, assign91250_body0_e139920_d_n13,) = {
    if (((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2132 == 0.0)) {
        let assign91250_body0_e139918: f64 = (locals.var_exp_bvbs * 1.14200738981568e26);
        (assign91250_body0_e139918, (locals.var_exp_bvbs_dn0 * 1.14200738981568e26), (locals.var_exp_bvbs_dn2 * 1.14200738981568e26), (locals.var_exp_bvbs_dn4 * 1.14200738981568e26), (locals.var_exp_bvbs_dn5 * 1.14200738981568e26), (locals.var_exp_bvbs_dn6 * 1.14200738981568e26), (locals.var_exp_bvbs_dn7 * 1.14200738981568e26), (locals.var_exp_bvbs_dn8 * 1.14200738981568e26), (locals.var_exp_bvbs_dn9 * 1.14200738981568e26), (locals.var_exp_bvbs_dn10 * 1.14200738981568e26), (locals.var_exp_bvbs_dn13 * 1.14200738981568e26),)
    } else {
        (locals.var_exp_bvbs, locals.var_exp_bvbs_dn0, locals.var_exp_bvbs_dn2, locals.var_exp_bvbs_dn4, locals.var_exp_bvbs_dn5, locals.var_exp_bvbs_dn6, locals.var_exp_bvbs_dn7, locals.var_exp_bvbs_dn8, locals.var_exp_bvbs_dn9, locals.var_exp_bvbs_dn10, locals.var_exp_bvbs_dn13,)
    }
};
            locals.var_exp_bvbs = assign91250_body0_e139920;
            locals.var_exp_bvbs_dn0 = assign91250_body0_e139920_d_n0;
            locals.var_exp_bvbs_dn2 = assign91250_body0_e139920_d_n2;
            locals.var_exp_bvbs_dn4 = assign91250_body0_e139920_d_n4;
            locals.var_exp_bvbs_dn5 = assign91250_body0_e139920_d_n5;
            locals.var_exp_bvbs_dn6 = assign91250_body0_e139920_d_n6;
            locals.var_exp_bvbs_dn7 = assign91250_body0_e139920_d_n7;
            locals.var_exp_bvbs_dn8 = assign91250_body0_e139920_d_n8;
            locals.var_exp_bvbs_dn9 = assign91250_body0_e139920_d_n9;
            locals.var_exp_bvbs_dn10 = assign91250_body0_e139920_d_n10;
            locals.var_exp_bvbs_dn13 = assign91250_body0_e139920_d_n13;
            locals.var_exp_bvbs_rv = 0.0;
            let (assign91250_body1_e139931, assign91250_body1_e139931_d_n0, assign91250_body1_e139931_d_n2, assign91250_body1_e139931_d_n4, assign91250_body1_e139931_d_n5, assign91250_body1_e139931_d_n6, assign91250_body1_e139931_d_n7, assign91250_body1_e139931_d_n8, assign91250_body1_e139931_d_n9, assign91250_body1_e139931_d_n10, assign91250_body1_e139931_d_n13,) = {
    if (((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2132 == 0.0)) {
        let assign91250_body1_e139929: f64 = (locals.var_tmf1 - 60.0);
        (assign91250_body1_e139929, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
            locals.var_tmf1 = assign91250_body1_e139931;
            locals.var_tmf1_dn0 = assign91250_body1_e139931_d_n0;
            locals.var_tmf1_dn2 = assign91250_body1_e139931_d_n2;
            locals.var_tmf1_dn4 = assign91250_body1_e139931_d_n4;
            locals.var_tmf1_dn5 = assign91250_body1_e139931_d_n5;
            locals.var_tmf1_dn6 = assign91250_body1_e139931_d_n6;
            locals.var_tmf1_dn7 = assign91250_body1_e139931_d_n7;
            locals.var_tmf1_dn8 = assign91250_body1_e139931_d_n8;
            locals.var_tmf1_dn9 = assign91250_body1_e139931_d_n9;
            locals.var_tmf1_dn10 = assign91250_body1_e139931_d_n10;
            locals.var_tmf1_dn13 = assign91250_body1_e139931_d_n13;
            locals.var_tmf1_rv = 0.0;
        }

        let (assign91260_e139943, assign91260_e139943_d_n0, assign91260_e139943_d_n2, assign91260_e139943_d_n4, assign91260_e139943_d_n5, assign91260_e139943_d_n6, assign91260_e139943_d_n7, assign91260_e139943_d_n8, assign91260_e139943_d_n9, assign91260_e139943_d_n10, assign91260_e139943_d_n13,) = {
    if (((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2132 == 0.0)) {
        let assign91260_e139940: f64 = (locals.var_tmf1).exp();
        let assign91260_e139941: f64 = (locals.var_exp_bvbs * assign91260_e139940);
        (assign91260_e139941, ((locals.var_exp_bvbs_dn0 * assign91260_e139940) + (locals.var_exp_bvbs * (assign91260_e139940 * locals.var_tmf1_dn0))), ((locals.var_exp_bvbs_dn2 * assign91260_e139940) + (locals.var_exp_bvbs * (assign91260_e139940 * locals.var_tmf1_dn2))), ((locals.var_exp_bvbs_dn4 * assign91260_e139940) + (locals.var_exp_bvbs * (assign91260_e139940 * locals.var_tmf1_dn4))), ((locals.var_exp_bvbs_dn5 * assign91260_e139940) + (locals.var_exp_bvbs * (assign91260_e139940 * locals.var_tmf1_dn5))), ((locals.var_exp_bvbs_dn6 * assign91260_e139940) + (locals.var_exp_bvbs * (assign91260_e139940 * locals.var_tmf1_dn6))), ((locals.var_exp_bvbs_dn7 * assign91260_e139940) + (locals.var_exp_bvbs * (assign91260_e139940 * locals.var_tmf1_dn7))), ((locals.var_exp_bvbs_dn8 * assign91260_e139940) + (locals.var_exp_bvbs * (assign91260_e139940 * locals.var_tmf1_dn8))), ((locals.var_exp_bvbs_dn9 * assign91260_e139940) + (locals.var_exp_bvbs * (assign91260_e139940 * locals.var_tmf1_dn9))), ((locals.var_exp_bvbs_dn10 * assign91260_e139940) + (locals.var_exp_bvbs * (assign91260_e139940 * locals.var_tmf1_dn10))), ((locals.var_exp_bvbs_dn13 * assign91260_e139940) + (locals.var_exp_bvbs * (assign91260_e139940 * locals.var_tmf1_dn13))),)
    } else {
        (locals.var_exp_bvbs, locals.var_exp_bvbs_dn0, locals.var_exp_bvbs_dn2, locals.var_exp_bvbs_dn4, locals.var_exp_bvbs_dn5, locals.var_exp_bvbs_dn6, locals.var_exp_bvbs_dn7, locals.var_exp_bvbs_dn8, locals.var_exp_bvbs_dn9, locals.var_exp_bvbs_dn10, locals.var_exp_bvbs_dn13,)
    }
};
        locals.var_exp_bvbs = assign91260_e139943;
        locals.var_exp_bvbs_dn0 = assign91260_e139943_d_n0;
        locals.var_exp_bvbs_dn2 = assign91260_e139943_d_n2;
        locals.var_exp_bvbs_dn4 = assign91260_e139943_d_n4;
        locals.var_exp_bvbs_dn5 = assign91260_e139943_d_n5;
        locals.var_exp_bvbs_dn6 = assign91260_e139943_d_n6;
        locals.var_exp_bvbs_dn7 = assign91260_e139943_d_n7;
        locals.var_exp_bvbs_dn8 = assign91260_e139943_d_n8;
        locals.var_exp_bvbs_dn9 = assign91260_e139943_d_n9;
        locals.var_exp_bvbs_dn10 = assign91260_e139943_d_n10;
        locals.var_exp_bvbs_dn13 = assign91260_e139943_d_n13;
        locals.var_exp_bvbs_rv = 0.0;

        let (assign91270_e139952, assign91270_e139952_d_n0, assign91270_e139952_d_n2, assign91270_e139952_d_n4, assign91270_e139952_d_n5, assign91270_e139952_d_n6, assign91270_e139952_d_n7, assign91270_e139952_d_n8, assign91270_e139952_d_n9, assign91270_e139952_d_n10, assign91270_e139952_d_n13,) = {
    if (((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2132 == 0.0)) {
        (locals.var_exp_bvbs, locals.var_exp_bvbs_dn0, locals.var_exp_bvbs_dn2, locals.var_exp_bvbs_dn4, locals.var_exp_bvbs_dn5, locals.var_exp_bvbs_dn6, locals.var_exp_bvbs_dn7, locals.var_exp_bvbs_dn8, locals.var_exp_bvbs_dn9, locals.var_exp_bvbs_dn10, locals.var_exp_bvbs_dn13,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign91270_e139952;
        locals.var_t0_dn0 = assign91270_e139952_d_n0;
        locals.var_t0_dn2 = assign91270_e139952_d_n2;
        locals.var_t0_dn4 = assign91270_e139952_d_n4;
        locals.var_t0_dn5 = assign91270_e139952_d_n5;
        locals.var_t0_dn6 = assign91270_e139952_d_n6;
        locals.var_t0_dn7 = assign91270_e139952_d_n7;
        locals.var_t0_dn8 = assign91270_e139952_d_n8;
        locals.var_t0_dn9 = assign91270_e139952_d_n9;
        locals.var_t0_dn10 = assign91270_e139952_d_n10;
        locals.var_t0_dn13 = assign91270_e139952_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign91280_e139967, assign91280_e139967_d_n0, assign91280_e139967_d_n2, assign91280_e139967_d_n4, assign91280_e139967_d_n5, assign91280_e139967_d_n6, assign91280_e139967_d_n7, assign91280_e139967_d_n8, assign91280_e139967_d_n9, assign91280_e139967_d_n10, assign91280_e139967_d_n13,) = {
    if (((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_flg_never_reach_vfbover != 0.0)) {
        let assign91280_e139959: f64 = (-locals.var_vgpld);
        let assign91280_e139961: f64 = (assign91280_e139959 * 0.5);
        let assign91280_e139963: f64 = (assign91280_e139961 - 0.5);
        let assign91280_e139965: f64 = (assign91280_e139963 - 1.0);
        (assign91280_e139965, 0.0, ((-locals.var_vgpld_dn2) * 0.5), 0.0, 0.0, ((-locals.var_vgpld_dn6) * 0.5), ((-locals.var_vgpld_dn7) * 0.5), ((-locals.var_vgpld_dn8) * 0.5), 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign91280_e139967;
        locals.var_tmf1_dn0 = assign91280_e139967_d_n0;
        locals.var_tmf1_dn2 = assign91280_e139967_d_n2;
        locals.var_tmf1_dn4 = assign91280_e139967_d_n4;
        locals.var_tmf1_dn5 = assign91280_e139967_d_n5;
        locals.var_tmf1_dn6 = assign91280_e139967_d_n6;
        locals.var_tmf1_dn7 = assign91280_e139967_d_n7;
        locals.var_tmf1_dn8 = assign91280_e139967_d_n8;
        locals.var_tmf1_dn9 = assign91280_e139967_d_n9;
        locals.var_tmf1_dn10 = assign91280_e139967_d_n10;
        locals.var_tmf1_dn13 = assign91280_e139967_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign91290_e139979, assign91290_e139979_d_n0, assign91290_e139979_d_n2, assign91290_e139979_d_n4, assign91290_e139979_d_n5, assign91290_e139979_d_n6, assign91290_e139979_d_n7, assign91290_e139979_d_n8, assign91290_e139979_d_n9, assign91290_e139979_d_n10, assign91290_e139979_d_n13,) = {
    if (((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_flg_never_reach_vfbover != 0.0)) {
        let assign91290_e139975: f64 = (4.0 * 0.5);
        let assign91290_e139977: f64 = assign91290_e139975;
        (assign91290_e139977, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign91290_e139979;
        locals.var_tmf2_dn0 = assign91290_e139979_d_n0;
        locals.var_tmf2_dn2 = assign91290_e139979_d_n2;
        locals.var_tmf2_dn4 = assign91290_e139979_d_n4;
        locals.var_tmf2_dn5 = assign91290_e139979_d_n5;
        locals.var_tmf2_dn6 = assign91290_e139979_d_n6;
        locals.var_tmf2_dn7 = assign91290_e139979_d_n7;
        locals.var_tmf2_dn8 = assign91290_e139979_d_n8;
        locals.var_tmf2_dn9 = assign91290_e139979_d_n9;
        locals.var_tmf2_dn10 = assign91290_e139979_d_n10;
        locals.var_tmf2_dn13 = assign91290_e139979_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign91300_e139993, assign91300_e139993_d_n0, assign91300_e139993_d_n2, assign91300_e139993_d_n4, assign91300_e139993_d_n5, assign91300_e139993_d_n6, assign91300_e139993_d_n7, assign91300_e139993_d_n8, assign91300_e139993_d_n9, assign91300_e139993_d_n10, assign91300_e139993_d_n13,) = {
    if (((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_flg_never_reach_vfbover != 0.0)) {
        let (assign91300_e139991, assign91300_e139991_d_n0, assign91300_e139991_d_n2, assign91300_e139991_d_n4, assign91300_e139991_d_n5, assign91300_e139991_d_n6, assign91300_e139991_d_n7, assign91300_e139991_d_n8, assign91300_e139991_d_n9, assign91300_e139991_d_n10, assign91300_e139991_d_n13,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
            } else {
                let assign91300_e139990: f64 = (-locals.var_tmf2);
                (assign91300_e139990, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn13),)
            }
        };
        (assign91300_e139991, assign91300_e139991_d_n0, assign91300_e139991_d_n2, assign91300_e139991_d_n4, assign91300_e139991_d_n5, assign91300_e139991_d_n6, assign91300_e139991_d_n7, assign91300_e139991_d_n8, assign91300_e139991_d_n9, assign91300_e139991_d_n10, assign91300_e139991_d_n13,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign91300_e139993;
        locals.var_tmf2_dn0 = assign91300_e139993_d_n0;
        locals.var_tmf2_dn2 = assign91300_e139993_d_n2;
        locals.var_tmf2_dn4 = assign91300_e139993_d_n4;
        locals.var_tmf2_dn5 = assign91300_e139993_d_n5;
        locals.var_tmf2_dn6 = assign91300_e139993_d_n6;
        locals.var_tmf2_dn7 = assign91300_e139993_d_n7;
        locals.var_tmf2_dn8 = assign91300_e139993_d_n8;
        locals.var_tmf2_dn9 = assign91300_e139993_d_n9;
        locals.var_tmf2_dn10 = assign91300_e139993_d_n10;
        locals.var_tmf2_dn13 = assign91300_e139993_d_n13;
        locals.var_tmf2_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_341(
        locals: &mut StampLocals,
    ) {
        let (assign91310_e140006, assign91310_e140006_d_n0, assign91310_e140006_d_n2, assign91310_e140006_d_n4, assign91310_e140006_d_n5, assign91310_e140006_d_n6, assign91310_e140006_d_n7, assign91310_e140006_d_n8, assign91310_e140006_d_n9, assign91310_e140006_d_n10, assign91310_e140006_d_n13,) = {
    if (((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_flg_never_reach_vfbover != 0.0)) {
        let assign91310_e140001: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign91310_e140003: f64 = (assign91310_e140001 + locals.var_tmf2);
        let assign91310_e140004: f64 = (assign91310_e140003).sqrt();
        (assign91310_e140004, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign91310_e140004)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign91310_e140004)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign91310_e140004)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign91310_e140004)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign91310_e140004)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign91310_e140004)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign91310_e140004)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign91310_e140004)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign91310_e140004)), ((((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)) + locals.var_tmf2_dn13) / (2.0 * assign91310_e140004)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign91310_e140006;
        locals.var_tmf2_dn0 = assign91310_e140006_d_n0;
        locals.var_tmf2_dn2 = assign91310_e140006_d_n2;
        locals.var_tmf2_dn4 = assign91310_e140006_d_n4;
        locals.var_tmf2_dn5 = assign91310_e140006_d_n5;
        locals.var_tmf2_dn6 = assign91310_e140006_d_n6;
        locals.var_tmf2_dn7 = assign91310_e140006_d_n7;
        locals.var_tmf2_dn8 = assign91310_e140006_d_n8;
        locals.var_tmf2_dn9 = assign91310_e140006_d_n9;
        locals.var_tmf2_dn10 = assign91310_e140006_d_n10;
        locals.var_tmf2_dn13 = assign91310_e140006_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign91320_e140020, assign91320_e140020_d_n0, assign91320_e140020_d_n2, assign91320_e140020_d_n4, assign91320_e140020_d_n5, assign91320_e140020_d_n6, assign91320_e140020_d_n7, assign91320_e140020_d_n8, assign91320_e140020_d_n9, assign91320_e140020_d_n10, assign91320_e140020_d_n13,) = {
    if (((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_flg_never_reach_vfbover != 0.0)) {
        let assign91320_e140016: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign91320_e140017: f64 = (1.0 + assign91320_e140016);
        let assign91320_e140018: f64 = (0.5 * assign91320_e140017);
        (assign91320_e140018, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn13 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign91320_e140020;
        locals.var_t0_dn0 = assign91320_e140020_d_n0;
        locals.var_t0_dn2 = assign91320_e140020_d_n2;
        locals.var_t0_dn4 = assign91320_e140020_d_n4;
        locals.var_t0_dn5 = assign91320_e140020_d_n5;
        locals.var_t0_dn6 = assign91320_e140020_d_n6;
        locals.var_t0_dn7 = assign91320_e140020_d_n7;
        locals.var_t0_dn8 = assign91320_e140020_d_n8;
        locals.var_t0_dn9 = assign91320_e140020_d_n9;
        locals.var_t0_dn10 = assign91320_e140020_d_n10;
        locals.var_t0_dn13 = assign91320_e140020_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign91330_e140034, assign91330_e140034_d_n0, assign91330_e140034_d_n2, assign91330_e140034_d_n4, assign91330_e140034_d_n5, assign91330_e140034_d_n6, assign91330_e140034_d_n7, assign91330_e140034_d_n8, assign91330_e140034_d_n9, assign91330_e140034_d_n10, assign91330_e140034_d_n13,) = {
    if (((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_flg_never_reach_vfbover != 0.0)) {
        let assign91330_e140030: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign91330_e140031: f64 = (0.5 * assign91330_e140030);
        let assign91330_e140032: f64 = (0.5 + assign91330_e140031);
        (assign91330_e140032, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_tmf1_dn13 + locals.var_tmf2_dn13)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign91330_e140034;
        locals.var_t1_dn0 = assign91330_e140034_d_n0;
        locals.var_t1_dn2 = assign91330_e140034_d_n2;
        locals.var_t1_dn4 = assign91330_e140034_d_n4;
        locals.var_t1_dn5 = assign91330_e140034_d_n5;
        locals.var_t1_dn6 = assign91330_e140034_d_n6;
        locals.var_t1_dn7 = assign91330_e140034_d_n7;
        locals.var_t1_dn8 = assign91330_e140034_d_n8;
        locals.var_t1_dn9 = assign91330_e140034_d_n9;
        locals.var_t1_dn10 = assign91330_e140034_d_n10;
        locals.var_t1_dn13 = assign91330_e140034_d_n13;
        locals.var_t1_rv = 0.0;

        let assign91340_e140037: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign91340_e140040: f64 = (-locals.var_t1);
        let assign91340_e140045: f64 = if ((assign91340_e140037 > assign91340_e140040) && (locals.var_t1 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard2133 = assign91340_e140045;
        locals.var_guard2133_rv = 0.0;

        let (assign91350_e140061, assign91350_e140061_d_n0, assign91350_e140061_d_n2, assign91350_e140061_d_n4, assign91350_e140061_d_n5, assign91350_e140061_d_n6, assign91350_e140061_d_n7, assign91350_e140061_d_n8, assign91350_e140061_d_n9, assign91350_e140061_d_n10, assign91350_e140061_d_n13,) = {
    if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2133 != 0.0)) {
        let assign91350_e140055: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign91350_e140057: f64 = assign91350_e140055;
        let assign91350_e140059: f64 = (assign91350_e140057 + locals.var_t1);
        (assign91350_e140059, (locals.var_vxbgmtcl_dn0 + locals.var_t1_dn0), ((locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2) + locals.var_t1_dn2), (locals.var_vxbgmtcl_dn4 + locals.var_t1_dn4), (locals.var_vxbgmtcl_dn5 + locals.var_t1_dn5), ((locals.var_vgpld_dn6 + locals.var_vxbgmtcl_dn6) + locals.var_t1_dn6), ((locals.var_vgpld_dn7 + locals.var_vxbgmtcl_dn7) + locals.var_t1_dn7), ((locals.var_vgpld_dn8 + locals.var_vxbgmtcl_dn8) + locals.var_t1_dn8), (locals.var_vxbgmtcl_dn9 + locals.var_t1_dn9), (locals.var_vxbgmtcl_dn10 + locals.var_t1_dn10), (locals.var_vxbgmtcl_dn13 + locals.var_t1_dn13),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign91350_e140061;
        locals.var_tmf1_dn0 = assign91350_e140061_d_n0;
        locals.var_tmf1_dn2 = assign91350_e140061_d_n2;
        locals.var_tmf1_dn4 = assign91350_e140061_d_n4;
        locals.var_tmf1_dn5 = assign91350_e140061_d_n5;
        locals.var_tmf1_dn6 = assign91350_e140061_d_n6;
        locals.var_tmf1_dn7 = assign91350_e140061_d_n7;
        locals.var_tmf1_dn8 = assign91350_e140061_d_n8;
        locals.var_tmf1_dn9 = assign91350_e140061_d_n9;
        locals.var_tmf1_dn10 = assign91350_e140061_d_n10;
        locals.var_tmf1_dn13 = assign91350_e140061_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign91360_e140073, assign91360_e140073_d_n0, assign91360_e140073_d_n2, assign91360_e140073_d_n4, assign91360_e140073_d_n5, assign91360_e140073_d_n6, assign91360_e140073_d_n7, assign91360_e140073_d_n8, assign91360_e140073_d_n9, assign91360_e140073_d_n10, assign91360_e140073_d_n13,) = {
    if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2133 != 0.0)) {
        let assign91360_e140071: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign91360_e140071, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn13,)
    }
};
        locals.var_x2 = assign91360_e140073;
        locals.var_x2_dn0 = assign91360_e140073_d_n0;
        locals.var_x2_dn2 = assign91360_e140073_d_n2;
        locals.var_x2_dn4 = assign91360_e140073_d_n4;
        locals.var_x2_dn5 = assign91360_e140073_d_n5;
        locals.var_x2_dn6 = assign91360_e140073_d_n6;
        locals.var_x2_dn7 = assign91360_e140073_d_n7;
        locals.var_x2_dn8 = assign91360_e140073_d_n8;
        locals.var_x2_dn9 = assign91360_e140073_d_n9;
        locals.var_x2_dn10 = assign91360_e140073_d_n10;
        locals.var_x2_dn13 = assign91360_e140073_d_n13;
        locals.var_x2_rv = 0.0;

        let (assign91370_e140085, assign91370_e140085_d_n0, assign91370_e140085_d_n2, assign91370_e140085_d_n4, assign91370_e140085_d_n5, assign91370_e140085_d_n6, assign91370_e140085_d_n7, assign91370_e140085_d_n8, assign91370_e140085_d_n9, assign91370_e140085_d_n10, assign91370_e140085_d_n13,) = {
    if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2133 != 0.0)) {
        let assign91370_e140083: f64 = (locals.var_t1 * locals.var_t1);
        (assign91370_e140083, ((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0)), ((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2)), ((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)), ((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)), ((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)), ((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)), ((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)), ((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)), ((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)), ((locals.var_t1_dn13 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn13)),)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn13,)
    }
};
        locals.var_xmax2 = assign91370_e140085;
        locals.var_xmax2_dn0 = assign91370_e140085_d_n0;
        locals.var_xmax2_dn2 = assign91370_e140085_d_n2;
        locals.var_xmax2_dn4 = assign91370_e140085_d_n4;
        locals.var_xmax2_dn5 = assign91370_e140085_d_n5;
        locals.var_xmax2_dn6 = assign91370_e140085_d_n6;
        locals.var_xmax2_dn7 = assign91370_e140085_d_n7;
        locals.var_xmax2_dn8 = assign91370_e140085_d_n8;
        locals.var_xmax2_dn9 = assign91370_e140085_d_n9;
        locals.var_xmax2_dn10 = assign91370_e140085_d_n10;
        locals.var_xmax2_dn13 = assign91370_e140085_d_n13;
        locals.var_xmax2_rv = 0.0;

        let (assign91380_e140095, assign91380_e140095_d_n0, assign91380_e140095_d_n2, assign91380_e140095_d_n4, assign91380_e140095_d_n5, assign91380_e140095_d_n6, assign91380_e140095_d_n7, assign91380_e140095_d_n8, assign91380_e140095_d_n9, assign91380_e140095_d_n10, assign91380_e140095_d_n13,) = {
    if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2133 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign91380_e140095;
        locals.var_xp_dn0 = assign91380_e140095_d_n0;
        locals.var_xp_dn2 = assign91380_e140095_d_n2;
        locals.var_xp_dn4 = assign91380_e140095_d_n4;
        locals.var_xp_dn5 = assign91380_e140095_d_n5;
        locals.var_xp_dn6 = assign91380_e140095_d_n6;
        locals.var_xp_dn7 = assign91380_e140095_d_n7;
        locals.var_xp_dn8 = assign91380_e140095_d_n8;
        locals.var_xp_dn9 = assign91380_e140095_d_n9;
        locals.var_xp_dn10 = assign91380_e140095_d_n10;
        locals.var_xp_dn13 = assign91380_e140095_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign91390_e140105, assign91390_e140105_d_n0, assign91390_e140105_d_n2, assign91390_e140105_d_n4, assign91390_e140105_d_n5, assign91390_e140105_d_n6, assign91390_e140105_d_n7, assign91390_e140105_d_n8, assign91390_e140105_d_n9, assign91390_e140105_d_n10, assign91390_e140105_d_n13,) = {
    if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2133 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign91390_e140105;
        locals.var_xmp_dn0 = assign91390_e140105_d_n0;
        locals.var_xmp_dn2 = assign91390_e140105_d_n2;
        locals.var_xmp_dn4 = assign91390_e140105_d_n4;
        locals.var_xmp_dn5 = assign91390_e140105_d_n5;
        locals.var_xmp_dn6 = assign91390_e140105_d_n6;
        locals.var_xmp_dn7 = assign91390_e140105_d_n7;
        locals.var_xmp_dn8 = assign91390_e140105_d_n8;
        locals.var_xmp_dn9 = assign91390_e140105_d_n9;
        locals.var_xmp_dn10 = assign91390_e140105_d_n10;
        locals.var_xmp_dn13 = assign91390_e140105_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign91400_e140115,) = {
    if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2133 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign91400_e140115;
        locals.var_m0_rv = 0.0;

        let (assign91410_e140125,) = {
    if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2133 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign91410_e140125;
        locals.var_mm_rv = 0.0;

        let (assign91420_e140135, assign91420_e140135_d_n0, assign91420_e140135_d_n2, assign91420_e140135_d_n4, assign91420_e140135_d_n5, assign91420_e140135_d_n6, assign91420_e140135_d_n7, assign91420_e140135_d_n8, assign91420_e140135_d_n9, assign91420_e140135_d_n10, assign91420_e140135_d_n13,) = {
    if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2133 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign91420_e140135;
        locals.var_arg_dn0 = assign91420_e140135_d_n0;
        locals.var_arg_dn2 = assign91420_e140135_d_n2;
        locals.var_arg_dn4 = assign91420_e140135_d_n4;
        locals.var_arg_dn5 = assign91420_e140135_d_n5;
        locals.var_arg_dn6 = assign91420_e140135_d_n6;
        locals.var_arg_dn7 = assign91420_e140135_d_n7;
        locals.var_arg_dn8 = assign91420_e140135_d_n8;
        locals.var_arg_dn9 = assign91420_e140135_d_n9;
        locals.var_arg_dn10 = assign91420_e140135_d_n10;
        locals.var_arg_dn13 = assign91420_e140135_d_n13;
        locals.var_arg_rv = 0.0;

        let (assign91430_e140145, assign91430_e140145_d_n0, assign91430_e140145_d_n2, assign91430_e140145_d_n4, assign91430_e140145_d_n5, assign91430_e140145_d_n6, assign91430_e140145_d_n7, assign91430_e140145_d_n8, assign91430_e140145_d_n9, assign91430_e140145_d_n10, assign91430_e140145_d_n13,) = {
    if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2133 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign91430_e140145;
        locals.var_dnm_dn0 = assign91430_e140145_d_n0;
        locals.var_dnm_dn2 = assign91430_e140145_d_n2;
        locals.var_dnm_dn4 = assign91430_e140145_d_n4;
        locals.var_dnm_dn5 = assign91430_e140145_d_n5;
        locals.var_dnm_dn6 = assign91430_e140145_d_n6;
        locals.var_dnm_dn7 = assign91430_e140145_d_n7;
        locals.var_dnm_dn8 = assign91430_e140145_d_n8;
        locals.var_dnm_dn9 = assign91430_e140145_d_n9;
        locals.var_dnm_dn10 = assign91430_e140145_d_n10;
        locals.var_dnm_dn13 = assign91430_e140145_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign91440_e140157, assign91440_e140157_d_n0, assign91440_e140157_d_n2, assign91440_e140157_d_n4, assign91440_e140157_d_n5, assign91440_e140157_d_n6, assign91440_e140157_d_n7, assign91440_e140157_d_n8, assign91440_e140157_d_n9, assign91440_e140157_d_n10, assign91440_e140157_d_n13,) = {
    if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2133 != 0.0)) {
        let assign91440_e140155: f64 = (locals.var_xp * locals.var_x2);
        (assign91440_e140155, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign91440_e140157;
        locals.var_xp_dn0 = assign91440_e140157_d_n0;
        locals.var_xp_dn2 = assign91440_e140157_d_n2;
        locals.var_xp_dn4 = assign91440_e140157_d_n4;
        locals.var_xp_dn5 = assign91440_e140157_d_n5;
        locals.var_xp_dn6 = assign91440_e140157_d_n6;
        locals.var_xp_dn7 = assign91440_e140157_d_n7;
        locals.var_xp_dn8 = assign91440_e140157_d_n8;
        locals.var_xp_dn9 = assign91440_e140157_d_n9;
        locals.var_xp_dn10 = assign91440_e140157_d_n10;
        locals.var_xp_dn13 = assign91440_e140157_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign91450_e140169, assign91450_e140169_d_n0, assign91450_e140169_d_n2, assign91450_e140169_d_n4, assign91450_e140169_d_n5, assign91450_e140169_d_n6, assign91450_e140169_d_n7, assign91450_e140169_d_n8, assign91450_e140169_d_n9, assign91450_e140169_d_n10, assign91450_e140169_d_n13,) = {
    if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2133 != 0.0)) {
        let assign91450_e140167: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign91450_e140167, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign91450_e140169;
        locals.var_xmp_dn0 = assign91450_e140169_d_n0;
        locals.var_xmp_dn2 = assign91450_e140169_d_n2;
        locals.var_xmp_dn4 = assign91450_e140169_d_n4;
        locals.var_xmp_dn5 = assign91450_e140169_d_n5;
        locals.var_xmp_dn6 = assign91450_e140169_d_n6;
        locals.var_xmp_dn7 = assign91450_e140169_d_n7;
        locals.var_xmp_dn8 = assign91450_e140169_d_n8;
        locals.var_xmp_dn9 = assign91450_e140169_d_n9;
        locals.var_xmp_dn10 = assign91450_e140169_d_n10;
        locals.var_xmp_dn13 = assign91450_e140169_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign91460_e140181, assign91460_e140181_d_n0, assign91460_e140181_d_n2, assign91460_e140181_d_n4, assign91460_e140181_d_n5, assign91460_e140181_d_n6, assign91460_e140181_d_n7, assign91460_e140181_d_n8, assign91460_e140181_d_n9, assign91460_e140181_d_n10, assign91460_e140181_d_n13,) = {
    if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2133 != 0.0)) {
        let assign91460_e140179: f64 = (locals.var_xp + locals.var_xmp);
        (assign91460_e140179, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn13 + locals.var_xmp_dn13),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign91460_e140181;
        locals.var_arg_dn0 = assign91460_e140181_d_n0;
        locals.var_arg_dn2 = assign91460_e140181_d_n2;
        locals.var_arg_dn4 = assign91460_e140181_d_n4;
        locals.var_arg_dn5 = assign91460_e140181_d_n5;
        locals.var_arg_dn6 = assign91460_e140181_d_n6;
        locals.var_arg_dn7 = assign91460_e140181_d_n7;
        locals.var_arg_dn8 = assign91460_e140181_d_n8;
        locals.var_arg_dn9 = assign91460_e140181_d_n9;
        locals.var_arg_dn10 = assign91460_e140181_d_n10;
        locals.var_arg_dn13 = assign91460_e140181_d_n13;
        locals.var_arg_rv = 0.0;

        let (assign91470_e140191, assign91470_e140191_d_n0, assign91470_e140191_d_n2, assign91470_e140191_d_n4, assign91470_e140191_d_n5, assign91470_e140191_d_n6, assign91470_e140191_d_n7, assign91470_e140191_d_n8, assign91470_e140191_d_n9, assign91470_e140191_d_n10, assign91470_e140191_d_n13,) = {
    if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2133 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign91470_e140191;
        locals.var_dnm_dn0 = assign91470_e140191_d_n0;
        locals.var_dnm_dn2 = assign91470_e140191_d_n2;
        locals.var_dnm_dn4 = assign91470_e140191_d_n4;
        locals.var_dnm_dn5 = assign91470_e140191_d_n5;
        locals.var_dnm_dn6 = assign91470_e140191_d_n6;
        locals.var_dnm_dn7 = assign91470_e140191_d_n7;
        locals.var_dnm_dn8 = assign91470_e140191_d_n8;
        locals.var_dnm_dn9 = assign91470_e140191_d_n9;
        locals.var_dnm_dn10 = assign91470_e140191_d_n10;
        locals.var_dnm_dn13 = assign91470_e140191_d_n13;
        locals.var_dnm_rv = 0.0;

        let assign91480_e140206: f64 = if ((((1.0 == 1.0) || (1.0 == 2.0)) || (1.0 == 4.0)) || (1.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard2134 = assign91480_e140206;
        locals.var_guard2134_rv = 0.0;

        let assign91490_e140209: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard2135 = assign91490_e140209;
        locals.var_guard2135_rv = 0.0;

        let (assign91500_e140223,) = {
    if ((((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2133 != 0.0)) && (locals.var_guard2134 != 0.0)) && (locals.var_guard2135 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign91500_e140223;
        locals.var_mm_rv = 0.0;

        let assign91510_e140226: f64 = if 1.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard2136 = assign91510_e140226;
        locals.var_guard2136_rv = 0.0;

        let (assign91520_e140243,) = {
    if (((((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2133 != 0.0)) && (locals.var_guard2134 != 0.0)) && (locals.var_guard2135 == 0.0)) && (locals.var_guard2136 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign91520_e140243;
        locals.var_mm_rv = 0.0;

        let assign91530_e140246: f64 = if 1.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard2137 = assign91530_e140246;
        locals.var_guard2137_rv = 0.0;

        let (assign91540_e140266,) = {
    if ((((((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2133 != 0.0)) && (locals.var_guard2134 != 0.0)) && (locals.var_guard2135 == 0.0)) && (locals.var_guard2136 == 0.0)) && (locals.var_guard2137 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign91540_e140266;
        locals.var_mm_rv = 0.0;

        let assign91550_e140269: f64 = if 1.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard2138 = assign91550_e140269;
        locals.var_guard2138_rv = 0.0;

        let (assign91560_e140292,) = {
    if (((((((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2133 != 0.0)) && (locals.var_guard2134 != 0.0)) && (locals.var_guard2135 == 0.0)) && (locals.var_guard2136 == 0.0)) && (locals.var_guard2137 == 0.0)) && (locals.var_guard2138 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign91560_e140292;
        locals.var_mm_rv = 0.0;

        let (assign91570_e140304,) = {
    if (((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2133 != 0.0)) && (locals.var_guard2134 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign91570_e140304;
        locals.var_m0_rv = 0.0;

        let mut assign91580_loop_guard: usize = 0;
        while {
            let assign91580_cond_e140317: f64 = if ((((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2133 != 0.0)) && (locals.var_guard2134 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign91580_cond_e140317 != 0.0
        } {
            assign91580_loop_guard += 1;
            assert!(assign91580_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign91580_body0_e140330, assign91580_body0_e140330_d_n0, assign91580_body0_e140330_d_n2, assign91580_body0_e140330_d_n4, assign91580_body0_e140330_d_n5, assign91580_body0_e140330_d_n6, assign91580_body0_e140330_d_n7, assign91580_body0_e140330_d_n8, assign91580_body0_e140330_d_n9, assign91580_body0_e140330_d_n10, assign91580_body0_e140330_d_n13,) = {
    if (((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2133 != 0.0)) && (locals.var_guard2134 != 0.0)) {
        let assign91580_body0_e140328: f64 = (locals.var_dnm).sqrt();
        (assign91580_body0_e140328, (locals.var_dnm_dn0 / (2.0 * assign91580_body0_e140328)), (locals.var_dnm_dn2 / (2.0 * assign91580_body0_e140328)), (locals.var_dnm_dn4 / (2.0 * assign91580_body0_e140328)), (locals.var_dnm_dn5 / (2.0 * assign91580_body0_e140328)), (locals.var_dnm_dn6 / (2.0 * assign91580_body0_e140328)), (locals.var_dnm_dn7 / (2.0 * assign91580_body0_e140328)), (locals.var_dnm_dn8 / (2.0 * assign91580_body0_e140328)), (locals.var_dnm_dn9 / (2.0 * assign91580_body0_e140328)), (locals.var_dnm_dn10 / (2.0 * assign91580_body0_e140328)), (locals.var_dnm_dn13 / (2.0 * assign91580_body0_e140328)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
            locals.var_dnm = assign91580_body0_e140330;
            locals.var_dnm_dn0 = assign91580_body0_e140330_d_n0;
            locals.var_dnm_dn2 = assign91580_body0_e140330_d_n2;
            locals.var_dnm_dn4 = assign91580_body0_e140330_d_n4;
            locals.var_dnm_dn5 = assign91580_body0_e140330_d_n5;
            locals.var_dnm_dn6 = assign91580_body0_e140330_d_n6;
            locals.var_dnm_dn7 = assign91580_body0_e140330_d_n7;
            locals.var_dnm_dn8 = assign91580_body0_e140330_d_n8;
            locals.var_dnm_dn9 = assign91580_body0_e140330_d_n9;
            locals.var_dnm_dn10 = assign91580_body0_e140330_d_n10;
            locals.var_dnm_dn13 = assign91580_body0_e140330_d_n13;
            locals.var_dnm_rv = 0.0;
            let (assign91580_body1_e140344,) = {
    if (((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2133 != 0.0)) && (locals.var_guard2134 != 0.0)) {
        let assign91580_body1_e140342: f64 = (locals.var_m0 + 1.0);
        (assign91580_body1_e140342,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign91580_body1_e140344;
            locals.var_m0_rv = 0.0;
        }

        let (assign91590_e140368, assign91590_e140368_d_n0, assign91590_e140368_d_n2, assign91590_e140368_d_n4, assign91590_e140368_d_n5, assign91590_e140368_d_n6, assign91590_e140368_d_n7, assign91590_e140368_d_n8, assign91590_e140368_d_n9, assign91590_e140368_d_n10, assign91590_e140368_d_n13,) = {
    if (((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2133 != 0.0)) && (locals.var_guard2134 == 0.0)) {
        let (assign91590_e140366, assign91590_e140366_d_n0, assign91590_e140366_d_n2, assign91590_e140366_d_n4, assign91590_e140366_d_n5, assign91590_e140366_d_n6, assign91590_e140366_d_n7, assign91590_e140366_d_n8, assign91590_e140366_d_n9, assign91590_e140366_d_n10, assign91590_e140366_d_n13,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign91590_e140363: f64 = 2.0;
                let assign91590_e140364: f64 = (1.0 / assign91590_e140363);
                let assign91590_e140365: f64 = (locals.var_dnm).powf(assign91590_e140364);
                (assign91590_e140365, if 0.0 == 0.0 && ((assign91590_e140364) as f64).is_finite() && ((assign91590_e140364) as f64).fract() == 0.0 { if assign91590_e140364 == 0.0 { 0.0 } else { (assign91590_e140364 * ((locals.var_dnm).powf(assign91590_e140364 - 1.0) * locals.var_dnm_dn0)) } } else { (assign91590_e140365 * (assign91590_e140364 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign91590_e140364) as f64).is_finite() && ((assign91590_e140364) as f64).fract() == 0.0 { if assign91590_e140364 == 0.0 { 0.0 } else { (assign91590_e140364 * ((locals.var_dnm).powf(assign91590_e140364 - 1.0) * locals.var_dnm_dn2)) } } else { (assign91590_e140365 * (assign91590_e140364 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign91590_e140364) as f64).is_finite() && ((assign91590_e140364) as f64).fract() == 0.0 { if assign91590_e140364 == 0.0 { 0.0 } else { (assign91590_e140364 * ((locals.var_dnm).powf(assign91590_e140364 - 1.0) * locals.var_dnm_dn4)) } } else { (assign91590_e140365 * (assign91590_e140364 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign91590_e140364) as f64).is_finite() && ((assign91590_e140364) as f64).fract() == 0.0 { if assign91590_e140364 == 0.0 { 0.0 } else { (assign91590_e140364 * ((locals.var_dnm).powf(assign91590_e140364 - 1.0) * locals.var_dnm_dn5)) } } else { (assign91590_e140365 * (assign91590_e140364 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign91590_e140364) as f64).is_finite() && ((assign91590_e140364) as f64).fract() == 0.0 { if assign91590_e140364 == 0.0 { 0.0 } else { (assign91590_e140364 * ((locals.var_dnm).powf(assign91590_e140364 - 1.0) * locals.var_dnm_dn6)) } } else { (assign91590_e140365 * (assign91590_e140364 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign91590_e140364) as f64).is_finite() && ((assign91590_e140364) as f64).fract() == 0.0 { if assign91590_e140364 == 0.0 { 0.0 } else { (assign91590_e140364 * ((locals.var_dnm).powf(assign91590_e140364 - 1.0) * locals.var_dnm_dn7)) } } else { (assign91590_e140365 * (assign91590_e140364 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign91590_e140364) as f64).is_finite() && ((assign91590_e140364) as f64).fract() == 0.0 { if assign91590_e140364 == 0.0 { 0.0 } else { (assign91590_e140364 * ((locals.var_dnm).powf(assign91590_e140364 - 1.0) * locals.var_dnm_dn8)) } } else { (assign91590_e140365 * (assign91590_e140364 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign91590_e140364) as f64).is_finite() && ((assign91590_e140364) as f64).fract() == 0.0 { if assign91590_e140364 == 0.0 { 0.0 } else { (assign91590_e140364 * ((locals.var_dnm).powf(assign91590_e140364 - 1.0) * locals.var_dnm_dn9)) } } else { (assign91590_e140365 * (assign91590_e140364 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign91590_e140364) as f64).is_finite() && ((assign91590_e140364) as f64).fract() == 0.0 { if assign91590_e140364 == 0.0 { 0.0 } else { (assign91590_e140364 * ((locals.var_dnm).powf(assign91590_e140364 - 1.0) * locals.var_dnm_dn10)) } } else { (assign91590_e140365 * (assign91590_e140364 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign91590_e140364) as f64).is_finite() && ((assign91590_e140364) as f64).fract() == 0.0 { if assign91590_e140364 == 0.0 { 0.0 } else { (assign91590_e140364 * ((locals.var_dnm).powf(assign91590_e140364 - 1.0) * locals.var_dnm_dn13)) } } else { (assign91590_e140365 * (assign91590_e140364 * (locals.var_dnm_dn13 / locals.var_dnm))) },)
            }
        };
        (assign91590_e140366, assign91590_e140366_d_n0, assign91590_e140366_d_n2, assign91590_e140366_d_n4, assign91590_e140366_d_n5, assign91590_e140366_d_n6, assign91590_e140366_d_n7, assign91590_e140366_d_n8, assign91590_e140366_d_n9, assign91590_e140366_d_n10, assign91590_e140366_d_n13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign91590_e140368;
        locals.var_dnm_dn0 = assign91590_e140368_d_n0;
        locals.var_dnm_dn2 = assign91590_e140368_d_n2;
        locals.var_dnm_dn4 = assign91590_e140368_d_n4;
        locals.var_dnm_dn5 = assign91590_e140368_d_n5;
        locals.var_dnm_dn6 = assign91590_e140368_d_n6;
        locals.var_dnm_dn7 = assign91590_e140368_d_n7;
        locals.var_dnm_dn8 = assign91590_e140368_d_n8;
        locals.var_dnm_dn9 = assign91590_e140368_d_n9;
        locals.var_dnm_dn10 = assign91590_e140368_d_n10;
        locals.var_dnm_dn13 = assign91590_e140368_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign91600_e140380, assign91600_e140380_d_n0, assign91600_e140380_d_n2, assign91600_e140380_d_n4, assign91600_e140380_d_n5, assign91600_e140380_d_n6, assign91600_e140380_d_n7, assign91600_e140380_d_n8, assign91600_e140380_d_n9, assign91600_e140380_d_n10, assign91600_e140380_d_n13,) = {
    if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2133 != 0.0)) {
        let assign91600_e140378: f64 = (1.0 / locals.var_dnm);
        (assign91600_e140378, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn13 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign91600_e140380;
        locals.var_dnm_dn0 = assign91600_e140380_d_n0;
        locals.var_dnm_dn2 = assign91600_e140380_d_n2;
        locals.var_dnm_dn4 = assign91600_e140380_d_n4;
        locals.var_dnm_dn5 = assign91600_e140380_d_n5;
        locals.var_dnm_dn6 = assign91600_e140380_d_n6;
        locals.var_dnm_dn7 = assign91600_e140380_d_n7;
        locals.var_dnm_dn8 = assign91600_e140380_d_n8;
        locals.var_dnm_dn9 = assign91600_e140380_d_n9;
        locals.var_dnm_dn10 = assign91600_e140380_d_n10;
        locals.var_dnm_dn13 = assign91600_e140380_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign91610_e140394, assign91610_e140394_d_n0, assign91610_e140394_d_n2, assign91610_e140394_d_n4, assign91610_e140394_d_n5, assign91610_e140394_d_n6, assign91610_e140394_d_n7, assign91610_e140394_d_n8, assign91610_e140394_d_n9, assign91610_e140394_d_n10, assign91610_e140394_d_n13,) = {
    if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2133 != 0.0)) {
        let assign91610_e140390: f64 = (locals.var_tmf1 * locals.var_t1);
        let assign91610_e140392: f64 = (assign91610_e140390 * locals.var_dnm);
        (assign91610_e140392, ((((locals.var_tmf1_dn0 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn0)) * locals.var_dnm) + (assign91610_e140390 * locals.var_dnm_dn0)), ((((locals.var_tmf1_dn2 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn2)) * locals.var_dnm) + (assign91610_e140390 * locals.var_dnm_dn2)), ((((locals.var_tmf1_dn4 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn4)) * locals.var_dnm) + (assign91610_e140390 * locals.var_dnm_dn4)), ((((locals.var_tmf1_dn5 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn5)) * locals.var_dnm) + (assign91610_e140390 * locals.var_dnm_dn5)), ((((locals.var_tmf1_dn6 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn6)) * locals.var_dnm) + (assign91610_e140390 * locals.var_dnm_dn6)), ((((locals.var_tmf1_dn7 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn7)) * locals.var_dnm) + (assign91610_e140390 * locals.var_dnm_dn7)), ((((locals.var_tmf1_dn8 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn8)) * locals.var_dnm) + (assign91610_e140390 * locals.var_dnm_dn8)), ((((locals.var_tmf1_dn9 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn9)) * locals.var_dnm) + (assign91610_e140390 * locals.var_dnm_dn9)), ((((locals.var_tmf1_dn10 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn10)) * locals.var_dnm) + (assign91610_e140390 * locals.var_dnm_dn10)), ((((locals.var_tmf1_dn13 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn13)) * locals.var_dnm) + (assign91610_e140390 * locals.var_dnm_dn13)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn13,)
    }
};
        locals.var_tmf0 = assign91610_e140394;
        locals.var_tmf0_dn0 = assign91610_e140394_d_n0;
        locals.var_tmf0_dn2 = assign91610_e140394_d_n2;
        locals.var_tmf0_dn4 = assign91610_e140394_d_n4;
        locals.var_tmf0_dn5 = assign91610_e140394_d_n5;
        locals.var_tmf0_dn6 = assign91610_e140394_d_n6;
        locals.var_tmf0_dn7 = assign91610_e140394_d_n7;
        locals.var_tmf0_dn8 = assign91610_e140394_d_n8;
        locals.var_tmf0_dn9 = assign91610_e140394_d_n9;
        locals.var_tmf0_dn10 = assign91610_e140394_d_n10;
        locals.var_tmf0_dn13 = assign91610_e140394_d_n13;
        locals.var_tmf0_rv = 0.0;

    }
}
