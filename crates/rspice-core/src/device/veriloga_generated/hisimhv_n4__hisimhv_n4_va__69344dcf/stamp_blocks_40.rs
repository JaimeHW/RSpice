#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_262(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign72660_e109717, assign72660_e109717_d_n0, assign72660_e109717_d_n2, assign72660_e109717_d_n4, assign72660_e109717_d_n5, assign72660_e109717_d_n6, assign72660_e109717_d_n7, assign72660_e109717_d_n8, assign72660_e109717_d_n9, assign72660_e109717_d_n10, assign72660_e109717_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1684 != 0.0)) {
        let assign72660_e109711: f64 = (locals.var_chi - 1.0);
        let assign72660_e109713: f64 = (-locals.var_chi);
        let assign72660_e109714: f64 = (assign72660_e109713).exp();
        let assign72660_e109715: f64 = (assign72660_e109711 + assign72660_e109714);
        (assign72660_e109715, (locals.var_chi_dn0 + (assign72660_e109714 * (-locals.var_chi_dn0))), (locals.var_chi_dn2 + (assign72660_e109714 * (-locals.var_chi_dn2))), (locals.var_chi_dn4 + (assign72660_e109714 * (-locals.var_chi_dn4))), (locals.var_chi_dn5 + (assign72660_e109714 * (-locals.var_chi_dn5))), (locals.var_chi_dn6 + (assign72660_e109714 * (-locals.var_chi_dn6))), (locals.var_chi_dn7 + (assign72660_e109714 * (-locals.var_chi_dn7))), (locals.var_chi_dn8 + (assign72660_e109714 * (-locals.var_chi_dn8))), (locals.var_chi_dn9 + (assign72660_e109714 * (-locals.var_chi_dn9))), (locals.var_chi_dn10 + (assign72660_e109714 * (-locals.var_chi_dn10))), (locals.var_chi_dn13 + (assign72660_e109714 * (-locals.var_chi_dn13))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign72660_e109717;
        locals.var_t1_dn0 = assign72660_e109717_d_n0;
        locals.var_t1_dn2 = assign72660_e109717_d_n2;
        locals.var_t1_dn4 = assign72660_e109717_d_n4;
        locals.var_t1_dn5 = assign72660_e109717_d_n5;
        locals.var_t1_dn6 = assign72660_e109717_d_n6;
        locals.var_t1_dn7 = assign72660_e109717_d_n7;
        locals.var_t1_dn8 = assign72660_e109717_d_n8;
        locals.var_t1_dn9 = assign72660_e109717_d_n9;
        locals.var_t1_dn10 = assign72660_e109717_d_n10;
        locals.var_t1_dn13 = assign72660_e109717_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign72670_e109727, assign72670_e109727_d_n0, assign72670_e109727_d_n2, assign72670_e109727_d_n4, assign72670_e109727_d_n5, assign72670_e109727_d_n6, assign72670_e109727_d_n7, assign72670_e109727_d_n8, assign72670_e109727_d_n9, assign72670_e109727_d_n10, assign72670_e109727_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1684 != 0.0)) {
        let assign72670_e109725: f64 = (locals.var_t1).sqrt();
        (assign72670_e109725, (locals.var_t1_dn0 / (2.0 * assign72670_e109725)), (locals.var_t1_dn2 / (2.0 * assign72670_e109725)), (locals.var_t1_dn4 / (2.0 * assign72670_e109725)), (locals.var_t1_dn5 / (2.0 * assign72670_e109725)), (locals.var_t1_dn6 / (2.0 * assign72670_e109725)), (locals.var_t1_dn7 / (2.0 * assign72670_e109725)), (locals.var_t1_dn8 / (2.0 * assign72670_e109725)), (locals.var_t1_dn9 / (2.0 * assign72670_e109725)), (locals.var_t1_dn10 / (2.0 * assign72670_e109725)), (locals.var_t1_dn13 / (2.0 * assign72670_e109725)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign72670_e109727;
        locals.var_t2_dn0 = assign72670_e109727_d_n0;
        locals.var_t2_dn2 = assign72670_e109727_d_n2;
        locals.var_t2_dn4 = assign72670_e109727_d_n4;
        locals.var_t2_dn5 = assign72670_e109727_d_n5;
        locals.var_t2_dn6 = assign72670_e109727_d_n6;
        locals.var_t2_dn7 = assign72670_e109727_d_n7;
        locals.var_t2_dn8 = assign72670_e109727_d_n8;
        locals.var_t2_dn9 = assign72670_e109727_d_n9;
        locals.var_t2_dn10 = assign72670_e109727_d_n10;
        locals.var_t2_dn13 = assign72670_e109727_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign72690_e109758, assign72690_e109758_d_n0, assign72690_e109758_d_n2, assign72690_e109758_d_n4, assign72690_e109758_d_n5, assign72690_e109758_d_n6, assign72690_e109758_d_n7, assign72690_e109758_d_n8, assign72690_e109758_d_n9, assign72690_e109758_d_n10, assign72690_e109758_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1684 == 0.0)) {
        let assign72690_e109749: f64 = (0.7071067811865475 * locals.var_chi);
        let assign72690_e109753: f64 = (locals.var_chi * 0.3333333333333333);
        let assign72690_e109754: f64 = (1.0 - assign72690_e109753);
        let assign72690_e109755: f64 = (assign72690_e109754).sqrt();
        let assign72690_e109756: f64 = (assign72690_e109749 * assign72690_e109755);
        (assign72690_e109756, (((0.7071067811865475 * locals.var_chi_dn0) * assign72690_e109755) + (assign72690_e109749 * ((-(locals.var_chi_dn0 * 0.3333333333333333)) / (2.0 * assign72690_e109755)))), (((0.7071067811865475 * locals.var_chi_dn2) * assign72690_e109755) + (assign72690_e109749 * ((-(locals.var_chi_dn2 * 0.3333333333333333)) / (2.0 * assign72690_e109755)))), (((0.7071067811865475 * locals.var_chi_dn4) * assign72690_e109755) + (assign72690_e109749 * ((-(locals.var_chi_dn4 * 0.3333333333333333)) / (2.0 * assign72690_e109755)))), (((0.7071067811865475 * locals.var_chi_dn5) * assign72690_e109755) + (assign72690_e109749 * ((-(locals.var_chi_dn5 * 0.3333333333333333)) / (2.0 * assign72690_e109755)))), (((0.7071067811865475 * locals.var_chi_dn6) * assign72690_e109755) + (assign72690_e109749 * ((-(locals.var_chi_dn6 * 0.3333333333333333)) / (2.0 * assign72690_e109755)))), (((0.7071067811865475 * locals.var_chi_dn7) * assign72690_e109755) + (assign72690_e109749 * ((-(locals.var_chi_dn7 * 0.3333333333333333)) / (2.0 * assign72690_e109755)))), (((0.7071067811865475 * locals.var_chi_dn8) * assign72690_e109755) + (assign72690_e109749 * ((-(locals.var_chi_dn8 * 0.3333333333333333)) / (2.0 * assign72690_e109755)))), (((0.7071067811865475 * locals.var_chi_dn9) * assign72690_e109755) + (assign72690_e109749 * ((-(locals.var_chi_dn9 * 0.3333333333333333)) / (2.0 * assign72690_e109755)))), (((0.7071067811865475 * locals.var_chi_dn10) * assign72690_e109755) + (assign72690_e109749 * ((-(locals.var_chi_dn10 * 0.3333333333333333)) / (2.0 * assign72690_e109755)))), (((0.7071067811865475 * locals.var_chi_dn13) * assign72690_e109755) + (assign72690_e109749 * ((-(locals.var_chi_dn13 * 0.3333333333333333)) / (2.0 * assign72690_e109755)))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign72690_e109758;
        locals.var_t2_dn0 = assign72690_e109758_d_n0;
        locals.var_t2_dn2 = assign72690_e109758_d_n2;
        locals.var_t2_dn4 = assign72690_e109758_d_n4;
        locals.var_t2_dn5 = assign72690_e109758_d_n5;
        locals.var_t2_dn6 = assign72690_e109758_d_n6;
        locals.var_t2_dn7 = assign72690_e109758_d_n7;
        locals.var_t2_dn8 = assign72690_e109758_d_n8;
        locals.var_t2_dn9 = assign72690_e109758_d_n9;
        locals.var_t2_dn10 = assign72690_e109758_d_n10;
        locals.var_t2_dn13 = assign72690_e109758_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign72700_e109767, assign72700_e109767_d_n0, assign72700_e109767_d_n2, assign72700_e109767_d_n4, assign72700_e109767_d_n5, assign72700_e109767_d_n6, assign72700_e109767_d_n7, assign72700_e109767_d_n8, assign72700_e109767_d_n9, assign72700_e109767_d_n10, assign72700_e109767_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) {
        let assign72700_e109765: f64 = (locals.var_cnst0over_func * locals.var_t2);
        (assign72700_e109765, ((locals.var_cnst0over_func_dn0 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn0)), ((locals.var_cnst0over_func_dn2 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn2)), ((locals.var_cnst0over_func_dn4 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn4)), ((locals.var_cnst0over_func_dn5 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn5)), ((locals.var_cnst0over_func_dn6 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn6)), ((locals.var_cnst0over_func_dn7 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn7)), ((locals.var_cnst0over_func_dn8 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn8)), ((locals.var_cnst0over_func_dn9 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn9)), ((locals.var_cnst0over_func_dn10 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn10)), ((locals.var_cnst0over_func_dn13 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn13)),)
    } else {
        (locals.var_qbuld, locals.var_qbuld_dn0, locals.var_qbuld_dn2, locals.var_qbuld_dn4, locals.var_qbuld_dn5, locals.var_qbuld_dn6, locals.var_qbuld_dn7, locals.var_qbuld_dn8, locals.var_qbuld_dn9, locals.var_qbuld_dn10, locals.var_qbuld_dn13,)
    }
};
        locals.var_qbuld = assign72700_e109767;
        locals.var_qbuld_dn0 = assign72700_e109767_d_n0;
        locals.var_qbuld_dn2 = assign72700_e109767_d_n2;
        locals.var_qbuld_dn4 = assign72700_e109767_d_n4;
        locals.var_qbuld_dn5 = assign72700_e109767_d_n5;
        locals.var_qbuld_dn6 = assign72700_e109767_d_n6;
        locals.var_qbuld_dn7 = assign72700_e109767_d_n7;
        locals.var_qbuld_dn8 = assign72700_e109767_d_n8;
        locals.var_qbuld_dn9 = assign72700_e109767_d_n9;
        locals.var_qbuld_dn10 = assign72700_e109767_d_n10;
        locals.var_qbuld_dn13 = assign72700_e109767_d_n13;
        locals.var_qbuld_rv = 0.0;

        let (assign72710_e109778, assign72710_e109778_d_n0, assign72710_e109778_d_n2, assign72710_e109778_d_n4, assign72710_e109778_d_n5, assign72710_e109778_d_n6, assign72710_e109778_d_n7, assign72710_e109778_d_n8, assign72710_e109778_d_n9, assign72710_e109778_d_n10, assign72710_e109778_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) {
        let assign72710_e109775: f64 = (locals.var_vgpld - locals.var_ps0ld);
        let assign72710_e109776: f64 = (locals.var_cox0_func * assign72710_e109775);
        (assign72710_e109776, (locals.var_cox0_func * (-locals.var_ps0ld_dn0)), (locals.var_cox0_func * (locals.var_vgpld_dn2 - locals.var_ps0ld_dn2)), (locals.var_cox0_func * (-locals.var_ps0ld_dn4)), (locals.var_cox0_func * (-locals.var_ps0ld_dn5)), (locals.var_cox0_func * (locals.var_vgpld_dn6 - locals.var_ps0ld_dn6)), (locals.var_cox0_func * (locals.var_vgpld_dn7 - locals.var_ps0ld_dn7)), (locals.var_cox0_func * (locals.var_vgpld_dn8 - locals.var_ps0ld_dn8)), (locals.var_cox0_func * (-locals.var_ps0ld_dn9)), (locals.var_cox0_func * (-locals.var_ps0ld_dn10)), (locals.var_cox0_func * (-locals.var_ps0ld_dn13)),)
    } else {
        (locals.var_qsuld, locals.var_qsuld_dn0, locals.var_qsuld_dn2, locals.var_qsuld_dn4, locals.var_qsuld_dn5, locals.var_qsuld_dn6, locals.var_qsuld_dn7, locals.var_qsuld_dn8, locals.var_qsuld_dn9, locals.var_qsuld_dn10, locals.var_qsuld_dn13,)
    }
};
        locals.var_qsuld = assign72710_e109778;
        locals.var_qsuld_dn0 = assign72710_e109778_d_n0;
        locals.var_qsuld_dn2 = assign72710_e109778_d_n2;
        locals.var_qsuld_dn4 = assign72710_e109778_d_n4;
        locals.var_qsuld_dn5 = assign72710_e109778_d_n5;
        locals.var_qsuld_dn6 = assign72710_e109778_d_n6;
        locals.var_qsuld_dn7 = assign72710_e109778_d_n7;
        locals.var_qsuld_dn8 = assign72710_e109778_d_n8;
        locals.var_qsuld_dn9 = assign72710_e109778_d_n9;
        locals.var_qsuld_dn10 = assign72710_e109778_d_n10;
        locals.var_qsuld_dn13 = assign72710_e109778_d_n13;
        locals.var_qsuld_rv = 0.0;

        let (assign72720_e109787, assign72720_e109787_d_n0, assign72720_e109787_d_n2, assign72720_e109787_d_n4, assign72720_e109787_d_n5, assign72720_e109787_d_n6, assign72720_e109787_d_n7, assign72720_e109787_d_n8, assign72720_e109787_d_n9, assign72720_e109787_d_n10, assign72720_e109787_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) {
        let assign72720_e109785: f64 = (locals.var_qbuld / locals.var_q_nsubld);
        (assign72720_e109785, (locals.var_qbuld_dn0 / locals.var_q_nsubld), (locals.var_qbuld_dn2 / locals.var_q_nsubld), (locals.var_qbuld_dn4 / locals.var_q_nsubld), (locals.var_qbuld_dn5 / locals.var_q_nsubld), (locals.var_qbuld_dn6 / locals.var_q_nsubld), (locals.var_qbuld_dn7 / locals.var_q_nsubld), (locals.var_qbuld_dn8 / locals.var_q_nsubld), (locals.var_qbuld_dn9 / locals.var_q_nsubld), (locals.var_qbuld_dn10 / locals.var_q_nsubld), (locals.var_qbuld_dn13 / locals.var_q_nsubld),)
    } else {
        (locals.var_wdld0, locals.var_wdld0_dn0, locals.var_wdld0_dn2, locals.var_wdld0_dn4, locals.var_wdld0_dn5, locals.var_wdld0_dn6, locals.var_wdld0_dn7, locals.var_wdld0_dn8, locals.var_wdld0_dn9, locals.var_wdld0_dn10, locals.var_wdld0_dn13,)
    }
};
        locals.var_wdld0 = assign72720_e109787;
        locals.var_wdld0_dn0 = assign72720_e109787_d_n0;
        locals.var_wdld0_dn2 = assign72720_e109787_d_n2;
        locals.var_wdld0_dn4 = assign72720_e109787_d_n4;
        locals.var_wdld0_dn5 = assign72720_e109787_d_n5;
        locals.var_wdld0_dn6 = assign72720_e109787_d_n6;
        locals.var_wdld0_dn7 = assign72720_e109787_d_n7;
        locals.var_wdld0_dn8 = assign72720_e109787_d_n8;
        locals.var_wdld0_dn9 = assign72720_e109787_d_n9;
        locals.var_wdld0_dn10 = assign72720_e109787_d_n10;
        locals.var_wdld0_dn13 = assign72720_e109787_d_n13;
        locals.var_wdld0_rv = 0.0;

        let assign72730_e109790: f64 = if p.p33 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1687 = assign72730_e109790;
        locals.var_guard1687_rv = 0.0;

        let assign72740_e109795: f64 = (locals.var_ddriftldc * 0.1);
        let assign72740_e109796: f64 = (locals.var_ddriftldc - assign72740_e109795);
        let assign72740_e109800: f64 = (locals.var_ddriftldc * 0.1);
        let assign72740_e109803: f64 = if ((locals.var_wdld0 > assign72740_e109796) && (assign72740_e109800 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1688 = assign72740_e109803;
        locals.var_guard1688_rv = 0.0;

        let (assign72750_e109820, assign72750_e109820_d_n0, assign72750_e109820_d_n2, assign72750_e109820_d_n4, assign72750_e109820_d_n5, assign72750_e109820_d_n6, assign72750_e109820_d_n7, assign72750_e109820_d_n8, assign72750_e109820_d_n9, assign72750_e109820_d_n10, assign72750_e109820_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1687 != 0.0)) && (locals.var_guard1688 != 0.0)) {
        let assign72750_e109814: f64 = (locals.var_wdld0 - locals.var_ddriftldc);
        let assign72750_e109817: f64 = (locals.var_ddriftldc * 0.1);
        let assign72750_e109818: f64 = (assign72750_e109814 + assign72750_e109817);
        (assign72750_e109818, ((locals.var_wdld0_dn0 - locals.var_ddriftldc_dn0) + (locals.var_ddriftldc_dn0 * 0.1)), ((locals.var_wdld0_dn2 - locals.var_ddriftldc_dn2) + (locals.var_ddriftldc_dn2 * 0.1)), ((locals.var_wdld0_dn4 - locals.var_ddriftldc_dn4) + (locals.var_ddriftldc_dn4 * 0.1)), ((locals.var_wdld0_dn5 - locals.var_ddriftldc_dn5) + (locals.var_ddriftldc_dn5 * 0.1)), ((locals.var_wdld0_dn6 - locals.var_ddriftldc_dn6) + (locals.var_ddriftldc_dn6 * 0.1)), ((locals.var_wdld0_dn7 - locals.var_ddriftldc_dn7) + (locals.var_ddriftldc_dn7 * 0.1)), ((locals.var_wdld0_dn8 - locals.var_ddriftldc_dn8) + (locals.var_ddriftldc_dn8 * 0.1)), ((locals.var_wdld0_dn9 - locals.var_ddriftldc_dn9) + (locals.var_ddriftldc_dn9 * 0.1)), ((locals.var_wdld0_dn10 - locals.var_ddriftldc_dn10) + (locals.var_ddriftldc_dn10 * 0.1)), ((locals.var_wdld0_dn13 - locals.var_ddriftldc_dn13) + (locals.var_ddriftldc_dn13 * 0.1)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign72750_e109820;
        locals.var_tmf1_dn0 = assign72750_e109820_d_n0;
        locals.var_tmf1_dn2 = assign72750_e109820_d_n2;
        locals.var_tmf1_dn4 = assign72750_e109820_d_n4;
        locals.var_tmf1_dn5 = assign72750_e109820_d_n5;
        locals.var_tmf1_dn6 = assign72750_e109820_d_n6;
        locals.var_tmf1_dn7 = assign72750_e109820_d_n7;
        locals.var_tmf1_dn8 = assign72750_e109820_d_n8;
        locals.var_tmf1_dn9 = assign72750_e109820_d_n9;
        locals.var_tmf1_dn10 = assign72750_e109820_d_n10;
        locals.var_tmf1_dn13 = assign72750_e109820_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign72760_e109833, assign72760_e109833_d_n0, assign72760_e109833_d_n2, assign72760_e109833_d_n4, assign72760_e109833_d_n5, assign72760_e109833_d_n6, assign72760_e109833_d_n7, assign72760_e109833_d_n8, assign72760_e109833_d_n9, assign72760_e109833_d_n10, assign72760_e109833_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1687 != 0.0)) && (locals.var_guard1688 != 0.0)) {
        let assign72760_e109831: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign72760_e109831, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn13,)
    }
};
        locals.var_x2 = assign72760_e109833;
        locals.var_x2_dn0 = assign72760_e109833_d_n0;
        locals.var_x2_dn2 = assign72760_e109833_d_n2;
        locals.var_x2_dn4 = assign72760_e109833_d_n4;
        locals.var_x2_dn5 = assign72760_e109833_d_n5;
        locals.var_x2_dn6 = assign72760_e109833_d_n6;
        locals.var_x2_dn7 = assign72760_e109833_d_n7;
        locals.var_x2_dn8 = assign72760_e109833_d_n8;
        locals.var_x2_dn9 = assign72760_e109833_d_n9;
        locals.var_x2_dn10 = assign72760_e109833_d_n10;
        locals.var_x2_dn13 = assign72760_e109833_d_n13;
        locals.var_x2_rv = 0.0;

        let (assign72770_e109850, assign72770_e109850_d_n0, assign72770_e109850_d_n2, assign72770_e109850_d_n4, assign72770_e109850_d_n5, assign72770_e109850_d_n6, assign72770_e109850_d_n7, assign72770_e109850_d_n8, assign72770_e109850_d_n9, assign72770_e109850_d_n10, assign72770_e109850_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1687 != 0.0)) && (locals.var_guard1688 != 0.0)) {
        let assign72770_e109844: f64 = (locals.var_ddriftldc * 0.1);
        let assign72770_e109847: f64 = (locals.var_ddriftldc * 0.1);
        let assign72770_e109848: f64 = (assign72770_e109844 * assign72770_e109847);
        (assign72770_e109848, (((locals.var_ddriftldc_dn0 * 0.1) * assign72770_e109847) + (assign72770_e109844 * (locals.var_ddriftldc_dn0 * 0.1))), (((locals.var_ddriftldc_dn2 * 0.1) * assign72770_e109847) + (assign72770_e109844 * (locals.var_ddriftldc_dn2 * 0.1))), (((locals.var_ddriftldc_dn4 * 0.1) * assign72770_e109847) + (assign72770_e109844 * (locals.var_ddriftldc_dn4 * 0.1))), (((locals.var_ddriftldc_dn5 * 0.1) * assign72770_e109847) + (assign72770_e109844 * (locals.var_ddriftldc_dn5 * 0.1))), (((locals.var_ddriftldc_dn6 * 0.1) * assign72770_e109847) + (assign72770_e109844 * (locals.var_ddriftldc_dn6 * 0.1))), (((locals.var_ddriftldc_dn7 * 0.1) * assign72770_e109847) + (assign72770_e109844 * (locals.var_ddriftldc_dn7 * 0.1))), (((locals.var_ddriftldc_dn8 * 0.1) * assign72770_e109847) + (assign72770_e109844 * (locals.var_ddriftldc_dn8 * 0.1))), (((locals.var_ddriftldc_dn9 * 0.1) * assign72770_e109847) + (assign72770_e109844 * (locals.var_ddriftldc_dn9 * 0.1))), (((locals.var_ddriftldc_dn10 * 0.1) * assign72770_e109847) + (assign72770_e109844 * (locals.var_ddriftldc_dn10 * 0.1))), (((locals.var_ddriftldc_dn13 * 0.1) * assign72770_e109847) + (assign72770_e109844 * (locals.var_ddriftldc_dn13 * 0.1))),)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn13,)
    }
};
        locals.var_xmax2 = assign72770_e109850;
        locals.var_xmax2_dn0 = assign72770_e109850_d_n0;
        locals.var_xmax2_dn2 = assign72770_e109850_d_n2;
        locals.var_xmax2_dn4 = assign72770_e109850_d_n4;
        locals.var_xmax2_dn5 = assign72770_e109850_d_n5;
        locals.var_xmax2_dn6 = assign72770_e109850_d_n6;
        locals.var_xmax2_dn7 = assign72770_e109850_d_n7;
        locals.var_xmax2_dn8 = assign72770_e109850_d_n8;
        locals.var_xmax2_dn9 = assign72770_e109850_d_n9;
        locals.var_xmax2_dn10 = assign72770_e109850_d_n10;
        locals.var_xmax2_dn13 = assign72770_e109850_d_n13;
        locals.var_xmax2_rv = 0.0;

        let (assign72780_e109861, assign72780_e109861_d_n0, assign72780_e109861_d_n2, assign72780_e109861_d_n4, assign72780_e109861_d_n5, assign72780_e109861_d_n6, assign72780_e109861_d_n7, assign72780_e109861_d_n8, assign72780_e109861_d_n9, assign72780_e109861_d_n10, assign72780_e109861_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1687 != 0.0)) && (locals.var_guard1688 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign72780_e109861;
        locals.var_xp_dn0 = assign72780_e109861_d_n0;
        locals.var_xp_dn2 = assign72780_e109861_d_n2;
        locals.var_xp_dn4 = assign72780_e109861_d_n4;
        locals.var_xp_dn5 = assign72780_e109861_d_n5;
        locals.var_xp_dn6 = assign72780_e109861_d_n6;
        locals.var_xp_dn7 = assign72780_e109861_d_n7;
        locals.var_xp_dn8 = assign72780_e109861_d_n8;
        locals.var_xp_dn9 = assign72780_e109861_d_n9;
        locals.var_xp_dn10 = assign72780_e109861_d_n10;
        locals.var_xp_dn13 = assign72780_e109861_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign72790_e109872, assign72790_e109872_d_n0, assign72790_e109872_d_n2, assign72790_e109872_d_n4, assign72790_e109872_d_n5, assign72790_e109872_d_n6, assign72790_e109872_d_n7, assign72790_e109872_d_n8, assign72790_e109872_d_n9, assign72790_e109872_d_n10, assign72790_e109872_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1687 != 0.0)) && (locals.var_guard1688 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign72790_e109872;
        locals.var_xmp_dn0 = assign72790_e109872_d_n0;
        locals.var_xmp_dn2 = assign72790_e109872_d_n2;
        locals.var_xmp_dn4 = assign72790_e109872_d_n4;
        locals.var_xmp_dn5 = assign72790_e109872_d_n5;
        locals.var_xmp_dn6 = assign72790_e109872_d_n6;
        locals.var_xmp_dn7 = assign72790_e109872_d_n7;
        locals.var_xmp_dn8 = assign72790_e109872_d_n8;
        locals.var_xmp_dn9 = assign72790_e109872_d_n9;
        locals.var_xmp_dn10 = assign72790_e109872_d_n10;
        locals.var_xmp_dn13 = assign72790_e109872_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign72800_e109883,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1687 != 0.0)) && (locals.var_guard1688 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign72800_e109883;
        locals.var_m0_rv = 0.0;

        let (assign72810_e109894,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1687 != 0.0)) && (locals.var_guard1688 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign72810_e109894;
        locals.var_mm_rv = 0.0;

        let (assign72820_e109905, assign72820_e109905_d_n0, assign72820_e109905_d_n2, assign72820_e109905_d_n4, assign72820_e109905_d_n5, assign72820_e109905_d_n6, assign72820_e109905_d_n7, assign72820_e109905_d_n8, assign72820_e109905_d_n9, assign72820_e109905_d_n10, assign72820_e109905_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1687 != 0.0)) && (locals.var_guard1688 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign72820_e109905;
        locals.var_arg_dn0 = assign72820_e109905_d_n0;
        locals.var_arg_dn2 = assign72820_e109905_d_n2;
        locals.var_arg_dn4 = assign72820_e109905_d_n4;
        locals.var_arg_dn5 = assign72820_e109905_d_n5;
        locals.var_arg_dn6 = assign72820_e109905_d_n6;
        locals.var_arg_dn7 = assign72820_e109905_d_n7;
        locals.var_arg_dn8 = assign72820_e109905_d_n8;
        locals.var_arg_dn9 = assign72820_e109905_d_n9;
        locals.var_arg_dn10 = assign72820_e109905_d_n10;
        locals.var_arg_dn13 = assign72820_e109905_d_n13;
        locals.var_arg_rv = 0.0;

        let (assign72830_e109916, assign72830_e109916_d_n0, assign72830_e109916_d_n2, assign72830_e109916_d_n4, assign72830_e109916_d_n5, assign72830_e109916_d_n6, assign72830_e109916_d_n7, assign72830_e109916_d_n8, assign72830_e109916_d_n9, assign72830_e109916_d_n10, assign72830_e109916_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1687 != 0.0)) && (locals.var_guard1688 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign72830_e109916;
        locals.var_dnm_dn0 = assign72830_e109916_d_n0;
        locals.var_dnm_dn2 = assign72830_e109916_d_n2;
        locals.var_dnm_dn4 = assign72830_e109916_d_n4;
        locals.var_dnm_dn5 = assign72830_e109916_d_n5;
        locals.var_dnm_dn6 = assign72830_e109916_d_n6;
        locals.var_dnm_dn7 = assign72830_e109916_d_n7;
        locals.var_dnm_dn8 = assign72830_e109916_d_n8;
        locals.var_dnm_dn9 = assign72830_e109916_d_n9;
        locals.var_dnm_dn10 = assign72830_e109916_d_n10;
        locals.var_dnm_dn13 = assign72830_e109916_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign72840_e109929, assign72840_e109929_d_n0, assign72840_e109929_d_n2, assign72840_e109929_d_n4, assign72840_e109929_d_n5, assign72840_e109929_d_n6, assign72840_e109929_d_n7, assign72840_e109929_d_n8, assign72840_e109929_d_n9, assign72840_e109929_d_n10, assign72840_e109929_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1687 != 0.0)) && (locals.var_guard1688 != 0.0)) {
        let assign72840_e109927: f64 = (locals.var_xp * locals.var_x2);
        (assign72840_e109927, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign72840_e109929;
        locals.var_xp_dn0 = assign72840_e109929_d_n0;
        locals.var_xp_dn2 = assign72840_e109929_d_n2;
        locals.var_xp_dn4 = assign72840_e109929_d_n4;
        locals.var_xp_dn5 = assign72840_e109929_d_n5;
        locals.var_xp_dn6 = assign72840_e109929_d_n6;
        locals.var_xp_dn7 = assign72840_e109929_d_n7;
        locals.var_xp_dn8 = assign72840_e109929_d_n8;
        locals.var_xp_dn9 = assign72840_e109929_d_n9;
        locals.var_xp_dn10 = assign72840_e109929_d_n10;
        locals.var_xp_dn13 = assign72840_e109929_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign72850_e109942, assign72850_e109942_d_n0, assign72850_e109942_d_n2, assign72850_e109942_d_n4, assign72850_e109942_d_n5, assign72850_e109942_d_n6, assign72850_e109942_d_n7, assign72850_e109942_d_n8, assign72850_e109942_d_n9, assign72850_e109942_d_n10, assign72850_e109942_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1687 != 0.0)) && (locals.var_guard1688 != 0.0)) {
        let assign72850_e109940: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign72850_e109940, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign72850_e109942;
        locals.var_xmp_dn0 = assign72850_e109942_d_n0;
        locals.var_xmp_dn2 = assign72850_e109942_d_n2;
        locals.var_xmp_dn4 = assign72850_e109942_d_n4;
        locals.var_xmp_dn5 = assign72850_e109942_d_n5;
        locals.var_xmp_dn6 = assign72850_e109942_d_n6;
        locals.var_xmp_dn7 = assign72850_e109942_d_n7;
        locals.var_xmp_dn8 = assign72850_e109942_d_n8;
        locals.var_xmp_dn9 = assign72850_e109942_d_n9;
        locals.var_xmp_dn10 = assign72850_e109942_d_n10;
        locals.var_xmp_dn13 = assign72850_e109942_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign72860_e109955, assign72860_e109955_d_n0, assign72860_e109955_d_n2, assign72860_e109955_d_n4, assign72860_e109955_d_n5, assign72860_e109955_d_n6, assign72860_e109955_d_n7, assign72860_e109955_d_n8, assign72860_e109955_d_n9, assign72860_e109955_d_n10, assign72860_e109955_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1687 != 0.0)) && (locals.var_guard1688 != 0.0)) {
        let assign72860_e109953: f64 = (locals.var_xp * locals.var_x2);
        (assign72860_e109953, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign72860_e109955;
        locals.var_xp_dn0 = assign72860_e109955_d_n0;
        locals.var_xp_dn2 = assign72860_e109955_d_n2;
        locals.var_xp_dn4 = assign72860_e109955_d_n4;
        locals.var_xp_dn5 = assign72860_e109955_d_n5;
        locals.var_xp_dn6 = assign72860_e109955_d_n6;
        locals.var_xp_dn7 = assign72860_e109955_d_n7;
        locals.var_xp_dn8 = assign72860_e109955_d_n8;
        locals.var_xp_dn9 = assign72860_e109955_d_n9;
        locals.var_xp_dn10 = assign72860_e109955_d_n10;
        locals.var_xp_dn13 = assign72860_e109955_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign72870_e109968, assign72870_e109968_d_n0, assign72870_e109968_d_n2, assign72870_e109968_d_n4, assign72870_e109968_d_n5, assign72870_e109968_d_n6, assign72870_e109968_d_n7, assign72870_e109968_d_n8, assign72870_e109968_d_n9, assign72870_e109968_d_n10, assign72870_e109968_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1687 != 0.0)) && (locals.var_guard1688 != 0.0)) {
        let assign72870_e109966: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign72870_e109966, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign72870_e109968;
        locals.var_xmp_dn0 = assign72870_e109968_d_n0;
        locals.var_xmp_dn2 = assign72870_e109968_d_n2;
        locals.var_xmp_dn4 = assign72870_e109968_d_n4;
        locals.var_xmp_dn5 = assign72870_e109968_d_n5;
        locals.var_xmp_dn6 = assign72870_e109968_d_n6;
        locals.var_xmp_dn7 = assign72870_e109968_d_n7;
        locals.var_xmp_dn8 = assign72870_e109968_d_n8;
        locals.var_xmp_dn9 = assign72870_e109968_d_n9;
        locals.var_xmp_dn10 = assign72870_e109968_d_n10;
        locals.var_xmp_dn13 = assign72870_e109968_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign72880_e109981, assign72880_e109981_d_n0, assign72880_e109981_d_n2, assign72880_e109981_d_n4, assign72880_e109981_d_n5, assign72880_e109981_d_n6, assign72880_e109981_d_n7, assign72880_e109981_d_n8, assign72880_e109981_d_n9, assign72880_e109981_d_n10, assign72880_e109981_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1687 != 0.0)) && (locals.var_guard1688 != 0.0)) {
        let assign72880_e109979: f64 = (locals.var_xp + locals.var_xmp);
        (assign72880_e109979, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn13 + locals.var_xmp_dn13),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign72880_e109981;
        locals.var_arg_dn0 = assign72880_e109981_d_n0;
        locals.var_arg_dn2 = assign72880_e109981_d_n2;
        locals.var_arg_dn4 = assign72880_e109981_d_n4;
        locals.var_arg_dn5 = assign72880_e109981_d_n5;
        locals.var_arg_dn6 = assign72880_e109981_d_n6;
        locals.var_arg_dn7 = assign72880_e109981_d_n7;
        locals.var_arg_dn8 = assign72880_e109981_d_n8;
        locals.var_arg_dn9 = assign72880_e109981_d_n9;
        locals.var_arg_dn10 = assign72880_e109981_d_n10;
        locals.var_arg_dn13 = assign72880_e109981_d_n13;
        locals.var_arg_rv = 0.0;

        let (assign72890_e109992, assign72890_e109992_d_n0, assign72890_e109992_d_n2, assign72890_e109992_d_n4, assign72890_e109992_d_n5, assign72890_e109992_d_n6, assign72890_e109992_d_n7, assign72890_e109992_d_n8, assign72890_e109992_d_n9, assign72890_e109992_d_n10, assign72890_e109992_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1687 != 0.0)) && (locals.var_guard1688 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign72890_e109992;
        locals.var_dnm_dn0 = assign72890_e109992_d_n0;
        locals.var_dnm_dn2 = assign72890_e109992_d_n2;
        locals.var_dnm_dn4 = assign72890_e109992_d_n4;
        locals.var_dnm_dn5 = assign72890_e109992_d_n5;
        locals.var_dnm_dn6 = assign72890_e109992_d_n6;
        locals.var_dnm_dn7 = assign72890_e109992_d_n7;
        locals.var_dnm_dn8 = assign72890_e109992_d_n8;
        locals.var_dnm_dn9 = assign72890_e109992_d_n9;
        locals.var_dnm_dn10 = assign72890_e109992_d_n10;
        locals.var_dnm_dn13 = assign72890_e109992_d_n13;
        locals.var_dnm_rv = 0.0;

        let assign72900_e110007: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard1689 = assign72900_e110007;
        locals.var_guard1689_rv = 0.0;

        let assign72910_e110010: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1690 = assign72910_e110010;
        locals.var_guard1690_rv = 0.0;

        let (assign72920_e110025,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1687 != 0.0)) && (locals.var_guard1688 != 0.0)) && (locals.var_guard1689 != 0.0)) && (locals.var_guard1690 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign72920_e110025;
        locals.var_mm_rv = 0.0;

        let assign72930_e110028: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1691 = assign72930_e110028;
        locals.var_guard1691_rv = 0.0;

        let (assign72940_e110046,) = {
    if (((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1687 != 0.0)) && (locals.var_guard1688 != 0.0)) && (locals.var_guard1689 != 0.0)) && (locals.var_guard1690 == 0.0)) && (locals.var_guard1691 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign72940_e110046;
        locals.var_mm_rv = 0.0;

        let assign72950_e110049: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard1692 = assign72950_e110049;
        locals.var_guard1692_rv = 0.0;

        let (assign72960_e110070,) = {
    if ((((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1687 != 0.0)) && (locals.var_guard1688 != 0.0)) && (locals.var_guard1689 != 0.0)) && (locals.var_guard1690 == 0.0)) && (locals.var_guard1691 == 0.0)) && (locals.var_guard1692 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign72960_e110070;
        locals.var_mm_rv = 0.0;

        let assign72970_e110073: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard1693 = assign72970_e110073;
        locals.var_guard1693_rv = 0.0;

        let (assign72980_e110097,) = {
    if (((((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1687 != 0.0)) && (locals.var_guard1688 != 0.0)) && (locals.var_guard1689 != 0.0)) && (locals.var_guard1690 == 0.0)) && (locals.var_guard1691 == 0.0)) && (locals.var_guard1692 == 0.0)) && (locals.var_guard1693 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign72980_e110097;
        locals.var_mm_rv = 0.0;

        let (assign72990_e110110,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1687 != 0.0)) && (locals.var_guard1688 != 0.0)) && (locals.var_guard1689 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign72990_e110110;
        locals.var_m0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_263(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let mut assign73000_loop_guard: usize = 0;
        while {
            let assign73000_cond_e110124: f64 = if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1687 != 0.0)) && (locals.var_guard1688 != 0.0)) && (locals.var_guard1689 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign73000_cond_e110124 != 0.0
        } {
            assign73000_loop_guard += 1;
            assert!(assign73000_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign73000_body0_e110138, assign73000_body0_e110138_d_n0, assign73000_body0_e110138_d_n2, assign73000_body0_e110138_d_n4, assign73000_body0_e110138_d_n5, assign73000_body0_e110138_d_n6, assign73000_body0_e110138_d_n7, assign73000_body0_e110138_d_n8, assign73000_body0_e110138_d_n9, assign73000_body0_e110138_d_n10, assign73000_body0_e110138_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1687 != 0.0)) && (locals.var_guard1688 != 0.0)) && (locals.var_guard1689 != 0.0)) {
        let assign73000_body0_e110136: f64 = (locals.var_dnm).sqrt();
        (assign73000_body0_e110136, (locals.var_dnm_dn0 / (2.0 * assign73000_body0_e110136)), (locals.var_dnm_dn2 / (2.0 * assign73000_body0_e110136)), (locals.var_dnm_dn4 / (2.0 * assign73000_body0_e110136)), (locals.var_dnm_dn5 / (2.0 * assign73000_body0_e110136)), (locals.var_dnm_dn6 / (2.0 * assign73000_body0_e110136)), (locals.var_dnm_dn7 / (2.0 * assign73000_body0_e110136)), (locals.var_dnm_dn8 / (2.0 * assign73000_body0_e110136)), (locals.var_dnm_dn9 / (2.0 * assign73000_body0_e110136)), (locals.var_dnm_dn10 / (2.0 * assign73000_body0_e110136)), (locals.var_dnm_dn13 / (2.0 * assign73000_body0_e110136)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
            locals.var_dnm = assign73000_body0_e110138;
            locals.var_dnm_dn0 = assign73000_body0_e110138_d_n0;
            locals.var_dnm_dn2 = assign73000_body0_e110138_d_n2;
            locals.var_dnm_dn4 = assign73000_body0_e110138_d_n4;
            locals.var_dnm_dn5 = assign73000_body0_e110138_d_n5;
            locals.var_dnm_dn6 = assign73000_body0_e110138_d_n6;
            locals.var_dnm_dn7 = assign73000_body0_e110138_d_n7;
            locals.var_dnm_dn8 = assign73000_body0_e110138_d_n8;
            locals.var_dnm_dn9 = assign73000_body0_e110138_d_n9;
            locals.var_dnm_dn10 = assign73000_body0_e110138_d_n10;
            locals.var_dnm_dn13 = assign73000_body0_e110138_d_n13;
            locals.var_dnm_rv = 0.0;
            let (assign73000_body1_e110153,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1687 != 0.0)) && (locals.var_guard1688 != 0.0)) && (locals.var_guard1689 != 0.0)) {
        let assign73000_body1_e110151: f64 = (locals.var_m0 + 1.0);
        (assign73000_body1_e110151,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign73000_body1_e110153;
            locals.var_m0_rv = 0.0;
        }

        let (assign73010_e110178, assign73010_e110178_d_n0, assign73010_e110178_d_n2, assign73010_e110178_d_n4, assign73010_e110178_d_n5, assign73010_e110178_d_n6, assign73010_e110178_d_n7, assign73010_e110178_d_n8, assign73010_e110178_d_n9, assign73010_e110178_d_n10, assign73010_e110178_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1687 != 0.0)) && (locals.var_guard1688 != 0.0)) && (locals.var_guard1689 == 0.0)) {
        let (assign73010_e110176, assign73010_e110176_d_n0, assign73010_e110176_d_n2, assign73010_e110176_d_n4, assign73010_e110176_d_n5, assign73010_e110176_d_n6, assign73010_e110176_d_n7, assign73010_e110176_d_n8, assign73010_e110176_d_n9, assign73010_e110176_d_n10, assign73010_e110176_d_n13,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign73010_e110173: f64 = (2.0 * 2.0);
                let assign73010_e110174: f64 = (1.0 / assign73010_e110173);
                let assign73010_e110175: f64 = (locals.var_dnm).powf(assign73010_e110174);
                (assign73010_e110175, if 0.0 == 0.0 && ((assign73010_e110174) as f64).is_finite() && ((assign73010_e110174) as f64).fract() == 0.0 { if assign73010_e110174 == 0.0 { 0.0 } else { (assign73010_e110174 * ((locals.var_dnm).powf(assign73010_e110174 - 1.0) * locals.var_dnm_dn0)) } } else { (assign73010_e110175 * (assign73010_e110174 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign73010_e110174) as f64).is_finite() && ((assign73010_e110174) as f64).fract() == 0.0 { if assign73010_e110174 == 0.0 { 0.0 } else { (assign73010_e110174 * ((locals.var_dnm).powf(assign73010_e110174 - 1.0) * locals.var_dnm_dn2)) } } else { (assign73010_e110175 * (assign73010_e110174 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign73010_e110174) as f64).is_finite() && ((assign73010_e110174) as f64).fract() == 0.0 { if assign73010_e110174 == 0.0 { 0.0 } else { (assign73010_e110174 * ((locals.var_dnm).powf(assign73010_e110174 - 1.0) * locals.var_dnm_dn4)) } } else { (assign73010_e110175 * (assign73010_e110174 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign73010_e110174) as f64).is_finite() && ((assign73010_e110174) as f64).fract() == 0.0 { if assign73010_e110174 == 0.0 { 0.0 } else { (assign73010_e110174 * ((locals.var_dnm).powf(assign73010_e110174 - 1.0) * locals.var_dnm_dn5)) } } else { (assign73010_e110175 * (assign73010_e110174 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign73010_e110174) as f64).is_finite() && ((assign73010_e110174) as f64).fract() == 0.0 { if assign73010_e110174 == 0.0 { 0.0 } else { (assign73010_e110174 * ((locals.var_dnm).powf(assign73010_e110174 - 1.0) * locals.var_dnm_dn6)) } } else { (assign73010_e110175 * (assign73010_e110174 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign73010_e110174) as f64).is_finite() && ((assign73010_e110174) as f64).fract() == 0.0 { if assign73010_e110174 == 0.0 { 0.0 } else { (assign73010_e110174 * ((locals.var_dnm).powf(assign73010_e110174 - 1.0) * locals.var_dnm_dn7)) } } else { (assign73010_e110175 * (assign73010_e110174 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign73010_e110174) as f64).is_finite() && ((assign73010_e110174) as f64).fract() == 0.0 { if assign73010_e110174 == 0.0 { 0.0 } else { (assign73010_e110174 * ((locals.var_dnm).powf(assign73010_e110174 - 1.0) * locals.var_dnm_dn8)) } } else { (assign73010_e110175 * (assign73010_e110174 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign73010_e110174) as f64).is_finite() && ((assign73010_e110174) as f64).fract() == 0.0 { if assign73010_e110174 == 0.0 { 0.0 } else { (assign73010_e110174 * ((locals.var_dnm).powf(assign73010_e110174 - 1.0) * locals.var_dnm_dn9)) } } else { (assign73010_e110175 * (assign73010_e110174 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign73010_e110174) as f64).is_finite() && ((assign73010_e110174) as f64).fract() == 0.0 { if assign73010_e110174 == 0.0 { 0.0 } else { (assign73010_e110174 * ((locals.var_dnm).powf(assign73010_e110174 - 1.0) * locals.var_dnm_dn10)) } } else { (assign73010_e110175 * (assign73010_e110174 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign73010_e110174) as f64).is_finite() && ((assign73010_e110174) as f64).fract() == 0.0 { if assign73010_e110174 == 0.0 { 0.0 } else { (assign73010_e110174 * ((locals.var_dnm).powf(assign73010_e110174 - 1.0) * locals.var_dnm_dn13)) } } else { (assign73010_e110175 * (assign73010_e110174 * (locals.var_dnm_dn13 / locals.var_dnm))) },)
            }
        };
        (assign73010_e110176, assign73010_e110176_d_n0, assign73010_e110176_d_n2, assign73010_e110176_d_n4, assign73010_e110176_d_n5, assign73010_e110176_d_n6, assign73010_e110176_d_n7, assign73010_e110176_d_n8, assign73010_e110176_d_n9, assign73010_e110176_d_n10, assign73010_e110176_d_n13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign73010_e110178;
        locals.var_dnm_dn0 = assign73010_e110178_d_n0;
        locals.var_dnm_dn2 = assign73010_e110178_d_n2;
        locals.var_dnm_dn4 = assign73010_e110178_d_n4;
        locals.var_dnm_dn5 = assign73010_e110178_d_n5;
        locals.var_dnm_dn6 = assign73010_e110178_d_n6;
        locals.var_dnm_dn7 = assign73010_e110178_d_n7;
        locals.var_dnm_dn8 = assign73010_e110178_d_n8;
        locals.var_dnm_dn9 = assign73010_e110178_d_n9;
        locals.var_dnm_dn10 = assign73010_e110178_d_n10;
        locals.var_dnm_dn13 = assign73010_e110178_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign73020_e110191, assign73020_e110191_d_n0, assign73020_e110191_d_n2, assign73020_e110191_d_n4, assign73020_e110191_d_n5, assign73020_e110191_d_n6, assign73020_e110191_d_n7, assign73020_e110191_d_n8, assign73020_e110191_d_n9, assign73020_e110191_d_n10, assign73020_e110191_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1687 != 0.0)) && (locals.var_guard1688 != 0.0)) {
        let assign73020_e110189: f64 = (1.0 / locals.var_dnm);
        (assign73020_e110189, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn13 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign73020_e110191;
        locals.var_dnm_dn0 = assign73020_e110191_d_n0;
        locals.var_dnm_dn2 = assign73020_e110191_d_n2;
        locals.var_dnm_dn4 = assign73020_e110191_d_n4;
        locals.var_dnm_dn5 = assign73020_e110191_d_n5;
        locals.var_dnm_dn6 = assign73020_e110191_d_n6;
        locals.var_dnm_dn7 = assign73020_e110191_d_n7;
        locals.var_dnm_dn8 = assign73020_e110191_d_n8;
        locals.var_dnm_dn9 = assign73020_e110191_d_n9;
        locals.var_dnm_dn10 = assign73020_e110191_d_n10;
        locals.var_dnm_dn13 = assign73020_e110191_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign73030_e110208, assign73030_e110208_d_n0, assign73030_e110208_d_n2, assign73030_e110208_d_n4, assign73030_e110208_d_n5, assign73030_e110208_d_n6, assign73030_e110208_d_n7, assign73030_e110208_d_n8, assign73030_e110208_d_n9, assign73030_e110208_d_n10, assign73030_e110208_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1687 != 0.0)) && (locals.var_guard1688 != 0.0)) {
        let assign73030_e110203: f64 = (locals.var_ddriftldc * 0.1);
        let assign73030_e110204: f64 = (locals.var_tmf1 * assign73030_e110203);
        let assign73030_e110206: f64 = (assign73030_e110204 * locals.var_dnm);
        (assign73030_e110206, ((((locals.var_tmf1_dn0 * assign73030_e110203) + (locals.var_tmf1 * (locals.var_ddriftldc_dn0 * 0.1))) * locals.var_dnm) + (assign73030_e110204 * locals.var_dnm_dn0)), ((((locals.var_tmf1_dn2 * assign73030_e110203) + (locals.var_tmf1 * (locals.var_ddriftldc_dn2 * 0.1))) * locals.var_dnm) + (assign73030_e110204 * locals.var_dnm_dn2)), ((((locals.var_tmf1_dn4 * assign73030_e110203) + (locals.var_tmf1 * (locals.var_ddriftldc_dn4 * 0.1))) * locals.var_dnm) + (assign73030_e110204 * locals.var_dnm_dn4)), ((((locals.var_tmf1_dn5 * assign73030_e110203) + (locals.var_tmf1 * (locals.var_ddriftldc_dn5 * 0.1))) * locals.var_dnm) + (assign73030_e110204 * locals.var_dnm_dn5)), ((((locals.var_tmf1_dn6 * assign73030_e110203) + (locals.var_tmf1 * (locals.var_ddriftldc_dn6 * 0.1))) * locals.var_dnm) + (assign73030_e110204 * locals.var_dnm_dn6)), ((((locals.var_tmf1_dn7 * assign73030_e110203) + (locals.var_tmf1 * (locals.var_ddriftldc_dn7 * 0.1))) * locals.var_dnm) + (assign73030_e110204 * locals.var_dnm_dn7)), ((((locals.var_tmf1_dn8 * assign73030_e110203) + (locals.var_tmf1 * (locals.var_ddriftldc_dn8 * 0.1))) * locals.var_dnm) + (assign73030_e110204 * locals.var_dnm_dn8)), ((((locals.var_tmf1_dn9 * assign73030_e110203) + (locals.var_tmf1 * (locals.var_ddriftldc_dn9 * 0.1))) * locals.var_dnm) + (assign73030_e110204 * locals.var_dnm_dn9)), ((((locals.var_tmf1_dn10 * assign73030_e110203) + (locals.var_tmf1 * (locals.var_ddriftldc_dn10 * 0.1))) * locals.var_dnm) + (assign73030_e110204 * locals.var_dnm_dn10)), ((((locals.var_tmf1_dn13 * assign73030_e110203) + (locals.var_tmf1 * (locals.var_ddriftldc_dn13 * 0.1))) * locals.var_dnm) + (assign73030_e110204 * locals.var_dnm_dn13)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn13,)
    }
};
        locals.var_tmf0 = assign73030_e110208;
        locals.var_tmf0_dn0 = assign73030_e110208_d_n0;
        locals.var_tmf0_dn2 = assign73030_e110208_d_n2;
        locals.var_tmf0_dn4 = assign73030_e110208_d_n4;
        locals.var_tmf0_dn5 = assign73030_e110208_d_n5;
        locals.var_tmf0_dn6 = assign73030_e110208_d_n6;
        locals.var_tmf0_dn7 = assign73030_e110208_d_n7;
        locals.var_tmf0_dn8 = assign73030_e110208_d_n8;
        locals.var_tmf0_dn9 = assign73030_e110208_d_n9;
        locals.var_tmf0_dn10 = assign73030_e110208_d_n10;
        locals.var_tmf0_dn13 = assign73030_e110208_d_n13;
        locals.var_tmf0_rv = 0.0;

        let (assign73040_e110227, assign73040_e110227_d_n0, assign73040_e110227_d_n2, assign73040_e110227_d_n4, assign73040_e110227_d_n5, assign73040_e110227_d_n6, assign73040_e110227_d_n7, assign73040_e110227_d_n8, assign73040_e110227_d_n9, assign73040_e110227_d_n10, assign73040_e110227_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1687 != 0.0)) && (locals.var_guard1688 != 0.0)) {
        let assign73040_e110219: f64 = (locals.var_ddriftldc * 0.1);
        let assign73040_e110221: f64 = (assign73040_e110219 * locals.var_xmp);
        let assign73040_e110223: f64 = (assign73040_e110221 * locals.var_dnm);
        let assign73040_e110225: f64 = (assign73040_e110223 / locals.var_arg);
        (assign73040_e110225, ((((((((locals.var_ddriftldc_dn0 * 0.1) * locals.var_xmp) + (assign73040_e110219 * locals.var_xmp_dn0)) * locals.var_dnm) + (assign73040_e110221 * locals.var_dnm_dn0)) * locals.var_arg) - (assign73040_e110223 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((((locals.var_ddriftldc_dn2 * 0.1) * locals.var_xmp) + (assign73040_e110219 * locals.var_xmp_dn2)) * locals.var_dnm) + (assign73040_e110221 * locals.var_dnm_dn2)) * locals.var_arg) - (assign73040_e110223 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((((locals.var_ddriftldc_dn4 * 0.1) * locals.var_xmp) + (assign73040_e110219 * locals.var_xmp_dn4)) * locals.var_dnm) + (assign73040_e110221 * locals.var_dnm_dn4)) * locals.var_arg) - (assign73040_e110223 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((((locals.var_ddriftldc_dn5 * 0.1) * locals.var_xmp) + (assign73040_e110219 * locals.var_xmp_dn5)) * locals.var_dnm) + (assign73040_e110221 * locals.var_dnm_dn5)) * locals.var_arg) - (assign73040_e110223 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((((locals.var_ddriftldc_dn6 * 0.1) * locals.var_xmp) + (assign73040_e110219 * locals.var_xmp_dn6)) * locals.var_dnm) + (assign73040_e110221 * locals.var_dnm_dn6)) * locals.var_arg) - (assign73040_e110223 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((((locals.var_ddriftldc_dn7 * 0.1) * locals.var_xmp) + (assign73040_e110219 * locals.var_xmp_dn7)) * locals.var_dnm) + (assign73040_e110221 * locals.var_dnm_dn7)) * locals.var_arg) - (assign73040_e110223 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((((locals.var_ddriftldc_dn8 * 0.1) * locals.var_xmp) + (assign73040_e110219 * locals.var_xmp_dn8)) * locals.var_dnm) + (assign73040_e110221 * locals.var_dnm_dn8)) * locals.var_arg) - (assign73040_e110223 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((((locals.var_ddriftldc_dn9 * 0.1) * locals.var_xmp) + (assign73040_e110219 * locals.var_xmp_dn9)) * locals.var_dnm) + (assign73040_e110221 * locals.var_dnm_dn9)) * locals.var_arg) - (assign73040_e110223 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((((locals.var_ddriftldc_dn10 * 0.1) * locals.var_xmp) + (assign73040_e110219 * locals.var_xmp_dn10)) * locals.var_dnm) + (assign73040_e110221 * locals.var_dnm_dn10)) * locals.var_arg) - (assign73040_e110223 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((((locals.var_ddriftldc_dn13 * 0.1) * locals.var_xmp) + (assign73040_e110219 * locals.var_xmp_dn13)) * locals.var_dnm) + (assign73040_e110221 * locals.var_dnm_dn13)) * locals.var_arg) - (assign73040_e110223 * locals.var_arg_dn13)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign73040_e110227;
        locals.var_t0_dn0 = assign73040_e110227_d_n0;
        locals.var_t0_dn2 = assign73040_e110227_d_n2;
        locals.var_t0_dn4 = assign73040_e110227_d_n4;
        locals.var_t0_dn5 = assign73040_e110227_d_n5;
        locals.var_t0_dn6 = assign73040_e110227_d_n6;
        locals.var_t0_dn7 = assign73040_e110227_d_n7;
        locals.var_t0_dn8 = assign73040_e110227_d_n8;
        locals.var_t0_dn9 = assign73040_e110227_d_n9;
        locals.var_t0_dn10 = assign73040_e110227_d_n10;
        locals.var_t0_dn13 = assign73040_e110227_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign73050_e110244, assign73050_e110244_d_n0, assign73050_e110244_d_n2, assign73050_e110244_d_n4, assign73050_e110244_d_n5, assign73050_e110244_d_n6, assign73050_e110244_d_n7, assign73050_e110244_d_n8, assign73050_e110244_d_n9, assign73050_e110244_d_n10, assign73050_e110244_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1687 != 0.0)) && (locals.var_guard1688 != 0.0)) {
        let assign73050_e110239: f64 = (locals.var_ddriftldc * 0.1);
        let assign73050_e110240: f64 = (locals.var_ddriftldc - assign73050_e110239);
        let assign73050_e110242: f64 = (assign73050_e110240 + locals.var_tmf0);
        (assign73050_e110242, ((locals.var_ddriftldc_dn0 - (locals.var_ddriftldc_dn0 * 0.1)) + locals.var_tmf0_dn0), ((locals.var_ddriftldc_dn2 - (locals.var_ddriftldc_dn2 * 0.1)) + locals.var_tmf0_dn2), ((locals.var_ddriftldc_dn4 - (locals.var_ddriftldc_dn4 * 0.1)) + locals.var_tmf0_dn4), ((locals.var_ddriftldc_dn5 - (locals.var_ddriftldc_dn5 * 0.1)) + locals.var_tmf0_dn5), ((locals.var_ddriftldc_dn6 - (locals.var_ddriftldc_dn6 * 0.1)) + locals.var_tmf0_dn6), ((locals.var_ddriftldc_dn7 - (locals.var_ddriftldc_dn7 * 0.1)) + locals.var_tmf0_dn7), ((locals.var_ddriftldc_dn8 - (locals.var_ddriftldc_dn8 * 0.1)) + locals.var_tmf0_dn8), ((locals.var_ddriftldc_dn9 - (locals.var_ddriftldc_dn9 * 0.1)) + locals.var_tmf0_dn9), ((locals.var_ddriftldc_dn10 - (locals.var_ddriftldc_dn10 * 0.1)) + locals.var_tmf0_dn10), ((locals.var_ddriftldc_dn13 - (locals.var_ddriftldc_dn13 * 0.1)) + locals.var_tmf0_dn13),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign73050_e110244;
        locals.var_t1_dn0 = assign73050_e110244_d_n0;
        locals.var_t1_dn2 = assign73050_e110244_d_n2;
        locals.var_t1_dn4 = assign73050_e110244_d_n4;
        locals.var_t1_dn5 = assign73050_e110244_d_n5;
        locals.var_t1_dn6 = assign73050_e110244_d_n6;
        locals.var_t1_dn7 = assign73050_e110244_d_n7;
        locals.var_t1_dn8 = assign73050_e110244_d_n8;
        locals.var_t1_dn9 = assign73050_e110244_d_n9;
        locals.var_t1_dn10 = assign73050_e110244_d_n10;
        locals.var_t1_dn13 = assign73050_e110244_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign73060_e110255, assign73060_e110255_d_n0, assign73060_e110255_d_n2, assign73060_e110255_d_n4, assign73060_e110255_d_n5, assign73060_e110255_d_n6, assign73060_e110255_d_n7, assign73060_e110255_d_n8, assign73060_e110255_d_n9, assign73060_e110255_d_n10, assign73060_e110255_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1687 != 0.0)) && (locals.var_guard1688 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign73060_e110255;
        locals.var_t0_dn0 = assign73060_e110255_d_n0;
        locals.var_t0_dn2 = assign73060_e110255_d_n2;
        locals.var_t0_dn4 = assign73060_e110255_d_n4;
        locals.var_t0_dn5 = assign73060_e110255_d_n5;
        locals.var_t0_dn6 = assign73060_e110255_d_n6;
        locals.var_t0_dn7 = assign73060_e110255_d_n7;
        locals.var_t0_dn8 = assign73060_e110255_d_n8;
        locals.var_t0_dn9 = assign73060_e110255_d_n9;
        locals.var_t0_dn10 = assign73060_e110255_d_n10;
        locals.var_t0_dn13 = assign73060_e110255_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign73070_e110267, assign73070_e110267_d_n0, assign73070_e110267_d_n2, assign73070_e110267_d_n4, assign73070_e110267_d_n5, assign73070_e110267_d_n6, assign73070_e110267_d_n7, assign73070_e110267_d_n8, assign73070_e110267_d_n9, assign73070_e110267_d_n10, assign73070_e110267_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1687 != 0.0)) && (locals.var_guard1688 == 0.0)) {
        (locals.var_wdld0, locals.var_wdld0_dn0, locals.var_wdld0_dn2, locals.var_wdld0_dn4, locals.var_wdld0_dn5, locals.var_wdld0_dn6, locals.var_wdld0_dn7, locals.var_wdld0_dn8, locals.var_wdld0_dn9, locals.var_wdld0_dn10, locals.var_wdld0_dn13,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign73070_e110267;
        locals.var_t1_dn0 = assign73070_e110267_d_n0;
        locals.var_t1_dn2 = assign73070_e110267_d_n2;
        locals.var_t1_dn4 = assign73070_e110267_d_n4;
        locals.var_t1_dn5 = assign73070_e110267_d_n5;
        locals.var_t1_dn6 = assign73070_e110267_d_n6;
        locals.var_t1_dn7 = assign73070_e110267_d_n7;
        locals.var_t1_dn8 = assign73070_e110267_d_n8;
        locals.var_t1_dn9 = assign73070_e110267_d_n9;
        locals.var_t1_dn10 = assign73070_e110267_d_n10;
        locals.var_t1_dn13 = assign73070_e110267_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign73080_e110279, assign73080_e110279_d_n0, assign73080_e110279_d_n2, assign73080_e110279_d_n4, assign73080_e110279_d_n5, assign73080_e110279_d_n6, assign73080_e110279_d_n7, assign73080_e110279_d_n8, assign73080_e110279_d_n9, assign73080_e110279_d_n10, assign73080_e110279_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1687 != 0.0)) && (locals.var_guard1688 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign73080_e110279;
        locals.var_t0_dn0 = assign73080_e110279_d_n0;
        locals.var_t0_dn2 = assign73080_e110279_d_n2;
        locals.var_t0_dn4 = assign73080_e110279_d_n4;
        locals.var_t0_dn5 = assign73080_e110279_d_n5;
        locals.var_t0_dn6 = assign73080_e110279_d_n6;
        locals.var_t0_dn7 = assign73080_e110279_d_n7;
        locals.var_t0_dn8 = assign73080_e110279_d_n8;
        locals.var_t0_dn9 = assign73080_e110279_d_n9;
        locals.var_t0_dn10 = assign73080_e110279_d_n10;
        locals.var_t0_dn13 = assign73080_e110279_d_n13;
        locals.var_t0_rv = 0.0;

        let assign73090_e110282: f64 = if locals.var_t0 < 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1694 = assign73090_e110282;
        locals.var_guard1694_rv = 0.0;

        let (assign73100_e110295,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1687 != 0.0)) && (locals.var_guard1694 != 0.0)) {
        let assign73100_e110293: f64 = (locals.var_flg_fd_mode + 2.0);
        (assign73100_e110293,)
    } else {
        (locals.var_flg_fd_mode,)
    }
};
        locals.var_flg_fd_mode = assign73100_e110295;
        locals.var_flg_fd_mode_rv = 0.0;

        let (assign73110_e110310, assign73110_e110310_d_n0, assign73110_e110310_d_n2, assign73110_e110310_d_n4, assign73110_e110310_d_n5, assign73110_e110310_d_n6, assign73110_e110310_d_n7, assign73110_e110310_d_n8, assign73110_e110310_d_n9, assign73110_e110310_d_n10, assign73110_e110310_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1687 == 0.0)) {
        let (assign73110_e110308, assign73110_e110308_d_n0, assign73110_e110308_d_n2, assign73110_e110308_d_n4, assign73110_e110308_d_n5, assign73110_e110308_d_n6, assign73110_e110308_d_n7, assign73110_e110308_d_n8, assign73110_e110308_d_n9, assign73110_e110308_d_n10, assign73110_e110308_d_n13,) = {
            if (locals.var_wdld0 <= locals.var_ddriftldc) {
                (locals.var_wdld0, locals.var_wdld0_dn0, locals.var_wdld0_dn2, locals.var_wdld0_dn4, locals.var_wdld0_dn5, locals.var_wdld0_dn6, locals.var_wdld0_dn7, locals.var_wdld0_dn8, locals.var_wdld0_dn9, locals.var_wdld0_dn10, locals.var_wdld0_dn13,)
            } else {
                (locals.var_ddriftldc, locals.var_ddriftldc_dn0, locals.var_ddriftldc_dn2, locals.var_ddriftldc_dn4, locals.var_ddriftldc_dn5, locals.var_ddriftldc_dn6, locals.var_ddriftldc_dn7, locals.var_ddriftldc_dn8, locals.var_ddriftldc_dn9, locals.var_ddriftldc_dn10, locals.var_ddriftldc_dn13,)
            }
        };
        (assign73110_e110308, assign73110_e110308_d_n0, assign73110_e110308_d_n2, assign73110_e110308_d_n4, assign73110_e110308_d_n5, assign73110_e110308_d_n6, assign73110_e110308_d_n7, assign73110_e110308_d_n8, assign73110_e110308_d_n9, assign73110_e110308_d_n10, assign73110_e110308_d_n13,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign73110_e110310;
        locals.var_t1_dn0 = assign73110_e110310_d_n0;
        locals.var_t1_dn2 = assign73110_e110310_d_n2;
        locals.var_t1_dn4 = assign73110_e110310_d_n4;
        locals.var_t1_dn5 = assign73110_e110310_d_n5;
        locals.var_t1_dn6 = assign73110_e110310_d_n6;
        locals.var_t1_dn7 = assign73110_e110310_d_n7;
        locals.var_t1_dn8 = assign73110_e110310_d_n8;
        locals.var_t1_dn9 = assign73110_e110310_d_n9;
        locals.var_t1_dn10 = assign73110_e110310_d_n10;
        locals.var_t1_dn13 = assign73110_e110310_d_n13;
        locals.var_t1_rv = 0.0;

        let assign73120_e110313: f64 = if locals.var_wdld0 >= locals.var_ddriftldc { 1.0 } else { 0.0 };
        locals.var_guard1695 = assign73120_e110313;
        locals.var_guard1695_rv = 0.0;

        let (assign73130_e110327,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1687 == 0.0)) && (locals.var_guard1695 != 0.0)) {
        let assign73130_e110325: f64 = (locals.var_flg_fd_mode + 2.0);
        (assign73130_e110325,)
    } else {
        (locals.var_flg_fd_mode,)
    }
};
        locals.var_flg_fd_mode = assign73130_e110327;
        locals.var_flg_fd_mode_rv = 0.0;

        let assign73140_e110330: f64 = if locals.var_flg_fd_mode >= 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1696 = assign73140_e110330;
        locals.var_guard1696_rv = 0.0;

        let (assign73150_e110339, assign73150_e110339_d_n0, assign73150_e110339_d_n2, assign73150_e110339_d_n4, assign73150_e110339_d_n5, assign73150_e110339_d_n6, assign73150_e110339_d_n7, assign73150_e110339_d_n8, assign73150_e110339_d_n9, assign73150_e110339_d_n10, assign73150_e110339_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1696 != 0.0)) {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn13,)
    } else {
        (locals.var_ps0ld_bef1, locals.var_ps0ld_bef1_dn0, locals.var_ps0ld_bef1_dn2, locals.var_ps0ld_bef1_dn4, locals.var_ps0ld_bef1_dn5, locals.var_ps0ld_bef1_dn6, locals.var_ps0ld_bef1_dn7, locals.var_ps0ld_bef1_dn8, locals.var_ps0ld_bef1_dn9, locals.var_ps0ld_bef1_dn10, locals.var_ps0ld_bef1_dn13,)
    }
};
        locals.var_ps0ld_bef1 = assign73150_e110339;
        locals.var_ps0ld_bef1_dn0 = assign73150_e110339_d_n0;
        locals.var_ps0ld_bef1_dn2 = assign73150_e110339_d_n2;
        locals.var_ps0ld_bef1_dn4 = assign73150_e110339_d_n4;
        locals.var_ps0ld_bef1_dn5 = assign73150_e110339_d_n5;
        locals.var_ps0ld_bef1_dn6 = assign73150_e110339_d_n6;
        locals.var_ps0ld_bef1_dn7 = assign73150_e110339_d_n7;
        locals.var_ps0ld_bef1_dn8 = assign73150_e110339_d_n8;
        locals.var_ps0ld_bef1_dn9 = assign73150_e110339_d_n9;
        locals.var_ps0ld_bef1_dn10 = assign73150_e110339_d_n10;
        locals.var_ps0ld_bef1_dn13 = assign73150_e110339_d_n13;
        locals.var_ps0ld_bef1_rv = 0.0;

        let (assign73160_e110350, assign73160_e110350_d_n0, assign73160_e110350_d_n2, assign73160_e110350_d_n4, assign73160_e110350_d_n5, assign73160_e110350_d_n6, assign73160_e110350_d_n7, assign73160_e110350_d_n8, assign73160_e110350_d_n9, assign73160_e110350_d_n10, assign73160_e110350_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1696 != 0.0)) {
        let assign73160_e110348: f64 = (locals.var_t1 * locals.var_q_nsubld);
        (assign73160_e110348, (locals.var_t1_dn0 * locals.var_q_nsubld), (locals.var_t1_dn2 * locals.var_q_nsubld), (locals.var_t1_dn4 * locals.var_q_nsubld), (locals.var_t1_dn5 * locals.var_q_nsubld), (locals.var_t1_dn6 * locals.var_q_nsubld), (locals.var_t1_dn7 * locals.var_q_nsubld), (locals.var_t1_dn8 * locals.var_q_nsubld), (locals.var_t1_dn9 * locals.var_q_nsubld), (locals.var_t1_dn10 * locals.var_q_nsubld), (locals.var_t1_dn13 * locals.var_q_nsubld),)
    } else {
        (locals.var_qbuld, locals.var_qbuld_dn0, locals.var_qbuld_dn2, locals.var_qbuld_dn4, locals.var_qbuld_dn5, locals.var_qbuld_dn6, locals.var_qbuld_dn7, locals.var_qbuld_dn8, locals.var_qbuld_dn9, locals.var_qbuld_dn10, locals.var_qbuld_dn13,)
    }
};
        locals.var_qbuld = assign73160_e110350;
        locals.var_qbuld_dn0 = assign73160_e110350_d_n0;
        locals.var_qbuld_dn2 = assign73160_e110350_d_n2;
        locals.var_qbuld_dn4 = assign73160_e110350_d_n4;
        locals.var_qbuld_dn5 = assign73160_e110350_d_n5;
        locals.var_qbuld_dn6 = assign73160_e110350_d_n6;
        locals.var_qbuld_dn7 = assign73160_e110350_d_n7;
        locals.var_qbuld_dn8 = assign73160_e110350_d_n8;
        locals.var_qbuld_dn9 = assign73160_e110350_d_n9;
        locals.var_qbuld_dn10 = assign73160_e110350_d_n10;
        locals.var_qbuld_dn13 = assign73160_e110350_d_n13;
        locals.var_qbuld_rv = 0.0;

        let (assign73170_e110363, assign73170_e110363_d_n0, assign73170_e110363_d_n2, assign73170_e110363_d_n4, assign73170_e110363_d_n5, assign73170_e110363_d_n6, assign73170_e110363_d_n7, assign73170_e110363_d_n8, assign73170_e110363_d_n9, assign73170_e110363_d_n10, assign73170_e110363_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1696 != 0.0)) {
        let assign73170_e110360: f64 = (locals.var_qbuld / locals.var_cox0_func);
        let assign73170_e110361: f64 = (locals.var_vgpld - assign73170_e110360);
        (assign73170_e110361, (-(locals.var_qbuld_dn0 / locals.var_cox0_func)), (locals.var_vgpld_dn2 - (locals.var_qbuld_dn2 / locals.var_cox0_func)), (-(locals.var_qbuld_dn4 / locals.var_cox0_func)), (-(locals.var_qbuld_dn5 / locals.var_cox0_func)), (locals.var_vgpld_dn6 - (locals.var_qbuld_dn6 / locals.var_cox0_func)), (locals.var_vgpld_dn7 - (locals.var_qbuld_dn7 / locals.var_cox0_func)), (locals.var_vgpld_dn8 - (locals.var_qbuld_dn8 / locals.var_cox0_func)), (-(locals.var_qbuld_dn9 / locals.var_cox0_func)), (-(locals.var_qbuld_dn10 / locals.var_cox0_func)), (-(locals.var_qbuld_dn13 / locals.var_cox0_func)),)
    } else {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn13,)
    }
};
        locals.var_ps0ld = assign73170_e110363;
        locals.var_ps0ld_dn0 = assign73170_e110363_d_n0;
        locals.var_ps0ld_dn2 = assign73170_e110363_d_n2;
        locals.var_ps0ld_dn4 = assign73170_e110363_d_n4;
        locals.var_ps0ld_dn5 = assign73170_e110363_d_n5;
        locals.var_ps0ld_dn6 = assign73170_e110363_d_n6;
        locals.var_ps0ld_dn7 = assign73170_e110363_d_n7;
        locals.var_ps0ld_dn8 = assign73170_e110363_d_n8;
        locals.var_ps0ld_dn9 = assign73170_e110363_d_n9;
        locals.var_ps0ld_dn10 = assign73170_e110363_d_n10;
        locals.var_ps0ld_dn13 = assign73170_e110363_d_n13;
        locals.var_ps0ld_rv = 0.0;

        let assign73180_e110366: f64 = if p.p33 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1697 = assign73180_e110366;
        locals.var_guard1697_rv = 0.0;

        let assign73190_e110370: f64 = (locals.var_ps0ld_bef1 - 0.1);
        let assign73190_e110375: f64 = if ((locals.var_ps0ld > assign73190_e110370) && (0.1 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1698 = assign73190_e110375;
        locals.var_guard1698_rv = 0.0;

        let (assign73200_e110392, assign73200_e110392_d_n0, assign73200_e110392_d_n2, assign73200_e110392_d_n4, assign73200_e110392_d_n5, assign73200_e110392_d_n6, assign73200_e110392_d_n7, assign73200_e110392_d_n8, assign73200_e110392_d_n9, assign73200_e110392_d_n10, assign73200_e110392_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1696 != 0.0)) && (locals.var_guard1697 != 0.0)) && (locals.var_guard1698 != 0.0)) {
        let assign73200_e110388: f64 = (locals.var_ps0ld - locals.var_ps0ld_bef1);
        let assign73200_e110390: f64 = (assign73200_e110388 + 0.1);
        (assign73200_e110390, (locals.var_ps0ld_dn0 - locals.var_ps0ld_bef1_dn0), (locals.var_ps0ld_dn2 - locals.var_ps0ld_bef1_dn2), (locals.var_ps0ld_dn4 - locals.var_ps0ld_bef1_dn4), (locals.var_ps0ld_dn5 - locals.var_ps0ld_bef1_dn5), (locals.var_ps0ld_dn6 - locals.var_ps0ld_bef1_dn6), (locals.var_ps0ld_dn7 - locals.var_ps0ld_bef1_dn7), (locals.var_ps0ld_dn8 - locals.var_ps0ld_bef1_dn8), (locals.var_ps0ld_dn9 - locals.var_ps0ld_bef1_dn9), (locals.var_ps0ld_dn10 - locals.var_ps0ld_bef1_dn10), (locals.var_ps0ld_dn13 - locals.var_ps0ld_bef1_dn13),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign73200_e110392;
        locals.var_tmf1_dn0 = assign73200_e110392_d_n0;
        locals.var_tmf1_dn2 = assign73200_e110392_d_n2;
        locals.var_tmf1_dn4 = assign73200_e110392_d_n4;
        locals.var_tmf1_dn5 = assign73200_e110392_d_n5;
        locals.var_tmf1_dn6 = assign73200_e110392_d_n6;
        locals.var_tmf1_dn7 = assign73200_e110392_d_n7;
        locals.var_tmf1_dn8 = assign73200_e110392_d_n8;
        locals.var_tmf1_dn9 = assign73200_e110392_d_n9;
        locals.var_tmf1_dn10 = assign73200_e110392_d_n10;
        locals.var_tmf1_dn13 = assign73200_e110392_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign73210_e110407, assign73210_e110407_d_n0, assign73210_e110407_d_n2, assign73210_e110407_d_n4, assign73210_e110407_d_n5, assign73210_e110407_d_n6, assign73210_e110407_d_n7, assign73210_e110407_d_n8, assign73210_e110407_d_n9, assign73210_e110407_d_n10, assign73210_e110407_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1696 != 0.0)) && (locals.var_guard1697 != 0.0)) && (locals.var_guard1698 != 0.0)) {
        let assign73210_e110405: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign73210_e110405, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn13,)
    }
};
        locals.var_x2 = assign73210_e110407;
        locals.var_x2_dn0 = assign73210_e110407_d_n0;
        locals.var_x2_dn2 = assign73210_e110407_d_n2;
        locals.var_x2_dn4 = assign73210_e110407_d_n4;
        locals.var_x2_dn5 = assign73210_e110407_d_n5;
        locals.var_x2_dn6 = assign73210_e110407_d_n6;
        locals.var_x2_dn7 = assign73210_e110407_d_n7;
        locals.var_x2_dn8 = assign73210_e110407_d_n8;
        locals.var_x2_dn9 = assign73210_e110407_d_n9;
        locals.var_x2_dn10 = assign73210_e110407_d_n10;
        locals.var_x2_dn13 = assign73210_e110407_d_n13;
        locals.var_x2_rv = 0.0;

        let (assign73220_e110422, assign73220_e110422_d_n0, assign73220_e110422_d_n2, assign73220_e110422_d_n4, assign73220_e110422_d_n5, assign73220_e110422_d_n6, assign73220_e110422_d_n7, assign73220_e110422_d_n8, assign73220_e110422_d_n9, assign73220_e110422_d_n10, assign73220_e110422_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1696 != 0.0)) && (locals.var_guard1697 != 0.0)) && (locals.var_guard1698 != 0.0)) {
        let assign73220_e110420: f64 = (0.1 * 0.1);
        (assign73220_e110420, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn13,)
    }
};
        locals.var_xmax2 = assign73220_e110422;
        locals.var_xmax2_dn0 = assign73220_e110422_d_n0;
        locals.var_xmax2_dn2 = assign73220_e110422_d_n2;
        locals.var_xmax2_dn4 = assign73220_e110422_d_n4;
        locals.var_xmax2_dn5 = assign73220_e110422_d_n5;
        locals.var_xmax2_dn6 = assign73220_e110422_d_n6;
        locals.var_xmax2_dn7 = assign73220_e110422_d_n7;
        locals.var_xmax2_dn8 = assign73220_e110422_d_n8;
        locals.var_xmax2_dn9 = assign73220_e110422_d_n9;
        locals.var_xmax2_dn10 = assign73220_e110422_d_n10;
        locals.var_xmax2_dn13 = assign73220_e110422_d_n13;
        locals.var_xmax2_rv = 0.0;

        let (assign73230_e110435, assign73230_e110435_d_n0, assign73230_e110435_d_n2, assign73230_e110435_d_n4, assign73230_e110435_d_n5, assign73230_e110435_d_n6, assign73230_e110435_d_n7, assign73230_e110435_d_n8, assign73230_e110435_d_n9, assign73230_e110435_d_n10, assign73230_e110435_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1696 != 0.0)) && (locals.var_guard1697 != 0.0)) && (locals.var_guard1698 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign73230_e110435;
        locals.var_xp_dn0 = assign73230_e110435_d_n0;
        locals.var_xp_dn2 = assign73230_e110435_d_n2;
        locals.var_xp_dn4 = assign73230_e110435_d_n4;
        locals.var_xp_dn5 = assign73230_e110435_d_n5;
        locals.var_xp_dn6 = assign73230_e110435_d_n6;
        locals.var_xp_dn7 = assign73230_e110435_d_n7;
        locals.var_xp_dn8 = assign73230_e110435_d_n8;
        locals.var_xp_dn9 = assign73230_e110435_d_n9;
        locals.var_xp_dn10 = assign73230_e110435_d_n10;
        locals.var_xp_dn13 = assign73230_e110435_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign73240_e110448, assign73240_e110448_d_n0, assign73240_e110448_d_n2, assign73240_e110448_d_n4, assign73240_e110448_d_n5, assign73240_e110448_d_n6, assign73240_e110448_d_n7, assign73240_e110448_d_n8, assign73240_e110448_d_n9, assign73240_e110448_d_n10, assign73240_e110448_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1696 != 0.0)) && (locals.var_guard1697 != 0.0)) && (locals.var_guard1698 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign73240_e110448;
        locals.var_xmp_dn0 = assign73240_e110448_d_n0;
        locals.var_xmp_dn2 = assign73240_e110448_d_n2;
        locals.var_xmp_dn4 = assign73240_e110448_d_n4;
        locals.var_xmp_dn5 = assign73240_e110448_d_n5;
        locals.var_xmp_dn6 = assign73240_e110448_d_n6;
        locals.var_xmp_dn7 = assign73240_e110448_d_n7;
        locals.var_xmp_dn8 = assign73240_e110448_d_n8;
        locals.var_xmp_dn9 = assign73240_e110448_d_n9;
        locals.var_xmp_dn10 = assign73240_e110448_d_n10;
        locals.var_xmp_dn13 = assign73240_e110448_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign73250_e110461,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1696 != 0.0)) && (locals.var_guard1697 != 0.0)) && (locals.var_guard1698 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign73250_e110461;
        locals.var_m0_rv = 0.0;

        let (assign73260_e110474,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1696 != 0.0)) && (locals.var_guard1697 != 0.0)) && (locals.var_guard1698 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign73260_e110474;
        locals.var_mm_rv = 0.0;

        let (assign73270_e110487, assign73270_e110487_d_n0, assign73270_e110487_d_n2, assign73270_e110487_d_n4, assign73270_e110487_d_n5, assign73270_e110487_d_n6, assign73270_e110487_d_n7, assign73270_e110487_d_n8, assign73270_e110487_d_n9, assign73270_e110487_d_n10, assign73270_e110487_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1696 != 0.0)) && (locals.var_guard1697 != 0.0)) && (locals.var_guard1698 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign73270_e110487;
        locals.var_arg_dn0 = assign73270_e110487_d_n0;
        locals.var_arg_dn2 = assign73270_e110487_d_n2;
        locals.var_arg_dn4 = assign73270_e110487_d_n4;
        locals.var_arg_dn5 = assign73270_e110487_d_n5;
        locals.var_arg_dn6 = assign73270_e110487_d_n6;
        locals.var_arg_dn7 = assign73270_e110487_d_n7;
        locals.var_arg_dn8 = assign73270_e110487_d_n8;
        locals.var_arg_dn9 = assign73270_e110487_d_n9;
        locals.var_arg_dn10 = assign73270_e110487_d_n10;
        locals.var_arg_dn13 = assign73270_e110487_d_n13;
        locals.var_arg_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_264(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign73280_e110500, assign73280_e110500_d_n0, assign73280_e110500_d_n2, assign73280_e110500_d_n4, assign73280_e110500_d_n5, assign73280_e110500_d_n6, assign73280_e110500_d_n7, assign73280_e110500_d_n8, assign73280_e110500_d_n9, assign73280_e110500_d_n10, assign73280_e110500_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1696 != 0.0)) && (locals.var_guard1697 != 0.0)) && (locals.var_guard1698 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign73280_e110500;
        locals.var_dnm_dn0 = assign73280_e110500_d_n0;
        locals.var_dnm_dn2 = assign73280_e110500_d_n2;
        locals.var_dnm_dn4 = assign73280_e110500_d_n4;
        locals.var_dnm_dn5 = assign73280_e110500_d_n5;
        locals.var_dnm_dn6 = assign73280_e110500_d_n6;
        locals.var_dnm_dn7 = assign73280_e110500_d_n7;
        locals.var_dnm_dn8 = assign73280_e110500_d_n8;
        locals.var_dnm_dn9 = assign73280_e110500_d_n9;
        locals.var_dnm_dn10 = assign73280_e110500_d_n10;
        locals.var_dnm_dn13 = assign73280_e110500_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign73290_e110515, assign73290_e110515_d_n0, assign73290_e110515_d_n2, assign73290_e110515_d_n4, assign73290_e110515_d_n5, assign73290_e110515_d_n6, assign73290_e110515_d_n7, assign73290_e110515_d_n8, assign73290_e110515_d_n9, assign73290_e110515_d_n10, assign73290_e110515_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1696 != 0.0)) && (locals.var_guard1697 != 0.0)) && (locals.var_guard1698 != 0.0)) {
        let assign73290_e110513: f64 = (locals.var_xp * locals.var_x2);
        (assign73290_e110513, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign73290_e110515;
        locals.var_xp_dn0 = assign73290_e110515_d_n0;
        locals.var_xp_dn2 = assign73290_e110515_d_n2;
        locals.var_xp_dn4 = assign73290_e110515_d_n4;
        locals.var_xp_dn5 = assign73290_e110515_d_n5;
        locals.var_xp_dn6 = assign73290_e110515_d_n6;
        locals.var_xp_dn7 = assign73290_e110515_d_n7;
        locals.var_xp_dn8 = assign73290_e110515_d_n8;
        locals.var_xp_dn9 = assign73290_e110515_d_n9;
        locals.var_xp_dn10 = assign73290_e110515_d_n10;
        locals.var_xp_dn13 = assign73290_e110515_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign73300_e110530, assign73300_e110530_d_n0, assign73300_e110530_d_n2, assign73300_e110530_d_n4, assign73300_e110530_d_n5, assign73300_e110530_d_n6, assign73300_e110530_d_n7, assign73300_e110530_d_n8, assign73300_e110530_d_n9, assign73300_e110530_d_n10, assign73300_e110530_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1696 != 0.0)) && (locals.var_guard1697 != 0.0)) && (locals.var_guard1698 != 0.0)) {
        let assign73300_e110528: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign73300_e110528, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign73300_e110530;
        locals.var_xmp_dn0 = assign73300_e110530_d_n0;
        locals.var_xmp_dn2 = assign73300_e110530_d_n2;
        locals.var_xmp_dn4 = assign73300_e110530_d_n4;
        locals.var_xmp_dn5 = assign73300_e110530_d_n5;
        locals.var_xmp_dn6 = assign73300_e110530_d_n6;
        locals.var_xmp_dn7 = assign73300_e110530_d_n7;
        locals.var_xmp_dn8 = assign73300_e110530_d_n8;
        locals.var_xmp_dn9 = assign73300_e110530_d_n9;
        locals.var_xmp_dn10 = assign73300_e110530_d_n10;
        locals.var_xmp_dn13 = assign73300_e110530_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign73310_e110545, assign73310_e110545_d_n0, assign73310_e110545_d_n2, assign73310_e110545_d_n4, assign73310_e110545_d_n5, assign73310_e110545_d_n6, assign73310_e110545_d_n7, assign73310_e110545_d_n8, assign73310_e110545_d_n9, assign73310_e110545_d_n10, assign73310_e110545_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1696 != 0.0)) && (locals.var_guard1697 != 0.0)) && (locals.var_guard1698 != 0.0)) {
        let assign73310_e110543: f64 = (locals.var_xp * locals.var_x2);
        (assign73310_e110543, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign73310_e110545;
        locals.var_xp_dn0 = assign73310_e110545_d_n0;
        locals.var_xp_dn2 = assign73310_e110545_d_n2;
        locals.var_xp_dn4 = assign73310_e110545_d_n4;
        locals.var_xp_dn5 = assign73310_e110545_d_n5;
        locals.var_xp_dn6 = assign73310_e110545_d_n6;
        locals.var_xp_dn7 = assign73310_e110545_d_n7;
        locals.var_xp_dn8 = assign73310_e110545_d_n8;
        locals.var_xp_dn9 = assign73310_e110545_d_n9;
        locals.var_xp_dn10 = assign73310_e110545_d_n10;
        locals.var_xp_dn13 = assign73310_e110545_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign73320_e110560, assign73320_e110560_d_n0, assign73320_e110560_d_n2, assign73320_e110560_d_n4, assign73320_e110560_d_n5, assign73320_e110560_d_n6, assign73320_e110560_d_n7, assign73320_e110560_d_n8, assign73320_e110560_d_n9, assign73320_e110560_d_n10, assign73320_e110560_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1696 != 0.0)) && (locals.var_guard1697 != 0.0)) && (locals.var_guard1698 != 0.0)) {
        let assign73320_e110558: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign73320_e110558, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign73320_e110560;
        locals.var_xmp_dn0 = assign73320_e110560_d_n0;
        locals.var_xmp_dn2 = assign73320_e110560_d_n2;
        locals.var_xmp_dn4 = assign73320_e110560_d_n4;
        locals.var_xmp_dn5 = assign73320_e110560_d_n5;
        locals.var_xmp_dn6 = assign73320_e110560_d_n6;
        locals.var_xmp_dn7 = assign73320_e110560_d_n7;
        locals.var_xmp_dn8 = assign73320_e110560_d_n8;
        locals.var_xmp_dn9 = assign73320_e110560_d_n9;
        locals.var_xmp_dn10 = assign73320_e110560_d_n10;
        locals.var_xmp_dn13 = assign73320_e110560_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign73330_e110575, assign73330_e110575_d_n0, assign73330_e110575_d_n2, assign73330_e110575_d_n4, assign73330_e110575_d_n5, assign73330_e110575_d_n6, assign73330_e110575_d_n7, assign73330_e110575_d_n8, assign73330_e110575_d_n9, assign73330_e110575_d_n10, assign73330_e110575_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1696 != 0.0)) && (locals.var_guard1697 != 0.0)) && (locals.var_guard1698 != 0.0)) {
        let assign73330_e110573: f64 = (locals.var_xp + locals.var_xmp);
        (assign73330_e110573, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn13 + locals.var_xmp_dn13),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign73330_e110575;
        locals.var_arg_dn0 = assign73330_e110575_d_n0;
        locals.var_arg_dn2 = assign73330_e110575_d_n2;
        locals.var_arg_dn4 = assign73330_e110575_d_n4;
        locals.var_arg_dn5 = assign73330_e110575_d_n5;
        locals.var_arg_dn6 = assign73330_e110575_d_n6;
        locals.var_arg_dn7 = assign73330_e110575_d_n7;
        locals.var_arg_dn8 = assign73330_e110575_d_n8;
        locals.var_arg_dn9 = assign73330_e110575_d_n9;
        locals.var_arg_dn10 = assign73330_e110575_d_n10;
        locals.var_arg_dn13 = assign73330_e110575_d_n13;
        locals.var_arg_rv = 0.0;

        let (assign73340_e110588, assign73340_e110588_d_n0, assign73340_e110588_d_n2, assign73340_e110588_d_n4, assign73340_e110588_d_n5, assign73340_e110588_d_n6, assign73340_e110588_d_n7, assign73340_e110588_d_n8, assign73340_e110588_d_n9, assign73340_e110588_d_n10, assign73340_e110588_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1696 != 0.0)) && (locals.var_guard1697 != 0.0)) && (locals.var_guard1698 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign73340_e110588;
        locals.var_dnm_dn0 = assign73340_e110588_d_n0;
        locals.var_dnm_dn2 = assign73340_e110588_d_n2;
        locals.var_dnm_dn4 = assign73340_e110588_d_n4;
        locals.var_dnm_dn5 = assign73340_e110588_d_n5;
        locals.var_dnm_dn6 = assign73340_e110588_d_n6;
        locals.var_dnm_dn7 = assign73340_e110588_d_n7;
        locals.var_dnm_dn8 = assign73340_e110588_d_n8;
        locals.var_dnm_dn9 = assign73340_e110588_d_n9;
        locals.var_dnm_dn10 = assign73340_e110588_d_n10;
        locals.var_dnm_dn13 = assign73340_e110588_d_n13;
        locals.var_dnm_rv = 0.0;

        let assign73350_e110603: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard1699 = assign73350_e110603;
        locals.var_guard1699_rv = 0.0;

        let assign73360_e110606: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1700 = assign73360_e110606;
        locals.var_guard1700_rv = 0.0;

        let (assign73370_e110623,) = {
    if (((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1696 != 0.0)) && (locals.var_guard1697 != 0.0)) && (locals.var_guard1698 != 0.0)) && (locals.var_guard1699 != 0.0)) && (locals.var_guard1700 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign73370_e110623;
        locals.var_mm_rv = 0.0;

        let assign73380_e110626: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1701 = assign73380_e110626;
        locals.var_guard1701_rv = 0.0;

        let (assign73390_e110646,) = {
    if ((((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1696 != 0.0)) && (locals.var_guard1697 != 0.0)) && (locals.var_guard1698 != 0.0)) && (locals.var_guard1699 != 0.0)) && (locals.var_guard1700 == 0.0)) && (locals.var_guard1701 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign73390_e110646;
        locals.var_mm_rv = 0.0;

        let assign73400_e110649: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard1702 = assign73400_e110649;
        locals.var_guard1702_rv = 0.0;

        let (assign73410_e110672,) = {
    if (((((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1696 != 0.0)) && (locals.var_guard1697 != 0.0)) && (locals.var_guard1698 != 0.0)) && (locals.var_guard1699 != 0.0)) && (locals.var_guard1700 == 0.0)) && (locals.var_guard1701 == 0.0)) && (locals.var_guard1702 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign73410_e110672;
        locals.var_mm_rv = 0.0;

        let assign73420_e110675: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard1703 = assign73420_e110675;
        locals.var_guard1703_rv = 0.0;

        let (assign73430_e110701,) = {
    if ((((((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1696 != 0.0)) && (locals.var_guard1697 != 0.0)) && (locals.var_guard1698 != 0.0)) && (locals.var_guard1699 != 0.0)) && (locals.var_guard1700 == 0.0)) && (locals.var_guard1701 == 0.0)) && (locals.var_guard1702 == 0.0)) && (locals.var_guard1703 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign73430_e110701;
        locals.var_mm_rv = 0.0;

        let (assign73440_e110716,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1696 != 0.0)) && (locals.var_guard1697 != 0.0)) && (locals.var_guard1698 != 0.0)) && (locals.var_guard1699 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign73440_e110716;
        locals.var_m0_rv = 0.0;

        let mut assign73450_loop_guard: usize = 0;
        while {
            let assign73450_cond_e110732: f64 = if (((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1696 != 0.0)) && (locals.var_guard1697 != 0.0)) && (locals.var_guard1698 != 0.0)) && (locals.var_guard1699 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign73450_cond_e110732 != 0.0
        } {
            assign73450_loop_guard += 1;
            assert!(assign73450_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign73450_body0_e110748, assign73450_body0_e110748_d_n0, assign73450_body0_e110748_d_n2, assign73450_body0_e110748_d_n4, assign73450_body0_e110748_d_n5, assign73450_body0_e110748_d_n6, assign73450_body0_e110748_d_n7, assign73450_body0_e110748_d_n8, assign73450_body0_e110748_d_n9, assign73450_body0_e110748_d_n10, assign73450_body0_e110748_d_n13,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1696 != 0.0)) && (locals.var_guard1697 != 0.0)) && (locals.var_guard1698 != 0.0)) && (locals.var_guard1699 != 0.0)) {
        let assign73450_body0_e110746: f64 = (locals.var_dnm).sqrt();
        (assign73450_body0_e110746, (locals.var_dnm_dn0 / (2.0 * assign73450_body0_e110746)), (locals.var_dnm_dn2 / (2.0 * assign73450_body0_e110746)), (locals.var_dnm_dn4 / (2.0 * assign73450_body0_e110746)), (locals.var_dnm_dn5 / (2.0 * assign73450_body0_e110746)), (locals.var_dnm_dn6 / (2.0 * assign73450_body0_e110746)), (locals.var_dnm_dn7 / (2.0 * assign73450_body0_e110746)), (locals.var_dnm_dn8 / (2.0 * assign73450_body0_e110746)), (locals.var_dnm_dn9 / (2.0 * assign73450_body0_e110746)), (locals.var_dnm_dn10 / (2.0 * assign73450_body0_e110746)), (locals.var_dnm_dn13 / (2.0 * assign73450_body0_e110746)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
            locals.var_dnm = assign73450_body0_e110748;
            locals.var_dnm_dn0 = assign73450_body0_e110748_d_n0;
            locals.var_dnm_dn2 = assign73450_body0_e110748_d_n2;
            locals.var_dnm_dn4 = assign73450_body0_e110748_d_n4;
            locals.var_dnm_dn5 = assign73450_body0_e110748_d_n5;
            locals.var_dnm_dn6 = assign73450_body0_e110748_d_n6;
            locals.var_dnm_dn7 = assign73450_body0_e110748_d_n7;
            locals.var_dnm_dn8 = assign73450_body0_e110748_d_n8;
            locals.var_dnm_dn9 = assign73450_body0_e110748_d_n9;
            locals.var_dnm_dn10 = assign73450_body0_e110748_d_n10;
            locals.var_dnm_dn13 = assign73450_body0_e110748_d_n13;
            locals.var_dnm_rv = 0.0;
            let (assign73450_body1_e110765,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1696 != 0.0)) && (locals.var_guard1697 != 0.0)) && (locals.var_guard1698 != 0.0)) && (locals.var_guard1699 != 0.0)) {
        let assign73450_body1_e110763: f64 = (locals.var_m0 + 1.0);
        (assign73450_body1_e110763,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign73450_body1_e110765;
            locals.var_m0_rv = 0.0;
        }

        let (assign73460_e110792, assign73460_e110792_d_n0, assign73460_e110792_d_n2, assign73460_e110792_d_n4, assign73460_e110792_d_n5, assign73460_e110792_d_n6, assign73460_e110792_d_n7, assign73460_e110792_d_n8, assign73460_e110792_d_n9, assign73460_e110792_d_n10, assign73460_e110792_d_n13,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1696 != 0.0)) && (locals.var_guard1697 != 0.0)) && (locals.var_guard1698 != 0.0)) && (locals.var_guard1699 == 0.0)) {
        let (assign73460_e110790, assign73460_e110790_d_n0, assign73460_e110790_d_n2, assign73460_e110790_d_n4, assign73460_e110790_d_n5, assign73460_e110790_d_n6, assign73460_e110790_d_n7, assign73460_e110790_d_n8, assign73460_e110790_d_n9, assign73460_e110790_d_n10, assign73460_e110790_d_n13,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign73460_e110787: f64 = (2.0 * 2.0);
                let assign73460_e110788: f64 = (1.0 / assign73460_e110787);
                let assign73460_e110789: f64 = (locals.var_dnm).powf(assign73460_e110788);
                (assign73460_e110789, if 0.0 == 0.0 && ((assign73460_e110788) as f64).is_finite() && ((assign73460_e110788) as f64).fract() == 0.0 { if assign73460_e110788 == 0.0 { 0.0 } else { (assign73460_e110788 * ((locals.var_dnm).powf(assign73460_e110788 - 1.0) * locals.var_dnm_dn0)) } } else { (assign73460_e110789 * (assign73460_e110788 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign73460_e110788) as f64).is_finite() && ((assign73460_e110788) as f64).fract() == 0.0 { if assign73460_e110788 == 0.0 { 0.0 } else { (assign73460_e110788 * ((locals.var_dnm).powf(assign73460_e110788 - 1.0) * locals.var_dnm_dn2)) } } else { (assign73460_e110789 * (assign73460_e110788 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign73460_e110788) as f64).is_finite() && ((assign73460_e110788) as f64).fract() == 0.0 { if assign73460_e110788 == 0.0 { 0.0 } else { (assign73460_e110788 * ((locals.var_dnm).powf(assign73460_e110788 - 1.0) * locals.var_dnm_dn4)) } } else { (assign73460_e110789 * (assign73460_e110788 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign73460_e110788) as f64).is_finite() && ((assign73460_e110788) as f64).fract() == 0.0 { if assign73460_e110788 == 0.0 { 0.0 } else { (assign73460_e110788 * ((locals.var_dnm).powf(assign73460_e110788 - 1.0) * locals.var_dnm_dn5)) } } else { (assign73460_e110789 * (assign73460_e110788 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign73460_e110788) as f64).is_finite() && ((assign73460_e110788) as f64).fract() == 0.0 { if assign73460_e110788 == 0.0 { 0.0 } else { (assign73460_e110788 * ((locals.var_dnm).powf(assign73460_e110788 - 1.0) * locals.var_dnm_dn6)) } } else { (assign73460_e110789 * (assign73460_e110788 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign73460_e110788) as f64).is_finite() && ((assign73460_e110788) as f64).fract() == 0.0 { if assign73460_e110788 == 0.0 { 0.0 } else { (assign73460_e110788 * ((locals.var_dnm).powf(assign73460_e110788 - 1.0) * locals.var_dnm_dn7)) } } else { (assign73460_e110789 * (assign73460_e110788 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign73460_e110788) as f64).is_finite() && ((assign73460_e110788) as f64).fract() == 0.0 { if assign73460_e110788 == 0.0 { 0.0 } else { (assign73460_e110788 * ((locals.var_dnm).powf(assign73460_e110788 - 1.0) * locals.var_dnm_dn8)) } } else { (assign73460_e110789 * (assign73460_e110788 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign73460_e110788) as f64).is_finite() && ((assign73460_e110788) as f64).fract() == 0.0 { if assign73460_e110788 == 0.0 { 0.0 } else { (assign73460_e110788 * ((locals.var_dnm).powf(assign73460_e110788 - 1.0) * locals.var_dnm_dn9)) } } else { (assign73460_e110789 * (assign73460_e110788 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign73460_e110788) as f64).is_finite() && ((assign73460_e110788) as f64).fract() == 0.0 { if assign73460_e110788 == 0.0 { 0.0 } else { (assign73460_e110788 * ((locals.var_dnm).powf(assign73460_e110788 - 1.0) * locals.var_dnm_dn10)) } } else { (assign73460_e110789 * (assign73460_e110788 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign73460_e110788) as f64).is_finite() && ((assign73460_e110788) as f64).fract() == 0.0 { if assign73460_e110788 == 0.0 { 0.0 } else { (assign73460_e110788 * ((locals.var_dnm).powf(assign73460_e110788 - 1.0) * locals.var_dnm_dn13)) } } else { (assign73460_e110789 * (assign73460_e110788 * (locals.var_dnm_dn13 / locals.var_dnm))) },)
            }
        };
        (assign73460_e110790, assign73460_e110790_d_n0, assign73460_e110790_d_n2, assign73460_e110790_d_n4, assign73460_e110790_d_n5, assign73460_e110790_d_n6, assign73460_e110790_d_n7, assign73460_e110790_d_n8, assign73460_e110790_d_n9, assign73460_e110790_d_n10, assign73460_e110790_d_n13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign73460_e110792;
        locals.var_dnm_dn0 = assign73460_e110792_d_n0;
        locals.var_dnm_dn2 = assign73460_e110792_d_n2;
        locals.var_dnm_dn4 = assign73460_e110792_d_n4;
        locals.var_dnm_dn5 = assign73460_e110792_d_n5;
        locals.var_dnm_dn6 = assign73460_e110792_d_n6;
        locals.var_dnm_dn7 = assign73460_e110792_d_n7;
        locals.var_dnm_dn8 = assign73460_e110792_d_n8;
        locals.var_dnm_dn9 = assign73460_e110792_d_n9;
        locals.var_dnm_dn10 = assign73460_e110792_d_n10;
        locals.var_dnm_dn13 = assign73460_e110792_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign73470_e110807, assign73470_e110807_d_n0, assign73470_e110807_d_n2, assign73470_e110807_d_n4, assign73470_e110807_d_n5, assign73470_e110807_d_n6, assign73470_e110807_d_n7, assign73470_e110807_d_n8, assign73470_e110807_d_n9, assign73470_e110807_d_n10, assign73470_e110807_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1696 != 0.0)) && (locals.var_guard1697 != 0.0)) && (locals.var_guard1698 != 0.0)) {
        let assign73470_e110805: f64 = (1.0 / locals.var_dnm);
        (assign73470_e110805, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn13 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign73470_e110807;
        locals.var_dnm_dn0 = assign73470_e110807_d_n0;
        locals.var_dnm_dn2 = assign73470_e110807_d_n2;
        locals.var_dnm_dn4 = assign73470_e110807_d_n4;
        locals.var_dnm_dn5 = assign73470_e110807_d_n5;
        locals.var_dnm_dn6 = assign73470_e110807_d_n6;
        locals.var_dnm_dn7 = assign73470_e110807_d_n7;
        locals.var_dnm_dn8 = assign73470_e110807_d_n8;
        locals.var_dnm_dn9 = assign73470_e110807_d_n9;
        locals.var_dnm_dn10 = assign73470_e110807_d_n10;
        locals.var_dnm_dn13 = assign73470_e110807_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign73480_e110824, assign73480_e110824_d_n0, assign73480_e110824_d_n2, assign73480_e110824_d_n4, assign73480_e110824_d_n5, assign73480_e110824_d_n6, assign73480_e110824_d_n7, assign73480_e110824_d_n8, assign73480_e110824_d_n9, assign73480_e110824_d_n10, assign73480_e110824_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1696 != 0.0)) && (locals.var_guard1697 != 0.0)) && (locals.var_guard1698 != 0.0)) {
        let assign73480_e110820: f64 = (locals.var_tmf1 * 0.1);
        let assign73480_e110822: f64 = (assign73480_e110820 * locals.var_dnm);
        (assign73480_e110822, (((locals.var_tmf1_dn0 * 0.1) * locals.var_dnm) + (assign73480_e110820 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * 0.1) * locals.var_dnm) + (assign73480_e110820 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn4 * 0.1) * locals.var_dnm) + (assign73480_e110820 * locals.var_dnm_dn4)), (((locals.var_tmf1_dn5 * 0.1) * locals.var_dnm) + (assign73480_e110820 * locals.var_dnm_dn5)), (((locals.var_tmf1_dn6 * 0.1) * locals.var_dnm) + (assign73480_e110820 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * 0.1) * locals.var_dnm) + (assign73480_e110820 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn8 * 0.1) * locals.var_dnm) + (assign73480_e110820 * locals.var_dnm_dn8)), (((locals.var_tmf1_dn9 * 0.1) * locals.var_dnm) + (assign73480_e110820 * locals.var_dnm_dn9)), (((locals.var_tmf1_dn10 * 0.1) * locals.var_dnm) + (assign73480_e110820 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn13 * 0.1) * locals.var_dnm) + (assign73480_e110820 * locals.var_dnm_dn13)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn13,)
    }
};
        locals.var_tmf0 = assign73480_e110824;
        locals.var_tmf0_dn0 = assign73480_e110824_d_n0;
        locals.var_tmf0_dn2 = assign73480_e110824_d_n2;
        locals.var_tmf0_dn4 = assign73480_e110824_d_n4;
        locals.var_tmf0_dn5 = assign73480_e110824_d_n5;
        locals.var_tmf0_dn6 = assign73480_e110824_d_n6;
        locals.var_tmf0_dn7 = assign73480_e110824_d_n7;
        locals.var_tmf0_dn8 = assign73480_e110824_d_n8;
        locals.var_tmf0_dn9 = assign73480_e110824_d_n9;
        locals.var_tmf0_dn10 = assign73480_e110824_d_n10;
        locals.var_tmf0_dn13 = assign73480_e110824_d_n13;
        locals.var_tmf0_rv = 0.0;

        let (assign73490_e110843, assign73490_e110843_d_n0, assign73490_e110843_d_n2, assign73490_e110843_d_n4, assign73490_e110843_d_n5, assign73490_e110843_d_n6, assign73490_e110843_d_n7, assign73490_e110843_d_n8, assign73490_e110843_d_n9, assign73490_e110843_d_n10, assign73490_e110843_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1696 != 0.0)) && (locals.var_guard1697 != 0.0)) && (locals.var_guard1698 != 0.0)) {
        let assign73490_e110837: f64 = (0.1 * locals.var_xmp);
        let assign73490_e110839: f64 = (assign73490_e110837 * locals.var_dnm);
        let assign73490_e110841: f64 = (assign73490_e110839 / locals.var_arg);
        (assign73490_e110841, ((((((0.1 * locals.var_xmp_dn0) * locals.var_dnm) + (assign73490_e110837 * locals.var_dnm_dn0)) * locals.var_arg) - (assign73490_e110839 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn2) * locals.var_dnm) + (assign73490_e110837 * locals.var_dnm_dn2)) * locals.var_arg) - (assign73490_e110839 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn4) * locals.var_dnm) + (assign73490_e110837 * locals.var_dnm_dn4)) * locals.var_arg) - (assign73490_e110839 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn5) * locals.var_dnm) + (assign73490_e110837 * locals.var_dnm_dn5)) * locals.var_arg) - (assign73490_e110839 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn6) * locals.var_dnm) + (assign73490_e110837 * locals.var_dnm_dn6)) * locals.var_arg) - (assign73490_e110839 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn7) * locals.var_dnm) + (assign73490_e110837 * locals.var_dnm_dn7)) * locals.var_arg) - (assign73490_e110839 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn8) * locals.var_dnm) + (assign73490_e110837 * locals.var_dnm_dn8)) * locals.var_arg) - (assign73490_e110839 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn9) * locals.var_dnm) + (assign73490_e110837 * locals.var_dnm_dn9)) * locals.var_arg) - (assign73490_e110839 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn10) * locals.var_dnm) + (assign73490_e110837 * locals.var_dnm_dn10)) * locals.var_arg) - (assign73490_e110839 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn13) * locals.var_dnm) + (assign73490_e110837 * locals.var_dnm_dn13)) * locals.var_arg) - (assign73490_e110839 * locals.var_arg_dn13)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign73490_e110843;
        locals.var_t0_dn0 = assign73490_e110843_d_n0;
        locals.var_t0_dn2 = assign73490_e110843_d_n2;
        locals.var_t0_dn4 = assign73490_e110843_d_n4;
        locals.var_t0_dn5 = assign73490_e110843_d_n5;
        locals.var_t0_dn6 = assign73490_e110843_d_n6;
        locals.var_t0_dn7 = assign73490_e110843_d_n7;
        locals.var_t0_dn8 = assign73490_e110843_d_n8;
        locals.var_t0_dn9 = assign73490_e110843_d_n9;
        locals.var_t0_dn10 = assign73490_e110843_d_n10;
        locals.var_t0_dn13 = assign73490_e110843_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign73500_e110860, assign73500_e110860_d_n0, assign73500_e110860_d_n2, assign73500_e110860_d_n4, assign73500_e110860_d_n5, assign73500_e110860_d_n6, assign73500_e110860_d_n7, assign73500_e110860_d_n8, assign73500_e110860_d_n9, assign73500_e110860_d_n10, assign73500_e110860_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1696 != 0.0)) && (locals.var_guard1697 != 0.0)) && (locals.var_guard1698 != 0.0)) {
        let assign73500_e110856: f64 = (locals.var_ps0ld_bef1 - 0.1);
        let assign73500_e110858: f64 = (assign73500_e110856 + locals.var_tmf0);
        (assign73500_e110858, (locals.var_ps0ld_bef1_dn0 + locals.var_tmf0_dn0), (locals.var_ps0ld_bef1_dn2 + locals.var_tmf0_dn2), (locals.var_ps0ld_bef1_dn4 + locals.var_tmf0_dn4), (locals.var_ps0ld_bef1_dn5 + locals.var_tmf0_dn5), (locals.var_ps0ld_bef1_dn6 + locals.var_tmf0_dn6), (locals.var_ps0ld_bef1_dn7 + locals.var_tmf0_dn7), (locals.var_ps0ld_bef1_dn8 + locals.var_tmf0_dn8), (locals.var_ps0ld_bef1_dn9 + locals.var_tmf0_dn9), (locals.var_ps0ld_bef1_dn10 + locals.var_tmf0_dn10), (locals.var_ps0ld_bef1_dn13 + locals.var_tmf0_dn13),)
    } else {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn13,)
    }
};
        locals.var_ps0ld = assign73500_e110860;
        locals.var_ps0ld_dn0 = assign73500_e110860_d_n0;
        locals.var_ps0ld_dn2 = assign73500_e110860_d_n2;
        locals.var_ps0ld_dn4 = assign73500_e110860_d_n4;
        locals.var_ps0ld_dn5 = assign73500_e110860_d_n5;
        locals.var_ps0ld_dn6 = assign73500_e110860_d_n6;
        locals.var_ps0ld_dn7 = assign73500_e110860_d_n7;
        locals.var_ps0ld_dn8 = assign73500_e110860_d_n8;
        locals.var_ps0ld_dn9 = assign73500_e110860_d_n9;
        locals.var_ps0ld_dn10 = assign73500_e110860_d_n10;
        locals.var_ps0ld_dn13 = assign73500_e110860_d_n13;
        locals.var_ps0ld_rv = 0.0;

        let (assign73510_e110873, assign73510_e110873_d_n0, assign73510_e110873_d_n2, assign73510_e110873_d_n4, assign73510_e110873_d_n5, assign73510_e110873_d_n6, assign73510_e110873_d_n7, assign73510_e110873_d_n8, assign73510_e110873_d_n9, assign73510_e110873_d_n10, assign73510_e110873_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1696 != 0.0)) && (locals.var_guard1697 != 0.0)) && (locals.var_guard1698 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign73510_e110873;
        locals.var_t0_dn0 = assign73510_e110873_d_n0;
        locals.var_t0_dn2 = assign73510_e110873_d_n2;
        locals.var_t0_dn4 = assign73510_e110873_d_n4;
        locals.var_t0_dn5 = assign73510_e110873_d_n5;
        locals.var_t0_dn6 = assign73510_e110873_d_n6;
        locals.var_t0_dn7 = assign73510_e110873_d_n7;
        locals.var_t0_dn8 = assign73510_e110873_d_n8;
        locals.var_t0_dn9 = assign73510_e110873_d_n9;
        locals.var_t0_dn10 = assign73510_e110873_d_n10;
        locals.var_t0_dn13 = assign73510_e110873_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign73520_e110887, assign73520_e110887_d_n0, assign73520_e110887_d_n2, assign73520_e110887_d_n4, assign73520_e110887_d_n5, assign73520_e110887_d_n6, assign73520_e110887_d_n7, assign73520_e110887_d_n8, assign73520_e110887_d_n9, assign73520_e110887_d_n10, assign73520_e110887_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1696 != 0.0)) && (locals.var_guard1697 != 0.0)) && (locals.var_guard1698 == 0.0)) {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn13,)
    } else {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn13,)
    }
};
        locals.var_ps0ld = assign73520_e110887;
        locals.var_ps0ld_dn0 = assign73520_e110887_d_n0;
        locals.var_ps0ld_dn2 = assign73520_e110887_d_n2;
        locals.var_ps0ld_dn4 = assign73520_e110887_d_n4;
        locals.var_ps0ld_dn5 = assign73520_e110887_d_n5;
        locals.var_ps0ld_dn6 = assign73520_e110887_d_n6;
        locals.var_ps0ld_dn7 = assign73520_e110887_d_n7;
        locals.var_ps0ld_dn8 = assign73520_e110887_d_n8;
        locals.var_ps0ld_dn9 = assign73520_e110887_d_n9;
        locals.var_ps0ld_dn10 = assign73520_e110887_d_n10;
        locals.var_ps0ld_dn13 = assign73520_e110887_d_n13;
        locals.var_ps0ld_rv = 0.0;

        let (assign73530_e110901, assign73530_e110901_d_n0, assign73530_e110901_d_n2, assign73530_e110901_d_n4, assign73530_e110901_d_n5, assign73530_e110901_d_n6, assign73530_e110901_d_n7, assign73530_e110901_d_n8, assign73530_e110901_d_n9, assign73530_e110901_d_n10, assign73530_e110901_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1696 != 0.0)) && (locals.var_guard1697 != 0.0)) && (locals.var_guard1698 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign73530_e110901;
        locals.var_t0_dn0 = assign73530_e110901_d_n0;
        locals.var_t0_dn2 = assign73530_e110901_d_n2;
        locals.var_t0_dn4 = assign73530_e110901_d_n4;
        locals.var_t0_dn5 = assign73530_e110901_d_n5;
        locals.var_t0_dn6 = assign73530_e110901_d_n6;
        locals.var_t0_dn7 = assign73530_e110901_d_n7;
        locals.var_t0_dn8 = assign73530_e110901_d_n8;
        locals.var_t0_dn9 = assign73530_e110901_d_n9;
        locals.var_t0_dn10 = assign73530_e110901_d_n10;
        locals.var_t0_dn13 = assign73530_e110901_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign73540_e110918, assign73540_e110918_d_n0, assign73540_e110918_d_n2, assign73540_e110918_d_n4, assign73540_e110918_d_n5, assign73540_e110918_d_n6, assign73540_e110918_d_n7, assign73540_e110918_d_n8, assign73540_e110918_d_n9, assign73540_e110918_d_n10, assign73540_e110918_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1696 != 0.0)) && (locals.var_guard1697 == 0.0)) {
        let (assign73540_e110916, assign73540_e110916_d_n0, assign73540_e110916_d_n2, assign73540_e110916_d_n4, assign73540_e110916_d_n5, assign73540_e110916_d_n6, assign73540_e110916_d_n7, assign73540_e110916_d_n8, assign73540_e110916_d_n9, assign73540_e110916_d_n10, assign73540_e110916_d_n13,) = {
            if (locals.var_ps0ld <= locals.var_ps0ld_bef1) {
                (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn13,)
            } else {
                (locals.var_ps0ld_bef1, locals.var_ps0ld_bef1_dn0, locals.var_ps0ld_bef1_dn2, locals.var_ps0ld_bef1_dn4, locals.var_ps0ld_bef1_dn5, locals.var_ps0ld_bef1_dn6, locals.var_ps0ld_bef1_dn7, locals.var_ps0ld_bef1_dn8, locals.var_ps0ld_bef1_dn9, locals.var_ps0ld_bef1_dn10, locals.var_ps0ld_bef1_dn13,)
            }
        };
        (assign73540_e110916, assign73540_e110916_d_n0, assign73540_e110916_d_n2, assign73540_e110916_d_n4, assign73540_e110916_d_n5, assign73540_e110916_d_n6, assign73540_e110916_d_n7, assign73540_e110916_d_n8, assign73540_e110916_d_n9, assign73540_e110916_d_n10, assign73540_e110916_d_n13,)
    } else {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn13,)
    }
};
        locals.var_ps0ld = assign73540_e110918;
        locals.var_ps0ld_dn0 = assign73540_e110918_d_n0;
        locals.var_ps0ld_dn2 = assign73540_e110918_d_n2;
        locals.var_ps0ld_dn4 = assign73540_e110918_d_n4;
        locals.var_ps0ld_dn5 = assign73540_e110918_d_n5;
        locals.var_ps0ld_dn6 = assign73540_e110918_d_n6;
        locals.var_ps0ld_dn7 = assign73540_e110918_d_n7;
        locals.var_ps0ld_dn8 = assign73540_e110918_d_n8;
        locals.var_ps0ld_dn9 = assign73540_e110918_d_n9;
        locals.var_ps0ld_dn10 = assign73540_e110918_d_n10;
        locals.var_ps0ld_dn13 = assign73540_e110918_d_n13;
        locals.var_ps0ld_rv = 0.0;

        let (assign73550_e110925, assign73550_e110925_d_n0, assign73550_e110925_d_n2, assign73550_e110925_d_n4, assign73550_e110925_d_n5, assign73550_e110925_d_n6, assign73550_e110925_d_n7, assign73550_e110925_d_n8, assign73550_e110925_d_n9, assign73550_e110925_d_n10, assign73550_e110925_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn13,)
    } else {
        (locals.var_ps0ld_ini, locals.var_ps0ld_ini_dn0, locals.var_ps0ld_ini_dn2, locals.var_ps0ld_ini_dn4, locals.var_ps0ld_ini_dn5, locals.var_ps0ld_ini_dn6, locals.var_ps0ld_ini_dn7, locals.var_ps0ld_ini_dn8, locals.var_ps0ld_ini_dn9, locals.var_ps0ld_ini_dn10, locals.var_ps0ld_ini_dn13,)
    }
};
        locals.var_ps0ld_ini = assign73550_e110925;
        locals.var_ps0ld_ini_dn0 = assign73550_e110925_d_n0;
        locals.var_ps0ld_ini_dn2 = assign73550_e110925_d_n2;
        locals.var_ps0ld_ini_dn4 = assign73550_e110925_d_n4;
        locals.var_ps0ld_ini_dn5 = assign73550_e110925_d_n5;
        locals.var_ps0ld_ini_dn6 = assign73550_e110925_d_n6;
        locals.var_ps0ld_ini_dn7 = assign73550_e110925_d_n7;
        locals.var_ps0ld_ini_dn8 = assign73550_e110925_d_n8;
        locals.var_ps0ld_ini_dn9 = assign73550_e110925_d_n9;
        locals.var_ps0ld_ini_dn10 = assign73550_e110925_d_n10;
        locals.var_ps0ld_ini_dn13 = assign73550_e110925_d_n13;
        locals.var_ps0ld_ini_rv = 0.0;

        let assign73560_e110928: f64 = if p.p33 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1704 = assign73560_e110928;
        locals.var_guard1704_rv = 0.0;

        let (assign73570_e110937,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1704 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_flg_conv,)
    }
};
        locals.var_flg_conv = assign73570_e110937;
        locals.var_flg_conv_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_265(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign73580_e110953, assign73580_e110953_d_n0, assign73580_e110953_d_n2, assign73580_e110953_d_n4, assign73580_e110953_d_n5, assign73580_e110953_d_n6, assign73580_e110953_d_n7, assign73580_e110953_d_n8, assign73580_e110953_d_n9, assign73580_e110953_d_n10, assign73580_e110953_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1704 != 0.0)) {
        let assign73580_e110947: f64 = (1.034943e-10 / locals.var_q_nsubld);
        let assign73580_e110949: f64 = (assign73580_e110947 * locals.var_beta_inv);
        let assign73580_e110950: f64 = (2.0 * assign73580_e110949);
        let assign73580_e110951: f64 = (assign73580_e110950).sqrt();
        (assign73580_e110951, ((2.0 * (assign73580_e110947 * locals.var_beta_inv_dn0)) / (2.0 * assign73580_e110951)), ((2.0 * (assign73580_e110947 * locals.var_beta_inv_dn2)) / (2.0 * assign73580_e110951)), ((2.0 * (assign73580_e110947 * locals.var_beta_inv_dn4)) / (2.0 * assign73580_e110951)), ((2.0 * (assign73580_e110947 * locals.var_beta_inv_dn5)) / (2.0 * assign73580_e110951)), ((2.0 * (assign73580_e110947 * locals.var_beta_inv_dn6)) / (2.0 * assign73580_e110951)), ((2.0 * (assign73580_e110947 * locals.var_beta_inv_dn7)) / (2.0 * assign73580_e110951)), ((2.0 * (assign73580_e110947 * locals.var_beta_inv_dn8)) / (2.0 * assign73580_e110951)), ((2.0 * (assign73580_e110947 * locals.var_beta_inv_dn9)) / (2.0 * assign73580_e110951)), ((2.0 * (assign73580_e110947 * locals.var_beta_inv_dn10)) / (2.0 * assign73580_e110951)), ((2.0 * (assign73580_e110947 * locals.var_beta_inv_dn13)) / (2.0 * assign73580_e110951)),)
    } else {
        (locals.var_c_w_ld, locals.var_c_w_ld_dn0, locals.var_c_w_ld_dn2, locals.var_c_w_ld_dn4, locals.var_c_w_ld_dn5, locals.var_c_w_ld_dn6, locals.var_c_w_ld_dn7, locals.var_c_w_ld_dn8, locals.var_c_w_ld_dn9, locals.var_c_w_ld_dn10, locals.var_c_w_ld_dn13,)
    }
};
        locals.var_c_w_ld = assign73580_e110953;
        locals.var_c_w_ld_dn0 = assign73580_e110953_d_n0;
        locals.var_c_w_ld_dn2 = assign73580_e110953_d_n2;
        locals.var_c_w_ld_dn4 = assign73580_e110953_d_n4;
        locals.var_c_w_ld_dn5 = assign73580_e110953_d_n5;
        locals.var_c_w_ld_dn6 = assign73580_e110953_d_n6;
        locals.var_c_w_ld_dn7 = assign73580_e110953_d_n7;
        locals.var_c_w_ld_dn8 = assign73580_e110953_d_n8;
        locals.var_c_w_ld_dn9 = assign73580_e110953_d_n9;
        locals.var_c_w_ld_dn10 = assign73580_e110953_d_n10;
        locals.var_c_w_ld_dn13 = assign73580_e110953_d_n13;
        locals.var_c_w_ld_rv = 0.0;

        let assign73590_e110956: f64 = if locals.var_wdep_func > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1705 = assign73590_e110956;
        locals.var_guard1705_rv = 0.0;

        let (assign73600_e110969, assign73600_e110969_d_n0, assign73600_e110969_d_n2, assign73600_e110969_d_n4, assign73600_e110969_d_n5, assign73600_e110969_d_n6, assign73600_e110969_d_n7, assign73600_e110969_d_n8, assign73600_e110969_d_n9, assign73600_e110969_d_n10, assign73600_e110969_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1704 != 0.0)) && (locals.var_guard1705 != 0.0)) {
        let assign73600_e110967: f64 = (p.p334 - locals.var_wdep_func);
        (assign73600_e110967, (-locals.var_wdep_func_dn0), (-locals.var_wdep_func_dn2), (-locals.var_wdep_func_dn4), (-locals.var_wdep_func_dn5), (-locals.var_wdep_func_dn6), (-locals.var_wdep_func_dn7), (-locals.var_wdep_func_dn8), (-locals.var_wdep_func_dn9), (-locals.var_wdep_func_dn10), (-locals.var_wdep_func_dn13),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign73600_e110969;
        locals.var_t2_dn0 = assign73600_e110969_d_n0;
        locals.var_t2_dn2 = assign73600_e110969_d_n2;
        locals.var_t2_dn4 = assign73600_e110969_d_n4;
        locals.var_t2_dn5 = assign73600_e110969_d_n5;
        locals.var_t2_dn6 = assign73600_e110969_d_n6;
        locals.var_t2_dn7 = assign73600_e110969_d_n7;
        locals.var_t2_dn8 = assign73600_e110969_d_n8;
        locals.var_t2_dn9 = assign73600_e110969_d_n9;
        locals.var_t2_dn10 = assign73600_e110969_d_n10;
        locals.var_t2_dn13 = assign73600_e110969_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign73610_e110994, assign73610_e110994_d_n0, assign73610_e110994_d_n2, assign73610_e110994_d_n4, assign73610_e110994_d_n5, assign73610_e110994_d_n6, assign73610_e110994_d_n7, assign73610_e110994_d_n8, assign73610_e110994_d_n9, assign73610_e110994_d_n10, assign73610_e110994_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1704 != 0.0)) && (locals.var_guard1705 == 0.0)) {
        let assign73610_e110981: f64 = (locals.var_vdsi + p.p137);
        let assign73610_e110984: f64 = (locals.var_vdsi + p.p137);
        let assign73610_e110985: f64 = (assign73610_e110981 * assign73610_e110984);
        let assign73610_e110988: f64 = (4.0 * 0.1);
        let assign73610_e110990: f64 = (assign73610_e110988 * 0.1);
        let assign73610_e110991: f64 = (assign73610_e110985 + assign73610_e110990);
        let assign73610_e110992: f64 = (assign73610_e110991).sqrt();
        (assign73610_e110992, 0.0, 0.0, 0.0, (((locals.var_vdsi_dn5 * assign73610_e110984) + (assign73610_e110981 * locals.var_vdsi_dn5)) / (2.0 * assign73610_e110992)), 0.0, (((locals.var_vdsi_dn7 * assign73610_e110984) + (assign73610_e110981 * locals.var_vdsi_dn7)) / (2.0 * assign73610_e110992)), 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign73610_e110994;
        locals.var_tmf2_dn0 = assign73610_e110994_d_n0;
        locals.var_tmf2_dn2 = assign73610_e110994_d_n2;
        locals.var_tmf2_dn4 = assign73610_e110994_d_n4;
        locals.var_tmf2_dn5 = assign73610_e110994_d_n5;
        locals.var_tmf2_dn6 = assign73610_e110994_d_n6;
        locals.var_tmf2_dn7 = assign73610_e110994_d_n7;
        locals.var_tmf2_dn8 = assign73610_e110994_d_n8;
        locals.var_tmf2_dn9 = assign73610_e110994_d_n9;
        locals.var_tmf2_dn10 = assign73610_e110994_d_n10;
        locals.var_tmf2_dn13 = assign73610_e110994_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign73620_e111014, assign73620_e111014_d_n0, assign73620_e111014_d_n2, assign73620_e111014_d_n4, assign73620_e111014_d_n5, assign73620_e111014_d_n6, assign73620_e111014_d_n7, assign73620_e111014_d_n8, assign73620_e111014_d_n9, assign73620_e111014_d_n10, assign73620_e111014_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1704 != 0.0)) && (locals.var_guard1705 == 0.0)) {
        let assign73620_e111008: f64 = (locals.var_vdsi + p.p137);
        let assign73620_e111010: f64 = (assign73620_e111008 / locals.var_tmf2);
        let assign73620_e111011: f64 = (1.0 + assign73620_e111010);
        let assign73620_e111012: f64 = (0.5 * assign73620_e111011);
        (assign73620_e111012, (0.5 * (-((assign73620_e111008 * locals.var_tmf2_dn0) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign73620_e111008 * locals.var_tmf2_dn2) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign73620_e111008 * locals.var_tmf2_dn4) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (((locals.var_vdsi_dn5 * locals.var_tmf2) - (assign73620_e111008 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (-((assign73620_e111008 * locals.var_tmf2_dn6) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (((locals.var_vdsi_dn7 * locals.var_tmf2) - (assign73620_e111008 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (-((assign73620_e111008 * locals.var_tmf2_dn8) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign73620_e111008 * locals.var_tmf2_dn9) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign73620_e111008 * locals.var_tmf2_dn10) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign73620_e111008 * locals.var_tmf2_dn13) / (locals.var_tmf2 * locals.var_tmf2)))),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign73620_e111014;
        locals.var_t9_dn0 = assign73620_e111014_d_n0;
        locals.var_t9_dn2 = assign73620_e111014_d_n2;
        locals.var_t9_dn4 = assign73620_e111014_d_n4;
        locals.var_t9_dn5 = assign73620_e111014_d_n5;
        locals.var_t9_dn6 = assign73620_e111014_d_n6;
        locals.var_t9_dn7 = assign73620_e111014_d_n7;
        locals.var_t9_dn8 = assign73620_e111014_d_n8;
        locals.var_t9_dn9 = assign73620_e111014_d_n9;
        locals.var_t9_dn10 = assign73620_e111014_d_n10;
        locals.var_t9_dn13 = assign73620_e111014_d_n13;
        locals.var_t9_rv = 0.0;

        let (assign73630_e111032, assign73630_e111032_d_n0, assign73630_e111032_d_n2, assign73630_e111032_d_n4, assign73630_e111032_d_n5, assign73630_e111032_d_n6, assign73630_e111032_d_n7, assign73630_e111032_d_n8, assign73630_e111032_d_n9, assign73630_e111032_d_n10, assign73630_e111032_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1704 != 0.0)) && (locals.var_guard1705 == 0.0)) {
        let assign73630_e111027: f64 = (locals.var_vdsi + p.p137);
        let assign73630_e111029: f64 = (assign73630_e111027 + locals.var_tmf2);
        let assign73630_e111030: f64 = (0.5 * assign73630_e111029);
        (assign73630_e111030, (0.5 * locals.var_tmf2_dn0), (0.5 * locals.var_tmf2_dn2), (0.5 * locals.var_tmf2_dn4), (0.5 * (locals.var_vdsi_dn5 + locals.var_tmf2_dn5)), (0.5 * locals.var_tmf2_dn6), (0.5 * (locals.var_vdsi_dn7 + locals.var_tmf2_dn7)), (0.5 * locals.var_tmf2_dn8), (0.5 * locals.var_tmf2_dn9), (0.5 * locals.var_tmf2_dn10), (0.5 * locals.var_tmf2_dn13),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign73630_e111032;
        locals.var_t2_dn0 = assign73630_e111032_d_n0;
        locals.var_t2_dn2 = assign73630_e111032_d_n2;
        locals.var_t2_dn4 = assign73630_e111032_d_n4;
        locals.var_t2_dn5 = assign73630_e111032_d_n5;
        locals.var_t2_dn6 = assign73630_e111032_d_n6;
        locals.var_t2_dn7 = assign73630_e111032_d_n7;
        locals.var_t2_dn8 = assign73630_e111032_d_n8;
        locals.var_t2_dn9 = assign73630_e111032_d_n9;
        locals.var_t2_dn10 = assign73630_e111032_d_n10;
        locals.var_t2_dn13 = assign73630_e111032_d_n13;
        locals.var_t2_rv = 0.0;

        let assign73640_e111035: f64 = if locals.var_t2 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1706 = assign73640_e111035;
        locals.var_guard1706_rv = 0.0;

        let (assign73650_e111049, assign73650_e111049_d_n0, assign73650_e111049_d_n2, assign73650_e111049_d_n4, assign73650_e111049_d_n5, assign73650_e111049_d_n6, assign73650_e111049_d_n7, assign73650_e111049_d_n8, assign73650_e111049_d_n9, assign73650_e111049_d_n10, assign73650_e111049_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1704 != 0.0)) && (locals.var_guard1705 == 0.0)) && (locals.var_guard1706 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign73650_e111049;
        locals.var_t2_dn0 = assign73650_e111049_d_n0;
        locals.var_t2_dn2 = assign73650_e111049_d_n2;
        locals.var_t2_dn4 = assign73650_e111049_d_n4;
        locals.var_t2_dn5 = assign73650_e111049_d_n5;
        locals.var_t2_dn6 = assign73650_e111049_d_n6;
        locals.var_t2_dn7 = assign73650_e111049_d_n7;
        locals.var_t2_dn8 = assign73650_e111049_d_n8;
        locals.var_t2_dn9 = assign73650_e111049_d_n9;
        locals.var_t2_dn10 = assign73650_e111049_d_n10;
        locals.var_t2_dn13 = assign73650_e111049_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign73660_e111063, assign73660_e111063_d_n0, assign73660_e111063_d_n2, assign73660_e111063_d_n4, assign73660_e111063_d_n5, assign73660_e111063_d_n6, assign73660_e111063_d_n7, assign73660_e111063_d_n8, assign73660_e111063_d_n9, assign73660_e111063_d_n10, assign73660_e111063_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1704 != 0.0)) && (locals.var_guard1705 == 0.0)) && (locals.var_guard1706 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign73660_e111063;
        locals.var_t9_dn0 = assign73660_e111063_d_n0;
        locals.var_t9_dn2 = assign73660_e111063_d_n2;
        locals.var_t9_dn4 = assign73660_e111063_d_n4;
        locals.var_t9_dn5 = assign73660_e111063_d_n5;
        locals.var_t9_dn6 = assign73660_e111063_d_n6;
        locals.var_t9_dn7 = assign73660_e111063_d_n7;
        locals.var_t9_dn8 = assign73660_e111063_d_n8;
        locals.var_t9_dn9 = assign73660_e111063_d_n9;
        locals.var_t9_dn10 = assign73660_e111063_d_n10;
        locals.var_t9_dn13 = assign73660_e111063_d_n13;
        locals.var_t9_rv = 0.0;

        let (assign73670_e111080, assign73670_e111080_d_n0, assign73670_e111080_d_n2, assign73670_e111080_d_n4, assign73670_e111080_d_n5, assign73670_e111080_d_n6, assign73670_e111080_d_n7, assign73670_e111080_d_n8, assign73670_e111080_d_n9, assign73670_e111080_d_n10, assign73670_e111080_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1704 != 0.0)) && (locals.var_guard1705 == 0.0)) {
        let assign73670_e111075: f64 = (locals.var_kjunc * locals.var_t2);
        let assign73670_e111076: f64 = (assign73670_e111075).sqrt();
        let assign73670_e111078: f64 = (assign73670_e111076 * p.p432);
        (assign73670_e111078, ((((locals.var_kjunc_dn0 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn0)) / (2.0 * assign73670_e111076)) * p.p432), ((((locals.var_kjunc_dn2 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn2)) / (2.0 * assign73670_e111076)) * p.p432), ((((locals.var_kjunc_dn4 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn4)) / (2.0 * assign73670_e111076)) * p.p432), ((((locals.var_kjunc_dn5 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn5)) / (2.0 * assign73670_e111076)) * p.p432), ((((locals.var_kjunc_dn6 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn6)) / (2.0 * assign73670_e111076)) * p.p432), ((((locals.var_kjunc_dn7 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn7)) / (2.0 * assign73670_e111076)) * p.p432), ((((locals.var_kjunc_dn8 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn8)) / (2.0 * assign73670_e111076)) * p.p432), ((((locals.var_kjunc_dn9 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn9)) / (2.0 * assign73670_e111076)) * p.p432), ((((locals.var_kjunc_dn10 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn10)) / (2.0 * assign73670_e111076)) * p.p432), ((((locals.var_kjunc_dn13 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn13)) / (2.0 * assign73670_e111076)) * p.p432),)
    } else {
        (locals.var_wjunc0, locals.var_wjunc0_dn0, locals.var_wjunc0_dn2, locals.var_wjunc0_dn4, locals.var_wjunc0_dn5, locals.var_wjunc0_dn6, locals.var_wjunc0_dn7, locals.var_wjunc0_dn8, locals.var_wjunc0_dn9, locals.var_wjunc0_dn10, locals.var_wjunc0_dn13,)
    }
};
        locals.var_wjunc0 = assign73670_e111080;
        locals.var_wjunc0_dn0 = assign73670_e111080_d_n0;
        locals.var_wjunc0_dn2 = assign73670_e111080_d_n2;
        locals.var_wjunc0_dn4 = assign73670_e111080_d_n4;
        locals.var_wjunc0_dn5 = assign73670_e111080_d_n5;
        locals.var_wjunc0_dn6 = assign73670_e111080_d_n6;
        locals.var_wjunc0_dn7 = assign73670_e111080_d_n7;
        locals.var_wjunc0_dn8 = assign73670_e111080_d_n8;
        locals.var_wjunc0_dn9 = assign73670_e111080_d_n9;
        locals.var_wjunc0_dn10 = assign73670_e111080_d_n10;
        locals.var_wjunc0_dn13 = assign73670_e111080_d_n13;
        locals.var_wjunc0_rv = 0.0;

        let (assign73680_e111094, assign73680_e111094_d_n0, assign73680_e111094_d_n2, assign73680_e111094_d_n4, assign73680_e111094_d_n5, assign73680_e111094_d_n6, assign73680_e111094_d_n7, assign73680_e111094_d_n8, assign73680_e111094_d_n9, assign73680_e111094_d_n10, assign73680_e111094_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1704 != 0.0)) && (locals.var_guard1705 == 0.0)) {
        let assign73680_e111092: f64 = (p.p334 - locals.var_wjunc0);
        (assign73680_e111092, (-locals.var_wjunc0_dn0), (-locals.var_wjunc0_dn2), (-locals.var_wjunc0_dn4), (-locals.var_wjunc0_dn5), (-locals.var_wjunc0_dn6), (-locals.var_wjunc0_dn7), (-locals.var_wjunc0_dn8), (-locals.var_wjunc0_dn9), (-locals.var_wjunc0_dn10), (-locals.var_wjunc0_dn13),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign73680_e111094;
        locals.var_t2_dn0 = assign73680_e111094_d_n0;
        locals.var_t2_dn2 = assign73680_e111094_d_n2;
        locals.var_t2_dn4 = assign73680_e111094_d_n4;
        locals.var_t2_dn5 = assign73680_e111094_d_n5;
        locals.var_t2_dn6 = assign73680_e111094_d_n6;
        locals.var_t2_dn7 = assign73680_e111094_d_n7;
        locals.var_t2_dn8 = assign73680_e111094_d_n8;
        locals.var_t2_dn9 = assign73680_e111094_d_n9;
        locals.var_t2_dn10 = assign73680_e111094_d_n10;
        locals.var_t2_dn13 = assign73680_e111094_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign73690_e111116, assign73690_e111116_d_n0, assign73690_e111116_d_n2, assign73690_e111116_d_n4, assign73690_e111116_d_n5, assign73690_e111116_d_n6, assign73690_e111116_d_n7, assign73690_e111116_d_n8, assign73690_e111116_d_n9, assign73690_e111116_d_n10, assign73690_e111116_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1704 != 0.0)) {
        let assign73690_e111103: f64 = (locals.var_t2 * locals.var_t2);
        let assign73690_e111107: f64 = (p.p334 * 0.01);
        let assign73690_e111108: f64 = (4.0 * assign73690_e111107);
        let assign73690_e111111: f64 = (p.p334 * 0.01);
        let assign73690_e111112: f64 = (assign73690_e111108 * assign73690_e111111);
        let assign73690_e111113: f64 = (assign73690_e111103 + assign73690_e111112);
        let assign73690_e111114: f64 = (assign73690_e111113).sqrt();
        (assign73690_e111114, (((locals.var_t2_dn0 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn0)) / (2.0 * assign73690_e111114)), (((locals.var_t2_dn2 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn2)) / (2.0 * assign73690_e111114)), (((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4)) / (2.0 * assign73690_e111114)), (((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5)) / (2.0 * assign73690_e111114)), (((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6)) / (2.0 * assign73690_e111114)), (((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7)) / (2.0 * assign73690_e111114)), (((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8)) / (2.0 * assign73690_e111114)), (((locals.var_t2_dn9 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn9)) / (2.0 * assign73690_e111114)), (((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10)) / (2.0 * assign73690_e111114)), (((locals.var_t2_dn13 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn13)) / (2.0 * assign73690_e111114)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign73690_e111116;
        locals.var_tmf2_dn0 = assign73690_e111116_d_n0;
        locals.var_tmf2_dn2 = assign73690_e111116_d_n2;
        locals.var_tmf2_dn4 = assign73690_e111116_d_n4;
        locals.var_tmf2_dn5 = assign73690_e111116_d_n5;
        locals.var_tmf2_dn6 = assign73690_e111116_d_n6;
        locals.var_tmf2_dn7 = assign73690_e111116_d_n7;
        locals.var_tmf2_dn8 = assign73690_e111116_d_n8;
        locals.var_tmf2_dn9 = assign73690_e111116_d_n9;
        locals.var_tmf2_dn10 = assign73690_e111116_d_n10;
        locals.var_tmf2_dn13 = assign73690_e111116_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign73700_e111131, assign73700_e111131_d_n0, assign73700_e111131_d_n2, assign73700_e111131_d_n4, assign73700_e111131_d_n5, assign73700_e111131_d_n6, assign73700_e111131_d_n7, assign73700_e111131_d_n8, assign73700_e111131_d_n9, assign73700_e111131_d_n10, assign73700_e111131_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1704 != 0.0)) {
        let assign73700_e111127: f64 = (locals.var_t2 / locals.var_tmf2);
        let assign73700_e111128: f64 = (1.0 + assign73700_e111127);
        let assign73700_e111129: f64 = (0.5 * assign73700_e111128);
        (assign73700_e111129, (0.5 * (((locals.var_t2_dn0 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn2 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn4 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn5 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn6 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn7 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn8 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn9 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn10 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn13 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign73700_e111131;
        locals.var_t9_dn0 = assign73700_e111131_d_n0;
        locals.var_t9_dn2 = assign73700_e111131_d_n2;
        locals.var_t9_dn4 = assign73700_e111131_d_n4;
        locals.var_t9_dn5 = assign73700_e111131_d_n5;
        locals.var_t9_dn6 = assign73700_e111131_d_n6;
        locals.var_t9_dn7 = assign73700_e111131_d_n7;
        locals.var_t9_dn8 = assign73700_e111131_d_n8;
        locals.var_t9_dn9 = assign73700_e111131_d_n9;
        locals.var_t9_dn10 = assign73700_e111131_d_n10;
        locals.var_t9_dn13 = assign73700_e111131_d_n13;
        locals.var_t9_rv = 0.0;

        let (assign73710_e111144, assign73710_e111144_d_n0, assign73710_e111144_d_n2, assign73710_e111144_d_n4, assign73710_e111144_d_n5, assign73710_e111144_d_n6, assign73710_e111144_d_n7, assign73710_e111144_d_n8, assign73710_e111144_d_n9, assign73710_e111144_d_n10, assign73710_e111144_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1704 != 0.0)) {
        let assign73710_e111141: f64 = (locals.var_t2 + locals.var_tmf2);
        let assign73710_e111142: f64 = (0.5 * assign73710_e111141);
        (assign73710_e111142, (0.5 * (locals.var_t2_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_t2_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_t2_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_t2_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_t2_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_t2_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_t2_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_t2_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_t2_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_t2_dn13 + locals.var_tmf2_dn13)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign73710_e111144;
        locals.var_t2_dn0 = assign73710_e111144_d_n0;
        locals.var_t2_dn2 = assign73710_e111144_d_n2;
        locals.var_t2_dn4 = assign73710_e111144_d_n4;
        locals.var_t2_dn5 = assign73710_e111144_d_n5;
        locals.var_t2_dn6 = assign73710_e111144_d_n6;
        locals.var_t2_dn7 = assign73710_e111144_d_n7;
        locals.var_t2_dn8 = assign73710_e111144_d_n8;
        locals.var_t2_dn9 = assign73710_e111144_d_n9;
        locals.var_t2_dn10 = assign73710_e111144_d_n10;
        locals.var_t2_dn13 = assign73710_e111144_d_n13;
        locals.var_t2_rv = 0.0;

        let assign73720_e111147: f64 = if locals.var_t2 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1707 = assign73720_e111147;
        locals.var_guard1707_rv = 0.0;

        let (assign73730_e111158, assign73730_e111158_d_n0, assign73730_e111158_d_n2, assign73730_e111158_d_n4, assign73730_e111158_d_n5, assign73730_e111158_d_n6, assign73730_e111158_d_n7, assign73730_e111158_d_n8, assign73730_e111158_d_n9, assign73730_e111158_d_n10, assign73730_e111158_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1704 != 0.0)) && (locals.var_guard1707 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign73730_e111158;
        locals.var_t2_dn0 = assign73730_e111158_d_n0;
        locals.var_t2_dn2 = assign73730_e111158_d_n2;
        locals.var_t2_dn4 = assign73730_e111158_d_n4;
        locals.var_t2_dn5 = assign73730_e111158_d_n5;
        locals.var_t2_dn6 = assign73730_e111158_d_n6;
        locals.var_t2_dn7 = assign73730_e111158_d_n7;
        locals.var_t2_dn8 = assign73730_e111158_d_n8;
        locals.var_t2_dn9 = assign73730_e111158_d_n9;
        locals.var_t2_dn10 = assign73730_e111158_d_n10;
        locals.var_t2_dn13 = assign73730_e111158_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign73740_e111169, assign73740_e111169_d_n0, assign73740_e111169_d_n2, assign73740_e111169_d_n4, assign73740_e111169_d_n5, assign73740_e111169_d_n6, assign73740_e111169_d_n7, assign73740_e111169_d_n8, assign73740_e111169_d_n9, assign73740_e111169_d_n10, assign73740_e111169_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1704 != 0.0)) && (locals.var_guard1707 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign73740_e111169;
        locals.var_t9_dn0 = assign73740_e111169_d_n0;
        locals.var_t9_dn2 = assign73740_e111169_d_n2;
        locals.var_t9_dn4 = assign73740_e111169_d_n4;
        locals.var_t9_dn5 = assign73740_e111169_d_n5;
        locals.var_t9_dn6 = assign73740_e111169_d_n6;
        locals.var_t9_dn7 = assign73740_e111169_d_n7;
        locals.var_t9_dn8 = assign73740_e111169_d_n8;
        locals.var_t9_dn9 = assign73740_e111169_d_n9;
        locals.var_t9_dn10 = assign73740_e111169_d_n10;
        locals.var_t9_dn13 = assign73740_e111169_d_n13;
        locals.var_t9_rv = 0.0;

        let (assign73750_e111178, assign73750_e111178_d_n0, assign73750_e111178_d_n2, assign73750_e111178_d_n4, assign73750_e111178_d_n5, assign73750_e111178_d_n6, assign73750_e111178_d_n7, assign73750_e111178_d_n8, assign73750_e111178_d_n9, assign73750_e111178_d_n10, assign73750_e111178_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1704 != 0.0)) {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    } else {
        (locals.var_ddriftldc, locals.var_ddriftldc_dn0, locals.var_ddriftldc_dn2, locals.var_ddriftldc_dn4, locals.var_ddriftldc_dn5, locals.var_ddriftldc_dn6, locals.var_ddriftldc_dn7, locals.var_ddriftldc_dn8, locals.var_ddriftldc_dn9, locals.var_ddriftldc_dn10, locals.var_ddriftldc_dn13,)
    }
};
        locals.var_ddriftldc = assign73750_e111178;
        locals.var_ddriftldc_dn0 = assign73750_e111178_d_n0;
        locals.var_ddriftldc_dn2 = assign73750_e111178_d_n2;
        locals.var_ddriftldc_dn4 = assign73750_e111178_d_n4;
        locals.var_ddriftldc_dn5 = assign73750_e111178_d_n5;
        locals.var_ddriftldc_dn6 = assign73750_e111178_d_n6;
        locals.var_ddriftldc_dn7 = assign73750_e111178_d_n7;
        locals.var_ddriftldc_dn8 = assign73750_e111178_d_n8;
        locals.var_ddriftldc_dn9 = assign73750_e111178_d_n9;
        locals.var_ddriftldc_dn10 = assign73750_e111178_d_n10;
        locals.var_ddriftldc_dn13 = assign73750_e111178_d_n13;
        locals.var_ddriftldc_rv = 0.0;

        let (assign73760_e111195, assign73760_e111195_d_n0, assign73760_e111195_d_n2, assign73760_e111195_d_n4, assign73760_e111195_d_n5, assign73760_e111195_d_n6, assign73760_e111195_d_n7, assign73760_e111195_d_n8, assign73760_e111195_d_n9, assign73760_e111195_d_n10, assign73760_e111195_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1704 != 0.0)) {
        let assign73760_e111187: f64 = (locals.var_q_nsubld * locals.var_ddriftldc);
        let assign73760_e111189: f64 = (assign73760_e111187 * locals.var_ddriftldc);
        let assign73760_e111191: f64 = (assign73760_e111189 / 2.0);
        let assign73760_e111193: f64 = (assign73760_e111191 / 1.034943e-10);
        (assign73760_e111193, (((((locals.var_q_nsubld * locals.var_ddriftldc_dn0) * locals.var_ddriftldc) + (assign73760_e111187 * locals.var_ddriftldc_dn0)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld * locals.var_ddriftldc_dn2) * locals.var_ddriftldc) + (assign73760_e111187 * locals.var_ddriftldc_dn2)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld * locals.var_ddriftldc_dn4) * locals.var_ddriftldc) + (assign73760_e111187 * locals.var_ddriftldc_dn4)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld * locals.var_ddriftldc_dn5) * locals.var_ddriftldc) + (assign73760_e111187 * locals.var_ddriftldc_dn5)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld * locals.var_ddriftldc_dn6) * locals.var_ddriftldc) + (assign73760_e111187 * locals.var_ddriftldc_dn6)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld * locals.var_ddriftldc_dn7) * locals.var_ddriftldc) + (assign73760_e111187 * locals.var_ddriftldc_dn7)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld * locals.var_ddriftldc_dn8) * locals.var_ddriftldc) + (assign73760_e111187 * locals.var_ddriftldc_dn8)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld * locals.var_ddriftldc_dn9) * locals.var_ddriftldc) + (assign73760_e111187 * locals.var_ddriftldc_dn9)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld * locals.var_ddriftldc_dn10) * locals.var_ddriftldc) + (assign73760_e111187 * locals.var_ddriftldc_dn10)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld * locals.var_ddriftldc_dn13) * locals.var_ddriftldc) + (assign73760_e111187 * locals.var_ddriftldc_dn13)) / 2.0) / 1.034943e-10),)
    } else {
        (locals.var_dphi_sb, locals.var_dphi_sb_dn0, locals.var_dphi_sb_dn2, locals.var_dphi_sb_dn4, locals.var_dphi_sb_dn5, locals.var_dphi_sb_dn6, locals.var_dphi_sb_dn7, locals.var_dphi_sb_dn8, locals.var_dphi_sb_dn9, locals.var_dphi_sb_dn10, locals.var_dphi_sb_dn13,)
    }
};
        locals.var_dphi_sb = assign73760_e111195;
        locals.var_dphi_sb_dn0 = assign73760_e111195_d_n0;
        locals.var_dphi_sb_dn2 = assign73760_e111195_d_n2;
        locals.var_dphi_sb_dn4 = assign73760_e111195_d_n4;
        locals.var_dphi_sb_dn5 = assign73760_e111195_d_n5;
        locals.var_dphi_sb_dn6 = assign73760_e111195_d_n6;
        locals.var_dphi_sb_dn7 = assign73760_e111195_d_n7;
        locals.var_dphi_sb_dn8 = assign73760_e111195_d_n8;
        locals.var_dphi_sb_dn9 = assign73760_e111195_d_n9;
        locals.var_dphi_sb_dn10 = assign73760_e111195_d_n10;
        locals.var_dphi_sb_dn13 = assign73760_e111195_d_n13;
        locals.var_dphi_sb_rv = 0.0;

        let (assign73770_e111209, assign73770_e111209_d_n0, assign73770_e111209_d_n2, assign73770_e111209_d_n4, assign73770_e111209_d_n5, assign73770_e111209_d_n6, assign73770_e111209_d_n7, assign73770_e111209_d_n8, assign73770_e111209_d_n9, assign73770_e111209_d_n10, assign73770_e111209_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1704 != 0.0)) {
        let assign73770_e111204: f64 = (2.0 * locals.var_beta);
        let assign73770_e111206: f64 = (assign73770_e111204 * locals.var_dphi_sb);
        let assign73770_e111207: f64 = (assign73770_e111206).sqrt();
        (assign73770_e111207, ((((2.0 * locals.var_beta_dn0) * locals.var_dphi_sb) + (assign73770_e111204 * locals.var_dphi_sb_dn0)) / (2.0 * assign73770_e111207)), ((((2.0 * locals.var_beta_dn2) * locals.var_dphi_sb) + (assign73770_e111204 * locals.var_dphi_sb_dn2)) / (2.0 * assign73770_e111207)), ((((2.0 * locals.var_beta_dn4) * locals.var_dphi_sb) + (assign73770_e111204 * locals.var_dphi_sb_dn4)) / (2.0 * assign73770_e111207)), ((((2.0 * locals.var_beta_dn5) * locals.var_dphi_sb) + (assign73770_e111204 * locals.var_dphi_sb_dn5)) / (2.0 * assign73770_e111207)), ((((2.0 * locals.var_beta_dn6) * locals.var_dphi_sb) + (assign73770_e111204 * locals.var_dphi_sb_dn6)) / (2.0 * assign73770_e111207)), ((((2.0 * locals.var_beta_dn7) * locals.var_dphi_sb) + (assign73770_e111204 * locals.var_dphi_sb_dn7)) / (2.0 * assign73770_e111207)), ((((2.0 * locals.var_beta_dn8) * locals.var_dphi_sb) + (assign73770_e111204 * locals.var_dphi_sb_dn8)) / (2.0 * assign73770_e111207)), ((((2.0 * locals.var_beta_dn9) * locals.var_dphi_sb) + (assign73770_e111204 * locals.var_dphi_sb_dn9)) / (2.0 * assign73770_e111207)), ((((2.0 * locals.var_beta_dn10) * locals.var_dphi_sb) + (assign73770_e111204 * locals.var_dphi_sb_dn10)) / (2.0 * assign73770_e111207)), ((((2.0 * locals.var_beta_dn13) * locals.var_dphi_sb) + (assign73770_e111204 * locals.var_dphi_sb_dn13)) / (2.0 * assign73770_e111207)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign73770_e111209;
        locals.var_t0_dn0 = assign73770_e111209_d_n0;
        locals.var_t0_dn2 = assign73770_e111209_d_n2;
        locals.var_t0_dn4 = assign73770_e111209_d_n4;
        locals.var_t0_dn5 = assign73770_e111209_d_n5;
        locals.var_t0_dn6 = assign73770_e111209_d_n6;
        locals.var_t0_dn7 = assign73770_e111209_d_n7;
        locals.var_t0_dn8 = assign73770_e111209_d_n8;
        locals.var_t0_dn9 = assign73770_e111209_d_n9;
        locals.var_t0_dn10 = assign73770_e111209_d_n10;
        locals.var_t0_dn13 = assign73770_e111209_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign73780_e111225, assign73780_e111225_d_n0, assign73780_e111225_d_n2, assign73780_e111225_d_n4, assign73780_e111225_d_n5, assign73780_e111225_d_n6, assign73780_e111225_d_n7, assign73780_e111225_d_n8, assign73780_e111225_d_n9, assign73780_e111225_d_n10, assign73780_e111225_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1704 != 0.0)) {
        let assign73780_e111217: f64 = { let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign73780_e111219: f64 = (-locals.var_t0);
        let assign73780_e111220: f64 = { let limited_exp_arg = assign73780_e111219; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign73780_e111221: f64 = (assign73780_e111217 + assign73780_e111220);
        let assign73780_e111223: f64 = (assign73780_e111221 / 2.0);
        (assign73780_e111223, ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn0) + ({ let limited_exp_arg = assign73780_e111219; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn0))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn2) + ({ let limited_exp_arg = assign73780_e111219; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn2))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn4) + ({ let limited_exp_arg = assign73780_e111219; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn4))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn5) + ({ let limited_exp_arg = assign73780_e111219; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn5))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn6) + ({ let limited_exp_arg = assign73780_e111219; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn6))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn7) + ({ let limited_exp_arg = assign73780_e111219; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn7))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn8) + ({ let limited_exp_arg = assign73780_e111219; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn8))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn9) + ({ let limited_exp_arg = assign73780_e111219; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn9))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn10) + ({ let limited_exp_arg = assign73780_e111219; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn10))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn13) + ({ let limited_exp_arg = assign73780_e111219; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn13))) / 2.0),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign73780_e111225;
        locals.var_t1_dn0 = assign73780_e111225_d_n0;
        locals.var_t1_dn2 = assign73780_e111225_d_n2;
        locals.var_t1_dn4 = assign73780_e111225_d_n4;
        locals.var_t1_dn5 = assign73780_e111225_d_n5;
        locals.var_t1_dn6 = assign73780_e111225_d_n6;
        locals.var_t1_dn7 = assign73780_e111225_d_n7;
        locals.var_t1_dn8 = assign73780_e111225_d_n8;
        locals.var_t1_dn9 = assign73780_e111225_d_n9;
        locals.var_t1_dn10 = assign73780_e111225_d_n10;
        locals.var_t1_dn13 = assign73780_e111225_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign73790_e111237, assign73790_e111237_d_n0, assign73790_e111237_d_n2, assign73790_e111237_d_n4, assign73790_e111237_d_n5, assign73790_e111237_d_n6, assign73790_e111237_d_n7, assign73790_e111237_d_n8, assign73790_e111237_d_n9, assign73790_e111237_d_n10, assign73790_e111237_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1704 != 0.0)) {
        let assign73790_e111233: f64 = (locals.var_t1).ln();
        let assign73790_e111235: f64 = (assign73790_e111233 / locals.var_dphi_sb);
        (assign73790_e111235, ((((locals.var_t1_dn0 / locals.var_t1) * locals.var_dphi_sb) - (assign73790_e111233 * locals.var_dphi_sb_dn0)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn2 / locals.var_t1) * locals.var_dphi_sb) - (assign73790_e111233 * locals.var_dphi_sb_dn2)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn4 / locals.var_t1) * locals.var_dphi_sb) - (assign73790_e111233 * locals.var_dphi_sb_dn4)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn5 / locals.var_t1) * locals.var_dphi_sb) - (assign73790_e111233 * locals.var_dphi_sb_dn5)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn6 / locals.var_t1) * locals.var_dphi_sb) - (assign73790_e111233 * locals.var_dphi_sb_dn6)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn7 / locals.var_t1) * locals.var_dphi_sb) - (assign73790_e111233 * locals.var_dphi_sb_dn7)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn8 / locals.var_t1) * locals.var_dphi_sb) - (assign73790_e111233 * locals.var_dphi_sb_dn8)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn9 / locals.var_t1) * locals.var_dphi_sb) - (assign73790_e111233 * locals.var_dphi_sb_dn9)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn10 / locals.var_t1) * locals.var_dphi_sb) - (assign73790_e111233 * locals.var_dphi_sb_dn10)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn13 / locals.var_t1) * locals.var_dphi_sb) - (assign73790_e111233 * locals.var_dphi_sb_dn13)) / (locals.var_dphi_sb * locals.var_dphi_sb)),)
    } else {
        (locals.var_c_sb, locals.var_c_sb_dn0, locals.var_c_sb_dn2, locals.var_c_sb_dn4, locals.var_c_sb_dn5, locals.var_c_sb_dn6, locals.var_c_sb_dn7, locals.var_c_sb_dn8, locals.var_c_sb_dn9, locals.var_c_sb_dn10, locals.var_c_sb_dn13,)
    }
};
        locals.var_c_sb = assign73790_e111237;
        locals.var_c_sb_dn0 = assign73790_e111237_d_n0;
        locals.var_c_sb_dn2 = assign73790_e111237_d_n2;
        locals.var_c_sb_dn4 = assign73790_e111237_d_n4;
        locals.var_c_sb_dn5 = assign73790_e111237_d_n5;
        locals.var_c_sb_dn6 = assign73790_e111237_d_n6;
        locals.var_c_sb_dn7 = assign73790_e111237_d_n7;
        locals.var_c_sb_dn8 = assign73790_e111237_d_n8;
        locals.var_c_sb_dn9 = assign73790_e111237_d_n9;
        locals.var_c_sb_dn10 = assign73790_e111237_d_n10;
        locals.var_c_sb_dn13 = assign73790_e111237_d_n13;
        locals.var_c_sb_rv = 0.0;

        let (assign73800_e111246,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1704 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_lp_s0,)
    }
};
        locals.var_lp_s0 = assign73800_e111246;
        locals.var_lp_s0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_266(
        locals: &mut StampLocals,
    ) {
        let mut assign73810_loop_guard: usize = 0;
        while {
            let assign73810_cond_e111256: f64 = (locals.var_lp_s0_max + 1.0);
            let assign73810_cond_e111258: f64 = if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1704 != 0.0)) && (locals.var_lp_s0 <= assign73810_cond_e111256)) { 1.0 } else { 0.0 };
            assign73810_cond_e111258 != 0.0
        } {
            assign73810_loop_guard += 1;
            assert!(assign73810_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign73810_body3_e111294, assign73810_body3_e111294_d_n0, assign73810_body3_e111294_d_n2, assign73810_body3_e111294_d_n4, assign73810_body3_e111294_d_n5, assign73810_body3_e111294_d_n6, assign73810_body3_e111294_d_n7, assign73810_body3_e111294_d_n8, assign73810_body3_e111294_d_n9, assign73810_body3_e111294_d_n10, assign73810_body3_e111294_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1704 != 0.0)) {
        let assign73810_body3_e111292: f64 = (locals.var_ps0ld + locals.var_vxbgmtcl);
        (assign73810_body3_e111292, (locals.var_ps0ld_dn0 + locals.var_vxbgmtcl_dn0), (locals.var_ps0ld_dn2 + locals.var_vxbgmtcl_dn2), (locals.var_ps0ld_dn4 + locals.var_vxbgmtcl_dn4), (locals.var_ps0ld_dn5 + locals.var_vxbgmtcl_dn5), (locals.var_ps0ld_dn6 + locals.var_vxbgmtcl_dn6), (locals.var_ps0ld_dn7 + locals.var_vxbgmtcl_dn7), (locals.var_ps0ld_dn8 + locals.var_vxbgmtcl_dn8), (locals.var_ps0ld_dn9 + locals.var_vxbgmtcl_dn9), (locals.var_ps0ld_dn10 + locals.var_vxbgmtcl_dn10), (locals.var_ps0ld_dn13 + locals.var_vxbgmtcl_dn13),)
    } else {
        (locals.var_ps0ld_vxb, locals.var_ps0ld_vxb_dn0, locals.var_ps0ld_vxb_dn2, locals.var_ps0ld_vxb_dn4, locals.var_ps0ld_vxb_dn5, locals.var_ps0ld_vxb_dn6, locals.var_ps0ld_vxb_dn7, locals.var_ps0ld_vxb_dn8, locals.var_ps0ld_vxb_dn9, locals.var_ps0ld_vxb_dn10, locals.var_ps0ld_vxb_dn13,)
    }
};
            locals.var_ps0ld_vxb = assign73810_body3_e111294;
            locals.var_ps0ld_vxb_dn0 = assign73810_body3_e111294_d_n0;
            locals.var_ps0ld_vxb_dn2 = assign73810_body3_e111294_d_n2;
            locals.var_ps0ld_vxb_dn4 = assign73810_body3_e111294_d_n4;
            locals.var_ps0ld_vxb_dn5 = assign73810_body3_e111294_d_n5;
            locals.var_ps0ld_vxb_dn6 = assign73810_body3_e111294_d_n6;
            locals.var_ps0ld_vxb_dn7 = assign73810_body3_e111294_d_n7;
            locals.var_ps0ld_vxb_dn8 = assign73810_body3_e111294_d_n8;
            locals.var_ps0ld_vxb_dn9 = assign73810_body3_e111294_d_n9;
            locals.var_ps0ld_vxb_dn10 = assign73810_body3_e111294_d_n10;
            locals.var_ps0ld_vxb_dn13 = assign73810_body3_e111294_d_n13;
            locals.var_ps0ld_vxb_rv = 0.0;
            let (assign73810_body4_e111305, assign73810_body4_e111305_d_n0, assign73810_body4_e111305_d_n2, assign73810_body4_e111305_d_n4, assign73810_body4_e111305_d_n5, assign73810_body4_e111305_d_n6, assign73810_body4_e111305_d_n7, assign73810_body4_e111305_d_n8, assign73810_body4_e111305_d_n9, assign73810_body4_e111305_d_n10, assign73810_body4_e111305_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1704 != 0.0)) {
        let assign73810_body4_e111303: f64 = (locals.var_beta * locals.var_ps0ld_vxb);
        (assign73810_body4_e111303, ((locals.var_beta_dn0 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn0)), ((locals.var_beta_dn2 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn2)), ((locals.var_beta_dn4 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn4)), ((locals.var_beta_dn5 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn5)), ((locals.var_beta_dn6 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn6)), ((locals.var_beta_dn7 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn7)), ((locals.var_beta_dn8 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn8)), ((locals.var_beta_dn9 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn9)), ((locals.var_beta_dn10 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn10)), ((locals.var_beta_dn13 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn13)),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn13,)
    }
};
            locals.var_chi = assign73810_body4_e111305;
            locals.var_chi_dn0 = assign73810_body4_e111305_d_n0;
            locals.var_chi_dn2 = assign73810_body4_e111305_d_n2;
            locals.var_chi_dn4 = assign73810_body4_e111305_d_n4;
            locals.var_chi_dn5 = assign73810_body4_e111305_d_n5;
            locals.var_chi_dn6 = assign73810_body4_e111305_d_n6;
            locals.var_chi_dn7 = assign73810_body4_e111305_d_n7;
            locals.var_chi_dn8 = assign73810_body4_e111305_d_n8;
            locals.var_chi_dn9 = assign73810_body4_e111305_d_n9;
            locals.var_chi_dn10 = assign73810_body4_e111305_d_n10;
            locals.var_chi_dn13 = assign73810_body4_e111305_d_n13;
            locals.var_chi_rv = 0.0;
            let (assign73810_body5_e111318, assign73810_body5_e111318_d_n0, assign73810_body5_e111318_d_n2, assign73810_body5_e111318_d_n4, assign73810_body5_e111318_d_n5, assign73810_body5_e111318_d_n6, assign73810_body5_e111318_d_n7, assign73810_body5_e111318_d_n8, assign73810_body5_e111318_d_n9, assign73810_body5_e111318_d_n10, assign73810_body5_e111318_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1704 != 0.0)) {
        let assign73810_body5_e111315: f64 = (locals.var_ps0ld_vxb - locals.var_dphi_sb);
        let assign73810_body5_e111316: f64 = (locals.var_c_sb * assign73810_body5_e111315);
        (assign73810_body5_e111316, ((locals.var_c_sb_dn0 * assign73810_body5_e111315) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn0 - locals.var_dphi_sb_dn0))), ((locals.var_c_sb_dn2 * assign73810_body5_e111315) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn2 - locals.var_dphi_sb_dn2))), ((locals.var_c_sb_dn4 * assign73810_body5_e111315) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn4 - locals.var_dphi_sb_dn4))), ((locals.var_c_sb_dn5 * assign73810_body5_e111315) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn5 - locals.var_dphi_sb_dn5))), ((locals.var_c_sb_dn6 * assign73810_body5_e111315) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn6 - locals.var_dphi_sb_dn6))), ((locals.var_c_sb_dn7 * assign73810_body5_e111315) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn7 - locals.var_dphi_sb_dn7))), ((locals.var_c_sb_dn8 * assign73810_body5_e111315) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn8 - locals.var_dphi_sb_dn8))), ((locals.var_c_sb_dn9 * assign73810_body5_e111315) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn9 - locals.var_dphi_sb_dn9))), ((locals.var_c_sb_dn10 * assign73810_body5_e111315) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn10 - locals.var_dphi_sb_dn10))), ((locals.var_c_sb_dn13 * assign73810_body5_e111315) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn13 - locals.var_dphi_sb_dn13))),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn4, locals.var_ty_dn5, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn8, locals.var_ty_dn9, locals.var_ty_dn10, locals.var_ty_dn13,)
    }
};
            locals.var_ty = assign73810_body5_e111318;
            locals.var_ty_dn0 = assign73810_body5_e111318_d_n0;
            locals.var_ty_dn2 = assign73810_body5_e111318_d_n2;
            locals.var_ty_dn4 = assign73810_body5_e111318_d_n4;
            locals.var_ty_dn5 = assign73810_body5_e111318_d_n5;
            locals.var_ty_dn6 = assign73810_body5_e111318_d_n6;
            locals.var_ty_dn7 = assign73810_body5_e111318_d_n7;
            locals.var_ty_dn8 = assign73810_body5_e111318_d_n8;
            locals.var_ty_dn9 = assign73810_body5_e111318_d_n9;
            locals.var_ty_dn10 = assign73810_body5_e111318_d_n10;
            locals.var_ty_dn13 = assign73810_body5_e111318_d_n13;
            locals.var_ty_rv = 0.0;
            let assign73810_body6_e111321: f64 = if locals.var_ty < 60.0 { 1.0 } else { 0.0 };
            locals.var_guard1709 = assign73810_body6_e111321;
            locals.var_guard1709_rv = 0.0;
            let (assign73810_body7_e111333, assign73810_body7_e111333_d_n0, assign73810_body7_e111333_d_n2, assign73810_body7_e111333_d_n4, assign73810_body7_e111333_d_n5, assign73810_body7_e111333_d_n6, assign73810_body7_e111333_d_n7, assign73810_body7_e111333_d_n8, assign73810_body7_e111333_d_n9, assign73810_body7_e111333_d_n10, assign73810_body7_e111333_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1704 != 0.0)) && (locals.var_guard1709 != 0.0)) {
        let assign73810_body7_e111331: f64 = (locals.var_ty).exp();
        (assign73810_body7_e111331, (assign73810_body7_e111331 * locals.var_ty_dn0), (assign73810_body7_e111331 * locals.var_ty_dn2), (assign73810_body7_e111331 * locals.var_ty_dn4), (assign73810_body7_e111331 * locals.var_ty_dn5), (assign73810_body7_e111331 * locals.var_ty_dn6), (assign73810_body7_e111331 * locals.var_ty_dn7), (assign73810_body7_e111331 * locals.var_ty_dn8), (assign73810_body7_e111331 * locals.var_ty_dn9), (assign73810_body7_e111331 * locals.var_ty_dn10), (assign73810_body7_e111331 * locals.var_ty_dn13),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
            locals.var_t1 = assign73810_body7_e111333;
            locals.var_t1_dn0 = assign73810_body7_e111333_d_n0;
            locals.var_t1_dn2 = assign73810_body7_e111333_d_n2;
            locals.var_t1_dn4 = assign73810_body7_e111333_d_n4;
            locals.var_t1_dn5 = assign73810_body7_e111333_d_n5;
            locals.var_t1_dn6 = assign73810_body7_e111333_d_n6;
            locals.var_t1_dn7 = assign73810_body7_e111333_d_n7;
            locals.var_t1_dn8 = assign73810_body7_e111333_d_n8;
            locals.var_t1_dn9 = assign73810_body7_e111333_d_n9;
            locals.var_t1_dn10 = assign73810_body7_e111333_d_n10;
            locals.var_t1_dn13 = assign73810_body7_e111333_d_n13;
            locals.var_t1_rv = 0.0;
            let (assign73810_body8_e111348, assign73810_body8_e111348_d_n0, assign73810_body8_e111348_d_n2, assign73810_body8_e111348_d_n4, assign73810_body8_e111348_d_n5, assign73810_body8_e111348_d_n6, assign73810_body8_e111348_d_n7, assign73810_body8_e111348_d_n8, assign73810_body8_e111348_d_n9, assign73810_body8_e111348_d_n10, assign73810_body8_e111348_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1704 != 0.0)) && (locals.var_guard1709 != 0.0)) {
        let assign73810_body8_e111343: f64 = (-locals.var_c_sb);
        let assign73810_body8_e111345: f64 = (assign73810_body8_e111343 * locals.var_dphi_sb);
        let assign73810_body8_e111346: f64 = (assign73810_body8_e111345).exp();
        (assign73810_body8_e111346, (assign73810_body8_e111346 * (((-locals.var_c_sb_dn0) * locals.var_dphi_sb) + (assign73810_body8_e111343 * locals.var_dphi_sb_dn0))), (assign73810_body8_e111346 * (((-locals.var_c_sb_dn2) * locals.var_dphi_sb) + (assign73810_body8_e111343 * locals.var_dphi_sb_dn2))), (assign73810_body8_e111346 * (((-locals.var_c_sb_dn4) * locals.var_dphi_sb) + (assign73810_body8_e111343 * locals.var_dphi_sb_dn4))), (assign73810_body8_e111346 * (((-locals.var_c_sb_dn5) * locals.var_dphi_sb) + (assign73810_body8_e111343 * locals.var_dphi_sb_dn5))), (assign73810_body8_e111346 * (((-locals.var_c_sb_dn6) * locals.var_dphi_sb) + (assign73810_body8_e111343 * locals.var_dphi_sb_dn6))), (assign73810_body8_e111346 * (((-locals.var_c_sb_dn7) * locals.var_dphi_sb) + (assign73810_body8_e111343 * locals.var_dphi_sb_dn7))), (assign73810_body8_e111346 * (((-locals.var_c_sb_dn8) * locals.var_dphi_sb) + (assign73810_body8_e111343 * locals.var_dphi_sb_dn8))), (assign73810_body8_e111346 * (((-locals.var_c_sb_dn9) * locals.var_dphi_sb) + (assign73810_body8_e111343 * locals.var_dphi_sb_dn9))), (assign73810_body8_e111346 * (((-locals.var_c_sb_dn10) * locals.var_dphi_sb) + (assign73810_body8_e111343 * locals.var_dphi_sb_dn10))), (assign73810_body8_e111346 * (((-locals.var_c_sb_dn13) * locals.var_dphi_sb) + (assign73810_body8_e111343 * locals.var_dphi_sb_dn13))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
            locals.var_t0 = assign73810_body8_e111348;
            locals.var_t0_dn0 = assign73810_body8_e111348_d_n0;
            locals.var_t0_dn2 = assign73810_body8_e111348_d_n2;
            locals.var_t0_dn4 = assign73810_body8_e111348_d_n4;
            locals.var_t0_dn5 = assign73810_body8_e111348_d_n5;
            locals.var_t0_dn6 = assign73810_body8_e111348_d_n6;
            locals.var_t0_dn7 = assign73810_body8_e111348_d_n7;
            locals.var_t0_dn8 = assign73810_body8_e111348_d_n8;
            locals.var_t0_dn9 = assign73810_body8_e111348_d_n9;
            locals.var_t0_dn10 = assign73810_body8_e111348_d_n10;
            locals.var_t0_dn13 = assign73810_body8_e111348_d_n13;
            locals.var_t0_rv = 0.0;
            let (assign73810_body9_e111361, assign73810_body9_e111361_d_n0, assign73810_body9_e111361_d_n2, assign73810_body9_e111361_d_n4, assign73810_body9_e111361_d_n5, assign73810_body9_e111361_d_n6, assign73810_body9_e111361_d_n7, assign73810_body9_e111361_d_n8, assign73810_body9_e111361_d_n9, assign73810_body9_e111361_d_n10, assign73810_body9_e111361_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1704 != 0.0)) && (locals.var_guard1709 != 0.0)) {
        let assign73810_body9_e111359: f64 = (locals.var_t1 - locals.var_t0);
        (assign73810_body9_e111359, (locals.var_t1_dn0 - locals.var_t0_dn0), (locals.var_t1_dn2 - locals.var_t0_dn2), (locals.var_t1_dn4 - locals.var_t0_dn4), (locals.var_t1_dn5 - locals.var_t0_dn5), (locals.var_t1_dn6 - locals.var_t0_dn6), (locals.var_t1_dn7 - locals.var_t0_dn7), (locals.var_t1_dn8 - locals.var_t0_dn8), (locals.var_t1_dn9 - locals.var_t0_dn9), (locals.var_t1_dn10 - locals.var_t0_dn10), (locals.var_t1_dn13 - locals.var_t0_dn13),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
            locals.var_t2 = assign73810_body9_e111361;
            locals.var_t2_dn0 = assign73810_body9_e111361_d_n0;
            locals.var_t2_dn2 = assign73810_body9_e111361_d_n2;
            locals.var_t2_dn4 = assign73810_body9_e111361_d_n4;
            locals.var_t2_dn5 = assign73810_body9_e111361_d_n5;
            locals.var_t2_dn6 = assign73810_body9_e111361_d_n6;
            locals.var_t2_dn7 = assign73810_body9_e111361_d_n7;
            locals.var_t2_dn8 = assign73810_body9_e111361_d_n8;
            locals.var_t2_dn9 = assign73810_body9_e111361_d_n9;
            locals.var_t2_dn10 = assign73810_body9_e111361_d_n10;
            locals.var_t2_dn13 = assign73810_body9_e111361_d_n13;
            locals.var_t2_rv = 0.0;
            let (assign73810_body10_e111377, assign73810_body10_e111377_d_n0, assign73810_body10_e111377_d_n2, assign73810_body10_e111377_d_n4, assign73810_body10_e111377_d_n5, assign73810_body10_e111377_d_n6, assign73810_body10_e111377_d_n7, assign73810_body10_e111377_d_n8, assign73810_body10_e111377_d_n9, assign73810_body10_e111377_d_n10, assign73810_body10_e111377_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1704 != 0.0)) && (locals.var_guard1709 != 0.0)) {
        let assign73810_body10_e111372: f64 = (1.0 + locals.var_t2);
        let assign73810_body10_e111373: f64 = (assign73810_body10_e111372).ln();
        let assign73810_body10_e111375: f64 = (assign73810_body10_e111373 / locals.var_c_sb);
        (assign73810_body10_e111375, ((((locals.var_t2_dn0 / assign73810_body10_e111372) * locals.var_c_sb) - (assign73810_body10_e111373 * locals.var_c_sb_dn0)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn2 / assign73810_body10_e111372) * locals.var_c_sb) - (assign73810_body10_e111373 * locals.var_c_sb_dn2)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn4 / assign73810_body10_e111372) * locals.var_c_sb) - (assign73810_body10_e111373 * locals.var_c_sb_dn4)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn5 / assign73810_body10_e111372) * locals.var_c_sb) - (assign73810_body10_e111373 * locals.var_c_sb_dn5)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn6 / assign73810_body10_e111372) * locals.var_c_sb) - (assign73810_body10_e111373 * locals.var_c_sb_dn6)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn7 / assign73810_body10_e111372) * locals.var_c_sb) - (assign73810_body10_e111373 * locals.var_c_sb_dn7)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn8 / assign73810_body10_e111372) * locals.var_c_sb) - (assign73810_body10_e111373 * locals.var_c_sb_dn8)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn9 / assign73810_body10_e111372) * locals.var_c_sb) - (assign73810_body10_e111373 * locals.var_c_sb_dn9)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn10 / assign73810_body10_e111372) * locals.var_c_sb) - (assign73810_body10_e111373 * locals.var_c_sb_dn10)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn13 / assign73810_body10_e111372) * locals.var_c_sb) - (assign73810_body10_e111373 * locals.var_c_sb_dn13)) / (locals.var_c_sb * locals.var_c_sb)),)
    } else {
        (locals.var_phi_b, locals.var_phi_b_dn0, locals.var_phi_b_dn2, locals.var_phi_b_dn4, locals.var_phi_b_dn5, locals.var_phi_b_dn6, locals.var_phi_b_dn7, locals.var_phi_b_dn8, locals.var_phi_b_dn9, locals.var_phi_b_dn10, locals.var_phi_b_dn13,)
    }
};
            locals.var_phi_b = assign73810_body10_e111377;
            locals.var_phi_b_dn0 = assign73810_body10_e111377_d_n0;
            locals.var_phi_b_dn2 = assign73810_body10_e111377_d_n2;
            locals.var_phi_b_dn4 = assign73810_body10_e111377_d_n4;
            locals.var_phi_b_dn5 = assign73810_body10_e111377_d_n5;
            locals.var_phi_b_dn6 = assign73810_body10_e111377_d_n6;
            locals.var_phi_b_dn7 = assign73810_body10_e111377_d_n7;
            locals.var_phi_b_dn8 = assign73810_body10_e111377_d_n8;
            locals.var_phi_b_dn9 = assign73810_body10_e111377_d_n9;
            locals.var_phi_b_dn10 = assign73810_body10_e111377_d_n10;
            locals.var_phi_b_dn13 = assign73810_body10_e111377_d_n13;
            locals.var_phi_b_rv = 0.0;
            let (assign73810_body11_e111392, assign73810_body11_e111392_d_n0, assign73810_body11_e111392_d_n2, assign73810_body11_e111392_d_n4, assign73810_body11_e111392_d_n5, assign73810_body11_e111392_d_n6, assign73810_body11_e111392_d_n7, assign73810_body11_e111392_d_n8, assign73810_body11_e111392_d_n9, assign73810_body11_e111392_d_n10, assign73810_body11_e111392_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1704 != 0.0)) && (locals.var_guard1709 != 0.0)) {
        let assign73810_body11_e111389: f64 = (1.0 + locals.var_t2);
        let assign73810_body11_e111390: f64 = (locals.var_t1 / assign73810_body11_e111389);
        (assign73810_body11_e111390, (((locals.var_t1_dn0 * assign73810_body11_e111389) - (locals.var_t1 * locals.var_t2_dn0)) / (assign73810_body11_e111389 * assign73810_body11_e111389)), (((locals.var_t1_dn2 * assign73810_body11_e111389) - (locals.var_t1 * locals.var_t2_dn2)) / (assign73810_body11_e111389 * assign73810_body11_e111389)), (((locals.var_t1_dn4 * assign73810_body11_e111389) - (locals.var_t1 * locals.var_t2_dn4)) / (assign73810_body11_e111389 * assign73810_body11_e111389)), (((locals.var_t1_dn5 * assign73810_body11_e111389) - (locals.var_t1 * locals.var_t2_dn5)) / (assign73810_body11_e111389 * assign73810_body11_e111389)), (((locals.var_t1_dn6 * assign73810_body11_e111389) - (locals.var_t1 * locals.var_t2_dn6)) / (assign73810_body11_e111389 * assign73810_body11_e111389)), (((locals.var_t1_dn7 * assign73810_body11_e111389) - (locals.var_t1 * locals.var_t2_dn7)) / (assign73810_body11_e111389 * assign73810_body11_e111389)), (((locals.var_t1_dn8 * assign73810_body11_e111389) - (locals.var_t1 * locals.var_t2_dn8)) / (assign73810_body11_e111389 * assign73810_body11_e111389)), (((locals.var_t1_dn9 * assign73810_body11_e111389) - (locals.var_t1 * locals.var_t2_dn9)) / (assign73810_body11_e111389 * assign73810_body11_e111389)), (((locals.var_t1_dn10 * assign73810_body11_e111389) - (locals.var_t1 * locals.var_t2_dn10)) / (assign73810_body11_e111389 * assign73810_body11_e111389)), (((locals.var_t1_dn13 * assign73810_body11_e111389) - (locals.var_t1 * locals.var_t2_dn13)) / (assign73810_body11_e111389 * assign73810_body11_e111389)),)
    } else {
        (locals.var_phi_b_dpss, locals.var_phi_b_dpss_dn0, locals.var_phi_b_dpss_dn2, locals.var_phi_b_dpss_dn4, locals.var_phi_b_dpss_dn5, locals.var_phi_b_dpss_dn6, locals.var_phi_b_dpss_dn7, locals.var_phi_b_dpss_dn8, locals.var_phi_b_dpss_dn9, locals.var_phi_b_dpss_dn10, locals.var_phi_b_dpss_dn13,)
    }
};
            locals.var_phi_b_dpss = assign73810_body11_e111392;
            locals.var_phi_b_dpss_dn0 = assign73810_body11_e111392_d_n0;
            locals.var_phi_b_dpss_dn2 = assign73810_body11_e111392_d_n2;
            locals.var_phi_b_dpss_dn4 = assign73810_body11_e111392_d_n4;
            locals.var_phi_b_dpss_dn5 = assign73810_body11_e111392_d_n5;
            locals.var_phi_b_dpss_dn6 = assign73810_body11_e111392_d_n6;
            locals.var_phi_b_dpss_dn7 = assign73810_body11_e111392_d_n7;
            locals.var_phi_b_dpss_dn8 = assign73810_body11_e111392_d_n8;
            locals.var_phi_b_dpss_dn9 = assign73810_body11_e111392_d_n9;
            locals.var_phi_b_dpss_dn10 = assign73810_body11_e111392_d_n10;
            locals.var_phi_b_dpss_dn13 = assign73810_body11_e111392_d_n13;
            locals.var_phi_b_dpss_rv = 0.0;
            let (assign73810_body13_e111420, assign73810_body13_e111420_d_n0, assign73810_body13_e111420_d_n2, assign73810_body13_e111420_d_n4, assign73810_body13_e111420_d_n5, assign73810_body13_e111420_d_n6, assign73810_body13_e111420_d_n7, assign73810_body13_e111420_d_n8, assign73810_body13_e111420_d_n9, assign73810_body13_e111420_d_n10, assign73810_body13_e111420_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1704 != 0.0)) && (locals.var_guard1709 == 0.0)) {
        let assign73810_body13_e111418: f64 = (locals.var_ps0ld_vxb - locals.var_dphi_sb);
        (assign73810_body13_e111418, (locals.var_ps0ld_vxb_dn0 - locals.var_dphi_sb_dn0), (locals.var_ps0ld_vxb_dn2 - locals.var_dphi_sb_dn2), (locals.var_ps0ld_vxb_dn4 - locals.var_dphi_sb_dn4), (locals.var_ps0ld_vxb_dn5 - locals.var_dphi_sb_dn5), (locals.var_ps0ld_vxb_dn6 - locals.var_dphi_sb_dn6), (locals.var_ps0ld_vxb_dn7 - locals.var_dphi_sb_dn7), (locals.var_ps0ld_vxb_dn8 - locals.var_dphi_sb_dn8), (locals.var_ps0ld_vxb_dn9 - locals.var_dphi_sb_dn9), (locals.var_ps0ld_vxb_dn10 - locals.var_dphi_sb_dn10), (locals.var_ps0ld_vxb_dn13 - locals.var_dphi_sb_dn13),)
    } else {
        (locals.var_phi_b, locals.var_phi_b_dn0, locals.var_phi_b_dn2, locals.var_phi_b_dn4, locals.var_phi_b_dn5, locals.var_phi_b_dn6, locals.var_phi_b_dn7, locals.var_phi_b_dn8, locals.var_phi_b_dn9, locals.var_phi_b_dn10, locals.var_phi_b_dn13,)
    }
};
            locals.var_phi_b = assign73810_body13_e111420;
            locals.var_phi_b_dn0 = assign73810_body13_e111420_d_n0;
            locals.var_phi_b_dn2 = assign73810_body13_e111420_d_n2;
            locals.var_phi_b_dn4 = assign73810_body13_e111420_d_n4;
            locals.var_phi_b_dn5 = assign73810_body13_e111420_d_n5;
            locals.var_phi_b_dn6 = assign73810_body13_e111420_d_n6;
            locals.var_phi_b_dn7 = assign73810_body13_e111420_d_n7;
            locals.var_phi_b_dn8 = assign73810_body13_e111420_d_n8;
            locals.var_phi_b_dn9 = assign73810_body13_e111420_d_n9;
            locals.var_phi_b_dn10 = assign73810_body13_e111420_d_n10;
            locals.var_phi_b_dn13 = assign73810_body13_e111420_d_n13;
            locals.var_phi_b_rv = 0.0;
            let (assign73810_body14_e111432, assign73810_body14_e111432_d_n0, assign73810_body14_e111432_d_n2, assign73810_body14_e111432_d_n4, assign73810_body14_e111432_d_n5, assign73810_body14_e111432_d_n6, assign73810_body14_e111432_d_n7, assign73810_body14_e111432_d_n8, assign73810_body14_e111432_d_n9, assign73810_body14_e111432_d_n10, assign73810_body14_e111432_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1704 != 0.0)) && (locals.var_guard1709 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_phi_b_dpss, locals.var_phi_b_dpss_dn0, locals.var_phi_b_dpss_dn2, locals.var_phi_b_dpss_dn4, locals.var_phi_b_dpss_dn5, locals.var_phi_b_dpss_dn6, locals.var_phi_b_dpss_dn7, locals.var_phi_b_dpss_dn8, locals.var_phi_b_dpss_dn9, locals.var_phi_b_dpss_dn10, locals.var_phi_b_dpss_dn13,)
    }
};
            locals.var_phi_b_dpss = assign73810_body14_e111432;
            locals.var_phi_b_dpss_dn0 = assign73810_body14_e111432_d_n0;
            locals.var_phi_b_dpss_dn2 = assign73810_body14_e111432_d_n2;
            locals.var_phi_b_dpss_dn4 = assign73810_body14_e111432_d_n4;
            locals.var_phi_b_dpss_dn5 = assign73810_body14_e111432_d_n5;
            locals.var_phi_b_dpss_dn6 = assign73810_body14_e111432_d_n6;
            locals.var_phi_b_dpss_dn7 = assign73810_body14_e111432_d_n7;
            locals.var_phi_b_dpss_dn8 = assign73810_body14_e111432_d_n8;
            locals.var_phi_b_dpss_dn9 = assign73810_body14_e111432_d_n9;
            locals.var_phi_b_dpss_dn10 = assign73810_body14_e111432_d_n10;
            locals.var_phi_b_dpss_dn13 = assign73810_body14_e111432_d_n13;
            locals.var_phi_b_dpss_rv = 0.0;
            let (assign73810_body15_e111443, assign73810_body15_e111443_d_n0, assign73810_body15_e111443_d_n2, assign73810_body15_e111443_d_n4, assign73810_body15_e111443_d_n5, assign73810_body15_e111443_d_n6, assign73810_body15_e111443_d_n7, assign73810_body15_e111443_d_n8, assign73810_body15_e111443_d_n9, assign73810_body15_e111443_d_n10, assign73810_body15_e111443_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1704 != 0.0)) {
        let assign73810_body15_e111441: f64 = (locals.var_beta * locals.var_phi_b);
        (assign73810_body15_e111441, ((locals.var_beta_dn0 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn0)), ((locals.var_beta_dn2 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn2)), ((locals.var_beta_dn4 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn4)), ((locals.var_beta_dn5 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn5)), ((locals.var_beta_dn6 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn6)), ((locals.var_beta_dn7 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn7)), ((locals.var_beta_dn8 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn8)), ((locals.var_beta_dn9 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn9)), ((locals.var_beta_dn10 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn10)), ((locals.var_beta_dn13 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn13)),)
    } else {
        (locals.var_chib, locals.var_chib_dn0, locals.var_chib_dn2, locals.var_chib_dn4, locals.var_chib_dn5, locals.var_chib_dn6, locals.var_chib_dn7, locals.var_chib_dn8, locals.var_chib_dn9, locals.var_chib_dn10, locals.var_chib_dn13,)
    }
};
            locals.var_chib = assign73810_body15_e111443;
            locals.var_chib_dn0 = assign73810_body15_e111443_d_n0;
            locals.var_chib_dn2 = assign73810_body15_e111443_d_n2;
            locals.var_chib_dn4 = assign73810_body15_e111443_d_n4;
            locals.var_chib_dn5 = assign73810_body15_e111443_d_n5;
            locals.var_chib_dn6 = assign73810_body15_e111443_d_n6;
            locals.var_chib_dn7 = assign73810_body15_e111443_d_n7;
            locals.var_chib_dn8 = assign73810_body15_e111443_d_n8;
            locals.var_chib_dn9 = assign73810_body15_e111443_d_n9;
            locals.var_chib_dn10 = assign73810_body15_e111443_d_n10;
            locals.var_chib_dn13 = assign73810_body15_e111443_d_n13;
            locals.var_chib_rv = 0.0;
            let assign73810_body16_e111446: f64 = if locals.var_chi < 0.0 { 1.0 } else { 0.0 };
            locals.var_guard1710 = assign73810_body16_e111446;
            locals.var_guard1710_rv = 0.0;
            let (assign73810_body18_e111471, assign73810_body18_e111471_d_n0, assign73810_body18_e111471_d_n2, assign73810_body18_e111471_d_n4, assign73810_body18_e111471_d_n5, assign73810_body18_e111471_d_n6, assign73810_body18_e111471_d_n7, assign73810_body18_e111471_d_n8, assign73810_body18_e111471_d_n9, assign73810_body18_e111471_d_n10, assign73810_body18_e111471_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1704 != 0.0)) && (locals.var_guard1710 != 0.0)) {
        let assign73810_body18_e111469: f64 = (-0.7071067811865475);
        (assign73810_body18_e111469, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
            locals.var_t0 = assign73810_body18_e111471;
            locals.var_t0_dn0 = assign73810_body18_e111471_d_n0;
            locals.var_t0_dn2 = assign73810_body18_e111471_d_n2;
            locals.var_t0_dn4 = assign73810_body18_e111471_d_n4;
            locals.var_t0_dn5 = assign73810_body18_e111471_d_n5;
            locals.var_t0_dn6 = assign73810_body18_e111471_d_n6;
            locals.var_t0_dn7 = assign73810_body18_e111471_d_n7;
            locals.var_t0_dn8 = assign73810_body18_e111471_d_n8;
            locals.var_t0_dn9 = assign73810_body18_e111471_d_n9;
            locals.var_t0_dn10 = assign73810_body18_e111471_d_n10;
            locals.var_t0_dn13 = assign73810_body18_e111471_d_n13;
            locals.var_t0_rv = 0.0;
            let (assign73810_body19_e111484, assign73810_body19_e111484_d_n0, assign73810_body19_e111484_d_n2, assign73810_body19_e111484_d_n4, assign73810_body19_e111484_d_n5, assign73810_body19_e111484_d_n6, assign73810_body19_e111484_d_n7, assign73810_body19_e111484_d_n8, assign73810_body19_e111484_d_n9, assign73810_body19_e111484_d_n10, assign73810_body19_e111484_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1704 != 0.0)) && (locals.var_guard1710 != 0.0)) {
        let assign73810_body19_e111482: f64 = (locals.var_chi * locals.var_t0);
        (assign73810_body19_e111482, ((locals.var_chi_dn0 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn0)), ((locals.var_chi_dn2 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn2)), ((locals.var_chi_dn4 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn4)), ((locals.var_chi_dn5 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn5)), ((locals.var_chi_dn6 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn6)), ((locals.var_chi_dn7 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn7)), ((locals.var_chi_dn8 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn8)), ((locals.var_chi_dn9 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn9)), ((locals.var_chi_dn10 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn10)), ((locals.var_chi_dn13 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn13)),)
    } else {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn4, locals.var_fb_dn5, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn8, locals.var_fb_dn9, locals.var_fb_dn10, locals.var_fb_dn13,)
    }
};
            locals.var_fb = assign73810_body19_e111484;
            locals.var_fb_dn0 = assign73810_body19_e111484_d_n0;
            locals.var_fb_dn2 = assign73810_body19_e111484_d_n2;
            locals.var_fb_dn4 = assign73810_body19_e111484_d_n4;
            locals.var_fb_dn5 = assign73810_body19_e111484_d_n5;
            locals.var_fb_dn6 = assign73810_body19_e111484_d_n6;
            locals.var_fb_dn7 = assign73810_body19_e111484_d_n7;
            locals.var_fb_dn8 = assign73810_body19_e111484_d_n8;
            locals.var_fb_dn9 = assign73810_body19_e111484_d_n9;
            locals.var_fb_dn10 = assign73810_body19_e111484_d_n10;
            locals.var_fb_dn13 = assign73810_body19_e111484_d_n13;
            locals.var_fb_rv = 0.0;
            let (assign73810_body20_e111497, assign73810_body20_e111497_d_n0, assign73810_body20_e111497_d_n2, assign73810_body20_e111497_d_n4, assign73810_body20_e111497_d_n5, assign73810_body20_e111497_d_n6, assign73810_body20_e111497_d_n7, assign73810_body20_e111497_d_n8, assign73810_body20_e111497_d_n9, assign73810_body20_e111497_d_n10, assign73810_body20_e111497_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1704 != 0.0)) && (locals.var_guard1710 != 0.0)) {
        let assign73810_body20_e111495: f64 = (locals.var_beta * locals.var_t0);
        (assign73810_body20_e111495, ((locals.var_beta_dn0 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn0)), ((locals.var_beta_dn2 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn2)), ((locals.var_beta_dn4 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn4)), ((locals.var_beta_dn5 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn5)), ((locals.var_beta_dn6 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn6)), ((locals.var_beta_dn7 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn7)), ((locals.var_beta_dn8 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn8)), ((locals.var_beta_dn9 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn9)), ((locals.var_beta_dn10 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn10)), ((locals.var_beta_dn13 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn13)),)
    } else {
        (locals.var_fb_dpss, locals.var_fb_dpss_dn0, locals.var_fb_dpss_dn2, locals.var_fb_dpss_dn4, locals.var_fb_dpss_dn5, locals.var_fb_dpss_dn6, locals.var_fb_dpss_dn7, locals.var_fb_dpss_dn8, locals.var_fb_dpss_dn9, locals.var_fb_dpss_dn10, locals.var_fb_dpss_dn13,)
    }
};
            locals.var_fb_dpss = assign73810_body20_e111497;
            locals.var_fb_dpss_dn0 = assign73810_body20_e111497_d_n0;
            locals.var_fb_dpss_dn2 = assign73810_body20_e111497_d_n2;
            locals.var_fb_dpss_dn4 = assign73810_body20_e111497_d_n4;
            locals.var_fb_dpss_dn5 = assign73810_body20_e111497_d_n5;
            locals.var_fb_dpss_dn6 = assign73810_body20_e111497_d_n6;
            locals.var_fb_dpss_dn7 = assign73810_body20_e111497_d_n7;
            locals.var_fb_dpss_dn8 = assign73810_body20_e111497_d_n8;
            locals.var_fb_dpss_dn9 = assign73810_body20_e111497_d_n9;
            locals.var_fb_dpss_dn10 = assign73810_body20_e111497_d_n10;
            locals.var_fb_dpss_dn13 = assign73810_body20_e111497_d_n13;
            locals.var_fb_dpss_rv = 0.0;
            let assign73810_body21_e111500: f64 = if locals.var_chi < 1e-6 { 1.0 } else { 0.0 };
            locals.var_guard1711 = assign73810_body21_e111500;
            locals.var_guard1711_rv = 0.0;
            let (assign73810_body23_e111552, assign73810_body23_e111552_d_n0, assign73810_body23_e111552_d_n2, assign73810_body23_e111552_d_n4, assign73810_body23_e111552_d_n5, assign73810_body23_e111552_d_n6, assign73810_body23_e111552_d_n7, assign73810_body23_e111552_d_n8, assign73810_body23_e111552_d_n9, assign73810_body23_e111552_d_n10, assign73810_body23_e111552_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1704 != 0.0)) && (locals.var_guard1710 == 0.0)) && (locals.var_guard1711 != 0.0)) {
        let assign73810_body23_e111530: f64 = (locals.var_chi * locals.var_chi);
        let assign73810_body23_e111532: f64 = (assign73810_body23_e111530 / 2.0);
        let assign73810_body23_e111536: f64 = (locals.var_chi / 3.0);
        let assign73810_body23_e111540: f64 = (locals.var_chi / 4.0);
        let assign73810_body23_e111544: f64 = (locals.var_chi / 5.0);
        let assign73810_body23_e111545: f64 = (1.0 - assign73810_body23_e111544);
        let assign73810_body23_e111546: f64 = (assign73810_body23_e111540 * assign73810_body23_e111545);
        let assign73810_body23_e111547: f64 = (1.0 - assign73810_body23_e111546);
        let assign73810_body23_e111548: f64 = (assign73810_body23_e111536 * assign73810_body23_e111547);
        let assign73810_body23_e111549: f64 = (1.0 - assign73810_body23_e111548);
        let assign73810_body23_e111550: f64 = (assign73810_body23_e111532 * assign73810_body23_e111549);
        (assign73810_body23_e111550, (((((locals.var_chi_dn0 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn0)) / 2.0) * assign73810_body23_e111549) + (assign73810_body23_e111532 * (-(((locals.var_chi_dn0 / 3.0) * assign73810_body23_e111547) + (assign73810_body23_e111536 * (-(((locals.var_chi_dn0 / 4.0) * assign73810_body23_e111545) + (assign73810_body23_e111540 * (-(locals.var_chi_dn0 / 5.0)))))))))), (((((locals.var_chi_dn2 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn2)) / 2.0) * assign73810_body23_e111549) + (assign73810_body23_e111532 * (-(((locals.var_chi_dn2 / 3.0) * assign73810_body23_e111547) + (assign73810_body23_e111536 * (-(((locals.var_chi_dn2 / 4.0) * assign73810_body23_e111545) + (assign73810_body23_e111540 * (-(locals.var_chi_dn2 / 5.0)))))))))), (((((locals.var_chi_dn4 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn4)) / 2.0) * assign73810_body23_e111549) + (assign73810_body23_e111532 * (-(((locals.var_chi_dn4 / 3.0) * assign73810_body23_e111547) + (assign73810_body23_e111536 * (-(((locals.var_chi_dn4 / 4.0) * assign73810_body23_e111545) + (assign73810_body23_e111540 * (-(locals.var_chi_dn4 / 5.0)))))))))), (((((locals.var_chi_dn5 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn5)) / 2.0) * assign73810_body23_e111549) + (assign73810_body23_e111532 * (-(((locals.var_chi_dn5 / 3.0) * assign73810_body23_e111547) + (assign73810_body23_e111536 * (-(((locals.var_chi_dn5 / 4.0) * assign73810_body23_e111545) + (assign73810_body23_e111540 * (-(locals.var_chi_dn5 / 5.0)))))))))), (((((locals.var_chi_dn6 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn6)) / 2.0) * assign73810_body23_e111549) + (assign73810_body23_e111532 * (-(((locals.var_chi_dn6 / 3.0) * assign73810_body23_e111547) + (assign73810_body23_e111536 * (-(((locals.var_chi_dn6 / 4.0) * assign73810_body23_e111545) + (assign73810_body23_e111540 * (-(locals.var_chi_dn6 / 5.0)))))))))), (((((locals.var_chi_dn7 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn7)) / 2.0) * assign73810_body23_e111549) + (assign73810_body23_e111532 * (-(((locals.var_chi_dn7 / 3.0) * assign73810_body23_e111547) + (assign73810_body23_e111536 * (-(((locals.var_chi_dn7 / 4.0) * assign73810_body23_e111545) + (assign73810_body23_e111540 * (-(locals.var_chi_dn7 / 5.0)))))))))), (((((locals.var_chi_dn8 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn8)) / 2.0) * assign73810_body23_e111549) + (assign73810_body23_e111532 * (-(((locals.var_chi_dn8 / 3.0) * assign73810_body23_e111547) + (assign73810_body23_e111536 * (-(((locals.var_chi_dn8 / 4.0) * assign73810_body23_e111545) + (assign73810_body23_e111540 * (-(locals.var_chi_dn8 / 5.0)))))))))), (((((locals.var_chi_dn9 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn9)) / 2.0) * assign73810_body23_e111549) + (assign73810_body23_e111532 * (-(((locals.var_chi_dn9 / 3.0) * assign73810_body23_e111547) + (assign73810_body23_e111536 * (-(((locals.var_chi_dn9 / 4.0) * assign73810_body23_e111545) + (assign73810_body23_e111540 * (-(locals.var_chi_dn9 / 5.0)))))))))), (((((locals.var_chi_dn10 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn10)) / 2.0) * assign73810_body23_e111549) + (assign73810_body23_e111532 * (-(((locals.var_chi_dn10 / 3.0) * assign73810_body23_e111547) + (assign73810_body23_e111536 * (-(((locals.var_chi_dn10 / 4.0) * assign73810_body23_e111545) + (assign73810_body23_e111540 * (-(locals.var_chi_dn10 / 5.0)))))))))), (((((locals.var_chi_dn13 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn13)) / 2.0) * assign73810_body23_e111549) + (assign73810_body23_e111532 * (-(((locals.var_chi_dn13 / 3.0) * assign73810_body23_e111547) + (assign73810_body23_e111536 * (-(((locals.var_chi_dn13 / 4.0) * assign73810_body23_e111545) + (assign73810_body23_e111540 * (-(locals.var_chi_dn13 / 5.0)))))))))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
            locals.var_t0 = assign73810_body23_e111552;
            locals.var_t0_dn0 = assign73810_body23_e111552_d_n0;
            locals.var_t0_dn2 = assign73810_body23_e111552_d_n2;
            locals.var_t0_dn4 = assign73810_body23_e111552_d_n4;
            locals.var_t0_dn5 = assign73810_body23_e111552_d_n5;
            locals.var_t0_dn6 = assign73810_body23_e111552_d_n6;
            locals.var_t0_dn7 = assign73810_body23_e111552_d_n7;
            locals.var_t0_dn8 = assign73810_body23_e111552_d_n8;
            locals.var_t0_dn9 = assign73810_body23_e111552_d_n9;
            locals.var_t0_dn10 = assign73810_body23_e111552_d_n10;
            locals.var_t0_dn13 = assign73810_body23_e111552_d_n13;
            locals.var_t0_rv = 0.0;
            let (assign73810_body24_e111584, assign73810_body24_e111584_d_n0, assign73810_body24_e111584_d_n2, assign73810_body24_e111584_d_n4, assign73810_body24_e111584_d_n5, assign73810_body24_e111584_d_n6, assign73810_body24_e111584_d_n7, assign73810_body24_e111584_d_n8, assign73810_body24_e111584_d_n9, assign73810_body24_e111584_d_n10, assign73810_body24_e111584_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1704 != 0.0)) && (locals.var_guard1710 == 0.0)) && (locals.var_guard1711 != 0.0)) {
        let assign73810_body24_e111568: f64 = (locals.var_chi / 2.0);
        let assign73810_body24_e111572: f64 = (locals.var_chi / 3.0);
        let assign73810_body24_e111576: f64 = (locals.var_chi / 4.0);
        let assign73810_body24_e111577: f64 = (1.0 - assign73810_body24_e111576);
        let assign73810_body24_e111578: f64 = (assign73810_body24_e111572 * assign73810_body24_e111577);
        let assign73810_body24_e111579: f64 = (1.0 - assign73810_body24_e111578);
        let assign73810_body24_e111580: f64 = (assign73810_body24_e111568 * assign73810_body24_e111579);
        let assign73810_body24_e111581: f64 = (1.0 - assign73810_body24_e111580);
        let assign73810_body24_e111582: f64 = (locals.var_chi * assign73810_body24_e111581);
        (assign73810_body24_e111582, ((locals.var_chi_dn0 * assign73810_body24_e111581) + (locals.var_chi * (-(((locals.var_chi_dn0 / 2.0) * assign73810_body24_e111579) + (assign73810_body24_e111568 * (-(((locals.var_chi_dn0 / 3.0) * assign73810_body24_e111577) + (assign73810_body24_e111572 * (-(locals.var_chi_dn0 / 4.0)))))))))), ((locals.var_chi_dn2 * assign73810_body24_e111581) + (locals.var_chi * (-(((locals.var_chi_dn2 / 2.0) * assign73810_body24_e111579) + (assign73810_body24_e111568 * (-(((locals.var_chi_dn2 / 3.0) * assign73810_body24_e111577) + (assign73810_body24_e111572 * (-(locals.var_chi_dn2 / 4.0)))))))))), ((locals.var_chi_dn4 * assign73810_body24_e111581) + (locals.var_chi * (-(((locals.var_chi_dn4 / 2.0) * assign73810_body24_e111579) + (assign73810_body24_e111568 * (-(((locals.var_chi_dn4 / 3.0) * assign73810_body24_e111577) + (assign73810_body24_e111572 * (-(locals.var_chi_dn4 / 4.0)))))))))), ((locals.var_chi_dn5 * assign73810_body24_e111581) + (locals.var_chi * (-(((locals.var_chi_dn5 / 2.0) * assign73810_body24_e111579) + (assign73810_body24_e111568 * (-(((locals.var_chi_dn5 / 3.0) * assign73810_body24_e111577) + (assign73810_body24_e111572 * (-(locals.var_chi_dn5 / 4.0)))))))))), ((locals.var_chi_dn6 * assign73810_body24_e111581) + (locals.var_chi * (-(((locals.var_chi_dn6 / 2.0) * assign73810_body24_e111579) + (assign73810_body24_e111568 * (-(((locals.var_chi_dn6 / 3.0) * assign73810_body24_e111577) + (assign73810_body24_e111572 * (-(locals.var_chi_dn6 / 4.0)))))))))), ((locals.var_chi_dn7 * assign73810_body24_e111581) + (locals.var_chi * (-(((locals.var_chi_dn7 / 2.0) * assign73810_body24_e111579) + (assign73810_body24_e111568 * (-(((locals.var_chi_dn7 / 3.0) * assign73810_body24_e111577) + (assign73810_body24_e111572 * (-(locals.var_chi_dn7 / 4.0)))))))))), ((locals.var_chi_dn8 * assign73810_body24_e111581) + (locals.var_chi * (-(((locals.var_chi_dn8 / 2.0) * assign73810_body24_e111579) + (assign73810_body24_e111568 * (-(((locals.var_chi_dn8 / 3.0) * assign73810_body24_e111577) + (assign73810_body24_e111572 * (-(locals.var_chi_dn8 / 4.0)))))))))), ((locals.var_chi_dn9 * assign73810_body24_e111581) + (locals.var_chi * (-(((locals.var_chi_dn9 / 2.0) * assign73810_body24_e111579) + (assign73810_body24_e111568 * (-(((locals.var_chi_dn9 / 3.0) * assign73810_body24_e111577) + (assign73810_body24_e111572 * (-(locals.var_chi_dn9 / 4.0)))))))))), ((locals.var_chi_dn10 * assign73810_body24_e111581) + (locals.var_chi * (-(((locals.var_chi_dn10 / 2.0) * assign73810_body24_e111579) + (assign73810_body24_e111568 * (-(((locals.var_chi_dn10 / 3.0) * assign73810_body24_e111577) + (assign73810_body24_e111572 * (-(locals.var_chi_dn10 / 4.0)))))))))), ((locals.var_chi_dn13 * assign73810_body24_e111581) + (locals.var_chi * (-(((locals.var_chi_dn13 / 2.0) * assign73810_body24_e111579) + (assign73810_body24_e111568 * (-(((locals.var_chi_dn13 / 3.0) * assign73810_body24_e111577) + (assign73810_body24_e111572 * (-(locals.var_chi_dn13 / 4.0)))))))))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
            locals.var_t1 = assign73810_body24_e111584;
            locals.var_t1_dn0 = assign73810_body24_e111584_d_n0;
            locals.var_t1_dn2 = assign73810_body24_e111584_d_n2;
            locals.var_t1_dn4 = assign73810_body24_e111584_d_n4;
            locals.var_t1_dn5 = assign73810_body24_e111584_d_n5;
            locals.var_t1_dn6 = assign73810_body24_e111584_d_n6;
            locals.var_t1_dn7 = assign73810_body24_e111584_d_n7;
            locals.var_t1_dn8 = assign73810_body24_e111584_d_n8;
            locals.var_t1_dn9 = assign73810_body24_e111584_d_n9;
            locals.var_t1_dn10 = assign73810_body24_e111584_d_n10;
            locals.var_t1_dn13 = assign73810_body24_e111584_d_n13;
            locals.var_t1_rv = 0.0;
            let (assign73810_body25_e111620, assign73810_body25_e111620_d_n0, assign73810_body25_e111620_d_n2, assign73810_body25_e111620_d_n4, assign73810_body25_e111620_d_n5, assign73810_body25_e111620_d_n6, assign73810_body25_e111620_d_n7, assign73810_body25_e111620_d_n8, assign73810_body25_e111620_d_n9, assign73810_body25_e111620_d_n10, assign73810_body25_e111620_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1704 != 0.0)) && (locals.var_guard1710 == 0.0)) && (locals.var_guard1711 != 0.0)) {
        let assign73810_body25_e111598: f64 = (locals.var_chib * locals.var_chib);
        let assign73810_body25_e111600: f64 = (assign73810_body25_e111598 / 2.0);
        let assign73810_body25_e111604: f64 = (locals.var_chib / 3.0);
        let assign73810_body25_e111608: f64 = (locals.var_chib / 4.0);
        let assign73810_body25_e111612: f64 = (locals.var_chib / 5.0);
        let assign73810_body25_e111613: f64 = (1.0 - assign73810_body25_e111612);
        let assign73810_body25_e111614: f64 = (assign73810_body25_e111608 * assign73810_body25_e111613);
        let assign73810_body25_e111615: f64 = (1.0 - assign73810_body25_e111614);
        let assign73810_body25_e111616: f64 = (assign73810_body25_e111604 * assign73810_body25_e111615);
        let assign73810_body25_e111617: f64 = (1.0 - assign73810_body25_e111616);
        let assign73810_body25_e111618: f64 = (assign73810_body25_e111600 * assign73810_body25_e111617);
        (assign73810_body25_e111618, (((((locals.var_chib_dn0 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn0)) / 2.0) * assign73810_body25_e111617) + (assign73810_body25_e111600 * (-(((locals.var_chib_dn0 / 3.0) * assign73810_body25_e111615) + (assign73810_body25_e111604 * (-(((locals.var_chib_dn0 / 4.0) * assign73810_body25_e111613) + (assign73810_body25_e111608 * (-(locals.var_chib_dn0 / 5.0)))))))))), (((((locals.var_chib_dn2 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn2)) / 2.0) * assign73810_body25_e111617) + (assign73810_body25_e111600 * (-(((locals.var_chib_dn2 / 3.0) * assign73810_body25_e111615) + (assign73810_body25_e111604 * (-(((locals.var_chib_dn2 / 4.0) * assign73810_body25_e111613) + (assign73810_body25_e111608 * (-(locals.var_chib_dn2 / 5.0)))))))))), (((((locals.var_chib_dn4 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn4)) / 2.0) * assign73810_body25_e111617) + (assign73810_body25_e111600 * (-(((locals.var_chib_dn4 / 3.0) * assign73810_body25_e111615) + (assign73810_body25_e111604 * (-(((locals.var_chib_dn4 / 4.0) * assign73810_body25_e111613) + (assign73810_body25_e111608 * (-(locals.var_chib_dn4 / 5.0)))))))))), (((((locals.var_chib_dn5 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn5)) / 2.0) * assign73810_body25_e111617) + (assign73810_body25_e111600 * (-(((locals.var_chib_dn5 / 3.0) * assign73810_body25_e111615) + (assign73810_body25_e111604 * (-(((locals.var_chib_dn5 / 4.0) * assign73810_body25_e111613) + (assign73810_body25_e111608 * (-(locals.var_chib_dn5 / 5.0)))))))))), (((((locals.var_chib_dn6 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn6)) / 2.0) * assign73810_body25_e111617) + (assign73810_body25_e111600 * (-(((locals.var_chib_dn6 / 3.0) * assign73810_body25_e111615) + (assign73810_body25_e111604 * (-(((locals.var_chib_dn6 / 4.0) * assign73810_body25_e111613) + (assign73810_body25_e111608 * (-(locals.var_chib_dn6 / 5.0)))))))))), (((((locals.var_chib_dn7 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn7)) / 2.0) * assign73810_body25_e111617) + (assign73810_body25_e111600 * (-(((locals.var_chib_dn7 / 3.0) * assign73810_body25_e111615) + (assign73810_body25_e111604 * (-(((locals.var_chib_dn7 / 4.0) * assign73810_body25_e111613) + (assign73810_body25_e111608 * (-(locals.var_chib_dn7 / 5.0)))))))))), (((((locals.var_chib_dn8 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn8)) / 2.0) * assign73810_body25_e111617) + (assign73810_body25_e111600 * (-(((locals.var_chib_dn8 / 3.0) * assign73810_body25_e111615) + (assign73810_body25_e111604 * (-(((locals.var_chib_dn8 / 4.0) * assign73810_body25_e111613) + (assign73810_body25_e111608 * (-(locals.var_chib_dn8 / 5.0)))))))))), (((((locals.var_chib_dn9 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn9)) / 2.0) * assign73810_body25_e111617) + (assign73810_body25_e111600 * (-(((locals.var_chib_dn9 / 3.0) * assign73810_body25_e111615) + (assign73810_body25_e111604 * (-(((locals.var_chib_dn9 / 4.0) * assign73810_body25_e111613) + (assign73810_body25_e111608 * (-(locals.var_chib_dn9 / 5.0)))))))))), (((((locals.var_chib_dn10 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn10)) / 2.0) * assign73810_body25_e111617) + (assign73810_body25_e111600 * (-(((locals.var_chib_dn10 / 3.0) * assign73810_body25_e111615) + (assign73810_body25_e111604 * (-(((locals.var_chib_dn10 / 4.0) * assign73810_body25_e111613) + (assign73810_body25_e111608 * (-(locals.var_chib_dn10 / 5.0)))))))))), (((((locals.var_chib_dn13 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn13)) / 2.0) * assign73810_body25_e111617) + (assign73810_body25_e111600 * (-(((locals.var_chib_dn13 / 3.0) * assign73810_body25_e111615) + (assign73810_body25_e111604 * (-(((locals.var_chib_dn13 / 4.0) * assign73810_body25_e111613) + (assign73810_body25_e111608 * (-(locals.var_chib_dn13 / 5.0)))))))))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
            locals.var_t2 = assign73810_body25_e111620;
            locals.var_t2_dn0 = assign73810_body25_e111620_d_n0;
            locals.var_t2_dn2 = assign73810_body25_e111620_d_n2;
            locals.var_t2_dn4 = assign73810_body25_e111620_d_n4;
            locals.var_t2_dn5 = assign73810_body25_e111620_d_n5;
            locals.var_t2_dn6 = assign73810_body25_e111620_d_n6;
            locals.var_t2_dn7 = assign73810_body25_e111620_d_n7;
            locals.var_t2_dn8 = assign73810_body25_e111620_d_n8;
            locals.var_t2_dn9 = assign73810_body25_e111620_d_n9;
            locals.var_t2_dn10 = assign73810_body25_e111620_d_n10;
            locals.var_t2_dn13 = assign73810_body25_e111620_d_n13;
            locals.var_t2_rv = 0.0;
            let (assign73810_body26_e111652, assign73810_body26_e111652_d_n0, assign73810_body26_e111652_d_n2, assign73810_body26_e111652_d_n4, assign73810_body26_e111652_d_n5, assign73810_body26_e111652_d_n6, assign73810_body26_e111652_d_n7, assign73810_body26_e111652_d_n8, assign73810_body26_e111652_d_n9, assign73810_body26_e111652_d_n10, assign73810_body26_e111652_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1704 != 0.0)) && (locals.var_guard1710 == 0.0)) && (locals.var_guard1711 != 0.0)) {
        let assign73810_body26_e111636: f64 = (locals.var_chib / 2.0);
        let assign73810_body26_e111640: f64 = (locals.var_chib / 3.0);
        let assign73810_body26_e111644: f64 = (locals.var_chib / 4.0);
        let assign73810_body26_e111645: f64 = (1.0 - assign73810_body26_e111644);
        let assign73810_body26_e111646: f64 = (assign73810_body26_e111640 * assign73810_body26_e111645);
        let assign73810_body26_e111647: f64 = (1.0 - assign73810_body26_e111646);
        let assign73810_body26_e111648: f64 = (assign73810_body26_e111636 * assign73810_body26_e111647);
        let assign73810_body26_e111649: f64 = (1.0 - assign73810_body26_e111648);
        let assign73810_body26_e111650: f64 = (locals.var_chib * assign73810_body26_e111649);
        (assign73810_body26_e111650, ((locals.var_chib_dn0 * assign73810_body26_e111649) + (locals.var_chib * (-(((locals.var_chib_dn0 / 2.0) * assign73810_body26_e111647) + (assign73810_body26_e111636 * (-(((locals.var_chib_dn0 / 3.0) * assign73810_body26_e111645) + (assign73810_body26_e111640 * (-(locals.var_chib_dn0 / 4.0)))))))))), ((locals.var_chib_dn2 * assign73810_body26_e111649) + (locals.var_chib * (-(((locals.var_chib_dn2 / 2.0) * assign73810_body26_e111647) + (assign73810_body26_e111636 * (-(((locals.var_chib_dn2 / 3.0) * assign73810_body26_e111645) + (assign73810_body26_e111640 * (-(locals.var_chib_dn2 / 4.0)))))))))), ((locals.var_chib_dn4 * assign73810_body26_e111649) + (locals.var_chib * (-(((locals.var_chib_dn4 / 2.0) * assign73810_body26_e111647) + (assign73810_body26_e111636 * (-(((locals.var_chib_dn4 / 3.0) * assign73810_body26_e111645) + (assign73810_body26_e111640 * (-(locals.var_chib_dn4 / 4.0)))))))))), ((locals.var_chib_dn5 * assign73810_body26_e111649) + (locals.var_chib * (-(((locals.var_chib_dn5 / 2.0) * assign73810_body26_e111647) + (assign73810_body26_e111636 * (-(((locals.var_chib_dn5 / 3.0) * assign73810_body26_e111645) + (assign73810_body26_e111640 * (-(locals.var_chib_dn5 / 4.0)))))))))), ((locals.var_chib_dn6 * assign73810_body26_e111649) + (locals.var_chib * (-(((locals.var_chib_dn6 / 2.0) * assign73810_body26_e111647) + (assign73810_body26_e111636 * (-(((locals.var_chib_dn6 / 3.0) * assign73810_body26_e111645) + (assign73810_body26_e111640 * (-(locals.var_chib_dn6 / 4.0)))))))))), ((locals.var_chib_dn7 * assign73810_body26_e111649) + (locals.var_chib * (-(((locals.var_chib_dn7 / 2.0) * assign73810_body26_e111647) + (assign73810_body26_e111636 * (-(((locals.var_chib_dn7 / 3.0) * assign73810_body26_e111645) + (assign73810_body26_e111640 * (-(locals.var_chib_dn7 / 4.0)))))))))), ((locals.var_chib_dn8 * assign73810_body26_e111649) + (locals.var_chib * (-(((locals.var_chib_dn8 / 2.0) * assign73810_body26_e111647) + (assign73810_body26_e111636 * (-(((locals.var_chib_dn8 / 3.0) * assign73810_body26_e111645) + (assign73810_body26_e111640 * (-(locals.var_chib_dn8 / 4.0)))))))))), ((locals.var_chib_dn9 * assign73810_body26_e111649) + (locals.var_chib * (-(((locals.var_chib_dn9 / 2.0) * assign73810_body26_e111647) + (assign73810_body26_e111636 * (-(((locals.var_chib_dn9 / 3.0) * assign73810_body26_e111645) + (assign73810_body26_e111640 * (-(locals.var_chib_dn9 / 4.0)))))))))), ((locals.var_chib_dn10 * assign73810_body26_e111649) + (locals.var_chib * (-(((locals.var_chib_dn10 / 2.0) * assign73810_body26_e111647) + (assign73810_body26_e111636 * (-(((locals.var_chib_dn10 / 3.0) * assign73810_body26_e111645) + (assign73810_body26_e111640 * (-(locals.var_chib_dn10 / 4.0)))))))))), ((locals.var_chib_dn13 * assign73810_body26_e111649) + (locals.var_chib * (-(((locals.var_chib_dn13 / 2.0) * assign73810_body26_e111647) + (assign73810_body26_e111636 * (-(((locals.var_chib_dn13 / 3.0) * assign73810_body26_e111645) + (assign73810_body26_e111640 * (-(locals.var_chib_dn13 / 4.0)))))))))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
            locals.var_t3 = assign73810_body26_e111652;
            locals.var_t3_dn0 = assign73810_body26_e111652_d_n0;
            locals.var_t3_dn2 = assign73810_body26_e111652_d_n2;
            locals.var_t3_dn4 = assign73810_body26_e111652_d_n4;
            locals.var_t3_dn5 = assign73810_body26_e111652_d_n5;
            locals.var_t3_dn6 = assign73810_body26_e111652_d_n6;
            locals.var_t3_dn7 = assign73810_body26_e111652_d_n7;
            locals.var_t3_dn8 = assign73810_body26_e111652_d_n8;
            locals.var_t3_dn9 = assign73810_body26_e111652_d_n9;
            locals.var_t3_dn10 = assign73810_body26_e111652_d_n10;
            locals.var_t3_dn13 = assign73810_body26_e111652_d_n13;
            locals.var_t3_rv = 0.0;
            let (assign73810_body27_e111668, assign73810_body27_e111668_d_n0, assign73810_body27_e111668_d_n2, assign73810_body27_e111668_d_n4, assign73810_body27_e111668_d_n5, assign73810_body27_e111668_d_n6, assign73810_body27_e111668_d_n7, assign73810_body27_e111668_d_n8, assign73810_body27_e111668_d_n9, assign73810_body27_e111668_d_n10, assign73810_body27_e111668_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1704 != 0.0)) && (locals.var_guard1710 == 0.0)) && (locals.var_guard1711 != 0.0)) {
        let assign73810_body27_e111666: f64 = (locals.var_t0 - locals.var_t2);
        (assign73810_body27_e111666, (locals.var_t0_dn0 - locals.var_t2_dn0), (locals.var_t0_dn2 - locals.var_t2_dn2), (locals.var_t0_dn4 - locals.var_t2_dn4), (locals.var_t0_dn5 - locals.var_t2_dn5), (locals.var_t0_dn6 - locals.var_t2_dn6), (locals.var_t0_dn7 - locals.var_t2_dn7), (locals.var_t0_dn8 - locals.var_t2_dn8), (locals.var_t0_dn9 - locals.var_t2_dn9), (locals.var_t0_dn10 - locals.var_t2_dn10), (locals.var_t0_dn13 - locals.var_t2_dn13),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
            locals.var_t4 = assign73810_body27_e111668;
            locals.var_t4_dn0 = assign73810_body27_e111668_d_n0;
            locals.var_t4_dn2 = assign73810_body27_e111668_d_n2;
            locals.var_t4_dn4 = assign73810_body27_e111668_d_n4;
            locals.var_t4_dn5 = assign73810_body27_e111668_d_n5;
            locals.var_t4_dn6 = assign73810_body27_e111668_d_n6;
            locals.var_t4_dn7 = assign73810_body27_e111668_d_n7;
            locals.var_t4_dn8 = assign73810_body27_e111668_d_n8;
            locals.var_t4_dn9 = assign73810_body27_e111668_d_n9;
            locals.var_t4_dn10 = assign73810_body27_e111668_d_n10;
            locals.var_t4_dn13 = assign73810_body27_e111668_d_n13;
            locals.var_t4_rv = 0.0;
            let assign73810_body28_e111671: f64 = if locals.var_t4 > 0.0 { 1.0 } else { 0.0 };
            locals.var_guard1712 = assign73810_body28_e111671;
            locals.var_guard1712_rv = 0.0;
            let (assign73810_body29_e111688, assign73810_body29_e111688_d_n0, assign73810_body29_e111688_d_n2, assign73810_body29_e111688_d_n4, assign73810_body29_e111688_d_n5, assign73810_body29_e111688_d_n6, assign73810_body29_e111688_d_n7, assign73810_body29_e111688_d_n8, assign73810_body29_e111688_d_n9, assign73810_body29_e111688_d_n10, assign73810_body29_e111688_d_n13,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1704 != 0.0)) && (locals.var_guard1710 == 0.0)) && (locals.var_guard1711 != 0.0)) && (locals.var_guard1712 != 0.0)) {
        let assign73810_body29_e111686: f64 = (locals.var_t4).sqrt();
        (assign73810_body29_e111686, (locals.var_t4_dn0 / (2.0 * assign73810_body29_e111686)), (locals.var_t4_dn2 / (2.0 * assign73810_body29_e111686)), (locals.var_t4_dn4 / (2.0 * assign73810_body29_e111686)), (locals.var_t4_dn5 / (2.0 * assign73810_body29_e111686)), (locals.var_t4_dn6 / (2.0 * assign73810_body29_e111686)), (locals.var_t4_dn7 / (2.0 * assign73810_body29_e111686)), (locals.var_t4_dn8 / (2.0 * assign73810_body29_e111686)), (locals.var_t4_dn9 / (2.0 * assign73810_body29_e111686)), (locals.var_t4_dn10 / (2.0 * assign73810_body29_e111686)), (locals.var_t4_dn13 / (2.0 * assign73810_body29_e111686)),)
    } else {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn4, locals.var_fb_dn5, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn8, locals.var_fb_dn9, locals.var_fb_dn10, locals.var_fb_dn13,)
    }
};
            locals.var_fb = assign73810_body29_e111688;
            locals.var_fb_dn0 = assign73810_body29_e111688_d_n0;
            locals.var_fb_dn2 = assign73810_body29_e111688_d_n2;
            locals.var_fb_dn4 = assign73810_body29_e111688_d_n4;
            locals.var_fb_dn5 = assign73810_body29_e111688_d_n5;
            locals.var_fb_dn6 = assign73810_body29_e111688_d_n6;
            locals.var_fb_dn7 = assign73810_body29_e111688_d_n7;
            locals.var_fb_dn8 = assign73810_body29_e111688_d_n8;
            locals.var_fb_dn9 = assign73810_body29_e111688_d_n9;
            locals.var_fb_dn10 = assign73810_body29_e111688_d_n10;
            locals.var_fb_dn13 = assign73810_body29_e111688_d_n13;
            locals.var_fb_rv = 0.0;
            let (assign73810_body30_e111714, assign73810_body30_e111714_d_n0, assign73810_body30_e111714_d_n2, assign73810_body30_e111714_d_n4, assign73810_body30_e111714_d_n5, assign73810_body30_e111714_d_n6, assign73810_body30_e111714_d_n7, assign73810_body30_e111714_d_n8, assign73810_body30_e111714_d_n9, assign73810_body30_e111714_d_n10, assign73810_body30_e111714_d_n13,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1704 != 0.0)) && (locals.var_guard1710 == 0.0)) && (locals.var_guard1711 != 0.0)) && (locals.var_guard1712 != 0.0)) {
        let assign73810_body30_e111704: f64 = (locals.var_beta * 0.5);
        let assign73810_body30_e111708: f64 = (locals.var_phi_b_dpss * locals.var_t3);
        let assign73810_body30_e111709: f64 = (locals.var_t1 - assign73810_body30_e111708);
        let assign73810_body30_e111710: f64 = (assign73810_body30_e111704 * assign73810_body30_e111709);
        let assign73810_body30_e111712: f64 = (assign73810_body30_e111710 / locals.var_fb);
        (assign73810_body30_e111712, ((((((locals.var_beta_dn0 * 0.5) * assign73810_body30_e111709) + (assign73810_body30_e111704 * (locals.var_t1_dn0 - ((locals.var_phi_b_dpss_dn0 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn0))))) * locals.var_fb) - (assign73810_body30_e111710 * locals.var_fb_dn0)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn2 * 0.5) * assign73810_body30_e111709) + (assign73810_body30_e111704 * (locals.var_t1_dn2 - ((locals.var_phi_b_dpss_dn2 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn2))))) * locals.var_fb) - (assign73810_body30_e111710 * locals.var_fb_dn2)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn4 * 0.5) * assign73810_body30_e111709) + (assign73810_body30_e111704 * (locals.var_t1_dn4 - ((locals.var_phi_b_dpss_dn4 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn4))))) * locals.var_fb) - (assign73810_body30_e111710 * locals.var_fb_dn4)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn5 * 0.5) * assign73810_body30_e111709) + (assign73810_body30_e111704 * (locals.var_t1_dn5 - ((locals.var_phi_b_dpss_dn5 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn5))))) * locals.var_fb) - (assign73810_body30_e111710 * locals.var_fb_dn5)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn6 * 0.5) * assign73810_body30_e111709) + (assign73810_body30_e111704 * (locals.var_t1_dn6 - ((locals.var_phi_b_dpss_dn6 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn6))))) * locals.var_fb) - (assign73810_body30_e111710 * locals.var_fb_dn6)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn7 * 0.5) * assign73810_body30_e111709) + (assign73810_body30_e111704 * (locals.var_t1_dn7 - ((locals.var_phi_b_dpss_dn7 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn7))))) * locals.var_fb) - (assign73810_body30_e111710 * locals.var_fb_dn7)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn8 * 0.5) * assign73810_body30_e111709) + (assign73810_body30_e111704 * (locals.var_t1_dn8 - ((locals.var_phi_b_dpss_dn8 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn8))))) * locals.var_fb) - (assign73810_body30_e111710 * locals.var_fb_dn8)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn9 * 0.5) * assign73810_body30_e111709) + (assign73810_body30_e111704 * (locals.var_t1_dn9 - ((locals.var_phi_b_dpss_dn9 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn9))))) * locals.var_fb) - (assign73810_body30_e111710 * locals.var_fb_dn9)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn10 * 0.5) * assign73810_body30_e111709) + (assign73810_body30_e111704 * (locals.var_t1_dn10 - ((locals.var_phi_b_dpss_dn10 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn10))))) * locals.var_fb) - (assign73810_body30_e111710 * locals.var_fb_dn10)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn13 * 0.5) * assign73810_body30_e111709) + (assign73810_body30_e111704 * (locals.var_t1_dn13 - ((locals.var_phi_b_dpss_dn13 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn13))))) * locals.var_fb) - (assign73810_body30_e111710 * locals.var_fb_dn13)) / (locals.var_fb * locals.var_fb)),)
    } else {
        (locals.var_fb_dpss, locals.var_fb_dpss_dn0, locals.var_fb_dpss_dn2, locals.var_fb_dpss_dn4, locals.var_fb_dpss_dn5, locals.var_fb_dpss_dn6, locals.var_fb_dpss_dn7, locals.var_fb_dpss_dn8, locals.var_fb_dpss_dn9, locals.var_fb_dpss_dn10, locals.var_fb_dpss_dn13,)
    }
};
            locals.var_fb_dpss = assign73810_body30_e111714;
            locals.var_fb_dpss_dn0 = assign73810_body30_e111714_d_n0;
            locals.var_fb_dpss_dn2 = assign73810_body30_e111714_d_n2;
            locals.var_fb_dpss_dn4 = assign73810_body30_e111714_d_n4;
            locals.var_fb_dpss_dn5 = assign73810_body30_e111714_d_n5;
            locals.var_fb_dpss_dn6 = assign73810_body30_e111714_d_n6;
            locals.var_fb_dpss_dn7 = assign73810_body30_e111714_d_n7;
            locals.var_fb_dpss_dn8 = assign73810_body30_e111714_d_n8;
            locals.var_fb_dpss_dn9 = assign73810_body30_e111714_d_n9;
            locals.var_fb_dpss_dn10 = assign73810_body30_e111714_d_n10;
            locals.var_fb_dpss_dn13 = assign73810_body30_e111714_d_n13;
            locals.var_fb_dpss_rv = 0.0;
            let (assign73810_body32_e111750, assign73810_body32_e111750_d_n0, assign73810_body32_e111750_d_n2, assign73810_body32_e111750_d_n4, assign73810_body32_e111750_d_n5, assign73810_body32_e111750_d_n6, assign73810_body32_e111750_d_n7, assign73810_body32_e111750_d_n8, assign73810_body32_e111750_d_n9, assign73810_body32_e111750_d_n10, assign73810_body32_e111750_d_n13,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1704 != 0.0)) && (locals.var_guard1710 == 0.0)) && (locals.var_guard1711 != 0.0)) && (locals.var_guard1712 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn4, locals.var_fb_dn5, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn8, locals.var_fb_dn9, locals.var_fb_dn10, locals.var_fb_dn13,)
    }
};
            locals.var_fb = assign73810_body32_e111750;
            locals.var_fb_dn0 = assign73810_body32_e111750_d_n0;
            locals.var_fb_dn2 = assign73810_body32_e111750_d_n2;
            locals.var_fb_dn4 = assign73810_body32_e111750_d_n4;
            locals.var_fb_dn5 = assign73810_body32_e111750_d_n5;
            locals.var_fb_dn6 = assign73810_body32_e111750_d_n6;
            locals.var_fb_dn7 = assign73810_body32_e111750_d_n7;
            locals.var_fb_dn8 = assign73810_body32_e111750_d_n8;
            locals.var_fb_dn9 = assign73810_body32_e111750_d_n9;
            locals.var_fb_dn10 = assign73810_body32_e111750_d_n10;
            locals.var_fb_dn13 = assign73810_body32_e111750_d_n13;
            locals.var_fb_rv = 0.0;
            let (assign73810_body33_e111767, assign73810_body33_e111767_d_n0, assign73810_body33_e111767_d_n2, assign73810_body33_e111767_d_n4, assign73810_body33_e111767_d_n5, assign73810_body33_e111767_d_n6, assign73810_body33_e111767_d_n7, assign73810_body33_e111767_d_n8, assign73810_body33_e111767_d_n9, assign73810_body33_e111767_d_n10, assign73810_body33_e111767_d_n13,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1704 != 0.0)) && (locals.var_guard1710 == 0.0)) && (locals.var_guard1711 != 0.0)) && (locals.var_guard1712 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fb_dpss, locals.var_fb_dpss_dn0, locals.var_fb_dpss_dn2, locals.var_fb_dpss_dn4, locals.var_fb_dpss_dn5, locals.var_fb_dpss_dn6, locals.var_fb_dpss_dn7, locals.var_fb_dpss_dn8, locals.var_fb_dpss_dn9, locals.var_fb_dpss_dn10, locals.var_fb_dpss_dn13,)
    }
};
            locals.var_fb_dpss = assign73810_body33_e111767;
            locals.var_fb_dpss_dn0 = assign73810_body33_e111767_d_n0;
            locals.var_fb_dpss_dn2 = assign73810_body33_e111767_d_n2;
            locals.var_fb_dpss_dn4 = assign73810_body33_e111767_d_n4;
            locals.var_fb_dpss_dn5 = assign73810_body33_e111767_d_n5;
            locals.var_fb_dpss_dn6 = assign73810_body33_e111767_d_n6;
            locals.var_fb_dpss_dn7 = assign73810_body33_e111767_d_n7;
            locals.var_fb_dpss_dn8 = assign73810_body33_e111767_d_n8;
            locals.var_fb_dpss_dn9 = assign73810_body33_e111767_d_n9;
            locals.var_fb_dpss_dn10 = assign73810_body33_e111767_d_n10;
            locals.var_fb_dpss_dn13 = assign73810_body33_e111767_d_n13;
            locals.var_fb_dpss_rv = 0.0;
            let (assign73810_body34_e111784, assign73810_body34_e111784_d_n0, assign73810_body34_e111784_d_n2, assign73810_body34_e111784_d_n4, assign73810_body34_e111784_d_n5, assign73810_body34_e111784_d_n6, assign73810_body34_e111784_d_n7, assign73810_body34_e111784_d_n8, assign73810_body34_e111784_d_n9, assign73810_body34_e111784_d_n10, assign73810_body34_e111784_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1704 != 0.0)) && (locals.var_guard1710 == 0.0)) && (locals.var_guard1711 == 0.0)) {
        let assign73810_body34_e111781: f64 = (-locals.var_chi);
        let assign73810_body34_e111782: f64 = (assign73810_body34_e111781).exp();
        (assign73810_body34_e111782, (assign73810_body34_e111782 * (-locals.var_chi_dn0)), (assign73810_body34_e111782 * (-locals.var_chi_dn2)), (assign73810_body34_e111782 * (-locals.var_chi_dn4)), (assign73810_body34_e111782 * (-locals.var_chi_dn5)), (assign73810_body34_e111782 * (-locals.var_chi_dn6)), (assign73810_body34_e111782 * (-locals.var_chi_dn7)), (assign73810_body34_e111782 * (-locals.var_chi_dn8)), (assign73810_body34_e111782 * (-locals.var_chi_dn9)), (assign73810_body34_e111782 * (-locals.var_chi_dn10)), (assign73810_body34_e111782 * (-locals.var_chi_dn13)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
            locals.var_t0 = assign73810_body34_e111784;
            locals.var_t0_dn0 = assign73810_body34_e111784_d_n0;
            locals.var_t0_dn2 = assign73810_body34_e111784_d_n2;
            locals.var_t0_dn4 = assign73810_body34_e111784_d_n4;
            locals.var_t0_dn5 = assign73810_body34_e111784_d_n5;
            locals.var_t0_dn6 = assign73810_body34_e111784_d_n6;
            locals.var_t0_dn7 = assign73810_body34_e111784_d_n7;
            locals.var_t0_dn8 = assign73810_body34_e111784_d_n8;
            locals.var_t0_dn9 = assign73810_body34_e111784_d_n9;
            locals.var_t0_dn10 = assign73810_body34_e111784_d_n10;
            locals.var_t0_dn13 = assign73810_body34_e111784_d_n13;
            locals.var_t0_rv = 0.0;
            let (assign73810_body35_e111801, assign73810_body35_e111801_d_n0, assign73810_body35_e111801_d_n2, assign73810_body35_e111801_d_n4, assign73810_body35_e111801_d_n5, assign73810_body35_e111801_d_n6, assign73810_body35_e111801_d_n7, assign73810_body35_e111801_d_n8, assign73810_body35_e111801_d_n9, assign73810_body35_e111801_d_n10, assign73810_body35_e111801_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1704 != 0.0)) && (locals.var_guard1710 == 0.0)) && (locals.var_guard1711 == 0.0)) {
        let assign73810_body35_e111798: f64 = (-locals.var_chib);
        let assign73810_body35_e111799: f64 = (assign73810_body35_e111798).exp();
        (assign73810_body35_e111799, (assign73810_body35_e111799 * (-locals.var_chib_dn0)), (assign73810_body35_e111799 * (-locals.var_chib_dn2)), (assign73810_body35_e111799 * (-locals.var_chib_dn4)), (assign73810_body35_e111799 * (-locals.var_chib_dn5)), (assign73810_body35_e111799 * (-locals.var_chib_dn6)), (assign73810_body35_e111799 * (-locals.var_chib_dn7)), (assign73810_body35_e111799 * (-locals.var_chib_dn8)), (assign73810_body35_e111799 * (-locals.var_chib_dn9)), (assign73810_body35_e111799 * (-locals.var_chib_dn10)), (assign73810_body35_e111799 * (-locals.var_chib_dn13)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
            locals.var_t1 = assign73810_body35_e111801;
            locals.var_t1_dn0 = assign73810_body35_e111801_d_n0;
            locals.var_t1_dn2 = assign73810_body35_e111801_d_n2;
            locals.var_t1_dn4 = assign73810_body35_e111801_d_n4;
            locals.var_t1_dn5 = assign73810_body35_e111801_d_n5;
            locals.var_t1_dn6 = assign73810_body35_e111801_d_n6;
            locals.var_t1_dn7 = assign73810_body35_e111801_d_n7;
            locals.var_t1_dn8 = assign73810_body35_e111801_d_n8;
            locals.var_t1_dn9 = assign73810_body35_e111801_d_n9;
            locals.var_t1_dn10 = assign73810_body35_e111801_d_n10;
            locals.var_t1_dn13 = assign73810_body35_e111801_d_n13;
            locals.var_t1_rv = 0.0;
            let (assign73810_body36_e111822, assign73810_body36_e111822_d_n0, assign73810_body36_e111822_d_n2, assign73810_body36_e111822_d_n4, assign73810_body36_e111822_d_n5, assign73810_body36_e111822_d_n6, assign73810_body36_e111822_d_n7, assign73810_body36_e111822_d_n8, assign73810_body36_e111822_d_n9, assign73810_body36_e111822_d_n10, assign73810_body36_e111822_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1704 != 0.0)) && (locals.var_guard1710 == 0.0)) && (locals.var_guard1711 == 0.0)) {
        let assign73810_body36_e111816: f64 = (locals.var_chi - locals.var_chib);
        let assign73810_body36_e111819: f64 = (locals.var_t0 - locals.var_t1);
        let assign73810_body36_e111820: f64 = (assign73810_body36_e111816 + assign73810_body36_e111819);
        (assign73810_body36_e111820, ((locals.var_chi_dn0 - locals.var_chib_dn0) + (locals.var_t0_dn0 - locals.var_t1_dn0)), ((locals.var_chi_dn2 - locals.var_chib_dn2) + (locals.var_t0_dn2 - locals.var_t1_dn2)), ((locals.var_chi_dn4 - locals.var_chib_dn4) + (locals.var_t0_dn4 - locals.var_t1_dn4)), ((locals.var_chi_dn5 - locals.var_chib_dn5) + (locals.var_t0_dn5 - locals.var_t1_dn5)), ((locals.var_chi_dn6 - locals.var_chib_dn6) + (locals.var_t0_dn6 - locals.var_t1_dn6)), ((locals.var_chi_dn7 - locals.var_chib_dn7) + (locals.var_t0_dn7 - locals.var_t1_dn7)), ((locals.var_chi_dn8 - locals.var_chib_dn8) + (locals.var_t0_dn8 - locals.var_t1_dn8)), ((locals.var_chi_dn9 - locals.var_chib_dn9) + (locals.var_t0_dn9 - locals.var_t1_dn9)), ((locals.var_chi_dn10 - locals.var_chib_dn10) + (locals.var_t0_dn10 - locals.var_t1_dn10)), ((locals.var_chi_dn13 - locals.var_chib_dn13) + (locals.var_t0_dn13 - locals.var_t1_dn13)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
            locals.var_t4 = assign73810_body36_e111822;
            locals.var_t4_dn0 = assign73810_body36_e111822_d_n0;
            locals.var_t4_dn2 = assign73810_body36_e111822_d_n2;
            locals.var_t4_dn4 = assign73810_body36_e111822_d_n4;
            locals.var_t4_dn5 = assign73810_body36_e111822_d_n5;
            locals.var_t4_dn6 = assign73810_body36_e111822_d_n6;
            locals.var_t4_dn7 = assign73810_body36_e111822_d_n7;
            locals.var_t4_dn8 = assign73810_body36_e111822_d_n8;
            locals.var_t4_dn9 = assign73810_body36_e111822_d_n9;
            locals.var_t4_dn10 = assign73810_body36_e111822_d_n10;
            locals.var_t4_dn13 = assign73810_body36_e111822_d_n13;
            locals.var_t4_rv = 0.0;
            let assign73810_body37_e111825: f64 = if locals.var_t4 > 0.0 { 1.0 } else { 0.0 };
            locals.var_guard1713 = assign73810_body37_e111825;
            locals.var_guard1713_rv = 0.0;
            let (assign73810_body38_e111843, assign73810_body38_e111843_d_n0, assign73810_body38_e111843_d_n2, assign73810_body38_e111843_d_n4, assign73810_body38_e111843_d_n5, assign73810_body38_e111843_d_n6, assign73810_body38_e111843_d_n7, assign73810_body38_e111843_d_n8, assign73810_body38_e111843_d_n9, assign73810_body38_e111843_d_n10, assign73810_body38_e111843_d_n13,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1704 != 0.0)) && (locals.var_guard1710 == 0.0)) && (locals.var_guard1711 == 0.0)) && (locals.var_guard1713 != 0.0)) {
        let assign73810_body38_e111841: f64 = (locals.var_t4).sqrt();
        (assign73810_body38_e111841, (locals.var_t4_dn0 / (2.0 * assign73810_body38_e111841)), (locals.var_t4_dn2 / (2.0 * assign73810_body38_e111841)), (locals.var_t4_dn4 / (2.0 * assign73810_body38_e111841)), (locals.var_t4_dn5 / (2.0 * assign73810_body38_e111841)), (locals.var_t4_dn6 / (2.0 * assign73810_body38_e111841)), (locals.var_t4_dn7 / (2.0 * assign73810_body38_e111841)), (locals.var_t4_dn8 / (2.0 * assign73810_body38_e111841)), (locals.var_t4_dn9 / (2.0 * assign73810_body38_e111841)), (locals.var_t4_dn10 / (2.0 * assign73810_body38_e111841)), (locals.var_t4_dn13 / (2.0 * assign73810_body38_e111841)),)
    } else {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn4, locals.var_fb_dn5, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn8, locals.var_fb_dn9, locals.var_fb_dn10, locals.var_fb_dn13,)
    }
};
            locals.var_fb = assign73810_body38_e111843;
            locals.var_fb_dn0 = assign73810_body38_e111843_d_n0;
            locals.var_fb_dn2 = assign73810_body38_e111843_d_n2;
            locals.var_fb_dn4 = assign73810_body38_e111843_d_n4;
            locals.var_fb_dn5 = assign73810_body38_e111843_d_n5;
            locals.var_fb_dn6 = assign73810_body38_e111843_d_n6;
            locals.var_fb_dn7 = assign73810_body38_e111843_d_n7;
            locals.var_fb_dn8 = assign73810_body38_e111843_d_n8;
            locals.var_fb_dn9 = assign73810_body38_e111843_d_n9;
            locals.var_fb_dn10 = assign73810_body38_e111843_d_n10;
            locals.var_fb_dn13 = assign73810_body38_e111843_d_n13;
            locals.var_fb_rv = 0.0;
            let (assign73810_body39_e111874, assign73810_body39_e111874_d_n0, assign73810_body39_e111874_d_n2, assign73810_body39_e111874_d_n4, assign73810_body39_e111874_d_n5, assign73810_body39_e111874_d_n6, assign73810_body39_e111874_d_n7, assign73810_body39_e111874_d_n8, assign73810_body39_e111874_d_n9, assign73810_body39_e111874_d_n10, assign73810_body39_e111874_d_n13,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1704 != 0.0)) && (locals.var_guard1710 == 0.0)) && (locals.var_guard1711 == 0.0)) && (locals.var_guard1713 != 0.0)) {
        let assign73810_body39_e111860: f64 = (locals.var_beta * 0.5);
        let assign73810_body39_e111863: f64 = (1.0 - locals.var_t0);
        let assign73810_body39_e111867: f64 = (1.0 - locals.var_t1);
        let assign73810_body39_e111868: f64 = (locals.var_phi_b_dpss * assign73810_body39_e111867);
        let assign73810_body39_e111869: f64 = (assign73810_body39_e111863 - assign73810_body39_e111868);
        let assign73810_body39_e111870: f64 = (assign73810_body39_e111860 * assign73810_body39_e111869);
        let assign73810_body39_e111872: f64 = (assign73810_body39_e111870 / locals.var_fb);
        (assign73810_body39_e111872, ((((((locals.var_beta_dn0 * 0.5) * assign73810_body39_e111869) + (assign73810_body39_e111860 * ((-locals.var_t0_dn0) - ((locals.var_phi_b_dpss_dn0 * assign73810_body39_e111867) + (locals.var_phi_b_dpss * (-locals.var_t1_dn0)))))) * locals.var_fb) - (assign73810_body39_e111870 * locals.var_fb_dn0)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn2 * 0.5) * assign73810_body39_e111869) + (assign73810_body39_e111860 * ((-locals.var_t0_dn2) - ((locals.var_phi_b_dpss_dn2 * assign73810_body39_e111867) + (locals.var_phi_b_dpss * (-locals.var_t1_dn2)))))) * locals.var_fb) - (assign73810_body39_e111870 * locals.var_fb_dn2)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn4 * 0.5) * assign73810_body39_e111869) + (assign73810_body39_e111860 * ((-locals.var_t0_dn4) - ((locals.var_phi_b_dpss_dn4 * assign73810_body39_e111867) + (locals.var_phi_b_dpss * (-locals.var_t1_dn4)))))) * locals.var_fb) - (assign73810_body39_e111870 * locals.var_fb_dn4)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn5 * 0.5) * assign73810_body39_e111869) + (assign73810_body39_e111860 * ((-locals.var_t0_dn5) - ((locals.var_phi_b_dpss_dn5 * assign73810_body39_e111867) + (locals.var_phi_b_dpss * (-locals.var_t1_dn5)))))) * locals.var_fb) - (assign73810_body39_e111870 * locals.var_fb_dn5)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn6 * 0.5) * assign73810_body39_e111869) + (assign73810_body39_e111860 * ((-locals.var_t0_dn6) - ((locals.var_phi_b_dpss_dn6 * assign73810_body39_e111867) + (locals.var_phi_b_dpss * (-locals.var_t1_dn6)))))) * locals.var_fb) - (assign73810_body39_e111870 * locals.var_fb_dn6)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn7 * 0.5) * assign73810_body39_e111869) + (assign73810_body39_e111860 * ((-locals.var_t0_dn7) - ((locals.var_phi_b_dpss_dn7 * assign73810_body39_e111867) + (locals.var_phi_b_dpss * (-locals.var_t1_dn7)))))) * locals.var_fb) - (assign73810_body39_e111870 * locals.var_fb_dn7)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn8 * 0.5) * assign73810_body39_e111869) + (assign73810_body39_e111860 * ((-locals.var_t0_dn8) - ((locals.var_phi_b_dpss_dn8 * assign73810_body39_e111867) + (locals.var_phi_b_dpss * (-locals.var_t1_dn8)))))) * locals.var_fb) - (assign73810_body39_e111870 * locals.var_fb_dn8)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn9 * 0.5) * assign73810_body39_e111869) + (assign73810_body39_e111860 * ((-locals.var_t0_dn9) - ((locals.var_phi_b_dpss_dn9 * assign73810_body39_e111867) + (locals.var_phi_b_dpss * (-locals.var_t1_dn9)))))) * locals.var_fb) - (assign73810_body39_e111870 * locals.var_fb_dn9)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn10 * 0.5) * assign73810_body39_e111869) + (assign73810_body39_e111860 * ((-locals.var_t0_dn10) - ((locals.var_phi_b_dpss_dn10 * assign73810_body39_e111867) + (locals.var_phi_b_dpss * (-locals.var_t1_dn10)))))) * locals.var_fb) - (assign73810_body39_e111870 * locals.var_fb_dn10)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn13 * 0.5) * assign73810_body39_e111869) + (assign73810_body39_e111860 * ((-locals.var_t0_dn13) - ((locals.var_phi_b_dpss_dn13 * assign73810_body39_e111867) + (locals.var_phi_b_dpss * (-locals.var_t1_dn13)))))) * locals.var_fb) - (assign73810_body39_e111870 * locals.var_fb_dn13)) / (locals.var_fb * locals.var_fb)),)
    } else {
        (locals.var_fb_dpss, locals.var_fb_dpss_dn0, locals.var_fb_dpss_dn2, locals.var_fb_dpss_dn4, locals.var_fb_dpss_dn5, locals.var_fb_dpss_dn6, locals.var_fb_dpss_dn7, locals.var_fb_dpss_dn8, locals.var_fb_dpss_dn9, locals.var_fb_dpss_dn10, locals.var_fb_dpss_dn13,)
    }
};
            locals.var_fb_dpss = assign73810_body39_e111874;
            locals.var_fb_dpss_dn0 = assign73810_body39_e111874_d_n0;
            locals.var_fb_dpss_dn2 = assign73810_body39_e111874_d_n2;
            locals.var_fb_dpss_dn4 = assign73810_body39_e111874_d_n4;
            locals.var_fb_dpss_dn5 = assign73810_body39_e111874_d_n5;
            locals.var_fb_dpss_dn6 = assign73810_body39_e111874_d_n6;
            locals.var_fb_dpss_dn7 = assign73810_body39_e111874_d_n7;
            locals.var_fb_dpss_dn8 = assign73810_body39_e111874_d_n8;
            locals.var_fb_dpss_dn9 = assign73810_body39_e111874_d_n9;
            locals.var_fb_dpss_dn10 = assign73810_body39_e111874_d_n10;
            locals.var_fb_dpss_dn13 = assign73810_body39_e111874_d_n13;
            locals.var_fb_dpss_rv = 0.0;
            let (assign73810_body41_e111912, assign73810_body41_e111912_d_n0, assign73810_body41_e111912_d_n2, assign73810_body41_e111912_d_n4, assign73810_body41_e111912_d_n5, assign73810_body41_e111912_d_n6, assign73810_body41_e111912_d_n7, assign73810_body41_e111912_d_n8, assign73810_body41_e111912_d_n9, assign73810_body41_e111912_d_n10, assign73810_body41_e111912_d_n13,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1704 != 0.0)) && (locals.var_guard1710 == 0.0)) && (locals.var_guard1711 == 0.0)) && (locals.var_guard1713 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn4, locals.var_fb_dn5, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn8, locals.var_fb_dn9, locals.var_fb_dn10, locals.var_fb_dn13,)
    }
};
            locals.var_fb = assign73810_body41_e111912;
            locals.var_fb_dn0 = assign73810_body41_e111912_d_n0;
            locals.var_fb_dn2 = assign73810_body41_e111912_d_n2;
            locals.var_fb_dn4 = assign73810_body41_e111912_d_n4;
            locals.var_fb_dn5 = assign73810_body41_e111912_d_n5;
            locals.var_fb_dn6 = assign73810_body41_e111912_d_n6;
            locals.var_fb_dn7 = assign73810_body41_e111912_d_n7;
            locals.var_fb_dn8 = assign73810_body41_e111912_d_n8;
            locals.var_fb_dn9 = assign73810_body41_e111912_d_n9;
            locals.var_fb_dn10 = assign73810_body41_e111912_d_n10;
            locals.var_fb_dn13 = assign73810_body41_e111912_d_n13;
            locals.var_fb_rv = 0.0;
            let (assign73810_body42_e111930, assign73810_body42_e111930_d_n0, assign73810_body42_e111930_d_n2, assign73810_body42_e111930_d_n4, assign73810_body42_e111930_d_n5, assign73810_body42_e111930_d_n6, assign73810_body42_e111930_d_n7, assign73810_body42_e111930_d_n8, assign73810_body42_e111930_d_n9, assign73810_body42_e111930_d_n10, assign73810_body42_e111930_d_n13,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1704 != 0.0)) && (locals.var_guard1710 == 0.0)) && (locals.var_guard1711 == 0.0)) && (locals.var_guard1713 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fb_dpss, locals.var_fb_dpss_dn0, locals.var_fb_dpss_dn2, locals.var_fb_dpss_dn4, locals.var_fb_dpss_dn5, locals.var_fb_dpss_dn6, locals.var_fb_dpss_dn7, locals.var_fb_dpss_dn8, locals.var_fb_dpss_dn9, locals.var_fb_dpss_dn10, locals.var_fb_dpss_dn13,)
    }
};
            locals.var_fb_dpss = assign73810_body42_e111930;
            locals.var_fb_dpss_dn0 = assign73810_body42_e111930_d_n0;
            locals.var_fb_dpss_dn2 = assign73810_body42_e111930_d_n2;
            locals.var_fb_dpss_dn4 = assign73810_body42_e111930_d_n4;
            locals.var_fb_dpss_dn5 = assign73810_body42_e111930_d_n5;
            locals.var_fb_dpss_dn6 = assign73810_body42_e111930_d_n6;
            locals.var_fb_dpss_dn7 = assign73810_body42_e111930_d_n7;
            locals.var_fb_dpss_dn8 = assign73810_body42_e111930_d_n8;
            locals.var_fb_dpss_dn9 = assign73810_body42_e111930_d_n9;
            locals.var_fb_dpss_dn10 = assign73810_body42_e111930_d_n10;
            locals.var_fb_dpss_dn13 = assign73810_body42_e111930_d_n13;
            locals.var_fb_dpss_rv = 0.0;
            let assign73810_body43_e111933: f64 = if locals.var_chi < 0.0 { 1.0 } else { 0.0 };
            locals.var_guard1714 = assign73810_body43_e111933;
            locals.var_guard1714_rv = 0.0;
            let (assign73810_body45_e111957, assign73810_body45_e111957_d_n0, assign73810_body45_e111957_d_n2, assign73810_body45_e111957_d_n4, assign73810_body45_e111957_d_n5, assign73810_body45_e111957_d_n6, assign73810_body45_e111957_d_n7, assign73810_body45_e111957_d_n8, assign73810_body45_e111957_d_n9, assign73810_body45_e111957_d_n10, assign73810_body45_e111957_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1704 != 0.0)) && (locals.var_guard1714 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fs01, locals.var_fs01_dn0, locals.var_fs01_dn2, locals.var_fs01_dn4, locals.var_fs01_dn5, locals.var_fs01_dn6, locals.var_fs01_dn7, locals.var_fs01_dn8, locals.var_fs01_dn9, locals.var_fs01_dn10, locals.var_fs01_dn13,)
    }
};
            locals.var_fs01 = assign73810_body45_e111957;
            locals.var_fs01_dn0 = assign73810_body45_e111957_d_n0;
            locals.var_fs01_dn2 = assign73810_body45_e111957_d_n2;
            locals.var_fs01_dn4 = assign73810_body45_e111957_d_n4;
            locals.var_fs01_dn5 = assign73810_body45_e111957_d_n5;
            locals.var_fs01_dn6 = assign73810_body45_e111957_d_n6;
            locals.var_fs01_dn7 = assign73810_body45_e111957_d_n7;
            locals.var_fs01_dn8 = assign73810_body45_e111957_d_n8;
            locals.var_fs01_dn9 = assign73810_body45_e111957_d_n9;
            locals.var_fs01_dn10 = assign73810_body45_e111957_d_n10;
            locals.var_fs01_dn13 = assign73810_body45_e111957_d_n13;
            locals.var_fs01_rv = 0.0;
            let (assign73810_body46_e111968, assign73810_body46_e111968_d_n0, assign73810_body46_e111968_d_n2, assign73810_body46_e111968_d_n4, assign73810_body46_e111968_d_n5, assign73810_body46_e111968_d_n6, assign73810_body46_e111968_d_n7, assign73810_body46_e111968_d_n8, assign73810_body46_e111968_d_n9, assign73810_body46_e111968_d_n10, assign73810_body46_e111968_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1704 != 0.0)) && (locals.var_guard1714 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fs01_dps0, locals.var_fs01_dps0_dn0, locals.var_fs01_dps0_dn2, locals.var_fs01_dps0_dn4, locals.var_fs01_dps0_dn5, locals.var_fs01_dps0_dn6, locals.var_fs01_dps0_dn7, locals.var_fs01_dps0_dn8, locals.var_fs01_dps0_dn9, locals.var_fs01_dps0_dn10, locals.var_fs01_dps0_dn13,)
    }
};
            locals.var_fs01_dps0 = assign73810_body46_e111968;
            locals.var_fs01_dps0_dn0 = assign73810_body46_e111968_d_n0;
            locals.var_fs01_dps0_dn2 = assign73810_body46_e111968_d_n2;
            locals.var_fs01_dps0_dn4 = assign73810_body46_e111968_d_n4;
            locals.var_fs01_dps0_dn5 = assign73810_body46_e111968_d_n5;
            locals.var_fs01_dps0_dn6 = assign73810_body46_e111968_d_n6;
            locals.var_fs01_dps0_dn7 = assign73810_body46_e111968_d_n7;
            locals.var_fs01_dps0_dn8 = assign73810_body46_e111968_d_n8;
            locals.var_fs01_dps0_dn9 = assign73810_body46_e111968_d_n9;
            locals.var_fs01_dps0_dn10 = assign73810_body46_e111968_d_n10;
            locals.var_fs01_dps0_dn13 = assign73810_body46_e111968_d_n13;
            locals.var_fs01_dps0_rv = 0.0;
            let (assign73810_body47_e111980, assign73810_body47_e111980_d_n0, assign73810_body47_e111980_d_n2, assign73810_body47_e111980_d_n4, assign73810_body47_e111980_d_n5, assign73810_body47_e111980_d_n6, assign73810_body47_e111980_d_n7, assign73810_body47_e111980_d_n8, assign73810_body47_e111980_d_n9, assign73810_body47_e111980_d_n10, assign73810_body47_e111980_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1704 != 0.0)) && (locals.var_guard1714 != 0.0)) {
        let assign73810_body47_e111978: f64 = (-locals.var_fb);
        (assign73810_body47_e111978, (-locals.var_fb_dn0), (-locals.var_fb_dn2), (-locals.var_fb_dn4), (-locals.var_fb_dn5), (-locals.var_fb_dn6), (-locals.var_fb_dn7), (-locals.var_fb_dn8), (-locals.var_fb_dn9), (-locals.var_fb_dn10), (-locals.var_fb_dn13),)
    } else {
        (locals.var_fs02, locals.var_fs02_dn0, locals.var_fs02_dn2, locals.var_fs02_dn4, locals.var_fs02_dn5, locals.var_fs02_dn6, locals.var_fs02_dn7, locals.var_fs02_dn8, locals.var_fs02_dn9, locals.var_fs02_dn10, locals.var_fs02_dn13,)
    }
};
            locals.var_fs02 = assign73810_body47_e111980;
            locals.var_fs02_dn0 = assign73810_body47_e111980_d_n0;
            locals.var_fs02_dn2 = assign73810_body47_e111980_d_n2;
            locals.var_fs02_dn4 = assign73810_body47_e111980_d_n4;
            locals.var_fs02_dn5 = assign73810_body47_e111980_d_n5;
            locals.var_fs02_dn6 = assign73810_body47_e111980_d_n6;
            locals.var_fs02_dn7 = assign73810_body47_e111980_d_n7;
            locals.var_fs02_dn8 = assign73810_body47_e111980_d_n8;
            locals.var_fs02_dn9 = assign73810_body47_e111980_d_n9;
            locals.var_fs02_dn10 = assign73810_body47_e111980_d_n10;
            locals.var_fs02_dn13 = assign73810_body47_e111980_d_n13;
            locals.var_fs02_rv = 0.0;
            let (assign73810_body48_e111992, assign73810_body48_e111992_d_n0, assign73810_body48_e111992_d_n2, assign73810_body48_e111992_d_n4, assign73810_body48_e111992_d_n5, assign73810_body48_e111992_d_n6, assign73810_body48_e111992_d_n7, assign73810_body48_e111992_d_n8, assign73810_body48_e111992_d_n9, assign73810_body48_e111992_d_n10, assign73810_body48_e111992_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1704 != 0.0)) && (locals.var_guard1714 != 0.0)) {
        let assign73810_body48_e111990: f64 = (-locals.var_fb_dpss);
        (assign73810_body48_e111990, (-locals.var_fb_dpss_dn0), (-locals.var_fb_dpss_dn2), (-locals.var_fb_dpss_dn4), (-locals.var_fb_dpss_dn5), (-locals.var_fb_dpss_dn6), (-locals.var_fb_dpss_dn7), (-locals.var_fb_dpss_dn8), (-locals.var_fb_dpss_dn9), (-locals.var_fb_dpss_dn10), (-locals.var_fb_dpss_dn13),)
    } else {
        (locals.var_fs02_dps0, locals.var_fs02_dps0_dn0, locals.var_fs02_dps0_dn2, locals.var_fs02_dps0_dn4, locals.var_fs02_dps0_dn5, locals.var_fs02_dps0_dn6, locals.var_fs02_dps0_dn7, locals.var_fs02_dps0_dn8, locals.var_fs02_dps0_dn9, locals.var_fs02_dps0_dn10, locals.var_fs02_dps0_dn13,)
    }
};
            locals.var_fs02_dps0 = assign73810_body48_e111992;
            locals.var_fs02_dps0_dn0 = assign73810_body48_e111992_d_n0;
            locals.var_fs02_dps0_dn2 = assign73810_body48_e111992_d_n2;
            locals.var_fs02_dps0_dn4 = assign73810_body48_e111992_d_n4;
            locals.var_fs02_dps0_dn5 = assign73810_body48_e111992_d_n5;
            locals.var_fs02_dps0_dn6 = assign73810_body48_e111992_d_n6;
            locals.var_fs02_dps0_dn7 = assign73810_body48_e111992_d_n7;
            locals.var_fs02_dps0_dn8 = assign73810_body48_e111992_d_n8;
            locals.var_fs02_dps0_dn9 = assign73810_body48_e111992_d_n9;
            locals.var_fs02_dps0_dn10 = assign73810_body48_e111992_d_n10;
            locals.var_fs02_dps0_dn13 = assign73810_body48_e111992_d_n13;
            locals.var_fs02_dps0_rv = 0.0;
            let assign73810_body49_e111995: f64 = if locals.var_chi < 60.0 { 1.0 } else { 0.0 };
            locals.var_guard1715 = assign73810_body49_e111995;
            locals.var_guard1715_rv = 0.0;
            let assign73810_body50_e111998: f64 = if locals.var_chi < 5e-5 { 1.0 } else { 0.0 };
            locals.var_guard1716 = assign73810_body50_e111998;
            locals.var_guard1716_rv = 0.0;
            let (assign73810_body51_e112036, assign73810_body51_e112036_d_n0, assign73810_body51_e112036_d_n2, assign73810_body51_e112036_d_n4, assign73810_body51_e112036_d_n5, assign73810_body51_e112036_d_n6, assign73810_body51_e112036_d_n7, assign73810_body51_e112036_d_n8, assign73810_body51_e112036_d_n9, assign73810_body51_e112036_d_n10, assign73810_body51_e112036_d_n13,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1704 != 0.0)) && (locals.var_guard1714 == 0.0)) && (locals.var_guard1715 != 0.0)) && (locals.var_guard1716 != 0.0)) {
        let assign73810_body51_e112014: f64 = (locals.var_chi * locals.var_chi);
        let assign73810_body51_e112016: f64 = (assign73810_body51_e112014 / 2.0);
        let assign73810_body51_e112020: f64 = (locals.var_chi / 3.0);
        let assign73810_body51_e112024: f64 = (locals.var_chi / 4.0);
        let assign73810_body51_e112028: f64 = (locals.var_chi / 5.0);
        let assign73810_body51_e112029: f64 = (1.0 + assign73810_body51_e112028);
        let assign73810_body51_e112030: f64 = (assign73810_body51_e112024 * assign73810_body51_e112029);
        let assign73810_body51_e112031: f64 = (1.0 + assign73810_body51_e112030);
        let assign73810_body51_e112032: f64 = (assign73810_body51_e112020 * assign73810_body51_e112031);
        let assign73810_body51_e112033: f64 = (1.0 + assign73810_body51_e112032);
        let assign73810_body51_e112034: f64 = (assign73810_body51_e112016 * assign73810_body51_e112033);
        (assign73810_body51_e112034, (((((locals.var_chi_dn0 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn0)) / 2.0) * assign73810_body51_e112033) + (assign73810_body51_e112016 * (((locals.var_chi_dn0 / 3.0) * assign73810_body51_e112031) + (assign73810_body51_e112020 * (((locals.var_chi_dn0 / 4.0) * assign73810_body51_e112029) + (assign73810_body51_e112024 * (locals.var_chi_dn0 / 5.0))))))), (((((locals.var_chi_dn2 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn2)) / 2.0) * assign73810_body51_e112033) + (assign73810_body51_e112016 * (((locals.var_chi_dn2 / 3.0) * assign73810_body51_e112031) + (assign73810_body51_e112020 * (((locals.var_chi_dn2 / 4.0) * assign73810_body51_e112029) + (assign73810_body51_e112024 * (locals.var_chi_dn2 / 5.0))))))), (((((locals.var_chi_dn4 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn4)) / 2.0) * assign73810_body51_e112033) + (assign73810_body51_e112016 * (((locals.var_chi_dn4 / 3.0) * assign73810_body51_e112031) + (assign73810_body51_e112020 * (((locals.var_chi_dn4 / 4.0) * assign73810_body51_e112029) + (assign73810_body51_e112024 * (locals.var_chi_dn4 / 5.0))))))), (((((locals.var_chi_dn5 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn5)) / 2.0) * assign73810_body51_e112033) + (assign73810_body51_e112016 * (((locals.var_chi_dn5 / 3.0) * assign73810_body51_e112031) + (assign73810_body51_e112020 * (((locals.var_chi_dn5 / 4.0) * assign73810_body51_e112029) + (assign73810_body51_e112024 * (locals.var_chi_dn5 / 5.0))))))), (((((locals.var_chi_dn6 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn6)) / 2.0) * assign73810_body51_e112033) + (assign73810_body51_e112016 * (((locals.var_chi_dn6 / 3.0) * assign73810_body51_e112031) + (assign73810_body51_e112020 * (((locals.var_chi_dn6 / 4.0) * assign73810_body51_e112029) + (assign73810_body51_e112024 * (locals.var_chi_dn6 / 5.0))))))), (((((locals.var_chi_dn7 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn7)) / 2.0) * assign73810_body51_e112033) + (assign73810_body51_e112016 * (((locals.var_chi_dn7 / 3.0) * assign73810_body51_e112031) + (assign73810_body51_e112020 * (((locals.var_chi_dn7 / 4.0) * assign73810_body51_e112029) + (assign73810_body51_e112024 * (locals.var_chi_dn7 / 5.0))))))), (((((locals.var_chi_dn8 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn8)) / 2.0) * assign73810_body51_e112033) + (assign73810_body51_e112016 * (((locals.var_chi_dn8 / 3.0) * assign73810_body51_e112031) + (assign73810_body51_e112020 * (((locals.var_chi_dn8 / 4.0) * assign73810_body51_e112029) + (assign73810_body51_e112024 * (locals.var_chi_dn8 / 5.0))))))), (((((locals.var_chi_dn9 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn9)) / 2.0) * assign73810_body51_e112033) + (assign73810_body51_e112016 * (((locals.var_chi_dn9 / 3.0) * assign73810_body51_e112031) + (assign73810_body51_e112020 * (((locals.var_chi_dn9 / 4.0) * assign73810_body51_e112029) + (assign73810_body51_e112024 * (locals.var_chi_dn9 / 5.0))))))), (((((locals.var_chi_dn10 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn10)) / 2.0) * assign73810_body51_e112033) + (assign73810_body51_e112016 * (((locals.var_chi_dn10 / 3.0) * assign73810_body51_e112031) + (assign73810_body51_e112020 * (((locals.var_chi_dn10 / 4.0) * assign73810_body51_e112029) + (assign73810_body51_e112024 * (locals.var_chi_dn10 / 5.0))))))), (((((locals.var_chi_dn13 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn13)) / 2.0) * assign73810_body51_e112033) + (assign73810_body51_e112016 * (((locals.var_chi_dn13 / 3.0) * assign73810_body51_e112031) + (assign73810_body51_e112020 * (((locals.var_chi_dn13 / 4.0) * assign73810_body51_e112029) + (assign73810_body51_e112024 * (locals.var_chi_dn13 / 5.0))))))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
            locals.var_t0 = assign73810_body51_e112036;
            locals.var_t0_dn0 = assign73810_body51_e112036_d_n0;
            locals.var_t0_dn2 = assign73810_body51_e112036_d_n2;
            locals.var_t0_dn4 = assign73810_body51_e112036_d_n4;
            locals.var_t0_dn5 = assign73810_body51_e112036_d_n5;
            locals.var_t0_dn6 = assign73810_body51_e112036_d_n6;
            locals.var_t0_dn7 = assign73810_body51_e112036_d_n7;
            locals.var_t0_dn8 = assign73810_body51_e112036_d_n8;
            locals.var_t0_dn9 = assign73810_body51_e112036_d_n9;
            locals.var_t0_dn10 = assign73810_body51_e112036_d_n10;
            locals.var_t0_dn13 = assign73810_body51_e112036_d_n13;
            locals.var_t0_rv = 0.0;
            let (assign73810_body52_e112070, assign73810_body52_e112070_d_n0, assign73810_body52_e112070_d_n2, assign73810_body52_e112070_d_n4, assign73810_body52_e112070_d_n5, assign73810_body52_e112070_d_n6, assign73810_body52_e112070_d_n7, assign73810_body52_e112070_d_n8, assign73810_body52_e112070_d_n9, assign73810_body52_e112070_d_n10, assign73810_body52_e112070_d_n13,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1704 != 0.0)) && (locals.var_guard1714 == 0.0)) && (locals.var_guard1715 != 0.0)) && (locals.var_guard1716 != 0.0)) {
        let assign73810_body52_e112054: f64 = (locals.var_chi / 2.0);
        let assign73810_body52_e112058: f64 = (locals.var_chi / 3.0);
        let assign73810_body52_e112062: f64 = (locals.var_chi / 4.0);
        let assign73810_body52_e112063: f64 = (1.0 + assign73810_body52_e112062);
        let assign73810_body52_e112064: f64 = (assign73810_body52_e112058 * assign73810_body52_e112063);
        let assign73810_body52_e112065: f64 = (1.0 + assign73810_body52_e112064);
        let assign73810_body52_e112066: f64 = (assign73810_body52_e112054 * assign73810_body52_e112065);
        let assign73810_body52_e112067: f64 = (1.0 + assign73810_body52_e112066);
        let assign73810_body52_e112068: f64 = (locals.var_chi * assign73810_body52_e112067);
        (assign73810_body52_e112068, ((locals.var_chi_dn0 * assign73810_body52_e112067) + (locals.var_chi * (((locals.var_chi_dn0 / 2.0) * assign73810_body52_e112065) + (assign73810_body52_e112054 * (((locals.var_chi_dn0 / 3.0) * assign73810_body52_e112063) + (assign73810_body52_e112058 * (locals.var_chi_dn0 / 4.0))))))), ((locals.var_chi_dn2 * assign73810_body52_e112067) + (locals.var_chi * (((locals.var_chi_dn2 / 2.0) * assign73810_body52_e112065) + (assign73810_body52_e112054 * (((locals.var_chi_dn2 / 3.0) * assign73810_body52_e112063) + (assign73810_body52_e112058 * (locals.var_chi_dn2 / 4.0))))))), ((locals.var_chi_dn4 * assign73810_body52_e112067) + (locals.var_chi * (((locals.var_chi_dn4 / 2.0) * assign73810_body52_e112065) + (assign73810_body52_e112054 * (((locals.var_chi_dn4 / 3.0) * assign73810_body52_e112063) + (assign73810_body52_e112058 * (locals.var_chi_dn4 / 4.0))))))), ((locals.var_chi_dn5 * assign73810_body52_e112067) + (locals.var_chi * (((locals.var_chi_dn5 / 2.0) * assign73810_body52_e112065) + (assign73810_body52_e112054 * (((locals.var_chi_dn5 / 3.0) * assign73810_body52_e112063) + (assign73810_body52_e112058 * (locals.var_chi_dn5 / 4.0))))))), ((locals.var_chi_dn6 * assign73810_body52_e112067) + (locals.var_chi * (((locals.var_chi_dn6 / 2.0) * assign73810_body52_e112065) + (assign73810_body52_e112054 * (((locals.var_chi_dn6 / 3.0) * assign73810_body52_e112063) + (assign73810_body52_e112058 * (locals.var_chi_dn6 / 4.0))))))), ((locals.var_chi_dn7 * assign73810_body52_e112067) + (locals.var_chi * (((locals.var_chi_dn7 / 2.0) * assign73810_body52_e112065) + (assign73810_body52_e112054 * (((locals.var_chi_dn7 / 3.0) * assign73810_body52_e112063) + (assign73810_body52_e112058 * (locals.var_chi_dn7 / 4.0))))))), ((locals.var_chi_dn8 * assign73810_body52_e112067) + (locals.var_chi * (((locals.var_chi_dn8 / 2.0) * assign73810_body52_e112065) + (assign73810_body52_e112054 * (((locals.var_chi_dn8 / 3.0) * assign73810_body52_e112063) + (assign73810_body52_e112058 * (locals.var_chi_dn8 / 4.0))))))), ((locals.var_chi_dn9 * assign73810_body52_e112067) + (locals.var_chi * (((locals.var_chi_dn9 / 2.0) * assign73810_body52_e112065) + (assign73810_body52_e112054 * (((locals.var_chi_dn9 / 3.0) * assign73810_body52_e112063) + (assign73810_body52_e112058 * (locals.var_chi_dn9 / 4.0))))))), ((locals.var_chi_dn10 * assign73810_body52_e112067) + (locals.var_chi * (((locals.var_chi_dn10 / 2.0) * assign73810_body52_e112065) + (assign73810_body52_e112054 * (((locals.var_chi_dn10 / 3.0) * assign73810_body52_e112063) + (assign73810_body52_e112058 * (locals.var_chi_dn10 / 4.0))))))), ((locals.var_chi_dn13 * assign73810_body52_e112067) + (locals.var_chi * (((locals.var_chi_dn13 / 2.0) * assign73810_body52_e112065) + (assign73810_body52_e112054 * (((locals.var_chi_dn13 / 3.0) * assign73810_body52_e112063) + (assign73810_body52_e112058 * (locals.var_chi_dn13 / 4.0))))))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
            locals.var_t1 = assign73810_body52_e112070;
            locals.var_t1_dn0 = assign73810_body52_e112070_d_n0;
            locals.var_t1_dn2 = assign73810_body52_e112070_d_n2;
            locals.var_t1_dn4 = assign73810_body52_e112070_d_n4;
            locals.var_t1_dn5 = assign73810_body52_e112070_d_n5;
            locals.var_t1_dn6 = assign73810_body52_e112070_d_n6;
            locals.var_t1_dn7 = assign73810_body52_e112070_d_n7;
            locals.var_t1_dn8 = assign73810_body52_e112070_d_n8;
            locals.var_t1_dn9 = assign73810_body52_e112070_d_n9;
            locals.var_t1_dn10 = assign73810_body52_e112070_d_n10;
            locals.var_t1_dn13 = assign73810_body52_e112070_d_n13;
            locals.var_t1_rv = 0.0;
            let (assign73810_body53_e112088, assign73810_body53_e112088_d_n0, assign73810_body53_e112088_d_n2, assign73810_body53_e112088_d_n4, assign73810_body53_e112088_d_n5, assign73810_body53_e112088_d_n6, assign73810_body53_e112088_d_n7, assign73810_body53_e112088_d_n8, assign73810_body53_e112088_d_n9, assign73810_body53_e112088_d_n10, assign73810_body53_e112088_d_n13,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1704 != 0.0)) && (locals.var_guard1714 == 0.0)) && (locals.var_guard1715 != 0.0)) && (locals.var_guard1716 != 0.0)) {
        let assign73810_body53_e112086: f64 = (locals.var_cfs1 * locals.var_t0);
        (assign73810_body53_e112086, ((locals.var_cfs1_dn0 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn0)), ((locals.var_cfs1_dn2 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn2)), ((locals.var_cfs1_dn4 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn4)), ((locals.var_cfs1_dn5 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn5)), ((locals.var_cfs1_dn6 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn6)), ((locals.var_cfs1_dn7 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn7)), ((locals.var_cfs1_dn8 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn8)), ((locals.var_cfs1_dn9 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn9)), ((locals.var_cfs1_dn10 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn10)), ((locals.var_cfs1_dn13 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn13)),)
    } else {
        (locals.var_fs01, locals.var_fs01_dn0, locals.var_fs01_dn2, locals.var_fs01_dn4, locals.var_fs01_dn5, locals.var_fs01_dn6, locals.var_fs01_dn7, locals.var_fs01_dn8, locals.var_fs01_dn9, locals.var_fs01_dn10, locals.var_fs01_dn13,)
    }
};
            locals.var_fs01 = assign73810_body53_e112088;
            locals.var_fs01_dn0 = assign73810_body53_e112088_d_n0;
            locals.var_fs01_dn2 = assign73810_body53_e112088_d_n2;
            locals.var_fs01_dn4 = assign73810_body53_e112088_d_n4;
            locals.var_fs01_dn5 = assign73810_body53_e112088_d_n5;
            locals.var_fs01_dn6 = assign73810_body53_e112088_d_n6;
            locals.var_fs01_dn7 = assign73810_body53_e112088_d_n7;
            locals.var_fs01_dn8 = assign73810_body53_e112088_d_n8;
            locals.var_fs01_dn9 = assign73810_body53_e112088_d_n9;
            locals.var_fs01_dn10 = assign73810_body53_e112088_d_n10;
            locals.var_fs01_dn13 = assign73810_body53_e112088_d_n13;
            locals.var_fs01_rv = 0.0;
            let (assign73810_body54_e112108, assign73810_body54_e112108_d_n0, assign73810_body54_e112108_d_n2, assign73810_body54_e112108_d_n4, assign73810_body54_e112108_d_n5, assign73810_body54_e112108_d_n6, assign73810_body54_e112108_d_n7, assign73810_body54_e112108_d_n8, assign73810_body54_e112108_d_n9, assign73810_body54_e112108_d_n10, assign73810_body54_e112108_d_n13,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1704 != 0.0)) && (locals.var_guard1714 == 0.0)) && (locals.var_guard1715 != 0.0)) && (locals.var_guard1716 != 0.0)) {
        let assign73810_body54_e112104: f64 = (locals.var_cfs1 * locals.var_t1);
        let assign73810_body54_e112106: f64 = (assign73810_body54_e112104 * locals.var_beta);
        (assign73810_body54_e112106, ((((locals.var_cfs1_dn0 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn0)) * locals.var_beta) + (assign73810_body54_e112104 * locals.var_beta_dn0)), ((((locals.var_cfs1_dn2 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn2)) * locals.var_beta) + (assign73810_body54_e112104 * locals.var_beta_dn2)), ((((locals.var_cfs1_dn4 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn4)) * locals.var_beta) + (assign73810_body54_e112104 * locals.var_beta_dn4)), ((((locals.var_cfs1_dn5 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn5)) * locals.var_beta) + (assign73810_body54_e112104 * locals.var_beta_dn5)), ((((locals.var_cfs1_dn6 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn6)) * locals.var_beta) + (assign73810_body54_e112104 * locals.var_beta_dn6)), ((((locals.var_cfs1_dn7 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn7)) * locals.var_beta) + (assign73810_body54_e112104 * locals.var_beta_dn7)), ((((locals.var_cfs1_dn8 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn8)) * locals.var_beta) + (assign73810_body54_e112104 * locals.var_beta_dn8)), ((((locals.var_cfs1_dn9 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn9)) * locals.var_beta) + (assign73810_body54_e112104 * locals.var_beta_dn9)), ((((locals.var_cfs1_dn10 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn10)) * locals.var_beta) + (assign73810_body54_e112104 * locals.var_beta_dn10)), ((((locals.var_cfs1_dn13 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn13)) * locals.var_beta) + (assign73810_body54_e112104 * locals.var_beta_dn13)),)
    } else {
        (locals.var_fs01_dps0, locals.var_fs01_dps0_dn0, locals.var_fs01_dps0_dn2, locals.var_fs01_dps0_dn4, locals.var_fs01_dps0_dn5, locals.var_fs01_dps0_dn6, locals.var_fs01_dps0_dn7, locals.var_fs01_dps0_dn8, locals.var_fs01_dps0_dn9, locals.var_fs01_dps0_dn10, locals.var_fs01_dps0_dn13,)
    }
};
            locals.var_fs01_dps0 = assign73810_body54_e112108;
            locals.var_fs01_dps0_dn0 = assign73810_body54_e112108_d_n0;
            locals.var_fs01_dps0_dn2 = assign73810_body54_e112108_d_n2;
            locals.var_fs01_dps0_dn4 = assign73810_body54_e112108_d_n4;
            locals.var_fs01_dps0_dn5 = assign73810_body54_e112108_d_n5;
            locals.var_fs01_dps0_dn6 = assign73810_body54_e112108_d_n6;
            locals.var_fs01_dps0_dn7 = assign73810_body54_e112108_d_n7;
            locals.var_fs01_dps0_dn8 = assign73810_body54_e112108_d_n8;
            locals.var_fs01_dps0_dn9 = assign73810_body54_e112108_d_n9;
            locals.var_fs01_dps0_dn10 = assign73810_body54_e112108_d_n10;
            locals.var_fs01_dps0_dn13 = assign73810_body54_e112108_d_n13;
            locals.var_fs01_dps0_rv = 0.0;
            let (assign73810_body55_e112126, assign73810_body55_e112126_d_n0, assign73810_body55_e112126_d_n2, assign73810_body55_e112126_d_n4, assign73810_body55_e112126_d_n5, assign73810_body55_e112126_d_n6, assign73810_body55_e112126_d_n7, assign73810_body55_e112126_d_n8, assign73810_body55_e112126_d_n9, assign73810_body55_e112126_d_n10, assign73810_body55_e112126_d_n13,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1704 != 0.0)) && (locals.var_guard1714 == 0.0)) && (locals.var_guard1715 != 0.0)) && (locals.var_guard1716 == 0.0)) {
        let assign73810_body55_e112124: f64 = (locals.var_chi).exp();
        (assign73810_body55_e112124, (assign73810_body55_e112124 * locals.var_chi_dn0), (assign73810_body55_e112124 * locals.var_chi_dn2), (assign73810_body55_e112124 * locals.var_chi_dn4), (assign73810_body55_e112124 * locals.var_chi_dn5), (assign73810_body55_e112124 * locals.var_chi_dn6), (assign73810_body55_e112124 * locals.var_chi_dn7), (assign73810_body55_e112124 * locals.var_chi_dn8), (assign73810_body55_e112124 * locals.var_chi_dn9), (assign73810_body55_e112124 * locals.var_chi_dn10), (assign73810_body55_e112124 * locals.var_chi_dn13),)
    } else {
        (locals.var_exp_chi, locals.var_exp_chi_dn0, locals.var_exp_chi_dn2, locals.var_exp_chi_dn4, locals.var_exp_chi_dn5, locals.var_exp_chi_dn6, locals.var_exp_chi_dn7, locals.var_exp_chi_dn8, locals.var_exp_chi_dn9, locals.var_exp_chi_dn10, locals.var_exp_chi_dn13,)
    }
};
            locals.var_exp_chi = assign73810_body55_e112126;
            locals.var_exp_chi_dn0 = assign73810_body55_e112126_d_n0;
            locals.var_exp_chi_dn2 = assign73810_body55_e112126_d_n2;
            locals.var_exp_chi_dn4 = assign73810_body55_e112126_d_n4;
            locals.var_exp_chi_dn5 = assign73810_body55_e112126_d_n5;
            locals.var_exp_chi_dn6 = assign73810_body55_e112126_d_n6;
            locals.var_exp_chi_dn7 = assign73810_body55_e112126_d_n7;
            locals.var_exp_chi_dn8 = assign73810_body55_e112126_d_n8;
            locals.var_exp_chi_dn9 = assign73810_body55_e112126_d_n9;
            locals.var_exp_chi_dn10 = assign73810_body55_e112126_d_n10;
            locals.var_exp_chi_dn13 = assign73810_body55_e112126_d_n13;
            locals.var_exp_chi_rv = 0.0;
            let (assign73810_body56_e112145, assign73810_body56_e112145_d_n0, assign73810_body56_e112145_d_n2, assign73810_body56_e112145_d_n4, assign73810_body56_e112145_d_n5, assign73810_body56_e112145_d_n6, assign73810_body56_e112145_d_n7, assign73810_body56_e112145_d_n8, assign73810_body56_e112145_d_n9, assign73810_body56_e112145_d_n10, assign73810_body56_e112145_d_n13,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1704 != 0.0)) && (locals.var_guard1714 == 0.0)) && (locals.var_guard1715 != 0.0)) && (locals.var_guard1716 == 0.0)) {
        let assign73810_body56_e112143: f64 = (locals.var_exp_chi - 1.0);
        (assign73810_body56_e112143, locals.var_exp_chi_dn0, locals.var_exp_chi_dn2, locals.var_exp_chi_dn4, locals.var_exp_chi_dn5, locals.var_exp_chi_dn6, locals.var_exp_chi_dn7, locals.var_exp_chi_dn8, locals.var_exp_chi_dn9, locals.var_exp_chi_dn10, locals.var_exp_chi_dn13,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
            locals.var_t1 = assign73810_body56_e112145;
            locals.var_t1_dn0 = assign73810_body56_e112145_d_n0;
            locals.var_t1_dn2 = assign73810_body56_e112145_d_n2;
            locals.var_t1_dn4 = assign73810_body56_e112145_d_n4;
            locals.var_t1_dn5 = assign73810_body56_e112145_d_n5;
            locals.var_t1_dn6 = assign73810_body56_e112145_d_n6;
            locals.var_t1_dn7 = assign73810_body56_e112145_d_n7;
            locals.var_t1_dn8 = assign73810_body56_e112145_d_n8;
            locals.var_t1_dn9 = assign73810_body56_e112145_d_n9;
            locals.var_t1_dn10 = assign73810_body56_e112145_d_n10;
            locals.var_t1_dn13 = assign73810_body56_e112145_d_n13;
            locals.var_t1_rv = 0.0;
            let (assign73810_body57_e112166, assign73810_body57_e112166_d_n0, assign73810_body57_e112166_d_n2, assign73810_body57_e112166_d_n4, assign73810_body57_e112166_d_n5, assign73810_body57_e112166_d_n6, assign73810_body57_e112166_d_n7, assign73810_body57_e112166_d_n8, assign73810_body57_e112166_d_n9, assign73810_body57_e112166_d_n10, assign73810_body57_e112166_d_n13,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1704 != 0.0)) && (locals.var_guard1714 == 0.0)) && (locals.var_guard1715 != 0.0)) && (locals.var_guard1716 == 0.0)) {
        let assign73810_body57_e112163: f64 = (locals.var_t1 - locals.var_chi);
        let assign73810_body57_e112164: f64 = (locals.var_cfs1 * assign73810_body57_e112163);
        (assign73810_body57_e112164, ((locals.var_cfs1_dn0 * assign73810_body57_e112163) + (locals.var_cfs1 * (locals.var_t1_dn0 - locals.var_chi_dn0))), ((locals.var_cfs1_dn2 * assign73810_body57_e112163) + (locals.var_cfs1 * (locals.var_t1_dn2 - locals.var_chi_dn2))), ((locals.var_cfs1_dn4 * assign73810_body57_e112163) + (locals.var_cfs1 * (locals.var_t1_dn4 - locals.var_chi_dn4))), ((locals.var_cfs1_dn5 * assign73810_body57_e112163) + (locals.var_cfs1 * (locals.var_t1_dn5 - locals.var_chi_dn5))), ((locals.var_cfs1_dn6 * assign73810_body57_e112163) + (locals.var_cfs1 * (locals.var_t1_dn6 - locals.var_chi_dn6))), ((locals.var_cfs1_dn7 * assign73810_body57_e112163) + (locals.var_cfs1 * (locals.var_t1_dn7 - locals.var_chi_dn7))), ((locals.var_cfs1_dn8 * assign73810_body57_e112163) + (locals.var_cfs1 * (locals.var_t1_dn8 - locals.var_chi_dn8))), ((locals.var_cfs1_dn9 * assign73810_body57_e112163) + (locals.var_cfs1 * (locals.var_t1_dn9 - locals.var_chi_dn9))), ((locals.var_cfs1_dn10 * assign73810_body57_e112163) + (locals.var_cfs1 * (locals.var_t1_dn10 - locals.var_chi_dn10))), ((locals.var_cfs1_dn13 * assign73810_body57_e112163) + (locals.var_cfs1 * (locals.var_t1_dn13 - locals.var_chi_dn13))),)
    } else {
        (locals.var_fs01, locals.var_fs01_dn0, locals.var_fs01_dn2, locals.var_fs01_dn4, locals.var_fs01_dn5, locals.var_fs01_dn6, locals.var_fs01_dn7, locals.var_fs01_dn8, locals.var_fs01_dn9, locals.var_fs01_dn10, locals.var_fs01_dn13,)
    }
};
            locals.var_fs01 = assign73810_body57_e112166;
            locals.var_fs01_dn0 = assign73810_body57_e112166_d_n0;
            locals.var_fs01_dn2 = assign73810_body57_e112166_d_n2;
            locals.var_fs01_dn4 = assign73810_body57_e112166_d_n4;
            locals.var_fs01_dn5 = assign73810_body57_e112166_d_n5;
            locals.var_fs01_dn6 = assign73810_body57_e112166_d_n6;
            locals.var_fs01_dn7 = assign73810_body57_e112166_d_n7;
            locals.var_fs01_dn8 = assign73810_body57_e112166_d_n8;
            locals.var_fs01_dn9 = assign73810_body57_e112166_d_n9;
            locals.var_fs01_dn10 = assign73810_body57_e112166_d_n10;
            locals.var_fs01_dn13 = assign73810_body57_e112166_d_n13;
            locals.var_fs01_rv = 0.0;
            let (assign73810_body58_e112187, assign73810_body58_e112187_d_n0, assign73810_body58_e112187_d_n2, assign73810_body58_e112187_d_n4, assign73810_body58_e112187_d_n5, assign73810_body58_e112187_d_n6, assign73810_body58_e112187_d_n7, assign73810_body58_e112187_d_n8, assign73810_body58_e112187_d_n9, assign73810_body58_e112187_d_n10, assign73810_body58_e112187_d_n13,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1704 != 0.0)) && (locals.var_guard1714 == 0.0)) && (locals.var_guard1715 != 0.0)) && (locals.var_guard1716 == 0.0)) {
        let assign73810_body58_e112183: f64 = (locals.var_cfs1 * locals.var_beta);
        let assign73810_body58_e112185: f64 = (assign73810_body58_e112183 * locals.var_t1);
        (assign73810_body58_e112185, ((((locals.var_cfs1_dn0 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn0)) * locals.var_t1) + (assign73810_body58_e112183 * locals.var_t1_dn0)), ((((locals.var_cfs1_dn2 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn2)) * locals.var_t1) + (assign73810_body58_e112183 * locals.var_t1_dn2)), ((((locals.var_cfs1_dn4 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn4)) * locals.var_t1) + (assign73810_body58_e112183 * locals.var_t1_dn4)), ((((locals.var_cfs1_dn5 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn5)) * locals.var_t1) + (assign73810_body58_e112183 * locals.var_t1_dn5)), ((((locals.var_cfs1_dn6 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn6)) * locals.var_t1) + (assign73810_body58_e112183 * locals.var_t1_dn6)), ((((locals.var_cfs1_dn7 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn7)) * locals.var_t1) + (assign73810_body58_e112183 * locals.var_t1_dn7)), ((((locals.var_cfs1_dn8 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn8)) * locals.var_t1) + (assign73810_body58_e112183 * locals.var_t1_dn8)), ((((locals.var_cfs1_dn9 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn9)) * locals.var_t1) + (assign73810_body58_e112183 * locals.var_t1_dn9)), ((((locals.var_cfs1_dn10 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn10)) * locals.var_t1) + (assign73810_body58_e112183 * locals.var_t1_dn10)), ((((locals.var_cfs1_dn13 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn13)) * locals.var_t1) + (assign73810_body58_e112183 * locals.var_t1_dn13)),)
    } else {
        (locals.var_fs01_dps0, locals.var_fs01_dps0_dn0, locals.var_fs01_dps0_dn2, locals.var_fs01_dps0_dn4, locals.var_fs01_dps0_dn5, locals.var_fs01_dps0_dn6, locals.var_fs01_dps0_dn7, locals.var_fs01_dps0_dn8, locals.var_fs01_dps0_dn9, locals.var_fs01_dps0_dn10, locals.var_fs01_dps0_dn13,)
    }
};
            locals.var_fs01_dps0 = assign73810_body58_e112187;
            locals.var_fs01_dps0_dn0 = assign73810_body58_e112187_d_n0;
            locals.var_fs01_dps0_dn2 = assign73810_body58_e112187_d_n2;
            locals.var_fs01_dps0_dn4 = assign73810_body58_e112187_d_n4;
            locals.var_fs01_dps0_dn5 = assign73810_body58_e112187_d_n5;
            locals.var_fs01_dps0_dn6 = assign73810_body58_e112187_d_n6;
            locals.var_fs01_dps0_dn7 = assign73810_body58_e112187_d_n7;
            locals.var_fs01_dps0_dn8 = assign73810_body58_e112187_d_n8;
            locals.var_fs01_dps0_dn9 = assign73810_body58_e112187_d_n9;
            locals.var_fs01_dps0_dn10 = assign73810_body58_e112187_d_n10;
            locals.var_fs01_dps0_dn13 = assign73810_body58_e112187_d_n13;
            locals.var_fs01_dps0_rv = 0.0;
            let (assign73810_body60_e112222, assign73810_body60_e112222_d_n0, assign73810_body60_e112222_d_n2, assign73810_body60_e112222_d_n4, assign73810_body60_e112222_d_n5, assign73810_body60_e112222_d_n6, assign73810_body60_e112222_d_n7, assign73810_body60_e112222_d_n8, assign73810_body60_e112222_d_n9, assign73810_body60_e112222_d_n10, assign73810_body60_e112222_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1704 != 0.0)) && (locals.var_guard1714 == 0.0)) && (locals.var_guard1715 == 0.0)) {
        let assign73810_body60_e112219: f64 = (locals.var_beta * locals.var_ps0ld);
        let assign73810_body60_e112220: f64 = (assign73810_body60_e112219).exp();
        (assign73810_body60_e112220, (assign73810_body60_e112220 * ((locals.var_beta_dn0 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn0))), (assign73810_body60_e112220 * ((locals.var_beta_dn2 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn2))), (assign73810_body60_e112220 * ((locals.var_beta_dn4 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn4))), (assign73810_body60_e112220 * ((locals.var_beta_dn5 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn5))), (assign73810_body60_e112220 * ((locals.var_beta_dn6 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn6))), (assign73810_body60_e112220 * ((locals.var_beta_dn7 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn7))), (assign73810_body60_e112220 * ((locals.var_beta_dn8 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn8))), (assign73810_body60_e112220 * ((locals.var_beta_dn9 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn9))), (assign73810_body60_e112220 * ((locals.var_beta_dn10 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn10))), (assign73810_body60_e112220 * ((locals.var_beta_dn13 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn13))),)
    } else {
        (locals.var_exp_bps0, locals.var_exp_bps0_dn0, locals.var_exp_bps0_dn2, locals.var_exp_bps0_dn4, locals.var_exp_bps0_dn5, locals.var_exp_bps0_dn6, locals.var_exp_bps0_dn7, locals.var_exp_bps0_dn8, locals.var_exp_bps0_dn9, locals.var_exp_bps0_dn10, locals.var_exp_bps0_dn13,)
    }
};
            locals.var_exp_bps0 = assign73810_body60_e112222;
            locals.var_exp_bps0_dn0 = assign73810_body60_e112222_d_n0;
            locals.var_exp_bps0_dn2 = assign73810_body60_e112222_d_n2;
            locals.var_exp_bps0_dn4 = assign73810_body60_e112222_d_n4;
            locals.var_exp_bps0_dn5 = assign73810_body60_e112222_d_n5;
            locals.var_exp_bps0_dn6 = assign73810_body60_e112222_d_n6;
            locals.var_exp_bps0_dn7 = assign73810_body60_e112222_d_n7;
            locals.var_exp_bps0_dn8 = assign73810_body60_e112222_d_n8;
            locals.var_exp_bps0_dn9 = assign73810_body60_e112222_d_n9;
            locals.var_exp_bps0_dn10 = assign73810_body60_e112222_d_n10;
            locals.var_exp_bps0_dn13 = assign73810_body60_e112222_d_n13;
            locals.var_exp_bps0_rv = 0.0;
            let (assign73810_body61_e112245, assign73810_body61_e112245_d_n0, assign73810_body61_e112245_d_n2, assign73810_body61_e112245_d_n4, assign73810_body61_e112245_d_n5, assign73810_body61_e112245_d_n6, assign73810_body61_e112245_d_n7, assign73810_body61_e112245_d_n8, assign73810_body61_e112245_d_n9, assign73810_body61_e112245_d_n10, assign73810_body61_e112245_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1704 != 0.0)) && (locals.var_guard1714 == 0.0)) && (locals.var_guard1715 == 0.0)) {
        let assign73810_body61_e112240: f64 = (locals.var_chi + 1.0);
        let assign73810_body61_e112241: f64 = (locals.var_exp_bvbs * assign73810_body61_e112240);
        let assign73810_body61_e112242: f64 = (locals.var_exp_bps0 - assign73810_body61_e112241);
        let assign73810_body61_e112243: f64 = (locals.var_cnst1over * assign73810_body61_e112242);
        (assign73810_body61_e112243, ((locals.var_cnst1over_dn0 * assign73810_body61_e112242) + (locals.var_cnst1over * (locals.var_exp_bps0_dn0 - ((locals.var_exp_bvbs_dn0 * assign73810_body61_e112240) + (locals.var_exp_bvbs * locals.var_chi_dn0))))), ((locals.var_cnst1over_dn2 * assign73810_body61_e112242) + (locals.var_cnst1over * (locals.var_exp_bps0_dn2 - ((locals.var_exp_bvbs_dn2 * assign73810_body61_e112240) + (locals.var_exp_bvbs * locals.var_chi_dn2))))), ((locals.var_cnst1over_dn4 * assign73810_body61_e112242) + (locals.var_cnst1over * (locals.var_exp_bps0_dn4 - ((locals.var_exp_bvbs_dn4 * assign73810_body61_e112240) + (locals.var_exp_bvbs * locals.var_chi_dn4))))), ((locals.var_cnst1over_dn5 * assign73810_body61_e112242) + (locals.var_cnst1over * (locals.var_exp_bps0_dn5 - ((locals.var_exp_bvbs_dn5 * assign73810_body61_e112240) + (locals.var_exp_bvbs * locals.var_chi_dn5))))), ((locals.var_cnst1over_dn6 * assign73810_body61_e112242) + (locals.var_cnst1over * (locals.var_exp_bps0_dn6 - ((locals.var_exp_bvbs_dn6 * assign73810_body61_e112240) + (locals.var_exp_bvbs * locals.var_chi_dn6))))), ((locals.var_cnst1over_dn7 * assign73810_body61_e112242) + (locals.var_cnst1over * (locals.var_exp_bps0_dn7 - ((locals.var_exp_bvbs_dn7 * assign73810_body61_e112240) + (locals.var_exp_bvbs * locals.var_chi_dn7))))), ((locals.var_cnst1over_dn8 * assign73810_body61_e112242) + (locals.var_cnst1over * (locals.var_exp_bps0_dn8 - ((locals.var_exp_bvbs_dn8 * assign73810_body61_e112240) + (locals.var_exp_bvbs * locals.var_chi_dn8))))), ((locals.var_cnst1over_dn9 * assign73810_body61_e112242) + (locals.var_cnst1over * (locals.var_exp_bps0_dn9 - ((locals.var_exp_bvbs_dn9 * assign73810_body61_e112240) + (locals.var_exp_bvbs * locals.var_chi_dn9))))), ((locals.var_cnst1over_dn10 * assign73810_body61_e112242) + (locals.var_cnst1over * (locals.var_exp_bps0_dn10 - ((locals.var_exp_bvbs_dn10 * assign73810_body61_e112240) + (locals.var_exp_bvbs * locals.var_chi_dn10))))), ((locals.var_cnst1over_dn13 * assign73810_body61_e112242) + (locals.var_cnst1over * (locals.var_exp_bps0_dn13 - ((locals.var_exp_bvbs_dn13 * assign73810_body61_e112240) + (locals.var_exp_bvbs * locals.var_chi_dn13))))),)
    } else {
        (locals.var_fs01, locals.var_fs01_dn0, locals.var_fs01_dn2, locals.var_fs01_dn4, locals.var_fs01_dn5, locals.var_fs01_dn6, locals.var_fs01_dn7, locals.var_fs01_dn8, locals.var_fs01_dn9, locals.var_fs01_dn10, locals.var_fs01_dn13,)
    }
};
            locals.var_fs01 = assign73810_body61_e112245;
            locals.var_fs01_dn0 = assign73810_body61_e112245_d_n0;
            locals.var_fs01_dn2 = assign73810_body61_e112245_d_n2;
            locals.var_fs01_dn4 = assign73810_body61_e112245_d_n4;
            locals.var_fs01_dn5 = assign73810_body61_e112245_d_n5;
            locals.var_fs01_dn6 = assign73810_body61_e112245_d_n6;
            locals.var_fs01_dn7 = assign73810_body61_e112245_d_n7;
            locals.var_fs01_dn8 = assign73810_body61_e112245_d_n8;
            locals.var_fs01_dn9 = assign73810_body61_e112245_d_n9;
            locals.var_fs01_dn10 = assign73810_body61_e112245_d_n10;
            locals.var_fs01_dn13 = assign73810_body61_e112245_d_n13;
            locals.var_fs01_rv = 0.0;
            let (assign73810_body62_e112266, assign73810_body62_e112266_d_n0, assign73810_body62_e112266_d_n2, assign73810_body62_e112266_d_n4, assign73810_body62_e112266_d_n5, assign73810_body62_e112266_d_n6, assign73810_body62_e112266_d_n7, assign73810_body62_e112266_d_n8, assign73810_body62_e112266_d_n9, assign73810_body62_e112266_d_n10, assign73810_body62_e112266_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1704 != 0.0)) && (locals.var_guard1714 == 0.0)) && (locals.var_guard1715 == 0.0)) {
        let assign73810_body62_e112260: f64 = (locals.var_cnst1over * locals.var_beta);
        let assign73810_body62_e112263: f64 = (locals.var_exp_bps0 - locals.var_exp_bvbs);
        let assign73810_body62_e112264: f64 = (assign73810_body62_e112260 * assign73810_body62_e112263);
        (assign73810_body62_e112264, ((((locals.var_cnst1over_dn0 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn0)) * assign73810_body62_e112263) + (assign73810_body62_e112260 * (locals.var_exp_bps0_dn0 - locals.var_exp_bvbs_dn0))), ((((locals.var_cnst1over_dn2 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn2)) * assign73810_body62_e112263) + (assign73810_body62_e112260 * (locals.var_exp_bps0_dn2 - locals.var_exp_bvbs_dn2))), ((((locals.var_cnst1over_dn4 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn4)) * assign73810_body62_e112263) + (assign73810_body62_e112260 * (locals.var_exp_bps0_dn4 - locals.var_exp_bvbs_dn4))), ((((locals.var_cnst1over_dn5 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn5)) * assign73810_body62_e112263) + (assign73810_body62_e112260 * (locals.var_exp_bps0_dn5 - locals.var_exp_bvbs_dn5))), ((((locals.var_cnst1over_dn6 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn6)) * assign73810_body62_e112263) + (assign73810_body62_e112260 * (locals.var_exp_bps0_dn6 - locals.var_exp_bvbs_dn6))), ((((locals.var_cnst1over_dn7 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn7)) * assign73810_body62_e112263) + (assign73810_body62_e112260 * (locals.var_exp_bps0_dn7 - locals.var_exp_bvbs_dn7))), ((((locals.var_cnst1over_dn8 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn8)) * assign73810_body62_e112263) + (assign73810_body62_e112260 * (locals.var_exp_bps0_dn8 - locals.var_exp_bvbs_dn8))), ((((locals.var_cnst1over_dn9 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn9)) * assign73810_body62_e112263) + (assign73810_body62_e112260 * (locals.var_exp_bps0_dn9 - locals.var_exp_bvbs_dn9))), ((((locals.var_cnst1over_dn10 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn10)) * assign73810_body62_e112263) + (assign73810_body62_e112260 * (locals.var_exp_bps0_dn10 - locals.var_exp_bvbs_dn10))), ((((locals.var_cnst1over_dn13 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn13)) * assign73810_body62_e112263) + (assign73810_body62_e112260 * (locals.var_exp_bps0_dn13 - locals.var_exp_bvbs_dn13))),)
    } else {
        (locals.var_fs01_dps0, locals.var_fs01_dps0_dn0, locals.var_fs01_dps0_dn2, locals.var_fs01_dps0_dn4, locals.var_fs01_dps0_dn5, locals.var_fs01_dps0_dn6, locals.var_fs01_dps0_dn7, locals.var_fs01_dps0_dn8, locals.var_fs01_dps0_dn9, locals.var_fs01_dps0_dn10, locals.var_fs01_dps0_dn13,)
    }
};
            locals.var_fs01_dps0 = assign73810_body62_e112266;
            locals.var_fs01_dps0_dn0 = assign73810_body62_e112266_d_n0;
            locals.var_fs01_dps0_dn2 = assign73810_body62_e112266_d_n2;
            locals.var_fs01_dps0_dn4 = assign73810_body62_e112266_d_n4;
            locals.var_fs01_dps0_dn5 = assign73810_body62_e112266_d_n5;
            locals.var_fs01_dps0_dn6 = assign73810_body62_e112266_d_n6;
            locals.var_fs01_dps0_dn7 = assign73810_body62_e112266_d_n7;
            locals.var_fs01_dps0_dn8 = assign73810_body62_e112266_d_n8;
            locals.var_fs01_dps0_dn9 = assign73810_body62_e112266_d_n9;
            locals.var_fs01_dps0_dn10 = assign73810_body62_e112266_d_n10;
            locals.var_fs01_dps0_dn13 = assign73810_body62_e112266_d_n13;
            locals.var_fs01_dps0_rv = 0.0;
            let assign73810_body63_e112269: f64 = if locals.var_fs01 > 0.0 { 1.0 } else { 0.0 };
            locals.var_guard1717 = assign73810_body63_e112269;
            locals.var_guard1717_rv = 0.0;
            let (assign73810_body64_e112288, assign73810_body64_e112288_d_n0, assign73810_body64_e112288_d_n2, assign73810_body64_e112288_d_n4, assign73810_body64_e112288_d_n5, assign73810_body64_e112288_d_n6, assign73810_body64_e112288_d_n7, assign73810_body64_e112288_d_n8, assign73810_body64_e112288_d_n9, assign73810_body64_e112288_d_n10, assign73810_body64_e112288_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1704 != 0.0)) && (locals.var_guard1714 == 0.0)) && (locals.var_guard1717 != 0.0)) {
        let assign73810_body64_e112283: f64 = (locals.var_fb * locals.var_fb);
        let assign73810_body64_e112285: f64 = (assign73810_body64_e112283 + locals.var_fs01);
        let assign73810_body64_e112286: f64 = (assign73810_body64_e112285).sqrt();
        (assign73810_body64_e112286, ((((locals.var_fb_dn0 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn0)) + locals.var_fs01_dn0) / (2.0 * assign73810_body64_e112286)), ((((locals.var_fb_dn2 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn2)) + locals.var_fs01_dn2) / (2.0 * assign73810_body64_e112286)), ((((locals.var_fb_dn4 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn4)) + locals.var_fs01_dn4) / (2.0 * assign73810_body64_e112286)), ((((locals.var_fb_dn5 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn5)) + locals.var_fs01_dn5) / (2.0 * assign73810_body64_e112286)), ((((locals.var_fb_dn6 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn6)) + locals.var_fs01_dn6) / (2.0 * assign73810_body64_e112286)), ((((locals.var_fb_dn7 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn7)) + locals.var_fs01_dn7) / (2.0 * assign73810_body64_e112286)), ((((locals.var_fb_dn8 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn8)) + locals.var_fs01_dn8) / (2.0 * assign73810_body64_e112286)), ((((locals.var_fb_dn9 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn9)) + locals.var_fs01_dn9) / (2.0 * assign73810_body64_e112286)), ((((locals.var_fb_dn10 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn10)) + locals.var_fs01_dn10) / (2.0 * assign73810_body64_e112286)), ((((locals.var_fb_dn13 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn13)) + locals.var_fs01_dn13) / (2.0 * assign73810_body64_e112286)),)
    } else {
        (locals.var_fs02, locals.var_fs02_dn0, locals.var_fs02_dn2, locals.var_fs02_dn4, locals.var_fs02_dn5, locals.var_fs02_dn6, locals.var_fs02_dn7, locals.var_fs02_dn8, locals.var_fs02_dn9, locals.var_fs02_dn10, locals.var_fs02_dn13,)
    }
};
            locals.var_fs02 = assign73810_body64_e112288;
            locals.var_fs02_dn0 = assign73810_body64_e112288_d_n0;
            locals.var_fs02_dn2 = assign73810_body64_e112288_d_n2;
            locals.var_fs02_dn4 = assign73810_body64_e112288_d_n4;
            locals.var_fs02_dn5 = assign73810_body64_e112288_d_n5;
            locals.var_fs02_dn6 = assign73810_body64_e112288_d_n6;
            locals.var_fs02_dn7 = assign73810_body64_e112288_d_n7;
            locals.var_fs02_dn8 = assign73810_body64_e112288_d_n8;
            locals.var_fs02_dn9 = assign73810_body64_e112288_d_n9;
            locals.var_fs02_dn10 = assign73810_body64_e112288_d_n10;
            locals.var_fs02_dn13 = assign73810_body64_e112288_d_n13;
            locals.var_fs02_rv = 0.0;
            let (assign73810_body65_e112312, assign73810_body65_e112312_d_n0, assign73810_body65_e112312_d_n2, assign73810_body65_e112312_d_n4, assign73810_body65_e112312_d_n5, assign73810_body65_e112312_d_n6, assign73810_body65_e112312_d_n7, assign73810_body65_e112312_d_n8, assign73810_body65_e112312_d_n9, assign73810_body65_e112312_d_n10, assign73810_body65_e112312_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1704 != 0.0)) && (locals.var_guard1714 == 0.0)) && (locals.var_guard1717 != 0.0)) {
        let assign73810_body65_e112303: f64 = (2.0 * locals.var_fb_dpss);
        let assign73810_body65_e112305: f64 = (assign73810_body65_e112303 * locals.var_fb);
        let assign73810_body65_e112307: f64 = (assign73810_body65_e112305 + locals.var_fs01_dps0);
        let assign73810_body65_e112308: f64 = (0.5 * assign73810_body65_e112307);
        let assign73810_body65_e112310: f64 = (assign73810_body65_e112308 / locals.var_fs02);
        (assign73810_body65_e112310, ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn0) * locals.var_fb) + (assign73810_body65_e112303 * locals.var_fb_dn0)) + locals.var_fs01_dps0_dn0)) * locals.var_fs02) - (assign73810_body65_e112308 * locals.var_fs02_dn0)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn2) * locals.var_fb) + (assign73810_body65_e112303 * locals.var_fb_dn2)) + locals.var_fs01_dps0_dn2)) * locals.var_fs02) - (assign73810_body65_e112308 * locals.var_fs02_dn2)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn4) * locals.var_fb) + (assign73810_body65_e112303 * locals.var_fb_dn4)) + locals.var_fs01_dps0_dn4)) * locals.var_fs02) - (assign73810_body65_e112308 * locals.var_fs02_dn4)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn5) * locals.var_fb) + (assign73810_body65_e112303 * locals.var_fb_dn5)) + locals.var_fs01_dps0_dn5)) * locals.var_fs02) - (assign73810_body65_e112308 * locals.var_fs02_dn5)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn6) * locals.var_fb) + (assign73810_body65_e112303 * locals.var_fb_dn6)) + locals.var_fs01_dps0_dn6)) * locals.var_fs02) - (assign73810_body65_e112308 * locals.var_fs02_dn6)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn7) * locals.var_fb) + (assign73810_body65_e112303 * locals.var_fb_dn7)) + locals.var_fs01_dps0_dn7)) * locals.var_fs02) - (assign73810_body65_e112308 * locals.var_fs02_dn7)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn8) * locals.var_fb) + (assign73810_body65_e112303 * locals.var_fb_dn8)) + locals.var_fs01_dps0_dn8)) * locals.var_fs02) - (assign73810_body65_e112308 * locals.var_fs02_dn8)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn9) * locals.var_fb) + (assign73810_body65_e112303 * locals.var_fb_dn9)) + locals.var_fs01_dps0_dn9)) * locals.var_fs02) - (assign73810_body65_e112308 * locals.var_fs02_dn9)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn10) * locals.var_fb) + (assign73810_body65_e112303 * locals.var_fb_dn10)) + locals.var_fs01_dps0_dn10)) * locals.var_fs02) - (assign73810_body65_e112308 * locals.var_fs02_dn10)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn13) * locals.var_fb) + (assign73810_body65_e112303 * locals.var_fb_dn13)) + locals.var_fs01_dps0_dn13)) * locals.var_fs02) - (assign73810_body65_e112308 * locals.var_fs02_dn13)) / (locals.var_fs02 * locals.var_fs02)),)
    } else {
        (locals.var_fs02_dps0, locals.var_fs02_dps0_dn0, locals.var_fs02_dps0_dn2, locals.var_fs02_dps0_dn4, locals.var_fs02_dps0_dn5, locals.var_fs02_dps0_dn6, locals.var_fs02_dps0_dn7, locals.var_fs02_dps0_dn8, locals.var_fs02_dps0_dn9, locals.var_fs02_dps0_dn10, locals.var_fs02_dps0_dn13,)
    }
};
            locals.var_fs02_dps0 = assign73810_body65_e112312;
            locals.var_fs02_dps0_dn0 = assign73810_body65_e112312_d_n0;
            locals.var_fs02_dps0_dn2 = assign73810_body65_e112312_d_n2;
            locals.var_fs02_dps0_dn4 = assign73810_body65_e112312_d_n4;
            locals.var_fs02_dps0_dn5 = assign73810_body65_e112312_d_n5;
            locals.var_fs02_dps0_dn6 = assign73810_body65_e112312_d_n6;
            locals.var_fs02_dps0_dn7 = assign73810_body65_e112312_d_n7;
            locals.var_fs02_dps0_dn8 = assign73810_body65_e112312_d_n8;
            locals.var_fs02_dps0_dn9 = assign73810_body65_e112312_d_n9;
            locals.var_fs02_dps0_dn10 = assign73810_body65_e112312_d_n10;
            locals.var_fs02_dps0_dn13 = assign73810_body65_e112312_d_n13;
            locals.var_fs02_dps0_rv = 0.0;
            let (assign73810_body67_e112344, assign73810_body67_e112344_d_n0, assign73810_body67_e112344_d_n2, assign73810_body67_e112344_d_n4, assign73810_body67_e112344_d_n5, assign73810_body67_e112344_d_n6, assign73810_body67_e112344_d_n7, assign73810_body67_e112344_d_n8, assign73810_body67_e112344_d_n9, assign73810_body67_e112344_d_n10, assign73810_body67_e112344_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1704 != 0.0)) && (locals.var_guard1714 == 0.0)) && (locals.var_guard1717 == 0.0)) {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn4, locals.var_fb_dn5, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn8, locals.var_fb_dn9, locals.var_fb_dn10, locals.var_fb_dn13,)
    } else {
        (locals.var_fs02, locals.var_fs02_dn0, locals.var_fs02_dn2, locals.var_fs02_dn4, locals.var_fs02_dn5, locals.var_fs02_dn6, locals.var_fs02_dn7, locals.var_fs02_dn8, locals.var_fs02_dn9, locals.var_fs02_dn10, locals.var_fs02_dn13,)
    }
};
            locals.var_fs02 = assign73810_body67_e112344;
            locals.var_fs02_dn0 = assign73810_body67_e112344_d_n0;
            locals.var_fs02_dn2 = assign73810_body67_e112344_d_n2;
            locals.var_fs02_dn4 = assign73810_body67_e112344_d_n4;
            locals.var_fs02_dn5 = assign73810_body67_e112344_d_n5;
            locals.var_fs02_dn6 = assign73810_body67_e112344_d_n6;
            locals.var_fs02_dn7 = assign73810_body67_e112344_d_n7;
            locals.var_fs02_dn8 = assign73810_body67_e112344_d_n8;
            locals.var_fs02_dn9 = assign73810_body67_e112344_d_n9;
            locals.var_fs02_dn10 = assign73810_body67_e112344_d_n10;
            locals.var_fs02_dn13 = assign73810_body67_e112344_d_n13;
            locals.var_fs02_rv = 0.0;
            let (assign73810_body68_e112359, assign73810_body68_e112359_d_n0, assign73810_body68_e112359_d_n2, assign73810_body68_e112359_d_n4, assign73810_body68_e112359_d_n5, assign73810_body68_e112359_d_n6, assign73810_body68_e112359_d_n7, assign73810_body68_e112359_d_n8, assign73810_body68_e112359_d_n9, assign73810_body68_e112359_d_n10, assign73810_body68_e112359_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1704 != 0.0)) && (locals.var_guard1714 == 0.0)) && (locals.var_guard1717 == 0.0)) {
        (locals.var_fb_dpss, locals.var_fb_dpss_dn0, locals.var_fb_dpss_dn2, locals.var_fb_dpss_dn4, locals.var_fb_dpss_dn5, locals.var_fb_dpss_dn6, locals.var_fb_dpss_dn7, locals.var_fb_dpss_dn8, locals.var_fb_dpss_dn9, locals.var_fb_dpss_dn10, locals.var_fb_dpss_dn13,)
    } else {
        (locals.var_fs02_dps0, locals.var_fs02_dps0_dn0, locals.var_fs02_dps0_dn2, locals.var_fs02_dps0_dn4, locals.var_fs02_dps0_dn5, locals.var_fs02_dps0_dn6, locals.var_fs02_dps0_dn7, locals.var_fs02_dps0_dn8, locals.var_fs02_dps0_dn9, locals.var_fs02_dps0_dn10, locals.var_fs02_dps0_dn13,)
    }
};
            locals.var_fs02_dps0 = assign73810_body68_e112359;
            locals.var_fs02_dps0_dn0 = assign73810_body68_e112359_d_n0;
            locals.var_fs02_dps0_dn2 = assign73810_body68_e112359_d_n2;
            locals.var_fs02_dps0_dn4 = assign73810_body68_e112359_d_n4;
            locals.var_fs02_dps0_dn5 = assign73810_body68_e112359_d_n5;
            locals.var_fs02_dps0_dn6 = assign73810_body68_e112359_d_n6;
            locals.var_fs02_dps0_dn7 = assign73810_body68_e112359_d_n7;
            locals.var_fs02_dps0_dn8 = assign73810_body68_e112359_d_n8;
            locals.var_fs02_dps0_dn9 = assign73810_body68_e112359_d_n9;
            locals.var_fs02_dps0_dn10 = assign73810_body68_e112359_d_n10;
            locals.var_fs02_dps0_dn13 = assign73810_body68_e112359_d_n13;
            locals.var_fs02_dps0_rv = 0.0;
            let (assign73810_body69_e112375, assign73810_body69_e112375_d_n0, assign73810_body69_e112375_d_n2, assign73810_body69_e112375_d_n4, assign73810_body69_e112375_d_n5, assign73810_body69_e112375_d_n6, assign73810_body69_e112375_d_n7, assign73810_body69_e112375_d_n8, assign73810_body69_e112375_d_n9, assign73810_body69_e112375_d_n10, assign73810_body69_e112375_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1704 != 0.0)) {
        let assign73810_body69_e112367: f64 = (-locals.var_vgpld);
        let assign73810_body69_e112369: f64 = (assign73810_body69_e112367 + locals.var_ps0ld);
        let assign73810_body69_e112372: f64 = (locals.var_fac1 * locals.var_fs02);
        let assign73810_body69_e112373: f64 = (assign73810_body69_e112369 + assign73810_body69_e112372);
        (assign73810_body69_e112373, (locals.var_ps0ld_dn0 + ((locals.var_fac1_dn0 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn0))), (((-locals.var_vgpld_dn2) + locals.var_ps0ld_dn2) + ((locals.var_fac1_dn2 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn2))), (locals.var_ps0ld_dn4 + ((locals.var_fac1_dn4 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn4))), (locals.var_ps0ld_dn5 + ((locals.var_fac1_dn5 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn5))), (((-locals.var_vgpld_dn6) + locals.var_ps0ld_dn6) + ((locals.var_fac1_dn6 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn6))), (((-locals.var_vgpld_dn7) + locals.var_ps0ld_dn7) + ((locals.var_fac1_dn7 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn7))), (((-locals.var_vgpld_dn8) + locals.var_ps0ld_dn8) + ((locals.var_fac1_dn8 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn8))), (locals.var_ps0ld_dn9 + ((locals.var_fac1_dn9 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn9))), (locals.var_ps0ld_dn10 + ((locals.var_fac1_dn10 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn10))), (locals.var_ps0ld_dn13 + ((locals.var_fac1_dn13 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn13))),)
    } else {
        (locals.var_fs0, locals.var_fs0_dn0, locals.var_fs0_dn2, locals.var_fs0_dn4, locals.var_fs0_dn5, locals.var_fs0_dn6, locals.var_fs0_dn7, locals.var_fs0_dn8, locals.var_fs0_dn9, locals.var_fs0_dn10, locals.var_fs0_dn13,)
    }
};
            locals.var_fs0 = assign73810_body69_e112375;
            locals.var_fs0_dn0 = assign73810_body69_e112375_d_n0;
            locals.var_fs0_dn2 = assign73810_body69_e112375_d_n2;
            locals.var_fs0_dn4 = assign73810_body69_e112375_d_n4;
            locals.var_fs0_dn5 = assign73810_body69_e112375_d_n5;
            locals.var_fs0_dn6 = assign73810_body69_e112375_d_n6;
            locals.var_fs0_dn7 = assign73810_body69_e112375_d_n7;
            locals.var_fs0_dn8 = assign73810_body69_e112375_d_n8;
            locals.var_fs0_dn9 = assign73810_body69_e112375_d_n9;
            locals.var_fs0_dn10 = assign73810_body69_e112375_d_n10;
            locals.var_fs0_dn13 = assign73810_body69_e112375_d_n13;
            locals.var_fs0_rv = 0.0;
            let (assign73810_body70_e112388, assign73810_body70_e112388_d_n0, assign73810_body70_e112388_d_n2, assign73810_body70_e112388_d_n4, assign73810_body70_e112388_d_n5, assign73810_body70_e112388_d_n6, assign73810_body70_e112388_d_n7, assign73810_body70_e112388_d_n8, assign73810_body70_e112388_d_n9, assign73810_body70_e112388_d_n10, assign73810_body70_e112388_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1704 != 0.0)) {
        let assign73810_body70_e112385: f64 = (locals.var_fac1 * locals.var_fs02_dps0);
        let assign73810_body70_e112386: f64 = (1.0 + assign73810_body70_e112385);
        (assign73810_body70_e112386, ((locals.var_fac1_dn0 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn0)), ((locals.var_fac1_dn2 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn2)), ((locals.var_fac1_dn4 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn4)), ((locals.var_fac1_dn5 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn5)), ((locals.var_fac1_dn6 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn6)), ((locals.var_fac1_dn7 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn7)), ((locals.var_fac1_dn8 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn8)), ((locals.var_fac1_dn9 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn9)), ((locals.var_fac1_dn10 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn10)), ((locals.var_fac1_dn13 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn13)),)
    } else {
        (locals.var_fs0_dps0, locals.var_fs0_dps0_dn0, locals.var_fs0_dps0_dn2, locals.var_fs0_dps0_dn4, locals.var_fs0_dps0_dn5, locals.var_fs0_dps0_dn6, locals.var_fs0_dps0_dn7, locals.var_fs0_dps0_dn8, locals.var_fs0_dps0_dn9, locals.var_fs0_dps0_dn10, locals.var_fs0_dps0_dn13,)
    }
};
            locals.var_fs0_dps0 = assign73810_body70_e112388;
            locals.var_fs0_dps0_dn0 = assign73810_body70_e112388_d_n0;
            locals.var_fs0_dps0_dn2 = assign73810_body70_e112388_d_n2;
            locals.var_fs0_dps0_dn4 = assign73810_body70_e112388_d_n4;
            locals.var_fs0_dps0_dn5 = assign73810_body70_e112388_d_n5;
            locals.var_fs0_dps0_dn6 = assign73810_body70_e112388_d_n6;
            locals.var_fs0_dps0_dn7 = assign73810_body70_e112388_d_n7;
            locals.var_fs0_dps0_dn8 = assign73810_body70_e112388_d_n8;
            locals.var_fs0_dps0_dn9 = assign73810_body70_e112388_d_n9;
            locals.var_fs0_dps0_dn10 = assign73810_body70_e112388_d_n10;
            locals.var_fs0_dps0_dn13 = assign73810_body70_e112388_d_n13;
            locals.var_fs0_dps0_rv = 0.0;
            let assign73810_body71_e112391: f64 = if locals.var_flg_conv == 1.0 { 1.0 } else { 0.0 };
            locals.var_guard1718 = assign73810_body71_e112391;
            locals.var_guard1718_rv = 0.0;
            let (assign73810_body72_e112404,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1704 != 0.0)) && (locals.var_guard1718 != 0.0)) {
        let assign73810_body72_e112402: f64 = (locals.var_lp_s0_max + 1.0);
        (assign73810_body72_e112402,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign73810_body72_e112404;
            locals.var_lp_s0_rv = 0.0;
            let (assign73810_body73_e112419, assign73810_body73_e112419_d_n0, assign73810_body73_e112419_d_n2, assign73810_body73_e112419_d_n4, assign73810_body73_e112419_d_n5, assign73810_body73_e112419_d_n6, assign73810_body73_e112419_d_n7, assign73810_body73_e112419_d_n8, assign73810_body73_e112419_d_n9, assign73810_body73_e112419_d_n10, assign73810_body73_e112419_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1704 != 0.0)) && (locals.var_guard1718 == 0.0)) {
        let assign73810_body73_e112415: f64 = (-locals.var_fs0);
        let assign73810_body73_e112417: f64 = (assign73810_body73_e112415 / locals.var_fs0_dps0);
        (assign73810_body73_e112417, ((((-locals.var_fs0_dn0) * locals.var_fs0_dps0) - (assign73810_body73_e112415 * locals.var_fs0_dps0_dn0)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn2) * locals.var_fs0_dps0) - (assign73810_body73_e112415 * locals.var_fs0_dps0_dn2)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn4) * locals.var_fs0_dps0) - (assign73810_body73_e112415 * locals.var_fs0_dps0_dn4)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn5) * locals.var_fs0_dps0) - (assign73810_body73_e112415 * locals.var_fs0_dps0_dn5)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn6) * locals.var_fs0_dps0) - (assign73810_body73_e112415 * locals.var_fs0_dps0_dn6)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn7) * locals.var_fs0_dps0) - (assign73810_body73_e112415 * locals.var_fs0_dps0_dn7)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn8) * locals.var_fs0_dps0) - (assign73810_body73_e112415 * locals.var_fs0_dps0_dn8)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn9) * locals.var_fs0_dps0) - (assign73810_body73_e112415 * locals.var_fs0_dps0_dn9)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn10) * locals.var_fs0_dps0) - (assign73810_body73_e112415 * locals.var_fs0_dps0_dn10)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn13) * locals.var_fs0_dps0) - (assign73810_body73_e112415 * locals.var_fs0_dps0_dn13)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)),)
    } else {
        (locals.var_dps0, locals.var_dps0_dn0, locals.var_dps0_dn2, locals.var_dps0_dn4, locals.var_dps0_dn5, locals.var_dps0_dn6, locals.var_dps0_dn7, locals.var_dps0_dn8, locals.var_dps0_dn9, locals.var_dps0_dn10, locals.var_dps0_dn13,)
    }
};
            locals.var_dps0 = assign73810_body73_e112419;
            locals.var_dps0_dn0 = assign73810_body73_e112419_d_n0;
            locals.var_dps0_dn2 = assign73810_body73_e112419_d_n2;
            locals.var_dps0_dn4 = assign73810_body73_e112419_d_n4;
            locals.var_dps0_dn5 = assign73810_body73_e112419_d_n5;
            locals.var_dps0_dn6 = assign73810_body73_e112419_d_n6;
            locals.var_dps0_dn7 = assign73810_body73_e112419_d_n7;
            locals.var_dps0_dn8 = assign73810_body73_e112419_d_n8;
            locals.var_dps0_dn9 = assign73810_body73_e112419_d_n9;
            locals.var_dps0_dn10 = assign73810_body73_e112419_d_n10;
            locals.var_dps0_dn13 = assign73810_body73_e112419_d_n13;
            locals.var_dps0_rv = 0.0;
            let (assign73810_body74_e112444, assign73810_body74_e112444_d_n0, assign73810_body74_e112444_d_n2, assign73810_body74_e112444_d_n4, assign73810_body74_e112444_d_n5, assign73810_body74_e112444_d_n6, assign73810_body74_e112444_d_n7, assign73810_body74_e112444_d_n8, assign73810_body74_e112444_d_n9, assign73810_body74_e112444_d_n10, assign73810_body74_e112444_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1704 != 0.0)) && (locals.var_guard1718 == 0.0)) {
        let assign73810_body74_e112431: f64 = (0.5 * 0.1);
        let assign73810_body74_e112435: f64 = (locals.var_ps0ld).abs();
        let (assign73810_body74_e112440, assign73810_body74_e112440_d_n0, assign73810_body74_e112440_d_n2, assign73810_body74_e112440_d_n4, assign73810_body74_e112440_d_n5, assign73810_body74_e112440_d_n6, assign73810_body74_e112440_d_n7, assign73810_body74_e112440_d_n8, assign73810_body74_e112440_d_n9, assign73810_body74_e112440_d_n10, assign73810_body74_e112440_d_n13,) = {
            if (1.0 >= assign73810_body74_e112435) {
                (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign73810_body74_e112439: f64 = (locals.var_ps0ld).abs();
                (assign73810_body74_e112439, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn0 } else { (-locals.var_ps0ld_dn0) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn2 } else { (-locals.var_ps0ld_dn2) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn4 } else { (-locals.var_ps0ld_dn4) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn5 } else { (-locals.var_ps0ld_dn5) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn6 } else { (-locals.var_ps0ld_dn6) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn7 } else { (-locals.var_ps0ld_dn7) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn8 } else { (-locals.var_ps0ld_dn8) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn9 } else { (-locals.var_ps0ld_dn9) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn10 } else { (-locals.var_ps0ld_dn10) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn13 } else { (-locals.var_ps0ld_dn13) },)
            }
        };
        let assign73810_body74_e112441: f64 = (1.0 + assign73810_body74_e112440);
        let assign73810_body74_e112442: f64 = (assign73810_body74_e112431 * assign73810_body74_e112441);
        (assign73810_body74_e112442, (assign73810_body74_e112431 * assign73810_body74_e112440_d_n0), (assign73810_body74_e112431 * assign73810_body74_e112440_d_n2), (assign73810_body74_e112431 * assign73810_body74_e112440_d_n4), (assign73810_body74_e112431 * assign73810_body74_e112440_d_n5), (assign73810_body74_e112431 * assign73810_body74_e112440_d_n6), (assign73810_body74_e112431 * assign73810_body74_e112440_d_n7), (assign73810_body74_e112431 * assign73810_body74_e112440_d_n8), (assign73810_body74_e112431 * assign73810_body74_e112440_d_n9), (assign73810_body74_e112431 * assign73810_body74_e112440_d_n10), (assign73810_body74_e112431 * assign73810_body74_e112440_d_n13),)
    } else {
        (locals.var_dplim, locals.var_dplim_dn0, locals.var_dplim_dn2, locals.var_dplim_dn4, locals.var_dplim_dn5, locals.var_dplim_dn6, locals.var_dplim_dn7, locals.var_dplim_dn8, locals.var_dplim_dn9, locals.var_dplim_dn10, locals.var_dplim_dn13,)
    }
};
            locals.var_dplim = assign73810_body74_e112444;
            locals.var_dplim_dn0 = assign73810_body74_e112444_d_n0;
            locals.var_dplim_dn2 = assign73810_body74_e112444_d_n2;
            locals.var_dplim_dn4 = assign73810_body74_e112444_d_n4;
            locals.var_dplim_dn5 = assign73810_body74_e112444_d_n5;
            locals.var_dplim_dn6 = assign73810_body74_e112444_d_n6;
            locals.var_dplim_dn7 = assign73810_body74_e112444_d_n7;
            locals.var_dplim_dn8 = assign73810_body74_e112444_d_n8;
            locals.var_dplim_dn9 = assign73810_body74_e112444_d_n9;
            locals.var_dplim_dn10 = assign73810_body74_e112444_d_n10;
            locals.var_dplim_dn13 = assign73810_body74_e112444_d_n13;
            locals.var_dplim_rv = 0.0;
            let assign73810_body75_e112446: f64 = (locals.var_dps0).abs();
            let assign73810_body75_e112448: f64 = if assign73810_body75_e112446 > locals.var_dplim { 1.0 } else { 0.0 };
            locals.var_guard1719 = assign73810_body75_e112448;
            locals.var_guard1719_rv = 0.0;
            let (assign73810_body76_e112470, assign73810_body76_e112470_d_n0, assign73810_body76_e112470_d_n2, assign73810_body76_e112470_d_n4, assign73810_body76_e112470_d_n5, assign73810_body76_e112470_d_n6, assign73810_body76_e112470_d_n7, assign73810_body76_e112470_d_n8, assign73810_body76_e112470_d_n9, assign73810_body76_e112470_d_n10, assign73810_body76_e112470_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1704 != 0.0)) && (locals.var_guard1718 == 0.0)) && (locals.var_guard1719 != 0.0)) {
        let (assign73810_body76_e112467,) = {
            if (locals.var_dps0 >= 0.0) {
                (1.0,)
            } else {
                let assign73810_body76_e112466: f64 = (-1.0);
                (assign73810_body76_e112466,)
            }
        };
        let assign73810_body76_e112468: f64 = (locals.var_dplim * assign73810_body76_e112467);
        (assign73810_body76_e112468, (locals.var_dplim_dn0 * assign73810_body76_e112467), (locals.var_dplim_dn2 * assign73810_body76_e112467), (locals.var_dplim_dn4 * assign73810_body76_e112467), (locals.var_dplim_dn5 * assign73810_body76_e112467), (locals.var_dplim_dn6 * assign73810_body76_e112467), (locals.var_dplim_dn7 * assign73810_body76_e112467), (locals.var_dplim_dn8 * assign73810_body76_e112467), (locals.var_dplim_dn9 * assign73810_body76_e112467), (locals.var_dplim_dn10 * assign73810_body76_e112467), (locals.var_dplim_dn13 * assign73810_body76_e112467),)
    } else {
        (locals.var_dps0, locals.var_dps0_dn0, locals.var_dps0_dn2, locals.var_dps0_dn4, locals.var_dps0_dn5, locals.var_dps0_dn6, locals.var_dps0_dn7, locals.var_dps0_dn8, locals.var_dps0_dn9, locals.var_dps0_dn10, locals.var_dps0_dn13,)
    }
};
            locals.var_dps0 = assign73810_body76_e112470;
            locals.var_dps0_dn0 = assign73810_body76_e112470_d_n0;
            locals.var_dps0_dn2 = assign73810_body76_e112470_d_n2;
            locals.var_dps0_dn4 = assign73810_body76_e112470_d_n4;
            locals.var_dps0_dn5 = assign73810_body76_e112470_d_n5;
            locals.var_dps0_dn6 = assign73810_body76_e112470_d_n6;
            locals.var_dps0_dn7 = assign73810_body76_e112470_d_n7;
            locals.var_dps0_dn8 = assign73810_body76_e112470_d_n8;
            locals.var_dps0_dn9 = assign73810_body76_e112470_d_n9;
            locals.var_dps0_dn10 = assign73810_body76_e112470_d_n10;
            locals.var_dps0_dn13 = assign73810_body76_e112470_d_n13;
            locals.var_dps0_rv = 0.0;
            let (assign73810_body77_e112484, assign73810_body77_e112484_d_n0, assign73810_body77_e112484_d_n2, assign73810_body77_e112484_d_n4, assign73810_body77_e112484_d_n5, assign73810_body77_e112484_d_n6, assign73810_body77_e112484_d_n7, assign73810_body77_e112484_d_n8, assign73810_body77_e112484_d_n9, assign73810_body77_e112484_d_n10, assign73810_body77_e112484_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1704 != 0.0)) && (locals.var_guard1718 == 0.0)) {
        let assign73810_body77_e112482: f64 = (locals.var_ps0ld + locals.var_dps0);
        (assign73810_body77_e112482, (locals.var_ps0ld_dn0 + locals.var_dps0_dn0), (locals.var_ps0ld_dn2 + locals.var_dps0_dn2), (locals.var_ps0ld_dn4 + locals.var_dps0_dn4), (locals.var_ps0ld_dn5 + locals.var_dps0_dn5), (locals.var_ps0ld_dn6 + locals.var_dps0_dn6), (locals.var_ps0ld_dn7 + locals.var_dps0_dn7), (locals.var_ps0ld_dn8 + locals.var_dps0_dn8), (locals.var_ps0ld_dn9 + locals.var_dps0_dn9), (locals.var_ps0ld_dn10 + locals.var_dps0_dn10), (locals.var_ps0ld_dn13 + locals.var_dps0_dn13),)
    } else {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn13,)
    }
};
            locals.var_ps0ld = assign73810_body77_e112484;
            locals.var_ps0ld_dn0 = assign73810_body77_e112484_d_n0;
            locals.var_ps0ld_dn2 = assign73810_body77_e112484_d_n2;
            locals.var_ps0ld_dn4 = assign73810_body77_e112484_d_n4;
            locals.var_ps0ld_dn5 = assign73810_body77_e112484_d_n5;
            locals.var_ps0ld_dn6 = assign73810_body77_e112484_d_n6;
            locals.var_ps0ld_dn7 = assign73810_body77_e112484_d_n7;
            locals.var_ps0ld_dn8 = assign73810_body77_e112484_d_n8;
            locals.var_ps0ld_dn9 = assign73810_body77_e112484_d_n9;
            locals.var_ps0ld_dn10 = assign73810_body77_e112484_d_n10;
            locals.var_ps0ld_dn13 = assign73810_body77_e112484_d_n13;
            locals.var_ps0ld_rv = 0.0;
            let assign73810_body78_e112486: f64 = (locals.var_dps0).abs();
            let assign73810_body78_e112490: f64 = (locals.var_fs0).abs();
            let assign73810_body78_e112493: f64 = if ((assign73810_body78_e112486 <= 1e-12) && (assign73810_body78_e112490 <= 1e-8)) { 1.0 } else { 0.0 };
            locals.var_guard1720 = assign73810_body78_e112493;
            locals.var_guard1720_rv = 0.0;
            let (assign73810_body79_e112507,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1704 != 0.0)) && (locals.var_guard1718 == 0.0)) && (locals.var_guard1720 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_conv,)
    }
};
            locals.var_flg_conv = assign73810_body79_e112507;
            locals.var_flg_conv_rv = 0.0;
            let (assign73810_body80_e112518,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1704 != 0.0)) {
        let assign73810_body80_e112516: f64 = (locals.var_lp_s0 + 1.0);
        (assign73810_body80_e112516,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign73810_body80_e112518;
            locals.var_lp_s0_rv = 0.0;
        }

    }

    pub(super) fn stamp_reactive_block_267(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign73830_e112532, assign73830_e112532_d_n0, assign73830_e112532_d_n2, assign73830_e112532_d_n4, assign73830_e112532_d_n5, assign73830_e112532_d_n6, assign73830_e112532_d_n7, assign73830_e112532_d_n8, assign73830_e112532_d_n9, assign73830_e112532_d_n10, assign73830_e112532_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1704 != 0.0)) {
        let assign73830_e112530: f64 = (locals.var_c_w_ld * locals.var_fb);
        (assign73830_e112530, ((locals.var_c_w_ld_dn0 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn0)), ((locals.var_c_w_ld_dn2 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn2)), ((locals.var_c_w_ld_dn4 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn4)), ((locals.var_c_w_ld_dn5 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn5)), ((locals.var_c_w_ld_dn6 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn6)), ((locals.var_c_w_ld_dn7 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn7)), ((locals.var_c_w_ld_dn8 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn8)), ((locals.var_c_w_ld_dn9 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn9)), ((locals.var_c_w_ld_dn10 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn10)), ((locals.var_c_w_ld_dn13 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn13)),)
    } else {
        (locals.var_wdld, locals.var_wdld_dn0, locals.var_wdld_dn2, locals.var_wdld_dn4, locals.var_wdld_dn5, locals.var_wdld_dn6, locals.var_wdld_dn7, locals.var_wdld_dn8, locals.var_wdld_dn9, locals.var_wdld_dn10, locals.var_wdld_dn13,)
    }
};
        locals.var_wdld = assign73830_e112532;
        locals.var_wdld_dn0 = assign73830_e112532_d_n0;
        locals.var_wdld_dn2 = assign73830_e112532_d_n2;
        locals.var_wdld_dn4 = assign73830_e112532_d_n4;
        locals.var_wdld_dn5 = assign73830_e112532_d_n5;
        locals.var_wdld_dn6 = assign73830_e112532_d_n6;
        locals.var_wdld_dn7 = assign73830_e112532_d_n7;
        locals.var_wdld_dn8 = assign73830_e112532_d_n8;
        locals.var_wdld_dn9 = assign73830_e112532_d_n9;
        locals.var_wdld_dn10 = assign73830_e112532_d_n10;
        locals.var_wdld_dn13 = assign73830_e112532_d_n13;
        locals.var_wdld_rv = 0.0;

        let (assign73840_e112543, assign73840_e112543_d_n0, assign73840_e112543_d_n2, assign73840_e112543_d_n4, assign73840_e112543_d_n5, assign73840_e112543_d_n6, assign73840_e112543_d_n7, assign73840_e112543_d_n8, assign73840_e112543_d_n9, assign73840_e112543_d_n10, assign73840_e112543_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1704 != 0.0)) {
        let assign73840_e112541: f64 = (locals.var_q_nsubld * locals.var_wdld);
        (assign73840_e112541, (locals.var_q_nsubld * locals.var_wdld_dn0), (locals.var_q_nsubld * locals.var_wdld_dn2), (locals.var_q_nsubld * locals.var_wdld_dn4), (locals.var_q_nsubld * locals.var_wdld_dn5), (locals.var_q_nsubld * locals.var_wdld_dn6), (locals.var_q_nsubld * locals.var_wdld_dn7), (locals.var_q_nsubld * locals.var_wdld_dn8), (locals.var_q_nsubld * locals.var_wdld_dn9), (locals.var_q_nsubld * locals.var_wdld_dn10), (locals.var_q_nsubld * locals.var_wdld_dn13),)
    } else {
        (locals.var_q_dep_ld, locals.var_q_dep_ld_dn0, locals.var_q_dep_ld_dn2, locals.var_q_dep_ld_dn4, locals.var_q_dep_ld_dn5, locals.var_q_dep_ld_dn6, locals.var_q_dep_ld_dn7, locals.var_q_dep_ld_dn8, locals.var_q_dep_ld_dn9, locals.var_q_dep_ld_dn10, locals.var_q_dep_ld_dn13,)
    }
};
        locals.var_q_dep_ld = assign73840_e112543;
        locals.var_q_dep_ld_dn0 = assign73840_e112543_d_n0;
        locals.var_q_dep_ld_dn2 = assign73840_e112543_d_n2;
        locals.var_q_dep_ld_dn4 = assign73840_e112543_d_n4;
        locals.var_q_dep_ld_dn5 = assign73840_e112543_d_n5;
        locals.var_q_dep_ld_dn6 = assign73840_e112543_d_n6;
        locals.var_q_dep_ld_dn7 = assign73840_e112543_d_n7;
        locals.var_q_dep_ld_dn8 = assign73840_e112543_d_n8;
        locals.var_q_dep_ld_dn9 = assign73840_e112543_d_n9;
        locals.var_q_dep_ld_dn10 = assign73840_e112543_d_n10;
        locals.var_q_dep_ld_dn13 = assign73840_e112543_d_n13;
        locals.var_q_dep_ld_rv = 0.0;

        let (assign73850_e112558, assign73850_e112558_d_n0, assign73850_e112558_d_n2, assign73850_e112558_d_n4, assign73850_e112558_d_n5, assign73850_e112558_d_n6, assign73850_e112558_d_n7, assign73850_e112558_d_n8, assign73850_e112558_d_n9, assign73850_e112558_d_n10, assign73850_e112558_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1704 != 0.0)) {
        let assign73850_e112552: f64 = (locals.var_q_dep_ld / locals.var_cnst0over_func);
        let assign73850_e112555: f64 = (10.0 * 2.220446049250313e-16);
        let assign73850_e112556: f64 = (assign73850_e112552 + assign73850_e112555);
        (assign73850_e112556, (((locals.var_q_dep_ld_dn0 * locals.var_cnst0over_func) - (locals.var_q_dep_ld * locals.var_cnst0over_func_dn0)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld_dn2 * locals.var_cnst0over_func) - (locals.var_q_dep_ld * locals.var_cnst0over_func_dn2)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld_dn4 * locals.var_cnst0over_func) - (locals.var_q_dep_ld * locals.var_cnst0over_func_dn4)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld_dn5 * locals.var_cnst0over_func) - (locals.var_q_dep_ld * locals.var_cnst0over_func_dn5)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld_dn6 * locals.var_cnst0over_func) - (locals.var_q_dep_ld * locals.var_cnst0over_func_dn6)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld_dn7 * locals.var_cnst0over_func) - (locals.var_q_dep_ld * locals.var_cnst0over_func_dn7)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld_dn8 * locals.var_cnst0over_func) - (locals.var_q_dep_ld * locals.var_cnst0over_func_dn8)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld_dn9 * locals.var_cnst0over_func) - (locals.var_q_dep_ld * locals.var_cnst0over_func_dn9)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld_dn10 * locals.var_cnst0over_func) - (locals.var_q_dep_ld * locals.var_cnst0over_func_dn10)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld_dn13 * locals.var_cnst0over_func) - (locals.var_q_dep_ld * locals.var_cnst0over_func_dn13)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)),)
    } else {
        (locals.var_xi0p12, locals.var_xi0p12_dn0, locals.var_xi0p12_dn2, locals.var_xi0p12_dn4, locals.var_xi0p12_dn5, locals.var_xi0p12_dn6, locals.var_xi0p12_dn7, locals.var_xi0p12_dn8, locals.var_xi0p12_dn9, locals.var_xi0p12_dn10, locals.var_xi0p12_dn13,)
    }
};
        locals.var_xi0p12 = assign73850_e112558;
        locals.var_xi0p12_dn0 = assign73850_e112558_d_n0;
        locals.var_xi0p12_dn2 = assign73850_e112558_d_n2;
        locals.var_xi0p12_dn4 = assign73850_e112558_d_n4;
        locals.var_xi0p12_dn5 = assign73850_e112558_d_n5;
        locals.var_xi0p12_dn6 = assign73850_e112558_d_n6;
        locals.var_xi0p12_dn7 = assign73850_e112558_d_n7;
        locals.var_xi0p12_dn8 = assign73850_e112558_d_n8;
        locals.var_xi0p12_dn9 = assign73850_e112558_d_n9;
        locals.var_xi0p12_dn10 = assign73850_e112558_d_n10;
        locals.var_xi0p12_dn13 = assign73850_e112558_d_n13;
        locals.var_xi0p12_rv = 0.0;

        let (assign73860_e112569, assign73860_e112569_d_n0, assign73860_e112569_d_n2, assign73860_e112569_d_n4, assign73860_e112569_d_n5, assign73860_e112569_d_n6, assign73860_e112569_d_n7, assign73860_e112569_d_n8, assign73860_e112569_d_n9, assign73860_e112569_d_n10, assign73860_e112569_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1704 != 0.0)) {
        let assign73860_e112567: f64 = (locals.var_cnst0over_func * locals.var_xi0p12);
        (assign73860_e112567, ((locals.var_cnst0over_func_dn0 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn0)), ((locals.var_cnst0over_func_dn2 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn2)), ((locals.var_cnst0over_func_dn4 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn4)), ((locals.var_cnst0over_func_dn5 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn5)), ((locals.var_cnst0over_func_dn6 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn6)), ((locals.var_cnst0over_func_dn7 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn7)), ((locals.var_cnst0over_func_dn8 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn8)), ((locals.var_cnst0over_func_dn9 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn9)), ((locals.var_cnst0over_func_dn10 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn10)), ((locals.var_cnst0over_func_dn13 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn13)),)
    } else {
        (locals.var_qbuld, locals.var_qbuld_dn0, locals.var_qbuld_dn2, locals.var_qbuld_dn4, locals.var_qbuld_dn5, locals.var_qbuld_dn6, locals.var_qbuld_dn7, locals.var_qbuld_dn8, locals.var_qbuld_dn9, locals.var_qbuld_dn10, locals.var_qbuld_dn13,)
    }
};
        locals.var_qbuld = assign73860_e112569;
        locals.var_qbuld_dn0 = assign73860_e112569_d_n0;
        locals.var_qbuld_dn2 = assign73860_e112569_d_n2;
        locals.var_qbuld_dn4 = assign73860_e112569_d_n4;
        locals.var_qbuld_dn5 = assign73860_e112569_d_n5;
        locals.var_qbuld_dn6 = assign73860_e112569_d_n6;
        locals.var_qbuld_dn7 = assign73860_e112569_d_n7;
        locals.var_qbuld_dn8 = assign73860_e112569_d_n8;
        locals.var_qbuld_dn9 = assign73860_e112569_d_n9;
        locals.var_qbuld_dn10 = assign73860_e112569_d_n10;
        locals.var_qbuld_dn13 = assign73860_e112569_d_n13;
        locals.var_qbuld_rv = 0.0;

        let (assign73870_e112582, assign73870_e112582_d_n0, assign73870_e112582_d_n2, assign73870_e112582_d_n4, assign73870_e112582_d_n5, assign73870_e112582_d_n6, assign73870_e112582_d_n7, assign73870_e112582_d_n8, assign73870_e112582_d_n9, assign73870_e112582_d_n10, assign73870_e112582_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1704 != 0.0)) {
        let assign73870_e112579: f64 = (locals.var_fs02 + locals.var_xi0p12);
        let assign73870_e112580: f64 = (1.0 / assign73870_e112579);
        (assign73870_e112580, (-((locals.var_fs02_dn0 + locals.var_xi0p12_dn0) / (assign73870_e112579 * assign73870_e112579))), (-((locals.var_fs02_dn2 + locals.var_xi0p12_dn2) / (assign73870_e112579 * assign73870_e112579))), (-((locals.var_fs02_dn4 + locals.var_xi0p12_dn4) / (assign73870_e112579 * assign73870_e112579))), (-((locals.var_fs02_dn5 + locals.var_xi0p12_dn5) / (assign73870_e112579 * assign73870_e112579))), (-((locals.var_fs02_dn6 + locals.var_xi0p12_dn6) / (assign73870_e112579 * assign73870_e112579))), (-((locals.var_fs02_dn7 + locals.var_xi0p12_dn7) / (assign73870_e112579 * assign73870_e112579))), (-((locals.var_fs02_dn8 + locals.var_xi0p12_dn8) / (assign73870_e112579 * assign73870_e112579))), (-((locals.var_fs02_dn9 + locals.var_xi0p12_dn9) / (assign73870_e112579 * assign73870_e112579))), (-((locals.var_fs02_dn10 + locals.var_xi0p12_dn10) / (assign73870_e112579 * assign73870_e112579))), (-((locals.var_fs02_dn13 + locals.var_xi0p12_dn13) / (assign73870_e112579 * assign73870_e112579))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign73870_e112582;
        locals.var_t1_dn0 = assign73870_e112582_d_n0;
        locals.var_t1_dn2 = assign73870_e112582_d_n2;
        locals.var_t1_dn4 = assign73870_e112582_d_n4;
        locals.var_t1_dn5 = assign73870_e112582_d_n5;
        locals.var_t1_dn6 = assign73870_e112582_d_n6;
        locals.var_t1_dn7 = assign73870_e112582_d_n7;
        locals.var_t1_dn8 = assign73870_e112582_d_n8;
        locals.var_t1_dn9 = assign73870_e112582_d_n9;
        locals.var_t1_dn10 = assign73870_e112582_d_n10;
        locals.var_t1_dn13 = assign73870_e112582_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign73880_e112595, assign73880_e112595_d_n0, assign73880_e112595_d_n2, assign73880_e112595_d_n4, assign73880_e112595_d_n5, assign73880_e112595_d_n6, assign73880_e112595_d_n7, assign73880_e112595_d_n8, assign73880_e112595_d_n9, assign73880_e112595_d_n10, assign73880_e112595_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1704 != 0.0)) {
        let assign73880_e112591: f64 = (locals.var_cnst0over_func * locals.var_fs01);
        let assign73880_e112593: f64 = (assign73880_e112591 * locals.var_t1);
        (assign73880_e112593, ((((locals.var_cnst0over_func_dn0 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn0)) * locals.var_t1) + (assign73880_e112591 * locals.var_t1_dn0)), ((((locals.var_cnst0over_func_dn2 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn2)) * locals.var_t1) + (assign73880_e112591 * locals.var_t1_dn2)), ((((locals.var_cnst0over_func_dn4 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn4)) * locals.var_t1) + (assign73880_e112591 * locals.var_t1_dn4)), ((((locals.var_cnst0over_func_dn5 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn5)) * locals.var_t1) + (assign73880_e112591 * locals.var_t1_dn5)), ((((locals.var_cnst0over_func_dn6 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn6)) * locals.var_t1) + (assign73880_e112591 * locals.var_t1_dn6)), ((((locals.var_cnst0over_func_dn7 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn7)) * locals.var_t1) + (assign73880_e112591 * locals.var_t1_dn7)), ((((locals.var_cnst0over_func_dn8 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn8)) * locals.var_t1) + (assign73880_e112591 * locals.var_t1_dn8)), ((((locals.var_cnst0over_func_dn9 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn9)) * locals.var_t1) + (assign73880_e112591 * locals.var_t1_dn9)), ((((locals.var_cnst0over_func_dn10 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn10)) * locals.var_t1) + (assign73880_e112591 * locals.var_t1_dn10)), ((((locals.var_cnst0over_func_dn13 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn13)) * locals.var_t1) + (assign73880_e112591 * locals.var_t1_dn13)),)
    } else {
        (locals.var_qiuld, locals.var_qiuld_dn0, locals.var_qiuld_dn2, locals.var_qiuld_dn4, locals.var_qiuld_dn5, locals.var_qiuld_dn6, locals.var_qiuld_dn7, locals.var_qiuld_dn8, locals.var_qiuld_dn9, locals.var_qiuld_dn10, locals.var_qiuld_dn13,)
    }
};
        locals.var_qiuld = assign73880_e112595;
        locals.var_qiuld_dn0 = assign73880_e112595_d_n0;
        locals.var_qiuld_dn2 = assign73880_e112595_d_n2;
        locals.var_qiuld_dn4 = assign73880_e112595_d_n4;
        locals.var_qiuld_dn5 = assign73880_e112595_d_n5;
        locals.var_qiuld_dn6 = assign73880_e112595_d_n6;
        locals.var_qiuld_dn7 = assign73880_e112595_d_n7;
        locals.var_qiuld_dn8 = assign73880_e112595_d_n8;
        locals.var_qiuld_dn9 = assign73880_e112595_d_n9;
        locals.var_qiuld_dn10 = assign73880_e112595_d_n10;
        locals.var_qiuld_dn13 = assign73880_e112595_d_n13;
        locals.var_qiuld_rv = 0.0;

        let (assign73890_e112606, assign73890_e112606_d_n0, assign73890_e112606_d_n2, assign73890_e112606_d_n4, assign73890_e112606_d_n5, assign73890_e112606_d_n6, assign73890_e112606_d_n7, assign73890_e112606_d_n8, assign73890_e112606_d_n9, assign73890_e112606_d_n10, assign73890_e112606_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1704 != 0.0)) {
        let assign73890_e112604: f64 = (locals.var_qbuld + locals.var_qiuld);
        (assign73890_e112604, (locals.var_qbuld_dn0 + locals.var_qiuld_dn0), (locals.var_qbuld_dn2 + locals.var_qiuld_dn2), (locals.var_qbuld_dn4 + locals.var_qiuld_dn4), (locals.var_qbuld_dn5 + locals.var_qiuld_dn5), (locals.var_qbuld_dn6 + locals.var_qiuld_dn6), (locals.var_qbuld_dn7 + locals.var_qiuld_dn7), (locals.var_qbuld_dn8 + locals.var_qiuld_dn8), (locals.var_qbuld_dn9 + locals.var_qiuld_dn9), (locals.var_qbuld_dn10 + locals.var_qiuld_dn10), (locals.var_qbuld_dn13 + locals.var_qiuld_dn13),)
    } else {
        (locals.var_qsuld, locals.var_qsuld_dn0, locals.var_qsuld_dn2, locals.var_qsuld_dn4, locals.var_qsuld_dn5, locals.var_qsuld_dn6, locals.var_qsuld_dn7, locals.var_qsuld_dn8, locals.var_qsuld_dn9, locals.var_qsuld_dn10, locals.var_qsuld_dn13,)
    }
};
        locals.var_qsuld = assign73890_e112606;
        locals.var_qsuld_dn0 = assign73890_e112606_d_n0;
        locals.var_qsuld_dn2 = assign73890_e112606_d_n2;
        locals.var_qsuld_dn4 = assign73890_e112606_d_n4;
        locals.var_qsuld_dn5 = assign73890_e112606_d_n5;
        locals.var_qsuld_dn6 = assign73890_e112606_d_n6;
        locals.var_qsuld_dn7 = assign73890_e112606_d_n7;
        locals.var_qsuld_dn8 = assign73890_e112606_d_n8;
        locals.var_qsuld_dn9 = assign73890_e112606_d_n9;
        locals.var_qsuld_dn10 = assign73890_e112606_d_n10;
        locals.var_qsuld_dn13 = assign73890_e112606_d_n13;
        locals.var_qsuld_rv = 0.0;

        let assign73900_e112609: f64 = if p.p33 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard1722 = assign73900_e112609;
        locals.var_guard1722_rv = 0.0;

        let (assign73910_e112619, assign73910_e112619_d_n0, assign73910_e112619_d_n2, assign73910_e112619_d_n4, assign73910_e112619_d_n5, assign73910_e112619_d_n6, assign73910_e112619_d_n7, assign73910_e112619_d_n8, assign73910_e112619_d_n9, assign73910_e112619_d_n10, assign73910_e112619_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1722 != 0.0)) {
        let assign73910_e112615: f64 = (-locals.var_vxbgmtcl);
        let assign73910_e112616: f64 = (locals.var_beta * assign73910_e112615);
        let assign73910_e112617: f64 = (assign73910_e112616).exp();
        (assign73910_e112617, (assign73910_e112617 * ((locals.var_beta_dn0 * assign73910_e112615) + (locals.var_beta * (-locals.var_vxbgmtcl_dn0)))), (assign73910_e112617 * ((locals.var_beta_dn2 * assign73910_e112615) + (locals.var_beta * (-locals.var_vxbgmtcl_dn2)))), (assign73910_e112617 * ((locals.var_beta_dn4 * assign73910_e112615) + (locals.var_beta * (-locals.var_vxbgmtcl_dn4)))), (assign73910_e112617 * ((locals.var_beta_dn5 * assign73910_e112615) + (locals.var_beta * (-locals.var_vxbgmtcl_dn5)))), (assign73910_e112617 * ((locals.var_beta_dn6 * assign73910_e112615) + (locals.var_beta * (-locals.var_vxbgmtcl_dn6)))), (assign73910_e112617 * ((locals.var_beta_dn7 * assign73910_e112615) + (locals.var_beta * (-locals.var_vxbgmtcl_dn7)))), (assign73910_e112617 * ((locals.var_beta_dn8 * assign73910_e112615) + (locals.var_beta * (-locals.var_vxbgmtcl_dn8)))), (assign73910_e112617 * ((locals.var_beta_dn9 * assign73910_e112615) + (locals.var_beta * (-locals.var_vxbgmtcl_dn9)))), (assign73910_e112617 * ((locals.var_beta_dn10 * assign73910_e112615) + (locals.var_beta * (-locals.var_vxbgmtcl_dn10)))), (assign73910_e112617 * ((locals.var_beta_dn13 * assign73910_e112615) + (locals.var_beta * (-locals.var_vxbgmtcl_dn13)))),)
    } else {
        (locals.var_exp_bvbs, locals.var_exp_bvbs_dn0, locals.var_exp_bvbs_dn2, locals.var_exp_bvbs_dn4, locals.var_exp_bvbs_dn5, locals.var_exp_bvbs_dn6, locals.var_exp_bvbs_dn7, locals.var_exp_bvbs_dn8, locals.var_exp_bvbs_dn9, locals.var_exp_bvbs_dn10, locals.var_exp_bvbs_dn13,)
    }
};
        locals.var_exp_bvbs = assign73910_e112619;
        locals.var_exp_bvbs_dn0 = assign73910_e112619_d_n0;
        locals.var_exp_bvbs_dn2 = assign73910_e112619_d_n2;
        locals.var_exp_bvbs_dn4 = assign73910_e112619_d_n4;
        locals.var_exp_bvbs_dn5 = assign73910_e112619_d_n5;
        locals.var_exp_bvbs_dn6 = assign73910_e112619_d_n6;
        locals.var_exp_bvbs_dn7 = assign73910_e112619_d_n7;
        locals.var_exp_bvbs_dn8 = assign73910_e112619_d_n8;
        locals.var_exp_bvbs_dn9 = assign73910_e112619_d_n9;
        locals.var_exp_bvbs_dn10 = assign73910_e112619_d_n10;
        locals.var_exp_bvbs_dn13 = assign73910_e112619_d_n13;
        locals.var_exp_bvbs_rv = 0.0;

        let (assign73920_e112627, assign73920_e112627_d_n0, assign73920_e112627_d_n2, assign73920_e112627_d_n4, assign73920_e112627_d_n5, assign73920_e112627_d_n6, assign73920_e112627_d_n7, assign73920_e112627_d_n8, assign73920_e112627_d_n9, assign73920_e112627_d_n10, assign73920_e112627_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1722 != 0.0)) {
        let assign73920_e112625: f64 = (locals.var_nin / locals.var_nover_func);
        (assign73920_e112625, (locals.var_nin_dn0 / locals.var_nover_func), (locals.var_nin_dn2 / locals.var_nover_func), (locals.var_nin_dn4 / locals.var_nover_func), (locals.var_nin_dn5 / locals.var_nover_func), (locals.var_nin_dn6 / locals.var_nover_func), (locals.var_nin_dn7 / locals.var_nover_func), (locals.var_nin_dn8 / locals.var_nover_func), (locals.var_nin_dn9 / locals.var_nover_func), (locals.var_nin_dn10 / locals.var_nover_func), (locals.var_nin_dn13 / locals.var_nover_func),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign73920_e112627;
        locals.var_t0_dn0 = assign73920_e112627_d_n0;
        locals.var_t0_dn2 = assign73920_e112627_d_n2;
        locals.var_t0_dn4 = assign73920_e112627_d_n4;
        locals.var_t0_dn5 = assign73920_e112627_d_n5;
        locals.var_t0_dn6 = assign73920_e112627_d_n6;
        locals.var_t0_dn7 = assign73920_e112627_d_n7;
        locals.var_t0_dn8 = assign73920_e112627_d_n8;
        locals.var_t0_dn9 = assign73920_e112627_d_n9;
        locals.var_t0_dn10 = assign73920_e112627_d_n10;
        locals.var_t0_dn13 = assign73920_e112627_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign73930_e112635, assign73930_e112635_d_n0, assign73930_e112635_d_n2, assign73930_e112635_d_n4, assign73930_e112635_d_n5, assign73930_e112635_d_n6, assign73930_e112635_d_n7, assign73930_e112635_d_n8, assign73930_e112635_d_n9, assign73930_e112635_d_n10, assign73930_e112635_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1722 != 0.0)) {
        let assign73930_e112633: f64 = (locals.var_t0 * locals.var_t0);
        (assign73930_e112633, ((locals.var_t0_dn0 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn0)), ((locals.var_t0_dn2 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn2)), ((locals.var_t0_dn4 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn4)), ((locals.var_t0_dn5 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn5)), ((locals.var_t0_dn6 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn6)), ((locals.var_t0_dn7 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn7)), ((locals.var_t0_dn8 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn8)), ((locals.var_t0_dn9 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn9)), ((locals.var_t0_dn10 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn10)), ((locals.var_t0_dn13 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn13)),)
    } else {
        (locals.var_cnst1over, locals.var_cnst1over_dn0, locals.var_cnst1over_dn2, locals.var_cnst1over_dn4, locals.var_cnst1over_dn5, locals.var_cnst1over_dn6, locals.var_cnst1over_dn7, locals.var_cnst1over_dn8, locals.var_cnst1over_dn9, locals.var_cnst1over_dn10, locals.var_cnst1over_dn13,)
    }
};
        locals.var_cnst1over = assign73930_e112635;
        locals.var_cnst1over_dn0 = assign73930_e112635_d_n0;
        locals.var_cnst1over_dn2 = assign73930_e112635_d_n2;
        locals.var_cnst1over_dn4 = assign73930_e112635_d_n4;
        locals.var_cnst1over_dn5 = assign73930_e112635_d_n5;
        locals.var_cnst1over_dn6 = assign73930_e112635_d_n6;
        locals.var_cnst1over_dn7 = assign73930_e112635_d_n7;
        locals.var_cnst1over_dn8 = assign73930_e112635_d_n8;
        locals.var_cnst1over_dn9 = assign73930_e112635_d_n9;
        locals.var_cnst1over_dn10 = assign73930_e112635_d_n10;
        locals.var_cnst1over_dn13 = assign73930_e112635_d_n13;
        locals.var_cnst1over_rv = 0.0;

        let (assign73940_e112643, assign73940_e112643_d_n0, assign73940_e112643_d_n2, assign73940_e112643_d_n4, assign73940_e112643_d_n5, assign73940_e112643_d_n6, assign73940_e112643_d_n7, assign73940_e112643_d_n8, assign73940_e112643_d_n9, assign73940_e112643_d_n10, assign73940_e112643_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1722 != 0.0)) {
        let assign73940_e112641: f64 = (locals.var_cnst1over * locals.var_exp_bvbs);
        (assign73940_e112641, ((locals.var_cnst1over_dn0 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn0)), ((locals.var_cnst1over_dn2 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn2)), ((locals.var_cnst1over_dn4 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn4)), ((locals.var_cnst1over_dn5 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn5)), ((locals.var_cnst1over_dn6 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn6)), ((locals.var_cnst1over_dn7 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn7)), ((locals.var_cnst1over_dn8 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn8)), ((locals.var_cnst1over_dn9 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn9)), ((locals.var_cnst1over_dn10 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn10)), ((locals.var_cnst1over_dn13 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn13)),)
    } else {
        (locals.var_cfs1, locals.var_cfs1_dn0, locals.var_cfs1_dn2, locals.var_cfs1_dn4, locals.var_cfs1_dn5, locals.var_cfs1_dn6, locals.var_cfs1_dn7, locals.var_cfs1_dn8, locals.var_cfs1_dn9, locals.var_cfs1_dn10, locals.var_cfs1_dn13,)
    }
};
        locals.var_cfs1 = assign73940_e112643;
        locals.var_cfs1_dn0 = assign73940_e112643_d_n0;
        locals.var_cfs1_dn2 = assign73940_e112643_d_n2;
        locals.var_cfs1_dn4 = assign73940_e112643_d_n4;
        locals.var_cfs1_dn5 = assign73940_e112643_d_n5;
        locals.var_cfs1_dn6 = assign73940_e112643_d_n6;
        locals.var_cfs1_dn7 = assign73940_e112643_d_n7;
        locals.var_cfs1_dn8 = assign73940_e112643_d_n8;
        locals.var_cfs1_dn9 = assign73940_e112643_d_n9;
        locals.var_cfs1_dn10 = assign73940_e112643_d_n10;
        locals.var_cfs1_dn13 = assign73940_e112643_d_n13;
        locals.var_cfs1_rv = 0.0;

        let (assign73950_e112649, assign73950_e112649_d_n0, assign73950_e112649_d_n2, assign73950_e112649_d_n4, assign73950_e112649_d_n5, assign73950_e112649_d_n6, assign73950_e112649_d_n7, assign73950_e112649_d_n8, assign73950_e112649_d_n9, assign73950_e112649_d_n10, assign73950_e112649_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1722 != 0.0)) {
        (locals.var_ps0ld_ini, locals.var_ps0ld_ini_dn0, locals.var_ps0ld_ini_dn2, locals.var_ps0ld_ini_dn4, locals.var_ps0ld_ini_dn5, locals.var_ps0ld_ini_dn6, locals.var_ps0ld_ini_dn7, locals.var_ps0ld_ini_dn8, locals.var_ps0ld_ini_dn9, locals.var_ps0ld_ini_dn10, locals.var_ps0ld_ini_dn13,)
    } else {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn13,)
    }
};
        locals.var_ps0ld = assign73950_e112649;
        locals.var_ps0ld_dn0 = assign73950_e112649_d_n0;
        locals.var_ps0ld_dn2 = assign73950_e112649_d_n2;
        locals.var_ps0ld_dn4 = assign73950_e112649_d_n4;
        locals.var_ps0ld_dn5 = assign73950_e112649_d_n5;
        locals.var_ps0ld_dn6 = assign73950_e112649_d_n6;
        locals.var_ps0ld_dn7 = assign73950_e112649_d_n7;
        locals.var_ps0ld_dn8 = assign73950_e112649_d_n8;
        locals.var_ps0ld_dn9 = assign73950_e112649_d_n9;
        locals.var_ps0ld_dn10 = assign73950_e112649_d_n10;
        locals.var_ps0ld_dn13 = assign73950_e112649_d_n13;
        locals.var_ps0ld_rv = 0.0;

        let (assign73960_e112655,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1722 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_flg_conv,)
    }
};
        locals.var_flg_conv = assign73960_e112655;
        locals.var_flg_conv_rv = 0.0;

        let (assign73970_e112668, assign73970_e112668_d_n0, assign73970_e112668_d_n2, assign73970_e112668_d_n4, assign73970_e112668_d_n5, assign73970_e112668_d_n6, assign73970_e112668_d_n7, assign73970_e112668_d_n8, assign73970_e112668_d_n9, assign73970_e112668_d_n10, assign73970_e112668_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1722 != 0.0)) {
        let assign73970_e112662: f64 = (1.034943e-10 / locals.var_q_nsubld);
        let assign73970_e112664: f64 = (assign73970_e112662 * locals.var_beta_inv);
        let assign73970_e112665: f64 = (2.0 * assign73970_e112664);
        let assign73970_e112666: f64 = (assign73970_e112665).sqrt();
        (assign73970_e112666, ((2.0 * (assign73970_e112662 * locals.var_beta_inv_dn0)) / (2.0 * assign73970_e112666)), ((2.0 * (assign73970_e112662 * locals.var_beta_inv_dn2)) / (2.0 * assign73970_e112666)), ((2.0 * (assign73970_e112662 * locals.var_beta_inv_dn4)) / (2.0 * assign73970_e112666)), ((2.0 * (assign73970_e112662 * locals.var_beta_inv_dn5)) / (2.0 * assign73970_e112666)), ((2.0 * (assign73970_e112662 * locals.var_beta_inv_dn6)) / (2.0 * assign73970_e112666)), ((2.0 * (assign73970_e112662 * locals.var_beta_inv_dn7)) / (2.0 * assign73970_e112666)), ((2.0 * (assign73970_e112662 * locals.var_beta_inv_dn8)) / (2.0 * assign73970_e112666)), ((2.0 * (assign73970_e112662 * locals.var_beta_inv_dn9)) / (2.0 * assign73970_e112666)), ((2.0 * (assign73970_e112662 * locals.var_beta_inv_dn10)) / (2.0 * assign73970_e112666)), ((2.0 * (assign73970_e112662 * locals.var_beta_inv_dn13)) / (2.0 * assign73970_e112666)),)
    } else {
        (locals.var_c_w_ld, locals.var_c_w_ld_dn0, locals.var_c_w_ld_dn2, locals.var_c_w_ld_dn4, locals.var_c_w_ld_dn5, locals.var_c_w_ld_dn6, locals.var_c_w_ld_dn7, locals.var_c_w_ld_dn8, locals.var_c_w_ld_dn9, locals.var_c_w_ld_dn10, locals.var_c_w_ld_dn13,)
    }
};
        locals.var_c_w_ld = assign73970_e112668;
        locals.var_c_w_ld_dn0 = assign73970_e112668_d_n0;
        locals.var_c_w_ld_dn2 = assign73970_e112668_d_n2;
        locals.var_c_w_ld_dn4 = assign73970_e112668_d_n4;
        locals.var_c_w_ld_dn5 = assign73970_e112668_d_n5;
        locals.var_c_w_ld_dn6 = assign73970_e112668_d_n6;
        locals.var_c_w_ld_dn7 = assign73970_e112668_d_n7;
        locals.var_c_w_ld_dn8 = assign73970_e112668_d_n8;
        locals.var_c_w_ld_dn9 = assign73970_e112668_d_n9;
        locals.var_c_w_ld_dn10 = assign73970_e112668_d_n10;
        locals.var_c_w_ld_dn13 = assign73970_e112668_d_n13;
        locals.var_c_w_ld_rv = 0.0;

        let assign73980_e112671: f64 = if locals.var_wdep_func > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1723 = assign73980_e112671;
        locals.var_guard1723_rv = 0.0;

        let (assign73990_e112681, assign73990_e112681_d_n0, assign73990_e112681_d_n2, assign73990_e112681_d_n4, assign73990_e112681_d_n5, assign73990_e112681_d_n6, assign73990_e112681_d_n7, assign73990_e112681_d_n8, assign73990_e112681_d_n9, assign73990_e112681_d_n10, assign73990_e112681_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1722 != 0.0)) && (locals.var_guard1723 != 0.0)) {
        let assign73990_e112679: f64 = (p.p334 - locals.var_wdep_func);
        (assign73990_e112679, (-locals.var_wdep_func_dn0), (-locals.var_wdep_func_dn2), (-locals.var_wdep_func_dn4), (-locals.var_wdep_func_dn5), (-locals.var_wdep_func_dn6), (-locals.var_wdep_func_dn7), (-locals.var_wdep_func_dn8), (-locals.var_wdep_func_dn9), (-locals.var_wdep_func_dn10), (-locals.var_wdep_func_dn13),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign73990_e112681;
        locals.var_t2_dn0 = assign73990_e112681_d_n0;
        locals.var_t2_dn2 = assign73990_e112681_d_n2;
        locals.var_t2_dn4 = assign73990_e112681_d_n4;
        locals.var_t2_dn5 = assign73990_e112681_d_n5;
        locals.var_t2_dn6 = assign73990_e112681_d_n6;
        locals.var_t2_dn7 = assign73990_e112681_d_n7;
        locals.var_t2_dn8 = assign73990_e112681_d_n8;
        locals.var_t2_dn9 = assign73990_e112681_d_n9;
        locals.var_t2_dn10 = assign73990_e112681_d_n10;
        locals.var_t2_dn13 = assign73990_e112681_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign74000_e112703, assign74000_e112703_d_n0, assign74000_e112703_d_n2, assign74000_e112703_d_n4, assign74000_e112703_d_n5, assign74000_e112703_d_n6, assign74000_e112703_d_n7, assign74000_e112703_d_n8, assign74000_e112703_d_n9, assign74000_e112703_d_n10, assign74000_e112703_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1722 != 0.0)) && (locals.var_guard1723 == 0.0)) {
        let assign74000_e112690: f64 = (locals.var_vdsi + p.p137);
        let assign74000_e112693: f64 = (locals.var_vdsi + p.p137);
        let assign74000_e112694: f64 = (assign74000_e112690 * assign74000_e112693);
        let assign74000_e112697: f64 = (4.0 * 0.1);
        let assign74000_e112699: f64 = (assign74000_e112697 * 0.1);
        let assign74000_e112700: f64 = (assign74000_e112694 + assign74000_e112699);
        let assign74000_e112701: f64 = (assign74000_e112700).sqrt();
        (assign74000_e112701, 0.0, 0.0, 0.0, (((locals.var_vdsi_dn5 * assign74000_e112693) + (assign74000_e112690 * locals.var_vdsi_dn5)) / (2.0 * assign74000_e112701)), 0.0, (((locals.var_vdsi_dn7 * assign74000_e112693) + (assign74000_e112690 * locals.var_vdsi_dn7)) / (2.0 * assign74000_e112701)), 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign74000_e112703;
        locals.var_tmf2_dn0 = assign74000_e112703_d_n0;
        locals.var_tmf2_dn2 = assign74000_e112703_d_n2;
        locals.var_tmf2_dn4 = assign74000_e112703_d_n4;
        locals.var_tmf2_dn5 = assign74000_e112703_d_n5;
        locals.var_tmf2_dn6 = assign74000_e112703_d_n6;
        locals.var_tmf2_dn7 = assign74000_e112703_d_n7;
        locals.var_tmf2_dn8 = assign74000_e112703_d_n8;
        locals.var_tmf2_dn9 = assign74000_e112703_d_n9;
        locals.var_tmf2_dn10 = assign74000_e112703_d_n10;
        locals.var_tmf2_dn13 = assign74000_e112703_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign74010_e112720, assign74010_e112720_d_n0, assign74010_e112720_d_n2, assign74010_e112720_d_n4, assign74010_e112720_d_n5, assign74010_e112720_d_n6, assign74010_e112720_d_n7, assign74010_e112720_d_n8, assign74010_e112720_d_n9, assign74010_e112720_d_n10, assign74010_e112720_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1722 != 0.0)) && (locals.var_guard1723 == 0.0)) {
        let assign74010_e112714: f64 = (locals.var_vdsi + p.p137);
        let assign74010_e112716: f64 = (assign74010_e112714 / locals.var_tmf2);
        let assign74010_e112717: f64 = (1.0 + assign74010_e112716);
        let assign74010_e112718: f64 = (0.5 * assign74010_e112717);
        (assign74010_e112718, (0.5 * (-((assign74010_e112714 * locals.var_tmf2_dn0) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign74010_e112714 * locals.var_tmf2_dn2) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign74010_e112714 * locals.var_tmf2_dn4) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (((locals.var_vdsi_dn5 * locals.var_tmf2) - (assign74010_e112714 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (-((assign74010_e112714 * locals.var_tmf2_dn6) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (((locals.var_vdsi_dn7 * locals.var_tmf2) - (assign74010_e112714 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (-((assign74010_e112714 * locals.var_tmf2_dn8) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign74010_e112714 * locals.var_tmf2_dn9) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign74010_e112714 * locals.var_tmf2_dn10) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign74010_e112714 * locals.var_tmf2_dn13) / (locals.var_tmf2 * locals.var_tmf2)))),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign74010_e112720;
        locals.var_t9_dn0 = assign74010_e112720_d_n0;
        locals.var_t9_dn2 = assign74010_e112720_d_n2;
        locals.var_t9_dn4 = assign74010_e112720_d_n4;
        locals.var_t9_dn5 = assign74010_e112720_d_n5;
        locals.var_t9_dn6 = assign74010_e112720_d_n6;
        locals.var_t9_dn7 = assign74010_e112720_d_n7;
        locals.var_t9_dn8 = assign74010_e112720_d_n8;
        locals.var_t9_dn9 = assign74010_e112720_d_n9;
        locals.var_t9_dn10 = assign74010_e112720_d_n10;
        locals.var_t9_dn13 = assign74010_e112720_d_n13;
        locals.var_t9_rv = 0.0;

        let (assign74020_e112735, assign74020_e112735_d_n0, assign74020_e112735_d_n2, assign74020_e112735_d_n4, assign74020_e112735_d_n5, assign74020_e112735_d_n6, assign74020_e112735_d_n7, assign74020_e112735_d_n8, assign74020_e112735_d_n9, assign74020_e112735_d_n10, assign74020_e112735_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1722 != 0.0)) && (locals.var_guard1723 == 0.0)) {
        let assign74020_e112730: f64 = (locals.var_vdsi + p.p137);
        let assign74020_e112732: f64 = (assign74020_e112730 + locals.var_tmf2);
        let assign74020_e112733: f64 = (0.5 * assign74020_e112732);
        (assign74020_e112733, (0.5 * locals.var_tmf2_dn0), (0.5 * locals.var_tmf2_dn2), (0.5 * locals.var_tmf2_dn4), (0.5 * (locals.var_vdsi_dn5 + locals.var_tmf2_dn5)), (0.5 * locals.var_tmf2_dn6), (0.5 * (locals.var_vdsi_dn7 + locals.var_tmf2_dn7)), (0.5 * locals.var_tmf2_dn8), (0.5 * locals.var_tmf2_dn9), (0.5 * locals.var_tmf2_dn10), (0.5 * locals.var_tmf2_dn13),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign74020_e112735;
        locals.var_t2_dn0 = assign74020_e112735_d_n0;
        locals.var_t2_dn2 = assign74020_e112735_d_n2;
        locals.var_t2_dn4 = assign74020_e112735_d_n4;
        locals.var_t2_dn5 = assign74020_e112735_d_n5;
        locals.var_t2_dn6 = assign74020_e112735_d_n6;
        locals.var_t2_dn7 = assign74020_e112735_d_n7;
        locals.var_t2_dn8 = assign74020_e112735_d_n8;
        locals.var_t2_dn9 = assign74020_e112735_d_n9;
        locals.var_t2_dn10 = assign74020_e112735_d_n10;
        locals.var_t2_dn13 = assign74020_e112735_d_n13;
        locals.var_t2_rv = 0.0;

        let assign74030_e112738: f64 = if locals.var_t2 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1724 = assign74030_e112738;
        locals.var_guard1724_rv = 0.0;

        let (assign74040_e112749, assign74040_e112749_d_n0, assign74040_e112749_d_n2, assign74040_e112749_d_n4, assign74040_e112749_d_n5, assign74040_e112749_d_n6, assign74040_e112749_d_n7, assign74040_e112749_d_n8, assign74040_e112749_d_n9, assign74040_e112749_d_n10, assign74040_e112749_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1722 != 0.0)) && (locals.var_guard1723 == 0.0)) && (locals.var_guard1724 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign74040_e112749;
        locals.var_t2_dn0 = assign74040_e112749_d_n0;
        locals.var_t2_dn2 = assign74040_e112749_d_n2;
        locals.var_t2_dn4 = assign74040_e112749_d_n4;
        locals.var_t2_dn5 = assign74040_e112749_d_n5;
        locals.var_t2_dn6 = assign74040_e112749_d_n6;
        locals.var_t2_dn7 = assign74040_e112749_d_n7;
        locals.var_t2_dn8 = assign74040_e112749_d_n8;
        locals.var_t2_dn9 = assign74040_e112749_d_n9;
        locals.var_t2_dn10 = assign74040_e112749_d_n10;
        locals.var_t2_dn13 = assign74040_e112749_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign74050_e112760, assign74050_e112760_d_n0, assign74050_e112760_d_n2, assign74050_e112760_d_n4, assign74050_e112760_d_n5, assign74050_e112760_d_n6, assign74050_e112760_d_n7, assign74050_e112760_d_n8, assign74050_e112760_d_n9, assign74050_e112760_d_n10, assign74050_e112760_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1722 != 0.0)) && (locals.var_guard1723 == 0.0)) && (locals.var_guard1724 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign74050_e112760;
        locals.var_t9_dn0 = assign74050_e112760_d_n0;
        locals.var_t9_dn2 = assign74050_e112760_d_n2;
        locals.var_t9_dn4 = assign74050_e112760_d_n4;
        locals.var_t9_dn5 = assign74050_e112760_d_n5;
        locals.var_t9_dn6 = assign74050_e112760_d_n6;
        locals.var_t9_dn7 = assign74050_e112760_d_n7;
        locals.var_t9_dn8 = assign74050_e112760_d_n8;
        locals.var_t9_dn9 = assign74050_e112760_d_n9;
        locals.var_t9_dn10 = assign74050_e112760_d_n10;
        locals.var_t9_dn13 = assign74050_e112760_d_n13;
        locals.var_t9_rv = 0.0;

        let (assign74060_e112774, assign74060_e112774_d_n0, assign74060_e112774_d_n2, assign74060_e112774_d_n4, assign74060_e112774_d_n5, assign74060_e112774_d_n6, assign74060_e112774_d_n7, assign74060_e112774_d_n8, assign74060_e112774_d_n9, assign74060_e112774_d_n10, assign74060_e112774_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1722 != 0.0)) && (locals.var_guard1723 == 0.0)) {
        let assign74060_e112769: f64 = (locals.var_kjunc * locals.var_t2);
        let assign74060_e112770: f64 = (assign74060_e112769).sqrt();
        let assign74060_e112772: f64 = (assign74060_e112770 * p.p432);
        (assign74060_e112772, ((((locals.var_kjunc_dn0 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn0)) / (2.0 * assign74060_e112770)) * p.p432), ((((locals.var_kjunc_dn2 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn2)) / (2.0 * assign74060_e112770)) * p.p432), ((((locals.var_kjunc_dn4 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn4)) / (2.0 * assign74060_e112770)) * p.p432), ((((locals.var_kjunc_dn5 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn5)) / (2.0 * assign74060_e112770)) * p.p432), ((((locals.var_kjunc_dn6 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn6)) / (2.0 * assign74060_e112770)) * p.p432), ((((locals.var_kjunc_dn7 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn7)) / (2.0 * assign74060_e112770)) * p.p432), ((((locals.var_kjunc_dn8 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn8)) / (2.0 * assign74060_e112770)) * p.p432), ((((locals.var_kjunc_dn9 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn9)) / (2.0 * assign74060_e112770)) * p.p432), ((((locals.var_kjunc_dn10 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn10)) / (2.0 * assign74060_e112770)) * p.p432), ((((locals.var_kjunc_dn13 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn13)) / (2.0 * assign74060_e112770)) * p.p432),)
    } else {
        (locals.var_wjunc0, locals.var_wjunc0_dn0, locals.var_wjunc0_dn2, locals.var_wjunc0_dn4, locals.var_wjunc0_dn5, locals.var_wjunc0_dn6, locals.var_wjunc0_dn7, locals.var_wjunc0_dn8, locals.var_wjunc0_dn9, locals.var_wjunc0_dn10, locals.var_wjunc0_dn13,)
    }
};
        locals.var_wjunc0 = assign74060_e112774;
        locals.var_wjunc0_dn0 = assign74060_e112774_d_n0;
        locals.var_wjunc0_dn2 = assign74060_e112774_d_n2;
        locals.var_wjunc0_dn4 = assign74060_e112774_d_n4;
        locals.var_wjunc0_dn5 = assign74060_e112774_d_n5;
        locals.var_wjunc0_dn6 = assign74060_e112774_d_n6;
        locals.var_wjunc0_dn7 = assign74060_e112774_d_n7;
        locals.var_wjunc0_dn8 = assign74060_e112774_d_n8;
        locals.var_wjunc0_dn9 = assign74060_e112774_d_n9;
        locals.var_wjunc0_dn10 = assign74060_e112774_d_n10;
        locals.var_wjunc0_dn13 = assign74060_e112774_d_n13;
        locals.var_wjunc0_rv = 0.0;

        let (assign74070_e112785, assign74070_e112785_d_n0, assign74070_e112785_d_n2, assign74070_e112785_d_n4, assign74070_e112785_d_n5, assign74070_e112785_d_n6, assign74070_e112785_d_n7, assign74070_e112785_d_n8, assign74070_e112785_d_n9, assign74070_e112785_d_n10, assign74070_e112785_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1722 != 0.0)) && (locals.var_guard1723 == 0.0)) {
        let assign74070_e112783: f64 = (p.p334 - locals.var_wjunc0);
        (assign74070_e112783, (-locals.var_wjunc0_dn0), (-locals.var_wjunc0_dn2), (-locals.var_wjunc0_dn4), (-locals.var_wjunc0_dn5), (-locals.var_wjunc0_dn6), (-locals.var_wjunc0_dn7), (-locals.var_wjunc0_dn8), (-locals.var_wjunc0_dn9), (-locals.var_wjunc0_dn10), (-locals.var_wjunc0_dn13),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign74070_e112785;
        locals.var_t2_dn0 = assign74070_e112785_d_n0;
        locals.var_t2_dn2 = assign74070_e112785_d_n2;
        locals.var_t2_dn4 = assign74070_e112785_d_n4;
        locals.var_t2_dn5 = assign74070_e112785_d_n5;
        locals.var_t2_dn6 = assign74070_e112785_d_n6;
        locals.var_t2_dn7 = assign74070_e112785_d_n7;
        locals.var_t2_dn8 = assign74070_e112785_d_n8;
        locals.var_t2_dn9 = assign74070_e112785_d_n9;
        locals.var_t2_dn10 = assign74070_e112785_d_n10;
        locals.var_t2_dn13 = assign74070_e112785_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign74080_e112804, assign74080_e112804_d_n0, assign74080_e112804_d_n2, assign74080_e112804_d_n4, assign74080_e112804_d_n5, assign74080_e112804_d_n6, assign74080_e112804_d_n7, assign74080_e112804_d_n8, assign74080_e112804_d_n9, assign74080_e112804_d_n10, assign74080_e112804_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1722 != 0.0)) {
        let assign74080_e112791: f64 = (locals.var_t2 * locals.var_t2);
        let assign74080_e112795: f64 = (p.p334 * 0.01);
        let assign74080_e112796: f64 = (4.0 * assign74080_e112795);
        let assign74080_e112799: f64 = (p.p334 * 0.01);
        let assign74080_e112800: f64 = (assign74080_e112796 * assign74080_e112799);
        let assign74080_e112801: f64 = (assign74080_e112791 + assign74080_e112800);
        let assign74080_e112802: f64 = (assign74080_e112801).sqrt();
        (assign74080_e112802, (((locals.var_t2_dn0 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn0)) / (2.0 * assign74080_e112802)), (((locals.var_t2_dn2 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn2)) / (2.0 * assign74080_e112802)), (((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4)) / (2.0 * assign74080_e112802)), (((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5)) / (2.0 * assign74080_e112802)), (((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6)) / (2.0 * assign74080_e112802)), (((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7)) / (2.0 * assign74080_e112802)), (((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8)) / (2.0 * assign74080_e112802)), (((locals.var_t2_dn9 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn9)) / (2.0 * assign74080_e112802)), (((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10)) / (2.0 * assign74080_e112802)), (((locals.var_t2_dn13 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn13)) / (2.0 * assign74080_e112802)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign74080_e112804;
        locals.var_tmf2_dn0 = assign74080_e112804_d_n0;
        locals.var_tmf2_dn2 = assign74080_e112804_d_n2;
        locals.var_tmf2_dn4 = assign74080_e112804_d_n4;
        locals.var_tmf2_dn5 = assign74080_e112804_d_n5;
        locals.var_tmf2_dn6 = assign74080_e112804_d_n6;
        locals.var_tmf2_dn7 = assign74080_e112804_d_n7;
        locals.var_tmf2_dn8 = assign74080_e112804_d_n8;
        locals.var_tmf2_dn9 = assign74080_e112804_d_n9;
        locals.var_tmf2_dn10 = assign74080_e112804_d_n10;
        locals.var_tmf2_dn13 = assign74080_e112804_d_n13;
        locals.var_tmf2_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_268(
        locals: &mut StampLocals,
    ) {
        let (assign74090_e112816, assign74090_e112816_d_n0, assign74090_e112816_d_n2, assign74090_e112816_d_n4, assign74090_e112816_d_n5, assign74090_e112816_d_n6, assign74090_e112816_d_n7, assign74090_e112816_d_n8, assign74090_e112816_d_n9, assign74090_e112816_d_n10, assign74090_e112816_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1722 != 0.0)) {
        let assign74090_e112812: f64 = (locals.var_t2 / locals.var_tmf2);
        let assign74090_e112813: f64 = (1.0 + assign74090_e112812);
        let assign74090_e112814: f64 = (0.5 * assign74090_e112813);
        (assign74090_e112814, (0.5 * (((locals.var_t2_dn0 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn2 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn4 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn5 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn6 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn7 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn8 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn9 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn10 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn13 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign74090_e112816;
        locals.var_t9_dn0 = assign74090_e112816_d_n0;
        locals.var_t9_dn2 = assign74090_e112816_d_n2;
        locals.var_t9_dn4 = assign74090_e112816_d_n4;
        locals.var_t9_dn5 = assign74090_e112816_d_n5;
        locals.var_t9_dn6 = assign74090_e112816_d_n6;
        locals.var_t9_dn7 = assign74090_e112816_d_n7;
        locals.var_t9_dn8 = assign74090_e112816_d_n8;
        locals.var_t9_dn9 = assign74090_e112816_d_n9;
        locals.var_t9_dn10 = assign74090_e112816_d_n10;
        locals.var_t9_dn13 = assign74090_e112816_d_n13;
        locals.var_t9_rv = 0.0;

        let (assign74100_e112826, assign74100_e112826_d_n0, assign74100_e112826_d_n2, assign74100_e112826_d_n4, assign74100_e112826_d_n5, assign74100_e112826_d_n6, assign74100_e112826_d_n7, assign74100_e112826_d_n8, assign74100_e112826_d_n9, assign74100_e112826_d_n10, assign74100_e112826_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1722 != 0.0)) {
        let assign74100_e112823: f64 = (locals.var_t2 + locals.var_tmf2);
        let assign74100_e112824: f64 = (0.5 * assign74100_e112823);
        (assign74100_e112824, (0.5 * (locals.var_t2_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_t2_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_t2_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_t2_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_t2_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_t2_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_t2_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_t2_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_t2_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_t2_dn13 + locals.var_tmf2_dn13)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign74100_e112826;
        locals.var_t2_dn0 = assign74100_e112826_d_n0;
        locals.var_t2_dn2 = assign74100_e112826_d_n2;
        locals.var_t2_dn4 = assign74100_e112826_d_n4;
        locals.var_t2_dn5 = assign74100_e112826_d_n5;
        locals.var_t2_dn6 = assign74100_e112826_d_n6;
        locals.var_t2_dn7 = assign74100_e112826_d_n7;
        locals.var_t2_dn8 = assign74100_e112826_d_n8;
        locals.var_t2_dn9 = assign74100_e112826_d_n9;
        locals.var_t2_dn10 = assign74100_e112826_d_n10;
        locals.var_t2_dn13 = assign74100_e112826_d_n13;
        locals.var_t2_rv = 0.0;

        let assign74110_e112829: f64 = if locals.var_t2 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1725 = assign74110_e112829;
        locals.var_guard1725_rv = 0.0;

        let (assign74120_e112837, assign74120_e112837_d_n0, assign74120_e112837_d_n2, assign74120_e112837_d_n4, assign74120_e112837_d_n5, assign74120_e112837_d_n6, assign74120_e112837_d_n7, assign74120_e112837_d_n8, assign74120_e112837_d_n9, assign74120_e112837_d_n10, assign74120_e112837_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1722 != 0.0)) && (locals.var_guard1725 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign74120_e112837;
        locals.var_t2_dn0 = assign74120_e112837_d_n0;
        locals.var_t2_dn2 = assign74120_e112837_d_n2;
        locals.var_t2_dn4 = assign74120_e112837_d_n4;
        locals.var_t2_dn5 = assign74120_e112837_d_n5;
        locals.var_t2_dn6 = assign74120_e112837_d_n6;
        locals.var_t2_dn7 = assign74120_e112837_d_n7;
        locals.var_t2_dn8 = assign74120_e112837_d_n8;
        locals.var_t2_dn9 = assign74120_e112837_d_n9;
        locals.var_t2_dn10 = assign74120_e112837_d_n10;
        locals.var_t2_dn13 = assign74120_e112837_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign74130_e112845, assign74130_e112845_d_n0, assign74130_e112845_d_n2, assign74130_e112845_d_n4, assign74130_e112845_d_n5, assign74130_e112845_d_n6, assign74130_e112845_d_n7, assign74130_e112845_d_n8, assign74130_e112845_d_n9, assign74130_e112845_d_n10, assign74130_e112845_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1722 != 0.0)) && (locals.var_guard1725 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign74130_e112845;
        locals.var_t9_dn0 = assign74130_e112845_d_n0;
        locals.var_t9_dn2 = assign74130_e112845_d_n2;
        locals.var_t9_dn4 = assign74130_e112845_d_n4;
        locals.var_t9_dn5 = assign74130_e112845_d_n5;
        locals.var_t9_dn6 = assign74130_e112845_d_n6;
        locals.var_t9_dn7 = assign74130_e112845_d_n7;
        locals.var_t9_dn8 = assign74130_e112845_d_n8;
        locals.var_t9_dn9 = assign74130_e112845_d_n9;
        locals.var_t9_dn10 = assign74130_e112845_d_n10;
        locals.var_t9_dn13 = assign74130_e112845_d_n13;
        locals.var_t9_rv = 0.0;

        let (assign74140_e112851, assign74140_e112851_d_n0, assign74140_e112851_d_n2, assign74140_e112851_d_n4, assign74140_e112851_d_n5, assign74140_e112851_d_n6, assign74140_e112851_d_n7, assign74140_e112851_d_n8, assign74140_e112851_d_n9, assign74140_e112851_d_n10, assign74140_e112851_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1722 != 0.0)) {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    } else {
        (locals.var_ddriftldc, locals.var_ddriftldc_dn0, locals.var_ddriftldc_dn2, locals.var_ddriftldc_dn4, locals.var_ddriftldc_dn5, locals.var_ddriftldc_dn6, locals.var_ddriftldc_dn7, locals.var_ddriftldc_dn8, locals.var_ddriftldc_dn9, locals.var_ddriftldc_dn10, locals.var_ddriftldc_dn13,)
    }
};
        locals.var_ddriftldc = assign74140_e112851;
        locals.var_ddriftldc_dn0 = assign74140_e112851_d_n0;
        locals.var_ddriftldc_dn2 = assign74140_e112851_d_n2;
        locals.var_ddriftldc_dn4 = assign74140_e112851_d_n4;
        locals.var_ddriftldc_dn5 = assign74140_e112851_d_n5;
        locals.var_ddriftldc_dn6 = assign74140_e112851_d_n6;
        locals.var_ddriftldc_dn7 = assign74140_e112851_d_n7;
        locals.var_ddriftldc_dn8 = assign74140_e112851_d_n8;
        locals.var_ddriftldc_dn9 = assign74140_e112851_d_n9;
        locals.var_ddriftldc_dn10 = assign74140_e112851_d_n10;
        locals.var_ddriftldc_dn13 = assign74140_e112851_d_n13;
        locals.var_ddriftldc_rv = 0.0;

        let (assign74150_e112865, assign74150_e112865_d_n0, assign74150_e112865_d_n2, assign74150_e112865_d_n4, assign74150_e112865_d_n5, assign74150_e112865_d_n6, assign74150_e112865_d_n7, assign74150_e112865_d_n8, assign74150_e112865_d_n9, assign74150_e112865_d_n10, assign74150_e112865_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1722 != 0.0)) {
        let assign74150_e112857: f64 = (locals.var_q_nsubld * locals.var_ddriftldc);
        let assign74150_e112859: f64 = (assign74150_e112857 * locals.var_ddriftldc);
        let assign74150_e112861: f64 = (assign74150_e112859 / 2.0);
        let assign74150_e112863: f64 = (assign74150_e112861 / 1.034943e-10);
        (assign74150_e112863, (((((locals.var_q_nsubld * locals.var_ddriftldc_dn0) * locals.var_ddriftldc) + (assign74150_e112857 * locals.var_ddriftldc_dn0)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld * locals.var_ddriftldc_dn2) * locals.var_ddriftldc) + (assign74150_e112857 * locals.var_ddriftldc_dn2)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld * locals.var_ddriftldc_dn4) * locals.var_ddriftldc) + (assign74150_e112857 * locals.var_ddriftldc_dn4)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld * locals.var_ddriftldc_dn5) * locals.var_ddriftldc) + (assign74150_e112857 * locals.var_ddriftldc_dn5)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld * locals.var_ddriftldc_dn6) * locals.var_ddriftldc) + (assign74150_e112857 * locals.var_ddriftldc_dn6)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld * locals.var_ddriftldc_dn7) * locals.var_ddriftldc) + (assign74150_e112857 * locals.var_ddriftldc_dn7)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld * locals.var_ddriftldc_dn8) * locals.var_ddriftldc) + (assign74150_e112857 * locals.var_ddriftldc_dn8)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld * locals.var_ddriftldc_dn9) * locals.var_ddriftldc) + (assign74150_e112857 * locals.var_ddriftldc_dn9)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld * locals.var_ddriftldc_dn10) * locals.var_ddriftldc) + (assign74150_e112857 * locals.var_ddriftldc_dn10)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld * locals.var_ddriftldc_dn13) * locals.var_ddriftldc) + (assign74150_e112857 * locals.var_ddriftldc_dn13)) / 2.0) / 1.034943e-10),)
    } else {
        (locals.var_dphi_sb, locals.var_dphi_sb_dn0, locals.var_dphi_sb_dn2, locals.var_dphi_sb_dn4, locals.var_dphi_sb_dn5, locals.var_dphi_sb_dn6, locals.var_dphi_sb_dn7, locals.var_dphi_sb_dn8, locals.var_dphi_sb_dn9, locals.var_dphi_sb_dn10, locals.var_dphi_sb_dn13,)
    }
};
        locals.var_dphi_sb = assign74150_e112865;
        locals.var_dphi_sb_dn0 = assign74150_e112865_d_n0;
        locals.var_dphi_sb_dn2 = assign74150_e112865_d_n2;
        locals.var_dphi_sb_dn4 = assign74150_e112865_d_n4;
        locals.var_dphi_sb_dn5 = assign74150_e112865_d_n5;
        locals.var_dphi_sb_dn6 = assign74150_e112865_d_n6;
        locals.var_dphi_sb_dn7 = assign74150_e112865_d_n7;
        locals.var_dphi_sb_dn8 = assign74150_e112865_d_n8;
        locals.var_dphi_sb_dn9 = assign74150_e112865_d_n9;
        locals.var_dphi_sb_dn10 = assign74150_e112865_d_n10;
        locals.var_dphi_sb_dn13 = assign74150_e112865_d_n13;
        locals.var_dphi_sb_rv = 0.0;

        let (assign74160_e112876, assign74160_e112876_d_n0, assign74160_e112876_d_n2, assign74160_e112876_d_n4, assign74160_e112876_d_n5, assign74160_e112876_d_n6, assign74160_e112876_d_n7, assign74160_e112876_d_n8, assign74160_e112876_d_n9, assign74160_e112876_d_n10, assign74160_e112876_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1722 != 0.0)) {
        let assign74160_e112871: f64 = (2.0 * locals.var_beta);
        let assign74160_e112873: f64 = (assign74160_e112871 * locals.var_dphi_sb);
        let assign74160_e112874: f64 = (assign74160_e112873).sqrt();
        (assign74160_e112874, ((((2.0 * locals.var_beta_dn0) * locals.var_dphi_sb) + (assign74160_e112871 * locals.var_dphi_sb_dn0)) / (2.0 * assign74160_e112874)), ((((2.0 * locals.var_beta_dn2) * locals.var_dphi_sb) + (assign74160_e112871 * locals.var_dphi_sb_dn2)) / (2.0 * assign74160_e112874)), ((((2.0 * locals.var_beta_dn4) * locals.var_dphi_sb) + (assign74160_e112871 * locals.var_dphi_sb_dn4)) / (2.0 * assign74160_e112874)), ((((2.0 * locals.var_beta_dn5) * locals.var_dphi_sb) + (assign74160_e112871 * locals.var_dphi_sb_dn5)) / (2.0 * assign74160_e112874)), ((((2.0 * locals.var_beta_dn6) * locals.var_dphi_sb) + (assign74160_e112871 * locals.var_dphi_sb_dn6)) / (2.0 * assign74160_e112874)), ((((2.0 * locals.var_beta_dn7) * locals.var_dphi_sb) + (assign74160_e112871 * locals.var_dphi_sb_dn7)) / (2.0 * assign74160_e112874)), ((((2.0 * locals.var_beta_dn8) * locals.var_dphi_sb) + (assign74160_e112871 * locals.var_dphi_sb_dn8)) / (2.0 * assign74160_e112874)), ((((2.0 * locals.var_beta_dn9) * locals.var_dphi_sb) + (assign74160_e112871 * locals.var_dphi_sb_dn9)) / (2.0 * assign74160_e112874)), ((((2.0 * locals.var_beta_dn10) * locals.var_dphi_sb) + (assign74160_e112871 * locals.var_dphi_sb_dn10)) / (2.0 * assign74160_e112874)), ((((2.0 * locals.var_beta_dn13) * locals.var_dphi_sb) + (assign74160_e112871 * locals.var_dphi_sb_dn13)) / (2.0 * assign74160_e112874)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign74160_e112876;
        locals.var_t0_dn0 = assign74160_e112876_d_n0;
        locals.var_t0_dn2 = assign74160_e112876_d_n2;
        locals.var_t0_dn4 = assign74160_e112876_d_n4;
        locals.var_t0_dn5 = assign74160_e112876_d_n5;
        locals.var_t0_dn6 = assign74160_e112876_d_n6;
        locals.var_t0_dn7 = assign74160_e112876_d_n7;
        locals.var_t0_dn8 = assign74160_e112876_d_n8;
        locals.var_t0_dn9 = assign74160_e112876_d_n9;
        locals.var_t0_dn10 = assign74160_e112876_d_n10;
        locals.var_t0_dn13 = assign74160_e112876_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign74170_e112889, assign74170_e112889_d_n0, assign74170_e112889_d_n2, assign74170_e112889_d_n4, assign74170_e112889_d_n5, assign74170_e112889_d_n6, assign74170_e112889_d_n7, assign74170_e112889_d_n8, assign74170_e112889_d_n9, assign74170_e112889_d_n10, assign74170_e112889_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1722 != 0.0)) {
        let assign74170_e112881: f64 = { let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign74170_e112883: f64 = (-locals.var_t0);
        let assign74170_e112884: f64 = { let limited_exp_arg = assign74170_e112883; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign74170_e112885: f64 = (assign74170_e112881 + assign74170_e112884);
        let assign74170_e112887: f64 = (assign74170_e112885 / 2.0);
        (assign74170_e112887, ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn0) + ({ let limited_exp_arg = assign74170_e112883; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn0))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn2) + ({ let limited_exp_arg = assign74170_e112883; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn2))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn4) + ({ let limited_exp_arg = assign74170_e112883; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn4))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn5) + ({ let limited_exp_arg = assign74170_e112883; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn5))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn6) + ({ let limited_exp_arg = assign74170_e112883; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn6))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn7) + ({ let limited_exp_arg = assign74170_e112883; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn7))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn8) + ({ let limited_exp_arg = assign74170_e112883; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn8))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn9) + ({ let limited_exp_arg = assign74170_e112883; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn9))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn10) + ({ let limited_exp_arg = assign74170_e112883; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn10))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn13) + ({ let limited_exp_arg = assign74170_e112883; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn13))) / 2.0),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign74170_e112889;
        locals.var_t1_dn0 = assign74170_e112889_d_n0;
        locals.var_t1_dn2 = assign74170_e112889_d_n2;
        locals.var_t1_dn4 = assign74170_e112889_d_n4;
        locals.var_t1_dn5 = assign74170_e112889_d_n5;
        locals.var_t1_dn6 = assign74170_e112889_d_n6;
        locals.var_t1_dn7 = assign74170_e112889_d_n7;
        locals.var_t1_dn8 = assign74170_e112889_d_n8;
        locals.var_t1_dn9 = assign74170_e112889_d_n9;
        locals.var_t1_dn10 = assign74170_e112889_d_n10;
        locals.var_t1_dn13 = assign74170_e112889_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign74180_e112898, assign74180_e112898_d_n0, assign74180_e112898_d_n2, assign74180_e112898_d_n4, assign74180_e112898_d_n5, assign74180_e112898_d_n6, assign74180_e112898_d_n7, assign74180_e112898_d_n8, assign74180_e112898_d_n9, assign74180_e112898_d_n10, assign74180_e112898_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1722 != 0.0)) {
        let assign74180_e112894: f64 = (locals.var_t1).ln();
        let assign74180_e112896: f64 = (assign74180_e112894 / locals.var_dphi_sb);
        (assign74180_e112896, ((((locals.var_t1_dn0 / locals.var_t1) * locals.var_dphi_sb) - (assign74180_e112894 * locals.var_dphi_sb_dn0)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn2 / locals.var_t1) * locals.var_dphi_sb) - (assign74180_e112894 * locals.var_dphi_sb_dn2)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn4 / locals.var_t1) * locals.var_dphi_sb) - (assign74180_e112894 * locals.var_dphi_sb_dn4)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn5 / locals.var_t1) * locals.var_dphi_sb) - (assign74180_e112894 * locals.var_dphi_sb_dn5)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn6 / locals.var_t1) * locals.var_dphi_sb) - (assign74180_e112894 * locals.var_dphi_sb_dn6)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn7 / locals.var_t1) * locals.var_dphi_sb) - (assign74180_e112894 * locals.var_dphi_sb_dn7)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn8 / locals.var_t1) * locals.var_dphi_sb) - (assign74180_e112894 * locals.var_dphi_sb_dn8)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn9 / locals.var_t1) * locals.var_dphi_sb) - (assign74180_e112894 * locals.var_dphi_sb_dn9)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn10 / locals.var_t1) * locals.var_dphi_sb) - (assign74180_e112894 * locals.var_dphi_sb_dn10)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn13 / locals.var_t1) * locals.var_dphi_sb) - (assign74180_e112894 * locals.var_dphi_sb_dn13)) / (locals.var_dphi_sb * locals.var_dphi_sb)),)
    } else {
        (locals.var_c_sb, locals.var_c_sb_dn0, locals.var_c_sb_dn2, locals.var_c_sb_dn4, locals.var_c_sb_dn5, locals.var_c_sb_dn6, locals.var_c_sb_dn7, locals.var_c_sb_dn8, locals.var_c_sb_dn9, locals.var_c_sb_dn10, locals.var_c_sb_dn13,)
    }
};
        locals.var_c_sb = assign74180_e112898;
        locals.var_c_sb_dn0 = assign74180_e112898_d_n0;
        locals.var_c_sb_dn2 = assign74180_e112898_d_n2;
        locals.var_c_sb_dn4 = assign74180_e112898_d_n4;
        locals.var_c_sb_dn5 = assign74180_e112898_d_n5;
        locals.var_c_sb_dn6 = assign74180_e112898_d_n6;
        locals.var_c_sb_dn7 = assign74180_e112898_d_n7;
        locals.var_c_sb_dn8 = assign74180_e112898_d_n8;
        locals.var_c_sb_dn9 = assign74180_e112898_d_n9;
        locals.var_c_sb_dn10 = assign74180_e112898_d_n10;
        locals.var_c_sb_dn13 = assign74180_e112898_d_n13;
        locals.var_c_sb_rv = 0.0;

        let (assign74190_e112904,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1722 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_lp_s0,)
    }
};
        locals.var_lp_s0 = assign74190_e112904;
        locals.var_lp_s0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_269(
        locals: &mut StampLocals,
    ) {
        let mut assign74200_loop_guard: usize = 0;
        while {
            let assign74200_cond_e112911: f64 = (locals.var_lp_s0_max + 1.0);
            let assign74200_cond_e112913: f64 = if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1722 != 0.0)) && (locals.var_lp_s0 <= assign74200_cond_e112911)) { 1.0 } else { 0.0 };
            assign74200_cond_e112913 != 0.0
        } {
            assign74200_loop_guard += 1;
            assert!(assign74200_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign74200_body3_e112940, assign74200_body3_e112940_d_n0, assign74200_body3_e112940_d_n2, assign74200_body3_e112940_d_n4, assign74200_body3_e112940_d_n5, assign74200_body3_e112940_d_n6, assign74200_body3_e112940_d_n7, assign74200_body3_e112940_d_n8, assign74200_body3_e112940_d_n9, assign74200_body3_e112940_d_n10, assign74200_body3_e112940_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1722 != 0.0)) {
        let assign74200_body3_e112938: f64 = (locals.var_ps0ld + locals.var_vxbgmtcl);
        (assign74200_body3_e112938, (locals.var_ps0ld_dn0 + locals.var_vxbgmtcl_dn0), (locals.var_ps0ld_dn2 + locals.var_vxbgmtcl_dn2), (locals.var_ps0ld_dn4 + locals.var_vxbgmtcl_dn4), (locals.var_ps0ld_dn5 + locals.var_vxbgmtcl_dn5), (locals.var_ps0ld_dn6 + locals.var_vxbgmtcl_dn6), (locals.var_ps0ld_dn7 + locals.var_vxbgmtcl_dn7), (locals.var_ps0ld_dn8 + locals.var_vxbgmtcl_dn8), (locals.var_ps0ld_dn9 + locals.var_vxbgmtcl_dn9), (locals.var_ps0ld_dn10 + locals.var_vxbgmtcl_dn10), (locals.var_ps0ld_dn13 + locals.var_vxbgmtcl_dn13),)
    } else {
        (locals.var_ps0ld_vxb, locals.var_ps0ld_vxb_dn0, locals.var_ps0ld_vxb_dn2, locals.var_ps0ld_vxb_dn4, locals.var_ps0ld_vxb_dn5, locals.var_ps0ld_vxb_dn6, locals.var_ps0ld_vxb_dn7, locals.var_ps0ld_vxb_dn8, locals.var_ps0ld_vxb_dn9, locals.var_ps0ld_vxb_dn10, locals.var_ps0ld_vxb_dn13,)
    }
};
            locals.var_ps0ld_vxb = assign74200_body3_e112940;
            locals.var_ps0ld_vxb_dn0 = assign74200_body3_e112940_d_n0;
            locals.var_ps0ld_vxb_dn2 = assign74200_body3_e112940_d_n2;
            locals.var_ps0ld_vxb_dn4 = assign74200_body3_e112940_d_n4;
            locals.var_ps0ld_vxb_dn5 = assign74200_body3_e112940_d_n5;
            locals.var_ps0ld_vxb_dn6 = assign74200_body3_e112940_d_n6;
            locals.var_ps0ld_vxb_dn7 = assign74200_body3_e112940_d_n7;
            locals.var_ps0ld_vxb_dn8 = assign74200_body3_e112940_d_n8;
            locals.var_ps0ld_vxb_dn9 = assign74200_body3_e112940_d_n9;
            locals.var_ps0ld_vxb_dn10 = assign74200_body3_e112940_d_n10;
            locals.var_ps0ld_vxb_dn13 = assign74200_body3_e112940_d_n13;
            locals.var_ps0ld_vxb_rv = 0.0;
            let (assign74200_body4_e112948, assign74200_body4_e112948_d_n0, assign74200_body4_e112948_d_n2, assign74200_body4_e112948_d_n4, assign74200_body4_e112948_d_n5, assign74200_body4_e112948_d_n6, assign74200_body4_e112948_d_n7, assign74200_body4_e112948_d_n8, assign74200_body4_e112948_d_n9, assign74200_body4_e112948_d_n10, assign74200_body4_e112948_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1722 != 0.0)) {
        let assign74200_body4_e112946: f64 = (locals.var_beta * locals.var_ps0ld_vxb);
        (assign74200_body4_e112946, ((locals.var_beta_dn0 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn0)), ((locals.var_beta_dn2 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn2)), ((locals.var_beta_dn4 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn4)), ((locals.var_beta_dn5 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn5)), ((locals.var_beta_dn6 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn6)), ((locals.var_beta_dn7 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn7)), ((locals.var_beta_dn8 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn8)), ((locals.var_beta_dn9 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn9)), ((locals.var_beta_dn10 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn10)), ((locals.var_beta_dn13 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn13)),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn13,)
    }
};
            locals.var_chi = assign74200_body4_e112948;
            locals.var_chi_dn0 = assign74200_body4_e112948_d_n0;
            locals.var_chi_dn2 = assign74200_body4_e112948_d_n2;
            locals.var_chi_dn4 = assign74200_body4_e112948_d_n4;
            locals.var_chi_dn5 = assign74200_body4_e112948_d_n5;
            locals.var_chi_dn6 = assign74200_body4_e112948_d_n6;
            locals.var_chi_dn7 = assign74200_body4_e112948_d_n7;
            locals.var_chi_dn8 = assign74200_body4_e112948_d_n8;
            locals.var_chi_dn9 = assign74200_body4_e112948_d_n9;
            locals.var_chi_dn10 = assign74200_body4_e112948_d_n10;
            locals.var_chi_dn13 = assign74200_body4_e112948_d_n13;
            locals.var_chi_rv = 0.0;
            let (assign74200_body5_e112958, assign74200_body5_e112958_d_n0, assign74200_body5_e112958_d_n2, assign74200_body5_e112958_d_n4, assign74200_body5_e112958_d_n5, assign74200_body5_e112958_d_n6, assign74200_body5_e112958_d_n7, assign74200_body5_e112958_d_n8, assign74200_body5_e112958_d_n9, assign74200_body5_e112958_d_n10, assign74200_body5_e112958_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1722 != 0.0)) {
        let assign74200_body5_e112955: f64 = (locals.var_ps0ld_vxb - locals.var_dphi_sb);
        let assign74200_body5_e112956: f64 = (locals.var_c_sb * assign74200_body5_e112955);
        (assign74200_body5_e112956, ((locals.var_c_sb_dn0 * assign74200_body5_e112955) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn0 - locals.var_dphi_sb_dn0))), ((locals.var_c_sb_dn2 * assign74200_body5_e112955) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn2 - locals.var_dphi_sb_dn2))), ((locals.var_c_sb_dn4 * assign74200_body5_e112955) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn4 - locals.var_dphi_sb_dn4))), ((locals.var_c_sb_dn5 * assign74200_body5_e112955) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn5 - locals.var_dphi_sb_dn5))), ((locals.var_c_sb_dn6 * assign74200_body5_e112955) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn6 - locals.var_dphi_sb_dn6))), ((locals.var_c_sb_dn7 * assign74200_body5_e112955) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn7 - locals.var_dphi_sb_dn7))), ((locals.var_c_sb_dn8 * assign74200_body5_e112955) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn8 - locals.var_dphi_sb_dn8))), ((locals.var_c_sb_dn9 * assign74200_body5_e112955) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn9 - locals.var_dphi_sb_dn9))), ((locals.var_c_sb_dn10 * assign74200_body5_e112955) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn10 - locals.var_dphi_sb_dn10))), ((locals.var_c_sb_dn13 * assign74200_body5_e112955) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn13 - locals.var_dphi_sb_dn13))),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn4, locals.var_ty_dn5, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn8, locals.var_ty_dn9, locals.var_ty_dn10, locals.var_ty_dn13,)
    }
};
            locals.var_ty = assign74200_body5_e112958;
            locals.var_ty_dn0 = assign74200_body5_e112958_d_n0;
            locals.var_ty_dn2 = assign74200_body5_e112958_d_n2;
            locals.var_ty_dn4 = assign74200_body5_e112958_d_n4;
            locals.var_ty_dn5 = assign74200_body5_e112958_d_n5;
            locals.var_ty_dn6 = assign74200_body5_e112958_d_n6;
            locals.var_ty_dn7 = assign74200_body5_e112958_d_n7;
            locals.var_ty_dn8 = assign74200_body5_e112958_d_n8;
            locals.var_ty_dn9 = assign74200_body5_e112958_d_n9;
            locals.var_ty_dn10 = assign74200_body5_e112958_d_n10;
            locals.var_ty_dn13 = assign74200_body5_e112958_d_n13;
            locals.var_ty_rv = 0.0;
            let assign74200_body6_e112961: f64 = if locals.var_ty < 60.0 { 1.0 } else { 0.0 };
            locals.var_guard1727 = assign74200_body6_e112961;
            locals.var_guard1727_rv = 0.0;
            let (assign74200_body7_e112970, assign74200_body7_e112970_d_n0, assign74200_body7_e112970_d_n2, assign74200_body7_e112970_d_n4, assign74200_body7_e112970_d_n5, assign74200_body7_e112970_d_n6, assign74200_body7_e112970_d_n7, assign74200_body7_e112970_d_n8, assign74200_body7_e112970_d_n9, assign74200_body7_e112970_d_n10, assign74200_body7_e112970_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1722 != 0.0)) && (locals.var_guard1727 != 0.0)) {
        let assign74200_body7_e112968: f64 = (locals.var_ty).exp();
        (assign74200_body7_e112968, (assign74200_body7_e112968 * locals.var_ty_dn0), (assign74200_body7_e112968 * locals.var_ty_dn2), (assign74200_body7_e112968 * locals.var_ty_dn4), (assign74200_body7_e112968 * locals.var_ty_dn5), (assign74200_body7_e112968 * locals.var_ty_dn6), (assign74200_body7_e112968 * locals.var_ty_dn7), (assign74200_body7_e112968 * locals.var_ty_dn8), (assign74200_body7_e112968 * locals.var_ty_dn9), (assign74200_body7_e112968 * locals.var_ty_dn10), (assign74200_body7_e112968 * locals.var_ty_dn13),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
            locals.var_t1 = assign74200_body7_e112970;
            locals.var_t1_dn0 = assign74200_body7_e112970_d_n0;
            locals.var_t1_dn2 = assign74200_body7_e112970_d_n2;
            locals.var_t1_dn4 = assign74200_body7_e112970_d_n4;
            locals.var_t1_dn5 = assign74200_body7_e112970_d_n5;
            locals.var_t1_dn6 = assign74200_body7_e112970_d_n6;
            locals.var_t1_dn7 = assign74200_body7_e112970_d_n7;
            locals.var_t1_dn8 = assign74200_body7_e112970_d_n8;
            locals.var_t1_dn9 = assign74200_body7_e112970_d_n9;
            locals.var_t1_dn10 = assign74200_body7_e112970_d_n10;
            locals.var_t1_dn13 = assign74200_body7_e112970_d_n13;
            locals.var_t1_rv = 0.0;
            let (assign74200_body8_e112982, assign74200_body8_e112982_d_n0, assign74200_body8_e112982_d_n2, assign74200_body8_e112982_d_n4, assign74200_body8_e112982_d_n5, assign74200_body8_e112982_d_n6, assign74200_body8_e112982_d_n7, assign74200_body8_e112982_d_n8, assign74200_body8_e112982_d_n9, assign74200_body8_e112982_d_n10, assign74200_body8_e112982_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1722 != 0.0)) && (locals.var_guard1727 != 0.0)) {
        let assign74200_body8_e112977: f64 = (-locals.var_c_sb);
        let assign74200_body8_e112979: f64 = (assign74200_body8_e112977 * locals.var_dphi_sb);
        let assign74200_body8_e112980: f64 = (assign74200_body8_e112979).exp();
        (assign74200_body8_e112980, (assign74200_body8_e112980 * (((-locals.var_c_sb_dn0) * locals.var_dphi_sb) + (assign74200_body8_e112977 * locals.var_dphi_sb_dn0))), (assign74200_body8_e112980 * (((-locals.var_c_sb_dn2) * locals.var_dphi_sb) + (assign74200_body8_e112977 * locals.var_dphi_sb_dn2))), (assign74200_body8_e112980 * (((-locals.var_c_sb_dn4) * locals.var_dphi_sb) + (assign74200_body8_e112977 * locals.var_dphi_sb_dn4))), (assign74200_body8_e112980 * (((-locals.var_c_sb_dn5) * locals.var_dphi_sb) + (assign74200_body8_e112977 * locals.var_dphi_sb_dn5))), (assign74200_body8_e112980 * (((-locals.var_c_sb_dn6) * locals.var_dphi_sb) + (assign74200_body8_e112977 * locals.var_dphi_sb_dn6))), (assign74200_body8_e112980 * (((-locals.var_c_sb_dn7) * locals.var_dphi_sb) + (assign74200_body8_e112977 * locals.var_dphi_sb_dn7))), (assign74200_body8_e112980 * (((-locals.var_c_sb_dn8) * locals.var_dphi_sb) + (assign74200_body8_e112977 * locals.var_dphi_sb_dn8))), (assign74200_body8_e112980 * (((-locals.var_c_sb_dn9) * locals.var_dphi_sb) + (assign74200_body8_e112977 * locals.var_dphi_sb_dn9))), (assign74200_body8_e112980 * (((-locals.var_c_sb_dn10) * locals.var_dphi_sb) + (assign74200_body8_e112977 * locals.var_dphi_sb_dn10))), (assign74200_body8_e112980 * (((-locals.var_c_sb_dn13) * locals.var_dphi_sb) + (assign74200_body8_e112977 * locals.var_dphi_sb_dn13))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
            locals.var_t0 = assign74200_body8_e112982;
            locals.var_t0_dn0 = assign74200_body8_e112982_d_n0;
            locals.var_t0_dn2 = assign74200_body8_e112982_d_n2;
            locals.var_t0_dn4 = assign74200_body8_e112982_d_n4;
            locals.var_t0_dn5 = assign74200_body8_e112982_d_n5;
            locals.var_t0_dn6 = assign74200_body8_e112982_d_n6;
            locals.var_t0_dn7 = assign74200_body8_e112982_d_n7;
            locals.var_t0_dn8 = assign74200_body8_e112982_d_n8;
            locals.var_t0_dn9 = assign74200_body8_e112982_d_n9;
            locals.var_t0_dn10 = assign74200_body8_e112982_d_n10;
            locals.var_t0_dn13 = assign74200_body8_e112982_d_n13;
            locals.var_t0_rv = 0.0;
            let (assign74200_body9_e112992, assign74200_body9_e112992_d_n0, assign74200_body9_e112992_d_n2, assign74200_body9_e112992_d_n4, assign74200_body9_e112992_d_n5, assign74200_body9_e112992_d_n6, assign74200_body9_e112992_d_n7, assign74200_body9_e112992_d_n8, assign74200_body9_e112992_d_n9, assign74200_body9_e112992_d_n10, assign74200_body9_e112992_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1722 != 0.0)) && (locals.var_guard1727 != 0.0)) {
        let assign74200_body9_e112990: f64 = (locals.var_t1 - locals.var_t0);
        (assign74200_body9_e112990, (locals.var_t1_dn0 - locals.var_t0_dn0), (locals.var_t1_dn2 - locals.var_t0_dn2), (locals.var_t1_dn4 - locals.var_t0_dn4), (locals.var_t1_dn5 - locals.var_t0_dn5), (locals.var_t1_dn6 - locals.var_t0_dn6), (locals.var_t1_dn7 - locals.var_t0_dn7), (locals.var_t1_dn8 - locals.var_t0_dn8), (locals.var_t1_dn9 - locals.var_t0_dn9), (locals.var_t1_dn10 - locals.var_t0_dn10), (locals.var_t1_dn13 - locals.var_t0_dn13),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
            locals.var_t2 = assign74200_body9_e112992;
            locals.var_t2_dn0 = assign74200_body9_e112992_d_n0;
            locals.var_t2_dn2 = assign74200_body9_e112992_d_n2;
            locals.var_t2_dn4 = assign74200_body9_e112992_d_n4;
            locals.var_t2_dn5 = assign74200_body9_e112992_d_n5;
            locals.var_t2_dn6 = assign74200_body9_e112992_d_n6;
            locals.var_t2_dn7 = assign74200_body9_e112992_d_n7;
            locals.var_t2_dn8 = assign74200_body9_e112992_d_n8;
            locals.var_t2_dn9 = assign74200_body9_e112992_d_n9;
            locals.var_t2_dn10 = assign74200_body9_e112992_d_n10;
            locals.var_t2_dn13 = assign74200_body9_e112992_d_n13;
            locals.var_t2_rv = 0.0;
            let (assign74200_body10_e113005, assign74200_body10_e113005_d_n0, assign74200_body10_e113005_d_n2, assign74200_body10_e113005_d_n4, assign74200_body10_e113005_d_n5, assign74200_body10_e113005_d_n6, assign74200_body10_e113005_d_n7, assign74200_body10_e113005_d_n8, assign74200_body10_e113005_d_n9, assign74200_body10_e113005_d_n10, assign74200_body10_e113005_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1722 != 0.0)) && (locals.var_guard1727 != 0.0)) {
        let assign74200_body10_e113000: f64 = (1.0 + locals.var_t2);
        let assign74200_body10_e113001: f64 = (assign74200_body10_e113000).ln();
        let assign74200_body10_e113003: f64 = (assign74200_body10_e113001 / locals.var_c_sb);
        (assign74200_body10_e113003, ((((locals.var_t2_dn0 / assign74200_body10_e113000) * locals.var_c_sb) - (assign74200_body10_e113001 * locals.var_c_sb_dn0)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn2 / assign74200_body10_e113000) * locals.var_c_sb) - (assign74200_body10_e113001 * locals.var_c_sb_dn2)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn4 / assign74200_body10_e113000) * locals.var_c_sb) - (assign74200_body10_e113001 * locals.var_c_sb_dn4)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn5 / assign74200_body10_e113000) * locals.var_c_sb) - (assign74200_body10_e113001 * locals.var_c_sb_dn5)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn6 / assign74200_body10_e113000) * locals.var_c_sb) - (assign74200_body10_e113001 * locals.var_c_sb_dn6)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn7 / assign74200_body10_e113000) * locals.var_c_sb) - (assign74200_body10_e113001 * locals.var_c_sb_dn7)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn8 / assign74200_body10_e113000) * locals.var_c_sb) - (assign74200_body10_e113001 * locals.var_c_sb_dn8)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn9 / assign74200_body10_e113000) * locals.var_c_sb) - (assign74200_body10_e113001 * locals.var_c_sb_dn9)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn10 / assign74200_body10_e113000) * locals.var_c_sb) - (assign74200_body10_e113001 * locals.var_c_sb_dn10)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn13 / assign74200_body10_e113000) * locals.var_c_sb) - (assign74200_body10_e113001 * locals.var_c_sb_dn13)) / (locals.var_c_sb * locals.var_c_sb)),)
    } else {
        (locals.var_phi_b, locals.var_phi_b_dn0, locals.var_phi_b_dn2, locals.var_phi_b_dn4, locals.var_phi_b_dn5, locals.var_phi_b_dn6, locals.var_phi_b_dn7, locals.var_phi_b_dn8, locals.var_phi_b_dn9, locals.var_phi_b_dn10, locals.var_phi_b_dn13,)
    }
};
            locals.var_phi_b = assign74200_body10_e113005;
            locals.var_phi_b_dn0 = assign74200_body10_e113005_d_n0;
            locals.var_phi_b_dn2 = assign74200_body10_e113005_d_n2;
            locals.var_phi_b_dn4 = assign74200_body10_e113005_d_n4;
            locals.var_phi_b_dn5 = assign74200_body10_e113005_d_n5;
            locals.var_phi_b_dn6 = assign74200_body10_e113005_d_n6;
            locals.var_phi_b_dn7 = assign74200_body10_e113005_d_n7;
            locals.var_phi_b_dn8 = assign74200_body10_e113005_d_n8;
            locals.var_phi_b_dn9 = assign74200_body10_e113005_d_n9;
            locals.var_phi_b_dn10 = assign74200_body10_e113005_d_n10;
            locals.var_phi_b_dn13 = assign74200_body10_e113005_d_n13;
            locals.var_phi_b_rv = 0.0;
            let (assign74200_body11_e113017, assign74200_body11_e113017_d_n0, assign74200_body11_e113017_d_n2, assign74200_body11_e113017_d_n4, assign74200_body11_e113017_d_n5, assign74200_body11_e113017_d_n6, assign74200_body11_e113017_d_n7, assign74200_body11_e113017_d_n8, assign74200_body11_e113017_d_n9, assign74200_body11_e113017_d_n10, assign74200_body11_e113017_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1722 != 0.0)) && (locals.var_guard1727 != 0.0)) {
        let assign74200_body11_e113014: f64 = (1.0 + locals.var_t2);
        let assign74200_body11_e113015: f64 = (locals.var_t1 / assign74200_body11_e113014);
        (assign74200_body11_e113015, (((locals.var_t1_dn0 * assign74200_body11_e113014) - (locals.var_t1 * locals.var_t2_dn0)) / (assign74200_body11_e113014 * assign74200_body11_e113014)), (((locals.var_t1_dn2 * assign74200_body11_e113014) - (locals.var_t1 * locals.var_t2_dn2)) / (assign74200_body11_e113014 * assign74200_body11_e113014)), (((locals.var_t1_dn4 * assign74200_body11_e113014) - (locals.var_t1 * locals.var_t2_dn4)) / (assign74200_body11_e113014 * assign74200_body11_e113014)), (((locals.var_t1_dn5 * assign74200_body11_e113014) - (locals.var_t1 * locals.var_t2_dn5)) / (assign74200_body11_e113014 * assign74200_body11_e113014)), (((locals.var_t1_dn6 * assign74200_body11_e113014) - (locals.var_t1 * locals.var_t2_dn6)) / (assign74200_body11_e113014 * assign74200_body11_e113014)), (((locals.var_t1_dn7 * assign74200_body11_e113014) - (locals.var_t1 * locals.var_t2_dn7)) / (assign74200_body11_e113014 * assign74200_body11_e113014)), (((locals.var_t1_dn8 * assign74200_body11_e113014) - (locals.var_t1 * locals.var_t2_dn8)) / (assign74200_body11_e113014 * assign74200_body11_e113014)), (((locals.var_t1_dn9 * assign74200_body11_e113014) - (locals.var_t1 * locals.var_t2_dn9)) / (assign74200_body11_e113014 * assign74200_body11_e113014)), (((locals.var_t1_dn10 * assign74200_body11_e113014) - (locals.var_t1 * locals.var_t2_dn10)) / (assign74200_body11_e113014 * assign74200_body11_e113014)), (((locals.var_t1_dn13 * assign74200_body11_e113014) - (locals.var_t1 * locals.var_t2_dn13)) / (assign74200_body11_e113014 * assign74200_body11_e113014)),)
    } else {
        (locals.var_phi_b_dpss, locals.var_phi_b_dpss_dn0, locals.var_phi_b_dpss_dn2, locals.var_phi_b_dpss_dn4, locals.var_phi_b_dpss_dn5, locals.var_phi_b_dpss_dn6, locals.var_phi_b_dpss_dn7, locals.var_phi_b_dpss_dn8, locals.var_phi_b_dpss_dn9, locals.var_phi_b_dpss_dn10, locals.var_phi_b_dpss_dn13,)
    }
};
            locals.var_phi_b_dpss = assign74200_body11_e113017;
            locals.var_phi_b_dpss_dn0 = assign74200_body11_e113017_d_n0;
            locals.var_phi_b_dpss_dn2 = assign74200_body11_e113017_d_n2;
            locals.var_phi_b_dpss_dn4 = assign74200_body11_e113017_d_n4;
            locals.var_phi_b_dpss_dn5 = assign74200_body11_e113017_d_n5;
            locals.var_phi_b_dpss_dn6 = assign74200_body11_e113017_d_n6;
            locals.var_phi_b_dpss_dn7 = assign74200_body11_e113017_d_n7;
            locals.var_phi_b_dpss_dn8 = assign74200_body11_e113017_d_n8;
            locals.var_phi_b_dpss_dn9 = assign74200_body11_e113017_d_n9;
            locals.var_phi_b_dpss_dn10 = assign74200_body11_e113017_d_n10;
            locals.var_phi_b_dpss_dn13 = assign74200_body11_e113017_d_n13;
            locals.var_phi_b_dpss_rv = 0.0;
            let (assign74200_body12_e113028, assign74200_body12_e113028_d_n0, assign74200_body12_e113028_d_n2, assign74200_body12_e113028_d_n4, assign74200_body12_e113028_d_n5, assign74200_body12_e113028_d_n6, assign74200_body12_e113028_d_n7, assign74200_body12_e113028_d_n8, assign74200_body12_e113028_d_n9, assign74200_body12_e113028_d_n10, assign74200_body12_e113028_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1722 != 0.0)) && (locals.var_guard1727 == 0.0)) {
        let assign74200_body12_e113026: f64 = (locals.var_ps0ld_vxb - locals.var_dphi_sb);
        (assign74200_body12_e113026, (locals.var_ps0ld_vxb_dn0 - locals.var_dphi_sb_dn0), (locals.var_ps0ld_vxb_dn2 - locals.var_dphi_sb_dn2), (locals.var_ps0ld_vxb_dn4 - locals.var_dphi_sb_dn4), (locals.var_ps0ld_vxb_dn5 - locals.var_dphi_sb_dn5), (locals.var_ps0ld_vxb_dn6 - locals.var_dphi_sb_dn6), (locals.var_ps0ld_vxb_dn7 - locals.var_dphi_sb_dn7), (locals.var_ps0ld_vxb_dn8 - locals.var_dphi_sb_dn8), (locals.var_ps0ld_vxb_dn9 - locals.var_dphi_sb_dn9), (locals.var_ps0ld_vxb_dn10 - locals.var_dphi_sb_dn10), (locals.var_ps0ld_vxb_dn13 - locals.var_dphi_sb_dn13),)
    } else {
        (locals.var_phi_b, locals.var_phi_b_dn0, locals.var_phi_b_dn2, locals.var_phi_b_dn4, locals.var_phi_b_dn5, locals.var_phi_b_dn6, locals.var_phi_b_dn7, locals.var_phi_b_dn8, locals.var_phi_b_dn9, locals.var_phi_b_dn10, locals.var_phi_b_dn13,)
    }
};
            locals.var_phi_b = assign74200_body12_e113028;
            locals.var_phi_b_dn0 = assign74200_body12_e113028_d_n0;
            locals.var_phi_b_dn2 = assign74200_body12_e113028_d_n2;
            locals.var_phi_b_dn4 = assign74200_body12_e113028_d_n4;
            locals.var_phi_b_dn5 = assign74200_body12_e113028_d_n5;
            locals.var_phi_b_dn6 = assign74200_body12_e113028_d_n6;
            locals.var_phi_b_dn7 = assign74200_body12_e113028_d_n7;
            locals.var_phi_b_dn8 = assign74200_body12_e113028_d_n8;
            locals.var_phi_b_dn9 = assign74200_body12_e113028_d_n9;
            locals.var_phi_b_dn10 = assign74200_body12_e113028_d_n10;
            locals.var_phi_b_dn13 = assign74200_body12_e113028_d_n13;
            locals.var_phi_b_rv = 0.0;
            let (assign74200_body13_e113037, assign74200_body13_e113037_d_n0, assign74200_body13_e113037_d_n2, assign74200_body13_e113037_d_n4, assign74200_body13_e113037_d_n5, assign74200_body13_e113037_d_n6, assign74200_body13_e113037_d_n7, assign74200_body13_e113037_d_n8, assign74200_body13_e113037_d_n9, assign74200_body13_e113037_d_n10, assign74200_body13_e113037_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1722 != 0.0)) && (locals.var_guard1727 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_phi_b_dpss, locals.var_phi_b_dpss_dn0, locals.var_phi_b_dpss_dn2, locals.var_phi_b_dpss_dn4, locals.var_phi_b_dpss_dn5, locals.var_phi_b_dpss_dn6, locals.var_phi_b_dpss_dn7, locals.var_phi_b_dpss_dn8, locals.var_phi_b_dpss_dn9, locals.var_phi_b_dpss_dn10, locals.var_phi_b_dpss_dn13,)
    }
};
            locals.var_phi_b_dpss = assign74200_body13_e113037;
            locals.var_phi_b_dpss_dn0 = assign74200_body13_e113037_d_n0;
            locals.var_phi_b_dpss_dn2 = assign74200_body13_e113037_d_n2;
            locals.var_phi_b_dpss_dn4 = assign74200_body13_e113037_d_n4;
            locals.var_phi_b_dpss_dn5 = assign74200_body13_e113037_d_n5;
            locals.var_phi_b_dpss_dn6 = assign74200_body13_e113037_d_n6;
            locals.var_phi_b_dpss_dn7 = assign74200_body13_e113037_d_n7;
            locals.var_phi_b_dpss_dn8 = assign74200_body13_e113037_d_n8;
            locals.var_phi_b_dpss_dn9 = assign74200_body13_e113037_d_n9;
            locals.var_phi_b_dpss_dn10 = assign74200_body13_e113037_d_n10;
            locals.var_phi_b_dpss_dn13 = assign74200_body13_e113037_d_n13;
            locals.var_phi_b_dpss_rv = 0.0;
            let (assign74200_body14_e113045, assign74200_body14_e113045_d_n0, assign74200_body14_e113045_d_n2, assign74200_body14_e113045_d_n4, assign74200_body14_e113045_d_n5, assign74200_body14_e113045_d_n6, assign74200_body14_e113045_d_n7, assign74200_body14_e113045_d_n8, assign74200_body14_e113045_d_n9, assign74200_body14_e113045_d_n10, assign74200_body14_e113045_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1722 != 0.0)) {
        let assign74200_body14_e113043: f64 = (locals.var_beta * locals.var_phi_b);
        (assign74200_body14_e113043, ((locals.var_beta_dn0 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn0)), ((locals.var_beta_dn2 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn2)), ((locals.var_beta_dn4 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn4)), ((locals.var_beta_dn5 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn5)), ((locals.var_beta_dn6 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn6)), ((locals.var_beta_dn7 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn7)), ((locals.var_beta_dn8 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn8)), ((locals.var_beta_dn9 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn9)), ((locals.var_beta_dn10 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn10)), ((locals.var_beta_dn13 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn13)),)
    } else {
        (locals.var_chib, locals.var_chib_dn0, locals.var_chib_dn2, locals.var_chib_dn4, locals.var_chib_dn5, locals.var_chib_dn6, locals.var_chib_dn7, locals.var_chib_dn8, locals.var_chib_dn9, locals.var_chib_dn10, locals.var_chib_dn13,)
    }
};
            locals.var_chib = assign74200_body14_e113045;
            locals.var_chib_dn0 = assign74200_body14_e113045_d_n0;
            locals.var_chib_dn2 = assign74200_body14_e113045_d_n2;
            locals.var_chib_dn4 = assign74200_body14_e113045_d_n4;
            locals.var_chib_dn5 = assign74200_body14_e113045_d_n5;
            locals.var_chib_dn6 = assign74200_body14_e113045_d_n6;
            locals.var_chib_dn7 = assign74200_body14_e113045_d_n7;
            locals.var_chib_dn8 = assign74200_body14_e113045_d_n8;
            locals.var_chib_dn9 = assign74200_body14_e113045_d_n9;
            locals.var_chib_dn10 = assign74200_body14_e113045_d_n10;
            locals.var_chib_dn13 = assign74200_body14_e113045_d_n13;
            locals.var_chib_rv = 0.0;
            let assign74200_body15_e113047: f64 = (locals.var_chi).abs();
            let assign74200_body15_e113049: f64 = if assign74200_body15_e113047 < 1e-6 { 1.0 } else { 0.0 };
            locals.var_guard1728 = assign74200_body15_e113049;
            locals.var_guard1728_rv = 0.0;
            let (assign74200_body17_e113095, assign74200_body17_e113095_d_n0, assign74200_body17_e113095_d_n2, assign74200_body17_e113095_d_n4, assign74200_body17_e113095_d_n5, assign74200_body17_e113095_d_n6, assign74200_body17_e113095_d_n7, assign74200_body17_e113095_d_n8, assign74200_body17_e113095_d_n9, assign74200_body17_e113095_d_n10, assign74200_body17_e113095_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1722 != 0.0)) && (locals.var_guard1728 != 0.0)) {
        let assign74200_body17_e113073: f64 = (locals.var_chi * locals.var_chi);
        let assign74200_body17_e113075: f64 = (assign74200_body17_e113073 / 2.0);
        let assign74200_body17_e113079: f64 = (locals.var_chi / 3.0);
        let assign74200_body17_e113083: f64 = (locals.var_chi / 4.0);
        let assign74200_body17_e113087: f64 = (locals.var_chi / 5.0);
        let assign74200_body17_e113088: f64 = (1.0 - assign74200_body17_e113087);
        let assign74200_body17_e113089: f64 = (assign74200_body17_e113083 * assign74200_body17_e113088);
        let assign74200_body17_e113090: f64 = (1.0 - assign74200_body17_e113089);
        let assign74200_body17_e113091: f64 = (assign74200_body17_e113079 * assign74200_body17_e113090);
        let assign74200_body17_e113092: f64 = (1.0 - assign74200_body17_e113091);
        let assign74200_body17_e113093: f64 = (assign74200_body17_e113075 * assign74200_body17_e113092);
        (assign74200_body17_e113093, (((((locals.var_chi_dn0 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn0)) / 2.0) * assign74200_body17_e113092) + (assign74200_body17_e113075 * (-(((locals.var_chi_dn0 / 3.0) * assign74200_body17_e113090) + (assign74200_body17_e113079 * (-(((locals.var_chi_dn0 / 4.0) * assign74200_body17_e113088) + (assign74200_body17_e113083 * (-(locals.var_chi_dn0 / 5.0)))))))))), (((((locals.var_chi_dn2 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn2)) / 2.0) * assign74200_body17_e113092) + (assign74200_body17_e113075 * (-(((locals.var_chi_dn2 / 3.0) * assign74200_body17_e113090) + (assign74200_body17_e113079 * (-(((locals.var_chi_dn2 / 4.0) * assign74200_body17_e113088) + (assign74200_body17_e113083 * (-(locals.var_chi_dn2 / 5.0)))))))))), (((((locals.var_chi_dn4 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn4)) / 2.0) * assign74200_body17_e113092) + (assign74200_body17_e113075 * (-(((locals.var_chi_dn4 / 3.0) * assign74200_body17_e113090) + (assign74200_body17_e113079 * (-(((locals.var_chi_dn4 / 4.0) * assign74200_body17_e113088) + (assign74200_body17_e113083 * (-(locals.var_chi_dn4 / 5.0)))))))))), (((((locals.var_chi_dn5 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn5)) / 2.0) * assign74200_body17_e113092) + (assign74200_body17_e113075 * (-(((locals.var_chi_dn5 / 3.0) * assign74200_body17_e113090) + (assign74200_body17_e113079 * (-(((locals.var_chi_dn5 / 4.0) * assign74200_body17_e113088) + (assign74200_body17_e113083 * (-(locals.var_chi_dn5 / 5.0)))))))))), (((((locals.var_chi_dn6 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn6)) / 2.0) * assign74200_body17_e113092) + (assign74200_body17_e113075 * (-(((locals.var_chi_dn6 / 3.0) * assign74200_body17_e113090) + (assign74200_body17_e113079 * (-(((locals.var_chi_dn6 / 4.0) * assign74200_body17_e113088) + (assign74200_body17_e113083 * (-(locals.var_chi_dn6 / 5.0)))))))))), (((((locals.var_chi_dn7 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn7)) / 2.0) * assign74200_body17_e113092) + (assign74200_body17_e113075 * (-(((locals.var_chi_dn7 / 3.0) * assign74200_body17_e113090) + (assign74200_body17_e113079 * (-(((locals.var_chi_dn7 / 4.0) * assign74200_body17_e113088) + (assign74200_body17_e113083 * (-(locals.var_chi_dn7 / 5.0)))))))))), (((((locals.var_chi_dn8 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn8)) / 2.0) * assign74200_body17_e113092) + (assign74200_body17_e113075 * (-(((locals.var_chi_dn8 / 3.0) * assign74200_body17_e113090) + (assign74200_body17_e113079 * (-(((locals.var_chi_dn8 / 4.0) * assign74200_body17_e113088) + (assign74200_body17_e113083 * (-(locals.var_chi_dn8 / 5.0)))))))))), (((((locals.var_chi_dn9 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn9)) / 2.0) * assign74200_body17_e113092) + (assign74200_body17_e113075 * (-(((locals.var_chi_dn9 / 3.0) * assign74200_body17_e113090) + (assign74200_body17_e113079 * (-(((locals.var_chi_dn9 / 4.0) * assign74200_body17_e113088) + (assign74200_body17_e113083 * (-(locals.var_chi_dn9 / 5.0)))))))))), (((((locals.var_chi_dn10 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn10)) / 2.0) * assign74200_body17_e113092) + (assign74200_body17_e113075 * (-(((locals.var_chi_dn10 / 3.0) * assign74200_body17_e113090) + (assign74200_body17_e113079 * (-(((locals.var_chi_dn10 / 4.0) * assign74200_body17_e113088) + (assign74200_body17_e113083 * (-(locals.var_chi_dn10 / 5.0)))))))))), (((((locals.var_chi_dn13 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn13)) / 2.0) * assign74200_body17_e113092) + (assign74200_body17_e113075 * (-(((locals.var_chi_dn13 / 3.0) * assign74200_body17_e113090) + (assign74200_body17_e113079 * (-(((locals.var_chi_dn13 / 4.0) * assign74200_body17_e113088) + (assign74200_body17_e113083 * (-(locals.var_chi_dn13 / 5.0)))))))))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
            locals.var_t0 = assign74200_body17_e113095;
            locals.var_t0_dn0 = assign74200_body17_e113095_d_n0;
            locals.var_t0_dn2 = assign74200_body17_e113095_d_n2;
            locals.var_t0_dn4 = assign74200_body17_e113095_d_n4;
            locals.var_t0_dn5 = assign74200_body17_e113095_d_n5;
            locals.var_t0_dn6 = assign74200_body17_e113095_d_n6;
            locals.var_t0_dn7 = assign74200_body17_e113095_d_n7;
            locals.var_t0_dn8 = assign74200_body17_e113095_d_n8;
            locals.var_t0_dn9 = assign74200_body17_e113095_d_n9;
            locals.var_t0_dn10 = assign74200_body17_e113095_d_n10;
            locals.var_t0_dn13 = assign74200_body17_e113095_d_n13;
            locals.var_t0_rv = 0.0;
            let (assign74200_body18_e113121, assign74200_body18_e113121_d_n0, assign74200_body18_e113121_d_n2, assign74200_body18_e113121_d_n4, assign74200_body18_e113121_d_n5, assign74200_body18_e113121_d_n6, assign74200_body18_e113121_d_n7, assign74200_body18_e113121_d_n8, assign74200_body18_e113121_d_n9, assign74200_body18_e113121_d_n10, assign74200_body18_e113121_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1722 != 0.0)) && (locals.var_guard1728 != 0.0)) {
        let assign74200_body18_e113105: f64 = (locals.var_chi / 2.0);
        let assign74200_body18_e113109: f64 = (locals.var_chi / 3.0);
        let assign74200_body18_e113113: f64 = (locals.var_chi / 4.0);
        let assign74200_body18_e113114: f64 = (1.0 - assign74200_body18_e113113);
        let assign74200_body18_e113115: f64 = (assign74200_body18_e113109 * assign74200_body18_e113114);
        let assign74200_body18_e113116: f64 = (1.0 - assign74200_body18_e113115);
        let assign74200_body18_e113117: f64 = (assign74200_body18_e113105 * assign74200_body18_e113116);
        let assign74200_body18_e113118: f64 = (1.0 - assign74200_body18_e113117);
        let assign74200_body18_e113119: f64 = (locals.var_chi * assign74200_body18_e113118);
        (assign74200_body18_e113119, ((locals.var_chi_dn0 * assign74200_body18_e113118) + (locals.var_chi * (-(((locals.var_chi_dn0 / 2.0) * assign74200_body18_e113116) + (assign74200_body18_e113105 * (-(((locals.var_chi_dn0 / 3.0) * assign74200_body18_e113114) + (assign74200_body18_e113109 * (-(locals.var_chi_dn0 / 4.0)))))))))), ((locals.var_chi_dn2 * assign74200_body18_e113118) + (locals.var_chi * (-(((locals.var_chi_dn2 / 2.0) * assign74200_body18_e113116) + (assign74200_body18_e113105 * (-(((locals.var_chi_dn2 / 3.0) * assign74200_body18_e113114) + (assign74200_body18_e113109 * (-(locals.var_chi_dn2 / 4.0)))))))))), ((locals.var_chi_dn4 * assign74200_body18_e113118) + (locals.var_chi * (-(((locals.var_chi_dn4 / 2.0) * assign74200_body18_e113116) + (assign74200_body18_e113105 * (-(((locals.var_chi_dn4 / 3.0) * assign74200_body18_e113114) + (assign74200_body18_e113109 * (-(locals.var_chi_dn4 / 4.0)))))))))), ((locals.var_chi_dn5 * assign74200_body18_e113118) + (locals.var_chi * (-(((locals.var_chi_dn5 / 2.0) * assign74200_body18_e113116) + (assign74200_body18_e113105 * (-(((locals.var_chi_dn5 / 3.0) * assign74200_body18_e113114) + (assign74200_body18_e113109 * (-(locals.var_chi_dn5 / 4.0)))))))))), ((locals.var_chi_dn6 * assign74200_body18_e113118) + (locals.var_chi * (-(((locals.var_chi_dn6 / 2.0) * assign74200_body18_e113116) + (assign74200_body18_e113105 * (-(((locals.var_chi_dn6 / 3.0) * assign74200_body18_e113114) + (assign74200_body18_e113109 * (-(locals.var_chi_dn6 / 4.0)))))))))), ((locals.var_chi_dn7 * assign74200_body18_e113118) + (locals.var_chi * (-(((locals.var_chi_dn7 / 2.0) * assign74200_body18_e113116) + (assign74200_body18_e113105 * (-(((locals.var_chi_dn7 / 3.0) * assign74200_body18_e113114) + (assign74200_body18_e113109 * (-(locals.var_chi_dn7 / 4.0)))))))))), ((locals.var_chi_dn8 * assign74200_body18_e113118) + (locals.var_chi * (-(((locals.var_chi_dn8 / 2.0) * assign74200_body18_e113116) + (assign74200_body18_e113105 * (-(((locals.var_chi_dn8 / 3.0) * assign74200_body18_e113114) + (assign74200_body18_e113109 * (-(locals.var_chi_dn8 / 4.0)))))))))), ((locals.var_chi_dn9 * assign74200_body18_e113118) + (locals.var_chi * (-(((locals.var_chi_dn9 / 2.0) * assign74200_body18_e113116) + (assign74200_body18_e113105 * (-(((locals.var_chi_dn9 / 3.0) * assign74200_body18_e113114) + (assign74200_body18_e113109 * (-(locals.var_chi_dn9 / 4.0)))))))))), ((locals.var_chi_dn10 * assign74200_body18_e113118) + (locals.var_chi * (-(((locals.var_chi_dn10 / 2.0) * assign74200_body18_e113116) + (assign74200_body18_e113105 * (-(((locals.var_chi_dn10 / 3.0) * assign74200_body18_e113114) + (assign74200_body18_e113109 * (-(locals.var_chi_dn10 / 4.0)))))))))), ((locals.var_chi_dn13 * assign74200_body18_e113118) + (locals.var_chi * (-(((locals.var_chi_dn13 / 2.0) * assign74200_body18_e113116) + (assign74200_body18_e113105 * (-(((locals.var_chi_dn13 / 3.0) * assign74200_body18_e113114) + (assign74200_body18_e113109 * (-(locals.var_chi_dn13 / 4.0)))))))))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
            locals.var_t1 = assign74200_body18_e113121;
            locals.var_t1_dn0 = assign74200_body18_e113121_d_n0;
            locals.var_t1_dn2 = assign74200_body18_e113121_d_n2;
            locals.var_t1_dn4 = assign74200_body18_e113121_d_n4;
            locals.var_t1_dn5 = assign74200_body18_e113121_d_n5;
            locals.var_t1_dn6 = assign74200_body18_e113121_d_n6;
            locals.var_t1_dn7 = assign74200_body18_e113121_d_n7;
            locals.var_t1_dn8 = assign74200_body18_e113121_d_n8;
            locals.var_t1_dn9 = assign74200_body18_e113121_d_n9;
            locals.var_t1_dn10 = assign74200_body18_e113121_d_n10;
            locals.var_t1_dn13 = assign74200_body18_e113121_d_n13;
            locals.var_t1_rv = 0.0;
            let (assign74200_body19_e113151, assign74200_body19_e113151_d_n0, assign74200_body19_e113151_d_n2, assign74200_body19_e113151_d_n4, assign74200_body19_e113151_d_n5, assign74200_body19_e113151_d_n6, assign74200_body19_e113151_d_n7, assign74200_body19_e113151_d_n8, assign74200_body19_e113151_d_n9, assign74200_body19_e113151_d_n10, assign74200_body19_e113151_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1722 != 0.0)) && (locals.var_guard1728 != 0.0)) {
        let assign74200_body19_e113129: f64 = (locals.var_chib * locals.var_chib);
        let assign74200_body19_e113131: f64 = (assign74200_body19_e113129 / 2.0);
        let assign74200_body19_e113135: f64 = (locals.var_chib / 3.0);
        let assign74200_body19_e113139: f64 = (locals.var_chib / 4.0);
        let assign74200_body19_e113143: f64 = (locals.var_chib / 5.0);
        let assign74200_body19_e113144: f64 = (1.0 - assign74200_body19_e113143);
        let assign74200_body19_e113145: f64 = (assign74200_body19_e113139 * assign74200_body19_e113144);
        let assign74200_body19_e113146: f64 = (1.0 - assign74200_body19_e113145);
        let assign74200_body19_e113147: f64 = (assign74200_body19_e113135 * assign74200_body19_e113146);
        let assign74200_body19_e113148: f64 = (1.0 - assign74200_body19_e113147);
        let assign74200_body19_e113149: f64 = (assign74200_body19_e113131 * assign74200_body19_e113148);
        (assign74200_body19_e113149, (((((locals.var_chib_dn0 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn0)) / 2.0) * assign74200_body19_e113148) + (assign74200_body19_e113131 * (-(((locals.var_chib_dn0 / 3.0) * assign74200_body19_e113146) + (assign74200_body19_e113135 * (-(((locals.var_chib_dn0 / 4.0) * assign74200_body19_e113144) + (assign74200_body19_e113139 * (-(locals.var_chib_dn0 / 5.0)))))))))), (((((locals.var_chib_dn2 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn2)) / 2.0) * assign74200_body19_e113148) + (assign74200_body19_e113131 * (-(((locals.var_chib_dn2 / 3.0) * assign74200_body19_e113146) + (assign74200_body19_e113135 * (-(((locals.var_chib_dn2 / 4.0) * assign74200_body19_e113144) + (assign74200_body19_e113139 * (-(locals.var_chib_dn2 / 5.0)))))))))), (((((locals.var_chib_dn4 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn4)) / 2.0) * assign74200_body19_e113148) + (assign74200_body19_e113131 * (-(((locals.var_chib_dn4 / 3.0) * assign74200_body19_e113146) + (assign74200_body19_e113135 * (-(((locals.var_chib_dn4 / 4.0) * assign74200_body19_e113144) + (assign74200_body19_e113139 * (-(locals.var_chib_dn4 / 5.0)))))))))), (((((locals.var_chib_dn5 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn5)) / 2.0) * assign74200_body19_e113148) + (assign74200_body19_e113131 * (-(((locals.var_chib_dn5 / 3.0) * assign74200_body19_e113146) + (assign74200_body19_e113135 * (-(((locals.var_chib_dn5 / 4.0) * assign74200_body19_e113144) + (assign74200_body19_e113139 * (-(locals.var_chib_dn5 / 5.0)))))))))), (((((locals.var_chib_dn6 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn6)) / 2.0) * assign74200_body19_e113148) + (assign74200_body19_e113131 * (-(((locals.var_chib_dn6 / 3.0) * assign74200_body19_e113146) + (assign74200_body19_e113135 * (-(((locals.var_chib_dn6 / 4.0) * assign74200_body19_e113144) + (assign74200_body19_e113139 * (-(locals.var_chib_dn6 / 5.0)))))))))), (((((locals.var_chib_dn7 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn7)) / 2.0) * assign74200_body19_e113148) + (assign74200_body19_e113131 * (-(((locals.var_chib_dn7 / 3.0) * assign74200_body19_e113146) + (assign74200_body19_e113135 * (-(((locals.var_chib_dn7 / 4.0) * assign74200_body19_e113144) + (assign74200_body19_e113139 * (-(locals.var_chib_dn7 / 5.0)))))))))), (((((locals.var_chib_dn8 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn8)) / 2.0) * assign74200_body19_e113148) + (assign74200_body19_e113131 * (-(((locals.var_chib_dn8 / 3.0) * assign74200_body19_e113146) + (assign74200_body19_e113135 * (-(((locals.var_chib_dn8 / 4.0) * assign74200_body19_e113144) + (assign74200_body19_e113139 * (-(locals.var_chib_dn8 / 5.0)))))))))), (((((locals.var_chib_dn9 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn9)) / 2.0) * assign74200_body19_e113148) + (assign74200_body19_e113131 * (-(((locals.var_chib_dn9 / 3.0) * assign74200_body19_e113146) + (assign74200_body19_e113135 * (-(((locals.var_chib_dn9 / 4.0) * assign74200_body19_e113144) + (assign74200_body19_e113139 * (-(locals.var_chib_dn9 / 5.0)))))))))), (((((locals.var_chib_dn10 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn10)) / 2.0) * assign74200_body19_e113148) + (assign74200_body19_e113131 * (-(((locals.var_chib_dn10 / 3.0) * assign74200_body19_e113146) + (assign74200_body19_e113135 * (-(((locals.var_chib_dn10 / 4.0) * assign74200_body19_e113144) + (assign74200_body19_e113139 * (-(locals.var_chib_dn10 / 5.0)))))))))), (((((locals.var_chib_dn13 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn13)) / 2.0) * assign74200_body19_e113148) + (assign74200_body19_e113131 * (-(((locals.var_chib_dn13 / 3.0) * assign74200_body19_e113146) + (assign74200_body19_e113135 * (-(((locals.var_chib_dn13 / 4.0) * assign74200_body19_e113144) + (assign74200_body19_e113139 * (-(locals.var_chib_dn13 / 5.0)))))))))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
            locals.var_t2 = assign74200_body19_e113151;
            locals.var_t2_dn0 = assign74200_body19_e113151_d_n0;
            locals.var_t2_dn2 = assign74200_body19_e113151_d_n2;
            locals.var_t2_dn4 = assign74200_body19_e113151_d_n4;
            locals.var_t2_dn5 = assign74200_body19_e113151_d_n5;
            locals.var_t2_dn6 = assign74200_body19_e113151_d_n6;
            locals.var_t2_dn7 = assign74200_body19_e113151_d_n7;
            locals.var_t2_dn8 = assign74200_body19_e113151_d_n8;
            locals.var_t2_dn9 = assign74200_body19_e113151_d_n9;
            locals.var_t2_dn10 = assign74200_body19_e113151_d_n10;
            locals.var_t2_dn13 = assign74200_body19_e113151_d_n13;
            locals.var_t2_rv = 0.0;
            let (assign74200_body20_e113177, assign74200_body20_e113177_d_n0, assign74200_body20_e113177_d_n2, assign74200_body20_e113177_d_n4, assign74200_body20_e113177_d_n5, assign74200_body20_e113177_d_n6, assign74200_body20_e113177_d_n7, assign74200_body20_e113177_d_n8, assign74200_body20_e113177_d_n9, assign74200_body20_e113177_d_n10, assign74200_body20_e113177_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1722 != 0.0)) && (locals.var_guard1728 != 0.0)) {
        let assign74200_body20_e113161: f64 = (locals.var_chib / 2.0);
        let assign74200_body20_e113165: f64 = (locals.var_chib / 3.0);
        let assign74200_body20_e113169: f64 = (locals.var_chib / 4.0);
        let assign74200_body20_e113170: f64 = (1.0 - assign74200_body20_e113169);
        let assign74200_body20_e113171: f64 = (assign74200_body20_e113165 * assign74200_body20_e113170);
        let assign74200_body20_e113172: f64 = (1.0 - assign74200_body20_e113171);
        let assign74200_body20_e113173: f64 = (assign74200_body20_e113161 * assign74200_body20_e113172);
        let assign74200_body20_e113174: f64 = (1.0 - assign74200_body20_e113173);
        let assign74200_body20_e113175: f64 = (locals.var_chib * assign74200_body20_e113174);
        (assign74200_body20_e113175, ((locals.var_chib_dn0 * assign74200_body20_e113174) + (locals.var_chib * (-(((locals.var_chib_dn0 / 2.0) * assign74200_body20_e113172) + (assign74200_body20_e113161 * (-(((locals.var_chib_dn0 / 3.0) * assign74200_body20_e113170) + (assign74200_body20_e113165 * (-(locals.var_chib_dn0 / 4.0)))))))))), ((locals.var_chib_dn2 * assign74200_body20_e113174) + (locals.var_chib * (-(((locals.var_chib_dn2 / 2.0) * assign74200_body20_e113172) + (assign74200_body20_e113161 * (-(((locals.var_chib_dn2 / 3.0) * assign74200_body20_e113170) + (assign74200_body20_e113165 * (-(locals.var_chib_dn2 / 4.0)))))))))), ((locals.var_chib_dn4 * assign74200_body20_e113174) + (locals.var_chib * (-(((locals.var_chib_dn4 / 2.0) * assign74200_body20_e113172) + (assign74200_body20_e113161 * (-(((locals.var_chib_dn4 / 3.0) * assign74200_body20_e113170) + (assign74200_body20_e113165 * (-(locals.var_chib_dn4 / 4.0)))))))))), ((locals.var_chib_dn5 * assign74200_body20_e113174) + (locals.var_chib * (-(((locals.var_chib_dn5 / 2.0) * assign74200_body20_e113172) + (assign74200_body20_e113161 * (-(((locals.var_chib_dn5 / 3.0) * assign74200_body20_e113170) + (assign74200_body20_e113165 * (-(locals.var_chib_dn5 / 4.0)))))))))), ((locals.var_chib_dn6 * assign74200_body20_e113174) + (locals.var_chib * (-(((locals.var_chib_dn6 / 2.0) * assign74200_body20_e113172) + (assign74200_body20_e113161 * (-(((locals.var_chib_dn6 / 3.0) * assign74200_body20_e113170) + (assign74200_body20_e113165 * (-(locals.var_chib_dn6 / 4.0)))))))))), ((locals.var_chib_dn7 * assign74200_body20_e113174) + (locals.var_chib * (-(((locals.var_chib_dn7 / 2.0) * assign74200_body20_e113172) + (assign74200_body20_e113161 * (-(((locals.var_chib_dn7 / 3.0) * assign74200_body20_e113170) + (assign74200_body20_e113165 * (-(locals.var_chib_dn7 / 4.0)))))))))), ((locals.var_chib_dn8 * assign74200_body20_e113174) + (locals.var_chib * (-(((locals.var_chib_dn8 / 2.0) * assign74200_body20_e113172) + (assign74200_body20_e113161 * (-(((locals.var_chib_dn8 / 3.0) * assign74200_body20_e113170) + (assign74200_body20_e113165 * (-(locals.var_chib_dn8 / 4.0)))))))))), ((locals.var_chib_dn9 * assign74200_body20_e113174) + (locals.var_chib * (-(((locals.var_chib_dn9 / 2.0) * assign74200_body20_e113172) + (assign74200_body20_e113161 * (-(((locals.var_chib_dn9 / 3.0) * assign74200_body20_e113170) + (assign74200_body20_e113165 * (-(locals.var_chib_dn9 / 4.0)))))))))), ((locals.var_chib_dn10 * assign74200_body20_e113174) + (locals.var_chib * (-(((locals.var_chib_dn10 / 2.0) * assign74200_body20_e113172) + (assign74200_body20_e113161 * (-(((locals.var_chib_dn10 / 3.0) * assign74200_body20_e113170) + (assign74200_body20_e113165 * (-(locals.var_chib_dn10 / 4.0)))))))))), ((locals.var_chib_dn13 * assign74200_body20_e113174) + (locals.var_chib * (-(((locals.var_chib_dn13 / 2.0) * assign74200_body20_e113172) + (assign74200_body20_e113161 * (-(((locals.var_chib_dn13 / 3.0) * assign74200_body20_e113170) + (assign74200_body20_e113165 * (-(locals.var_chib_dn13 / 4.0)))))))))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
            locals.var_t3 = assign74200_body20_e113177;
            locals.var_t3_dn0 = assign74200_body20_e113177_d_n0;
            locals.var_t3_dn2 = assign74200_body20_e113177_d_n2;
            locals.var_t3_dn4 = assign74200_body20_e113177_d_n4;
            locals.var_t3_dn5 = assign74200_body20_e113177_d_n5;
            locals.var_t3_dn6 = assign74200_body20_e113177_d_n6;
            locals.var_t3_dn7 = assign74200_body20_e113177_d_n7;
            locals.var_t3_dn8 = assign74200_body20_e113177_d_n8;
            locals.var_t3_dn9 = assign74200_body20_e113177_d_n9;
            locals.var_t3_dn10 = assign74200_body20_e113177_d_n10;
            locals.var_t3_dn13 = assign74200_body20_e113177_d_n13;
            locals.var_t3_rv = 0.0;
            let (assign74200_body21_e113187, assign74200_body21_e113187_d_n0, assign74200_body21_e113187_d_n2, assign74200_body21_e113187_d_n4, assign74200_body21_e113187_d_n5, assign74200_body21_e113187_d_n6, assign74200_body21_e113187_d_n7, assign74200_body21_e113187_d_n8, assign74200_body21_e113187_d_n9, assign74200_body21_e113187_d_n10, assign74200_body21_e113187_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1722 != 0.0)) && (locals.var_guard1728 != 0.0)) {
        let assign74200_body21_e113185: f64 = (locals.var_t0 - locals.var_t2);
        (assign74200_body21_e113185, (locals.var_t0_dn0 - locals.var_t2_dn0), (locals.var_t0_dn2 - locals.var_t2_dn2), (locals.var_t0_dn4 - locals.var_t2_dn4), (locals.var_t0_dn5 - locals.var_t2_dn5), (locals.var_t0_dn6 - locals.var_t2_dn6), (locals.var_t0_dn7 - locals.var_t2_dn7), (locals.var_t0_dn8 - locals.var_t2_dn8), (locals.var_t0_dn9 - locals.var_t2_dn9), (locals.var_t0_dn10 - locals.var_t2_dn10), (locals.var_t0_dn13 - locals.var_t2_dn13),)
    } else {
        (locals.var_fbsq, locals.var_fbsq_dn0, locals.var_fbsq_dn2, locals.var_fbsq_dn4, locals.var_fbsq_dn5, locals.var_fbsq_dn6, locals.var_fbsq_dn7, locals.var_fbsq_dn8, locals.var_fbsq_dn9, locals.var_fbsq_dn10, locals.var_fbsq_dn13,)
    }
};
            locals.var_fbsq = assign74200_body21_e113187;
            locals.var_fbsq_dn0 = assign74200_body21_e113187_d_n0;
            locals.var_fbsq_dn2 = assign74200_body21_e113187_d_n2;
            locals.var_fbsq_dn4 = assign74200_body21_e113187_d_n4;
            locals.var_fbsq_dn5 = assign74200_body21_e113187_d_n5;
            locals.var_fbsq_dn6 = assign74200_body21_e113187_d_n6;
            locals.var_fbsq_dn7 = assign74200_body21_e113187_d_n7;
            locals.var_fbsq_dn8 = assign74200_body21_e113187_d_n8;
            locals.var_fbsq_dn9 = assign74200_body21_e113187_d_n9;
            locals.var_fbsq_dn10 = assign74200_body21_e113187_d_n10;
            locals.var_fbsq_dn13 = assign74200_body21_e113187_d_n13;
            locals.var_fbsq_rv = 0.0;
            let (assign74200_body22_e113201, assign74200_body22_e113201_d_n0, assign74200_body22_e113201_d_n2, assign74200_body22_e113201_d_n4, assign74200_body22_e113201_d_n5, assign74200_body22_e113201_d_n6, assign74200_body22_e113201_d_n7, assign74200_body22_e113201_d_n8, assign74200_body22_e113201_d_n9, assign74200_body22_e113201_d_n10, assign74200_body22_e113201_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1722 != 0.0)) && (locals.var_guard1728 != 0.0)) {
        let assign74200_body22_e113197: f64 = (locals.var_phi_b_dpss * locals.var_t3);
        let assign74200_body22_e113198: f64 = (locals.var_t1 - assign74200_body22_e113197);
        let assign74200_body22_e113199: f64 = (locals.var_beta * assign74200_body22_e113198);
        (assign74200_body22_e113199, ((locals.var_beta_dn0 * assign74200_body22_e113198) + (locals.var_beta * (locals.var_t1_dn0 - ((locals.var_phi_b_dpss_dn0 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn0))))), ((locals.var_beta_dn2 * assign74200_body22_e113198) + (locals.var_beta * (locals.var_t1_dn2 - ((locals.var_phi_b_dpss_dn2 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn2))))), ((locals.var_beta_dn4 * assign74200_body22_e113198) + (locals.var_beta * (locals.var_t1_dn4 - ((locals.var_phi_b_dpss_dn4 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn4))))), ((locals.var_beta_dn5 * assign74200_body22_e113198) + (locals.var_beta * (locals.var_t1_dn5 - ((locals.var_phi_b_dpss_dn5 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn5))))), ((locals.var_beta_dn6 * assign74200_body22_e113198) + (locals.var_beta * (locals.var_t1_dn6 - ((locals.var_phi_b_dpss_dn6 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn6))))), ((locals.var_beta_dn7 * assign74200_body22_e113198) + (locals.var_beta * (locals.var_t1_dn7 - ((locals.var_phi_b_dpss_dn7 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn7))))), ((locals.var_beta_dn8 * assign74200_body22_e113198) + (locals.var_beta * (locals.var_t1_dn8 - ((locals.var_phi_b_dpss_dn8 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn8))))), ((locals.var_beta_dn9 * assign74200_body22_e113198) + (locals.var_beta * (locals.var_t1_dn9 - ((locals.var_phi_b_dpss_dn9 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn9))))), ((locals.var_beta_dn10 * assign74200_body22_e113198) + (locals.var_beta * (locals.var_t1_dn10 - ((locals.var_phi_b_dpss_dn10 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn10))))), ((locals.var_beta_dn13 * assign74200_body22_e113198) + (locals.var_beta * (locals.var_t1_dn13 - ((locals.var_phi_b_dpss_dn13 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn13))))),)
    } else {
        (locals.var_fbsq_dpss, locals.var_fbsq_dpss_dn0, locals.var_fbsq_dpss_dn2, locals.var_fbsq_dpss_dn4, locals.var_fbsq_dpss_dn5, locals.var_fbsq_dpss_dn6, locals.var_fbsq_dpss_dn7, locals.var_fbsq_dpss_dn8, locals.var_fbsq_dpss_dn9, locals.var_fbsq_dpss_dn10, locals.var_fbsq_dpss_dn13,)
    }
};
            locals.var_fbsq_dpss = assign74200_body22_e113201;
            locals.var_fbsq_dpss_dn0 = assign74200_body22_e113201_d_n0;
            locals.var_fbsq_dpss_dn2 = assign74200_body22_e113201_d_n2;
            locals.var_fbsq_dpss_dn4 = assign74200_body22_e113201_d_n4;
            locals.var_fbsq_dpss_dn5 = assign74200_body22_e113201_d_n5;
            locals.var_fbsq_dpss_dn6 = assign74200_body22_e113201_d_n6;
            locals.var_fbsq_dpss_dn7 = assign74200_body22_e113201_d_n7;
            locals.var_fbsq_dpss_dn8 = assign74200_body22_e113201_d_n8;
            locals.var_fbsq_dpss_dn9 = assign74200_body22_e113201_d_n9;
            locals.var_fbsq_dpss_dn10 = assign74200_body22_e113201_d_n10;
            locals.var_fbsq_dpss_dn13 = assign74200_body22_e113201_d_n13;
            locals.var_fbsq_dpss_rv = 0.0;
            let (assign74200_body24_e113229, assign74200_body24_e113229_d_n0, assign74200_body24_e113229_d_n2, assign74200_body24_e113229_d_n4, assign74200_body24_e113229_d_n5, assign74200_body24_e113229_d_n6, assign74200_body24_e113229_d_n7, assign74200_body24_e113229_d_n8, assign74200_body24_e113229_d_n9, assign74200_body24_e113229_d_n10, assign74200_body24_e113229_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1722 != 0.0)) && (locals.var_guard1728 == 0.0)) {
        let assign74200_body24_e113226: f64 = (-locals.var_chi);
        let assign74200_body24_e113227: f64 = (assign74200_body24_e113226).exp();
        (assign74200_body24_e113227, (assign74200_body24_e113227 * (-locals.var_chi_dn0)), (assign74200_body24_e113227 * (-locals.var_chi_dn2)), (assign74200_body24_e113227 * (-locals.var_chi_dn4)), (assign74200_body24_e113227 * (-locals.var_chi_dn5)), (assign74200_body24_e113227 * (-locals.var_chi_dn6)), (assign74200_body24_e113227 * (-locals.var_chi_dn7)), (assign74200_body24_e113227 * (-locals.var_chi_dn8)), (assign74200_body24_e113227 * (-locals.var_chi_dn9)), (assign74200_body24_e113227 * (-locals.var_chi_dn10)), (assign74200_body24_e113227 * (-locals.var_chi_dn13)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
            locals.var_t0 = assign74200_body24_e113229;
            locals.var_t0_dn0 = assign74200_body24_e113229_d_n0;
            locals.var_t0_dn2 = assign74200_body24_e113229_d_n2;
            locals.var_t0_dn4 = assign74200_body24_e113229_d_n4;
            locals.var_t0_dn5 = assign74200_body24_e113229_d_n5;
            locals.var_t0_dn6 = assign74200_body24_e113229_d_n6;
            locals.var_t0_dn7 = assign74200_body24_e113229_d_n7;
            locals.var_t0_dn8 = assign74200_body24_e113229_d_n8;
            locals.var_t0_dn9 = assign74200_body24_e113229_d_n9;
            locals.var_t0_dn10 = assign74200_body24_e113229_d_n10;
            locals.var_t0_dn13 = assign74200_body24_e113229_d_n13;
            locals.var_t0_rv = 0.0;
            let (assign74200_body25_e113240, assign74200_body25_e113240_d_n0, assign74200_body25_e113240_d_n2, assign74200_body25_e113240_d_n4, assign74200_body25_e113240_d_n5, assign74200_body25_e113240_d_n6, assign74200_body25_e113240_d_n7, assign74200_body25_e113240_d_n8, assign74200_body25_e113240_d_n9, assign74200_body25_e113240_d_n10, assign74200_body25_e113240_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1722 != 0.0)) && (locals.var_guard1728 == 0.0)) {
        let assign74200_body25_e113237: f64 = (-locals.var_chib);
        let assign74200_body25_e113238: f64 = (assign74200_body25_e113237).exp();
        (assign74200_body25_e113238, (assign74200_body25_e113238 * (-locals.var_chib_dn0)), (assign74200_body25_e113238 * (-locals.var_chib_dn2)), (assign74200_body25_e113238 * (-locals.var_chib_dn4)), (assign74200_body25_e113238 * (-locals.var_chib_dn5)), (assign74200_body25_e113238 * (-locals.var_chib_dn6)), (assign74200_body25_e113238 * (-locals.var_chib_dn7)), (assign74200_body25_e113238 * (-locals.var_chib_dn8)), (assign74200_body25_e113238 * (-locals.var_chib_dn9)), (assign74200_body25_e113238 * (-locals.var_chib_dn10)), (assign74200_body25_e113238 * (-locals.var_chib_dn13)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
            locals.var_t1 = assign74200_body25_e113240;
            locals.var_t1_dn0 = assign74200_body25_e113240_d_n0;
            locals.var_t1_dn2 = assign74200_body25_e113240_d_n2;
            locals.var_t1_dn4 = assign74200_body25_e113240_d_n4;
            locals.var_t1_dn5 = assign74200_body25_e113240_d_n5;
            locals.var_t1_dn6 = assign74200_body25_e113240_d_n6;
            locals.var_t1_dn7 = assign74200_body25_e113240_d_n7;
            locals.var_t1_dn8 = assign74200_body25_e113240_d_n8;
            locals.var_t1_dn9 = assign74200_body25_e113240_d_n9;
            locals.var_t1_dn10 = assign74200_body25_e113240_d_n10;
            locals.var_t1_dn13 = assign74200_body25_e113240_d_n13;
            locals.var_t1_rv = 0.0;
            let (assign74200_body26_e113255, assign74200_body26_e113255_d_n0, assign74200_body26_e113255_d_n2, assign74200_body26_e113255_d_n4, assign74200_body26_e113255_d_n5, assign74200_body26_e113255_d_n6, assign74200_body26_e113255_d_n7, assign74200_body26_e113255_d_n8, assign74200_body26_e113255_d_n9, assign74200_body26_e113255_d_n10, assign74200_body26_e113255_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1722 != 0.0)) && (locals.var_guard1728 == 0.0)) {
        let assign74200_body26_e113249: f64 = (locals.var_chi - locals.var_chib);
        let assign74200_body26_e113252: f64 = (locals.var_t0 - locals.var_t1);
        let assign74200_body26_e113253: f64 = (assign74200_body26_e113249 + assign74200_body26_e113252);
        (assign74200_body26_e113253, ((locals.var_chi_dn0 - locals.var_chib_dn0) + (locals.var_t0_dn0 - locals.var_t1_dn0)), ((locals.var_chi_dn2 - locals.var_chib_dn2) + (locals.var_t0_dn2 - locals.var_t1_dn2)), ((locals.var_chi_dn4 - locals.var_chib_dn4) + (locals.var_t0_dn4 - locals.var_t1_dn4)), ((locals.var_chi_dn5 - locals.var_chib_dn5) + (locals.var_t0_dn5 - locals.var_t1_dn5)), ((locals.var_chi_dn6 - locals.var_chib_dn6) + (locals.var_t0_dn6 - locals.var_t1_dn6)), ((locals.var_chi_dn7 - locals.var_chib_dn7) + (locals.var_t0_dn7 - locals.var_t1_dn7)), ((locals.var_chi_dn8 - locals.var_chib_dn8) + (locals.var_t0_dn8 - locals.var_t1_dn8)), ((locals.var_chi_dn9 - locals.var_chib_dn9) + (locals.var_t0_dn9 - locals.var_t1_dn9)), ((locals.var_chi_dn10 - locals.var_chib_dn10) + (locals.var_t0_dn10 - locals.var_t1_dn10)), ((locals.var_chi_dn13 - locals.var_chib_dn13) + (locals.var_t0_dn13 - locals.var_t1_dn13)),)
    } else {
        (locals.var_fbsq, locals.var_fbsq_dn0, locals.var_fbsq_dn2, locals.var_fbsq_dn4, locals.var_fbsq_dn5, locals.var_fbsq_dn6, locals.var_fbsq_dn7, locals.var_fbsq_dn8, locals.var_fbsq_dn9, locals.var_fbsq_dn10, locals.var_fbsq_dn13,)
    }
};
            locals.var_fbsq = assign74200_body26_e113255;
            locals.var_fbsq_dn0 = assign74200_body26_e113255_d_n0;
            locals.var_fbsq_dn2 = assign74200_body26_e113255_d_n2;
            locals.var_fbsq_dn4 = assign74200_body26_e113255_d_n4;
            locals.var_fbsq_dn5 = assign74200_body26_e113255_d_n5;
            locals.var_fbsq_dn6 = assign74200_body26_e113255_d_n6;
            locals.var_fbsq_dn7 = assign74200_body26_e113255_d_n7;
            locals.var_fbsq_dn8 = assign74200_body26_e113255_d_n8;
            locals.var_fbsq_dn9 = assign74200_body26_e113255_d_n9;
            locals.var_fbsq_dn10 = assign74200_body26_e113255_d_n10;
            locals.var_fbsq_dn13 = assign74200_body26_e113255_d_n13;
            locals.var_fbsq_rv = 0.0;
            let (assign74200_body27_e113274, assign74200_body27_e113274_d_n0, assign74200_body27_e113274_d_n2, assign74200_body27_e113274_d_n4, assign74200_body27_e113274_d_n5, assign74200_body27_e113274_d_n6, assign74200_body27_e113274_d_n7, assign74200_body27_e113274_d_n8, assign74200_body27_e113274_d_n9, assign74200_body27_e113274_d_n10, assign74200_body27_e113274_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1722 != 0.0)) && (locals.var_guard1728 == 0.0)) {
        let assign74200_body27_e113265: f64 = (1.0 - locals.var_t0);
        let assign74200_body27_e113269: f64 = (1.0 - locals.var_t1);
        let assign74200_body27_e113270: f64 = (locals.var_phi_b_dpss * assign74200_body27_e113269);
        let assign74200_body27_e113271: f64 = (assign74200_body27_e113265 - assign74200_body27_e113270);
        let assign74200_body27_e113272: f64 = (locals.var_beta * assign74200_body27_e113271);
        (assign74200_body27_e113272, ((locals.var_beta_dn0 * assign74200_body27_e113271) + (locals.var_beta * ((-locals.var_t0_dn0) - ((locals.var_phi_b_dpss_dn0 * assign74200_body27_e113269) + (locals.var_phi_b_dpss * (-locals.var_t1_dn0)))))), ((locals.var_beta_dn2 * assign74200_body27_e113271) + (locals.var_beta * ((-locals.var_t0_dn2) - ((locals.var_phi_b_dpss_dn2 * assign74200_body27_e113269) + (locals.var_phi_b_dpss * (-locals.var_t1_dn2)))))), ((locals.var_beta_dn4 * assign74200_body27_e113271) + (locals.var_beta * ((-locals.var_t0_dn4) - ((locals.var_phi_b_dpss_dn4 * assign74200_body27_e113269) + (locals.var_phi_b_dpss * (-locals.var_t1_dn4)))))), ((locals.var_beta_dn5 * assign74200_body27_e113271) + (locals.var_beta * ((-locals.var_t0_dn5) - ((locals.var_phi_b_dpss_dn5 * assign74200_body27_e113269) + (locals.var_phi_b_dpss * (-locals.var_t1_dn5)))))), ((locals.var_beta_dn6 * assign74200_body27_e113271) + (locals.var_beta * ((-locals.var_t0_dn6) - ((locals.var_phi_b_dpss_dn6 * assign74200_body27_e113269) + (locals.var_phi_b_dpss * (-locals.var_t1_dn6)))))), ((locals.var_beta_dn7 * assign74200_body27_e113271) + (locals.var_beta * ((-locals.var_t0_dn7) - ((locals.var_phi_b_dpss_dn7 * assign74200_body27_e113269) + (locals.var_phi_b_dpss * (-locals.var_t1_dn7)))))), ((locals.var_beta_dn8 * assign74200_body27_e113271) + (locals.var_beta * ((-locals.var_t0_dn8) - ((locals.var_phi_b_dpss_dn8 * assign74200_body27_e113269) + (locals.var_phi_b_dpss * (-locals.var_t1_dn8)))))), ((locals.var_beta_dn9 * assign74200_body27_e113271) + (locals.var_beta * ((-locals.var_t0_dn9) - ((locals.var_phi_b_dpss_dn9 * assign74200_body27_e113269) + (locals.var_phi_b_dpss * (-locals.var_t1_dn9)))))), ((locals.var_beta_dn10 * assign74200_body27_e113271) + (locals.var_beta * ((-locals.var_t0_dn10) - ((locals.var_phi_b_dpss_dn10 * assign74200_body27_e113269) + (locals.var_phi_b_dpss * (-locals.var_t1_dn10)))))), ((locals.var_beta_dn13 * assign74200_body27_e113271) + (locals.var_beta * ((-locals.var_t0_dn13) - ((locals.var_phi_b_dpss_dn13 * assign74200_body27_e113269) + (locals.var_phi_b_dpss * (-locals.var_t1_dn13)))))),)
    } else {
        (locals.var_fbsq_dpss, locals.var_fbsq_dpss_dn0, locals.var_fbsq_dpss_dn2, locals.var_fbsq_dpss_dn4, locals.var_fbsq_dpss_dn5, locals.var_fbsq_dpss_dn6, locals.var_fbsq_dpss_dn7, locals.var_fbsq_dpss_dn8, locals.var_fbsq_dpss_dn9, locals.var_fbsq_dpss_dn10, locals.var_fbsq_dpss_dn13,)
    }
};
            locals.var_fbsq_dpss = assign74200_body27_e113274;
            locals.var_fbsq_dpss_dn0 = assign74200_body27_e113274_d_n0;
            locals.var_fbsq_dpss_dn2 = assign74200_body27_e113274_d_n2;
            locals.var_fbsq_dpss_dn4 = assign74200_body27_e113274_d_n4;
            locals.var_fbsq_dpss_dn5 = assign74200_body27_e113274_d_n5;
            locals.var_fbsq_dpss_dn6 = assign74200_body27_e113274_d_n6;
            locals.var_fbsq_dpss_dn7 = assign74200_body27_e113274_d_n7;
            locals.var_fbsq_dpss_dn8 = assign74200_body27_e113274_d_n8;
            locals.var_fbsq_dpss_dn9 = assign74200_body27_e113274_d_n9;
            locals.var_fbsq_dpss_dn10 = assign74200_body27_e113274_d_n10;
            locals.var_fbsq_dpss_dn13 = assign74200_body27_e113274_d_n13;
            locals.var_fbsq_dpss_rv = 0.0;
            let assign74200_body28_e113276: f64 = (locals.var_chi).abs();
            let assign74200_body28_e113278: f64 = if assign74200_body28_e113276 < 5e-5 { 1.0 } else { 0.0 };
            locals.var_guard1729 = assign74200_body28_e113278;
            locals.var_guard1729_rv = 0.0;
            let (assign74200_body29_e113308, assign74200_body29_e113308_d_n0, assign74200_body29_e113308_d_n2, assign74200_body29_e113308_d_n4, assign74200_body29_e113308_d_n5, assign74200_body29_e113308_d_n6, assign74200_body29_e113308_d_n7, assign74200_body29_e113308_d_n8, assign74200_body29_e113308_d_n9, assign74200_body29_e113308_d_n10, assign74200_body29_e113308_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1722 != 0.0)) && (locals.var_guard1729 != 0.0)) {
        let assign74200_body29_e113286: f64 = (locals.var_chi * locals.var_chi);
        let assign74200_body29_e113288: f64 = (assign74200_body29_e113286 / 2.0);
        let assign74200_body29_e113292: f64 = (locals.var_chi / 3.0);
        let assign74200_body29_e113296: f64 = (locals.var_chi / 4.0);
        let assign74200_body29_e113300: f64 = (locals.var_chi / 5.0);
        let assign74200_body29_e113301: f64 = (1.0 + assign74200_body29_e113300);
        let assign74200_body29_e113302: f64 = (assign74200_body29_e113296 * assign74200_body29_e113301);
        let assign74200_body29_e113303: f64 = (1.0 + assign74200_body29_e113302);
        let assign74200_body29_e113304: f64 = (assign74200_body29_e113292 * assign74200_body29_e113303);
        let assign74200_body29_e113305: f64 = (1.0 + assign74200_body29_e113304);
        let assign74200_body29_e113306: f64 = (assign74200_body29_e113288 * assign74200_body29_e113305);
        (assign74200_body29_e113306, (((((locals.var_chi_dn0 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn0)) / 2.0) * assign74200_body29_e113305) + (assign74200_body29_e113288 * (((locals.var_chi_dn0 / 3.0) * assign74200_body29_e113303) + (assign74200_body29_e113292 * (((locals.var_chi_dn0 / 4.0) * assign74200_body29_e113301) + (assign74200_body29_e113296 * (locals.var_chi_dn0 / 5.0))))))), (((((locals.var_chi_dn2 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn2)) / 2.0) * assign74200_body29_e113305) + (assign74200_body29_e113288 * (((locals.var_chi_dn2 / 3.0) * assign74200_body29_e113303) + (assign74200_body29_e113292 * (((locals.var_chi_dn2 / 4.0) * assign74200_body29_e113301) + (assign74200_body29_e113296 * (locals.var_chi_dn2 / 5.0))))))), (((((locals.var_chi_dn4 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn4)) / 2.0) * assign74200_body29_e113305) + (assign74200_body29_e113288 * (((locals.var_chi_dn4 / 3.0) * assign74200_body29_e113303) + (assign74200_body29_e113292 * (((locals.var_chi_dn4 / 4.0) * assign74200_body29_e113301) + (assign74200_body29_e113296 * (locals.var_chi_dn4 / 5.0))))))), (((((locals.var_chi_dn5 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn5)) / 2.0) * assign74200_body29_e113305) + (assign74200_body29_e113288 * (((locals.var_chi_dn5 / 3.0) * assign74200_body29_e113303) + (assign74200_body29_e113292 * (((locals.var_chi_dn5 / 4.0) * assign74200_body29_e113301) + (assign74200_body29_e113296 * (locals.var_chi_dn5 / 5.0))))))), (((((locals.var_chi_dn6 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn6)) / 2.0) * assign74200_body29_e113305) + (assign74200_body29_e113288 * (((locals.var_chi_dn6 / 3.0) * assign74200_body29_e113303) + (assign74200_body29_e113292 * (((locals.var_chi_dn6 / 4.0) * assign74200_body29_e113301) + (assign74200_body29_e113296 * (locals.var_chi_dn6 / 5.0))))))), (((((locals.var_chi_dn7 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn7)) / 2.0) * assign74200_body29_e113305) + (assign74200_body29_e113288 * (((locals.var_chi_dn7 / 3.0) * assign74200_body29_e113303) + (assign74200_body29_e113292 * (((locals.var_chi_dn7 / 4.0) * assign74200_body29_e113301) + (assign74200_body29_e113296 * (locals.var_chi_dn7 / 5.0))))))), (((((locals.var_chi_dn8 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn8)) / 2.0) * assign74200_body29_e113305) + (assign74200_body29_e113288 * (((locals.var_chi_dn8 / 3.0) * assign74200_body29_e113303) + (assign74200_body29_e113292 * (((locals.var_chi_dn8 / 4.0) * assign74200_body29_e113301) + (assign74200_body29_e113296 * (locals.var_chi_dn8 / 5.0))))))), (((((locals.var_chi_dn9 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn9)) / 2.0) * assign74200_body29_e113305) + (assign74200_body29_e113288 * (((locals.var_chi_dn9 / 3.0) * assign74200_body29_e113303) + (assign74200_body29_e113292 * (((locals.var_chi_dn9 / 4.0) * assign74200_body29_e113301) + (assign74200_body29_e113296 * (locals.var_chi_dn9 / 5.0))))))), (((((locals.var_chi_dn10 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn10)) / 2.0) * assign74200_body29_e113305) + (assign74200_body29_e113288 * (((locals.var_chi_dn10 / 3.0) * assign74200_body29_e113303) + (assign74200_body29_e113292 * (((locals.var_chi_dn10 / 4.0) * assign74200_body29_e113301) + (assign74200_body29_e113296 * (locals.var_chi_dn10 / 5.0))))))), (((((locals.var_chi_dn13 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn13)) / 2.0) * assign74200_body29_e113305) + (assign74200_body29_e113288 * (((locals.var_chi_dn13 / 3.0) * assign74200_body29_e113303) + (assign74200_body29_e113292 * (((locals.var_chi_dn13 / 4.0) * assign74200_body29_e113301) + (assign74200_body29_e113296 * (locals.var_chi_dn13 / 5.0))))))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
            locals.var_t0 = assign74200_body29_e113308;
            locals.var_t0_dn0 = assign74200_body29_e113308_d_n0;
            locals.var_t0_dn2 = assign74200_body29_e113308_d_n2;
            locals.var_t0_dn4 = assign74200_body29_e113308_d_n4;
            locals.var_t0_dn5 = assign74200_body29_e113308_d_n5;
            locals.var_t0_dn6 = assign74200_body29_e113308_d_n6;
            locals.var_t0_dn7 = assign74200_body29_e113308_d_n7;
            locals.var_t0_dn8 = assign74200_body29_e113308_d_n8;
            locals.var_t0_dn9 = assign74200_body29_e113308_d_n9;
            locals.var_t0_dn10 = assign74200_body29_e113308_d_n10;
            locals.var_t0_dn13 = assign74200_body29_e113308_d_n13;
            locals.var_t0_rv = 0.0;
            let (assign74200_body30_e113334, assign74200_body30_e113334_d_n0, assign74200_body30_e113334_d_n2, assign74200_body30_e113334_d_n4, assign74200_body30_e113334_d_n5, assign74200_body30_e113334_d_n6, assign74200_body30_e113334_d_n7, assign74200_body30_e113334_d_n8, assign74200_body30_e113334_d_n9, assign74200_body30_e113334_d_n10, assign74200_body30_e113334_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1722 != 0.0)) && (locals.var_guard1729 != 0.0)) {
        let assign74200_body30_e113318: f64 = (locals.var_chi / 2.0);
        let assign74200_body30_e113322: f64 = (locals.var_chi / 3.0);
        let assign74200_body30_e113326: f64 = (locals.var_chi / 4.0);
        let assign74200_body30_e113327: f64 = (1.0 + assign74200_body30_e113326);
        let assign74200_body30_e113328: f64 = (assign74200_body30_e113322 * assign74200_body30_e113327);
        let assign74200_body30_e113329: f64 = (1.0 + assign74200_body30_e113328);
        let assign74200_body30_e113330: f64 = (assign74200_body30_e113318 * assign74200_body30_e113329);
        let assign74200_body30_e113331: f64 = (1.0 + assign74200_body30_e113330);
        let assign74200_body30_e113332: f64 = (locals.var_chi * assign74200_body30_e113331);
        (assign74200_body30_e113332, ((locals.var_chi_dn0 * assign74200_body30_e113331) + (locals.var_chi * (((locals.var_chi_dn0 / 2.0) * assign74200_body30_e113329) + (assign74200_body30_e113318 * (((locals.var_chi_dn0 / 3.0) * assign74200_body30_e113327) + (assign74200_body30_e113322 * (locals.var_chi_dn0 / 4.0))))))), ((locals.var_chi_dn2 * assign74200_body30_e113331) + (locals.var_chi * (((locals.var_chi_dn2 / 2.0) * assign74200_body30_e113329) + (assign74200_body30_e113318 * (((locals.var_chi_dn2 / 3.0) * assign74200_body30_e113327) + (assign74200_body30_e113322 * (locals.var_chi_dn2 / 4.0))))))), ((locals.var_chi_dn4 * assign74200_body30_e113331) + (locals.var_chi * (((locals.var_chi_dn4 / 2.0) * assign74200_body30_e113329) + (assign74200_body30_e113318 * (((locals.var_chi_dn4 / 3.0) * assign74200_body30_e113327) + (assign74200_body30_e113322 * (locals.var_chi_dn4 / 4.0))))))), ((locals.var_chi_dn5 * assign74200_body30_e113331) + (locals.var_chi * (((locals.var_chi_dn5 / 2.0) * assign74200_body30_e113329) + (assign74200_body30_e113318 * (((locals.var_chi_dn5 / 3.0) * assign74200_body30_e113327) + (assign74200_body30_e113322 * (locals.var_chi_dn5 / 4.0))))))), ((locals.var_chi_dn6 * assign74200_body30_e113331) + (locals.var_chi * (((locals.var_chi_dn6 / 2.0) * assign74200_body30_e113329) + (assign74200_body30_e113318 * (((locals.var_chi_dn6 / 3.0) * assign74200_body30_e113327) + (assign74200_body30_e113322 * (locals.var_chi_dn6 / 4.0))))))), ((locals.var_chi_dn7 * assign74200_body30_e113331) + (locals.var_chi * (((locals.var_chi_dn7 / 2.0) * assign74200_body30_e113329) + (assign74200_body30_e113318 * (((locals.var_chi_dn7 / 3.0) * assign74200_body30_e113327) + (assign74200_body30_e113322 * (locals.var_chi_dn7 / 4.0))))))), ((locals.var_chi_dn8 * assign74200_body30_e113331) + (locals.var_chi * (((locals.var_chi_dn8 / 2.0) * assign74200_body30_e113329) + (assign74200_body30_e113318 * (((locals.var_chi_dn8 / 3.0) * assign74200_body30_e113327) + (assign74200_body30_e113322 * (locals.var_chi_dn8 / 4.0))))))), ((locals.var_chi_dn9 * assign74200_body30_e113331) + (locals.var_chi * (((locals.var_chi_dn9 / 2.0) * assign74200_body30_e113329) + (assign74200_body30_e113318 * (((locals.var_chi_dn9 / 3.0) * assign74200_body30_e113327) + (assign74200_body30_e113322 * (locals.var_chi_dn9 / 4.0))))))), ((locals.var_chi_dn10 * assign74200_body30_e113331) + (locals.var_chi * (((locals.var_chi_dn10 / 2.0) * assign74200_body30_e113329) + (assign74200_body30_e113318 * (((locals.var_chi_dn10 / 3.0) * assign74200_body30_e113327) + (assign74200_body30_e113322 * (locals.var_chi_dn10 / 4.0))))))), ((locals.var_chi_dn13 * assign74200_body30_e113331) + (locals.var_chi * (((locals.var_chi_dn13 / 2.0) * assign74200_body30_e113329) + (assign74200_body30_e113318 * (((locals.var_chi_dn13 / 3.0) * assign74200_body30_e113327) + (assign74200_body30_e113322 * (locals.var_chi_dn13 / 4.0))))))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
            locals.var_t1 = assign74200_body30_e113334;
            locals.var_t1_dn0 = assign74200_body30_e113334_d_n0;
            locals.var_t1_dn2 = assign74200_body30_e113334_d_n2;
            locals.var_t1_dn4 = assign74200_body30_e113334_d_n4;
            locals.var_t1_dn5 = assign74200_body30_e113334_d_n5;
            locals.var_t1_dn6 = assign74200_body30_e113334_d_n6;
            locals.var_t1_dn7 = assign74200_body30_e113334_d_n7;
            locals.var_t1_dn8 = assign74200_body30_e113334_d_n8;
            locals.var_t1_dn9 = assign74200_body30_e113334_d_n9;
            locals.var_t1_dn10 = assign74200_body30_e113334_d_n10;
            locals.var_t1_dn13 = assign74200_body30_e113334_d_n13;
            locals.var_t1_rv = 0.0;
            let (assign74200_body31_e113344, assign74200_body31_e113344_d_n0, assign74200_body31_e113344_d_n2, assign74200_body31_e113344_d_n4, assign74200_body31_e113344_d_n5, assign74200_body31_e113344_d_n6, assign74200_body31_e113344_d_n7, assign74200_body31_e113344_d_n8, assign74200_body31_e113344_d_n9, assign74200_body31_e113344_d_n10, assign74200_body31_e113344_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1722 != 0.0)) && (locals.var_guard1729 != 0.0)) {
        let assign74200_body31_e113342: f64 = (locals.var_cfs1 * locals.var_t0);
        (assign74200_body31_e113342, ((locals.var_cfs1_dn0 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn0)), ((locals.var_cfs1_dn2 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn2)), ((locals.var_cfs1_dn4 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn4)), ((locals.var_cfs1_dn5 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn5)), ((locals.var_cfs1_dn6 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn6)), ((locals.var_cfs1_dn7 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn7)), ((locals.var_cfs1_dn8 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn8)), ((locals.var_cfs1_dn9 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn9)), ((locals.var_cfs1_dn10 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn10)), ((locals.var_cfs1_dn13 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn13)),)
    } else {
        (locals.var_fs01, locals.var_fs01_dn0, locals.var_fs01_dn2, locals.var_fs01_dn4, locals.var_fs01_dn5, locals.var_fs01_dn6, locals.var_fs01_dn7, locals.var_fs01_dn8, locals.var_fs01_dn9, locals.var_fs01_dn10, locals.var_fs01_dn13,)
    }
};
            locals.var_fs01 = assign74200_body31_e113344;
            locals.var_fs01_dn0 = assign74200_body31_e113344_d_n0;
            locals.var_fs01_dn2 = assign74200_body31_e113344_d_n2;
            locals.var_fs01_dn4 = assign74200_body31_e113344_d_n4;
            locals.var_fs01_dn5 = assign74200_body31_e113344_d_n5;
            locals.var_fs01_dn6 = assign74200_body31_e113344_d_n6;
            locals.var_fs01_dn7 = assign74200_body31_e113344_d_n7;
            locals.var_fs01_dn8 = assign74200_body31_e113344_d_n8;
            locals.var_fs01_dn9 = assign74200_body31_e113344_d_n9;
            locals.var_fs01_dn10 = assign74200_body31_e113344_d_n10;
            locals.var_fs01_dn13 = assign74200_body31_e113344_d_n13;
            locals.var_fs01_rv = 0.0;
            let (assign74200_body32_e113356, assign74200_body32_e113356_d_n0, assign74200_body32_e113356_d_n2, assign74200_body32_e113356_d_n4, assign74200_body32_e113356_d_n5, assign74200_body32_e113356_d_n6, assign74200_body32_e113356_d_n7, assign74200_body32_e113356_d_n8, assign74200_body32_e113356_d_n9, assign74200_body32_e113356_d_n10, assign74200_body32_e113356_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1722 != 0.0)) && (locals.var_guard1729 != 0.0)) {
        let assign74200_body32_e113352: f64 = (locals.var_cfs1 * locals.var_t1);
        let assign74200_body32_e113354: f64 = (assign74200_body32_e113352 * locals.var_beta);
        (assign74200_body32_e113354, ((((locals.var_cfs1_dn0 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn0)) * locals.var_beta) + (assign74200_body32_e113352 * locals.var_beta_dn0)), ((((locals.var_cfs1_dn2 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn2)) * locals.var_beta) + (assign74200_body32_e113352 * locals.var_beta_dn2)), ((((locals.var_cfs1_dn4 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn4)) * locals.var_beta) + (assign74200_body32_e113352 * locals.var_beta_dn4)), ((((locals.var_cfs1_dn5 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn5)) * locals.var_beta) + (assign74200_body32_e113352 * locals.var_beta_dn5)), ((((locals.var_cfs1_dn6 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn6)) * locals.var_beta) + (assign74200_body32_e113352 * locals.var_beta_dn6)), ((((locals.var_cfs1_dn7 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn7)) * locals.var_beta) + (assign74200_body32_e113352 * locals.var_beta_dn7)), ((((locals.var_cfs1_dn8 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn8)) * locals.var_beta) + (assign74200_body32_e113352 * locals.var_beta_dn8)), ((((locals.var_cfs1_dn9 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn9)) * locals.var_beta) + (assign74200_body32_e113352 * locals.var_beta_dn9)), ((((locals.var_cfs1_dn10 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn10)) * locals.var_beta) + (assign74200_body32_e113352 * locals.var_beta_dn10)), ((((locals.var_cfs1_dn13 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn13)) * locals.var_beta) + (assign74200_body32_e113352 * locals.var_beta_dn13)),)
    } else {
        (locals.var_fs01_dps0, locals.var_fs01_dps0_dn0, locals.var_fs01_dps0_dn2, locals.var_fs01_dps0_dn4, locals.var_fs01_dps0_dn5, locals.var_fs01_dps0_dn6, locals.var_fs01_dps0_dn7, locals.var_fs01_dps0_dn8, locals.var_fs01_dps0_dn9, locals.var_fs01_dps0_dn10, locals.var_fs01_dps0_dn13,)
    }
};
            locals.var_fs01_dps0 = assign74200_body32_e113356;
            locals.var_fs01_dps0_dn0 = assign74200_body32_e113356_d_n0;
            locals.var_fs01_dps0_dn2 = assign74200_body32_e113356_d_n2;
            locals.var_fs01_dps0_dn4 = assign74200_body32_e113356_d_n4;
            locals.var_fs01_dps0_dn5 = assign74200_body32_e113356_d_n5;
            locals.var_fs01_dps0_dn6 = assign74200_body32_e113356_d_n6;
            locals.var_fs01_dps0_dn7 = assign74200_body32_e113356_d_n7;
            locals.var_fs01_dps0_dn8 = assign74200_body32_e113356_d_n8;
            locals.var_fs01_dps0_dn9 = assign74200_body32_e113356_d_n9;
            locals.var_fs01_dps0_dn10 = assign74200_body32_e113356_d_n10;
            locals.var_fs01_dps0_dn13 = assign74200_body32_e113356_d_n13;
            locals.var_fs01_dps0_rv = 0.0;
            let assign74200_body33_e113358: f64 = (locals.var_chi).abs();
            let assign74200_body33_e113360: f64 = if assign74200_body33_e113358 < 60.0 { 1.0 } else { 0.0 };
            locals.var_guard1730 = assign74200_body33_e113360;
            locals.var_guard1730_rv = 0.0;
            let (assign74200_body35_e113391, assign74200_body35_e113391_d_n0, assign74200_body35_e113391_d_n2, assign74200_body35_e113391_d_n4, assign74200_body35_e113391_d_n5, assign74200_body35_e113391_d_n6, assign74200_body35_e113391_d_n7, assign74200_body35_e113391_d_n8, assign74200_body35_e113391_d_n9, assign74200_body35_e113391_d_n10, assign74200_body35_e113391_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1722 != 0.0)) && (locals.var_guard1729 == 0.0)) && (locals.var_guard1730 != 0.0)) {
        let assign74200_body35_e113389: f64 = (locals.var_chi).exp();
        (assign74200_body35_e113389, (assign74200_body35_e113389 * locals.var_chi_dn0), (assign74200_body35_e113389 * locals.var_chi_dn2), (assign74200_body35_e113389 * locals.var_chi_dn4), (assign74200_body35_e113389 * locals.var_chi_dn5), (assign74200_body35_e113389 * locals.var_chi_dn6), (assign74200_body35_e113389 * locals.var_chi_dn7), (assign74200_body35_e113389 * locals.var_chi_dn8), (assign74200_body35_e113389 * locals.var_chi_dn9), (assign74200_body35_e113389 * locals.var_chi_dn10), (assign74200_body35_e113389 * locals.var_chi_dn13),)
    } else {
        (locals.var_exp_chi, locals.var_exp_chi_dn0, locals.var_exp_chi_dn2, locals.var_exp_chi_dn4, locals.var_exp_chi_dn5, locals.var_exp_chi_dn6, locals.var_exp_chi_dn7, locals.var_exp_chi_dn8, locals.var_exp_chi_dn9, locals.var_exp_chi_dn10, locals.var_exp_chi_dn13,)
    }
};
            locals.var_exp_chi = assign74200_body35_e113391;
            locals.var_exp_chi_dn0 = assign74200_body35_e113391_d_n0;
            locals.var_exp_chi_dn2 = assign74200_body35_e113391_d_n2;
            locals.var_exp_chi_dn4 = assign74200_body35_e113391_d_n4;
            locals.var_exp_chi_dn5 = assign74200_body35_e113391_d_n5;
            locals.var_exp_chi_dn6 = assign74200_body35_e113391_d_n6;
            locals.var_exp_chi_dn7 = assign74200_body35_e113391_d_n7;
            locals.var_exp_chi_dn8 = assign74200_body35_e113391_d_n8;
            locals.var_exp_chi_dn9 = assign74200_body35_e113391_d_n9;
            locals.var_exp_chi_dn10 = assign74200_body35_e113391_d_n10;
            locals.var_exp_chi_dn13 = assign74200_body35_e113391_d_n13;
            locals.var_exp_chi_rv = 0.0;
            let (assign74200_body36_e113404, assign74200_body36_e113404_d_n0, assign74200_body36_e113404_d_n2, assign74200_body36_e113404_d_n4, assign74200_body36_e113404_d_n5, assign74200_body36_e113404_d_n6, assign74200_body36_e113404_d_n7, assign74200_body36_e113404_d_n8, assign74200_body36_e113404_d_n9, assign74200_body36_e113404_d_n10, assign74200_body36_e113404_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1722 != 0.0)) && (locals.var_guard1729 == 0.0)) && (locals.var_guard1730 != 0.0)) {
        let assign74200_body36_e113402: f64 = (locals.var_exp_chi - 1.0);
        (assign74200_body36_e113402, locals.var_exp_chi_dn0, locals.var_exp_chi_dn2, locals.var_exp_chi_dn4, locals.var_exp_chi_dn5, locals.var_exp_chi_dn6, locals.var_exp_chi_dn7, locals.var_exp_chi_dn8, locals.var_exp_chi_dn9, locals.var_exp_chi_dn10, locals.var_exp_chi_dn13,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
            locals.var_t1 = assign74200_body36_e113404;
            locals.var_t1_dn0 = assign74200_body36_e113404_d_n0;
            locals.var_t1_dn2 = assign74200_body36_e113404_d_n2;
            locals.var_t1_dn4 = assign74200_body36_e113404_d_n4;
            locals.var_t1_dn5 = assign74200_body36_e113404_d_n5;
            locals.var_t1_dn6 = assign74200_body36_e113404_d_n6;
            locals.var_t1_dn7 = assign74200_body36_e113404_d_n7;
            locals.var_t1_dn8 = assign74200_body36_e113404_d_n8;
            locals.var_t1_dn9 = assign74200_body36_e113404_d_n9;
            locals.var_t1_dn10 = assign74200_body36_e113404_d_n10;
            locals.var_t1_dn13 = assign74200_body36_e113404_d_n13;
            locals.var_t1_rv = 0.0;
            let (assign74200_body37_e113419, assign74200_body37_e113419_d_n0, assign74200_body37_e113419_d_n2, assign74200_body37_e113419_d_n4, assign74200_body37_e113419_d_n5, assign74200_body37_e113419_d_n6, assign74200_body37_e113419_d_n7, assign74200_body37_e113419_d_n8, assign74200_body37_e113419_d_n9, assign74200_body37_e113419_d_n10, assign74200_body37_e113419_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1722 != 0.0)) && (locals.var_guard1729 == 0.0)) && (locals.var_guard1730 != 0.0)) {
        let assign74200_body37_e113416: f64 = (locals.var_t1 - locals.var_chi);
        let assign74200_body37_e113417: f64 = (locals.var_cfs1 * assign74200_body37_e113416);
        (assign74200_body37_e113417, ((locals.var_cfs1_dn0 * assign74200_body37_e113416) + (locals.var_cfs1 * (locals.var_t1_dn0 - locals.var_chi_dn0))), ((locals.var_cfs1_dn2 * assign74200_body37_e113416) + (locals.var_cfs1 * (locals.var_t1_dn2 - locals.var_chi_dn2))), ((locals.var_cfs1_dn4 * assign74200_body37_e113416) + (locals.var_cfs1 * (locals.var_t1_dn4 - locals.var_chi_dn4))), ((locals.var_cfs1_dn5 * assign74200_body37_e113416) + (locals.var_cfs1 * (locals.var_t1_dn5 - locals.var_chi_dn5))), ((locals.var_cfs1_dn6 * assign74200_body37_e113416) + (locals.var_cfs1 * (locals.var_t1_dn6 - locals.var_chi_dn6))), ((locals.var_cfs1_dn7 * assign74200_body37_e113416) + (locals.var_cfs1 * (locals.var_t1_dn7 - locals.var_chi_dn7))), ((locals.var_cfs1_dn8 * assign74200_body37_e113416) + (locals.var_cfs1 * (locals.var_t1_dn8 - locals.var_chi_dn8))), ((locals.var_cfs1_dn9 * assign74200_body37_e113416) + (locals.var_cfs1 * (locals.var_t1_dn9 - locals.var_chi_dn9))), ((locals.var_cfs1_dn10 * assign74200_body37_e113416) + (locals.var_cfs1 * (locals.var_t1_dn10 - locals.var_chi_dn10))), ((locals.var_cfs1_dn13 * assign74200_body37_e113416) + (locals.var_cfs1 * (locals.var_t1_dn13 - locals.var_chi_dn13))),)
    } else {
        (locals.var_fs01, locals.var_fs01_dn0, locals.var_fs01_dn2, locals.var_fs01_dn4, locals.var_fs01_dn5, locals.var_fs01_dn6, locals.var_fs01_dn7, locals.var_fs01_dn8, locals.var_fs01_dn9, locals.var_fs01_dn10, locals.var_fs01_dn13,)
    }
};
            locals.var_fs01 = assign74200_body37_e113419;
            locals.var_fs01_dn0 = assign74200_body37_e113419_d_n0;
            locals.var_fs01_dn2 = assign74200_body37_e113419_d_n2;
            locals.var_fs01_dn4 = assign74200_body37_e113419_d_n4;
            locals.var_fs01_dn5 = assign74200_body37_e113419_d_n5;
            locals.var_fs01_dn6 = assign74200_body37_e113419_d_n6;
            locals.var_fs01_dn7 = assign74200_body37_e113419_d_n7;
            locals.var_fs01_dn8 = assign74200_body37_e113419_d_n8;
            locals.var_fs01_dn9 = assign74200_body37_e113419_d_n9;
            locals.var_fs01_dn10 = assign74200_body37_e113419_d_n10;
            locals.var_fs01_dn13 = assign74200_body37_e113419_d_n13;
            locals.var_fs01_rv = 0.0;
            let (assign74200_body38_e113434, assign74200_body38_e113434_d_n0, assign74200_body38_e113434_d_n2, assign74200_body38_e113434_d_n4, assign74200_body38_e113434_d_n5, assign74200_body38_e113434_d_n6, assign74200_body38_e113434_d_n7, assign74200_body38_e113434_d_n8, assign74200_body38_e113434_d_n9, assign74200_body38_e113434_d_n10, assign74200_body38_e113434_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1722 != 0.0)) && (locals.var_guard1729 == 0.0)) && (locals.var_guard1730 != 0.0)) {
        let assign74200_body38_e113430: f64 = (locals.var_cfs1 * locals.var_beta);
        let assign74200_body38_e113432: f64 = (assign74200_body38_e113430 * locals.var_t1);
        (assign74200_body38_e113432, ((((locals.var_cfs1_dn0 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn0)) * locals.var_t1) + (assign74200_body38_e113430 * locals.var_t1_dn0)), ((((locals.var_cfs1_dn2 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn2)) * locals.var_t1) + (assign74200_body38_e113430 * locals.var_t1_dn2)), ((((locals.var_cfs1_dn4 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn4)) * locals.var_t1) + (assign74200_body38_e113430 * locals.var_t1_dn4)), ((((locals.var_cfs1_dn5 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn5)) * locals.var_t1) + (assign74200_body38_e113430 * locals.var_t1_dn5)), ((((locals.var_cfs1_dn6 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn6)) * locals.var_t1) + (assign74200_body38_e113430 * locals.var_t1_dn6)), ((((locals.var_cfs1_dn7 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn7)) * locals.var_t1) + (assign74200_body38_e113430 * locals.var_t1_dn7)), ((((locals.var_cfs1_dn8 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn8)) * locals.var_t1) + (assign74200_body38_e113430 * locals.var_t1_dn8)), ((((locals.var_cfs1_dn9 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn9)) * locals.var_t1) + (assign74200_body38_e113430 * locals.var_t1_dn9)), ((((locals.var_cfs1_dn10 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn10)) * locals.var_t1) + (assign74200_body38_e113430 * locals.var_t1_dn10)), ((((locals.var_cfs1_dn13 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn13)) * locals.var_t1) + (assign74200_body38_e113430 * locals.var_t1_dn13)),)
    } else {
        (locals.var_fs01_dps0, locals.var_fs01_dps0_dn0, locals.var_fs01_dps0_dn2, locals.var_fs01_dps0_dn4, locals.var_fs01_dps0_dn5, locals.var_fs01_dps0_dn6, locals.var_fs01_dps0_dn7, locals.var_fs01_dps0_dn8, locals.var_fs01_dps0_dn9, locals.var_fs01_dps0_dn10, locals.var_fs01_dps0_dn13,)
    }
};
            locals.var_fs01_dps0 = assign74200_body38_e113434;
            locals.var_fs01_dps0_dn0 = assign74200_body38_e113434_d_n0;
            locals.var_fs01_dps0_dn2 = assign74200_body38_e113434_d_n2;
            locals.var_fs01_dps0_dn4 = assign74200_body38_e113434_d_n4;
            locals.var_fs01_dps0_dn5 = assign74200_body38_e113434_d_n5;
            locals.var_fs01_dps0_dn6 = assign74200_body38_e113434_d_n6;
            locals.var_fs01_dps0_dn7 = assign74200_body38_e113434_d_n7;
            locals.var_fs01_dps0_dn8 = assign74200_body38_e113434_d_n8;
            locals.var_fs01_dps0_dn9 = assign74200_body38_e113434_d_n9;
            locals.var_fs01_dps0_dn10 = assign74200_body38_e113434_d_n10;
            locals.var_fs01_dps0_dn13 = assign74200_body38_e113434_d_n13;
            locals.var_fs01_dps0_rv = 0.0;
            let (assign74200_body40_e113469, assign74200_body40_e113469_d_n0, assign74200_body40_e113469_d_n2, assign74200_body40_e113469_d_n4, assign74200_body40_e113469_d_n5, assign74200_body40_e113469_d_n6, assign74200_body40_e113469_d_n7, assign74200_body40_e113469_d_n8, assign74200_body40_e113469_d_n9, assign74200_body40_e113469_d_n10, assign74200_body40_e113469_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1722 != 0.0)) && (locals.var_guard1729 == 0.0)) && (locals.var_guard1730 == 0.0)) {
        let assign74200_body40_e113466: f64 = (locals.var_beta * locals.var_ps0ld);
        let assign74200_body40_e113467: f64 = (assign74200_body40_e113466).exp();
        (assign74200_body40_e113467, (assign74200_body40_e113467 * ((locals.var_beta_dn0 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn0))), (assign74200_body40_e113467 * ((locals.var_beta_dn2 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn2))), (assign74200_body40_e113467 * ((locals.var_beta_dn4 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn4))), (assign74200_body40_e113467 * ((locals.var_beta_dn5 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn5))), (assign74200_body40_e113467 * ((locals.var_beta_dn6 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn6))), (assign74200_body40_e113467 * ((locals.var_beta_dn7 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn7))), (assign74200_body40_e113467 * ((locals.var_beta_dn8 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn8))), (assign74200_body40_e113467 * ((locals.var_beta_dn9 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn9))), (assign74200_body40_e113467 * ((locals.var_beta_dn10 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn10))), (assign74200_body40_e113467 * ((locals.var_beta_dn13 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn13))),)
    } else {
        (locals.var_exp_bps0, locals.var_exp_bps0_dn0, locals.var_exp_bps0_dn2, locals.var_exp_bps0_dn4, locals.var_exp_bps0_dn5, locals.var_exp_bps0_dn6, locals.var_exp_bps0_dn7, locals.var_exp_bps0_dn8, locals.var_exp_bps0_dn9, locals.var_exp_bps0_dn10, locals.var_exp_bps0_dn13,)
    }
};
            locals.var_exp_bps0 = assign74200_body40_e113469;
            locals.var_exp_bps0_dn0 = assign74200_body40_e113469_d_n0;
            locals.var_exp_bps0_dn2 = assign74200_body40_e113469_d_n2;
            locals.var_exp_bps0_dn4 = assign74200_body40_e113469_d_n4;
            locals.var_exp_bps0_dn5 = assign74200_body40_e113469_d_n5;
            locals.var_exp_bps0_dn6 = assign74200_body40_e113469_d_n6;
            locals.var_exp_bps0_dn7 = assign74200_body40_e113469_d_n7;
            locals.var_exp_bps0_dn8 = assign74200_body40_e113469_d_n8;
            locals.var_exp_bps0_dn9 = assign74200_body40_e113469_d_n9;
            locals.var_exp_bps0_dn10 = assign74200_body40_e113469_d_n10;
            locals.var_exp_bps0_dn13 = assign74200_body40_e113469_d_n13;
            locals.var_exp_bps0_rv = 0.0;
            let (assign74200_body41_e113489, assign74200_body41_e113489_d_n0, assign74200_body41_e113489_d_n2, assign74200_body41_e113489_d_n4, assign74200_body41_e113489_d_n5, assign74200_body41_e113489_d_n6, assign74200_body41_e113489_d_n7, assign74200_body41_e113489_d_n8, assign74200_body41_e113489_d_n9, assign74200_body41_e113489_d_n10, assign74200_body41_e113489_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1722 != 0.0)) && (locals.var_guard1729 == 0.0)) && (locals.var_guard1730 == 0.0)) {
        let assign74200_body41_e113484: f64 = (locals.var_chi + 1.0);
        let assign74200_body41_e113485: f64 = (locals.var_exp_bvbs * assign74200_body41_e113484);
        let assign74200_body41_e113486: f64 = (locals.var_exp_bps0 - assign74200_body41_e113485);
        let assign74200_body41_e113487: f64 = (locals.var_cnst1over * assign74200_body41_e113486);
        (assign74200_body41_e113487, ((locals.var_cnst1over_dn0 * assign74200_body41_e113486) + (locals.var_cnst1over * (locals.var_exp_bps0_dn0 - ((locals.var_exp_bvbs_dn0 * assign74200_body41_e113484) + (locals.var_exp_bvbs * locals.var_chi_dn0))))), ((locals.var_cnst1over_dn2 * assign74200_body41_e113486) + (locals.var_cnst1over * (locals.var_exp_bps0_dn2 - ((locals.var_exp_bvbs_dn2 * assign74200_body41_e113484) + (locals.var_exp_bvbs * locals.var_chi_dn2))))), ((locals.var_cnst1over_dn4 * assign74200_body41_e113486) + (locals.var_cnst1over * (locals.var_exp_bps0_dn4 - ((locals.var_exp_bvbs_dn4 * assign74200_body41_e113484) + (locals.var_exp_bvbs * locals.var_chi_dn4))))), ((locals.var_cnst1over_dn5 * assign74200_body41_e113486) + (locals.var_cnst1over * (locals.var_exp_bps0_dn5 - ((locals.var_exp_bvbs_dn5 * assign74200_body41_e113484) + (locals.var_exp_bvbs * locals.var_chi_dn5))))), ((locals.var_cnst1over_dn6 * assign74200_body41_e113486) + (locals.var_cnst1over * (locals.var_exp_bps0_dn6 - ((locals.var_exp_bvbs_dn6 * assign74200_body41_e113484) + (locals.var_exp_bvbs * locals.var_chi_dn6))))), ((locals.var_cnst1over_dn7 * assign74200_body41_e113486) + (locals.var_cnst1over * (locals.var_exp_bps0_dn7 - ((locals.var_exp_bvbs_dn7 * assign74200_body41_e113484) + (locals.var_exp_bvbs * locals.var_chi_dn7))))), ((locals.var_cnst1over_dn8 * assign74200_body41_e113486) + (locals.var_cnst1over * (locals.var_exp_bps0_dn8 - ((locals.var_exp_bvbs_dn8 * assign74200_body41_e113484) + (locals.var_exp_bvbs * locals.var_chi_dn8))))), ((locals.var_cnst1over_dn9 * assign74200_body41_e113486) + (locals.var_cnst1over * (locals.var_exp_bps0_dn9 - ((locals.var_exp_bvbs_dn9 * assign74200_body41_e113484) + (locals.var_exp_bvbs * locals.var_chi_dn9))))), ((locals.var_cnst1over_dn10 * assign74200_body41_e113486) + (locals.var_cnst1over * (locals.var_exp_bps0_dn10 - ((locals.var_exp_bvbs_dn10 * assign74200_body41_e113484) + (locals.var_exp_bvbs * locals.var_chi_dn10))))), ((locals.var_cnst1over_dn13 * assign74200_body41_e113486) + (locals.var_cnst1over * (locals.var_exp_bps0_dn13 - ((locals.var_exp_bvbs_dn13 * assign74200_body41_e113484) + (locals.var_exp_bvbs * locals.var_chi_dn13))))),)
    } else {
        (locals.var_fs01, locals.var_fs01_dn0, locals.var_fs01_dn2, locals.var_fs01_dn4, locals.var_fs01_dn5, locals.var_fs01_dn6, locals.var_fs01_dn7, locals.var_fs01_dn8, locals.var_fs01_dn9, locals.var_fs01_dn10, locals.var_fs01_dn13,)
    }
};
            locals.var_fs01 = assign74200_body41_e113489;
            locals.var_fs01_dn0 = assign74200_body41_e113489_d_n0;
            locals.var_fs01_dn2 = assign74200_body41_e113489_d_n2;
            locals.var_fs01_dn4 = assign74200_body41_e113489_d_n4;
            locals.var_fs01_dn5 = assign74200_body41_e113489_d_n5;
            locals.var_fs01_dn6 = assign74200_body41_e113489_d_n6;
            locals.var_fs01_dn7 = assign74200_body41_e113489_d_n7;
            locals.var_fs01_dn8 = assign74200_body41_e113489_d_n8;
            locals.var_fs01_dn9 = assign74200_body41_e113489_d_n9;
            locals.var_fs01_dn10 = assign74200_body41_e113489_d_n10;
            locals.var_fs01_dn13 = assign74200_body41_e113489_d_n13;
            locals.var_fs01_rv = 0.0;
            let (assign74200_body42_e113507, assign74200_body42_e113507_d_n0, assign74200_body42_e113507_d_n2, assign74200_body42_e113507_d_n4, assign74200_body42_e113507_d_n5, assign74200_body42_e113507_d_n6, assign74200_body42_e113507_d_n7, assign74200_body42_e113507_d_n8, assign74200_body42_e113507_d_n9, assign74200_body42_e113507_d_n10, assign74200_body42_e113507_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1722 != 0.0)) && (locals.var_guard1729 == 0.0)) && (locals.var_guard1730 == 0.0)) {
        let assign74200_body42_e113501: f64 = (locals.var_cnst1over * locals.var_beta);
        let assign74200_body42_e113504: f64 = (locals.var_exp_bps0 - locals.var_exp_bvbs);
        let assign74200_body42_e113505: f64 = (assign74200_body42_e113501 * assign74200_body42_e113504);
        (assign74200_body42_e113505, ((((locals.var_cnst1over_dn0 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn0)) * assign74200_body42_e113504) + (assign74200_body42_e113501 * (locals.var_exp_bps0_dn0 - locals.var_exp_bvbs_dn0))), ((((locals.var_cnst1over_dn2 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn2)) * assign74200_body42_e113504) + (assign74200_body42_e113501 * (locals.var_exp_bps0_dn2 - locals.var_exp_bvbs_dn2))), ((((locals.var_cnst1over_dn4 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn4)) * assign74200_body42_e113504) + (assign74200_body42_e113501 * (locals.var_exp_bps0_dn4 - locals.var_exp_bvbs_dn4))), ((((locals.var_cnst1over_dn5 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn5)) * assign74200_body42_e113504) + (assign74200_body42_e113501 * (locals.var_exp_bps0_dn5 - locals.var_exp_bvbs_dn5))), ((((locals.var_cnst1over_dn6 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn6)) * assign74200_body42_e113504) + (assign74200_body42_e113501 * (locals.var_exp_bps0_dn6 - locals.var_exp_bvbs_dn6))), ((((locals.var_cnst1over_dn7 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn7)) * assign74200_body42_e113504) + (assign74200_body42_e113501 * (locals.var_exp_bps0_dn7 - locals.var_exp_bvbs_dn7))), ((((locals.var_cnst1over_dn8 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn8)) * assign74200_body42_e113504) + (assign74200_body42_e113501 * (locals.var_exp_bps0_dn8 - locals.var_exp_bvbs_dn8))), ((((locals.var_cnst1over_dn9 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn9)) * assign74200_body42_e113504) + (assign74200_body42_e113501 * (locals.var_exp_bps0_dn9 - locals.var_exp_bvbs_dn9))), ((((locals.var_cnst1over_dn10 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn10)) * assign74200_body42_e113504) + (assign74200_body42_e113501 * (locals.var_exp_bps0_dn10 - locals.var_exp_bvbs_dn10))), ((((locals.var_cnst1over_dn13 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn13)) * assign74200_body42_e113504) + (assign74200_body42_e113501 * (locals.var_exp_bps0_dn13 - locals.var_exp_bvbs_dn13))),)
    } else {
        (locals.var_fs01_dps0, locals.var_fs01_dps0_dn0, locals.var_fs01_dps0_dn2, locals.var_fs01_dps0_dn4, locals.var_fs01_dps0_dn5, locals.var_fs01_dps0_dn6, locals.var_fs01_dps0_dn7, locals.var_fs01_dps0_dn8, locals.var_fs01_dps0_dn9, locals.var_fs01_dps0_dn10, locals.var_fs01_dps0_dn13,)
    }
};
            locals.var_fs01_dps0 = assign74200_body42_e113507;
            locals.var_fs01_dps0_dn0 = assign74200_body42_e113507_d_n0;
            locals.var_fs01_dps0_dn2 = assign74200_body42_e113507_d_n2;
            locals.var_fs01_dps0_dn4 = assign74200_body42_e113507_d_n4;
            locals.var_fs01_dps0_dn5 = assign74200_body42_e113507_d_n5;
            locals.var_fs01_dps0_dn6 = assign74200_body42_e113507_d_n6;
            locals.var_fs01_dps0_dn7 = assign74200_body42_e113507_d_n7;
            locals.var_fs01_dps0_dn8 = assign74200_body42_e113507_d_n8;
            locals.var_fs01_dps0_dn9 = assign74200_body42_e113507_d_n9;
            locals.var_fs01_dps0_dn10 = assign74200_body42_e113507_d_n10;
            locals.var_fs01_dps0_dn13 = assign74200_body42_e113507_d_n13;
            locals.var_fs01_dps0_rv = 0.0;
            let assign74200_body43_e113510: f64 = if locals.var_fs01 > 0.0 { 1.0 } else { 0.0 };
            locals.var_guard1731 = assign74200_body43_e113510;
            locals.var_guard1731_rv = 0.0;
            let (assign74200_body44_e113521, assign74200_body44_e113521_d_n0, assign74200_body44_e113521_d_n2, assign74200_body44_e113521_d_n4, assign74200_body44_e113521_d_n5, assign74200_body44_e113521_d_n6, assign74200_body44_e113521_d_n7, assign74200_body44_e113521_d_n8, assign74200_body44_e113521_d_n9, assign74200_body44_e113521_d_n10, assign74200_body44_e113521_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1722 != 0.0)) && (locals.var_guard1731 != 0.0)) {
        let assign74200_body44_e113518: f64 = (locals.var_fbsq + locals.var_fs01);
        let assign74200_body44_e113519: f64 = (assign74200_body44_e113518).sqrt();
        (assign74200_body44_e113519, ((locals.var_fbsq_dn0 + locals.var_fs01_dn0) / (2.0 * assign74200_body44_e113519)), ((locals.var_fbsq_dn2 + locals.var_fs01_dn2) / (2.0 * assign74200_body44_e113519)), ((locals.var_fbsq_dn4 + locals.var_fs01_dn4) / (2.0 * assign74200_body44_e113519)), ((locals.var_fbsq_dn5 + locals.var_fs01_dn5) / (2.0 * assign74200_body44_e113519)), ((locals.var_fbsq_dn6 + locals.var_fs01_dn6) / (2.0 * assign74200_body44_e113519)), ((locals.var_fbsq_dn7 + locals.var_fs01_dn7) / (2.0 * assign74200_body44_e113519)), ((locals.var_fbsq_dn8 + locals.var_fs01_dn8) / (2.0 * assign74200_body44_e113519)), ((locals.var_fbsq_dn9 + locals.var_fs01_dn9) / (2.0 * assign74200_body44_e113519)), ((locals.var_fbsq_dn10 + locals.var_fs01_dn10) / (2.0 * assign74200_body44_e113519)), ((locals.var_fbsq_dn13 + locals.var_fs01_dn13) / (2.0 * assign74200_body44_e113519)),)
    } else {
        (locals.var_fs02, locals.var_fs02_dn0, locals.var_fs02_dn2, locals.var_fs02_dn4, locals.var_fs02_dn5, locals.var_fs02_dn6, locals.var_fs02_dn7, locals.var_fs02_dn8, locals.var_fs02_dn9, locals.var_fs02_dn10, locals.var_fs02_dn13,)
    }
};
            locals.var_fs02 = assign74200_body44_e113521;
            locals.var_fs02_dn0 = assign74200_body44_e113521_d_n0;
            locals.var_fs02_dn2 = assign74200_body44_e113521_d_n2;
            locals.var_fs02_dn4 = assign74200_body44_e113521_d_n4;
            locals.var_fs02_dn5 = assign74200_body44_e113521_d_n5;
            locals.var_fs02_dn6 = assign74200_body44_e113521_d_n6;
            locals.var_fs02_dn7 = assign74200_body44_e113521_d_n7;
            locals.var_fs02_dn8 = assign74200_body44_e113521_d_n8;
            locals.var_fs02_dn9 = assign74200_body44_e113521_d_n9;
            locals.var_fs02_dn10 = assign74200_body44_e113521_d_n10;
            locals.var_fs02_dn13 = assign74200_body44_e113521_d_n13;
            locals.var_fs02_rv = 0.0;
            let (assign74200_body45_e113535, assign74200_body45_e113535_d_n0, assign74200_body45_e113535_d_n2, assign74200_body45_e113535_d_n4, assign74200_body45_e113535_d_n5, assign74200_body45_e113535_d_n6, assign74200_body45_e113535_d_n7, assign74200_body45_e113535_d_n8, assign74200_body45_e113535_d_n9, assign74200_body45_e113535_d_n10, assign74200_body45_e113535_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1722 != 0.0)) && (locals.var_guard1731 != 0.0)) {
        let assign74200_body45_e113530: f64 = (locals.var_fbsq_dpss + locals.var_fs01_dps0);
        let assign74200_body45_e113531: f64 = (0.5 * assign74200_body45_e113530);
        let assign74200_body45_e113533: f64 = (assign74200_body45_e113531 / locals.var_fs02);
        (assign74200_body45_e113533, ((((0.5 * (locals.var_fbsq_dpss_dn0 + locals.var_fs01_dps0_dn0)) * locals.var_fs02) - (assign74200_body45_e113531 * locals.var_fs02_dn0)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * (locals.var_fbsq_dpss_dn2 + locals.var_fs01_dps0_dn2)) * locals.var_fs02) - (assign74200_body45_e113531 * locals.var_fs02_dn2)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * (locals.var_fbsq_dpss_dn4 + locals.var_fs01_dps0_dn4)) * locals.var_fs02) - (assign74200_body45_e113531 * locals.var_fs02_dn4)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * (locals.var_fbsq_dpss_dn5 + locals.var_fs01_dps0_dn5)) * locals.var_fs02) - (assign74200_body45_e113531 * locals.var_fs02_dn5)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * (locals.var_fbsq_dpss_dn6 + locals.var_fs01_dps0_dn6)) * locals.var_fs02) - (assign74200_body45_e113531 * locals.var_fs02_dn6)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * (locals.var_fbsq_dpss_dn7 + locals.var_fs01_dps0_dn7)) * locals.var_fs02) - (assign74200_body45_e113531 * locals.var_fs02_dn7)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * (locals.var_fbsq_dpss_dn8 + locals.var_fs01_dps0_dn8)) * locals.var_fs02) - (assign74200_body45_e113531 * locals.var_fs02_dn8)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * (locals.var_fbsq_dpss_dn9 + locals.var_fs01_dps0_dn9)) * locals.var_fs02) - (assign74200_body45_e113531 * locals.var_fs02_dn9)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * (locals.var_fbsq_dpss_dn10 + locals.var_fs01_dps0_dn10)) * locals.var_fs02) - (assign74200_body45_e113531 * locals.var_fs02_dn10)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * (locals.var_fbsq_dpss_dn13 + locals.var_fs01_dps0_dn13)) * locals.var_fs02) - (assign74200_body45_e113531 * locals.var_fs02_dn13)) / (locals.var_fs02 * locals.var_fs02)),)
    } else {
        (locals.var_fs02_dps0, locals.var_fs02_dps0_dn0, locals.var_fs02_dps0_dn2, locals.var_fs02_dps0_dn4, locals.var_fs02_dps0_dn5, locals.var_fs02_dps0_dn6, locals.var_fs02_dps0_dn7, locals.var_fs02_dps0_dn8, locals.var_fs02_dps0_dn9, locals.var_fs02_dps0_dn10, locals.var_fs02_dps0_dn13,)
    }
};
            locals.var_fs02_dps0 = assign74200_body45_e113535;
            locals.var_fs02_dps0_dn0 = assign74200_body45_e113535_d_n0;
            locals.var_fs02_dps0_dn2 = assign74200_body45_e113535_d_n2;
            locals.var_fs02_dps0_dn4 = assign74200_body45_e113535_d_n4;
            locals.var_fs02_dps0_dn5 = assign74200_body45_e113535_d_n5;
            locals.var_fs02_dps0_dn6 = assign74200_body45_e113535_d_n6;
            locals.var_fs02_dps0_dn7 = assign74200_body45_e113535_d_n7;
            locals.var_fs02_dps0_dn8 = assign74200_body45_e113535_d_n8;
            locals.var_fs02_dps0_dn9 = assign74200_body45_e113535_d_n9;
            locals.var_fs02_dps0_dn10 = assign74200_body45_e113535_d_n10;
            locals.var_fs02_dps0_dn13 = assign74200_body45_e113535_d_n13;
            locals.var_fs02_dps0_rv = 0.0;
            let assign74200_body46_e113538: f64 = if locals.var_fbsq > 0.0 { 1.0 } else { 0.0 };
            locals.var_guard1732 = assign74200_body46_e113538;
            locals.var_guard1732_rv = 0.0;
            let (assign74200_body47_e113550, assign74200_body47_e113550_d_n0, assign74200_body47_e113550_d_n2, assign74200_body47_e113550_d_n4, assign74200_body47_e113550_d_n5, assign74200_body47_e113550_d_n6, assign74200_body47_e113550_d_n7, assign74200_body47_e113550_d_n8, assign74200_body47_e113550_d_n9, assign74200_body47_e113550_d_n10, assign74200_body47_e113550_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1722 != 0.0)) && (locals.var_guard1731 == 0.0)) && (locals.var_guard1732 != 0.0)) {
        let assign74200_body47_e113548: f64 = (locals.var_fbsq).sqrt();
        (assign74200_body47_e113548, (locals.var_fbsq_dn0 / (2.0 * assign74200_body47_e113548)), (locals.var_fbsq_dn2 / (2.0 * assign74200_body47_e113548)), (locals.var_fbsq_dn4 / (2.0 * assign74200_body47_e113548)), (locals.var_fbsq_dn5 / (2.0 * assign74200_body47_e113548)), (locals.var_fbsq_dn6 / (2.0 * assign74200_body47_e113548)), (locals.var_fbsq_dn7 / (2.0 * assign74200_body47_e113548)), (locals.var_fbsq_dn8 / (2.0 * assign74200_body47_e113548)), (locals.var_fbsq_dn9 / (2.0 * assign74200_body47_e113548)), (locals.var_fbsq_dn10 / (2.0 * assign74200_body47_e113548)), (locals.var_fbsq_dn13 / (2.0 * assign74200_body47_e113548)),)
    } else {
        (locals.var_fs02, locals.var_fs02_dn0, locals.var_fs02_dn2, locals.var_fs02_dn4, locals.var_fs02_dn5, locals.var_fs02_dn6, locals.var_fs02_dn7, locals.var_fs02_dn8, locals.var_fs02_dn9, locals.var_fs02_dn10, locals.var_fs02_dn13,)
    }
};
            locals.var_fs02 = assign74200_body47_e113550;
            locals.var_fs02_dn0 = assign74200_body47_e113550_d_n0;
            locals.var_fs02_dn2 = assign74200_body47_e113550_d_n2;
            locals.var_fs02_dn4 = assign74200_body47_e113550_d_n4;
            locals.var_fs02_dn5 = assign74200_body47_e113550_d_n5;
            locals.var_fs02_dn6 = assign74200_body47_e113550_d_n6;
            locals.var_fs02_dn7 = assign74200_body47_e113550_d_n7;
            locals.var_fs02_dn8 = assign74200_body47_e113550_d_n8;
            locals.var_fs02_dn9 = assign74200_body47_e113550_d_n9;
            locals.var_fs02_dn10 = assign74200_body47_e113550_d_n10;
            locals.var_fs02_dn13 = assign74200_body47_e113550_d_n13;
            locals.var_fs02_rv = 0.0;
            let (assign74200_body48_e113565, assign74200_body48_e113565_d_n0, assign74200_body48_e113565_d_n2, assign74200_body48_e113565_d_n4, assign74200_body48_e113565_d_n5, assign74200_body48_e113565_d_n6, assign74200_body48_e113565_d_n7, assign74200_body48_e113565_d_n8, assign74200_body48_e113565_d_n9, assign74200_body48_e113565_d_n10, assign74200_body48_e113565_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1722 != 0.0)) && (locals.var_guard1731 == 0.0)) && (locals.var_guard1732 != 0.0)) {
        let assign74200_body48_e113561: f64 = (0.5 * locals.var_fbsq_dpss);
        let assign74200_body48_e113563: f64 = (assign74200_body48_e113561 / locals.var_fs02);
        (assign74200_body48_e113563, ((((0.5 * locals.var_fbsq_dpss_dn0) * locals.var_fs02) - (assign74200_body48_e113561 * locals.var_fs02_dn0)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * locals.var_fbsq_dpss_dn2) * locals.var_fs02) - (assign74200_body48_e113561 * locals.var_fs02_dn2)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * locals.var_fbsq_dpss_dn4) * locals.var_fs02) - (assign74200_body48_e113561 * locals.var_fs02_dn4)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * locals.var_fbsq_dpss_dn5) * locals.var_fs02) - (assign74200_body48_e113561 * locals.var_fs02_dn5)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * locals.var_fbsq_dpss_dn6) * locals.var_fs02) - (assign74200_body48_e113561 * locals.var_fs02_dn6)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * locals.var_fbsq_dpss_dn7) * locals.var_fs02) - (assign74200_body48_e113561 * locals.var_fs02_dn7)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * locals.var_fbsq_dpss_dn8) * locals.var_fs02) - (assign74200_body48_e113561 * locals.var_fs02_dn8)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * locals.var_fbsq_dpss_dn9) * locals.var_fs02) - (assign74200_body48_e113561 * locals.var_fs02_dn9)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * locals.var_fbsq_dpss_dn10) * locals.var_fs02) - (assign74200_body48_e113561 * locals.var_fs02_dn10)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * locals.var_fbsq_dpss_dn13) * locals.var_fs02) - (assign74200_body48_e113561 * locals.var_fs02_dn13)) / (locals.var_fs02 * locals.var_fs02)),)
    } else {
        (locals.var_fs02_dps0, locals.var_fs02_dps0_dn0, locals.var_fs02_dps0_dn2, locals.var_fs02_dps0_dn4, locals.var_fs02_dps0_dn5, locals.var_fs02_dps0_dn6, locals.var_fs02_dps0_dn7, locals.var_fs02_dps0_dn8, locals.var_fs02_dps0_dn9, locals.var_fs02_dps0_dn10, locals.var_fs02_dps0_dn13,)
    }
};
            locals.var_fs02_dps0 = assign74200_body48_e113565;
            locals.var_fs02_dps0_dn0 = assign74200_body48_e113565_d_n0;
            locals.var_fs02_dps0_dn2 = assign74200_body48_e113565_d_n2;
            locals.var_fs02_dps0_dn4 = assign74200_body48_e113565_d_n4;
            locals.var_fs02_dps0_dn5 = assign74200_body48_e113565_d_n5;
            locals.var_fs02_dps0_dn6 = assign74200_body48_e113565_d_n6;
            locals.var_fs02_dps0_dn7 = assign74200_body48_e113565_d_n7;
            locals.var_fs02_dps0_dn8 = assign74200_body48_e113565_d_n8;
            locals.var_fs02_dps0_dn9 = assign74200_body48_e113565_d_n9;
            locals.var_fs02_dps0_dn10 = assign74200_body48_e113565_d_n10;
            locals.var_fs02_dps0_dn13 = assign74200_body48_e113565_d_n13;
            locals.var_fs02_dps0_rv = 0.0;
            let (assign74200_body49_e113577, assign74200_body49_e113577_d_n0, assign74200_body49_e113577_d_n2, assign74200_body49_e113577_d_n4, assign74200_body49_e113577_d_n5, assign74200_body49_e113577_d_n6, assign74200_body49_e113577_d_n7, assign74200_body49_e113577_d_n8, assign74200_body49_e113577_d_n9, assign74200_body49_e113577_d_n10, assign74200_body49_e113577_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1722 != 0.0)) && (locals.var_guard1731 == 0.0)) && (locals.var_guard1732 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fs02, locals.var_fs02_dn0, locals.var_fs02_dn2, locals.var_fs02_dn4, locals.var_fs02_dn5, locals.var_fs02_dn6, locals.var_fs02_dn7, locals.var_fs02_dn8, locals.var_fs02_dn9, locals.var_fs02_dn10, locals.var_fs02_dn13,)
    }
};
            locals.var_fs02 = assign74200_body49_e113577;
            locals.var_fs02_dn0 = assign74200_body49_e113577_d_n0;
            locals.var_fs02_dn2 = assign74200_body49_e113577_d_n2;
            locals.var_fs02_dn4 = assign74200_body49_e113577_d_n4;
            locals.var_fs02_dn5 = assign74200_body49_e113577_d_n5;
            locals.var_fs02_dn6 = assign74200_body49_e113577_d_n6;
            locals.var_fs02_dn7 = assign74200_body49_e113577_d_n7;
            locals.var_fs02_dn8 = assign74200_body49_e113577_d_n8;
            locals.var_fs02_dn9 = assign74200_body49_e113577_d_n9;
            locals.var_fs02_dn10 = assign74200_body49_e113577_d_n10;
            locals.var_fs02_dn13 = assign74200_body49_e113577_d_n13;
            locals.var_fs02_rv = 0.0;
            let (assign74200_body50_e113589, assign74200_body50_e113589_d_n0, assign74200_body50_e113589_d_n2, assign74200_body50_e113589_d_n4, assign74200_body50_e113589_d_n5, assign74200_body50_e113589_d_n6, assign74200_body50_e113589_d_n7, assign74200_body50_e113589_d_n8, assign74200_body50_e113589_d_n9, assign74200_body50_e113589_d_n10, assign74200_body50_e113589_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1722 != 0.0)) && (locals.var_guard1731 == 0.0)) && (locals.var_guard1732 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fs02_dps0, locals.var_fs02_dps0_dn0, locals.var_fs02_dps0_dn2, locals.var_fs02_dps0_dn4, locals.var_fs02_dps0_dn5, locals.var_fs02_dps0_dn6, locals.var_fs02_dps0_dn7, locals.var_fs02_dps0_dn8, locals.var_fs02_dps0_dn9, locals.var_fs02_dps0_dn10, locals.var_fs02_dps0_dn13,)
    }
};
            locals.var_fs02_dps0 = assign74200_body50_e113589;
            locals.var_fs02_dps0_dn0 = assign74200_body50_e113589_d_n0;
            locals.var_fs02_dps0_dn2 = assign74200_body50_e113589_d_n2;
            locals.var_fs02_dps0_dn4 = assign74200_body50_e113589_d_n4;
            locals.var_fs02_dps0_dn5 = assign74200_body50_e113589_d_n5;
            locals.var_fs02_dps0_dn6 = assign74200_body50_e113589_d_n6;
            locals.var_fs02_dps0_dn7 = assign74200_body50_e113589_d_n7;
            locals.var_fs02_dps0_dn8 = assign74200_body50_e113589_d_n8;
            locals.var_fs02_dps0_dn9 = assign74200_body50_e113589_d_n9;
            locals.var_fs02_dps0_dn10 = assign74200_body50_e113589_d_n10;
            locals.var_fs02_dps0_dn13 = assign74200_body50_e113589_d_n13;
            locals.var_fs02_dps0_rv = 0.0;
            let (assign74200_body51_e113603, assign74200_body51_e113603_d_n0, assign74200_body51_e113603_d_n2, assign74200_body51_e113603_d_n4, assign74200_body51_e113603_d_n5, assign74200_body51_e113603_d_n6, assign74200_body51_e113603_d_n7, assign74200_body51_e113603_d_n8, assign74200_body51_e113603_d_n9, assign74200_body51_e113603_d_n10, assign74200_body51_e113603_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1722 != 0.0)) {
        let (assign74200_body51_e113599,) = {
            if (locals.var_chi >= 0.0) {
                (1.0,)
            } else {
                let assign74200_body51_e113598: f64 = (-1.0);
                (assign74200_body51_e113598,)
            }
        };
        let assign74200_body51_e113601: f64 = (assign74200_body51_e113599 * locals.var_fs02);
        (assign74200_body51_e113601, (assign74200_body51_e113599 * locals.var_fs02_dn0), (assign74200_body51_e113599 * locals.var_fs02_dn2), (assign74200_body51_e113599 * locals.var_fs02_dn4), (assign74200_body51_e113599 * locals.var_fs02_dn5), (assign74200_body51_e113599 * locals.var_fs02_dn6), (assign74200_body51_e113599 * locals.var_fs02_dn7), (assign74200_body51_e113599 * locals.var_fs02_dn8), (assign74200_body51_e113599 * locals.var_fs02_dn9), (assign74200_body51_e113599 * locals.var_fs02_dn10), (assign74200_body51_e113599 * locals.var_fs02_dn13),)
    } else {
        (locals.var_fs02, locals.var_fs02_dn0, locals.var_fs02_dn2, locals.var_fs02_dn4, locals.var_fs02_dn5, locals.var_fs02_dn6, locals.var_fs02_dn7, locals.var_fs02_dn8, locals.var_fs02_dn9, locals.var_fs02_dn10, locals.var_fs02_dn13,)
    }
};
            locals.var_fs02 = assign74200_body51_e113603;
            locals.var_fs02_dn0 = assign74200_body51_e113603_d_n0;
            locals.var_fs02_dn2 = assign74200_body51_e113603_d_n2;
            locals.var_fs02_dn4 = assign74200_body51_e113603_d_n4;
            locals.var_fs02_dn5 = assign74200_body51_e113603_d_n5;
            locals.var_fs02_dn6 = assign74200_body51_e113603_d_n6;
            locals.var_fs02_dn7 = assign74200_body51_e113603_d_n7;
            locals.var_fs02_dn8 = assign74200_body51_e113603_d_n8;
            locals.var_fs02_dn9 = assign74200_body51_e113603_d_n9;
            locals.var_fs02_dn10 = assign74200_body51_e113603_d_n10;
            locals.var_fs02_dn13 = assign74200_body51_e113603_d_n13;
            locals.var_fs02_rv = 0.0;
            let (assign74200_body52_e113617, assign74200_body52_e113617_d_n0, assign74200_body52_e113617_d_n2, assign74200_body52_e113617_d_n4, assign74200_body52_e113617_d_n5, assign74200_body52_e113617_d_n6, assign74200_body52_e113617_d_n7, assign74200_body52_e113617_d_n8, assign74200_body52_e113617_d_n9, assign74200_body52_e113617_d_n10, assign74200_body52_e113617_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1722 != 0.0)) {
        let (assign74200_body52_e113613,) = {
            if (locals.var_chi >= 0.0) {
                (1.0,)
            } else {
                let assign74200_body52_e113612: f64 = (-1.0);
                (assign74200_body52_e113612,)
            }
        };
        let assign74200_body52_e113615: f64 = (assign74200_body52_e113613 * locals.var_fs02_dps0);
        (assign74200_body52_e113615, (assign74200_body52_e113613 * locals.var_fs02_dps0_dn0), (assign74200_body52_e113613 * locals.var_fs02_dps0_dn2), (assign74200_body52_e113613 * locals.var_fs02_dps0_dn4), (assign74200_body52_e113613 * locals.var_fs02_dps0_dn5), (assign74200_body52_e113613 * locals.var_fs02_dps0_dn6), (assign74200_body52_e113613 * locals.var_fs02_dps0_dn7), (assign74200_body52_e113613 * locals.var_fs02_dps0_dn8), (assign74200_body52_e113613 * locals.var_fs02_dps0_dn9), (assign74200_body52_e113613 * locals.var_fs02_dps0_dn10), (assign74200_body52_e113613 * locals.var_fs02_dps0_dn13),)
    } else {
        (locals.var_fs02_dps0, locals.var_fs02_dps0_dn0, locals.var_fs02_dps0_dn2, locals.var_fs02_dps0_dn4, locals.var_fs02_dps0_dn5, locals.var_fs02_dps0_dn6, locals.var_fs02_dps0_dn7, locals.var_fs02_dps0_dn8, locals.var_fs02_dps0_dn9, locals.var_fs02_dps0_dn10, locals.var_fs02_dps0_dn13,)
    }
};
            locals.var_fs02_dps0 = assign74200_body52_e113617;
            locals.var_fs02_dps0_dn0 = assign74200_body52_e113617_d_n0;
            locals.var_fs02_dps0_dn2 = assign74200_body52_e113617_d_n2;
            locals.var_fs02_dps0_dn4 = assign74200_body52_e113617_d_n4;
            locals.var_fs02_dps0_dn5 = assign74200_body52_e113617_d_n5;
            locals.var_fs02_dps0_dn6 = assign74200_body52_e113617_d_n6;
            locals.var_fs02_dps0_dn7 = assign74200_body52_e113617_d_n7;
            locals.var_fs02_dps0_dn8 = assign74200_body52_e113617_d_n8;
            locals.var_fs02_dps0_dn9 = assign74200_body52_e113617_d_n9;
            locals.var_fs02_dps0_dn10 = assign74200_body52_e113617_d_n10;
            locals.var_fs02_dps0_dn13 = assign74200_body52_e113617_d_n13;
            locals.var_fs02_dps0_rv = 0.0;
            let (assign74200_body53_e113630, assign74200_body53_e113630_d_n0, assign74200_body53_e113630_d_n2, assign74200_body53_e113630_d_n4, assign74200_body53_e113630_d_n5, assign74200_body53_e113630_d_n6, assign74200_body53_e113630_d_n7, assign74200_body53_e113630_d_n8, assign74200_body53_e113630_d_n9, assign74200_body53_e113630_d_n10, assign74200_body53_e113630_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1722 != 0.0)) {
        let assign74200_body53_e113622: f64 = (-locals.var_vgpld);
        let assign74200_body53_e113624: f64 = (assign74200_body53_e113622 + locals.var_ps0ld);
        let assign74200_body53_e113627: f64 = (locals.var_fac1 * locals.var_fs02);
        let assign74200_body53_e113628: f64 = (assign74200_body53_e113624 + assign74200_body53_e113627);
        (assign74200_body53_e113628, (locals.var_ps0ld_dn0 + ((locals.var_fac1_dn0 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn0))), (((-locals.var_vgpld_dn2) + locals.var_ps0ld_dn2) + ((locals.var_fac1_dn2 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn2))), (locals.var_ps0ld_dn4 + ((locals.var_fac1_dn4 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn4))), (locals.var_ps0ld_dn5 + ((locals.var_fac1_dn5 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn5))), (((-locals.var_vgpld_dn6) + locals.var_ps0ld_dn6) + ((locals.var_fac1_dn6 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn6))), (((-locals.var_vgpld_dn7) + locals.var_ps0ld_dn7) + ((locals.var_fac1_dn7 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn7))), (((-locals.var_vgpld_dn8) + locals.var_ps0ld_dn8) + ((locals.var_fac1_dn8 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn8))), (locals.var_ps0ld_dn9 + ((locals.var_fac1_dn9 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn9))), (locals.var_ps0ld_dn10 + ((locals.var_fac1_dn10 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn10))), (locals.var_ps0ld_dn13 + ((locals.var_fac1_dn13 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn13))),)
    } else {
        (locals.var_fs0, locals.var_fs0_dn0, locals.var_fs0_dn2, locals.var_fs0_dn4, locals.var_fs0_dn5, locals.var_fs0_dn6, locals.var_fs0_dn7, locals.var_fs0_dn8, locals.var_fs0_dn9, locals.var_fs0_dn10, locals.var_fs0_dn13,)
    }
};
            locals.var_fs0 = assign74200_body53_e113630;
            locals.var_fs0_dn0 = assign74200_body53_e113630_d_n0;
            locals.var_fs0_dn2 = assign74200_body53_e113630_d_n2;
            locals.var_fs0_dn4 = assign74200_body53_e113630_d_n4;
            locals.var_fs0_dn5 = assign74200_body53_e113630_d_n5;
            locals.var_fs0_dn6 = assign74200_body53_e113630_d_n6;
            locals.var_fs0_dn7 = assign74200_body53_e113630_d_n7;
            locals.var_fs0_dn8 = assign74200_body53_e113630_d_n8;
            locals.var_fs0_dn9 = assign74200_body53_e113630_d_n9;
            locals.var_fs0_dn10 = assign74200_body53_e113630_d_n10;
            locals.var_fs0_dn13 = assign74200_body53_e113630_d_n13;
            locals.var_fs0_rv = 0.0;
            let (assign74200_body54_e113640, assign74200_body54_e113640_d_n0, assign74200_body54_e113640_d_n2, assign74200_body54_e113640_d_n4, assign74200_body54_e113640_d_n5, assign74200_body54_e113640_d_n6, assign74200_body54_e113640_d_n7, assign74200_body54_e113640_d_n8, assign74200_body54_e113640_d_n9, assign74200_body54_e113640_d_n10, assign74200_body54_e113640_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1722 != 0.0)) {
        let assign74200_body54_e113637: f64 = (locals.var_fac1 * locals.var_fs02_dps0);
        let assign74200_body54_e113638: f64 = (1.0 + assign74200_body54_e113637);
        (assign74200_body54_e113638, ((locals.var_fac1_dn0 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn0)), ((locals.var_fac1_dn2 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn2)), ((locals.var_fac1_dn4 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn4)), ((locals.var_fac1_dn5 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn5)), ((locals.var_fac1_dn6 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn6)), ((locals.var_fac1_dn7 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn7)), ((locals.var_fac1_dn8 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn8)), ((locals.var_fac1_dn9 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn9)), ((locals.var_fac1_dn10 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn10)), ((locals.var_fac1_dn13 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn13)),)
    } else {
        (locals.var_fs0_dps0, locals.var_fs0_dps0_dn0, locals.var_fs0_dps0_dn2, locals.var_fs0_dps0_dn4, locals.var_fs0_dps0_dn5, locals.var_fs0_dps0_dn6, locals.var_fs0_dps0_dn7, locals.var_fs0_dps0_dn8, locals.var_fs0_dps0_dn9, locals.var_fs0_dps0_dn10, locals.var_fs0_dps0_dn13,)
    }
};
            locals.var_fs0_dps0 = assign74200_body54_e113640;
            locals.var_fs0_dps0_dn0 = assign74200_body54_e113640_d_n0;
            locals.var_fs0_dps0_dn2 = assign74200_body54_e113640_d_n2;
            locals.var_fs0_dps0_dn4 = assign74200_body54_e113640_d_n4;
            locals.var_fs0_dps0_dn5 = assign74200_body54_e113640_d_n5;
            locals.var_fs0_dps0_dn6 = assign74200_body54_e113640_d_n6;
            locals.var_fs0_dps0_dn7 = assign74200_body54_e113640_d_n7;
            locals.var_fs0_dps0_dn8 = assign74200_body54_e113640_d_n8;
            locals.var_fs0_dps0_dn9 = assign74200_body54_e113640_d_n9;
            locals.var_fs0_dps0_dn10 = assign74200_body54_e113640_d_n10;
            locals.var_fs0_dps0_dn13 = assign74200_body54_e113640_d_n13;
            locals.var_fs0_dps0_rv = 0.0;
            let assign74200_body55_e113643: f64 = if locals.var_flg_conv > 0.0 { 1.0 } else { 0.0 };
            locals.var_guard1733 = assign74200_body55_e113643;
            locals.var_guard1733_rv = 0.0;
            let (assign74200_body56_e113653,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1722 != 0.0)) && (locals.var_guard1733 != 0.0)) {
        let assign74200_body56_e113651: f64 = (locals.var_lp_s0_max + 1.0);
        (assign74200_body56_e113651,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign74200_body56_e113653;
            locals.var_lp_s0_rv = 0.0;
            let (assign74200_body57_e113665, assign74200_body57_e113665_d_n0, assign74200_body57_e113665_d_n2, assign74200_body57_e113665_d_n4, assign74200_body57_e113665_d_n5, assign74200_body57_e113665_d_n6, assign74200_body57_e113665_d_n7, assign74200_body57_e113665_d_n8, assign74200_body57_e113665_d_n9, assign74200_body57_e113665_d_n10, assign74200_body57_e113665_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1722 != 0.0)) && (locals.var_guard1733 == 0.0)) {
        let assign74200_body57_e113661: f64 = (-locals.var_fs0);
        let assign74200_body57_e113663: f64 = (assign74200_body57_e113661 / locals.var_fs0_dps0);
        (assign74200_body57_e113663, ((((-locals.var_fs0_dn0) * locals.var_fs0_dps0) - (assign74200_body57_e113661 * locals.var_fs0_dps0_dn0)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn2) * locals.var_fs0_dps0) - (assign74200_body57_e113661 * locals.var_fs0_dps0_dn2)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn4) * locals.var_fs0_dps0) - (assign74200_body57_e113661 * locals.var_fs0_dps0_dn4)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn5) * locals.var_fs0_dps0) - (assign74200_body57_e113661 * locals.var_fs0_dps0_dn5)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn6) * locals.var_fs0_dps0) - (assign74200_body57_e113661 * locals.var_fs0_dps0_dn6)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn7) * locals.var_fs0_dps0) - (assign74200_body57_e113661 * locals.var_fs0_dps0_dn7)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn8) * locals.var_fs0_dps0) - (assign74200_body57_e113661 * locals.var_fs0_dps0_dn8)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn9) * locals.var_fs0_dps0) - (assign74200_body57_e113661 * locals.var_fs0_dps0_dn9)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn10) * locals.var_fs0_dps0) - (assign74200_body57_e113661 * locals.var_fs0_dps0_dn10)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn13) * locals.var_fs0_dps0) - (assign74200_body57_e113661 * locals.var_fs0_dps0_dn13)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)),)
    } else {
        (locals.var_dps0, locals.var_dps0_dn0, locals.var_dps0_dn2, locals.var_dps0_dn4, locals.var_dps0_dn5, locals.var_dps0_dn6, locals.var_dps0_dn7, locals.var_dps0_dn8, locals.var_dps0_dn9, locals.var_dps0_dn10, locals.var_dps0_dn13,)
    }
};
            locals.var_dps0 = assign74200_body57_e113665;
            locals.var_dps0_dn0 = assign74200_body57_e113665_d_n0;
            locals.var_dps0_dn2 = assign74200_body57_e113665_d_n2;
            locals.var_dps0_dn4 = assign74200_body57_e113665_d_n4;
            locals.var_dps0_dn5 = assign74200_body57_e113665_d_n5;
            locals.var_dps0_dn6 = assign74200_body57_e113665_d_n6;
            locals.var_dps0_dn7 = assign74200_body57_e113665_d_n7;
            locals.var_dps0_dn8 = assign74200_body57_e113665_d_n8;
            locals.var_dps0_dn9 = assign74200_body57_e113665_d_n9;
            locals.var_dps0_dn10 = assign74200_body57_e113665_d_n10;
            locals.var_dps0_dn13 = assign74200_body57_e113665_d_n13;
            locals.var_dps0_rv = 0.0;
            let (assign74200_body58_e113687, assign74200_body58_e113687_d_n0, assign74200_body58_e113687_d_n2, assign74200_body58_e113687_d_n4, assign74200_body58_e113687_d_n5, assign74200_body58_e113687_d_n6, assign74200_body58_e113687_d_n7, assign74200_body58_e113687_d_n8, assign74200_body58_e113687_d_n9, assign74200_body58_e113687_d_n10, assign74200_body58_e113687_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1722 != 0.0)) && (locals.var_guard1733 == 0.0)) {
        let assign74200_body58_e113674: f64 = (0.5 * 0.1);
        let assign74200_body58_e113678: f64 = (locals.var_ps0ld).abs();
        let (assign74200_body58_e113683, assign74200_body58_e113683_d_n0, assign74200_body58_e113683_d_n2, assign74200_body58_e113683_d_n4, assign74200_body58_e113683_d_n5, assign74200_body58_e113683_d_n6, assign74200_body58_e113683_d_n7, assign74200_body58_e113683_d_n8, assign74200_body58_e113683_d_n9, assign74200_body58_e113683_d_n10, assign74200_body58_e113683_d_n13,) = {
            if (1.0 >= assign74200_body58_e113678) {
                (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign74200_body58_e113682: f64 = (locals.var_ps0ld).abs();
                (assign74200_body58_e113682, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn0 } else { (-locals.var_ps0ld_dn0) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn2 } else { (-locals.var_ps0ld_dn2) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn4 } else { (-locals.var_ps0ld_dn4) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn5 } else { (-locals.var_ps0ld_dn5) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn6 } else { (-locals.var_ps0ld_dn6) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn7 } else { (-locals.var_ps0ld_dn7) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn8 } else { (-locals.var_ps0ld_dn8) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn9 } else { (-locals.var_ps0ld_dn9) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn10 } else { (-locals.var_ps0ld_dn10) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn13 } else { (-locals.var_ps0ld_dn13) },)
            }
        };
        let assign74200_body58_e113684: f64 = (1.0 + assign74200_body58_e113683);
        let assign74200_body58_e113685: f64 = (assign74200_body58_e113674 * assign74200_body58_e113684);
        (assign74200_body58_e113685, (assign74200_body58_e113674 * assign74200_body58_e113683_d_n0), (assign74200_body58_e113674 * assign74200_body58_e113683_d_n2), (assign74200_body58_e113674 * assign74200_body58_e113683_d_n4), (assign74200_body58_e113674 * assign74200_body58_e113683_d_n5), (assign74200_body58_e113674 * assign74200_body58_e113683_d_n6), (assign74200_body58_e113674 * assign74200_body58_e113683_d_n7), (assign74200_body58_e113674 * assign74200_body58_e113683_d_n8), (assign74200_body58_e113674 * assign74200_body58_e113683_d_n9), (assign74200_body58_e113674 * assign74200_body58_e113683_d_n10), (assign74200_body58_e113674 * assign74200_body58_e113683_d_n13),)
    } else {
        (locals.var_dplim, locals.var_dplim_dn0, locals.var_dplim_dn2, locals.var_dplim_dn4, locals.var_dplim_dn5, locals.var_dplim_dn6, locals.var_dplim_dn7, locals.var_dplim_dn8, locals.var_dplim_dn9, locals.var_dplim_dn10, locals.var_dplim_dn13,)
    }
};
            locals.var_dplim = assign74200_body58_e113687;
            locals.var_dplim_dn0 = assign74200_body58_e113687_d_n0;
            locals.var_dplim_dn2 = assign74200_body58_e113687_d_n2;
            locals.var_dplim_dn4 = assign74200_body58_e113687_d_n4;
            locals.var_dplim_dn5 = assign74200_body58_e113687_d_n5;
            locals.var_dplim_dn6 = assign74200_body58_e113687_d_n6;
            locals.var_dplim_dn7 = assign74200_body58_e113687_d_n7;
            locals.var_dplim_dn8 = assign74200_body58_e113687_d_n8;
            locals.var_dplim_dn9 = assign74200_body58_e113687_d_n9;
            locals.var_dplim_dn10 = assign74200_body58_e113687_d_n10;
            locals.var_dplim_dn13 = assign74200_body58_e113687_d_n13;
            locals.var_dplim_rv = 0.0;
            let assign74200_body59_e113689: f64 = (locals.var_dps0).abs();
            let assign74200_body59_e113691: f64 = if assign74200_body59_e113689 > locals.var_dplim { 1.0 } else { 0.0 };
            locals.var_guard1734 = assign74200_body59_e113691;
            locals.var_guard1734_rv = 0.0;
            let (assign74200_body60_e113710, assign74200_body60_e113710_d_n0, assign74200_body60_e113710_d_n2, assign74200_body60_e113710_d_n4, assign74200_body60_e113710_d_n5, assign74200_body60_e113710_d_n6, assign74200_body60_e113710_d_n7, assign74200_body60_e113710_d_n8, assign74200_body60_e113710_d_n9, assign74200_body60_e113710_d_n10, assign74200_body60_e113710_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1722 != 0.0)) && (locals.var_guard1733 == 0.0)) && (locals.var_guard1734 != 0.0)) {
        let (assign74200_body60_e113707,) = {
            if (locals.var_dps0 >= 0.0) {
                (1.0,)
            } else {
                let assign74200_body60_e113706: f64 = (-1.0);
                (assign74200_body60_e113706,)
            }
        };
        let assign74200_body60_e113708: f64 = (locals.var_dplim * assign74200_body60_e113707);
        (assign74200_body60_e113708, (locals.var_dplim_dn0 * assign74200_body60_e113707), (locals.var_dplim_dn2 * assign74200_body60_e113707), (locals.var_dplim_dn4 * assign74200_body60_e113707), (locals.var_dplim_dn5 * assign74200_body60_e113707), (locals.var_dplim_dn6 * assign74200_body60_e113707), (locals.var_dplim_dn7 * assign74200_body60_e113707), (locals.var_dplim_dn8 * assign74200_body60_e113707), (locals.var_dplim_dn9 * assign74200_body60_e113707), (locals.var_dplim_dn10 * assign74200_body60_e113707), (locals.var_dplim_dn13 * assign74200_body60_e113707),)
    } else {
        (locals.var_dps0, locals.var_dps0_dn0, locals.var_dps0_dn2, locals.var_dps0_dn4, locals.var_dps0_dn5, locals.var_dps0_dn6, locals.var_dps0_dn7, locals.var_dps0_dn8, locals.var_dps0_dn9, locals.var_dps0_dn10, locals.var_dps0_dn13,)
    }
};
            locals.var_dps0 = assign74200_body60_e113710;
            locals.var_dps0_dn0 = assign74200_body60_e113710_d_n0;
            locals.var_dps0_dn2 = assign74200_body60_e113710_d_n2;
            locals.var_dps0_dn4 = assign74200_body60_e113710_d_n4;
            locals.var_dps0_dn5 = assign74200_body60_e113710_d_n5;
            locals.var_dps0_dn6 = assign74200_body60_e113710_d_n6;
            locals.var_dps0_dn7 = assign74200_body60_e113710_d_n7;
            locals.var_dps0_dn8 = assign74200_body60_e113710_d_n8;
            locals.var_dps0_dn9 = assign74200_body60_e113710_d_n9;
            locals.var_dps0_dn10 = assign74200_body60_e113710_d_n10;
            locals.var_dps0_dn13 = assign74200_body60_e113710_d_n13;
            locals.var_dps0_rv = 0.0;
            let (assign74200_body61_e113721, assign74200_body61_e113721_d_n0, assign74200_body61_e113721_d_n2, assign74200_body61_e113721_d_n4, assign74200_body61_e113721_d_n5, assign74200_body61_e113721_d_n6, assign74200_body61_e113721_d_n7, assign74200_body61_e113721_d_n8, assign74200_body61_e113721_d_n9, assign74200_body61_e113721_d_n10, assign74200_body61_e113721_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1722 != 0.0)) && (locals.var_guard1733 == 0.0)) {
        let assign74200_body61_e113719: f64 = (locals.var_ps0ld + locals.var_dps0);
        (assign74200_body61_e113719, (locals.var_ps0ld_dn0 + locals.var_dps0_dn0), (locals.var_ps0ld_dn2 + locals.var_dps0_dn2), (locals.var_ps0ld_dn4 + locals.var_dps0_dn4), (locals.var_ps0ld_dn5 + locals.var_dps0_dn5), (locals.var_ps0ld_dn6 + locals.var_dps0_dn6), (locals.var_ps0ld_dn7 + locals.var_dps0_dn7), (locals.var_ps0ld_dn8 + locals.var_dps0_dn8), (locals.var_ps0ld_dn9 + locals.var_dps0_dn9), (locals.var_ps0ld_dn10 + locals.var_dps0_dn10), (locals.var_ps0ld_dn13 + locals.var_dps0_dn13),)
    } else {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn13,)
    }
};
            locals.var_ps0ld = assign74200_body61_e113721;
            locals.var_ps0ld_dn0 = assign74200_body61_e113721_d_n0;
            locals.var_ps0ld_dn2 = assign74200_body61_e113721_d_n2;
            locals.var_ps0ld_dn4 = assign74200_body61_e113721_d_n4;
            locals.var_ps0ld_dn5 = assign74200_body61_e113721_d_n5;
            locals.var_ps0ld_dn6 = assign74200_body61_e113721_d_n6;
            locals.var_ps0ld_dn7 = assign74200_body61_e113721_d_n7;
            locals.var_ps0ld_dn8 = assign74200_body61_e113721_d_n8;
            locals.var_ps0ld_dn9 = assign74200_body61_e113721_d_n9;
            locals.var_ps0ld_dn10 = assign74200_body61_e113721_d_n10;
            locals.var_ps0ld_dn13 = assign74200_body61_e113721_d_n13;
            locals.var_ps0ld_rv = 0.0;
            let assign74200_body62_e113723: f64 = (locals.var_dps0).abs();
            let assign74200_body62_e113727: f64 = (locals.var_fs0).abs();
            let assign74200_body62_e113730: f64 = if ((assign74200_body62_e113723 <= 1e-12) && (assign74200_body62_e113727 <= 1e-8)) { 1.0 } else { 0.0 };
            locals.var_guard1735 = assign74200_body62_e113730;
            locals.var_guard1735_rv = 0.0;
            let (assign74200_body63_e113743,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1722 != 0.0)) && (locals.var_guard1733 == 0.0)) && (locals.var_guard1735 != 0.0)) {
        let assign74200_body63_e113741: f64 = (locals.var_flg_conv + 2.0);
        (assign74200_body63_e113741,)
    } else {
        (locals.var_flg_conv,)
    }
};
            locals.var_flg_conv = assign74200_body63_e113743;
            locals.var_flg_conv_rv = 0.0;
            let (assign74200_body64_e113751,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1722 != 0.0)) {
        let assign74200_body64_e113749: f64 = (locals.var_lp_s0 + 1.0);
        (assign74200_body64_e113749,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign74200_body64_e113751;
            locals.var_lp_s0_rv = 0.0;
        }

    }

    pub(super) fn stamp_reactive_block_270(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign74220_e113774, assign74220_e113774_d_n0, assign74220_e113774_d_n2, assign74220_e113774_d_n4, assign74220_e113774_d_n5, assign74220_e113774_d_n6, assign74220_e113774_d_n7, assign74220_e113774_d_n8, assign74220_e113774_d_n9, assign74220_e113774_d_n10, assign74220_e113774_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1722 != 0.0)) {
        let (assign74220_e113772, assign74220_e113772_d_n0, assign74220_e113772_d_n2, assign74220_e113772_d_n4, assign74220_e113772_d_n5, assign74220_e113772_d_n6, assign74220_e113772_d_n7, assign74220_e113772_d_n8, assign74220_e113772_d_n9, assign74220_e113772_d_n10, assign74220_e113772_d_n13,) = {
            if (locals.var_fbsq >= 0.0) {
                let (assign74220_e113767,) = {
                    if (locals.var_chi >= 0.0) {
                        (1.0,)
                    } else {
                        let assign74220_e113766: f64 = (-1.0);
                        (assign74220_e113766,)
                    }
                };
                let assign74220_e113769: f64 = (locals.var_fbsq).sqrt();
                let assign74220_e113770: f64 = (assign74220_e113767 * assign74220_e113769);
                (assign74220_e113770, (assign74220_e113767 * (locals.var_fbsq_dn0 / (2.0 * assign74220_e113769))), (assign74220_e113767 * (locals.var_fbsq_dn2 / (2.0 * assign74220_e113769))), (assign74220_e113767 * (locals.var_fbsq_dn4 / (2.0 * assign74220_e113769))), (assign74220_e113767 * (locals.var_fbsq_dn5 / (2.0 * assign74220_e113769))), (assign74220_e113767 * (locals.var_fbsq_dn6 / (2.0 * assign74220_e113769))), (assign74220_e113767 * (locals.var_fbsq_dn7 / (2.0 * assign74220_e113769))), (assign74220_e113767 * (locals.var_fbsq_dn8 / (2.0 * assign74220_e113769))), (assign74220_e113767 * (locals.var_fbsq_dn9 / (2.0 * assign74220_e113769))), (assign74220_e113767 * (locals.var_fbsq_dn10 / (2.0 * assign74220_e113769))), (assign74220_e113767 * (locals.var_fbsq_dn13 / (2.0 * assign74220_e113769))),)
            } else {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign74220_e113772, assign74220_e113772_d_n0, assign74220_e113772_d_n2, assign74220_e113772_d_n4, assign74220_e113772_d_n5, assign74220_e113772_d_n6, assign74220_e113772_d_n7, assign74220_e113772_d_n8, assign74220_e113772_d_n9, assign74220_e113772_d_n10, assign74220_e113772_d_n13,)
    } else {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn4, locals.var_fb_dn5, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn8, locals.var_fb_dn9, locals.var_fb_dn10, locals.var_fb_dn13,)
    }
};
        locals.var_fb = assign74220_e113774;
        locals.var_fb_dn0 = assign74220_e113774_d_n0;
        locals.var_fb_dn2 = assign74220_e113774_d_n2;
        locals.var_fb_dn4 = assign74220_e113774_d_n4;
        locals.var_fb_dn5 = assign74220_e113774_d_n5;
        locals.var_fb_dn6 = assign74220_e113774_d_n6;
        locals.var_fb_dn7 = assign74220_e113774_d_n7;
        locals.var_fb_dn8 = assign74220_e113774_d_n8;
        locals.var_fb_dn9 = assign74220_e113774_d_n9;
        locals.var_fb_dn10 = assign74220_e113774_d_n10;
        locals.var_fb_dn13 = assign74220_e113774_d_n13;
        locals.var_fb_rv = 0.0;

        let (assign74230_e113782, assign74230_e113782_d_n0, assign74230_e113782_d_n2, assign74230_e113782_d_n4, assign74230_e113782_d_n5, assign74230_e113782_d_n6, assign74230_e113782_d_n7, assign74230_e113782_d_n8, assign74230_e113782_d_n9, assign74230_e113782_d_n10, assign74230_e113782_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1722 != 0.0)) {
        let assign74230_e113780: f64 = (locals.var_c_w_ld * locals.var_fb);
        (assign74230_e113780, ((locals.var_c_w_ld_dn0 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn0)), ((locals.var_c_w_ld_dn2 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn2)), ((locals.var_c_w_ld_dn4 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn4)), ((locals.var_c_w_ld_dn5 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn5)), ((locals.var_c_w_ld_dn6 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn6)), ((locals.var_c_w_ld_dn7 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn7)), ((locals.var_c_w_ld_dn8 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn8)), ((locals.var_c_w_ld_dn9 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn9)), ((locals.var_c_w_ld_dn10 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn10)), ((locals.var_c_w_ld_dn13 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn13)),)
    } else {
        (locals.var_wdld, locals.var_wdld_dn0, locals.var_wdld_dn2, locals.var_wdld_dn4, locals.var_wdld_dn5, locals.var_wdld_dn6, locals.var_wdld_dn7, locals.var_wdld_dn8, locals.var_wdld_dn9, locals.var_wdld_dn10, locals.var_wdld_dn13,)
    }
};
        locals.var_wdld = assign74230_e113782;
        locals.var_wdld_dn0 = assign74230_e113782_d_n0;
        locals.var_wdld_dn2 = assign74230_e113782_d_n2;
        locals.var_wdld_dn4 = assign74230_e113782_d_n4;
        locals.var_wdld_dn5 = assign74230_e113782_d_n5;
        locals.var_wdld_dn6 = assign74230_e113782_d_n6;
        locals.var_wdld_dn7 = assign74230_e113782_d_n7;
        locals.var_wdld_dn8 = assign74230_e113782_d_n8;
        locals.var_wdld_dn9 = assign74230_e113782_d_n9;
        locals.var_wdld_dn10 = assign74230_e113782_d_n10;
        locals.var_wdld_dn13 = assign74230_e113782_d_n13;
        locals.var_wdld_rv = 0.0;

        let (assign74240_e113790, assign74240_e113790_d_n0, assign74240_e113790_d_n2, assign74240_e113790_d_n4, assign74240_e113790_d_n5, assign74240_e113790_d_n6, assign74240_e113790_d_n7, assign74240_e113790_d_n8, assign74240_e113790_d_n9, assign74240_e113790_d_n10, assign74240_e113790_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1722 != 0.0)) {
        let assign74240_e113788: f64 = (locals.var_q_nsubld * locals.var_wdld);
        (assign74240_e113788, (locals.var_q_nsubld * locals.var_wdld_dn0), (locals.var_q_nsubld * locals.var_wdld_dn2), (locals.var_q_nsubld * locals.var_wdld_dn4), (locals.var_q_nsubld * locals.var_wdld_dn5), (locals.var_q_nsubld * locals.var_wdld_dn6), (locals.var_q_nsubld * locals.var_wdld_dn7), (locals.var_q_nsubld * locals.var_wdld_dn8), (locals.var_q_nsubld * locals.var_wdld_dn9), (locals.var_q_nsubld * locals.var_wdld_dn10), (locals.var_q_nsubld * locals.var_wdld_dn13),)
    } else {
        (locals.var_q_dep_ld, locals.var_q_dep_ld_dn0, locals.var_q_dep_ld_dn2, locals.var_q_dep_ld_dn4, locals.var_q_dep_ld_dn5, locals.var_q_dep_ld_dn6, locals.var_q_dep_ld_dn7, locals.var_q_dep_ld_dn8, locals.var_q_dep_ld_dn9, locals.var_q_dep_ld_dn10, locals.var_q_dep_ld_dn13,)
    }
};
        locals.var_q_dep_ld = assign74240_e113790;
        locals.var_q_dep_ld_dn0 = assign74240_e113790_d_n0;
        locals.var_q_dep_ld_dn2 = assign74240_e113790_d_n2;
        locals.var_q_dep_ld_dn4 = assign74240_e113790_d_n4;
        locals.var_q_dep_ld_dn5 = assign74240_e113790_d_n5;
        locals.var_q_dep_ld_dn6 = assign74240_e113790_d_n6;
        locals.var_q_dep_ld_dn7 = assign74240_e113790_d_n7;
        locals.var_q_dep_ld_dn8 = assign74240_e113790_d_n8;
        locals.var_q_dep_ld_dn9 = assign74240_e113790_d_n9;
        locals.var_q_dep_ld_dn10 = assign74240_e113790_d_n10;
        locals.var_q_dep_ld_dn13 = assign74240_e113790_d_n13;
        locals.var_q_dep_ld_rv = 0.0;

        let (assign74250_e113802, assign74250_e113802_d_n0, assign74250_e113802_d_n2, assign74250_e113802_d_n4, assign74250_e113802_d_n5, assign74250_e113802_d_n6, assign74250_e113802_d_n7, assign74250_e113802_d_n8, assign74250_e113802_d_n9, assign74250_e113802_d_n10, assign74250_e113802_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1722 != 0.0)) {
        let assign74250_e113796: f64 = (locals.var_q_dep_ld / locals.var_cnst0over_func);
        let assign74250_e113799: f64 = (10.0 * 2.220446049250313e-16);
        let assign74250_e113800: f64 = (assign74250_e113796 + assign74250_e113799);
        (assign74250_e113800, (((locals.var_q_dep_ld_dn0 * locals.var_cnst0over_func) - (locals.var_q_dep_ld * locals.var_cnst0over_func_dn0)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld_dn2 * locals.var_cnst0over_func) - (locals.var_q_dep_ld * locals.var_cnst0over_func_dn2)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld_dn4 * locals.var_cnst0over_func) - (locals.var_q_dep_ld * locals.var_cnst0over_func_dn4)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld_dn5 * locals.var_cnst0over_func) - (locals.var_q_dep_ld * locals.var_cnst0over_func_dn5)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld_dn6 * locals.var_cnst0over_func) - (locals.var_q_dep_ld * locals.var_cnst0over_func_dn6)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld_dn7 * locals.var_cnst0over_func) - (locals.var_q_dep_ld * locals.var_cnst0over_func_dn7)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld_dn8 * locals.var_cnst0over_func) - (locals.var_q_dep_ld * locals.var_cnst0over_func_dn8)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld_dn9 * locals.var_cnst0over_func) - (locals.var_q_dep_ld * locals.var_cnst0over_func_dn9)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld_dn10 * locals.var_cnst0over_func) - (locals.var_q_dep_ld * locals.var_cnst0over_func_dn10)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld_dn13 * locals.var_cnst0over_func) - (locals.var_q_dep_ld * locals.var_cnst0over_func_dn13)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)),)
    } else {
        (locals.var_xi0p12, locals.var_xi0p12_dn0, locals.var_xi0p12_dn2, locals.var_xi0p12_dn4, locals.var_xi0p12_dn5, locals.var_xi0p12_dn6, locals.var_xi0p12_dn7, locals.var_xi0p12_dn8, locals.var_xi0p12_dn9, locals.var_xi0p12_dn10, locals.var_xi0p12_dn13,)
    }
};
        locals.var_xi0p12 = assign74250_e113802;
        locals.var_xi0p12_dn0 = assign74250_e113802_d_n0;
        locals.var_xi0p12_dn2 = assign74250_e113802_d_n2;
        locals.var_xi0p12_dn4 = assign74250_e113802_d_n4;
        locals.var_xi0p12_dn5 = assign74250_e113802_d_n5;
        locals.var_xi0p12_dn6 = assign74250_e113802_d_n6;
        locals.var_xi0p12_dn7 = assign74250_e113802_d_n7;
        locals.var_xi0p12_dn8 = assign74250_e113802_d_n8;
        locals.var_xi0p12_dn9 = assign74250_e113802_d_n9;
        locals.var_xi0p12_dn10 = assign74250_e113802_d_n10;
        locals.var_xi0p12_dn13 = assign74250_e113802_d_n13;
        locals.var_xi0p12_rv = 0.0;

        let (assign74260_e113810, assign74260_e113810_d_n0, assign74260_e113810_d_n2, assign74260_e113810_d_n4, assign74260_e113810_d_n5, assign74260_e113810_d_n6, assign74260_e113810_d_n7, assign74260_e113810_d_n8, assign74260_e113810_d_n9, assign74260_e113810_d_n10, assign74260_e113810_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1722 != 0.0)) {
        let assign74260_e113808: f64 = (locals.var_cnst0over_func * locals.var_xi0p12);
        (assign74260_e113808, ((locals.var_cnst0over_func_dn0 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn0)), ((locals.var_cnst0over_func_dn2 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn2)), ((locals.var_cnst0over_func_dn4 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn4)), ((locals.var_cnst0over_func_dn5 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn5)), ((locals.var_cnst0over_func_dn6 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn6)), ((locals.var_cnst0over_func_dn7 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn7)), ((locals.var_cnst0over_func_dn8 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn8)), ((locals.var_cnst0over_func_dn9 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn9)), ((locals.var_cnst0over_func_dn10 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn10)), ((locals.var_cnst0over_func_dn13 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn13)),)
    } else {
        (locals.var_qbuld, locals.var_qbuld_dn0, locals.var_qbuld_dn2, locals.var_qbuld_dn4, locals.var_qbuld_dn5, locals.var_qbuld_dn6, locals.var_qbuld_dn7, locals.var_qbuld_dn8, locals.var_qbuld_dn9, locals.var_qbuld_dn10, locals.var_qbuld_dn13,)
    }
};
        locals.var_qbuld = assign74260_e113810;
        locals.var_qbuld_dn0 = assign74260_e113810_d_n0;
        locals.var_qbuld_dn2 = assign74260_e113810_d_n2;
        locals.var_qbuld_dn4 = assign74260_e113810_d_n4;
        locals.var_qbuld_dn5 = assign74260_e113810_d_n5;
        locals.var_qbuld_dn6 = assign74260_e113810_d_n6;
        locals.var_qbuld_dn7 = assign74260_e113810_d_n7;
        locals.var_qbuld_dn8 = assign74260_e113810_d_n8;
        locals.var_qbuld_dn9 = assign74260_e113810_d_n9;
        locals.var_qbuld_dn10 = assign74260_e113810_d_n10;
        locals.var_qbuld_dn13 = assign74260_e113810_d_n13;
        locals.var_qbuld_rv = 0.0;

        let (assign74270_e113820, assign74270_e113820_d_n0, assign74270_e113820_d_n2, assign74270_e113820_d_n4, assign74270_e113820_d_n5, assign74270_e113820_d_n6, assign74270_e113820_d_n7, assign74270_e113820_d_n8, assign74270_e113820_d_n9, assign74270_e113820_d_n10, assign74270_e113820_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1722 != 0.0)) {
        let assign74270_e113817: f64 = (locals.var_fs02 + locals.var_xi0p12);
        let assign74270_e113818: f64 = (1.0 / assign74270_e113817);
        (assign74270_e113818, (-((locals.var_fs02_dn0 + locals.var_xi0p12_dn0) / (assign74270_e113817 * assign74270_e113817))), (-((locals.var_fs02_dn2 + locals.var_xi0p12_dn2) / (assign74270_e113817 * assign74270_e113817))), (-((locals.var_fs02_dn4 + locals.var_xi0p12_dn4) / (assign74270_e113817 * assign74270_e113817))), (-((locals.var_fs02_dn5 + locals.var_xi0p12_dn5) / (assign74270_e113817 * assign74270_e113817))), (-((locals.var_fs02_dn6 + locals.var_xi0p12_dn6) / (assign74270_e113817 * assign74270_e113817))), (-((locals.var_fs02_dn7 + locals.var_xi0p12_dn7) / (assign74270_e113817 * assign74270_e113817))), (-((locals.var_fs02_dn8 + locals.var_xi0p12_dn8) / (assign74270_e113817 * assign74270_e113817))), (-((locals.var_fs02_dn9 + locals.var_xi0p12_dn9) / (assign74270_e113817 * assign74270_e113817))), (-((locals.var_fs02_dn10 + locals.var_xi0p12_dn10) / (assign74270_e113817 * assign74270_e113817))), (-((locals.var_fs02_dn13 + locals.var_xi0p12_dn13) / (assign74270_e113817 * assign74270_e113817))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign74270_e113820;
        locals.var_t1_dn0 = assign74270_e113820_d_n0;
        locals.var_t1_dn2 = assign74270_e113820_d_n2;
        locals.var_t1_dn4 = assign74270_e113820_d_n4;
        locals.var_t1_dn5 = assign74270_e113820_d_n5;
        locals.var_t1_dn6 = assign74270_e113820_d_n6;
        locals.var_t1_dn7 = assign74270_e113820_d_n7;
        locals.var_t1_dn8 = assign74270_e113820_d_n8;
        locals.var_t1_dn9 = assign74270_e113820_d_n9;
        locals.var_t1_dn10 = assign74270_e113820_d_n10;
        locals.var_t1_dn13 = assign74270_e113820_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign74280_e113830, assign74280_e113830_d_n0, assign74280_e113830_d_n2, assign74280_e113830_d_n4, assign74280_e113830_d_n5, assign74280_e113830_d_n6, assign74280_e113830_d_n7, assign74280_e113830_d_n8, assign74280_e113830_d_n9, assign74280_e113830_d_n10, assign74280_e113830_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1722 != 0.0)) {
        let assign74280_e113826: f64 = (locals.var_cnst0over_func * locals.var_fs01);
        let assign74280_e113828: f64 = (assign74280_e113826 * locals.var_t1);
        (assign74280_e113828, ((((locals.var_cnst0over_func_dn0 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn0)) * locals.var_t1) + (assign74280_e113826 * locals.var_t1_dn0)), ((((locals.var_cnst0over_func_dn2 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn2)) * locals.var_t1) + (assign74280_e113826 * locals.var_t1_dn2)), ((((locals.var_cnst0over_func_dn4 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn4)) * locals.var_t1) + (assign74280_e113826 * locals.var_t1_dn4)), ((((locals.var_cnst0over_func_dn5 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn5)) * locals.var_t1) + (assign74280_e113826 * locals.var_t1_dn5)), ((((locals.var_cnst0over_func_dn6 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn6)) * locals.var_t1) + (assign74280_e113826 * locals.var_t1_dn6)), ((((locals.var_cnst0over_func_dn7 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn7)) * locals.var_t1) + (assign74280_e113826 * locals.var_t1_dn7)), ((((locals.var_cnst0over_func_dn8 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn8)) * locals.var_t1) + (assign74280_e113826 * locals.var_t1_dn8)), ((((locals.var_cnst0over_func_dn9 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn9)) * locals.var_t1) + (assign74280_e113826 * locals.var_t1_dn9)), ((((locals.var_cnst0over_func_dn10 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn10)) * locals.var_t1) + (assign74280_e113826 * locals.var_t1_dn10)), ((((locals.var_cnst0over_func_dn13 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn13)) * locals.var_t1) + (assign74280_e113826 * locals.var_t1_dn13)),)
    } else {
        (locals.var_qiuld, locals.var_qiuld_dn0, locals.var_qiuld_dn2, locals.var_qiuld_dn4, locals.var_qiuld_dn5, locals.var_qiuld_dn6, locals.var_qiuld_dn7, locals.var_qiuld_dn8, locals.var_qiuld_dn9, locals.var_qiuld_dn10, locals.var_qiuld_dn13,)
    }
};
        locals.var_qiuld = assign74280_e113830;
        locals.var_qiuld_dn0 = assign74280_e113830_d_n0;
        locals.var_qiuld_dn2 = assign74280_e113830_d_n2;
        locals.var_qiuld_dn4 = assign74280_e113830_d_n4;
        locals.var_qiuld_dn5 = assign74280_e113830_d_n5;
        locals.var_qiuld_dn6 = assign74280_e113830_d_n6;
        locals.var_qiuld_dn7 = assign74280_e113830_d_n7;
        locals.var_qiuld_dn8 = assign74280_e113830_d_n8;
        locals.var_qiuld_dn9 = assign74280_e113830_d_n9;
        locals.var_qiuld_dn10 = assign74280_e113830_d_n10;
        locals.var_qiuld_dn13 = assign74280_e113830_d_n13;
        locals.var_qiuld_rv = 0.0;

        let (assign74290_e113838, assign74290_e113838_d_n0, assign74290_e113838_d_n2, assign74290_e113838_d_n4, assign74290_e113838_d_n5, assign74290_e113838_d_n6, assign74290_e113838_d_n7, assign74290_e113838_d_n8, assign74290_e113838_d_n9, assign74290_e113838_d_n10, assign74290_e113838_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1722 != 0.0)) {
        let assign74290_e113836: f64 = (locals.var_qbuld + locals.var_qiuld);
        (assign74290_e113836, (locals.var_qbuld_dn0 + locals.var_qiuld_dn0), (locals.var_qbuld_dn2 + locals.var_qiuld_dn2), (locals.var_qbuld_dn4 + locals.var_qiuld_dn4), (locals.var_qbuld_dn5 + locals.var_qiuld_dn5), (locals.var_qbuld_dn6 + locals.var_qiuld_dn6), (locals.var_qbuld_dn7 + locals.var_qiuld_dn7), (locals.var_qbuld_dn8 + locals.var_qiuld_dn8), (locals.var_qbuld_dn9 + locals.var_qiuld_dn9), (locals.var_qbuld_dn10 + locals.var_qiuld_dn10), (locals.var_qbuld_dn13 + locals.var_qiuld_dn13),)
    } else {
        (locals.var_qsuld, locals.var_qsuld_dn0, locals.var_qsuld_dn2, locals.var_qsuld_dn4, locals.var_qsuld_dn5, locals.var_qsuld_dn6, locals.var_qsuld_dn7, locals.var_qsuld_dn8, locals.var_qsuld_dn9, locals.var_qsuld_dn10, locals.var_qsuld_dn13,)
    }
};
        locals.var_qsuld = assign74290_e113838;
        locals.var_qsuld_dn0 = assign74290_e113838_d_n0;
        locals.var_qsuld_dn2 = assign74290_e113838_d_n2;
        locals.var_qsuld_dn4 = assign74290_e113838_d_n4;
        locals.var_qsuld_dn5 = assign74290_e113838_d_n5;
        locals.var_qsuld_dn6 = assign74290_e113838_d_n6;
        locals.var_qsuld_dn7 = assign74290_e113838_d_n7;
        locals.var_qsuld_dn8 = assign74290_e113838_d_n8;
        locals.var_qsuld_dn9 = assign74290_e113838_d_n9;
        locals.var_qsuld_dn10 = assign74290_e113838_d_n10;
        locals.var_qsuld_dn13 = assign74290_e113838_d_n13;
        locals.var_qsuld_rv = 0.0;

        let (assign74300_e113844, assign74300_e113844_d_n0, assign74300_e113844_d_n2, assign74300_e113844_d_n4, assign74300_e113844_d_n5, assign74300_e113844_d_n6, assign74300_e113844_d_n7, assign74300_e113844_d_n8, assign74300_e113844_d_n9, assign74300_e113844_d_n10, assign74300_e113844_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign74300_e113842: f64 = (locals.var_qsuld - locals.var_qbuld);
        (assign74300_e113842, (locals.var_qsuld_dn0 - locals.var_qbuld_dn0), (locals.var_qsuld_dn2 - locals.var_qbuld_dn2), (locals.var_qsuld_dn4 - locals.var_qbuld_dn4), (locals.var_qsuld_dn5 - locals.var_qbuld_dn5), (locals.var_qsuld_dn6 - locals.var_qbuld_dn6), (locals.var_qsuld_dn7 - locals.var_qbuld_dn7), (locals.var_qsuld_dn8 - locals.var_qbuld_dn8), (locals.var_qsuld_dn9 - locals.var_qbuld_dn9), (locals.var_qsuld_dn10 - locals.var_qbuld_dn10), (locals.var_qsuld_dn13 - locals.var_qbuld_dn13),)
    } else {
        (locals.var_qiuld, locals.var_qiuld_dn0, locals.var_qiuld_dn2, locals.var_qiuld_dn4, locals.var_qiuld_dn5, locals.var_qiuld_dn6, locals.var_qiuld_dn7, locals.var_qiuld_dn8, locals.var_qiuld_dn9, locals.var_qiuld_dn10, locals.var_qiuld_dn13,)
    }
};
        locals.var_qiuld = assign74300_e113844;
        locals.var_qiuld_dn0 = assign74300_e113844_d_n0;
        locals.var_qiuld_dn2 = assign74300_e113844_d_n2;
        locals.var_qiuld_dn4 = assign74300_e113844_d_n4;
        locals.var_qiuld_dn5 = assign74300_e113844_d_n5;
        locals.var_qiuld_dn6 = assign74300_e113844_d_n6;
        locals.var_qiuld_dn7 = assign74300_e113844_d_n7;
        locals.var_qiuld_dn8 = assign74300_e113844_d_n8;
        locals.var_qiuld_dn9 = assign74300_e113844_d_n9;
        locals.var_qiuld_dn10 = assign74300_e113844_d_n10;
        locals.var_qiuld_dn13 = assign74300_e113844_d_n13;
        locals.var_qiuld_rv = 0.0;

        let assign74310_e113847: f64 = if locals.var_lover_func < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1737 = assign74310_e113847;
        locals.var_guard1737_rv = 0.0;

        let (assign74320_e113854, assign74320_e113854_d_n0, assign74320_e113854_d_n2, assign74320_e113854_d_n4, assign74320_e113854_d_n5, assign74320_e113854_d_n6, assign74320_e113854_d_n7, assign74320_e113854_d_n8, assign74320_e113854_d_n9, assign74320_e113854_d_n10, assign74320_e113854_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1737 != 0.0)) {
        let assign74320_e113852: f64 = (-locals.var_lover_func);
        (assign74320_e113852, (-locals.var_lover_func_dn0), (-locals.var_lover_func_dn2), (-locals.var_lover_func_dn4), (-locals.var_lover_func_dn5), (-locals.var_lover_func_dn6), (-locals.var_lover_func_dn7), (-locals.var_lover_func_dn8), (-locals.var_lover_func_dn9), (-locals.var_lover_func_dn10), (-locals.var_lover_func_dn13),)
    } else {
        (locals.var_lover_func, locals.var_lover_func_dn0, locals.var_lover_func_dn2, locals.var_lover_func_dn4, locals.var_lover_func_dn5, locals.var_lover_func_dn6, locals.var_lover_func_dn7, locals.var_lover_func_dn8, locals.var_lover_func_dn9, locals.var_lover_func_dn10, locals.var_lover_func_dn13,)
    }
};
        locals.var_lover_func = assign74320_e113854;
        locals.var_lover_func_dn0 = assign74320_e113854_d_n0;
        locals.var_lover_func_dn2 = assign74320_e113854_d_n2;
        locals.var_lover_func_dn4 = assign74320_e113854_d_n4;
        locals.var_lover_func_dn5 = assign74320_e113854_d_n5;
        locals.var_lover_func_dn6 = assign74320_e113854_d_n6;
        locals.var_lover_func_dn7 = assign74320_e113854_d_n7;
        locals.var_lover_func_dn8 = assign74320_e113854_d_n8;
        locals.var_lover_func_dn9 = assign74320_e113854_d_n9;
        locals.var_lover_func_dn10 = assign74320_e113854_d_n10;
        locals.var_lover_func_dn13 = assign74320_e113854_d_n13;
        locals.var_lover_func_rv = 0.0;

        let assign74330_e113857: f64 = if p.p55 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1738 = assign74330_e113857;
        locals.var_guard1738_rv = 0.0;

        let assign74340_e113860: f64 = if p.p50 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1739 = assign74340_e113860;
        locals.var_guard1739_rv = 0.0;

        let (assign74350_e113871, assign74350_e113871_d_n0, assign74350_e113871_d_n2, assign74350_e113871_d_n4, assign74350_e113871_d_n5, assign74350_e113871_d_n6, assign74350_e113871_d_n7, assign74350_e113871_d_n8, assign74350_e113871_d_n9, assign74350_e113871_d_n10, assign74350_e113871_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1737 != 0.0)) && (locals.var_guard1738 != 0.0)) && (locals.var_guard1739 != 0.0)) {
        let assign74350_e113869: f64 = (-locals.var_ps0ld);
        (assign74350_e113869, (-locals.var_ps0ld_dn0), (-locals.var_ps0ld_dn2), (-locals.var_ps0ld_dn4), (-locals.var_ps0ld_dn5), (-locals.var_ps0ld_dn6), (-locals.var_ps0ld_dn7), (-locals.var_ps0ld_dn8), (-locals.var_ps0ld_dn9), (-locals.var_ps0ld_dn10), (-locals.var_ps0ld_dn13),)
    } else {
        (locals.var_vx, locals.var_vx_dn0, locals.var_vx_dn2, locals.var_vx_dn4, locals.var_vx_dn5, locals.var_vx_dn6, locals.var_vx_dn7, locals.var_vx_dn8, locals.var_vx_dn9, locals.var_vx_dn10, locals.var_vx_dn13,)
    }
};
        locals.var_vx = assign74350_e113871;
        locals.var_vx_dn0 = assign74350_e113871_d_n0;
        locals.var_vx_dn2 = assign74350_e113871_d_n2;
        locals.var_vx_dn4 = assign74350_e113871_d_n4;
        locals.var_vx_dn5 = assign74350_e113871_d_n5;
        locals.var_vx_dn6 = assign74350_e113871_d_n6;
        locals.var_vx_dn7 = assign74350_e113871_d_n7;
        locals.var_vx_dn8 = assign74350_e113871_d_n8;
        locals.var_vx_dn9 = assign74350_e113871_d_n9;
        locals.var_vx_dn10 = assign74350_e113871_d_n10;
        locals.var_vx_dn13 = assign74350_e113871_d_n13;
        locals.var_vx_rv = 0.0;

        let (assign74360_e113882, assign74360_e113882_d_n0, assign74360_e113882_d_n2, assign74360_e113882_d_n4, assign74360_e113882_d_n5, assign74360_e113882_d_n6, assign74360_e113882_d_n7, assign74360_e113882_d_n8, assign74360_e113882_d_n9, assign74360_e113882_d_n10, assign74360_e113882_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1737 != 0.0)) && (locals.var_guard1738 != 0.0)) && (locals.var_guard1739 == 0.0)) {
        (locals.var_vxbgmt, locals.var_vxbgmt_dn0, locals.var_vxbgmt_dn2, locals.var_vxbgmt_dn4, locals.var_vxbgmt_dn5, locals.var_vxbgmt_dn6, locals.var_vxbgmt_dn7, locals.var_vxbgmt_dn8, locals.var_vxbgmt_dn9, locals.var_vxbgmt_dn10, locals.var_vxbgmt_dn13,)
    } else {
        (locals.var_vx, locals.var_vx_dn0, locals.var_vx_dn2, locals.var_vx_dn4, locals.var_vx_dn5, locals.var_vx_dn6, locals.var_vx_dn7, locals.var_vx_dn8, locals.var_vx_dn9, locals.var_vx_dn10, locals.var_vx_dn13,)
    }
};
        locals.var_vx = assign74360_e113882;
        locals.var_vx_dn0 = assign74360_e113882_d_n0;
        locals.var_vx_dn2 = assign74360_e113882_d_n2;
        locals.var_vx_dn4 = assign74360_e113882_d_n4;
        locals.var_vx_dn5 = assign74360_e113882_d_n5;
        locals.var_vx_dn6 = assign74360_e113882_d_n6;
        locals.var_vx_dn7 = assign74360_e113882_d_n7;
        locals.var_vx_dn8 = assign74360_e113882_d_n8;
        locals.var_vx_dn9 = assign74360_e113882_d_n9;
        locals.var_vx_dn10 = assign74360_e113882_d_n10;
        locals.var_vx_dn13 = assign74360_e113882_d_n13;
        locals.var_vx_rv = 0.0;

        let (assign74370_e113903, assign74370_e113903_d_n0, assign74370_e113903_d_n2, assign74370_e113903_d_n4, assign74370_e113903_d_n5, assign74370_e113903_d_n6, assign74370_e113903_d_n7, assign74370_e113903_d_n8, assign74370_e113903_d_n9, assign74370_e113903_d_n10, assign74370_e113903_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1737 != 0.0)) && (locals.var_guard1738 != 0.0)) {
        let assign74370_e113890: f64 = (locals.var_vx + p.p137);
        let assign74370_e113893: f64 = (locals.var_vx + p.p137);
        let assign74370_e113894: f64 = (assign74370_e113890 * assign74370_e113893);
        let assign74370_e113897: f64 = (4.0 * 0.1);
        let assign74370_e113899: f64 = (assign74370_e113897 * 0.1);
        let assign74370_e113900: f64 = (assign74370_e113894 + assign74370_e113899);
        let assign74370_e113901: f64 = (assign74370_e113900).sqrt();
        (assign74370_e113901, (((locals.var_vx_dn0 * assign74370_e113893) + (assign74370_e113890 * locals.var_vx_dn0)) / (2.0 * assign74370_e113901)), (((locals.var_vx_dn2 * assign74370_e113893) + (assign74370_e113890 * locals.var_vx_dn2)) / (2.0 * assign74370_e113901)), (((locals.var_vx_dn4 * assign74370_e113893) + (assign74370_e113890 * locals.var_vx_dn4)) / (2.0 * assign74370_e113901)), (((locals.var_vx_dn5 * assign74370_e113893) + (assign74370_e113890 * locals.var_vx_dn5)) / (2.0 * assign74370_e113901)), (((locals.var_vx_dn6 * assign74370_e113893) + (assign74370_e113890 * locals.var_vx_dn6)) / (2.0 * assign74370_e113901)), (((locals.var_vx_dn7 * assign74370_e113893) + (assign74370_e113890 * locals.var_vx_dn7)) / (2.0 * assign74370_e113901)), (((locals.var_vx_dn8 * assign74370_e113893) + (assign74370_e113890 * locals.var_vx_dn8)) / (2.0 * assign74370_e113901)), (((locals.var_vx_dn9 * assign74370_e113893) + (assign74370_e113890 * locals.var_vx_dn9)) / (2.0 * assign74370_e113901)), (((locals.var_vx_dn10 * assign74370_e113893) + (assign74370_e113890 * locals.var_vx_dn10)) / (2.0 * assign74370_e113901)), (((locals.var_vx_dn13 * assign74370_e113893) + (assign74370_e113890 * locals.var_vx_dn13)) / (2.0 * assign74370_e113901)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign74370_e113903;
        locals.var_tmf2_dn0 = assign74370_e113903_d_n0;
        locals.var_tmf2_dn2 = assign74370_e113903_d_n2;
        locals.var_tmf2_dn4 = assign74370_e113903_d_n4;
        locals.var_tmf2_dn5 = assign74370_e113903_d_n5;
        locals.var_tmf2_dn6 = assign74370_e113903_d_n6;
        locals.var_tmf2_dn7 = assign74370_e113903_d_n7;
        locals.var_tmf2_dn8 = assign74370_e113903_d_n8;
        locals.var_tmf2_dn9 = assign74370_e113903_d_n9;
        locals.var_tmf2_dn10 = assign74370_e113903_d_n10;
        locals.var_tmf2_dn13 = assign74370_e113903_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign74380_e113919, assign74380_e113919_d_n0, assign74380_e113919_d_n2, assign74380_e113919_d_n4, assign74380_e113919_d_n5, assign74380_e113919_d_n6, assign74380_e113919_d_n7, assign74380_e113919_d_n8, assign74380_e113919_d_n9, assign74380_e113919_d_n10, assign74380_e113919_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1737 != 0.0)) && (locals.var_guard1738 != 0.0)) {
        let assign74380_e113913: f64 = (locals.var_vx + p.p137);
        let assign74380_e113915: f64 = (assign74380_e113913 / locals.var_tmf2);
        let assign74380_e113916: f64 = (1.0 + assign74380_e113915);
        let assign74380_e113917: f64 = (0.5 * assign74380_e113916);
        (assign74380_e113917, (0.5 * (((locals.var_vx_dn0 * locals.var_tmf2) - (assign74380_e113913 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vx_dn2 * locals.var_tmf2) - (assign74380_e113913 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vx_dn4 * locals.var_tmf2) - (assign74380_e113913 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vx_dn5 * locals.var_tmf2) - (assign74380_e113913 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vx_dn6 * locals.var_tmf2) - (assign74380_e113913 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vx_dn7 * locals.var_tmf2) - (assign74380_e113913 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vx_dn8 * locals.var_tmf2) - (assign74380_e113913 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vx_dn9 * locals.var_tmf2) - (assign74380_e113913 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vx_dn10 * locals.var_tmf2) - (assign74380_e113913 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vx_dn13 * locals.var_tmf2) - (assign74380_e113913 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign74380_e113919;
        locals.var_t9_dn0 = assign74380_e113919_d_n0;
        locals.var_t9_dn2 = assign74380_e113919_d_n2;
        locals.var_t9_dn4 = assign74380_e113919_d_n4;
        locals.var_t9_dn5 = assign74380_e113919_d_n5;
        locals.var_t9_dn6 = assign74380_e113919_d_n6;
        locals.var_t9_dn7 = assign74380_e113919_d_n7;
        locals.var_t9_dn8 = assign74380_e113919_d_n8;
        locals.var_t9_dn9 = assign74380_e113919_d_n9;
        locals.var_t9_dn10 = assign74380_e113919_d_n10;
        locals.var_t9_dn13 = assign74380_e113919_d_n13;
        locals.var_t9_rv = 0.0;

        let (assign74390_e113933, assign74390_e113933_d_n0, assign74390_e113933_d_n2, assign74390_e113933_d_n4, assign74390_e113933_d_n5, assign74390_e113933_d_n6, assign74390_e113933_d_n7, assign74390_e113933_d_n8, assign74390_e113933_d_n9, assign74390_e113933_d_n10, assign74390_e113933_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1737 != 0.0)) && (locals.var_guard1738 != 0.0)) {
        let assign74390_e113928: f64 = (locals.var_vx + p.p137);
        let assign74390_e113930: f64 = (assign74390_e113928 + locals.var_tmf2);
        let assign74390_e113931: f64 = (0.5 * assign74390_e113930);
        (assign74390_e113931, (0.5 * (locals.var_vx_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_vx_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_vx_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_vx_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_vx_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_vx_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_vx_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_vx_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_vx_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_vx_dn13 + locals.var_tmf2_dn13)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign74390_e113933;
        locals.var_t2_dn0 = assign74390_e113933_d_n0;
        locals.var_t2_dn2 = assign74390_e113933_d_n2;
        locals.var_t2_dn4 = assign74390_e113933_d_n4;
        locals.var_t2_dn5 = assign74390_e113933_d_n5;
        locals.var_t2_dn6 = assign74390_e113933_d_n6;
        locals.var_t2_dn7 = assign74390_e113933_d_n7;
        locals.var_t2_dn8 = assign74390_e113933_d_n8;
        locals.var_t2_dn9 = assign74390_e113933_d_n9;
        locals.var_t2_dn10 = assign74390_e113933_d_n10;
        locals.var_t2_dn13 = assign74390_e113933_d_n13;
        locals.var_t2_rv = 0.0;

        let assign74400_e113936: f64 = if locals.var_t2 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1740 = assign74400_e113936;
        locals.var_guard1740_rv = 0.0;

        let (assign74410_e113946, assign74410_e113946_d_n0, assign74410_e113946_d_n2, assign74410_e113946_d_n4, assign74410_e113946_d_n5, assign74410_e113946_d_n6, assign74410_e113946_d_n7, assign74410_e113946_d_n8, assign74410_e113946_d_n9, assign74410_e113946_d_n10, assign74410_e113946_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1737 != 0.0)) && (locals.var_guard1738 != 0.0)) && (locals.var_guard1740 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign74410_e113946;
        locals.var_t2_dn0 = assign74410_e113946_d_n0;
        locals.var_t2_dn2 = assign74410_e113946_d_n2;
        locals.var_t2_dn4 = assign74410_e113946_d_n4;
        locals.var_t2_dn5 = assign74410_e113946_d_n5;
        locals.var_t2_dn6 = assign74410_e113946_d_n6;
        locals.var_t2_dn7 = assign74410_e113946_d_n7;
        locals.var_t2_dn8 = assign74410_e113946_d_n8;
        locals.var_t2_dn9 = assign74410_e113946_d_n9;
        locals.var_t2_dn10 = assign74410_e113946_d_n10;
        locals.var_t2_dn13 = assign74410_e113946_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign74420_e113956, assign74420_e113956_d_n0, assign74420_e113956_d_n2, assign74420_e113956_d_n4, assign74420_e113956_d_n5, assign74420_e113956_d_n6, assign74420_e113956_d_n7, assign74420_e113956_d_n8, assign74420_e113956_d_n9, assign74420_e113956_d_n10, assign74420_e113956_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1737 != 0.0)) && (locals.var_guard1738 != 0.0)) && (locals.var_guard1740 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign74420_e113956;
        locals.var_t9_dn0 = assign74420_e113956_d_n0;
        locals.var_t9_dn2 = assign74420_e113956_d_n2;
        locals.var_t9_dn4 = assign74420_e113956_d_n4;
        locals.var_t9_dn5 = assign74420_e113956_d_n5;
        locals.var_t9_dn6 = assign74420_e113956_d_n6;
        locals.var_t9_dn7 = assign74420_e113956_d_n7;
        locals.var_t9_dn8 = assign74420_e113956_d_n8;
        locals.var_t9_dn9 = assign74420_e113956_d_n9;
        locals.var_t9_dn10 = assign74420_e113956_d_n10;
        locals.var_t9_dn13 = assign74420_e113956_d_n13;
        locals.var_t9_rv = 0.0;

        let (assign74430_e113969, assign74430_e113969_d_n0, assign74430_e113969_d_n2, assign74430_e113969_d_n4, assign74430_e113969_d_n5, assign74430_e113969_d_n6, assign74430_e113969_d_n7, assign74430_e113969_d_n8, assign74430_e113969_d_n9, assign74430_e113969_d_n10, assign74430_e113969_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1737 != 0.0)) && (locals.var_guard1738 != 0.0)) {
        let assign74430_e113964: f64 = (locals.var_kjunc * locals.var_t2);
        let assign74430_e113965: f64 = (assign74430_e113964).sqrt();
        let assign74430_e113967: f64 = (assign74430_e113965 * p.p432);
        (assign74430_e113967, ((((locals.var_kjunc_dn0 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn0)) / (2.0 * assign74430_e113965)) * p.p432), ((((locals.var_kjunc_dn2 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn2)) / (2.0 * assign74430_e113965)) * p.p432), ((((locals.var_kjunc_dn4 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn4)) / (2.0 * assign74430_e113965)) * p.p432), ((((locals.var_kjunc_dn5 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn5)) / (2.0 * assign74430_e113965)) * p.p432), ((((locals.var_kjunc_dn6 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn6)) / (2.0 * assign74430_e113965)) * p.p432), ((((locals.var_kjunc_dn7 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn7)) / (2.0 * assign74430_e113965)) * p.p432), ((((locals.var_kjunc_dn8 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn8)) / (2.0 * assign74430_e113965)) * p.p432), ((((locals.var_kjunc_dn9 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn9)) / (2.0 * assign74430_e113965)) * p.p432), ((((locals.var_kjunc_dn10 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn10)) / (2.0 * assign74430_e113965)) * p.p432), ((((locals.var_kjunc_dn13 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn13)) / (2.0 * assign74430_e113965)) * p.p432),)
    } else {
        (locals.var_wjunc0, locals.var_wjunc0_dn0, locals.var_wjunc0_dn2, locals.var_wjunc0_dn4, locals.var_wjunc0_dn5, locals.var_wjunc0_dn6, locals.var_wjunc0_dn7, locals.var_wjunc0_dn8, locals.var_wjunc0_dn9, locals.var_wjunc0_dn10, locals.var_wjunc0_dn13,)
    }
};
        locals.var_wjunc0 = assign74430_e113969;
        locals.var_wjunc0_dn0 = assign74430_e113969_d_n0;
        locals.var_wjunc0_dn2 = assign74430_e113969_d_n2;
        locals.var_wjunc0_dn4 = assign74430_e113969_d_n4;
        locals.var_wjunc0_dn5 = assign74430_e113969_d_n5;
        locals.var_wjunc0_dn6 = assign74430_e113969_d_n6;
        locals.var_wjunc0_dn7 = assign74430_e113969_d_n7;
        locals.var_wjunc0_dn8 = assign74430_e113969_d_n8;
        locals.var_wjunc0_dn9 = assign74430_e113969_d_n9;
        locals.var_wjunc0_dn10 = assign74430_e113969_d_n10;
        locals.var_wjunc0_dn13 = assign74430_e113969_d_n13;
        locals.var_wjunc0_rv = 0.0;

        let (assign74440_e113983, assign74440_e113983_d_n0, assign74440_e113983_d_n2, assign74440_e113983_d_n4, assign74440_e113983_d_n5, assign74440_e113983_d_n6, assign74440_e113983_d_n7, assign74440_e113983_d_n8, assign74440_e113983_d_n9, assign74440_e113983_d_n10, assign74440_e113983_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1737 != 0.0)) && (locals.var_guard1738 != 0.0)) {
        let assign74440_e113977: f64 = (locals.var_lover_func - locals.var_wjunc0);
        let assign74440_e113980: f64 = (0.1 * locals.var_lover_func);
        let assign74440_e113981: f64 = (assign74440_e113977 - assign74440_e113980);
        (assign74440_e113981, ((locals.var_lover_func_dn0 - locals.var_wjunc0_dn0) - (0.1 * locals.var_lover_func_dn0)), ((locals.var_lover_func_dn2 - locals.var_wjunc0_dn2) - (0.1 * locals.var_lover_func_dn2)), ((locals.var_lover_func_dn4 - locals.var_wjunc0_dn4) - (0.1 * locals.var_lover_func_dn4)), ((locals.var_lover_func_dn5 - locals.var_wjunc0_dn5) - (0.1 * locals.var_lover_func_dn5)), ((locals.var_lover_func_dn6 - locals.var_wjunc0_dn6) - (0.1 * locals.var_lover_func_dn6)), ((locals.var_lover_func_dn7 - locals.var_wjunc0_dn7) - (0.1 * locals.var_lover_func_dn7)), ((locals.var_lover_func_dn8 - locals.var_wjunc0_dn8) - (0.1 * locals.var_lover_func_dn8)), ((locals.var_lover_func_dn9 - locals.var_wjunc0_dn9) - (0.1 * locals.var_lover_func_dn9)), ((locals.var_lover_func_dn10 - locals.var_wjunc0_dn10) - (0.1 * locals.var_lover_func_dn10)), ((locals.var_lover_func_dn13 - locals.var_wjunc0_dn13) - (0.1 * locals.var_lover_func_dn13)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign74440_e113983;
        locals.var_tmf1_dn0 = assign74440_e113983_d_n0;
        locals.var_tmf1_dn2 = assign74440_e113983_d_n2;
        locals.var_tmf1_dn4 = assign74440_e113983_d_n4;
        locals.var_tmf1_dn5 = assign74440_e113983_d_n5;
        locals.var_tmf1_dn6 = assign74440_e113983_d_n6;
        locals.var_tmf1_dn7 = assign74440_e113983_d_n7;
        locals.var_tmf1_dn8 = assign74440_e113983_d_n8;
        locals.var_tmf1_dn9 = assign74440_e113983_d_n9;
        locals.var_tmf1_dn10 = assign74440_e113983_d_n10;
        locals.var_tmf1_dn13 = assign74440_e113983_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign74450_e113997, assign74450_e113997_d_n0, assign74450_e113997_d_n2, assign74450_e113997_d_n4, assign74450_e113997_d_n5, assign74450_e113997_d_n6, assign74450_e113997_d_n7, assign74450_e113997_d_n8, assign74450_e113997_d_n9, assign74450_e113997_d_n10, assign74450_e113997_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1737 != 0.0)) && (locals.var_guard1738 != 0.0)) {
        let assign74450_e113991: f64 = (4.0 * locals.var_lover_func);
        let assign74450_e113994: f64 = (0.1 * locals.var_lover_func);
        let assign74450_e113995: f64 = (assign74450_e113991 * assign74450_e113994);
        (assign74450_e113995, (((4.0 * locals.var_lover_func_dn0) * assign74450_e113994) + (assign74450_e113991 * (0.1 * locals.var_lover_func_dn0))), (((4.0 * locals.var_lover_func_dn2) * assign74450_e113994) + (assign74450_e113991 * (0.1 * locals.var_lover_func_dn2))), (((4.0 * locals.var_lover_func_dn4) * assign74450_e113994) + (assign74450_e113991 * (0.1 * locals.var_lover_func_dn4))), (((4.0 * locals.var_lover_func_dn5) * assign74450_e113994) + (assign74450_e113991 * (0.1 * locals.var_lover_func_dn5))), (((4.0 * locals.var_lover_func_dn6) * assign74450_e113994) + (assign74450_e113991 * (0.1 * locals.var_lover_func_dn6))), (((4.0 * locals.var_lover_func_dn7) * assign74450_e113994) + (assign74450_e113991 * (0.1 * locals.var_lover_func_dn7))), (((4.0 * locals.var_lover_func_dn8) * assign74450_e113994) + (assign74450_e113991 * (0.1 * locals.var_lover_func_dn8))), (((4.0 * locals.var_lover_func_dn9) * assign74450_e113994) + (assign74450_e113991 * (0.1 * locals.var_lover_func_dn9))), (((4.0 * locals.var_lover_func_dn10) * assign74450_e113994) + (assign74450_e113991 * (0.1 * locals.var_lover_func_dn10))), (((4.0 * locals.var_lover_func_dn13) * assign74450_e113994) + (assign74450_e113991 * (0.1 * locals.var_lover_func_dn13))),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign74450_e113997;
        locals.var_tmf2_dn0 = assign74450_e113997_d_n0;
        locals.var_tmf2_dn2 = assign74450_e113997_d_n2;
        locals.var_tmf2_dn4 = assign74450_e113997_d_n4;
        locals.var_tmf2_dn5 = assign74450_e113997_d_n5;
        locals.var_tmf2_dn6 = assign74450_e113997_d_n6;
        locals.var_tmf2_dn7 = assign74450_e113997_d_n7;
        locals.var_tmf2_dn8 = assign74450_e113997_d_n8;
        locals.var_tmf2_dn9 = assign74450_e113997_d_n9;
        locals.var_tmf2_dn10 = assign74450_e113997_d_n10;
        locals.var_tmf2_dn13 = assign74450_e113997_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign74460_e114011, assign74460_e114011_d_n0, assign74460_e114011_d_n2, assign74460_e114011_d_n4, assign74460_e114011_d_n5, assign74460_e114011_d_n6, assign74460_e114011_d_n7, assign74460_e114011_d_n8, assign74460_e114011_d_n9, assign74460_e114011_d_n10, assign74460_e114011_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1737 != 0.0)) && (locals.var_guard1738 != 0.0)) {
        let (assign74460_e114009, assign74460_e114009_d_n0, assign74460_e114009_d_n2, assign74460_e114009_d_n4, assign74460_e114009_d_n5, assign74460_e114009_d_n6, assign74460_e114009_d_n7, assign74460_e114009_d_n8, assign74460_e114009_d_n9, assign74460_e114009_d_n10, assign74460_e114009_d_n13,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
            } else {
                let assign74460_e114008: f64 = (-locals.var_tmf2);
                (assign74460_e114008, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn13),)
            }
        };
        (assign74460_e114009, assign74460_e114009_d_n0, assign74460_e114009_d_n2, assign74460_e114009_d_n4, assign74460_e114009_d_n5, assign74460_e114009_d_n6, assign74460_e114009_d_n7, assign74460_e114009_d_n8, assign74460_e114009_d_n9, assign74460_e114009_d_n10, assign74460_e114009_d_n13,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign74460_e114011;
        locals.var_tmf2_dn0 = assign74460_e114011_d_n0;
        locals.var_tmf2_dn2 = assign74460_e114011_d_n2;
        locals.var_tmf2_dn4 = assign74460_e114011_d_n4;
        locals.var_tmf2_dn5 = assign74460_e114011_d_n5;
        locals.var_tmf2_dn6 = assign74460_e114011_d_n6;
        locals.var_tmf2_dn7 = assign74460_e114011_d_n7;
        locals.var_tmf2_dn8 = assign74460_e114011_d_n8;
        locals.var_tmf2_dn9 = assign74460_e114011_d_n9;
        locals.var_tmf2_dn10 = assign74460_e114011_d_n10;
        locals.var_tmf2_dn13 = assign74460_e114011_d_n13;
        locals.var_tmf2_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_271(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign74470_e114024, assign74470_e114024_d_n0, assign74470_e114024_d_n2, assign74470_e114024_d_n4, assign74470_e114024_d_n5, assign74470_e114024_d_n6, assign74470_e114024_d_n7, assign74470_e114024_d_n8, assign74470_e114024_d_n9, assign74470_e114024_d_n10, assign74470_e114024_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1737 != 0.0)) && (locals.var_guard1738 != 0.0)) {
        let assign74470_e114019: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign74470_e114021: f64 = (assign74470_e114019 + locals.var_tmf2);
        let assign74470_e114022: f64 = (assign74470_e114021).sqrt();
        (assign74470_e114022, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign74470_e114022)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign74470_e114022)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign74470_e114022)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign74470_e114022)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign74470_e114022)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign74470_e114022)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign74470_e114022)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign74470_e114022)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign74470_e114022)), ((((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)) + locals.var_tmf2_dn13) / (2.0 * assign74470_e114022)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign74470_e114024;
        locals.var_tmf2_dn0 = assign74470_e114024_d_n0;
        locals.var_tmf2_dn2 = assign74470_e114024_d_n2;
        locals.var_tmf2_dn4 = assign74470_e114024_d_n4;
        locals.var_tmf2_dn5 = assign74470_e114024_d_n5;
        locals.var_tmf2_dn6 = assign74470_e114024_d_n6;
        locals.var_tmf2_dn7 = assign74470_e114024_d_n7;
        locals.var_tmf2_dn8 = assign74470_e114024_d_n8;
        locals.var_tmf2_dn9 = assign74470_e114024_d_n9;
        locals.var_tmf2_dn10 = assign74470_e114024_d_n10;
        locals.var_tmf2_dn13 = assign74470_e114024_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign74480_e114038, assign74480_e114038_d_n0, assign74480_e114038_d_n2, assign74480_e114038_d_n4, assign74480_e114038_d_n5, assign74480_e114038_d_n6, assign74480_e114038_d_n7, assign74480_e114038_d_n8, assign74480_e114038_d_n9, assign74480_e114038_d_n10, assign74480_e114038_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1737 != 0.0)) && (locals.var_guard1738 != 0.0)) {
        let assign74480_e114034: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign74480_e114035: f64 = (1.0 + assign74480_e114034);
        let assign74480_e114036: f64 = (0.5 * assign74480_e114035);
        (assign74480_e114036, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn13 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign74480_e114038;
        locals.var_t0_dn0 = assign74480_e114038_d_n0;
        locals.var_t0_dn2 = assign74480_e114038_d_n2;
        locals.var_t0_dn4 = assign74480_e114038_d_n4;
        locals.var_t0_dn5 = assign74480_e114038_d_n5;
        locals.var_t0_dn6 = assign74480_e114038_d_n6;
        locals.var_t0_dn7 = assign74480_e114038_d_n7;
        locals.var_t0_dn8 = assign74480_e114038_d_n8;
        locals.var_t0_dn9 = assign74480_e114038_d_n9;
        locals.var_t0_dn10 = assign74480_e114038_d_n10;
        locals.var_t0_dn13 = assign74480_e114038_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign74490_e114052, assign74490_e114052_d_n0, assign74490_e114052_d_n2, assign74490_e114052_d_n4, assign74490_e114052_d_n5, assign74490_e114052_d_n6, assign74490_e114052_d_n7, assign74490_e114052_d_n8, assign74490_e114052_d_n9, assign74490_e114052_d_n10, assign74490_e114052_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1737 != 0.0)) && (locals.var_guard1738 != 0.0)) {
        let assign74490_e114048: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign74490_e114049: f64 = (0.5 * assign74490_e114048);
        let assign74490_e114050: f64 = (locals.var_lover_func - assign74490_e114049);
        (assign74490_e114050, (locals.var_lover_func_dn0 - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (locals.var_lover_func_dn2 - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (locals.var_lover_func_dn4 - (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), (locals.var_lover_func_dn5 - (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), (locals.var_lover_func_dn6 - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (locals.var_lover_func_dn7 - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (locals.var_lover_func_dn8 - (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), (locals.var_lover_func_dn9 - (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), (locals.var_lover_func_dn10 - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (locals.var_lover_func_dn13 - (0.5 * (locals.var_tmf1_dn13 + locals.var_tmf2_dn13))),)
    } else {
        (locals.var_wjuncld, locals.var_wjuncld_dn0, locals.var_wjuncld_dn2, locals.var_wjuncld_dn4, locals.var_wjuncld_dn5, locals.var_wjuncld_dn6, locals.var_wjuncld_dn7, locals.var_wjuncld_dn8, locals.var_wjuncld_dn9, locals.var_wjuncld_dn10, locals.var_wjuncld_dn13,)
    }
};
        locals.var_wjuncld = assign74490_e114052;
        locals.var_wjuncld_dn0 = assign74490_e114052_d_n0;
        locals.var_wjuncld_dn2 = assign74490_e114052_d_n2;
        locals.var_wjuncld_dn4 = assign74490_e114052_d_n4;
        locals.var_wjuncld_dn5 = assign74490_e114052_d_n5;
        locals.var_wjuncld_dn6 = assign74490_e114052_d_n6;
        locals.var_wjuncld_dn7 = assign74490_e114052_d_n7;
        locals.var_wjuncld_dn8 = assign74490_e114052_d_n8;
        locals.var_wjuncld_dn9 = assign74490_e114052_d_n9;
        locals.var_wjuncld_dn10 = assign74490_e114052_d_n10;
        locals.var_wjuncld_dn13 = assign74490_e114052_d_n13;
        locals.var_wjuncld_rv = 0.0;

        let (assign74500_e114062, assign74500_e114062_d_n0, assign74500_e114062_d_n2, assign74500_e114062_d_n4, assign74500_e114062_d_n5, assign74500_e114062_d_n6, assign74500_e114062_d_n7, assign74500_e114062_d_n8, assign74500_e114062_d_n9, assign74500_e114062_d_n10, assign74500_e114062_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1737 != 0.0)) && (locals.var_guard1738 != 0.0)) {
        let assign74500_e114060: f64 = (locals.var_lover_func - locals.var_wjuncld);
        (assign74500_e114060, (locals.var_lover_func_dn0 - locals.var_wjuncld_dn0), (locals.var_lover_func_dn2 - locals.var_wjuncld_dn2), (locals.var_lover_func_dn4 - locals.var_wjuncld_dn4), (locals.var_lover_func_dn5 - locals.var_wjuncld_dn5), (locals.var_lover_func_dn6 - locals.var_wjuncld_dn6), (locals.var_lover_func_dn7 - locals.var_wjuncld_dn7), (locals.var_lover_func_dn8 - locals.var_wjuncld_dn8), (locals.var_lover_func_dn9 - locals.var_wjuncld_dn9), (locals.var_lover_func_dn10 - locals.var_wjuncld_dn10), (locals.var_lover_func_dn13 - locals.var_wjuncld_dn13),)
    } else {
        (locals.var_lover_func, locals.var_lover_func_dn0, locals.var_lover_func_dn2, locals.var_lover_func_dn4, locals.var_lover_func_dn5, locals.var_lover_func_dn6, locals.var_lover_func_dn7, locals.var_lover_func_dn8, locals.var_lover_func_dn9, locals.var_lover_func_dn10, locals.var_lover_func_dn13,)
    }
};
        locals.var_lover_func = assign74500_e114062;
        locals.var_lover_func_dn0 = assign74500_e114062_d_n0;
        locals.var_lover_func_dn2 = assign74500_e114062_d_n2;
        locals.var_lover_func_dn4 = assign74500_e114062_d_n4;
        locals.var_lover_func_dn5 = assign74500_e114062_d_n5;
        locals.var_lover_func_dn6 = assign74500_e114062_d_n6;
        locals.var_lover_func_dn7 = assign74500_e114062_d_n7;
        locals.var_lover_func_dn8 = assign74500_e114062_d_n8;
        locals.var_lover_func_dn9 = assign74500_e114062_d_n9;
        locals.var_lover_func_dn10 = assign74500_e114062_d_n10;
        locals.var_lover_func_dn13 = assign74500_e114062_d_n13;
        locals.var_lover_func_rv = 0.0;

        let assign74510_e114065: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1741 = assign74510_e114065;
        locals.var_guard1741_rv = 0.0;

        let assign74520_e114068: f64 = if 1.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1742 = assign74520_e114068;
        locals.var_guard1742_rv = 0.0;

        let assign74530_e114071: f64 = if 1.0 == 3.0 { 1.0 } else { 0.0 };
        locals.var_guard1743 = assign74530_e114071;
        locals.var_guard1743_rv = 0.0;

        let assign74540_e114074: f64 = if 1.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard1744 = assign74540_e114074;
        locals.var_guard1744_rv = 0.0;

        let assign74550_e114077: f64 = if p.p55 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1745 = assign74550_e114077;
        locals.var_guard1745_rv = 0.0;

        let (assign74560_e114087, assign74560_e114087_d_n0, assign74560_e114087_d_n2, assign74560_e114087_d_n4, assign74560_e114087_d_n5, assign74560_e114087_d_n6, assign74560_e114087_d_n7, assign74560_e114087_d_n8, assign74560_e114087_d_n9, assign74560_e114087_d_n10, assign74560_e114087_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1741 != 0.0)) && (locals.var_guard1745 != 0.0)) {
        let assign74560_e114085: f64 = (locals.var_weffcv_nf * locals.var_lover_func);
        (assign74560_e114085, (locals.var_weffcv_nf * locals.var_lover_func_dn0), (locals.var_weffcv_nf * locals.var_lover_func_dn2), (locals.var_weffcv_nf * locals.var_lover_func_dn4), (locals.var_weffcv_nf * locals.var_lover_func_dn5), (locals.var_weffcv_nf * locals.var_lover_func_dn6), (locals.var_weffcv_nf * locals.var_lover_func_dn7), (locals.var_weffcv_nf * locals.var_lover_func_dn8), (locals.var_weffcv_nf * locals.var_lover_func_dn9), (locals.var_weffcv_nf * locals.var_lover_func_dn10), (locals.var_weffcv_nf * locals.var_lover_func_dn13),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign74560_e114087;
        locals.var_t4_dn0 = assign74560_e114087_d_n0;
        locals.var_t4_dn2 = assign74560_e114087_d_n2;
        locals.var_t4_dn4 = assign74560_e114087_d_n4;
        locals.var_t4_dn5 = assign74560_e114087_d_n5;
        locals.var_t4_dn6 = assign74560_e114087_d_n6;
        locals.var_t4_dn7 = assign74560_e114087_d_n7;
        locals.var_t4_dn8 = assign74560_e114087_d_n8;
        locals.var_t4_dn9 = assign74560_e114087_d_n9;
        locals.var_t4_dn10 = assign74560_e114087_d_n10;
        locals.var_t4_dn13 = assign74560_e114087_d_n13;
        locals.var_t4_rv = 0.0;

        let (assign74570_e114102, assign74570_e114102_d_n0, assign74570_e114102_d_n2, assign74570_e114102_d_n4, assign74570_e114102_d_n5, assign74570_e114102_d_n6, assign74570_e114102_d_n7, assign74570_e114102_d_n8, assign74570_e114102_d_n9, assign74570_e114102_d_n10, assign74570_e114102_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1741 != 0.0)) && (locals.var_guard1745 == 0.0)) {
        let assign74570_e114096: f64 = (locals.var_weffcv_nf * locals.var_lover_func);
        let assign74570_e114099: f64 = (1.0 - locals.var_uc_cvdsover);
        let assign74570_e114100: f64 = (assign74570_e114096 * assign74570_e114099);
        (assign74570_e114100, ((locals.var_weffcv_nf * locals.var_lover_func_dn0) * assign74570_e114099), ((locals.var_weffcv_nf * locals.var_lover_func_dn2) * assign74570_e114099), ((locals.var_weffcv_nf * locals.var_lover_func_dn4) * assign74570_e114099), ((locals.var_weffcv_nf * locals.var_lover_func_dn5) * assign74570_e114099), ((locals.var_weffcv_nf * locals.var_lover_func_dn6) * assign74570_e114099), ((locals.var_weffcv_nf * locals.var_lover_func_dn7) * assign74570_e114099), ((locals.var_weffcv_nf * locals.var_lover_func_dn8) * assign74570_e114099), ((locals.var_weffcv_nf * locals.var_lover_func_dn9) * assign74570_e114099), ((locals.var_weffcv_nf * locals.var_lover_func_dn10) * assign74570_e114099), ((locals.var_weffcv_nf * locals.var_lover_func_dn13) * assign74570_e114099),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign74570_e114102;
        locals.var_t4_dn0 = assign74570_e114102_d_n0;
        locals.var_t4_dn2 = assign74570_e114102_d_n2;
        locals.var_t4_dn4 = assign74570_e114102_d_n4;
        locals.var_t4_dn5 = assign74570_e114102_d_n5;
        locals.var_t4_dn6 = assign74570_e114102_d_n6;
        locals.var_t4_dn7 = assign74570_e114102_d_n7;
        locals.var_t4_dn8 = assign74570_e114102_d_n8;
        locals.var_t4_dn9 = assign74570_e114102_d_n9;
        locals.var_t4_dn10 = assign74570_e114102_d_n10;
        locals.var_t4_dn13 = assign74570_e114102_d_n13;
        locals.var_t4_rv = 0.0;

        let (assign74580_e114110, assign74580_e114110_d_n0, assign74580_e114110_d_n2, assign74580_e114110_d_n4, assign74580_e114110_d_n5, assign74580_e114110_d_n6, assign74580_e114110_d_n7, assign74580_e114110_d_n8, assign74580_e114110_d_n9, assign74580_e114110_d_n10, assign74580_e114110_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1741 != 0.0)) {
        let assign74580_e114108: f64 = (locals.var_t4 * locals.var_qsuld);
        (assign74580_e114108, ((locals.var_t4_dn0 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn0)), ((locals.var_t4_dn2 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn2)), ((locals.var_t4_dn4 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn4)), ((locals.var_t4_dn5 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn5)), ((locals.var_t4_dn6 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn6)), ((locals.var_t4_dn7 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn7)), ((locals.var_t4_dn8 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn8)), ((locals.var_t4_dn9 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn9)), ((locals.var_t4_dn10 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn10)), ((locals.var_t4_dn13 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn13)),)
    } else {
        (locals.var_qovs, locals.var_qovs_dn0, locals.var_qovs_dn2, locals.var_qovs_dn4, locals.var_qovs_dn5, locals.var_qovs_dn6, locals.var_qovs_dn7, locals.var_qovs_dn8, locals.var_qovs_dn9, locals.var_qovs_dn10, locals.var_qovs_dn13,)
    }
};
        locals.var_qovs = assign74580_e114110;
        locals.var_qovs_dn0 = assign74580_e114110_d_n0;
        locals.var_qovs_dn2 = assign74580_e114110_d_n2;
        locals.var_qovs_dn4 = assign74580_e114110_d_n4;
        locals.var_qovs_dn5 = assign74580_e114110_d_n5;
        locals.var_qovs_dn6 = assign74580_e114110_d_n6;
        locals.var_qovs_dn7 = assign74580_e114110_d_n7;
        locals.var_qovs_dn8 = assign74580_e114110_d_n8;
        locals.var_qovs_dn9 = assign74580_e114110_d_n9;
        locals.var_qovs_dn10 = assign74580_e114110_d_n10;
        locals.var_qovs_dn13 = assign74580_e114110_d_n13;
        locals.var_qovs_rv = 0.0;

        let (assign74590_e114118, assign74590_e114118_d_n0, assign74590_e114118_d_n2, assign74590_e114118_d_n4, assign74590_e114118_d_n5, assign74590_e114118_d_n6, assign74590_e114118_d_n7, assign74590_e114118_d_n8, assign74590_e114118_d_n9, assign74590_e114118_d_n10, assign74590_e114118_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1741 != 0.0)) {
        let assign74590_e114116: f64 = (locals.var_t4 * locals.var_qbuld);
        (assign74590_e114116, ((locals.var_t4_dn0 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn0)), ((locals.var_t4_dn2 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn2)), ((locals.var_t4_dn4 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn4)), ((locals.var_t4_dn5 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn5)), ((locals.var_t4_dn6 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn6)), ((locals.var_t4_dn7 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn7)), ((locals.var_t4_dn8 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn8)), ((locals.var_t4_dn9 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn9)), ((locals.var_t4_dn10 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn10)), ((locals.var_t4_dn13 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn13)),)
    } else {
        (locals.var_qbsld, locals.var_qbsld_dn0, locals.var_qbsld_dn2, locals.var_qbsld_dn4, locals.var_qbsld_dn5, locals.var_qbsld_dn6, locals.var_qbsld_dn7, locals.var_qbsld_dn8, locals.var_qbsld_dn9, locals.var_qbsld_dn10, locals.var_qbsld_dn13,)
    }
};
        locals.var_qbsld = assign74590_e114118;
        locals.var_qbsld_dn0 = assign74590_e114118_d_n0;
        locals.var_qbsld_dn2 = assign74590_e114118_d_n2;
        locals.var_qbsld_dn4 = assign74590_e114118_d_n4;
        locals.var_qbsld_dn5 = assign74590_e114118_d_n5;
        locals.var_qbsld_dn6 = assign74590_e114118_d_n6;
        locals.var_qbsld_dn7 = assign74590_e114118_d_n7;
        locals.var_qbsld_dn8 = assign74590_e114118_d_n8;
        locals.var_qbsld_dn9 = assign74590_e114118_d_n9;
        locals.var_qbsld_dn10 = assign74590_e114118_d_n10;
        locals.var_qbsld_dn13 = assign74590_e114118_d_n13;
        locals.var_qbsld_rv = 0.0;

        let (assign74620_e114143, assign74620_e114143_d_n0, assign74620_e114143_d_n2, assign74620_e114143_d_n4, assign74620_e114143_d_n5, assign74620_e114143_d_n6, assign74620_e114143_d_n7, assign74620_e114143_d_n8, assign74620_e114143_d_n9, assign74620_e114143_d_n10, assign74620_e114143_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && ((locals.var_guard1742 != 0.0) && (locals.var_guard1741 == 0.0))) {
        let assign74620_e114139: f64 = (locals.var_weffcv_nf * locals.var_lover_func);
        let assign74620_e114141: f64 = (assign74620_e114139 * locals.var_uc_cvdsover);
        (assign74620_e114141, ((locals.var_weffcv_nf * locals.var_lover_func_dn0) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn2) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn4) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn5) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn6) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn7) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn8) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn9) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn10) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn13) * locals.var_uc_cvdsover),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign74620_e114143;
        locals.var_t4_dn0 = assign74620_e114143_d_n0;
        locals.var_t4_dn2 = assign74620_e114143_d_n2;
        locals.var_t4_dn4 = assign74620_e114143_d_n4;
        locals.var_t4_dn5 = assign74620_e114143_d_n5;
        locals.var_t4_dn6 = assign74620_e114143_d_n6;
        locals.var_t4_dn7 = assign74620_e114143_d_n7;
        locals.var_t4_dn8 = assign74620_e114143_d_n8;
        locals.var_t4_dn9 = assign74620_e114143_d_n9;
        locals.var_t4_dn10 = assign74620_e114143_d_n10;
        locals.var_t4_dn13 = assign74620_e114143_d_n13;
        locals.var_t4_rv = 0.0;

        let (assign74630_e114154, assign74630_e114154_d_n0, assign74630_e114154_d_n2, assign74630_e114154_d_n4, assign74630_e114154_d_n5, assign74630_e114154_d_n6, assign74630_e114154_d_n7, assign74630_e114154_d_n8, assign74630_e114154_d_n9, assign74630_e114154_d_n10, assign74630_e114154_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && ((locals.var_guard1742 != 0.0) && (locals.var_guard1741 == 0.0))) {
        let assign74630_e114152: f64 = (locals.var_t4 * locals.var_qsuld);
        (assign74630_e114152, ((locals.var_t4_dn0 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn0)), ((locals.var_t4_dn2 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn2)), ((locals.var_t4_dn4 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn4)), ((locals.var_t4_dn5 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn5)), ((locals.var_t4_dn6 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn6)), ((locals.var_t4_dn7 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn7)), ((locals.var_t4_dn8 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn8)), ((locals.var_t4_dn9 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn9)), ((locals.var_t4_dn10 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn10)), ((locals.var_t4_dn13 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn13)),)
    } else {
        (locals.var_qovsext, locals.var_qovsext_dn0, locals.var_qovsext_dn2, locals.var_qovsext_dn4, locals.var_qovsext_dn5, locals.var_qovsext_dn6, locals.var_qovsext_dn7, locals.var_qovsext_dn8, locals.var_qovsext_dn9, locals.var_qovsext_dn10, locals.var_qovsext_dn13,)
    }
};
        locals.var_qovsext = assign74630_e114154;
        locals.var_qovsext_dn0 = assign74630_e114154_d_n0;
        locals.var_qovsext_dn2 = assign74630_e114154_d_n2;
        locals.var_qovsext_dn4 = assign74630_e114154_d_n4;
        locals.var_qovsext_dn5 = assign74630_e114154_d_n5;
        locals.var_qovsext_dn6 = assign74630_e114154_d_n6;
        locals.var_qovsext_dn7 = assign74630_e114154_d_n7;
        locals.var_qovsext_dn8 = assign74630_e114154_d_n8;
        locals.var_qovsext_dn9 = assign74630_e114154_d_n9;
        locals.var_qovsext_dn10 = assign74630_e114154_d_n10;
        locals.var_qovsext_dn13 = assign74630_e114154_d_n13;
        locals.var_qovsext_rv = 0.0;

        let (assign74640_e114165, assign74640_e114165_d_n0, assign74640_e114165_d_n2, assign74640_e114165_d_n4, assign74640_e114165_d_n5, assign74640_e114165_d_n6, assign74640_e114165_d_n7, assign74640_e114165_d_n8, assign74640_e114165_d_n9, assign74640_e114165_d_n10, assign74640_e114165_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && ((locals.var_guard1742 != 0.0) && (locals.var_guard1741 == 0.0))) {
        let assign74640_e114163: f64 = (locals.var_t4 * locals.var_qbuld);
        (assign74640_e114163, ((locals.var_t4_dn0 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn0)), ((locals.var_t4_dn2 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn2)), ((locals.var_t4_dn4 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn4)), ((locals.var_t4_dn5 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn5)), ((locals.var_t4_dn6 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn6)), ((locals.var_t4_dn7 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn7)), ((locals.var_t4_dn8 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn8)), ((locals.var_t4_dn9 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn9)), ((locals.var_t4_dn10 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn10)), ((locals.var_t4_dn13 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn13)),)
    } else {
        (locals.var_qbsldext, locals.var_qbsldext_dn0, locals.var_qbsldext_dn2, locals.var_qbsldext_dn4, locals.var_qbsldext_dn5, locals.var_qbsldext_dn6, locals.var_qbsldext_dn7, locals.var_qbsldext_dn8, locals.var_qbsldext_dn9, locals.var_qbsldext_dn10, locals.var_qbsldext_dn13,)
    }
};
        locals.var_qbsldext = assign74640_e114165;
        locals.var_qbsldext_dn0 = assign74640_e114165_d_n0;
        locals.var_qbsldext_dn2 = assign74640_e114165_d_n2;
        locals.var_qbsldext_dn4 = assign74640_e114165_d_n4;
        locals.var_qbsldext_dn5 = assign74640_e114165_d_n5;
        locals.var_qbsldext_dn6 = assign74640_e114165_d_n6;
        locals.var_qbsldext_dn7 = assign74640_e114165_d_n7;
        locals.var_qbsldext_dn8 = assign74640_e114165_d_n8;
        locals.var_qbsldext_dn9 = assign74640_e114165_d_n9;
        locals.var_qbsldext_dn10 = assign74640_e114165_d_n10;
        locals.var_qbsldext_dn13 = assign74640_e114165_d_n13;
        locals.var_qbsldext_rv = 0.0;

        let assign74650_e114168: f64 = if p.p55 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1746 = assign74650_e114168;
        locals.var_guard1746_rv = 0.0;

        let (assign74660_e114183, assign74660_e114183_d_n0, assign74660_e114183_d_n2, assign74660_e114183_d_n4, assign74660_e114183_d_n5, assign74660_e114183_d_n6, assign74660_e114183_d_n7, assign74660_e114183_d_n8, assign74660_e114183_d_n9, assign74660_e114183_d_n10, assign74660_e114183_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && ((locals.var_guard1743 != 0.0) && (!((locals.var_guard1741 != 0.0) || (locals.var_guard1742 != 0.0))))) && (locals.var_guard1746 != 0.0)) {
        let assign74660_e114181: f64 = (locals.var_weffcv_nf * locals.var_lover_func);
        (assign74660_e114181, (locals.var_weffcv_nf * locals.var_lover_func_dn0), (locals.var_weffcv_nf * locals.var_lover_func_dn2), (locals.var_weffcv_nf * locals.var_lover_func_dn4), (locals.var_weffcv_nf * locals.var_lover_func_dn5), (locals.var_weffcv_nf * locals.var_lover_func_dn6), (locals.var_weffcv_nf * locals.var_lover_func_dn7), (locals.var_weffcv_nf * locals.var_lover_func_dn8), (locals.var_weffcv_nf * locals.var_lover_func_dn9), (locals.var_weffcv_nf * locals.var_lover_func_dn10), (locals.var_weffcv_nf * locals.var_lover_func_dn13),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign74660_e114183;
        locals.var_t4_dn0 = assign74660_e114183_d_n0;
        locals.var_t4_dn2 = assign74660_e114183_d_n2;
        locals.var_t4_dn4 = assign74660_e114183_d_n4;
        locals.var_t4_dn5 = assign74660_e114183_d_n5;
        locals.var_t4_dn6 = assign74660_e114183_d_n6;
        locals.var_t4_dn7 = assign74660_e114183_d_n7;
        locals.var_t4_dn8 = assign74660_e114183_d_n8;
        locals.var_t4_dn9 = assign74660_e114183_d_n9;
        locals.var_t4_dn10 = assign74660_e114183_d_n10;
        locals.var_t4_dn13 = assign74660_e114183_d_n13;
        locals.var_t4_rv = 0.0;

        let (assign74670_e114203, assign74670_e114203_d_n0, assign74670_e114203_d_n2, assign74670_e114203_d_n4, assign74670_e114203_d_n5, assign74670_e114203_d_n6, assign74670_e114203_d_n7, assign74670_e114203_d_n8, assign74670_e114203_d_n9, assign74670_e114203_d_n10, assign74670_e114203_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && ((locals.var_guard1743 != 0.0) && (!((locals.var_guard1741 != 0.0) || (locals.var_guard1742 != 0.0))))) && (locals.var_guard1746 == 0.0)) {
        let assign74670_e114197: f64 = (locals.var_weffcv_nf * locals.var_lover_func);
        let assign74670_e114200: f64 = (1.0 - locals.var_uc_cvdsover);
        let assign74670_e114201: f64 = (assign74670_e114197 * assign74670_e114200);
        (assign74670_e114201, ((locals.var_weffcv_nf * locals.var_lover_func_dn0) * assign74670_e114200), ((locals.var_weffcv_nf * locals.var_lover_func_dn2) * assign74670_e114200), ((locals.var_weffcv_nf * locals.var_lover_func_dn4) * assign74670_e114200), ((locals.var_weffcv_nf * locals.var_lover_func_dn5) * assign74670_e114200), ((locals.var_weffcv_nf * locals.var_lover_func_dn6) * assign74670_e114200), ((locals.var_weffcv_nf * locals.var_lover_func_dn7) * assign74670_e114200), ((locals.var_weffcv_nf * locals.var_lover_func_dn8) * assign74670_e114200), ((locals.var_weffcv_nf * locals.var_lover_func_dn9) * assign74670_e114200), ((locals.var_weffcv_nf * locals.var_lover_func_dn10) * assign74670_e114200), ((locals.var_weffcv_nf * locals.var_lover_func_dn13) * assign74670_e114200),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign74670_e114203;
        locals.var_t4_dn0 = assign74670_e114203_d_n0;
        locals.var_t4_dn2 = assign74670_e114203_d_n2;
        locals.var_t4_dn4 = assign74670_e114203_d_n4;
        locals.var_t4_dn5 = assign74670_e114203_d_n5;
        locals.var_t4_dn6 = assign74670_e114203_d_n6;
        locals.var_t4_dn7 = assign74670_e114203_d_n7;
        locals.var_t4_dn8 = assign74670_e114203_d_n8;
        locals.var_t4_dn9 = assign74670_e114203_d_n9;
        locals.var_t4_dn10 = assign74670_e114203_d_n10;
        locals.var_t4_dn13 = assign74670_e114203_d_n13;
        locals.var_t4_rv = 0.0;

        let (assign74680_e114214, assign74680_e114214_d_n0, assign74680_e114214_d_n2, assign74680_e114214_d_n4, assign74680_e114214_d_n5, assign74680_e114214_d_n6, assign74680_e114214_d_n7, assign74680_e114214_d_n8, assign74680_e114214_d_n9, assign74680_e114214_d_n10, assign74680_e114214_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && ((locals.var_guard1743 != 0.0) && (!((locals.var_guard1741 != 0.0) || (locals.var_guard1742 != 0.0))))) {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn13,)
    } else {
        (locals.var_rd_ps0ld, locals.var_rd_ps0ld_dn0, locals.var_rd_ps0ld_dn2, locals.var_rd_ps0ld_dn4, locals.var_rd_ps0ld_dn5, locals.var_rd_ps0ld_dn6, locals.var_rd_ps0ld_dn7, locals.var_rd_ps0ld_dn8, locals.var_rd_ps0ld_dn9, locals.var_rd_ps0ld_dn10, locals.var_rd_ps0ld_dn13,)
    }
};
        locals.var_rd_ps0ld = assign74680_e114214;
        locals.var_rd_ps0ld_dn0 = assign74680_e114214_d_n0;
        locals.var_rd_ps0ld_dn2 = assign74680_e114214_d_n2;
        locals.var_rd_ps0ld_dn4 = assign74680_e114214_d_n4;
        locals.var_rd_ps0ld_dn5 = assign74680_e114214_d_n5;
        locals.var_rd_ps0ld_dn6 = assign74680_e114214_d_n6;
        locals.var_rd_ps0ld_dn7 = assign74680_e114214_d_n7;
        locals.var_rd_ps0ld_dn8 = assign74680_e114214_d_n8;
        locals.var_rd_ps0ld_dn9 = assign74680_e114214_d_n9;
        locals.var_rd_ps0ld_dn10 = assign74680_e114214_d_n10;
        locals.var_rd_ps0ld_dn13 = assign74680_e114214_d_n13;
        locals.var_rd_ps0ld_rv = 0.0;

        let assign74690_e114217: f64 = if p.p430 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1747 = assign74690_e114217;
        locals.var_guard1747_rv = 0.0;

        let (assign74700_e114230, assign74700_e114230_d_n0, assign74700_e114230_d_n2, assign74700_e114230_d_n4, assign74700_e114230_d_n5, assign74700_e114230_d_n6, assign74700_e114230_d_n7, assign74700_e114230_d_n8, assign74700_e114230_d_n9, assign74700_e114230_d_n10, assign74700_e114230_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && ((locals.var_guard1743 != 0.0) && (!((locals.var_guard1741 != 0.0) || (locals.var_guard1742 != 0.0))))) && (locals.var_guard1747 != 0.0)) {
        (locals.var_qbuld, locals.var_qbuld_dn0, locals.var_qbuld_dn2, locals.var_qbuld_dn4, locals.var_qbuld_dn5, locals.var_qbuld_dn6, locals.var_qbuld_dn7, locals.var_qbuld_dn8, locals.var_qbuld_dn9, locals.var_qbuld_dn10, locals.var_qbuld_dn13,)
    } else {
        (locals.var_rd_qbuld, locals.var_rd_qbuld_dn0, locals.var_rd_qbuld_dn2, locals.var_rd_qbuld_dn4, locals.var_rd_qbuld_dn5, locals.var_rd_qbuld_dn6, locals.var_rd_qbuld_dn7, locals.var_rd_qbuld_dn8, locals.var_rd_qbuld_dn9, locals.var_rd_qbuld_dn10, locals.var_rd_qbuld_dn13,)
    }
};
        locals.var_rd_qbuld = assign74700_e114230;
        locals.var_rd_qbuld_dn0 = assign74700_e114230_d_n0;
        locals.var_rd_qbuld_dn2 = assign74700_e114230_d_n2;
        locals.var_rd_qbuld_dn4 = assign74700_e114230_d_n4;
        locals.var_rd_qbuld_dn5 = assign74700_e114230_d_n5;
        locals.var_rd_qbuld_dn6 = assign74700_e114230_d_n6;
        locals.var_rd_qbuld_dn7 = assign74700_e114230_d_n7;
        locals.var_rd_qbuld_dn8 = assign74700_e114230_d_n8;
        locals.var_rd_qbuld_dn9 = assign74700_e114230_d_n9;
        locals.var_rd_qbuld_dn10 = assign74700_e114230_d_n10;
        locals.var_rd_qbuld_dn13 = assign74700_e114230_d_n13;
        locals.var_rd_qbuld_rv = 0.0;

        let (assign74710_e114243, assign74710_e114243_d_n0, assign74710_e114243_d_n2, assign74710_e114243_d_n4, assign74710_e114243_d_n5, assign74710_e114243_d_n6, assign74710_e114243_d_n7, assign74710_e114243_d_n8, assign74710_e114243_d_n9, assign74710_e114243_d_n10, assign74710_e114243_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && ((locals.var_guard1743 != 0.0) && (!((locals.var_guard1741 != 0.0) || (locals.var_guard1742 != 0.0))))) {
        let assign74710_e114241: f64 = (locals.var_t4 * locals.var_qsuld);
        (assign74710_e114241, ((locals.var_t4_dn0 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn0)), ((locals.var_t4_dn2 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn2)), ((locals.var_t4_dn4 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn4)), ((locals.var_t4_dn5 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn5)), ((locals.var_t4_dn6 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn6)), ((locals.var_t4_dn7 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn7)), ((locals.var_t4_dn8 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn8)), ((locals.var_t4_dn9 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn9)), ((locals.var_t4_dn10 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn10)), ((locals.var_t4_dn13 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn13)),)
    } else {
        (locals.var_qovd, locals.var_qovd_dn0, locals.var_qovd_dn2, locals.var_qovd_dn4, locals.var_qovd_dn5, locals.var_qovd_dn6, locals.var_qovd_dn7, locals.var_qovd_dn8, locals.var_qovd_dn9, locals.var_qovd_dn10, locals.var_qovd_dn13,)
    }
};
        locals.var_qovd = assign74710_e114243;
        locals.var_qovd_dn0 = assign74710_e114243_d_n0;
        locals.var_qovd_dn2 = assign74710_e114243_d_n2;
        locals.var_qovd_dn4 = assign74710_e114243_d_n4;
        locals.var_qovd_dn5 = assign74710_e114243_d_n5;
        locals.var_qovd_dn6 = assign74710_e114243_d_n6;
        locals.var_qovd_dn7 = assign74710_e114243_d_n7;
        locals.var_qovd_dn8 = assign74710_e114243_d_n8;
        locals.var_qovd_dn9 = assign74710_e114243_d_n9;
        locals.var_qovd_dn10 = assign74710_e114243_d_n10;
        locals.var_qovd_dn13 = assign74710_e114243_d_n13;
        locals.var_qovd_rv = 0.0;

        let (assign74720_e114256, assign74720_e114256_d_n0, assign74720_e114256_d_n2, assign74720_e114256_d_n4, assign74720_e114256_d_n5, assign74720_e114256_d_n6, assign74720_e114256_d_n7, assign74720_e114256_d_n8, assign74720_e114256_d_n9, assign74720_e114256_d_n10, assign74720_e114256_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && ((locals.var_guard1743 != 0.0) && (!((locals.var_guard1741 != 0.0) || (locals.var_guard1742 != 0.0))))) {
        let assign74720_e114254: f64 = (locals.var_t4 * locals.var_qbuld);
        (assign74720_e114254, ((locals.var_t4_dn0 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn0)), ((locals.var_t4_dn2 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn2)), ((locals.var_t4_dn4 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn4)), ((locals.var_t4_dn5 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn5)), ((locals.var_t4_dn6 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn6)), ((locals.var_t4_dn7 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn7)), ((locals.var_t4_dn8 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn8)), ((locals.var_t4_dn9 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn9)), ((locals.var_t4_dn10 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn10)), ((locals.var_t4_dn13 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn13)),)
    } else {
        (locals.var_qbdld, locals.var_qbdld_dn0, locals.var_qbdld_dn2, locals.var_qbdld_dn4, locals.var_qbdld_dn5, locals.var_qbdld_dn6, locals.var_qbdld_dn7, locals.var_qbdld_dn8, locals.var_qbdld_dn9, locals.var_qbdld_dn10, locals.var_qbdld_dn13,)
    }
};
        locals.var_qbdld = assign74720_e114256;
        locals.var_qbdld_dn0 = assign74720_e114256_d_n0;
        locals.var_qbdld_dn2 = assign74720_e114256_d_n2;
        locals.var_qbdld_dn4 = assign74720_e114256_d_n4;
        locals.var_qbdld_dn5 = assign74720_e114256_d_n5;
        locals.var_qbdld_dn6 = assign74720_e114256_d_n6;
        locals.var_qbdld_dn7 = assign74720_e114256_d_n7;
        locals.var_qbdld_dn8 = assign74720_e114256_d_n8;
        locals.var_qbdld_dn9 = assign74720_e114256_d_n9;
        locals.var_qbdld_dn10 = assign74720_e114256_d_n10;
        locals.var_qbdld_dn13 = assign74720_e114256_d_n13;
        locals.var_qbdld_rv = 0.0;

        let (assign74730_e114267, assign74730_e114267_d_n0, assign74730_e114267_d_n2, assign74730_e114267_d_n4, assign74730_e114267_d_n5, assign74730_e114267_d_n6, assign74730_e114267_d_n7, assign74730_e114267_d_n8, assign74730_e114267_d_n9, assign74730_e114267_d_n10, assign74730_e114267_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && ((locals.var_guard1743 != 0.0) && (!((locals.var_guard1741 != 0.0) || (locals.var_guard1742 != 0.0))))) {
        (locals.var_qbdld, locals.var_qbdld_dn0, locals.var_qbdld_dn2, locals.var_qbdld_dn4, locals.var_qbdld_dn5, locals.var_qbdld_dn6, locals.var_qbdld_dn7, locals.var_qbdld_dn8, locals.var_qbdld_dn9, locals.var_qbdld_dn10, locals.var_qbdld_dn13,)
    } else {
        (locals.var_qbd_qs, locals.var_qbd_qs_dn0, locals.var_qbd_qs_dn2, locals.var_qbd_qs_dn4, locals.var_qbd_qs_dn5, locals.var_qbd_qs_dn6, locals.var_qbd_qs_dn7, locals.var_qbd_qs_dn8, locals.var_qbd_qs_dn9, locals.var_qbd_qs_dn10, locals.var_qbd_qs_dn13,)
    }
};
        locals.var_qbd_qs = assign74730_e114267;
        locals.var_qbd_qs_dn0 = assign74730_e114267_d_n0;
        locals.var_qbd_qs_dn2 = assign74730_e114267_d_n2;
        locals.var_qbd_qs_dn4 = assign74730_e114267_d_n4;
        locals.var_qbd_qs_dn5 = assign74730_e114267_d_n5;
        locals.var_qbd_qs_dn6 = assign74730_e114267_d_n6;
        locals.var_qbd_qs_dn7 = assign74730_e114267_d_n7;
        locals.var_qbd_qs_dn8 = assign74730_e114267_d_n8;
        locals.var_qbd_qs_dn9 = assign74730_e114267_d_n9;
        locals.var_qbd_qs_dn10 = assign74730_e114267_d_n10;
        locals.var_qbd_qs_dn13 = assign74730_e114267_d_n13;
        locals.var_qbd_qs_rv = 0.0;

        let (assign74740_e114284, assign74740_e114284_d_n0, assign74740_e114284_d_n2, assign74740_e114284_d_n4, assign74740_e114284_d_n5, assign74740_e114284_d_n6, assign74740_e114284_d_n7, assign74740_e114284_d_n8, assign74740_e114284_d_n9, assign74740_e114284_d_n10, assign74740_e114284_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && ((locals.var_guard1744 != 0.0) && (!(((locals.var_guard1741 != 0.0) || (locals.var_guard1742 != 0.0)) || (locals.var_guard1743 != 0.0))))) {
        let assign74740_e114280: f64 = (locals.var_weffcv_nf * locals.var_lover_func);
        let assign74740_e114282: f64 = (assign74740_e114280 * locals.var_uc_cvdsover);
        (assign74740_e114282, ((locals.var_weffcv_nf * locals.var_lover_func_dn0) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn2) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn4) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn5) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn6) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn7) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn8) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn9) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn10) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn13) * locals.var_uc_cvdsover),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign74740_e114284;
        locals.var_t4_dn0 = assign74740_e114284_d_n0;
        locals.var_t4_dn2 = assign74740_e114284_d_n2;
        locals.var_t4_dn4 = assign74740_e114284_d_n4;
        locals.var_t4_dn5 = assign74740_e114284_d_n5;
        locals.var_t4_dn6 = assign74740_e114284_d_n6;
        locals.var_t4_dn7 = assign74740_e114284_d_n7;
        locals.var_t4_dn8 = assign74740_e114284_d_n8;
        locals.var_t4_dn9 = assign74740_e114284_d_n9;
        locals.var_t4_dn10 = assign74740_e114284_d_n10;
        locals.var_t4_dn13 = assign74740_e114284_d_n13;
        locals.var_t4_rv = 0.0;

        let (assign74750_e114299, assign74750_e114299_d_n0, assign74750_e114299_d_n2, assign74750_e114299_d_n4, assign74750_e114299_d_n5, assign74750_e114299_d_n6, assign74750_e114299_d_n7, assign74750_e114299_d_n8, assign74750_e114299_d_n9, assign74750_e114299_d_n10, assign74750_e114299_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && ((locals.var_guard1744 != 0.0) && (!(((locals.var_guard1741 != 0.0) || (locals.var_guard1742 != 0.0)) || (locals.var_guard1743 != 0.0))))) {
        let assign74750_e114297: f64 = (locals.var_t4 * locals.var_qsuld);
        (assign74750_e114297, ((locals.var_t4_dn0 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn0)), ((locals.var_t4_dn2 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn2)), ((locals.var_t4_dn4 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn4)), ((locals.var_t4_dn5 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn5)), ((locals.var_t4_dn6 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn6)), ((locals.var_t4_dn7 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn7)), ((locals.var_t4_dn8 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn8)), ((locals.var_t4_dn9 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn9)), ((locals.var_t4_dn10 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn10)), ((locals.var_t4_dn13 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn13)),)
    } else {
        (locals.var_qovdext, locals.var_qovdext_dn0, locals.var_qovdext_dn2, locals.var_qovdext_dn4, locals.var_qovdext_dn5, locals.var_qovdext_dn6, locals.var_qovdext_dn7, locals.var_qovdext_dn8, locals.var_qovdext_dn9, locals.var_qovdext_dn10, locals.var_qovdext_dn13,)
    }
};
        locals.var_qovdext = assign74750_e114299;
        locals.var_qovdext_dn0 = assign74750_e114299_d_n0;
        locals.var_qovdext_dn2 = assign74750_e114299_d_n2;
        locals.var_qovdext_dn4 = assign74750_e114299_d_n4;
        locals.var_qovdext_dn5 = assign74750_e114299_d_n5;
        locals.var_qovdext_dn6 = assign74750_e114299_d_n6;
        locals.var_qovdext_dn7 = assign74750_e114299_d_n7;
        locals.var_qovdext_dn8 = assign74750_e114299_d_n8;
        locals.var_qovdext_dn9 = assign74750_e114299_d_n9;
        locals.var_qovdext_dn10 = assign74750_e114299_d_n10;
        locals.var_qovdext_dn13 = assign74750_e114299_d_n13;
        locals.var_qovdext_rv = 0.0;

        let (assign74760_e114314, assign74760_e114314_d_n0, assign74760_e114314_d_n2, assign74760_e114314_d_n4, assign74760_e114314_d_n5, assign74760_e114314_d_n6, assign74760_e114314_d_n7, assign74760_e114314_d_n8, assign74760_e114314_d_n9, assign74760_e114314_d_n10, assign74760_e114314_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && ((locals.var_guard1744 != 0.0) && (!(((locals.var_guard1741 != 0.0) || (locals.var_guard1742 != 0.0)) || (locals.var_guard1743 != 0.0))))) {
        let assign74760_e114312: f64 = (locals.var_t4 * locals.var_qbuld);
        (assign74760_e114312, ((locals.var_t4_dn0 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn0)), ((locals.var_t4_dn2 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn2)), ((locals.var_t4_dn4 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn4)), ((locals.var_t4_dn5 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn5)), ((locals.var_t4_dn6 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn6)), ((locals.var_t4_dn7 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn7)), ((locals.var_t4_dn8 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn8)), ((locals.var_t4_dn9 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn9)), ((locals.var_t4_dn10 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn10)), ((locals.var_t4_dn13 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn13)),)
    } else {
        (locals.var_qbdldext, locals.var_qbdldext_dn0, locals.var_qbdldext_dn2, locals.var_qbdldext_dn4, locals.var_qbdldext_dn5, locals.var_qbdldext_dn6, locals.var_qbdldext_dn7, locals.var_qbdldext_dn8, locals.var_qbdldext_dn9, locals.var_qbdldext_dn10, locals.var_qbdldext_dn13,)
    }
};
        locals.var_qbdldext = assign74760_e114314;
        locals.var_qbdldext_dn0 = assign74760_e114314_d_n0;
        locals.var_qbdldext_dn2 = assign74760_e114314_d_n2;
        locals.var_qbdldext_dn4 = assign74760_e114314_d_n4;
        locals.var_qbdldext_dn5 = assign74760_e114314_d_n5;
        locals.var_qbdldext_dn6 = assign74760_e114314_d_n6;
        locals.var_qbdldext_dn7 = assign74760_e114314_d_n7;
        locals.var_qbdldext_dn8 = assign74760_e114314_d_n8;
        locals.var_qbdldext_dn9 = assign74760_e114314_d_n9;
        locals.var_qbdldext_dn10 = assign74760_e114314_d_n10;
        locals.var_qbdldext_dn13 = assign74760_e114314_d_n13;
        locals.var_qbdldext_rv = 0.0;

        locals.var_flg_calcqover = 0.0;
        locals.var_flg_calcqover_rv = 0.0;

        let assign74780_e114318: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1748 = assign74780_e114318;
        locals.var_guard1748_rv = 0.0;

        let assign74790_e114321: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1749 = assign74790_e114321;
        locals.var_guard1749_rv = 0.0;

        let assign74800_e114324: f64 = if 2.0 == 3.0 { 1.0 } else { 0.0 };
        locals.var_guard1750 = assign74800_e114324;
        locals.var_guard1750_rv = 0.0;

        let assign74810_e114327: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard1751 = assign74810_e114327;
        locals.var_guard1751_rv = 0.0;

        let assign74820_e114338: f64 = if (((p.p36 == 1.0) && (p.p66 > 0.0)) && (locals.var_uc_novers > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1752 = assign74820_e114338;
        locals.var_guard1752_rv = 0.0;

        let (assign74830_e114344,) = {
    if ((locals.var_guard1748 != 0.0) && (locals.var_guard1752 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_calcqover,)
    }
};
        locals.var_flg_calcqover = assign74830_e114344;
        locals.var_flg_calcqover_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_272(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign74840_e114350,) = {
    if ((locals.var_guard1748 != 0.0) && (locals.var_guard1752 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_coovlps,)
    }
};
        locals.var_flg_coovlps = assign74840_e114350;
        locals.var_flg_coovlps_rv = 0.0;

        let (assign74850_e114358, assign74850_e114358_d_n2, assign74850_e114358_d_n6, assign74850_e114358_d_n7, assign74850_e114358_d_n8,) = {
    if ((locals.var_guard1748 != 0.0) && (locals.var_guard1752 != 0.0)) {
        let assign74850_e114356: f64 = (locals.var_vgsi - locals.var_vbsi);
        (assign74850_e114356, 0.0, locals.var_vgsi_dn6, (locals.var_vgsi_dn7 - locals.var_vbsi_dn7), (-locals.var_vbsi_dn8),)
    } else {
        (locals.var_vgbgmt, locals.var_vgbgmt_dn2, locals.var_vgbgmt_dn6, locals.var_vgbgmt_dn7, locals.var_vgbgmt_dn8,)
    }
};
        locals.var_vgbgmt = assign74850_e114358;
        locals.var_vgbgmt_dn2 = assign74850_e114358_d_n2;
        locals.var_vgbgmt_dn6 = assign74850_e114358_d_n6;
        locals.var_vgbgmt_dn7 = assign74850_e114358_d_n7;
        locals.var_vgbgmt_dn8 = assign74850_e114358_d_n8;
        locals.var_vgbgmt_rv = 0.0;

        let (assign74860_e114365, assign74860_e114365_d_n0, assign74860_e114365_d_n2, assign74860_e114365_d_n4, assign74860_e114365_d_n5, assign74860_e114365_d_n6, assign74860_e114365_d_n7, assign74860_e114365_d_n8, assign74860_e114365_d_n9, assign74860_e114365_d_n10, assign74860_e114365_d_n13,) = {
    if ((locals.var_guard1748 != 0.0) && (locals.var_guard1752 != 0.0)) {
        let assign74860_e114363: f64 = (-locals.var_vbsi);
        (assign74860_e114363, 0.0, 0.0, 0.0, 0.0, 0.0, (-locals.var_vbsi_dn7), (-locals.var_vbsi_dn8), 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vxbgmt, locals.var_vxbgmt_dn0, locals.var_vxbgmt_dn2, locals.var_vxbgmt_dn4, locals.var_vxbgmt_dn5, locals.var_vxbgmt_dn6, locals.var_vxbgmt_dn7, locals.var_vxbgmt_dn8, locals.var_vxbgmt_dn9, locals.var_vxbgmt_dn10, locals.var_vxbgmt_dn13,)
    }
};
        locals.var_vxbgmt = assign74860_e114365;
        locals.var_vxbgmt_dn0 = assign74860_e114365_d_n0;
        locals.var_vxbgmt_dn2 = assign74860_e114365_d_n2;
        locals.var_vxbgmt_dn4 = assign74860_e114365_d_n4;
        locals.var_vxbgmt_dn5 = assign74860_e114365_d_n5;
        locals.var_vxbgmt_dn6 = assign74860_e114365_d_n6;
        locals.var_vxbgmt_dn7 = assign74860_e114365_d_n7;
        locals.var_vxbgmt_dn8 = assign74860_e114365_d_n8;
        locals.var_vxbgmt_dn9 = assign74860_e114365_d_n9;
        locals.var_vxbgmt_dn10 = assign74860_e114365_d_n10;
        locals.var_vxbgmt_dn13 = assign74860_e114365_d_n13;
        locals.var_vxbgmt_rv = 0.0;

        let (assign74870_e114371,) = {
    if ((locals.var_guard1748 != 0.0) && (locals.var_guard1752 != 0.0)) {
        (locals.var_uc_novers,)
    } else {
        (locals.var_nover_func,)
    }
};
        locals.var_nover_func = assign74870_e114371;
        locals.var_nover_func_rv = 0.0;

        let (assign74880_e114377, assign74880_e114377_d_n0, assign74880_e114377_d_n2, assign74880_e114377_d_n4, assign74880_e114377_d_n5, assign74880_e114377_d_n6, assign74880_e114377_d_n7, assign74880_e114377_d_n8, assign74880_e114377_d_n9, assign74880_e114377_d_n10, assign74880_e114377_d_n13,) = {
    if ((locals.var_guard1748 != 0.0) && (locals.var_guard1752 != 0.0)) {
        (p.p66, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_lover_func, locals.var_lover_func_dn0, locals.var_lover_func_dn2, locals.var_lover_func_dn4, locals.var_lover_func_dn5, locals.var_lover_func_dn6, locals.var_lover_func_dn7, locals.var_lover_func_dn8, locals.var_lover_func_dn9, locals.var_lover_func_dn10, locals.var_lover_func_dn13,)
    }
};
        locals.var_lover_func = assign74880_e114377;
        locals.var_lover_func_dn0 = assign74880_e114377_d_n0;
        locals.var_lover_func_dn2 = assign74880_e114377_d_n2;
        locals.var_lover_func_dn4 = assign74880_e114377_d_n4;
        locals.var_lover_func_dn5 = assign74880_e114377_d_n5;
        locals.var_lover_func_dn6 = assign74880_e114377_d_n6;
        locals.var_lover_func_dn7 = assign74880_e114377_d_n7;
        locals.var_lover_func_dn8 = assign74880_e114377_d_n8;
        locals.var_lover_func_dn9 = assign74880_e114377_d_n9;
        locals.var_lover_func_dn10 = assign74880_e114377_d_n10;
        locals.var_lover_func_dn13 = assign74880_e114377_d_n13;
        locals.var_lover_func_rv = 0.0;

        let (assign74890_e114383, assign74890_e114383_d_n0, assign74890_e114383_d_n2, assign74890_e114383_d_n4, assign74890_e114383_d_n5, assign74890_e114383_d_n6, assign74890_e114383_d_n7, assign74890_e114383_d_n8, assign74890_e114383_d_n9, assign74890_e114383_d_n10, assign74890_e114383_d_n13,) = {
    if ((locals.var_guard1748 != 0.0) && (locals.var_guard1752 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_wdep_func, locals.var_wdep_func_dn0, locals.var_wdep_func_dn2, locals.var_wdep_func_dn4, locals.var_wdep_func_dn5, locals.var_wdep_func_dn6, locals.var_wdep_func_dn7, locals.var_wdep_func_dn8, locals.var_wdep_func_dn9, locals.var_wdep_func_dn10, locals.var_wdep_func_dn13,)
    }
};
        locals.var_wdep_func = assign74890_e114383;
        locals.var_wdep_func_dn0 = assign74890_e114383_d_n0;
        locals.var_wdep_func_dn2 = assign74890_e114383_d_n2;
        locals.var_wdep_func_dn4 = assign74890_e114383_d_n4;
        locals.var_wdep_func_dn5 = assign74890_e114383_d_n5;
        locals.var_wdep_func_dn6 = assign74890_e114383_d_n6;
        locals.var_wdep_func_dn7 = assign74890_e114383_d_n7;
        locals.var_wdep_func_dn8 = assign74890_e114383_d_n8;
        locals.var_wdep_func_dn9 = assign74890_e114383_d_n9;
        locals.var_wdep_func_dn10 = assign74890_e114383_d_n10;
        locals.var_wdep_func_dn13 = assign74890_e114383_d_n13;
        locals.var_wdep_func_rv = 0.0;

        let (assign74900_e114389, assign74900_e114389_d_n0, assign74900_e114389_d_n2, assign74900_e114389_d_n4, assign74900_e114389_d_n5, assign74900_e114389_d_n6, assign74900_e114389_d_n7, assign74900_e114389_d_n8, assign74900_e114389_d_n9, assign74900_e114389_d_n10, assign74900_e114389_d_n13,) = {
    if ((locals.var_guard1748 != 0.0) && (locals.var_guard1752 != 0.0)) {
        (locals.var_cnst0overs, locals.var_cnst0overs_dn0, locals.var_cnst0overs_dn2, locals.var_cnst0overs_dn4, locals.var_cnst0overs_dn5, locals.var_cnst0overs_dn6, locals.var_cnst0overs_dn7, locals.var_cnst0overs_dn8, locals.var_cnst0overs_dn9, locals.var_cnst0overs_dn10, locals.var_cnst0overs_dn13,)
    } else {
        (locals.var_cnst0over_func, locals.var_cnst0over_func_dn0, locals.var_cnst0over_func_dn2, locals.var_cnst0over_func_dn4, locals.var_cnst0over_func_dn5, locals.var_cnst0over_func_dn6, locals.var_cnst0over_func_dn7, locals.var_cnst0over_func_dn8, locals.var_cnst0over_func_dn9, locals.var_cnst0over_func_dn10, locals.var_cnst0over_func_dn13,)
    }
};
        locals.var_cnst0over_func = assign74900_e114389;
        locals.var_cnst0over_func_dn0 = assign74900_e114389_d_n0;
        locals.var_cnst0over_func_dn2 = assign74900_e114389_d_n2;
        locals.var_cnst0over_func_dn4 = assign74900_e114389_d_n4;
        locals.var_cnst0over_func_dn5 = assign74900_e114389_d_n5;
        locals.var_cnst0over_func_dn6 = assign74900_e114389_d_n6;
        locals.var_cnst0over_func_dn7 = assign74900_e114389_d_n7;
        locals.var_cnst0over_func_dn8 = assign74900_e114389_d_n8;
        locals.var_cnst0over_func_dn9 = assign74900_e114389_d_n9;
        locals.var_cnst0over_func_dn10 = assign74900_e114389_d_n10;
        locals.var_cnst0over_func_dn13 = assign74900_e114389_d_n13;
        locals.var_cnst0over_func_rv = 0.0;

        let (assign74910_e114395,) = {
    if ((locals.var_guard1748 != 0.0) && (locals.var_guard1752 != 0.0)) {
        (locals.var_cox0,)
    } else {
        (locals.var_cox0_func,)
    }
};
        locals.var_cox0_func = assign74910_e114395;
        locals.var_cox0_func_rv = 0.0;

        let assign74920_e114414: f64 = if (((((p.p36 == 1.0) && (p.p66 > 0.0)) && (locals.var_uc_novers > 0.0)) && (locals.var_uc_cvdsover != 0.0)) && (p.p55 != 1.0)) { 1.0 } else { 0.0 };
        locals.var_guard1753 = assign74920_e114414;
        locals.var_guard1753_rv = 0.0;

        let (assign74930_e114423,) = {
    if (((locals.var_guard1749 != 0.0) && (locals.var_guard1748 == 0.0)) && (locals.var_guard1753 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_calcqover,)
    }
};
        locals.var_flg_calcqover = assign74930_e114423;
        locals.var_flg_calcqover_rv = 0.0;

        let (assign74940_e114434, assign74940_e114434_d_n2, assign74940_e114434_d_n6, assign74940_e114434_d_n7, assign74940_e114434_d_n8,) = {
    if (((locals.var_guard1749 != 0.0) && (locals.var_guard1748 == 0.0)) && (locals.var_guard1753 != 0.0)) {
        let assign74940_e114432: f64 = (locals.var_vgsei - locals.var_vbsei);
        (assign74940_e114432, (locals.var_vgsei_dn2 - locals.var_vbsei_dn2), locals.var_vgsei_dn6, 0.0, (-locals.var_vbsei_dn8),)
    } else {
        (locals.var_vgbgmt, locals.var_vgbgmt_dn2, locals.var_vgbgmt_dn6, locals.var_vgbgmt_dn7, locals.var_vgbgmt_dn8,)
    }
};
        locals.var_vgbgmt = assign74940_e114434;
        locals.var_vgbgmt_dn2 = assign74940_e114434_d_n2;
        locals.var_vgbgmt_dn6 = assign74940_e114434_d_n6;
        locals.var_vgbgmt_dn7 = assign74940_e114434_d_n7;
        locals.var_vgbgmt_dn8 = assign74940_e114434_d_n8;
        locals.var_vgbgmt_rv = 0.0;

        let (assign74950_e114444, assign74950_e114444_d_n0, assign74950_e114444_d_n2, assign74950_e114444_d_n4, assign74950_e114444_d_n5, assign74950_e114444_d_n6, assign74950_e114444_d_n7, assign74950_e114444_d_n8, assign74950_e114444_d_n9, assign74950_e114444_d_n10, assign74950_e114444_d_n13,) = {
    if (((locals.var_guard1749 != 0.0) && (locals.var_guard1748 == 0.0)) && (locals.var_guard1753 != 0.0)) {
        let assign74950_e114442: f64 = (-locals.var_vbsei);
        (assign74950_e114442, 0.0, (-locals.var_vbsei_dn2), 0.0, 0.0, 0.0, 0.0, (-locals.var_vbsei_dn8), 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vxbgmt, locals.var_vxbgmt_dn0, locals.var_vxbgmt_dn2, locals.var_vxbgmt_dn4, locals.var_vxbgmt_dn5, locals.var_vxbgmt_dn6, locals.var_vxbgmt_dn7, locals.var_vxbgmt_dn8, locals.var_vxbgmt_dn9, locals.var_vxbgmt_dn10, locals.var_vxbgmt_dn13,)
    }
};
        locals.var_vxbgmt = assign74950_e114444;
        locals.var_vxbgmt_dn0 = assign74950_e114444_d_n0;
        locals.var_vxbgmt_dn2 = assign74950_e114444_d_n2;
        locals.var_vxbgmt_dn4 = assign74950_e114444_d_n4;
        locals.var_vxbgmt_dn5 = assign74950_e114444_d_n5;
        locals.var_vxbgmt_dn6 = assign74950_e114444_d_n6;
        locals.var_vxbgmt_dn7 = assign74950_e114444_d_n7;
        locals.var_vxbgmt_dn8 = assign74950_e114444_d_n8;
        locals.var_vxbgmt_dn9 = assign74950_e114444_d_n9;
        locals.var_vxbgmt_dn10 = assign74950_e114444_d_n10;
        locals.var_vxbgmt_dn13 = assign74950_e114444_d_n13;
        locals.var_vxbgmt_rv = 0.0;

        let assign74960_e114455: f64 = if (((p.p35 == 1.0) && (p.p63 > 0.0)) && (locals.var_uc_nover > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1754 = assign74960_e114455;
        locals.var_guard1754_rv = 0.0;

        let (assign74970_e114466,) = {
    if (((locals.var_guard1750 != 0.0) && (!((locals.var_guard1748 != 0.0) || (locals.var_guard1749 != 0.0)))) && (locals.var_guard1754 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_calcqover,)
    }
};
        locals.var_flg_calcqover = assign74970_e114466;
        locals.var_flg_calcqover_rv = 0.0;

        let (assign74980_e114477,) = {
    if (((locals.var_guard1750 != 0.0) && (!((locals.var_guard1748 != 0.0) || (locals.var_guard1749 != 0.0)))) && (locals.var_guard1754 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_coovlp,)
    }
};
        locals.var_flg_coovlp = assign74980_e114477;
        locals.var_flg_coovlp_rv = 0.0;

        let (assign74990_e114490, assign74990_e114490_d_n2, assign74990_e114490_d_n6, assign74990_e114490_d_n7, assign74990_e114490_d_n8,) = {
    if (((locals.var_guard1750 != 0.0) && (!((locals.var_guard1748 != 0.0) || (locals.var_guard1749 != 0.0)))) && (locals.var_guard1754 != 0.0)) {
        let assign74990_e114488: f64 = (locals.var_vgsi - locals.var_vbsi);
        (assign74990_e114488, 0.0, locals.var_vgsi_dn6, (locals.var_vgsi_dn7 - locals.var_vbsi_dn7), (-locals.var_vbsi_dn8),)
    } else {
        (locals.var_vgbgmt, locals.var_vgbgmt_dn2, locals.var_vgbgmt_dn6, locals.var_vgbgmt_dn7, locals.var_vgbgmt_dn8,)
    }
};
        locals.var_vgbgmt = assign74990_e114490;
        locals.var_vgbgmt_dn2 = assign74990_e114490_d_n2;
        locals.var_vgbgmt_dn6 = assign74990_e114490_d_n6;
        locals.var_vgbgmt_dn7 = assign74990_e114490_d_n7;
        locals.var_vgbgmt_dn8 = assign74990_e114490_d_n8;
        locals.var_vgbgmt_rv = 0.0;

        let (assign75000_e114503, assign75000_e114503_d_n0, assign75000_e114503_d_n2, assign75000_e114503_d_n4, assign75000_e114503_d_n5, assign75000_e114503_d_n6, assign75000_e114503_d_n7, assign75000_e114503_d_n8, assign75000_e114503_d_n9, assign75000_e114503_d_n10, assign75000_e114503_d_n13,) = {
    if (((locals.var_guard1750 != 0.0) && (!((locals.var_guard1748 != 0.0) || (locals.var_guard1749 != 0.0)))) && (locals.var_guard1754 != 0.0)) {
        let assign75000_e114501: f64 = (locals.var_vdsi - locals.var_vbsi);
        (assign75000_e114501, 0.0, 0.0, 0.0, locals.var_vdsi_dn5, 0.0, (locals.var_vdsi_dn7 - locals.var_vbsi_dn7), (-locals.var_vbsi_dn8), 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vxbgmt, locals.var_vxbgmt_dn0, locals.var_vxbgmt_dn2, locals.var_vxbgmt_dn4, locals.var_vxbgmt_dn5, locals.var_vxbgmt_dn6, locals.var_vxbgmt_dn7, locals.var_vxbgmt_dn8, locals.var_vxbgmt_dn9, locals.var_vxbgmt_dn10, locals.var_vxbgmt_dn13,)
    }
};
        locals.var_vxbgmt = assign75000_e114503;
        locals.var_vxbgmt_dn0 = assign75000_e114503_d_n0;
        locals.var_vxbgmt_dn2 = assign75000_e114503_d_n2;
        locals.var_vxbgmt_dn4 = assign75000_e114503_d_n4;
        locals.var_vxbgmt_dn5 = assign75000_e114503_d_n5;
        locals.var_vxbgmt_dn6 = assign75000_e114503_d_n6;
        locals.var_vxbgmt_dn7 = assign75000_e114503_d_n7;
        locals.var_vxbgmt_dn8 = assign75000_e114503_d_n8;
        locals.var_vxbgmt_dn9 = assign75000_e114503_d_n9;
        locals.var_vxbgmt_dn10 = assign75000_e114503_d_n10;
        locals.var_vxbgmt_dn13 = assign75000_e114503_d_n13;
        locals.var_vxbgmt_rv = 0.0;

        let (assign75010_e114514,) = {
    if (((locals.var_guard1750 != 0.0) && (!((locals.var_guard1748 != 0.0) || (locals.var_guard1749 != 0.0)))) && (locals.var_guard1754 != 0.0)) {
        (locals.var_uc_nover,)
    } else {
        (locals.var_nover_func,)
    }
};
        locals.var_nover_func = assign75010_e114514;
        locals.var_nover_func_rv = 0.0;

        let (assign75020_e114529, assign75020_e114529_d_n0, assign75020_e114529_d_n2, assign75020_e114529_d_n4, assign75020_e114529_d_n5, assign75020_e114529_d_n6, assign75020_e114529_d_n7, assign75020_e114529_d_n8, assign75020_e114529_d_n9, assign75020_e114529_d_n10, assign75020_e114529_d_n13,) = {
    if (((locals.var_guard1750 != 0.0) && (!((locals.var_guard1748 != 0.0) || (locals.var_guard1749 != 0.0)))) && (locals.var_guard1754 != 0.0)) {
        let assign75020_e114526: f64 = (p.p64 * p.p55);
        let assign75020_e114527: f64 = (p.p63 + assign75020_e114526);
        (assign75020_e114527, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_lover_func, locals.var_lover_func_dn0, locals.var_lover_func_dn2, locals.var_lover_func_dn4, locals.var_lover_func_dn5, locals.var_lover_func_dn6, locals.var_lover_func_dn7, locals.var_lover_func_dn8, locals.var_lover_func_dn9, locals.var_lover_func_dn10, locals.var_lover_func_dn13,)
    }
};
        locals.var_lover_func = assign75020_e114529;
        locals.var_lover_func_dn0 = assign75020_e114529_d_n0;
        locals.var_lover_func_dn2 = assign75020_e114529_d_n2;
        locals.var_lover_func_dn4 = assign75020_e114529_d_n4;
        locals.var_lover_func_dn5 = assign75020_e114529_d_n5;
        locals.var_lover_func_dn6 = assign75020_e114529_d_n6;
        locals.var_lover_func_dn7 = assign75020_e114529_d_n7;
        locals.var_lover_func_dn8 = assign75020_e114529_d_n8;
        locals.var_lover_func_dn9 = assign75020_e114529_d_n9;
        locals.var_lover_func_dn10 = assign75020_e114529_d_n10;
        locals.var_lover_func_dn13 = assign75020_e114529_d_n13;
        locals.var_lover_func_rv = 0.0;

        let (assign75030_e114540, assign75030_e114540_d_n0, assign75030_e114540_d_n2, assign75030_e114540_d_n4, assign75030_e114540_d_n5, assign75030_e114540_d_n6, assign75030_e114540_d_n7, assign75030_e114540_d_n8, assign75030_e114540_d_n9, assign75030_e114540_d_n10, assign75030_e114540_d_n13,) = {
    if (((locals.var_guard1750 != 0.0) && (!((locals.var_guard1748 != 0.0) || (locals.var_guard1749 != 0.0)))) && (locals.var_guard1754 != 0.0)) {
        (locals.var_wdep, locals.var_wdep_dn0, locals.var_wdep_dn2, locals.var_wdep_dn4, locals.var_wdep_dn5, locals.var_wdep_dn6, locals.var_wdep_dn7, locals.var_wdep_dn8, locals.var_wdep_dn9, locals.var_wdep_dn10, locals.var_wdep_dn13,)
    } else {
        (locals.var_wdep_func, locals.var_wdep_func_dn0, locals.var_wdep_func_dn2, locals.var_wdep_func_dn4, locals.var_wdep_func_dn5, locals.var_wdep_func_dn6, locals.var_wdep_func_dn7, locals.var_wdep_func_dn8, locals.var_wdep_func_dn9, locals.var_wdep_func_dn10, locals.var_wdep_func_dn13,)
    }
};
        locals.var_wdep_func = assign75030_e114540;
        locals.var_wdep_func_dn0 = assign75030_e114540_d_n0;
        locals.var_wdep_func_dn2 = assign75030_e114540_d_n2;
        locals.var_wdep_func_dn4 = assign75030_e114540_d_n4;
        locals.var_wdep_func_dn5 = assign75030_e114540_d_n5;
        locals.var_wdep_func_dn6 = assign75030_e114540_d_n6;
        locals.var_wdep_func_dn7 = assign75030_e114540_d_n7;
        locals.var_wdep_func_dn8 = assign75030_e114540_d_n8;
        locals.var_wdep_func_dn9 = assign75030_e114540_d_n9;
        locals.var_wdep_func_dn10 = assign75030_e114540_d_n10;
        locals.var_wdep_func_dn13 = assign75030_e114540_d_n13;
        locals.var_wdep_func_rv = 0.0;

        let (assign75040_e114551, assign75040_e114551_d_n0, assign75040_e114551_d_n2, assign75040_e114551_d_n4, assign75040_e114551_d_n5, assign75040_e114551_d_n6, assign75040_e114551_d_n7, assign75040_e114551_d_n8, assign75040_e114551_d_n9, assign75040_e114551_d_n10, assign75040_e114551_d_n13,) = {
    if (((locals.var_guard1750 != 0.0) && (!((locals.var_guard1748 != 0.0) || (locals.var_guard1749 != 0.0)))) && (locals.var_guard1754 != 0.0)) {
        (locals.var_cnst0over, locals.var_cnst0over_dn0, locals.var_cnst0over_dn2, locals.var_cnst0over_dn4, locals.var_cnst0over_dn5, locals.var_cnst0over_dn6, locals.var_cnst0over_dn7, locals.var_cnst0over_dn8, locals.var_cnst0over_dn9, locals.var_cnst0over_dn10, locals.var_cnst0over_dn13,)
    } else {
        (locals.var_cnst0over_func, locals.var_cnst0over_func_dn0, locals.var_cnst0over_func_dn2, locals.var_cnst0over_func_dn4, locals.var_cnst0over_func_dn5, locals.var_cnst0over_func_dn6, locals.var_cnst0over_func_dn7, locals.var_cnst0over_func_dn8, locals.var_cnst0over_func_dn9, locals.var_cnst0over_func_dn10, locals.var_cnst0over_func_dn13,)
    }
};
        locals.var_cnst0over_func = assign75040_e114551;
        locals.var_cnst0over_func_dn0 = assign75040_e114551_d_n0;
        locals.var_cnst0over_func_dn2 = assign75040_e114551_d_n2;
        locals.var_cnst0over_func_dn4 = assign75040_e114551_d_n4;
        locals.var_cnst0over_func_dn5 = assign75040_e114551_d_n5;
        locals.var_cnst0over_func_dn6 = assign75040_e114551_d_n6;
        locals.var_cnst0over_func_dn7 = assign75040_e114551_d_n7;
        locals.var_cnst0over_func_dn8 = assign75040_e114551_d_n8;
        locals.var_cnst0over_func_dn9 = assign75040_e114551_d_n9;
        locals.var_cnst0over_func_dn10 = assign75040_e114551_d_n10;
        locals.var_cnst0over_func_dn13 = assign75040_e114551_d_n13;
        locals.var_cnst0over_func_rv = 0.0;

        let (assign75050_e114562,) = {
    if (((locals.var_guard1750 != 0.0) && (!((locals.var_guard1748 != 0.0) || (locals.var_guard1749 != 0.0)))) && (locals.var_guard1754 != 0.0)) {
        (locals.var_coxb0,)
    } else {
        (locals.var_cox0_func,)
    }
};
        locals.var_cox0_func = assign75050_e114562;
        locals.var_cox0_func_rv = 0.0;

        let (assign75060_e114574, assign75060_e114574_d_n0, assign75060_e114574_d_n2, assign75060_e114574_d_n4, assign75060_e114574_d_n5, assign75060_e114574_d_n6, assign75060_e114574_d_n7, assign75060_e114574_d_n8, assign75060_e114574_d_n9, assign75060_e114574_d_n10, assign75060_e114574_d_n13,) = {
    if (((locals.var_guard1750 != 0.0) && (!((locals.var_guard1748 != 0.0) || (locals.var_guard1749 != 0.0)))) && (locals.var_guard1754 != 0.0)) {
        let assign75060_e114572: f64 = (-locals.var_lover_func);
        (assign75060_e114572, (-locals.var_lover_func_dn0), (-locals.var_lover_func_dn2), (-locals.var_lover_func_dn4), (-locals.var_lover_func_dn5), (-locals.var_lover_func_dn6), (-locals.var_lover_func_dn7), (-locals.var_lover_func_dn8), (-locals.var_lover_func_dn9), (-locals.var_lover_func_dn10), (-locals.var_lover_func_dn13),)
    } else {
        (locals.var_lover_func, locals.var_lover_func_dn0, locals.var_lover_func_dn2, locals.var_lover_func_dn4, locals.var_lover_func_dn5, locals.var_lover_func_dn6, locals.var_lover_func_dn7, locals.var_lover_func_dn8, locals.var_lover_func_dn9, locals.var_lover_func_dn10, locals.var_lover_func_dn13,)
    }
};
        locals.var_lover_func = assign75060_e114574;
        locals.var_lover_func_dn0 = assign75060_e114574_d_n0;
        locals.var_lover_func_dn2 = assign75060_e114574_d_n2;
        locals.var_lover_func_dn4 = assign75060_e114574_d_n4;
        locals.var_lover_func_dn5 = assign75060_e114574_d_n5;
        locals.var_lover_func_dn6 = assign75060_e114574_d_n6;
        locals.var_lover_func_dn7 = assign75060_e114574_d_n7;
        locals.var_lover_func_dn8 = assign75060_e114574_d_n8;
        locals.var_lover_func_dn9 = assign75060_e114574_d_n9;
        locals.var_lover_func_dn10 = assign75060_e114574_d_n10;
        locals.var_lover_func_dn13 = assign75060_e114574_d_n13;
        locals.var_lover_func_rv = 0.0;

        let assign75070_e114585: f64 = if (((locals.var_lover_func < 0.0) && (p.p432 > 0.0)) && (p.p55 == 1.0)) { 1.0 } else { 0.0 };
        locals.var_guard1755 = assign75070_e114585;
        locals.var_guard1755_rv = 0.0;

        let (assign75080_e114599, assign75080_e114599_d_n0, assign75080_e114599_d_n2, assign75080_e114599_d_n4, assign75080_e114599_d_n5, assign75080_e114599_d_n6, assign75080_e114599_d_n7, assign75080_e114599_d_n8, assign75080_e114599_d_n9, assign75080_e114599_d_n10, assign75080_e114599_d_n13,) = {
    if ((((locals.var_guard1750 != 0.0) && (!((locals.var_guard1748 != 0.0) || (locals.var_guard1749 != 0.0)))) && (locals.var_guard1754 != 0.0)) && (locals.var_guard1755 != 0.0)) {
        let assign75080_e114597: f64 = (-locals.var_lover_func);
        (assign75080_e114597, (-locals.var_lover_func_dn0), (-locals.var_lover_func_dn2), (-locals.var_lover_func_dn4), (-locals.var_lover_func_dn5), (-locals.var_lover_func_dn6), (-locals.var_lover_func_dn7), (-locals.var_lover_func_dn8), (-locals.var_lover_func_dn9), (-locals.var_lover_func_dn10), (-locals.var_lover_func_dn13),)
    } else {
        (locals.var_lover_func, locals.var_lover_func_dn0, locals.var_lover_func_dn2, locals.var_lover_func_dn4, locals.var_lover_func_dn5, locals.var_lover_func_dn6, locals.var_lover_func_dn7, locals.var_lover_func_dn8, locals.var_lover_func_dn9, locals.var_lover_func_dn10, locals.var_lover_func_dn13,)
    }
};
        locals.var_lover_func = assign75080_e114599;
        locals.var_lover_func_dn0 = assign75080_e114599_d_n0;
        locals.var_lover_func_dn2 = assign75080_e114599_d_n2;
        locals.var_lover_func_dn4 = assign75080_e114599_d_n4;
        locals.var_lover_func_dn5 = assign75080_e114599_d_n5;
        locals.var_lover_func_dn6 = assign75080_e114599_d_n6;
        locals.var_lover_func_dn7 = assign75080_e114599_d_n7;
        locals.var_lover_func_dn8 = assign75080_e114599_d_n8;
        locals.var_lover_func_dn9 = assign75080_e114599_d_n9;
        locals.var_lover_func_dn10 = assign75080_e114599_d_n10;
        locals.var_lover_func_dn13 = assign75080_e114599_d_n13;
        locals.var_lover_func_rv = 0.0;

        let (assign75090_e114612, assign75090_e114612_d_n0, assign75090_e114612_d_n2, assign75090_e114612_d_n4, assign75090_e114612_d_n5, assign75090_e114612_d_n6, assign75090_e114612_d_n7, assign75090_e114612_d_n8, assign75090_e114612_d_n9, assign75090_e114612_d_n10, assign75090_e114612_d_n13,) = {
    if ((((locals.var_guard1750 != 0.0) && (!((locals.var_guard1748 != 0.0) || (locals.var_guard1749 != 0.0)))) && (locals.var_guard1754 != 0.0)) && (locals.var_guard1755 != 0.0)) {
        (p.p63, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign75090_e114612;
        locals.var_t1_dn0 = assign75090_e114612_d_n0;
        locals.var_t1_dn2 = assign75090_e114612_d_n2;
        locals.var_t1_dn4 = assign75090_e114612_d_n4;
        locals.var_t1_dn5 = assign75090_e114612_d_n5;
        locals.var_t1_dn6 = assign75090_e114612_d_n6;
        locals.var_t1_dn7 = assign75090_e114612_d_n7;
        locals.var_t1_dn8 = assign75090_e114612_d_n8;
        locals.var_t1_dn9 = assign75090_e114612_d_n9;
        locals.var_t1_dn10 = assign75090_e114612_d_n10;
        locals.var_t1_dn13 = assign75090_e114612_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign75100_e114631, assign75100_e114631_d_n0, assign75100_e114631_d_n2, assign75100_e114631_d_n4, assign75100_e114631_d_n5, assign75100_e114631_d_n6, assign75100_e114631_d_n7, assign75100_e114631_d_n8, assign75100_e114631_d_n9, assign75100_e114631_d_n10, assign75100_e114631_d_n13,) = {
    if ((((locals.var_guard1750 != 0.0) && (!((locals.var_guard1748 != 0.0) || (locals.var_guard1749 != 0.0)))) && (locals.var_guard1754 != 0.0)) && (locals.var_guard1755 != 0.0)) {
        let assign75100_e114625: f64 = (locals.var_t1 * locals.var_t1);
        let assign75100_e114627: f64 = (assign75100_e114625 / locals.var_kjunc);
        let assign75100_e114629: f64 = (assign75100_e114627 - p.p137);
        (assign75100_e114629, (((((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0)) * locals.var_kjunc) - (assign75100_e114625 * locals.var_kjunc_dn0)) / (locals.var_kjunc * locals.var_kjunc)), (((((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2)) * locals.var_kjunc) - (assign75100_e114625 * locals.var_kjunc_dn2)) / (locals.var_kjunc * locals.var_kjunc)), (((((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)) * locals.var_kjunc) - (assign75100_e114625 * locals.var_kjunc_dn4)) / (locals.var_kjunc * locals.var_kjunc)), (((((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)) * locals.var_kjunc) - (assign75100_e114625 * locals.var_kjunc_dn5)) / (locals.var_kjunc * locals.var_kjunc)), (((((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)) * locals.var_kjunc) - (assign75100_e114625 * locals.var_kjunc_dn6)) / (locals.var_kjunc * locals.var_kjunc)), (((((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)) * locals.var_kjunc) - (assign75100_e114625 * locals.var_kjunc_dn7)) / (locals.var_kjunc * locals.var_kjunc)), (((((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)) * locals.var_kjunc) - (assign75100_e114625 * locals.var_kjunc_dn8)) / (locals.var_kjunc * locals.var_kjunc)), (((((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)) * locals.var_kjunc) - (assign75100_e114625 * locals.var_kjunc_dn9)) / (locals.var_kjunc * locals.var_kjunc)), (((((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)) * locals.var_kjunc) - (assign75100_e114625 * locals.var_kjunc_dn10)) / (locals.var_kjunc * locals.var_kjunc)), (((((locals.var_t1_dn13 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn13)) * locals.var_kjunc) - (assign75100_e114625 * locals.var_kjunc_dn13)) / (locals.var_kjunc * locals.var_kjunc)),)
    } else {
        (locals.var_vxb_lim, locals.var_vxb_lim_dn0, locals.var_vxb_lim_dn2, locals.var_vxb_lim_dn4, locals.var_vxb_lim_dn5, locals.var_vxb_lim_dn6, locals.var_vxb_lim_dn7, locals.var_vxb_lim_dn8, locals.var_vxb_lim_dn9, locals.var_vxb_lim_dn10, locals.var_vxb_lim_dn13,)
    }
};
        locals.var_vxb_lim = assign75100_e114631;
        locals.var_vxb_lim_dn0 = assign75100_e114631_d_n0;
        locals.var_vxb_lim_dn2 = assign75100_e114631_d_n2;
        locals.var_vxb_lim_dn4 = assign75100_e114631_d_n4;
        locals.var_vxb_lim_dn5 = assign75100_e114631_d_n5;
        locals.var_vxb_lim_dn6 = assign75100_e114631_d_n6;
        locals.var_vxb_lim_dn7 = assign75100_e114631_d_n7;
        locals.var_vxb_lim_dn8 = assign75100_e114631_d_n8;
        locals.var_vxb_lim_dn9 = assign75100_e114631_d_n9;
        locals.var_vxb_lim_dn10 = assign75100_e114631_d_n10;
        locals.var_vxb_lim_dn13 = assign75100_e114631_d_n13;
        locals.var_vxb_lim_rv = 0.0;

        let assign75110_e114634: f64 = if p.p113 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1756 = assign75110_e114634;
        locals.var_guard1756_rv = 0.0;

        let assign75120_e114641: f64 = if ((locals.var_vxbgmt == 0.0) || (p.p113 <= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1757 = assign75120_e114641;
        locals.var_guard1757_rv = 0.0;

        let (assign75130_e114658, assign75130_e114658_d_n0, assign75130_e114658_d_n2, assign75130_e114658_d_n4, assign75130_e114658_d_n5, assign75130_e114658_d_n6, assign75130_e114658_d_n7, assign75130_e114658_d_n8, assign75130_e114658_d_n9, assign75130_e114658_d_n10, assign75130_e114658_d_n13,) = {
    if ((((((locals.var_guard1750 != 0.0) && (!((locals.var_guard1748 != 0.0) || (locals.var_guard1749 != 0.0)))) && (locals.var_guard1754 != 0.0)) && (locals.var_guard1755 != 0.0)) && (locals.var_guard1756 != 0.0)) && (locals.var_guard1757 != 0.0)) {
        (locals.var_vxbgmt, locals.var_vxbgmt_dn0, locals.var_vxbgmt_dn2, locals.var_vxbgmt_dn4, locals.var_vxbgmt_dn5, locals.var_vxbgmt_dn6, locals.var_vxbgmt_dn7, locals.var_vxbgmt_dn8, locals.var_vxbgmt_dn9, locals.var_vxbgmt_dn10, locals.var_vxbgmt_dn13,)
    } else {
        (locals.var_vxbgmt, locals.var_vxbgmt_dn0, locals.var_vxbgmt_dn2, locals.var_vxbgmt_dn4, locals.var_vxbgmt_dn5, locals.var_vxbgmt_dn6, locals.var_vxbgmt_dn7, locals.var_vxbgmt_dn8, locals.var_vxbgmt_dn9, locals.var_vxbgmt_dn10, locals.var_vxbgmt_dn13,)
    }
};
        locals.var_vxbgmt = assign75130_e114658;
        locals.var_vxbgmt_dn0 = assign75130_e114658_d_n0;
        locals.var_vxbgmt_dn2 = assign75130_e114658_d_n2;
        locals.var_vxbgmt_dn4 = assign75130_e114658_d_n4;
        locals.var_vxbgmt_dn5 = assign75130_e114658_d_n5;
        locals.var_vxbgmt_dn6 = assign75130_e114658_d_n6;
        locals.var_vxbgmt_dn7 = assign75130_e114658_d_n7;
        locals.var_vxbgmt_dn8 = assign75130_e114658_d_n8;
        locals.var_vxbgmt_dn9 = assign75130_e114658_d_n9;
        locals.var_vxbgmt_dn10 = assign75130_e114658_d_n10;
        locals.var_vxbgmt_dn13 = assign75130_e114658_d_n13;
        locals.var_vxbgmt_rv = 0.0;

        let (assign75140_e114682, assign75140_e114682_d_n0, assign75140_e114682_d_n2, assign75140_e114682_d_n4, assign75140_e114682_d_n5, assign75140_e114682_d_n6, assign75140_e114682_d_n7, assign75140_e114682_d_n8, assign75140_e114682_d_n9, assign75140_e114682_d_n10, assign75140_e114682_d_n13,) = {
    if ((((((locals.var_guard1750 != 0.0) && (!((locals.var_guard1748 != 0.0) || (locals.var_guard1749 != 0.0)))) && (locals.var_guard1754 != 0.0)) && (locals.var_guard1755 != 0.0)) && (locals.var_guard1756 != 0.0)) && (locals.var_guard1757 == 0.0)) {
        let (assign75140_e114680,) = {
            if (locals.var_vxbgmt < 0.0) {
                let assign75140_e114678: f64 = (-1.0);
                (assign75140_e114678,)
            } else {
                (1.0,)
            }
        };
        (assign75140_e114680, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf3, locals.var_tmf3_dn0, locals.var_tmf3_dn2, locals.var_tmf3_dn4, locals.var_tmf3_dn5, locals.var_tmf3_dn6, locals.var_tmf3_dn7, locals.var_tmf3_dn8, locals.var_tmf3_dn9, locals.var_tmf3_dn10, locals.var_tmf3_dn13,)
    }
};
        locals.var_tmf3 = assign75140_e114682;
        locals.var_tmf3_dn0 = assign75140_e114682_d_n0;
        locals.var_tmf3_dn2 = assign75140_e114682_d_n2;
        locals.var_tmf3_dn4 = assign75140_e114682_d_n4;
        locals.var_tmf3_dn5 = assign75140_e114682_d_n5;
        locals.var_tmf3_dn6 = assign75140_e114682_d_n6;
        locals.var_tmf3_dn7 = assign75140_e114682_d_n7;
        locals.var_tmf3_dn8 = assign75140_e114682_d_n8;
        locals.var_tmf3_dn9 = assign75140_e114682_d_n9;
        locals.var_tmf3_dn10 = assign75140_e114682_d_n10;
        locals.var_tmf3_dn13 = assign75140_e114682_d_n13;
        locals.var_tmf3_rv = 0.0;

        let (assign75150_e114702, assign75150_e114702_d_n0, assign75150_e114702_d_n2, assign75150_e114702_d_n4, assign75150_e114702_d_n5, assign75150_e114702_d_n6, assign75150_e114702_d_n7, assign75150_e114702_d_n8, assign75150_e114702_d_n9, assign75150_e114702_d_n10, assign75150_e114702_d_n13,) = {
    if ((((((locals.var_guard1750 != 0.0) && (!((locals.var_guard1748 != 0.0) || (locals.var_guard1749 != 0.0)))) && (locals.var_guard1754 != 0.0)) && (locals.var_guard1755 != 0.0)) && (locals.var_guard1756 != 0.0)) && (locals.var_guard1757 == 0.0)) {
        let assign75150_e114700: f64 = (locals.var_tmf3 * locals.var_vxbgmt);
        (assign75150_e114700, ((locals.var_tmf3_dn0 * locals.var_vxbgmt) + (locals.var_tmf3 * locals.var_vxbgmt_dn0)), ((locals.var_tmf3_dn2 * locals.var_vxbgmt) + (locals.var_tmf3 * locals.var_vxbgmt_dn2)), ((locals.var_tmf3_dn4 * locals.var_vxbgmt) + (locals.var_tmf3 * locals.var_vxbgmt_dn4)), ((locals.var_tmf3_dn5 * locals.var_vxbgmt) + (locals.var_tmf3 * locals.var_vxbgmt_dn5)), ((locals.var_tmf3_dn6 * locals.var_vxbgmt) + (locals.var_tmf3 * locals.var_vxbgmt_dn6)), ((locals.var_tmf3_dn7 * locals.var_vxbgmt) + (locals.var_tmf3 * locals.var_vxbgmt_dn7)), ((locals.var_tmf3_dn8 * locals.var_vxbgmt) + (locals.var_tmf3 * locals.var_vxbgmt_dn8)), ((locals.var_tmf3_dn9 * locals.var_vxbgmt) + (locals.var_tmf3 * locals.var_vxbgmt_dn9)), ((locals.var_tmf3_dn10 * locals.var_vxbgmt) + (locals.var_tmf3 * locals.var_vxbgmt_dn10)), ((locals.var_tmf3_dn13 * locals.var_vxbgmt) + (locals.var_tmf3 * locals.var_vxbgmt_dn13)),)
    } else {
        (locals.var_tmf4, locals.var_tmf4_dn0, locals.var_tmf4_dn2, locals.var_tmf4_dn4, locals.var_tmf4_dn5, locals.var_tmf4_dn6, locals.var_tmf4_dn7, locals.var_tmf4_dn8, locals.var_tmf4_dn9, locals.var_tmf4_dn10, locals.var_tmf4_dn13,)
    }
};
        locals.var_tmf4 = assign75150_e114702;
        locals.var_tmf4_dn0 = assign75150_e114702_d_n0;
        locals.var_tmf4_dn2 = assign75150_e114702_d_n2;
        locals.var_tmf4_dn4 = assign75150_e114702_d_n4;
        locals.var_tmf4_dn5 = assign75150_e114702_d_n5;
        locals.var_tmf4_dn6 = assign75150_e114702_d_n6;
        locals.var_tmf4_dn7 = assign75150_e114702_d_n7;
        locals.var_tmf4_dn8 = assign75150_e114702_d_n8;
        locals.var_tmf4_dn9 = assign75150_e114702_d_n9;
        locals.var_tmf4_dn10 = assign75150_e114702_d_n10;
        locals.var_tmf4_dn13 = assign75150_e114702_d_n13;
        locals.var_tmf4_rv = 0.0;

        let (assign75160_e114726, assign75160_e114726_d_n0, assign75160_e114726_d_n2, assign75160_e114726_d_n4, assign75160_e114726_d_n5, assign75160_e114726_d_n6, assign75160_e114726_d_n7, assign75160_e114726_d_n8, assign75160_e114726_d_n9, assign75160_e114726_d_n10, assign75160_e114726_d_n13,) = {
    if ((((((locals.var_guard1750 != 0.0) && (!((locals.var_guard1748 != 0.0) || (locals.var_guard1749 != 0.0)))) && (locals.var_guard1754 != 0.0)) && (locals.var_guard1755 != 0.0)) && (locals.var_guard1756 != 0.0)) && (locals.var_guard1757 == 0.0)) {
        let assign75160_e114721: f64 = (locals.var_tmf4 / locals.var_vxb_lim);
        let assign75160_e114723: f64 = (assign75160_e114721).powf(p.p113);
        let assign75160_e114724: f64 = (1.0 + assign75160_e114723);
        (assign75160_e114724, if 0.0 == 0.0 && ((p.p113) as f64).is_finite() && ((p.p113) as f64).fract() == 0.0 { if p.p113 == 0.0 { 0.0 } else { (p.p113 * ((assign75160_e114721).powf(p.p113 - 1.0) * (((locals.var_tmf4_dn0 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn0)) / (locals.var_vxb_lim * locals.var_vxb_lim)))) } } else { (assign75160_e114723 * (p.p113 * ((((locals.var_tmf4_dn0 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn0)) / (locals.var_vxb_lim * locals.var_vxb_lim)) / assign75160_e114721))) }, if 0.0 == 0.0 && ((p.p113) as f64).is_finite() && ((p.p113) as f64).fract() == 0.0 { if p.p113 == 0.0 { 0.0 } else { (p.p113 * ((assign75160_e114721).powf(p.p113 - 1.0) * (((locals.var_tmf4_dn2 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn2)) / (locals.var_vxb_lim * locals.var_vxb_lim)))) } } else { (assign75160_e114723 * (p.p113 * ((((locals.var_tmf4_dn2 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn2)) / (locals.var_vxb_lim * locals.var_vxb_lim)) / assign75160_e114721))) }, if 0.0 == 0.0 && ((p.p113) as f64).is_finite() && ((p.p113) as f64).fract() == 0.0 { if p.p113 == 0.0 { 0.0 } else { (p.p113 * ((assign75160_e114721).powf(p.p113 - 1.0) * (((locals.var_tmf4_dn4 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn4)) / (locals.var_vxb_lim * locals.var_vxb_lim)))) } } else { (assign75160_e114723 * (p.p113 * ((((locals.var_tmf4_dn4 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn4)) / (locals.var_vxb_lim * locals.var_vxb_lim)) / assign75160_e114721))) }, if 0.0 == 0.0 && ((p.p113) as f64).is_finite() && ((p.p113) as f64).fract() == 0.0 { if p.p113 == 0.0 { 0.0 } else { (p.p113 * ((assign75160_e114721).powf(p.p113 - 1.0) * (((locals.var_tmf4_dn5 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn5)) / (locals.var_vxb_lim * locals.var_vxb_lim)))) } } else { (assign75160_e114723 * (p.p113 * ((((locals.var_tmf4_dn5 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn5)) / (locals.var_vxb_lim * locals.var_vxb_lim)) / assign75160_e114721))) }, if 0.0 == 0.0 && ((p.p113) as f64).is_finite() && ((p.p113) as f64).fract() == 0.0 { if p.p113 == 0.0 { 0.0 } else { (p.p113 * ((assign75160_e114721).powf(p.p113 - 1.0) * (((locals.var_tmf4_dn6 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn6)) / (locals.var_vxb_lim * locals.var_vxb_lim)))) } } else { (assign75160_e114723 * (p.p113 * ((((locals.var_tmf4_dn6 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn6)) / (locals.var_vxb_lim * locals.var_vxb_lim)) / assign75160_e114721))) }, if 0.0 == 0.0 && ((p.p113) as f64).is_finite() && ((p.p113) as f64).fract() == 0.0 { if p.p113 == 0.0 { 0.0 } else { (p.p113 * ((assign75160_e114721).powf(p.p113 - 1.0) * (((locals.var_tmf4_dn7 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn7)) / (locals.var_vxb_lim * locals.var_vxb_lim)))) } } else { (assign75160_e114723 * (p.p113 * ((((locals.var_tmf4_dn7 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn7)) / (locals.var_vxb_lim * locals.var_vxb_lim)) / assign75160_e114721))) }, if 0.0 == 0.0 && ((p.p113) as f64).is_finite() && ((p.p113) as f64).fract() == 0.0 { if p.p113 == 0.0 { 0.0 } else { (p.p113 * ((assign75160_e114721).powf(p.p113 - 1.0) * (((locals.var_tmf4_dn8 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn8)) / (locals.var_vxb_lim * locals.var_vxb_lim)))) } } else { (assign75160_e114723 * (p.p113 * ((((locals.var_tmf4_dn8 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn8)) / (locals.var_vxb_lim * locals.var_vxb_lim)) / assign75160_e114721))) }, if 0.0 == 0.0 && ((p.p113) as f64).is_finite() && ((p.p113) as f64).fract() == 0.0 { if p.p113 == 0.0 { 0.0 } else { (p.p113 * ((assign75160_e114721).powf(p.p113 - 1.0) * (((locals.var_tmf4_dn9 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn9)) / (locals.var_vxb_lim * locals.var_vxb_lim)))) } } else { (assign75160_e114723 * (p.p113 * ((((locals.var_tmf4_dn9 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn9)) / (locals.var_vxb_lim * locals.var_vxb_lim)) / assign75160_e114721))) }, if 0.0 == 0.0 && ((p.p113) as f64).is_finite() && ((p.p113) as f64).fract() == 0.0 { if p.p113 == 0.0 { 0.0 } else { (p.p113 * ((assign75160_e114721).powf(p.p113 - 1.0) * (((locals.var_tmf4_dn10 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn10)) / (locals.var_vxb_lim * locals.var_vxb_lim)))) } } else { (assign75160_e114723 * (p.p113 * ((((locals.var_tmf4_dn10 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn10)) / (locals.var_vxb_lim * locals.var_vxb_lim)) / assign75160_e114721))) }, if 0.0 == 0.0 && ((p.p113) as f64).is_finite() && ((p.p113) as f64).fract() == 0.0 { if p.p113 == 0.0 { 0.0 } else { (p.p113 * ((assign75160_e114721).powf(p.p113 - 1.0) * (((locals.var_tmf4_dn13 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn13)) / (locals.var_vxb_lim * locals.var_vxb_lim)))) } } else { (assign75160_e114723 * (p.p113 * ((((locals.var_tmf4_dn13 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn13)) / (locals.var_vxb_lim * locals.var_vxb_lim)) / assign75160_e114721))) },)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign75160_e114726;
        locals.var_tmf1_dn0 = assign75160_e114726_d_n0;
        locals.var_tmf1_dn2 = assign75160_e114726_d_n2;
        locals.var_tmf1_dn4 = assign75160_e114726_d_n4;
        locals.var_tmf1_dn5 = assign75160_e114726_d_n5;
        locals.var_tmf1_dn6 = assign75160_e114726_d_n6;
        locals.var_tmf1_dn7 = assign75160_e114726_d_n7;
        locals.var_tmf1_dn8 = assign75160_e114726_d_n8;
        locals.var_tmf1_dn9 = assign75160_e114726_d_n9;
        locals.var_tmf1_dn10 = assign75160_e114726_d_n10;
        locals.var_tmf1_dn13 = assign75160_e114726_d_n13;
        locals.var_tmf1_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_273(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign75170_e114748, assign75170_e114748_d_n0, assign75170_e114748_d_n2, assign75170_e114748_d_n4, assign75170_e114748_d_n5, assign75170_e114748_d_n6, assign75170_e114748_d_n7, assign75170_e114748_d_n8, assign75170_e114748_d_n9, assign75170_e114748_d_n10, assign75170_e114748_d_n13,) = {
    if ((((((locals.var_guard1750 != 0.0) && (!((locals.var_guard1748 != 0.0) || (locals.var_guard1749 != 0.0)))) && (locals.var_guard1754 != 0.0)) && (locals.var_guard1755 != 0.0)) && (locals.var_guard1756 != 0.0)) && (locals.var_guard1757 == 0.0)) {
        let assign75170_e114745: f64 = (1.0 / p.p113);
        let assign75170_e114746: f64 = (locals.var_tmf1).powf(assign75170_e114745);
        (assign75170_e114746, if 0.0 == 0.0 && ((assign75170_e114745) as f64).is_finite() && ((assign75170_e114745) as f64).fract() == 0.0 { if assign75170_e114745 == 0.0 { 0.0 } else { (assign75170_e114745 * ((locals.var_tmf1).powf(assign75170_e114745 - 1.0) * locals.var_tmf1_dn0)) } } else { (assign75170_e114746 * (assign75170_e114745 * (locals.var_tmf1_dn0 / locals.var_tmf1))) }, if 0.0 == 0.0 && ((assign75170_e114745) as f64).is_finite() && ((assign75170_e114745) as f64).fract() == 0.0 { if assign75170_e114745 == 0.0 { 0.0 } else { (assign75170_e114745 * ((locals.var_tmf1).powf(assign75170_e114745 - 1.0) * locals.var_tmf1_dn2)) } } else { (assign75170_e114746 * (assign75170_e114745 * (locals.var_tmf1_dn2 / locals.var_tmf1))) }, if 0.0 == 0.0 && ((assign75170_e114745) as f64).is_finite() && ((assign75170_e114745) as f64).fract() == 0.0 { if assign75170_e114745 == 0.0 { 0.0 } else { (assign75170_e114745 * ((locals.var_tmf1).powf(assign75170_e114745 - 1.0) * locals.var_tmf1_dn4)) } } else { (assign75170_e114746 * (assign75170_e114745 * (locals.var_tmf1_dn4 / locals.var_tmf1))) }, if 0.0 == 0.0 && ((assign75170_e114745) as f64).is_finite() && ((assign75170_e114745) as f64).fract() == 0.0 { if assign75170_e114745 == 0.0 { 0.0 } else { (assign75170_e114745 * ((locals.var_tmf1).powf(assign75170_e114745 - 1.0) * locals.var_tmf1_dn5)) } } else { (assign75170_e114746 * (assign75170_e114745 * (locals.var_tmf1_dn5 / locals.var_tmf1))) }, if 0.0 == 0.0 && ((assign75170_e114745) as f64).is_finite() && ((assign75170_e114745) as f64).fract() == 0.0 { if assign75170_e114745 == 0.0 { 0.0 } else { (assign75170_e114745 * ((locals.var_tmf1).powf(assign75170_e114745 - 1.0) * locals.var_tmf1_dn6)) } } else { (assign75170_e114746 * (assign75170_e114745 * (locals.var_tmf1_dn6 / locals.var_tmf1))) }, if 0.0 == 0.0 && ((assign75170_e114745) as f64).is_finite() && ((assign75170_e114745) as f64).fract() == 0.0 { if assign75170_e114745 == 0.0 { 0.0 } else { (assign75170_e114745 * ((locals.var_tmf1).powf(assign75170_e114745 - 1.0) * locals.var_tmf1_dn7)) } } else { (assign75170_e114746 * (assign75170_e114745 * (locals.var_tmf1_dn7 / locals.var_tmf1))) }, if 0.0 == 0.0 && ((assign75170_e114745) as f64).is_finite() && ((assign75170_e114745) as f64).fract() == 0.0 { if assign75170_e114745 == 0.0 { 0.0 } else { (assign75170_e114745 * ((locals.var_tmf1).powf(assign75170_e114745 - 1.0) * locals.var_tmf1_dn8)) } } else { (assign75170_e114746 * (assign75170_e114745 * (locals.var_tmf1_dn8 / locals.var_tmf1))) }, if 0.0 == 0.0 && ((assign75170_e114745) as f64).is_finite() && ((assign75170_e114745) as f64).fract() == 0.0 { if assign75170_e114745 == 0.0 { 0.0 } else { (assign75170_e114745 * ((locals.var_tmf1).powf(assign75170_e114745 - 1.0) * locals.var_tmf1_dn9)) } } else { (assign75170_e114746 * (assign75170_e114745 * (locals.var_tmf1_dn9 / locals.var_tmf1))) }, if 0.0 == 0.0 && ((assign75170_e114745) as f64).is_finite() && ((assign75170_e114745) as f64).fract() == 0.0 { if assign75170_e114745 == 0.0 { 0.0 } else { (assign75170_e114745 * ((locals.var_tmf1).powf(assign75170_e114745 - 1.0) * locals.var_tmf1_dn10)) } } else { (assign75170_e114746 * (assign75170_e114745 * (locals.var_tmf1_dn10 / locals.var_tmf1))) }, if 0.0 == 0.0 && ((assign75170_e114745) as f64).is_finite() && ((assign75170_e114745) as f64).fract() == 0.0 { if assign75170_e114745 == 0.0 { 0.0 } else { (assign75170_e114745 * ((locals.var_tmf1).powf(assign75170_e114745 - 1.0) * locals.var_tmf1_dn13)) } } else { (assign75170_e114746 * (assign75170_e114745 * (locals.var_tmf1_dn13 / locals.var_tmf1))) },)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign75170_e114748;
        locals.var_tmf2_dn0 = assign75170_e114748_d_n0;
        locals.var_tmf2_dn2 = assign75170_e114748_d_n2;
        locals.var_tmf2_dn4 = assign75170_e114748_d_n4;
        locals.var_tmf2_dn5 = assign75170_e114748_d_n5;
        locals.var_tmf2_dn6 = assign75170_e114748_d_n6;
        locals.var_tmf2_dn7 = assign75170_e114748_d_n7;
        locals.var_tmf2_dn8 = assign75170_e114748_d_n8;
        locals.var_tmf2_dn9 = assign75170_e114748_d_n9;
        locals.var_tmf2_dn10 = assign75170_e114748_d_n10;
        locals.var_tmf2_dn13 = assign75170_e114748_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign75180_e114770, assign75180_e114770_d_n0, assign75180_e114770_d_n2, assign75180_e114770_d_n4, assign75180_e114770_d_n5, assign75180_e114770_d_n6, assign75180_e114770_d_n7, assign75180_e114770_d_n8, assign75180_e114770_d_n9, assign75180_e114770_d_n10, assign75180_e114770_d_n13,) = {
    if ((((((locals.var_guard1750 != 0.0) && (!((locals.var_guard1748 != 0.0) || (locals.var_guard1749 != 0.0)))) && (locals.var_guard1754 != 0.0)) && (locals.var_guard1755 != 0.0)) && (locals.var_guard1756 != 0.0)) && (locals.var_guard1757 == 0.0)) {
        let assign75180_e114766: f64 = (locals.var_tmf3 * locals.var_tmf4);
        let assign75180_e114768: f64 = (assign75180_e114766 / locals.var_tmf2);
        (assign75180_e114768, (((((locals.var_tmf3_dn0 * locals.var_tmf4) + (locals.var_tmf3 * locals.var_tmf4_dn0)) * locals.var_tmf2) - (assign75180_e114766 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2)), (((((locals.var_tmf3_dn2 * locals.var_tmf4) + (locals.var_tmf3 * locals.var_tmf4_dn2)) * locals.var_tmf2) - (assign75180_e114766 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2)), (((((locals.var_tmf3_dn4 * locals.var_tmf4) + (locals.var_tmf3 * locals.var_tmf4_dn4)) * locals.var_tmf2) - (assign75180_e114766 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2)), (((((locals.var_tmf3_dn5 * locals.var_tmf4) + (locals.var_tmf3 * locals.var_tmf4_dn5)) * locals.var_tmf2) - (assign75180_e114766 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2)), (((((locals.var_tmf3_dn6 * locals.var_tmf4) + (locals.var_tmf3 * locals.var_tmf4_dn6)) * locals.var_tmf2) - (assign75180_e114766 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2)), (((((locals.var_tmf3_dn7 * locals.var_tmf4) + (locals.var_tmf3 * locals.var_tmf4_dn7)) * locals.var_tmf2) - (assign75180_e114766 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2)), (((((locals.var_tmf3_dn8 * locals.var_tmf4) + (locals.var_tmf3 * locals.var_tmf4_dn8)) * locals.var_tmf2) - (assign75180_e114766 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2)), (((((locals.var_tmf3_dn9 * locals.var_tmf4) + (locals.var_tmf3 * locals.var_tmf4_dn9)) * locals.var_tmf2) - (assign75180_e114766 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2)), (((((locals.var_tmf3_dn10 * locals.var_tmf4) + (locals.var_tmf3 * locals.var_tmf4_dn10)) * locals.var_tmf2) - (assign75180_e114766 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2)), (((((locals.var_tmf3_dn13 * locals.var_tmf4) + (locals.var_tmf3 * locals.var_tmf4_dn13)) * locals.var_tmf2) - (assign75180_e114766 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2)),)
    } else {
        (locals.var_vxbgmt, locals.var_vxbgmt_dn0, locals.var_vxbgmt_dn2, locals.var_vxbgmt_dn4, locals.var_vxbgmt_dn5, locals.var_vxbgmt_dn6, locals.var_vxbgmt_dn7, locals.var_vxbgmt_dn8, locals.var_vxbgmt_dn9, locals.var_vxbgmt_dn10, locals.var_vxbgmt_dn13,)
    }
};
        locals.var_vxbgmt = assign75180_e114770;
        locals.var_vxbgmt_dn0 = assign75180_e114770_d_n0;
        locals.var_vxbgmt_dn2 = assign75180_e114770_d_n2;
        locals.var_vxbgmt_dn4 = assign75180_e114770_d_n4;
        locals.var_vxbgmt_dn5 = assign75180_e114770_d_n5;
        locals.var_vxbgmt_dn6 = assign75180_e114770_d_n6;
        locals.var_vxbgmt_dn7 = assign75180_e114770_d_n7;
        locals.var_vxbgmt_dn8 = assign75180_e114770_d_n8;
        locals.var_vxbgmt_dn9 = assign75180_e114770_d_n9;
        locals.var_vxbgmt_dn10 = assign75180_e114770_d_n10;
        locals.var_vxbgmt_dn13 = assign75180_e114770_d_n13;
        locals.var_vxbgmt_rv = 0.0;

        let (assign75190_e114798, assign75190_e114798_d_n0, assign75190_e114798_d_n2, assign75190_e114798_d_n4, assign75190_e114798_d_n5, assign75190_e114798_d_n6, assign75190_e114798_d_n7, assign75190_e114798_d_n8, assign75190_e114798_d_n9, assign75190_e114798_d_n10, assign75190_e114798_d_n13,) = {
    if (((((locals.var_guard1750 != 0.0) && (!((locals.var_guard1748 != 0.0) || (locals.var_guard1749 != 0.0)))) && (locals.var_guard1754 != 0.0)) && (locals.var_guard1755 != 0.0)) && (locals.var_guard1756 != 0.0)) {
        let assign75190_e114785: f64 = (locals.var_vxbgmt + p.p137);
        let assign75190_e114788: f64 = (locals.var_vxbgmt + p.p137);
        let assign75190_e114789: f64 = (assign75190_e114785 * assign75190_e114788);
        let assign75190_e114792: f64 = (4.0 * 0.1);
        let assign75190_e114794: f64 = (assign75190_e114792 * 0.1);
        let assign75190_e114795: f64 = (assign75190_e114789 + assign75190_e114794);
        let assign75190_e114796: f64 = (assign75190_e114795).sqrt();
        (assign75190_e114796, (((locals.var_vxbgmt_dn0 * assign75190_e114788) + (assign75190_e114785 * locals.var_vxbgmt_dn0)) / (2.0 * assign75190_e114796)), (((locals.var_vxbgmt_dn2 * assign75190_e114788) + (assign75190_e114785 * locals.var_vxbgmt_dn2)) / (2.0 * assign75190_e114796)), (((locals.var_vxbgmt_dn4 * assign75190_e114788) + (assign75190_e114785 * locals.var_vxbgmt_dn4)) / (2.0 * assign75190_e114796)), (((locals.var_vxbgmt_dn5 * assign75190_e114788) + (assign75190_e114785 * locals.var_vxbgmt_dn5)) / (2.0 * assign75190_e114796)), (((locals.var_vxbgmt_dn6 * assign75190_e114788) + (assign75190_e114785 * locals.var_vxbgmt_dn6)) / (2.0 * assign75190_e114796)), (((locals.var_vxbgmt_dn7 * assign75190_e114788) + (assign75190_e114785 * locals.var_vxbgmt_dn7)) / (2.0 * assign75190_e114796)), (((locals.var_vxbgmt_dn8 * assign75190_e114788) + (assign75190_e114785 * locals.var_vxbgmt_dn8)) / (2.0 * assign75190_e114796)), (((locals.var_vxbgmt_dn9 * assign75190_e114788) + (assign75190_e114785 * locals.var_vxbgmt_dn9)) / (2.0 * assign75190_e114796)), (((locals.var_vxbgmt_dn10 * assign75190_e114788) + (assign75190_e114785 * locals.var_vxbgmt_dn10)) / (2.0 * assign75190_e114796)), (((locals.var_vxbgmt_dn13 * assign75190_e114788) + (assign75190_e114785 * locals.var_vxbgmt_dn13)) / (2.0 * assign75190_e114796)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign75190_e114798;
        locals.var_tmf2_dn0 = assign75190_e114798_d_n0;
        locals.var_tmf2_dn2 = assign75190_e114798_d_n2;
        locals.var_tmf2_dn4 = assign75190_e114798_d_n4;
        locals.var_tmf2_dn5 = assign75190_e114798_d_n5;
        locals.var_tmf2_dn6 = assign75190_e114798_d_n6;
        locals.var_tmf2_dn7 = assign75190_e114798_d_n7;
        locals.var_tmf2_dn8 = assign75190_e114798_d_n8;
        locals.var_tmf2_dn9 = assign75190_e114798_d_n9;
        locals.var_tmf2_dn10 = assign75190_e114798_d_n10;
        locals.var_tmf2_dn13 = assign75190_e114798_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign75200_e114821, assign75200_e114821_d_n0, assign75200_e114821_d_n2, assign75200_e114821_d_n4, assign75200_e114821_d_n5, assign75200_e114821_d_n6, assign75200_e114821_d_n7, assign75200_e114821_d_n8, assign75200_e114821_d_n9, assign75200_e114821_d_n10, assign75200_e114821_d_n13,) = {
    if (((((locals.var_guard1750 != 0.0) && (!((locals.var_guard1748 != 0.0) || (locals.var_guard1749 != 0.0)))) && (locals.var_guard1754 != 0.0)) && (locals.var_guard1755 != 0.0)) && (locals.var_guard1756 != 0.0)) {
        let assign75200_e114815: f64 = (locals.var_vxbgmt + p.p137);
        let assign75200_e114817: f64 = (assign75200_e114815 / locals.var_tmf2);
        let assign75200_e114818: f64 = (1.0 + assign75200_e114817);
        let assign75200_e114819: f64 = (0.5 * assign75200_e114818);
        (assign75200_e114819, (0.5 * (((locals.var_vxbgmt_dn0 * locals.var_tmf2) - (assign75200_e114815 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vxbgmt_dn2 * locals.var_tmf2) - (assign75200_e114815 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vxbgmt_dn4 * locals.var_tmf2) - (assign75200_e114815 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vxbgmt_dn5 * locals.var_tmf2) - (assign75200_e114815 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vxbgmt_dn6 * locals.var_tmf2) - (assign75200_e114815 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vxbgmt_dn7 * locals.var_tmf2) - (assign75200_e114815 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vxbgmt_dn8 * locals.var_tmf2) - (assign75200_e114815 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vxbgmt_dn9 * locals.var_tmf2) - (assign75200_e114815 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vxbgmt_dn10 * locals.var_tmf2) - (assign75200_e114815 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vxbgmt_dn13 * locals.var_tmf2) - (assign75200_e114815 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign75200_e114821;
        locals.var_t9_dn0 = assign75200_e114821_d_n0;
        locals.var_t9_dn2 = assign75200_e114821_d_n2;
        locals.var_t9_dn4 = assign75200_e114821_d_n4;
        locals.var_t9_dn5 = assign75200_e114821_d_n5;
        locals.var_t9_dn6 = assign75200_e114821_d_n6;
        locals.var_t9_dn7 = assign75200_e114821_d_n7;
        locals.var_t9_dn8 = assign75200_e114821_d_n8;
        locals.var_t9_dn9 = assign75200_e114821_d_n9;
        locals.var_t9_dn10 = assign75200_e114821_d_n10;
        locals.var_t9_dn13 = assign75200_e114821_d_n13;
        locals.var_t9_rv = 0.0;

        let (assign75210_e114842, assign75210_e114842_d_n0, assign75210_e114842_d_n2, assign75210_e114842_d_n4, assign75210_e114842_d_n5, assign75210_e114842_d_n6, assign75210_e114842_d_n7, assign75210_e114842_d_n8, assign75210_e114842_d_n9, assign75210_e114842_d_n10, assign75210_e114842_d_n13,) = {
    if (((((locals.var_guard1750 != 0.0) && (!((locals.var_guard1748 != 0.0) || (locals.var_guard1749 != 0.0)))) && (locals.var_guard1754 != 0.0)) && (locals.var_guard1755 != 0.0)) && (locals.var_guard1756 != 0.0)) {
        let assign75210_e114837: f64 = (locals.var_vxbgmt + p.p137);
        let assign75210_e114839: f64 = (assign75210_e114837 + locals.var_tmf2);
        let assign75210_e114840: f64 = (0.5 * assign75210_e114839);
        (assign75210_e114840, (0.5 * (locals.var_vxbgmt_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_vxbgmt_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_vxbgmt_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_vxbgmt_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_vxbgmt_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_vxbgmt_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_vxbgmt_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_vxbgmt_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_vxbgmt_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_vxbgmt_dn13 + locals.var_tmf2_dn13)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign75210_e114842;
        locals.var_t2_dn0 = assign75210_e114842_d_n0;
        locals.var_t2_dn2 = assign75210_e114842_d_n2;
        locals.var_t2_dn4 = assign75210_e114842_d_n4;
        locals.var_t2_dn5 = assign75210_e114842_d_n5;
        locals.var_t2_dn6 = assign75210_e114842_d_n6;
        locals.var_t2_dn7 = assign75210_e114842_d_n7;
        locals.var_t2_dn8 = assign75210_e114842_d_n8;
        locals.var_t2_dn9 = assign75210_e114842_d_n9;
        locals.var_t2_dn10 = assign75210_e114842_d_n10;
        locals.var_t2_dn13 = assign75210_e114842_d_n13;
        locals.var_t2_rv = 0.0;

        let assign75220_e114845: f64 = if locals.var_t2 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1758 = assign75220_e114845;
        locals.var_guard1758_rv = 0.0;

        let (assign75230_e114862, assign75230_e114862_d_n0, assign75230_e114862_d_n2, assign75230_e114862_d_n4, assign75230_e114862_d_n5, assign75230_e114862_d_n6, assign75230_e114862_d_n7, assign75230_e114862_d_n8, assign75230_e114862_d_n9, assign75230_e114862_d_n10, assign75230_e114862_d_n13,) = {
    if ((((((locals.var_guard1750 != 0.0) && (!((locals.var_guard1748 != 0.0) || (locals.var_guard1749 != 0.0)))) && (locals.var_guard1754 != 0.0)) && (locals.var_guard1755 != 0.0)) && (locals.var_guard1756 != 0.0)) && (locals.var_guard1758 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign75230_e114862;
        locals.var_t2_dn0 = assign75230_e114862_d_n0;
        locals.var_t2_dn2 = assign75230_e114862_d_n2;
        locals.var_t2_dn4 = assign75230_e114862_d_n4;
        locals.var_t2_dn5 = assign75230_e114862_d_n5;
        locals.var_t2_dn6 = assign75230_e114862_d_n6;
        locals.var_t2_dn7 = assign75230_e114862_d_n7;
        locals.var_t2_dn8 = assign75230_e114862_d_n8;
        locals.var_t2_dn9 = assign75230_e114862_d_n9;
        locals.var_t2_dn10 = assign75230_e114862_d_n10;
        locals.var_t2_dn13 = assign75230_e114862_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign75240_e114879, assign75240_e114879_d_n0, assign75240_e114879_d_n2, assign75240_e114879_d_n4, assign75240_e114879_d_n5, assign75240_e114879_d_n6, assign75240_e114879_d_n7, assign75240_e114879_d_n8, assign75240_e114879_d_n9, assign75240_e114879_d_n10, assign75240_e114879_d_n13,) = {
    if ((((((locals.var_guard1750 != 0.0) && (!((locals.var_guard1748 != 0.0) || (locals.var_guard1749 != 0.0)))) && (locals.var_guard1754 != 0.0)) && (locals.var_guard1755 != 0.0)) && (locals.var_guard1756 != 0.0)) && (locals.var_guard1758 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign75240_e114879;
        locals.var_t9_dn0 = assign75240_e114879_d_n0;
        locals.var_t9_dn2 = assign75240_e114879_d_n2;
        locals.var_t9_dn4 = assign75240_e114879_d_n4;
        locals.var_t9_dn5 = assign75240_e114879_d_n5;
        locals.var_t9_dn6 = assign75240_e114879_d_n6;
        locals.var_t9_dn7 = assign75240_e114879_d_n7;
        locals.var_t9_dn8 = assign75240_e114879_d_n8;
        locals.var_t9_dn9 = assign75240_e114879_d_n9;
        locals.var_t9_dn10 = assign75240_e114879_d_n10;
        locals.var_t9_dn13 = assign75240_e114879_d_n13;
        locals.var_t9_rv = 0.0;

        let (assign75250_e114899, assign75250_e114899_d_n0, assign75250_e114899_d_n2, assign75250_e114899_d_n4, assign75250_e114899_d_n5, assign75250_e114899_d_n6, assign75250_e114899_d_n7, assign75250_e114899_d_n8, assign75250_e114899_d_n9, assign75250_e114899_d_n10, assign75250_e114899_d_n13,) = {
    if (((((locals.var_guard1750 != 0.0) && (!((locals.var_guard1748 != 0.0) || (locals.var_guard1749 != 0.0)))) && (locals.var_guard1754 != 0.0)) && (locals.var_guard1755 != 0.0)) && (locals.var_guard1756 != 0.0)) {
        let assign75250_e114894: f64 = (locals.var_kjunc * locals.var_t2);
        let assign75250_e114895: f64 = (assign75250_e114894).sqrt();
        let assign75250_e114897: f64 = (assign75250_e114895 * p.p432);
        (assign75250_e114897, ((((locals.var_kjunc_dn0 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn0)) / (2.0 * assign75250_e114895)) * p.p432), ((((locals.var_kjunc_dn2 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn2)) / (2.0 * assign75250_e114895)) * p.p432), ((((locals.var_kjunc_dn4 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn4)) / (2.0 * assign75250_e114895)) * p.p432), ((((locals.var_kjunc_dn5 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn5)) / (2.0 * assign75250_e114895)) * p.p432), ((((locals.var_kjunc_dn6 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn6)) / (2.0 * assign75250_e114895)) * p.p432), ((((locals.var_kjunc_dn7 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn7)) / (2.0 * assign75250_e114895)) * p.p432), ((((locals.var_kjunc_dn8 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn8)) / (2.0 * assign75250_e114895)) * p.p432), ((((locals.var_kjunc_dn9 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn9)) / (2.0 * assign75250_e114895)) * p.p432), ((((locals.var_kjunc_dn10 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn10)) / (2.0 * assign75250_e114895)) * p.p432), ((((locals.var_kjunc_dn13 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn13)) / (2.0 * assign75250_e114895)) * p.p432),)
    } else {
        (locals.var_wjunc0, locals.var_wjunc0_dn0, locals.var_wjunc0_dn2, locals.var_wjunc0_dn4, locals.var_wjunc0_dn5, locals.var_wjunc0_dn6, locals.var_wjunc0_dn7, locals.var_wjunc0_dn8, locals.var_wjunc0_dn9, locals.var_wjunc0_dn10, locals.var_wjunc0_dn13,)
    }
};
        locals.var_wjunc0 = assign75250_e114899;
        locals.var_wjunc0_dn0 = assign75250_e114899_d_n0;
        locals.var_wjunc0_dn2 = assign75250_e114899_d_n2;
        locals.var_wjunc0_dn4 = assign75250_e114899_d_n4;
        locals.var_wjunc0_dn5 = assign75250_e114899_d_n5;
        locals.var_wjunc0_dn6 = assign75250_e114899_d_n6;
        locals.var_wjunc0_dn7 = assign75250_e114899_d_n7;
        locals.var_wjunc0_dn8 = assign75250_e114899_d_n8;
        locals.var_wjunc0_dn9 = assign75250_e114899_d_n9;
        locals.var_wjunc0_dn10 = assign75250_e114899_d_n10;
        locals.var_wjunc0_dn13 = assign75250_e114899_d_n13;
        locals.var_wjunc0_rv = 0.0;

        let (assign75260_e114916, assign75260_e114916_d_n0, assign75260_e114916_d_n2, assign75260_e114916_d_n4, assign75260_e114916_d_n5, assign75260_e114916_d_n6, assign75260_e114916_d_n7, assign75260_e114916_d_n8, assign75260_e114916_d_n9, assign75260_e114916_d_n10, assign75260_e114916_d_n13,) = {
    if (((((locals.var_guard1750 != 0.0) && (!((locals.var_guard1748 != 0.0) || (locals.var_guard1749 != 0.0)))) && (locals.var_guard1754 != 0.0)) && (locals.var_guard1755 != 0.0)) && (locals.var_guard1756 != 0.0)) {
        let assign75260_e114914: f64 = (locals.var_lover_func - locals.var_wjunc0);
        (assign75260_e114914, (locals.var_lover_func_dn0 - locals.var_wjunc0_dn0), (locals.var_lover_func_dn2 - locals.var_wjunc0_dn2), (locals.var_lover_func_dn4 - locals.var_wjunc0_dn4), (locals.var_lover_func_dn5 - locals.var_wjunc0_dn5), (locals.var_lover_func_dn6 - locals.var_wjunc0_dn6), (locals.var_lover_func_dn7 - locals.var_wjunc0_dn7), (locals.var_lover_func_dn8 - locals.var_wjunc0_dn8), (locals.var_lover_func_dn9 - locals.var_wjunc0_dn9), (locals.var_lover_func_dn10 - locals.var_wjunc0_dn10), (locals.var_lover_func_dn13 - locals.var_wjunc0_dn13),)
    } else {
        (locals.var_lover_func, locals.var_lover_func_dn0, locals.var_lover_func_dn2, locals.var_lover_func_dn4, locals.var_lover_func_dn5, locals.var_lover_func_dn6, locals.var_lover_func_dn7, locals.var_lover_func_dn8, locals.var_lover_func_dn9, locals.var_lover_func_dn10, locals.var_lover_func_dn13,)
    }
};
        locals.var_lover_func = assign75260_e114916;
        locals.var_lover_func_dn0 = assign75260_e114916_d_n0;
        locals.var_lover_func_dn2 = assign75260_e114916_d_n2;
        locals.var_lover_func_dn4 = assign75260_e114916_d_n4;
        locals.var_lover_func_dn5 = assign75260_e114916_d_n5;
        locals.var_lover_func_dn6 = assign75260_e114916_d_n6;
        locals.var_lover_func_dn7 = assign75260_e114916_d_n7;
        locals.var_lover_func_dn8 = assign75260_e114916_d_n8;
        locals.var_lover_func_dn9 = assign75260_e114916_d_n9;
        locals.var_lover_func_dn10 = assign75260_e114916_d_n10;
        locals.var_lover_func_dn13 = assign75260_e114916_d_n13;
        locals.var_lover_func_rv = 0.0;

        let assign75270_e114935: f64 = if (((((p.p35 == 1.0) && (p.p63 > 0.0)) && (locals.var_uc_nover > 0.0)) && (locals.var_uc_cvdsover != 0.0)) && (p.p55 != 1.0)) { 1.0 } else { 0.0 };
        locals.var_guard1759 = assign75270_e114935;
        locals.var_guard1759_rv = 0.0;

        let (assign75280_e114948,) = {
    if (((locals.var_guard1751 != 0.0) && (!(((locals.var_guard1748 != 0.0) || (locals.var_guard1749 != 0.0)) || (locals.var_guard1750 != 0.0)))) && (locals.var_guard1759 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_calcqover,)
    }
};
        locals.var_flg_calcqover = assign75280_e114948;
        locals.var_flg_calcqover_rv = 0.0;

        let (assign75290_e114963, assign75290_e114963_d_n2, assign75290_e114963_d_n6, assign75290_e114963_d_n7, assign75290_e114963_d_n8,) = {
    if (((locals.var_guard1751 != 0.0) && (!(((locals.var_guard1748 != 0.0) || (locals.var_guard1749 != 0.0)) || (locals.var_guard1750 != 0.0)))) && (locals.var_guard1759 != 0.0)) {
        let assign75290_e114961: f64 = (locals.var_vgsei - locals.var_vbsei);
        (assign75290_e114961, (locals.var_vgsei_dn2 - locals.var_vbsei_dn2), locals.var_vgsei_dn6, 0.0, (-locals.var_vbsei_dn8),)
    } else {
        (locals.var_vgbgmt, locals.var_vgbgmt_dn2, locals.var_vgbgmt_dn6, locals.var_vgbgmt_dn7, locals.var_vgbgmt_dn8,)
    }
};
        locals.var_vgbgmt = assign75290_e114963;
        locals.var_vgbgmt_dn2 = assign75290_e114963_d_n2;
        locals.var_vgbgmt_dn6 = assign75290_e114963_d_n6;
        locals.var_vgbgmt_dn7 = assign75290_e114963_d_n7;
        locals.var_vgbgmt_dn8 = assign75290_e114963_d_n8;
        locals.var_vgbgmt_rv = 0.0;

        let (assign75300_e114978, assign75300_e114978_d_n0, assign75300_e114978_d_n2, assign75300_e114978_d_n4, assign75300_e114978_d_n5, assign75300_e114978_d_n6, assign75300_e114978_d_n7, assign75300_e114978_d_n8, assign75300_e114978_d_n9, assign75300_e114978_d_n10, assign75300_e114978_d_n13,) = {
    if (((locals.var_guard1751 != 0.0) && (!(((locals.var_guard1748 != 0.0) || (locals.var_guard1749 != 0.0)) || (locals.var_guard1750 != 0.0)))) && (locals.var_guard1759 != 0.0)) {
        let assign75300_e114976: f64 = (locals.var_vdsei - locals.var_vbsei);
        (assign75300_e114976, locals.var_vdsei_dn0, (locals.var_vdsei_dn2 - locals.var_vbsei_dn2), 0.0, 0.0, 0.0, 0.0, (-locals.var_vbsei_dn8), 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vxbgmt, locals.var_vxbgmt_dn0, locals.var_vxbgmt_dn2, locals.var_vxbgmt_dn4, locals.var_vxbgmt_dn5, locals.var_vxbgmt_dn6, locals.var_vxbgmt_dn7, locals.var_vxbgmt_dn8, locals.var_vxbgmt_dn9, locals.var_vxbgmt_dn10, locals.var_vxbgmt_dn13,)
    }
};
        locals.var_vxbgmt = assign75300_e114978;
        locals.var_vxbgmt_dn0 = assign75300_e114978_d_n0;
        locals.var_vxbgmt_dn2 = assign75300_e114978_d_n2;
        locals.var_vxbgmt_dn4 = assign75300_e114978_d_n4;
        locals.var_vxbgmt_dn5 = assign75300_e114978_d_n5;
        locals.var_vxbgmt_dn6 = assign75300_e114978_d_n6;
        locals.var_vxbgmt_dn7 = assign75300_e114978_d_n7;
        locals.var_vxbgmt_dn8 = assign75300_e114978_d_n8;
        locals.var_vxbgmt_dn9 = assign75300_e114978_d_n9;
        locals.var_vxbgmt_dn10 = assign75300_e114978_d_n10;
        locals.var_vxbgmt_dn13 = assign75300_e114978_d_n13;
        locals.var_vxbgmt_rv = 0.0;

        let (assign75310_e114982, assign75310_e114982_d_n0, assign75310_e114982_d_n2, assign75310_e114982_d_n4, assign75310_e114982_d_n5, assign75310_e114982_d_n6, assign75310_e114982_d_n7, assign75310_e114982_d_n8, assign75310_e114982_d_n9, assign75310_e114982_d_n10, assign75310_e114982_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.4, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vbs_bnd_over__blk1767, locals.var_vbs_bnd_over__blk1767_dn0, locals.var_vbs_bnd_over__blk1767_dn2, locals.var_vbs_bnd_over__blk1767_dn4, locals.var_vbs_bnd_over__blk1767_dn5, locals.var_vbs_bnd_over__blk1767_dn6, locals.var_vbs_bnd_over__blk1767_dn7, locals.var_vbs_bnd_over__blk1767_dn8, locals.var_vbs_bnd_over__blk1767_dn9, locals.var_vbs_bnd_over__blk1767_dn10, locals.var_vbs_bnd_over__blk1767_dn13,)
    }
};
        locals.var_vbs_bnd_over__blk1767 = assign75310_e114982;
        locals.var_vbs_bnd_over__blk1767_dn0 = assign75310_e114982_d_n0;
        locals.var_vbs_bnd_over__blk1767_dn2 = assign75310_e114982_d_n2;
        locals.var_vbs_bnd_over__blk1767_dn4 = assign75310_e114982_d_n4;
        locals.var_vbs_bnd_over__blk1767_dn5 = assign75310_e114982_d_n5;
        locals.var_vbs_bnd_over__blk1767_dn6 = assign75310_e114982_d_n6;
        locals.var_vbs_bnd_over__blk1767_dn7 = assign75310_e114982_d_n7;
        locals.var_vbs_bnd_over__blk1767_dn8 = assign75310_e114982_d_n8;
        locals.var_vbs_bnd_over__blk1767_dn9 = assign75310_e114982_d_n9;
        locals.var_vbs_bnd_over__blk1767_dn10 = assign75310_e114982_d_n10;
        locals.var_vbs_bnd_over__blk1767_dn13 = assign75310_e114982_d_n13;
        locals.var_vbs_bnd_over__blk1767_rv = 0.0;

        let (assign75330_e114990,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0,)
    } else {
        (locals.var_flg_fd_mode__blk1768,)
    }
};
        locals.var_flg_fd_mode__blk1768 = assign75330_e114990;
        locals.var_flg_fd_mode__blk1768_rv = 0.0;

        let (assign75340_e114994, assign75340_e114994_d_n0, assign75340_e114994_d_n2, assign75340_e114994_d_n4, assign75340_e114994_d_n5, assign75340_e114994_d_n6, assign75340_e114994_d_n7, assign75340_e114994_d_n8, assign75340_e114994_d_n9, assign75340_e114994_d_n10, assign75340_e114994_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn4, locals.var_fb_dn5, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn8, locals.var_fb_dn9, locals.var_fb_dn10, locals.var_fb_dn13,)
    }
};
        locals.var_fb = assign75340_e114994;
        locals.var_fb_dn0 = assign75340_e114994_d_n0;
        locals.var_fb_dn2 = assign75340_e114994_d_n2;
        locals.var_fb_dn4 = assign75340_e114994_d_n4;
        locals.var_fb_dn5 = assign75340_e114994_d_n5;
        locals.var_fb_dn6 = assign75340_e114994_d_n6;
        locals.var_fb_dn7 = assign75340_e114994_d_n7;
        locals.var_fb_dn8 = assign75340_e114994_d_n8;
        locals.var_fb_dn9 = assign75340_e114994_d_n9;
        locals.var_fb_dn10 = assign75340_e114994_d_n10;
        locals.var_fb_dn13 = assign75340_e114994_d_n13;
        locals.var_fb_rv = 0.0;

        let (assign75350_e114998, assign75350_e114998_d_n0, assign75350_e114998_d_n2, assign75350_e114998_d_n4, assign75350_e114998_d_n5, assign75350_e114998_d_n6, assign75350_e114998_d_n7, assign75350_e114998_d_n8, assign75350_e114998_d_n9, assign75350_e114998_d_n10, assign75350_e114998_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fs01, locals.var_fs01_dn0, locals.var_fs01_dn2, locals.var_fs01_dn4, locals.var_fs01_dn5, locals.var_fs01_dn6, locals.var_fs01_dn7, locals.var_fs01_dn8, locals.var_fs01_dn9, locals.var_fs01_dn10, locals.var_fs01_dn13,)
    }
};
        locals.var_fs01 = assign75350_e114998;
        locals.var_fs01_dn0 = assign75350_e114998_d_n0;
        locals.var_fs01_dn2 = assign75350_e114998_d_n2;
        locals.var_fs01_dn4 = assign75350_e114998_d_n4;
        locals.var_fs01_dn5 = assign75350_e114998_d_n5;
        locals.var_fs01_dn6 = assign75350_e114998_d_n6;
        locals.var_fs01_dn7 = assign75350_e114998_d_n7;
        locals.var_fs01_dn8 = assign75350_e114998_d_n8;
        locals.var_fs01_dn9 = assign75350_e114998_d_n9;
        locals.var_fs01_dn10 = assign75350_e114998_d_n10;
        locals.var_fs01_dn13 = assign75350_e114998_d_n13;
        locals.var_fs01_rv = 0.0;

        let (assign75360_e115002, assign75360_e115002_d_n0, assign75360_e115002_d_n2, assign75360_e115002_d_n4, assign75360_e115002_d_n5, assign75360_e115002_d_n6, assign75360_e115002_d_n7, assign75360_e115002_d_n8, assign75360_e115002_d_n9, assign75360_e115002_d_n10, assign75360_e115002_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fs02, locals.var_fs02_dn0, locals.var_fs02_dn2, locals.var_fs02_dn4, locals.var_fs02_dn5, locals.var_fs02_dn6, locals.var_fs02_dn7, locals.var_fs02_dn8, locals.var_fs02_dn9, locals.var_fs02_dn10, locals.var_fs02_dn13,)
    }
};
        locals.var_fs02 = assign75360_e115002;
        locals.var_fs02_dn0 = assign75360_e115002_d_n0;
        locals.var_fs02_dn2 = assign75360_e115002_d_n2;
        locals.var_fs02_dn4 = assign75360_e115002_d_n4;
        locals.var_fs02_dn5 = assign75360_e115002_d_n5;
        locals.var_fs02_dn6 = assign75360_e115002_d_n6;
        locals.var_fs02_dn7 = assign75360_e115002_d_n7;
        locals.var_fs02_dn8 = assign75360_e115002_d_n8;
        locals.var_fs02_dn9 = assign75360_e115002_d_n9;
        locals.var_fs02_dn10 = assign75360_e115002_d_n10;
        locals.var_fs02_dn13 = assign75360_e115002_d_n13;
        locals.var_fs02_rv = 0.0;

        let (assign75370_e115006, assign75370_e115006_d_n0, assign75370_e115006_d_n2, assign75370_e115006_d_n4, assign75370_e115006_d_n5, assign75370_e115006_d_n6, assign75370_e115006_d_n7, assign75370_e115006_d_n8, assign75370_e115006_d_n9, assign75370_e115006_d_n10, assign75370_e115006_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fs0, locals.var_fs0_dn0, locals.var_fs0_dn2, locals.var_fs0_dn4, locals.var_fs0_dn5, locals.var_fs0_dn6, locals.var_fs0_dn7, locals.var_fs0_dn8, locals.var_fs0_dn9, locals.var_fs0_dn10, locals.var_fs0_dn13,)
    }
};
        locals.var_fs0 = assign75370_e115006;
        locals.var_fs0_dn0 = assign75370_e115006_d_n0;
        locals.var_fs0_dn2 = assign75370_e115006_d_n2;
        locals.var_fs0_dn4 = assign75370_e115006_d_n4;
        locals.var_fs0_dn5 = assign75370_e115006_d_n5;
        locals.var_fs0_dn6 = assign75370_e115006_d_n6;
        locals.var_fs0_dn7 = assign75370_e115006_d_n7;
        locals.var_fs0_dn8 = assign75370_e115006_d_n8;
        locals.var_fs0_dn9 = assign75370_e115006_d_n9;
        locals.var_fs0_dn10 = assign75370_e115006_d_n10;
        locals.var_fs0_dn13 = assign75370_e115006_d_n13;
        locals.var_fs0_rv = 0.0;

        let (assign75380_e115010, assign75380_e115010_d_n0, assign75380_e115010_d_n2, assign75380_e115010_d_n4, assign75380_e115010_d_n5, assign75380_e115010_d_n6, assign75380_e115010_d_n7, assign75380_e115010_d_n8, assign75380_e115010_d_n9, assign75380_e115010_d_n10, assign75380_e115010_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dps0, locals.var_dps0_dn0, locals.var_dps0_dn2, locals.var_dps0_dn4, locals.var_dps0_dn5, locals.var_dps0_dn6, locals.var_dps0_dn7, locals.var_dps0_dn8, locals.var_dps0_dn9, locals.var_dps0_dn10, locals.var_dps0_dn13,)
    }
};
        locals.var_dps0 = assign75380_e115010;
        locals.var_dps0_dn0 = assign75380_e115010_d_n0;
        locals.var_dps0_dn2 = assign75380_e115010_d_n2;
        locals.var_dps0_dn4 = assign75380_e115010_d_n4;
        locals.var_dps0_dn5 = assign75380_e115010_d_n5;
        locals.var_dps0_dn6 = assign75380_e115010_d_n6;
        locals.var_dps0_dn7 = assign75380_e115010_d_n7;
        locals.var_dps0_dn8 = assign75380_e115010_d_n8;
        locals.var_dps0_dn9 = assign75380_e115010_d_n9;
        locals.var_dps0_dn10 = assign75380_e115010_d_n10;
        locals.var_dps0_dn13 = assign75380_e115010_d_n13;
        locals.var_dps0_rv = 0.0;

        let (assign75390_e115014, assign75390_e115014_d_n0, assign75390_e115014_d_n2, assign75390_e115014_d_n4, assign75390_e115014_d_n5, assign75390_e115014_d_n6, assign75390_e115014_d_n7, assign75390_e115014_d_n8, assign75390_e115014_d_n9, assign75390_e115014_d_n10, assign75390_e115014_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fs0_dps0, locals.var_fs0_dps0_dn0, locals.var_fs0_dps0_dn2, locals.var_fs0_dps0_dn4, locals.var_fs0_dps0_dn5, locals.var_fs0_dps0_dn6, locals.var_fs0_dps0_dn7, locals.var_fs0_dps0_dn8, locals.var_fs0_dps0_dn9, locals.var_fs0_dps0_dn10, locals.var_fs0_dps0_dn13,)
    }
};
        locals.var_fs0_dps0 = assign75390_e115014;
        locals.var_fs0_dps0_dn0 = assign75390_e115014_d_n0;
        locals.var_fs0_dps0_dn2 = assign75390_e115014_d_n2;
        locals.var_fs0_dps0_dn4 = assign75390_e115014_d_n4;
        locals.var_fs0_dps0_dn5 = assign75390_e115014_d_n5;
        locals.var_fs0_dps0_dn6 = assign75390_e115014_d_n6;
        locals.var_fs0_dps0_dn7 = assign75390_e115014_d_n7;
        locals.var_fs0_dps0_dn8 = assign75390_e115014_d_n8;
        locals.var_fs0_dps0_dn9 = assign75390_e115014_d_n9;
        locals.var_fs0_dps0_dn10 = assign75390_e115014_d_n10;
        locals.var_fs0_dps0_dn13 = assign75390_e115014_d_n13;
        locals.var_fs0_dps0_rv = 0.0;

        let (assign75400_e115018, assign75400_e115018_d_n0, assign75400_e115018_d_n2, assign75400_e115018_d_n4, assign75400_e115018_d_n5, assign75400_e115018_d_n6, assign75400_e115018_d_n7, assign75400_e115018_d_n8, assign75400_e115018_d_n9, assign75400_e115018_d_n10, assign75400_e115018_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fs02_dps0, locals.var_fs02_dps0_dn0, locals.var_fs02_dps0_dn2, locals.var_fs02_dps0_dn4, locals.var_fs02_dps0_dn5, locals.var_fs02_dps0_dn6, locals.var_fs02_dps0_dn7, locals.var_fs02_dps0_dn8, locals.var_fs02_dps0_dn9, locals.var_fs02_dps0_dn10, locals.var_fs02_dps0_dn13,)
    }
};
        locals.var_fs02_dps0 = assign75400_e115018;
        locals.var_fs02_dps0_dn0 = assign75400_e115018_d_n0;
        locals.var_fs02_dps0_dn2 = assign75400_e115018_d_n2;
        locals.var_fs02_dps0_dn4 = assign75400_e115018_d_n4;
        locals.var_fs02_dps0_dn5 = assign75400_e115018_d_n5;
        locals.var_fs02_dps0_dn6 = assign75400_e115018_d_n6;
        locals.var_fs02_dps0_dn7 = assign75400_e115018_d_n7;
        locals.var_fs02_dps0_dn8 = assign75400_e115018_d_n8;
        locals.var_fs02_dps0_dn9 = assign75400_e115018_d_n9;
        locals.var_fs02_dps0_dn10 = assign75400_e115018_d_n10;
        locals.var_fs02_dps0_dn13 = assign75400_e115018_d_n13;
        locals.var_fs02_dps0_rv = 0.0;

        let (assign75410_e115022, assign75410_e115022_d_n0, assign75410_e115022_d_n2, assign75410_e115022_d_n4, assign75410_e115022_d_n5, assign75410_e115022_d_n6, assign75410_e115022_d_n7, assign75410_e115022_d_n8, assign75410_e115022_d_n9, assign75410_e115022_d_n10, assign75410_e115022_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fb_dpss, locals.var_fb_dpss_dn0, locals.var_fb_dpss_dn2, locals.var_fb_dpss_dn4, locals.var_fb_dpss_dn5, locals.var_fb_dpss_dn6, locals.var_fb_dpss_dn7, locals.var_fb_dpss_dn8, locals.var_fb_dpss_dn9, locals.var_fb_dpss_dn10, locals.var_fb_dpss_dn13,)
    }
};
        locals.var_fb_dpss = assign75410_e115022;
        locals.var_fb_dpss_dn0 = assign75410_e115022_d_n0;
        locals.var_fb_dpss_dn2 = assign75410_e115022_d_n2;
        locals.var_fb_dpss_dn4 = assign75410_e115022_d_n4;
        locals.var_fb_dpss_dn5 = assign75410_e115022_d_n5;
        locals.var_fb_dpss_dn6 = assign75410_e115022_d_n6;
        locals.var_fb_dpss_dn7 = assign75410_e115022_d_n7;
        locals.var_fb_dpss_dn8 = assign75410_e115022_d_n8;
        locals.var_fb_dpss_dn9 = assign75410_e115022_d_n9;
        locals.var_fb_dpss_dn10 = assign75410_e115022_d_n10;
        locals.var_fb_dpss_dn13 = assign75410_e115022_d_n13;
        locals.var_fb_dpss_rv = 0.0;

        let (assign75420_e115026, assign75420_e115026_d_n0, assign75420_e115026_d_n2, assign75420_e115026_d_n4, assign75420_e115026_d_n5, assign75420_e115026_d_n6, assign75420_e115026_d_n7, assign75420_e115026_d_n8, assign75420_e115026_d_n9, assign75420_e115026_d_n10, assign75420_e115026_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fs01_dps0, locals.var_fs01_dps0_dn0, locals.var_fs01_dps0_dn2, locals.var_fs01_dps0_dn4, locals.var_fs01_dps0_dn5, locals.var_fs01_dps0_dn6, locals.var_fs01_dps0_dn7, locals.var_fs01_dps0_dn8, locals.var_fs01_dps0_dn9, locals.var_fs01_dps0_dn10, locals.var_fs01_dps0_dn13,)
    }
};
        locals.var_fs01_dps0 = assign75420_e115026;
        locals.var_fs01_dps0_dn0 = assign75420_e115026_d_n0;
        locals.var_fs01_dps0_dn2 = assign75420_e115026_d_n2;
        locals.var_fs01_dps0_dn4 = assign75420_e115026_d_n4;
        locals.var_fs01_dps0_dn5 = assign75420_e115026_d_n5;
        locals.var_fs01_dps0_dn6 = assign75420_e115026_d_n6;
        locals.var_fs01_dps0_dn7 = assign75420_e115026_d_n7;
        locals.var_fs01_dps0_dn8 = assign75420_e115026_d_n8;
        locals.var_fs01_dps0_dn9 = assign75420_e115026_d_n9;
        locals.var_fs01_dps0_dn10 = assign75420_e115026_d_n10;
        locals.var_fs01_dps0_dn13 = assign75420_e115026_d_n13;
        locals.var_fs01_dps0_rv = 0.0;

        let (assign75430_e115030, assign75430_e115030_d_n0, assign75430_e115030_d_n2, assign75430_e115030_d_n4, assign75430_e115030_d_n5, assign75430_e115030_d_n6, assign75430_e115030_d_n7, assign75430_e115030_d_n8, assign75430_e115030_d_n9, assign75430_e115030_d_n10, assign75430_e115030_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_chi_1, locals.var_chi_1_dn0, locals.var_chi_1_dn2, locals.var_chi_1_dn4, locals.var_chi_1_dn5, locals.var_chi_1_dn6, locals.var_chi_1_dn7, locals.var_chi_1_dn8, locals.var_chi_1_dn9, locals.var_chi_1_dn10, locals.var_chi_1_dn13,)
    }
};
        locals.var_chi_1 = assign75430_e115030;
        locals.var_chi_1_dn0 = assign75430_e115030_d_n0;
        locals.var_chi_1_dn2 = assign75430_e115030_d_n2;
        locals.var_chi_1_dn4 = assign75430_e115030_d_n4;
        locals.var_chi_1_dn5 = assign75430_e115030_d_n5;
        locals.var_chi_1_dn6 = assign75430_e115030_d_n6;
        locals.var_chi_1_dn7 = assign75430_e115030_d_n7;
        locals.var_chi_1_dn8 = assign75430_e115030_d_n8;
        locals.var_chi_1_dn9 = assign75430_e115030_d_n9;
        locals.var_chi_1_dn10 = assign75430_e115030_d_n10;
        locals.var_chi_1_dn13 = assign75430_e115030_d_n13;
        locals.var_chi_1_rv = 0.0;

        let (assign75440_e115034, assign75440_e115034_d_n0, assign75440_e115034_d_n2, assign75440_e115034_d_n4, assign75440_e115034_d_n5, assign75440_e115034_d_n6, assign75440_e115034_d_n7, assign75440_e115034_d_n8, assign75440_e115034_d_n9, assign75440_e115034_d_n10, assign75440_e115034_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_chi_a, locals.var_chi_a_dn0, locals.var_chi_a_dn2, locals.var_chi_a_dn4, locals.var_chi_a_dn5, locals.var_chi_a_dn6, locals.var_chi_a_dn7, locals.var_chi_a_dn8, locals.var_chi_a_dn9, locals.var_chi_a_dn10, locals.var_chi_a_dn13,)
    }
};
        locals.var_chi_a = assign75440_e115034;
        locals.var_chi_a_dn0 = assign75440_e115034_d_n0;
        locals.var_chi_a_dn2 = assign75440_e115034_d_n2;
        locals.var_chi_a_dn4 = assign75440_e115034_d_n4;
        locals.var_chi_a_dn5 = assign75440_e115034_d_n5;
        locals.var_chi_a_dn6 = assign75440_e115034_d_n6;
        locals.var_chi_a_dn7 = assign75440_e115034_d_n7;
        locals.var_chi_a_dn8 = assign75440_e115034_d_n8;
        locals.var_chi_a_dn9 = assign75440_e115034_d_n9;
        locals.var_chi_a_dn10 = assign75440_e115034_d_n10;
        locals.var_chi_a_dn13 = assign75440_e115034_d_n13;
        locals.var_chi_a_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_274(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        locals: &mut StampLocals,
    ) {
        let (assign75450_e115038, assign75450_e115038_d_n0, assign75450_e115038_d_n2, assign75450_e115038_d_n4, assign75450_e115038_d_n5, assign75450_e115038_d_n6, assign75450_e115038_d_n7, assign75450_e115038_d_n8, assign75450_e115038_d_n9, assign75450_e115038_d_n10, assign75450_e115038_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_chi_b, locals.var_chi_b_dn0, locals.var_chi_b_dn2, locals.var_chi_b_dn4, locals.var_chi_b_dn5, locals.var_chi_b_dn6, locals.var_chi_b_dn7, locals.var_chi_b_dn8, locals.var_chi_b_dn9, locals.var_chi_b_dn10, locals.var_chi_b_dn13,)
    }
};
        locals.var_chi_b = assign75450_e115038;
        locals.var_chi_b_dn0 = assign75450_e115038_d_n0;
        locals.var_chi_b_dn2 = assign75450_e115038_d_n2;
        locals.var_chi_b_dn4 = assign75450_e115038_d_n4;
        locals.var_chi_b_dn5 = assign75450_e115038_d_n5;
        locals.var_chi_b_dn6 = assign75450_e115038_d_n6;
        locals.var_chi_b_dn7 = assign75450_e115038_d_n7;
        locals.var_chi_b_dn8 = assign75450_e115038_d_n8;
        locals.var_chi_b_dn9 = assign75450_e115038_d_n9;
        locals.var_chi_b_dn10 = assign75450_e115038_d_n10;
        locals.var_chi_b_dn13 = assign75450_e115038_d_n13;
        locals.var_chi_b_rv = 0.0;

        let (assign75460_e115043,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign75460_e115041: f64 = (-1.0);
        (assign75460_e115041,)
    } else {
        (locals.var_flg_conv,)
    }
};
        locals.var_flg_conv = assign75460_e115043;
        locals.var_flg_conv_rv = 0.0;

        let (assign75470_e115047, assign75470_e115047_d_n0, assign75470_e115047_d_n2, assign75470_e115047_d_n4, assign75470_e115047_d_n5, assign75470_e115047_d_n6, assign75470_e115047_d_n7, assign75470_e115047_d_n8, assign75470_e115047_d_n9, assign75470_e115047_d_n10, assign75470_e115047_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ps0ld_ini__blk1769, locals.var_ps0ld_ini__blk1769_dn0, locals.var_ps0ld_ini__blk1769_dn2, locals.var_ps0ld_ini__blk1769_dn4, locals.var_ps0ld_ini__blk1769_dn5, locals.var_ps0ld_ini__blk1769_dn6, locals.var_ps0ld_ini__blk1769_dn7, locals.var_ps0ld_ini__blk1769_dn8, locals.var_ps0ld_ini__blk1769_dn9, locals.var_ps0ld_ini__blk1769_dn10, locals.var_ps0ld_ini__blk1769_dn13,)
    }
};
        locals.var_ps0ld_ini__blk1769 = assign75470_e115047;
        locals.var_ps0ld_ini__blk1769_dn0 = assign75470_e115047_d_n0;
        locals.var_ps0ld_ini__blk1769_dn2 = assign75470_e115047_d_n2;
        locals.var_ps0ld_ini__blk1769_dn4 = assign75470_e115047_d_n4;
        locals.var_ps0ld_ini__blk1769_dn5 = assign75470_e115047_d_n5;
        locals.var_ps0ld_ini__blk1769_dn6 = assign75470_e115047_d_n6;
        locals.var_ps0ld_ini__blk1769_dn7 = assign75470_e115047_d_n7;
        locals.var_ps0ld_ini__blk1769_dn8 = assign75470_e115047_d_n8;
        locals.var_ps0ld_ini__blk1769_dn9 = assign75470_e115047_d_n9;
        locals.var_ps0ld_ini__blk1769_dn10 = assign75470_e115047_d_n10;
        locals.var_ps0ld_ini__blk1769_dn13 = assign75470_e115047_d_n13;
        locals.var_ps0ld_ini__blk1769_rv = 0.0;

        let (assign75480_e115051, assign75480_e115051_d_n0, assign75480_e115051_d_n2, assign75480_e115051_d_n4, assign75480_e115051_d_n5, assign75480_e115051_d_n6, assign75480_e115051_d_n7, assign75480_e115051_d_n8, assign75480_e115051_d_n9, assign75480_e115051_d_n10, assign75480_e115051_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fbsq__blk1770, locals.var_fbsq__blk1770_dn0, locals.var_fbsq__blk1770_dn2, locals.var_fbsq__blk1770_dn4, locals.var_fbsq__blk1770_dn5, locals.var_fbsq__blk1770_dn6, locals.var_fbsq__blk1770_dn7, locals.var_fbsq__blk1770_dn8, locals.var_fbsq__blk1770_dn9, locals.var_fbsq__blk1770_dn10, locals.var_fbsq__blk1770_dn13,)
    }
};
        locals.var_fbsq__blk1770 = assign75480_e115051;
        locals.var_fbsq__blk1770_dn0 = assign75480_e115051_d_n0;
        locals.var_fbsq__blk1770_dn2 = assign75480_e115051_d_n2;
        locals.var_fbsq__blk1770_dn4 = assign75480_e115051_d_n4;
        locals.var_fbsq__blk1770_dn5 = assign75480_e115051_d_n5;
        locals.var_fbsq__blk1770_dn6 = assign75480_e115051_d_n6;
        locals.var_fbsq__blk1770_dn7 = assign75480_e115051_d_n7;
        locals.var_fbsq__blk1770_dn8 = assign75480_e115051_d_n8;
        locals.var_fbsq__blk1770_dn9 = assign75480_e115051_d_n9;
        locals.var_fbsq__blk1770_dn10 = assign75480_e115051_d_n10;
        locals.var_fbsq__blk1770_dn13 = assign75480_e115051_d_n13;
        locals.var_fbsq__blk1770_rv = 0.0;

        let (assign75490_e115062, assign75490_e115062_d_n0, assign75490_e115062_d_n2, assign75490_e115062_d_n4, assign75490_e115062_d_n5, assign75490_e115062_d_n6, assign75490_e115062_d_n7, assign75490_e115062_d_n8, assign75490_e115062_d_n9, assign75490_e115062_d_n10, assign75490_e115062_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign75490_e115055: f64 = (2.0 * locals.var_beta_inv);
        let assign75490_e115058: f64 = (locals.var_nover_func / locals.var_nin);
        let assign75490_e115059: f64 = (assign75490_e115058).ln();
        let assign75490_e115060: f64 = (assign75490_e115055 * assign75490_e115059);
        (assign75490_e115060, (((2.0 * locals.var_beta_inv_dn0) * assign75490_e115059) + (assign75490_e115055 * ((-((locals.var_nover_func * locals.var_nin_dn0) / (locals.var_nin * locals.var_nin))) / assign75490_e115058))), (((2.0 * locals.var_beta_inv_dn2) * assign75490_e115059) + (assign75490_e115055 * ((-((locals.var_nover_func * locals.var_nin_dn2) / (locals.var_nin * locals.var_nin))) / assign75490_e115058))), (((2.0 * locals.var_beta_inv_dn4) * assign75490_e115059) + (assign75490_e115055 * ((-((locals.var_nover_func * locals.var_nin_dn4) / (locals.var_nin * locals.var_nin))) / assign75490_e115058))), (((2.0 * locals.var_beta_inv_dn5) * assign75490_e115059) + (assign75490_e115055 * ((-((locals.var_nover_func * locals.var_nin_dn5) / (locals.var_nin * locals.var_nin))) / assign75490_e115058))), (((2.0 * locals.var_beta_inv_dn6) * assign75490_e115059) + (assign75490_e115055 * ((-((locals.var_nover_func * locals.var_nin_dn6) / (locals.var_nin * locals.var_nin))) / assign75490_e115058))), (((2.0 * locals.var_beta_inv_dn7) * assign75490_e115059) + (assign75490_e115055 * ((-((locals.var_nover_func * locals.var_nin_dn7) / (locals.var_nin * locals.var_nin))) / assign75490_e115058))), (((2.0 * locals.var_beta_inv_dn8) * assign75490_e115059) + (assign75490_e115055 * ((-((locals.var_nover_func * locals.var_nin_dn8) / (locals.var_nin * locals.var_nin))) / assign75490_e115058))), (((2.0 * locals.var_beta_inv_dn9) * assign75490_e115059) + (assign75490_e115055 * ((-((locals.var_nover_func * locals.var_nin_dn9) / (locals.var_nin * locals.var_nin))) / assign75490_e115058))), (((2.0 * locals.var_beta_inv_dn10) * assign75490_e115059) + (assign75490_e115055 * ((-((locals.var_nover_func * locals.var_nin_dn10) / (locals.var_nin * locals.var_nin))) / assign75490_e115058))), (((2.0 * locals.var_beta_inv_dn13) * assign75490_e115059) + (assign75490_e115055 * ((-((locals.var_nover_func * locals.var_nin_dn13) / (locals.var_nin * locals.var_nin))) / assign75490_e115058))),)
    } else {
        (locals.var_pb2over__blk1765, locals.var_pb2over__blk1765_dn0, locals.var_pb2over__blk1765_dn2, locals.var_pb2over__blk1765_dn4, locals.var_pb2over__blk1765_dn5, locals.var_pb2over__blk1765_dn6, locals.var_pb2over__blk1765_dn7, locals.var_pb2over__blk1765_dn8, locals.var_pb2over__blk1765_dn9, locals.var_pb2over__blk1765_dn10, locals.var_pb2over__blk1765_dn13,)
    }
};
        locals.var_pb2over__blk1765 = assign75490_e115062;
        locals.var_pb2over__blk1765_dn0 = assign75490_e115062_d_n0;
        locals.var_pb2over__blk1765_dn2 = assign75490_e115062_d_n2;
        locals.var_pb2over__blk1765_dn4 = assign75490_e115062_d_n4;
        locals.var_pb2over__blk1765_dn5 = assign75490_e115062_d_n5;
        locals.var_pb2over__blk1765_dn6 = assign75490_e115062_d_n6;
        locals.var_pb2over__blk1765_dn7 = assign75490_e115062_d_n7;
        locals.var_pb2over__blk1765_dn8 = assign75490_e115062_d_n8;
        locals.var_pb2over__blk1765_dn9 = assign75490_e115062_d_n9;
        locals.var_pb2over__blk1765_dn10 = assign75490_e115062_d_n10;
        locals.var_pb2over__blk1765_dn13 = assign75490_e115062_d_n13;
        locals.var_pb2over__blk1765_rv = 0.0;

        let (assign75500_e115070, assign75500_e115070_d_n0, assign75500_e115070_d_n2, assign75500_e115070_d_n4, assign75500_e115070_d_n5, assign75500_e115070_d_n6, assign75500_e115070_d_n7, assign75500_e115070_d_n8, assign75500_e115070_d_n9, assign75500_e115070_d_n10, assign75500_e115070_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign75500_e115066: f64 = (0.8 - locals.var_pb2over__blk1765);
        let assign75500_e115068: f64 = (assign75500_e115066 - 0.1);
        (assign75500_e115068, (-locals.var_pb2over__blk1765_dn0), (-locals.var_pb2over__blk1765_dn2), (-locals.var_pb2over__blk1765_dn4), (-locals.var_pb2over__blk1765_dn5), (-locals.var_pb2over__blk1765_dn6), (-locals.var_pb2over__blk1765_dn7), (-locals.var_pb2over__blk1765_dn8), (-locals.var_pb2over__blk1765_dn9), (-locals.var_pb2over__blk1765_dn10), (-locals.var_pb2over__blk1765_dn13),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign75500_e115070;
        locals.var_tmf1_dn0 = assign75500_e115070_d_n0;
        locals.var_tmf1_dn2 = assign75500_e115070_d_n2;
        locals.var_tmf1_dn4 = assign75500_e115070_d_n4;
        locals.var_tmf1_dn5 = assign75500_e115070_d_n5;
        locals.var_tmf1_dn6 = assign75500_e115070_d_n6;
        locals.var_tmf1_dn7 = assign75500_e115070_d_n7;
        locals.var_tmf1_dn8 = assign75500_e115070_d_n8;
        locals.var_tmf1_dn9 = assign75500_e115070_d_n9;
        locals.var_tmf1_dn10 = assign75500_e115070_d_n10;
        locals.var_tmf1_dn13 = assign75500_e115070_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign75510_e115078, assign75510_e115078_d_n0, assign75510_e115078_d_n2, assign75510_e115078_d_n4, assign75510_e115078_d_n5, assign75510_e115078_d_n6, assign75510_e115078_d_n7, assign75510_e115078_d_n8, assign75510_e115078_d_n9, assign75510_e115078_d_n10, assign75510_e115078_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign75510_e115074: f64 = (4.0 * 0.8);
        let assign75510_e115076: f64 = (assign75510_e115074 * 0.1);
        (assign75510_e115076, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign75510_e115078;
        locals.var_tmf2_dn0 = assign75510_e115078_d_n0;
        locals.var_tmf2_dn2 = assign75510_e115078_d_n2;
        locals.var_tmf2_dn4 = assign75510_e115078_d_n4;
        locals.var_tmf2_dn5 = assign75510_e115078_d_n5;
        locals.var_tmf2_dn6 = assign75510_e115078_d_n6;
        locals.var_tmf2_dn7 = assign75510_e115078_d_n7;
        locals.var_tmf2_dn8 = assign75510_e115078_d_n8;
        locals.var_tmf2_dn9 = assign75510_e115078_d_n9;
        locals.var_tmf2_dn10 = assign75510_e115078_d_n10;
        locals.var_tmf2_dn13 = assign75510_e115078_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign75520_e115088, assign75520_e115088_d_n0, assign75520_e115088_d_n2, assign75520_e115088_d_n4, assign75520_e115088_d_n5, assign75520_e115088_d_n6, assign75520_e115088_d_n7, assign75520_e115088_d_n8, assign75520_e115088_d_n9, assign75520_e115088_d_n10, assign75520_e115088_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let (assign75520_e115086, assign75520_e115086_d_n0, assign75520_e115086_d_n2, assign75520_e115086_d_n4, assign75520_e115086_d_n5, assign75520_e115086_d_n6, assign75520_e115086_d_n7, assign75520_e115086_d_n8, assign75520_e115086_d_n9, assign75520_e115086_d_n10, assign75520_e115086_d_n13,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
            } else {
                let assign75520_e115085: f64 = (-locals.var_tmf2);
                (assign75520_e115085, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn13),)
            }
        };
        (assign75520_e115086, assign75520_e115086_d_n0, assign75520_e115086_d_n2, assign75520_e115086_d_n4, assign75520_e115086_d_n5, assign75520_e115086_d_n6, assign75520_e115086_d_n7, assign75520_e115086_d_n8, assign75520_e115086_d_n9, assign75520_e115086_d_n10, assign75520_e115086_d_n13,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign75520_e115088;
        locals.var_tmf2_dn0 = assign75520_e115088_d_n0;
        locals.var_tmf2_dn2 = assign75520_e115088_d_n2;
        locals.var_tmf2_dn4 = assign75520_e115088_d_n4;
        locals.var_tmf2_dn5 = assign75520_e115088_d_n5;
        locals.var_tmf2_dn6 = assign75520_e115088_d_n6;
        locals.var_tmf2_dn7 = assign75520_e115088_d_n7;
        locals.var_tmf2_dn8 = assign75520_e115088_d_n8;
        locals.var_tmf2_dn9 = assign75520_e115088_d_n9;
        locals.var_tmf2_dn10 = assign75520_e115088_d_n10;
        locals.var_tmf2_dn13 = assign75520_e115088_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign75530_e115097, assign75530_e115097_d_n0, assign75530_e115097_d_n2, assign75530_e115097_d_n4, assign75530_e115097_d_n5, assign75530_e115097_d_n6, assign75530_e115097_d_n7, assign75530_e115097_d_n8, assign75530_e115097_d_n9, assign75530_e115097_d_n10, assign75530_e115097_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign75530_e115092: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign75530_e115094: f64 = (assign75530_e115092 + locals.var_tmf2);
        let assign75530_e115095: f64 = (assign75530_e115094).sqrt();
        (assign75530_e115095, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign75530_e115095)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign75530_e115095)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign75530_e115095)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign75530_e115095)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign75530_e115095)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign75530_e115095)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign75530_e115095)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign75530_e115095)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign75530_e115095)), ((((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)) + locals.var_tmf2_dn13) / (2.0 * assign75530_e115095)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign75530_e115097;
        locals.var_tmf2_dn0 = assign75530_e115097_d_n0;
        locals.var_tmf2_dn2 = assign75530_e115097_d_n2;
        locals.var_tmf2_dn4 = assign75530_e115097_d_n4;
        locals.var_tmf2_dn5 = assign75530_e115097_d_n5;
        locals.var_tmf2_dn6 = assign75530_e115097_d_n6;
        locals.var_tmf2_dn7 = assign75530_e115097_d_n7;
        locals.var_tmf2_dn8 = assign75530_e115097_d_n8;
        locals.var_tmf2_dn9 = assign75530_e115097_d_n9;
        locals.var_tmf2_dn10 = assign75530_e115097_d_n10;
        locals.var_tmf2_dn13 = assign75530_e115097_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign75540_e115107, assign75540_e115107_d_n0, assign75540_e115107_d_n2, assign75540_e115107_d_n4, assign75540_e115107_d_n5, assign75540_e115107_d_n6, assign75540_e115107_d_n7, assign75540_e115107_d_n8, assign75540_e115107_d_n9, assign75540_e115107_d_n10, assign75540_e115107_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign75540_e115103: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign75540_e115104: f64 = (1.0 + assign75540_e115103);
        let assign75540_e115105: f64 = (0.5 * assign75540_e115104);
        (assign75540_e115105, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn13 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign75540_e115107;
        locals.var_t0_dn0 = assign75540_e115107_d_n0;
        locals.var_t0_dn2 = assign75540_e115107_d_n2;
        locals.var_t0_dn4 = assign75540_e115107_d_n4;
        locals.var_t0_dn5 = assign75540_e115107_d_n5;
        locals.var_t0_dn6 = assign75540_e115107_d_n6;
        locals.var_t0_dn7 = assign75540_e115107_d_n7;
        locals.var_t0_dn8 = assign75540_e115107_d_n8;
        locals.var_t0_dn9 = assign75540_e115107_d_n9;
        locals.var_t0_dn10 = assign75540_e115107_d_n10;
        locals.var_t0_dn13 = assign75540_e115107_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign75550_e115117, assign75550_e115117_d_n0, assign75550_e115117_d_n2, assign75550_e115117_d_n4, assign75550_e115117_d_n5, assign75550_e115117_d_n6, assign75550_e115117_d_n7, assign75550_e115117_d_n8, assign75550_e115117_d_n9, assign75550_e115117_d_n10, assign75550_e115117_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign75550_e115113: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign75550_e115114: f64 = (0.5 * assign75550_e115113);
        let assign75550_e115115: f64 = (0.8 - assign75550_e115114);
        (assign75550_e115115, (-(0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (-(0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (-(0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), (-(0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), (-(0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (-(0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (-(0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), (-(0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), (-(0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (-(0.5 * (locals.var_tmf1_dn13 + locals.var_tmf2_dn13))),)
    } else {
        (locals.var_vbs_max_over__blk1766, locals.var_vbs_max_over__blk1766_dn0, locals.var_vbs_max_over__blk1766_dn2, locals.var_vbs_max_over__blk1766_dn4, locals.var_vbs_max_over__blk1766_dn5, locals.var_vbs_max_over__blk1766_dn6, locals.var_vbs_max_over__blk1766_dn7, locals.var_vbs_max_over__blk1766_dn8, locals.var_vbs_max_over__blk1766_dn9, locals.var_vbs_max_over__blk1766_dn10, locals.var_vbs_max_over__blk1766_dn13,)
    }
};
        locals.var_vbs_max_over__blk1766 = assign75550_e115117;
        locals.var_vbs_max_over__blk1766_dn0 = assign75550_e115117_d_n0;
        locals.var_vbs_max_over__blk1766_dn2 = assign75550_e115117_d_n2;
        locals.var_vbs_max_over__blk1766_dn4 = assign75550_e115117_d_n4;
        locals.var_vbs_max_over__blk1766_dn5 = assign75550_e115117_d_n5;
        locals.var_vbs_max_over__blk1766_dn6 = assign75550_e115117_d_n6;
        locals.var_vbs_max_over__blk1766_dn7 = assign75550_e115117_d_n7;
        locals.var_vbs_max_over__blk1766_dn8 = assign75550_e115117_d_n8;
        locals.var_vbs_max_over__blk1766_dn9 = assign75550_e115117_d_n9;
        locals.var_vbs_max_over__blk1766_dn10 = assign75550_e115117_d_n10;
        locals.var_vbs_max_over__blk1766_dn13 = assign75550_e115117_d_n13;
        locals.var_vbs_max_over__blk1766_rv = 0.0;

        let assign75560_e115121: f64 = (locals.var_vbs_max_over__blk1766 * 0.5);
        let assign75560_e115122: f64 = if locals.var_vbs_bnd_over__blk1767 > assign75560_e115121 { 1.0 } else { 0.0 };
        locals.var_guard1772 = assign75560_e115122;
        locals.var_guard1772_rv = 0.0;

        let (assign75570_e115130, assign75570_e115130_d_n0, assign75570_e115130_d_n2, assign75570_e115130_d_n4, assign75570_e115130_d_n5, assign75570_e115130_d_n6, assign75570_e115130_d_n7, assign75570_e115130_d_n8, assign75570_e115130_d_n9, assign75570_e115130_d_n10, assign75570_e115130_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1772 != 0.0)) {
        let assign75570_e115128: f64 = (0.5 * locals.var_vbs_max_over__blk1766);
        (assign75570_e115128, (0.5 * locals.var_vbs_max_over__blk1766_dn0), (0.5 * locals.var_vbs_max_over__blk1766_dn2), (0.5 * locals.var_vbs_max_over__blk1766_dn4), (0.5 * locals.var_vbs_max_over__blk1766_dn5), (0.5 * locals.var_vbs_max_over__blk1766_dn6), (0.5 * locals.var_vbs_max_over__blk1766_dn7), (0.5 * locals.var_vbs_max_over__blk1766_dn8), (0.5 * locals.var_vbs_max_over__blk1766_dn9), (0.5 * locals.var_vbs_max_over__blk1766_dn10), (0.5 * locals.var_vbs_max_over__blk1766_dn13),)
    } else {
        (locals.var_vbs_bnd_over__blk1767, locals.var_vbs_bnd_over__blk1767_dn0, locals.var_vbs_bnd_over__blk1767_dn2, locals.var_vbs_bnd_over__blk1767_dn4, locals.var_vbs_bnd_over__blk1767_dn5, locals.var_vbs_bnd_over__blk1767_dn6, locals.var_vbs_bnd_over__blk1767_dn7, locals.var_vbs_bnd_over__blk1767_dn8, locals.var_vbs_bnd_over__blk1767_dn9, locals.var_vbs_bnd_over__blk1767_dn10, locals.var_vbs_bnd_over__blk1767_dn13,)
    }
};
        locals.var_vbs_bnd_over__blk1767 = assign75570_e115130;
        locals.var_vbs_bnd_over__blk1767_dn0 = assign75570_e115130_d_n0;
        locals.var_vbs_bnd_over__blk1767_dn2 = assign75570_e115130_d_n2;
        locals.var_vbs_bnd_over__blk1767_dn4 = assign75570_e115130_d_n4;
        locals.var_vbs_bnd_over__blk1767_dn5 = assign75570_e115130_d_n5;
        locals.var_vbs_bnd_over__blk1767_dn6 = assign75570_e115130_d_n6;
        locals.var_vbs_bnd_over__blk1767_dn7 = assign75570_e115130_d_n7;
        locals.var_vbs_bnd_over__blk1767_dn8 = assign75570_e115130_d_n8;
        locals.var_vbs_bnd_over__blk1767_dn9 = assign75570_e115130_d_n9;
        locals.var_vbs_bnd_over__blk1767_dn10 = assign75570_e115130_d_n10;
        locals.var_vbs_bnd_over__blk1767_dn13 = assign75570_e115130_d_n13;
        locals.var_vbs_bnd_over__blk1767_rv = 0.0;

        let assign75580_e115132: f64 = if param_given[338] { 1.0 } else { 0.0 };
        locals.var_guard1773 = assign75580_e115132;
        locals.var_guard1773_rv = 0.0;

        let (assign75590_e115138, assign75590_e115138_d_n0, assign75590_e115138_d_n2, assign75590_e115138_d_n4, assign75590_e115138_d_n5, assign75590_e115138_d_n6, assign75590_e115138_d_n7, assign75590_e115138_d_n8, assign75590_e115138_d_n9, assign75590_e115138_d_n10, assign75590_e115138_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1773 != 0.0)) {
        (p.p338, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vbs_max_over__blk1766, locals.var_vbs_max_over__blk1766_dn0, locals.var_vbs_max_over__blk1766_dn2, locals.var_vbs_max_over__blk1766_dn4, locals.var_vbs_max_over__blk1766_dn5, locals.var_vbs_max_over__blk1766_dn6, locals.var_vbs_max_over__blk1766_dn7, locals.var_vbs_max_over__blk1766_dn8, locals.var_vbs_max_over__blk1766_dn9, locals.var_vbs_max_over__blk1766_dn10, locals.var_vbs_max_over__blk1766_dn13,)
    }
};
        locals.var_vbs_max_over__blk1766 = assign75590_e115138;
        locals.var_vbs_max_over__blk1766_dn0 = assign75590_e115138_d_n0;
        locals.var_vbs_max_over__blk1766_dn2 = assign75590_e115138_d_n2;
        locals.var_vbs_max_over__blk1766_dn4 = assign75590_e115138_d_n4;
        locals.var_vbs_max_over__blk1766_dn5 = assign75590_e115138_d_n5;
        locals.var_vbs_max_over__blk1766_dn6 = assign75590_e115138_d_n6;
        locals.var_vbs_max_over__blk1766_dn7 = assign75590_e115138_d_n7;
        locals.var_vbs_max_over__blk1766_dn8 = assign75590_e115138_d_n8;
        locals.var_vbs_max_over__blk1766_dn9 = assign75590_e115138_d_n9;
        locals.var_vbs_max_over__blk1766_dn10 = assign75590_e115138_d_n10;
        locals.var_vbs_max_over__blk1766_dn13 = assign75590_e115138_d_n13;
        locals.var_vbs_max_over__blk1766_rv = 0.0;

        let assign75600_e115140: f64 = if param_given[339] { 1.0 } else { 0.0 };
        locals.var_guard1774 = assign75600_e115140;
        locals.var_guard1774_rv = 0.0;

        let (assign75610_e115146, assign75610_e115146_d_n0, assign75610_e115146_d_n2, assign75610_e115146_d_n4, assign75610_e115146_d_n5, assign75610_e115146_d_n6, assign75610_e115146_d_n7, assign75610_e115146_d_n8, assign75610_e115146_d_n9, assign75610_e115146_d_n10, assign75610_e115146_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1774 != 0.0)) {
        (p.p339, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vbs_bnd_over__blk1767, locals.var_vbs_bnd_over__blk1767_dn0, locals.var_vbs_bnd_over__blk1767_dn2, locals.var_vbs_bnd_over__blk1767_dn4, locals.var_vbs_bnd_over__blk1767_dn5, locals.var_vbs_bnd_over__blk1767_dn6, locals.var_vbs_bnd_over__blk1767_dn7, locals.var_vbs_bnd_over__blk1767_dn8, locals.var_vbs_bnd_over__blk1767_dn9, locals.var_vbs_bnd_over__blk1767_dn10, locals.var_vbs_bnd_over__blk1767_dn13,)
    }
};
        locals.var_vbs_bnd_over__blk1767 = assign75610_e115146;
        locals.var_vbs_bnd_over__blk1767_dn0 = assign75610_e115146_d_n0;
        locals.var_vbs_bnd_over__blk1767_dn2 = assign75610_e115146_d_n2;
        locals.var_vbs_bnd_over__blk1767_dn4 = assign75610_e115146_d_n4;
        locals.var_vbs_bnd_over__blk1767_dn5 = assign75610_e115146_d_n5;
        locals.var_vbs_bnd_over__blk1767_dn6 = assign75610_e115146_d_n6;
        locals.var_vbs_bnd_over__blk1767_dn7 = assign75610_e115146_d_n7;
        locals.var_vbs_bnd_over__blk1767_dn8 = assign75610_e115146_d_n8;
        locals.var_vbs_bnd_over__blk1767_dn9 = assign75610_e115146_d_n9;
        locals.var_vbs_bnd_over__blk1767_dn10 = assign75610_e115146_d_n10;
        locals.var_vbs_bnd_over__blk1767_dn13 = assign75610_e115146_d_n13;
        locals.var_vbs_bnd_over__blk1767_rv = 0.0;

        let assign75620_e115148: f64 = if param_given[338] { 1.0 } else { 0.0 };
        locals.var_guard1775 = assign75620_e115148;
        locals.var_guard1775_rv = 0.0;

        let (assign75630_e115159, assign75630_e115159_d_n0, assign75630_e115159_d_n2, assign75630_e115159_d_n4, assign75630_e115159_d_n5, assign75630_e115159_d_n6, assign75630_e115159_d_n7, assign75630_e115159_d_n8, assign75630_e115159_d_n9, assign75630_e115159_d_n10, assign75630_e115159_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1774 == 0.0)) && (locals.var_guard1775 != 0.0)) {
        let assign75630_e115157: f64 = (0.5 * locals.var_vbs_max_over__blk1766);
        (assign75630_e115157, (0.5 * locals.var_vbs_max_over__blk1766_dn0), (0.5 * locals.var_vbs_max_over__blk1766_dn2), (0.5 * locals.var_vbs_max_over__blk1766_dn4), (0.5 * locals.var_vbs_max_over__blk1766_dn5), (0.5 * locals.var_vbs_max_over__blk1766_dn6), (0.5 * locals.var_vbs_max_over__blk1766_dn7), (0.5 * locals.var_vbs_max_over__blk1766_dn8), (0.5 * locals.var_vbs_max_over__blk1766_dn9), (0.5 * locals.var_vbs_max_over__blk1766_dn10), (0.5 * locals.var_vbs_max_over__blk1766_dn13),)
    } else {
        (locals.var_vbs_bnd_over__blk1767, locals.var_vbs_bnd_over__blk1767_dn0, locals.var_vbs_bnd_over__blk1767_dn2, locals.var_vbs_bnd_over__blk1767_dn4, locals.var_vbs_bnd_over__blk1767_dn5, locals.var_vbs_bnd_over__blk1767_dn6, locals.var_vbs_bnd_over__blk1767_dn7, locals.var_vbs_bnd_over__blk1767_dn8, locals.var_vbs_bnd_over__blk1767_dn9, locals.var_vbs_bnd_over__blk1767_dn10, locals.var_vbs_bnd_over__blk1767_dn13,)
    }
};
        locals.var_vbs_bnd_over__blk1767 = assign75630_e115159;
        locals.var_vbs_bnd_over__blk1767_dn0 = assign75630_e115159_d_n0;
        locals.var_vbs_bnd_over__blk1767_dn2 = assign75630_e115159_d_n2;
        locals.var_vbs_bnd_over__blk1767_dn4 = assign75630_e115159_d_n4;
        locals.var_vbs_bnd_over__blk1767_dn5 = assign75630_e115159_d_n5;
        locals.var_vbs_bnd_over__blk1767_dn6 = assign75630_e115159_d_n6;
        locals.var_vbs_bnd_over__blk1767_dn7 = assign75630_e115159_d_n7;
        locals.var_vbs_bnd_over__blk1767_dn8 = assign75630_e115159_d_n8;
        locals.var_vbs_bnd_over__blk1767_dn9 = assign75630_e115159_d_n9;
        locals.var_vbs_bnd_over__blk1767_dn10 = assign75630_e115159_d_n10;
        locals.var_vbs_bnd_over__blk1767_dn13 = assign75630_e115159_d_n13;
        locals.var_vbs_bnd_over__blk1767_rv = 0.0;

        let assign75640_e115163: f64 = (locals.var_vbs_max_over__blk1766 * 0.5);
        let assign75640_e115164: f64 = if locals.var_vbs_bnd_over__blk1767 > assign75640_e115163 { 1.0 } else { 0.0 };
        locals.var_guard1776 = assign75640_e115164;
        locals.var_guard1776_rv = 0.0;

        let (assign75650_e115172, assign75650_e115172_d_n0, assign75650_e115172_d_n2, assign75650_e115172_d_n4, assign75650_e115172_d_n5, assign75650_e115172_d_n6, assign75650_e115172_d_n7, assign75650_e115172_d_n8, assign75650_e115172_d_n9, assign75650_e115172_d_n10, assign75650_e115172_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1776 != 0.0)) {
        let assign75650_e115170: f64 = (0.5 * locals.var_vbs_max_over__blk1766);
        (assign75650_e115170, (0.5 * locals.var_vbs_max_over__blk1766_dn0), (0.5 * locals.var_vbs_max_over__blk1766_dn2), (0.5 * locals.var_vbs_max_over__blk1766_dn4), (0.5 * locals.var_vbs_max_over__blk1766_dn5), (0.5 * locals.var_vbs_max_over__blk1766_dn6), (0.5 * locals.var_vbs_max_over__blk1766_dn7), (0.5 * locals.var_vbs_max_over__blk1766_dn8), (0.5 * locals.var_vbs_max_over__blk1766_dn9), (0.5 * locals.var_vbs_max_over__blk1766_dn10), (0.5 * locals.var_vbs_max_over__blk1766_dn13),)
    } else {
        (locals.var_vbs_bnd_over__blk1767, locals.var_vbs_bnd_over__blk1767_dn0, locals.var_vbs_bnd_over__blk1767_dn2, locals.var_vbs_bnd_over__blk1767_dn4, locals.var_vbs_bnd_over__blk1767_dn5, locals.var_vbs_bnd_over__blk1767_dn6, locals.var_vbs_bnd_over__blk1767_dn7, locals.var_vbs_bnd_over__blk1767_dn8, locals.var_vbs_bnd_over__blk1767_dn9, locals.var_vbs_bnd_over__blk1767_dn10, locals.var_vbs_bnd_over__blk1767_dn13,)
    }
};
        locals.var_vbs_bnd_over__blk1767 = assign75650_e115172;
        locals.var_vbs_bnd_over__blk1767_dn0 = assign75650_e115172_d_n0;
        locals.var_vbs_bnd_over__blk1767_dn2 = assign75650_e115172_d_n2;
        locals.var_vbs_bnd_over__blk1767_dn4 = assign75650_e115172_d_n4;
        locals.var_vbs_bnd_over__blk1767_dn5 = assign75650_e115172_d_n5;
        locals.var_vbs_bnd_over__blk1767_dn6 = assign75650_e115172_d_n6;
        locals.var_vbs_bnd_over__blk1767_dn7 = assign75650_e115172_d_n7;
        locals.var_vbs_bnd_over__blk1767_dn8 = assign75650_e115172_d_n8;
        locals.var_vbs_bnd_over__blk1767_dn9 = assign75650_e115172_d_n9;
        locals.var_vbs_bnd_over__blk1767_dn10 = assign75650_e115172_d_n10;
        locals.var_vbs_bnd_over__blk1767_dn13 = assign75650_e115172_d_n13;
        locals.var_vbs_bnd_over__blk1767_rv = 0.0;

        let assign75660_e115175: f64 = if p.p38 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1777 = assign75660_e115175;
        locals.var_guard1777_rv = 0.0;

        let (assign75670_e115182, assign75670_e115182_d_n0, assign75670_e115182_d_n2, assign75670_e115182_d_n4, assign75670_e115182_d_n5, assign75670_e115182_d_n6, assign75670_e115182_d_n7, assign75670_e115182_d_n8, assign75670_e115182_d_n9, assign75670_e115182_d_n10, assign75670_e115182_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1777 != 0.0)) {
        let assign75670_e115180: f64 = (-locals.var_vxbgmt);
        (assign75670_e115180, (-locals.var_vxbgmt_dn0), (-locals.var_vxbgmt_dn2), (-locals.var_vxbgmt_dn4), (-locals.var_vxbgmt_dn5), (-locals.var_vxbgmt_dn6), (-locals.var_vxbgmt_dn7), (-locals.var_vxbgmt_dn8), (-locals.var_vxbgmt_dn9), (-locals.var_vxbgmt_dn10), (-locals.var_vxbgmt_dn13),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign75670_e115182;
        locals.var_t0_dn0 = assign75670_e115182_d_n0;
        locals.var_t0_dn2 = assign75670_e115182_d_n2;
        locals.var_t0_dn4 = assign75670_e115182_d_n4;
        locals.var_t0_dn5 = assign75670_e115182_d_n5;
        locals.var_t0_dn6 = assign75670_e115182_d_n6;
        locals.var_t0_dn7 = assign75670_e115182_d_n7;
        locals.var_t0_dn8 = assign75670_e115182_d_n8;
        locals.var_t0_dn9 = assign75670_e115182_d_n9;
        locals.var_t0_dn10 = assign75670_e115182_d_n10;
        locals.var_t0_dn13 = assign75670_e115182_d_n13;
        locals.var_t0_rv = 0.0;

        let assign75680_e115185: f64 = if locals.var_t0 > locals.var_vbs_bnd_over__blk1767 { 1.0 } else { 0.0 };
        locals.var_guard1778 = assign75680_e115185;
        locals.var_guard1778_rv = 0.0;

        let (assign75690_e115195, assign75690_e115195_d_n0, assign75690_e115195_d_n2, assign75690_e115195_d_n4, assign75690_e115195_d_n5, assign75690_e115195_d_n6, assign75690_e115195_d_n7, assign75690_e115195_d_n8, assign75690_e115195_d_n9, assign75690_e115195_d_n10, assign75690_e115195_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1777 != 0.0)) && (locals.var_guard1778 != 0.0)) {
        let assign75690_e115193: f64 = (locals.var_t0 - locals.var_vbs_bnd_over__blk1767);
        (assign75690_e115193, (locals.var_t0_dn0 - locals.var_vbs_bnd_over__blk1767_dn0), (locals.var_t0_dn2 - locals.var_vbs_bnd_over__blk1767_dn2), (locals.var_t0_dn4 - locals.var_vbs_bnd_over__blk1767_dn4), (locals.var_t0_dn5 - locals.var_vbs_bnd_over__blk1767_dn5), (locals.var_t0_dn6 - locals.var_vbs_bnd_over__blk1767_dn6), (locals.var_t0_dn7 - locals.var_vbs_bnd_over__blk1767_dn7), (locals.var_t0_dn8 - locals.var_vbs_bnd_over__blk1767_dn8), (locals.var_t0_dn9 - locals.var_vbs_bnd_over__blk1767_dn9), (locals.var_t0_dn10 - locals.var_vbs_bnd_over__blk1767_dn10), (locals.var_t0_dn13 - locals.var_vbs_bnd_over__blk1767_dn13),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign75690_e115195;
        locals.var_t1_dn0 = assign75690_e115195_d_n0;
        locals.var_t1_dn2 = assign75690_e115195_d_n2;
        locals.var_t1_dn4 = assign75690_e115195_d_n4;
        locals.var_t1_dn5 = assign75690_e115195_d_n5;
        locals.var_t1_dn6 = assign75690_e115195_d_n6;
        locals.var_t1_dn7 = assign75690_e115195_d_n7;
        locals.var_t1_dn8 = assign75690_e115195_d_n8;
        locals.var_t1_dn9 = assign75690_e115195_d_n9;
        locals.var_t1_dn10 = assign75690_e115195_d_n10;
        locals.var_t1_dn13 = assign75690_e115195_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign75700_e115205, assign75700_e115205_d_n0, assign75700_e115205_d_n2, assign75700_e115205_d_n4, assign75700_e115205_d_n5, assign75700_e115205_d_n6, assign75700_e115205_d_n7, assign75700_e115205_d_n8, assign75700_e115205_d_n9, assign75700_e115205_d_n10, assign75700_e115205_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1777 != 0.0)) && (locals.var_guard1778 != 0.0)) {
        let assign75700_e115203: f64 = (locals.var_vbs_max_over__blk1766 - locals.var_vbs_bnd_over__blk1767);
        (assign75700_e115203, (locals.var_vbs_max_over__blk1766_dn0 - locals.var_vbs_bnd_over__blk1767_dn0), (locals.var_vbs_max_over__blk1766_dn2 - locals.var_vbs_bnd_over__blk1767_dn2), (locals.var_vbs_max_over__blk1766_dn4 - locals.var_vbs_bnd_over__blk1767_dn4), (locals.var_vbs_max_over__blk1766_dn5 - locals.var_vbs_bnd_over__blk1767_dn5), (locals.var_vbs_max_over__blk1766_dn6 - locals.var_vbs_bnd_over__blk1767_dn6), (locals.var_vbs_max_over__blk1766_dn7 - locals.var_vbs_bnd_over__blk1767_dn7), (locals.var_vbs_max_over__blk1766_dn8 - locals.var_vbs_bnd_over__blk1767_dn8), (locals.var_vbs_max_over__blk1766_dn9 - locals.var_vbs_bnd_over__blk1767_dn9), (locals.var_vbs_max_over__blk1766_dn10 - locals.var_vbs_bnd_over__blk1767_dn10), (locals.var_vbs_max_over__blk1766_dn13 - locals.var_vbs_bnd_over__blk1767_dn13),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign75700_e115205;
        locals.var_t2_dn0 = assign75700_e115205_d_n0;
        locals.var_t2_dn2 = assign75700_e115205_d_n2;
        locals.var_t2_dn4 = assign75700_e115205_d_n4;
        locals.var_t2_dn5 = assign75700_e115205_d_n5;
        locals.var_t2_dn6 = assign75700_e115205_d_n6;
        locals.var_t2_dn7 = assign75700_e115205_d_n7;
        locals.var_t2_dn8 = assign75700_e115205_d_n8;
        locals.var_t2_dn9 = assign75700_e115205_d_n9;
        locals.var_t2_dn10 = assign75700_e115205_d_n10;
        locals.var_t2_dn13 = assign75700_e115205_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign75710_e115215, assign75710_e115215_d_n0, assign75710_e115215_d_n2, assign75710_e115215_d_n4, assign75710_e115215_d_n5, assign75710_e115215_d_n6, assign75710_e115215_d_n7, assign75710_e115215_d_n8, assign75710_e115215_d_n9, assign75710_e115215_d_n10, assign75710_e115215_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1777 != 0.0)) && (locals.var_guard1778 != 0.0)) {
        let assign75710_e115213: f64 = (locals.var_t1 / locals.var_t2);
        (assign75710_e115213, (((locals.var_t1_dn0 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn0)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn2 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn2)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn4 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn4)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn5 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn5)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn6 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn6)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn7 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn7)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn8 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn8)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn9 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn9)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn10 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn10)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn13 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn13)) / (locals.var_t2 * locals.var_t2)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign75710_e115215;
        locals.var_tmf1_dn0 = assign75710_e115215_d_n0;
        locals.var_tmf1_dn2 = assign75710_e115215_d_n2;
        locals.var_tmf1_dn4 = assign75710_e115215_d_n4;
        locals.var_tmf1_dn5 = assign75710_e115215_d_n5;
        locals.var_tmf1_dn6 = assign75710_e115215_d_n6;
        locals.var_tmf1_dn7 = assign75710_e115215_d_n7;
        locals.var_tmf1_dn8 = assign75710_e115215_d_n8;
        locals.var_tmf1_dn9 = assign75710_e115215_d_n9;
        locals.var_tmf1_dn10 = assign75710_e115215_d_n10;
        locals.var_tmf1_dn13 = assign75710_e115215_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign75720_e115225, assign75720_e115225_d_n0, assign75720_e115225_d_n2, assign75720_e115225_d_n4, assign75720_e115225_d_n5, assign75720_e115225_d_n6, assign75720_e115225_d_n7, assign75720_e115225_d_n8, assign75720_e115225_d_n9, assign75720_e115225_d_n10, assign75720_e115225_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1777 != 0.0)) && (locals.var_guard1778 != 0.0)) {
        let assign75720_e115223: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign75720_e115223, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign75720_e115225;
        locals.var_tmf2_dn0 = assign75720_e115225_d_n0;
        locals.var_tmf2_dn2 = assign75720_e115225_d_n2;
        locals.var_tmf2_dn4 = assign75720_e115225_d_n4;
        locals.var_tmf2_dn5 = assign75720_e115225_d_n5;
        locals.var_tmf2_dn6 = assign75720_e115225_d_n6;
        locals.var_tmf2_dn7 = assign75720_e115225_d_n7;
        locals.var_tmf2_dn8 = assign75720_e115225_d_n8;
        locals.var_tmf2_dn9 = assign75720_e115225_d_n9;
        locals.var_tmf2_dn10 = assign75720_e115225_d_n10;
        locals.var_tmf2_dn13 = assign75720_e115225_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign75730_e115235, assign75730_e115235_d_n0, assign75730_e115235_d_n2, assign75730_e115235_d_n4, assign75730_e115235_d_n5, assign75730_e115235_d_n6, assign75730_e115235_d_n7, assign75730_e115235_d_n8, assign75730_e115235_d_n9, assign75730_e115235_d_n10, assign75730_e115235_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1777 != 0.0)) && (locals.var_guard1778 != 0.0)) {
        let assign75730_e115233: f64 = (locals.var_tmf2 * locals.var_tmf1);
        (assign75730_e115233, ((locals.var_tmf2_dn0 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn0)), ((locals.var_tmf2_dn2 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn2)), ((locals.var_tmf2_dn4 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn4)), ((locals.var_tmf2_dn5 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn5)), ((locals.var_tmf2_dn6 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn6)), ((locals.var_tmf2_dn7 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn7)), ((locals.var_tmf2_dn8 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn8)), ((locals.var_tmf2_dn9 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn9)), ((locals.var_tmf2_dn10 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn10)), ((locals.var_tmf2_dn13 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn13)),)
    } else {
        (locals.var_tmf3, locals.var_tmf3_dn0, locals.var_tmf3_dn2, locals.var_tmf3_dn4, locals.var_tmf3_dn5, locals.var_tmf3_dn6, locals.var_tmf3_dn7, locals.var_tmf3_dn8, locals.var_tmf3_dn9, locals.var_tmf3_dn10, locals.var_tmf3_dn13,)
    }
};
        locals.var_tmf3 = assign75730_e115235;
        locals.var_tmf3_dn0 = assign75730_e115235_d_n0;
        locals.var_tmf3_dn2 = assign75730_e115235_d_n2;
        locals.var_tmf3_dn4 = assign75730_e115235_d_n4;
        locals.var_tmf3_dn5 = assign75730_e115235_d_n5;
        locals.var_tmf3_dn6 = assign75730_e115235_d_n6;
        locals.var_tmf3_dn7 = assign75730_e115235_d_n7;
        locals.var_tmf3_dn8 = assign75730_e115235_d_n8;
        locals.var_tmf3_dn9 = assign75730_e115235_d_n9;
        locals.var_tmf3_dn10 = assign75730_e115235_d_n10;
        locals.var_tmf3_dn13 = assign75730_e115235_d_n13;
        locals.var_tmf3_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_275(
        locals: &mut StampLocals,
    ) {
        let (assign75740_e115245, assign75740_e115245_d_n0, assign75740_e115245_d_n2, assign75740_e115245_d_n4, assign75740_e115245_d_n5, assign75740_e115245_d_n6, assign75740_e115245_d_n7, assign75740_e115245_d_n8, assign75740_e115245_d_n9, assign75740_e115245_d_n10, assign75740_e115245_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1777 != 0.0)) && (locals.var_guard1778 != 0.0)) {
        let assign75740_e115243: f64 = (locals.var_tmf2 * locals.var_tmf2);
        (assign75740_e115243, ((locals.var_tmf2_dn0 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn0)), ((locals.var_tmf2_dn2 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn2)), ((locals.var_tmf2_dn4 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn4)), ((locals.var_tmf2_dn5 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn5)), ((locals.var_tmf2_dn6 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn6)), ((locals.var_tmf2_dn7 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn7)), ((locals.var_tmf2_dn8 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn8)), ((locals.var_tmf2_dn9 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn9)), ((locals.var_tmf2_dn10 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn10)), ((locals.var_tmf2_dn13 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn13)),)
    } else {
        (locals.var_tmf4, locals.var_tmf4_dn0, locals.var_tmf4_dn2, locals.var_tmf4_dn4, locals.var_tmf4_dn5, locals.var_tmf4_dn6, locals.var_tmf4_dn7, locals.var_tmf4_dn8, locals.var_tmf4_dn9, locals.var_tmf4_dn10, locals.var_tmf4_dn13,)
    }
};
        locals.var_tmf4 = assign75740_e115245;
        locals.var_tmf4_dn0 = assign75740_e115245_d_n0;
        locals.var_tmf4_dn2 = assign75740_e115245_d_n2;
        locals.var_tmf4_dn4 = assign75740_e115245_d_n4;
        locals.var_tmf4_dn5 = assign75740_e115245_d_n5;
        locals.var_tmf4_dn6 = assign75740_e115245_d_n6;
        locals.var_tmf4_dn7 = assign75740_e115245_d_n7;
        locals.var_tmf4_dn8 = assign75740_e115245_d_n8;
        locals.var_tmf4_dn9 = assign75740_e115245_d_n9;
        locals.var_tmf4_dn10 = assign75740_e115245_d_n10;
        locals.var_tmf4_dn13 = assign75740_e115245_d_n13;
        locals.var_tmf4_rv = 0.0;

        let (assign75750_e115263, assign75750_e115263_d_n0, assign75750_e115263_d_n2, assign75750_e115263_d_n4, assign75750_e115263_d_n5, assign75750_e115263_d_n6, assign75750_e115263_d_n7, assign75750_e115263_d_n8, assign75750_e115263_d_n9, assign75750_e115263_d_n10, assign75750_e115263_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1777 != 0.0)) && (locals.var_guard1778 != 0.0)) {
        let assign75750_e115254: f64 = (1.0 + locals.var_tmf1);
        let assign75750_e115256: f64 = (assign75750_e115254 + locals.var_tmf2);
        let assign75750_e115258: f64 = (assign75750_e115256 + locals.var_tmf3);
        let assign75750_e115260: f64 = (assign75750_e115258 + locals.var_tmf4);
        let assign75750_e115261: f64 = (1.0 / assign75750_e115260);
        (assign75750_e115261, (-((((locals.var_tmf1_dn0 + locals.var_tmf2_dn0) + locals.var_tmf3_dn0) + locals.var_tmf4_dn0) / (assign75750_e115260 * assign75750_e115260))), (-((((locals.var_tmf1_dn2 + locals.var_tmf2_dn2) + locals.var_tmf3_dn2) + locals.var_tmf4_dn2) / (assign75750_e115260 * assign75750_e115260))), (-((((locals.var_tmf1_dn4 + locals.var_tmf2_dn4) + locals.var_tmf3_dn4) + locals.var_tmf4_dn4) / (assign75750_e115260 * assign75750_e115260))), (-((((locals.var_tmf1_dn5 + locals.var_tmf2_dn5) + locals.var_tmf3_dn5) + locals.var_tmf4_dn5) / (assign75750_e115260 * assign75750_e115260))), (-((((locals.var_tmf1_dn6 + locals.var_tmf2_dn6) + locals.var_tmf3_dn6) + locals.var_tmf4_dn6) / (assign75750_e115260 * assign75750_e115260))), (-((((locals.var_tmf1_dn7 + locals.var_tmf2_dn7) + locals.var_tmf3_dn7) + locals.var_tmf4_dn7) / (assign75750_e115260 * assign75750_e115260))), (-((((locals.var_tmf1_dn8 + locals.var_tmf2_dn8) + locals.var_tmf3_dn8) + locals.var_tmf4_dn8) / (assign75750_e115260 * assign75750_e115260))), (-((((locals.var_tmf1_dn9 + locals.var_tmf2_dn9) + locals.var_tmf3_dn9) + locals.var_tmf4_dn9) / (assign75750_e115260 * assign75750_e115260))), (-((((locals.var_tmf1_dn10 + locals.var_tmf2_dn10) + locals.var_tmf3_dn10) + locals.var_tmf4_dn10) / (assign75750_e115260 * assign75750_e115260))), (-((((locals.var_tmf1_dn13 + locals.var_tmf2_dn13) + locals.var_tmf3_dn13) + locals.var_tmf4_dn13) / (assign75750_e115260 * assign75750_e115260))),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn13,)
    }
};
        locals.var_tmf0 = assign75750_e115263;
        locals.var_tmf0_dn0 = assign75750_e115263_d_n0;
        locals.var_tmf0_dn2 = assign75750_e115263_d_n2;
        locals.var_tmf0_dn4 = assign75750_e115263_d_n4;
        locals.var_tmf0_dn5 = assign75750_e115263_d_n5;
        locals.var_tmf0_dn6 = assign75750_e115263_d_n6;
        locals.var_tmf0_dn7 = assign75750_e115263_d_n7;
        locals.var_tmf0_dn8 = assign75750_e115263_d_n8;
        locals.var_tmf0_dn9 = assign75750_e115263_d_n9;
        locals.var_tmf0_dn10 = assign75750_e115263_d_n10;
        locals.var_tmf0_dn13 = assign75750_e115263_d_n13;
        locals.var_tmf0_rv = 0.0;

        let (assign75760_e115288, assign75760_e115288_d_n0, assign75760_e115288_d_n2, assign75760_e115288_d_n4, assign75760_e115288_d_n5, assign75760_e115288_d_n6, assign75760_e115288_d_n7, assign75760_e115288_d_n8, assign75760_e115288_d_n9, assign75760_e115288_d_n10, assign75760_e115288_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1777 != 0.0)) && (locals.var_guard1778 != 0.0)) {
        let assign75760_e115272: f64 = (2.0 * locals.var_tmf1);
        let assign75760_e115273: f64 = (1.0 + assign75760_e115272);
        let assign75760_e115276: f64 = (3.0 * locals.var_tmf2);
        let assign75760_e115277: f64 = (assign75760_e115273 + assign75760_e115276);
        let assign75760_e115280: f64 = (4.0 * locals.var_tmf3);
        let assign75760_e115281: f64 = (assign75760_e115277 + assign75760_e115280);
        let assign75760_e115282: f64 = (-assign75760_e115281);
        let assign75760_e115284: f64 = (assign75760_e115282 * locals.var_tmf0);
        let assign75760_e115286: f64 = (assign75760_e115284 * locals.var_tmf0);
        (assign75760_e115286, (((((-(((2.0 * locals.var_tmf1_dn0) + (3.0 * locals.var_tmf2_dn0)) + (4.0 * locals.var_tmf3_dn0))) * locals.var_tmf0) + (assign75760_e115282 * locals.var_tmf0_dn0)) * locals.var_tmf0) + (assign75760_e115284 * locals.var_tmf0_dn0)), (((((-(((2.0 * locals.var_tmf1_dn2) + (3.0 * locals.var_tmf2_dn2)) + (4.0 * locals.var_tmf3_dn2))) * locals.var_tmf0) + (assign75760_e115282 * locals.var_tmf0_dn2)) * locals.var_tmf0) + (assign75760_e115284 * locals.var_tmf0_dn2)), (((((-(((2.0 * locals.var_tmf1_dn4) + (3.0 * locals.var_tmf2_dn4)) + (4.0 * locals.var_tmf3_dn4))) * locals.var_tmf0) + (assign75760_e115282 * locals.var_tmf0_dn4)) * locals.var_tmf0) + (assign75760_e115284 * locals.var_tmf0_dn4)), (((((-(((2.0 * locals.var_tmf1_dn5) + (3.0 * locals.var_tmf2_dn5)) + (4.0 * locals.var_tmf3_dn5))) * locals.var_tmf0) + (assign75760_e115282 * locals.var_tmf0_dn5)) * locals.var_tmf0) + (assign75760_e115284 * locals.var_tmf0_dn5)), (((((-(((2.0 * locals.var_tmf1_dn6) + (3.0 * locals.var_tmf2_dn6)) + (4.0 * locals.var_tmf3_dn6))) * locals.var_tmf0) + (assign75760_e115282 * locals.var_tmf0_dn6)) * locals.var_tmf0) + (assign75760_e115284 * locals.var_tmf0_dn6)), (((((-(((2.0 * locals.var_tmf1_dn7) + (3.0 * locals.var_tmf2_dn7)) + (4.0 * locals.var_tmf3_dn7))) * locals.var_tmf0) + (assign75760_e115282 * locals.var_tmf0_dn7)) * locals.var_tmf0) + (assign75760_e115284 * locals.var_tmf0_dn7)), (((((-(((2.0 * locals.var_tmf1_dn8) + (3.0 * locals.var_tmf2_dn8)) + (4.0 * locals.var_tmf3_dn8))) * locals.var_tmf0) + (assign75760_e115282 * locals.var_tmf0_dn8)) * locals.var_tmf0) + (assign75760_e115284 * locals.var_tmf0_dn8)), (((((-(((2.0 * locals.var_tmf1_dn9) + (3.0 * locals.var_tmf2_dn9)) + (4.0 * locals.var_tmf3_dn9))) * locals.var_tmf0) + (assign75760_e115282 * locals.var_tmf0_dn9)) * locals.var_tmf0) + (assign75760_e115284 * locals.var_tmf0_dn9)), (((((-(((2.0 * locals.var_tmf1_dn10) + (3.0 * locals.var_tmf2_dn10)) + (4.0 * locals.var_tmf3_dn10))) * locals.var_tmf0) + (assign75760_e115282 * locals.var_tmf0_dn10)) * locals.var_tmf0) + (assign75760_e115284 * locals.var_tmf0_dn10)), (((((-(((2.0 * locals.var_tmf1_dn13) + (3.0 * locals.var_tmf2_dn13)) + (4.0 * locals.var_tmf3_dn13))) * locals.var_tmf0) + (assign75760_e115282 * locals.var_tmf0_dn13)) * locals.var_tmf0) + (assign75760_e115284 * locals.var_tmf0_dn13)),)
    } else {
        (locals.var_t11, locals.var_t11_dn0, locals.var_t11_dn2, locals.var_t11_dn4, locals.var_t11_dn5, locals.var_t11_dn6, locals.var_t11_dn7, locals.var_t11_dn8, locals.var_t11_dn9, locals.var_t11_dn10, locals.var_t11_dn13,)
    }
};
        locals.var_t11 = assign75760_e115288;
        locals.var_t11_dn0 = assign75760_e115288_d_n0;
        locals.var_t11_dn2 = assign75760_e115288_d_n2;
        locals.var_t11_dn4 = assign75760_e115288_d_n4;
        locals.var_t11_dn5 = assign75760_e115288_d_n5;
        locals.var_t11_dn6 = assign75760_e115288_d_n6;
        locals.var_t11_dn7 = assign75760_e115288_d_n7;
        locals.var_t11_dn8 = assign75760_e115288_d_n8;
        locals.var_t11_dn9 = assign75760_e115288_d_n9;
        locals.var_t11_dn10 = assign75760_e115288_d_n10;
        locals.var_t11_dn13 = assign75760_e115288_d_n13;
        locals.var_t11_rv = 0.0;

        let (assign75770_e115300, assign75770_e115300_d_n0, assign75770_e115300_d_n2, assign75770_e115300_d_n4, assign75770_e115300_d_n5, assign75770_e115300_d_n6, assign75770_e115300_d_n7, assign75770_e115300_d_n8, assign75770_e115300_d_n9, assign75770_e115300_d_n10, assign75770_e115300_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1777 != 0.0)) && (locals.var_guard1778 != 0.0)) {
        let assign75770_e115297: f64 = (1.0 - locals.var_tmf0);
        let assign75770_e115298: f64 = (locals.var_t2 * assign75770_e115297);
        (assign75770_e115298, ((locals.var_t2_dn0 * assign75770_e115297) + (locals.var_t2 * (-locals.var_tmf0_dn0))), ((locals.var_t2_dn2 * assign75770_e115297) + (locals.var_t2 * (-locals.var_tmf0_dn2))), ((locals.var_t2_dn4 * assign75770_e115297) + (locals.var_t2 * (-locals.var_tmf0_dn4))), ((locals.var_t2_dn5 * assign75770_e115297) + (locals.var_t2 * (-locals.var_tmf0_dn5))), ((locals.var_t2_dn6 * assign75770_e115297) + (locals.var_t2 * (-locals.var_tmf0_dn6))), ((locals.var_t2_dn7 * assign75770_e115297) + (locals.var_t2 * (-locals.var_tmf0_dn7))), ((locals.var_t2_dn8 * assign75770_e115297) + (locals.var_t2 * (-locals.var_tmf0_dn8))), ((locals.var_t2_dn9 * assign75770_e115297) + (locals.var_t2 * (-locals.var_tmf0_dn9))), ((locals.var_t2_dn10 * assign75770_e115297) + (locals.var_t2 * (-locals.var_tmf0_dn10))), ((locals.var_t2_dn13 * assign75770_e115297) + (locals.var_t2 * (-locals.var_tmf0_dn13))),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn4, locals.var_ty_dn5, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn8, locals.var_ty_dn9, locals.var_ty_dn10, locals.var_ty_dn13,)
    }
};
        locals.var_ty = assign75770_e115300;
        locals.var_ty_dn0 = assign75770_e115300_d_n0;
        locals.var_ty_dn2 = assign75770_e115300_d_n2;
        locals.var_ty_dn4 = assign75770_e115300_d_n4;
        locals.var_ty_dn5 = assign75770_e115300_d_n5;
        locals.var_ty_dn6 = assign75770_e115300_d_n6;
        locals.var_ty_dn7 = assign75770_e115300_d_n7;
        locals.var_ty_dn8 = assign75770_e115300_d_n8;
        locals.var_ty_dn9 = assign75770_e115300_d_n9;
        locals.var_ty_dn10 = assign75770_e115300_d_n10;
        locals.var_ty_dn13 = assign75770_e115300_d_n13;
        locals.var_ty_rv = 0.0;

        let (assign75780_e115314, assign75780_e115314_d_n0, assign75780_e115314_d_n2, assign75780_e115314_d_n4, assign75780_e115314_d_n5, assign75780_e115314_d_n6, assign75780_e115314_d_n7, assign75780_e115314_d_n8, assign75780_e115314_d_n9, assign75780_e115314_d_n10, assign75780_e115314_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1777 != 0.0)) && (locals.var_guard1778 != 0.0)) {
        let assign75780_e115308: f64 = (1.0 - locals.var_tmf0);
        let assign75780_e115311: f64 = (locals.var_tmf1 * locals.var_t11);
        let assign75780_e115312: f64 = (assign75780_e115308 + assign75780_e115311);
        (assign75780_e115312, ((-locals.var_tmf0_dn0) + ((locals.var_tmf1_dn0 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn0))), ((-locals.var_tmf0_dn2) + ((locals.var_tmf1_dn2 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn2))), ((-locals.var_tmf0_dn4) + ((locals.var_tmf1_dn4 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn4))), ((-locals.var_tmf0_dn5) + ((locals.var_tmf1_dn5 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn5))), ((-locals.var_tmf0_dn6) + ((locals.var_tmf1_dn6 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn6))), ((-locals.var_tmf0_dn7) + ((locals.var_tmf1_dn7 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn7))), ((-locals.var_tmf0_dn8) + ((locals.var_tmf1_dn8 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn8))), ((-locals.var_tmf0_dn9) + ((locals.var_tmf1_dn9 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn9))), ((-locals.var_tmf0_dn10) + ((locals.var_tmf1_dn10 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn10))), ((-locals.var_tmf0_dn13) + ((locals.var_tmf1_dn13 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn13))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign75780_e115314;
        locals.var_t0_dn0 = assign75780_e115314_d_n0;
        locals.var_t0_dn2 = assign75780_e115314_d_n2;
        locals.var_t0_dn4 = assign75780_e115314_d_n4;
        locals.var_t0_dn5 = assign75780_e115314_d_n5;
        locals.var_t0_dn6 = assign75780_e115314_d_n6;
        locals.var_t0_dn7 = assign75780_e115314_d_n7;
        locals.var_t0_dn8 = assign75780_e115314_d_n8;
        locals.var_t0_dn9 = assign75780_e115314_d_n9;
        locals.var_t0_dn10 = assign75780_e115314_d_n10;
        locals.var_t0_dn13 = assign75780_e115314_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign75790_e115323, assign75790_e115323_d_n0, assign75790_e115323_d_n2, assign75790_e115323_d_n4, assign75790_e115323_d_n5, assign75790_e115323_d_n6, assign75790_e115323_d_n7, assign75790_e115323_d_n8, assign75790_e115323_d_n9, assign75790_e115323_d_n10, assign75790_e115323_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1777 != 0.0)) && (locals.var_guard1778 != 0.0)) {
        let assign75790_e115321: f64 = (-locals.var_t11);
        (assign75790_e115321, (-locals.var_t11_dn0), (-locals.var_t11_dn2), (-locals.var_t11_dn4), (-locals.var_t11_dn5), (-locals.var_t11_dn6), (-locals.var_t11_dn7), (-locals.var_t11_dn8), (-locals.var_t11_dn9), (-locals.var_t11_dn10), (-locals.var_t11_dn13),)
    } else {
        (locals.var_t11, locals.var_t11_dn0, locals.var_t11_dn2, locals.var_t11_dn4, locals.var_t11_dn5, locals.var_t11_dn6, locals.var_t11_dn7, locals.var_t11_dn8, locals.var_t11_dn9, locals.var_t11_dn10, locals.var_t11_dn13,)
    }
};
        locals.var_t11 = assign75790_e115323;
        locals.var_t11_dn0 = assign75790_e115323_d_n0;
        locals.var_t11_dn2 = assign75790_e115323_d_n2;
        locals.var_t11_dn4 = assign75790_e115323_d_n4;
        locals.var_t11_dn5 = assign75790_e115323_d_n5;
        locals.var_t11_dn6 = assign75790_e115323_d_n6;
        locals.var_t11_dn7 = assign75790_e115323_d_n7;
        locals.var_t11_dn8 = assign75790_e115323_d_n8;
        locals.var_t11_dn9 = assign75790_e115323_d_n9;
        locals.var_t11_dn10 = assign75790_e115323_d_n10;
        locals.var_t11_dn13 = assign75790_e115323_d_n13;
        locals.var_t11_rv = 0.0;

        let (assign75800_e115333, assign75800_e115333_d_n0, assign75800_e115333_d_n2, assign75800_e115333_d_n4, assign75800_e115333_d_n5, assign75800_e115333_d_n6, assign75800_e115333_d_n7, assign75800_e115333_d_n8, assign75800_e115333_d_n9, assign75800_e115333_d_n10, assign75800_e115333_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1777 != 0.0)) && (locals.var_guard1778 != 0.0)) {
        let assign75800_e115331: f64 = (locals.var_vbs_bnd_over__blk1767 + locals.var_ty);
        (assign75800_e115331, (locals.var_vbs_bnd_over__blk1767_dn0 + locals.var_ty_dn0), (locals.var_vbs_bnd_over__blk1767_dn2 + locals.var_ty_dn2), (locals.var_vbs_bnd_over__blk1767_dn4 + locals.var_ty_dn4), (locals.var_vbs_bnd_over__blk1767_dn5 + locals.var_ty_dn5), (locals.var_vbs_bnd_over__blk1767_dn6 + locals.var_ty_dn6), (locals.var_vbs_bnd_over__blk1767_dn7 + locals.var_ty_dn7), (locals.var_vbs_bnd_over__blk1767_dn8 + locals.var_ty_dn8), (locals.var_vbs_bnd_over__blk1767_dn9 + locals.var_ty_dn9), (locals.var_vbs_bnd_over__blk1767_dn10 + locals.var_ty_dn10), (locals.var_vbs_bnd_over__blk1767_dn13 + locals.var_ty_dn13),)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn13,)
    }
};
        locals.var_t10 = assign75800_e115333;
        locals.var_t10_dn0 = assign75800_e115333_d_n0;
        locals.var_t10_dn2 = assign75800_e115333_d_n2;
        locals.var_t10_dn4 = assign75800_e115333_d_n4;
        locals.var_t10_dn5 = assign75800_e115333_d_n5;
        locals.var_t10_dn6 = assign75800_e115333_d_n6;
        locals.var_t10_dn7 = assign75800_e115333_d_n7;
        locals.var_t10_dn8 = assign75800_e115333_d_n8;
        locals.var_t10_dn9 = assign75800_e115333_d_n9;
        locals.var_t10_dn10 = assign75800_e115333_d_n10;
        locals.var_t10_dn13 = assign75800_e115333_d_n13;
        locals.var_t10_rv = 0.0;

        let (assign75810_e115342, assign75810_e115342_d_n0, assign75810_e115342_d_n2, assign75810_e115342_d_n4, assign75810_e115342_d_n5, assign75810_e115342_d_n6, assign75810_e115342_d_n7, assign75810_e115342_d_n8, assign75810_e115342_d_n9, assign75810_e115342_d_n10, assign75810_e115342_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1777 != 0.0)) && (locals.var_guard1778 == 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn13,)
    }
};
        locals.var_t10 = assign75810_e115342;
        locals.var_t10_dn0 = assign75810_e115342_d_n0;
        locals.var_t10_dn2 = assign75810_e115342_d_n2;
        locals.var_t10_dn4 = assign75810_e115342_d_n4;
        locals.var_t10_dn5 = assign75810_e115342_d_n5;
        locals.var_t10_dn6 = assign75810_e115342_d_n6;
        locals.var_t10_dn7 = assign75810_e115342_d_n7;
        locals.var_t10_dn8 = assign75810_e115342_d_n8;
        locals.var_t10_dn9 = assign75810_e115342_d_n9;
        locals.var_t10_dn10 = assign75810_e115342_d_n10;
        locals.var_t10_dn13 = assign75810_e115342_d_n13;
        locals.var_t10_rv = 0.0;

        let (assign75820_e115349, assign75820_e115349_d_n0, assign75820_e115349_d_n2, assign75820_e115349_d_n4, assign75820_e115349_d_n5, assign75820_e115349_d_n6, assign75820_e115349_d_n7, assign75820_e115349_d_n8, assign75820_e115349_d_n9, assign75820_e115349_d_n10, assign75820_e115349_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1777 != 0.0)) {
        let assign75820_e115347: f64 = (-locals.var_t10);
        (assign75820_e115347, (-locals.var_t10_dn0), (-locals.var_t10_dn2), (-locals.var_t10_dn4), (-locals.var_t10_dn5), (-locals.var_t10_dn6), (-locals.var_t10_dn7), (-locals.var_t10_dn8), (-locals.var_t10_dn9), (-locals.var_t10_dn10), (-locals.var_t10_dn13),)
    } else {
        (locals.var_vxbgmtcl, locals.var_vxbgmtcl_dn0, locals.var_vxbgmtcl_dn2, locals.var_vxbgmtcl_dn4, locals.var_vxbgmtcl_dn5, locals.var_vxbgmtcl_dn6, locals.var_vxbgmtcl_dn7, locals.var_vxbgmtcl_dn8, locals.var_vxbgmtcl_dn9, locals.var_vxbgmtcl_dn10, locals.var_vxbgmtcl_dn13,)
    }
};
        locals.var_vxbgmtcl = assign75820_e115349;
        locals.var_vxbgmtcl_dn0 = assign75820_e115349_d_n0;
        locals.var_vxbgmtcl_dn2 = assign75820_e115349_d_n2;
        locals.var_vxbgmtcl_dn4 = assign75820_e115349_d_n4;
        locals.var_vxbgmtcl_dn5 = assign75820_e115349_d_n5;
        locals.var_vxbgmtcl_dn6 = assign75820_e115349_d_n6;
        locals.var_vxbgmtcl_dn7 = assign75820_e115349_d_n7;
        locals.var_vxbgmtcl_dn8 = assign75820_e115349_d_n8;
        locals.var_vxbgmtcl_dn9 = assign75820_e115349_d_n9;
        locals.var_vxbgmtcl_dn10 = assign75820_e115349_d_n10;
        locals.var_vxbgmtcl_dn13 = assign75820_e115349_d_n13;
        locals.var_vxbgmtcl_rv = 0.0;

        let (assign75830_e115356, assign75830_e115356_d_n0, assign75830_e115356_d_n2, assign75830_e115356_d_n4, assign75830_e115356_d_n5, assign75830_e115356_d_n6, assign75830_e115356_d_n7, assign75830_e115356_d_n8, assign75830_e115356_d_n9, assign75830_e115356_d_n10, assign75830_e115356_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1777 == 0.0)) {
        (locals.var_vxbgmt, locals.var_vxbgmt_dn0, locals.var_vxbgmt_dn2, locals.var_vxbgmt_dn4, locals.var_vxbgmt_dn5, locals.var_vxbgmt_dn6, locals.var_vxbgmt_dn7, locals.var_vxbgmt_dn8, locals.var_vxbgmt_dn9, locals.var_vxbgmt_dn10, locals.var_vxbgmt_dn13,)
    } else {
        (locals.var_vxbgmtcl, locals.var_vxbgmtcl_dn0, locals.var_vxbgmtcl_dn2, locals.var_vxbgmtcl_dn4, locals.var_vxbgmtcl_dn5, locals.var_vxbgmtcl_dn6, locals.var_vxbgmtcl_dn7, locals.var_vxbgmtcl_dn8, locals.var_vxbgmtcl_dn9, locals.var_vxbgmtcl_dn10, locals.var_vxbgmtcl_dn13,)
    }
};
        locals.var_vxbgmtcl = assign75830_e115356;
        locals.var_vxbgmtcl_dn0 = assign75830_e115356_d_n0;
        locals.var_vxbgmtcl_dn2 = assign75830_e115356_d_n2;
        locals.var_vxbgmtcl_dn4 = assign75830_e115356_d_n4;
        locals.var_vxbgmtcl_dn5 = assign75830_e115356_d_n5;
        locals.var_vxbgmtcl_dn6 = assign75830_e115356_d_n6;
        locals.var_vxbgmtcl_dn7 = assign75830_e115356_d_n7;
        locals.var_vxbgmtcl_dn8 = assign75830_e115356_d_n8;
        locals.var_vxbgmtcl_dn9 = assign75830_e115356_d_n9;
        locals.var_vxbgmtcl_dn10 = assign75830_e115356_d_n10;
        locals.var_vxbgmtcl_dn13 = assign75830_e115356_d_n13;
        locals.var_vxbgmtcl_rv = 0.0;

        let (assign75840_e115362, assign75840_e115362_d_n0, assign75840_e115362_d_n2, assign75840_e115362_d_n4, assign75840_e115362_d_n5, assign75840_e115362_d_n6, assign75840_e115362_d_n7, assign75840_e115362_d_n8, assign75840_e115362_d_n9, assign75840_e115362_d_n10, assign75840_e115362_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign75840_e115360: f64 = (locals.var_cnst0over_func / locals.var_cox0_func);
        (assign75840_e115360, (locals.var_cnst0over_func_dn0 / locals.var_cox0_func), (locals.var_cnst0over_func_dn2 / locals.var_cox0_func), (locals.var_cnst0over_func_dn4 / locals.var_cox0_func), (locals.var_cnst0over_func_dn5 / locals.var_cox0_func), (locals.var_cnst0over_func_dn6 / locals.var_cox0_func), (locals.var_cnst0over_func_dn7 / locals.var_cox0_func), (locals.var_cnst0over_func_dn8 / locals.var_cox0_func), (locals.var_cnst0over_func_dn9 / locals.var_cox0_func), (locals.var_cnst0over_func_dn10 / locals.var_cox0_func), (locals.var_cnst0over_func_dn13 / locals.var_cox0_func),)
    } else {
        (locals.var_fac1, locals.var_fac1_dn0, locals.var_fac1_dn2, locals.var_fac1_dn4, locals.var_fac1_dn5, locals.var_fac1_dn6, locals.var_fac1_dn7, locals.var_fac1_dn8, locals.var_fac1_dn9, locals.var_fac1_dn10, locals.var_fac1_dn13,)
    }
};
        locals.var_fac1 = assign75840_e115362;
        locals.var_fac1_dn0 = assign75840_e115362_d_n0;
        locals.var_fac1_dn2 = assign75840_e115362_d_n2;
        locals.var_fac1_dn4 = assign75840_e115362_d_n4;
        locals.var_fac1_dn5 = assign75840_e115362_d_n5;
        locals.var_fac1_dn6 = assign75840_e115362_d_n6;
        locals.var_fac1_dn7 = assign75840_e115362_d_n7;
        locals.var_fac1_dn8 = assign75840_e115362_d_n8;
        locals.var_fac1_dn9 = assign75840_e115362_d_n9;
        locals.var_fac1_dn10 = assign75840_e115362_d_n10;
        locals.var_fac1_dn13 = assign75840_e115362_d_n13;
        locals.var_fac1_rv = 0.0;

        let (assign75850_e115368, assign75850_e115368_d_n0, assign75850_e115368_d_n2, assign75850_e115368_d_n4, assign75850_e115368_d_n5, assign75850_e115368_d_n6, assign75850_e115368_d_n7, assign75850_e115368_d_n8, assign75850_e115368_d_n9, assign75850_e115368_d_n10, assign75850_e115368_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign75850_e115366: f64 = (locals.var_fac1 * locals.var_fac1);
        (assign75850_e115366, ((locals.var_fac1_dn0 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn0)), ((locals.var_fac1_dn2 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn2)), ((locals.var_fac1_dn4 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn4)), ((locals.var_fac1_dn5 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn5)), ((locals.var_fac1_dn6 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn6)), ((locals.var_fac1_dn7 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn7)), ((locals.var_fac1_dn8 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn8)), ((locals.var_fac1_dn9 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn9)), ((locals.var_fac1_dn10 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn10)), ((locals.var_fac1_dn13 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn13)),)
    } else {
        (locals.var_fac1p2, locals.var_fac1p2_dn0, locals.var_fac1p2_dn2, locals.var_fac1p2_dn4, locals.var_fac1p2_dn5, locals.var_fac1p2_dn6, locals.var_fac1p2_dn7, locals.var_fac1p2_dn8, locals.var_fac1p2_dn9, locals.var_fac1p2_dn10, locals.var_fac1p2_dn13,)
    }
};
        locals.var_fac1p2 = assign75850_e115368;
        locals.var_fac1p2_dn0 = assign75850_e115368_d_n0;
        locals.var_fac1p2_dn2 = assign75850_e115368_d_n2;
        locals.var_fac1p2_dn4 = assign75850_e115368_d_n4;
        locals.var_fac1p2_dn5 = assign75850_e115368_d_n5;
        locals.var_fac1p2_dn6 = assign75850_e115368_d_n6;
        locals.var_fac1p2_dn7 = assign75850_e115368_d_n7;
        locals.var_fac1p2_dn8 = assign75850_e115368_d_n8;
        locals.var_fac1p2_dn9 = assign75850_e115368_d_n9;
        locals.var_fac1p2_dn10 = assign75850_e115368_d_n10;
        locals.var_fac1p2_dn13 = assign75850_e115368_d_n13;
        locals.var_fac1p2_rv = 0.0;

        let (assign75860_e115375, assign75860_e115375_d_n2, assign75860_e115375_d_n6, assign75860_e115375_d_n7, assign75860_e115375_d_n8,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign75860_e115371: f64 = (-locals.var_vgbgmt);
        let assign75860_e115373: f64 = (assign75860_e115371 + locals.var_uc_vfbover);
        (assign75860_e115373, (-locals.var_vgbgmt_dn2), (-locals.var_vgbgmt_dn6), (-locals.var_vgbgmt_dn7), (-locals.var_vgbgmt_dn8),)
    } else {
        (locals.var_vgpld, locals.var_vgpld_dn2, locals.var_vgpld_dn6, locals.var_vgpld_dn7, locals.var_vgpld_dn8,)
    }
};
        locals.var_vgpld = assign75860_e115375;
        locals.var_vgpld_dn2 = assign75860_e115375_d_n2;
        locals.var_vgpld_dn6 = assign75860_e115375_d_n6;
        locals.var_vgpld_dn7 = assign75860_e115375_d_n7;
        locals.var_vgpld_dn8 = assign75860_e115375_d_n8;
        locals.var_vgpld_rv = 0.0;

        let (assign75870_e115384, assign75870_e115384_d_n0, assign75870_e115384_d_n2, assign75870_e115384_d_n4, assign75870_e115384_d_n5, assign75870_e115384_d_n6, assign75870_e115384_d_n7, assign75870_e115384_d_n8, assign75870_e115384_d_n9, assign75870_e115384_d_n10, assign75870_e115384_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign75870_e115378: f64 = (-locals.var_vxbgmtcl);
        let assign75870_e115381: f64 = (10.0 * 2.220446049250313e-16);
        let assign75870_e115382: f64 = (assign75870_e115378 + assign75870_e115381);
        (assign75870_e115382, (-locals.var_vxbgmtcl_dn0), (-locals.var_vxbgmtcl_dn2), (-locals.var_vxbgmtcl_dn4), (-locals.var_vxbgmtcl_dn5), (-locals.var_vxbgmtcl_dn6), (-locals.var_vxbgmtcl_dn7), (-locals.var_vxbgmtcl_dn8), (-locals.var_vxbgmtcl_dn9), (-locals.var_vxbgmtcl_dn10), (-locals.var_vxbgmtcl_dn13),)
    } else {
        (locals.var_vgb_fb_ld, locals.var_vgb_fb_ld_dn0, locals.var_vgb_fb_ld_dn2, locals.var_vgb_fb_ld_dn4, locals.var_vgb_fb_ld_dn5, locals.var_vgb_fb_ld_dn6, locals.var_vgb_fb_ld_dn7, locals.var_vgb_fb_ld_dn8, locals.var_vgb_fb_ld_dn9, locals.var_vgb_fb_ld_dn10, locals.var_vgb_fb_ld_dn13,)
    }
};
        locals.var_vgb_fb_ld = assign75870_e115384;
        locals.var_vgb_fb_ld_dn0 = assign75870_e115384_d_n0;
        locals.var_vgb_fb_ld_dn2 = assign75870_e115384_d_n2;
        locals.var_vgb_fb_ld_dn4 = assign75870_e115384_d_n4;
        locals.var_vgb_fb_ld_dn5 = assign75870_e115384_d_n5;
        locals.var_vgb_fb_ld_dn6 = assign75870_e115384_d_n6;
        locals.var_vgb_fb_ld_dn7 = assign75870_e115384_d_n7;
        locals.var_vgb_fb_ld_dn8 = assign75870_e115384_d_n8;
        locals.var_vgb_fb_ld_dn9 = assign75870_e115384_d_n9;
        locals.var_vgb_fb_ld_dn10 = assign75870_e115384_d_n10;
        locals.var_vgb_fb_ld_dn13 = assign75870_e115384_d_n13;
        locals.var_vgb_fb_ld_rv = 0.0;

        let (assign75880_e115388, assign75880_e115388_d_n0, assign75880_e115388_d_n2, assign75880_e115388_d_n4, assign75880_e115388_d_n5, assign75880_e115388_d_n6, assign75880_e115388_d_n7, assign75880_e115388_d_n8, assign75880_e115388_d_n9, assign75880_e115388_d_n10, assign75880_e115388_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_q_dep_ld__blk1761, locals.var_q_dep_ld__blk1761_dn0, locals.var_q_dep_ld__blk1761_dn2, locals.var_q_dep_ld__blk1761_dn4, locals.var_q_dep_ld__blk1761_dn5, locals.var_q_dep_ld__blk1761_dn6, locals.var_q_dep_ld__blk1761_dn7, locals.var_q_dep_ld__blk1761_dn8, locals.var_q_dep_ld__blk1761_dn9, locals.var_q_dep_ld__blk1761_dn10, locals.var_q_dep_ld__blk1761_dn13,)
    }
};
        locals.var_q_dep_ld__blk1761 = assign75880_e115388;
        locals.var_q_dep_ld__blk1761_dn0 = assign75880_e115388_d_n0;
        locals.var_q_dep_ld__blk1761_dn2 = assign75880_e115388_d_n2;
        locals.var_q_dep_ld__blk1761_dn4 = assign75880_e115388_d_n4;
        locals.var_q_dep_ld__blk1761_dn5 = assign75880_e115388_d_n5;
        locals.var_q_dep_ld__blk1761_dn6 = assign75880_e115388_d_n6;
        locals.var_q_dep_ld__blk1761_dn7 = assign75880_e115388_d_n7;
        locals.var_q_dep_ld__blk1761_dn8 = assign75880_e115388_d_n8;
        locals.var_q_dep_ld__blk1761_dn9 = assign75880_e115388_d_n9;
        locals.var_q_dep_ld__blk1761_dn10 = assign75880_e115388_d_n10;
        locals.var_q_dep_ld__blk1761_dn13 = assign75880_e115388_d_n13;
        locals.var_q_dep_ld__blk1761_rv = 0.0;

        let (assign75890_e115394,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign75890_e115392: f64 = (1.6021918e-19 * locals.var_nover_func);
        (assign75890_e115392,)
    } else {
        (locals.var_q_nsubld__blk1762,)
    }
};
        locals.var_q_nsubld__blk1762 = assign75890_e115394;
        locals.var_q_nsubld__blk1762_rv = 0.0;

        let (assign75900_e115400, assign75900_e115400_d_n0, assign75900_e115400_d_n2, assign75900_e115400_d_n4, assign75900_e115400_d_n5, assign75900_e115400_d_n6, assign75900_e115400_d_n7, assign75900_e115400_d_n8, assign75900_e115400_d_n9, assign75900_e115400_d_n10, assign75900_e115400_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign75900_e115398: f64 = (locals.var_nin / locals.var_nover_func);
        (assign75900_e115398, (locals.var_nin_dn0 / locals.var_nover_func), (locals.var_nin_dn2 / locals.var_nover_func), (locals.var_nin_dn4 / locals.var_nover_func), (locals.var_nin_dn5 / locals.var_nover_func), (locals.var_nin_dn6 / locals.var_nover_func), (locals.var_nin_dn7 / locals.var_nover_func), (locals.var_nin_dn8 / locals.var_nover_func), (locals.var_nin_dn9 / locals.var_nover_func), (locals.var_nin_dn10 / locals.var_nover_func), (locals.var_nin_dn13 / locals.var_nover_func),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign75900_e115400;
        locals.var_t0_dn0 = assign75900_e115400_d_n0;
        locals.var_t0_dn2 = assign75900_e115400_d_n2;
        locals.var_t0_dn4 = assign75900_e115400_d_n4;
        locals.var_t0_dn5 = assign75900_e115400_d_n5;
        locals.var_t0_dn6 = assign75900_e115400_d_n6;
        locals.var_t0_dn7 = assign75900_e115400_d_n7;
        locals.var_t0_dn8 = assign75900_e115400_d_n8;
        locals.var_t0_dn9 = assign75900_e115400_d_n9;
        locals.var_t0_dn10 = assign75900_e115400_d_n10;
        locals.var_t0_dn13 = assign75900_e115400_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign75910_e115406, assign75910_e115406_d_n0, assign75910_e115406_d_n2, assign75910_e115406_d_n4, assign75910_e115406_d_n5, assign75910_e115406_d_n6, assign75910_e115406_d_n7, assign75910_e115406_d_n8, assign75910_e115406_d_n9, assign75910_e115406_d_n10, assign75910_e115406_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign75910_e115404: f64 = (locals.var_t0 * locals.var_t0);
        (assign75910_e115404, ((locals.var_t0_dn0 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn0)), ((locals.var_t0_dn2 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn2)), ((locals.var_t0_dn4 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn4)), ((locals.var_t0_dn5 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn5)), ((locals.var_t0_dn6 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn6)), ((locals.var_t0_dn7 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn7)), ((locals.var_t0_dn8 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn8)), ((locals.var_t0_dn9 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn9)), ((locals.var_t0_dn10 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn10)), ((locals.var_t0_dn13 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn13)),)
    } else {
        (locals.var_cnst1over, locals.var_cnst1over_dn0, locals.var_cnst1over_dn2, locals.var_cnst1over_dn4, locals.var_cnst1over_dn5, locals.var_cnst1over_dn6, locals.var_cnst1over_dn7, locals.var_cnst1over_dn8, locals.var_cnst1over_dn9, locals.var_cnst1over_dn10, locals.var_cnst1over_dn13,)
    }
};
        locals.var_cnst1over = assign75910_e115406;
        locals.var_cnst1over_dn0 = assign75910_e115406_d_n0;
        locals.var_cnst1over_dn2 = assign75910_e115406_d_n2;
        locals.var_cnst1over_dn4 = assign75910_e115406_d_n4;
        locals.var_cnst1over_dn5 = assign75910_e115406_d_n5;
        locals.var_cnst1over_dn6 = assign75910_e115406_d_n6;
        locals.var_cnst1over_dn7 = assign75910_e115406_d_n7;
        locals.var_cnst1over_dn8 = assign75910_e115406_d_n8;
        locals.var_cnst1over_dn9 = assign75910_e115406_d_n9;
        locals.var_cnst1over_dn10 = assign75910_e115406_d_n10;
        locals.var_cnst1over_dn13 = assign75910_e115406_d_n13;
        locals.var_cnst1over_rv = 0.0;

        let assign75920_e115409: f64 = (-locals.var_vxbgmtcl);
        let assign75920_e115410: f64 = (locals.var_beta * assign75920_e115409);
        let assign75920_e115412: f64 = if assign75920_e115410 >= 500.0 { 1.0 } else { 0.0 };
        locals.var_guard1779 = assign75920_e115412;
        locals.var_guard1779_rv = 0.0;

        let (assign75930_e115427, assign75930_e115427_d_n0, assign75930_e115427_d_n2, assign75930_e115427_d_n4, assign75930_e115427_d_n5, assign75930_e115427_d_n6, assign75930_e115427_d_n7, assign75930_e115427_d_n8, assign75930_e115427_d_n9, assign75930_e115427_d_n10, assign75930_e115427_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1779 != 0.0)) {
        let assign75930_e115420: f64 = (-locals.var_vxbgmtcl);
        let assign75930_e115421: f64 = (locals.var_beta * assign75930_e115420);
        let assign75930_e115422: f64 = (1.0 + assign75930_e115421);
        let assign75930_e115424: f64 = (assign75930_e115422 - 500.0);
        let assign75930_e115425: f64 = (1.403592217853e217 * assign75930_e115424);
        (assign75930_e115425, (1.403592217853e217 * ((locals.var_beta_dn0 * assign75930_e115420) + (locals.var_beta * (-locals.var_vxbgmtcl_dn0)))), (1.403592217853e217 * ((locals.var_beta_dn2 * assign75930_e115420) + (locals.var_beta * (-locals.var_vxbgmtcl_dn2)))), (1.403592217853e217 * ((locals.var_beta_dn4 * assign75930_e115420) + (locals.var_beta * (-locals.var_vxbgmtcl_dn4)))), (1.403592217853e217 * ((locals.var_beta_dn5 * assign75930_e115420) + (locals.var_beta * (-locals.var_vxbgmtcl_dn5)))), (1.403592217853e217 * ((locals.var_beta_dn6 * assign75930_e115420) + (locals.var_beta * (-locals.var_vxbgmtcl_dn6)))), (1.403592217853e217 * ((locals.var_beta_dn7 * assign75930_e115420) + (locals.var_beta * (-locals.var_vxbgmtcl_dn7)))), (1.403592217853e217 * ((locals.var_beta_dn8 * assign75930_e115420) + (locals.var_beta * (-locals.var_vxbgmtcl_dn8)))), (1.403592217853e217 * ((locals.var_beta_dn9 * assign75930_e115420) + (locals.var_beta * (-locals.var_vxbgmtcl_dn9)))), (1.403592217853e217 * ((locals.var_beta_dn10 * assign75930_e115420) + (locals.var_beta * (-locals.var_vxbgmtcl_dn10)))), (1.403592217853e217 * ((locals.var_beta_dn13 * assign75930_e115420) + (locals.var_beta * (-locals.var_vxbgmtcl_dn13)))),)
    } else {
        (locals.var_exp_bvbs, locals.var_exp_bvbs_dn0, locals.var_exp_bvbs_dn2, locals.var_exp_bvbs_dn4, locals.var_exp_bvbs_dn5, locals.var_exp_bvbs_dn6, locals.var_exp_bvbs_dn7, locals.var_exp_bvbs_dn8, locals.var_exp_bvbs_dn9, locals.var_exp_bvbs_dn10, locals.var_exp_bvbs_dn13,)
    }
};
        locals.var_exp_bvbs = assign75930_e115427;
        locals.var_exp_bvbs_dn0 = assign75930_e115427_d_n0;
        locals.var_exp_bvbs_dn2 = assign75930_e115427_d_n2;
        locals.var_exp_bvbs_dn4 = assign75930_e115427_d_n4;
        locals.var_exp_bvbs_dn5 = assign75930_e115427_d_n5;
        locals.var_exp_bvbs_dn6 = assign75930_e115427_d_n6;
        locals.var_exp_bvbs_dn7 = assign75930_e115427_d_n7;
        locals.var_exp_bvbs_dn8 = assign75930_e115427_d_n8;
        locals.var_exp_bvbs_dn9 = assign75930_e115427_d_n9;
        locals.var_exp_bvbs_dn10 = assign75930_e115427_d_n10;
        locals.var_exp_bvbs_dn13 = assign75930_e115427_d_n13;
        locals.var_exp_bvbs_rv = 0.0;

        let (assign75940_e115433, assign75940_e115433_d_n0, assign75940_e115433_d_n2, assign75940_e115433_d_n4, assign75940_e115433_d_n5, assign75940_e115433_d_n6, assign75940_e115433_d_n7, assign75940_e115433_d_n8, assign75940_e115433_d_n9, assign75940_e115433_d_n10, assign75940_e115433_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1779 != 0.0)) {
        (1.403592217853e217, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign75940_e115433;
        locals.var_t0_dn0 = assign75940_e115433_d_n0;
        locals.var_t0_dn2 = assign75940_e115433_d_n2;
        locals.var_t0_dn4 = assign75940_e115433_d_n4;
        locals.var_t0_dn5 = assign75940_e115433_d_n5;
        locals.var_t0_dn6 = assign75940_e115433_d_n6;
        locals.var_t0_dn7 = assign75940_e115433_d_n7;
        locals.var_t0_dn8 = assign75940_e115433_d_n8;
        locals.var_t0_dn9 = assign75940_e115433_d_n9;
        locals.var_t0_dn10 = assign75940_e115433_d_n10;
        locals.var_t0_dn13 = assign75940_e115433_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign75950_e115443, assign75950_e115443_d_n0, assign75950_e115443_d_n2, assign75950_e115443_d_n4, assign75950_e115443_d_n5, assign75950_e115443_d_n6, assign75950_e115443_d_n7, assign75950_e115443_d_n8, assign75950_e115443_d_n9, assign75950_e115443_d_n10, assign75950_e115443_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1779 == 0.0)) {
        let assign75950_e115440: f64 = (-locals.var_vxbgmtcl);
        let assign75950_e115441: f64 = (locals.var_beta * assign75950_e115440);
        (assign75950_e115441, ((locals.var_beta_dn0 * assign75950_e115440) + (locals.var_beta * (-locals.var_vxbgmtcl_dn0))), ((locals.var_beta_dn2 * assign75950_e115440) + (locals.var_beta * (-locals.var_vxbgmtcl_dn2))), ((locals.var_beta_dn4 * assign75950_e115440) + (locals.var_beta * (-locals.var_vxbgmtcl_dn4))), ((locals.var_beta_dn5 * assign75950_e115440) + (locals.var_beta * (-locals.var_vxbgmtcl_dn5))), ((locals.var_beta_dn6 * assign75950_e115440) + (locals.var_beta * (-locals.var_vxbgmtcl_dn6))), ((locals.var_beta_dn7 * assign75950_e115440) + (locals.var_beta * (-locals.var_vxbgmtcl_dn7))), ((locals.var_beta_dn8 * assign75950_e115440) + (locals.var_beta * (-locals.var_vxbgmtcl_dn8))), ((locals.var_beta_dn9 * assign75950_e115440) + (locals.var_beta * (-locals.var_vxbgmtcl_dn9))), ((locals.var_beta_dn10 * assign75950_e115440) + (locals.var_beta * (-locals.var_vxbgmtcl_dn10))), ((locals.var_beta_dn13 * assign75950_e115440) + (locals.var_beta * (-locals.var_vxbgmtcl_dn13))),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign75950_e115443;
        locals.var_tmf1_dn0 = assign75950_e115443_d_n0;
        locals.var_tmf1_dn2 = assign75950_e115443_d_n2;
        locals.var_tmf1_dn4 = assign75950_e115443_d_n4;
        locals.var_tmf1_dn5 = assign75950_e115443_d_n5;
        locals.var_tmf1_dn6 = assign75950_e115443_d_n6;
        locals.var_tmf1_dn7 = assign75950_e115443_d_n7;
        locals.var_tmf1_dn8 = assign75950_e115443_d_n8;
        locals.var_tmf1_dn9 = assign75950_e115443_d_n9;
        locals.var_tmf1_dn10 = assign75950_e115443_d_n10;
        locals.var_tmf1_dn13 = assign75950_e115443_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign75960_e115450, assign75960_e115450_d_n0, assign75960_e115450_d_n2, assign75960_e115450_d_n4, assign75960_e115450_d_n5, assign75960_e115450_d_n6, assign75960_e115450_d_n7, assign75960_e115450_d_n8, assign75960_e115450_d_n9, assign75960_e115450_d_n10, assign75960_e115450_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1779 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_exp_bvbs, locals.var_exp_bvbs_dn0, locals.var_exp_bvbs_dn2, locals.var_exp_bvbs_dn4, locals.var_exp_bvbs_dn5, locals.var_exp_bvbs_dn6, locals.var_exp_bvbs_dn7, locals.var_exp_bvbs_dn8, locals.var_exp_bvbs_dn9, locals.var_exp_bvbs_dn10, locals.var_exp_bvbs_dn13,)
    }
};
        locals.var_exp_bvbs = assign75960_e115450;
        locals.var_exp_bvbs_dn0 = assign75960_e115450_d_n0;
        locals.var_exp_bvbs_dn2 = assign75960_e115450_d_n2;
        locals.var_exp_bvbs_dn4 = assign75960_e115450_d_n4;
        locals.var_exp_bvbs_dn5 = assign75960_e115450_d_n5;
        locals.var_exp_bvbs_dn6 = assign75960_e115450_d_n6;
        locals.var_exp_bvbs_dn7 = assign75960_e115450_d_n7;
        locals.var_exp_bvbs_dn8 = assign75960_e115450_d_n8;
        locals.var_exp_bvbs_dn9 = assign75960_e115450_d_n9;
        locals.var_exp_bvbs_dn10 = assign75960_e115450_d_n10;
        locals.var_exp_bvbs_dn13 = assign75960_e115450_d_n13;
        locals.var_exp_bvbs_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_276(
        locals: &mut StampLocals,
    ) {
        let mut assign75970_loop_guard: usize = 0;
        while {
            let assign75970_cond_e115458: f64 = if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1779 == 0.0)) && (locals.var_tmf1 >= 60.0)) { 1.0 } else { 0.0 };
            assign75970_cond_e115458 != 0.0
        } {
            assign75970_loop_guard += 1;
            assert!(assign75970_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign75970_body0_e115467, assign75970_body0_e115467_d_n0, assign75970_body0_e115467_d_n2, assign75970_body0_e115467_d_n4, assign75970_body0_e115467_d_n5, assign75970_body0_e115467_d_n6, assign75970_body0_e115467_d_n7, assign75970_body0_e115467_d_n8, assign75970_body0_e115467_d_n9, assign75970_body0_e115467_d_n10, assign75970_body0_e115467_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1779 == 0.0)) {
        let assign75970_body0_e115465: f64 = (locals.var_exp_bvbs * 1.14200738981568e26);
        (assign75970_body0_e115465, (locals.var_exp_bvbs_dn0 * 1.14200738981568e26), (locals.var_exp_bvbs_dn2 * 1.14200738981568e26), (locals.var_exp_bvbs_dn4 * 1.14200738981568e26), (locals.var_exp_bvbs_dn5 * 1.14200738981568e26), (locals.var_exp_bvbs_dn6 * 1.14200738981568e26), (locals.var_exp_bvbs_dn7 * 1.14200738981568e26), (locals.var_exp_bvbs_dn8 * 1.14200738981568e26), (locals.var_exp_bvbs_dn9 * 1.14200738981568e26), (locals.var_exp_bvbs_dn10 * 1.14200738981568e26), (locals.var_exp_bvbs_dn13 * 1.14200738981568e26),)
    } else {
        (locals.var_exp_bvbs, locals.var_exp_bvbs_dn0, locals.var_exp_bvbs_dn2, locals.var_exp_bvbs_dn4, locals.var_exp_bvbs_dn5, locals.var_exp_bvbs_dn6, locals.var_exp_bvbs_dn7, locals.var_exp_bvbs_dn8, locals.var_exp_bvbs_dn9, locals.var_exp_bvbs_dn10, locals.var_exp_bvbs_dn13,)
    }
};
            locals.var_exp_bvbs = assign75970_body0_e115467;
            locals.var_exp_bvbs_dn0 = assign75970_body0_e115467_d_n0;
            locals.var_exp_bvbs_dn2 = assign75970_body0_e115467_d_n2;
            locals.var_exp_bvbs_dn4 = assign75970_body0_e115467_d_n4;
            locals.var_exp_bvbs_dn5 = assign75970_body0_e115467_d_n5;
            locals.var_exp_bvbs_dn6 = assign75970_body0_e115467_d_n6;
            locals.var_exp_bvbs_dn7 = assign75970_body0_e115467_d_n7;
            locals.var_exp_bvbs_dn8 = assign75970_body0_e115467_d_n8;
            locals.var_exp_bvbs_dn9 = assign75970_body0_e115467_d_n9;
            locals.var_exp_bvbs_dn10 = assign75970_body0_e115467_d_n10;
            locals.var_exp_bvbs_dn13 = assign75970_body0_e115467_d_n13;
            locals.var_exp_bvbs_rv = 0.0;
            let (assign75970_body1_e115476, assign75970_body1_e115476_d_n0, assign75970_body1_e115476_d_n2, assign75970_body1_e115476_d_n4, assign75970_body1_e115476_d_n5, assign75970_body1_e115476_d_n6, assign75970_body1_e115476_d_n7, assign75970_body1_e115476_d_n8, assign75970_body1_e115476_d_n9, assign75970_body1_e115476_d_n10, assign75970_body1_e115476_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1779 == 0.0)) {
        let assign75970_body1_e115474: f64 = (locals.var_tmf1 - 60.0);
        (assign75970_body1_e115474, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
            locals.var_tmf1 = assign75970_body1_e115476;
            locals.var_tmf1_dn0 = assign75970_body1_e115476_d_n0;
            locals.var_tmf1_dn2 = assign75970_body1_e115476_d_n2;
            locals.var_tmf1_dn4 = assign75970_body1_e115476_d_n4;
            locals.var_tmf1_dn5 = assign75970_body1_e115476_d_n5;
            locals.var_tmf1_dn6 = assign75970_body1_e115476_d_n6;
            locals.var_tmf1_dn7 = assign75970_body1_e115476_d_n7;
            locals.var_tmf1_dn8 = assign75970_body1_e115476_d_n8;
            locals.var_tmf1_dn9 = assign75970_body1_e115476_d_n9;
            locals.var_tmf1_dn10 = assign75970_body1_e115476_d_n10;
            locals.var_tmf1_dn13 = assign75970_body1_e115476_d_n13;
            locals.var_tmf1_rv = 0.0;
        }

        let (assign75980_e115486, assign75980_e115486_d_n0, assign75980_e115486_d_n2, assign75980_e115486_d_n4, assign75980_e115486_d_n5, assign75980_e115486_d_n6, assign75980_e115486_d_n7, assign75980_e115486_d_n8, assign75980_e115486_d_n9, assign75980_e115486_d_n10, assign75980_e115486_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1779 == 0.0)) {
        let assign75980_e115483: f64 = (locals.var_tmf1).exp();
        let assign75980_e115484: f64 = (locals.var_exp_bvbs * assign75980_e115483);
        (assign75980_e115484, ((locals.var_exp_bvbs_dn0 * assign75980_e115483) + (locals.var_exp_bvbs * (assign75980_e115483 * locals.var_tmf1_dn0))), ((locals.var_exp_bvbs_dn2 * assign75980_e115483) + (locals.var_exp_bvbs * (assign75980_e115483 * locals.var_tmf1_dn2))), ((locals.var_exp_bvbs_dn4 * assign75980_e115483) + (locals.var_exp_bvbs * (assign75980_e115483 * locals.var_tmf1_dn4))), ((locals.var_exp_bvbs_dn5 * assign75980_e115483) + (locals.var_exp_bvbs * (assign75980_e115483 * locals.var_tmf1_dn5))), ((locals.var_exp_bvbs_dn6 * assign75980_e115483) + (locals.var_exp_bvbs * (assign75980_e115483 * locals.var_tmf1_dn6))), ((locals.var_exp_bvbs_dn7 * assign75980_e115483) + (locals.var_exp_bvbs * (assign75980_e115483 * locals.var_tmf1_dn7))), ((locals.var_exp_bvbs_dn8 * assign75980_e115483) + (locals.var_exp_bvbs * (assign75980_e115483 * locals.var_tmf1_dn8))), ((locals.var_exp_bvbs_dn9 * assign75980_e115483) + (locals.var_exp_bvbs * (assign75980_e115483 * locals.var_tmf1_dn9))), ((locals.var_exp_bvbs_dn10 * assign75980_e115483) + (locals.var_exp_bvbs * (assign75980_e115483 * locals.var_tmf1_dn10))), ((locals.var_exp_bvbs_dn13 * assign75980_e115483) + (locals.var_exp_bvbs * (assign75980_e115483 * locals.var_tmf1_dn13))),)
    } else {
        (locals.var_exp_bvbs, locals.var_exp_bvbs_dn0, locals.var_exp_bvbs_dn2, locals.var_exp_bvbs_dn4, locals.var_exp_bvbs_dn5, locals.var_exp_bvbs_dn6, locals.var_exp_bvbs_dn7, locals.var_exp_bvbs_dn8, locals.var_exp_bvbs_dn9, locals.var_exp_bvbs_dn10, locals.var_exp_bvbs_dn13,)
    }
};
        locals.var_exp_bvbs = assign75980_e115486;
        locals.var_exp_bvbs_dn0 = assign75980_e115486_d_n0;
        locals.var_exp_bvbs_dn2 = assign75980_e115486_d_n2;
        locals.var_exp_bvbs_dn4 = assign75980_e115486_d_n4;
        locals.var_exp_bvbs_dn5 = assign75980_e115486_d_n5;
        locals.var_exp_bvbs_dn6 = assign75980_e115486_d_n6;
        locals.var_exp_bvbs_dn7 = assign75980_e115486_d_n7;
        locals.var_exp_bvbs_dn8 = assign75980_e115486_d_n8;
        locals.var_exp_bvbs_dn9 = assign75980_e115486_d_n9;
        locals.var_exp_bvbs_dn10 = assign75980_e115486_d_n10;
        locals.var_exp_bvbs_dn13 = assign75980_e115486_d_n13;
        locals.var_exp_bvbs_rv = 0.0;

        let (assign75990_e115493, assign75990_e115493_d_n0, assign75990_e115493_d_n2, assign75990_e115493_d_n4, assign75990_e115493_d_n5, assign75990_e115493_d_n6, assign75990_e115493_d_n7, assign75990_e115493_d_n8, assign75990_e115493_d_n9, assign75990_e115493_d_n10, assign75990_e115493_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1779 == 0.0)) {
        (locals.var_exp_bvbs, locals.var_exp_bvbs_dn0, locals.var_exp_bvbs_dn2, locals.var_exp_bvbs_dn4, locals.var_exp_bvbs_dn5, locals.var_exp_bvbs_dn6, locals.var_exp_bvbs_dn7, locals.var_exp_bvbs_dn8, locals.var_exp_bvbs_dn9, locals.var_exp_bvbs_dn10, locals.var_exp_bvbs_dn13,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign75990_e115493;
        locals.var_t0_dn0 = assign75990_e115493_d_n0;
        locals.var_t0_dn2 = assign75990_e115493_d_n2;
        locals.var_t0_dn4 = assign75990_e115493_d_n4;
        locals.var_t0_dn5 = assign75990_e115493_d_n5;
        locals.var_t0_dn6 = assign75990_e115493_d_n6;
        locals.var_t0_dn7 = assign75990_e115493_d_n7;
        locals.var_t0_dn8 = assign75990_e115493_d_n8;
        locals.var_t0_dn9 = assign75990_e115493_d_n9;
        locals.var_t0_dn10 = assign75990_e115493_d_n10;
        locals.var_t0_dn13 = assign75990_e115493_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign76000_e115506, assign76000_e115506_d_n0, assign76000_e115506_d_n2, assign76000_e115506_d_n4, assign76000_e115506_d_n5, assign76000_e115506_d_n6, assign76000_e115506_d_n7, assign76000_e115506_d_n8, assign76000_e115506_d_n9, assign76000_e115506_d_n10, assign76000_e115506_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) {
        let assign76000_e115498: f64 = (-locals.var_vgpld);
        let assign76000_e115500: f64 = (assign76000_e115498 * 0.5);
        let assign76000_e115502: f64 = (assign76000_e115500 - 0.5);
        let assign76000_e115504: f64 = (assign76000_e115502 - 1.0);
        (assign76000_e115504, 0.0, ((-locals.var_vgpld_dn2) * 0.5), 0.0, 0.0, ((-locals.var_vgpld_dn6) * 0.5), ((-locals.var_vgpld_dn7) * 0.5), ((-locals.var_vgpld_dn8) * 0.5), 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign76000_e115506;
        locals.var_tmf1_dn0 = assign76000_e115506_d_n0;
        locals.var_tmf1_dn2 = assign76000_e115506_d_n2;
        locals.var_tmf1_dn4 = assign76000_e115506_d_n4;
        locals.var_tmf1_dn5 = assign76000_e115506_d_n5;
        locals.var_tmf1_dn6 = assign76000_e115506_d_n6;
        locals.var_tmf1_dn7 = assign76000_e115506_d_n7;
        locals.var_tmf1_dn8 = assign76000_e115506_d_n8;
        locals.var_tmf1_dn9 = assign76000_e115506_d_n9;
        locals.var_tmf1_dn10 = assign76000_e115506_d_n10;
        locals.var_tmf1_dn13 = assign76000_e115506_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign76010_e115516, assign76010_e115516_d_n0, assign76010_e115516_d_n2, assign76010_e115516_d_n4, assign76010_e115516_d_n5, assign76010_e115516_d_n6, assign76010_e115516_d_n7, assign76010_e115516_d_n8, assign76010_e115516_d_n9, assign76010_e115516_d_n10, assign76010_e115516_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) {
        let assign76010_e115512: f64 = (4.0 * 0.5);
        let assign76010_e115514: f64 = assign76010_e115512;
        (assign76010_e115514, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign76010_e115516;
        locals.var_tmf2_dn0 = assign76010_e115516_d_n0;
        locals.var_tmf2_dn2 = assign76010_e115516_d_n2;
        locals.var_tmf2_dn4 = assign76010_e115516_d_n4;
        locals.var_tmf2_dn5 = assign76010_e115516_d_n5;
        locals.var_tmf2_dn6 = assign76010_e115516_d_n6;
        locals.var_tmf2_dn7 = assign76010_e115516_d_n7;
        locals.var_tmf2_dn8 = assign76010_e115516_d_n8;
        locals.var_tmf2_dn9 = assign76010_e115516_d_n9;
        locals.var_tmf2_dn10 = assign76010_e115516_d_n10;
        locals.var_tmf2_dn13 = assign76010_e115516_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign76020_e115528, assign76020_e115528_d_n0, assign76020_e115528_d_n2, assign76020_e115528_d_n4, assign76020_e115528_d_n5, assign76020_e115528_d_n6, assign76020_e115528_d_n7, assign76020_e115528_d_n8, assign76020_e115528_d_n9, assign76020_e115528_d_n10, assign76020_e115528_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) {
        let (assign76020_e115526, assign76020_e115526_d_n0, assign76020_e115526_d_n2, assign76020_e115526_d_n4, assign76020_e115526_d_n5, assign76020_e115526_d_n6, assign76020_e115526_d_n7, assign76020_e115526_d_n8, assign76020_e115526_d_n9, assign76020_e115526_d_n10, assign76020_e115526_d_n13,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
            } else {
                let assign76020_e115525: f64 = (-locals.var_tmf2);
                (assign76020_e115525, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn13),)
            }
        };
        (assign76020_e115526, assign76020_e115526_d_n0, assign76020_e115526_d_n2, assign76020_e115526_d_n4, assign76020_e115526_d_n5, assign76020_e115526_d_n6, assign76020_e115526_d_n7, assign76020_e115526_d_n8, assign76020_e115526_d_n9, assign76020_e115526_d_n10, assign76020_e115526_d_n13,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign76020_e115528;
        locals.var_tmf2_dn0 = assign76020_e115528_d_n0;
        locals.var_tmf2_dn2 = assign76020_e115528_d_n2;
        locals.var_tmf2_dn4 = assign76020_e115528_d_n4;
        locals.var_tmf2_dn5 = assign76020_e115528_d_n5;
        locals.var_tmf2_dn6 = assign76020_e115528_d_n6;
        locals.var_tmf2_dn7 = assign76020_e115528_d_n7;
        locals.var_tmf2_dn8 = assign76020_e115528_d_n8;
        locals.var_tmf2_dn9 = assign76020_e115528_d_n9;
        locals.var_tmf2_dn10 = assign76020_e115528_d_n10;
        locals.var_tmf2_dn13 = assign76020_e115528_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign76030_e115539, assign76030_e115539_d_n0, assign76030_e115539_d_n2, assign76030_e115539_d_n4, assign76030_e115539_d_n5, assign76030_e115539_d_n6, assign76030_e115539_d_n7, assign76030_e115539_d_n8, assign76030_e115539_d_n9, assign76030_e115539_d_n10, assign76030_e115539_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) {
        let assign76030_e115534: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign76030_e115536: f64 = (assign76030_e115534 + locals.var_tmf2);
        let assign76030_e115537: f64 = (assign76030_e115536).sqrt();
        (assign76030_e115537, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign76030_e115537)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign76030_e115537)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign76030_e115537)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign76030_e115537)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign76030_e115537)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign76030_e115537)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign76030_e115537)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign76030_e115537)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign76030_e115537)), ((((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)) + locals.var_tmf2_dn13) / (2.0 * assign76030_e115537)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign76030_e115539;
        locals.var_tmf2_dn0 = assign76030_e115539_d_n0;
        locals.var_tmf2_dn2 = assign76030_e115539_d_n2;
        locals.var_tmf2_dn4 = assign76030_e115539_d_n4;
        locals.var_tmf2_dn5 = assign76030_e115539_d_n5;
        locals.var_tmf2_dn6 = assign76030_e115539_d_n6;
        locals.var_tmf2_dn7 = assign76030_e115539_d_n7;
        locals.var_tmf2_dn8 = assign76030_e115539_d_n8;
        locals.var_tmf2_dn9 = assign76030_e115539_d_n9;
        locals.var_tmf2_dn10 = assign76030_e115539_d_n10;
        locals.var_tmf2_dn13 = assign76030_e115539_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign76040_e115551, assign76040_e115551_d_n0, assign76040_e115551_d_n2, assign76040_e115551_d_n4, assign76040_e115551_d_n5, assign76040_e115551_d_n6, assign76040_e115551_d_n7, assign76040_e115551_d_n8, assign76040_e115551_d_n9, assign76040_e115551_d_n10, assign76040_e115551_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) {
        let assign76040_e115547: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign76040_e115548: f64 = (1.0 + assign76040_e115547);
        let assign76040_e115549: f64 = (0.5 * assign76040_e115548);
        (assign76040_e115549, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn13 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign76040_e115551;
        locals.var_t0_dn0 = assign76040_e115551_d_n0;
        locals.var_t0_dn2 = assign76040_e115551_d_n2;
        locals.var_t0_dn4 = assign76040_e115551_d_n4;
        locals.var_t0_dn5 = assign76040_e115551_d_n5;
        locals.var_t0_dn6 = assign76040_e115551_d_n6;
        locals.var_t0_dn7 = assign76040_e115551_d_n7;
        locals.var_t0_dn8 = assign76040_e115551_d_n8;
        locals.var_t0_dn9 = assign76040_e115551_d_n9;
        locals.var_t0_dn10 = assign76040_e115551_d_n10;
        locals.var_t0_dn13 = assign76040_e115551_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign76050_e115563, assign76050_e115563_d_n0, assign76050_e115563_d_n2, assign76050_e115563_d_n4, assign76050_e115563_d_n5, assign76050_e115563_d_n6, assign76050_e115563_d_n7, assign76050_e115563_d_n8, assign76050_e115563_d_n9, assign76050_e115563_d_n10, assign76050_e115563_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) {
        let assign76050_e115559: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign76050_e115560: f64 = (0.5 * assign76050_e115559);
        let assign76050_e115561: f64 = (0.5 + assign76050_e115560);
        (assign76050_e115561, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_tmf1_dn13 + locals.var_tmf2_dn13)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign76050_e115563;
        locals.var_t1_dn0 = assign76050_e115563_d_n0;
        locals.var_t1_dn2 = assign76050_e115563_d_n2;
        locals.var_t1_dn4 = assign76050_e115563_d_n4;
        locals.var_t1_dn5 = assign76050_e115563_d_n5;
        locals.var_t1_dn6 = assign76050_e115563_d_n6;
        locals.var_t1_dn7 = assign76050_e115563_d_n7;
        locals.var_t1_dn8 = assign76050_e115563_d_n8;
        locals.var_t1_dn9 = assign76050_e115563_d_n9;
        locals.var_t1_dn10 = assign76050_e115563_d_n10;
        locals.var_t1_dn13 = assign76050_e115563_d_n13;
        locals.var_t1_rv = 0.0;

        let assign76060_e115566: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign76060_e115569: f64 = (-locals.var_t1);
        let assign76060_e115574: f64 = if ((assign76060_e115566 > assign76060_e115569) && (locals.var_t1 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1780 = assign76060_e115574;
        locals.var_guard1780_rv = 0.0;

        let (assign76070_e115588, assign76070_e115588_d_n0, assign76070_e115588_d_n2, assign76070_e115588_d_n4, assign76070_e115588_d_n5, assign76070_e115588_d_n6, assign76070_e115588_d_n7, assign76070_e115588_d_n8, assign76070_e115588_d_n9, assign76070_e115588_d_n10, assign76070_e115588_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1780 != 0.0)) {
        let assign76070_e115582: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign76070_e115584: f64 = assign76070_e115582;
        let assign76070_e115586: f64 = (assign76070_e115584 + locals.var_t1);
        (assign76070_e115586, (locals.var_vxbgmtcl_dn0 + locals.var_t1_dn0), ((locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2) + locals.var_t1_dn2), (locals.var_vxbgmtcl_dn4 + locals.var_t1_dn4), (locals.var_vxbgmtcl_dn5 + locals.var_t1_dn5), ((locals.var_vgpld_dn6 + locals.var_vxbgmtcl_dn6) + locals.var_t1_dn6), ((locals.var_vgpld_dn7 + locals.var_vxbgmtcl_dn7) + locals.var_t1_dn7), ((locals.var_vgpld_dn8 + locals.var_vxbgmtcl_dn8) + locals.var_t1_dn8), (locals.var_vxbgmtcl_dn9 + locals.var_t1_dn9), (locals.var_vxbgmtcl_dn10 + locals.var_t1_dn10), (locals.var_vxbgmtcl_dn13 + locals.var_t1_dn13),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign76070_e115588;
        locals.var_tmf1_dn0 = assign76070_e115588_d_n0;
        locals.var_tmf1_dn2 = assign76070_e115588_d_n2;
        locals.var_tmf1_dn4 = assign76070_e115588_d_n4;
        locals.var_tmf1_dn5 = assign76070_e115588_d_n5;
        locals.var_tmf1_dn6 = assign76070_e115588_d_n6;
        locals.var_tmf1_dn7 = assign76070_e115588_d_n7;
        locals.var_tmf1_dn8 = assign76070_e115588_d_n8;
        locals.var_tmf1_dn9 = assign76070_e115588_d_n9;
        locals.var_tmf1_dn10 = assign76070_e115588_d_n10;
        locals.var_tmf1_dn13 = assign76070_e115588_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign76080_e115598, assign76080_e115598_d_n0, assign76080_e115598_d_n2, assign76080_e115598_d_n4, assign76080_e115598_d_n5, assign76080_e115598_d_n6, assign76080_e115598_d_n7, assign76080_e115598_d_n8, assign76080_e115598_d_n9, assign76080_e115598_d_n10, assign76080_e115598_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1780 != 0.0)) {
        let assign76080_e115596: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign76080_e115596, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn13,)
    }
};
        locals.var_x2 = assign76080_e115598;
        locals.var_x2_dn0 = assign76080_e115598_d_n0;
        locals.var_x2_dn2 = assign76080_e115598_d_n2;
        locals.var_x2_dn4 = assign76080_e115598_d_n4;
        locals.var_x2_dn5 = assign76080_e115598_d_n5;
        locals.var_x2_dn6 = assign76080_e115598_d_n6;
        locals.var_x2_dn7 = assign76080_e115598_d_n7;
        locals.var_x2_dn8 = assign76080_e115598_d_n8;
        locals.var_x2_dn9 = assign76080_e115598_d_n9;
        locals.var_x2_dn10 = assign76080_e115598_d_n10;
        locals.var_x2_dn13 = assign76080_e115598_d_n13;
        locals.var_x2_rv = 0.0;

        let (assign76090_e115608, assign76090_e115608_d_n0, assign76090_e115608_d_n2, assign76090_e115608_d_n4, assign76090_e115608_d_n5, assign76090_e115608_d_n6, assign76090_e115608_d_n7, assign76090_e115608_d_n8, assign76090_e115608_d_n9, assign76090_e115608_d_n10, assign76090_e115608_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1780 != 0.0)) {
        let assign76090_e115606: f64 = (locals.var_t1 * locals.var_t1);
        (assign76090_e115606, ((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0)), ((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2)), ((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)), ((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)), ((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)), ((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)), ((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)), ((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)), ((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)), ((locals.var_t1_dn13 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn13)),)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn13,)
    }
};
        locals.var_xmax2 = assign76090_e115608;
        locals.var_xmax2_dn0 = assign76090_e115608_d_n0;
        locals.var_xmax2_dn2 = assign76090_e115608_d_n2;
        locals.var_xmax2_dn4 = assign76090_e115608_d_n4;
        locals.var_xmax2_dn5 = assign76090_e115608_d_n5;
        locals.var_xmax2_dn6 = assign76090_e115608_d_n6;
        locals.var_xmax2_dn7 = assign76090_e115608_d_n7;
        locals.var_xmax2_dn8 = assign76090_e115608_d_n8;
        locals.var_xmax2_dn9 = assign76090_e115608_d_n9;
        locals.var_xmax2_dn10 = assign76090_e115608_d_n10;
        locals.var_xmax2_dn13 = assign76090_e115608_d_n13;
        locals.var_xmax2_rv = 0.0;

        let (assign76100_e115616, assign76100_e115616_d_n0, assign76100_e115616_d_n2, assign76100_e115616_d_n4, assign76100_e115616_d_n5, assign76100_e115616_d_n6, assign76100_e115616_d_n7, assign76100_e115616_d_n8, assign76100_e115616_d_n9, assign76100_e115616_d_n10, assign76100_e115616_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1780 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign76100_e115616;
        locals.var_xp_dn0 = assign76100_e115616_d_n0;
        locals.var_xp_dn2 = assign76100_e115616_d_n2;
        locals.var_xp_dn4 = assign76100_e115616_d_n4;
        locals.var_xp_dn5 = assign76100_e115616_d_n5;
        locals.var_xp_dn6 = assign76100_e115616_d_n6;
        locals.var_xp_dn7 = assign76100_e115616_d_n7;
        locals.var_xp_dn8 = assign76100_e115616_d_n8;
        locals.var_xp_dn9 = assign76100_e115616_d_n9;
        locals.var_xp_dn10 = assign76100_e115616_d_n10;
        locals.var_xp_dn13 = assign76100_e115616_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign76110_e115624, assign76110_e115624_d_n0, assign76110_e115624_d_n2, assign76110_e115624_d_n4, assign76110_e115624_d_n5, assign76110_e115624_d_n6, assign76110_e115624_d_n7, assign76110_e115624_d_n8, assign76110_e115624_d_n9, assign76110_e115624_d_n10, assign76110_e115624_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1780 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign76110_e115624;
        locals.var_xmp_dn0 = assign76110_e115624_d_n0;
        locals.var_xmp_dn2 = assign76110_e115624_d_n2;
        locals.var_xmp_dn4 = assign76110_e115624_d_n4;
        locals.var_xmp_dn5 = assign76110_e115624_d_n5;
        locals.var_xmp_dn6 = assign76110_e115624_d_n6;
        locals.var_xmp_dn7 = assign76110_e115624_d_n7;
        locals.var_xmp_dn8 = assign76110_e115624_d_n8;
        locals.var_xmp_dn9 = assign76110_e115624_d_n9;
        locals.var_xmp_dn10 = assign76110_e115624_d_n10;
        locals.var_xmp_dn13 = assign76110_e115624_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign76120_e115632,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1780 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign76120_e115632;
        locals.var_m0_rv = 0.0;

        let (assign76130_e115640,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1780 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign76130_e115640;
        locals.var_mm_rv = 0.0;

        let (assign76140_e115648, assign76140_e115648_d_n0, assign76140_e115648_d_n2, assign76140_e115648_d_n4, assign76140_e115648_d_n5, assign76140_e115648_d_n6, assign76140_e115648_d_n7, assign76140_e115648_d_n8, assign76140_e115648_d_n9, assign76140_e115648_d_n10, assign76140_e115648_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1780 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign76140_e115648;
        locals.var_arg_dn0 = assign76140_e115648_d_n0;
        locals.var_arg_dn2 = assign76140_e115648_d_n2;
        locals.var_arg_dn4 = assign76140_e115648_d_n4;
        locals.var_arg_dn5 = assign76140_e115648_d_n5;
        locals.var_arg_dn6 = assign76140_e115648_d_n6;
        locals.var_arg_dn7 = assign76140_e115648_d_n7;
        locals.var_arg_dn8 = assign76140_e115648_d_n8;
        locals.var_arg_dn9 = assign76140_e115648_d_n9;
        locals.var_arg_dn10 = assign76140_e115648_d_n10;
        locals.var_arg_dn13 = assign76140_e115648_d_n13;
        locals.var_arg_rv = 0.0;

        let (assign76150_e115656, assign76150_e115656_d_n0, assign76150_e115656_d_n2, assign76150_e115656_d_n4, assign76150_e115656_d_n5, assign76150_e115656_d_n6, assign76150_e115656_d_n7, assign76150_e115656_d_n8, assign76150_e115656_d_n9, assign76150_e115656_d_n10, assign76150_e115656_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1780 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign76150_e115656;
        locals.var_dnm_dn0 = assign76150_e115656_d_n0;
        locals.var_dnm_dn2 = assign76150_e115656_d_n2;
        locals.var_dnm_dn4 = assign76150_e115656_d_n4;
        locals.var_dnm_dn5 = assign76150_e115656_d_n5;
        locals.var_dnm_dn6 = assign76150_e115656_d_n6;
        locals.var_dnm_dn7 = assign76150_e115656_d_n7;
        locals.var_dnm_dn8 = assign76150_e115656_d_n8;
        locals.var_dnm_dn9 = assign76150_e115656_d_n9;
        locals.var_dnm_dn10 = assign76150_e115656_d_n10;
        locals.var_dnm_dn13 = assign76150_e115656_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign76160_e115666, assign76160_e115666_d_n0, assign76160_e115666_d_n2, assign76160_e115666_d_n4, assign76160_e115666_d_n5, assign76160_e115666_d_n6, assign76160_e115666_d_n7, assign76160_e115666_d_n8, assign76160_e115666_d_n9, assign76160_e115666_d_n10, assign76160_e115666_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1780 != 0.0)) {
        let assign76160_e115664: f64 = (locals.var_xp * locals.var_x2);
        (assign76160_e115664, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign76160_e115666;
        locals.var_xp_dn0 = assign76160_e115666_d_n0;
        locals.var_xp_dn2 = assign76160_e115666_d_n2;
        locals.var_xp_dn4 = assign76160_e115666_d_n4;
        locals.var_xp_dn5 = assign76160_e115666_d_n5;
        locals.var_xp_dn6 = assign76160_e115666_d_n6;
        locals.var_xp_dn7 = assign76160_e115666_d_n7;
        locals.var_xp_dn8 = assign76160_e115666_d_n8;
        locals.var_xp_dn9 = assign76160_e115666_d_n9;
        locals.var_xp_dn10 = assign76160_e115666_d_n10;
        locals.var_xp_dn13 = assign76160_e115666_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign76170_e115676, assign76170_e115676_d_n0, assign76170_e115676_d_n2, assign76170_e115676_d_n4, assign76170_e115676_d_n5, assign76170_e115676_d_n6, assign76170_e115676_d_n7, assign76170_e115676_d_n8, assign76170_e115676_d_n9, assign76170_e115676_d_n10, assign76170_e115676_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1780 != 0.0)) {
        let assign76170_e115674: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign76170_e115674, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign76170_e115676;
        locals.var_xmp_dn0 = assign76170_e115676_d_n0;
        locals.var_xmp_dn2 = assign76170_e115676_d_n2;
        locals.var_xmp_dn4 = assign76170_e115676_d_n4;
        locals.var_xmp_dn5 = assign76170_e115676_d_n5;
        locals.var_xmp_dn6 = assign76170_e115676_d_n6;
        locals.var_xmp_dn7 = assign76170_e115676_d_n7;
        locals.var_xmp_dn8 = assign76170_e115676_d_n8;
        locals.var_xmp_dn9 = assign76170_e115676_d_n9;
        locals.var_xmp_dn10 = assign76170_e115676_d_n10;
        locals.var_xmp_dn13 = assign76170_e115676_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign76180_e115686, assign76180_e115686_d_n0, assign76180_e115686_d_n2, assign76180_e115686_d_n4, assign76180_e115686_d_n5, assign76180_e115686_d_n6, assign76180_e115686_d_n7, assign76180_e115686_d_n8, assign76180_e115686_d_n9, assign76180_e115686_d_n10, assign76180_e115686_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1780 != 0.0)) {
        let assign76180_e115684: f64 = (locals.var_xp + locals.var_xmp);
        (assign76180_e115684, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn13 + locals.var_xmp_dn13),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign76180_e115686;
        locals.var_arg_dn0 = assign76180_e115686_d_n0;
        locals.var_arg_dn2 = assign76180_e115686_d_n2;
        locals.var_arg_dn4 = assign76180_e115686_d_n4;
        locals.var_arg_dn5 = assign76180_e115686_d_n5;
        locals.var_arg_dn6 = assign76180_e115686_d_n6;
        locals.var_arg_dn7 = assign76180_e115686_d_n7;
        locals.var_arg_dn8 = assign76180_e115686_d_n8;
        locals.var_arg_dn9 = assign76180_e115686_d_n9;
        locals.var_arg_dn10 = assign76180_e115686_d_n10;
        locals.var_arg_dn13 = assign76180_e115686_d_n13;
        locals.var_arg_rv = 0.0;

        let (assign76190_e115694, assign76190_e115694_d_n0, assign76190_e115694_d_n2, assign76190_e115694_d_n4, assign76190_e115694_d_n5, assign76190_e115694_d_n6, assign76190_e115694_d_n7, assign76190_e115694_d_n8, assign76190_e115694_d_n9, assign76190_e115694_d_n10, assign76190_e115694_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1780 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign76190_e115694;
        locals.var_dnm_dn0 = assign76190_e115694_d_n0;
        locals.var_dnm_dn2 = assign76190_e115694_d_n2;
        locals.var_dnm_dn4 = assign76190_e115694_d_n4;
        locals.var_dnm_dn5 = assign76190_e115694_d_n5;
        locals.var_dnm_dn6 = assign76190_e115694_d_n6;
        locals.var_dnm_dn7 = assign76190_e115694_d_n7;
        locals.var_dnm_dn8 = assign76190_e115694_d_n8;
        locals.var_dnm_dn9 = assign76190_e115694_d_n9;
        locals.var_dnm_dn10 = assign76190_e115694_d_n10;
        locals.var_dnm_dn13 = assign76190_e115694_d_n13;
        locals.var_dnm_rv = 0.0;

        let assign76200_e115709: f64 = if ((((1.0 == 1.0) || (1.0 == 2.0)) || (1.0 == 4.0)) || (1.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard1781 = assign76200_e115709;
        locals.var_guard1781_rv = 0.0;

        let assign76210_e115712: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1782 = assign76210_e115712;
        locals.var_guard1782_rv = 0.0;

        let (assign76220_e115724,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1780 != 0.0)) && (locals.var_guard1781 != 0.0)) && (locals.var_guard1782 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign76220_e115724;
        locals.var_mm_rv = 0.0;

        let assign76230_e115727: f64 = if 1.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1783 = assign76230_e115727;
        locals.var_guard1783_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_277(
        locals: &mut StampLocals,
    ) {
        let (assign76240_e115742,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1780 != 0.0)) && (locals.var_guard1781 != 0.0)) && (locals.var_guard1782 == 0.0)) && (locals.var_guard1783 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign76240_e115742;
        locals.var_mm_rv = 0.0;

        let assign76250_e115745: f64 = if 1.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard1784 = assign76250_e115745;
        locals.var_guard1784_rv = 0.0;

        let (assign76260_e115763,) = {
    if (((((((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1780 != 0.0)) && (locals.var_guard1781 != 0.0)) && (locals.var_guard1782 == 0.0)) && (locals.var_guard1783 == 0.0)) && (locals.var_guard1784 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign76260_e115763;
        locals.var_mm_rv = 0.0;

        let assign76270_e115766: f64 = if 1.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard1785 = assign76270_e115766;
        locals.var_guard1785_rv = 0.0;

        let (assign76280_e115787,) = {
    if ((((((((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1780 != 0.0)) && (locals.var_guard1781 != 0.0)) && (locals.var_guard1782 == 0.0)) && (locals.var_guard1783 == 0.0)) && (locals.var_guard1784 == 0.0)) && (locals.var_guard1785 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign76280_e115787;
        locals.var_mm_rv = 0.0;

        let (assign76290_e115797,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1780 != 0.0)) && (locals.var_guard1781 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign76290_e115797;
        locals.var_m0_rv = 0.0;

        let mut assign76300_loop_guard: usize = 0;
        while {
            let assign76300_cond_e115808: f64 = if (((((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1780 != 0.0)) && (locals.var_guard1781 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign76300_cond_e115808 != 0.0
        } {
            assign76300_loop_guard += 1;
            assert!(assign76300_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign76300_body0_e115819, assign76300_body0_e115819_d_n0, assign76300_body0_e115819_d_n2, assign76300_body0_e115819_d_n4, assign76300_body0_e115819_d_n5, assign76300_body0_e115819_d_n6, assign76300_body0_e115819_d_n7, assign76300_body0_e115819_d_n8, assign76300_body0_e115819_d_n9, assign76300_body0_e115819_d_n10, assign76300_body0_e115819_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1780 != 0.0)) && (locals.var_guard1781 != 0.0)) {
        let assign76300_body0_e115817: f64 = (locals.var_dnm).sqrt();
        (assign76300_body0_e115817, (locals.var_dnm_dn0 / (2.0 * assign76300_body0_e115817)), (locals.var_dnm_dn2 / (2.0 * assign76300_body0_e115817)), (locals.var_dnm_dn4 / (2.0 * assign76300_body0_e115817)), (locals.var_dnm_dn5 / (2.0 * assign76300_body0_e115817)), (locals.var_dnm_dn6 / (2.0 * assign76300_body0_e115817)), (locals.var_dnm_dn7 / (2.0 * assign76300_body0_e115817)), (locals.var_dnm_dn8 / (2.0 * assign76300_body0_e115817)), (locals.var_dnm_dn9 / (2.0 * assign76300_body0_e115817)), (locals.var_dnm_dn10 / (2.0 * assign76300_body0_e115817)), (locals.var_dnm_dn13 / (2.0 * assign76300_body0_e115817)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
            locals.var_dnm = assign76300_body0_e115819;
            locals.var_dnm_dn0 = assign76300_body0_e115819_d_n0;
            locals.var_dnm_dn2 = assign76300_body0_e115819_d_n2;
            locals.var_dnm_dn4 = assign76300_body0_e115819_d_n4;
            locals.var_dnm_dn5 = assign76300_body0_e115819_d_n5;
            locals.var_dnm_dn6 = assign76300_body0_e115819_d_n6;
            locals.var_dnm_dn7 = assign76300_body0_e115819_d_n7;
            locals.var_dnm_dn8 = assign76300_body0_e115819_d_n8;
            locals.var_dnm_dn9 = assign76300_body0_e115819_d_n9;
            locals.var_dnm_dn10 = assign76300_body0_e115819_d_n10;
            locals.var_dnm_dn13 = assign76300_body0_e115819_d_n13;
            locals.var_dnm_rv = 0.0;
            let (assign76300_body1_e115831,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1780 != 0.0)) && (locals.var_guard1781 != 0.0)) {
        let assign76300_body1_e115829: f64 = (locals.var_m0 + 1.0);
        (assign76300_body1_e115829,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign76300_body1_e115831;
            locals.var_m0_rv = 0.0;
        }

        let (assign76310_e115853, assign76310_e115853_d_n0, assign76310_e115853_d_n2, assign76310_e115853_d_n4, assign76310_e115853_d_n5, assign76310_e115853_d_n6, assign76310_e115853_d_n7, assign76310_e115853_d_n8, assign76310_e115853_d_n9, assign76310_e115853_d_n10, assign76310_e115853_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1780 != 0.0)) && (locals.var_guard1781 == 0.0)) {
        let (assign76310_e115851, assign76310_e115851_d_n0, assign76310_e115851_d_n2, assign76310_e115851_d_n4, assign76310_e115851_d_n5, assign76310_e115851_d_n6, assign76310_e115851_d_n7, assign76310_e115851_d_n8, assign76310_e115851_d_n9, assign76310_e115851_d_n10, assign76310_e115851_d_n13,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign76310_e115848: f64 = 2.0;
                let assign76310_e115849: f64 = (1.0 / assign76310_e115848);
                let assign76310_e115850: f64 = (locals.var_dnm).powf(assign76310_e115849);
                (assign76310_e115850, if 0.0 == 0.0 && ((assign76310_e115849) as f64).is_finite() && ((assign76310_e115849) as f64).fract() == 0.0 { if assign76310_e115849 == 0.0 { 0.0 } else { (assign76310_e115849 * ((locals.var_dnm).powf(assign76310_e115849 - 1.0) * locals.var_dnm_dn0)) } } else { (assign76310_e115850 * (assign76310_e115849 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign76310_e115849) as f64).is_finite() && ((assign76310_e115849) as f64).fract() == 0.0 { if assign76310_e115849 == 0.0 { 0.0 } else { (assign76310_e115849 * ((locals.var_dnm).powf(assign76310_e115849 - 1.0) * locals.var_dnm_dn2)) } } else { (assign76310_e115850 * (assign76310_e115849 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign76310_e115849) as f64).is_finite() && ((assign76310_e115849) as f64).fract() == 0.0 { if assign76310_e115849 == 0.0 { 0.0 } else { (assign76310_e115849 * ((locals.var_dnm).powf(assign76310_e115849 - 1.0) * locals.var_dnm_dn4)) } } else { (assign76310_e115850 * (assign76310_e115849 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign76310_e115849) as f64).is_finite() && ((assign76310_e115849) as f64).fract() == 0.0 { if assign76310_e115849 == 0.0 { 0.0 } else { (assign76310_e115849 * ((locals.var_dnm).powf(assign76310_e115849 - 1.0) * locals.var_dnm_dn5)) } } else { (assign76310_e115850 * (assign76310_e115849 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign76310_e115849) as f64).is_finite() && ((assign76310_e115849) as f64).fract() == 0.0 { if assign76310_e115849 == 0.0 { 0.0 } else { (assign76310_e115849 * ((locals.var_dnm).powf(assign76310_e115849 - 1.0) * locals.var_dnm_dn6)) } } else { (assign76310_e115850 * (assign76310_e115849 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign76310_e115849) as f64).is_finite() && ((assign76310_e115849) as f64).fract() == 0.0 { if assign76310_e115849 == 0.0 { 0.0 } else { (assign76310_e115849 * ((locals.var_dnm).powf(assign76310_e115849 - 1.0) * locals.var_dnm_dn7)) } } else { (assign76310_e115850 * (assign76310_e115849 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign76310_e115849) as f64).is_finite() && ((assign76310_e115849) as f64).fract() == 0.0 { if assign76310_e115849 == 0.0 { 0.0 } else { (assign76310_e115849 * ((locals.var_dnm).powf(assign76310_e115849 - 1.0) * locals.var_dnm_dn8)) } } else { (assign76310_e115850 * (assign76310_e115849 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign76310_e115849) as f64).is_finite() && ((assign76310_e115849) as f64).fract() == 0.0 { if assign76310_e115849 == 0.0 { 0.0 } else { (assign76310_e115849 * ((locals.var_dnm).powf(assign76310_e115849 - 1.0) * locals.var_dnm_dn9)) } } else { (assign76310_e115850 * (assign76310_e115849 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign76310_e115849) as f64).is_finite() && ((assign76310_e115849) as f64).fract() == 0.0 { if assign76310_e115849 == 0.0 { 0.0 } else { (assign76310_e115849 * ((locals.var_dnm).powf(assign76310_e115849 - 1.0) * locals.var_dnm_dn10)) } } else { (assign76310_e115850 * (assign76310_e115849 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign76310_e115849) as f64).is_finite() && ((assign76310_e115849) as f64).fract() == 0.0 { if assign76310_e115849 == 0.0 { 0.0 } else { (assign76310_e115849 * ((locals.var_dnm).powf(assign76310_e115849 - 1.0) * locals.var_dnm_dn13)) } } else { (assign76310_e115850 * (assign76310_e115849 * (locals.var_dnm_dn13 / locals.var_dnm))) },)
            }
        };
        (assign76310_e115851, assign76310_e115851_d_n0, assign76310_e115851_d_n2, assign76310_e115851_d_n4, assign76310_e115851_d_n5, assign76310_e115851_d_n6, assign76310_e115851_d_n7, assign76310_e115851_d_n8, assign76310_e115851_d_n9, assign76310_e115851_d_n10, assign76310_e115851_d_n13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign76310_e115853;
        locals.var_dnm_dn0 = assign76310_e115853_d_n0;
        locals.var_dnm_dn2 = assign76310_e115853_d_n2;
        locals.var_dnm_dn4 = assign76310_e115853_d_n4;
        locals.var_dnm_dn5 = assign76310_e115853_d_n5;
        locals.var_dnm_dn6 = assign76310_e115853_d_n6;
        locals.var_dnm_dn7 = assign76310_e115853_d_n7;
        locals.var_dnm_dn8 = assign76310_e115853_d_n8;
        locals.var_dnm_dn9 = assign76310_e115853_d_n9;
        locals.var_dnm_dn10 = assign76310_e115853_d_n10;
        locals.var_dnm_dn13 = assign76310_e115853_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign76320_e115863, assign76320_e115863_d_n0, assign76320_e115863_d_n2, assign76320_e115863_d_n4, assign76320_e115863_d_n5, assign76320_e115863_d_n6, assign76320_e115863_d_n7, assign76320_e115863_d_n8, assign76320_e115863_d_n9, assign76320_e115863_d_n10, assign76320_e115863_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1780 != 0.0)) {
        let assign76320_e115861: f64 = (1.0 / locals.var_dnm);
        (assign76320_e115861, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn13 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign76320_e115863;
        locals.var_dnm_dn0 = assign76320_e115863_d_n0;
        locals.var_dnm_dn2 = assign76320_e115863_d_n2;
        locals.var_dnm_dn4 = assign76320_e115863_d_n4;
        locals.var_dnm_dn5 = assign76320_e115863_d_n5;
        locals.var_dnm_dn6 = assign76320_e115863_d_n6;
        locals.var_dnm_dn7 = assign76320_e115863_d_n7;
        locals.var_dnm_dn8 = assign76320_e115863_d_n8;
        locals.var_dnm_dn9 = assign76320_e115863_d_n9;
        locals.var_dnm_dn10 = assign76320_e115863_d_n10;
        locals.var_dnm_dn13 = assign76320_e115863_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign76330_e115875, assign76330_e115875_d_n0, assign76330_e115875_d_n2, assign76330_e115875_d_n4, assign76330_e115875_d_n5, assign76330_e115875_d_n6, assign76330_e115875_d_n7, assign76330_e115875_d_n8, assign76330_e115875_d_n9, assign76330_e115875_d_n10, assign76330_e115875_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1780 != 0.0)) {
        let assign76330_e115871: f64 = (locals.var_tmf1 * locals.var_t1);
        let assign76330_e115873: f64 = (assign76330_e115871 * locals.var_dnm);
        (assign76330_e115873, ((((locals.var_tmf1_dn0 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn0)) * locals.var_dnm) + (assign76330_e115871 * locals.var_dnm_dn0)), ((((locals.var_tmf1_dn2 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn2)) * locals.var_dnm) + (assign76330_e115871 * locals.var_dnm_dn2)), ((((locals.var_tmf1_dn4 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn4)) * locals.var_dnm) + (assign76330_e115871 * locals.var_dnm_dn4)), ((((locals.var_tmf1_dn5 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn5)) * locals.var_dnm) + (assign76330_e115871 * locals.var_dnm_dn5)), ((((locals.var_tmf1_dn6 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn6)) * locals.var_dnm) + (assign76330_e115871 * locals.var_dnm_dn6)), ((((locals.var_tmf1_dn7 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn7)) * locals.var_dnm) + (assign76330_e115871 * locals.var_dnm_dn7)), ((((locals.var_tmf1_dn8 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn8)) * locals.var_dnm) + (assign76330_e115871 * locals.var_dnm_dn8)), ((((locals.var_tmf1_dn9 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn9)) * locals.var_dnm) + (assign76330_e115871 * locals.var_dnm_dn9)), ((((locals.var_tmf1_dn10 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn10)) * locals.var_dnm) + (assign76330_e115871 * locals.var_dnm_dn10)), ((((locals.var_tmf1_dn13 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn13)) * locals.var_dnm) + (assign76330_e115871 * locals.var_dnm_dn13)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn13,)
    }
};
        locals.var_tmf0 = assign76330_e115875;
        locals.var_tmf0_dn0 = assign76330_e115875_d_n0;
        locals.var_tmf0_dn2 = assign76330_e115875_d_n2;
        locals.var_tmf0_dn4 = assign76330_e115875_d_n4;
        locals.var_tmf0_dn5 = assign76330_e115875_d_n5;
        locals.var_tmf0_dn6 = assign76330_e115875_d_n6;
        locals.var_tmf0_dn7 = assign76330_e115875_d_n7;
        locals.var_tmf0_dn8 = assign76330_e115875_d_n8;
        locals.var_tmf0_dn9 = assign76330_e115875_d_n9;
        locals.var_tmf0_dn10 = assign76330_e115875_d_n10;
        locals.var_tmf0_dn13 = assign76330_e115875_d_n13;
        locals.var_tmf0_rv = 0.0;

        let (assign76340_e115889, assign76340_e115889_d_n0, assign76340_e115889_d_n2, assign76340_e115889_d_n4, assign76340_e115889_d_n5, assign76340_e115889_d_n6, assign76340_e115889_d_n7, assign76340_e115889_d_n8, assign76340_e115889_d_n9, assign76340_e115889_d_n10, assign76340_e115889_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1780 != 0.0)) {
        let assign76340_e115883: f64 = (locals.var_t1 * locals.var_xmp);
        let assign76340_e115885: f64 = (assign76340_e115883 * locals.var_dnm);
        let assign76340_e115887: f64 = (assign76340_e115885 / locals.var_arg);
        (assign76340_e115887, (((((((locals.var_t1_dn0 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn0)) * locals.var_dnm) + (assign76340_e115883 * locals.var_dnm_dn0)) * locals.var_arg) - (assign76340_e115885 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn2 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn2)) * locals.var_dnm) + (assign76340_e115883 * locals.var_dnm_dn2)) * locals.var_arg) - (assign76340_e115885 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn4 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn4)) * locals.var_dnm) + (assign76340_e115883 * locals.var_dnm_dn4)) * locals.var_arg) - (assign76340_e115885 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn5 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn5)) * locals.var_dnm) + (assign76340_e115883 * locals.var_dnm_dn5)) * locals.var_arg) - (assign76340_e115885 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn6 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn6)) * locals.var_dnm) + (assign76340_e115883 * locals.var_dnm_dn6)) * locals.var_arg) - (assign76340_e115885 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn7 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn7)) * locals.var_dnm) + (assign76340_e115883 * locals.var_dnm_dn7)) * locals.var_arg) - (assign76340_e115885 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn8 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn8)) * locals.var_dnm) + (assign76340_e115883 * locals.var_dnm_dn8)) * locals.var_arg) - (assign76340_e115885 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn9 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn9)) * locals.var_dnm) + (assign76340_e115883 * locals.var_dnm_dn9)) * locals.var_arg) - (assign76340_e115885 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn10 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn10)) * locals.var_dnm) + (assign76340_e115883 * locals.var_dnm_dn10)) * locals.var_arg) - (assign76340_e115885 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn13 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn13)) * locals.var_dnm) + (assign76340_e115883 * locals.var_dnm_dn13)) * locals.var_arg) - (assign76340_e115885 * locals.var_arg_dn13)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign76340_e115889;
        locals.var_t0_dn0 = assign76340_e115889_d_n0;
        locals.var_t0_dn2 = assign76340_e115889_d_n2;
        locals.var_t0_dn4 = assign76340_e115889_d_n4;
        locals.var_t0_dn5 = assign76340_e115889_d_n5;
        locals.var_t0_dn6 = assign76340_e115889_d_n6;
        locals.var_t0_dn7 = assign76340_e115889_d_n7;
        locals.var_t0_dn8 = assign76340_e115889_d_n8;
        locals.var_t0_dn9 = assign76340_e115889_d_n9;
        locals.var_t0_dn10 = assign76340_e115889_d_n10;
        locals.var_t0_dn13 = assign76340_e115889_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign76350_e115901, assign76350_e115901_d_n0, assign76350_e115901_d_n2, assign76350_e115901_d_n4, assign76350_e115901_d_n5, assign76350_e115901_d_n6, assign76350_e115901_d_n7, assign76350_e115901_d_n8, assign76350_e115901_d_n9, assign76350_e115901_d_n10, assign76350_e115901_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1780 != 0.0)) {
        let assign76350_e115897: f64 = (-locals.var_t1);
        let assign76350_e115899: f64 = (assign76350_e115897 + locals.var_tmf0);
        (assign76350_e115899, ((-locals.var_t1_dn0) + locals.var_tmf0_dn0), ((-locals.var_t1_dn2) + locals.var_tmf0_dn2), ((-locals.var_t1_dn4) + locals.var_tmf0_dn4), ((-locals.var_t1_dn5) + locals.var_tmf0_dn5), ((-locals.var_t1_dn6) + locals.var_tmf0_dn6), ((-locals.var_t1_dn7) + locals.var_tmf0_dn7), ((-locals.var_t1_dn8) + locals.var_tmf0_dn8), ((-locals.var_t1_dn9) + locals.var_tmf0_dn9), ((-locals.var_t1_dn10) + locals.var_tmf0_dn10), ((-locals.var_t1_dn13) + locals.var_tmf0_dn13),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign76350_e115901;
        locals.var_t1_dn0 = assign76350_e115901_d_n0;
        locals.var_t1_dn2 = assign76350_e115901_d_n2;
        locals.var_t1_dn4 = assign76350_e115901_d_n4;
        locals.var_t1_dn5 = assign76350_e115901_d_n5;
        locals.var_t1_dn6 = assign76350_e115901_d_n6;
        locals.var_t1_dn7 = assign76350_e115901_d_n7;
        locals.var_t1_dn8 = assign76350_e115901_d_n8;
        locals.var_t1_dn9 = assign76350_e115901_d_n9;
        locals.var_t1_dn10 = assign76350_e115901_d_n10;
        locals.var_t1_dn13 = assign76350_e115901_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign76360_e115909, assign76360_e115909_d_n0, assign76360_e115909_d_n2, assign76360_e115909_d_n4, assign76360_e115909_d_n5, assign76360_e115909_d_n6, assign76360_e115909_d_n7, assign76360_e115909_d_n8, assign76360_e115909_d_n9, assign76360_e115909_d_n10, assign76360_e115909_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1780 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign76360_e115909;
        locals.var_t0_dn0 = assign76360_e115909_d_n0;
        locals.var_t0_dn2 = assign76360_e115909_d_n2;
        locals.var_t0_dn4 = assign76360_e115909_d_n4;
        locals.var_t0_dn5 = assign76360_e115909_d_n5;
        locals.var_t0_dn6 = assign76360_e115909_d_n6;
        locals.var_t0_dn7 = assign76360_e115909_d_n7;
        locals.var_t0_dn8 = assign76360_e115909_d_n8;
        locals.var_t0_dn9 = assign76360_e115909_d_n9;
        locals.var_t0_dn10 = assign76360_e115909_d_n10;
        locals.var_t0_dn13 = assign76360_e115909_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign76370_e115920, assign76370_e115920_d_n0, assign76370_e115920_d_n2, assign76370_e115920_d_n4, assign76370_e115920_d_n5, assign76370_e115920_d_n6, assign76370_e115920_d_n7, assign76370_e115920_d_n8, assign76370_e115920_d_n9, assign76370_e115920_d_n10, assign76370_e115920_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1780 == 0.0)) {
        let assign76370_e115918: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        (assign76370_e115918, locals.var_vxbgmtcl_dn0, (locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2), locals.var_vxbgmtcl_dn4, locals.var_vxbgmtcl_dn5, (locals.var_vgpld_dn6 + locals.var_vxbgmtcl_dn6), (locals.var_vgpld_dn7 + locals.var_vxbgmtcl_dn7), (locals.var_vgpld_dn8 + locals.var_vxbgmtcl_dn8), locals.var_vxbgmtcl_dn9, locals.var_vxbgmtcl_dn10, locals.var_vxbgmtcl_dn13,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign76370_e115920;
        locals.var_t1_dn0 = assign76370_e115920_d_n0;
        locals.var_t1_dn2 = assign76370_e115920_d_n2;
        locals.var_t1_dn4 = assign76370_e115920_d_n4;
        locals.var_t1_dn5 = assign76370_e115920_d_n5;
        locals.var_t1_dn6 = assign76370_e115920_d_n6;
        locals.var_t1_dn7 = assign76370_e115920_d_n7;
        locals.var_t1_dn8 = assign76370_e115920_d_n8;
        locals.var_t1_dn9 = assign76370_e115920_d_n9;
        locals.var_t1_dn10 = assign76370_e115920_d_n10;
        locals.var_t1_dn13 = assign76370_e115920_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign76380_e115929, assign76380_e115929_d_n0, assign76380_e115929_d_n2, assign76380_e115929_d_n4, assign76380_e115929_d_n5, assign76380_e115929_d_n6, assign76380_e115929_d_n7, assign76380_e115929_d_n8, assign76380_e115929_d_n9, assign76380_e115929_d_n10, assign76380_e115929_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1780 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign76380_e115929;
        locals.var_t0_dn0 = assign76380_e115929_d_n0;
        locals.var_t0_dn2 = assign76380_e115929_d_n2;
        locals.var_t0_dn4 = assign76380_e115929_d_n4;
        locals.var_t0_dn5 = assign76380_e115929_d_n5;
        locals.var_t0_dn6 = assign76380_e115929_d_n6;
        locals.var_t0_dn7 = assign76380_e115929_d_n7;
        locals.var_t0_dn8 = assign76380_e115929_d_n8;
        locals.var_t0_dn9 = assign76380_e115929_d_n9;
        locals.var_t0_dn10 = assign76380_e115929_d_n10;
        locals.var_t0_dn13 = assign76380_e115929_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign76390_e115937, assign76390_e115937_d_n0, assign76390_e115937_d_n2, assign76390_e115937_d_n4, assign76390_e115937_d_n5, assign76390_e115937_d_n6, assign76390_e115937_d_n7, assign76390_e115937_d_n8, assign76390_e115937_d_n9, assign76390_e115937_d_n10, assign76390_e115937_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) {
        let assign76390_e115935: f64 = (locals.var_t1 - locals.var_vgpld);
        (assign76390_e115935, locals.var_t1_dn0, (locals.var_t1_dn2 - locals.var_vgpld_dn2), locals.var_t1_dn4, locals.var_t1_dn5, (locals.var_t1_dn6 - locals.var_vgpld_dn6), (locals.var_t1_dn7 - locals.var_vgpld_dn7), (locals.var_t1_dn8 - locals.var_vgpld_dn8), locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    } else {
        (locals.var_vxbgmtcl, locals.var_vxbgmtcl_dn0, locals.var_vxbgmtcl_dn2, locals.var_vxbgmtcl_dn4, locals.var_vxbgmtcl_dn5, locals.var_vxbgmtcl_dn6, locals.var_vxbgmtcl_dn7, locals.var_vxbgmtcl_dn8, locals.var_vxbgmtcl_dn9, locals.var_vxbgmtcl_dn10, locals.var_vxbgmtcl_dn13,)
    }
};
        locals.var_vxbgmtcl = assign76390_e115937;
        locals.var_vxbgmtcl_dn0 = assign76390_e115937_d_n0;
        locals.var_vxbgmtcl_dn2 = assign76390_e115937_d_n2;
        locals.var_vxbgmtcl_dn4 = assign76390_e115937_d_n4;
        locals.var_vxbgmtcl_dn5 = assign76390_e115937_d_n5;
        locals.var_vxbgmtcl_dn6 = assign76390_e115937_d_n6;
        locals.var_vxbgmtcl_dn7 = assign76390_e115937_d_n7;
        locals.var_vxbgmtcl_dn8 = assign76390_e115937_d_n8;
        locals.var_vxbgmtcl_dn9 = assign76390_e115937_d_n9;
        locals.var_vxbgmtcl_dn10 = assign76390_e115937_d_n10;
        locals.var_vxbgmtcl_dn13 = assign76390_e115937_d_n13;
        locals.var_vxbgmtcl_rv = 0.0;

        let (assign76400_e115948, assign76400_e115948_d_n0, assign76400_e115948_d_n2, assign76400_e115948_d_n4, assign76400_e115948_d_n5, assign76400_e115948_d_n6, assign76400_e115948_d_n7, assign76400_e115948_d_n8, assign76400_e115948_d_n9, assign76400_e115948_d_n10, assign76400_e115948_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) {
        let assign76400_e115942: f64 = (-locals.var_vxbgmtcl);
        let assign76400_e115945: f64 = (10.0 * 2.220446049250313e-16);
        let assign76400_e115946: f64 = (assign76400_e115942 + assign76400_e115945);
        (assign76400_e115946, (-locals.var_vxbgmtcl_dn0), (-locals.var_vxbgmtcl_dn2), (-locals.var_vxbgmtcl_dn4), (-locals.var_vxbgmtcl_dn5), (-locals.var_vxbgmtcl_dn6), (-locals.var_vxbgmtcl_dn7), (-locals.var_vxbgmtcl_dn8), (-locals.var_vxbgmtcl_dn9), (-locals.var_vxbgmtcl_dn10), (-locals.var_vxbgmtcl_dn13),)
    } else {
        (locals.var_vgb_fb_ld, locals.var_vgb_fb_ld_dn0, locals.var_vgb_fb_ld_dn2, locals.var_vgb_fb_ld_dn4, locals.var_vgb_fb_ld_dn5, locals.var_vgb_fb_ld_dn6, locals.var_vgb_fb_ld_dn7, locals.var_vgb_fb_ld_dn8, locals.var_vgb_fb_ld_dn9, locals.var_vgb_fb_ld_dn10, locals.var_vgb_fb_ld_dn13,)
    }
};
        locals.var_vgb_fb_ld = assign76400_e115948;
        locals.var_vgb_fb_ld_dn0 = assign76400_e115948_d_n0;
        locals.var_vgb_fb_ld_dn2 = assign76400_e115948_d_n2;
        locals.var_vgb_fb_ld_dn4 = assign76400_e115948_d_n4;
        locals.var_vgb_fb_ld_dn5 = assign76400_e115948_d_n5;
        locals.var_vgb_fb_ld_dn6 = assign76400_e115948_d_n6;
        locals.var_vgb_fb_ld_dn7 = assign76400_e115948_d_n7;
        locals.var_vgb_fb_ld_dn8 = assign76400_e115948_d_n8;
        locals.var_vgb_fb_ld_dn9 = assign76400_e115948_d_n9;
        locals.var_vgb_fb_ld_dn10 = assign76400_e115948_d_n10;
        locals.var_vgb_fb_ld_dn13 = assign76400_e115948_d_n13;
        locals.var_vgb_fb_ld_rv = 0.0;

        let assign76410_e115951: f64 = if locals.var_vgpld < locals.var_vgb_fb_ld { 1.0 } else { 0.0 };
        locals.var_guard1786 = assign76410_e115951;
        locals.var_guard1786_rv = 0.0;

        let (assign76430_e115972, assign76430_e115972_d_n0, assign76430_e115972_d_n2, assign76430_e115972_d_n4, assign76430_e115972_d_n5, assign76430_e115972_d_n6, assign76430_e115972_d_n7, assign76430_e115972_d_n8, assign76430_e115972_d_n9, assign76430_e115972_d_n10, assign76430_e115972_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 != 0.0)) {
        let assign76430_e115964: f64 = (2.0 * locals.var_beta_inv);
        let assign76430_e115966: f64 = (-locals.var_vgs_min);
        let assign76430_e115968: f64 = (assign76430_e115966 / locals.var_fac1);
        let assign76430_e115969: f64 = (assign76430_e115968).ln();
        let assign76430_e115970: f64 = (assign76430_e115964 * assign76430_e115969);
        (assign76430_e115970, (((2.0 * locals.var_beta_inv_dn0) * assign76430_e115969) + (assign76430_e115964 * ((-((assign76430_e115966 * locals.var_fac1_dn0) / (locals.var_fac1 * locals.var_fac1))) / assign76430_e115968))), (((2.0 * locals.var_beta_inv_dn2) * assign76430_e115969) + (assign76430_e115964 * ((-((assign76430_e115966 * locals.var_fac1_dn2) / (locals.var_fac1 * locals.var_fac1))) / assign76430_e115968))), (((2.0 * locals.var_beta_inv_dn4) * assign76430_e115969) + (assign76430_e115964 * ((-((assign76430_e115966 * locals.var_fac1_dn4) / (locals.var_fac1 * locals.var_fac1))) / assign76430_e115968))), (((2.0 * locals.var_beta_inv_dn5) * assign76430_e115969) + (assign76430_e115964 * ((-((assign76430_e115966 * locals.var_fac1_dn5) / (locals.var_fac1 * locals.var_fac1))) / assign76430_e115968))), (((2.0 * locals.var_beta_inv_dn6) * assign76430_e115969) + (assign76430_e115964 * ((-((assign76430_e115966 * locals.var_fac1_dn6) / (locals.var_fac1 * locals.var_fac1))) / assign76430_e115968))), (((2.0 * locals.var_beta_inv_dn7) * assign76430_e115969) + (assign76430_e115964 * ((-((assign76430_e115966 * locals.var_fac1_dn7) / (locals.var_fac1 * locals.var_fac1))) / assign76430_e115968))), (((2.0 * locals.var_beta_inv_dn8) * assign76430_e115969) + (assign76430_e115964 * ((-((assign76430_e115966 * locals.var_fac1_dn8) / (locals.var_fac1 * locals.var_fac1))) / assign76430_e115968))), (((2.0 * locals.var_beta_inv_dn9) * assign76430_e115969) + (assign76430_e115964 * ((-((assign76430_e115966 * locals.var_fac1_dn9) / (locals.var_fac1 * locals.var_fac1))) / assign76430_e115968))), (((2.0 * locals.var_beta_inv_dn10) * assign76430_e115969) + (assign76430_e115964 * ((-((assign76430_e115966 * locals.var_fac1_dn10) / (locals.var_fac1 * locals.var_fac1))) / assign76430_e115968))), (((2.0 * locals.var_beta_inv_dn13) * assign76430_e115969) + (assign76430_e115964 * ((-((assign76430_e115966 * locals.var_fac1_dn13) / (locals.var_fac1 * locals.var_fac1))) / assign76430_e115968))),)
    } else {
        (locals.var_ps0_min, locals.var_ps0_min_dn0, locals.var_ps0_min_dn2, locals.var_ps0_min_dn4, locals.var_ps0_min_dn5, locals.var_ps0_min_dn6, locals.var_ps0_min_dn7, locals.var_ps0_min_dn8, locals.var_ps0_min_dn9, locals.var_ps0_min_dn10, locals.var_ps0_min_dn13,)
    }
};
        locals.var_ps0_min = assign76430_e115972;
        locals.var_ps0_min_dn0 = assign76430_e115972_d_n0;
        locals.var_ps0_min_dn2 = assign76430_e115972_d_n2;
        locals.var_ps0_min_dn4 = assign76430_e115972_d_n4;
        locals.var_ps0_min_dn5 = assign76430_e115972_d_n5;
        locals.var_ps0_min_dn6 = assign76430_e115972_d_n6;
        locals.var_ps0_min_dn7 = assign76430_e115972_d_n7;
        locals.var_ps0_min_dn8 = assign76430_e115972_d_n8;
        locals.var_ps0_min_dn9 = assign76430_e115972_d_n9;
        locals.var_ps0_min_dn10 = assign76430_e115972_d_n10;
        locals.var_ps0_min_dn13 = assign76430_e115972_d_n13;
        locals.var_ps0_min_rv = 0.0;

        let (assign76440_e115982, assign76440_e115982_d_n0, assign76440_e115982_d_n2, assign76440_e115982_d_n4, assign76440_e115982_d_n5, assign76440_e115982_d_n6, assign76440_e115982_d_n7, assign76440_e115982_d_n8, assign76440_e115982_d_n9, assign76440_e115982_d_n10, assign76440_e115982_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 != 0.0)) {
        let assign76440_e115979: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign76440_e115980: f64 = (locals.var_beta * assign76440_e115979);
        (assign76440_e115980, ((locals.var_beta_dn0 * assign76440_e115979) + (locals.var_beta * locals.var_vxbgmtcl_dn0)), ((locals.var_beta_dn2 * assign76440_e115979) + (locals.var_beta * (locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2))), ((locals.var_beta_dn4 * assign76440_e115979) + (locals.var_beta * locals.var_vxbgmtcl_dn4)), ((locals.var_beta_dn5 * assign76440_e115979) + (locals.var_beta * locals.var_vxbgmtcl_dn5)), ((locals.var_beta_dn6 * assign76440_e115979) + (locals.var_beta * (locals.var_vgpld_dn6 + locals.var_vxbgmtcl_dn6))), ((locals.var_beta_dn7 * assign76440_e115979) + (locals.var_beta * (locals.var_vgpld_dn7 + locals.var_vxbgmtcl_dn7))), ((locals.var_beta_dn8 * assign76440_e115979) + (locals.var_beta * (locals.var_vgpld_dn8 + locals.var_vxbgmtcl_dn8))), ((locals.var_beta_dn9 * assign76440_e115979) + (locals.var_beta * locals.var_vxbgmtcl_dn9)), ((locals.var_beta_dn10 * assign76440_e115979) + (locals.var_beta * locals.var_vxbgmtcl_dn10)), ((locals.var_beta_dn13 * assign76440_e115979) + (locals.var_beta * locals.var_vxbgmtcl_dn13)),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn13,)
    }
};
        locals.var_tx = assign76440_e115982;
        locals.var_tx_dn0 = assign76440_e115982_d_n0;
        locals.var_tx_dn2 = assign76440_e115982_d_n2;
        locals.var_tx_dn4 = assign76440_e115982_d_n4;
        locals.var_tx_dn5 = assign76440_e115982_d_n5;
        locals.var_tx_dn6 = assign76440_e115982_d_n6;
        locals.var_tx_dn7 = assign76440_e115982_d_n7;
        locals.var_tx_dn8 = assign76440_e115982_d_n8;
        locals.var_tx_dn9 = assign76440_e115982_d_n9;
        locals.var_tx_dn10 = assign76440_e115982_d_n10;
        locals.var_tx_dn13 = assign76440_e115982_d_n13;
        locals.var_tx_rv = 0.0;

        let (assign76450_e115992, assign76450_e115992_d_n0, assign76450_e115992_d_n2, assign76450_e115992_d_n4, assign76450_e115992_d_n5, assign76450_e115992_d_n6, assign76450_e115992_d_n7, assign76450_e115992_d_n8, assign76450_e115992_d_n9, assign76450_e115992_d_n10, assign76450_e115992_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 != 0.0)) {
        let assign76450_e115989: f64 = (locals.var_beta * locals.var_cnst0over_func);
        let assign76450_e115990: f64 = (1.0 / assign76450_e115989);
        (assign76450_e115990, (-(((locals.var_beta_dn0 * locals.var_cnst0over_func) + (locals.var_beta * locals.var_cnst0over_func_dn0)) / (assign76450_e115989 * assign76450_e115989))), (-(((locals.var_beta_dn2 * locals.var_cnst0over_func) + (locals.var_beta * locals.var_cnst0over_func_dn2)) / (assign76450_e115989 * assign76450_e115989))), (-(((locals.var_beta_dn4 * locals.var_cnst0over_func) + (locals.var_beta * locals.var_cnst0over_func_dn4)) / (assign76450_e115989 * assign76450_e115989))), (-(((locals.var_beta_dn5 * locals.var_cnst0over_func) + (locals.var_beta * locals.var_cnst0over_func_dn5)) / (assign76450_e115989 * assign76450_e115989))), (-(((locals.var_beta_dn6 * locals.var_cnst0over_func) + (locals.var_beta * locals.var_cnst0over_func_dn6)) / (assign76450_e115989 * assign76450_e115989))), (-(((locals.var_beta_dn7 * locals.var_cnst0over_func) + (locals.var_beta * locals.var_cnst0over_func_dn7)) / (assign76450_e115989 * assign76450_e115989))), (-(((locals.var_beta_dn8 * locals.var_cnst0over_func) + (locals.var_beta * locals.var_cnst0over_func_dn8)) / (assign76450_e115989 * assign76450_e115989))), (-(((locals.var_beta_dn9 * locals.var_cnst0over_func) + (locals.var_beta * locals.var_cnst0over_func_dn9)) / (assign76450_e115989 * assign76450_e115989))), (-(((locals.var_beta_dn10 * locals.var_cnst0over_func) + (locals.var_beta * locals.var_cnst0over_func_dn10)) / (assign76450_e115989 * assign76450_e115989))), (-(((locals.var_beta_dn13 * locals.var_cnst0over_func) + (locals.var_beta * locals.var_cnst0over_func_dn13)) / (assign76450_e115989 * assign76450_e115989))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign76450_e115992;
        locals.var_t1_dn0 = assign76450_e115992_d_n0;
        locals.var_t1_dn2 = assign76450_e115992_d_n2;
        locals.var_t1_dn4 = assign76450_e115992_d_n4;
        locals.var_t1_dn5 = assign76450_e115992_d_n5;
        locals.var_t1_dn6 = assign76450_e115992_d_n6;
        locals.var_t1_dn7 = assign76450_e115992_d_n7;
        locals.var_t1_dn8 = assign76450_e115992_d_n8;
        locals.var_t1_dn9 = assign76450_e115992_d_n9;
        locals.var_t1_dn10 = assign76450_e115992_d_n10;
        locals.var_t1_dn13 = assign76450_e115992_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign76460_e116000, assign76460_e116000_d_n0, assign76460_e116000_d_n2, assign76460_e116000_d_n4, assign76460_e116000_d_n5, assign76460_e116000_d_n6, assign76460_e116000_d_n7, assign76460_e116000_d_n8, assign76460_e116000_d_n9, assign76460_e116000_d_n10, assign76460_e116000_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 != 0.0)) {
        let assign76460_e115998: f64 = (locals.var_t1 * locals.var_cox0_func);
        (assign76460_e115998, (locals.var_t1_dn0 * locals.var_cox0_func), (locals.var_t1_dn2 * locals.var_cox0_func), (locals.var_t1_dn4 * locals.var_cox0_func), (locals.var_t1_dn5 * locals.var_cox0_func), (locals.var_t1_dn6 * locals.var_cox0_func), (locals.var_t1_dn7 * locals.var_cox0_func), (locals.var_t1_dn8 * locals.var_cox0_func), (locals.var_t1_dn9 * locals.var_cox0_func), (locals.var_t1_dn10 * locals.var_cox0_func), (locals.var_t1_dn13 * locals.var_cox0_func),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn4, locals.var_ty_dn5, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn8, locals.var_ty_dn9, locals.var_ty_dn10, locals.var_ty_dn13,)
    }
};
        locals.var_ty = assign76460_e116000;
        locals.var_ty_dn0 = assign76460_e116000_d_n0;
        locals.var_ty_dn2 = assign76460_e116000_d_n2;
        locals.var_ty_dn4 = assign76460_e116000_d_n4;
        locals.var_ty_dn5 = assign76460_e116000_d_n5;
        locals.var_ty_dn6 = assign76460_e116000_d_n6;
        locals.var_ty_dn7 = assign76460_e116000_d_n7;
        locals.var_ty_dn8 = assign76460_e116000_d_n8;
        locals.var_ty_dn9 = assign76460_e116000_d_n9;
        locals.var_ty_dn10 = assign76460_e116000_d_n10;
        locals.var_ty_dn13 = assign76460_e116000_d_n13;
        locals.var_ty_rv = 0.0;

        let (assign76470_e116012, assign76470_e116012_d_n0, assign76470_e116012_d_n2, assign76470_e116012_d_n4, assign76470_e116012_d_n5, assign76470_e116012_d_n6, assign76470_e116012_d_n7, assign76470_e116012_d_n8, assign76470_e116012_d_n9, assign76470_e116012_d_n10, assign76470_e116012_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 != 0.0)) {
        let assign76470_e116007: f64 = (3.0 * 1.414213562373095);
        let assign76470_e116009: f64 = (assign76470_e116007 * locals.var_ty);
        let assign76470_e116010: f64 = (2.0 + assign76470_e116009);
        (assign76470_e116010, (assign76470_e116007 * locals.var_ty_dn0), (assign76470_e116007 * locals.var_ty_dn2), (assign76470_e116007 * locals.var_ty_dn4), (assign76470_e116007 * locals.var_ty_dn5), (assign76470_e116007 * locals.var_ty_dn6), (assign76470_e116007 * locals.var_ty_dn7), (assign76470_e116007 * locals.var_ty_dn8), (assign76470_e116007 * locals.var_ty_dn9), (assign76470_e116007 * locals.var_ty_dn10), (assign76470_e116007 * locals.var_ty_dn13),)
    } else {
        (locals.var_ac41, locals.var_ac41_dn0, locals.var_ac41_dn2, locals.var_ac41_dn4, locals.var_ac41_dn5, locals.var_ac41_dn6, locals.var_ac41_dn7, locals.var_ac41_dn8, locals.var_ac41_dn9, locals.var_ac41_dn10, locals.var_ac41_dn13,)
    }
};
        locals.var_ac41 = assign76470_e116012;
        locals.var_ac41_dn0 = assign76470_e116012_d_n0;
        locals.var_ac41_dn2 = assign76470_e116012_d_n2;
        locals.var_ac41_dn4 = assign76470_e116012_d_n4;
        locals.var_ac41_dn5 = assign76470_e116012_d_n5;
        locals.var_ac41_dn6 = assign76470_e116012_d_n6;
        locals.var_ac41_dn7 = assign76470_e116012_d_n7;
        locals.var_ac41_dn8 = assign76470_e116012_d_n8;
        locals.var_ac41_dn9 = assign76470_e116012_d_n9;
        locals.var_ac41_dn10 = assign76470_e116012_d_n10;
        locals.var_ac41_dn13 = assign76470_e116012_d_n13;
        locals.var_ac41_rv = 0.0;

        let (assign76480_e116024, assign76480_e116024_d_n0, assign76480_e116024_d_n2, assign76480_e116024_d_n4, assign76480_e116024_d_n5, assign76480_e116024_d_n6, assign76480_e116024_d_n7, assign76480_e116024_d_n8, assign76480_e116024_d_n9, assign76480_e116024_d_n10, assign76480_e116024_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 != 0.0)) {
        let assign76480_e116018: f64 = (8.0 * locals.var_ac41);
        let assign76480_e116020: f64 = (assign76480_e116018 * locals.var_ac41);
        let assign76480_e116022: f64 = (assign76480_e116020 * locals.var_ac41);
        (assign76480_e116022, (((((8.0 * locals.var_ac41_dn0) * locals.var_ac41) + (assign76480_e116018 * locals.var_ac41_dn0)) * locals.var_ac41) + (assign76480_e116020 * locals.var_ac41_dn0)), (((((8.0 * locals.var_ac41_dn2) * locals.var_ac41) + (assign76480_e116018 * locals.var_ac41_dn2)) * locals.var_ac41) + (assign76480_e116020 * locals.var_ac41_dn2)), (((((8.0 * locals.var_ac41_dn4) * locals.var_ac41) + (assign76480_e116018 * locals.var_ac41_dn4)) * locals.var_ac41) + (assign76480_e116020 * locals.var_ac41_dn4)), (((((8.0 * locals.var_ac41_dn5) * locals.var_ac41) + (assign76480_e116018 * locals.var_ac41_dn5)) * locals.var_ac41) + (assign76480_e116020 * locals.var_ac41_dn5)), (((((8.0 * locals.var_ac41_dn6) * locals.var_ac41) + (assign76480_e116018 * locals.var_ac41_dn6)) * locals.var_ac41) + (assign76480_e116020 * locals.var_ac41_dn6)), (((((8.0 * locals.var_ac41_dn7) * locals.var_ac41) + (assign76480_e116018 * locals.var_ac41_dn7)) * locals.var_ac41) + (assign76480_e116020 * locals.var_ac41_dn7)), (((((8.0 * locals.var_ac41_dn8) * locals.var_ac41) + (assign76480_e116018 * locals.var_ac41_dn8)) * locals.var_ac41) + (assign76480_e116020 * locals.var_ac41_dn8)), (((((8.0 * locals.var_ac41_dn9) * locals.var_ac41) + (assign76480_e116018 * locals.var_ac41_dn9)) * locals.var_ac41) + (assign76480_e116020 * locals.var_ac41_dn9)), (((((8.0 * locals.var_ac41_dn10) * locals.var_ac41) + (assign76480_e116018 * locals.var_ac41_dn10)) * locals.var_ac41) + (assign76480_e116020 * locals.var_ac41_dn10)), (((((8.0 * locals.var_ac41_dn13) * locals.var_ac41) + (assign76480_e116018 * locals.var_ac41_dn13)) * locals.var_ac41) + (assign76480_e116020 * locals.var_ac41_dn13)),)
    } else {
        (locals.var_ac4, locals.var_ac4_dn0, locals.var_ac4_dn2, locals.var_ac4_dn4, locals.var_ac4_dn5, locals.var_ac4_dn6, locals.var_ac4_dn7, locals.var_ac4_dn8, locals.var_ac4_dn9, locals.var_ac4_dn10, locals.var_ac4_dn13,)
    }
};
        locals.var_ac4 = assign76480_e116024;
        locals.var_ac4_dn0 = assign76480_e116024_d_n0;
        locals.var_ac4_dn2 = assign76480_e116024_d_n2;
        locals.var_ac4_dn4 = assign76480_e116024_d_n4;
        locals.var_ac4_dn5 = assign76480_e116024_d_n5;
        locals.var_ac4_dn6 = assign76480_e116024_d_n6;
        locals.var_ac4_dn7 = assign76480_e116024_d_n7;
        locals.var_ac4_dn8 = assign76480_e116024_d_n8;
        locals.var_ac4_dn9 = assign76480_e116024_d_n9;
        locals.var_ac4_dn10 = assign76480_e116024_d_n10;
        locals.var_ac4_dn13 = assign76480_e116024_d_n13;
        locals.var_ac4_rv = 0.0;

        let (assign76490_e116040, assign76490_e116040_d_n0, assign76490_e116040_d_n2, assign76490_e116040_d_n4, assign76490_e116040_d_n5, assign76490_e116040_d_n6, assign76490_e116040_d_n7, assign76490_e116040_d_n8, assign76490_e116040_d_n9, assign76490_e116040_d_n10, assign76490_e116040_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 != 0.0)) {
        let assign76490_e116030: f64 = (7.0 * 1.414213562373095);
        let assign76490_e116033: f64 = (9.0 * locals.var_ty);
        let assign76490_e116036: f64 = (locals.var_tx - 2.0);
        let assign76490_e116037: f64 = (assign76490_e116033 * assign76490_e116036);
        let assign76490_e116038: f64 = (assign76490_e116030 - assign76490_e116037);
        (assign76490_e116038, (-(((9.0 * locals.var_ty_dn0) * assign76490_e116036) + (assign76490_e116033 * locals.var_tx_dn0))), (-(((9.0 * locals.var_ty_dn2) * assign76490_e116036) + (assign76490_e116033 * locals.var_tx_dn2))), (-(((9.0 * locals.var_ty_dn4) * assign76490_e116036) + (assign76490_e116033 * locals.var_tx_dn4))), (-(((9.0 * locals.var_ty_dn5) * assign76490_e116036) + (assign76490_e116033 * locals.var_tx_dn5))), (-(((9.0 * locals.var_ty_dn6) * assign76490_e116036) + (assign76490_e116033 * locals.var_tx_dn6))), (-(((9.0 * locals.var_ty_dn7) * assign76490_e116036) + (assign76490_e116033 * locals.var_tx_dn7))), (-(((9.0 * locals.var_ty_dn8) * assign76490_e116036) + (assign76490_e116033 * locals.var_tx_dn8))), (-(((9.0 * locals.var_ty_dn9) * assign76490_e116036) + (assign76490_e116033 * locals.var_tx_dn9))), (-(((9.0 * locals.var_ty_dn10) * assign76490_e116036) + (assign76490_e116033 * locals.var_tx_dn10))), (-(((9.0 * locals.var_ty_dn13) * assign76490_e116036) + (assign76490_e116033 * locals.var_tx_dn13))),)
    } else {
        (locals.var_ac31, locals.var_ac31_dn0, locals.var_ac31_dn2, locals.var_ac31_dn4, locals.var_ac31_dn5, locals.var_ac31_dn6, locals.var_ac31_dn7, locals.var_ac31_dn8, locals.var_ac31_dn9, locals.var_ac31_dn10, locals.var_ac31_dn13,)
    }
};
        locals.var_ac31 = assign76490_e116040;
        locals.var_ac31_dn0 = assign76490_e116040_d_n0;
        locals.var_ac31_dn2 = assign76490_e116040_d_n2;
        locals.var_ac31_dn4 = assign76490_e116040_d_n4;
        locals.var_ac31_dn5 = assign76490_e116040_d_n5;
        locals.var_ac31_dn6 = assign76490_e116040_d_n6;
        locals.var_ac31_dn7 = assign76490_e116040_d_n7;
        locals.var_ac31_dn8 = assign76490_e116040_d_n8;
        locals.var_ac31_dn9 = assign76490_e116040_d_n9;
        locals.var_ac31_dn10 = assign76490_e116040_d_n10;
        locals.var_ac31_dn13 = assign76490_e116040_d_n13;
        locals.var_ac31_rv = 0.0;

        let (assign76500_e116048, assign76500_e116048_d_n0, assign76500_e116048_d_n2, assign76500_e116048_d_n4, assign76500_e116048_d_n5, assign76500_e116048_d_n6, assign76500_e116048_d_n7, assign76500_e116048_d_n8, assign76500_e116048_d_n9, assign76500_e116048_d_n10, assign76500_e116048_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 != 0.0)) {
        let assign76500_e116046: f64 = (locals.var_ac31 * locals.var_ac31);
        (assign76500_e116046, ((locals.var_ac31_dn0 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn0)), ((locals.var_ac31_dn2 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn2)), ((locals.var_ac31_dn4 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn4)), ((locals.var_ac31_dn5 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn5)), ((locals.var_ac31_dn6 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn6)), ((locals.var_ac31_dn7 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn7)), ((locals.var_ac31_dn8 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn8)), ((locals.var_ac31_dn9 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn9)), ((locals.var_ac31_dn10 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn10)), ((locals.var_ac31_dn13 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn13)),)
    } else {
        (locals.var_ac3, locals.var_ac3_dn0, locals.var_ac3_dn2, locals.var_ac3_dn4, locals.var_ac3_dn5, locals.var_ac3_dn6, locals.var_ac3_dn7, locals.var_ac3_dn8, locals.var_ac3_dn9, locals.var_ac3_dn10, locals.var_ac3_dn13,)
    }
};
        locals.var_ac3 = assign76500_e116048;
        locals.var_ac3_dn0 = assign76500_e116048_d_n0;
        locals.var_ac3_dn2 = assign76500_e116048_d_n2;
        locals.var_ac3_dn4 = assign76500_e116048_d_n4;
        locals.var_ac3_dn5 = assign76500_e116048_d_n5;
        locals.var_ac3_dn6 = assign76500_e116048_d_n6;
        locals.var_ac3_dn7 = assign76500_e116048_d_n7;
        locals.var_ac3_dn8 = assign76500_e116048_d_n8;
        locals.var_ac3_dn9 = assign76500_e116048_d_n9;
        locals.var_ac3_dn10 = assign76500_e116048_d_n10;
        locals.var_ac3_dn13 = assign76500_e116048_d_n13;
        locals.var_ac3_rv = 0.0;

        let assign76510_e116052: f64 = (locals.var_ac3 * 1e-8);
        let assign76510_e116053: f64 = if locals.var_ac4 < assign76510_e116052 { 1.0 } else { 0.0 };
        locals.var_guard1787 = assign76510_e116053;
        locals.var_guard1787_rv = 0.0;

    }
}
