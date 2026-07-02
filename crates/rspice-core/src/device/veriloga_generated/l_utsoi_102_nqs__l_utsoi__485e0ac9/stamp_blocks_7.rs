#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_55(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        if ((locals.var_guard1080 != 0.0) && (locals.var_guard1192 != 0.0)) {
            let assign38380_e43416: f64 = (locals.var_q_temp2__blk815 * locals.var_q_invexpq__blk831);
            (locals.var_q_sh_term__blk833, locals.var_q_sh_term__blk833_dn4, locals.var_q_sh_term__blk833_dn6, locals.var_q_sh_term__blk833_dn7, locals.var_q_sh_term__blk833_dn8, locals.var_q_sh_term__blk833_dn9, ) = (assign38380_e43416, ((locals.var_q_temp2__blk815_dn4 * locals.var_q_invexpq__blk831) + (locals.var_q_temp2__blk815 * locals.var_q_invexpq__blk831_dn4)), ((locals.var_q_temp2__blk815_dn6 * locals.var_q_invexpq__blk831) + (locals.var_q_temp2__blk815 * locals.var_q_invexpq__blk831_dn6)), ((locals.var_q_temp2__blk815_dn7 * locals.var_q_invexpq__blk831) + (locals.var_q_temp2__blk815 * locals.var_q_invexpq__blk831_dn7)), ((locals.var_q_temp2__blk815_dn8 * locals.var_q_invexpq__blk831) + (locals.var_q_temp2__blk815 * locals.var_q_invexpq__blk831_dn8)), ((locals.var_q_temp2__blk815_dn9 * locals.var_q_invexpq__blk831) + (locals.var_q_temp2__blk815 * locals.var_q_invexpq__blk831_dn9)), );
            locals.var_q_sh_term__blk833_rv = 0.0;
        }

        if ((locals.var_guard1080 != 0.0) && (locals.var_guard1192 != 0.0)) {
            let assign38390_e43423: f64 = (locals.var_q_temp2__blk815).ln();
            let assign38390_e43425: f64 = (assign38390_e43423 - locals.var_q_rac_qsq__blk828);
            (locals.var_q_ln_term__blk834, locals.var_q_ln_term__blk834_dn4, locals.var_q_ln_term__blk834_dn6, locals.var_q_ln_term__blk834_dn7, locals.var_q_ln_term__blk834_dn8, locals.var_q_ln_term__blk834_dn9, ) = (assign38390_e43425, ((locals.var_q_temp2__blk815_dn4 / locals.var_q_temp2__blk815) - locals.var_q_rac_qsq__blk828_dn4), ((locals.var_q_temp2__blk815_dn6 / locals.var_q_temp2__blk815) - locals.var_q_rac_qsq__blk828_dn6), ((locals.var_q_temp2__blk815_dn7 / locals.var_q_temp2__blk815) - locals.var_q_rac_qsq__blk828_dn7), ((locals.var_q_temp2__blk815_dn8 / locals.var_q_temp2__blk815) - locals.var_q_rac_qsq__blk828_dn8), ((locals.var_q_temp2__blk815_dn9 / locals.var_q_temp2__blk815) - locals.var_q_rac_qsq__blk828_dn9), );
            locals.var_q_ln_term__blk834_rv = 0.0;
        }

        let assign38400_e43430: f64 = (-0.005);
        let assign38400_e43431: f64 = if locals.var_q_qsq__blk825 < assign38400_e43430 { 1.0 } else { 0.0 };
        locals.var_guard1193 = assign38400_e43431;
        locals.var_guard1193_rv = 0.0;

        if (((locals.var_guard1080 != 0.0) && (locals.var_guard1192 == 0.0)) && (locals.var_guard1193 != 0.0)) {
            let assign38410_e43440: f64 = (0.5 * locals.var_q_rac_qsq__blk828);
            let assign38410_e43441: f64 = (assign38410_e43440).sin();
            (locals.var_q_temp2__blk815, locals.var_q_temp2__blk815_dn4, locals.var_q_temp2__blk815_dn6, locals.var_q_temp2__blk815_dn7, locals.var_q_temp2__blk815_dn8, locals.var_q_temp2__blk815_dn9, ) = (assign38410_e43441, ((assign38410_e43440).cos() * (0.5 * locals.var_q_rac_qsq__blk828_dn4)), ((assign38410_e43440).cos() * (0.5 * locals.var_q_rac_qsq__blk828_dn6)), ((assign38410_e43440).cos() * (0.5 * locals.var_q_rac_qsq__blk828_dn7)), ((assign38410_e43440).cos() * (0.5 * locals.var_q_rac_qsq__blk828_dn8)), ((assign38410_e43440).cos() * (0.5 * locals.var_q_rac_qsq__blk828_dn9)), );
            locals.var_q_temp2__blk815_rv = 0.0;
        }

        if (((locals.var_guard1080 != 0.0) && (locals.var_guard1192 == 0.0)) && (locals.var_guard1193 != 0.0)) {
            let assign38420_e43451: f64 = (-locals.var_q_qsq__blk825);
            let assign38420_e43454: f64 = (locals.var_q_temp2__blk815 * locals.var_q_temp2__blk815);
            let assign38420_e43455: f64 = (assign38420_e43451 / assign38420_e43454);
            (locals.var_q_sh_term__blk833, locals.var_q_sh_term__blk833_dn4, locals.var_q_sh_term__blk833_dn6, locals.var_q_sh_term__blk833_dn7, locals.var_q_sh_term__blk833_dn8, locals.var_q_sh_term__blk833_dn9, ) = (assign38420_e43455, ((((-locals.var_q_qsq__blk825_dn4) * assign38420_e43454) - (assign38420_e43451 * ((locals.var_q_temp2__blk815_dn4 * locals.var_q_temp2__blk815) + (locals.var_q_temp2__blk815 * locals.var_q_temp2__blk815_dn4)))) / (assign38420_e43454 * assign38420_e43454)), ((((-locals.var_q_qsq__blk825_dn6) * assign38420_e43454) - (assign38420_e43451 * ((locals.var_q_temp2__blk815_dn6 * locals.var_q_temp2__blk815) + (locals.var_q_temp2__blk815 * locals.var_q_temp2__blk815_dn6)))) / (assign38420_e43454 * assign38420_e43454)), ((((-locals.var_q_qsq__blk825_dn7) * assign38420_e43454) - (assign38420_e43451 * ((locals.var_q_temp2__blk815_dn7 * locals.var_q_temp2__blk815) + (locals.var_q_temp2__blk815 * locals.var_q_temp2__blk815_dn7)))) / (assign38420_e43454 * assign38420_e43454)), ((((-locals.var_q_qsq__blk825_dn8) * assign38420_e43454) - (assign38420_e43451 * ((locals.var_q_temp2__blk815_dn8 * locals.var_q_temp2__blk815) + (locals.var_q_temp2__blk815 * locals.var_q_temp2__blk815_dn8)))) / (assign38420_e43454 * assign38420_e43454)), ((((-locals.var_q_qsq__blk825_dn9) * assign38420_e43454) - (assign38420_e43451 * ((locals.var_q_temp2__blk815_dn9 * locals.var_q_temp2__blk815) + (locals.var_q_temp2__blk815 * locals.var_q_temp2__blk815_dn9)))) / (assign38420_e43454 * assign38420_e43454)), );
            locals.var_q_sh_term__blk833_rv = 0.0;
        }

        if (((locals.var_guard1080 != 0.0) && (locals.var_guard1192 == 0.0)) && (locals.var_guard1193 != 0.0)) {
            let assign38430_e43465: f64 = (locals.var_q_sh_term__blk833).ln();
            (locals.var_q_ln_term__blk834, locals.var_q_ln_term__blk834_dn4, locals.var_q_ln_term__blk834_dn6, locals.var_q_ln_term__blk834_dn7, locals.var_q_ln_term__blk834_dn8, locals.var_q_ln_term__blk834_dn9, ) = (assign38430_e43465, (locals.var_q_sh_term__blk833_dn4 / locals.var_q_sh_term__blk833), (locals.var_q_sh_term__blk833_dn6 / locals.var_q_sh_term__blk833), (locals.var_q_sh_term__blk833_dn7 / locals.var_q_sh_term__blk833), (locals.var_q_sh_term__blk833_dn8 / locals.var_q_sh_term__blk833), (locals.var_q_sh_term__blk833_dn9 / locals.var_q_sh_term__blk833), );
            locals.var_q_ln_term__blk834_rv = 0.0;
        }

        if (((locals.var_guard1080 != 0.0) && (locals.var_guard1192 == 0.0)) && (locals.var_guard1193 == 0.0)) {
            let assign38440_e43478: f64 = (locals.var_q_qsq__blk825 * 0.3333333333333);
            let assign38440_e43482: f64 = (0.05 * locals.var_q_qsq__blk825);
            let assign38440_e43486: f64 = (0.0396825396825397 * locals.var_q_qsq__blk825);
            let assign38440_e43487: f64 = (1.0 - assign38440_e43486);
            let assign38440_e43488: f64 = (assign38440_e43482 * assign38440_e43487);
            let assign38440_e43489: f64 = (1.0 - assign38440_e43488);
            let assign38440_e43490: f64 = (assign38440_e43478 * assign38440_e43489);
            let assign38440_e43491: f64 = (4.0 - assign38440_e43490);
            (locals.var_q_sh_term__blk833, locals.var_q_sh_term__blk833_dn4, locals.var_q_sh_term__blk833_dn6, locals.var_q_sh_term__blk833_dn7, locals.var_q_sh_term__blk833_dn8, locals.var_q_sh_term__blk833_dn9, ) = (assign38440_e43491, (-(((locals.var_q_qsq__blk825_dn4 * 0.3333333333333) * assign38440_e43489) + (assign38440_e43478 * (-(((0.05 * locals.var_q_qsq__blk825_dn4) * assign38440_e43487) + (assign38440_e43482 * (-(0.0396825396825397 * locals.var_q_qsq__blk825_dn4)))))))), (-(((locals.var_q_qsq__blk825_dn6 * 0.3333333333333) * assign38440_e43489) + (assign38440_e43478 * (-(((0.05 * locals.var_q_qsq__blk825_dn6) * assign38440_e43487) + (assign38440_e43482 * (-(0.0396825396825397 * locals.var_q_qsq__blk825_dn6)))))))), (-(((locals.var_q_qsq__blk825_dn7 * 0.3333333333333) * assign38440_e43489) + (assign38440_e43478 * (-(((0.05 * locals.var_q_qsq__blk825_dn7) * assign38440_e43487) + (assign38440_e43482 * (-(0.0396825396825397 * locals.var_q_qsq__blk825_dn7)))))))), (-(((locals.var_q_qsq__blk825_dn8 * 0.3333333333333) * assign38440_e43489) + (assign38440_e43478 * (-(((0.05 * locals.var_q_qsq__blk825_dn8) * assign38440_e43487) + (assign38440_e43482 * (-(0.0396825396825397 * locals.var_q_qsq__blk825_dn8)))))))), (-(((locals.var_q_qsq__blk825_dn9 * 0.3333333333333) * assign38440_e43489) + (assign38440_e43478 * (-(((0.05 * locals.var_q_qsq__blk825_dn9) * assign38440_e43487) + (assign38440_e43482 * (-(0.0396825396825397 * locals.var_q_qsq__blk825_dn9)))))))), );
            locals.var_q_sh_term__blk833_rv = 0.0;
        }

        if (((locals.var_guard1080 != 0.0) && (locals.var_guard1192 == 0.0)) && (locals.var_guard1193 == 0.0)) {
            let assign38450_e43502: f64 = (locals.var_q_sh_term__blk833).ln();
            (locals.var_q_ln_term__blk834, locals.var_q_ln_term__blk834_dn4, locals.var_q_ln_term__blk834_dn6, locals.var_q_ln_term__blk834_dn7, locals.var_q_ln_term__blk834_dn8, locals.var_q_ln_term__blk834_dn9, ) = (assign38450_e43502, (locals.var_q_sh_term__blk833_dn4 / locals.var_q_sh_term__blk833), (locals.var_q_sh_term__blk833_dn6 / locals.var_q_sh_term__blk833), (locals.var_q_sh_term__blk833_dn7 / locals.var_q_sh_term__blk833), (locals.var_q_sh_term__blk833_dn8 / locals.var_q_sh_term__blk833), (locals.var_q_sh_term__blk833_dn9 / locals.var_q_sh_term__blk833), );
            locals.var_q_ln_term__blk834_rv = 0.0;
        }

        let assign38460_e43507: f64 = (1.01 * locals.var_q_k1q1__blk823);
        let assign38460_e43509: f64 = (assign38460_e43507 + locals.var_q_qcoth__blk829);
        let assign38460_e43511: f64 = if assign38460_e43509 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1194 = assign38460_e43511;
        locals.var_guard1194_rv = 0.0;

        if ((locals.var_guard1080 != 0.0) && (locals.var_guard1194 != 0.0)) {
            let assign38470_e43517: f64 = (locals.var_q_k1q1__blk823 + locals.var_q_qcoth__blk829);
            (locals.var_q_expnum__blk837, locals.var_q_expnum__blk837_dn4, locals.var_q_expnum__blk837_dn6, locals.var_q_expnum__blk837_dn7, locals.var_q_expnum__blk837_dn8, locals.var_q_expnum__blk837_dn9, ) = (assign38470_e43517, (locals.var_q_k1q1__blk823_dn4 + locals.var_q_qcoth__blk829_dn4), (locals.var_q_k1q1__blk823_dn6 + locals.var_q_qcoth__blk829_dn6), (locals.var_q_k1q1__blk823_dn7 + locals.var_q_qcoth__blk829_dn7), (locals.var_q_k1q1__blk823_dn8 + locals.var_q_qcoth__blk829_dn8), (locals.var_q_k1q1__blk823_dn9 + locals.var_q_qcoth__blk829_dn9), );
            locals.var_q_expnum__blk837_rv = 0.0;
        }

        if ((locals.var_guard1080 != 0.0) && (locals.var_guard1194 != 0.0)) {
            let assign38480_e43525: f64 = (locals.var_k1__blk932 + locals.var_q_d1_qcoth__blk830);
            (locals.var_q_d1_expnum__blk838, locals.var_q_d1_expnum__blk838_dn4, locals.var_q_d1_expnum__blk838_dn6, locals.var_q_d1_expnum__blk838_dn7, locals.var_q_d1_expnum__blk838_dn8, locals.var_q_d1_expnum__blk838_dn9, ) = (assign38480_e43525, (locals.var_k1__blk932_dn4 + locals.var_q_d1_qcoth__blk830_dn4), (locals.var_k1__blk932_dn6 + locals.var_q_d1_qcoth__blk830_dn6), (locals.var_k1__blk932_dn7 + locals.var_q_d1_qcoth__blk830_dn7), (locals.var_k1__blk932_dn8 + locals.var_q_d1_qcoth__blk830_dn8), (locals.var_k1__blk932_dn9 + locals.var_q_d1_qcoth__blk830_dn9), );
            locals.var_q_d1_expnum__blk838_rv = 0.0;
        }

        if ((locals.var_guard1080 != 0.0) && (locals.var_guard1194 != 0.0)) {
            (locals.var_q_d2_expnum__blk839, locals.var_q_d2_expnum__blk839_dn4, locals.var_q_d2_expnum__blk839_dn6, locals.var_q_d2_expnum__blk839_dn7, locals.var_q_d2_expnum__blk839_dn8, locals.var_q_d2_expnum__blk839_dn9, ) = (locals.var_q_d2_qcoth__blk832, locals.var_q_d2_qcoth__blk832_dn4, locals.var_q_d2_qcoth__blk832_dn6, locals.var_q_d2_qcoth__blk832_dn7, locals.var_q_d2_qcoth__blk832_dn8, locals.var_q_d2_qcoth__blk832_dn9, );
            locals.var_q_d2_expnum__blk839_rv = 0.0;
        }

        if ((locals.var_guard1080 != 0.0) && (locals.var_guard1194 == 0.0)) {
            let assign38500_e43541: f64 = (locals.var_q_k1q1__blk823 - locals.var_q_qcoth__blk829);
            let assign38500_e43542: f64 = (1.0 / assign38500_e43541);
            (locals.var_q_temp2__blk815, locals.var_q_temp2__blk815_dn4, locals.var_q_temp2__blk815_dn6, locals.var_q_temp2__blk815_dn7, locals.var_q_temp2__blk815_dn8, locals.var_q_temp2__blk815_dn9, ) = (assign38500_e43542, (-((locals.var_q_k1q1__blk823_dn4 - locals.var_q_qcoth__blk829_dn4) / (assign38500_e43541 * assign38500_e43541))), (-((locals.var_q_k1q1__blk823_dn6 - locals.var_q_qcoth__blk829_dn6) / (assign38500_e43541 * assign38500_e43541))), (-((locals.var_q_k1q1__blk823_dn7 - locals.var_q_qcoth__blk829_dn7) / (assign38500_e43541 * assign38500_e43541))), (-((locals.var_q_k1q1__blk823_dn8 - locals.var_q_qcoth__blk829_dn8) / (assign38500_e43541 * assign38500_e43541))), (-((locals.var_q_k1q1__blk823_dn9 - locals.var_q_qcoth__blk829_dn9) / (assign38500_e43541 * assign38500_e43541))), );
            locals.var_q_temp2__blk815_rv = 0.0;
        }

        if ((locals.var_guard1080 != 0.0) && (locals.var_guard1194 == 0.0)) {
            let assign38510_e43551: f64 = (locals.var_q_d1_qcoth__blk830 - locals.var_k1__blk932);
            (locals.var_q_temp3__blk816, locals.var_q_temp3__blk816_dn4, locals.var_q_temp3__blk816_dn6, locals.var_q_temp3__blk816_dn7, locals.var_q_temp3__blk816_dn8, locals.var_q_temp3__blk816_dn9, ) = (assign38510_e43551, (locals.var_q_d1_qcoth__blk830_dn4 - locals.var_k1__blk932_dn4), (locals.var_q_d1_qcoth__blk830_dn6 - locals.var_k1__blk932_dn6), (locals.var_q_d1_qcoth__blk830_dn7 - locals.var_k1__blk932_dn7), (locals.var_q_d1_qcoth__blk830_dn8 - locals.var_k1__blk932_dn8), (locals.var_q_d1_qcoth__blk830_dn9 - locals.var_k1__blk932_dn9), );
            locals.var_q_temp3__blk816_rv = 0.0;
        }

        if ((locals.var_guard1080 != 0.0) && (locals.var_guard1194 == 0.0)) {
            let assign38520_e43560: f64 = (locals.var_q_aexp__blk824 - locals.var_q_sh_term__blk833);
            let assign38520_e43562: f64 = (assign38520_e43560 * locals.var_q_temp2__blk815);
            (locals.var_q_expnum__blk837, locals.var_q_expnum__blk837_dn4, locals.var_q_expnum__blk837_dn6, locals.var_q_expnum__blk837_dn7, locals.var_q_expnum__blk837_dn8, locals.var_q_expnum__blk837_dn9, ) = (assign38520_e43562, (((locals.var_q_aexp__blk824_dn4 - locals.var_q_sh_term__blk833_dn4) * locals.var_q_temp2__blk815) + (assign38520_e43560 * locals.var_q_temp2__blk815_dn4)), (((locals.var_q_aexp__blk824_dn6 - locals.var_q_sh_term__blk833_dn6) * locals.var_q_temp2__blk815) + (assign38520_e43560 * locals.var_q_temp2__blk815_dn6)), (((locals.var_q_aexp__blk824_dn7 - locals.var_q_sh_term__blk833_dn7) * locals.var_q_temp2__blk815) + (assign38520_e43560 * locals.var_q_temp2__blk815_dn7)), (((locals.var_q_aexp__blk824_dn8 - locals.var_q_sh_term__blk833_dn8) * locals.var_q_temp2__blk815) + (assign38520_e43560 * locals.var_q_temp2__blk815_dn8)), (((locals.var_q_aexp__blk824_dn9 - locals.var_q_sh_term__blk833_dn9) * locals.var_q_temp2__blk815) + (assign38520_e43560 * locals.var_q_temp2__blk815_dn9)), );
            locals.var_q_expnum__blk837_rv = 0.0;
        }

        if ((locals.var_guard1080 != 0.0) && (locals.var_guard1194 == 0.0)) {
            let assign38530_e43571: f64 = (locals.var_q_temp3__blk816 * locals.var_q_expnum__blk837);
            let assign38530_e43573: f64 = (assign38530_e43571 - locals.var_q_aexp__blk824);
            let assign38530_e43576: f64 = (locals.var_q_d1_ln__blk835 * locals.var_q_sh_term__blk833);
            let assign38530_e43577: f64 = (assign38530_e43573 - assign38530_e43576);
            let assign38530_e43579: f64 = (assign38530_e43577 * locals.var_q_temp2__blk815);
            (locals.var_q_d1_expnum__blk838, locals.var_q_d1_expnum__blk838_dn4, locals.var_q_d1_expnum__blk838_dn6, locals.var_q_d1_expnum__blk838_dn7, locals.var_q_d1_expnum__blk838_dn8, locals.var_q_d1_expnum__blk838_dn9, ) = (assign38530_e43579, ((((((locals.var_q_temp3__blk816_dn4 * locals.var_q_expnum__blk837) + (locals.var_q_temp3__blk816 * locals.var_q_expnum__blk837_dn4)) - locals.var_q_aexp__blk824_dn4) - ((locals.var_q_d1_ln__blk835_dn4 * locals.var_q_sh_term__blk833) + (locals.var_q_d1_ln__blk835 * locals.var_q_sh_term__blk833_dn4))) * locals.var_q_temp2__blk815) + (assign38530_e43577 * locals.var_q_temp2__blk815_dn4)), ((((((locals.var_q_temp3__blk816_dn6 * locals.var_q_expnum__blk837) + (locals.var_q_temp3__blk816 * locals.var_q_expnum__blk837_dn6)) - locals.var_q_aexp__blk824_dn6) - ((locals.var_q_d1_ln__blk835_dn6 * locals.var_q_sh_term__blk833) + (locals.var_q_d1_ln__blk835 * locals.var_q_sh_term__blk833_dn6))) * locals.var_q_temp2__blk815) + (assign38530_e43577 * locals.var_q_temp2__blk815_dn6)), ((((((locals.var_q_temp3__blk816_dn7 * locals.var_q_expnum__blk837) + (locals.var_q_temp3__blk816 * locals.var_q_expnum__blk837_dn7)) - locals.var_q_aexp__blk824_dn7) - ((locals.var_q_d1_ln__blk835_dn7 * locals.var_q_sh_term__blk833) + (locals.var_q_d1_ln__blk835 * locals.var_q_sh_term__blk833_dn7))) * locals.var_q_temp2__blk815) + (assign38530_e43577 * locals.var_q_temp2__blk815_dn7)), ((((((locals.var_q_temp3__blk816_dn8 * locals.var_q_expnum__blk837) + (locals.var_q_temp3__blk816 * locals.var_q_expnum__blk837_dn8)) - locals.var_q_aexp__blk824_dn8) - ((locals.var_q_d1_ln__blk835_dn8 * locals.var_q_sh_term__blk833) + (locals.var_q_d1_ln__blk835 * locals.var_q_sh_term__blk833_dn8))) * locals.var_q_temp2__blk815) + (assign38530_e43577 * locals.var_q_temp2__blk815_dn8)), ((((((locals.var_q_temp3__blk816_dn9 * locals.var_q_expnum__blk837) + (locals.var_q_temp3__blk816 * locals.var_q_expnum__blk837_dn9)) - locals.var_q_aexp__blk824_dn9) - ((locals.var_q_d1_ln__blk835_dn9 * locals.var_q_sh_term__blk833) + (locals.var_q_d1_ln__blk835 * locals.var_q_sh_term__blk833_dn9))) * locals.var_q_temp2__blk815) + (assign38530_e43577 * locals.var_q_temp2__blk815_dn9)), );
            locals.var_q_d1_expnum__blk838_rv = 0.0;
        }

        if ((locals.var_guard1080 != 0.0) && (locals.var_guard1194 == 0.0)) {
            let assign38540_e43588: f64 = (locals.var_q_d2_qcoth__blk832 * locals.var_q_expnum__blk837);
            let assign38540_e43591: f64 = (2.0 * locals.var_q_temp3__blk816);
            let assign38540_e43593: f64 = (assign38540_e43591 * locals.var_q_d1_expnum__blk838);
            let assign38540_e43594: f64 = (assign38540_e43588 + assign38540_e43593);
            let assign38540_e43596: f64 = (assign38540_e43594 + locals.var_q_aexp__blk824);
            let assign38540_e43600: f64 = (locals.var_q_d1_ln__blk835 * locals.var_q_d1_ln__blk835);
            let assign38540_e43601: f64 = (locals.var_q_d2_ln__blk836 + assign38540_e43600);
            let assign38540_e43603: f64 = (assign38540_e43601 * locals.var_q_sh_term__blk833);
            let assign38540_e43604: f64 = (assign38540_e43596 - assign38540_e43603);
            let assign38540_e43606: f64 = (assign38540_e43604 * locals.var_q_temp2__blk815);
            (locals.var_q_d2_expnum__blk839, locals.var_q_d2_expnum__blk839_dn4, locals.var_q_d2_expnum__blk839_dn6, locals.var_q_d2_expnum__blk839_dn7, locals.var_q_d2_expnum__blk839_dn8, locals.var_q_d2_expnum__blk839_dn9, ) = (assign38540_e43606, (((((((locals.var_q_d2_qcoth__blk832_dn4 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qcoth__blk832 * locals.var_q_expnum__blk837_dn4)) + (((2.0 * locals.var_q_temp3__blk816_dn4) * locals.var_q_d1_expnum__blk838) + (assign38540_e43591 * locals.var_q_d1_expnum__blk838_dn4))) + locals.var_q_aexp__blk824_dn4) - (((locals.var_q_d2_ln__blk836_dn4 + ((locals.var_q_d1_ln__blk835_dn4 * locals.var_q_d1_ln__blk835) + (locals.var_q_d1_ln__blk835 * locals.var_q_d1_ln__blk835_dn4))) * locals.var_q_sh_term__blk833) + (assign38540_e43601 * locals.var_q_sh_term__blk833_dn4))) * locals.var_q_temp2__blk815) + (assign38540_e43604 * locals.var_q_temp2__blk815_dn4)), (((((((locals.var_q_d2_qcoth__blk832_dn6 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qcoth__blk832 * locals.var_q_expnum__blk837_dn6)) + (((2.0 * locals.var_q_temp3__blk816_dn6) * locals.var_q_d1_expnum__blk838) + (assign38540_e43591 * locals.var_q_d1_expnum__blk838_dn6))) + locals.var_q_aexp__blk824_dn6) - (((locals.var_q_d2_ln__blk836_dn6 + ((locals.var_q_d1_ln__blk835_dn6 * locals.var_q_d1_ln__blk835) + (locals.var_q_d1_ln__blk835 * locals.var_q_d1_ln__blk835_dn6))) * locals.var_q_sh_term__blk833) + (assign38540_e43601 * locals.var_q_sh_term__blk833_dn6))) * locals.var_q_temp2__blk815) + (assign38540_e43604 * locals.var_q_temp2__blk815_dn6)), (((((((locals.var_q_d2_qcoth__blk832_dn7 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qcoth__blk832 * locals.var_q_expnum__blk837_dn7)) + (((2.0 * locals.var_q_temp3__blk816_dn7) * locals.var_q_d1_expnum__blk838) + (assign38540_e43591 * locals.var_q_d1_expnum__blk838_dn7))) + locals.var_q_aexp__blk824_dn7) - (((locals.var_q_d2_ln__blk836_dn7 + ((locals.var_q_d1_ln__blk835_dn7 * locals.var_q_d1_ln__blk835) + (locals.var_q_d1_ln__blk835 * locals.var_q_d1_ln__blk835_dn7))) * locals.var_q_sh_term__blk833) + (assign38540_e43601 * locals.var_q_sh_term__blk833_dn7))) * locals.var_q_temp2__blk815) + (assign38540_e43604 * locals.var_q_temp2__blk815_dn7)), (((((((locals.var_q_d2_qcoth__blk832_dn8 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qcoth__blk832 * locals.var_q_expnum__blk837_dn8)) + (((2.0 * locals.var_q_temp3__blk816_dn8) * locals.var_q_d1_expnum__blk838) + (assign38540_e43591 * locals.var_q_d1_expnum__blk838_dn8))) + locals.var_q_aexp__blk824_dn8) - (((locals.var_q_d2_ln__blk836_dn8 + ((locals.var_q_d1_ln__blk835_dn8 * locals.var_q_d1_ln__blk835) + (locals.var_q_d1_ln__blk835 * locals.var_q_d1_ln__blk835_dn8))) * locals.var_q_sh_term__blk833) + (assign38540_e43601 * locals.var_q_sh_term__blk833_dn8))) * locals.var_q_temp2__blk815) + (assign38540_e43604 * locals.var_q_temp2__blk815_dn8)), (((((((locals.var_q_d2_qcoth__blk832_dn9 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qcoth__blk832 * locals.var_q_expnum__blk837_dn9)) + (((2.0 * locals.var_q_temp3__blk816_dn9) * locals.var_q_d1_expnum__blk838) + (assign38540_e43591 * locals.var_q_d1_expnum__blk838_dn9))) + locals.var_q_aexp__blk824_dn9) - (((locals.var_q_d2_ln__blk836_dn9 + ((locals.var_q_d1_ln__blk835_dn9 * locals.var_q_d1_ln__blk835) + (locals.var_q_d1_ln__blk835 * locals.var_q_d1_ln__blk835_dn9))) * locals.var_q_sh_term__blk833) + (assign38540_e43601 * locals.var_q_sh_term__blk833_dn9))) * locals.var_q_temp2__blk815) + (assign38540_e43604 * locals.var_q_temp2__blk815_dn9)), );
            locals.var_q_d2_expnum__blk839_rv = 0.0;
        }

        let assign38550_e43611: f64 = if locals.var_q_expnum__blk837 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1195 = assign38550_e43611;
        locals.var_guard1195_rv = 0.0;

        if ((locals.var_guard1080 != 0.0) && (locals.var_guard1195 != 0.0)) {
            let assign38560_e43616: f64 = (locals.var_q_expnum__blk837).ln();
            (locals.var_q_lnexpnum__blk840, locals.var_q_lnexpnum__blk840_dn4, locals.var_q_lnexpnum__blk840_dn6, locals.var_q_lnexpnum__blk840_dn7, locals.var_q_lnexpnum__blk840_dn8, locals.var_q_lnexpnum__blk840_dn9, ) = (assign38560_e43616, (locals.var_q_expnum__blk837_dn4 / locals.var_q_expnum__blk837), (locals.var_q_expnum__blk837_dn6 / locals.var_q_expnum__blk837), (locals.var_q_expnum__blk837_dn7 / locals.var_q_expnum__blk837), (locals.var_q_expnum__blk837_dn8 / locals.var_q_expnum__blk837), (locals.var_q_expnum__blk837_dn9 / locals.var_q_expnum__blk837), );
            locals.var_q_lnexpnum__blk840_rv = 0.0;
        }

        if ((locals.var_guard1080 != 0.0) && (locals.var_guard1195 != 0.0)) {
            let assign38570_e43624: f64 = (1.0 / locals.var_q_expnum__blk837);
            (locals.var_q_temp1__blk814, locals.var_q_temp1__blk814_dn4, locals.var_q_temp1__blk814_dn6, locals.var_q_temp1__blk814_dn7, locals.var_q_temp1__blk814_dn8, locals.var_q_temp1__blk814_dn9, ) = (assign38570_e43624, (-(locals.var_q_expnum__blk837_dn4 / (locals.var_q_expnum__blk837 * locals.var_q_expnum__blk837))), (-(locals.var_q_expnum__blk837_dn6 / (locals.var_q_expnum__blk837 * locals.var_q_expnum__blk837))), (-(locals.var_q_expnum__blk837_dn7 / (locals.var_q_expnum__blk837 * locals.var_q_expnum__blk837))), (-(locals.var_q_expnum__blk837_dn8 / (locals.var_q_expnum__blk837 * locals.var_q_expnum__blk837))), (-(locals.var_q_expnum__blk837_dn9 / (locals.var_q_expnum__blk837 * locals.var_q_expnum__blk837))), );
            locals.var_q_temp1__blk814_rv = 0.0;
        }

        if ((locals.var_guard1080 != 0.0) && (locals.var_guard1195 != 0.0)) {
            let assign38580_e43632: f64 = (locals.var_q_d1_expnum__blk838 * locals.var_q_temp1__blk814);
            (locals.var_q_d1_lnexpnum__blk841, locals.var_q_d1_lnexpnum__blk841_dn4, locals.var_q_d1_lnexpnum__blk841_dn6, locals.var_q_d1_lnexpnum__blk841_dn7, locals.var_q_d1_lnexpnum__blk841_dn8, locals.var_q_d1_lnexpnum__blk841_dn9, ) = (assign38580_e43632, ((locals.var_q_d1_expnum__blk838_dn4 * locals.var_q_temp1__blk814) + (locals.var_q_d1_expnum__blk838 * locals.var_q_temp1__blk814_dn4)), ((locals.var_q_d1_expnum__blk838_dn6 * locals.var_q_temp1__blk814) + (locals.var_q_d1_expnum__blk838 * locals.var_q_temp1__blk814_dn6)), ((locals.var_q_d1_expnum__blk838_dn7 * locals.var_q_temp1__blk814) + (locals.var_q_d1_expnum__blk838 * locals.var_q_temp1__blk814_dn7)), ((locals.var_q_d1_expnum__blk838_dn8 * locals.var_q_temp1__blk814) + (locals.var_q_d1_expnum__blk838 * locals.var_q_temp1__blk814_dn8)), ((locals.var_q_d1_expnum__blk838_dn9 * locals.var_q_temp1__blk814) + (locals.var_q_d1_expnum__blk838 * locals.var_q_temp1__blk814_dn9)), );
            locals.var_q_d1_lnexpnum__blk841_rv = 0.0;
        }

        if ((locals.var_guard1080 != 0.0) && (locals.var_guard1195 != 0.0)) {
            let assign38590_e43640: f64 = (locals.var_q_d2_expnum__blk839 * locals.var_q_temp1__blk814);
            let assign38590_e43643: f64 = (locals.var_q_d1_lnexpnum__blk841 * locals.var_q_d1_lnexpnum__blk841);
            let assign38590_e43644: f64 = (assign38590_e43640 - assign38590_e43643);
            (locals.var_q_d2_lnexpnum__blk842, locals.var_q_d2_lnexpnum__blk842_dn4, locals.var_q_d2_lnexpnum__blk842_dn6, locals.var_q_d2_lnexpnum__blk842_dn7, locals.var_q_d2_lnexpnum__blk842_dn8, locals.var_q_d2_lnexpnum__blk842_dn9, ) = (assign38590_e43644, (((locals.var_q_d2_expnum__blk839_dn4 * locals.var_q_temp1__blk814) + (locals.var_q_d2_expnum__blk839 * locals.var_q_temp1__blk814_dn4)) - ((locals.var_q_d1_lnexpnum__blk841_dn4 * locals.var_q_d1_lnexpnum__blk841) + (locals.var_q_d1_lnexpnum__blk841 * locals.var_q_d1_lnexpnum__blk841_dn4))), (((locals.var_q_d2_expnum__blk839_dn6 * locals.var_q_temp1__blk814) + (locals.var_q_d2_expnum__blk839 * locals.var_q_temp1__blk814_dn6)) - ((locals.var_q_d1_lnexpnum__blk841_dn6 * locals.var_q_d1_lnexpnum__blk841) + (locals.var_q_d1_lnexpnum__blk841 * locals.var_q_d1_lnexpnum__blk841_dn6))), (((locals.var_q_d2_expnum__blk839_dn7 * locals.var_q_temp1__blk814) + (locals.var_q_d2_expnum__blk839 * locals.var_q_temp1__blk814_dn7)) - ((locals.var_q_d1_lnexpnum__blk841_dn7 * locals.var_q_d1_lnexpnum__blk841) + (locals.var_q_d1_lnexpnum__blk841 * locals.var_q_d1_lnexpnum__blk841_dn7))), (((locals.var_q_d2_expnum__blk839_dn8 * locals.var_q_temp1__blk814) + (locals.var_q_d2_expnum__blk839 * locals.var_q_temp1__blk814_dn8)) - ((locals.var_q_d1_lnexpnum__blk841_dn8 * locals.var_q_d1_lnexpnum__blk841) + (locals.var_q_d1_lnexpnum__blk841 * locals.var_q_d1_lnexpnum__blk841_dn8))), (((locals.var_q_d2_expnum__blk839_dn9 * locals.var_q_temp1__blk814) + (locals.var_q_d2_expnum__blk839 * locals.var_q_temp1__blk814_dn9)) - ((locals.var_q_d1_lnexpnum__blk841_dn9 * locals.var_q_d1_lnexpnum__blk841) + (locals.var_q_d1_lnexpnum__blk841 * locals.var_q_d1_lnexpnum__blk841_dn9))), );
            locals.var_q_d2_lnexpnum__blk842_rv = 0.0;
        }

        if ((locals.var_guard1080 != 0.0) && (locals.var_guard1195 == 0.0)) {
            let assign38600_e43653: f64 = (locals.var_q_k1q1__blk823 + 0.6931471805599);
            let assign38600_e43655: f64 = (-locals.var_q_k1q1__blk823);
            let assign38600_e43656: f64 = (assign38600_e43655).ln();
            let assign38600_e43657: f64 = (assign38600_e43653 + assign38600_e43656);
            (locals.var_q_lnexpnum__blk840, locals.var_q_lnexpnum__blk840_dn4, locals.var_q_lnexpnum__blk840_dn6, locals.var_q_lnexpnum__blk840_dn7, locals.var_q_lnexpnum__blk840_dn8, locals.var_q_lnexpnum__blk840_dn9, ) = (assign38600_e43657, (locals.var_q_k1q1__blk823_dn4 + ((-locals.var_q_k1q1__blk823_dn4) / assign38600_e43655)), (locals.var_q_k1q1__blk823_dn6 + ((-locals.var_q_k1q1__blk823_dn6) / assign38600_e43655)), (locals.var_q_k1q1__blk823_dn7 + ((-locals.var_q_k1q1__blk823_dn7) / assign38600_e43655)), (locals.var_q_k1q1__blk823_dn8 + ((-locals.var_q_k1q1__blk823_dn8) / assign38600_e43655)), (locals.var_q_k1q1__blk823_dn9 + ((-locals.var_q_k1q1__blk823_dn9) / assign38600_e43655)), );
            locals.var_q_lnexpnum__blk840_rv = 0.0;
        }

        if ((locals.var_guard1080 != 0.0) && (locals.var_guard1195 == 0.0)) {
            let assign38610_e43666: f64 = (1.0 / locals.var_q1d__blk1001);
            (locals.var_q_temp1__blk814, locals.var_q_temp1__blk814_dn4, locals.var_q_temp1__blk814_dn6, locals.var_q_temp1__blk814_dn7, locals.var_q_temp1__blk814_dn8, locals.var_q_temp1__blk814_dn9, ) = (assign38610_e43666, (-(locals.var_q1d__blk1001_dn4 / (locals.var_q1d__blk1001 * locals.var_q1d__blk1001))), (-(locals.var_q1d__blk1001_dn6 / (locals.var_q1d__blk1001 * locals.var_q1d__blk1001))), (-(locals.var_q1d__blk1001_dn7 / (locals.var_q1d__blk1001 * locals.var_q1d__blk1001))), (-(locals.var_q1d__blk1001_dn8 / (locals.var_q1d__blk1001 * locals.var_q1d__blk1001))), (-(locals.var_q1d__blk1001_dn9 / (locals.var_q1d__blk1001 * locals.var_q1d__blk1001))), );
            locals.var_q_temp1__blk814_rv = 0.0;
        }

        if ((locals.var_guard1080 != 0.0) && (locals.var_guard1195 == 0.0)) {
            let assign38620_e43675: f64 = (locals.var_k1__blk932 + locals.var_q_temp1__blk814);
            (locals.var_q_d1_lnexpnum__blk841, locals.var_q_d1_lnexpnum__blk841_dn4, locals.var_q_d1_lnexpnum__blk841_dn6, locals.var_q_d1_lnexpnum__blk841_dn7, locals.var_q_d1_lnexpnum__blk841_dn8, locals.var_q_d1_lnexpnum__blk841_dn9, ) = (assign38620_e43675, (locals.var_k1__blk932_dn4 + locals.var_q_temp1__blk814_dn4), (locals.var_k1__blk932_dn6 + locals.var_q_temp1__blk814_dn6), (locals.var_k1__blk932_dn7 + locals.var_q_temp1__blk814_dn7), (locals.var_k1__blk932_dn8 + locals.var_q_temp1__blk814_dn8), (locals.var_k1__blk932_dn9 + locals.var_q_temp1__blk814_dn9), );
            locals.var_q_d1_lnexpnum__blk841_rv = 0.0;
        }

        if ((locals.var_guard1080 != 0.0) && (locals.var_guard1195 == 0.0)) {
            let assign38630_e43683: f64 = (-locals.var_q_temp1__blk814);
            let assign38630_e43685: f64 = (assign38630_e43683 * locals.var_q_temp1__blk814);
            (locals.var_q_d2_lnexpnum__blk842, locals.var_q_d2_lnexpnum__blk842_dn4, locals.var_q_d2_lnexpnum__blk842_dn6, locals.var_q_d2_lnexpnum__blk842_dn7, locals.var_q_d2_lnexpnum__blk842_dn8, locals.var_q_d2_lnexpnum__blk842_dn9, ) = (assign38630_e43685, (((-locals.var_q_temp1__blk814_dn4) * locals.var_q_temp1__blk814) + (assign38630_e43683 * locals.var_q_temp1__blk814_dn4)), (((-locals.var_q_temp1__blk814_dn6) * locals.var_q_temp1__blk814) + (assign38630_e43683 * locals.var_q_temp1__blk814_dn6)), (((-locals.var_q_temp1__blk814_dn7) * locals.var_q_temp1__blk814) + (assign38630_e43683 * locals.var_q_temp1__blk814_dn7)), (((-locals.var_q_temp1__blk814_dn8) * locals.var_q_temp1__blk814) + (assign38630_e43683 * locals.var_q_temp1__blk814_dn8)), (((-locals.var_q_temp1__blk814_dn9) * locals.var_q_temp1__blk814) + (assign38630_e43683 * locals.var_q_temp1__blk814_dn9)), );
            locals.var_q_d2_lnexpnum__blk842_rv = 0.0;
        }

        if (locals.var_guard1080 != 0.0) {
            let assign38640_e43691: f64 = (locals.var_xg2x__blk931 - locals.var_xg1x__blk930);
            let assign38640_e43693: f64 = (assign38640_e43691 + locals.var_q1d__blk1001);
            let assign38640_e43696: f64 = (2.0 * locals.var_q_lnexpnum__blk840);
            let assign38640_e43697: f64 = (assign38640_e43693 + assign38640_e43696);
            let assign38640_e43699: f64 = (assign38640_e43697 - locals.var_q_ln_term__blk834);
            (locals.var_q_q2_int__blk843, locals.var_q_q2_int__blk843_dn4, locals.var_q_q2_int__blk843_dn6, locals.var_q_q2_int__blk843_dn7, locals.var_q_q2_int__blk843_dn8, locals.var_q_q2_int__blk843_dn9, ) = (assign38640_e43699, ((((locals.var_xg2x__blk931_dn4 - locals.var_xg1x__blk930_dn4) + locals.var_q1d__blk1001_dn4) + (2.0 * locals.var_q_lnexpnum__blk840_dn4)) - locals.var_q_ln_term__blk834_dn4), ((((locals.var_xg2x__blk931_dn6 - locals.var_xg1x__blk930_dn6) + locals.var_q1d__blk1001_dn6) + (2.0 * locals.var_q_lnexpnum__blk840_dn6)) - locals.var_q_ln_term__blk834_dn6), ((((locals.var_xg2x__blk931_dn7 - locals.var_xg1x__blk930_dn7) + locals.var_q1d__blk1001_dn7) + (2.0 * locals.var_q_lnexpnum__blk840_dn7)) - locals.var_q_ln_term__blk834_dn7), ((((locals.var_xg2x__blk931_dn8 - locals.var_xg1x__blk930_dn8) + locals.var_q1d__blk1001_dn8) + (2.0 * locals.var_q_lnexpnum__blk840_dn8)) - locals.var_q_ln_term__blk834_dn8), ((((locals.var_xg2x__blk931_dn9 - locals.var_xg1x__blk930_dn9) + locals.var_q1d__blk1001_dn9) + (2.0 * locals.var_q_lnexpnum__blk840_dn9)) - locals.var_q_ln_term__blk834_dn9), );
            locals.var_q_q2_int__blk843_rv = 0.0;
        }

        if (locals.var_guard1080 != 0.0) {
            let assign38650_e43706: f64 = (2.0 * locals.var_q_d1_lnexpnum__blk841);
            let assign38650_e43707: f64 = (1.0 + assign38650_e43706);
            let assign38650_e43709: f64 = (assign38650_e43707 - locals.var_q_d1_ln__blk835);
            (locals.var_q_d1_q2__blk844, locals.var_q_d1_q2__blk844_dn4, locals.var_q_d1_q2__blk844_dn6, locals.var_q_d1_q2__blk844_dn7, locals.var_q_d1_q2__blk844_dn8, locals.var_q_d1_q2__blk844_dn9, ) = (assign38650_e43709, ((2.0 * locals.var_q_d1_lnexpnum__blk841_dn4) - locals.var_q_d1_ln__blk835_dn4), ((2.0 * locals.var_q_d1_lnexpnum__blk841_dn6) - locals.var_q_d1_ln__blk835_dn6), ((2.0 * locals.var_q_d1_lnexpnum__blk841_dn7) - locals.var_q_d1_ln__blk835_dn7), ((2.0 * locals.var_q_d1_lnexpnum__blk841_dn8) - locals.var_q_d1_ln__blk835_dn8), ((2.0 * locals.var_q_d1_lnexpnum__blk841_dn9) - locals.var_q_d1_ln__blk835_dn9), );
            locals.var_q_d1_q2__blk844_rv = 0.0;
        }

        if (locals.var_guard1080 != 0.0) {
            let assign38660_e43715: f64 = (2.0 * locals.var_q_d2_lnexpnum__blk842);
            let assign38660_e43717: f64 = (assign38660_e43715 - locals.var_q_d2_ln__blk836);
            (locals.var_q_d2_q2__blk845, locals.var_q_d2_q2__blk845_dn4, locals.var_q_d2_q2__blk845_dn6, locals.var_q_d2_q2__blk845_dn7, locals.var_q_d2_q2__blk845_dn8, locals.var_q_d2_q2__blk845_dn9, ) = (assign38660_e43717, ((2.0 * locals.var_q_d2_lnexpnum__blk842_dn4) - locals.var_q_d2_ln__blk836_dn4), ((2.0 * locals.var_q_d2_lnexpnum__blk842_dn6) - locals.var_q_d2_ln__blk836_dn6), ((2.0 * locals.var_q_d2_lnexpnum__blk842_dn7) - locals.var_q_d2_ln__blk836_dn7), ((2.0 * locals.var_q_d2_lnexpnum__blk842_dn8) - locals.var_q_d2_ln__blk836_dn8), ((2.0 * locals.var_q_d2_lnexpnum__blk842_dn9) - locals.var_q_d2_ln__blk836_dn9), );
            locals.var_q_d2_q2__blk845_rv = 0.0;
        }

        if (locals.var_guard1080 != 0.0) {
            let assign38670_e43724: f64 = (locals.var_k2__blk933 * locals.var_q_q2_int__blk843);
            let assign38670_e43725: f64 = (locals.var_q_k1q1__blk823 + assign38670_e43724);
            (locals.var_q_qi_int__blk846, locals.var_q_qi_int__blk846_dn4, locals.var_q_qi_int__blk846_dn6, locals.var_q_qi_int__blk846_dn7, locals.var_q_qi_int__blk846_dn8, locals.var_q_qi_int__blk846_dn9, ) = (assign38670_e43725, (locals.var_q_k1q1__blk823_dn4 + ((locals.var_k2__blk933_dn4 * locals.var_q_q2_int__blk843) + (locals.var_k2__blk933 * locals.var_q_q2_int__blk843_dn4))), (locals.var_q_k1q1__blk823_dn6 + ((locals.var_k2__blk933_dn6 * locals.var_q_q2_int__blk843) + (locals.var_k2__blk933 * locals.var_q_q2_int__blk843_dn6))), (locals.var_q_k1q1__blk823_dn7 + ((locals.var_k2__blk933_dn7 * locals.var_q_q2_int__blk843) + (locals.var_k2__blk933 * locals.var_q_q2_int__blk843_dn7))), (locals.var_q_k1q1__blk823_dn8 + ((locals.var_k2__blk933_dn8 * locals.var_q_q2_int__blk843) + (locals.var_k2__blk933 * locals.var_q_q2_int__blk843_dn8))), (locals.var_q_k1q1__blk823_dn9 + ((locals.var_k2__blk933_dn9 * locals.var_q_q2_int__blk843) + (locals.var_k2__blk933 * locals.var_q_q2_int__blk843_dn9))), );
            locals.var_q_qi_int__blk846_rv = 0.0;
        }

        if (locals.var_guard1080 != 0.0) {
            let assign38680_e43732: f64 = (locals.var_k2__blk933 * locals.var_q_d1_q2__blk844);
            let assign38680_e43733: f64 = (locals.var_k1__blk932 + assign38680_e43732);
            (locals.var_q_d1_qi__blk847, locals.var_q_d1_qi__blk847_dn4, locals.var_q_d1_qi__blk847_dn6, locals.var_q_d1_qi__blk847_dn7, locals.var_q_d1_qi__blk847_dn8, locals.var_q_d1_qi__blk847_dn9, ) = (assign38680_e43733, (locals.var_k1__blk932_dn4 + ((locals.var_k2__blk933_dn4 * locals.var_q_d1_q2__blk844) + (locals.var_k2__blk933 * locals.var_q_d1_q2__blk844_dn4))), (locals.var_k1__blk932_dn6 + ((locals.var_k2__blk933_dn6 * locals.var_q_d1_q2__blk844) + (locals.var_k2__blk933 * locals.var_q_d1_q2__blk844_dn6))), (locals.var_k1__blk932_dn7 + ((locals.var_k2__blk933_dn7 * locals.var_q_d1_q2__blk844) + (locals.var_k2__blk933 * locals.var_q_d1_q2__blk844_dn7))), (locals.var_k1__blk932_dn8 + ((locals.var_k2__blk933_dn8 * locals.var_q_d1_q2__blk844) + (locals.var_k2__blk933 * locals.var_q_d1_q2__blk844_dn8))), (locals.var_k1__blk932_dn9 + ((locals.var_k2__blk933_dn9 * locals.var_q_d1_q2__blk844) + (locals.var_k2__blk933 * locals.var_q_d1_q2__blk844_dn9))), );
            locals.var_q_d1_qi__blk847_rv = 0.0;
        }

        if (locals.var_guard1080 != 0.0) {
            let assign38690_e43739: f64 = (locals.var_k2__blk933 * locals.var_q_d2_q2__blk845);
            (locals.var_q_d2_qi__blk848, locals.var_q_d2_qi__blk848_dn4, locals.var_q_d2_qi__blk848_dn6, locals.var_q_d2_qi__blk848_dn7, locals.var_q_d2_qi__blk848_dn8, locals.var_q_d2_qi__blk848_dn9, ) = (assign38690_e43739, ((locals.var_k2__blk933_dn4 * locals.var_q_d2_q2__blk845) + (locals.var_k2__blk933 * locals.var_q_d2_q2__blk845_dn4)), ((locals.var_k2__blk933_dn6 * locals.var_q_d2_q2__blk845) + (locals.var_k2__blk933 * locals.var_q_d2_q2__blk845_dn6)), ((locals.var_k2__blk933_dn7 * locals.var_q_d2_q2__blk845) + (locals.var_k2__blk933 * locals.var_q_d2_q2__blk845_dn7)), ((locals.var_k2__blk933_dn8 * locals.var_q_d2_q2__blk845) + (locals.var_k2__blk933 * locals.var_q_d2_q2__blk845_dn8)), ((locals.var_k2__blk933_dn9 * locals.var_q_d2_q2__blk845) + (locals.var_k2__blk933 * locals.var_q_d2_q2__blk845_dn9)), );
            locals.var_q_d2_qi__blk848_rv = 0.0;
        }

        if (locals.var_guard1080 != 0.0) {
            let assign38700_e43745: f64 = (locals.var_q_qi_int__blk846 * locals.var_q_expnum__blk837);
            let assign38700_e43747: f64 = (assign38700_e43745 - locals.var_q_aexp__blk824);
            (locals.var_q_zero__blk849, locals.var_q_zero__blk849_dn4, locals.var_q_zero__blk849_dn6, locals.var_q_zero__blk849_dn7, locals.var_q_zero__blk849_dn8, locals.var_q_zero__blk849_dn9, ) = (assign38700_e43747, (((locals.var_q_qi_int__blk846_dn4 * locals.var_q_expnum__blk837) + (locals.var_q_qi_int__blk846 * locals.var_q_expnum__blk837_dn4)) - locals.var_q_aexp__blk824_dn4), (((locals.var_q_qi_int__blk846_dn6 * locals.var_q_expnum__blk837) + (locals.var_q_qi_int__blk846 * locals.var_q_expnum__blk837_dn6)) - locals.var_q_aexp__blk824_dn6), (((locals.var_q_qi_int__blk846_dn7 * locals.var_q_expnum__blk837) + (locals.var_q_qi_int__blk846 * locals.var_q_expnum__blk837_dn7)) - locals.var_q_aexp__blk824_dn7), (((locals.var_q_qi_int__blk846_dn8 * locals.var_q_expnum__blk837) + (locals.var_q_qi_int__blk846 * locals.var_q_expnum__blk837_dn8)) - locals.var_q_aexp__blk824_dn8), (((locals.var_q_qi_int__blk846_dn9 * locals.var_q_expnum__blk837) + (locals.var_q_qi_int__blk846 * locals.var_q_expnum__blk837_dn9)) - locals.var_q_aexp__blk824_dn9), );
            locals.var_q_zero__blk849_rv = 0.0;
        }

        if (locals.var_guard1080 != 0.0) {
            let assign38710_e43753: f64 = (locals.var_q_d1_qi__blk847 * locals.var_q_expnum__blk837);
            let assign38710_e43756: f64 = (locals.var_q_qi_int__blk846 * locals.var_q_d1_expnum__blk838);
            let assign38710_e43757: f64 = (assign38710_e43753 + assign38710_e43756);
            let assign38710_e43759: f64 = (assign38710_e43757 + locals.var_q_aexp__blk824);
            (locals.var_q_d1_zero__blk850, locals.var_q_d1_zero__blk850_dn4, locals.var_q_d1_zero__blk850_dn6, locals.var_q_d1_zero__blk850_dn7, locals.var_q_d1_zero__blk850_dn8, locals.var_q_d1_zero__blk850_dn9, ) = (assign38710_e43759, ((((locals.var_q_d1_qi__blk847_dn4 * locals.var_q_expnum__blk837) + (locals.var_q_d1_qi__blk847 * locals.var_q_expnum__blk837_dn4)) + ((locals.var_q_qi_int__blk846_dn4 * locals.var_q_d1_expnum__blk838) + (locals.var_q_qi_int__blk846 * locals.var_q_d1_expnum__blk838_dn4))) + locals.var_q_aexp__blk824_dn4), ((((locals.var_q_d1_qi__blk847_dn6 * locals.var_q_expnum__blk837) + (locals.var_q_d1_qi__blk847 * locals.var_q_expnum__blk837_dn6)) + ((locals.var_q_qi_int__blk846_dn6 * locals.var_q_d1_expnum__blk838) + (locals.var_q_qi_int__blk846 * locals.var_q_d1_expnum__blk838_dn6))) + locals.var_q_aexp__blk824_dn6), ((((locals.var_q_d1_qi__blk847_dn7 * locals.var_q_expnum__blk837) + (locals.var_q_d1_qi__blk847 * locals.var_q_expnum__blk837_dn7)) + ((locals.var_q_qi_int__blk846_dn7 * locals.var_q_d1_expnum__blk838) + (locals.var_q_qi_int__blk846 * locals.var_q_d1_expnum__blk838_dn7))) + locals.var_q_aexp__blk824_dn7), ((((locals.var_q_d1_qi__blk847_dn8 * locals.var_q_expnum__blk837) + (locals.var_q_d1_qi__blk847 * locals.var_q_expnum__blk837_dn8)) + ((locals.var_q_qi_int__blk846_dn8 * locals.var_q_d1_expnum__blk838) + (locals.var_q_qi_int__blk846 * locals.var_q_d1_expnum__blk838_dn8))) + locals.var_q_aexp__blk824_dn8), ((((locals.var_q_d1_qi__blk847_dn9 * locals.var_q_expnum__blk837) + (locals.var_q_d1_qi__blk847 * locals.var_q_expnum__blk837_dn9)) + ((locals.var_q_qi_int__blk846_dn9 * locals.var_q_d1_expnum__blk838) + (locals.var_q_qi_int__blk846 * locals.var_q_d1_expnum__blk838_dn9))) + locals.var_q_aexp__blk824_dn9), );
            locals.var_q_d1_zero__blk850_rv = 0.0;
        }

        if (locals.var_guard1080 != 0.0) {
            let assign38720_e43765: f64 = (locals.var_q_d2_qi__blk848 * locals.var_q_expnum__blk837);
            let assign38720_e43768: f64 = (2.0 * locals.var_q_d1_qi__blk847);
            let assign38720_e43770: f64 = (assign38720_e43768 * locals.var_q_d1_expnum__blk838);
            let assign38720_e43771: f64 = (assign38720_e43765 + assign38720_e43770);
            let assign38720_e43774: f64 = (locals.var_q_qi_int__blk846 * locals.var_q_d2_expnum__blk839);
            let assign38720_e43775: f64 = (assign38720_e43771 + assign38720_e43774);
            let assign38720_e43777: f64 = (assign38720_e43775 - locals.var_q_aexp__blk824);
            (locals.var_q_d2_zero__blk851, locals.var_q_d2_zero__blk851_dn4, locals.var_q_d2_zero__blk851_dn6, locals.var_q_d2_zero__blk851_dn7, locals.var_q_d2_zero__blk851_dn8, locals.var_q_d2_zero__blk851_dn9, ) = (assign38720_e43777, (((((locals.var_q_d2_qi__blk848_dn4 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qi__blk848 * locals.var_q_expnum__blk837_dn4)) + (((2.0 * locals.var_q_d1_qi__blk847_dn4) * locals.var_q_d1_expnum__blk838) + (assign38720_e43768 * locals.var_q_d1_expnum__blk838_dn4))) + ((locals.var_q_qi_int__blk846_dn4 * locals.var_q_d2_expnum__blk839) + (locals.var_q_qi_int__blk846 * locals.var_q_d2_expnum__blk839_dn4))) - locals.var_q_aexp__blk824_dn4), (((((locals.var_q_d2_qi__blk848_dn6 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qi__blk848 * locals.var_q_expnum__blk837_dn6)) + (((2.0 * locals.var_q_d1_qi__blk847_dn6) * locals.var_q_d1_expnum__blk838) + (assign38720_e43768 * locals.var_q_d1_expnum__blk838_dn6))) + ((locals.var_q_qi_int__blk846_dn6 * locals.var_q_d2_expnum__blk839) + (locals.var_q_qi_int__blk846 * locals.var_q_d2_expnum__blk839_dn6))) - locals.var_q_aexp__blk824_dn6), (((((locals.var_q_d2_qi__blk848_dn7 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qi__blk848 * locals.var_q_expnum__blk837_dn7)) + (((2.0 * locals.var_q_d1_qi__blk847_dn7) * locals.var_q_d1_expnum__blk838) + (assign38720_e43768 * locals.var_q_d1_expnum__blk838_dn7))) + ((locals.var_q_qi_int__blk846_dn7 * locals.var_q_d2_expnum__blk839) + (locals.var_q_qi_int__blk846 * locals.var_q_d2_expnum__blk839_dn7))) - locals.var_q_aexp__blk824_dn7), (((((locals.var_q_d2_qi__blk848_dn8 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qi__blk848 * locals.var_q_expnum__blk837_dn8)) + (((2.0 * locals.var_q_d1_qi__blk847_dn8) * locals.var_q_d1_expnum__blk838) + (assign38720_e43768 * locals.var_q_d1_expnum__blk838_dn8))) + ((locals.var_q_qi_int__blk846_dn8 * locals.var_q_d2_expnum__blk839) + (locals.var_q_qi_int__blk846 * locals.var_q_d2_expnum__blk839_dn8))) - locals.var_q_aexp__blk824_dn8), (((((locals.var_q_d2_qi__blk848_dn9 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qi__blk848 * locals.var_q_expnum__blk837_dn9)) + (((2.0 * locals.var_q_d1_qi__blk847_dn9) * locals.var_q_d1_expnum__blk838) + (assign38720_e43768 * locals.var_q_d1_expnum__blk838_dn9))) + ((locals.var_q_qi_int__blk846_dn9 * locals.var_q_d2_expnum__blk839) + (locals.var_q_qi_int__blk846 * locals.var_q_d2_expnum__blk839_dn9))) - locals.var_q_aexp__blk824_dn9), );
            locals.var_q_d2_zero__blk851_rv = 0.0;
        }

        if (locals.var_guard1080 != 0.0) {
            let assign38730_e43783: f64 = (locals.var_q_d1_zero__blk850 * locals.var_q_d1_zero__blk850);
            let assign38730_e43786: f64 = (0.5 * locals.var_q_zero__blk849);
            let assign38730_e43788: f64 = (assign38730_e43786 * locals.var_q_d2_zero__blk851);
            let assign38730_e43789: f64 = (assign38730_e43783 - assign38730_e43788);
            (locals.var_q_temp__blk860, locals.var_q_temp__blk860_dn4, locals.var_q_temp__blk860_dn6, locals.var_q_temp__blk860_dn7, locals.var_q_temp__blk860_dn8, locals.var_q_temp__blk860_dn9, ) = (assign38730_e43789, (((locals.var_q_d1_zero__blk850_dn4 * locals.var_q_d1_zero__blk850) + (locals.var_q_d1_zero__blk850 * locals.var_q_d1_zero__blk850_dn4)) - (((0.5 * locals.var_q_zero__blk849_dn4) * locals.var_q_d2_zero__blk851) + (assign38730_e43786 * locals.var_q_d2_zero__blk851_dn4))), (((locals.var_q_d1_zero__blk850_dn6 * locals.var_q_d1_zero__blk850) + (locals.var_q_d1_zero__blk850 * locals.var_q_d1_zero__blk850_dn6)) - (((0.5 * locals.var_q_zero__blk849_dn6) * locals.var_q_d2_zero__blk851) + (assign38730_e43786 * locals.var_q_d2_zero__blk851_dn6))), (((locals.var_q_d1_zero__blk850_dn7 * locals.var_q_d1_zero__blk850) + (locals.var_q_d1_zero__blk850 * locals.var_q_d1_zero__blk850_dn7)) - (((0.5 * locals.var_q_zero__blk849_dn7) * locals.var_q_d2_zero__blk851) + (assign38730_e43786 * locals.var_q_d2_zero__blk851_dn7))), (((locals.var_q_d1_zero__blk850_dn8 * locals.var_q_d1_zero__blk850) + (locals.var_q_d1_zero__blk850 * locals.var_q_d1_zero__blk850_dn8)) - (((0.5 * locals.var_q_zero__blk849_dn8) * locals.var_q_d2_zero__blk851) + (assign38730_e43786 * locals.var_q_d2_zero__blk851_dn8))), (((locals.var_q_d1_zero__blk850_dn9 * locals.var_q_d1_zero__blk850) + (locals.var_q_d1_zero__blk850 * locals.var_q_d1_zero__blk850_dn9)) - (((0.5 * locals.var_q_zero__blk849_dn9) * locals.var_q_d2_zero__blk851) + (assign38730_e43786 * locals.var_q_d2_zero__blk851_dn9))), );
            locals.var_q_temp__blk860_rv = 0.0;
        }

        if (locals.var_guard1080 != 0.0) {
            let assign38740_e43794: f64 = (-locals.var_q_zero__blk849);
            let assign38740_e43796: f64 = (assign38740_e43794 * locals.var_q_d1_zero__blk850);
            let assign38740_e43798: f64 = (assign38740_e43796 * locals.var_q_temp__blk860);
            let assign38740_e43801: f64 = (locals.var_q_temp__blk860 * locals.var_q_temp__blk860);
            let assign38740_e43803: f64 = (assign38740_e43801 + 1e-200);
            let assign38740_e43804: f64 = (assign38740_e43798 / assign38740_e43803);
            (locals.var_q_eps2__blk852, locals.var_q_eps2__blk852_dn4, locals.var_q_eps2__blk852_dn6, locals.var_q_eps2__blk852_dn7, locals.var_q_eps2__blk852_dn8, locals.var_q_eps2__blk852_dn9, ) = (assign38740_e43804, ((((((((-locals.var_q_zero__blk849_dn4) * locals.var_q_d1_zero__blk850) + (assign38740_e43794 * locals.var_q_d1_zero__blk850_dn4)) * locals.var_q_temp__blk860) + (assign38740_e43796 * locals.var_q_temp__blk860_dn4)) * assign38740_e43803) - (assign38740_e43798 * ((locals.var_q_temp__blk860_dn4 * locals.var_q_temp__blk860) + (locals.var_q_temp__blk860 * locals.var_q_temp__blk860_dn4)))) / (assign38740_e43803 * assign38740_e43803)), ((((((((-locals.var_q_zero__blk849_dn6) * locals.var_q_d1_zero__blk850) + (assign38740_e43794 * locals.var_q_d1_zero__blk850_dn6)) * locals.var_q_temp__blk860) + (assign38740_e43796 * locals.var_q_temp__blk860_dn6)) * assign38740_e43803) - (assign38740_e43798 * ((locals.var_q_temp__blk860_dn6 * locals.var_q_temp__blk860) + (locals.var_q_temp__blk860 * locals.var_q_temp__blk860_dn6)))) / (assign38740_e43803 * assign38740_e43803)), ((((((((-locals.var_q_zero__blk849_dn7) * locals.var_q_d1_zero__blk850) + (assign38740_e43794 * locals.var_q_d1_zero__blk850_dn7)) * locals.var_q_temp__blk860) + (assign38740_e43796 * locals.var_q_temp__blk860_dn7)) * assign38740_e43803) - (assign38740_e43798 * ((locals.var_q_temp__blk860_dn7 * locals.var_q_temp__blk860) + (locals.var_q_temp__blk860 * locals.var_q_temp__blk860_dn7)))) / (assign38740_e43803 * assign38740_e43803)), ((((((((-locals.var_q_zero__blk849_dn8) * locals.var_q_d1_zero__blk850) + (assign38740_e43794 * locals.var_q_d1_zero__blk850_dn8)) * locals.var_q_temp__blk860) + (assign38740_e43796 * locals.var_q_temp__blk860_dn8)) * assign38740_e43803) - (assign38740_e43798 * ((locals.var_q_temp__blk860_dn8 * locals.var_q_temp__blk860) + (locals.var_q_temp__blk860 * locals.var_q_temp__blk860_dn8)))) / (assign38740_e43803 * assign38740_e43803)), ((((((((-locals.var_q_zero__blk849_dn9) * locals.var_q_d1_zero__blk850) + (assign38740_e43794 * locals.var_q_d1_zero__blk850_dn9)) * locals.var_q_temp__blk860) + (assign38740_e43796 * locals.var_q_temp__blk860_dn9)) * assign38740_e43803) - (assign38740_e43798 * ((locals.var_q_temp__blk860_dn9 * locals.var_q_temp__blk860) + (locals.var_q_temp__blk860 * locals.var_q_temp__blk860_dn9)))) / (assign38740_e43803 * assign38740_e43803)), );
            locals.var_q_eps2__blk852_rv = 0.0;
        }

        if (locals.var_guard1080 != 0.0) {
            let assign38750_e43810: f64 = (locals.var_q1d__blk1001 + locals.var_q_eps2__blk852);
            (locals.var_q1d__blk1001, locals.var_q1d__blk1001_dn4, locals.var_q1d__blk1001_dn6, locals.var_q1d__blk1001_dn7, locals.var_q1d__blk1001_dn8, locals.var_q1d__blk1001_dn9, ) = (assign38750_e43810, (locals.var_q1d__blk1001_dn4 + locals.var_q_eps2__blk852_dn4), (locals.var_q1d__blk1001_dn6 + locals.var_q_eps2__blk852_dn6), (locals.var_q1d__blk1001_dn7 + locals.var_q_eps2__blk852_dn7), (locals.var_q1d__blk1001_dn8 + locals.var_q_eps2__blk852_dn8), (locals.var_q1d__blk1001_dn9 + locals.var_q_eps2__blk852_dn9), );
            locals.var_q1d__blk1001_rv = 0.0;
        }

        let assign38760_e43815: f64 = if p.p10 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1196 = assign38760_e43815;
        locals.var_guard1196_rv = 0.0;

        let assign38770_e43817: f64 = (locals.var_q_eps2__blk852).abs();
        let assign38770_e43819: f64 = if assign38770_e43817 > 0.01 { 1.0 } else { 0.0 };
        locals.var_guard1197 = assign38770_e43819;
        locals.var_guard1197_rv = 0.0;

        if (((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) {
            let assign38780_e43827: f64 = (locals.var_k1__blk932 * locals.var_q1d__blk1001);
            (locals.var_q_k1q1__blk823, locals.var_q_k1q1__blk823_dn4, locals.var_q_k1q1__blk823_dn6, locals.var_q_k1q1__blk823_dn7, locals.var_q_k1q1__blk823_dn8, locals.var_q_k1q1__blk823_dn9, ) = (assign38780_e43827, ((locals.var_k1__blk932_dn4 * locals.var_q1d__blk1001) + (locals.var_k1__blk932 * locals.var_q1d__blk1001_dn4)), ((locals.var_k1__blk932_dn6 * locals.var_q1d__blk1001) + (locals.var_k1__blk932 * locals.var_q1d__blk1001_dn6)), ((locals.var_k1__blk932_dn7 * locals.var_q1d__blk1001) + (locals.var_k1__blk932 * locals.var_q1d__blk1001_dn7)), ((locals.var_k1__blk932_dn8 * locals.var_q1d__blk1001) + (locals.var_k1__blk932 * locals.var_q1d__blk1001_dn8)), ((locals.var_k1__blk932_dn9 * locals.var_q1d__blk1001) + (locals.var_k1__blk932 * locals.var_q1d__blk1001_dn9)), );
            locals.var_q_k1q1__blk823_rv = 0.0;
        }

        let assign38790_e43832: f64 = (locals.var_xg1x__blk930 - locals.var_q1d__blk1001);
        let assign38790_e43834: f64 = (assign38790_e43832 - locals.var_xdeff__blk1000);
        let assign38790_e43836: f64 = if assign38790_e43834 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard1198 = assign38790_e43836;
        locals.var_guard1198_rv = 0.0;

        if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1198 != 0.0)) {
            let assign38800_e43846: f64 = (locals.var_xg1x__blk930 - locals.var_q1d__blk1001);
            let assign38800_e43848: f64 = (assign38800_e43846 - locals.var_xdeff__blk1000);
            let assign38800_e43849: f64 = (assign38800_e43848).exp();
            (locals.var_q_temp1__blk814, locals.var_q_temp1__blk814_dn4, locals.var_q_temp1__blk814_dn6, locals.var_q_temp1__blk814_dn7, locals.var_q_temp1__blk814_dn8, locals.var_q_temp1__blk814_dn9, ) = (assign38800_e43849, (assign38800_e43849 * ((locals.var_xg1x__blk930_dn4 - locals.var_q1d__blk1001_dn4) - locals.var_xdeff__blk1000_dn4)), (assign38800_e43849 * ((locals.var_xg1x__blk930_dn6 - locals.var_q1d__blk1001_dn6) - locals.var_xdeff__blk1000_dn6)), (assign38800_e43849 * ((locals.var_xg1x__blk930_dn7 - locals.var_q1d__blk1001_dn7) - locals.var_xdeff__blk1000_dn7)), (assign38800_e43849 * ((locals.var_xg1x__blk930_dn8 - locals.var_q1d__blk1001_dn8) - locals.var_xdeff__blk1000_dn8)), (assign38800_e43849 * ((locals.var_xg1x__blk930_dn9 - locals.var_q1d__blk1001_dn9) - locals.var_xdeff__blk1000_dn9)), );
            locals.var_q_temp1__blk814_rv = 0.0;
        }

        if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1198 == 0.0)) {
            let assign38810_e43864: f64 = (locals.var_xg1x__blk930 - locals.var_q1d__blk1001);
            let assign38810_e43866: f64 = (assign38810_e43864 - locals.var_xdeff__blk1000);
            let assign38810_e43868: f64 = (assign38810_e43866 - 80.0);
            let assign38810_e43873: f64 = (locals.var_xg1x__blk930 - locals.var_q1d__blk1001);
            let assign38810_e43875: f64 = (assign38810_e43873 - locals.var_xdeff__blk1000);
            let assign38810_e43877: f64 = (assign38810_e43875 - 80.0);
            let assign38810_e43878: f64 = (0.5 * assign38810_e43877);
            let assign38810_e43882: f64 = (locals.var_xg1x__blk930 - locals.var_q1d__blk1001);
            let assign38810_e43884: f64 = (assign38810_e43882 - locals.var_xdeff__blk1000);
            let assign38810_e43886: f64 = (assign38810_e43884 - 80.0);
            let assign38810_e43888: f64 = (assign38810_e43886 * 0.3333333333333);
            let assign38810_e43889: f64 = (1.0 + assign38810_e43888);
            let assign38810_e43890: f64 = (assign38810_e43878 * assign38810_e43889);
            let assign38810_e43891: f64 = (1.0 + assign38810_e43890);
            let assign38810_e43892: f64 = (assign38810_e43868 * assign38810_e43891);
            let assign38810_e43893: f64 = (1.0 + assign38810_e43892);
            let assign38810_e43894: f64 = (5.54062e34 * assign38810_e43893);
            (locals.var_q_temp1__blk814, locals.var_q_temp1__blk814_dn4, locals.var_q_temp1__blk814_dn6, locals.var_q_temp1__blk814_dn7, locals.var_q_temp1__blk814_dn8, locals.var_q_temp1__blk814_dn9, ) = (assign38810_e43894, (5.54062e34 * ((((locals.var_xg1x__blk930_dn4 - locals.var_q1d__blk1001_dn4) - locals.var_xdeff__blk1000_dn4) * assign38810_e43891) + (assign38810_e43868 * (((0.5 * ((locals.var_xg1x__blk930_dn4 - locals.var_q1d__blk1001_dn4) - locals.var_xdeff__blk1000_dn4)) * assign38810_e43889) + (assign38810_e43878 * (((locals.var_xg1x__blk930_dn4 - locals.var_q1d__blk1001_dn4) - locals.var_xdeff__blk1000_dn4) * 0.3333333333333)))))), (5.54062e34 * ((((locals.var_xg1x__blk930_dn6 - locals.var_q1d__blk1001_dn6) - locals.var_xdeff__blk1000_dn6) * assign38810_e43891) + (assign38810_e43868 * (((0.5 * ((locals.var_xg1x__blk930_dn6 - locals.var_q1d__blk1001_dn6) - locals.var_xdeff__blk1000_dn6)) * assign38810_e43889) + (assign38810_e43878 * (((locals.var_xg1x__blk930_dn6 - locals.var_q1d__blk1001_dn6) - locals.var_xdeff__blk1000_dn6) * 0.3333333333333)))))), (5.54062e34 * ((((locals.var_xg1x__blk930_dn7 - locals.var_q1d__blk1001_dn7) - locals.var_xdeff__blk1000_dn7) * assign38810_e43891) + (assign38810_e43868 * (((0.5 * ((locals.var_xg1x__blk930_dn7 - locals.var_q1d__blk1001_dn7) - locals.var_xdeff__blk1000_dn7)) * assign38810_e43889) + (assign38810_e43878 * (((locals.var_xg1x__blk930_dn7 - locals.var_q1d__blk1001_dn7) - locals.var_xdeff__blk1000_dn7) * 0.3333333333333)))))), (5.54062e34 * ((((locals.var_xg1x__blk930_dn8 - locals.var_q1d__blk1001_dn8) - locals.var_xdeff__blk1000_dn8) * assign38810_e43891) + (assign38810_e43868 * (((0.5 * ((locals.var_xg1x__blk930_dn8 - locals.var_q1d__blk1001_dn8) - locals.var_xdeff__blk1000_dn8)) * assign38810_e43889) + (assign38810_e43878 * (((locals.var_xg1x__blk930_dn8 - locals.var_q1d__blk1001_dn8) - locals.var_xdeff__blk1000_dn8) * 0.3333333333333)))))), (5.54062e34 * ((((locals.var_xg1x__blk930_dn9 - locals.var_q1d__blk1001_dn9) - locals.var_xdeff__blk1000_dn9) * assign38810_e43891) + (assign38810_e43868 * (((0.5 * ((locals.var_xg1x__blk930_dn9 - locals.var_q1d__blk1001_dn9) - locals.var_xdeff__blk1000_dn9)) * assign38810_e43889) + (assign38810_e43878 * (((locals.var_xg1x__blk930_dn9 - locals.var_q1d__blk1001_dn9) - locals.var_xdeff__blk1000_dn9) * 0.3333333333333)))))), );
            locals.var_q_temp1__blk814_rv = 0.0;
        }

        if (((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) {
            let assign38820_e43904: f64 = (locals.var_a0__blk905 * locals.var_q_temp1__blk814);
            (locals.var_q_aexp__blk824, locals.var_q_aexp__blk824_dn4, locals.var_q_aexp__blk824_dn6, locals.var_q_aexp__blk824_dn7, locals.var_q_aexp__blk824_dn8, locals.var_q_aexp__blk824_dn9, ) = (assign38820_e43904, ((locals.var_a0__blk905_dn4 * locals.var_q_temp1__blk814) + (locals.var_a0__blk905 * locals.var_q_temp1__blk814_dn4)), ((locals.var_a0__blk905_dn6 * locals.var_q_temp1__blk814) + (locals.var_a0__blk905 * locals.var_q_temp1__blk814_dn6)), ((locals.var_a0__blk905_dn7 * locals.var_q_temp1__blk814) + (locals.var_a0__blk905 * locals.var_q_temp1__blk814_dn7)), ((locals.var_a0__blk905_dn8 * locals.var_q_temp1__blk814) + (locals.var_a0__blk905 * locals.var_q_temp1__blk814_dn8)), ((locals.var_a0__blk905_dn9 * locals.var_q_temp1__blk814) + (locals.var_a0__blk905 * locals.var_q_temp1__blk814_dn9)), );
            locals.var_q_aexp__blk824_rv = 0.0;
        }

        if (((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) {
            let assign38830_e43914: f64 = (locals.var_q_k1q1__blk823 * locals.var_q_k1q1__blk823);
            let assign38830_e43916: f64 = (assign38830_e43914 - locals.var_q_aexp__blk824);
            (locals.var_q_qsq__blk825, locals.var_q_qsq__blk825_dn4, locals.var_q_qsq__blk825_dn6, locals.var_q_qsq__blk825_dn7, locals.var_q_qsq__blk825_dn8, locals.var_q_qsq__blk825_dn9, ) = (assign38830_e43916, (((locals.var_q_k1q1__blk823_dn4 * locals.var_q_k1q1__blk823) + (locals.var_q_k1q1__blk823 * locals.var_q_k1q1__blk823_dn4)) - locals.var_q_aexp__blk824_dn4), (((locals.var_q_k1q1__blk823_dn6 * locals.var_q_k1q1__blk823) + (locals.var_q_k1q1__blk823 * locals.var_q_k1q1__blk823_dn6)) - locals.var_q_aexp__blk824_dn6), (((locals.var_q_k1q1__blk823_dn7 * locals.var_q_k1q1__blk823) + (locals.var_q_k1q1__blk823 * locals.var_q_k1q1__blk823_dn7)) - locals.var_q_aexp__blk824_dn7), (((locals.var_q_k1q1__blk823_dn8 * locals.var_q_k1q1__blk823) + (locals.var_q_k1q1__blk823 * locals.var_q_k1q1__blk823_dn8)) - locals.var_q_aexp__blk824_dn8), (((locals.var_q_k1q1__blk823_dn9 * locals.var_q_k1q1__blk823) + (locals.var_q_k1q1__blk823 * locals.var_q_k1q1__blk823_dn9)) - locals.var_q_aexp__blk824_dn9), );
            locals.var_q_qsq__blk825_rv = 0.0;
        }

        if (((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) {
            let assign38840_e43926: f64 = (2.0 * locals.var_k1__blk932);
            let assign38840_e43928: f64 = (assign38840_e43926 * locals.var_q_k1q1__blk823);
            let assign38840_e43930: f64 = (assign38840_e43928 + locals.var_q_aexp__blk824);
            (locals.var_q_d1_qsq__blk826, locals.var_q_d1_qsq__blk826_dn4, locals.var_q_d1_qsq__blk826_dn6, locals.var_q_d1_qsq__blk826_dn7, locals.var_q_d1_qsq__blk826_dn8, locals.var_q_d1_qsq__blk826_dn9, ) = (assign38840_e43930, ((((2.0 * locals.var_k1__blk932_dn4) * locals.var_q_k1q1__blk823) + (assign38840_e43926 * locals.var_q_k1q1__blk823_dn4)) + locals.var_q_aexp__blk824_dn4), ((((2.0 * locals.var_k1__blk932_dn6) * locals.var_q_k1q1__blk823) + (assign38840_e43926 * locals.var_q_k1q1__blk823_dn6)) + locals.var_q_aexp__blk824_dn6), ((((2.0 * locals.var_k1__blk932_dn7) * locals.var_q_k1q1__blk823) + (assign38840_e43926 * locals.var_q_k1q1__blk823_dn7)) + locals.var_q_aexp__blk824_dn7), ((((2.0 * locals.var_k1__blk932_dn8) * locals.var_q_k1q1__blk823) + (assign38840_e43926 * locals.var_q_k1q1__blk823_dn8)) + locals.var_q_aexp__blk824_dn8), ((((2.0 * locals.var_k1__blk932_dn9) * locals.var_q_k1q1__blk823) + (assign38840_e43926 * locals.var_q_k1q1__blk823_dn9)) + locals.var_q_aexp__blk824_dn9), );
            locals.var_q_d1_qsq__blk826_rv = 0.0;
        }

        if (((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) {
            let assign38850_e43940: f64 = (2.0 * locals.var_k1__blk932);
            let assign38850_e43942: f64 = (assign38850_e43940 * locals.var_k1__blk932);
            let assign38850_e43944: f64 = (assign38850_e43942 - locals.var_q_aexp__blk824);
            (locals.var_q_d2_qsq__blk827, locals.var_q_d2_qsq__blk827_dn4, locals.var_q_d2_qsq__blk827_dn6, locals.var_q_d2_qsq__blk827_dn7, locals.var_q_d2_qsq__blk827_dn8, locals.var_q_d2_qsq__blk827_dn9, ) = (assign38850_e43944, ((((2.0 * locals.var_k1__blk932_dn4) * locals.var_k1__blk932) + (assign38850_e43940 * locals.var_k1__blk932_dn4)) - locals.var_q_aexp__blk824_dn4), ((((2.0 * locals.var_k1__blk932_dn6) * locals.var_k1__blk932) + (assign38850_e43940 * locals.var_k1__blk932_dn6)) - locals.var_q_aexp__blk824_dn6), ((((2.0 * locals.var_k1__blk932_dn7) * locals.var_k1__blk932) + (assign38850_e43940 * locals.var_k1__blk932_dn7)) - locals.var_q_aexp__blk824_dn7), ((((2.0 * locals.var_k1__blk932_dn8) * locals.var_k1__blk932) + (assign38850_e43940 * locals.var_k1__blk932_dn8)) - locals.var_q_aexp__blk824_dn8), ((((2.0 * locals.var_k1__blk932_dn9) * locals.var_k1__blk932) + (assign38850_e43940 * locals.var_k1__blk932_dn9)) - locals.var_q_aexp__blk824_dn9), );
            locals.var_q_d2_qsq__blk827_rv = 0.0;
        }

        let assign38860_e43949: f64 = (-0.005);
        let assign38860_e43950: f64 = if locals.var_q_qsq__blk825 < assign38860_e43949 { 1.0 } else { 0.0 };
        locals.var_guard1199 = assign38860_e43950;
        locals.var_guard1199_rv = 0.0;

        if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1199 != 0.0)) {
            let assign38870_e43959: f64 = (locals.var_q_qsq__blk825).abs();
            let assign38870_e43960: f64 = (assign38870_e43959).sqrt();
            (locals.var_q_rac_qsq__blk828, locals.var_q_rac_qsq__blk828_dn4, locals.var_q_rac_qsq__blk828_dn6, locals.var_q_rac_qsq__blk828_dn7, locals.var_q_rac_qsq__blk828_dn8, locals.var_q_rac_qsq__blk828_dn9, ) = (assign38870_e43960, (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn4 } else { (-locals.var_q_qsq__blk825_dn4) } / (2.0 * assign38870_e43960)), (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn6 } else { (-locals.var_q_qsq__blk825_dn6) } / (2.0 * assign38870_e43960)), (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn7 } else { (-locals.var_q_qsq__blk825_dn7) } / (2.0 * assign38870_e43960)), (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn8 } else { (-locals.var_q_qsq__blk825_dn8) } / (2.0 * assign38870_e43960)), (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn9 } else { (-locals.var_q_qsq__blk825_dn9) } / (2.0 * assign38870_e43960)), );
            locals.var_q_rac_qsq__blk828_rv = 0.0;
        }

        if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1199 != 0.0)) {
            let assign38880_e43973: f64 = (0.5 * locals.var_q_rac_qsq__blk828);
            let assign38880_e43974: f64 = (assign38880_e43973).tan();
            let assign38880_e43975: f64 = (locals.var_q_rac_qsq__blk828 / assign38880_e43974);
            (locals.var_q_qcoth__blk829, locals.var_q_qcoth__blk829_dn4, locals.var_q_qcoth__blk829_dn6, locals.var_q_qcoth__blk829_dn7, locals.var_q_qcoth__blk829_dn8, locals.var_q_qcoth__blk829_dn9, ) = (assign38880_e43975, (((locals.var_q_rac_qsq__blk828_dn4 * assign38880_e43974) - (locals.var_q_rac_qsq__blk828 * ((0.5 * locals.var_q_rac_qsq__blk828_dn4) / ((assign38880_e43973).cos() * (assign38880_e43973).cos())))) / (assign38880_e43974 * assign38880_e43974)), (((locals.var_q_rac_qsq__blk828_dn6 * assign38880_e43974) - (locals.var_q_rac_qsq__blk828 * ((0.5 * locals.var_q_rac_qsq__blk828_dn6) / ((assign38880_e43973).cos() * (assign38880_e43973).cos())))) / (assign38880_e43974 * assign38880_e43974)), (((locals.var_q_rac_qsq__blk828_dn7 * assign38880_e43974) - (locals.var_q_rac_qsq__blk828 * ((0.5 * locals.var_q_rac_qsq__blk828_dn7) / ((assign38880_e43973).cos() * (assign38880_e43973).cos())))) / (assign38880_e43974 * assign38880_e43974)), (((locals.var_q_rac_qsq__blk828_dn8 * assign38880_e43974) - (locals.var_q_rac_qsq__blk828 * ((0.5 * locals.var_q_rac_qsq__blk828_dn8) / ((assign38880_e43973).cos() * (assign38880_e43973).cos())))) / (assign38880_e43974 * assign38880_e43974)), (((locals.var_q_rac_qsq__blk828_dn9 * assign38880_e43974) - (locals.var_q_rac_qsq__blk828 * ((0.5 * locals.var_q_rac_qsq__blk828_dn9) / ((assign38880_e43973).cos() * (assign38880_e43973).cos())))) / (assign38880_e43974 * assign38880_e43974)), );
            locals.var_q_qcoth__blk829_rv = 0.0;
        }

        if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1199 != 0.0)) {
            let assign38890_e43987: f64 = (0.25 * locals.var_q_d1_qsq__blk826);
            let assign38890_e43989: f64 = (assign38890_e43987 / locals.var_q_qsq__blk825);
            (locals.var_q_temp1__blk814, locals.var_q_temp1__blk814_dn4, locals.var_q_temp1__blk814_dn6, locals.var_q_temp1__blk814_dn7, locals.var_q_temp1__blk814_dn8, locals.var_q_temp1__blk814_dn9, ) = (assign38890_e43989, ((((0.25 * locals.var_q_d1_qsq__blk826_dn4) * locals.var_q_qsq__blk825) - (assign38890_e43987 * locals.var_q_qsq__blk825_dn4)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((0.25 * locals.var_q_d1_qsq__blk826_dn6) * locals.var_q_qsq__blk825) - (assign38890_e43987 * locals.var_q_qsq__blk825_dn6)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((0.25 * locals.var_q_d1_qsq__blk826_dn7) * locals.var_q_qsq__blk825) - (assign38890_e43987 * locals.var_q_qsq__blk825_dn7)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((0.25 * locals.var_q_d1_qsq__blk826_dn8) * locals.var_q_qsq__blk825) - (assign38890_e43987 * locals.var_q_qsq__blk825_dn8)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((0.25 * locals.var_q_d1_qsq__blk826_dn9) * locals.var_q_qsq__blk825) - (assign38890_e43987 * locals.var_q_qsq__blk825_dn9)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), );
            locals.var_q_temp1__blk814_rv = 0.0;
        }

        if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1199 != 0.0)) {
            let assign38900_e44003: f64 = (2.0 - locals.var_q_qcoth__blk829);
            let assign38900_e44004: f64 = (locals.var_q_qcoth__blk829 * assign38900_e44003);
            let assign38900_e44005: f64 = (locals.var_q_qsq__blk825 + assign38900_e44004);
            let assign38900_e44007: f64 = (assign38900_e44005 * locals.var_q_temp1__blk814);
            (locals.var_q_d1_qcoth__blk830, locals.var_q_d1_qcoth__blk830_dn4, locals.var_q_d1_qcoth__blk830_dn6, locals.var_q_d1_qcoth__blk830_dn7, locals.var_q_d1_qcoth__blk830_dn8, locals.var_q_d1_qcoth__blk830_dn9, ) = (assign38900_e44007, (((locals.var_q_qsq__blk825_dn4 + ((locals.var_q_qcoth__blk829_dn4 * assign38900_e44003) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn4)))) * locals.var_q_temp1__blk814) + (assign38900_e44005 * locals.var_q_temp1__blk814_dn4)), (((locals.var_q_qsq__blk825_dn6 + ((locals.var_q_qcoth__blk829_dn6 * assign38900_e44003) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn6)))) * locals.var_q_temp1__blk814) + (assign38900_e44005 * locals.var_q_temp1__blk814_dn6)), (((locals.var_q_qsq__blk825_dn7 + ((locals.var_q_qcoth__blk829_dn7 * assign38900_e44003) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn7)))) * locals.var_q_temp1__blk814) + (assign38900_e44005 * locals.var_q_temp1__blk814_dn7)), (((locals.var_q_qsq__blk825_dn8 + ((locals.var_q_qcoth__blk829_dn8 * assign38900_e44003) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn8)))) * locals.var_q_temp1__blk814) + (assign38900_e44005 * locals.var_q_temp1__blk814_dn8)), (((locals.var_q_qsq__blk825_dn9 + ((locals.var_q_qcoth__blk829_dn9 * assign38900_e44003) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn9)))) * locals.var_q_temp1__blk814) + (assign38900_e44005 * locals.var_q_temp1__blk814_dn9)), );
            locals.var_q_d1_qcoth__blk830_rv = 0.0;
        }

        if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1199 != 0.0)) {
            let assign38910_e44020: f64 = (2.0 * locals.var_q_d1_qcoth__blk830);
            let assign38910_e44023: f64 = (1.0 + locals.var_q_qcoth__blk829);
            let assign38910_e44024: f64 = (assign38910_e44020 * assign38910_e44023);
            let assign38910_e44025: f64 = (locals.var_q_d1_qsq__blk826 - assign38910_e44024);
            let assign38910_e44027: f64 = (assign38910_e44025 * locals.var_q_temp1__blk814);
            let assign38910_e44030: f64 = (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827);
            let assign38910_e44032: f64 = (assign38910_e44030 / locals.var_q_d1_qsq__blk826);
            let assign38910_e44033: f64 = (assign38910_e44027 + assign38910_e44032);
            (locals.var_q_d2_qcoth__blk832, locals.var_q_d2_qcoth__blk832_dn4, locals.var_q_d2_qcoth__blk832_dn6, locals.var_q_d2_qcoth__blk832_dn7, locals.var_q_d2_qcoth__blk832_dn8, locals.var_q_d2_qcoth__blk832_dn9, ) = (assign38910_e44033, ((((locals.var_q_d1_qsq__blk826_dn4 - (((2.0 * locals.var_q_d1_qcoth__blk830_dn4) * assign38910_e44023) + (assign38910_e44020 * locals.var_q_qcoth__blk829_dn4))) * locals.var_q_temp1__blk814) + (assign38910_e44025 * locals.var_q_temp1__blk814_dn4)) + (((((locals.var_q_d1_qcoth__blk830_dn4 * locals.var_q_d2_qsq__blk827) + (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827_dn4)) * locals.var_q_d1_qsq__blk826) - (assign38910_e44030 * locals.var_q_d1_qsq__blk826_dn4)) / (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826))), ((((locals.var_q_d1_qsq__blk826_dn6 - (((2.0 * locals.var_q_d1_qcoth__blk830_dn6) * assign38910_e44023) + (assign38910_e44020 * locals.var_q_qcoth__blk829_dn6))) * locals.var_q_temp1__blk814) + (assign38910_e44025 * locals.var_q_temp1__blk814_dn6)) + (((((locals.var_q_d1_qcoth__blk830_dn6 * locals.var_q_d2_qsq__blk827) + (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827_dn6)) * locals.var_q_d1_qsq__blk826) - (assign38910_e44030 * locals.var_q_d1_qsq__blk826_dn6)) / (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826))), ((((locals.var_q_d1_qsq__blk826_dn7 - (((2.0 * locals.var_q_d1_qcoth__blk830_dn7) * assign38910_e44023) + (assign38910_e44020 * locals.var_q_qcoth__blk829_dn7))) * locals.var_q_temp1__blk814) + (assign38910_e44025 * locals.var_q_temp1__blk814_dn7)) + (((((locals.var_q_d1_qcoth__blk830_dn7 * locals.var_q_d2_qsq__blk827) + (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827_dn7)) * locals.var_q_d1_qsq__blk826) - (assign38910_e44030 * locals.var_q_d1_qsq__blk826_dn7)) / (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826))), ((((locals.var_q_d1_qsq__blk826_dn8 - (((2.0 * locals.var_q_d1_qcoth__blk830_dn8) * assign38910_e44023) + (assign38910_e44020 * locals.var_q_qcoth__blk829_dn8))) * locals.var_q_temp1__blk814) + (assign38910_e44025 * locals.var_q_temp1__blk814_dn8)) + (((((locals.var_q_d1_qcoth__blk830_dn8 * locals.var_q_d2_qsq__blk827) + (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827_dn8)) * locals.var_q_d1_qsq__blk826) - (assign38910_e44030 * locals.var_q_d1_qsq__blk826_dn8)) / (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826))), ((((locals.var_q_d1_qsq__blk826_dn9 - (((2.0 * locals.var_q_d1_qcoth__blk830_dn9) * assign38910_e44023) + (assign38910_e44020 * locals.var_q_qcoth__blk829_dn9))) * locals.var_q_temp1__blk814) + (assign38910_e44025 * locals.var_q_temp1__blk814_dn9)) + (((((locals.var_q_d1_qcoth__blk830_dn9 * locals.var_q_d2_qsq__blk827) + (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827_dn9)) * locals.var_q_d1_qsq__blk826) - (assign38910_e44030 * locals.var_q_d1_qsq__blk826_dn9)) / (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826))), );
            locals.var_q_d2_qcoth__blk832_rv = 0.0;
        }

        if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1199 != 0.0)) {
            let assign38920_e44046: f64 = (0.5 * locals.var_q_qcoth__blk829);
            let assign38920_e44047: f64 = (1.0 - assign38920_e44046);
            (locals.var_q_temp2__blk815, locals.var_q_temp2__blk815_dn4, locals.var_q_temp2__blk815_dn6, locals.var_q_temp2__blk815_dn7, locals.var_q_temp2__blk815_dn8, locals.var_q_temp2__blk815_dn9, ) = (assign38920_e44047, (-(0.5 * locals.var_q_qcoth__blk829_dn4)), (-(0.5 * locals.var_q_qcoth__blk829_dn6)), (-(0.5 * locals.var_q_qcoth__blk829_dn7)), (-(0.5 * locals.var_q_qcoth__blk829_dn8)), (-(0.5 * locals.var_q_qcoth__blk829_dn9)), );
            locals.var_q_temp2__blk815_rv = 0.0;
        }

        if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1199 != 0.0)) {
            let assign38930_e44059: f64 = (locals.var_q_d1_qsq__blk826 / locals.var_q_qsq__blk825);
            let assign38930_e44061: f64 = (assign38930_e44059 * locals.var_q_temp2__blk815);
            (locals.var_q_d1_ln__blk835, locals.var_q_d1_ln__blk835_dn4, locals.var_q_d1_ln__blk835_dn6, locals.var_q_d1_ln__blk835_dn7, locals.var_q_d1_ln__blk835_dn8, locals.var_q_d1_ln__blk835_dn9, ) = (assign38930_e44061, (((((locals.var_q_d1_qsq__blk826_dn4 * locals.var_q_qsq__blk825) - (locals.var_q_d1_qsq__blk826 * locals.var_q_qsq__blk825_dn4)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)) * locals.var_q_temp2__blk815) + (assign38930_e44059 * locals.var_q_temp2__blk815_dn4)), (((((locals.var_q_d1_qsq__blk826_dn6 * locals.var_q_qsq__blk825) - (locals.var_q_d1_qsq__blk826 * locals.var_q_qsq__blk825_dn6)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)) * locals.var_q_temp2__blk815) + (assign38930_e44059 * locals.var_q_temp2__blk815_dn6)), (((((locals.var_q_d1_qsq__blk826_dn7 * locals.var_q_qsq__blk825) - (locals.var_q_d1_qsq__blk826 * locals.var_q_qsq__blk825_dn7)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)) * locals.var_q_temp2__blk815) + (assign38930_e44059 * locals.var_q_temp2__blk815_dn7)), (((((locals.var_q_d1_qsq__blk826_dn8 * locals.var_q_qsq__blk825) - (locals.var_q_d1_qsq__blk826 * locals.var_q_qsq__blk825_dn8)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)) * locals.var_q_temp2__blk815) + (assign38930_e44059 * locals.var_q_temp2__blk815_dn8)), (((((locals.var_q_d1_qsq__blk826_dn9 * locals.var_q_qsq__blk825) - (locals.var_q_d1_qsq__blk826 * locals.var_q_qsq__blk825_dn9)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)) * locals.var_q_temp2__blk815) + (assign38930_e44059 * locals.var_q_temp2__blk815_dn9)), );
            locals.var_q_d1_ln__blk835_rv = 0.0;
        }

        if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1199 != 0.0)) {
            let assign38940_e44073: f64 = (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815);
            let assign38940_e44078: f64 = (0.5 * locals.var_q_d1_qcoth__blk830);
            let assign38940_e44079: f64 = (locals.var_q_d1_ln__blk835 + assign38940_e44078);
            let assign38940_e44080: f64 = (locals.var_q_d1_qsq__blk826 * assign38940_e44079);
            let assign38940_e44081: f64 = (assign38940_e44073 - assign38940_e44080);
            let assign38940_e44083: f64 = (assign38940_e44081 / locals.var_q_qsq__blk825);
            (locals.var_q_d2_ln__blk836, locals.var_q_d2_ln__blk836_dn4, locals.var_q_d2_ln__blk836_dn6, locals.var_q_d2_ln__blk836_dn7, locals.var_q_d2_ln__blk836_dn8, locals.var_q_d2_ln__blk836_dn9, ) = (assign38940_e44083, ((((((locals.var_q_d2_qsq__blk827_dn4 * locals.var_q_temp2__blk815) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815_dn4)) - ((locals.var_q_d1_qsq__blk826_dn4 * assign38940_e44079) + (locals.var_q_d1_qsq__blk826 * (locals.var_q_d1_ln__blk835_dn4 + (0.5 * locals.var_q_d1_qcoth__blk830_dn4))))) * locals.var_q_qsq__blk825) - (assign38940_e44081 * locals.var_q_qsq__blk825_dn4)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((((locals.var_q_d2_qsq__blk827_dn6 * locals.var_q_temp2__blk815) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815_dn6)) - ((locals.var_q_d1_qsq__blk826_dn6 * assign38940_e44079) + (locals.var_q_d1_qsq__blk826 * (locals.var_q_d1_ln__blk835_dn6 + (0.5 * locals.var_q_d1_qcoth__blk830_dn6))))) * locals.var_q_qsq__blk825) - (assign38940_e44081 * locals.var_q_qsq__blk825_dn6)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((((locals.var_q_d2_qsq__blk827_dn7 * locals.var_q_temp2__blk815) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815_dn7)) - ((locals.var_q_d1_qsq__blk826_dn7 * assign38940_e44079) + (locals.var_q_d1_qsq__blk826 * (locals.var_q_d1_ln__blk835_dn7 + (0.5 * locals.var_q_d1_qcoth__blk830_dn7))))) * locals.var_q_qsq__blk825) - (assign38940_e44081 * locals.var_q_qsq__blk825_dn7)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((((locals.var_q_d2_qsq__blk827_dn8 * locals.var_q_temp2__blk815) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815_dn8)) - ((locals.var_q_d1_qsq__blk826_dn8 * assign38940_e44079) + (locals.var_q_d1_qsq__blk826 * (locals.var_q_d1_ln__blk835_dn8 + (0.5 * locals.var_q_d1_qcoth__blk830_dn8))))) * locals.var_q_qsq__blk825) - (assign38940_e44081 * locals.var_q_qsq__blk825_dn8)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((((locals.var_q_d2_qsq__blk827_dn9 * locals.var_q_temp2__blk815) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815_dn9)) - ((locals.var_q_d1_qsq__blk826_dn9 * assign38940_e44079) + (locals.var_q_d1_qsq__blk826 * (locals.var_q_d1_ln__blk835_dn9 + (0.5 * locals.var_q_d1_qcoth__blk830_dn9))))) * locals.var_q_qsq__blk825) - (assign38940_e44081 * locals.var_q_qsq__blk825_dn9)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), );
            locals.var_q_d2_ln__blk836_rv = 0.0;
        }

        let assign38950_e44088: f64 = if locals.var_q_qsq__blk825 > 0.005 { 1.0 } else { 0.0 };
        locals.var_guard1200 = assign38950_e44088;
        locals.var_guard1200_rv = 0.0;

        if (((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1199 == 0.0)) && (locals.var_guard1200 != 0.0)) {
            let assign38960_e44100: f64 = (locals.var_q_qsq__blk825).abs();
            let assign38960_e44101: f64 = (assign38960_e44100).sqrt();
            (locals.var_q_rac_qsq__blk828, locals.var_q_rac_qsq__blk828_dn4, locals.var_q_rac_qsq__blk828_dn6, locals.var_q_rac_qsq__blk828_dn7, locals.var_q_rac_qsq__blk828_dn8, locals.var_q_rac_qsq__blk828_dn9, ) = (assign38960_e44101, (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn4 } else { (-locals.var_q_qsq__blk825_dn4) } / (2.0 * assign38960_e44101)), (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn6 } else { (-locals.var_q_qsq__blk825_dn6) } / (2.0 * assign38960_e44101)), (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn7 } else { (-locals.var_q_qsq__blk825_dn7) } / (2.0 * assign38960_e44101)), (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn8 } else { (-locals.var_q_qsq__blk825_dn8) } / (2.0 * assign38960_e44101)), (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn9 } else { (-locals.var_q_qsq__blk825_dn9) } / (2.0 * assign38960_e44101)), );
            locals.var_q_rac_qsq__blk828_rv = 0.0;
        }

        if (((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1199 == 0.0)) && (locals.var_guard1200 != 0.0)) {
            let assign38970_e44115: f64 = (-locals.var_q_rac_qsq__blk828);
            let assign38970_e44116: f64 = (assign38970_e44115).exp();
            (locals.var_q_invexpq__blk831, locals.var_q_invexpq__blk831_dn4, locals.var_q_invexpq__blk831_dn6, locals.var_q_invexpq__blk831_dn7, locals.var_q_invexpq__blk831_dn8, locals.var_q_invexpq__blk831_dn9, ) = (assign38970_e44116, (assign38970_e44116 * (-locals.var_q_rac_qsq__blk828_dn4)), (assign38970_e44116 * (-locals.var_q_rac_qsq__blk828_dn6)), (assign38970_e44116 * (-locals.var_q_rac_qsq__blk828_dn7)), (assign38970_e44116 * (-locals.var_q_rac_qsq__blk828_dn8)), (assign38970_e44116 * (-locals.var_q_rac_qsq__blk828_dn9)), );
            locals.var_q_invexpq__blk831_rv = 0.0;
        }

        if (((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1199 == 0.0)) && (locals.var_guard1200 != 0.0)) {
            let assign38980_e44132: f64 = (1.0 + locals.var_q_invexpq__blk831);
            let assign38980_e44133: f64 = (locals.var_q_rac_qsq__blk828 * assign38980_e44132);
            let assign38980_e44136: f64 = (1.0 - locals.var_q_invexpq__blk831);
            let assign38980_e44137: f64 = (assign38980_e44133 / assign38980_e44136);
            (locals.var_q_qcoth__blk829, locals.var_q_qcoth__blk829_dn4, locals.var_q_qcoth__blk829_dn6, locals.var_q_qcoth__blk829_dn7, locals.var_q_qcoth__blk829_dn8, locals.var_q_qcoth__blk829_dn9, ) = (assign38980_e44137, (((((locals.var_q_rac_qsq__blk828_dn4 * assign38980_e44132) + (locals.var_q_rac_qsq__blk828 * locals.var_q_invexpq__blk831_dn4)) * assign38980_e44136) - (assign38980_e44133 * (-locals.var_q_invexpq__blk831_dn4))) / (assign38980_e44136 * assign38980_e44136)), (((((locals.var_q_rac_qsq__blk828_dn6 * assign38980_e44132) + (locals.var_q_rac_qsq__blk828 * locals.var_q_invexpq__blk831_dn6)) * assign38980_e44136) - (assign38980_e44133 * (-locals.var_q_invexpq__blk831_dn6))) / (assign38980_e44136 * assign38980_e44136)), (((((locals.var_q_rac_qsq__blk828_dn7 * assign38980_e44132) + (locals.var_q_rac_qsq__blk828 * locals.var_q_invexpq__blk831_dn7)) * assign38980_e44136) - (assign38980_e44133 * (-locals.var_q_invexpq__blk831_dn7))) / (assign38980_e44136 * assign38980_e44136)), (((((locals.var_q_rac_qsq__blk828_dn8 * assign38980_e44132) + (locals.var_q_rac_qsq__blk828 * locals.var_q_invexpq__blk831_dn8)) * assign38980_e44136) - (assign38980_e44133 * (-locals.var_q_invexpq__blk831_dn8))) / (assign38980_e44136 * assign38980_e44136)), (((((locals.var_q_rac_qsq__blk828_dn9 * assign38980_e44132) + (locals.var_q_rac_qsq__blk828 * locals.var_q_invexpq__blk831_dn9)) * assign38980_e44136) - (assign38980_e44133 * (-locals.var_q_invexpq__blk831_dn9))) / (assign38980_e44136 * assign38980_e44136)), );
            locals.var_q_qcoth__blk829_rv = 0.0;
        }

        if (((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1199 == 0.0)) && (locals.var_guard1200 != 0.0)) {
            let assign38990_e44152: f64 = (0.25 * locals.var_q_d1_qsq__blk826);
            let assign38990_e44154: f64 = (assign38990_e44152 / locals.var_q_qsq__blk825);
            (locals.var_q_temp1__blk814, locals.var_q_temp1__blk814_dn4, locals.var_q_temp1__blk814_dn6, locals.var_q_temp1__blk814_dn7, locals.var_q_temp1__blk814_dn8, locals.var_q_temp1__blk814_dn9, ) = (assign38990_e44154, ((((0.25 * locals.var_q_d1_qsq__blk826_dn4) * locals.var_q_qsq__blk825) - (assign38990_e44152 * locals.var_q_qsq__blk825_dn4)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((0.25 * locals.var_q_d1_qsq__blk826_dn6) * locals.var_q_qsq__blk825) - (assign38990_e44152 * locals.var_q_qsq__blk825_dn6)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((0.25 * locals.var_q_d1_qsq__blk826_dn7) * locals.var_q_qsq__blk825) - (assign38990_e44152 * locals.var_q_qsq__blk825_dn7)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((0.25 * locals.var_q_d1_qsq__blk826_dn8) * locals.var_q_qsq__blk825) - (assign38990_e44152 * locals.var_q_qsq__blk825_dn8)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((0.25 * locals.var_q_d1_qsq__blk826_dn9) * locals.var_q_qsq__blk825) - (assign38990_e44152 * locals.var_q_qsq__blk825_dn9)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), );
            locals.var_q_temp1__blk814_rv = 0.0;
        }

        if (((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1199 == 0.0)) && (locals.var_guard1200 != 0.0)) {
            let assign39000_e44171: f64 = (2.0 - locals.var_q_qcoth__blk829);
            let assign39000_e44172: f64 = (locals.var_q_qcoth__blk829 * assign39000_e44171);
            let assign39000_e44173: f64 = (locals.var_q_qsq__blk825 + assign39000_e44172);
            let assign39000_e44175: f64 = (assign39000_e44173 * locals.var_q_temp1__blk814);
            (locals.var_q_d1_qcoth__blk830, locals.var_q_d1_qcoth__blk830_dn4, locals.var_q_d1_qcoth__blk830_dn6, locals.var_q_d1_qcoth__blk830_dn7, locals.var_q_d1_qcoth__blk830_dn8, locals.var_q_d1_qcoth__blk830_dn9, ) = (assign39000_e44175, (((locals.var_q_qsq__blk825_dn4 + ((locals.var_q_qcoth__blk829_dn4 * assign39000_e44171) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn4)))) * locals.var_q_temp1__blk814) + (assign39000_e44173 * locals.var_q_temp1__blk814_dn4)), (((locals.var_q_qsq__blk825_dn6 + ((locals.var_q_qcoth__blk829_dn6 * assign39000_e44171) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn6)))) * locals.var_q_temp1__blk814) + (assign39000_e44173 * locals.var_q_temp1__blk814_dn6)), (((locals.var_q_qsq__blk825_dn7 + ((locals.var_q_qcoth__blk829_dn7 * assign39000_e44171) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn7)))) * locals.var_q_temp1__blk814) + (assign39000_e44173 * locals.var_q_temp1__blk814_dn7)), (((locals.var_q_qsq__blk825_dn8 + ((locals.var_q_qcoth__blk829_dn8 * assign39000_e44171) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn8)))) * locals.var_q_temp1__blk814) + (assign39000_e44173 * locals.var_q_temp1__blk814_dn8)), (((locals.var_q_qsq__blk825_dn9 + ((locals.var_q_qcoth__blk829_dn9 * assign39000_e44171) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn9)))) * locals.var_q_temp1__blk814) + (assign39000_e44173 * locals.var_q_temp1__blk814_dn9)), );
            locals.var_q_d1_qcoth__blk830_rv = 0.0;
        }

        if (((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1199 == 0.0)) && (locals.var_guard1200 != 0.0)) {
            let assign39010_e44191: f64 = (2.0 * locals.var_q_d1_qcoth__blk830);
            let assign39010_e44194: f64 = (1.0 + locals.var_q_qcoth__blk829);
            let assign39010_e44195: f64 = (assign39010_e44191 * assign39010_e44194);
            let assign39010_e44196: f64 = (locals.var_q_d1_qsq__blk826 - assign39010_e44195);
            let assign39010_e44198: f64 = (assign39010_e44196 * locals.var_q_temp1__blk814);
            let assign39010_e44201: f64 = (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827);
            let assign39010_e44203: f64 = (assign39010_e44201 / locals.var_q_d1_qsq__blk826);
            let assign39010_e44204: f64 = (assign39010_e44198 + assign39010_e44203);
            (locals.var_q_d2_qcoth__blk832, locals.var_q_d2_qcoth__blk832_dn4, locals.var_q_d2_qcoth__blk832_dn6, locals.var_q_d2_qcoth__blk832_dn7, locals.var_q_d2_qcoth__blk832_dn8, locals.var_q_d2_qcoth__blk832_dn9, ) = (assign39010_e44204, ((((locals.var_q_d1_qsq__blk826_dn4 - (((2.0 * locals.var_q_d1_qcoth__blk830_dn4) * assign39010_e44194) + (assign39010_e44191 * locals.var_q_qcoth__blk829_dn4))) * locals.var_q_temp1__blk814) + (assign39010_e44196 * locals.var_q_temp1__blk814_dn4)) + (((((locals.var_q_d1_qcoth__blk830_dn4 * locals.var_q_d2_qsq__blk827) + (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827_dn4)) * locals.var_q_d1_qsq__blk826) - (assign39010_e44201 * locals.var_q_d1_qsq__blk826_dn4)) / (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826))), ((((locals.var_q_d1_qsq__blk826_dn6 - (((2.0 * locals.var_q_d1_qcoth__blk830_dn6) * assign39010_e44194) + (assign39010_e44191 * locals.var_q_qcoth__blk829_dn6))) * locals.var_q_temp1__blk814) + (assign39010_e44196 * locals.var_q_temp1__blk814_dn6)) + (((((locals.var_q_d1_qcoth__blk830_dn6 * locals.var_q_d2_qsq__blk827) + (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827_dn6)) * locals.var_q_d1_qsq__blk826) - (assign39010_e44201 * locals.var_q_d1_qsq__blk826_dn6)) / (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826))), ((((locals.var_q_d1_qsq__blk826_dn7 - (((2.0 * locals.var_q_d1_qcoth__blk830_dn7) * assign39010_e44194) + (assign39010_e44191 * locals.var_q_qcoth__blk829_dn7))) * locals.var_q_temp1__blk814) + (assign39010_e44196 * locals.var_q_temp1__blk814_dn7)) + (((((locals.var_q_d1_qcoth__blk830_dn7 * locals.var_q_d2_qsq__blk827) + (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827_dn7)) * locals.var_q_d1_qsq__blk826) - (assign39010_e44201 * locals.var_q_d1_qsq__blk826_dn7)) / (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826))), ((((locals.var_q_d1_qsq__blk826_dn8 - (((2.0 * locals.var_q_d1_qcoth__blk830_dn8) * assign39010_e44194) + (assign39010_e44191 * locals.var_q_qcoth__blk829_dn8))) * locals.var_q_temp1__blk814) + (assign39010_e44196 * locals.var_q_temp1__blk814_dn8)) + (((((locals.var_q_d1_qcoth__blk830_dn8 * locals.var_q_d2_qsq__blk827) + (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827_dn8)) * locals.var_q_d1_qsq__blk826) - (assign39010_e44201 * locals.var_q_d1_qsq__blk826_dn8)) / (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826))), ((((locals.var_q_d1_qsq__blk826_dn9 - (((2.0 * locals.var_q_d1_qcoth__blk830_dn9) * assign39010_e44194) + (assign39010_e44191 * locals.var_q_qcoth__blk829_dn9))) * locals.var_q_temp1__blk814) + (assign39010_e44196 * locals.var_q_temp1__blk814_dn9)) + (((((locals.var_q_d1_qcoth__blk830_dn9 * locals.var_q_d2_qsq__blk827) + (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827_dn9)) * locals.var_q_d1_qsq__blk826) - (assign39010_e44201 * locals.var_q_d1_qsq__blk826_dn9)) / (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826))), );
            locals.var_q_d2_qcoth__blk832_rv = 0.0;
        }

        if (((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1199 == 0.0)) && (locals.var_guard1200 != 0.0)) {
            let assign39020_e44220: f64 = (0.5 * locals.var_q_qcoth__blk829);
            let assign39020_e44221: f64 = (1.0 - assign39020_e44220);
            (locals.var_q_temp2__blk815, locals.var_q_temp2__blk815_dn4, locals.var_q_temp2__blk815_dn6, locals.var_q_temp2__blk815_dn7, locals.var_q_temp2__blk815_dn8, locals.var_q_temp2__blk815_dn9, ) = (assign39020_e44221, (-(0.5 * locals.var_q_qcoth__blk829_dn4)), (-(0.5 * locals.var_q_qcoth__blk829_dn6)), (-(0.5 * locals.var_q_qcoth__blk829_dn7)), (-(0.5 * locals.var_q_qcoth__blk829_dn8)), (-(0.5 * locals.var_q_qcoth__blk829_dn9)), );
            locals.var_q_temp2__blk815_rv = 0.0;
        }

        if (((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1199 == 0.0)) && (locals.var_guard1200 != 0.0)) {
            let assign39030_e44236: f64 = (locals.var_q_d1_qsq__blk826 / locals.var_q_qsq__blk825);
            let assign39030_e44238: f64 = (assign39030_e44236 * locals.var_q_temp2__blk815);
            (locals.var_q_d1_ln__blk835, locals.var_q_d1_ln__blk835_dn4, locals.var_q_d1_ln__blk835_dn6, locals.var_q_d1_ln__blk835_dn7, locals.var_q_d1_ln__blk835_dn8, locals.var_q_d1_ln__blk835_dn9, ) = (assign39030_e44238, (((((locals.var_q_d1_qsq__blk826_dn4 * locals.var_q_qsq__blk825) - (locals.var_q_d1_qsq__blk826 * locals.var_q_qsq__blk825_dn4)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)) * locals.var_q_temp2__blk815) + (assign39030_e44236 * locals.var_q_temp2__blk815_dn4)), (((((locals.var_q_d1_qsq__blk826_dn6 * locals.var_q_qsq__blk825) - (locals.var_q_d1_qsq__blk826 * locals.var_q_qsq__blk825_dn6)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)) * locals.var_q_temp2__blk815) + (assign39030_e44236 * locals.var_q_temp2__blk815_dn6)), (((((locals.var_q_d1_qsq__blk826_dn7 * locals.var_q_qsq__blk825) - (locals.var_q_d1_qsq__blk826 * locals.var_q_qsq__blk825_dn7)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)) * locals.var_q_temp2__blk815) + (assign39030_e44236 * locals.var_q_temp2__blk815_dn7)), (((((locals.var_q_d1_qsq__blk826_dn8 * locals.var_q_qsq__blk825) - (locals.var_q_d1_qsq__blk826 * locals.var_q_qsq__blk825_dn8)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)) * locals.var_q_temp2__blk815) + (assign39030_e44236 * locals.var_q_temp2__blk815_dn8)), (((((locals.var_q_d1_qsq__blk826_dn9 * locals.var_q_qsq__blk825) - (locals.var_q_d1_qsq__blk826 * locals.var_q_qsq__blk825_dn9)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)) * locals.var_q_temp2__blk815) + (assign39030_e44236 * locals.var_q_temp2__blk815_dn9)), );
            locals.var_q_d1_ln__blk835_rv = 0.0;
        }

    }

    pub(super) fn stamp_reactive_block_56(
        locals: &mut StampLocals,
    ) {
        if (((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1199 == 0.0)) && (locals.var_guard1200 != 0.0)) {
            let assign39040_e44253: f64 = (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815);
            let assign39040_e44258: f64 = (0.5 * locals.var_q_d1_qcoth__blk830);
            let assign39040_e44259: f64 = (locals.var_q_d1_ln__blk835 + assign39040_e44258);
            let assign39040_e44260: f64 = (locals.var_q_d1_qsq__blk826 * assign39040_e44259);
            let assign39040_e44261: f64 = (assign39040_e44253 - assign39040_e44260);
            let assign39040_e44263: f64 = (assign39040_e44261 / locals.var_q_qsq__blk825);
            (locals.var_q_d2_ln__blk836, locals.var_q_d2_ln__blk836_dn4, locals.var_q_d2_ln__blk836_dn6, locals.var_q_d2_ln__blk836_dn7, locals.var_q_d2_ln__blk836_dn8, locals.var_q_d2_ln__blk836_dn9, ) = (assign39040_e44263, ((((((locals.var_q_d2_qsq__blk827_dn4 * locals.var_q_temp2__blk815) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815_dn4)) - ((locals.var_q_d1_qsq__blk826_dn4 * assign39040_e44259) + (locals.var_q_d1_qsq__blk826 * (locals.var_q_d1_ln__blk835_dn4 + (0.5 * locals.var_q_d1_qcoth__blk830_dn4))))) * locals.var_q_qsq__blk825) - (assign39040_e44261 * locals.var_q_qsq__blk825_dn4)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((((locals.var_q_d2_qsq__blk827_dn6 * locals.var_q_temp2__blk815) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815_dn6)) - ((locals.var_q_d1_qsq__blk826_dn6 * assign39040_e44259) + (locals.var_q_d1_qsq__blk826 * (locals.var_q_d1_ln__blk835_dn6 + (0.5 * locals.var_q_d1_qcoth__blk830_dn6))))) * locals.var_q_qsq__blk825) - (assign39040_e44261 * locals.var_q_qsq__blk825_dn6)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((((locals.var_q_d2_qsq__blk827_dn7 * locals.var_q_temp2__blk815) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815_dn7)) - ((locals.var_q_d1_qsq__blk826_dn7 * assign39040_e44259) + (locals.var_q_d1_qsq__blk826 * (locals.var_q_d1_ln__blk835_dn7 + (0.5 * locals.var_q_d1_qcoth__blk830_dn7))))) * locals.var_q_qsq__blk825) - (assign39040_e44261 * locals.var_q_qsq__blk825_dn7)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((((locals.var_q_d2_qsq__blk827_dn8 * locals.var_q_temp2__blk815) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815_dn8)) - ((locals.var_q_d1_qsq__blk826_dn8 * assign39040_e44259) + (locals.var_q_d1_qsq__blk826 * (locals.var_q_d1_ln__blk835_dn8 + (0.5 * locals.var_q_d1_qcoth__blk830_dn8))))) * locals.var_q_qsq__blk825) - (assign39040_e44261 * locals.var_q_qsq__blk825_dn8)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((((locals.var_q_d2_qsq__blk827_dn9 * locals.var_q_temp2__blk815) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815_dn9)) - ((locals.var_q_d1_qsq__blk826_dn9 * assign39040_e44259) + (locals.var_q_d1_qsq__blk826 * (locals.var_q_d1_ln__blk835_dn9 + (0.5 * locals.var_q_d1_qcoth__blk830_dn9))))) * locals.var_q_qsq__blk825) - (assign39040_e44261 * locals.var_q_qsq__blk825_dn9)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), );
            locals.var_q_d2_ln__blk836_rv = 0.0;
        }

        if (((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1199 == 0.0)) && (locals.var_guard1200 == 0.0)) {
            let assign39050_e44281: f64 = (locals.var_q_qsq__blk825 * 0.0166666666667);
            let assign39050_e44285: f64 = (locals.var_q_qsq__blk825 * 0.0238095238095);
            let assign39050_e44289: f64 = (locals.var_q_qsq__blk825 * 0.025);
            let assign39050_e44290: f64 = (1.0 - assign39050_e44289);
            let assign39050_e44291: f64 = (assign39050_e44285 * assign39050_e44290);
            let assign39050_e44292: f64 = (1.0 - assign39050_e44291);
            let assign39050_e44293: f64 = (assign39050_e44281 * assign39050_e44292);
            let assign39050_e44294: f64 = (1.0 - assign39050_e44293);
            let assign39050_e44295: f64 = (0.1666666666667 * assign39050_e44294);
            (locals.var_q_temp3__blk816, locals.var_q_temp3__blk816_dn4, locals.var_q_temp3__blk816_dn6, locals.var_q_temp3__blk816_dn7, locals.var_q_temp3__blk816_dn8, locals.var_q_temp3__blk816_dn9, ) = (assign39050_e44295, (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn4 * 0.0166666666667) * assign39050_e44292) + (assign39050_e44281 * (-(((locals.var_q_qsq__blk825_dn4 * 0.0238095238095) * assign39050_e44290) + (assign39050_e44285 * (-(locals.var_q_qsq__blk825_dn4 * 0.025))))))))), (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn6 * 0.0166666666667) * assign39050_e44292) + (assign39050_e44281 * (-(((locals.var_q_qsq__blk825_dn6 * 0.0238095238095) * assign39050_e44290) + (assign39050_e44285 * (-(locals.var_q_qsq__blk825_dn6 * 0.025))))))))), (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn7 * 0.0166666666667) * assign39050_e44292) + (assign39050_e44281 * (-(((locals.var_q_qsq__blk825_dn7 * 0.0238095238095) * assign39050_e44290) + (assign39050_e44285 * (-(locals.var_q_qsq__blk825_dn7 * 0.025))))))))), (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn8 * 0.0166666666667) * assign39050_e44292) + (assign39050_e44281 * (-(((locals.var_q_qsq__blk825_dn8 * 0.0238095238095) * assign39050_e44290) + (assign39050_e44285 * (-(locals.var_q_qsq__blk825_dn8 * 0.025))))))))), (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn9 * 0.0166666666667) * assign39050_e44292) + (assign39050_e44281 * (-(((locals.var_q_qsq__blk825_dn9 * 0.0238095238095) * assign39050_e44290) + (assign39050_e44285 * (-(locals.var_q_qsq__blk825_dn9 * 0.025))))))))), );
            locals.var_q_temp3__blk816_rv = 0.0;
        }

        if (((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1199 == 0.0)) && (locals.var_guard1200 == 0.0)) {
            let assign39060_e44312: f64 = (locals.var_q_qsq__blk825 * locals.var_q_temp3__blk816);
            let assign39060_e44313: f64 = (2.0 + assign39060_e44312);
            (locals.var_q_qcoth__blk829, locals.var_q_qcoth__blk829_dn4, locals.var_q_qcoth__blk829_dn6, locals.var_q_qcoth__blk829_dn7, locals.var_q_qcoth__blk829_dn8, locals.var_q_qcoth__blk829_dn9, ) = (assign39060_e44313, ((locals.var_q_qsq__blk825_dn4 * locals.var_q_temp3__blk816) + (locals.var_q_qsq__blk825 * locals.var_q_temp3__blk816_dn4)), ((locals.var_q_qsq__blk825_dn6 * locals.var_q_temp3__blk816) + (locals.var_q_qsq__blk825 * locals.var_q_temp3__blk816_dn6)), ((locals.var_q_qsq__blk825_dn7 * locals.var_q_temp3__blk816) + (locals.var_q_qsq__blk825 * locals.var_q_temp3__blk816_dn7)), ((locals.var_q_qsq__blk825_dn8 * locals.var_q_temp3__blk816) + (locals.var_q_qsq__blk825 * locals.var_q_temp3__blk816_dn8)), ((locals.var_q_qsq__blk825_dn9 * locals.var_q_temp3__blk816) + (locals.var_q_qsq__blk825 * locals.var_q_temp3__blk816_dn9)), );
            locals.var_q_qcoth__blk829_rv = 0.0;
        }

        if (((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1199 == 0.0)) && (locals.var_guard1200 == 0.0)) {
            let assign39070_e44331: f64 = (locals.var_q_qsq__blk825 * 0.0333333333333);
            let assign39070_e44335: f64 = (locals.var_q_qsq__blk825 * 0.0357142857143);
            let assign39070_e44339: f64 = (locals.var_q_qsq__blk825 * 0.0333333333333);
            let assign39070_e44340: f64 = (1.0 - assign39070_e44339);
            let assign39070_e44341: f64 = (assign39070_e44335 * assign39070_e44340);
            let assign39070_e44342: f64 = (1.0 - assign39070_e44341);
            let assign39070_e44343: f64 = (assign39070_e44331 * assign39070_e44342);
            let assign39070_e44344: f64 = (1.0 - assign39070_e44343);
            let assign39070_e44345: f64 = (0.1666666666667 * assign39070_e44344);
            (locals.var_q_temp1__blk814, locals.var_q_temp1__blk814_dn4, locals.var_q_temp1__blk814_dn6, locals.var_q_temp1__blk814_dn7, locals.var_q_temp1__blk814_dn8, locals.var_q_temp1__blk814_dn9, ) = (assign39070_e44345, (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn4 * 0.0333333333333) * assign39070_e44342) + (assign39070_e44331 * (-(((locals.var_q_qsq__blk825_dn4 * 0.0357142857143) * assign39070_e44340) + (assign39070_e44335 * (-(locals.var_q_qsq__blk825_dn4 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn6 * 0.0333333333333) * assign39070_e44342) + (assign39070_e44331 * (-(((locals.var_q_qsq__blk825_dn6 * 0.0357142857143) * assign39070_e44340) + (assign39070_e44335 * (-(locals.var_q_qsq__blk825_dn6 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn7 * 0.0333333333333) * assign39070_e44342) + (assign39070_e44331 * (-(((locals.var_q_qsq__blk825_dn7 * 0.0357142857143) * assign39070_e44340) + (assign39070_e44335 * (-(locals.var_q_qsq__blk825_dn7 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn8 * 0.0333333333333) * assign39070_e44342) + (assign39070_e44331 * (-(((locals.var_q_qsq__blk825_dn8 * 0.0357142857143) * assign39070_e44340) + (assign39070_e44335 * (-(locals.var_q_qsq__blk825_dn8 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn9 * 0.0333333333333) * assign39070_e44342) + (assign39070_e44331 * (-(((locals.var_q_qsq__blk825_dn9 * 0.0357142857143) * assign39070_e44340) + (assign39070_e44335 * (-(locals.var_q_qsq__blk825_dn9 * 0.0333333333333))))))))), );
            locals.var_q_temp1__blk814_rv = 0.0;
        }

        if (((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1199 == 0.0)) && (locals.var_guard1200 == 0.0)) {
            let assign39080_e44361: f64 = (locals.var_q_d1_qsq__blk826 * locals.var_q_temp1__blk814);
            (locals.var_q_d1_qcoth__blk830, locals.var_q_d1_qcoth__blk830_dn4, locals.var_q_d1_qcoth__blk830_dn6, locals.var_q_d1_qcoth__blk830_dn7, locals.var_q_d1_qcoth__blk830_dn8, locals.var_q_d1_qcoth__blk830_dn9, ) = (assign39080_e44361, ((locals.var_q_d1_qsq__blk826_dn4 * locals.var_q_temp1__blk814) + (locals.var_q_d1_qsq__blk826 * locals.var_q_temp1__blk814_dn4)), ((locals.var_q_d1_qsq__blk826_dn6 * locals.var_q_temp1__blk814) + (locals.var_q_d1_qsq__blk826 * locals.var_q_temp1__blk814_dn6)), ((locals.var_q_d1_qsq__blk826_dn7 * locals.var_q_temp1__blk814) + (locals.var_q_d1_qsq__blk826 * locals.var_q_temp1__blk814_dn7)), ((locals.var_q_d1_qsq__blk826_dn8 * locals.var_q_temp1__blk814) + (locals.var_q_d1_qsq__blk826 * locals.var_q_temp1__blk814_dn8)), ((locals.var_q_d1_qsq__blk826_dn9 * locals.var_q_temp1__blk814) + (locals.var_q_d1_qsq__blk826 * locals.var_q_temp1__blk814_dn9)), );
            locals.var_q_d1_qcoth__blk830_rv = 0.0;
        }

        if (((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1199 == 0.0)) && (locals.var_guard1200 == 0.0)) {
            let assign39090_e44379: f64 = (locals.var_q_qsq__blk825 * 0.0714285714286);
            let assign39090_e44383: f64 = (0.05 * locals.var_q_qsq__blk825);
            let assign39090_e44387: f64 = (0.0420875420875421 * locals.var_q_qsq__blk825);
            let assign39090_e44388: f64 = (1.0 - assign39090_e44387);
            let assign39090_e44389: f64 = (assign39090_e44383 * assign39090_e44388);
            let assign39090_e44390: f64 = (1.0 - assign39090_e44389);
            let assign39090_e44391: f64 = (assign39090_e44379 * assign39090_e44390);
            let assign39090_e44392: f64 = (1.0 - assign39090_e44391);
            let assign39090_e44393: f64 = (0.0055555555556 * assign39090_e44392);
            (locals.var_q_temp2__blk815, locals.var_q_temp2__blk815_dn4, locals.var_q_temp2__blk815_dn6, locals.var_q_temp2__blk815_dn7, locals.var_q_temp2__blk815_dn8, locals.var_q_temp2__blk815_dn9, ) = (assign39090_e44393, (0.0055555555556 * (-(((locals.var_q_qsq__blk825_dn4 * 0.0714285714286) * assign39090_e44390) + (assign39090_e44379 * (-(((0.05 * locals.var_q_qsq__blk825_dn4) * assign39090_e44388) + (assign39090_e44383 * (-(0.0420875420875421 * locals.var_q_qsq__blk825_dn4))))))))), (0.0055555555556 * (-(((locals.var_q_qsq__blk825_dn6 * 0.0714285714286) * assign39090_e44390) + (assign39090_e44379 * (-(((0.05 * locals.var_q_qsq__blk825_dn6) * assign39090_e44388) + (assign39090_e44383 * (-(0.0420875420875421 * locals.var_q_qsq__blk825_dn6))))))))), (0.0055555555556 * (-(((locals.var_q_qsq__blk825_dn7 * 0.0714285714286) * assign39090_e44390) + (assign39090_e44379 * (-(((0.05 * locals.var_q_qsq__blk825_dn7) * assign39090_e44388) + (assign39090_e44383 * (-(0.0420875420875421 * locals.var_q_qsq__blk825_dn7))))))))), (0.0055555555556 * (-(((locals.var_q_qsq__blk825_dn8 * 0.0714285714286) * assign39090_e44390) + (assign39090_e44379 * (-(((0.05 * locals.var_q_qsq__blk825_dn8) * assign39090_e44388) + (assign39090_e44383 * (-(0.0420875420875421 * locals.var_q_qsq__blk825_dn8))))))))), (0.0055555555556 * (-(((locals.var_q_qsq__blk825_dn9 * 0.0714285714286) * assign39090_e44390) + (assign39090_e44379 * (-(((0.05 * locals.var_q_qsq__blk825_dn9) * assign39090_e44388) + (assign39090_e44383 * (-(0.0420875420875421 * locals.var_q_qsq__blk825_dn9))))))))), );
            locals.var_q_temp2__blk815_rv = 0.0;
        }

        if (((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1199 == 0.0)) && (locals.var_guard1200 == 0.0)) {
            let assign39100_e44409: f64 = (locals.var_q_d2_qsq__blk827 * locals.var_q_temp1__blk814);
            let assign39100_e44412: f64 = (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826);
            let assign39100_e44414: f64 = (assign39100_e44412 * locals.var_q_temp2__blk815);
            let assign39100_e44415: f64 = (assign39100_e44409 - assign39100_e44414);
            (locals.var_q_d2_qcoth__blk832, locals.var_q_d2_qcoth__blk832_dn4, locals.var_q_d2_qcoth__blk832_dn6, locals.var_q_d2_qcoth__blk832_dn7, locals.var_q_d2_qcoth__blk832_dn8, locals.var_q_d2_qcoth__blk832_dn9, ) = (assign39100_e44415, (((locals.var_q_d2_qsq__blk827_dn4 * locals.var_q_temp1__blk814) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp1__blk814_dn4)) - ((((locals.var_q_d1_qsq__blk826_dn4 * locals.var_q_d1_qsq__blk826) + (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826_dn4)) * locals.var_q_temp2__blk815) + (assign39100_e44412 * locals.var_q_temp2__blk815_dn4))), (((locals.var_q_d2_qsq__blk827_dn6 * locals.var_q_temp1__blk814) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp1__blk814_dn6)) - ((((locals.var_q_d1_qsq__blk826_dn6 * locals.var_q_d1_qsq__blk826) + (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826_dn6)) * locals.var_q_temp2__blk815) + (assign39100_e44412 * locals.var_q_temp2__blk815_dn6))), (((locals.var_q_d2_qsq__blk827_dn7 * locals.var_q_temp1__blk814) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp1__blk814_dn7)) - ((((locals.var_q_d1_qsq__blk826_dn7 * locals.var_q_d1_qsq__blk826) + (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826_dn7)) * locals.var_q_temp2__blk815) + (assign39100_e44412 * locals.var_q_temp2__blk815_dn7))), (((locals.var_q_d2_qsq__blk827_dn8 * locals.var_q_temp1__blk814) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp1__blk814_dn8)) - ((((locals.var_q_d1_qsq__blk826_dn8 * locals.var_q_d1_qsq__blk826) + (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826_dn8)) * locals.var_q_temp2__blk815) + (assign39100_e44412 * locals.var_q_temp2__blk815_dn8))), (((locals.var_q_d2_qsq__blk827_dn9 * locals.var_q_temp1__blk814) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp1__blk814_dn9)) - ((((locals.var_q_d1_qsq__blk826_dn9 * locals.var_q_d1_qsq__blk826) + (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826_dn9)) * locals.var_q_temp2__blk815) + (assign39100_e44412 * locals.var_q_temp2__blk815_dn9))), );
            locals.var_q_d2_qcoth__blk832_rv = 0.0;
        }

        if (((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1199 == 0.0)) && (locals.var_guard1200 == 0.0)) {
            let assign39110_e44430: f64 = (-0.5);
            let assign39110_e44432: f64 = (assign39110_e44430 * locals.var_q_d1_qsq__blk826);
            let assign39110_e44434: f64 = (assign39110_e44432 * locals.var_q_temp3__blk816);
            (locals.var_q_d1_ln__blk835, locals.var_q_d1_ln__blk835_dn4, locals.var_q_d1_ln__blk835_dn6, locals.var_q_d1_ln__blk835_dn7, locals.var_q_d1_ln__blk835_dn8, locals.var_q_d1_ln__blk835_dn9, ) = (assign39110_e44434, (((assign39110_e44430 * locals.var_q_d1_qsq__blk826_dn4) * locals.var_q_temp3__blk816) + (assign39110_e44432 * locals.var_q_temp3__blk816_dn4)), (((assign39110_e44430 * locals.var_q_d1_qsq__blk826_dn6) * locals.var_q_temp3__blk816) + (assign39110_e44432 * locals.var_q_temp3__blk816_dn6)), (((assign39110_e44430 * locals.var_q_d1_qsq__blk826_dn7) * locals.var_q_temp3__blk816) + (assign39110_e44432 * locals.var_q_temp3__blk816_dn7)), (((assign39110_e44430 * locals.var_q_d1_qsq__blk826_dn8) * locals.var_q_temp3__blk816) + (assign39110_e44432 * locals.var_q_temp3__blk816_dn8)), (((assign39110_e44430 * locals.var_q_d1_qsq__blk826_dn9) * locals.var_q_temp3__blk816) + (assign39110_e44432 * locals.var_q_temp3__blk816_dn9)), );
            locals.var_q_d1_ln__blk835_rv = 0.0;
        }

        if (((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1199 == 0.0)) && (locals.var_guard1200 == 0.0)) {
            let assign39120_e44449: f64 = (-0.5);
            let assign39120_e44451: f64 = (assign39120_e44449 * locals.var_q_d2_qsq__blk827);
            let assign39120_e44453: f64 = (assign39120_e44451 * locals.var_q_temp3__blk816);
            let assign39120_e44456: f64 = (0.25 * 0.0055555555556);
            let assign39120_e44458: f64 = (assign39120_e44456 * locals.var_q_d1_qsq__blk826);
            let assign39120_e44460: f64 = (assign39120_e44458 * locals.var_q_d1_qsq__blk826);
            let assign39120_e44464: f64 = (locals.var_q_qsq__blk825 * 0.0238095238095);
            let assign39120_e44468: f64 = (0.075 * locals.var_q_qsq__blk825);
            let assign39120_e44469: f64 = (2.0 - assign39120_e44468);
            let assign39120_e44470: f64 = (assign39120_e44464 * assign39120_e44469);
            let assign39120_e44471: f64 = (1.0 - assign39120_e44470);
            let assign39120_e44472: f64 = (assign39120_e44460 * assign39120_e44471);
            let assign39120_e44473: f64 = (assign39120_e44453 + assign39120_e44472);
            (locals.var_q_d2_ln__blk836, locals.var_q_d2_ln__blk836_dn4, locals.var_q_d2_ln__blk836_dn6, locals.var_q_d2_ln__blk836_dn7, locals.var_q_d2_ln__blk836_dn8, locals.var_q_d2_ln__blk836_dn9, ) = (assign39120_e44473, ((((assign39120_e44449 * locals.var_q_d2_qsq__blk827_dn4) * locals.var_q_temp3__blk816) + (assign39120_e44451 * locals.var_q_temp3__blk816_dn4)) + (((((assign39120_e44456 * locals.var_q_d1_qsq__blk826_dn4) * locals.var_q_d1_qsq__blk826) + (assign39120_e44458 * locals.var_q_d1_qsq__blk826_dn4)) * assign39120_e44471) + (assign39120_e44460 * (-(((locals.var_q_qsq__blk825_dn4 * 0.0238095238095) * assign39120_e44469) + (assign39120_e44464 * (-(0.075 * locals.var_q_qsq__blk825_dn4)))))))), ((((assign39120_e44449 * locals.var_q_d2_qsq__blk827_dn6) * locals.var_q_temp3__blk816) + (assign39120_e44451 * locals.var_q_temp3__blk816_dn6)) + (((((assign39120_e44456 * locals.var_q_d1_qsq__blk826_dn6) * locals.var_q_d1_qsq__blk826) + (assign39120_e44458 * locals.var_q_d1_qsq__blk826_dn6)) * assign39120_e44471) + (assign39120_e44460 * (-(((locals.var_q_qsq__blk825_dn6 * 0.0238095238095) * assign39120_e44469) + (assign39120_e44464 * (-(0.075 * locals.var_q_qsq__blk825_dn6)))))))), ((((assign39120_e44449 * locals.var_q_d2_qsq__blk827_dn7) * locals.var_q_temp3__blk816) + (assign39120_e44451 * locals.var_q_temp3__blk816_dn7)) + (((((assign39120_e44456 * locals.var_q_d1_qsq__blk826_dn7) * locals.var_q_d1_qsq__blk826) + (assign39120_e44458 * locals.var_q_d1_qsq__blk826_dn7)) * assign39120_e44471) + (assign39120_e44460 * (-(((locals.var_q_qsq__blk825_dn7 * 0.0238095238095) * assign39120_e44469) + (assign39120_e44464 * (-(0.075 * locals.var_q_qsq__blk825_dn7)))))))), ((((assign39120_e44449 * locals.var_q_d2_qsq__blk827_dn8) * locals.var_q_temp3__blk816) + (assign39120_e44451 * locals.var_q_temp3__blk816_dn8)) + (((((assign39120_e44456 * locals.var_q_d1_qsq__blk826_dn8) * locals.var_q_d1_qsq__blk826) + (assign39120_e44458 * locals.var_q_d1_qsq__blk826_dn8)) * assign39120_e44471) + (assign39120_e44460 * (-(((locals.var_q_qsq__blk825_dn8 * 0.0238095238095) * assign39120_e44469) + (assign39120_e44464 * (-(0.075 * locals.var_q_qsq__blk825_dn8)))))))), ((((assign39120_e44449 * locals.var_q_d2_qsq__blk827_dn9) * locals.var_q_temp3__blk816) + (assign39120_e44451 * locals.var_q_temp3__blk816_dn9)) + (((((assign39120_e44456 * locals.var_q_d1_qsq__blk826_dn9) * locals.var_q_d1_qsq__blk826) + (assign39120_e44458 * locals.var_q_d1_qsq__blk826_dn9)) * assign39120_e44471) + (assign39120_e44460 * (-(((locals.var_q_qsq__blk825_dn9 * 0.0238095238095) * assign39120_e44469) + (assign39120_e44464 * (-(0.075 * locals.var_q_qsq__blk825_dn9)))))))), );
            locals.var_q_d2_ln__blk836_rv = 0.0;
        }

        let assign39130_e44478: f64 = if locals.var_q_qsq__blk825 > 0.005 { 1.0 } else { 0.0 };
        locals.var_guard1201 = assign39130_e44478;
        locals.var_guard1201_rv = 0.0;

        if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1201 != 0.0)) {
            let assign39140_e44488: f64 = (4.0 * locals.var_q_qsq__blk825);
            let assign39140_e44493: f64 = (2.0 - locals.var_q_invexpq__blk831);
            let assign39140_e44494: f64 = (locals.var_q_invexpq__blk831 * assign39140_e44493);
            let assign39140_e44495: f64 = (1.0 - assign39140_e44494);
            let assign39140_e44496: f64 = (assign39140_e44488 / assign39140_e44495);
            (locals.var_q_temp2__blk815, locals.var_q_temp2__blk815_dn4, locals.var_q_temp2__blk815_dn6, locals.var_q_temp2__blk815_dn7, locals.var_q_temp2__blk815_dn8, locals.var_q_temp2__blk815_dn9, ) = (assign39140_e44496, ((((4.0 * locals.var_q_qsq__blk825_dn4) * assign39140_e44495) - (assign39140_e44488 * (-((locals.var_q_invexpq__blk831_dn4 * assign39140_e44493) + (locals.var_q_invexpq__blk831 * (-locals.var_q_invexpq__blk831_dn4)))))) / (assign39140_e44495 * assign39140_e44495)), ((((4.0 * locals.var_q_qsq__blk825_dn6) * assign39140_e44495) - (assign39140_e44488 * (-((locals.var_q_invexpq__blk831_dn6 * assign39140_e44493) + (locals.var_q_invexpq__blk831 * (-locals.var_q_invexpq__blk831_dn6)))))) / (assign39140_e44495 * assign39140_e44495)), ((((4.0 * locals.var_q_qsq__blk825_dn7) * assign39140_e44495) - (assign39140_e44488 * (-((locals.var_q_invexpq__blk831_dn7 * assign39140_e44493) + (locals.var_q_invexpq__blk831 * (-locals.var_q_invexpq__blk831_dn7)))))) / (assign39140_e44495 * assign39140_e44495)), ((((4.0 * locals.var_q_qsq__blk825_dn8) * assign39140_e44495) - (assign39140_e44488 * (-((locals.var_q_invexpq__blk831_dn8 * assign39140_e44493) + (locals.var_q_invexpq__blk831 * (-locals.var_q_invexpq__blk831_dn8)))))) / (assign39140_e44495 * assign39140_e44495)), ((((4.0 * locals.var_q_qsq__blk825_dn9) * assign39140_e44495) - (assign39140_e44488 * (-((locals.var_q_invexpq__blk831_dn9 * assign39140_e44493) + (locals.var_q_invexpq__blk831 * (-locals.var_q_invexpq__blk831_dn9)))))) / (assign39140_e44495 * assign39140_e44495)), );
            locals.var_q_temp2__blk815_rv = 0.0;
        }

        if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1201 != 0.0)) {
            let assign39150_e44508: f64 = (locals.var_q_temp2__blk815 * locals.var_q_invexpq__blk831);
            (locals.var_q_sh_term__blk833, locals.var_q_sh_term__blk833_dn4, locals.var_q_sh_term__blk833_dn6, locals.var_q_sh_term__blk833_dn7, locals.var_q_sh_term__blk833_dn8, locals.var_q_sh_term__blk833_dn9, ) = (assign39150_e44508, ((locals.var_q_temp2__blk815_dn4 * locals.var_q_invexpq__blk831) + (locals.var_q_temp2__blk815 * locals.var_q_invexpq__blk831_dn4)), ((locals.var_q_temp2__blk815_dn6 * locals.var_q_invexpq__blk831) + (locals.var_q_temp2__blk815 * locals.var_q_invexpq__blk831_dn6)), ((locals.var_q_temp2__blk815_dn7 * locals.var_q_invexpq__blk831) + (locals.var_q_temp2__blk815 * locals.var_q_invexpq__blk831_dn7)), ((locals.var_q_temp2__blk815_dn8 * locals.var_q_invexpq__blk831) + (locals.var_q_temp2__blk815 * locals.var_q_invexpq__blk831_dn8)), ((locals.var_q_temp2__blk815_dn9 * locals.var_q_invexpq__blk831) + (locals.var_q_temp2__blk815 * locals.var_q_invexpq__blk831_dn9)), );
            locals.var_q_sh_term__blk833_rv = 0.0;
        }

        if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1201 != 0.0)) {
            let assign39160_e44519: f64 = (locals.var_q_temp2__blk815).ln();
            let assign39160_e44521: f64 = (assign39160_e44519 - locals.var_q_rac_qsq__blk828);
            (locals.var_q_ln_term__blk834, locals.var_q_ln_term__blk834_dn4, locals.var_q_ln_term__blk834_dn6, locals.var_q_ln_term__blk834_dn7, locals.var_q_ln_term__blk834_dn8, locals.var_q_ln_term__blk834_dn9, ) = (assign39160_e44521, ((locals.var_q_temp2__blk815_dn4 / locals.var_q_temp2__blk815) - locals.var_q_rac_qsq__blk828_dn4), ((locals.var_q_temp2__blk815_dn6 / locals.var_q_temp2__blk815) - locals.var_q_rac_qsq__blk828_dn6), ((locals.var_q_temp2__blk815_dn7 / locals.var_q_temp2__blk815) - locals.var_q_rac_qsq__blk828_dn7), ((locals.var_q_temp2__blk815_dn8 / locals.var_q_temp2__blk815) - locals.var_q_rac_qsq__blk828_dn8), ((locals.var_q_temp2__blk815_dn9 / locals.var_q_temp2__blk815) - locals.var_q_rac_qsq__blk828_dn9), );
            locals.var_q_ln_term__blk834_rv = 0.0;
        }

        let assign39170_e44526: f64 = (-0.005);
        let assign39170_e44527: f64 = if locals.var_q_qsq__blk825 < assign39170_e44526 { 1.0 } else { 0.0 };
        locals.var_guard1202 = assign39170_e44527;
        locals.var_guard1202_rv = 0.0;

        if (((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1201 == 0.0)) && (locals.var_guard1202 != 0.0)) {
            let assign39180_e44540: f64 = (0.5 * locals.var_q_rac_qsq__blk828);
            let assign39180_e44541: f64 = (assign39180_e44540).sin();
            (locals.var_q_temp2__blk815, locals.var_q_temp2__blk815_dn4, locals.var_q_temp2__blk815_dn6, locals.var_q_temp2__blk815_dn7, locals.var_q_temp2__blk815_dn8, locals.var_q_temp2__blk815_dn9, ) = (assign39180_e44541, ((assign39180_e44540).cos() * (0.5 * locals.var_q_rac_qsq__blk828_dn4)), ((assign39180_e44540).cos() * (0.5 * locals.var_q_rac_qsq__blk828_dn6)), ((assign39180_e44540).cos() * (0.5 * locals.var_q_rac_qsq__blk828_dn7)), ((assign39180_e44540).cos() * (0.5 * locals.var_q_rac_qsq__blk828_dn8)), ((assign39180_e44540).cos() * (0.5 * locals.var_q_rac_qsq__blk828_dn9)), );
            locals.var_q_temp2__blk815_rv = 0.0;
        }

        if (((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1201 == 0.0)) && (locals.var_guard1202 != 0.0)) {
            let assign39190_e44555: f64 = (-locals.var_q_qsq__blk825);
            let assign39190_e44558: f64 = (locals.var_q_temp2__blk815 * locals.var_q_temp2__blk815);
            let assign39190_e44559: f64 = (assign39190_e44555 / assign39190_e44558);
            (locals.var_q_sh_term__blk833, locals.var_q_sh_term__blk833_dn4, locals.var_q_sh_term__blk833_dn6, locals.var_q_sh_term__blk833_dn7, locals.var_q_sh_term__blk833_dn8, locals.var_q_sh_term__blk833_dn9, ) = (assign39190_e44559, ((((-locals.var_q_qsq__blk825_dn4) * assign39190_e44558) - (assign39190_e44555 * ((locals.var_q_temp2__blk815_dn4 * locals.var_q_temp2__blk815) + (locals.var_q_temp2__blk815 * locals.var_q_temp2__blk815_dn4)))) / (assign39190_e44558 * assign39190_e44558)), ((((-locals.var_q_qsq__blk825_dn6) * assign39190_e44558) - (assign39190_e44555 * ((locals.var_q_temp2__blk815_dn6 * locals.var_q_temp2__blk815) + (locals.var_q_temp2__blk815 * locals.var_q_temp2__blk815_dn6)))) / (assign39190_e44558 * assign39190_e44558)), ((((-locals.var_q_qsq__blk825_dn7) * assign39190_e44558) - (assign39190_e44555 * ((locals.var_q_temp2__blk815_dn7 * locals.var_q_temp2__blk815) + (locals.var_q_temp2__blk815 * locals.var_q_temp2__blk815_dn7)))) / (assign39190_e44558 * assign39190_e44558)), ((((-locals.var_q_qsq__blk825_dn8) * assign39190_e44558) - (assign39190_e44555 * ((locals.var_q_temp2__blk815_dn8 * locals.var_q_temp2__blk815) + (locals.var_q_temp2__blk815 * locals.var_q_temp2__blk815_dn8)))) / (assign39190_e44558 * assign39190_e44558)), ((((-locals.var_q_qsq__blk825_dn9) * assign39190_e44558) - (assign39190_e44555 * ((locals.var_q_temp2__blk815_dn9 * locals.var_q_temp2__blk815) + (locals.var_q_temp2__blk815 * locals.var_q_temp2__blk815_dn9)))) / (assign39190_e44558 * assign39190_e44558)), );
            locals.var_q_sh_term__blk833_rv = 0.0;
        }

        if (((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1201 == 0.0)) && (locals.var_guard1202 != 0.0)) {
            let assign39200_e44573: f64 = (locals.var_q_sh_term__blk833).ln();
            (locals.var_q_ln_term__blk834, locals.var_q_ln_term__blk834_dn4, locals.var_q_ln_term__blk834_dn6, locals.var_q_ln_term__blk834_dn7, locals.var_q_ln_term__blk834_dn8, locals.var_q_ln_term__blk834_dn9, ) = (assign39200_e44573, (locals.var_q_sh_term__blk833_dn4 / locals.var_q_sh_term__blk833), (locals.var_q_sh_term__blk833_dn6 / locals.var_q_sh_term__blk833), (locals.var_q_sh_term__blk833_dn7 / locals.var_q_sh_term__blk833), (locals.var_q_sh_term__blk833_dn8 / locals.var_q_sh_term__blk833), (locals.var_q_sh_term__blk833_dn9 / locals.var_q_sh_term__blk833), );
            locals.var_q_ln_term__blk834_rv = 0.0;
        }

        if (((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1201 == 0.0)) && (locals.var_guard1202 == 0.0)) {
            let assign39210_e44590: f64 = (locals.var_q_qsq__blk825 * 0.3333333333333);
            let assign39210_e44594: f64 = (0.05 * locals.var_q_qsq__blk825);
            let assign39210_e44598: f64 = (0.0396825396825397 * locals.var_q_qsq__blk825);
            let assign39210_e44599: f64 = (1.0 - assign39210_e44598);
            let assign39210_e44600: f64 = (assign39210_e44594 * assign39210_e44599);
            let assign39210_e44601: f64 = (1.0 - assign39210_e44600);
            let assign39210_e44602: f64 = (assign39210_e44590 * assign39210_e44601);
            let assign39210_e44603: f64 = (4.0 - assign39210_e44602);
            (locals.var_q_sh_term__blk833, locals.var_q_sh_term__blk833_dn4, locals.var_q_sh_term__blk833_dn6, locals.var_q_sh_term__blk833_dn7, locals.var_q_sh_term__blk833_dn8, locals.var_q_sh_term__blk833_dn9, ) = (assign39210_e44603, (-(((locals.var_q_qsq__blk825_dn4 * 0.3333333333333) * assign39210_e44601) + (assign39210_e44590 * (-(((0.05 * locals.var_q_qsq__blk825_dn4) * assign39210_e44599) + (assign39210_e44594 * (-(0.0396825396825397 * locals.var_q_qsq__blk825_dn4)))))))), (-(((locals.var_q_qsq__blk825_dn6 * 0.3333333333333) * assign39210_e44601) + (assign39210_e44590 * (-(((0.05 * locals.var_q_qsq__blk825_dn6) * assign39210_e44599) + (assign39210_e44594 * (-(0.0396825396825397 * locals.var_q_qsq__blk825_dn6)))))))), (-(((locals.var_q_qsq__blk825_dn7 * 0.3333333333333) * assign39210_e44601) + (assign39210_e44590 * (-(((0.05 * locals.var_q_qsq__blk825_dn7) * assign39210_e44599) + (assign39210_e44594 * (-(0.0396825396825397 * locals.var_q_qsq__blk825_dn7)))))))), (-(((locals.var_q_qsq__blk825_dn8 * 0.3333333333333) * assign39210_e44601) + (assign39210_e44590 * (-(((0.05 * locals.var_q_qsq__blk825_dn8) * assign39210_e44599) + (assign39210_e44594 * (-(0.0396825396825397 * locals.var_q_qsq__blk825_dn8)))))))), (-(((locals.var_q_qsq__blk825_dn9 * 0.3333333333333) * assign39210_e44601) + (assign39210_e44590 * (-(((0.05 * locals.var_q_qsq__blk825_dn9) * assign39210_e44599) + (assign39210_e44594 * (-(0.0396825396825397 * locals.var_q_qsq__blk825_dn9)))))))), );
            locals.var_q_sh_term__blk833_rv = 0.0;
        }

        if (((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1201 == 0.0)) && (locals.var_guard1202 == 0.0)) {
            let assign39220_e44618: f64 = (locals.var_q_sh_term__blk833).ln();
            (locals.var_q_ln_term__blk834, locals.var_q_ln_term__blk834_dn4, locals.var_q_ln_term__blk834_dn6, locals.var_q_ln_term__blk834_dn7, locals.var_q_ln_term__blk834_dn8, locals.var_q_ln_term__blk834_dn9, ) = (assign39220_e44618, (locals.var_q_sh_term__blk833_dn4 / locals.var_q_sh_term__blk833), (locals.var_q_sh_term__blk833_dn6 / locals.var_q_sh_term__blk833), (locals.var_q_sh_term__blk833_dn7 / locals.var_q_sh_term__blk833), (locals.var_q_sh_term__blk833_dn8 / locals.var_q_sh_term__blk833), (locals.var_q_sh_term__blk833_dn9 / locals.var_q_sh_term__blk833), );
            locals.var_q_ln_term__blk834_rv = 0.0;
        }

        let assign39230_e44623: f64 = (1.01 * locals.var_q_k1q1__blk823);
        let assign39230_e44625: f64 = (assign39230_e44623 + locals.var_q_qcoth__blk829);
        let assign39230_e44627: f64 = if assign39230_e44625 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1203 = assign39230_e44627;
        locals.var_guard1203_rv = 0.0;

        if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1203 != 0.0)) {
            let assign39240_e44637: f64 = (locals.var_q_k1q1__blk823 + locals.var_q_qcoth__blk829);
            (locals.var_q_expnum__blk837, locals.var_q_expnum__blk837_dn4, locals.var_q_expnum__blk837_dn6, locals.var_q_expnum__blk837_dn7, locals.var_q_expnum__blk837_dn8, locals.var_q_expnum__blk837_dn9, ) = (assign39240_e44637, (locals.var_q_k1q1__blk823_dn4 + locals.var_q_qcoth__blk829_dn4), (locals.var_q_k1q1__blk823_dn6 + locals.var_q_qcoth__blk829_dn6), (locals.var_q_k1q1__blk823_dn7 + locals.var_q_qcoth__blk829_dn7), (locals.var_q_k1q1__blk823_dn8 + locals.var_q_qcoth__blk829_dn8), (locals.var_q_k1q1__blk823_dn9 + locals.var_q_qcoth__blk829_dn9), );
            locals.var_q_expnum__blk837_rv = 0.0;
        }

        if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1203 != 0.0)) {
            let assign39250_e44649: f64 = (locals.var_k1__blk932 + locals.var_q_d1_qcoth__blk830);
            (locals.var_q_d1_expnum__blk838, locals.var_q_d1_expnum__blk838_dn4, locals.var_q_d1_expnum__blk838_dn6, locals.var_q_d1_expnum__blk838_dn7, locals.var_q_d1_expnum__blk838_dn8, locals.var_q_d1_expnum__blk838_dn9, ) = (assign39250_e44649, (locals.var_k1__blk932_dn4 + locals.var_q_d1_qcoth__blk830_dn4), (locals.var_k1__blk932_dn6 + locals.var_q_d1_qcoth__blk830_dn6), (locals.var_k1__blk932_dn7 + locals.var_q_d1_qcoth__blk830_dn7), (locals.var_k1__blk932_dn8 + locals.var_q_d1_qcoth__blk830_dn8), (locals.var_k1__blk932_dn9 + locals.var_q_d1_qcoth__blk830_dn9), );
            locals.var_q_d1_expnum__blk838_rv = 0.0;
        }

        if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1203 != 0.0)) {
            (locals.var_q_d2_expnum__blk839, locals.var_q_d2_expnum__blk839_dn4, locals.var_q_d2_expnum__blk839_dn6, locals.var_q_d2_expnum__blk839_dn7, locals.var_q_d2_expnum__blk839_dn8, locals.var_q_d2_expnum__blk839_dn9, ) = (locals.var_q_d2_qcoth__blk832, locals.var_q_d2_qcoth__blk832_dn4, locals.var_q_d2_qcoth__blk832_dn6, locals.var_q_d2_qcoth__blk832_dn7, locals.var_q_d2_qcoth__blk832_dn8, locals.var_q_d2_qcoth__blk832_dn9, );
            locals.var_q_d2_expnum__blk839_rv = 0.0;
        }

        if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1203 == 0.0)) {
            let assign39270_e44673: f64 = (locals.var_q_k1q1__blk823 - locals.var_q_qcoth__blk829);
            let assign39270_e44674: f64 = (1.0 / assign39270_e44673);
            (locals.var_q_temp2__blk815, locals.var_q_temp2__blk815_dn4, locals.var_q_temp2__blk815_dn6, locals.var_q_temp2__blk815_dn7, locals.var_q_temp2__blk815_dn8, locals.var_q_temp2__blk815_dn9, ) = (assign39270_e44674, (-((locals.var_q_k1q1__blk823_dn4 - locals.var_q_qcoth__blk829_dn4) / (assign39270_e44673 * assign39270_e44673))), (-((locals.var_q_k1q1__blk823_dn6 - locals.var_q_qcoth__blk829_dn6) / (assign39270_e44673 * assign39270_e44673))), (-((locals.var_q_k1q1__blk823_dn7 - locals.var_q_qcoth__blk829_dn7) / (assign39270_e44673 * assign39270_e44673))), (-((locals.var_q_k1q1__blk823_dn8 - locals.var_q_qcoth__blk829_dn8) / (assign39270_e44673 * assign39270_e44673))), (-((locals.var_q_k1q1__blk823_dn9 - locals.var_q_qcoth__blk829_dn9) / (assign39270_e44673 * assign39270_e44673))), );
            locals.var_q_temp2__blk815_rv = 0.0;
        }

        if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1203 == 0.0)) {
            let assign39280_e44687: f64 = (locals.var_q_d1_qcoth__blk830 - locals.var_k1__blk932);
            (locals.var_q_temp3__blk816, locals.var_q_temp3__blk816_dn4, locals.var_q_temp3__blk816_dn6, locals.var_q_temp3__blk816_dn7, locals.var_q_temp3__blk816_dn8, locals.var_q_temp3__blk816_dn9, ) = (assign39280_e44687, (locals.var_q_d1_qcoth__blk830_dn4 - locals.var_k1__blk932_dn4), (locals.var_q_d1_qcoth__blk830_dn6 - locals.var_k1__blk932_dn6), (locals.var_q_d1_qcoth__blk830_dn7 - locals.var_k1__blk932_dn7), (locals.var_q_d1_qcoth__blk830_dn8 - locals.var_k1__blk932_dn8), (locals.var_q_d1_qcoth__blk830_dn9 - locals.var_k1__blk932_dn9), );
            locals.var_q_temp3__blk816_rv = 0.0;
        }

        if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1203 == 0.0)) {
            let assign39290_e44700: f64 = (locals.var_q_aexp__blk824 - locals.var_q_sh_term__blk833);
            let assign39290_e44702: f64 = (assign39290_e44700 * locals.var_q_temp2__blk815);
            (locals.var_q_expnum__blk837, locals.var_q_expnum__blk837_dn4, locals.var_q_expnum__blk837_dn6, locals.var_q_expnum__blk837_dn7, locals.var_q_expnum__blk837_dn8, locals.var_q_expnum__blk837_dn9, ) = (assign39290_e44702, (((locals.var_q_aexp__blk824_dn4 - locals.var_q_sh_term__blk833_dn4) * locals.var_q_temp2__blk815) + (assign39290_e44700 * locals.var_q_temp2__blk815_dn4)), (((locals.var_q_aexp__blk824_dn6 - locals.var_q_sh_term__blk833_dn6) * locals.var_q_temp2__blk815) + (assign39290_e44700 * locals.var_q_temp2__blk815_dn6)), (((locals.var_q_aexp__blk824_dn7 - locals.var_q_sh_term__blk833_dn7) * locals.var_q_temp2__blk815) + (assign39290_e44700 * locals.var_q_temp2__blk815_dn7)), (((locals.var_q_aexp__blk824_dn8 - locals.var_q_sh_term__blk833_dn8) * locals.var_q_temp2__blk815) + (assign39290_e44700 * locals.var_q_temp2__blk815_dn8)), (((locals.var_q_aexp__blk824_dn9 - locals.var_q_sh_term__blk833_dn9) * locals.var_q_temp2__blk815) + (assign39290_e44700 * locals.var_q_temp2__blk815_dn9)), );
            locals.var_q_expnum__blk837_rv = 0.0;
        }

        if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1203 == 0.0)) {
            let assign39300_e44715: f64 = (locals.var_q_temp3__blk816 * locals.var_q_expnum__blk837);
            let assign39300_e44717: f64 = (assign39300_e44715 - locals.var_q_aexp__blk824);
            let assign39300_e44720: f64 = (locals.var_q_d1_ln__blk835 * locals.var_q_sh_term__blk833);
            let assign39300_e44721: f64 = (assign39300_e44717 - assign39300_e44720);
            let assign39300_e44723: f64 = (assign39300_e44721 * locals.var_q_temp2__blk815);
            (locals.var_q_d1_expnum__blk838, locals.var_q_d1_expnum__blk838_dn4, locals.var_q_d1_expnum__blk838_dn6, locals.var_q_d1_expnum__blk838_dn7, locals.var_q_d1_expnum__blk838_dn8, locals.var_q_d1_expnum__blk838_dn9, ) = (assign39300_e44723, ((((((locals.var_q_temp3__blk816_dn4 * locals.var_q_expnum__blk837) + (locals.var_q_temp3__blk816 * locals.var_q_expnum__blk837_dn4)) - locals.var_q_aexp__blk824_dn4) - ((locals.var_q_d1_ln__blk835_dn4 * locals.var_q_sh_term__blk833) + (locals.var_q_d1_ln__blk835 * locals.var_q_sh_term__blk833_dn4))) * locals.var_q_temp2__blk815) + (assign39300_e44721 * locals.var_q_temp2__blk815_dn4)), ((((((locals.var_q_temp3__blk816_dn6 * locals.var_q_expnum__blk837) + (locals.var_q_temp3__blk816 * locals.var_q_expnum__blk837_dn6)) - locals.var_q_aexp__blk824_dn6) - ((locals.var_q_d1_ln__blk835_dn6 * locals.var_q_sh_term__blk833) + (locals.var_q_d1_ln__blk835 * locals.var_q_sh_term__blk833_dn6))) * locals.var_q_temp2__blk815) + (assign39300_e44721 * locals.var_q_temp2__blk815_dn6)), ((((((locals.var_q_temp3__blk816_dn7 * locals.var_q_expnum__blk837) + (locals.var_q_temp3__blk816 * locals.var_q_expnum__blk837_dn7)) - locals.var_q_aexp__blk824_dn7) - ((locals.var_q_d1_ln__blk835_dn7 * locals.var_q_sh_term__blk833) + (locals.var_q_d1_ln__blk835 * locals.var_q_sh_term__blk833_dn7))) * locals.var_q_temp2__blk815) + (assign39300_e44721 * locals.var_q_temp2__blk815_dn7)), ((((((locals.var_q_temp3__blk816_dn8 * locals.var_q_expnum__blk837) + (locals.var_q_temp3__blk816 * locals.var_q_expnum__blk837_dn8)) - locals.var_q_aexp__blk824_dn8) - ((locals.var_q_d1_ln__blk835_dn8 * locals.var_q_sh_term__blk833) + (locals.var_q_d1_ln__blk835 * locals.var_q_sh_term__blk833_dn8))) * locals.var_q_temp2__blk815) + (assign39300_e44721 * locals.var_q_temp2__blk815_dn8)), ((((((locals.var_q_temp3__blk816_dn9 * locals.var_q_expnum__blk837) + (locals.var_q_temp3__blk816 * locals.var_q_expnum__blk837_dn9)) - locals.var_q_aexp__blk824_dn9) - ((locals.var_q_d1_ln__blk835_dn9 * locals.var_q_sh_term__blk833) + (locals.var_q_d1_ln__blk835 * locals.var_q_sh_term__blk833_dn9))) * locals.var_q_temp2__blk815) + (assign39300_e44721 * locals.var_q_temp2__blk815_dn9)), );
            locals.var_q_d1_expnum__blk838_rv = 0.0;
        }

        if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1203 == 0.0)) {
            let assign39310_e44736: f64 = (locals.var_q_d2_qcoth__blk832 * locals.var_q_expnum__blk837);
            let assign39310_e44739: f64 = (2.0 * locals.var_q_temp3__blk816);
            let assign39310_e44741: f64 = (assign39310_e44739 * locals.var_q_d1_expnum__blk838);
            let assign39310_e44742: f64 = (assign39310_e44736 + assign39310_e44741);
            let assign39310_e44744: f64 = (assign39310_e44742 + locals.var_q_aexp__blk824);
            let assign39310_e44748: f64 = (locals.var_q_d1_ln__blk835 * locals.var_q_d1_ln__blk835);
            let assign39310_e44749: f64 = (locals.var_q_d2_ln__blk836 + assign39310_e44748);
            let assign39310_e44751: f64 = (assign39310_e44749 * locals.var_q_sh_term__blk833);
            let assign39310_e44752: f64 = (assign39310_e44744 - assign39310_e44751);
            let assign39310_e44754: f64 = (assign39310_e44752 * locals.var_q_temp2__blk815);
            (locals.var_q_d2_expnum__blk839, locals.var_q_d2_expnum__blk839_dn4, locals.var_q_d2_expnum__blk839_dn6, locals.var_q_d2_expnum__blk839_dn7, locals.var_q_d2_expnum__blk839_dn8, locals.var_q_d2_expnum__blk839_dn9, ) = (assign39310_e44754, (((((((locals.var_q_d2_qcoth__blk832_dn4 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qcoth__blk832 * locals.var_q_expnum__blk837_dn4)) + (((2.0 * locals.var_q_temp3__blk816_dn4) * locals.var_q_d1_expnum__blk838) + (assign39310_e44739 * locals.var_q_d1_expnum__blk838_dn4))) + locals.var_q_aexp__blk824_dn4) - (((locals.var_q_d2_ln__blk836_dn4 + ((locals.var_q_d1_ln__blk835_dn4 * locals.var_q_d1_ln__blk835) + (locals.var_q_d1_ln__blk835 * locals.var_q_d1_ln__blk835_dn4))) * locals.var_q_sh_term__blk833) + (assign39310_e44749 * locals.var_q_sh_term__blk833_dn4))) * locals.var_q_temp2__blk815) + (assign39310_e44752 * locals.var_q_temp2__blk815_dn4)), (((((((locals.var_q_d2_qcoth__blk832_dn6 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qcoth__blk832 * locals.var_q_expnum__blk837_dn6)) + (((2.0 * locals.var_q_temp3__blk816_dn6) * locals.var_q_d1_expnum__blk838) + (assign39310_e44739 * locals.var_q_d1_expnum__blk838_dn6))) + locals.var_q_aexp__blk824_dn6) - (((locals.var_q_d2_ln__blk836_dn6 + ((locals.var_q_d1_ln__blk835_dn6 * locals.var_q_d1_ln__blk835) + (locals.var_q_d1_ln__blk835 * locals.var_q_d1_ln__blk835_dn6))) * locals.var_q_sh_term__blk833) + (assign39310_e44749 * locals.var_q_sh_term__blk833_dn6))) * locals.var_q_temp2__blk815) + (assign39310_e44752 * locals.var_q_temp2__blk815_dn6)), (((((((locals.var_q_d2_qcoth__blk832_dn7 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qcoth__blk832 * locals.var_q_expnum__blk837_dn7)) + (((2.0 * locals.var_q_temp3__blk816_dn7) * locals.var_q_d1_expnum__blk838) + (assign39310_e44739 * locals.var_q_d1_expnum__blk838_dn7))) + locals.var_q_aexp__blk824_dn7) - (((locals.var_q_d2_ln__blk836_dn7 + ((locals.var_q_d1_ln__blk835_dn7 * locals.var_q_d1_ln__blk835) + (locals.var_q_d1_ln__blk835 * locals.var_q_d1_ln__blk835_dn7))) * locals.var_q_sh_term__blk833) + (assign39310_e44749 * locals.var_q_sh_term__blk833_dn7))) * locals.var_q_temp2__blk815) + (assign39310_e44752 * locals.var_q_temp2__blk815_dn7)), (((((((locals.var_q_d2_qcoth__blk832_dn8 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qcoth__blk832 * locals.var_q_expnum__blk837_dn8)) + (((2.0 * locals.var_q_temp3__blk816_dn8) * locals.var_q_d1_expnum__blk838) + (assign39310_e44739 * locals.var_q_d1_expnum__blk838_dn8))) + locals.var_q_aexp__blk824_dn8) - (((locals.var_q_d2_ln__blk836_dn8 + ((locals.var_q_d1_ln__blk835_dn8 * locals.var_q_d1_ln__blk835) + (locals.var_q_d1_ln__blk835 * locals.var_q_d1_ln__blk835_dn8))) * locals.var_q_sh_term__blk833) + (assign39310_e44749 * locals.var_q_sh_term__blk833_dn8))) * locals.var_q_temp2__blk815) + (assign39310_e44752 * locals.var_q_temp2__blk815_dn8)), (((((((locals.var_q_d2_qcoth__blk832_dn9 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qcoth__blk832 * locals.var_q_expnum__blk837_dn9)) + (((2.0 * locals.var_q_temp3__blk816_dn9) * locals.var_q_d1_expnum__blk838) + (assign39310_e44739 * locals.var_q_d1_expnum__blk838_dn9))) + locals.var_q_aexp__blk824_dn9) - (((locals.var_q_d2_ln__blk836_dn9 + ((locals.var_q_d1_ln__blk835_dn9 * locals.var_q_d1_ln__blk835) + (locals.var_q_d1_ln__blk835 * locals.var_q_d1_ln__blk835_dn9))) * locals.var_q_sh_term__blk833) + (assign39310_e44749 * locals.var_q_sh_term__blk833_dn9))) * locals.var_q_temp2__blk815) + (assign39310_e44752 * locals.var_q_temp2__blk815_dn9)), );
            locals.var_q_d2_expnum__blk839_rv = 0.0;
        }

        let assign39320_e44759: f64 = if locals.var_q_expnum__blk837 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1204 = assign39320_e44759;
        locals.var_guard1204_rv = 0.0;

        if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1204 != 0.0)) {
            let assign39330_e44768: f64 = (locals.var_q_expnum__blk837).ln();
            (locals.var_q_lnexpnum__blk840, locals.var_q_lnexpnum__blk840_dn4, locals.var_q_lnexpnum__blk840_dn6, locals.var_q_lnexpnum__blk840_dn7, locals.var_q_lnexpnum__blk840_dn8, locals.var_q_lnexpnum__blk840_dn9, ) = (assign39330_e44768, (locals.var_q_expnum__blk837_dn4 / locals.var_q_expnum__blk837), (locals.var_q_expnum__blk837_dn6 / locals.var_q_expnum__blk837), (locals.var_q_expnum__blk837_dn7 / locals.var_q_expnum__blk837), (locals.var_q_expnum__blk837_dn8 / locals.var_q_expnum__blk837), (locals.var_q_expnum__blk837_dn9 / locals.var_q_expnum__blk837), );
            locals.var_q_lnexpnum__blk840_rv = 0.0;
        }

        if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1204 != 0.0)) {
            let assign39340_e44780: f64 = (1.0 / locals.var_q_expnum__blk837);
            (locals.var_q_temp1__blk814, locals.var_q_temp1__blk814_dn4, locals.var_q_temp1__blk814_dn6, locals.var_q_temp1__blk814_dn7, locals.var_q_temp1__blk814_dn8, locals.var_q_temp1__blk814_dn9, ) = (assign39340_e44780, (-(locals.var_q_expnum__blk837_dn4 / (locals.var_q_expnum__blk837 * locals.var_q_expnum__blk837))), (-(locals.var_q_expnum__blk837_dn6 / (locals.var_q_expnum__blk837 * locals.var_q_expnum__blk837))), (-(locals.var_q_expnum__blk837_dn7 / (locals.var_q_expnum__blk837 * locals.var_q_expnum__blk837))), (-(locals.var_q_expnum__blk837_dn8 / (locals.var_q_expnum__blk837 * locals.var_q_expnum__blk837))), (-(locals.var_q_expnum__blk837_dn9 / (locals.var_q_expnum__blk837 * locals.var_q_expnum__blk837))), );
            locals.var_q_temp1__blk814_rv = 0.0;
        }

        if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1204 != 0.0)) {
            let assign39350_e44792: f64 = (locals.var_q_d1_expnum__blk838 * locals.var_q_temp1__blk814);
            (locals.var_q_d1_lnexpnum__blk841, locals.var_q_d1_lnexpnum__blk841_dn4, locals.var_q_d1_lnexpnum__blk841_dn6, locals.var_q_d1_lnexpnum__blk841_dn7, locals.var_q_d1_lnexpnum__blk841_dn8, locals.var_q_d1_lnexpnum__blk841_dn9, ) = (assign39350_e44792, ((locals.var_q_d1_expnum__blk838_dn4 * locals.var_q_temp1__blk814) + (locals.var_q_d1_expnum__blk838 * locals.var_q_temp1__blk814_dn4)), ((locals.var_q_d1_expnum__blk838_dn6 * locals.var_q_temp1__blk814) + (locals.var_q_d1_expnum__blk838 * locals.var_q_temp1__blk814_dn6)), ((locals.var_q_d1_expnum__blk838_dn7 * locals.var_q_temp1__blk814) + (locals.var_q_d1_expnum__blk838 * locals.var_q_temp1__blk814_dn7)), ((locals.var_q_d1_expnum__blk838_dn8 * locals.var_q_temp1__blk814) + (locals.var_q_d1_expnum__blk838 * locals.var_q_temp1__blk814_dn8)), ((locals.var_q_d1_expnum__blk838_dn9 * locals.var_q_temp1__blk814) + (locals.var_q_d1_expnum__blk838 * locals.var_q_temp1__blk814_dn9)), );
            locals.var_q_d1_lnexpnum__blk841_rv = 0.0;
        }

        if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1204 != 0.0)) {
            let assign39360_e44804: f64 = (locals.var_q_d2_expnum__blk839 * locals.var_q_temp1__blk814);
            let assign39360_e44807: f64 = (locals.var_q_d1_lnexpnum__blk841 * locals.var_q_d1_lnexpnum__blk841);
            let assign39360_e44808: f64 = (assign39360_e44804 - assign39360_e44807);
            (locals.var_q_d2_lnexpnum__blk842, locals.var_q_d2_lnexpnum__blk842_dn4, locals.var_q_d2_lnexpnum__blk842_dn6, locals.var_q_d2_lnexpnum__blk842_dn7, locals.var_q_d2_lnexpnum__blk842_dn8, locals.var_q_d2_lnexpnum__blk842_dn9, ) = (assign39360_e44808, (((locals.var_q_d2_expnum__blk839_dn4 * locals.var_q_temp1__blk814) + (locals.var_q_d2_expnum__blk839 * locals.var_q_temp1__blk814_dn4)) - ((locals.var_q_d1_lnexpnum__blk841_dn4 * locals.var_q_d1_lnexpnum__blk841) + (locals.var_q_d1_lnexpnum__blk841 * locals.var_q_d1_lnexpnum__blk841_dn4))), (((locals.var_q_d2_expnum__blk839_dn6 * locals.var_q_temp1__blk814) + (locals.var_q_d2_expnum__blk839 * locals.var_q_temp1__blk814_dn6)) - ((locals.var_q_d1_lnexpnum__blk841_dn6 * locals.var_q_d1_lnexpnum__blk841) + (locals.var_q_d1_lnexpnum__blk841 * locals.var_q_d1_lnexpnum__blk841_dn6))), (((locals.var_q_d2_expnum__blk839_dn7 * locals.var_q_temp1__blk814) + (locals.var_q_d2_expnum__blk839 * locals.var_q_temp1__blk814_dn7)) - ((locals.var_q_d1_lnexpnum__blk841_dn7 * locals.var_q_d1_lnexpnum__blk841) + (locals.var_q_d1_lnexpnum__blk841 * locals.var_q_d1_lnexpnum__blk841_dn7))), (((locals.var_q_d2_expnum__blk839_dn8 * locals.var_q_temp1__blk814) + (locals.var_q_d2_expnum__blk839 * locals.var_q_temp1__blk814_dn8)) - ((locals.var_q_d1_lnexpnum__blk841_dn8 * locals.var_q_d1_lnexpnum__blk841) + (locals.var_q_d1_lnexpnum__blk841 * locals.var_q_d1_lnexpnum__blk841_dn8))), (((locals.var_q_d2_expnum__blk839_dn9 * locals.var_q_temp1__blk814) + (locals.var_q_d2_expnum__blk839 * locals.var_q_temp1__blk814_dn9)) - ((locals.var_q_d1_lnexpnum__blk841_dn9 * locals.var_q_d1_lnexpnum__blk841) + (locals.var_q_d1_lnexpnum__blk841 * locals.var_q_d1_lnexpnum__blk841_dn9))), );
            locals.var_q_d2_lnexpnum__blk842_rv = 0.0;
        }

        if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1204 == 0.0)) {
            let assign39370_e44821: f64 = (locals.var_q_k1q1__blk823 + 0.6931471805599);
            let assign39370_e44823: f64 = (-locals.var_q_k1q1__blk823);
            let assign39370_e44824: f64 = (assign39370_e44823).ln();
            let assign39370_e44825: f64 = (assign39370_e44821 + assign39370_e44824);
            (locals.var_q_lnexpnum__blk840, locals.var_q_lnexpnum__blk840_dn4, locals.var_q_lnexpnum__blk840_dn6, locals.var_q_lnexpnum__blk840_dn7, locals.var_q_lnexpnum__blk840_dn8, locals.var_q_lnexpnum__blk840_dn9, ) = (assign39370_e44825, (locals.var_q_k1q1__blk823_dn4 + ((-locals.var_q_k1q1__blk823_dn4) / assign39370_e44823)), (locals.var_q_k1q1__blk823_dn6 + ((-locals.var_q_k1q1__blk823_dn6) / assign39370_e44823)), (locals.var_q_k1q1__blk823_dn7 + ((-locals.var_q_k1q1__blk823_dn7) / assign39370_e44823)), (locals.var_q_k1q1__blk823_dn8 + ((-locals.var_q_k1q1__blk823_dn8) / assign39370_e44823)), (locals.var_q_k1q1__blk823_dn9 + ((-locals.var_q_k1q1__blk823_dn9) / assign39370_e44823)), );
            locals.var_q_lnexpnum__blk840_rv = 0.0;
        }

        if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1204 == 0.0)) {
            let assign39380_e44838: f64 = (1.0 / locals.var_q1d__blk1001);
            (locals.var_q_temp1__blk814, locals.var_q_temp1__blk814_dn4, locals.var_q_temp1__blk814_dn6, locals.var_q_temp1__blk814_dn7, locals.var_q_temp1__blk814_dn8, locals.var_q_temp1__blk814_dn9, ) = (assign39380_e44838, (-(locals.var_q1d__blk1001_dn4 / (locals.var_q1d__blk1001 * locals.var_q1d__blk1001))), (-(locals.var_q1d__blk1001_dn6 / (locals.var_q1d__blk1001 * locals.var_q1d__blk1001))), (-(locals.var_q1d__blk1001_dn7 / (locals.var_q1d__blk1001 * locals.var_q1d__blk1001))), (-(locals.var_q1d__blk1001_dn8 / (locals.var_q1d__blk1001 * locals.var_q1d__blk1001))), (-(locals.var_q1d__blk1001_dn9 / (locals.var_q1d__blk1001 * locals.var_q1d__blk1001))), );
            locals.var_q_temp1__blk814_rv = 0.0;
        }

        if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1204 == 0.0)) {
            let assign39390_e44851: f64 = (locals.var_k1__blk932 + locals.var_q_temp1__blk814);
            (locals.var_q_d1_lnexpnum__blk841, locals.var_q_d1_lnexpnum__blk841_dn4, locals.var_q_d1_lnexpnum__blk841_dn6, locals.var_q_d1_lnexpnum__blk841_dn7, locals.var_q_d1_lnexpnum__blk841_dn8, locals.var_q_d1_lnexpnum__blk841_dn9, ) = (assign39390_e44851, (locals.var_k1__blk932_dn4 + locals.var_q_temp1__blk814_dn4), (locals.var_k1__blk932_dn6 + locals.var_q_temp1__blk814_dn6), (locals.var_k1__blk932_dn7 + locals.var_q_temp1__blk814_dn7), (locals.var_k1__blk932_dn8 + locals.var_q_temp1__blk814_dn8), (locals.var_k1__blk932_dn9 + locals.var_q_temp1__blk814_dn9), );
            locals.var_q_d1_lnexpnum__blk841_rv = 0.0;
        }

        if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1204 == 0.0)) {
            let assign39400_e44863: f64 = (-locals.var_q_temp1__blk814);
            let assign39400_e44865: f64 = (assign39400_e44863 * locals.var_q_temp1__blk814);
            (locals.var_q_d2_lnexpnum__blk842, locals.var_q_d2_lnexpnum__blk842_dn4, locals.var_q_d2_lnexpnum__blk842_dn6, locals.var_q_d2_lnexpnum__blk842_dn7, locals.var_q_d2_lnexpnum__blk842_dn8, locals.var_q_d2_lnexpnum__blk842_dn9, ) = (assign39400_e44865, (((-locals.var_q_temp1__blk814_dn4) * locals.var_q_temp1__blk814) + (assign39400_e44863 * locals.var_q_temp1__blk814_dn4)), (((-locals.var_q_temp1__blk814_dn6) * locals.var_q_temp1__blk814) + (assign39400_e44863 * locals.var_q_temp1__blk814_dn6)), (((-locals.var_q_temp1__blk814_dn7) * locals.var_q_temp1__blk814) + (assign39400_e44863 * locals.var_q_temp1__blk814_dn7)), (((-locals.var_q_temp1__blk814_dn8) * locals.var_q_temp1__blk814) + (assign39400_e44863 * locals.var_q_temp1__blk814_dn8)), (((-locals.var_q_temp1__blk814_dn9) * locals.var_q_temp1__blk814) + (assign39400_e44863 * locals.var_q_temp1__blk814_dn9)), );
            locals.var_q_d2_lnexpnum__blk842_rv = 0.0;
        }

        if (((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) {
            let assign39410_e44875: f64 = (locals.var_xg2x__blk931 - locals.var_xg1x__blk930);
            let assign39410_e44877: f64 = (assign39410_e44875 + locals.var_q1d__blk1001);
            let assign39410_e44880: f64 = (2.0 * locals.var_q_lnexpnum__blk840);
            let assign39410_e44881: f64 = (assign39410_e44877 + assign39410_e44880);
            let assign39410_e44883: f64 = (assign39410_e44881 - locals.var_q_ln_term__blk834);
            (locals.var_q_q2_int__blk843, locals.var_q_q2_int__blk843_dn4, locals.var_q_q2_int__blk843_dn6, locals.var_q_q2_int__blk843_dn7, locals.var_q_q2_int__blk843_dn8, locals.var_q_q2_int__blk843_dn9, ) = (assign39410_e44883, ((((locals.var_xg2x__blk931_dn4 - locals.var_xg1x__blk930_dn4) + locals.var_q1d__blk1001_dn4) + (2.0 * locals.var_q_lnexpnum__blk840_dn4)) - locals.var_q_ln_term__blk834_dn4), ((((locals.var_xg2x__blk931_dn6 - locals.var_xg1x__blk930_dn6) + locals.var_q1d__blk1001_dn6) + (2.0 * locals.var_q_lnexpnum__blk840_dn6)) - locals.var_q_ln_term__blk834_dn6), ((((locals.var_xg2x__blk931_dn7 - locals.var_xg1x__blk930_dn7) + locals.var_q1d__blk1001_dn7) + (2.0 * locals.var_q_lnexpnum__blk840_dn7)) - locals.var_q_ln_term__blk834_dn7), ((((locals.var_xg2x__blk931_dn8 - locals.var_xg1x__blk930_dn8) + locals.var_q1d__blk1001_dn8) + (2.0 * locals.var_q_lnexpnum__blk840_dn8)) - locals.var_q_ln_term__blk834_dn8), ((((locals.var_xg2x__blk931_dn9 - locals.var_xg1x__blk930_dn9) + locals.var_q1d__blk1001_dn9) + (2.0 * locals.var_q_lnexpnum__blk840_dn9)) - locals.var_q_ln_term__blk834_dn9), );
            locals.var_q_q2_int__blk843_rv = 0.0;
        }

        if (((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) {
            let assign39420_e44894: f64 = (2.0 * locals.var_q_d1_lnexpnum__blk841);
            let assign39420_e44895: f64 = (1.0 + assign39420_e44894);
            let assign39420_e44897: f64 = (assign39420_e44895 - locals.var_q_d1_ln__blk835);
            (locals.var_q_d1_q2__blk844, locals.var_q_d1_q2__blk844_dn4, locals.var_q_d1_q2__blk844_dn6, locals.var_q_d1_q2__blk844_dn7, locals.var_q_d1_q2__blk844_dn8, locals.var_q_d1_q2__blk844_dn9, ) = (assign39420_e44897, ((2.0 * locals.var_q_d1_lnexpnum__blk841_dn4) - locals.var_q_d1_ln__blk835_dn4), ((2.0 * locals.var_q_d1_lnexpnum__blk841_dn6) - locals.var_q_d1_ln__blk835_dn6), ((2.0 * locals.var_q_d1_lnexpnum__blk841_dn7) - locals.var_q_d1_ln__blk835_dn7), ((2.0 * locals.var_q_d1_lnexpnum__blk841_dn8) - locals.var_q_d1_ln__blk835_dn8), ((2.0 * locals.var_q_d1_lnexpnum__blk841_dn9) - locals.var_q_d1_ln__blk835_dn9), );
            locals.var_q_d1_q2__blk844_rv = 0.0;
        }

        if (((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) {
            let assign39430_e44907: f64 = (2.0 * locals.var_q_d2_lnexpnum__blk842);
            let assign39430_e44909: f64 = (assign39430_e44907 - locals.var_q_d2_ln__blk836);
            (locals.var_q_d2_q2__blk845, locals.var_q_d2_q2__blk845_dn4, locals.var_q_d2_q2__blk845_dn6, locals.var_q_d2_q2__blk845_dn7, locals.var_q_d2_q2__blk845_dn8, locals.var_q_d2_q2__blk845_dn9, ) = (assign39430_e44909, ((2.0 * locals.var_q_d2_lnexpnum__blk842_dn4) - locals.var_q_d2_ln__blk836_dn4), ((2.0 * locals.var_q_d2_lnexpnum__blk842_dn6) - locals.var_q_d2_ln__blk836_dn6), ((2.0 * locals.var_q_d2_lnexpnum__blk842_dn7) - locals.var_q_d2_ln__blk836_dn7), ((2.0 * locals.var_q_d2_lnexpnum__blk842_dn8) - locals.var_q_d2_ln__blk836_dn8), ((2.0 * locals.var_q_d2_lnexpnum__blk842_dn9) - locals.var_q_d2_ln__blk836_dn9), );
            locals.var_q_d2_q2__blk845_rv = 0.0;
        }

        if (((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) {
            let assign39440_e44920: f64 = (locals.var_k2__blk933 * locals.var_q_q2_int__blk843);
            let assign39440_e44921: f64 = (locals.var_q_k1q1__blk823 + assign39440_e44920);
            (locals.var_q_qi_int__blk846, locals.var_q_qi_int__blk846_dn4, locals.var_q_qi_int__blk846_dn6, locals.var_q_qi_int__blk846_dn7, locals.var_q_qi_int__blk846_dn8, locals.var_q_qi_int__blk846_dn9, ) = (assign39440_e44921, (locals.var_q_k1q1__blk823_dn4 + ((locals.var_k2__blk933_dn4 * locals.var_q_q2_int__blk843) + (locals.var_k2__blk933 * locals.var_q_q2_int__blk843_dn4))), (locals.var_q_k1q1__blk823_dn6 + ((locals.var_k2__blk933_dn6 * locals.var_q_q2_int__blk843) + (locals.var_k2__blk933 * locals.var_q_q2_int__blk843_dn6))), (locals.var_q_k1q1__blk823_dn7 + ((locals.var_k2__blk933_dn7 * locals.var_q_q2_int__blk843) + (locals.var_k2__blk933 * locals.var_q_q2_int__blk843_dn7))), (locals.var_q_k1q1__blk823_dn8 + ((locals.var_k2__blk933_dn8 * locals.var_q_q2_int__blk843) + (locals.var_k2__blk933 * locals.var_q_q2_int__blk843_dn8))), (locals.var_q_k1q1__blk823_dn9 + ((locals.var_k2__blk933_dn9 * locals.var_q_q2_int__blk843) + (locals.var_k2__blk933 * locals.var_q_q2_int__blk843_dn9))), );
            locals.var_q_qi_int__blk846_rv = 0.0;
        }

        if (((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) {
            let assign39450_e44932: f64 = (locals.var_k2__blk933 * locals.var_q_d1_q2__blk844);
            let assign39450_e44933: f64 = (locals.var_k1__blk932 + assign39450_e44932);
            (locals.var_q_d1_qi__blk847, locals.var_q_d1_qi__blk847_dn4, locals.var_q_d1_qi__blk847_dn6, locals.var_q_d1_qi__blk847_dn7, locals.var_q_d1_qi__blk847_dn8, locals.var_q_d1_qi__blk847_dn9, ) = (assign39450_e44933, (locals.var_k1__blk932_dn4 + ((locals.var_k2__blk933_dn4 * locals.var_q_d1_q2__blk844) + (locals.var_k2__blk933 * locals.var_q_d1_q2__blk844_dn4))), (locals.var_k1__blk932_dn6 + ((locals.var_k2__blk933_dn6 * locals.var_q_d1_q2__blk844) + (locals.var_k2__blk933 * locals.var_q_d1_q2__blk844_dn6))), (locals.var_k1__blk932_dn7 + ((locals.var_k2__blk933_dn7 * locals.var_q_d1_q2__blk844) + (locals.var_k2__blk933 * locals.var_q_d1_q2__blk844_dn7))), (locals.var_k1__blk932_dn8 + ((locals.var_k2__blk933_dn8 * locals.var_q_d1_q2__blk844) + (locals.var_k2__blk933 * locals.var_q_d1_q2__blk844_dn8))), (locals.var_k1__blk932_dn9 + ((locals.var_k2__blk933_dn9 * locals.var_q_d1_q2__blk844) + (locals.var_k2__blk933 * locals.var_q_d1_q2__blk844_dn9))), );
            locals.var_q_d1_qi__blk847_rv = 0.0;
        }

        if (((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) {
            let assign39460_e44943: f64 = (locals.var_k2__blk933 * locals.var_q_d2_q2__blk845);
            (locals.var_q_d2_qi__blk848, locals.var_q_d2_qi__blk848_dn4, locals.var_q_d2_qi__blk848_dn6, locals.var_q_d2_qi__blk848_dn7, locals.var_q_d2_qi__blk848_dn8, locals.var_q_d2_qi__blk848_dn9, ) = (assign39460_e44943, ((locals.var_k2__blk933_dn4 * locals.var_q_d2_q2__blk845) + (locals.var_k2__blk933 * locals.var_q_d2_q2__blk845_dn4)), ((locals.var_k2__blk933_dn6 * locals.var_q_d2_q2__blk845) + (locals.var_k2__blk933 * locals.var_q_d2_q2__blk845_dn6)), ((locals.var_k2__blk933_dn7 * locals.var_q_d2_q2__blk845) + (locals.var_k2__blk933 * locals.var_q_d2_q2__blk845_dn7)), ((locals.var_k2__blk933_dn8 * locals.var_q_d2_q2__blk845) + (locals.var_k2__blk933 * locals.var_q_d2_q2__blk845_dn8)), ((locals.var_k2__blk933_dn9 * locals.var_q_d2_q2__blk845) + (locals.var_k2__blk933 * locals.var_q_d2_q2__blk845_dn9)), );
            locals.var_q_d2_qi__blk848_rv = 0.0;
        }

        if (((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) {
            let assign39470_e44953: f64 = (locals.var_q_qi_int__blk846 * locals.var_q_expnum__blk837);
            let assign39470_e44955: f64 = (assign39470_e44953 - locals.var_q_aexp__blk824);
            (locals.var_q_zero__blk849, locals.var_q_zero__blk849_dn4, locals.var_q_zero__blk849_dn6, locals.var_q_zero__blk849_dn7, locals.var_q_zero__blk849_dn8, locals.var_q_zero__blk849_dn9, ) = (assign39470_e44955, (((locals.var_q_qi_int__blk846_dn4 * locals.var_q_expnum__blk837) + (locals.var_q_qi_int__blk846 * locals.var_q_expnum__blk837_dn4)) - locals.var_q_aexp__blk824_dn4), (((locals.var_q_qi_int__blk846_dn6 * locals.var_q_expnum__blk837) + (locals.var_q_qi_int__blk846 * locals.var_q_expnum__blk837_dn6)) - locals.var_q_aexp__blk824_dn6), (((locals.var_q_qi_int__blk846_dn7 * locals.var_q_expnum__blk837) + (locals.var_q_qi_int__blk846 * locals.var_q_expnum__blk837_dn7)) - locals.var_q_aexp__blk824_dn7), (((locals.var_q_qi_int__blk846_dn8 * locals.var_q_expnum__blk837) + (locals.var_q_qi_int__blk846 * locals.var_q_expnum__blk837_dn8)) - locals.var_q_aexp__blk824_dn8), (((locals.var_q_qi_int__blk846_dn9 * locals.var_q_expnum__blk837) + (locals.var_q_qi_int__blk846 * locals.var_q_expnum__blk837_dn9)) - locals.var_q_aexp__blk824_dn9), );
            locals.var_q_zero__blk849_rv = 0.0;
        }

        if (((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) {
            let assign39480_e44965: f64 = (locals.var_q_d1_qi__blk847 * locals.var_q_expnum__blk837);
            let assign39480_e44968: f64 = (locals.var_q_qi_int__blk846 * locals.var_q_d1_expnum__blk838);
            let assign39480_e44969: f64 = (assign39480_e44965 + assign39480_e44968);
            let assign39480_e44971: f64 = (assign39480_e44969 + locals.var_q_aexp__blk824);
            (locals.var_q_d1_zero__blk850, locals.var_q_d1_zero__blk850_dn4, locals.var_q_d1_zero__blk850_dn6, locals.var_q_d1_zero__blk850_dn7, locals.var_q_d1_zero__blk850_dn8, locals.var_q_d1_zero__blk850_dn9, ) = (assign39480_e44971, ((((locals.var_q_d1_qi__blk847_dn4 * locals.var_q_expnum__blk837) + (locals.var_q_d1_qi__blk847 * locals.var_q_expnum__blk837_dn4)) + ((locals.var_q_qi_int__blk846_dn4 * locals.var_q_d1_expnum__blk838) + (locals.var_q_qi_int__blk846 * locals.var_q_d1_expnum__blk838_dn4))) + locals.var_q_aexp__blk824_dn4), ((((locals.var_q_d1_qi__blk847_dn6 * locals.var_q_expnum__blk837) + (locals.var_q_d1_qi__blk847 * locals.var_q_expnum__blk837_dn6)) + ((locals.var_q_qi_int__blk846_dn6 * locals.var_q_d1_expnum__blk838) + (locals.var_q_qi_int__blk846 * locals.var_q_d1_expnum__blk838_dn6))) + locals.var_q_aexp__blk824_dn6), ((((locals.var_q_d1_qi__blk847_dn7 * locals.var_q_expnum__blk837) + (locals.var_q_d1_qi__blk847 * locals.var_q_expnum__blk837_dn7)) + ((locals.var_q_qi_int__blk846_dn7 * locals.var_q_d1_expnum__blk838) + (locals.var_q_qi_int__blk846 * locals.var_q_d1_expnum__blk838_dn7))) + locals.var_q_aexp__blk824_dn7), ((((locals.var_q_d1_qi__blk847_dn8 * locals.var_q_expnum__blk837) + (locals.var_q_d1_qi__blk847 * locals.var_q_expnum__blk837_dn8)) + ((locals.var_q_qi_int__blk846_dn8 * locals.var_q_d1_expnum__blk838) + (locals.var_q_qi_int__blk846 * locals.var_q_d1_expnum__blk838_dn8))) + locals.var_q_aexp__blk824_dn8), ((((locals.var_q_d1_qi__blk847_dn9 * locals.var_q_expnum__blk837) + (locals.var_q_d1_qi__blk847 * locals.var_q_expnum__blk837_dn9)) + ((locals.var_q_qi_int__blk846_dn9 * locals.var_q_d1_expnum__blk838) + (locals.var_q_qi_int__blk846 * locals.var_q_d1_expnum__blk838_dn9))) + locals.var_q_aexp__blk824_dn9), );
            locals.var_q_d1_zero__blk850_rv = 0.0;
        }

        if (((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) {
            let assign39490_e44981: f64 = (locals.var_q_d2_qi__blk848 * locals.var_q_expnum__blk837);
            let assign39490_e44984: f64 = (2.0 * locals.var_q_d1_qi__blk847);
            let assign39490_e44986: f64 = (assign39490_e44984 * locals.var_q_d1_expnum__blk838);
            let assign39490_e44987: f64 = (assign39490_e44981 + assign39490_e44986);
            let assign39490_e44990: f64 = (locals.var_q_qi_int__blk846 * locals.var_q_d2_expnum__blk839);
            let assign39490_e44991: f64 = (assign39490_e44987 + assign39490_e44990);
            let assign39490_e44993: f64 = (assign39490_e44991 - locals.var_q_aexp__blk824);
            (locals.var_q_d2_zero__blk851, locals.var_q_d2_zero__blk851_dn4, locals.var_q_d2_zero__blk851_dn6, locals.var_q_d2_zero__blk851_dn7, locals.var_q_d2_zero__blk851_dn8, locals.var_q_d2_zero__blk851_dn9, ) = (assign39490_e44993, (((((locals.var_q_d2_qi__blk848_dn4 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qi__blk848 * locals.var_q_expnum__blk837_dn4)) + (((2.0 * locals.var_q_d1_qi__blk847_dn4) * locals.var_q_d1_expnum__blk838) + (assign39490_e44984 * locals.var_q_d1_expnum__blk838_dn4))) + ((locals.var_q_qi_int__blk846_dn4 * locals.var_q_d2_expnum__blk839) + (locals.var_q_qi_int__blk846 * locals.var_q_d2_expnum__blk839_dn4))) - locals.var_q_aexp__blk824_dn4), (((((locals.var_q_d2_qi__blk848_dn6 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qi__blk848 * locals.var_q_expnum__blk837_dn6)) + (((2.0 * locals.var_q_d1_qi__blk847_dn6) * locals.var_q_d1_expnum__blk838) + (assign39490_e44984 * locals.var_q_d1_expnum__blk838_dn6))) + ((locals.var_q_qi_int__blk846_dn6 * locals.var_q_d2_expnum__blk839) + (locals.var_q_qi_int__blk846 * locals.var_q_d2_expnum__blk839_dn6))) - locals.var_q_aexp__blk824_dn6), (((((locals.var_q_d2_qi__blk848_dn7 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qi__blk848 * locals.var_q_expnum__blk837_dn7)) + (((2.0 * locals.var_q_d1_qi__blk847_dn7) * locals.var_q_d1_expnum__blk838) + (assign39490_e44984 * locals.var_q_d1_expnum__blk838_dn7))) + ((locals.var_q_qi_int__blk846_dn7 * locals.var_q_d2_expnum__blk839) + (locals.var_q_qi_int__blk846 * locals.var_q_d2_expnum__blk839_dn7))) - locals.var_q_aexp__blk824_dn7), (((((locals.var_q_d2_qi__blk848_dn8 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qi__blk848 * locals.var_q_expnum__blk837_dn8)) + (((2.0 * locals.var_q_d1_qi__blk847_dn8) * locals.var_q_d1_expnum__blk838) + (assign39490_e44984 * locals.var_q_d1_expnum__blk838_dn8))) + ((locals.var_q_qi_int__blk846_dn8 * locals.var_q_d2_expnum__blk839) + (locals.var_q_qi_int__blk846 * locals.var_q_d2_expnum__blk839_dn8))) - locals.var_q_aexp__blk824_dn8), (((((locals.var_q_d2_qi__blk848_dn9 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qi__blk848 * locals.var_q_expnum__blk837_dn9)) + (((2.0 * locals.var_q_d1_qi__blk847_dn9) * locals.var_q_d1_expnum__blk838) + (assign39490_e44984 * locals.var_q_d1_expnum__blk838_dn9))) + ((locals.var_q_qi_int__blk846_dn9 * locals.var_q_d2_expnum__blk839) + (locals.var_q_qi_int__blk846 * locals.var_q_d2_expnum__blk839_dn9))) - locals.var_q_aexp__blk824_dn9), );
            locals.var_q_d2_zero__blk851_rv = 0.0;
        }

        if (((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) {
            let assign39500_e45003: f64 = (locals.var_q_d1_zero__blk850 * locals.var_q_d1_zero__blk850);
            let assign39500_e45006: f64 = (0.5 * locals.var_q_zero__blk849);
            let assign39500_e45008: f64 = (assign39500_e45006 * locals.var_q_d2_zero__blk851);
            let assign39500_e45009: f64 = (assign39500_e45003 - assign39500_e45008);
            (locals.var_q_temp__blk860, locals.var_q_temp__blk860_dn4, locals.var_q_temp__blk860_dn6, locals.var_q_temp__blk860_dn7, locals.var_q_temp__blk860_dn8, locals.var_q_temp__blk860_dn9, ) = (assign39500_e45009, (((locals.var_q_d1_zero__blk850_dn4 * locals.var_q_d1_zero__blk850) + (locals.var_q_d1_zero__blk850 * locals.var_q_d1_zero__blk850_dn4)) - (((0.5 * locals.var_q_zero__blk849_dn4) * locals.var_q_d2_zero__blk851) + (assign39500_e45006 * locals.var_q_d2_zero__blk851_dn4))), (((locals.var_q_d1_zero__blk850_dn6 * locals.var_q_d1_zero__blk850) + (locals.var_q_d1_zero__blk850 * locals.var_q_d1_zero__blk850_dn6)) - (((0.5 * locals.var_q_zero__blk849_dn6) * locals.var_q_d2_zero__blk851) + (assign39500_e45006 * locals.var_q_d2_zero__blk851_dn6))), (((locals.var_q_d1_zero__blk850_dn7 * locals.var_q_d1_zero__blk850) + (locals.var_q_d1_zero__blk850 * locals.var_q_d1_zero__blk850_dn7)) - (((0.5 * locals.var_q_zero__blk849_dn7) * locals.var_q_d2_zero__blk851) + (assign39500_e45006 * locals.var_q_d2_zero__blk851_dn7))), (((locals.var_q_d1_zero__blk850_dn8 * locals.var_q_d1_zero__blk850) + (locals.var_q_d1_zero__blk850 * locals.var_q_d1_zero__blk850_dn8)) - (((0.5 * locals.var_q_zero__blk849_dn8) * locals.var_q_d2_zero__blk851) + (assign39500_e45006 * locals.var_q_d2_zero__blk851_dn8))), (((locals.var_q_d1_zero__blk850_dn9 * locals.var_q_d1_zero__blk850) + (locals.var_q_d1_zero__blk850 * locals.var_q_d1_zero__blk850_dn9)) - (((0.5 * locals.var_q_zero__blk849_dn9) * locals.var_q_d2_zero__blk851) + (assign39500_e45006 * locals.var_q_d2_zero__blk851_dn9))), );
            locals.var_q_temp__blk860_rv = 0.0;
        }

        if (((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) {
            let assign39510_e45018: f64 = (-locals.var_q_zero__blk849);
            let assign39510_e45020: f64 = (assign39510_e45018 * locals.var_q_d1_zero__blk850);
            let assign39510_e45022: f64 = (assign39510_e45020 * locals.var_q_temp__blk860);
            let assign39510_e45025: f64 = (locals.var_q_temp__blk860 * locals.var_q_temp__blk860);
            let assign39510_e45027: f64 = (assign39510_e45025 + 1e-200);
            let assign39510_e45028: f64 = (assign39510_e45022 / assign39510_e45027);
            (locals.var_q_eps2__blk852, locals.var_q_eps2__blk852_dn4, locals.var_q_eps2__blk852_dn6, locals.var_q_eps2__blk852_dn7, locals.var_q_eps2__blk852_dn8, locals.var_q_eps2__blk852_dn9, ) = (assign39510_e45028, ((((((((-locals.var_q_zero__blk849_dn4) * locals.var_q_d1_zero__blk850) + (assign39510_e45018 * locals.var_q_d1_zero__blk850_dn4)) * locals.var_q_temp__blk860) + (assign39510_e45020 * locals.var_q_temp__blk860_dn4)) * assign39510_e45027) - (assign39510_e45022 * ((locals.var_q_temp__blk860_dn4 * locals.var_q_temp__blk860) + (locals.var_q_temp__blk860 * locals.var_q_temp__blk860_dn4)))) / (assign39510_e45027 * assign39510_e45027)), ((((((((-locals.var_q_zero__blk849_dn6) * locals.var_q_d1_zero__blk850) + (assign39510_e45018 * locals.var_q_d1_zero__blk850_dn6)) * locals.var_q_temp__blk860) + (assign39510_e45020 * locals.var_q_temp__blk860_dn6)) * assign39510_e45027) - (assign39510_e45022 * ((locals.var_q_temp__blk860_dn6 * locals.var_q_temp__blk860) + (locals.var_q_temp__blk860 * locals.var_q_temp__blk860_dn6)))) / (assign39510_e45027 * assign39510_e45027)), ((((((((-locals.var_q_zero__blk849_dn7) * locals.var_q_d1_zero__blk850) + (assign39510_e45018 * locals.var_q_d1_zero__blk850_dn7)) * locals.var_q_temp__blk860) + (assign39510_e45020 * locals.var_q_temp__blk860_dn7)) * assign39510_e45027) - (assign39510_e45022 * ((locals.var_q_temp__blk860_dn7 * locals.var_q_temp__blk860) + (locals.var_q_temp__blk860 * locals.var_q_temp__blk860_dn7)))) / (assign39510_e45027 * assign39510_e45027)), ((((((((-locals.var_q_zero__blk849_dn8) * locals.var_q_d1_zero__blk850) + (assign39510_e45018 * locals.var_q_d1_zero__blk850_dn8)) * locals.var_q_temp__blk860) + (assign39510_e45020 * locals.var_q_temp__blk860_dn8)) * assign39510_e45027) - (assign39510_e45022 * ((locals.var_q_temp__blk860_dn8 * locals.var_q_temp__blk860) + (locals.var_q_temp__blk860 * locals.var_q_temp__blk860_dn8)))) / (assign39510_e45027 * assign39510_e45027)), ((((((((-locals.var_q_zero__blk849_dn9) * locals.var_q_d1_zero__blk850) + (assign39510_e45018 * locals.var_q_d1_zero__blk850_dn9)) * locals.var_q_temp__blk860) + (assign39510_e45020 * locals.var_q_temp__blk860_dn9)) * assign39510_e45027) - (assign39510_e45022 * ((locals.var_q_temp__blk860_dn9 * locals.var_q_temp__blk860) + (locals.var_q_temp__blk860 * locals.var_q_temp__blk860_dn9)))) / (assign39510_e45027 * assign39510_e45027)), );
            locals.var_q_eps2__blk852_rv = 0.0;
        }

        if (((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) {
            let assign39520_e45038: f64 = (locals.var_q1d__blk1001 + locals.var_q_eps2__blk852);
            (locals.var_q1d__blk1001, locals.var_q1d__blk1001_dn4, locals.var_q1d__blk1001_dn6, locals.var_q1d__blk1001_dn7, locals.var_q1d__blk1001_dn8, locals.var_q1d__blk1001_dn9, ) = (assign39520_e45038, (locals.var_q1d__blk1001_dn4 + locals.var_q_eps2__blk852_dn4), (locals.var_q1d__blk1001_dn6 + locals.var_q_eps2__blk852_dn6), (locals.var_q1d__blk1001_dn7 + locals.var_q_eps2__blk852_dn7), (locals.var_q1d__blk1001_dn8 + locals.var_q_eps2__blk852_dn8), (locals.var_q1d__blk1001_dn9 + locals.var_q_eps2__blk852_dn9), );
            locals.var_q1d__blk1001_rv = 0.0;
        }

        if (locals.var_guard1080 != 0.0) {
            let assign39530_e45044: f64 = (locals.var_k1__blk932 * locals.var_q1d__blk1001);
            (locals.var_k1q1d__blk1004, locals.var_k1q1d__blk1004_dn4, locals.var_k1q1d__blk1004_dn6, locals.var_k1q1d__blk1004_dn7, locals.var_k1q1d__blk1004_dn8, locals.var_k1q1d__blk1004_dn9, ) = (assign39530_e45044, ((locals.var_k1__blk932_dn4 * locals.var_q1d__blk1001) + (locals.var_k1__blk932 * locals.var_q1d__blk1001_dn4)), ((locals.var_k1__blk932_dn6 * locals.var_q1d__blk1001) + (locals.var_k1__blk932 * locals.var_q1d__blk1001_dn6)), ((locals.var_k1__blk932_dn7 * locals.var_q1d__blk1001) + (locals.var_k1__blk932 * locals.var_q1d__blk1001_dn7)), ((locals.var_k1__blk932_dn8 * locals.var_q1d__blk1001) + (locals.var_k1__blk932 * locals.var_q1d__blk1001_dn8)), ((locals.var_k1__blk932_dn9 * locals.var_q1d__blk1001) + (locals.var_k1__blk932 * locals.var_q1d__blk1001_dn9)), );
            locals.var_k1q1d__blk1004_rv = 0.0;
        }

        let assign39540_e45049: f64 = (locals.var_xg1x__blk930 - locals.var_q1d__blk1001);
        let assign39540_e45051: f64 = (assign39540_e45049 - locals.var_xdeff__blk1000);
        let assign39540_e45053: f64 = if assign39540_e45051 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard1205 = assign39540_e45053;
        locals.var_guard1205_rv = 0.0;

        if ((locals.var_guard1080 != 0.0) && (locals.var_guard1205 != 0.0)) {
            let assign39550_e45059: f64 = (locals.var_xg1x__blk930 - locals.var_q1d__blk1001);
            let assign39550_e45061: f64 = (assign39550_e45059 - locals.var_xdeff__blk1000);
            let assign39550_e45062: f64 = (assign39550_e45061).exp();
            (locals.var_q_temp1__blk814, locals.var_q_temp1__blk814_dn4, locals.var_q_temp1__blk814_dn6, locals.var_q_temp1__blk814_dn7, locals.var_q_temp1__blk814_dn8, locals.var_q_temp1__blk814_dn9, ) = (assign39550_e45062, (assign39550_e45062 * ((locals.var_xg1x__blk930_dn4 - locals.var_q1d__blk1001_dn4) - locals.var_xdeff__blk1000_dn4)), (assign39550_e45062 * ((locals.var_xg1x__blk930_dn6 - locals.var_q1d__blk1001_dn6) - locals.var_xdeff__blk1000_dn6)), (assign39550_e45062 * ((locals.var_xg1x__blk930_dn7 - locals.var_q1d__blk1001_dn7) - locals.var_xdeff__blk1000_dn7)), (assign39550_e45062 * ((locals.var_xg1x__blk930_dn8 - locals.var_q1d__blk1001_dn8) - locals.var_xdeff__blk1000_dn8)), (assign39550_e45062 * ((locals.var_xg1x__blk930_dn9 - locals.var_q1d__blk1001_dn9) - locals.var_xdeff__blk1000_dn9)), );
            locals.var_q_temp1__blk814_rv = 0.0;
        }

        if ((locals.var_guard1080 != 0.0) && (locals.var_guard1205 == 0.0)) {
            let assign39560_e45073: f64 = (locals.var_xg1x__blk930 - locals.var_q1d__blk1001);
            let assign39560_e45075: f64 = (assign39560_e45073 - locals.var_xdeff__blk1000);
            let assign39560_e45077: f64 = (assign39560_e45075 - 80.0);
            let assign39560_e45082: f64 = (locals.var_xg1x__blk930 - locals.var_q1d__blk1001);
            let assign39560_e45084: f64 = (assign39560_e45082 - locals.var_xdeff__blk1000);
            let assign39560_e45086: f64 = (assign39560_e45084 - 80.0);
            let assign39560_e45087: f64 = (0.5 * assign39560_e45086);
            let assign39560_e45091: f64 = (locals.var_xg1x__blk930 - locals.var_q1d__blk1001);
            let assign39560_e45093: f64 = (assign39560_e45091 - locals.var_xdeff__blk1000);
            let assign39560_e45095: f64 = (assign39560_e45093 - 80.0);
            let assign39560_e45097: f64 = (assign39560_e45095 * 0.3333333333333);
            let assign39560_e45098: f64 = (1.0 + assign39560_e45097);
            let assign39560_e45099: f64 = (assign39560_e45087 * assign39560_e45098);
            let assign39560_e45100: f64 = (1.0 + assign39560_e45099);
            let assign39560_e45101: f64 = (assign39560_e45077 * assign39560_e45100);
            let assign39560_e45102: f64 = (1.0 + assign39560_e45101);
            let assign39560_e45103: f64 = (5.54062e34 * assign39560_e45102);
            (locals.var_q_temp1__blk814, locals.var_q_temp1__blk814_dn4, locals.var_q_temp1__blk814_dn6, locals.var_q_temp1__blk814_dn7, locals.var_q_temp1__blk814_dn8, locals.var_q_temp1__blk814_dn9, ) = (assign39560_e45103, (5.54062e34 * ((((locals.var_xg1x__blk930_dn4 - locals.var_q1d__blk1001_dn4) - locals.var_xdeff__blk1000_dn4) * assign39560_e45100) + (assign39560_e45077 * (((0.5 * ((locals.var_xg1x__blk930_dn4 - locals.var_q1d__blk1001_dn4) - locals.var_xdeff__blk1000_dn4)) * assign39560_e45098) + (assign39560_e45087 * (((locals.var_xg1x__blk930_dn4 - locals.var_q1d__blk1001_dn4) - locals.var_xdeff__blk1000_dn4) * 0.3333333333333)))))), (5.54062e34 * ((((locals.var_xg1x__blk930_dn6 - locals.var_q1d__blk1001_dn6) - locals.var_xdeff__blk1000_dn6) * assign39560_e45100) + (assign39560_e45077 * (((0.5 * ((locals.var_xg1x__blk930_dn6 - locals.var_q1d__blk1001_dn6) - locals.var_xdeff__blk1000_dn6)) * assign39560_e45098) + (assign39560_e45087 * (((locals.var_xg1x__blk930_dn6 - locals.var_q1d__blk1001_dn6) - locals.var_xdeff__blk1000_dn6) * 0.3333333333333)))))), (5.54062e34 * ((((locals.var_xg1x__blk930_dn7 - locals.var_q1d__blk1001_dn7) - locals.var_xdeff__blk1000_dn7) * assign39560_e45100) + (assign39560_e45077 * (((0.5 * ((locals.var_xg1x__blk930_dn7 - locals.var_q1d__blk1001_dn7) - locals.var_xdeff__blk1000_dn7)) * assign39560_e45098) + (assign39560_e45087 * (((locals.var_xg1x__blk930_dn7 - locals.var_q1d__blk1001_dn7) - locals.var_xdeff__blk1000_dn7) * 0.3333333333333)))))), (5.54062e34 * ((((locals.var_xg1x__blk930_dn8 - locals.var_q1d__blk1001_dn8) - locals.var_xdeff__blk1000_dn8) * assign39560_e45100) + (assign39560_e45077 * (((0.5 * ((locals.var_xg1x__blk930_dn8 - locals.var_q1d__blk1001_dn8) - locals.var_xdeff__blk1000_dn8)) * assign39560_e45098) + (assign39560_e45087 * (((locals.var_xg1x__blk930_dn8 - locals.var_q1d__blk1001_dn8) - locals.var_xdeff__blk1000_dn8) * 0.3333333333333)))))), (5.54062e34 * ((((locals.var_xg1x__blk930_dn9 - locals.var_q1d__blk1001_dn9) - locals.var_xdeff__blk1000_dn9) * assign39560_e45100) + (assign39560_e45077 * (((0.5 * ((locals.var_xg1x__blk930_dn9 - locals.var_q1d__blk1001_dn9) - locals.var_xdeff__blk1000_dn9)) * assign39560_e45098) + (assign39560_e45087 * (((locals.var_xg1x__blk930_dn9 - locals.var_q1d__blk1001_dn9) - locals.var_xdeff__blk1000_dn9) * 0.3333333333333)))))), );
            locals.var_q_temp1__blk814_rv = 0.0;
        }

        if (locals.var_guard1080 != 0.0) {
            let assign39570_e45109: f64 = (locals.var_a0__blk905 * locals.var_q_temp1__blk814);
            (locals.var_aexp1d__blk1007, locals.var_aexp1d__blk1007_dn4, locals.var_aexp1d__blk1007_dn6, locals.var_aexp1d__blk1007_dn7, locals.var_aexp1d__blk1007_dn8, locals.var_aexp1d__blk1007_dn9, ) = (assign39570_e45109, ((locals.var_a0__blk905_dn4 * locals.var_q_temp1__blk814) + (locals.var_a0__blk905 * locals.var_q_temp1__blk814_dn4)), ((locals.var_a0__blk905_dn6 * locals.var_q_temp1__blk814) + (locals.var_a0__blk905 * locals.var_q_temp1__blk814_dn6)), ((locals.var_a0__blk905_dn7 * locals.var_q_temp1__blk814) + (locals.var_a0__blk905 * locals.var_q_temp1__blk814_dn7)), ((locals.var_a0__blk905_dn8 * locals.var_q_temp1__blk814) + (locals.var_a0__blk905 * locals.var_q_temp1__blk814_dn8)), ((locals.var_a0__blk905_dn9 * locals.var_q_temp1__blk814) + (locals.var_a0__blk905 * locals.var_q_temp1__blk814_dn9)), );
            locals.var_aexp1d__blk1007_rv = 0.0;
        }

        if (locals.var_guard1080 != 0.0) {
            let assign39580_e45115: f64 = (locals.var_k1q1d__blk1004 * locals.var_k1q1d__blk1004);
            let assign39580_e45117: f64 = (assign39580_e45115 - locals.var_aexp1d__blk1007);
            (locals.var_qsqd__blk1006, locals.var_qsqd__blk1006_dn4, locals.var_qsqd__blk1006_dn6, locals.var_qsqd__blk1006_dn7, locals.var_qsqd__blk1006_dn8, locals.var_qsqd__blk1006_dn9, ) = (assign39580_e45117, (((locals.var_k1q1d__blk1004_dn4 * locals.var_k1q1d__blk1004) + (locals.var_k1q1d__blk1004 * locals.var_k1q1d__blk1004_dn4)) - locals.var_aexp1d__blk1007_dn4), (((locals.var_k1q1d__blk1004_dn6 * locals.var_k1q1d__blk1004) + (locals.var_k1q1d__blk1004 * locals.var_k1q1d__blk1004_dn6)) - locals.var_aexp1d__blk1007_dn6), (((locals.var_k1q1d__blk1004_dn7 * locals.var_k1q1d__blk1004) + (locals.var_k1q1d__blk1004 * locals.var_k1q1d__blk1004_dn7)) - locals.var_aexp1d__blk1007_dn7), (((locals.var_k1q1d__blk1004_dn8 * locals.var_k1q1d__blk1004) + (locals.var_k1q1d__blk1004 * locals.var_k1q1d__blk1004_dn8)) - locals.var_aexp1d__blk1007_dn8), (((locals.var_k1q1d__blk1004_dn9 * locals.var_k1q1d__blk1004) + (locals.var_k1q1d__blk1004 * locals.var_k1q1d__blk1004_dn9)) - locals.var_aexp1d__blk1007_dn9), );
            locals.var_qsqd__blk1006_rv = 0.0;
        }

        let assign39590_e45122: f64 = if locals.var_aexp1d__blk1007 <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1206 = assign39590_e45122;
        locals.var_guard1206_rv = 0.0;

        if ((locals.var_guard1080 != 0.0) && (locals.var_guard1206 != 0.0)) {
            (locals.var_qid__blk1003, locals.var_qid__blk1003_dn4, locals.var_qid__blk1003_dn6, locals.var_qid__blk1003_dn7, locals.var_qid__blk1003_dn8, locals.var_qid__blk1003_dn9, ) = (1e-80, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_qid__blk1003_rv = 0.0;
        }

        if ((locals.var_guard1080 != 0.0) && (locals.var_guard1206 != 0.0)) {
            let assign39610_e45134: f64 = (locals.var_qid__blk1003 - locals.var_k1q1d__blk1004);
            (locals.var_k2q2d__blk1005, locals.var_k2q2d__blk1005_dn4, locals.var_k2q2d__blk1005_dn6, locals.var_k2q2d__blk1005_dn7, locals.var_k2q2d__blk1005_dn8, locals.var_k2q2d__blk1005_dn9, ) = (assign39610_e45134, (locals.var_qid__blk1003_dn4 - locals.var_k1q1d__blk1004_dn4), (locals.var_qid__blk1003_dn6 - locals.var_k1q1d__blk1004_dn6), (locals.var_qid__blk1003_dn7 - locals.var_k1q1d__blk1004_dn7), (locals.var_qid__blk1003_dn8 - locals.var_k1q1d__blk1004_dn8), (locals.var_qid__blk1003_dn9 - locals.var_k1q1d__blk1004_dn9), );
            locals.var_k2q2d__blk1005_rv = 0.0;
        }

        if ((locals.var_guard1080 != 0.0) && (locals.var_guard1206 != 0.0)) {
            let assign39620_e45142: f64 = (locals.var_k2q2d__blk1005 / locals.var_k2__blk933);
            (locals.var_q2d__blk1002, locals.var_q2d__blk1002_dn4, locals.var_q2d__blk1002_dn6, locals.var_q2d__blk1002_dn7, locals.var_q2d__blk1002_dn8, locals.var_q2d__blk1002_dn9, ) = (assign39620_e45142, (((locals.var_k2q2d__blk1005_dn4 * locals.var_k2__blk933) - (locals.var_k2q2d__blk1005 * locals.var_k2__blk933_dn4)) / (locals.var_k2__blk933 * locals.var_k2__blk933)), (((locals.var_k2q2d__blk1005_dn6 * locals.var_k2__blk933) - (locals.var_k2q2d__blk1005 * locals.var_k2__blk933_dn6)) / (locals.var_k2__blk933 * locals.var_k2__blk933)), (((locals.var_k2q2d__blk1005_dn7 * locals.var_k2__blk933) - (locals.var_k2q2d__blk1005 * locals.var_k2__blk933_dn7)) / (locals.var_k2__blk933 * locals.var_k2__blk933)), (((locals.var_k2q2d__blk1005_dn8 * locals.var_k2__blk933) - (locals.var_k2q2d__blk1005 * locals.var_k2__blk933_dn8)) / (locals.var_k2__blk933 * locals.var_k2__blk933)), (((locals.var_k2q2d__blk1005_dn9 * locals.var_k2__blk933) - (locals.var_k2q2d__blk1005 * locals.var_k2__blk933_dn9)) / (locals.var_k2__blk933 * locals.var_k2__blk933)), );
            locals.var_q2d__blk1002_rv = 0.0;
        }

        let assign39630_e45147: f64 = (-0.005);
        let assign39630_e45148: f64 = if locals.var_qsqd__blk1006 < assign39630_e45147 { 1.0 } else { 0.0 };
        locals.var_guard1207 = assign39630_e45148;
        locals.var_guard1207_rv = 0.0;

        if (((locals.var_guard1080 != 0.0) && (locals.var_guard1206 == 0.0)) && (locals.var_guard1207 != 0.0)) {
            let assign39640_e45156: f64 = (locals.var_qsqd__blk1006).abs();
            let assign39640_e45157: f64 = (assign39640_e45156).sqrt();
            (locals.var_q_rac_qsq__blk828, locals.var_q_rac_qsq__blk828_dn4, locals.var_q_rac_qsq__blk828_dn6, locals.var_q_rac_qsq__blk828_dn7, locals.var_q_rac_qsq__blk828_dn8, locals.var_q_rac_qsq__blk828_dn9, ) = (assign39640_e45157, (if locals.var_qsqd__blk1006 >= 0.0 { locals.var_qsqd__blk1006_dn4 } else { (-locals.var_qsqd__blk1006_dn4) } / (2.0 * assign39640_e45157)), (if locals.var_qsqd__blk1006 >= 0.0 { locals.var_qsqd__blk1006_dn6 } else { (-locals.var_qsqd__blk1006_dn6) } / (2.0 * assign39640_e45157)), (if locals.var_qsqd__blk1006 >= 0.0 { locals.var_qsqd__blk1006_dn7 } else { (-locals.var_qsqd__blk1006_dn7) } / (2.0 * assign39640_e45157)), (if locals.var_qsqd__blk1006 >= 0.0 { locals.var_qsqd__blk1006_dn8 } else { (-locals.var_qsqd__blk1006_dn8) } / (2.0 * assign39640_e45157)), (if locals.var_qsqd__blk1006 >= 0.0 { locals.var_qsqd__blk1006_dn9 } else { (-locals.var_qsqd__blk1006_dn9) } / (2.0 * assign39640_e45157)), );
            locals.var_q_rac_qsq__blk828_rv = 0.0;
        }

        if (((locals.var_guard1080 != 0.0) && (locals.var_guard1206 == 0.0)) && (locals.var_guard1207 != 0.0)) {
            let assign39650_e45169: f64 = (0.5 * locals.var_q_rac_qsq__blk828);
            let assign39650_e45170: f64 = (assign39650_e45169).tan();
            let assign39650_e45171: f64 = (locals.var_q_rac_qsq__blk828 / assign39650_e45170);
            (locals.var_q_qcoth__blk829, locals.var_q_qcoth__blk829_dn4, locals.var_q_qcoth__blk829_dn6, locals.var_q_qcoth__blk829_dn7, locals.var_q_qcoth__blk829_dn8, locals.var_q_qcoth__blk829_dn9, ) = (assign39650_e45171, (((locals.var_q_rac_qsq__blk828_dn4 * assign39650_e45170) - (locals.var_q_rac_qsq__blk828 * ((0.5 * locals.var_q_rac_qsq__blk828_dn4) / ((assign39650_e45169).cos() * (assign39650_e45169).cos())))) / (assign39650_e45170 * assign39650_e45170)), (((locals.var_q_rac_qsq__blk828_dn6 * assign39650_e45170) - (locals.var_q_rac_qsq__blk828 * ((0.5 * locals.var_q_rac_qsq__blk828_dn6) / ((assign39650_e45169).cos() * (assign39650_e45169).cos())))) / (assign39650_e45170 * assign39650_e45170)), (((locals.var_q_rac_qsq__blk828_dn7 * assign39650_e45170) - (locals.var_q_rac_qsq__blk828 * ((0.5 * locals.var_q_rac_qsq__blk828_dn7) / ((assign39650_e45169).cos() * (assign39650_e45169).cos())))) / (assign39650_e45170 * assign39650_e45170)), (((locals.var_q_rac_qsq__blk828_dn8 * assign39650_e45170) - (locals.var_q_rac_qsq__blk828 * ((0.5 * locals.var_q_rac_qsq__blk828_dn8) / ((assign39650_e45169).cos() * (assign39650_e45169).cos())))) / (assign39650_e45170 * assign39650_e45170)), (((locals.var_q_rac_qsq__blk828_dn9 * assign39650_e45170) - (locals.var_q_rac_qsq__blk828 * ((0.5 * locals.var_q_rac_qsq__blk828_dn9) / ((assign39650_e45169).cos() * (assign39650_e45169).cos())))) / (assign39650_e45170 * assign39650_e45170)), );
            locals.var_q_qcoth__blk829_rv = 0.0;
        }

        let assign39660_e45176: f64 = if locals.var_qsqd__blk1006 > 0.005 { 1.0 } else { 0.0 };
        locals.var_guard1208 = assign39660_e45176;
        locals.var_guard1208_rv = 0.0;

        if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1206 == 0.0)) && (locals.var_guard1207 == 0.0)) && (locals.var_guard1208 != 0.0)) {
            let assign39670_e45187: f64 = (locals.var_qsqd__blk1006).abs();
            let assign39670_e45188: f64 = (assign39670_e45187).sqrt();
            (locals.var_q_rac_qsq__blk828, locals.var_q_rac_qsq__blk828_dn4, locals.var_q_rac_qsq__blk828_dn6, locals.var_q_rac_qsq__blk828_dn7, locals.var_q_rac_qsq__blk828_dn8, locals.var_q_rac_qsq__blk828_dn9, ) = (assign39670_e45188, (if locals.var_qsqd__blk1006 >= 0.0 { locals.var_qsqd__blk1006_dn4 } else { (-locals.var_qsqd__blk1006_dn4) } / (2.0 * assign39670_e45188)), (if locals.var_qsqd__blk1006 >= 0.0 { locals.var_qsqd__blk1006_dn6 } else { (-locals.var_qsqd__blk1006_dn6) } / (2.0 * assign39670_e45188)), (if locals.var_qsqd__blk1006 >= 0.0 { locals.var_qsqd__blk1006_dn7 } else { (-locals.var_qsqd__blk1006_dn7) } / (2.0 * assign39670_e45188)), (if locals.var_qsqd__blk1006 >= 0.0 { locals.var_qsqd__blk1006_dn8 } else { (-locals.var_qsqd__blk1006_dn8) } / (2.0 * assign39670_e45188)), (if locals.var_qsqd__blk1006 >= 0.0 { locals.var_qsqd__blk1006_dn9 } else { (-locals.var_qsqd__blk1006_dn9) } / (2.0 * assign39670_e45188)), );
            locals.var_q_rac_qsq__blk828_rv = 0.0;
        }

        if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1206 == 0.0)) && (locals.var_guard1207 == 0.0)) && (locals.var_guard1208 != 0.0)) {
            let assign39680_e45201: f64 = (-locals.var_q_rac_qsq__blk828);
            let assign39680_e45202: f64 = (assign39680_e45201).exp();
            (locals.var_q_invexpq__blk831, locals.var_q_invexpq__blk831_dn4, locals.var_q_invexpq__blk831_dn6, locals.var_q_invexpq__blk831_dn7, locals.var_q_invexpq__blk831_dn8, locals.var_q_invexpq__blk831_dn9, ) = (assign39680_e45202, (assign39680_e45202 * (-locals.var_q_rac_qsq__blk828_dn4)), (assign39680_e45202 * (-locals.var_q_rac_qsq__blk828_dn6)), (assign39680_e45202 * (-locals.var_q_rac_qsq__blk828_dn7)), (assign39680_e45202 * (-locals.var_q_rac_qsq__blk828_dn8)), (assign39680_e45202 * (-locals.var_q_rac_qsq__blk828_dn9)), );
            locals.var_q_invexpq__blk831_rv = 0.0;
        }

    }

    pub(super) fn stamp_reactive_block_57(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1206 == 0.0)) && (locals.var_guard1207 == 0.0)) && (locals.var_guard1208 != 0.0)) {
            let assign39690_e45217: f64 = (1.0 + locals.var_q_invexpq__blk831);
            let assign39690_e45218: f64 = (locals.var_q_rac_qsq__blk828 * assign39690_e45217);
            let assign39690_e45221: f64 = (1.0 - locals.var_q_invexpq__blk831);
            let assign39690_e45222: f64 = (assign39690_e45218 / assign39690_e45221);
            (locals.var_q_qcoth__blk829, locals.var_q_qcoth__blk829_dn4, locals.var_q_qcoth__blk829_dn6, locals.var_q_qcoth__blk829_dn7, locals.var_q_qcoth__blk829_dn8, locals.var_q_qcoth__blk829_dn9, ) = (assign39690_e45222, (((((locals.var_q_rac_qsq__blk828_dn4 * assign39690_e45217) + (locals.var_q_rac_qsq__blk828 * locals.var_q_invexpq__blk831_dn4)) * assign39690_e45221) - (assign39690_e45218 * (-locals.var_q_invexpq__blk831_dn4))) / (assign39690_e45221 * assign39690_e45221)), (((((locals.var_q_rac_qsq__blk828_dn6 * assign39690_e45217) + (locals.var_q_rac_qsq__blk828 * locals.var_q_invexpq__blk831_dn6)) * assign39690_e45221) - (assign39690_e45218 * (-locals.var_q_invexpq__blk831_dn6))) / (assign39690_e45221 * assign39690_e45221)), (((((locals.var_q_rac_qsq__blk828_dn7 * assign39690_e45217) + (locals.var_q_rac_qsq__blk828 * locals.var_q_invexpq__blk831_dn7)) * assign39690_e45221) - (assign39690_e45218 * (-locals.var_q_invexpq__blk831_dn7))) / (assign39690_e45221 * assign39690_e45221)), (((((locals.var_q_rac_qsq__blk828_dn8 * assign39690_e45217) + (locals.var_q_rac_qsq__blk828 * locals.var_q_invexpq__blk831_dn8)) * assign39690_e45221) - (assign39690_e45218 * (-locals.var_q_invexpq__blk831_dn8))) / (assign39690_e45221 * assign39690_e45221)), (((((locals.var_q_rac_qsq__blk828_dn9 * assign39690_e45217) + (locals.var_q_rac_qsq__blk828 * locals.var_q_invexpq__blk831_dn9)) * assign39690_e45221) - (assign39690_e45218 * (-locals.var_q_invexpq__blk831_dn9))) / (assign39690_e45221 * assign39690_e45221)), );
            locals.var_q_qcoth__blk829_rv = 0.0;
        }

        if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1206 == 0.0)) && (locals.var_guard1207 == 0.0)) && (locals.var_guard1208 == 0.0)) {
            let assign39700_e45238: f64 = (locals.var_qsqd__blk1006 * 0.1666666666667);
            let assign39700_e45242: f64 = (locals.var_qsqd__blk1006 * 0.0166666666667);
            let assign39700_e45246: f64 = (locals.var_qsqd__blk1006 * 0.0238095238095);
            let assign39700_e45247: f64 = (1.0 - assign39700_e45246);
            let assign39700_e45248: f64 = (assign39700_e45242 * assign39700_e45247);
            let assign39700_e45249: f64 = (1.0 - assign39700_e45248);
            let assign39700_e45250: f64 = (assign39700_e45238 * assign39700_e45249);
            let assign39700_e45251: f64 = (2.0 + assign39700_e45250);
            (locals.var_q_qcoth__blk829, locals.var_q_qcoth__blk829_dn4, locals.var_q_qcoth__blk829_dn6, locals.var_q_qcoth__blk829_dn7, locals.var_q_qcoth__blk829_dn8, locals.var_q_qcoth__blk829_dn9, ) = (assign39700_e45251, (((locals.var_qsqd__blk1006_dn4 * 0.1666666666667) * assign39700_e45249) + (assign39700_e45238 * (-(((locals.var_qsqd__blk1006_dn4 * 0.0166666666667) * assign39700_e45247) + (assign39700_e45242 * (-(locals.var_qsqd__blk1006_dn4 * 0.0238095238095))))))), (((locals.var_qsqd__blk1006_dn6 * 0.1666666666667) * assign39700_e45249) + (assign39700_e45238 * (-(((locals.var_qsqd__blk1006_dn6 * 0.0166666666667) * assign39700_e45247) + (assign39700_e45242 * (-(locals.var_qsqd__blk1006_dn6 * 0.0238095238095))))))), (((locals.var_qsqd__blk1006_dn7 * 0.1666666666667) * assign39700_e45249) + (assign39700_e45238 * (-(((locals.var_qsqd__blk1006_dn7 * 0.0166666666667) * assign39700_e45247) + (assign39700_e45242 * (-(locals.var_qsqd__blk1006_dn7 * 0.0238095238095))))))), (((locals.var_qsqd__blk1006_dn8 * 0.1666666666667) * assign39700_e45249) + (assign39700_e45238 * (-(((locals.var_qsqd__blk1006_dn8 * 0.0166666666667) * assign39700_e45247) + (assign39700_e45242 * (-(locals.var_qsqd__blk1006_dn8 * 0.0238095238095))))))), (((locals.var_qsqd__blk1006_dn9 * 0.1666666666667) * assign39700_e45249) + (assign39700_e45238 * (-(((locals.var_qsqd__blk1006_dn9 * 0.0166666666667) * assign39700_e45247) + (assign39700_e45242 * (-(locals.var_qsqd__blk1006_dn9 * 0.0238095238095))))))), );
            locals.var_q_qcoth__blk829_rv = 0.0;
        }

        let assign39710_e45256: f64 = (1.01 * locals.var_k1q1d__blk1004);
        let assign39710_e45258: f64 = (assign39710_e45256 + locals.var_q_qcoth__blk829);
        let assign39710_e45260: f64 = if assign39710_e45258 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1209 = assign39710_e45260;
        locals.var_guard1209_rv = 0.0;

        if (((locals.var_guard1080 != 0.0) && (locals.var_guard1206 == 0.0)) && (locals.var_guard1209 != 0.0)) {
            let assign39720_e45269: f64 = (locals.var_k1q1d__blk1004 + locals.var_q_qcoth__blk829);
            (locals.var_q_temp1__blk814, locals.var_q_temp1__blk814_dn4, locals.var_q_temp1__blk814_dn6, locals.var_q_temp1__blk814_dn7, locals.var_q_temp1__blk814_dn8, locals.var_q_temp1__blk814_dn9, ) = (assign39720_e45269, (locals.var_k1q1d__blk1004_dn4 + locals.var_q_qcoth__blk829_dn4), (locals.var_k1q1d__blk1004_dn6 + locals.var_q_qcoth__blk829_dn6), (locals.var_k1q1d__blk1004_dn7 + locals.var_q_qcoth__blk829_dn7), (locals.var_k1q1d__blk1004_dn8 + locals.var_q_qcoth__blk829_dn8), (locals.var_k1q1d__blk1004_dn9 + locals.var_q_qcoth__blk829_dn9), );
            locals.var_q_temp1__blk814_rv = 0.0;
        }

        let assign39730_e45274: f64 = (locals.var_aexp1d__blk1007 * locals.var_k1q1d__blk1004);
        let assign39730_e45277: f64 = (0.9 * locals.var_k1q1d__blk1004);
        let assign39730_e45279: f64 = (assign39730_e45277 * locals.var_k1q1d__blk1004);
        let assign39730_e45281: f64 = (assign39730_e45279 * locals.var_q_temp1__blk814);
        let assign39730_e45282: f64 = if assign39730_e45274 < assign39730_e45281 { 1.0 } else { 0.0 };
        locals.var_guard1210 = assign39730_e45282;
        locals.var_guard1210_rv = 0.0;

        if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1206 == 0.0)) && (locals.var_guard1209 != 0.0)) && (locals.var_guard1210 != 0.0)) {
            let assign39740_e45293: f64 = (locals.var_aexp1d__blk1007 / locals.var_q_temp1__blk814);
            let assign39740_e45295: f64 = (assign39740_e45293 + 1e-80);
            (locals.var_qid__blk1003, locals.var_qid__blk1003_dn4, locals.var_qid__blk1003_dn6, locals.var_qid__blk1003_dn7, locals.var_qid__blk1003_dn8, locals.var_qid__blk1003_dn9, ) = (assign39740_e45295, (((locals.var_aexp1d__blk1007_dn4 * locals.var_q_temp1__blk814) - (locals.var_aexp1d__blk1007 * locals.var_q_temp1__blk814_dn4)) / (locals.var_q_temp1__blk814 * locals.var_q_temp1__blk814)), (((locals.var_aexp1d__blk1007_dn6 * locals.var_q_temp1__blk814) - (locals.var_aexp1d__blk1007 * locals.var_q_temp1__blk814_dn6)) / (locals.var_q_temp1__blk814 * locals.var_q_temp1__blk814)), (((locals.var_aexp1d__blk1007_dn7 * locals.var_q_temp1__blk814) - (locals.var_aexp1d__blk1007 * locals.var_q_temp1__blk814_dn7)) / (locals.var_q_temp1__blk814 * locals.var_q_temp1__blk814)), (((locals.var_aexp1d__blk1007_dn8 * locals.var_q_temp1__blk814) - (locals.var_aexp1d__blk1007 * locals.var_q_temp1__blk814_dn8)) / (locals.var_q_temp1__blk814 * locals.var_q_temp1__blk814)), (((locals.var_aexp1d__blk1007_dn9 * locals.var_q_temp1__blk814) - (locals.var_aexp1d__blk1007 * locals.var_q_temp1__blk814_dn9)) / (locals.var_q_temp1__blk814 * locals.var_q_temp1__blk814)), );
            locals.var_qid__blk1003_rv = 0.0;
        }

        if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1206 == 0.0)) && (locals.var_guard1209 != 0.0)) && (locals.var_guard1210 != 0.0)) {
            let assign39750_e45308: f64 = (locals.var_qid__blk1003 - locals.var_k1q1d__blk1004);
            (locals.var_k2q2d__blk1005, locals.var_k2q2d__blk1005_dn4, locals.var_k2q2d__blk1005_dn6, locals.var_k2q2d__blk1005_dn7, locals.var_k2q2d__blk1005_dn8, locals.var_k2q2d__blk1005_dn9, ) = (assign39750_e45308, (locals.var_qid__blk1003_dn4 - locals.var_k1q1d__blk1004_dn4), (locals.var_qid__blk1003_dn6 - locals.var_k1q1d__blk1004_dn6), (locals.var_qid__blk1003_dn7 - locals.var_k1q1d__blk1004_dn7), (locals.var_qid__blk1003_dn8 - locals.var_k1q1d__blk1004_dn8), (locals.var_qid__blk1003_dn9 - locals.var_k1q1d__blk1004_dn9), );
            locals.var_k2q2d__blk1005_rv = 0.0;
        }

        if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1206 == 0.0)) && (locals.var_guard1209 != 0.0)) && (locals.var_guard1210 != 0.0)) {
            let assign39760_e45321: f64 = (locals.var_k2q2d__blk1005 / locals.var_k2__blk933);
            (locals.var_q2d__blk1002, locals.var_q2d__blk1002_dn4, locals.var_q2d__blk1002_dn6, locals.var_q2d__blk1002_dn7, locals.var_q2d__blk1002_dn8, locals.var_q2d__blk1002_dn9, ) = (assign39760_e45321, (((locals.var_k2q2d__blk1005_dn4 * locals.var_k2__blk933) - (locals.var_k2q2d__blk1005 * locals.var_k2__blk933_dn4)) / (locals.var_k2__blk933 * locals.var_k2__blk933)), (((locals.var_k2q2d__blk1005_dn6 * locals.var_k2__blk933) - (locals.var_k2q2d__blk1005 * locals.var_k2__blk933_dn6)) / (locals.var_k2__blk933 * locals.var_k2__blk933)), (((locals.var_k2q2d__blk1005_dn7 * locals.var_k2__blk933) - (locals.var_k2q2d__blk1005 * locals.var_k2__blk933_dn7)) / (locals.var_k2__blk933 * locals.var_k2__blk933)), (((locals.var_k2q2d__blk1005_dn8 * locals.var_k2__blk933) - (locals.var_k2q2d__blk1005 * locals.var_k2__blk933_dn8)) / (locals.var_k2__blk933 * locals.var_k2__blk933)), (((locals.var_k2q2d__blk1005_dn9 * locals.var_k2__blk933) - (locals.var_k2q2d__blk1005 * locals.var_k2__blk933_dn9)) / (locals.var_k2__blk933 * locals.var_k2__blk933)), );
            locals.var_q2d__blk1002_rv = 0.0;
        }

        let assign39770_e45326: f64 = if locals.var_qsqd__blk1006 > 0.005 { 1.0 } else { 0.0 };
        locals.var_guard1211 = assign39770_e45326;
        locals.var_guard1211_rv = 0.0;

        if (((((locals.var_guard1080 != 0.0) && (locals.var_guard1206 == 0.0)) && (locals.var_guard1209 != 0.0)) && (locals.var_guard1210 == 0.0)) && (locals.var_guard1211 != 0.0)) {
            let assign39780_e45340: f64 = (4.0 * locals.var_qsqd__blk1006);
            let assign39780_e45345: f64 = (2.0 - locals.var_q_invexpq__blk831);
            let assign39780_e45346: f64 = (locals.var_q_invexpq__blk831 * assign39780_e45345);
            let assign39780_e45347: f64 = (1.0 - assign39780_e45346);
            let assign39780_e45348: f64 = (assign39780_e45340 / assign39780_e45347);
            let assign39780_e45349: f64 = (assign39780_e45348).ln();
            let assign39780_e45351: f64 = (assign39780_e45349 - locals.var_q_rac_qsq__blk828);
            (locals.var_q_temp2__blk815, locals.var_q_temp2__blk815_dn4, locals.var_q_temp2__blk815_dn6, locals.var_q_temp2__blk815_dn7, locals.var_q_temp2__blk815_dn8, locals.var_q_temp2__blk815_dn9, ) = (assign39780_e45351, ((((((4.0 * locals.var_qsqd__blk1006_dn4) * assign39780_e45347) - (assign39780_e45340 * (-((locals.var_q_invexpq__blk831_dn4 * assign39780_e45345) + (locals.var_q_invexpq__blk831 * (-locals.var_q_invexpq__blk831_dn4)))))) / (assign39780_e45347 * assign39780_e45347)) / assign39780_e45348) - locals.var_q_rac_qsq__blk828_dn4), ((((((4.0 * locals.var_qsqd__blk1006_dn6) * assign39780_e45347) - (assign39780_e45340 * (-((locals.var_q_invexpq__blk831_dn6 * assign39780_e45345) + (locals.var_q_invexpq__blk831 * (-locals.var_q_invexpq__blk831_dn6)))))) / (assign39780_e45347 * assign39780_e45347)) / assign39780_e45348) - locals.var_q_rac_qsq__blk828_dn6), ((((((4.0 * locals.var_qsqd__blk1006_dn7) * assign39780_e45347) - (assign39780_e45340 * (-((locals.var_q_invexpq__blk831_dn7 * assign39780_e45345) + (locals.var_q_invexpq__blk831 * (-locals.var_q_invexpq__blk831_dn7)))))) / (assign39780_e45347 * assign39780_e45347)) / assign39780_e45348) - locals.var_q_rac_qsq__blk828_dn7), ((((((4.0 * locals.var_qsqd__blk1006_dn8) * assign39780_e45347) - (assign39780_e45340 * (-((locals.var_q_invexpq__blk831_dn8 * assign39780_e45345) + (locals.var_q_invexpq__blk831 * (-locals.var_q_invexpq__blk831_dn8)))))) / (assign39780_e45347 * assign39780_e45347)) / assign39780_e45348) - locals.var_q_rac_qsq__blk828_dn8), ((((((4.0 * locals.var_qsqd__blk1006_dn9) * assign39780_e45347) - (assign39780_e45340 * (-((locals.var_q_invexpq__blk831_dn9 * assign39780_e45345) + (locals.var_q_invexpq__blk831 * (-locals.var_q_invexpq__blk831_dn9)))))) / (assign39780_e45347 * assign39780_e45347)) / assign39780_e45348) - locals.var_q_rac_qsq__blk828_dn9), );
            locals.var_q_temp2__blk815_rv = 0.0;
        }

        let assign39790_e45356: f64 = (-0.005);
        let assign39790_e45357: f64 = if locals.var_qsqd__blk1006 < assign39790_e45356 { 1.0 } else { 0.0 };
        locals.var_guard1212 = assign39790_e45357;
        locals.var_guard1212_rv = 0.0;

        if ((((((locals.var_guard1080 != 0.0) && (locals.var_guard1206 == 0.0)) && (locals.var_guard1209 != 0.0)) && (locals.var_guard1210 == 0.0)) && (locals.var_guard1211 == 0.0)) && (locals.var_guard1212 != 0.0)) {
            let assign39800_e45374: f64 = (0.5 * locals.var_q_rac_qsq__blk828);
            let assign39800_e45375: f64 = (assign39800_e45374).sin();
            (locals.var_q_temp3__blk816, locals.var_q_temp3__blk816_dn4, locals.var_q_temp3__blk816_dn6, locals.var_q_temp3__blk816_dn7, locals.var_q_temp3__blk816_dn8, locals.var_q_temp3__blk816_dn9, ) = (assign39800_e45375, ((assign39800_e45374).cos() * (0.5 * locals.var_q_rac_qsq__blk828_dn4)), ((assign39800_e45374).cos() * (0.5 * locals.var_q_rac_qsq__blk828_dn6)), ((assign39800_e45374).cos() * (0.5 * locals.var_q_rac_qsq__blk828_dn7)), ((assign39800_e45374).cos() * (0.5 * locals.var_q_rac_qsq__blk828_dn8)), ((assign39800_e45374).cos() * (0.5 * locals.var_q_rac_qsq__blk828_dn9)), );
            locals.var_q_temp3__blk816_rv = 0.0;
        }

        if ((((((locals.var_guard1080 != 0.0) && (locals.var_guard1206 == 0.0)) && (locals.var_guard1209 != 0.0)) && (locals.var_guard1210 == 0.0)) && (locals.var_guard1211 == 0.0)) && (locals.var_guard1212 != 0.0)) {
            let assign39810_e45393: f64 = (-locals.var_qsqd__blk1006);
            let assign39810_e45396: f64 = (locals.var_q_temp3__blk816 * locals.var_q_temp3__blk816);
            let assign39810_e45397: f64 = (assign39810_e45393 / assign39810_e45396);
            let assign39810_e45398: f64 = (assign39810_e45397).ln();
            (locals.var_q_temp2__blk815, locals.var_q_temp2__blk815_dn4, locals.var_q_temp2__blk815_dn6, locals.var_q_temp2__blk815_dn7, locals.var_q_temp2__blk815_dn8, locals.var_q_temp2__blk815_dn9, ) = (assign39810_e45398, (((((-locals.var_qsqd__blk1006_dn4) * assign39810_e45396) - (assign39810_e45393 * ((locals.var_q_temp3__blk816_dn4 * locals.var_q_temp3__blk816) + (locals.var_q_temp3__blk816 * locals.var_q_temp3__blk816_dn4)))) / (assign39810_e45396 * assign39810_e45396)) / assign39810_e45397), (((((-locals.var_qsqd__blk1006_dn6) * assign39810_e45396) - (assign39810_e45393 * ((locals.var_q_temp3__blk816_dn6 * locals.var_q_temp3__blk816) + (locals.var_q_temp3__blk816 * locals.var_q_temp3__blk816_dn6)))) / (assign39810_e45396 * assign39810_e45396)) / assign39810_e45397), (((((-locals.var_qsqd__blk1006_dn7) * assign39810_e45396) - (assign39810_e45393 * ((locals.var_q_temp3__blk816_dn7 * locals.var_q_temp3__blk816) + (locals.var_q_temp3__blk816 * locals.var_q_temp3__blk816_dn7)))) / (assign39810_e45396 * assign39810_e45396)) / assign39810_e45397), (((((-locals.var_qsqd__blk1006_dn8) * assign39810_e45396) - (assign39810_e45393 * ((locals.var_q_temp3__blk816_dn8 * locals.var_q_temp3__blk816) + (locals.var_q_temp3__blk816 * locals.var_q_temp3__blk816_dn8)))) / (assign39810_e45396 * assign39810_e45396)) / assign39810_e45397), (((((-locals.var_qsqd__blk1006_dn9) * assign39810_e45396) - (assign39810_e45393 * ((locals.var_q_temp3__blk816_dn9 * locals.var_q_temp3__blk816) + (locals.var_q_temp3__blk816 * locals.var_q_temp3__blk816_dn9)))) / (assign39810_e45396 * assign39810_e45396)) / assign39810_e45397), );
            locals.var_q_temp2__blk815_rv = 0.0;
        }

        if ((((((locals.var_guard1080 != 0.0) && (locals.var_guard1206 == 0.0)) && (locals.var_guard1209 != 0.0)) && (locals.var_guard1210 == 0.0)) && (locals.var_guard1211 == 0.0)) && (locals.var_guard1212 == 0.0)) {
            let assign39820_e45419: f64 = (locals.var_qsqd__blk1006 * 0.3333333333333);
            let assign39820_e45423: f64 = (0.05 * locals.var_qsqd__blk1006);
            let assign39820_e45427: f64 = (0.0396825396825397 * locals.var_qsqd__blk1006);
            let assign39820_e45428: f64 = (1.0 - assign39820_e45427);
            let assign39820_e45429: f64 = (assign39820_e45423 * assign39820_e45428);
            let assign39820_e45430: f64 = (1.0 - assign39820_e45429);
            let assign39820_e45431: f64 = (assign39820_e45419 * assign39820_e45430);
            let assign39820_e45432: f64 = (4.0 - assign39820_e45431);
            let assign39820_e45433: f64 = (assign39820_e45432).ln();
            (locals.var_q_temp2__blk815, locals.var_q_temp2__blk815_dn4, locals.var_q_temp2__blk815_dn6, locals.var_q_temp2__blk815_dn7, locals.var_q_temp2__blk815_dn8, locals.var_q_temp2__blk815_dn9, ) = (assign39820_e45433, ((-(((locals.var_qsqd__blk1006_dn4 * 0.3333333333333) * assign39820_e45430) + (assign39820_e45419 * (-(((0.05 * locals.var_qsqd__blk1006_dn4) * assign39820_e45428) + (assign39820_e45423 * (-(0.0396825396825397 * locals.var_qsqd__blk1006_dn4)))))))) / assign39820_e45432), ((-(((locals.var_qsqd__blk1006_dn6 * 0.3333333333333) * assign39820_e45430) + (assign39820_e45419 * (-(((0.05 * locals.var_qsqd__blk1006_dn6) * assign39820_e45428) + (assign39820_e45423 * (-(0.0396825396825397 * locals.var_qsqd__blk1006_dn6)))))))) / assign39820_e45432), ((-(((locals.var_qsqd__blk1006_dn7 * 0.3333333333333) * assign39820_e45430) + (assign39820_e45419 * (-(((0.05 * locals.var_qsqd__blk1006_dn7) * assign39820_e45428) + (assign39820_e45423 * (-(0.0396825396825397 * locals.var_qsqd__blk1006_dn7)))))))) / assign39820_e45432), ((-(((locals.var_qsqd__blk1006_dn8 * 0.3333333333333) * assign39820_e45430) + (assign39820_e45419 * (-(((0.05 * locals.var_qsqd__blk1006_dn8) * assign39820_e45428) + (assign39820_e45423 * (-(0.0396825396825397 * locals.var_qsqd__blk1006_dn8)))))))) / assign39820_e45432), ((-(((locals.var_qsqd__blk1006_dn9 * 0.3333333333333) * assign39820_e45430) + (assign39820_e45419 * (-(((0.05 * locals.var_qsqd__blk1006_dn9) * assign39820_e45428) + (assign39820_e45423 * (-(0.0396825396825397 * locals.var_qsqd__blk1006_dn9)))))))) / assign39820_e45432), );
            locals.var_q_temp2__blk815_rv = 0.0;
        }

        if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1206 == 0.0)) && (locals.var_guard1209 != 0.0)) && (locals.var_guard1210 == 0.0)) {
            let assign39830_e45447: f64 = (locals.var_xg2x__blk931 - locals.var_xg1x__blk930);
            let assign39830_e45449: f64 = (assign39830_e45447 + locals.var_q1d__blk1001);
            let assign39830_e45452: f64 = (locals.var_q_temp1__blk814).ln();
            let assign39830_e45453: f64 = (2.0 * assign39830_e45452);
            let assign39830_e45454: f64 = (assign39830_e45449 + assign39830_e45453);
            let assign39830_e45456: f64 = (assign39830_e45454 - locals.var_q_temp2__blk815);
            (locals.var_q2d__blk1002, locals.var_q2d__blk1002_dn4, locals.var_q2d__blk1002_dn6, locals.var_q2d__blk1002_dn7, locals.var_q2d__blk1002_dn8, locals.var_q2d__blk1002_dn9, ) = (assign39830_e45456, ((((locals.var_xg2x__blk931_dn4 - locals.var_xg1x__blk930_dn4) + locals.var_q1d__blk1001_dn4) + (2.0 * (locals.var_q_temp1__blk814_dn4 / locals.var_q_temp1__blk814))) - locals.var_q_temp2__blk815_dn4), ((((locals.var_xg2x__blk931_dn6 - locals.var_xg1x__blk930_dn6) + locals.var_q1d__blk1001_dn6) + (2.0 * (locals.var_q_temp1__blk814_dn6 / locals.var_q_temp1__blk814))) - locals.var_q_temp2__blk815_dn6), ((((locals.var_xg2x__blk931_dn7 - locals.var_xg1x__blk930_dn7) + locals.var_q1d__blk1001_dn7) + (2.0 * (locals.var_q_temp1__blk814_dn7 / locals.var_q_temp1__blk814))) - locals.var_q_temp2__blk815_dn7), ((((locals.var_xg2x__blk931_dn8 - locals.var_xg1x__blk930_dn8) + locals.var_q1d__blk1001_dn8) + (2.0 * (locals.var_q_temp1__blk814_dn8 / locals.var_q_temp1__blk814))) - locals.var_q_temp2__blk815_dn8), ((((locals.var_xg2x__blk931_dn9 - locals.var_xg1x__blk930_dn9) + locals.var_q1d__blk1001_dn9) + (2.0 * (locals.var_q_temp1__blk814_dn9 / locals.var_q_temp1__blk814))) - locals.var_q_temp2__blk815_dn9), );
            locals.var_q2d__blk1002_rv = 0.0;
        }

        if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1206 == 0.0)) && (locals.var_guard1209 != 0.0)) && (locals.var_guard1210 == 0.0)) {
            let assign39840_e45470: f64 = (locals.var_k2__blk933 * locals.var_q2d__blk1002);
            (locals.var_k2q2d__blk1005, locals.var_k2q2d__blk1005_dn4, locals.var_k2q2d__blk1005_dn6, locals.var_k2q2d__blk1005_dn7, locals.var_k2q2d__blk1005_dn8, locals.var_k2q2d__blk1005_dn9, ) = (assign39840_e45470, ((locals.var_k2__blk933_dn4 * locals.var_q2d__blk1002) + (locals.var_k2__blk933 * locals.var_q2d__blk1002_dn4)), ((locals.var_k2__blk933_dn6 * locals.var_q2d__blk1002) + (locals.var_k2__blk933 * locals.var_q2d__blk1002_dn6)), ((locals.var_k2__blk933_dn7 * locals.var_q2d__blk1002) + (locals.var_k2__blk933 * locals.var_q2d__blk1002_dn7)), ((locals.var_k2__blk933_dn8 * locals.var_q2d__blk1002) + (locals.var_k2__blk933 * locals.var_q2d__blk1002_dn8)), ((locals.var_k2__blk933_dn9 * locals.var_q2d__blk1002) + (locals.var_k2__blk933 * locals.var_q2d__blk1002_dn9)), );
            locals.var_k2q2d__blk1005_rv = 0.0;
        }

        if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1206 == 0.0)) && (locals.var_guard1209 != 0.0)) && (locals.var_guard1210 == 0.0)) {
            let assign39850_e45484: f64 = (locals.var_k1q1d__blk1004 + locals.var_k2q2d__blk1005);
            (locals.var_qid__blk1003, locals.var_qid__blk1003_dn4, locals.var_qid__blk1003_dn6, locals.var_qid__blk1003_dn7, locals.var_qid__blk1003_dn8, locals.var_qid__blk1003_dn9, ) = (assign39850_e45484, (locals.var_k1q1d__blk1004_dn4 + locals.var_k2q2d__blk1005_dn4), (locals.var_k1q1d__blk1004_dn6 + locals.var_k2q2d__blk1005_dn6), (locals.var_k1q1d__blk1004_dn7 + locals.var_k2q2d__blk1005_dn7), (locals.var_k1q1d__blk1004_dn8 + locals.var_k2q2d__blk1005_dn8), (locals.var_k1q1d__blk1004_dn9 + locals.var_k2q2d__blk1005_dn9), );
            locals.var_qid__blk1003_rv = 0.0;
        }

        let assign39860_e45489: f64 = if locals.var_qsqd__blk1006 > 0.005 { 1.0 } else { 0.0 };
        locals.var_guard1213 = assign39860_e45489;
        locals.var_guard1213_rv = 0.0;

        let assign39870_e45492: f64 = (locals.var_q1d__blk1001 + locals.var_xdeff__blk1000);
        let assign39870_e45494: f64 = (assign39870_e45492 - locals.var_xg1x__blk930);
        let assign39870_e45496: f64 = (assign39870_e45494 - locals.var_q_rac_qsq__blk828);
        let assign39870_e45498: f64 = if assign39870_e45496 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard1214 = assign39870_e45498;
        locals.var_guard1214_rv = 0.0;

        if (((((locals.var_guard1080 != 0.0) && (locals.var_guard1206 == 0.0)) && (locals.var_guard1209 == 0.0)) && (locals.var_guard1213 != 0.0)) && (locals.var_guard1214 != 0.0)) {
            let assign39880_e45512: f64 = (locals.var_q1d__blk1001 + locals.var_xdeff__blk1000);
            let assign39880_e45514: f64 = (assign39880_e45512 - locals.var_xg1x__blk930);
            let assign39880_e45516: f64 = (assign39880_e45514 - locals.var_q_rac_qsq__blk828);
            let assign39880_e45517: f64 = (assign39880_e45516).exp();
            (locals.var_q_temp3__blk816, locals.var_q_temp3__blk816_dn4, locals.var_q_temp3__blk816_dn6, locals.var_q_temp3__blk816_dn7, locals.var_q_temp3__blk816_dn8, locals.var_q_temp3__blk816_dn9, ) = (assign39880_e45517, (assign39880_e45517 * (((locals.var_q1d__blk1001_dn4 + locals.var_xdeff__blk1000_dn4) - locals.var_xg1x__blk930_dn4) - locals.var_q_rac_qsq__blk828_dn4)), (assign39880_e45517 * (((locals.var_q1d__blk1001_dn6 + locals.var_xdeff__blk1000_dn6) - locals.var_xg1x__blk930_dn6) - locals.var_q_rac_qsq__blk828_dn6)), (assign39880_e45517 * (((locals.var_q1d__blk1001_dn7 + locals.var_xdeff__blk1000_dn7) - locals.var_xg1x__blk930_dn7) - locals.var_q_rac_qsq__blk828_dn7)), (assign39880_e45517 * (((locals.var_q1d__blk1001_dn8 + locals.var_xdeff__blk1000_dn8) - locals.var_xg1x__blk930_dn8) - locals.var_q_rac_qsq__blk828_dn8)), (assign39880_e45517 * (((locals.var_q1d__blk1001_dn9 + locals.var_xdeff__blk1000_dn9) - locals.var_xg1x__blk930_dn9) - locals.var_q_rac_qsq__blk828_dn9)), );
            locals.var_q_temp3__blk816_rv = 0.0;
        }

        if (((((locals.var_guard1080 != 0.0) && (locals.var_guard1206 == 0.0)) && (locals.var_guard1209 == 0.0)) && (locals.var_guard1213 != 0.0)) && (locals.var_guard1214 == 0.0)) {
            let assign39890_e45536: f64 = (locals.var_q1d__blk1001 + locals.var_xdeff__blk1000);
            let assign39890_e45538: f64 = (assign39890_e45536 - locals.var_xg1x__blk930);
            let assign39890_e45540: f64 = (assign39890_e45538 - locals.var_q_rac_qsq__blk828);
            let assign39890_e45542: f64 = (assign39890_e45540 - 80.0);
            let assign39890_e45547: f64 = (locals.var_q1d__blk1001 + locals.var_xdeff__blk1000);
            let assign39890_e45549: f64 = (assign39890_e45547 - locals.var_xg1x__blk930);
            let assign39890_e45551: f64 = (assign39890_e45549 - locals.var_q_rac_qsq__blk828);
            let assign39890_e45553: f64 = (assign39890_e45551 - 80.0);
            let assign39890_e45554: f64 = (0.5 * assign39890_e45553);
            let assign39890_e45558: f64 = (locals.var_q1d__blk1001 + locals.var_xdeff__blk1000);
            let assign39890_e45560: f64 = (assign39890_e45558 - locals.var_xg1x__blk930);
            let assign39890_e45562: f64 = (assign39890_e45560 - locals.var_q_rac_qsq__blk828);
            let assign39890_e45564: f64 = (assign39890_e45562 - 80.0);
            let assign39890_e45566: f64 = (assign39890_e45564 * 0.3333333333333);
            let assign39890_e45567: f64 = (1.0 + assign39890_e45566);
            let assign39890_e45568: f64 = (assign39890_e45554 * assign39890_e45567);
            let assign39890_e45569: f64 = (1.0 + assign39890_e45568);
            let assign39890_e45570: f64 = (assign39890_e45542 * assign39890_e45569);
            let assign39890_e45571: f64 = (1.0 + assign39890_e45570);
            let assign39890_e45572: f64 = (5.54062e34 * assign39890_e45571);
            (locals.var_q_temp3__blk816, locals.var_q_temp3__blk816_dn4, locals.var_q_temp3__blk816_dn6, locals.var_q_temp3__blk816_dn7, locals.var_q_temp3__blk816_dn8, locals.var_q_temp3__blk816_dn9, ) = (assign39890_e45572, (5.54062e34 * (((((locals.var_q1d__blk1001_dn4 + locals.var_xdeff__blk1000_dn4) - locals.var_xg1x__blk930_dn4) - locals.var_q_rac_qsq__blk828_dn4) * assign39890_e45569) + (assign39890_e45542 * (((0.5 * (((locals.var_q1d__blk1001_dn4 + locals.var_xdeff__blk1000_dn4) - locals.var_xg1x__blk930_dn4) - locals.var_q_rac_qsq__blk828_dn4)) * assign39890_e45567) + (assign39890_e45554 * ((((locals.var_q1d__blk1001_dn4 + locals.var_xdeff__blk1000_dn4) - locals.var_xg1x__blk930_dn4) - locals.var_q_rac_qsq__blk828_dn4) * 0.3333333333333)))))), (5.54062e34 * (((((locals.var_q1d__blk1001_dn6 + locals.var_xdeff__blk1000_dn6) - locals.var_xg1x__blk930_dn6) - locals.var_q_rac_qsq__blk828_dn6) * assign39890_e45569) + (assign39890_e45542 * (((0.5 * (((locals.var_q1d__blk1001_dn6 + locals.var_xdeff__blk1000_dn6) - locals.var_xg1x__blk930_dn6) - locals.var_q_rac_qsq__blk828_dn6)) * assign39890_e45567) + (assign39890_e45554 * ((((locals.var_q1d__blk1001_dn6 + locals.var_xdeff__blk1000_dn6) - locals.var_xg1x__blk930_dn6) - locals.var_q_rac_qsq__blk828_dn6) * 0.3333333333333)))))), (5.54062e34 * (((((locals.var_q1d__blk1001_dn7 + locals.var_xdeff__blk1000_dn7) - locals.var_xg1x__blk930_dn7) - locals.var_q_rac_qsq__blk828_dn7) * assign39890_e45569) + (assign39890_e45542 * (((0.5 * (((locals.var_q1d__blk1001_dn7 + locals.var_xdeff__blk1000_dn7) - locals.var_xg1x__blk930_dn7) - locals.var_q_rac_qsq__blk828_dn7)) * assign39890_e45567) + (assign39890_e45554 * ((((locals.var_q1d__blk1001_dn7 + locals.var_xdeff__blk1000_dn7) - locals.var_xg1x__blk930_dn7) - locals.var_q_rac_qsq__blk828_dn7) * 0.3333333333333)))))), (5.54062e34 * (((((locals.var_q1d__blk1001_dn8 + locals.var_xdeff__blk1000_dn8) - locals.var_xg1x__blk930_dn8) - locals.var_q_rac_qsq__blk828_dn8) * assign39890_e45569) + (assign39890_e45542 * (((0.5 * (((locals.var_q1d__blk1001_dn8 + locals.var_xdeff__blk1000_dn8) - locals.var_xg1x__blk930_dn8) - locals.var_q_rac_qsq__blk828_dn8)) * assign39890_e45567) + (assign39890_e45554 * ((((locals.var_q1d__blk1001_dn8 + locals.var_xdeff__blk1000_dn8) - locals.var_xg1x__blk930_dn8) - locals.var_q_rac_qsq__blk828_dn8) * 0.3333333333333)))))), (5.54062e34 * (((((locals.var_q1d__blk1001_dn9 + locals.var_xdeff__blk1000_dn9) - locals.var_xg1x__blk930_dn9) - locals.var_q_rac_qsq__blk828_dn9) * assign39890_e45569) + (assign39890_e45542 * (((0.5 * (((locals.var_q1d__blk1001_dn9 + locals.var_xdeff__blk1000_dn9) - locals.var_xg1x__blk930_dn9) - locals.var_q_rac_qsq__blk828_dn9)) * assign39890_e45567) + (assign39890_e45554 * ((((locals.var_q1d__blk1001_dn9 + locals.var_xdeff__blk1000_dn9) - locals.var_xg1x__blk930_dn9) - locals.var_q_rac_qsq__blk828_dn9) * 0.3333333333333)))))), );
            locals.var_q_temp3__blk816_rv = 0.0;
        }

        if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1206 == 0.0)) && (locals.var_guard1209 == 0.0)) && (locals.var_guard1213 != 0.0)) {
            let assign39900_e45586: f64 = (locals.var_q_temp3__blk816 / locals.var_a0__blk905);
            (locals.var_q_temp2__blk815, locals.var_q_temp2__blk815_dn4, locals.var_q_temp2__blk815_dn6, locals.var_q_temp2__blk815_dn7, locals.var_q_temp2__blk815_dn8, locals.var_q_temp2__blk815_dn9, ) = (assign39900_e45586, (((locals.var_q_temp3__blk816_dn4 * locals.var_a0__blk905) - (locals.var_q_temp3__blk816 * locals.var_a0__blk905_dn4)) / (locals.var_a0__blk905 * locals.var_a0__blk905)), (((locals.var_q_temp3__blk816_dn6 * locals.var_a0__blk905) - (locals.var_q_temp3__blk816 * locals.var_a0__blk905_dn6)) / (locals.var_a0__blk905 * locals.var_a0__blk905)), (((locals.var_q_temp3__blk816_dn7 * locals.var_a0__blk905) - (locals.var_q_temp3__blk816 * locals.var_a0__blk905_dn7)) / (locals.var_a0__blk905 * locals.var_a0__blk905)), (((locals.var_q_temp3__blk816_dn8 * locals.var_a0__blk905) - (locals.var_q_temp3__blk816 * locals.var_a0__blk905_dn8)) / (locals.var_a0__blk905 * locals.var_a0__blk905)), (((locals.var_q_temp3__blk816_dn9 * locals.var_a0__blk905) - (locals.var_q_temp3__blk816 * locals.var_a0__blk905_dn9)) / (locals.var_a0__blk905 * locals.var_a0__blk905)), );
            locals.var_q_temp2__blk815_rv = 0.0;
        }

        if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1206 == 0.0)) && (locals.var_guard1209 == 0.0)) && (locals.var_guard1213 != 0.0)) {
            let assign39910_e45600: f64 = (4.0 * locals.var_qsqd__blk1006);
            let assign39910_e45602: f64 = (assign39910_e45600 * locals.var_q_temp2__blk815);
            let assign39910_e45607: f64 = (2.0 - locals.var_q_invexpq__blk831);
            let assign39910_e45608: f64 = (locals.var_q_invexpq__blk831 * assign39910_e45607);
            let assign39910_e45609: f64 = (1.0 - assign39910_e45608);
            let assign39910_e45610: f64 = (assign39910_e45602 / assign39910_e45609);
            (locals.var_q_temp1__blk814, locals.var_q_temp1__blk814_dn4, locals.var_q_temp1__blk814_dn6, locals.var_q_temp1__blk814_dn7, locals.var_q_temp1__blk814_dn8, locals.var_q_temp1__blk814_dn9, ) = (assign39910_e45610, ((((((4.0 * locals.var_qsqd__blk1006_dn4) * locals.var_q_temp2__blk815) + (assign39910_e45600 * locals.var_q_temp2__blk815_dn4)) * assign39910_e45609) - (assign39910_e45602 * (-((locals.var_q_invexpq__blk831_dn4 * assign39910_e45607) + (locals.var_q_invexpq__blk831 * (-locals.var_q_invexpq__blk831_dn4)))))) / (assign39910_e45609 * assign39910_e45609)), ((((((4.0 * locals.var_qsqd__blk1006_dn6) * locals.var_q_temp2__blk815) + (assign39910_e45600 * locals.var_q_temp2__blk815_dn6)) * assign39910_e45609) - (assign39910_e45602 * (-((locals.var_q_invexpq__blk831_dn6 * assign39910_e45607) + (locals.var_q_invexpq__blk831 * (-locals.var_q_invexpq__blk831_dn6)))))) / (assign39910_e45609 * assign39910_e45609)), ((((((4.0 * locals.var_qsqd__blk1006_dn7) * locals.var_q_temp2__blk815) + (assign39910_e45600 * locals.var_q_temp2__blk815_dn7)) * assign39910_e45609) - (assign39910_e45602 * (-((locals.var_q_invexpq__blk831_dn7 * assign39910_e45607) + (locals.var_q_invexpq__blk831 * (-locals.var_q_invexpq__blk831_dn7)))))) / (assign39910_e45609 * assign39910_e45609)), ((((((4.0 * locals.var_qsqd__blk1006_dn8) * locals.var_q_temp2__blk815) + (assign39910_e45600 * locals.var_q_temp2__blk815_dn8)) * assign39910_e45609) - (assign39910_e45602 * (-((locals.var_q_invexpq__blk831_dn8 * assign39910_e45607) + (locals.var_q_invexpq__blk831 * (-locals.var_q_invexpq__blk831_dn8)))))) / (assign39910_e45609 * assign39910_e45609)), ((((((4.0 * locals.var_qsqd__blk1006_dn9) * locals.var_q_temp2__blk815) + (assign39910_e45600 * locals.var_q_temp2__blk815_dn9)) * assign39910_e45609) - (assign39910_e45602 * (-((locals.var_q_invexpq__blk831_dn9 * assign39910_e45607) + (locals.var_q_invexpq__blk831 * (-locals.var_q_invexpq__blk831_dn9)))))) / (assign39910_e45609 * assign39910_e45609)), );
            locals.var_q_temp1__blk814_rv = 0.0;
        }

        let assign39920_e45615: f64 = (-0.005);
        let assign39920_e45616: f64 = if locals.var_qsqd__blk1006 < assign39920_e45615 { 1.0 } else { 0.0 };
        locals.var_guard1215 = assign39920_e45616;
        locals.var_guard1215_rv = 0.0;

        if (((((locals.var_guard1080 != 0.0) && (locals.var_guard1206 == 0.0)) && (locals.var_guard1209 == 0.0)) && (locals.var_guard1213 == 0.0)) && (locals.var_guard1215 != 0.0)) {
            let assign39930_e45631: f64 = (0.5 * locals.var_q_rac_qsq__blk828);
            let assign39930_e45632: f64 = (assign39930_e45631).sin();
            (locals.var_q_temp2__blk815, locals.var_q_temp2__blk815_dn4, locals.var_q_temp2__blk815_dn6, locals.var_q_temp2__blk815_dn7, locals.var_q_temp2__blk815_dn8, locals.var_q_temp2__blk815_dn9, ) = (assign39930_e45632, ((assign39930_e45631).cos() * (0.5 * locals.var_q_rac_qsq__blk828_dn4)), ((assign39930_e45631).cos() * (0.5 * locals.var_q_rac_qsq__blk828_dn6)), ((assign39930_e45631).cos() * (0.5 * locals.var_q_rac_qsq__blk828_dn7)), ((assign39930_e45631).cos() * (0.5 * locals.var_q_rac_qsq__blk828_dn8)), ((assign39930_e45631).cos() * (0.5 * locals.var_q_rac_qsq__blk828_dn9)), );
            locals.var_q_temp2__blk815_rv = 0.0;
        }

        if (((((locals.var_guard1080 != 0.0) && (locals.var_guard1206 == 0.0)) && (locals.var_guard1209 == 0.0)) && (locals.var_guard1213 == 0.0)) && (locals.var_guard1215 != 0.0)) {
            let assign39940_e45648: f64 = (-locals.var_qsqd__blk1006);
            let assign39940_e45651: f64 = (locals.var_q_temp2__blk815 * locals.var_q_temp2__blk815);
            let assign39940_e45652: f64 = (assign39940_e45648 / assign39940_e45651);
            let assign39940_e45654: f64 = (assign39940_e45652 / locals.var_aexp1d__blk1007);
            (locals.var_q_temp1__blk814, locals.var_q_temp1__blk814_dn4, locals.var_q_temp1__blk814_dn6, locals.var_q_temp1__blk814_dn7, locals.var_q_temp1__blk814_dn8, locals.var_q_temp1__blk814_dn9, ) = (assign39940_e45654, (((((((-locals.var_qsqd__blk1006_dn4) * assign39940_e45651) - (assign39940_e45648 * ((locals.var_q_temp2__blk815_dn4 * locals.var_q_temp2__blk815) + (locals.var_q_temp2__blk815 * locals.var_q_temp2__blk815_dn4)))) / (assign39940_e45651 * assign39940_e45651)) * locals.var_aexp1d__blk1007) - (assign39940_e45652 * locals.var_aexp1d__blk1007_dn4)) / (locals.var_aexp1d__blk1007 * locals.var_aexp1d__blk1007)), (((((((-locals.var_qsqd__blk1006_dn6) * assign39940_e45651) - (assign39940_e45648 * ((locals.var_q_temp2__blk815_dn6 * locals.var_q_temp2__blk815) + (locals.var_q_temp2__blk815 * locals.var_q_temp2__blk815_dn6)))) / (assign39940_e45651 * assign39940_e45651)) * locals.var_aexp1d__blk1007) - (assign39940_e45652 * locals.var_aexp1d__blk1007_dn6)) / (locals.var_aexp1d__blk1007 * locals.var_aexp1d__blk1007)), (((((((-locals.var_qsqd__blk1006_dn7) * assign39940_e45651) - (assign39940_e45648 * ((locals.var_q_temp2__blk815_dn7 * locals.var_q_temp2__blk815) + (locals.var_q_temp2__blk815 * locals.var_q_temp2__blk815_dn7)))) / (assign39940_e45651 * assign39940_e45651)) * locals.var_aexp1d__blk1007) - (assign39940_e45652 * locals.var_aexp1d__blk1007_dn7)) / (locals.var_aexp1d__blk1007 * locals.var_aexp1d__blk1007)), (((((((-locals.var_qsqd__blk1006_dn8) * assign39940_e45651) - (assign39940_e45648 * ((locals.var_q_temp2__blk815_dn8 * locals.var_q_temp2__blk815) + (locals.var_q_temp2__blk815 * locals.var_q_temp2__blk815_dn8)))) / (assign39940_e45651 * assign39940_e45651)) * locals.var_aexp1d__blk1007) - (assign39940_e45652 * locals.var_aexp1d__blk1007_dn8)) / (locals.var_aexp1d__blk1007 * locals.var_aexp1d__blk1007)), (((((((-locals.var_qsqd__blk1006_dn9) * assign39940_e45651) - (assign39940_e45648 * ((locals.var_q_temp2__blk815_dn9 * locals.var_q_temp2__blk815) + (locals.var_q_temp2__blk815 * locals.var_q_temp2__blk815_dn9)))) / (assign39940_e45651 * assign39940_e45651)) * locals.var_aexp1d__blk1007) - (assign39940_e45652 * locals.var_aexp1d__blk1007_dn9)) / (locals.var_aexp1d__blk1007 * locals.var_aexp1d__blk1007)), );
            locals.var_q_temp1__blk814_rv = 0.0;
        }

        if (((((locals.var_guard1080 != 0.0) && (locals.var_guard1206 == 0.0)) && (locals.var_guard1209 == 0.0)) && (locals.var_guard1213 == 0.0)) && (locals.var_guard1215 == 0.0)) {
            let assign39950_e45673: f64 = (locals.var_qsqd__blk1006 * 0.3333333333333);
            let assign39950_e45677: f64 = (0.05 * locals.var_qsqd__blk1006);
            let assign39950_e45681: f64 = (0.0396825396825397 * locals.var_qsqd__blk1006);
            let assign39950_e45682: f64 = (1.0 - assign39950_e45681);
            let assign39950_e45683: f64 = (assign39950_e45677 * assign39950_e45682);
            let assign39950_e45684: f64 = (1.0 - assign39950_e45683);
            let assign39950_e45685: f64 = (assign39950_e45673 * assign39950_e45684);
            let assign39950_e45686: f64 = (4.0 - assign39950_e45685);
            let assign39950_e45688: f64 = (assign39950_e45686 / locals.var_aexp1d__blk1007);
            (locals.var_q_temp1__blk814, locals.var_q_temp1__blk814_dn4, locals.var_q_temp1__blk814_dn6, locals.var_q_temp1__blk814_dn7, locals.var_q_temp1__blk814_dn8, locals.var_q_temp1__blk814_dn9, ) = (assign39950_e45688, ((((-(((locals.var_qsqd__blk1006_dn4 * 0.3333333333333) * assign39950_e45684) + (assign39950_e45673 * (-(((0.05 * locals.var_qsqd__blk1006_dn4) * assign39950_e45682) + (assign39950_e45677 * (-(0.0396825396825397 * locals.var_qsqd__blk1006_dn4)))))))) * locals.var_aexp1d__blk1007) - (assign39950_e45686 * locals.var_aexp1d__blk1007_dn4)) / (locals.var_aexp1d__blk1007 * locals.var_aexp1d__blk1007)), ((((-(((locals.var_qsqd__blk1006_dn6 * 0.3333333333333) * assign39950_e45684) + (assign39950_e45673 * (-(((0.05 * locals.var_qsqd__blk1006_dn6) * assign39950_e45682) + (assign39950_e45677 * (-(0.0396825396825397 * locals.var_qsqd__blk1006_dn6)))))))) * locals.var_aexp1d__blk1007) - (assign39950_e45686 * locals.var_aexp1d__blk1007_dn6)) / (locals.var_aexp1d__blk1007 * locals.var_aexp1d__blk1007)), ((((-(((locals.var_qsqd__blk1006_dn7 * 0.3333333333333) * assign39950_e45684) + (assign39950_e45673 * (-(((0.05 * locals.var_qsqd__blk1006_dn7) * assign39950_e45682) + (assign39950_e45677 * (-(0.0396825396825397 * locals.var_qsqd__blk1006_dn7)))))))) * locals.var_aexp1d__blk1007) - (assign39950_e45686 * locals.var_aexp1d__blk1007_dn7)) / (locals.var_aexp1d__blk1007 * locals.var_aexp1d__blk1007)), ((((-(((locals.var_qsqd__blk1006_dn8 * 0.3333333333333) * assign39950_e45684) + (assign39950_e45673 * (-(((0.05 * locals.var_qsqd__blk1006_dn8) * assign39950_e45682) + (assign39950_e45677 * (-(0.0396825396825397 * locals.var_qsqd__blk1006_dn8)))))))) * locals.var_aexp1d__blk1007) - (assign39950_e45686 * locals.var_aexp1d__blk1007_dn8)) / (locals.var_aexp1d__blk1007 * locals.var_aexp1d__blk1007)), ((((-(((locals.var_qsqd__blk1006_dn9 * 0.3333333333333) * assign39950_e45684) + (assign39950_e45673 * (-(((0.05 * locals.var_qsqd__blk1006_dn9) * assign39950_e45682) + (assign39950_e45677 * (-(0.0396825396825397 * locals.var_qsqd__blk1006_dn9)))))))) * locals.var_aexp1d__blk1007) - (assign39950_e45686 * locals.var_aexp1d__blk1007_dn9)) / (locals.var_aexp1d__blk1007 * locals.var_aexp1d__blk1007)), );
            locals.var_q_temp1__blk814_rv = 0.0;
        }

        if (((locals.var_guard1080 != 0.0) && (locals.var_guard1206 == 0.0)) && (locals.var_guard1209 == 0.0)) {
            let assign39960_e45700: f64 = (locals.var_k1q1d__blk1004 - locals.var_q_qcoth__blk829);
            let assign39960_e45703: f64 = (1.0 - locals.var_q_temp1__blk814);
            let assign39960_e45704: f64 = (assign39960_e45700 / assign39960_e45703);
            let assign39960_e45706: f64 = (assign39960_e45704 + 1e-80);
            (locals.var_qid__blk1003, locals.var_qid__blk1003_dn4, locals.var_qid__blk1003_dn6, locals.var_qid__blk1003_dn7, locals.var_qid__blk1003_dn8, locals.var_qid__blk1003_dn9, ) = (assign39960_e45706, ((((locals.var_k1q1d__blk1004_dn4 - locals.var_q_qcoth__blk829_dn4) * assign39960_e45703) - (assign39960_e45700 * (-locals.var_q_temp1__blk814_dn4))) / (assign39960_e45703 * assign39960_e45703)), ((((locals.var_k1q1d__blk1004_dn6 - locals.var_q_qcoth__blk829_dn6) * assign39960_e45703) - (assign39960_e45700 * (-locals.var_q_temp1__blk814_dn6))) / (assign39960_e45703 * assign39960_e45703)), ((((locals.var_k1q1d__blk1004_dn7 - locals.var_q_qcoth__blk829_dn7) * assign39960_e45703) - (assign39960_e45700 * (-locals.var_q_temp1__blk814_dn7))) / (assign39960_e45703 * assign39960_e45703)), ((((locals.var_k1q1d__blk1004_dn8 - locals.var_q_qcoth__blk829_dn8) * assign39960_e45703) - (assign39960_e45700 * (-locals.var_q_temp1__blk814_dn8))) / (assign39960_e45703 * assign39960_e45703)), ((((locals.var_k1q1d__blk1004_dn9 - locals.var_q_qcoth__blk829_dn9) * assign39960_e45703) - (assign39960_e45700 * (-locals.var_q_temp1__blk814_dn9))) / (assign39960_e45703 * assign39960_e45703)), );
            locals.var_qid__blk1003_rv = 0.0;
        }

        if (((locals.var_guard1080 != 0.0) && (locals.var_guard1206 == 0.0)) && (locals.var_guard1209 == 0.0)) {
            let assign39970_e45718: f64 = (locals.var_qid__blk1003 - locals.var_k1q1d__blk1004);
            (locals.var_k2q2d__blk1005, locals.var_k2q2d__blk1005_dn4, locals.var_k2q2d__blk1005_dn6, locals.var_k2q2d__blk1005_dn7, locals.var_k2q2d__blk1005_dn8, locals.var_k2q2d__blk1005_dn9, ) = (assign39970_e45718, (locals.var_qid__blk1003_dn4 - locals.var_k1q1d__blk1004_dn4), (locals.var_qid__blk1003_dn6 - locals.var_k1q1d__blk1004_dn6), (locals.var_qid__blk1003_dn7 - locals.var_k1q1d__blk1004_dn7), (locals.var_qid__blk1003_dn8 - locals.var_k1q1d__blk1004_dn8), (locals.var_qid__blk1003_dn9 - locals.var_k1q1d__blk1004_dn9), );
            locals.var_k2q2d__blk1005_rv = 0.0;
        }

        if (((locals.var_guard1080 != 0.0) && (locals.var_guard1206 == 0.0)) && (locals.var_guard1209 == 0.0)) {
            let assign39980_e45730: f64 = (locals.var_k2q2d__blk1005 / locals.var_k2__blk933);
            (locals.var_q2d__blk1002, locals.var_q2d__blk1002_dn4, locals.var_q2d__blk1002_dn6, locals.var_q2d__blk1002_dn7, locals.var_q2d__blk1002_dn8, locals.var_q2d__blk1002_dn9, ) = (assign39980_e45730, (((locals.var_k2q2d__blk1005_dn4 * locals.var_k2__blk933) - (locals.var_k2q2d__blk1005 * locals.var_k2__blk933_dn4)) / (locals.var_k2__blk933 * locals.var_k2__blk933)), (((locals.var_k2q2d__blk1005_dn6 * locals.var_k2__blk933) - (locals.var_k2q2d__blk1005 * locals.var_k2__blk933_dn6)) / (locals.var_k2__blk933 * locals.var_k2__blk933)), (((locals.var_k2q2d__blk1005_dn7 * locals.var_k2__blk933) - (locals.var_k2q2d__blk1005 * locals.var_k2__blk933_dn7)) / (locals.var_k2__blk933 * locals.var_k2__blk933)), (((locals.var_k2q2d__blk1005_dn8 * locals.var_k2__blk933) - (locals.var_k2q2d__blk1005 * locals.var_k2__blk933_dn8)) / (locals.var_k2__blk933 * locals.var_k2__blk933)), (((locals.var_k2q2d__blk1005_dn9 * locals.var_k2__blk933) - (locals.var_k2q2d__blk1005 * locals.var_k2__blk933_dn9)) / (locals.var_k2__blk933 * locals.var_k2__blk933)), );
            locals.var_q2d__blk1002_rv = 0.0;
        }

        let assign39990_e45735: f64 = (locals.var_xg2x__blk931 - locals.var_q2d__blk1002);
        let assign39990_e45737: f64 = (assign39990_e45735 - locals.var_xdeff__blk1000);
        let assign39990_e45739: f64 = if assign39990_e45737 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard1216 = assign39990_e45739;
        locals.var_guard1216_rv = 0.0;

        if ((locals.var_guard1080 != 0.0) && (locals.var_guard1216 != 0.0)) {
            let assign40000_e45745: f64 = (locals.var_xg2x__blk931 - locals.var_q2d__blk1002);
            let assign40000_e45747: f64 = (assign40000_e45745 - locals.var_xdeff__blk1000);
            let assign40000_e45748: f64 = (assign40000_e45747).exp();
            (locals.var_q_temp1__blk814, locals.var_q_temp1__blk814_dn4, locals.var_q_temp1__blk814_dn6, locals.var_q_temp1__blk814_dn7, locals.var_q_temp1__blk814_dn8, locals.var_q_temp1__blk814_dn9, ) = (assign40000_e45748, (assign40000_e45748 * ((locals.var_xg2x__blk931_dn4 - locals.var_q2d__blk1002_dn4) - locals.var_xdeff__blk1000_dn4)), (assign40000_e45748 * ((locals.var_xg2x__blk931_dn6 - locals.var_q2d__blk1002_dn6) - locals.var_xdeff__blk1000_dn6)), (assign40000_e45748 * ((locals.var_xg2x__blk931_dn7 - locals.var_q2d__blk1002_dn7) - locals.var_xdeff__blk1000_dn7)), (assign40000_e45748 * ((locals.var_xg2x__blk931_dn8 - locals.var_q2d__blk1002_dn8) - locals.var_xdeff__blk1000_dn8)), (assign40000_e45748 * ((locals.var_xg2x__blk931_dn9 - locals.var_q2d__blk1002_dn9) - locals.var_xdeff__blk1000_dn9)), );
            locals.var_q_temp1__blk814_rv = 0.0;
        }

        if ((locals.var_guard1080 != 0.0) && (locals.var_guard1216 == 0.0)) {
            let assign40010_e45759: f64 = (locals.var_xg2x__blk931 - locals.var_q2d__blk1002);
            let assign40010_e45761: f64 = (assign40010_e45759 - locals.var_xdeff__blk1000);
            let assign40010_e45763: f64 = (assign40010_e45761 - 80.0);
            let assign40010_e45768: f64 = (locals.var_xg2x__blk931 - locals.var_q2d__blk1002);
            let assign40010_e45770: f64 = (assign40010_e45768 - locals.var_xdeff__blk1000);
            let assign40010_e45772: f64 = (assign40010_e45770 - 80.0);
            let assign40010_e45773: f64 = (0.5 * assign40010_e45772);
            let assign40010_e45777: f64 = (locals.var_xg2x__blk931 - locals.var_q2d__blk1002);
            let assign40010_e45779: f64 = (assign40010_e45777 - locals.var_xdeff__blk1000);
            let assign40010_e45781: f64 = (assign40010_e45779 - 80.0);
            let assign40010_e45783: f64 = (assign40010_e45781 * 0.3333333333333);
            let assign40010_e45784: f64 = (1.0 + assign40010_e45783);
            let assign40010_e45785: f64 = (assign40010_e45773 * assign40010_e45784);
            let assign40010_e45786: f64 = (1.0 + assign40010_e45785);
            let assign40010_e45787: f64 = (assign40010_e45763 * assign40010_e45786);
            let assign40010_e45788: f64 = (1.0 + assign40010_e45787);
            let assign40010_e45789: f64 = (5.54062e34 * assign40010_e45788);
            (locals.var_q_temp1__blk814, locals.var_q_temp1__blk814_dn4, locals.var_q_temp1__blk814_dn6, locals.var_q_temp1__blk814_dn7, locals.var_q_temp1__blk814_dn8, locals.var_q_temp1__blk814_dn9, ) = (assign40010_e45789, (5.54062e34 * ((((locals.var_xg2x__blk931_dn4 - locals.var_q2d__blk1002_dn4) - locals.var_xdeff__blk1000_dn4) * assign40010_e45786) + (assign40010_e45763 * (((0.5 * ((locals.var_xg2x__blk931_dn4 - locals.var_q2d__blk1002_dn4) - locals.var_xdeff__blk1000_dn4)) * assign40010_e45784) + (assign40010_e45773 * (((locals.var_xg2x__blk931_dn4 - locals.var_q2d__blk1002_dn4) - locals.var_xdeff__blk1000_dn4) * 0.3333333333333)))))), (5.54062e34 * ((((locals.var_xg2x__blk931_dn6 - locals.var_q2d__blk1002_dn6) - locals.var_xdeff__blk1000_dn6) * assign40010_e45786) + (assign40010_e45763 * (((0.5 * ((locals.var_xg2x__blk931_dn6 - locals.var_q2d__blk1002_dn6) - locals.var_xdeff__blk1000_dn6)) * assign40010_e45784) + (assign40010_e45773 * (((locals.var_xg2x__blk931_dn6 - locals.var_q2d__blk1002_dn6) - locals.var_xdeff__blk1000_dn6) * 0.3333333333333)))))), (5.54062e34 * ((((locals.var_xg2x__blk931_dn7 - locals.var_q2d__blk1002_dn7) - locals.var_xdeff__blk1000_dn7) * assign40010_e45786) + (assign40010_e45763 * (((0.5 * ((locals.var_xg2x__blk931_dn7 - locals.var_q2d__blk1002_dn7) - locals.var_xdeff__blk1000_dn7)) * assign40010_e45784) + (assign40010_e45773 * (((locals.var_xg2x__blk931_dn7 - locals.var_q2d__blk1002_dn7) - locals.var_xdeff__blk1000_dn7) * 0.3333333333333)))))), (5.54062e34 * ((((locals.var_xg2x__blk931_dn8 - locals.var_q2d__blk1002_dn8) - locals.var_xdeff__blk1000_dn8) * assign40010_e45786) + (assign40010_e45763 * (((0.5 * ((locals.var_xg2x__blk931_dn8 - locals.var_q2d__blk1002_dn8) - locals.var_xdeff__blk1000_dn8)) * assign40010_e45784) + (assign40010_e45773 * (((locals.var_xg2x__blk931_dn8 - locals.var_q2d__blk1002_dn8) - locals.var_xdeff__blk1000_dn8) * 0.3333333333333)))))), (5.54062e34 * ((((locals.var_xg2x__blk931_dn9 - locals.var_q2d__blk1002_dn9) - locals.var_xdeff__blk1000_dn9) * assign40010_e45786) + (assign40010_e45763 * (((0.5 * ((locals.var_xg2x__blk931_dn9 - locals.var_q2d__blk1002_dn9) - locals.var_xdeff__blk1000_dn9)) * assign40010_e45784) + (assign40010_e45773 * (((locals.var_xg2x__blk931_dn9 - locals.var_q2d__blk1002_dn9) - locals.var_xdeff__blk1000_dn9) * 0.3333333333333)))))), );
            locals.var_q_temp1__blk814_rv = 0.0;
        }

        if (locals.var_guard1080 != 0.0) {
            let assign40020_e45795: f64 = (locals.var_a0__blk905 * locals.var_q_temp1__blk814);
            (locals.var_aexp2d__blk1008, locals.var_aexp2d__blk1008_dn4, locals.var_aexp2d__blk1008_dn6, locals.var_aexp2d__blk1008_dn7, locals.var_aexp2d__blk1008_dn8, locals.var_aexp2d__blk1008_dn9, ) = (assign40020_e45795, ((locals.var_a0__blk905_dn4 * locals.var_q_temp1__blk814) + (locals.var_a0__blk905 * locals.var_q_temp1__blk814_dn4)), ((locals.var_a0__blk905_dn6 * locals.var_q_temp1__blk814) + (locals.var_a0__blk905 * locals.var_q_temp1__blk814_dn6)), ((locals.var_a0__blk905_dn7 * locals.var_q_temp1__blk814) + (locals.var_a0__blk905 * locals.var_q_temp1__blk814_dn7)), ((locals.var_a0__blk905_dn8 * locals.var_q_temp1__blk814) + (locals.var_a0__blk905 * locals.var_q_temp1__blk814_dn8)), ((locals.var_a0__blk905_dn9 * locals.var_q_temp1__blk814) + (locals.var_a0__blk905 * locals.var_q_temp1__blk814_dn9)), );
            locals.var_aexp2d__blk1008_rv = 0.0;
        }

        if (locals.var_guard1080 != 0.0) {
            (locals.var_a1d__blk1011, locals.var_a1d__blk1011_dn4, locals.var_a1d__blk1011_dn6, locals.var_a1d__blk1011_dn7, locals.var_a1d__blk1011_dn8, locals.var_a1d__blk1011_dn9, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_a1d__blk1011_rv = 0.0;
            (locals.var_a2d__blk1012, locals.var_a2d__blk1012_dn4, locals.var_a2d__blk1012_dn6, locals.var_a2d__blk1012_dn7, locals.var_a2d__blk1012_dn8, locals.var_a2d__blk1012_dn9, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_a2d__blk1012_rv = 0.0;
            (locals.var_b1d__blk1009, locals.var_b1d__blk1009_dn4, locals.var_b1d__blk1009_dn6, locals.var_b1d__blk1009_dn7, locals.var_b1d__blk1009_dn8, locals.var_b1d__blk1009_dn9, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_b1d__blk1009_rv = 0.0;
            (locals.var_b2d__blk1010, locals.var_b2d__blk1010_dn4, locals.var_b2d__blk1010_dn6, locals.var_b2d__blk1010_dn7, locals.var_b2d__blk1010_dn8, locals.var_b2d__blk1010_dn9, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_b2d__blk1010_rv = 0.0;
            (locals.var_sumd__blk1013, locals.var_sumd__blk1013_dn4, locals.var_sumd__blk1013_dn6, locals.var_sumd__blk1013_dn7, locals.var_sumd__blk1013_dn8, locals.var_sumd__blk1013_dn9, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_sumd__blk1013_rv = 0.0;
            (locals.var_dqsqd_dxn_qi__blk1014, locals.var_dqsqd_dxn_qi__blk1014_dn4, locals.var_dqsqd_dxn_qi__blk1014_dn6, locals.var_dqsqd_dxn_qi__blk1014_dn7, locals.var_dqsqd_dxn_qi__blk1014_dn8, locals.var_dqsqd_dxn_qi__blk1014_dn9, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_dqsqd_dxn_qi__blk1014_rv = 0.0;
        }

        let assign40090_e45824: f64 = if locals.var_qis__blk938 > 1e-6 { 1.0 } else { 0.0 };
        locals.var_guard1217 = assign40090_e45824;
        locals.var_guard1217_rv = 0.0;

        if ((locals.var_guard1080 != 0.0) && (locals.var_guard1217 != 0.0)) {
            let assign40100_e45830: f64 = (locals.var_aexp1d__blk1007 * locals.var_inv_k1__blk906);
            (locals.var_b1d__blk1009, locals.var_b1d__blk1009_dn4, locals.var_b1d__blk1009_dn6, locals.var_b1d__blk1009_dn7, locals.var_b1d__blk1009_dn8, locals.var_b1d__blk1009_dn9, ) = (assign40100_e45830, ((locals.var_aexp1d__blk1007_dn4 * locals.var_inv_k1__blk906) + (locals.var_aexp1d__blk1007 * locals.var_inv_k1__blk906_dn4)), ((locals.var_aexp1d__blk1007_dn6 * locals.var_inv_k1__blk906) + (locals.var_aexp1d__blk1007 * locals.var_inv_k1__blk906_dn6)), ((locals.var_aexp1d__blk1007_dn7 * locals.var_inv_k1__blk906) + (locals.var_aexp1d__blk1007 * locals.var_inv_k1__blk906_dn7)), ((locals.var_aexp1d__blk1007_dn8 * locals.var_inv_k1__blk906) + (locals.var_aexp1d__blk1007 * locals.var_inv_k1__blk906_dn8)), ((locals.var_aexp1d__blk1007_dn9 * locals.var_inv_k1__blk906) + (locals.var_aexp1d__blk1007 * locals.var_inv_k1__blk906_dn9)), );
            locals.var_b1d__blk1009_rv = 0.0;
        }

        if ((locals.var_guard1080 != 0.0) && (locals.var_guard1217 != 0.0)) {
            let assign40110_e45838: f64 = (locals.var_aexp2d__blk1008 * locals.var_inv_k2__blk907);
            (locals.var_b2d__blk1010, locals.var_b2d__blk1010_dn4, locals.var_b2d__blk1010_dn6, locals.var_b2d__blk1010_dn7, locals.var_b2d__blk1010_dn8, locals.var_b2d__blk1010_dn9, ) = (assign40110_e45838, ((locals.var_aexp2d__blk1008_dn4 * locals.var_inv_k2__blk907) + (locals.var_aexp2d__blk1008 * locals.var_inv_k2__blk907_dn4)), ((locals.var_aexp2d__blk1008_dn6 * locals.var_inv_k2__blk907) + (locals.var_aexp2d__blk1008 * locals.var_inv_k2__blk907_dn6)), ((locals.var_aexp2d__blk1008_dn7 * locals.var_inv_k2__blk907) + (locals.var_aexp2d__blk1008 * locals.var_inv_k2__blk907_dn7)), ((locals.var_aexp2d__blk1008_dn8 * locals.var_inv_k2__blk907) + (locals.var_aexp2d__blk1008 * locals.var_inv_k2__blk907_dn8)), ((locals.var_aexp2d__blk1008_dn9 * locals.var_inv_k2__blk907) + (locals.var_aexp2d__blk1008 * locals.var_inv_k2__blk907_dn9)), );
            locals.var_b2d__blk1010_rv = 0.0;
        }

        if ((locals.var_guard1080 != 0.0) && (locals.var_guard1217 != 0.0)) {
            let assign40120_e45847: f64 = (2.0 * locals.var_k1q1d__blk1004);
            let assign40120_e45848: f64 = (locals.var_b1d__blk1009 + assign40120_e45847);
            (locals.var_a1d__blk1011, locals.var_a1d__blk1011_dn4, locals.var_a1d__blk1011_dn6, locals.var_a1d__blk1011_dn7, locals.var_a1d__blk1011_dn8, locals.var_a1d__blk1011_dn9, ) = (assign40120_e45848, (locals.var_b1d__blk1009_dn4 + (2.0 * locals.var_k1q1d__blk1004_dn4)), (locals.var_b1d__blk1009_dn6 + (2.0 * locals.var_k1q1d__blk1004_dn6)), (locals.var_b1d__blk1009_dn7 + (2.0 * locals.var_k1q1d__blk1004_dn7)), (locals.var_b1d__blk1009_dn8 + (2.0 * locals.var_k1q1d__blk1004_dn8)), (locals.var_b1d__blk1009_dn9 + (2.0 * locals.var_k1q1d__blk1004_dn9)), );
            locals.var_a1d__blk1011_rv = 0.0;
        }

        if ((locals.var_guard1080 != 0.0) && (locals.var_guard1217 != 0.0)) {
            let assign40130_e45857: f64 = (2.0 * locals.var_k2q2d__blk1005);
            let assign40130_e45858: f64 = (locals.var_b2d__blk1010 + assign40130_e45857);
            (locals.var_a2d__blk1012, locals.var_a2d__blk1012_dn4, locals.var_a2d__blk1012_dn6, locals.var_a2d__blk1012_dn7, locals.var_a2d__blk1012_dn8, locals.var_a2d__blk1012_dn9, ) = (assign40130_e45858, (locals.var_b2d__blk1010_dn4 + (2.0 * locals.var_k2q2d__blk1005_dn4)), (locals.var_b2d__blk1010_dn6 + (2.0 * locals.var_k2q2d__blk1005_dn6)), (locals.var_b2d__blk1010_dn7 + (2.0 * locals.var_k2q2d__blk1005_dn7)), (locals.var_b2d__blk1010_dn8 + (2.0 * locals.var_k2q2d__blk1005_dn8)), (locals.var_b2d__blk1010_dn9 + (2.0 * locals.var_k2q2d__blk1005_dn9)), );
            locals.var_a2d__blk1012_rv = 0.0;
        }

        if ((locals.var_guard1080 != 0.0) && (locals.var_guard1217 != 0.0)) {
            let assign40140_e45866: f64 = (2.0 * locals.var_qid__blk1003);
            let assign40140_e45868: f64 = (assign40140_e45866 + locals.var_b1d__blk1009);
            let assign40140_e45870: f64 = (assign40140_e45868 + locals.var_b2d__blk1010);
            (locals.var_sumd__blk1013, locals.var_sumd__blk1013_dn4, locals.var_sumd__blk1013_dn6, locals.var_sumd__blk1013_dn7, locals.var_sumd__blk1013_dn8, locals.var_sumd__blk1013_dn9, ) = (assign40140_e45870, (((2.0 * locals.var_qid__blk1003_dn4) + locals.var_b1d__blk1009_dn4) + locals.var_b2d__blk1010_dn4), (((2.0 * locals.var_qid__blk1003_dn6) + locals.var_b1d__blk1009_dn6) + locals.var_b2d__blk1010_dn6), (((2.0 * locals.var_qid__blk1003_dn7) + locals.var_b1d__blk1009_dn7) + locals.var_b2d__blk1010_dn7), (((2.0 * locals.var_qid__blk1003_dn8) + locals.var_b1d__blk1009_dn8) + locals.var_b2d__blk1010_dn8), (((2.0 * locals.var_qid__blk1003_dn9) + locals.var_b1d__blk1009_dn9) + locals.var_b2d__blk1010_dn9), );
            locals.var_sumd__blk1013_rv = 0.0;
        }

        let assign40150_e45874: f64 = (locals.var_qsqd__blk1006).abs();
        let assign40150_e45876: f64 = if assign40150_e45874 > 0.005 { 1.0 } else { 0.0 };
        locals.var_guard1218 = assign40150_e45876;
        locals.var_guard1218_rv = 0.0;

        if (((locals.var_guard1080 != 0.0) && (locals.var_guard1217 != 0.0)) && (locals.var_guard1218 != 0.0)) {
            let assign40160_e45884: f64 = (locals.var_a1d__blk1011 * locals.var_a2d__blk1012);
            let assign40160_e45888: f64 = (locals.var_q1d__blk1001 + 2.0);
            let assign40160_e45889: f64 = (2.0 * assign40160_e45888);
            let assign40160_e45891: f64 = (assign40160_e45889 * locals.var_a2d__blk1012);
            let assign40160_e45892: f64 = (assign40160_e45884 + assign40160_e45891);
            let assign40160_e45896: f64 = (locals.var_q2d__blk1002 + 2.0);
            let assign40160_e45897: f64 = (2.0 * assign40160_e45896);
            let assign40160_e45899: f64 = (assign40160_e45897 * locals.var_a1d__blk1011);
            let assign40160_e45900: f64 = (assign40160_e45892 + assign40160_e45899);
            (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9, ) = (assign40160_e45900, ((((locals.var_a1d__blk1011_dn4 * locals.var_a2d__blk1012) + (locals.var_a1d__blk1011 * locals.var_a2d__blk1012_dn4)) + (((2.0 * locals.var_q1d__blk1001_dn4) * locals.var_a2d__blk1012) + (assign40160_e45889 * locals.var_a2d__blk1012_dn4))) + (((2.0 * locals.var_q2d__blk1002_dn4) * locals.var_a1d__blk1011) + (assign40160_e45897 * locals.var_a1d__blk1011_dn4))), ((((locals.var_a1d__blk1011_dn6 * locals.var_a2d__blk1012) + (locals.var_a1d__blk1011 * locals.var_a2d__blk1012_dn6)) + (((2.0 * locals.var_q1d__blk1001_dn6) * locals.var_a2d__blk1012) + (assign40160_e45889 * locals.var_a2d__blk1012_dn6))) + (((2.0 * locals.var_q2d__blk1002_dn6) * locals.var_a1d__blk1011) + (assign40160_e45897 * locals.var_a1d__blk1011_dn6))), ((((locals.var_a1d__blk1011_dn7 * locals.var_a2d__blk1012) + (locals.var_a1d__blk1011 * locals.var_a2d__blk1012_dn7)) + (((2.0 * locals.var_q1d__blk1001_dn7) * locals.var_a2d__blk1012) + (assign40160_e45889 * locals.var_a2d__blk1012_dn7))) + (((2.0 * locals.var_q2d__blk1002_dn7) * locals.var_a1d__blk1011) + (assign40160_e45897 * locals.var_a1d__blk1011_dn7))), ((((locals.var_a1d__blk1011_dn8 * locals.var_a2d__blk1012) + (locals.var_a1d__blk1011 * locals.var_a2d__blk1012_dn8)) + (((2.0 * locals.var_q1d__blk1001_dn8) * locals.var_a2d__blk1012) + (assign40160_e45889 * locals.var_a2d__blk1012_dn8))) + (((2.0 * locals.var_q2d__blk1002_dn8) * locals.var_a1d__blk1011) + (assign40160_e45897 * locals.var_a1d__blk1011_dn8))), ((((locals.var_a1d__blk1011_dn9 * locals.var_a2d__blk1012) + (locals.var_a1d__blk1011 * locals.var_a2d__blk1012_dn9)) + (((2.0 * locals.var_q1d__blk1001_dn9) * locals.var_a2d__blk1012) + (assign40160_e45889 * locals.var_a2d__blk1012_dn9))) + (((2.0 * locals.var_q2d__blk1002_dn9) * locals.var_a1d__blk1011) + (assign40160_e45897 * locals.var_a1d__blk1011_dn9))), );
            locals.var_temp1_rv = 0.0;
        }

        if (((locals.var_guard1080 != 0.0) && (locals.var_guard1217 != 0.0)) && (locals.var_guard1218 != 0.0)) {
            let assign40170_e45909: f64 = (-4.0);
            let assign40170_e45911: f64 = (assign40170_e45909 * locals.var_qsqd__blk1006);
            let assign40170_e45913: f64 = (assign40170_e45911 * locals.var_sumd__blk1013);
            let assign40170_e45916: f64 = (locals.var_qid__blk1003 * locals.var_temp1);
            let assign40170_e45917: f64 = (assign40170_e45913 / assign40170_e45916);
            (locals.var_dqsqd_dxn_qi__blk1014, locals.var_dqsqd_dxn_qi__blk1014_dn4, locals.var_dqsqd_dxn_qi__blk1014_dn6, locals.var_dqsqd_dxn_qi__blk1014_dn7, locals.var_dqsqd_dxn_qi__blk1014_dn8, locals.var_dqsqd_dxn_qi__blk1014_dn9, ) = (assign40170_e45917, ((((((assign40170_e45909 * locals.var_qsqd__blk1006_dn4) * locals.var_sumd__blk1013) + (assign40170_e45911 * locals.var_sumd__blk1013_dn4)) * assign40170_e45916) - (assign40170_e45913 * ((locals.var_qid__blk1003_dn4 * locals.var_temp1) + (locals.var_qid__blk1003 * locals.var_temp1_dn4)))) / (assign40170_e45916 * assign40170_e45916)), ((((((assign40170_e45909 * locals.var_qsqd__blk1006_dn6) * locals.var_sumd__blk1013) + (assign40170_e45911 * locals.var_sumd__blk1013_dn6)) * assign40170_e45916) - (assign40170_e45913 * ((locals.var_qid__blk1003_dn6 * locals.var_temp1) + (locals.var_qid__blk1003 * locals.var_temp1_dn6)))) / (assign40170_e45916 * assign40170_e45916)), ((((((assign40170_e45909 * locals.var_qsqd__blk1006_dn7) * locals.var_sumd__blk1013) + (assign40170_e45911 * locals.var_sumd__blk1013_dn7)) * assign40170_e45916) - (assign40170_e45913 * ((locals.var_qid__blk1003_dn7 * locals.var_temp1) + (locals.var_qid__blk1003 * locals.var_temp1_dn7)))) / (assign40170_e45916 * assign40170_e45916)), ((((((assign40170_e45909 * locals.var_qsqd__blk1006_dn8) * locals.var_sumd__blk1013) + (assign40170_e45911 * locals.var_sumd__blk1013_dn8)) * assign40170_e45916) - (assign40170_e45913 * ((locals.var_qid__blk1003_dn8 * locals.var_temp1) + (locals.var_qid__blk1003 * locals.var_temp1_dn8)))) / (assign40170_e45916 * assign40170_e45916)), ((((((assign40170_e45909 * locals.var_qsqd__blk1006_dn9) * locals.var_sumd__blk1013) + (assign40170_e45911 * locals.var_sumd__blk1013_dn9)) * assign40170_e45916) - (assign40170_e45913 * ((locals.var_qid__blk1003_dn9 * locals.var_temp1) + (locals.var_qid__blk1003 * locals.var_temp1_dn9)))) / (assign40170_e45916 * assign40170_e45916)), );
            locals.var_dqsqd_dxn_qi__blk1014_rv = 0.0;
        }

        if (((locals.var_guard1080 != 0.0) && (locals.var_guard1217 != 0.0)) && (locals.var_guard1218 == 0.0)) {
            let assign40180_e45930: f64 = (locals.var_qsqd__blk1006 * 0.0333333333333);
            let assign40180_e45934: f64 = (locals.var_qsqd__blk1006 * 0.0357142857143);
            let assign40180_e45938: f64 = (locals.var_qsqd__blk1006 * 0.0333333333333);
            let assign40180_e45939: f64 = (1.0 - assign40180_e45938);
            let assign40180_e45940: f64 = (assign40180_e45934 * assign40180_e45939);
            let assign40180_e45941: f64 = (1.0 - assign40180_e45940);
            let assign40180_e45942: f64 = (assign40180_e45930 * assign40180_e45941);
            let assign40180_e45943: f64 = (1.0 - assign40180_e45942);
            let assign40180_e45944: f64 = (0.1666666666667 * assign40180_e45943);
            (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9, ) = (assign40180_e45944, (0.1666666666667 * (-(((locals.var_qsqd__blk1006_dn4 * 0.0333333333333) * assign40180_e45941) + (assign40180_e45930 * (-(((locals.var_qsqd__blk1006_dn4 * 0.0357142857143) * assign40180_e45939) + (assign40180_e45934 * (-(locals.var_qsqd__blk1006_dn4 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_qsqd__blk1006_dn6 * 0.0333333333333) * assign40180_e45941) + (assign40180_e45930 * (-(((locals.var_qsqd__blk1006_dn6 * 0.0357142857143) * assign40180_e45939) + (assign40180_e45934 * (-(locals.var_qsqd__blk1006_dn6 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_qsqd__blk1006_dn7 * 0.0333333333333) * assign40180_e45941) + (assign40180_e45930 * (-(((locals.var_qsqd__blk1006_dn7 * 0.0357142857143) * assign40180_e45939) + (assign40180_e45934 * (-(locals.var_qsqd__blk1006_dn7 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_qsqd__blk1006_dn8 * 0.0333333333333) * assign40180_e45941) + (assign40180_e45930 * (-(((locals.var_qsqd__blk1006_dn8 * 0.0357142857143) * assign40180_e45939) + (assign40180_e45934 * (-(locals.var_qsqd__blk1006_dn8 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_qsqd__blk1006_dn9 * 0.0333333333333) * assign40180_e45941) + (assign40180_e45930 * (-(((locals.var_qsqd__blk1006_dn9 * 0.0357142857143) * assign40180_e45939) + (assign40180_e45934 * (-(locals.var_qsqd__blk1006_dn9 * 0.0333333333333))))))))), );
            locals.var_temp1_rv = 0.0;
        }

        if (((locals.var_guard1080 != 0.0) && (locals.var_guard1217 != 0.0)) && (locals.var_guard1218 == 0.0)) {
            let assign40190_e45955: f64 = (locals.var_a1d__blk1011 * locals.var_aexp1d__blk1007);
            let assign40190_e45958: f64 = (locals.var_a2d__blk1012 * locals.var_aexp2d__blk1008);
            let assign40190_e45959: f64 = (assign40190_e45955 + assign40190_e45958);
            let assign40190_e45962: f64 = (locals.var_a1d__blk1011 * locals.var_a2d__blk1012);
            let assign40190_e45964: f64 = (assign40190_e45962 * locals.var_qid__blk1003);
            let assign40190_e45968: f64 = (locals.var_qid__blk1003 * locals.var_temp1);
            let assign40190_e45969: f64 = (1.0 + assign40190_e45968);
            let assign40190_e45970: f64 = (assign40190_e45964 * assign40190_e45969);
            let assign40190_e45971: f64 = (assign40190_e45959 + assign40190_e45970);
            (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9, ) = (assign40190_e45971, ((((locals.var_a1d__blk1011_dn4 * locals.var_aexp1d__blk1007) + (locals.var_a1d__blk1011 * locals.var_aexp1d__blk1007_dn4)) + ((locals.var_a2d__blk1012_dn4 * locals.var_aexp2d__blk1008) + (locals.var_a2d__blk1012 * locals.var_aexp2d__blk1008_dn4))) + ((((((locals.var_a1d__blk1011_dn4 * locals.var_a2d__blk1012) + (locals.var_a1d__blk1011 * locals.var_a2d__blk1012_dn4)) * locals.var_qid__blk1003) + (assign40190_e45962 * locals.var_qid__blk1003_dn4)) * assign40190_e45969) + (assign40190_e45964 * ((locals.var_qid__blk1003_dn4 * locals.var_temp1) + (locals.var_qid__blk1003 * locals.var_temp1_dn4))))), ((((locals.var_a1d__blk1011_dn6 * locals.var_aexp1d__blk1007) + (locals.var_a1d__blk1011 * locals.var_aexp1d__blk1007_dn6)) + ((locals.var_a2d__blk1012_dn6 * locals.var_aexp2d__blk1008) + (locals.var_a2d__blk1012 * locals.var_aexp2d__blk1008_dn6))) + ((((((locals.var_a1d__blk1011_dn6 * locals.var_a2d__blk1012) + (locals.var_a1d__blk1011 * locals.var_a2d__blk1012_dn6)) * locals.var_qid__blk1003) + (assign40190_e45962 * locals.var_qid__blk1003_dn6)) * assign40190_e45969) + (assign40190_e45964 * ((locals.var_qid__blk1003_dn6 * locals.var_temp1) + (locals.var_qid__blk1003 * locals.var_temp1_dn6))))), ((((locals.var_a1d__blk1011_dn7 * locals.var_aexp1d__blk1007) + (locals.var_a1d__blk1011 * locals.var_aexp1d__blk1007_dn7)) + ((locals.var_a2d__blk1012_dn7 * locals.var_aexp2d__blk1008) + (locals.var_a2d__blk1012 * locals.var_aexp2d__blk1008_dn7))) + ((((((locals.var_a1d__blk1011_dn7 * locals.var_a2d__blk1012) + (locals.var_a1d__blk1011 * locals.var_a2d__blk1012_dn7)) * locals.var_qid__blk1003) + (assign40190_e45962 * locals.var_qid__blk1003_dn7)) * assign40190_e45969) + (assign40190_e45964 * ((locals.var_qid__blk1003_dn7 * locals.var_temp1) + (locals.var_qid__blk1003 * locals.var_temp1_dn7))))), ((((locals.var_a1d__blk1011_dn8 * locals.var_aexp1d__blk1007) + (locals.var_a1d__blk1011 * locals.var_aexp1d__blk1007_dn8)) + ((locals.var_a2d__blk1012_dn8 * locals.var_aexp2d__blk1008) + (locals.var_a2d__blk1012 * locals.var_aexp2d__blk1008_dn8))) + ((((((locals.var_a1d__blk1011_dn8 * locals.var_a2d__blk1012) + (locals.var_a1d__blk1011 * locals.var_a2d__blk1012_dn8)) * locals.var_qid__blk1003) + (assign40190_e45962 * locals.var_qid__blk1003_dn8)) * assign40190_e45969) + (assign40190_e45964 * ((locals.var_qid__blk1003_dn8 * locals.var_temp1) + (locals.var_qid__blk1003 * locals.var_temp1_dn8))))), ((((locals.var_a1d__blk1011_dn9 * locals.var_aexp1d__blk1007) + (locals.var_a1d__blk1011 * locals.var_aexp1d__blk1007_dn9)) + ((locals.var_a2d__blk1012_dn9 * locals.var_aexp2d__blk1008) + (locals.var_a2d__blk1012 * locals.var_aexp2d__blk1008_dn9))) + ((((((locals.var_a1d__blk1011_dn9 * locals.var_a2d__blk1012) + (locals.var_a1d__blk1011 * locals.var_a2d__blk1012_dn9)) * locals.var_qid__blk1003) + (assign40190_e45962 * locals.var_qid__blk1003_dn9)) * assign40190_e45969) + (assign40190_e45964 * ((locals.var_qid__blk1003_dn9 * locals.var_temp1) + (locals.var_qid__blk1003 * locals.var_temp1_dn9))))), );
            locals.var_temp2_rv = 0.0;
        }

        if (((locals.var_guard1080 != 0.0) && (locals.var_guard1217 != 0.0)) && (locals.var_guard1218 == 0.0)) {
            let assign40200_e45982: f64 = (locals.var_aexp1d__blk1007 * locals.var_aexp2d__blk1008);
            let assign40200_e45984: f64 = (assign40200_e45982 * locals.var_sumd__blk1013);
            let assign40200_e45987: f64 = (locals.var_qid__blk1003 * locals.var_temp2);
            let assign40200_e45988: f64 = (assign40200_e45984 / assign40200_e45987);
            (locals.var_dqsqd_dxn_qi__blk1014, locals.var_dqsqd_dxn_qi__blk1014_dn4, locals.var_dqsqd_dxn_qi__blk1014_dn6, locals.var_dqsqd_dxn_qi__blk1014_dn7, locals.var_dqsqd_dxn_qi__blk1014_dn8, locals.var_dqsqd_dxn_qi__blk1014_dn9, ) = (assign40200_e45988, (((((((locals.var_aexp1d__blk1007_dn4 * locals.var_aexp2d__blk1008) + (locals.var_aexp1d__blk1007 * locals.var_aexp2d__blk1008_dn4)) * locals.var_sumd__blk1013) + (assign40200_e45982 * locals.var_sumd__blk1013_dn4)) * assign40200_e45987) - (assign40200_e45984 * ((locals.var_qid__blk1003_dn4 * locals.var_temp2) + (locals.var_qid__blk1003 * locals.var_temp2_dn4)))) / (assign40200_e45987 * assign40200_e45987)), (((((((locals.var_aexp1d__blk1007_dn6 * locals.var_aexp2d__blk1008) + (locals.var_aexp1d__blk1007 * locals.var_aexp2d__blk1008_dn6)) * locals.var_sumd__blk1013) + (assign40200_e45982 * locals.var_sumd__blk1013_dn6)) * assign40200_e45987) - (assign40200_e45984 * ((locals.var_qid__blk1003_dn6 * locals.var_temp2) + (locals.var_qid__blk1003 * locals.var_temp2_dn6)))) / (assign40200_e45987 * assign40200_e45987)), (((((((locals.var_aexp1d__blk1007_dn7 * locals.var_aexp2d__blk1008) + (locals.var_aexp1d__blk1007 * locals.var_aexp2d__blk1008_dn7)) * locals.var_sumd__blk1013) + (assign40200_e45982 * locals.var_sumd__blk1013_dn7)) * assign40200_e45987) - (assign40200_e45984 * ((locals.var_qid__blk1003_dn7 * locals.var_temp2) + (locals.var_qid__blk1003 * locals.var_temp2_dn7)))) / (assign40200_e45987 * assign40200_e45987)), (((((((locals.var_aexp1d__blk1007_dn8 * locals.var_aexp2d__blk1008) + (locals.var_aexp1d__blk1007 * locals.var_aexp2d__blk1008_dn8)) * locals.var_sumd__blk1013) + (assign40200_e45982 * locals.var_sumd__blk1013_dn8)) * assign40200_e45987) - (assign40200_e45984 * ((locals.var_qid__blk1003_dn8 * locals.var_temp2) + (locals.var_qid__blk1003 * locals.var_temp2_dn8)))) / (assign40200_e45987 * assign40200_e45987)), (((((((locals.var_aexp1d__blk1007_dn9 * locals.var_aexp2d__blk1008) + (locals.var_aexp1d__blk1007 * locals.var_aexp2d__blk1008_dn9)) * locals.var_sumd__blk1013) + (assign40200_e45982 * locals.var_sumd__blk1013_dn9)) * assign40200_e45987) - (assign40200_e45984 * ((locals.var_qid__blk1003_dn9 * locals.var_temp2) + (locals.var_qid__blk1003 * locals.var_temp2_dn9)))) / (assign40200_e45987 * assign40200_e45987)), );
            locals.var_dqsqd_dxn_qi__blk1014_rv = 0.0;
        }

        if (locals.var_guard1080 != 0.0) {
            let assign40210_e45994: f64 = (locals.var_qid__blk1003).ln();
            let assign40210_e45995: f64 = (locals.var_xdeff__blk1000 + assign40210_e45994);
            (locals.var_xdriftd__blk1015, locals.var_xdriftd__blk1015_dn4, locals.var_xdriftd__blk1015_dn6, locals.var_xdriftd__blk1015_dn7, locals.var_xdriftd__blk1015_dn8, locals.var_xdriftd__blk1015_dn9, ) = (assign40210_e45995, (locals.var_xdeff__blk1000_dn4 + (locals.var_qid__blk1003_dn4 / locals.var_qid__blk1003)), (locals.var_xdeff__blk1000_dn6 + (locals.var_qid__blk1003_dn6 / locals.var_qid__blk1003)), (locals.var_xdeff__blk1000_dn7 + (locals.var_qid__blk1003_dn7 / locals.var_qid__blk1003)), (locals.var_xdeff__blk1000_dn8 + (locals.var_qid__blk1003_dn8 / locals.var_qid__blk1003)), (locals.var_xdeff__blk1000_dn9 + (locals.var_qid__blk1003_dn9 / locals.var_qid__blk1003)), );
            locals.var_xdriftd__blk1015_rv = 0.0;
        }

        if (locals.var_guard1080 != 0.0) {
            let assign40220_e46002: f64 = (locals.var_qis__blk938 + locals.var_qid__blk1003);
            let assign40220_e46003: f64 = (0.5 * assign40220_e46002);
            (locals.var_qim__blk1016, locals.var_qim__blk1016_dn4, locals.var_qim__blk1016_dn6, locals.var_qim__blk1016_dn7, locals.var_qim__blk1016_dn8, locals.var_qim__blk1016_dn9, ) = (assign40220_e46003, (0.5 * (locals.var_qis__blk938_dn4 + locals.var_qid__blk1003_dn4)), (0.5 * (locals.var_qis__blk938_dn6 + locals.var_qid__blk1003_dn6)), (0.5 * (locals.var_qis__blk938_dn7 + locals.var_qid__blk1003_dn7)), (0.5 * (locals.var_qis__blk938_dn8 + locals.var_qid__blk1003_dn8)), (0.5 * (locals.var_qis__blk938_dn9 + locals.var_qid__blk1003_dn9)), );
            locals.var_qim__blk1016_rv = 0.0;
        }

        if (locals.var_guard1080 != 0.0) {
            let assign40230_e46009: f64 = (locals.var_xdriftd__blk1015 - locals.var_xdrifts__blk951);
            (locals.var_dxdrift__blk1017, locals.var_dxdrift__blk1017_dn4, locals.var_dxdrift__blk1017_dn6, locals.var_dxdrift__blk1017_dn7, locals.var_dxdrift__blk1017_dn8, locals.var_dxdrift__blk1017_dn9, ) = (assign40230_e46009, (locals.var_xdriftd__blk1015_dn4 - locals.var_xdrifts__blk951_dn4), (locals.var_xdriftd__blk1015_dn6 - locals.var_xdrifts__blk951_dn6), (locals.var_xdriftd__blk1015_dn7 - locals.var_xdrifts__blk951_dn7), (locals.var_xdriftd__blk1015_dn8 - locals.var_xdrifts__blk951_dn8), (locals.var_xdriftd__blk1015_dn9 - locals.var_xdrifts__blk951_dn9), );
            locals.var_dxdrift__blk1017_rv = 0.0;
        }

        if (locals.var_guard1080 != 0.0) {
            (locals.var_ratio_pd__blk1020, locals.var_ratio_pd__blk1020_dn4, locals.var_ratio_pd__blk1020_dn6, locals.var_ratio_pd__blk1020_dn7, locals.var_ratio_pd__blk1020_dn8, locals.var_ratio_pd__blk1020_dn9, ) = (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_ratio_pd__blk1020_rv = 0.0;
        }

        let assign40250_e46018: f64 = if p.p9 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1219 = assign40250_e46018;
        locals.var_guard1219_rv = 0.0;

        if ((locals.var_guard1080 != 0.0) && (locals.var_guard1219 != 0.0)) {
            let assign40260_e46025: f64 = (locals.var_k1q1s__blk939 + locals.var_k1q1d__blk1004);
            let assign40260_e46026: f64 = (0.5 * assign40260_e46025);
            let assign40260_e46028: f64 = (assign40260_e46026 / locals.var_k1__blk932);
            (locals.var_qim_pd__blk1018, locals.var_qim_pd__blk1018_dn4, locals.var_qim_pd__blk1018_dn6, locals.var_qim_pd__blk1018_dn7, locals.var_qim_pd__blk1018_dn8, locals.var_qim_pd__blk1018_dn9, ) = (assign40260_e46028, ((((0.5 * (locals.var_k1q1s__blk939_dn4 + locals.var_k1q1d__blk1004_dn4)) * locals.var_k1__blk932) - (assign40260_e46026 * locals.var_k1__blk932_dn4)) / (locals.var_k1__blk932 * locals.var_k1__blk932)), ((((0.5 * (locals.var_k1q1s__blk939_dn6 + locals.var_k1q1d__blk1004_dn6)) * locals.var_k1__blk932) - (assign40260_e46026 * locals.var_k1__blk932_dn6)) / (locals.var_k1__blk932 * locals.var_k1__blk932)), ((((0.5 * (locals.var_k1q1s__blk939_dn7 + locals.var_k1q1d__blk1004_dn7)) * locals.var_k1__blk932) - (assign40260_e46026 * locals.var_k1__blk932_dn7)) / (locals.var_k1__blk932 * locals.var_k1__blk932)), ((((0.5 * (locals.var_k1q1s__blk939_dn8 + locals.var_k1q1d__blk1004_dn8)) * locals.var_k1__blk932) - (assign40260_e46026 * locals.var_k1__blk932_dn8)) / (locals.var_k1__blk932 * locals.var_k1__blk932)), ((((0.5 * (locals.var_k1q1s__blk939_dn9 + locals.var_k1q1d__blk1004_dn9)) * locals.var_k1__blk932) - (assign40260_e46026 * locals.var_k1__blk932_dn9)) / (locals.var_k1__blk932 * locals.var_k1__blk932)), );
            locals.var_qim_pd__blk1018_rv = 0.0;
        }

        if ((locals.var_guard1080 != 0.0) && (locals.var_guard1219 != 0.0)) {
            let assign40270_e46037: f64 = (locals.var_qim_pd__blk1018 + 1e-5);
            let assign40270_e46040: f64 = (locals.var_qim_pd__blk1018 - 1e-5);
            let assign40270_e46043: f64 = (locals.var_qim_pd__blk1018 - 1e-5);
            let assign40270_e46044: f64 = (assign40270_e46040 * assign40270_e46043);
            let assign40270_e46046: f64 = (assign40270_e46044 + 1.0);
            let assign40270_e46047: f64 = (assign40270_e46046).sqrt();
            let assign40270_e46048: f64 = (assign40270_e46037 + assign40270_e46047);
            let assign40270_e46049: f64 = (0.5 * assign40270_e46048);
            (locals.var_qim_pd__blk1018, locals.var_qim_pd__blk1018_dn4, locals.var_qim_pd__blk1018_dn6, locals.var_qim_pd__blk1018_dn7, locals.var_qim_pd__blk1018_dn8, locals.var_qim_pd__blk1018_dn9, ) = (assign40270_e46049, (0.5 * (locals.var_qim_pd__blk1018_dn4 + (((locals.var_qim_pd__blk1018_dn4 * assign40270_e46043) + (assign40270_e46040 * locals.var_qim_pd__blk1018_dn4)) / (2.0 * assign40270_e46047)))), (0.5 * (locals.var_qim_pd__blk1018_dn6 + (((locals.var_qim_pd__blk1018_dn6 * assign40270_e46043) + (assign40270_e46040 * locals.var_qim_pd__blk1018_dn6)) / (2.0 * assign40270_e46047)))), (0.5 * (locals.var_qim_pd__blk1018_dn7 + (((locals.var_qim_pd__blk1018_dn7 * assign40270_e46043) + (assign40270_e46040 * locals.var_qim_pd__blk1018_dn7)) / (2.0 * assign40270_e46047)))), (0.5 * (locals.var_qim_pd__blk1018_dn8 + (((locals.var_qim_pd__blk1018_dn8 * assign40270_e46043) + (assign40270_e46040 * locals.var_qim_pd__blk1018_dn8)) / (2.0 * assign40270_e46047)))), (0.5 * (locals.var_qim_pd__blk1018_dn9 + (((locals.var_qim_pd__blk1018_dn9 * assign40270_e46043) + (assign40270_e46040 * locals.var_qim_pd__blk1018_dn9)) / (2.0 * assign40270_e46047)))), );
            locals.var_qim_pd__blk1018_rv = 0.0;
        }

        if ((locals.var_guard1080 != 0.0) && (locals.var_guard1219 != 0.0)) {
            let assign40280_e46057: f64 = (locals.var_qim_pd__blk1018 / locals.var_inv_phit);
            let assign40280_e46060: f64 = (0.25 * locals.var_kp);
            let assign40280_e46062: f64 = (assign40280_e46060 * locals.var_kp);
            let assign40280_e46063: f64 = (assign40280_e46057 + assign40280_e46062);
            let assign40280_e46064: f64 = (assign40280_e46063).sqrt();
            let assign40280_e46067: f64 = (0.5 * locals.var_kp);
            let assign40280_e46068: f64 = (assign40280_e46064 - assign40280_e46067);
            (locals.var_temp0, locals.var_temp0_dn4, locals.var_temp0_dn6, locals.var_temp0_dn7, locals.var_temp0_dn8, locals.var_temp0_dn9, ) = (assign40280_e46068, ((((((locals.var_qim_pd__blk1018_dn4 * locals.var_inv_phit) - (locals.var_qim_pd__blk1018 * locals.var_inv_phit_dn4)) / (locals.var_inv_phit * locals.var_inv_phit)) + (((0.25 * locals.var_kp_dn4) * locals.var_kp) + (assign40280_e46060 * locals.var_kp_dn4))) / (2.0 * assign40280_e46064)) - (0.5 * locals.var_kp_dn4)), ((((((locals.var_qim_pd__blk1018_dn6 * locals.var_inv_phit) - (locals.var_qim_pd__blk1018 * locals.var_inv_phit_dn6)) / (locals.var_inv_phit * locals.var_inv_phit)) + (((0.25 * locals.var_kp_dn6) * locals.var_kp) + (assign40280_e46060 * locals.var_kp_dn6))) / (2.0 * assign40280_e46064)) - (0.5 * locals.var_kp_dn6)), ((((((locals.var_qim_pd__blk1018_dn7 * locals.var_inv_phit) - (locals.var_qim_pd__blk1018 * locals.var_inv_phit_dn7)) / (locals.var_inv_phit * locals.var_inv_phit)) + (((0.25 * locals.var_kp_dn7) * locals.var_kp) + (assign40280_e46060 * locals.var_kp_dn7))) / (2.0 * assign40280_e46064)) - (0.5 * locals.var_kp_dn7)), ((((((locals.var_qim_pd__blk1018_dn8 * locals.var_inv_phit) - (locals.var_qim_pd__blk1018 * locals.var_inv_phit_dn8)) / (locals.var_inv_phit * locals.var_inv_phit)) + (((0.25 * locals.var_kp_dn8) * locals.var_kp) + (assign40280_e46060 * locals.var_kp_dn8))) / (2.0 * assign40280_e46064)) - (0.5 * locals.var_kp_dn8)), ((((((locals.var_qim_pd__blk1018_dn9 * locals.var_inv_phit) - (locals.var_qim_pd__blk1018 * locals.var_inv_phit_dn9)) / (locals.var_inv_phit * locals.var_inv_phit)) + (((0.25 * locals.var_kp_dn9) * locals.var_kp) + (assign40280_e46060 * locals.var_kp_dn9))) / (2.0 * assign40280_e46064)) - (0.5 * locals.var_kp_dn9)), );
            locals.var_temp0_rv = 0.0;
        }

        if ((locals.var_guard1080 != 0.0) && (locals.var_guard1219 != 0.0)) {
            let assign40290_e46076: f64 = (locals.var_temp0).powf(2.0);
            let assign40290_e46078: f64 = (assign40290_e46076 * locals.var_inv_phit);
            (locals.var_xp_pd__blk1019, locals.var_xp_pd__blk1019_dn4, locals.var_xp_pd__blk1019_dn6, locals.var_xp_pd__blk1019_dn7, locals.var_xp_pd__blk1019_dn8, locals.var_xp_pd__blk1019_dn9, ) = (assign40290_e46078, ((if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((locals.var_temp0).powf(2.0 - 1.0) * locals.var_temp0_dn4)) } } else { (assign40290_e46076 * (2.0 * (locals.var_temp0_dn4 / locals.var_temp0))) } * locals.var_inv_phit) + (assign40290_e46076 * locals.var_inv_phit_dn4)), ((if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((locals.var_temp0).powf(2.0 - 1.0) * locals.var_temp0_dn6)) } } else { (assign40290_e46076 * (2.0 * (locals.var_temp0_dn6 / locals.var_temp0))) } * locals.var_inv_phit) + (assign40290_e46076 * locals.var_inv_phit_dn6)), ((if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((locals.var_temp0).powf(2.0 - 1.0) * locals.var_temp0_dn7)) } } else { (assign40290_e46076 * (2.0 * (locals.var_temp0_dn7 / locals.var_temp0))) } * locals.var_inv_phit) + (assign40290_e46076 * locals.var_inv_phit_dn7)), ((if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((locals.var_temp0).powf(2.0 - 1.0) * locals.var_temp0_dn8)) } } else { (assign40290_e46076 * (2.0 * (locals.var_temp0_dn8 / locals.var_temp0))) } * locals.var_inv_phit) + (assign40290_e46076 * locals.var_inv_phit_dn8)), ((if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((locals.var_temp0).powf(2.0 - 1.0) * locals.var_temp0_dn9)) } } else { (assign40290_e46076 * (2.0 * (locals.var_temp0_dn9 / locals.var_temp0))) } * locals.var_inv_phit) + (assign40290_e46076 * locals.var_inv_phit_dn9)), );
            locals.var_xp_pd__blk1019_rv = 0.0;
        }

        if ((locals.var_guard1080 != 0.0) && (locals.var_guard1219 != 0.0)) {
            let assign40300_e46087: f64 = (locals.var_xp_pd__blk1019 / locals.var_qim_pd__blk1018);
            let assign40300_e46088: f64 = (1.0 - assign40300_e46087);
            (locals.var_ratio_pd__blk1020, locals.var_ratio_pd__blk1020_dn4, locals.var_ratio_pd__blk1020_dn6, locals.var_ratio_pd__blk1020_dn7, locals.var_ratio_pd__blk1020_dn8, locals.var_ratio_pd__blk1020_dn9, ) = (assign40300_e46088, (-(((locals.var_xp_pd__blk1019_dn4 * locals.var_qim_pd__blk1018) - (locals.var_xp_pd__blk1019 * locals.var_qim_pd__blk1018_dn4)) / (locals.var_qim_pd__blk1018 * locals.var_qim_pd__blk1018))), (-(((locals.var_xp_pd__blk1019_dn6 * locals.var_qim_pd__blk1018) - (locals.var_xp_pd__blk1019 * locals.var_qim_pd__blk1018_dn6)) / (locals.var_qim_pd__blk1018 * locals.var_qim_pd__blk1018))), (-(((locals.var_xp_pd__blk1019_dn7 * locals.var_qim_pd__blk1018) - (locals.var_xp_pd__blk1019 * locals.var_qim_pd__blk1018_dn7)) / (locals.var_qim_pd__blk1018 * locals.var_qim_pd__blk1018))), (-(((locals.var_xp_pd__blk1019_dn8 * locals.var_qim_pd__blk1018) - (locals.var_xp_pd__blk1019 * locals.var_qim_pd__blk1018_dn8)) / (locals.var_qim_pd__blk1018 * locals.var_qim_pd__blk1018))), (-(((locals.var_xp_pd__blk1019_dn9 * locals.var_qim_pd__blk1018) - (locals.var_xp_pd__blk1019 * locals.var_qim_pd__blk1018_dn9)) / (locals.var_qim_pd__blk1018 * locals.var_qim_pd__blk1018))), );
            locals.var_ratio_pd__blk1020_rv = 0.0;
        }

        let assign40310_e46093: f64 = (locals.var_k1q1d__blk1004 / 2.0);
        let assign40310_e46095: f64 = if assign40310_e46093 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard1220 = assign40310_e46095;
        locals.var_guard1220_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_58(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        if ((locals.var_guard1080 != 0.0) && (locals.var_guard1220 != 0.0)) {
            let assign40320_e46102: f64 = (locals.var_k1q1d__blk1004 / 2.0);
            let assign40320_e46103: f64 = (assign40320_e46102).exp();
            let assign40320_e46104: f64 = (1.0 + assign40320_e46103);
            let assign40320_e46105: f64 = (assign40320_e46104).ln();
            (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9, ) = (assign40320_e46105, ((assign40320_e46103 * (locals.var_k1q1d__blk1004_dn4 / 2.0)) / assign40320_e46104), ((assign40320_e46103 * (locals.var_k1q1d__blk1004_dn6 / 2.0)) / assign40320_e46104), ((assign40320_e46103 * (locals.var_k1q1d__blk1004_dn7 / 2.0)) / assign40320_e46104), ((assign40320_e46103 * (locals.var_k1q1d__blk1004_dn8 / 2.0)) / assign40320_e46104), ((assign40320_e46103 * (locals.var_k1q1d__blk1004_dn9 / 2.0)) / assign40320_e46104), );
            locals.var_temp1_rv = 0.0;
        }

        if ((locals.var_guard1080 != 0.0) && (locals.var_guard1220 == 0.0)) {
            let assign40330_e46114: f64 = (locals.var_k1q1d__blk1004 / 2.0);
            (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9, ) = (assign40330_e46114, (locals.var_k1q1d__blk1004_dn4 / 2.0), (locals.var_k1q1d__blk1004_dn6 / 2.0), (locals.var_k1q1d__blk1004_dn7 / 2.0), (locals.var_k1q1d__blk1004_dn8 / 2.0), (locals.var_k1q1d__blk1004_dn9 / 2.0), );
            locals.var_temp1_rv = 0.0;
        }

        if (locals.var_guard1080 != 0.0) {
            let assign40340_e46120: f64 = (2.0 * locals.var_temp1);
            (locals.var_esurf1d__blk1021, locals.var_esurf1d__blk1021_dn4, locals.var_esurf1d__blk1021_dn6, locals.var_esurf1d__blk1021_dn7, locals.var_esurf1d__blk1021_dn8, locals.var_esurf1d__blk1021_dn9, ) = (assign40340_e46120, (2.0 * locals.var_temp1_dn4), (2.0 * locals.var_temp1_dn6), (2.0 * locals.var_temp1_dn7), (2.0 * locals.var_temp1_dn8), (2.0 * locals.var_temp1_dn9), );
            locals.var_esurf1d__blk1021_rv = 0.0;
        }

        let assign40350_e46125: f64 = (locals.var_k2q2d__blk1005 / 2.0);
        let assign40350_e46127: f64 = if assign40350_e46125 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard1221 = assign40350_e46127;
        locals.var_guard1221_rv = 0.0;

        if ((locals.var_guard1080 != 0.0) && (locals.var_guard1221 != 0.0)) {
            let assign40360_e46134: f64 = (locals.var_k2q2d__blk1005 / 2.0);
            let assign40360_e46135: f64 = (assign40360_e46134).exp();
            let assign40360_e46136: f64 = (1.0 + assign40360_e46135);
            let assign40360_e46137: f64 = (assign40360_e46136).ln();
            (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9, ) = (assign40360_e46137, ((assign40360_e46135 * (locals.var_k2q2d__blk1005_dn4 / 2.0)) / assign40360_e46136), ((assign40360_e46135 * (locals.var_k2q2d__blk1005_dn6 / 2.0)) / assign40360_e46136), ((assign40360_e46135 * (locals.var_k2q2d__blk1005_dn7 / 2.0)) / assign40360_e46136), ((assign40360_e46135 * (locals.var_k2q2d__blk1005_dn8 / 2.0)) / assign40360_e46136), ((assign40360_e46135 * (locals.var_k2q2d__blk1005_dn9 / 2.0)) / assign40360_e46136), );
            locals.var_temp2_rv = 0.0;
        }

        if ((locals.var_guard1080 != 0.0) && (locals.var_guard1221 == 0.0)) {
            let assign40370_e46146: f64 = (locals.var_k2q2d__blk1005 / 2.0);
            (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9, ) = (assign40370_e46146, (locals.var_k2q2d__blk1005_dn4 / 2.0), (locals.var_k2q2d__blk1005_dn6 / 2.0), (locals.var_k2q2d__blk1005_dn7 / 2.0), (locals.var_k2q2d__blk1005_dn8 / 2.0), (locals.var_k2q2d__blk1005_dn9 / 2.0), );
            locals.var_temp2_rv = 0.0;
        }

        if (locals.var_guard1080 != 0.0) {
            let assign40380_e46152: f64 = (2.0 * locals.var_temp2);
            (locals.var_esurf2d__blk1022, locals.var_esurf2d__blk1022_dn4, locals.var_esurf2d__blk1022_dn6, locals.var_esurf2d__blk1022_dn7, locals.var_esurf2d__blk1022_dn8, locals.var_esurf2d__blk1022_dn9, ) = (assign40380_e46152, (2.0 * locals.var_temp2_dn4), (2.0 * locals.var_temp2_dn6), (2.0 * locals.var_temp2_dn7), (2.0 * locals.var_temp2_dn8), (2.0 * locals.var_temp2_dn9), );
            locals.var_esurf2d__blk1022_rv = 0.0;
        }

        if (locals.var_guard1080 != 0.0) {
            let assign40390_e46158: f64 = (locals.var_esurf2d__blk1022 - locals.var_k2q2d__blk1005);
            (locals.var_ecpl1d__blk1023, locals.var_ecpl1d__blk1023_dn4, locals.var_ecpl1d__blk1023_dn6, locals.var_ecpl1d__blk1023_dn7, locals.var_ecpl1d__blk1023_dn8, locals.var_ecpl1d__blk1023_dn9, ) = (assign40390_e46158, (locals.var_esurf2d__blk1022_dn4 - locals.var_k2q2d__blk1005_dn4), (locals.var_esurf2d__blk1022_dn6 - locals.var_k2q2d__blk1005_dn6), (locals.var_esurf2d__blk1022_dn7 - locals.var_k2q2d__blk1005_dn7), (locals.var_esurf2d__blk1022_dn8 - locals.var_k2q2d__blk1005_dn8), (locals.var_esurf2d__blk1022_dn9 - locals.var_k2q2d__blk1005_dn9), );
            locals.var_ecpl1d__blk1023_rv = 0.0;
        }

        if (locals.var_guard1080 != 0.0) {
            let assign40400_e46164: f64 = (locals.var_esurf1d__blk1021 - locals.var_k1q1d__blk1004);
            (locals.var_ecpl2d__blk1024, locals.var_ecpl2d__blk1024_dn4, locals.var_ecpl2d__blk1024_dn6, locals.var_ecpl2d__blk1024_dn7, locals.var_ecpl2d__blk1024_dn8, locals.var_ecpl2d__blk1024_dn9, ) = (assign40400_e46164, (locals.var_esurf1d__blk1021_dn4 - locals.var_k1q1d__blk1004_dn4), (locals.var_esurf1d__blk1021_dn6 - locals.var_k1q1d__blk1004_dn6), (locals.var_esurf1d__blk1021_dn7 - locals.var_k1q1d__blk1004_dn7), (locals.var_esurf1d__blk1021_dn8 - locals.var_k1q1d__blk1004_dn8), (locals.var_esurf1d__blk1021_dn9 - locals.var_k1q1d__blk1004_dn9), );
            locals.var_ecpl2d__blk1024_rv = 0.0;
        }

        if (locals.var_guard1080 != 0.0) {
            let assign40410_e46170: f64 = (locals.var_eta_mu * locals.var_esurf1d__blk1021);
            let assign40410_e46173: f64 = (locals.var_one_m_eta * locals.var_ecpl1d__blk1023);
            let assign40410_e46174: f64 = (assign40410_e46170 + assign40410_e46173);
            (locals.var_eeff1d__blk1025, locals.var_eeff1d__blk1025_dn4, locals.var_eeff1d__blk1025_dn6, locals.var_eeff1d__blk1025_dn7, locals.var_eeff1d__blk1025_dn8, locals.var_eeff1d__blk1025_dn9, ) = (assign40410_e46174, ((locals.var_eta_mu * locals.var_esurf1d__blk1021_dn4) + (locals.var_one_m_eta * locals.var_ecpl1d__blk1023_dn4)), ((locals.var_eta_mu * locals.var_esurf1d__blk1021_dn6) + (locals.var_one_m_eta * locals.var_ecpl1d__blk1023_dn6)), ((locals.var_eta_mu * locals.var_esurf1d__blk1021_dn7) + (locals.var_one_m_eta * locals.var_ecpl1d__blk1023_dn7)), ((locals.var_eta_mu * locals.var_esurf1d__blk1021_dn8) + (locals.var_one_m_eta * locals.var_ecpl1d__blk1023_dn8)), ((locals.var_eta_mu * locals.var_esurf1d__blk1021_dn9) + (locals.var_one_m_eta * locals.var_ecpl1d__blk1023_dn9)), );
            locals.var_eeff1d__blk1025_rv = 0.0;
        }

        if (locals.var_guard1080 != 0.0) {
            let assign40420_e46180: f64 = (locals.var_eta_mu * locals.var_esurf2d__blk1022);
            let assign40420_e46183: f64 = (locals.var_one_m_eta * locals.var_ecpl2d__blk1024);
            let assign40420_e46184: f64 = (assign40420_e46180 + assign40420_e46183);
            (locals.var_eeff2d__blk1026, locals.var_eeff2d__blk1026_dn4, locals.var_eeff2d__blk1026_dn6, locals.var_eeff2d__blk1026_dn7, locals.var_eeff2d__blk1026_dn8, locals.var_eeff2d__blk1026_dn9, ) = (assign40420_e46184, ((locals.var_eta_mu * locals.var_esurf2d__blk1022_dn4) + (locals.var_one_m_eta * locals.var_ecpl2d__blk1024_dn4)), ((locals.var_eta_mu * locals.var_esurf2d__blk1022_dn6) + (locals.var_one_m_eta * locals.var_ecpl2d__blk1024_dn6)), ((locals.var_eta_mu * locals.var_esurf2d__blk1022_dn7) + (locals.var_one_m_eta * locals.var_ecpl2d__blk1024_dn7)), ((locals.var_eta_mu * locals.var_esurf2d__blk1022_dn8) + (locals.var_one_m_eta * locals.var_ecpl2d__blk1024_dn8)), ((locals.var_eta_mu * locals.var_esurf2d__blk1022_dn9) + (locals.var_one_m_eta * locals.var_ecpl2d__blk1024_dn9)), );
            locals.var_eeff2d__blk1026_rv = 0.0;
        }

        if (locals.var_guard1080 != 0.0) {
            let assign40430_e46191: f64 = (locals.var_esurf1s__blk952 + locals.var_esurf1d__blk1021);
            let assign40430_e46192: f64 = (0.5 * assign40430_e46191);
            (locals.var_esurf1__blk1027, locals.var_esurf1__blk1027_dn4, locals.var_esurf1__blk1027_dn6, locals.var_esurf1__blk1027_dn7, locals.var_esurf1__blk1027_dn8, locals.var_esurf1__blk1027_dn9, ) = (assign40430_e46192, (0.5 * (locals.var_esurf1s__blk952_dn4 + locals.var_esurf1d__blk1021_dn4)), (0.5 * (locals.var_esurf1s__blk952_dn6 + locals.var_esurf1d__blk1021_dn6)), (0.5 * (locals.var_esurf1s__blk952_dn7 + locals.var_esurf1d__blk1021_dn7)), (0.5 * (locals.var_esurf1s__blk952_dn8 + locals.var_esurf1d__blk1021_dn8)), (0.5 * (locals.var_esurf1s__blk952_dn9 + locals.var_esurf1d__blk1021_dn9)), );
            locals.var_esurf1__blk1027_rv = 0.0;
        }

        if (locals.var_guard1080 != 0.0) {
            let assign40440_e46199: f64 = (locals.var_esurf2s__blk953 + locals.var_esurf2d__blk1022);
            let assign40440_e46200: f64 = (0.5 * assign40440_e46199);
            (locals.var_esurf2__blk1028, locals.var_esurf2__blk1028_dn4, locals.var_esurf2__blk1028_dn6, locals.var_esurf2__blk1028_dn7, locals.var_esurf2__blk1028_dn8, locals.var_esurf2__blk1028_dn9, ) = (assign40440_e46200, (0.5 * (locals.var_esurf2s__blk953_dn4 + locals.var_esurf2d__blk1022_dn4)), (0.5 * (locals.var_esurf2s__blk953_dn6 + locals.var_esurf2d__blk1022_dn6)), (0.5 * (locals.var_esurf2s__blk953_dn7 + locals.var_esurf2d__blk1022_dn7)), (0.5 * (locals.var_esurf2s__blk953_dn8 + locals.var_esurf2d__blk1022_dn8)), (0.5 * (locals.var_esurf2s__blk953_dn9 + locals.var_esurf2d__blk1022_dn9)), );
            locals.var_esurf2__blk1028_rv = 0.0;
        }

        if (locals.var_guard1080 != 0.0) {
            let assign40450_e46207: f64 = (locals.var_esurf1__blk1027 + locals.var_esurf2__blk1028);
            let assign40450_e46208: f64 = (1.0 / assign40450_e46207);
            (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign40450_e46208, (-((locals.var_esurf1__blk1027_dn4 + locals.var_esurf2__blk1028_dn4) / (assign40450_e46207 * assign40450_e46207))), (-((locals.var_esurf1__blk1027_dn6 + locals.var_esurf2__blk1028_dn6) / (assign40450_e46207 * assign40450_e46207))), (-((locals.var_esurf1__blk1027_dn7 + locals.var_esurf2__blk1028_dn7) / (assign40450_e46207 * assign40450_e46207))), (-((locals.var_esurf1__blk1027_dn8 + locals.var_esurf2__blk1028_dn8) / (assign40450_e46207 * assign40450_e46207))), (-((locals.var_esurf1__blk1027_dn9 + locals.var_esurf2__blk1028_dn9) / (assign40450_e46207 * assign40450_e46207))), );
            locals.var_temp_rv = 0.0;
        }

        if (locals.var_guard1080 != 0.0) {
            let assign40460_e46214: f64 = (locals.var_qim__blk1016 * locals.var_esurf1__blk1027);
            let assign40460_e46216: f64 = (assign40460_e46214 * locals.var_temp);
            (locals.var_qi1m__blk1029, locals.var_qi1m__blk1029_dn4, locals.var_qi1m__blk1029_dn6, locals.var_qi1m__blk1029_dn7, locals.var_qi1m__blk1029_dn8, locals.var_qi1m__blk1029_dn9, ) = (assign40460_e46216, ((((locals.var_qim__blk1016_dn4 * locals.var_esurf1__blk1027) + (locals.var_qim__blk1016 * locals.var_esurf1__blk1027_dn4)) * locals.var_temp) + (assign40460_e46214 * locals.var_temp_dn4)), ((((locals.var_qim__blk1016_dn6 * locals.var_esurf1__blk1027) + (locals.var_qim__blk1016 * locals.var_esurf1__blk1027_dn6)) * locals.var_temp) + (assign40460_e46214 * locals.var_temp_dn6)), ((((locals.var_qim__blk1016_dn7 * locals.var_esurf1__blk1027) + (locals.var_qim__blk1016 * locals.var_esurf1__blk1027_dn7)) * locals.var_temp) + (assign40460_e46214 * locals.var_temp_dn7)), ((((locals.var_qim__blk1016_dn8 * locals.var_esurf1__blk1027) + (locals.var_qim__blk1016 * locals.var_esurf1__blk1027_dn8)) * locals.var_temp) + (assign40460_e46214 * locals.var_temp_dn8)), ((((locals.var_qim__blk1016_dn9 * locals.var_esurf1__blk1027) + (locals.var_qim__blk1016 * locals.var_esurf1__blk1027_dn9)) * locals.var_temp) + (assign40460_e46214 * locals.var_temp_dn9)), );
            locals.var_qi1m__blk1029_rv = 0.0;
        }

        if (locals.var_guard1080 != 0.0) {
            let assign40470_e46222: f64 = (locals.var_qim__blk1016 * locals.var_esurf2__blk1028);
            let assign40470_e46224: f64 = (assign40470_e46222 * locals.var_temp);
            (locals.var_qi2m__blk1030, locals.var_qi2m__blk1030_dn4, locals.var_qi2m__blk1030_dn6, locals.var_qi2m__blk1030_dn7, locals.var_qi2m__blk1030_dn8, locals.var_qi2m__blk1030_dn9, ) = (assign40470_e46224, ((((locals.var_qim__blk1016_dn4 * locals.var_esurf2__blk1028) + (locals.var_qim__blk1016 * locals.var_esurf2__blk1028_dn4)) * locals.var_temp) + (assign40470_e46222 * locals.var_temp_dn4)), ((((locals.var_qim__blk1016_dn6 * locals.var_esurf2__blk1028) + (locals.var_qim__blk1016 * locals.var_esurf2__blk1028_dn6)) * locals.var_temp) + (assign40470_e46222 * locals.var_temp_dn6)), ((((locals.var_qim__blk1016_dn7 * locals.var_esurf2__blk1028) + (locals.var_qim__blk1016 * locals.var_esurf2__blk1028_dn7)) * locals.var_temp) + (assign40470_e46222 * locals.var_temp_dn7)), ((((locals.var_qim__blk1016_dn8 * locals.var_esurf2__blk1028) + (locals.var_qim__blk1016 * locals.var_esurf2__blk1028_dn8)) * locals.var_temp) + (assign40470_e46222 * locals.var_temp_dn8)), ((((locals.var_qim__blk1016_dn9 * locals.var_esurf2__blk1028) + (locals.var_qim__blk1016 * locals.var_esurf2__blk1028_dn9)) * locals.var_temp) + (assign40470_e46222 * locals.var_temp_dn9)), );
            locals.var_qi2m__blk1030_rv = 0.0;
        }

        if (locals.var_guard1080 != 0.0) {
            let assign40480_e46231: f64 = (locals.var_ecpl1s__blk954 + locals.var_ecpl1d__blk1023);
            let assign40480_e46232: f64 = (0.5 * assign40480_e46231);
            (locals.var_ecpl1__blk1031, locals.var_ecpl1__blk1031_dn4, locals.var_ecpl1__blk1031_dn6, locals.var_ecpl1__blk1031_dn7, locals.var_ecpl1__blk1031_dn8, locals.var_ecpl1__blk1031_dn9, ) = (assign40480_e46232, (0.5 * (locals.var_ecpl1s__blk954_dn4 + locals.var_ecpl1d__blk1023_dn4)), (0.5 * (locals.var_ecpl1s__blk954_dn6 + locals.var_ecpl1d__blk1023_dn6)), (0.5 * (locals.var_ecpl1s__blk954_dn7 + locals.var_ecpl1d__blk1023_dn7)), (0.5 * (locals.var_ecpl1s__blk954_dn8 + locals.var_ecpl1d__blk1023_dn8)), (0.5 * (locals.var_ecpl1s__blk954_dn9 + locals.var_ecpl1d__blk1023_dn9)), );
            locals.var_ecpl1__blk1031_rv = 0.0;
        }

        if (locals.var_guard1080 != 0.0) {
            let assign40490_e46239: f64 = (locals.var_ecpl2s__blk955 + locals.var_ecpl2d__blk1024);
            let assign40490_e46240: f64 = (0.5 * assign40490_e46239);
            (locals.var_ecpl2__blk1032, locals.var_ecpl2__blk1032_dn4, locals.var_ecpl2__blk1032_dn6, locals.var_ecpl2__blk1032_dn7, locals.var_ecpl2__blk1032_dn8, locals.var_ecpl2__blk1032_dn9, ) = (assign40490_e46240, (0.5 * (locals.var_ecpl2s__blk955_dn4 + locals.var_ecpl2d__blk1024_dn4)), (0.5 * (locals.var_ecpl2s__blk955_dn6 + locals.var_ecpl2d__blk1024_dn6)), (0.5 * (locals.var_ecpl2s__blk955_dn7 + locals.var_ecpl2d__blk1024_dn7)), (0.5 * (locals.var_ecpl2s__blk955_dn8 + locals.var_ecpl2d__blk1024_dn8)), (0.5 * (locals.var_ecpl2s__blk955_dn9 + locals.var_ecpl2d__blk1024_dn9)), );
            locals.var_ecpl2__blk1032_rv = 0.0;
        }

        if (locals.var_guard1080 != 0.0) {
            let assign40500_e46247: f64 = (locals.var_eeff1s__blk956 + locals.var_eeff1d__blk1025);
            let assign40500_e46248: f64 = (0.5 * assign40500_e46247);
            (locals.var_eeff1__blk1033, locals.var_eeff1__blk1033_dn4, locals.var_eeff1__blk1033_dn6, locals.var_eeff1__blk1033_dn7, locals.var_eeff1__blk1033_dn8, locals.var_eeff1__blk1033_dn9, ) = (assign40500_e46248, (0.5 * (locals.var_eeff1s__blk956_dn4 + locals.var_eeff1d__blk1025_dn4)), (0.5 * (locals.var_eeff1s__blk956_dn6 + locals.var_eeff1d__blk1025_dn6)), (0.5 * (locals.var_eeff1s__blk956_dn7 + locals.var_eeff1d__blk1025_dn7)), (0.5 * (locals.var_eeff1s__blk956_dn8 + locals.var_eeff1d__blk1025_dn8)), (0.5 * (locals.var_eeff1s__blk956_dn9 + locals.var_eeff1d__blk1025_dn9)), );
            locals.var_eeff1__blk1033_rv = 0.0;
        }

        if (locals.var_guard1080 != 0.0) {
            let assign40510_e46255: f64 = (locals.var_eeff2s__blk957 + locals.var_eeff2d__blk1026);
            let assign40510_e46256: f64 = (0.5 * assign40510_e46255);
            (locals.var_eeff2__blk1034, locals.var_eeff2__blk1034_dn4, locals.var_eeff2__blk1034_dn6, locals.var_eeff2__blk1034_dn7, locals.var_eeff2__blk1034_dn8, locals.var_eeff2__blk1034_dn9, ) = (assign40510_e46256, (0.5 * (locals.var_eeff2s__blk957_dn4 + locals.var_eeff2d__blk1026_dn4)), (0.5 * (locals.var_eeff2s__blk957_dn6 + locals.var_eeff2d__blk1026_dn6)), (0.5 * (locals.var_eeff2s__blk957_dn7 + locals.var_eeff2d__blk1026_dn7)), (0.5 * (locals.var_eeff2s__blk957_dn8 + locals.var_eeff2d__blk1026_dn8)), (0.5 * (locals.var_eeff2s__blk957_dn9 + locals.var_eeff2d__blk1026_dn9)), );
            locals.var_eeff2__blk1034_rv = 0.0;
        }

        if (locals.var_guard1080 != 0.0) {
            let assign40520_e46262: f64 = (locals.var_esurf1__blk1027 * locals.var_betn1_t);
            let assign40520_e46265: f64 = (locals.var_stbet_i * locals.var_lnrtn);
            let assign40520_e46266: f64 = (assign40520_e46265).exp();
            let assign40520_e46267: f64 = (assign40520_e46262 * assign40520_e46266);
            let assign40520_e46269: f64 = (assign40520_e46267 * locals.var_ratio_pd__blk1020);
            (locals.var_c1__blk1035, locals.var_c1__blk1035_dn4, locals.var_c1__blk1035_dn6, locals.var_c1__blk1035_dn7, locals.var_c1__blk1035_dn8, locals.var_c1__blk1035_dn9, ) = (assign40520_e46269, ((((((locals.var_esurf1__blk1027_dn4 * locals.var_betn1_t) + (locals.var_esurf1__blk1027 * locals.var_betn1_t_dn4)) * assign40520_e46266) + (assign40520_e46262 * (assign40520_e46266 * (locals.var_stbet_i * locals.var_lnrtn_dn4)))) * locals.var_ratio_pd__blk1020) + (assign40520_e46267 * locals.var_ratio_pd__blk1020_dn4)), ((((((locals.var_esurf1__blk1027_dn6 * locals.var_betn1_t) + (locals.var_esurf1__blk1027 * locals.var_betn1_t_dn6)) * assign40520_e46266) + (assign40520_e46262 * (assign40520_e46266 * (locals.var_stbet_i * locals.var_lnrtn_dn6)))) * locals.var_ratio_pd__blk1020) + (assign40520_e46267 * locals.var_ratio_pd__blk1020_dn6)), ((((((locals.var_esurf1__blk1027_dn7 * locals.var_betn1_t) + (locals.var_esurf1__blk1027 * locals.var_betn1_t_dn7)) * assign40520_e46266) + (assign40520_e46262 * (assign40520_e46266 * (locals.var_stbet_i * locals.var_lnrtn_dn7)))) * locals.var_ratio_pd__blk1020) + (assign40520_e46267 * locals.var_ratio_pd__blk1020_dn7)), ((((((locals.var_esurf1__blk1027_dn8 * locals.var_betn1_t) + (locals.var_esurf1__blk1027 * locals.var_betn1_t_dn8)) * assign40520_e46266) + (assign40520_e46262 * (assign40520_e46266 * (locals.var_stbet_i * locals.var_lnrtn_dn8)))) * locals.var_ratio_pd__blk1020) + (assign40520_e46267 * locals.var_ratio_pd__blk1020_dn8)), ((((((locals.var_esurf1__blk1027_dn9 * locals.var_betn1_t) + (locals.var_esurf1__blk1027 * locals.var_betn1_t_dn9)) * assign40520_e46266) + (assign40520_e46262 * (assign40520_e46266 * (locals.var_stbet_i * locals.var_lnrtn_dn9)))) * locals.var_ratio_pd__blk1020) + (assign40520_e46267 * locals.var_ratio_pd__blk1020_dn9)), );
            locals.var_c1__blk1035_rv = 0.0;
        }

        if (locals.var_guard1080 != 0.0) {
            let assign40530_e46275: f64 = (locals.var_esurf2__blk1028 * locals.var_betn2_t);
            let assign40530_e46278: f64 = (locals.var_stbet_i * locals.var_lnrtn);
            let assign40530_e46279: f64 = (assign40530_e46278).exp();
            let assign40530_e46280: f64 = (assign40530_e46275 * assign40530_e46279);
            (locals.var_c2__blk1036, locals.var_c2__blk1036_dn4, locals.var_c2__blk1036_dn6, locals.var_c2__blk1036_dn7, locals.var_c2__blk1036_dn8, locals.var_c2__blk1036_dn9, ) = (assign40530_e46280, ((((locals.var_esurf2__blk1028_dn4 * locals.var_betn2_t) + (locals.var_esurf2__blk1028 * locals.var_betn2_t_dn4)) * assign40530_e46279) + (assign40530_e46275 * (assign40530_e46279 * (locals.var_stbet_i * locals.var_lnrtn_dn4)))), ((((locals.var_esurf2__blk1028_dn6 * locals.var_betn2_t) + (locals.var_esurf2__blk1028 * locals.var_betn2_t_dn6)) * assign40530_e46279) + (assign40530_e46275 * (assign40530_e46279 * (locals.var_stbet_i * locals.var_lnrtn_dn6)))), ((((locals.var_esurf2__blk1028_dn7 * locals.var_betn2_t) + (locals.var_esurf2__blk1028 * locals.var_betn2_t_dn7)) * assign40530_e46279) + (assign40530_e46275 * (assign40530_e46279 * (locals.var_stbet_i * locals.var_lnrtn_dn7)))), ((((locals.var_esurf2__blk1028_dn8 * locals.var_betn2_t) + (locals.var_esurf2__blk1028 * locals.var_betn2_t_dn8)) * assign40530_e46279) + (assign40530_e46275 * (assign40530_e46279 * (locals.var_stbet_i * locals.var_lnrtn_dn8)))), ((((locals.var_esurf2__blk1028_dn9 * locals.var_betn2_t) + (locals.var_esurf2__blk1028 * locals.var_betn2_t_dn9)) * assign40530_e46279) + (assign40530_e46275 * (assign40530_e46279 * (locals.var_stbet_i * locals.var_lnrtn_dn9)))), );
            locals.var_c2__blk1036_rv = 0.0;
        }

        if (locals.var_guard1080 != 0.0) {
            let assign40540_e46286: f64 = (locals.var_c1__blk1035 + locals.var_c2__blk1036);
            (locals.var_csum__blk1037, locals.var_csum__blk1037_dn4, locals.var_csum__blk1037_dn6, locals.var_csum__blk1037_dn7, locals.var_csum__blk1037_dn8, locals.var_csum__blk1037_dn9, ) = (assign40540_e46286, (locals.var_c1__blk1035_dn4 + locals.var_c2__blk1036_dn4), (locals.var_c1__blk1035_dn6 + locals.var_c2__blk1036_dn6), (locals.var_c1__blk1035_dn7 + locals.var_c2__blk1036_dn7), (locals.var_c1__blk1035_dn8 + locals.var_c2__blk1036_dn8), (locals.var_c1__blk1035_dn9 + locals.var_c2__blk1036_dn9), );
            locals.var_csum__blk1037_rv = 0.0;
        }

        if (locals.var_guard1080 != 0.0) {
            let assign40550_e46294: f64 = (locals.var_xcorb_i * locals.var_ecpl2__blk1032);
            let assign40550_e46295: f64 = (locals.var_ecpl1__blk1031 + assign40550_e46294);
            let assign40550_e46296: f64 = (locals.var_xcor_i * assign40550_e46295);
            (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9, ) = (assign40550_e46296, ((locals.var_xcor_i_dn4 * assign40550_e46295) + (locals.var_xcor_i * (locals.var_ecpl1__blk1031_dn4 + (locals.var_xcorb_i * locals.var_ecpl2__blk1032_dn4)))), ((locals.var_xcor_i_dn6 * assign40550_e46295) + (locals.var_xcor_i * (locals.var_ecpl1__blk1031_dn6 + (locals.var_xcorb_i * locals.var_ecpl2__blk1032_dn6)))), ((locals.var_xcor_i_dn7 * assign40550_e46295) + (locals.var_xcor_i * (locals.var_ecpl1__blk1031_dn7 + (locals.var_xcorb_i * locals.var_ecpl2__blk1032_dn7)))), ((locals.var_xcor_i_dn8 * assign40550_e46295) + (locals.var_xcor_i * (locals.var_ecpl1__blk1031_dn8 + (locals.var_xcorb_i * locals.var_ecpl2__blk1032_dn8)))), ((locals.var_xcor_i_dn9 * assign40550_e46295) + (locals.var_xcor_i * (locals.var_ecpl1__blk1031_dn9 + (locals.var_xcorb_i * locals.var_ecpl2__blk1032_dn9)))), );
            locals.var_temp1_rv = 0.0;
        }

        if (locals.var_guard1080 != 0.0) {
            let assign40560_e46303: f64 = (1.0 + locals.var_temp1);
            let assign40560_e46305: f64 = assign40560_e46303;
            let assign40560_e46308: f64 = (1.0 + locals.var_temp1);
            let assign40560_e46310: f64 = assign40560_e46308;
            let assign40560_e46313: f64 = (1.0 + locals.var_temp1);
            let assign40560_e46315: f64 = assign40560_e46313;
            let assign40560_e46316: f64 = (assign40560_e46310 * assign40560_e46315);
            let assign40560_e46318: f64 = (assign40560_e46316 + 0.01);
            let assign40560_e46319: f64 = (assign40560_e46318).sqrt();
            let assign40560_e46320: f64 = (assign40560_e46305 + assign40560_e46319);
            let assign40560_e46321: f64 = (0.5 * assign40560_e46320);
            (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9, ) = (assign40560_e46321, (0.5 * (locals.var_temp1_dn4 + (((locals.var_temp1_dn4 * assign40560_e46315) + (assign40560_e46310 * locals.var_temp1_dn4)) / (2.0 * assign40560_e46319)))), (0.5 * (locals.var_temp1_dn6 + (((locals.var_temp1_dn6 * assign40560_e46315) + (assign40560_e46310 * locals.var_temp1_dn6)) / (2.0 * assign40560_e46319)))), (0.5 * (locals.var_temp1_dn7 + (((locals.var_temp1_dn7 * assign40560_e46315) + (assign40560_e46310 * locals.var_temp1_dn7)) / (2.0 * assign40560_e46319)))), (0.5 * (locals.var_temp1_dn8 + (((locals.var_temp1_dn8 * assign40560_e46315) + (assign40560_e46310 * locals.var_temp1_dn8)) / (2.0 * assign40560_e46319)))), (0.5 * (locals.var_temp1_dn9 + (((locals.var_temp1_dn9 * assign40560_e46315) + (assign40560_e46310 * locals.var_temp1_dn9)) / (2.0 * assign40560_e46319)))), );
            locals.var_temp2_rv = 0.0;
        }

        if (locals.var_guard1080 != 0.0) {
            let assign40570_e46329: f64 = (0.2 * locals.var_temp1);
            let assign40570_e46330: f64 = (1.0 + assign40570_e46329);
            let assign40570_e46332: f64 = assign40570_e46330;
            let assign40570_e46336: f64 = (0.2 * locals.var_temp1);
            let assign40570_e46337: f64 = (1.0 + assign40570_e46336);
            let assign40570_e46339: f64 = assign40570_e46337;
            let assign40570_e46343: f64 = (0.2 * locals.var_temp1);
            let assign40570_e46344: f64 = (1.0 + assign40570_e46343);
            let assign40570_e46346: f64 = assign40570_e46344;
            let assign40570_e46347: f64 = (assign40570_e46339 * assign40570_e46346);
            let assign40570_e46349: f64 = (assign40570_e46347 + 0.01);
            let assign40570_e46350: f64 = (assign40570_e46349).sqrt();
            let assign40570_e46351: f64 = (assign40570_e46332 + assign40570_e46350);
            let assign40570_e46352: f64 = (0.5 * assign40570_e46351);
            (locals.var_temp3, locals.var_temp3_dn4, locals.var_temp3_dn6, locals.var_temp3_dn7, locals.var_temp3_dn8, locals.var_temp3_dn9, ) = (assign40570_e46352, (0.5 * ((0.2 * locals.var_temp1_dn4) + ((((0.2 * locals.var_temp1_dn4) * assign40570_e46346) + (assign40570_e46339 * (0.2 * locals.var_temp1_dn4))) / (2.0 * assign40570_e46350)))), (0.5 * ((0.2 * locals.var_temp1_dn6) + ((((0.2 * locals.var_temp1_dn6) * assign40570_e46346) + (assign40570_e46339 * (0.2 * locals.var_temp1_dn6))) / (2.0 * assign40570_e46350)))), (0.5 * ((0.2 * locals.var_temp1_dn7) + ((((0.2 * locals.var_temp1_dn7) * assign40570_e46346) + (assign40570_e46339 * (0.2 * locals.var_temp1_dn7))) / (2.0 * assign40570_e46350)))), (0.5 * ((0.2 * locals.var_temp1_dn8) + ((((0.2 * locals.var_temp1_dn8) * assign40570_e46346) + (assign40570_e46339 * (0.2 * locals.var_temp1_dn8))) / (2.0 * assign40570_e46350)))), (0.5 * ((0.2 * locals.var_temp1_dn9) + ((((0.2 * locals.var_temp1_dn9) * assign40570_e46346) + (assign40570_e46339 * (0.2 * locals.var_temp1_dn9))) / (2.0 * assign40570_e46350)))), );
            locals.var_temp3_rv = 0.0;
        }

        if (locals.var_guard1080 != 0.0) {
            let assign40580_e46358: f64 = (locals.var_temp2 / locals.var_temp3);
            (locals.var_fcor__blk1038, locals.var_fcor__blk1038_dn4, locals.var_fcor__blk1038_dn6, locals.var_fcor__blk1038_dn7, locals.var_fcor__blk1038_dn8, locals.var_fcor__blk1038_dn9, ) = (assign40580_e46358, (((locals.var_temp2_dn4 * locals.var_temp3) - (locals.var_temp2 * locals.var_temp3_dn4)) / (locals.var_temp3 * locals.var_temp3)), (((locals.var_temp2_dn6 * locals.var_temp3) - (locals.var_temp2 * locals.var_temp3_dn6)) / (locals.var_temp3 * locals.var_temp3)), (((locals.var_temp2_dn7 * locals.var_temp3) - (locals.var_temp2 * locals.var_temp3_dn7)) / (locals.var_temp3 * locals.var_temp3)), (((locals.var_temp2_dn8 * locals.var_temp3) - (locals.var_temp2 * locals.var_temp3_dn8)) / (locals.var_temp3 * locals.var_temp3)), (((locals.var_temp2_dn9 * locals.var_temp3) - (locals.var_temp2 * locals.var_temp3_dn9)) / (locals.var_temp3 * locals.var_temp3)), );
            locals.var_fcor__blk1038_rv = 0.0;
        }

        if (locals.var_guard1080 != 0.0) {
            let assign40590_e46366: f64 = (locals.var_csfi_i * locals.var_ecpl1__blk1031);
            let assign40590_e46367: f64 = (1.0 + assign40590_e46366);
            let assign40590_e46370: f64 = (locals.var_csbi_i * locals.var_ecpl2__blk1032);
            let assign40590_e46371: f64 = (assign40590_e46367 + assign40590_e46370);
            let assign40590_e46372: f64 = (locals.var_cs_i * assign40590_e46371);
            let assign40590_e46374: f64 = (-locals.var_thecs_i);
            let assign40590_e46378: f64 = (locals.var_qi1m__blk1029 * locals.var_inv_qi1cs);
            let assign40590_e46379: f64 = (1.0 + assign40590_e46378);
            let assign40590_e46382: f64 = (locals.var_qi2m__blk1030 * locals.var_inv_qi2cs);
            let assign40590_e46383: f64 = (assign40590_e46379 + assign40590_e46382);
            let assign40590_e46384: f64 = (assign40590_e46383).ln();
            let assign40590_e46385: f64 = (assign40590_e46374 * assign40590_e46384);
            let assign40590_e46386: f64 = (assign40590_e46385).exp();
            let assign40590_e46387: f64 = (assign40590_e46372 * assign40590_e46386);
            (locals.var_gcs__blk1039, locals.var_gcs__blk1039_dn4, locals.var_gcs__blk1039_dn6, locals.var_gcs__blk1039_dn7, locals.var_gcs__blk1039_dn8, locals.var_gcs__blk1039_dn9, ) = (assign40590_e46387, ((((locals.var_cs_i_dn4 * assign40590_e46371) + (locals.var_cs_i * ((locals.var_csfi_i * locals.var_ecpl1__blk1031_dn4) + (locals.var_csbi_i * locals.var_ecpl2__blk1032_dn4)))) * assign40590_e46386) + (assign40590_e46372 * (assign40590_e46386 * (((-locals.var_thecs_i_dn4) * assign40590_e46384) + (assign40590_e46374 * (((locals.var_qi1m__blk1029_dn4 * locals.var_inv_qi1cs) + (locals.var_qi2m__blk1030_dn4 * locals.var_inv_qi2cs)) / assign40590_e46383)))))), ((((locals.var_cs_i_dn6 * assign40590_e46371) + (locals.var_cs_i * ((locals.var_csfi_i * locals.var_ecpl1__blk1031_dn6) + (locals.var_csbi_i * locals.var_ecpl2__blk1032_dn6)))) * assign40590_e46386) + (assign40590_e46372 * (assign40590_e46386 * (((-locals.var_thecs_i_dn6) * assign40590_e46384) + (assign40590_e46374 * (((locals.var_qi1m__blk1029_dn6 * locals.var_inv_qi1cs) + (locals.var_qi2m__blk1030_dn6 * locals.var_inv_qi2cs)) / assign40590_e46383)))))), ((((locals.var_cs_i_dn7 * assign40590_e46371) + (locals.var_cs_i * ((locals.var_csfi_i * locals.var_ecpl1__blk1031_dn7) + (locals.var_csbi_i * locals.var_ecpl2__blk1032_dn7)))) * assign40590_e46386) + (assign40590_e46372 * (assign40590_e46386 * (((-locals.var_thecs_i_dn7) * assign40590_e46384) + (assign40590_e46374 * (((locals.var_qi1m__blk1029_dn7 * locals.var_inv_qi1cs) + (locals.var_qi2m__blk1030_dn7 * locals.var_inv_qi2cs)) / assign40590_e46383)))))), ((((locals.var_cs_i_dn8 * assign40590_e46371) + (locals.var_cs_i * ((locals.var_csfi_i * locals.var_ecpl1__blk1031_dn8) + (locals.var_csbi_i * locals.var_ecpl2__blk1032_dn8)))) * assign40590_e46386) + (assign40590_e46372 * (assign40590_e46386 * (((-locals.var_thecs_i_dn8) * assign40590_e46384) + (assign40590_e46374 * (((locals.var_qi1m__blk1029_dn8 * locals.var_inv_qi1cs) + (locals.var_qi2m__blk1030_dn8 * locals.var_inv_qi2cs)) / assign40590_e46383)))))), ((((locals.var_cs_i_dn9 * assign40590_e46371) + (locals.var_cs_i * ((locals.var_csfi_i * locals.var_ecpl1__blk1031_dn9) + (locals.var_csbi_i * locals.var_ecpl2__blk1032_dn9)))) * assign40590_e46386) + (assign40590_e46372 * (assign40590_e46386 * (((-locals.var_thecs_i_dn9) * assign40590_e46384) + (assign40590_e46374 * (((locals.var_qi1m__blk1029_dn9 * locals.var_inv_qi1cs) + (locals.var_qi2m__blk1030_dn9 * locals.var_inv_qi2cs)) / assign40590_e46383)))))), );
            locals.var_gcs__blk1039_rv = 0.0;
        }

        let assign40600_e46392: f64 = if locals.var_rsg_i == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1222 = assign40600_e46392;
        locals.var_guard1222_rv = 0.0;

        if ((locals.var_guard1080 != 0.0) && (locals.var_guard1222 != 0.0)) {
            (locals.var_temp3, locals.var_temp3_dn4, locals.var_temp3_dn6, locals.var_temp3_dn7, locals.var_temp3_dn8, locals.var_temp3_dn9, ) = (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_temp3_rv = 0.0;
        }

        let assign40620_e46401: f64 = if locals.var_rsg_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1223 = assign40620_e46401;
        locals.var_guard1223_rv = 0.0;

        if (((locals.var_guard1080 != 0.0) && (locals.var_guard1222 == 0.0)) && (locals.var_guard1223 != 0.0)) {
            let assign40630_e46412: f64 = (locals.var_qim__blk1016 + 1e-12);
            let assign40630_e46413: f64 = (assign40630_e46412).ln();
            let assign40630_e46414: f64 = (locals.var_thersg_i * assign40630_e46413);
            let assign40630_e46415: f64 = (assign40630_e46414).exp();
            let assign40630_e46416: f64 = (locals.var_rsg_i * assign40630_e46415);
            (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9, ) = (assign40630_e46416, (locals.var_rsg_i * (assign40630_e46415 * (locals.var_thersg_i * (locals.var_qim__blk1016_dn4 / assign40630_e46412)))), (locals.var_rsg_i * (assign40630_e46415 * (locals.var_thersg_i * (locals.var_qim__blk1016_dn6 / assign40630_e46412)))), (locals.var_rsg_i * (assign40630_e46415 * (locals.var_thersg_i * (locals.var_qim__blk1016_dn7 / assign40630_e46412)))), (locals.var_rsg_i * (assign40630_e46415 * (locals.var_thersg_i * (locals.var_qim__blk1016_dn8 / assign40630_e46412)))), (locals.var_rsg_i * (assign40630_e46415 * (locals.var_thersg_i * (locals.var_qim__blk1016_dn9 / assign40630_e46412)))), );
            locals.var_temp1_rv = 0.0;
        }

        if (((locals.var_guard1080 != 0.0) && (locals.var_guard1222 == 0.0)) && (locals.var_guard1223 != 0.0)) {
            let assign40640_e46427: f64 = (1.0 - locals.var_temp1);
            (locals.var_temp3, locals.var_temp3_dn4, locals.var_temp3_dn6, locals.var_temp3_dn7, locals.var_temp3_dn8, locals.var_temp3_dn9, ) = (assign40640_e46427, (-locals.var_temp1_dn4), (-locals.var_temp1_dn6), (-locals.var_temp1_dn7), (-locals.var_temp1_dn8), (-locals.var_temp1_dn9), );
            locals.var_temp3_rv = 0.0;
        }

        if (((locals.var_guard1080 != 0.0) && (locals.var_guard1222 == 0.0)) && (locals.var_guard1223 == 0.0)) {
            let assign40650_e46441: f64 = (locals.var_qim__blk1016 + 1e-12);
            let assign40650_e46442: f64 = (assign40650_e46441).ln();
            let assign40650_e46443: f64 = (locals.var_thersg_i * assign40650_e46442);
            let assign40650_e46444: f64 = (assign40650_e46443).exp();
            let assign40650_e46445: f64 = (locals.var_rsg_i * assign40650_e46444);
            (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9, ) = (assign40650_e46445, (locals.var_rsg_i * (assign40650_e46444 * (locals.var_thersg_i * (locals.var_qim__blk1016_dn4 / assign40650_e46441)))), (locals.var_rsg_i * (assign40650_e46444 * (locals.var_thersg_i * (locals.var_qim__blk1016_dn6 / assign40650_e46441)))), (locals.var_rsg_i * (assign40650_e46444 * (locals.var_thersg_i * (locals.var_qim__blk1016_dn7 / assign40650_e46441)))), (locals.var_rsg_i * (assign40650_e46444 * (locals.var_thersg_i * (locals.var_qim__blk1016_dn8 / assign40650_e46441)))), (locals.var_rsg_i * (assign40650_e46444 * (locals.var_thersg_i * (locals.var_qim__blk1016_dn9 / assign40650_e46441)))), );
            locals.var_temp1_rv = 0.0;
        }

        if (((locals.var_guard1080 != 0.0) && (locals.var_guard1222 == 0.0)) && (locals.var_guard1223 == 0.0)) {
            let assign40660_e46458: f64 = (1.0 + locals.var_temp1);
            let assign40660_e46459: f64 = (1.0 / assign40660_e46458);
            (locals.var_temp3, locals.var_temp3_dn4, locals.var_temp3_dn6, locals.var_temp3_dn7, locals.var_temp3_dn8, locals.var_temp3_dn9, ) = (assign40660_e46459, (-(locals.var_temp1_dn4 / (assign40660_e46458 * assign40660_e46458))), (-(locals.var_temp1_dn6 / (assign40660_e46458 * assign40660_e46458))), (-(locals.var_temp1_dn7 / (assign40660_e46458 * assign40660_e46458))), (-(locals.var_temp1_dn8 / (assign40660_e46458 * assign40660_e46458))), (-(locals.var_temp1_dn9 / (assign40660_e46458 * assign40660_e46458))), );
            locals.var_temp3_rv = 0.0;
        }

        if (locals.var_guard1080 != 0.0) {
            let assign40670_e46466: f64 = (locals.var_qim__blk1016 * locals.var_temp3);
            let assign40670_e46468: f64 = (assign40670_e46466 + locals.var_rsig_i);
            let assign40670_e46469: f64 = (locals.var_frscsi__blk964 * assign40670_e46468);
            (locals.var_grs__blk1040, locals.var_grs__blk1040_dn4, locals.var_grs__blk1040_dn6, locals.var_grs__blk1040_dn7, locals.var_grs__blk1040_dn8, locals.var_grs__blk1040_dn9, ) = (assign40670_e46469, ((locals.var_frscsi__blk964_dn4 * assign40670_e46468) + (locals.var_frscsi__blk964 * ((locals.var_qim__blk1016_dn4 * locals.var_temp3) + (locals.var_qim__blk1016 * locals.var_temp3_dn4)))), ((locals.var_frscsi__blk964_dn6 * assign40670_e46468) + (locals.var_frscsi__blk964 * ((locals.var_qim__blk1016_dn6 * locals.var_temp3) + (locals.var_qim__blk1016 * locals.var_temp3_dn6)))), ((locals.var_frscsi__blk964_dn7 * assign40670_e46468) + (locals.var_frscsi__blk964 * ((locals.var_qim__blk1016_dn7 * locals.var_temp3) + (locals.var_qim__blk1016 * locals.var_temp3_dn7)))), ((locals.var_frscsi__blk964_dn8 * assign40670_e46468) + (locals.var_frscsi__blk964 * ((locals.var_qim__blk1016_dn8 * locals.var_temp3) + (locals.var_qim__blk1016 * locals.var_temp3_dn8)))), ((locals.var_frscsi__blk964_dn9 * assign40670_e46468) + (locals.var_frscsi__blk964 * ((locals.var_qim__blk1016_dn9 * locals.var_temp3) + (locals.var_qim__blk1016 * locals.var_temp3_dn9)))), );
            locals.var_grs__blk1040_rv = 0.0;
        }

        if (locals.var_guard1080 != 0.0) {
            let assign40680_e46477: f64 = (locals.var_fmue * locals.var_eeff1__blk1033);
            let assign40680_e46479: f64 = (assign40680_e46477 + 1e-6);
            let assign40680_e46480: f64 = (assign40680_e46479).ln();
            let assign40680_e46481: f64 = (locals.var_themu_i * assign40680_e46480);
            let assign40680_e46482: f64 = (assign40680_e46481).exp();
            let assign40680_e46483: f64 = (1.0 + assign40680_e46482);
            let assign40680_e46485: f64 = (assign40680_e46483 + locals.var_gcs__blk1039);
            let assign40680_e46488: f64 = (locals.var_betn1_i * locals.var_grs__blk1040);
            let assign40680_e46489: f64 = (assign40680_e46485 + assign40680_e46488);
            (locals.var_gmob1__blk1041, locals.var_gmob1__blk1041_dn4, locals.var_gmob1__blk1041_dn6, locals.var_gmob1__blk1041_dn7, locals.var_gmob1__blk1041_dn8, locals.var_gmob1__blk1041_dn9, ) = (assign40680_e46489, (((assign40680_e46482 * ((locals.var_themu_i_dn4 * assign40680_e46480) + (locals.var_themu_i * (((locals.var_fmue_dn4 * locals.var_eeff1__blk1033) + (locals.var_fmue * locals.var_eeff1__blk1033_dn4)) / assign40680_e46479)))) + locals.var_gcs__blk1039_dn4) + ((locals.var_betn1_i_dn4 * locals.var_grs__blk1040) + (locals.var_betn1_i * locals.var_grs__blk1040_dn4))), (((assign40680_e46482 * ((locals.var_themu_i_dn6 * assign40680_e46480) + (locals.var_themu_i * (((locals.var_fmue_dn6 * locals.var_eeff1__blk1033) + (locals.var_fmue * locals.var_eeff1__blk1033_dn6)) / assign40680_e46479)))) + locals.var_gcs__blk1039_dn6) + ((locals.var_betn1_i_dn6 * locals.var_grs__blk1040) + (locals.var_betn1_i * locals.var_grs__blk1040_dn6))), (((assign40680_e46482 * ((locals.var_themu_i_dn7 * assign40680_e46480) + (locals.var_themu_i * (((locals.var_fmue_dn7 * locals.var_eeff1__blk1033) + (locals.var_fmue * locals.var_eeff1__blk1033_dn7)) / assign40680_e46479)))) + locals.var_gcs__blk1039_dn7) + ((locals.var_betn1_i_dn7 * locals.var_grs__blk1040) + (locals.var_betn1_i * locals.var_grs__blk1040_dn7))), (((assign40680_e46482 * ((locals.var_themu_i_dn8 * assign40680_e46480) + (locals.var_themu_i * (((locals.var_fmue_dn8 * locals.var_eeff1__blk1033) + (locals.var_fmue * locals.var_eeff1__blk1033_dn8)) / assign40680_e46479)))) + locals.var_gcs__blk1039_dn8) + ((locals.var_betn1_i_dn8 * locals.var_grs__blk1040) + (locals.var_betn1_i * locals.var_grs__blk1040_dn8))), (((assign40680_e46482 * ((locals.var_themu_i_dn9 * assign40680_e46480) + (locals.var_themu_i * (((locals.var_fmue_dn9 * locals.var_eeff1__blk1033) + (locals.var_fmue * locals.var_eeff1__blk1033_dn9)) / assign40680_e46479)))) + locals.var_gcs__blk1039_dn9) + ((locals.var_betn1_i_dn9 * locals.var_grs__blk1040) + (locals.var_betn1_i * locals.var_grs__blk1040_dn9))), );
            locals.var_gmob1__blk1041_rv = 0.0;
        }

        if (locals.var_guard1080 != 0.0) {
            let assign40690_e46497: f64 = (locals.var_fmue * locals.var_eeff2__blk1034);
            let assign40690_e46499: f64 = (assign40690_e46497 + 1e-6);
            let assign40690_e46500: f64 = (assign40690_e46499).ln();
            let assign40690_e46501: f64 = (locals.var_themu_i * assign40690_e46500);
            let assign40690_e46502: f64 = (assign40690_e46501).exp();
            let assign40690_e46503: f64 = (1.0 + assign40690_e46502);
            let assign40690_e46505: f64 = (assign40690_e46503 + locals.var_gcs__blk1039);
            let assign40690_e46508: f64 = (locals.var_betn2_i * locals.var_grs__blk1040);
            let assign40690_e46509: f64 = (assign40690_e46505 + assign40690_e46508);
            (locals.var_gmob2__blk1042, locals.var_gmob2__blk1042_dn4, locals.var_gmob2__blk1042_dn6, locals.var_gmob2__blk1042_dn7, locals.var_gmob2__blk1042_dn8, locals.var_gmob2__blk1042_dn9, ) = (assign40690_e46509, (((assign40690_e46502 * ((locals.var_themu_i_dn4 * assign40690_e46500) + (locals.var_themu_i * (((locals.var_fmue_dn4 * locals.var_eeff2__blk1034) + (locals.var_fmue * locals.var_eeff2__blk1034_dn4)) / assign40690_e46499)))) + locals.var_gcs__blk1039_dn4) + ((locals.var_betn2_i_dn4 * locals.var_grs__blk1040) + (locals.var_betn2_i * locals.var_grs__blk1040_dn4))), (((assign40690_e46502 * ((locals.var_themu_i_dn6 * assign40690_e46500) + (locals.var_themu_i * (((locals.var_fmue_dn6 * locals.var_eeff2__blk1034) + (locals.var_fmue * locals.var_eeff2__blk1034_dn6)) / assign40690_e46499)))) + locals.var_gcs__blk1039_dn6) + ((locals.var_betn2_i_dn6 * locals.var_grs__blk1040) + (locals.var_betn2_i * locals.var_grs__blk1040_dn6))), (((assign40690_e46502 * ((locals.var_themu_i_dn7 * assign40690_e46500) + (locals.var_themu_i * (((locals.var_fmue_dn7 * locals.var_eeff2__blk1034) + (locals.var_fmue * locals.var_eeff2__blk1034_dn7)) / assign40690_e46499)))) + locals.var_gcs__blk1039_dn7) + ((locals.var_betn2_i_dn7 * locals.var_grs__blk1040) + (locals.var_betn2_i * locals.var_grs__blk1040_dn7))), (((assign40690_e46502 * ((locals.var_themu_i_dn8 * assign40690_e46500) + (locals.var_themu_i * (((locals.var_fmue_dn8 * locals.var_eeff2__blk1034) + (locals.var_fmue * locals.var_eeff2__blk1034_dn8)) / assign40690_e46499)))) + locals.var_gcs__blk1039_dn8) + ((locals.var_betn2_i_dn8 * locals.var_grs__blk1040) + (locals.var_betn2_i * locals.var_grs__blk1040_dn8))), (((assign40690_e46502 * ((locals.var_themu_i_dn9 * assign40690_e46500) + (locals.var_themu_i * (((locals.var_fmue_dn9 * locals.var_eeff2__blk1034) + (locals.var_fmue * locals.var_eeff2__blk1034_dn9)) / assign40690_e46499)))) + locals.var_gcs__blk1039_dn9) + ((locals.var_betn2_i_dn9 * locals.var_grs__blk1040) + (locals.var_betn2_i * locals.var_grs__blk1040_dn9))), );
            locals.var_gmob2__blk1042_rv = 0.0;
        }

        if (locals.var_guard1080 != 0.0) {
            let assign40700_e46515: f64 = (locals.var_fcor__blk1038 * locals.var_csum__blk1037);
            let assign40700_e46518: f64 = (locals.var_c1__blk1035 / locals.var_gmob1__blk1041);
            let assign40700_e46521: f64 = (locals.var_c2__blk1036 / locals.var_gmob2__blk1042);
            let assign40700_e46522: f64 = (assign40700_e46518 + assign40700_e46521);
            let assign40700_e46523: f64 = (assign40700_e46515 / assign40700_e46522);
            (locals.var_gmob__blk1043, locals.var_gmob__blk1043_dn4, locals.var_gmob__blk1043_dn6, locals.var_gmob__blk1043_dn7, locals.var_gmob__blk1043_dn8, locals.var_gmob__blk1043_dn9, ) = (assign40700_e46523, (((((locals.var_fcor__blk1038_dn4 * locals.var_csum__blk1037) + (locals.var_fcor__blk1038 * locals.var_csum__blk1037_dn4)) * assign40700_e46522) - (assign40700_e46515 * ((((locals.var_c1__blk1035_dn4 * locals.var_gmob1__blk1041) - (locals.var_c1__blk1035 * locals.var_gmob1__blk1041_dn4)) / (locals.var_gmob1__blk1041 * locals.var_gmob1__blk1041)) + (((locals.var_c2__blk1036_dn4 * locals.var_gmob2__blk1042) - (locals.var_c2__blk1036 * locals.var_gmob2__blk1042_dn4)) / (locals.var_gmob2__blk1042 * locals.var_gmob2__blk1042))))) / (assign40700_e46522 * assign40700_e46522)), (((((locals.var_fcor__blk1038_dn6 * locals.var_csum__blk1037) + (locals.var_fcor__blk1038 * locals.var_csum__blk1037_dn6)) * assign40700_e46522) - (assign40700_e46515 * ((((locals.var_c1__blk1035_dn6 * locals.var_gmob1__blk1041) - (locals.var_c1__blk1035 * locals.var_gmob1__blk1041_dn6)) / (locals.var_gmob1__blk1041 * locals.var_gmob1__blk1041)) + (((locals.var_c2__blk1036_dn6 * locals.var_gmob2__blk1042) - (locals.var_c2__blk1036 * locals.var_gmob2__blk1042_dn6)) / (locals.var_gmob2__blk1042 * locals.var_gmob2__blk1042))))) / (assign40700_e46522 * assign40700_e46522)), (((((locals.var_fcor__blk1038_dn7 * locals.var_csum__blk1037) + (locals.var_fcor__blk1038 * locals.var_csum__blk1037_dn7)) * assign40700_e46522) - (assign40700_e46515 * ((((locals.var_c1__blk1035_dn7 * locals.var_gmob1__blk1041) - (locals.var_c1__blk1035 * locals.var_gmob1__blk1041_dn7)) / (locals.var_gmob1__blk1041 * locals.var_gmob1__blk1041)) + (((locals.var_c2__blk1036_dn7 * locals.var_gmob2__blk1042) - (locals.var_c2__blk1036 * locals.var_gmob2__blk1042_dn7)) / (locals.var_gmob2__blk1042 * locals.var_gmob2__blk1042))))) / (assign40700_e46522 * assign40700_e46522)), (((((locals.var_fcor__blk1038_dn8 * locals.var_csum__blk1037) + (locals.var_fcor__blk1038 * locals.var_csum__blk1037_dn8)) * assign40700_e46522) - (assign40700_e46515 * ((((locals.var_c1__blk1035_dn8 * locals.var_gmob1__blk1041) - (locals.var_c1__blk1035 * locals.var_gmob1__blk1041_dn8)) / (locals.var_gmob1__blk1041 * locals.var_gmob1__blk1041)) + (((locals.var_c2__blk1036_dn8 * locals.var_gmob2__blk1042) - (locals.var_c2__blk1036 * locals.var_gmob2__blk1042_dn8)) / (locals.var_gmob2__blk1042 * locals.var_gmob2__blk1042))))) / (assign40700_e46522 * assign40700_e46522)), (((((locals.var_fcor__blk1038_dn9 * locals.var_csum__blk1037) + (locals.var_fcor__blk1038 * locals.var_csum__blk1037_dn9)) * assign40700_e46522) - (assign40700_e46515 * ((((locals.var_c1__blk1035_dn9 * locals.var_gmob1__blk1041) - (locals.var_c1__blk1035 * locals.var_gmob1__blk1041_dn9)) / (locals.var_gmob1__blk1041 * locals.var_gmob1__blk1041)) + (((locals.var_c2__blk1036_dn9 * locals.var_gmob2__blk1042) - (locals.var_c2__blk1036 * locals.var_gmob2__blk1042_dn9)) / (locals.var_gmob2__blk1042 * locals.var_gmob2__blk1042))))) / (assign40700_e46522 * assign40700_e46522)), );
            locals.var_gmob__blk1043_rv = 0.0;
        }

        if (locals.var_guard1080 != 0.0) {
            let assign40710_e46530: f64 = (4.0 + locals.var_qim__blk1016);
            let assign40710_e46531: f64 = (1.0 / assign40710_e46530);
            (locals.var_inv_qimstar1__blk1044, locals.var_inv_qimstar1__blk1044_dn4, locals.var_inv_qimstar1__blk1044_dn6, locals.var_inv_qimstar1__blk1044_dn7, locals.var_inv_qimstar1__blk1044_dn8, locals.var_inv_qimstar1__blk1044_dn9, ) = (assign40710_e46531, (-(locals.var_qim__blk1016_dn4 / (assign40710_e46530 * assign40710_e46530))), (-(locals.var_qim__blk1016_dn6 / (assign40710_e46530 * assign40710_e46530))), (-(locals.var_qim__blk1016_dn7 / (assign40710_e46530 * assign40710_e46530))), (-(locals.var_qim__blk1016_dn8 / (assign40710_e46530 * assign40710_e46530))), (-(locals.var_qim__blk1016_dn9 / (assign40710_e46530 * assign40710_e46530))), );
            locals.var_inv_qimstar1__blk1044_rv = 0.0;
        }

        let assign40720_e46536: f64 = if locals.var_alpb_i > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1224 = assign40720_e46536;
        locals.var_guard1224_rv = 0.0;

        if ((locals.var_guard1080 != 0.0) && (locals.var_guard1224 != 0.0)) {
            let assign40730_e46544: f64 = (locals.var_alpb_i * locals.var_qi2m__blk1030);
            let assign40730_e46545: f64 = (1.0 + assign40730_e46544);
            let assign40730_e46546: f64 = (1.0 / assign40730_e46545);
            (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign40730_e46546, (-((locals.var_alpb_i * locals.var_qi2m__blk1030_dn4) / (assign40730_e46545 * assign40730_e46545))), (-((locals.var_alpb_i * locals.var_qi2m__blk1030_dn6) / (assign40730_e46545 * assign40730_e46545))), (-((locals.var_alpb_i * locals.var_qi2m__blk1030_dn7) / (assign40730_e46545 * assign40730_e46545))), (-((locals.var_alpb_i * locals.var_qi2m__blk1030_dn8) / (assign40730_e46545 * assign40730_e46545))), (-((locals.var_alpb_i * locals.var_qi2m__blk1030_dn9) / (assign40730_e46545 * assign40730_e46545))), );
            locals.var_temp_rv = 0.0;
        }

        if ((locals.var_guard1080 != 0.0) && (locals.var_guard1224 == 0.0)) {
            let assign40740_e46556: f64 = (locals.var_alpb_i * locals.var_qi2m__blk1030);
            let assign40740_e46557: f64 = (1.0 - assign40740_e46556);
            (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign40740_e46557, (-(locals.var_alpb_i * locals.var_qi2m__blk1030_dn4)), (-(locals.var_alpb_i * locals.var_qi2m__blk1030_dn6)), (-(locals.var_alpb_i * locals.var_qi2m__blk1030_dn7)), (-(locals.var_alpb_i * locals.var_qi2m__blk1030_dn8)), (-(locals.var_alpb_i * locals.var_qi2m__blk1030_dn9)), );
            locals.var_temp_rv = 0.0;
        }

        if (locals.var_guard1080 != 0.0) {
            let assign40750_e46563: f64 = (locals.var_qim__blk1016 * locals.var_inv_qimstar1__blk1044);
            let assign40750_e46565: f64 = (assign40750_e46563 * locals.var_temp);
            (locals.var_r1__blk1045, locals.var_r1__blk1045_dn4, locals.var_r1__blk1045_dn6, locals.var_r1__blk1045_dn7, locals.var_r1__blk1045_dn8, locals.var_r1__blk1045_dn9, ) = (assign40750_e46565, ((((locals.var_qim__blk1016_dn4 * locals.var_inv_qimstar1__blk1044) + (locals.var_qim__blk1016 * locals.var_inv_qimstar1__blk1044_dn4)) * locals.var_temp) + (assign40750_e46563 * locals.var_temp_dn4)), ((((locals.var_qim__blk1016_dn6 * locals.var_inv_qimstar1__blk1044) + (locals.var_qim__blk1016 * locals.var_inv_qimstar1__blk1044_dn6)) * locals.var_temp) + (assign40750_e46563 * locals.var_temp_dn6)), ((((locals.var_qim__blk1016_dn7 * locals.var_inv_qimstar1__blk1044) + (locals.var_qim__blk1016 * locals.var_inv_qimstar1__blk1044_dn7)) * locals.var_temp) + (assign40750_e46563 * locals.var_temp_dn7)), ((((locals.var_qim__blk1016_dn8 * locals.var_inv_qimstar1__blk1044) + (locals.var_qim__blk1016 * locals.var_inv_qimstar1__blk1044_dn8)) * locals.var_temp) + (assign40750_e46563 * locals.var_temp_dn8)), ((((locals.var_qim__blk1016_dn9 * locals.var_inv_qimstar1__blk1044) + (locals.var_qim__blk1016 * locals.var_inv_qimstar1__blk1044_dn9)) * locals.var_temp) + (assign40750_e46563 * locals.var_temp_dn9)), );
            locals.var_r1__blk1045_rv = 0.0;
        }

        if (locals.var_guard1080 != 0.0) {
            let assign40760_e46572: f64 = (locals.var_xd - locals.var_xdeff__blk1000);
            let assign40760_e46575: f64 = (locals.var_vp_i * locals.var_inv_phit);
            let assign40760_e46578: f64 = (locals.var_vpg_i * locals.var_qim__blk1016);
            let assign40760_e46580: f64 = (assign40760_e46578 * locals.var_qim__blk1016);
            let assign40760_e46581: f64 = (assign40760_e46575 + assign40760_e46580);
            let assign40760_e46582: f64 = (assign40760_e46572 / assign40760_e46581);
            let assign40760_e46583: f64 = (1.0 + assign40760_e46582);
            let assign40760_e46584: f64 = (assign40760_e46583).ln();
            let assign40760_e46586: f64 = (assign40760_e46584 * locals.var_r1__blk1045);
            (locals.var_dl_l_fact__blk1046, locals.var_dl_l_fact__blk1046_dn4, locals.var_dl_l_fact__blk1046_dn6, locals.var_dl_l_fact__blk1046_dn7, locals.var_dl_l_fact__blk1046_dn8, locals.var_dl_l_fact__blk1046_dn9, ) = (assign40760_e46586, (((((((locals.var_xd_dn4 - locals.var_xdeff__blk1000_dn4) * assign40760_e46581) - (assign40760_e46572 * ((locals.var_vp_i * locals.var_inv_phit_dn4) + (((locals.var_vpg_i * locals.var_qim__blk1016_dn4) * locals.var_qim__blk1016) + (assign40760_e46578 * locals.var_qim__blk1016_dn4))))) / (assign40760_e46581 * assign40760_e46581)) / assign40760_e46583) * locals.var_r1__blk1045) + (assign40760_e46584 * locals.var_r1__blk1045_dn4)), (((((((locals.var_xd_dn6 - locals.var_xdeff__blk1000_dn6) * assign40760_e46581) - (assign40760_e46572 * ((locals.var_vp_i * locals.var_inv_phit_dn6) + (((locals.var_vpg_i * locals.var_qim__blk1016_dn6) * locals.var_qim__blk1016) + (assign40760_e46578 * locals.var_qim__blk1016_dn6))))) / (assign40760_e46581 * assign40760_e46581)) / assign40760_e46583) * locals.var_r1__blk1045) + (assign40760_e46584 * locals.var_r1__blk1045_dn6)), (((((((locals.var_xd_dn7 - locals.var_xdeff__blk1000_dn7) * assign40760_e46581) - (assign40760_e46572 * ((locals.var_vp_i * locals.var_inv_phit_dn7) + (((locals.var_vpg_i * locals.var_qim__blk1016_dn7) * locals.var_qim__blk1016) + (assign40760_e46578 * locals.var_qim__blk1016_dn7))))) / (assign40760_e46581 * assign40760_e46581)) / assign40760_e46583) * locals.var_r1__blk1045) + (assign40760_e46584 * locals.var_r1__blk1045_dn7)), (((((((locals.var_xd_dn8 - locals.var_xdeff__blk1000_dn8) * assign40760_e46581) - (assign40760_e46572 * ((locals.var_vp_i * locals.var_inv_phit_dn8) + (((locals.var_vpg_i * locals.var_qim__blk1016_dn8) * locals.var_qim__blk1016) + (assign40760_e46578 * locals.var_qim__blk1016_dn8))))) / (assign40760_e46581 * assign40760_e46581)) / assign40760_e46583) * locals.var_r1__blk1045) + (assign40760_e46584 * locals.var_r1__blk1045_dn8)), (((((((locals.var_xd_dn9 - locals.var_xdeff__blk1000_dn9) * assign40760_e46581) - (assign40760_e46572 * ((locals.var_vp_i * locals.var_inv_phit_dn9) + (((locals.var_vpg_i * locals.var_qim__blk1016_dn9) * locals.var_qim__blk1016) + (assign40760_e46578 * locals.var_qim__blk1016_dn9))))) / (assign40760_e46581 * assign40760_e46581)) / assign40760_e46583) * locals.var_r1__blk1045) + (assign40760_e46584 * locals.var_r1__blk1045_dn9)), );
            locals.var_dl_l_fact__blk1046_rv = 0.0;
        }

        if (locals.var_guard1080 != 0.0) {
            let assign40770_e46592: f64 = (locals.var_alp_loc__blk898 * locals.var_dl_l_fact__blk1046);
            (locals.var_dl_l__blk1047, locals.var_dl_l__blk1047_dn4, locals.var_dl_l__blk1047_dn6, locals.var_dl_l__blk1047_dn7, locals.var_dl_l__blk1047_dn8, locals.var_dl_l__blk1047_dn9, ) = (assign40770_e46592, (locals.var_alp_loc__blk898 * locals.var_dl_l_fact__blk1046_dn4), (locals.var_alp_loc__blk898 * locals.var_dl_l_fact__blk1046_dn6), (locals.var_alp_loc__blk898 * locals.var_dl_l_fact__blk1046_dn7), (locals.var_alp_loc__blk898 * locals.var_dl_l_fact__blk1046_dn8), (locals.var_alp_loc__blk898 * locals.var_dl_l_fact__blk1046_dn9), );
            locals.var_dl_l__blk1047_rv = 0.0;
        }

        if (locals.var_guard1080 != 0.0) {
            let assign40780_e46601: f64 = (1.0 + locals.var_dl_l__blk1047);
            let assign40780_e46602: f64 = (locals.var_dl_l__blk1047 * assign40780_e46601);
            let assign40780_e46603: f64 = (1.0 + assign40780_e46602);
            let assign40780_e46604: f64 = (1.0 / assign40780_e46603);
            (locals.var_gdl__blk1048, locals.var_gdl__blk1048_dn4, locals.var_gdl__blk1048_dn6, locals.var_gdl__blk1048_dn7, locals.var_gdl__blk1048_dn8, locals.var_gdl__blk1048_dn9, ) = (assign40780_e46604, (-(((locals.var_dl_l__blk1047_dn4 * assign40780_e46601) + (locals.var_dl_l__blk1047 * locals.var_dl_l__blk1047_dn4)) / (assign40780_e46603 * assign40780_e46603))), (-(((locals.var_dl_l__blk1047_dn6 * assign40780_e46601) + (locals.var_dl_l__blk1047 * locals.var_dl_l__blk1047_dn6)) / (assign40780_e46603 * assign40780_e46603))), (-(((locals.var_dl_l__blk1047_dn7 * assign40780_e46601) + (locals.var_dl_l__blk1047 * locals.var_dl_l__blk1047_dn7)) / (assign40780_e46603 * assign40780_e46603))), (-(((locals.var_dl_l__blk1047_dn8 * assign40780_e46601) + (locals.var_dl_l__blk1047 * locals.var_dl_l__blk1047_dn8)) / (assign40780_e46603 * assign40780_e46603))), (-(((locals.var_dl_l__blk1047_dn9 * assign40780_e46601) + (locals.var_dl_l__blk1047 * locals.var_dl_l__blk1047_dn9)) / (assign40780_e46603 * assign40780_e46603))), );
            locals.var_gdl__blk1048_rv = 0.0;
        }

        if (locals.var_guard1080 != 0.0) {
            let assign40790_e46610: f64 = (100.0 * locals.var_esurf1__blk1027);
            let assign40790_e46613: f64 = (100.0 + locals.var_esurf1__blk1027);
            let assign40790_e46614: f64 = (assign40790_e46610 / assign40790_e46613);
            (locals.var_wsat1__blk976, locals.var_wsat1__blk976_dn4, locals.var_wsat1__blk976_dn6, locals.var_wsat1__blk976_dn7, locals.var_wsat1__blk976_dn8, locals.var_wsat1__blk976_dn9, ) = (assign40790_e46614, ((((100.0 * locals.var_esurf1__blk1027_dn4) * assign40790_e46613) - (assign40790_e46610 * locals.var_esurf1__blk1027_dn4)) / (assign40790_e46613 * assign40790_e46613)), ((((100.0 * locals.var_esurf1__blk1027_dn6) * assign40790_e46613) - (assign40790_e46610 * locals.var_esurf1__blk1027_dn6)) / (assign40790_e46613 * assign40790_e46613)), ((((100.0 * locals.var_esurf1__blk1027_dn7) * assign40790_e46613) - (assign40790_e46610 * locals.var_esurf1__blk1027_dn7)) / (assign40790_e46613 * assign40790_e46613)), ((((100.0 * locals.var_esurf1__blk1027_dn8) * assign40790_e46613) - (assign40790_e46610 * locals.var_esurf1__blk1027_dn8)) / (assign40790_e46613 * assign40790_e46613)), ((((100.0 * locals.var_esurf1__blk1027_dn9) * assign40790_e46613) - (assign40790_e46610 * locals.var_esurf1__blk1027_dn9)) / (assign40790_e46613 * assign40790_e46613)), );
            locals.var_wsat1__blk976_rv = 0.0;
        }

        let assign40800_e46619: f64 = if locals.var_thesat1_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1225 = assign40800_e46619;
        locals.var_guard1225_rv = 0.0;

        if ((locals.var_guard1080 != 0.0) && (locals.var_guard1225 != 0.0)) {
            let assign40810_e46627: f64 = (locals.var_thesat1_i * locals.var_wsat1__blk976);
            let assign40810_e46628: f64 = (1.0 - assign40810_e46627);
            let assign40810_e46629: f64 = (1.0 / assign40810_e46628);
            (locals.var_sat_fact1__blk977, locals.var_sat_fact1__blk977_dn4, locals.var_sat_fact1__blk977_dn6, locals.var_sat_fact1__blk977_dn7, locals.var_sat_fact1__blk977_dn8, locals.var_sat_fact1__blk977_dn9, ) = (assign40810_e46629, (-((-(locals.var_thesat1_i * locals.var_wsat1__blk976_dn4)) / (assign40810_e46628 * assign40810_e46628))), (-((-(locals.var_thesat1_i * locals.var_wsat1__blk976_dn6)) / (assign40810_e46628 * assign40810_e46628))), (-((-(locals.var_thesat1_i * locals.var_wsat1__blk976_dn7)) / (assign40810_e46628 * assign40810_e46628))), (-((-(locals.var_thesat1_i * locals.var_wsat1__blk976_dn8)) / (assign40810_e46628 * assign40810_e46628))), (-((-(locals.var_thesat1_i * locals.var_wsat1__blk976_dn9)) / (assign40810_e46628 * assign40810_e46628))), );
            locals.var_sat_fact1__blk977_rv = 0.0;
        }

        if ((locals.var_guard1080 != 0.0) && (locals.var_guard1225 == 0.0)) {
            let assign40820_e46639: f64 = (locals.var_thesat1_i * locals.var_wsat1__blk976);
            let assign40820_e46640: f64 = (1.0 + assign40820_e46639);
            (locals.var_sat_fact1__blk977, locals.var_sat_fact1__blk977_dn4, locals.var_sat_fact1__blk977_dn6, locals.var_sat_fact1__blk977_dn7, locals.var_sat_fact1__blk977_dn8, locals.var_sat_fact1__blk977_dn9, ) = (assign40820_e46640, (locals.var_thesat1_i * locals.var_wsat1__blk976_dn4), (locals.var_thesat1_i * locals.var_wsat1__blk976_dn6), (locals.var_thesat1_i * locals.var_wsat1__blk976_dn7), (locals.var_thesat1_i * locals.var_wsat1__blk976_dn8), (locals.var_thesat1_i * locals.var_wsat1__blk976_dn9), );
            locals.var_sat_fact1__blk977_rv = 0.0;
        }

        if (locals.var_guard1080 != 0.0) {
            let assign40830_e46646: f64 = (100.0 * locals.var_esurf2__blk1028);
            let assign40830_e46649: f64 = (100.0 + locals.var_esurf2__blk1028);
            let assign40830_e46650: f64 = (assign40830_e46646 / assign40830_e46649);
            (locals.var_wsat2__blk978, locals.var_wsat2__blk978_dn4, locals.var_wsat2__blk978_dn6, locals.var_wsat2__blk978_dn7, locals.var_wsat2__blk978_dn8, locals.var_wsat2__blk978_dn9, ) = (assign40830_e46650, ((((100.0 * locals.var_esurf2__blk1028_dn4) * assign40830_e46649) - (assign40830_e46646 * locals.var_esurf2__blk1028_dn4)) / (assign40830_e46649 * assign40830_e46649)), ((((100.0 * locals.var_esurf2__blk1028_dn6) * assign40830_e46649) - (assign40830_e46646 * locals.var_esurf2__blk1028_dn6)) / (assign40830_e46649 * assign40830_e46649)), ((((100.0 * locals.var_esurf2__blk1028_dn7) * assign40830_e46649) - (assign40830_e46646 * locals.var_esurf2__blk1028_dn7)) / (assign40830_e46649 * assign40830_e46649)), ((((100.0 * locals.var_esurf2__blk1028_dn8) * assign40830_e46649) - (assign40830_e46646 * locals.var_esurf2__blk1028_dn8)) / (assign40830_e46649 * assign40830_e46649)), ((((100.0 * locals.var_esurf2__blk1028_dn9) * assign40830_e46649) - (assign40830_e46646 * locals.var_esurf2__blk1028_dn9)) / (assign40830_e46649 * assign40830_e46649)), );
            locals.var_wsat2__blk978_rv = 0.0;
        }

        let assign40840_e46655: f64 = if locals.var_thesat2_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1226 = assign40840_e46655;
        locals.var_guard1226_rv = 0.0;

        if ((locals.var_guard1080 != 0.0) && (locals.var_guard1226 != 0.0)) {
            let assign40850_e46663: f64 = (locals.var_thesat2_i * locals.var_wsat2__blk978);
            let assign40850_e46664: f64 = (1.0 - assign40850_e46663);
            let assign40850_e46665: f64 = (1.0 / assign40850_e46664);
            (locals.var_sat_fact2__blk979, locals.var_sat_fact2__blk979_dn4, locals.var_sat_fact2__blk979_dn6, locals.var_sat_fact2__blk979_dn7, locals.var_sat_fact2__blk979_dn8, locals.var_sat_fact2__blk979_dn9, ) = (assign40850_e46665, (-((-(locals.var_thesat2_i * locals.var_wsat2__blk978_dn4)) / (assign40850_e46664 * assign40850_e46664))), (-((-(locals.var_thesat2_i * locals.var_wsat2__blk978_dn6)) / (assign40850_e46664 * assign40850_e46664))), (-((-(locals.var_thesat2_i * locals.var_wsat2__blk978_dn7)) / (assign40850_e46664 * assign40850_e46664))), (-((-(locals.var_thesat2_i * locals.var_wsat2__blk978_dn8)) / (assign40850_e46664 * assign40850_e46664))), (-((-(locals.var_thesat2_i * locals.var_wsat2__blk978_dn9)) / (assign40850_e46664 * assign40850_e46664))), );
            locals.var_sat_fact2__blk979_rv = 0.0;
        }

        if ((locals.var_guard1080 != 0.0) && (locals.var_guard1226 == 0.0)) {
            let assign40860_e46675: f64 = (locals.var_thesat2_i * locals.var_wsat2__blk978);
            let assign40860_e46676: f64 = (1.0 + assign40860_e46675);
            (locals.var_sat_fact2__blk979, locals.var_sat_fact2__blk979_dn4, locals.var_sat_fact2__blk979_dn6, locals.var_sat_fact2__blk979_dn7, locals.var_sat_fact2__blk979_dn8, locals.var_sat_fact2__blk979_dn9, ) = (assign40860_e46676, (locals.var_thesat2_i * locals.var_wsat2__blk978_dn4), (locals.var_thesat2_i * locals.var_wsat2__blk978_dn6), (locals.var_thesat2_i * locals.var_wsat2__blk978_dn7), (locals.var_thesat2_i * locals.var_wsat2__blk978_dn8), (locals.var_thesat2_i * locals.var_wsat2__blk978_dn9), );
            locals.var_sat_fact2__blk979_rv = 0.0;
        }

        if (locals.var_guard1080 != 0.0) {
            let assign40870_e46682: f64 = (locals.var_sat_phit_loc__blk896 * locals.var_dxdrift__blk1017);
            let assign40870_e46684: f64 = (assign40870_e46682 * 0.5);
            let assign40870_e46687: f64 = (locals.var_sat_fact1__blk977 + locals.var_sat_fact2__blk979);
            let assign40870_e46688: f64 = (assign40870_e46684 * assign40870_e46687);
            (locals.var_ggamma__blk1049, locals.var_ggamma__blk1049_dn4, locals.var_ggamma__blk1049_dn6, locals.var_ggamma__blk1049_dn7, locals.var_ggamma__blk1049_dn8, locals.var_ggamma__blk1049_dn9, ) = (assign40870_e46688, (((((locals.var_sat_phit_loc__blk896_dn4 * locals.var_dxdrift__blk1017) + (locals.var_sat_phit_loc__blk896 * locals.var_dxdrift__blk1017_dn4)) * 0.5) * assign40870_e46687) + (assign40870_e46684 * (locals.var_sat_fact1__blk977_dn4 + locals.var_sat_fact2__blk979_dn4))), (((((locals.var_sat_phit_loc__blk896_dn6 * locals.var_dxdrift__blk1017) + (locals.var_sat_phit_loc__blk896 * locals.var_dxdrift__blk1017_dn6)) * 0.5) * assign40870_e46687) + (assign40870_e46684 * (locals.var_sat_fact1__blk977_dn6 + locals.var_sat_fact2__blk979_dn6))), (((((locals.var_sat_phit_loc__blk896_dn7 * locals.var_dxdrift__blk1017) + (locals.var_sat_phit_loc__blk896 * locals.var_dxdrift__blk1017_dn7)) * 0.5) * assign40870_e46687) + (assign40870_e46684 * (locals.var_sat_fact1__blk977_dn7 + locals.var_sat_fact2__blk979_dn7))), (((((locals.var_sat_phit_loc__blk896_dn8 * locals.var_dxdrift__blk1017) + (locals.var_sat_phit_loc__blk896 * locals.var_dxdrift__blk1017_dn8)) * 0.5) * assign40870_e46687) + (assign40870_e46684 * (locals.var_sat_fact1__blk977_dn8 + locals.var_sat_fact2__blk979_dn8))), (((((locals.var_sat_phit_loc__blk896_dn9 * locals.var_dxdrift__blk1017) + (locals.var_sat_phit_loc__blk896 * locals.var_dxdrift__blk1017_dn9)) * 0.5) * assign40870_e46687) + (assign40870_e46684 * (locals.var_sat_fact1__blk977_dn9 + locals.var_sat_fact2__blk979_dn9))), );
            locals.var_ggamma__blk1049_rv = 0.0;
        }

        if (locals.var_guard1080 != 0.0) {
            let assign40880_e46695: f64 = (locals.var_gmob__blk1043 * locals.var_gdl__blk1048);
            let assign40880_e46696: f64 = (locals.var_ggamma__blk1049 / assign40880_e46695);
            (locals.var_sqrt_zsat__blk1050, locals.var_sqrt_zsat__blk1050_dn4, locals.var_sqrt_zsat__blk1050_dn6, locals.var_sqrt_zsat__blk1050_dn7, locals.var_sqrt_zsat__blk1050_dn8, locals.var_sqrt_zsat__blk1050_dn9, ) = (assign40880_e46696, (((locals.var_ggamma__blk1049_dn4 * assign40880_e46695) - (locals.var_ggamma__blk1049 * ((locals.var_gmob__blk1043_dn4 * locals.var_gdl__blk1048) + (locals.var_gmob__blk1043 * locals.var_gdl__blk1048_dn4)))) / (assign40880_e46695 * assign40880_e46695)), (((locals.var_ggamma__blk1049_dn6 * assign40880_e46695) - (locals.var_ggamma__blk1049 * ((locals.var_gmob__blk1043_dn6 * locals.var_gdl__blk1048) + (locals.var_gmob__blk1043 * locals.var_gdl__blk1048_dn6)))) / (assign40880_e46695 * assign40880_e46695)), (((locals.var_ggamma__blk1049_dn7 * assign40880_e46695) - (locals.var_ggamma__blk1049 * ((locals.var_gmob__blk1043_dn7 * locals.var_gdl__blk1048) + (locals.var_gmob__blk1043 * locals.var_gdl__blk1048_dn7)))) / (assign40880_e46695 * assign40880_e46695)), (((locals.var_ggamma__blk1049_dn8 * assign40880_e46695) - (locals.var_ggamma__blk1049 * ((locals.var_gmob__blk1043_dn8 * locals.var_gdl__blk1048) + (locals.var_gmob__blk1043 * locals.var_gdl__blk1048_dn8)))) / (assign40880_e46695 * assign40880_e46695)), (((locals.var_ggamma__blk1049_dn9 * assign40880_e46695) - (locals.var_ggamma__blk1049 * ((locals.var_gmob__blk1043_dn9 * locals.var_gdl__blk1048) + (locals.var_gmob__blk1043 * locals.var_gdl__blk1048_dn9)))) / (assign40880_e46695 * assign40880_e46695)), );
            locals.var_sqrt_zsat__blk1050_rv = 0.0;
        }

        if (locals.var_guard1080 != 0.0) {
            let assign40890_e46702: f64 = (locals.var_sqrt_zsat__blk1050 * locals.var_sqrt_zsat__blk1050);
            (locals.var_zsat__blk1051, locals.var_zsat__blk1051_dn4, locals.var_zsat__blk1051_dn6, locals.var_zsat__blk1051_dn7, locals.var_zsat__blk1051_dn8, locals.var_zsat__blk1051_dn9, ) = (assign40890_e46702, ((locals.var_sqrt_zsat__blk1050_dn4 * locals.var_sqrt_zsat__blk1050) + (locals.var_sqrt_zsat__blk1050 * locals.var_sqrt_zsat__blk1050_dn4)), ((locals.var_sqrt_zsat__blk1050_dn6 * locals.var_sqrt_zsat__blk1050) + (locals.var_sqrt_zsat__blk1050 * locals.var_sqrt_zsat__blk1050_dn6)), ((locals.var_sqrt_zsat__blk1050_dn7 * locals.var_sqrt_zsat__blk1050) + (locals.var_sqrt_zsat__blk1050 * locals.var_sqrt_zsat__blk1050_dn7)), ((locals.var_sqrt_zsat__blk1050_dn8 * locals.var_sqrt_zsat__blk1050) + (locals.var_sqrt_zsat__blk1050 * locals.var_sqrt_zsat__blk1050_dn8)), ((locals.var_sqrt_zsat__blk1050_dn9 * locals.var_sqrt_zsat__blk1050) + (locals.var_sqrt_zsat__blk1050 * locals.var_sqrt_zsat__blk1050_dn9)), );
            locals.var_zsat__blk1051_rv = 0.0;
        }

        if (locals.var_guard1080 != 0.0) {
            let assign40900_e46708: f64 = (1.0 + locals.var_zsat__blk1051);
            let assign40900_e46709: f64 = (assign40900_e46708).sqrt();
            (locals.var_vsat_fact__blk1052, locals.var_vsat_fact__blk1052_dn4, locals.var_vsat_fact__blk1052_dn6, locals.var_vsat_fact__blk1052_dn7, locals.var_vsat_fact__blk1052_dn8, locals.var_vsat_fact__blk1052_dn9, ) = (assign40900_e46709, (locals.var_zsat__blk1051_dn4 / (2.0 * assign40900_e46709)), (locals.var_zsat__blk1051_dn6 / (2.0 * assign40900_e46709)), (locals.var_zsat__blk1051_dn7 / (2.0 * assign40900_e46709)), (locals.var_zsat__blk1051_dn8 / (2.0 * assign40900_e46709)), (locals.var_zsat__blk1051_dn9 / (2.0 * assign40900_e46709)), );
            locals.var_vsat_fact__blk1052_rv = 0.0;
        }

        if (locals.var_guard1080 != 0.0) {
            let assign40910_e46716: f64 = (1.5 * locals.var_zsat__blk1051);
            let assign40910_e46717: f64 = (1.0 + assign40910_e46716);
            let assign40910_e46719: f64 = (assign40910_e46717 / locals.var_vsat_fact__blk1052);
            (locals.var_hsat__blk1053, locals.var_hsat__blk1053_dn4, locals.var_hsat__blk1053_dn6, locals.var_hsat__blk1053_dn7, locals.var_hsat__blk1053_dn8, locals.var_hsat__blk1053_dn9, ) = (assign40910_e46719, ((((1.5 * locals.var_zsat__blk1051_dn4) * locals.var_vsat_fact__blk1052) - (assign40910_e46717 * locals.var_vsat_fact__blk1052_dn4)) / (locals.var_vsat_fact__blk1052 * locals.var_vsat_fact__blk1052)), ((((1.5 * locals.var_zsat__blk1051_dn6) * locals.var_vsat_fact__blk1052) - (assign40910_e46717 * locals.var_vsat_fact__blk1052_dn6)) / (locals.var_vsat_fact__blk1052 * locals.var_vsat_fact__blk1052)), ((((1.5 * locals.var_zsat__blk1051_dn7) * locals.var_vsat_fact__blk1052) - (assign40910_e46717 * locals.var_vsat_fact__blk1052_dn7)) / (locals.var_vsat_fact__blk1052 * locals.var_vsat_fact__blk1052)), ((((1.5 * locals.var_zsat__blk1051_dn8) * locals.var_vsat_fact__blk1052) - (assign40910_e46717 * locals.var_vsat_fact__blk1052_dn8)) / (locals.var_vsat_fact__blk1052 * locals.var_vsat_fact__blk1052)), ((((1.5 * locals.var_zsat__blk1051_dn9) * locals.var_vsat_fact__blk1052) - (assign40910_e46717 * locals.var_vsat_fact__blk1052_dn9)) / (locals.var_vsat_fact__blk1052 * locals.var_vsat_fact__blk1052)), );
            locals.var_hsat__blk1053_rv = 0.0;
        }

        let assign40920_e46724: f64 = if p.p13 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1227 = assign40920_e46724;
        locals.var_guard1227_rv = 0.0;

        if ((locals.var_guard1080 != 0.0) && (locals.var_guard1227 != 0.0)) {
            let assign40930_e46730: f64 = (0.6 * locals.var_qq);
            let assign40930_e46732: f64 = (-0.1666666666667);
            let assign40930_e46735: f64 = (locals.var_esurf1__blk1027 * locals.var_esurf1__blk1027);
            let assign40930_e46737: f64 = (assign40930_e46735 + 60.0);
            let assign40930_e46738: f64 = (assign40930_e46737).ln();
            let assign40930_e46739: f64 = (assign40930_e46732 * assign40930_e46738);
            let assign40930_e46740: f64 = (assign40930_e46739).exp();
            let assign40930_e46741: f64 = (assign40930_e46730 * assign40930_e46740);
            (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9, ) = (assign40930_e46741, (((0.6 * locals.var_qq_dn4) * assign40930_e46740) + (assign40930_e46730 * (assign40930_e46740 * (assign40930_e46732 * (((locals.var_esurf1__blk1027_dn4 * locals.var_esurf1__blk1027) + (locals.var_esurf1__blk1027 * locals.var_esurf1__blk1027_dn4)) / assign40930_e46737))))), (((0.6 * locals.var_qq_dn6) * assign40930_e46740) + (assign40930_e46730 * (assign40930_e46740 * (assign40930_e46732 * (((locals.var_esurf1__blk1027_dn6 * locals.var_esurf1__blk1027) + (locals.var_esurf1__blk1027 * locals.var_esurf1__blk1027_dn6)) / assign40930_e46737))))), (((0.6 * locals.var_qq_dn7) * assign40930_e46740) + (assign40930_e46730 * (assign40930_e46740 * (assign40930_e46732 * (((locals.var_esurf1__blk1027_dn7 * locals.var_esurf1__blk1027) + (locals.var_esurf1__blk1027 * locals.var_esurf1__blk1027_dn7)) / assign40930_e46737))))), (((0.6 * locals.var_qq_dn8) * assign40930_e46740) + (assign40930_e46730 * (assign40930_e46740 * (assign40930_e46732 * (((locals.var_esurf1__blk1027_dn8 * locals.var_esurf1__blk1027) + (locals.var_esurf1__blk1027 * locals.var_esurf1__blk1027_dn8)) / assign40930_e46737))))), (((0.6 * locals.var_qq_dn9) * assign40930_e46740) + (assign40930_e46730 * (assign40930_e46740 * (assign40930_e46732 * (((locals.var_esurf1__blk1027_dn9 * locals.var_esurf1__blk1027) + (locals.var_esurf1__blk1027 * locals.var_esurf1__blk1027_dn9)) / assign40930_e46737))))), );
            locals.var_temp1_rv = 0.0;
        }

        if ((locals.var_guard1080 != 0.0) && (locals.var_guard1227 != 0.0)) {
            let assign40940_e46749: f64 = (0.6 * locals.var_qq);
            let assign40940_e46751: f64 = (-0.1666666666667);
            let assign40940_e46754: f64 = (locals.var_esurf2__blk1028 * locals.var_esurf2__blk1028);
            let assign40940_e46756: f64 = (assign40940_e46754 + 60.0);
            let assign40940_e46757: f64 = (assign40940_e46756).ln();
            let assign40940_e46758: f64 = (assign40940_e46751 * assign40940_e46757);
            let assign40940_e46759: f64 = (assign40940_e46758).exp();
            let assign40940_e46760: f64 = (assign40940_e46749 * assign40940_e46759);
            (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9, ) = (assign40940_e46760, (((0.6 * locals.var_qq_dn4) * assign40940_e46759) + (assign40940_e46749 * (assign40940_e46759 * (assign40940_e46751 * (((locals.var_esurf2__blk1028_dn4 * locals.var_esurf2__blk1028) + (locals.var_esurf2__blk1028 * locals.var_esurf2__blk1028_dn4)) / assign40940_e46756))))), (((0.6 * locals.var_qq_dn6) * assign40940_e46759) + (assign40940_e46749 * (assign40940_e46759 * (assign40940_e46751 * (((locals.var_esurf2__blk1028_dn6 * locals.var_esurf2__blk1028) + (locals.var_esurf2__blk1028 * locals.var_esurf2__blk1028_dn6)) / assign40940_e46756))))), (((0.6 * locals.var_qq_dn7) * assign40940_e46759) + (assign40940_e46749 * (assign40940_e46759 * (assign40940_e46751 * (((locals.var_esurf2__blk1028_dn7 * locals.var_esurf2__blk1028) + (locals.var_esurf2__blk1028 * locals.var_esurf2__blk1028_dn7)) / assign40940_e46756))))), (((0.6 * locals.var_qq_dn8) * assign40940_e46759) + (assign40940_e46749 * (assign40940_e46759 * (assign40940_e46751 * (((locals.var_esurf2__blk1028_dn8 * locals.var_esurf2__blk1028) + (locals.var_esurf2__blk1028 * locals.var_esurf2__blk1028_dn8)) / assign40940_e46756))))), (((0.6 * locals.var_qq_dn9) * assign40940_e46759) + (assign40940_e46749 * (assign40940_e46759 * (assign40940_e46751 * (((locals.var_esurf2__blk1028_dn9 * locals.var_esurf2__blk1028) + (locals.var_esurf2__blk1028 * locals.var_esurf2__blk1028_dn9)) / assign40940_e46756))))), );
            locals.var_temp2_rv = 0.0;
        }

    }

    pub(super) fn stamp_reactive_block_59(
        locals: &mut StampLocals,
    ) {
        if ((locals.var_guard1080 != 0.0) && (locals.var_guard1227 != 0.0)) {
            let assign40950_e46769: f64 = (locals.var_k1__blk932 * locals.var_temp1);
            let assign40950_e46770: f64 = (1.0 + assign40950_e46769);
            let assign40950_e46772: f64 = (assign40950_e46770 / locals.var_tox1fact__blk913);
            (locals.var_qmfact1__blk1054, locals.var_qmfact1__blk1054_dn4, locals.var_qmfact1__blk1054_dn6, locals.var_qmfact1__blk1054_dn7, locals.var_qmfact1__blk1054_dn8, locals.var_qmfact1__blk1054_dn9, ) = (assign40950_e46772, (((((locals.var_k1__blk932_dn4 * locals.var_temp1) + (locals.var_k1__blk932 * locals.var_temp1_dn4)) * locals.var_tox1fact__blk913) - (assign40950_e46770 * locals.var_tox1fact__blk913_dn4)) / (locals.var_tox1fact__blk913 * locals.var_tox1fact__blk913)), (((((locals.var_k1__blk932_dn6 * locals.var_temp1) + (locals.var_k1__blk932 * locals.var_temp1_dn6)) * locals.var_tox1fact__blk913) - (assign40950_e46770 * locals.var_tox1fact__blk913_dn6)) / (locals.var_tox1fact__blk913 * locals.var_tox1fact__blk913)), (((((locals.var_k1__blk932_dn7 * locals.var_temp1) + (locals.var_k1__blk932 * locals.var_temp1_dn7)) * locals.var_tox1fact__blk913) - (assign40950_e46770 * locals.var_tox1fact__blk913_dn7)) / (locals.var_tox1fact__blk913 * locals.var_tox1fact__blk913)), (((((locals.var_k1__blk932_dn8 * locals.var_temp1) + (locals.var_k1__blk932 * locals.var_temp1_dn8)) * locals.var_tox1fact__blk913) - (assign40950_e46770 * locals.var_tox1fact__blk913_dn8)) / (locals.var_tox1fact__blk913 * locals.var_tox1fact__blk913)), (((((locals.var_k1__blk932_dn9 * locals.var_temp1) + (locals.var_k1__blk932 * locals.var_temp1_dn9)) * locals.var_tox1fact__blk913) - (assign40950_e46770 * locals.var_tox1fact__blk913_dn9)) / (locals.var_tox1fact__blk913 * locals.var_tox1fact__blk913)), );
            locals.var_qmfact1__blk1054_rv = 0.0;
        }

        if ((locals.var_guard1080 != 0.0) && (locals.var_guard1227 != 0.0)) {
            let assign40960_e46781: f64 = (locals.var_k2__blk933 * locals.var_temp2);
            let assign40960_e46782: f64 = (1.0 + assign40960_e46781);
            let assign40960_e46784: f64 = (assign40960_e46782 / locals.var_tox2fact__blk914);
            (locals.var_qmfact2__blk1055, locals.var_qmfact2__blk1055_dn4, locals.var_qmfact2__blk1055_dn6, locals.var_qmfact2__blk1055_dn7, locals.var_qmfact2__blk1055_dn8, locals.var_qmfact2__blk1055_dn9, ) = (assign40960_e46784, (((((locals.var_k2__blk933_dn4 * locals.var_temp2) + (locals.var_k2__blk933 * locals.var_temp2_dn4)) * locals.var_tox2fact__blk914) - (assign40960_e46782 * locals.var_tox2fact__blk914_dn4)) / (locals.var_tox2fact__blk914 * locals.var_tox2fact__blk914)), (((((locals.var_k2__blk933_dn6 * locals.var_temp2) + (locals.var_k2__blk933 * locals.var_temp2_dn6)) * locals.var_tox2fact__blk914) - (assign40960_e46782 * locals.var_tox2fact__blk914_dn6)) / (locals.var_tox2fact__blk914 * locals.var_tox2fact__blk914)), (((((locals.var_k2__blk933_dn7 * locals.var_temp2) + (locals.var_k2__blk933 * locals.var_temp2_dn7)) * locals.var_tox2fact__blk914) - (assign40960_e46782 * locals.var_tox2fact__blk914_dn7)) / (locals.var_tox2fact__blk914 * locals.var_tox2fact__blk914)), (((((locals.var_k2__blk933_dn8 * locals.var_temp2) + (locals.var_k2__blk933 * locals.var_temp2_dn8)) * locals.var_tox2fact__blk914) - (assign40960_e46782 * locals.var_tox2fact__blk914_dn8)) / (locals.var_tox2fact__blk914 * locals.var_tox2fact__blk914)), (((((locals.var_k2__blk933_dn9 * locals.var_temp2) + (locals.var_k2__blk933 * locals.var_temp2_dn9)) * locals.var_tox2fact__blk914) - (assign40960_e46782 * locals.var_tox2fact__blk914_dn9)) / (locals.var_tox2fact__blk914 * locals.var_tox2fact__blk914)), );
            locals.var_qmfact2__blk1055_rv = 0.0;
        }

        if ((locals.var_guard1080 != 0.0) && (locals.var_guard1227 == 0.0)) {
            (locals.var_qmfact1__blk1054, locals.var_qmfact1__blk1054_dn4, locals.var_qmfact1__blk1054_dn6, locals.var_qmfact1__blk1054_dn7, locals.var_qmfact1__blk1054_dn8, locals.var_qmfact1__blk1054_dn9, ) = (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_qmfact1__blk1054_rv = 0.0;
            (locals.var_qmfact2__blk1055, locals.var_qmfact2__blk1055_dn4, locals.var_qmfact2__blk1055_dn6, locals.var_qmfact2__blk1055_dn7, locals.var_qmfact2__blk1055_dn8, locals.var_qmfact2__blk1055_dn9, ) = (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_qmfact2__blk1055_rv = 0.0;
        }

        let assign40990_e46803: f64 = if locals.var_qis__blk938 > 1e-6 { 1.0 } else { 0.0 };
        locals.var_guard1228 = assign40990_e46803;
        locals.var_guard1228_rv = 0.0;

        let assign41000_e46806: f64 = if locals.var_qid__blk1003 > 1e-6 { 1.0 } else { 0.0 };
        locals.var_guard1229 = assign41000_e46806;
        locals.var_guard1229_rv = 0.0;

        let assign41010_e46808: f64 = (locals.var_a2d__blk1012).abs();
        let assign41010_e46810: f64 = if assign41010_e46808 < 0.01 { 1.0 } else { 0.0 };
        locals.var_guard1230 = assign41010_e46810;
        locals.var_guard1230_rv = 0.0;

        if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1228 != 0.0)) && (locals.var_guard1229 != 0.0)) && (locals.var_guard1230 != 0.0)) {
            let assign41020_e46820: f64 = (2.0 + locals.var_q1d__blk1001);
            let assign41020_e46823: f64 = (0.5 * locals.var_a1d__blk1011);
            let assign41020_e46824: f64 = (assign41020_e46820 + assign41020_e46823);
            let assign41020_e46827: f64 = (2.0 + locals.var_q2d__blk1002);
            let assign41020_e46829: f64 = (assign41020_e46827 * locals.var_a1d__blk1011);
            let assign41020_e46830: f64 = (assign41020_e46824 / assign41020_e46829);
            (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign41020_e46830, ((((locals.var_q1d__blk1001_dn4 + (0.5 * locals.var_a1d__blk1011_dn4)) * assign41020_e46829) - (assign41020_e46824 * ((locals.var_q2d__blk1002_dn4 * locals.var_a1d__blk1011) + (assign41020_e46827 * locals.var_a1d__blk1011_dn4)))) / (assign41020_e46829 * assign41020_e46829)), ((((locals.var_q1d__blk1001_dn6 + (0.5 * locals.var_a1d__blk1011_dn6)) * assign41020_e46829) - (assign41020_e46824 * ((locals.var_q2d__blk1002_dn6 * locals.var_a1d__blk1011) + (assign41020_e46827 * locals.var_a1d__blk1011_dn6)))) / (assign41020_e46829 * assign41020_e46829)), ((((locals.var_q1d__blk1001_dn7 + (0.5 * locals.var_a1d__blk1011_dn7)) * assign41020_e46829) - (assign41020_e46824 * ((locals.var_q2d__blk1002_dn7 * locals.var_a1d__blk1011) + (assign41020_e46827 * locals.var_a1d__blk1011_dn7)))) / (assign41020_e46829 * assign41020_e46829)), ((((locals.var_q1d__blk1001_dn8 + (0.5 * locals.var_a1d__blk1011_dn8)) * assign41020_e46829) - (assign41020_e46824 * ((locals.var_q2d__blk1002_dn8 * locals.var_a1d__blk1011) + (assign41020_e46827 * locals.var_a1d__blk1011_dn8)))) / (assign41020_e46829 * assign41020_e46829)), ((((locals.var_q1d__blk1001_dn9 + (0.5 * locals.var_a1d__blk1011_dn9)) * assign41020_e46829) - (assign41020_e46824 * ((locals.var_q2d__blk1002_dn9 * locals.var_a1d__blk1011) + (assign41020_e46827 * locals.var_a1d__blk1011_dn9)))) / (assign41020_e46829 * assign41020_e46829)), );
            locals.var_temp_rv = 0.0;
        }

        if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1228 != 0.0)) && (locals.var_guard1229 != 0.0)) && (locals.var_guard1230 != 0.0)) {
            let assign41030_e46842: f64 = (locals.var_temp * locals.var_a2d__blk1012);
            (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9, ) = (assign41030_e46842, ((locals.var_temp_dn4 * locals.var_a2d__blk1012) + (locals.var_temp * locals.var_a2d__blk1012_dn4)), ((locals.var_temp_dn6 * locals.var_a2d__blk1012) + (locals.var_temp * locals.var_a2d__blk1012_dn6)), ((locals.var_temp_dn7 * locals.var_a2d__blk1012) + (locals.var_temp * locals.var_a2d__blk1012_dn7)), ((locals.var_temp_dn8 * locals.var_a2d__blk1012) + (locals.var_temp * locals.var_a2d__blk1012_dn8)), ((locals.var_temp_dn9 * locals.var_a2d__blk1012) + (locals.var_temp * locals.var_a2d__blk1012_dn9)), );
            locals.var_temp1_rv = 0.0;
        }

        if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1228 != 0.0)) && (locals.var_guard1229 != 0.0)) && (locals.var_guard1230 != 0.0)) {
            let assign41040_e46854: f64 = (locals.var_temp1 * locals.var_temp1);
            (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9, ) = (assign41040_e46854, ((locals.var_temp1_dn4 * locals.var_temp1) + (locals.var_temp1 * locals.var_temp1_dn4)), ((locals.var_temp1_dn6 * locals.var_temp1) + (locals.var_temp1 * locals.var_temp1_dn6)), ((locals.var_temp1_dn7 * locals.var_temp1) + (locals.var_temp1 * locals.var_temp1_dn7)), ((locals.var_temp1_dn8 * locals.var_temp1) + (locals.var_temp1 * locals.var_temp1_dn8)), ((locals.var_temp1_dn9 * locals.var_temp1) + (locals.var_temp1 * locals.var_temp1_dn9)), );
            locals.var_temp2_rv = 0.0;
        }

        if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1228 != 0.0)) && (locals.var_guard1229 != 0.0)) && (locals.var_guard1230 != 0.0)) {
            let assign41050_e46866: f64 = (1.0 - locals.var_temp1);
            let assign41050_e46868: f64 = (assign41050_e46866 + locals.var_temp2);
            (locals.var_temp3, locals.var_temp3_dn4, locals.var_temp3_dn6, locals.var_temp3_dn7, locals.var_temp3_dn8, locals.var_temp3_dn9, ) = (assign41050_e46868, ((-locals.var_temp1_dn4) + locals.var_temp2_dn4), ((-locals.var_temp1_dn6) + locals.var_temp2_dn6), ((-locals.var_temp1_dn7) + locals.var_temp2_dn7), ((-locals.var_temp1_dn8) + locals.var_temp2_dn8), ((-locals.var_temp1_dn9) + locals.var_temp2_dn9), );
            locals.var_temp3_rv = 0.0;
        }

        if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1228 != 0.0)) && (locals.var_guard1229 != 0.0)) && (locals.var_guard1230 != 0.0)) {
            let assign41060_e46881: f64 = (locals.var_temp1 * locals.var_temp2);
            let assign41060_e46882: f64 = (locals.var_temp3 - assign41060_e46881);
            (locals.var_temp4, locals.var_temp4_dn4, locals.var_temp4_dn6, locals.var_temp4_dn7, locals.var_temp4_dn8, locals.var_temp4_dn9, ) = (assign41060_e46882, (locals.var_temp3_dn4 - ((locals.var_temp1_dn4 * locals.var_temp2) + (locals.var_temp1 * locals.var_temp2_dn4))), (locals.var_temp3_dn6 - ((locals.var_temp1_dn6 * locals.var_temp2) + (locals.var_temp1 * locals.var_temp2_dn6))), (locals.var_temp3_dn7 - ((locals.var_temp1_dn7 * locals.var_temp2) + (locals.var_temp1 * locals.var_temp2_dn7))), (locals.var_temp3_dn8 - ((locals.var_temp1_dn8 * locals.var_temp2) + (locals.var_temp1 * locals.var_temp2_dn8))), (locals.var_temp3_dn9 - ((locals.var_temp1_dn9 * locals.var_temp2) + (locals.var_temp1 * locals.var_temp2_dn9))), );
            locals.var_temp4_rv = 0.0;
        }

        if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1228 != 0.0)) && (locals.var_guard1229 != 0.0)) && (locals.var_guard1230 != 0.0)) {
            let assign41070_e46895: f64 = (2.0 * locals.var_qsqd__blk1006);
            let assign41070_e46899: f64 = (1.0 / locals.var_a1d__blk1011);
            let assign41070_e46900: f64 = (locals.var_temp - assign41070_e46899);
            let assign41070_e46901: f64 = (assign41070_e46895 * assign41070_e46900);
            let assign41070_e46903: f64 = (assign41070_e46901 * locals.var_temp4);
            let assign41070_e46904: f64 = (locals.var_k2q2d__blk1005 - assign41070_e46903);
            let assign41070_e46907: f64 = (2.0 + locals.var_q2d__blk1002);
            let assign41070_e46908: f64 = (assign41070_e46904 / assign41070_e46907);
            (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9, ) = (assign41070_e46908, ((((locals.var_k2q2d__blk1005_dn4 - (((((2.0 * locals.var_qsqd__blk1006_dn4) * assign41070_e46900) + (assign41070_e46895 * (locals.var_temp_dn4 - (-(locals.var_a1d__blk1011_dn4 / (locals.var_a1d__blk1011 * locals.var_a1d__blk1011)))))) * locals.var_temp4) + (assign41070_e46901 * locals.var_temp4_dn4))) * assign41070_e46907) - (assign41070_e46904 * locals.var_q2d__blk1002_dn4)) / (assign41070_e46907 * assign41070_e46907)), ((((locals.var_k2q2d__blk1005_dn6 - (((((2.0 * locals.var_qsqd__blk1006_dn6) * assign41070_e46900) + (assign41070_e46895 * (locals.var_temp_dn6 - (-(locals.var_a1d__blk1011_dn6 / (locals.var_a1d__blk1011 * locals.var_a1d__blk1011)))))) * locals.var_temp4) + (assign41070_e46901 * locals.var_temp4_dn6))) * assign41070_e46907) - (assign41070_e46904 * locals.var_q2d__blk1002_dn6)) / (assign41070_e46907 * assign41070_e46907)), ((((locals.var_k2q2d__blk1005_dn7 - (((((2.0 * locals.var_qsqd__blk1006_dn7) * assign41070_e46900) + (assign41070_e46895 * (locals.var_temp_dn7 - (-(locals.var_a1d__blk1011_dn7 / (locals.var_a1d__blk1011 * locals.var_a1d__blk1011)))))) * locals.var_temp4) + (assign41070_e46901 * locals.var_temp4_dn7))) * assign41070_e46907) - (assign41070_e46904 * locals.var_q2d__blk1002_dn7)) / (assign41070_e46907 * assign41070_e46907)), ((((locals.var_k2q2d__blk1005_dn8 - (((((2.0 * locals.var_qsqd__blk1006_dn8) * assign41070_e46900) + (assign41070_e46895 * (locals.var_temp_dn8 - (-(locals.var_a1d__blk1011_dn8 / (locals.var_a1d__blk1011 * locals.var_a1d__blk1011)))))) * locals.var_temp4) + (assign41070_e46901 * locals.var_temp4_dn8))) * assign41070_e46907) - (assign41070_e46904 * locals.var_q2d__blk1002_dn8)) / (assign41070_e46907 * assign41070_e46907)), ((((locals.var_k2q2d__blk1005_dn9 - (((((2.0 * locals.var_qsqd__blk1006_dn9) * assign41070_e46900) + (assign41070_e46895 * (locals.var_temp_dn9 - (-(locals.var_a1d__blk1011_dn9 / (locals.var_a1d__blk1011 * locals.var_a1d__blk1011)))))) * locals.var_temp4) + (assign41070_e46901 * locals.var_temp4_dn9))) * assign41070_e46907) - (assign41070_e46904 * locals.var_q2d__blk1002_dn9)) / (assign41070_e46907 * assign41070_e46907)), );
            locals.var_temp1_rv = 0.0;
        }

        if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1228 != 0.0)) && (locals.var_guard1229 != 0.0)) && (locals.var_guard1230 != 0.0)) {
            let assign41080_e46920: f64 = (locals.var_dqsqd_dxn_qi__blk1014 * locals.var_qid__blk1003);
            let assign41080_e46922: f64 = (assign41080_e46920 - locals.var_aexp1d__blk1007);
            let assign41080_e46924: f64 = (assign41080_e46922 / locals.var_a1d__blk1011);
            let assign41080_e46926: f64 = (assign41080_e46924 - locals.var_temp1);
            let assign41080_e46928: f64 = (assign41080_e46926 / locals.var_qid__blk1003);
            (locals.var_dqid_dxn_qi__blk1056, locals.var_dqid_dxn_qi__blk1056_dn4, locals.var_dqid_dxn_qi__blk1056_dn6, locals.var_dqid_dxn_qi__blk1056_dn7, locals.var_dqid_dxn_qi__blk1056_dn8, locals.var_dqid_dxn_qi__blk1056_dn9, ) = (assign41080_e46928, ((((((((((locals.var_dqsqd_dxn_qi__blk1014_dn4 * locals.var_qid__blk1003) + (locals.var_dqsqd_dxn_qi__blk1014 * locals.var_qid__blk1003_dn4)) - locals.var_aexp1d__blk1007_dn4) * locals.var_a1d__blk1011) - (assign41080_e46922 * locals.var_a1d__blk1011_dn4)) / (locals.var_a1d__blk1011 * locals.var_a1d__blk1011)) - locals.var_temp1_dn4) * locals.var_qid__blk1003) - (assign41080_e46926 * locals.var_qid__blk1003_dn4)) / (locals.var_qid__blk1003 * locals.var_qid__blk1003)), ((((((((((locals.var_dqsqd_dxn_qi__blk1014_dn6 * locals.var_qid__blk1003) + (locals.var_dqsqd_dxn_qi__blk1014 * locals.var_qid__blk1003_dn6)) - locals.var_aexp1d__blk1007_dn6) * locals.var_a1d__blk1011) - (assign41080_e46922 * locals.var_a1d__blk1011_dn6)) / (locals.var_a1d__blk1011 * locals.var_a1d__blk1011)) - locals.var_temp1_dn6) * locals.var_qid__blk1003) - (assign41080_e46926 * locals.var_qid__blk1003_dn6)) / (locals.var_qid__blk1003 * locals.var_qid__blk1003)), ((((((((((locals.var_dqsqd_dxn_qi__blk1014_dn7 * locals.var_qid__blk1003) + (locals.var_dqsqd_dxn_qi__blk1014 * locals.var_qid__blk1003_dn7)) - locals.var_aexp1d__blk1007_dn7) * locals.var_a1d__blk1011) - (assign41080_e46922 * locals.var_a1d__blk1011_dn7)) / (locals.var_a1d__blk1011 * locals.var_a1d__blk1011)) - locals.var_temp1_dn7) * locals.var_qid__blk1003) - (assign41080_e46926 * locals.var_qid__blk1003_dn7)) / (locals.var_qid__blk1003 * locals.var_qid__blk1003)), ((((((((((locals.var_dqsqd_dxn_qi__blk1014_dn8 * locals.var_qid__blk1003) + (locals.var_dqsqd_dxn_qi__blk1014 * locals.var_qid__blk1003_dn8)) - locals.var_aexp1d__blk1007_dn8) * locals.var_a1d__blk1011) - (assign41080_e46922 * locals.var_a1d__blk1011_dn8)) / (locals.var_a1d__blk1011 * locals.var_a1d__blk1011)) - locals.var_temp1_dn8) * locals.var_qid__blk1003) - (assign41080_e46926 * locals.var_qid__blk1003_dn8)) / (locals.var_qid__blk1003 * locals.var_qid__blk1003)), ((((((((((locals.var_dqsqd_dxn_qi__blk1014_dn9 * locals.var_qid__blk1003) + (locals.var_dqsqd_dxn_qi__blk1014 * locals.var_qid__blk1003_dn9)) - locals.var_aexp1d__blk1007_dn9) * locals.var_a1d__blk1011) - (assign41080_e46922 * locals.var_a1d__blk1011_dn9)) / (locals.var_a1d__blk1011 * locals.var_a1d__blk1011)) - locals.var_temp1_dn9) * locals.var_qid__blk1003) - (assign41080_e46926 * locals.var_qid__blk1003_dn9)) / (locals.var_qid__blk1003 * locals.var_qid__blk1003)), );
            locals.var_dqid_dxn_qi__blk1056_rv = 0.0;
        }

        if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1228 != 0.0)) && (locals.var_guard1229 != 0.0)) && (locals.var_guard1230 != 0.0)) {
            let assign41090_e46940: f64 = (locals.var_dqid_dxn_qi__blk1056 * locals.var_qid__blk1003);
            let assign41090_e46943: f64 = (locals.var_dqid_dxn_qi__blk1056 + 1.0);
            let assign41090_e46944: f64 = (assign41090_e46940 / assign41090_e46943);
            (locals.var_dd__blk1057, locals.var_dd__blk1057_dn4, locals.var_dd__blk1057_dn6, locals.var_dd__blk1057_dn7, locals.var_dd__blk1057_dn8, locals.var_dd__blk1057_dn9, ) = (assign41090_e46944, (((((locals.var_dqid_dxn_qi__blk1056_dn4 * locals.var_qid__blk1003) + (locals.var_dqid_dxn_qi__blk1056 * locals.var_qid__blk1003_dn4)) * assign41090_e46943) - (assign41090_e46940 * locals.var_dqid_dxn_qi__blk1056_dn4)) / (assign41090_e46943 * assign41090_e46943)), (((((locals.var_dqid_dxn_qi__blk1056_dn6 * locals.var_qid__blk1003) + (locals.var_dqid_dxn_qi__blk1056 * locals.var_qid__blk1003_dn6)) * assign41090_e46943) - (assign41090_e46940 * locals.var_dqid_dxn_qi__blk1056_dn6)) / (assign41090_e46943 * assign41090_e46943)), (((((locals.var_dqid_dxn_qi__blk1056_dn7 * locals.var_qid__blk1003) + (locals.var_dqid_dxn_qi__blk1056 * locals.var_qid__blk1003_dn7)) * assign41090_e46943) - (assign41090_e46940 * locals.var_dqid_dxn_qi__blk1056_dn7)) / (assign41090_e46943 * assign41090_e46943)), (((((locals.var_dqid_dxn_qi__blk1056_dn8 * locals.var_qid__blk1003) + (locals.var_dqid_dxn_qi__blk1056 * locals.var_qid__blk1003_dn8)) * assign41090_e46943) - (assign41090_e46940 * locals.var_dqid_dxn_qi__blk1056_dn8)) / (assign41090_e46943 * assign41090_e46943)), (((((locals.var_dqid_dxn_qi__blk1056_dn9 * locals.var_qid__blk1003) + (locals.var_dqid_dxn_qi__blk1056 * locals.var_qid__blk1003_dn9)) * assign41090_e46943) - (assign41090_e46940 * locals.var_dqid_dxn_qi__blk1056_dn9)) / (assign41090_e46943 * assign41090_e46943)), );
            locals.var_dd__blk1057_rv = 0.0;
        }

        if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1228 != 0.0)) && (locals.var_guard1229 != 0.0)) && (locals.var_guard1230 == 0.0)) {
            let assign41100_e46957: f64 = (locals.var_dqsqd_dxn_qi__blk1014 * locals.var_sumd__blk1013);
            let assign41100_e46960: f64 = (locals.var_a1d__blk1011 * locals.var_a2d__blk1012);
            let assign41100_e46961: f64 = (assign41100_e46957 / assign41100_e46960);
            let assign41100_e46964: f64 = (locals.var_aexp1d__blk1007 / locals.var_a1d__blk1011);
            let assign41100_e46967: f64 = (locals.var_aexp2d__blk1008 / locals.var_a2d__blk1012);
            let assign41100_e46968: f64 = (assign41100_e46964 + assign41100_e46967);
            let assign41100_e46970: f64 = (assign41100_e46968 / locals.var_qid__blk1003);
            let assign41100_e46971: f64 = (assign41100_e46961 - assign41100_e46970);
            (locals.var_dqid_dxn_qi__blk1056, locals.var_dqid_dxn_qi__blk1056_dn4, locals.var_dqid_dxn_qi__blk1056_dn6, locals.var_dqid_dxn_qi__blk1056_dn7, locals.var_dqid_dxn_qi__blk1056_dn8, locals.var_dqid_dxn_qi__blk1056_dn9, ) = (assign41100_e46971, ((((((locals.var_dqsqd_dxn_qi__blk1014_dn4 * locals.var_sumd__blk1013) + (locals.var_dqsqd_dxn_qi__blk1014 * locals.var_sumd__blk1013_dn4)) * assign41100_e46960) - (assign41100_e46957 * ((locals.var_a1d__blk1011_dn4 * locals.var_a2d__blk1012) + (locals.var_a1d__blk1011 * locals.var_a2d__blk1012_dn4)))) / (assign41100_e46960 * assign41100_e46960)) - (((((((locals.var_aexp1d__blk1007_dn4 * locals.var_a1d__blk1011) - (locals.var_aexp1d__blk1007 * locals.var_a1d__blk1011_dn4)) / (locals.var_a1d__blk1011 * locals.var_a1d__blk1011)) + (((locals.var_aexp2d__blk1008_dn4 * locals.var_a2d__blk1012) - (locals.var_aexp2d__blk1008 * locals.var_a2d__blk1012_dn4)) / (locals.var_a2d__blk1012 * locals.var_a2d__blk1012))) * locals.var_qid__blk1003) - (assign41100_e46968 * locals.var_qid__blk1003_dn4)) / (locals.var_qid__blk1003 * locals.var_qid__blk1003))), ((((((locals.var_dqsqd_dxn_qi__blk1014_dn6 * locals.var_sumd__blk1013) + (locals.var_dqsqd_dxn_qi__blk1014 * locals.var_sumd__blk1013_dn6)) * assign41100_e46960) - (assign41100_e46957 * ((locals.var_a1d__blk1011_dn6 * locals.var_a2d__blk1012) + (locals.var_a1d__blk1011 * locals.var_a2d__blk1012_dn6)))) / (assign41100_e46960 * assign41100_e46960)) - (((((((locals.var_aexp1d__blk1007_dn6 * locals.var_a1d__blk1011) - (locals.var_aexp1d__blk1007 * locals.var_a1d__blk1011_dn6)) / (locals.var_a1d__blk1011 * locals.var_a1d__blk1011)) + (((locals.var_aexp2d__blk1008_dn6 * locals.var_a2d__blk1012) - (locals.var_aexp2d__blk1008 * locals.var_a2d__blk1012_dn6)) / (locals.var_a2d__blk1012 * locals.var_a2d__blk1012))) * locals.var_qid__blk1003) - (assign41100_e46968 * locals.var_qid__blk1003_dn6)) / (locals.var_qid__blk1003 * locals.var_qid__blk1003))), ((((((locals.var_dqsqd_dxn_qi__blk1014_dn7 * locals.var_sumd__blk1013) + (locals.var_dqsqd_dxn_qi__blk1014 * locals.var_sumd__blk1013_dn7)) * assign41100_e46960) - (assign41100_e46957 * ((locals.var_a1d__blk1011_dn7 * locals.var_a2d__blk1012) + (locals.var_a1d__blk1011 * locals.var_a2d__blk1012_dn7)))) / (assign41100_e46960 * assign41100_e46960)) - (((((((locals.var_aexp1d__blk1007_dn7 * locals.var_a1d__blk1011) - (locals.var_aexp1d__blk1007 * locals.var_a1d__blk1011_dn7)) / (locals.var_a1d__blk1011 * locals.var_a1d__blk1011)) + (((locals.var_aexp2d__blk1008_dn7 * locals.var_a2d__blk1012) - (locals.var_aexp2d__blk1008 * locals.var_a2d__blk1012_dn7)) / (locals.var_a2d__blk1012 * locals.var_a2d__blk1012))) * locals.var_qid__blk1003) - (assign41100_e46968 * locals.var_qid__blk1003_dn7)) / (locals.var_qid__blk1003 * locals.var_qid__blk1003))), ((((((locals.var_dqsqd_dxn_qi__blk1014_dn8 * locals.var_sumd__blk1013) + (locals.var_dqsqd_dxn_qi__blk1014 * locals.var_sumd__blk1013_dn8)) * assign41100_e46960) - (assign41100_e46957 * ((locals.var_a1d__blk1011_dn8 * locals.var_a2d__blk1012) + (locals.var_a1d__blk1011 * locals.var_a2d__blk1012_dn8)))) / (assign41100_e46960 * assign41100_e46960)) - (((((((locals.var_aexp1d__blk1007_dn8 * locals.var_a1d__blk1011) - (locals.var_aexp1d__blk1007 * locals.var_a1d__blk1011_dn8)) / (locals.var_a1d__blk1011 * locals.var_a1d__blk1011)) + (((locals.var_aexp2d__blk1008_dn8 * locals.var_a2d__blk1012) - (locals.var_aexp2d__blk1008 * locals.var_a2d__blk1012_dn8)) / (locals.var_a2d__blk1012 * locals.var_a2d__blk1012))) * locals.var_qid__blk1003) - (assign41100_e46968 * locals.var_qid__blk1003_dn8)) / (locals.var_qid__blk1003 * locals.var_qid__blk1003))), ((((((locals.var_dqsqd_dxn_qi__blk1014_dn9 * locals.var_sumd__blk1013) + (locals.var_dqsqd_dxn_qi__blk1014 * locals.var_sumd__blk1013_dn9)) * assign41100_e46960) - (assign41100_e46957 * ((locals.var_a1d__blk1011_dn9 * locals.var_a2d__blk1012) + (locals.var_a1d__blk1011 * locals.var_a2d__blk1012_dn9)))) / (assign41100_e46960 * assign41100_e46960)) - (((((((locals.var_aexp1d__blk1007_dn9 * locals.var_a1d__blk1011) - (locals.var_aexp1d__blk1007 * locals.var_a1d__blk1011_dn9)) / (locals.var_a1d__blk1011 * locals.var_a1d__blk1011)) + (((locals.var_aexp2d__blk1008_dn9 * locals.var_a2d__blk1012) - (locals.var_aexp2d__blk1008 * locals.var_a2d__blk1012_dn9)) / (locals.var_a2d__blk1012 * locals.var_a2d__blk1012))) * locals.var_qid__blk1003) - (assign41100_e46968 * locals.var_qid__blk1003_dn9)) / (locals.var_qid__blk1003 * locals.var_qid__blk1003))), );
            locals.var_dqid_dxn_qi__blk1056_rv = 0.0;
        }

        if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1228 != 0.0)) && (locals.var_guard1229 != 0.0)) && (locals.var_guard1230 == 0.0)) {
            let assign41110_e46984: f64 = (locals.var_dqid_dxn_qi__blk1056 * locals.var_qid__blk1003);
            let assign41110_e46987: f64 = (locals.var_dqid_dxn_qi__blk1056 + 1.0);
            let assign41110_e46988: f64 = (assign41110_e46984 / assign41110_e46987);
            (locals.var_dd__blk1057, locals.var_dd__blk1057_dn4, locals.var_dd__blk1057_dn6, locals.var_dd__blk1057_dn7, locals.var_dd__blk1057_dn8, locals.var_dd__blk1057_dn9, ) = (assign41110_e46988, (((((locals.var_dqid_dxn_qi__blk1056_dn4 * locals.var_qid__blk1003) + (locals.var_dqid_dxn_qi__blk1056 * locals.var_qid__blk1003_dn4)) * assign41110_e46987) - (assign41110_e46984 * locals.var_dqid_dxn_qi__blk1056_dn4)) / (assign41110_e46987 * assign41110_e46987)), (((((locals.var_dqid_dxn_qi__blk1056_dn6 * locals.var_qid__blk1003) + (locals.var_dqid_dxn_qi__blk1056 * locals.var_qid__blk1003_dn6)) * assign41110_e46987) - (assign41110_e46984 * locals.var_dqid_dxn_qi__blk1056_dn6)) / (assign41110_e46987 * assign41110_e46987)), (((((locals.var_dqid_dxn_qi__blk1056_dn7 * locals.var_qid__blk1003) + (locals.var_dqid_dxn_qi__blk1056 * locals.var_qid__blk1003_dn7)) * assign41110_e46987) - (assign41110_e46984 * locals.var_dqid_dxn_qi__blk1056_dn7)) / (assign41110_e46987 * assign41110_e46987)), (((((locals.var_dqid_dxn_qi__blk1056_dn8 * locals.var_qid__blk1003) + (locals.var_dqid_dxn_qi__blk1056 * locals.var_qid__blk1003_dn8)) * assign41110_e46987) - (assign41110_e46984 * locals.var_dqid_dxn_qi__blk1056_dn8)) / (assign41110_e46987 * assign41110_e46987)), (((((locals.var_dqid_dxn_qi__blk1056_dn9 * locals.var_qid__blk1003) + (locals.var_dqid_dxn_qi__blk1056 * locals.var_qid__blk1003_dn9)) * assign41110_e46987) - (assign41110_e46984 * locals.var_dqid_dxn_qi__blk1056_dn9)) / (assign41110_e46987 * assign41110_e46987)), );
            locals.var_dd__blk1057_rv = 0.0;
        }

        if (((locals.var_guard1080 != 0.0) && (locals.var_guard1228 != 0.0)) && (locals.var_guard1229 == 0.0)) {
            (locals.var_dd__blk1057, locals.var_dd__blk1057_dn4, locals.var_dd__blk1057_dn6, locals.var_dd__blk1057_dn7, locals.var_dd__blk1057_dn8, locals.var_dd__blk1057_dn9, ) = (locals.var_dinf__blk974, locals.var_dinf__blk974_dn4, locals.var_dinf__blk974_dn6, locals.var_dinf__blk974_dn7, locals.var_dinf__blk974_dn8, locals.var_dinf__blk974_dn9, );
            locals.var_dd__blk1057_rv = 0.0;
        }

        if ((locals.var_guard1080 != 0.0) && (locals.var_guard1228 != 0.0)) {
            let assign41130_e47005: f64 = (locals.var_dd__blk1057 - locals.var_ds__blk981);
            (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9, ) = (assign41130_e47005, (locals.var_dd__blk1057_dn4 - locals.var_ds__blk981_dn4), (locals.var_dd__blk1057_dn6 - locals.var_ds__blk981_dn6), (locals.var_dd__blk1057_dn7 - locals.var_ds__blk981_dn7), (locals.var_dd__blk1057_dn8 - locals.var_ds__blk981_dn8), (locals.var_dd__blk1057_dn9 - locals.var_ds__blk981_dn9), );
            locals.var_temp1_rv = 0.0;
        }

        if ((locals.var_guard1080 != 0.0) && (locals.var_guard1228 != 0.0)) {
            let assign41140_e47014: f64 = (36.0 * locals.var_temp1);
            let assign41140_e47016: f64 = (assign41140_e47014 * locals.var_temp1);
            let assign41140_e47017: f64 = (1.0 + assign41140_e47016);
            (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9, ) = (assign41140_e47017, (((36.0 * locals.var_temp1_dn4) * locals.var_temp1) + (assign41140_e47014 * locals.var_temp1_dn4)), (((36.0 * locals.var_temp1_dn6) * locals.var_temp1) + (assign41140_e47014 * locals.var_temp1_dn6)), (((36.0 * locals.var_temp1_dn7) * locals.var_temp1) + (assign41140_e47014 * locals.var_temp1_dn7)), (((36.0 * locals.var_temp1_dn8) * locals.var_temp1) + (assign41140_e47014 * locals.var_temp1_dn8)), (((36.0 * locals.var_temp1_dn9) * locals.var_temp1) + (assign41140_e47014 * locals.var_temp1_dn9)), );
            locals.var_temp2_rv = 0.0;
        }

        let assign41150_e47021: f64 = (locals.var_temp1).abs();
        let assign41150_e47023: f64 = if assign41150_e47021 > 0.001 { 1.0 } else { 0.0 };
        locals.var_guard1231 = assign41150_e47023;
        locals.var_guard1231_rv = 0.0;

        if (((locals.var_guard1080 != 0.0) && (locals.var_guard1228 != 0.0)) && (locals.var_guard1231 != 0.0)) {
            let assign41160_e47031: f64 = (locals.var_qid__blk1003 - locals.var_qis__blk938);
            (locals.var_temp3, locals.var_temp3_dn4, locals.var_temp3_dn6, locals.var_temp3_dn7, locals.var_temp3_dn8, locals.var_temp3_dn9, ) = (assign41160_e47031, (locals.var_qid__blk1003_dn4 - locals.var_qis__blk938_dn4), (locals.var_qid__blk1003_dn6 - locals.var_qis__blk938_dn6), (locals.var_qid__blk1003_dn7 - locals.var_qis__blk938_dn7), (locals.var_qid__blk1003_dn8 - locals.var_qis__blk938_dn8), (locals.var_qid__blk1003_dn9 - locals.var_qis__blk938_dn9), );
            locals.var_temp3_rv = 0.0;
        }

        if (((locals.var_guard1080 != 0.0) && (locals.var_guard1228 != 0.0)) && (locals.var_guard1231 != 0.0)) {
            let assign41170_e47042: f64 = (locals.var_dd__blk1057 * locals.var_dxdrift__blk1017);
            let assign41170_e47043: f64 = (locals.var_temp3 - assign41170_e47042);
            (locals.var_ls__blk1058, locals.var_ls__blk1058_dn4, locals.var_ls__blk1058_dn6, locals.var_ls__blk1058_dn7, locals.var_ls__blk1058_dn8, locals.var_ls__blk1058_dn9, ) = (assign41170_e47043, (locals.var_temp3_dn4 - ((locals.var_dd__blk1057_dn4 * locals.var_dxdrift__blk1017) + (locals.var_dd__blk1057 * locals.var_dxdrift__blk1017_dn4))), (locals.var_temp3_dn6 - ((locals.var_dd__blk1057_dn6 * locals.var_dxdrift__blk1017) + (locals.var_dd__blk1057 * locals.var_dxdrift__blk1017_dn6))), (locals.var_temp3_dn7 - ((locals.var_dd__blk1057_dn7 * locals.var_dxdrift__blk1017) + (locals.var_dd__blk1057 * locals.var_dxdrift__blk1017_dn7))), (locals.var_temp3_dn8 - ((locals.var_dd__blk1057_dn8 * locals.var_dxdrift__blk1017) + (locals.var_dd__blk1057 * locals.var_dxdrift__blk1017_dn8))), (locals.var_temp3_dn9 - ((locals.var_dd__blk1057_dn9 * locals.var_dxdrift__blk1017) + (locals.var_dd__blk1057 * locals.var_dxdrift__blk1017_dn9))), );
            locals.var_ls__blk1058_rv = 0.0;
        }

        if (((locals.var_guard1080 != 0.0) && (locals.var_guard1228 != 0.0)) && (locals.var_guard1231 != 0.0)) {
            let assign41180_e47054: f64 = (locals.var_ds__blk981 * locals.var_dxdrift__blk1017);
            let assign41180_e47055: f64 = (locals.var_temp3 - assign41180_e47054);
            (locals.var_ld__blk1059, locals.var_ld__blk1059_dn4, locals.var_ld__blk1059_dn6, locals.var_ld__blk1059_dn7, locals.var_ld__blk1059_dn8, locals.var_ld__blk1059_dn9, ) = (assign41180_e47055, (locals.var_temp3_dn4 - ((locals.var_ds__blk981_dn4 * locals.var_dxdrift__blk1017) + (locals.var_ds__blk981 * locals.var_dxdrift__blk1017_dn4))), (locals.var_temp3_dn6 - ((locals.var_ds__blk981_dn6 * locals.var_dxdrift__blk1017) + (locals.var_ds__blk981 * locals.var_dxdrift__blk1017_dn6))), (locals.var_temp3_dn7 - ((locals.var_ds__blk981_dn7 * locals.var_dxdrift__blk1017) + (locals.var_ds__blk981 * locals.var_dxdrift__blk1017_dn7))), (locals.var_temp3_dn8 - ((locals.var_ds__blk981_dn8 * locals.var_dxdrift__blk1017) + (locals.var_ds__blk981 * locals.var_dxdrift__blk1017_dn8))), (locals.var_temp3_dn9 - ((locals.var_ds__blk981_dn9 * locals.var_dxdrift__blk1017) + (locals.var_ds__blk981 * locals.var_dxdrift__blk1017_dn9))), );
            locals.var_ld__blk1059_rv = 0.0;
        }

        if (((locals.var_guard1080 != 0.0) && (locals.var_guard1228 != 0.0)) && (locals.var_guard1231 != 0.0)) {
            let assign41190_e47065: f64 = (locals.var_ls__blk1058 * locals.var_ls__blk1058);
            let assign41190_e47067: f64 = (assign41190_e47065 + locals.var_temp2);
            let assign41190_e47068: f64 = (assign41190_e47067).sqrt();
            (locals.var_us__blk1060, locals.var_us__blk1060_dn4, locals.var_us__blk1060_dn6, locals.var_us__blk1060_dn7, locals.var_us__blk1060_dn8, locals.var_us__blk1060_dn9, ) = (assign41190_e47068, ((((locals.var_ls__blk1058_dn4 * locals.var_ls__blk1058) + (locals.var_ls__blk1058 * locals.var_ls__blk1058_dn4)) + locals.var_temp2_dn4) / (2.0 * assign41190_e47068)), ((((locals.var_ls__blk1058_dn6 * locals.var_ls__blk1058) + (locals.var_ls__blk1058 * locals.var_ls__blk1058_dn6)) + locals.var_temp2_dn6) / (2.0 * assign41190_e47068)), ((((locals.var_ls__blk1058_dn7 * locals.var_ls__blk1058) + (locals.var_ls__blk1058 * locals.var_ls__blk1058_dn7)) + locals.var_temp2_dn7) / (2.0 * assign41190_e47068)), ((((locals.var_ls__blk1058_dn8 * locals.var_ls__blk1058) + (locals.var_ls__blk1058 * locals.var_ls__blk1058_dn8)) + locals.var_temp2_dn8) / (2.0 * assign41190_e47068)), ((((locals.var_ls__blk1058_dn9 * locals.var_ls__blk1058) + (locals.var_ls__blk1058 * locals.var_ls__blk1058_dn9)) + locals.var_temp2_dn9) / (2.0 * assign41190_e47068)), );
            locals.var_us__blk1060_rv = 0.0;
        }

        if (((locals.var_guard1080 != 0.0) && (locals.var_guard1228 != 0.0)) && (locals.var_guard1231 != 0.0)) {
            let assign41200_e47078: f64 = (locals.var_ld__blk1059 * locals.var_ld__blk1059);
            let assign41200_e47080: f64 = (assign41200_e47078 + locals.var_temp2);
            let assign41200_e47081: f64 = (assign41200_e47080).sqrt();
            (locals.var_ud__blk1061, locals.var_ud__blk1061_dn4, locals.var_ud__blk1061_dn6, locals.var_ud__blk1061_dn7, locals.var_ud__blk1061_dn8, locals.var_ud__blk1061_dn9, ) = (assign41200_e47081, ((((locals.var_ld__blk1059_dn4 * locals.var_ld__blk1059) + (locals.var_ld__blk1059 * locals.var_ld__blk1059_dn4)) + locals.var_temp2_dn4) / (2.0 * assign41200_e47081)), ((((locals.var_ld__blk1059_dn6 * locals.var_ld__blk1059) + (locals.var_ld__blk1059 * locals.var_ld__blk1059_dn6)) + locals.var_temp2_dn6) / (2.0 * assign41200_e47081)), ((((locals.var_ld__blk1059_dn7 * locals.var_ld__blk1059) + (locals.var_ld__blk1059 * locals.var_ld__blk1059_dn7)) + locals.var_temp2_dn7) / (2.0 * assign41200_e47081)), ((((locals.var_ld__blk1059_dn8 * locals.var_ld__blk1059) + (locals.var_ld__blk1059 * locals.var_ld__blk1059_dn8)) + locals.var_temp2_dn8) / (2.0 * assign41200_e47081)), ((((locals.var_ld__blk1059_dn9 * locals.var_ld__blk1059) + (locals.var_ld__blk1059 * locals.var_ld__blk1059_dn9)) + locals.var_temp2_dn9) / (2.0 * assign41200_e47081)), );
            locals.var_ud__blk1061_rv = 0.0;
        }

        if (((locals.var_guard1080 != 0.0) && (locals.var_guard1228 != 0.0)) && (locals.var_guard1231 != 0.0)) {
            let assign41210_e47091: f64 = (0.25 / locals.var_temp1);
            let assign41210_e47094: f64 = (locals.var_ud__blk1061 * locals.var_ls__blk1058);
            let assign41210_e47097: f64 = (locals.var_us__blk1060 * locals.var_ld__blk1059);
            let assign41210_e47098: f64 = (assign41210_e47094 - assign41210_e47097);
            let assign41210_e47102: f64 = (locals.var_ld__blk1059 + locals.var_ud__blk1061);
            let assign41210_e47105: f64 = (locals.var_ls__blk1058 + locals.var_us__blk1060);
            let assign41210_e47106: f64 = (assign41210_e47102 / assign41210_e47105);
            let assign41210_e47107: f64 = (assign41210_e47106).ln();
            let assign41210_e47108: f64 = (locals.var_temp2 * assign41210_e47107);
            let assign41210_e47109: f64 = (assign41210_e47098 + assign41210_e47108);
            let assign41210_e47110: f64 = (assign41210_e47091 * assign41210_e47109);
            (locals.var_idrift2__blk1062, locals.var_idrift2__blk1062_dn4, locals.var_idrift2__blk1062_dn6, locals.var_idrift2__blk1062_dn7, locals.var_idrift2__blk1062_dn8, locals.var_idrift2__blk1062_dn9, ) = (assign41210_e47110, (((-((0.25 * locals.var_temp1_dn4) / (locals.var_temp1 * locals.var_temp1))) * assign41210_e47109) + (assign41210_e47091 * ((((locals.var_ud__blk1061_dn4 * locals.var_ls__blk1058) + (locals.var_ud__blk1061 * locals.var_ls__blk1058_dn4)) - ((locals.var_us__blk1060_dn4 * locals.var_ld__blk1059) + (locals.var_us__blk1060 * locals.var_ld__blk1059_dn4))) + ((locals.var_temp2_dn4 * assign41210_e47107) + (locals.var_temp2 * (((((locals.var_ld__blk1059_dn4 + locals.var_ud__blk1061_dn4) * assign41210_e47105) - (assign41210_e47102 * (locals.var_ls__blk1058_dn4 + locals.var_us__blk1060_dn4))) / (assign41210_e47105 * assign41210_e47105)) / assign41210_e47106)))))), (((-((0.25 * locals.var_temp1_dn6) / (locals.var_temp1 * locals.var_temp1))) * assign41210_e47109) + (assign41210_e47091 * ((((locals.var_ud__blk1061_dn6 * locals.var_ls__blk1058) + (locals.var_ud__blk1061 * locals.var_ls__blk1058_dn6)) - ((locals.var_us__blk1060_dn6 * locals.var_ld__blk1059) + (locals.var_us__blk1060 * locals.var_ld__blk1059_dn6))) + ((locals.var_temp2_dn6 * assign41210_e47107) + (locals.var_temp2 * (((((locals.var_ld__blk1059_dn6 + locals.var_ud__blk1061_dn6) * assign41210_e47105) - (assign41210_e47102 * (locals.var_ls__blk1058_dn6 + locals.var_us__blk1060_dn6))) / (assign41210_e47105 * assign41210_e47105)) / assign41210_e47106)))))), (((-((0.25 * locals.var_temp1_dn7) / (locals.var_temp1 * locals.var_temp1))) * assign41210_e47109) + (assign41210_e47091 * ((((locals.var_ud__blk1061_dn7 * locals.var_ls__blk1058) + (locals.var_ud__blk1061 * locals.var_ls__blk1058_dn7)) - ((locals.var_us__blk1060_dn7 * locals.var_ld__blk1059) + (locals.var_us__blk1060 * locals.var_ld__blk1059_dn7))) + ((locals.var_temp2_dn7 * assign41210_e47107) + (locals.var_temp2 * (((((locals.var_ld__blk1059_dn7 + locals.var_ud__blk1061_dn7) * assign41210_e47105) - (assign41210_e47102 * (locals.var_ls__blk1058_dn7 + locals.var_us__blk1060_dn7))) / (assign41210_e47105 * assign41210_e47105)) / assign41210_e47106)))))), (((-((0.25 * locals.var_temp1_dn8) / (locals.var_temp1 * locals.var_temp1))) * assign41210_e47109) + (assign41210_e47091 * ((((locals.var_ud__blk1061_dn8 * locals.var_ls__blk1058) + (locals.var_ud__blk1061 * locals.var_ls__blk1058_dn8)) - ((locals.var_us__blk1060_dn8 * locals.var_ld__blk1059) + (locals.var_us__blk1060 * locals.var_ld__blk1059_dn8))) + ((locals.var_temp2_dn8 * assign41210_e47107) + (locals.var_temp2 * (((((locals.var_ld__blk1059_dn8 + locals.var_ud__blk1061_dn8) * assign41210_e47105) - (assign41210_e47102 * (locals.var_ls__blk1058_dn8 + locals.var_us__blk1060_dn8))) / (assign41210_e47105 * assign41210_e47105)) / assign41210_e47106)))))), (((-((0.25 * locals.var_temp1_dn9) / (locals.var_temp1 * locals.var_temp1))) * assign41210_e47109) + (assign41210_e47091 * ((((locals.var_ud__blk1061_dn9 * locals.var_ls__blk1058) + (locals.var_ud__blk1061 * locals.var_ls__blk1058_dn9)) - ((locals.var_us__blk1060_dn9 * locals.var_ld__blk1059) + (locals.var_us__blk1060 * locals.var_ld__blk1059_dn9))) + ((locals.var_temp2_dn9 * assign41210_e47107) + (locals.var_temp2 * (((((locals.var_ld__blk1059_dn9 + locals.var_ud__blk1061_dn9) * assign41210_e47105) - (assign41210_e47102 * (locals.var_ls__blk1058_dn9 + locals.var_us__blk1060_dn9))) / (assign41210_e47105 * assign41210_e47105)) / assign41210_e47106)))))), );
            locals.var_idrift2__blk1062_rv = 0.0;
        }

        if (((locals.var_guard1080 != 0.0) && (locals.var_guard1228 != 0.0)) && (locals.var_guard1231 == 0.0)) {
            let assign41220_e47121: f64 = (locals.var_dxdrift__blk1017 * locals.var_temp1);
            (locals.var_temp3, locals.var_temp3_dn4, locals.var_temp3_dn6, locals.var_temp3_dn7, locals.var_temp3_dn8, locals.var_temp3_dn9, ) = (assign41220_e47121, ((locals.var_dxdrift__blk1017_dn4 * locals.var_temp1) + (locals.var_dxdrift__blk1017 * locals.var_temp1_dn4)), ((locals.var_dxdrift__blk1017_dn6 * locals.var_temp1) + (locals.var_dxdrift__blk1017 * locals.var_temp1_dn6)), ((locals.var_dxdrift__blk1017_dn7 * locals.var_temp1) + (locals.var_dxdrift__blk1017 * locals.var_temp1_dn7)), ((locals.var_dxdrift__blk1017_dn8 * locals.var_temp1) + (locals.var_dxdrift__blk1017 * locals.var_temp1_dn8)), ((locals.var_dxdrift__blk1017_dn9 * locals.var_temp1) + (locals.var_dxdrift__blk1017 * locals.var_temp1_dn9)), );
            locals.var_temp3_rv = 0.0;
        }

        if (((locals.var_guard1080 != 0.0) && (locals.var_guard1228 != 0.0)) && (locals.var_guard1231 == 0.0)) {
            let assign41230_e47131: f64 = (-0.25);
            let assign41230_e47133: f64 = (assign41230_e47131 * 0.1666666666667);
            let assign41230_e47135: f64 = (assign41230_e47133 * locals.var_dxdrift__blk1017);
            let assign41230_e47137: f64 = (assign41230_e47135 * locals.var_temp3);
            let assign41230_e47139: f64 = (assign41230_e47137 * locals.var_temp3);
            let assign41230_e47141: f64 = (locals.var_temp2).sqrt();
            let assign41230_e47142: f64 = (assign41230_e47139 / assign41230_e47141);
            (locals.var_idrift2__blk1062, locals.var_idrift2__blk1062_dn4, locals.var_idrift2__blk1062_dn6, locals.var_idrift2__blk1062_dn7, locals.var_idrift2__blk1062_dn8, locals.var_idrift2__blk1062_dn9, ) = (assign41230_e47142, ((((((((assign41230_e47133 * locals.var_dxdrift__blk1017_dn4) * locals.var_temp3) + (assign41230_e47135 * locals.var_temp3_dn4)) * locals.var_temp3) + (assign41230_e47137 * locals.var_temp3_dn4)) * assign41230_e47141) - (assign41230_e47139 * (locals.var_temp2_dn4 / (2.0 * assign41230_e47141)))) / (assign41230_e47141 * assign41230_e47141)), ((((((((assign41230_e47133 * locals.var_dxdrift__blk1017_dn6) * locals.var_temp3) + (assign41230_e47135 * locals.var_temp3_dn6)) * locals.var_temp3) + (assign41230_e47137 * locals.var_temp3_dn6)) * assign41230_e47141) - (assign41230_e47139 * (locals.var_temp2_dn6 / (2.0 * assign41230_e47141)))) / (assign41230_e47141 * assign41230_e47141)), ((((((((assign41230_e47133 * locals.var_dxdrift__blk1017_dn7) * locals.var_temp3) + (assign41230_e47135 * locals.var_temp3_dn7)) * locals.var_temp3) + (assign41230_e47137 * locals.var_temp3_dn7)) * assign41230_e47141) - (assign41230_e47139 * (locals.var_temp2_dn7 / (2.0 * assign41230_e47141)))) / (assign41230_e47141 * assign41230_e47141)), ((((((((assign41230_e47133 * locals.var_dxdrift__blk1017_dn8) * locals.var_temp3) + (assign41230_e47135 * locals.var_temp3_dn8)) * locals.var_temp3) + (assign41230_e47137 * locals.var_temp3_dn8)) * assign41230_e47141) - (assign41230_e47139 * (locals.var_temp2_dn8 / (2.0 * assign41230_e47141)))) / (assign41230_e47141 * assign41230_e47141)), ((((((((assign41230_e47133 * locals.var_dxdrift__blk1017_dn9) * locals.var_temp3) + (assign41230_e47135 * locals.var_temp3_dn9)) * locals.var_temp3) + (assign41230_e47137 * locals.var_temp3_dn9)) * assign41230_e47141) - (assign41230_e47139 * (locals.var_temp2_dn9 / (2.0 * assign41230_e47141)))) / (assign41230_e47141 * assign41230_e47141)), );
            locals.var_idrift2__blk1062_rv = 0.0;
        }

        if ((locals.var_guard1080 != 0.0) && (locals.var_guard1228 == 0.0)) {
            (locals.var_dd__blk1057, locals.var_dd__blk1057_dn4, locals.var_dd__blk1057_dn6, locals.var_dd__blk1057_dn7, locals.var_dd__blk1057_dn8, locals.var_dd__blk1057_dn9, ) = (locals.var_dinf__blk974, locals.var_dinf__blk974_dn4, locals.var_dinf__blk974_dn6, locals.var_dinf__blk974_dn7, locals.var_dinf__blk974_dn8, locals.var_dinf__blk974_dn9, );
            locals.var_dd__blk1057_rv = 0.0;
            (locals.var_idrift2__blk1062, locals.var_idrift2__blk1062_dn4, locals.var_idrift2__blk1062_dn6, locals.var_idrift2__blk1062_dn7, locals.var_idrift2__blk1062_dn8, locals.var_idrift2__blk1062_dn9, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_idrift2__blk1062_rv = 0.0;
        }

        if (locals.var_guard1080 != 0.0) {
            let assign41260_e47162: f64 = (locals.var_qim__blk1016 * locals.var_dxdrift__blk1017);
            let assign41260_e47164: f64 = (assign41260_e47162 + locals.var_idrift2__blk1062);
            let assign41260_e47166: f64 = (assign41260_e47164 + locals.var_qis__blk938);
            let assign41260_e47168: f64 = (assign41260_e47166 - locals.var_qid__blk1003);
            (locals.var_norm_ids__blk1063, locals.var_norm_ids__blk1063_dn4, locals.var_norm_ids__blk1063_dn6, locals.var_norm_ids__blk1063_dn7, locals.var_norm_ids__blk1063_dn8, locals.var_norm_ids__blk1063_dn9, ) = (assign41260_e47168, (((((locals.var_qim__blk1016_dn4 * locals.var_dxdrift__blk1017) + (locals.var_qim__blk1016 * locals.var_dxdrift__blk1017_dn4)) + locals.var_idrift2__blk1062_dn4) + locals.var_qis__blk938_dn4) - locals.var_qid__blk1003_dn4), (((((locals.var_qim__blk1016_dn6 * locals.var_dxdrift__blk1017) + (locals.var_qim__blk1016 * locals.var_dxdrift__blk1017_dn6)) + locals.var_idrift2__blk1062_dn6) + locals.var_qis__blk938_dn6) - locals.var_qid__blk1003_dn6), (((((locals.var_qim__blk1016_dn7 * locals.var_dxdrift__blk1017) + (locals.var_qim__blk1016 * locals.var_dxdrift__blk1017_dn7)) + locals.var_idrift2__blk1062_dn7) + locals.var_qis__blk938_dn7) - locals.var_qid__blk1003_dn7), (((((locals.var_qim__blk1016_dn8 * locals.var_dxdrift__blk1017) + (locals.var_qim__blk1016 * locals.var_dxdrift__blk1017_dn8)) + locals.var_idrift2__blk1062_dn8) + locals.var_qis__blk938_dn8) - locals.var_qid__blk1003_dn8), (((((locals.var_qim__blk1016_dn9 * locals.var_dxdrift__blk1017) + (locals.var_qim__blk1016 * locals.var_dxdrift__blk1017_dn9)) + locals.var_idrift2__blk1062_dn9) + locals.var_qis__blk938_dn9) - locals.var_qid__blk1003_dn9), );
            locals.var_norm_ids__blk1063_rv = 0.0;
        }

        let assign41270_e47173: f64 = if locals.var_qis__blk938 > 1e-6 { 1.0 } else { 0.0 };
        locals.var_guard1232 = assign41270_e47173;
        locals.var_guard1232_rv = 0.0;

        let assign41280_e47176: f64 = if locals.var_norm_ids__blk1063 > 1e-30 { 1.0 } else { 0.0 };
        locals.var_guard1233 = assign41280_e47176;
        locals.var_guard1233_rv = 0.0;

        if (((locals.var_guard1080 != 0.0) && (locals.var_guard1232 != 0.0)) && (locals.var_guard1233 != 0.0)) {
            let assign41290_e47185: f64 = (locals.var_aexp1s__blk943 / locals.var_qis__blk938);
            let assign41290_e47187: f64 = (assign41290_e47185 - locals.var_dqsqs_dxn_qi__blk950);
            let assign41290_e47188: f64 = (locals.var_a1s__blk947 / assign41290_e47187);
            (locals.var_q1s_chap__blk1064, locals.var_q1s_chap__blk1064_dn4, locals.var_q1s_chap__blk1064_dn6, locals.var_q1s_chap__blk1064_dn7, locals.var_q1s_chap__blk1064_dn8, locals.var_q1s_chap__blk1064_dn9, ) = (assign41290_e47188, (((locals.var_a1s__blk947_dn4 * assign41290_e47187) - (locals.var_a1s__blk947 * ((((locals.var_aexp1s__blk943_dn4 * locals.var_qis__blk938) - (locals.var_aexp1s__blk943 * locals.var_qis__blk938_dn4)) / (locals.var_qis__blk938 * locals.var_qis__blk938)) - locals.var_dqsqs_dxn_qi__blk950_dn4))) / (assign41290_e47187 * assign41290_e47187)), (((locals.var_a1s__blk947_dn6 * assign41290_e47187) - (locals.var_a1s__blk947 * ((((locals.var_aexp1s__blk943_dn6 * locals.var_qis__blk938) - (locals.var_aexp1s__blk943 * locals.var_qis__blk938_dn6)) / (locals.var_qis__blk938 * locals.var_qis__blk938)) - locals.var_dqsqs_dxn_qi__blk950_dn6))) / (assign41290_e47187 * assign41290_e47187)), (((locals.var_a1s__blk947_dn7 * assign41290_e47187) - (locals.var_a1s__blk947 * ((((locals.var_aexp1s__blk943_dn7 * locals.var_qis__blk938) - (locals.var_aexp1s__blk943 * locals.var_qis__blk938_dn7)) / (locals.var_qis__blk938 * locals.var_qis__blk938)) - locals.var_dqsqs_dxn_qi__blk950_dn7))) / (assign41290_e47187 * assign41290_e47187)), (((locals.var_a1s__blk947_dn8 * assign41290_e47187) - (locals.var_a1s__blk947 * ((((locals.var_aexp1s__blk943_dn8 * locals.var_qis__blk938) - (locals.var_aexp1s__blk943 * locals.var_qis__blk938_dn8)) / (locals.var_qis__blk938 * locals.var_qis__blk938)) - locals.var_dqsqs_dxn_qi__blk950_dn8))) / (assign41290_e47187 * assign41290_e47187)), (((locals.var_a1s__blk947_dn9 * assign41290_e47187) - (locals.var_a1s__blk947 * ((((locals.var_aexp1s__blk943_dn9 * locals.var_qis__blk938) - (locals.var_aexp1s__blk943 * locals.var_qis__blk938_dn9)) / (locals.var_qis__blk938 * locals.var_qis__blk938)) - locals.var_dqsqs_dxn_qi__blk950_dn9))) / (assign41290_e47187 * assign41290_e47187)), );
            locals.var_q1s_chap__blk1064_rv = 0.0;
        }

        if (((locals.var_guard1080 != 0.0) && (locals.var_guard1232 != 0.0)) && (locals.var_guard1233 != 0.0)) {
            let assign41300_e47199: f64 = (locals.var_aexp1d__blk1007 / locals.var_qid__blk1003);
            let assign41300_e47201: f64 = (assign41300_e47199 - locals.var_dqsqd_dxn_qi__blk1014);
            let assign41300_e47202: f64 = (locals.var_a1d__blk1011 / assign41300_e47201);
            (locals.var_q1d_chap__blk1065, locals.var_q1d_chap__blk1065_dn4, locals.var_q1d_chap__blk1065_dn6, locals.var_q1d_chap__blk1065_dn7, locals.var_q1d_chap__blk1065_dn8, locals.var_q1d_chap__blk1065_dn9, ) = (assign41300_e47202, (((locals.var_a1d__blk1011_dn4 * assign41300_e47201) - (locals.var_a1d__blk1011 * ((((locals.var_aexp1d__blk1007_dn4 * locals.var_qid__blk1003) - (locals.var_aexp1d__blk1007 * locals.var_qid__blk1003_dn4)) / (locals.var_qid__blk1003 * locals.var_qid__blk1003)) - locals.var_dqsqd_dxn_qi__blk1014_dn4))) / (assign41300_e47201 * assign41300_e47201)), (((locals.var_a1d__blk1011_dn6 * assign41300_e47201) - (locals.var_a1d__blk1011 * ((((locals.var_aexp1d__blk1007_dn6 * locals.var_qid__blk1003) - (locals.var_aexp1d__blk1007 * locals.var_qid__blk1003_dn6)) / (locals.var_qid__blk1003 * locals.var_qid__blk1003)) - locals.var_dqsqd_dxn_qi__blk1014_dn6))) / (assign41300_e47201 * assign41300_e47201)), (((locals.var_a1d__blk1011_dn7 * assign41300_e47201) - (locals.var_a1d__blk1011 * ((((locals.var_aexp1d__blk1007_dn7 * locals.var_qid__blk1003) - (locals.var_aexp1d__blk1007 * locals.var_qid__blk1003_dn7)) / (locals.var_qid__blk1003 * locals.var_qid__blk1003)) - locals.var_dqsqd_dxn_qi__blk1014_dn7))) / (assign41300_e47201 * assign41300_e47201)), (((locals.var_a1d__blk1011_dn8 * assign41300_e47201) - (locals.var_a1d__blk1011 * ((((locals.var_aexp1d__blk1007_dn8 * locals.var_qid__blk1003) - (locals.var_aexp1d__blk1007 * locals.var_qid__blk1003_dn8)) / (locals.var_qid__blk1003 * locals.var_qid__blk1003)) - locals.var_dqsqd_dxn_qi__blk1014_dn8))) / (assign41300_e47201 * assign41300_e47201)), (((locals.var_a1d__blk1011_dn9 * assign41300_e47201) - (locals.var_a1d__blk1011 * ((((locals.var_aexp1d__blk1007_dn9 * locals.var_qid__blk1003) - (locals.var_aexp1d__blk1007 * locals.var_qid__blk1003_dn9)) / (locals.var_qid__blk1003 * locals.var_qid__blk1003)) - locals.var_dqsqd_dxn_qi__blk1014_dn9))) / (assign41300_e47201 * assign41300_e47201)), );
            locals.var_q1d_chap__blk1065_rv = 0.0;
        }

        if (((locals.var_guard1080 != 0.0) && (locals.var_guard1232 != 0.0)) && (locals.var_guard1233 != 0.0)) {
            let assign41310_e47212: f64 = (locals.var_q1s_chap__blk1064 - locals.var_q1d_chap__blk1065);
            let assign41310_e47214: f64 = (assign41310_e47212 / locals.var_norm_ids__blk1063);
            (locals.var_inv_k1h1_0__blk1066, locals.var_inv_k1h1_0__blk1066_dn4, locals.var_inv_k1h1_0__blk1066_dn6, locals.var_inv_k1h1_0__blk1066_dn7, locals.var_inv_k1h1_0__blk1066_dn8, locals.var_inv_k1h1_0__blk1066_dn9, ) = (assign41310_e47214, ((((locals.var_q1s_chap__blk1064_dn4 - locals.var_q1d_chap__blk1065_dn4) * locals.var_norm_ids__blk1063) - (assign41310_e47212 * locals.var_norm_ids__blk1063_dn4)) / (locals.var_norm_ids__blk1063 * locals.var_norm_ids__blk1063)), ((((locals.var_q1s_chap__blk1064_dn6 - locals.var_q1d_chap__blk1065_dn6) * locals.var_norm_ids__blk1063) - (assign41310_e47212 * locals.var_norm_ids__blk1063_dn6)) / (locals.var_norm_ids__blk1063 * locals.var_norm_ids__blk1063)), ((((locals.var_q1s_chap__blk1064_dn7 - locals.var_q1d_chap__blk1065_dn7) * locals.var_norm_ids__blk1063) - (assign41310_e47212 * locals.var_norm_ids__blk1063_dn7)) / (locals.var_norm_ids__blk1063 * locals.var_norm_ids__blk1063)), ((((locals.var_q1s_chap__blk1064_dn8 - locals.var_q1d_chap__blk1065_dn8) * locals.var_norm_ids__blk1063) - (assign41310_e47212 * locals.var_norm_ids__blk1063_dn8)) / (locals.var_norm_ids__blk1063 * locals.var_norm_ids__blk1063)), ((((locals.var_q1s_chap__blk1064_dn9 - locals.var_q1d_chap__blk1065_dn9) * locals.var_norm_ids__blk1063) - (assign41310_e47212 * locals.var_norm_ids__blk1063_dn9)) / (locals.var_norm_ids__blk1063 * locals.var_norm_ids__blk1063)), );
            locals.var_inv_k1h1_0__blk1066_rv = 0.0;
        }

        if (((locals.var_guard1080 != 0.0) && (locals.var_guard1232 != 0.0)) && (locals.var_guard1233 != 0.0)) {
            let assign41320_e47225: f64 = (locals.var_aexp2s__blk944 / locals.var_qis__blk938);
            let assign41320_e47227: f64 = (assign41320_e47225 - locals.var_dqsqs_dxn_qi__blk950);
            let assign41320_e47228: f64 = (locals.var_a2s__blk948 / assign41320_e47227);
            (locals.var_q2s_chap__blk1067, locals.var_q2s_chap__blk1067_dn4, locals.var_q2s_chap__blk1067_dn6, locals.var_q2s_chap__blk1067_dn7, locals.var_q2s_chap__blk1067_dn8, locals.var_q2s_chap__blk1067_dn9, ) = (assign41320_e47228, (((locals.var_a2s__blk948_dn4 * assign41320_e47227) - (locals.var_a2s__blk948 * ((((locals.var_aexp2s__blk944_dn4 * locals.var_qis__blk938) - (locals.var_aexp2s__blk944 * locals.var_qis__blk938_dn4)) / (locals.var_qis__blk938 * locals.var_qis__blk938)) - locals.var_dqsqs_dxn_qi__blk950_dn4))) / (assign41320_e47227 * assign41320_e47227)), (((locals.var_a2s__blk948_dn6 * assign41320_e47227) - (locals.var_a2s__blk948 * ((((locals.var_aexp2s__blk944_dn6 * locals.var_qis__blk938) - (locals.var_aexp2s__blk944 * locals.var_qis__blk938_dn6)) / (locals.var_qis__blk938 * locals.var_qis__blk938)) - locals.var_dqsqs_dxn_qi__blk950_dn6))) / (assign41320_e47227 * assign41320_e47227)), (((locals.var_a2s__blk948_dn7 * assign41320_e47227) - (locals.var_a2s__blk948 * ((((locals.var_aexp2s__blk944_dn7 * locals.var_qis__blk938) - (locals.var_aexp2s__blk944 * locals.var_qis__blk938_dn7)) / (locals.var_qis__blk938 * locals.var_qis__blk938)) - locals.var_dqsqs_dxn_qi__blk950_dn7))) / (assign41320_e47227 * assign41320_e47227)), (((locals.var_a2s__blk948_dn8 * assign41320_e47227) - (locals.var_a2s__blk948 * ((((locals.var_aexp2s__blk944_dn8 * locals.var_qis__blk938) - (locals.var_aexp2s__blk944 * locals.var_qis__blk938_dn8)) / (locals.var_qis__blk938 * locals.var_qis__blk938)) - locals.var_dqsqs_dxn_qi__blk950_dn8))) / (assign41320_e47227 * assign41320_e47227)), (((locals.var_a2s__blk948_dn9 * assign41320_e47227) - (locals.var_a2s__blk948 * ((((locals.var_aexp2s__blk944_dn9 * locals.var_qis__blk938) - (locals.var_aexp2s__blk944 * locals.var_qis__blk938_dn9)) / (locals.var_qis__blk938 * locals.var_qis__blk938)) - locals.var_dqsqs_dxn_qi__blk950_dn9))) / (assign41320_e47227 * assign41320_e47227)), );
            locals.var_q2s_chap__blk1067_rv = 0.0;
        }

        if (((locals.var_guard1080 != 0.0) && (locals.var_guard1232 != 0.0)) && (locals.var_guard1233 != 0.0)) {
            let assign41330_e47239: f64 = (locals.var_aexp2d__blk1008 / locals.var_qid__blk1003);
            let assign41330_e47241: f64 = (assign41330_e47239 - locals.var_dqsqd_dxn_qi__blk1014);
            let assign41330_e47242: f64 = (locals.var_a2d__blk1012 / assign41330_e47241);
            (locals.var_q2d_chap__blk1068, locals.var_q2d_chap__blk1068_dn4, locals.var_q2d_chap__blk1068_dn6, locals.var_q2d_chap__blk1068_dn7, locals.var_q2d_chap__blk1068_dn8, locals.var_q2d_chap__blk1068_dn9, ) = (assign41330_e47242, (((locals.var_a2d__blk1012_dn4 * assign41330_e47241) - (locals.var_a2d__blk1012 * ((((locals.var_aexp2d__blk1008_dn4 * locals.var_qid__blk1003) - (locals.var_aexp2d__blk1008 * locals.var_qid__blk1003_dn4)) / (locals.var_qid__blk1003 * locals.var_qid__blk1003)) - locals.var_dqsqd_dxn_qi__blk1014_dn4))) / (assign41330_e47241 * assign41330_e47241)), (((locals.var_a2d__blk1012_dn6 * assign41330_e47241) - (locals.var_a2d__blk1012 * ((((locals.var_aexp2d__blk1008_dn6 * locals.var_qid__blk1003) - (locals.var_aexp2d__blk1008 * locals.var_qid__blk1003_dn6)) / (locals.var_qid__blk1003 * locals.var_qid__blk1003)) - locals.var_dqsqd_dxn_qi__blk1014_dn6))) / (assign41330_e47241 * assign41330_e47241)), (((locals.var_a2d__blk1012_dn7 * assign41330_e47241) - (locals.var_a2d__blk1012 * ((((locals.var_aexp2d__blk1008_dn7 * locals.var_qid__blk1003) - (locals.var_aexp2d__blk1008 * locals.var_qid__blk1003_dn7)) / (locals.var_qid__blk1003 * locals.var_qid__blk1003)) - locals.var_dqsqd_dxn_qi__blk1014_dn7))) / (assign41330_e47241 * assign41330_e47241)), (((locals.var_a2d__blk1012_dn8 * assign41330_e47241) - (locals.var_a2d__blk1012 * ((((locals.var_aexp2d__blk1008_dn8 * locals.var_qid__blk1003) - (locals.var_aexp2d__blk1008 * locals.var_qid__blk1003_dn8)) / (locals.var_qid__blk1003 * locals.var_qid__blk1003)) - locals.var_dqsqd_dxn_qi__blk1014_dn8))) / (assign41330_e47241 * assign41330_e47241)), (((locals.var_a2d__blk1012_dn9 * assign41330_e47241) - (locals.var_a2d__blk1012 * ((((locals.var_aexp2d__blk1008_dn9 * locals.var_qid__blk1003) - (locals.var_aexp2d__blk1008 * locals.var_qid__blk1003_dn9)) / (locals.var_qid__blk1003 * locals.var_qid__blk1003)) - locals.var_dqsqd_dxn_qi__blk1014_dn9))) / (assign41330_e47241 * assign41330_e47241)), );
            locals.var_q2d_chap__blk1068_rv = 0.0;
        }

        if (((locals.var_guard1080 != 0.0) && (locals.var_guard1232 != 0.0)) && (locals.var_guard1233 != 0.0)) {
            let assign41340_e47252: f64 = (locals.var_q2s_chap__blk1067 - locals.var_q2d_chap__blk1068);
            let assign41340_e47254: f64 = (assign41340_e47252 / locals.var_norm_ids__blk1063);
            (locals.var_inv_k2h2_0__blk1069, locals.var_inv_k2h2_0__blk1069_dn4, locals.var_inv_k2h2_0__blk1069_dn6, locals.var_inv_k2h2_0__blk1069_dn7, locals.var_inv_k2h2_0__blk1069_dn8, locals.var_inv_k2h2_0__blk1069_dn9, ) = (assign41340_e47254, ((((locals.var_q2s_chap__blk1067_dn4 - locals.var_q2d_chap__blk1068_dn4) * locals.var_norm_ids__blk1063) - (assign41340_e47252 * locals.var_norm_ids__blk1063_dn4)) / (locals.var_norm_ids__blk1063 * locals.var_norm_ids__blk1063)), ((((locals.var_q2s_chap__blk1067_dn6 - locals.var_q2d_chap__blk1068_dn6) * locals.var_norm_ids__blk1063) - (assign41340_e47252 * locals.var_norm_ids__blk1063_dn6)) / (locals.var_norm_ids__blk1063 * locals.var_norm_ids__blk1063)), ((((locals.var_q2s_chap__blk1067_dn7 - locals.var_q2d_chap__blk1068_dn7) * locals.var_norm_ids__blk1063) - (assign41340_e47252 * locals.var_norm_ids__blk1063_dn7)) / (locals.var_norm_ids__blk1063 * locals.var_norm_ids__blk1063)), ((((locals.var_q2s_chap__blk1067_dn8 - locals.var_q2d_chap__blk1068_dn8) * locals.var_norm_ids__blk1063) - (assign41340_e47252 * locals.var_norm_ids__blk1063_dn8)) / (locals.var_norm_ids__blk1063 * locals.var_norm_ids__blk1063)), ((((locals.var_q2s_chap__blk1067_dn9 - locals.var_q2d_chap__blk1068_dn9) * locals.var_norm_ids__blk1063) - (assign41340_e47252 * locals.var_norm_ids__blk1063_dn9)) / (locals.var_norm_ids__blk1063 * locals.var_norm_ids__blk1063)), );
            locals.var_inv_k2h2_0__blk1069_rv = 0.0;
        }

        if (((locals.var_guard1080 != 0.0) && (locals.var_guard1232 != 0.0)) && (locals.var_guard1233 == 0.0)) {
            (locals.var_inv_k1h1_0__blk1066, locals.var_inv_k1h1_0__blk1066_dn4, locals.var_inv_k1h1_0__blk1066_dn6, locals.var_inv_k1h1_0__blk1066_dn7, locals.var_inv_k1h1_0__blk1066_dn8, locals.var_inv_k1h1_0__blk1066_dn9, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_inv_k1h1_0__blk1066_rv = 0.0;
            (locals.var_inv_k2h2_0__blk1069, locals.var_inv_k2h2_0__blk1069_dn4, locals.var_inv_k2h2_0__blk1069_dn6, locals.var_inv_k2h2_0__blk1069_dn7, locals.var_inv_k2h2_0__blk1069_dn8, locals.var_inv_k2h2_0__blk1069_dn9, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_inv_k2h2_0__blk1069_rv = 0.0;
        }

        if ((locals.var_guard1080 != 0.0) && (locals.var_guard1232 == 0.0)) {
            let assign41370_e47280: f64 = (-2.0);
            let assign41370_e47282: f64 = (assign41370_e47280 * locals.var_s1__blk969);
            let assign41370_e47285: f64 = (locals.var_inv_k1__blk906 / locals.var_q1chapinf__blk972);
            let assign41370_e47287: f64 = (assign41370_e47285 + locals.var_inv_dinf__blk975);
            let assign41370_e47288: f64 = (assign41370_e47282 * assign41370_e47287);
            (locals.var_zeta1__blk1070, locals.var_zeta1__blk1070_dn4, locals.var_zeta1__blk1070_dn6, locals.var_zeta1__blk1070_dn7, locals.var_zeta1__blk1070_dn8, locals.var_zeta1__blk1070_dn9, ) = (assign41370_e47288, (((assign41370_e47280 * locals.var_s1__blk969_dn4) * assign41370_e47287) + (assign41370_e47282 * ((((locals.var_inv_k1__blk906_dn4 * locals.var_q1chapinf__blk972) - (locals.var_inv_k1__blk906 * locals.var_q1chapinf__blk972_dn4)) / (locals.var_q1chapinf__blk972 * locals.var_q1chapinf__blk972)) + locals.var_inv_dinf__blk975_dn4))), (((assign41370_e47280 * locals.var_s1__blk969_dn6) * assign41370_e47287) + (assign41370_e47282 * ((((locals.var_inv_k1__blk906_dn6 * locals.var_q1chapinf__blk972) - (locals.var_inv_k1__blk906 * locals.var_q1chapinf__blk972_dn6)) / (locals.var_q1chapinf__blk972 * locals.var_q1chapinf__blk972)) + locals.var_inv_dinf__blk975_dn6))), (((assign41370_e47280 * locals.var_s1__blk969_dn7) * assign41370_e47287) + (assign41370_e47282 * ((((locals.var_inv_k1__blk906_dn7 * locals.var_q1chapinf__blk972) - (locals.var_inv_k1__blk906 * locals.var_q1chapinf__blk972_dn7)) / (locals.var_q1chapinf__blk972 * locals.var_q1chapinf__blk972)) + locals.var_inv_dinf__blk975_dn7))), (((assign41370_e47280 * locals.var_s1__blk969_dn8) * assign41370_e47287) + (assign41370_e47282 * ((((locals.var_inv_k1__blk906_dn8 * locals.var_q1chapinf__blk972) - (locals.var_inv_k1__blk906 * locals.var_q1chapinf__blk972_dn8)) / (locals.var_q1chapinf__blk972 * locals.var_q1chapinf__blk972)) + locals.var_inv_dinf__blk975_dn8))), (((assign41370_e47280 * locals.var_s1__blk969_dn9) * assign41370_e47287) + (assign41370_e47282 * ((((locals.var_inv_k1__blk906_dn9 * locals.var_q1chapinf__blk972) - (locals.var_inv_k1__blk906 * locals.var_q1chapinf__blk972_dn9)) / (locals.var_q1chapinf__blk972 * locals.var_q1chapinf__blk972)) + locals.var_inv_dinf__blk975_dn9))), );
            locals.var_zeta1__blk1070_rv = 0.0;
        }

        if ((locals.var_guard1080 != 0.0) && (locals.var_guard1232 == 0.0)) {
            let assign41380_e47296: f64 = (-2.0);
            let assign41380_e47298: f64 = (assign41380_e47296 * locals.var_s2__blk970);
            let assign41380_e47301: f64 = (locals.var_inv_k2__blk907 / locals.var_q2chapinf__blk973);
            let assign41380_e47303: f64 = (assign41380_e47301 + locals.var_inv_dinf__blk975);
            let assign41380_e47304: f64 = (assign41380_e47298 * assign41380_e47303);
            (locals.var_zeta2__blk1071, locals.var_zeta2__blk1071_dn4, locals.var_zeta2__blk1071_dn6, locals.var_zeta2__blk1071_dn7, locals.var_zeta2__blk1071_dn8, locals.var_zeta2__blk1071_dn9, ) = (assign41380_e47304, (((assign41380_e47296 * locals.var_s2__blk970_dn4) * assign41380_e47303) + (assign41380_e47298 * ((((locals.var_inv_k2__blk907_dn4 * locals.var_q2chapinf__blk973) - (locals.var_inv_k2__blk907 * locals.var_q2chapinf__blk973_dn4)) / (locals.var_q2chapinf__blk973 * locals.var_q2chapinf__blk973)) + locals.var_inv_dinf__blk975_dn4))), (((assign41380_e47296 * locals.var_s2__blk970_dn6) * assign41380_e47303) + (assign41380_e47298 * ((((locals.var_inv_k2__blk907_dn6 * locals.var_q2chapinf__blk973) - (locals.var_inv_k2__blk907 * locals.var_q2chapinf__blk973_dn6)) / (locals.var_q2chapinf__blk973 * locals.var_q2chapinf__blk973)) + locals.var_inv_dinf__blk975_dn6))), (((assign41380_e47296 * locals.var_s2__blk970_dn7) * assign41380_e47303) + (assign41380_e47298 * ((((locals.var_inv_k2__blk907_dn7 * locals.var_q2chapinf__blk973) - (locals.var_inv_k2__blk907 * locals.var_q2chapinf__blk973_dn7)) / (locals.var_q2chapinf__blk973 * locals.var_q2chapinf__blk973)) + locals.var_inv_dinf__blk975_dn7))), (((assign41380_e47296 * locals.var_s2__blk970_dn8) * assign41380_e47303) + (assign41380_e47298 * ((((locals.var_inv_k2__blk907_dn8 * locals.var_q2chapinf__blk973) - (locals.var_inv_k2__blk907 * locals.var_q2chapinf__blk973_dn8)) / (locals.var_q2chapinf__blk973 * locals.var_q2chapinf__blk973)) + locals.var_inv_dinf__blk975_dn8))), (((assign41380_e47296 * locals.var_s2__blk970_dn9) * assign41380_e47303) + (assign41380_e47298 * ((((locals.var_inv_k2__blk907_dn9 * locals.var_q2chapinf__blk973) - (locals.var_inv_k2__blk907 * locals.var_q2chapinf__blk973_dn9)) / (locals.var_q2chapinf__blk973 * locals.var_q2chapinf__blk973)) + locals.var_inv_dinf__blk975_dn9))), );
            locals.var_zeta2__blk1071_rv = 0.0;
        }

        if ((locals.var_guard1080 != 0.0) && (locals.var_guard1232 == 0.0)) {
            let assign41390_e47313: f64 = (locals.var_zeta2__blk1071 - locals.var_zeta1__blk1070);
            let assign41390_e47315: f64 = (assign41390_e47313 * locals.var_inv_dinf__blk975);
            (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign41390_e47315, (((locals.var_zeta2__blk1071_dn4 - locals.var_zeta1__blk1070_dn4) * locals.var_inv_dinf__blk975) + (assign41390_e47313 * locals.var_inv_dinf__blk975_dn4)), (((locals.var_zeta2__blk1071_dn6 - locals.var_zeta1__blk1070_dn6) * locals.var_inv_dinf__blk975) + (assign41390_e47313 * locals.var_inv_dinf__blk975_dn6)), (((locals.var_zeta2__blk1071_dn7 - locals.var_zeta1__blk1070_dn7) * locals.var_inv_dinf__blk975) + (assign41390_e47313 * locals.var_inv_dinf__blk975_dn7)), (((locals.var_zeta2__blk1071_dn8 - locals.var_zeta1__blk1070_dn8) * locals.var_inv_dinf__blk975) + (assign41390_e47313 * locals.var_inv_dinf__blk975_dn8)), (((locals.var_zeta2__blk1071_dn9 - locals.var_zeta1__blk1070_dn9) * locals.var_inv_dinf__blk975) + (assign41390_e47313 * locals.var_inv_dinf__blk975_dn9)), );
            locals.var_temp_rv = 0.0;
        }

        if ((locals.var_guard1080 != 0.0) && (locals.var_guard1232 == 0.0)) {
            let assign41400_e47324: f64 = (locals.var_zeta1__blk1070 * locals.var_inv_k1__blk906);
            (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9, ) = (assign41400_e47324, ((locals.var_zeta1__blk1070_dn4 * locals.var_inv_k1__blk906) + (locals.var_zeta1__blk1070 * locals.var_inv_k1__blk906_dn4)), ((locals.var_zeta1__blk1070_dn6 * locals.var_inv_k1__blk906) + (locals.var_zeta1__blk1070 * locals.var_inv_k1__blk906_dn6)), ((locals.var_zeta1__blk1070_dn7 * locals.var_inv_k1__blk906) + (locals.var_zeta1__blk1070 * locals.var_inv_k1__blk906_dn7)), ((locals.var_zeta1__blk1070_dn8 * locals.var_inv_k1__blk906) + (locals.var_zeta1__blk1070 * locals.var_inv_k1__blk906_dn8)), ((locals.var_zeta1__blk1070_dn9 * locals.var_inv_k1__blk906) + (locals.var_zeta1__blk1070 * locals.var_inv_k1__blk906_dn9)), );
            locals.var_temp1_rv = 0.0;
        }

        if ((locals.var_guard1080 != 0.0) && (locals.var_guard1232 == 0.0)) {
            let assign41410_e47333: f64 = (locals.var_zeta2__blk1071 * locals.var_inv_k2__blk907);
            (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9, ) = (assign41410_e47333, ((locals.var_zeta2__blk1071_dn4 * locals.var_inv_k2__blk907) + (locals.var_zeta2__blk1071 * locals.var_inv_k2__blk907_dn4)), ((locals.var_zeta2__blk1071_dn6 * locals.var_inv_k2__blk907) + (locals.var_zeta2__blk1071 * locals.var_inv_k2__blk907_dn6)), ((locals.var_zeta2__blk1071_dn7 * locals.var_inv_k2__blk907) + (locals.var_zeta2__blk1071 * locals.var_inv_k2__blk907_dn7)), ((locals.var_zeta2__blk1071_dn8 * locals.var_inv_k2__blk907) + (locals.var_zeta2__blk1071 * locals.var_inv_k2__blk907_dn8)), ((locals.var_zeta2__blk1071_dn9 * locals.var_inv_k2__blk907) + (locals.var_zeta2__blk1071 * locals.var_inv_k2__blk907_dn9)), );
            locals.var_temp2_rv = 0.0;
        }

        if ((locals.var_guard1080 != 0.0) && (locals.var_guard1232 == 0.0)) {
            let assign41420_e47342: f64 = (locals.var_temp1 + locals.var_temp2);
            (locals.var_temp3, locals.var_temp3_dn4, locals.var_temp3_dn6, locals.var_temp3_dn7, locals.var_temp3_dn8, locals.var_temp3_dn9, ) = (assign41420_e47342, (locals.var_temp1_dn4 + locals.var_temp2_dn4), (locals.var_temp1_dn6 + locals.var_temp2_dn6), (locals.var_temp1_dn7 + locals.var_temp2_dn7), (locals.var_temp1_dn8 + locals.var_temp2_dn8), (locals.var_temp1_dn9 + locals.var_temp2_dn9), );
            locals.var_temp3_rv = 0.0;
        }

        if ((locals.var_guard1080 != 0.0) && (locals.var_guard1232 == 0.0)) {
            let assign41430_e47353: f64 = (locals.var_s1__blk969 * locals.var_inv_k1__blk906);
            let assign41430_e47356: f64 = (locals.var_s2__blk970 * locals.var_inv_k2__blk907);
            let assign41430_e47357: f64 = (assign41430_e47353 + assign41430_e47356);
            let assign41430_e47358: f64 = (2.0 * assign41430_e47357);
            let assign41430_e47359: f64 = (3.0 + assign41430_e47358);
            (locals.var_temp4, locals.var_temp4_dn4, locals.var_temp4_dn6, locals.var_temp4_dn7, locals.var_temp4_dn8, locals.var_temp4_dn9, ) = (assign41430_e47359, (2.0 * (((locals.var_s1__blk969_dn4 * locals.var_inv_k1__blk906) + (locals.var_s1__blk969 * locals.var_inv_k1__blk906_dn4)) + ((locals.var_s2__blk970_dn4 * locals.var_inv_k2__blk907) + (locals.var_s2__blk970 * locals.var_inv_k2__blk907_dn4)))), (2.0 * (((locals.var_s1__blk969_dn6 * locals.var_inv_k1__blk906) + (locals.var_s1__blk969 * locals.var_inv_k1__blk906_dn6)) + ((locals.var_s2__blk970_dn6 * locals.var_inv_k2__blk907) + (locals.var_s2__blk970 * locals.var_inv_k2__blk907_dn6)))), (2.0 * (((locals.var_s1__blk969_dn7 * locals.var_inv_k1__blk906) + (locals.var_s1__blk969 * locals.var_inv_k1__blk906_dn7)) + ((locals.var_s2__blk970_dn7 * locals.var_inv_k2__blk907) + (locals.var_s2__blk970 * locals.var_inv_k2__blk907_dn7)))), (2.0 * (((locals.var_s1__blk969_dn8 * locals.var_inv_k1__blk906) + (locals.var_s1__blk969 * locals.var_inv_k1__blk906_dn8)) + ((locals.var_s2__blk970_dn8 * locals.var_inv_k2__blk907) + (locals.var_s2__blk970 * locals.var_inv_k2__blk907_dn8)))), (2.0 * (((locals.var_s1__blk969_dn9 * locals.var_inv_k1__blk906) + (locals.var_s1__blk969 * locals.var_inv_k1__blk906_dn9)) + ((locals.var_s2__blk970_dn9 * locals.var_inv_k2__blk907) + (locals.var_s2__blk970 * locals.var_inv_k2__blk907_dn9)))), );
            locals.var_temp4_rv = 0.0;
        }

        if ((locals.var_guard1080 != 0.0) && (locals.var_guard1232 == 0.0)) {
            let assign41440_e47368: f64 = (locals.var_temp2 + locals.var_temp);
            let assign41440_e47371: f64 = (locals.var_temp3 / locals.var_q1chapinf__blk972);
            let assign41440_e47372: f64 = (assign41440_e47368 - assign41440_e47371);
            let assign41440_e47374: f64 = (assign41440_e47372 / locals.var_temp4);
            (locals.var_ksi1__blk1072, locals.var_ksi1__blk1072_dn4, locals.var_ksi1__blk1072_dn6, locals.var_ksi1__blk1072_dn7, locals.var_ksi1__blk1072_dn8, locals.var_ksi1__blk1072_dn9, ) = (assign41440_e47374, (((((locals.var_temp2_dn4 + locals.var_temp_dn4) - (((locals.var_temp3_dn4 * locals.var_q1chapinf__blk972) - (locals.var_temp3 * locals.var_q1chapinf__blk972_dn4)) / (locals.var_q1chapinf__blk972 * locals.var_q1chapinf__blk972))) * locals.var_temp4) - (assign41440_e47372 * locals.var_temp4_dn4)) / (locals.var_temp4 * locals.var_temp4)), (((((locals.var_temp2_dn6 + locals.var_temp_dn6) - (((locals.var_temp3_dn6 * locals.var_q1chapinf__blk972) - (locals.var_temp3 * locals.var_q1chapinf__blk972_dn6)) / (locals.var_q1chapinf__blk972 * locals.var_q1chapinf__blk972))) * locals.var_temp4) - (assign41440_e47372 * locals.var_temp4_dn6)) / (locals.var_temp4 * locals.var_temp4)), (((((locals.var_temp2_dn7 + locals.var_temp_dn7) - (((locals.var_temp3_dn7 * locals.var_q1chapinf__blk972) - (locals.var_temp3 * locals.var_q1chapinf__blk972_dn7)) / (locals.var_q1chapinf__blk972 * locals.var_q1chapinf__blk972))) * locals.var_temp4) - (assign41440_e47372 * locals.var_temp4_dn7)) / (locals.var_temp4 * locals.var_temp4)), (((((locals.var_temp2_dn8 + locals.var_temp_dn8) - (((locals.var_temp3_dn8 * locals.var_q1chapinf__blk972) - (locals.var_temp3 * locals.var_q1chapinf__blk972_dn8)) / (locals.var_q1chapinf__blk972 * locals.var_q1chapinf__blk972))) * locals.var_temp4) - (assign41440_e47372 * locals.var_temp4_dn8)) / (locals.var_temp4 * locals.var_temp4)), (((((locals.var_temp2_dn9 + locals.var_temp_dn9) - (((locals.var_temp3_dn9 * locals.var_q1chapinf__blk972) - (locals.var_temp3 * locals.var_q1chapinf__blk972_dn9)) / (locals.var_q1chapinf__blk972 * locals.var_q1chapinf__blk972))) * locals.var_temp4) - (assign41440_e47372 * locals.var_temp4_dn9)) / (locals.var_temp4 * locals.var_temp4)), );
            locals.var_ksi1__blk1072_rv = 0.0;
        }

        if ((locals.var_guard1080 != 0.0) && (locals.var_guard1232 == 0.0)) {
            let assign41450_e47383: f64 = (locals.var_temp1 - locals.var_temp);
            let assign41450_e47386: f64 = (locals.var_temp3 / locals.var_q2chapinf__blk973);
            let assign41450_e47387: f64 = (assign41450_e47383 - assign41450_e47386);
            let assign41450_e47389: f64 = (assign41450_e47387 / locals.var_temp4);
            (locals.var_ksi2__blk1073, locals.var_ksi2__blk1073_dn4, locals.var_ksi2__blk1073_dn6, locals.var_ksi2__blk1073_dn7, locals.var_ksi2__blk1073_dn8, locals.var_ksi2__blk1073_dn9, ) = (assign41450_e47389, (((((locals.var_temp1_dn4 - locals.var_temp_dn4) - (((locals.var_temp3_dn4 * locals.var_q2chapinf__blk973) - (locals.var_temp3 * locals.var_q2chapinf__blk973_dn4)) / (locals.var_q2chapinf__blk973 * locals.var_q2chapinf__blk973))) * locals.var_temp4) - (assign41450_e47387 * locals.var_temp4_dn4)) / (locals.var_temp4 * locals.var_temp4)), (((((locals.var_temp1_dn6 - locals.var_temp_dn6) - (((locals.var_temp3_dn6 * locals.var_q2chapinf__blk973) - (locals.var_temp3 * locals.var_q2chapinf__blk973_dn6)) / (locals.var_q2chapinf__blk973 * locals.var_q2chapinf__blk973))) * locals.var_temp4) - (assign41450_e47387 * locals.var_temp4_dn6)) / (locals.var_temp4 * locals.var_temp4)), (((((locals.var_temp1_dn7 - locals.var_temp_dn7) - (((locals.var_temp3_dn7 * locals.var_q2chapinf__blk973) - (locals.var_temp3 * locals.var_q2chapinf__blk973_dn7)) / (locals.var_q2chapinf__blk973 * locals.var_q2chapinf__blk973))) * locals.var_temp4) - (assign41450_e47387 * locals.var_temp4_dn7)) / (locals.var_temp4 * locals.var_temp4)), (((((locals.var_temp1_dn8 - locals.var_temp_dn8) - (((locals.var_temp3_dn8 * locals.var_q2chapinf__blk973) - (locals.var_temp3 * locals.var_q2chapinf__blk973_dn8)) / (locals.var_q2chapinf__blk973 * locals.var_q2chapinf__blk973))) * locals.var_temp4) - (assign41450_e47387 * locals.var_temp4_dn8)) / (locals.var_temp4 * locals.var_temp4)), (((((locals.var_temp1_dn9 - locals.var_temp_dn9) - (((locals.var_temp3_dn9 * locals.var_q2chapinf__blk973) - (locals.var_temp3 * locals.var_q2chapinf__blk973_dn9)) / (locals.var_q2chapinf__blk973 * locals.var_q2chapinf__blk973))) * locals.var_temp4) - (assign41450_e47387 * locals.var_temp4_dn9)) / (locals.var_temp4 * locals.var_temp4)), );
            locals.var_ksi2__blk1073_rv = 0.0;
        }

        if ((locals.var_guard1080 != 0.0) && (locals.var_guard1232 == 0.0)) {
            let assign41460_e47397: f64 = (-locals.var_q1chapinf__blk972);
            let assign41460_e47400: f64 = (locals.var_ksi1__blk1072 * locals.var_q1chapinf__blk972);
            let assign41460_e47402: f64 = (assign41460_e47400 + locals.var_inv_dinf__blk975);
            let assign41460_e47403: f64 = (assign41460_e47397 * assign41460_e47402);
            (locals.var_inv_k1h1_0__blk1066, locals.var_inv_k1h1_0__blk1066_dn4, locals.var_inv_k1h1_0__blk1066_dn6, locals.var_inv_k1h1_0__blk1066_dn7, locals.var_inv_k1h1_0__blk1066_dn8, locals.var_inv_k1h1_0__blk1066_dn9, ) = (assign41460_e47403, (((-locals.var_q1chapinf__blk972_dn4) * assign41460_e47402) + (assign41460_e47397 * (((locals.var_ksi1__blk1072_dn4 * locals.var_q1chapinf__blk972) + (locals.var_ksi1__blk1072 * locals.var_q1chapinf__blk972_dn4)) + locals.var_inv_dinf__blk975_dn4))), (((-locals.var_q1chapinf__blk972_dn6) * assign41460_e47402) + (assign41460_e47397 * (((locals.var_ksi1__blk1072_dn6 * locals.var_q1chapinf__blk972) + (locals.var_ksi1__blk1072 * locals.var_q1chapinf__blk972_dn6)) + locals.var_inv_dinf__blk975_dn6))), (((-locals.var_q1chapinf__blk972_dn7) * assign41460_e47402) + (assign41460_e47397 * (((locals.var_ksi1__blk1072_dn7 * locals.var_q1chapinf__blk972) + (locals.var_ksi1__blk1072 * locals.var_q1chapinf__blk972_dn7)) + locals.var_inv_dinf__blk975_dn7))), (((-locals.var_q1chapinf__blk972_dn8) * assign41460_e47402) + (assign41460_e47397 * (((locals.var_ksi1__blk1072_dn8 * locals.var_q1chapinf__blk972) + (locals.var_ksi1__blk1072 * locals.var_q1chapinf__blk972_dn8)) + locals.var_inv_dinf__blk975_dn8))), (((-locals.var_q1chapinf__blk972_dn9) * assign41460_e47402) + (assign41460_e47397 * (((locals.var_ksi1__blk1072_dn9 * locals.var_q1chapinf__blk972) + (locals.var_ksi1__blk1072 * locals.var_q1chapinf__blk972_dn9)) + locals.var_inv_dinf__blk975_dn9))), );
            locals.var_inv_k1h1_0__blk1066_rv = 0.0;
        }

        if ((locals.var_guard1080 != 0.0) && (locals.var_guard1232 == 0.0)) {
            let assign41470_e47411: f64 = (-locals.var_q2chapinf__blk973);
            let assign41470_e47414: f64 = (locals.var_ksi2__blk1073 * locals.var_q2chapinf__blk973);
            let assign41470_e47416: f64 = (assign41470_e47414 + locals.var_inv_dinf__blk975);
            let assign41470_e47417: f64 = (assign41470_e47411 * assign41470_e47416);
            (locals.var_inv_k2h2_0__blk1069, locals.var_inv_k2h2_0__blk1069_dn4, locals.var_inv_k2h2_0__blk1069_dn6, locals.var_inv_k2h2_0__blk1069_dn7, locals.var_inv_k2h2_0__blk1069_dn8, locals.var_inv_k2h2_0__blk1069_dn9, ) = (assign41470_e47417, (((-locals.var_q2chapinf__blk973_dn4) * assign41470_e47416) + (assign41470_e47411 * (((locals.var_ksi2__blk1073_dn4 * locals.var_q2chapinf__blk973) + (locals.var_ksi2__blk1073 * locals.var_q2chapinf__blk973_dn4)) + locals.var_inv_dinf__blk975_dn4))), (((-locals.var_q2chapinf__blk973_dn6) * assign41470_e47416) + (assign41470_e47411 * (((locals.var_ksi2__blk1073_dn6 * locals.var_q2chapinf__blk973) + (locals.var_ksi2__blk1073 * locals.var_q2chapinf__blk973_dn6)) + locals.var_inv_dinf__blk975_dn6))), (((-locals.var_q2chapinf__blk973_dn7) * assign41470_e47416) + (assign41470_e47411 * (((locals.var_ksi2__blk1073_dn7 * locals.var_q2chapinf__blk973) + (locals.var_ksi2__blk1073 * locals.var_q2chapinf__blk973_dn7)) + locals.var_inv_dinf__blk975_dn7))), (((-locals.var_q2chapinf__blk973_dn8) * assign41470_e47416) + (assign41470_e47411 * (((locals.var_ksi2__blk1073_dn8 * locals.var_q2chapinf__blk973) + (locals.var_ksi2__blk1073 * locals.var_q2chapinf__blk973_dn8)) + locals.var_inv_dinf__blk975_dn8))), (((-locals.var_q2chapinf__blk973_dn9) * assign41470_e47416) + (assign41470_e47411 * (((locals.var_ksi2__blk1073_dn9 * locals.var_q2chapinf__blk973) + (locals.var_ksi2__blk1073 * locals.var_q2chapinf__blk973_dn9)) + locals.var_inv_dinf__blk975_dn9))), );
            locals.var_inv_k2h2_0__blk1069_rv = 0.0;
        }

        if (locals.var_guard1080 != 0.0) {
            let assign41480_e47423: f64 = (locals.var_inv_k1h1_0__blk1066 * locals.var_hsat__blk1053);
            (locals.var_inv_k1h1__blk1074, locals.var_inv_k1h1__blk1074_dn4, locals.var_inv_k1h1__blk1074_dn6, locals.var_inv_k1h1__blk1074_dn7, locals.var_inv_k1h1__blk1074_dn8, locals.var_inv_k1h1__blk1074_dn9, ) = (assign41480_e47423, ((locals.var_inv_k1h1_0__blk1066_dn4 * locals.var_hsat__blk1053) + (locals.var_inv_k1h1_0__blk1066 * locals.var_hsat__blk1053_dn4)), ((locals.var_inv_k1h1_0__blk1066_dn6 * locals.var_hsat__blk1053) + (locals.var_inv_k1h1_0__blk1066 * locals.var_hsat__blk1053_dn6)), ((locals.var_inv_k1h1_0__blk1066_dn7 * locals.var_hsat__blk1053) + (locals.var_inv_k1h1_0__blk1066 * locals.var_hsat__blk1053_dn7)), ((locals.var_inv_k1h1_0__blk1066_dn8 * locals.var_hsat__blk1053) + (locals.var_inv_k1h1_0__blk1066 * locals.var_hsat__blk1053_dn8)), ((locals.var_inv_k1h1_0__blk1066_dn9 * locals.var_hsat__blk1053) + (locals.var_inv_k1h1_0__blk1066 * locals.var_hsat__blk1053_dn9)), );
            locals.var_inv_k1h1__blk1074_rv = 0.0;
        }

        if (locals.var_guard1080 != 0.0) {
            let assign41490_e47429: f64 = (locals.var_inv_k2h2_0__blk1069 * locals.var_hsat__blk1053);
            (locals.var_inv_k2h2__blk1075, locals.var_inv_k2h2__blk1075_dn4, locals.var_inv_k2h2__blk1075_dn6, locals.var_inv_k2h2__blk1075_dn7, locals.var_inv_k2h2__blk1075_dn8, locals.var_inv_k2h2__blk1075_dn9, ) = (assign41490_e47429, ((locals.var_inv_k2h2_0__blk1069_dn4 * locals.var_hsat__blk1053) + (locals.var_inv_k2h2_0__blk1069 * locals.var_hsat__blk1053_dn4)), ((locals.var_inv_k2h2_0__blk1069_dn6 * locals.var_hsat__blk1053) + (locals.var_inv_k2h2_0__blk1069 * locals.var_hsat__blk1053_dn6)), ((locals.var_inv_k2h2_0__blk1069_dn7 * locals.var_hsat__blk1053) + (locals.var_inv_k2h2_0__blk1069 * locals.var_hsat__blk1053_dn7)), ((locals.var_inv_k2h2_0__blk1069_dn8 * locals.var_hsat__blk1053) + (locals.var_inv_k2h2_0__blk1069 * locals.var_hsat__blk1053_dn8)), ((locals.var_inv_k2h2_0__blk1069_dn9 * locals.var_hsat__blk1053) + (locals.var_inv_k2h2_0__blk1069 * locals.var_hsat__blk1053_dn9)), );
            locals.var_inv_k2h2__blk1075_rv = 0.0;
        }

        if (locals.var_guard1080 != 0.0) {
            let assign41500_e47436: f64 = (locals.var_k1q1d__blk1004 - locals.var_k1q1s__blk939);
            let assign41500_e47437: f64 = (0.5 * assign41500_e47436);
            (locals.var_delta_k1q1__blk1076, locals.var_delta_k1q1__blk1076_dn4, locals.var_delta_k1q1__blk1076_dn6, locals.var_delta_k1q1__blk1076_dn7, locals.var_delta_k1q1__blk1076_dn8, locals.var_delta_k1q1__blk1076_dn9, ) = (assign41500_e47437, (0.5 * (locals.var_k1q1d__blk1004_dn4 - locals.var_k1q1s__blk939_dn4)), (0.5 * (locals.var_k1q1d__blk1004_dn6 - locals.var_k1q1s__blk939_dn6)), (0.5 * (locals.var_k1q1d__blk1004_dn7 - locals.var_k1q1s__blk939_dn7)), (0.5 * (locals.var_k1q1d__blk1004_dn8 - locals.var_k1q1s__blk939_dn8)), (0.5 * (locals.var_k1q1d__blk1004_dn9 - locals.var_k1q1s__blk939_dn9)), );
            locals.var_delta_k1q1__blk1076_rv = 0.0;
        }

        if (locals.var_guard1080 != 0.0) {
            let assign41510_e47444: f64 = (locals.var_k2q2d__blk1005 - locals.var_k2q2s__blk940);
            let assign41510_e47445: f64 = (0.5 * assign41510_e47444);
            (locals.var_delta_k2q2__blk1077, locals.var_delta_k2q2__blk1077_dn4, locals.var_delta_k2q2__blk1077_dn6, locals.var_delta_k2q2__blk1077_dn7, locals.var_delta_k2q2__blk1077_dn8, locals.var_delta_k2q2__blk1077_dn9, ) = (assign41510_e47445, (0.5 * (locals.var_k2q2d__blk1005_dn4 - locals.var_k2q2s__blk940_dn4)), (0.5 * (locals.var_k2q2d__blk1005_dn6 - locals.var_k2q2s__blk940_dn6)), (0.5 * (locals.var_k2q2d__blk1005_dn7 - locals.var_k2q2s__blk940_dn7)), (0.5 * (locals.var_k2q2d__blk1005_dn8 - locals.var_k2q2s__blk940_dn8)), (0.5 * (locals.var_k2q2d__blk1005_dn9 - locals.var_k2q2s__blk940_dn9)), );
            locals.var_delta_k2q2__blk1077_rv = 0.0;
        }

        if (locals.var_guard1080 != 0.0) {
            let assign41520_e47451: f64 = (locals.var_delta_k1q1__blk1076 * locals.var_inv_k1h1__blk1074);
            (locals.var_prod1__blk1078, locals.var_prod1__blk1078_dn4, locals.var_prod1__blk1078_dn6, locals.var_prod1__blk1078_dn7, locals.var_prod1__blk1078_dn8, locals.var_prod1__blk1078_dn9, ) = (assign41520_e47451, ((locals.var_delta_k1q1__blk1076_dn4 * locals.var_inv_k1h1__blk1074) + (locals.var_delta_k1q1__blk1076 * locals.var_inv_k1h1__blk1074_dn4)), ((locals.var_delta_k1q1__blk1076_dn6 * locals.var_inv_k1h1__blk1074) + (locals.var_delta_k1q1__blk1076 * locals.var_inv_k1h1__blk1074_dn6)), ((locals.var_delta_k1q1__blk1076_dn7 * locals.var_inv_k1h1__blk1074) + (locals.var_delta_k1q1__blk1076 * locals.var_inv_k1h1__blk1074_dn7)), ((locals.var_delta_k1q1__blk1076_dn8 * locals.var_inv_k1h1__blk1074) + (locals.var_delta_k1q1__blk1076 * locals.var_inv_k1h1__blk1074_dn8)), ((locals.var_delta_k1q1__blk1076_dn9 * locals.var_inv_k1h1__blk1074) + (locals.var_delta_k1q1__blk1076 * locals.var_inv_k1h1__blk1074_dn9)), );
            locals.var_prod1__blk1078_rv = 0.0;
        }

        if (locals.var_guard1080 != 0.0) {
            let assign41530_e47457: f64 = (locals.var_delta_k2q2__blk1077 * locals.var_inv_k2h2__blk1075);
            (locals.var_prod2__blk1079, locals.var_prod2__blk1079_dn4, locals.var_prod2__blk1079_dn6, locals.var_prod2__blk1079_dn7, locals.var_prod2__blk1079_dn8, locals.var_prod2__blk1079_dn9, ) = (assign41530_e47457, ((locals.var_delta_k2q2__blk1077_dn4 * locals.var_inv_k2h2__blk1075) + (locals.var_delta_k2q2__blk1077 * locals.var_inv_k2h2__blk1075_dn4)), ((locals.var_delta_k2q2__blk1077_dn6 * locals.var_inv_k2h2__blk1075) + (locals.var_delta_k2q2__blk1077 * locals.var_inv_k2h2__blk1075_dn6)), ((locals.var_delta_k2q2__blk1077_dn7 * locals.var_inv_k2h2__blk1075) + (locals.var_delta_k2q2__blk1077 * locals.var_inv_k2h2__blk1075_dn7)), ((locals.var_delta_k2q2__blk1077_dn8 * locals.var_inv_k2h2__blk1075) + (locals.var_delta_k2q2__blk1077 * locals.var_inv_k2h2__blk1075_dn8)), ((locals.var_delta_k2q2__blk1077_dn9 * locals.var_inv_k2h2__blk1075) + (locals.var_delta_k2q2__blk1077 * locals.var_inv_k2h2__blk1075_dn9)), );
            locals.var_prod2__blk1079_rv = 0.0;
        }

        if (locals.var_guard1080 != 0.0) {
            (locals.var_xg20shift_ac, locals.var_xg20shift_ac_dn4, locals.var_xg20shift_ac_dn6, locals.var_xg20shift_ac_dn7, locals.var_xg20shift_ac_dn8, locals.var_xg20shift_ac_dn9, ) = (locals.var_xg20shift__blk900, locals.var_xg20shift__blk900_dn4, locals.var_xg20shift__blk900_dn6, locals.var_xg20shift__blk900_dn7, locals.var_xg20shift__blk900_dn8, locals.var_xg20shift__blk900_dn9, );
            locals.var_xg20shift_ac_rv = 0.0;
            (locals.var_diff_min_ac, locals.var_diff_min_ac_dn4, locals.var_diff_min_ac_dn6, locals.var_diff_min_ac_dn7, locals.var_diff_min_ac_dn8, locals.var_diff_min_ac_dn9, ) = (locals.var_diff_min__blk904, locals.var_diff_min__blk904_dn4, locals.var_diff_min__blk904_dn6, locals.var_diff_min__blk904_dn7, locals.var_diff_min__blk904_dn8, locals.var_diff_min__blk904_dn9, );
            locals.var_diff_min_ac_rv = 0.0;
            (locals.var_a0_ac, locals.var_a0_ac_dn4, locals.var_a0_ac_dn6, locals.var_a0_ac_dn7, locals.var_a0_ac_dn8, locals.var_a0_ac_dn9, ) = (locals.var_a0__blk905, locals.var_a0__blk905_dn4, locals.var_a0__blk905_dn6, locals.var_a0__blk905_dn7, locals.var_a0__blk905_dn8, locals.var_a0__blk905_dn9, );
            locals.var_a0_ac_rv = 0.0;
            (locals.var_inv_k1_ac, locals.var_inv_k1_ac_dn4, locals.var_inv_k1_ac_dn6, locals.var_inv_k1_ac_dn7, locals.var_inv_k1_ac_dn8, locals.var_inv_k1_ac_dn9, ) = (locals.var_inv_k1__blk906, locals.var_inv_k1__blk906_dn4, locals.var_inv_k1__blk906_dn6, locals.var_inv_k1__blk906_dn7, locals.var_inv_k1__blk906_dn8, locals.var_inv_k1__blk906_dn9, );
            locals.var_inv_k1_ac_rv = 0.0;
            (locals.var_inv_k2_ac, locals.var_inv_k2_ac_dn4, locals.var_inv_k2_ac_dn6, locals.var_inv_k2_ac_dn7, locals.var_inv_k2_ac_dn8, locals.var_inv_k2_ac_dn9, ) = (locals.var_inv_k2__blk907, locals.var_inv_k2__blk907_dn4, locals.var_inv_k2__blk907_dn6, locals.var_inv_k2__blk907_dn7, locals.var_inv_k2__blk907_dn8, locals.var_inv_k2__blk907_dn9, );
            locals.var_inv_k2_ac_rv = 0.0;
            (locals.var_keq_ac, locals.var_keq_ac_dn4, locals.var_keq_ac_dn6, locals.var_keq_ac_dn7, locals.var_keq_ac_dn8, locals.var_keq_ac_dn9, ) = (locals.var_keq__blk934, locals.var_keq__blk934_dn4, locals.var_keq__blk934_dn6, locals.var_keq__blk934_dn7, locals.var_keq__blk934_dn8, locals.var_keq__blk934_dn9, );
            locals.var_keq_ac_rv = 0.0;
            (locals.var_dx_wi_ac, locals.var_dx_wi_ac_dn4, locals.var_dx_wi_ac_dn6, locals.var_dx_wi_ac_dn7, locals.var_dx_wi_ac_dn8, locals.var_dx_wi_ac_dn9, ) = (locals.var_dx_wi__blk935, locals.var_dx_wi__blk935_dn4, locals.var_dx_wi__blk935_dn6, locals.var_dx_wi__blk935_dn7, locals.var_dx_wi__blk935_dn8, locals.var_dx_wi__blk935_dn9, );
            locals.var_dx_wi_ac_rv = 0.0;
            (locals.var_csiprime_ac, locals.var_csiprime_ac_dn4, locals.var_csiprime_ac_dn6, locals.var_csiprime_ac_dn7, locals.var_csiprime_ac_dn8, locals.var_csiprime_ac_dn9, ) = (locals.var_csiprime__blk919, locals.var_csiprime__blk919_dn4, locals.var_csiprime__blk919_dn6, locals.var_csiprime__blk919_dn7, locals.var_csiprime__blk919_dn8, locals.var_csiprime__blk919_dn9, );
            locals.var_csiprime_ac_rv = 0.0;
            (locals.var_dx_wi_1d_ac, locals.var_dx_wi_1d_ac_dn4, locals.var_dx_wi_1d_ac_dn6, locals.var_dx_wi_1d_ac_dn7, locals.var_dx_wi_1d_ac_dn8, locals.var_dx_wi_1d_ac_dn9, ) = (locals.var_dx_wi_1d__blk918, locals.var_dx_wi_1d__blk918_dn4, locals.var_dx_wi_1d__blk918_dn6, locals.var_dx_wi_1d__blk918_dn7, locals.var_dx_wi_1d__blk918_dn8, locals.var_dx_wi_1d__blk918_dn9, );
            locals.var_dx_wi_1d_ac_rv = 0.0;
            (locals.var_dleff_ac, locals.var_dleff_ac_dn4, locals.var_dleff_ac_dn6, locals.var_dleff_ac_dn7, locals.var_dleff_ac_dn8, locals.var_dleff_ac_dn9, ) = (locals.var_dleff__blk922, locals.var_dleff__blk922_dn4, locals.var_dleff__blk922_dn6, locals.var_dleff__blk922_dn7, locals.var_dleff__blk922_dn8, locals.var_dleff__blk922_dn9, );
            locals.var_dleff_ac_rv = 0.0;
            (locals.var_xedge_ac, locals.var_xedge_ac_dn4, locals.var_xedge_ac_dn6, locals.var_xedge_ac_dn7, locals.var_xedge_ac_dn8, locals.var_xedge_ac_dn9, ) = (locals.var_xedge__blk923, locals.var_xedge__blk923_dn4, locals.var_xedge__blk923_dn6, locals.var_xedge__blk923_dn7, locals.var_xedge__blk923_dn8, locals.var_xedge__blk923_dn9, );
            locals.var_xedge_ac_rv = 0.0;
            (locals.var_sce1_ac, locals.var_sce1_ac_dn4, locals.var_sce1_ac_dn6, locals.var_sce1_ac_dn7, locals.var_sce1_ac_dn8, locals.var_sce1_ac_dn9, ) = (locals.var_sce1__blk924, locals.var_sce1__blk924_dn4, locals.var_sce1__blk924_dn6, locals.var_sce1__blk924_dn7, locals.var_sce1__blk924_dn8, locals.var_sce1__blk924_dn9, );
            locals.var_sce1_ac_rv = 0.0;
            (locals.var_sce2_ac, locals.var_sce2_ac_dn4, locals.var_sce2_ac_dn6, locals.var_sce2_ac_dn7, locals.var_sce2_ac_dn8, locals.var_sce2_ac_dn9, ) = (locals.var_sce2__blk925, locals.var_sce2__blk925_dn4, locals.var_sce2__blk925_dn6, locals.var_sce2__blk925_dn7, locals.var_sce2__blk925_dn8, locals.var_sce2__blk925_dn9, );
            locals.var_sce2_ac_rv = 0.0;
            (locals.var_dxg1_dibl_ac, locals.var_dxg1_dibl_ac_dn4, locals.var_dxg1_dibl_ac_dn6, locals.var_dxg1_dibl_ac_dn7, locals.var_dxg1_dibl_ac_dn8, locals.var_dxg1_dibl_ac_dn9, ) = (locals.var_dxg1_dibl__blk926, locals.var_dxg1_dibl__blk926_dn4, locals.var_dxg1_dibl__blk926_dn6, locals.var_dxg1_dibl__blk926_dn7, locals.var_dxg1_dibl__blk926_dn8, locals.var_dxg1_dibl__blk926_dn9, );
            locals.var_dxg1_dibl_ac_rv = 0.0;
            (locals.var_xg2_ac, locals.var_xg2_ac_dn4, locals.var_xg2_ac_dn6, locals.var_xg2_ac_dn7, locals.var_xg2_ac_dn8, locals.var_xg2_ac_dn9, ) = (locals.var_xg2__blk929, locals.var_xg2__blk929_dn4, locals.var_xg2__blk929_dn6, locals.var_xg2__blk929_dn7, locals.var_xg2__blk929_dn8, locals.var_xg2__blk929_dn9, );
            locals.var_xg2_ac_rv = 0.0;
        }

    }

    pub(super) fn stamp_reactive_block_60(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        if (locals.var_guard1080 != 0.0) {
            (locals.var_xg2x_ac, locals.var_xg2x_ac_dn4, locals.var_xg2x_ac_dn6, locals.var_xg2x_ac_dn7, locals.var_xg2x_ac_dn8, locals.var_xg2x_ac_dn9, ) = (locals.var_xg2x__blk931, locals.var_xg2x__blk931_dn4, locals.var_xg2x__blk931_dn6, locals.var_xg2x__blk931_dn7, locals.var_xg2x__blk931_dn8, locals.var_xg2x__blk931_dn9, );
            locals.var_xg2x_ac_rv = 0.0;
            (locals.var_k1_ac, locals.var_k1_ac_dn4, locals.var_k1_ac_dn6, locals.var_k1_ac_dn7, locals.var_k1_ac_dn8, locals.var_k1_ac_dn9, ) = (locals.var_k1__blk932, locals.var_k1__blk932_dn4, locals.var_k1__blk932_dn6, locals.var_k1__blk932_dn7, locals.var_k1__blk932_dn8, locals.var_k1__blk932_dn9, );
            locals.var_k1_ac_rv = 0.0;
            (locals.var_k2_ac, locals.var_k2_ac_dn4, locals.var_k2_ac_dn6, locals.var_k2_ac_dn7, locals.var_k2_ac_dn8, locals.var_k2_ac_dn9, ) = (locals.var_k2__blk933, locals.var_k2__blk933_dn4, locals.var_k2__blk933_dn6, locals.var_k2__blk933_dn7, locals.var_k2__blk933_dn8, locals.var_k2__blk933_dn9, );
            locals.var_k2_ac_rv = 0.0;
            (locals.var_k1q1s_ac, locals.var_k1q1s_ac_dn4, locals.var_k1q1s_ac_dn6, locals.var_k1q1s_ac_dn7, locals.var_k1q1s_ac_dn8, locals.var_k1q1s_ac_dn9, ) = (locals.var_k1q1s__blk939, locals.var_k1q1s__blk939_dn4, locals.var_k1q1s__blk939_dn6, locals.var_k1q1s__blk939_dn7, locals.var_k1q1s__blk939_dn8, locals.var_k1q1s__blk939_dn9, );
            locals.var_k1q1s_ac_rv = 0.0;
            (locals.var_k2q2s_ac, locals.var_k2q2s_ac_dn4, locals.var_k2q2s_ac_dn6, locals.var_k2q2s_ac_dn7, locals.var_k2q2s_ac_dn8, locals.var_k2q2s_ac_dn9, ) = (locals.var_k2q2s__blk940, locals.var_k2q2s__blk940_dn4, locals.var_k2q2s__blk940_dn6, locals.var_k2q2s__blk940_dn7, locals.var_k2q2s__blk940_dn8, locals.var_k2q2s__blk940_dn9, );
            locals.var_k2q2s_ac_rv = 0.0;
            (locals.var_xdrifts_ac, locals.var_xdrifts_ac_dn4, locals.var_xdrifts_ac_dn6, locals.var_xdrifts_ac_dn7, locals.var_xdrifts_ac_dn8, locals.var_xdrifts_ac_dn9, ) = (locals.var_xdrifts__blk951, locals.var_xdrifts__blk951_dn4, locals.var_xdrifts__blk951_dn6, locals.var_xdrifts__blk951_dn7, locals.var_xdrifts__blk951_dn8, locals.var_xdrifts__blk951_dn9, );
            locals.var_xdrifts_ac_rv = 0.0;
            (locals.var_k1q1d_ac, locals.var_k1q1d_ac_dn4, locals.var_k1q1d_ac_dn6, locals.var_k1q1d_ac_dn7, locals.var_k1q1d_ac_dn8, locals.var_k1q1d_ac_dn9, ) = (locals.var_k1q1d__blk1004, locals.var_k1q1d__blk1004_dn4, locals.var_k1q1d__blk1004_dn6, locals.var_k1q1d__blk1004_dn7, locals.var_k1q1d__blk1004_dn8, locals.var_k1q1d__blk1004_dn9, );
            locals.var_k1q1d_ac_rv = 0.0;
            (locals.var_k2q2d_ac, locals.var_k2q2d_ac_dn4, locals.var_k2q2d_ac_dn6, locals.var_k2q2d_ac_dn7, locals.var_k2q2d_ac_dn8, locals.var_k2q2d_ac_dn9, ) = (locals.var_k2q2d__blk1005, locals.var_k2q2d__blk1005_dn4, locals.var_k2q2d__blk1005_dn6, locals.var_k2q2d__blk1005_dn7, locals.var_k2q2d__blk1005_dn8, locals.var_k2q2d__blk1005_dn9, );
            locals.var_k2q2d_ac_rv = 0.0;
            (locals.var_xdriftd_ac, locals.var_xdriftd_ac_dn4, locals.var_xdriftd_ac_dn6, locals.var_xdriftd_ac_dn7, locals.var_xdriftd_ac_dn8, locals.var_xdriftd_ac_dn9, ) = (locals.var_xdriftd__blk1015, locals.var_xdriftd__blk1015_dn4, locals.var_xdriftd__blk1015_dn6, locals.var_xdriftd__blk1015_dn7, locals.var_xdriftd__blk1015_dn8, locals.var_xdriftd__blk1015_dn9, );
            locals.var_xdriftd_ac_rv = 0.0;
            (locals.var_qim_ac, locals.var_qim_ac_dn4, locals.var_qim_ac_dn6, locals.var_qim_ac_dn7, locals.var_qim_ac_dn8, locals.var_qim_ac_dn9, ) = (locals.var_qim__blk1016, locals.var_qim__blk1016_dn4, locals.var_qim__blk1016_dn6, locals.var_qim__blk1016_dn7, locals.var_qim__blk1016_dn8, locals.var_qim__blk1016_dn9, );
            locals.var_qim_ac_rv = 0.0;
            (locals.var_ratio_pd_ac, locals.var_ratio_pd_ac_dn4, locals.var_ratio_pd_ac_dn6, locals.var_ratio_pd_ac_dn7, locals.var_ratio_pd_ac_dn8, locals.var_ratio_pd_ac_dn9, ) = (locals.var_ratio_pd__blk1020, locals.var_ratio_pd__blk1020_dn4, locals.var_ratio_pd__blk1020_dn6, locals.var_ratio_pd__blk1020_dn7, locals.var_ratio_pd__blk1020_dn8, locals.var_ratio_pd__blk1020_dn9, );
            locals.var_ratio_pd_ac_rv = 0.0;
            (locals.var_qi1m_ac, locals.var_qi1m_ac_dn4, locals.var_qi1m_ac_dn6, locals.var_qi1m_ac_dn7, locals.var_qi1m_ac_dn8, locals.var_qi1m_ac_dn9, ) = (locals.var_qi1m__blk1029, locals.var_qi1m__blk1029_dn4, locals.var_qi1m__blk1029_dn6, locals.var_qi1m__blk1029_dn7, locals.var_qi1m__blk1029_dn8, locals.var_qi1m__blk1029_dn9, );
            locals.var_qi1m_ac_rv = 0.0;
            (locals.var_qi2m_ac, locals.var_qi2m_ac_dn4, locals.var_qi2m_ac_dn6, locals.var_qi2m_ac_dn7, locals.var_qi2m_ac_dn8, locals.var_qi2m_ac_dn9, ) = (locals.var_qi2m__blk1030, locals.var_qi2m__blk1030_dn4, locals.var_qi2m__blk1030_dn6, locals.var_qi2m__blk1030_dn7, locals.var_qi2m__blk1030_dn8, locals.var_qi2m__blk1030_dn9, );
            locals.var_qi2m_ac_rv = 0.0;
            (locals.var_zsat_ac, locals.var_zsat_ac_dn4, locals.var_zsat_ac_dn6, locals.var_zsat_ac_dn7, locals.var_zsat_ac_dn8, locals.var_zsat_ac_dn9, ) = (locals.var_zsat__blk1051, locals.var_zsat__blk1051_dn4, locals.var_zsat__blk1051_dn6, locals.var_zsat__blk1051_dn7, locals.var_zsat__blk1051_dn8, locals.var_zsat__blk1051_dn9, );
            locals.var_zsat_ac_rv = 0.0;
            (locals.var_qmfact1_ac, locals.var_qmfact1_ac_dn4, locals.var_qmfact1_ac_dn6, locals.var_qmfact1_ac_dn7, locals.var_qmfact1_ac_dn8, locals.var_qmfact1_ac_dn9, ) = (locals.var_qmfact1__blk1054, locals.var_qmfact1__blk1054_dn4, locals.var_qmfact1__blk1054_dn6, locals.var_qmfact1__blk1054_dn7, locals.var_qmfact1__blk1054_dn8, locals.var_qmfact1__blk1054_dn9, );
            locals.var_qmfact1_ac_rv = 0.0;
            (locals.var_qmfact2_ac, locals.var_qmfact2_ac_dn4, locals.var_qmfact2_ac_dn6, locals.var_qmfact2_ac_dn7, locals.var_qmfact2_ac_dn8, locals.var_qmfact2_ac_dn9, ) = (locals.var_qmfact2__blk1055, locals.var_qmfact2__blk1055_dn4, locals.var_qmfact2__blk1055_dn6, locals.var_qmfact2__blk1055_dn7, locals.var_qmfact2__blk1055_dn8, locals.var_qmfact2__blk1055_dn9, );
            locals.var_qmfact2_ac_rv = 0.0;
            (locals.var_delta_k1q1_ac, locals.var_delta_k1q1_ac_dn4, locals.var_delta_k1q1_ac_dn6, locals.var_delta_k1q1_ac_dn7, locals.var_delta_k1q1_ac_dn8, locals.var_delta_k1q1_ac_dn9, ) = (locals.var_delta_k1q1__blk1076, locals.var_delta_k1q1__blk1076_dn4, locals.var_delta_k1q1__blk1076_dn6, locals.var_delta_k1q1__blk1076_dn7, locals.var_delta_k1q1__blk1076_dn8, locals.var_delta_k1q1__blk1076_dn9, );
            locals.var_delta_k1q1_ac_rv = 0.0;
            (locals.var_delta_k2q2_ac, locals.var_delta_k2q2_ac_dn4, locals.var_delta_k2q2_ac_dn6, locals.var_delta_k2q2_ac_dn7, locals.var_delta_k2q2_ac_dn8, locals.var_delta_k2q2_ac_dn9, ) = (locals.var_delta_k2q2__blk1077, locals.var_delta_k2q2__blk1077_dn4, locals.var_delta_k2q2__blk1077_dn6, locals.var_delta_k2q2__blk1077_dn7, locals.var_delta_k2q2__blk1077_dn8, locals.var_delta_k2q2__blk1077_dn9, );
            locals.var_delta_k2q2_ac_rv = 0.0;
            (locals.var_prod1_ac, locals.var_prod1_ac_dn4, locals.var_prod1_ac_dn6, locals.var_prod1_ac_dn7, locals.var_prod1_ac_dn8, locals.var_prod1_ac_dn9, ) = (locals.var_prod1__blk1078, locals.var_prod1__blk1078_dn4, locals.var_prod1__blk1078_dn6, locals.var_prod1__blk1078_dn7, locals.var_prod1__blk1078_dn8, locals.var_prod1__blk1078_dn9, );
            locals.var_prod1_ac_rv = 0.0;
            (locals.var_prod2_ac, locals.var_prod2_ac_dn4, locals.var_prod2_ac_dn6, locals.var_prod2_ac_dn7, locals.var_prod2_ac_dn8, locals.var_prod2_ac_dn9, ) = (locals.var_prod2__blk1079, locals.var_prod2__blk1079_dn4, locals.var_prod2__blk1079_dn6, locals.var_prod2__blk1079_dn7, locals.var_prod2__blk1079_dn8, locals.var_prod2__blk1079_dn9, );
            locals.var_prod2_ac_rv = 0.0;
        }

        if (locals.var_guard1080 == 0.0) {
            (locals.var_xg20shift_ac, locals.var_xg20shift_ac_dn4, locals.var_xg20shift_ac_dn6, locals.var_xg20shift_ac_dn7, locals.var_xg20shift_ac_dn8, locals.var_xg20shift_ac_dn9, ) = (locals.var_xg20shift_dc, locals.var_xg20shift_dc_dn4, locals.var_xg20shift_dc_dn6, locals.var_xg20shift_dc_dn7, locals.var_xg20shift_dc_dn8, locals.var_xg20shift_dc_dn9, );
            locals.var_xg20shift_ac_rv = 0.0;
            (locals.var_diff_min_ac, locals.var_diff_min_ac_dn4, locals.var_diff_min_ac_dn6, locals.var_diff_min_ac_dn7, locals.var_diff_min_ac_dn8, locals.var_diff_min_ac_dn9, ) = (locals.var_diff_min_dc, locals.var_diff_min_dc_dn4, locals.var_diff_min_dc_dn6, locals.var_diff_min_dc_dn7, locals.var_diff_min_dc_dn8, locals.var_diff_min_dc_dn9, );
            locals.var_diff_min_ac_rv = 0.0;
            (locals.var_a0_ac, locals.var_a0_ac_dn4, locals.var_a0_ac_dn6, locals.var_a0_ac_dn7, locals.var_a0_ac_dn8, locals.var_a0_ac_dn9, ) = (locals.var_a0_dc, locals.var_a0_dc_dn4, locals.var_a0_dc_dn6, locals.var_a0_dc_dn7, locals.var_a0_dc_dn8, locals.var_a0_dc_dn9, );
            locals.var_a0_ac_rv = 0.0;
            (locals.var_inv_k1_ac, locals.var_inv_k1_ac_dn4, locals.var_inv_k1_ac_dn6, locals.var_inv_k1_ac_dn7, locals.var_inv_k1_ac_dn8, locals.var_inv_k1_ac_dn9, ) = (locals.var_inv_k1_dc, locals.var_inv_k1_dc_dn4, locals.var_inv_k1_dc_dn6, locals.var_inv_k1_dc_dn7, locals.var_inv_k1_dc_dn8, locals.var_inv_k1_dc_dn9, );
            locals.var_inv_k1_ac_rv = 0.0;
            (locals.var_inv_k2_ac, locals.var_inv_k2_ac_dn4, locals.var_inv_k2_ac_dn6, locals.var_inv_k2_ac_dn7, locals.var_inv_k2_ac_dn8, locals.var_inv_k2_ac_dn9, ) = (locals.var_inv_k2_dc, locals.var_inv_k2_dc_dn4, locals.var_inv_k2_dc_dn6, locals.var_inv_k2_dc_dn7, locals.var_inv_k2_dc_dn8, locals.var_inv_k2_dc_dn9, );
            locals.var_inv_k2_ac_rv = 0.0;
            (locals.var_keq_ac, locals.var_keq_ac_dn4, locals.var_keq_ac_dn6, locals.var_keq_ac_dn7, locals.var_keq_ac_dn8, locals.var_keq_ac_dn9, ) = (locals.var_keq_dc, locals.var_keq_dc_dn4, locals.var_keq_dc_dn6, locals.var_keq_dc_dn7, locals.var_keq_dc_dn8, locals.var_keq_dc_dn9, );
            locals.var_keq_ac_rv = 0.0;
            (locals.var_dx_wi_ac, locals.var_dx_wi_ac_dn4, locals.var_dx_wi_ac_dn6, locals.var_dx_wi_ac_dn7, locals.var_dx_wi_ac_dn8, locals.var_dx_wi_ac_dn9, ) = (locals.var_dx_wi_dc, locals.var_dx_wi_dc_dn4, locals.var_dx_wi_dc_dn6, locals.var_dx_wi_dc_dn7, locals.var_dx_wi_dc_dn8, locals.var_dx_wi_dc_dn9, );
            locals.var_dx_wi_ac_rv = 0.0;
            (locals.var_csiprime_ac, locals.var_csiprime_ac_dn4, locals.var_csiprime_ac_dn6, locals.var_csiprime_ac_dn7, locals.var_csiprime_ac_dn8, locals.var_csiprime_ac_dn9, ) = (locals.var_csiprime_dc, locals.var_csiprime_dc_dn4, locals.var_csiprime_dc_dn6, locals.var_csiprime_dc_dn7, locals.var_csiprime_dc_dn8, locals.var_csiprime_dc_dn9, );
            locals.var_csiprime_ac_rv = 0.0;
            (locals.var_dx_wi_1d_ac, locals.var_dx_wi_1d_ac_dn4, locals.var_dx_wi_1d_ac_dn6, locals.var_dx_wi_1d_ac_dn7, locals.var_dx_wi_1d_ac_dn8, locals.var_dx_wi_1d_ac_dn9, ) = (locals.var_dx_wi_1d_dc, locals.var_dx_wi_1d_dc_dn4, locals.var_dx_wi_1d_dc_dn6, locals.var_dx_wi_1d_dc_dn7, locals.var_dx_wi_1d_dc_dn8, locals.var_dx_wi_1d_dc_dn9, );
            locals.var_dx_wi_1d_ac_rv = 0.0;
            (locals.var_dleff_ac, locals.var_dleff_ac_dn4, locals.var_dleff_ac_dn6, locals.var_dleff_ac_dn7, locals.var_dleff_ac_dn8, locals.var_dleff_ac_dn9, ) = (locals.var_dleff_dc, locals.var_dleff_dc_dn4, locals.var_dleff_dc_dn6, locals.var_dleff_dc_dn7, locals.var_dleff_dc_dn8, locals.var_dleff_dc_dn9, );
            locals.var_dleff_ac_rv = 0.0;
            (locals.var_xedge_ac, locals.var_xedge_ac_dn4, locals.var_xedge_ac_dn6, locals.var_xedge_ac_dn7, locals.var_xedge_ac_dn8, locals.var_xedge_ac_dn9, ) = (locals.var_xedge_dc, locals.var_xedge_dc_dn4, locals.var_xedge_dc_dn6, locals.var_xedge_dc_dn7, locals.var_xedge_dc_dn8, locals.var_xedge_dc_dn9, );
            locals.var_xedge_ac_rv = 0.0;
            (locals.var_sce1_ac, locals.var_sce1_ac_dn4, locals.var_sce1_ac_dn6, locals.var_sce1_ac_dn7, locals.var_sce1_ac_dn8, locals.var_sce1_ac_dn9, ) = (locals.var_sce1_dc, locals.var_sce1_dc_dn4, locals.var_sce1_dc_dn6, locals.var_sce1_dc_dn7, locals.var_sce1_dc_dn8, locals.var_sce1_dc_dn9, );
            locals.var_sce1_ac_rv = 0.0;
            (locals.var_sce2_ac, locals.var_sce2_ac_dn4, locals.var_sce2_ac_dn6, locals.var_sce2_ac_dn7, locals.var_sce2_ac_dn8, locals.var_sce2_ac_dn9, ) = (locals.var_sce2_dc, locals.var_sce2_dc_dn4, locals.var_sce2_dc_dn6, locals.var_sce2_dc_dn7, locals.var_sce2_dc_dn8, locals.var_sce2_dc_dn9, );
            locals.var_sce2_ac_rv = 0.0;
            (locals.var_dxg1_dibl_ac, locals.var_dxg1_dibl_ac_dn4, locals.var_dxg1_dibl_ac_dn6, locals.var_dxg1_dibl_ac_dn7, locals.var_dxg1_dibl_ac_dn8, locals.var_dxg1_dibl_ac_dn9, ) = (locals.var_dxg1_dibl_dc, locals.var_dxg1_dibl_dc_dn4, locals.var_dxg1_dibl_dc_dn6, locals.var_dxg1_dibl_dc_dn7, locals.var_dxg1_dibl_dc_dn8, locals.var_dxg1_dibl_dc_dn9, );
            locals.var_dxg1_dibl_ac_rv = 0.0;
            (locals.var_xg2_ac, locals.var_xg2_ac_dn4, locals.var_xg2_ac_dn6, locals.var_xg2_ac_dn7, locals.var_xg2_ac_dn8, locals.var_xg2_ac_dn9, ) = (locals.var_xg2_dc, locals.var_xg2_dc_dn4, locals.var_xg2_dc_dn6, locals.var_xg2_dc_dn7, locals.var_xg2_dc_dn8, locals.var_xg2_dc_dn9, );
            locals.var_xg2_ac_rv = 0.0;
            (locals.var_xg2x_ac, locals.var_xg2x_ac_dn4, locals.var_xg2x_ac_dn6, locals.var_xg2x_ac_dn7, locals.var_xg2x_ac_dn8, locals.var_xg2x_ac_dn9, ) = (locals.var_xg2x_dc, locals.var_xg2x_dc_dn4, locals.var_xg2x_dc_dn6, locals.var_xg2x_dc_dn7, locals.var_xg2x_dc_dn8, locals.var_xg2x_dc_dn9, );
            locals.var_xg2x_ac_rv = 0.0;
            (locals.var_k1_ac, locals.var_k1_ac_dn4, locals.var_k1_ac_dn6, locals.var_k1_ac_dn7, locals.var_k1_ac_dn8, locals.var_k1_ac_dn9, ) = (locals.var_k1_dc, locals.var_k1_dc_dn4, locals.var_k1_dc_dn6, locals.var_k1_dc_dn7, locals.var_k1_dc_dn8, locals.var_k1_dc_dn9, );
            locals.var_k1_ac_rv = 0.0;
            (locals.var_k2_ac, locals.var_k2_ac_dn4, locals.var_k2_ac_dn6, locals.var_k2_ac_dn7, locals.var_k2_ac_dn8, locals.var_k2_ac_dn9, ) = (locals.var_k2_dc, locals.var_k2_dc_dn4, locals.var_k2_dc_dn6, locals.var_k2_dc_dn7, locals.var_k2_dc_dn8, locals.var_k2_dc_dn9, );
            locals.var_k2_ac_rv = 0.0;
            (locals.var_k1q1s_ac, locals.var_k1q1s_ac_dn4, locals.var_k1q1s_ac_dn6, locals.var_k1q1s_ac_dn7, locals.var_k1q1s_ac_dn8, locals.var_k1q1s_ac_dn9, ) = (locals.var_k1q1s_dc, locals.var_k1q1s_dc_dn4, locals.var_k1q1s_dc_dn6, locals.var_k1q1s_dc_dn7, locals.var_k1q1s_dc_dn8, locals.var_k1q1s_dc_dn9, );
            locals.var_k1q1s_ac_rv = 0.0;
            (locals.var_k2q2s_ac, locals.var_k2q2s_ac_dn4, locals.var_k2q2s_ac_dn6, locals.var_k2q2s_ac_dn7, locals.var_k2q2s_ac_dn8, locals.var_k2q2s_ac_dn9, ) = (locals.var_k2q2s_dc, locals.var_k2q2s_dc_dn4, locals.var_k2q2s_dc_dn6, locals.var_k2q2s_dc_dn7, locals.var_k2q2s_dc_dn8, locals.var_k2q2s_dc_dn9, );
            locals.var_k2q2s_ac_rv = 0.0;
            (locals.var_xdrifts_ac, locals.var_xdrifts_ac_dn4, locals.var_xdrifts_ac_dn6, locals.var_xdrifts_ac_dn7, locals.var_xdrifts_ac_dn8, locals.var_xdrifts_ac_dn9, ) = (locals.var_xdrifts_dc, locals.var_xdrifts_dc_dn4, locals.var_xdrifts_dc_dn6, locals.var_xdrifts_dc_dn7, locals.var_xdrifts_dc_dn8, locals.var_xdrifts_dc_dn9, );
            locals.var_xdrifts_ac_rv = 0.0;
            (locals.var_k1q1d_ac, locals.var_k1q1d_ac_dn4, locals.var_k1q1d_ac_dn6, locals.var_k1q1d_ac_dn7, locals.var_k1q1d_ac_dn8, locals.var_k1q1d_ac_dn9, ) = (locals.var_k1q1d_dc, locals.var_k1q1d_dc_dn4, locals.var_k1q1d_dc_dn6, locals.var_k1q1d_dc_dn7, locals.var_k1q1d_dc_dn8, locals.var_k1q1d_dc_dn9, );
            locals.var_k1q1d_ac_rv = 0.0;
            (locals.var_k2q2d_ac, locals.var_k2q2d_ac_dn4, locals.var_k2q2d_ac_dn6, locals.var_k2q2d_ac_dn7, locals.var_k2q2d_ac_dn8, locals.var_k2q2d_ac_dn9, ) = (locals.var_k2q2d_dc, locals.var_k2q2d_dc_dn4, locals.var_k2q2d_dc_dn6, locals.var_k2q2d_dc_dn7, locals.var_k2q2d_dc_dn8, locals.var_k2q2d_dc_dn9, );
            locals.var_k2q2d_ac_rv = 0.0;
            (locals.var_xdriftd_ac, locals.var_xdriftd_ac_dn4, locals.var_xdriftd_ac_dn6, locals.var_xdriftd_ac_dn7, locals.var_xdriftd_ac_dn8, locals.var_xdriftd_ac_dn9, ) = (locals.var_xdriftd_dc, locals.var_xdriftd_dc_dn4, locals.var_xdriftd_dc_dn6, locals.var_xdriftd_dc_dn7, locals.var_xdriftd_dc_dn8, locals.var_xdriftd_dc_dn9, );
            locals.var_xdriftd_ac_rv = 0.0;
            (locals.var_qim_ac, locals.var_qim_ac_dn4, locals.var_qim_ac_dn6, locals.var_qim_ac_dn7, locals.var_qim_ac_dn8, locals.var_qim_ac_dn9, ) = (locals.var_qim_dc, locals.var_qim_dc_dn4, locals.var_qim_dc_dn6, locals.var_qim_dc_dn7, locals.var_qim_dc_dn8, locals.var_qim_dc_dn9, );
            locals.var_qim_ac_rv = 0.0;
            (locals.var_ratio_pd_ac, locals.var_ratio_pd_ac_dn4, locals.var_ratio_pd_ac_dn6, locals.var_ratio_pd_ac_dn7, locals.var_ratio_pd_ac_dn8, locals.var_ratio_pd_ac_dn9, ) = (locals.var_ratio_pd_dc, locals.var_ratio_pd_dc_dn4, locals.var_ratio_pd_dc_dn6, locals.var_ratio_pd_dc_dn7, locals.var_ratio_pd_dc_dn8, locals.var_ratio_pd_dc_dn9, );
            locals.var_ratio_pd_ac_rv = 0.0;
            (locals.var_qi1m_ac, locals.var_qi1m_ac_dn4, locals.var_qi1m_ac_dn6, locals.var_qi1m_ac_dn7, locals.var_qi1m_ac_dn8, locals.var_qi1m_ac_dn9, ) = (locals.var_qi1m_dc, locals.var_qi1m_dc_dn4, locals.var_qi1m_dc_dn6, locals.var_qi1m_dc_dn7, locals.var_qi1m_dc_dn8, locals.var_qi1m_dc_dn9, );
            locals.var_qi1m_ac_rv = 0.0;
            (locals.var_qi2m_ac, locals.var_qi2m_ac_dn4, locals.var_qi2m_ac_dn6, locals.var_qi2m_ac_dn7, locals.var_qi2m_ac_dn8, locals.var_qi2m_ac_dn9, ) = (locals.var_qi2m_dc, locals.var_qi2m_dc_dn4, locals.var_qi2m_dc_dn6, locals.var_qi2m_dc_dn7, locals.var_qi2m_dc_dn8, locals.var_qi2m_dc_dn9, );
            locals.var_qi2m_ac_rv = 0.0;
            (locals.var_zsat_ac, locals.var_zsat_ac_dn4, locals.var_zsat_ac_dn6, locals.var_zsat_ac_dn7, locals.var_zsat_ac_dn8, locals.var_zsat_ac_dn9, ) = (locals.var_zsat_dc, locals.var_zsat_dc_dn4, locals.var_zsat_dc_dn6, locals.var_zsat_dc_dn7, locals.var_zsat_dc_dn8, locals.var_zsat_dc_dn9, );
            locals.var_zsat_ac_rv = 0.0;
            (locals.var_qmfact1_ac, locals.var_qmfact1_ac_dn4, locals.var_qmfact1_ac_dn6, locals.var_qmfact1_ac_dn7, locals.var_qmfact1_ac_dn8, locals.var_qmfact1_ac_dn9, ) = (locals.var_qmfact1_dc, locals.var_qmfact1_dc_dn4, locals.var_qmfact1_dc_dn6, locals.var_qmfact1_dc_dn7, locals.var_qmfact1_dc_dn8, locals.var_qmfact1_dc_dn9, );
            locals.var_qmfact1_ac_rv = 0.0;
            (locals.var_qmfact2_ac, locals.var_qmfact2_ac_dn4, locals.var_qmfact2_ac_dn6, locals.var_qmfact2_ac_dn7, locals.var_qmfact2_ac_dn8, locals.var_qmfact2_ac_dn9, ) = (locals.var_qmfact2_dc, locals.var_qmfact2_dc_dn4, locals.var_qmfact2_dc_dn6, locals.var_qmfact2_dc_dn7, locals.var_qmfact2_dc_dn8, locals.var_qmfact2_dc_dn9, );
            locals.var_qmfact2_ac_rv = 0.0;
            (locals.var_delta_k1q1_ac, locals.var_delta_k1q1_ac_dn4, locals.var_delta_k1q1_ac_dn6, locals.var_delta_k1q1_ac_dn7, locals.var_delta_k1q1_ac_dn8, locals.var_delta_k1q1_ac_dn9, ) = (locals.var_delta_k1q1_dc, locals.var_delta_k1q1_dc_dn4, locals.var_delta_k1q1_dc_dn6, locals.var_delta_k1q1_dc_dn7, locals.var_delta_k1q1_dc_dn8, locals.var_delta_k1q1_dc_dn9, );
            locals.var_delta_k1q1_ac_rv = 0.0;
            (locals.var_delta_k2q2_ac, locals.var_delta_k2q2_ac_dn4, locals.var_delta_k2q2_ac_dn6, locals.var_delta_k2q2_ac_dn7, locals.var_delta_k2q2_ac_dn8, locals.var_delta_k2q2_ac_dn9, ) = (locals.var_delta_k2q2_dc, locals.var_delta_k2q2_dc_dn4, locals.var_delta_k2q2_dc_dn6, locals.var_delta_k2q2_dc_dn7, locals.var_delta_k2q2_dc_dn8, locals.var_delta_k2q2_dc_dn9, );
            locals.var_delta_k2q2_ac_rv = 0.0;
            (locals.var_prod1_ac, locals.var_prod1_ac_dn4, locals.var_prod1_ac_dn6, locals.var_prod1_ac_dn7, locals.var_prod1_ac_dn8, locals.var_prod1_ac_dn9, ) = (locals.var_prod1_dc, locals.var_prod1_dc_dn4, locals.var_prod1_dc_dn6, locals.var_prod1_dc_dn7, locals.var_prod1_dc_dn8, locals.var_prod1_dc_dn9, );
            locals.var_prod1_ac_rv = 0.0;
            (locals.var_prod2_ac, locals.var_prod2_ac_dn4, locals.var_prod2_ac_dn6, locals.var_prod2_ac_dn7, locals.var_prod2_ac_dn8, locals.var_prod2_ac_dn9, ) = (locals.var_prod2_dc, locals.var_prod2_dc_dn4, locals.var_prod2_dc_dn6, locals.var_prod2_dc_dn7, locals.var_prod2_dc_dn8, locals.var_prod2_dc_dn9, );
            locals.var_prod2_ac_rv = 0.0;
        }

        let assign42240_e47778: f64 = (locals.var_dx_wi_1d_ac - locals.var_dx_wi_ac);
        let assign42240_e47779: f64 = (locals.var_fsceac_i * assign42240_e47778);
        let assign42240_e47783: f64 = (0.25 * locals.var_qim_ac);
        let assign42240_e47784: f64 = (1.0 + assign42240_e47783);
        let assign42240_e47785: f64 = (assign42240_e47779 / assign42240_e47784);
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign42240_e47785, ((((locals.var_fsceac_i * (locals.var_dx_wi_1d_ac_dn4 - locals.var_dx_wi_ac_dn4)) * assign42240_e47784) - (assign42240_e47779 * (0.25 * locals.var_qim_ac_dn4))) / (assign42240_e47784 * assign42240_e47784)), ((((locals.var_fsceac_i * (locals.var_dx_wi_1d_ac_dn6 - locals.var_dx_wi_ac_dn6)) * assign42240_e47784) - (assign42240_e47779 * (0.25 * locals.var_qim_ac_dn6))) / (assign42240_e47784 * assign42240_e47784)), ((((locals.var_fsceac_i * (locals.var_dx_wi_1d_ac_dn7 - locals.var_dx_wi_ac_dn7)) * assign42240_e47784) - (assign42240_e47779 * (0.25 * locals.var_qim_ac_dn7))) / (assign42240_e47784 * assign42240_e47784)), ((((locals.var_fsceac_i * (locals.var_dx_wi_1d_ac_dn8 - locals.var_dx_wi_ac_dn8)) * assign42240_e47784) - (assign42240_e47779 * (0.25 * locals.var_qim_ac_dn8))) / (assign42240_e47784 * assign42240_e47784)), ((((locals.var_fsceac_i * (locals.var_dx_wi_1d_ac_dn9 - locals.var_dx_wi_ac_dn9)) * assign42240_e47784) - (assign42240_e47779 * (0.25 * locals.var_qim_ac_dn9))) / (assign42240_e47784 * assign42240_e47784)), );
        locals.var_temp_rv = 0.0;

        let assign42250_e47789: f64 = (locals.var_k1q1s_ac + locals.var_k1q1d_ac);
        let assign42250_e47790: f64 = (0.5 * assign42250_e47789);
        let assign42250_e47792: f64 = (assign42250_e47790 + locals.var_temp);
        (locals.var_k1q1m, locals.var_k1q1m_dn4, locals.var_k1q1m_dn6, locals.var_k1q1m_dn7, locals.var_k1q1m_dn8, locals.var_k1q1m_dn9, ) = (assign42250_e47792, ((0.5 * (locals.var_k1q1s_ac_dn4 + locals.var_k1q1d_ac_dn4)) + locals.var_temp_dn4), ((0.5 * (locals.var_k1q1s_ac_dn6 + locals.var_k1q1d_ac_dn6)) + locals.var_temp_dn6), ((0.5 * (locals.var_k1q1s_ac_dn7 + locals.var_k1q1d_ac_dn7)) + locals.var_temp_dn7), ((0.5 * (locals.var_k1q1s_ac_dn8 + locals.var_k1q1d_ac_dn8)) + locals.var_temp_dn8), ((0.5 * (locals.var_k1q1s_ac_dn9 + locals.var_k1q1d_ac_dn9)) + locals.var_temp_dn9), );
        locals.var_k1q1m_rv = 0.0;

        let assign42260_e47796: f64 = (locals.var_k2q2s_ac + locals.var_k2q2d_ac);
        let assign42260_e47797: f64 = (0.5 * assign42260_e47796);
        let assign42260_e47799: f64 = (assign42260_e47797 - locals.var_temp);
        (locals.var_k2q2m, locals.var_k2q2m_dn4, locals.var_k2q2m_dn6, locals.var_k2q2m_dn7, locals.var_k2q2m_dn8, locals.var_k2q2m_dn9, ) = (assign42260_e47799, ((0.5 * (locals.var_k2q2s_ac_dn4 + locals.var_k2q2d_ac_dn4)) - locals.var_temp_dn4), ((0.5 * (locals.var_k2q2s_ac_dn6 + locals.var_k2q2d_ac_dn6)) - locals.var_temp_dn6), ((0.5 * (locals.var_k2q2s_ac_dn7 + locals.var_k2q2d_ac_dn7)) - locals.var_temp_dn7), ((0.5 * (locals.var_k2q2s_ac_dn8 + locals.var_k2q2d_ac_dn8)) - locals.var_temp_dn8), ((0.5 * (locals.var_k2q2s_ac_dn9 + locals.var_k2q2d_ac_dn9)) - locals.var_temp_dn9), );
        locals.var_k2q2m_rv = 0.0;

        let assign42270_e47802: f64 = if p.p13 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1234 = assign42270_e47802;
        locals.var_guard1234_rv = 0.0;

        if (locals.var_guard1234 != 0.0) {
            let assign42280_e47807: f64 = (locals.var_qi1m_ac / locals.var_qmfact1_ac);
            let assign42280_e47808: f64 = (locals.var_k1q1m + assign42280_e47807);
            let assign42280_e47810: f64 = (assign42280_e47808 - locals.var_qi1m_ac);
            (locals.var_k1q1eff, locals.var_k1q1eff_dn4, locals.var_k1q1eff_dn6, locals.var_k1q1eff_dn7, locals.var_k1q1eff_dn8, locals.var_k1q1eff_dn9, ) = (assign42280_e47810, ((locals.var_k1q1m_dn4 + (((locals.var_qi1m_ac_dn4 * locals.var_qmfact1_ac) - (locals.var_qi1m_ac * locals.var_qmfact1_ac_dn4)) / (locals.var_qmfact1_ac * locals.var_qmfact1_ac))) - locals.var_qi1m_ac_dn4), ((locals.var_k1q1m_dn6 + (((locals.var_qi1m_ac_dn6 * locals.var_qmfact1_ac) - (locals.var_qi1m_ac * locals.var_qmfact1_ac_dn6)) / (locals.var_qmfact1_ac * locals.var_qmfact1_ac))) - locals.var_qi1m_ac_dn6), ((locals.var_k1q1m_dn7 + (((locals.var_qi1m_ac_dn7 * locals.var_qmfact1_ac) - (locals.var_qi1m_ac * locals.var_qmfact1_ac_dn7)) / (locals.var_qmfact1_ac * locals.var_qmfact1_ac))) - locals.var_qi1m_ac_dn7), ((locals.var_k1q1m_dn8 + (((locals.var_qi1m_ac_dn8 * locals.var_qmfact1_ac) - (locals.var_qi1m_ac * locals.var_qmfact1_ac_dn8)) / (locals.var_qmfact1_ac * locals.var_qmfact1_ac))) - locals.var_qi1m_ac_dn8), ((locals.var_k1q1m_dn9 + (((locals.var_qi1m_ac_dn9 * locals.var_qmfact1_ac) - (locals.var_qi1m_ac * locals.var_qmfact1_ac_dn9)) / (locals.var_qmfact1_ac * locals.var_qmfact1_ac))) - locals.var_qi1m_ac_dn9), );
            locals.var_k1q1eff_rv = 0.0;
        }

        if (locals.var_guard1234 != 0.0) {
            let assign42290_e47817: f64 = (locals.var_qi2m_ac / locals.var_qmfact2_ac);
            let assign42290_e47818: f64 = (locals.var_k2q2m + assign42290_e47817);
            let assign42290_e47820: f64 = (assign42290_e47818 - locals.var_qi2m_ac);
            (locals.var_k2q2eff, locals.var_k2q2eff_dn4, locals.var_k2q2eff_dn6, locals.var_k2q2eff_dn7, locals.var_k2q2eff_dn8, locals.var_k2q2eff_dn9, ) = (assign42290_e47820, ((locals.var_k2q2m_dn4 + (((locals.var_qi2m_ac_dn4 * locals.var_qmfact2_ac) - (locals.var_qi2m_ac * locals.var_qmfact2_ac_dn4)) / (locals.var_qmfact2_ac * locals.var_qmfact2_ac))) - locals.var_qi2m_ac_dn4), ((locals.var_k2q2m_dn6 + (((locals.var_qi2m_ac_dn6 * locals.var_qmfact2_ac) - (locals.var_qi2m_ac * locals.var_qmfact2_ac_dn6)) / (locals.var_qmfact2_ac * locals.var_qmfact2_ac))) - locals.var_qi2m_ac_dn6), ((locals.var_k2q2m_dn7 + (((locals.var_qi2m_ac_dn7 * locals.var_qmfact2_ac) - (locals.var_qi2m_ac * locals.var_qmfact2_ac_dn7)) / (locals.var_qmfact2_ac * locals.var_qmfact2_ac))) - locals.var_qi2m_ac_dn7), ((locals.var_k2q2m_dn8 + (((locals.var_qi2m_ac_dn8 * locals.var_qmfact2_ac) - (locals.var_qi2m_ac * locals.var_qmfact2_ac_dn8)) / (locals.var_qmfact2_ac * locals.var_qmfact2_ac))) - locals.var_qi2m_ac_dn8), ((locals.var_k2q2m_dn9 + (((locals.var_qi2m_ac_dn9 * locals.var_qmfact2_ac) - (locals.var_qi2m_ac * locals.var_qmfact2_ac_dn9)) / (locals.var_qmfact2_ac * locals.var_qmfact2_ac))) - locals.var_qi2m_ac_dn9), );
            locals.var_k2q2eff_rv = 0.0;
        }

        if (locals.var_guard1234 == 0.0) {
            (locals.var_k1q1eff, locals.var_k1q1eff_dn4, locals.var_k1q1eff_dn6, locals.var_k1q1eff_dn7, locals.var_k1q1eff_dn8, locals.var_k1q1eff_dn9, ) = (locals.var_k1q1m, locals.var_k1q1m_dn4, locals.var_k1q1m_dn6, locals.var_k1q1m_dn7, locals.var_k1q1m_dn8, locals.var_k1q1m_dn9, );
            locals.var_k1q1eff_rv = 0.0;
            (locals.var_k2q2eff, locals.var_k2q2eff_dn4, locals.var_k2q2eff_dn6, locals.var_k2q2eff_dn7, locals.var_k2q2eff_dn8, locals.var_k2q2eff_dn9, ) = (locals.var_k2q2m, locals.var_k2q2m_dn4, locals.var_k2q2m_dn6, locals.var_k2q2m_dn7, locals.var_k2q2m_dn8, locals.var_k2q2m_dn9, );
            locals.var_k2q2eff_rv = 0.0;
        }

        let assign42320_e47835: f64 = (locals.var_delta_k1q1_ac * locals.var_prod1_ac);
        let assign42320_e47837: f64 = (assign42320_e47835 * 0.3333333333333);
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9, ) = (assign42320_e47837, (((locals.var_delta_k1q1_ac_dn4 * locals.var_prod1_ac) + (locals.var_delta_k1q1_ac * locals.var_prod1_ac_dn4)) * 0.3333333333333), (((locals.var_delta_k1q1_ac_dn6 * locals.var_prod1_ac) + (locals.var_delta_k1q1_ac * locals.var_prod1_ac_dn6)) * 0.3333333333333), (((locals.var_delta_k1q1_ac_dn7 * locals.var_prod1_ac) + (locals.var_delta_k1q1_ac * locals.var_prod1_ac_dn7)) * 0.3333333333333), (((locals.var_delta_k1q1_ac_dn8 * locals.var_prod1_ac) + (locals.var_delta_k1q1_ac * locals.var_prod1_ac_dn8)) * 0.3333333333333), (((locals.var_delta_k1q1_ac_dn9 * locals.var_prod1_ac) + (locals.var_delta_k1q1_ac * locals.var_prod1_ac_dn9)) * 0.3333333333333), );
        locals.var_temp1_rv = 0.0;

        let assign42330_e47840: f64 = (locals.var_delta_k1q1_ac * 0.1666666666667);
        let assign42330_e47846: f64 = (0.2 * locals.var_prod1_ac);
        let assign42330_e47847: f64 = (1.0 - assign42330_e47846);
        let assign42330_e47848: f64 = (locals.var_prod1_ac * assign42330_e47847);
        let assign42330_e47849: f64 = (1.0 + assign42330_e47848);
        let assign42330_e47850: f64 = (assign42330_e47840 * assign42330_e47849);
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9, ) = (assign42330_e47850, (((locals.var_delta_k1q1_ac_dn4 * 0.1666666666667) * assign42330_e47849) + (assign42330_e47840 * ((locals.var_prod1_ac_dn4 * assign42330_e47847) + (locals.var_prod1_ac * (-(0.2 * locals.var_prod1_ac_dn4)))))), (((locals.var_delta_k1q1_ac_dn6 * 0.1666666666667) * assign42330_e47849) + (assign42330_e47840 * ((locals.var_prod1_ac_dn6 * assign42330_e47847) + (locals.var_prod1_ac * (-(0.2 * locals.var_prod1_ac_dn6)))))), (((locals.var_delta_k1q1_ac_dn7 * 0.1666666666667) * assign42330_e47849) + (assign42330_e47840 * ((locals.var_prod1_ac_dn7 * assign42330_e47847) + (locals.var_prod1_ac * (-(0.2 * locals.var_prod1_ac_dn7)))))), (((locals.var_delta_k1q1_ac_dn8 * 0.1666666666667) * assign42330_e47849) + (assign42330_e47840 * ((locals.var_prod1_ac_dn8 * assign42330_e47847) + (locals.var_prod1_ac * (-(0.2 * locals.var_prod1_ac_dn8)))))), (((locals.var_delta_k1q1_ac_dn9 * 0.1666666666667) * assign42330_e47849) + (assign42330_e47840 * ((locals.var_prod1_ac_dn9 * assign42330_e47847) + (locals.var_prod1_ac * (-(0.2 * locals.var_prod1_ac_dn9)))))), );
        locals.var_temp2_rv = 0.0;

        let assign42340_e47853: f64 = (0.5 * locals.var_k1q1eff);
        let assign42340_e47855: f64 = (assign42340_e47853 * locals.var_ratio_pd_ac);
        let assign42340_e47857: f64 = (assign42340_e47855 + locals.var_temp2);
        (locals.var_k1q1deff, locals.var_k1q1deff_dn4, locals.var_k1q1deff_dn6, locals.var_k1q1deff_dn7, locals.var_k1q1deff_dn8, locals.var_k1q1deff_dn9, ) = (assign42340_e47857, ((((0.5 * locals.var_k1q1eff_dn4) * locals.var_ratio_pd_ac) + (assign42340_e47853 * locals.var_ratio_pd_ac_dn4)) + locals.var_temp2_dn4), ((((0.5 * locals.var_k1q1eff_dn6) * locals.var_ratio_pd_ac) + (assign42340_e47853 * locals.var_ratio_pd_ac_dn6)) + locals.var_temp2_dn6), ((((0.5 * locals.var_k1q1eff_dn7) * locals.var_ratio_pd_ac) + (assign42340_e47853 * locals.var_ratio_pd_ac_dn7)) + locals.var_temp2_dn7), ((((0.5 * locals.var_k1q1eff_dn8) * locals.var_ratio_pd_ac) + (assign42340_e47853 * locals.var_ratio_pd_ac_dn8)) + locals.var_temp2_dn8), ((((0.5 * locals.var_k1q1eff_dn9) * locals.var_ratio_pd_ac) + (assign42340_e47853 * locals.var_ratio_pd_ac_dn9)) + locals.var_temp2_dn9), );
        locals.var_k1q1deff_rv = 0.0;

        let assign42350_e47860: f64 = (locals.var_k1q1eff * locals.var_ratio_pd_ac);
        let assign42350_e47862: f64 = (assign42350_e47860 + locals.var_temp1);
        (locals.var_k1q1eff, locals.var_k1q1eff_dn4, locals.var_k1q1eff_dn6, locals.var_k1q1eff_dn7, locals.var_k1q1eff_dn8, locals.var_k1q1eff_dn9, ) = (assign42350_e47862, (((locals.var_k1q1eff_dn4 * locals.var_ratio_pd_ac) + (locals.var_k1q1eff * locals.var_ratio_pd_ac_dn4)) + locals.var_temp1_dn4), (((locals.var_k1q1eff_dn6 * locals.var_ratio_pd_ac) + (locals.var_k1q1eff * locals.var_ratio_pd_ac_dn6)) + locals.var_temp1_dn6), (((locals.var_k1q1eff_dn7 * locals.var_ratio_pd_ac) + (locals.var_k1q1eff * locals.var_ratio_pd_ac_dn7)) + locals.var_temp1_dn7), (((locals.var_k1q1eff_dn8 * locals.var_ratio_pd_ac) + (locals.var_k1q1eff * locals.var_ratio_pd_ac_dn8)) + locals.var_temp1_dn8), (((locals.var_k1q1eff_dn9 * locals.var_ratio_pd_ac) + (locals.var_k1q1eff * locals.var_ratio_pd_ac_dn9)) + locals.var_temp1_dn9), );
        locals.var_k1q1eff_rv = 0.0;

        let assign42360_e47865: f64 = (locals.var_delta_k2q2_ac * locals.var_prod2_ac);
        let assign42360_e47867: f64 = (assign42360_e47865 * 0.3333333333333);
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9, ) = (assign42360_e47867, (((locals.var_delta_k2q2_ac_dn4 * locals.var_prod2_ac) + (locals.var_delta_k2q2_ac * locals.var_prod2_ac_dn4)) * 0.3333333333333), (((locals.var_delta_k2q2_ac_dn6 * locals.var_prod2_ac) + (locals.var_delta_k2q2_ac * locals.var_prod2_ac_dn6)) * 0.3333333333333), (((locals.var_delta_k2q2_ac_dn7 * locals.var_prod2_ac) + (locals.var_delta_k2q2_ac * locals.var_prod2_ac_dn7)) * 0.3333333333333), (((locals.var_delta_k2q2_ac_dn8 * locals.var_prod2_ac) + (locals.var_delta_k2q2_ac * locals.var_prod2_ac_dn8)) * 0.3333333333333), (((locals.var_delta_k2q2_ac_dn9 * locals.var_prod2_ac) + (locals.var_delta_k2q2_ac * locals.var_prod2_ac_dn9)) * 0.3333333333333), );
        locals.var_temp1_rv = 0.0;

        let assign42370_e47870: f64 = (locals.var_delta_k2q2_ac * 0.1666666666667);
        let assign42370_e47876: f64 = (0.2 * locals.var_prod2_ac);
        let assign42370_e47877: f64 = (1.0 - assign42370_e47876);
        let assign42370_e47878: f64 = (locals.var_prod2_ac * assign42370_e47877);
        let assign42370_e47879: f64 = (1.0 + assign42370_e47878);
        let assign42370_e47880: f64 = (assign42370_e47870 * assign42370_e47879);
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9, ) = (assign42370_e47880, (((locals.var_delta_k2q2_ac_dn4 * 0.1666666666667) * assign42370_e47879) + (assign42370_e47870 * ((locals.var_prod2_ac_dn4 * assign42370_e47877) + (locals.var_prod2_ac * (-(0.2 * locals.var_prod2_ac_dn4)))))), (((locals.var_delta_k2q2_ac_dn6 * 0.1666666666667) * assign42370_e47879) + (assign42370_e47870 * ((locals.var_prod2_ac_dn6 * assign42370_e47877) + (locals.var_prod2_ac * (-(0.2 * locals.var_prod2_ac_dn6)))))), (((locals.var_delta_k2q2_ac_dn7 * 0.1666666666667) * assign42370_e47879) + (assign42370_e47870 * ((locals.var_prod2_ac_dn7 * assign42370_e47877) + (locals.var_prod2_ac * (-(0.2 * locals.var_prod2_ac_dn7)))))), (((locals.var_delta_k2q2_ac_dn8 * 0.1666666666667) * assign42370_e47879) + (assign42370_e47870 * ((locals.var_prod2_ac_dn8 * assign42370_e47877) + (locals.var_prod2_ac * (-(0.2 * locals.var_prod2_ac_dn8)))))), (((locals.var_delta_k2q2_ac_dn9 * 0.1666666666667) * assign42370_e47879) + (assign42370_e47870 * ((locals.var_prod2_ac_dn9 * assign42370_e47877) + (locals.var_prod2_ac * (-(0.2 * locals.var_prod2_ac_dn9)))))), );
        locals.var_temp2_rv = 0.0;

        let assign42380_e47883: f64 = (0.5 * locals.var_k2q2eff);
        let assign42380_e47885: f64 = (assign42380_e47883 + locals.var_temp2);
        (locals.var_k2q2deff, locals.var_k2q2deff_dn4, locals.var_k2q2deff_dn6, locals.var_k2q2deff_dn7, locals.var_k2q2deff_dn8, locals.var_k2q2deff_dn9, ) = (assign42380_e47885, ((0.5 * locals.var_k2q2eff_dn4) + locals.var_temp2_dn4), ((0.5 * locals.var_k2q2eff_dn6) + locals.var_temp2_dn6), ((0.5 * locals.var_k2q2eff_dn7) + locals.var_temp2_dn7), ((0.5 * locals.var_k2q2eff_dn8) + locals.var_temp2_dn8), ((0.5 * locals.var_k2q2eff_dn9) + locals.var_temp2_dn9), );
        locals.var_k2q2deff_rv = 0.0;

        let assign42390_e47888: f64 = (locals.var_k2q2eff + locals.var_temp1);
        (locals.var_k2q2eff, locals.var_k2q2eff_dn4, locals.var_k2q2eff_dn6, locals.var_k2q2eff_dn7, locals.var_k2q2eff_dn8, locals.var_k2q2eff_dn9, ) = (assign42390_e47888, (locals.var_k2q2eff_dn4 + locals.var_temp1_dn4), (locals.var_k2q2eff_dn6 + locals.var_temp1_dn6), (locals.var_k2q2eff_dn7 + locals.var_temp1_dn7), (locals.var_k2q2eff_dn8 + locals.var_temp1_dn8), (locals.var_k2q2eff_dn9 + locals.var_temp1_dn9), );
        locals.var_k2q2eff_rv = 0.0;

        let assign42400_e47891: f64 = (locals.var_csiprime_ac * locals.var_area_phit);
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign42400_e47891, ((locals.var_csiprime_ac_dn4 * locals.var_area_phit) + (locals.var_csiprime_ac * locals.var_area_phit_dn4)), ((locals.var_csiprime_ac_dn6 * locals.var_area_phit) + (locals.var_csiprime_ac * locals.var_area_phit_dn6)), ((locals.var_csiprime_ac_dn7 * locals.var_area_phit) + (locals.var_csiprime_ac * locals.var_area_phit_dn7)), ((locals.var_csiprime_ac_dn8 * locals.var_area_phit) + (locals.var_csiprime_ac * locals.var_area_phit_dn8)), ((locals.var_csiprime_ac_dn9 * locals.var_area_phit) + (locals.var_csiprime_ac * locals.var_area_phit_dn9)), );
        locals.var_temp_rv = 0.0;

        let assign42410_e47894: f64 = (locals.var_temp * locals.var_k1q1eff);
        (locals.var_qg, locals.var_qg_dn4, locals.var_qg_dn6, locals.var_qg_dn7, locals.var_qg_dn8, locals.var_qg_dn9, ) = (assign42410_e47894, ((locals.var_temp_dn4 * locals.var_k1q1eff) + (locals.var_temp * locals.var_k1q1eff_dn4)), ((locals.var_temp_dn6 * locals.var_k1q1eff) + (locals.var_temp * locals.var_k1q1eff_dn6)), ((locals.var_temp_dn7 * locals.var_k1q1eff) + (locals.var_temp * locals.var_k1q1eff_dn7)), ((locals.var_temp_dn8 * locals.var_k1q1eff) + (locals.var_temp * locals.var_k1q1eff_dn8)), ((locals.var_temp_dn9 * locals.var_k1q1eff) + (locals.var_temp * locals.var_k1q1eff_dn9)), );
        locals.var_qg_rv = 0.0;

        let assign42420_e47897: f64 = (locals.var_temp * locals.var_k2q2eff);
        (locals.var_qb, locals.var_qb_dn4, locals.var_qb_dn6, locals.var_qb_dn7, locals.var_qb_dn8, locals.var_qb_dn9, ) = (assign42420_e47897, ((locals.var_temp_dn4 * locals.var_k2q2eff) + (locals.var_temp * locals.var_k2q2eff_dn4)), ((locals.var_temp_dn6 * locals.var_k2q2eff) + (locals.var_temp * locals.var_k2q2eff_dn6)), ((locals.var_temp_dn7 * locals.var_k2q2eff) + (locals.var_temp * locals.var_k2q2eff_dn7)), ((locals.var_temp_dn8 * locals.var_k2q2eff) + (locals.var_temp * locals.var_k2q2eff_dn8)), ((locals.var_temp_dn9 * locals.var_k2q2eff) + (locals.var_temp * locals.var_k2q2eff_dn9)), );
        locals.var_qb_rv = 0.0;

        let assign42430_e47899: f64 = (-locals.var_temp);
        let assign42430_e47902: f64 = (locals.var_k1q1deff + locals.var_k2q2deff);
        let assign42430_e47903: f64 = (assign42430_e47899 * assign42430_e47902);
        (locals.var_qd, locals.var_qd_dn4, locals.var_qd_dn6, locals.var_qd_dn7, locals.var_qd_dn8, locals.var_qd_dn9, ) = (assign42430_e47903, (((-locals.var_temp_dn4) * assign42430_e47902) + (assign42430_e47899 * (locals.var_k1q1deff_dn4 + locals.var_k2q2deff_dn4))), (((-locals.var_temp_dn6) * assign42430_e47902) + (assign42430_e47899 * (locals.var_k1q1deff_dn6 + locals.var_k2q2deff_dn6))), (((-locals.var_temp_dn7) * assign42430_e47902) + (assign42430_e47899 * (locals.var_k1q1deff_dn7 + locals.var_k2q2deff_dn7))), (((-locals.var_temp_dn8) * assign42430_e47902) + (assign42430_e47899 * (locals.var_k1q1deff_dn8 + locals.var_k2q2deff_dn8))), (((-locals.var_temp_dn9) * assign42430_e47902) + (assign42430_e47899 * (locals.var_k1q1deff_dn9 + locals.var_k2q2deff_dn9))), );
        locals.var_qd_rv = 0.0;

        let assign42440_e47906: f64 = if locals.var_fif_i > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1235 = assign42440_e47906;
        locals.var_guard1235_rv = 0.0;

        if (locals.var_guard1235 != 0.0) {
            let assign42450_e47911: f64 = (2.0 * 0.6931471805599);
            let assign42450_e47912: f64 = (locals.var_xth_1d + assign42450_e47911);
            (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign42450_e47912, locals.var_xth_1d_dn4, locals.var_xth_1d_dn6, locals.var_xth_1d_dn7, locals.var_xth_1d_dn8, locals.var_xth_1d_dn9, );
            locals.var_temp_rv = 0.0;
        }

        if (locals.var_guard1235 != 0.0) {
            let assign42460_e47918: f64 = (locals.var_xdrifts_ac + locals.var_temp);
            (locals.var_xeffs, locals.var_xeffs_dn4, locals.var_xeffs_dn6, locals.var_xeffs_dn7, locals.var_xeffs_dn8, locals.var_xeffs_dn9, ) = (assign42460_e47918, (locals.var_xdrifts_ac_dn4 + locals.var_temp_dn4), (locals.var_xdrifts_ac_dn6 + locals.var_temp_dn6), (locals.var_xdrifts_ac_dn7 + locals.var_temp_dn7), (locals.var_xdrifts_ac_dn8 + locals.var_temp_dn8), (locals.var_xdrifts_ac_dn9 + locals.var_temp_dn9), );
            locals.var_xeffs_rv = 0.0;
        }

        if (locals.var_guard1235 != 0.0) {
            let assign42470_e47924: f64 = (locals.var_xdriftd_ac + locals.var_temp);
            (locals.var_xeffd, locals.var_xeffd_dn4, locals.var_xeffd_dn6, locals.var_xeffd_dn7, locals.var_xeffd_dn8, locals.var_xeffd_dn9, ) = (assign42470_e47924, (locals.var_xdriftd_ac_dn4 + locals.var_temp_dn4), (locals.var_xdriftd_ac_dn6 + locals.var_temp_dn6), (locals.var_xdriftd_ac_dn7 + locals.var_temp_dn7), (locals.var_xdriftd_ac_dn8 + locals.var_temp_dn8), (locals.var_xdriftd_ac_dn9 + locals.var_temp_dn9), );
            locals.var_xeffd_rv = 0.0;
        }

        if (locals.var_guard1235 != 0.0) {
            let assign42480_e47931: f64 = (locals.var_xeffs + locals.var_xth_1d);
            let assign42480_e47934: f64 = (locals.var_xeffs - locals.var_xth_1d);
            let assign42480_e47937: f64 = (locals.var_xeffs - locals.var_xth_1d);
            let assign42480_e47938: f64 = (assign42480_e47934 * assign42480_e47937);
            let assign42480_e47940: f64 = (assign42480_e47938 + 9.0);
            let assign42480_e47941: f64 = (assign42480_e47940).sqrt();
            let assign42480_e47942: f64 = (assign42480_e47931 - assign42480_e47941);
            let assign42480_e47943: f64 = (0.5 * assign42480_e47942);
            (locals.var_xstars, locals.var_xstars_dn4, locals.var_xstars_dn6, locals.var_xstars_dn7, locals.var_xstars_dn8, locals.var_xstars_dn9, ) = (assign42480_e47943, (0.5 * ((locals.var_xeffs_dn4 + locals.var_xth_1d_dn4) - ((((locals.var_xeffs_dn4 - locals.var_xth_1d_dn4) * assign42480_e47937) + (assign42480_e47934 * (locals.var_xeffs_dn4 - locals.var_xth_1d_dn4))) / (2.0 * assign42480_e47941)))), (0.5 * ((locals.var_xeffs_dn6 + locals.var_xth_1d_dn6) - ((((locals.var_xeffs_dn6 - locals.var_xth_1d_dn6) * assign42480_e47937) + (assign42480_e47934 * (locals.var_xeffs_dn6 - locals.var_xth_1d_dn6))) / (2.0 * assign42480_e47941)))), (0.5 * ((locals.var_xeffs_dn7 + locals.var_xth_1d_dn7) - ((((locals.var_xeffs_dn7 - locals.var_xth_1d_dn7) * assign42480_e47937) + (assign42480_e47934 * (locals.var_xeffs_dn7 - locals.var_xth_1d_dn7))) / (2.0 * assign42480_e47941)))), (0.5 * ((locals.var_xeffs_dn8 + locals.var_xth_1d_dn8) - ((((locals.var_xeffs_dn8 - locals.var_xth_1d_dn8) * assign42480_e47937) + (assign42480_e47934 * (locals.var_xeffs_dn8 - locals.var_xth_1d_dn8))) / (2.0 * assign42480_e47941)))), (0.5 * ((locals.var_xeffs_dn9 + locals.var_xth_1d_dn9) - ((((locals.var_xeffs_dn9 - locals.var_xth_1d_dn9) * assign42480_e47937) + (assign42480_e47934 * (locals.var_xeffs_dn9 - locals.var_xth_1d_dn9))) / (2.0 * assign42480_e47941)))), );
            locals.var_xstars_rv = 0.0;
        }

        if (locals.var_guard1235 != 0.0) {
            let assign42490_e47951: f64 = (locals.var_xth_1d + locals.var_xd);
            let assign42490_e47952: f64 = (locals.var_xeffd + assign42490_e47951);
            let assign42490_e47956: f64 = (locals.var_xth_1d + locals.var_xd);
            let assign42490_e47957: f64 = (locals.var_xeffd - assign42490_e47956);
            let assign42490_e47961: f64 = (locals.var_xth_1d + locals.var_xd);
            let assign42490_e47962: f64 = (locals.var_xeffd - assign42490_e47961);
            let assign42490_e47963: f64 = (assign42490_e47957 * assign42490_e47962);
            let assign42490_e47965: f64 = (assign42490_e47963 + 9.0);
            let assign42490_e47966: f64 = (assign42490_e47965).sqrt();
            let assign42490_e47967: f64 = (assign42490_e47952 - assign42490_e47966);
            let assign42490_e47968: f64 = (0.5 * assign42490_e47967);
            (locals.var_xstard, locals.var_xstard_dn4, locals.var_xstard_dn6, locals.var_xstard_dn7, locals.var_xstard_dn8, locals.var_xstard_dn9, ) = (assign42490_e47968, (0.5 * ((locals.var_xeffd_dn4 + (locals.var_xth_1d_dn4 + locals.var_xd_dn4)) - ((((locals.var_xeffd_dn4 - (locals.var_xth_1d_dn4 + locals.var_xd_dn4)) * assign42490_e47962) + (assign42490_e47957 * (locals.var_xeffd_dn4 - (locals.var_xth_1d_dn4 + locals.var_xd_dn4)))) / (2.0 * assign42490_e47966)))), (0.5 * ((locals.var_xeffd_dn6 + (locals.var_xth_1d_dn6 + locals.var_xd_dn6)) - ((((locals.var_xeffd_dn6 - (locals.var_xth_1d_dn6 + locals.var_xd_dn6)) * assign42490_e47962) + (assign42490_e47957 * (locals.var_xeffd_dn6 - (locals.var_xth_1d_dn6 + locals.var_xd_dn6)))) / (2.0 * assign42490_e47966)))), (0.5 * ((locals.var_xeffd_dn7 + (locals.var_xth_1d_dn7 + locals.var_xd_dn7)) - ((((locals.var_xeffd_dn7 - (locals.var_xth_1d_dn7 + locals.var_xd_dn7)) * assign42490_e47962) + (assign42490_e47957 * (locals.var_xeffd_dn7 - (locals.var_xth_1d_dn7 + locals.var_xd_dn7)))) / (2.0 * assign42490_e47966)))), (0.5 * ((locals.var_xeffd_dn8 + (locals.var_xth_1d_dn8 + locals.var_xd_dn8)) - ((((locals.var_xeffd_dn8 - (locals.var_xth_1d_dn8 + locals.var_xd_dn8)) * assign42490_e47962) + (assign42490_e47957 * (locals.var_xeffd_dn8 - (locals.var_xth_1d_dn8 + locals.var_xd_dn8)))) / (2.0 * assign42490_e47966)))), (0.5 * ((locals.var_xeffd_dn9 + (locals.var_xth_1d_dn9 + locals.var_xd_dn9)) - ((((locals.var_xeffd_dn9 - (locals.var_xth_1d_dn9 + locals.var_xd_dn9)) * assign42490_e47962) + (assign42490_e47957 * (locals.var_xeffd_dn9 - (locals.var_xth_1d_dn9 + locals.var_xd_dn9)))) / (2.0 * assign42490_e47966)))), );
            locals.var_xstard_rv = 0.0;
        }

        if (locals.var_guard1235 != 0.0) {
            let assign42500_e47976: f64 = (0.5 + locals.var_inv_k2_ac);
            let assign42500_e47977: f64 = (locals.var_keq_ac * assign42500_e47976);
            let assign42500_e47978: f64 = (assign42500_e47977).sqrt();
            let assign42500_e47979: f64 = (locals.var_lambda2d * assign42500_e47978);
            (locals.var_lambdaf, locals.var_lambdaf_dn4, locals.var_lambdaf_dn6, locals.var_lambdaf_dn7, locals.var_lambdaf_dn8, locals.var_lambdaf_dn9, ) = (assign42500_e47979, (locals.var_lambda2d * (((locals.var_keq_ac_dn4 * assign42500_e47976) + (locals.var_keq_ac * locals.var_inv_k2_ac_dn4)) / (2.0 * assign42500_e47978))), (locals.var_lambda2d * (((locals.var_keq_ac_dn6 * assign42500_e47976) + (locals.var_keq_ac * locals.var_inv_k2_ac_dn6)) / (2.0 * assign42500_e47978))), (locals.var_lambda2d * (((locals.var_keq_ac_dn7 * assign42500_e47976) + (locals.var_keq_ac * locals.var_inv_k2_ac_dn7)) / (2.0 * assign42500_e47978))), (locals.var_lambda2d * (((locals.var_keq_ac_dn8 * assign42500_e47976) + (locals.var_keq_ac * locals.var_inv_k2_ac_dn8)) / (2.0 * assign42500_e47978))), (locals.var_lambda2d * (((locals.var_keq_ac_dn9 * assign42500_e47976) + (locals.var_keq_ac * locals.var_inv_k2_ac_dn9)) / (2.0 * assign42500_e47978))), );
            locals.var_lambdaf_rv = 0.0;
        }

        if (locals.var_guard1235 != 0.0) {
            let assign42510_e47986: f64 = (locals.var_keq_ac * locals.var_k1_ac);
            let assign42510_e47988: f64 = (assign42510_e47986 * locals.var_inv_k2_ac);
            let assign42510_e47991: f64 = (0.5 + locals.var_inv_k1_ac);
            let assign42510_e47992: f64 = (assign42510_e47988 * assign42510_e47991);
            let assign42510_e47993: f64 = (assign42510_e47992).sqrt();
            let assign42510_e47994: f64 = (locals.var_lambda2d * assign42510_e47993);
            (locals.var_lambdab, locals.var_lambdab_dn4, locals.var_lambdab_dn6, locals.var_lambdab_dn7, locals.var_lambdab_dn8, locals.var_lambdab_dn9, ) = (assign42510_e47994, (locals.var_lambda2d * (((((((locals.var_keq_ac_dn4 * locals.var_k1_ac) + (locals.var_keq_ac * locals.var_k1_ac_dn4)) * locals.var_inv_k2_ac) + (assign42510_e47986 * locals.var_inv_k2_ac_dn4)) * assign42510_e47991) + (assign42510_e47988 * locals.var_inv_k1_ac_dn4)) / (2.0 * assign42510_e47993))), (locals.var_lambda2d * (((((((locals.var_keq_ac_dn6 * locals.var_k1_ac) + (locals.var_keq_ac * locals.var_k1_ac_dn6)) * locals.var_inv_k2_ac) + (assign42510_e47986 * locals.var_inv_k2_ac_dn6)) * assign42510_e47991) + (assign42510_e47988 * locals.var_inv_k1_ac_dn6)) / (2.0 * assign42510_e47993))), (locals.var_lambda2d * (((((((locals.var_keq_ac_dn7 * locals.var_k1_ac) + (locals.var_keq_ac * locals.var_k1_ac_dn7)) * locals.var_inv_k2_ac) + (assign42510_e47986 * locals.var_inv_k2_ac_dn7)) * assign42510_e47991) + (assign42510_e47988 * locals.var_inv_k1_ac_dn7)) / (2.0 * assign42510_e47993))), (locals.var_lambda2d * (((((((locals.var_keq_ac_dn8 * locals.var_k1_ac) + (locals.var_keq_ac * locals.var_k1_ac_dn8)) * locals.var_inv_k2_ac) + (assign42510_e47986 * locals.var_inv_k2_ac_dn8)) * assign42510_e47991) + (assign42510_e47988 * locals.var_inv_k1_ac_dn8)) / (2.0 * assign42510_e47993))), (locals.var_lambda2d * (((((((locals.var_keq_ac_dn9 * locals.var_k1_ac) + (locals.var_keq_ac * locals.var_k1_ac_dn9)) * locals.var_inv_k2_ac) + (assign42510_e47986 * locals.var_inv_k2_ac_dn9)) * assign42510_e47991) + (assign42510_e47988 * locals.var_inv_k1_ac_dn9)) / (2.0 * assign42510_e47993))), );
            locals.var_lambdab_rv = 0.0;
        }

        if (locals.var_guard1235 != 0.0) {
            let assign42520_e48000: f64 = (locals.var_lambdaf * locals.var_lambdaf);
            let assign42520_e48002: f64 = (assign42520_e48000 * locals.var_inner_sd);
            (locals.var_xalphaf, locals.var_xalphaf_dn4, locals.var_xalphaf_dn6, locals.var_xalphaf_dn7, locals.var_xalphaf_dn8, locals.var_xalphaf_dn9, ) = (assign42520_e48002, ((((locals.var_lambdaf_dn4 * locals.var_lambdaf) + (locals.var_lambdaf * locals.var_lambdaf_dn4)) * locals.var_inner_sd) + (assign42520_e48000 * locals.var_inner_sd_dn4)), ((((locals.var_lambdaf_dn6 * locals.var_lambdaf) + (locals.var_lambdaf * locals.var_lambdaf_dn6)) * locals.var_inner_sd) + (assign42520_e48000 * locals.var_inner_sd_dn6)), ((((locals.var_lambdaf_dn7 * locals.var_lambdaf) + (locals.var_lambdaf * locals.var_lambdaf_dn7)) * locals.var_inner_sd) + (assign42520_e48000 * locals.var_inner_sd_dn7)), ((((locals.var_lambdaf_dn8 * locals.var_lambdaf) + (locals.var_lambdaf * locals.var_lambdaf_dn8)) * locals.var_inner_sd) + (assign42520_e48000 * locals.var_inner_sd_dn8)), ((((locals.var_lambdaf_dn9 * locals.var_lambdaf) + (locals.var_lambdaf * locals.var_lambdaf_dn9)) * locals.var_inner_sd) + (assign42520_e48000 * locals.var_inner_sd_dn9)), );
            locals.var_xalphaf_rv = 0.0;
        }

        if (locals.var_guard1235 != 0.0) {
            let assign42530_e48008: f64 = (locals.var_lambdab * locals.var_lambdab);
            let assign42530_e48010: f64 = (assign42530_e48008 * locals.var_inner_sd);
            (locals.var_xalphab, locals.var_xalphab_dn4, locals.var_xalphab_dn6, locals.var_xalphab_dn7, locals.var_xalphab_dn8, locals.var_xalphab_dn9, ) = (assign42530_e48010, ((((locals.var_lambdab_dn4 * locals.var_lambdab) + (locals.var_lambdab * locals.var_lambdab_dn4)) * locals.var_inner_sd) + (assign42530_e48008 * locals.var_inner_sd_dn4)), ((((locals.var_lambdab_dn6 * locals.var_lambdab) + (locals.var_lambdab * locals.var_lambdab_dn6)) * locals.var_inner_sd) + (assign42530_e48008 * locals.var_inner_sd_dn6)), ((((locals.var_lambdab_dn7 * locals.var_lambdab) + (locals.var_lambdab * locals.var_lambdab_dn7)) * locals.var_inner_sd) + (assign42530_e48008 * locals.var_inner_sd_dn7)), ((((locals.var_lambdab_dn8 * locals.var_lambdab) + (locals.var_lambdab * locals.var_lambdab_dn8)) * locals.var_inner_sd) + (assign42530_e48008 * locals.var_inner_sd_dn8)), ((((locals.var_lambdab_dn9 * locals.var_lambdab) + (locals.var_lambdab * locals.var_lambdab_dn9)) * locals.var_inner_sd) + (assign42530_e48008 * locals.var_inner_sd_dn9)), );
            locals.var_xalphab_rv = 0.0;
        }

        if (locals.var_guard1235 != 0.0) {
            let assign42540_e48016: f64 = (locals.var_xsd - locals.var_xstars);
            (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9, ) = (assign42540_e48016, (locals.var_xsd_dn4 - locals.var_xstars_dn4), (locals.var_xsd_dn6 - locals.var_xstars_dn6), (locals.var_xsd_dn7 - locals.var_xstars_dn7), (locals.var_xsd_dn8 - locals.var_xstars_dn8), (locals.var_xsd_dn9 - locals.var_xstars_dn9), );
            locals.var_temp1_rv = 0.0;
        }

        if (locals.var_guard1235 != 0.0) {
            let assign42550_e48022: f64 = (locals.var_xsd + locals.var_xd);
            let assign42550_e48024: f64 = (assign42550_e48022 - locals.var_xstard);
            (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9, ) = (assign42550_e48024, ((locals.var_xsd_dn4 + locals.var_xd_dn4) - locals.var_xstard_dn4), ((locals.var_xsd_dn6 + locals.var_xd_dn6) - locals.var_xstard_dn6), ((locals.var_xsd_dn7 + locals.var_xd_dn7) - locals.var_xstard_dn7), ((locals.var_xsd_dn8 + locals.var_xd_dn8) - locals.var_xstard_dn8), ((locals.var_xsd_dn9 + locals.var_xd_dn9) - locals.var_xstard_dn9), );
            locals.var_temp2_rv = 0.0;
        }

        if (locals.var_guard1235 != 0.0) {
            let assign42560_e48030: f64 = (2.0 * locals.var_xalphaf);
            (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign42560_e48030, (2.0 * locals.var_xalphaf_dn4), (2.0 * locals.var_xalphaf_dn6), (2.0 * locals.var_xalphaf_dn7), (2.0 * locals.var_xalphaf_dn8), (2.0 * locals.var_xalphaf_dn9), );
            locals.var_temp_rv = 0.0;
        }

        if (locals.var_guard1235 != 0.0) {
            let assign42570_e48039: f64 = (locals.var_temp1 / locals.var_xalphaf);
            let assign42570_e48040: f64 = (1.0 + assign42570_e48039);
            let assign42570_e48041: f64 = (assign42570_e48040).sqrt();
            let assign42570_e48043: f64 = (assign42570_e48041 - 1.0);
            let assign42570_e48044: f64 = (locals.var_temp * assign42570_e48043);
            let assign42570_e48045: f64 = (locals.var_xstars + assign42570_e48044);
            (locals.var_xedgefs, locals.var_xedgefs_dn4, locals.var_xedgefs_dn6, locals.var_xedgefs_dn7, locals.var_xedgefs_dn8, locals.var_xedgefs_dn9, ) = (assign42570_e48045, (locals.var_xstars_dn4 + ((locals.var_temp_dn4 * assign42570_e48043) + (locals.var_temp * ((((locals.var_temp1_dn4 * locals.var_xalphaf) - (locals.var_temp1 * locals.var_xalphaf_dn4)) / (locals.var_xalphaf * locals.var_xalphaf)) / (2.0 * assign42570_e48041))))), (locals.var_xstars_dn6 + ((locals.var_temp_dn6 * assign42570_e48043) + (locals.var_temp * ((((locals.var_temp1_dn6 * locals.var_xalphaf) - (locals.var_temp1 * locals.var_xalphaf_dn6)) / (locals.var_xalphaf * locals.var_xalphaf)) / (2.0 * assign42570_e48041))))), (locals.var_xstars_dn7 + ((locals.var_temp_dn7 * assign42570_e48043) + (locals.var_temp * ((((locals.var_temp1_dn7 * locals.var_xalphaf) - (locals.var_temp1 * locals.var_xalphaf_dn7)) / (locals.var_xalphaf * locals.var_xalphaf)) / (2.0 * assign42570_e48041))))), (locals.var_xstars_dn8 + ((locals.var_temp_dn8 * assign42570_e48043) + (locals.var_temp * ((((locals.var_temp1_dn8 * locals.var_xalphaf) - (locals.var_temp1 * locals.var_xalphaf_dn8)) / (locals.var_xalphaf * locals.var_xalphaf)) / (2.0 * assign42570_e48041))))), (locals.var_xstars_dn9 + ((locals.var_temp_dn9 * assign42570_e48043) + (locals.var_temp * ((((locals.var_temp1_dn9 * locals.var_xalphaf) - (locals.var_temp1 * locals.var_xalphaf_dn9)) / (locals.var_xalphaf * locals.var_xalphaf)) / (2.0 * assign42570_e48041))))), );
            locals.var_xedgefs_rv = 0.0;
        }

    }

    pub(super) fn stamp_reactive_block_61(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        if (locals.var_guard1235 != 0.0) {
            let assign42580_e48054: f64 = (locals.var_temp2 / locals.var_xalphaf);
            let assign42580_e48055: f64 = (1.0 + assign42580_e48054);
            let assign42580_e48056: f64 = (assign42580_e48055).sqrt();
            let assign42580_e48058: f64 = (assign42580_e48056 - 1.0);
            let assign42580_e48059: f64 = (locals.var_temp * assign42580_e48058);
            let assign42580_e48060: f64 = (locals.var_xstard + assign42580_e48059);
            (locals.var_xedgefd, locals.var_xedgefd_dn4, locals.var_xedgefd_dn6, locals.var_xedgefd_dn7, locals.var_xedgefd_dn8, locals.var_xedgefd_dn9, ) = (assign42580_e48060, (locals.var_xstard_dn4 + ((locals.var_temp_dn4 * assign42580_e48058) + (locals.var_temp * ((((locals.var_temp2_dn4 * locals.var_xalphaf) - (locals.var_temp2 * locals.var_xalphaf_dn4)) / (locals.var_xalphaf * locals.var_xalphaf)) / (2.0 * assign42580_e48056))))), (locals.var_xstard_dn6 + ((locals.var_temp_dn6 * assign42580_e48058) + (locals.var_temp * ((((locals.var_temp2_dn6 * locals.var_xalphaf) - (locals.var_temp2 * locals.var_xalphaf_dn6)) / (locals.var_xalphaf * locals.var_xalphaf)) / (2.0 * assign42580_e48056))))), (locals.var_xstard_dn7 + ((locals.var_temp_dn7 * assign42580_e48058) + (locals.var_temp * ((((locals.var_temp2_dn7 * locals.var_xalphaf) - (locals.var_temp2 * locals.var_xalphaf_dn7)) / (locals.var_xalphaf * locals.var_xalphaf)) / (2.0 * assign42580_e48056))))), (locals.var_xstard_dn8 + ((locals.var_temp_dn8 * assign42580_e48058) + (locals.var_temp * ((((locals.var_temp2_dn8 * locals.var_xalphaf) - (locals.var_temp2 * locals.var_xalphaf_dn8)) / (locals.var_xalphaf * locals.var_xalphaf)) / (2.0 * assign42580_e48056))))), (locals.var_xstard_dn9 + ((locals.var_temp_dn9 * assign42580_e48058) + (locals.var_temp * ((((locals.var_temp2_dn9 * locals.var_xalphaf) - (locals.var_temp2 * locals.var_xalphaf_dn9)) / (locals.var_xalphaf * locals.var_xalphaf)) / (2.0 * assign42580_e48056))))), );
            locals.var_xedgefd_rv = 0.0;
        }

        if (locals.var_guard1235 != 0.0) {
            let assign42590_e48066: f64 = (2.0 * locals.var_xalphab);
            (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign42590_e48066, (2.0 * locals.var_xalphab_dn4), (2.0 * locals.var_xalphab_dn6), (2.0 * locals.var_xalphab_dn7), (2.0 * locals.var_xalphab_dn8), (2.0 * locals.var_xalphab_dn9), );
            locals.var_temp_rv = 0.0;
        }

        if (locals.var_guard1235 != 0.0) {
            let assign42600_e48075: f64 = (locals.var_temp1 / locals.var_xalphab);
            let assign42600_e48076: f64 = (1.0 + assign42600_e48075);
            let assign42600_e48077: f64 = (assign42600_e48076).sqrt();
            let assign42600_e48079: f64 = (assign42600_e48077 - 1.0);
            let assign42600_e48080: f64 = (locals.var_temp * assign42600_e48079);
            let assign42600_e48081: f64 = (locals.var_xstars + assign42600_e48080);
            (locals.var_xedgebs, locals.var_xedgebs_dn4, locals.var_xedgebs_dn6, locals.var_xedgebs_dn7, locals.var_xedgebs_dn8, locals.var_xedgebs_dn9, ) = (assign42600_e48081, (locals.var_xstars_dn4 + ((locals.var_temp_dn4 * assign42600_e48079) + (locals.var_temp * ((((locals.var_temp1_dn4 * locals.var_xalphab) - (locals.var_temp1 * locals.var_xalphab_dn4)) / (locals.var_xalphab * locals.var_xalphab)) / (2.0 * assign42600_e48077))))), (locals.var_xstars_dn6 + ((locals.var_temp_dn6 * assign42600_e48079) + (locals.var_temp * ((((locals.var_temp1_dn6 * locals.var_xalphab) - (locals.var_temp1 * locals.var_xalphab_dn6)) / (locals.var_xalphab * locals.var_xalphab)) / (2.0 * assign42600_e48077))))), (locals.var_xstars_dn7 + ((locals.var_temp_dn7 * assign42600_e48079) + (locals.var_temp * ((((locals.var_temp1_dn7 * locals.var_xalphab) - (locals.var_temp1 * locals.var_xalphab_dn7)) / (locals.var_xalphab * locals.var_xalphab)) / (2.0 * assign42600_e48077))))), (locals.var_xstars_dn8 + ((locals.var_temp_dn8 * assign42600_e48079) + (locals.var_temp * ((((locals.var_temp1_dn8 * locals.var_xalphab) - (locals.var_temp1 * locals.var_xalphab_dn8)) / (locals.var_xalphab * locals.var_xalphab)) / (2.0 * assign42600_e48077))))), (locals.var_xstars_dn9 + ((locals.var_temp_dn9 * assign42600_e48079) + (locals.var_temp * ((((locals.var_temp1_dn9 * locals.var_xalphab) - (locals.var_temp1 * locals.var_xalphab_dn9)) / (locals.var_xalphab * locals.var_xalphab)) / (2.0 * assign42600_e48077))))), );
            locals.var_xedgebs_rv = 0.0;
        }

        if (locals.var_guard1235 != 0.0) {
            let assign42610_e48090: f64 = (locals.var_temp2 / locals.var_xalphab);
            let assign42610_e48091: f64 = (1.0 + assign42610_e48090);
            let assign42610_e48092: f64 = (assign42610_e48091).sqrt();
            let assign42610_e48094: f64 = (assign42610_e48092 - 1.0);
            let assign42610_e48095: f64 = (locals.var_temp * assign42610_e48094);
            let assign42610_e48096: f64 = (locals.var_xstard + assign42610_e48095);
            (locals.var_xedgebd, locals.var_xedgebd_dn4, locals.var_xedgebd_dn6, locals.var_xedgebd_dn7, locals.var_xedgebd_dn8, locals.var_xedgebd_dn9, ) = (assign42610_e48096, (locals.var_xstard_dn4 + ((locals.var_temp_dn4 * assign42610_e48094) + (locals.var_temp * ((((locals.var_temp2_dn4 * locals.var_xalphab) - (locals.var_temp2 * locals.var_xalphab_dn4)) / (locals.var_xalphab * locals.var_xalphab)) / (2.0 * assign42610_e48092))))), (locals.var_xstard_dn6 + ((locals.var_temp_dn6 * assign42610_e48094) + (locals.var_temp * ((((locals.var_temp2_dn6 * locals.var_xalphab) - (locals.var_temp2 * locals.var_xalphab_dn6)) / (locals.var_xalphab * locals.var_xalphab)) / (2.0 * assign42610_e48092))))), (locals.var_xstard_dn7 + ((locals.var_temp_dn7 * assign42610_e48094) + (locals.var_temp * ((((locals.var_temp2_dn7 * locals.var_xalphab) - (locals.var_temp2 * locals.var_xalphab_dn7)) / (locals.var_xalphab * locals.var_xalphab)) / (2.0 * assign42610_e48092))))), (locals.var_xstard_dn8 + ((locals.var_temp_dn8 * assign42610_e48094) + (locals.var_temp * ((((locals.var_temp2_dn8 * locals.var_xalphab) - (locals.var_temp2 * locals.var_xalphab_dn8)) / (locals.var_xalphab * locals.var_xalphab)) / (2.0 * assign42610_e48092))))), (locals.var_xstard_dn9 + ((locals.var_temp_dn9 * assign42610_e48094) + (locals.var_temp * ((((locals.var_temp2_dn9 * locals.var_xalphab) - (locals.var_temp2 * locals.var_xalphab_dn9)) / (locals.var_xalphab * locals.var_xalphab)) / (2.0 * assign42610_e48092))))), );
            locals.var_xedgebd_rv = 0.0;
        }

        if (locals.var_guard1235 != 0.0) {
            let assign42620_e48102: f64 = (locals.var_fif_phit * locals.var_csiprime_ac);
            (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign42620_e48102, ((locals.var_fif_phit_dn4 * locals.var_csiprime_ac) + (locals.var_fif_phit * locals.var_csiprime_ac_dn4)), ((locals.var_fif_phit_dn6 * locals.var_csiprime_ac) + (locals.var_fif_phit * locals.var_csiprime_ac_dn6)), ((locals.var_fif_phit_dn7 * locals.var_csiprime_ac) + (locals.var_fif_phit * locals.var_csiprime_ac_dn7)), ((locals.var_fif_phit_dn8 * locals.var_csiprime_ac) + (locals.var_fif_phit * locals.var_csiprime_ac_dn8)), ((locals.var_fif_phit_dn9 * locals.var_csiprime_ac) + (locals.var_fif_phit * locals.var_csiprime_ac_dn9)), );
            locals.var_temp_rv = 0.0;
        }

        if (locals.var_guard1235 != 0.0) {
            let assign42630_e48107: f64 = (-locals.var_temp);
            let assign42630_e48109: f64 = (assign42630_e48107 * locals.var_lambdaf);
            let assign42630_e48111: f64 = (assign42630_e48109 * locals.var_k1_ac);
            let assign42630_e48113: f64 = (assign42630_e48111 * locals.var_sce1_ac);
            (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9, ) = (assign42630_e48113, (((((((-locals.var_temp_dn4) * locals.var_lambdaf) + (assign42630_e48107 * locals.var_lambdaf_dn4)) * locals.var_k1_ac) + (assign42630_e48109 * locals.var_k1_ac_dn4)) * locals.var_sce1_ac) + (assign42630_e48111 * locals.var_sce1_ac_dn4)), (((((((-locals.var_temp_dn6) * locals.var_lambdaf) + (assign42630_e48107 * locals.var_lambdaf_dn6)) * locals.var_k1_ac) + (assign42630_e48109 * locals.var_k1_ac_dn6)) * locals.var_sce1_ac) + (assign42630_e48111 * locals.var_sce1_ac_dn6)), (((((((-locals.var_temp_dn7) * locals.var_lambdaf) + (assign42630_e48107 * locals.var_lambdaf_dn7)) * locals.var_k1_ac) + (assign42630_e48109 * locals.var_k1_ac_dn7)) * locals.var_sce1_ac) + (assign42630_e48111 * locals.var_sce1_ac_dn7)), (((((((-locals.var_temp_dn8) * locals.var_lambdaf) + (assign42630_e48107 * locals.var_lambdaf_dn8)) * locals.var_k1_ac) + (assign42630_e48109 * locals.var_k1_ac_dn8)) * locals.var_sce1_ac) + (assign42630_e48111 * locals.var_sce1_ac_dn8)), (((((((-locals.var_temp_dn9) * locals.var_lambdaf) + (assign42630_e48107 * locals.var_lambdaf_dn9)) * locals.var_k1_ac) + (assign42630_e48109 * locals.var_k1_ac_dn9)) * locals.var_sce1_ac) + (assign42630_e48111 * locals.var_sce1_ac_dn9)), );
            locals.var_temp1_rv = 0.0;
        }

        if (locals.var_guard1235 != 0.0) {
            let assign42640_e48118: f64 = (-locals.var_temp);
            let assign42640_e48120: f64 = (assign42640_e48118 * locals.var_lambdab);
            let assign42640_e48122: f64 = (assign42640_e48120 * locals.var_k2_ac);
            let assign42640_e48124: f64 = (assign42640_e48122 * locals.var_sce2_ac);
            (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9, ) = (assign42640_e48124, (((((((-locals.var_temp_dn4) * locals.var_lambdab) + (assign42640_e48118 * locals.var_lambdab_dn4)) * locals.var_k2_ac) + (assign42640_e48120 * locals.var_k2_ac_dn4)) * locals.var_sce2_ac) + (assign42640_e48122 * locals.var_sce2_ac_dn4)), (((((((-locals.var_temp_dn6) * locals.var_lambdab) + (assign42640_e48118 * locals.var_lambdab_dn6)) * locals.var_k2_ac) + (assign42640_e48120 * locals.var_k2_ac_dn6)) * locals.var_sce2_ac) + (assign42640_e48122 * locals.var_sce2_ac_dn6)), (((((((-locals.var_temp_dn7) * locals.var_lambdab) + (assign42640_e48118 * locals.var_lambdab_dn7)) * locals.var_k2_ac) + (assign42640_e48120 * locals.var_k2_ac_dn7)) * locals.var_sce2_ac) + (assign42640_e48122 * locals.var_sce2_ac_dn7)), (((((((-locals.var_temp_dn8) * locals.var_lambdab) + (assign42640_e48118 * locals.var_lambdab_dn8)) * locals.var_k2_ac) + (assign42640_e48120 * locals.var_k2_ac_dn8)) * locals.var_sce2_ac) + (assign42640_e48122 * locals.var_sce2_ac_dn8)), (((((((-locals.var_temp_dn9) * locals.var_lambdab) + (assign42640_e48118 * locals.var_lambdab_dn9)) * locals.var_k2_ac) + (assign42640_e48120 * locals.var_k2_ac_dn9)) * locals.var_sce2_ac) + (assign42640_e48122 * locals.var_sce2_ac_dn9)), );
            locals.var_temp2_rv = 0.0;
        }

        if (locals.var_guard1235 != 0.0) {
            let assign42650_e48131: f64 = (locals.var_xedgefs - locals.var_xeffs);
            let assign42650_e48133: f64 = assign42650_e48131;
            let assign42650_e48136: f64 = (locals.var_xedgefs - locals.var_xeffs);
            let assign42650_e48138: f64 = assign42650_e48136;
            let assign42650_e48141: f64 = (locals.var_xedgefs - locals.var_xeffs);
            let assign42650_e48143: f64 = assign42650_e48141;
            let assign42650_e48144: f64 = (assign42650_e48138 * assign42650_e48143);
            let assign42650_e48146: f64 = (assign42650_e48144 + 1.0);
            let assign42650_e48147: f64 = (assign42650_e48146).sqrt();
            let assign42650_e48148: f64 = (assign42650_e48133 + assign42650_e48147);
            let assign42650_e48149: f64 = (0.5 * assign42650_e48148);
            (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign42650_e48149, (0.5 * ((locals.var_xedgefs_dn4 - locals.var_xeffs_dn4) + ((((locals.var_xedgefs_dn4 - locals.var_xeffs_dn4) * assign42650_e48143) + (assign42650_e48138 * (locals.var_xedgefs_dn4 - locals.var_xeffs_dn4))) / (2.0 * assign42650_e48147)))), (0.5 * ((locals.var_xedgefs_dn6 - locals.var_xeffs_dn6) + ((((locals.var_xedgefs_dn6 - locals.var_xeffs_dn6) * assign42650_e48143) + (assign42650_e48138 * (locals.var_xedgefs_dn6 - locals.var_xeffs_dn6))) / (2.0 * assign42650_e48147)))), (0.5 * ((locals.var_xedgefs_dn7 - locals.var_xeffs_dn7) + ((((locals.var_xedgefs_dn7 - locals.var_xeffs_dn7) * assign42650_e48143) + (assign42650_e48138 * (locals.var_xedgefs_dn7 - locals.var_xeffs_dn7))) / (2.0 * assign42650_e48147)))), (0.5 * ((locals.var_xedgefs_dn8 - locals.var_xeffs_dn8) + ((((locals.var_xedgefs_dn8 - locals.var_xeffs_dn8) * assign42650_e48143) + (assign42650_e48138 * (locals.var_xedgefs_dn8 - locals.var_xeffs_dn8))) / (2.0 * assign42650_e48147)))), (0.5 * ((locals.var_xedgefs_dn9 - locals.var_xeffs_dn9) + ((((locals.var_xedgefs_dn9 - locals.var_xeffs_dn9) * assign42650_e48143) + (assign42650_e48138 * (locals.var_xedgefs_dn9 - locals.var_xeffs_dn9))) / (2.0 * assign42650_e48147)))), );
            locals.var_temp_rv = 0.0;
        }

        if (locals.var_guard1235 != 0.0) {
            let assign42660_e48155: f64 = (locals.var_temp1 * locals.var_temp);
            let assign42660_e48157: f64 = (assign42660_e48155 * locals.var_temp);
            let assign42660_e48160: f64 = (locals.var_xedgefs - locals.var_xstars);
            let assign42660_e48161: f64 = (assign42660_e48157 / assign42660_e48160);
            (locals.var_qgsif, locals.var_qgsif_dn4, locals.var_qgsif_dn6, locals.var_qgsif_dn7, locals.var_qgsif_dn8, locals.var_qgsif_dn9, ) = (assign42660_e48161, (((((((locals.var_temp1_dn4 * locals.var_temp) + (locals.var_temp1 * locals.var_temp_dn4)) * locals.var_temp) + (assign42660_e48155 * locals.var_temp_dn4)) * assign42660_e48160) - (assign42660_e48157 * (locals.var_xedgefs_dn4 - locals.var_xstars_dn4))) / (assign42660_e48160 * assign42660_e48160)), (((((((locals.var_temp1_dn6 * locals.var_temp) + (locals.var_temp1 * locals.var_temp_dn6)) * locals.var_temp) + (assign42660_e48155 * locals.var_temp_dn6)) * assign42660_e48160) - (assign42660_e48157 * (locals.var_xedgefs_dn6 - locals.var_xstars_dn6))) / (assign42660_e48160 * assign42660_e48160)), (((((((locals.var_temp1_dn7 * locals.var_temp) + (locals.var_temp1 * locals.var_temp_dn7)) * locals.var_temp) + (assign42660_e48155 * locals.var_temp_dn7)) * assign42660_e48160) - (assign42660_e48157 * (locals.var_xedgefs_dn7 - locals.var_xstars_dn7))) / (assign42660_e48160 * assign42660_e48160)), (((((((locals.var_temp1_dn8 * locals.var_temp) + (locals.var_temp1 * locals.var_temp_dn8)) * locals.var_temp) + (assign42660_e48155 * locals.var_temp_dn8)) * assign42660_e48160) - (assign42660_e48157 * (locals.var_xedgefs_dn8 - locals.var_xstars_dn8))) / (assign42660_e48160 * assign42660_e48160)), (((((((locals.var_temp1_dn9 * locals.var_temp) + (locals.var_temp1 * locals.var_temp_dn9)) * locals.var_temp) + (assign42660_e48155 * locals.var_temp_dn9)) * assign42660_e48160) - (assign42660_e48157 * (locals.var_xedgefs_dn9 - locals.var_xstars_dn9))) / (assign42660_e48160 * assign42660_e48160)), );
            locals.var_qgsif_rv = 0.0;
        }

        if (locals.var_guard1235 != 0.0) {
            let assign42670_e48168: f64 = (locals.var_xedgefd - locals.var_xeffd);
            let assign42670_e48170: f64 = assign42670_e48168;
            let assign42670_e48173: f64 = (locals.var_xedgefd - locals.var_xeffd);
            let assign42670_e48175: f64 = assign42670_e48173;
            let assign42670_e48178: f64 = (locals.var_xedgefd - locals.var_xeffd);
            let assign42670_e48180: f64 = assign42670_e48178;
            let assign42670_e48181: f64 = (assign42670_e48175 * assign42670_e48180);
            let assign42670_e48183: f64 = (assign42670_e48181 + 1.0);
            let assign42670_e48184: f64 = (assign42670_e48183).sqrt();
            let assign42670_e48185: f64 = (assign42670_e48170 + assign42670_e48184);
            let assign42670_e48186: f64 = (0.5 * assign42670_e48185);
            (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign42670_e48186, (0.5 * ((locals.var_xedgefd_dn4 - locals.var_xeffd_dn4) + ((((locals.var_xedgefd_dn4 - locals.var_xeffd_dn4) * assign42670_e48180) + (assign42670_e48175 * (locals.var_xedgefd_dn4 - locals.var_xeffd_dn4))) / (2.0 * assign42670_e48184)))), (0.5 * ((locals.var_xedgefd_dn6 - locals.var_xeffd_dn6) + ((((locals.var_xedgefd_dn6 - locals.var_xeffd_dn6) * assign42670_e48180) + (assign42670_e48175 * (locals.var_xedgefd_dn6 - locals.var_xeffd_dn6))) / (2.0 * assign42670_e48184)))), (0.5 * ((locals.var_xedgefd_dn7 - locals.var_xeffd_dn7) + ((((locals.var_xedgefd_dn7 - locals.var_xeffd_dn7) * assign42670_e48180) + (assign42670_e48175 * (locals.var_xedgefd_dn7 - locals.var_xeffd_dn7))) / (2.0 * assign42670_e48184)))), (0.5 * ((locals.var_xedgefd_dn8 - locals.var_xeffd_dn8) + ((((locals.var_xedgefd_dn8 - locals.var_xeffd_dn8) * assign42670_e48180) + (assign42670_e48175 * (locals.var_xedgefd_dn8 - locals.var_xeffd_dn8))) / (2.0 * assign42670_e48184)))), (0.5 * ((locals.var_xedgefd_dn9 - locals.var_xeffd_dn9) + ((((locals.var_xedgefd_dn9 - locals.var_xeffd_dn9) * assign42670_e48180) + (assign42670_e48175 * (locals.var_xedgefd_dn9 - locals.var_xeffd_dn9))) / (2.0 * assign42670_e48184)))), );
            locals.var_temp_rv = 0.0;
        }

        if (locals.var_guard1235 != 0.0) {
            let assign42680_e48192: f64 = (locals.var_temp1 * locals.var_temp);
            let assign42680_e48194: f64 = (assign42680_e48192 * locals.var_temp);
            let assign42680_e48197: f64 = (locals.var_xedgefd - locals.var_xstard);
            let assign42680_e48198: f64 = (assign42680_e48194 / assign42680_e48197);
            (locals.var_qgdif, locals.var_qgdif_dn4, locals.var_qgdif_dn6, locals.var_qgdif_dn7, locals.var_qgdif_dn8, locals.var_qgdif_dn9, ) = (assign42680_e48198, (((((((locals.var_temp1_dn4 * locals.var_temp) + (locals.var_temp1 * locals.var_temp_dn4)) * locals.var_temp) + (assign42680_e48192 * locals.var_temp_dn4)) * assign42680_e48197) - (assign42680_e48194 * (locals.var_xedgefd_dn4 - locals.var_xstard_dn4))) / (assign42680_e48197 * assign42680_e48197)), (((((((locals.var_temp1_dn6 * locals.var_temp) + (locals.var_temp1 * locals.var_temp_dn6)) * locals.var_temp) + (assign42680_e48192 * locals.var_temp_dn6)) * assign42680_e48197) - (assign42680_e48194 * (locals.var_xedgefd_dn6 - locals.var_xstard_dn6))) / (assign42680_e48197 * assign42680_e48197)), (((((((locals.var_temp1_dn7 * locals.var_temp) + (locals.var_temp1 * locals.var_temp_dn7)) * locals.var_temp) + (assign42680_e48192 * locals.var_temp_dn7)) * assign42680_e48197) - (assign42680_e48194 * (locals.var_xedgefd_dn7 - locals.var_xstard_dn7))) / (assign42680_e48197 * assign42680_e48197)), (((((((locals.var_temp1_dn8 * locals.var_temp) + (locals.var_temp1 * locals.var_temp_dn8)) * locals.var_temp) + (assign42680_e48192 * locals.var_temp_dn8)) * assign42680_e48197) - (assign42680_e48194 * (locals.var_xedgefd_dn8 - locals.var_xstard_dn8))) / (assign42680_e48197 * assign42680_e48197)), (((((((locals.var_temp1_dn9 * locals.var_temp) + (locals.var_temp1 * locals.var_temp_dn9)) * locals.var_temp) + (assign42680_e48192 * locals.var_temp_dn9)) * assign42680_e48197) - (assign42680_e48194 * (locals.var_xedgefd_dn9 - locals.var_xstard_dn9))) / (assign42680_e48197 * assign42680_e48197)), );
            locals.var_qgdif_rv = 0.0;
        }

        if (locals.var_guard1235 != 0.0) {
            let assign42690_e48205: f64 = (locals.var_xedgebs - locals.var_xeffs);
            let assign42690_e48207: f64 = assign42690_e48205;
            let assign42690_e48210: f64 = (locals.var_xedgebs - locals.var_xeffs);
            let assign42690_e48212: f64 = assign42690_e48210;
            let assign42690_e48215: f64 = (locals.var_xedgebs - locals.var_xeffs);
            let assign42690_e48217: f64 = assign42690_e48215;
            let assign42690_e48218: f64 = (assign42690_e48212 * assign42690_e48217);
            let assign42690_e48220: f64 = (assign42690_e48218 + 1.0);
            let assign42690_e48221: f64 = (assign42690_e48220).sqrt();
            let assign42690_e48222: f64 = (assign42690_e48207 + assign42690_e48221);
            let assign42690_e48223: f64 = (0.5 * assign42690_e48222);
            (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign42690_e48223, (0.5 * ((locals.var_xedgebs_dn4 - locals.var_xeffs_dn4) + ((((locals.var_xedgebs_dn4 - locals.var_xeffs_dn4) * assign42690_e48217) + (assign42690_e48212 * (locals.var_xedgebs_dn4 - locals.var_xeffs_dn4))) / (2.0 * assign42690_e48221)))), (0.5 * ((locals.var_xedgebs_dn6 - locals.var_xeffs_dn6) + ((((locals.var_xedgebs_dn6 - locals.var_xeffs_dn6) * assign42690_e48217) + (assign42690_e48212 * (locals.var_xedgebs_dn6 - locals.var_xeffs_dn6))) / (2.0 * assign42690_e48221)))), (0.5 * ((locals.var_xedgebs_dn7 - locals.var_xeffs_dn7) + ((((locals.var_xedgebs_dn7 - locals.var_xeffs_dn7) * assign42690_e48217) + (assign42690_e48212 * (locals.var_xedgebs_dn7 - locals.var_xeffs_dn7))) / (2.0 * assign42690_e48221)))), (0.5 * ((locals.var_xedgebs_dn8 - locals.var_xeffs_dn8) + ((((locals.var_xedgebs_dn8 - locals.var_xeffs_dn8) * assign42690_e48217) + (assign42690_e48212 * (locals.var_xedgebs_dn8 - locals.var_xeffs_dn8))) / (2.0 * assign42690_e48221)))), (0.5 * ((locals.var_xedgebs_dn9 - locals.var_xeffs_dn9) + ((((locals.var_xedgebs_dn9 - locals.var_xeffs_dn9) * assign42690_e48217) + (assign42690_e48212 * (locals.var_xedgebs_dn9 - locals.var_xeffs_dn9))) / (2.0 * assign42690_e48221)))), );
            locals.var_temp_rv = 0.0;
        }

        if (locals.var_guard1235 != 0.0) {
            let assign42700_e48229: f64 = (locals.var_temp2 * locals.var_temp);
            let assign42700_e48231: f64 = (assign42700_e48229 * locals.var_temp);
            let assign42700_e48234: f64 = (locals.var_xedgebs - locals.var_xstars);
            let assign42700_e48235: f64 = (assign42700_e48231 / assign42700_e48234);
            (locals.var_qbsif, locals.var_qbsif_dn4, locals.var_qbsif_dn6, locals.var_qbsif_dn7, locals.var_qbsif_dn8, locals.var_qbsif_dn9, ) = (assign42700_e48235, (((((((locals.var_temp2_dn4 * locals.var_temp) + (locals.var_temp2 * locals.var_temp_dn4)) * locals.var_temp) + (assign42700_e48229 * locals.var_temp_dn4)) * assign42700_e48234) - (assign42700_e48231 * (locals.var_xedgebs_dn4 - locals.var_xstars_dn4))) / (assign42700_e48234 * assign42700_e48234)), (((((((locals.var_temp2_dn6 * locals.var_temp) + (locals.var_temp2 * locals.var_temp_dn6)) * locals.var_temp) + (assign42700_e48229 * locals.var_temp_dn6)) * assign42700_e48234) - (assign42700_e48231 * (locals.var_xedgebs_dn6 - locals.var_xstars_dn6))) / (assign42700_e48234 * assign42700_e48234)), (((((((locals.var_temp2_dn7 * locals.var_temp) + (locals.var_temp2 * locals.var_temp_dn7)) * locals.var_temp) + (assign42700_e48229 * locals.var_temp_dn7)) * assign42700_e48234) - (assign42700_e48231 * (locals.var_xedgebs_dn7 - locals.var_xstars_dn7))) / (assign42700_e48234 * assign42700_e48234)), (((((((locals.var_temp2_dn8 * locals.var_temp) + (locals.var_temp2 * locals.var_temp_dn8)) * locals.var_temp) + (assign42700_e48229 * locals.var_temp_dn8)) * assign42700_e48234) - (assign42700_e48231 * (locals.var_xedgebs_dn8 - locals.var_xstars_dn8))) / (assign42700_e48234 * assign42700_e48234)), (((((((locals.var_temp2_dn9 * locals.var_temp) + (locals.var_temp2 * locals.var_temp_dn9)) * locals.var_temp) + (assign42700_e48229 * locals.var_temp_dn9)) * assign42700_e48234) - (assign42700_e48231 * (locals.var_xedgebs_dn9 - locals.var_xstars_dn9))) / (assign42700_e48234 * assign42700_e48234)), );
            locals.var_qbsif_rv = 0.0;
        }

        if (locals.var_guard1235 != 0.0) {
            let assign42710_e48242: f64 = (locals.var_xedgebd - locals.var_xeffd);
            let assign42710_e48244: f64 = assign42710_e48242;
            let assign42710_e48247: f64 = (locals.var_xedgebd - locals.var_xeffd);
            let assign42710_e48249: f64 = assign42710_e48247;
            let assign42710_e48252: f64 = (locals.var_xedgebd - locals.var_xeffd);
            let assign42710_e48254: f64 = assign42710_e48252;
            let assign42710_e48255: f64 = (assign42710_e48249 * assign42710_e48254);
            let assign42710_e48257: f64 = (assign42710_e48255 + 1.0);
            let assign42710_e48258: f64 = (assign42710_e48257).sqrt();
            let assign42710_e48259: f64 = (assign42710_e48244 + assign42710_e48258);
            let assign42710_e48260: f64 = (0.5 * assign42710_e48259);
            (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign42710_e48260, (0.5 * ((locals.var_xedgebd_dn4 - locals.var_xeffd_dn4) + ((((locals.var_xedgebd_dn4 - locals.var_xeffd_dn4) * assign42710_e48254) + (assign42710_e48249 * (locals.var_xedgebd_dn4 - locals.var_xeffd_dn4))) / (2.0 * assign42710_e48258)))), (0.5 * ((locals.var_xedgebd_dn6 - locals.var_xeffd_dn6) + ((((locals.var_xedgebd_dn6 - locals.var_xeffd_dn6) * assign42710_e48254) + (assign42710_e48249 * (locals.var_xedgebd_dn6 - locals.var_xeffd_dn6))) / (2.0 * assign42710_e48258)))), (0.5 * ((locals.var_xedgebd_dn7 - locals.var_xeffd_dn7) + ((((locals.var_xedgebd_dn7 - locals.var_xeffd_dn7) * assign42710_e48254) + (assign42710_e48249 * (locals.var_xedgebd_dn7 - locals.var_xeffd_dn7))) / (2.0 * assign42710_e48258)))), (0.5 * ((locals.var_xedgebd_dn8 - locals.var_xeffd_dn8) + ((((locals.var_xedgebd_dn8 - locals.var_xeffd_dn8) * assign42710_e48254) + (assign42710_e48249 * (locals.var_xedgebd_dn8 - locals.var_xeffd_dn8))) / (2.0 * assign42710_e48258)))), (0.5 * ((locals.var_xedgebd_dn9 - locals.var_xeffd_dn9) + ((((locals.var_xedgebd_dn9 - locals.var_xeffd_dn9) * assign42710_e48254) + (assign42710_e48249 * (locals.var_xedgebd_dn9 - locals.var_xeffd_dn9))) / (2.0 * assign42710_e48258)))), );
            locals.var_temp_rv = 0.0;
        }

        if (locals.var_guard1235 != 0.0) {
            let assign42720_e48266: f64 = (locals.var_temp2 * locals.var_temp);
            let assign42720_e48268: f64 = (assign42720_e48266 * locals.var_temp);
            let assign42720_e48271: f64 = (locals.var_xedgebd - locals.var_xstard);
            let assign42720_e48272: f64 = (assign42720_e48268 / assign42720_e48271);
            (locals.var_qbdif, locals.var_qbdif_dn4, locals.var_qbdif_dn6, locals.var_qbdif_dn7, locals.var_qbdif_dn8, locals.var_qbdif_dn9, ) = (assign42720_e48272, (((((((locals.var_temp2_dn4 * locals.var_temp) + (locals.var_temp2 * locals.var_temp_dn4)) * locals.var_temp) + (assign42720_e48266 * locals.var_temp_dn4)) * assign42720_e48271) - (assign42720_e48268 * (locals.var_xedgebd_dn4 - locals.var_xstard_dn4))) / (assign42720_e48271 * assign42720_e48271)), (((((((locals.var_temp2_dn6 * locals.var_temp) + (locals.var_temp2 * locals.var_temp_dn6)) * locals.var_temp) + (assign42720_e48266 * locals.var_temp_dn6)) * assign42720_e48271) - (assign42720_e48268 * (locals.var_xedgebd_dn6 - locals.var_xstard_dn6))) / (assign42720_e48271 * assign42720_e48271)), (((((((locals.var_temp2_dn7 * locals.var_temp) + (locals.var_temp2 * locals.var_temp_dn7)) * locals.var_temp) + (assign42720_e48266 * locals.var_temp_dn7)) * assign42720_e48271) - (assign42720_e48268 * (locals.var_xedgebd_dn7 - locals.var_xstard_dn7))) / (assign42720_e48271 * assign42720_e48271)), (((((((locals.var_temp2_dn8 * locals.var_temp) + (locals.var_temp2 * locals.var_temp_dn8)) * locals.var_temp) + (assign42720_e48266 * locals.var_temp_dn8)) * assign42720_e48271) - (assign42720_e48268 * (locals.var_xedgebd_dn8 - locals.var_xstard_dn8))) / (assign42720_e48271 * assign42720_e48271)), (((((((locals.var_temp2_dn9 * locals.var_temp) + (locals.var_temp2 * locals.var_temp_dn9)) * locals.var_temp) + (assign42720_e48266 * locals.var_temp_dn9)) * assign42720_e48271) - (assign42720_e48268 * (locals.var_xedgebd_dn9 - locals.var_xstard_dn9))) / (assign42720_e48271 * assign42720_e48271)), );
            locals.var_qbdif_rv = 0.0;
        }

        if (locals.var_guard1235 == 0.0) {
            (locals.var_qgsif, locals.var_qgsif_dn4, locals.var_qgsif_dn6, locals.var_qgsif_dn7, locals.var_qgsif_dn8, locals.var_qgsif_dn9, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_qgsif_rv = 0.0;
            (locals.var_qgdif, locals.var_qgdif_dn4, locals.var_qgdif_dn6, locals.var_qgdif_dn7, locals.var_qgdif_dn8, locals.var_qgdif_dn9, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_qgdif_rv = 0.0;
            (locals.var_qbsif, locals.var_qbsif_dn4, locals.var_qbsif_dn6, locals.var_qbsif_dn7, locals.var_qbsif_dn8, locals.var_qbsif_dn9, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_qbsif_rv = 0.0;
            (locals.var_qbdif, locals.var_qbdif_dn4, locals.var_qbdif_dn6, locals.var_qbdif_dn7, locals.var_qbdif_dn8, locals.var_qbdif_dn9, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_qbdif_rv = 0.0;
        }

        let assign42770_e48297: f64 = (locals.var_cfr_i * locals.var_vgsu);
        (locals.var_qgse, locals.var_qgse_dn4, locals.var_qgse_dn6, locals.var_qgse_dn7, locals.var_qgse_dn8, locals.var_qgse_dn9, ) = (assign42770_e48297, (locals.var_cfr_i_dn4 * locals.var_vgsu), ((locals.var_cfr_i_dn6 * locals.var_vgsu) + (locals.var_cfr_i * locals.var_vgsu_dn6)), (locals.var_cfr_i_dn7 * locals.var_vgsu), (locals.var_cfr_i_dn8 * locals.var_vgsu), ((locals.var_cfr_i_dn9 * locals.var_vgsu) + (locals.var_cfr_i * locals.var_vgsu_dn9)), );
        locals.var_qgse_rv = 0.0;

        let assign42780_e48300: f64 = (locals.var_cfrd_i * locals.var_vgdu);
        (locals.var_qgde, locals.var_qgde_dn4, locals.var_qgde_dn6, locals.var_qgde_dn7, locals.var_qgde_dn8, locals.var_qgde_dn9, ) = (assign42780_e48300, (locals.var_cfrd_i_dn4 * locals.var_vgdu), ((locals.var_cfrd_i_dn6 * locals.var_vgdu) + (locals.var_cfrd_i * locals.var_vgdu_dn6)), ((locals.var_cfrd_i_dn7 * locals.var_vgdu) + (locals.var_cfrd_i * locals.var_vgdu_dn7)), (locals.var_cfrd_i_dn8 * locals.var_vgdu), ((locals.var_cfrd_i_dn9 * locals.var_vgdu) + (locals.var_cfrd_i * locals.var_vgdu_dn9)), );
        locals.var_qgde_rv = 0.0;

        let assign42790_e48305: f64 = (locals.var_covdl_i * locals.var_dleff_ac);
        let assign42790_e48309: f64 = (locals.var_covdlb_i * locals.var_xg20shift_ac);
        let assign42790_e48310: f64 = (1.0 - assign42790_e48309);
        let assign42790_e48311: f64 = (assign42790_e48305 * assign42790_e48310);
        let assign42790_e48312: f64 = (1.0 - assign42790_e48311);
        let assign42790_e48314: f64 = assign42790_e48312;
        let assign42790_e48318: f64 = (locals.var_covdl_i * locals.var_dleff_ac);
        let assign42790_e48322: f64 = (locals.var_covdlb_i * locals.var_xg20shift_ac);
        let assign42790_e48323: f64 = (1.0 - assign42790_e48322);
        let assign42790_e48324: f64 = (assign42790_e48318 * assign42790_e48323);
        let assign42790_e48325: f64 = (1.0 - assign42790_e48324);
        let assign42790_e48327: f64 = assign42790_e48325;
        let assign42790_e48331: f64 = (locals.var_covdl_i * locals.var_dleff_ac);
        let assign42790_e48335: f64 = (locals.var_covdlb_i * locals.var_xg20shift_ac);
        let assign42790_e48336: f64 = (1.0 - assign42790_e48335);
        let assign42790_e48337: f64 = (assign42790_e48331 * assign42790_e48336);
        let assign42790_e48338: f64 = (1.0 - assign42790_e48337);
        let assign42790_e48340: f64 = assign42790_e48338;
        let assign42790_e48341: f64 = (assign42790_e48327 * assign42790_e48340);
        let assign42790_e48343: f64 = (assign42790_e48341 + 0.2);
        let assign42790_e48344: f64 = (assign42790_e48343).sqrt();
        let assign42790_e48345: f64 = (assign42790_e48314 + assign42790_e48344);
        let assign42790_e48346: f64 = (0.5 * assign42790_e48345);
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign42790_e48346, (0.5 * ((-(((locals.var_covdl_i * locals.var_dleff_ac_dn4) * assign42790_e48310) + (assign42790_e48305 * (-(locals.var_covdlb_i * locals.var_xg20shift_ac_dn4))))) + ((((-(((locals.var_covdl_i * locals.var_dleff_ac_dn4) * assign42790_e48323) + (assign42790_e48318 * (-(locals.var_covdlb_i * locals.var_xg20shift_ac_dn4))))) * assign42790_e48340) + (assign42790_e48327 * (-(((locals.var_covdl_i * locals.var_dleff_ac_dn4) * assign42790_e48336) + (assign42790_e48331 * (-(locals.var_covdlb_i * locals.var_xg20shift_ac_dn4))))))) / (2.0 * assign42790_e48344)))), (0.5 * ((-(((locals.var_covdl_i * locals.var_dleff_ac_dn6) * assign42790_e48310) + (assign42790_e48305 * (-(locals.var_covdlb_i * locals.var_xg20shift_ac_dn6))))) + ((((-(((locals.var_covdl_i * locals.var_dleff_ac_dn6) * assign42790_e48323) + (assign42790_e48318 * (-(locals.var_covdlb_i * locals.var_xg20shift_ac_dn6))))) * assign42790_e48340) + (assign42790_e48327 * (-(((locals.var_covdl_i * locals.var_dleff_ac_dn6) * assign42790_e48336) + (assign42790_e48331 * (-(locals.var_covdlb_i * locals.var_xg20shift_ac_dn6))))))) / (2.0 * assign42790_e48344)))), (0.5 * ((-(((locals.var_covdl_i * locals.var_dleff_ac_dn7) * assign42790_e48310) + (assign42790_e48305 * (-(locals.var_covdlb_i * locals.var_xg20shift_ac_dn7))))) + ((((-(((locals.var_covdl_i * locals.var_dleff_ac_dn7) * assign42790_e48323) + (assign42790_e48318 * (-(locals.var_covdlb_i * locals.var_xg20shift_ac_dn7))))) * assign42790_e48340) + (assign42790_e48327 * (-(((locals.var_covdl_i * locals.var_dleff_ac_dn7) * assign42790_e48336) + (assign42790_e48331 * (-(locals.var_covdlb_i * locals.var_xg20shift_ac_dn7))))))) / (2.0 * assign42790_e48344)))), (0.5 * ((-(((locals.var_covdl_i * locals.var_dleff_ac_dn8) * assign42790_e48310) + (assign42790_e48305 * (-(locals.var_covdlb_i * locals.var_xg20shift_ac_dn8))))) + ((((-(((locals.var_covdl_i * locals.var_dleff_ac_dn8) * assign42790_e48323) + (assign42790_e48318 * (-(locals.var_covdlb_i * locals.var_xg20shift_ac_dn8))))) * assign42790_e48340) + (assign42790_e48327 * (-(((locals.var_covdl_i * locals.var_dleff_ac_dn8) * assign42790_e48336) + (assign42790_e48331 * (-(locals.var_covdlb_i * locals.var_xg20shift_ac_dn8))))))) / (2.0 * assign42790_e48344)))), (0.5 * ((-(((locals.var_covdl_i * locals.var_dleff_ac_dn9) * assign42790_e48310) + (assign42790_e48305 * (-(locals.var_covdlb_i * locals.var_xg20shift_ac_dn9))))) + ((((-(((locals.var_covdl_i * locals.var_dleff_ac_dn9) * assign42790_e48323) + (assign42790_e48318 * (-(locals.var_covdlb_i * locals.var_xg20shift_ac_dn9))))) * assign42790_e48340) + (assign42790_e48327 * (-(((locals.var_covdl_i * locals.var_dleff_ac_dn9) * assign42790_e48336) + (assign42790_e48331 * (-(locals.var_covdlb_i * locals.var_xg20shift_ac_dn9))))))) / (2.0 * assign42790_e48344)))), );
        locals.var_temp_rv = 0.0;

        let assign42800_e48349: f64 = (locals.var_cov_i * locals.var_vovscv);
        let assign42800_e48351: f64 = (assign42800_e48349 * locals.var_temp);
        (locals.var_qovs, locals.var_qovs_dn4, locals.var_qovs_dn6, locals.var_qovs_dn7, locals.var_qovs_dn8, locals.var_qovs_dn9, ) = (assign42800_e48351, ((((locals.var_cov_i_dn4 * locals.var_vovscv) + (locals.var_cov_i * locals.var_vovscv_dn4)) * locals.var_temp) + (assign42800_e48349 * locals.var_temp_dn4)), ((((locals.var_cov_i_dn6 * locals.var_vovscv) + (locals.var_cov_i * locals.var_vovscv_dn6)) * locals.var_temp) + (assign42800_e48349 * locals.var_temp_dn6)), ((((locals.var_cov_i_dn7 * locals.var_vovscv) + (locals.var_cov_i * locals.var_vovscv_dn7)) * locals.var_temp) + (assign42800_e48349 * locals.var_temp_dn7)), ((((locals.var_cov_i_dn8 * locals.var_vovscv) + (locals.var_cov_i * locals.var_vovscv_dn8)) * locals.var_temp) + (assign42800_e48349 * locals.var_temp_dn8)), ((((locals.var_cov_i_dn9 * locals.var_vovscv) + (locals.var_cov_i * locals.var_vovscv_dn9)) * locals.var_temp) + (assign42800_e48349 * locals.var_temp_dn9)), );
        locals.var_qovs_rv = 0.0;

        let assign42810_e48354: f64 = (locals.var_covd_i * locals.var_vovdcv);
        let assign42810_e48356: f64 = (assign42810_e48354 * locals.var_temp);
        (locals.var_qovd, locals.var_qovd_dn4, locals.var_qovd_dn6, locals.var_qovd_dn7, locals.var_qovd_dn8, locals.var_qovd_dn9, ) = (assign42810_e48356, ((((locals.var_covd_i_dn4 * locals.var_vovdcv) + (locals.var_covd_i * locals.var_vovdcv_dn4)) * locals.var_temp) + (assign42810_e48354 * locals.var_temp_dn4)), ((((locals.var_covd_i_dn6 * locals.var_vovdcv) + (locals.var_covd_i * locals.var_vovdcv_dn6)) * locals.var_temp) + (assign42810_e48354 * locals.var_temp_dn6)), ((((locals.var_covd_i_dn7 * locals.var_vovdcv) + (locals.var_covd_i * locals.var_vovdcv_dn7)) * locals.var_temp) + (assign42810_e48354 * locals.var_temp_dn7)), ((((locals.var_covd_i_dn8 * locals.var_vovdcv) + (locals.var_covd_i * locals.var_vovdcv_dn8)) * locals.var_temp) + (assign42810_e48354 * locals.var_temp_dn8)), ((((locals.var_covd_i_dn9 * locals.var_vovdcv) + (locals.var_covd_i * locals.var_vovdcv_dn9)) * locals.var_temp) + (assign42810_e48354 * locals.var_temp_dn9)), );
        locals.var_qovd_rv = 0.0;

        let assign42820_e48359: f64 = (locals.var_cgbov_i * locals.var_vgb);
        (locals.var_qgbe, locals.var_qgbe_dn4, locals.var_qgbe_dn6, locals.var_qgbe_dn7, locals.var_qgbe_dn8, locals.var_qgbe_dn9, ) = (assign42820_e48359, (locals.var_cgbov_i_dn4 * locals.var_vgb), ((locals.var_cgbov_i_dn6 * locals.var_vgb) + (locals.var_cgbov_i * locals.var_vgb_dn6)), ((locals.var_cgbov_i_dn7 * locals.var_vgb) + (locals.var_cgbov_i * locals.var_vgb_dn7)), ((locals.var_cgbov_i_dn8 * locals.var_vgb) + (locals.var_cgbov_i * locals.var_vgb_dn8)), ((locals.var_cgbov_i_dn9 * locals.var_vgb) + (locals.var_cgbov_i * locals.var_vgb_dn9)), );
        locals.var_qgbe_rv = 0.0;

        let assign42830_e48362: f64 = (locals.var_csd_i * locals.var_vds);
        (locals.var_qdse, locals.var_qdse_dn6, locals.var_qdse_dn7, ) = (assign42830_e48362, (locals.var_csd_i * locals.var_vds_dn6), (locals.var_csd_i * locals.var_vds_dn7), );
        locals.var_qdse_rv = 0.0;

        let assign42840_e48365: f64 = (locals.var_cox2init * locals.var_asource_i);
        let assign42840_e48368: f64 = (locals.var_csdbp_i * locals.var_psource_i);
        let assign42840_e48369: f64 = (assign42840_e48365 + assign42840_e48368);
        let assign42840_e48370: f64 = (-assign42840_e48369);
        let assign42840_e48372: f64 = (assign42840_e48370 * locals.var_vsbu);
        (locals.var_qssub, locals.var_qssub_dn6, locals.var_qssub_dn8, ) = (assign42840_e48372, (assign42840_e48370 * locals.var_vsbu_dn6), (assign42840_e48370 * locals.var_vsbu_dn8), );
        locals.var_qssub_rv = 0.0;

        let assign42850_e48375: f64 = (locals.var_cox2init * locals.var_adrain_i);
        let assign42850_e48378: f64 = (locals.var_csdbp_i * locals.var_pdrain_i);
        let assign42850_e48379: f64 = (assign42850_e48375 + assign42850_e48378);
        let assign42850_e48380: f64 = (-assign42850_e48379);
        let assign42850_e48382: f64 = (assign42850_e48380 * locals.var_vdbu);
        (locals.var_qdsub, locals.var_qdsub_dn6, locals.var_qdsub_dn7, locals.var_qdsub_dn8, ) = (assign42850_e48382, (assign42850_e48380 * locals.var_vdbu_dn6), (assign42850_e48380 * locals.var_vdbu_dn7), (assign42850_e48380 * locals.var_vdbu_dn8), );
        locals.var_qdsub_rv = 0.0;

        (locals.var_qg_wo_mult, locals.var_qg_wo_mult_dn4, locals.var_qg_wo_mult_dn6, locals.var_qg_wo_mult_dn7, locals.var_qg_wo_mult_dn8, locals.var_qg_wo_mult_dn9, ) = (locals.var_qg, locals.var_qg_dn4, locals.var_qg_dn6, locals.var_qg_dn7, locals.var_qg_dn8, locals.var_qg_dn9, );
        locals.var_qg_wo_mult_rv = 0.0;

        (locals.var_qb_wo_mult, locals.var_qb_wo_mult_dn4, locals.var_qb_wo_mult_dn6, locals.var_qb_wo_mult_dn7, locals.var_qb_wo_mult_dn8, locals.var_qb_wo_mult_dn9, ) = (locals.var_qb, locals.var_qb_dn4, locals.var_qb_dn6, locals.var_qb_dn7, locals.var_qb_dn8, locals.var_qb_dn9, );
        locals.var_qb_wo_mult_rv = 0.0;

        (locals.var_qd_wo_mult, locals.var_qd_wo_mult_dn4, locals.var_qd_wo_mult_dn6, locals.var_qd_wo_mult_dn7, locals.var_qd_wo_mult_dn8, locals.var_qd_wo_mult_dn9, ) = (locals.var_qd, locals.var_qd_dn4, locals.var_qd_dn6, locals.var_qd_dn7, locals.var_qd_dn8, locals.var_qd_dn9, );
        locals.var_qd_wo_mult_rv = 0.0;

        let assign43040_e48452: f64 = (locals.var_qg + locals.var_qb);
        let assign43040_e48454: f64 = (assign43040_e48452 + locals.var_qd);
        let assign43040_e48455: f64 = (-assign43040_e48454);
        (locals.var_qs, locals.var_qs_dn4, locals.var_qs_dn6, locals.var_qs_dn7, locals.var_qs_dn8, locals.var_qs_dn9, ) = (assign43040_e48455, (-((locals.var_qg_dn4 + locals.var_qb_dn4) + locals.var_qd_dn4)), (-((locals.var_qg_dn6 + locals.var_qb_dn6) + locals.var_qd_dn6)), (-((locals.var_qg_dn7 + locals.var_qb_dn7) + locals.var_qd_dn7)), (-((locals.var_qg_dn8 + locals.var_qb_dn8) + locals.var_qd_dn8)), (-((locals.var_qg_dn9 + locals.var_qb_dn9) + locals.var_qd_dn9)), );
        locals.var_qs_rv = 0.0;

        let assign43050_e48458: f64 = if locals.var_sigvds < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1249 = assign43050_e48458;
        locals.var_guard1249_rv = 0.0;

        if (locals.var_guard1249 != 0.0) {
            (locals.var_qd_wo_mult, locals.var_qd_wo_mult_dn4, locals.var_qd_wo_mult_dn6, locals.var_qd_wo_mult_dn7, locals.var_qd_wo_mult_dn8, locals.var_qd_wo_mult_dn9, ) = (locals.var_qs, locals.var_qs_dn4, locals.var_qs_dn6, locals.var_qs_dn7, locals.var_qs_dn8, locals.var_qs_dn9, );
            locals.var_qd_wo_mult_rv = 0.0;
        }

        let assign43070_e48465: f64 = (p.p32 * locals.var_mult_i_int);
        let assign43070_e48467: f64 = (assign43070_e48465 * locals.var_qg);
        (locals.var_qg, locals.var_qg_dn4, locals.var_qg_dn6, locals.var_qg_dn7, locals.var_qg_dn8, locals.var_qg_dn9, ) = (assign43070_e48467, (assign43070_e48465 * locals.var_qg_dn4), (assign43070_e48465 * locals.var_qg_dn6), (assign43070_e48465 * locals.var_qg_dn7), (assign43070_e48465 * locals.var_qg_dn8), (assign43070_e48465 * locals.var_qg_dn9), );
        locals.var_qg_rv = 0.0;

        let assign43080_e48470: f64 = (p.p32 * locals.var_mult_i_int);
        let assign43080_e48472: f64 = (assign43080_e48470 * locals.var_qb);
        (locals.var_qb, locals.var_qb_dn4, locals.var_qb_dn6, locals.var_qb_dn7, locals.var_qb_dn8, locals.var_qb_dn9, ) = (assign43080_e48472, (assign43080_e48470 * locals.var_qb_dn4), (assign43080_e48470 * locals.var_qb_dn6), (assign43080_e48470 * locals.var_qb_dn7), (assign43080_e48470 * locals.var_qb_dn8), (assign43080_e48470 * locals.var_qb_dn9), );
        locals.var_qb_rv = 0.0;

        let assign43090_e48475: f64 = (p.p32 * locals.var_mult_i_int);
        let assign43090_e48477: f64 = (assign43090_e48475 * locals.var_qd);
        (locals.var_qd, locals.var_qd_dn4, locals.var_qd_dn6, locals.var_qd_dn7, locals.var_qd_dn8, locals.var_qd_dn9, ) = (assign43090_e48477, (assign43090_e48475 * locals.var_qd_dn4), (assign43090_e48475 * locals.var_qd_dn6), (assign43090_e48475 * locals.var_qd_dn7), (assign43090_e48475 * locals.var_qd_dn8), (assign43090_e48475 * locals.var_qd_dn9), );
        locals.var_qd_rv = 0.0;

        let assign43100_e48480: f64 = (locals.var_qg + locals.var_qb);
        let assign43100_e48482: f64 = (assign43100_e48480 + locals.var_qd);
        let assign43100_e48483: f64 = (-assign43100_e48482);
        (locals.var_qs, locals.var_qs_dn4, locals.var_qs_dn6, locals.var_qs_dn7, locals.var_qs_dn8, locals.var_qs_dn9, ) = (assign43100_e48483, (-((locals.var_qg_dn4 + locals.var_qb_dn4) + locals.var_qd_dn4)), (-((locals.var_qg_dn6 + locals.var_qb_dn6) + locals.var_qd_dn6)), (-((locals.var_qg_dn7 + locals.var_qb_dn7) + locals.var_qd_dn7)), (-((locals.var_qg_dn8 + locals.var_qb_dn8) + locals.var_qd_dn8)), (-((locals.var_qg_dn9 + locals.var_qb_dn9) + locals.var_qd_dn9)), );
        locals.var_qs_rv = 0.0;

        let assign43110_e48486: f64 = (p.p32 * locals.var_mult_i_int);
        let assign43110_e48488: f64 = (assign43110_e48486 * locals.var_qgsif);
        (locals.var_qgsif, locals.var_qgsif_dn4, locals.var_qgsif_dn6, locals.var_qgsif_dn7, locals.var_qgsif_dn8, locals.var_qgsif_dn9, ) = (assign43110_e48488, (assign43110_e48486 * locals.var_qgsif_dn4), (assign43110_e48486 * locals.var_qgsif_dn6), (assign43110_e48486 * locals.var_qgsif_dn7), (assign43110_e48486 * locals.var_qgsif_dn8), (assign43110_e48486 * locals.var_qgsif_dn9), );
        locals.var_qgsif_rv = 0.0;

        let assign43120_e48491: f64 = (p.p32 * locals.var_mult_i_int);
        let assign43120_e48493: f64 = (assign43120_e48491 * locals.var_qgdif);
        (locals.var_qgdif, locals.var_qgdif_dn4, locals.var_qgdif_dn6, locals.var_qgdif_dn7, locals.var_qgdif_dn8, locals.var_qgdif_dn9, ) = (assign43120_e48493, (assign43120_e48491 * locals.var_qgdif_dn4), (assign43120_e48491 * locals.var_qgdif_dn6), (assign43120_e48491 * locals.var_qgdif_dn7), (assign43120_e48491 * locals.var_qgdif_dn8), (assign43120_e48491 * locals.var_qgdif_dn9), );
        locals.var_qgdif_rv = 0.0;

        let assign43130_e48496: f64 = (p.p32 * locals.var_mult_i_int);
        let assign43130_e48498: f64 = (assign43130_e48496 * locals.var_qbsif);
        (locals.var_qbsif, locals.var_qbsif_dn4, locals.var_qbsif_dn6, locals.var_qbsif_dn7, locals.var_qbsif_dn8, locals.var_qbsif_dn9, ) = (assign43130_e48498, (assign43130_e48496 * locals.var_qbsif_dn4), (assign43130_e48496 * locals.var_qbsif_dn6), (assign43130_e48496 * locals.var_qbsif_dn7), (assign43130_e48496 * locals.var_qbsif_dn8), (assign43130_e48496 * locals.var_qbsif_dn9), );
        locals.var_qbsif_rv = 0.0;

        let assign43140_e48501: f64 = (p.p32 * locals.var_mult_i_int);
        let assign43140_e48503: f64 = (assign43140_e48501 * locals.var_qbdif);
        (locals.var_qbdif, locals.var_qbdif_dn4, locals.var_qbdif_dn6, locals.var_qbdif_dn7, locals.var_qbdif_dn8, locals.var_qbdif_dn9, ) = (assign43140_e48503, (assign43140_e48501 * locals.var_qbdif_dn4), (assign43140_e48501 * locals.var_qbdif_dn6), (assign43140_e48501 * locals.var_qbdif_dn7), (assign43140_e48501 * locals.var_qbdif_dn8), (assign43140_e48501 * locals.var_qbdif_dn9), );
        locals.var_qbdif_rv = 0.0;

        let assign43150_e48506: f64 = (p.p32 * locals.var_mult_i_int);
        let assign43150_e48508: f64 = (assign43150_e48506 * locals.var_qgse);
        (locals.var_qgse, locals.var_qgse_dn4, locals.var_qgse_dn6, locals.var_qgse_dn7, locals.var_qgse_dn8, locals.var_qgse_dn9, ) = (assign43150_e48508, (assign43150_e48506 * locals.var_qgse_dn4), (assign43150_e48506 * locals.var_qgse_dn6), (assign43150_e48506 * locals.var_qgse_dn7), (assign43150_e48506 * locals.var_qgse_dn8), (assign43150_e48506 * locals.var_qgse_dn9), );
        locals.var_qgse_rv = 0.0;

        let assign43160_e48511: f64 = (p.p32 * locals.var_mult_i_int);
        let assign43160_e48513: f64 = (assign43160_e48511 * locals.var_qgde);
        (locals.var_qgde, locals.var_qgde_dn4, locals.var_qgde_dn6, locals.var_qgde_dn7, locals.var_qgde_dn8, locals.var_qgde_dn9, ) = (assign43160_e48513, (assign43160_e48511 * locals.var_qgde_dn4), (assign43160_e48511 * locals.var_qgde_dn6), (assign43160_e48511 * locals.var_qgde_dn7), (assign43160_e48511 * locals.var_qgde_dn8), (assign43160_e48511 * locals.var_qgde_dn9), );
        locals.var_qgde_rv = 0.0;

        let assign43170_e48516: f64 = (p.p32 * locals.var_mult_i_int);
        let assign43170_e48518: f64 = (assign43170_e48516 * locals.var_qovs);
        (locals.var_qovs, locals.var_qovs_dn4, locals.var_qovs_dn6, locals.var_qovs_dn7, locals.var_qovs_dn8, locals.var_qovs_dn9, ) = (assign43170_e48518, (assign43170_e48516 * locals.var_qovs_dn4), (assign43170_e48516 * locals.var_qovs_dn6), (assign43170_e48516 * locals.var_qovs_dn7), (assign43170_e48516 * locals.var_qovs_dn8), (assign43170_e48516 * locals.var_qovs_dn9), );
        locals.var_qovs_rv = 0.0;

        let assign43180_e48521: f64 = (p.p32 * locals.var_mult_i_int);
        let assign43180_e48523: f64 = (assign43180_e48521 * locals.var_qovd);
        (locals.var_qovd, locals.var_qovd_dn4, locals.var_qovd_dn6, locals.var_qovd_dn7, locals.var_qovd_dn8, locals.var_qovd_dn9, ) = (assign43180_e48523, (assign43180_e48521 * locals.var_qovd_dn4), (assign43180_e48521 * locals.var_qovd_dn6), (assign43180_e48521 * locals.var_qovd_dn7), (assign43180_e48521 * locals.var_qovd_dn8), (assign43180_e48521 * locals.var_qovd_dn9), );
        locals.var_qovd_rv = 0.0;

        let assign43190_e48526: f64 = (p.p32 * locals.var_mult_i_int);
        let assign43190_e48528: f64 = (assign43190_e48526 * locals.var_qgbe);
        (locals.var_qgbe, locals.var_qgbe_dn4, locals.var_qgbe_dn6, locals.var_qgbe_dn7, locals.var_qgbe_dn8, locals.var_qgbe_dn9, ) = (assign43190_e48528, (assign43190_e48526 * locals.var_qgbe_dn4), (assign43190_e48526 * locals.var_qgbe_dn6), (assign43190_e48526 * locals.var_qgbe_dn7), (assign43190_e48526 * locals.var_qgbe_dn8), (assign43190_e48526 * locals.var_qgbe_dn9), );
        locals.var_qgbe_rv = 0.0;

        let assign43200_e48531: f64 = (p.p32 * locals.var_mult_i_int);
        let assign43200_e48533: f64 = (assign43200_e48531 * locals.var_qssub);
        (locals.var_qssub, locals.var_qssub_dn6, locals.var_qssub_dn8, ) = (assign43200_e48533, (assign43200_e48531 * locals.var_qssub_dn6), (assign43200_e48531 * locals.var_qssub_dn8), );
        locals.var_qssub_rv = 0.0;

        let assign43210_e48536: f64 = (p.p32 * locals.var_mult_i_int);
        let assign43210_e48538: f64 = (assign43210_e48536 * locals.var_qdsub);
        (locals.var_qdsub, locals.var_qdsub_dn6, locals.var_qdsub_dn7, locals.var_qdsub_dn8, ) = (assign43210_e48538, (assign43210_e48536 * locals.var_qdsub_dn6), (assign43210_e48536 * locals.var_qdsub_dn7), (assign43210_e48536 * locals.var_qdsub_dn8), );
        locals.var_qdsub_rv = 0.0;

        let assign43220_e48541: f64 = (p.p32 * locals.var_mult_i_int);
        let assign43220_e48543: f64 = (assign43220_e48541 * locals.var_qdse);
        (locals.var_qdse, locals.var_qdse_dn6, locals.var_qdse_dn7, ) = (assign43220_e48543, (assign43220_e48541 * locals.var_qdse_dn6), (assign43220_e48541 * locals.var_qdse_dn7), );
        locals.var_qdse_rv = 0.0;

        let assign43240_e48549: f64 = if locals.var_sigvds < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1250 = assign43240_e48549;
        locals.var_guard1250_rv = 0.0;

        if (locals.var_guard1250 != 0.0) {
            (locals.var_temp_q, locals.var_temp_q_dn4, locals.var_temp_q_dn6, locals.var_temp_q_dn7, locals.var_temp_q_dn8, locals.var_temp_q_dn9, ) = (locals.var_qd, locals.var_qd_dn4, locals.var_qd_dn6, locals.var_qd_dn7, locals.var_qd_dn8, locals.var_qd_dn9, );
            locals.var_temp_q_rv = 0.0;
            (locals.var_qd, locals.var_qd_dn4, locals.var_qd_dn6, locals.var_qd_dn7, locals.var_qd_dn8, locals.var_qd_dn9, ) = (locals.var_qs, locals.var_qs_dn4, locals.var_qs_dn6, locals.var_qs_dn7, locals.var_qs_dn8, locals.var_qs_dn9, );
            locals.var_qd_rv = 0.0;
            (locals.var_qs, locals.var_qs_dn4, locals.var_qs_dn6, locals.var_qs_dn7, locals.var_qs_dn8, locals.var_qs_dn9, ) = (locals.var_temp_q, locals.var_temp_q_dn4, locals.var_temp_q_dn6, locals.var_temp_q_dn7, locals.var_temp_q_dn8, locals.var_temp_q_dn9, );
            locals.var_qs_rv = 0.0;
        }

        if (locals.var_guard1250 != 0.0) {
            let assign43280_e48564: f64 = (-locals.var_qdse);
            (locals.var_qdse, locals.var_qdse_dn6, locals.var_qdse_dn7, ) = (assign43280_e48564, (-locals.var_qdse_dn6), (-locals.var_qdse_dn7), );
            locals.var_qdse_rv = 0.0;
        }

        if (locals.var_guard1250 != 0.0) {
            (locals.var_temp_q, locals.var_temp_q_dn4, locals.var_temp_q_dn6, locals.var_temp_q_dn7, locals.var_temp_q_dn8, locals.var_temp_q_dn9, ) = (locals.var_qgdif, locals.var_qgdif_dn4, locals.var_qgdif_dn6, locals.var_qgdif_dn7, locals.var_qgdif_dn8, locals.var_qgdif_dn9, );
            locals.var_temp_q_rv = 0.0;
            (locals.var_qgdif, locals.var_qgdif_dn4, locals.var_qgdif_dn6, locals.var_qgdif_dn7, locals.var_qgdif_dn8, locals.var_qgdif_dn9, ) = (locals.var_qgsif, locals.var_qgsif_dn4, locals.var_qgsif_dn6, locals.var_qgsif_dn7, locals.var_qgsif_dn8, locals.var_qgsif_dn9, );
            locals.var_qgdif_rv = 0.0;
            (locals.var_qgsif, locals.var_qgsif_dn4, locals.var_qgsif_dn6, locals.var_qgsif_dn7, locals.var_qgsif_dn8, locals.var_qgsif_dn9, ) = (locals.var_temp_q, locals.var_temp_q_dn4, locals.var_temp_q_dn6, locals.var_temp_q_dn7, locals.var_temp_q_dn8, locals.var_temp_q_dn9, );
            locals.var_qgsif_rv = 0.0;
            (locals.var_temp_q, locals.var_temp_q_dn4, locals.var_temp_q_dn6, locals.var_temp_q_dn7, locals.var_temp_q_dn8, locals.var_temp_q_dn9, ) = (locals.var_qbdif, locals.var_qbdif_dn4, locals.var_qbdif_dn6, locals.var_qbdif_dn7, locals.var_qbdif_dn8, locals.var_qbdif_dn9, );
            locals.var_temp_q_rv = 0.0;
            (locals.var_qbdif, locals.var_qbdif_dn4, locals.var_qbdif_dn6, locals.var_qbdif_dn7, locals.var_qbdif_dn8, locals.var_qbdif_dn9, ) = (locals.var_qbsif, locals.var_qbsif_dn4, locals.var_qbsif_dn6, locals.var_qbsif_dn7, locals.var_qbsif_dn8, locals.var_qbsif_dn9, );
            locals.var_qbdif_rv = 0.0;
            (locals.var_qbsif, locals.var_qbsif_dn4, locals.var_qbsif_dn6, locals.var_qbsif_dn7, locals.var_qbsif_dn8, locals.var_qbsif_dn9, ) = (locals.var_temp_q, locals.var_temp_q_dn4, locals.var_temp_q_dn6, locals.var_temp_q_dn7, locals.var_temp_q_dn8, locals.var_temp_q_dn9, );
            locals.var_qbsif_rv = 0.0;
        }

        let assign43350_e48593: f64 = if locals.var_mult_i_int > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1251 = assign43350_e48593;
        locals.var_guard1251_rv = 0.0;

        if (locals.var_guard1251 != 0.0) {
            let assign43360_e48597: f64 = (1e-9 * locals.var_betneff);
            let assign43360_e48600: f64 = (locals.var_gvsat * locals.var_areaq_i);
            let assign43360_e48601: f64 = (assign43360_e48597 / assign43360_e48600);
            let assign43360_e48605: f64 = (locals.var_qg_wo_mult + locals.var_qb_wo_mult);
            let assign43360_e48606: f64 = (locals.var_kdrift_i * assign43360_e48605);
            let assign43360_e48609: f64 = (locals.var_areaq_i * locals.var_cox1init);
            let assign43360_e48610: f64 = (assign43360_e48606 / assign43360_e48609);
            let assign43360_e48613: f64 = (locals.var_kdiff_i * locals.var_phit);
            let assign43360_e48614: f64 = (assign43360_e48610 + assign43360_e48613);
            let assign43360_e48615: f64 = (assign43360_e48601 * assign43360_e48614);
            (locals.var_itaueff, locals.var_itaueff_dn4, locals.var_itaueff_dn6, locals.var_itaueff_dn7, locals.var_itaueff_dn8, locals.var_itaueff_dn9, ) = (assign43360_e48615, ((((((1e-9 * locals.var_betneff_dn4) * assign43360_e48600) - (assign43360_e48597 * (locals.var_gvsat_dn4 * locals.var_areaq_i))) / (assign43360_e48600 * assign43360_e48600)) * assign43360_e48614) + (assign43360_e48601 * ((((locals.var_kdrift_i_dn4 * assign43360_e48605) + (locals.var_kdrift_i * (locals.var_qg_wo_mult_dn4 + locals.var_qb_wo_mult_dn4))) / assign43360_e48609) + ((locals.var_kdiff_i_dn4 * locals.var_phit) + (locals.var_kdiff_i * locals.var_phit_dn4))))), ((((((1e-9 * locals.var_betneff_dn6) * assign43360_e48600) - (assign43360_e48597 * (locals.var_gvsat_dn6 * locals.var_areaq_i))) / (assign43360_e48600 * assign43360_e48600)) * assign43360_e48614) + (assign43360_e48601 * ((((locals.var_kdrift_i_dn6 * assign43360_e48605) + (locals.var_kdrift_i * (locals.var_qg_wo_mult_dn6 + locals.var_qb_wo_mult_dn6))) / assign43360_e48609) + ((locals.var_kdiff_i_dn6 * locals.var_phit) + (locals.var_kdiff_i * locals.var_phit_dn6))))), ((((((1e-9 * locals.var_betneff_dn7) * assign43360_e48600) - (assign43360_e48597 * (locals.var_gvsat_dn7 * locals.var_areaq_i))) / (assign43360_e48600 * assign43360_e48600)) * assign43360_e48614) + (assign43360_e48601 * ((((locals.var_kdrift_i_dn7 * assign43360_e48605) + (locals.var_kdrift_i * (locals.var_qg_wo_mult_dn7 + locals.var_qb_wo_mult_dn7))) / assign43360_e48609) + ((locals.var_kdiff_i_dn7 * locals.var_phit) + (locals.var_kdiff_i * locals.var_phit_dn7))))), ((((((1e-9 * locals.var_betneff_dn8) * assign43360_e48600) - (assign43360_e48597 * (locals.var_gvsat_dn8 * locals.var_areaq_i))) / (assign43360_e48600 * assign43360_e48600)) * assign43360_e48614) + (assign43360_e48601 * ((((locals.var_kdrift_i_dn8 * assign43360_e48605) + (locals.var_kdrift_i * (locals.var_qg_wo_mult_dn8 + locals.var_qb_wo_mult_dn8))) / assign43360_e48609) + ((locals.var_kdiff_i_dn8 * locals.var_phit) + (locals.var_kdiff_i * locals.var_phit_dn8))))), ((((((1e-9 * locals.var_betneff_dn9) * assign43360_e48600) - (assign43360_e48597 * (locals.var_gvsat_dn9 * locals.var_areaq_i))) / (assign43360_e48600 * assign43360_e48600)) * assign43360_e48614) + (assign43360_e48601 * ((((locals.var_kdrift_i_dn9 * assign43360_e48605) + (locals.var_kdrift_i * (locals.var_qg_wo_mult_dn9 + locals.var_qb_wo_mult_dn9))) / assign43360_e48609) + ((locals.var_kdiff_i_dn9 * locals.var_phit) + (locals.var_kdiff_i * locals.var_phit_dn9))))), );
            locals.var_itaueff_rv = 0.0;
        }

        if (locals.var_guard1251 == 0.0) {
            (locals.var_itaueff, locals.var_itaueff_dn4, locals.var_itaueff_dn6, locals.var_itaueff_dn7, locals.var_itaueff_dn8, locals.var_itaueff_dn9, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_itaueff_rv = 0.0;
        }

        let assign43380_e48625: f64 = (locals.var_csiprime_dc / 1.602176565e-19);
        let assign43380_e48627: f64 = (assign43380_e48625 * locals.var_phit);
        (locals.var_nunit, locals.var_nunit_dn4, locals.var_nunit_dn6, locals.var_nunit_dn7, locals.var_nunit_dn8, locals.var_nunit_dn9, ) = (assign43380_e48627, (((locals.var_csiprime_dc_dn4 / 1.602176565e-19) * locals.var_phit) + (assign43380_e48625 * locals.var_phit_dn4)), (((locals.var_csiprime_dc_dn6 / 1.602176565e-19) * locals.var_phit) + (assign43380_e48625 * locals.var_phit_dn6)), (((locals.var_csiprime_dc_dn7 / 1.602176565e-19) * locals.var_phit) + (assign43380_e48625 * locals.var_phit_dn7)), (((locals.var_csiprime_dc_dn8 / 1.602176565e-19) * locals.var_phit) + (assign43380_e48625 * locals.var_phit_dn8)), (((locals.var_csiprime_dc_dn9 / 1.602176565e-19) * locals.var_phit) + (assign43380_e48625 * locals.var_phit_dn9)), );
        locals.var_nunit_rv = 0.0;

        let assign43390_e48629: f64 = (-0.5);
        let assign43390_e48632: f64 = (locals.var_ds_dc + locals.var_dd_dc);
        let assign43390_e48633: f64 = (assign43390_e48629 * assign43390_e48632);
        (locals.var_dm, locals.var_dm_dn4, locals.var_dm_dn6, locals.var_dm_dn7, locals.var_dm_dn8, locals.var_dm_dn9, ) = (assign43390_e48633, (assign43390_e48629 * (locals.var_ds_dc_dn4 + locals.var_dd_dc_dn4)), (assign43390_e48629 * (locals.var_ds_dc_dn6 + locals.var_dd_dc_dn6)), (assign43390_e48629 * (locals.var_ds_dc_dn7 + locals.var_dd_dc_dn7)), (assign43390_e48629 * (locals.var_ds_dc_dn8 + locals.var_dd_dc_dn8)), (assign43390_e48629 * (locals.var_ds_dc_dn9 + locals.var_dd_dc_dn9)), );
        locals.var_dm_rv = 0.0;

        let assign43400_e48636: f64 = (locals.var_qim_dc + locals.var_dm);
        (locals.var_qimstar, locals.var_qimstar_dn4, locals.var_qimstar_dn6, locals.var_qimstar_dn7, locals.var_qimstar_dn8, locals.var_qimstar_dn9, ) = (assign43400_e48636, (locals.var_qim_dc_dn4 + locals.var_dm_dn4), (locals.var_qim_dc_dn6 + locals.var_dm_dn6), (locals.var_qim_dc_dn7 + locals.var_dm_dn7), (locals.var_qim_dc_dn8 + locals.var_dm_dn8), (locals.var_qim_dc_dn9 + locals.var_dm_dn9), );
        locals.var_qimstar_rv = 0.0;

        let assign43410_e48639: f64 = (locals.var_qim_dc / locals.var_qimstar);
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign43410_e48639, (((locals.var_qim_dc_dn4 * locals.var_qimstar) - (locals.var_qim_dc * locals.var_qimstar_dn4)) / (locals.var_qimstar * locals.var_qimstar)), (((locals.var_qim_dc_dn6 * locals.var_qimstar) - (locals.var_qim_dc * locals.var_qimstar_dn6)) / (locals.var_qimstar * locals.var_qimstar)), (((locals.var_qim_dc_dn7 * locals.var_qimstar) - (locals.var_qim_dc * locals.var_qimstar_dn7)) / (locals.var_qimstar * locals.var_qimstar)), (((locals.var_qim_dc_dn8 * locals.var_qimstar) - (locals.var_qim_dc * locals.var_qimstar_dn8)) / (locals.var_qimstar * locals.var_qimstar)), (((locals.var_qim_dc_dn9 * locals.var_qimstar) - (locals.var_qim_dc * locals.var_qimstar_dn9)) / (locals.var_qimstar * locals.var_qimstar)), );
        locals.var_temp_rv = 0.0;

        let assign43420_e48643: f64 = locals.var_temp;
        let assign43420_e48646: f64 = locals.var_temp;
        let assign43420_e48649: f64 = locals.var_temp;
        let assign43420_e48650: f64 = (assign43420_e48646 * assign43420_e48649);
        let assign43420_e48652: f64 = (assign43420_e48650 + 1e-20);
        let assign43420_e48653: f64 = (assign43420_e48652).sqrt();
        let assign43420_e48654: f64 = (assign43420_e48643 + assign43420_e48653);
        let assign43420_e48655: f64 = (0.5 * assign43420_e48654);
        (locals.var_t1, locals.var_t1_dn4, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, ) = (assign43420_e48655, (0.5 * (locals.var_temp_dn4 + (((locals.var_temp_dn4 * assign43420_e48649) + (assign43420_e48646 * locals.var_temp_dn4)) / (2.0 * assign43420_e48653)))), (0.5 * (locals.var_temp_dn6 + (((locals.var_temp_dn6 * assign43420_e48649) + (assign43420_e48646 * locals.var_temp_dn6)) / (2.0 * assign43420_e48653)))), (0.5 * (locals.var_temp_dn7 + (((locals.var_temp_dn7 * assign43420_e48649) + (assign43420_e48646 * locals.var_temp_dn7)) / (2.0 * assign43420_e48653)))), (0.5 * (locals.var_temp_dn8 + (((locals.var_temp_dn8 * assign43420_e48649) + (assign43420_e48646 * locals.var_temp_dn8)) / (2.0 * assign43420_e48653)))), (0.5 * (locals.var_temp_dn9 + (((locals.var_temp_dn9 * assign43420_e48649) + (assign43420_e48646 * locals.var_temp_dn9)) / (2.0 * assign43420_e48653)))), );
        locals.var_t1_rv = 0.0;

        let assign43430_e48657: f64 = (-0.1666666666667);
        let assign43430_e48659: f64 = (assign43430_e48657 * locals.var_delta_k1q1_dc);
        let assign43430_e48661: f64 = (assign43430_e48659 * locals.var_inv_k1h1_0_dc);
        (locals.var_sqrt_t2, locals.var_sqrt_t2_dn4, locals.var_sqrt_t2_dn6, locals.var_sqrt_t2_dn7, locals.var_sqrt_t2_dn8, locals.var_sqrt_t2_dn9, ) = (assign43430_e48661, (((assign43430_e48657 * locals.var_delta_k1q1_dc_dn4) * locals.var_inv_k1h1_0_dc) + (assign43430_e48659 * locals.var_inv_k1h1_0_dc_dn4)), (((assign43430_e48657 * locals.var_delta_k1q1_dc_dn6) * locals.var_inv_k1h1_0_dc) + (assign43430_e48659 * locals.var_inv_k1h1_0_dc_dn6)), (((assign43430_e48657 * locals.var_delta_k1q1_dc_dn7) * locals.var_inv_k1h1_0_dc) + (assign43430_e48659 * locals.var_inv_k1h1_0_dc_dn7)), (((assign43430_e48657 * locals.var_delta_k1q1_dc_dn8) * locals.var_inv_k1h1_0_dc) + (assign43430_e48659 * locals.var_inv_k1h1_0_dc_dn8)), (((assign43430_e48657 * locals.var_delta_k1q1_dc_dn9) * locals.var_inv_k1h1_0_dc) + (assign43430_e48659 * locals.var_inv_k1h1_0_dc_dn9)), );
        locals.var_sqrt_t2_rv = 0.0;

        let assign43440_e48664: f64 = (locals.var_sqrt_t2 * locals.var_sqrt_t2);
        (locals.var_t2, locals.var_t2_dn4, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, ) = (assign43440_e48664, ((locals.var_sqrt_t2_dn4 * locals.var_sqrt_t2) + (locals.var_sqrt_t2 * locals.var_sqrt_t2_dn4)), ((locals.var_sqrt_t2_dn6 * locals.var_sqrt_t2) + (locals.var_sqrt_t2 * locals.var_sqrt_t2_dn6)), ((locals.var_sqrt_t2_dn7 * locals.var_sqrt_t2) + (locals.var_sqrt_t2 * locals.var_sqrt_t2_dn7)), ((locals.var_sqrt_t2_dn8 * locals.var_sqrt_t2) + (locals.var_sqrt_t2 * locals.var_sqrt_t2_dn8)), ((locals.var_sqrt_t2_dn9 * locals.var_sqrt_t2) + (locals.var_sqrt_t2 * locals.var_sqrt_t2_dn9)), );
        locals.var_t2_rv = 0.0;

        let assign43450_e48667: f64 = (locals.var_hsat_dc - 1.0);
        (locals.var_r, locals.var_r_dn4, locals.var_r_dn6, locals.var_r_dn7, locals.var_r_dn8, locals.var_r_dn9, ) = (assign43450_e48667, locals.var_hsat_dc_dn4, locals.var_hsat_dc_dn6, locals.var_hsat_dc_dn7, locals.var_hsat_dc_dn8, locals.var_hsat_dc_dn9, );
        locals.var_r_rv = 0.0;

        let assign43490_e48697: f64 = (12.0 * locals.var_t2);
        (locals.var_t2x12, locals.var_t2x12_dn4, locals.var_t2x12_dn6, locals.var_t2x12_dn7, locals.var_t2x12_dn8, locals.var_t2x12_dn9, ) = (assign43490_e48697, (12.0 * locals.var_t2_dn4), (12.0 * locals.var_t2_dn6), (12.0 * locals.var_t2_dn7), (12.0 * locals.var_t2_dn8), (12.0 * locals.var_t2_dn9), );
        locals.var_t2x12_rv = 0.0;

        let assign43500_e48700: f64 = (locals.var_t1 + locals.var_t2x12);
        let assign43500_e48704: f64 = (1.0 + locals.var_t1);
        let assign43500_e48705: f64 = (2.0 * assign43500_e48704);
        let assign43500_e48707: f64 = (assign43500_e48705 * locals.var_t2x12);
        let assign43500_e48709: f64 = (assign43500_e48707 * locals.var_r);
        let assign43500_e48710: f64 = (assign43500_e48700 - assign43500_e48709);
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9, ) = (assign43500_e48710, ((locals.var_t1_dn4 + locals.var_t2x12_dn4) - (((((2.0 * locals.var_t1_dn4) * locals.var_t2x12) + (assign43500_e48705 * locals.var_t2x12_dn4)) * locals.var_r) + (assign43500_e48707 * locals.var_r_dn4))), ((locals.var_t1_dn6 + locals.var_t2x12_dn6) - (((((2.0 * locals.var_t1_dn6) * locals.var_t2x12) + (assign43500_e48705 * locals.var_t2x12_dn6)) * locals.var_r) + (assign43500_e48707 * locals.var_r_dn6))), ((locals.var_t1_dn7 + locals.var_t2x12_dn7) - (((((2.0 * locals.var_t1_dn7) * locals.var_t2x12) + (assign43500_e48705 * locals.var_t2x12_dn7)) * locals.var_r) + (assign43500_e48707 * locals.var_r_dn7))), ((locals.var_t1_dn8 + locals.var_t2x12_dn8) - (((((2.0 * locals.var_t1_dn8) * locals.var_t2x12) + (assign43500_e48705 * locals.var_t2x12_dn8)) * locals.var_r) + (assign43500_e48707 * locals.var_r_dn8))), ((locals.var_t1_dn9 + locals.var_t2x12_dn9) - (((((2.0 * locals.var_t1_dn9) * locals.var_t2x12) + (assign43500_e48705 * locals.var_t2x12_dn9)) * locals.var_r) + (assign43500_e48707 * locals.var_r_dn9))), );
        locals.var_temp1_rv = 0.0;

        let assign43510_e48713: f64 = (locals.var_temp1).max(1e-40);
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9, ) = (assign43510_e48713, if locals.var_temp1 >= 1e-40 { locals.var_temp1_dn4 } else { 0.0 }, if locals.var_temp1 >= 1e-40 { locals.var_temp1_dn6 } else { 0.0 }, if locals.var_temp1 >= 1e-40 { locals.var_temp1_dn7 } else { 0.0 }, if locals.var_temp1 >= 1e-40 { locals.var_temp1_dn8 } else { 0.0 }, if locals.var_temp1 >= 1e-40 { locals.var_temp1_dn9 } else { 0.0 }, );
        locals.var_temp2_rv = 0.0;

        let assign43580_e48765: f64 = (locals.var_k1_ac * locals.var_csiprime_ac);
        let assign43580_e48767: f64 = (assign43580_e48765 * locals.var_areaq_i);
        let assign43580_e48769: f64 = (assign43580_e48767 / locals.var_qmfact1_ac);
        (locals.var_cox_qm, locals.var_cox_qm_dn4, locals.var_cox_qm_dn6, locals.var_cox_qm_dn7, locals.var_cox_qm_dn8, locals.var_cox_qm_dn9, ) = (assign43580_e48769, ((((((locals.var_k1_ac_dn4 * locals.var_csiprime_ac) + (locals.var_k1_ac * locals.var_csiprime_ac_dn4)) * locals.var_areaq_i) * locals.var_qmfact1_ac) - (assign43580_e48767 * locals.var_qmfact1_ac_dn4)) / (locals.var_qmfact1_ac * locals.var_qmfact1_ac)), ((((((locals.var_k1_ac_dn6 * locals.var_csiprime_ac) + (locals.var_k1_ac * locals.var_csiprime_ac_dn6)) * locals.var_areaq_i) * locals.var_qmfact1_ac) - (assign43580_e48767 * locals.var_qmfact1_ac_dn6)) / (locals.var_qmfact1_ac * locals.var_qmfact1_ac)), ((((((locals.var_k1_ac_dn7 * locals.var_csiprime_ac) + (locals.var_k1_ac * locals.var_csiprime_ac_dn7)) * locals.var_areaq_i) * locals.var_qmfact1_ac) - (assign43580_e48767 * locals.var_qmfact1_ac_dn7)) / (locals.var_qmfact1_ac * locals.var_qmfact1_ac)), ((((((locals.var_k1_ac_dn8 * locals.var_csiprime_ac) + (locals.var_k1_ac * locals.var_csiprime_ac_dn8)) * locals.var_areaq_i) * locals.var_qmfact1_ac) - (assign43580_e48767 * locals.var_qmfact1_ac_dn8)) / (locals.var_qmfact1_ac * locals.var_qmfact1_ac)), ((((((locals.var_k1_ac_dn9 * locals.var_csiprime_ac) + (locals.var_k1_ac * locals.var_csiprime_ac_dn9)) * locals.var_areaq_i) * locals.var_qmfact1_ac) - (assign43580_e48767 * locals.var_qmfact1_ac_dn9)) / (locals.var_qmfact1_ac * locals.var_qmfact1_ac)), );
        locals.var_cox_qm_rv = 0.0;

        let assign43590_e48772: f64 = (1.0 + locals.var_zsat_ac);
        let assign43590_e48774: f64 = (assign43590_e48772 * locals.var_cox_qm);
        (locals.var_cgeff, locals.var_cgeff_dn4, locals.var_cgeff_dn6, locals.var_cgeff_dn7, locals.var_cgeff_dn8, locals.var_cgeff_dn9, ) = (assign43590_e48774, ((locals.var_zsat_ac_dn4 * locals.var_cox_qm) + (assign43590_e48772 * locals.var_cox_qm_dn4)), ((locals.var_zsat_ac_dn6 * locals.var_cox_qm) + (assign43590_e48772 * locals.var_cox_qm_dn6)), ((locals.var_zsat_ac_dn7 * locals.var_cox_qm) + (assign43590_e48772 * locals.var_cox_qm_dn7)), ((locals.var_zsat_ac_dn8 * locals.var_cox_qm) + (assign43590_e48772 * locals.var_cox_qm_dn8)), ((locals.var_zsat_ac_dn9 * locals.var_cox_qm) + (assign43590_e48772 * locals.var_cox_qm_dn9)), );
        locals.var_cgeff_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_62(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign43600_e48779: f64 = (0.25 * locals.var_sigvds);
        let assign43600_e48781: f64 = (assign43600_e48779 * locals.var_sqrt_t2);
        let assign43600_e48782: f64 = (0.5 - assign43600_e48781);
        let assign43600_e48783: f64 = (locals.var_cgeff * assign43600_e48782);
        (locals.var_cdgeff, locals.var_cdgeff_dn4, locals.var_cdgeff_dn6, locals.var_cdgeff_dn7, locals.var_cdgeff_dn8, locals.var_cdgeff_dn9, ) = (assign43600_e48783, ((locals.var_cgeff_dn4 * assign43600_e48782) + (locals.var_cgeff * (-(assign43600_e48779 * locals.var_sqrt_t2_dn4)))), ((locals.var_cgeff_dn6 * assign43600_e48782) + (locals.var_cgeff * (-(assign43600_e48779 * locals.var_sqrt_t2_dn6)))), ((locals.var_cgeff_dn7 * assign43600_e48782) + (locals.var_cgeff * (-(assign43600_e48779 * locals.var_sqrt_t2_dn7)))), ((locals.var_cgeff_dn8 * assign43600_e48782) + (locals.var_cgeff * (-(assign43600_e48779 * locals.var_sqrt_t2_dn8)))), ((locals.var_cgeff_dn9 * assign43600_e48782) + (locals.var_cgeff * (-(assign43600_e48779 * locals.var_sqrt_t2_dn9)))), );
        locals.var_cdgeff_rv = 0.0;

        let assign43610_e48786: f64 = (locals.var_cgeff - locals.var_cdgeff);
        (locals.var_csgeff, locals.var_csgeff_dn4, locals.var_csgeff_dn6, locals.var_csgeff_dn7, locals.var_csgeff_dn8, locals.var_csgeff_dn9, ) = (assign43610_e48786, (locals.var_cgeff_dn4 - locals.var_cdgeff_dn4), (locals.var_cgeff_dn6 - locals.var_cdgeff_dn6), (locals.var_cgeff_dn7 - locals.var_cdgeff_dn7), (locals.var_cgeff_dn8 - locals.var_cdgeff_dn8), (locals.var_cgeff_dn9 - locals.var_cdgeff_dn9), );
        locals.var_csgeff_rv = 0.0;

        let assign43640_e48791: f64 = if p.p6 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1285 = assign43640_e48791;
        locals.var_guard1285_rv = 0.0;

        if (locals.var_guard1285 != 0.0) {
            let assign43650_e48795: f64 = (locals.var_t1 / 12.0);
            let assign43650_e48799: f64 = (locals.var_t1 + 0.2);
            let assign43650_e48801: f64 = (assign43650_e48799 - locals.var_t2x12);
            let assign43650_e48802: f64 = (locals.var_t2 * assign43650_e48801);
            let assign43650_e48803: f64 = (assign43650_e48795 - assign43650_e48802);
            let assign43650_e48806: f64 = (1.6 * locals.var_t2);
            let assign43650_e48809: f64 = (locals.var_t1 + 1.0);
            let assign43650_e48811: f64 = (assign43650_e48809 - locals.var_t2x12);
            let assign43650_e48812: f64 = (assign43650_e48806 * assign43650_e48811);
            let assign43650_e48814: f64 = (assign43650_e48812 * locals.var_r);
            let assign43650_e48815: f64 = (assign43650_e48803 - assign43650_e48814);
            (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9, ) = (assign43650_e48815, (((locals.var_t1_dn4 / 12.0) - ((locals.var_t2_dn4 * assign43650_e48801) + (locals.var_t2 * (locals.var_t1_dn4 - locals.var_t2x12_dn4)))) - (((((1.6 * locals.var_t2_dn4) * assign43650_e48811) + (assign43650_e48806 * (locals.var_t1_dn4 - locals.var_t2x12_dn4))) * locals.var_r) + (assign43650_e48812 * locals.var_r_dn4))), (((locals.var_t1_dn6 / 12.0) - ((locals.var_t2_dn6 * assign43650_e48801) + (locals.var_t2 * (locals.var_t1_dn6 - locals.var_t2x12_dn6)))) - (((((1.6 * locals.var_t2_dn6) * assign43650_e48811) + (assign43650_e48806 * (locals.var_t1_dn6 - locals.var_t2x12_dn6))) * locals.var_r) + (assign43650_e48812 * locals.var_r_dn6))), (((locals.var_t1_dn7 / 12.0) - ((locals.var_t2_dn7 * assign43650_e48801) + (locals.var_t2 * (locals.var_t1_dn7 - locals.var_t2x12_dn7)))) - (((((1.6 * locals.var_t2_dn7) * assign43650_e48811) + (assign43650_e48806 * (locals.var_t1_dn7 - locals.var_t2x12_dn7))) * locals.var_r) + (assign43650_e48812 * locals.var_r_dn7))), (((locals.var_t1_dn8 / 12.0) - ((locals.var_t2_dn8 * assign43650_e48801) + (locals.var_t2 * (locals.var_t1_dn8 - locals.var_t2x12_dn8)))) - (((((1.6 * locals.var_t2_dn8) * assign43650_e48811) + (assign43650_e48806 * (locals.var_t1_dn8 - locals.var_t2x12_dn8))) * locals.var_r) + (assign43650_e48812 * locals.var_r_dn8))), (((locals.var_t1_dn9 / 12.0) - ((locals.var_t2_dn9 * assign43650_e48801) + (locals.var_t2 * (locals.var_t1_dn9 - locals.var_t2x12_dn9)))) - (((((1.6 * locals.var_t2_dn9) * assign43650_e48811) + (assign43650_e48806 * (locals.var_t1_dn9 - locals.var_t2x12_dn9))) * locals.var_r) + (assign43650_e48812 * locals.var_r_dn9))), );
            locals.var_temp1_rv = 0.0;
        }

        if (locals.var_guard1285 != 0.0) {
            let assign43660_e48821: f64 = (locals.var_temp1).max(1e-40);
            (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9, ) = (assign43660_e48821, if locals.var_temp1 >= 1e-40 { locals.var_temp1_dn4 } else { 0.0 }, if locals.var_temp1 >= 1e-40 { locals.var_temp1_dn6 } else { 0.0 }, if locals.var_temp1 >= 1e-40 { locals.var_temp1_dn7 } else { 0.0 }, if locals.var_temp1 >= 1e-40 { locals.var_temp1_dn8 } else { 0.0 }, if locals.var_temp1 >= 1e-40 { locals.var_temp1_dn9 } else { 0.0 }, );
            locals.var_temp2_rv = 0.0;
        }

        (locals.var_nstar, locals.var_nstar_dn4, locals.var_nstar_dn6, locals.var_nstar_dn7, locals.var_nstar_dn8, locals.var_nstar_dn9, ) = (locals.var_nunit, locals.var_nunit_dn4, locals.var_nunit_dn6, locals.var_nunit_dn7, locals.var_nunit_dn8, locals.var_nunit_dn9, );
        locals.var_nstar_rv = 0.0;

        let assign43770_e48968: f64 = (locals.var_qim_dc + 1.0);
        let assign43770_e48969: f64 = (locals.var_nunit * assign43770_e48968);
        (locals.var_nmstar, locals.var_nmstar_dn4, locals.var_nmstar_dn6, locals.var_nmstar_dn7, locals.var_nmstar_dn8, locals.var_nmstar_dn9, ) = (assign43770_e48969, ((locals.var_nunit_dn4 * assign43770_e48968) + (locals.var_nunit * locals.var_qim_dc_dn4)), ((locals.var_nunit_dn6 * assign43770_e48968) + (locals.var_nunit * locals.var_qim_dc_dn6)), ((locals.var_nunit_dn7 * assign43770_e48968) + (locals.var_nunit * locals.var_qim_dc_dn7)), ((locals.var_nunit_dn8 * assign43770_e48968) + (locals.var_nunit * locals.var_qim_dc_dn8)), ((locals.var_nunit_dn9 * assign43770_e48968) + (locals.var_nunit * locals.var_qim_dc_dn9)), );
        locals.var_nmstar_rv = 0.0;

        let assign43780_e48973: f64 = (locals.var_qis_dc - locals.var_qid_dc);
        let assign43780_e48974: f64 = (locals.var_nunit * assign43780_e48973);
        (locals.var_deltan, locals.var_deltan_dn4, locals.var_deltan_dn6, locals.var_deltan_dn7, locals.var_deltan_dn8, locals.var_deltan_dn9, ) = (assign43780_e48974, ((locals.var_nunit_dn4 * assign43780_e48973) + (locals.var_nunit * (locals.var_qis_dc_dn4 - locals.var_qid_dc_dn4))), ((locals.var_nunit_dn6 * assign43780_e48973) + (locals.var_nunit * (locals.var_qis_dc_dn6 - locals.var_qid_dc_dn6))), ((locals.var_nunit_dn7 * assign43780_e48973) + (locals.var_nunit * (locals.var_qis_dc_dn7 - locals.var_qid_dc_dn7))), ((locals.var_nunit_dn8 * assign43780_e48973) + (locals.var_nunit * (locals.var_qis_dc_dn8 - locals.var_qid_dc_dn8))), ((locals.var_nunit_dn9 * assign43780_e48973) + (locals.var_nunit * (locals.var_qis_dc_dn9 - locals.var_qid_dc_dn9))), );
        locals.var_deltan_rv = 0.0;

        let assign43790_e48978: f64 = (locals.var_nfb_i * locals.var_nstar);
        let assign43790_e48979: f64 = (locals.var_nfa_i - assign43790_e48978);
        let assign43790_e48982: f64 = (locals.var_nfc_i * locals.var_nstar);
        let assign43790_e48984: f64 = (assign43790_e48982 * locals.var_nstar);
        let assign43790_e48985: f64 = (assign43790_e48979 + assign43790_e48984);
        let assign43790_e48989: f64 = (0.5 * locals.var_deltan);
        let assign43790_e48990: f64 = (locals.var_nmstar + assign43790_e48989);
        let assign43790_e48994: f64 = (0.5 * locals.var_deltan);
        let assign43790_e48995: f64 = (locals.var_nmstar - assign43790_e48994);
        let assign43790_e48996: f64 = (assign43790_e48990 / assign43790_e48995);
        let assign43790_e48997: f64 = (assign43790_e48996).ln();
        let assign43790_e48998: f64 = (assign43790_e48985 * assign43790_e48997);
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9, ) = (assign43790_e48998, ((((-(locals.var_nfb_i * locals.var_nstar_dn4)) + (((locals.var_nfc_i * locals.var_nstar_dn4) * locals.var_nstar) + (assign43790_e48982 * locals.var_nstar_dn4))) * assign43790_e48997) + (assign43790_e48985 * (((((locals.var_nmstar_dn4 + (0.5 * locals.var_deltan_dn4)) * assign43790_e48995) - (assign43790_e48990 * (locals.var_nmstar_dn4 - (0.5 * locals.var_deltan_dn4)))) / (assign43790_e48995 * assign43790_e48995)) / assign43790_e48996))), ((((-(locals.var_nfb_i * locals.var_nstar_dn6)) + (((locals.var_nfc_i * locals.var_nstar_dn6) * locals.var_nstar) + (assign43790_e48982 * locals.var_nstar_dn6))) * assign43790_e48997) + (assign43790_e48985 * (((((locals.var_nmstar_dn6 + (0.5 * locals.var_deltan_dn6)) * assign43790_e48995) - (assign43790_e48990 * (locals.var_nmstar_dn6 - (0.5 * locals.var_deltan_dn6)))) / (assign43790_e48995 * assign43790_e48995)) / assign43790_e48996))), ((((-(locals.var_nfb_i * locals.var_nstar_dn7)) + (((locals.var_nfc_i * locals.var_nstar_dn7) * locals.var_nstar) + (assign43790_e48982 * locals.var_nstar_dn7))) * assign43790_e48997) + (assign43790_e48985 * (((((locals.var_nmstar_dn7 + (0.5 * locals.var_deltan_dn7)) * assign43790_e48995) - (assign43790_e48990 * (locals.var_nmstar_dn7 - (0.5 * locals.var_deltan_dn7)))) / (assign43790_e48995 * assign43790_e48995)) / assign43790_e48996))), ((((-(locals.var_nfb_i * locals.var_nstar_dn8)) + (((locals.var_nfc_i * locals.var_nstar_dn8) * locals.var_nstar) + (assign43790_e48982 * locals.var_nstar_dn8))) * assign43790_e48997) + (assign43790_e48985 * (((((locals.var_nmstar_dn8 + (0.5 * locals.var_deltan_dn8)) * assign43790_e48995) - (assign43790_e48990 * (locals.var_nmstar_dn8 - (0.5 * locals.var_deltan_dn8)))) / (assign43790_e48995 * assign43790_e48995)) / assign43790_e48996))), ((((-(locals.var_nfb_i * locals.var_nstar_dn9)) + (((locals.var_nfc_i * locals.var_nstar_dn9) * locals.var_nstar) + (assign43790_e48982 * locals.var_nstar_dn9))) * assign43790_e48997) + (assign43790_e48985 * (((((locals.var_nmstar_dn9 + (0.5 * locals.var_deltan_dn9)) * assign43790_e48995) - (assign43790_e48990 * (locals.var_nmstar_dn9 - (0.5 * locals.var_deltan_dn9)))) / (assign43790_e48995 * assign43790_e48995)) / assign43790_e48996))), );
        locals.var_temp1_rv = 0.0;

        let assign43800_e49005: f64 = (2.0 * locals.var_nstar);
        let assign43800_e49006: f64 = (locals.var_nmstar - assign43800_e49005);
        let assign43800_e49007: f64 = (locals.var_nfc_i * assign43800_e49006);
        let assign43800_e49008: f64 = (locals.var_nfb_i + assign43800_e49007);
        let assign43800_e49010: f64 = (assign43800_e49008 * locals.var_deltan);
        let assign43800_e49011: f64 = (locals.var_temp1 + assign43800_e49010);
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9, ) = (assign43800_e49011, (locals.var_temp1_dn4 + (((locals.var_nfc_i * (locals.var_nmstar_dn4 - (2.0 * locals.var_nstar_dn4))) * locals.var_deltan) + (assign43800_e49008 * locals.var_deltan_dn4))), (locals.var_temp1_dn6 + (((locals.var_nfc_i * (locals.var_nmstar_dn6 - (2.0 * locals.var_nstar_dn6))) * locals.var_deltan) + (assign43800_e49008 * locals.var_deltan_dn6))), (locals.var_temp1_dn7 + (((locals.var_nfc_i * (locals.var_nmstar_dn7 - (2.0 * locals.var_nstar_dn7))) * locals.var_deltan) + (assign43800_e49008 * locals.var_deltan_dn7))), (locals.var_temp1_dn8 + (((locals.var_nfc_i * (locals.var_nmstar_dn8 - (2.0 * locals.var_nstar_dn8))) * locals.var_deltan) + (assign43800_e49008 * locals.var_deltan_dn8))), (locals.var_temp1_dn9 + (((locals.var_nfc_i * (locals.var_nmstar_dn9 - (2.0 * locals.var_nstar_dn9))) * locals.var_deltan) + (assign43800_e49008 * locals.var_deltan_dn9))), );
        locals.var_temp2_rv = 0.0;

        let assign43810_e49015: f64 = (locals.var_nfe_i * locals.var_esurf1_dc);
        let assign43810_e49018: f64 = (locals.var_nfeb_i * locals.var_esurf2_dc);
        let assign43810_e49019: f64 = (assign43810_e49015 + assign43810_e49018);
        let assign43810_e49022: f64 = (locals.var_qim_dc + 1.0);
        let assign43810_e49023: f64 = (assign43810_e49019 / assign43810_e49022);
        let assign43810_e49024: f64 = (1.0 + assign43810_e49023);
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign43810_e49024, (((((locals.var_nfe_i * locals.var_esurf1_dc_dn4) + (locals.var_nfeb_i * locals.var_esurf2_dc_dn4)) * assign43810_e49022) - (assign43810_e49019 * locals.var_qim_dc_dn4)) / (assign43810_e49022 * assign43810_e49022)), (((((locals.var_nfe_i * locals.var_esurf1_dc_dn6) + (locals.var_nfeb_i * locals.var_esurf2_dc_dn6)) * assign43810_e49022) - (assign43810_e49019 * locals.var_qim_dc_dn6)) / (assign43810_e49022 * assign43810_e49022)), (((((locals.var_nfe_i * locals.var_esurf1_dc_dn7) + (locals.var_nfeb_i * locals.var_esurf2_dc_dn7)) * assign43810_e49022) - (assign43810_e49019 * locals.var_qim_dc_dn7)) / (assign43810_e49022 * assign43810_e49022)), (((((locals.var_nfe_i * locals.var_esurf1_dc_dn8) + (locals.var_nfeb_i * locals.var_esurf2_dc_dn8)) * assign43810_e49022) - (assign43810_e49019 * locals.var_qim_dc_dn8)) / (assign43810_e49022 * assign43810_e49022)), (((((locals.var_nfe_i * locals.var_esurf1_dc_dn9) + (locals.var_nfeb_i * locals.var_esurf2_dc_dn9)) * assign43810_e49022) - (assign43810_e49019 * locals.var_qim_dc_dn9)) / (assign43810_e49022 * assign43810_e49022)), );
        locals.var_temp_rv = 0.0;

        let assign43820_e49028: f64 = (locals.var_temp + 0.01);
        let assign43820_e49031: f64 = (locals.var_temp - 0.01);
        let assign43820_e49034: f64 = (locals.var_temp - 0.01);
        let assign43820_e49035: f64 = (assign43820_e49031 * assign43820_e49034);
        let assign43820_e49037: f64 = (assign43820_e49035 + 0.0001);
        let assign43820_e49038: f64 = (assign43820_e49037).sqrt();
        let assign43820_e49039: f64 = (assign43820_e49028 + assign43820_e49038);
        let assign43820_e49040: f64 = (0.5 * assign43820_e49039);
        (locals.var_temp3, locals.var_temp3_dn4, locals.var_temp3_dn6, locals.var_temp3_dn7, locals.var_temp3_dn8, locals.var_temp3_dn9, ) = (assign43820_e49040, (0.5 * (locals.var_temp_dn4 + (((locals.var_temp_dn4 * assign43820_e49034) + (assign43820_e49031 * locals.var_temp_dn4)) / (2.0 * assign43820_e49038)))), (0.5 * (locals.var_temp_dn6 + (((locals.var_temp_dn6 * assign43820_e49034) + (assign43820_e49031 * locals.var_temp_dn6)) / (2.0 * assign43820_e49038)))), (0.5 * (locals.var_temp_dn7 + (((locals.var_temp_dn7 * assign43820_e49034) + (assign43820_e49031 * locals.var_temp_dn7)) / (2.0 * assign43820_e49038)))), (0.5 * (locals.var_temp_dn8 + (((locals.var_temp_dn8 * assign43820_e49034) + (assign43820_e49031 * locals.var_temp_dn8)) / (2.0 * assign43820_e49038)))), (0.5 * (locals.var_temp_dn9 + (((locals.var_temp_dn9 * assign43820_e49034) + (assign43820_e49031 * locals.var_temp_dn9)) / (2.0 * assign43820_e49038)))), );
        locals.var_temp3_rv = 0.0;

        let assign43830_e49043: f64 = (1.602176565e-19 * locals.var_fact_ids);
        let assign43830_e49045: f64 = (assign43830_e49043 * locals.var_ids);
        let assign43830_e49047: f64 = (assign43830_e49045 / locals.var_gvsat);
        let assign43830_e49049: f64 = (assign43830_e49047 * locals.var_temp2);
        let assign43830_e49051: f64 = (assign43830_e49049 / locals.var_nstar);
        let assign43830_e49053: f64 = (assign43830_e49051 * locals.var_temp3);
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign43830_e49053, (((((((((((((1.602176565e-19 * locals.var_fact_ids_dn4) * locals.var_ids) + (assign43830_e49043 * locals.var_ids_dn4)) * locals.var_gvsat) - (assign43830_e49045 * locals.var_gvsat_dn4)) / (locals.var_gvsat * locals.var_gvsat)) * locals.var_temp2) + (assign43830_e49047 * locals.var_temp2_dn4)) * locals.var_nstar) - (assign43830_e49049 * locals.var_nstar_dn4)) / (locals.var_nstar * locals.var_nstar)) * locals.var_temp3) + (assign43830_e49051 * locals.var_temp3_dn4)), (((((((((((((1.602176565e-19 * locals.var_fact_ids_dn6) * locals.var_ids) + (assign43830_e49043 * locals.var_ids_dn6)) * locals.var_gvsat) - (assign43830_e49045 * locals.var_gvsat_dn6)) / (locals.var_gvsat * locals.var_gvsat)) * locals.var_temp2) + (assign43830_e49047 * locals.var_temp2_dn6)) * locals.var_nstar) - (assign43830_e49049 * locals.var_nstar_dn6)) / (locals.var_nstar * locals.var_nstar)) * locals.var_temp3) + (assign43830_e49051 * locals.var_temp3_dn6)), (((((((((((((1.602176565e-19 * locals.var_fact_ids_dn7) * locals.var_ids) + (assign43830_e49043 * locals.var_ids_dn7)) * locals.var_gvsat) - (assign43830_e49045 * locals.var_gvsat_dn7)) / (locals.var_gvsat * locals.var_gvsat)) * locals.var_temp2) + (assign43830_e49047 * locals.var_temp2_dn7)) * locals.var_nstar) - (assign43830_e49049 * locals.var_nstar_dn7)) / (locals.var_nstar * locals.var_nstar)) * locals.var_temp3) + (assign43830_e49051 * locals.var_temp3_dn7)), (((((((((((((1.602176565e-19 * locals.var_fact_ids_dn8) * locals.var_ids) + (assign43830_e49043 * locals.var_ids_dn8)) * locals.var_gvsat) - (assign43830_e49045 * locals.var_gvsat_dn8)) / (locals.var_gvsat * locals.var_gvsat)) * locals.var_temp2) + (assign43830_e49047 * locals.var_temp2_dn8)) * locals.var_nstar) - (assign43830_e49049 * locals.var_nstar_dn8)) / (locals.var_nstar * locals.var_nstar)) * locals.var_temp3) + (assign43830_e49051 * locals.var_temp3_dn8)), (((((((((((((1.602176565e-19 * locals.var_fact_ids_dn9) * locals.var_ids) + (assign43830_e49043 * locals.var_ids_dn9)) * locals.var_gvsat) - (assign43830_e49045 * locals.var_gvsat_dn9)) / (locals.var_gvsat * locals.var_gvsat)) * locals.var_temp2) + (assign43830_e49047 * locals.var_temp2_dn9)) * locals.var_nstar) - (assign43830_e49049 * locals.var_nstar_dn9)) / (locals.var_nstar * locals.var_nstar)) * locals.var_temp3) + (assign43830_e49051 * locals.var_temp3_dn9)), );
        locals.var_temp_rv = 0.0;

        let assign43900_e49113: f64 = (locals.var_tkd * 8.617332384961e-5);
        let assign43900_e49114: f64 = (1.0 / assign43900_e49113);
        (locals.var_inv_phit0_op, locals.var_inv_phit0_op_dn4, locals.var_inv_phit0_op_dn6, locals.var_inv_phit0_op_dn7, locals.var_inv_phit0_op_dn8, locals.var_inv_phit0_op_dn9, ) = (assign43900_e49114, (-((locals.var_tkd_dn4 * 8.617332384961e-5) / (assign43900_e49113 * assign43900_e49113))), (-((locals.var_tkd_dn6 * 8.617332384961e-5) / (assign43900_e49113 * assign43900_e49113))), (-((locals.var_tkd_dn7 * 8.617332384961e-5) / (assign43900_e49113 * assign43900_e49113))), (-((locals.var_tkd_dn8 * 8.617332384961e-5) / (assign43900_e49113 * assign43900_e49113))), (-((locals.var_tkd_dn9 * 8.617332384961e-5) / (assign43900_e49113 * assign43900_e49113))), );
        locals.var_inv_phit0_op_rv = 0.0;

        let assign43910_e49118: f64 = (0.000473 * locals.var_tkd);
        let assign43910_e49120: f64 = (assign43910_e49118 * locals.var_tkd);
        let assign43910_e49123: f64 = (636.0 + locals.var_tkd);
        let assign43910_e49124: f64 = (assign43910_e49120 / assign43910_e49123);
        let assign43910_e49125: f64 = (1.17 - assign43910_e49124);
        (locals.var_egsi_op, locals.var_egsi_op_dn4, locals.var_egsi_op_dn6, locals.var_egsi_op_dn7, locals.var_egsi_op_dn8, locals.var_egsi_op_dn9, ) = (assign43910_e49125, (-((((((0.000473 * locals.var_tkd_dn4) * locals.var_tkd) + (assign43910_e49118 * locals.var_tkd_dn4)) * assign43910_e49123) - (assign43910_e49120 * locals.var_tkd_dn4)) / (assign43910_e49123 * assign43910_e49123))), (-((((((0.000473 * locals.var_tkd_dn6) * locals.var_tkd) + (assign43910_e49118 * locals.var_tkd_dn6)) * assign43910_e49123) - (assign43910_e49120 * locals.var_tkd_dn6)) / (assign43910_e49123 * assign43910_e49123))), (-((((((0.000473 * locals.var_tkd_dn7) * locals.var_tkd) + (assign43910_e49118 * locals.var_tkd_dn7)) * assign43910_e49123) - (assign43910_e49120 * locals.var_tkd_dn7)) / (assign43910_e49123 * assign43910_e49123))), (-((((((0.000473 * locals.var_tkd_dn8) * locals.var_tkd) + (assign43910_e49118 * locals.var_tkd_dn8)) * assign43910_e49123) - (assign43910_e49120 * locals.var_tkd_dn8)) / (assign43910_e49123 * assign43910_e49123))), (-((((((0.000473 * locals.var_tkd_dn9) * locals.var_tkd) + (assign43910_e49118 * locals.var_tkd_dn9)) * assign43910_e49123) - (assign43910_e49120 * locals.var_tkd_dn9)) / (assign43910_e49123 * assign43910_e49123))), );
        locals.var_egsi_op_rv = 0.0;

        let assign43920_e49129: f64 = (0.0004774 * locals.var_tkd);
        let assign43920_e49131: f64 = (assign43920_e49129 * locals.var_tkd);
        let assign43920_e49134: f64 = (235.0 + locals.var_tkd);
        let assign43920_e49135: f64 = (assign43920_e49131 / assign43920_e49134);
        let assign43920_e49136: f64 = (0.744 - assign43920_e49135);
        (locals.var_egge_op, locals.var_egge_op_dn4, locals.var_egge_op_dn6, locals.var_egge_op_dn7, locals.var_egge_op_dn8, locals.var_egge_op_dn9, ) = (assign43920_e49136, (-((((((0.0004774 * locals.var_tkd_dn4) * locals.var_tkd) + (assign43920_e49129 * locals.var_tkd_dn4)) * assign43920_e49134) - (assign43920_e49131 * locals.var_tkd_dn4)) / (assign43920_e49134 * assign43920_e49134))), (-((((((0.0004774 * locals.var_tkd_dn6) * locals.var_tkd) + (assign43920_e49129 * locals.var_tkd_dn6)) * assign43920_e49134) - (assign43920_e49131 * locals.var_tkd_dn6)) / (assign43920_e49134 * assign43920_e49134))), (-((((((0.0004774 * locals.var_tkd_dn7) * locals.var_tkd) + (assign43920_e49129 * locals.var_tkd_dn7)) * assign43920_e49134) - (assign43920_e49131 * locals.var_tkd_dn7)) / (assign43920_e49134 * assign43920_e49134))), (-((((((0.0004774 * locals.var_tkd_dn8) * locals.var_tkd) + (assign43920_e49129 * locals.var_tkd_dn8)) * assign43920_e49134) - (assign43920_e49131 * locals.var_tkd_dn8)) / (assign43920_e49134 * assign43920_e49134))), (-((((((0.0004774 * locals.var_tkd_dn9) * locals.var_tkd) + (assign43920_e49129 * locals.var_tkd_dn9)) * assign43920_e49134) - (assign43920_e49131 * locals.var_tkd_dn9)) / (assign43920_e49134 * assign43920_e49134))), );
        locals.var_egge_op_rv = 0.0;

        let assign43930_e49139: f64 = (locals.var_egge_op - locals.var_egsi_op);
        let assign43930_e49141: f64 = (-0.4);
        let assign43930_e49143: f64 = (assign43930_e49141 * locals.var_one_m_xge);
        let assign43930_e49144: f64 = (assign43930_e49139 + assign43930_e49143);
        let assign43930_e49146: f64 = (assign43930_e49144 * locals.var_xge_i);
        (locals.var_deg_op, locals.var_deg_op_dn4, locals.var_deg_op_dn6, locals.var_deg_op_dn7, locals.var_deg_op_dn8, locals.var_deg_op_dn9, ) = (assign43930_e49146, ((locals.var_egge_op_dn4 - locals.var_egsi_op_dn4) * locals.var_xge_i), ((locals.var_egge_op_dn6 - locals.var_egsi_op_dn6) * locals.var_xge_i), ((locals.var_egge_op_dn7 - locals.var_egsi_op_dn7) * locals.var_xge_i), ((locals.var_egge_op_dn8 - locals.var_egsi_op_dn8) * locals.var_xge_i), ((locals.var_egge_op_dn9 - locals.var_egsi_op_dn9) * locals.var_xge_i), );
        locals.var_deg_op_rv = 0.0;

        let assign43940_e49149: f64 = (locals.var_egsi_op + locals.var_deg_op);
        (locals.var_eg_op, locals.var_eg_op_dn4, locals.var_eg_op_dn6, locals.var_eg_op_dn7, locals.var_eg_op_dn8, locals.var_eg_op_dn9, ) = (assign43940_e49149, (locals.var_egsi_op_dn4 + locals.var_deg_op_dn4), (locals.var_egsi_op_dn6 + locals.var_deg_op_dn6), (locals.var_egsi_op_dn7 + locals.var_deg_op_dn7), (locals.var_egsi_op_dn8 + locals.var_deg_op_dn8), (locals.var_egsi_op_dn9 + locals.var_deg_op_dn9), );
        locals.var_eg_op_rv = 0.0;

        let assign43950_e49152: f64 = (0.5 * locals.var_eg_op);
        let assign43950_e49154: f64 = (assign43950_e49152 * locals.var_inv_phit0_op);
        (locals.var_eg_2phit0_op, locals.var_eg_2phit0_op_dn4, locals.var_eg_2phit0_op_dn6, locals.var_eg_2phit0_op_dn7, locals.var_eg_2phit0_op_dn8, locals.var_eg_2phit0_op_dn9, ) = (assign43950_e49154, (((0.5 * locals.var_eg_op_dn4) * locals.var_inv_phit0_op) + (assign43950_e49152 * locals.var_inv_phit0_op_dn4)), (((0.5 * locals.var_eg_op_dn6) * locals.var_inv_phit0_op) + (assign43950_e49152 * locals.var_inv_phit0_op_dn6)), (((0.5 * locals.var_eg_op_dn7) * locals.var_inv_phit0_op) + (assign43950_e49152 * locals.var_inv_phit0_op_dn7)), (((0.5 * locals.var_eg_op_dn8) * locals.var_inv_phit0_op) + (assign43950_e49152 * locals.var_inv_phit0_op_dn8)), (((0.5 * locals.var_eg_op_dn9) * locals.var_inv_phit0_op) + (assign43950_e49152 * locals.var_inv_phit0_op_dn9)), );
        locals.var_eg_2phit0_op_rv = 0.0;

        let assign43960_e49157: f64 = (0.05 * locals.var_xge_i);
        let assign43960_e49160: f64 = (0.5 * locals.var_deg_op);
        let assign43960_e49161: f64 = (assign43960_e49157 - assign43960_e49160);
        (locals.var_dvfbch_op, locals.var_dvfbch_op_dn4, locals.var_dvfbch_op_dn6, locals.var_dvfbch_op_dn7, locals.var_dvfbch_op_dn8, locals.var_dvfbch_op_dn9, ) = (assign43960_e49161, (-(0.5 * locals.var_deg_op_dn4)), (-(0.5 * locals.var_deg_op_dn6)), (-(0.5 * locals.var_deg_op_dn7)), (-(0.5 * locals.var_deg_op_dn8)), (-(0.5 * locals.var_deg_op_dn9)), );
        locals.var_dvfbch_op_rv = 0.0;

        let assign43970_e49164: f64 = (locals.var_tkd * 0.0033333333333);
        let assign43970_e49165: f64 = (assign43970_e49164).sqrt();
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign43970_e49165, ((locals.var_tkd_dn4 * 0.0033333333333) / (2.0 * assign43970_e49165)), ((locals.var_tkd_dn6 * 0.0033333333333) / (2.0 * assign43970_e49165)), ((locals.var_tkd_dn7 * 0.0033333333333) / (2.0 * assign43970_e49165)), ((locals.var_tkd_dn8 * 0.0033333333333) / (2.0 * assign43970_e49165)), ((locals.var_tkd_dn9 * 0.0033333333333) / (2.0 * assign43970_e49165)), );
        locals.var_temp_rv = 0.0;

        let assign43980_e49168: f64 = (4.05e25 * locals.var_temp);
        let assign43980_e49170: f64 = (assign43980_e49168 * locals.var_temp);
        let assign43980_e49172: f64 = (assign43980_e49170 * locals.var_temp);
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9, ) = (assign43980_e49172, (((((4.05e25 * locals.var_temp_dn4) * locals.var_temp) + (assign43980_e49168 * locals.var_temp_dn4)) * locals.var_temp) + (assign43980_e49170 * locals.var_temp_dn4)), (((((4.05e25 * locals.var_temp_dn6) * locals.var_temp) + (assign43980_e49168 * locals.var_temp_dn6)) * locals.var_temp) + (assign43980_e49170 * locals.var_temp_dn6)), (((((4.05e25 * locals.var_temp_dn7) * locals.var_temp) + (assign43980_e49168 * locals.var_temp_dn7)) * locals.var_temp) + (assign43980_e49170 * locals.var_temp_dn7)), (((((4.05e25 * locals.var_temp_dn8) * locals.var_temp) + (assign43980_e49168 * locals.var_temp_dn8)) * locals.var_temp) + (assign43980_e49170 * locals.var_temp_dn8)), (((((4.05e25 * locals.var_temp_dn9) * locals.var_temp) + (assign43980_e49168 * locals.var_temp_dn9)) * locals.var_temp) + (assign43980_e49170 * locals.var_temp_dn9)), );
        locals.var_temp1_rv = 0.0;

        let assign43990_e49175: f64 = (locals.var_temp1 * locals.var_niratio);
        (locals.var_neff_op, locals.var_neff_op_dn4, locals.var_neff_op_dn6, locals.var_neff_op_dn7, locals.var_neff_op_dn8, locals.var_neff_op_dn9, ) = (assign43990_e49175, (locals.var_temp1_dn4 * locals.var_niratio), (locals.var_temp1_dn6 * locals.var_niratio), (locals.var_temp1_dn7 * locals.var_niratio), (locals.var_temp1_dn8 * locals.var_niratio), (locals.var_temp1_dn9 * locals.var_niratio), );
        locals.var_neff_op_rv = 0.0;

        let assign44000_e49180: f64 = (locals.var_ct_i * locals.var_tkr);
        let assign44000_e49182: f64 = (assign44000_e49180 / locals.var_tkd);
        let assign44000_e49183: f64 = (1.0 + assign44000_e49182);
        let assign44000_e49184: f64 = (locals.var_inv_phit0_op / assign44000_e49183);
        (locals.var_inv_phit_op, locals.var_inv_phit_op_dn4, locals.var_inv_phit_op_dn6, locals.var_inv_phit_op_dn7, locals.var_inv_phit_op_dn8, locals.var_inv_phit_op_dn9, ) = (assign44000_e49184, (((locals.var_inv_phit0_op_dn4 * assign44000_e49183) - (locals.var_inv_phit0_op * (-((assign44000_e49180 * locals.var_tkd_dn4) / (locals.var_tkd * locals.var_tkd))))) / (assign44000_e49183 * assign44000_e49183)), (((locals.var_inv_phit0_op_dn6 * assign44000_e49183) - (locals.var_inv_phit0_op * (-((assign44000_e49180 * locals.var_tkd_dn6) / (locals.var_tkd * locals.var_tkd))))) / (assign44000_e49183 * assign44000_e49183)), (((locals.var_inv_phit0_op_dn7 * assign44000_e49183) - (locals.var_inv_phit0_op * (-((assign44000_e49180 * locals.var_tkd_dn7) / (locals.var_tkd * locals.var_tkd))))) / (assign44000_e49183 * assign44000_e49183)), (((locals.var_inv_phit0_op_dn8 * assign44000_e49183) - (locals.var_inv_phit0_op * (-((assign44000_e49180 * locals.var_tkd_dn8) / (locals.var_tkd * locals.var_tkd))))) / (assign44000_e49183 * assign44000_e49183)), (((locals.var_inv_phit0_op_dn9 * assign44000_e49183) - (locals.var_inv_phit0_op * (-((assign44000_e49180 * locals.var_tkd_dn9) / (locals.var_tkd * locals.var_tkd))))) / (assign44000_e49183 * assign44000_e49183)), );
        locals.var_inv_phit_op_rv = 0.0;

        let assign44010_e49187: f64 = (2.0 * 1.602176565e-19);
        let assign44010_e49189: f64 = (assign44010_e49187 * locals.var_neff_op);
        let assign44010_e49191: f64 = (assign44010_e49189 * locals.var_epsch);
        let assign44010_e49193: f64 = (assign44010_e49191 * locals.var_inv_phit_op);
        (locals.var_a0_csisq_op, locals.var_a0_csisq_op_dn4, locals.var_a0_csisq_op_dn6, locals.var_a0_csisq_op_dn7, locals.var_a0_csisq_op_dn8, locals.var_a0_csisq_op_dn9, ) = (assign44010_e49193, ((((assign44010_e49187 * locals.var_neff_op_dn4) * locals.var_epsch) * locals.var_inv_phit_op) + (assign44010_e49191 * locals.var_inv_phit_op_dn4)), ((((assign44010_e49187 * locals.var_neff_op_dn6) * locals.var_epsch) * locals.var_inv_phit_op) + (assign44010_e49191 * locals.var_inv_phit_op_dn6)), ((((assign44010_e49187 * locals.var_neff_op_dn7) * locals.var_epsch) * locals.var_inv_phit_op) + (assign44010_e49191 * locals.var_inv_phit_op_dn7)), ((((assign44010_e49187 * locals.var_neff_op_dn8) * locals.var_epsch) * locals.var_inv_phit_op) + (assign44010_e49191 * locals.var_inv_phit_op_dn8)), ((((assign44010_e49187 * locals.var_neff_op_dn9) * locals.var_epsch) * locals.var_inv_phit_op) + (assign44010_e49191 * locals.var_inv_phit_op_dn9)), );
        locals.var_a0_csisq_op_rv = 0.0;

        let assign44020_e49196: f64 = (locals.var_csiprime_0 * locals.var_csiprime_0);
        let assign44020_e49198: f64 = (assign44020_e49196 / locals.var_a0_csisq_op);
        let assign44020_e49199: f64 = (assign44020_e49198).ln();
        let assign44020_e49201: f64 = (assign44020_e49199 - 0.6931471805599);
        let assign44020_e49203: f64 = (assign44020_e49201 + locals.var_eg_2phit0_op);
        (locals.var_xth_1d_op, locals.var_xth_1d_op_dn4, locals.var_xth_1d_op_dn6, locals.var_xth_1d_op_dn7, locals.var_xth_1d_op_dn8, locals.var_xth_1d_op_dn9, ) = (assign44020_e49203, (((-((assign44020_e49196 * locals.var_a0_csisq_op_dn4) / (locals.var_a0_csisq_op * locals.var_a0_csisq_op))) / assign44020_e49198) + locals.var_eg_2phit0_op_dn4), (((-((assign44020_e49196 * locals.var_a0_csisq_op_dn6) / (locals.var_a0_csisq_op * locals.var_a0_csisq_op))) / assign44020_e49198) + locals.var_eg_2phit0_op_dn6), (((-((assign44020_e49196 * locals.var_a0_csisq_op_dn7) / (locals.var_a0_csisq_op * locals.var_a0_csisq_op))) / assign44020_e49198) + locals.var_eg_2phit0_op_dn7), (((-((assign44020_e49196 * locals.var_a0_csisq_op_dn8) / (locals.var_a0_csisq_op * locals.var_a0_csisq_op))) / assign44020_e49198) + locals.var_eg_2phit0_op_dn8), (((-((assign44020_e49196 * locals.var_a0_csisq_op_dn9) / (locals.var_a0_csisq_op * locals.var_a0_csisq_op))) / assign44020_e49198) + locals.var_eg_2phit0_op_dn9), );
        locals.var_xth_1d_op_rv = 0.0;

        let assign44030_e49206: f64 = (0.5 * 1.602176565e-19);
        let assign44030_e49208: f64 = (assign44030_e49206 * locals.var_nsddc_i);
        let assign44030_e49210: f64 = (assign44030_e49208 * locals.var_tsi_i);
        let assign44030_e49213: f64 = (locals.var_cox1prime + locals.var_cox2prime);
        let assign44030_e49214: f64 = (assign44030_e49210 / assign44030_e49213);
        let assign44030_e49216: f64 = (assign44030_e49214 * locals.var_inv_phit_op);
        (locals.var_xsddep_op, locals.var_xsddep_op_dn4, locals.var_xsddep_op_dn6, locals.var_xsddep_op_dn7, locals.var_xsddep_op_dn8, locals.var_xsddep_op_dn9, ) = (assign44030_e49216, (assign44030_e49214 * locals.var_inv_phit_op_dn4), (assign44030_e49214 * locals.var_inv_phit_op_dn6), (assign44030_e49214 * locals.var_inv_phit_op_dn7), (assign44030_e49214 * locals.var_inv_phit_op_dn8), (assign44030_e49214 * locals.var_inv_phit_op_dn9), );
        locals.var_xsddep_op_rv = 0.0;

        let assign44040_e49219: f64 = (locals.var_cfd_i * locals.var_inv_phit_op);
        (locals.var_xd0_op, locals.var_xd0_op_dn4, locals.var_xd0_op_dn6, locals.var_xd0_op_dn7, locals.var_xd0_op_dn8, locals.var_xd0_op_dn9, ) = (assign44040_e49219, (locals.var_cfd_i * locals.var_inv_phit_op_dn4), (locals.var_cfd_i * locals.var_inv_phit_op_dn6), (locals.var_cfd_i * locals.var_inv_phit_op_dn7), (locals.var_cfd_i * locals.var_inv_phit_op_dn8), (locals.var_cfd_i * locals.var_inv_phit_op_dn9), );
        locals.var_xd0_op_rv = 0.0;

        (locals.var_qq_op, locals.var_qq_op_dn4, locals.var_qq_op_dn6, locals.var_qq_op_dn7, locals.var_qq_op_dn8, locals.var_qq_op_dn9, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
        locals.var_qq_op_rv = 0.0;

        (locals.var_dvfbpdep_op, locals.var_dvfbpdep_op_dn4, locals.var_dvfbpdep_op_dn6, locals.var_dvfbpdep_op_dn7, locals.var_dvfbpdep_op_dn8, locals.var_dvfbpdep_op_dn9, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
        locals.var_dvfbpdep_op_rv = 0.0;

        let assign44070_e49224: f64 = if p.p9 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1356 = assign44070_e49224;
        locals.var_guard1356_rv = 0.0;

        if (locals.var_guard1356 != 0.0) {
            let assign44080_e49228: f64 = (1.0 / locals.var_inv_phit0_op);
            let assign44080_e49231: f64 = (locals.var_np_i / locals.var_neff_poly);
            let assign44080_e49232: f64 = (assign44080_e49231).ln();
            let assign44080_e49233: f64 = (assign44080_e49228 * assign44080_e49232);
            (locals.var_dvfbpdep_op, locals.var_dvfbpdep_op_dn4, locals.var_dvfbpdep_op_dn6, locals.var_dvfbpdep_op_dn7, locals.var_dvfbpdep_op_dn8, locals.var_dvfbpdep_op_dn9, ) = (assign44080_e49233, (((-(locals.var_inv_phit0_op_dn4 / (locals.var_inv_phit0_op * locals.var_inv_phit0_op))) * assign44080_e49232) + (assign44080_e49228 * ((((locals.var_np_i_dn4 * locals.var_neff_poly) - (locals.var_np_i * locals.var_neff_poly_dn4)) / (locals.var_neff_poly * locals.var_neff_poly)) / assign44080_e49231))), (((-(locals.var_inv_phit0_op_dn6 / (locals.var_inv_phit0_op * locals.var_inv_phit0_op))) * assign44080_e49232) + (assign44080_e49228 * ((((locals.var_np_i_dn6 * locals.var_neff_poly) - (locals.var_np_i * locals.var_neff_poly_dn6)) / (locals.var_neff_poly * locals.var_neff_poly)) / assign44080_e49231))), (((-(locals.var_inv_phit0_op_dn7 / (locals.var_inv_phit0_op * locals.var_inv_phit0_op))) * assign44080_e49232) + (assign44080_e49228 * ((((locals.var_np_i_dn7 * locals.var_neff_poly) - (locals.var_np_i * locals.var_neff_poly_dn7)) / (locals.var_neff_poly * locals.var_neff_poly)) / assign44080_e49231))), (((-(locals.var_inv_phit0_op_dn8 / (locals.var_inv_phit0_op * locals.var_inv_phit0_op))) * assign44080_e49232) + (assign44080_e49228 * ((((locals.var_np_i_dn8 * locals.var_neff_poly) - (locals.var_np_i * locals.var_neff_poly_dn8)) / (locals.var_neff_poly * locals.var_neff_poly)) / assign44080_e49231))), (((-(locals.var_inv_phit0_op_dn9 / (locals.var_inv_phit0_op * locals.var_inv_phit0_op))) * assign44080_e49232) + (assign44080_e49228 * ((((locals.var_np_i_dn9 * locals.var_neff_poly) - (locals.var_np_i * locals.var_neff_poly_dn9)) / (locals.var_neff_poly * locals.var_neff_poly)) / assign44080_e49231))), );
            locals.var_dvfbpdep_op_rv = 0.0;
        }

        let assign44090_e49238: f64 = if p.p13 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1357 = assign44090_e49238;
        locals.var_guard1357_rv = 0.0;

        let assign44100_e49241: f64 = 1.0;
        let assign44100_e49242: f64 = if p.p14 == assign44100_e49241 { 1.0 } else { 0.0 };
        locals.var_guard1358 = assign44100_e49242;
        locals.var_guard1358_rv = 0.0;

        if ((locals.var_guard1357 != 0.0) && (locals.var_guard1358 != 0.0)) {
            let assign44110_e49248: f64 = (0.4 * p.p13);
            let assign44110_e49250: f64 = (assign44110_e49248 * 1.27520989);
            let assign44110_e49252: f64 = (-0.3333333333333);
            let assign44110_e49255: f64 = (locals.var_tsisq / locals.var_inv_phit_op);
            let assign44110_e49256: f64 = (assign44110_e49255).ln();
            let assign44110_e49257: f64 = (assign44110_e49252 * assign44110_e49256);
            let assign44110_e49258: f64 = (assign44110_e49257).exp();
            let assign44110_e49259: f64 = (assign44110_e49250 * assign44110_e49258);
            (locals.var_qq_op, locals.var_qq_op_dn4, locals.var_qq_op_dn6, locals.var_qq_op_dn7, locals.var_qq_op_dn8, locals.var_qq_op_dn9, ) = (assign44110_e49259, (assign44110_e49250 * (assign44110_e49258 * (assign44110_e49252 * ((-((locals.var_tsisq * locals.var_inv_phit_op_dn4) / (locals.var_inv_phit_op * locals.var_inv_phit_op))) / assign44110_e49255)))), (assign44110_e49250 * (assign44110_e49258 * (assign44110_e49252 * ((-((locals.var_tsisq * locals.var_inv_phit_op_dn6) / (locals.var_inv_phit_op * locals.var_inv_phit_op))) / assign44110_e49255)))), (assign44110_e49250 * (assign44110_e49258 * (assign44110_e49252 * ((-((locals.var_tsisq * locals.var_inv_phit_op_dn7) / (locals.var_inv_phit_op * locals.var_inv_phit_op))) / assign44110_e49255)))), (assign44110_e49250 * (assign44110_e49258 * (assign44110_e49252 * ((-((locals.var_tsisq * locals.var_inv_phit_op_dn8) / (locals.var_inv_phit_op * locals.var_inv_phit_op))) / assign44110_e49255)))), (assign44110_e49250 * (assign44110_e49258 * (assign44110_e49252 * ((-((locals.var_tsisq * locals.var_inv_phit_op_dn9) / (locals.var_inv_phit_op * locals.var_inv_phit_op))) / assign44110_e49255)))), );
            locals.var_qq_op_rv = 0.0;
        }

        if ((locals.var_guard1357 != 0.0) && (locals.var_guard1358 == 0.0)) {
            let assign44120_e49268: f64 = (0.4 * p.p13);
            let assign44120_e49270: f64 = (assign44120_e49268 * 1.5412087);
            let assign44120_e49272: f64 = (-0.3333333333333);
            let assign44120_e49275: f64 = (locals.var_tsisq / locals.var_inv_phit_op);
            let assign44120_e49276: f64 = (assign44120_e49275).ln();
            let assign44120_e49277: f64 = (assign44120_e49272 * assign44120_e49276);
            let assign44120_e49278: f64 = (assign44120_e49277).exp();
            let assign44120_e49279: f64 = (assign44120_e49270 * assign44120_e49278);
            (locals.var_qq_op, locals.var_qq_op_dn4, locals.var_qq_op_dn6, locals.var_qq_op_dn7, locals.var_qq_op_dn8, locals.var_qq_op_dn9, ) = (assign44120_e49279, (assign44120_e49270 * (assign44120_e49278 * (assign44120_e49272 * ((-((locals.var_tsisq * locals.var_inv_phit_op_dn4) / (locals.var_inv_phit_op * locals.var_inv_phit_op))) / assign44120_e49275)))), (assign44120_e49270 * (assign44120_e49278 * (assign44120_e49272 * ((-((locals.var_tsisq * locals.var_inv_phit_op_dn6) / (locals.var_inv_phit_op * locals.var_inv_phit_op))) / assign44120_e49275)))), (assign44120_e49270 * (assign44120_e49278 * (assign44120_e49272 * ((-((locals.var_tsisq * locals.var_inv_phit_op_dn7) / (locals.var_inv_phit_op * locals.var_inv_phit_op))) / assign44120_e49275)))), (assign44120_e49270 * (assign44120_e49278 * (assign44120_e49272 * ((-((locals.var_tsisq * locals.var_inv_phit_op_dn8) / (locals.var_inv_phit_op * locals.var_inv_phit_op))) / assign44120_e49275)))), (assign44120_e49270 * (assign44120_e49278 * (assign44120_e49272 * ((-((locals.var_tsisq * locals.var_inv_phit_op_dn9) / (locals.var_inv_phit_op * locals.var_inv_phit_op))) / assign44120_e49275)))), );
            locals.var_qq_op_rv = 0.0;
        }

        let assign44130_e49284: f64 = (locals.var_vds * locals.var_inv_phit_op);
        (locals.var_xd_op, locals.var_xd_op_dn4, locals.var_xd_op_dn6, locals.var_xd_op_dn7, locals.var_xd_op_dn8, locals.var_xd_op_dn9, ) = (assign44130_e49284, (locals.var_vds * locals.var_inv_phit_op_dn4), ((locals.var_vds_dn6 * locals.var_inv_phit_op) + (locals.var_vds * locals.var_inv_phit_op_dn6)), ((locals.var_vds_dn7 * locals.var_inv_phit_op) + (locals.var_vds * locals.var_inv_phit_op_dn7)), (locals.var_vds * locals.var_inv_phit_op_dn8), (locals.var_vds * locals.var_inv_phit_op_dn9), );
        locals.var_xd_op_rv = 0.0;

        let assign44140_e49287: f64 = (locals.var_vds * locals.var_vds);
        let assign44140_e49289: f64 = (assign44140_e49287 + 0.01);
        let assign44140_e49290: f64 = (assign44140_e49289).sqrt();
        let assign44140_e49292: f64 = (assign44140_e49290 - 0.1);
        let assign44140_e49294: f64 = (assign44140_e49292 * locals.var_inv_phit_op);
        (locals.var_xdsx_op, locals.var_xdsx_op_dn4, locals.var_xdsx_op_dn6, locals.var_xdsx_op_dn7, locals.var_xdsx_op_dn8, locals.var_xdsx_op_dn9, ) = (assign44140_e49294, (assign44140_e49292 * locals.var_inv_phit_op_dn4), (((((locals.var_vds_dn6 * locals.var_vds) + (locals.var_vds * locals.var_vds_dn6)) / (2.0 * assign44140_e49290)) * locals.var_inv_phit_op) + (assign44140_e49292 * locals.var_inv_phit_op_dn6)), (((((locals.var_vds_dn7 * locals.var_vds) + (locals.var_vds * locals.var_vds_dn7)) / (2.0 * assign44140_e49290)) * locals.var_inv_phit_op) + (assign44140_e49292 * locals.var_inv_phit_op_dn7)), (assign44140_e49292 * locals.var_inv_phit_op_dn8), (assign44140_e49292 * locals.var_inv_phit_op_dn9), );
        locals.var_xdsx_op_rv = 0.0;

        let assign44150_e49298: f64 = (locals.var_xd_op - locals.var_xdsx_op);
        let assign44150_e49299: f64 = (0.5 * assign44150_e49298);
        (locals.var_dxdsx_op, locals.var_dxdsx_op_dn4, locals.var_dxdsx_op_dn6, locals.var_dxdsx_op_dn7, locals.var_dxdsx_op_dn8, locals.var_dxdsx_op_dn9, ) = (assign44150_e49299, (0.5 * (locals.var_xd_op_dn4 - locals.var_xdsx_op_dn4)), (0.5 * (locals.var_xd_op_dn6 - locals.var_xdsx_op_dn6)), (0.5 * (locals.var_xd_op_dn7 - locals.var_xdsx_op_dn7)), (0.5 * (locals.var_xd_op_dn8 - locals.var_xdsx_op_dn8)), (0.5 * (locals.var_xd_op_dn9 - locals.var_xdsx_op_dn9)), );
        locals.var_dxdsx_op_rv = 0.0;

        let assign44160_e49302: f64 = (locals.var_k2_dc / locals.var_k1_dc);
        let assign44160_e49305: f64 = (1.0 + locals.var_k2_dc);
        let assign44160_e49306: f64 = (assign44160_e49302 / assign44160_e49305);
        (locals.var_r1init_op, locals.var_r1init_op_dn4, locals.var_r1init_op_dn6, locals.var_r1init_op_dn7, locals.var_r1init_op_dn8, locals.var_r1init_op_dn9, ) = (assign44160_e49306, ((((((locals.var_k2_dc_dn4 * locals.var_k1_dc) - (locals.var_k2_dc * locals.var_k1_dc_dn4)) / (locals.var_k1_dc * locals.var_k1_dc)) * assign44160_e49305) - (assign44160_e49302 * locals.var_k2_dc_dn4)) / (assign44160_e49305 * assign44160_e49305)), ((((((locals.var_k2_dc_dn6 * locals.var_k1_dc) - (locals.var_k2_dc * locals.var_k1_dc_dn6)) / (locals.var_k1_dc * locals.var_k1_dc)) * assign44160_e49305) - (assign44160_e49302 * locals.var_k2_dc_dn6)) / (assign44160_e49305 * assign44160_e49305)), ((((((locals.var_k2_dc_dn7 * locals.var_k1_dc) - (locals.var_k2_dc * locals.var_k1_dc_dn7)) / (locals.var_k1_dc * locals.var_k1_dc)) * assign44160_e49305) - (assign44160_e49302 * locals.var_k2_dc_dn7)) / (assign44160_e49305 * assign44160_e49305)), ((((((locals.var_k2_dc_dn8 * locals.var_k1_dc) - (locals.var_k2_dc * locals.var_k1_dc_dn8)) / (locals.var_k1_dc * locals.var_k1_dc)) * assign44160_e49305) - (assign44160_e49302 * locals.var_k2_dc_dn8)) / (assign44160_e49305 * assign44160_e49305)), ((((((locals.var_k2_dc_dn9 * locals.var_k1_dc) - (locals.var_k2_dc * locals.var_k1_dc_dn9)) / (locals.var_k1_dc * locals.var_k1_dc)) * assign44160_e49305) - (assign44160_e49302 * locals.var_k2_dc_dn9)) / (assign44160_e49305 * assign44160_e49305)), );
        locals.var_r1init_op_rv = 0.0;

        let assign44170_e49309: f64 = (locals.var_k1_dc / locals.var_k2_dc);
        let assign44170_e49312: f64 = (1.0 + locals.var_k1_dc);
        let assign44170_e49313: f64 = (assign44170_e49309 / assign44170_e49312);
        (locals.var_r2init_op, locals.var_r2init_op_dn4, locals.var_r2init_op_dn6, locals.var_r2init_op_dn7, locals.var_r2init_op_dn8, locals.var_r2init_op_dn9, ) = (assign44170_e49313, ((((((locals.var_k1_dc_dn4 * locals.var_k2_dc) - (locals.var_k1_dc * locals.var_k2_dc_dn4)) / (locals.var_k2_dc * locals.var_k2_dc)) * assign44170_e49312) - (assign44170_e49309 * locals.var_k1_dc_dn4)) / (assign44170_e49312 * assign44170_e49312)), ((((((locals.var_k1_dc_dn6 * locals.var_k2_dc) - (locals.var_k1_dc * locals.var_k2_dc_dn6)) / (locals.var_k2_dc * locals.var_k2_dc)) * assign44170_e49312) - (assign44170_e49309 * locals.var_k1_dc_dn6)) / (assign44170_e49312 * assign44170_e49312)), ((((((locals.var_k1_dc_dn7 * locals.var_k2_dc) - (locals.var_k1_dc * locals.var_k2_dc_dn7)) / (locals.var_k2_dc * locals.var_k2_dc)) * assign44170_e49312) - (assign44170_e49309 * locals.var_k1_dc_dn7)) / (assign44170_e49312 * assign44170_e49312)), ((((((locals.var_k1_dc_dn8 * locals.var_k2_dc) - (locals.var_k1_dc * locals.var_k2_dc_dn8)) / (locals.var_k2_dc * locals.var_k2_dc)) * assign44170_e49312) - (assign44170_e49309 * locals.var_k1_dc_dn8)) / (assign44170_e49312 * assign44170_e49312)), ((((((locals.var_k1_dc_dn9 * locals.var_k2_dc) - (locals.var_k1_dc * locals.var_k2_dc_dn9)) / (locals.var_k2_dc * locals.var_k2_dc)) * assign44170_e49312) - (assign44170_e49309 * locals.var_k1_dc_dn9)) / (assign44170_e49312 * assign44170_e49312)), );
        locals.var_r2init_op_rv = 0.0;

        let assign44180_e49317: f64 = (1.0 + locals.var_r1init_op);
        let assign44180_e49318: f64 = (locals.var_k1_dc * assign44180_e49317);
        let assign44180_e49320: f64 = (assign44180_e49318 * locals.var_diff_min_dc);
        let assign44180_e49322: f64 = (assign44180_e49320 / locals.var_a0_dc);
        let assign44180_e49323: f64 = (assign44180_e49322).ln();
        let assign44180_e49325: f64 = (assign44180_e49323 + 2.0);
        (locals.var_x1init_op, locals.var_x1init_op_dn4, locals.var_x1init_op_dn6, locals.var_x1init_op_dn7, locals.var_x1init_op_dn8, locals.var_x1init_op_dn9, ) = (assign44180_e49325, ((((((((locals.var_k1_dc_dn4 * assign44180_e49317) + (locals.var_k1_dc * locals.var_r1init_op_dn4)) * locals.var_diff_min_dc) + (assign44180_e49318 * locals.var_diff_min_dc_dn4)) * locals.var_a0_dc) - (assign44180_e49320 * locals.var_a0_dc_dn4)) / (locals.var_a0_dc * locals.var_a0_dc)) / assign44180_e49322), ((((((((locals.var_k1_dc_dn6 * assign44180_e49317) + (locals.var_k1_dc * locals.var_r1init_op_dn6)) * locals.var_diff_min_dc) + (assign44180_e49318 * locals.var_diff_min_dc_dn6)) * locals.var_a0_dc) - (assign44180_e49320 * locals.var_a0_dc_dn6)) / (locals.var_a0_dc * locals.var_a0_dc)) / assign44180_e49322), ((((((((locals.var_k1_dc_dn7 * assign44180_e49317) + (locals.var_k1_dc * locals.var_r1init_op_dn7)) * locals.var_diff_min_dc) + (assign44180_e49318 * locals.var_diff_min_dc_dn7)) * locals.var_a0_dc) - (assign44180_e49320 * locals.var_a0_dc_dn7)) / (locals.var_a0_dc * locals.var_a0_dc)) / assign44180_e49322), ((((((((locals.var_k1_dc_dn8 * assign44180_e49317) + (locals.var_k1_dc * locals.var_r1init_op_dn8)) * locals.var_diff_min_dc) + (assign44180_e49318 * locals.var_diff_min_dc_dn8)) * locals.var_a0_dc) - (assign44180_e49320 * locals.var_a0_dc_dn8)) / (locals.var_a0_dc * locals.var_a0_dc)) / assign44180_e49322), ((((((((locals.var_k1_dc_dn9 * assign44180_e49317) + (locals.var_k1_dc * locals.var_r1init_op_dn9)) * locals.var_diff_min_dc) + (assign44180_e49318 * locals.var_diff_min_dc_dn9)) * locals.var_a0_dc) - (assign44180_e49320 * locals.var_a0_dc_dn9)) / (locals.var_a0_dc * locals.var_a0_dc)) / assign44180_e49322), );
        locals.var_x1init_op_rv = 0.0;

        let assign44190_e49329: f64 = (1.0 + locals.var_r2init_op);
        let assign44190_e49330: f64 = (locals.var_k2_dc * assign44190_e49329);
        let assign44190_e49332: f64 = (assign44190_e49330 * locals.var_diff_min_dc);
        let assign44190_e49334: f64 = (assign44190_e49332 / locals.var_a0_dc);
        let assign44190_e49335: f64 = (assign44190_e49334).ln();
        let assign44190_e49337: f64 = (assign44190_e49335 + 2.0);
        (locals.var_x2init_op, locals.var_x2init_op_dn4, locals.var_x2init_op_dn6, locals.var_x2init_op_dn7, locals.var_x2init_op_dn8, locals.var_x2init_op_dn9, ) = (assign44190_e49337, ((((((((locals.var_k2_dc_dn4 * assign44190_e49329) + (locals.var_k2_dc * locals.var_r2init_op_dn4)) * locals.var_diff_min_dc) + (assign44190_e49330 * locals.var_diff_min_dc_dn4)) * locals.var_a0_dc) - (assign44190_e49332 * locals.var_a0_dc_dn4)) / (locals.var_a0_dc * locals.var_a0_dc)) / assign44190_e49334), ((((((((locals.var_k2_dc_dn6 * assign44190_e49329) + (locals.var_k2_dc * locals.var_r2init_op_dn6)) * locals.var_diff_min_dc) + (assign44190_e49330 * locals.var_diff_min_dc_dn6)) * locals.var_a0_dc) - (assign44190_e49332 * locals.var_a0_dc_dn6)) / (locals.var_a0_dc * locals.var_a0_dc)) / assign44190_e49334), ((((((((locals.var_k2_dc_dn7 * assign44190_e49329) + (locals.var_k2_dc * locals.var_r2init_op_dn7)) * locals.var_diff_min_dc) + (assign44190_e49330 * locals.var_diff_min_dc_dn7)) * locals.var_a0_dc) - (assign44190_e49332 * locals.var_a0_dc_dn7)) / (locals.var_a0_dc * locals.var_a0_dc)) / assign44190_e49334), ((((((((locals.var_k2_dc_dn8 * assign44190_e49329) + (locals.var_k2_dc * locals.var_r2init_op_dn8)) * locals.var_diff_min_dc) + (assign44190_e49330 * locals.var_diff_min_dc_dn8)) * locals.var_a0_dc) - (assign44190_e49332 * locals.var_a0_dc_dn8)) / (locals.var_a0_dc * locals.var_a0_dc)) / assign44190_e49334), ((((((((locals.var_k2_dc_dn9 * assign44190_e49329) + (locals.var_k2_dc * locals.var_r2init_op_dn9)) * locals.var_diff_min_dc) + (assign44190_e49330 * locals.var_diff_min_dc_dn9)) * locals.var_a0_dc) - (assign44190_e49332 * locals.var_a0_dc_dn9)) / (locals.var_a0_dc * locals.var_a0_dc)) / assign44190_e49334), );
        locals.var_x2init_op_rv = 0.0;

        let assign44200_e49340: f64 = (1.0 + locals.var_r1init_op);
        let assign44200_e49342: f64 = (assign44200_e49340 * locals.var_x1init_op);
        let assign44200_e49345: f64 = (locals.var_xg2x_dc * locals.var_r1init_op);
        let assign44200_e49346: f64 = (assign44200_e49342 - assign44200_e49345);
        (locals.var_xth1init_op, locals.var_xth1init_op_dn4, locals.var_xth1init_op_dn6, locals.var_xth1init_op_dn7, locals.var_xth1init_op_dn8, locals.var_xth1init_op_dn9, ) = (assign44200_e49346, (((locals.var_r1init_op_dn4 * locals.var_x1init_op) + (assign44200_e49340 * locals.var_x1init_op_dn4)) - ((locals.var_xg2x_dc_dn4 * locals.var_r1init_op) + (locals.var_xg2x_dc * locals.var_r1init_op_dn4))), (((locals.var_r1init_op_dn6 * locals.var_x1init_op) + (assign44200_e49340 * locals.var_x1init_op_dn6)) - ((locals.var_xg2x_dc_dn6 * locals.var_r1init_op) + (locals.var_xg2x_dc * locals.var_r1init_op_dn6))), (((locals.var_r1init_op_dn7 * locals.var_x1init_op) + (assign44200_e49340 * locals.var_x1init_op_dn7)) - ((locals.var_xg2x_dc_dn7 * locals.var_r1init_op) + (locals.var_xg2x_dc * locals.var_r1init_op_dn7))), (((locals.var_r1init_op_dn8 * locals.var_x1init_op) + (assign44200_e49340 * locals.var_x1init_op_dn8)) - ((locals.var_xg2x_dc_dn8 * locals.var_r1init_op) + (locals.var_xg2x_dc * locals.var_r1init_op_dn8))), (((locals.var_r1init_op_dn9 * locals.var_x1init_op) + (assign44200_e49340 * locals.var_x1init_op_dn9)) - ((locals.var_xg2x_dc_dn9 * locals.var_r1init_op) + (locals.var_xg2x_dc * locals.var_r1init_op_dn9))), );
        locals.var_xth1init_op_rv = 0.0;

        let assign44210_e49350: f64 = (1.0 / locals.var_r2init_op);
        let assign44210_e49351: f64 = (1.0 + assign44210_e49350);
        let assign44210_e49353: f64 = (assign44210_e49351 * locals.var_x2init_op);
        let assign44210_e49356: f64 = (locals.var_xg2x_dc / locals.var_r2init_op);
        let assign44210_e49357: f64 = (assign44210_e49353 - assign44210_e49356);
        (locals.var_xth2init_op, locals.var_xth2init_op_dn4, locals.var_xth2init_op_dn6, locals.var_xth2init_op_dn7, locals.var_xth2init_op_dn8, locals.var_xth2init_op_dn9, ) = (assign44210_e49357, ((((-(locals.var_r2init_op_dn4 / (locals.var_r2init_op * locals.var_r2init_op))) * locals.var_x2init_op) + (assign44210_e49351 * locals.var_x2init_op_dn4)) - (((locals.var_xg2x_dc_dn4 * locals.var_r2init_op) - (locals.var_xg2x_dc * locals.var_r2init_op_dn4)) / (locals.var_r2init_op * locals.var_r2init_op))), ((((-(locals.var_r2init_op_dn6 / (locals.var_r2init_op * locals.var_r2init_op))) * locals.var_x2init_op) + (assign44210_e49351 * locals.var_x2init_op_dn6)) - (((locals.var_xg2x_dc_dn6 * locals.var_r2init_op) - (locals.var_xg2x_dc * locals.var_r2init_op_dn6)) / (locals.var_r2init_op * locals.var_r2init_op))), ((((-(locals.var_r2init_op_dn7 / (locals.var_r2init_op * locals.var_r2init_op))) * locals.var_x2init_op) + (assign44210_e49351 * locals.var_x2init_op_dn7)) - (((locals.var_xg2x_dc_dn7 * locals.var_r2init_op) - (locals.var_xg2x_dc * locals.var_r2init_op_dn7)) / (locals.var_r2init_op * locals.var_r2init_op))), ((((-(locals.var_r2init_op_dn8 / (locals.var_r2init_op * locals.var_r2init_op))) * locals.var_x2init_op) + (assign44210_e49351 * locals.var_x2init_op_dn8)) - (((locals.var_xg2x_dc_dn8 * locals.var_r2init_op) - (locals.var_xg2x_dc * locals.var_r2init_op_dn8)) / (locals.var_r2init_op * locals.var_r2init_op))), ((((-(locals.var_r2init_op_dn9 / (locals.var_r2init_op * locals.var_r2init_op))) * locals.var_x2init_op) + (assign44210_e49351 * locals.var_x2init_op_dn9)) - (((locals.var_xg2x_dc_dn9 * locals.var_r2init_op) - (locals.var_xg2x_dc * locals.var_r2init_op_dn9)) / (locals.var_r2init_op * locals.var_r2init_op))), );
        locals.var_xth2init_op_rv = 0.0;

        let assign44220_e49361: f64 = (locals.var_xth1init_op + locals.var_xth2init_op);
        let assign44220_e49364: f64 = (locals.var_xth1init_op - locals.var_xth2init_op);
        let assign44220_e49367: f64 = (locals.var_xth1init_op - locals.var_xth2init_op);
        let assign44220_e49368: f64 = (assign44220_e49364 * assign44220_e49367);
        let assign44220_e49370: f64 = (assign44220_e49368 + 38.0);
        let assign44220_e49371: f64 = (assign44220_e49370).sqrt();
        let assign44220_e49372: f64 = (assign44220_e49361 - assign44220_e49371);
        let assign44220_e49373: f64 = (0.5 * assign44220_e49372);
        let assign44220_e49375: f64 = (assign44220_e49373 - locals.var_xg2_dc);
        let assign44220_e49377: f64 = (assign44220_e49375 / locals.var_cic1_i);
        let assign44220_e49379: f64 = (assign44220_e49377 + locals.var_xg2_dc);
        (locals.var_xg1thinit_op, locals.var_xg1thinit_op_dn4, locals.var_xg1thinit_op_dn6, locals.var_xg1thinit_op_dn7, locals.var_xg1thinit_op_dn8, locals.var_xg1thinit_op_dn9, ) = (assign44220_e49379, ((((0.5 * ((locals.var_xth1init_op_dn4 + locals.var_xth2init_op_dn4) - ((((locals.var_xth1init_op_dn4 - locals.var_xth2init_op_dn4) * assign44220_e49367) + (assign44220_e49364 * (locals.var_xth1init_op_dn4 - locals.var_xth2init_op_dn4))) / (2.0 * assign44220_e49371)))) - locals.var_xg2_dc_dn4) / locals.var_cic1_i) + locals.var_xg2_dc_dn4), ((((0.5 * ((locals.var_xth1init_op_dn6 + locals.var_xth2init_op_dn6) - ((((locals.var_xth1init_op_dn6 - locals.var_xth2init_op_dn6) * assign44220_e49367) + (assign44220_e49364 * (locals.var_xth1init_op_dn6 - locals.var_xth2init_op_dn6))) / (2.0 * assign44220_e49371)))) - locals.var_xg2_dc_dn6) / locals.var_cic1_i) + locals.var_xg2_dc_dn6), ((((0.5 * ((locals.var_xth1init_op_dn7 + locals.var_xth2init_op_dn7) - ((((locals.var_xth1init_op_dn7 - locals.var_xth2init_op_dn7) * assign44220_e49367) + (assign44220_e49364 * (locals.var_xth1init_op_dn7 - locals.var_xth2init_op_dn7))) / (2.0 * assign44220_e49371)))) - locals.var_xg2_dc_dn7) / locals.var_cic1_i) + locals.var_xg2_dc_dn7), ((((0.5 * ((locals.var_xth1init_op_dn8 + locals.var_xth2init_op_dn8) - ((((locals.var_xth1init_op_dn8 - locals.var_xth2init_op_dn8) * assign44220_e49367) + (assign44220_e49364 * (locals.var_xth1init_op_dn8 - locals.var_xth2init_op_dn8))) / (2.0 * assign44220_e49371)))) - locals.var_xg2_dc_dn8) / locals.var_cic1_i) + locals.var_xg2_dc_dn8), ((((0.5 * ((locals.var_xth1init_op_dn9 + locals.var_xth2init_op_dn9) - ((((locals.var_xth1init_op_dn9 - locals.var_xth2init_op_dn9) * assign44220_e49367) + (assign44220_e49364 * (locals.var_xth1init_op_dn9 - locals.var_xth2init_op_dn9))) / (2.0 * assign44220_e49371)))) - locals.var_xg2_dc_dn9) / locals.var_cic1_i) + locals.var_xg2_dc_dn9), );
        locals.var_xg1thinit_op_rv = 0.0;

        let assign44230_e49383: f64 = (locals.var_xg1thinit_op - locals.var_xedge_dc);
        let assign44230_e49385: f64 = (assign44230_e49383 / locals.var_sce1_dc);
        let assign44230_e49387: f64 = (assign44230_e49385 - locals.var_dxg1_dibl_dc);
        let assign44230_e49389: f64 = (assign44230_e49387 + locals.var_xedge_dc);
        let assign44230_e49390: f64 = (locals.var_phit * assign44230_e49389);
        let assign44230_e49392: f64 = (assign44230_e49390 + locals.var_vfb1_i);
        (locals.var_vthinit_op, locals.var_vthinit_op_dn4, locals.var_vthinit_op_dn6, locals.var_vthinit_op_dn7, locals.var_vthinit_op_dn8, locals.var_vthinit_op_dn9, ) = (assign44230_e49392, (((locals.var_phit_dn4 * assign44230_e49389) + (locals.var_phit * ((((((locals.var_xg1thinit_op_dn4 - locals.var_xedge_dc_dn4) * locals.var_sce1_dc) - (assign44230_e49383 * locals.var_sce1_dc_dn4)) / (locals.var_sce1_dc * locals.var_sce1_dc)) - locals.var_dxg1_dibl_dc_dn4) + locals.var_xedge_dc_dn4))) + locals.var_vfb1_i_dn4), (((locals.var_phit_dn6 * assign44230_e49389) + (locals.var_phit * ((((((locals.var_xg1thinit_op_dn6 - locals.var_xedge_dc_dn6) * locals.var_sce1_dc) - (assign44230_e49383 * locals.var_sce1_dc_dn6)) / (locals.var_sce1_dc * locals.var_sce1_dc)) - locals.var_dxg1_dibl_dc_dn6) + locals.var_xedge_dc_dn6))) + locals.var_vfb1_i_dn6), (((locals.var_phit_dn7 * assign44230_e49389) + (locals.var_phit * ((((((locals.var_xg1thinit_op_dn7 - locals.var_xedge_dc_dn7) * locals.var_sce1_dc) - (assign44230_e49383 * locals.var_sce1_dc_dn7)) / (locals.var_sce1_dc * locals.var_sce1_dc)) - locals.var_dxg1_dibl_dc_dn7) + locals.var_xedge_dc_dn7))) + locals.var_vfb1_i_dn7), (((locals.var_phit_dn8 * assign44230_e49389) + (locals.var_phit * ((((((locals.var_xg1thinit_op_dn8 - locals.var_xedge_dc_dn8) * locals.var_sce1_dc) - (assign44230_e49383 * locals.var_sce1_dc_dn8)) / (locals.var_sce1_dc * locals.var_sce1_dc)) - locals.var_dxg1_dibl_dc_dn8) + locals.var_xedge_dc_dn8))) + locals.var_vfb1_i_dn8), (((locals.var_phit_dn9 * assign44230_e49389) + (locals.var_phit * ((((((locals.var_xg1thinit_op_dn9 - locals.var_xedge_dc_dn9) * locals.var_sce1_dc) - (assign44230_e49383 * locals.var_sce1_dc_dn9)) / (locals.var_sce1_dc * locals.var_sce1_dc)) - locals.var_dxg1_dibl_dc_dn9) + locals.var_xedge_dc_dn9))) + locals.var_vfb1_i_dn9), );
        locals.var_vthinit_op_rv = 0.0;

        let assign44240_e49396: f64 = (locals.var_tkd - locals.var_tkr);
        let assign44240_e49397: f64 = (locals.var_stcf_i * assign44240_e49396);
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign44240_e49397, ((locals.var_stcf_i_dn4 * assign44240_e49396) + (locals.var_stcf_i * locals.var_tkd_dn4)), ((locals.var_stcf_i_dn6 * assign44240_e49396) + (locals.var_stcf_i * locals.var_tkd_dn6)), ((locals.var_stcf_i_dn7 * assign44240_e49396) + (locals.var_stcf_i * locals.var_tkd_dn7)), ((locals.var_stcf_i_dn8 * assign44240_e49396) + (locals.var_stcf_i * locals.var_tkd_dn8)), ((locals.var_stcf_i_dn9 * assign44240_e49396) + (locals.var_stcf_i * locals.var_tkd_dn9)), );
        locals.var_temp_rv = 0.0;

        let assign44270_e49406: f64 = (p.p14 * locals.var_stvfb_i);
        let assign44270_e49409: f64 = (locals.var_tkd - locals.var_tkr);
        let assign44270_e49410: f64 = (assign44270_e49406 * assign44270_e49409);
        let assign44270_e49412: f64 = (assign44270_e49410 + locals.var_dvfbqm);
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign44270_e49412, (assign44270_e49406 * locals.var_tkd_dn4), (assign44270_e49406 * locals.var_tkd_dn6), (assign44270_e49406 * locals.var_tkd_dn7), (assign44270_e49406 * locals.var_tkd_dn8), (assign44270_e49406 * locals.var_tkd_dn9), );
        locals.var_temp_rv = 0.0;

        let assign44280_e49416: f64 = (locals.var_vfb1_t + locals.var_dvfbch_op);
        let assign44280_e49418: f64 = (assign44280_e49416 + locals.var_dvfb1nch);
        let assign44280_e49419: f64 = (p.p14 * assign44280_e49418);
        let assign44280_e49421: f64 = (assign44280_e49419 + locals.var_temp);
        let assign44280_e49423: f64 = (assign44280_e49421 + p.p34);
        let assign44280_e49425: f64 = (assign44280_e49423 - locals.var_dvfbpdep_op);
        (locals.var_vfb1_op, locals.var_vfb1_op_dn4, locals.var_vfb1_op_dn6, locals.var_vfb1_op_dn7, locals.var_vfb1_op_dn8, locals.var_vfb1_op_dn9, ) = (assign44280_e49425, (((p.p14 * ((locals.var_vfb1_t_dn4 + locals.var_dvfbch_op_dn4) + locals.var_dvfb1nch_dn4)) + locals.var_temp_dn4) - locals.var_dvfbpdep_op_dn4), (((p.p14 * ((locals.var_vfb1_t_dn6 + locals.var_dvfbch_op_dn6) + locals.var_dvfb1nch_dn6)) + locals.var_temp_dn6) - locals.var_dvfbpdep_op_dn6), (((p.p14 * ((locals.var_vfb1_t_dn7 + locals.var_dvfbch_op_dn7) + locals.var_dvfb1nch_dn7)) + locals.var_temp_dn7) - locals.var_dvfbpdep_op_dn7), (((p.p14 * ((locals.var_vfb1_t_dn8 + locals.var_dvfbch_op_dn8) + locals.var_dvfb1nch_dn8)) + locals.var_temp_dn8) - locals.var_dvfbpdep_op_dn8), (((p.p14 * ((locals.var_vfb1_t_dn9 + locals.var_dvfbch_op_dn9) + locals.var_dvfb1nch_dn9)) + locals.var_temp_dn9) - locals.var_dvfbpdep_op_dn9), );
        locals.var_vfb1_op_rv = 0.0;

        let assign44290_e49429: f64 = (locals.var_vfb2_t + locals.var_dvfbch_op);
        let assign44290_e49431: f64 = (assign44290_e49429 + locals.var_dvfb2nch);
        let assign44290_e49432: f64 = (p.p14 * assign44290_e49431);
        let assign44290_e49434: f64 = (assign44290_e49432 + locals.var_temp);
        (locals.var_vfb2_op, locals.var_vfb2_op_dn4, locals.var_vfb2_op_dn6, locals.var_vfb2_op_dn7, locals.var_vfb2_op_dn8, locals.var_vfb2_op_dn9, ) = (assign44290_e49434, ((p.p14 * ((locals.var_vfb2_t_dn4 + locals.var_dvfbch_op_dn4) + locals.var_dvfb2nch_dn4)) + locals.var_temp_dn4), ((p.p14 * ((locals.var_vfb2_t_dn6 + locals.var_dvfbch_op_dn6) + locals.var_dvfb2nch_dn6)) + locals.var_temp_dn6), ((p.p14 * ((locals.var_vfb2_t_dn7 + locals.var_dvfbch_op_dn7) + locals.var_dvfb2nch_dn7)) + locals.var_temp_dn7), ((p.p14 * ((locals.var_vfb2_t_dn8 + locals.var_dvfbch_op_dn8) + locals.var_dvfb2nch_dn8)) + locals.var_temp_dn8), ((p.p14 * ((locals.var_vfb2_t_dn9 + locals.var_dvfbch_op_dn9) + locals.var_dvfb2nch_dn9)) + locals.var_temp_dn9), );
        locals.var_vfb2_op_rv = 0.0;

        let assign44300_e49437: f64 = (locals.var_vthinit_op - locals.var_vfb1_op);
        let assign44300_e49439: f64 = (assign44300_e49437 * locals.var_inv_phit_op);
        let assign44300_e49441: f64 = (assign44300_e49439 - locals.var_dxdsx_op);
        (locals.var_xg10_op, locals.var_xg10_op_dn4, locals.var_xg10_op_dn6, locals.var_xg10_op_dn7, locals.var_xg10_op_dn8, locals.var_xg10_op_dn9, ) = (assign44300_e49441, ((((locals.var_vthinit_op_dn4 - locals.var_vfb1_op_dn4) * locals.var_inv_phit_op) + (assign44300_e49437 * locals.var_inv_phit_op_dn4)) - locals.var_dxdsx_op_dn4), ((((locals.var_vthinit_op_dn6 - locals.var_vfb1_op_dn6) * locals.var_inv_phit_op) + (assign44300_e49437 * locals.var_inv_phit_op_dn6)) - locals.var_dxdsx_op_dn6), ((((locals.var_vthinit_op_dn7 - locals.var_vfb1_op_dn7) * locals.var_inv_phit_op) + (assign44300_e49437 * locals.var_inv_phit_op_dn7)) - locals.var_dxdsx_op_dn7), ((((locals.var_vthinit_op_dn8 - locals.var_vfb1_op_dn8) * locals.var_inv_phit_op) + (assign44300_e49437 * locals.var_inv_phit_op_dn8)) - locals.var_dxdsx_op_dn8), ((((locals.var_vthinit_op_dn9 - locals.var_vfb1_op_dn9) * locals.var_inv_phit_op) + (assign44300_e49437 * locals.var_inv_phit_op_dn9)) - locals.var_dxdsx_op_dn9), );
        locals.var_xg10_op_rv = 0.0;

        let assign44310_e49443: f64 = (-locals.var_vsb);
        let assign44310_e49445: f64 = (assign44310_e49443 - locals.var_vfb2_op);
        let assign44310_e49447: f64 = (assign44310_e49445 * locals.var_inv_phit_op);
        let assign44310_e49449: f64 = (assign44310_e49447 - locals.var_dxdsx_op);
        (locals.var_xg20_op, locals.var_xg20_op_dn4, locals.var_xg20_op_dn6, locals.var_xg20_op_dn7, locals.var_xg20_op_dn8, locals.var_xg20_op_dn9, ) = (assign44310_e49449, ((((-locals.var_vfb2_op_dn4) * locals.var_inv_phit_op) + (assign44310_e49445 * locals.var_inv_phit_op_dn4)) - locals.var_dxdsx_op_dn4), (((((-locals.var_vsb_dn6) - locals.var_vfb2_op_dn6) * locals.var_inv_phit_op) + (assign44310_e49445 * locals.var_inv_phit_op_dn6)) - locals.var_dxdsx_op_dn6), (((((-locals.var_vsb_dn7) - locals.var_vfb2_op_dn7) * locals.var_inv_phit_op) + (assign44310_e49445 * locals.var_inv_phit_op_dn7)) - locals.var_dxdsx_op_dn7), (((((-locals.var_vsb_dn8) - locals.var_vfb2_op_dn8) * locals.var_inv_phit_op) + (assign44310_e49445 * locals.var_inv_phit_op_dn8)) - locals.var_dxdsx_op_dn8), ((((-locals.var_vfb2_op_dn9) * locals.var_inv_phit_op) + (assign44310_e49445 * locals.var_inv_phit_op_dn9)) - locals.var_dxdsx_op_dn9), );
        locals.var_xg20_op_rv = 0.0;

        let assign44320_e49452: f64 = if p.p2 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1359 = assign44320_e49452;
        locals.var_guard1359_rv = 0.0;

        if (locals.var_guard1359 != 0.0) {
            let assign44330_e49456: f64 = (p.p14 * locals.var_typesub_i);
            let assign44330_e49459: f64 = (locals.var_xg10_op - locals.var_xg20_op);
            let assign44330_e49460: f64 = (assign44330_e49456 * assign44330_e49459);
            let assign44330_e49462: f64 = (assign44330_e49460 / locals.var_gfsub);
            (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign44330_e49462, ((((assign44330_e49456 * (locals.var_xg10_op_dn4 - locals.var_xg20_op_dn4)) * locals.var_gfsub) - (assign44330_e49460 * locals.var_gfsub_dn4)) / (locals.var_gfsub * locals.var_gfsub)), ((((assign44330_e49456 * (locals.var_xg10_op_dn6 - locals.var_xg20_op_dn6)) * locals.var_gfsub) - (assign44330_e49460 * locals.var_gfsub_dn6)) / (locals.var_gfsub * locals.var_gfsub)), ((((assign44330_e49456 * (locals.var_xg10_op_dn7 - locals.var_xg20_op_dn7)) * locals.var_gfsub) - (assign44330_e49460 * locals.var_gfsub_dn7)) / (locals.var_gfsub * locals.var_gfsub)), ((((assign44330_e49456 * (locals.var_xg10_op_dn8 - locals.var_xg20_op_dn8)) * locals.var_gfsub) - (assign44330_e49460 * locals.var_gfsub_dn8)) / (locals.var_gfsub * locals.var_gfsub)), ((((assign44330_e49456 * (locals.var_xg10_op_dn9 - locals.var_xg20_op_dn9)) * locals.var_gfsub) - (assign44330_e49460 * locals.var_gfsub_dn9)) / (locals.var_gfsub * locals.var_gfsub)), );
            locals.var_temp_rv = 0.0;
        }

        let assign44340_e49467: f64 = if locals.var_temp < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1360 = assign44340_e49467;
        locals.var_guard1360_rv = 0.0;

        if ((locals.var_guard1359 != 0.0) && (locals.var_guard1360 != 0.0)) {
            let assign44350_e49472: f64 = (-2.0);
            let assign44350_e49475: f64 = (1.0 - locals.var_temp);
            let assign44350_e49476: f64 = (assign44350_e49475).ln();
            let assign44350_e49477: f64 = (assign44350_e49472 * assign44350_e49476);
            (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9, ) = (assign44350_e49477, (assign44350_e49472 * ((-locals.var_temp_dn4) / assign44350_e49475)), (assign44350_e49472 * ((-locals.var_temp_dn6) / assign44350_e49475)), (assign44350_e49472 * ((-locals.var_temp_dn7) / assign44350_e49475)), (assign44350_e49472 * ((-locals.var_temp_dn8) / assign44350_e49475)), (assign44350_e49472 * ((-locals.var_temp_dn9) / assign44350_e49475)), );
            locals.var_temp1_rv = 0.0;
        }

        if ((locals.var_guard1359 != 0.0) && (locals.var_guard1360 == 0.0)) {
            let assign44360_e49486: f64 = (locals.var_temp * locals.var_temp);
            let assign44360_e49490: f64 = (2.0 * locals.var_temp);
            let assign44360_e49492: f64 = (assign44360_e49490 / locals.var_gfsub);
            let assign44360_e49493: f64 = (1.0 + assign44360_e49492);
            let assign44360_e49494: f64 = (assign44360_e49486 / assign44360_e49493);
            (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9, ) = (assign44360_e49494, (((((locals.var_temp_dn4 * locals.var_temp) + (locals.var_temp * locals.var_temp_dn4)) * assign44360_e49493) - (assign44360_e49486 * ((((2.0 * locals.var_temp_dn4) * locals.var_gfsub) - (assign44360_e49490 * locals.var_gfsub_dn4)) / (locals.var_gfsub * locals.var_gfsub)))) / (assign44360_e49493 * assign44360_e49493)), (((((locals.var_temp_dn6 * locals.var_temp) + (locals.var_temp * locals.var_temp_dn6)) * assign44360_e49493) - (assign44360_e49486 * ((((2.0 * locals.var_temp_dn6) * locals.var_gfsub) - (assign44360_e49490 * locals.var_gfsub_dn6)) / (locals.var_gfsub * locals.var_gfsub)))) / (assign44360_e49493 * assign44360_e49493)), (((((locals.var_temp_dn7 * locals.var_temp) + (locals.var_temp * locals.var_temp_dn7)) * assign44360_e49493) - (assign44360_e49486 * ((((2.0 * locals.var_temp_dn7) * locals.var_gfsub) - (assign44360_e49490 * locals.var_gfsub_dn7)) / (locals.var_gfsub * locals.var_gfsub)))) / (assign44360_e49493 * assign44360_e49493)), (((((locals.var_temp_dn8 * locals.var_temp) + (locals.var_temp * locals.var_temp_dn8)) * assign44360_e49493) - (assign44360_e49486 * ((((2.0 * locals.var_temp_dn8) * locals.var_gfsub) - (assign44360_e49490 * locals.var_gfsub_dn8)) / (locals.var_gfsub * locals.var_gfsub)))) / (assign44360_e49493 * assign44360_e49493)), (((((locals.var_temp_dn9 * locals.var_temp) + (locals.var_temp * locals.var_temp_dn9)) * assign44360_e49493) - (assign44360_e49486 * ((((2.0 * locals.var_temp_dn9) * locals.var_gfsub) - (assign44360_e49490 * locals.var_gfsub_dn9)) / (locals.var_gfsub * locals.var_gfsub)))) / (assign44360_e49493 * assign44360_e49493)), );
            locals.var_temp1_rv = 0.0;
        }

        if (locals.var_guard1359 != 0.0) {
            let assign44370_e49501: f64 = (p.p14 * locals.var_typesub_i);
            let assign44370_e49503: f64 = (assign44370_e49501 * locals.var_temp1);
            let assign44370_e49504: f64 = (locals.var_xg20_op + assign44370_e49503);
            (locals.var_xg2eff_op, locals.var_xg2eff_op_dn4, locals.var_xg2eff_op_dn6, locals.var_xg2eff_op_dn7, locals.var_xg2eff_op_dn8, locals.var_xg2eff_op_dn9, ) = (assign44370_e49504, (locals.var_xg20_op_dn4 + (assign44370_e49501 * locals.var_temp1_dn4)), (locals.var_xg20_op_dn6 + (assign44370_e49501 * locals.var_temp1_dn6)), (locals.var_xg20_op_dn7 + (assign44370_e49501 * locals.var_temp1_dn7)), (locals.var_xg20_op_dn8 + (assign44370_e49501 * locals.var_temp1_dn8)), (locals.var_xg20_op_dn9 + (assign44370_e49501 * locals.var_temp1_dn9)), );
            locals.var_xg2eff_op_rv = 0.0;
        }

        if (locals.var_guard1359 == 0.0) {
            (locals.var_xg2eff_op, locals.var_xg2eff_op_dn4, locals.var_xg2eff_op_dn6, locals.var_xg2eff_op_dn7, locals.var_xg2eff_op_dn8, locals.var_xg2eff_op_dn9, ) = (locals.var_xg20_op, locals.var_xg20_op_dn4, locals.var_xg20_op_dn6, locals.var_xg20_op_dn7, locals.var_xg20_op_dn8, locals.var_xg20_op_dn9, );
            locals.var_xg2eff_op_rv = 0.0;
        }

        let assign44390_e49515: f64 = (locals.var_xg10_op - locals.var_xg2eff_op);
        let assign44390_e49516: f64 = (locals.var_keq_1d * assign44390_e49515);
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign44390_e49516, (locals.var_keq_1d * (locals.var_xg10_op_dn4 - locals.var_xg2eff_op_dn4)), (locals.var_keq_1d * (locals.var_xg10_op_dn6 - locals.var_xg2eff_op_dn6)), (locals.var_keq_1d * (locals.var_xg10_op_dn7 - locals.var_xg2eff_op_dn7)), (locals.var_keq_1d * (locals.var_xg10_op_dn8 - locals.var_xg2eff_op_dn8)), (locals.var_keq_1d * (locals.var_xg10_op_dn9 - locals.var_xg2eff_op_dn9)), );
        locals.var_temp_rv = 0.0;

        let assign44400_e49519: f64 = if p.p13 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1361 = assign44400_e49519;
        locals.var_guard1361_rv = 0.0;

        if (locals.var_guard1361 != 0.0) {
            let assign44410_e49524: f64 = (locals.var_temp + locals.var_emin);
            let assign44410_e49527: f64 = (locals.var_temp - locals.var_emin);
            let assign44410_e49530: f64 = (locals.var_temp - locals.var_emin);
            let assign44410_e49531: f64 = (assign44410_e49527 * assign44410_e49530);
            let assign44410_e49534: f64 = (locals.var_emin * locals.var_emin);
            let assign44410_e49535: f64 = (assign44410_e49531 + assign44410_e49534);
            let assign44410_e49536: f64 = (assign44410_e49535).sqrt();
            let assign44410_e49537: f64 = (assign44410_e49524 + assign44410_e49536);
            let assign44410_e49538: f64 = (0.5 * assign44410_e49537);
            (locals.var_e1_op, locals.var_e1_op_dn4, locals.var_e1_op_dn6, locals.var_e1_op_dn7, locals.var_e1_op_dn8, locals.var_e1_op_dn9, ) = (assign44410_e49538, (0.5 * ((locals.var_temp_dn4 + locals.var_emin_dn4) + (((((locals.var_temp_dn4 - locals.var_emin_dn4) * assign44410_e49530) + (assign44410_e49527 * (locals.var_temp_dn4 - locals.var_emin_dn4))) + ((locals.var_emin_dn4 * locals.var_emin) + (locals.var_emin * locals.var_emin_dn4))) / (2.0 * assign44410_e49536)))), (0.5 * ((locals.var_temp_dn6 + locals.var_emin_dn6) + (((((locals.var_temp_dn6 - locals.var_emin_dn6) * assign44410_e49530) + (assign44410_e49527 * (locals.var_temp_dn6 - locals.var_emin_dn6))) + ((locals.var_emin_dn6 * locals.var_emin) + (locals.var_emin * locals.var_emin_dn6))) / (2.0 * assign44410_e49536)))), (0.5 * ((locals.var_temp_dn7 + locals.var_emin_dn7) + (((((locals.var_temp_dn7 - locals.var_emin_dn7) * assign44410_e49530) + (assign44410_e49527 * (locals.var_temp_dn7 - locals.var_emin_dn7))) + ((locals.var_emin_dn7 * locals.var_emin) + (locals.var_emin * locals.var_emin_dn7))) / (2.0 * assign44410_e49536)))), (0.5 * ((locals.var_temp_dn8 + locals.var_emin_dn8) + (((((locals.var_temp_dn8 - locals.var_emin_dn8) * assign44410_e49530) + (assign44410_e49527 * (locals.var_temp_dn8 - locals.var_emin_dn8))) + ((locals.var_emin_dn8 * locals.var_emin) + (locals.var_emin * locals.var_emin_dn8))) / (2.0 * assign44410_e49536)))), (0.5 * ((locals.var_temp_dn9 + locals.var_emin_dn9) + (((((locals.var_temp_dn9 - locals.var_emin_dn9) * assign44410_e49530) + (assign44410_e49527 * (locals.var_temp_dn9 - locals.var_emin_dn9))) + ((locals.var_emin_dn9 * locals.var_emin) + (locals.var_emin * locals.var_emin_dn9))) / (2.0 * assign44410_e49536)))), );
            locals.var_e1_op_rv = 0.0;
        }

        if (locals.var_guard1361 != 0.0) {
            let assign44420_e49544: f64 = (-locals.var_temp);
            let assign44420_e49546: f64 = (assign44420_e49544 + locals.var_emin);
            let assign44420_e49548: f64 = (-locals.var_temp);
            let assign44420_e49550: f64 = (assign44420_e49548 - locals.var_emin);
            let assign44420_e49552: f64 = (-locals.var_temp);
            let assign44420_e49554: f64 = (assign44420_e49552 - locals.var_emin);
            let assign44420_e49555: f64 = (assign44420_e49550 * assign44420_e49554);
            let assign44420_e49558: f64 = (locals.var_emin * locals.var_emin);
            let assign44420_e49559: f64 = (assign44420_e49555 + assign44420_e49558);
            let assign44420_e49560: f64 = (assign44420_e49559).sqrt();
            let assign44420_e49561: f64 = (assign44420_e49546 + assign44420_e49560);
            let assign44420_e49562: f64 = (0.5 * assign44420_e49561);
            (locals.var_e2_op, locals.var_e2_op_dn4, locals.var_e2_op_dn6, locals.var_e2_op_dn7, locals.var_e2_op_dn8, locals.var_e2_op_dn9, ) = (assign44420_e49562, (0.5 * (((-locals.var_temp_dn4) + locals.var_emin_dn4) + ((((((-locals.var_temp_dn4) - locals.var_emin_dn4) * assign44420_e49554) + (assign44420_e49550 * ((-locals.var_temp_dn4) - locals.var_emin_dn4))) + ((locals.var_emin_dn4 * locals.var_emin) + (locals.var_emin * locals.var_emin_dn4))) / (2.0 * assign44420_e49560)))), (0.5 * (((-locals.var_temp_dn6) + locals.var_emin_dn6) + ((((((-locals.var_temp_dn6) - locals.var_emin_dn6) * assign44420_e49554) + (assign44420_e49550 * ((-locals.var_temp_dn6) - locals.var_emin_dn6))) + ((locals.var_emin_dn6 * locals.var_emin) + (locals.var_emin * locals.var_emin_dn6))) / (2.0 * assign44420_e49560)))), (0.5 * (((-locals.var_temp_dn7) + locals.var_emin_dn7) + ((((((-locals.var_temp_dn7) - locals.var_emin_dn7) * assign44420_e49554) + (assign44420_e49550 * ((-locals.var_temp_dn7) - locals.var_emin_dn7))) + ((locals.var_emin_dn7 * locals.var_emin) + (locals.var_emin * locals.var_emin_dn7))) / (2.0 * assign44420_e49560)))), (0.5 * (((-locals.var_temp_dn8) + locals.var_emin_dn8) + ((((((-locals.var_temp_dn8) - locals.var_emin_dn8) * assign44420_e49554) + (assign44420_e49550 * ((-locals.var_temp_dn8) - locals.var_emin_dn8))) + ((locals.var_emin_dn8 * locals.var_emin) + (locals.var_emin * locals.var_emin_dn8))) / (2.0 * assign44420_e49560)))), (0.5 * (((-locals.var_temp_dn9) + locals.var_emin_dn9) + ((((((-locals.var_temp_dn9) - locals.var_emin_dn9) * assign44420_e49554) + (assign44420_e49550 * ((-locals.var_temp_dn9) - locals.var_emin_dn9))) + ((locals.var_emin_dn9 * locals.var_emin) + (locals.var_emin * locals.var_emin_dn9))) / (2.0 * assign44420_e49560)))), );
            locals.var_e2_op_rv = 0.0;
        }

        if (locals.var_guard1361 != 0.0) {
            let assign44430_e49568: f64 = (-0.3333333333333);
            let assign44430_e49570: f64 = (locals.var_e1_op).ln();
            let assign44430_e49571: f64 = (assign44430_e49568 * assign44430_e49570);
            let assign44430_e49572: f64 = (assign44430_e49571).exp();
            let assign44430_e49573: f64 = (locals.var_qq_op * assign44430_e49572);
            (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9, ) = (assign44430_e49573, ((locals.var_qq_op_dn4 * assign44430_e49572) + (locals.var_qq_op * (assign44430_e49572 * (assign44430_e49568 * (locals.var_e1_op_dn4 / locals.var_e1_op))))), ((locals.var_qq_op_dn6 * assign44430_e49572) + (locals.var_qq_op * (assign44430_e49572 * (assign44430_e49568 * (locals.var_e1_op_dn6 / locals.var_e1_op))))), ((locals.var_qq_op_dn7 * assign44430_e49572) + (locals.var_qq_op * (assign44430_e49572 * (assign44430_e49568 * (locals.var_e1_op_dn7 / locals.var_e1_op))))), ((locals.var_qq_op_dn8 * assign44430_e49572) + (locals.var_qq_op * (assign44430_e49572 * (assign44430_e49568 * (locals.var_e1_op_dn8 / locals.var_e1_op))))), ((locals.var_qq_op_dn9 * assign44430_e49572) + (locals.var_qq_op * (assign44430_e49572 * (assign44430_e49568 * (locals.var_e1_op_dn9 / locals.var_e1_op))))), );
            locals.var_temp1_rv = 0.0;
        }

        if (locals.var_guard1361 != 0.0) {
            let assign44440_e49579: f64 = (-0.3333333333333);
            let assign44440_e49581: f64 = (locals.var_e2_op).ln();
            let assign44440_e49582: f64 = (assign44440_e49579 * assign44440_e49581);
            let assign44440_e49583: f64 = (assign44440_e49582).exp();
            let assign44440_e49584: f64 = (locals.var_qq_op * assign44440_e49583);
            (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9, ) = (assign44440_e49584, ((locals.var_qq_op_dn4 * assign44440_e49583) + (locals.var_qq_op * (assign44440_e49583 * (assign44440_e49579 * (locals.var_e2_op_dn4 / locals.var_e2_op))))), ((locals.var_qq_op_dn6 * assign44440_e49583) + (locals.var_qq_op * (assign44440_e49583 * (assign44440_e49579 * (locals.var_e2_op_dn6 / locals.var_e2_op))))), ((locals.var_qq_op_dn7 * assign44440_e49583) + (locals.var_qq_op * (assign44440_e49583 * (assign44440_e49579 * (locals.var_e2_op_dn7 / locals.var_e2_op))))), ((locals.var_qq_op_dn8 * assign44440_e49583) + (locals.var_qq_op * (assign44440_e49583 * (assign44440_e49579 * (locals.var_e2_op_dn8 / locals.var_e2_op))))), ((locals.var_qq_op_dn9 * assign44440_e49583) + (locals.var_qq_op * (assign44440_e49583 * (assign44440_e49579 * (locals.var_e2_op_dn9 / locals.var_e2_op))))), );
            locals.var_temp2_rv = 0.0;
        }

        if (locals.var_guard1361 != 0.0) {
            let assign44450_e49590: f64 = (1.0 - locals.var_temp1);
            let assign44450_e49592: f64 = (assign44450_e49590 - locals.var_temp2);
            (locals.var_temp3, locals.var_temp3_dn4, locals.var_temp3_dn6, locals.var_temp3_dn7, locals.var_temp3_dn8, locals.var_temp3_dn9, ) = (assign44450_e49592, ((-locals.var_temp1_dn4) - locals.var_temp2_dn4), ((-locals.var_temp1_dn6) - locals.var_temp2_dn6), ((-locals.var_temp1_dn7) - locals.var_temp2_dn7), ((-locals.var_temp1_dn8) - locals.var_temp2_dn8), ((-locals.var_temp1_dn9) - locals.var_temp2_dn9), );
            locals.var_temp3_rv = 0.0;
        }

        if (locals.var_guard1361 != 0.0) {
            let assign44470_e49604: f64 = (locals.var_k1_1d * locals.var_temp3);
            let assign44470_e49608: f64 = (locals.var_k1_1d * locals.var_temp1);
            let assign44470_e49609: f64 = (1.0 + assign44470_e49608);
            let assign44470_e49610: f64 = (assign44470_e49604 / assign44470_e49609);
            (locals.var_k1_1d_qm_op, locals.var_k1_1d_qm_op_dn4, locals.var_k1_1d_qm_op_dn6, locals.var_k1_1d_qm_op_dn7, locals.var_k1_1d_qm_op_dn8, locals.var_k1_1d_qm_op_dn9, ) = (assign44470_e49610, ((((locals.var_k1_1d * locals.var_temp3_dn4) * assign44470_e49609) - (assign44470_e49604 * (locals.var_k1_1d * locals.var_temp1_dn4))) / (assign44470_e49609 * assign44470_e49609)), ((((locals.var_k1_1d * locals.var_temp3_dn6) * assign44470_e49609) - (assign44470_e49604 * (locals.var_k1_1d * locals.var_temp1_dn6))) / (assign44470_e49609 * assign44470_e49609)), ((((locals.var_k1_1d * locals.var_temp3_dn7) * assign44470_e49609) - (assign44470_e49604 * (locals.var_k1_1d * locals.var_temp1_dn7))) / (assign44470_e49609 * assign44470_e49609)), ((((locals.var_k1_1d * locals.var_temp3_dn8) * assign44470_e49609) - (assign44470_e49604 * (locals.var_k1_1d * locals.var_temp1_dn8))) / (assign44470_e49609 * assign44470_e49609)), ((((locals.var_k1_1d * locals.var_temp3_dn9) * assign44470_e49609) - (assign44470_e49604 * (locals.var_k1_1d * locals.var_temp1_dn9))) / (assign44470_e49609 * assign44470_e49609)), );
            locals.var_k1_1d_qm_op_rv = 0.0;
        }

        if (locals.var_guard1361 != 0.0) {
            let assign44480_e49616: f64 = (locals.var_k2_1d * locals.var_temp3);
            let assign44480_e49620: f64 = (locals.var_k2_1d * locals.var_temp2);
            let assign44480_e49621: f64 = (1.0 + assign44480_e49620);
            let assign44480_e49622: f64 = (assign44480_e49616 / assign44480_e49621);
            (locals.var_k2_1d_qm_op, locals.var_k2_1d_qm_op_dn4, locals.var_k2_1d_qm_op_dn6, locals.var_k2_1d_qm_op_dn7, locals.var_k2_1d_qm_op_dn8, locals.var_k2_1d_qm_op_dn9, ) = (assign44480_e49622, ((((locals.var_k2_1d * locals.var_temp3_dn4) * assign44480_e49621) - (assign44480_e49616 * (locals.var_k2_1d * locals.var_temp2_dn4))) / (assign44480_e49621 * assign44480_e49621)), ((((locals.var_k2_1d * locals.var_temp3_dn6) * assign44480_e49621) - (assign44480_e49616 * (locals.var_k2_1d * locals.var_temp2_dn6))) / (assign44480_e49621 * assign44480_e49621)), ((((locals.var_k2_1d * locals.var_temp3_dn7) * assign44480_e49621) - (assign44480_e49616 * (locals.var_k2_1d * locals.var_temp2_dn7))) / (assign44480_e49621 * assign44480_e49621)), ((((locals.var_k2_1d * locals.var_temp3_dn8) * assign44480_e49621) - (assign44480_e49616 * (locals.var_k2_1d * locals.var_temp2_dn8))) / (assign44480_e49621 * assign44480_e49621)), ((((locals.var_k2_1d * locals.var_temp3_dn9) * assign44480_e49621) - (assign44480_e49616 * (locals.var_k2_1d * locals.var_temp2_dn9))) / (assign44480_e49621 * assign44480_e49621)), );
            locals.var_k2_1d_qm_op_rv = 0.0;
        }

    }

    pub(super) fn stamp_reactive_block_63(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        if (locals.var_guard1361 != 0.0) {
            let assign44490_e49630: f64 = (1.0 / locals.var_k1_1d_qm_op);
            let assign44490_e49631: f64 = (1.0 + assign44490_e49630);
            let assign44490_e49634: f64 = (1.0 / locals.var_k2_1d_qm_op);
            let assign44490_e49635: f64 = (assign44490_e49631 + assign44490_e49634);
            let assign44490_e49636: f64 = (1.0 / assign44490_e49635);
            (locals.var_keq_1d_qm_op, locals.var_keq_1d_qm_op_dn4, locals.var_keq_1d_qm_op_dn6, locals.var_keq_1d_qm_op_dn7, locals.var_keq_1d_qm_op_dn8, locals.var_keq_1d_qm_op_dn9, ) = (assign44490_e49636, (-(((-(locals.var_k1_1d_qm_op_dn4 / (locals.var_k1_1d_qm_op * locals.var_k1_1d_qm_op))) + (-(locals.var_k2_1d_qm_op_dn4 / (locals.var_k2_1d_qm_op * locals.var_k2_1d_qm_op)))) / (assign44490_e49635 * assign44490_e49635))), (-(((-(locals.var_k1_1d_qm_op_dn6 / (locals.var_k1_1d_qm_op * locals.var_k1_1d_qm_op))) + (-(locals.var_k2_1d_qm_op_dn6 / (locals.var_k2_1d_qm_op * locals.var_k2_1d_qm_op)))) / (assign44490_e49635 * assign44490_e49635))), (-(((-(locals.var_k1_1d_qm_op_dn7 / (locals.var_k1_1d_qm_op * locals.var_k1_1d_qm_op))) + (-(locals.var_k2_1d_qm_op_dn7 / (locals.var_k2_1d_qm_op * locals.var_k2_1d_qm_op)))) / (assign44490_e49635 * assign44490_e49635))), (-(((-(locals.var_k1_1d_qm_op_dn8 / (locals.var_k1_1d_qm_op * locals.var_k1_1d_qm_op))) + (-(locals.var_k2_1d_qm_op_dn8 / (locals.var_k2_1d_qm_op * locals.var_k2_1d_qm_op)))) / (assign44490_e49635 * assign44490_e49635))), (-(((-(locals.var_k1_1d_qm_op_dn9 / (locals.var_k1_1d_qm_op * locals.var_k1_1d_qm_op))) + (-(locals.var_k2_1d_qm_op_dn9 / (locals.var_k2_1d_qm_op * locals.var_k2_1d_qm_op)))) / (assign44490_e49635 * assign44490_e49635))), );
            locals.var_keq_1d_qm_op_rv = 0.0;
        }

        if (locals.var_guard1361 == 0.0) {
            (locals.var_k1_1d_qm_op, locals.var_k1_1d_qm_op_dn4, locals.var_k1_1d_qm_op_dn6, locals.var_k1_1d_qm_op_dn7, locals.var_k1_1d_qm_op_dn8, locals.var_k1_1d_qm_op_dn9, ) = (locals.var_k1_1d, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_k1_1d_qm_op_rv = 0.0;
            (locals.var_k2_1d_qm_op, locals.var_k2_1d_qm_op_dn4, locals.var_k2_1d_qm_op_dn6, locals.var_k2_1d_qm_op_dn7, locals.var_k2_1d_qm_op_dn8, locals.var_k2_1d_qm_op_dn9, ) = (locals.var_k2_1d, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_k2_1d_qm_op_rv = 0.0;
            (locals.var_keq_1d_qm_op, locals.var_keq_1d_qm_op_dn4, locals.var_keq_1d_qm_op_dn6, locals.var_keq_1d_qm_op_dn7, locals.var_keq_1d_qm_op_dn8, locals.var_keq_1d_qm_op_dn9, ) = (locals.var_keq_1d, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_keq_1d_qm_op_rv = 0.0;
        }

        let assign44540_e49662: f64 = (locals.var_xg10_op - locals.var_xg2eff_op);
        let assign44540_e49663: f64 = (locals.var_keq_1d_qm_op * assign44540_e49662);
        (locals.var_dx_wi_1d_op, locals.var_dx_wi_1d_op_dn4, locals.var_dx_wi_1d_op_dn6, locals.var_dx_wi_1d_op_dn7, locals.var_dx_wi_1d_op_dn8, locals.var_dx_wi_1d_op_dn9, ) = (assign44540_e49663, ((locals.var_keq_1d_qm_op_dn4 * assign44540_e49662) + (locals.var_keq_1d_qm_op * (locals.var_xg10_op_dn4 - locals.var_xg2eff_op_dn4))), ((locals.var_keq_1d_qm_op_dn6 * assign44540_e49662) + (locals.var_keq_1d_qm_op * (locals.var_xg10_op_dn6 - locals.var_xg2eff_op_dn6))), ((locals.var_keq_1d_qm_op_dn7 * assign44540_e49662) + (locals.var_keq_1d_qm_op * (locals.var_xg10_op_dn7 - locals.var_xg2eff_op_dn7))), ((locals.var_keq_1d_qm_op_dn8 * assign44540_e49662) + (locals.var_keq_1d_qm_op * (locals.var_xg10_op_dn8 - locals.var_xg2eff_op_dn8))), ((locals.var_keq_1d_qm_op_dn9 * assign44540_e49662) + (locals.var_keq_1d_qm_op * (locals.var_xg10_op_dn9 - locals.var_xg2eff_op_dn9))), );
        locals.var_dx_wi_1d_op_rv = 0.0;

        let assign44550_e49666: f64 = if locals.var_dx_wi_1d_op > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1362 = assign44550_e49666;
        locals.var_guard1362_rv = 0.0;

        let assign44560_e49668: f64 = (-locals.var_dx_wi_1d_op);
        let assign44560_e49670: f64 = if assign44560_e49668 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard1363 = assign44560_e49670;
        locals.var_guard1363_rv = 0.0;

        if ((locals.var_guard1362 != 0.0) && (locals.var_guard1363 != 0.0)) {
            let assign44570_e49676: f64 = (-locals.var_dx_wi_1d_op);
            let assign44570_e49677: f64 = (assign44570_e49676).exp();
            let assign44570_e49678: f64 = (1.0 + assign44570_e49677);
            let assign44570_e49679: f64 = (assign44570_e49678).ln();
            (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign44570_e49679, ((assign44570_e49677 * (-locals.var_dx_wi_1d_op_dn4)) / assign44570_e49678), ((assign44570_e49677 * (-locals.var_dx_wi_1d_op_dn6)) / assign44570_e49678), ((assign44570_e49677 * (-locals.var_dx_wi_1d_op_dn7)) / assign44570_e49678), ((assign44570_e49677 * (-locals.var_dx_wi_1d_op_dn8)) / assign44570_e49678), ((assign44570_e49677 * (-locals.var_dx_wi_1d_op_dn9)) / assign44570_e49678), );
            locals.var_temp_rv = 0.0;
        }

        if ((locals.var_guard1362 != 0.0) && (locals.var_guard1363 == 0.0)) {
            let assign44580_e49687: f64 = (-locals.var_dx_wi_1d_op);
            (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign44580_e49687, (-locals.var_dx_wi_1d_op_dn4), (-locals.var_dx_wi_1d_op_dn6), (-locals.var_dx_wi_1d_op_dn7), (-locals.var_dx_wi_1d_op_dn8), (-locals.var_dx_wi_1d_op_dn9), );
            locals.var_temp_rv = 0.0;
        }

        if (locals.var_guard1362 != 0.0) {
            let assign44590_e49694: f64 = (locals.var_dx_wi_1d_op / locals.var_k1_1d_qm_op);
            let assign44590_e49695: f64 = (locals.var_xg10_op - assign44590_e49694);
            let assign44590_e49697: f64 = (assign44590_e49695 + locals.var_temp);
            let assign44590_e49699: f64 = (assign44590_e49697 - 0.6931471805599);
            (locals.var_x_wi_1d_op, locals.var_x_wi_1d_op_dn4, locals.var_x_wi_1d_op_dn6, locals.var_x_wi_1d_op_dn7, locals.var_x_wi_1d_op_dn8, locals.var_x_wi_1d_op_dn9, ) = (assign44590_e49699, ((locals.var_xg10_op_dn4 - (((locals.var_dx_wi_1d_op_dn4 * locals.var_k1_1d_qm_op) - (locals.var_dx_wi_1d_op * locals.var_k1_1d_qm_op_dn4)) / (locals.var_k1_1d_qm_op * locals.var_k1_1d_qm_op))) + locals.var_temp_dn4), ((locals.var_xg10_op_dn6 - (((locals.var_dx_wi_1d_op_dn6 * locals.var_k1_1d_qm_op) - (locals.var_dx_wi_1d_op * locals.var_k1_1d_qm_op_dn6)) / (locals.var_k1_1d_qm_op * locals.var_k1_1d_qm_op))) + locals.var_temp_dn6), ((locals.var_xg10_op_dn7 - (((locals.var_dx_wi_1d_op_dn7 * locals.var_k1_1d_qm_op) - (locals.var_dx_wi_1d_op * locals.var_k1_1d_qm_op_dn7)) / (locals.var_k1_1d_qm_op * locals.var_k1_1d_qm_op))) + locals.var_temp_dn7), ((locals.var_xg10_op_dn8 - (((locals.var_dx_wi_1d_op_dn8 * locals.var_k1_1d_qm_op) - (locals.var_dx_wi_1d_op * locals.var_k1_1d_qm_op_dn8)) / (locals.var_k1_1d_qm_op * locals.var_k1_1d_qm_op))) + locals.var_temp_dn8), ((locals.var_xg10_op_dn9 - (((locals.var_dx_wi_1d_op_dn9 * locals.var_k1_1d_qm_op) - (locals.var_dx_wi_1d_op * locals.var_k1_1d_qm_op_dn9)) / (locals.var_k1_1d_qm_op * locals.var_k1_1d_qm_op))) + locals.var_temp_dn9), );
            locals.var_x_wi_1d_op_rv = 0.0;
        }

        let assign44600_e49704: f64 = if locals.var_dx_wi_1d_op < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard1364 = assign44600_e49704;
        locals.var_guard1364_rv = 0.0;

        if ((locals.var_guard1362 == 0.0) && (locals.var_guard1364 != 0.0)) {
            let assign44610_e49711: f64 = (locals.var_dx_wi_1d_op).exp();
            let assign44610_e49712: f64 = (1.0 + assign44610_e49711);
            let assign44610_e49713: f64 = (assign44610_e49712).ln();
            (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign44610_e49713, ((assign44610_e49711 * locals.var_dx_wi_1d_op_dn4) / assign44610_e49712), ((assign44610_e49711 * locals.var_dx_wi_1d_op_dn6) / assign44610_e49712), ((assign44610_e49711 * locals.var_dx_wi_1d_op_dn7) / assign44610_e49712), ((assign44610_e49711 * locals.var_dx_wi_1d_op_dn8) / assign44610_e49712), ((assign44610_e49711 * locals.var_dx_wi_1d_op_dn9) / assign44610_e49712), );
            locals.var_temp_rv = 0.0;
        }

        if ((locals.var_guard1362 == 0.0) && (locals.var_guard1364 == 0.0)) {
            (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (locals.var_dx_wi_1d_op, locals.var_dx_wi_1d_op_dn4, locals.var_dx_wi_1d_op_dn6, locals.var_dx_wi_1d_op_dn7, locals.var_dx_wi_1d_op_dn8, locals.var_dx_wi_1d_op_dn9, );
            locals.var_temp_rv = 0.0;
        }

        if (locals.var_guard1362 == 0.0) {
            let assign44630_e49729: f64 = (locals.var_dx_wi_1d_op / locals.var_k2_1d_qm_op);
            let assign44630_e49730: f64 = (locals.var_xg2eff_op + assign44630_e49729);
            let assign44630_e49732: f64 = (assign44630_e49730 + locals.var_temp);
            let assign44630_e49734: f64 = (assign44630_e49732 - 0.6931471805599);
            (locals.var_x_wi_1d_op, locals.var_x_wi_1d_op_dn4, locals.var_x_wi_1d_op_dn6, locals.var_x_wi_1d_op_dn7, locals.var_x_wi_1d_op_dn8, locals.var_x_wi_1d_op_dn9, ) = (assign44630_e49734, ((locals.var_xg2eff_op_dn4 + (((locals.var_dx_wi_1d_op_dn4 * locals.var_k2_1d_qm_op) - (locals.var_dx_wi_1d_op * locals.var_k2_1d_qm_op_dn4)) / (locals.var_k2_1d_qm_op * locals.var_k2_1d_qm_op))) + locals.var_temp_dn4), ((locals.var_xg2eff_op_dn6 + (((locals.var_dx_wi_1d_op_dn6 * locals.var_k2_1d_qm_op) - (locals.var_dx_wi_1d_op * locals.var_k2_1d_qm_op_dn6)) / (locals.var_k2_1d_qm_op * locals.var_k2_1d_qm_op))) + locals.var_temp_dn6), ((locals.var_xg2eff_op_dn7 + (((locals.var_dx_wi_1d_op_dn7 * locals.var_k2_1d_qm_op) - (locals.var_dx_wi_1d_op * locals.var_k2_1d_qm_op_dn7)) / (locals.var_k2_1d_qm_op * locals.var_k2_1d_qm_op))) + locals.var_temp_dn7), ((locals.var_xg2eff_op_dn8 + (((locals.var_dx_wi_1d_op_dn8 * locals.var_k2_1d_qm_op) - (locals.var_dx_wi_1d_op * locals.var_k2_1d_qm_op_dn8)) / (locals.var_k2_1d_qm_op * locals.var_k2_1d_qm_op))) + locals.var_temp_dn8), ((locals.var_xg2eff_op_dn9 + (((locals.var_dx_wi_1d_op_dn9 * locals.var_k2_1d_qm_op) - (locals.var_dx_wi_1d_op * locals.var_k2_1d_qm_op_dn9)) / (locals.var_k2_1d_qm_op * locals.var_k2_1d_qm_op))) + locals.var_temp_dn9), );
            locals.var_x_wi_1d_op_rv = 0.0;
        }

        let assign44640_e49740: f64 = (locals.var_x_wi_1d_op + locals.var_xth_1d_op);
        let assign44640_e49743: f64 = (locals.var_x_wi_1d_op - locals.var_xth_1d_op);
        let assign44640_e49746: f64 = (locals.var_x_wi_1d_op - locals.var_xth_1d_op);
        let assign44640_e49747: f64 = (assign44640_e49743 * assign44640_e49746);
        let assign44640_e49749: f64 = (assign44640_e49747 + 4.0);
        let assign44640_e49750: f64 = (assign44640_e49749).sqrt();
        let assign44640_e49751: f64 = (assign44640_e49740 - assign44640_e49750);
        let assign44640_e49752: f64 = (0.5 * assign44640_e49751);
        (locals.var_x_1d_op, locals.var_x_1d_op_dn4, locals.var_x_1d_op_dn6, locals.var_x_1d_op_dn7, locals.var_x_1d_op_dn8, locals.var_x_1d_op_dn9, ) = (assign44640_e49752, (0.5 * ((locals.var_x_wi_1d_op_dn4 + locals.var_xth_1d_op_dn4) - ((((locals.var_x_wi_1d_op_dn4 - locals.var_xth_1d_op_dn4) * assign44640_e49746) + (assign44640_e49743 * (locals.var_x_wi_1d_op_dn4 - locals.var_xth_1d_op_dn4))) / (2.0 * assign44640_e49750)))), (0.5 * ((locals.var_x_wi_1d_op_dn6 + locals.var_xth_1d_op_dn6) - ((((locals.var_x_wi_1d_op_dn6 - locals.var_xth_1d_op_dn6) * assign44640_e49746) + (assign44640_e49743 * (locals.var_x_wi_1d_op_dn6 - locals.var_xth_1d_op_dn6))) / (2.0 * assign44640_e49750)))), (0.5 * ((locals.var_x_wi_1d_op_dn7 + locals.var_xth_1d_op_dn7) - ((((locals.var_x_wi_1d_op_dn7 - locals.var_xth_1d_op_dn7) * assign44640_e49746) + (assign44640_e49743 * (locals.var_x_wi_1d_op_dn7 - locals.var_xth_1d_op_dn7))) / (2.0 * assign44640_e49750)))), (0.5 * ((locals.var_x_wi_1d_op_dn8 + locals.var_xth_1d_op_dn8) - ((((locals.var_x_wi_1d_op_dn8 - locals.var_xth_1d_op_dn8) * assign44640_e49746) + (assign44640_e49743 * (locals.var_x_wi_1d_op_dn8 - locals.var_xth_1d_op_dn8))) / (2.0 * assign44640_e49750)))), (0.5 * ((locals.var_x_wi_1d_op_dn9 + locals.var_xth_1d_op_dn9) - ((((locals.var_x_wi_1d_op_dn9 - locals.var_xth_1d_op_dn9) * assign44640_e49746) + (assign44640_e49743 * (locals.var_x_wi_1d_op_dn9 - locals.var_xth_1d_op_dn9))) / (2.0 * assign44640_e49750)))), );
        locals.var_x_1d_op_rv = 0.0;

        let assign44650_e49757: f64 = (locals.var_xth_1d_op - locals.var_x_1d_op);
        let assign44650_e49758: f64 = (2.0 * assign44650_e49757);
        let assign44650_e49760: f64 = (assign44650_e49758 / locals.var_xsddep_op);
        let assign44650_e49761: f64 = (1.0 + assign44650_e49760);
        let assign44650_e49762: f64 = (assign44650_e49761).sqrt();
        let assign44650_e49764: f64 = (assign44650_e49762 - 1.0);
        (locals.var_dleff_op, locals.var_dleff_op_dn4, locals.var_dleff_op_dn6, locals.var_dleff_op_dn7, locals.var_dleff_op_dn8, locals.var_dleff_op_dn9, ) = (assign44650_e49764, (((((2.0 * (locals.var_xth_1d_op_dn4 - locals.var_x_1d_op_dn4)) * locals.var_xsddep_op) - (assign44650_e49758 * locals.var_xsddep_op_dn4)) / (locals.var_xsddep_op * locals.var_xsddep_op)) / (2.0 * assign44650_e49762)), (((((2.0 * (locals.var_xth_1d_op_dn6 - locals.var_x_1d_op_dn6)) * locals.var_xsddep_op) - (assign44650_e49758 * locals.var_xsddep_op_dn6)) / (locals.var_xsddep_op * locals.var_xsddep_op)) / (2.0 * assign44650_e49762)), (((((2.0 * (locals.var_xth_1d_op_dn7 - locals.var_x_1d_op_dn7)) * locals.var_xsddep_op) - (assign44650_e49758 * locals.var_xsddep_op_dn7)) / (locals.var_xsddep_op * locals.var_xsddep_op)) / (2.0 * assign44650_e49762)), (((((2.0 * (locals.var_xth_1d_op_dn8 - locals.var_x_1d_op_dn8)) * locals.var_xsddep_op) - (assign44650_e49758 * locals.var_xsddep_op_dn8)) / (locals.var_xsddep_op * locals.var_xsddep_op)) / (2.0 * assign44650_e49762)), (((((2.0 * (locals.var_xth_1d_op_dn9 - locals.var_x_1d_op_dn9)) * locals.var_xsddep_op) - (assign44650_e49758 * locals.var_xsddep_op_dn9)) / (locals.var_xsddep_op * locals.var_xsddep_op)) / (2.0 * assign44650_e49762)), );
        locals.var_dleff_op_rv = 0.0;

        let assign44670_e49774: f64 = (locals.var_pscedlb_i * locals.var_xg20_op);
        let assign44670_e49775: f64 = (1.0 + assign44670_e49774);
        let assign44670_e49777: f64 = (assign44670_e49775 + 0.5);
        let assign44670_e49781: f64 = (locals.var_pscedlb_i * locals.var_xg20_op);
        let assign44670_e49782: f64 = (1.0 + assign44670_e49781);
        let assign44670_e49784: f64 = (assign44670_e49782 - 0.5);
        let assign44670_e49788: f64 = (locals.var_pscedlb_i * locals.var_xg20_op);
        let assign44670_e49789: f64 = (1.0 + assign44670_e49788);
        let assign44670_e49791: f64 = (assign44670_e49789 - 0.5);
        let assign44670_e49792: f64 = (assign44670_e49784 * assign44670_e49791);
        let assign44670_e49794: f64 = (assign44670_e49792 + 0.01);
        let assign44670_e49795: f64 = (assign44670_e49794).sqrt();
        let assign44670_e49796: f64 = (assign44670_e49777 + assign44670_e49795);
        let assign44670_e49797: f64 = (0.5 * assign44670_e49796);
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign44670_e49797, (0.5 * ((locals.var_pscedlb_i * locals.var_xg20_op_dn4) + ((((locals.var_pscedlb_i * locals.var_xg20_op_dn4) * assign44670_e49791) + (assign44670_e49784 * (locals.var_pscedlb_i * locals.var_xg20_op_dn4))) / (2.0 * assign44670_e49795)))), (0.5 * ((locals.var_pscedlb_i * locals.var_xg20_op_dn6) + ((((locals.var_pscedlb_i * locals.var_xg20_op_dn6) * assign44670_e49791) + (assign44670_e49784 * (locals.var_pscedlb_i * locals.var_xg20_op_dn6))) / (2.0 * assign44670_e49795)))), (0.5 * ((locals.var_pscedlb_i * locals.var_xg20_op_dn7) + ((((locals.var_pscedlb_i * locals.var_xg20_op_dn7) * assign44670_e49791) + (assign44670_e49784 * (locals.var_pscedlb_i * locals.var_xg20_op_dn7))) / (2.0 * assign44670_e49795)))), (0.5 * ((locals.var_pscedlb_i * locals.var_xg20_op_dn8) + ((((locals.var_pscedlb_i * locals.var_xg20_op_dn8) * assign44670_e49791) + (assign44670_e49784 * (locals.var_pscedlb_i * locals.var_xg20_op_dn8))) / (2.0 * assign44670_e49795)))), (0.5 * ((locals.var_pscedlb_i * locals.var_xg20_op_dn9) + ((((locals.var_pscedlb_i * locals.var_xg20_op_dn9) * assign44670_e49791) + (assign44670_e49784 * (locals.var_pscedlb_i * locals.var_xg20_op_dn9))) / (2.0 * assign44670_e49795)))), );
        locals.var_temp_rv = 0.0;

        let assign44700_e49814: f64 = (2.0 * locals.var_xd0_op);
        let assign44700_e49818: f64 = (locals.var_xdsx_op / locals.var_xd0_op);
        let assign44700_e49819: f64 = (1.0 + assign44700_e49818);
        let assign44700_e49820: f64 = (assign44700_e49819).sqrt();
        let assign44700_e49822: f64 = (assign44700_e49820 - 1.0);
        let assign44700_e49823: f64 = (assign44700_e49814 * assign44700_e49822);
        let assign44700_e49827: f64 = (locals.var_cfdl_i * locals.var_dleff_op);
        let assign44700_e49828: f64 = (1.0 + assign44700_e49827);
        let assign44700_e49829: f64 = (assign44700_e49823 * assign44700_e49828);
        let assign44700_e49833: f64 = (locals.var_cfdlb_i * locals.var_xg20_op);
        let assign44700_e49834: f64 = (1.0 + assign44700_e49833);
        let assign44700_e49835: f64 = (assign44700_e49829 * assign44700_e49834);
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign44700_e49835, (((((((2.0 * locals.var_xd0_op_dn4) * assign44700_e49822) + (assign44700_e49814 * ((((locals.var_xdsx_op_dn4 * locals.var_xd0_op) - (locals.var_xdsx_op * locals.var_xd0_op_dn4)) / (locals.var_xd0_op * locals.var_xd0_op)) / (2.0 * assign44700_e49820)))) * assign44700_e49828) + (assign44700_e49823 * (locals.var_cfdl_i * locals.var_dleff_op_dn4))) * assign44700_e49834) + (assign44700_e49829 * (locals.var_cfdlb_i * locals.var_xg20_op_dn4))), (((((((2.0 * locals.var_xd0_op_dn6) * assign44700_e49822) + (assign44700_e49814 * ((((locals.var_xdsx_op_dn6 * locals.var_xd0_op) - (locals.var_xdsx_op * locals.var_xd0_op_dn6)) / (locals.var_xd0_op * locals.var_xd0_op)) / (2.0 * assign44700_e49820)))) * assign44700_e49828) + (assign44700_e49823 * (locals.var_cfdl_i * locals.var_dleff_op_dn6))) * assign44700_e49834) + (assign44700_e49829 * (locals.var_cfdlb_i * locals.var_xg20_op_dn6))), (((((((2.0 * locals.var_xd0_op_dn7) * assign44700_e49822) + (assign44700_e49814 * ((((locals.var_xdsx_op_dn7 * locals.var_xd0_op) - (locals.var_xdsx_op * locals.var_xd0_op_dn7)) / (locals.var_xd0_op * locals.var_xd0_op)) / (2.0 * assign44700_e49820)))) * assign44700_e49828) + (assign44700_e49823 * (locals.var_cfdl_i * locals.var_dleff_op_dn7))) * assign44700_e49834) + (assign44700_e49829 * (locals.var_cfdlb_i * locals.var_xg20_op_dn7))), (((((((2.0 * locals.var_xd0_op_dn8) * assign44700_e49822) + (assign44700_e49814 * ((((locals.var_xdsx_op_dn8 * locals.var_xd0_op) - (locals.var_xdsx_op * locals.var_xd0_op_dn8)) / (locals.var_xd0_op * locals.var_xd0_op)) / (2.0 * assign44700_e49820)))) * assign44700_e49828) + (assign44700_e49823 * (locals.var_cfdl_i * locals.var_dleff_op_dn8))) * assign44700_e49834) + (assign44700_e49829 * (locals.var_cfdlb_i * locals.var_xg20_op_dn8))), (((((((2.0 * locals.var_xd0_op_dn9) * assign44700_e49822) + (assign44700_e49814 * ((((locals.var_xdsx_op_dn9 * locals.var_xd0_op) - (locals.var_xdsx_op * locals.var_xd0_op_dn9)) / (locals.var_xd0_op * locals.var_xd0_op)) / (2.0 * assign44700_e49820)))) * assign44700_e49828) + (assign44700_e49823 * (locals.var_cfdl_i * locals.var_dleff_op_dn9))) * assign44700_e49834) + (assign44700_e49829 * (locals.var_cfdlb_i * locals.var_xg20_op_dn9))), );
        locals.var_temp_rv = 0.0;

        let assign44940_e50018: f64 = if p.p11 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1366 = assign44940_e50018;
        locals.var_guard1366_rv = 0.0;

        if (locals.var_guard1366 != 0.0) {
            let assign44950_e50022: f64 = (locals.var_k2_ac / locals.var_k1_ac);
            let assign44950_e50025: f64 = (1.0 + locals.var_k2_ac);
            let assign44950_e50026: f64 = (assign44950_e50022 / assign44950_e50025);
            (locals.var_r1init_op, locals.var_r1init_op_dn4, locals.var_r1init_op_dn6, locals.var_r1init_op_dn7, locals.var_r1init_op_dn8, locals.var_r1init_op_dn9, ) = (assign44950_e50026, ((((((locals.var_k2_ac_dn4 * locals.var_k1_ac) - (locals.var_k2_ac * locals.var_k1_ac_dn4)) / (locals.var_k1_ac * locals.var_k1_ac)) * assign44950_e50025) - (assign44950_e50022 * locals.var_k2_ac_dn4)) / (assign44950_e50025 * assign44950_e50025)), ((((((locals.var_k2_ac_dn6 * locals.var_k1_ac) - (locals.var_k2_ac * locals.var_k1_ac_dn6)) / (locals.var_k1_ac * locals.var_k1_ac)) * assign44950_e50025) - (assign44950_e50022 * locals.var_k2_ac_dn6)) / (assign44950_e50025 * assign44950_e50025)), ((((((locals.var_k2_ac_dn7 * locals.var_k1_ac) - (locals.var_k2_ac * locals.var_k1_ac_dn7)) / (locals.var_k1_ac * locals.var_k1_ac)) * assign44950_e50025) - (assign44950_e50022 * locals.var_k2_ac_dn7)) / (assign44950_e50025 * assign44950_e50025)), ((((((locals.var_k2_ac_dn8 * locals.var_k1_ac) - (locals.var_k2_ac * locals.var_k1_ac_dn8)) / (locals.var_k1_ac * locals.var_k1_ac)) * assign44950_e50025) - (assign44950_e50022 * locals.var_k2_ac_dn8)) / (assign44950_e50025 * assign44950_e50025)), ((((((locals.var_k2_ac_dn9 * locals.var_k1_ac) - (locals.var_k2_ac * locals.var_k1_ac_dn9)) / (locals.var_k1_ac * locals.var_k1_ac)) * assign44950_e50025) - (assign44950_e50022 * locals.var_k2_ac_dn9)) / (assign44950_e50025 * assign44950_e50025)), );
            locals.var_r1init_op_rv = 0.0;
        }

        if (locals.var_guard1366 != 0.0) {
            let assign44960_e50032: f64 = (locals.var_k1_ac / locals.var_k2_ac);
            let assign44960_e50035: f64 = (1.0 + locals.var_k1_ac);
            let assign44960_e50036: f64 = (assign44960_e50032 / assign44960_e50035);
            (locals.var_r2init_op, locals.var_r2init_op_dn4, locals.var_r2init_op_dn6, locals.var_r2init_op_dn7, locals.var_r2init_op_dn8, locals.var_r2init_op_dn9, ) = (assign44960_e50036, ((((((locals.var_k1_ac_dn4 * locals.var_k2_ac) - (locals.var_k1_ac * locals.var_k2_ac_dn4)) / (locals.var_k2_ac * locals.var_k2_ac)) * assign44960_e50035) - (assign44960_e50032 * locals.var_k1_ac_dn4)) / (assign44960_e50035 * assign44960_e50035)), ((((((locals.var_k1_ac_dn6 * locals.var_k2_ac) - (locals.var_k1_ac * locals.var_k2_ac_dn6)) / (locals.var_k2_ac * locals.var_k2_ac)) * assign44960_e50035) - (assign44960_e50032 * locals.var_k1_ac_dn6)) / (assign44960_e50035 * assign44960_e50035)), ((((((locals.var_k1_ac_dn7 * locals.var_k2_ac) - (locals.var_k1_ac * locals.var_k2_ac_dn7)) / (locals.var_k2_ac * locals.var_k2_ac)) * assign44960_e50035) - (assign44960_e50032 * locals.var_k1_ac_dn7)) / (assign44960_e50035 * assign44960_e50035)), ((((((locals.var_k1_ac_dn8 * locals.var_k2_ac) - (locals.var_k1_ac * locals.var_k2_ac_dn8)) / (locals.var_k2_ac * locals.var_k2_ac)) * assign44960_e50035) - (assign44960_e50032 * locals.var_k1_ac_dn8)) / (assign44960_e50035 * assign44960_e50035)), ((((((locals.var_k1_ac_dn9 * locals.var_k2_ac) - (locals.var_k1_ac * locals.var_k2_ac_dn9)) / (locals.var_k2_ac * locals.var_k2_ac)) * assign44960_e50035) - (assign44960_e50032 * locals.var_k1_ac_dn9)) / (assign44960_e50035 * assign44960_e50035)), );
            locals.var_r2init_op_rv = 0.0;
        }

        if (locals.var_guard1366 != 0.0) {
            let assign44970_e50043: f64 = (1.0 + locals.var_r1init_op);
            let assign44970_e50044: f64 = (locals.var_k1_ac * assign44970_e50043);
            let assign44970_e50046: f64 = (assign44970_e50044 * locals.var_diff_min_ac);
            let assign44970_e50048: f64 = (assign44970_e50046 / locals.var_a0_ac);
            let assign44970_e50049: f64 = (assign44970_e50048).ln();
            let assign44970_e50051: f64 = (assign44970_e50049 + 2.0);
            (locals.var_x1init_op, locals.var_x1init_op_dn4, locals.var_x1init_op_dn6, locals.var_x1init_op_dn7, locals.var_x1init_op_dn8, locals.var_x1init_op_dn9, ) = (assign44970_e50051, ((((((((locals.var_k1_ac_dn4 * assign44970_e50043) + (locals.var_k1_ac * locals.var_r1init_op_dn4)) * locals.var_diff_min_ac) + (assign44970_e50044 * locals.var_diff_min_ac_dn4)) * locals.var_a0_ac) - (assign44970_e50046 * locals.var_a0_ac_dn4)) / (locals.var_a0_ac * locals.var_a0_ac)) / assign44970_e50048), ((((((((locals.var_k1_ac_dn6 * assign44970_e50043) + (locals.var_k1_ac * locals.var_r1init_op_dn6)) * locals.var_diff_min_ac) + (assign44970_e50044 * locals.var_diff_min_ac_dn6)) * locals.var_a0_ac) - (assign44970_e50046 * locals.var_a0_ac_dn6)) / (locals.var_a0_ac * locals.var_a0_ac)) / assign44970_e50048), ((((((((locals.var_k1_ac_dn7 * assign44970_e50043) + (locals.var_k1_ac * locals.var_r1init_op_dn7)) * locals.var_diff_min_ac) + (assign44970_e50044 * locals.var_diff_min_ac_dn7)) * locals.var_a0_ac) - (assign44970_e50046 * locals.var_a0_ac_dn7)) / (locals.var_a0_ac * locals.var_a0_ac)) / assign44970_e50048), ((((((((locals.var_k1_ac_dn8 * assign44970_e50043) + (locals.var_k1_ac * locals.var_r1init_op_dn8)) * locals.var_diff_min_ac) + (assign44970_e50044 * locals.var_diff_min_ac_dn8)) * locals.var_a0_ac) - (assign44970_e50046 * locals.var_a0_ac_dn8)) / (locals.var_a0_ac * locals.var_a0_ac)) / assign44970_e50048), ((((((((locals.var_k1_ac_dn9 * assign44970_e50043) + (locals.var_k1_ac * locals.var_r1init_op_dn9)) * locals.var_diff_min_ac) + (assign44970_e50044 * locals.var_diff_min_ac_dn9)) * locals.var_a0_ac) - (assign44970_e50046 * locals.var_a0_ac_dn9)) / (locals.var_a0_ac * locals.var_a0_ac)) / assign44970_e50048), );
            locals.var_x1init_op_rv = 0.0;
        }

        if (locals.var_guard1366 != 0.0) {
            let assign44980_e50058: f64 = (1.0 + locals.var_r2init_op);
            let assign44980_e50059: f64 = (locals.var_k2_ac * assign44980_e50058);
            let assign44980_e50061: f64 = (assign44980_e50059 * locals.var_diff_min_ac);
            let assign44980_e50063: f64 = (assign44980_e50061 / locals.var_a0_ac);
            let assign44980_e50064: f64 = (assign44980_e50063).ln();
            let assign44980_e50066: f64 = (assign44980_e50064 + 2.0);
            (locals.var_x2init_op, locals.var_x2init_op_dn4, locals.var_x2init_op_dn6, locals.var_x2init_op_dn7, locals.var_x2init_op_dn8, locals.var_x2init_op_dn9, ) = (assign44980_e50066, ((((((((locals.var_k2_ac_dn4 * assign44980_e50058) + (locals.var_k2_ac * locals.var_r2init_op_dn4)) * locals.var_diff_min_ac) + (assign44980_e50059 * locals.var_diff_min_ac_dn4)) * locals.var_a0_ac) - (assign44980_e50061 * locals.var_a0_ac_dn4)) / (locals.var_a0_ac * locals.var_a0_ac)) / assign44980_e50063), ((((((((locals.var_k2_ac_dn6 * assign44980_e50058) + (locals.var_k2_ac * locals.var_r2init_op_dn6)) * locals.var_diff_min_ac) + (assign44980_e50059 * locals.var_diff_min_ac_dn6)) * locals.var_a0_ac) - (assign44980_e50061 * locals.var_a0_ac_dn6)) / (locals.var_a0_ac * locals.var_a0_ac)) / assign44980_e50063), ((((((((locals.var_k2_ac_dn7 * assign44980_e50058) + (locals.var_k2_ac * locals.var_r2init_op_dn7)) * locals.var_diff_min_ac) + (assign44980_e50059 * locals.var_diff_min_ac_dn7)) * locals.var_a0_ac) - (assign44980_e50061 * locals.var_a0_ac_dn7)) / (locals.var_a0_ac * locals.var_a0_ac)) / assign44980_e50063), ((((((((locals.var_k2_ac_dn8 * assign44980_e50058) + (locals.var_k2_ac * locals.var_r2init_op_dn8)) * locals.var_diff_min_ac) + (assign44980_e50059 * locals.var_diff_min_ac_dn8)) * locals.var_a0_ac) - (assign44980_e50061 * locals.var_a0_ac_dn8)) / (locals.var_a0_ac * locals.var_a0_ac)) / assign44980_e50063), ((((((((locals.var_k2_ac_dn9 * assign44980_e50058) + (locals.var_k2_ac * locals.var_r2init_op_dn9)) * locals.var_diff_min_ac) + (assign44980_e50059 * locals.var_diff_min_ac_dn9)) * locals.var_a0_ac) - (assign44980_e50061 * locals.var_a0_ac_dn9)) / (locals.var_a0_ac * locals.var_a0_ac)) / assign44980_e50063), );
            locals.var_x2init_op_rv = 0.0;
        }

        if (locals.var_guard1366 != 0.0) {
            let assign44990_e50072: f64 = (1.0 + locals.var_r1init_op);
            let assign44990_e50074: f64 = (assign44990_e50072 * locals.var_x1init_op);
            let assign44990_e50077: f64 = (locals.var_xg2x_ac * locals.var_r1init_op);
            let assign44990_e50078: f64 = (assign44990_e50074 - assign44990_e50077);
            (locals.var_xth1init_op, locals.var_xth1init_op_dn4, locals.var_xth1init_op_dn6, locals.var_xth1init_op_dn7, locals.var_xth1init_op_dn8, locals.var_xth1init_op_dn9, ) = (assign44990_e50078, (((locals.var_r1init_op_dn4 * locals.var_x1init_op) + (assign44990_e50072 * locals.var_x1init_op_dn4)) - ((locals.var_xg2x_ac_dn4 * locals.var_r1init_op) + (locals.var_xg2x_ac * locals.var_r1init_op_dn4))), (((locals.var_r1init_op_dn6 * locals.var_x1init_op) + (assign44990_e50072 * locals.var_x1init_op_dn6)) - ((locals.var_xg2x_ac_dn6 * locals.var_r1init_op) + (locals.var_xg2x_ac * locals.var_r1init_op_dn6))), (((locals.var_r1init_op_dn7 * locals.var_x1init_op) + (assign44990_e50072 * locals.var_x1init_op_dn7)) - ((locals.var_xg2x_ac_dn7 * locals.var_r1init_op) + (locals.var_xg2x_ac * locals.var_r1init_op_dn7))), (((locals.var_r1init_op_dn8 * locals.var_x1init_op) + (assign44990_e50072 * locals.var_x1init_op_dn8)) - ((locals.var_xg2x_ac_dn8 * locals.var_r1init_op) + (locals.var_xg2x_ac * locals.var_r1init_op_dn8))), (((locals.var_r1init_op_dn9 * locals.var_x1init_op) + (assign44990_e50072 * locals.var_x1init_op_dn9)) - ((locals.var_xg2x_ac_dn9 * locals.var_r1init_op) + (locals.var_xg2x_ac * locals.var_r1init_op_dn9))), );
            locals.var_xth1init_op_rv = 0.0;
        }

        if (locals.var_guard1366 != 0.0) {
            let assign45000_e50085: f64 = (1.0 / locals.var_r2init_op);
            let assign45000_e50086: f64 = (1.0 + assign45000_e50085);
            let assign45000_e50088: f64 = (assign45000_e50086 * locals.var_x2init_op);
            let assign45000_e50091: f64 = (locals.var_xg2x_ac / locals.var_r2init_op);
            let assign45000_e50092: f64 = (assign45000_e50088 - assign45000_e50091);
            (locals.var_xth2init_op, locals.var_xth2init_op_dn4, locals.var_xth2init_op_dn6, locals.var_xth2init_op_dn7, locals.var_xth2init_op_dn8, locals.var_xth2init_op_dn9, ) = (assign45000_e50092, ((((-(locals.var_r2init_op_dn4 / (locals.var_r2init_op * locals.var_r2init_op))) * locals.var_x2init_op) + (assign45000_e50086 * locals.var_x2init_op_dn4)) - (((locals.var_xg2x_ac_dn4 * locals.var_r2init_op) - (locals.var_xg2x_ac * locals.var_r2init_op_dn4)) / (locals.var_r2init_op * locals.var_r2init_op))), ((((-(locals.var_r2init_op_dn6 / (locals.var_r2init_op * locals.var_r2init_op))) * locals.var_x2init_op) + (assign45000_e50086 * locals.var_x2init_op_dn6)) - (((locals.var_xg2x_ac_dn6 * locals.var_r2init_op) - (locals.var_xg2x_ac * locals.var_r2init_op_dn6)) / (locals.var_r2init_op * locals.var_r2init_op))), ((((-(locals.var_r2init_op_dn7 / (locals.var_r2init_op * locals.var_r2init_op))) * locals.var_x2init_op) + (assign45000_e50086 * locals.var_x2init_op_dn7)) - (((locals.var_xg2x_ac_dn7 * locals.var_r2init_op) - (locals.var_xg2x_ac * locals.var_r2init_op_dn7)) / (locals.var_r2init_op * locals.var_r2init_op))), ((((-(locals.var_r2init_op_dn8 / (locals.var_r2init_op * locals.var_r2init_op))) * locals.var_x2init_op) + (assign45000_e50086 * locals.var_x2init_op_dn8)) - (((locals.var_xg2x_ac_dn8 * locals.var_r2init_op) - (locals.var_xg2x_ac * locals.var_r2init_op_dn8)) / (locals.var_r2init_op * locals.var_r2init_op))), ((((-(locals.var_r2init_op_dn9 / (locals.var_r2init_op * locals.var_r2init_op))) * locals.var_x2init_op) + (assign45000_e50086 * locals.var_x2init_op_dn9)) - (((locals.var_xg2x_ac_dn9 * locals.var_r2init_op) - (locals.var_xg2x_ac * locals.var_r2init_op_dn9)) / (locals.var_r2init_op * locals.var_r2init_op))), );
            locals.var_xth2init_op_rv = 0.0;
        }

        if (locals.var_guard1366 != 0.0) {
            let assign45010_e50099: f64 = (locals.var_xth1init_op + locals.var_xth2init_op);
            let assign45010_e50102: f64 = (locals.var_xth1init_op - locals.var_xth2init_op);
            let assign45010_e50105: f64 = (locals.var_xth1init_op - locals.var_xth2init_op);
            let assign45010_e50106: f64 = (assign45010_e50102 * assign45010_e50105);
            let assign45010_e50108: f64 = (assign45010_e50106 + 38.0);
            let assign45010_e50109: f64 = (assign45010_e50108).sqrt();
            let assign45010_e50110: f64 = (assign45010_e50099 - assign45010_e50109);
            let assign45010_e50111: f64 = (0.5 * assign45010_e50110);
            let assign45010_e50113: f64 = (assign45010_e50111 - locals.var_xg2_ac);
            let assign45010_e50115: f64 = (assign45010_e50113 / locals.var_cic1_i);
            let assign45010_e50117: f64 = (assign45010_e50115 + locals.var_xg2_ac);
            (locals.var_xg1thinit_op, locals.var_xg1thinit_op_dn4, locals.var_xg1thinit_op_dn6, locals.var_xg1thinit_op_dn7, locals.var_xg1thinit_op_dn8, locals.var_xg1thinit_op_dn9, ) = (assign45010_e50117, ((((0.5 * ((locals.var_xth1init_op_dn4 + locals.var_xth2init_op_dn4) - ((((locals.var_xth1init_op_dn4 - locals.var_xth2init_op_dn4) * assign45010_e50105) + (assign45010_e50102 * (locals.var_xth1init_op_dn4 - locals.var_xth2init_op_dn4))) / (2.0 * assign45010_e50109)))) - locals.var_xg2_ac_dn4) / locals.var_cic1_i) + locals.var_xg2_ac_dn4), ((((0.5 * ((locals.var_xth1init_op_dn6 + locals.var_xth2init_op_dn6) - ((((locals.var_xth1init_op_dn6 - locals.var_xth2init_op_dn6) * assign45010_e50105) + (assign45010_e50102 * (locals.var_xth1init_op_dn6 - locals.var_xth2init_op_dn6))) / (2.0 * assign45010_e50109)))) - locals.var_xg2_ac_dn6) / locals.var_cic1_i) + locals.var_xg2_ac_dn6), ((((0.5 * ((locals.var_xth1init_op_dn7 + locals.var_xth2init_op_dn7) - ((((locals.var_xth1init_op_dn7 - locals.var_xth2init_op_dn7) * assign45010_e50105) + (assign45010_e50102 * (locals.var_xth1init_op_dn7 - locals.var_xth2init_op_dn7))) / (2.0 * assign45010_e50109)))) - locals.var_xg2_ac_dn7) / locals.var_cic1_i) + locals.var_xg2_ac_dn7), ((((0.5 * ((locals.var_xth1init_op_dn8 + locals.var_xth2init_op_dn8) - ((((locals.var_xth1init_op_dn8 - locals.var_xth2init_op_dn8) * assign45010_e50105) + (assign45010_e50102 * (locals.var_xth1init_op_dn8 - locals.var_xth2init_op_dn8))) / (2.0 * assign45010_e50109)))) - locals.var_xg2_ac_dn8) / locals.var_cic1_i) + locals.var_xg2_ac_dn8), ((((0.5 * ((locals.var_xth1init_op_dn9 + locals.var_xth2init_op_dn9) - ((((locals.var_xth1init_op_dn9 - locals.var_xth2init_op_dn9) * assign45010_e50105) + (assign45010_e50102 * (locals.var_xth1init_op_dn9 - locals.var_xth2init_op_dn9))) / (2.0 * assign45010_e50109)))) - locals.var_xg2_ac_dn9) / locals.var_cic1_i) + locals.var_xg2_ac_dn9), );
            locals.var_xg1thinit_op_rv = 0.0;
        }

        if (locals.var_guard1366 != 0.0) {
            let assign45020_e50124: f64 = (locals.var_xg1thinit_op - locals.var_xedge_ac);
            let assign45020_e50126: f64 = (assign45020_e50124 / locals.var_sce1_ac);
            let assign45020_e50128: f64 = (assign45020_e50126 - locals.var_dxg1_dibl_ac);
            let assign45020_e50130: f64 = (assign45020_e50128 + locals.var_xedge_ac);
            let assign45020_e50131: f64 = (locals.var_phit * assign45020_e50130);
            let assign45020_e50133: f64 = (assign45020_e50131 + locals.var_vfbac1_i);
            (locals.var_vthinit_op, locals.var_vthinit_op_dn4, locals.var_vthinit_op_dn6, locals.var_vthinit_op_dn7, locals.var_vthinit_op_dn8, locals.var_vthinit_op_dn9, ) = (assign45020_e50133, (((locals.var_phit_dn4 * assign45020_e50130) + (locals.var_phit * ((((((locals.var_xg1thinit_op_dn4 - locals.var_xedge_ac_dn4) * locals.var_sce1_ac) - (assign45020_e50124 * locals.var_sce1_ac_dn4)) / (locals.var_sce1_ac * locals.var_sce1_ac)) - locals.var_dxg1_dibl_ac_dn4) + locals.var_xedge_ac_dn4))) + locals.var_vfbac1_i_dn4), (((locals.var_phit_dn6 * assign45020_e50130) + (locals.var_phit * ((((((locals.var_xg1thinit_op_dn6 - locals.var_xedge_ac_dn6) * locals.var_sce1_ac) - (assign45020_e50124 * locals.var_sce1_ac_dn6)) / (locals.var_sce1_ac * locals.var_sce1_ac)) - locals.var_dxg1_dibl_ac_dn6) + locals.var_xedge_ac_dn6))) + locals.var_vfbac1_i_dn6), (((locals.var_phit_dn7 * assign45020_e50130) + (locals.var_phit * ((((((locals.var_xg1thinit_op_dn7 - locals.var_xedge_ac_dn7) * locals.var_sce1_ac) - (assign45020_e50124 * locals.var_sce1_ac_dn7)) / (locals.var_sce1_ac * locals.var_sce1_ac)) - locals.var_dxg1_dibl_ac_dn7) + locals.var_xedge_ac_dn7))) + locals.var_vfbac1_i_dn7), (((locals.var_phit_dn8 * assign45020_e50130) + (locals.var_phit * ((((((locals.var_xg1thinit_op_dn8 - locals.var_xedge_ac_dn8) * locals.var_sce1_ac) - (assign45020_e50124 * locals.var_sce1_ac_dn8)) / (locals.var_sce1_ac * locals.var_sce1_ac)) - locals.var_dxg1_dibl_ac_dn8) + locals.var_xedge_ac_dn8))) + locals.var_vfbac1_i_dn8), (((locals.var_phit_dn9 * assign45020_e50130) + (locals.var_phit * ((((((locals.var_xg1thinit_op_dn9 - locals.var_xedge_ac_dn9) * locals.var_sce1_ac) - (assign45020_e50124 * locals.var_sce1_ac_dn9)) / (locals.var_sce1_ac * locals.var_sce1_ac)) - locals.var_dxg1_dibl_ac_dn9) + locals.var_xedge_ac_dn9))) + locals.var_vfbac1_i_dn9), );
            locals.var_vthinit_op_rv = 0.0;
        }

        if (locals.var_guard1366 != 0.0) {
            let assign45030_e50140: f64 = (locals.var_tkd - locals.var_tkr);
            let assign45030_e50141: f64 = (locals.var_stcf_i * assign45030_e50140);
            (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign45030_e50141, ((locals.var_stcf_i_dn4 * assign45030_e50140) + (locals.var_stcf_i * locals.var_tkd_dn4)), ((locals.var_stcf_i_dn6 * assign45030_e50140) + (locals.var_stcf_i * locals.var_tkd_dn6)), ((locals.var_stcf_i_dn7 * assign45030_e50140) + (locals.var_stcf_i * locals.var_tkd_dn7)), ((locals.var_stcf_i_dn8 * assign45030_e50140) + (locals.var_stcf_i * locals.var_tkd_dn8)), ((locals.var_stcf_i_dn9 * assign45030_e50140) + (locals.var_stcf_i * locals.var_tkd_dn9)), );
            locals.var_temp_rv = 0.0;
        }

        if (locals.var_guard1366 != 0.0) {
            let assign45060_e50159: f64 = (p.p14 * locals.var_stvfb_i);
            let assign45060_e50162: f64 = (locals.var_tkd - locals.var_tkr);
            let assign45060_e50163: f64 = (assign45060_e50159 * assign45060_e50162);
            let assign45060_e50165: f64 = (assign45060_e50163 + locals.var_dvfbqm);
            (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign45060_e50165, (assign45060_e50159 * locals.var_tkd_dn4), (assign45060_e50159 * locals.var_tkd_dn6), (assign45060_e50159 * locals.var_tkd_dn7), (assign45060_e50159 * locals.var_tkd_dn8), (assign45060_e50159 * locals.var_tkd_dn9), );
            locals.var_temp_rv = 0.0;
        }

        if (locals.var_guard1366 != 0.0) {
            let assign45070_e50172: f64 = (locals.var_vfbac1_t + locals.var_dvfbch_op);
            let assign45070_e50174: f64 = (assign45070_e50172 + locals.var_dvfb1nch);
            let assign45070_e50175: f64 = (p.p14 * assign45070_e50174);
            let assign45070_e50177: f64 = (assign45070_e50175 + locals.var_temp);
            let assign45070_e50179: f64 = (assign45070_e50177 + p.p34);
            let assign45070_e50181: f64 = (assign45070_e50179 - locals.var_dvfbpdep_op);
            (locals.var_vfb1_op, locals.var_vfb1_op_dn4, locals.var_vfb1_op_dn6, locals.var_vfb1_op_dn7, locals.var_vfb1_op_dn8, locals.var_vfb1_op_dn9, ) = (assign45070_e50181, (((p.p14 * ((locals.var_vfbac1_t_dn4 + locals.var_dvfbch_op_dn4) + locals.var_dvfb1nch_dn4)) + locals.var_temp_dn4) - locals.var_dvfbpdep_op_dn4), (((p.p14 * ((locals.var_vfbac1_t_dn6 + locals.var_dvfbch_op_dn6) + locals.var_dvfb1nch_dn6)) + locals.var_temp_dn6) - locals.var_dvfbpdep_op_dn6), (((p.p14 * ((locals.var_vfbac1_t_dn7 + locals.var_dvfbch_op_dn7) + locals.var_dvfb1nch_dn7)) + locals.var_temp_dn7) - locals.var_dvfbpdep_op_dn7), (((p.p14 * ((locals.var_vfbac1_t_dn8 + locals.var_dvfbch_op_dn8) + locals.var_dvfb1nch_dn8)) + locals.var_temp_dn8) - locals.var_dvfbpdep_op_dn8), (((p.p14 * ((locals.var_vfbac1_t_dn9 + locals.var_dvfbch_op_dn9) + locals.var_dvfb1nch_dn9)) + locals.var_temp_dn9) - locals.var_dvfbpdep_op_dn9), );
            locals.var_vfb1_op_rv = 0.0;
        }

        if (locals.var_guard1366 != 0.0) {
            let assign45080_e50188: f64 = (locals.var_vfbac2_t + locals.var_dvfbch_op);
            let assign45080_e50190: f64 = (assign45080_e50188 + locals.var_dvfb2nch);
            let assign45080_e50191: f64 = (p.p14 * assign45080_e50190);
            let assign45080_e50193: f64 = (assign45080_e50191 + locals.var_temp);
            (locals.var_vfb2_op, locals.var_vfb2_op_dn4, locals.var_vfb2_op_dn6, locals.var_vfb2_op_dn7, locals.var_vfb2_op_dn8, locals.var_vfb2_op_dn9, ) = (assign45080_e50193, ((p.p14 * ((locals.var_vfbac2_t_dn4 + locals.var_dvfbch_op_dn4) + locals.var_dvfb2nch_dn4)) + locals.var_temp_dn4), ((p.p14 * ((locals.var_vfbac2_t_dn6 + locals.var_dvfbch_op_dn6) + locals.var_dvfb2nch_dn6)) + locals.var_temp_dn6), ((p.p14 * ((locals.var_vfbac2_t_dn7 + locals.var_dvfbch_op_dn7) + locals.var_dvfb2nch_dn7)) + locals.var_temp_dn7), ((p.p14 * ((locals.var_vfbac2_t_dn8 + locals.var_dvfbch_op_dn8) + locals.var_dvfb2nch_dn8)) + locals.var_temp_dn8), ((p.p14 * ((locals.var_vfbac2_t_dn9 + locals.var_dvfbch_op_dn9) + locals.var_dvfb2nch_dn9)) + locals.var_temp_dn9), );
            locals.var_vfb2_op_rv = 0.0;
        }

        if (locals.var_guard1366 != 0.0) {
            let assign45090_e50199: f64 = (locals.var_vthinit_op - locals.var_vfb1_op);
            let assign45090_e50201: f64 = (assign45090_e50199 * locals.var_inv_phit_op);
            let assign45090_e50203: f64 = (assign45090_e50201 - locals.var_dxdsx_op);
            (locals.var_xg10_op, locals.var_xg10_op_dn4, locals.var_xg10_op_dn6, locals.var_xg10_op_dn7, locals.var_xg10_op_dn8, locals.var_xg10_op_dn9, ) = (assign45090_e50203, ((((locals.var_vthinit_op_dn4 - locals.var_vfb1_op_dn4) * locals.var_inv_phit_op) + (assign45090_e50199 * locals.var_inv_phit_op_dn4)) - locals.var_dxdsx_op_dn4), ((((locals.var_vthinit_op_dn6 - locals.var_vfb1_op_dn6) * locals.var_inv_phit_op) + (assign45090_e50199 * locals.var_inv_phit_op_dn6)) - locals.var_dxdsx_op_dn6), ((((locals.var_vthinit_op_dn7 - locals.var_vfb1_op_dn7) * locals.var_inv_phit_op) + (assign45090_e50199 * locals.var_inv_phit_op_dn7)) - locals.var_dxdsx_op_dn7), ((((locals.var_vthinit_op_dn8 - locals.var_vfb1_op_dn8) * locals.var_inv_phit_op) + (assign45090_e50199 * locals.var_inv_phit_op_dn8)) - locals.var_dxdsx_op_dn8), ((((locals.var_vthinit_op_dn9 - locals.var_vfb1_op_dn9) * locals.var_inv_phit_op) + (assign45090_e50199 * locals.var_inv_phit_op_dn9)) - locals.var_dxdsx_op_dn9), );
            locals.var_xg10_op_rv = 0.0;
        }

        if (locals.var_guard1366 != 0.0) {
            let assign45100_e50208: f64 = (-locals.var_vsb);
            let assign45100_e50210: f64 = (assign45100_e50208 - locals.var_vfb2_op);
            let assign45100_e50212: f64 = (assign45100_e50210 * locals.var_inv_phit_op);
            let assign45100_e50214: f64 = (assign45100_e50212 - locals.var_dxdsx_op);
            (locals.var_xg20_op, locals.var_xg20_op_dn4, locals.var_xg20_op_dn6, locals.var_xg20_op_dn7, locals.var_xg20_op_dn8, locals.var_xg20_op_dn9, ) = (assign45100_e50214, ((((-locals.var_vfb2_op_dn4) * locals.var_inv_phit_op) + (assign45100_e50210 * locals.var_inv_phit_op_dn4)) - locals.var_dxdsx_op_dn4), (((((-locals.var_vsb_dn6) - locals.var_vfb2_op_dn6) * locals.var_inv_phit_op) + (assign45100_e50210 * locals.var_inv_phit_op_dn6)) - locals.var_dxdsx_op_dn6), (((((-locals.var_vsb_dn7) - locals.var_vfb2_op_dn7) * locals.var_inv_phit_op) + (assign45100_e50210 * locals.var_inv_phit_op_dn7)) - locals.var_dxdsx_op_dn7), (((((-locals.var_vsb_dn8) - locals.var_vfb2_op_dn8) * locals.var_inv_phit_op) + (assign45100_e50210 * locals.var_inv_phit_op_dn8)) - locals.var_dxdsx_op_dn8), ((((-locals.var_vfb2_op_dn9) * locals.var_inv_phit_op) + (assign45100_e50210 * locals.var_inv_phit_op_dn9)) - locals.var_dxdsx_op_dn9), );
            locals.var_xg20_op_rv = 0.0;
        }

        let assign45110_e50219: f64 = if p.p2 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1367 = assign45110_e50219;
        locals.var_guard1367_rv = 0.0;

        if ((locals.var_guard1366 != 0.0) && (locals.var_guard1367 != 0.0)) {
            let assign45120_e50225: f64 = (p.p14 * locals.var_typesub_i);
            let assign45120_e50228: f64 = (locals.var_xg10_op - locals.var_xg20_op);
            let assign45120_e50229: f64 = (assign45120_e50225 * assign45120_e50228);
            let assign45120_e50231: f64 = (assign45120_e50229 / locals.var_gfsub);
            (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign45120_e50231, ((((assign45120_e50225 * (locals.var_xg10_op_dn4 - locals.var_xg20_op_dn4)) * locals.var_gfsub) - (assign45120_e50229 * locals.var_gfsub_dn4)) / (locals.var_gfsub * locals.var_gfsub)), ((((assign45120_e50225 * (locals.var_xg10_op_dn6 - locals.var_xg20_op_dn6)) * locals.var_gfsub) - (assign45120_e50229 * locals.var_gfsub_dn6)) / (locals.var_gfsub * locals.var_gfsub)), ((((assign45120_e50225 * (locals.var_xg10_op_dn7 - locals.var_xg20_op_dn7)) * locals.var_gfsub) - (assign45120_e50229 * locals.var_gfsub_dn7)) / (locals.var_gfsub * locals.var_gfsub)), ((((assign45120_e50225 * (locals.var_xg10_op_dn8 - locals.var_xg20_op_dn8)) * locals.var_gfsub) - (assign45120_e50229 * locals.var_gfsub_dn8)) / (locals.var_gfsub * locals.var_gfsub)), ((((assign45120_e50225 * (locals.var_xg10_op_dn9 - locals.var_xg20_op_dn9)) * locals.var_gfsub) - (assign45120_e50229 * locals.var_gfsub_dn9)) / (locals.var_gfsub * locals.var_gfsub)), );
            locals.var_temp_rv = 0.0;
        }

        let assign45130_e50236: f64 = if locals.var_temp < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1368 = assign45130_e50236;
        locals.var_guard1368_rv = 0.0;

        if (((locals.var_guard1366 != 0.0) && (locals.var_guard1367 != 0.0)) && (locals.var_guard1368 != 0.0)) {
            let assign45140_e50243: f64 = (-2.0);
            let assign45140_e50246: f64 = (1.0 - locals.var_temp);
            let assign45140_e50247: f64 = (assign45140_e50246).ln();
            let assign45140_e50248: f64 = (assign45140_e50243 * assign45140_e50247);
            (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9, ) = (assign45140_e50248, (assign45140_e50243 * ((-locals.var_temp_dn4) / assign45140_e50246)), (assign45140_e50243 * ((-locals.var_temp_dn6) / assign45140_e50246)), (assign45140_e50243 * ((-locals.var_temp_dn7) / assign45140_e50246)), (assign45140_e50243 * ((-locals.var_temp_dn8) / assign45140_e50246)), (assign45140_e50243 * ((-locals.var_temp_dn9) / assign45140_e50246)), );
            locals.var_temp1_rv = 0.0;
        }

        if (((locals.var_guard1366 != 0.0) && (locals.var_guard1367 != 0.0)) && (locals.var_guard1368 == 0.0)) {
            let assign45150_e50259: f64 = (locals.var_temp * locals.var_temp);
            let assign45150_e50263: f64 = (2.0 * locals.var_temp);
            let assign45150_e50265: f64 = (assign45150_e50263 / locals.var_gfsub);
            let assign45150_e50266: f64 = (1.0 + assign45150_e50265);
            let assign45150_e50267: f64 = (assign45150_e50259 / assign45150_e50266);
            (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9, ) = (assign45150_e50267, (((((locals.var_temp_dn4 * locals.var_temp) + (locals.var_temp * locals.var_temp_dn4)) * assign45150_e50266) - (assign45150_e50259 * ((((2.0 * locals.var_temp_dn4) * locals.var_gfsub) - (assign45150_e50263 * locals.var_gfsub_dn4)) / (locals.var_gfsub * locals.var_gfsub)))) / (assign45150_e50266 * assign45150_e50266)), (((((locals.var_temp_dn6 * locals.var_temp) + (locals.var_temp * locals.var_temp_dn6)) * assign45150_e50266) - (assign45150_e50259 * ((((2.0 * locals.var_temp_dn6) * locals.var_gfsub) - (assign45150_e50263 * locals.var_gfsub_dn6)) / (locals.var_gfsub * locals.var_gfsub)))) / (assign45150_e50266 * assign45150_e50266)), (((((locals.var_temp_dn7 * locals.var_temp) + (locals.var_temp * locals.var_temp_dn7)) * assign45150_e50266) - (assign45150_e50259 * ((((2.0 * locals.var_temp_dn7) * locals.var_gfsub) - (assign45150_e50263 * locals.var_gfsub_dn7)) / (locals.var_gfsub * locals.var_gfsub)))) / (assign45150_e50266 * assign45150_e50266)), (((((locals.var_temp_dn8 * locals.var_temp) + (locals.var_temp * locals.var_temp_dn8)) * assign45150_e50266) - (assign45150_e50259 * ((((2.0 * locals.var_temp_dn8) * locals.var_gfsub) - (assign45150_e50263 * locals.var_gfsub_dn8)) / (locals.var_gfsub * locals.var_gfsub)))) / (assign45150_e50266 * assign45150_e50266)), (((((locals.var_temp_dn9 * locals.var_temp) + (locals.var_temp * locals.var_temp_dn9)) * assign45150_e50266) - (assign45150_e50259 * ((((2.0 * locals.var_temp_dn9) * locals.var_gfsub) - (assign45150_e50263 * locals.var_gfsub_dn9)) / (locals.var_gfsub * locals.var_gfsub)))) / (assign45150_e50266 * assign45150_e50266)), );
            locals.var_temp1_rv = 0.0;
        }

        if ((locals.var_guard1366 != 0.0) && (locals.var_guard1367 != 0.0)) {
            let assign45160_e50276: f64 = (p.p14 * locals.var_typesub_i);
            let assign45160_e50278: f64 = (assign45160_e50276 * locals.var_temp1);
            let assign45160_e50279: f64 = (locals.var_xg20_op + assign45160_e50278);
            (locals.var_xg2eff_op, locals.var_xg2eff_op_dn4, locals.var_xg2eff_op_dn6, locals.var_xg2eff_op_dn7, locals.var_xg2eff_op_dn8, locals.var_xg2eff_op_dn9, ) = (assign45160_e50279, (locals.var_xg20_op_dn4 + (assign45160_e50276 * locals.var_temp1_dn4)), (locals.var_xg20_op_dn6 + (assign45160_e50276 * locals.var_temp1_dn6)), (locals.var_xg20_op_dn7 + (assign45160_e50276 * locals.var_temp1_dn7)), (locals.var_xg20_op_dn8 + (assign45160_e50276 * locals.var_temp1_dn8)), (locals.var_xg20_op_dn9 + (assign45160_e50276 * locals.var_temp1_dn9)), );
            locals.var_xg2eff_op_rv = 0.0;
        }

        if ((locals.var_guard1366 != 0.0) && (locals.var_guard1367 == 0.0)) {
            (locals.var_xg2eff_op, locals.var_xg2eff_op_dn4, locals.var_xg2eff_op_dn6, locals.var_xg2eff_op_dn7, locals.var_xg2eff_op_dn8, locals.var_xg2eff_op_dn9, ) = (locals.var_xg20_op, locals.var_xg20_op_dn4, locals.var_xg20_op_dn6, locals.var_xg20_op_dn7, locals.var_xg20_op_dn8, locals.var_xg20_op_dn9, );
            locals.var_xg2eff_op_rv = 0.0;
        }

        if (locals.var_guard1366 != 0.0) {
            let assign45180_e50293: f64 = (locals.var_xg10_op - locals.var_xg2eff_op);
            let assign45180_e50294: f64 = (locals.var_keq_1d * assign45180_e50293);
            (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign45180_e50294, (locals.var_keq_1d * (locals.var_xg10_op_dn4 - locals.var_xg2eff_op_dn4)), (locals.var_keq_1d * (locals.var_xg10_op_dn6 - locals.var_xg2eff_op_dn6)), (locals.var_keq_1d * (locals.var_xg10_op_dn7 - locals.var_xg2eff_op_dn7)), (locals.var_keq_1d * (locals.var_xg10_op_dn8 - locals.var_xg2eff_op_dn8)), (locals.var_keq_1d * (locals.var_xg10_op_dn9 - locals.var_xg2eff_op_dn9)), );
            locals.var_temp_rv = 0.0;
        }

        let assign45190_e50299: f64 = if p.p13 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1369 = assign45190_e50299;
        locals.var_guard1369_rv = 0.0;

        if ((locals.var_guard1366 != 0.0) && (locals.var_guard1369 != 0.0)) {
            let assign45200_e50306: f64 = (locals.var_temp + locals.var_emin);
            let assign45200_e50309: f64 = (locals.var_temp - locals.var_emin);
            let assign45200_e50312: f64 = (locals.var_temp - locals.var_emin);
            let assign45200_e50313: f64 = (assign45200_e50309 * assign45200_e50312);
            let assign45200_e50316: f64 = (locals.var_emin * locals.var_emin);
            let assign45200_e50317: f64 = (assign45200_e50313 + assign45200_e50316);
            let assign45200_e50318: f64 = (assign45200_e50317).sqrt();
            let assign45200_e50319: f64 = (assign45200_e50306 + assign45200_e50318);
            let assign45200_e50320: f64 = (0.5 * assign45200_e50319);
            (locals.var_e1_op, locals.var_e1_op_dn4, locals.var_e1_op_dn6, locals.var_e1_op_dn7, locals.var_e1_op_dn8, locals.var_e1_op_dn9, ) = (assign45200_e50320, (0.5 * ((locals.var_temp_dn4 + locals.var_emin_dn4) + (((((locals.var_temp_dn4 - locals.var_emin_dn4) * assign45200_e50312) + (assign45200_e50309 * (locals.var_temp_dn4 - locals.var_emin_dn4))) + ((locals.var_emin_dn4 * locals.var_emin) + (locals.var_emin * locals.var_emin_dn4))) / (2.0 * assign45200_e50318)))), (0.5 * ((locals.var_temp_dn6 + locals.var_emin_dn6) + (((((locals.var_temp_dn6 - locals.var_emin_dn6) * assign45200_e50312) + (assign45200_e50309 * (locals.var_temp_dn6 - locals.var_emin_dn6))) + ((locals.var_emin_dn6 * locals.var_emin) + (locals.var_emin * locals.var_emin_dn6))) / (2.0 * assign45200_e50318)))), (0.5 * ((locals.var_temp_dn7 + locals.var_emin_dn7) + (((((locals.var_temp_dn7 - locals.var_emin_dn7) * assign45200_e50312) + (assign45200_e50309 * (locals.var_temp_dn7 - locals.var_emin_dn7))) + ((locals.var_emin_dn7 * locals.var_emin) + (locals.var_emin * locals.var_emin_dn7))) / (2.0 * assign45200_e50318)))), (0.5 * ((locals.var_temp_dn8 + locals.var_emin_dn8) + (((((locals.var_temp_dn8 - locals.var_emin_dn8) * assign45200_e50312) + (assign45200_e50309 * (locals.var_temp_dn8 - locals.var_emin_dn8))) + ((locals.var_emin_dn8 * locals.var_emin) + (locals.var_emin * locals.var_emin_dn8))) / (2.0 * assign45200_e50318)))), (0.5 * ((locals.var_temp_dn9 + locals.var_emin_dn9) + (((((locals.var_temp_dn9 - locals.var_emin_dn9) * assign45200_e50312) + (assign45200_e50309 * (locals.var_temp_dn9 - locals.var_emin_dn9))) + ((locals.var_emin_dn9 * locals.var_emin) + (locals.var_emin * locals.var_emin_dn9))) / (2.0 * assign45200_e50318)))), );
            locals.var_e1_op_rv = 0.0;
        }

        if ((locals.var_guard1366 != 0.0) && (locals.var_guard1369 != 0.0)) {
            let assign45210_e50328: f64 = (-locals.var_temp);
            let assign45210_e50330: f64 = (assign45210_e50328 + locals.var_emin);
            let assign45210_e50332: f64 = (-locals.var_temp);
            let assign45210_e50334: f64 = (assign45210_e50332 - locals.var_emin);
            let assign45210_e50336: f64 = (-locals.var_temp);
            let assign45210_e50338: f64 = (assign45210_e50336 - locals.var_emin);
            let assign45210_e50339: f64 = (assign45210_e50334 * assign45210_e50338);
            let assign45210_e50342: f64 = (locals.var_emin * locals.var_emin);
            let assign45210_e50343: f64 = (assign45210_e50339 + assign45210_e50342);
            let assign45210_e50344: f64 = (assign45210_e50343).sqrt();
            let assign45210_e50345: f64 = (assign45210_e50330 + assign45210_e50344);
            let assign45210_e50346: f64 = (0.5 * assign45210_e50345);
            (locals.var_e2_op, locals.var_e2_op_dn4, locals.var_e2_op_dn6, locals.var_e2_op_dn7, locals.var_e2_op_dn8, locals.var_e2_op_dn9, ) = (assign45210_e50346, (0.5 * (((-locals.var_temp_dn4) + locals.var_emin_dn4) + ((((((-locals.var_temp_dn4) - locals.var_emin_dn4) * assign45210_e50338) + (assign45210_e50334 * ((-locals.var_temp_dn4) - locals.var_emin_dn4))) + ((locals.var_emin_dn4 * locals.var_emin) + (locals.var_emin * locals.var_emin_dn4))) / (2.0 * assign45210_e50344)))), (0.5 * (((-locals.var_temp_dn6) + locals.var_emin_dn6) + ((((((-locals.var_temp_dn6) - locals.var_emin_dn6) * assign45210_e50338) + (assign45210_e50334 * ((-locals.var_temp_dn6) - locals.var_emin_dn6))) + ((locals.var_emin_dn6 * locals.var_emin) + (locals.var_emin * locals.var_emin_dn6))) / (2.0 * assign45210_e50344)))), (0.5 * (((-locals.var_temp_dn7) + locals.var_emin_dn7) + ((((((-locals.var_temp_dn7) - locals.var_emin_dn7) * assign45210_e50338) + (assign45210_e50334 * ((-locals.var_temp_dn7) - locals.var_emin_dn7))) + ((locals.var_emin_dn7 * locals.var_emin) + (locals.var_emin * locals.var_emin_dn7))) / (2.0 * assign45210_e50344)))), (0.5 * (((-locals.var_temp_dn8) + locals.var_emin_dn8) + ((((((-locals.var_temp_dn8) - locals.var_emin_dn8) * assign45210_e50338) + (assign45210_e50334 * ((-locals.var_temp_dn8) - locals.var_emin_dn8))) + ((locals.var_emin_dn8 * locals.var_emin) + (locals.var_emin * locals.var_emin_dn8))) / (2.0 * assign45210_e50344)))), (0.5 * (((-locals.var_temp_dn9) + locals.var_emin_dn9) + ((((((-locals.var_temp_dn9) - locals.var_emin_dn9) * assign45210_e50338) + (assign45210_e50334 * ((-locals.var_temp_dn9) - locals.var_emin_dn9))) + ((locals.var_emin_dn9 * locals.var_emin) + (locals.var_emin * locals.var_emin_dn9))) / (2.0 * assign45210_e50344)))), );
            locals.var_e2_op_rv = 0.0;
        }

        if ((locals.var_guard1366 != 0.0) && (locals.var_guard1369 != 0.0)) {
            let assign45220_e50354: f64 = (-0.3333333333333);
            let assign45220_e50356: f64 = (locals.var_e1_op).ln();
            let assign45220_e50357: f64 = (assign45220_e50354 * assign45220_e50356);
            let assign45220_e50358: f64 = (assign45220_e50357).exp();
            let assign45220_e50359: f64 = (locals.var_qq_op * assign45220_e50358);
            (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9, ) = (assign45220_e50359, ((locals.var_qq_op_dn4 * assign45220_e50358) + (locals.var_qq_op * (assign45220_e50358 * (assign45220_e50354 * (locals.var_e1_op_dn4 / locals.var_e1_op))))), ((locals.var_qq_op_dn6 * assign45220_e50358) + (locals.var_qq_op * (assign45220_e50358 * (assign45220_e50354 * (locals.var_e1_op_dn6 / locals.var_e1_op))))), ((locals.var_qq_op_dn7 * assign45220_e50358) + (locals.var_qq_op * (assign45220_e50358 * (assign45220_e50354 * (locals.var_e1_op_dn7 / locals.var_e1_op))))), ((locals.var_qq_op_dn8 * assign45220_e50358) + (locals.var_qq_op * (assign45220_e50358 * (assign45220_e50354 * (locals.var_e1_op_dn8 / locals.var_e1_op))))), ((locals.var_qq_op_dn9 * assign45220_e50358) + (locals.var_qq_op * (assign45220_e50358 * (assign45220_e50354 * (locals.var_e1_op_dn9 / locals.var_e1_op))))), );
            locals.var_temp1_rv = 0.0;
        }

        if ((locals.var_guard1366 != 0.0) && (locals.var_guard1369 != 0.0)) {
            let assign45230_e50367: f64 = (-0.3333333333333);
            let assign45230_e50369: f64 = (locals.var_e2_op).ln();
            let assign45230_e50370: f64 = (assign45230_e50367 * assign45230_e50369);
            let assign45230_e50371: f64 = (assign45230_e50370).exp();
            let assign45230_e50372: f64 = (locals.var_qq_op * assign45230_e50371);
            (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9, ) = (assign45230_e50372, ((locals.var_qq_op_dn4 * assign45230_e50371) + (locals.var_qq_op * (assign45230_e50371 * (assign45230_e50367 * (locals.var_e2_op_dn4 / locals.var_e2_op))))), ((locals.var_qq_op_dn6 * assign45230_e50371) + (locals.var_qq_op * (assign45230_e50371 * (assign45230_e50367 * (locals.var_e2_op_dn6 / locals.var_e2_op))))), ((locals.var_qq_op_dn7 * assign45230_e50371) + (locals.var_qq_op * (assign45230_e50371 * (assign45230_e50367 * (locals.var_e2_op_dn7 / locals.var_e2_op))))), ((locals.var_qq_op_dn8 * assign45230_e50371) + (locals.var_qq_op * (assign45230_e50371 * (assign45230_e50367 * (locals.var_e2_op_dn8 / locals.var_e2_op))))), ((locals.var_qq_op_dn9 * assign45230_e50371) + (locals.var_qq_op * (assign45230_e50371 * (assign45230_e50367 * (locals.var_e2_op_dn9 / locals.var_e2_op))))), );
            locals.var_temp2_rv = 0.0;
        }

        if ((locals.var_guard1366 != 0.0) && (locals.var_guard1369 != 0.0)) {
            let assign45240_e50380: f64 = (1.0 - locals.var_temp1);
            let assign45240_e50382: f64 = (assign45240_e50380 - locals.var_temp2);
            (locals.var_temp3, locals.var_temp3_dn4, locals.var_temp3_dn6, locals.var_temp3_dn7, locals.var_temp3_dn8, locals.var_temp3_dn9, ) = (assign45240_e50382, ((-locals.var_temp1_dn4) - locals.var_temp2_dn4), ((-locals.var_temp1_dn6) - locals.var_temp2_dn6), ((-locals.var_temp1_dn7) - locals.var_temp2_dn7), ((-locals.var_temp1_dn8) - locals.var_temp2_dn8), ((-locals.var_temp1_dn9) - locals.var_temp2_dn9), );
            locals.var_temp3_rv = 0.0;
        }

        if ((locals.var_guard1366 != 0.0) && (locals.var_guard1369 != 0.0)) {
            let assign45260_e50398: f64 = (locals.var_k1_1d * locals.var_temp3);
            let assign45260_e50402: f64 = (locals.var_k1_1d * locals.var_temp1);
            let assign45260_e50403: f64 = (1.0 + assign45260_e50402);
            let assign45260_e50404: f64 = (assign45260_e50398 / assign45260_e50403);
            (locals.var_k1_1d_qm_op, locals.var_k1_1d_qm_op_dn4, locals.var_k1_1d_qm_op_dn6, locals.var_k1_1d_qm_op_dn7, locals.var_k1_1d_qm_op_dn8, locals.var_k1_1d_qm_op_dn9, ) = (assign45260_e50404, ((((locals.var_k1_1d * locals.var_temp3_dn4) * assign45260_e50403) - (assign45260_e50398 * (locals.var_k1_1d * locals.var_temp1_dn4))) / (assign45260_e50403 * assign45260_e50403)), ((((locals.var_k1_1d * locals.var_temp3_dn6) * assign45260_e50403) - (assign45260_e50398 * (locals.var_k1_1d * locals.var_temp1_dn6))) / (assign45260_e50403 * assign45260_e50403)), ((((locals.var_k1_1d * locals.var_temp3_dn7) * assign45260_e50403) - (assign45260_e50398 * (locals.var_k1_1d * locals.var_temp1_dn7))) / (assign45260_e50403 * assign45260_e50403)), ((((locals.var_k1_1d * locals.var_temp3_dn8) * assign45260_e50403) - (assign45260_e50398 * (locals.var_k1_1d * locals.var_temp1_dn8))) / (assign45260_e50403 * assign45260_e50403)), ((((locals.var_k1_1d * locals.var_temp3_dn9) * assign45260_e50403) - (assign45260_e50398 * (locals.var_k1_1d * locals.var_temp1_dn9))) / (assign45260_e50403 * assign45260_e50403)), );
            locals.var_k1_1d_qm_op_rv = 0.0;
        }

        if ((locals.var_guard1366 != 0.0) && (locals.var_guard1369 != 0.0)) {
            let assign45270_e50412: f64 = (locals.var_k2_1d * locals.var_temp3);
            let assign45270_e50416: f64 = (locals.var_k2_1d * locals.var_temp2);
            let assign45270_e50417: f64 = (1.0 + assign45270_e50416);
            let assign45270_e50418: f64 = (assign45270_e50412 / assign45270_e50417);
            (locals.var_k2_1d_qm_op, locals.var_k2_1d_qm_op_dn4, locals.var_k2_1d_qm_op_dn6, locals.var_k2_1d_qm_op_dn7, locals.var_k2_1d_qm_op_dn8, locals.var_k2_1d_qm_op_dn9, ) = (assign45270_e50418, ((((locals.var_k2_1d * locals.var_temp3_dn4) * assign45270_e50417) - (assign45270_e50412 * (locals.var_k2_1d * locals.var_temp2_dn4))) / (assign45270_e50417 * assign45270_e50417)), ((((locals.var_k2_1d * locals.var_temp3_dn6) * assign45270_e50417) - (assign45270_e50412 * (locals.var_k2_1d * locals.var_temp2_dn6))) / (assign45270_e50417 * assign45270_e50417)), ((((locals.var_k2_1d * locals.var_temp3_dn7) * assign45270_e50417) - (assign45270_e50412 * (locals.var_k2_1d * locals.var_temp2_dn7))) / (assign45270_e50417 * assign45270_e50417)), ((((locals.var_k2_1d * locals.var_temp3_dn8) * assign45270_e50417) - (assign45270_e50412 * (locals.var_k2_1d * locals.var_temp2_dn8))) / (assign45270_e50417 * assign45270_e50417)), ((((locals.var_k2_1d * locals.var_temp3_dn9) * assign45270_e50417) - (assign45270_e50412 * (locals.var_k2_1d * locals.var_temp2_dn9))) / (assign45270_e50417 * assign45270_e50417)), );
            locals.var_k2_1d_qm_op_rv = 0.0;
        }

        if ((locals.var_guard1366 != 0.0) && (locals.var_guard1369 != 0.0)) {
            let assign45280_e50428: f64 = (1.0 / locals.var_k1_1d_qm_op);
            let assign45280_e50429: f64 = (1.0 + assign45280_e50428);
            let assign45280_e50432: f64 = (1.0 / locals.var_k2_1d_qm_op);
            let assign45280_e50433: f64 = (assign45280_e50429 + assign45280_e50432);
            let assign45280_e50434: f64 = (1.0 / assign45280_e50433);
            (locals.var_keq_1d_qm_op, locals.var_keq_1d_qm_op_dn4, locals.var_keq_1d_qm_op_dn6, locals.var_keq_1d_qm_op_dn7, locals.var_keq_1d_qm_op_dn8, locals.var_keq_1d_qm_op_dn9, ) = (assign45280_e50434, (-(((-(locals.var_k1_1d_qm_op_dn4 / (locals.var_k1_1d_qm_op * locals.var_k1_1d_qm_op))) + (-(locals.var_k2_1d_qm_op_dn4 / (locals.var_k2_1d_qm_op * locals.var_k2_1d_qm_op)))) / (assign45280_e50433 * assign45280_e50433))), (-(((-(locals.var_k1_1d_qm_op_dn6 / (locals.var_k1_1d_qm_op * locals.var_k1_1d_qm_op))) + (-(locals.var_k2_1d_qm_op_dn6 / (locals.var_k2_1d_qm_op * locals.var_k2_1d_qm_op)))) / (assign45280_e50433 * assign45280_e50433))), (-(((-(locals.var_k1_1d_qm_op_dn7 / (locals.var_k1_1d_qm_op * locals.var_k1_1d_qm_op))) + (-(locals.var_k2_1d_qm_op_dn7 / (locals.var_k2_1d_qm_op * locals.var_k2_1d_qm_op)))) / (assign45280_e50433 * assign45280_e50433))), (-(((-(locals.var_k1_1d_qm_op_dn8 / (locals.var_k1_1d_qm_op * locals.var_k1_1d_qm_op))) + (-(locals.var_k2_1d_qm_op_dn8 / (locals.var_k2_1d_qm_op * locals.var_k2_1d_qm_op)))) / (assign45280_e50433 * assign45280_e50433))), (-(((-(locals.var_k1_1d_qm_op_dn9 / (locals.var_k1_1d_qm_op * locals.var_k1_1d_qm_op))) + (-(locals.var_k2_1d_qm_op_dn9 / (locals.var_k2_1d_qm_op * locals.var_k2_1d_qm_op)))) / (assign45280_e50433 * assign45280_e50433))), );
            locals.var_keq_1d_qm_op_rv = 0.0;
        }

        if ((locals.var_guard1366 != 0.0) && (locals.var_guard1369 == 0.0)) {
            (locals.var_k1_1d_qm_op, locals.var_k1_1d_qm_op_dn4, locals.var_k1_1d_qm_op_dn6, locals.var_k1_1d_qm_op_dn7, locals.var_k1_1d_qm_op_dn8, locals.var_k1_1d_qm_op_dn9, ) = (locals.var_k1_1d, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_k1_1d_qm_op_rv = 0.0;
            (locals.var_k2_1d_qm_op, locals.var_k2_1d_qm_op_dn4, locals.var_k2_1d_qm_op_dn6, locals.var_k2_1d_qm_op_dn7, locals.var_k2_1d_qm_op_dn8, locals.var_k2_1d_qm_op_dn9, ) = (locals.var_k2_1d, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_k2_1d_qm_op_rv = 0.0;
            (locals.var_keq_1d_qm_op, locals.var_keq_1d_qm_op_dn4, locals.var_keq_1d_qm_op_dn6, locals.var_keq_1d_qm_op_dn7, locals.var_keq_1d_qm_op_dn8, locals.var_keq_1d_qm_op_dn9, ) = (locals.var_keq_1d, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_keq_1d_qm_op_rv = 0.0;
        }

        if (locals.var_guard1366 != 0.0) {
            let assign45330_e50469: f64 = (locals.var_xg10_op - locals.var_xg2eff_op);
            let assign45330_e50470: f64 = (locals.var_keq_1d_qm_op * assign45330_e50469);
            (locals.var_dx_wi_1d_op, locals.var_dx_wi_1d_op_dn4, locals.var_dx_wi_1d_op_dn6, locals.var_dx_wi_1d_op_dn7, locals.var_dx_wi_1d_op_dn8, locals.var_dx_wi_1d_op_dn9, ) = (assign45330_e50470, ((locals.var_keq_1d_qm_op_dn4 * assign45330_e50469) + (locals.var_keq_1d_qm_op * (locals.var_xg10_op_dn4 - locals.var_xg2eff_op_dn4))), ((locals.var_keq_1d_qm_op_dn6 * assign45330_e50469) + (locals.var_keq_1d_qm_op * (locals.var_xg10_op_dn6 - locals.var_xg2eff_op_dn6))), ((locals.var_keq_1d_qm_op_dn7 * assign45330_e50469) + (locals.var_keq_1d_qm_op * (locals.var_xg10_op_dn7 - locals.var_xg2eff_op_dn7))), ((locals.var_keq_1d_qm_op_dn8 * assign45330_e50469) + (locals.var_keq_1d_qm_op * (locals.var_xg10_op_dn8 - locals.var_xg2eff_op_dn8))), ((locals.var_keq_1d_qm_op_dn9 * assign45330_e50469) + (locals.var_keq_1d_qm_op * (locals.var_xg10_op_dn9 - locals.var_xg2eff_op_dn9))), );
            locals.var_dx_wi_1d_op_rv = 0.0;
        }

        let assign45340_e50475: f64 = if locals.var_dx_wi_1d_op > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1370 = assign45340_e50475;
        locals.var_guard1370_rv = 0.0;

        let assign45350_e50477: f64 = (-locals.var_dx_wi_1d_op);
        let assign45350_e50479: f64 = if assign45350_e50477 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard1371 = assign45350_e50479;
        locals.var_guard1371_rv = 0.0;

        if (((locals.var_guard1366 != 0.0) && (locals.var_guard1370 != 0.0)) && (locals.var_guard1371 != 0.0)) {
            let assign45360_e50487: f64 = (-locals.var_dx_wi_1d_op);
            let assign45360_e50488: f64 = (assign45360_e50487).exp();
            let assign45360_e50489: f64 = (1.0 + assign45360_e50488);
            let assign45360_e50490: f64 = (assign45360_e50489).ln();
            (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign45360_e50490, ((assign45360_e50488 * (-locals.var_dx_wi_1d_op_dn4)) / assign45360_e50489), ((assign45360_e50488 * (-locals.var_dx_wi_1d_op_dn6)) / assign45360_e50489), ((assign45360_e50488 * (-locals.var_dx_wi_1d_op_dn7)) / assign45360_e50489), ((assign45360_e50488 * (-locals.var_dx_wi_1d_op_dn8)) / assign45360_e50489), ((assign45360_e50488 * (-locals.var_dx_wi_1d_op_dn9)) / assign45360_e50489), );
            locals.var_temp_rv = 0.0;
        }

        if (((locals.var_guard1366 != 0.0) && (locals.var_guard1370 != 0.0)) && (locals.var_guard1371 == 0.0)) {
            let assign45370_e50500: f64 = (-locals.var_dx_wi_1d_op);
            (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign45370_e50500, (-locals.var_dx_wi_1d_op_dn4), (-locals.var_dx_wi_1d_op_dn6), (-locals.var_dx_wi_1d_op_dn7), (-locals.var_dx_wi_1d_op_dn8), (-locals.var_dx_wi_1d_op_dn9), );
            locals.var_temp_rv = 0.0;
        }

        if ((locals.var_guard1366 != 0.0) && (locals.var_guard1370 != 0.0)) {
            let assign45380_e50509: f64 = (locals.var_dx_wi_1d_op / locals.var_k1_1d_qm_op);
            let assign45380_e50510: f64 = (locals.var_xg10_op - assign45380_e50509);
            let assign45380_e50512: f64 = (assign45380_e50510 + locals.var_temp);
            let assign45380_e50514: f64 = (assign45380_e50512 - 0.6931471805599);
            (locals.var_x_wi_1d_op, locals.var_x_wi_1d_op_dn4, locals.var_x_wi_1d_op_dn6, locals.var_x_wi_1d_op_dn7, locals.var_x_wi_1d_op_dn8, locals.var_x_wi_1d_op_dn9, ) = (assign45380_e50514, ((locals.var_xg10_op_dn4 - (((locals.var_dx_wi_1d_op_dn4 * locals.var_k1_1d_qm_op) - (locals.var_dx_wi_1d_op * locals.var_k1_1d_qm_op_dn4)) / (locals.var_k1_1d_qm_op * locals.var_k1_1d_qm_op))) + locals.var_temp_dn4), ((locals.var_xg10_op_dn6 - (((locals.var_dx_wi_1d_op_dn6 * locals.var_k1_1d_qm_op) - (locals.var_dx_wi_1d_op * locals.var_k1_1d_qm_op_dn6)) / (locals.var_k1_1d_qm_op * locals.var_k1_1d_qm_op))) + locals.var_temp_dn6), ((locals.var_xg10_op_dn7 - (((locals.var_dx_wi_1d_op_dn7 * locals.var_k1_1d_qm_op) - (locals.var_dx_wi_1d_op * locals.var_k1_1d_qm_op_dn7)) / (locals.var_k1_1d_qm_op * locals.var_k1_1d_qm_op))) + locals.var_temp_dn7), ((locals.var_xg10_op_dn8 - (((locals.var_dx_wi_1d_op_dn8 * locals.var_k1_1d_qm_op) - (locals.var_dx_wi_1d_op * locals.var_k1_1d_qm_op_dn8)) / (locals.var_k1_1d_qm_op * locals.var_k1_1d_qm_op))) + locals.var_temp_dn8), ((locals.var_xg10_op_dn9 - (((locals.var_dx_wi_1d_op_dn9 * locals.var_k1_1d_qm_op) - (locals.var_dx_wi_1d_op * locals.var_k1_1d_qm_op_dn9)) / (locals.var_k1_1d_qm_op * locals.var_k1_1d_qm_op))) + locals.var_temp_dn9), );
            locals.var_x_wi_1d_op_rv = 0.0;
        }

        let assign45390_e50519: f64 = if locals.var_dx_wi_1d_op < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard1372 = assign45390_e50519;
        locals.var_guard1372_rv = 0.0;

        if (((locals.var_guard1366 != 0.0) && (locals.var_guard1370 == 0.0)) && (locals.var_guard1372 != 0.0)) {
            let assign45400_e50528: f64 = (locals.var_dx_wi_1d_op).exp();
            let assign45400_e50529: f64 = (1.0 + assign45400_e50528);
            let assign45400_e50530: f64 = (assign45400_e50529).ln();
            (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign45400_e50530, ((assign45400_e50528 * locals.var_dx_wi_1d_op_dn4) / assign45400_e50529), ((assign45400_e50528 * locals.var_dx_wi_1d_op_dn6) / assign45400_e50529), ((assign45400_e50528 * locals.var_dx_wi_1d_op_dn7) / assign45400_e50529), ((assign45400_e50528 * locals.var_dx_wi_1d_op_dn8) / assign45400_e50529), ((assign45400_e50528 * locals.var_dx_wi_1d_op_dn9) / assign45400_e50529), );
            locals.var_temp_rv = 0.0;
        }

        if (((locals.var_guard1366 != 0.0) && (locals.var_guard1370 == 0.0)) && (locals.var_guard1372 == 0.0)) {
            (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (locals.var_dx_wi_1d_op, locals.var_dx_wi_1d_op_dn4, locals.var_dx_wi_1d_op_dn6, locals.var_dx_wi_1d_op_dn7, locals.var_dx_wi_1d_op_dn8, locals.var_dx_wi_1d_op_dn9, );
            locals.var_temp_rv = 0.0;
        }

        if ((locals.var_guard1366 != 0.0) && (locals.var_guard1370 == 0.0)) {
            let assign45420_e50550: f64 = (locals.var_dx_wi_1d_op / locals.var_k2_1d_qm_op);
            let assign45420_e50551: f64 = (locals.var_xg2eff_op + assign45420_e50550);
            let assign45420_e50553: f64 = (assign45420_e50551 + locals.var_temp);
            let assign45420_e50555: f64 = (assign45420_e50553 - 0.6931471805599);
            (locals.var_x_wi_1d_op, locals.var_x_wi_1d_op_dn4, locals.var_x_wi_1d_op_dn6, locals.var_x_wi_1d_op_dn7, locals.var_x_wi_1d_op_dn8, locals.var_x_wi_1d_op_dn9, ) = (assign45420_e50555, ((locals.var_xg2eff_op_dn4 + (((locals.var_dx_wi_1d_op_dn4 * locals.var_k2_1d_qm_op) - (locals.var_dx_wi_1d_op * locals.var_k2_1d_qm_op_dn4)) / (locals.var_k2_1d_qm_op * locals.var_k2_1d_qm_op))) + locals.var_temp_dn4), ((locals.var_xg2eff_op_dn6 + (((locals.var_dx_wi_1d_op_dn6 * locals.var_k2_1d_qm_op) - (locals.var_dx_wi_1d_op * locals.var_k2_1d_qm_op_dn6)) / (locals.var_k2_1d_qm_op * locals.var_k2_1d_qm_op))) + locals.var_temp_dn6), ((locals.var_xg2eff_op_dn7 + (((locals.var_dx_wi_1d_op_dn7 * locals.var_k2_1d_qm_op) - (locals.var_dx_wi_1d_op * locals.var_k2_1d_qm_op_dn7)) / (locals.var_k2_1d_qm_op * locals.var_k2_1d_qm_op))) + locals.var_temp_dn7), ((locals.var_xg2eff_op_dn8 + (((locals.var_dx_wi_1d_op_dn8 * locals.var_k2_1d_qm_op) - (locals.var_dx_wi_1d_op * locals.var_k2_1d_qm_op_dn8)) / (locals.var_k2_1d_qm_op * locals.var_k2_1d_qm_op))) + locals.var_temp_dn8), ((locals.var_xg2eff_op_dn9 + (((locals.var_dx_wi_1d_op_dn9 * locals.var_k2_1d_qm_op) - (locals.var_dx_wi_1d_op * locals.var_k2_1d_qm_op_dn9)) / (locals.var_k2_1d_qm_op * locals.var_k2_1d_qm_op))) + locals.var_temp_dn9), );
            locals.var_x_wi_1d_op_rv = 0.0;
        }

    }

    pub(super) fn stamp_reactive_block_64(
        locals: &mut StampLocals,
    ) {
        if (locals.var_guard1366 != 0.0) {
            let assign45430_e50562: f64 = (locals.var_x_wi_1d_op + locals.var_xth_1d_op);
            let assign45430_e50565: f64 = (locals.var_x_wi_1d_op - locals.var_xth_1d_op);
            let assign45430_e50568: f64 = (locals.var_x_wi_1d_op - locals.var_xth_1d_op);
            let assign45430_e50569: f64 = (assign45430_e50565 * assign45430_e50568);
            let assign45430_e50571: f64 = (assign45430_e50569 + 4.0);
            let assign45430_e50572: f64 = (assign45430_e50571).sqrt();
            let assign45430_e50573: f64 = (assign45430_e50562 - assign45430_e50572);
            let assign45430_e50574: f64 = (0.5 * assign45430_e50573);
            (locals.var_x_1d_op, locals.var_x_1d_op_dn4, locals.var_x_1d_op_dn6, locals.var_x_1d_op_dn7, locals.var_x_1d_op_dn8, locals.var_x_1d_op_dn9, ) = (assign45430_e50574, (0.5 * ((locals.var_x_wi_1d_op_dn4 + locals.var_xth_1d_op_dn4) - ((((locals.var_x_wi_1d_op_dn4 - locals.var_xth_1d_op_dn4) * assign45430_e50568) + (assign45430_e50565 * (locals.var_x_wi_1d_op_dn4 - locals.var_xth_1d_op_dn4))) / (2.0 * assign45430_e50572)))), (0.5 * ((locals.var_x_wi_1d_op_dn6 + locals.var_xth_1d_op_dn6) - ((((locals.var_x_wi_1d_op_dn6 - locals.var_xth_1d_op_dn6) * assign45430_e50568) + (assign45430_e50565 * (locals.var_x_wi_1d_op_dn6 - locals.var_xth_1d_op_dn6))) / (2.0 * assign45430_e50572)))), (0.5 * ((locals.var_x_wi_1d_op_dn7 + locals.var_xth_1d_op_dn7) - ((((locals.var_x_wi_1d_op_dn7 - locals.var_xth_1d_op_dn7) * assign45430_e50568) + (assign45430_e50565 * (locals.var_x_wi_1d_op_dn7 - locals.var_xth_1d_op_dn7))) / (2.0 * assign45430_e50572)))), (0.5 * ((locals.var_x_wi_1d_op_dn8 + locals.var_xth_1d_op_dn8) - ((((locals.var_x_wi_1d_op_dn8 - locals.var_xth_1d_op_dn8) * assign45430_e50568) + (assign45430_e50565 * (locals.var_x_wi_1d_op_dn8 - locals.var_xth_1d_op_dn8))) / (2.0 * assign45430_e50572)))), (0.5 * ((locals.var_x_wi_1d_op_dn9 + locals.var_xth_1d_op_dn9) - ((((locals.var_x_wi_1d_op_dn9 - locals.var_xth_1d_op_dn9) * assign45430_e50568) + (assign45430_e50565 * (locals.var_x_wi_1d_op_dn9 - locals.var_xth_1d_op_dn9))) / (2.0 * assign45430_e50572)))), );
            locals.var_x_1d_op_rv = 0.0;
        }

        if (locals.var_guard1366 != 0.0) {
            let assign45440_e50582: f64 = (locals.var_xth_1d_op - locals.var_x_1d_op);
            let assign45440_e50583: f64 = (2.0 * assign45440_e50582);
            let assign45440_e50585: f64 = (assign45440_e50583 / locals.var_xsddep_op);
            let assign45440_e50586: f64 = (1.0 + assign45440_e50585);
            let assign45440_e50587: f64 = (assign45440_e50586).sqrt();
            let assign45440_e50589: f64 = (assign45440_e50587 - 1.0);
            (locals.var_dleff_op, locals.var_dleff_op_dn4, locals.var_dleff_op_dn6, locals.var_dleff_op_dn7, locals.var_dleff_op_dn8, locals.var_dleff_op_dn9, ) = (assign45440_e50589, (((((2.0 * (locals.var_xth_1d_op_dn4 - locals.var_x_1d_op_dn4)) * locals.var_xsddep_op) - (assign45440_e50583 * locals.var_xsddep_op_dn4)) / (locals.var_xsddep_op * locals.var_xsddep_op)) / (2.0 * assign45440_e50587)), (((((2.0 * (locals.var_xth_1d_op_dn6 - locals.var_x_1d_op_dn6)) * locals.var_xsddep_op) - (assign45440_e50583 * locals.var_xsddep_op_dn6)) / (locals.var_xsddep_op * locals.var_xsddep_op)) / (2.0 * assign45440_e50587)), (((((2.0 * (locals.var_xth_1d_op_dn7 - locals.var_x_1d_op_dn7)) * locals.var_xsddep_op) - (assign45440_e50583 * locals.var_xsddep_op_dn7)) / (locals.var_xsddep_op * locals.var_xsddep_op)) / (2.0 * assign45440_e50587)), (((((2.0 * (locals.var_xth_1d_op_dn8 - locals.var_x_1d_op_dn8)) * locals.var_xsddep_op) - (assign45440_e50583 * locals.var_xsddep_op_dn8)) / (locals.var_xsddep_op * locals.var_xsddep_op)) / (2.0 * assign45440_e50587)), (((((2.0 * (locals.var_xth_1d_op_dn9 - locals.var_x_1d_op_dn9)) * locals.var_xsddep_op) - (assign45440_e50583 * locals.var_xsddep_op_dn9)) / (locals.var_xsddep_op * locals.var_xsddep_op)) / (2.0 * assign45440_e50587)), );
            locals.var_dleff_op_rv = 0.0;
        }

        if (locals.var_guard1366 != 0.0) {
            let assign45460_e50605: f64 = (locals.var_pscedlb_i * locals.var_xg20_op);
            let assign45460_e50606: f64 = (1.0 + assign45460_e50605);
            let assign45460_e50608: f64 = (assign45460_e50606 + 0.5);
            let assign45460_e50612: f64 = (locals.var_pscedlb_i * locals.var_xg20_op);
            let assign45460_e50613: f64 = (1.0 + assign45460_e50612);
            let assign45460_e50615: f64 = (assign45460_e50613 - 0.5);
            let assign45460_e50619: f64 = (locals.var_pscedlb_i * locals.var_xg20_op);
            let assign45460_e50620: f64 = (1.0 + assign45460_e50619);
            let assign45460_e50622: f64 = (assign45460_e50620 - 0.5);
            let assign45460_e50623: f64 = (assign45460_e50615 * assign45460_e50622);
            let assign45460_e50625: f64 = (assign45460_e50623 + 0.01);
            let assign45460_e50626: f64 = (assign45460_e50625).sqrt();
            let assign45460_e50627: f64 = (assign45460_e50608 + assign45460_e50626);
            let assign45460_e50628: f64 = (0.5 * assign45460_e50627);
            (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign45460_e50628, (0.5 * ((locals.var_pscedlb_i * locals.var_xg20_op_dn4) + ((((locals.var_pscedlb_i * locals.var_xg20_op_dn4) * assign45460_e50622) + (assign45460_e50615 * (locals.var_pscedlb_i * locals.var_xg20_op_dn4))) / (2.0 * assign45460_e50626)))), (0.5 * ((locals.var_pscedlb_i * locals.var_xg20_op_dn6) + ((((locals.var_pscedlb_i * locals.var_xg20_op_dn6) * assign45460_e50622) + (assign45460_e50615 * (locals.var_pscedlb_i * locals.var_xg20_op_dn6))) / (2.0 * assign45460_e50626)))), (0.5 * ((locals.var_pscedlb_i * locals.var_xg20_op_dn7) + ((((locals.var_pscedlb_i * locals.var_xg20_op_dn7) * assign45460_e50622) + (assign45460_e50615 * (locals.var_pscedlb_i * locals.var_xg20_op_dn7))) / (2.0 * assign45460_e50626)))), (0.5 * ((locals.var_pscedlb_i * locals.var_xg20_op_dn8) + ((((locals.var_pscedlb_i * locals.var_xg20_op_dn8) * assign45460_e50622) + (assign45460_e50615 * (locals.var_pscedlb_i * locals.var_xg20_op_dn8))) / (2.0 * assign45460_e50626)))), (0.5 * ((locals.var_pscedlb_i * locals.var_xg20_op_dn9) + ((((locals.var_pscedlb_i * locals.var_xg20_op_dn9) * assign45460_e50622) + (assign45460_e50615 * (locals.var_pscedlb_i * locals.var_xg20_op_dn9))) / (2.0 * assign45460_e50626)))), );
            locals.var_temp_rv = 0.0;
        }

        if (locals.var_guard1366 != 0.0) {
            let assign45490_e50654: f64 = (2.0 * locals.var_xd0_op);
            let assign45490_e50658: f64 = (locals.var_xdsx_op / locals.var_xd0_op);
            let assign45490_e50659: f64 = (1.0 + assign45490_e50658);
            let assign45490_e50660: f64 = (assign45490_e50659).sqrt();
            let assign45490_e50662: f64 = (assign45490_e50660 - 1.0);
            let assign45490_e50663: f64 = (assign45490_e50654 * assign45490_e50662);
            let assign45490_e50667: f64 = (locals.var_cfdl_i * locals.var_dleff_op);
            let assign45490_e50668: f64 = (1.0 + assign45490_e50667);
            let assign45490_e50669: f64 = (assign45490_e50663 * assign45490_e50668);
            let assign45490_e50673: f64 = (locals.var_cfdlb_i * locals.var_xg20_op);
            let assign45490_e50674: f64 = (1.0 + assign45490_e50673);
            let assign45490_e50675: f64 = (assign45490_e50669 * assign45490_e50674);
            (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign45490_e50675, (((((((2.0 * locals.var_xd0_op_dn4) * assign45490_e50662) + (assign45490_e50654 * ((((locals.var_xdsx_op_dn4 * locals.var_xd0_op) - (locals.var_xdsx_op * locals.var_xd0_op_dn4)) / (locals.var_xd0_op * locals.var_xd0_op)) / (2.0 * assign45490_e50660)))) * assign45490_e50668) + (assign45490_e50663 * (locals.var_cfdl_i * locals.var_dleff_op_dn4))) * assign45490_e50674) + (assign45490_e50669 * (locals.var_cfdlb_i * locals.var_xg20_op_dn4))), (((((((2.0 * locals.var_xd0_op_dn6) * assign45490_e50662) + (assign45490_e50654 * ((((locals.var_xdsx_op_dn6 * locals.var_xd0_op) - (locals.var_xdsx_op * locals.var_xd0_op_dn6)) / (locals.var_xd0_op * locals.var_xd0_op)) / (2.0 * assign45490_e50660)))) * assign45490_e50668) + (assign45490_e50663 * (locals.var_cfdl_i * locals.var_dleff_op_dn6))) * assign45490_e50674) + (assign45490_e50669 * (locals.var_cfdlb_i * locals.var_xg20_op_dn6))), (((((((2.0 * locals.var_xd0_op_dn7) * assign45490_e50662) + (assign45490_e50654 * ((((locals.var_xdsx_op_dn7 * locals.var_xd0_op) - (locals.var_xdsx_op * locals.var_xd0_op_dn7)) / (locals.var_xd0_op * locals.var_xd0_op)) / (2.0 * assign45490_e50660)))) * assign45490_e50668) + (assign45490_e50663 * (locals.var_cfdl_i * locals.var_dleff_op_dn7))) * assign45490_e50674) + (assign45490_e50669 * (locals.var_cfdlb_i * locals.var_xg20_op_dn7))), (((((((2.0 * locals.var_xd0_op_dn8) * assign45490_e50662) + (assign45490_e50654 * ((((locals.var_xdsx_op_dn8 * locals.var_xd0_op) - (locals.var_xdsx_op * locals.var_xd0_op_dn8)) / (locals.var_xd0_op * locals.var_xd0_op)) / (2.0 * assign45490_e50660)))) * assign45490_e50668) + (assign45490_e50663 * (locals.var_cfdl_i * locals.var_dleff_op_dn8))) * assign45490_e50674) + (assign45490_e50669 * (locals.var_cfdlb_i * locals.var_xg20_op_dn8))), (((((((2.0 * locals.var_xd0_op_dn9) * assign45490_e50662) + (assign45490_e50654 * ((((locals.var_xdsx_op_dn9 * locals.var_xd0_op) - (locals.var_xdsx_op * locals.var_xd0_op_dn9)) / (locals.var_xd0_op * locals.var_xd0_op)) / (2.0 * assign45490_e50660)))) * assign45490_e50668) + (assign45490_e50663 * (locals.var_cfdl_i * locals.var_dleff_op_dn9))) * assign45490_e50674) + (assign45490_e50669 * (locals.var_cfdlb_i * locals.var_xg20_op_dn9))), );
            locals.var_temp_rv = 0.0;
        }

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
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv10 = ctx.node_voltage(nodes[10]);
        let nv11 = ctx.node_voltage(nodes[11]);
        let nv12 = ctx.node_voltage(nodes[12]);
        let nv13 = ctx.node_voltage(nodes[13]);
        let (eq0_e510, eq0_e510_d_n4, eq0_e510_d_n6, eq0_e510_d_n7, eq0_e510_d_n8, eq0_e510_d_n9,) = {
    if (locals.var_guard1239 != 0.0) {
        let eq0_e508: f64 = (p.p14 * locals.var_idse);
        let eq0_e508_d_n4: f64 = (p.p14 * locals.var_idse_dn4);
        let eq0_e508_d_n6: f64 = (p.p14 * locals.var_idse_dn6);
        let eq0_e508_d_n7: f64 = (p.p14 * locals.var_idse_dn7);
        let eq0_e508_d_n8: f64 = (p.p14 * locals.var_idse_dn8);
        let eq0_e508_d_n9: f64 = (p.p14 * locals.var_idse_dn9);
        (eq0_e508, eq0_e508_d_n4, eq0_e508_d_n6, eq0_e508_d_n7, eq0_e508_d_n8, eq0_e508_d_n9,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq0_value: f64 = eq0_e510;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(6),
            Some(7),
            multiplicity * (eq0_value),
            [4, 6, 7, 8, 9],
            [multiplicity * (eq0_e510_d_n4), multiplicity * (eq0_e510_d_n6), multiplicity * (eq0_e510_d_n7), multiplicity * (eq0_e510_d_n8), multiplicity * (eq0_e510_d_n9)],
            [],
            [],
            1.0,
        );
        let (eq1_e517, eq1_e517_d_n4, eq1_e517_d_n6, eq1_e517_d_n7, eq1_e517_d_n8, eq1_e517_d_n9,) = {
    if (locals.var_guard1239 == 0.0) {
        let eq1_e515: f64 = (p.p14 * locals.var_idse);
        let eq1_e515_d_n4: f64 = (p.p14 * locals.var_idse_dn4);
        let eq1_e515_d_n6: f64 = (p.p14 * locals.var_idse_dn6);
        let eq1_e515_d_n7: f64 = (p.p14 * locals.var_idse_dn7);
        let eq1_e515_d_n8: f64 = (p.p14 * locals.var_idse_dn8);
        let eq1_e515_d_n9: f64 = (p.p14 * locals.var_idse_dn9);
        (eq1_e515, eq1_e515_d_n4, eq1_e515_d_n6, eq1_e515_d_n7, eq1_e515_d_n8, eq1_e515_d_n9,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq1_value: f64 = eq1_e517;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(7),
            Some(6),
            multiplicity * (eq1_value),
            [4, 6, 7, 8, 9],
            [multiplicity * (eq1_e517_d_n4), multiplicity * (eq1_e517_d_n6), multiplicity * (eq1_e517_d_n7), multiplicity * (eq1_e517_d_n8), multiplicity * (eq1_e517_d_n9)],
            [],
            [],
            1.0,
        );
        let eq2_e521: f64 = (locals.var_igidle - locals.var_igisle);
        let eq2_e521_d_n4: f64 = (locals.var_igidle_dn4 - locals.var_igisle_dn4);
        let eq2_e521_d_n6: f64 = (locals.var_igidle_dn6 - locals.var_igisle_dn6);
        let eq2_e521_d_n7: f64 = (locals.var_igidle_dn7 - locals.var_igisle_dn7);
        let eq2_e521_d_n8: f64 = (locals.var_igidle_dn8 - locals.var_igisle_dn8);
        let eq2_e521_d_n9: f64 = (locals.var_igidle_dn9 - locals.var_igisle_dn9);
        let eq2_e522: f64 = (p.p14 * eq2_e521);
        let eq2_e522_d_n4: f64 = (p.p14 * eq2_e521_d_n4);
        let eq2_e522_d_n6: f64 = (p.p14 * eq2_e521_d_n6);
        let eq2_e522_d_n7: f64 = (p.p14 * eq2_e521_d_n7);
        let eq2_e522_d_n8: f64 = (p.p14 * eq2_e521_d_n8);
        let eq2_e522_d_n9: f64 = (p.p14 * eq2_e521_d_n9);
        let eq2_value: f64 = eq2_e522;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(7),
            Some(6),
            multiplicity * (eq2_value),
            [4, 6, 7, 8, 9],
            [multiplicity * (eq2_e522_d_n4), multiplicity * (eq2_e522_d_n6), multiplicity * (eq2_e522_d_n7), multiplicity * (eq2_e522_d_n8), multiplicity * (eq2_e522_d_n9)],
            [],
            [],
            1.0,
        );
        let eq3_e525: f64 = (p.p14 * locals.var_igse);
        let eq3_e525_d_n4: f64 = (p.p14 * locals.var_igse_dn4);
        let eq3_e525_d_n6: f64 = (p.p14 * locals.var_igse_dn6);
        let eq3_e525_d_n7: f64 = (p.p14 * locals.var_igse_dn7);
        let eq3_e525_d_n8: f64 = (p.p14 * locals.var_igse_dn8);
        let eq3_e525_d_n9: f64 = (p.p14 * locals.var_igse_dn9);
        let eq3_value: f64 = eq3_e525;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(9),
            Some(6),
            multiplicity * (eq3_value),
            [4, 6, 7, 8, 9],
            [multiplicity * (eq3_e525_d_n4), multiplicity * (eq3_e525_d_n6), multiplicity * (eq3_e525_d_n7), multiplicity * (eq3_e525_d_n8), multiplicity * (eq3_e525_d_n9)],
            [],
            [],
            1.0,
        );
        let eq4_e528: f64 = (p.p14 * locals.var_igde);
        let eq4_e528_d_n4: f64 = (p.p14 * locals.var_igde_dn4);
        let eq4_e528_d_n6: f64 = (p.p14 * locals.var_igde_dn6);
        let eq4_e528_d_n7: f64 = (p.p14 * locals.var_igde_dn7);
        let eq4_e528_d_n8: f64 = (p.p14 * locals.var_igde_dn8);
        let eq4_e528_d_n9: f64 = (p.p14 * locals.var_igde_dn9);
        let eq4_value: f64 = eq4_e528;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(9),
            Some(7),
            multiplicity * (eq4_value),
            [4, 6, 7, 8, 9],
            [multiplicity * (eq4_e528_d_n4), multiplicity * (eq4_e528_d_n6), multiplicity * (eq4_e528_d_n7), multiplicity * (eq4_e528_d_n8), multiplicity * (eq4_e528_d_n9)],
            [],
            [],
            1.0,
        );
        let eq9_value: f64 = locals.var_ithpwre;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(4),
            None,
            multiplicity * (eq9_value),
            [4, 6, 7, 8, 9],
            [multiplicity * (locals.var_ithpwre_dn4), multiplicity * (locals.var_ithpwre_dn6), multiplicity * (locals.var_ithpwre_dn7), multiplicity * (locals.var_ithpwre_dn8), multiplicity * (locals.var_ithpwre_dn9)],
            [],
            [],
            1.0,
        );
        let eq23_e642: f64 = (locals.var_qg_wo_mult + locals.var_qb_wo_mult);
        let eq23_e642_d_n4: f64 = (locals.var_qg_wo_mult_dn4 + locals.var_qb_wo_mult_dn4);
        let eq23_e642_d_n6: f64 = (locals.var_qg_wo_mult_dn6 + locals.var_qb_wo_mult_dn6);
        let eq23_e642_d_n7: f64 = (locals.var_qg_wo_mult_dn7 + locals.var_qb_wo_mult_dn7);
        let eq23_e642_d_n8: f64 = (locals.var_qg_wo_mult_dn8 + locals.var_qb_wo_mult_dn8);
        let eq23_e642_d_n9: f64 = (locals.var_qg_wo_mult_dn9 + locals.var_qb_wo_mult_dn9);
        let eq23_e643: f64 = (locals.var_fracinv_i * eq23_e642);
        let eq23_e643_d_n4: f64 = (locals.var_fracinv_i * eq23_e642_d_n4);
        let eq23_e643_d_n6: f64 = (locals.var_fracinv_i * eq23_e642_d_n6);
        let eq23_e643_d_n7: f64 = (locals.var_fracinv_i * eq23_e642_d_n7);
        let eq23_e643_d_n8: f64 = (locals.var_fracinv_i * eq23_e642_d_n8);
        let eq23_e643_d_n9: f64 = (locals.var_fracinv_i * eq23_e642_d_n9);
        let eq23_e644: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, eq23_e643);
        let eq23_value: f64 = eq23_e644;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(10),
            Some(13),
            multiplicity * (eq23_value),
            [4, 6, 7, 8, 9],
            [multiplicity * ((eq23_e643_d_n4 * ddt_scale)), multiplicity * ((eq23_e643_d_n6 * ddt_scale)), multiplicity * ((eq23_e643_d_n7 * ddt_scale)), multiplicity * ((eq23_e643_d_n8 * ddt_scale)), multiplicity * ((eq23_e643_d_n9 * ddt_scale))],
            [],
            [],
            1.0,
        );
        let eq24_e647: f64 = (locals.var_itaueff * (nv10 - nv13));
        let eq24_e647_d_n4: f64 = (locals.var_itaueff_dn4 * (nv10 - nv13));
        let eq24_e647_d_n6: f64 = (locals.var_itaueff_dn6 * (nv10 - nv13));
        let eq24_e647_d_n7: f64 = (locals.var_itaueff_dn7 * (nv10 - nv13));
        let eq24_e647_d_n8: f64 = (locals.var_itaueff_dn8 * (nv10 - nv13));
        let eq24_e647_d_n9: f64 = (locals.var_itaueff_dn9 * (nv10 - nv13));
        let eq24_value: f64 = eq24_e647;
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(10),
            Some(13),
            multiplicity * (eq24_value),
            [4, 6, 7, 8, 9, 10, 13],
            [multiplicity * (eq24_e647_d_n4), multiplicity * (eq24_e647_d_n6), multiplicity * (eq24_e647_d_n7), multiplicity * (eq24_e647_d_n8), multiplicity * (eq24_e647_d_n9), multiplicity * (locals.var_itaueff), multiplicity * ((-locals.var_itaueff))],
            [],
            [],
            1.0,
        );
        let eq26_e653: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 2, locals.var_qd_wo_mult);
        let eq26_value: f64 = eq26_e653;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(12),
            Some(13),
            multiplicity * (eq26_value),
            [4, 6, 7, 8, 9],
            [multiplicity * ((locals.var_qd_wo_mult_dn4 * ddt_scale)), multiplicity * ((locals.var_qd_wo_mult_dn6 * ddt_scale)), multiplicity * ((locals.var_qd_wo_mult_dn7 * ddt_scale)), multiplicity * ((locals.var_qd_wo_mult_dn8 * ddt_scale)), multiplicity * ((locals.var_qd_wo_mult_dn9 * ddt_scale))],
            [],
            [],
            1.0,
        );
        let eq27_e656: f64 = (locals.var_itaueff * (nv12 - nv13));
        let eq27_e656_d_n4: f64 = (locals.var_itaueff_dn4 * (nv12 - nv13));
        let eq27_e656_d_n6: f64 = (locals.var_itaueff_dn6 * (nv12 - nv13));
        let eq27_e656_d_n7: f64 = (locals.var_itaueff_dn7 * (nv12 - nv13));
        let eq27_e656_d_n8: f64 = (locals.var_itaueff_dn8 * (nv12 - nv13));
        let eq27_e656_d_n9: f64 = (locals.var_itaueff_dn9 * (nv12 - nv13));
        let eq27_value: f64 = eq27_e656;
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(12),
            Some(13),
            multiplicity * (eq27_value),
            [4, 6, 7, 8, 9, 12, 13],
            [multiplicity * (eq27_e656_d_n4), multiplicity * (eq27_e656_d_n6), multiplicity * (eq27_e656_d_n7), multiplicity * (eq27_e656_d_n8), multiplicity * (eq27_e656_d_n9), multiplicity * (locals.var_itaueff), multiplicity * ((-locals.var_itaueff))],
            [],
            [],
            1.0,
        );
        let eq29_e662: f64 = (locals.var_kfracinv_i).sqrt();
        let eq29_e665: f64 = (1.0 - locals.var_fracinv_i);
        let eq29_e668: f64 = (locals.var_qg_wo_mult + locals.var_qb_wo_mult);
        let eq29_e669: f64 = (eq29_e665 * eq29_e668);
        let eq29_e669_d_n4: f64 = (eq29_e665 * eq23_e642_d_n4);
        let eq29_e669_d_n6: f64 = (eq29_e665 * eq23_e642_d_n6);
        let eq29_e669_d_n7: f64 = (eq29_e665 * eq23_e642_d_n7);
        let eq29_e669_d_n8: f64 = (eq29_e665 * eq23_e642_d_n8);
        let eq29_e669_d_n9: f64 = (eq29_e665 * eq23_e642_d_n9);
        let eq29_e670: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 4, eq29_e669);
        let eq29_e671: f64 = (eq29_e662 * eq29_e670);
        let eq29_e671_d_n4: f64 = (eq29_e662 * (eq29_e669_d_n4 * ddt_scale));
        let eq29_e671_d_n6: f64 = (eq29_e662 * (eq29_e669_d_n6 * ddt_scale));
        let eq29_e671_d_n7: f64 = (eq29_e662 * (eq29_e669_d_n7 * ddt_scale));
        let eq29_e671_d_n8: f64 = (eq29_e662 * (eq29_e669_d_n8 * ddt_scale));
        let eq29_e671_d_n9: f64 = (eq29_e662 * (eq29_e669_d_n9 * ddt_scale));
        let eq29_value: f64 = eq29_e671;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(11),
            Some(13),
            multiplicity * (eq29_value),
            [4, 6, 7, 8, 9],
            [multiplicity * (eq29_e671_d_n4), multiplicity * (eq29_e671_d_n6), multiplicity * (eq29_e671_d_n7), multiplicity * (eq29_e671_d_n8), multiplicity * (eq29_e671_d_n9)],
            [],
            [],
            1.0,
        );
        let eq30_e674: f64 = (locals.var_itaueff * (nv11 - nv13));
        let eq30_e674_d_n4: f64 = (locals.var_itaueff_dn4 * (nv11 - nv13));
        let eq30_e674_d_n6: f64 = (locals.var_itaueff_dn6 * (nv11 - nv13));
        let eq30_e674_d_n7: f64 = (locals.var_itaueff_dn7 * (nv11 - nv13));
        let eq30_e674_d_n8: f64 = (locals.var_itaueff_dn8 * (nv11 - nv13));
        let eq30_e674_d_n9: f64 = (locals.var_itaueff_dn9 * (nv11 - nv13));
        let eq30_value: f64 = eq30_e674;
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(11),
            Some(13),
            multiplicity * (eq30_value),
            [4, 6, 7, 8, 9, 11, 13],
            [multiplicity * (eq30_e674_d_n4), multiplicity * (eq30_e674_d_n6), multiplicity * (eq30_e674_d_n7), multiplicity * (eq30_e674_d_n8), multiplicity * (eq30_e674_d_n9), multiplicity * (locals.var_itaueff), multiplicity * ((-locals.var_itaueff))],
            [],
            [],
            1.0,
        );
        let eq31_e678: f64 = (1e-9 * (nv11 - nv13));
        let eq31_e679: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 5, eq31_e678);
        let eq31_e680: f64 = (locals.var_kfracinv_i * eq31_e679);
        let eq31_e680_d_n11: f64 = (locals.var_kfracinv_i * (1e-9 * ddt_scale));
        let eq31_e680_d_n13: f64 = (locals.var_kfracinv_i * ((-1e-9) * ddt_scale));
        let eq31_value: f64 = eq31_e680;
        stamper.stamp_current_node2_local(
            Some(11),
            Some(13),
            multiplicity * (eq31_value),
            11,
            multiplicity * (eq31_e680_d_n11),
            13,
            multiplicity * (eq31_e680_d_n13),
        );
        let eq32_e683: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 6, locals.var_qb);
        let eq32_e685: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 7, locals.var_qssub);
        let eq32_e686: f64 = (eq32_e683 + eq32_e685);
        let eq32_e686_d_n6: f64 = ((locals.var_qb_dn6 * ddt_scale) + (locals.var_qssub_dn6 * ddt_scale));
        let eq32_e686_d_n8: f64 = ((locals.var_qb_dn8 * ddt_scale) + (locals.var_qssub_dn8 * ddt_scale));
        let eq32_e688: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 8, locals.var_qbsif);
        let eq32_e689: f64 = (eq32_e686 + eq32_e688);
        let eq32_e689_d_n4: f64 = ((locals.var_qb_dn4 * ddt_scale) + (locals.var_qbsif_dn4 * ddt_scale));
        let eq32_e689_d_n6: f64 = (eq32_e686_d_n6 + (locals.var_qbsif_dn6 * ddt_scale));
        let eq32_e689_d_n7: f64 = ((locals.var_qb_dn7 * ddt_scale) + (locals.var_qbsif_dn7 * ddt_scale));
        let eq32_e689_d_n8: f64 = (eq32_e686_d_n8 + (locals.var_qbsif_dn8 * ddt_scale));
        let eq32_e689_d_n9: f64 = ((locals.var_qb_dn9 * ddt_scale) + (locals.var_qbsif_dn9 * ddt_scale));
        let eq32_e690: f64 = (p.p14 * eq32_e689);
        let eq32_e690_d_n4: f64 = (p.p14 * eq32_e689_d_n4);
        let eq32_e690_d_n6: f64 = (p.p14 * eq32_e689_d_n6);
        let eq32_e690_d_n7: f64 = (p.p14 * eq32_e689_d_n7);
        let eq32_e690_d_n8: f64 = (p.p14 * eq32_e689_d_n8);
        let eq32_e690_d_n9: f64 = (p.p14 * eq32_e689_d_n9);
        let eq32_value: f64 = eq32_e690;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(8),
            Some(6),
            multiplicity * (eq32_value),
            [4, 6, 7, 8, 9],
            [multiplicity * (eq32_e690_d_n4), multiplicity * (eq32_e690_d_n6), multiplicity * (eq32_e690_d_n7), multiplicity * (eq32_e690_d_n8), multiplicity * (eq32_e690_d_n9)],
            [],
            [],
            1.0,
        );
        let eq33_e693: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 9, locals.var_qgde);
        let eq33_e695: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 10, locals.var_qovd);
        let eq33_e696: f64 = (eq33_e693 + eq33_e695);
        let eq33_e696_d_n4: f64 = ((locals.var_qgde_dn4 * ddt_scale) + (locals.var_qovd_dn4 * ddt_scale));
        let eq33_e696_d_n6: f64 = ((locals.var_qgde_dn6 * ddt_scale) + (locals.var_qovd_dn6 * ddt_scale));
        let eq33_e696_d_n7: f64 = ((locals.var_qgde_dn7 * ddt_scale) + (locals.var_qovd_dn7 * ddt_scale));
        let eq33_e696_d_n8: f64 = ((locals.var_qgde_dn8 * ddt_scale) + (locals.var_qovd_dn8 * ddt_scale));
        let eq33_e696_d_n9: f64 = ((locals.var_qgde_dn9 * ddt_scale) + (locals.var_qovd_dn9 * ddt_scale));
        let eq33_e698: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 11, locals.var_qgdif);
        let eq33_e699: f64 = (eq33_e696 + eq33_e698);
        let eq33_e699_d_n4: f64 = (eq33_e696_d_n4 + (locals.var_qgdif_dn4 * ddt_scale));
        let eq33_e699_d_n6: f64 = (eq33_e696_d_n6 + (locals.var_qgdif_dn6 * ddt_scale));
        let eq33_e699_d_n7: f64 = (eq33_e696_d_n7 + (locals.var_qgdif_dn7 * ddt_scale));
        let eq33_e699_d_n8: f64 = (eq33_e696_d_n8 + (locals.var_qgdif_dn8 * ddt_scale));
        let eq33_e699_d_n9: f64 = (eq33_e696_d_n9 + (locals.var_qgdif_dn9 * ddt_scale));
        let eq33_e700: f64 = (p.p14 * eq33_e699);
        let eq33_e700_d_n4: f64 = (p.p14 * eq33_e699_d_n4);
        let eq33_e700_d_n6: f64 = (p.p14 * eq33_e699_d_n6);
        let eq33_e700_d_n7: f64 = (p.p14 * eq33_e699_d_n7);
        let eq33_e700_d_n8: f64 = (p.p14 * eq33_e699_d_n8);
        let eq33_e700_d_n9: f64 = (p.p14 * eq33_e699_d_n9);
        let eq33_value: f64 = eq33_e700;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(9),
            Some(7),
            multiplicity * (eq33_value),
            [4, 6, 7, 8, 9],
            [multiplicity * (eq33_e700_d_n4), multiplicity * (eq33_e700_d_n6), multiplicity * (eq33_e700_d_n7), multiplicity * (eq33_e700_d_n8), multiplicity * (eq33_e700_d_n9)],
            [],
            [],
            1.0,
        );
        let eq34_e703: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 12, locals.var_qdsub);
        let eq34_e705: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 13, locals.var_qbdif);
        let eq34_e706: f64 = (eq34_e703 + eq34_e705);
        let eq34_e706_d_n6: f64 = ((locals.var_qdsub_dn6 * ddt_scale) + (locals.var_qbdif_dn6 * ddt_scale));
        let eq34_e706_d_n7: f64 = ((locals.var_qdsub_dn7 * ddt_scale) + (locals.var_qbdif_dn7 * ddt_scale));
        let eq34_e706_d_n8: f64 = ((locals.var_qdsub_dn8 * ddt_scale) + (locals.var_qbdif_dn8 * ddt_scale));
        let eq34_e707: f64 = (p.p14 * eq34_e706);
        let eq34_e707_d_n4: f64 = (p.p14 * (locals.var_qbdif_dn4 * ddt_scale));
        let eq34_e707_d_n6: f64 = (p.p14 * eq34_e706_d_n6);
        let eq34_e707_d_n7: f64 = (p.p14 * eq34_e706_d_n7);
        let eq34_e707_d_n8: f64 = (p.p14 * eq34_e706_d_n8);
        let eq34_e707_d_n9: f64 = (p.p14 * (locals.var_qbdif_dn9 * ddt_scale));
        let eq34_value: f64 = eq34_e707;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(8),
            Some(7),
            multiplicity * (eq34_value),
            [4, 6, 7, 8, 9],
            [multiplicity * (eq34_e707_d_n4), multiplicity * (eq34_e707_d_n6), multiplicity * (eq34_e707_d_n7), multiplicity * (eq34_e707_d_n8), multiplicity * (eq34_e707_d_n9)],
            [],
            [],
            1.0,
        );
        let eq35_e710: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 14, locals.var_qgbe);
        let eq35_e711: f64 = (p.p14 * eq35_e710);
        let eq35_e711_d_n4: f64 = (p.p14 * (locals.var_qgbe_dn4 * ddt_scale));
        let eq35_e711_d_n6: f64 = (p.p14 * (locals.var_qgbe_dn6 * ddt_scale));
        let eq35_e711_d_n7: f64 = (p.p14 * (locals.var_qgbe_dn7 * ddt_scale));
        let eq35_e711_d_n8: f64 = (p.p14 * (locals.var_qgbe_dn8 * ddt_scale));
        let eq35_e711_d_n9: f64 = (p.p14 * (locals.var_qgbe_dn9 * ddt_scale));
        let eq35_value: f64 = eq35_e711;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(9),
            Some(8),
            multiplicity * (eq35_value),
            [4, 6, 7, 8, 9],
            [multiplicity * (eq35_e711_d_n4), multiplicity * (eq35_e711_d_n6), multiplicity * (eq35_e711_d_n7), multiplicity * (eq35_e711_d_n8), multiplicity * (eq35_e711_d_n9)],
            [],
            [],
            1.0,
        );
        let eq36_e714: f64 = (-locals.var_itaueff);
        let eq36_e716: f64 = (eq36_e714 * p.p32);
        let eq36_e716_d_n4: f64 = ((-locals.var_itaueff_dn4) * p.p32);
        let eq36_e716_d_n6: f64 = ((-locals.var_itaueff_dn6) * p.p32);
        let eq36_e716_d_n7: f64 = ((-locals.var_itaueff_dn7) * p.p32);
        let eq36_e716_d_n8: f64 = ((-locals.var_itaueff_dn8) * p.p32);
        let eq36_e716_d_n9: f64 = ((-locals.var_itaueff_dn9) * p.p32);
        let eq36_e718: f64 = (eq36_e716 * locals.var_mult_i_int);
        let eq36_e718_d_n4: f64 = (eq36_e716_d_n4 * locals.var_mult_i_int);
        let eq36_e718_d_n6: f64 = (eq36_e716_d_n6 * locals.var_mult_i_int);
        let eq36_e718_d_n7: f64 = (eq36_e716_d_n7 * locals.var_mult_i_int);
        let eq36_e718_d_n8: f64 = (eq36_e716_d_n8 * locals.var_mult_i_int);
        let eq36_e718_d_n9: f64 = (eq36_e716_d_n9 * locals.var_mult_i_int);
        let eq36_e722: f64 = (locals.var_kfracinv_i).sqrt();
        let __rspice_inv_cse_0: f64 = 1.0 / eq36_e722;
        let eq36_e723: f64 = ((nv11 - nv13) * __rspice_inv_cse_0);
        let eq36_e723_d_n11: f64 = (1.0 * __rspice_inv_cse_0);
        let eq36_e723_d_n13: f64 = ((-1.0) * __rspice_inv_cse_0);
        let eq36_e724: f64 = ((nv10 - nv13) + eq36_e723);
        let eq36_e724_d_n13: f64 = (-1.0 + eq36_e723_d_n13);
        let eq36_e725: f64 = (eq36_e718 * eq36_e724);
        let eq36_e725_d_n4: f64 = (eq36_e718_d_n4 * eq36_e724);
        let eq36_e725_d_n6: f64 = (eq36_e718_d_n6 * eq36_e724);
        let eq36_e725_d_n7: f64 = (eq36_e718_d_n7 * eq36_e724);
        let eq36_e725_d_n8: f64 = (eq36_e718_d_n8 * eq36_e724);
        let eq36_e725_d_n9: f64 = (eq36_e718_d_n9 * eq36_e724);
        let eq36_e725_d_n11: f64 = (eq36_e718 * eq36_e723_d_n11);
        let eq36_e725_d_n13: f64 = (eq36_e718 * eq36_e724_d_n13);
        let eq36_e727: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 15, locals.var_qb);
        let eq36_e728: f64 = (eq36_e725 - eq36_e727);
        let eq36_e728_d_n4: f64 = (eq36_e725_d_n4 - (locals.var_qb_dn4 * ddt_scale));
        let eq36_e728_d_n6: f64 = (eq36_e725_d_n6 - (locals.var_qb_dn6 * ddt_scale));
        let eq36_e728_d_n7: f64 = (eq36_e725_d_n7 - (locals.var_qb_dn7 * ddt_scale));
        let eq36_e728_d_n8: f64 = (eq36_e725_d_n8 - (locals.var_qb_dn8 * ddt_scale));
        let eq36_e728_d_n9: f64 = (eq36_e725_d_n9 - (locals.var_qb_dn9 * ddt_scale));
        let eq36_e730: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 16, locals.var_qgse);
        let eq36_e731: f64 = (eq36_e728 + eq36_e730);
        let eq36_e731_d_n4: f64 = (eq36_e728_d_n4 + (locals.var_qgse_dn4 * ddt_scale));
        let eq36_e731_d_n6: f64 = (eq36_e728_d_n6 + (locals.var_qgse_dn6 * ddt_scale));
        let eq36_e731_d_n7: f64 = (eq36_e728_d_n7 + (locals.var_qgse_dn7 * ddt_scale));
        let eq36_e731_d_n8: f64 = (eq36_e728_d_n8 + (locals.var_qgse_dn8 * ddt_scale));
        let eq36_e731_d_n9: f64 = (eq36_e728_d_n9 + (locals.var_qgse_dn9 * ddt_scale));
        let eq36_e733: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 17, locals.var_qovs);
        let eq36_e734: f64 = (eq36_e731 + eq36_e733);
        let eq36_e734_d_n4: f64 = (eq36_e731_d_n4 + (locals.var_qovs_dn4 * ddt_scale));
        let eq36_e734_d_n6: f64 = (eq36_e731_d_n6 + (locals.var_qovs_dn6 * ddt_scale));
        let eq36_e734_d_n7: f64 = (eq36_e731_d_n7 + (locals.var_qovs_dn7 * ddt_scale));
        let eq36_e734_d_n8: f64 = (eq36_e731_d_n8 + (locals.var_qovs_dn8 * ddt_scale));
        let eq36_e734_d_n9: f64 = (eq36_e731_d_n9 + (locals.var_qovs_dn9 * ddt_scale));
        let eq36_e736: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 18, locals.var_qgsif);
        let eq36_e737: f64 = (eq36_e734 + eq36_e736);
        let eq36_e737_d_n4: f64 = (eq36_e734_d_n4 + (locals.var_qgsif_dn4 * ddt_scale));
        let eq36_e737_d_n6: f64 = (eq36_e734_d_n6 + (locals.var_qgsif_dn6 * ddt_scale));
        let eq36_e737_d_n7: f64 = (eq36_e734_d_n7 + (locals.var_qgsif_dn7 * ddt_scale));
        let eq36_e737_d_n8: f64 = (eq36_e734_d_n8 + (locals.var_qgsif_dn8 * ddt_scale));
        let eq36_e737_d_n9: f64 = (eq36_e734_d_n9 + (locals.var_qgsif_dn9 * ddt_scale));
        let eq36_e738: f64 = (p.p14 * eq36_e737);
        let eq36_e738_d_n4: f64 = (p.p14 * eq36_e737_d_n4);
        let eq36_e738_d_n6: f64 = (p.p14 * eq36_e737_d_n6);
        let eq36_e738_d_n7: f64 = (p.p14 * eq36_e737_d_n7);
        let eq36_e738_d_n8: f64 = (p.p14 * eq36_e737_d_n8);
        let eq36_e738_d_n9: f64 = (p.p14 * eq36_e737_d_n9);
        let eq36_e738_d_n10: f64 = (p.p14 * eq36_e718);
        let eq36_e738_d_n11: f64 = (p.p14 * eq36_e725_d_n11);
        let eq36_e738_d_n13: f64 = (p.p14 * eq36_e725_d_n13);
        let eq36_value: f64 = eq36_e738;
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(9),
            Some(6),
            multiplicity * (eq36_value),
            [4, 6, 7, 8, 9, 10, 11, 13],
            [multiplicity * (eq36_e738_d_n4), multiplicity * (eq36_e738_d_n6), multiplicity * (eq36_e738_d_n7), multiplicity * (eq36_e738_d_n8), multiplicity * (eq36_e738_d_n9), multiplicity * (eq36_e738_d_n10), multiplicity * (eq36_e738_d_n11), multiplicity * (eq36_e738_d_n13)],
            [],
            [],
            1.0,
        );
        let eq37_e741: f64 = (-locals.var_itaueff);
        let eq37_e743: f64 = (eq37_e741 * p.p31);
        let eq37_e743_d_n4: f64 = ((-locals.var_itaueff_dn4) * p.p31);
        let eq37_e743_d_n6: f64 = ((-locals.var_itaueff_dn6) * p.p31);
        let eq37_e743_d_n7: f64 = ((-locals.var_itaueff_dn7) * p.p31);
        let eq37_e743_d_n8: f64 = ((-locals.var_itaueff_dn8) * p.p31);
        let eq37_e743_d_n9: f64 = ((-locals.var_itaueff_dn9) * p.p31);
        let eq37_e745: f64 = (eq37_e743 * locals.var_mult_i_int);
        let eq37_e745_d_n4: f64 = (eq37_e743_d_n4 * locals.var_mult_i_int);
        let eq37_e745_d_n6: f64 = (eq37_e743_d_n6 * locals.var_mult_i_int);
        let eq37_e745_d_n7: f64 = (eq37_e743_d_n7 * locals.var_mult_i_int);
        let eq37_e745_d_n8: f64 = (eq37_e743_d_n8 * locals.var_mult_i_int);
        let eq37_e745_d_n9: f64 = (eq37_e743_d_n9 * locals.var_mult_i_int);
        let eq37_e747: f64 = (eq37_e745 * (nv12 - nv13));
        let eq37_e747_d_n4: f64 = (eq37_e745_d_n4 * (nv12 - nv13));
        let eq37_e747_d_n6: f64 = (eq37_e745_d_n6 * (nv12 - nv13));
        let eq37_e747_d_n7: f64 = (eq37_e745_d_n7 * (nv12 - nv13));
        let eq37_e747_d_n8: f64 = (eq37_e745_d_n8 * (nv12 - nv13));
        let eq37_e747_d_n9: f64 = (eq37_e745_d_n9 * (nv12 - nv13));
        let eq37_e749: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 19, locals.var_qdse);
        let eq37_e750: f64 = (eq37_e747 + eq37_e749);
        let eq37_e750_d_n6: f64 = (eq37_e747_d_n6 + (locals.var_qdse_dn6 * ddt_scale));
        let eq37_e750_d_n7: f64 = (eq37_e747_d_n7 + (locals.var_qdse_dn7 * ddt_scale));
        let eq37_e751: f64 = (p.p14 * eq37_e750);
        let eq37_e751_d_n4: f64 = (p.p14 * eq37_e747_d_n4);
        let eq37_e751_d_n6: f64 = (p.p14 * eq37_e750_d_n6);
        let eq37_e751_d_n7: f64 = (p.p14 * eq37_e750_d_n7);
        let eq37_e751_d_n8: f64 = (p.p14 * eq37_e747_d_n8);
        let eq37_e751_d_n9: f64 = (p.p14 * eq37_e747_d_n9);
        let eq37_e751_d_n12: f64 = (p.p14 * eq37_e745);
        let eq37_e751_d_n13: f64 = (p.p14 * (-eq37_e745));
        let eq37_value: f64 = eq37_e751;
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(7),
            Some(6),
            multiplicity * (eq37_value),
            [4, 6, 7, 8, 9, 12, 13],
            [multiplicity * (eq37_e751_d_n4), multiplicity * (eq37_e751_d_n6), multiplicity * (eq37_e751_d_n7), multiplicity * (eq37_e751_d_n8), multiplicity * (eq37_e751_d_n9), multiplicity * (eq37_e751_d_n12), multiplicity * (eq37_e751_d_n13)],
            [],
            [],
            1.0,
        );
        let eq40_e759: f64 = (locals.var_gsig * (nv5 - 0.0));
        let eq40_e759_d_n4: f64 = (locals.var_gsig_dn4 * (nv5 - 0.0));
        let eq40_e759_d_n6: f64 = (locals.var_gsig_dn6 * (nv5 - 0.0));
        let eq40_e759_d_n7: f64 = (locals.var_gsig_dn7 * (nv5 - 0.0));
        let eq40_e759_d_n8: f64 = (locals.var_gsig_dn8 * (nv5 - 0.0));
        let eq40_e759_d_n9: f64 = (locals.var_gsig_dn9 * (nv5 - 0.0));
        let eq40_value: f64 = eq40_e759;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(5),
            None,
            multiplicity * (eq40_value),
            [4, 5, 6, 7, 8, 9],
            [multiplicity * (eq40_e759_d_n4), multiplicity * (locals.var_gsig), multiplicity * (eq40_e759_d_n6), multiplicity * (eq40_e759_d_n7), multiplicity * (eq40_e759_d_n8), multiplicity * (eq40_e759_d_n9)],
            [],
            [],
            1.0,
        );
        let eq41_e762: f64 = (locals.var_cgeff * (nv5 - 0.0));
        let eq41_e762_d_n4: f64 = (locals.var_cgeff_dn4 * (nv5 - 0.0));
        let eq41_e762_d_n6: f64 = (locals.var_cgeff_dn6 * (nv5 - 0.0));
        let eq41_e762_d_n7: f64 = (locals.var_cgeff_dn7 * (nv5 - 0.0));
        let eq41_e762_d_n8: f64 = (locals.var_cgeff_dn8 * (nv5 - 0.0));
        let eq41_e762_d_n9: f64 = (locals.var_cgeff_dn9 * (nv5 - 0.0));
        let eq41_e763: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 21, eq41_e762);
        let eq41_value: f64 = eq41_e763;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(5),
            None,
            multiplicity * (eq41_value),
            [4, 5, 6, 7, 8, 9],
            [multiplicity * ((eq41_e762_d_n4 * ddt_scale)), multiplicity * ((locals.var_cgeff * ddt_scale)), multiplicity * ((eq41_e762_d_n6 * ddt_scale)), multiplicity * ((eq41_e762_d_n7 * ddt_scale)), multiplicity * ((eq41_e762_d_n8 * ddt_scale)), multiplicity * ((eq41_e762_d_n9 * ddt_scale))],
            [],
            [],
            1.0,
        );
    }

    pub(super) fn stamp_transient_equations_block_1(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
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
        let nv5 = ctx.node_voltage(nodes[5]);
        let eq42_e765: f64 = (-locals.var_csgeff);
        let eq42_e767: f64 = (eq42_e765 * (nv5 - 0.0));
        let eq42_e767_d_n4: f64 = ((-locals.var_csgeff_dn4) * (nv5 - 0.0));
        let eq42_e767_d_n6: f64 = ((-locals.var_csgeff_dn6) * (nv5 - 0.0));
        let eq42_e767_d_n7: f64 = ((-locals.var_csgeff_dn7) * (nv5 - 0.0));
        let eq42_e767_d_n8: f64 = ((-locals.var_csgeff_dn8) * (nv5 - 0.0));
        let eq42_e767_d_n9: f64 = ((-locals.var_csgeff_dn9) * (nv5 - 0.0));
        let eq42_e768: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 22, eq42_e767);
        let eq42_value: f64 = eq42_e768;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(9),
            Some(6),
            multiplicity * (eq42_value),
            [4, 5, 6, 7, 8, 9],
            [multiplicity * ((eq42_e767_d_n4 * ddt_scale)), multiplicity * ((eq42_e765 * ddt_scale)), multiplicity * ((eq42_e767_d_n6 * ddt_scale)), multiplicity * ((eq42_e767_d_n7 * ddt_scale)), multiplicity * ((eq42_e767_d_n8 * ddt_scale)), multiplicity * ((eq42_e767_d_n9 * ddt_scale))],
            [],
            [],
            1.0,
        );
        let eq43_e770: f64 = (-locals.var_cdgeff);
        let eq43_e772: f64 = (eq43_e770 * (nv5 - 0.0));
        let eq43_e772_d_n4: f64 = ((-locals.var_cdgeff_dn4) * (nv5 - 0.0));
        let eq43_e772_d_n6: f64 = ((-locals.var_cdgeff_dn6) * (nv5 - 0.0));
        let eq43_e772_d_n7: f64 = ((-locals.var_cdgeff_dn7) * (nv5 - 0.0));
        let eq43_e772_d_n8: f64 = ((-locals.var_cdgeff_dn8) * (nv5 - 0.0));
        let eq43_e772_d_n9: f64 = ((-locals.var_cdgeff_dn9) * (nv5 - 0.0));
        let eq43_e773: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 23, eq43_e772);
        let eq43_value: f64 = eq43_e773;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(9),
            Some(7),
            multiplicity * (eq43_value),
            [4, 5, 6, 7, 8, 9],
            [multiplicity * ((eq43_e772_d_n4 * ddt_scale)), multiplicity * ((eq43_e770 * ddt_scale)), multiplicity * ((eq43_e772_d_n6 * ddt_scale)), multiplicity * ((eq43_e772_d_n7 * ddt_scale)), multiplicity * ((eq43_e772_d_n8 * ddt_scale)), multiplicity * ((eq43_e772_d_n9 * ddt_scale))],
            [],
            [],
            1.0,
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
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv10 = ctx.node_voltage(nodes[10]);
        let nv11 = ctx.node_voltage(nodes[11]);
        let nv12 = ctx.node_voltage(nodes[12]);
        let nv13 = ctx.node_voltage(nodes[13]);
        let eq23_e642: f64 = (locals.var_qg_wo_mult + locals.var_qb_wo_mult);
        let eq23_e642_d_n4: f64 = (locals.var_qg_wo_mult_dn4 + locals.var_qb_wo_mult_dn4);
        let eq23_e642_d_n6: f64 = (locals.var_qg_wo_mult_dn6 + locals.var_qb_wo_mult_dn6);
        let eq23_e642_d_n7: f64 = (locals.var_qg_wo_mult_dn7 + locals.var_qb_wo_mult_dn7);
        let eq23_e642_d_n8: f64 = (locals.var_qg_wo_mult_dn8 + locals.var_qb_wo_mult_dn8);
        let eq23_e642_d_n9: f64 = (locals.var_qg_wo_mult_dn9 + locals.var_qb_wo_mult_dn9);
        let eq23_e643: f64 = (locals.var_fracinv_i * eq23_e642);
        let eq23_e643_d_n4: f64 = (locals.var_fracinv_i * eq23_e642_d_n4);
        let eq23_e643_d_n6: f64 = (locals.var_fracinv_i * eq23_e642_d_n6);
        let eq23_e643_d_n7: f64 = (locals.var_fracinv_i * eq23_e642_d_n7);
        let eq23_e643_d_n8: f64 = (locals.var_fracinv_i * eq23_e642_d_n8);
        let eq23_e643_d_n9: f64 = (locals.var_fracinv_i * eq23_e642_d_n9);
        let eq23_e644_q: f64 = eq23_e643;
        let eq23_reactive_node_derivatives: [f64; 14] = [0.0, 0.0, 0.0, 0.0, eq23_e643_d_n4, 0.0, eq23_e643_d_n6, eq23_e643_d_n7, eq23_e643_d_n8, eq23_e643_d_n9, 0.0, 0.0, 0.0, 0.0];
        let eq23_reactive_branch_derivatives: [f64; 4] = [0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[10]),
            Some(nodes[13]),
            nodes,
            &eq23_reactive_node_derivatives,
            branches,
            &eq23_reactive_branch_derivatives,
            multiplicity,
        );
        let eq26_e653_q: f64 = locals.var_qd_wo_mult;
        let eq26_reactive_node_derivatives: [f64; 14] = [0.0, 0.0, 0.0, 0.0, locals.var_qd_wo_mult_dn4, 0.0, locals.var_qd_wo_mult_dn6, locals.var_qd_wo_mult_dn7, locals.var_qd_wo_mult_dn8, locals.var_qd_wo_mult_dn9, 0.0, 0.0, 0.0, 0.0];
        let eq26_reactive_branch_derivatives: [f64; 4] = [0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[12]),
            Some(nodes[13]),
            nodes,
            &eq26_reactive_node_derivatives,
            branches,
            &eq26_reactive_branch_derivatives,
            multiplicity,
        );
        let eq29_e662: f64 = (locals.var_kfracinv_i).sqrt();
        let eq29_e665: f64 = (1.0 - locals.var_fracinv_i);
        let eq29_e668: f64 = (locals.var_qg_wo_mult + locals.var_qb_wo_mult);
        let eq29_e669: f64 = (eq29_e665 * eq29_e668);
        let eq29_e669_d_n4: f64 = (eq29_e665 * eq23_e642_d_n4);
        let eq29_e669_d_n6: f64 = (eq29_e665 * eq23_e642_d_n6);
        let eq29_e669_d_n7: f64 = (eq29_e665 * eq23_e642_d_n7);
        let eq29_e669_d_n8: f64 = (eq29_e665 * eq23_e642_d_n8);
        let eq29_e669_d_n9: f64 = (eq29_e665 * eq23_e642_d_n9);
        let eq29_e670_q: f64 = eq29_e669;
        let eq29_e671: f64 = (eq29_e662 * eq29_e669);
        let eq29_e671_d_n4: f64 = (eq29_e662 * eq29_e669_d_n4);
        let eq29_e671_d_n6: f64 = (eq29_e662 * eq29_e669_d_n6);
        let eq29_e671_d_n7: f64 = (eq29_e662 * eq29_e669_d_n7);
        let eq29_e671_d_n8: f64 = (eq29_e662 * eq29_e669_d_n8);
        let eq29_e671_d_n9: f64 = (eq29_e662 * eq29_e669_d_n9);
        let eq29_e671_q: f64 = (eq29_e662 * eq29_e670_q);
        let eq29_reactive_node_derivatives: [f64; 14] = [0.0, 0.0, 0.0, 0.0, eq29_e671_d_n4, 0.0, eq29_e671_d_n6, eq29_e671_d_n7, eq29_e671_d_n8, eq29_e671_d_n9, 0.0, 0.0, 0.0, 0.0];
        let eq29_reactive_branch_derivatives: [f64; 4] = [0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[11]),
            Some(nodes[13]),
            nodes,
            &eq29_reactive_node_derivatives,
            branches,
            &eq29_reactive_branch_derivatives,
            multiplicity,
        );
        let eq31_e678: f64 = (1e-9 * (nv11 - nv13));
        let eq31_e679_q: f64 = eq31_e678;
        let eq31_e680: f64 = (locals.var_kfracinv_i * eq31_e678);
        let eq31_e680_d_n11: f64 = (locals.var_kfracinv_i * 1e-9);
        let eq31_e680_d_n13: f64 = (locals.var_kfracinv_i * (-1e-9));
        let eq31_e680_q: f64 = (locals.var_kfracinv_i * eq31_e679_q);
        stamper.stamp_current_reactive_node2(
            Some(nodes[11]),
            Some(nodes[13]),
            nodes[11],
            multiplicity * (eq31_e680_d_n11),
            nodes[13],
            multiplicity * (eq31_e680_d_n13),
        );
        let eq32_e683_q: f64 = locals.var_qb;
        let eq32_e685_q: f64 = locals.var_qssub;
        let eq32_e686: f64 = (locals.var_qb + locals.var_qssub);
        let eq32_e686_d_n6: f64 = (locals.var_qb_dn6 + locals.var_qssub_dn6);
        let eq32_e686_d_n8: f64 = (locals.var_qb_dn8 + locals.var_qssub_dn8);
        let eq32_e686_q: f64 = (eq32_e683_q + eq32_e685_q);
        let eq32_e688_q: f64 = locals.var_qbsif;
        let eq32_e689: f64 = (eq32_e686 + locals.var_qbsif);
        let eq32_e689_d_n4: f64 = (locals.var_qb_dn4 + locals.var_qbsif_dn4);
        let eq32_e689_d_n6: f64 = (eq32_e686_d_n6 + locals.var_qbsif_dn6);
        let eq32_e689_d_n7: f64 = (locals.var_qb_dn7 + locals.var_qbsif_dn7);
        let eq32_e689_d_n8: f64 = (eq32_e686_d_n8 + locals.var_qbsif_dn8);
        let eq32_e689_d_n9: f64 = (locals.var_qb_dn9 + locals.var_qbsif_dn9);
        let eq32_e689_q: f64 = (eq32_e686_q + eq32_e688_q);
        let eq32_e690: f64 = (p.p14 * eq32_e689);
        let eq32_e690_d_n4: f64 = (p.p14 * eq32_e689_d_n4);
        let eq32_e690_d_n6: f64 = (p.p14 * eq32_e689_d_n6);
        let eq32_e690_d_n7: f64 = (p.p14 * eq32_e689_d_n7);
        let eq32_e690_d_n8: f64 = (p.p14 * eq32_e689_d_n8);
        let eq32_e690_d_n9: f64 = (p.p14 * eq32_e689_d_n9);
        let eq32_e690_q: f64 = (p.p14 * eq32_e689_q);
        let eq32_reactive_node_derivatives: [f64; 14] = [0.0, 0.0, 0.0, 0.0, eq32_e690_d_n4, 0.0, eq32_e690_d_n6, eq32_e690_d_n7, eq32_e690_d_n8, eq32_e690_d_n9, 0.0, 0.0, 0.0, 0.0];
        let eq32_reactive_branch_derivatives: [f64; 4] = [0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[6]),
            nodes,
            &eq32_reactive_node_derivatives,
            branches,
            &eq32_reactive_branch_derivatives,
            multiplicity,
        );
        let eq33_e693_q: f64 = locals.var_qgde;
        let eq33_e695_q: f64 = locals.var_qovd;
        let eq33_e696: f64 = (locals.var_qgde + locals.var_qovd);
        let eq33_e696_d_n4: f64 = (locals.var_qgde_dn4 + locals.var_qovd_dn4);
        let eq33_e696_d_n6: f64 = (locals.var_qgde_dn6 + locals.var_qovd_dn6);
        let eq33_e696_d_n7: f64 = (locals.var_qgde_dn7 + locals.var_qovd_dn7);
        let eq33_e696_d_n8: f64 = (locals.var_qgde_dn8 + locals.var_qovd_dn8);
        let eq33_e696_d_n9: f64 = (locals.var_qgde_dn9 + locals.var_qovd_dn9);
        let eq33_e696_q: f64 = (eq33_e693_q + eq33_e695_q);
        let eq33_e698_q: f64 = locals.var_qgdif;
        let eq33_e699: f64 = (eq33_e696 + locals.var_qgdif);
        let eq33_e699_d_n4: f64 = (eq33_e696_d_n4 + locals.var_qgdif_dn4);
        let eq33_e699_d_n6: f64 = (eq33_e696_d_n6 + locals.var_qgdif_dn6);
        let eq33_e699_d_n7: f64 = (eq33_e696_d_n7 + locals.var_qgdif_dn7);
        let eq33_e699_d_n8: f64 = (eq33_e696_d_n8 + locals.var_qgdif_dn8);
        let eq33_e699_d_n9: f64 = (eq33_e696_d_n9 + locals.var_qgdif_dn9);
        let eq33_e699_q: f64 = (eq33_e696_q + eq33_e698_q);
        let eq33_e700: f64 = (p.p14 * eq33_e699);
        let eq33_e700_d_n4: f64 = (p.p14 * eq33_e699_d_n4);
        let eq33_e700_d_n6: f64 = (p.p14 * eq33_e699_d_n6);
        let eq33_e700_d_n7: f64 = (p.p14 * eq33_e699_d_n7);
        let eq33_e700_d_n8: f64 = (p.p14 * eq33_e699_d_n8);
        let eq33_e700_d_n9: f64 = (p.p14 * eq33_e699_d_n9);
        let eq33_e700_q: f64 = (p.p14 * eq33_e699_q);
        let eq33_reactive_node_derivatives: [f64; 14] = [0.0, 0.0, 0.0, 0.0, eq33_e700_d_n4, 0.0, eq33_e700_d_n6, eq33_e700_d_n7, eq33_e700_d_n8, eq33_e700_d_n9, 0.0, 0.0, 0.0, 0.0];
        let eq33_reactive_branch_derivatives: [f64; 4] = [0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[7]),
            nodes,
            &eq33_reactive_node_derivatives,
            branches,
            &eq33_reactive_branch_derivatives,
            multiplicity,
        );
        let eq34_e703_q: f64 = locals.var_qdsub;
        let eq34_e705_q: f64 = locals.var_qbdif;
        let eq34_e706: f64 = (locals.var_qdsub + locals.var_qbdif);
        let eq34_e706_d_n6: f64 = (locals.var_qdsub_dn6 + locals.var_qbdif_dn6);
        let eq34_e706_d_n7: f64 = (locals.var_qdsub_dn7 + locals.var_qbdif_dn7);
        let eq34_e706_d_n8: f64 = (locals.var_qdsub_dn8 + locals.var_qbdif_dn8);
        let eq34_e706_q: f64 = (eq34_e703_q + eq34_e705_q);
        let eq34_e707: f64 = (p.p14 * eq34_e706);
        let eq34_e707_d_n4: f64 = (p.p14 * locals.var_qbdif_dn4);
        let eq34_e707_d_n6: f64 = (p.p14 * eq34_e706_d_n6);
        let eq34_e707_d_n7: f64 = (p.p14 * eq34_e706_d_n7);
        let eq34_e707_d_n8: f64 = (p.p14 * eq34_e706_d_n8);
        let eq34_e707_d_n9: f64 = (p.p14 * locals.var_qbdif_dn9);
        let eq34_e707_q: f64 = (p.p14 * eq34_e706_q);
        let eq34_reactive_node_derivatives: [f64; 14] = [0.0, 0.0, 0.0, 0.0, eq34_e707_d_n4, 0.0, eq34_e707_d_n6, eq34_e707_d_n7, eq34_e707_d_n8, eq34_e707_d_n9, 0.0, 0.0, 0.0, 0.0];
        let eq34_reactive_branch_derivatives: [f64; 4] = [0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[7]),
            nodes,
            &eq34_reactive_node_derivatives,
            branches,
            &eq34_reactive_branch_derivatives,
            multiplicity,
        );
        let eq35_e710_q: f64 = locals.var_qgbe;
        let eq35_e711: f64 = (p.p14 * locals.var_qgbe);
        let eq35_e711_d_n4: f64 = (p.p14 * locals.var_qgbe_dn4);
        let eq35_e711_d_n6: f64 = (p.p14 * locals.var_qgbe_dn6);
        let eq35_e711_d_n7: f64 = (p.p14 * locals.var_qgbe_dn7);
        let eq35_e711_d_n8: f64 = (p.p14 * locals.var_qgbe_dn8);
        let eq35_e711_d_n9: f64 = (p.p14 * locals.var_qgbe_dn9);
        let eq35_e711_q: f64 = (p.p14 * eq35_e710_q);
        let eq35_reactive_node_derivatives: [f64; 14] = [0.0, 0.0, 0.0, 0.0, eq35_e711_d_n4, 0.0, eq35_e711_d_n6, eq35_e711_d_n7, eq35_e711_d_n8, eq35_e711_d_n9, 0.0, 0.0, 0.0, 0.0];
        let eq35_reactive_branch_derivatives: [f64; 4] = [0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[8]),
            nodes,
            &eq35_reactive_node_derivatives,
            branches,
            &eq35_reactive_branch_derivatives,
            multiplicity,
        );
        let eq36_e714: f64 = (-locals.var_itaueff);
        let eq36_e716: f64 = (eq36_e714 * p.p32);
        let eq36_e716_d_n4: f64 = ((-locals.var_itaueff_dn4) * p.p32);
        let eq36_e716_d_n6: f64 = ((-locals.var_itaueff_dn6) * p.p32);
        let eq36_e716_d_n7: f64 = ((-locals.var_itaueff_dn7) * p.p32);
        let eq36_e716_d_n8: f64 = ((-locals.var_itaueff_dn8) * p.p32);
        let eq36_e716_d_n9: f64 = ((-locals.var_itaueff_dn9) * p.p32);
        let eq36_e718: f64 = (eq36_e716 * locals.var_mult_i_int);
        let eq36_e718_d_n4: f64 = (eq36_e716_d_n4 * locals.var_mult_i_int);
        let eq36_e718_d_n6: f64 = (eq36_e716_d_n6 * locals.var_mult_i_int);
        let eq36_e718_d_n7: f64 = (eq36_e716_d_n7 * locals.var_mult_i_int);
        let eq36_e718_d_n8: f64 = (eq36_e716_d_n8 * locals.var_mult_i_int);
        let eq36_e718_d_n9: f64 = (eq36_e716_d_n9 * locals.var_mult_i_int);
        let eq36_e722: f64 = (locals.var_kfracinv_i).sqrt();
        let __rspice_inv_cse_0: f64 = 1.0 / eq36_e722;
        let eq36_e723: f64 = ((nv11 - nv13) * __rspice_inv_cse_0);
        let eq36_e723_d_n11: f64 = (1.0 * __rspice_inv_cse_0);
        let eq36_e723_d_n13: f64 = ((-1.0) * __rspice_inv_cse_0);
        let eq36_e724: f64 = ((nv10 - nv13) + eq36_e723);
        let eq36_e724_d_n13: f64 = (-1.0 + eq36_e723_d_n13);
        let eq36_e725: f64 = (eq36_e718 * eq36_e724);
        let eq36_e725_d_n4: f64 = (eq36_e718_d_n4 * eq36_e724);
        let eq36_e725_d_n6: f64 = (eq36_e718_d_n6 * eq36_e724);
        let eq36_e725_d_n7: f64 = (eq36_e718_d_n7 * eq36_e724);
        let eq36_e725_d_n8: f64 = (eq36_e718_d_n8 * eq36_e724);
        let eq36_e725_d_n9: f64 = (eq36_e718_d_n9 * eq36_e724);
        let eq36_e725_d_n11: f64 = (eq36_e718 * eq36_e723_d_n11);
        let eq36_e725_d_n13: f64 = (eq36_e718 * eq36_e724_d_n13);
        let eq36_e727_q: f64 = locals.var_qb;
        let eq36_e728: f64 = (eq36_e725 - locals.var_qb);
        let eq36_e728_d_n4: f64 = (eq36_e725_d_n4 - locals.var_qb_dn4);
        let eq36_e728_d_n6: f64 = (eq36_e725_d_n6 - locals.var_qb_dn6);
        let eq36_e728_d_n7: f64 = (eq36_e725_d_n7 - locals.var_qb_dn7);
        let eq36_e728_d_n8: f64 = (eq36_e725_d_n8 - locals.var_qb_dn8);
        let eq36_e728_d_n9: f64 = (eq36_e725_d_n9 - locals.var_qb_dn9);
        let eq36_e728_q: f64 = (-eq36_e727_q);
        let eq36_e730_q: f64 = locals.var_qgse;
        let eq36_e731: f64 = (eq36_e728 + locals.var_qgse);
        let eq36_e731_d_n4: f64 = (eq36_e728_d_n4 + locals.var_qgse_dn4);
        let eq36_e731_d_n6: f64 = (eq36_e728_d_n6 + locals.var_qgse_dn6);
        let eq36_e731_d_n7: f64 = (eq36_e728_d_n7 + locals.var_qgse_dn7);
        let eq36_e731_d_n8: f64 = (eq36_e728_d_n8 + locals.var_qgse_dn8);
        let eq36_e731_d_n9: f64 = (eq36_e728_d_n9 + locals.var_qgse_dn9);
        let eq36_e731_q: f64 = (eq36_e728_q + eq36_e730_q);
        let eq36_e731_q_d_n4: f64 = ((-locals.var_qb_dn4) + locals.var_qgse_dn4);
        let eq36_e731_q_d_n6: f64 = ((-locals.var_qb_dn6) + locals.var_qgse_dn6);
        let eq36_e731_q_d_n7: f64 = ((-locals.var_qb_dn7) + locals.var_qgse_dn7);
        let eq36_e731_q_d_n8: f64 = ((-locals.var_qb_dn8) + locals.var_qgse_dn8);
        let eq36_e731_q_d_n9: f64 = ((-locals.var_qb_dn9) + locals.var_qgse_dn9);
        let eq36_e733_q: f64 = locals.var_qovs;
        let eq36_e734: f64 = (eq36_e731 + locals.var_qovs);
        let eq36_e734_d_n4: f64 = (eq36_e731_d_n4 + locals.var_qovs_dn4);
        let eq36_e734_d_n6: f64 = (eq36_e731_d_n6 + locals.var_qovs_dn6);
        let eq36_e734_d_n7: f64 = (eq36_e731_d_n7 + locals.var_qovs_dn7);
        let eq36_e734_d_n8: f64 = (eq36_e731_d_n8 + locals.var_qovs_dn8);
        let eq36_e734_d_n9: f64 = (eq36_e731_d_n9 + locals.var_qovs_dn9);
        let eq36_e734_q: f64 = (eq36_e731_q + eq36_e733_q);
        let eq36_e734_q_d_n4: f64 = (eq36_e731_q_d_n4 + locals.var_qovs_dn4);
        let eq36_e734_q_d_n6: f64 = (eq36_e731_q_d_n6 + locals.var_qovs_dn6);
        let eq36_e734_q_d_n7: f64 = (eq36_e731_q_d_n7 + locals.var_qovs_dn7);
        let eq36_e734_q_d_n8: f64 = (eq36_e731_q_d_n8 + locals.var_qovs_dn8);
        let eq36_e734_q_d_n9: f64 = (eq36_e731_q_d_n9 + locals.var_qovs_dn9);
        let eq36_e736_q: f64 = locals.var_qgsif;
        let eq36_e737: f64 = (eq36_e734 + locals.var_qgsif);
        let eq36_e737_d_n4: f64 = (eq36_e734_d_n4 + locals.var_qgsif_dn4);
        let eq36_e737_d_n6: f64 = (eq36_e734_d_n6 + locals.var_qgsif_dn6);
        let eq36_e737_d_n7: f64 = (eq36_e734_d_n7 + locals.var_qgsif_dn7);
        let eq36_e737_d_n8: f64 = (eq36_e734_d_n8 + locals.var_qgsif_dn8);
        let eq36_e737_d_n9: f64 = (eq36_e734_d_n9 + locals.var_qgsif_dn9);
        let eq36_e737_q: f64 = (eq36_e734_q + eq36_e736_q);
        let eq36_e737_q_d_n4: f64 = (eq36_e734_q_d_n4 + locals.var_qgsif_dn4);
        let eq36_e737_q_d_n6: f64 = (eq36_e734_q_d_n6 + locals.var_qgsif_dn6);
        let eq36_e737_q_d_n7: f64 = (eq36_e734_q_d_n7 + locals.var_qgsif_dn7);
        let eq36_e737_q_d_n8: f64 = (eq36_e734_q_d_n8 + locals.var_qgsif_dn8);
        let eq36_e737_q_d_n9: f64 = (eq36_e734_q_d_n9 + locals.var_qgsif_dn9);
        let eq36_e738: f64 = (p.p14 * eq36_e737);
        let eq36_e738_d_n4: f64 = (p.p14 * eq36_e737_d_n4);
        let eq36_e738_d_n6: f64 = (p.p14 * eq36_e737_d_n6);
        let eq36_e738_d_n7: f64 = (p.p14 * eq36_e737_d_n7);
        let eq36_e738_d_n8: f64 = (p.p14 * eq36_e737_d_n8);
        let eq36_e738_d_n9: f64 = (p.p14 * eq36_e737_d_n9);
        let eq36_e738_d_n10: f64 = (p.p14 * eq36_e718);
        let eq36_e738_d_n11: f64 = (p.p14 * eq36_e725_d_n11);
        let eq36_e738_d_n13: f64 = (p.p14 * eq36_e725_d_n13);
        let eq36_e738_q: f64 = (p.p14 * eq36_e737_q);
        let eq36_e738_q_d_n4: f64 = (p.p14 * eq36_e737_q_d_n4);
        let eq36_e738_q_d_n6: f64 = (p.p14 * eq36_e737_q_d_n6);
        let eq36_e738_q_d_n7: f64 = (p.p14 * eq36_e737_q_d_n7);
        let eq36_e738_q_d_n8: f64 = (p.p14 * eq36_e737_q_d_n8);
        let eq36_e738_q_d_n9: f64 = (p.p14 * eq36_e737_q_d_n9);
        let eq36_reactive_node_derivatives: [f64; 14] = [0.0, 0.0, 0.0, 0.0, eq36_e738_q_d_n4, 0.0, eq36_e738_q_d_n6, eq36_e738_q_d_n7, eq36_e738_q_d_n8, eq36_e738_q_d_n9, 0.0, 0.0, 0.0, 0.0];
        let eq36_reactive_branch_derivatives: [f64; 4] = [0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[6]),
            nodes,
            &eq36_reactive_node_derivatives,
            branches,
            &eq36_reactive_branch_derivatives,
            multiplicity,
        );
        let eq37_e741: f64 = (-locals.var_itaueff);
        let eq37_e743: f64 = (eq37_e741 * p.p31);
        let eq37_e743_d_n4: f64 = ((-locals.var_itaueff_dn4) * p.p31);
        let eq37_e743_d_n6: f64 = ((-locals.var_itaueff_dn6) * p.p31);
        let eq37_e743_d_n7: f64 = ((-locals.var_itaueff_dn7) * p.p31);
        let eq37_e743_d_n8: f64 = ((-locals.var_itaueff_dn8) * p.p31);
        let eq37_e743_d_n9: f64 = ((-locals.var_itaueff_dn9) * p.p31);
        let eq37_e745: f64 = (eq37_e743 * locals.var_mult_i_int);
        let eq37_e745_d_n4: f64 = (eq37_e743_d_n4 * locals.var_mult_i_int);
        let eq37_e745_d_n6: f64 = (eq37_e743_d_n6 * locals.var_mult_i_int);
        let eq37_e745_d_n7: f64 = (eq37_e743_d_n7 * locals.var_mult_i_int);
        let eq37_e745_d_n8: f64 = (eq37_e743_d_n8 * locals.var_mult_i_int);
        let eq37_e745_d_n9: f64 = (eq37_e743_d_n9 * locals.var_mult_i_int);
        let eq37_e747: f64 = (eq37_e745 * (nv12 - nv13));
        let eq37_e747_d_n4: f64 = (eq37_e745_d_n4 * (nv12 - nv13));
        let eq37_e747_d_n6: f64 = (eq37_e745_d_n6 * (nv12 - nv13));
        let eq37_e747_d_n7: f64 = (eq37_e745_d_n7 * (nv12 - nv13));
        let eq37_e747_d_n8: f64 = (eq37_e745_d_n8 * (nv12 - nv13));
        let eq37_e747_d_n9: f64 = (eq37_e745_d_n9 * (nv12 - nv13));
        let eq37_e749_q: f64 = locals.var_qdse;
        let eq37_e750: f64 = (eq37_e747 + locals.var_qdse);
        let eq37_e750_d_n6: f64 = (eq37_e747_d_n6 + locals.var_qdse_dn6);
        let eq37_e750_d_n7: f64 = (eq37_e747_d_n7 + locals.var_qdse_dn7);
        let eq37_e750_q: f64 = eq37_e749_q;
        let eq37_e751: f64 = (p.p14 * eq37_e750);
        let eq37_e751_d_n4: f64 = (p.p14 * eq37_e747_d_n4);
        let eq37_e751_d_n6: f64 = (p.p14 * eq37_e750_d_n6);
        let eq37_e751_d_n7: f64 = (p.p14 * eq37_e750_d_n7);
        let eq37_e751_d_n8: f64 = (p.p14 * eq37_e747_d_n8);
        let eq37_e751_d_n9: f64 = (p.p14 * eq37_e747_d_n9);
        let eq37_e751_d_n12: f64 = (p.p14 * eq37_e745);
        let eq37_e751_d_n13: f64 = (p.p14 * (-eq37_e745));
        let eq37_e751_q: f64 = (p.p14 * eq37_e750_q);
        let eq37_e751_q_d_n6: f64 = (p.p14 * locals.var_qdse_dn6);
        let eq37_e751_q_d_n7: f64 = (p.p14 * locals.var_qdse_dn7);
        stamper.stamp_current_reactive_node2(
            Some(nodes[7]),
            Some(nodes[6]),
            nodes[6],
            multiplicity * (eq37_e751_q_d_n6),
            nodes[7],
            multiplicity * (eq37_e751_q_d_n7),
        );
        let eq41_e762: f64 = (locals.var_cgeff * (nv5 - 0.0));
        let eq41_e762_d_n4: f64 = (locals.var_cgeff_dn4 * (nv5 - 0.0));
        let eq41_e762_d_n6: f64 = (locals.var_cgeff_dn6 * (nv5 - 0.0));
        let eq41_e762_d_n7: f64 = (locals.var_cgeff_dn7 * (nv5 - 0.0));
        let eq41_e762_d_n8: f64 = (locals.var_cgeff_dn8 * (nv5 - 0.0));
        let eq41_e762_d_n9: f64 = (locals.var_cgeff_dn9 * (nv5 - 0.0));
        let eq41_e763_q: f64 = eq41_e762;
        let eq41_reactive_node_derivatives: [f64; 14] = [0.0, 0.0, 0.0, 0.0, eq41_e762_d_n4, locals.var_cgeff, eq41_e762_d_n6, eq41_e762_d_n7, eq41_e762_d_n8, eq41_e762_d_n9, 0.0, 0.0, 0.0, 0.0];
        let eq41_reactive_branch_derivatives: [f64; 4] = [0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            None,
            nodes,
            &eq41_reactive_node_derivatives,
            branches,
            &eq41_reactive_branch_derivatives,
            multiplicity,
        );
        let eq42_e765: f64 = (-locals.var_csgeff);
        let eq42_e767: f64 = (eq42_e765 * (nv5 - 0.0));
        let eq42_e767_d_n4: f64 = ((-locals.var_csgeff_dn4) * (nv5 - 0.0));
        let eq42_e767_d_n6: f64 = ((-locals.var_csgeff_dn6) * (nv5 - 0.0));
        let eq42_e767_d_n7: f64 = ((-locals.var_csgeff_dn7) * (nv5 - 0.0));
        let eq42_e767_d_n8: f64 = ((-locals.var_csgeff_dn8) * (nv5 - 0.0));
        let eq42_e767_d_n9: f64 = ((-locals.var_csgeff_dn9) * (nv5 - 0.0));
        let eq42_e768_q: f64 = eq42_e767;
        let eq42_reactive_node_derivatives: [f64; 14] = [0.0, 0.0, 0.0, 0.0, eq42_e767_d_n4, eq42_e765, eq42_e767_d_n6, eq42_e767_d_n7, eq42_e767_d_n8, eq42_e767_d_n9, 0.0, 0.0, 0.0, 0.0];
        let eq42_reactive_branch_derivatives: [f64; 4] = [0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[6]),
            nodes,
            &eq42_reactive_node_derivatives,
            branches,
            &eq42_reactive_branch_derivatives,
            multiplicity,
        );
        let eq43_e770: f64 = (-locals.var_cdgeff);
        let eq43_e772: f64 = (eq43_e770 * (nv5 - 0.0));
        let eq43_e772_d_n4: f64 = ((-locals.var_cdgeff_dn4) * (nv5 - 0.0));
        let eq43_e772_d_n6: f64 = ((-locals.var_cdgeff_dn6) * (nv5 - 0.0));
        let eq43_e772_d_n7: f64 = ((-locals.var_cdgeff_dn7) * (nv5 - 0.0));
        let eq43_e772_d_n8: f64 = ((-locals.var_cdgeff_dn8) * (nv5 - 0.0));
        let eq43_e772_d_n9: f64 = ((-locals.var_cdgeff_dn9) * (nv5 - 0.0));
        let eq43_e773_q: f64 = eq43_e772;
        let eq43_reactive_node_derivatives: [f64; 14] = [0.0, 0.0, 0.0, 0.0, eq43_e772_d_n4, eq43_e770, eq43_e772_d_n6, eq43_e772_d_n7, eq43_e772_d_n8, eq43_e772_d_n9, 0.0, 0.0, 0.0, 0.0];
        let eq43_reactive_branch_derivatives: [f64; 4] = [0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[7]),
            nodes,
            &eq43_reactive_node_derivatives,
            branches,
            &eq43_reactive_branch_derivatives,
            multiplicity,
        );
    }
}
