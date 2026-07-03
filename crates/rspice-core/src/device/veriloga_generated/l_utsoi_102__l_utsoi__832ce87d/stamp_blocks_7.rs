#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    pub(super) fn stamp_reactive_block_54(
        locals: &mut StampLocals,
    ) {
        if ((locals.var_guard1080 != 0.0) && (locals.var_guard1187 == 0.0)) {
            let assign37680_e42492: f64 = (locals.var_q_temp3__blk816 * locals.var_q_expnum__blk837);
            let assign37680_e42494: f64 = (assign37680_e42492 - locals.var_q_aexp__blk824);
            let assign37680_e42497: f64 = (locals.var_q_d1_ln__blk835 * locals.var_q_sh_term__blk833);
            let assign37680_e42498: f64 = (assign37680_e42494 - assign37680_e42497);
            let assign37680_e42500: f64 = (assign37680_e42498 * locals.var_q_temp2__blk815);
            (locals.var_q_d1_expnum__blk838, locals.var_q_d1_expnum__blk838_dn4, locals.var_q_d1_expnum__blk838_dn6, locals.var_q_d1_expnum__blk838_dn7, locals.var_q_d1_expnum__blk838_dn8, locals.var_q_d1_expnum__blk838_dn9, ) = (assign37680_e42500, ((((((locals.var_q_temp3__blk816_dn4 * locals.var_q_expnum__blk837) + (locals.var_q_temp3__blk816 * locals.var_q_expnum__blk837_dn4)) - locals.var_q_aexp__blk824_dn4) - ((locals.var_q_d1_ln__blk835_dn4 * locals.var_q_sh_term__blk833) + (locals.var_q_d1_ln__blk835 * locals.var_q_sh_term__blk833_dn4))) * locals.var_q_temp2__blk815) + (assign37680_e42498 * locals.var_q_temp2__blk815_dn4)), ((((((locals.var_q_temp3__blk816_dn6 * locals.var_q_expnum__blk837) + (locals.var_q_temp3__blk816 * locals.var_q_expnum__blk837_dn6)) - locals.var_q_aexp__blk824_dn6) - ((locals.var_q_d1_ln__blk835_dn6 * locals.var_q_sh_term__blk833) + (locals.var_q_d1_ln__blk835 * locals.var_q_sh_term__blk833_dn6))) * locals.var_q_temp2__blk815) + (assign37680_e42498 * locals.var_q_temp2__blk815_dn6)), ((((((locals.var_q_temp3__blk816_dn7 * locals.var_q_expnum__blk837) + (locals.var_q_temp3__blk816 * locals.var_q_expnum__blk837_dn7)) - locals.var_q_aexp__blk824_dn7) - ((locals.var_q_d1_ln__blk835_dn7 * locals.var_q_sh_term__blk833) + (locals.var_q_d1_ln__blk835 * locals.var_q_sh_term__blk833_dn7))) * locals.var_q_temp2__blk815) + (assign37680_e42498 * locals.var_q_temp2__blk815_dn7)), ((((((locals.var_q_temp3__blk816_dn8 * locals.var_q_expnum__blk837) + (locals.var_q_temp3__blk816 * locals.var_q_expnum__blk837_dn8)) - locals.var_q_aexp__blk824_dn8) - ((locals.var_q_d1_ln__blk835_dn8 * locals.var_q_sh_term__blk833) + (locals.var_q_d1_ln__blk835 * locals.var_q_sh_term__blk833_dn8))) * locals.var_q_temp2__blk815) + (assign37680_e42498 * locals.var_q_temp2__blk815_dn8)), ((((((locals.var_q_temp3__blk816_dn9 * locals.var_q_expnum__blk837) + (locals.var_q_temp3__blk816 * locals.var_q_expnum__blk837_dn9)) - locals.var_q_aexp__blk824_dn9) - ((locals.var_q_d1_ln__blk835_dn9 * locals.var_q_sh_term__blk833) + (locals.var_q_d1_ln__blk835 * locals.var_q_sh_term__blk833_dn9))) * locals.var_q_temp2__blk815) + (assign37680_e42498 * locals.var_q_temp2__blk815_dn9)), );
            locals.var_q_d1_expnum__blk838_rv = 0.0;
        }
        if ((locals.var_guard1080 != 0.0) && (locals.var_guard1187 == 0.0)) {
            let assign37690_e42509: f64 = (locals.var_q_d2_qcoth__blk832 * locals.var_q_expnum__blk837);
            let assign37690_e42512: f64 = (2.0 * locals.var_q_temp3__blk816);
            let assign37690_e42514: f64 = (assign37690_e42512 * locals.var_q_d1_expnum__blk838);
            let assign37690_e42515: f64 = (assign37690_e42509 + assign37690_e42514);
            let assign37690_e42517: f64 = (assign37690_e42515 + locals.var_q_aexp__blk824);
            let assign37690_e42521: f64 = (locals.var_q_d1_ln__blk835 * locals.var_q_d1_ln__blk835);
            let assign37690_e42522: f64 = (locals.var_q_d2_ln__blk836 + assign37690_e42521);
            let assign37690_e42524: f64 = (assign37690_e42522 * locals.var_q_sh_term__blk833);
            let assign37690_e42525: f64 = (assign37690_e42517 - assign37690_e42524);
            let assign37690_e42527: f64 = (assign37690_e42525 * locals.var_q_temp2__blk815);
            (locals.var_q_d2_expnum__blk839, locals.var_q_d2_expnum__blk839_dn4, locals.var_q_d2_expnum__blk839_dn6, locals.var_q_d2_expnum__blk839_dn7, locals.var_q_d2_expnum__blk839_dn8, locals.var_q_d2_expnum__blk839_dn9, ) = (assign37690_e42527, (((((((locals.var_q_d2_qcoth__blk832_dn4 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qcoth__blk832 * locals.var_q_expnum__blk837_dn4)) + (((2.0 * locals.var_q_temp3__blk816_dn4) * locals.var_q_d1_expnum__blk838) + (assign37690_e42512 * locals.var_q_d1_expnum__blk838_dn4))) + locals.var_q_aexp__blk824_dn4) - (((locals.var_q_d2_ln__blk836_dn4 + ((locals.var_q_d1_ln__blk835_dn4 * locals.var_q_d1_ln__blk835) + (locals.var_q_d1_ln__blk835 * locals.var_q_d1_ln__blk835_dn4))) * locals.var_q_sh_term__blk833) + (assign37690_e42522 * locals.var_q_sh_term__blk833_dn4))) * locals.var_q_temp2__blk815) + (assign37690_e42525 * locals.var_q_temp2__blk815_dn4)), (((((((locals.var_q_d2_qcoth__blk832_dn6 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qcoth__blk832 * locals.var_q_expnum__blk837_dn6)) + (((2.0 * locals.var_q_temp3__blk816_dn6) * locals.var_q_d1_expnum__blk838) + (assign37690_e42512 * locals.var_q_d1_expnum__blk838_dn6))) + locals.var_q_aexp__blk824_dn6) - (((locals.var_q_d2_ln__blk836_dn6 + ((locals.var_q_d1_ln__blk835_dn6 * locals.var_q_d1_ln__blk835) + (locals.var_q_d1_ln__blk835 * locals.var_q_d1_ln__blk835_dn6))) * locals.var_q_sh_term__blk833) + (assign37690_e42522 * locals.var_q_sh_term__blk833_dn6))) * locals.var_q_temp2__blk815) + (assign37690_e42525 * locals.var_q_temp2__blk815_dn6)), (((((((locals.var_q_d2_qcoth__blk832_dn7 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qcoth__blk832 * locals.var_q_expnum__blk837_dn7)) + (((2.0 * locals.var_q_temp3__blk816_dn7) * locals.var_q_d1_expnum__blk838) + (assign37690_e42512 * locals.var_q_d1_expnum__blk838_dn7))) + locals.var_q_aexp__blk824_dn7) - (((locals.var_q_d2_ln__blk836_dn7 + ((locals.var_q_d1_ln__blk835_dn7 * locals.var_q_d1_ln__blk835) + (locals.var_q_d1_ln__blk835 * locals.var_q_d1_ln__blk835_dn7))) * locals.var_q_sh_term__blk833) + (assign37690_e42522 * locals.var_q_sh_term__blk833_dn7))) * locals.var_q_temp2__blk815) + (assign37690_e42525 * locals.var_q_temp2__blk815_dn7)), (((((((locals.var_q_d2_qcoth__blk832_dn8 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qcoth__blk832 * locals.var_q_expnum__blk837_dn8)) + (((2.0 * locals.var_q_temp3__blk816_dn8) * locals.var_q_d1_expnum__blk838) + (assign37690_e42512 * locals.var_q_d1_expnum__blk838_dn8))) + locals.var_q_aexp__blk824_dn8) - (((locals.var_q_d2_ln__blk836_dn8 + ((locals.var_q_d1_ln__blk835_dn8 * locals.var_q_d1_ln__blk835) + (locals.var_q_d1_ln__blk835 * locals.var_q_d1_ln__blk835_dn8))) * locals.var_q_sh_term__blk833) + (assign37690_e42522 * locals.var_q_sh_term__blk833_dn8))) * locals.var_q_temp2__blk815) + (assign37690_e42525 * locals.var_q_temp2__blk815_dn8)), (((((((locals.var_q_d2_qcoth__blk832_dn9 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qcoth__blk832 * locals.var_q_expnum__blk837_dn9)) + (((2.0 * locals.var_q_temp3__blk816_dn9) * locals.var_q_d1_expnum__blk838) + (assign37690_e42512 * locals.var_q_d1_expnum__blk838_dn9))) + locals.var_q_aexp__blk824_dn9) - (((locals.var_q_d2_ln__blk836_dn9 + ((locals.var_q_d1_ln__blk835_dn9 * locals.var_q_d1_ln__blk835) + (locals.var_q_d1_ln__blk835 * locals.var_q_d1_ln__blk835_dn9))) * locals.var_q_sh_term__blk833) + (assign37690_e42522 * locals.var_q_sh_term__blk833_dn9))) * locals.var_q_temp2__blk815) + (assign37690_e42525 * locals.var_q_temp2__blk815_dn9)), );
            locals.var_q_d2_expnum__blk839_rv = 0.0;
        }
        let assign37700_e42532: f64 = if locals.var_q_expnum__blk837 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1188 = assign37700_e42532;
        locals.var_guard1188_rv = 0.0;
        if ((locals.var_guard1080 != 0.0) && (locals.var_guard1188 != 0.0)) {
            let assign37710_e42537: f64 = (locals.var_q_expnum__blk837).ln();
            (locals.var_q_lnexpnum__blk840, locals.var_q_lnexpnum__blk840_dn4, locals.var_q_lnexpnum__blk840_dn6, locals.var_q_lnexpnum__blk840_dn7, locals.var_q_lnexpnum__blk840_dn8, locals.var_q_lnexpnum__blk840_dn9, ) = (assign37710_e42537, (locals.var_q_expnum__blk837_dn4 / locals.var_q_expnum__blk837), (locals.var_q_expnum__blk837_dn6 / locals.var_q_expnum__blk837), (locals.var_q_expnum__blk837_dn7 / locals.var_q_expnum__blk837), (locals.var_q_expnum__blk837_dn8 / locals.var_q_expnum__blk837), (locals.var_q_expnum__blk837_dn9 / locals.var_q_expnum__blk837), );
            locals.var_q_lnexpnum__blk840_rv = 0.0;
        }
        if ((locals.var_guard1080 != 0.0) && (locals.var_guard1188 != 0.0)) {
            let assign37720_e42545: f64 = (1.0 / locals.var_q_expnum__blk837);
            (locals.var_q_temp1__blk814, locals.var_q_temp1__blk814_dn4, locals.var_q_temp1__blk814_dn6, locals.var_q_temp1__blk814_dn7, locals.var_q_temp1__blk814_dn8, locals.var_q_temp1__blk814_dn9, ) = (assign37720_e42545, (-(locals.var_q_expnum__blk837_dn4 / (locals.var_q_expnum__blk837 * locals.var_q_expnum__blk837))), (-(locals.var_q_expnum__blk837_dn6 / (locals.var_q_expnum__blk837 * locals.var_q_expnum__blk837))), (-(locals.var_q_expnum__blk837_dn7 / (locals.var_q_expnum__blk837 * locals.var_q_expnum__blk837))), (-(locals.var_q_expnum__blk837_dn8 / (locals.var_q_expnum__blk837 * locals.var_q_expnum__blk837))), (-(locals.var_q_expnum__blk837_dn9 / (locals.var_q_expnum__blk837 * locals.var_q_expnum__blk837))), );
            locals.var_q_temp1__blk814_rv = 0.0;
        }
        if ((locals.var_guard1080 != 0.0) && (locals.var_guard1188 != 0.0)) {
            let assign37730_e42553: f64 = (locals.var_q_d1_expnum__blk838 * locals.var_q_temp1__blk814);
            (locals.var_q_d1_lnexpnum__blk841, locals.var_q_d1_lnexpnum__blk841_dn4, locals.var_q_d1_lnexpnum__blk841_dn6, locals.var_q_d1_lnexpnum__blk841_dn7, locals.var_q_d1_lnexpnum__blk841_dn8, locals.var_q_d1_lnexpnum__blk841_dn9, ) = (assign37730_e42553, ((locals.var_q_d1_expnum__blk838_dn4 * locals.var_q_temp1__blk814) + (locals.var_q_d1_expnum__blk838 * locals.var_q_temp1__blk814_dn4)), ((locals.var_q_d1_expnum__blk838_dn6 * locals.var_q_temp1__blk814) + (locals.var_q_d1_expnum__blk838 * locals.var_q_temp1__blk814_dn6)), ((locals.var_q_d1_expnum__blk838_dn7 * locals.var_q_temp1__blk814) + (locals.var_q_d1_expnum__blk838 * locals.var_q_temp1__blk814_dn7)), ((locals.var_q_d1_expnum__blk838_dn8 * locals.var_q_temp1__blk814) + (locals.var_q_d1_expnum__blk838 * locals.var_q_temp1__blk814_dn8)), ((locals.var_q_d1_expnum__blk838_dn9 * locals.var_q_temp1__blk814) + (locals.var_q_d1_expnum__blk838 * locals.var_q_temp1__blk814_dn9)), );
            locals.var_q_d1_lnexpnum__blk841_rv = 0.0;
        }
        if ((locals.var_guard1080 != 0.0) && (locals.var_guard1188 != 0.0)) {
            let assign37740_e42561: f64 = (locals.var_q_d2_expnum__blk839 * locals.var_q_temp1__blk814);
            let assign37740_e42564: f64 = (locals.var_q_d1_lnexpnum__blk841 * locals.var_q_d1_lnexpnum__blk841);
            let assign37740_e42565: f64 = (assign37740_e42561 - assign37740_e42564);
            (locals.var_q_d2_lnexpnum__blk842, locals.var_q_d2_lnexpnum__blk842_dn4, locals.var_q_d2_lnexpnum__blk842_dn6, locals.var_q_d2_lnexpnum__blk842_dn7, locals.var_q_d2_lnexpnum__blk842_dn8, locals.var_q_d2_lnexpnum__blk842_dn9, ) = (assign37740_e42565, (((locals.var_q_d2_expnum__blk839_dn4 * locals.var_q_temp1__blk814) + (locals.var_q_d2_expnum__blk839 * locals.var_q_temp1__blk814_dn4)) - ((locals.var_q_d1_lnexpnum__blk841_dn4 * locals.var_q_d1_lnexpnum__blk841) + (locals.var_q_d1_lnexpnum__blk841 * locals.var_q_d1_lnexpnum__blk841_dn4))), (((locals.var_q_d2_expnum__blk839_dn6 * locals.var_q_temp1__blk814) + (locals.var_q_d2_expnum__blk839 * locals.var_q_temp1__blk814_dn6)) - ((locals.var_q_d1_lnexpnum__blk841_dn6 * locals.var_q_d1_lnexpnum__blk841) + (locals.var_q_d1_lnexpnum__blk841 * locals.var_q_d1_lnexpnum__blk841_dn6))), (((locals.var_q_d2_expnum__blk839_dn7 * locals.var_q_temp1__blk814) + (locals.var_q_d2_expnum__blk839 * locals.var_q_temp1__blk814_dn7)) - ((locals.var_q_d1_lnexpnum__blk841_dn7 * locals.var_q_d1_lnexpnum__blk841) + (locals.var_q_d1_lnexpnum__blk841 * locals.var_q_d1_lnexpnum__blk841_dn7))), (((locals.var_q_d2_expnum__blk839_dn8 * locals.var_q_temp1__blk814) + (locals.var_q_d2_expnum__blk839 * locals.var_q_temp1__blk814_dn8)) - ((locals.var_q_d1_lnexpnum__blk841_dn8 * locals.var_q_d1_lnexpnum__blk841) + (locals.var_q_d1_lnexpnum__blk841 * locals.var_q_d1_lnexpnum__blk841_dn8))), (((locals.var_q_d2_expnum__blk839_dn9 * locals.var_q_temp1__blk814) + (locals.var_q_d2_expnum__blk839 * locals.var_q_temp1__blk814_dn9)) - ((locals.var_q_d1_lnexpnum__blk841_dn9 * locals.var_q_d1_lnexpnum__blk841) + (locals.var_q_d1_lnexpnum__blk841 * locals.var_q_d1_lnexpnum__blk841_dn9))), );
            locals.var_q_d2_lnexpnum__blk842_rv = 0.0;
        }
        if ((locals.var_guard1080 != 0.0) && (locals.var_guard1188 == 0.0)) {
            let assign37750_e42574: f64 = (locals.var_q_k1q1__blk823 + 0.6931471805599);
            let assign37750_e42576: f64 = (-locals.var_q_k1q1__blk823);
            let assign37750_e42577: f64 = (assign37750_e42576).ln();
            let assign37750_e42578: f64 = (assign37750_e42574 + assign37750_e42577);
            (locals.var_q_lnexpnum__blk840, locals.var_q_lnexpnum__blk840_dn4, locals.var_q_lnexpnum__blk840_dn6, locals.var_q_lnexpnum__blk840_dn7, locals.var_q_lnexpnum__blk840_dn8, locals.var_q_lnexpnum__blk840_dn9, ) = (assign37750_e42578, (locals.var_q_k1q1__blk823_dn4 + ((-locals.var_q_k1q1__blk823_dn4) / assign37750_e42576)), (locals.var_q_k1q1__blk823_dn6 + ((-locals.var_q_k1q1__blk823_dn6) / assign37750_e42576)), (locals.var_q_k1q1__blk823_dn7 + ((-locals.var_q_k1q1__blk823_dn7) / assign37750_e42576)), (locals.var_q_k1q1__blk823_dn8 + ((-locals.var_q_k1q1__blk823_dn8) / assign37750_e42576)), (locals.var_q_k1q1__blk823_dn9 + ((-locals.var_q_k1q1__blk823_dn9) / assign37750_e42576)), );
            locals.var_q_lnexpnum__blk840_rv = 0.0;
        }
        if ((locals.var_guard1080 != 0.0) && (locals.var_guard1188 == 0.0)) {
            let assign37760_e42587: f64 = (1.0 / locals.var_q1d__blk1001);
            (locals.var_q_temp1__blk814, locals.var_q_temp1__blk814_dn4, locals.var_q_temp1__blk814_dn6, locals.var_q_temp1__blk814_dn7, locals.var_q_temp1__blk814_dn8, locals.var_q_temp1__blk814_dn9, ) = (assign37760_e42587, (-(locals.var_q1d__blk1001_dn4 / (locals.var_q1d__blk1001 * locals.var_q1d__blk1001))), (-(locals.var_q1d__blk1001_dn6 / (locals.var_q1d__blk1001 * locals.var_q1d__blk1001))), (-(locals.var_q1d__blk1001_dn7 / (locals.var_q1d__blk1001 * locals.var_q1d__blk1001))), (-(locals.var_q1d__blk1001_dn8 / (locals.var_q1d__blk1001 * locals.var_q1d__blk1001))), (-(locals.var_q1d__blk1001_dn9 / (locals.var_q1d__blk1001 * locals.var_q1d__blk1001))), );
            locals.var_q_temp1__blk814_rv = 0.0;
        }
        if ((locals.var_guard1080 != 0.0) && (locals.var_guard1188 == 0.0)) {
            let assign37770_e42596: f64 = (locals.var_k1__blk932 + locals.var_q_temp1__blk814);
            (locals.var_q_d1_lnexpnum__blk841, locals.var_q_d1_lnexpnum__blk841_dn4, locals.var_q_d1_lnexpnum__blk841_dn6, locals.var_q_d1_lnexpnum__blk841_dn7, locals.var_q_d1_lnexpnum__blk841_dn8, locals.var_q_d1_lnexpnum__blk841_dn9, ) = (assign37770_e42596, (locals.var_k1__blk932_dn4 + locals.var_q_temp1__blk814_dn4), (locals.var_k1__blk932_dn6 + locals.var_q_temp1__blk814_dn6), (locals.var_k1__blk932_dn7 + locals.var_q_temp1__blk814_dn7), (locals.var_k1__blk932_dn8 + locals.var_q_temp1__blk814_dn8), (locals.var_k1__blk932_dn9 + locals.var_q_temp1__blk814_dn9), );
            locals.var_q_d1_lnexpnum__blk841_rv = 0.0;
        }
        if ((locals.var_guard1080 != 0.0) && (locals.var_guard1188 == 0.0)) {
            let assign37780_e42604: f64 = (-locals.var_q_temp1__blk814);
            let assign37780_e42606: f64 = (assign37780_e42604 * locals.var_q_temp1__blk814);
            (locals.var_q_d2_lnexpnum__blk842, locals.var_q_d2_lnexpnum__blk842_dn4, locals.var_q_d2_lnexpnum__blk842_dn6, locals.var_q_d2_lnexpnum__blk842_dn7, locals.var_q_d2_lnexpnum__blk842_dn8, locals.var_q_d2_lnexpnum__blk842_dn9, ) = (assign37780_e42606, (((-locals.var_q_temp1__blk814_dn4) * locals.var_q_temp1__blk814) + (assign37780_e42604 * locals.var_q_temp1__blk814_dn4)), (((-locals.var_q_temp1__blk814_dn6) * locals.var_q_temp1__blk814) + (assign37780_e42604 * locals.var_q_temp1__blk814_dn6)), (((-locals.var_q_temp1__blk814_dn7) * locals.var_q_temp1__blk814) + (assign37780_e42604 * locals.var_q_temp1__blk814_dn7)), (((-locals.var_q_temp1__blk814_dn8) * locals.var_q_temp1__blk814) + (assign37780_e42604 * locals.var_q_temp1__blk814_dn8)), (((-locals.var_q_temp1__blk814_dn9) * locals.var_q_temp1__blk814) + (assign37780_e42604 * locals.var_q_temp1__blk814_dn9)), );
            locals.var_q_d2_lnexpnum__blk842_rv = 0.0;
        }
        if (locals.var_guard1080 != 0.0) {
            let assign37790_e42612: f64 = (locals.var_xg2x__blk931 - locals.var_xg1x__blk930);
            let assign37790_e42614: f64 = (assign37790_e42612 + locals.var_q1d__blk1001);
            let assign37790_e42617: f64 = (2.0 * locals.var_q_lnexpnum__blk840);
            let assign37790_e42618: f64 = (assign37790_e42614 + assign37790_e42617);
            let assign37790_e42620: f64 = (assign37790_e42618 - locals.var_q_ln_term__blk834);
            (locals.var_q_q2_int__blk843, locals.var_q_q2_int__blk843_dn4, locals.var_q_q2_int__blk843_dn6, locals.var_q_q2_int__blk843_dn7, locals.var_q_q2_int__blk843_dn8, locals.var_q_q2_int__blk843_dn9, ) = (assign37790_e42620, ((((locals.var_xg2x__blk931_dn4 - locals.var_xg1x__blk930_dn4) + locals.var_q1d__blk1001_dn4) + (2.0 * locals.var_q_lnexpnum__blk840_dn4)) - locals.var_q_ln_term__blk834_dn4), ((((locals.var_xg2x__blk931_dn6 - locals.var_xg1x__blk930_dn6) + locals.var_q1d__blk1001_dn6) + (2.0 * locals.var_q_lnexpnum__blk840_dn6)) - locals.var_q_ln_term__blk834_dn6), ((((locals.var_xg2x__blk931_dn7 - locals.var_xg1x__blk930_dn7) + locals.var_q1d__blk1001_dn7) + (2.0 * locals.var_q_lnexpnum__blk840_dn7)) - locals.var_q_ln_term__blk834_dn7), ((((locals.var_xg2x__blk931_dn8 - locals.var_xg1x__blk930_dn8) + locals.var_q1d__blk1001_dn8) + (2.0 * locals.var_q_lnexpnum__blk840_dn8)) - locals.var_q_ln_term__blk834_dn8), ((((locals.var_xg2x__blk931_dn9 - locals.var_xg1x__blk930_dn9) + locals.var_q1d__blk1001_dn9) + (2.0 * locals.var_q_lnexpnum__blk840_dn9)) - locals.var_q_ln_term__blk834_dn9), );
            locals.var_q_q2_int__blk843_rv = 0.0;
        }
        if (locals.var_guard1080 != 0.0) {
            let assign37800_e42627: f64 = (2.0 * locals.var_q_d1_lnexpnum__blk841);
            let assign37800_e42628: f64 = (1.0 + assign37800_e42627);
            let assign37800_e42630: f64 = (assign37800_e42628 - locals.var_q_d1_ln__blk835);
            (locals.var_q_d1_q2__blk844, locals.var_q_d1_q2__blk844_dn4, locals.var_q_d1_q2__blk844_dn6, locals.var_q_d1_q2__blk844_dn7, locals.var_q_d1_q2__blk844_dn8, locals.var_q_d1_q2__blk844_dn9, ) = (assign37800_e42630, ((2.0 * locals.var_q_d1_lnexpnum__blk841_dn4) - locals.var_q_d1_ln__blk835_dn4), ((2.0 * locals.var_q_d1_lnexpnum__blk841_dn6) - locals.var_q_d1_ln__blk835_dn6), ((2.0 * locals.var_q_d1_lnexpnum__blk841_dn7) - locals.var_q_d1_ln__blk835_dn7), ((2.0 * locals.var_q_d1_lnexpnum__blk841_dn8) - locals.var_q_d1_ln__blk835_dn8), ((2.0 * locals.var_q_d1_lnexpnum__blk841_dn9) - locals.var_q_d1_ln__blk835_dn9), );
            locals.var_q_d1_q2__blk844_rv = 0.0;
        }
        if (locals.var_guard1080 != 0.0) {
            let assign37810_e42636: f64 = (2.0 * locals.var_q_d2_lnexpnum__blk842);
            let assign37810_e42638: f64 = (assign37810_e42636 - locals.var_q_d2_ln__blk836);
            (locals.var_q_d2_q2__blk845, locals.var_q_d2_q2__blk845_dn4, locals.var_q_d2_q2__blk845_dn6, locals.var_q_d2_q2__blk845_dn7, locals.var_q_d2_q2__blk845_dn8, locals.var_q_d2_q2__blk845_dn9, ) = (assign37810_e42638, ((2.0 * locals.var_q_d2_lnexpnum__blk842_dn4) - locals.var_q_d2_ln__blk836_dn4), ((2.0 * locals.var_q_d2_lnexpnum__blk842_dn6) - locals.var_q_d2_ln__blk836_dn6), ((2.0 * locals.var_q_d2_lnexpnum__blk842_dn7) - locals.var_q_d2_ln__blk836_dn7), ((2.0 * locals.var_q_d2_lnexpnum__blk842_dn8) - locals.var_q_d2_ln__blk836_dn8), ((2.0 * locals.var_q_d2_lnexpnum__blk842_dn9) - locals.var_q_d2_ln__blk836_dn9), );
            locals.var_q_d2_q2__blk845_rv = 0.0;
        }
        if (locals.var_guard1080 != 0.0) {
            let assign37820_e42645: f64 = (locals.var_k2__blk933 * locals.var_q_q2_int__blk843);
            let assign37820_e42646: f64 = (locals.var_q_k1q1__blk823 + assign37820_e42645);
            (locals.var_q_qi_int__blk846, locals.var_q_qi_int__blk846_dn4, locals.var_q_qi_int__blk846_dn6, locals.var_q_qi_int__blk846_dn7, locals.var_q_qi_int__blk846_dn8, locals.var_q_qi_int__blk846_dn9, ) = (assign37820_e42646, (locals.var_q_k1q1__blk823_dn4 + ((locals.var_k2__blk933_dn4 * locals.var_q_q2_int__blk843) + (locals.var_k2__blk933 * locals.var_q_q2_int__blk843_dn4))), (locals.var_q_k1q1__blk823_dn6 + ((locals.var_k2__blk933_dn6 * locals.var_q_q2_int__blk843) + (locals.var_k2__blk933 * locals.var_q_q2_int__blk843_dn6))), (locals.var_q_k1q1__blk823_dn7 + ((locals.var_k2__blk933_dn7 * locals.var_q_q2_int__blk843) + (locals.var_k2__blk933 * locals.var_q_q2_int__blk843_dn7))), (locals.var_q_k1q1__blk823_dn8 + ((locals.var_k2__blk933_dn8 * locals.var_q_q2_int__blk843) + (locals.var_k2__blk933 * locals.var_q_q2_int__blk843_dn8))), (locals.var_q_k1q1__blk823_dn9 + ((locals.var_k2__blk933_dn9 * locals.var_q_q2_int__blk843) + (locals.var_k2__blk933 * locals.var_q_q2_int__blk843_dn9))), );
            locals.var_q_qi_int__blk846_rv = 0.0;
        }
        if (locals.var_guard1080 != 0.0) {
            let assign37830_e42653: f64 = (locals.var_k2__blk933 * locals.var_q_d1_q2__blk844);
            let assign37830_e42654: f64 = (locals.var_k1__blk932 + assign37830_e42653);
            (locals.var_q_d1_qi__blk847, locals.var_q_d1_qi__blk847_dn4, locals.var_q_d1_qi__blk847_dn6, locals.var_q_d1_qi__blk847_dn7, locals.var_q_d1_qi__blk847_dn8, locals.var_q_d1_qi__blk847_dn9, ) = (assign37830_e42654, (locals.var_k1__blk932_dn4 + ((locals.var_k2__blk933_dn4 * locals.var_q_d1_q2__blk844) + (locals.var_k2__blk933 * locals.var_q_d1_q2__blk844_dn4))), (locals.var_k1__blk932_dn6 + ((locals.var_k2__blk933_dn6 * locals.var_q_d1_q2__blk844) + (locals.var_k2__blk933 * locals.var_q_d1_q2__blk844_dn6))), (locals.var_k1__blk932_dn7 + ((locals.var_k2__blk933_dn7 * locals.var_q_d1_q2__blk844) + (locals.var_k2__blk933 * locals.var_q_d1_q2__blk844_dn7))), (locals.var_k1__blk932_dn8 + ((locals.var_k2__blk933_dn8 * locals.var_q_d1_q2__blk844) + (locals.var_k2__blk933 * locals.var_q_d1_q2__blk844_dn8))), (locals.var_k1__blk932_dn9 + ((locals.var_k2__blk933_dn9 * locals.var_q_d1_q2__blk844) + (locals.var_k2__blk933 * locals.var_q_d1_q2__blk844_dn9))), );
            locals.var_q_d1_qi__blk847_rv = 0.0;
        }
        if (locals.var_guard1080 != 0.0) {
            let assign37840_e42660: f64 = (locals.var_k2__blk933 * locals.var_q_d2_q2__blk845);
            (locals.var_q_d2_qi__blk848, locals.var_q_d2_qi__blk848_dn4, locals.var_q_d2_qi__blk848_dn6, locals.var_q_d2_qi__blk848_dn7, locals.var_q_d2_qi__blk848_dn8, locals.var_q_d2_qi__blk848_dn9, ) = (assign37840_e42660, ((locals.var_k2__blk933_dn4 * locals.var_q_d2_q2__blk845) + (locals.var_k2__blk933 * locals.var_q_d2_q2__blk845_dn4)), ((locals.var_k2__blk933_dn6 * locals.var_q_d2_q2__blk845) + (locals.var_k2__blk933 * locals.var_q_d2_q2__blk845_dn6)), ((locals.var_k2__blk933_dn7 * locals.var_q_d2_q2__blk845) + (locals.var_k2__blk933 * locals.var_q_d2_q2__blk845_dn7)), ((locals.var_k2__blk933_dn8 * locals.var_q_d2_q2__blk845) + (locals.var_k2__blk933 * locals.var_q_d2_q2__blk845_dn8)), ((locals.var_k2__blk933_dn9 * locals.var_q_d2_q2__blk845) + (locals.var_k2__blk933 * locals.var_q_d2_q2__blk845_dn9)), );
            locals.var_q_d2_qi__blk848_rv = 0.0;
        }
        if (locals.var_guard1080 != 0.0) {
            let assign37850_e42666: f64 = (locals.var_q_qi_int__blk846 * locals.var_q_expnum__blk837);
            let assign37850_e42668: f64 = (assign37850_e42666 - locals.var_q_aexp__blk824);
            (locals.var_q_zero__blk849, locals.var_q_zero__blk849_dn4, locals.var_q_zero__blk849_dn6, locals.var_q_zero__blk849_dn7, locals.var_q_zero__blk849_dn8, locals.var_q_zero__blk849_dn9, ) = (assign37850_e42668, (((locals.var_q_qi_int__blk846_dn4 * locals.var_q_expnum__blk837) + (locals.var_q_qi_int__blk846 * locals.var_q_expnum__blk837_dn4)) - locals.var_q_aexp__blk824_dn4), (((locals.var_q_qi_int__blk846_dn6 * locals.var_q_expnum__blk837) + (locals.var_q_qi_int__blk846 * locals.var_q_expnum__blk837_dn6)) - locals.var_q_aexp__blk824_dn6), (((locals.var_q_qi_int__blk846_dn7 * locals.var_q_expnum__blk837) + (locals.var_q_qi_int__blk846 * locals.var_q_expnum__blk837_dn7)) - locals.var_q_aexp__blk824_dn7), (((locals.var_q_qi_int__blk846_dn8 * locals.var_q_expnum__blk837) + (locals.var_q_qi_int__blk846 * locals.var_q_expnum__blk837_dn8)) - locals.var_q_aexp__blk824_dn8), (((locals.var_q_qi_int__blk846_dn9 * locals.var_q_expnum__blk837) + (locals.var_q_qi_int__blk846 * locals.var_q_expnum__blk837_dn9)) - locals.var_q_aexp__blk824_dn9), );
            locals.var_q_zero__blk849_rv = 0.0;
        }
        if (locals.var_guard1080 != 0.0) {
            let assign37860_e42674: f64 = (locals.var_q_d1_qi__blk847 * locals.var_q_expnum__blk837);
            let assign37860_e42677: f64 = (locals.var_q_qi_int__blk846 * locals.var_q_d1_expnum__blk838);
            let assign37860_e42678: f64 = (assign37860_e42674 + assign37860_e42677);
            let assign37860_e42680: f64 = (assign37860_e42678 + locals.var_q_aexp__blk824);
            (locals.var_q_d1_zero__blk850, locals.var_q_d1_zero__blk850_dn4, locals.var_q_d1_zero__blk850_dn6, locals.var_q_d1_zero__blk850_dn7, locals.var_q_d1_zero__blk850_dn8, locals.var_q_d1_zero__blk850_dn9, ) = (assign37860_e42680, ((((locals.var_q_d1_qi__blk847_dn4 * locals.var_q_expnum__blk837) + (locals.var_q_d1_qi__blk847 * locals.var_q_expnum__blk837_dn4)) + ((locals.var_q_qi_int__blk846_dn4 * locals.var_q_d1_expnum__blk838) + (locals.var_q_qi_int__blk846 * locals.var_q_d1_expnum__blk838_dn4))) + locals.var_q_aexp__blk824_dn4), ((((locals.var_q_d1_qi__blk847_dn6 * locals.var_q_expnum__blk837) + (locals.var_q_d1_qi__blk847 * locals.var_q_expnum__blk837_dn6)) + ((locals.var_q_qi_int__blk846_dn6 * locals.var_q_d1_expnum__blk838) + (locals.var_q_qi_int__blk846 * locals.var_q_d1_expnum__blk838_dn6))) + locals.var_q_aexp__blk824_dn6), ((((locals.var_q_d1_qi__blk847_dn7 * locals.var_q_expnum__blk837) + (locals.var_q_d1_qi__blk847 * locals.var_q_expnum__blk837_dn7)) + ((locals.var_q_qi_int__blk846_dn7 * locals.var_q_d1_expnum__blk838) + (locals.var_q_qi_int__blk846 * locals.var_q_d1_expnum__blk838_dn7))) + locals.var_q_aexp__blk824_dn7), ((((locals.var_q_d1_qi__blk847_dn8 * locals.var_q_expnum__blk837) + (locals.var_q_d1_qi__blk847 * locals.var_q_expnum__blk837_dn8)) + ((locals.var_q_qi_int__blk846_dn8 * locals.var_q_d1_expnum__blk838) + (locals.var_q_qi_int__blk846 * locals.var_q_d1_expnum__blk838_dn8))) + locals.var_q_aexp__blk824_dn8), ((((locals.var_q_d1_qi__blk847_dn9 * locals.var_q_expnum__blk837) + (locals.var_q_d1_qi__blk847 * locals.var_q_expnum__blk837_dn9)) + ((locals.var_q_qi_int__blk846_dn9 * locals.var_q_d1_expnum__blk838) + (locals.var_q_qi_int__blk846 * locals.var_q_d1_expnum__blk838_dn9))) + locals.var_q_aexp__blk824_dn9), );
            locals.var_q_d1_zero__blk850_rv = 0.0;
        }
        if (locals.var_guard1080 != 0.0) {
            let assign37870_e42686: f64 = (locals.var_q_d2_qi__blk848 * locals.var_q_expnum__blk837);
            let assign37870_e42689: f64 = (2.0 * locals.var_q_d1_qi__blk847);
            let assign37870_e42691: f64 = (assign37870_e42689 * locals.var_q_d1_expnum__blk838);
            let assign37870_e42692: f64 = (assign37870_e42686 + assign37870_e42691);
            let assign37870_e42695: f64 = (locals.var_q_qi_int__blk846 * locals.var_q_d2_expnum__blk839);
            let assign37870_e42696: f64 = (assign37870_e42692 + assign37870_e42695);
            let assign37870_e42698: f64 = (assign37870_e42696 - locals.var_q_aexp__blk824);
            (locals.var_q_d2_zero__blk851, locals.var_q_d2_zero__blk851_dn4, locals.var_q_d2_zero__blk851_dn6, locals.var_q_d2_zero__blk851_dn7, locals.var_q_d2_zero__blk851_dn8, locals.var_q_d2_zero__blk851_dn9, ) = (assign37870_e42698, (((((locals.var_q_d2_qi__blk848_dn4 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qi__blk848 * locals.var_q_expnum__blk837_dn4)) + (((2.0 * locals.var_q_d1_qi__blk847_dn4) * locals.var_q_d1_expnum__blk838) + (assign37870_e42689 * locals.var_q_d1_expnum__blk838_dn4))) + ((locals.var_q_qi_int__blk846_dn4 * locals.var_q_d2_expnum__blk839) + (locals.var_q_qi_int__blk846 * locals.var_q_d2_expnum__blk839_dn4))) - locals.var_q_aexp__blk824_dn4), (((((locals.var_q_d2_qi__blk848_dn6 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qi__blk848 * locals.var_q_expnum__blk837_dn6)) + (((2.0 * locals.var_q_d1_qi__blk847_dn6) * locals.var_q_d1_expnum__blk838) + (assign37870_e42689 * locals.var_q_d1_expnum__blk838_dn6))) + ((locals.var_q_qi_int__blk846_dn6 * locals.var_q_d2_expnum__blk839) + (locals.var_q_qi_int__blk846 * locals.var_q_d2_expnum__blk839_dn6))) - locals.var_q_aexp__blk824_dn6), (((((locals.var_q_d2_qi__blk848_dn7 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qi__blk848 * locals.var_q_expnum__blk837_dn7)) + (((2.0 * locals.var_q_d1_qi__blk847_dn7) * locals.var_q_d1_expnum__blk838) + (assign37870_e42689 * locals.var_q_d1_expnum__blk838_dn7))) + ((locals.var_q_qi_int__blk846_dn7 * locals.var_q_d2_expnum__blk839) + (locals.var_q_qi_int__blk846 * locals.var_q_d2_expnum__blk839_dn7))) - locals.var_q_aexp__blk824_dn7), (((((locals.var_q_d2_qi__blk848_dn8 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qi__blk848 * locals.var_q_expnum__blk837_dn8)) + (((2.0 * locals.var_q_d1_qi__blk847_dn8) * locals.var_q_d1_expnum__blk838) + (assign37870_e42689 * locals.var_q_d1_expnum__blk838_dn8))) + ((locals.var_q_qi_int__blk846_dn8 * locals.var_q_d2_expnum__blk839) + (locals.var_q_qi_int__blk846 * locals.var_q_d2_expnum__blk839_dn8))) - locals.var_q_aexp__blk824_dn8), (((((locals.var_q_d2_qi__blk848_dn9 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qi__blk848 * locals.var_q_expnum__blk837_dn9)) + (((2.0 * locals.var_q_d1_qi__blk847_dn9) * locals.var_q_d1_expnum__blk838) + (assign37870_e42689 * locals.var_q_d1_expnum__blk838_dn9))) + ((locals.var_q_qi_int__blk846_dn9 * locals.var_q_d2_expnum__blk839) + (locals.var_q_qi_int__blk846 * locals.var_q_d2_expnum__blk839_dn9))) - locals.var_q_aexp__blk824_dn9), );
            locals.var_q_d2_zero__blk851_rv = 0.0;
        }
        if (locals.var_guard1080 != 0.0) {
            let assign37880_e42704: f64 = (locals.var_q_d1_zero__blk850 * locals.var_q_d1_zero__blk850);
            let assign37880_e42707: f64 = (0.5 * locals.var_q_zero__blk849);
            let assign37880_e42709: f64 = (assign37880_e42707 * locals.var_q_d2_zero__blk851);
            let assign37880_e42710: f64 = (assign37880_e42704 - assign37880_e42709);
            (locals.var_q_temp__blk860, locals.var_q_temp__blk860_dn4, locals.var_q_temp__blk860_dn6, locals.var_q_temp__blk860_dn7, locals.var_q_temp__blk860_dn8, locals.var_q_temp__blk860_dn9, ) = (assign37880_e42710, (((locals.var_q_d1_zero__blk850_dn4 * locals.var_q_d1_zero__blk850) + (locals.var_q_d1_zero__blk850 * locals.var_q_d1_zero__blk850_dn4)) - (((0.5 * locals.var_q_zero__blk849_dn4) * locals.var_q_d2_zero__blk851) + (assign37880_e42707 * locals.var_q_d2_zero__blk851_dn4))), (((locals.var_q_d1_zero__blk850_dn6 * locals.var_q_d1_zero__blk850) + (locals.var_q_d1_zero__blk850 * locals.var_q_d1_zero__blk850_dn6)) - (((0.5 * locals.var_q_zero__blk849_dn6) * locals.var_q_d2_zero__blk851) + (assign37880_e42707 * locals.var_q_d2_zero__blk851_dn6))), (((locals.var_q_d1_zero__blk850_dn7 * locals.var_q_d1_zero__blk850) + (locals.var_q_d1_zero__blk850 * locals.var_q_d1_zero__blk850_dn7)) - (((0.5 * locals.var_q_zero__blk849_dn7) * locals.var_q_d2_zero__blk851) + (assign37880_e42707 * locals.var_q_d2_zero__blk851_dn7))), (((locals.var_q_d1_zero__blk850_dn8 * locals.var_q_d1_zero__blk850) + (locals.var_q_d1_zero__blk850 * locals.var_q_d1_zero__blk850_dn8)) - (((0.5 * locals.var_q_zero__blk849_dn8) * locals.var_q_d2_zero__blk851) + (assign37880_e42707 * locals.var_q_d2_zero__blk851_dn8))), (((locals.var_q_d1_zero__blk850_dn9 * locals.var_q_d1_zero__blk850) + (locals.var_q_d1_zero__blk850 * locals.var_q_d1_zero__blk850_dn9)) - (((0.5 * locals.var_q_zero__blk849_dn9) * locals.var_q_d2_zero__blk851) + (assign37880_e42707 * locals.var_q_d2_zero__blk851_dn9))), );
            locals.var_q_temp__blk860_rv = 0.0;
        }
        if (locals.var_guard1080 != 0.0) {
            let assign37890_e42715: f64 = (-locals.var_q_zero__blk849);
            let assign37890_e42717: f64 = (assign37890_e42715 * locals.var_q_d1_zero__blk850);
            let assign37890_e42719: f64 = (assign37890_e42717 * locals.var_q_temp__blk860);
            let assign37890_e42722: f64 = (locals.var_q_temp__blk860 * locals.var_q_temp__blk860);
            let assign37890_e42724: f64 = (assign37890_e42722 + 1e-200);
            let assign37890_e42725: f64 = (assign37890_e42719 / assign37890_e42724);
            (locals.var_q_eps2__blk852, locals.var_q_eps2__blk852_dn4, locals.var_q_eps2__blk852_dn6, locals.var_q_eps2__blk852_dn7, locals.var_q_eps2__blk852_dn8, locals.var_q_eps2__blk852_dn9, ) = (assign37890_e42725, ((((((((-locals.var_q_zero__blk849_dn4) * locals.var_q_d1_zero__blk850) + (assign37890_e42715 * locals.var_q_d1_zero__blk850_dn4)) * locals.var_q_temp__blk860) + (assign37890_e42717 * locals.var_q_temp__blk860_dn4)) * assign37890_e42724) - (assign37890_e42719 * ((locals.var_q_temp__blk860_dn4 * locals.var_q_temp__blk860) + (locals.var_q_temp__blk860 * locals.var_q_temp__blk860_dn4)))) / (assign37890_e42724 * assign37890_e42724)), ((((((((-locals.var_q_zero__blk849_dn6) * locals.var_q_d1_zero__blk850) + (assign37890_e42715 * locals.var_q_d1_zero__blk850_dn6)) * locals.var_q_temp__blk860) + (assign37890_e42717 * locals.var_q_temp__blk860_dn6)) * assign37890_e42724) - (assign37890_e42719 * ((locals.var_q_temp__blk860_dn6 * locals.var_q_temp__blk860) + (locals.var_q_temp__blk860 * locals.var_q_temp__blk860_dn6)))) / (assign37890_e42724 * assign37890_e42724)), ((((((((-locals.var_q_zero__blk849_dn7) * locals.var_q_d1_zero__blk850) + (assign37890_e42715 * locals.var_q_d1_zero__blk850_dn7)) * locals.var_q_temp__blk860) + (assign37890_e42717 * locals.var_q_temp__blk860_dn7)) * assign37890_e42724) - (assign37890_e42719 * ((locals.var_q_temp__blk860_dn7 * locals.var_q_temp__blk860) + (locals.var_q_temp__blk860 * locals.var_q_temp__blk860_dn7)))) / (assign37890_e42724 * assign37890_e42724)), ((((((((-locals.var_q_zero__blk849_dn8) * locals.var_q_d1_zero__blk850) + (assign37890_e42715 * locals.var_q_d1_zero__blk850_dn8)) * locals.var_q_temp__blk860) + (assign37890_e42717 * locals.var_q_temp__blk860_dn8)) * assign37890_e42724) - (assign37890_e42719 * ((locals.var_q_temp__blk860_dn8 * locals.var_q_temp__blk860) + (locals.var_q_temp__blk860 * locals.var_q_temp__blk860_dn8)))) / (assign37890_e42724 * assign37890_e42724)), ((((((((-locals.var_q_zero__blk849_dn9) * locals.var_q_d1_zero__blk850) + (assign37890_e42715 * locals.var_q_d1_zero__blk850_dn9)) * locals.var_q_temp__blk860) + (assign37890_e42717 * locals.var_q_temp__blk860_dn9)) * assign37890_e42724) - (assign37890_e42719 * ((locals.var_q_temp__blk860_dn9 * locals.var_q_temp__blk860) + (locals.var_q_temp__blk860 * locals.var_q_temp__blk860_dn9)))) / (assign37890_e42724 * assign37890_e42724)), );
            locals.var_q_eps2__blk852_rv = 0.0;
        }
        if (locals.var_guard1080 != 0.0) {
            let assign37900_e42731: f64 = (locals.var_q1d__blk1001 + locals.var_q_eps2__blk852);
            (locals.var_q1d__blk1001, locals.var_q1d__blk1001_dn4, locals.var_q1d__blk1001_dn6, locals.var_q1d__blk1001_dn7, locals.var_q1d__blk1001_dn8, locals.var_q1d__blk1001_dn9, ) = (assign37900_e42731, (locals.var_q1d__blk1001_dn4 + locals.var_q_eps2__blk852_dn4), (locals.var_q1d__blk1001_dn6 + locals.var_q_eps2__blk852_dn6), (locals.var_q1d__blk1001_dn7 + locals.var_q_eps2__blk852_dn7), (locals.var_q1d__blk1001_dn8 + locals.var_q_eps2__blk852_dn8), (locals.var_q1d__blk1001_dn9 + locals.var_q_eps2__blk852_dn9), );
            locals.var_q1d__blk1001_rv = 0.0;
        }
        if (locals.var_guard1080 != 0.0) {
            let assign37910_e42737: f64 = (locals.var_k1__blk932 * locals.var_q1d__blk1001);
            (locals.var_q_k1q1__blk823, locals.var_q_k1q1__blk823_dn4, locals.var_q_k1q1__blk823_dn6, locals.var_q_k1q1__blk823_dn7, locals.var_q_k1q1__blk823_dn8, locals.var_q_k1q1__blk823_dn9, ) = (assign37910_e42737, ((locals.var_k1__blk932_dn4 * locals.var_q1d__blk1001) + (locals.var_k1__blk932 * locals.var_q1d__blk1001_dn4)), ((locals.var_k1__blk932_dn6 * locals.var_q1d__blk1001) + (locals.var_k1__blk932 * locals.var_q1d__blk1001_dn6)), ((locals.var_k1__blk932_dn7 * locals.var_q1d__blk1001) + (locals.var_k1__blk932 * locals.var_q1d__blk1001_dn7)), ((locals.var_k1__blk932_dn8 * locals.var_q1d__blk1001) + (locals.var_k1__blk932 * locals.var_q1d__blk1001_dn8)), ((locals.var_k1__blk932_dn9 * locals.var_q1d__blk1001) + (locals.var_k1__blk932 * locals.var_q1d__blk1001_dn9)), );
            locals.var_q_k1q1__blk823_rv = 0.0;
        }
        let assign37920_e42742: f64 = (locals.var_xg1x__blk930 - locals.var_q1d__blk1001);
        let assign37920_e42744: f64 = (assign37920_e42742 - locals.var_xdeff__blk1000);
        let assign37920_e42746: f64 = if assign37920_e42744 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard1189 = assign37920_e42746;
        locals.var_guard1189_rv = 0.0;
        if ((locals.var_guard1080 != 0.0) && (locals.var_guard1189 != 0.0)) {
            let assign37930_e42752: f64 = (locals.var_xg1x__blk930 - locals.var_q1d__blk1001);
            let assign37930_e42754: f64 = (assign37930_e42752 - locals.var_xdeff__blk1000);
            let assign37930_e42755: f64 = (assign37930_e42754).exp();
            (locals.var_q_temp1__blk814, locals.var_q_temp1__blk814_dn4, locals.var_q_temp1__blk814_dn6, locals.var_q_temp1__blk814_dn7, locals.var_q_temp1__blk814_dn8, locals.var_q_temp1__blk814_dn9, ) = (assign37930_e42755, (assign37930_e42755 * ((locals.var_xg1x__blk930_dn4 - locals.var_q1d__blk1001_dn4) - locals.var_xdeff__blk1000_dn4)), (assign37930_e42755 * ((locals.var_xg1x__blk930_dn6 - locals.var_q1d__blk1001_dn6) - locals.var_xdeff__blk1000_dn6)), (assign37930_e42755 * ((locals.var_xg1x__blk930_dn7 - locals.var_q1d__blk1001_dn7) - locals.var_xdeff__blk1000_dn7)), (assign37930_e42755 * ((locals.var_xg1x__blk930_dn8 - locals.var_q1d__blk1001_dn8) - locals.var_xdeff__blk1000_dn8)), (assign37930_e42755 * ((locals.var_xg1x__blk930_dn9 - locals.var_q1d__blk1001_dn9) - locals.var_xdeff__blk1000_dn9)), );
            locals.var_q_temp1__blk814_rv = 0.0;
        }
        if ((locals.var_guard1080 != 0.0) && (locals.var_guard1189 == 0.0)) {
            let assign37940_e42766: f64 = (locals.var_xg1x__blk930 - locals.var_q1d__blk1001);
            let assign37940_e42768: f64 = (assign37940_e42766 - locals.var_xdeff__blk1000);
            let assign37940_e42770: f64 = (assign37940_e42768 - 80.0);
            let assign37940_e42775: f64 = (locals.var_xg1x__blk930 - locals.var_q1d__blk1001);
            let assign37940_e42777: f64 = (assign37940_e42775 - locals.var_xdeff__blk1000);
            let assign37940_e42779: f64 = (assign37940_e42777 - 80.0);
            let assign37940_e42780: f64 = (0.5 * assign37940_e42779);
            let assign37940_e42784: f64 = (locals.var_xg1x__blk930 - locals.var_q1d__blk1001);
            let assign37940_e42786: f64 = (assign37940_e42784 - locals.var_xdeff__blk1000);
            let assign37940_e42788: f64 = (assign37940_e42786 - 80.0);
            let assign37940_e42790: f64 = (assign37940_e42788 * 0.3333333333333);
            let assign37940_e42791: f64 = (1.0 + assign37940_e42790);
            let assign37940_e42792: f64 = (assign37940_e42780 * assign37940_e42791);
            let assign37940_e42793: f64 = (1.0 + assign37940_e42792);
            let assign37940_e42794: f64 = (assign37940_e42770 * assign37940_e42793);
            let assign37940_e42795: f64 = (1.0 + assign37940_e42794);
            let assign37940_e42796: f64 = (5.54062e34 * assign37940_e42795);
            (locals.var_q_temp1__blk814, locals.var_q_temp1__blk814_dn4, locals.var_q_temp1__blk814_dn6, locals.var_q_temp1__blk814_dn7, locals.var_q_temp1__blk814_dn8, locals.var_q_temp1__blk814_dn9, ) = (assign37940_e42796, (5.54062e34 * ((((locals.var_xg1x__blk930_dn4 - locals.var_q1d__blk1001_dn4) - locals.var_xdeff__blk1000_dn4) * assign37940_e42793) + (assign37940_e42770 * (((0.5 * ((locals.var_xg1x__blk930_dn4 - locals.var_q1d__blk1001_dn4) - locals.var_xdeff__blk1000_dn4)) * assign37940_e42791) + (assign37940_e42780 * (((locals.var_xg1x__blk930_dn4 - locals.var_q1d__blk1001_dn4) - locals.var_xdeff__blk1000_dn4) * 0.3333333333333)))))), (5.54062e34 * ((((locals.var_xg1x__blk930_dn6 - locals.var_q1d__blk1001_dn6) - locals.var_xdeff__blk1000_dn6) * assign37940_e42793) + (assign37940_e42770 * (((0.5 * ((locals.var_xg1x__blk930_dn6 - locals.var_q1d__blk1001_dn6) - locals.var_xdeff__blk1000_dn6)) * assign37940_e42791) + (assign37940_e42780 * (((locals.var_xg1x__blk930_dn6 - locals.var_q1d__blk1001_dn6) - locals.var_xdeff__blk1000_dn6) * 0.3333333333333)))))), (5.54062e34 * ((((locals.var_xg1x__blk930_dn7 - locals.var_q1d__blk1001_dn7) - locals.var_xdeff__blk1000_dn7) * assign37940_e42793) + (assign37940_e42770 * (((0.5 * ((locals.var_xg1x__blk930_dn7 - locals.var_q1d__blk1001_dn7) - locals.var_xdeff__blk1000_dn7)) * assign37940_e42791) + (assign37940_e42780 * (((locals.var_xg1x__blk930_dn7 - locals.var_q1d__blk1001_dn7) - locals.var_xdeff__blk1000_dn7) * 0.3333333333333)))))), (5.54062e34 * ((((locals.var_xg1x__blk930_dn8 - locals.var_q1d__blk1001_dn8) - locals.var_xdeff__blk1000_dn8) * assign37940_e42793) + (assign37940_e42770 * (((0.5 * ((locals.var_xg1x__blk930_dn8 - locals.var_q1d__blk1001_dn8) - locals.var_xdeff__blk1000_dn8)) * assign37940_e42791) + (assign37940_e42780 * (((locals.var_xg1x__blk930_dn8 - locals.var_q1d__blk1001_dn8) - locals.var_xdeff__blk1000_dn8) * 0.3333333333333)))))), (5.54062e34 * ((((locals.var_xg1x__blk930_dn9 - locals.var_q1d__blk1001_dn9) - locals.var_xdeff__blk1000_dn9) * assign37940_e42793) + (assign37940_e42770 * (((0.5 * ((locals.var_xg1x__blk930_dn9 - locals.var_q1d__blk1001_dn9) - locals.var_xdeff__blk1000_dn9)) * assign37940_e42791) + (assign37940_e42780 * (((locals.var_xg1x__blk930_dn9 - locals.var_q1d__blk1001_dn9) - locals.var_xdeff__blk1000_dn9) * 0.3333333333333)))))), );
            locals.var_q_temp1__blk814_rv = 0.0;
        }
        if (locals.var_guard1080 != 0.0) {
            let assign37950_e42802: f64 = (locals.var_a0__blk905 * locals.var_q_temp1__blk814);
            (locals.var_q_aexp__blk824, locals.var_q_aexp__blk824_dn4, locals.var_q_aexp__blk824_dn6, locals.var_q_aexp__blk824_dn7, locals.var_q_aexp__blk824_dn8, locals.var_q_aexp__blk824_dn9, ) = (assign37950_e42802, ((locals.var_a0__blk905_dn4 * locals.var_q_temp1__blk814) + (locals.var_a0__blk905 * locals.var_q_temp1__blk814_dn4)), ((locals.var_a0__blk905_dn6 * locals.var_q_temp1__blk814) + (locals.var_a0__blk905 * locals.var_q_temp1__blk814_dn6)), ((locals.var_a0__blk905_dn7 * locals.var_q_temp1__blk814) + (locals.var_a0__blk905 * locals.var_q_temp1__blk814_dn7)), ((locals.var_a0__blk905_dn8 * locals.var_q_temp1__blk814) + (locals.var_a0__blk905 * locals.var_q_temp1__blk814_dn8)), ((locals.var_a0__blk905_dn9 * locals.var_q_temp1__blk814) + (locals.var_a0__blk905 * locals.var_q_temp1__blk814_dn9)), );
            locals.var_q_aexp__blk824_rv = 0.0;
        }
        if (locals.var_guard1080 != 0.0) {
            let assign37960_e42808: f64 = (locals.var_q_k1q1__blk823 * locals.var_q_k1q1__blk823);
            let assign37960_e42810: f64 = (assign37960_e42808 - locals.var_q_aexp__blk824);
            (locals.var_q_qsq__blk825, locals.var_q_qsq__blk825_dn4, locals.var_q_qsq__blk825_dn6, locals.var_q_qsq__blk825_dn7, locals.var_q_qsq__blk825_dn8, locals.var_q_qsq__blk825_dn9, ) = (assign37960_e42810, (((locals.var_q_k1q1__blk823_dn4 * locals.var_q_k1q1__blk823) + (locals.var_q_k1q1__blk823 * locals.var_q_k1q1__blk823_dn4)) - locals.var_q_aexp__blk824_dn4), (((locals.var_q_k1q1__blk823_dn6 * locals.var_q_k1q1__blk823) + (locals.var_q_k1q1__blk823 * locals.var_q_k1q1__blk823_dn6)) - locals.var_q_aexp__blk824_dn6), (((locals.var_q_k1q1__blk823_dn7 * locals.var_q_k1q1__blk823) + (locals.var_q_k1q1__blk823 * locals.var_q_k1q1__blk823_dn7)) - locals.var_q_aexp__blk824_dn7), (((locals.var_q_k1q1__blk823_dn8 * locals.var_q_k1q1__blk823) + (locals.var_q_k1q1__blk823 * locals.var_q_k1q1__blk823_dn8)) - locals.var_q_aexp__blk824_dn8), (((locals.var_q_k1q1__blk823_dn9 * locals.var_q_k1q1__blk823) + (locals.var_q_k1q1__blk823 * locals.var_q_k1q1__blk823_dn9)) - locals.var_q_aexp__blk824_dn9), );
            locals.var_q_qsq__blk825_rv = 0.0;
        }
        if (locals.var_guard1080 != 0.0) {
            let assign37970_e42816: f64 = (2.0 * locals.var_k1__blk932);
            let assign37970_e42818: f64 = (assign37970_e42816 * locals.var_q_k1q1__blk823);
            let assign37970_e42820: f64 = (assign37970_e42818 + locals.var_q_aexp__blk824);
            (locals.var_q_d1_qsq__blk826, locals.var_q_d1_qsq__blk826_dn4, locals.var_q_d1_qsq__blk826_dn6, locals.var_q_d1_qsq__blk826_dn7, locals.var_q_d1_qsq__blk826_dn8, locals.var_q_d1_qsq__blk826_dn9, ) = (assign37970_e42820, ((((2.0 * locals.var_k1__blk932_dn4) * locals.var_q_k1q1__blk823) + (assign37970_e42816 * locals.var_q_k1q1__blk823_dn4)) + locals.var_q_aexp__blk824_dn4), ((((2.0 * locals.var_k1__blk932_dn6) * locals.var_q_k1q1__blk823) + (assign37970_e42816 * locals.var_q_k1q1__blk823_dn6)) + locals.var_q_aexp__blk824_dn6), ((((2.0 * locals.var_k1__blk932_dn7) * locals.var_q_k1q1__blk823) + (assign37970_e42816 * locals.var_q_k1q1__blk823_dn7)) + locals.var_q_aexp__blk824_dn7), ((((2.0 * locals.var_k1__blk932_dn8) * locals.var_q_k1q1__blk823) + (assign37970_e42816 * locals.var_q_k1q1__blk823_dn8)) + locals.var_q_aexp__blk824_dn8), ((((2.0 * locals.var_k1__blk932_dn9) * locals.var_q_k1q1__blk823) + (assign37970_e42816 * locals.var_q_k1q1__blk823_dn9)) + locals.var_q_aexp__blk824_dn9), );
            locals.var_q_d1_qsq__blk826_rv = 0.0;
        }
        if (locals.var_guard1080 != 0.0) {
            let assign37980_e42826: f64 = (2.0 * locals.var_k1__blk932);
            let assign37980_e42828: f64 = (assign37980_e42826 * locals.var_k1__blk932);
            let assign37980_e42830: f64 = (assign37980_e42828 - locals.var_q_aexp__blk824);
            (locals.var_q_d2_qsq__blk827, locals.var_q_d2_qsq__blk827_dn4, locals.var_q_d2_qsq__blk827_dn6, locals.var_q_d2_qsq__blk827_dn7, locals.var_q_d2_qsq__blk827_dn8, locals.var_q_d2_qsq__blk827_dn9, ) = (assign37980_e42830, ((((2.0 * locals.var_k1__blk932_dn4) * locals.var_k1__blk932) + (assign37980_e42826 * locals.var_k1__blk932_dn4)) - locals.var_q_aexp__blk824_dn4), ((((2.0 * locals.var_k1__blk932_dn6) * locals.var_k1__blk932) + (assign37980_e42826 * locals.var_k1__blk932_dn6)) - locals.var_q_aexp__blk824_dn6), ((((2.0 * locals.var_k1__blk932_dn7) * locals.var_k1__blk932) + (assign37980_e42826 * locals.var_k1__blk932_dn7)) - locals.var_q_aexp__blk824_dn7), ((((2.0 * locals.var_k1__blk932_dn8) * locals.var_k1__blk932) + (assign37980_e42826 * locals.var_k1__blk932_dn8)) - locals.var_q_aexp__blk824_dn8), ((((2.0 * locals.var_k1__blk932_dn9) * locals.var_k1__blk932) + (assign37980_e42826 * locals.var_k1__blk932_dn9)) - locals.var_q_aexp__blk824_dn9), );
            locals.var_q_d2_qsq__blk827_rv = 0.0;
        }
        let assign37990_e42835: f64 = (-0.005);
        let assign37990_e42836: f64 = if locals.var_q_qsq__blk825 < assign37990_e42835 { 1.0 } else { 0.0 };
        locals.var_guard1190 = assign37990_e42836;
        locals.var_guard1190_rv = 0.0;
        if ((locals.var_guard1080 != 0.0) && (locals.var_guard1190 != 0.0)) {
            let assign38000_e42841: f64 = (locals.var_q_qsq__blk825).abs();
            let assign38000_e42842: f64 = (assign38000_e42841).sqrt();
            (locals.var_q_rac_qsq__blk828, locals.var_q_rac_qsq__blk828_dn4, locals.var_q_rac_qsq__blk828_dn6, locals.var_q_rac_qsq__blk828_dn7, locals.var_q_rac_qsq__blk828_dn8, locals.var_q_rac_qsq__blk828_dn9, ) = (assign38000_e42842, (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn4 } else { (-locals.var_q_qsq__blk825_dn4) } / (2.0 * assign38000_e42842)), (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn6 } else { (-locals.var_q_qsq__blk825_dn6) } / (2.0 * assign38000_e42842)), (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn7 } else { (-locals.var_q_qsq__blk825_dn7) } / (2.0 * assign38000_e42842)), (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn8 } else { (-locals.var_q_qsq__blk825_dn8) } / (2.0 * assign38000_e42842)), (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn9 } else { (-locals.var_q_qsq__blk825_dn9) } / (2.0 * assign38000_e42842)), );
            locals.var_q_rac_qsq__blk828_rv = 0.0;
        }
        if ((locals.var_guard1080 != 0.0) && (locals.var_guard1190 != 0.0)) {
            let assign38010_e42851: f64 = (0.5 * locals.var_q_rac_qsq__blk828);
            let assign38010_e42852: f64 = (assign38010_e42851).tan();
            let assign38010_e42853: f64 = (locals.var_q_rac_qsq__blk828 / assign38010_e42852);
            (locals.var_q_qcoth__blk829, locals.var_q_qcoth__blk829_dn4, locals.var_q_qcoth__blk829_dn6, locals.var_q_qcoth__blk829_dn7, locals.var_q_qcoth__blk829_dn8, locals.var_q_qcoth__blk829_dn9, ) = (assign38010_e42853, (((locals.var_q_rac_qsq__blk828_dn4 * assign38010_e42852) - (locals.var_q_rac_qsq__blk828 * ((0.5 * locals.var_q_rac_qsq__blk828_dn4) / ((assign38010_e42851).cos() * (assign38010_e42851).cos())))) / (assign38010_e42852 * assign38010_e42852)), (((locals.var_q_rac_qsq__blk828_dn6 * assign38010_e42852) - (locals.var_q_rac_qsq__blk828 * ((0.5 * locals.var_q_rac_qsq__blk828_dn6) / ((assign38010_e42851).cos() * (assign38010_e42851).cos())))) / (assign38010_e42852 * assign38010_e42852)), (((locals.var_q_rac_qsq__blk828_dn7 * assign38010_e42852) - (locals.var_q_rac_qsq__blk828 * ((0.5 * locals.var_q_rac_qsq__blk828_dn7) / ((assign38010_e42851).cos() * (assign38010_e42851).cos())))) / (assign38010_e42852 * assign38010_e42852)), (((locals.var_q_rac_qsq__blk828_dn8 * assign38010_e42852) - (locals.var_q_rac_qsq__blk828 * ((0.5 * locals.var_q_rac_qsq__blk828_dn8) / ((assign38010_e42851).cos() * (assign38010_e42851).cos())))) / (assign38010_e42852 * assign38010_e42852)), (((locals.var_q_rac_qsq__blk828_dn9 * assign38010_e42852) - (locals.var_q_rac_qsq__blk828 * ((0.5 * locals.var_q_rac_qsq__blk828_dn9) / ((assign38010_e42851).cos() * (assign38010_e42851).cos())))) / (assign38010_e42852 * assign38010_e42852)), );
            locals.var_q_qcoth__blk829_rv = 0.0;
        }
        if ((locals.var_guard1080 != 0.0) && (locals.var_guard1190 != 0.0)) {
            let assign38020_e42861: f64 = (0.25 * locals.var_q_d1_qsq__blk826);
            let assign38020_e42863: f64 = (assign38020_e42861 / locals.var_q_qsq__blk825);
            (locals.var_q_temp1__blk814, locals.var_q_temp1__blk814_dn4, locals.var_q_temp1__blk814_dn6, locals.var_q_temp1__blk814_dn7, locals.var_q_temp1__blk814_dn8, locals.var_q_temp1__blk814_dn9, ) = (assign38020_e42863, ((((0.25 * locals.var_q_d1_qsq__blk826_dn4) * locals.var_q_qsq__blk825) - (assign38020_e42861 * locals.var_q_qsq__blk825_dn4)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((0.25 * locals.var_q_d1_qsq__blk826_dn6) * locals.var_q_qsq__blk825) - (assign38020_e42861 * locals.var_q_qsq__blk825_dn6)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((0.25 * locals.var_q_d1_qsq__blk826_dn7) * locals.var_q_qsq__blk825) - (assign38020_e42861 * locals.var_q_qsq__blk825_dn7)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((0.25 * locals.var_q_d1_qsq__blk826_dn8) * locals.var_q_qsq__blk825) - (assign38020_e42861 * locals.var_q_qsq__blk825_dn8)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((0.25 * locals.var_q_d1_qsq__blk826_dn9) * locals.var_q_qsq__blk825) - (assign38020_e42861 * locals.var_q_qsq__blk825_dn9)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), );
            locals.var_q_temp1__blk814_rv = 0.0;
        }
        if ((locals.var_guard1080 != 0.0) && (locals.var_guard1190 != 0.0)) {
            let assign38030_e42873: f64 = (2.0 - locals.var_q_qcoth__blk829);
            let assign38030_e42874: f64 = (locals.var_q_qcoth__blk829 * assign38030_e42873);
            let assign38030_e42875: f64 = (locals.var_q_qsq__blk825 + assign38030_e42874);
            let assign38030_e42877: f64 = (assign38030_e42875 * locals.var_q_temp1__blk814);
            (locals.var_q_d1_qcoth__blk830, locals.var_q_d1_qcoth__blk830_dn4, locals.var_q_d1_qcoth__blk830_dn6, locals.var_q_d1_qcoth__blk830_dn7, locals.var_q_d1_qcoth__blk830_dn8, locals.var_q_d1_qcoth__blk830_dn9, ) = (assign38030_e42877, (((locals.var_q_qsq__blk825_dn4 + ((locals.var_q_qcoth__blk829_dn4 * assign38030_e42873) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn4)))) * locals.var_q_temp1__blk814) + (assign38030_e42875 * locals.var_q_temp1__blk814_dn4)), (((locals.var_q_qsq__blk825_dn6 + ((locals.var_q_qcoth__blk829_dn6 * assign38030_e42873) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn6)))) * locals.var_q_temp1__blk814) + (assign38030_e42875 * locals.var_q_temp1__blk814_dn6)), (((locals.var_q_qsq__blk825_dn7 + ((locals.var_q_qcoth__blk829_dn7 * assign38030_e42873) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn7)))) * locals.var_q_temp1__blk814) + (assign38030_e42875 * locals.var_q_temp1__blk814_dn7)), (((locals.var_q_qsq__blk825_dn8 + ((locals.var_q_qcoth__blk829_dn8 * assign38030_e42873) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn8)))) * locals.var_q_temp1__blk814) + (assign38030_e42875 * locals.var_q_temp1__blk814_dn8)), (((locals.var_q_qsq__blk825_dn9 + ((locals.var_q_qcoth__blk829_dn9 * assign38030_e42873) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn9)))) * locals.var_q_temp1__blk814) + (assign38030_e42875 * locals.var_q_temp1__blk814_dn9)), );
            locals.var_q_d1_qcoth__blk830_rv = 0.0;
        }
        if ((locals.var_guard1080 != 0.0) && (locals.var_guard1190 != 0.0)) {
            let assign38040_e42886: f64 = (2.0 * locals.var_q_d1_qcoth__blk830);
            let assign38040_e42889: f64 = (1.0 + locals.var_q_qcoth__blk829);
            let assign38040_e42890: f64 = (assign38040_e42886 * assign38040_e42889);
            let assign38040_e42891: f64 = (locals.var_q_d1_qsq__blk826 - assign38040_e42890);
            let assign38040_e42893: f64 = (assign38040_e42891 * locals.var_q_temp1__blk814);
            let assign38040_e42896: f64 = (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827);
            let assign38040_e42898: f64 = (assign38040_e42896 / locals.var_q_d1_qsq__blk826);
            let assign38040_e42899: f64 = (assign38040_e42893 + assign38040_e42898);
            (locals.var_q_d2_qcoth__blk832, locals.var_q_d2_qcoth__blk832_dn4, locals.var_q_d2_qcoth__blk832_dn6, locals.var_q_d2_qcoth__blk832_dn7, locals.var_q_d2_qcoth__blk832_dn8, locals.var_q_d2_qcoth__blk832_dn9, ) = (assign38040_e42899, ((((locals.var_q_d1_qsq__blk826_dn4 - (((2.0 * locals.var_q_d1_qcoth__blk830_dn4) * assign38040_e42889) + (assign38040_e42886 * locals.var_q_qcoth__blk829_dn4))) * locals.var_q_temp1__blk814) + (assign38040_e42891 * locals.var_q_temp1__blk814_dn4)) + (((((locals.var_q_d1_qcoth__blk830_dn4 * locals.var_q_d2_qsq__blk827) + (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827_dn4)) * locals.var_q_d1_qsq__blk826) - (assign38040_e42896 * locals.var_q_d1_qsq__blk826_dn4)) / (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826))), ((((locals.var_q_d1_qsq__blk826_dn6 - (((2.0 * locals.var_q_d1_qcoth__blk830_dn6) * assign38040_e42889) + (assign38040_e42886 * locals.var_q_qcoth__blk829_dn6))) * locals.var_q_temp1__blk814) + (assign38040_e42891 * locals.var_q_temp1__blk814_dn6)) + (((((locals.var_q_d1_qcoth__blk830_dn6 * locals.var_q_d2_qsq__blk827) + (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827_dn6)) * locals.var_q_d1_qsq__blk826) - (assign38040_e42896 * locals.var_q_d1_qsq__blk826_dn6)) / (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826))), ((((locals.var_q_d1_qsq__blk826_dn7 - (((2.0 * locals.var_q_d1_qcoth__blk830_dn7) * assign38040_e42889) + (assign38040_e42886 * locals.var_q_qcoth__blk829_dn7))) * locals.var_q_temp1__blk814) + (assign38040_e42891 * locals.var_q_temp1__blk814_dn7)) + (((((locals.var_q_d1_qcoth__blk830_dn7 * locals.var_q_d2_qsq__blk827) + (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827_dn7)) * locals.var_q_d1_qsq__blk826) - (assign38040_e42896 * locals.var_q_d1_qsq__blk826_dn7)) / (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826))), ((((locals.var_q_d1_qsq__blk826_dn8 - (((2.0 * locals.var_q_d1_qcoth__blk830_dn8) * assign38040_e42889) + (assign38040_e42886 * locals.var_q_qcoth__blk829_dn8))) * locals.var_q_temp1__blk814) + (assign38040_e42891 * locals.var_q_temp1__blk814_dn8)) + (((((locals.var_q_d1_qcoth__blk830_dn8 * locals.var_q_d2_qsq__blk827) + (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827_dn8)) * locals.var_q_d1_qsq__blk826) - (assign38040_e42896 * locals.var_q_d1_qsq__blk826_dn8)) / (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826))), ((((locals.var_q_d1_qsq__blk826_dn9 - (((2.0 * locals.var_q_d1_qcoth__blk830_dn9) * assign38040_e42889) + (assign38040_e42886 * locals.var_q_qcoth__blk829_dn9))) * locals.var_q_temp1__blk814) + (assign38040_e42891 * locals.var_q_temp1__blk814_dn9)) + (((((locals.var_q_d1_qcoth__blk830_dn9 * locals.var_q_d2_qsq__blk827) + (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827_dn9)) * locals.var_q_d1_qsq__blk826) - (assign38040_e42896 * locals.var_q_d1_qsq__blk826_dn9)) / (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826))), );
            locals.var_q_d2_qcoth__blk832_rv = 0.0;
        }
        if ((locals.var_guard1080 != 0.0) && (locals.var_guard1190 != 0.0)) {
            let assign38050_e42908: f64 = (0.5 * locals.var_q_qcoth__blk829);
            let assign38050_e42909: f64 = (1.0 - assign38050_e42908);
            (locals.var_q_temp2__blk815, locals.var_q_temp2__blk815_dn4, locals.var_q_temp2__blk815_dn6, locals.var_q_temp2__blk815_dn7, locals.var_q_temp2__blk815_dn8, locals.var_q_temp2__blk815_dn9, ) = (assign38050_e42909, (-(0.5 * locals.var_q_qcoth__blk829_dn4)), (-(0.5 * locals.var_q_qcoth__blk829_dn6)), (-(0.5 * locals.var_q_qcoth__blk829_dn7)), (-(0.5 * locals.var_q_qcoth__blk829_dn8)), (-(0.5 * locals.var_q_qcoth__blk829_dn9)), );
            locals.var_q_temp2__blk815_rv = 0.0;
        }
        if ((locals.var_guard1080 != 0.0) && (locals.var_guard1190 != 0.0)) {
            let assign38060_e42917: f64 = (locals.var_q_d1_qsq__blk826 / locals.var_q_qsq__blk825);
            let assign38060_e42919: f64 = (assign38060_e42917 * locals.var_q_temp2__blk815);
            (locals.var_q_d1_ln__blk835, locals.var_q_d1_ln__blk835_dn4, locals.var_q_d1_ln__blk835_dn6, locals.var_q_d1_ln__blk835_dn7, locals.var_q_d1_ln__blk835_dn8, locals.var_q_d1_ln__blk835_dn9, ) = (assign38060_e42919, (((((locals.var_q_d1_qsq__blk826_dn4 * locals.var_q_qsq__blk825) - (locals.var_q_d1_qsq__blk826 * locals.var_q_qsq__blk825_dn4)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)) * locals.var_q_temp2__blk815) + (assign38060_e42917 * locals.var_q_temp2__blk815_dn4)), (((((locals.var_q_d1_qsq__blk826_dn6 * locals.var_q_qsq__blk825) - (locals.var_q_d1_qsq__blk826 * locals.var_q_qsq__blk825_dn6)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)) * locals.var_q_temp2__blk815) + (assign38060_e42917 * locals.var_q_temp2__blk815_dn6)), (((((locals.var_q_d1_qsq__blk826_dn7 * locals.var_q_qsq__blk825) - (locals.var_q_d1_qsq__blk826 * locals.var_q_qsq__blk825_dn7)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)) * locals.var_q_temp2__blk815) + (assign38060_e42917 * locals.var_q_temp2__blk815_dn7)), (((((locals.var_q_d1_qsq__blk826_dn8 * locals.var_q_qsq__blk825) - (locals.var_q_d1_qsq__blk826 * locals.var_q_qsq__blk825_dn8)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)) * locals.var_q_temp2__blk815) + (assign38060_e42917 * locals.var_q_temp2__blk815_dn8)), (((((locals.var_q_d1_qsq__blk826_dn9 * locals.var_q_qsq__blk825) - (locals.var_q_d1_qsq__blk826 * locals.var_q_qsq__blk825_dn9)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)) * locals.var_q_temp2__blk815) + (assign38060_e42917 * locals.var_q_temp2__blk815_dn9)), );
            locals.var_q_d1_ln__blk835_rv = 0.0;
        }
        if ((locals.var_guard1080 != 0.0) && (locals.var_guard1190 != 0.0)) {
            let assign38070_e42927: f64 = (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815);
            let assign38070_e42932: f64 = (0.5 * locals.var_q_d1_qcoth__blk830);
            let assign38070_e42933: f64 = (locals.var_q_d1_ln__blk835 + assign38070_e42932);
            let assign38070_e42934: f64 = (locals.var_q_d1_qsq__blk826 * assign38070_e42933);
            let assign38070_e42935: f64 = (assign38070_e42927 - assign38070_e42934);
            let assign38070_e42937: f64 = (assign38070_e42935 / locals.var_q_qsq__blk825);
            (locals.var_q_d2_ln__blk836, locals.var_q_d2_ln__blk836_dn4, locals.var_q_d2_ln__blk836_dn6, locals.var_q_d2_ln__blk836_dn7, locals.var_q_d2_ln__blk836_dn8, locals.var_q_d2_ln__blk836_dn9, ) = (assign38070_e42937, ((((((locals.var_q_d2_qsq__blk827_dn4 * locals.var_q_temp2__blk815) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815_dn4)) - ((locals.var_q_d1_qsq__blk826_dn4 * assign38070_e42933) + (locals.var_q_d1_qsq__blk826 * (locals.var_q_d1_ln__blk835_dn4 + (0.5 * locals.var_q_d1_qcoth__blk830_dn4))))) * locals.var_q_qsq__blk825) - (assign38070_e42935 * locals.var_q_qsq__blk825_dn4)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((((locals.var_q_d2_qsq__blk827_dn6 * locals.var_q_temp2__blk815) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815_dn6)) - ((locals.var_q_d1_qsq__blk826_dn6 * assign38070_e42933) + (locals.var_q_d1_qsq__blk826 * (locals.var_q_d1_ln__blk835_dn6 + (0.5 * locals.var_q_d1_qcoth__blk830_dn6))))) * locals.var_q_qsq__blk825) - (assign38070_e42935 * locals.var_q_qsq__blk825_dn6)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((((locals.var_q_d2_qsq__blk827_dn7 * locals.var_q_temp2__blk815) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815_dn7)) - ((locals.var_q_d1_qsq__blk826_dn7 * assign38070_e42933) + (locals.var_q_d1_qsq__blk826 * (locals.var_q_d1_ln__blk835_dn7 + (0.5 * locals.var_q_d1_qcoth__blk830_dn7))))) * locals.var_q_qsq__blk825) - (assign38070_e42935 * locals.var_q_qsq__blk825_dn7)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((((locals.var_q_d2_qsq__blk827_dn8 * locals.var_q_temp2__blk815) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815_dn8)) - ((locals.var_q_d1_qsq__blk826_dn8 * assign38070_e42933) + (locals.var_q_d1_qsq__blk826 * (locals.var_q_d1_ln__blk835_dn8 + (0.5 * locals.var_q_d1_qcoth__blk830_dn8))))) * locals.var_q_qsq__blk825) - (assign38070_e42935 * locals.var_q_qsq__blk825_dn8)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((((locals.var_q_d2_qsq__blk827_dn9 * locals.var_q_temp2__blk815) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815_dn9)) - ((locals.var_q_d1_qsq__blk826_dn9 * assign38070_e42933) + (locals.var_q_d1_qsq__blk826 * (locals.var_q_d1_ln__blk835_dn9 + (0.5 * locals.var_q_d1_qcoth__blk830_dn9))))) * locals.var_q_qsq__blk825) - (assign38070_e42935 * locals.var_q_qsq__blk825_dn9)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), );
            locals.var_q_d2_ln__blk836_rv = 0.0;
        }
        let assign38080_e42942: f64 = if locals.var_q_qsq__blk825 > 0.005 { 1.0 } else { 0.0 };
        locals.var_guard1191 = assign38080_e42942;
        locals.var_guard1191_rv = 0.0;
        if (((locals.var_guard1080 != 0.0) && (locals.var_guard1190 == 0.0)) && (locals.var_guard1191 != 0.0)) {
            let assign38090_e42950: f64 = (locals.var_q_qsq__blk825).abs();
            let assign38090_e42951: f64 = (assign38090_e42950).sqrt();
            (locals.var_q_rac_qsq__blk828, locals.var_q_rac_qsq__blk828_dn4, locals.var_q_rac_qsq__blk828_dn6, locals.var_q_rac_qsq__blk828_dn7, locals.var_q_rac_qsq__blk828_dn8, locals.var_q_rac_qsq__blk828_dn9, ) = (assign38090_e42951, (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn4 } else { (-locals.var_q_qsq__blk825_dn4) } / (2.0 * assign38090_e42951)), (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn6 } else { (-locals.var_q_qsq__blk825_dn6) } / (2.0 * assign38090_e42951)), (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn7 } else { (-locals.var_q_qsq__blk825_dn7) } / (2.0 * assign38090_e42951)), (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn8 } else { (-locals.var_q_qsq__blk825_dn8) } / (2.0 * assign38090_e42951)), (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn9 } else { (-locals.var_q_qsq__blk825_dn9) } / (2.0 * assign38090_e42951)), );
            locals.var_q_rac_qsq__blk828_rv = 0.0;
        }
        if (((locals.var_guard1080 != 0.0) && (locals.var_guard1190 == 0.0)) && (locals.var_guard1191 != 0.0)) {
            let assign38100_e42961: f64 = (-locals.var_q_rac_qsq__blk828);
            let assign38100_e42962: f64 = (assign38100_e42961).exp();
            (locals.var_q_invexpq__blk831, locals.var_q_invexpq__blk831_dn4, locals.var_q_invexpq__blk831_dn6, locals.var_q_invexpq__blk831_dn7, locals.var_q_invexpq__blk831_dn8, locals.var_q_invexpq__blk831_dn9, ) = (assign38100_e42962, (assign38100_e42962 * (-locals.var_q_rac_qsq__blk828_dn4)), (assign38100_e42962 * (-locals.var_q_rac_qsq__blk828_dn6)), (assign38100_e42962 * (-locals.var_q_rac_qsq__blk828_dn7)), (assign38100_e42962 * (-locals.var_q_rac_qsq__blk828_dn8)), (assign38100_e42962 * (-locals.var_q_rac_qsq__blk828_dn9)), );
            locals.var_q_invexpq__blk831_rv = 0.0;
        }
        if (((locals.var_guard1080 != 0.0) && (locals.var_guard1190 == 0.0)) && (locals.var_guard1191 != 0.0)) {
            let assign38110_e42974: f64 = (1.0 + locals.var_q_invexpq__blk831);
            let assign38110_e42975: f64 = (locals.var_q_rac_qsq__blk828 * assign38110_e42974);
            let assign38110_e42978: f64 = (1.0 - locals.var_q_invexpq__blk831);
            let assign38110_e42979: f64 = (assign38110_e42975 / assign38110_e42978);
            (locals.var_q_qcoth__blk829, locals.var_q_qcoth__blk829_dn4, locals.var_q_qcoth__blk829_dn6, locals.var_q_qcoth__blk829_dn7, locals.var_q_qcoth__blk829_dn8, locals.var_q_qcoth__blk829_dn9, ) = (assign38110_e42979, (((((locals.var_q_rac_qsq__blk828_dn4 * assign38110_e42974) + (locals.var_q_rac_qsq__blk828 * locals.var_q_invexpq__blk831_dn4)) * assign38110_e42978) - (assign38110_e42975 * (-locals.var_q_invexpq__blk831_dn4))) / (assign38110_e42978 * assign38110_e42978)), (((((locals.var_q_rac_qsq__blk828_dn6 * assign38110_e42974) + (locals.var_q_rac_qsq__blk828 * locals.var_q_invexpq__blk831_dn6)) * assign38110_e42978) - (assign38110_e42975 * (-locals.var_q_invexpq__blk831_dn6))) / (assign38110_e42978 * assign38110_e42978)), (((((locals.var_q_rac_qsq__blk828_dn7 * assign38110_e42974) + (locals.var_q_rac_qsq__blk828 * locals.var_q_invexpq__blk831_dn7)) * assign38110_e42978) - (assign38110_e42975 * (-locals.var_q_invexpq__blk831_dn7))) / (assign38110_e42978 * assign38110_e42978)), (((((locals.var_q_rac_qsq__blk828_dn8 * assign38110_e42974) + (locals.var_q_rac_qsq__blk828 * locals.var_q_invexpq__blk831_dn8)) * assign38110_e42978) - (assign38110_e42975 * (-locals.var_q_invexpq__blk831_dn8))) / (assign38110_e42978 * assign38110_e42978)), (((((locals.var_q_rac_qsq__blk828_dn9 * assign38110_e42974) + (locals.var_q_rac_qsq__blk828 * locals.var_q_invexpq__blk831_dn9)) * assign38110_e42978) - (assign38110_e42975 * (-locals.var_q_invexpq__blk831_dn9))) / (assign38110_e42978 * assign38110_e42978)), );
            locals.var_q_qcoth__blk829_rv = 0.0;
        }
        if (((locals.var_guard1080 != 0.0) && (locals.var_guard1190 == 0.0)) && (locals.var_guard1191 != 0.0)) {
            let assign38120_e42990: f64 = (0.25 * locals.var_q_d1_qsq__blk826);
            let assign38120_e42992: f64 = (assign38120_e42990 / locals.var_q_qsq__blk825);
            (locals.var_q_temp1__blk814, locals.var_q_temp1__blk814_dn4, locals.var_q_temp1__blk814_dn6, locals.var_q_temp1__blk814_dn7, locals.var_q_temp1__blk814_dn8, locals.var_q_temp1__blk814_dn9, ) = (assign38120_e42992, ((((0.25 * locals.var_q_d1_qsq__blk826_dn4) * locals.var_q_qsq__blk825) - (assign38120_e42990 * locals.var_q_qsq__blk825_dn4)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((0.25 * locals.var_q_d1_qsq__blk826_dn6) * locals.var_q_qsq__blk825) - (assign38120_e42990 * locals.var_q_qsq__blk825_dn6)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((0.25 * locals.var_q_d1_qsq__blk826_dn7) * locals.var_q_qsq__blk825) - (assign38120_e42990 * locals.var_q_qsq__blk825_dn7)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((0.25 * locals.var_q_d1_qsq__blk826_dn8) * locals.var_q_qsq__blk825) - (assign38120_e42990 * locals.var_q_qsq__blk825_dn8)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((0.25 * locals.var_q_d1_qsq__blk826_dn9) * locals.var_q_qsq__blk825) - (assign38120_e42990 * locals.var_q_qsq__blk825_dn9)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), );
            locals.var_q_temp1__blk814_rv = 0.0;
        }
        if (((locals.var_guard1080 != 0.0) && (locals.var_guard1190 == 0.0)) && (locals.var_guard1191 != 0.0)) {
            let assign38130_e43005: f64 = (2.0 - locals.var_q_qcoth__blk829);
            let assign38130_e43006: f64 = (locals.var_q_qcoth__blk829 * assign38130_e43005);
            let assign38130_e43007: f64 = (locals.var_q_qsq__blk825 + assign38130_e43006);
            let assign38130_e43009: f64 = (assign38130_e43007 * locals.var_q_temp1__blk814);
            (locals.var_q_d1_qcoth__blk830, locals.var_q_d1_qcoth__blk830_dn4, locals.var_q_d1_qcoth__blk830_dn6, locals.var_q_d1_qcoth__blk830_dn7, locals.var_q_d1_qcoth__blk830_dn8, locals.var_q_d1_qcoth__blk830_dn9, ) = (assign38130_e43009, (((locals.var_q_qsq__blk825_dn4 + ((locals.var_q_qcoth__blk829_dn4 * assign38130_e43005) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn4)))) * locals.var_q_temp1__blk814) + (assign38130_e43007 * locals.var_q_temp1__blk814_dn4)), (((locals.var_q_qsq__blk825_dn6 + ((locals.var_q_qcoth__blk829_dn6 * assign38130_e43005) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn6)))) * locals.var_q_temp1__blk814) + (assign38130_e43007 * locals.var_q_temp1__blk814_dn6)), (((locals.var_q_qsq__blk825_dn7 + ((locals.var_q_qcoth__blk829_dn7 * assign38130_e43005) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn7)))) * locals.var_q_temp1__blk814) + (assign38130_e43007 * locals.var_q_temp1__blk814_dn7)), (((locals.var_q_qsq__blk825_dn8 + ((locals.var_q_qcoth__blk829_dn8 * assign38130_e43005) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn8)))) * locals.var_q_temp1__blk814) + (assign38130_e43007 * locals.var_q_temp1__blk814_dn8)), (((locals.var_q_qsq__blk825_dn9 + ((locals.var_q_qcoth__blk829_dn9 * assign38130_e43005) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn9)))) * locals.var_q_temp1__blk814) + (assign38130_e43007 * locals.var_q_temp1__blk814_dn9)), );
            locals.var_q_d1_qcoth__blk830_rv = 0.0;
        }
        if (((locals.var_guard1080 != 0.0) && (locals.var_guard1190 == 0.0)) && (locals.var_guard1191 != 0.0)) {
            let assign38140_e43021: f64 = (2.0 * locals.var_q_d1_qcoth__blk830);
            let assign38140_e43024: f64 = (1.0 + locals.var_q_qcoth__blk829);
            let assign38140_e43025: f64 = (assign38140_e43021 * assign38140_e43024);
            let assign38140_e43026: f64 = (locals.var_q_d1_qsq__blk826 - assign38140_e43025);
            let assign38140_e43028: f64 = (assign38140_e43026 * locals.var_q_temp1__blk814);
            let assign38140_e43031: f64 = (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827);
            let assign38140_e43033: f64 = (assign38140_e43031 / locals.var_q_d1_qsq__blk826);
            let assign38140_e43034: f64 = (assign38140_e43028 + assign38140_e43033);
            (locals.var_q_d2_qcoth__blk832, locals.var_q_d2_qcoth__blk832_dn4, locals.var_q_d2_qcoth__blk832_dn6, locals.var_q_d2_qcoth__blk832_dn7, locals.var_q_d2_qcoth__blk832_dn8, locals.var_q_d2_qcoth__blk832_dn9, ) = (assign38140_e43034, ((((locals.var_q_d1_qsq__blk826_dn4 - (((2.0 * locals.var_q_d1_qcoth__blk830_dn4) * assign38140_e43024) + (assign38140_e43021 * locals.var_q_qcoth__blk829_dn4))) * locals.var_q_temp1__blk814) + (assign38140_e43026 * locals.var_q_temp1__blk814_dn4)) + (((((locals.var_q_d1_qcoth__blk830_dn4 * locals.var_q_d2_qsq__blk827) + (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827_dn4)) * locals.var_q_d1_qsq__blk826) - (assign38140_e43031 * locals.var_q_d1_qsq__blk826_dn4)) / (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826))), ((((locals.var_q_d1_qsq__blk826_dn6 - (((2.0 * locals.var_q_d1_qcoth__blk830_dn6) * assign38140_e43024) + (assign38140_e43021 * locals.var_q_qcoth__blk829_dn6))) * locals.var_q_temp1__blk814) + (assign38140_e43026 * locals.var_q_temp1__blk814_dn6)) + (((((locals.var_q_d1_qcoth__blk830_dn6 * locals.var_q_d2_qsq__blk827) + (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827_dn6)) * locals.var_q_d1_qsq__blk826) - (assign38140_e43031 * locals.var_q_d1_qsq__blk826_dn6)) / (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826))), ((((locals.var_q_d1_qsq__blk826_dn7 - (((2.0 * locals.var_q_d1_qcoth__blk830_dn7) * assign38140_e43024) + (assign38140_e43021 * locals.var_q_qcoth__blk829_dn7))) * locals.var_q_temp1__blk814) + (assign38140_e43026 * locals.var_q_temp1__blk814_dn7)) + (((((locals.var_q_d1_qcoth__blk830_dn7 * locals.var_q_d2_qsq__blk827) + (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827_dn7)) * locals.var_q_d1_qsq__blk826) - (assign38140_e43031 * locals.var_q_d1_qsq__blk826_dn7)) / (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826))), ((((locals.var_q_d1_qsq__blk826_dn8 - (((2.0 * locals.var_q_d1_qcoth__blk830_dn8) * assign38140_e43024) + (assign38140_e43021 * locals.var_q_qcoth__blk829_dn8))) * locals.var_q_temp1__blk814) + (assign38140_e43026 * locals.var_q_temp1__blk814_dn8)) + (((((locals.var_q_d1_qcoth__blk830_dn8 * locals.var_q_d2_qsq__blk827) + (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827_dn8)) * locals.var_q_d1_qsq__blk826) - (assign38140_e43031 * locals.var_q_d1_qsq__blk826_dn8)) / (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826))), ((((locals.var_q_d1_qsq__blk826_dn9 - (((2.0 * locals.var_q_d1_qcoth__blk830_dn9) * assign38140_e43024) + (assign38140_e43021 * locals.var_q_qcoth__blk829_dn9))) * locals.var_q_temp1__blk814) + (assign38140_e43026 * locals.var_q_temp1__blk814_dn9)) + (((((locals.var_q_d1_qcoth__blk830_dn9 * locals.var_q_d2_qsq__blk827) + (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827_dn9)) * locals.var_q_d1_qsq__blk826) - (assign38140_e43031 * locals.var_q_d1_qsq__blk826_dn9)) / (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826))), );
            locals.var_q_d2_qcoth__blk832_rv = 0.0;
        }
        if (((locals.var_guard1080 != 0.0) && (locals.var_guard1190 == 0.0)) && (locals.var_guard1191 != 0.0)) {
            let assign38150_e43046: f64 = (0.5 * locals.var_q_qcoth__blk829);
            let assign38150_e43047: f64 = (1.0 - assign38150_e43046);
            (locals.var_q_temp2__blk815, locals.var_q_temp2__blk815_dn4, locals.var_q_temp2__blk815_dn6, locals.var_q_temp2__blk815_dn7, locals.var_q_temp2__blk815_dn8, locals.var_q_temp2__blk815_dn9, ) = (assign38150_e43047, (-(0.5 * locals.var_q_qcoth__blk829_dn4)), (-(0.5 * locals.var_q_qcoth__blk829_dn6)), (-(0.5 * locals.var_q_qcoth__blk829_dn7)), (-(0.5 * locals.var_q_qcoth__blk829_dn8)), (-(0.5 * locals.var_q_qcoth__blk829_dn9)), );
            locals.var_q_temp2__blk815_rv = 0.0;
        }
        if (((locals.var_guard1080 != 0.0) && (locals.var_guard1190 == 0.0)) && (locals.var_guard1191 != 0.0)) {
            let assign38160_e43058: f64 = (locals.var_q_d1_qsq__blk826 / locals.var_q_qsq__blk825);
            let assign38160_e43060: f64 = (assign38160_e43058 * locals.var_q_temp2__blk815);
            (locals.var_q_d1_ln__blk835, locals.var_q_d1_ln__blk835_dn4, locals.var_q_d1_ln__blk835_dn6, locals.var_q_d1_ln__blk835_dn7, locals.var_q_d1_ln__blk835_dn8, locals.var_q_d1_ln__blk835_dn9, ) = (assign38160_e43060, (((((locals.var_q_d1_qsq__blk826_dn4 * locals.var_q_qsq__blk825) - (locals.var_q_d1_qsq__blk826 * locals.var_q_qsq__blk825_dn4)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)) * locals.var_q_temp2__blk815) + (assign38160_e43058 * locals.var_q_temp2__blk815_dn4)), (((((locals.var_q_d1_qsq__blk826_dn6 * locals.var_q_qsq__blk825) - (locals.var_q_d1_qsq__blk826 * locals.var_q_qsq__blk825_dn6)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)) * locals.var_q_temp2__blk815) + (assign38160_e43058 * locals.var_q_temp2__blk815_dn6)), (((((locals.var_q_d1_qsq__blk826_dn7 * locals.var_q_qsq__blk825) - (locals.var_q_d1_qsq__blk826 * locals.var_q_qsq__blk825_dn7)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)) * locals.var_q_temp2__blk815) + (assign38160_e43058 * locals.var_q_temp2__blk815_dn7)), (((((locals.var_q_d1_qsq__blk826_dn8 * locals.var_q_qsq__blk825) - (locals.var_q_d1_qsq__blk826 * locals.var_q_qsq__blk825_dn8)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)) * locals.var_q_temp2__blk815) + (assign38160_e43058 * locals.var_q_temp2__blk815_dn8)), (((((locals.var_q_d1_qsq__blk826_dn9 * locals.var_q_qsq__blk825) - (locals.var_q_d1_qsq__blk826 * locals.var_q_qsq__blk825_dn9)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)) * locals.var_q_temp2__blk815) + (assign38160_e43058 * locals.var_q_temp2__blk815_dn9)), );
            locals.var_q_d1_ln__blk835_rv = 0.0;
        }
        if (((locals.var_guard1080 != 0.0) && (locals.var_guard1190 == 0.0)) && (locals.var_guard1191 != 0.0)) {
            let assign38170_e43071: f64 = (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815);
            let assign38170_e43076: f64 = (0.5 * locals.var_q_d1_qcoth__blk830);
            let assign38170_e43077: f64 = (locals.var_q_d1_ln__blk835 + assign38170_e43076);
            let assign38170_e43078: f64 = (locals.var_q_d1_qsq__blk826 * assign38170_e43077);
            let assign38170_e43079: f64 = (assign38170_e43071 - assign38170_e43078);
            let assign38170_e43081: f64 = (assign38170_e43079 / locals.var_q_qsq__blk825);
            (locals.var_q_d2_ln__blk836, locals.var_q_d2_ln__blk836_dn4, locals.var_q_d2_ln__blk836_dn6, locals.var_q_d2_ln__blk836_dn7, locals.var_q_d2_ln__blk836_dn8, locals.var_q_d2_ln__blk836_dn9, ) = (assign38170_e43081, ((((((locals.var_q_d2_qsq__blk827_dn4 * locals.var_q_temp2__blk815) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815_dn4)) - ((locals.var_q_d1_qsq__blk826_dn4 * assign38170_e43077) + (locals.var_q_d1_qsq__blk826 * (locals.var_q_d1_ln__blk835_dn4 + (0.5 * locals.var_q_d1_qcoth__blk830_dn4))))) * locals.var_q_qsq__blk825) - (assign38170_e43079 * locals.var_q_qsq__blk825_dn4)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((((locals.var_q_d2_qsq__blk827_dn6 * locals.var_q_temp2__blk815) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815_dn6)) - ((locals.var_q_d1_qsq__blk826_dn6 * assign38170_e43077) + (locals.var_q_d1_qsq__blk826 * (locals.var_q_d1_ln__blk835_dn6 + (0.5 * locals.var_q_d1_qcoth__blk830_dn6))))) * locals.var_q_qsq__blk825) - (assign38170_e43079 * locals.var_q_qsq__blk825_dn6)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((((locals.var_q_d2_qsq__blk827_dn7 * locals.var_q_temp2__blk815) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815_dn7)) - ((locals.var_q_d1_qsq__blk826_dn7 * assign38170_e43077) + (locals.var_q_d1_qsq__blk826 * (locals.var_q_d1_ln__blk835_dn7 + (0.5 * locals.var_q_d1_qcoth__blk830_dn7))))) * locals.var_q_qsq__blk825) - (assign38170_e43079 * locals.var_q_qsq__blk825_dn7)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((((locals.var_q_d2_qsq__blk827_dn8 * locals.var_q_temp2__blk815) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815_dn8)) - ((locals.var_q_d1_qsq__blk826_dn8 * assign38170_e43077) + (locals.var_q_d1_qsq__blk826 * (locals.var_q_d1_ln__blk835_dn8 + (0.5 * locals.var_q_d1_qcoth__blk830_dn8))))) * locals.var_q_qsq__blk825) - (assign38170_e43079 * locals.var_q_qsq__blk825_dn8)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((((locals.var_q_d2_qsq__blk827_dn9 * locals.var_q_temp2__blk815) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815_dn9)) - ((locals.var_q_d1_qsq__blk826_dn9 * assign38170_e43077) + (locals.var_q_d1_qsq__blk826 * (locals.var_q_d1_ln__blk835_dn9 + (0.5 * locals.var_q_d1_qcoth__blk830_dn9))))) * locals.var_q_qsq__blk825) - (assign38170_e43079 * locals.var_q_qsq__blk825_dn9)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), );
            locals.var_q_d2_ln__blk836_rv = 0.0;
        }
        if (((locals.var_guard1080 != 0.0) && (locals.var_guard1190 == 0.0)) && (locals.var_guard1191 == 0.0)) {
            let assign38180_e43095: f64 = (locals.var_q_qsq__blk825 * 0.0166666666667);
            let assign38180_e43099: f64 = (locals.var_q_qsq__blk825 * 0.0238095238095);
            let assign38180_e43103: f64 = (locals.var_q_qsq__blk825 * 0.025);
            let assign38180_e43104: f64 = (1.0 - assign38180_e43103);
            let assign38180_e43105: f64 = (assign38180_e43099 * assign38180_e43104);
            let assign38180_e43106: f64 = (1.0 - assign38180_e43105);
            let assign38180_e43107: f64 = (assign38180_e43095 * assign38180_e43106);
            let assign38180_e43108: f64 = (1.0 - assign38180_e43107);
            let assign38180_e43109: f64 = (0.1666666666667 * assign38180_e43108);
            (locals.var_q_temp3__blk816, locals.var_q_temp3__blk816_dn4, locals.var_q_temp3__blk816_dn6, locals.var_q_temp3__blk816_dn7, locals.var_q_temp3__blk816_dn8, locals.var_q_temp3__blk816_dn9, ) = (assign38180_e43109, (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn4 * 0.0166666666667) * assign38180_e43106) + (assign38180_e43095 * (-(((locals.var_q_qsq__blk825_dn4 * 0.0238095238095) * assign38180_e43104) + (assign38180_e43099 * (-(locals.var_q_qsq__blk825_dn4 * 0.025))))))))), (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn6 * 0.0166666666667) * assign38180_e43106) + (assign38180_e43095 * (-(((locals.var_q_qsq__blk825_dn6 * 0.0238095238095) * assign38180_e43104) + (assign38180_e43099 * (-(locals.var_q_qsq__blk825_dn6 * 0.025))))))))), (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn7 * 0.0166666666667) * assign38180_e43106) + (assign38180_e43095 * (-(((locals.var_q_qsq__blk825_dn7 * 0.0238095238095) * assign38180_e43104) + (assign38180_e43099 * (-(locals.var_q_qsq__blk825_dn7 * 0.025))))))))), (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn8 * 0.0166666666667) * assign38180_e43106) + (assign38180_e43095 * (-(((locals.var_q_qsq__blk825_dn8 * 0.0238095238095) * assign38180_e43104) + (assign38180_e43099 * (-(locals.var_q_qsq__blk825_dn8 * 0.025))))))))), (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn9 * 0.0166666666667) * assign38180_e43106) + (assign38180_e43095 * (-(((locals.var_q_qsq__blk825_dn9 * 0.0238095238095) * assign38180_e43104) + (assign38180_e43099 * (-(locals.var_q_qsq__blk825_dn9 * 0.025))))))))), );
            locals.var_q_temp3__blk816_rv = 0.0;
        }
        if (((locals.var_guard1080 != 0.0) && (locals.var_guard1190 == 0.0)) && (locals.var_guard1191 == 0.0)) {
            let assign38190_e43122: f64 = (locals.var_q_qsq__blk825 * locals.var_q_temp3__blk816);
            let assign38190_e43123: f64 = (2.0 + assign38190_e43122);
            (locals.var_q_qcoth__blk829, locals.var_q_qcoth__blk829_dn4, locals.var_q_qcoth__blk829_dn6, locals.var_q_qcoth__blk829_dn7, locals.var_q_qcoth__blk829_dn8, locals.var_q_qcoth__blk829_dn9, ) = (assign38190_e43123, ((locals.var_q_qsq__blk825_dn4 * locals.var_q_temp3__blk816) + (locals.var_q_qsq__blk825 * locals.var_q_temp3__blk816_dn4)), ((locals.var_q_qsq__blk825_dn6 * locals.var_q_temp3__blk816) + (locals.var_q_qsq__blk825 * locals.var_q_temp3__blk816_dn6)), ((locals.var_q_qsq__blk825_dn7 * locals.var_q_temp3__blk816) + (locals.var_q_qsq__blk825 * locals.var_q_temp3__blk816_dn7)), ((locals.var_q_qsq__blk825_dn8 * locals.var_q_temp3__blk816) + (locals.var_q_qsq__blk825 * locals.var_q_temp3__blk816_dn8)), ((locals.var_q_qsq__blk825_dn9 * locals.var_q_temp3__blk816) + (locals.var_q_qsq__blk825 * locals.var_q_temp3__blk816_dn9)), );
            locals.var_q_qcoth__blk829_rv = 0.0;
        }
        if (((locals.var_guard1080 != 0.0) && (locals.var_guard1190 == 0.0)) && (locals.var_guard1191 == 0.0)) {
            let assign38200_e43137: f64 = (locals.var_q_qsq__blk825 * 0.0333333333333);
            let assign38200_e43141: f64 = (locals.var_q_qsq__blk825 * 0.0357142857143);
            let assign38200_e43145: f64 = (locals.var_q_qsq__blk825 * 0.0333333333333);
            let assign38200_e43146: f64 = (1.0 - assign38200_e43145);
            let assign38200_e43147: f64 = (assign38200_e43141 * assign38200_e43146);
            let assign38200_e43148: f64 = (1.0 - assign38200_e43147);
            let assign38200_e43149: f64 = (assign38200_e43137 * assign38200_e43148);
            let assign38200_e43150: f64 = (1.0 - assign38200_e43149);
            let assign38200_e43151: f64 = (0.1666666666667 * assign38200_e43150);
            (locals.var_q_temp1__blk814, locals.var_q_temp1__blk814_dn4, locals.var_q_temp1__blk814_dn6, locals.var_q_temp1__blk814_dn7, locals.var_q_temp1__blk814_dn8, locals.var_q_temp1__blk814_dn9, ) = (assign38200_e43151, (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn4 * 0.0333333333333) * assign38200_e43148) + (assign38200_e43137 * (-(((locals.var_q_qsq__blk825_dn4 * 0.0357142857143) * assign38200_e43146) + (assign38200_e43141 * (-(locals.var_q_qsq__blk825_dn4 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn6 * 0.0333333333333) * assign38200_e43148) + (assign38200_e43137 * (-(((locals.var_q_qsq__blk825_dn6 * 0.0357142857143) * assign38200_e43146) + (assign38200_e43141 * (-(locals.var_q_qsq__blk825_dn6 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn7 * 0.0333333333333) * assign38200_e43148) + (assign38200_e43137 * (-(((locals.var_q_qsq__blk825_dn7 * 0.0357142857143) * assign38200_e43146) + (assign38200_e43141 * (-(locals.var_q_qsq__blk825_dn7 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn8 * 0.0333333333333) * assign38200_e43148) + (assign38200_e43137 * (-(((locals.var_q_qsq__blk825_dn8 * 0.0357142857143) * assign38200_e43146) + (assign38200_e43141 * (-(locals.var_q_qsq__blk825_dn8 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn9 * 0.0333333333333) * assign38200_e43148) + (assign38200_e43137 * (-(((locals.var_q_qsq__blk825_dn9 * 0.0357142857143) * assign38200_e43146) + (assign38200_e43141 * (-(locals.var_q_qsq__blk825_dn9 * 0.0333333333333))))))))), );
            locals.var_q_temp1__blk814_rv = 0.0;
        }
        if (((locals.var_guard1080 != 0.0) && (locals.var_guard1190 == 0.0)) && (locals.var_guard1191 == 0.0)) {
            let assign38210_e43163: f64 = (locals.var_q_d1_qsq__blk826 * locals.var_q_temp1__blk814);
            (locals.var_q_d1_qcoth__blk830, locals.var_q_d1_qcoth__blk830_dn4, locals.var_q_d1_qcoth__blk830_dn6, locals.var_q_d1_qcoth__blk830_dn7, locals.var_q_d1_qcoth__blk830_dn8, locals.var_q_d1_qcoth__blk830_dn9, ) = (assign38210_e43163, ((locals.var_q_d1_qsq__blk826_dn4 * locals.var_q_temp1__blk814) + (locals.var_q_d1_qsq__blk826 * locals.var_q_temp1__blk814_dn4)), ((locals.var_q_d1_qsq__blk826_dn6 * locals.var_q_temp1__blk814) + (locals.var_q_d1_qsq__blk826 * locals.var_q_temp1__blk814_dn6)), ((locals.var_q_d1_qsq__blk826_dn7 * locals.var_q_temp1__blk814) + (locals.var_q_d1_qsq__blk826 * locals.var_q_temp1__blk814_dn7)), ((locals.var_q_d1_qsq__blk826_dn8 * locals.var_q_temp1__blk814) + (locals.var_q_d1_qsq__blk826 * locals.var_q_temp1__blk814_dn8)), ((locals.var_q_d1_qsq__blk826_dn9 * locals.var_q_temp1__blk814) + (locals.var_q_d1_qsq__blk826 * locals.var_q_temp1__blk814_dn9)), );
            locals.var_q_d1_qcoth__blk830_rv = 0.0;
        }
        if (((locals.var_guard1080 != 0.0) && (locals.var_guard1190 == 0.0)) && (locals.var_guard1191 == 0.0)) {
            let assign38220_e43177: f64 = (locals.var_q_qsq__blk825 * 0.0714285714286);
            let assign38220_e43181: f64 = (0.05 * locals.var_q_qsq__blk825);
            let assign38220_e43185: f64 = (0.0420875420875421 * locals.var_q_qsq__blk825);
            let assign38220_e43186: f64 = (1.0 - assign38220_e43185);
            let assign38220_e43187: f64 = (assign38220_e43181 * assign38220_e43186);
            let assign38220_e43188: f64 = (1.0 - assign38220_e43187);
            let assign38220_e43189: f64 = (assign38220_e43177 * assign38220_e43188);
            let assign38220_e43190: f64 = (1.0 - assign38220_e43189);
            let assign38220_e43191: f64 = (0.0055555555556 * assign38220_e43190);
            (locals.var_q_temp2__blk815, locals.var_q_temp2__blk815_dn4, locals.var_q_temp2__blk815_dn6, locals.var_q_temp2__blk815_dn7, locals.var_q_temp2__blk815_dn8, locals.var_q_temp2__blk815_dn9, ) = (assign38220_e43191, (0.0055555555556 * (-(((locals.var_q_qsq__blk825_dn4 * 0.0714285714286) * assign38220_e43188) + (assign38220_e43177 * (-(((0.05 * locals.var_q_qsq__blk825_dn4) * assign38220_e43186) + (assign38220_e43181 * (-(0.0420875420875421 * locals.var_q_qsq__blk825_dn4))))))))), (0.0055555555556 * (-(((locals.var_q_qsq__blk825_dn6 * 0.0714285714286) * assign38220_e43188) + (assign38220_e43177 * (-(((0.05 * locals.var_q_qsq__blk825_dn6) * assign38220_e43186) + (assign38220_e43181 * (-(0.0420875420875421 * locals.var_q_qsq__blk825_dn6))))))))), (0.0055555555556 * (-(((locals.var_q_qsq__blk825_dn7 * 0.0714285714286) * assign38220_e43188) + (assign38220_e43177 * (-(((0.05 * locals.var_q_qsq__blk825_dn7) * assign38220_e43186) + (assign38220_e43181 * (-(0.0420875420875421 * locals.var_q_qsq__blk825_dn7))))))))), (0.0055555555556 * (-(((locals.var_q_qsq__blk825_dn8 * 0.0714285714286) * assign38220_e43188) + (assign38220_e43177 * (-(((0.05 * locals.var_q_qsq__blk825_dn8) * assign38220_e43186) + (assign38220_e43181 * (-(0.0420875420875421 * locals.var_q_qsq__blk825_dn8))))))))), (0.0055555555556 * (-(((locals.var_q_qsq__blk825_dn9 * 0.0714285714286) * assign38220_e43188) + (assign38220_e43177 * (-(((0.05 * locals.var_q_qsq__blk825_dn9) * assign38220_e43186) + (assign38220_e43181 * (-(0.0420875420875421 * locals.var_q_qsq__blk825_dn9))))))))), );
            locals.var_q_temp2__blk815_rv = 0.0;
        }
        if (((locals.var_guard1080 != 0.0) && (locals.var_guard1190 == 0.0)) && (locals.var_guard1191 == 0.0)) {
            let assign38230_e43203: f64 = (locals.var_q_d2_qsq__blk827 * locals.var_q_temp1__blk814);
            let assign38230_e43206: f64 = (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826);
            let assign38230_e43208: f64 = (assign38230_e43206 * locals.var_q_temp2__blk815);
            let assign38230_e43209: f64 = (assign38230_e43203 - assign38230_e43208);
            (locals.var_q_d2_qcoth__blk832, locals.var_q_d2_qcoth__blk832_dn4, locals.var_q_d2_qcoth__blk832_dn6, locals.var_q_d2_qcoth__blk832_dn7, locals.var_q_d2_qcoth__blk832_dn8, locals.var_q_d2_qcoth__blk832_dn9, ) = (assign38230_e43209, (((locals.var_q_d2_qsq__blk827_dn4 * locals.var_q_temp1__blk814) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp1__blk814_dn4)) - ((((locals.var_q_d1_qsq__blk826_dn4 * locals.var_q_d1_qsq__blk826) + (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826_dn4)) * locals.var_q_temp2__blk815) + (assign38230_e43206 * locals.var_q_temp2__blk815_dn4))), (((locals.var_q_d2_qsq__blk827_dn6 * locals.var_q_temp1__blk814) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp1__blk814_dn6)) - ((((locals.var_q_d1_qsq__blk826_dn6 * locals.var_q_d1_qsq__blk826) + (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826_dn6)) * locals.var_q_temp2__blk815) + (assign38230_e43206 * locals.var_q_temp2__blk815_dn6))), (((locals.var_q_d2_qsq__blk827_dn7 * locals.var_q_temp1__blk814) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp1__blk814_dn7)) - ((((locals.var_q_d1_qsq__blk826_dn7 * locals.var_q_d1_qsq__blk826) + (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826_dn7)) * locals.var_q_temp2__blk815) + (assign38230_e43206 * locals.var_q_temp2__blk815_dn7))), (((locals.var_q_d2_qsq__blk827_dn8 * locals.var_q_temp1__blk814) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp1__blk814_dn8)) - ((((locals.var_q_d1_qsq__blk826_dn8 * locals.var_q_d1_qsq__blk826) + (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826_dn8)) * locals.var_q_temp2__blk815) + (assign38230_e43206 * locals.var_q_temp2__blk815_dn8))), (((locals.var_q_d2_qsq__blk827_dn9 * locals.var_q_temp1__blk814) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp1__blk814_dn9)) - ((((locals.var_q_d1_qsq__blk826_dn9 * locals.var_q_d1_qsq__blk826) + (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826_dn9)) * locals.var_q_temp2__blk815) + (assign38230_e43206 * locals.var_q_temp2__blk815_dn9))), );
            locals.var_q_d2_qcoth__blk832_rv = 0.0;
        }
        if (((locals.var_guard1080 != 0.0) && (locals.var_guard1190 == 0.0)) && (locals.var_guard1191 == 0.0)) {
            let assign38240_e43220: f64 = (-0.5);
            let assign38240_e43222: f64 = (assign38240_e43220 * locals.var_q_d1_qsq__blk826);
            let assign38240_e43224: f64 = (assign38240_e43222 * locals.var_q_temp3__blk816);
            (locals.var_q_d1_ln__blk835, locals.var_q_d1_ln__blk835_dn4, locals.var_q_d1_ln__blk835_dn6, locals.var_q_d1_ln__blk835_dn7, locals.var_q_d1_ln__blk835_dn8, locals.var_q_d1_ln__blk835_dn9, ) = (assign38240_e43224, (((assign38240_e43220 * locals.var_q_d1_qsq__blk826_dn4) * locals.var_q_temp3__blk816) + (assign38240_e43222 * locals.var_q_temp3__blk816_dn4)), (((assign38240_e43220 * locals.var_q_d1_qsq__blk826_dn6) * locals.var_q_temp3__blk816) + (assign38240_e43222 * locals.var_q_temp3__blk816_dn6)), (((assign38240_e43220 * locals.var_q_d1_qsq__blk826_dn7) * locals.var_q_temp3__blk816) + (assign38240_e43222 * locals.var_q_temp3__blk816_dn7)), (((assign38240_e43220 * locals.var_q_d1_qsq__blk826_dn8) * locals.var_q_temp3__blk816) + (assign38240_e43222 * locals.var_q_temp3__blk816_dn8)), (((assign38240_e43220 * locals.var_q_d1_qsq__blk826_dn9) * locals.var_q_temp3__blk816) + (assign38240_e43222 * locals.var_q_temp3__blk816_dn9)), );
            locals.var_q_d1_ln__blk835_rv = 0.0;
        }
        if (((locals.var_guard1080 != 0.0) && (locals.var_guard1190 == 0.0)) && (locals.var_guard1191 == 0.0)) {
            let assign38250_e43235: f64 = (-0.5);
            let assign38250_e43237: f64 = (assign38250_e43235 * locals.var_q_d2_qsq__blk827);
            let assign38250_e43239: f64 = (assign38250_e43237 * locals.var_q_temp3__blk816);
            let assign38250_e43242: f64 = (0.25 * 0.0055555555556);
            let assign38250_e43244: f64 = (assign38250_e43242 * locals.var_q_d1_qsq__blk826);
            let assign38250_e43246: f64 = (assign38250_e43244 * locals.var_q_d1_qsq__blk826);
            let assign38250_e43250: f64 = (locals.var_q_qsq__blk825 * 0.0238095238095);
            let assign38250_e43254: f64 = (0.075 * locals.var_q_qsq__blk825);
            let assign38250_e43255: f64 = (2.0 - assign38250_e43254);
            let assign38250_e43256: f64 = (assign38250_e43250 * assign38250_e43255);
            let assign38250_e43257: f64 = (1.0 - assign38250_e43256);
            let assign38250_e43258: f64 = (assign38250_e43246 * assign38250_e43257);
            let assign38250_e43259: f64 = (assign38250_e43239 + assign38250_e43258);
            (locals.var_q_d2_ln__blk836, locals.var_q_d2_ln__blk836_dn4, locals.var_q_d2_ln__blk836_dn6, locals.var_q_d2_ln__blk836_dn7, locals.var_q_d2_ln__blk836_dn8, locals.var_q_d2_ln__blk836_dn9, ) = (assign38250_e43259, ((((assign38250_e43235 * locals.var_q_d2_qsq__blk827_dn4) * locals.var_q_temp3__blk816) + (assign38250_e43237 * locals.var_q_temp3__blk816_dn4)) + (((((assign38250_e43242 * locals.var_q_d1_qsq__blk826_dn4) * locals.var_q_d1_qsq__blk826) + (assign38250_e43244 * locals.var_q_d1_qsq__blk826_dn4)) * assign38250_e43257) + (assign38250_e43246 * (-(((locals.var_q_qsq__blk825_dn4 * 0.0238095238095) * assign38250_e43255) + (assign38250_e43250 * (-(0.075 * locals.var_q_qsq__blk825_dn4)))))))), ((((assign38250_e43235 * locals.var_q_d2_qsq__blk827_dn6) * locals.var_q_temp3__blk816) + (assign38250_e43237 * locals.var_q_temp3__blk816_dn6)) + (((((assign38250_e43242 * locals.var_q_d1_qsq__blk826_dn6) * locals.var_q_d1_qsq__blk826) + (assign38250_e43244 * locals.var_q_d1_qsq__blk826_dn6)) * assign38250_e43257) + (assign38250_e43246 * (-(((locals.var_q_qsq__blk825_dn6 * 0.0238095238095) * assign38250_e43255) + (assign38250_e43250 * (-(0.075 * locals.var_q_qsq__blk825_dn6)))))))), ((((assign38250_e43235 * locals.var_q_d2_qsq__blk827_dn7) * locals.var_q_temp3__blk816) + (assign38250_e43237 * locals.var_q_temp3__blk816_dn7)) + (((((assign38250_e43242 * locals.var_q_d1_qsq__blk826_dn7) * locals.var_q_d1_qsq__blk826) + (assign38250_e43244 * locals.var_q_d1_qsq__blk826_dn7)) * assign38250_e43257) + (assign38250_e43246 * (-(((locals.var_q_qsq__blk825_dn7 * 0.0238095238095) * assign38250_e43255) + (assign38250_e43250 * (-(0.075 * locals.var_q_qsq__blk825_dn7)))))))), ((((assign38250_e43235 * locals.var_q_d2_qsq__blk827_dn8) * locals.var_q_temp3__blk816) + (assign38250_e43237 * locals.var_q_temp3__blk816_dn8)) + (((((assign38250_e43242 * locals.var_q_d1_qsq__blk826_dn8) * locals.var_q_d1_qsq__blk826) + (assign38250_e43244 * locals.var_q_d1_qsq__blk826_dn8)) * assign38250_e43257) + (assign38250_e43246 * (-(((locals.var_q_qsq__blk825_dn8 * 0.0238095238095) * assign38250_e43255) + (assign38250_e43250 * (-(0.075 * locals.var_q_qsq__blk825_dn8)))))))), ((((assign38250_e43235 * locals.var_q_d2_qsq__blk827_dn9) * locals.var_q_temp3__blk816) + (assign38250_e43237 * locals.var_q_temp3__blk816_dn9)) + (((((assign38250_e43242 * locals.var_q_d1_qsq__blk826_dn9) * locals.var_q_d1_qsq__blk826) + (assign38250_e43244 * locals.var_q_d1_qsq__blk826_dn9)) * assign38250_e43257) + (assign38250_e43246 * (-(((locals.var_q_qsq__blk825_dn9 * 0.0238095238095) * assign38250_e43255) + (assign38250_e43250 * (-(0.075 * locals.var_q_qsq__blk825_dn9)))))))), );
            locals.var_q_d2_ln__blk836_rv = 0.0;
        }
        let assign38260_e43264: f64 = if locals.var_q_qsq__blk825 > 0.005 { 1.0 } else { 0.0 };
        locals.var_guard1192 = assign38260_e43264;
        locals.var_guard1192_rv = 0.0;
        if ((locals.var_guard1080 != 0.0) && (locals.var_guard1192 != 0.0)) {
            let assign38270_e43270: f64 = (4.0 * locals.var_q_qsq__blk825);
            let assign38270_e43275: f64 = (2.0 - locals.var_q_invexpq__blk831);
            let assign38270_e43276: f64 = (locals.var_q_invexpq__blk831 * assign38270_e43275);
            let assign38270_e43277: f64 = (1.0 - assign38270_e43276);
            let assign38270_e43278: f64 = (assign38270_e43270 / assign38270_e43277);
            (locals.var_q_temp2__blk815, locals.var_q_temp2__blk815_dn4, locals.var_q_temp2__blk815_dn6, locals.var_q_temp2__blk815_dn7, locals.var_q_temp2__blk815_dn8, locals.var_q_temp2__blk815_dn9, ) = (assign38270_e43278, ((((4.0 * locals.var_q_qsq__blk825_dn4) * assign38270_e43277) - (assign38270_e43270 * (-((locals.var_q_invexpq__blk831_dn4 * assign38270_e43275) + (locals.var_q_invexpq__blk831 * (-locals.var_q_invexpq__blk831_dn4)))))) / (assign38270_e43277 * assign38270_e43277)), ((((4.0 * locals.var_q_qsq__blk825_dn6) * assign38270_e43277) - (assign38270_e43270 * (-((locals.var_q_invexpq__blk831_dn6 * assign38270_e43275) + (locals.var_q_invexpq__blk831 * (-locals.var_q_invexpq__blk831_dn6)))))) / (assign38270_e43277 * assign38270_e43277)), ((((4.0 * locals.var_q_qsq__blk825_dn7) * assign38270_e43277) - (assign38270_e43270 * (-((locals.var_q_invexpq__blk831_dn7 * assign38270_e43275) + (locals.var_q_invexpq__blk831 * (-locals.var_q_invexpq__blk831_dn7)))))) / (assign38270_e43277 * assign38270_e43277)), ((((4.0 * locals.var_q_qsq__blk825_dn8) * assign38270_e43277) - (assign38270_e43270 * (-((locals.var_q_invexpq__blk831_dn8 * assign38270_e43275) + (locals.var_q_invexpq__blk831 * (-locals.var_q_invexpq__blk831_dn8)))))) / (assign38270_e43277 * assign38270_e43277)), ((((4.0 * locals.var_q_qsq__blk825_dn9) * assign38270_e43277) - (assign38270_e43270 * (-((locals.var_q_invexpq__blk831_dn9 * assign38270_e43275) + (locals.var_q_invexpq__blk831 * (-locals.var_q_invexpq__blk831_dn9)))))) / (assign38270_e43277 * assign38270_e43277)), );
            locals.var_q_temp2__blk815_rv = 0.0;
        }
    }
    pub(super) fn stamp_reactive_block_55(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        if ((locals.var_guard1080 != 0.0) && (locals.var_guard1192 != 0.0)) {
            let assign38280_e43286: f64 = (locals.var_q_temp2__blk815 * locals.var_q_invexpq__blk831);
            (locals.var_q_sh_term__blk833, locals.var_q_sh_term__blk833_dn4, locals.var_q_sh_term__blk833_dn6, locals.var_q_sh_term__blk833_dn7, locals.var_q_sh_term__blk833_dn8, locals.var_q_sh_term__blk833_dn9, ) = (assign38280_e43286, ((locals.var_q_temp2__blk815_dn4 * locals.var_q_invexpq__blk831) + (locals.var_q_temp2__blk815 * locals.var_q_invexpq__blk831_dn4)), ((locals.var_q_temp2__blk815_dn6 * locals.var_q_invexpq__blk831) + (locals.var_q_temp2__blk815 * locals.var_q_invexpq__blk831_dn6)), ((locals.var_q_temp2__blk815_dn7 * locals.var_q_invexpq__blk831) + (locals.var_q_temp2__blk815 * locals.var_q_invexpq__blk831_dn7)), ((locals.var_q_temp2__blk815_dn8 * locals.var_q_invexpq__blk831) + (locals.var_q_temp2__blk815 * locals.var_q_invexpq__blk831_dn8)), ((locals.var_q_temp2__blk815_dn9 * locals.var_q_invexpq__blk831) + (locals.var_q_temp2__blk815 * locals.var_q_invexpq__blk831_dn9)), );
            locals.var_q_sh_term__blk833_rv = 0.0;
        }
        if ((locals.var_guard1080 != 0.0) && (locals.var_guard1192 != 0.0)) {
            let assign38290_e43293: f64 = (locals.var_q_temp2__blk815).ln();
            let assign38290_e43295: f64 = (assign38290_e43293 - locals.var_q_rac_qsq__blk828);
            (locals.var_q_ln_term__blk834, locals.var_q_ln_term__blk834_dn4, locals.var_q_ln_term__blk834_dn6, locals.var_q_ln_term__blk834_dn7, locals.var_q_ln_term__blk834_dn8, locals.var_q_ln_term__blk834_dn9, ) = (assign38290_e43295, ((locals.var_q_temp2__blk815_dn4 / locals.var_q_temp2__blk815) - locals.var_q_rac_qsq__blk828_dn4), ((locals.var_q_temp2__blk815_dn6 / locals.var_q_temp2__blk815) - locals.var_q_rac_qsq__blk828_dn6), ((locals.var_q_temp2__blk815_dn7 / locals.var_q_temp2__blk815) - locals.var_q_rac_qsq__blk828_dn7), ((locals.var_q_temp2__blk815_dn8 / locals.var_q_temp2__blk815) - locals.var_q_rac_qsq__blk828_dn8), ((locals.var_q_temp2__blk815_dn9 / locals.var_q_temp2__blk815) - locals.var_q_rac_qsq__blk828_dn9), );
            locals.var_q_ln_term__blk834_rv = 0.0;
        }
        let assign38300_e43300: f64 = (-0.005);
        let assign38300_e43301: f64 = if locals.var_q_qsq__blk825 < assign38300_e43300 { 1.0 } else { 0.0 };
        locals.var_guard1193 = assign38300_e43301;
        locals.var_guard1193_rv = 0.0;
        if (((locals.var_guard1080 != 0.0) && (locals.var_guard1192 == 0.0)) && (locals.var_guard1193 != 0.0)) {
            let assign38310_e43310: f64 = (0.5 * locals.var_q_rac_qsq__blk828);
            let assign38310_e43311: f64 = (assign38310_e43310).sin();
            (locals.var_q_temp2__blk815, locals.var_q_temp2__blk815_dn4, locals.var_q_temp2__blk815_dn6, locals.var_q_temp2__blk815_dn7, locals.var_q_temp2__blk815_dn8, locals.var_q_temp2__blk815_dn9, ) = (assign38310_e43311, ((assign38310_e43310).cos() * (0.5 * locals.var_q_rac_qsq__blk828_dn4)), ((assign38310_e43310).cos() * (0.5 * locals.var_q_rac_qsq__blk828_dn6)), ((assign38310_e43310).cos() * (0.5 * locals.var_q_rac_qsq__blk828_dn7)), ((assign38310_e43310).cos() * (0.5 * locals.var_q_rac_qsq__blk828_dn8)), ((assign38310_e43310).cos() * (0.5 * locals.var_q_rac_qsq__blk828_dn9)), );
            locals.var_q_temp2__blk815_rv = 0.0;
        }
        if (((locals.var_guard1080 != 0.0) && (locals.var_guard1192 == 0.0)) && (locals.var_guard1193 != 0.0)) {
            let assign38320_e43321: f64 = (-locals.var_q_qsq__blk825);
            let assign38320_e43324: f64 = (locals.var_q_temp2__blk815 * locals.var_q_temp2__blk815);
            let assign38320_e43325: f64 = (assign38320_e43321 / assign38320_e43324);
            (locals.var_q_sh_term__blk833, locals.var_q_sh_term__blk833_dn4, locals.var_q_sh_term__blk833_dn6, locals.var_q_sh_term__blk833_dn7, locals.var_q_sh_term__blk833_dn8, locals.var_q_sh_term__blk833_dn9, ) = (assign38320_e43325, ((((-locals.var_q_qsq__blk825_dn4) * assign38320_e43324) - (assign38320_e43321 * ((locals.var_q_temp2__blk815_dn4 * locals.var_q_temp2__blk815) + (locals.var_q_temp2__blk815 * locals.var_q_temp2__blk815_dn4)))) / (assign38320_e43324 * assign38320_e43324)), ((((-locals.var_q_qsq__blk825_dn6) * assign38320_e43324) - (assign38320_e43321 * ((locals.var_q_temp2__blk815_dn6 * locals.var_q_temp2__blk815) + (locals.var_q_temp2__blk815 * locals.var_q_temp2__blk815_dn6)))) / (assign38320_e43324 * assign38320_e43324)), ((((-locals.var_q_qsq__blk825_dn7) * assign38320_e43324) - (assign38320_e43321 * ((locals.var_q_temp2__blk815_dn7 * locals.var_q_temp2__blk815) + (locals.var_q_temp2__blk815 * locals.var_q_temp2__blk815_dn7)))) / (assign38320_e43324 * assign38320_e43324)), ((((-locals.var_q_qsq__blk825_dn8) * assign38320_e43324) - (assign38320_e43321 * ((locals.var_q_temp2__blk815_dn8 * locals.var_q_temp2__blk815) + (locals.var_q_temp2__blk815 * locals.var_q_temp2__blk815_dn8)))) / (assign38320_e43324 * assign38320_e43324)), ((((-locals.var_q_qsq__blk825_dn9) * assign38320_e43324) - (assign38320_e43321 * ((locals.var_q_temp2__blk815_dn9 * locals.var_q_temp2__blk815) + (locals.var_q_temp2__blk815 * locals.var_q_temp2__blk815_dn9)))) / (assign38320_e43324 * assign38320_e43324)), );
            locals.var_q_sh_term__blk833_rv = 0.0;
        }
        if (((locals.var_guard1080 != 0.0) && (locals.var_guard1192 == 0.0)) && (locals.var_guard1193 != 0.0)) {
            let assign38330_e43335: f64 = (locals.var_q_sh_term__blk833).ln();
            (locals.var_q_ln_term__blk834, locals.var_q_ln_term__blk834_dn4, locals.var_q_ln_term__blk834_dn6, locals.var_q_ln_term__blk834_dn7, locals.var_q_ln_term__blk834_dn8, locals.var_q_ln_term__blk834_dn9, ) = (assign38330_e43335, (locals.var_q_sh_term__blk833_dn4 / locals.var_q_sh_term__blk833), (locals.var_q_sh_term__blk833_dn6 / locals.var_q_sh_term__blk833), (locals.var_q_sh_term__blk833_dn7 / locals.var_q_sh_term__blk833), (locals.var_q_sh_term__blk833_dn8 / locals.var_q_sh_term__blk833), (locals.var_q_sh_term__blk833_dn9 / locals.var_q_sh_term__blk833), );
            locals.var_q_ln_term__blk834_rv = 0.0;
        }
        if (((locals.var_guard1080 != 0.0) && (locals.var_guard1192 == 0.0)) && (locals.var_guard1193 == 0.0)) {
            let assign38340_e43348: f64 = (locals.var_q_qsq__blk825 * 0.3333333333333);
            let assign38340_e43352: f64 = (0.05 * locals.var_q_qsq__blk825);
            let assign38340_e43356: f64 = (0.0396825396825397 * locals.var_q_qsq__blk825);
            let assign38340_e43357: f64 = (1.0 - assign38340_e43356);
            let assign38340_e43358: f64 = (assign38340_e43352 * assign38340_e43357);
            let assign38340_e43359: f64 = (1.0 - assign38340_e43358);
            let assign38340_e43360: f64 = (assign38340_e43348 * assign38340_e43359);
            let assign38340_e43361: f64 = (4.0 - assign38340_e43360);
            (locals.var_q_sh_term__blk833, locals.var_q_sh_term__blk833_dn4, locals.var_q_sh_term__blk833_dn6, locals.var_q_sh_term__blk833_dn7, locals.var_q_sh_term__blk833_dn8, locals.var_q_sh_term__blk833_dn9, ) = (assign38340_e43361, (-(((locals.var_q_qsq__blk825_dn4 * 0.3333333333333) * assign38340_e43359) + (assign38340_e43348 * (-(((0.05 * locals.var_q_qsq__blk825_dn4) * assign38340_e43357) + (assign38340_e43352 * (-(0.0396825396825397 * locals.var_q_qsq__blk825_dn4)))))))), (-(((locals.var_q_qsq__blk825_dn6 * 0.3333333333333) * assign38340_e43359) + (assign38340_e43348 * (-(((0.05 * locals.var_q_qsq__blk825_dn6) * assign38340_e43357) + (assign38340_e43352 * (-(0.0396825396825397 * locals.var_q_qsq__blk825_dn6)))))))), (-(((locals.var_q_qsq__blk825_dn7 * 0.3333333333333) * assign38340_e43359) + (assign38340_e43348 * (-(((0.05 * locals.var_q_qsq__blk825_dn7) * assign38340_e43357) + (assign38340_e43352 * (-(0.0396825396825397 * locals.var_q_qsq__blk825_dn7)))))))), (-(((locals.var_q_qsq__blk825_dn8 * 0.3333333333333) * assign38340_e43359) + (assign38340_e43348 * (-(((0.05 * locals.var_q_qsq__blk825_dn8) * assign38340_e43357) + (assign38340_e43352 * (-(0.0396825396825397 * locals.var_q_qsq__blk825_dn8)))))))), (-(((locals.var_q_qsq__blk825_dn9 * 0.3333333333333) * assign38340_e43359) + (assign38340_e43348 * (-(((0.05 * locals.var_q_qsq__blk825_dn9) * assign38340_e43357) + (assign38340_e43352 * (-(0.0396825396825397 * locals.var_q_qsq__blk825_dn9)))))))), );
            locals.var_q_sh_term__blk833_rv = 0.0;
        }
        if (((locals.var_guard1080 != 0.0) && (locals.var_guard1192 == 0.0)) && (locals.var_guard1193 == 0.0)) {
            let assign38350_e43372: f64 = (locals.var_q_sh_term__blk833).ln();
            (locals.var_q_ln_term__blk834, locals.var_q_ln_term__blk834_dn4, locals.var_q_ln_term__blk834_dn6, locals.var_q_ln_term__blk834_dn7, locals.var_q_ln_term__blk834_dn8, locals.var_q_ln_term__blk834_dn9, ) = (assign38350_e43372, (locals.var_q_sh_term__blk833_dn4 / locals.var_q_sh_term__blk833), (locals.var_q_sh_term__blk833_dn6 / locals.var_q_sh_term__blk833), (locals.var_q_sh_term__blk833_dn7 / locals.var_q_sh_term__blk833), (locals.var_q_sh_term__blk833_dn8 / locals.var_q_sh_term__blk833), (locals.var_q_sh_term__blk833_dn9 / locals.var_q_sh_term__blk833), );
            locals.var_q_ln_term__blk834_rv = 0.0;
        }
        let assign38360_e43377: f64 = (1.01 * locals.var_q_k1q1__blk823);
        let assign38360_e43379: f64 = (assign38360_e43377 + locals.var_q_qcoth__blk829);
        let assign38360_e43381: f64 = if assign38360_e43379 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1194 = assign38360_e43381;
        locals.var_guard1194_rv = 0.0;
        if ((locals.var_guard1080 != 0.0) && (locals.var_guard1194 != 0.0)) {
            let assign38370_e43387: f64 = (locals.var_q_k1q1__blk823 + locals.var_q_qcoth__blk829);
            (locals.var_q_expnum__blk837, locals.var_q_expnum__blk837_dn4, locals.var_q_expnum__blk837_dn6, locals.var_q_expnum__blk837_dn7, locals.var_q_expnum__blk837_dn8, locals.var_q_expnum__blk837_dn9, ) = (assign38370_e43387, (locals.var_q_k1q1__blk823_dn4 + locals.var_q_qcoth__blk829_dn4), (locals.var_q_k1q1__blk823_dn6 + locals.var_q_qcoth__blk829_dn6), (locals.var_q_k1q1__blk823_dn7 + locals.var_q_qcoth__blk829_dn7), (locals.var_q_k1q1__blk823_dn8 + locals.var_q_qcoth__blk829_dn8), (locals.var_q_k1q1__blk823_dn9 + locals.var_q_qcoth__blk829_dn9), );
            locals.var_q_expnum__blk837_rv = 0.0;
        }
        if ((locals.var_guard1080 != 0.0) && (locals.var_guard1194 != 0.0)) {
            let assign38380_e43395: f64 = (locals.var_k1__blk932 + locals.var_q_d1_qcoth__blk830);
            (locals.var_q_d1_expnum__blk838, locals.var_q_d1_expnum__blk838_dn4, locals.var_q_d1_expnum__blk838_dn6, locals.var_q_d1_expnum__blk838_dn7, locals.var_q_d1_expnum__blk838_dn8, locals.var_q_d1_expnum__blk838_dn9, ) = (assign38380_e43395, (locals.var_k1__blk932_dn4 + locals.var_q_d1_qcoth__blk830_dn4), (locals.var_k1__blk932_dn6 + locals.var_q_d1_qcoth__blk830_dn6), (locals.var_k1__blk932_dn7 + locals.var_q_d1_qcoth__blk830_dn7), (locals.var_k1__blk932_dn8 + locals.var_q_d1_qcoth__blk830_dn8), (locals.var_k1__blk932_dn9 + locals.var_q_d1_qcoth__blk830_dn9), );
            locals.var_q_d1_expnum__blk838_rv = 0.0;
        }
        if ((locals.var_guard1080 != 0.0) && (locals.var_guard1194 != 0.0)) {
            (locals.var_q_d2_expnum__blk839, locals.var_q_d2_expnum__blk839_dn4, locals.var_q_d2_expnum__blk839_dn6, locals.var_q_d2_expnum__blk839_dn7, locals.var_q_d2_expnum__blk839_dn8, locals.var_q_d2_expnum__blk839_dn9, ) = (locals.var_q_d2_qcoth__blk832, locals.var_q_d2_qcoth__blk832_dn4, locals.var_q_d2_qcoth__blk832_dn6, locals.var_q_d2_qcoth__blk832_dn7, locals.var_q_d2_qcoth__blk832_dn8, locals.var_q_d2_qcoth__blk832_dn9, );
            locals.var_q_d2_expnum__blk839_rv = 0.0;
        }
        if ((locals.var_guard1080 != 0.0) && (locals.var_guard1194 == 0.0)) {
            let assign38400_e43411: f64 = (locals.var_q_k1q1__blk823 - locals.var_q_qcoth__blk829);
            let assign38400_e43412: f64 = (1.0 / assign38400_e43411);
            (locals.var_q_temp2__blk815, locals.var_q_temp2__blk815_dn4, locals.var_q_temp2__blk815_dn6, locals.var_q_temp2__blk815_dn7, locals.var_q_temp2__blk815_dn8, locals.var_q_temp2__blk815_dn9, ) = (assign38400_e43412, (-((locals.var_q_k1q1__blk823_dn4 - locals.var_q_qcoth__blk829_dn4) / (assign38400_e43411 * assign38400_e43411))), (-((locals.var_q_k1q1__blk823_dn6 - locals.var_q_qcoth__blk829_dn6) / (assign38400_e43411 * assign38400_e43411))), (-((locals.var_q_k1q1__blk823_dn7 - locals.var_q_qcoth__blk829_dn7) / (assign38400_e43411 * assign38400_e43411))), (-((locals.var_q_k1q1__blk823_dn8 - locals.var_q_qcoth__blk829_dn8) / (assign38400_e43411 * assign38400_e43411))), (-((locals.var_q_k1q1__blk823_dn9 - locals.var_q_qcoth__blk829_dn9) / (assign38400_e43411 * assign38400_e43411))), );
            locals.var_q_temp2__blk815_rv = 0.0;
        }
        if ((locals.var_guard1080 != 0.0) && (locals.var_guard1194 == 0.0)) {
            let assign38410_e43421: f64 = (locals.var_q_d1_qcoth__blk830 - locals.var_k1__blk932);
            (locals.var_q_temp3__blk816, locals.var_q_temp3__blk816_dn4, locals.var_q_temp3__blk816_dn6, locals.var_q_temp3__blk816_dn7, locals.var_q_temp3__blk816_dn8, locals.var_q_temp3__blk816_dn9, ) = (assign38410_e43421, (locals.var_q_d1_qcoth__blk830_dn4 - locals.var_k1__blk932_dn4), (locals.var_q_d1_qcoth__blk830_dn6 - locals.var_k1__blk932_dn6), (locals.var_q_d1_qcoth__blk830_dn7 - locals.var_k1__blk932_dn7), (locals.var_q_d1_qcoth__blk830_dn8 - locals.var_k1__blk932_dn8), (locals.var_q_d1_qcoth__blk830_dn9 - locals.var_k1__blk932_dn9), );
            locals.var_q_temp3__blk816_rv = 0.0;
        }
        if ((locals.var_guard1080 != 0.0) && (locals.var_guard1194 == 0.0)) {
            let assign38420_e43430: f64 = (locals.var_q_aexp__blk824 - locals.var_q_sh_term__blk833);
            let assign38420_e43432: f64 = (assign38420_e43430 * locals.var_q_temp2__blk815);
            (locals.var_q_expnum__blk837, locals.var_q_expnum__blk837_dn4, locals.var_q_expnum__blk837_dn6, locals.var_q_expnum__blk837_dn7, locals.var_q_expnum__blk837_dn8, locals.var_q_expnum__blk837_dn9, ) = (assign38420_e43432, (((locals.var_q_aexp__blk824_dn4 - locals.var_q_sh_term__blk833_dn4) * locals.var_q_temp2__blk815) + (assign38420_e43430 * locals.var_q_temp2__blk815_dn4)), (((locals.var_q_aexp__blk824_dn6 - locals.var_q_sh_term__blk833_dn6) * locals.var_q_temp2__blk815) + (assign38420_e43430 * locals.var_q_temp2__blk815_dn6)), (((locals.var_q_aexp__blk824_dn7 - locals.var_q_sh_term__blk833_dn7) * locals.var_q_temp2__blk815) + (assign38420_e43430 * locals.var_q_temp2__blk815_dn7)), (((locals.var_q_aexp__blk824_dn8 - locals.var_q_sh_term__blk833_dn8) * locals.var_q_temp2__blk815) + (assign38420_e43430 * locals.var_q_temp2__blk815_dn8)), (((locals.var_q_aexp__blk824_dn9 - locals.var_q_sh_term__blk833_dn9) * locals.var_q_temp2__blk815) + (assign38420_e43430 * locals.var_q_temp2__blk815_dn9)), );
            locals.var_q_expnum__blk837_rv = 0.0;
        }
        if ((locals.var_guard1080 != 0.0) && (locals.var_guard1194 == 0.0)) {
            let assign38430_e43441: f64 = (locals.var_q_temp3__blk816 * locals.var_q_expnum__blk837);
            let assign38430_e43443: f64 = (assign38430_e43441 - locals.var_q_aexp__blk824);
            let assign38430_e43446: f64 = (locals.var_q_d1_ln__blk835 * locals.var_q_sh_term__blk833);
            let assign38430_e43447: f64 = (assign38430_e43443 - assign38430_e43446);
            let assign38430_e43449: f64 = (assign38430_e43447 * locals.var_q_temp2__blk815);
            (locals.var_q_d1_expnum__blk838, locals.var_q_d1_expnum__blk838_dn4, locals.var_q_d1_expnum__blk838_dn6, locals.var_q_d1_expnum__blk838_dn7, locals.var_q_d1_expnum__blk838_dn8, locals.var_q_d1_expnum__blk838_dn9, ) = (assign38430_e43449, ((((((locals.var_q_temp3__blk816_dn4 * locals.var_q_expnum__blk837) + (locals.var_q_temp3__blk816 * locals.var_q_expnum__blk837_dn4)) - locals.var_q_aexp__blk824_dn4) - ((locals.var_q_d1_ln__blk835_dn4 * locals.var_q_sh_term__blk833) + (locals.var_q_d1_ln__blk835 * locals.var_q_sh_term__blk833_dn4))) * locals.var_q_temp2__blk815) + (assign38430_e43447 * locals.var_q_temp2__blk815_dn4)), ((((((locals.var_q_temp3__blk816_dn6 * locals.var_q_expnum__blk837) + (locals.var_q_temp3__blk816 * locals.var_q_expnum__blk837_dn6)) - locals.var_q_aexp__blk824_dn6) - ((locals.var_q_d1_ln__blk835_dn6 * locals.var_q_sh_term__blk833) + (locals.var_q_d1_ln__blk835 * locals.var_q_sh_term__blk833_dn6))) * locals.var_q_temp2__blk815) + (assign38430_e43447 * locals.var_q_temp2__blk815_dn6)), ((((((locals.var_q_temp3__blk816_dn7 * locals.var_q_expnum__blk837) + (locals.var_q_temp3__blk816 * locals.var_q_expnum__blk837_dn7)) - locals.var_q_aexp__blk824_dn7) - ((locals.var_q_d1_ln__blk835_dn7 * locals.var_q_sh_term__blk833) + (locals.var_q_d1_ln__blk835 * locals.var_q_sh_term__blk833_dn7))) * locals.var_q_temp2__blk815) + (assign38430_e43447 * locals.var_q_temp2__blk815_dn7)), ((((((locals.var_q_temp3__blk816_dn8 * locals.var_q_expnum__blk837) + (locals.var_q_temp3__blk816 * locals.var_q_expnum__blk837_dn8)) - locals.var_q_aexp__blk824_dn8) - ((locals.var_q_d1_ln__blk835_dn8 * locals.var_q_sh_term__blk833) + (locals.var_q_d1_ln__blk835 * locals.var_q_sh_term__blk833_dn8))) * locals.var_q_temp2__blk815) + (assign38430_e43447 * locals.var_q_temp2__blk815_dn8)), ((((((locals.var_q_temp3__blk816_dn9 * locals.var_q_expnum__blk837) + (locals.var_q_temp3__blk816 * locals.var_q_expnum__blk837_dn9)) - locals.var_q_aexp__blk824_dn9) - ((locals.var_q_d1_ln__blk835_dn9 * locals.var_q_sh_term__blk833) + (locals.var_q_d1_ln__blk835 * locals.var_q_sh_term__blk833_dn9))) * locals.var_q_temp2__blk815) + (assign38430_e43447 * locals.var_q_temp2__blk815_dn9)), );
            locals.var_q_d1_expnum__blk838_rv = 0.0;
        }
        if ((locals.var_guard1080 != 0.0) && (locals.var_guard1194 == 0.0)) {
            let assign38440_e43458: f64 = (locals.var_q_d2_qcoth__blk832 * locals.var_q_expnum__blk837);
            let assign38440_e43461: f64 = (2.0 * locals.var_q_temp3__blk816);
            let assign38440_e43463: f64 = (assign38440_e43461 * locals.var_q_d1_expnum__blk838);
            let assign38440_e43464: f64 = (assign38440_e43458 + assign38440_e43463);
            let assign38440_e43466: f64 = (assign38440_e43464 + locals.var_q_aexp__blk824);
            let assign38440_e43470: f64 = (locals.var_q_d1_ln__blk835 * locals.var_q_d1_ln__blk835);
            let assign38440_e43471: f64 = (locals.var_q_d2_ln__blk836 + assign38440_e43470);
            let assign38440_e43473: f64 = (assign38440_e43471 * locals.var_q_sh_term__blk833);
            let assign38440_e43474: f64 = (assign38440_e43466 - assign38440_e43473);
            let assign38440_e43476: f64 = (assign38440_e43474 * locals.var_q_temp2__blk815);
            (locals.var_q_d2_expnum__blk839, locals.var_q_d2_expnum__blk839_dn4, locals.var_q_d2_expnum__blk839_dn6, locals.var_q_d2_expnum__blk839_dn7, locals.var_q_d2_expnum__blk839_dn8, locals.var_q_d2_expnum__blk839_dn9, ) = (assign38440_e43476, (((((((locals.var_q_d2_qcoth__blk832_dn4 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qcoth__blk832 * locals.var_q_expnum__blk837_dn4)) + (((2.0 * locals.var_q_temp3__blk816_dn4) * locals.var_q_d1_expnum__blk838) + (assign38440_e43461 * locals.var_q_d1_expnum__blk838_dn4))) + locals.var_q_aexp__blk824_dn4) - (((locals.var_q_d2_ln__blk836_dn4 + ((locals.var_q_d1_ln__blk835_dn4 * locals.var_q_d1_ln__blk835) + (locals.var_q_d1_ln__blk835 * locals.var_q_d1_ln__blk835_dn4))) * locals.var_q_sh_term__blk833) + (assign38440_e43471 * locals.var_q_sh_term__blk833_dn4))) * locals.var_q_temp2__blk815) + (assign38440_e43474 * locals.var_q_temp2__blk815_dn4)), (((((((locals.var_q_d2_qcoth__blk832_dn6 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qcoth__blk832 * locals.var_q_expnum__blk837_dn6)) + (((2.0 * locals.var_q_temp3__blk816_dn6) * locals.var_q_d1_expnum__blk838) + (assign38440_e43461 * locals.var_q_d1_expnum__blk838_dn6))) + locals.var_q_aexp__blk824_dn6) - (((locals.var_q_d2_ln__blk836_dn6 + ((locals.var_q_d1_ln__blk835_dn6 * locals.var_q_d1_ln__blk835) + (locals.var_q_d1_ln__blk835 * locals.var_q_d1_ln__blk835_dn6))) * locals.var_q_sh_term__blk833) + (assign38440_e43471 * locals.var_q_sh_term__blk833_dn6))) * locals.var_q_temp2__blk815) + (assign38440_e43474 * locals.var_q_temp2__blk815_dn6)), (((((((locals.var_q_d2_qcoth__blk832_dn7 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qcoth__blk832 * locals.var_q_expnum__blk837_dn7)) + (((2.0 * locals.var_q_temp3__blk816_dn7) * locals.var_q_d1_expnum__blk838) + (assign38440_e43461 * locals.var_q_d1_expnum__blk838_dn7))) + locals.var_q_aexp__blk824_dn7) - (((locals.var_q_d2_ln__blk836_dn7 + ((locals.var_q_d1_ln__blk835_dn7 * locals.var_q_d1_ln__blk835) + (locals.var_q_d1_ln__blk835 * locals.var_q_d1_ln__blk835_dn7))) * locals.var_q_sh_term__blk833) + (assign38440_e43471 * locals.var_q_sh_term__blk833_dn7))) * locals.var_q_temp2__blk815) + (assign38440_e43474 * locals.var_q_temp2__blk815_dn7)), (((((((locals.var_q_d2_qcoth__blk832_dn8 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qcoth__blk832 * locals.var_q_expnum__blk837_dn8)) + (((2.0 * locals.var_q_temp3__blk816_dn8) * locals.var_q_d1_expnum__blk838) + (assign38440_e43461 * locals.var_q_d1_expnum__blk838_dn8))) + locals.var_q_aexp__blk824_dn8) - (((locals.var_q_d2_ln__blk836_dn8 + ((locals.var_q_d1_ln__blk835_dn8 * locals.var_q_d1_ln__blk835) + (locals.var_q_d1_ln__blk835 * locals.var_q_d1_ln__blk835_dn8))) * locals.var_q_sh_term__blk833) + (assign38440_e43471 * locals.var_q_sh_term__blk833_dn8))) * locals.var_q_temp2__blk815) + (assign38440_e43474 * locals.var_q_temp2__blk815_dn8)), (((((((locals.var_q_d2_qcoth__blk832_dn9 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qcoth__blk832 * locals.var_q_expnum__blk837_dn9)) + (((2.0 * locals.var_q_temp3__blk816_dn9) * locals.var_q_d1_expnum__blk838) + (assign38440_e43461 * locals.var_q_d1_expnum__blk838_dn9))) + locals.var_q_aexp__blk824_dn9) - (((locals.var_q_d2_ln__blk836_dn9 + ((locals.var_q_d1_ln__blk835_dn9 * locals.var_q_d1_ln__blk835) + (locals.var_q_d1_ln__blk835 * locals.var_q_d1_ln__blk835_dn9))) * locals.var_q_sh_term__blk833) + (assign38440_e43471 * locals.var_q_sh_term__blk833_dn9))) * locals.var_q_temp2__blk815) + (assign38440_e43474 * locals.var_q_temp2__blk815_dn9)), );
            locals.var_q_d2_expnum__blk839_rv = 0.0;
        }
        let assign38450_e43481: f64 = if locals.var_q_expnum__blk837 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1195 = assign38450_e43481;
        locals.var_guard1195_rv = 0.0;
        if ((locals.var_guard1080 != 0.0) && (locals.var_guard1195 != 0.0)) {
            let assign38460_e43486: f64 = (locals.var_q_expnum__blk837).ln();
            (locals.var_q_lnexpnum__blk840, locals.var_q_lnexpnum__blk840_dn4, locals.var_q_lnexpnum__blk840_dn6, locals.var_q_lnexpnum__blk840_dn7, locals.var_q_lnexpnum__blk840_dn8, locals.var_q_lnexpnum__blk840_dn9, ) = (assign38460_e43486, (locals.var_q_expnum__blk837_dn4 / locals.var_q_expnum__blk837), (locals.var_q_expnum__blk837_dn6 / locals.var_q_expnum__blk837), (locals.var_q_expnum__blk837_dn7 / locals.var_q_expnum__blk837), (locals.var_q_expnum__blk837_dn8 / locals.var_q_expnum__blk837), (locals.var_q_expnum__blk837_dn9 / locals.var_q_expnum__blk837), );
            locals.var_q_lnexpnum__blk840_rv = 0.0;
        }
        if ((locals.var_guard1080 != 0.0) && (locals.var_guard1195 != 0.0)) {
            let assign38470_e43494: f64 = (1.0 / locals.var_q_expnum__blk837);
            (locals.var_q_temp1__blk814, locals.var_q_temp1__blk814_dn4, locals.var_q_temp1__blk814_dn6, locals.var_q_temp1__blk814_dn7, locals.var_q_temp1__blk814_dn8, locals.var_q_temp1__blk814_dn9, ) = (assign38470_e43494, (-(locals.var_q_expnum__blk837_dn4 / (locals.var_q_expnum__blk837 * locals.var_q_expnum__blk837))), (-(locals.var_q_expnum__blk837_dn6 / (locals.var_q_expnum__blk837 * locals.var_q_expnum__blk837))), (-(locals.var_q_expnum__blk837_dn7 / (locals.var_q_expnum__blk837 * locals.var_q_expnum__blk837))), (-(locals.var_q_expnum__blk837_dn8 / (locals.var_q_expnum__blk837 * locals.var_q_expnum__blk837))), (-(locals.var_q_expnum__blk837_dn9 / (locals.var_q_expnum__blk837 * locals.var_q_expnum__blk837))), );
            locals.var_q_temp1__blk814_rv = 0.0;
        }
        if ((locals.var_guard1080 != 0.0) && (locals.var_guard1195 != 0.0)) {
            let assign38480_e43502: f64 = (locals.var_q_d1_expnum__blk838 * locals.var_q_temp1__blk814);
            (locals.var_q_d1_lnexpnum__blk841, locals.var_q_d1_lnexpnum__blk841_dn4, locals.var_q_d1_lnexpnum__blk841_dn6, locals.var_q_d1_lnexpnum__blk841_dn7, locals.var_q_d1_lnexpnum__blk841_dn8, locals.var_q_d1_lnexpnum__blk841_dn9, ) = (assign38480_e43502, ((locals.var_q_d1_expnum__blk838_dn4 * locals.var_q_temp1__blk814) + (locals.var_q_d1_expnum__blk838 * locals.var_q_temp1__blk814_dn4)), ((locals.var_q_d1_expnum__blk838_dn6 * locals.var_q_temp1__blk814) + (locals.var_q_d1_expnum__blk838 * locals.var_q_temp1__blk814_dn6)), ((locals.var_q_d1_expnum__blk838_dn7 * locals.var_q_temp1__blk814) + (locals.var_q_d1_expnum__blk838 * locals.var_q_temp1__blk814_dn7)), ((locals.var_q_d1_expnum__blk838_dn8 * locals.var_q_temp1__blk814) + (locals.var_q_d1_expnum__blk838 * locals.var_q_temp1__blk814_dn8)), ((locals.var_q_d1_expnum__blk838_dn9 * locals.var_q_temp1__blk814) + (locals.var_q_d1_expnum__blk838 * locals.var_q_temp1__blk814_dn9)), );
            locals.var_q_d1_lnexpnum__blk841_rv = 0.0;
        }
        if ((locals.var_guard1080 != 0.0) && (locals.var_guard1195 != 0.0)) {
            let assign38490_e43510: f64 = (locals.var_q_d2_expnum__blk839 * locals.var_q_temp1__blk814);
            let assign38490_e43513: f64 = (locals.var_q_d1_lnexpnum__blk841 * locals.var_q_d1_lnexpnum__blk841);
            let assign38490_e43514: f64 = (assign38490_e43510 - assign38490_e43513);
            (locals.var_q_d2_lnexpnum__blk842, locals.var_q_d2_lnexpnum__blk842_dn4, locals.var_q_d2_lnexpnum__blk842_dn6, locals.var_q_d2_lnexpnum__blk842_dn7, locals.var_q_d2_lnexpnum__blk842_dn8, locals.var_q_d2_lnexpnum__blk842_dn9, ) = (assign38490_e43514, (((locals.var_q_d2_expnum__blk839_dn4 * locals.var_q_temp1__blk814) + (locals.var_q_d2_expnum__blk839 * locals.var_q_temp1__blk814_dn4)) - ((locals.var_q_d1_lnexpnum__blk841_dn4 * locals.var_q_d1_lnexpnum__blk841) + (locals.var_q_d1_lnexpnum__blk841 * locals.var_q_d1_lnexpnum__blk841_dn4))), (((locals.var_q_d2_expnum__blk839_dn6 * locals.var_q_temp1__blk814) + (locals.var_q_d2_expnum__blk839 * locals.var_q_temp1__blk814_dn6)) - ((locals.var_q_d1_lnexpnum__blk841_dn6 * locals.var_q_d1_lnexpnum__blk841) + (locals.var_q_d1_lnexpnum__blk841 * locals.var_q_d1_lnexpnum__blk841_dn6))), (((locals.var_q_d2_expnum__blk839_dn7 * locals.var_q_temp1__blk814) + (locals.var_q_d2_expnum__blk839 * locals.var_q_temp1__blk814_dn7)) - ((locals.var_q_d1_lnexpnum__blk841_dn7 * locals.var_q_d1_lnexpnum__blk841) + (locals.var_q_d1_lnexpnum__blk841 * locals.var_q_d1_lnexpnum__blk841_dn7))), (((locals.var_q_d2_expnum__blk839_dn8 * locals.var_q_temp1__blk814) + (locals.var_q_d2_expnum__blk839 * locals.var_q_temp1__blk814_dn8)) - ((locals.var_q_d1_lnexpnum__blk841_dn8 * locals.var_q_d1_lnexpnum__blk841) + (locals.var_q_d1_lnexpnum__blk841 * locals.var_q_d1_lnexpnum__blk841_dn8))), (((locals.var_q_d2_expnum__blk839_dn9 * locals.var_q_temp1__blk814) + (locals.var_q_d2_expnum__blk839 * locals.var_q_temp1__blk814_dn9)) - ((locals.var_q_d1_lnexpnum__blk841_dn9 * locals.var_q_d1_lnexpnum__blk841) + (locals.var_q_d1_lnexpnum__blk841 * locals.var_q_d1_lnexpnum__blk841_dn9))), );
            locals.var_q_d2_lnexpnum__blk842_rv = 0.0;
        }
        if ((locals.var_guard1080 != 0.0) && (locals.var_guard1195 == 0.0)) {
            let assign38500_e43523: f64 = (locals.var_q_k1q1__blk823 + 0.6931471805599);
            let assign38500_e43525: f64 = (-locals.var_q_k1q1__blk823);
            let assign38500_e43526: f64 = (assign38500_e43525).ln();
            let assign38500_e43527: f64 = (assign38500_e43523 + assign38500_e43526);
            (locals.var_q_lnexpnum__blk840, locals.var_q_lnexpnum__blk840_dn4, locals.var_q_lnexpnum__blk840_dn6, locals.var_q_lnexpnum__blk840_dn7, locals.var_q_lnexpnum__blk840_dn8, locals.var_q_lnexpnum__blk840_dn9, ) = (assign38500_e43527, (locals.var_q_k1q1__blk823_dn4 + ((-locals.var_q_k1q1__blk823_dn4) / assign38500_e43525)), (locals.var_q_k1q1__blk823_dn6 + ((-locals.var_q_k1q1__blk823_dn6) / assign38500_e43525)), (locals.var_q_k1q1__blk823_dn7 + ((-locals.var_q_k1q1__blk823_dn7) / assign38500_e43525)), (locals.var_q_k1q1__blk823_dn8 + ((-locals.var_q_k1q1__blk823_dn8) / assign38500_e43525)), (locals.var_q_k1q1__blk823_dn9 + ((-locals.var_q_k1q1__blk823_dn9) / assign38500_e43525)), );
            locals.var_q_lnexpnum__blk840_rv = 0.0;
        }
        if ((locals.var_guard1080 != 0.0) && (locals.var_guard1195 == 0.0)) {
            let assign38510_e43536: f64 = (1.0 / locals.var_q1d__blk1001);
            (locals.var_q_temp1__blk814, locals.var_q_temp1__blk814_dn4, locals.var_q_temp1__blk814_dn6, locals.var_q_temp1__blk814_dn7, locals.var_q_temp1__blk814_dn8, locals.var_q_temp1__blk814_dn9, ) = (assign38510_e43536, (-(locals.var_q1d__blk1001_dn4 / (locals.var_q1d__blk1001 * locals.var_q1d__blk1001))), (-(locals.var_q1d__blk1001_dn6 / (locals.var_q1d__blk1001 * locals.var_q1d__blk1001))), (-(locals.var_q1d__blk1001_dn7 / (locals.var_q1d__blk1001 * locals.var_q1d__blk1001))), (-(locals.var_q1d__blk1001_dn8 / (locals.var_q1d__blk1001 * locals.var_q1d__blk1001))), (-(locals.var_q1d__blk1001_dn9 / (locals.var_q1d__blk1001 * locals.var_q1d__blk1001))), );
            locals.var_q_temp1__blk814_rv = 0.0;
        }
        if ((locals.var_guard1080 != 0.0) && (locals.var_guard1195 == 0.0)) {
            let assign38520_e43545: f64 = (locals.var_k1__blk932 + locals.var_q_temp1__blk814);
            (locals.var_q_d1_lnexpnum__blk841, locals.var_q_d1_lnexpnum__blk841_dn4, locals.var_q_d1_lnexpnum__blk841_dn6, locals.var_q_d1_lnexpnum__blk841_dn7, locals.var_q_d1_lnexpnum__blk841_dn8, locals.var_q_d1_lnexpnum__blk841_dn9, ) = (assign38520_e43545, (locals.var_k1__blk932_dn4 + locals.var_q_temp1__blk814_dn4), (locals.var_k1__blk932_dn6 + locals.var_q_temp1__blk814_dn6), (locals.var_k1__blk932_dn7 + locals.var_q_temp1__blk814_dn7), (locals.var_k1__blk932_dn8 + locals.var_q_temp1__blk814_dn8), (locals.var_k1__blk932_dn9 + locals.var_q_temp1__blk814_dn9), );
            locals.var_q_d1_lnexpnum__blk841_rv = 0.0;
        }
        if ((locals.var_guard1080 != 0.0) && (locals.var_guard1195 == 0.0)) {
            let assign38530_e43553: f64 = (-locals.var_q_temp1__blk814);
            let assign38530_e43555: f64 = (assign38530_e43553 * locals.var_q_temp1__blk814);
            (locals.var_q_d2_lnexpnum__blk842, locals.var_q_d2_lnexpnum__blk842_dn4, locals.var_q_d2_lnexpnum__blk842_dn6, locals.var_q_d2_lnexpnum__blk842_dn7, locals.var_q_d2_lnexpnum__blk842_dn8, locals.var_q_d2_lnexpnum__blk842_dn9, ) = (assign38530_e43555, (((-locals.var_q_temp1__blk814_dn4) * locals.var_q_temp1__blk814) + (assign38530_e43553 * locals.var_q_temp1__blk814_dn4)), (((-locals.var_q_temp1__blk814_dn6) * locals.var_q_temp1__blk814) + (assign38530_e43553 * locals.var_q_temp1__blk814_dn6)), (((-locals.var_q_temp1__blk814_dn7) * locals.var_q_temp1__blk814) + (assign38530_e43553 * locals.var_q_temp1__blk814_dn7)), (((-locals.var_q_temp1__blk814_dn8) * locals.var_q_temp1__blk814) + (assign38530_e43553 * locals.var_q_temp1__blk814_dn8)), (((-locals.var_q_temp1__blk814_dn9) * locals.var_q_temp1__blk814) + (assign38530_e43553 * locals.var_q_temp1__blk814_dn9)), );
            locals.var_q_d2_lnexpnum__blk842_rv = 0.0;
        }
        if (locals.var_guard1080 != 0.0) {
            let assign38540_e43561: f64 = (locals.var_xg2x__blk931 - locals.var_xg1x__blk930);
            let assign38540_e43563: f64 = (assign38540_e43561 + locals.var_q1d__blk1001);
            let assign38540_e43566: f64 = (2.0 * locals.var_q_lnexpnum__blk840);
            let assign38540_e43567: f64 = (assign38540_e43563 + assign38540_e43566);
            let assign38540_e43569: f64 = (assign38540_e43567 - locals.var_q_ln_term__blk834);
            (locals.var_q_q2_int__blk843, locals.var_q_q2_int__blk843_dn4, locals.var_q_q2_int__blk843_dn6, locals.var_q_q2_int__blk843_dn7, locals.var_q_q2_int__blk843_dn8, locals.var_q_q2_int__blk843_dn9, ) = (assign38540_e43569, ((((locals.var_xg2x__blk931_dn4 - locals.var_xg1x__blk930_dn4) + locals.var_q1d__blk1001_dn4) + (2.0 * locals.var_q_lnexpnum__blk840_dn4)) - locals.var_q_ln_term__blk834_dn4), ((((locals.var_xg2x__blk931_dn6 - locals.var_xg1x__blk930_dn6) + locals.var_q1d__blk1001_dn6) + (2.0 * locals.var_q_lnexpnum__blk840_dn6)) - locals.var_q_ln_term__blk834_dn6), ((((locals.var_xg2x__blk931_dn7 - locals.var_xg1x__blk930_dn7) + locals.var_q1d__blk1001_dn7) + (2.0 * locals.var_q_lnexpnum__blk840_dn7)) - locals.var_q_ln_term__blk834_dn7), ((((locals.var_xg2x__blk931_dn8 - locals.var_xg1x__blk930_dn8) + locals.var_q1d__blk1001_dn8) + (2.0 * locals.var_q_lnexpnum__blk840_dn8)) - locals.var_q_ln_term__blk834_dn8), ((((locals.var_xg2x__blk931_dn9 - locals.var_xg1x__blk930_dn9) + locals.var_q1d__blk1001_dn9) + (2.0 * locals.var_q_lnexpnum__blk840_dn9)) - locals.var_q_ln_term__blk834_dn9), );
            locals.var_q_q2_int__blk843_rv = 0.0;
        }
        if (locals.var_guard1080 != 0.0) {
            let assign38550_e43576: f64 = (2.0 * locals.var_q_d1_lnexpnum__blk841);
            let assign38550_e43577: f64 = (1.0 + assign38550_e43576);
            let assign38550_e43579: f64 = (assign38550_e43577 - locals.var_q_d1_ln__blk835);
            (locals.var_q_d1_q2__blk844, locals.var_q_d1_q2__blk844_dn4, locals.var_q_d1_q2__blk844_dn6, locals.var_q_d1_q2__blk844_dn7, locals.var_q_d1_q2__blk844_dn8, locals.var_q_d1_q2__blk844_dn9, ) = (assign38550_e43579, ((2.0 * locals.var_q_d1_lnexpnum__blk841_dn4) - locals.var_q_d1_ln__blk835_dn4), ((2.0 * locals.var_q_d1_lnexpnum__blk841_dn6) - locals.var_q_d1_ln__blk835_dn6), ((2.0 * locals.var_q_d1_lnexpnum__blk841_dn7) - locals.var_q_d1_ln__blk835_dn7), ((2.0 * locals.var_q_d1_lnexpnum__blk841_dn8) - locals.var_q_d1_ln__blk835_dn8), ((2.0 * locals.var_q_d1_lnexpnum__blk841_dn9) - locals.var_q_d1_ln__blk835_dn9), );
            locals.var_q_d1_q2__blk844_rv = 0.0;
        }
        if (locals.var_guard1080 != 0.0) {
            let assign38560_e43585: f64 = (2.0 * locals.var_q_d2_lnexpnum__blk842);
            let assign38560_e43587: f64 = (assign38560_e43585 - locals.var_q_d2_ln__blk836);
            (locals.var_q_d2_q2__blk845, locals.var_q_d2_q2__blk845_dn4, locals.var_q_d2_q2__blk845_dn6, locals.var_q_d2_q2__blk845_dn7, locals.var_q_d2_q2__blk845_dn8, locals.var_q_d2_q2__blk845_dn9, ) = (assign38560_e43587, ((2.0 * locals.var_q_d2_lnexpnum__blk842_dn4) - locals.var_q_d2_ln__blk836_dn4), ((2.0 * locals.var_q_d2_lnexpnum__blk842_dn6) - locals.var_q_d2_ln__blk836_dn6), ((2.0 * locals.var_q_d2_lnexpnum__blk842_dn7) - locals.var_q_d2_ln__blk836_dn7), ((2.0 * locals.var_q_d2_lnexpnum__blk842_dn8) - locals.var_q_d2_ln__blk836_dn8), ((2.0 * locals.var_q_d2_lnexpnum__blk842_dn9) - locals.var_q_d2_ln__blk836_dn9), );
            locals.var_q_d2_q2__blk845_rv = 0.0;
        }
        if (locals.var_guard1080 != 0.0) {
            let assign38570_e43594: f64 = (locals.var_k2__blk933 * locals.var_q_q2_int__blk843);
            let assign38570_e43595: f64 = (locals.var_q_k1q1__blk823 + assign38570_e43594);
            (locals.var_q_qi_int__blk846, locals.var_q_qi_int__blk846_dn4, locals.var_q_qi_int__blk846_dn6, locals.var_q_qi_int__blk846_dn7, locals.var_q_qi_int__blk846_dn8, locals.var_q_qi_int__blk846_dn9, ) = (assign38570_e43595, (locals.var_q_k1q1__blk823_dn4 + ((locals.var_k2__blk933_dn4 * locals.var_q_q2_int__blk843) + (locals.var_k2__blk933 * locals.var_q_q2_int__blk843_dn4))), (locals.var_q_k1q1__blk823_dn6 + ((locals.var_k2__blk933_dn6 * locals.var_q_q2_int__blk843) + (locals.var_k2__blk933 * locals.var_q_q2_int__blk843_dn6))), (locals.var_q_k1q1__blk823_dn7 + ((locals.var_k2__blk933_dn7 * locals.var_q_q2_int__blk843) + (locals.var_k2__blk933 * locals.var_q_q2_int__blk843_dn7))), (locals.var_q_k1q1__blk823_dn8 + ((locals.var_k2__blk933_dn8 * locals.var_q_q2_int__blk843) + (locals.var_k2__blk933 * locals.var_q_q2_int__blk843_dn8))), (locals.var_q_k1q1__blk823_dn9 + ((locals.var_k2__blk933_dn9 * locals.var_q_q2_int__blk843) + (locals.var_k2__blk933 * locals.var_q_q2_int__blk843_dn9))), );
            locals.var_q_qi_int__blk846_rv = 0.0;
        }
        if (locals.var_guard1080 != 0.0) {
            let assign38580_e43602: f64 = (locals.var_k2__blk933 * locals.var_q_d1_q2__blk844);
            let assign38580_e43603: f64 = (locals.var_k1__blk932 + assign38580_e43602);
            (locals.var_q_d1_qi__blk847, locals.var_q_d1_qi__blk847_dn4, locals.var_q_d1_qi__blk847_dn6, locals.var_q_d1_qi__blk847_dn7, locals.var_q_d1_qi__blk847_dn8, locals.var_q_d1_qi__blk847_dn9, ) = (assign38580_e43603, (locals.var_k1__blk932_dn4 + ((locals.var_k2__blk933_dn4 * locals.var_q_d1_q2__blk844) + (locals.var_k2__blk933 * locals.var_q_d1_q2__blk844_dn4))), (locals.var_k1__blk932_dn6 + ((locals.var_k2__blk933_dn6 * locals.var_q_d1_q2__blk844) + (locals.var_k2__blk933 * locals.var_q_d1_q2__blk844_dn6))), (locals.var_k1__blk932_dn7 + ((locals.var_k2__blk933_dn7 * locals.var_q_d1_q2__blk844) + (locals.var_k2__blk933 * locals.var_q_d1_q2__blk844_dn7))), (locals.var_k1__blk932_dn8 + ((locals.var_k2__blk933_dn8 * locals.var_q_d1_q2__blk844) + (locals.var_k2__blk933 * locals.var_q_d1_q2__blk844_dn8))), (locals.var_k1__blk932_dn9 + ((locals.var_k2__blk933_dn9 * locals.var_q_d1_q2__blk844) + (locals.var_k2__blk933 * locals.var_q_d1_q2__blk844_dn9))), );
            locals.var_q_d1_qi__blk847_rv = 0.0;
        }
        if (locals.var_guard1080 != 0.0) {
            let assign38590_e43609: f64 = (locals.var_k2__blk933 * locals.var_q_d2_q2__blk845);
            (locals.var_q_d2_qi__blk848, locals.var_q_d2_qi__blk848_dn4, locals.var_q_d2_qi__blk848_dn6, locals.var_q_d2_qi__blk848_dn7, locals.var_q_d2_qi__blk848_dn8, locals.var_q_d2_qi__blk848_dn9, ) = (assign38590_e43609, ((locals.var_k2__blk933_dn4 * locals.var_q_d2_q2__blk845) + (locals.var_k2__blk933 * locals.var_q_d2_q2__blk845_dn4)), ((locals.var_k2__blk933_dn6 * locals.var_q_d2_q2__blk845) + (locals.var_k2__blk933 * locals.var_q_d2_q2__blk845_dn6)), ((locals.var_k2__blk933_dn7 * locals.var_q_d2_q2__blk845) + (locals.var_k2__blk933 * locals.var_q_d2_q2__blk845_dn7)), ((locals.var_k2__blk933_dn8 * locals.var_q_d2_q2__blk845) + (locals.var_k2__blk933 * locals.var_q_d2_q2__blk845_dn8)), ((locals.var_k2__blk933_dn9 * locals.var_q_d2_q2__blk845) + (locals.var_k2__blk933 * locals.var_q_d2_q2__blk845_dn9)), );
            locals.var_q_d2_qi__blk848_rv = 0.0;
        }
        if (locals.var_guard1080 != 0.0) {
            let assign38600_e43615: f64 = (locals.var_q_qi_int__blk846 * locals.var_q_expnum__blk837);
            let assign38600_e43617: f64 = (assign38600_e43615 - locals.var_q_aexp__blk824);
            (locals.var_q_zero__blk849, locals.var_q_zero__blk849_dn4, locals.var_q_zero__blk849_dn6, locals.var_q_zero__blk849_dn7, locals.var_q_zero__blk849_dn8, locals.var_q_zero__blk849_dn9, ) = (assign38600_e43617, (((locals.var_q_qi_int__blk846_dn4 * locals.var_q_expnum__blk837) + (locals.var_q_qi_int__blk846 * locals.var_q_expnum__blk837_dn4)) - locals.var_q_aexp__blk824_dn4), (((locals.var_q_qi_int__blk846_dn6 * locals.var_q_expnum__blk837) + (locals.var_q_qi_int__blk846 * locals.var_q_expnum__blk837_dn6)) - locals.var_q_aexp__blk824_dn6), (((locals.var_q_qi_int__blk846_dn7 * locals.var_q_expnum__blk837) + (locals.var_q_qi_int__blk846 * locals.var_q_expnum__blk837_dn7)) - locals.var_q_aexp__blk824_dn7), (((locals.var_q_qi_int__blk846_dn8 * locals.var_q_expnum__blk837) + (locals.var_q_qi_int__blk846 * locals.var_q_expnum__blk837_dn8)) - locals.var_q_aexp__blk824_dn8), (((locals.var_q_qi_int__blk846_dn9 * locals.var_q_expnum__blk837) + (locals.var_q_qi_int__blk846 * locals.var_q_expnum__blk837_dn9)) - locals.var_q_aexp__blk824_dn9), );
            locals.var_q_zero__blk849_rv = 0.0;
        }
        if (locals.var_guard1080 != 0.0) {
            let assign38610_e43623: f64 = (locals.var_q_d1_qi__blk847 * locals.var_q_expnum__blk837);
            let assign38610_e43626: f64 = (locals.var_q_qi_int__blk846 * locals.var_q_d1_expnum__blk838);
            let assign38610_e43627: f64 = (assign38610_e43623 + assign38610_e43626);
            let assign38610_e43629: f64 = (assign38610_e43627 + locals.var_q_aexp__blk824);
            (locals.var_q_d1_zero__blk850, locals.var_q_d1_zero__blk850_dn4, locals.var_q_d1_zero__blk850_dn6, locals.var_q_d1_zero__blk850_dn7, locals.var_q_d1_zero__blk850_dn8, locals.var_q_d1_zero__blk850_dn9, ) = (assign38610_e43629, ((((locals.var_q_d1_qi__blk847_dn4 * locals.var_q_expnum__blk837) + (locals.var_q_d1_qi__blk847 * locals.var_q_expnum__blk837_dn4)) + ((locals.var_q_qi_int__blk846_dn4 * locals.var_q_d1_expnum__blk838) + (locals.var_q_qi_int__blk846 * locals.var_q_d1_expnum__blk838_dn4))) + locals.var_q_aexp__blk824_dn4), ((((locals.var_q_d1_qi__blk847_dn6 * locals.var_q_expnum__blk837) + (locals.var_q_d1_qi__blk847 * locals.var_q_expnum__blk837_dn6)) + ((locals.var_q_qi_int__blk846_dn6 * locals.var_q_d1_expnum__blk838) + (locals.var_q_qi_int__blk846 * locals.var_q_d1_expnum__blk838_dn6))) + locals.var_q_aexp__blk824_dn6), ((((locals.var_q_d1_qi__blk847_dn7 * locals.var_q_expnum__blk837) + (locals.var_q_d1_qi__blk847 * locals.var_q_expnum__blk837_dn7)) + ((locals.var_q_qi_int__blk846_dn7 * locals.var_q_d1_expnum__blk838) + (locals.var_q_qi_int__blk846 * locals.var_q_d1_expnum__blk838_dn7))) + locals.var_q_aexp__blk824_dn7), ((((locals.var_q_d1_qi__blk847_dn8 * locals.var_q_expnum__blk837) + (locals.var_q_d1_qi__blk847 * locals.var_q_expnum__blk837_dn8)) + ((locals.var_q_qi_int__blk846_dn8 * locals.var_q_d1_expnum__blk838) + (locals.var_q_qi_int__blk846 * locals.var_q_d1_expnum__blk838_dn8))) + locals.var_q_aexp__blk824_dn8), ((((locals.var_q_d1_qi__blk847_dn9 * locals.var_q_expnum__blk837) + (locals.var_q_d1_qi__blk847 * locals.var_q_expnum__blk837_dn9)) + ((locals.var_q_qi_int__blk846_dn9 * locals.var_q_d1_expnum__blk838) + (locals.var_q_qi_int__blk846 * locals.var_q_d1_expnum__blk838_dn9))) + locals.var_q_aexp__blk824_dn9), );
            locals.var_q_d1_zero__blk850_rv = 0.0;
        }
        if (locals.var_guard1080 != 0.0) {
            let assign38620_e43635: f64 = (locals.var_q_d2_qi__blk848 * locals.var_q_expnum__blk837);
            let assign38620_e43638: f64 = (2.0 * locals.var_q_d1_qi__blk847);
            let assign38620_e43640: f64 = (assign38620_e43638 * locals.var_q_d1_expnum__blk838);
            let assign38620_e43641: f64 = (assign38620_e43635 + assign38620_e43640);
            let assign38620_e43644: f64 = (locals.var_q_qi_int__blk846 * locals.var_q_d2_expnum__blk839);
            let assign38620_e43645: f64 = (assign38620_e43641 + assign38620_e43644);
            let assign38620_e43647: f64 = (assign38620_e43645 - locals.var_q_aexp__blk824);
            (locals.var_q_d2_zero__blk851, locals.var_q_d2_zero__blk851_dn4, locals.var_q_d2_zero__blk851_dn6, locals.var_q_d2_zero__blk851_dn7, locals.var_q_d2_zero__blk851_dn8, locals.var_q_d2_zero__blk851_dn9, ) = (assign38620_e43647, (((((locals.var_q_d2_qi__blk848_dn4 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qi__blk848 * locals.var_q_expnum__blk837_dn4)) + (((2.0 * locals.var_q_d1_qi__blk847_dn4) * locals.var_q_d1_expnum__blk838) + (assign38620_e43638 * locals.var_q_d1_expnum__blk838_dn4))) + ((locals.var_q_qi_int__blk846_dn4 * locals.var_q_d2_expnum__blk839) + (locals.var_q_qi_int__blk846 * locals.var_q_d2_expnum__blk839_dn4))) - locals.var_q_aexp__blk824_dn4), (((((locals.var_q_d2_qi__blk848_dn6 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qi__blk848 * locals.var_q_expnum__blk837_dn6)) + (((2.0 * locals.var_q_d1_qi__blk847_dn6) * locals.var_q_d1_expnum__blk838) + (assign38620_e43638 * locals.var_q_d1_expnum__blk838_dn6))) + ((locals.var_q_qi_int__blk846_dn6 * locals.var_q_d2_expnum__blk839) + (locals.var_q_qi_int__blk846 * locals.var_q_d2_expnum__blk839_dn6))) - locals.var_q_aexp__blk824_dn6), (((((locals.var_q_d2_qi__blk848_dn7 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qi__blk848 * locals.var_q_expnum__blk837_dn7)) + (((2.0 * locals.var_q_d1_qi__blk847_dn7) * locals.var_q_d1_expnum__blk838) + (assign38620_e43638 * locals.var_q_d1_expnum__blk838_dn7))) + ((locals.var_q_qi_int__blk846_dn7 * locals.var_q_d2_expnum__blk839) + (locals.var_q_qi_int__blk846 * locals.var_q_d2_expnum__blk839_dn7))) - locals.var_q_aexp__blk824_dn7), (((((locals.var_q_d2_qi__blk848_dn8 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qi__blk848 * locals.var_q_expnum__blk837_dn8)) + (((2.0 * locals.var_q_d1_qi__blk847_dn8) * locals.var_q_d1_expnum__blk838) + (assign38620_e43638 * locals.var_q_d1_expnum__blk838_dn8))) + ((locals.var_q_qi_int__blk846_dn8 * locals.var_q_d2_expnum__blk839) + (locals.var_q_qi_int__blk846 * locals.var_q_d2_expnum__blk839_dn8))) - locals.var_q_aexp__blk824_dn8), (((((locals.var_q_d2_qi__blk848_dn9 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qi__blk848 * locals.var_q_expnum__blk837_dn9)) + (((2.0 * locals.var_q_d1_qi__blk847_dn9) * locals.var_q_d1_expnum__blk838) + (assign38620_e43638 * locals.var_q_d1_expnum__blk838_dn9))) + ((locals.var_q_qi_int__blk846_dn9 * locals.var_q_d2_expnum__blk839) + (locals.var_q_qi_int__blk846 * locals.var_q_d2_expnum__blk839_dn9))) - locals.var_q_aexp__blk824_dn9), );
            locals.var_q_d2_zero__blk851_rv = 0.0;
        }
        if (locals.var_guard1080 != 0.0) {
            let assign38630_e43653: f64 = (locals.var_q_d1_zero__blk850 * locals.var_q_d1_zero__blk850);
            let assign38630_e43656: f64 = (0.5 * locals.var_q_zero__blk849);
            let assign38630_e43658: f64 = (assign38630_e43656 * locals.var_q_d2_zero__blk851);
            let assign38630_e43659: f64 = (assign38630_e43653 - assign38630_e43658);
            (locals.var_q_temp__blk860, locals.var_q_temp__blk860_dn4, locals.var_q_temp__blk860_dn6, locals.var_q_temp__blk860_dn7, locals.var_q_temp__blk860_dn8, locals.var_q_temp__blk860_dn9, ) = (assign38630_e43659, (((locals.var_q_d1_zero__blk850_dn4 * locals.var_q_d1_zero__blk850) + (locals.var_q_d1_zero__blk850 * locals.var_q_d1_zero__blk850_dn4)) - (((0.5 * locals.var_q_zero__blk849_dn4) * locals.var_q_d2_zero__blk851) + (assign38630_e43656 * locals.var_q_d2_zero__blk851_dn4))), (((locals.var_q_d1_zero__blk850_dn6 * locals.var_q_d1_zero__blk850) + (locals.var_q_d1_zero__blk850 * locals.var_q_d1_zero__blk850_dn6)) - (((0.5 * locals.var_q_zero__blk849_dn6) * locals.var_q_d2_zero__blk851) + (assign38630_e43656 * locals.var_q_d2_zero__blk851_dn6))), (((locals.var_q_d1_zero__blk850_dn7 * locals.var_q_d1_zero__blk850) + (locals.var_q_d1_zero__blk850 * locals.var_q_d1_zero__blk850_dn7)) - (((0.5 * locals.var_q_zero__blk849_dn7) * locals.var_q_d2_zero__blk851) + (assign38630_e43656 * locals.var_q_d2_zero__blk851_dn7))), (((locals.var_q_d1_zero__blk850_dn8 * locals.var_q_d1_zero__blk850) + (locals.var_q_d1_zero__blk850 * locals.var_q_d1_zero__blk850_dn8)) - (((0.5 * locals.var_q_zero__blk849_dn8) * locals.var_q_d2_zero__blk851) + (assign38630_e43656 * locals.var_q_d2_zero__blk851_dn8))), (((locals.var_q_d1_zero__blk850_dn9 * locals.var_q_d1_zero__blk850) + (locals.var_q_d1_zero__blk850 * locals.var_q_d1_zero__blk850_dn9)) - (((0.5 * locals.var_q_zero__blk849_dn9) * locals.var_q_d2_zero__blk851) + (assign38630_e43656 * locals.var_q_d2_zero__blk851_dn9))), );
            locals.var_q_temp__blk860_rv = 0.0;
        }
        if (locals.var_guard1080 != 0.0) {
            let assign38640_e43664: f64 = (-locals.var_q_zero__blk849);
            let assign38640_e43666: f64 = (assign38640_e43664 * locals.var_q_d1_zero__blk850);
            let assign38640_e43668: f64 = (assign38640_e43666 * locals.var_q_temp__blk860);
            let assign38640_e43671: f64 = (locals.var_q_temp__blk860 * locals.var_q_temp__blk860);
            let assign38640_e43673: f64 = (assign38640_e43671 + 1e-200);
            let assign38640_e43674: f64 = (assign38640_e43668 / assign38640_e43673);
            (locals.var_q_eps2__blk852, locals.var_q_eps2__blk852_dn4, locals.var_q_eps2__blk852_dn6, locals.var_q_eps2__blk852_dn7, locals.var_q_eps2__blk852_dn8, locals.var_q_eps2__blk852_dn9, ) = (assign38640_e43674, ((((((((-locals.var_q_zero__blk849_dn4) * locals.var_q_d1_zero__blk850) + (assign38640_e43664 * locals.var_q_d1_zero__blk850_dn4)) * locals.var_q_temp__blk860) + (assign38640_e43666 * locals.var_q_temp__blk860_dn4)) * assign38640_e43673) - (assign38640_e43668 * ((locals.var_q_temp__blk860_dn4 * locals.var_q_temp__blk860) + (locals.var_q_temp__blk860 * locals.var_q_temp__blk860_dn4)))) / (assign38640_e43673 * assign38640_e43673)), ((((((((-locals.var_q_zero__blk849_dn6) * locals.var_q_d1_zero__blk850) + (assign38640_e43664 * locals.var_q_d1_zero__blk850_dn6)) * locals.var_q_temp__blk860) + (assign38640_e43666 * locals.var_q_temp__blk860_dn6)) * assign38640_e43673) - (assign38640_e43668 * ((locals.var_q_temp__blk860_dn6 * locals.var_q_temp__blk860) + (locals.var_q_temp__blk860 * locals.var_q_temp__blk860_dn6)))) / (assign38640_e43673 * assign38640_e43673)), ((((((((-locals.var_q_zero__blk849_dn7) * locals.var_q_d1_zero__blk850) + (assign38640_e43664 * locals.var_q_d1_zero__blk850_dn7)) * locals.var_q_temp__blk860) + (assign38640_e43666 * locals.var_q_temp__blk860_dn7)) * assign38640_e43673) - (assign38640_e43668 * ((locals.var_q_temp__blk860_dn7 * locals.var_q_temp__blk860) + (locals.var_q_temp__blk860 * locals.var_q_temp__blk860_dn7)))) / (assign38640_e43673 * assign38640_e43673)), ((((((((-locals.var_q_zero__blk849_dn8) * locals.var_q_d1_zero__blk850) + (assign38640_e43664 * locals.var_q_d1_zero__blk850_dn8)) * locals.var_q_temp__blk860) + (assign38640_e43666 * locals.var_q_temp__blk860_dn8)) * assign38640_e43673) - (assign38640_e43668 * ((locals.var_q_temp__blk860_dn8 * locals.var_q_temp__blk860) + (locals.var_q_temp__blk860 * locals.var_q_temp__blk860_dn8)))) / (assign38640_e43673 * assign38640_e43673)), ((((((((-locals.var_q_zero__blk849_dn9) * locals.var_q_d1_zero__blk850) + (assign38640_e43664 * locals.var_q_d1_zero__blk850_dn9)) * locals.var_q_temp__blk860) + (assign38640_e43666 * locals.var_q_temp__blk860_dn9)) * assign38640_e43673) - (assign38640_e43668 * ((locals.var_q_temp__blk860_dn9 * locals.var_q_temp__blk860) + (locals.var_q_temp__blk860 * locals.var_q_temp__blk860_dn9)))) / (assign38640_e43673 * assign38640_e43673)), );
            locals.var_q_eps2__blk852_rv = 0.0;
        }
        if (locals.var_guard1080 != 0.0) {
            let assign38650_e43680: f64 = (locals.var_q1d__blk1001 + locals.var_q_eps2__blk852);
            (locals.var_q1d__blk1001, locals.var_q1d__blk1001_dn4, locals.var_q1d__blk1001_dn6, locals.var_q1d__blk1001_dn7, locals.var_q1d__blk1001_dn8, locals.var_q1d__blk1001_dn9, ) = (assign38650_e43680, (locals.var_q1d__blk1001_dn4 + locals.var_q_eps2__blk852_dn4), (locals.var_q1d__blk1001_dn6 + locals.var_q_eps2__blk852_dn6), (locals.var_q1d__blk1001_dn7 + locals.var_q_eps2__blk852_dn7), (locals.var_q1d__blk1001_dn8 + locals.var_q_eps2__blk852_dn8), (locals.var_q1d__blk1001_dn9 + locals.var_q_eps2__blk852_dn9), );
            locals.var_q1d__blk1001_rv = 0.0;
        }
        let assign38660_e43685: f64 = if p.p10 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1196 = assign38660_e43685;
        locals.var_guard1196_rv = 0.0;
        let assign38670_e43687: f64 = (locals.var_q_eps2__blk852).abs();
        let assign38670_e43689: f64 = if assign38670_e43687 > 0.01 { 1.0 } else { 0.0 };
        locals.var_guard1197 = assign38670_e43689;
        locals.var_guard1197_rv = 0.0;
        if (((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) {
            let assign38680_e43697: f64 = (locals.var_k1__blk932 * locals.var_q1d__blk1001);
            (locals.var_q_k1q1__blk823, locals.var_q_k1q1__blk823_dn4, locals.var_q_k1q1__blk823_dn6, locals.var_q_k1q1__blk823_dn7, locals.var_q_k1q1__blk823_dn8, locals.var_q_k1q1__blk823_dn9, ) = (assign38680_e43697, ((locals.var_k1__blk932_dn4 * locals.var_q1d__blk1001) + (locals.var_k1__blk932 * locals.var_q1d__blk1001_dn4)), ((locals.var_k1__blk932_dn6 * locals.var_q1d__blk1001) + (locals.var_k1__blk932 * locals.var_q1d__blk1001_dn6)), ((locals.var_k1__blk932_dn7 * locals.var_q1d__blk1001) + (locals.var_k1__blk932 * locals.var_q1d__blk1001_dn7)), ((locals.var_k1__blk932_dn8 * locals.var_q1d__blk1001) + (locals.var_k1__blk932 * locals.var_q1d__blk1001_dn8)), ((locals.var_k1__blk932_dn9 * locals.var_q1d__blk1001) + (locals.var_k1__blk932 * locals.var_q1d__blk1001_dn9)), );
            locals.var_q_k1q1__blk823_rv = 0.0;
        }
        let assign38690_e43702: f64 = (locals.var_xg1x__blk930 - locals.var_q1d__blk1001);
        let assign38690_e43704: f64 = (assign38690_e43702 - locals.var_xdeff__blk1000);
        let assign38690_e43706: f64 = if assign38690_e43704 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard1198 = assign38690_e43706;
        locals.var_guard1198_rv = 0.0;
        if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1198 != 0.0)) {
            let assign38700_e43716: f64 = (locals.var_xg1x__blk930 - locals.var_q1d__blk1001);
            let assign38700_e43718: f64 = (assign38700_e43716 - locals.var_xdeff__blk1000);
            let assign38700_e43719: f64 = (assign38700_e43718).exp();
            (locals.var_q_temp1__blk814, locals.var_q_temp1__blk814_dn4, locals.var_q_temp1__blk814_dn6, locals.var_q_temp1__blk814_dn7, locals.var_q_temp1__blk814_dn8, locals.var_q_temp1__blk814_dn9, ) = (assign38700_e43719, (assign38700_e43719 * ((locals.var_xg1x__blk930_dn4 - locals.var_q1d__blk1001_dn4) - locals.var_xdeff__blk1000_dn4)), (assign38700_e43719 * ((locals.var_xg1x__blk930_dn6 - locals.var_q1d__blk1001_dn6) - locals.var_xdeff__blk1000_dn6)), (assign38700_e43719 * ((locals.var_xg1x__blk930_dn7 - locals.var_q1d__blk1001_dn7) - locals.var_xdeff__blk1000_dn7)), (assign38700_e43719 * ((locals.var_xg1x__blk930_dn8 - locals.var_q1d__blk1001_dn8) - locals.var_xdeff__blk1000_dn8)), (assign38700_e43719 * ((locals.var_xg1x__blk930_dn9 - locals.var_q1d__blk1001_dn9) - locals.var_xdeff__blk1000_dn9)), );
            locals.var_q_temp1__blk814_rv = 0.0;
        }
        if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1198 == 0.0)) {
            let assign38710_e43734: f64 = (locals.var_xg1x__blk930 - locals.var_q1d__blk1001);
            let assign38710_e43736: f64 = (assign38710_e43734 - locals.var_xdeff__blk1000);
            let assign38710_e43738: f64 = (assign38710_e43736 - 80.0);
            let assign38710_e43743: f64 = (locals.var_xg1x__blk930 - locals.var_q1d__blk1001);
            let assign38710_e43745: f64 = (assign38710_e43743 - locals.var_xdeff__blk1000);
            let assign38710_e43747: f64 = (assign38710_e43745 - 80.0);
            let assign38710_e43748: f64 = (0.5 * assign38710_e43747);
            let assign38710_e43752: f64 = (locals.var_xg1x__blk930 - locals.var_q1d__blk1001);
            let assign38710_e43754: f64 = (assign38710_e43752 - locals.var_xdeff__blk1000);
            let assign38710_e43756: f64 = (assign38710_e43754 - 80.0);
            let assign38710_e43758: f64 = (assign38710_e43756 * 0.3333333333333);
            let assign38710_e43759: f64 = (1.0 + assign38710_e43758);
            let assign38710_e43760: f64 = (assign38710_e43748 * assign38710_e43759);
            let assign38710_e43761: f64 = (1.0 + assign38710_e43760);
            let assign38710_e43762: f64 = (assign38710_e43738 * assign38710_e43761);
            let assign38710_e43763: f64 = (1.0 + assign38710_e43762);
            let assign38710_e43764: f64 = (5.54062e34 * assign38710_e43763);
            (locals.var_q_temp1__blk814, locals.var_q_temp1__blk814_dn4, locals.var_q_temp1__blk814_dn6, locals.var_q_temp1__blk814_dn7, locals.var_q_temp1__blk814_dn8, locals.var_q_temp1__blk814_dn9, ) = (assign38710_e43764, (5.54062e34 * ((((locals.var_xg1x__blk930_dn4 - locals.var_q1d__blk1001_dn4) - locals.var_xdeff__blk1000_dn4) * assign38710_e43761) + (assign38710_e43738 * (((0.5 * ((locals.var_xg1x__blk930_dn4 - locals.var_q1d__blk1001_dn4) - locals.var_xdeff__blk1000_dn4)) * assign38710_e43759) + (assign38710_e43748 * (((locals.var_xg1x__blk930_dn4 - locals.var_q1d__blk1001_dn4) - locals.var_xdeff__blk1000_dn4) * 0.3333333333333)))))), (5.54062e34 * ((((locals.var_xg1x__blk930_dn6 - locals.var_q1d__blk1001_dn6) - locals.var_xdeff__blk1000_dn6) * assign38710_e43761) + (assign38710_e43738 * (((0.5 * ((locals.var_xg1x__blk930_dn6 - locals.var_q1d__blk1001_dn6) - locals.var_xdeff__blk1000_dn6)) * assign38710_e43759) + (assign38710_e43748 * (((locals.var_xg1x__blk930_dn6 - locals.var_q1d__blk1001_dn6) - locals.var_xdeff__blk1000_dn6) * 0.3333333333333)))))), (5.54062e34 * ((((locals.var_xg1x__blk930_dn7 - locals.var_q1d__blk1001_dn7) - locals.var_xdeff__blk1000_dn7) * assign38710_e43761) + (assign38710_e43738 * (((0.5 * ((locals.var_xg1x__blk930_dn7 - locals.var_q1d__blk1001_dn7) - locals.var_xdeff__blk1000_dn7)) * assign38710_e43759) + (assign38710_e43748 * (((locals.var_xg1x__blk930_dn7 - locals.var_q1d__blk1001_dn7) - locals.var_xdeff__blk1000_dn7) * 0.3333333333333)))))), (5.54062e34 * ((((locals.var_xg1x__blk930_dn8 - locals.var_q1d__blk1001_dn8) - locals.var_xdeff__blk1000_dn8) * assign38710_e43761) + (assign38710_e43738 * (((0.5 * ((locals.var_xg1x__blk930_dn8 - locals.var_q1d__blk1001_dn8) - locals.var_xdeff__blk1000_dn8)) * assign38710_e43759) + (assign38710_e43748 * (((locals.var_xg1x__blk930_dn8 - locals.var_q1d__blk1001_dn8) - locals.var_xdeff__blk1000_dn8) * 0.3333333333333)))))), (5.54062e34 * ((((locals.var_xg1x__blk930_dn9 - locals.var_q1d__blk1001_dn9) - locals.var_xdeff__blk1000_dn9) * assign38710_e43761) + (assign38710_e43738 * (((0.5 * ((locals.var_xg1x__blk930_dn9 - locals.var_q1d__blk1001_dn9) - locals.var_xdeff__blk1000_dn9)) * assign38710_e43759) + (assign38710_e43748 * (((locals.var_xg1x__blk930_dn9 - locals.var_q1d__blk1001_dn9) - locals.var_xdeff__blk1000_dn9) * 0.3333333333333)))))), );
            locals.var_q_temp1__blk814_rv = 0.0;
        }
        if (((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) {
            let assign38720_e43774: f64 = (locals.var_a0__blk905 * locals.var_q_temp1__blk814);
            (locals.var_q_aexp__blk824, locals.var_q_aexp__blk824_dn4, locals.var_q_aexp__blk824_dn6, locals.var_q_aexp__blk824_dn7, locals.var_q_aexp__blk824_dn8, locals.var_q_aexp__blk824_dn9, ) = (assign38720_e43774, ((locals.var_a0__blk905_dn4 * locals.var_q_temp1__blk814) + (locals.var_a0__blk905 * locals.var_q_temp1__blk814_dn4)), ((locals.var_a0__blk905_dn6 * locals.var_q_temp1__blk814) + (locals.var_a0__blk905 * locals.var_q_temp1__blk814_dn6)), ((locals.var_a0__blk905_dn7 * locals.var_q_temp1__blk814) + (locals.var_a0__blk905 * locals.var_q_temp1__blk814_dn7)), ((locals.var_a0__blk905_dn8 * locals.var_q_temp1__blk814) + (locals.var_a0__blk905 * locals.var_q_temp1__blk814_dn8)), ((locals.var_a0__blk905_dn9 * locals.var_q_temp1__blk814) + (locals.var_a0__blk905 * locals.var_q_temp1__blk814_dn9)), );
            locals.var_q_aexp__blk824_rv = 0.0;
        }
        if (((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) {
            let assign38730_e43784: f64 = (locals.var_q_k1q1__blk823 * locals.var_q_k1q1__blk823);
            let assign38730_e43786: f64 = (assign38730_e43784 - locals.var_q_aexp__blk824);
            (locals.var_q_qsq__blk825, locals.var_q_qsq__blk825_dn4, locals.var_q_qsq__blk825_dn6, locals.var_q_qsq__blk825_dn7, locals.var_q_qsq__blk825_dn8, locals.var_q_qsq__blk825_dn9, ) = (assign38730_e43786, (((locals.var_q_k1q1__blk823_dn4 * locals.var_q_k1q1__blk823) + (locals.var_q_k1q1__blk823 * locals.var_q_k1q1__blk823_dn4)) - locals.var_q_aexp__blk824_dn4), (((locals.var_q_k1q1__blk823_dn6 * locals.var_q_k1q1__blk823) + (locals.var_q_k1q1__blk823 * locals.var_q_k1q1__blk823_dn6)) - locals.var_q_aexp__blk824_dn6), (((locals.var_q_k1q1__blk823_dn7 * locals.var_q_k1q1__blk823) + (locals.var_q_k1q1__blk823 * locals.var_q_k1q1__blk823_dn7)) - locals.var_q_aexp__blk824_dn7), (((locals.var_q_k1q1__blk823_dn8 * locals.var_q_k1q1__blk823) + (locals.var_q_k1q1__blk823 * locals.var_q_k1q1__blk823_dn8)) - locals.var_q_aexp__blk824_dn8), (((locals.var_q_k1q1__blk823_dn9 * locals.var_q_k1q1__blk823) + (locals.var_q_k1q1__blk823 * locals.var_q_k1q1__blk823_dn9)) - locals.var_q_aexp__blk824_dn9), );
            locals.var_q_qsq__blk825_rv = 0.0;
        }
        if (((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) {
            let assign38740_e43796: f64 = (2.0 * locals.var_k1__blk932);
            let assign38740_e43798: f64 = (assign38740_e43796 * locals.var_q_k1q1__blk823);
            let assign38740_e43800: f64 = (assign38740_e43798 + locals.var_q_aexp__blk824);
            (locals.var_q_d1_qsq__blk826, locals.var_q_d1_qsq__blk826_dn4, locals.var_q_d1_qsq__blk826_dn6, locals.var_q_d1_qsq__blk826_dn7, locals.var_q_d1_qsq__blk826_dn8, locals.var_q_d1_qsq__blk826_dn9, ) = (assign38740_e43800, ((((2.0 * locals.var_k1__blk932_dn4) * locals.var_q_k1q1__blk823) + (assign38740_e43796 * locals.var_q_k1q1__blk823_dn4)) + locals.var_q_aexp__blk824_dn4), ((((2.0 * locals.var_k1__blk932_dn6) * locals.var_q_k1q1__blk823) + (assign38740_e43796 * locals.var_q_k1q1__blk823_dn6)) + locals.var_q_aexp__blk824_dn6), ((((2.0 * locals.var_k1__blk932_dn7) * locals.var_q_k1q1__blk823) + (assign38740_e43796 * locals.var_q_k1q1__blk823_dn7)) + locals.var_q_aexp__blk824_dn7), ((((2.0 * locals.var_k1__blk932_dn8) * locals.var_q_k1q1__blk823) + (assign38740_e43796 * locals.var_q_k1q1__blk823_dn8)) + locals.var_q_aexp__blk824_dn8), ((((2.0 * locals.var_k1__blk932_dn9) * locals.var_q_k1q1__blk823) + (assign38740_e43796 * locals.var_q_k1q1__blk823_dn9)) + locals.var_q_aexp__blk824_dn9), );
            locals.var_q_d1_qsq__blk826_rv = 0.0;
        }
        if (((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) {
            let assign38750_e43810: f64 = (2.0 * locals.var_k1__blk932);
            let assign38750_e43812: f64 = (assign38750_e43810 * locals.var_k1__blk932);
            let assign38750_e43814: f64 = (assign38750_e43812 - locals.var_q_aexp__blk824);
            (locals.var_q_d2_qsq__blk827, locals.var_q_d2_qsq__blk827_dn4, locals.var_q_d2_qsq__blk827_dn6, locals.var_q_d2_qsq__blk827_dn7, locals.var_q_d2_qsq__blk827_dn8, locals.var_q_d2_qsq__blk827_dn9, ) = (assign38750_e43814, ((((2.0 * locals.var_k1__blk932_dn4) * locals.var_k1__blk932) + (assign38750_e43810 * locals.var_k1__blk932_dn4)) - locals.var_q_aexp__blk824_dn4), ((((2.0 * locals.var_k1__blk932_dn6) * locals.var_k1__blk932) + (assign38750_e43810 * locals.var_k1__blk932_dn6)) - locals.var_q_aexp__blk824_dn6), ((((2.0 * locals.var_k1__blk932_dn7) * locals.var_k1__blk932) + (assign38750_e43810 * locals.var_k1__blk932_dn7)) - locals.var_q_aexp__blk824_dn7), ((((2.0 * locals.var_k1__blk932_dn8) * locals.var_k1__blk932) + (assign38750_e43810 * locals.var_k1__blk932_dn8)) - locals.var_q_aexp__blk824_dn8), ((((2.0 * locals.var_k1__blk932_dn9) * locals.var_k1__blk932) + (assign38750_e43810 * locals.var_k1__blk932_dn9)) - locals.var_q_aexp__blk824_dn9), );
            locals.var_q_d2_qsq__blk827_rv = 0.0;
        }
        let assign38760_e43819: f64 = (-0.005);
        let assign38760_e43820: f64 = if locals.var_q_qsq__blk825 < assign38760_e43819 { 1.0 } else { 0.0 };
        locals.var_guard1199 = assign38760_e43820;
        locals.var_guard1199_rv = 0.0;
        if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1199 != 0.0)) {
            let assign38770_e43829: f64 = (locals.var_q_qsq__blk825).abs();
            let assign38770_e43830: f64 = (assign38770_e43829).sqrt();
            (locals.var_q_rac_qsq__blk828, locals.var_q_rac_qsq__blk828_dn4, locals.var_q_rac_qsq__blk828_dn6, locals.var_q_rac_qsq__blk828_dn7, locals.var_q_rac_qsq__blk828_dn8, locals.var_q_rac_qsq__blk828_dn9, ) = (assign38770_e43830, (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn4 } else { (-locals.var_q_qsq__blk825_dn4) } / (2.0 * assign38770_e43830)), (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn6 } else { (-locals.var_q_qsq__blk825_dn6) } / (2.0 * assign38770_e43830)), (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn7 } else { (-locals.var_q_qsq__blk825_dn7) } / (2.0 * assign38770_e43830)), (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn8 } else { (-locals.var_q_qsq__blk825_dn8) } / (2.0 * assign38770_e43830)), (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn9 } else { (-locals.var_q_qsq__blk825_dn9) } / (2.0 * assign38770_e43830)), );
            locals.var_q_rac_qsq__blk828_rv = 0.0;
        }
        if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1199 != 0.0)) {
            let assign38780_e43843: f64 = (0.5 * locals.var_q_rac_qsq__blk828);
            let assign38780_e43844: f64 = (assign38780_e43843).tan();
            let assign38780_e43845: f64 = (locals.var_q_rac_qsq__blk828 / assign38780_e43844);
            (locals.var_q_qcoth__blk829, locals.var_q_qcoth__blk829_dn4, locals.var_q_qcoth__blk829_dn6, locals.var_q_qcoth__blk829_dn7, locals.var_q_qcoth__blk829_dn8, locals.var_q_qcoth__blk829_dn9, ) = (assign38780_e43845, (((locals.var_q_rac_qsq__blk828_dn4 * assign38780_e43844) - (locals.var_q_rac_qsq__blk828 * ((0.5 * locals.var_q_rac_qsq__blk828_dn4) / ((assign38780_e43843).cos() * (assign38780_e43843).cos())))) / (assign38780_e43844 * assign38780_e43844)), (((locals.var_q_rac_qsq__blk828_dn6 * assign38780_e43844) - (locals.var_q_rac_qsq__blk828 * ((0.5 * locals.var_q_rac_qsq__blk828_dn6) / ((assign38780_e43843).cos() * (assign38780_e43843).cos())))) / (assign38780_e43844 * assign38780_e43844)), (((locals.var_q_rac_qsq__blk828_dn7 * assign38780_e43844) - (locals.var_q_rac_qsq__blk828 * ((0.5 * locals.var_q_rac_qsq__blk828_dn7) / ((assign38780_e43843).cos() * (assign38780_e43843).cos())))) / (assign38780_e43844 * assign38780_e43844)), (((locals.var_q_rac_qsq__blk828_dn8 * assign38780_e43844) - (locals.var_q_rac_qsq__blk828 * ((0.5 * locals.var_q_rac_qsq__blk828_dn8) / ((assign38780_e43843).cos() * (assign38780_e43843).cos())))) / (assign38780_e43844 * assign38780_e43844)), (((locals.var_q_rac_qsq__blk828_dn9 * assign38780_e43844) - (locals.var_q_rac_qsq__blk828 * ((0.5 * locals.var_q_rac_qsq__blk828_dn9) / ((assign38780_e43843).cos() * (assign38780_e43843).cos())))) / (assign38780_e43844 * assign38780_e43844)), );
            locals.var_q_qcoth__blk829_rv = 0.0;
        }
        if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1199 != 0.0)) {
            let assign38790_e43857: f64 = (0.25 * locals.var_q_d1_qsq__blk826);
            let assign38790_e43859: f64 = (assign38790_e43857 / locals.var_q_qsq__blk825);
            (locals.var_q_temp1__blk814, locals.var_q_temp1__blk814_dn4, locals.var_q_temp1__blk814_dn6, locals.var_q_temp1__blk814_dn7, locals.var_q_temp1__blk814_dn8, locals.var_q_temp1__blk814_dn9, ) = (assign38790_e43859, ((((0.25 * locals.var_q_d1_qsq__blk826_dn4) * locals.var_q_qsq__blk825) - (assign38790_e43857 * locals.var_q_qsq__blk825_dn4)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((0.25 * locals.var_q_d1_qsq__blk826_dn6) * locals.var_q_qsq__blk825) - (assign38790_e43857 * locals.var_q_qsq__blk825_dn6)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((0.25 * locals.var_q_d1_qsq__blk826_dn7) * locals.var_q_qsq__blk825) - (assign38790_e43857 * locals.var_q_qsq__blk825_dn7)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((0.25 * locals.var_q_d1_qsq__blk826_dn8) * locals.var_q_qsq__blk825) - (assign38790_e43857 * locals.var_q_qsq__blk825_dn8)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((0.25 * locals.var_q_d1_qsq__blk826_dn9) * locals.var_q_qsq__blk825) - (assign38790_e43857 * locals.var_q_qsq__blk825_dn9)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), );
            locals.var_q_temp1__blk814_rv = 0.0;
        }
        if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1199 != 0.0)) {
            let assign38800_e43873: f64 = (2.0 - locals.var_q_qcoth__blk829);
            let assign38800_e43874: f64 = (locals.var_q_qcoth__blk829 * assign38800_e43873);
            let assign38800_e43875: f64 = (locals.var_q_qsq__blk825 + assign38800_e43874);
            let assign38800_e43877: f64 = (assign38800_e43875 * locals.var_q_temp1__blk814);
            (locals.var_q_d1_qcoth__blk830, locals.var_q_d1_qcoth__blk830_dn4, locals.var_q_d1_qcoth__blk830_dn6, locals.var_q_d1_qcoth__blk830_dn7, locals.var_q_d1_qcoth__blk830_dn8, locals.var_q_d1_qcoth__blk830_dn9, ) = (assign38800_e43877, (((locals.var_q_qsq__blk825_dn4 + ((locals.var_q_qcoth__blk829_dn4 * assign38800_e43873) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn4)))) * locals.var_q_temp1__blk814) + (assign38800_e43875 * locals.var_q_temp1__blk814_dn4)), (((locals.var_q_qsq__blk825_dn6 + ((locals.var_q_qcoth__blk829_dn6 * assign38800_e43873) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn6)))) * locals.var_q_temp1__blk814) + (assign38800_e43875 * locals.var_q_temp1__blk814_dn6)), (((locals.var_q_qsq__blk825_dn7 + ((locals.var_q_qcoth__blk829_dn7 * assign38800_e43873) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn7)))) * locals.var_q_temp1__blk814) + (assign38800_e43875 * locals.var_q_temp1__blk814_dn7)), (((locals.var_q_qsq__blk825_dn8 + ((locals.var_q_qcoth__blk829_dn8 * assign38800_e43873) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn8)))) * locals.var_q_temp1__blk814) + (assign38800_e43875 * locals.var_q_temp1__blk814_dn8)), (((locals.var_q_qsq__blk825_dn9 + ((locals.var_q_qcoth__blk829_dn9 * assign38800_e43873) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn9)))) * locals.var_q_temp1__blk814) + (assign38800_e43875 * locals.var_q_temp1__blk814_dn9)), );
            locals.var_q_d1_qcoth__blk830_rv = 0.0;
        }
        if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1199 != 0.0)) {
            let assign38810_e43890: f64 = (2.0 * locals.var_q_d1_qcoth__blk830);
            let assign38810_e43893: f64 = (1.0 + locals.var_q_qcoth__blk829);
            let assign38810_e43894: f64 = (assign38810_e43890 * assign38810_e43893);
            let assign38810_e43895: f64 = (locals.var_q_d1_qsq__blk826 - assign38810_e43894);
            let assign38810_e43897: f64 = (assign38810_e43895 * locals.var_q_temp1__blk814);
            let assign38810_e43900: f64 = (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827);
            let assign38810_e43902: f64 = (assign38810_e43900 / locals.var_q_d1_qsq__blk826);
            let assign38810_e43903: f64 = (assign38810_e43897 + assign38810_e43902);
            (locals.var_q_d2_qcoth__blk832, locals.var_q_d2_qcoth__blk832_dn4, locals.var_q_d2_qcoth__blk832_dn6, locals.var_q_d2_qcoth__blk832_dn7, locals.var_q_d2_qcoth__blk832_dn8, locals.var_q_d2_qcoth__blk832_dn9, ) = (assign38810_e43903, ((((locals.var_q_d1_qsq__blk826_dn4 - (((2.0 * locals.var_q_d1_qcoth__blk830_dn4) * assign38810_e43893) + (assign38810_e43890 * locals.var_q_qcoth__blk829_dn4))) * locals.var_q_temp1__blk814) + (assign38810_e43895 * locals.var_q_temp1__blk814_dn4)) + (((((locals.var_q_d1_qcoth__blk830_dn4 * locals.var_q_d2_qsq__blk827) + (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827_dn4)) * locals.var_q_d1_qsq__blk826) - (assign38810_e43900 * locals.var_q_d1_qsq__blk826_dn4)) / (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826))), ((((locals.var_q_d1_qsq__blk826_dn6 - (((2.0 * locals.var_q_d1_qcoth__blk830_dn6) * assign38810_e43893) + (assign38810_e43890 * locals.var_q_qcoth__blk829_dn6))) * locals.var_q_temp1__blk814) + (assign38810_e43895 * locals.var_q_temp1__blk814_dn6)) + (((((locals.var_q_d1_qcoth__blk830_dn6 * locals.var_q_d2_qsq__blk827) + (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827_dn6)) * locals.var_q_d1_qsq__blk826) - (assign38810_e43900 * locals.var_q_d1_qsq__blk826_dn6)) / (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826))), ((((locals.var_q_d1_qsq__blk826_dn7 - (((2.0 * locals.var_q_d1_qcoth__blk830_dn7) * assign38810_e43893) + (assign38810_e43890 * locals.var_q_qcoth__blk829_dn7))) * locals.var_q_temp1__blk814) + (assign38810_e43895 * locals.var_q_temp1__blk814_dn7)) + (((((locals.var_q_d1_qcoth__blk830_dn7 * locals.var_q_d2_qsq__blk827) + (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827_dn7)) * locals.var_q_d1_qsq__blk826) - (assign38810_e43900 * locals.var_q_d1_qsq__blk826_dn7)) / (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826))), ((((locals.var_q_d1_qsq__blk826_dn8 - (((2.0 * locals.var_q_d1_qcoth__blk830_dn8) * assign38810_e43893) + (assign38810_e43890 * locals.var_q_qcoth__blk829_dn8))) * locals.var_q_temp1__blk814) + (assign38810_e43895 * locals.var_q_temp1__blk814_dn8)) + (((((locals.var_q_d1_qcoth__blk830_dn8 * locals.var_q_d2_qsq__blk827) + (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827_dn8)) * locals.var_q_d1_qsq__blk826) - (assign38810_e43900 * locals.var_q_d1_qsq__blk826_dn8)) / (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826))), ((((locals.var_q_d1_qsq__blk826_dn9 - (((2.0 * locals.var_q_d1_qcoth__blk830_dn9) * assign38810_e43893) + (assign38810_e43890 * locals.var_q_qcoth__blk829_dn9))) * locals.var_q_temp1__blk814) + (assign38810_e43895 * locals.var_q_temp1__blk814_dn9)) + (((((locals.var_q_d1_qcoth__blk830_dn9 * locals.var_q_d2_qsq__blk827) + (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827_dn9)) * locals.var_q_d1_qsq__blk826) - (assign38810_e43900 * locals.var_q_d1_qsq__blk826_dn9)) / (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826))), );
            locals.var_q_d2_qcoth__blk832_rv = 0.0;
        }
        if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1199 != 0.0)) {
            let assign38820_e43916: f64 = (0.5 * locals.var_q_qcoth__blk829);
            let assign38820_e43917: f64 = (1.0 - assign38820_e43916);
            (locals.var_q_temp2__blk815, locals.var_q_temp2__blk815_dn4, locals.var_q_temp2__blk815_dn6, locals.var_q_temp2__blk815_dn7, locals.var_q_temp2__blk815_dn8, locals.var_q_temp2__blk815_dn9, ) = (assign38820_e43917, (-(0.5 * locals.var_q_qcoth__blk829_dn4)), (-(0.5 * locals.var_q_qcoth__blk829_dn6)), (-(0.5 * locals.var_q_qcoth__blk829_dn7)), (-(0.5 * locals.var_q_qcoth__blk829_dn8)), (-(0.5 * locals.var_q_qcoth__blk829_dn9)), );
            locals.var_q_temp2__blk815_rv = 0.0;
        }
        if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1199 != 0.0)) {
            let assign38830_e43929: f64 = (locals.var_q_d1_qsq__blk826 / locals.var_q_qsq__blk825);
            let assign38830_e43931: f64 = (assign38830_e43929 * locals.var_q_temp2__blk815);
            (locals.var_q_d1_ln__blk835, locals.var_q_d1_ln__blk835_dn4, locals.var_q_d1_ln__blk835_dn6, locals.var_q_d1_ln__blk835_dn7, locals.var_q_d1_ln__blk835_dn8, locals.var_q_d1_ln__blk835_dn9, ) = (assign38830_e43931, (((((locals.var_q_d1_qsq__blk826_dn4 * locals.var_q_qsq__blk825) - (locals.var_q_d1_qsq__blk826 * locals.var_q_qsq__blk825_dn4)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)) * locals.var_q_temp2__blk815) + (assign38830_e43929 * locals.var_q_temp2__blk815_dn4)), (((((locals.var_q_d1_qsq__blk826_dn6 * locals.var_q_qsq__blk825) - (locals.var_q_d1_qsq__blk826 * locals.var_q_qsq__blk825_dn6)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)) * locals.var_q_temp2__blk815) + (assign38830_e43929 * locals.var_q_temp2__blk815_dn6)), (((((locals.var_q_d1_qsq__blk826_dn7 * locals.var_q_qsq__blk825) - (locals.var_q_d1_qsq__blk826 * locals.var_q_qsq__blk825_dn7)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)) * locals.var_q_temp2__blk815) + (assign38830_e43929 * locals.var_q_temp2__blk815_dn7)), (((((locals.var_q_d1_qsq__blk826_dn8 * locals.var_q_qsq__blk825) - (locals.var_q_d1_qsq__blk826 * locals.var_q_qsq__blk825_dn8)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)) * locals.var_q_temp2__blk815) + (assign38830_e43929 * locals.var_q_temp2__blk815_dn8)), (((((locals.var_q_d1_qsq__blk826_dn9 * locals.var_q_qsq__blk825) - (locals.var_q_d1_qsq__blk826 * locals.var_q_qsq__blk825_dn9)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)) * locals.var_q_temp2__blk815) + (assign38830_e43929 * locals.var_q_temp2__blk815_dn9)), );
            locals.var_q_d1_ln__blk835_rv = 0.0;
        }
        if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1199 != 0.0)) {
            let assign38840_e43943: f64 = (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815);
            let assign38840_e43948: f64 = (0.5 * locals.var_q_d1_qcoth__blk830);
            let assign38840_e43949: f64 = (locals.var_q_d1_ln__blk835 + assign38840_e43948);
            let assign38840_e43950: f64 = (locals.var_q_d1_qsq__blk826 * assign38840_e43949);
            let assign38840_e43951: f64 = (assign38840_e43943 - assign38840_e43950);
            let assign38840_e43953: f64 = (assign38840_e43951 / locals.var_q_qsq__blk825);
            (locals.var_q_d2_ln__blk836, locals.var_q_d2_ln__blk836_dn4, locals.var_q_d2_ln__blk836_dn6, locals.var_q_d2_ln__blk836_dn7, locals.var_q_d2_ln__blk836_dn8, locals.var_q_d2_ln__blk836_dn9, ) = (assign38840_e43953, ((((((locals.var_q_d2_qsq__blk827_dn4 * locals.var_q_temp2__blk815) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815_dn4)) - ((locals.var_q_d1_qsq__blk826_dn4 * assign38840_e43949) + (locals.var_q_d1_qsq__blk826 * (locals.var_q_d1_ln__blk835_dn4 + (0.5 * locals.var_q_d1_qcoth__blk830_dn4))))) * locals.var_q_qsq__blk825) - (assign38840_e43951 * locals.var_q_qsq__blk825_dn4)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((((locals.var_q_d2_qsq__blk827_dn6 * locals.var_q_temp2__blk815) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815_dn6)) - ((locals.var_q_d1_qsq__blk826_dn6 * assign38840_e43949) + (locals.var_q_d1_qsq__blk826 * (locals.var_q_d1_ln__blk835_dn6 + (0.5 * locals.var_q_d1_qcoth__blk830_dn6))))) * locals.var_q_qsq__blk825) - (assign38840_e43951 * locals.var_q_qsq__blk825_dn6)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((((locals.var_q_d2_qsq__blk827_dn7 * locals.var_q_temp2__blk815) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815_dn7)) - ((locals.var_q_d1_qsq__blk826_dn7 * assign38840_e43949) + (locals.var_q_d1_qsq__blk826 * (locals.var_q_d1_ln__blk835_dn7 + (0.5 * locals.var_q_d1_qcoth__blk830_dn7))))) * locals.var_q_qsq__blk825) - (assign38840_e43951 * locals.var_q_qsq__blk825_dn7)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((((locals.var_q_d2_qsq__blk827_dn8 * locals.var_q_temp2__blk815) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815_dn8)) - ((locals.var_q_d1_qsq__blk826_dn8 * assign38840_e43949) + (locals.var_q_d1_qsq__blk826 * (locals.var_q_d1_ln__blk835_dn8 + (0.5 * locals.var_q_d1_qcoth__blk830_dn8))))) * locals.var_q_qsq__blk825) - (assign38840_e43951 * locals.var_q_qsq__blk825_dn8)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((((locals.var_q_d2_qsq__blk827_dn9 * locals.var_q_temp2__blk815) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815_dn9)) - ((locals.var_q_d1_qsq__blk826_dn9 * assign38840_e43949) + (locals.var_q_d1_qsq__blk826 * (locals.var_q_d1_ln__blk835_dn9 + (0.5 * locals.var_q_d1_qcoth__blk830_dn9))))) * locals.var_q_qsq__blk825) - (assign38840_e43951 * locals.var_q_qsq__blk825_dn9)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), );
            locals.var_q_d2_ln__blk836_rv = 0.0;
        }
        let assign38850_e43958: f64 = if locals.var_q_qsq__blk825 > 0.005 { 1.0 } else { 0.0 };
        locals.var_guard1200 = assign38850_e43958;
        locals.var_guard1200_rv = 0.0;
        if (((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1199 == 0.0)) && (locals.var_guard1200 != 0.0)) {
            let assign38860_e43970: f64 = (locals.var_q_qsq__blk825).abs();
            let assign38860_e43971: f64 = (assign38860_e43970).sqrt();
            (locals.var_q_rac_qsq__blk828, locals.var_q_rac_qsq__blk828_dn4, locals.var_q_rac_qsq__blk828_dn6, locals.var_q_rac_qsq__blk828_dn7, locals.var_q_rac_qsq__blk828_dn8, locals.var_q_rac_qsq__blk828_dn9, ) = (assign38860_e43971, (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn4 } else { (-locals.var_q_qsq__blk825_dn4) } / (2.0 * assign38860_e43971)), (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn6 } else { (-locals.var_q_qsq__blk825_dn6) } / (2.0 * assign38860_e43971)), (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn7 } else { (-locals.var_q_qsq__blk825_dn7) } / (2.0 * assign38860_e43971)), (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn8 } else { (-locals.var_q_qsq__blk825_dn8) } / (2.0 * assign38860_e43971)), (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn9 } else { (-locals.var_q_qsq__blk825_dn9) } / (2.0 * assign38860_e43971)), );
            locals.var_q_rac_qsq__blk828_rv = 0.0;
        }
        if (((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1199 == 0.0)) && (locals.var_guard1200 != 0.0)) {
            let assign38870_e43985: f64 = (-locals.var_q_rac_qsq__blk828);
            let assign38870_e43986: f64 = (assign38870_e43985).exp();
            (locals.var_q_invexpq__blk831, locals.var_q_invexpq__blk831_dn4, locals.var_q_invexpq__blk831_dn6, locals.var_q_invexpq__blk831_dn7, locals.var_q_invexpq__blk831_dn8, locals.var_q_invexpq__blk831_dn9, ) = (assign38870_e43986, (assign38870_e43986 * (-locals.var_q_rac_qsq__blk828_dn4)), (assign38870_e43986 * (-locals.var_q_rac_qsq__blk828_dn6)), (assign38870_e43986 * (-locals.var_q_rac_qsq__blk828_dn7)), (assign38870_e43986 * (-locals.var_q_rac_qsq__blk828_dn8)), (assign38870_e43986 * (-locals.var_q_rac_qsq__blk828_dn9)), );
            locals.var_q_invexpq__blk831_rv = 0.0;
        }
        if (((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1199 == 0.0)) && (locals.var_guard1200 != 0.0)) {
            let assign38880_e44002: f64 = (1.0 + locals.var_q_invexpq__blk831);
            let assign38880_e44003: f64 = (locals.var_q_rac_qsq__blk828 * assign38880_e44002);
            let assign38880_e44006: f64 = (1.0 - locals.var_q_invexpq__blk831);
            let assign38880_e44007: f64 = (assign38880_e44003 / assign38880_e44006);
            (locals.var_q_qcoth__blk829, locals.var_q_qcoth__blk829_dn4, locals.var_q_qcoth__blk829_dn6, locals.var_q_qcoth__blk829_dn7, locals.var_q_qcoth__blk829_dn8, locals.var_q_qcoth__blk829_dn9, ) = (assign38880_e44007, (((((locals.var_q_rac_qsq__blk828_dn4 * assign38880_e44002) + (locals.var_q_rac_qsq__blk828 * locals.var_q_invexpq__blk831_dn4)) * assign38880_e44006) - (assign38880_e44003 * (-locals.var_q_invexpq__blk831_dn4))) / (assign38880_e44006 * assign38880_e44006)), (((((locals.var_q_rac_qsq__blk828_dn6 * assign38880_e44002) + (locals.var_q_rac_qsq__blk828 * locals.var_q_invexpq__blk831_dn6)) * assign38880_e44006) - (assign38880_e44003 * (-locals.var_q_invexpq__blk831_dn6))) / (assign38880_e44006 * assign38880_e44006)), (((((locals.var_q_rac_qsq__blk828_dn7 * assign38880_e44002) + (locals.var_q_rac_qsq__blk828 * locals.var_q_invexpq__blk831_dn7)) * assign38880_e44006) - (assign38880_e44003 * (-locals.var_q_invexpq__blk831_dn7))) / (assign38880_e44006 * assign38880_e44006)), (((((locals.var_q_rac_qsq__blk828_dn8 * assign38880_e44002) + (locals.var_q_rac_qsq__blk828 * locals.var_q_invexpq__blk831_dn8)) * assign38880_e44006) - (assign38880_e44003 * (-locals.var_q_invexpq__blk831_dn8))) / (assign38880_e44006 * assign38880_e44006)), (((((locals.var_q_rac_qsq__blk828_dn9 * assign38880_e44002) + (locals.var_q_rac_qsq__blk828 * locals.var_q_invexpq__blk831_dn9)) * assign38880_e44006) - (assign38880_e44003 * (-locals.var_q_invexpq__blk831_dn9))) / (assign38880_e44006 * assign38880_e44006)), );
            locals.var_q_qcoth__blk829_rv = 0.0;
        }
        if (((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1199 == 0.0)) && (locals.var_guard1200 != 0.0)) {
            let assign38890_e44022: f64 = (0.25 * locals.var_q_d1_qsq__blk826);
            let assign38890_e44024: f64 = (assign38890_e44022 / locals.var_q_qsq__blk825);
            (locals.var_q_temp1__blk814, locals.var_q_temp1__blk814_dn4, locals.var_q_temp1__blk814_dn6, locals.var_q_temp1__blk814_dn7, locals.var_q_temp1__blk814_dn8, locals.var_q_temp1__blk814_dn9, ) = (assign38890_e44024, ((((0.25 * locals.var_q_d1_qsq__blk826_dn4) * locals.var_q_qsq__blk825) - (assign38890_e44022 * locals.var_q_qsq__blk825_dn4)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((0.25 * locals.var_q_d1_qsq__blk826_dn6) * locals.var_q_qsq__blk825) - (assign38890_e44022 * locals.var_q_qsq__blk825_dn6)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((0.25 * locals.var_q_d1_qsq__blk826_dn7) * locals.var_q_qsq__blk825) - (assign38890_e44022 * locals.var_q_qsq__blk825_dn7)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((0.25 * locals.var_q_d1_qsq__blk826_dn8) * locals.var_q_qsq__blk825) - (assign38890_e44022 * locals.var_q_qsq__blk825_dn8)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((0.25 * locals.var_q_d1_qsq__blk826_dn9) * locals.var_q_qsq__blk825) - (assign38890_e44022 * locals.var_q_qsq__blk825_dn9)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), );
            locals.var_q_temp1__blk814_rv = 0.0;
        }
        if (((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1199 == 0.0)) && (locals.var_guard1200 != 0.0)) {
            let assign38900_e44041: f64 = (2.0 - locals.var_q_qcoth__blk829);
            let assign38900_e44042: f64 = (locals.var_q_qcoth__blk829 * assign38900_e44041);
            let assign38900_e44043: f64 = (locals.var_q_qsq__blk825 + assign38900_e44042);
            let assign38900_e44045: f64 = (assign38900_e44043 * locals.var_q_temp1__blk814);
            (locals.var_q_d1_qcoth__blk830, locals.var_q_d1_qcoth__blk830_dn4, locals.var_q_d1_qcoth__blk830_dn6, locals.var_q_d1_qcoth__blk830_dn7, locals.var_q_d1_qcoth__blk830_dn8, locals.var_q_d1_qcoth__blk830_dn9, ) = (assign38900_e44045, (((locals.var_q_qsq__blk825_dn4 + ((locals.var_q_qcoth__blk829_dn4 * assign38900_e44041) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn4)))) * locals.var_q_temp1__blk814) + (assign38900_e44043 * locals.var_q_temp1__blk814_dn4)), (((locals.var_q_qsq__blk825_dn6 + ((locals.var_q_qcoth__blk829_dn6 * assign38900_e44041) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn6)))) * locals.var_q_temp1__blk814) + (assign38900_e44043 * locals.var_q_temp1__blk814_dn6)), (((locals.var_q_qsq__blk825_dn7 + ((locals.var_q_qcoth__blk829_dn7 * assign38900_e44041) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn7)))) * locals.var_q_temp1__blk814) + (assign38900_e44043 * locals.var_q_temp1__blk814_dn7)), (((locals.var_q_qsq__blk825_dn8 + ((locals.var_q_qcoth__blk829_dn8 * assign38900_e44041) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn8)))) * locals.var_q_temp1__blk814) + (assign38900_e44043 * locals.var_q_temp1__blk814_dn8)), (((locals.var_q_qsq__blk825_dn9 + ((locals.var_q_qcoth__blk829_dn9 * assign38900_e44041) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn9)))) * locals.var_q_temp1__blk814) + (assign38900_e44043 * locals.var_q_temp1__blk814_dn9)), );
            locals.var_q_d1_qcoth__blk830_rv = 0.0;
        }
        if (((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1199 == 0.0)) && (locals.var_guard1200 != 0.0)) {
            let assign38910_e44061: f64 = (2.0 * locals.var_q_d1_qcoth__blk830);
            let assign38910_e44064: f64 = (1.0 + locals.var_q_qcoth__blk829);
            let assign38910_e44065: f64 = (assign38910_e44061 * assign38910_e44064);
            let assign38910_e44066: f64 = (locals.var_q_d1_qsq__blk826 - assign38910_e44065);
            let assign38910_e44068: f64 = (assign38910_e44066 * locals.var_q_temp1__blk814);
            let assign38910_e44071: f64 = (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827);
            let assign38910_e44073: f64 = (assign38910_e44071 / locals.var_q_d1_qsq__blk826);
            let assign38910_e44074: f64 = (assign38910_e44068 + assign38910_e44073);
            (locals.var_q_d2_qcoth__blk832, locals.var_q_d2_qcoth__blk832_dn4, locals.var_q_d2_qcoth__blk832_dn6, locals.var_q_d2_qcoth__blk832_dn7, locals.var_q_d2_qcoth__blk832_dn8, locals.var_q_d2_qcoth__blk832_dn9, ) = (assign38910_e44074, ((((locals.var_q_d1_qsq__blk826_dn4 - (((2.0 * locals.var_q_d1_qcoth__blk830_dn4) * assign38910_e44064) + (assign38910_e44061 * locals.var_q_qcoth__blk829_dn4))) * locals.var_q_temp1__blk814) + (assign38910_e44066 * locals.var_q_temp1__blk814_dn4)) + (((((locals.var_q_d1_qcoth__blk830_dn4 * locals.var_q_d2_qsq__blk827) + (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827_dn4)) * locals.var_q_d1_qsq__blk826) - (assign38910_e44071 * locals.var_q_d1_qsq__blk826_dn4)) / (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826))), ((((locals.var_q_d1_qsq__blk826_dn6 - (((2.0 * locals.var_q_d1_qcoth__blk830_dn6) * assign38910_e44064) + (assign38910_e44061 * locals.var_q_qcoth__blk829_dn6))) * locals.var_q_temp1__blk814) + (assign38910_e44066 * locals.var_q_temp1__blk814_dn6)) + (((((locals.var_q_d1_qcoth__blk830_dn6 * locals.var_q_d2_qsq__blk827) + (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827_dn6)) * locals.var_q_d1_qsq__blk826) - (assign38910_e44071 * locals.var_q_d1_qsq__blk826_dn6)) / (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826))), ((((locals.var_q_d1_qsq__blk826_dn7 - (((2.0 * locals.var_q_d1_qcoth__blk830_dn7) * assign38910_e44064) + (assign38910_e44061 * locals.var_q_qcoth__blk829_dn7))) * locals.var_q_temp1__blk814) + (assign38910_e44066 * locals.var_q_temp1__blk814_dn7)) + (((((locals.var_q_d1_qcoth__blk830_dn7 * locals.var_q_d2_qsq__blk827) + (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827_dn7)) * locals.var_q_d1_qsq__blk826) - (assign38910_e44071 * locals.var_q_d1_qsq__blk826_dn7)) / (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826))), ((((locals.var_q_d1_qsq__blk826_dn8 - (((2.0 * locals.var_q_d1_qcoth__blk830_dn8) * assign38910_e44064) + (assign38910_e44061 * locals.var_q_qcoth__blk829_dn8))) * locals.var_q_temp1__blk814) + (assign38910_e44066 * locals.var_q_temp1__blk814_dn8)) + (((((locals.var_q_d1_qcoth__blk830_dn8 * locals.var_q_d2_qsq__blk827) + (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827_dn8)) * locals.var_q_d1_qsq__blk826) - (assign38910_e44071 * locals.var_q_d1_qsq__blk826_dn8)) / (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826))), ((((locals.var_q_d1_qsq__blk826_dn9 - (((2.0 * locals.var_q_d1_qcoth__blk830_dn9) * assign38910_e44064) + (assign38910_e44061 * locals.var_q_qcoth__blk829_dn9))) * locals.var_q_temp1__blk814) + (assign38910_e44066 * locals.var_q_temp1__blk814_dn9)) + (((((locals.var_q_d1_qcoth__blk830_dn9 * locals.var_q_d2_qsq__blk827) + (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827_dn9)) * locals.var_q_d1_qsq__blk826) - (assign38910_e44071 * locals.var_q_d1_qsq__blk826_dn9)) / (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826))), );
            locals.var_q_d2_qcoth__blk832_rv = 0.0;
        }
        if (((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1199 == 0.0)) && (locals.var_guard1200 != 0.0)) {
            let assign38920_e44090: f64 = (0.5 * locals.var_q_qcoth__blk829);
            let assign38920_e44091: f64 = (1.0 - assign38920_e44090);
            (locals.var_q_temp2__blk815, locals.var_q_temp2__blk815_dn4, locals.var_q_temp2__blk815_dn6, locals.var_q_temp2__blk815_dn7, locals.var_q_temp2__blk815_dn8, locals.var_q_temp2__blk815_dn9, ) = (assign38920_e44091, (-(0.5 * locals.var_q_qcoth__blk829_dn4)), (-(0.5 * locals.var_q_qcoth__blk829_dn6)), (-(0.5 * locals.var_q_qcoth__blk829_dn7)), (-(0.5 * locals.var_q_qcoth__blk829_dn8)), (-(0.5 * locals.var_q_qcoth__blk829_dn9)), );
            locals.var_q_temp2__blk815_rv = 0.0;
        }
        if (((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1199 == 0.0)) && (locals.var_guard1200 != 0.0)) {
            let assign38930_e44106: f64 = (locals.var_q_d1_qsq__blk826 / locals.var_q_qsq__blk825);
            let assign38930_e44108: f64 = (assign38930_e44106 * locals.var_q_temp2__blk815);
            (locals.var_q_d1_ln__blk835, locals.var_q_d1_ln__blk835_dn4, locals.var_q_d1_ln__blk835_dn6, locals.var_q_d1_ln__blk835_dn7, locals.var_q_d1_ln__blk835_dn8, locals.var_q_d1_ln__blk835_dn9, ) = (assign38930_e44108, (((((locals.var_q_d1_qsq__blk826_dn4 * locals.var_q_qsq__blk825) - (locals.var_q_d1_qsq__blk826 * locals.var_q_qsq__blk825_dn4)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)) * locals.var_q_temp2__blk815) + (assign38930_e44106 * locals.var_q_temp2__blk815_dn4)), (((((locals.var_q_d1_qsq__blk826_dn6 * locals.var_q_qsq__blk825) - (locals.var_q_d1_qsq__blk826 * locals.var_q_qsq__blk825_dn6)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)) * locals.var_q_temp2__blk815) + (assign38930_e44106 * locals.var_q_temp2__blk815_dn6)), (((((locals.var_q_d1_qsq__blk826_dn7 * locals.var_q_qsq__blk825) - (locals.var_q_d1_qsq__blk826 * locals.var_q_qsq__blk825_dn7)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)) * locals.var_q_temp2__blk815) + (assign38930_e44106 * locals.var_q_temp2__blk815_dn7)), (((((locals.var_q_d1_qsq__blk826_dn8 * locals.var_q_qsq__blk825) - (locals.var_q_d1_qsq__blk826 * locals.var_q_qsq__blk825_dn8)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)) * locals.var_q_temp2__blk815) + (assign38930_e44106 * locals.var_q_temp2__blk815_dn8)), (((((locals.var_q_d1_qsq__blk826_dn9 * locals.var_q_qsq__blk825) - (locals.var_q_d1_qsq__blk826 * locals.var_q_qsq__blk825_dn9)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)) * locals.var_q_temp2__blk815) + (assign38930_e44106 * locals.var_q_temp2__blk815_dn9)), );
            locals.var_q_d1_ln__blk835_rv = 0.0;
        }
    }
    pub(super) fn stamp_reactive_block_56(
        locals: &mut StampLocals,
    ) {
        if (((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1199 == 0.0)) && (locals.var_guard1200 != 0.0)) {
            let assign38940_e44123: f64 = (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815);
            let assign38940_e44128: f64 = (0.5 * locals.var_q_d1_qcoth__blk830);
            let assign38940_e44129: f64 = (locals.var_q_d1_ln__blk835 + assign38940_e44128);
            let assign38940_e44130: f64 = (locals.var_q_d1_qsq__blk826 * assign38940_e44129);
            let assign38940_e44131: f64 = (assign38940_e44123 - assign38940_e44130);
            let assign38940_e44133: f64 = (assign38940_e44131 / locals.var_q_qsq__blk825);
            (locals.var_q_d2_ln__blk836, locals.var_q_d2_ln__blk836_dn4, locals.var_q_d2_ln__blk836_dn6, locals.var_q_d2_ln__blk836_dn7, locals.var_q_d2_ln__blk836_dn8, locals.var_q_d2_ln__blk836_dn9, ) = (assign38940_e44133, ((((((locals.var_q_d2_qsq__blk827_dn4 * locals.var_q_temp2__blk815) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815_dn4)) - ((locals.var_q_d1_qsq__blk826_dn4 * assign38940_e44129) + (locals.var_q_d1_qsq__blk826 * (locals.var_q_d1_ln__blk835_dn4 + (0.5 * locals.var_q_d1_qcoth__blk830_dn4))))) * locals.var_q_qsq__blk825) - (assign38940_e44131 * locals.var_q_qsq__blk825_dn4)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((((locals.var_q_d2_qsq__blk827_dn6 * locals.var_q_temp2__blk815) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815_dn6)) - ((locals.var_q_d1_qsq__blk826_dn6 * assign38940_e44129) + (locals.var_q_d1_qsq__blk826 * (locals.var_q_d1_ln__blk835_dn6 + (0.5 * locals.var_q_d1_qcoth__blk830_dn6))))) * locals.var_q_qsq__blk825) - (assign38940_e44131 * locals.var_q_qsq__blk825_dn6)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((((locals.var_q_d2_qsq__blk827_dn7 * locals.var_q_temp2__blk815) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815_dn7)) - ((locals.var_q_d1_qsq__blk826_dn7 * assign38940_e44129) + (locals.var_q_d1_qsq__blk826 * (locals.var_q_d1_ln__blk835_dn7 + (0.5 * locals.var_q_d1_qcoth__blk830_dn7))))) * locals.var_q_qsq__blk825) - (assign38940_e44131 * locals.var_q_qsq__blk825_dn7)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((((locals.var_q_d2_qsq__blk827_dn8 * locals.var_q_temp2__blk815) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815_dn8)) - ((locals.var_q_d1_qsq__blk826_dn8 * assign38940_e44129) + (locals.var_q_d1_qsq__blk826 * (locals.var_q_d1_ln__blk835_dn8 + (0.5 * locals.var_q_d1_qcoth__blk830_dn8))))) * locals.var_q_qsq__blk825) - (assign38940_e44131 * locals.var_q_qsq__blk825_dn8)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((((locals.var_q_d2_qsq__blk827_dn9 * locals.var_q_temp2__blk815) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815_dn9)) - ((locals.var_q_d1_qsq__blk826_dn9 * assign38940_e44129) + (locals.var_q_d1_qsq__blk826 * (locals.var_q_d1_ln__blk835_dn9 + (0.5 * locals.var_q_d1_qcoth__blk830_dn9))))) * locals.var_q_qsq__blk825) - (assign38940_e44131 * locals.var_q_qsq__blk825_dn9)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), );
            locals.var_q_d2_ln__blk836_rv = 0.0;
        }
        if (((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1199 == 0.0)) && (locals.var_guard1200 == 0.0)) {
            let assign38950_e44151: f64 = (locals.var_q_qsq__blk825 * 0.0166666666667);
            let assign38950_e44155: f64 = (locals.var_q_qsq__blk825 * 0.0238095238095);
            let assign38950_e44159: f64 = (locals.var_q_qsq__blk825 * 0.025);
            let assign38950_e44160: f64 = (1.0 - assign38950_e44159);
            let assign38950_e44161: f64 = (assign38950_e44155 * assign38950_e44160);
            let assign38950_e44162: f64 = (1.0 - assign38950_e44161);
            let assign38950_e44163: f64 = (assign38950_e44151 * assign38950_e44162);
            let assign38950_e44164: f64 = (1.0 - assign38950_e44163);
            let assign38950_e44165: f64 = (0.1666666666667 * assign38950_e44164);
            (locals.var_q_temp3__blk816, locals.var_q_temp3__blk816_dn4, locals.var_q_temp3__blk816_dn6, locals.var_q_temp3__blk816_dn7, locals.var_q_temp3__blk816_dn8, locals.var_q_temp3__blk816_dn9, ) = (assign38950_e44165, (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn4 * 0.0166666666667) * assign38950_e44162) + (assign38950_e44151 * (-(((locals.var_q_qsq__blk825_dn4 * 0.0238095238095) * assign38950_e44160) + (assign38950_e44155 * (-(locals.var_q_qsq__blk825_dn4 * 0.025))))))))), (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn6 * 0.0166666666667) * assign38950_e44162) + (assign38950_e44151 * (-(((locals.var_q_qsq__blk825_dn6 * 0.0238095238095) * assign38950_e44160) + (assign38950_e44155 * (-(locals.var_q_qsq__blk825_dn6 * 0.025))))))))), (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn7 * 0.0166666666667) * assign38950_e44162) + (assign38950_e44151 * (-(((locals.var_q_qsq__blk825_dn7 * 0.0238095238095) * assign38950_e44160) + (assign38950_e44155 * (-(locals.var_q_qsq__blk825_dn7 * 0.025))))))))), (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn8 * 0.0166666666667) * assign38950_e44162) + (assign38950_e44151 * (-(((locals.var_q_qsq__blk825_dn8 * 0.0238095238095) * assign38950_e44160) + (assign38950_e44155 * (-(locals.var_q_qsq__blk825_dn8 * 0.025))))))))), (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn9 * 0.0166666666667) * assign38950_e44162) + (assign38950_e44151 * (-(((locals.var_q_qsq__blk825_dn9 * 0.0238095238095) * assign38950_e44160) + (assign38950_e44155 * (-(locals.var_q_qsq__blk825_dn9 * 0.025))))))))), );
            locals.var_q_temp3__blk816_rv = 0.0;
        }
        if (((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1199 == 0.0)) && (locals.var_guard1200 == 0.0)) {
            let assign38960_e44182: f64 = (locals.var_q_qsq__blk825 * locals.var_q_temp3__blk816);
            let assign38960_e44183: f64 = (2.0 + assign38960_e44182);
            (locals.var_q_qcoth__blk829, locals.var_q_qcoth__blk829_dn4, locals.var_q_qcoth__blk829_dn6, locals.var_q_qcoth__blk829_dn7, locals.var_q_qcoth__blk829_dn8, locals.var_q_qcoth__blk829_dn9, ) = (assign38960_e44183, ((locals.var_q_qsq__blk825_dn4 * locals.var_q_temp3__blk816) + (locals.var_q_qsq__blk825 * locals.var_q_temp3__blk816_dn4)), ((locals.var_q_qsq__blk825_dn6 * locals.var_q_temp3__blk816) + (locals.var_q_qsq__blk825 * locals.var_q_temp3__blk816_dn6)), ((locals.var_q_qsq__blk825_dn7 * locals.var_q_temp3__blk816) + (locals.var_q_qsq__blk825 * locals.var_q_temp3__blk816_dn7)), ((locals.var_q_qsq__blk825_dn8 * locals.var_q_temp3__blk816) + (locals.var_q_qsq__blk825 * locals.var_q_temp3__blk816_dn8)), ((locals.var_q_qsq__blk825_dn9 * locals.var_q_temp3__blk816) + (locals.var_q_qsq__blk825 * locals.var_q_temp3__blk816_dn9)), );
            locals.var_q_qcoth__blk829_rv = 0.0;
        }
        if (((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1199 == 0.0)) && (locals.var_guard1200 == 0.0)) {
            let assign38970_e44201: f64 = (locals.var_q_qsq__blk825 * 0.0333333333333);
            let assign38970_e44205: f64 = (locals.var_q_qsq__blk825 * 0.0357142857143);
            let assign38970_e44209: f64 = (locals.var_q_qsq__blk825 * 0.0333333333333);
            let assign38970_e44210: f64 = (1.0 - assign38970_e44209);
            let assign38970_e44211: f64 = (assign38970_e44205 * assign38970_e44210);
            let assign38970_e44212: f64 = (1.0 - assign38970_e44211);
            let assign38970_e44213: f64 = (assign38970_e44201 * assign38970_e44212);
            let assign38970_e44214: f64 = (1.0 - assign38970_e44213);
            let assign38970_e44215: f64 = (0.1666666666667 * assign38970_e44214);
            (locals.var_q_temp1__blk814, locals.var_q_temp1__blk814_dn4, locals.var_q_temp1__blk814_dn6, locals.var_q_temp1__blk814_dn7, locals.var_q_temp1__blk814_dn8, locals.var_q_temp1__blk814_dn9, ) = (assign38970_e44215, (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn4 * 0.0333333333333) * assign38970_e44212) + (assign38970_e44201 * (-(((locals.var_q_qsq__blk825_dn4 * 0.0357142857143) * assign38970_e44210) + (assign38970_e44205 * (-(locals.var_q_qsq__blk825_dn4 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn6 * 0.0333333333333) * assign38970_e44212) + (assign38970_e44201 * (-(((locals.var_q_qsq__blk825_dn6 * 0.0357142857143) * assign38970_e44210) + (assign38970_e44205 * (-(locals.var_q_qsq__blk825_dn6 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn7 * 0.0333333333333) * assign38970_e44212) + (assign38970_e44201 * (-(((locals.var_q_qsq__blk825_dn7 * 0.0357142857143) * assign38970_e44210) + (assign38970_e44205 * (-(locals.var_q_qsq__blk825_dn7 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn8 * 0.0333333333333) * assign38970_e44212) + (assign38970_e44201 * (-(((locals.var_q_qsq__blk825_dn8 * 0.0357142857143) * assign38970_e44210) + (assign38970_e44205 * (-(locals.var_q_qsq__blk825_dn8 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn9 * 0.0333333333333) * assign38970_e44212) + (assign38970_e44201 * (-(((locals.var_q_qsq__blk825_dn9 * 0.0357142857143) * assign38970_e44210) + (assign38970_e44205 * (-(locals.var_q_qsq__blk825_dn9 * 0.0333333333333))))))))), );
            locals.var_q_temp1__blk814_rv = 0.0;
        }
        if (((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1199 == 0.0)) && (locals.var_guard1200 == 0.0)) {
            let assign38980_e44231: f64 = (locals.var_q_d1_qsq__blk826 * locals.var_q_temp1__blk814);
            (locals.var_q_d1_qcoth__blk830, locals.var_q_d1_qcoth__blk830_dn4, locals.var_q_d1_qcoth__blk830_dn6, locals.var_q_d1_qcoth__blk830_dn7, locals.var_q_d1_qcoth__blk830_dn8, locals.var_q_d1_qcoth__blk830_dn9, ) = (assign38980_e44231, ((locals.var_q_d1_qsq__blk826_dn4 * locals.var_q_temp1__blk814) + (locals.var_q_d1_qsq__blk826 * locals.var_q_temp1__blk814_dn4)), ((locals.var_q_d1_qsq__blk826_dn6 * locals.var_q_temp1__blk814) + (locals.var_q_d1_qsq__blk826 * locals.var_q_temp1__blk814_dn6)), ((locals.var_q_d1_qsq__blk826_dn7 * locals.var_q_temp1__blk814) + (locals.var_q_d1_qsq__blk826 * locals.var_q_temp1__blk814_dn7)), ((locals.var_q_d1_qsq__blk826_dn8 * locals.var_q_temp1__blk814) + (locals.var_q_d1_qsq__blk826 * locals.var_q_temp1__blk814_dn8)), ((locals.var_q_d1_qsq__blk826_dn9 * locals.var_q_temp1__blk814) + (locals.var_q_d1_qsq__blk826 * locals.var_q_temp1__blk814_dn9)), );
            locals.var_q_d1_qcoth__blk830_rv = 0.0;
        }
        if (((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1199 == 0.0)) && (locals.var_guard1200 == 0.0)) {
            let assign38990_e44249: f64 = (locals.var_q_qsq__blk825 * 0.0714285714286);
            let assign38990_e44253: f64 = (0.05 * locals.var_q_qsq__blk825);
            let assign38990_e44257: f64 = (0.0420875420875421 * locals.var_q_qsq__blk825);
            let assign38990_e44258: f64 = (1.0 - assign38990_e44257);
            let assign38990_e44259: f64 = (assign38990_e44253 * assign38990_e44258);
            let assign38990_e44260: f64 = (1.0 - assign38990_e44259);
            let assign38990_e44261: f64 = (assign38990_e44249 * assign38990_e44260);
            let assign38990_e44262: f64 = (1.0 - assign38990_e44261);
            let assign38990_e44263: f64 = (0.0055555555556 * assign38990_e44262);
            (locals.var_q_temp2__blk815, locals.var_q_temp2__blk815_dn4, locals.var_q_temp2__blk815_dn6, locals.var_q_temp2__blk815_dn7, locals.var_q_temp2__blk815_dn8, locals.var_q_temp2__blk815_dn9, ) = (assign38990_e44263, (0.0055555555556 * (-(((locals.var_q_qsq__blk825_dn4 * 0.0714285714286) * assign38990_e44260) + (assign38990_e44249 * (-(((0.05 * locals.var_q_qsq__blk825_dn4) * assign38990_e44258) + (assign38990_e44253 * (-(0.0420875420875421 * locals.var_q_qsq__blk825_dn4))))))))), (0.0055555555556 * (-(((locals.var_q_qsq__blk825_dn6 * 0.0714285714286) * assign38990_e44260) + (assign38990_e44249 * (-(((0.05 * locals.var_q_qsq__blk825_dn6) * assign38990_e44258) + (assign38990_e44253 * (-(0.0420875420875421 * locals.var_q_qsq__blk825_dn6))))))))), (0.0055555555556 * (-(((locals.var_q_qsq__blk825_dn7 * 0.0714285714286) * assign38990_e44260) + (assign38990_e44249 * (-(((0.05 * locals.var_q_qsq__blk825_dn7) * assign38990_e44258) + (assign38990_e44253 * (-(0.0420875420875421 * locals.var_q_qsq__blk825_dn7))))))))), (0.0055555555556 * (-(((locals.var_q_qsq__blk825_dn8 * 0.0714285714286) * assign38990_e44260) + (assign38990_e44249 * (-(((0.05 * locals.var_q_qsq__blk825_dn8) * assign38990_e44258) + (assign38990_e44253 * (-(0.0420875420875421 * locals.var_q_qsq__blk825_dn8))))))))), (0.0055555555556 * (-(((locals.var_q_qsq__blk825_dn9 * 0.0714285714286) * assign38990_e44260) + (assign38990_e44249 * (-(((0.05 * locals.var_q_qsq__blk825_dn9) * assign38990_e44258) + (assign38990_e44253 * (-(0.0420875420875421 * locals.var_q_qsq__blk825_dn9))))))))), );
            locals.var_q_temp2__blk815_rv = 0.0;
        }
        if (((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1199 == 0.0)) && (locals.var_guard1200 == 0.0)) {
            let assign39000_e44279: f64 = (locals.var_q_d2_qsq__blk827 * locals.var_q_temp1__blk814);
            let assign39000_e44282: f64 = (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826);
            let assign39000_e44284: f64 = (assign39000_e44282 * locals.var_q_temp2__blk815);
            let assign39000_e44285: f64 = (assign39000_e44279 - assign39000_e44284);
            (locals.var_q_d2_qcoth__blk832, locals.var_q_d2_qcoth__blk832_dn4, locals.var_q_d2_qcoth__blk832_dn6, locals.var_q_d2_qcoth__blk832_dn7, locals.var_q_d2_qcoth__blk832_dn8, locals.var_q_d2_qcoth__blk832_dn9, ) = (assign39000_e44285, (((locals.var_q_d2_qsq__blk827_dn4 * locals.var_q_temp1__blk814) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp1__blk814_dn4)) - ((((locals.var_q_d1_qsq__blk826_dn4 * locals.var_q_d1_qsq__blk826) + (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826_dn4)) * locals.var_q_temp2__blk815) + (assign39000_e44282 * locals.var_q_temp2__blk815_dn4))), (((locals.var_q_d2_qsq__blk827_dn6 * locals.var_q_temp1__blk814) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp1__blk814_dn6)) - ((((locals.var_q_d1_qsq__blk826_dn6 * locals.var_q_d1_qsq__blk826) + (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826_dn6)) * locals.var_q_temp2__blk815) + (assign39000_e44282 * locals.var_q_temp2__blk815_dn6))), (((locals.var_q_d2_qsq__blk827_dn7 * locals.var_q_temp1__blk814) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp1__blk814_dn7)) - ((((locals.var_q_d1_qsq__blk826_dn7 * locals.var_q_d1_qsq__blk826) + (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826_dn7)) * locals.var_q_temp2__blk815) + (assign39000_e44282 * locals.var_q_temp2__blk815_dn7))), (((locals.var_q_d2_qsq__blk827_dn8 * locals.var_q_temp1__blk814) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp1__blk814_dn8)) - ((((locals.var_q_d1_qsq__blk826_dn8 * locals.var_q_d1_qsq__blk826) + (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826_dn8)) * locals.var_q_temp2__blk815) + (assign39000_e44282 * locals.var_q_temp2__blk815_dn8))), (((locals.var_q_d2_qsq__blk827_dn9 * locals.var_q_temp1__blk814) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp1__blk814_dn9)) - ((((locals.var_q_d1_qsq__blk826_dn9 * locals.var_q_d1_qsq__blk826) + (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826_dn9)) * locals.var_q_temp2__blk815) + (assign39000_e44282 * locals.var_q_temp2__blk815_dn9))), );
            locals.var_q_d2_qcoth__blk832_rv = 0.0;
        }
        if (((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1199 == 0.0)) && (locals.var_guard1200 == 0.0)) {
            let assign39010_e44300: f64 = (-0.5);
            let assign39010_e44302: f64 = (assign39010_e44300 * locals.var_q_d1_qsq__blk826);
            let assign39010_e44304: f64 = (assign39010_e44302 * locals.var_q_temp3__blk816);
            (locals.var_q_d1_ln__blk835, locals.var_q_d1_ln__blk835_dn4, locals.var_q_d1_ln__blk835_dn6, locals.var_q_d1_ln__blk835_dn7, locals.var_q_d1_ln__blk835_dn8, locals.var_q_d1_ln__blk835_dn9, ) = (assign39010_e44304, (((assign39010_e44300 * locals.var_q_d1_qsq__blk826_dn4) * locals.var_q_temp3__blk816) + (assign39010_e44302 * locals.var_q_temp3__blk816_dn4)), (((assign39010_e44300 * locals.var_q_d1_qsq__blk826_dn6) * locals.var_q_temp3__blk816) + (assign39010_e44302 * locals.var_q_temp3__blk816_dn6)), (((assign39010_e44300 * locals.var_q_d1_qsq__blk826_dn7) * locals.var_q_temp3__blk816) + (assign39010_e44302 * locals.var_q_temp3__blk816_dn7)), (((assign39010_e44300 * locals.var_q_d1_qsq__blk826_dn8) * locals.var_q_temp3__blk816) + (assign39010_e44302 * locals.var_q_temp3__blk816_dn8)), (((assign39010_e44300 * locals.var_q_d1_qsq__blk826_dn9) * locals.var_q_temp3__blk816) + (assign39010_e44302 * locals.var_q_temp3__blk816_dn9)), );
            locals.var_q_d1_ln__blk835_rv = 0.0;
        }
        if (((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1199 == 0.0)) && (locals.var_guard1200 == 0.0)) {
            let assign39020_e44319: f64 = (-0.5);
            let assign39020_e44321: f64 = (assign39020_e44319 * locals.var_q_d2_qsq__blk827);
            let assign39020_e44323: f64 = (assign39020_e44321 * locals.var_q_temp3__blk816);
            let assign39020_e44326: f64 = (0.25 * 0.0055555555556);
            let assign39020_e44328: f64 = (assign39020_e44326 * locals.var_q_d1_qsq__blk826);
            let assign39020_e44330: f64 = (assign39020_e44328 * locals.var_q_d1_qsq__blk826);
            let assign39020_e44334: f64 = (locals.var_q_qsq__blk825 * 0.0238095238095);
            let assign39020_e44338: f64 = (0.075 * locals.var_q_qsq__blk825);
            let assign39020_e44339: f64 = (2.0 - assign39020_e44338);
            let assign39020_e44340: f64 = (assign39020_e44334 * assign39020_e44339);
            let assign39020_e44341: f64 = (1.0 - assign39020_e44340);
            let assign39020_e44342: f64 = (assign39020_e44330 * assign39020_e44341);
            let assign39020_e44343: f64 = (assign39020_e44323 + assign39020_e44342);
            (locals.var_q_d2_ln__blk836, locals.var_q_d2_ln__blk836_dn4, locals.var_q_d2_ln__blk836_dn6, locals.var_q_d2_ln__blk836_dn7, locals.var_q_d2_ln__blk836_dn8, locals.var_q_d2_ln__blk836_dn9, ) = (assign39020_e44343, ((((assign39020_e44319 * locals.var_q_d2_qsq__blk827_dn4) * locals.var_q_temp3__blk816) + (assign39020_e44321 * locals.var_q_temp3__blk816_dn4)) + (((((assign39020_e44326 * locals.var_q_d1_qsq__blk826_dn4) * locals.var_q_d1_qsq__blk826) + (assign39020_e44328 * locals.var_q_d1_qsq__blk826_dn4)) * assign39020_e44341) + (assign39020_e44330 * (-(((locals.var_q_qsq__blk825_dn4 * 0.0238095238095) * assign39020_e44339) + (assign39020_e44334 * (-(0.075 * locals.var_q_qsq__blk825_dn4)))))))), ((((assign39020_e44319 * locals.var_q_d2_qsq__blk827_dn6) * locals.var_q_temp3__blk816) + (assign39020_e44321 * locals.var_q_temp3__blk816_dn6)) + (((((assign39020_e44326 * locals.var_q_d1_qsq__blk826_dn6) * locals.var_q_d1_qsq__blk826) + (assign39020_e44328 * locals.var_q_d1_qsq__blk826_dn6)) * assign39020_e44341) + (assign39020_e44330 * (-(((locals.var_q_qsq__blk825_dn6 * 0.0238095238095) * assign39020_e44339) + (assign39020_e44334 * (-(0.075 * locals.var_q_qsq__blk825_dn6)))))))), ((((assign39020_e44319 * locals.var_q_d2_qsq__blk827_dn7) * locals.var_q_temp3__blk816) + (assign39020_e44321 * locals.var_q_temp3__blk816_dn7)) + (((((assign39020_e44326 * locals.var_q_d1_qsq__blk826_dn7) * locals.var_q_d1_qsq__blk826) + (assign39020_e44328 * locals.var_q_d1_qsq__blk826_dn7)) * assign39020_e44341) + (assign39020_e44330 * (-(((locals.var_q_qsq__blk825_dn7 * 0.0238095238095) * assign39020_e44339) + (assign39020_e44334 * (-(0.075 * locals.var_q_qsq__blk825_dn7)))))))), ((((assign39020_e44319 * locals.var_q_d2_qsq__blk827_dn8) * locals.var_q_temp3__blk816) + (assign39020_e44321 * locals.var_q_temp3__blk816_dn8)) + (((((assign39020_e44326 * locals.var_q_d1_qsq__blk826_dn8) * locals.var_q_d1_qsq__blk826) + (assign39020_e44328 * locals.var_q_d1_qsq__blk826_dn8)) * assign39020_e44341) + (assign39020_e44330 * (-(((locals.var_q_qsq__blk825_dn8 * 0.0238095238095) * assign39020_e44339) + (assign39020_e44334 * (-(0.075 * locals.var_q_qsq__blk825_dn8)))))))), ((((assign39020_e44319 * locals.var_q_d2_qsq__blk827_dn9) * locals.var_q_temp3__blk816) + (assign39020_e44321 * locals.var_q_temp3__blk816_dn9)) + (((((assign39020_e44326 * locals.var_q_d1_qsq__blk826_dn9) * locals.var_q_d1_qsq__blk826) + (assign39020_e44328 * locals.var_q_d1_qsq__blk826_dn9)) * assign39020_e44341) + (assign39020_e44330 * (-(((locals.var_q_qsq__blk825_dn9 * 0.0238095238095) * assign39020_e44339) + (assign39020_e44334 * (-(0.075 * locals.var_q_qsq__blk825_dn9)))))))), );
            locals.var_q_d2_ln__blk836_rv = 0.0;
        }
        let assign39030_e44348: f64 = if locals.var_q_qsq__blk825 > 0.005 { 1.0 } else { 0.0 };
        locals.var_guard1201 = assign39030_e44348;
        locals.var_guard1201_rv = 0.0;
        if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1201 != 0.0)) {
            let assign39040_e44358: f64 = (4.0 * locals.var_q_qsq__blk825);
            let assign39040_e44363: f64 = (2.0 - locals.var_q_invexpq__blk831);
            let assign39040_e44364: f64 = (locals.var_q_invexpq__blk831 * assign39040_e44363);
            let assign39040_e44365: f64 = (1.0 - assign39040_e44364);
            let assign39040_e44366: f64 = (assign39040_e44358 / assign39040_e44365);
            (locals.var_q_temp2__blk815, locals.var_q_temp2__blk815_dn4, locals.var_q_temp2__blk815_dn6, locals.var_q_temp2__blk815_dn7, locals.var_q_temp2__blk815_dn8, locals.var_q_temp2__blk815_dn9, ) = (assign39040_e44366, ((((4.0 * locals.var_q_qsq__blk825_dn4) * assign39040_e44365) - (assign39040_e44358 * (-((locals.var_q_invexpq__blk831_dn4 * assign39040_e44363) + (locals.var_q_invexpq__blk831 * (-locals.var_q_invexpq__blk831_dn4)))))) / (assign39040_e44365 * assign39040_e44365)), ((((4.0 * locals.var_q_qsq__blk825_dn6) * assign39040_e44365) - (assign39040_e44358 * (-((locals.var_q_invexpq__blk831_dn6 * assign39040_e44363) + (locals.var_q_invexpq__blk831 * (-locals.var_q_invexpq__blk831_dn6)))))) / (assign39040_e44365 * assign39040_e44365)), ((((4.0 * locals.var_q_qsq__blk825_dn7) * assign39040_e44365) - (assign39040_e44358 * (-((locals.var_q_invexpq__blk831_dn7 * assign39040_e44363) + (locals.var_q_invexpq__blk831 * (-locals.var_q_invexpq__blk831_dn7)))))) / (assign39040_e44365 * assign39040_e44365)), ((((4.0 * locals.var_q_qsq__blk825_dn8) * assign39040_e44365) - (assign39040_e44358 * (-((locals.var_q_invexpq__blk831_dn8 * assign39040_e44363) + (locals.var_q_invexpq__blk831 * (-locals.var_q_invexpq__blk831_dn8)))))) / (assign39040_e44365 * assign39040_e44365)), ((((4.0 * locals.var_q_qsq__blk825_dn9) * assign39040_e44365) - (assign39040_e44358 * (-((locals.var_q_invexpq__blk831_dn9 * assign39040_e44363) + (locals.var_q_invexpq__blk831 * (-locals.var_q_invexpq__blk831_dn9)))))) / (assign39040_e44365 * assign39040_e44365)), );
            locals.var_q_temp2__blk815_rv = 0.0;
        }
        if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1201 != 0.0)) {
            let assign39050_e44378: f64 = (locals.var_q_temp2__blk815 * locals.var_q_invexpq__blk831);
            (locals.var_q_sh_term__blk833, locals.var_q_sh_term__blk833_dn4, locals.var_q_sh_term__blk833_dn6, locals.var_q_sh_term__blk833_dn7, locals.var_q_sh_term__blk833_dn8, locals.var_q_sh_term__blk833_dn9, ) = (assign39050_e44378, ((locals.var_q_temp2__blk815_dn4 * locals.var_q_invexpq__blk831) + (locals.var_q_temp2__blk815 * locals.var_q_invexpq__blk831_dn4)), ((locals.var_q_temp2__blk815_dn6 * locals.var_q_invexpq__blk831) + (locals.var_q_temp2__blk815 * locals.var_q_invexpq__blk831_dn6)), ((locals.var_q_temp2__blk815_dn7 * locals.var_q_invexpq__blk831) + (locals.var_q_temp2__blk815 * locals.var_q_invexpq__blk831_dn7)), ((locals.var_q_temp2__blk815_dn8 * locals.var_q_invexpq__blk831) + (locals.var_q_temp2__blk815 * locals.var_q_invexpq__blk831_dn8)), ((locals.var_q_temp2__blk815_dn9 * locals.var_q_invexpq__blk831) + (locals.var_q_temp2__blk815 * locals.var_q_invexpq__blk831_dn9)), );
            locals.var_q_sh_term__blk833_rv = 0.0;
        }
        if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1201 != 0.0)) {
            let assign39060_e44389: f64 = (locals.var_q_temp2__blk815).ln();
            let assign39060_e44391: f64 = (assign39060_e44389 - locals.var_q_rac_qsq__blk828);
            (locals.var_q_ln_term__blk834, locals.var_q_ln_term__blk834_dn4, locals.var_q_ln_term__blk834_dn6, locals.var_q_ln_term__blk834_dn7, locals.var_q_ln_term__blk834_dn8, locals.var_q_ln_term__blk834_dn9, ) = (assign39060_e44391, ((locals.var_q_temp2__blk815_dn4 / locals.var_q_temp2__blk815) - locals.var_q_rac_qsq__blk828_dn4), ((locals.var_q_temp2__blk815_dn6 / locals.var_q_temp2__blk815) - locals.var_q_rac_qsq__blk828_dn6), ((locals.var_q_temp2__blk815_dn7 / locals.var_q_temp2__blk815) - locals.var_q_rac_qsq__blk828_dn7), ((locals.var_q_temp2__blk815_dn8 / locals.var_q_temp2__blk815) - locals.var_q_rac_qsq__blk828_dn8), ((locals.var_q_temp2__blk815_dn9 / locals.var_q_temp2__blk815) - locals.var_q_rac_qsq__blk828_dn9), );
            locals.var_q_ln_term__blk834_rv = 0.0;
        }
        let assign39070_e44396: f64 = (-0.005);
        let assign39070_e44397: f64 = if locals.var_q_qsq__blk825 < assign39070_e44396 { 1.0 } else { 0.0 };
        locals.var_guard1202 = assign39070_e44397;
        locals.var_guard1202_rv = 0.0;
        if (((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1201 == 0.0)) && (locals.var_guard1202 != 0.0)) {
            let assign39080_e44410: f64 = (0.5 * locals.var_q_rac_qsq__blk828);
            let assign39080_e44411: f64 = (assign39080_e44410).sin();
            (locals.var_q_temp2__blk815, locals.var_q_temp2__blk815_dn4, locals.var_q_temp2__blk815_dn6, locals.var_q_temp2__blk815_dn7, locals.var_q_temp2__blk815_dn8, locals.var_q_temp2__blk815_dn9, ) = (assign39080_e44411, ((assign39080_e44410).cos() * (0.5 * locals.var_q_rac_qsq__blk828_dn4)), ((assign39080_e44410).cos() * (0.5 * locals.var_q_rac_qsq__blk828_dn6)), ((assign39080_e44410).cos() * (0.5 * locals.var_q_rac_qsq__blk828_dn7)), ((assign39080_e44410).cos() * (0.5 * locals.var_q_rac_qsq__blk828_dn8)), ((assign39080_e44410).cos() * (0.5 * locals.var_q_rac_qsq__blk828_dn9)), );
            locals.var_q_temp2__blk815_rv = 0.0;
        }
        if (((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1201 == 0.0)) && (locals.var_guard1202 != 0.0)) {
            let assign39090_e44425: f64 = (-locals.var_q_qsq__blk825);
            let assign39090_e44428: f64 = (locals.var_q_temp2__blk815 * locals.var_q_temp2__blk815);
            let assign39090_e44429: f64 = (assign39090_e44425 / assign39090_e44428);
            (locals.var_q_sh_term__blk833, locals.var_q_sh_term__blk833_dn4, locals.var_q_sh_term__blk833_dn6, locals.var_q_sh_term__blk833_dn7, locals.var_q_sh_term__blk833_dn8, locals.var_q_sh_term__blk833_dn9, ) = (assign39090_e44429, ((((-locals.var_q_qsq__blk825_dn4) * assign39090_e44428) - (assign39090_e44425 * ((locals.var_q_temp2__blk815_dn4 * locals.var_q_temp2__blk815) + (locals.var_q_temp2__blk815 * locals.var_q_temp2__blk815_dn4)))) / (assign39090_e44428 * assign39090_e44428)), ((((-locals.var_q_qsq__blk825_dn6) * assign39090_e44428) - (assign39090_e44425 * ((locals.var_q_temp2__blk815_dn6 * locals.var_q_temp2__blk815) + (locals.var_q_temp2__blk815 * locals.var_q_temp2__blk815_dn6)))) / (assign39090_e44428 * assign39090_e44428)), ((((-locals.var_q_qsq__blk825_dn7) * assign39090_e44428) - (assign39090_e44425 * ((locals.var_q_temp2__blk815_dn7 * locals.var_q_temp2__blk815) + (locals.var_q_temp2__blk815 * locals.var_q_temp2__blk815_dn7)))) / (assign39090_e44428 * assign39090_e44428)), ((((-locals.var_q_qsq__blk825_dn8) * assign39090_e44428) - (assign39090_e44425 * ((locals.var_q_temp2__blk815_dn8 * locals.var_q_temp2__blk815) + (locals.var_q_temp2__blk815 * locals.var_q_temp2__blk815_dn8)))) / (assign39090_e44428 * assign39090_e44428)), ((((-locals.var_q_qsq__blk825_dn9) * assign39090_e44428) - (assign39090_e44425 * ((locals.var_q_temp2__blk815_dn9 * locals.var_q_temp2__blk815) + (locals.var_q_temp2__blk815 * locals.var_q_temp2__blk815_dn9)))) / (assign39090_e44428 * assign39090_e44428)), );
            locals.var_q_sh_term__blk833_rv = 0.0;
        }
        if (((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1201 == 0.0)) && (locals.var_guard1202 != 0.0)) {
            let assign39100_e44443: f64 = (locals.var_q_sh_term__blk833).ln();
            (locals.var_q_ln_term__blk834, locals.var_q_ln_term__blk834_dn4, locals.var_q_ln_term__blk834_dn6, locals.var_q_ln_term__blk834_dn7, locals.var_q_ln_term__blk834_dn8, locals.var_q_ln_term__blk834_dn9, ) = (assign39100_e44443, (locals.var_q_sh_term__blk833_dn4 / locals.var_q_sh_term__blk833), (locals.var_q_sh_term__blk833_dn6 / locals.var_q_sh_term__blk833), (locals.var_q_sh_term__blk833_dn7 / locals.var_q_sh_term__blk833), (locals.var_q_sh_term__blk833_dn8 / locals.var_q_sh_term__blk833), (locals.var_q_sh_term__blk833_dn9 / locals.var_q_sh_term__blk833), );
            locals.var_q_ln_term__blk834_rv = 0.0;
        }
        if (((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1201 == 0.0)) && (locals.var_guard1202 == 0.0)) {
            let assign39110_e44460: f64 = (locals.var_q_qsq__blk825 * 0.3333333333333);
            let assign39110_e44464: f64 = (0.05 * locals.var_q_qsq__blk825);
            let assign39110_e44468: f64 = (0.0396825396825397 * locals.var_q_qsq__blk825);
            let assign39110_e44469: f64 = (1.0 - assign39110_e44468);
            let assign39110_e44470: f64 = (assign39110_e44464 * assign39110_e44469);
            let assign39110_e44471: f64 = (1.0 - assign39110_e44470);
            let assign39110_e44472: f64 = (assign39110_e44460 * assign39110_e44471);
            let assign39110_e44473: f64 = (4.0 - assign39110_e44472);
            (locals.var_q_sh_term__blk833, locals.var_q_sh_term__blk833_dn4, locals.var_q_sh_term__blk833_dn6, locals.var_q_sh_term__blk833_dn7, locals.var_q_sh_term__blk833_dn8, locals.var_q_sh_term__blk833_dn9, ) = (assign39110_e44473, (-(((locals.var_q_qsq__blk825_dn4 * 0.3333333333333) * assign39110_e44471) + (assign39110_e44460 * (-(((0.05 * locals.var_q_qsq__blk825_dn4) * assign39110_e44469) + (assign39110_e44464 * (-(0.0396825396825397 * locals.var_q_qsq__blk825_dn4)))))))), (-(((locals.var_q_qsq__blk825_dn6 * 0.3333333333333) * assign39110_e44471) + (assign39110_e44460 * (-(((0.05 * locals.var_q_qsq__blk825_dn6) * assign39110_e44469) + (assign39110_e44464 * (-(0.0396825396825397 * locals.var_q_qsq__blk825_dn6)))))))), (-(((locals.var_q_qsq__blk825_dn7 * 0.3333333333333) * assign39110_e44471) + (assign39110_e44460 * (-(((0.05 * locals.var_q_qsq__blk825_dn7) * assign39110_e44469) + (assign39110_e44464 * (-(0.0396825396825397 * locals.var_q_qsq__blk825_dn7)))))))), (-(((locals.var_q_qsq__blk825_dn8 * 0.3333333333333) * assign39110_e44471) + (assign39110_e44460 * (-(((0.05 * locals.var_q_qsq__blk825_dn8) * assign39110_e44469) + (assign39110_e44464 * (-(0.0396825396825397 * locals.var_q_qsq__blk825_dn8)))))))), (-(((locals.var_q_qsq__blk825_dn9 * 0.3333333333333) * assign39110_e44471) + (assign39110_e44460 * (-(((0.05 * locals.var_q_qsq__blk825_dn9) * assign39110_e44469) + (assign39110_e44464 * (-(0.0396825396825397 * locals.var_q_qsq__blk825_dn9)))))))), );
            locals.var_q_sh_term__blk833_rv = 0.0;
        }
        if (((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1201 == 0.0)) && (locals.var_guard1202 == 0.0)) {
            let assign39120_e44488: f64 = (locals.var_q_sh_term__blk833).ln();
            (locals.var_q_ln_term__blk834, locals.var_q_ln_term__blk834_dn4, locals.var_q_ln_term__blk834_dn6, locals.var_q_ln_term__blk834_dn7, locals.var_q_ln_term__blk834_dn8, locals.var_q_ln_term__blk834_dn9, ) = (assign39120_e44488, (locals.var_q_sh_term__blk833_dn4 / locals.var_q_sh_term__blk833), (locals.var_q_sh_term__blk833_dn6 / locals.var_q_sh_term__blk833), (locals.var_q_sh_term__blk833_dn7 / locals.var_q_sh_term__blk833), (locals.var_q_sh_term__blk833_dn8 / locals.var_q_sh_term__blk833), (locals.var_q_sh_term__blk833_dn9 / locals.var_q_sh_term__blk833), );
            locals.var_q_ln_term__blk834_rv = 0.0;
        }
        let assign39130_e44493: f64 = (1.01 * locals.var_q_k1q1__blk823);
        let assign39130_e44495: f64 = (assign39130_e44493 + locals.var_q_qcoth__blk829);
        let assign39130_e44497: f64 = if assign39130_e44495 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1203 = assign39130_e44497;
        locals.var_guard1203_rv = 0.0;
        if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1203 != 0.0)) {
            let assign39140_e44507: f64 = (locals.var_q_k1q1__blk823 + locals.var_q_qcoth__blk829);
            (locals.var_q_expnum__blk837, locals.var_q_expnum__blk837_dn4, locals.var_q_expnum__blk837_dn6, locals.var_q_expnum__blk837_dn7, locals.var_q_expnum__blk837_dn8, locals.var_q_expnum__blk837_dn9, ) = (assign39140_e44507, (locals.var_q_k1q1__blk823_dn4 + locals.var_q_qcoth__blk829_dn4), (locals.var_q_k1q1__blk823_dn6 + locals.var_q_qcoth__blk829_dn6), (locals.var_q_k1q1__blk823_dn7 + locals.var_q_qcoth__blk829_dn7), (locals.var_q_k1q1__blk823_dn8 + locals.var_q_qcoth__blk829_dn8), (locals.var_q_k1q1__blk823_dn9 + locals.var_q_qcoth__blk829_dn9), );
            locals.var_q_expnum__blk837_rv = 0.0;
        }
        if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1203 != 0.0)) {
            let assign39150_e44519: f64 = (locals.var_k1__blk932 + locals.var_q_d1_qcoth__blk830);
            (locals.var_q_d1_expnum__blk838, locals.var_q_d1_expnum__blk838_dn4, locals.var_q_d1_expnum__blk838_dn6, locals.var_q_d1_expnum__blk838_dn7, locals.var_q_d1_expnum__blk838_dn8, locals.var_q_d1_expnum__blk838_dn9, ) = (assign39150_e44519, (locals.var_k1__blk932_dn4 + locals.var_q_d1_qcoth__blk830_dn4), (locals.var_k1__blk932_dn6 + locals.var_q_d1_qcoth__blk830_dn6), (locals.var_k1__blk932_dn7 + locals.var_q_d1_qcoth__blk830_dn7), (locals.var_k1__blk932_dn8 + locals.var_q_d1_qcoth__blk830_dn8), (locals.var_k1__blk932_dn9 + locals.var_q_d1_qcoth__blk830_dn9), );
            locals.var_q_d1_expnum__blk838_rv = 0.0;
        }
        if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1203 != 0.0)) {
            (locals.var_q_d2_expnum__blk839, locals.var_q_d2_expnum__blk839_dn4, locals.var_q_d2_expnum__blk839_dn6, locals.var_q_d2_expnum__blk839_dn7, locals.var_q_d2_expnum__blk839_dn8, locals.var_q_d2_expnum__blk839_dn9, ) = (locals.var_q_d2_qcoth__blk832, locals.var_q_d2_qcoth__blk832_dn4, locals.var_q_d2_qcoth__blk832_dn6, locals.var_q_d2_qcoth__blk832_dn7, locals.var_q_d2_qcoth__blk832_dn8, locals.var_q_d2_qcoth__blk832_dn9, );
            locals.var_q_d2_expnum__blk839_rv = 0.0;
        }
        if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1203 == 0.0)) {
            let assign39170_e44543: f64 = (locals.var_q_k1q1__blk823 - locals.var_q_qcoth__blk829);
            let assign39170_e44544: f64 = (1.0 / assign39170_e44543);
            (locals.var_q_temp2__blk815, locals.var_q_temp2__blk815_dn4, locals.var_q_temp2__blk815_dn6, locals.var_q_temp2__blk815_dn7, locals.var_q_temp2__blk815_dn8, locals.var_q_temp2__blk815_dn9, ) = (assign39170_e44544, (-((locals.var_q_k1q1__blk823_dn4 - locals.var_q_qcoth__blk829_dn4) / (assign39170_e44543 * assign39170_e44543))), (-((locals.var_q_k1q1__blk823_dn6 - locals.var_q_qcoth__blk829_dn6) / (assign39170_e44543 * assign39170_e44543))), (-((locals.var_q_k1q1__blk823_dn7 - locals.var_q_qcoth__blk829_dn7) / (assign39170_e44543 * assign39170_e44543))), (-((locals.var_q_k1q1__blk823_dn8 - locals.var_q_qcoth__blk829_dn8) / (assign39170_e44543 * assign39170_e44543))), (-((locals.var_q_k1q1__blk823_dn9 - locals.var_q_qcoth__blk829_dn9) / (assign39170_e44543 * assign39170_e44543))), );
            locals.var_q_temp2__blk815_rv = 0.0;
        }
        if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1203 == 0.0)) {
            let assign39180_e44557: f64 = (locals.var_q_d1_qcoth__blk830 - locals.var_k1__blk932);
            (locals.var_q_temp3__blk816, locals.var_q_temp3__blk816_dn4, locals.var_q_temp3__blk816_dn6, locals.var_q_temp3__blk816_dn7, locals.var_q_temp3__blk816_dn8, locals.var_q_temp3__blk816_dn9, ) = (assign39180_e44557, (locals.var_q_d1_qcoth__blk830_dn4 - locals.var_k1__blk932_dn4), (locals.var_q_d1_qcoth__blk830_dn6 - locals.var_k1__blk932_dn6), (locals.var_q_d1_qcoth__blk830_dn7 - locals.var_k1__blk932_dn7), (locals.var_q_d1_qcoth__blk830_dn8 - locals.var_k1__blk932_dn8), (locals.var_q_d1_qcoth__blk830_dn9 - locals.var_k1__blk932_dn9), );
            locals.var_q_temp3__blk816_rv = 0.0;
        }
        if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1203 == 0.0)) {
            let assign39190_e44570: f64 = (locals.var_q_aexp__blk824 - locals.var_q_sh_term__blk833);
            let assign39190_e44572: f64 = (assign39190_e44570 * locals.var_q_temp2__blk815);
            (locals.var_q_expnum__blk837, locals.var_q_expnum__blk837_dn4, locals.var_q_expnum__blk837_dn6, locals.var_q_expnum__blk837_dn7, locals.var_q_expnum__blk837_dn8, locals.var_q_expnum__blk837_dn9, ) = (assign39190_e44572, (((locals.var_q_aexp__blk824_dn4 - locals.var_q_sh_term__blk833_dn4) * locals.var_q_temp2__blk815) + (assign39190_e44570 * locals.var_q_temp2__blk815_dn4)), (((locals.var_q_aexp__blk824_dn6 - locals.var_q_sh_term__blk833_dn6) * locals.var_q_temp2__blk815) + (assign39190_e44570 * locals.var_q_temp2__blk815_dn6)), (((locals.var_q_aexp__blk824_dn7 - locals.var_q_sh_term__blk833_dn7) * locals.var_q_temp2__blk815) + (assign39190_e44570 * locals.var_q_temp2__blk815_dn7)), (((locals.var_q_aexp__blk824_dn8 - locals.var_q_sh_term__blk833_dn8) * locals.var_q_temp2__blk815) + (assign39190_e44570 * locals.var_q_temp2__blk815_dn8)), (((locals.var_q_aexp__blk824_dn9 - locals.var_q_sh_term__blk833_dn9) * locals.var_q_temp2__blk815) + (assign39190_e44570 * locals.var_q_temp2__blk815_dn9)), );
            locals.var_q_expnum__blk837_rv = 0.0;
        }
        if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1203 == 0.0)) {
            let assign39200_e44585: f64 = (locals.var_q_temp3__blk816 * locals.var_q_expnum__blk837);
            let assign39200_e44587: f64 = (assign39200_e44585 - locals.var_q_aexp__blk824);
            let assign39200_e44590: f64 = (locals.var_q_d1_ln__blk835 * locals.var_q_sh_term__blk833);
            let assign39200_e44591: f64 = (assign39200_e44587 - assign39200_e44590);
            let assign39200_e44593: f64 = (assign39200_e44591 * locals.var_q_temp2__blk815);
            (locals.var_q_d1_expnum__blk838, locals.var_q_d1_expnum__blk838_dn4, locals.var_q_d1_expnum__blk838_dn6, locals.var_q_d1_expnum__blk838_dn7, locals.var_q_d1_expnum__blk838_dn8, locals.var_q_d1_expnum__blk838_dn9, ) = (assign39200_e44593, ((((((locals.var_q_temp3__blk816_dn4 * locals.var_q_expnum__blk837) + (locals.var_q_temp3__blk816 * locals.var_q_expnum__blk837_dn4)) - locals.var_q_aexp__blk824_dn4) - ((locals.var_q_d1_ln__blk835_dn4 * locals.var_q_sh_term__blk833) + (locals.var_q_d1_ln__blk835 * locals.var_q_sh_term__blk833_dn4))) * locals.var_q_temp2__blk815) + (assign39200_e44591 * locals.var_q_temp2__blk815_dn4)), ((((((locals.var_q_temp3__blk816_dn6 * locals.var_q_expnum__blk837) + (locals.var_q_temp3__blk816 * locals.var_q_expnum__blk837_dn6)) - locals.var_q_aexp__blk824_dn6) - ((locals.var_q_d1_ln__blk835_dn6 * locals.var_q_sh_term__blk833) + (locals.var_q_d1_ln__blk835 * locals.var_q_sh_term__blk833_dn6))) * locals.var_q_temp2__blk815) + (assign39200_e44591 * locals.var_q_temp2__blk815_dn6)), ((((((locals.var_q_temp3__blk816_dn7 * locals.var_q_expnum__blk837) + (locals.var_q_temp3__blk816 * locals.var_q_expnum__blk837_dn7)) - locals.var_q_aexp__blk824_dn7) - ((locals.var_q_d1_ln__blk835_dn7 * locals.var_q_sh_term__blk833) + (locals.var_q_d1_ln__blk835 * locals.var_q_sh_term__blk833_dn7))) * locals.var_q_temp2__blk815) + (assign39200_e44591 * locals.var_q_temp2__blk815_dn7)), ((((((locals.var_q_temp3__blk816_dn8 * locals.var_q_expnum__blk837) + (locals.var_q_temp3__blk816 * locals.var_q_expnum__blk837_dn8)) - locals.var_q_aexp__blk824_dn8) - ((locals.var_q_d1_ln__blk835_dn8 * locals.var_q_sh_term__blk833) + (locals.var_q_d1_ln__blk835 * locals.var_q_sh_term__blk833_dn8))) * locals.var_q_temp2__blk815) + (assign39200_e44591 * locals.var_q_temp2__blk815_dn8)), ((((((locals.var_q_temp3__blk816_dn9 * locals.var_q_expnum__blk837) + (locals.var_q_temp3__blk816 * locals.var_q_expnum__blk837_dn9)) - locals.var_q_aexp__blk824_dn9) - ((locals.var_q_d1_ln__blk835_dn9 * locals.var_q_sh_term__blk833) + (locals.var_q_d1_ln__blk835 * locals.var_q_sh_term__blk833_dn9))) * locals.var_q_temp2__blk815) + (assign39200_e44591 * locals.var_q_temp2__blk815_dn9)), );
            locals.var_q_d1_expnum__blk838_rv = 0.0;
        }
        if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1203 == 0.0)) {
            let assign39210_e44606: f64 = (locals.var_q_d2_qcoth__blk832 * locals.var_q_expnum__blk837);
            let assign39210_e44609: f64 = (2.0 * locals.var_q_temp3__blk816);
            let assign39210_e44611: f64 = (assign39210_e44609 * locals.var_q_d1_expnum__blk838);
            let assign39210_e44612: f64 = (assign39210_e44606 + assign39210_e44611);
            let assign39210_e44614: f64 = (assign39210_e44612 + locals.var_q_aexp__blk824);
            let assign39210_e44618: f64 = (locals.var_q_d1_ln__blk835 * locals.var_q_d1_ln__blk835);
            let assign39210_e44619: f64 = (locals.var_q_d2_ln__blk836 + assign39210_e44618);
            let assign39210_e44621: f64 = (assign39210_e44619 * locals.var_q_sh_term__blk833);
            let assign39210_e44622: f64 = (assign39210_e44614 - assign39210_e44621);
            let assign39210_e44624: f64 = (assign39210_e44622 * locals.var_q_temp2__blk815);
            (locals.var_q_d2_expnum__blk839, locals.var_q_d2_expnum__blk839_dn4, locals.var_q_d2_expnum__blk839_dn6, locals.var_q_d2_expnum__blk839_dn7, locals.var_q_d2_expnum__blk839_dn8, locals.var_q_d2_expnum__blk839_dn9, ) = (assign39210_e44624, (((((((locals.var_q_d2_qcoth__blk832_dn4 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qcoth__blk832 * locals.var_q_expnum__blk837_dn4)) + (((2.0 * locals.var_q_temp3__blk816_dn4) * locals.var_q_d1_expnum__blk838) + (assign39210_e44609 * locals.var_q_d1_expnum__blk838_dn4))) + locals.var_q_aexp__blk824_dn4) - (((locals.var_q_d2_ln__blk836_dn4 + ((locals.var_q_d1_ln__blk835_dn4 * locals.var_q_d1_ln__blk835) + (locals.var_q_d1_ln__blk835 * locals.var_q_d1_ln__blk835_dn4))) * locals.var_q_sh_term__blk833) + (assign39210_e44619 * locals.var_q_sh_term__blk833_dn4))) * locals.var_q_temp2__blk815) + (assign39210_e44622 * locals.var_q_temp2__blk815_dn4)), (((((((locals.var_q_d2_qcoth__blk832_dn6 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qcoth__blk832 * locals.var_q_expnum__blk837_dn6)) + (((2.0 * locals.var_q_temp3__blk816_dn6) * locals.var_q_d1_expnum__blk838) + (assign39210_e44609 * locals.var_q_d1_expnum__blk838_dn6))) + locals.var_q_aexp__blk824_dn6) - (((locals.var_q_d2_ln__blk836_dn6 + ((locals.var_q_d1_ln__blk835_dn6 * locals.var_q_d1_ln__blk835) + (locals.var_q_d1_ln__blk835 * locals.var_q_d1_ln__blk835_dn6))) * locals.var_q_sh_term__blk833) + (assign39210_e44619 * locals.var_q_sh_term__blk833_dn6))) * locals.var_q_temp2__blk815) + (assign39210_e44622 * locals.var_q_temp2__blk815_dn6)), (((((((locals.var_q_d2_qcoth__blk832_dn7 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qcoth__blk832 * locals.var_q_expnum__blk837_dn7)) + (((2.0 * locals.var_q_temp3__blk816_dn7) * locals.var_q_d1_expnum__blk838) + (assign39210_e44609 * locals.var_q_d1_expnum__blk838_dn7))) + locals.var_q_aexp__blk824_dn7) - (((locals.var_q_d2_ln__blk836_dn7 + ((locals.var_q_d1_ln__blk835_dn7 * locals.var_q_d1_ln__blk835) + (locals.var_q_d1_ln__blk835 * locals.var_q_d1_ln__blk835_dn7))) * locals.var_q_sh_term__blk833) + (assign39210_e44619 * locals.var_q_sh_term__blk833_dn7))) * locals.var_q_temp2__blk815) + (assign39210_e44622 * locals.var_q_temp2__blk815_dn7)), (((((((locals.var_q_d2_qcoth__blk832_dn8 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qcoth__blk832 * locals.var_q_expnum__blk837_dn8)) + (((2.0 * locals.var_q_temp3__blk816_dn8) * locals.var_q_d1_expnum__blk838) + (assign39210_e44609 * locals.var_q_d1_expnum__blk838_dn8))) + locals.var_q_aexp__blk824_dn8) - (((locals.var_q_d2_ln__blk836_dn8 + ((locals.var_q_d1_ln__blk835_dn8 * locals.var_q_d1_ln__blk835) + (locals.var_q_d1_ln__blk835 * locals.var_q_d1_ln__blk835_dn8))) * locals.var_q_sh_term__blk833) + (assign39210_e44619 * locals.var_q_sh_term__blk833_dn8))) * locals.var_q_temp2__blk815) + (assign39210_e44622 * locals.var_q_temp2__blk815_dn8)), (((((((locals.var_q_d2_qcoth__blk832_dn9 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qcoth__blk832 * locals.var_q_expnum__blk837_dn9)) + (((2.0 * locals.var_q_temp3__blk816_dn9) * locals.var_q_d1_expnum__blk838) + (assign39210_e44609 * locals.var_q_d1_expnum__blk838_dn9))) + locals.var_q_aexp__blk824_dn9) - (((locals.var_q_d2_ln__blk836_dn9 + ((locals.var_q_d1_ln__blk835_dn9 * locals.var_q_d1_ln__blk835) + (locals.var_q_d1_ln__blk835 * locals.var_q_d1_ln__blk835_dn9))) * locals.var_q_sh_term__blk833) + (assign39210_e44619 * locals.var_q_sh_term__blk833_dn9))) * locals.var_q_temp2__blk815) + (assign39210_e44622 * locals.var_q_temp2__blk815_dn9)), );
            locals.var_q_d2_expnum__blk839_rv = 0.0;
        }
        let assign39220_e44629: f64 = if locals.var_q_expnum__blk837 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1204 = assign39220_e44629;
        locals.var_guard1204_rv = 0.0;
        if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1204 != 0.0)) {
            let assign39230_e44638: f64 = (locals.var_q_expnum__blk837).ln();
            (locals.var_q_lnexpnum__blk840, locals.var_q_lnexpnum__blk840_dn4, locals.var_q_lnexpnum__blk840_dn6, locals.var_q_lnexpnum__blk840_dn7, locals.var_q_lnexpnum__blk840_dn8, locals.var_q_lnexpnum__blk840_dn9, ) = (assign39230_e44638, (locals.var_q_expnum__blk837_dn4 / locals.var_q_expnum__blk837), (locals.var_q_expnum__blk837_dn6 / locals.var_q_expnum__blk837), (locals.var_q_expnum__blk837_dn7 / locals.var_q_expnum__blk837), (locals.var_q_expnum__blk837_dn8 / locals.var_q_expnum__blk837), (locals.var_q_expnum__blk837_dn9 / locals.var_q_expnum__blk837), );
            locals.var_q_lnexpnum__blk840_rv = 0.0;
        }
        if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1204 != 0.0)) {
            let assign39240_e44650: f64 = (1.0 / locals.var_q_expnum__blk837);
            (locals.var_q_temp1__blk814, locals.var_q_temp1__blk814_dn4, locals.var_q_temp1__blk814_dn6, locals.var_q_temp1__blk814_dn7, locals.var_q_temp1__blk814_dn8, locals.var_q_temp1__blk814_dn9, ) = (assign39240_e44650, (-(locals.var_q_expnum__blk837_dn4 / (locals.var_q_expnum__blk837 * locals.var_q_expnum__blk837))), (-(locals.var_q_expnum__blk837_dn6 / (locals.var_q_expnum__blk837 * locals.var_q_expnum__blk837))), (-(locals.var_q_expnum__blk837_dn7 / (locals.var_q_expnum__blk837 * locals.var_q_expnum__blk837))), (-(locals.var_q_expnum__blk837_dn8 / (locals.var_q_expnum__blk837 * locals.var_q_expnum__blk837))), (-(locals.var_q_expnum__blk837_dn9 / (locals.var_q_expnum__blk837 * locals.var_q_expnum__blk837))), );
            locals.var_q_temp1__blk814_rv = 0.0;
        }
        if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1204 != 0.0)) {
            let assign39250_e44662: f64 = (locals.var_q_d1_expnum__blk838 * locals.var_q_temp1__blk814);
            (locals.var_q_d1_lnexpnum__blk841, locals.var_q_d1_lnexpnum__blk841_dn4, locals.var_q_d1_lnexpnum__blk841_dn6, locals.var_q_d1_lnexpnum__blk841_dn7, locals.var_q_d1_lnexpnum__blk841_dn8, locals.var_q_d1_lnexpnum__blk841_dn9, ) = (assign39250_e44662, ((locals.var_q_d1_expnum__blk838_dn4 * locals.var_q_temp1__blk814) + (locals.var_q_d1_expnum__blk838 * locals.var_q_temp1__blk814_dn4)), ((locals.var_q_d1_expnum__blk838_dn6 * locals.var_q_temp1__blk814) + (locals.var_q_d1_expnum__blk838 * locals.var_q_temp1__blk814_dn6)), ((locals.var_q_d1_expnum__blk838_dn7 * locals.var_q_temp1__blk814) + (locals.var_q_d1_expnum__blk838 * locals.var_q_temp1__blk814_dn7)), ((locals.var_q_d1_expnum__blk838_dn8 * locals.var_q_temp1__blk814) + (locals.var_q_d1_expnum__blk838 * locals.var_q_temp1__blk814_dn8)), ((locals.var_q_d1_expnum__blk838_dn9 * locals.var_q_temp1__blk814) + (locals.var_q_d1_expnum__blk838 * locals.var_q_temp1__blk814_dn9)), );
            locals.var_q_d1_lnexpnum__blk841_rv = 0.0;
        }
        if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1204 != 0.0)) {
            let assign39260_e44674: f64 = (locals.var_q_d2_expnum__blk839 * locals.var_q_temp1__blk814);
            let assign39260_e44677: f64 = (locals.var_q_d1_lnexpnum__blk841 * locals.var_q_d1_lnexpnum__blk841);
            let assign39260_e44678: f64 = (assign39260_e44674 - assign39260_e44677);
            (locals.var_q_d2_lnexpnum__blk842, locals.var_q_d2_lnexpnum__blk842_dn4, locals.var_q_d2_lnexpnum__blk842_dn6, locals.var_q_d2_lnexpnum__blk842_dn7, locals.var_q_d2_lnexpnum__blk842_dn8, locals.var_q_d2_lnexpnum__blk842_dn9, ) = (assign39260_e44678, (((locals.var_q_d2_expnum__blk839_dn4 * locals.var_q_temp1__blk814) + (locals.var_q_d2_expnum__blk839 * locals.var_q_temp1__blk814_dn4)) - ((locals.var_q_d1_lnexpnum__blk841_dn4 * locals.var_q_d1_lnexpnum__blk841) + (locals.var_q_d1_lnexpnum__blk841 * locals.var_q_d1_lnexpnum__blk841_dn4))), (((locals.var_q_d2_expnum__blk839_dn6 * locals.var_q_temp1__blk814) + (locals.var_q_d2_expnum__blk839 * locals.var_q_temp1__blk814_dn6)) - ((locals.var_q_d1_lnexpnum__blk841_dn6 * locals.var_q_d1_lnexpnum__blk841) + (locals.var_q_d1_lnexpnum__blk841 * locals.var_q_d1_lnexpnum__blk841_dn6))), (((locals.var_q_d2_expnum__blk839_dn7 * locals.var_q_temp1__blk814) + (locals.var_q_d2_expnum__blk839 * locals.var_q_temp1__blk814_dn7)) - ((locals.var_q_d1_lnexpnum__blk841_dn7 * locals.var_q_d1_lnexpnum__blk841) + (locals.var_q_d1_lnexpnum__blk841 * locals.var_q_d1_lnexpnum__blk841_dn7))), (((locals.var_q_d2_expnum__blk839_dn8 * locals.var_q_temp1__blk814) + (locals.var_q_d2_expnum__blk839 * locals.var_q_temp1__blk814_dn8)) - ((locals.var_q_d1_lnexpnum__blk841_dn8 * locals.var_q_d1_lnexpnum__blk841) + (locals.var_q_d1_lnexpnum__blk841 * locals.var_q_d1_lnexpnum__blk841_dn8))), (((locals.var_q_d2_expnum__blk839_dn9 * locals.var_q_temp1__blk814) + (locals.var_q_d2_expnum__blk839 * locals.var_q_temp1__blk814_dn9)) - ((locals.var_q_d1_lnexpnum__blk841_dn9 * locals.var_q_d1_lnexpnum__blk841) + (locals.var_q_d1_lnexpnum__blk841 * locals.var_q_d1_lnexpnum__blk841_dn9))), );
            locals.var_q_d2_lnexpnum__blk842_rv = 0.0;
        }
        if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1204 == 0.0)) {
            let assign39270_e44691: f64 = (locals.var_q_k1q1__blk823 + 0.6931471805599);
            let assign39270_e44693: f64 = (-locals.var_q_k1q1__blk823);
            let assign39270_e44694: f64 = (assign39270_e44693).ln();
            let assign39270_e44695: f64 = (assign39270_e44691 + assign39270_e44694);
            (locals.var_q_lnexpnum__blk840, locals.var_q_lnexpnum__blk840_dn4, locals.var_q_lnexpnum__blk840_dn6, locals.var_q_lnexpnum__blk840_dn7, locals.var_q_lnexpnum__blk840_dn8, locals.var_q_lnexpnum__blk840_dn9, ) = (assign39270_e44695, (locals.var_q_k1q1__blk823_dn4 + ((-locals.var_q_k1q1__blk823_dn4) / assign39270_e44693)), (locals.var_q_k1q1__blk823_dn6 + ((-locals.var_q_k1q1__blk823_dn6) / assign39270_e44693)), (locals.var_q_k1q1__blk823_dn7 + ((-locals.var_q_k1q1__blk823_dn7) / assign39270_e44693)), (locals.var_q_k1q1__blk823_dn8 + ((-locals.var_q_k1q1__blk823_dn8) / assign39270_e44693)), (locals.var_q_k1q1__blk823_dn9 + ((-locals.var_q_k1q1__blk823_dn9) / assign39270_e44693)), );
            locals.var_q_lnexpnum__blk840_rv = 0.0;
        }
        if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1204 == 0.0)) {
            let assign39280_e44708: f64 = (1.0 / locals.var_q1d__blk1001);
            (locals.var_q_temp1__blk814, locals.var_q_temp1__blk814_dn4, locals.var_q_temp1__blk814_dn6, locals.var_q_temp1__blk814_dn7, locals.var_q_temp1__blk814_dn8, locals.var_q_temp1__blk814_dn9, ) = (assign39280_e44708, (-(locals.var_q1d__blk1001_dn4 / (locals.var_q1d__blk1001 * locals.var_q1d__blk1001))), (-(locals.var_q1d__blk1001_dn6 / (locals.var_q1d__blk1001 * locals.var_q1d__blk1001))), (-(locals.var_q1d__blk1001_dn7 / (locals.var_q1d__blk1001 * locals.var_q1d__blk1001))), (-(locals.var_q1d__blk1001_dn8 / (locals.var_q1d__blk1001 * locals.var_q1d__blk1001))), (-(locals.var_q1d__blk1001_dn9 / (locals.var_q1d__blk1001 * locals.var_q1d__blk1001))), );
            locals.var_q_temp1__blk814_rv = 0.0;
        }
        if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1204 == 0.0)) {
            let assign39290_e44721: f64 = (locals.var_k1__blk932 + locals.var_q_temp1__blk814);
            (locals.var_q_d1_lnexpnum__blk841, locals.var_q_d1_lnexpnum__blk841_dn4, locals.var_q_d1_lnexpnum__blk841_dn6, locals.var_q_d1_lnexpnum__blk841_dn7, locals.var_q_d1_lnexpnum__blk841_dn8, locals.var_q_d1_lnexpnum__blk841_dn9, ) = (assign39290_e44721, (locals.var_k1__blk932_dn4 + locals.var_q_temp1__blk814_dn4), (locals.var_k1__blk932_dn6 + locals.var_q_temp1__blk814_dn6), (locals.var_k1__blk932_dn7 + locals.var_q_temp1__blk814_dn7), (locals.var_k1__blk932_dn8 + locals.var_q_temp1__blk814_dn8), (locals.var_k1__blk932_dn9 + locals.var_q_temp1__blk814_dn9), );
            locals.var_q_d1_lnexpnum__blk841_rv = 0.0;
        }
        if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1204 == 0.0)) {
            let assign39300_e44733: f64 = (-locals.var_q_temp1__blk814);
            let assign39300_e44735: f64 = (assign39300_e44733 * locals.var_q_temp1__blk814);
            (locals.var_q_d2_lnexpnum__blk842, locals.var_q_d2_lnexpnum__blk842_dn4, locals.var_q_d2_lnexpnum__blk842_dn6, locals.var_q_d2_lnexpnum__blk842_dn7, locals.var_q_d2_lnexpnum__blk842_dn8, locals.var_q_d2_lnexpnum__blk842_dn9, ) = (assign39300_e44735, (((-locals.var_q_temp1__blk814_dn4) * locals.var_q_temp1__blk814) + (assign39300_e44733 * locals.var_q_temp1__blk814_dn4)), (((-locals.var_q_temp1__blk814_dn6) * locals.var_q_temp1__blk814) + (assign39300_e44733 * locals.var_q_temp1__blk814_dn6)), (((-locals.var_q_temp1__blk814_dn7) * locals.var_q_temp1__blk814) + (assign39300_e44733 * locals.var_q_temp1__blk814_dn7)), (((-locals.var_q_temp1__blk814_dn8) * locals.var_q_temp1__blk814) + (assign39300_e44733 * locals.var_q_temp1__blk814_dn8)), (((-locals.var_q_temp1__blk814_dn9) * locals.var_q_temp1__blk814) + (assign39300_e44733 * locals.var_q_temp1__blk814_dn9)), );
            locals.var_q_d2_lnexpnum__blk842_rv = 0.0;
        }
        if (((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) {
            let assign39310_e44745: f64 = (locals.var_xg2x__blk931 - locals.var_xg1x__blk930);
            let assign39310_e44747: f64 = (assign39310_e44745 + locals.var_q1d__blk1001);
            let assign39310_e44750: f64 = (2.0 * locals.var_q_lnexpnum__blk840);
            let assign39310_e44751: f64 = (assign39310_e44747 + assign39310_e44750);
            let assign39310_e44753: f64 = (assign39310_e44751 - locals.var_q_ln_term__blk834);
            (locals.var_q_q2_int__blk843, locals.var_q_q2_int__blk843_dn4, locals.var_q_q2_int__blk843_dn6, locals.var_q_q2_int__blk843_dn7, locals.var_q_q2_int__blk843_dn8, locals.var_q_q2_int__blk843_dn9, ) = (assign39310_e44753, ((((locals.var_xg2x__blk931_dn4 - locals.var_xg1x__blk930_dn4) + locals.var_q1d__blk1001_dn4) + (2.0 * locals.var_q_lnexpnum__blk840_dn4)) - locals.var_q_ln_term__blk834_dn4), ((((locals.var_xg2x__blk931_dn6 - locals.var_xg1x__blk930_dn6) + locals.var_q1d__blk1001_dn6) + (2.0 * locals.var_q_lnexpnum__blk840_dn6)) - locals.var_q_ln_term__blk834_dn6), ((((locals.var_xg2x__blk931_dn7 - locals.var_xg1x__blk930_dn7) + locals.var_q1d__blk1001_dn7) + (2.0 * locals.var_q_lnexpnum__blk840_dn7)) - locals.var_q_ln_term__blk834_dn7), ((((locals.var_xg2x__blk931_dn8 - locals.var_xg1x__blk930_dn8) + locals.var_q1d__blk1001_dn8) + (2.0 * locals.var_q_lnexpnum__blk840_dn8)) - locals.var_q_ln_term__blk834_dn8), ((((locals.var_xg2x__blk931_dn9 - locals.var_xg1x__blk930_dn9) + locals.var_q1d__blk1001_dn9) + (2.0 * locals.var_q_lnexpnum__blk840_dn9)) - locals.var_q_ln_term__blk834_dn9), );
            locals.var_q_q2_int__blk843_rv = 0.0;
        }
        if (((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) {
            let assign39320_e44764: f64 = (2.0 * locals.var_q_d1_lnexpnum__blk841);
            let assign39320_e44765: f64 = (1.0 + assign39320_e44764);
            let assign39320_e44767: f64 = (assign39320_e44765 - locals.var_q_d1_ln__blk835);
            (locals.var_q_d1_q2__blk844, locals.var_q_d1_q2__blk844_dn4, locals.var_q_d1_q2__blk844_dn6, locals.var_q_d1_q2__blk844_dn7, locals.var_q_d1_q2__blk844_dn8, locals.var_q_d1_q2__blk844_dn9, ) = (assign39320_e44767, ((2.0 * locals.var_q_d1_lnexpnum__blk841_dn4) - locals.var_q_d1_ln__blk835_dn4), ((2.0 * locals.var_q_d1_lnexpnum__blk841_dn6) - locals.var_q_d1_ln__blk835_dn6), ((2.0 * locals.var_q_d1_lnexpnum__blk841_dn7) - locals.var_q_d1_ln__blk835_dn7), ((2.0 * locals.var_q_d1_lnexpnum__blk841_dn8) - locals.var_q_d1_ln__blk835_dn8), ((2.0 * locals.var_q_d1_lnexpnum__blk841_dn9) - locals.var_q_d1_ln__blk835_dn9), );
            locals.var_q_d1_q2__blk844_rv = 0.0;
        }
        if (((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) {
            let assign39330_e44777: f64 = (2.0 * locals.var_q_d2_lnexpnum__blk842);
            let assign39330_e44779: f64 = (assign39330_e44777 - locals.var_q_d2_ln__blk836);
            (locals.var_q_d2_q2__blk845, locals.var_q_d2_q2__blk845_dn4, locals.var_q_d2_q2__blk845_dn6, locals.var_q_d2_q2__blk845_dn7, locals.var_q_d2_q2__blk845_dn8, locals.var_q_d2_q2__blk845_dn9, ) = (assign39330_e44779, ((2.0 * locals.var_q_d2_lnexpnum__blk842_dn4) - locals.var_q_d2_ln__blk836_dn4), ((2.0 * locals.var_q_d2_lnexpnum__blk842_dn6) - locals.var_q_d2_ln__blk836_dn6), ((2.0 * locals.var_q_d2_lnexpnum__blk842_dn7) - locals.var_q_d2_ln__blk836_dn7), ((2.0 * locals.var_q_d2_lnexpnum__blk842_dn8) - locals.var_q_d2_ln__blk836_dn8), ((2.0 * locals.var_q_d2_lnexpnum__blk842_dn9) - locals.var_q_d2_ln__blk836_dn9), );
            locals.var_q_d2_q2__blk845_rv = 0.0;
        }
        if (((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) {
            let assign39340_e44790: f64 = (locals.var_k2__blk933 * locals.var_q_q2_int__blk843);
            let assign39340_e44791: f64 = (locals.var_q_k1q1__blk823 + assign39340_e44790);
            (locals.var_q_qi_int__blk846, locals.var_q_qi_int__blk846_dn4, locals.var_q_qi_int__blk846_dn6, locals.var_q_qi_int__blk846_dn7, locals.var_q_qi_int__blk846_dn8, locals.var_q_qi_int__blk846_dn9, ) = (assign39340_e44791, (locals.var_q_k1q1__blk823_dn4 + ((locals.var_k2__blk933_dn4 * locals.var_q_q2_int__blk843) + (locals.var_k2__blk933 * locals.var_q_q2_int__blk843_dn4))), (locals.var_q_k1q1__blk823_dn6 + ((locals.var_k2__blk933_dn6 * locals.var_q_q2_int__blk843) + (locals.var_k2__blk933 * locals.var_q_q2_int__blk843_dn6))), (locals.var_q_k1q1__blk823_dn7 + ((locals.var_k2__blk933_dn7 * locals.var_q_q2_int__blk843) + (locals.var_k2__blk933 * locals.var_q_q2_int__blk843_dn7))), (locals.var_q_k1q1__blk823_dn8 + ((locals.var_k2__blk933_dn8 * locals.var_q_q2_int__blk843) + (locals.var_k2__blk933 * locals.var_q_q2_int__blk843_dn8))), (locals.var_q_k1q1__blk823_dn9 + ((locals.var_k2__blk933_dn9 * locals.var_q_q2_int__blk843) + (locals.var_k2__blk933 * locals.var_q_q2_int__blk843_dn9))), );
            locals.var_q_qi_int__blk846_rv = 0.0;
        }
        if (((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) {
            let assign39350_e44802: f64 = (locals.var_k2__blk933 * locals.var_q_d1_q2__blk844);
            let assign39350_e44803: f64 = (locals.var_k1__blk932 + assign39350_e44802);
            (locals.var_q_d1_qi__blk847, locals.var_q_d1_qi__blk847_dn4, locals.var_q_d1_qi__blk847_dn6, locals.var_q_d1_qi__blk847_dn7, locals.var_q_d1_qi__blk847_dn8, locals.var_q_d1_qi__blk847_dn9, ) = (assign39350_e44803, (locals.var_k1__blk932_dn4 + ((locals.var_k2__blk933_dn4 * locals.var_q_d1_q2__blk844) + (locals.var_k2__blk933 * locals.var_q_d1_q2__blk844_dn4))), (locals.var_k1__blk932_dn6 + ((locals.var_k2__blk933_dn6 * locals.var_q_d1_q2__blk844) + (locals.var_k2__blk933 * locals.var_q_d1_q2__blk844_dn6))), (locals.var_k1__blk932_dn7 + ((locals.var_k2__blk933_dn7 * locals.var_q_d1_q2__blk844) + (locals.var_k2__blk933 * locals.var_q_d1_q2__blk844_dn7))), (locals.var_k1__blk932_dn8 + ((locals.var_k2__blk933_dn8 * locals.var_q_d1_q2__blk844) + (locals.var_k2__blk933 * locals.var_q_d1_q2__blk844_dn8))), (locals.var_k1__blk932_dn9 + ((locals.var_k2__blk933_dn9 * locals.var_q_d1_q2__blk844) + (locals.var_k2__blk933 * locals.var_q_d1_q2__blk844_dn9))), );
            locals.var_q_d1_qi__blk847_rv = 0.0;
        }
        if (((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) {
            let assign39360_e44813: f64 = (locals.var_k2__blk933 * locals.var_q_d2_q2__blk845);
            (locals.var_q_d2_qi__blk848, locals.var_q_d2_qi__blk848_dn4, locals.var_q_d2_qi__blk848_dn6, locals.var_q_d2_qi__blk848_dn7, locals.var_q_d2_qi__blk848_dn8, locals.var_q_d2_qi__blk848_dn9, ) = (assign39360_e44813, ((locals.var_k2__blk933_dn4 * locals.var_q_d2_q2__blk845) + (locals.var_k2__blk933 * locals.var_q_d2_q2__blk845_dn4)), ((locals.var_k2__blk933_dn6 * locals.var_q_d2_q2__blk845) + (locals.var_k2__blk933 * locals.var_q_d2_q2__blk845_dn6)), ((locals.var_k2__blk933_dn7 * locals.var_q_d2_q2__blk845) + (locals.var_k2__blk933 * locals.var_q_d2_q2__blk845_dn7)), ((locals.var_k2__blk933_dn8 * locals.var_q_d2_q2__blk845) + (locals.var_k2__blk933 * locals.var_q_d2_q2__blk845_dn8)), ((locals.var_k2__blk933_dn9 * locals.var_q_d2_q2__blk845) + (locals.var_k2__blk933 * locals.var_q_d2_q2__blk845_dn9)), );
            locals.var_q_d2_qi__blk848_rv = 0.0;
        }
        if (((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) {
            let assign39370_e44823: f64 = (locals.var_q_qi_int__blk846 * locals.var_q_expnum__blk837);
            let assign39370_e44825: f64 = (assign39370_e44823 - locals.var_q_aexp__blk824);
            (locals.var_q_zero__blk849, locals.var_q_zero__blk849_dn4, locals.var_q_zero__blk849_dn6, locals.var_q_zero__blk849_dn7, locals.var_q_zero__blk849_dn8, locals.var_q_zero__blk849_dn9, ) = (assign39370_e44825, (((locals.var_q_qi_int__blk846_dn4 * locals.var_q_expnum__blk837) + (locals.var_q_qi_int__blk846 * locals.var_q_expnum__blk837_dn4)) - locals.var_q_aexp__blk824_dn4), (((locals.var_q_qi_int__blk846_dn6 * locals.var_q_expnum__blk837) + (locals.var_q_qi_int__blk846 * locals.var_q_expnum__blk837_dn6)) - locals.var_q_aexp__blk824_dn6), (((locals.var_q_qi_int__blk846_dn7 * locals.var_q_expnum__blk837) + (locals.var_q_qi_int__blk846 * locals.var_q_expnum__blk837_dn7)) - locals.var_q_aexp__blk824_dn7), (((locals.var_q_qi_int__blk846_dn8 * locals.var_q_expnum__blk837) + (locals.var_q_qi_int__blk846 * locals.var_q_expnum__blk837_dn8)) - locals.var_q_aexp__blk824_dn8), (((locals.var_q_qi_int__blk846_dn9 * locals.var_q_expnum__blk837) + (locals.var_q_qi_int__blk846 * locals.var_q_expnum__blk837_dn9)) - locals.var_q_aexp__blk824_dn9), );
            locals.var_q_zero__blk849_rv = 0.0;
        }
        if (((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) {
            let assign39380_e44835: f64 = (locals.var_q_d1_qi__blk847 * locals.var_q_expnum__blk837);
            let assign39380_e44838: f64 = (locals.var_q_qi_int__blk846 * locals.var_q_d1_expnum__blk838);
            let assign39380_e44839: f64 = (assign39380_e44835 + assign39380_e44838);
            let assign39380_e44841: f64 = (assign39380_e44839 + locals.var_q_aexp__blk824);
            (locals.var_q_d1_zero__blk850, locals.var_q_d1_zero__blk850_dn4, locals.var_q_d1_zero__blk850_dn6, locals.var_q_d1_zero__blk850_dn7, locals.var_q_d1_zero__blk850_dn8, locals.var_q_d1_zero__blk850_dn9, ) = (assign39380_e44841, ((((locals.var_q_d1_qi__blk847_dn4 * locals.var_q_expnum__blk837) + (locals.var_q_d1_qi__blk847 * locals.var_q_expnum__blk837_dn4)) + ((locals.var_q_qi_int__blk846_dn4 * locals.var_q_d1_expnum__blk838) + (locals.var_q_qi_int__blk846 * locals.var_q_d1_expnum__blk838_dn4))) + locals.var_q_aexp__blk824_dn4), ((((locals.var_q_d1_qi__blk847_dn6 * locals.var_q_expnum__blk837) + (locals.var_q_d1_qi__blk847 * locals.var_q_expnum__blk837_dn6)) + ((locals.var_q_qi_int__blk846_dn6 * locals.var_q_d1_expnum__blk838) + (locals.var_q_qi_int__blk846 * locals.var_q_d1_expnum__blk838_dn6))) + locals.var_q_aexp__blk824_dn6), ((((locals.var_q_d1_qi__blk847_dn7 * locals.var_q_expnum__blk837) + (locals.var_q_d1_qi__blk847 * locals.var_q_expnum__blk837_dn7)) + ((locals.var_q_qi_int__blk846_dn7 * locals.var_q_d1_expnum__blk838) + (locals.var_q_qi_int__blk846 * locals.var_q_d1_expnum__blk838_dn7))) + locals.var_q_aexp__blk824_dn7), ((((locals.var_q_d1_qi__blk847_dn8 * locals.var_q_expnum__blk837) + (locals.var_q_d1_qi__blk847 * locals.var_q_expnum__blk837_dn8)) + ((locals.var_q_qi_int__blk846_dn8 * locals.var_q_d1_expnum__blk838) + (locals.var_q_qi_int__blk846 * locals.var_q_d1_expnum__blk838_dn8))) + locals.var_q_aexp__blk824_dn8), ((((locals.var_q_d1_qi__blk847_dn9 * locals.var_q_expnum__blk837) + (locals.var_q_d1_qi__blk847 * locals.var_q_expnum__blk837_dn9)) + ((locals.var_q_qi_int__blk846_dn9 * locals.var_q_d1_expnum__blk838) + (locals.var_q_qi_int__blk846 * locals.var_q_d1_expnum__blk838_dn9))) + locals.var_q_aexp__blk824_dn9), );
            locals.var_q_d1_zero__blk850_rv = 0.0;
        }
        if (((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) {
            let assign39390_e44851: f64 = (locals.var_q_d2_qi__blk848 * locals.var_q_expnum__blk837);
            let assign39390_e44854: f64 = (2.0 * locals.var_q_d1_qi__blk847);
            let assign39390_e44856: f64 = (assign39390_e44854 * locals.var_q_d1_expnum__blk838);
            let assign39390_e44857: f64 = (assign39390_e44851 + assign39390_e44856);
            let assign39390_e44860: f64 = (locals.var_q_qi_int__blk846 * locals.var_q_d2_expnum__blk839);
            let assign39390_e44861: f64 = (assign39390_e44857 + assign39390_e44860);
            let assign39390_e44863: f64 = (assign39390_e44861 - locals.var_q_aexp__blk824);
            (locals.var_q_d2_zero__blk851, locals.var_q_d2_zero__blk851_dn4, locals.var_q_d2_zero__blk851_dn6, locals.var_q_d2_zero__blk851_dn7, locals.var_q_d2_zero__blk851_dn8, locals.var_q_d2_zero__blk851_dn9, ) = (assign39390_e44863, (((((locals.var_q_d2_qi__blk848_dn4 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qi__blk848 * locals.var_q_expnum__blk837_dn4)) + (((2.0 * locals.var_q_d1_qi__blk847_dn4) * locals.var_q_d1_expnum__blk838) + (assign39390_e44854 * locals.var_q_d1_expnum__blk838_dn4))) + ((locals.var_q_qi_int__blk846_dn4 * locals.var_q_d2_expnum__blk839) + (locals.var_q_qi_int__blk846 * locals.var_q_d2_expnum__blk839_dn4))) - locals.var_q_aexp__blk824_dn4), (((((locals.var_q_d2_qi__blk848_dn6 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qi__blk848 * locals.var_q_expnum__blk837_dn6)) + (((2.0 * locals.var_q_d1_qi__blk847_dn6) * locals.var_q_d1_expnum__blk838) + (assign39390_e44854 * locals.var_q_d1_expnum__blk838_dn6))) + ((locals.var_q_qi_int__blk846_dn6 * locals.var_q_d2_expnum__blk839) + (locals.var_q_qi_int__blk846 * locals.var_q_d2_expnum__blk839_dn6))) - locals.var_q_aexp__blk824_dn6), (((((locals.var_q_d2_qi__blk848_dn7 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qi__blk848 * locals.var_q_expnum__blk837_dn7)) + (((2.0 * locals.var_q_d1_qi__blk847_dn7) * locals.var_q_d1_expnum__blk838) + (assign39390_e44854 * locals.var_q_d1_expnum__blk838_dn7))) + ((locals.var_q_qi_int__blk846_dn7 * locals.var_q_d2_expnum__blk839) + (locals.var_q_qi_int__blk846 * locals.var_q_d2_expnum__blk839_dn7))) - locals.var_q_aexp__blk824_dn7), (((((locals.var_q_d2_qi__blk848_dn8 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qi__blk848 * locals.var_q_expnum__blk837_dn8)) + (((2.0 * locals.var_q_d1_qi__blk847_dn8) * locals.var_q_d1_expnum__blk838) + (assign39390_e44854 * locals.var_q_d1_expnum__blk838_dn8))) + ((locals.var_q_qi_int__blk846_dn8 * locals.var_q_d2_expnum__blk839) + (locals.var_q_qi_int__blk846 * locals.var_q_d2_expnum__blk839_dn8))) - locals.var_q_aexp__blk824_dn8), (((((locals.var_q_d2_qi__blk848_dn9 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qi__blk848 * locals.var_q_expnum__blk837_dn9)) + (((2.0 * locals.var_q_d1_qi__blk847_dn9) * locals.var_q_d1_expnum__blk838) + (assign39390_e44854 * locals.var_q_d1_expnum__blk838_dn9))) + ((locals.var_q_qi_int__blk846_dn9 * locals.var_q_d2_expnum__blk839) + (locals.var_q_qi_int__blk846 * locals.var_q_d2_expnum__blk839_dn9))) - locals.var_q_aexp__blk824_dn9), );
            locals.var_q_d2_zero__blk851_rv = 0.0;
        }
        if (((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) {
            let assign39400_e44873: f64 = (locals.var_q_d1_zero__blk850 * locals.var_q_d1_zero__blk850);
            let assign39400_e44876: f64 = (0.5 * locals.var_q_zero__blk849);
            let assign39400_e44878: f64 = (assign39400_e44876 * locals.var_q_d2_zero__blk851);
            let assign39400_e44879: f64 = (assign39400_e44873 - assign39400_e44878);
            (locals.var_q_temp__blk860, locals.var_q_temp__blk860_dn4, locals.var_q_temp__blk860_dn6, locals.var_q_temp__blk860_dn7, locals.var_q_temp__blk860_dn8, locals.var_q_temp__blk860_dn9, ) = (assign39400_e44879, (((locals.var_q_d1_zero__blk850_dn4 * locals.var_q_d1_zero__blk850) + (locals.var_q_d1_zero__blk850 * locals.var_q_d1_zero__blk850_dn4)) - (((0.5 * locals.var_q_zero__blk849_dn4) * locals.var_q_d2_zero__blk851) + (assign39400_e44876 * locals.var_q_d2_zero__blk851_dn4))), (((locals.var_q_d1_zero__blk850_dn6 * locals.var_q_d1_zero__blk850) + (locals.var_q_d1_zero__blk850 * locals.var_q_d1_zero__blk850_dn6)) - (((0.5 * locals.var_q_zero__blk849_dn6) * locals.var_q_d2_zero__blk851) + (assign39400_e44876 * locals.var_q_d2_zero__blk851_dn6))), (((locals.var_q_d1_zero__blk850_dn7 * locals.var_q_d1_zero__blk850) + (locals.var_q_d1_zero__blk850 * locals.var_q_d1_zero__blk850_dn7)) - (((0.5 * locals.var_q_zero__blk849_dn7) * locals.var_q_d2_zero__blk851) + (assign39400_e44876 * locals.var_q_d2_zero__blk851_dn7))), (((locals.var_q_d1_zero__blk850_dn8 * locals.var_q_d1_zero__blk850) + (locals.var_q_d1_zero__blk850 * locals.var_q_d1_zero__blk850_dn8)) - (((0.5 * locals.var_q_zero__blk849_dn8) * locals.var_q_d2_zero__blk851) + (assign39400_e44876 * locals.var_q_d2_zero__blk851_dn8))), (((locals.var_q_d1_zero__blk850_dn9 * locals.var_q_d1_zero__blk850) + (locals.var_q_d1_zero__blk850 * locals.var_q_d1_zero__blk850_dn9)) - (((0.5 * locals.var_q_zero__blk849_dn9) * locals.var_q_d2_zero__blk851) + (assign39400_e44876 * locals.var_q_d2_zero__blk851_dn9))), );
            locals.var_q_temp__blk860_rv = 0.0;
        }
        if (((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) {
            let assign39410_e44888: f64 = (-locals.var_q_zero__blk849);
            let assign39410_e44890: f64 = (assign39410_e44888 * locals.var_q_d1_zero__blk850);
            let assign39410_e44892: f64 = (assign39410_e44890 * locals.var_q_temp__blk860);
            let assign39410_e44895: f64 = (locals.var_q_temp__blk860 * locals.var_q_temp__blk860);
            let assign39410_e44897: f64 = (assign39410_e44895 + 1e-200);
            let assign39410_e44898: f64 = (assign39410_e44892 / assign39410_e44897);
            (locals.var_q_eps2__blk852, locals.var_q_eps2__blk852_dn4, locals.var_q_eps2__blk852_dn6, locals.var_q_eps2__blk852_dn7, locals.var_q_eps2__blk852_dn8, locals.var_q_eps2__blk852_dn9, ) = (assign39410_e44898, ((((((((-locals.var_q_zero__blk849_dn4) * locals.var_q_d1_zero__blk850) + (assign39410_e44888 * locals.var_q_d1_zero__blk850_dn4)) * locals.var_q_temp__blk860) + (assign39410_e44890 * locals.var_q_temp__blk860_dn4)) * assign39410_e44897) - (assign39410_e44892 * ((locals.var_q_temp__blk860_dn4 * locals.var_q_temp__blk860) + (locals.var_q_temp__blk860 * locals.var_q_temp__blk860_dn4)))) / (assign39410_e44897 * assign39410_e44897)), ((((((((-locals.var_q_zero__blk849_dn6) * locals.var_q_d1_zero__blk850) + (assign39410_e44888 * locals.var_q_d1_zero__blk850_dn6)) * locals.var_q_temp__blk860) + (assign39410_e44890 * locals.var_q_temp__blk860_dn6)) * assign39410_e44897) - (assign39410_e44892 * ((locals.var_q_temp__blk860_dn6 * locals.var_q_temp__blk860) + (locals.var_q_temp__blk860 * locals.var_q_temp__blk860_dn6)))) / (assign39410_e44897 * assign39410_e44897)), ((((((((-locals.var_q_zero__blk849_dn7) * locals.var_q_d1_zero__blk850) + (assign39410_e44888 * locals.var_q_d1_zero__blk850_dn7)) * locals.var_q_temp__blk860) + (assign39410_e44890 * locals.var_q_temp__blk860_dn7)) * assign39410_e44897) - (assign39410_e44892 * ((locals.var_q_temp__blk860_dn7 * locals.var_q_temp__blk860) + (locals.var_q_temp__blk860 * locals.var_q_temp__blk860_dn7)))) / (assign39410_e44897 * assign39410_e44897)), ((((((((-locals.var_q_zero__blk849_dn8) * locals.var_q_d1_zero__blk850) + (assign39410_e44888 * locals.var_q_d1_zero__blk850_dn8)) * locals.var_q_temp__blk860) + (assign39410_e44890 * locals.var_q_temp__blk860_dn8)) * assign39410_e44897) - (assign39410_e44892 * ((locals.var_q_temp__blk860_dn8 * locals.var_q_temp__blk860) + (locals.var_q_temp__blk860 * locals.var_q_temp__blk860_dn8)))) / (assign39410_e44897 * assign39410_e44897)), ((((((((-locals.var_q_zero__blk849_dn9) * locals.var_q_d1_zero__blk850) + (assign39410_e44888 * locals.var_q_d1_zero__blk850_dn9)) * locals.var_q_temp__blk860) + (assign39410_e44890 * locals.var_q_temp__blk860_dn9)) * assign39410_e44897) - (assign39410_e44892 * ((locals.var_q_temp__blk860_dn9 * locals.var_q_temp__blk860) + (locals.var_q_temp__blk860 * locals.var_q_temp__blk860_dn9)))) / (assign39410_e44897 * assign39410_e44897)), );
            locals.var_q_eps2__blk852_rv = 0.0;
        }
        if (((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) {
            let assign39420_e44908: f64 = (locals.var_q1d__blk1001 + locals.var_q_eps2__blk852);
            (locals.var_q1d__blk1001, locals.var_q1d__blk1001_dn4, locals.var_q1d__blk1001_dn6, locals.var_q1d__blk1001_dn7, locals.var_q1d__blk1001_dn8, locals.var_q1d__blk1001_dn9, ) = (assign39420_e44908, (locals.var_q1d__blk1001_dn4 + locals.var_q_eps2__blk852_dn4), (locals.var_q1d__blk1001_dn6 + locals.var_q_eps2__blk852_dn6), (locals.var_q1d__blk1001_dn7 + locals.var_q_eps2__blk852_dn7), (locals.var_q1d__blk1001_dn8 + locals.var_q_eps2__blk852_dn8), (locals.var_q1d__blk1001_dn9 + locals.var_q_eps2__blk852_dn9), );
            locals.var_q1d__blk1001_rv = 0.0;
        }
        if (locals.var_guard1080 != 0.0) {
            let assign39430_e44914: f64 = (locals.var_k1__blk932 * locals.var_q1d__blk1001);
            (locals.var_k1q1d__blk1004, locals.var_k1q1d__blk1004_dn4, locals.var_k1q1d__blk1004_dn6, locals.var_k1q1d__blk1004_dn7, locals.var_k1q1d__blk1004_dn8, locals.var_k1q1d__blk1004_dn9, ) = (assign39430_e44914, ((locals.var_k1__blk932_dn4 * locals.var_q1d__blk1001) + (locals.var_k1__blk932 * locals.var_q1d__blk1001_dn4)), ((locals.var_k1__blk932_dn6 * locals.var_q1d__blk1001) + (locals.var_k1__blk932 * locals.var_q1d__blk1001_dn6)), ((locals.var_k1__blk932_dn7 * locals.var_q1d__blk1001) + (locals.var_k1__blk932 * locals.var_q1d__blk1001_dn7)), ((locals.var_k1__blk932_dn8 * locals.var_q1d__blk1001) + (locals.var_k1__blk932 * locals.var_q1d__blk1001_dn8)), ((locals.var_k1__blk932_dn9 * locals.var_q1d__blk1001) + (locals.var_k1__blk932 * locals.var_q1d__blk1001_dn9)), );
            locals.var_k1q1d__blk1004_rv = 0.0;
        }
        let assign39440_e44919: f64 = (locals.var_xg1x__blk930 - locals.var_q1d__blk1001);
        let assign39440_e44921: f64 = (assign39440_e44919 - locals.var_xdeff__blk1000);
        let assign39440_e44923: f64 = if assign39440_e44921 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard1205 = assign39440_e44923;
        locals.var_guard1205_rv = 0.0;
        if ((locals.var_guard1080 != 0.0) && (locals.var_guard1205 != 0.0)) {
            let assign39450_e44929: f64 = (locals.var_xg1x__blk930 - locals.var_q1d__blk1001);
            let assign39450_e44931: f64 = (assign39450_e44929 - locals.var_xdeff__blk1000);
            let assign39450_e44932: f64 = (assign39450_e44931).exp();
            (locals.var_q_temp1__blk814, locals.var_q_temp1__blk814_dn4, locals.var_q_temp1__blk814_dn6, locals.var_q_temp1__blk814_dn7, locals.var_q_temp1__blk814_dn8, locals.var_q_temp1__blk814_dn9, ) = (assign39450_e44932, (assign39450_e44932 * ((locals.var_xg1x__blk930_dn4 - locals.var_q1d__blk1001_dn4) - locals.var_xdeff__blk1000_dn4)), (assign39450_e44932 * ((locals.var_xg1x__blk930_dn6 - locals.var_q1d__blk1001_dn6) - locals.var_xdeff__blk1000_dn6)), (assign39450_e44932 * ((locals.var_xg1x__blk930_dn7 - locals.var_q1d__blk1001_dn7) - locals.var_xdeff__blk1000_dn7)), (assign39450_e44932 * ((locals.var_xg1x__blk930_dn8 - locals.var_q1d__blk1001_dn8) - locals.var_xdeff__blk1000_dn8)), (assign39450_e44932 * ((locals.var_xg1x__blk930_dn9 - locals.var_q1d__blk1001_dn9) - locals.var_xdeff__blk1000_dn9)), );
            locals.var_q_temp1__blk814_rv = 0.0;
        }
        if ((locals.var_guard1080 != 0.0) && (locals.var_guard1205 == 0.0)) {
            let assign39460_e44943: f64 = (locals.var_xg1x__blk930 - locals.var_q1d__blk1001);
            let assign39460_e44945: f64 = (assign39460_e44943 - locals.var_xdeff__blk1000);
            let assign39460_e44947: f64 = (assign39460_e44945 - 80.0);
            let assign39460_e44952: f64 = (locals.var_xg1x__blk930 - locals.var_q1d__blk1001);
            let assign39460_e44954: f64 = (assign39460_e44952 - locals.var_xdeff__blk1000);
            let assign39460_e44956: f64 = (assign39460_e44954 - 80.0);
            let assign39460_e44957: f64 = (0.5 * assign39460_e44956);
            let assign39460_e44961: f64 = (locals.var_xg1x__blk930 - locals.var_q1d__blk1001);
            let assign39460_e44963: f64 = (assign39460_e44961 - locals.var_xdeff__blk1000);
            let assign39460_e44965: f64 = (assign39460_e44963 - 80.0);
            let assign39460_e44967: f64 = (assign39460_e44965 * 0.3333333333333);
            let assign39460_e44968: f64 = (1.0 + assign39460_e44967);
            let assign39460_e44969: f64 = (assign39460_e44957 * assign39460_e44968);
            let assign39460_e44970: f64 = (1.0 + assign39460_e44969);
            let assign39460_e44971: f64 = (assign39460_e44947 * assign39460_e44970);
            let assign39460_e44972: f64 = (1.0 + assign39460_e44971);
            let assign39460_e44973: f64 = (5.54062e34 * assign39460_e44972);
            (locals.var_q_temp1__blk814, locals.var_q_temp1__blk814_dn4, locals.var_q_temp1__blk814_dn6, locals.var_q_temp1__blk814_dn7, locals.var_q_temp1__blk814_dn8, locals.var_q_temp1__blk814_dn9, ) = (assign39460_e44973, (5.54062e34 * ((((locals.var_xg1x__blk930_dn4 - locals.var_q1d__blk1001_dn4) - locals.var_xdeff__blk1000_dn4) * assign39460_e44970) + (assign39460_e44947 * (((0.5 * ((locals.var_xg1x__blk930_dn4 - locals.var_q1d__blk1001_dn4) - locals.var_xdeff__blk1000_dn4)) * assign39460_e44968) + (assign39460_e44957 * (((locals.var_xg1x__blk930_dn4 - locals.var_q1d__blk1001_dn4) - locals.var_xdeff__blk1000_dn4) * 0.3333333333333)))))), (5.54062e34 * ((((locals.var_xg1x__blk930_dn6 - locals.var_q1d__blk1001_dn6) - locals.var_xdeff__blk1000_dn6) * assign39460_e44970) + (assign39460_e44947 * (((0.5 * ((locals.var_xg1x__blk930_dn6 - locals.var_q1d__blk1001_dn6) - locals.var_xdeff__blk1000_dn6)) * assign39460_e44968) + (assign39460_e44957 * (((locals.var_xg1x__blk930_dn6 - locals.var_q1d__blk1001_dn6) - locals.var_xdeff__blk1000_dn6) * 0.3333333333333)))))), (5.54062e34 * ((((locals.var_xg1x__blk930_dn7 - locals.var_q1d__blk1001_dn7) - locals.var_xdeff__blk1000_dn7) * assign39460_e44970) + (assign39460_e44947 * (((0.5 * ((locals.var_xg1x__blk930_dn7 - locals.var_q1d__blk1001_dn7) - locals.var_xdeff__blk1000_dn7)) * assign39460_e44968) + (assign39460_e44957 * (((locals.var_xg1x__blk930_dn7 - locals.var_q1d__blk1001_dn7) - locals.var_xdeff__blk1000_dn7) * 0.3333333333333)))))), (5.54062e34 * ((((locals.var_xg1x__blk930_dn8 - locals.var_q1d__blk1001_dn8) - locals.var_xdeff__blk1000_dn8) * assign39460_e44970) + (assign39460_e44947 * (((0.5 * ((locals.var_xg1x__blk930_dn8 - locals.var_q1d__blk1001_dn8) - locals.var_xdeff__blk1000_dn8)) * assign39460_e44968) + (assign39460_e44957 * (((locals.var_xg1x__blk930_dn8 - locals.var_q1d__blk1001_dn8) - locals.var_xdeff__blk1000_dn8) * 0.3333333333333)))))), (5.54062e34 * ((((locals.var_xg1x__blk930_dn9 - locals.var_q1d__blk1001_dn9) - locals.var_xdeff__blk1000_dn9) * assign39460_e44970) + (assign39460_e44947 * (((0.5 * ((locals.var_xg1x__blk930_dn9 - locals.var_q1d__blk1001_dn9) - locals.var_xdeff__blk1000_dn9)) * assign39460_e44968) + (assign39460_e44957 * (((locals.var_xg1x__blk930_dn9 - locals.var_q1d__blk1001_dn9) - locals.var_xdeff__blk1000_dn9) * 0.3333333333333)))))), );
            locals.var_q_temp1__blk814_rv = 0.0;
        }
        if (locals.var_guard1080 != 0.0) {
            let assign39470_e44979: f64 = (locals.var_a0__blk905 * locals.var_q_temp1__blk814);
            (locals.var_aexp1d__blk1007, locals.var_aexp1d__blk1007_dn4, locals.var_aexp1d__blk1007_dn6, locals.var_aexp1d__blk1007_dn7, locals.var_aexp1d__blk1007_dn8, locals.var_aexp1d__blk1007_dn9, ) = (assign39470_e44979, ((locals.var_a0__blk905_dn4 * locals.var_q_temp1__blk814) + (locals.var_a0__blk905 * locals.var_q_temp1__blk814_dn4)), ((locals.var_a0__blk905_dn6 * locals.var_q_temp1__blk814) + (locals.var_a0__blk905 * locals.var_q_temp1__blk814_dn6)), ((locals.var_a0__blk905_dn7 * locals.var_q_temp1__blk814) + (locals.var_a0__blk905 * locals.var_q_temp1__blk814_dn7)), ((locals.var_a0__blk905_dn8 * locals.var_q_temp1__blk814) + (locals.var_a0__blk905 * locals.var_q_temp1__blk814_dn8)), ((locals.var_a0__blk905_dn9 * locals.var_q_temp1__blk814) + (locals.var_a0__blk905 * locals.var_q_temp1__blk814_dn9)), );
            locals.var_aexp1d__blk1007_rv = 0.0;
        }
        if (locals.var_guard1080 != 0.0) {
            let assign39480_e44985: f64 = (locals.var_k1q1d__blk1004 * locals.var_k1q1d__blk1004);
            let assign39480_e44987: f64 = (assign39480_e44985 - locals.var_aexp1d__blk1007);
            (locals.var_qsqd__blk1006, locals.var_qsqd__blk1006_dn4, locals.var_qsqd__blk1006_dn6, locals.var_qsqd__blk1006_dn7, locals.var_qsqd__blk1006_dn8, locals.var_qsqd__blk1006_dn9, ) = (assign39480_e44987, (((locals.var_k1q1d__blk1004_dn4 * locals.var_k1q1d__blk1004) + (locals.var_k1q1d__blk1004 * locals.var_k1q1d__blk1004_dn4)) - locals.var_aexp1d__blk1007_dn4), (((locals.var_k1q1d__blk1004_dn6 * locals.var_k1q1d__blk1004) + (locals.var_k1q1d__blk1004 * locals.var_k1q1d__blk1004_dn6)) - locals.var_aexp1d__blk1007_dn6), (((locals.var_k1q1d__blk1004_dn7 * locals.var_k1q1d__blk1004) + (locals.var_k1q1d__blk1004 * locals.var_k1q1d__blk1004_dn7)) - locals.var_aexp1d__blk1007_dn7), (((locals.var_k1q1d__blk1004_dn8 * locals.var_k1q1d__blk1004) + (locals.var_k1q1d__blk1004 * locals.var_k1q1d__blk1004_dn8)) - locals.var_aexp1d__blk1007_dn8), (((locals.var_k1q1d__blk1004_dn9 * locals.var_k1q1d__blk1004) + (locals.var_k1q1d__blk1004 * locals.var_k1q1d__blk1004_dn9)) - locals.var_aexp1d__blk1007_dn9), );
            locals.var_qsqd__blk1006_rv = 0.0;
        }
        let assign39490_e44992: f64 = if locals.var_aexp1d__blk1007 <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1206 = assign39490_e44992;
        locals.var_guard1206_rv = 0.0;
        if ((locals.var_guard1080 != 0.0) && (locals.var_guard1206 != 0.0)) {
            (locals.var_qid__blk1003, locals.var_qid__blk1003_dn4, locals.var_qid__blk1003_dn6, locals.var_qid__blk1003_dn7, locals.var_qid__blk1003_dn8, locals.var_qid__blk1003_dn9, ) = (1e-80, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_qid__blk1003_rv = 0.0;
        }
        if ((locals.var_guard1080 != 0.0) && (locals.var_guard1206 != 0.0)) {
            let assign39510_e45004: f64 = (locals.var_qid__blk1003 - locals.var_k1q1d__blk1004);
            (locals.var_k2q2d__blk1005, locals.var_k2q2d__blk1005_dn4, locals.var_k2q2d__blk1005_dn6, locals.var_k2q2d__blk1005_dn7, locals.var_k2q2d__blk1005_dn8, locals.var_k2q2d__blk1005_dn9, ) = (assign39510_e45004, (locals.var_qid__blk1003_dn4 - locals.var_k1q1d__blk1004_dn4), (locals.var_qid__blk1003_dn6 - locals.var_k1q1d__blk1004_dn6), (locals.var_qid__blk1003_dn7 - locals.var_k1q1d__blk1004_dn7), (locals.var_qid__blk1003_dn8 - locals.var_k1q1d__blk1004_dn8), (locals.var_qid__blk1003_dn9 - locals.var_k1q1d__blk1004_dn9), );
            locals.var_k2q2d__blk1005_rv = 0.0;
        }
        if ((locals.var_guard1080 != 0.0) && (locals.var_guard1206 != 0.0)) {
            let assign39520_e45012: f64 = (locals.var_k2q2d__blk1005 / locals.var_k2__blk933);
            (locals.var_q2d__blk1002, locals.var_q2d__blk1002_dn4, locals.var_q2d__blk1002_dn6, locals.var_q2d__blk1002_dn7, locals.var_q2d__blk1002_dn8, locals.var_q2d__blk1002_dn9, ) = (assign39520_e45012, (((locals.var_k2q2d__blk1005_dn4 * locals.var_k2__blk933) - (locals.var_k2q2d__blk1005 * locals.var_k2__blk933_dn4)) / (locals.var_k2__blk933 * locals.var_k2__blk933)), (((locals.var_k2q2d__blk1005_dn6 * locals.var_k2__blk933) - (locals.var_k2q2d__blk1005 * locals.var_k2__blk933_dn6)) / (locals.var_k2__blk933 * locals.var_k2__blk933)), (((locals.var_k2q2d__blk1005_dn7 * locals.var_k2__blk933) - (locals.var_k2q2d__blk1005 * locals.var_k2__blk933_dn7)) / (locals.var_k2__blk933 * locals.var_k2__blk933)), (((locals.var_k2q2d__blk1005_dn8 * locals.var_k2__blk933) - (locals.var_k2q2d__blk1005 * locals.var_k2__blk933_dn8)) / (locals.var_k2__blk933 * locals.var_k2__blk933)), (((locals.var_k2q2d__blk1005_dn9 * locals.var_k2__blk933) - (locals.var_k2q2d__blk1005 * locals.var_k2__blk933_dn9)) / (locals.var_k2__blk933 * locals.var_k2__blk933)), );
            locals.var_q2d__blk1002_rv = 0.0;
        }
        let assign39530_e45017: f64 = (-0.005);
        let assign39530_e45018: f64 = if locals.var_qsqd__blk1006 < assign39530_e45017 { 1.0 } else { 0.0 };
        locals.var_guard1207 = assign39530_e45018;
        locals.var_guard1207_rv = 0.0;
        if (((locals.var_guard1080 != 0.0) && (locals.var_guard1206 == 0.0)) && (locals.var_guard1207 != 0.0)) {
            let assign39540_e45026: f64 = (locals.var_qsqd__blk1006).abs();
            let assign39540_e45027: f64 = (assign39540_e45026).sqrt();
            (locals.var_q_rac_qsq__blk828, locals.var_q_rac_qsq__blk828_dn4, locals.var_q_rac_qsq__blk828_dn6, locals.var_q_rac_qsq__blk828_dn7, locals.var_q_rac_qsq__blk828_dn8, locals.var_q_rac_qsq__blk828_dn9, ) = (assign39540_e45027, (if locals.var_qsqd__blk1006 >= 0.0 { locals.var_qsqd__blk1006_dn4 } else { (-locals.var_qsqd__blk1006_dn4) } / (2.0 * assign39540_e45027)), (if locals.var_qsqd__blk1006 >= 0.0 { locals.var_qsqd__blk1006_dn6 } else { (-locals.var_qsqd__blk1006_dn6) } / (2.0 * assign39540_e45027)), (if locals.var_qsqd__blk1006 >= 0.0 { locals.var_qsqd__blk1006_dn7 } else { (-locals.var_qsqd__blk1006_dn7) } / (2.0 * assign39540_e45027)), (if locals.var_qsqd__blk1006 >= 0.0 { locals.var_qsqd__blk1006_dn8 } else { (-locals.var_qsqd__blk1006_dn8) } / (2.0 * assign39540_e45027)), (if locals.var_qsqd__blk1006 >= 0.0 { locals.var_qsqd__blk1006_dn9 } else { (-locals.var_qsqd__blk1006_dn9) } / (2.0 * assign39540_e45027)), );
            locals.var_q_rac_qsq__blk828_rv = 0.0;
        }
        if (((locals.var_guard1080 != 0.0) && (locals.var_guard1206 == 0.0)) && (locals.var_guard1207 != 0.0)) {
            let assign39550_e45039: f64 = (0.5 * locals.var_q_rac_qsq__blk828);
            let assign39550_e45040: f64 = (assign39550_e45039).tan();
            let assign39550_e45041: f64 = (locals.var_q_rac_qsq__blk828 / assign39550_e45040);
            (locals.var_q_qcoth__blk829, locals.var_q_qcoth__blk829_dn4, locals.var_q_qcoth__blk829_dn6, locals.var_q_qcoth__blk829_dn7, locals.var_q_qcoth__blk829_dn8, locals.var_q_qcoth__blk829_dn9, ) = (assign39550_e45041, (((locals.var_q_rac_qsq__blk828_dn4 * assign39550_e45040) - (locals.var_q_rac_qsq__blk828 * ((0.5 * locals.var_q_rac_qsq__blk828_dn4) / ((assign39550_e45039).cos() * (assign39550_e45039).cos())))) / (assign39550_e45040 * assign39550_e45040)), (((locals.var_q_rac_qsq__blk828_dn6 * assign39550_e45040) - (locals.var_q_rac_qsq__blk828 * ((0.5 * locals.var_q_rac_qsq__blk828_dn6) / ((assign39550_e45039).cos() * (assign39550_e45039).cos())))) / (assign39550_e45040 * assign39550_e45040)), (((locals.var_q_rac_qsq__blk828_dn7 * assign39550_e45040) - (locals.var_q_rac_qsq__blk828 * ((0.5 * locals.var_q_rac_qsq__blk828_dn7) / ((assign39550_e45039).cos() * (assign39550_e45039).cos())))) / (assign39550_e45040 * assign39550_e45040)), (((locals.var_q_rac_qsq__blk828_dn8 * assign39550_e45040) - (locals.var_q_rac_qsq__blk828 * ((0.5 * locals.var_q_rac_qsq__blk828_dn8) / ((assign39550_e45039).cos() * (assign39550_e45039).cos())))) / (assign39550_e45040 * assign39550_e45040)), (((locals.var_q_rac_qsq__blk828_dn9 * assign39550_e45040) - (locals.var_q_rac_qsq__blk828 * ((0.5 * locals.var_q_rac_qsq__blk828_dn9) / ((assign39550_e45039).cos() * (assign39550_e45039).cos())))) / (assign39550_e45040 * assign39550_e45040)), );
            locals.var_q_qcoth__blk829_rv = 0.0;
        }
        let assign39560_e45046: f64 = if locals.var_qsqd__blk1006 > 0.005 { 1.0 } else { 0.0 };
        locals.var_guard1208 = assign39560_e45046;
        locals.var_guard1208_rv = 0.0;
        if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1206 == 0.0)) && (locals.var_guard1207 == 0.0)) && (locals.var_guard1208 != 0.0)) {
            let assign39570_e45057: f64 = (locals.var_qsqd__blk1006).abs();
            let assign39570_e45058: f64 = (assign39570_e45057).sqrt();
            (locals.var_q_rac_qsq__blk828, locals.var_q_rac_qsq__blk828_dn4, locals.var_q_rac_qsq__blk828_dn6, locals.var_q_rac_qsq__blk828_dn7, locals.var_q_rac_qsq__blk828_dn8, locals.var_q_rac_qsq__blk828_dn9, ) = (assign39570_e45058, (if locals.var_qsqd__blk1006 >= 0.0 { locals.var_qsqd__blk1006_dn4 } else { (-locals.var_qsqd__blk1006_dn4) } / (2.0 * assign39570_e45058)), (if locals.var_qsqd__blk1006 >= 0.0 { locals.var_qsqd__blk1006_dn6 } else { (-locals.var_qsqd__blk1006_dn6) } / (2.0 * assign39570_e45058)), (if locals.var_qsqd__blk1006 >= 0.0 { locals.var_qsqd__blk1006_dn7 } else { (-locals.var_qsqd__blk1006_dn7) } / (2.0 * assign39570_e45058)), (if locals.var_qsqd__blk1006 >= 0.0 { locals.var_qsqd__blk1006_dn8 } else { (-locals.var_qsqd__blk1006_dn8) } / (2.0 * assign39570_e45058)), (if locals.var_qsqd__blk1006 >= 0.0 { locals.var_qsqd__blk1006_dn9 } else { (-locals.var_qsqd__blk1006_dn9) } / (2.0 * assign39570_e45058)), );
            locals.var_q_rac_qsq__blk828_rv = 0.0;
        }
        if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1206 == 0.0)) && (locals.var_guard1207 == 0.0)) && (locals.var_guard1208 != 0.0)) {
            let assign39580_e45071: f64 = (-locals.var_q_rac_qsq__blk828);
            let assign39580_e45072: f64 = (assign39580_e45071).exp();
            (locals.var_q_invexpq__blk831, locals.var_q_invexpq__blk831_dn4, locals.var_q_invexpq__blk831_dn6, locals.var_q_invexpq__blk831_dn7, locals.var_q_invexpq__blk831_dn8, locals.var_q_invexpq__blk831_dn9, ) = (assign39580_e45072, (assign39580_e45072 * (-locals.var_q_rac_qsq__blk828_dn4)), (assign39580_e45072 * (-locals.var_q_rac_qsq__blk828_dn6)), (assign39580_e45072 * (-locals.var_q_rac_qsq__blk828_dn7)), (assign39580_e45072 * (-locals.var_q_rac_qsq__blk828_dn8)), (assign39580_e45072 * (-locals.var_q_rac_qsq__blk828_dn9)), );
            locals.var_q_invexpq__blk831_rv = 0.0;
        }
    }
    pub(super) fn stamp_reactive_block_57(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1206 == 0.0)) && (locals.var_guard1207 == 0.0)) && (locals.var_guard1208 != 0.0)) {
            let assign39590_e45087: f64 = (1.0 + locals.var_q_invexpq__blk831);
            let assign39590_e45088: f64 = (locals.var_q_rac_qsq__blk828 * assign39590_e45087);
            let assign39590_e45091: f64 = (1.0 - locals.var_q_invexpq__blk831);
            let assign39590_e45092: f64 = (assign39590_e45088 / assign39590_e45091);
            (locals.var_q_qcoth__blk829, locals.var_q_qcoth__blk829_dn4, locals.var_q_qcoth__blk829_dn6, locals.var_q_qcoth__blk829_dn7, locals.var_q_qcoth__blk829_dn8, locals.var_q_qcoth__blk829_dn9, ) = (assign39590_e45092, (((((locals.var_q_rac_qsq__blk828_dn4 * assign39590_e45087) + (locals.var_q_rac_qsq__blk828 * locals.var_q_invexpq__blk831_dn4)) * assign39590_e45091) - (assign39590_e45088 * (-locals.var_q_invexpq__blk831_dn4))) / (assign39590_e45091 * assign39590_e45091)), (((((locals.var_q_rac_qsq__blk828_dn6 * assign39590_e45087) + (locals.var_q_rac_qsq__blk828 * locals.var_q_invexpq__blk831_dn6)) * assign39590_e45091) - (assign39590_e45088 * (-locals.var_q_invexpq__blk831_dn6))) / (assign39590_e45091 * assign39590_e45091)), (((((locals.var_q_rac_qsq__blk828_dn7 * assign39590_e45087) + (locals.var_q_rac_qsq__blk828 * locals.var_q_invexpq__blk831_dn7)) * assign39590_e45091) - (assign39590_e45088 * (-locals.var_q_invexpq__blk831_dn7))) / (assign39590_e45091 * assign39590_e45091)), (((((locals.var_q_rac_qsq__blk828_dn8 * assign39590_e45087) + (locals.var_q_rac_qsq__blk828 * locals.var_q_invexpq__blk831_dn8)) * assign39590_e45091) - (assign39590_e45088 * (-locals.var_q_invexpq__blk831_dn8))) / (assign39590_e45091 * assign39590_e45091)), (((((locals.var_q_rac_qsq__blk828_dn9 * assign39590_e45087) + (locals.var_q_rac_qsq__blk828 * locals.var_q_invexpq__blk831_dn9)) * assign39590_e45091) - (assign39590_e45088 * (-locals.var_q_invexpq__blk831_dn9))) / (assign39590_e45091 * assign39590_e45091)), );
            locals.var_q_qcoth__blk829_rv = 0.0;
        }
        if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1206 == 0.0)) && (locals.var_guard1207 == 0.0)) && (locals.var_guard1208 == 0.0)) {
            let assign39600_e45108: f64 = (locals.var_qsqd__blk1006 * 0.1666666666667);
            let assign39600_e45112: f64 = (locals.var_qsqd__blk1006 * 0.0166666666667);
            let assign39600_e45116: f64 = (locals.var_qsqd__blk1006 * 0.0238095238095);
            let assign39600_e45117: f64 = (1.0 - assign39600_e45116);
            let assign39600_e45118: f64 = (assign39600_e45112 * assign39600_e45117);
            let assign39600_e45119: f64 = (1.0 - assign39600_e45118);
            let assign39600_e45120: f64 = (assign39600_e45108 * assign39600_e45119);
            let assign39600_e45121: f64 = (2.0 + assign39600_e45120);
            (locals.var_q_qcoth__blk829, locals.var_q_qcoth__blk829_dn4, locals.var_q_qcoth__blk829_dn6, locals.var_q_qcoth__blk829_dn7, locals.var_q_qcoth__blk829_dn8, locals.var_q_qcoth__blk829_dn9, ) = (assign39600_e45121, (((locals.var_qsqd__blk1006_dn4 * 0.1666666666667) * assign39600_e45119) + (assign39600_e45108 * (-(((locals.var_qsqd__blk1006_dn4 * 0.0166666666667) * assign39600_e45117) + (assign39600_e45112 * (-(locals.var_qsqd__blk1006_dn4 * 0.0238095238095))))))), (((locals.var_qsqd__blk1006_dn6 * 0.1666666666667) * assign39600_e45119) + (assign39600_e45108 * (-(((locals.var_qsqd__blk1006_dn6 * 0.0166666666667) * assign39600_e45117) + (assign39600_e45112 * (-(locals.var_qsqd__blk1006_dn6 * 0.0238095238095))))))), (((locals.var_qsqd__blk1006_dn7 * 0.1666666666667) * assign39600_e45119) + (assign39600_e45108 * (-(((locals.var_qsqd__blk1006_dn7 * 0.0166666666667) * assign39600_e45117) + (assign39600_e45112 * (-(locals.var_qsqd__blk1006_dn7 * 0.0238095238095))))))), (((locals.var_qsqd__blk1006_dn8 * 0.1666666666667) * assign39600_e45119) + (assign39600_e45108 * (-(((locals.var_qsqd__blk1006_dn8 * 0.0166666666667) * assign39600_e45117) + (assign39600_e45112 * (-(locals.var_qsqd__blk1006_dn8 * 0.0238095238095))))))), (((locals.var_qsqd__blk1006_dn9 * 0.1666666666667) * assign39600_e45119) + (assign39600_e45108 * (-(((locals.var_qsqd__blk1006_dn9 * 0.0166666666667) * assign39600_e45117) + (assign39600_e45112 * (-(locals.var_qsqd__blk1006_dn9 * 0.0238095238095))))))), );
            locals.var_q_qcoth__blk829_rv = 0.0;
        }
        let assign39610_e45126: f64 = (1.01 * locals.var_k1q1d__blk1004);
        let assign39610_e45128: f64 = (assign39610_e45126 + locals.var_q_qcoth__blk829);
        let assign39610_e45130: f64 = if assign39610_e45128 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1209 = assign39610_e45130;
        locals.var_guard1209_rv = 0.0;
        if (((locals.var_guard1080 != 0.0) && (locals.var_guard1206 == 0.0)) && (locals.var_guard1209 != 0.0)) {
            let assign39620_e45139: f64 = (locals.var_k1q1d__blk1004 + locals.var_q_qcoth__blk829);
            (locals.var_q_temp1__blk814, locals.var_q_temp1__blk814_dn4, locals.var_q_temp1__blk814_dn6, locals.var_q_temp1__blk814_dn7, locals.var_q_temp1__blk814_dn8, locals.var_q_temp1__blk814_dn9, ) = (assign39620_e45139, (locals.var_k1q1d__blk1004_dn4 + locals.var_q_qcoth__blk829_dn4), (locals.var_k1q1d__blk1004_dn6 + locals.var_q_qcoth__blk829_dn6), (locals.var_k1q1d__blk1004_dn7 + locals.var_q_qcoth__blk829_dn7), (locals.var_k1q1d__blk1004_dn8 + locals.var_q_qcoth__blk829_dn8), (locals.var_k1q1d__blk1004_dn9 + locals.var_q_qcoth__blk829_dn9), );
            locals.var_q_temp1__blk814_rv = 0.0;
        }
        let assign39630_e45144: f64 = (locals.var_aexp1d__blk1007 * locals.var_k1q1d__blk1004);
        let assign39630_e45147: f64 = (0.9 * locals.var_k1q1d__blk1004);
        let assign39630_e45149: f64 = (assign39630_e45147 * locals.var_k1q1d__blk1004);
        let assign39630_e45151: f64 = (assign39630_e45149 * locals.var_q_temp1__blk814);
        let assign39630_e45152: f64 = if assign39630_e45144 < assign39630_e45151 { 1.0 } else { 0.0 };
        locals.var_guard1210 = assign39630_e45152;
        locals.var_guard1210_rv = 0.0;
        if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1206 == 0.0)) && (locals.var_guard1209 != 0.0)) && (locals.var_guard1210 != 0.0)) {
            let assign39640_e45163: f64 = (locals.var_aexp1d__blk1007 / locals.var_q_temp1__blk814);
            let assign39640_e45165: f64 = (assign39640_e45163 + 1e-80);
            (locals.var_qid__blk1003, locals.var_qid__blk1003_dn4, locals.var_qid__blk1003_dn6, locals.var_qid__blk1003_dn7, locals.var_qid__blk1003_dn8, locals.var_qid__blk1003_dn9, ) = (assign39640_e45165, (((locals.var_aexp1d__blk1007_dn4 * locals.var_q_temp1__blk814) - (locals.var_aexp1d__blk1007 * locals.var_q_temp1__blk814_dn4)) / (locals.var_q_temp1__blk814 * locals.var_q_temp1__blk814)), (((locals.var_aexp1d__blk1007_dn6 * locals.var_q_temp1__blk814) - (locals.var_aexp1d__blk1007 * locals.var_q_temp1__blk814_dn6)) / (locals.var_q_temp1__blk814 * locals.var_q_temp1__blk814)), (((locals.var_aexp1d__blk1007_dn7 * locals.var_q_temp1__blk814) - (locals.var_aexp1d__blk1007 * locals.var_q_temp1__blk814_dn7)) / (locals.var_q_temp1__blk814 * locals.var_q_temp1__blk814)), (((locals.var_aexp1d__blk1007_dn8 * locals.var_q_temp1__blk814) - (locals.var_aexp1d__blk1007 * locals.var_q_temp1__blk814_dn8)) / (locals.var_q_temp1__blk814 * locals.var_q_temp1__blk814)), (((locals.var_aexp1d__blk1007_dn9 * locals.var_q_temp1__blk814) - (locals.var_aexp1d__blk1007 * locals.var_q_temp1__blk814_dn9)) / (locals.var_q_temp1__blk814 * locals.var_q_temp1__blk814)), );
            locals.var_qid__blk1003_rv = 0.0;
        }
        if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1206 == 0.0)) && (locals.var_guard1209 != 0.0)) && (locals.var_guard1210 != 0.0)) {
            let assign39650_e45178: f64 = (locals.var_qid__blk1003 - locals.var_k1q1d__blk1004);
            (locals.var_k2q2d__blk1005, locals.var_k2q2d__blk1005_dn4, locals.var_k2q2d__blk1005_dn6, locals.var_k2q2d__blk1005_dn7, locals.var_k2q2d__blk1005_dn8, locals.var_k2q2d__blk1005_dn9, ) = (assign39650_e45178, (locals.var_qid__blk1003_dn4 - locals.var_k1q1d__blk1004_dn4), (locals.var_qid__blk1003_dn6 - locals.var_k1q1d__blk1004_dn6), (locals.var_qid__blk1003_dn7 - locals.var_k1q1d__blk1004_dn7), (locals.var_qid__blk1003_dn8 - locals.var_k1q1d__blk1004_dn8), (locals.var_qid__blk1003_dn9 - locals.var_k1q1d__blk1004_dn9), );
            locals.var_k2q2d__blk1005_rv = 0.0;
        }
        if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1206 == 0.0)) && (locals.var_guard1209 != 0.0)) && (locals.var_guard1210 != 0.0)) {
            let assign39660_e45191: f64 = (locals.var_k2q2d__blk1005 / locals.var_k2__blk933);
            (locals.var_q2d__blk1002, locals.var_q2d__blk1002_dn4, locals.var_q2d__blk1002_dn6, locals.var_q2d__blk1002_dn7, locals.var_q2d__blk1002_dn8, locals.var_q2d__blk1002_dn9, ) = (assign39660_e45191, (((locals.var_k2q2d__blk1005_dn4 * locals.var_k2__blk933) - (locals.var_k2q2d__blk1005 * locals.var_k2__blk933_dn4)) / (locals.var_k2__blk933 * locals.var_k2__blk933)), (((locals.var_k2q2d__blk1005_dn6 * locals.var_k2__blk933) - (locals.var_k2q2d__blk1005 * locals.var_k2__blk933_dn6)) / (locals.var_k2__blk933 * locals.var_k2__blk933)), (((locals.var_k2q2d__blk1005_dn7 * locals.var_k2__blk933) - (locals.var_k2q2d__blk1005 * locals.var_k2__blk933_dn7)) / (locals.var_k2__blk933 * locals.var_k2__blk933)), (((locals.var_k2q2d__blk1005_dn8 * locals.var_k2__blk933) - (locals.var_k2q2d__blk1005 * locals.var_k2__blk933_dn8)) / (locals.var_k2__blk933 * locals.var_k2__blk933)), (((locals.var_k2q2d__blk1005_dn9 * locals.var_k2__blk933) - (locals.var_k2q2d__blk1005 * locals.var_k2__blk933_dn9)) / (locals.var_k2__blk933 * locals.var_k2__blk933)), );
            locals.var_q2d__blk1002_rv = 0.0;
        }
        let assign39670_e45196: f64 = if locals.var_qsqd__blk1006 > 0.005 { 1.0 } else { 0.0 };
        locals.var_guard1211 = assign39670_e45196;
        locals.var_guard1211_rv = 0.0;
        if (((((locals.var_guard1080 != 0.0) && (locals.var_guard1206 == 0.0)) && (locals.var_guard1209 != 0.0)) && (locals.var_guard1210 == 0.0)) && (locals.var_guard1211 != 0.0)) {
            let assign39680_e45210: f64 = (4.0 * locals.var_qsqd__blk1006);
            let assign39680_e45215: f64 = (2.0 - locals.var_q_invexpq__blk831);
            let assign39680_e45216: f64 = (locals.var_q_invexpq__blk831 * assign39680_e45215);
            let assign39680_e45217: f64 = (1.0 - assign39680_e45216);
            let assign39680_e45218: f64 = (assign39680_e45210 / assign39680_e45217);
            let assign39680_e45219: f64 = (assign39680_e45218).ln();
            let assign39680_e45221: f64 = (assign39680_e45219 - locals.var_q_rac_qsq__blk828);
            (locals.var_q_temp2__blk815, locals.var_q_temp2__blk815_dn4, locals.var_q_temp2__blk815_dn6, locals.var_q_temp2__blk815_dn7, locals.var_q_temp2__blk815_dn8, locals.var_q_temp2__blk815_dn9, ) = (assign39680_e45221, ((((((4.0 * locals.var_qsqd__blk1006_dn4) * assign39680_e45217) - (assign39680_e45210 * (-((locals.var_q_invexpq__blk831_dn4 * assign39680_e45215) + (locals.var_q_invexpq__blk831 * (-locals.var_q_invexpq__blk831_dn4)))))) / (assign39680_e45217 * assign39680_e45217)) / assign39680_e45218) - locals.var_q_rac_qsq__blk828_dn4), ((((((4.0 * locals.var_qsqd__blk1006_dn6) * assign39680_e45217) - (assign39680_e45210 * (-((locals.var_q_invexpq__blk831_dn6 * assign39680_e45215) + (locals.var_q_invexpq__blk831 * (-locals.var_q_invexpq__blk831_dn6)))))) / (assign39680_e45217 * assign39680_e45217)) / assign39680_e45218) - locals.var_q_rac_qsq__blk828_dn6), ((((((4.0 * locals.var_qsqd__blk1006_dn7) * assign39680_e45217) - (assign39680_e45210 * (-((locals.var_q_invexpq__blk831_dn7 * assign39680_e45215) + (locals.var_q_invexpq__blk831 * (-locals.var_q_invexpq__blk831_dn7)))))) / (assign39680_e45217 * assign39680_e45217)) / assign39680_e45218) - locals.var_q_rac_qsq__blk828_dn7), ((((((4.0 * locals.var_qsqd__blk1006_dn8) * assign39680_e45217) - (assign39680_e45210 * (-((locals.var_q_invexpq__blk831_dn8 * assign39680_e45215) + (locals.var_q_invexpq__blk831 * (-locals.var_q_invexpq__blk831_dn8)))))) / (assign39680_e45217 * assign39680_e45217)) / assign39680_e45218) - locals.var_q_rac_qsq__blk828_dn8), ((((((4.0 * locals.var_qsqd__blk1006_dn9) * assign39680_e45217) - (assign39680_e45210 * (-((locals.var_q_invexpq__blk831_dn9 * assign39680_e45215) + (locals.var_q_invexpq__blk831 * (-locals.var_q_invexpq__blk831_dn9)))))) / (assign39680_e45217 * assign39680_e45217)) / assign39680_e45218) - locals.var_q_rac_qsq__blk828_dn9), );
            locals.var_q_temp2__blk815_rv = 0.0;
        }
        let assign39690_e45226: f64 = (-0.005);
        let assign39690_e45227: f64 = if locals.var_qsqd__blk1006 < assign39690_e45226 { 1.0 } else { 0.0 };
        locals.var_guard1212 = assign39690_e45227;
        locals.var_guard1212_rv = 0.0;
        if ((((((locals.var_guard1080 != 0.0) && (locals.var_guard1206 == 0.0)) && (locals.var_guard1209 != 0.0)) && (locals.var_guard1210 == 0.0)) && (locals.var_guard1211 == 0.0)) && (locals.var_guard1212 != 0.0)) {
            let assign39700_e45244: f64 = (0.5 * locals.var_q_rac_qsq__blk828);
            let assign39700_e45245: f64 = (assign39700_e45244).sin();
            (locals.var_q_temp3__blk816, locals.var_q_temp3__blk816_dn4, locals.var_q_temp3__blk816_dn6, locals.var_q_temp3__blk816_dn7, locals.var_q_temp3__blk816_dn8, locals.var_q_temp3__blk816_dn9, ) = (assign39700_e45245, ((assign39700_e45244).cos() * (0.5 * locals.var_q_rac_qsq__blk828_dn4)), ((assign39700_e45244).cos() * (0.5 * locals.var_q_rac_qsq__blk828_dn6)), ((assign39700_e45244).cos() * (0.5 * locals.var_q_rac_qsq__blk828_dn7)), ((assign39700_e45244).cos() * (0.5 * locals.var_q_rac_qsq__blk828_dn8)), ((assign39700_e45244).cos() * (0.5 * locals.var_q_rac_qsq__blk828_dn9)), );
            locals.var_q_temp3__blk816_rv = 0.0;
        }
        if ((((((locals.var_guard1080 != 0.0) && (locals.var_guard1206 == 0.0)) && (locals.var_guard1209 != 0.0)) && (locals.var_guard1210 == 0.0)) && (locals.var_guard1211 == 0.0)) && (locals.var_guard1212 != 0.0)) {
            let assign39710_e45263: f64 = (-locals.var_qsqd__blk1006);
            let assign39710_e45266: f64 = (locals.var_q_temp3__blk816 * locals.var_q_temp3__blk816);
            let assign39710_e45267: f64 = (assign39710_e45263 / assign39710_e45266);
            let assign39710_e45268: f64 = (assign39710_e45267).ln();
            (locals.var_q_temp2__blk815, locals.var_q_temp2__blk815_dn4, locals.var_q_temp2__blk815_dn6, locals.var_q_temp2__blk815_dn7, locals.var_q_temp2__blk815_dn8, locals.var_q_temp2__blk815_dn9, ) = (assign39710_e45268, (((((-locals.var_qsqd__blk1006_dn4) * assign39710_e45266) - (assign39710_e45263 * ((locals.var_q_temp3__blk816_dn4 * locals.var_q_temp3__blk816) + (locals.var_q_temp3__blk816 * locals.var_q_temp3__blk816_dn4)))) / (assign39710_e45266 * assign39710_e45266)) / assign39710_e45267), (((((-locals.var_qsqd__blk1006_dn6) * assign39710_e45266) - (assign39710_e45263 * ((locals.var_q_temp3__blk816_dn6 * locals.var_q_temp3__blk816) + (locals.var_q_temp3__blk816 * locals.var_q_temp3__blk816_dn6)))) / (assign39710_e45266 * assign39710_e45266)) / assign39710_e45267), (((((-locals.var_qsqd__blk1006_dn7) * assign39710_e45266) - (assign39710_e45263 * ((locals.var_q_temp3__blk816_dn7 * locals.var_q_temp3__blk816) + (locals.var_q_temp3__blk816 * locals.var_q_temp3__blk816_dn7)))) / (assign39710_e45266 * assign39710_e45266)) / assign39710_e45267), (((((-locals.var_qsqd__blk1006_dn8) * assign39710_e45266) - (assign39710_e45263 * ((locals.var_q_temp3__blk816_dn8 * locals.var_q_temp3__blk816) + (locals.var_q_temp3__blk816 * locals.var_q_temp3__blk816_dn8)))) / (assign39710_e45266 * assign39710_e45266)) / assign39710_e45267), (((((-locals.var_qsqd__blk1006_dn9) * assign39710_e45266) - (assign39710_e45263 * ((locals.var_q_temp3__blk816_dn9 * locals.var_q_temp3__blk816) + (locals.var_q_temp3__blk816 * locals.var_q_temp3__blk816_dn9)))) / (assign39710_e45266 * assign39710_e45266)) / assign39710_e45267), );
            locals.var_q_temp2__blk815_rv = 0.0;
        }
        if ((((((locals.var_guard1080 != 0.0) && (locals.var_guard1206 == 0.0)) && (locals.var_guard1209 != 0.0)) && (locals.var_guard1210 == 0.0)) && (locals.var_guard1211 == 0.0)) && (locals.var_guard1212 == 0.0)) {
            let assign39720_e45289: f64 = (locals.var_qsqd__blk1006 * 0.3333333333333);
            let assign39720_e45293: f64 = (0.05 * locals.var_qsqd__blk1006);
            let assign39720_e45297: f64 = (0.0396825396825397 * locals.var_qsqd__blk1006);
            let assign39720_e45298: f64 = (1.0 - assign39720_e45297);
            let assign39720_e45299: f64 = (assign39720_e45293 * assign39720_e45298);
            let assign39720_e45300: f64 = (1.0 - assign39720_e45299);
            let assign39720_e45301: f64 = (assign39720_e45289 * assign39720_e45300);
            let assign39720_e45302: f64 = (4.0 - assign39720_e45301);
            let assign39720_e45303: f64 = (assign39720_e45302).ln();
            (locals.var_q_temp2__blk815, locals.var_q_temp2__blk815_dn4, locals.var_q_temp2__blk815_dn6, locals.var_q_temp2__blk815_dn7, locals.var_q_temp2__blk815_dn8, locals.var_q_temp2__blk815_dn9, ) = (assign39720_e45303, ((-(((locals.var_qsqd__blk1006_dn4 * 0.3333333333333) * assign39720_e45300) + (assign39720_e45289 * (-(((0.05 * locals.var_qsqd__blk1006_dn4) * assign39720_e45298) + (assign39720_e45293 * (-(0.0396825396825397 * locals.var_qsqd__blk1006_dn4)))))))) / assign39720_e45302), ((-(((locals.var_qsqd__blk1006_dn6 * 0.3333333333333) * assign39720_e45300) + (assign39720_e45289 * (-(((0.05 * locals.var_qsqd__blk1006_dn6) * assign39720_e45298) + (assign39720_e45293 * (-(0.0396825396825397 * locals.var_qsqd__blk1006_dn6)))))))) / assign39720_e45302), ((-(((locals.var_qsqd__blk1006_dn7 * 0.3333333333333) * assign39720_e45300) + (assign39720_e45289 * (-(((0.05 * locals.var_qsqd__blk1006_dn7) * assign39720_e45298) + (assign39720_e45293 * (-(0.0396825396825397 * locals.var_qsqd__blk1006_dn7)))))))) / assign39720_e45302), ((-(((locals.var_qsqd__blk1006_dn8 * 0.3333333333333) * assign39720_e45300) + (assign39720_e45289 * (-(((0.05 * locals.var_qsqd__blk1006_dn8) * assign39720_e45298) + (assign39720_e45293 * (-(0.0396825396825397 * locals.var_qsqd__blk1006_dn8)))))))) / assign39720_e45302), ((-(((locals.var_qsqd__blk1006_dn9 * 0.3333333333333) * assign39720_e45300) + (assign39720_e45289 * (-(((0.05 * locals.var_qsqd__blk1006_dn9) * assign39720_e45298) + (assign39720_e45293 * (-(0.0396825396825397 * locals.var_qsqd__blk1006_dn9)))))))) / assign39720_e45302), );
            locals.var_q_temp2__blk815_rv = 0.0;
        }
        if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1206 == 0.0)) && (locals.var_guard1209 != 0.0)) && (locals.var_guard1210 == 0.0)) {
            let assign39730_e45317: f64 = (locals.var_xg2x__blk931 - locals.var_xg1x__blk930);
            let assign39730_e45319: f64 = (assign39730_e45317 + locals.var_q1d__blk1001);
            let assign39730_e45322: f64 = (locals.var_q_temp1__blk814).ln();
            let assign39730_e45323: f64 = (2.0 * assign39730_e45322);
            let assign39730_e45324: f64 = (assign39730_e45319 + assign39730_e45323);
            let assign39730_e45326: f64 = (assign39730_e45324 - locals.var_q_temp2__blk815);
            (locals.var_q2d__blk1002, locals.var_q2d__blk1002_dn4, locals.var_q2d__blk1002_dn6, locals.var_q2d__blk1002_dn7, locals.var_q2d__blk1002_dn8, locals.var_q2d__blk1002_dn9, ) = (assign39730_e45326, ((((locals.var_xg2x__blk931_dn4 - locals.var_xg1x__blk930_dn4) + locals.var_q1d__blk1001_dn4) + (2.0 * (locals.var_q_temp1__blk814_dn4 / locals.var_q_temp1__blk814))) - locals.var_q_temp2__blk815_dn4), ((((locals.var_xg2x__blk931_dn6 - locals.var_xg1x__blk930_dn6) + locals.var_q1d__blk1001_dn6) + (2.0 * (locals.var_q_temp1__blk814_dn6 / locals.var_q_temp1__blk814))) - locals.var_q_temp2__blk815_dn6), ((((locals.var_xg2x__blk931_dn7 - locals.var_xg1x__blk930_dn7) + locals.var_q1d__blk1001_dn7) + (2.0 * (locals.var_q_temp1__blk814_dn7 / locals.var_q_temp1__blk814))) - locals.var_q_temp2__blk815_dn7), ((((locals.var_xg2x__blk931_dn8 - locals.var_xg1x__blk930_dn8) + locals.var_q1d__blk1001_dn8) + (2.0 * (locals.var_q_temp1__blk814_dn8 / locals.var_q_temp1__blk814))) - locals.var_q_temp2__blk815_dn8), ((((locals.var_xg2x__blk931_dn9 - locals.var_xg1x__blk930_dn9) + locals.var_q1d__blk1001_dn9) + (2.0 * (locals.var_q_temp1__blk814_dn9 / locals.var_q_temp1__blk814))) - locals.var_q_temp2__blk815_dn9), );
            locals.var_q2d__blk1002_rv = 0.0;
        }
        if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1206 == 0.0)) && (locals.var_guard1209 != 0.0)) && (locals.var_guard1210 == 0.0)) {
            let assign39740_e45340: f64 = (locals.var_k2__blk933 * locals.var_q2d__blk1002);
            (locals.var_k2q2d__blk1005, locals.var_k2q2d__blk1005_dn4, locals.var_k2q2d__blk1005_dn6, locals.var_k2q2d__blk1005_dn7, locals.var_k2q2d__blk1005_dn8, locals.var_k2q2d__blk1005_dn9, ) = (assign39740_e45340, ((locals.var_k2__blk933_dn4 * locals.var_q2d__blk1002) + (locals.var_k2__blk933 * locals.var_q2d__blk1002_dn4)), ((locals.var_k2__blk933_dn6 * locals.var_q2d__blk1002) + (locals.var_k2__blk933 * locals.var_q2d__blk1002_dn6)), ((locals.var_k2__blk933_dn7 * locals.var_q2d__blk1002) + (locals.var_k2__blk933 * locals.var_q2d__blk1002_dn7)), ((locals.var_k2__blk933_dn8 * locals.var_q2d__blk1002) + (locals.var_k2__blk933 * locals.var_q2d__blk1002_dn8)), ((locals.var_k2__blk933_dn9 * locals.var_q2d__blk1002) + (locals.var_k2__blk933 * locals.var_q2d__blk1002_dn9)), );
            locals.var_k2q2d__blk1005_rv = 0.0;
        }
        if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1206 == 0.0)) && (locals.var_guard1209 != 0.0)) && (locals.var_guard1210 == 0.0)) {
            let assign39750_e45354: f64 = (locals.var_k1q1d__blk1004 + locals.var_k2q2d__blk1005);
            (locals.var_qid__blk1003, locals.var_qid__blk1003_dn4, locals.var_qid__blk1003_dn6, locals.var_qid__blk1003_dn7, locals.var_qid__blk1003_dn8, locals.var_qid__blk1003_dn9, ) = (assign39750_e45354, (locals.var_k1q1d__blk1004_dn4 + locals.var_k2q2d__blk1005_dn4), (locals.var_k1q1d__blk1004_dn6 + locals.var_k2q2d__blk1005_dn6), (locals.var_k1q1d__blk1004_dn7 + locals.var_k2q2d__blk1005_dn7), (locals.var_k1q1d__blk1004_dn8 + locals.var_k2q2d__blk1005_dn8), (locals.var_k1q1d__blk1004_dn9 + locals.var_k2q2d__blk1005_dn9), );
            locals.var_qid__blk1003_rv = 0.0;
        }
        let assign39760_e45359: f64 = if locals.var_qsqd__blk1006 > 0.005 { 1.0 } else { 0.0 };
        locals.var_guard1213 = assign39760_e45359;
        locals.var_guard1213_rv = 0.0;
        let assign39770_e45362: f64 = (locals.var_q1d__blk1001 + locals.var_xdeff__blk1000);
        let assign39770_e45364: f64 = (assign39770_e45362 - locals.var_xg1x__blk930);
        let assign39770_e45366: f64 = (assign39770_e45364 - locals.var_q_rac_qsq__blk828);
        let assign39770_e45368: f64 = if assign39770_e45366 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard1214 = assign39770_e45368;
        locals.var_guard1214_rv = 0.0;
        if (((((locals.var_guard1080 != 0.0) && (locals.var_guard1206 == 0.0)) && (locals.var_guard1209 == 0.0)) && (locals.var_guard1213 != 0.0)) && (locals.var_guard1214 != 0.0)) {
            let assign39780_e45382: f64 = (locals.var_q1d__blk1001 + locals.var_xdeff__blk1000);
            let assign39780_e45384: f64 = (assign39780_e45382 - locals.var_xg1x__blk930);
            let assign39780_e45386: f64 = (assign39780_e45384 - locals.var_q_rac_qsq__blk828);
            let assign39780_e45387: f64 = (assign39780_e45386).exp();
            (locals.var_q_temp3__blk816, locals.var_q_temp3__blk816_dn4, locals.var_q_temp3__blk816_dn6, locals.var_q_temp3__blk816_dn7, locals.var_q_temp3__blk816_dn8, locals.var_q_temp3__blk816_dn9, ) = (assign39780_e45387, (assign39780_e45387 * (((locals.var_q1d__blk1001_dn4 + locals.var_xdeff__blk1000_dn4) - locals.var_xg1x__blk930_dn4) - locals.var_q_rac_qsq__blk828_dn4)), (assign39780_e45387 * (((locals.var_q1d__blk1001_dn6 + locals.var_xdeff__blk1000_dn6) - locals.var_xg1x__blk930_dn6) - locals.var_q_rac_qsq__blk828_dn6)), (assign39780_e45387 * (((locals.var_q1d__blk1001_dn7 + locals.var_xdeff__blk1000_dn7) - locals.var_xg1x__blk930_dn7) - locals.var_q_rac_qsq__blk828_dn7)), (assign39780_e45387 * (((locals.var_q1d__blk1001_dn8 + locals.var_xdeff__blk1000_dn8) - locals.var_xg1x__blk930_dn8) - locals.var_q_rac_qsq__blk828_dn8)), (assign39780_e45387 * (((locals.var_q1d__blk1001_dn9 + locals.var_xdeff__blk1000_dn9) - locals.var_xg1x__blk930_dn9) - locals.var_q_rac_qsq__blk828_dn9)), );
            locals.var_q_temp3__blk816_rv = 0.0;
        }
        if (((((locals.var_guard1080 != 0.0) && (locals.var_guard1206 == 0.0)) && (locals.var_guard1209 == 0.0)) && (locals.var_guard1213 != 0.0)) && (locals.var_guard1214 == 0.0)) {
            let assign39790_e45406: f64 = (locals.var_q1d__blk1001 + locals.var_xdeff__blk1000);
            let assign39790_e45408: f64 = (assign39790_e45406 - locals.var_xg1x__blk930);
            let assign39790_e45410: f64 = (assign39790_e45408 - locals.var_q_rac_qsq__blk828);
            let assign39790_e45412: f64 = (assign39790_e45410 - 80.0);
            let assign39790_e45417: f64 = (locals.var_q1d__blk1001 + locals.var_xdeff__blk1000);
            let assign39790_e45419: f64 = (assign39790_e45417 - locals.var_xg1x__blk930);
            let assign39790_e45421: f64 = (assign39790_e45419 - locals.var_q_rac_qsq__blk828);
            let assign39790_e45423: f64 = (assign39790_e45421 - 80.0);
            let assign39790_e45424: f64 = (0.5 * assign39790_e45423);
            let assign39790_e45428: f64 = (locals.var_q1d__blk1001 + locals.var_xdeff__blk1000);
            let assign39790_e45430: f64 = (assign39790_e45428 - locals.var_xg1x__blk930);
            let assign39790_e45432: f64 = (assign39790_e45430 - locals.var_q_rac_qsq__blk828);
            let assign39790_e45434: f64 = (assign39790_e45432 - 80.0);
            let assign39790_e45436: f64 = (assign39790_e45434 * 0.3333333333333);
            let assign39790_e45437: f64 = (1.0 + assign39790_e45436);
            let assign39790_e45438: f64 = (assign39790_e45424 * assign39790_e45437);
            let assign39790_e45439: f64 = (1.0 + assign39790_e45438);
            let assign39790_e45440: f64 = (assign39790_e45412 * assign39790_e45439);
            let assign39790_e45441: f64 = (1.0 + assign39790_e45440);
            let assign39790_e45442: f64 = (5.54062e34 * assign39790_e45441);
            (locals.var_q_temp3__blk816, locals.var_q_temp3__blk816_dn4, locals.var_q_temp3__blk816_dn6, locals.var_q_temp3__blk816_dn7, locals.var_q_temp3__blk816_dn8, locals.var_q_temp3__blk816_dn9, ) = (assign39790_e45442, (5.54062e34 * (((((locals.var_q1d__blk1001_dn4 + locals.var_xdeff__blk1000_dn4) - locals.var_xg1x__blk930_dn4) - locals.var_q_rac_qsq__blk828_dn4) * assign39790_e45439) + (assign39790_e45412 * (((0.5 * (((locals.var_q1d__blk1001_dn4 + locals.var_xdeff__blk1000_dn4) - locals.var_xg1x__blk930_dn4) - locals.var_q_rac_qsq__blk828_dn4)) * assign39790_e45437) + (assign39790_e45424 * ((((locals.var_q1d__blk1001_dn4 + locals.var_xdeff__blk1000_dn4) - locals.var_xg1x__blk930_dn4) - locals.var_q_rac_qsq__blk828_dn4) * 0.3333333333333)))))), (5.54062e34 * (((((locals.var_q1d__blk1001_dn6 + locals.var_xdeff__blk1000_dn6) - locals.var_xg1x__blk930_dn6) - locals.var_q_rac_qsq__blk828_dn6) * assign39790_e45439) + (assign39790_e45412 * (((0.5 * (((locals.var_q1d__blk1001_dn6 + locals.var_xdeff__blk1000_dn6) - locals.var_xg1x__blk930_dn6) - locals.var_q_rac_qsq__blk828_dn6)) * assign39790_e45437) + (assign39790_e45424 * ((((locals.var_q1d__blk1001_dn6 + locals.var_xdeff__blk1000_dn6) - locals.var_xg1x__blk930_dn6) - locals.var_q_rac_qsq__blk828_dn6) * 0.3333333333333)))))), (5.54062e34 * (((((locals.var_q1d__blk1001_dn7 + locals.var_xdeff__blk1000_dn7) - locals.var_xg1x__blk930_dn7) - locals.var_q_rac_qsq__blk828_dn7) * assign39790_e45439) + (assign39790_e45412 * (((0.5 * (((locals.var_q1d__blk1001_dn7 + locals.var_xdeff__blk1000_dn7) - locals.var_xg1x__blk930_dn7) - locals.var_q_rac_qsq__blk828_dn7)) * assign39790_e45437) + (assign39790_e45424 * ((((locals.var_q1d__blk1001_dn7 + locals.var_xdeff__blk1000_dn7) - locals.var_xg1x__blk930_dn7) - locals.var_q_rac_qsq__blk828_dn7) * 0.3333333333333)))))), (5.54062e34 * (((((locals.var_q1d__blk1001_dn8 + locals.var_xdeff__blk1000_dn8) - locals.var_xg1x__blk930_dn8) - locals.var_q_rac_qsq__blk828_dn8) * assign39790_e45439) + (assign39790_e45412 * (((0.5 * (((locals.var_q1d__blk1001_dn8 + locals.var_xdeff__blk1000_dn8) - locals.var_xg1x__blk930_dn8) - locals.var_q_rac_qsq__blk828_dn8)) * assign39790_e45437) + (assign39790_e45424 * ((((locals.var_q1d__blk1001_dn8 + locals.var_xdeff__blk1000_dn8) - locals.var_xg1x__blk930_dn8) - locals.var_q_rac_qsq__blk828_dn8) * 0.3333333333333)))))), (5.54062e34 * (((((locals.var_q1d__blk1001_dn9 + locals.var_xdeff__blk1000_dn9) - locals.var_xg1x__blk930_dn9) - locals.var_q_rac_qsq__blk828_dn9) * assign39790_e45439) + (assign39790_e45412 * (((0.5 * (((locals.var_q1d__blk1001_dn9 + locals.var_xdeff__blk1000_dn9) - locals.var_xg1x__blk930_dn9) - locals.var_q_rac_qsq__blk828_dn9)) * assign39790_e45437) + (assign39790_e45424 * ((((locals.var_q1d__blk1001_dn9 + locals.var_xdeff__blk1000_dn9) - locals.var_xg1x__blk930_dn9) - locals.var_q_rac_qsq__blk828_dn9) * 0.3333333333333)))))), );
            locals.var_q_temp3__blk816_rv = 0.0;
        }
        if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1206 == 0.0)) && (locals.var_guard1209 == 0.0)) && (locals.var_guard1213 != 0.0)) {
            let assign39800_e45456: f64 = (locals.var_q_temp3__blk816 / locals.var_a0__blk905);
            (locals.var_q_temp2__blk815, locals.var_q_temp2__blk815_dn4, locals.var_q_temp2__blk815_dn6, locals.var_q_temp2__blk815_dn7, locals.var_q_temp2__blk815_dn8, locals.var_q_temp2__blk815_dn9, ) = (assign39800_e45456, (((locals.var_q_temp3__blk816_dn4 * locals.var_a0__blk905) - (locals.var_q_temp3__blk816 * locals.var_a0__blk905_dn4)) / (locals.var_a0__blk905 * locals.var_a0__blk905)), (((locals.var_q_temp3__blk816_dn6 * locals.var_a0__blk905) - (locals.var_q_temp3__blk816 * locals.var_a0__blk905_dn6)) / (locals.var_a0__blk905 * locals.var_a0__blk905)), (((locals.var_q_temp3__blk816_dn7 * locals.var_a0__blk905) - (locals.var_q_temp3__blk816 * locals.var_a0__blk905_dn7)) / (locals.var_a0__blk905 * locals.var_a0__blk905)), (((locals.var_q_temp3__blk816_dn8 * locals.var_a0__blk905) - (locals.var_q_temp3__blk816 * locals.var_a0__blk905_dn8)) / (locals.var_a0__blk905 * locals.var_a0__blk905)), (((locals.var_q_temp3__blk816_dn9 * locals.var_a0__blk905) - (locals.var_q_temp3__blk816 * locals.var_a0__blk905_dn9)) / (locals.var_a0__blk905 * locals.var_a0__blk905)), );
            locals.var_q_temp2__blk815_rv = 0.0;
        }
        if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1206 == 0.0)) && (locals.var_guard1209 == 0.0)) && (locals.var_guard1213 != 0.0)) {
            let assign39810_e45470: f64 = (4.0 * locals.var_qsqd__blk1006);
            let assign39810_e45472: f64 = (assign39810_e45470 * locals.var_q_temp2__blk815);
            let assign39810_e45477: f64 = (2.0 - locals.var_q_invexpq__blk831);
            let assign39810_e45478: f64 = (locals.var_q_invexpq__blk831 * assign39810_e45477);
            let assign39810_e45479: f64 = (1.0 - assign39810_e45478);
            let assign39810_e45480: f64 = (assign39810_e45472 / assign39810_e45479);
            (locals.var_q_temp1__blk814, locals.var_q_temp1__blk814_dn4, locals.var_q_temp1__blk814_dn6, locals.var_q_temp1__blk814_dn7, locals.var_q_temp1__blk814_dn8, locals.var_q_temp1__blk814_dn9, ) = (assign39810_e45480, ((((((4.0 * locals.var_qsqd__blk1006_dn4) * locals.var_q_temp2__blk815) + (assign39810_e45470 * locals.var_q_temp2__blk815_dn4)) * assign39810_e45479) - (assign39810_e45472 * (-((locals.var_q_invexpq__blk831_dn4 * assign39810_e45477) + (locals.var_q_invexpq__blk831 * (-locals.var_q_invexpq__blk831_dn4)))))) / (assign39810_e45479 * assign39810_e45479)), ((((((4.0 * locals.var_qsqd__blk1006_dn6) * locals.var_q_temp2__blk815) + (assign39810_e45470 * locals.var_q_temp2__blk815_dn6)) * assign39810_e45479) - (assign39810_e45472 * (-((locals.var_q_invexpq__blk831_dn6 * assign39810_e45477) + (locals.var_q_invexpq__blk831 * (-locals.var_q_invexpq__blk831_dn6)))))) / (assign39810_e45479 * assign39810_e45479)), ((((((4.0 * locals.var_qsqd__blk1006_dn7) * locals.var_q_temp2__blk815) + (assign39810_e45470 * locals.var_q_temp2__blk815_dn7)) * assign39810_e45479) - (assign39810_e45472 * (-((locals.var_q_invexpq__blk831_dn7 * assign39810_e45477) + (locals.var_q_invexpq__blk831 * (-locals.var_q_invexpq__blk831_dn7)))))) / (assign39810_e45479 * assign39810_e45479)), ((((((4.0 * locals.var_qsqd__blk1006_dn8) * locals.var_q_temp2__blk815) + (assign39810_e45470 * locals.var_q_temp2__blk815_dn8)) * assign39810_e45479) - (assign39810_e45472 * (-((locals.var_q_invexpq__blk831_dn8 * assign39810_e45477) + (locals.var_q_invexpq__blk831 * (-locals.var_q_invexpq__blk831_dn8)))))) / (assign39810_e45479 * assign39810_e45479)), ((((((4.0 * locals.var_qsqd__blk1006_dn9) * locals.var_q_temp2__blk815) + (assign39810_e45470 * locals.var_q_temp2__blk815_dn9)) * assign39810_e45479) - (assign39810_e45472 * (-((locals.var_q_invexpq__blk831_dn9 * assign39810_e45477) + (locals.var_q_invexpq__blk831 * (-locals.var_q_invexpq__blk831_dn9)))))) / (assign39810_e45479 * assign39810_e45479)), );
            locals.var_q_temp1__blk814_rv = 0.0;
        }
        let assign39820_e45485: f64 = (-0.005);
        let assign39820_e45486: f64 = if locals.var_qsqd__blk1006 < assign39820_e45485 { 1.0 } else { 0.0 };
        locals.var_guard1215 = assign39820_e45486;
        locals.var_guard1215_rv = 0.0;
        if (((((locals.var_guard1080 != 0.0) && (locals.var_guard1206 == 0.0)) && (locals.var_guard1209 == 0.0)) && (locals.var_guard1213 == 0.0)) && (locals.var_guard1215 != 0.0)) {
            let assign39830_e45501: f64 = (0.5 * locals.var_q_rac_qsq__blk828);
            let assign39830_e45502: f64 = (assign39830_e45501).sin();
            (locals.var_q_temp2__blk815, locals.var_q_temp2__blk815_dn4, locals.var_q_temp2__blk815_dn6, locals.var_q_temp2__blk815_dn7, locals.var_q_temp2__blk815_dn8, locals.var_q_temp2__blk815_dn9, ) = (assign39830_e45502, ((assign39830_e45501).cos() * (0.5 * locals.var_q_rac_qsq__blk828_dn4)), ((assign39830_e45501).cos() * (0.5 * locals.var_q_rac_qsq__blk828_dn6)), ((assign39830_e45501).cos() * (0.5 * locals.var_q_rac_qsq__blk828_dn7)), ((assign39830_e45501).cos() * (0.5 * locals.var_q_rac_qsq__blk828_dn8)), ((assign39830_e45501).cos() * (0.5 * locals.var_q_rac_qsq__blk828_dn9)), );
            locals.var_q_temp2__blk815_rv = 0.0;
        }
        if (((((locals.var_guard1080 != 0.0) && (locals.var_guard1206 == 0.0)) && (locals.var_guard1209 == 0.0)) && (locals.var_guard1213 == 0.0)) && (locals.var_guard1215 != 0.0)) {
            let assign39840_e45518: f64 = (-locals.var_qsqd__blk1006);
            let assign39840_e45521: f64 = (locals.var_q_temp2__blk815 * locals.var_q_temp2__blk815);
            let assign39840_e45522: f64 = (assign39840_e45518 / assign39840_e45521);
            let assign39840_e45524: f64 = (assign39840_e45522 / locals.var_aexp1d__blk1007);
            (locals.var_q_temp1__blk814, locals.var_q_temp1__blk814_dn4, locals.var_q_temp1__blk814_dn6, locals.var_q_temp1__blk814_dn7, locals.var_q_temp1__blk814_dn8, locals.var_q_temp1__blk814_dn9, ) = (assign39840_e45524, (((((((-locals.var_qsqd__blk1006_dn4) * assign39840_e45521) - (assign39840_e45518 * ((locals.var_q_temp2__blk815_dn4 * locals.var_q_temp2__blk815) + (locals.var_q_temp2__blk815 * locals.var_q_temp2__blk815_dn4)))) / (assign39840_e45521 * assign39840_e45521)) * locals.var_aexp1d__blk1007) - (assign39840_e45522 * locals.var_aexp1d__blk1007_dn4)) / (locals.var_aexp1d__blk1007 * locals.var_aexp1d__blk1007)), (((((((-locals.var_qsqd__blk1006_dn6) * assign39840_e45521) - (assign39840_e45518 * ((locals.var_q_temp2__blk815_dn6 * locals.var_q_temp2__blk815) + (locals.var_q_temp2__blk815 * locals.var_q_temp2__blk815_dn6)))) / (assign39840_e45521 * assign39840_e45521)) * locals.var_aexp1d__blk1007) - (assign39840_e45522 * locals.var_aexp1d__blk1007_dn6)) / (locals.var_aexp1d__blk1007 * locals.var_aexp1d__blk1007)), (((((((-locals.var_qsqd__blk1006_dn7) * assign39840_e45521) - (assign39840_e45518 * ((locals.var_q_temp2__blk815_dn7 * locals.var_q_temp2__blk815) + (locals.var_q_temp2__blk815 * locals.var_q_temp2__blk815_dn7)))) / (assign39840_e45521 * assign39840_e45521)) * locals.var_aexp1d__blk1007) - (assign39840_e45522 * locals.var_aexp1d__blk1007_dn7)) / (locals.var_aexp1d__blk1007 * locals.var_aexp1d__blk1007)), (((((((-locals.var_qsqd__blk1006_dn8) * assign39840_e45521) - (assign39840_e45518 * ((locals.var_q_temp2__blk815_dn8 * locals.var_q_temp2__blk815) + (locals.var_q_temp2__blk815 * locals.var_q_temp2__blk815_dn8)))) / (assign39840_e45521 * assign39840_e45521)) * locals.var_aexp1d__blk1007) - (assign39840_e45522 * locals.var_aexp1d__blk1007_dn8)) / (locals.var_aexp1d__blk1007 * locals.var_aexp1d__blk1007)), (((((((-locals.var_qsqd__blk1006_dn9) * assign39840_e45521) - (assign39840_e45518 * ((locals.var_q_temp2__blk815_dn9 * locals.var_q_temp2__blk815) + (locals.var_q_temp2__blk815 * locals.var_q_temp2__blk815_dn9)))) / (assign39840_e45521 * assign39840_e45521)) * locals.var_aexp1d__blk1007) - (assign39840_e45522 * locals.var_aexp1d__blk1007_dn9)) / (locals.var_aexp1d__blk1007 * locals.var_aexp1d__blk1007)), );
            locals.var_q_temp1__blk814_rv = 0.0;
        }
        if (((((locals.var_guard1080 != 0.0) && (locals.var_guard1206 == 0.0)) && (locals.var_guard1209 == 0.0)) && (locals.var_guard1213 == 0.0)) && (locals.var_guard1215 == 0.0)) {
            let assign39850_e45543: f64 = (locals.var_qsqd__blk1006 * 0.3333333333333);
            let assign39850_e45547: f64 = (0.05 * locals.var_qsqd__blk1006);
            let assign39850_e45551: f64 = (0.0396825396825397 * locals.var_qsqd__blk1006);
            let assign39850_e45552: f64 = (1.0 - assign39850_e45551);
            let assign39850_e45553: f64 = (assign39850_e45547 * assign39850_e45552);
            let assign39850_e45554: f64 = (1.0 - assign39850_e45553);
            let assign39850_e45555: f64 = (assign39850_e45543 * assign39850_e45554);
            let assign39850_e45556: f64 = (4.0 - assign39850_e45555);
            let assign39850_e45558: f64 = (assign39850_e45556 / locals.var_aexp1d__blk1007);
            (locals.var_q_temp1__blk814, locals.var_q_temp1__blk814_dn4, locals.var_q_temp1__blk814_dn6, locals.var_q_temp1__blk814_dn7, locals.var_q_temp1__blk814_dn8, locals.var_q_temp1__blk814_dn9, ) = (assign39850_e45558, ((((-(((locals.var_qsqd__blk1006_dn4 * 0.3333333333333) * assign39850_e45554) + (assign39850_e45543 * (-(((0.05 * locals.var_qsqd__blk1006_dn4) * assign39850_e45552) + (assign39850_e45547 * (-(0.0396825396825397 * locals.var_qsqd__blk1006_dn4)))))))) * locals.var_aexp1d__blk1007) - (assign39850_e45556 * locals.var_aexp1d__blk1007_dn4)) / (locals.var_aexp1d__blk1007 * locals.var_aexp1d__blk1007)), ((((-(((locals.var_qsqd__blk1006_dn6 * 0.3333333333333) * assign39850_e45554) + (assign39850_e45543 * (-(((0.05 * locals.var_qsqd__blk1006_dn6) * assign39850_e45552) + (assign39850_e45547 * (-(0.0396825396825397 * locals.var_qsqd__blk1006_dn6)))))))) * locals.var_aexp1d__blk1007) - (assign39850_e45556 * locals.var_aexp1d__blk1007_dn6)) / (locals.var_aexp1d__blk1007 * locals.var_aexp1d__blk1007)), ((((-(((locals.var_qsqd__blk1006_dn7 * 0.3333333333333) * assign39850_e45554) + (assign39850_e45543 * (-(((0.05 * locals.var_qsqd__blk1006_dn7) * assign39850_e45552) + (assign39850_e45547 * (-(0.0396825396825397 * locals.var_qsqd__blk1006_dn7)))))))) * locals.var_aexp1d__blk1007) - (assign39850_e45556 * locals.var_aexp1d__blk1007_dn7)) / (locals.var_aexp1d__blk1007 * locals.var_aexp1d__blk1007)), ((((-(((locals.var_qsqd__blk1006_dn8 * 0.3333333333333) * assign39850_e45554) + (assign39850_e45543 * (-(((0.05 * locals.var_qsqd__blk1006_dn8) * assign39850_e45552) + (assign39850_e45547 * (-(0.0396825396825397 * locals.var_qsqd__blk1006_dn8)))))))) * locals.var_aexp1d__blk1007) - (assign39850_e45556 * locals.var_aexp1d__blk1007_dn8)) / (locals.var_aexp1d__blk1007 * locals.var_aexp1d__blk1007)), ((((-(((locals.var_qsqd__blk1006_dn9 * 0.3333333333333) * assign39850_e45554) + (assign39850_e45543 * (-(((0.05 * locals.var_qsqd__blk1006_dn9) * assign39850_e45552) + (assign39850_e45547 * (-(0.0396825396825397 * locals.var_qsqd__blk1006_dn9)))))))) * locals.var_aexp1d__blk1007) - (assign39850_e45556 * locals.var_aexp1d__blk1007_dn9)) / (locals.var_aexp1d__blk1007 * locals.var_aexp1d__blk1007)), );
            locals.var_q_temp1__blk814_rv = 0.0;
        }
        if (((locals.var_guard1080 != 0.0) && (locals.var_guard1206 == 0.0)) && (locals.var_guard1209 == 0.0)) {
            let assign39860_e45570: f64 = (locals.var_k1q1d__blk1004 - locals.var_q_qcoth__blk829);
            let assign39860_e45573: f64 = (1.0 - locals.var_q_temp1__blk814);
            let assign39860_e45574: f64 = (assign39860_e45570 / assign39860_e45573);
            let assign39860_e45576: f64 = (assign39860_e45574 + 1e-80);
            (locals.var_qid__blk1003, locals.var_qid__blk1003_dn4, locals.var_qid__blk1003_dn6, locals.var_qid__blk1003_dn7, locals.var_qid__blk1003_dn8, locals.var_qid__blk1003_dn9, ) = (assign39860_e45576, ((((locals.var_k1q1d__blk1004_dn4 - locals.var_q_qcoth__blk829_dn4) * assign39860_e45573) - (assign39860_e45570 * (-locals.var_q_temp1__blk814_dn4))) / (assign39860_e45573 * assign39860_e45573)), ((((locals.var_k1q1d__blk1004_dn6 - locals.var_q_qcoth__blk829_dn6) * assign39860_e45573) - (assign39860_e45570 * (-locals.var_q_temp1__blk814_dn6))) / (assign39860_e45573 * assign39860_e45573)), ((((locals.var_k1q1d__blk1004_dn7 - locals.var_q_qcoth__blk829_dn7) * assign39860_e45573) - (assign39860_e45570 * (-locals.var_q_temp1__blk814_dn7))) / (assign39860_e45573 * assign39860_e45573)), ((((locals.var_k1q1d__blk1004_dn8 - locals.var_q_qcoth__blk829_dn8) * assign39860_e45573) - (assign39860_e45570 * (-locals.var_q_temp1__blk814_dn8))) / (assign39860_e45573 * assign39860_e45573)), ((((locals.var_k1q1d__blk1004_dn9 - locals.var_q_qcoth__blk829_dn9) * assign39860_e45573) - (assign39860_e45570 * (-locals.var_q_temp1__blk814_dn9))) / (assign39860_e45573 * assign39860_e45573)), );
            locals.var_qid__blk1003_rv = 0.0;
        }
        if (((locals.var_guard1080 != 0.0) && (locals.var_guard1206 == 0.0)) && (locals.var_guard1209 == 0.0)) {
            let assign39870_e45588: f64 = (locals.var_qid__blk1003 - locals.var_k1q1d__blk1004);
            (locals.var_k2q2d__blk1005, locals.var_k2q2d__blk1005_dn4, locals.var_k2q2d__blk1005_dn6, locals.var_k2q2d__blk1005_dn7, locals.var_k2q2d__blk1005_dn8, locals.var_k2q2d__blk1005_dn9, ) = (assign39870_e45588, (locals.var_qid__blk1003_dn4 - locals.var_k1q1d__blk1004_dn4), (locals.var_qid__blk1003_dn6 - locals.var_k1q1d__blk1004_dn6), (locals.var_qid__blk1003_dn7 - locals.var_k1q1d__blk1004_dn7), (locals.var_qid__blk1003_dn8 - locals.var_k1q1d__blk1004_dn8), (locals.var_qid__blk1003_dn9 - locals.var_k1q1d__blk1004_dn9), );
            locals.var_k2q2d__blk1005_rv = 0.0;
        }
        if (((locals.var_guard1080 != 0.0) && (locals.var_guard1206 == 0.0)) && (locals.var_guard1209 == 0.0)) {
            let assign39880_e45600: f64 = (locals.var_k2q2d__blk1005 / locals.var_k2__blk933);
            (locals.var_q2d__blk1002, locals.var_q2d__blk1002_dn4, locals.var_q2d__blk1002_dn6, locals.var_q2d__blk1002_dn7, locals.var_q2d__blk1002_dn8, locals.var_q2d__blk1002_dn9, ) = (assign39880_e45600, (((locals.var_k2q2d__blk1005_dn4 * locals.var_k2__blk933) - (locals.var_k2q2d__blk1005 * locals.var_k2__blk933_dn4)) / (locals.var_k2__blk933 * locals.var_k2__blk933)), (((locals.var_k2q2d__blk1005_dn6 * locals.var_k2__blk933) - (locals.var_k2q2d__blk1005 * locals.var_k2__blk933_dn6)) / (locals.var_k2__blk933 * locals.var_k2__blk933)), (((locals.var_k2q2d__blk1005_dn7 * locals.var_k2__blk933) - (locals.var_k2q2d__blk1005 * locals.var_k2__blk933_dn7)) / (locals.var_k2__blk933 * locals.var_k2__blk933)), (((locals.var_k2q2d__blk1005_dn8 * locals.var_k2__blk933) - (locals.var_k2q2d__blk1005 * locals.var_k2__blk933_dn8)) / (locals.var_k2__blk933 * locals.var_k2__blk933)), (((locals.var_k2q2d__blk1005_dn9 * locals.var_k2__blk933) - (locals.var_k2q2d__blk1005 * locals.var_k2__blk933_dn9)) / (locals.var_k2__blk933 * locals.var_k2__blk933)), );
            locals.var_q2d__blk1002_rv = 0.0;
        }
        let assign39890_e45605: f64 = (locals.var_xg2x__blk931 - locals.var_q2d__blk1002);
        let assign39890_e45607: f64 = (assign39890_e45605 - locals.var_xdeff__blk1000);
        let assign39890_e45609: f64 = if assign39890_e45607 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard1216 = assign39890_e45609;
        locals.var_guard1216_rv = 0.0;
        if ((locals.var_guard1080 != 0.0) && (locals.var_guard1216 != 0.0)) {
            let assign39900_e45615: f64 = (locals.var_xg2x__blk931 - locals.var_q2d__blk1002);
            let assign39900_e45617: f64 = (assign39900_e45615 - locals.var_xdeff__blk1000);
            let assign39900_e45618: f64 = (assign39900_e45617).exp();
            (locals.var_q_temp1__blk814, locals.var_q_temp1__blk814_dn4, locals.var_q_temp1__blk814_dn6, locals.var_q_temp1__blk814_dn7, locals.var_q_temp1__blk814_dn8, locals.var_q_temp1__blk814_dn9, ) = (assign39900_e45618, (assign39900_e45618 * ((locals.var_xg2x__blk931_dn4 - locals.var_q2d__blk1002_dn4) - locals.var_xdeff__blk1000_dn4)), (assign39900_e45618 * ((locals.var_xg2x__blk931_dn6 - locals.var_q2d__blk1002_dn6) - locals.var_xdeff__blk1000_dn6)), (assign39900_e45618 * ((locals.var_xg2x__blk931_dn7 - locals.var_q2d__blk1002_dn7) - locals.var_xdeff__blk1000_dn7)), (assign39900_e45618 * ((locals.var_xg2x__blk931_dn8 - locals.var_q2d__blk1002_dn8) - locals.var_xdeff__blk1000_dn8)), (assign39900_e45618 * ((locals.var_xg2x__blk931_dn9 - locals.var_q2d__blk1002_dn9) - locals.var_xdeff__blk1000_dn9)), );
            locals.var_q_temp1__blk814_rv = 0.0;
        }
        if ((locals.var_guard1080 != 0.0) && (locals.var_guard1216 == 0.0)) {
            let assign39910_e45629: f64 = (locals.var_xg2x__blk931 - locals.var_q2d__blk1002);
            let assign39910_e45631: f64 = (assign39910_e45629 - locals.var_xdeff__blk1000);
            let assign39910_e45633: f64 = (assign39910_e45631 - 80.0);
            let assign39910_e45638: f64 = (locals.var_xg2x__blk931 - locals.var_q2d__blk1002);
            let assign39910_e45640: f64 = (assign39910_e45638 - locals.var_xdeff__blk1000);
            let assign39910_e45642: f64 = (assign39910_e45640 - 80.0);
            let assign39910_e45643: f64 = (0.5 * assign39910_e45642);
            let assign39910_e45647: f64 = (locals.var_xg2x__blk931 - locals.var_q2d__blk1002);
            let assign39910_e45649: f64 = (assign39910_e45647 - locals.var_xdeff__blk1000);
            let assign39910_e45651: f64 = (assign39910_e45649 - 80.0);
            let assign39910_e45653: f64 = (assign39910_e45651 * 0.3333333333333);
            let assign39910_e45654: f64 = (1.0 + assign39910_e45653);
            let assign39910_e45655: f64 = (assign39910_e45643 * assign39910_e45654);
            let assign39910_e45656: f64 = (1.0 + assign39910_e45655);
            let assign39910_e45657: f64 = (assign39910_e45633 * assign39910_e45656);
            let assign39910_e45658: f64 = (1.0 + assign39910_e45657);
            let assign39910_e45659: f64 = (5.54062e34 * assign39910_e45658);
            (locals.var_q_temp1__blk814, locals.var_q_temp1__blk814_dn4, locals.var_q_temp1__blk814_dn6, locals.var_q_temp1__blk814_dn7, locals.var_q_temp1__blk814_dn8, locals.var_q_temp1__blk814_dn9, ) = (assign39910_e45659, (5.54062e34 * ((((locals.var_xg2x__blk931_dn4 - locals.var_q2d__blk1002_dn4) - locals.var_xdeff__blk1000_dn4) * assign39910_e45656) + (assign39910_e45633 * (((0.5 * ((locals.var_xg2x__blk931_dn4 - locals.var_q2d__blk1002_dn4) - locals.var_xdeff__blk1000_dn4)) * assign39910_e45654) + (assign39910_e45643 * (((locals.var_xg2x__blk931_dn4 - locals.var_q2d__blk1002_dn4) - locals.var_xdeff__blk1000_dn4) * 0.3333333333333)))))), (5.54062e34 * ((((locals.var_xg2x__blk931_dn6 - locals.var_q2d__blk1002_dn6) - locals.var_xdeff__blk1000_dn6) * assign39910_e45656) + (assign39910_e45633 * (((0.5 * ((locals.var_xg2x__blk931_dn6 - locals.var_q2d__blk1002_dn6) - locals.var_xdeff__blk1000_dn6)) * assign39910_e45654) + (assign39910_e45643 * (((locals.var_xg2x__blk931_dn6 - locals.var_q2d__blk1002_dn6) - locals.var_xdeff__blk1000_dn6) * 0.3333333333333)))))), (5.54062e34 * ((((locals.var_xg2x__blk931_dn7 - locals.var_q2d__blk1002_dn7) - locals.var_xdeff__blk1000_dn7) * assign39910_e45656) + (assign39910_e45633 * (((0.5 * ((locals.var_xg2x__blk931_dn7 - locals.var_q2d__blk1002_dn7) - locals.var_xdeff__blk1000_dn7)) * assign39910_e45654) + (assign39910_e45643 * (((locals.var_xg2x__blk931_dn7 - locals.var_q2d__blk1002_dn7) - locals.var_xdeff__blk1000_dn7) * 0.3333333333333)))))), (5.54062e34 * ((((locals.var_xg2x__blk931_dn8 - locals.var_q2d__blk1002_dn8) - locals.var_xdeff__blk1000_dn8) * assign39910_e45656) + (assign39910_e45633 * (((0.5 * ((locals.var_xg2x__blk931_dn8 - locals.var_q2d__blk1002_dn8) - locals.var_xdeff__blk1000_dn8)) * assign39910_e45654) + (assign39910_e45643 * (((locals.var_xg2x__blk931_dn8 - locals.var_q2d__blk1002_dn8) - locals.var_xdeff__blk1000_dn8) * 0.3333333333333)))))), (5.54062e34 * ((((locals.var_xg2x__blk931_dn9 - locals.var_q2d__blk1002_dn9) - locals.var_xdeff__blk1000_dn9) * assign39910_e45656) + (assign39910_e45633 * (((0.5 * ((locals.var_xg2x__blk931_dn9 - locals.var_q2d__blk1002_dn9) - locals.var_xdeff__blk1000_dn9)) * assign39910_e45654) + (assign39910_e45643 * (((locals.var_xg2x__blk931_dn9 - locals.var_q2d__blk1002_dn9) - locals.var_xdeff__blk1000_dn9) * 0.3333333333333)))))), );
            locals.var_q_temp1__blk814_rv = 0.0;
        }
        if (locals.var_guard1080 != 0.0) {
            let assign39920_e45665: f64 = (locals.var_a0__blk905 * locals.var_q_temp1__blk814);
            (locals.var_aexp2d__blk1008, locals.var_aexp2d__blk1008_dn4, locals.var_aexp2d__blk1008_dn6, locals.var_aexp2d__blk1008_dn7, locals.var_aexp2d__blk1008_dn8, locals.var_aexp2d__blk1008_dn9, ) = (assign39920_e45665, ((locals.var_a0__blk905_dn4 * locals.var_q_temp1__blk814) + (locals.var_a0__blk905 * locals.var_q_temp1__blk814_dn4)), ((locals.var_a0__blk905_dn6 * locals.var_q_temp1__blk814) + (locals.var_a0__blk905 * locals.var_q_temp1__blk814_dn6)), ((locals.var_a0__blk905_dn7 * locals.var_q_temp1__blk814) + (locals.var_a0__blk905 * locals.var_q_temp1__blk814_dn7)), ((locals.var_a0__blk905_dn8 * locals.var_q_temp1__blk814) + (locals.var_a0__blk905 * locals.var_q_temp1__blk814_dn8)), ((locals.var_a0__blk905_dn9 * locals.var_q_temp1__blk814) + (locals.var_a0__blk905 * locals.var_q_temp1__blk814_dn9)), );
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
        let assign39990_e45694: f64 = if locals.var_qis__blk938 > 1e-6 { 1.0 } else { 0.0 };
        locals.var_guard1217 = assign39990_e45694;
        locals.var_guard1217_rv = 0.0;
        if ((locals.var_guard1080 != 0.0) && (locals.var_guard1217 != 0.0)) {
            let assign40000_e45700: f64 = (locals.var_aexp1d__blk1007 * locals.var_inv_k1__blk906);
            (locals.var_b1d__blk1009, locals.var_b1d__blk1009_dn4, locals.var_b1d__blk1009_dn6, locals.var_b1d__blk1009_dn7, locals.var_b1d__blk1009_dn8, locals.var_b1d__blk1009_dn9, ) = (assign40000_e45700, ((locals.var_aexp1d__blk1007_dn4 * locals.var_inv_k1__blk906) + (locals.var_aexp1d__blk1007 * locals.var_inv_k1__blk906_dn4)), ((locals.var_aexp1d__blk1007_dn6 * locals.var_inv_k1__blk906) + (locals.var_aexp1d__blk1007 * locals.var_inv_k1__blk906_dn6)), ((locals.var_aexp1d__blk1007_dn7 * locals.var_inv_k1__blk906) + (locals.var_aexp1d__blk1007 * locals.var_inv_k1__blk906_dn7)), ((locals.var_aexp1d__blk1007_dn8 * locals.var_inv_k1__blk906) + (locals.var_aexp1d__blk1007 * locals.var_inv_k1__blk906_dn8)), ((locals.var_aexp1d__blk1007_dn9 * locals.var_inv_k1__blk906) + (locals.var_aexp1d__blk1007 * locals.var_inv_k1__blk906_dn9)), );
            locals.var_b1d__blk1009_rv = 0.0;
        }
        if ((locals.var_guard1080 != 0.0) && (locals.var_guard1217 != 0.0)) {
            let assign40010_e45708: f64 = (locals.var_aexp2d__blk1008 * locals.var_inv_k2__blk907);
            (locals.var_b2d__blk1010, locals.var_b2d__blk1010_dn4, locals.var_b2d__blk1010_dn6, locals.var_b2d__blk1010_dn7, locals.var_b2d__blk1010_dn8, locals.var_b2d__blk1010_dn9, ) = (assign40010_e45708, ((locals.var_aexp2d__blk1008_dn4 * locals.var_inv_k2__blk907) + (locals.var_aexp2d__blk1008 * locals.var_inv_k2__blk907_dn4)), ((locals.var_aexp2d__blk1008_dn6 * locals.var_inv_k2__blk907) + (locals.var_aexp2d__blk1008 * locals.var_inv_k2__blk907_dn6)), ((locals.var_aexp2d__blk1008_dn7 * locals.var_inv_k2__blk907) + (locals.var_aexp2d__blk1008 * locals.var_inv_k2__blk907_dn7)), ((locals.var_aexp2d__blk1008_dn8 * locals.var_inv_k2__blk907) + (locals.var_aexp2d__blk1008 * locals.var_inv_k2__blk907_dn8)), ((locals.var_aexp2d__blk1008_dn9 * locals.var_inv_k2__blk907) + (locals.var_aexp2d__blk1008 * locals.var_inv_k2__blk907_dn9)), );
            locals.var_b2d__blk1010_rv = 0.0;
        }
        if ((locals.var_guard1080 != 0.0) && (locals.var_guard1217 != 0.0)) {
            let assign40020_e45717: f64 = (2.0 * locals.var_k1q1d__blk1004);
            let assign40020_e45718: f64 = (locals.var_b1d__blk1009 + assign40020_e45717);
            (locals.var_a1d__blk1011, locals.var_a1d__blk1011_dn4, locals.var_a1d__blk1011_dn6, locals.var_a1d__blk1011_dn7, locals.var_a1d__blk1011_dn8, locals.var_a1d__blk1011_dn9, ) = (assign40020_e45718, (locals.var_b1d__blk1009_dn4 + (2.0 * locals.var_k1q1d__blk1004_dn4)), (locals.var_b1d__blk1009_dn6 + (2.0 * locals.var_k1q1d__blk1004_dn6)), (locals.var_b1d__blk1009_dn7 + (2.0 * locals.var_k1q1d__blk1004_dn7)), (locals.var_b1d__blk1009_dn8 + (2.0 * locals.var_k1q1d__blk1004_dn8)), (locals.var_b1d__blk1009_dn9 + (2.0 * locals.var_k1q1d__blk1004_dn9)), );
            locals.var_a1d__blk1011_rv = 0.0;
        }
        if ((locals.var_guard1080 != 0.0) && (locals.var_guard1217 != 0.0)) {
            let assign40030_e45727: f64 = (2.0 * locals.var_k2q2d__blk1005);
            let assign40030_e45728: f64 = (locals.var_b2d__blk1010 + assign40030_e45727);
            (locals.var_a2d__blk1012, locals.var_a2d__blk1012_dn4, locals.var_a2d__blk1012_dn6, locals.var_a2d__blk1012_dn7, locals.var_a2d__blk1012_dn8, locals.var_a2d__blk1012_dn9, ) = (assign40030_e45728, (locals.var_b2d__blk1010_dn4 + (2.0 * locals.var_k2q2d__blk1005_dn4)), (locals.var_b2d__blk1010_dn6 + (2.0 * locals.var_k2q2d__blk1005_dn6)), (locals.var_b2d__blk1010_dn7 + (2.0 * locals.var_k2q2d__blk1005_dn7)), (locals.var_b2d__blk1010_dn8 + (2.0 * locals.var_k2q2d__blk1005_dn8)), (locals.var_b2d__blk1010_dn9 + (2.0 * locals.var_k2q2d__blk1005_dn9)), );
            locals.var_a2d__blk1012_rv = 0.0;
        }
        if ((locals.var_guard1080 != 0.0) && (locals.var_guard1217 != 0.0)) {
            let assign40040_e45736: f64 = (2.0 * locals.var_qid__blk1003);
            let assign40040_e45738: f64 = (assign40040_e45736 + locals.var_b1d__blk1009);
            let assign40040_e45740: f64 = (assign40040_e45738 + locals.var_b2d__blk1010);
            (locals.var_sumd__blk1013, locals.var_sumd__blk1013_dn4, locals.var_sumd__blk1013_dn6, locals.var_sumd__blk1013_dn7, locals.var_sumd__blk1013_dn8, locals.var_sumd__blk1013_dn9, ) = (assign40040_e45740, (((2.0 * locals.var_qid__blk1003_dn4) + locals.var_b1d__blk1009_dn4) + locals.var_b2d__blk1010_dn4), (((2.0 * locals.var_qid__blk1003_dn6) + locals.var_b1d__blk1009_dn6) + locals.var_b2d__blk1010_dn6), (((2.0 * locals.var_qid__blk1003_dn7) + locals.var_b1d__blk1009_dn7) + locals.var_b2d__blk1010_dn7), (((2.0 * locals.var_qid__blk1003_dn8) + locals.var_b1d__blk1009_dn8) + locals.var_b2d__blk1010_dn8), (((2.0 * locals.var_qid__blk1003_dn9) + locals.var_b1d__blk1009_dn9) + locals.var_b2d__blk1010_dn9), );
            locals.var_sumd__blk1013_rv = 0.0;
        }
        let assign40050_e45744: f64 = (locals.var_qsqd__blk1006).abs();
        let assign40050_e45746: f64 = if assign40050_e45744 > 0.005 { 1.0 } else { 0.0 };
        locals.var_guard1218 = assign40050_e45746;
        locals.var_guard1218_rv = 0.0;
        if (((locals.var_guard1080 != 0.0) && (locals.var_guard1217 != 0.0)) && (locals.var_guard1218 != 0.0)) {
            let assign40060_e45754: f64 = (locals.var_a1d__blk1011 * locals.var_a2d__blk1012);
            let assign40060_e45758: f64 = (locals.var_q1d__blk1001 + 2.0);
            let assign40060_e45759: f64 = (2.0 * assign40060_e45758);
            let assign40060_e45761: f64 = (assign40060_e45759 * locals.var_a2d__blk1012);
            let assign40060_e45762: f64 = (assign40060_e45754 + assign40060_e45761);
            let assign40060_e45766: f64 = (locals.var_q2d__blk1002 + 2.0);
            let assign40060_e45767: f64 = (2.0 * assign40060_e45766);
            let assign40060_e45769: f64 = (assign40060_e45767 * locals.var_a1d__blk1011);
            let assign40060_e45770: f64 = (assign40060_e45762 + assign40060_e45769);
            (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9, ) = (assign40060_e45770, ((((locals.var_a1d__blk1011_dn4 * locals.var_a2d__blk1012) + (locals.var_a1d__blk1011 * locals.var_a2d__blk1012_dn4)) + (((2.0 * locals.var_q1d__blk1001_dn4) * locals.var_a2d__blk1012) + (assign40060_e45759 * locals.var_a2d__blk1012_dn4))) + (((2.0 * locals.var_q2d__blk1002_dn4) * locals.var_a1d__blk1011) + (assign40060_e45767 * locals.var_a1d__blk1011_dn4))), ((((locals.var_a1d__blk1011_dn6 * locals.var_a2d__blk1012) + (locals.var_a1d__blk1011 * locals.var_a2d__blk1012_dn6)) + (((2.0 * locals.var_q1d__blk1001_dn6) * locals.var_a2d__blk1012) + (assign40060_e45759 * locals.var_a2d__blk1012_dn6))) + (((2.0 * locals.var_q2d__blk1002_dn6) * locals.var_a1d__blk1011) + (assign40060_e45767 * locals.var_a1d__blk1011_dn6))), ((((locals.var_a1d__blk1011_dn7 * locals.var_a2d__blk1012) + (locals.var_a1d__blk1011 * locals.var_a2d__blk1012_dn7)) + (((2.0 * locals.var_q1d__blk1001_dn7) * locals.var_a2d__blk1012) + (assign40060_e45759 * locals.var_a2d__blk1012_dn7))) + (((2.0 * locals.var_q2d__blk1002_dn7) * locals.var_a1d__blk1011) + (assign40060_e45767 * locals.var_a1d__blk1011_dn7))), ((((locals.var_a1d__blk1011_dn8 * locals.var_a2d__blk1012) + (locals.var_a1d__blk1011 * locals.var_a2d__blk1012_dn8)) + (((2.0 * locals.var_q1d__blk1001_dn8) * locals.var_a2d__blk1012) + (assign40060_e45759 * locals.var_a2d__blk1012_dn8))) + (((2.0 * locals.var_q2d__blk1002_dn8) * locals.var_a1d__blk1011) + (assign40060_e45767 * locals.var_a1d__blk1011_dn8))), ((((locals.var_a1d__blk1011_dn9 * locals.var_a2d__blk1012) + (locals.var_a1d__blk1011 * locals.var_a2d__blk1012_dn9)) + (((2.0 * locals.var_q1d__blk1001_dn9) * locals.var_a2d__blk1012) + (assign40060_e45759 * locals.var_a2d__blk1012_dn9))) + (((2.0 * locals.var_q2d__blk1002_dn9) * locals.var_a1d__blk1011) + (assign40060_e45767 * locals.var_a1d__blk1011_dn9))), );
            locals.var_temp1_rv = 0.0;
        }
        if (((locals.var_guard1080 != 0.0) && (locals.var_guard1217 != 0.0)) && (locals.var_guard1218 != 0.0)) {
            let assign40070_e45779: f64 = (-4.0);
            let assign40070_e45781: f64 = (assign40070_e45779 * locals.var_qsqd__blk1006);
            let assign40070_e45783: f64 = (assign40070_e45781 * locals.var_sumd__blk1013);
            let assign40070_e45786: f64 = (locals.var_qid__blk1003 * locals.var_temp1);
            let assign40070_e45787: f64 = (assign40070_e45783 / assign40070_e45786);
            (locals.var_dqsqd_dxn_qi__blk1014, locals.var_dqsqd_dxn_qi__blk1014_dn4, locals.var_dqsqd_dxn_qi__blk1014_dn6, locals.var_dqsqd_dxn_qi__blk1014_dn7, locals.var_dqsqd_dxn_qi__blk1014_dn8, locals.var_dqsqd_dxn_qi__blk1014_dn9, ) = (assign40070_e45787, ((((((assign40070_e45779 * locals.var_qsqd__blk1006_dn4) * locals.var_sumd__blk1013) + (assign40070_e45781 * locals.var_sumd__blk1013_dn4)) * assign40070_e45786) - (assign40070_e45783 * ((locals.var_qid__blk1003_dn4 * locals.var_temp1) + (locals.var_qid__blk1003 * locals.var_temp1_dn4)))) / (assign40070_e45786 * assign40070_e45786)), ((((((assign40070_e45779 * locals.var_qsqd__blk1006_dn6) * locals.var_sumd__blk1013) + (assign40070_e45781 * locals.var_sumd__blk1013_dn6)) * assign40070_e45786) - (assign40070_e45783 * ((locals.var_qid__blk1003_dn6 * locals.var_temp1) + (locals.var_qid__blk1003 * locals.var_temp1_dn6)))) / (assign40070_e45786 * assign40070_e45786)), ((((((assign40070_e45779 * locals.var_qsqd__blk1006_dn7) * locals.var_sumd__blk1013) + (assign40070_e45781 * locals.var_sumd__blk1013_dn7)) * assign40070_e45786) - (assign40070_e45783 * ((locals.var_qid__blk1003_dn7 * locals.var_temp1) + (locals.var_qid__blk1003 * locals.var_temp1_dn7)))) / (assign40070_e45786 * assign40070_e45786)), ((((((assign40070_e45779 * locals.var_qsqd__blk1006_dn8) * locals.var_sumd__blk1013) + (assign40070_e45781 * locals.var_sumd__blk1013_dn8)) * assign40070_e45786) - (assign40070_e45783 * ((locals.var_qid__blk1003_dn8 * locals.var_temp1) + (locals.var_qid__blk1003 * locals.var_temp1_dn8)))) / (assign40070_e45786 * assign40070_e45786)), ((((((assign40070_e45779 * locals.var_qsqd__blk1006_dn9) * locals.var_sumd__blk1013) + (assign40070_e45781 * locals.var_sumd__blk1013_dn9)) * assign40070_e45786) - (assign40070_e45783 * ((locals.var_qid__blk1003_dn9 * locals.var_temp1) + (locals.var_qid__blk1003 * locals.var_temp1_dn9)))) / (assign40070_e45786 * assign40070_e45786)), );
            locals.var_dqsqd_dxn_qi__blk1014_rv = 0.0;
        }
        if (((locals.var_guard1080 != 0.0) && (locals.var_guard1217 != 0.0)) && (locals.var_guard1218 == 0.0)) {
            let assign40080_e45800: f64 = (locals.var_qsqd__blk1006 * 0.0333333333333);
            let assign40080_e45804: f64 = (locals.var_qsqd__blk1006 * 0.0357142857143);
            let assign40080_e45808: f64 = (locals.var_qsqd__blk1006 * 0.0333333333333);
            let assign40080_e45809: f64 = (1.0 - assign40080_e45808);
            let assign40080_e45810: f64 = (assign40080_e45804 * assign40080_e45809);
            let assign40080_e45811: f64 = (1.0 - assign40080_e45810);
            let assign40080_e45812: f64 = (assign40080_e45800 * assign40080_e45811);
            let assign40080_e45813: f64 = (1.0 - assign40080_e45812);
            let assign40080_e45814: f64 = (0.1666666666667 * assign40080_e45813);
            (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9, ) = (assign40080_e45814, (0.1666666666667 * (-(((locals.var_qsqd__blk1006_dn4 * 0.0333333333333) * assign40080_e45811) + (assign40080_e45800 * (-(((locals.var_qsqd__blk1006_dn4 * 0.0357142857143) * assign40080_e45809) + (assign40080_e45804 * (-(locals.var_qsqd__blk1006_dn4 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_qsqd__blk1006_dn6 * 0.0333333333333) * assign40080_e45811) + (assign40080_e45800 * (-(((locals.var_qsqd__blk1006_dn6 * 0.0357142857143) * assign40080_e45809) + (assign40080_e45804 * (-(locals.var_qsqd__blk1006_dn6 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_qsqd__blk1006_dn7 * 0.0333333333333) * assign40080_e45811) + (assign40080_e45800 * (-(((locals.var_qsqd__blk1006_dn7 * 0.0357142857143) * assign40080_e45809) + (assign40080_e45804 * (-(locals.var_qsqd__blk1006_dn7 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_qsqd__blk1006_dn8 * 0.0333333333333) * assign40080_e45811) + (assign40080_e45800 * (-(((locals.var_qsqd__blk1006_dn8 * 0.0357142857143) * assign40080_e45809) + (assign40080_e45804 * (-(locals.var_qsqd__blk1006_dn8 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_qsqd__blk1006_dn9 * 0.0333333333333) * assign40080_e45811) + (assign40080_e45800 * (-(((locals.var_qsqd__blk1006_dn9 * 0.0357142857143) * assign40080_e45809) + (assign40080_e45804 * (-(locals.var_qsqd__blk1006_dn9 * 0.0333333333333))))))))), );
            locals.var_temp1_rv = 0.0;
        }
        if (((locals.var_guard1080 != 0.0) && (locals.var_guard1217 != 0.0)) && (locals.var_guard1218 == 0.0)) {
            let assign40090_e45825: f64 = (locals.var_a1d__blk1011 * locals.var_aexp1d__blk1007);
            let assign40090_e45828: f64 = (locals.var_a2d__blk1012 * locals.var_aexp2d__blk1008);
            let assign40090_e45829: f64 = (assign40090_e45825 + assign40090_e45828);
            let assign40090_e45832: f64 = (locals.var_a1d__blk1011 * locals.var_a2d__blk1012);
            let assign40090_e45834: f64 = (assign40090_e45832 * locals.var_qid__blk1003);
            let assign40090_e45838: f64 = (locals.var_qid__blk1003 * locals.var_temp1);
            let assign40090_e45839: f64 = (1.0 + assign40090_e45838);
            let assign40090_e45840: f64 = (assign40090_e45834 * assign40090_e45839);
            let assign40090_e45841: f64 = (assign40090_e45829 + assign40090_e45840);
            (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9, ) = (assign40090_e45841, ((((locals.var_a1d__blk1011_dn4 * locals.var_aexp1d__blk1007) + (locals.var_a1d__blk1011 * locals.var_aexp1d__blk1007_dn4)) + ((locals.var_a2d__blk1012_dn4 * locals.var_aexp2d__blk1008) + (locals.var_a2d__blk1012 * locals.var_aexp2d__blk1008_dn4))) + ((((((locals.var_a1d__blk1011_dn4 * locals.var_a2d__blk1012) + (locals.var_a1d__blk1011 * locals.var_a2d__blk1012_dn4)) * locals.var_qid__blk1003) + (assign40090_e45832 * locals.var_qid__blk1003_dn4)) * assign40090_e45839) + (assign40090_e45834 * ((locals.var_qid__blk1003_dn4 * locals.var_temp1) + (locals.var_qid__blk1003 * locals.var_temp1_dn4))))), ((((locals.var_a1d__blk1011_dn6 * locals.var_aexp1d__blk1007) + (locals.var_a1d__blk1011 * locals.var_aexp1d__blk1007_dn6)) + ((locals.var_a2d__blk1012_dn6 * locals.var_aexp2d__blk1008) + (locals.var_a2d__blk1012 * locals.var_aexp2d__blk1008_dn6))) + ((((((locals.var_a1d__blk1011_dn6 * locals.var_a2d__blk1012) + (locals.var_a1d__blk1011 * locals.var_a2d__blk1012_dn6)) * locals.var_qid__blk1003) + (assign40090_e45832 * locals.var_qid__blk1003_dn6)) * assign40090_e45839) + (assign40090_e45834 * ((locals.var_qid__blk1003_dn6 * locals.var_temp1) + (locals.var_qid__blk1003 * locals.var_temp1_dn6))))), ((((locals.var_a1d__blk1011_dn7 * locals.var_aexp1d__blk1007) + (locals.var_a1d__blk1011 * locals.var_aexp1d__blk1007_dn7)) + ((locals.var_a2d__blk1012_dn7 * locals.var_aexp2d__blk1008) + (locals.var_a2d__blk1012 * locals.var_aexp2d__blk1008_dn7))) + ((((((locals.var_a1d__blk1011_dn7 * locals.var_a2d__blk1012) + (locals.var_a1d__blk1011 * locals.var_a2d__blk1012_dn7)) * locals.var_qid__blk1003) + (assign40090_e45832 * locals.var_qid__blk1003_dn7)) * assign40090_e45839) + (assign40090_e45834 * ((locals.var_qid__blk1003_dn7 * locals.var_temp1) + (locals.var_qid__blk1003 * locals.var_temp1_dn7))))), ((((locals.var_a1d__blk1011_dn8 * locals.var_aexp1d__blk1007) + (locals.var_a1d__blk1011 * locals.var_aexp1d__blk1007_dn8)) + ((locals.var_a2d__blk1012_dn8 * locals.var_aexp2d__blk1008) + (locals.var_a2d__blk1012 * locals.var_aexp2d__blk1008_dn8))) + ((((((locals.var_a1d__blk1011_dn8 * locals.var_a2d__blk1012) + (locals.var_a1d__blk1011 * locals.var_a2d__blk1012_dn8)) * locals.var_qid__blk1003) + (assign40090_e45832 * locals.var_qid__blk1003_dn8)) * assign40090_e45839) + (assign40090_e45834 * ((locals.var_qid__blk1003_dn8 * locals.var_temp1) + (locals.var_qid__blk1003 * locals.var_temp1_dn8))))), ((((locals.var_a1d__blk1011_dn9 * locals.var_aexp1d__blk1007) + (locals.var_a1d__blk1011 * locals.var_aexp1d__blk1007_dn9)) + ((locals.var_a2d__blk1012_dn9 * locals.var_aexp2d__blk1008) + (locals.var_a2d__blk1012 * locals.var_aexp2d__blk1008_dn9))) + ((((((locals.var_a1d__blk1011_dn9 * locals.var_a2d__blk1012) + (locals.var_a1d__blk1011 * locals.var_a2d__blk1012_dn9)) * locals.var_qid__blk1003) + (assign40090_e45832 * locals.var_qid__blk1003_dn9)) * assign40090_e45839) + (assign40090_e45834 * ((locals.var_qid__blk1003_dn9 * locals.var_temp1) + (locals.var_qid__blk1003 * locals.var_temp1_dn9))))), );
            locals.var_temp2_rv = 0.0;
        }
        if (((locals.var_guard1080 != 0.0) && (locals.var_guard1217 != 0.0)) && (locals.var_guard1218 == 0.0)) {
            let assign40100_e45852: f64 = (locals.var_aexp1d__blk1007 * locals.var_aexp2d__blk1008);
            let assign40100_e45854: f64 = (assign40100_e45852 * locals.var_sumd__blk1013);
            let assign40100_e45857: f64 = (locals.var_qid__blk1003 * locals.var_temp2);
            let assign40100_e45858: f64 = (assign40100_e45854 / assign40100_e45857);
            (locals.var_dqsqd_dxn_qi__blk1014, locals.var_dqsqd_dxn_qi__blk1014_dn4, locals.var_dqsqd_dxn_qi__blk1014_dn6, locals.var_dqsqd_dxn_qi__blk1014_dn7, locals.var_dqsqd_dxn_qi__blk1014_dn8, locals.var_dqsqd_dxn_qi__blk1014_dn9, ) = (assign40100_e45858, (((((((locals.var_aexp1d__blk1007_dn4 * locals.var_aexp2d__blk1008) + (locals.var_aexp1d__blk1007 * locals.var_aexp2d__blk1008_dn4)) * locals.var_sumd__blk1013) + (assign40100_e45852 * locals.var_sumd__blk1013_dn4)) * assign40100_e45857) - (assign40100_e45854 * ((locals.var_qid__blk1003_dn4 * locals.var_temp2) + (locals.var_qid__blk1003 * locals.var_temp2_dn4)))) / (assign40100_e45857 * assign40100_e45857)), (((((((locals.var_aexp1d__blk1007_dn6 * locals.var_aexp2d__blk1008) + (locals.var_aexp1d__blk1007 * locals.var_aexp2d__blk1008_dn6)) * locals.var_sumd__blk1013) + (assign40100_e45852 * locals.var_sumd__blk1013_dn6)) * assign40100_e45857) - (assign40100_e45854 * ((locals.var_qid__blk1003_dn6 * locals.var_temp2) + (locals.var_qid__blk1003 * locals.var_temp2_dn6)))) / (assign40100_e45857 * assign40100_e45857)), (((((((locals.var_aexp1d__blk1007_dn7 * locals.var_aexp2d__blk1008) + (locals.var_aexp1d__blk1007 * locals.var_aexp2d__blk1008_dn7)) * locals.var_sumd__blk1013) + (assign40100_e45852 * locals.var_sumd__blk1013_dn7)) * assign40100_e45857) - (assign40100_e45854 * ((locals.var_qid__blk1003_dn7 * locals.var_temp2) + (locals.var_qid__blk1003 * locals.var_temp2_dn7)))) / (assign40100_e45857 * assign40100_e45857)), (((((((locals.var_aexp1d__blk1007_dn8 * locals.var_aexp2d__blk1008) + (locals.var_aexp1d__blk1007 * locals.var_aexp2d__blk1008_dn8)) * locals.var_sumd__blk1013) + (assign40100_e45852 * locals.var_sumd__blk1013_dn8)) * assign40100_e45857) - (assign40100_e45854 * ((locals.var_qid__blk1003_dn8 * locals.var_temp2) + (locals.var_qid__blk1003 * locals.var_temp2_dn8)))) / (assign40100_e45857 * assign40100_e45857)), (((((((locals.var_aexp1d__blk1007_dn9 * locals.var_aexp2d__blk1008) + (locals.var_aexp1d__blk1007 * locals.var_aexp2d__blk1008_dn9)) * locals.var_sumd__blk1013) + (assign40100_e45852 * locals.var_sumd__blk1013_dn9)) * assign40100_e45857) - (assign40100_e45854 * ((locals.var_qid__blk1003_dn9 * locals.var_temp2) + (locals.var_qid__blk1003 * locals.var_temp2_dn9)))) / (assign40100_e45857 * assign40100_e45857)), );
            locals.var_dqsqd_dxn_qi__blk1014_rv = 0.0;
        }
        if (locals.var_guard1080 != 0.0) {
            let assign40110_e45864: f64 = (locals.var_qid__blk1003).ln();
            let assign40110_e45865: f64 = (locals.var_xdeff__blk1000 + assign40110_e45864);
            (locals.var_xdriftd__blk1015, locals.var_xdriftd__blk1015_dn4, locals.var_xdriftd__blk1015_dn6, locals.var_xdriftd__blk1015_dn7, locals.var_xdriftd__blk1015_dn8, locals.var_xdriftd__blk1015_dn9, ) = (assign40110_e45865, (locals.var_xdeff__blk1000_dn4 + (locals.var_qid__blk1003_dn4 / locals.var_qid__blk1003)), (locals.var_xdeff__blk1000_dn6 + (locals.var_qid__blk1003_dn6 / locals.var_qid__blk1003)), (locals.var_xdeff__blk1000_dn7 + (locals.var_qid__blk1003_dn7 / locals.var_qid__blk1003)), (locals.var_xdeff__blk1000_dn8 + (locals.var_qid__blk1003_dn8 / locals.var_qid__blk1003)), (locals.var_xdeff__blk1000_dn9 + (locals.var_qid__blk1003_dn9 / locals.var_qid__blk1003)), );
            locals.var_xdriftd__blk1015_rv = 0.0;
        }
        if (locals.var_guard1080 != 0.0) {
            let assign40120_e45872: f64 = (locals.var_qis__blk938 + locals.var_qid__blk1003);
            let assign40120_e45873: f64 = (0.5 * assign40120_e45872);
            (locals.var_qim__blk1016, locals.var_qim__blk1016_dn4, locals.var_qim__blk1016_dn6, locals.var_qim__blk1016_dn7, locals.var_qim__blk1016_dn8, locals.var_qim__blk1016_dn9, ) = (assign40120_e45873, (0.5 * (locals.var_qis__blk938_dn4 + locals.var_qid__blk1003_dn4)), (0.5 * (locals.var_qis__blk938_dn6 + locals.var_qid__blk1003_dn6)), (0.5 * (locals.var_qis__blk938_dn7 + locals.var_qid__blk1003_dn7)), (0.5 * (locals.var_qis__blk938_dn8 + locals.var_qid__blk1003_dn8)), (0.5 * (locals.var_qis__blk938_dn9 + locals.var_qid__blk1003_dn9)), );
            locals.var_qim__blk1016_rv = 0.0;
        }
        if (locals.var_guard1080 != 0.0) {
            let assign40130_e45879: f64 = (locals.var_xdriftd__blk1015 - locals.var_xdrifts__blk951);
            (locals.var_dxdrift__blk1017, locals.var_dxdrift__blk1017_dn4, locals.var_dxdrift__blk1017_dn6, locals.var_dxdrift__blk1017_dn7, locals.var_dxdrift__blk1017_dn8, locals.var_dxdrift__blk1017_dn9, ) = (assign40130_e45879, (locals.var_xdriftd__blk1015_dn4 - locals.var_xdrifts__blk951_dn4), (locals.var_xdriftd__blk1015_dn6 - locals.var_xdrifts__blk951_dn6), (locals.var_xdriftd__blk1015_dn7 - locals.var_xdrifts__blk951_dn7), (locals.var_xdriftd__blk1015_dn8 - locals.var_xdrifts__blk951_dn8), (locals.var_xdriftd__blk1015_dn9 - locals.var_xdrifts__blk951_dn9), );
            locals.var_dxdrift__blk1017_rv = 0.0;
        }
        if (locals.var_guard1080 != 0.0) {
            (locals.var_ratio_pd__blk1020, locals.var_ratio_pd__blk1020_dn4, locals.var_ratio_pd__blk1020_dn6, locals.var_ratio_pd__blk1020_dn7, locals.var_ratio_pd__blk1020_dn8, locals.var_ratio_pd__blk1020_dn9, ) = (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_ratio_pd__blk1020_rv = 0.0;
        }
        let assign40150_e45888: f64 = if p.p9 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1219 = assign40150_e45888;
        locals.var_guard1219_rv = 0.0;
        if ((locals.var_guard1080 != 0.0) && (locals.var_guard1219 != 0.0)) {
            let assign40160_e45895: f64 = (locals.var_k1q1s__blk939 + locals.var_k1q1d__blk1004);
            let assign40160_e45896: f64 = (0.5 * assign40160_e45895);
            let assign40160_e45898: f64 = (assign40160_e45896 / locals.var_k1__blk932);
            (locals.var_qim_pd__blk1018, locals.var_qim_pd__blk1018_dn4, locals.var_qim_pd__blk1018_dn6, locals.var_qim_pd__blk1018_dn7, locals.var_qim_pd__blk1018_dn8, locals.var_qim_pd__blk1018_dn9, ) = (assign40160_e45898, ((((0.5 * (locals.var_k1q1s__blk939_dn4 + locals.var_k1q1d__blk1004_dn4)) * locals.var_k1__blk932) - (assign40160_e45896 * locals.var_k1__blk932_dn4)) / (locals.var_k1__blk932 * locals.var_k1__blk932)), ((((0.5 * (locals.var_k1q1s__blk939_dn6 + locals.var_k1q1d__blk1004_dn6)) * locals.var_k1__blk932) - (assign40160_e45896 * locals.var_k1__blk932_dn6)) / (locals.var_k1__blk932 * locals.var_k1__blk932)), ((((0.5 * (locals.var_k1q1s__blk939_dn7 + locals.var_k1q1d__blk1004_dn7)) * locals.var_k1__blk932) - (assign40160_e45896 * locals.var_k1__blk932_dn7)) / (locals.var_k1__blk932 * locals.var_k1__blk932)), ((((0.5 * (locals.var_k1q1s__blk939_dn8 + locals.var_k1q1d__blk1004_dn8)) * locals.var_k1__blk932) - (assign40160_e45896 * locals.var_k1__blk932_dn8)) / (locals.var_k1__blk932 * locals.var_k1__blk932)), ((((0.5 * (locals.var_k1q1s__blk939_dn9 + locals.var_k1q1d__blk1004_dn9)) * locals.var_k1__blk932) - (assign40160_e45896 * locals.var_k1__blk932_dn9)) / (locals.var_k1__blk932 * locals.var_k1__blk932)), );
            locals.var_qim_pd__blk1018_rv = 0.0;
        }
        if ((locals.var_guard1080 != 0.0) && (locals.var_guard1219 != 0.0)) {
            let assign40170_e45907: f64 = (locals.var_qim_pd__blk1018 + 1e-5);
            let assign40170_e45910: f64 = (locals.var_qim_pd__blk1018 - 1e-5);
            let assign40170_e45913: f64 = (locals.var_qim_pd__blk1018 - 1e-5);
            let assign40170_e45914: f64 = (assign40170_e45910 * assign40170_e45913);
            let assign40170_e45916: f64 = (assign40170_e45914 + 1.0);
            let assign40170_e45917: f64 = (assign40170_e45916).sqrt();
            let assign40170_e45918: f64 = (assign40170_e45907 + assign40170_e45917);
            let assign40170_e45919: f64 = (0.5 * assign40170_e45918);
            (locals.var_qim_pd__blk1018, locals.var_qim_pd__blk1018_dn4, locals.var_qim_pd__blk1018_dn6, locals.var_qim_pd__blk1018_dn7, locals.var_qim_pd__blk1018_dn8, locals.var_qim_pd__blk1018_dn9, ) = (assign40170_e45919, (0.5 * (locals.var_qim_pd__blk1018_dn4 + (((locals.var_qim_pd__blk1018_dn4 * assign40170_e45913) + (assign40170_e45910 * locals.var_qim_pd__blk1018_dn4)) / (2.0 * assign40170_e45917)))), (0.5 * (locals.var_qim_pd__blk1018_dn6 + (((locals.var_qim_pd__blk1018_dn6 * assign40170_e45913) + (assign40170_e45910 * locals.var_qim_pd__blk1018_dn6)) / (2.0 * assign40170_e45917)))), (0.5 * (locals.var_qim_pd__blk1018_dn7 + (((locals.var_qim_pd__blk1018_dn7 * assign40170_e45913) + (assign40170_e45910 * locals.var_qim_pd__blk1018_dn7)) / (2.0 * assign40170_e45917)))), (0.5 * (locals.var_qim_pd__blk1018_dn8 + (((locals.var_qim_pd__blk1018_dn8 * assign40170_e45913) + (assign40170_e45910 * locals.var_qim_pd__blk1018_dn8)) / (2.0 * assign40170_e45917)))), (0.5 * (locals.var_qim_pd__blk1018_dn9 + (((locals.var_qim_pd__blk1018_dn9 * assign40170_e45913) + (assign40170_e45910 * locals.var_qim_pd__blk1018_dn9)) / (2.0 * assign40170_e45917)))), );
            locals.var_qim_pd__blk1018_rv = 0.0;
        }
        if ((locals.var_guard1080 != 0.0) && (locals.var_guard1219 != 0.0)) {
            let assign40180_e45927: f64 = (locals.var_qim_pd__blk1018 / locals.var_inv_phit);
            let assign40180_e45930: f64 = (0.25 * locals.var_kp);
            let assign40180_e45932: f64 = (assign40180_e45930 * locals.var_kp);
            let assign40180_e45933: f64 = (assign40180_e45927 + assign40180_e45932);
            let assign40180_e45934: f64 = (assign40180_e45933).sqrt();
            let assign40180_e45937: f64 = (0.5 * locals.var_kp);
            let assign40180_e45938: f64 = (assign40180_e45934 - assign40180_e45937);
            (locals.var_temp0, locals.var_temp0_dn4, locals.var_temp0_dn6, locals.var_temp0_dn7, locals.var_temp0_dn8, locals.var_temp0_dn9, ) = (assign40180_e45938, ((((((locals.var_qim_pd__blk1018_dn4 * locals.var_inv_phit) - (locals.var_qim_pd__blk1018 * locals.var_inv_phit_dn4)) / (locals.var_inv_phit * locals.var_inv_phit)) + (((0.25 * locals.var_kp_dn4) * locals.var_kp) + (assign40180_e45930 * locals.var_kp_dn4))) / (2.0 * assign40180_e45934)) - (0.5 * locals.var_kp_dn4)), ((((((locals.var_qim_pd__blk1018_dn6 * locals.var_inv_phit) - (locals.var_qim_pd__blk1018 * locals.var_inv_phit_dn6)) / (locals.var_inv_phit * locals.var_inv_phit)) + (((0.25 * locals.var_kp_dn6) * locals.var_kp) + (assign40180_e45930 * locals.var_kp_dn6))) / (2.0 * assign40180_e45934)) - (0.5 * locals.var_kp_dn6)), ((((((locals.var_qim_pd__blk1018_dn7 * locals.var_inv_phit) - (locals.var_qim_pd__blk1018 * locals.var_inv_phit_dn7)) / (locals.var_inv_phit * locals.var_inv_phit)) + (((0.25 * locals.var_kp_dn7) * locals.var_kp) + (assign40180_e45930 * locals.var_kp_dn7))) / (2.0 * assign40180_e45934)) - (0.5 * locals.var_kp_dn7)), ((((((locals.var_qim_pd__blk1018_dn8 * locals.var_inv_phit) - (locals.var_qim_pd__blk1018 * locals.var_inv_phit_dn8)) / (locals.var_inv_phit * locals.var_inv_phit)) + (((0.25 * locals.var_kp_dn8) * locals.var_kp) + (assign40180_e45930 * locals.var_kp_dn8))) / (2.0 * assign40180_e45934)) - (0.5 * locals.var_kp_dn8)), ((((((locals.var_qim_pd__blk1018_dn9 * locals.var_inv_phit) - (locals.var_qim_pd__blk1018 * locals.var_inv_phit_dn9)) / (locals.var_inv_phit * locals.var_inv_phit)) + (((0.25 * locals.var_kp_dn9) * locals.var_kp) + (assign40180_e45930 * locals.var_kp_dn9))) / (2.0 * assign40180_e45934)) - (0.5 * locals.var_kp_dn9)), );
            locals.var_temp0_rv = 0.0;
        }
        if ((locals.var_guard1080 != 0.0) && (locals.var_guard1219 != 0.0)) {
            let assign40190_e45946: f64 = (locals.var_temp0).powf(2.0);
            let assign40190_e45948: f64 = (assign40190_e45946 * locals.var_inv_phit);
            (locals.var_xp_pd__blk1019, locals.var_xp_pd__blk1019_dn4, locals.var_xp_pd__blk1019_dn6, locals.var_xp_pd__blk1019_dn7, locals.var_xp_pd__blk1019_dn8, locals.var_xp_pd__blk1019_dn9, ) = (assign40190_e45948, ((if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((locals.var_temp0).powf(2.0 - 1.0) * locals.var_temp0_dn4)) } } else { (assign40190_e45946 * (2.0 * (locals.var_temp0_dn4 / locals.var_temp0))) } * locals.var_inv_phit) + (assign40190_e45946 * locals.var_inv_phit_dn4)), ((if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((locals.var_temp0).powf(2.0 - 1.0) * locals.var_temp0_dn6)) } } else { (assign40190_e45946 * (2.0 * (locals.var_temp0_dn6 / locals.var_temp0))) } * locals.var_inv_phit) + (assign40190_e45946 * locals.var_inv_phit_dn6)), ((if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((locals.var_temp0).powf(2.0 - 1.0) * locals.var_temp0_dn7)) } } else { (assign40190_e45946 * (2.0 * (locals.var_temp0_dn7 / locals.var_temp0))) } * locals.var_inv_phit) + (assign40190_e45946 * locals.var_inv_phit_dn7)), ((if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((locals.var_temp0).powf(2.0 - 1.0) * locals.var_temp0_dn8)) } } else { (assign40190_e45946 * (2.0 * (locals.var_temp0_dn8 / locals.var_temp0))) } * locals.var_inv_phit) + (assign40190_e45946 * locals.var_inv_phit_dn8)), ((if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((locals.var_temp0).powf(2.0 - 1.0) * locals.var_temp0_dn9)) } } else { (assign40190_e45946 * (2.0 * (locals.var_temp0_dn9 / locals.var_temp0))) } * locals.var_inv_phit) + (assign40190_e45946 * locals.var_inv_phit_dn9)), );
            locals.var_xp_pd__blk1019_rv = 0.0;
        }
        if ((locals.var_guard1080 != 0.0) && (locals.var_guard1219 != 0.0)) {
            let assign40200_e45957: f64 = (locals.var_xp_pd__blk1019 / locals.var_qim_pd__blk1018);
            let assign40200_e45958: f64 = (1.0 - assign40200_e45957);
            (locals.var_ratio_pd__blk1020, locals.var_ratio_pd__blk1020_dn4, locals.var_ratio_pd__blk1020_dn6, locals.var_ratio_pd__blk1020_dn7, locals.var_ratio_pd__blk1020_dn8, locals.var_ratio_pd__blk1020_dn9, ) = (assign40200_e45958, (-(((locals.var_xp_pd__blk1019_dn4 * locals.var_qim_pd__blk1018) - (locals.var_xp_pd__blk1019 * locals.var_qim_pd__blk1018_dn4)) / (locals.var_qim_pd__blk1018 * locals.var_qim_pd__blk1018))), (-(((locals.var_xp_pd__blk1019_dn6 * locals.var_qim_pd__blk1018) - (locals.var_xp_pd__blk1019 * locals.var_qim_pd__blk1018_dn6)) / (locals.var_qim_pd__blk1018 * locals.var_qim_pd__blk1018))), (-(((locals.var_xp_pd__blk1019_dn7 * locals.var_qim_pd__blk1018) - (locals.var_xp_pd__blk1019 * locals.var_qim_pd__blk1018_dn7)) / (locals.var_qim_pd__blk1018 * locals.var_qim_pd__blk1018))), (-(((locals.var_xp_pd__blk1019_dn8 * locals.var_qim_pd__blk1018) - (locals.var_xp_pd__blk1019 * locals.var_qim_pd__blk1018_dn8)) / (locals.var_qim_pd__blk1018 * locals.var_qim_pd__blk1018))), (-(((locals.var_xp_pd__blk1019_dn9 * locals.var_qim_pd__blk1018) - (locals.var_xp_pd__blk1019 * locals.var_qim_pd__blk1018_dn9)) / (locals.var_qim_pd__blk1018 * locals.var_qim_pd__blk1018))), );
            locals.var_ratio_pd__blk1020_rv = 0.0;
        }
        let assign40210_e45963: f64 = (locals.var_k1q1d__blk1004 / 2.0);
        let assign40210_e45965: f64 = if assign40210_e45963 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard1220 = assign40210_e45965;
        locals.var_guard1220_rv = 0.0;
    }
    pub(super) fn stamp_reactive_block_58(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        if ((locals.var_guard1080 != 0.0) && (locals.var_guard1220 != 0.0)) {
            let assign40220_e45972: f64 = (locals.var_k1q1d__blk1004 / 2.0);
            let assign40220_e45973: f64 = (assign40220_e45972).exp();
            let assign40220_e45974: f64 = (1.0 + assign40220_e45973);
            let assign40220_e45975: f64 = (assign40220_e45974).ln();
            (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9, ) = (assign40220_e45975, ((assign40220_e45973 * (locals.var_k1q1d__blk1004_dn4 / 2.0)) / assign40220_e45974), ((assign40220_e45973 * (locals.var_k1q1d__blk1004_dn6 / 2.0)) / assign40220_e45974), ((assign40220_e45973 * (locals.var_k1q1d__blk1004_dn7 / 2.0)) / assign40220_e45974), ((assign40220_e45973 * (locals.var_k1q1d__blk1004_dn8 / 2.0)) / assign40220_e45974), ((assign40220_e45973 * (locals.var_k1q1d__blk1004_dn9 / 2.0)) / assign40220_e45974), );
            locals.var_temp1_rv = 0.0;
        }
        if ((locals.var_guard1080 != 0.0) && (locals.var_guard1220 == 0.0)) {
            let assign40230_e45984: f64 = (locals.var_k1q1d__blk1004 / 2.0);
            (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9, ) = (assign40230_e45984, (locals.var_k1q1d__blk1004_dn4 / 2.0), (locals.var_k1q1d__blk1004_dn6 / 2.0), (locals.var_k1q1d__blk1004_dn7 / 2.0), (locals.var_k1q1d__blk1004_dn8 / 2.0), (locals.var_k1q1d__blk1004_dn9 / 2.0), );
            locals.var_temp1_rv = 0.0;
        }
        if (locals.var_guard1080 != 0.0) {
            let assign40240_e45990: f64 = (2.0 * locals.var_temp1);
            (locals.var_esurf1d__blk1021, locals.var_esurf1d__blk1021_dn4, locals.var_esurf1d__blk1021_dn6, locals.var_esurf1d__blk1021_dn7, locals.var_esurf1d__blk1021_dn8, locals.var_esurf1d__blk1021_dn9, ) = (assign40240_e45990, (2.0 * locals.var_temp1_dn4), (2.0 * locals.var_temp1_dn6), (2.0 * locals.var_temp1_dn7), (2.0 * locals.var_temp1_dn8), (2.0 * locals.var_temp1_dn9), );
            locals.var_esurf1d__blk1021_rv = 0.0;
        }
        let assign40250_e45995: f64 = (locals.var_k2q2d__blk1005 / 2.0);
        let assign40250_e45997: f64 = if assign40250_e45995 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard1221 = assign40250_e45997;
        locals.var_guard1221_rv = 0.0;
        if ((locals.var_guard1080 != 0.0) && (locals.var_guard1221 != 0.0)) {
            let assign40260_e46004: f64 = (locals.var_k2q2d__blk1005 / 2.0);
            let assign40260_e46005: f64 = (assign40260_e46004).exp();
            let assign40260_e46006: f64 = (1.0 + assign40260_e46005);
            let assign40260_e46007: f64 = (assign40260_e46006).ln();
            (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9, ) = (assign40260_e46007, ((assign40260_e46005 * (locals.var_k2q2d__blk1005_dn4 / 2.0)) / assign40260_e46006), ((assign40260_e46005 * (locals.var_k2q2d__blk1005_dn6 / 2.0)) / assign40260_e46006), ((assign40260_e46005 * (locals.var_k2q2d__blk1005_dn7 / 2.0)) / assign40260_e46006), ((assign40260_e46005 * (locals.var_k2q2d__blk1005_dn8 / 2.0)) / assign40260_e46006), ((assign40260_e46005 * (locals.var_k2q2d__blk1005_dn9 / 2.0)) / assign40260_e46006), );
            locals.var_temp2_rv = 0.0;
        }
        if ((locals.var_guard1080 != 0.0) && (locals.var_guard1221 == 0.0)) {
            let assign40270_e46016: f64 = (locals.var_k2q2d__blk1005 / 2.0);
            (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9, ) = (assign40270_e46016, (locals.var_k2q2d__blk1005_dn4 / 2.0), (locals.var_k2q2d__blk1005_dn6 / 2.0), (locals.var_k2q2d__blk1005_dn7 / 2.0), (locals.var_k2q2d__blk1005_dn8 / 2.0), (locals.var_k2q2d__blk1005_dn9 / 2.0), );
            locals.var_temp2_rv = 0.0;
        }
        if (locals.var_guard1080 != 0.0) {
            let assign40280_e46022: f64 = (2.0 * locals.var_temp2);
            (locals.var_esurf2d__blk1022, locals.var_esurf2d__blk1022_dn4, locals.var_esurf2d__blk1022_dn6, locals.var_esurf2d__blk1022_dn7, locals.var_esurf2d__blk1022_dn8, locals.var_esurf2d__blk1022_dn9, ) = (assign40280_e46022, (2.0 * locals.var_temp2_dn4), (2.0 * locals.var_temp2_dn6), (2.0 * locals.var_temp2_dn7), (2.0 * locals.var_temp2_dn8), (2.0 * locals.var_temp2_dn9), );
            locals.var_esurf2d__blk1022_rv = 0.0;
        }
        if (locals.var_guard1080 != 0.0) {
            let assign40290_e46028: f64 = (locals.var_esurf2d__blk1022 - locals.var_k2q2d__blk1005);
            (locals.var_ecpl1d__blk1023, locals.var_ecpl1d__blk1023_dn4, locals.var_ecpl1d__blk1023_dn6, locals.var_ecpl1d__blk1023_dn7, locals.var_ecpl1d__blk1023_dn8, locals.var_ecpl1d__blk1023_dn9, ) = (assign40290_e46028, (locals.var_esurf2d__blk1022_dn4 - locals.var_k2q2d__blk1005_dn4), (locals.var_esurf2d__blk1022_dn6 - locals.var_k2q2d__blk1005_dn6), (locals.var_esurf2d__blk1022_dn7 - locals.var_k2q2d__blk1005_dn7), (locals.var_esurf2d__blk1022_dn8 - locals.var_k2q2d__blk1005_dn8), (locals.var_esurf2d__blk1022_dn9 - locals.var_k2q2d__blk1005_dn9), );
            locals.var_ecpl1d__blk1023_rv = 0.0;
        }
        if (locals.var_guard1080 != 0.0) {
            let assign40300_e46034: f64 = (locals.var_esurf1d__blk1021 - locals.var_k1q1d__blk1004);
            (locals.var_ecpl2d__blk1024, locals.var_ecpl2d__blk1024_dn4, locals.var_ecpl2d__blk1024_dn6, locals.var_ecpl2d__blk1024_dn7, locals.var_ecpl2d__blk1024_dn8, locals.var_ecpl2d__blk1024_dn9, ) = (assign40300_e46034, (locals.var_esurf1d__blk1021_dn4 - locals.var_k1q1d__blk1004_dn4), (locals.var_esurf1d__blk1021_dn6 - locals.var_k1q1d__blk1004_dn6), (locals.var_esurf1d__blk1021_dn7 - locals.var_k1q1d__blk1004_dn7), (locals.var_esurf1d__blk1021_dn8 - locals.var_k1q1d__blk1004_dn8), (locals.var_esurf1d__blk1021_dn9 - locals.var_k1q1d__blk1004_dn9), );
            locals.var_ecpl2d__blk1024_rv = 0.0;
        }
        if (locals.var_guard1080 != 0.0) {
            let assign40310_e46040: f64 = (locals.var_eta_mu * locals.var_esurf1d__blk1021);
            let assign40310_e46043: f64 = (locals.var_one_m_eta * locals.var_ecpl1d__blk1023);
            let assign40310_e46044: f64 = (assign40310_e46040 + assign40310_e46043);
            (locals.var_eeff1d__blk1025, locals.var_eeff1d__blk1025_dn4, locals.var_eeff1d__blk1025_dn6, locals.var_eeff1d__blk1025_dn7, locals.var_eeff1d__blk1025_dn8, locals.var_eeff1d__blk1025_dn9, ) = (assign40310_e46044, ((locals.var_eta_mu * locals.var_esurf1d__blk1021_dn4) + (locals.var_one_m_eta * locals.var_ecpl1d__blk1023_dn4)), ((locals.var_eta_mu * locals.var_esurf1d__blk1021_dn6) + (locals.var_one_m_eta * locals.var_ecpl1d__blk1023_dn6)), ((locals.var_eta_mu * locals.var_esurf1d__blk1021_dn7) + (locals.var_one_m_eta * locals.var_ecpl1d__blk1023_dn7)), ((locals.var_eta_mu * locals.var_esurf1d__blk1021_dn8) + (locals.var_one_m_eta * locals.var_ecpl1d__blk1023_dn8)), ((locals.var_eta_mu * locals.var_esurf1d__blk1021_dn9) + (locals.var_one_m_eta * locals.var_ecpl1d__blk1023_dn9)), );
            locals.var_eeff1d__blk1025_rv = 0.0;
        }
        if (locals.var_guard1080 != 0.0) {
            let assign40320_e46050: f64 = (locals.var_eta_mu * locals.var_esurf2d__blk1022);
            let assign40320_e46053: f64 = (locals.var_one_m_eta * locals.var_ecpl2d__blk1024);
            let assign40320_e46054: f64 = (assign40320_e46050 + assign40320_e46053);
            (locals.var_eeff2d__blk1026, locals.var_eeff2d__blk1026_dn4, locals.var_eeff2d__blk1026_dn6, locals.var_eeff2d__blk1026_dn7, locals.var_eeff2d__blk1026_dn8, locals.var_eeff2d__blk1026_dn9, ) = (assign40320_e46054, ((locals.var_eta_mu * locals.var_esurf2d__blk1022_dn4) + (locals.var_one_m_eta * locals.var_ecpl2d__blk1024_dn4)), ((locals.var_eta_mu * locals.var_esurf2d__blk1022_dn6) + (locals.var_one_m_eta * locals.var_ecpl2d__blk1024_dn6)), ((locals.var_eta_mu * locals.var_esurf2d__blk1022_dn7) + (locals.var_one_m_eta * locals.var_ecpl2d__blk1024_dn7)), ((locals.var_eta_mu * locals.var_esurf2d__blk1022_dn8) + (locals.var_one_m_eta * locals.var_ecpl2d__blk1024_dn8)), ((locals.var_eta_mu * locals.var_esurf2d__blk1022_dn9) + (locals.var_one_m_eta * locals.var_ecpl2d__blk1024_dn9)), );
            locals.var_eeff2d__blk1026_rv = 0.0;
        }
        if (locals.var_guard1080 != 0.0) {
            let assign40330_e46061: f64 = (locals.var_esurf1s__blk952 + locals.var_esurf1d__blk1021);
            let assign40330_e46062: f64 = (0.5 * assign40330_e46061);
            (locals.var_esurf1__blk1027, locals.var_esurf1__blk1027_dn4, locals.var_esurf1__blk1027_dn6, locals.var_esurf1__blk1027_dn7, locals.var_esurf1__blk1027_dn8, locals.var_esurf1__blk1027_dn9, ) = (assign40330_e46062, (0.5 * (locals.var_esurf1s__blk952_dn4 + locals.var_esurf1d__blk1021_dn4)), (0.5 * (locals.var_esurf1s__blk952_dn6 + locals.var_esurf1d__blk1021_dn6)), (0.5 * (locals.var_esurf1s__blk952_dn7 + locals.var_esurf1d__blk1021_dn7)), (0.5 * (locals.var_esurf1s__blk952_dn8 + locals.var_esurf1d__blk1021_dn8)), (0.5 * (locals.var_esurf1s__blk952_dn9 + locals.var_esurf1d__blk1021_dn9)), );
            locals.var_esurf1__blk1027_rv = 0.0;
        }
        if (locals.var_guard1080 != 0.0) {
            let assign40340_e46069: f64 = (locals.var_esurf2s__blk953 + locals.var_esurf2d__blk1022);
            let assign40340_e46070: f64 = (0.5 * assign40340_e46069);
            (locals.var_esurf2__blk1028, locals.var_esurf2__blk1028_dn4, locals.var_esurf2__blk1028_dn6, locals.var_esurf2__blk1028_dn7, locals.var_esurf2__blk1028_dn8, locals.var_esurf2__blk1028_dn9, ) = (assign40340_e46070, (0.5 * (locals.var_esurf2s__blk953_dn4 + locals.var_esurf2d__blk1022_dn4)), (0.5 * (locals.var_esurf2s__blk953_dn6 + locals.var_esurf2d__blk1022_dn6)), (0.5 * (locals.var_esurf2s__blk953_dn7 + locals.var_esurf2d__blk1022_dn7)), (0.5 * (locals.var_esurf2s__blk953_dn8 + locals.var_esurf2d__blk1022_dn8)), (0.5 * (locals.var_esurf2s__blk953_dn9 + locals.var_esurf2d__blk1022_dn9)), );
            locals.var_esurf2__blk1028_rv = 0.0;
        }
        if (locals.var_guard1080 != 0.0) {
            let assign40350_e46077: f64 = (locals.var_esurf1__blk1027 + locals.var_esurf2__blk1028);
            let assign40350_e46078: f64 = (1.0 / assign40350_e46077);
            (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign40350_e46078, (-((locals.var_esurf1__blk1027_dn4 + locals.var_esurf2__blk1028_dn4) / (assign40350_e46077 * assign40350_e46077))), (-((locals.var_esurf1__blk1027_dn6 + locals.var_esurf2__blk1028_dn6) / (assign40350_e46077 * assign40350_e46077))), (-((locals.var_esurf1__blk1027_dn7 + locals.var_esurf2__blk1028_dn7) / (assign40350_e46077 * assign40350_e46077))), (-((locals.var_esurf1__blk1027_dn8 + locals.var_esurf2__blk1028_dn8) / (assign40350_e46077 * assign40350_e46077))), (-((locals.var_esurf1__blk1027_dn9 + locals.var_esurf2__blk1028_dn9) / (assign40350_e46077 * assign40350_e46077))), );
            locals.var_temp_rv = 0.0;
        }
        if (locals.var_guard1080 != 0.0) {
            let assign40360_e46084: f64 = (locals.var_qim__blk1016 * locals.var_esurf1__blk1027);
            let assign40360_e46086: f64 = (assign40360_e46084 * locals.var_temp);
            (locals.var_qi1m__blk1029, locals.var_qi1m__blk1029_dn4, locals.var_qi1m__blk1029_dn6, locals.var_qi1m__blk1029_dn7, locals.var_qi1m__blk1029_dn8, locals.var_qi1m__blk1029_dn9, ) = (assign40360_e46086, ((((locals.var_qim__blk1016_dn4 * locals.var_esurf1__blk1027) + (locals.var_qim__blk1016 * locals.var_esurf1__blk1027_dn4)) * locals.var_temp) + (assign40360_e46084 * locals.var_temp_dn4)), ((((locals.var_qim__blk1016_dn6 * locals.var_esurf1__blk1027) + (locals.var_qim__blk1016 * locals.var_esurf1__blk1027_dn6)) * locals.var_temp) + (assign40360_e46084 * locals.var_temp_dn6)), ((((locals.var_qim__blk1016_dn7 * locals.var_esurf1__blk1027) + (locals.var_qim__blk1016 * locals.var_esurf1__blk1027_dn7)) * locals.var_temp) + (assign40360_e46084 * locals.var_temp_dn7)), ((((locals.var_qim__blk1016_dn8 * locals.var_esurf1__blk1027) + (locals.var_qim__blk1016 * locals.var_esurf1__blk1027_dn8)) * locals.var_temp) + (assign40360_e46084 * locals.var_temp_dn8)), ((((locals.var_qim__blk1016_dn9 * locals.var_esurf1__blk1027) + (locals.var_qim__blk1016 * locals.var_esurf1__blk1027_dn9)) * locals.var_temp) + (assign40360_e46084 * locals.var_temp_dn9)), );
            locals.var_qi1m__blk1029_rv = 0.0;
        }
        if (locals.var_guard1080 != 0.0) {
            let assign40370_e46092: f64 = (locals.var_qim__blk1016 * locals.var_esurf2__blk1028);
            let assign40370_e46094: f64 = (assign40370_e46092 * locals.var_temp);
            (locals.var_qi2m__blk1030, locals.var_qi2m__blk1030_dn4, locals.var_qi2m__blk1030_dn6, locals.var_qi2m__blk1030_dn7, locals.var_qi2m__blk1030_dn8, locals.var_qi2m__blk1030_dn9, ) = (assign40370_e46094, ((((locals.var_qim__blk1016_dn4 * locals.var_esurf2__blk1028) + (locals.var_qim__blk1016 * locals.var_esurf2__blk1028_dn4)) * locals.var_temp) + (assign40370_e46092 * locals.var_temp_dn4)), ((((locals.var_qim__blk1016_dn6 * locals.var_esurf2__blk1028) + (locals.var_qim__blk1016 * locals.var_esurf2__blk1028_dn6)) * locals.var_temp) + (assign40370_e46092 * locals.var_temp_dn6)), ((((locals.var_qim__blk1016_dn7 * locals.var_esurf2__blk1028) + (locals.var_qim__blk1016 * locals.var_esurf2__blk1028_dn7)) * locals.var_temp) + (assign40370_e46092 * locals.var_temp_dn7)), ((((locals.var_qim__blk1016_dn8 * locals.var_esurf2__blk1028) + (locals.var_qim__blk1016 * locals.var_esurf2__blk1028_dn8)) * locals.var_temp) + (assign40370_e46092 * locals.var_temp_dn8)), ((((locals.var_qim__blk1016_dn9 * locals.var_esurf2__blk1028) + (locals.var_qim__blk1016 * locals.var_esurf2__blk1028_dn9)) * locals.var_temp) + (assign40370_e46092 * locals.var_temp_dn9)), );
            locals.var_qi2m__blk1030_rv = 0.0;
        }
        if (locals.var_guard1080 != 0.0) {
            let assign40380_e46101: f64 = (locals.var_ecpl1s__blk954 + locals.var_ecpl1d__blk1023);
            let assign40380_e46102: f64 = (0.5 * assign40380_e46101);
            (locals.var_ecpl1__blk1031, locals.var_ecpl1__blk1031_dn4, locals.var_ecpl1__blk1031_dn6, locals.var_ecpl1__blk1031_dn7, locals.var_ecpl1__blk1031_dn8, locals.var_ecpl1__blk1031_dn9, ) = (assign40380_e46102, (0.5 * (locals.var_ecpl1s__blk954_dn4 + locals.var_ecpl1d__blk1023_dn4)), (0.5 * (locals.var_ecpl1s__blk954_dn6 + locals.var_ecpl1d__blk1023_dn6)), (0.5 * (locals.var_ecpl1s__blk954_dn7 + locals.var_ecpl1d__blk1023_dn7)), (0.5 * (locals.var_ecpl1s__blk954_dn8 + locals.var_ecpl1d__blk1023_dn8)), (0.5 * (locals.var_ecpl1s__blk954_dn9 + locals.var_ecpl1d__blk1023_dn9)), );
            locals.var_ecpl1__blk1031_rv = 0.0;
        }
        if (locals.var_guard1080 != 0.0) {
            let assign40390_e46109: f64 = (locals.var_ecpl2s__blk955 + locals.var_ecpl2d__blk1024);
            let assign40390_e46110: f64 = (0.5 * assign40390_e46109);
            (locals.var_ecpl2__blk1032, locals.var_ecpl2__blk1032_dn4, locals.var_ecpl2__blk1032_dn6, locals.var_ecpl2__blk1032_dn7, locals.var_ecpl2__blk1032_dn8, locals.var_ecpl2__blk1032_dn9, ) = (assign40390_e46110, (0.5 * (locals.var_ecpl2s__blk955_dn4 + locals.var_ecpl2d__blk1024_dn4)), (0.5 * (locals.var_ecpl2s__blk955_dn6 + locals.var_ecpl2d__blk1024_dn6)), (0.5 * (locals.var_ecpl2s__blk955_dn7 + locals.var_ecpl2d__blk1024_dn7)), (0.5 * (locals.var_ecpl2s__blk955_dn8 + locals.var_ecpl2d__blk1024_dn8)), (0.5 * (locals.var_ecpl2s__blk955_dn9 + locals.var_ecpl2d__blk1024_dn9)), );
            locals.var_ecpl2__blk1032_rv = 0.0;
        }
        if (locals.var_guard1080 != 0.0) {
            let assign40400_e46117: f64 = (locals.var_eeff1s__blk956 + locals.var_eeff1d__blk1025);
            let assign40400_e46118: f64 = (0.5 * assign40400_e46117);
            (locals.var_eeff1__blk1033, locals.var_eeff1__blk1033_dn4, locals.var_eeff1__blk1033_dn6, locals.var_eeff1__blk1033_dn7, locals.var_eeff1__blk1033_dn8, locals.var_eeff1__blk1033_dn9, ) = (assign40400_e46118, (0.5 * (locals.var_eeff1s__blk956_dn4 + locals.var_eeff1d__blk1025_dn4)), (0.5 * (locals.var_eeff1s__blk956_dn6 + locals.var_eeff1d__blk1025_dn6)), (0.5 * (locals.var_eeff1s__blk956_dn7 + locals.var_eeff1d__blk1025_dn7)), (0.5 * (locals.var_eeff1s__blk956_dn8 + locals.var_eeff1d__blk1025_dn8)), (0.5 * (locals.var_eeff1s__blk956_dn9 + locals.var_eeff1d__blk1025_dn9)), );
            locals.var_eeff1__blk1033_rv = 0.0;
        }
        if (locals.var_guard1080 != 0.0) {
            let assign40410_e46125: f64 = (locals.var_eeff2s__blk957 + locals.var_eeff2d__blk1026);
            let assign40410_e46126: f64 = (0.5 * assign40410_e46125);
            (locals.var_eeff2__blk1034, locals.var_eeff2__blk1034_dn4, locals.var_eeff2__blk1034_dn6, locals.var_eeff2__blk1034_dn7, locals.var_eeff2__blk1034_dn8, locals.var_eeff2__blk1034_dn9, ) = (assign40410_e46126, (0.5 * (locals.var_eeff2s__blk957_dn4 + locals.var_eeff2d__blk1026_dn4)), (0.5 * (locals.var_eeff2s__blk957_dn6 + locals.var_eeff2d__blk1026_dn6)), (0.5 * (locals.var_eeff2s__blk957_dn7 + locals.var_eeff2d__blk1026_dn7)), (0.5 * (locals.var_eeff2s__blk957_dn8 + locals.var_eeff2d__blk1026_dn8)), (0.5 * (locals.var_eeff2s__blk957_dn9 + locals.var_eeff2d__blk1026_dn9)), );
            locals.var_eeff2__blk1034_rv = 0.0;
        }
        if (locals.var_guard1080 != 0.0) {
            let assign40420_e46132: f64 = (locals.var_esurf1__blk1027 * locals.var_betn1_t);
            let assign40420_e46135: f64 = (locals.var_stbet_i * locals.var_lnrtn);
            let assign40420_e46136: f64 = (assign40420_e46135).exp();
            let assign40420_e46137: f64 = (assign40420_e46132 * assign40420_e46136);
            let assign40420_e46139: f64 = (assign40420_e46137 * locals.var_ratio_pd__blk1020);
            (locals.var_c1__blk1035, locals.var_c1__blk1035_dn4, locals.var_c1__blk1035_dn6, locals.var_c1__blk1035_dn7, locals.var_c1__blk1035_dn8, locals.var_c1__blk1035_dn9, ) = (assign40420_e46139, ((((((locals.var_esurf1__blk1027_dn4 * locals.var_betn1_t) + (locals.var_esurf1__blk1027 * locals.var_betn1_t_dn4)) * assign40420_e46136) + (assign40420_e46132 * (assign40420_e46136 * (locals.var_stbet_i * locals.var_lnrtn_dn4)))) * locals.var_ratio_pd__blk1020) + (assign40420_e46137 * locals.var_ratio_pd__blk1020_dn4)), ((((((locals.var_esurf1__blk1027_dn6 * locals.var_betn1_t) + (locals.var_esurf1__blk1027 * locals.var_betn1_t_dn6)) * assign40420_e46136) + (assign40420_e46132 * (assign40420_e46136 * (locals.var_stbet_i * locals.var_lnrtn_dn6)))) * locals.var_ratio_pd__blk1020) + (assign40420_e46137 * locals.var_ratio_pd__blk1020_dn6)), ((((((locals.var_esurf1__blk1027_dn7 * locals.var_betn1_t) + (locals.var_esurf1__blk1027 * locals.var_betn1_t_dn7)) * assign40420_e46136) + (assign40420_e46132 * (assign40420_e46136 * (locals.var_stbet_i * locals.var_lnrtn_dn7)))) * locals.var_ratio_pd__blk1020) + (assign40420_e46137 * locals.var_ratio_pd__blk1020_dn7)), ((((((locals.var_esurf1__blk1027_dn8 * locals.var_betn1_t) + (locals.var_esurf1__blk1027 * locals.var_betn1_t_dn8)) * assign40420_e46136) + (assign40420_e46132 * (assign40420_e46136 * (locals.var_stbet_i * locals.var_lnrtn_dn8)))) * locals.var_ratio_pd__blk1020) + (assign40420_e46137 * locals.var_ratio_pd__blk1020_dn8)), ((((((locals.var_esurf1__blk1027_dn9 * locals.var_betn1_t) + (locals.var_esurf1__blk1027 * locals.var_betn1_t_dn9)) * assign40420_e46136) + (assign40420_e46132 * (assign40420_e46136 * (locals.var_stbet_i * locals.var_lnrtn_dn9)))) * locals.var_ratio_pd__blk1020) + (assign40420_e46137 * locals.var_ratio_pd__blk1020_dn9)), );
            locals.var_c1__blk1035_rv = 0.0;
        }
        if (locals.var_guard1080 != 0.0) {
            let assign40430_e46145: f64 = (locals.var_esurf2__blk1028 * locals.var_betn2_t);
            let assign40430_e46148: f64 = (locals.var_stbet_i * locals.var_lnrtn);
            let assign40430_e46149: f64 = (assign40430_e46148).exp();
            let assign40430_e46150: f64 = (assign40430_e46145 * assign40430_e46149);
            (locals.var_c2__blk1036, locals.var_c2__blk1036_dn4, locals.var_c2__blk1036_dn6, locals.var_c2__blk1036_dn7, locals.var_c2__blk1036_dn8, locals.var_c2__blk1036_dn9, ) = (assign40430_e46150, ((((locals.var_esurf2__blk1028_dn4 * locals.var_betn2_t) + (locals.var_esurf2__blk1028 * locals.var_betn2_t_dn4)) * assign40430_e46149) + (assign40430_e46145 * (assign40430_e46149 * (locals.var_stbet_i * locals.var_lnrtn_dn4)))), ((((locals.var_esurf2__blk1028_dn6 * locals.var_betn2_t) + (locals.var_esurf2__blk1028 * locals.var_betn2_t_dn6)) * assign40430_e46149) + (assign40430_e46145 * (assign40430_e46149 * (locals.var_stbet_i * locals.var_lnrtn_dn6)))), ((((locals.var_esurf2__blk1028_dn7 * locals.var_betn2_t) + (locals.var_esurf2__blk1028 * locals.var_betn2_t_dn7)) * assign40430_e46149) + (assign40430_e46145 * (assign40430_e46149 * (locals.var_stbet_i * locals.var_lnrtn_dn7)))), ((((locals.var_esurf2__blk1028_dn8 * locals.var_betn2_t) + (locals.var_esurf2__blk1028 * locals.var_betn2_t_dn8)) * assign40430_e46149) + (assign40430_e46145 * (assign40430_e46149 * (locals.var_stbet_i * locals.var_lnrtn_dn8)))), ((((locals.var_esurf2__blk1028_dn9 * locals.var_betn2_t) + (locals.var_esurf2__blk1028 * locals.var_betn2_t_dn9)) * assign40430_e46149) + (assign40430_e46145 * (assign40430_e46149 * (locals.var_stbet_i * locals.var_lnrtn_dn9)))), );
            locals.var_c2__blk1036_rv = 0.0;
        }
        if (locals.var_guard1080 != 0.0) {
            let assign40440_e46156: f64 = (locals.var_c1__blk1035 + locals.var_c2__blk1036);
            (locals.var_csum__blk1037, locals.var_csum__blk1037_dn4, locals.var_csum__blk1037_dn6, locals.var_csum__blk1037_dn7, locals.var_csum__blk1037_dn8, locals.var_csum__blk1037_dn9, ) = (assign40440_e46156, (locals.var_c1__blk1035_dn4 + locals.var_c2__blk1036_dn4), (locals.var_c1__blk1035_dn6 + locals.var_c2__blk1036_dn6), (locals.var_c1__blk1035_dn7 + locals.var_c2__blk1036_dn7), (locals.var_c1__blk1035_dn8 + locals.var_c2__blk1036_dn8), (locals.var_c1__blk1035_dn9 + locals.var_c2__blk1036_dn9), );
            locals.var_csum__blk1037_rv = 0.0;
        }
        if (locals.var_guard1080 != 0.0) {
            let assign40450_e46164: f64 = (locals.var_xcorb_i * locals.var_ecpl2__blk1032);
            let assign40450_e46165: f64 = (locals.var_ecpl1__blk1031 + assign40450_e46164);
            let assign40450_e46166: f64 = (locals.var_xcor_i * assign40450_e46165);
            (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9, ) = (assign40450_e46166, ((locals.var_xcor_i_dn4 * assign40450_e46165) + (locals.var_xcor_i * (locals.var_ecpl1__blk1031_dn4 + (locals.var_xcorb_i * locals.var_ecpl2__blk1032_dn4)))), ((locals.var_xcor_i_dn6 * assign40450_e46165) + (locals.var_xcor_i * (locals.var_ecpl1__blk1031_dn6 + (locals.var_xcorb_i * locals.var_ecpl2__blk1032_dn6)))), ((locals.var_xcor_i_dn7 * assign40450_e46165) + (locals.var_xcor_i * (locals.var_ecpl1__blk1031_dn7 + (locals.var_xcorb_i * locals.var_ecpl2__blk1032_dn7)))), ((locals.var_xcor_i_dn8 * assign40450_e46165) + (locals.var_xcor_i * (locals.var_ecpl1__blk1031_dn8 + (locals.var_xcorb_i * locals.var_ecpl2__blk1032_dn8)))), ((locals.var_xcor_i_dn9 * assign40450_e46165) + (locals.var_xcor_i * (locals.var_ecpl1__blk1031_dn9 + (locals.var_xcorb_i * locals.var_ecpl2__blk1032_dn9)))), );
            locals.var_temp1_rv = 0.0;
        }
        if (locals.var_guard1080 != 0.0) {
            let assign40460_e46173: f64 = (1.0 + locals.var_temp1);
            let assign40460_e46175: f64 = assign40460_e46173;
            let assign40460_e46178: f64 = (1.0 + locals.var_temp1);
            let assign40460_e46180: f64 = assign40460_e46178;
            let assign40460_e46183: f64 = (1.0 + locals.var_temp1);
            let assign40460_e46185: f64 = assign40460_e46183;
            let assign40460_e46186: f64 = (assign40460_e46180 * assign40460_e46185);
            let assign40460_e46188: f64 = (assign40460_e46186 + 0.01);
            let assign40460_e46189: f64 = (assign40460_e46188).sqrt();
            let assign40460_e46190: f64 = (assign40460_e46175 + assign40460_e46189);
            let assign40460_e46191: f64 = (0.5 * assign40460_e46190);
            (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9, ) = (assign40460_e46191, (0.5 * (locals.var_temp1_dn4 + (((locals.var_temp1_dn4 * assign40460_e46185) + (assign40460_e46180 * locals.var_temp1_dn4)) / (2.0 * assign40460_e46189)))), (0.5 * (locals.var_temp1_dn6 + (((locals.var_temp1_dn6 * assign40460_e46185) + (assign40460_e46180 * locals.var_temp1_dn6)) / (2.0 * assign40460_e46189)))), (0.5 * (locals.var_temp1_dn7 + (((locals.var_temp1_dn7 * assign40460_e46185) + (assign40460_e46180 * locals.var_temp1_dn7)) / (2.0 * assign40460_e46189)))), (0.5 * (locals.var_temp1_dn8 + (((locals.var_temp1_dn8 * assign40460_e46185) + (assign40460_e46180 * locals.var_temp1_dn8)) / (2.0 * assign40460_e46189)))), (0.5 * (locals.var_temp1_dn9 + (((locals.var_temp1_dn9 * assign40460_e46185) + (assign40460_e46180 * locals.var_temp1_dn9)) / (2.0 * assign40460_e46189)))), );
            locals.var_temp2_rv = 0.0;
        }
        if (locals.var_guard1080 != 0.0) {
            let assign40470_e46199: f64 = (0.2 * locals.var_temp1);
            let assign40470_e46200: f64 = (1.0 + assign40470_e46199);
            let assign40470_e46202: f64 = assign40470_e46200;
            let assign40470_e46206: f64 = (0.2 * locals.var_temp1);
            let assign40470_e46207: f64 = (1.0 + assign40470_e46206);
            let assign40470_e46209: f64 = assign40470_e46207;
            let assign40470_e46213: f64 = (0.2 * locals.var_temp1);
            let assign40470_e46214: f64 = (1.0 + assign40470_e46213);
            let assign40470_e46216: f64 = assign40470_e46214;
            let assign40470_e46217: f64 = (assign40470_e46209 * assign40470_e46216);
            let assign40470_e46219: f64 = (assign40470_e46217 + 0.01);
            let assign40470_e46220: f64 = (assign40470_e46219).sqrt();
            let assign40470_e46221: f64 = (assign40470_e46202 + assign40470_e46220);
            let assign40470_e46222: f64 = (0.5 * assign40470_e46221);
            (locals.var_temp3, locals.var_temp3_dn4, locals.var_temp3_dn6, locals.var_temp3_dn7, locals.var_temp3_dn8, locals.var_temp3_dn9, ) = (assign40470_e46222, (0.5 * ((0.2 * locals.var_temp1_dn4) + ((((0.2 * locals.var_temp1_dn4) * assign40470_e46216) + (assign40470_e46209 * (0.2 * locals.var_temp1_dn4))) / (2.0 * assign40470_e46220)))), (0.5 * ((0.2 * locals.var_temp1_dn6) + ((((0.2 * locals.var_temp1_dn6) * assign40470_e46216) + (assign40470_e46209 * (0.2 * locals.var_temp1_dn6))) / (2.0 * assign40470_e46220)))), (0.5 * ((0.2 * locals.var_temp1_dn7) + ((((0.2 * locals.var_temp1_dn7) * assign40470_e46216) + (assign40470_e46209 * (0.2 * locals.var_temp1_dn7))) / (2.0 * assign40470_e46220)))), (0.5 * ((0.2 * locals.var_temp1_dn8) + ((((0.2 * locals.var_temp1_dn8) * assign40470_e46216) + (assign40470_e46209 * (0.2 * locals.var_temp1_dn8))) / (2.0 * assign40470_e46220)))), (0.5 * ((0.2 * locals.var_temp1_dn9) + ((((0.2 * locals.var_temp1_dn9) * assign40470_e46216) + (assign40470_e46209 * (0.2 * locals.var_temp1_dn9))) / (2.0 * assign40470_e46220)))), );
            locals.var_temp3_rv = 0.0;
        }
        if (locals.var_guard1080 != 0.0) {
            let assign40480_e46228: f64 = (locals.var_temp2 / locals.var_temp3);
            (locals.var_fcor__blk1038, locals.var_fcor__blk1038_dn4, locals.var_fcor__blk1038_dn6, locals.var_fcor__blk1038_dn7, locals.var_fcor__blk1038_dn8, locals.var_fcor__blk1038_dn9, ) = (assign40480_e46228, (((locals.var_temp2_dn4 * locals.var_temp3) - (locals.var_temp2 * locals.var_temp3_dn4)) / (locals.var_temp3 * locals.var_temp3)), (((locals.var_temp2_dn6 * locals.var_temp3) - (locals.var_temp2 * locals.var_temp3_dn6)) / (locals.var_temp3 * locals.var_temp3)), (((locals.var_temp2_dn7 * locals.var_temp3) - (locals.var_temp2 * locals.var_temp3_dn7)) / (locals.var_temp3 * locals.var_temp3)), (((locals.var_temp2_dn8 * locals.var_temp3) - (locals.var_temp2 * locals.var_temp3_dn8)) / (locals.var_temp3 * locals.var_temp3)), (((locals.var_temp2_dn9 * locals.var_temp3) - (locals.var_temp2 * locals.var_temp3_dn9)) / (locals.var_temp3 * locals.var_temp3)), );
            locals.var_fcor__blk1038_rv = 0.0;
        }
        if (locals.var_guard1080 != 0.0) {
            let assign40490_e46236: f64 = (locals.var_csfi_i * locals.var_ecpl1__blk1031);
            let assign40490_e46237: f64 = (1.0 + assign40490_e46236);
            let assign40490_e46240: f64 = (locals.var_csbi_i * locals.var_ecpl2__blk1032);
            let assign40490_e46241: f64 = (assign40490_e46237 + assign40490_e46240);
            let assign40490_e46242: f64 = (locals.var_cs_i * assign40490_e46241);
            let assign40490_e46244: f64 = (-locals.var_thecs_i);
            let assign40490_e46248: f64 = (locals.var_qi1m__blk1029 * locals.var_inv_qi1cs);
            let assign40490_e46249: f64 = (1.0 + assign40490_e46248);
            let assign40490_e46252: f64 = (locals.var_qi2m__blk1030 * locals.var_inv_qi2cs);
            let assign40490_e46253: f64 = (assign40490_e46249 + assign40490_e46252);
            let assign40490_e46254: f64 = (assign40490_e46253).ln();
            let assign40490_e46255: f64 = (assign40490_e46244 * assign40490_e46254);
            let assign40490_e46256: f64 = (assign40490_e46255).exp();
            let assign40490_e46257: f64 = (assign40490_e46242 * assign40490_e46256);
            (locals.var_gcs__blk1039, locals.var_gcs__blk1039_dn4, locals.var_gcs__blk1039_dn6, locals.var_gcs__blk1039_dn7, locals.var_gcs__blk1039_dn8, locals.var_gcs__blk1039_dn9, ) = (assign40490_e46257, ((((locals.var_cs_i_dn4 * assign40490_e46241) + (locals.var_cs_i * ((locals.var_csfi_i * locals.var_ecpl1__blk1031_dn4) + (locals.var_csbi_i * locals.var_ecpl2__blk1032_dn4)))) * assign40490_e46256) + (assign40490_e46242 * (assign40490_e46256 * (((-locals.var_thecs_i_dn4) * assign40490_e46254) + (assign40490_e46244 * (((locals.var_qi1m__blk1029_dn4 * locals.var_inv_qi1cs) + (locals.var_qi2m__blk1030_dn4 * locals.var_inv_qi2cs)) / assign40490_e46253)))))), ((((locals.var_cs_i_dn6 * assign40490_e46241) + (locals.var_cs_i * ((locals.var_csfi_i * locals.var_ecpl1__blk1031_dn6) + (locals.var_csbi_i * locals.var_ecpl2__blk1032_dn6)))) * assign40490_e46256) + (assign40490_e46242 * (assign40490_e46256 * (((-locals.var_thecs_i_dn6) * assign40490_e46254) + (assign40490_e46244 * (((locals.var_qi1m__blk1029_dn6 * locals.var_inv_qi1cs) + (locals.var_qi2m__blk1030_dn6 * locals.var_inv_qi2cs)) / assign40490_e46253)))))), ((((locals.var_cs_i_dn7 * assign40490_e46241) + (locals.var_cs_i * ((locals.var_csfi_i * locals.var_ecpl1__blk1031_dn7) + (locals.var_csbi_i * locals.var_ecpl2__blk1032_dn7)))) * assign40490_e46256) + (assign40490_e46242 * (assign40490_e46256 * (((-locals.var_thecs_i_dn7) * assign40490_e46254) + (assign40490_e46244 * (((locals.var_qi1m__blk1029_dn7 * locals.var_inv_qi1cs) + (locals.var_qi2m__blk1030_dn7 * locals.var_inv_qi2cs)) / assign40490_e46253)))))), ((((locals.var_cs_i_dn8 * assign40490_e46241) + (locals.var_cs_i * ((locals.var_csfi_i * locals.var_ecpl1__blk1031_dn8) + (locals.var_csbi_i * locals.var_ecpl2__blk1032_dn8)))) * assign40490_e46256) + (assign40490_e46242 * (assign40490_e46256 * (((-locals.var_thecs_i_dn8) * assign40490_e46254) + (assign40490_e46244 * (((locals.var_qi1m__blk1029_dn8 * locals.var_inv_qi1cs) + (locals.var_qi2m__blk1030_dn8 * locals.var_inv_qi2cs)) / assign40490_e46253)))))), ((((locals.var_cs_i_dn9 * assign40490_e46241) + (locals.var_cs_i * ((locals.var_csfi_i * locals.var_ecpl1__blk1031_dn9) + (locals.var_csbi_i * locals.var_ecpl2__blk1032_dn9)))) * assign40490_e46256) + (assign40490_e46242 * (assign40490_e46256 * (((-locals.var_thecs_i_dn9) * assign40490_e46254) + (assign40490_e46244 * (((locals.var_qi1m__blk1029_dn9 * locals.var_inv_qi1cs) + (locals.var_qi2m__blk1030_dn9 * locals.var_inv_qi2cs)) / assign40490_e46253)))))), );
            locals.var_gcs__blk1039_rv = 0.0;
        }
        let assign40500_e46262: f64 = if locals.var_rsg_i == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1222 = assign40500_e46262;
        locals.var_guard1222_rv = 0.0;
        if ((locals.var_guard1080 != 0.0) && (locals.var_guard1222 != 0.0)) {
            (locals.var_temp3, locals.var_temp3_dn4, locals.var_temp3_dn6, locals.var_temp3_dn7, locals.var_temp3_dn8, locals.var_temp3_dn9, ) = (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_temp3_rv = 0.0;
        }
        let assign40520_e46271: f64 = if locals.var_rsg_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1223 = assign40520_e46271;
        locals.var_guard1223_rv = 0.0;
        if (((locals.var_guard1080 != 0.0) && (locals.var_guard1222 == 0.0)) && (locals.var_guard1223 != 0.0)) {
            let assign40530_e46282: f64 = (locals.var_qim__blk1016 + 1e-12);
            let assign40530_e46283: f64 = (assign40530_e46282).ln();
            let assign40530_e46284: f64 = (locals.var_thersg_i * assign40530_e46283);
            let assign40530_e46285: f64 = (assign40530_e46284).exp();
            let assign40530_e46286: f64 = (locals.var_rsg_i * assign40530_e46285);
            (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9, ) = (assign40530_e46286, (locals.var_rsg_i * (assign40530_e46285 * (locals.var_thersg_i * (locals.var_qim__blk1016_dn4 / assign40530_e46282)))), (locals.var_rsg_i * (assign40530_e46285 * (locals.var_thersg_i * (locals.var_qim__blk1016_dn6 / assign40530_e46282)))), (locals.var_rsg_i * (assign40530_e46285 * (locals.var_thersg_i * (locals.var_qim__blk1016_dn7 / assign40530_e46282)))), (locals.var_rsg_i * (assign40530_e46285 * (locals.var_thersg_i * (locals.var_qim__blk1016_dn8 / assign40530_e46282)))), (locals.var_rsg_i * (assign40530_e46285 * (locals.var_thersg_i * (locals.var_qim__blk1016_dn9 / assign40530_e46282)))), );
            locals.var_temp1_rv = 0.0;
        }
        if (((locals.var_guard1080 != 0.0) && (locals.var_guard1222 == 0.0)) && (locals.var_guard1223 != 0.0)) {
            let assign40540_e46297: f64 = (1.0 - locals.var_temp1);
            (locals.var_temp3, locals.var_temp3_dn4, locals.var_temp3_dn6, locals.var_temp3_dn7, locals.var_temp3_dn8, locals.var_temp3_dn9, ) = (assign40540_e46297, (-locals.var_temp1_dn4), (-locals.var_temp1_dn6), (-locals.var_temp1_dn7), (-locals.var_temp1_dn8), (-locals.var_temp1_dn9), );
            locals.var_temp3_rv = 0.0;
        }
        if (((locals.var_guard1080 != 0.0) && (locals.var_guard1222 == 0.0)) && (locals.var_guard1223 == 0.0)) {
            let assign40550_e46311: f64 = (locals.var_qim__blk1016 + 1e-12);
            let assign40550_e46312: f64 = (assign40550_e46311).ln();
            let assign40550_e46313: f64 = (locals.var_thersg_i * assign40550_e46312);
            let assign40550_e46314: f64 = (assign40550_e46313).exp();
            let assign40550_e46315: f64 = (locals.var_rsg_i * assign40550_e46314);
            (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9, ) = (assign40550_e46315, (locals.var_rsg_i * (assign40550_e46314 * (locals.var_thersg_i * (locals.var_qim__blk1016_dn4 / assign40550_e46311)))), (locals.var_rsg_i * (assign40550_e46314 * (locals.var_thersg_i * (locals.var_qim__blk1016_dn6 / assign40550_e46311)))), (locals.var_rsg_i * (assign40550_e46314 * (locals.var_thersg_i * (locals.var_qim__blk1016_dn7 / assign40550_e46311)))), (locals.var_rsg_i * (assign40550_e46314 * (locals.var_thersg_i * (locals.var_qim__blk1016_dn8 / assign40550_e46311)))), (locals.var_rsg_i * (assign40550_e46314 * (locals.var_thersg_i * (locals.var_qim__blk1016_dn9 / assign40550_e46311)))), );
            locals.var_temp1_rv = 0.0;
        }
        if (((locals.var_guard1080 != 0.0) && (locals.var_guard1222 == 0.0)) && (locals.var_guard1223 == 0.0)) {
            let assign40560_e46328: f64 = (1.0 + locals.var_temp1);
            let assign40560_e46329: f64 = (1.0 / assign40560_e46328);
            (locals.var_temp3, locals.var_temp3_dn4, locals.var_temp3_dn6, locals.var_temp3_dn7, locals.var_temp3_dn8, locals.var_temp3_dn9, ) = (assign40560_e46329, (-(locals.var_temp1_dn4 / (assign40560_e46328 * assign40560_e46328))), (-(locals.var_temp1_dn6 / (assign40560_e46328 * assign40560_e46328))), (-(locals.var_temp1_dn7 / (assign40560_e46328 * assign40560_e46328))), (-(locals.var_temp1_dn8 / (assign40560_e46328 * assign40560_e46328))), (-(locals.var_temp1_dn9 / (assign40560_e46328 * assign40560_e46328))), );
            locals.var_temp3_rv = 0.0;
        }
        if (locals.var_guard1080 != 0.0) {
            let assign40570_e46336: f64 = (locals.var_qim__blk1016 * locals.var_temp3);
            let assign40570_e46338: f64 = (assign40570_e46336 + locals.var_rsig_i);
            let assign40570_e46339: f64 = (locals.var_frscsi__blk964 * assign40570_e46338);
            (locals.var_grs__blk1040, locals.var_grs__blk1040_dn4, locals.var_grs__blk1040_dn6, locals.var_grs__blk1040_dn7, locals.var_grs__blk1040_dn8, locals.var_grs__blk1040_dn9, ) = (assign40570_e46339, ((locals.var_frscsi__blk964_dn4 * assign40570_e46338) + (locals.var_frscsi__blk964 * ((locals.var_qim__blk1016_dn4 * locals.var_temp3) + (locals.var_qim__blk1016 * locals.var_temp3_dn4)))), ((locals.var_frscsi__blk964_dn6 * assign40570_e46338) + (locals.var_frscsi__blk964 * ((locals.var_qim__blk1016_dn6 * locals.var_temp3) + (locals.var_qim__blk1016 * locals.var_temp3_dn6)))), ((locals.var_frscsi__blk964_dn7 * assign40570_e46338) + (locals.var_frscsi__blk964 * ((locals.var_qim__blk1016_dn7 * locals.var_temp3) + (locals.var_qim__blk1016 * locals.var_temp3_dn7)))), ((locals.var_frscsi__blk964_dn8 * assign40570_e46338) + (locals.var_frscsi__blk964 * ((locals.var_qim__blk1016_dn8 * locals.var_temp3) + (locals.var_qim__blk1016 * locals.var_temp3_dn8)))), ((locals.var_frscsi__blk964_dn9 * assign40570_e46338) + (locals.var_frscsi__blk964 * ((locals.var_qim__blk1016_dn9 * locals.var_temp3) + (locals.var_qim__blk1016 * locals.var_temp3_dn9)))), );
            locals.var_grs__blk1040_rv = 0.0;
        }
        if (locals.var_guard1080 != 0.0) {
            let assign40580_e46347: f64 = (locals.var_fmue * locals.var_eeff1__blk1033);
            let assign40580_e46349: f64 = (assign40580_e46347 + 1e-6);
            let assign40580_e46350: f64 = (assign40580_e46349).ln();
            let assign40580_e46351: f64 = (locals.var_themu_i * assign40580_e46350);
            let assign40580_e46352: f64 = (assign40580_e46351).exp();
            let assign40580_e46353: f64 = (1.0 + assign40580_e46352);
            let assign40580_e46355: f64 = (assign40580_e46353 + locals.var_gcs__blk1039);
            let assign40580_e46358: f64 = (locals.var_betn1_i * locals.var_grs__blk1040);
            let assign40580_e46359: f64 = (assign40580_e46355 + assign40580_e46358);
            (locals.var_gmob1__blk1041, locals.var_gmob1__blk1041_dn4, locals.var_gmob1__blk1041_dn6, locals.var_gmob1__blk1041_dn7, locals.var_gmob1__blk1041_dn8, locals.var_gmob1__blk1041_dn9, ) = (assign40580_e46359, (((assign40580_e46352 * ((locals.var_themu_i_dn4 * assign40580_e46350) + (locals.var_themu_i * (((locals.var_fmue_dn4 * locals.var_eeff1__blk1033) + (locals.var_fmue * locals.var_eeff1__blk1033_dn4)) / assign40580_e46349)))) + locals.var_gcs__blk1039_dn4) + ((locals.var_betn1_i_dn4 * locals.var_grs__blk1040) + (locals.var_betn1_i * locals.var_grs__blk1040_dn4))), (((assign40580_e46352 * ((locals.var_themu_i_dn6 * assign40580_e46350) + (locals.var_themu_i * (((locals.var_fmue_dn6 * locals.var_eeff1__blk1033) + (locals.var_fmue * locals.var_eeff1__blk1033_dn6)) / assign40580_e46349)))) + locals.var_gcs__blk1039_dn6) + ((locals.var_betn1_i_dn6 * locals.var_grs__blk1040) + (locals.var_betn1_i * locals.var_grs__blk1040_dn6))), (((assign40580_e46352 * ((locals.var_themu_i_dn7 * assign40580_e46350) + (locals.var_themu_i * (((locals.var_fmue_dn7 * locals.var_eeff1__blk1033) + (locals.var_fmue * locals.var_eeff1__blk1033_dn7)) / assign40580_e46349)))) + locals.var_gcs__blk1039_dn7) + ((locals.var_betn1_i_dn7 * locals.var_grs__blk1040) + (locals.var_betn1_i * locals.var_grs__blk1040_dn7))), (((assign40580_e46352 * ((locals.var_themu_i_dn8 * assign40580_e46350) + (locals.var_themu_i * (((locals.var_fmue_dn8 * locals.var_eeff1__blk1033) + (locals.var_fmue * locals.var_eeff1__blk1033_dn8)) / assign40580_e46349)))) + locals.var_gcs__blk1039_dn8) + ((locals.var_betn1_i_dn8 * locals.var_grs__blk1040) + (locals.var_betn1_i * locals.var_grs__blk1040_dn8))), (((assign40580_e46352 * ((locals.var_themu_i_dn9 * assign40580_e46350) + (locals.var_themu_i * (((locals.var_fmue_dn9 * locals.var_eeff1__blk1033) + (locals.var_fmue * locals.var_eeff1__blk1033_dn9)) / assign40580_e46349)))) + locals.var_gcs__blk1039_dn9) + ((locals.var_betn1_i_dn9 * locals.var_grs__blk1040) + (locals.var_betn1_i * locals.var_grs__blk1040_dn9))), );
            locals.var_gmob1__blk1041_rv = 0.0;
        }
        if (locals.var_guard1080 != 0.0) {
            let assign40590_e46367: f64 = (locals.var_fmue * locals.var_eeff2__blk1034);
            let assign40590_e46369: f64 = (assign40590_e46367 + 1e-6);
            let assign40590_e46370: f64 = (assign40590_e46369).ln();
            let assign40590_e46371: f64 = (locals.var_themu_i * assign40590_e46370);
            let assign40590_e46372: f64 = (assign40590_e46371).exp();
            let assign40590_e46373: f64 = (1.0 + assign40590_e46372);
            let assign40590_e46375: f64 = (assign40590_e46373 + locals.var_gcs__blk1039);
            let assign40590_e46378: f64 = (locals.var_betn2_i * locals.var_grs__blk1040);
            let assign40590_e46379: f64 = (assign40590_e46375 + assign40590_e46378);
            (locals.var_gmob2__blk1042, locals.var_gmob2__blk1042_dn4, locals.var_gmob2__blk1042_dn6, locals.var_gmob2__blk1042_dn7, locals.var_gmob2__blk1042_dn8, locals.var_gmob2__blk1042_dn9, ) = (assign40590_e46379, (((assign40590_e46372 * ((locals.var_themu_i_dn4 * assign40590_e46370) + (locals.var_themu_i * (((locals.var_fmue_dn4 * locals.var_eeff2__blk1034) + (locals.var_fmue * locals.var_eeff2__blk1034_dn4)) / assign40590_e46369)))) + locals.var_gcs__blk1039_dn4) + ((locals.var_betn2_i_dn4 * locals.var_grs__blk1040) + (locals.var_betn2_i * locals.var_grs__blk1040_dn4))), (((assign40590_e46372 * ((locals.var_themu_i_dn6 * assign40590_e46370) + (locals.var_themu_i * (((locals.var_fmue_dn6 * locals.var_eeff2__blk1034) + (locals.var_fmue * locals.var_eeff2__blk1034_dn6)) / assign40590_e46369)))) + locals.var_gcs__blk1039_dn6) + ((locals.var_betn2_i_dn6 * locals.var_grs__blk1040) + (locals.var_betn2_i * locals.var_grs__blk1040_dn6))), (((assign40590_e46372 * ((locals.var_themu_i_dn7 * assign40590_e46370) + (locals.var_themu_i * (((locals.var_fmue_dn7 * locals.var_eeff2__blk1034) + (locals.var_fmue * locals.var_eeff2__blk1034_dn7)) / assign40590_e46369)))) + locals.var_gcs__blk1039_dn7) + ((locals.var_betn2_i_dn7 * locals.var_grs__blk1040) + (locals.var_betn2_i * locals.var_grs__blk1040_dn7))), (((assign40590_e46372 * ((locals.var_themu_i_dn8 * assign40590_e46370) + (locals.var_themu_i * (((locals.var_fmue_dn8 * locals.var_eeff2__blk1034) + (locals.var_fmue * locals.var_eeff2__blk1034_dn8)) / assign40590_e46369)))) + locals.var_gcs__blk1039_dn8) + ((locals.var_betn2_i_dn8 * locals.var_grs__blk1040) + (locals.var_betn2_i * locals.var_grs__blk1040_dn8))), (((assign40590_e46372 * ((locals.var_themu_i_dn9 * assign40590_e46370) + (locals.var_themu_i * (((locals.var_fmue_dn9 * locals.var_eeff2__blk1034) + (locals.var_fmue * locals.var_eeff2__blk1034_dn9)) / assign40590_e46369)))) + locals.var_gcs__blk1039_dn9) + ((locals.var_betn2_i_dn9 * locals.var_grs__blk1040) + (locals.var_betn2_i * locals.var_grs__blk1040_dn9))), );
            locals.var_gmob2__blk1042_rv = 0.0;
        }
        if (locals.var_guard1080 != 0.0) {
            let assign40600_e46385: f64 = (locals.var_fcor__blk1038 * locals.var_csum__blk1037);
            let assign40600_e46388: f64 = (locals.var_c1__blk1035 / locals.var_gmob1__blk1041);
            let assign40600_e46391: f64 = (locals.var_c2__blk1036 / locals.var_gmob2__blk1042);
            let assign40600_e46392: f64 = (assign40600_e46388 + assign40600_e46391);
            let assign40600_e46393: f64 = (assign40600_e46385 / assign40600_e46392);
            (locals.var_gmob__blk1043, locals.var_gmob__blk1043_dn4, locals.var_gmob__blk1043_dn6, locals.var_gmob__blk1043_dn7, locals.var_gmob__blk1043_dn8, locals.var_gmob__blk1043_dn9, ) = (assign40600_e46393, (((((locals.var_fcor__blk1038_dn4 * locals.var_csum__blk1037) + (locals.var_fcor__blk1038 * locals.var_csum__blk1037_dn4)) * assign40600_e46392) - (assign40600_e46385 * ((((locals.var_c1__blk1035_dn4 * locals.var_gmob1__blk1041) - (locals.var_c1__blk1035 * locals.var_gmob1__blk1041_dn4)) / (locals.var_gmob1__blk1041 * locals.var_gmob1__blk1041)) + (((locals.var_c2__blk1036_dn4 * locals.var_gmob2__blk1042) - (locals.var_c2__blk1036 * locals.var_gmob2__blk1042_dn4)) / (locals.var_gmob2__blk1042 * locals.var_gmob2__blk1042))))) / (assign40600_e46392 * assign40600_e46392)), (((((locals.var_fcor__blk1038_dn6 * locals.var_csum__blk1037) + (locals.var_fcor__blk1038 * locals.var_csum__blk1037_dn6)) * assign40600_e46392) - (assign40600_e46385 * ((((locals.var_c1__blk1035_dn6 * locals.var_gmob1__blk1041) - (locals.var_c1__blk1035 * locals.var_gmob1__blk1041_dn6)) / (locals.var_gmob1__blk1041 * locals.var_gmob1__blk1041)) + (((locals.var_c2__blk1036_dn6 * locals.var_gmob2__blk1042) - (locals.var_c2__blk1036 * locals.var_gmob2__blk1042_dn6)) / (locals.var_gmob2__blk1042 * locals.var_gmob2__blk1042))))) / (assign40600_e46392 * assign40600_e46392)), (((((locals.var_fcor__blk1038_dn7 * locals.var_csum__blk1037) + (locals.var_fcor__blk1038 * locals.var_csum__blk1037_dn7)) * assign40600_e46392) - (assign40600_e46385 * ((((locals.var_c1__blk1035_dn7 * locals.var_gmob1__blk1041) - (locals.var_c1__blk1035 * locals.var_gmob1__blk1041_dn7)) / (locals.var_gmob1__blk1041 * locals.var_gmob1__blk1041)) + (((locals.var_c2__blk1036_dn7 * locals.var_gmob2__blk1042) - (locals.var_c2__blk1036 * locals.var_gmob2__blk1042_dn7)) / (locals.var_gmob2__blk1042 * locals.var_gmob2__blk1042))))) / (assign40600_e46392 * assign40600_e46392)), (((((locals.var_fcor__blk1038_dn8 * locals.var_csum__blk1037) + (locals.var_fcor__blk1038 * locals.var_csum__blk1037_dn8)) * assign40600_e46392) - (assign40600_e46385 * ((((locals.var_c1__blk1035_dn8 * locals.var_gmob1__blk1041) - (locals.var_c1__blk1035 * locals.var_gmob1__blk1041_dn8)) / (locals.var_gmob1__blk1041 * locals.var_gmob1__blk1041)) + (((locals.var_c2__blk1036_dn8 * locals.var_gmob2__blk1042) - (locals.var_c2__blk1036 * locals.var_gmob2__blk1042_dn8)) / (locals.var_gmob2__blk1042 * locals.var_gmob2__blk1042))))) / (assign40600_e46392 * assign40600_e46392)), (((((locals.var_fcor__blk1038_dn9 * locals.var_csum__blk1037) + (locals.var_fcor__blk1038 * locals.var_csum__blk1037_dn9)) * assign40600_e46392) - (assign40600_e46385 * ((((locals.var_c1__blk1035_dn9 * locals.var_gmob1__blk1041) - (locals.var_c1__blk1035 * locals.var_gmob1__blk1041_dn9)) / (locals.var_gmob1__blk1041 * locals.var_gmob1__blk1041)) + (((locals.var_c2__blk1036_dn9 * locals.var_gmob2__blk1042) - (locals.var_c2__blk1036 * locals.var_gmob2__blk1042_dn9)) / (locals.var_gmob2__blk1042 * locals.var_gmob2__blk1042))))) / (assign40600_e46392 * assign40600_e46392)), );
            locals.var_gmob__blk1043_rv = 0.0;
        }
        if (locals.var_guard1080 != 0.0) {
            let assign40610_e46400: f64 = (4.0 + locals.var_qim__blk1016);
            let assign40610_e46401: f64 = (1.0 / assign40610_e46400);
            (locals.var_inv_qimstar1__blk1044, locals.var_inv_qimstar1__blk1044_dn4, locals.var_inv_qimstar1__blk1044_dn6, locals.var_inv_qimstar1__blk1044_dn7, locals.var_inv_qimstar1__blk1044_dn8, locals.var_inv_qimstar1__blk1044_dn9, ) = (assign40610_e46401, (-(locals.var_qim__blk1016_dn4 / (assign40610_e46400 * assign40610_e46400))), (-(locals.var_qim__blk1016_dn6 / (assign40610_e46400 * assign40610_e46400))), (-(locals.var_qim__blk1016_dn7 / (assign40610_e46400 * assign40610_e46400))), (-(locals.var_qim__blk1016_dn8 / (assign40610_e46400 * assign40610_e46400))), (-(locals.var_qim__blk1016_dn9 / (assign40610_e46400 * assign40610_e46400))), );
            locals.var_inv_qimstar1__blk1044_rv = 0.0;
        }
        let assign40620_e46406: f64 = if locals.var_alpb_i > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1224 = assign40620_e46406;
        locals.var_guard1224_rv = 0.0;
        if ((locals.var_guard1080 != 0.0) && (locals.var_guard1224 != 0.0)) {
            let assign40630_e46414: f64 = (locals.var_alpb_i * locals.var_qi2m__blk1030);
            let assign40630_e46415: f64 = (1.0 + assign40630_e46414);
            let assign40630_e46416: f64 = (1.0 / assign40630_e46415);
            (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign40630_e46416, (-((locals.var_alpb_i * locals.var_qi2m__blk1030_dn4) / (assign40630_e46415 * assign40630_e46415))), (-((locals.var_alpb_i * locals.var_qi2m__blk1030_dn6) / (assign40630_e46415 * assign40630_e46415))), (-((locals.var_alpb_i * locals.var_qi2m__blk1030_dn7) / (assign40630_e46415 * assign40630_e46415))), (-((locals.var_alpb_i * locals.var_qi2m__blk1030_dn8) / (assign40630_e46415 * assign40630_e46415))), (-((locals.var_alpb_i * locals.var_qi2m__blk1030_dn9) / (assign40630_e46415 * assign40630_e46415))), );
            locals.var_temp_rv = 0.0;
        }
        if ((locals.var_guard1080 != 0.0) && (locals.var_guard1224 == 0.0)) {
            let assign40640_e46426: f64 = (locals.var_alpb_i * locals.var_qi2m__blk1030);
            let assign40640_e46427: f64 = (1.0 - assign40640_e46426);
            (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign40640_e46427, (-(locals.var_alpb_i * locals.var_qi2m__blk1030_dn4)), (-(locals.var_alpb_i * locals.var_qi2m__blk1030_dn6)), (-(locals.var_alpb_i * locals.var_qi2m__blk1030_dn7)), (-(locals.var_alpb_i * locals.var_qi2m__blk1030_dn8)), (-(locals.var_alpb_i * locals.var_qi2m__blk1030_dn9)), );
            locals.var_temp_rv = 0.0;
        }
        if (locals.var_guard1080 != 0.0) {
            let assign40650_e46433: f64 = (locals.var_qim__blk1016 * locals.var_inv_qimstar1__blk1044);
            let assign40650_e46435: f64 = (assign40650_e46433 * locals.var_temp);
            (locals.var_r1__blk1045, locals.var_r1__blk1045_dn4, locals.var_r1__blk1045_dn6, locals.var_r1__blk1045_dn7, locals.var_r1__blk1045_dn8, locals.var_r1__blk1045_dn9, ) = (assign40650_e46435, ((((locals.var_qim__blk1016_dn4 * locals.var_inv_qimstar1__blk1044) + (locals.var_qim__blk1016 * locals.var_inv_qimstar1__blk1044_dn4)) * locals.var_temp) + (assign40650_e46433 * locals.var_temp_dn4)), ((((locals.var_qim__blk1016_dn6 * locals.var_inv_qimstar1__blk1044) + (locals.var_qim__blk1016 * locals.var_inv_qimstar1__blk1044_dn6)) * locals.var_temp) + (assign40650_e46433 * locals.var_temp_dn6)), ((((locals.var_qim__blk1016_dn7 * locals.var_inv_qimstar1__blk1044) + (locals.var_qim__blk1016 * locals.var_inv_qimstar1__blk1044_dn7)) * locals.var_temp) + (assign40650_e46433 * locals.var_temp_dn7)), ((((locals.var_qim__blk1016_dn8 * locals.var_inv_qimstar1__blk1044) + (locals.var_qim__blk1016 * locals.var_inv_qimstar1__blk1044_dn8)) * locals.var_temp) + (assign40650_e46433 * locals.var_temp_dn8)), ((((locals.var_qim__blk1016_dn9 * locals.var_inv_qimstar1__blk1044) + (locals.var_qim__blk1016 * locals.var_inv_qimstar1__blk1044_dn9)) * locals.var_temp) + (assign40650_e46433 * locals.var_temp_dn9)), );
            locals.var_r1__blk1045_rv = 0.0;
        }
        if (locals.var_guard1080 != 0.0) {
            let assign40660_e46442: f64 = (locals.var_xd - locals.var_xdeff__blk1000);
            let assign40660_e46445: f64 = (locals.var_vp_i * locals.var_inv_phit);
            let assign40660_e46448: f64 = (locals.var_vpg_i * locals.var_qim__blk1016);
            let assign40660_e46450: f64 = (assign40660_e46448 * locals.var_qim__blk1016);
            let assign40660_e46451: f64 = (assign40660_e46445 + assign40660_e46450);
            let assign40660_e46452: f64 = (assign40660_e46442 / assign40660_e46451);
            let assign40660_e46453: f64 = (1.0 + assign40660_e46452);
            let assign40660_e46454: f64 = (assign40660_e46453).ln();
            let assign40660_e46456: f64 = (assign40660_e46454 * locals.var_r1__blk1045);
            (locals.var_dl_l_fact__blk1046, locals.var_dl_l_fact__blk1046_dn4, locals.var_dl_l_fact__blk1046_dn6, locals.var_dl_l_fact__blk1046_dn7, locals.var_dl_l_fact__blk1046_dn8, locals.var_dl_l_fact__blk1046_dn9, ) = (assign40660_e46456, (((((((locals.var_xd_dn4 - locals.var_xdeff__blk1000_dn4) * assign40660_e46451) - (assign40660_e46442 * ((locals.var_vp_i * locals.var_inv_phit_dn4) + (((locals.var_vpg_i * locals.var_qim__blk1016_dn4) * locals.var_qim__blk1016) + (assign40660_e46448 * locals.var_qim__blk1016_dn4))))) / (assign40660_e46451 * assign40660_e46451)) / assign40660_e46453) * locals.var_r1__blk1045) + (assign40660_e46454 * locals.var_r1__blk1045_dn4)), (((((((locals.var_xd_dn6 - locals.var_xdeff__blk1000_dn6) * assign40660_e46451) - (assign40660_e46442 * ((locals.var_vp_i * locals.var_inv_phit_dn6) + (((locals.var_vpg_i * locals.var_qim__blk1016_dn6) * locals.var_qim__blk1016) + (assign40660_e46448 * locals.var_qim__blk1016_dn6))))) / (assign40660_e46451 * assign40660_e46451)) / assign40660_e46453) * locals.var_r1__blk1045) + (assign40660_e46454 * locals.var_r1__blk1045_dn6)), (((((((locals.var_xd_dn7 - locals.var_xdeff__blk1000_dn7) * assign40660_e46451) - (assign40660_e46442 * ((locals.var_vp_i * locals.var_inv_phit_dn7) + (((locals.var_vpg_i * locals.var_qim__blk1016_dn7) * locals.var_qim__blk1016) + (assign40660_e46448 * locals.var_qim__blk1016_dn7))))) / (assign40660_e46451 * assign40660_e46451)) / assign40660_e46453) * locals.var_r1__blk1045) + (assign40660_e46454 * locals.var_r1__blk1045_dn7)), (((((((locals.var_xd_dn8 - locals.var_xdeff__blk1000_dn8) * assign40660_e46451) - (assign40660_e46442 * ((locals.var_vp_i * locals.var_inv_phit_dn8) + (((locals.var_vpg_i * locals.var_qim__blk1016_dn8) * locals.var_qim__blk1016) + (assign40660_e46448 * locals.var_qim__blk1016_dn8))))) / (assign40660_e46451 * assign40660_e46451)) / assign40660_e46453) * locals.var_r1__blk1045) + (assign40660_e46454 * locals.var_r1__blk1045_dn8)), (((((((locals.var_xd_dn9 - locals.var_xdeff__blk1000_dn9) * assign40660_e46451) - (assign40660_e46442 * ((locals.var_vp_i * locals.var_inv_phit_dn9) + (((locals.var_vpg_i * locals.var_qim__blk1016_dn9) * locals.var_qim__blk1016) + (assign40660_e46448 * locals.var_qim__blk1016_dn9))))) / (assign40660_e46451 * assign40660_e46451)) / assign40660_e46453) * locals.var_r1__blk1045) + (assign40660_e46454 * locals.var_r1__blk1045_dn9)), );
            locals.var_dl_l_fact__blk1046_rv = 0.0;
        }
        if (locals.var_guard1080 != 0.0) {
            let assign40670_e46462: f64 = (locals.var_alp_loc__blk898 * locals.var_dl_l_fact__blk1046);
            (locals.var_dl_l__blk1047, locals.var_dl_l__blk1047_dn4, locals.var_dl_l__blk1047_dn6, locals.var_dl_l__blk1047_dn7, locals.var_dl_l__blk1047_dn8, locals.var_dl_l__blk1047_dn9, ) = (assign40670_e46462, (locals.var_alp_loc__blk898 * locals.var_dl_l_fact__blk1046_dn4), (locals.var_alp_loc__blk898 * locals.var_dl_l_fact__blk1046_dn6), (locals.var_alp_loc__blk898 * locals.var_dl_l_fact__blk1046_dn7), (locals.var_alp_loc__blk898 * locals.var_dl_l_fact__blk1046_dn8), (locals.var_alp_loc__blk898 * locals.var_dl_l_fact__blk1046_dn9), );
            locals.var_dl_l__blk1047_rv = 0.0;
        }
        if (locals.var_guard1080 != 0.0) {
            let assign40680_e46471: f64 = (1.0 + locals.var_dl_l__blk1047);
            let assign40680_e46472: f64 = (locals.var_dl_l__blk1047 * assign40680_e46471);
            let assign40680_e46473: f64 = (1.0 + assign40680_e46472);
            let assign40680_e46474: f64 = (1.0 / assign40680_e46473);
            (locals.var_gdl__blk1048, locals.var_gdl__blk1048_dn4, locals.var_gdl__blk1048_dn6, locals.var_gdl__blk1048_dn7, locals.var_gdl__blk1048_dn8, locals.var_gdl__blk1048_dn9, ) = (assign40680_e46474, (-(((locals.var_dl_l__blk1047_dn4 * assign40680_e46471) + (locals.var_dl_l__blk1047 * locals.var_dl_l__blk1047_dn4)) / (assign40680_e46473 * assign40680_e46473))), (-(((locals.var_dl_l__blk1047_dn6 * assign40680_e46471) + (locals.var_dl_l__blk1047 * locals.var_dl_l__blk1047_dn6)) / (assign40680_e46473 * assign40680_e46473))), (-(((locals.var_dl_l__blk1047_dn7 * assign40680_e46471) + (locals.var_dl_l__blk1047 * locals.var_dl_l__blk1047_dn7)) / (assign40680_e46473 * assign40680_e46473))), (-(((locals.var_dl_l__blk1047_dn8 * assign40680_e46471) + (locals.var_dl_l__blk1047 * locals.var_dl_l__blk1047_dn8)) / (assign40680_e46473 * assign40680_e46473))), (-(((locals.var_dl_l__blk1047_dn9 * assign40680_e46471) + (locals.var_dl_l__blk1047 * locals.var_dl_l__blk1047_dn9)) / (assign40680_e46473 * assign40680_e46473))), );
            locals.var_gdl__blk1048_rv = 0.0;
        }
        if (locals.var_guard1080 != 0.0) {
            let assign40690_e46480: f64 = (100.0 * locals.var_esurf1__blk1027);
            let assign40690_e46483: f64 = (100.0 + locals.var_esurf1__blk1027);
            let assign40690_e46484: f64 = (assign40690_e46480 / assign40690_e46483);
            (locals.var_wsat1__blk976, locals.var_wsat1__blk976_dn4, locals.var_wsat1__blk976_dn6, locals.var_wsat1__blk976_dn7, locals.var_wsat1__blk976_dn8, locals.var_wsat1__blk976_dn9, ) = (assign40690_e46484, ((((100.0 * locals.var_esurf1__blk1027_dn4) * assign40690_e46483) - (assign40690_e46480 * locals.var_esurf1__blk1027_dn4)) / (assign40690_e46483 * assign40690_e46483)), ((((100.0 * locals.var_esurf1__blk1027_dn6) * assign40690_e46483) - (assign40690_e46480 * locals.var_esurf1__blk1027_dn6)) / (assign40690_e46483 * assign40690_e46483)), ((((100.0 * locals.var_esurf1__blk1027_dn7) * assign40690_e46483) - (assign40690_e46480 * locals.var_esurf1__blk1027_dn7)) / (assign40690_e46483 * assign40690_e46483)), ((((100.0 * locals.var_esurf1__blk1027_dn8) * assign40690_e46483) - (assign40690_e46480 * locals.var_esurf1__blk1027_dn8)) / (assign40690_e46483 * assign40690_e46483)), ((((100.0 * locals.var_esurf1__blk1027_dn9) * assign40690_e46483) - (assign40690_e46480 * locals.var_esurf1__blk1027_dn9)) / (assign40690_e46483 * assign40690_e46483)), );
            locals.var_wsat1__blk976_rv = 0.0;
        }
        let assign40700_e46489: f64 = if locals.var_thesat1_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1225 = assign40700_e46489;
        locals.var_guard1225_rv = 0.0;
        if ((locals.var_guard1080 != 0.0) && (locals.var_guard1225 != 0.0)) {
            let assign40710_e46497: f64 = (locals.var_thesat1_i * locals.var_wsat1__blk976);
            let assign40710_e46498: f64 = (1.0 - assign40710_e46497);
            let assign40710_e46499: f64 = (1.0 / assign40710_e46498);
            (locals.var_sat_fact1__blk977, locals.var_sat_fact1__blk977_dn4, locals.var_sat_fact1__blk977_dn6, locals.var_sat_fact1__blk977_dn7, locals.var_sat_fact1__blk977_dn8, locals.var_sat_fact1__blk977_dn9, ) = (assign40710_e46499, (-((-(locals.var_thesat1_i * locals.var_wsat1__blk976_dn4)) / (assign40710_e46498 * assign40710_e46498))), (-((-(locals.var_thesat1_i * locals.var_wsat1__blk976_dn6)) / (assign40710_e46498 * assign40710_e46498))), (-((-(locals.var_thesat1_i * locals.var_wsat1__blk976_dn7)) / (assign40710_e46498 * assign40710_e46498))), (-((-(locals.var_thesat1_i * locals.var_wsat1__blk976_dn8)) / (assign40710_e46498 * assign40710_e46498))), (-((-(locals.var_thesat1_i * locals.var_wsat1__blk976_dn9)) / (assign40710_e46498 * assign40710_e46498))), );
            locals.var_sat_fact1__blk977_rv = 0.0;
        }
        if ((locals.var_guard1080 != 0.0) && (locals.var_guard1225 == 0.0)) {
            let assign40720_e46509: f64 = (locals.var_thesat1_i * locals.var_wsat1__blk976);
            let assign40720_e46510: f64 = (1.0 + assign40720_e46509);
            (locals.var_sat_fact1__blk977, locals.var_sat_fact1__blk977_dn4, locals.var_sat_fact1__blk977_dn6, locals.var_sat_fact1__blk977_dn7, locals.var_sat_fact1__blk977_dn8, locals.var_sat_fact1__blk977_dn9, ) = (assign40720_e46510, (locals.var_thesat1_i * locals.var_wsat1__blk976_dn4), (locals.var_thesat1_i * locals.var_wsat1__blk976_dn6), (locals.var_thesat1_i * locals.var_wsat1__blk976_dn7), (locals.var_thesat1_i * locals.var_wsat1__blk976_dn8), (locals.var_thesat1_i * locals.var_wsat1__blk976_dn9), );
            locals.var_sat_fact1__blk977_rv = 0.0;
        }
        if (locals.var_guard1080 != 0.0) {
            let assign40730_e46516: f64 = (100.0 * locals.var_esurf2__blk1028);
            let assign40730_e46519: f64 = (100.0 + locals.var_esurf2__blk1028);
            let assign40730_e46520: f64 = (assign40730_e46516 / assign40730_e46519);
            (locals.var_wsat2__blk978, locals.var_wsat2__blk978_dn4, locals.var_wsat2__blk978_dn6, locals.var_wsat2__blk978_dn7, locals.var_wsat2__blk978_dn8, locals.var_wsat2__blk978_dn9, ) = (assign40730_e46520, ((((100.0 * locals.var_esurf2__blk1028_dn4) * assign40730_e46519) - (assign40730_e46516 * locals.var_esurf2__blk1028_dn4)) / (assign40730_e46519 * assign40730_e46519)), ((((100.0 * locals.var_esurf2__blk1028_dn6) * assign40730_e46519) - (assign40730_e46516 * locals.var_esurf2__blk1028_dn6)) / (assign40730_e46519 * assign40730_e46519)), ((((100.0 * locals.var_esurf2__blk1028_dn7) * assign40730_e46519) - (assign40730_e46516 * locals.var_esurf2__blk1028_dn7)) / (assign40730_e46519 * assign40730_e46519)), ((((100.0 * locals.var_esurf2__blk1028_dn8) * assign40730_e46519) - (assign40730_e46516 * locals.var_esurf2__blk1028_dn8)) / (assign40730_e46519 * assign40730_e46519)), ((((100.0 * locals.var_esurf2__blk1028_dn9) * assign40730_e46519) - (assign40730_e46516 * locals.var_esurf2__blk1028_dn9)) / (assign40730_e46519 * assign40730_e46519)), );
            locals.var_wsat2__blk978_rv = 0.0;
        }
        let assign40740_e46525: f64 = if locals.var_thesat2_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1226 = assign40740_e46525;
        locals.var_guard1226_rv = 0.0;
        if ((locals.var_guard1080 != 0.0) && (locals.var_guard1226 != 0.0)) {
            let assign40750_e46533: f64 = (locals.var_thesat2_i * locals.var_wsat2__blk978);
            let assign40750_e46534: f64 = (1.0 - assign40750_e46533);
            let assign40750_e46535: f64 = (1.0 / assign40750_e46534);
            (locals.var_sat_fact2__blk979, locals.var_sat_fact2__blk979_dn4, locals.var_sat_fact2__blk979_dn6, locals.var_sat_fact2__blk979_dn7, locals.var_sat_fact2__blk979_dn8, locals.var_sat_fact2__blk979_dn9, ) = (assign40750_e46535, (-((-(locals.var_thesat2_i * locals.var_wsat2__blk978_dn4)) / (assign40750_e46534 * assign40750_e46534))), (-((-(locals.var_thesat2_i * locals.var_wsat2__blk978_dn6)) / (assign40750_e46534 * assign40750_e46534))), (-((-(locals.var_thesat2_i * locals.var_wsat2__blk978_dn7)) / (assign40750_e46534 * assign40750_e46534))), (-((-(locals.var_thesat2_i * locals.var_wsat2__blk978_dn8)) / (assign40750_e46534 * assign40750_e46534))), (-((-(locals.var_thesat2_i * locals.var_wsat2__blk978_dn9)) / (assign40750_e46534 * assign40750_e46534))), );
            locals.var_sat_fact2__blk979_rv = 0.0;
        }
        if ((locals.var_guard1080 != 0.0) && (locals.var_guard1226 == 0.0)) {
            let assign40760_e46545: f64 = (locals.var_thesat2_i * locals.var_wsat2__blk978);
            let assign40760_e46546: f64 = (1.0 + assign40760_e46545);
            (locals.var_sat_fact2__blk979, locals.var_sat_fact2__blk979_dn4, locals.var_sat_fact2__blk979_dn6, locals.var_sat_fact2__blk979_dn7, locals.var_sat_fact2__blk979_dn8, locals.var_sat_fact2__blk979_dn9, ) = (assign40760_e46546, (locals.var_thesat2_i * locals.var_wsat2__blk978_dn4), (locals.var_thesat2_i * locals.var_wsat2__blk978_dn6), (locals.var_thesat2_i * locals.var_wsat2__blk978_dn7), (locals.var_thesat2_i * locals.var_wsat2__blk978_dn8), (locals.var_thesat2_i * locals.var_wsat2__blk978_dn9), );
            locals.var_sat_fact2__blk979_rv = 0.0;
        }
        if (locals.var_guard1080 != 0.0) {
            let assign40770_e46552: f64 = (locals.var_sat_phit_loc__blk896 * locals.var_dxdrift__blk1017);
            let assign40770_e46554: f64 = (assign40770_e46552 * 0.5);
            let assign40770_e46557: f64 = (locals.var_sat_fact1__blk977 + locals.var_sat_fact2__blk979);
            let assign40770_e46558: f64 = (assign40770_e46554 * assign40770_e46557);
            (locals.var_ggamma__blk1049, locals.var_ggamma__blk1049_dn4, locals.var_ggamma__blk1049_dn6, locals.var_ggamma__blk1049_dn7, locals.var_ggamma__blk1049_dn8, locals.var_ggamma__blk1049_dn9, ) = (assign40770_e46558, (((((locals.var_sat_phit_loc__blk896_dn4 * locals.var_dxdrift__blk1017) + (locals.var_sat_phit_loc__blk896 * locals.var_dxdrift__blk1017_dn4)) * 0.5) * assign40770_e46557) + (assign40770_e46554 * (locals.var_sat_fact1__blk977_dn4 + locals.var_sat_fact2__blk979_dn4))), (((((locals.var_sat_phit_loc__blk896_dn6 * locals.var_dxdrift__blk1017) + (locals.var_sat_phit_loc__blk896 * locals.var_dxdrift__blk1017_dn6)) * 0.5) * assign40770_e46557) + (assign40770_e46554 * (locals.var_sat_fact1__blk977_dn6 + locals.var_sat_fact2__blk979_dn6))), (((((locals.var_sat_phit_loc__blk896_dn7 * locals.var_dxdrift__blk1017) + (locals.var_sat_phit_loc__blk896 * locals.var_dxdrift__blk1017_dn7)) * 0.5) * assign40770_e46557) + (assign40770_e46554 * (locals.var_sat_fact1__blk977_dn7 + locals.var_sat_fact2__blk979_dn7))), (((((locals.var_sat_phit_loc__blk896_dn8 * locals.var_dxdrift__blk1017) + (locals.var_sat_phit_loc__blk896 * locals.var_dxdrift__blk1017_dn8)) * 0.5) * assign40770_e46557) + (assign40770_e46554 * (locals.var_sat_fact1__blk977_dn8 + locals.var_sat_fact2__blk979_dn8))), (((((locals.var_sat_phit_loc__blk896_dn9 * locals.var_dxdrift__blk1017) + (locals.var_sat_phit_loc__blk896 * locals.var_dxdrift__blk1017_dn9)) * 0.5) * assign40770_e46557) + (assign40770_e46554 * (locals.var_sat_fact1__blk977_dn9 + locals.var_sat_fact2__blk979_dn9))), );
            locals.var_ggamma__blk1049_rv = 0.0;
        }
        if (locals.var_guard1080 != 0.0) {
            let assign40780_e46565: f64 = (locals.var_gmob__blk1043 * locals.var_gdl__blk1048);
            let assign40780_e46566: f64 = (locals.var_ggamma__blk1049 / assign40780_e46565);
            (locals.var_sqrt_zsat__blk1050, locals.var_sqrt_zsat__blk1050_dn4, locals.var_sqrt_zsat__blk1050_dn6, locals.var_sqrt_zsat__blk1050_dn7, locals.var_sqrt_zsat__blk1050_dn8, locals.var_sqrt_zsat__blk1050_dn9, ) = (assign40780_e46566, (((locals.var_ggamma__blk1049_dn4 * assign40780_e46565) - (locals.var_ggamma__blk1049 * ((locals.var_gmob__blk1043_dn4 * locals.var_gdl__blk1048) + (locals.var_gmob__blk1043 * locals.var_gdl__blk1048_dn4)))) / (assign40780_e46565 * assign40780_e46565)), (((locals.var_ggamma__blk1049_dn6 * assign40780_e46565) - (locals.var_ggamma__blk1049 * ((locals.var_gmob__blk1043_dn6 * locals.var_gdl__blk1048) + (locals.var_gmob__blk1043 * locals.var_gdl__blk1048_dn6)))) / (assign40780_e46565 * assign40780_e46565)), (((locals.var_ggamma__blk1049_dn7 * assign40780_e46565) - (locals.var_ggamma__blk1049 * ((locals.var_gmob__blk1043_dn7 * locals.var_gdl__blk1048) + (locals.var_gmob__blk1043 * locals.var_gdl__blk1048_dn7)))) / (assign40780_e46565 * assign40780_e46565)), (((locals.var_ggamma__blk1049_dn8 * assign40780_e46565) - (locals.var_ggamma__blk1049 * ((locals.var_gmob__blk1043_dn8 * locals.var_gdl__blk1048) + (locals.var_gmob__blk1043 * locals.var_gdl__blk1048_dn8)))) / (assign40780_e46565 * assign40780_e46565)), (((locals.var_ggamma__blk1049_dn9 * assign40780_e46565) - (locals.var_ggamma__blk1049 * ((locals.var_gmob__blk1043_dn9 * locals.var_gdl__blk1048) + (locals.var_gmob__blk1043 * locals.var_gdl__blk1048_dn9)))) / (assign40780_e46565 * assign40780_e46565)), );
            locals.var_sqrt_zsat__blk1050_rv = 0.0;
        }
        if (locals.var_guard1080 != 0.0) {
            let assign40790_e46572: f64 = (locals.var_sqrt_zsat__blk1050 * locals.var_sqrt_zsat__blk1050);
            (locals.var_zsat__blk1051, locals.var_zsat__blk1051_dn4, locals.var_zsat__blk1051_dn6, locals.var_zsat__blk1051_dn7, locals.var_zsat__blk1051_dn8, locals.var_zsat__blk1051_dn9, ) = (assign40790_e46572, ((locals.var_sqrt_zsat__blk1050_dn4 * locals.var_sqrt_zsat__blk1050) + (locals.var_sqrt_zsat__blk1050 * locals.var_sqrt_zsat__blk1050_dn4)), ((locals.var_sqrt_zsat__blk1050_dn6 * locals.var_sqrt_zsat__blk1050) + (locals.var_sqrt_zsat__blk1050 * locals.var_sqrt_zsat__blk1050_dn6)), ((locals.var_sqrt_zsat__blk1050_dn7 * locals.var_sqrt_zsat__blk1050) + (locals.var_sqrt_zsat__blk1050 * locals.var_sqrt_zsat__blk1050_dn7)), ((locals.var_sqrt_zsat__blk1050_dn8 * locals.var_sqrt_zsat__blk1050) + (locals.var_sqrt_zsat__blk1050 * locals.var_sqrt_zsat__blk1050_dn8)), ((locals.var_sqrt_zsat__blk1050_dn9 * locals.var_sqrt_zsat__blk1050) + (locals.var_sqrt_zsat__blk1050 * locals.var_sqrt_zsat__blk1050_dn9)), );
            locals.var_zsat__blk1051_rv = 0.0;
        }
        if (locals.var_guard1080 != 0.0) {
            let assign40800_e46578: f64 = (1.0 + locals.var_zsat__blk1051);
            let assign40800_e46579: f64 = (assign40800_e46578).sqrt();
            (locals.var_vsat_fact__blk1052, locals.var_vsat_fact__blk1052_dn4, locals.var_vsat_fact__blk1052_dn6, locals.var_vsat_fact__blk1052_dn7, locals.var_vsat_fact__blk1052_dn8, locals.var_vsat_fact__blk1052_dn9, ) = (assign40800_e46579, (locals.var_zsat__blk1051_dn4 / (2.0 * assign40800_e46579)), (locals.var_zsat__blk1051_dn6 / (2.0 * assign40800_e46579)), (locals.var_zsat__blk1051_dn7 / (2.0 * assign40800_e46579)), (locals.var_zsat__blk1051_dn8 / (2.0 * assign40800_e46579)), (locals.var_zsat__blk1051_dn9 / (2.0 * assign40800_e46579)), );
            locals.var_vsat_fact__blk1052_rv = 0.0;
        }
        if (locals.var_guard1080 != 0.0) {
            let assign40810_e46586: f64 = (1.5 * locals.var_zsat__blk1051);
            let assign40810_e46587: f64 = (1.0 + assign40810_e46586);
            let assign40810_e46589: f64 = (assign40810_e46587 / locals.var_vsat_fact__blk1052);
            (locals.var_hsat__blk1053, locals.var_hsat__blk1053_dn4, locals.var_hsat__blk1053_dn6, locals.var_hsat__blk1053_dn7, locals.var_hsat__blk1053_dn8, locals.var_hsat__blk1053_dn9, ) = (assign40810_e46589, ((((1.5 * locals.var_zsat__blk1051_dn4) * locals.var_vsat_fact__blk1052) - (assign40810_e46587 * locals.var_vsat_fact__blk1052_dn4)) / (locals.var_vsat_fact__blk1052 * locals.var_vsat_fact__blk1052)), ((((1.5 * locals.var_zsat__blk1051_dn6) * locals.var_vsat_fact__blk1052) - (assign40810_e46587 * locals.var_vsat_fact__blk1052_dn6)) / (locals.var_vsat_fact__blk1052 * locals.var_vsat_fact__blk1052)), ((((1.5 * locals.var_zsat__blk1051_dn7) * locals.var_vsat_fact__blk1052) - (assign40810_e46587 * locals.var_vsat_fact__blk1052_dn7)) / (locals.var_vsat_fact__blk1052 * locals.var_vsat_fact__blk1052)), ((((1.5 * locals.var_zsat__blk1051_dn8) * locals.var_vsat_fact__blk1052) - (assign40810_e46587 * locals.var_vsat_fact__blk1052_dn8)) / (locals.var_vsat_fact__blk1052 * locals.var_vsat_fact__blk1052)), ((((1.5 * locals.var_zsat__blk1051_dn9) * locals.var_vsat_fact__blk1052) - (assign40810_e46587 * locals.var_vsat_fact__blk1052_dn9)) / (locals.var_vsat_fact__blk1052 * locals.var_vsat_fact__blk1052)), );
            locals.var_hsat__blk1053_rv = 0.0;
        }
        let assign40820_e46594: f64 = if p.p13 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1227 = assign40820_e46594;
        locals.var_guard1227_rv = 0.0;
        if ((locals.var_guard1080 != 0.0) && (locals.var_guard1227 != 0.0)) {
            let assign40830_e46600: f64 = (0.6 * locals.var_qq);
            let assign40830_e46602: f64 = (-0.1666666666667);
            let assign40830_e46605: f64 = (locals.var_esurf1__blk1027 * locals.var_esurf1__blk1027);
            let assign40830_e46607: f64 = (assign40830_e46605 + 60.0);
            let assign40830_e46608: f64 = (assign40830_e46607).ln();
            let assign40830_e46609: f64 = (assign40830_e46602 * assign40830_e46608);
            let assign40830_e46610: f64 = (assign40830_e46609).exp();
            let assign40830_e46611: f64 = (assign40830_e46600 * assign40830_e46610);
            (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9, ) = (assign40830_e46611, (((0.6 * locals.var_qq_dn4) * assign40830_e46610) + (assign40830_e46600 * (assign40830_e46610 * (assign40830_e46602 * (((locals.var_esurf1__blk1027_dn4 * locals.var_esurf1__blk1027) + (locals.var_esurf1__blk1027 * locals.var_esurf1__blk1027_dn4)) / assign40830_e46607))))), (((0.6 * locals.var_qq_dn6) * assign40830_e46610) + (assign40830_e46600 * (assign40830_e46610 * (assign40830_e46602 * (((locals.var_esurf1__blk1027_dn6 * locals.var_esurf1__blk1027) + (locals.var_esurf1__blk1027 * locals.var_esurf1__blk1027_dn6)) / assign40830_e46607))))), (((0.6 * locals.var_qq_dn7) * assign40830_e46610) + (assign40830_e46600 * (assign40830_e46610 * (assign40830_e46602 * (((locals.var_esurf1__blk1027_dn7 * locals.var_esurf1__blk1027) + (locals.var_esurf1__blk1027 * locals.var_esurf1__blk1027_dn7)) / assign40830_e46607))))), (((0.6 * locals.var_qq_dn8) * assign40830_e46610) + (assign40830_e46600 * (assign40830_e46610 * (assign40830_e46602 * (((locals.var_esurf1__blk1027_dn8 * locals.var_esurf1__blk1027) + (locals.var_esurf1__blk1027 * locals.var_esurf1__blk1027_dn8)) / assign40830_e46607))))), (((0.6 * locals.var_qq_dn9) * assign40830_e46610) + (assign40830_e46600 * (assign40830_e46610 * (assign40830_e46602 * (((locals.var_esurf1__blk1027_dn9 * locals.var_esurf1__blk1027) + (locals.var_esurf1__blk1027 * locals.var_esurf1__blk1027_dn9)) / assign40830_e46607))))), );
            locals.var_temp1_rv = 0.0;
        }
        if ((locals.var_guard1080 != 0.0) && (locals.var_guard1227 != 0.0)) {
            let assign40840_e46619: f64 = (0.6 * locals.var_qq);
            let assign40840_e46621: f64 = (-0.1666666666667);
            let assign40840_e46624: f64 = (locals.var_esurf2__blk1028 * locals.var_esurf2__blk1028);
            let assign40840_e46626: f64 = (assign40840_e46624 + 60.0);
            let assign40840_e46627: f64 = (assign40840_e46626).ln();
            let assign40840_e46628: f64 = (assign40840_e46621 * assign40840_e46627);
            let assign40840_e46629: f64 = (assign40840_e46628).exp();
            let assign40840_e46630: f64 = (assign40840_e46619 * assign40840_e46629);
            (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9, ) = (assign40840_e46630, (((0.6 * locals.var_qq_dn4) * assign40840_e46629) + (assign40840_e46619 * (assign40840_e46629 * (assign40840_e46621 * (((locals.var_esurf2__blk1028_dn4 * locals.var_esurf2__blk1028) + (locals.var_esurf2__blk1028 * locals.var_esurf2__blk1028_dn4)) / assign40840_e46626))))), (((0.6 * locals.var_qq_dn6) * assign40840_e46629) + (assign40840_e46619 * (assign40840_e46629 * (assign40840_e46621 * (((locals.var_esurf2__blk1028_dn6 * locals.var_esurf2__blk1028) + (locals.var_esurf2__blk1028 * locals.var_esurf2__blk1028_dn6)) / assign40840_e46626))))), (((0.6 * locals.var_qq_dn7) * assign40840_e46629) + (assign40840_e46619 * (assign40840_e46629 * (assign40840_e46621 * (((locals.var_esurf2__blk1028_dn7 * locals.var_esurf2__blk1028) + (locals.var_esurf2__blk1028 * locals.var_esurf2__blk1028_dn7)) / assign40840_e46626))))), (((0.6 * locals.var_qq_dn8) * assign40840_e46629) + (assign40840_e46619 * (assign40840_e46629 * (assign40840_e46621 * (((locals.var_esurf2__blk1028_dn8 * locals.var_esurf2__blk1028) + (locals.var_esurf2__blk1028 * locals.var_esurf2__blk1028_dn8)) / assign40840_e46626))))), (((0.6 * locals.var_qq_dn9) * assign40840_e46629) + (assign40840_e46619 * (assign40840_e46629 * (assign40840_e46621 * (((locals.var_esurf2__blk1028_dn9 * locals.var_esurf2__blk1028) + (locals.var_esurf2__blk1028 * locals.var_esurf2__blk1028_dn9)) / assign40840_e46626))))), );
            locals.var_temp2_rv = 0.0;
        }
    }
    pub(super) fn stamp_reactive_block_59(
        locals: &mut StampLocals,
    ) {
        if ((locals.var_guard1080 != 0.0) && (locals.var_guard1227 != 0.0)) {
            let assign40850_e46639: f64 = (locals.var_k1__blk932 * locals.var_temp1);
            let assign40850_e46640: f64 = (1.0 + assign40850_e46639);
            let assign40850_e46642: f64 = (assign40850_e46640 / locals.var_tox1fact__blk913);
            (locals.var_qmfact1__blk1054, locals.var_qmfact1__blk1054_dn4, locals.var_qmfact1__blk1054_dn6, locals.var_qmfact1__blk1054_dn7, locals.var_qmfact1__blk1054_dn8, locals.var_qmfact1__blk1054_dn9, ) = (assign40850_e46642, (((((locals.var_k1__blk932_dn4 * locals.var_temp1) + (locals.var_k1__blk932 * locals.var_temp1_dn4)) * locals.var_tox1fact__blk913) - (assign40850_e46640 * locals.var_tox1fact__blk913_dn4)) / (locals.var_tox1fact__blk913 * locals.var_tox1fact__blk913)), (((((locals.var_k1__blk932_dn6 * locals.var_temp1) + (locals.var_k1__blk932 * locals.var_temp1_dn6)) * locals.var_tox1fact__blk913) - (assign40850_e46640 * locals.var_tox1fact__blk913_dn6)) / (locals.var_tox1fact__blk913 * locals.var_tox1fact__blk913)), (((((locals.var_k1__blk932_dn7 * locals.var_temp1) + (locals.var_k1__blk932 * locals.var_temp1_dn7)) * locals.var_tox1fact__blk913) - (assign40850_e46640 * locals.var_tox1fact__blk913_dn7)) / (locals.var_tox1fact__blk913 * locals.var_tox1fact__blk913)), (((((locals.var_k1__blk932_dn8 * locals.var_temp1) + (locals.var_k1__blk932 * locals.var_temp1_dn8)) * locals.var_tox1fact__blk913) - (assign40850_e46640 * locals.var_tox1fact__blk913_dn8)) / (locals.var_tox1fact__blk913 * locals.var_tox1fact__blk913)), (((((locals.var_k1__blk932_dn9 * locals.var_temp1) + (locals.var_k1__blk932 * locals.var_temp1_dn9)) * locals.var_tox1fact__blk913) - (assign40850_e46640 * locals.var_tox1fact__blk913_dn9)) / (locals.var_tox1fact__blk913 * locals.var_tox1fact__blk913)), );
            locals.var_qmfact1__blk1054_rv = 0.0;
        }
        if ((locals.var_guard1080 != 0.0) && (locals.var_guard1227 != 0.0)) {
            let assign40860_e46651: f64 = (locals.var_k2__blk933 * locals.var_temp2);
            let assign40860_e46652: f64 = (1.0 + assign40860_e46651);
            let assign40860_e46654: f64 = (assign40860_e46652 / locals.var_tox2fact__blk914);
            (locals.var_qmfact2__blk1055, locals.var_qmfact2__blk1055_dn4, locals.var_qmfact2__blk1055_dn6, locals.var_qmfact2__blk1055_dn7, locals.var_qmfact2__blk1055_dn8, locals.var_qmfact2__blk1055_dn9, ) = (assign40860_e46654, (((((locals.var_k2__blk933_dn4 * locals.var_temp2) + (locals.var_k2__blk933 * locals.var_temp2_dn4)) * locals.var_tox2fact__blk914) - (assign40860_e46652 * locals.var_tox2fact__blk914_dn4)) / (locals.var_tox2fact__blk914 * locals.var_tox2fact__blk914)), (((((locals.var_k2__blk933_dn6 * locals.var_temp2) + (locals.var_k2__blk933 * locals.var_temp2_dn6)) * locals.var_tox2fact__blk914) - (assign40860_e46652 * locals.var_tox2fact__blk914_dn6)) / (locals.var_tox2fact__blk914 * locals.var_tox2fact__blk914)), (((((locals.var_k2__blk933_dn7 * locals.var_temp2) + (locals.var_k2__blk933 * locals.var_temp2_dn7)) * locals.var_tox2fact__blk914) - (assign40860_e46652 * locals.var_tox2fact__blk914_dn7)) / (locals.var_tox2fact__blk914 * locals.var_tox2fact__blk914)), (((((locals.var_k2__blk933_dn8 * locals.var_temp2) + (locals.var_k2__blk933 * locals.var_temp2_dn8)) * locals.var_tox2fact__blk914) - (assign40860_e46652 * locals.var_tox2fact__blk914_dn8)) / (locals.var_tox2fact__blk914 * locals.var_tox2fact__blk914)), (((((locals.var_k2__blk933_dn9 * locals.var_temp2) + (locals.var_k2__blk933 * locals.var_temp2_dn9)) * locals.var_tox2fact__blk914) - (assign40860_e46652 * locals.var_tox2fact__blk914_dn9)) / (locals.var_tox2fact__blk914 * locals.var_tox2fact__blk914)), );
            locals.var_qmfact2__blk1055_rv = 0.0;
        }
        if ((locals.var_guard1080 != 0.0) && (locals.var_guard1227 == 0.0)) {
            (locals.var_qmfact1__blk1054, locals.var_qmfact1__blk1054_dn4, locals.var_qmfact1__blk1054_dn6, locals.var_qmfact1__blk1054_dn7, locals.var_qmfact1__blk1054_dn8, locals.var_qmfact1__blk1054_dn9, ) = (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_qmfact1__blk1054_rv = 0.0;
            (locals.var_qmfact2__blk1055, locals.var_qmfact2__blk1055_dn4, locals.var_qmfact2__blk1055_dn6, locals.var_qmfact2__blk1055_dn7, locals.var_qmfact2__blk1055_dn8, locals.var_qmfact2__blk1055_dn9, ) = (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_qmfact2__blk1055_rv = 0.0;
        }
        let assign40890_e46673: f64 = if locals.var_qis__blk938 > 1e-6 { 1.0 } else { 0.0 };
        locals.var_guard1228 = assign40890_e46673;
        locals.var_guard1228_rv = 0.0;
        let assign40900_e46676: f64 = if locals.var_qid__blk1003 > 1e-6 { 1.0 } else { 0.0 };
        locals.var_guard1229 = assign40900_e46676;
        locals.var_guard1229_rv = 0.0;
        let assign40910_e46678: f64 = (locals.var_a2d__blk1012).abs();
        let assign40910_e46680: f64 = if assign40910_e46678 < 0.01 { 1.0 } else { 0.0 };
        locals.var_guard1230 = assign40910_e46680;
        locals.var_guard1230_rv = 0.0;
        if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1228 != 0.0)) && (locals.var_guard1229 != 0.0)) && (locals.var_guard1230 != 0.0)) {
            let assign40920_e46690: f64 = (2.0 + locals.var_q1d__blk1001);
            let assign40920_e46693: f64 = (0.5 * locals.var_a1d__blk1011);
            let assign40920_e46694: f64 = (assign40920_e46690 + assign40920_e46693);
            let assign40920_e46697: f64 = (2.0 + locals.var_q2d__blk1002);
            let assign40920_e46699: f64 = (assign40920_e46697 * locals.var_a1d__blk1011);
            let assign40920_e46700: f64 = (assign40920_e46694 / assign40920_e46699);
            (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign40920_e46700, ((((locals.var_q1d__blk1001_dn4 + (0.5 * locals.var_a1d__blk1011_dn4)) * assign40920_e46699) - (assign40920_e46694 * ((locals.var_q2d__blk1002_dn4 * locals.var_a1d__blk1011) + (assign40920_e46697 * locals.var_a1d__blk1011_dn4)))) / (assign40920_e46699 * assign40920_e46699)), ((((locals.var_q1d__blk1001_dn6 + (0.5 * locals.var_a1d__blk1011_dn6)) * assign40920_e46699) - (assign40920_e46694 * ((locals.var_q2d__blk1002_dn6 * locals.var_a1d__blk1011) + (assign40920_e46697 * locals.var_a1d__blk1011_dn6)))) / (assign40920_e46699 * assign40920_e46699)), ((((locals.var_q1d__blk1001_dn7 + (0.5 * locals.var_a1d__blk1011_dn7)) * assign40920_e46699) - (assign40920_e46694 * ((locals.var_q2d__blk1002_dn7 * locals.var_a1d__blk1011) + (assign40920_e46697 * locals.var_a1d__blk1011_dn7)))) / (assign40920_e46699 * assign40920_e46699)), ((((locals.var_q1d__blk1001_dn8 + (0.5 * locals.var_a1d__blk1011_dn8)) * assign40920_e46699) - (assign40920_e46694 * ((locals.var_q2d__blk1002_dn8 * locals.var_a1d__blk1011) + (assign40920_e46697 * locals.var_a1d__blk1011_dn8)))) / (assign40920_e46699 * assign40920_e46699)), ((((locals.var_q1d__blk1001_dn9 + (0.5 * locals.var_a1d__blk1011_dn9)) * assign40920_e46699) - (assign40920_e46694 * ((locals.var_q2d__blk1002_dn9 * locals.var_a1d__blk1011) + (assign40920_e46697 * locals.var_a1d__blk1011_dn9)))) / (assign40920_e46699 * assign40920_e46699)), );
            locals.var_temp_rv = 0.0;
        }
        if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1228 != 0.0)) && (locals.var_guard1229 != 0.0)) && (locals.var_guard1230 != 0.0)) {
            let assign40930_e46712: f64 = (locals.var_temp * locals.var_a2d__blk1012);
            (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9, ) = (assign40930_e46712, ((locals.var_temp_dn4 * locals.var_a2d__blk1012) + (locals.var_temp * locals.var_a2d__blk1012_dn4)), ((locals.var_temp_dn6 * locals.var_a2d__blk1012) + (locals.var_temp * locals.var_a2d__blk1012_dn6)), ((locals.var_temp_dn7 * locals.var_a2d__blk1012) + (locals.var_temp * locals.var_a2d__blk1012_dn7)), ((locals.var_temp_dn8 * locals.var_a2d__blk1012) + (locals.var_temp * locals.var_a2d__blk1012_dn8)), ((locals.var_temp_dn9 * locals.var_a2d__blk1012) + (locals.var_temp * locals.var_a2d__blk1012_dn9)), );
            locals.var_temp1_rv = 0.0;
        }
        if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1228 != 0.0)) && (locals.var_guard1229 != 0.0)) && (locals.var_guard1230 != 0.0)) {
            let assign40940_e46724: f64 = (locals.var_temp1 * locals.var_temp1);
            (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9, ) = (assign40940_e46724, ((locals.var_temp1_dn4 * locals.var_temp1) + (locals.var_temp1 * locals.var_temp1_dn4)), ((locals.var_temp1_dn6 * locals.var_temp1) + (locals.var_temp1 * locals.var_temp1_dn6)), ((locals.var_temp1_dn7 * locals.var_temp1) + (locals.var_temp1 * locals.var_temp1_dn7)), ((locals.var_temp1_dn8 * locals.var_temp1) + (locals.var_temp1 * locals.var_temp1_dn8)), ((locals.var_temp1_dn9 * locals.var_temp1) + (locals.var_temp1 * locals.var_temp1_dn9)), );
            locals.var_temp2_rv = 0.0;
        }
        if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1228 != 0.0)) && (locals.var_guard1229 != 0.0)) && (locals.var_guard1230 != 0.0)) {
            let assign40950_e46736: f64 = (1.0 - locals.var_temp1);
            let assign40950_e46738: f64 = (assign40950_e46736 + locals.var_temp2);
            (locals.var_temp3, locals.var_temp3_dn4, locals.var_temp3_dn6, locals.var_temp3_dn7, locals.var_temp3_dn8, locals.var_temp3_dn9, ) = (assign40950_e46738, ((-locals.var_temp1_dn4) + locals.var_temp2_dn4), ((-locals.var_temp1_dn6) + locals.var_temp2_dn6), ((-locals.var_temp1_dn7) + locals.var_temp2_dn7), ((-locals.var_temp1_dn8) + locals.var_temp2_dn8), ((-locals.var_temp1_dn9) + locals.var_temp2_dn9), );
            locals.var_temp3_rv = 0.0;
        }
        if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1228 != 0.0)) && (locals.var_guard1229 != 0.0)) && (locals.var_guard1230 != 0.0)) {
            let assign40960_e46751: f64 = (locals.var_temp1 * locals.var_temp2);
            let assign40960_e46752: f64 = (locals.var_temp3 - assign40960_e46751);
            (locals.var_temp4, locals.var_temp4_dn4, locals.var_temp4_dn6, locals.var_temp4_dn7, locals.var_temp4_dn8, locals.var_temp4_dn9, ) = (assign40960_e46752, (locals.var_temp3_dn4 - ((locals.var_temp1_dn4 * locals.var_temp2) + (locals.var_temp1 * locals.var_temp2_dn4))), (locals.var_temp3_dn6 - ((locals.var_temp1_dn6 * locals.var_temp2) + (locals.var_temp1 * locals.var_temp2_dn6))), (locals.var_temp3_dn7 - ((locals.var_temp1_dn7 * locals.var_temp2) + (locals.var_temp1 * locals.var_temp2_dn7))), (locals.var_temp3_dn8 - ((locals.var_temp1_dn8 * locals.var_temp2) + (locals.var_temp1 * locals.var_temp2_dn8))), (locals.var_temp3_dn9 - ((locals.var_temp1_dn9 * locals.var_temp2) + (locals.var_temp1 * locals.var_temp2_dn9))), );
            locals.var_temp4_rv = 0.0;
        }
        if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1228 != 0.0)) && (locals.var_guard1229 != 0.0)) && (locals.var_guard1230 != 0.0)) {
            let assign40970_e46765: f64 = (2.0 * locals.var_qsqd__blk1006);
            let assign40970_e46769: f64 = (1.0 / locals.var_a1d__blk1011);
            let assign40970_e46770: f64 = (locals.var_temp - assign40970_e46769);
            let assign40970_e46771: f64 = (assign40970_e46765 * assign40970_e46770);
            let assign40970_e46773: f64 = (assign40970_e46771 * locals.var_temp4);
            let assign40970_e46774: f64 = (locals.var_k2q2d__blk1005 - assign40970_e46773);
            let assign40970_e46777: f64 = (2.0 + locals.var_q2d__blk1002);
            let assign40970_e46778: f64 = (assign40970_e46774 / assign40970_e46777);
            (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9, ) = (assign40970_e46778, ((((locals.var_k2q2d__blk1005_dn4 - (((((2.0 * locals.var_qsqd__blk1006_dn4) * assign40970_e46770) + (assign40970_e46765 * (locals.var_temp_dn4 - (-(locals.var_a1d__blk1011_dn4 / (locals.var_a1d__blk1011 * locals.var_a1d__blk1011)))))) * locals.var_temp4) + (assign40970_e46771 * locals.var_temp4_dn4))) * assign40970_e46777) - (assign40970_e46774 * locals.var_q2d__blk1002_dn4)) / (assign40970_e46777 * assign40970_e46777)), ((((locals.var_k2q2d__blk1005_dn6 - (((((2.0 * locals.var_qsqd__blk1006_dn6) * assign40970_e46770) + (assign40970_e46765 * (locals.var_temp_dn6 - (-(locals.var_a1d__blk1011_dn6 / (locals.var_a1d__blk1011 * locals.var_a1d__blk1011)))))) * locals.var_temp4) + (assign40970_e46771 * locals.var_temp4_dn6))) * assign40970_e46777) - (assign40970_e46774 * locals.var_q2d__blk1002_dn6)) / (assign40970_e46777 * assign40970_e46777)), ((((locals.var_k2q2d__blk1005_dn7 - (((((2.0 * locals.var_qsqd__blk1006_dn7) * assign40970_e46770) + (assign40970_e46765 * (locals.var_temp_dn7 - (-(locals.var_a1d__blk1011_dn7 / (locals.var_a1d__blk1011 * locals.var_a1d__blk1011)))))) * locals.var_temp4) + (assign40970_e46771 * locals.var_temp4_dn7))) * assign40970_e46777) - (assign40970_e46774 * locals.var_q2d__blk1002_dn7)) / (assign40970_e46777 * assign40970_e46777)), ((((locals.var_k2q2d__blk1005_dn8 - (((((2.0 * locals.var_qsqd__blk1006_dn8) * assign40970_e46770) + (assign40970_e46765 * (locals.var_temp_dn8 - (-(locals.var_a1d__blk1011_dn8 / (locals.var_a1d__blk1011 * locals.var_a1d__blk1011)))))) * locals.var_temp4) + (assign40970_e46771 * locals.var_temp4_dn8))) * assign40970_e46777) - (assign40970_e46774 * locals.var_q2d__blk1002_dn8)) / (assign40970_e46777 * assign40970_e46777)), ((((locals.var_k2q2d__blk1005_dn9 - (((((2.0 * locals.var_qsqd__blk1006_dn9) * assign40970_e46770) + (assign40970_e46765 * (locals.var_temp_dn9 - (-(locals.var_a1d__blk1011_dn9 / (locals.var_a1d__blk1011 * locals.var_a1d__blk1011)))))) * locals.var_temp4) + (assign40970_e46771 * locals.var_temp4_dn9))) * assign40970_e46777) - (assign40970_e46774 * locals.var_q2d__blk1002_dn9)) / (assign40970_e46777 * assign40970_e46777)), );
            locals.var_temp1_rv = 0.0;
        }
        if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1228 != 0.0)) && (locals.var_guard1229 != 0.0)) && (locals.var_guard1230 != 0.0)) {
            let assign40980_e46790: f64 = (locals.var_dqsqd_dxn_qi__blk1014 * locals.var_qid__blk1003);
            let assign40980_e46792: f64 = (assign40980_e46790 - locals.var_aexp1d__blk1007);
            let assign40980_e46794: f64 = (assign40980_e46792 / locals.var_a1d__blk1011);
            let assign40980_e46796: f64 = (assign40980_e46794 - locals.var_temp1);
            let assign40980_e46798: f64 = (assign40980_e46796 / locals.var_qid__blk1003);
            (locals.var_dqid_dxn_qi__blk1056, locals.var_dqid_dxn_qi__blk1056_dn4, locals.var_dqid_dxn_qi__blk1056_dn6, locals.var_dqid_dxn_qi__blk1056_dn7, locals.var_dqid_dxn_qi__blk1056_dn8, locals.var_dqid_dxn_qi__blk1056_dn9, ) = (assign40980_e46798, ((((((((((locals.var_dqsqd_dxn_qi__blk1014_dn4 * locals.var_qid__blk1003) + (locals.var_dqsqd_dxn_qi__blk1014 * locals.var_qid__blk1003_dn4)) - locals.var_aexp1d__blk1007_dn4) * locals.var_a1d__blk1011) - (assign40980_e46792 * locals.var_a1d__blk1011_dn4)) / (locals.var_a1d__blk1011 * locals.var_a1d__blk1011)) - locals.var_temp1_dn4) * locals.var_qid__blk1003) - (assign40980_e46796 * locals.var_qid__blk1003_dn4)) / (locals.var_qid__blk1003 * locals.var_qid__blk1003)), ((((((((((locals.var_dqsqd_dxn_qi__blk1014_dn6 * locals.var_qid__blk1003) + (locals.var_dqsqd_dxn_qi__blk1014 * locals.var_qid__blk1003_dn6)) - locals.var_aexp1d__blk1007_dn6) * locals.var_a1d__blk1011) - (assign40980_e46792 * locals.var_a1d__blk1011_dn6)) / (locals.var_a1d__blk1011 * locals.var_a1d__blk1011)) - locals.var_temp1_dn6) * locals.var_qid__blk1003) - (assign40980_e46796 * locals.var_qid__blk1003_dn6)) / (locals.var_qid__blk1003 * locals.var_qid__blk1003)), ((((((((((locals.var_dqsqd_dxn_qi__blk1014_dn7 * locals.var_qid__blk1003) + (locals.var_dqsqd_dxn_qi__blk1014 * locals.var_qid__blk1003_dn7)) - locals.var_aexp1d__blk1007_dn7) * locals.var_a1d__blk1011) - (assign40980_e46792 * locals.var_a1d__blk1011_dn7)) / (locals.var_a1d__blk1011 * locals.var_a1d__blk1011)) - locals.var_temp1_dn7) * locals.var_qid__blk1003) - (assign40980_e46796 * locals.var_qid__blk1003_dn7)) / (locals.var_qid__blk1003 * locals.var_qid__blk1003)), ((((((((((locals.var_dqsqd_dxn_qi__blk1014_dn8 * locals.var_qid__blk1003) + (locals.var_dqsqd_dxn_qi__blk1014 * locals.var_qid__blk1003_dn8)) - locals.var_aexp1d__blk1007_dn8) * locals.var_a1d__blk1011) - (assign40980_e46792 * locals.var_a1d__blk1011_dn8)) / (locals.var_a1d__blk1011 * locals.var_a1d__blk1011)) - locals.var_temp1_dn8) * locals.var_qid__blk1003) - (assign40980_e46796 * locals.var_qid__blk1003_dn8)) / (locals.var_qid__blk1003 * locals.var_qid__blk1003)), ((((((((((locals.var_dqsqd_dxn_qi__blk1014_dn9 * locals.var_qid__blk1003) + (locals.var_dqsqd_dxn_qi__blk1014 * locals.var_qid__blk1003_dn9)) - locals.var_aexp1d__blk1007_dn9) * locals.var_a1d__blk1011) - (assign40980_e46792 * locals.var_a1d__blk1011_dn9)) / (locals.var_a1d__blk1011 * locals.var_a1d__blk1011)) - locals.var_temp1_dn9) * locals.var_qid__blk1003) - (assign40980_e46796 * locals.var_qid__blk1003_dn9)) / (locals.var_qid__blk1003 * locals.var_qid__blk1003)), );
            locals.var_dqid_dxn_qi__blk1056_rv = 0.0;
        }
        if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1228 != 0.0)) && (locals.var_guard1229 != 0.0)) && (locals.var_guard1230 != 0.0)) {
            let assign40990_e46810: f64 = (locals.var_dqid_dxn_qi__blk1056 * locals.var_qid__blk1003);
            let assign40990_e46813: f64 = (locals.var_dqid_dxn_qi__blk1056 + 1.0);
            let assign40990_e46814: f64 = (assign40990_e46810 / assign40990_e46813);
            (locals.var_dd__blk1057, locals.var_dd__blk1057_dn4, locals.var_dd__blk1057_dn6, locals.var_dd__blk1057_dn7, locals.var_dd__blk1057_dn8, locals.var_dd__blk1057_dn9, ) = (assign40990_e46814, (((((locals.var_dqid_dxn_qi__blk1056_dn4 * locals.var_qid__blk1003) + (locals.var_dqid_dxn_qi__blk1056 * locals.var_qid__blk1003_dn4)) * assign40990_e46813) - (assign40990_e46810 * locals.var_dqid_dxn_qi__blk1056_dn4)) / (assign40990_e46813 * assign40990_e46813)), (((((locals.var_dqid_dxn_qi__blk1056_dn6 * locals.var_qid__blk1003) + (locals.var_dqid_dxn_qi__blk1056 * locals.var_qid__blk1003_dn6)) * assign40990_e46813) - (assign40990_e46810 * locals.var_dqid_dxn_qi__blk1056_dn6)) / (assign40990_e46813 * assign40990_e46813)), (((((locals.var_dqid_dxn_qi__blk1056_dn7 * locals.var_qid__blk1003) + (locals.var_dqid_dxn_qi__blk1056 * locals.var_qid__blk1003_dn7)) * assign40990_e46813) - (assign40990_e46810 * locals.var_dqid_dxn_qi__blk1056_dn7)) / (assign40990_e46813 * assign40990_e46813)), (((((locals.var_dqid_dxn_qi__blk1056_dn8 * locals.var_qid__blk1003) + (locals.var_dqid_dxn_qi__blk1056 * locals.var_qid__blk1003_dn8)) * assign40990_e46813) - (assign40990_e46810 * locals.var_dqid_dxn_qi__blk1056_dn8)) / (assign40990_e46813 * assign40990_e46813)), (((((locals.var_dqid_dxn_qi__blk1056_dn9 * locals.var_qid__blk1003) + (locals.var_dqid_dxn_qi__blk1056 * locals.var_qid__blk1003_dn9)) * assign40990_e46813) - (assign40990_e46810 * locals.var_dqid_dxn_qi__blk1056_dn9)) / (assign40990_e46813 * assign40990_e46813)), );
            locals.var_dd__blk1057_rv = 0.0;
        }
        if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1228 != 0.0)) && (locals.var_guard1229 != 0.0)) && (locals.var_guard1230 == 0.0)) {
            let assign41000_e46827: f64 = (locals.var_dqsqd_dxn_qi__blk1014 * locals.var_sumd__blk1013);
            let assign41000_e46830: f64 = (locals.var_a1d__blk1011 * locals.var_a2d__blk1012);
            let assign41000_e46831: f64 = (assign41000_e46827 / assign41000_e46830);
            let assign41000_e46834: f64 = (locals.var_aexp1d__blk1007 / locals.var_a1d__blk1011);
            let assign41000_e46837: f64 = (locals.var_aexp2d__blk1008 / locals.var_a2d__blk1012);
            let assign41000_e46838: f64 = (assign41000_e46834 + assign41000_e46837);
            let assign41000_e46840: f64 = (assign41000_e46838 / locals.var_qid__blk1003);
            let assign41000_e46841: f64 = (assign41000_e46831 - assign41000_e46840);
            (locals.var_dqid_dxn_qi__blk1056, locals.var_dqid_dxn_qi__blk1056_dn4, locals.var_dqid_dxn_qi__blk1056_dn6, locals.var_dqid_dxn_qi__blk1056_dn7, locals.var_dqid_dxn_qi__blk1056_dn8, locals.var_dqid_dxn_qi__blk1056_dn9, ) = (assign41000_e46841, ((((((locals.var_dqsqd_dxn_qi__blk1014_dn4 * locals.var_sumd__blk1013) + (locals.var_dqsqd_dxn_qi__blk1014 * locals.var_sumd__blk1013_dn4)) * assign41000_e46830) - (assign41000_e46827 * ((locals.var_a1d__blk1011_dn4 * locals.var_a2d__blk1012) + (locals.var_a1d__blk1011 * locals.var_a2d__blk1012_dn4)))) / (assign41000_e46830 * assign41000_e46830)) - (((((((locals.var_aexp1d__blk1007_dn4 * locals.var_a1d__blk1011) - (locals.var_aexp1d__blk1007 * locals.var_a1d__blk1011_dn4)) / (locals.var_a1d__blk1011 * locals.var_a1d__blk1011)) + (((locals.var_aexp2d__blk1008_dn4 * locals.var_a2d__blk1012) - (locals.var_aexp2d__blk1008 * locals.var_a2d__blk1012_dn4)) / (locals.var_a2d__blk1012 * locals.var_a2d__blk1012))) * locals.var_qid__blk1003) - (assign41000_e46838 * locals.var_qid__blk1003_dn4)) / (locals.var_qid__blk1003 * locals.var_qid__blk1003))), ((((((locals.var_dqsqd_dxn_qi__blk1014_dn6 * locals.var_sumd__blk1013) + (locals.var_dqsqd_dxn_qi__blk1014 * locals.var_sumd__blk1013_dn6)) * assign41000_e46830) - (assign41000_e46827 * ((locals.var_a1d__blk1011_dn6 * locals.var_a2d__blk1012) + (locals.var_a1d__blk1011 * locals.var_a2d__blk1012_dn6)))) / (assign41000_e46830 * assign41000_e46830)) - (((((((locals.var_aexp1d__blk1007_dn6 * locals.var_a1d__blk1011) - (locals.var_aexp1d__blk1007 * locals.var_a1d__blk1011_dn6)) / (locals.var_a1d__blk1011 * locals.var_a1d__blk1011)) + (((locals.var_aexp2d__blk1008_dn6 * locals.var_a2d__blk1012) - (locals.var_aexp2d__blk1008 * locals.var_a2d__blk1012_dn6)) / (locals.var_a2d__blk1012 * locals.var_a2d__blk1012))) * locals.var_qid__blk1003) - (assign41000_e46838 * locals.var_qid__blk1003_dn6)) / (locals.var_qid__blk1003 * locals.var_qid__blk1003))), ((((((locals.var_dqsqd_dxn_qi__blk1014_dn7 * locals.var_sumd__blk1013) + (locals.var_dqsqd_dxn_qi__blk1014 * locals.var_sumd__blk1013_dn7)) * assign41000_e46830) - (assign41000_e46827 * ((locals.var_a1d__blk1011_dn7 * locals.var_a2d__blk1012) + (locals.var_a1d__blk1011 * locals.var_a2d__blk1012_dn7)))) / (assign41000_e46830 * assign41000_e46830)) - (((((((locals.var_aexp1d__blk1007_dn7 * locals.var_a1d__blk1011) - (locals.var_aexp1d__blk1007 * locals.var_a1d__blk1011_dn7)) / (locals.var_a1d__blk1011 * locals.var_a1d__blk1011)) + (((locals.var_aexp2d__blk1008_dn7 * locals.var_a2d__blk1012) - (locals.var_aexp2d__blk1008 * locals.var_a2d__blk1012_dn7)) / (locals.var_a2d__blk1012 * locals.var_a2d__blk1012))) * locals.var_qid__blk1003) - (assign41000_e46838 * locals.var_qid__blk1003_dn7)) / (locals.var_qid__blk1003 * locals.var_qid__blk1003))), ((((((locals.var_dqsqd_dxn_qi__blk1014_dn8 * locals.var_sumd__blk1013) + (locals.var_dqsqd_dxn_qi__blk1014 * locals.var_sumd__blk1013_dn8)) * assign41000_e46830) - (assign41000_e46827 * ((locals.var_a1d__blk1011_dn8 * locals.var_a2d__blk1012) + (locals.var_a1d__blk1011 * locals.var_a2d__blk1012_dn8)))) / (assign41000_e46830 * assign41000_e46830)) - (((((((locals.var_aexp1d__blk1007_dn8 * locals.var_a1d__blk1011) - (locals.var_aexp1d__blk1007 * locals.var_a1d__blk1011_dn8)) / (locals.var_a1d__blk1011 * locals.var_a1d__blk1011)) + (((locals.var_aexp2d__blk1008_dn8 * locals.var_a2d__blk1012) - (locals.var_aexp2d__blk1008 * locals.var_a2d__blk1012_dn8)) / (locals.var_a2d__blk1012 * locals.var_a2d__blk1012))) * locals.var_qid__blk1003) - (assign41000_e46838 * locals.var_qid__blk1003_dn8)) / (locals.var_qid__blk1003 * locals.var_qid__blk1003))), ((((((locals.var_dqsqd_dxn_qi__blk1014_dn9 * locals.var_sumd__blk1013) + (locals.var_dqsqd_dxn_qi__blk1014 * locals.var_sumd__blk1013_dn9)) * assign41000_e46830) - (assign41000_e46827 * ((locals.var_a1d__blk1011_dn9 * locals.var_a2d__blk1012) + (locals.var_a1d__blk1011 * locals.var_a2d__blk1012_dn9)))) / (assign41000_e46830 * assign41000_e46830)) - (((((((locals.var_aexp1d__blk1007_dn9 * locals.var_a1d__blk1011) - (locals.var_aexp1d__blk1007 * locals.var_a1d__blk1011_dn9)) / (locals.var_a1d__blk1011 * locals.var_a1d__blk1011)) + (((locals.var_aexp2d__blk1008_dn9 * locals.var_a2d__blk1012) - (locals.var_aexp2d__blk1008 * locals.var_a2d__blk1012_dn9)) / (locals.var_a2d__blk1012 * locals.var_a2d__blk1012))) * locals.var_qid__blk1003) - (assign41000_e46838 * locals.var_qid__blk1003_dn9)) / (locals.var_qid__blk1003 * locals.var_qid__blk1003))), );
            locals.var_dqid_dxn_qi__blk1056_rv = 0.0;
        }
        if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1228 != 0.0)) && (locals.var_guard1229 != 0.0)) && (locals.var_guard1230 == 0.0)) {
            let assign41010_e46854: f64 = (locals.var_dqid_dxn_qi__blk1056 * locals.var_qid__blk1003);
            let assign41010_e46857: f64 = (locals.var_dqid_dxn_qi__blk1056 + 1.0);
            let assign41010_e46858: f64 = (assign41010_e46854 / assign41010_e46857);
            (locals.var_dd__blk1057, locals.var_dd__blk1057_dn4, locals.var_dd__blk1057_dn6, locals.var_dd__blk1057_dn7, locals.var_dd__blk1057_dn8, locals.var_dd__blk1057_dn9, ) = (assign41010_e46858, (((((locals.var_dqid_dxn_qi__blk1056_dn4 * locals.var_qid__blk1003) + (locals.var_dqid_dxn_qi__blk1056 * locals.var_qid__blk1003_dn4)) * assign41010_e46857) - (assign41010_e46854 * locals.var_dqid_dxn_qi__blk1056_dn4)) / (assign41010_e46857 * assign41010_e46857)), (((((locals.var_dqid_dxn_qi__blk1056_dn6 * locals.var_qid__blk1003) + (locals.var_dqid_dxn_qi__blk1056 * locals.var_qid__blk1003_dn6)) * assign41010_e46857) - (assign41010_e46854 * locals.var_dqid_dxn_qi__blk1056_dn6)) / (assign41010_e46857 * assign41010_e46857)), (((((locals.var_dqid_dxn_qi__blk1056_dn7 * locals.var_qid__blk1003) + (locals.var_dqid_dxn_qi__blk1056 * locals.var_qid__blk1003_dn7)) * assign41010_e46857) - (assign41010_e46854 * locals.var_dqid_dxn_qi__blk1056_dn7)) / (assign41010_e46857 * assign41010_e46857)), (((((locals.var_dqid_dxn_qi__blk1056_dn8 * locals.var_qid__blk1003) + (locals.var_dqid_dxn_qi__blk1056 * locals.var_qid__blk1003_dn8)) * assign41010_e46857) - (assign41010_e46854 * locals.var_dqid_dxn_qi__blk1056_dn8)) / (assign41010_e46857 * assign41010_e46857)), (((((locals.var_dqid_dxn_qi__blk1056_dn9 * locals.var_qid__blk1003) + (locals.var_dqid_dxn_qi__blk1056 * locals.var_qid__blk1003_dn9)) * assign41010_e46857) - (assign41010_e46854 * locals.var_dqid_dxn_qi__blk1056_dn9)) / (assign41010_e46857 * assign41010_e46857)), );
            locals.var_dd__blk1057_rv = 0.0;
        }
        if (((locals.var_guard1080 != 0.0) && (locals.var_guard1228 != 0.0)) && (locals.var_guard1229 == 0.0)) {
            (locals.var_dd__blk1057, locals.var_dd__blk1057_dn4, locals.var_dd__blk1057_dn6, locals.var_dd__blk1057_dn7, locals.var_dd__blk1057_dn8, locals.var_dd__blk1057_dn9, ) = (locals.var_dinf__blk974, locals.var_dinf__blk974_dn4, locals.var_dinf__blk974_dn6, locals.var_dinf__blk974_dn7, locals.var_dinf__blk974_dn8, locals.var_dinf__blk974_dn9, );
            locals.var_dd__blk1057_rv = 0.0;
        }
        if ((locals.var_guard1080 != 0.0) && (locals.var_guard1228 != 0.0)) {
            let assign41030_e46875: f64 = (locals.var_dd__blk1057 - locals.var_ds__blk981);
            (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9, ) = (assign41030_e46875, (locals.var_dd__blk1057_dn4 - locals.var_ds__blk981_dn4), (locals.var_dd__blk1057_dn6 - locals.var_ds__blk981_dn6), (locals.var_dd__blk1057_dn7 - locals.var_ds__blk981_dn7), (locals.var_dd__blk1057_dn8 - locals.var_ds__blk981_dn8), (locals.var_dd__blk1057_dn9 - locals.var_ds__blk981_dn9), );
            locals.var_temp1_rv = 0.0;
        }
        if ((locals.var_guard1080 != 0.0) && (locals.var_guard1228 != 0.0)) {
            let assign41040_e46884: f64 = (36.0 * locals.var_temp1);
            let assign41040_e46886: f64 = (assign41040_e46884 * locals.var_temp1);
            let assign41040_e46887: f64 = (1.0 + assign41040_e46886);
            (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9, ) = (assign41040_e46887, (((36.0 * locals.var_temp1_dn4) * locals.var_temp1) + (assign41040_e46884 * locals.var_temp1_dn4)), (((36.0 * locals.var_temp1_dn6) * locals.var_temp1) + (assign41040_e46884 * locals.var_temp1_dn6)), (((36.0 * locals.var_temp1_dn7) * locals.var_temp1) + (assign41040_e46884 * locals.var_temp1_dn7)), (((36.0 * locals.var_temp1_dn8) * locals.var_temp1) + (assign41040_e46884 * locals.var_temp1_dn8)), (((36.0 * locals.var_temp1_dn9) * locals.var_temp1) + (assign41040_e46884 * locals.var_temp1_dn9)), );
            locals.var_temp2_rv = 0.0;
        }
        let assign41050_e46891: f64 = (locals.var_temp1).abs();
        let assign41050_e46893: f64 = if assign41050_e46891 > 0.001 { 1.0 } else { 0.0 };
        locals.var_guard1231 = assign41050_e46893;
        locals.var_guard1231_rv = 0.0;
        if (((locals.var_guard1080 != 0.0) && (locals.var_guard1228 != 0.0)) && (locals.var_guard1231 != 0.0)) {
            let assign41060_e46901: f64 = (locals.var_qid__blk1003 - locals.var_qis__blk938);
            (locals.var_temp3, locals.var_temp3_dn4, locals.var_temp3_dn6, locals.var_temp3_dn7, locals.var_temp3_dn8, locals.var_temp3_dn9, ) = (assign41060_e46901, (locals.var_qid__blk1003_dn4 - locals.var_qis__blk938_dn4), (locals.var_qid__blk1003_dn6 - locals.var_qis__blk938_dn6), (locals.var_qid__blk1003_dn7 - locals.var_qis__blk938_dn7), (locals.var_qid__blk1003_dn8 - locals.var_qis__blk938_dn8), (locals.var_qid__blk1003_dn9 - locals.var_qis__blk938_dn9), );
            locals.var_temp3_rv = 0.0;
        }
        if (((locals.var_guard1080 != 0.0) && (locals.var_guard1228 != 0.0)) && (locals.var_guard1231 != 0.0)) {
            let assign41070_e46912: f64 = (locals.var_dd__blk1057 * locals.var_dxdrift__blk1017);
            let assign41070_e46913: f64 = (locals.var_temp3 - assign41070_e46912);
            (locals.var_ls__blk1058, locals.var_ls__blk1058_dn4, locals.var_ls__blk1058_dn6, locals.var_ls__blk1058_dn7, locals.var_ls__blk1058_dn8, locals.var_ls__blk1058_dn9, ) = (assign41070_e46913, (locals.var_temp3_dn4 - ((locals.var_dd__blk1057_dn4 * locals.var_dxdrift__blk1017) + (locals.var_dd__blk1057 * locals.var_dxdrift__blk1017_dn4))), (locals.var_temp3_dn6 - ((locals.var_dd__blk1057_dn6 * locals.var_dxdrift__blk1017) + (locals.var_dd__blk1057 * locals.var_dxdrift__blk1017_dn6))), (locals.var_temp3_dn7 - ((locals.var_dd__blk1057_dn7 * locals.var_dxdrift__blk1017) + (locals.var_dd__blk1057 * locals.var_dxdrift__blk1017_dn7))), (locals.var_temp3_dn8 - ((locals.var_dd__blk1057_dn8 * locals.var_dxdrift__blk1017) + (locals.var_dd__blk1057 * locals.var_dxdrift__blk1017_dn8))), (locals.var_temp3_dn9 - ((locals.var_dd__blk1057_dn9 * locals.var_dxdrift__blk1017) + (locals.var_dd__blk1057 * locals.var_dxdrift__blk1017_dn9))), );
            locals.var_ls__blk1058_rv = 0.0;
        }
        if (((locals.var_guard1080 != 0.0) && (locals.var_guard1228 != 0.0)) && (locals.var_guard1231 != 0.0)) {
            let assign41080_e46924: f64 = (locals.var_ds__blk981 * locals.var_dxdrift__blk1017);
            let assign41080_e46925: f64 = (locals.var_temp3 - assign41080_e46924);
            (locals.var_ld__blk1059, locals.var_ld__blk1059_dn4, locals.var_ld__blk1059_dn6, locals.var_ld__blk1059_dn7, locals.var_ld__blk1059_dn8, locals.var_ld__blk1059_dn9, ) = (assign41080_e46925, (locals.var_temp3_dn4 - ((locals.var_ds__blk981_dn4 * locals.var_dxdrift__blk1017) + (locals.var_ds__blk981 * locals.var_dxdrift__blk1017_dn4))), (locals.var_temp3_dn6 - ((locals.var_ds__blk981_dn6 * locals.var_dxdrift__blk1017) + (locals.var_ds__blk981 * locals.var_dxdrift__blk1017_dn6))), (locals.var_temp3_dn7 - ((locals.var_ds__blk981_dn7 * locals.var_dxdrift__blk1017) + (locals.var_ds__blk981 * locals.var_dxdrift__blk1017_dn7))), (locals.var_temp3_dn8 - ((locals.var_ds__blk981_dn8 * locals.var_dxdrift__blk1017) + (locals.var_ds__blk981 * locals.var_dxdrift__blk1017_dn8))), (locals.var_temp3_dn9 - ((locals.var_ds__blk981_dn9 * locals.var_dxdrift__blk1017) + (locals.var_ds__blk981 * locals.var_dxdrift__blk1017_dn9))), );
            locals.var_ld__blk1059_rv = 0.0;
        }
        if (((locals.var_guard1080 != 0.0) && (locals.var_guard1228 != 0.0)) && (locals.var_guard1231 != 0.0)) {
            let assign41090_e46935: f64 = (locals.var_ls__blk1058 * locals.var_ls__blk1058);
            let assign41090_e46937: f64 = (assign41090_e46935 + locals.var_temp2);
            let assign41090_e46938: f64 = (assign41090_e46937).sqrt();
            (locals.var_us__blk1060, locals.var_us__blk1060_dn4, locals.var_us__blk1060_dn6, locals.var_us__blk1060_dn7, locals.var_us__blk1060_dn8, locals.var_us__blk1060_dn9, ) = (assign41090_e46938, ((((locals.var_ls__blk1058_dn4 * locals.var_ls__blk1058) + (locals.var_ls__blk1058 * locals.var_ls__blk1058_dn4)) + locals.var_temp2_dn4) / (2.0 * assign41090_e46938)), ((((locals.var_ls__blk1058_dn6 * locals.var_ls__blk1058) + (locals.var_ls__blk1058 * locals.var_ls__blk1058_dn6)) + locals.var_temp2_dn6) / (2.0 * assign41090_e46938)), ((((locals.var_ls__blk1058_dn7 * locals.var_ls__blk1058) + (locals.var_ls__blk1058 * locals.var_ls__blk1058_dn7)) + locals.var_temp2_dn7) / (2.0 * assign41090_e46938)), ((((locals.var_ls__blk1058_dn8 * locals.var_ls__blk1058) + (locals.var_ls__blk1058 * locals.var_ls__blk1058_dn8)) + locals.var_temp2_dn8) / (2.0 * assign41090_e46938)), ((((locals.var_ls__blk1058_dn9 * locals.var_ls__blk1058) + (locals.var_ls__blk1058 * locals.var_ls__blk1058_dn9)) + locals.var_temp2_dn9) / (2.0 * assign41090_e46938)), );
            locals.var_us__blk1060_rv = 0.0;
        }
        if (((locals.var_guard1080 != 0.0) && (locals.var_guard1228 != 0.0)) && (locals.var_guard1231 != 0.0)) {
            let assign41100_e46948: f64 = (locals.var_ld__blk1059 * locals.var_ld__blk1059);
            let assign41100_e46950: f64 = (assign41100_e46948 + locals.var_temp2);
            let assign41100_e46951: f64 = (assign41100_e46950).sqrt();
            (locals.var_ud__blk1061, locals.var_ud__blk1061_dn4, locals.var_ud__blk1061_dn6, locals.var_ud__blk1061_dn7, locals.var_ud__blk1061_dn8, locals.var_ud__blk1061_dn9, ) = (assign41100_e46951, ((((locals.var_ld__blk1059_dn4 * locals.var_ld__blk1059) + (locals.var_ld__blk1059 * locals.var_ld__blk1059_dn4)) + locals.var_temp2_dn4) / (2.0 * assign41100_e46951)), ((((locals.var_ld__blk1059_dn6 * locals.var_ld__blk1059) + (locals.var_ld__blk1059 * locals.var_ld__blk1059_dn6)) + locals.var_temp2_dn6) / (2.0 * assign41100_e46951)), ((((locals.var_ld__blk1059_dn7 * locals.var_ld__blk1059) + (locals.var_ld__blk1059 * locals.var_ld__blk1059_dn7)) + locals.var_temp2_dn7) / (2.0 * assign41100_e46951)), ((((locals.var_ld__blk1059_dn8 * locals.var_ld__blk1059) + (locals.var_ld__blk1059 * locals.var_ld__blk1059_dn8)) + locals.var_temp2_dn8) / (2.0 * assign41100_e46951)), ((((locals.var_ld__blk1059_dn9 * locals.var_ld__blk1059) + (locals.var_ld__blk1059 * locals.var_ld__blk1059_dn9)) + locals.var_temp2_dn9) / (2.0 * assign41100_e46951)), );
            locals.var_ud__blk1061_rv = 0.0;
        }
        if (((locals.var_guard1080 != 0.0) && (locals.var_guard1228 != 0.0)) && (locals.var_guard1231 != 0.0)) {
            let assign41110_e46961: f64 = (0.25 / locals.var_temp1);
            let assign41110_e46964: f64 = (locals.var_ud__blk1061 * locals.var_ls__blk1058);
            let assign41110_e46967: f64 = (locals.var_us__blk1060 * locals.var_ld__blk1059);
            let assign41110_e46968: f64 = (assign41110_e46964 - assign41110_e46967);
            let assign41110_e46972: f64 = (locals.var_ld__blk1059 + locals.var_ud__blk1061);
            let assign41110_e46975: f64 = (locals.var_ls__blk1058 + locals.var_us__blk1060);
            let assign41110_e46976: f64 = (assign41110_e46972 / assign41110_e46975);
            let assign41110_e46977: f64 = (assign41110_e46976).ln();
            let assign41110_e46978: f64 = (locals.var_temp2 * assign41110_e46977);
            let assign41110_e46979: f64 = (assign41110_e46968 + assign41110_e46978);
            let assign41110_e46980: f64 = (assign41110_e46961 * assign41110_e46979);
            (locals.var_idrift2__blk1062, locals.var_idrift2__blk1062_dn4, locals.var_idrift2__blk1062_dn6, locals.var_idrift2__blk1062_dn7, locals.var_idrift2__blk1062_dn8, locals.var_idrift2__blk1062_dn9, ) = (assign41110_e46980, (((-((0.25 * locals.var_temp1_dn4) / (locals.var_temp1 * locals.var_temp1))) * assign41110_e46979) + (assign41110_e46961 * ((((locals.var_ud__blk1061_dn4 * locals.var_ls__blk1058) + (locals.var_ud__blk1061 * locals.var_ls__blk1058_dn4)) - ((locals.var_us__blk1060_dn4 * locals.var_ld__blk1059) + (locals.var_us__blk1060 * locals.var_ld__blk1059_dn4))) + ((locals.var_temp2_dn4 * assign41110_e46977) + (locals.var_temp2 * (((((locals.var_ld__blk1059_dn4 + locals.var_ud__blk1061_dn4) * assign41110_e46975) - (assign41110_e46972 * (locals.var_ls__blk1058_dn4 + locals.var_us__blk1060_dn4))) / (assign41110_e46975 * assign41110_e46975)) / assign41110_e46976)))))), (((-((0.25 * locals.var_temp1_dn6) / (locals.var_temp1 * locals.var_temp1))) * assign41110_e46979) + (assign41110_e46961 * ((((locals.var_ud__blk1061_dn6 * locals.var_ls__blk1058) + (locals.var_ud__blk1061 * locals.var_ls__blk1058_dn6)) - ((locals.var_us__blk1060_dn6 * locals.var_ld__blk1059) + (locals.var_us__blk1060 * locals.var_ld__blk1059_dn6))) + ((locals.var_temp2_dn6 * assign41110_e46977) + (locals.var_temp2 * (((((locals.var_ld__blk1059_dn6 + locals.var_ud__blk1061_dn6) * assign41110_e46975) - (assign41110_e46972 * (locals.var_ls__blk1058_dn6 + locals.var_us__blk1060_dn6))) / (assign41110_e46975 * assign41110_e46975)) / assign41110_e46976)))))), (((-((0.25 * locals.var_temp1_dn7) / (locals.var_temp1 * locals.var_temp1))) * assign41110_e46979) + (assign41110_e46961 * ((((locals.var_ud__blk1061_dn7 * locals.var_ls__blk1058) + (locals.var_ud__blk1061 * locals.var_ls__blk1058_dn7)) - ((locals.var_us__blk1060_dn7 * locals.var_ld__blk1059) + (locals.var_us__blk1060 * locals.var_ld__blk1059_dn7))) + ((locals.var_temp2_dn7 * assign41110_e46977) + (locals.var_temp2 * (((((locals.var_ld__blk1059_dn7 + locals.var_ud__blk1061_dn7) * assign41110_e46975) - (assign41110_e46972 * (locals.var_ls__blk1058_dn7 + locals.var_us__blk1060_dn7))) / (assign41110_e46975 * assign41110_e46975)) / assign41110_e46976)))))), (((-((0.25 * locals.var_temp1_dn8) / (locals.var_temp1 * locals.var_temp1))) * assign41110_e46979) + (assign41110_e46961 * ((((locals.var_ud__blk1061_dn8 * locals.var_ls__blk1058) + (locals.var_ud__blk1061 * locals.var_ls__blk1058_dn8)) - ((locals.var_us__blk1060_dn8 * locals.var_ld__blk1059) + (locals.var_us__blk1060 * locals.var_ld__blk1059_dn8))) + ((locals.var_temp2_dn8 * assign41110_e46977) + (locals.var_temp2 * (((((locals.var_ld__blk1059_dn8 + locals.var_ud__blk1061_dn8) * assign41110_e46975) - (assign41110_e46972 * (locals.var_ls__blk1058_dn8 + locals.var_us__blk1060_dn8))) / (assign41110_e46975 * assign41110_e46975)) / assign41110_e46976)))))), (((-((0.25 * locals.var_temp1_dn9) / (locals.var_temp1 * locals.var_temp1))) * assign41110_e46979) + (assign41110_e46961 * ((((locals.var_ud__blk1061_dn9 * locals.var_ls__blk1058) + (locals.var_ud__blk1061 * locals.var_ls__blk1058_dn9)) - ((locals.var_us__blk1060_dn9 * locals.var_ld__blk1059) + (locals.var_us__blk1060 * locals.var_ld__blk1059_dn9))) + ((locals.var_temp2_dn9 * assign41110_e46977) + (locals.var_temp2 * (((((locals.var_ld__blk1059_dn9 + locals.var_ud__blk1061_dn9) * assign41110_e46975) - (assign41110_e46972 * (locals.var_ls__blk1058_dn9 + locals.var_us__blk1060_dn9))) / (assign41110_e46975 * assign41110_e46975)) / assign41110_e46976)))))), );
            locals.var_idrift2__blk1062_rv = 0.0;
        }
        if (((locals.var_guard1080 != 0.0) && (locals.var_guard1228 != 0.0)) && (locals.var_guard1231 == 0.0)) {
            let assign41120_e46991: f64 = (locals.var_dxdrift__blk1017 * locals.var_temp1);
            (locals.var_temp3, locals.var_temp3_dn4, locals.var_temp3_dn6, locals.var_temp3_dn7, locals.var_temp3_dn8, locals.var_temp3_dn9, ) = (assign41120_e46991, ((locals.var_dxdrift__blk1017_dn4 * locals.var_temp1) + (locals.var_dxdrift__blk1017 * locals.var_temp1_dn4)), ((locals.var_dxdrift__blk1017_dn6 * locals.var_temp1) + (locals.var_dxdrift__blk1017 * locals.var_temp1_dn6)), ((locals.var_dxdrift__blk1017_dn7 * locals.var_temp1) + (locals.var_dxdrift__blk1017 * locals.var_temp1_dn7)), ((locals.var_dxdrift__blk1017_dn8 * locals.var_temp1) + (locals.var_dxdrift__blk1017 * locals.var_temp1_dn8)), ((locals.var_dxdrift__blk1017_dn9 * locals.var_temp1) + (locals.var_dxdrift__blk1017 * locals.var_temp1_dn9)), );
            locals.var_temp3_rv = 0.0;
        }
        if (((locals.var_guard1080 != 0.0) && (locals.var_guard1228 != 0.0)) && (locals.var_guard1231 == 0.0)) {
            let assign41130_e47001: f64 = (-0.25);
            let assign41130_e47003: f64 = (assign41130_e47001 * 0.1666666666667);
            let assign41130_e47005: f64 = (assign41130_e47003 * locals.var_dxdrift__blk1017);
            let assign41130_e47007: f64 = (assign41130_e47005 * locals.var_temp3);
            let assign41130_e47009: f64 = (assign41130_e47007 * locals.var_temp3);
            let assign41130_e47011: f64 = (locals.var_temp2).sqrt();
            let assign41130_e47012: f64 = (assign41130_e47009 / assign41130_e47011);
            (locals.var_idrift2__blk1062, locals.var_idrift2__blk1062_dn4, locals.var_idrift2__blk1062_dn6, locals.var_idrift2__blk1062_dn7, locals.var_idrift2__blk1062_dn8, locals.var_idrift2__blk1062_dn9, ) = (assign41130_e47012, ((((((((assign41130_e47003 * locals.var_dxdrift__blk1017_dn4) * locals.var_temp3) + (assign41130_e47005 * locals.var_temp3_dn4)) * locals.var_temp3) + (assign41130_e47007 * locals.var_temp3_dn4)) * assign41130_e47011) - (assign41130_e47009 * (locals.var_temp2_dn4 / (2.0 * assign41130_e47011)))) / (assign41130_e47011 * assign41130_e47011)), ((((((((assign41130_e47003 * locals.var_dxdrift__blk1017_dn6) * locals.var_temp3) + (assign41130_e47005 * locals.var_temp3_dn6)) * locals.var_temp3) + (assign41130_e47007 * locals.var_temp3_dn6)) * assign41130_e47011) - (assign41130_e47009 * (locals.var_temp2_dn6 / (2.0 * assign41130_e47011)))) / (assign41130_e47011 * assign41130_e47011)), ((((((((assign41130_e47003 * locals.var_dxdrift__blk1017_dn7) * locals.var_temp3) + (assign41130_e47005 * locals.var_temp3_dn7)) * locals.var_temp3) + (assign41130_e47007 * locals.var_temp3_dn7)) * assign41130_e47011) - (assign41130_e47009 * (locals.var_temp2_dn7 / (2.0 * assign41130_e47011)))) / (assign41130_e47011 * assign41130_e47011)), ((((((((assign41130_e47003 * locals.var_dxdrift__blk1017_dn8) * locals.var_temp3) + (assign41130_e47005 * locals.var_temp3_dn8)) * locals.var_temp3) + (assign41130_e47007 * locals.var_temp3_dn8)) * assign41130_e47011) - (assign41130_e47009 * (locals.var_temp2_dn8 / (2.0 * assign41130_e47011)))) / (assign41130_e47011 * assign41130_e47011)), ((((((((assign41130_e47003 * locals.var_dxdrift__blk1017_dn9) * locals.var_temp3) + (assign41130_e47005 * locals.var_temp3_dn9)) * locals.var_temp3) + (assign41130_e47007 * locals.var_temp3_dn9)) * assign41130_e47011) - (assign41130_e47009 * (locals.var_temp2_dn9 / (2.0 * assign41130_e47011)))) / (assign41130_e47011 * assign41130_e47011)), );
            locals.var_idrift2__blk1062_rv = 0.0;
        }
        if ((locals.var_guard1080 != 0.0) && (locals.var_guard1228 == 0.0)) {
            (locals.var_dd__blk1057, locals.var_dd__blk1057_dn4, locals.var_dd__blk1057_dn6, locals.var_dd__blk1057_dn7, locals.var_dd__blk1057_dn8, locals.var_dd__blk1057_dn9, ) = (locals.var_dinf__blk974, locals.var_dinf__blk974_dn4, locals.var_dinf__blk974_dn6, locals.var_dinf__blk974_dn7, locals.var_dinf__blk974_dn8, locals.var_dinf__blk974_dn9, );
            locals.var_dd__blk1057_rv = 0.0;
            (locals.var_idrift2__blk1062, locals.var_idrift2__blk1062_dn4, locals.var_idrift2__blk1062_dn6, locals.var_idrift2__blk1062_dn7, locals.var_idrift2__blk1062_dn8, locals.var_idrift2__blk1062_dn9, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_idrift2__blk1062_rv = 0.0;
        }
        if (locals.var_guard1080 != 0.0) {
            let assign41160_e47032: f64 = (locals.var_qim__blk1016 * locals.var_dxdrift__blk1017);
            let assign41160_e47034: f64 = (assign41160_e47032 + locals.var_idrift2__blk1062);
            let assign41160_e47036: f64 = (assign41160_e47034 + locals.var_qis__blk938);
            let assign41160_e47038: f64 = (assign41160_e47036 - locals.var_qid__blk1003);
            (locals.var_norm_ids__blk1063, locals.var_norm_ids__blk1063_dn4, locals.var_norm_ids__blk1063_dn6, locals.var_norm_ids__blk1063_dn7, locals.var_norm_ids__blk1063_dn8, locals.var_norm_ids__blk1063_dn9, ) = (assign41160_e47038, (((((locals.var_qim__blk1016_dn4 * locals.var_dxdrift__blk1017) + (locals.var_qim__blk1016 * locals.var_dxdrift__blk1017_dn4)) + locals.var_idrift2__blk1062_dn4) + locals.var_qis__blk938_dn4) - locals.var_qid__blk1003_dn4), (((((locals.var_qim__blk1016_dn6 * locals.var_dxdrift__blk1017) + (locals.var_qim__blk1016 * locals.var_dxdrift__blk1017_dn6)) + locals.var_idrift2__blk1062_dn6) + locals.var_qis__blk938_dn6) - locals.var_qid__blk1003_dn6), (((((locals.var_qim__blk1016_dn7 * locals.var_dxdrift__blk1017) + (locals.var_qim__blk1016 * locals.var_dxdrift__blk1017_dn7)) + locals.var_idrift2__blk1062_dn7) + locals.var_qis__blk938_dn7) - locals.var_qid__blk1003_dn7), (((((locals.var_qim__blk1016_dn8 * locals.var_dxdrift__blk1017) + (locals.var_qim__blk1016 * locals.var_dxdrift__blk1017_dn8)) + locals.var_idrift2__blk1062_dn8) + locals.var_qis__blk938_dn8) - locals.var_qid__blk1003_dn8), (((((locals.var_qim__blk1016_dn9 * locals.var_dxdrift__blk1017) + (locals.var_qim__blk1016 * locals.var_dxdrift__blk1017_dn9)) + locals.var_idrift2__blk1062_dn9) + locals.var_qis__blk938_dn9) - locals.var_qid__blk1003_dn9), );
            locals.var_norm_ids__blk1063_rv = 0.0;
        }
        let assign41170_e47043: f64 = if locals.var_qis__blk938 > 1e-6 { 1.0 } else { 0.0 };
        locals.var_guard1232 = assign41170_e47043;
        locals.var_guard1232_rv = 0.0;
        let assign41180_e47046: f64 = if locals.var_norm_ids__blk1063 > 1e-30 { 1.0 } else { 0.0 };
        locals.var_guard1233 = assign41180_e47046;
        locals.var_guard1233_rv = 0.0;
        if (((locals.var_guard1080 != 0.0) && (locals.var_guard1232 != 0.0)) && (locals.var_guard1233 != 0.0)) {
            let assign41190_e47055: f64 = (locals.var_aexp1s__blk943 / locals.var_qis__blk938);
            let assign41190_e47057: f64 = (assign41190_e47055 - locals.var_dqsqs_dxn_qi__blk950);
            let assign41190_e47058: f64 = (locals.var_a1s__blk947 / assign41190_e47057);
            (locals.var_q1s_chap__blk1064, locals.var_q1s_chap__blk1064_dn4, locals.var_q1s_chap__blk1064_dn6, locals.var_q1s_chap__blk1064_dn7, locals.var_q1s_chap__blk1064_dn8, locals.var_q1s_chap__blk1064_dn9, ) = (assign41190_e47058, (((locals.var_a1s__blk947_dn4 * assign41190_e47057) - (locals.var_a1s__blk947 * ((((locals.var_aexp1s__blk943_dn4 * locals.var_qis__blk938) - (locals.var_aexp1s__blk943 * locals.var_qis__blk938_dn4)) / (locals.var_qis__blk938 * locals.var_qis__blk938)) - locals.var_dqsqs_dxn_qi__blk950_dn4))) / (assign41190_e47057 * assign41190_e47057)), (((locals.var_a1s__blk947_dn6 * assign41190_e47057) - (locals.var_a1s__blk947 * ((((locals.var_aexp1s__blk943_dn6 * locals.var_qis__blk938) - (locals.var_aexp1s__blk943 * locals.var_qis__blk938_dn6)) / (locals.var_qis__blk938 * locals.var_qis__blk938)) - locals.var_dqsqs_dxn_qi__blk950_dn6))) / (assign41190_e47057 * assign41190_e47057)), (((locals.var_a1s__blk947_dn7 * assign41190_e47057) - (locals.var_a1s__blk947 * ((((locals.var_aexp1s__blk943_dn7 * locals.var_qis__blk938) - (locals.var_aexp1s__blk943 * locals.var_qis__blk938_dn7)) / (locals.var_qis__blk938 * locals.var_qis__blk938)) - locals.var_dqsqs_dxn_qi__blk950_dn7))) / (assign41190_e47057 * assign41190_e47057)), (((locals.var_a1s__blk947_dn8 * assign41190_e47057) - (locals.var_a1s__blk947 * ((((locals.var_aexp1s__blk943_dn8 * locals.var_qis__blk938) - (locals.var_aexp1s__blk943 * locals.var_qis__blk938_dn8)) / (locals.var_qis__blk938 * locals.var_qis__blk938)) - locals.var_dqsqs_dxn_qi__blk950_dn8))) / (assign41190_e47057 * assign41190_e47057)), (((locals.var_a1s__blk947_dn9 * assign41190_e47057) - (locals.var_a1s__blk947 * ((((locals.var_aexp1s__blk943_dn9 * locals.var_qis__blk938) - (locals.var_aexp1s__blk943 * locals.var_qis__blk938_dn9)) / (locals.var_qis__blk938 * locals.var_qis__blk938)) - locals.var_dqsqs_dxn_qi__blk950_dn9))) / (assign41190_e47057 * assign41190_e47057)), );
            locals.var_q1s_chap__blk1064_rv = 0.0;
        }
        if (((locals.var_guard1080 != 0.0) && (locals.var_guard1232 != 0.0)) && (locals.var_guard1233 != 0.0)) {
            let assign41200_e47069: f64 = (locals.var_aexp1d__blk1007 / locals.var_qid__blk1003);
            let assign41200_e47071: f64 = (assign41200_e47069 - locals.var_dqsqd_dxn_qi__blk1014);
            let assign41200_e47072: f64 = (locals.var_a1d__blk1011 / assign41200_e47071);
            (locals.var_q1d_chap__blk1065, locals.var_q1d_chap__blk1065_dn4, locals.var_q1d_chap__blk1065_dn6, locals.var_q1d_chap__blk1065_dn7, locals.var_q1d_chap__blk1065_dn8, locals.var_q1d_chap__blk1065_dn9, ) = (assign41200_e47072, (((locals.var_a1d__blk1011_dn4 * assign41200_e47071) - (locals.var_a1d__blk1011 * ((((locals.var_aexp1d__blk1007_dn4 * locals.var_qid__blk1003) - (locals.var_aexp1d__blk1007 * locals.var_qid__blk1003_dn4)) / (locals.var_qid__blk1003 * locals.var_qid__blk1003)) - locals.var_dqsqd_dxn_qi__blk1014_dn4))) / (assign41200_e47071 * assign41200_e47071)), (((locals.var_a1d__blk1011_dn6 * assign41200_e47071) - (locals.var_a1d__blk1011 * ((((locals.var_aexp1d__blk1007_dn6 * locals.var_qid__blk1003) - (locals.var_aexp1d__blk1007 * locals.var_qid__blk1003_dn6)) / (locals.var_qid__blk1003 * locals.var_qid__blk1003)) - locals.var_dqsqd_dxn_qi__blk1014_dn6))) / (assign41200_e47071 * assign41200_e47071)), (((locals.var_a1d__blk1011_dn7 * assign41200_e47071) - (locals.var_a1d__blk1011 * ((((locals.var_aexp1d__blk1007_dn7 * locals.var_qid__blk1003) - (locals.var_aexp1d__blk1007 * locals.var_qid__blk1003_dn7)) / (locals.var_qid__blk1003 * locals.var_qid__blk1003)) - locals.var_dqsqd_dxn_qi__blk1014_dn7))) / (assign41200_e47071 * assign41200_e47071)), (((locals.var_a1d__blk1011_dn8 * assign41200_e47071) - (locals.var_a1d__blk1011 * ((((locals.var_aexp1d__blk1007_dn8 * locals.var_qid__blk1003) - (locals.var_aexp1d__blk1007 * locals.var_qid__blk1003_dn8)) / (locals.var_qid__blk1003 * locals.var_qid__blk1003)) - locals.var_dqsqd_dxn_qi__blk1014_dn8))) / (assign41200_e47071 * assign41200_e47071)), (((locals.var_a1d__blk1011_dn9 * assign41200_e47071) - (locals.var_a1d__blk1011 * ((((locals.var_aexp1d__blk1007_dn9 * locals.var_qid__blk1003) - (locals.var_aexp1d__blk1007 * locals.var_qid__blk1003_dn9)) / (locals.var_qid__blk1003 * locals.var_qid__blk1003)) - locals.var_dqsqd_dxn_qi__blk1014_dn9))) / (assign41200_e47071 * assign41200_e47071)), );
            locals.var_q1d_chap__blk1065_rv = 0.0;
        }
        if (((locals.var_guard1080 != 0.0) && (locals.var_guard1232 != 0.0)) && (locals.var_guard1233 != 0.0)) {
            let assign41210_e47082: f64 = (locals.var_q1s_chap__blk1064 - locals.var_q1d_chap__blk1065);
            let assign41210_e47084: f64 = (assign41210_e47082 / locals.var_norm_ids__blk1063);
            (locals.var_inv_k1h1_0__blk1066, locals.var_inv_k1h1_0__blk1066_dn4, locals.var_inv_k1h1_0__blk1066_dn6, locals.var_inv_k1h1_0__blk1066_dn7, locals.var_inv_k1h1_0__blk1066_dn8, locals.var_inv_k1h1_0__blk1066_dn9, ) = (assign41210_e47084, ((((locals.var_q1s_chap__blk1064_dn4 - locals.var_q1d_chap__blk1065_dn4) * locals.var_norm_ids__blk1063) - (assign41210_e47082 * locals.var_norm_ids__blk1063_dn4)) / (locals.var_norm_ids__blk1063 * locals.var_norm_ids__blk1063)), ((((locals.var_q1s_chap__blk1064_dn6 - locals.var_q1d_chap__blk1065_dn6) * locals.var_norm_ids__blk1063) - (assign41210_e47082 * locals.var_norm_ids__blk1063_dn6)) / (locals.var_norm_ids__blk1063 * locals.var_norm_ids__blk1063)), ((((locals.var_q1s_chap__blk1064_dn7 - locals.var_q1d_chap__blk1065_dn7) * locals.var_norm_ids__blk1063) - (assign41210_e47082 * locals.var_norm_ids__blk1063_dn7)) / (locals.var_norm_ids__blk1063 * locals.var_norm_ids__blk1063)), ((((locals.var_q1s_chap__blk1064_dn8 - locals.var_q1d_chap__blk1065_dn8) * locals.var_norm_ids__blk1063) - (assign41210_e47082 * locals.var_norm_ids__blk1063_dn8)) / (locals.var_norm_ids__blk1063 * locals.var_norm_ids__blk1063)), ((((locals.var_q1s_chap__blk1064_dn9 - locals.var_q1d_chap__blk1065_dn9) * locals.var_norm_ids__blk1063) - (assign41210_e47082 * locals.var_norm_ids__blk1063_dn9)) / (locals.var_norm_ids__blk1063 * locals.var_norm_ids__blk1063)), );
            locals.var_inv_k1h1_0__blk1066_rv = 0.0;
        }
        if (((locals.var_guard1080 != 0.0) && (locals.var_guard1232 != 0.0)) && (locals.var_guard1233 != 0.0)) {
            let assign41220_e47095: f64 = (locals.var_aexp2s__blk944 / locals.var_qis__blk938);
            let assign41220_e47097: f64 = (assign41220_e47095 - locals.var_dqsqs_dxn_qi__blk950);
            let assign41220_e47098: f64 = (locals.var_a2s__blk948 / assign41220_e47097);
            (locals.var_q2s_chap__blk1067, locals.var_q2s_chap__blk1067_dn4, locals.var_q2s_chap__blk1067_dn6, locals.var_q2s_chap__blk1067_dn7, locals.var_q2s_chap__blk1067_dn8, locals.var_q2s_chap__blk1067_dn9, ) = (assign41220_e47098, (((locals.var_a2s__blk948_dn4 * assign41220_e47097) - (locals.var_a2s__blk948 * ((((locals.var_aexp2s__blk944_dn4 * locals.var_qis__blk938) - (locals.var_aexp2s__blk944 * locals.var_qis__blk938_dn4)) / (locals.var_qis__blk938 * locals.var_qis__blk938)) - locals.var_dqsqs_dxn_qi__blk950_dn4))) / (assign41220_e47097 * assign41220_e47097)), (((locals.var_a2s__blk948_dn6 * assign41220_e47097) - (locals.var_a2s__blk948 * ((((locals.var_aexp2s__blk944_dn6 * locals.var_qis__blk938) - (locals.var_aexp2s__blk944 * locals.var_qis__blk938_dn6)) / (locals.var_qis__blk938 * locals.var_qis__blk938)) - locals.var_dqsqs_dxn_qi__blk950_dn6))) / (assign41220_e47097 * assign41220_e47097)), (((locals.var_a2s__blk948_dn7 * assign41220_e47097) - (locals.var_a2s__blk948 * ((((locals.var_aexp2s__blk944_dn7 * locals.var_qis__blk938) - (locals.var_aexp2s__blk944 * locals.var_qis__blk938_dn7)) / (locals.var_qis__blk938 * locals.var_qis__blk938)) - locals.var_dqsqs_dxn_qi__blk950_dn7))) / (assign41220_e47097 * assign41220_e47097)), (((locals.var_a2s__blk948_dn8 * assign41220_e47097) - (locals.var_a2s__blk948 * ((((locals.var_aexp2s__blk944_dn8 * locals.var_qis__blk938) - (locals.var_aexp2s__blk944 * locals.var_qis__blk938_dn8)) / (locals.var_qis__blk938 * locals.var_qis__blk938)) - locals.var_dqsqs_dxn_qi__blk950_dn8))) / (assign41220_e47097 * assign41220_e47097)), (((locals.var_a2s__blk948_dn9 * assign41220_e47097) - (locals.var_a2s__blk948 * ((((locals.var_aexp2s__blk944_dn9 * locals.var_qis__blk938) - (locals.var_aexp2s__blk944 * locals.var_qis__blk938_dn9)) / (locals.var_qis__blk938 * locals.var_qis__blk938)) - locals.var_dqsqs_dxn_qi__blk950_dn9))) / (assign41220_e47097 * assign41220_e47097)), );
            locals.var_q2s_chap__blk1067_rv = 0.0;
        }
        if (((locals.var_guard1080 != 0.0) && (locals.var_guard1232 != 0.0)) && (locals.var_guard1233 != 0.0)) {
            let assign41230_e47109: f64 = (locals.var_aexp2d__blk1008 / locals.var_qid__blk1003);
            let assign41230_e47111: f64 = (assign41230_e47109 - locals.var_dqsqd_dxn_qi__blk1014);
            let assign41230_e47112: f64 = (locals.var_a2d__blk1012 / assign41230_e47111);
            (locals.var_q2d_chap__blk1068, locals.var_q2d_chap__blk1068_dn4, locals.var_q2d_chap__blk1068_dn6, locals.var_q2d_chap__blk1068_dn7, locals.var_q2d_chap__blk1068_dn8, locals.var_q2d_chap__blk1068_dn9, ) = (assign41230_e47112, (((locals.var_a2d__blk1012_dn4 * assign41230_e47111) - (locals.var_a2d__blk1012 * ((((locals.var_aexp2d__blk1008_dn4 * locals.var_qid__blk1003) - (locals.var_aexp2d__blk1008 * locals.var_qid__blk1003_dn4)) / (locals.var_qid__blk1003 * locals.var_qid__blk1003)) - locals.var_dqsqd_dxn_qi__blk1014_dn4))) / (assign41230_e47111 * assign41230_e47111)), (((locals.var_a2d__blk1012_dn6 * assign41230_e47111) - (locals.var_a2d__blk1012 * ((((locals.var_aexp2d__blk1008_dn6 * locals.var_qid__blk1003) - (locals.var_aexp2d__blk1008 * locals.var_qid__blk1003_dn6)) / (locals.var_qid__blk1003 * locals.var_qid__blk1003)) - locals.var_dqsqd_dxn_qi__blk1014_dn6))) / (assign41230_e47111 * assign41230_e47111)), (((locals.var_a2d__blk1012_dn7 * assign41230_e47111) - (locals.var_a2d__blk1012 * ((((locals.var_aexp2d__blk1008_dn7 * locals.var_qid__blk1003) - (locals.var_aexp2d__blk1008 * locals.var_qid__blk1003_dn7)) / (locals.var_qid__blk1003 * locals.var_qid__blk1003)) - locals.var_dqsqd_dxn_qi__blk1014_dn7))) / (assign41230_e47111 * assign41230_e47111)), (((locals.var_a2d__blk1012_dn8 * assign41230_e47111) - (locals.var_a2d__blk1012 * ((((locals.var_aexp2d__blk1008_dn8 * locals.var_qid__blk1003) - (locals.var_aexp2d__blk1008 * locals.var_qid__blk1003_dn8)) / (locals.var_qid__blk1003 * locals.var_qid__blk1003)) - locals.var_dqsqd_dxn_qi__blk1014_dn8))) / (assign41230_e47111 * assign41230_e47111)), (((locals.var_a2d__blk1012_dn9 * assign41230_e47111) - (locals.var_a2d__blk1012 * ((((locals.var_aexp2d__blk1008_dn9 * locals.var_qid__blk1003) - (locals.var_aexp2d__blk1008 * locals.var_qid__blk1003_dn9)) / (locals.var_qid__blk1003 * locals.var_qid__blk1003)) - locals.var_dqsqd_dxn_qi__blk1014_dn9))) / (assign41230_e47111 * assign41230_e47111)), );
            locals.var_q2d_chap__blk1068_rv = 0.0;
        }
        if (((locals.var_guard1080 != 0.0) && (locals.var_guard1232 != 0.0)) && (locals.var_guard1233 != 0.0)) {
            let assign41240_e47122: f64 = (locals.var_q2s_chap__blk1067 - locals.var_q2d_chap__blk1068);
            let assign41240_e47124: f64 = (assign41240_e47122 / locals.var_norm_ids__blk1063);
            (locals.var_inv_k2h2_0__blk1069, locals.var_inv_k2h2_0__blk1069_dn4, locals.var_inv_k2h2_0__blk1069_dn6, locals.var_inv_k2h2_0__blk1069_dn7, locals.var_inv_k2h2_0__blk1069_dn8, locals.var_inv_k2h2_0__blk1069_dn9, ) = (assign41240_e47124, ((((locals.var_q2s_chap__blk1067_dn4 - locals.var_q2d_chap__blk1068_dn4) * locals.var_norm_ids__blk1063) - (assign41240_e47122 * locals.var_norm_ids__blk1063_dn4)) / (locals.var_norm_ids__blk1063 * locals.var_norm_ids__blk1063)), ((((locals.var_q2s_chap__blk1067_dn6 - locals.var_q2d_chap__blk1068_dn6) * locals.var_norm_ids__blk1063) - (assign41240_e47122 * locals.var_norm_ids__blk1063_dn6)) / (locals.var_norm_ids__blk1063 * locals.var_norm_ids__blk1063)), ((((locals.var_q2s_chap__blk1067_dn7 - locals.var_q2d_chap__blk1068_dn7) * locals.var_norm_ids__blk1063) - (assign41240_e47122 * locals.var_norm_ids__blk1063_dn7)) / (locals.var_norm_ids__blk1063 * locals.var_norm_ids__blk1063)), ((((locals.var_q2s_chap__blk1067_dn8 - locals.var_q2d_chap__blk1068_dn8) * locals.var_norm_ids__blk1063) - (assign41240_e47122 * locals.var_norm_ids__blk1063_dn8)) / (locals.var_norm_ids__blk1063 * locals.var_norm_ids__blk1063)), ((((locals.var_q2s_chap__blk1067_dn9 - locals.var_q2d_chap__blk1068_dn9) * locals.var_norm_ids__blk1063) - (assign41240_e47122 * locals.var_norm_ids__blk1063_dn9)) / (locals.var_norm_ids__blk1063 * locals.var_norm_ids__blk1063)), );
            locals.var_inv_k2h2_0__blk1069_rv = 0.0;
        }
        if (((locals.var_guard1080 != 0.0) && (locals.var_guard1232 != 0.0)) && (locals.var_guard1233 == 0.0)) {
            (locals.var_inv_k1h1_0__blk1066, locals.var_inv_k1h1_0__blk1066_dn4, locals.var_inv_k1h1_0__blk1066_dn6, locals.var_inv_k1h1_0__blk1066_dn7, locals.var_inv_k1h1_0__blk1066_dn8, locals.var_inv_k1h1_0__blk1066_dn9, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_inv_k1h1_0__blk1066_rv = 0.0;
            (locals.var_inv_k2h2_0__blk1069, locals.var_inv_k2h2_0__blk1069_dn4, locals.var_inv_k2h2_0__blk1069_dn6, locals.var_inv_k2h2_0__blk1069_dn7, locals.var_inv_k2h2_0__blk1069_dn8, locals.var_inv_k2h2_0__blk1069_dn9, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_inv_k2h2_0__blk1069_rv = 0.0;
        }
        if ((locals.var_guard1080 != 0.0) && (locals.var_guard1232 == 0.0)) {
            let assign41270_e47150: f64 = (-2.0);
            let assign41270_e47152: f64 = (assign41270_e47150 * locals.var_s1__blk969);
            let assign41270_e47155: f64 = (locals.var_inv_k1__blk906 / locals.var_q1chapinf__blk972);
            let assign41270_e47157: f64 = (assign41270_e47155 + locals.var_inv_dinf__blk975);
            let assign41270_e47158: f64 = (assign41270_e47152 * assign41270_e47157);
            (locals.var_zeta1__blk1070, locals.var_zeta1__blk1070_dn4, locals.var_zeta1__blk1070_dn6, locals.var_zeta1__blk1070_dn7, locals.var_zeta1__blk1070_dn8, locals.var_zeta1__blk1070_dn9, ) = (assign41270_e47158, (((assign41270_e47150 * locals.var_s1__blk969_dn4) * assign41270_e47157) + (assign41270_e47152 * ((((locals.var_inv_k1__blk906_dn4 * locals.var_q1chapinf__blk972) - (locals.var_inv_k1__blk906 * locals.var_q1chapinf__blk972_dn4)) / (locals.var_q1chapinf__blk972 * locals.var_q1chapinf__blk972)) + locals.var_inv_dinf__blk975_dn4))), (((assign41270_e47150 * locals.var_s1__blk969_dn6) * assign41270_e47157) + (assign41270_e47152 * ((((locals.var_inv_k1__blk906_dn6 * locals.var_q1chapinf__blk972) - (locals.var_inv_k1__blk906 * locals.var_q1chapinf__blk972_dn6)) / (locals.var_q1chapinf__blk972 * locals.var_q1chapinf__blk972)) + locals.var_inv_dinf__blk975_dn6))), (((assign41270_e47150 * locals.var_s1__blk969_dn7) * assign41270_e47157) + (assign41270_e47152 * ((((locals.var_inv_k1__blk906_dn7 * locals.var_q1chapinf__blk972) - (locals.var_inv_k1__blk906 * locals.var_q1chapinf__blk972_dn7)) / (locals.var_q1chapinf__blk972 * locals.var_q1chapinf__blk972)) + locals.var_inv_dinf__blk975_dn7))), (((assign41270_e47150 * locals.var_s1__blk969_dn8) * assign41270_e47157) + (assign41270_e47152 * ((((locals.var_inv_k1__blk906_dn8 * locals.var_q1chapinf__blk972) - (locals.var_inv_k1__blk906 * locals.var_q1chapinf__blk972_dn8)) / (locals.var_q1chapinf__blk972 * locals.var_q1chapinf__blk972)) + locals.var_inv_dinf__blk975_dn8))), (((assign41270_e47150 * locals.var_s1__blk969_dn9) * assign41270_e47157) + (assign41270_e47152 * ((((locals.var_inv_k1__blk906_dn9 * locals.var_q1chapinf__blk972) - (locals.var_inv_k1__blk906 * locals.var_q1chapinf__blk972_dn9)) / (locals.var_q1chapinf__blk972 * locals.var_q1chapinf__blk972)) + locals.var_inv_dinf__blk975_dn9))), );
            locals.var_zeta1__blk1070_rv = 0.0;
        }
        if ((locals.var_guard1080 != 0.0) && (locals.var_guard1232 == 0.0)) {
            let assign41280_e47166: f64 = (-2.0);
            let assign41280_e47168: f64 = (assign41280_e47166 * locals.var_s2__blk970);
            let assign41280_e47171: f64 = (locals.var_inv_k2__blk907 / locals.var_q2chapinf__blk973);
            let assign41280_e47173: f64 = (assign41280_e47171 + locals.var_inv_dinf__blk975);
            let assign41280_e47174: f64 = (assign41280_e47168 * assign41280_e47173);
            (locals.var_zeta2__blk1071, locals.var_zeta2__blk1071_dn4, locals.var_zeta2__blk1071_dn6, locals.var_zeta2__blk1071_dn7, locals.var_zeta2__blk1071_dn8, locals.var_zeta2__blk1071_dn9, ) = (assign41280_e47174, (((assign41280_e47166 * locals.var_s2__blk970_dn4) * assign41280_e47173) + (assign41280_e47168 * ((((locals.var_inv_k2__blk907_dn4 * locals.var_q2chapinf__blk973) - (locals.var_inv_k2__blk907 * locals.var_q2chapinf__blk973_dn4)) / (locals.var_q2chapinf__blk973 * locals.var_q2chapinf__blk973)) + locals.var_inv_dinf__blk975_dn4))), (((assign41280_e47166 * locals.var_s2__blk970_dn6) * assign41280_e47173) + (assign41280_e47168 * ((((locals.var_inv_k2__blk907_dn6 * locals.var_q2chapinf__blk973) - (locals.var_inv_k2__blk907 * locals.var_q2chapinf__blk973_dn6)) / (locals.var_q2chapinf__blk973 * locals.var_q2chapinf__blk973)) + locals.var_inv_dinf__blk975_dn6))), (((assign41280_e47166 * locals.var_s2__blk970_dn7) * assign41280_e47173) + (assign41280_e47168 * ((((locals.var_inv_k2__blk907_dn7 * locals.var_q2chapinf__blk973) - (locals.var_inv_k2__blk907 * locals.var_q2chapinf__blk973_dn7)) / (locals.var_q2chapinf__blk973 * locals.var_q2chapinf__blk973)) + locals.var_inv_dinf__blk975_dn7))), (((assign41280_e47166 * locals.var_s2__blk970_dn8) * assign41280_e47173) + (assign41280_e47168 * ((((locals.var_inv_k2__blk907_dn8 * locals.var_q2chapinf__blk973) - (locals.var_inv_k2__blk907 * locals.var_q2chapinf__blk973_dn8)) / (locals.var_q2chapinf__blk973 * locals.var_q2chapinf__blk973)) + locals.var_inv_dinf__blk975_dn8))), (((assign41280_e47166 * locals.var_s2__blk970_dn9) * assign41280_e47173) + (assign41280_e47168 * ((((locals.var_inv_k2__blk907_dn9 * locals.var_q2chapinf__blk973) - (locals.var_inv_k2__blk907 * locals.var_q2chapinf__blk973_dn9)) / (locals.var_q2chapinf__blk973 * locals.var_q2chapinf__blk973)) + locals.var_inv_dinf__blk975_dn9))), );
            locals.var_zeta2__blk1071_rv = 0.0;
        }
        if ((locals.var_guard1080 != 0.0) && (locals.var_guard1232 == 0.0)) {
            let assign41290_e47183: f64 = (locals.var_zeta2__blk1071 - locals.var_zeta1__blk1070);
            let assign41290_e47185: f64 = (assign41290_e47183 * locals.var_inv_dinf__blk975);
            (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign41290_e47185, (((locals.var_zeta2__blk1071_dn4 - locals.var_zeta1__blk1070_dn4) * locals.var_inv_dinf__blk975) + (assign41290_e47183 * locals.var_inv_dinf__blk975_dn4)), (((locals.var_zeta2__blk1071_dn6 - locals.var_zeta1__blk1070_dn6) * locals.var_inv_dinf__blk975) + (assign41290_e47183 * locals.var_inv_dinf__blk975_dn6)), (((locals.var_zeta2__blk1071_dn7 - locals.var_zeta1__blk1070_dn7) * locals.var_inv_dinf__blk975) + (assign41290_e47183 * locals.var_inv_dinf__blk975_dn7)), (((locals.var_zeta2__blk1071_dn8 - locals.var_zeta1__blk1070_dn8) * locals.var_inv_dinf__blk975) + (assign41290_e47183 * locals.var_inv_dinf__blk975_dn8)), (((locals.var_zeta2__blk1071_dn9 - locals.var_zeta1__blk1070_dn9) * locals.var_inv_dinf__blk975) + (assign41290_e47183 * locals.var_inv_dinf__blk975_dn9)), );
            locals.var_temp_rv = 0.0;
        }
        if ((locals.var_guard1080 != 0.0) && (locals.var_guard1232 == 0.0)) {
            let assign41300_e47194: f64 = (locals.var_zeta1__blk1070 * locals.var_inv_k1__blk906);
            (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9, ) = (assign41300_e47194, ((locals.var_zeta1__blk1070_dn4 * locals.var_inv_k1__blk906) + (locals.var_zeta1__blk1070 * locals.var_inv_k1__blk906_dn4)), ((locals.var_zeta1__blk1070_dn6 * locals.var_inv_k1__blk906) + (locals.var_zeta1__blk1070 * locals.var_inv_k1__blk906_dn6)), ((locals.var_zeta1__blk1070_dn7 * locals.var_inv_k1__blk906) + (locals.var_zeta1__blk1070 * locals.var_inv_k1__blk906_dn7)), ((locals.var_zeta1__blk1070_dn8 * locals.var_inv_k1__blk906) + (locals.var_zeta1__blk1070 * locals.var_inv_k1__blk906_dn8)), ((locals.var_zeta1__blk1070_dn9 * locals.var_inv_k1__blk906) + (locals.var_zeta1__blk1070 * locals.var_inv_k1__blk906_dn9)), );
            locals.var_temp1_rv = 0.0;
        }
        if ((locals.var_guard1080 != 0.0) && (locals.var_guard1232 == 0.0)) {
            let assign41310_e47203: f64 = (locals.var_zeta2__blk1071 * locals.var_inv_k2__blk907);
            (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9, ) = (assign41310_e47203, ((locals.var_zeta2__blk1071_dn4 * locals.var_inv_k2__blk907) + (locals.var_zeta2__blk1071 * locals.var_inv_k2__blk907_dn4)), ((locals.var_zeta2__blk1071_dn6 * locals.var_inv_k2__blk907) + (locals.var_zeta2__blk1071 * locals.var_inv_k2__blk907_dn6)), ((locals.var_zeta2__blk1071_dn7 * locals.var_inv_k2__blk907) + (locals.var_zeta2__blk1071 * locals.var_inv_k2__blk907_dn7)), ((locals.var_zeta2__blk1071_dn8 * locals.var_inv_k2__blk907) + (locals.var_zeta2__blk1071 * locals.var_inv_k2__blk907_dn8)), ((locals.var_zeta2__blk1071_dn9 * locals.var_inv_k2__blk907) + (locals.var_zeta2__blk1071 * locals.var_inv_k2__blk907_dn9)), );
            locals.var_temp2_rv = 0.0;
        }
        if ((locals.var_guard1080 != 0.0) && (locals.var_guard1232 == 0.0)) {
            let assign41320_e47212: f64 = (locals.var_temp1 + locals.var_temp2);
            (locals.var_temp3, locals.var_temp3_dn4, locals.var_temp3_dn6, locals.var_temp3_dn7, locals.var_temp3_dn8, locals.var_temp3_dn9, ) = (assign41320_e47212, (locals.var_temp1_dn4 + locals.var_temp2_dn4), (locals.var_temp1_dn6 + locals.var_temp2_dn6), (locals.var_temp1_dn7 + locals.var_temp2_dn7), (locals.var_temp1_dn8 + locals.var_temp2_dn8), (locals.var_temp1_dn9 + locals.var_temp2_dn9), );
            locals.var_temp3_rv = 0.0;
        }
        if ((locals.var_guard1080 != 0.0) && (locals.var_guard1232 == 0.0)) {
            let assign41330_e47223: f64 = (locals.var_s1__blk969 * locals.var_inv_k1__blk906);
            let assign41330_e47226: f64 = (locals.var_s2__blk970 * locals.var_inv_k2__blk907);
            let assign41330_e47227: f64 = (assign41330_e47223 + assign41330_e47226);
            let assign41330_e47228: f64 = (2.0 * assign41330_e47227);
            let assign41330_e47229: f64 = (3.0 + assign41330_e47228);
            (locals.var_temp4, locals.var_temp4_dn4, locals.var_temp4_dn6, locals.var_temp4_dn7, locals.var_temp4_dn8, locals.var_temp4_dn9, ) = (assign41330_e47229, (2.0 * (((locals.var_s1__blk969_dn4 * locals.var_inv_k1__blk906) + (locals.var_s1__blk969 * locals.var_inv_k1__blk906_dn4)) + ((locals.var_s2__blk970_dn4 * locals.var_inv_k2__blk907) + (locals.var_s2__blk970 * locals.var_inv_k2__blk907_dn4)))), (2.0 * (((locals.var_s1__blk969_dn6 * locals.var_inv_k1__blk906) + (locals.var_s1__blk969 * locals.var_inv_k1__blk906_dn6)) + ((locals.var_s2__blk970_dn6 * locals.var_inv_k2__blk907) + (locals.var_s2__blk970 * locals.var_inv_k2__blk907_dn6)))), (2.0 * (((locals.var_s1__blk969_dn7 * locals.var_inv_k1__blk906) + (locals.var_s1__blk969 * locals.var_inv_k1__blk906_dn7)) + ((locals.var_s2__blk970_dn7 * locals.var_inv_k2__blk907) + (locals.var_s2__blk970 * locals.var_inv_k2__blk907_dn7)))), (2.0 * (((locals.var_s1__blk969_dn8 * locals.var_inv_k1__blk906) + (locals.var_s1__blk969 * locals.var_inv_k1__blk906_dn8)) + ((locals.var_s2__blk970_dn8 * locals.var_inv_k2__blk907) + (locals.var_s2__blk970 * locals.var_inv_k2__blk907_dn8)))), (2.0 * (((locals.var_s1__blk969_dn9 * locals.var_inv_k1__blk906) + (locals.var_s1__blk969 * locals.var_inv_k1__blk906_dn9)) + ((locals.var_s2__blk970_dn9 * locals.var_inv_k2__blk907) + (locals.var_s2__blk970 * locals.var_inv_k2__blk907_dn9)))), );
            locals.var_temp4_rv = 0.0;
        }
        if ((locals.var_guard1080 != 0.0) && (locals.var_guard1232 == 0.0)) {
            let assign41340_e47238: f64 = (locals.var_temp2 + locals.var_temp);
            let assign41340_e47241: f64 = (locals.var_temp3 / locals.var_q1chapinf__blk972);
            let assign41340_e47242: f64 = (assign41340_e47238 - assign41340_e47241);
            let assign41340_e47244: f64 = (assign41340_e47242 / locals.var_temp4);
            (locals.var_ksi1__blk1072, locals.var_ksi1__blk1072_dn4, locals.var_ksi1__blk1072_dn6, locals.var_ksi1__blk1072_dn7, locals.var_ksi1__blk1072_dn8, locals.var_ksi1__blk1072_dn9, ) = (assign41340_e47244, (((((locals.var_temp2_dn4 + locals.var_temp_dn4) - (((locals.var_temp3_dn4 * locals.var_q1chapinf__blk972) - (locals.var_temp3 * locals.var_q1chapinf__blk972_dn4)) / (locals.var_q1chapinf__blk972 * locals.var_q1chapinf__blk972))) * locals.var_temp4) - (assign41340_e47242 * locals.var_temp4_dn4)) / (locals.var_temp4 * locals.var_temp4)), (((((locals.var_temp2_dn6 + locals.var_temp_dn6) - (((locals.var_temp3_dn6 * locals.var_q1chapinf__blk972) - (locals.var_temp3 * locals.var_q1chapinf__blk972_dn6)) / (locals.var_q1chapinf__blk972 * locals.var_q1chapinf__blk972))) * locals.var_temp4) - (assign41340_e47242 * locals.var_temp4_dn6)) / (locals.var_temp4 * locals.var_temp4)), (((((locals.var_temp2_dn7 + locals.var_temp_dn7) - (((locals.var_temp3_dn7 * locals.var_q1chapinf__blk972) - (locals.var_temp3 * locals.var_q1chapinf__blk972_dn7)) / (locals.var_q1chapinf__blk972 * locals.var_q1chapinf__blk972))) * locals.var_temp4) - (assign41340_e47242 * locals.var_temp4_dn7)) / (locals.var_temp4 * locals.var_temp4)), (((((locals.var_temp2_dn8 + locals.var_temp_dn8) - (((locals.var_temp3_dn8 * locals.var_q1chapinf__blk972) - (locals.var_temp3 * locals.var_q1chapinf__blk972_dn8)) / (locals.var_q1chapinf__blk972 * locals.var_q1chapinf__blk972))) * locals.var_temp4) - (assign41340_e47242 * locals.var_temp4_dn8)) / (locals.var_temp4 * locals.var_temp4)), (((((locals.var_temp2_dn9 + locals.var_temp_dn9) - (((locals.var_temp3_dn9 * locals.var_q1chapinf__blk972) - (locals.var_temp3 * locals.var_q1chapinf__blk972_dn9)) / (locals.var_q1chapinf__blk972 * locals.var_q1chapinf__blk972))) * locals.var_temp4) - (assign41340_e47242 * locals.var_temp4_dn9)) / (locals.var_temp4 * locals.var_temp4)), );
            locals.var_ksi1__blk1072_rv = 0.0;
        }
        if ((locals.var_guard1080 != 0.0) && (locals.var_guard1232 == 0.0)) {
            let assign41350_e47253: f64 = (locals.var_temp1 - locals.var_temp);
            let assign41350_e47256: f64 = (locals.var_temp3 / locals.var_q2chapinf__blk973);
            let assign41350_e47257: f64 = (assign41350_e47253 - assign41350_e47256);
            let assign41350_e47259: f64 = (assign41350_e47257 / locals.var_temp4);
            (locals.var_ksi2__blk1073, locals.var_ksi2__blk1073_dn4, locals.var_ksi2__blk1073_dn6, locals.var_ksi2__blk1073_dn7, locals.var_ksi2__blk1073_dn8, locals.var_ksi2__blk1073_dn9, ) = (assign41350_e47259, (((((locals.var_temp1_dn4 - locals.var_temp_dn4) - (((locals.var_temp3_dn4 * locals.var_q2chapinf__blk973) - (locals.var_temp3 * locals.var_q2chapinf__blk973_dn4)) / (locals.var_q2chapinf__blk973 * locals.var_q2chapinf__blk973))) * locals.var_temp4) - (assign41350_e47257 * locals.var_temp4_dn4)) / (locals.var_temp4 * locals.var_temp4)), (((((locals.var_temp1_dn6 - locals.var_temp_dn6) - (((locals.var_temp3_dn6 * locals.var_q2chapinf__blk973) - (locals.var_temp3 * locals.var_q2chapinf__blk973_dn6)) / (locals.var_q2chapinf__blk973 * locals.var_q2chapinf__blk973))) * locals.var_temp4) - (assign41350_e47257 * locals.var_temp4_dn6)) / (locals.var_temp4 * locals.var_temp4)), (((((locals.var_temp1_dn7 - locals.var_temp_dn7) - (((locals.var_temp3_dn7 * locals.var_q2chapinf__blk973) - (locals.var_temp3 * locals.var_q2chapinf__blk973_dn7)) / (locals.var_q2chapinf__blk973 * locals.var_q2chapinf__blk973))) * locals.var_temp4) - (assign41350_e47257 * locals.var_temp4_dn7)) / (locals.var_temp4 * locals.var_temp4)), (((((locals.var_temp1_dn8 - locals.var_temp_dn8) - (((locals.var_temp3_dn8 * locals.var_q2chapinf__blk973) - (locals.var_temp3 * locals.var_q2chapinf__blk973_dn8)) / (locals.var_q2chapinf__blk973 * locals.var_q2chapinf__blk973))) * locals.var_temp4) - (assign41350_e47257 * locals.var_temp4_dn8)) / (locals.var_temp4 * locals.var_temp4)), (((((locals.var_temp1_dn9 - locals.var_temp_dn9) - (((locals.var_temp3_dn9 * locals.var_q2chapinf__blk973) - (locals.var_temp3 * locals.var_q2chapinf__blk973_dn9)) / (locals.var_q2chapinf__blk973 * locals.var_q2chapinf__blk973))) * locals.var_temp4) - (assign41350_e47257 * locals.var_temp4_dn9)) / (locals.var_temp4 * locals.var_temp4)), );
            locals.var_ksi2__blk1073_rv = 0.0;
        }
        if ((locals.var_guard1080 != 0.0) && (locals.var_guard1232 == 0.0)) {
            let assign41360_e47267: f64 = (-locals.var_q1chapinf__blk972);
            let assign41360_e47270: f64 = (locals.var_ksi1__blk1072 * locals.var_q1chapinf__blk972);
            let assign41360_e47272: f64 = (assign41360_e47270 + locals.var_inv_dinf__blk975);
            let assign41360_e47273: f64 = (assign41360_e47267 * assign41360_e47272);
            (locals.var_inv_k1h1_0__blk1066, locals.var_inv_k1h1_0__blk1066_dn4, locals.var_inv_k1h1_0__blk1066_dn6, locals.var_inv_k1h1_0__blk1066_dn7, locals.var_inv_k1h1_0__blk1066_dn8, locals.var_inv_k1h1_0__blk1066_dn9, ) = (assign41360_e47273, (((-locals.var_q1chapinf__blk972_dn4) * assign41360_e47272) + (assign41360_e47267 * (((locals.var_ksi1__blk1072_dn4 * locals.var_q1chapinf__blk972) + (locals.var_ksi1__blk1072 * locals.var_q1chapinf__blk972_dn4)) + locals.var_inv_dinf__blk975_dn4))), (((-locals.var_q1chapinf__blk972_dn6) * assign41360_e47272) + (assign41360_e47267 * (((locals.var_ksi1__blk1072_dn6 * locals.var_q1chapinf__blk972) + (locals.var_ksi1__blk1072 * locals.var_q1chapinf__blk972_dn6)) + locals.var_inv_dinf__blk975_dn6))), (((-locals.var_q1chapinf__blk972_dn7) * assign41360_e47272) + (assign41360_e47267 * (((locals.var_ksi1__blk1072_dn7 * locals.var_q1chapinf__blk972) + (locals.var_ksi1__blk1072 * locals.var_q1chapinf__blk972_dn7)) + locals.var_inv_dinf__blk975_dn7))), (((-locals.var_q1chapinf__blk972_dn8) * assign41360_e47272) + (assign41360_e47267 * (((locals.var_ksi1__blk1072_dn8 * locals.var_q1chapinf__blk972) + (locals.var_ksi1__blk1072 * locals.var_q1chapinf__blk972_dn8)) + locals.var_inv_dinf__blk975_dn8))), (((-locals.var_q1chapinf__blk972_dn9) * assign41360_e47272) + (assign41360_e47267 * (((locals.var_ksi1__blk1072_dn9 * locals.var_q1chapinf__blk972) + (locals.var_ksi1__blk1072 * locals.var_q1chapinf__blk972_dn9)) + locals.var_inv_dinf__blk975_dn9))), );
            locals.var_inv_k1h1_0__blk1066_rv = 0.0;
        }
        if ((locals.var_guard1080 != 0.0) && (locals.var_guard1232 == 0.0)) {
            let assign41370_e47281: f64 = (-locals.var_q2chapinf__blk973);
            let assign41370_e47284: f64 = (locals.var_ksi2__blk1073 * locals.var_q2chapinf__blk973);
            let assign41370_e47286: f64 = (assign41370_e47284 + locals.var_inv_dinf__blk975);
            let assign41370_e47287: f64 = (assign41370_e47281 * assign41370_e47286);
            (locals.var_inv_k2h2_0__blk1069, locals.var_inv_k2h2_0__blk1069_dn4, locals.var_inv_k2h2_0__blk1069_dn6, locals.var_inv_k2h2_0__blk1069_dn7, locals.var_inv_k2h2_0__blk1069_dn8, locals.var_inv_k2h2_0__blk1069_dn9, ) = (assign41370_e47287, (((-locals.var_q2chapinf__blk973_dn4) * assign41370_e47286) + (assign41370_e47281 * (((locals.var_ksi2__blk1073_dn4 * locals.var_q2chapinf__blk973) + (locals.var_ksi2__blk1073 * locals.var_q2chapinf__blk973_dn4)) + locals.var_inv_dinf__blk975_dn4))), (((-locals.var_q2chapinf__blk973_dn6) * assign41370_e47286) + (assign41370_e47281 * (((locals.var_ksi2__blk1073_dn6 * locals.var_q2chapinf__blk973) + (locals.var_ksi2__blk1073 * locals.var_q2chapinf__blk973_dn6)) + locals.var_inv_dinf__blk975_dn6))), (((-locals.var_q2chapinf__blk973_dn7) * assign41370_e47286) + (assign41370_e47281 * (((locals.var_ksi2__blk1073_dn7 * locals.var_q2chapinf__blk973) + (locals.var_ksi2__blk1073 * locals.var_q2chapinf__blk973_dn7)) + locals.var_inv_dinf__blk975_dn7))), (((-locals.var_q2chapinf__blk973_dn8) * assign41370_e47286) + (assign41370_e47281 * (((locals.var_ksi2__blk1073_dn8 * locals.var_q2chapinf__blk973) + (locals.var_ksi2__blk1073 * locals.var_q2chapinf__blk973_dn8)) + locals.var_inv_dinf__blk975_dn8))), (((-locals.var_q2chapinf__blk973_dn9) * assign41370_e47286) + (assign41370_e47281 * (((locals.var_ksi2__blk1073_dn9 * locals.var_q2chapinf__blk973) + (locals.var_ksi2__blk1073 * locals.var_q2chapinf__blk973_dn9)) + locals.var_inv_dinf__blk975_dn9))), );
            locals.var_inv_k2h2_0__blk1069_rv = 0.0;
        }
        if (locals.var_guard1080 != 0.0) {
            let assign41380_e47293: f64 = (locals.var_inv_k1h1_0__blk1066 * locals.var_hsat__blk1053);
            (locals.var_inv_k1h1__blk1074, locals.var_inv_k1h1__blk1074_dn4, locals.var_inv_k1h1__blk1074_dn6, locals.var_inv_k1h1__blk1074_dn7, locals.var_inv_k1h1__blk1074_dn8, locals.var_inv_k1h1__blk1074_dn9, ) = (assign41380_e47293, ((locals.var_inv_k1h1_0__blk1066_dn4 * locals.var_hsat__blk1053) + (locals.var_inv_k1h1_0__blk1066 * locals.var_hsat__blk1053_dn4)), ((locals.var_inv_k1h1_0__blk1066_dn6 * locals.var_hsat__blk1053) + (locals.var_inv_k1h1_0__blk1066 * locals.var_hsat__blk1053_dn6)), ((locals.var_inv_k1h1_0__blk1066_dn7 * locals.var_hsat__blk1053) + (locals.var_inv_k1h1_0__blk1066 * locals.var_hsat__blk1053_dn7)), ((locals.var_inv_k1h1_0__blk1066_dn8 * locals.var_hsat__blk1053) + (locals.var_inv_k1h1_0__blk1066 * locals.var_hsat__blk1053_dn8)), ((locals.var_inv_k1h1_0__blk1066_dn9 * locals.var_hsat__blk1053) + (locals.var_inv_k1h1_0__blk1066 * locals.var_hsat__blk1053_dn9)), );
            locals.var_inv_k1h1__blk1074_rv = 0.0;
        }
        if (locals.var_guard1080 != 0.0) {
            let assign41390_e47299: f64 = (locals.var_inv_k2h2_0__blk1069 * locals.var_hsat__blk1053);
            (locals.var_inv_k2h2__blk1075, locals.var_inv_k2h2__blk1075_dn4, locals.var_inv_k2h2__blk1075_dn6, locals.var_inv_k2h2__blk1075_dn7, locals.var_inv_k2h2__blk1075_dn8, locals.var_inv_k2h2__blk1075_dn9, ) = (assign41390_e47299, ((locals.var_inv_k2h2_0__blk1069_dn4 * locals.var_hsat__blk1053) + (locals.var_inv_k2h2_0__blk1069 * locals.var_hsat__blk1053_dn4)), ((locals.var_inv_k2h2_0__blk1069_dn6 * locals.var_hsat__blk1053) + (locals.var_inv_k2h2_0__blk1069 * locals.var_hsat__blk1053_dn6)), ((locals.var_inv_k2h2_0__blk1069_dn7 * locals.var_hsat__blk1053) + (locals.var_inv_k2h2_0__blk1069 * locals.var_hsat__blk1053_dn7)), ((locals.var_inv_k2h2_0__blk1069_dn8 * locals.var_hsat__blk1053) + (locals.var_inv_k2h2_0__blk1069 * locals.var_hsat__blk1053_dn8)), ((locals.var_inv_k2h2_0__blk1069_dn9 * locals.var_hsat__blk1053) + (locals.var_inv_k2h2_0__blk1069 * locals.var_hsat__blk1053_dn9)), );
            locals.var_inv_k2h2__blk1075_rv = 0.0;
        }
        if (locals.var_guard1080 != 0.0) {
            let assign41400_e47306: f64 = (locals.var_k1q1d__blk1004 - locals.var_k1q1s__blk939);
            let assign41400_e47307: f64 = (0.5 * assign41400_e47306);
            (locals.var_delta_k1q1__blk1076, locals.var_delta_k1q1__blk1076_dn4, locals.var_delta_k1q1__blk1076_dn6, locals.var_delta_k1q1__blk1076_dn7, locals.var_delta_k1q1__blk1076_dn8, locals.var_delta_k1q1__blk1076_dn9, ) = (assign41400_e47307, (0.5 * (locals.var_k1q1d__blk1004_dn4 - locals.var_k1q1s__blk939_dn4)), (0.5 * (locals.var_k1q1d__blk1004_dn6 - locals.var_k1q1s__blk939_dn6)), (0.5 * (locals.var_k1q1d__blk1004_dn7 - locals.var_k1q1s__blk939_dn7)), (0.5 * (locals.var_k1q1d__blk1004_dn8 - locals.var_k1q1s__blk939_dn8)), (0.5 * (locals.var_k1q1d__blk1004_dn9 - locals.var_k1q1s__blk939_dn9)), );
            locals.var_delta_k1q1__blk1076_rv = 0.0;
        }
        if (locals.var_guard1080 != 0.0) {
            let assign41410_e47314: f64 = (locals.var_k2q2d__blk1005 - locals.var_k2q2s__blk940);
            let assign41410_e47315: f64 = (0.5 * assign41410_e47314);
            (locals.var_delta_k2q2__blk1077, locals.var_delta_k2q2__blk1077_dn4, locals.var_delta_k2q2__blk1077_dn6, locals.var_delta_k2q2__blk1077_dn7, locals.var_delta_k2q2__blk1077_dn8, locals.var_delta_k2q2__blk1077_dn9, ) = (assign41410_e47315, (0.5 * (locals.var_k2q2d__blk1005_dn4 - locals.var_k2q2s__blk940_dn4)), (0.5 * (locals.var_k2q2d__blk1005_dn6 - locals.var_k2q2s__blk940_dn6)), (0.5 * (locals.var_k2q2d__blk1005_dn7 - locals.var_k2q2s__blk940_dn7)), (0.5 * (locals.var_k2q2d__blk1005_dn8 - locals.var_k2q2s__blk940_dn8)), (0.5 * (locals.var_k2q2d__blk1005_dn9 - locals.var_k2q2s__blk940_dn9)), );
            locals.var_delta_k2q2__blk1077_rv = 0.0;
        }
        if (locals.var_guard1080 != 0.0) {
            let assign41420_e47321: f64 = (locals.var_delta_k1q1__blk1076 * locals.var_inv_k1h1__blk1074);
            (locals.var_prod1__blk1078, locals.var_prod1__blk1078_dn4, locals.var_prod1__blk1078_dn6, locals.var_prod1__blk1078_dn7, locals.var_prod1__blk1078_dn8, locals.var_prod1__blk1078_dn9, ) = (assign41420_e47321, ((locals.var_delta_k1q1__blk1076_dn4 * locals.var_inv_k1h1__blk1074) + (locals.var_delta_k1q1__blk1076 * locals.var_inv_k1h1__blk1074_dn4)), ((locals.var_delta_k1q1__blk1076_dn6 * locals.var_inv_k1h1__blk1074) + (locals.var_delta_k1q1__blk1076 * locals.var_inv_k1h1__blk1074_dn6)), ((locals.var_delta_k1q1__blk1076_dn7 * locals.var_inv_k1h1__blk1074) + (locals.var_delta_k1q1__blk1076 * locals.var_inv_k1h1__blk1074_dn7)), ((locals.var_delta_k1q1__blk1076_dn8 * locals.var_inv_k1h1__blk1074) + (locals.var_delta_k1q1__blk1076 * locals.var_inv_k1h1__blk1074_dn8)), ((locals.var_delta_k1q1__blk1076_dn9 * locals.var_inv_k1h1__blk1074) + (locals.var_delta_k1q1__blk1076 * locals.var_inv_k1h1__blk1074_dn9)), );
            locals.var_prod1__blk1078_rv = 0.0;
        }
        if (locals.var_guard1080 != 0.0) {
            let assign41430_e47327: f64 = (locals.var_delta_k2q2__blk1077 * locals.var_inv_k2h2__blk1075);
            (locals.var_prod2__blk1079, locals.var_prod2__blk1079_dn4, locals.var_prod2__blk1079_dn6, locals.var_prod2__blk1079_dn7, locals.var_prod2__blk1079_dn8, locals.var_prod2__blk1079_dn9, ) = (assign41430_e47327, ((locals.var_delta_k2q2__blk1077_dn4 * locals.var_inv_k2h2__blk1075) + (locals.var_delta_k2q2__blk1077 * locals.var_inv_k2h2__blk1075_dn4)), ((locals.var_delta_k2q2__blk1077_dn6 * locals.var_inv_k2h2__blk1075) + (locals.var_delta_k2q2__blk1077 * locals.var_inv_k2h2__blk1075_dn6)), ((locals.var_delta_k2q2__blk1077_dn7 * locals.var_inv_k2h2__blk1075) + (locals.var_delta_k2q2__blk1077 * locals.var_inv_k2h2__blk1075_dn7)), ((locals.var_delta_k2q2__blk1077_dn8 * locals.var_inv_k2h2__blk1075) + (locals.var_delta_k2q2__blk1077 * locals.var_inv_k2h2__blk1075_dn8)), ((locals.var_delta_k2q2__blk1077_dn9 * locals.var_inv_k2h2__blk1075) + (locals.var_delta_k2q2__blk1077 * locals.var_inv_k2h2__blk1075_dn9)), );
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
        let assign42140_e47648: f64 = (locals.var_dx_wi_1d_ac - locals.var_dx_wi_ac);
        let assign42140_e47649: f64 = (locals.var_fsceac_i * assign42140_e47648);
        let assign42140_e47653: f64 = (0.25 * locals.var_qim_ac);
        let assign42140_e47654: f64 = (1.0 + assign42140_e47653);
        let assign42140_e47655: f64 = (assign42140_e47649 / assign42140_e47654);
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign42140_e47655, ((((locals.var_fsceac_i * (locals.var_dx_wi_1d_ac_dn4 - locals.var_dx_wi_ac_dn4)) * assign42140_e47654) - (assign42140_e47649 * (0.25 * locals.var_qim_ac_dn4))) / (assign42140_e47654 * assign42140_e47654)), ((((locals.var_fsceac_i * (locals.var_dx_wi_1d_ac_dn6 - locals.var_dx_wi_ac_dn6)) * assign42140_e47654) - (assign42140_e47649 * (0.25 * locals.var_qim_ac_dn6))) / (assign42140_e47654 * assign42140_e47654)), ((((locals.var_fsceac_i * (locals.var_dx_wi_1d_ac_dn7 - locals.var_dx_wi_ac_dn7)) * assign42140_e47654) - (assign42140_e47649 * (0.25 * locals.var_qim_ac_dn7))) / (assign42140_e47654 * assign42140_e47654)), ((((locals.var_fsceac_i * (locals.var_dx_wi_1d_ac_dn8 - locals.var_dx_wi_ac_dn8)) * assign42140_e47654) - (assign42140_e47649 * (0.25 * locals.var_qim_ac_dn8))) / (assign42140_e47654 * assign42140_e47654)), ((((locals.var_fsceac_i * (locals.var_dx_wi_1d_ac_dn9 - locals.var_dx_wi_ac_dn9)) * assign42140_e47654) - (assign42140_e47649 * (0.25 * locals.var_qim_ac_dn9))) / (assign42140_e47654 * assign42140_e47654)), );
        locals.var_temp_rv = 0.0;
        let assign42150_e47659: f64 = (locals.var_k1q1s_ac + locals.var_k1q1d_ac);
        let assign42150_e47660: f64 = (0.5 * assign42150_e47659);
        let assign42150_e47662: f64 = (assign42150_e47660 + locals.var_temp);
        (locals.var_k1q1m, locals.var_k1q1m_dn4, locals.var_k1q1m_dn6, locals.var_k1q1m_dn7, locals.var_k1q1m_dn8, locals.var_k1q1m_dn9, ) = (assign42150_e47662, ((0.5 * (locals.var_k1q1s_ac_dn4 + locals.var_k1q1d_ac_dn4)) + locals.var_temp_dn4), ((0.5 * (locals.var_k1q1s_ac_dn6 + locals.var_k1q1d_ac_dn6)) + locals.var_temp_dn6), ((0.5 * (locals.var_k1q1s_ac_dn7 + locals.var_k1q1d_ac_dn7)) + locals.var_temp_dn7), ((0.5 * (locals.var_k1q1s_ac_dn8 + locals.var_k1q1d_ac_dn8)) + locals.var_temp_dn8), ((0.5 * (locals.var_k1q1s_ac_dn9 + locals.var_k1q1d_ac_dn9)) + locals.var_temp_dn9), );
        locals.var_k1q1m_rv = 0.0;
        let assign42160_e47666: f64 = (locals.var_k2q2s_ac + locals.var_k2q2d_ac);
        let assign42160_e47667: f64 = (0.5 * assign42160_e47666);
        let assign42160_e47669: f64 = (assign42160_e47667 - locals.var_temp);
        (locals.var_k2q2m, locals.var_k2q2m_dn4, locals.var_k2q2m_dn6, locals.var_k2q2m_dn7, locals.var_k2q2m_dn8, locals.var_k2q2m_dn9, ) = (assign42160_e47669, ((0.5 * (locals.var_k2q2s_ac_dn4 + locals.var_k2q2d_ac_dn4)) - locals.var_temp_dn4), ((0.5 * (locals.var_k2q2s_ac_dn6 + locals.var_k2q2d_ac_dn6)) - locals.var_temp_dn6), ((0.5 * (locals.var_k2q2s_ac_dn7 + locals.var_k2q2d_ac_dn7)) - locals.var_temp_dn7), ((0.5 * (locals.var_k2q2s_ac_dn8 + locals.var_k2q2d_ac_dn8)) - locals.var_temp_dn8), ((0.5 * (locals.var_k2q2s_ac_dn9 + locals.var_k2q2d_ac_dn9)) - locals.var_temp_dn9), );
        locals.var_k2q2m_rv = 0.0;
        let assign42170_e47672: f64 = if p.p13 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1234 = assign42170_e47672;
        locals.var_guard1234_rv = 0.0;
        if (locals.var_guard1234 != 0.0) {
            let assign42180_e47677: f64 = (locals.var_qi1m_ac / locals.var_qmfact1_ac);
            let assign42180_e47678: f64 = (locals.var_k1q1m + assign42180_e47677);
            let assign42180_e47680: f64 = (assign42180_e47678 - locals.var_qi1m_ac);
            (locals.var_k1q1eff, locals.var_k1q1eff_dn4, locals.var_k1q1eff_dn6, locals.var_k1q1eff_dn7, locals.var_k1q1eff_dn8, locals.var_k1q1eff_dn9, ) = (assign42180_e47680, ((locals.var_k1q1m_dn4 + (((locals.var_qi1m_ac_dn4 * locals.var_qmfact1_ac) - (locals.var_qi1m_ac * locals.var_qmfact1_ac_dn4)) / (locals.var_qmfact1_ac * locals.var_qmfact1_ac))) - locals.var_qi1m_ac_dn4), ((locals.var_k1q1m_dn6 + (((locals.var_qi1m_ac_dn6 * locals.var_qmfact1_ac) - (locals.var_qi1m_ac * locals.var_qmfact1_ac_dn6)) / (locals.var_qmfact1_ac * locals.var_qmfact1_ac))) - locals.var_qi1m_ac_dn6), ((locals.var_k1q1m_dn7 + (((locals.var_qi1m_ac_dn7 * locals.var_qmfact1_ac) - (locals.var_qi1m_ac * locals.var_qmfact1_ac_dn7)) / (locals.var_qmfact1_ac * locals.var_qmfact1_ac))) - locals.var_qi1m_ac_dn7), ((locals.var_k1q1m_dn8 + (((locals.var_qi1m_ac_dn8 * locals.var_qmfact1_ac) - (locals.var_qi1m_ac * locals.var_qmfact1_ac_dn8)) / (locals.var_qmfact1_ac * locals.var_qmfact1_ac))) - locals.var_qi1m_ac_dn8), ((locals.var_k1q1m_dn9 + (((locals.var_qi1m_ac_dn9 * locals.var_qmfact1_ac) - (locals.var_qi1m_ac * locals.var_qmfact1_ac_dn9)) / (locals.var_qmfact1_ac * locals.var_qmfact1_ac))) - locals.var_qi1m_ac_dn9), );
            locals.var_k1q1eff_rv = 0.0;
        }
        if (locals.var_guard1234 != 0.0) {
            let assign42190_e47687: f64 = (locals.var_qi2m_ac / locals.var_qmfact2_ac);
            let assign42190_e47688: f64 = (locals.var_k2q2m + assign42190_e47687);
            let assign42190_e47690: f64 = (assign42190_e47688 - locals.var_qi2m_ac);
            (locals.var_k2q2eff, locals.var_k2q2eff_dn4, locals.var_k2q2eff_dn6, locals.var_k2q2eff_dn7, locals.var_k2q2eff_dn8, locals.var_k2q2eff_dn9, ) = (assign42190_e47690, ((locals.var_k2q2m_dn4 + (((locals.var_qi2m_ac_dn4 * locals.var_qmfact2_ac) - (locals.var_qi2m_ac * locals.var_qmfact2_ac_dn4)) / (locals.var_qmfact2_ac * locals.var_qmfact2_ac))) - locals.var_qi2m_ac_dn4), ((locals.var_k2q2m_dn6 + (((locals.var_qi2m_ac_dn6 * locals.var_qmfact2_ac) - (locals.var_qi2m_ac * locals.var_qmfact2_ac_dn6)) / (locals.var_qmfact2_ac * locals.var_qmfact2_ac))) - locals.var_qi2m_ac_dn6), ((locals.var_k2q2m_dn7 + (((locals.var_qi2m_ac_dn7 * locals.var_qmfact2_ac) - (locals.var_qi2m_ac * locals.var_qmfact2_ac_dn7)) / (locals.var_qmfact2_ac * locals.var_qmfact2_ac))) - locals.var_qi2m_ac_dn7), ((locals.var_k2q2m_dn8 + (((locals.var_qi2m_ac_dn8 * locals.var_qmfact2_ac) - (locals.var_qi2m_ac * locals.var_qmfact2_ac_dn8)) / (locals.var_qmfact2_ac * locals.var_qmfact2_ac))) - locals.var_qi2m_ac_dn8), ((locals.var_k2q2m_dn9 + (((locals.var_qi2m_ac_dn9 * locals.var_qmfact2_ac) - (locals.var_qi2m_ac * locals.var_qmfact2_ac_dn9)) / (locals.var_qmfact2_ac * locals.var_qmfact2_ac))) - locals.var_qi2m_ac_dn9), );
            locals.var_k2q2eff_rv = 0.0;
        }
        if (locals.var_guard1234 == 0.0) {
            (locals.var_k1q1eff, locals.var_k1q1eff_dn4, locals.var_k1q1eff_dn6, locals.var_k1q1eff_dn7, locals.var_k1q1eff_dn8, locals.var_k1q1eff_dn9, ) = (locals.var_k1q1m, locals.var_k1q1m_dn4, locals.var_k1q1m_dn6, locals.var_k1q1m_dn7, locals.var_k1q1m_dn8, locals.var_k1q1m_dn9, );
            locals.var_k1q1eff_rv = 0.0;
            (locals.var_k2q2eff, locals.var_k2q2eff_dn4, locals.var_k2q2eff_dn6, locals.var_k2q2eff_dn7, locals.var_k2q2eff_dn8, locals.var_k2q2eff_dn9, ) = (locals.var_k2q2m, locals.var_k2q2m_dn4, locals.var_k2q2m_dn6, locals.var_k2q2m_dn7, locals.var_k2q2m_dn8, locals.var_k2q2m_dn9, );
            locals.var_k2q2eff_rv = 0.0;
        }
        let assign42220_e47705: f64 = (locals.var_delta_k1q1_ac * locals.var_prod1_ac);
        let assign42220_e47707: f64 = (assign42220_e47705 * 0.3333333333333);
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9, ) = (assign42220_e47707, (((locals.var_delta_k1q1_ac_dn4 * locals.var_prod1_ac) + (locals.var_delta_k1q1_ac * locals.var_prod1_ac_dn4)) * 0.3333333333333), (((locals.var_delta_k1q1_ac_dn6 * locals.var_prod1_ac) + (locals.var_delta_k1q1_ac * locals.var_prod1_ac_dn6)) * 0.3333333333333), (((locals.var_delta_k1q1_ac_dn7 * locals.var_prod1_ac) + (locals.var_delta_k1q1_ac * locals.var_prod1_ac_dn7)) * 0.3333333333333), (((locals.var_delta_k1q1_ac_dn8 * locals.var_prod1_ac) + (locals.var_delta_k1q1_ac * locals.var_prod1_ac_dn8)) * 0.3333333333333), (((locals.var_delta_k1q1_ac_dn9 * locals.var_prod1_ac) + (locals.var_delta_k1q1_ac * locals.var_prod1_ac_dn9)) * 0.3333333333333), );
        locals.var_temp1_rv = 0.0;
        let assign42230_e47710: f64 = (locals.var_delta_k1q1_ac * 0.1666666666667);
        let assign42230_e47716: f64 = (0.2 * locals.var_prod1_ac);
        let assign42230_e47717: f64 = (1.0 - assign42230_e47716);
        let assign42230_e47718: f64 = (locals.var_prod1_ac * assign42230_e47717);
        let assign42230_e47719: f64 = (1.0 + assign42230_e47718);
        let assign42230_e47720: f64 = (assign42230_e47710 * assign42230_e47719);
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9, ) = (assign42230_e47720, (((locals.var_delta_k1q1_ac_dn4 * 0.1666666666667) * assign42230_e47719) + (assign42230_e47710 * ((locals.var_prod1_ac_dn4 * assign42230_e47717) + (locals.var_prod1_ac * (-(0.2 * locals.var_prod1_ac_dn4)))))), (((locals.var_delta_k1q1_ac_dn6 * 0.1666666666667) * assign42230_e47719) + (assign42230_e47710 * ((locals.var_prod1_ac_dn6 * assign42230_e47717) + (locals.var_prod1_ac * (-(0.2 * locals.var_prod1_ac_dn6)))))), (((locals.var_delta_k1q1_ac_dn7 * 0.1666666666667) * assign42230_e47719) + (assign42230_e47710 * ((locals.var_prod1_ac_dn7 * assign42230_e47717) + (locals.var_prod1_ac * (-(0.2 * locals.var_prod1_ac_dn7)))))), (((locals.var_delta_k1q1_ac_dn8 * 0.1666666666667) * assign42230_e47719) + (assign42230_e47710 * ((locals.var_prod1_ac_dn8 * assign42230_e47717) + (locals.var_prod1_ac * (-(0.2 * locals.var_prod1_ac_dn8)))))), (((locals.var_delta_k1q1_ac_dn9 * 0.1666666666667) * assign42230_e47719) + (assign42230_e47710 * ((locals.var_prod1_ac_dn9 * assign42230_e47717) + (locals.var_prod1_ac * (-(0.2 * locals.var_prod1_ac_dn9)))))), );
        locals.var_temp2_rv = 0.0;
        let assign42240_e47723: f64 = (0.5 * locals.var_k1q1eff);
        let assign42240_e47725: f64 = (assign42240_e47723 * locals.var_ratio_pd_ac);
        let assign42240_e47727: f64 = (assign42240_e47725 + locals.var_temp2);
        (locals.var_k1q1deff, locals.var_k1q1deff_dn4, locals.var_k1q1deff_dn6, locals.var_k1q1deff_dn7, locals.var_k1q1deff_dn8, locals.var_k1q1deff_dn9, ) = (assign42240_e47727, ((((0.5 * locals.var_k1q1eff_dn4) * locals.var_ratio_pd_ac) + (assign42240_e47723 * locals.var_ratio_pd_ac_dn4)) + locals.var_temp2_dn4), ((((0.5 * locals.var_k1q1eff_dn6) * locals.var_ratio_pd_ac) + (assign42240_e47723 * locals.var_ratio_pd_ac_dn6)) + locals.var_temp2_dn6), ((((0.5 * locals.var_k1q1eff_dn7) * locals.var_ratio_pd_ac) + (assign42240_e47723 * locals.var_ratio_pd_ac_dn7)) + locals.var_temp2_dn7), ((((0.5 * locals.var_k1q1eff_dn8) * locals.var_ratio_pd_ac) + (assign42240_e47723 * locals.var_ratio_pd_ac_dn8)) + locals.var_temp2_dn8), ((((0.5 * locals.var_k1q1eff_dn9) * locals.var_ratio_pd_ac) + (assign42240_e47723 * locals.var_ratio_pd_ac_dn9)) + locals.var_temp2_dn9), );
        locals.var_k1q1deff_rv = 0.0;
        let assign42250_e47730: f64 = (locals.var_k1q1eff * locals.var_ratio_pd_ac);
        let assign42250_e47732: f64 = (assign42250_e47730 + locals.var_temp1);
        (locals.var_k1q1eff, locals.var_k1q1eff_dn4, locals.var_k1q1eff_dn6, locals.var_k1q1eff_dn7, locals.var_k1q1eff_dn8, locals.var_k1q1eff_dn9, ) = (assign42250_e47732, (((locals.var_k1q1eff_dn4 * locals.var_ratio_pd_ac) + (locals.var_k1q1eff * locals.var_ratio_pd_ac_dn4)) + locals.var_temp1_dn4), (((locals.var_k1q1eff_dn6 * locals.var_ratio_pd_ac) + (locals.var_k1q1eff * locals.var_ratio_pd_ac_dn6)) + locals.var_temp1_dn6), (((locals.var_k1q1eff_dn7 * locals.var_ratio_pd_ac) + (locals.var_k1q1eff * locals.var_ratio_pd_ac_dn7)) + locals.var_temp1_dn7), (((locals.var_k1q1eff_dn8 * locals.var_ratio_pd_ac) + (locals.var_k1q1eff * locals.var_ratio_pd_ac_dn8)) + locals.var_temp1_dn8), (((locals.var_k1q1eff_dn9 * locals.var_ratio_pd_ac) + (locals.var_k1q1eff * locals.var_ratio_pd_ac_dn9)) + locals.var_temp1_dn9), );
        locals.var_k1q1eff_rv = 0.0;
        let assign42260_e47735: f64 = (locals.var_delta_k2q2_ac * locals.var_prod2_ac);
        let assign42260_e47737: f64 = (assign42260_e47735 * 0.3333333333333);
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9, ) = (assign42260_e47737, (((locals.var_delta_k2q2_ac_dn4 * locals.var_prod2_ac) + (locals.var_delta_k2q2_ac * locals.var_prod2_ac_dn4)) * 0.3333333333333), (((locals.var_delta_k2q2_ac_dn6 * locals.var_prod2_ac) + (locals.var_delta_k2q2_ac * locals.var_prod2_ac_dn6)) * 0.3333333333333), (((locals.var_delta_k2q2_ac_dn7 * locals.var_prod2_ac) + (locals.var_delta_k2q2_ac * locals.var_prod2_ac_dn7)) * 0.3333333333333), (((locals.var_delta_k2q2_ac_dn8 * locals.var_prod2_ac) + (locals.var_delta_k2q2_ac * locals.var_prod2_ac_dn8)) * 0.3333333333333), (((locals.var_delta_k2q2_ac_dn9 * locals.var_prod2_ac) + (locals.var_delta_k2q2_ac * locals.var_prod2_ac_dn9)) * 0.3333333333333), );
        locals.var_temp1_rv = 0.0;
        let assign42270_e47740: f64 = (locals.var_delta_k2q2_ac * 0.1666666666667);
        let assign42270_e47746: f64 = (0.2 * locals.var_prod2_ac);
        let assign42270_e47747: f64 = (1.0 - assign42270_e47746);
        let assign42270_e47748: f64 = (locals.var_prod2_ac * assign42270_e47747);
        let assign42270_e47749: f64 = (1.0 + assign42270_e47748);
        let assign42270_e47750: f64 = (assign42270_e47740 * assign42270_e47749);
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9, ) = (assign42270_e47750, (((locals.var_delta_k2q2_ac_dn4 * 0.1666666666667) * assign42270_e47749) + (assign42270_e47740 * ((locals.var_prod2_ac_dn4 * assign42270_e47747) + (locals.var_prod2_ac * (-(0.2 * locals.var_prod2_ac_dn4)))))), (((locals.var_delta_k2q2_ac_dn6 * 0.1666666666667) * assign42270_e47749) + (assign42270_e47740 * ((locals.var_prod2_ac_dn6 * assign42270_e47747) + (locals.var_prod2_ac * (-(0.2 * locals.var_prod2_ac_dn6)))))), (((locals.var_delta_k2q2_ac_dn7 * 0.1666666666667) * assign42270_e47749) + (assign42270_e47740 * ((locals.var_prod2_ac_dn7 * assign42270_e47747) + (locals.var_prod2_ac * (-(0.2 * locals.var_prod2_ac_dn7)))))), (((locals.var_delta_k2q2_ac_dn8 * 0.1666666666667) * assign42270_e47749) + (assign42270_e47740 * ((locals.var_prod2_ac_dn8 * assign42270_e47747) + (locals.var_prod2_ac * (-(0.2 * locals.var_prod2_ac_dn8)))))), (((locals.var_delta_k2q2_ac_dn9 * 0.1666666666667) * assign42270_e47749) + (assign42270_e47740 * ((locals.var_prod2_ac_dn9 * assign42270_e47747) + (locals.var_prod2_ac * (-(0.2 * locals.var_prod2_ac_dn9)))))), );
        locals.var_temp2_rv = 0.0;
        let assign42280_e47753: f64 = (0.5 * locals.var_k2q2eff);
        let assign42280_e47755: f64 = (assign42280_e47753 + locals.var_temp2);
        (locals.var_k2q2deff, locals.var_k2q2deff_dn4, locals.var_k2q2deff_dn6, locals.var_k2q2deff_dn7, locals.var_k2q2deff_dn8, locals.var_k2q2deff_dn9, ) = (assign42280_e47755, ((0.5 * locals.var_k2q2eff_dn4) + locals.var_temp2_dn4), ((0.5 * locals.var_k2q2eff_dn6) + locals.var_temp2_dn6), ((0.5 * locals.var_k2q2eff_dn7) + locals.var_temp2_dn7), ((0.5 * locals.var_k2q2eff_dn8) + locals.var_temp2_dn8), ((0.5 * locals.var_k2q2eff_dn9) + locals.var_temp2_dn9), );
        locals.var_k2q2deff_rv = 0.0;
        let assign42290_e47758: f64 = (locals.var_k2q2eff + locals.var_temp1);
        (locals.var_k2q2eff, locals.var_k2q2eff_dn4, locals.var_k2q2eff_dn6, locals.var_k2q2eff_dn7, locals.var_k2q2eff_dn8, locals.var_k2q2eff_dn9, ) = (assign42290_e47758, (locals.var_k2q2eff_dn4 + locals.var_temp1_dn4), (locals.var_k2q2eff_dn6 + locals.var_temp1_dn6), (locals.var_k2q2eff_dn7 + locals.var_temp1_dn7), (locals.var_k2q2eff_dn8 + locals.var_temp1_dn8), (locals.var_k2q2eff_dn9 + locals.var_temp1_dn9), );
        locals.var_k2q2eff_rv = 0.0;
        let assign42300_e47761: f64 = (locals.var_csiprime_ac * locals.var_area_phit);
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign42300_e47761, ((locals.var_csiprime_ac_dn4 * locals.var_area_phit) + (locals.var_csiprime_ac * locals.var_area_phit_dn4)), ((locals.var_csiprime_ac_dn6 * locals.var_area_phit) + (locals.var_csiprime_ac * locals.var_area_phit_dn6)), ((locals.var_csiprime_ac_dn7 * locals.var_area_phit) + (locals.var_csiprime_ac * locals.var_area_phit_dn7)), ((locals.var_csiprime_ac_dn8 * locals.var_area_phit) + (locals.var_csiprime_ac * locals.var_area_phit_dn8)), ((locals.var_csiprime_ac_dn9 * locals.var_area_phit) + (locals.var_csiprime_ac * locals.var_area_phit_dn9)), );
        locals.var_temp_rv = 0.0;
        let assign42310_e47764: f64 = (locals.var_temp * locals.var_k1q1eff);
        (locals.var_qg, locals.var_qg_dn4, locals.var_qg_dn6, locals.var_qg_dn7, locals.var_qg_dn8, locals.var_qg_dn9, ) = (assign42310_e47764, ((locals.var_temp_dn4 * locals.var_k1q1eff) + (locals.var_temp * locals.var_k1q1eff_dn4)), ((locals.var_temp_dn6 * locals.var_k1q1eff) + (locals.var_temp * locals.var_k1q1eff_dn6)), ((locals.var_temp_dn7 * locals.var_k1q1eff) + (locals.var_temp * locals.var_k1q1eff_dn7)), ((locals.var_temp_dn8 * locals.var_k1q1eff) + (locals.var_temp * locals.var_k1q1eff_dn8)), ((locals.var_temp_dn9 * locals.var_k1q1eff) + (locals.var_temp * locals.var_k1q1eff_dn9)), );
        locals.var_qg_rv = 0.0;
        let assign42320_e47767: f64 = (locals.var_temp * locals.var_k2q2eff);
        (locals.var_qb, locals.var_qb_dn4, locals.var_qb_dn6, locals.var_qb_dn7, locals.var_qb_dn8, locals.var_qb_dn9, ) = (assign42320_e47767, ((locals.var_temp_dn4 * locals.var_k2q2eff) + (locals.var_temp * locals.var_k2q2eff_dn4)), ((locals.var_temp_dn6 * locals.var_k2q2eff) + (locals.var_temp * locals.var_k2q2eff_dn6)), ((locals.var_temp_dn7 * locals.var_k2q2eff) + (locals.var_temp * locals.var_k2q2eff_dn7)), ((locals.var_temp_dn8 * locals.var_k2q2eff) + (locals.var_temp * locals.var_k2q2eff_dn8)), ((locals.var_temp_dn9 * locals.var_k2q2eff) + (locals.var_temp * locals.var_k2q2eff_dn9)), );
        locals.var_qb_rv = 0.0;
        let assign42330_e47769: f64 = (-locals.var_temp);
        let assign42330_e47772: f64 = (locals.var_k1q1deff + locals.var_k2q2deff);
        let assign42330_e47773: f64 = (assign42330_e47769 * assign42330_e47772);
        (locals.var_qd, locals.var_qd_dn4, locals.var_qd_dn6, locals.var_qd_dn7, locals.var_qd_dn8, locals.var_qd_dn9, ) = (assign42330_e47773, (((-locals.var_temp_dn4) * assign42330_e47772) + (assign42330_e47769 * (locals.var_k1q1deff_dn4 + locals.var_k2q2deff_dn4))), (((-locals.var_temp_dn6) * assign42330_e47772) + (assign42330_e47769 * (locals.var_k1q1deff_dn6 + locals.var_k2q2deff_dn6))), (((-locals.var_temp_dn7) * assign42330_e47772) + (assign42330_e47769 * (locals.var_k1q1deff_dn7 + locals.var_k2q2deff_dn7))), (((-locals.var_temp_dn8) * assign42330_e47772) + (assign42330_e47769 * (locals.var_k1q1deff_dn8 + locals.var_k2q2deff_dn8))), (((-locals.var_temp_dn9) * assign42330_e47772) + (assign42330_e47769 * (locals.var_k1q1deff_dn9 + locals.var_k2q2deff_dn9))), );
        locals.var_qd_rv = 0.0;
        let assign42340_e47776: f64 = if locals.var_fif_i > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1235 = assign42340_e47776;
        locals.var_guard1235_rv = 0.0;
        if (locals.var_guard1235 != 0.0) {
            let assign42350_e47781: f64 = (2.0 * 0.6931471805599);
            let assign42350_e47782: f64 = (locals.var_xth_1d + assign42350_e47781);
            (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign42350_e47782, locals.var_xth_1d_dn4, locals.var_xth_1d_dn6, locals.var_xth_1d_dn7, locals.var_xth_1d_dn8, locals.var_xth_1d_dn9, );
            locals.var_temp_rv = 0.0;
        }
        if (locals.var_guard1235 != 0.0) {
            let assign42360_e47788: f64 = (locals.var_xdrifts_ac + locals.var_temp);
            (locals.var_xeffs, locals.var_xeffs_dn4, locals.var_xeffs_dn6, locals.var_xeffs_dn7, locals.var_xeffs_dn8, locals.var_xeffs_dn9, ) = (assign42360_e47788, (locals.var_xdrifts_ac_dn4 + locals.var_temp_dn4), (locals.var_xdrifts_ac_dn6 + locals.var_temp_dn6), (locals.var_xdrifts_ac_dn7 + locals.var_temp_dn7), (locals.var_xdrifts_ac_dn8 + locals.var_temp_dn8), (locals.var_xdrifts_ac_dn9 + locals.var_temp_dn9), );
            locals.var_xeffs_rv = 0.0;
        }
        if (locals.var_guard1235 != 0.0) {
            let assign42370_e47794: f64 = (locals.var_xdriftd_ac + locals.var_temp);
            (locals.var_xeffd, locals.var_xeffd_dn4, locals.var_xeffd_dn6, locals.var_xeffd_dn7, locals.var_xeffd_dn8, locals.var_xeffd_dn9, ) = (assign42370_e47794, (locals.var_xdriftd_ac_dn4 + locals.var_temp_dn4), (locals.var_xdriftd_ac_dn6 + locals.var_temp_dn6), (locals.var_xdriftd_ac_dn7 + locals.var_temp_dn7), (locals.var_xdriftd_ac_dn8 + locals.var_temp_dn8), (locals.var_xdriftd_ac_dn9 + locals.var_temp_dn9), );
            locals.var_xeffd_rv = 0.0;
        }
        if (locals.var_guard1235 != 0.0) {
            let assign42380_e47801: f64 = (locals.var_xeffs + locals.var_xth_1d);
            let assign42380_e47804: f64 = (locals.var_xeffs - locals.var_xth_1d);
            let assign42380_e47807: f64 = (locals.var_xeffs - locals.var_xth_1d);
            let assign42380_e47808: f64 = (assign42380_e47804 * assign42380_e47807);
            let assign42380_e47810: f64 = (assign42380_e47808 + 9.0);
            let assign42380_e47811: f64 = (assign42380_e47810).sqrt();
            let assign42380_e47812: f64 = (assign42380_e47801 - assign42380_e47811);
            let assign42380_e47813: f64 = (0.5 * assign42380_e47812);
            (locals.var_xstars, locals.var_xstars_dn4, locals.var_xstars_dn6, locals.var_xstars_dn7, locals.var_xstars_dn8, locals.var_xstars_dn9, ) = (assign42380_e47813, (0.5 * ((locals.var_xeffs_dn4 + locals.var_xth_1d_dn4) - ((((locals.var_xeffs_dn4 - locals.var_xth_1d_dn4) * assign42380_e47807) + (assign42380_e47804 * (locals.var_xeffs_dn4 - locals.var_xth_1d_dn4))) / (2.0 * assign42380_e47811)))), (0.5 * ((locals.var_xeffs_dn6 + locals.var_xth_1d_dn6) - ((((locals.var_xeffs_dn6 - locals.var_xth_1d_dn6) * assign42380_e47807) + (assign42380_e47804 * (locals.var_xeffs_dn6 - locals.var_xth_1d_dn6))) / (2.0 * assign42380_e47811)))), (0.5 * ((locals.var_xeffs_dn7 + locals.var_xth_1d_dn7) - ((((locals.var_xeffs_dn7 - locals.var_xth_1d_dn7) * assign42380_e47807) + (assign42380_e47804 * (locals.var_xeffs_dn7 - locals.var_xth_1d_dn7))) / (2.0 * assign42380_e47811)))), (0.5 * ((locals.var_xeffs_dn8 + locals.var_xth_1d_dn8) - ((((locals.var_xeffs_dn8 - locals.var_xth_1d_dn8) * assign42380_e47807) + (assign42380_e47804 * (locals.var_xeffs_dn8 - locals.var_xth_1d_dn8))) / (2.0 * assign42380_e47811)))), (0.5 * ((locals.var_xeffs_dn9 + locals.var_xth_1d_dn9) - ((((locals.var_xeffs_dn9 - locals.var_xth_1d_dn9) * assign42380_e47807) + (assign42380_e47804 * (locals.var_xeffs_dn9 - locals.var_xth_1d_dn9))) / (2.0 * assign42380_e47811)))), );
            locals.var_xstars_rv = 0.0;
        }
        if (locals.var_guard1235 != 0.0) {
            let assign42390_e47821: f64 = (locals.var_xth_1d + locals.var_xd);
            let assign42390_e47822: f64 = (locals.var_xeffd + assign42390_e47821);
            let assign42390_e47826: f64 = (locals.var_xth_1d + locals.var_xd);
            let assign42390_e47827: f64 = (locals.var_xeffd - assign42390_e47826);
            let assign42390_e47831: f64 = (locals.var_xth_1d + locals.var_xd);
            let assign42390_e47832: f64 = (locals.var_xeffd - assign42390_e47831);
            let assign42390_e47833: f64 = (assign42390_e47827 * assign42390_e47832);
            let assign42390_e47835: f64 = (assign42390_e47833 + 9.0);
            let assign42390_e47836: f64 = (assign42390_e47835).sqrt();
            let assign42390_e47837: f64 = (assign42390_e47822 - assign42390_e47836);
            let assign42390_e47838: f64 = (0.5 * assign42390_e47837);
            (locals.var_xstard, locals.var_xstard_dn4, locals.var_xstard_dn6, locals.var_xstard_dn7, locals.var_xstard_dn8, locals.var_xstard_dn9, ) = (assign42390_e47838, (0.5 * ((locals.var_xeffd_dn4 + (locals.var_xth_1d_dn4 + locals.var_xd_dn4)) - ((((locals.var_xeffd_dn4 - (locals.var_xth_1d_dn4 + locals.var_xd_dn4)) * assign42390_e47832) + (assign42390_e47827 * (locals.var_xeffd_dn4 - (locals.var_xth_1d_dn4 + locals.var_xd_dn4)))) / (2.0 * assign42390_e47836)))), (0.5 * ((locals.var_xeffd_dn6 + (locals.var_xth_1d_dn6 + locals.var_xd_dn6)) - ((((locals.var_xeffd_dn6 - (locals.var_xth_1d_dn6 + locals.var_xd_dn6)) * assign42390_e47832) + (assign42390_e47827 * (locals.var_xeffd_dn6 - (locals.var_xth_1d_dn6 + locals.var_xd_dn6)))) / (2.0 * assign42390_e47836)))), (0.5 * ((locals.var_xeffd_dn7 + (locals.var_xth_1d_dn7 + locals.var_xd_dn7)) - ((((locals.var_xeffd_dn7 - (locals.var_xth_1d_dn7 + locals.var_xd_dn7)) * assign42390_e47832) + (assign42390_e47827 * (locals.var_xeffd_dn7 - (locals.var_xth_1d_dn7 + locals.var_xd_dn7)))) / (2.0 * assign42390_e47836)))), (0.5 * ((locals.var_xeffd_dn8 + (locals.var_xth_1d_dn8 + locals.var_xd_dn8)) - ((((locals.var_xeffd_dn8 - (locals.var_xth_1d_dn8 + locals.var_xd_dn8)) * assign42390_e47832) + (assign42390_e47827 * (locals.var_xeffd_dn8 - (locals.var_xth_1d_dn8 + locals.var_xd_dn8)))) / (2.0 * assign42390_e47836)))), (0.5 * ((locals.var_xeffd_dn9 + (locals.var_xth_1d_dn9 + locals.var_xd_dn9)) - ((((locals.var_xeffd_dn9 - (locals.var_xth_1d_dn9 + locals.var_xd_dn9)) * assign42390_e47832) + (assign42390_e47827 * (locals.var_xeffd_dn9 - (locals.var_xth_1d_dn9 + locals.var_xd_dn9)))) / (2.0 * assign42390_e47836)))), );
            locals.var_xstard_rv = 0.0;
        }
        if (locals.var_guard1235 != 0.0) {
            let assign42400_e47846: f64 = (0.5 + locals.var_inv_k2_ac);
            let assign42400_e47847: f64 = (locals.var_keq_ac * assign42400_e47846);
            let assign42400_e47848: f64 = (assign42400_e47847).sqrt();
            let assign42400_e47849: f64 = (locals.var_lambda2d * assign42400_e47848);
            (locals.var_lambdaf, locals.var_lambdaf_dn4, locals.var_lambdaf_dn6, locals.var_lambdaf_dn7, locals.var_lambdaf_dn8, locals.var_lambdaf_dn9, ) = (assign42400_e47849, (locals.var_lambda2d * (((locals.var_keq_ac_dn4 * assign42400_e47846) + (locals.var_keq_ac * locals.var_inv_k2_ac_dn4)) / (2.0 * assign42400_e47848))), (locals.var_lambda2d * (((locals.var_keq_ac_dn6 * assign42400_e47846) + (locals.var_keq_ac * locals.var_inv_k2_ac_dn6)) / (2.0 * assign42400_e47848))), (locals.var_lambda2d * (((locals.var_keq_ac_dn7 * assign42400_e47846) + (locals.var_keq_ac * locals.var_inv_k2_ac_dn7)) / (2.0 * assign42400_e47848))), (locals.var_lambda2d * (((locals.var_keq_ac_dn8 * assign42400_e47846) + (locals.var_keq_ac * locals.var_inv_k2_ac_dn8)) / (2.0 * assign42400_e47848))), (locals.var_lambda2d * (((locals.var_keq_ac_dn9 * assign42400_e47846) + (locals.var_keq_ac * locals.var_inv_k2_ac_dn9)) / (2.0 * assign42400_e47848))), );
            locals.var_lambdaf_rv = 0.0;
        }
        if (locals.var_guard1235 != 0.0) {
            let assign42410_e47856: f64 = (locals.var_keq_ac * locals.var_k1_ac);
            let assign42410_e47858: f64 = (assign42410_e47856 * locals.var_inv_k2_ac);
            let assign42410_e47861: f64 = (0.5 + locals.var_inv_k1_ac);
            let assign42410_e47862: f64 = (assign42410_e47858 * assign42410_e47861);
            let assign42410_e47863: f64 = (assign42410_e47862).sqrt();
            let assign42410_e47864: f64 = (locals.var_lambda2d * assign42410_e47863);
            (locals.var_lambdab, locals.var_lambdab_dn4, locals.var_lambdab_dn6, locals.var_lambdab_dn7, locals.var_lambdab_dn8, locals.var_lambdab_dn9, ) = (assign42410_e47864, (locals.var_lambda2d * (((((((locals.var_keq_ac_dn4 * locals.var_k1_ac) + (locals.var_keq_ac * locals.var_k1_ac_dn4)) * locals.var_inv_k2_ac) + (assign42410_e47856 * locals.var_inv_k2_ac_dn4)) * assign42410_e47861) + (assign42410_e47858 * locals.var_inv_k1_ac_dn4)) / (2.0 * assign42410_e47863))), (locals.var_lambda2d * (((((((locals.var_keq_ac_dn6 * locals.var_k1_ac) + (locals.var_keq_ac * locals.var_k1_ac_dn6)) * locals.var_inv_k2_ac) + (assign42410_e47856 * locals.var_inv_k2_ac_dn6)) * assign42410_e47861) + (assign42410_e47858 * locals.var_inv_k1_ac_dn6)) / (2.0 * assign42410_e47863))), (locals.var_lambda2d * (((((((locals.var_keq_ac_dn7 * locals.var_k1_ac) + (locals.var_keq_ac * locals.var_k1_ac_dn7)) * locals.var_inv_k2_ac) + (assign42410_e47856 * locals.var_inv_k2_ac_dn7)) * assign42410_e47861) + (assign42410_e47858 * locals.var_inv_k1_ac_dn7)) / (2.0 * assign42410_e47863))), (locals.var_lambda2d * (((((((locals.var_keq_ac_dn8 * locals.var_k1_ac) + (locals.var_keq_ac * locals.var_k1_ac_dn8)) * locals.var_inv_k2_ac) + (assign42410_e47856 * locals.var_inv_k2_ac_dn8)) * assign42410_e47861) + (assign42410_e47858 * locals.var_inv_k1_ac_dn8)) / (2.0 * assign42410_e47863))), (locals.var_lambda2d * (((((((locals.var_keq_ac_dn9 * locals.var_k1_ac) + (locals.var_keq_ac * locals.var_k1_ac_dn9)) * locals.var_inv_k2_ac) + (assign42410_e47856 * locals.var_inv_k2_ac_dn9)) * assign42410_e47861) + (assign42410_e47858 * locals.var_inv_k1_ac_dn9)) / (2.0 * assign42410_e47863))), );
            locals.var_lambdab_rv = 0.0;
        }
        if (locals.var_guard1235 != 0.0) {
            let assign42420_e47870: f64 = (locals.var_lambdaf * locals.var_lambdaf);
            let assign42420_e47872: f64 = (assign42420_e47870 * locals.var_inner_sd);
            (locals.var_xalphaf, locals.var_xalphaf_dn4, locals.var_xalphaf_dn6, locals.var_xalphaf_dn7, locals.var_xalphaf_dn8, locals.var_xalphaf_dn9, ) = (assign42420_e47872, ((((locals.var_lambdaf_dn4 * locals.var_lambdaf) + (locals.var_lambdaf * locals.var_lambdaf_dn4)) * locals.var_inner_sd) + (assign42420_e47870 * locals.var_inner_sd_dn4)), ((((locals.var_lambdaf_dn6 * locals.var_lambdaf) + (locals.var_lambdaf * locals.var_lambdaf_dn6)) * locals.var_inner_sd) + (assign42420_e47870 * locals.var_inner_sd_dn6)), ((((locals.var_lambdaf_dn7 * locals.var_lambdaf) + (locals.var_lambdaf * locals.var_lambdaf_dn7)) * locals.var_inner_sd) + (assign42420_e47870 * locals.var_inner_sd_dn7)), ((((locals.var_lambdaf_dn8 * locals.var_lambdaf) + (locals.var_lambdaf * locals.var_lambdaf_dn8)) * locals.var_inner_sd) + (assign42420_e47870 * locals.var_inner_sd_dn8)), ((((locals.var_lambdaf_dn9 * locals.var_lambdaf) + (locals.var_lambdaf * locals.var_lambdaf_dn9)) * locals.var_inner_sd) + (assign42420_e47870 * locals.var_inner_sd_dn9)), );
            locals.var_xalphaf_rv = 0.0;
        }
        if (locals.var_guard1235 != 0.0) {
            let assign42430_e47878: f64 = (locals.var_lambdab * locals.var_lambdab);
            let assign42430_e47880: f64 = (assign42430_e47878 * locals.var_inner_sd);
            (locals.var_xalphab, locals.var_xalphab_dn4, locals.var_xalphab_dn6, locals.var_xalphab_dn7, locals.var_xalphab_dn8, locals.var_xalphab_dn9, ) = (assign42430_e47880, ((((locals.var_lambdab_dn4 * locals.var_lambdab) + (locals.var_lambdab * locals.var_lambdab_dn4)) * locals.var_inner_sd) + (assign42430_e47878 * locals.var_inner_sd_dn4)), ((((locals.var_lambdab_dn6 * locals.var_lambdab) + (locals.var_lambdab * locals.var_lambdab_dn6)) * locals.var_inner_sd) + (assign42430_e47878 * locals.var_inner_sd_dn6)), ((((locals.var_lambdab_dn7 * locals.var_lambdab) + (locals.var_lambdab * locals.var_lambdab_dn7)) * locals.var_inner_sd) + (assign42430_e47878 * locals.var_inner_sd_dn7)), ((((locals.var_lambdab_dn8 * locals.var_lambdab) + (locals.var_lambdab * locals.var_lambdab_dn8)) * locals.var_inner_sd) + (assign42430_e47878 * locals.var_inner_sd_dn8)), ((((locals.var_lambdab_dn9 * locals.var_lambdab) + (locals.var_lambdab * locals.var_lambdab_dn9)) * locals.var_inner_sd) + (assign42430_e47878 * locals.var_inner_sd_dn9)), );
            locals.var_xalphab_rv = 0.0;
        }
        if (locals.var_guard1235 != 0.0) {
            let assign42440_e47886: f64 = (locals.var_xsd - locals.var_xstars);
            (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9, ) = (assign42440_e47886, (locals.var_xsd_dn4 - locals.var_xstars_dn4), (locals.var_xsd_dn6 - locals.var_xstars_dn6), (locals.var_xsd_dn7 - locals.var_xstars_dn7), (locals.var_xsd_dn8 - locals.var_xstars_dn8), (locals.var_xsd_dn9 - locals.var_xstars_dn9), );
            locals.var_temp1_rv = 0.0;
        }
        if (locals.var_guard1235 != 0.0) {
            let assign42450_e47892: f64 = (locals.var_xsd + locals.var_xd);
            let assign42450_e47894: f64 = (assign42450_e47892 - locals.var_xstard);
            (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9, ) = (assign42450_e47894, ((locals.var_xsd_dn4 + locals.var_xd_dn4) - locals.var_xstard_dn4), ((locals.var_xsd_dn6 + locals.var_xd_dn6) - locals.var_xstard_dn6), ((locals.var_xsd_dn7 + locals.var_xd_dn7) - locals.var_xstard_dn7), ((locals.var_xsd_dn8 + locals.var_xd_dn8) - locals.var_xstard_dn8), ((locals.var_xsd_dn9 + locals.var_xd_dn9) - locals.var_xstard_dn9), );
            locals.var_temp2_rv = 0.0;
        }
        if (locals.var_guard1235 != 0.0) {
            let assign42460_e47900: f64 = (2.0 * locals.var_xalphaf);
            (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign42460_e47900, (2.0 * locals.var_xalphaf_dn4), (2.0 * locals.var_xalphaf_dn6), (2.0 * locals.var_xalphaf_dn7), (2.0 * locals.var_xalphaf_dn8), (2.0 * locals.var_xalphaf_dn9), );
            locals.var_temp_rv = 0.0;
        }
        if (locals.var_guard1235 != 0.0) {
            let assign42470_e47909: f64 = (locals.var_temp1 / locals.var_xalphaf);
            let assign42470_e47910: f64 = (1.0 + assign42470_e47909);
            let assign42470_e47911: f64 = (assign42470_e47910).sqrt();
            let assign42470_e47913: f64 = (assign42470_e47911 - 1.0);
            let assign42470_e47914: f64 = (locals.var_temp * assign42470_e47913);
            let assign42470_e47915: f64 = (locals.var_xstars + assign42470_e47914);
            (locals.var_xedgefs, locals.var_xedgefs_dn4, locals.var_xedgefs_dn6, locals.var_xedgefs_dn7, locals.var_xedgefs_dn8, locals.var_xedgefs_dn9, ) = (assign42470_e47915, (locals.var_xstars_dn4 + ((locals.var_temp_dn4 * assign42470_e47913) + (locals.var_temp * ((((locals.var_temp1_dn4 * locals.var_xalphaf) - (locals.var_temp1 * locals.var_xalphaf_dn4)) / (locals.var_xalphaf * locals.var_xalphaf)) / (2.0 * assign42470_e47911))))), (locals.var_xstars_dn6 + ((locals.var_temp_dn6 * assign42470_e47913) + (locals.var_temp * ((((locals.var_temp1_dn6 * locals.var_xalphaf) - (locals.var_temp1 * locals.var_xalphaf_dn6)) / (locals.var_xalphaf * locals.var_xalphaf)) / (2.0 * assign42470_e47911))))), (locals.var_xstars_dn7 + ((locals.var_temp_dn7 * assign42470_e47913) + (locals.var_temp * ((((locals.var_temp1_dn7 * locals.var_xalphaf) - (locals.var_temp1 * locals.var_xalphaf_dn7)) / (locals.var_xalphaf * locals.var_xalphaf)) / (2.0 * assign42470_e47911))))), (locals.var_xstars_dn8 + ((locals.var_temp_dn8 * assign42470_e47913) + (locals.var_temp * ((((locals.var_temp1_dn8 * locals.var_xalphaf) - (locals.var_temp1 * locals.var_xalphaf_dn8)) / (locals.var_xalphaf * locals.var_xalphaf)) / (2.0 * assign42470_e47911))))), (locals.var_xstars_dn9 + ((locals.var_temp_dn9 * assign42470_e47913) + (locals.var_temp * ((((locals.var_temp1_dn9 * locals.var_xalphaf) - (locals.var_temp1 * locals.var_xalphaf_dn9)) / (locals.var_xalphaf * locals.var_xalphaf)) / (2.0 * assign42470_e47911))))), );
            locals.var_xedgefs_rv = 0.0;
        }
    }
    pub(super) fn stamp_reactive_block_61(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        if (locals.var_guard1235 != 0.0) {
            let assign42480_e47924: f64 = (locals.var_temp2 / locals.var_xalphaf);
            let assign42480_e47925: f64 = (1.0 + assign42480_e47924);
            let assign42480_e47926: f64 = (assign42480_e47925).sqrt();
            let assign42480_e47928: f64 = (assign42480_e47926 - 1.0);
            let assign42480_e47929: f64 = (locals.var_temp * assign42480_e47928);
            let assign42480_e47930: f64 = (locals.var_xstard + assign42480_e47929);
            (locals.var_xedgefd, locals.var_xedgefd_dn4, locals.var_xedgefd_dn6, locals.var_xedgefd_dn7, locals.var_xedgefd_dn8, locals.var_xedgefd_dn9, ) = (assign42480_e47930, (locals.var_xstard_dn4 + ((locals.var_temp_dn4 * assign42480_e47928) + (locals.var_temp * ((((locals.var_temp2_dn4 * locals.var_xalphaf) - (locals.var_temp2 * locals.var_xalphaf_dn4)) / (locals.var_xalphaf * locals.var_xalphaf)) / (2.0 * assign42480_e47926))))), (locals.var_xstard_dn6 + ((locals.var_temp_dn6 * assign42480_e47928) + (locals.var_temp * ((((locals.var_temp2_dn6 * locals.var_xalphaf) - (locals.var_temp2 * locals.var_xalphaf_dn6)) / (locals.var_xalphaf * locals.var_xalphaf)) / (2.0 * assign42480_e47926))))), (locals.var_xstard_dn7 + ((locals.var_temp_dn7 * assign42480_e47928) + (locals.var_temp * ((((locals.var_temp2_dn7 * locals.var_xalphaf) - (locals.var_temp2 * locals.var_xalphaf_dn7)) / (locals.var_xalphaf * locals.var_xalphaf)) / (2.0 * assign42480_e47926))))), (locals.var_xstard_dn8 + ((locals.var_temp_dn8 * assign42480_e47928) + (locals.var_temp * ((((locals.var_temp2_dn8 * locals.var_xalphaf) - (locals.var_temp2 * locals.var_xalphaf_dn8)) / (locals.var_xalphaf * locals.var_xalphaf)) / (2.0 * assign42480_e47926))))), (locals.var_xstard_dn9 + ((locals.var_temp_dn9 * assign42480_e47928) + (locals.var_temp * ((((locals.var_temp2_dn9 * locals.var_xalphaf) - (locals.var_temp2 * locals.var_xalphaf_dn9)) / (locals.var_xalphaf * locals.var_xalphaf)) / (2.0 * assign42480_e47926))))), );
            locals.var_xedgefd_rv = 0.0;
        }
        if (locals.var_guard1235 != 0.0) {
            let assign42490_e47936: f64 = (2.0 * locals.var_xalphab);
            (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign42490_e47936, (2.0 * locals.var_xalphab_dn4), (2.0 * locals.var_xalphab_dn6), (2.0 * locals.var_xalphab_dn7), (2.0 * locals.var_xalphab_dn8), (2.0 * locals.var_xalphab_dn9), );
            locals.var_temp_rv = 0.0;
        }
        if (locals.var_guard1235 != 0.0) {
            let assign42500_e47945: f64 = (locals.var_temp1 / locals.var_xalphab);
            let assign42500_e47946: f64 = (1.0 + assign42500_e47945);
            let assign42500_e47947: f64 = (assign42500_e47946).sqrt();
            let assign42500_e47949: f64 = (assign42500_e47947 - 1.0);
            let assign42500_e47950: f64 = (locals.var_temp * assign42500_e47949);
            let assign42500_e47951: f64 = (locals.var_xstars + assign42500_e47950);
            (locals.var_xedgebs, locals.var_xedgebs_dn4, locals.var_xedgebs_dn6, locals.var_xedgebs_dn7, locals.var_xedgebs_dn8, locals.var_xedgebs_dn9, ) = (assign42500_e47951, (locals.var_xstars_dn4 + ((locals.var_temp_dn4 * assign42500_e47949) + (locals.var_temp * ((((locals.var_temp1_dn4 * locals.var_xalphab) - (locals.var_temp1 * locals.var_xalphab_dn4)) / (locals.var_xalphab * locals.var_xalphab)) / (2.0 * assign42500_e47947))))), (locals.var_xstars_dn6 + ((locals.var_temp_dn6 * assign42500_e47949) + (locals.var_temp * ((((locals.var_temp1_dn6 * locals.var_xalphab) - (locals.var_temp1 * locals.var_xalphab_dn6)) / (locals.var_xalphab * locals.var_xalphab)) / (2.0 * assign42500_e47947))))), (locals.var_xstars_dn7 + ((locals.var_temp_dn7 * assign42500_e47949) + (locals.var_temp * ((((locals.var_temp1_dn7 * locals.var_xalphab) - (locals.var_temp1 * locals.var_xalphab_dn7)) / (locals.var_xalphab * locals.var_xalphab)) / (2.0 * assign42500_e47947))))), (locals.var_xstars_dn8 + ((locals.var_temp_dn8 * assign42500_e47949) + (locals.var_temp * ((((locals.var_temp1_dn8 * locals.var_xalphab) - (locals.var_temp1 * locals.var_xalphab_dn8)) / (locals.var_xalphab * locals.var_xalphab)) / (2.0 * assign42500_e47947))))), (locals.var_xstars_dn9 + ((locals.var_temp_dn9 * assign42500_e47949) + (locals.var_temp * ((((locals.var_temp1_dn9 * locals.var_xalphab) - (locals.var_temp1 * locals.var_xalphab_dn9)) / (locals.var_xalphab * locals.var_xalphab)) / (2.0 * assign42500_e47947))))), );
            locals.var_xedgebs_rv = 0.0;
        }
        if (locals.var_guard1235 != 0.0) {
            let assign42510_e47960: f64 = (locals.var_temp2 / locals.var_xalphab);
            let assign42510_e47961: f64 = (1.0 + assign42510_e47960);
            let assign42510_e47962: f64 = (assign42510_e47961).sqrt();
            let assign42510_e47964: f64 = (assign42510_e47962 - 1.0);
            let assign42510_e47965: f64 = (locals.var_temp * assign42510_e47964);
            let assign42510_e47966: f64 = (locals.var_xstard + assign42510_e47965);
            (locals.var_xedgebd, locals.var_xedgebd_dn4, locals.var_xedgebd_dn6, locals.var_xedgebd_dn7, locals.var_xedgebd_dn8, locals.var_xedgebd_dn9, ) = (assign42510_e47966, (locals.var_xstard_dn4 + ((locals.var_temp_dn4 * assign42510_e47964) + (locals.var_temp * ((((locals.var_temp2_dn4 * locals.var_xalphab) - (locals.var_temp2 * locals.var_xalphab_dn4)) / (locals.var_xalphab * locals.var_xalphab)) / (2.0 * assign42510_e47962))))), (locals.var_xstard_dn6 + ((locals.var_temp_dn6 * assign42510_e47964) + (locals.var_temp * ((((locals.var_temp2_dn6 * locals.var_xalphab) - (locals.var_temp2 * locals.var_xalphab_dn6)) / (locals.var_xalphab * locals.var_xalphab)) / (2.0 * assign42510_e47962))))), (locals.var_xstard_dn7 + ((locals.var_temp_dn7 * assign42510_e47964) + (locals.var_temp * ((((locals.var_temp2_dn7 * locals.var_xalphab) - (locals.var_temp2 * locals.var_xalphab_dn7)) / (locals.var_xalphab * locals.var_xalphab)) / (2.0 * assign42510_e47962))))), (locals.var_xstard_dn8 + ((locals.var_temp_dn8 * assign42510_e47964) + (locals.var_temp * ((((locals.var_temp2_dn8 * locals.var_xalphab) - (locals.var_temp2 * locals.var_xalphab_dn8)) / (locals.var_xalphab * locals.var_xalphab)) / (2.0 * assign42510_e47962))))), (locals.var_xstard_dn9 + ((locals.var_temp_dn9 * assign42510_e47964) + (locals.var_temp * ((((locals.var_temp2_dn9 * locals.var_xalphab) - (locals.var_temp2 * locals.var_xalphab_dn9)) / (locals.var_xalphab * locals.var_xalphab)) / (2.0 * assign42510_e47962))))), );
            locals.var_xedgebd_rv = 0.0;
        }
        if (locals.var_guard1235 != 0.0) {
            let assign42520_e47972: f64 = (locals.var_fif_phit * locals.var_csiprime_ac);
            (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign42520_e47972, ((locals.var_fif_phit_dn4 * locals.var_csiprime_ac) + (locals.var_fif_phit * locals.var_csiprime_ac_dn4)), ((locals.var_fif_phit_dn6 * locals.var_csiprime_ac) + (locals.var_fif_phit * locals.var_csiprime_ac_dn6)), ((locals.var_fif_phit_dn7 * locals.var_csiprime_ac) + (locals.var_fif_phit * locals.var_csiprime_ac_dn7)), ((locals.var_fif_phit_dn8 * locals.var_csiprime_ac) + (locals.var_fif_phit * locals.var_csiprime_ac_dn8)), ((locals.var_fif_phit_dn9 * locals.var_csiprime_ac) + (locals.var_fif_phit * locals.var_csiprime_ac_dn9)), );
            locals.var_temp_rv = 0.0;
        }
        if (locals.var_guard1235 != 0.0) {
            let assign42530_e47977: f64 = (-locals.var_temp);
            let assign42530_e47979: f64 = (assign42530_e47977 * locals.var_lambdaf);
            let assign42530_e47981: f64 = (assign42530_e47979 * locals.var_k1_ac);
            let assign42530_e47983: f64 = (assign42530_e47981 * locals.var_sce1_ac);
            (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9, ) = (assign42530_e47983, (((((((-locals.var_temp_dn4) * locals.var_lambdaf) + (assign42530_e47977 * locals.var_lambdaf_dn4)) * locals.var_k1_ac) + (assign42530_e47979 * locals.var_k1_ac_dn4)) * locals.var_sce1_ac) + (assign42530_e47981 * locals.var_sce1_ac_dn4)), (((((((-locals.var_temp_dn6) * locals.var_lambdaf) + (assign42530_e47977 * locals.var_lambdaf_dn6)) * locals.var_k1_ac) + (assign42530_e47979 * locals.var_k1_ac_dn6)) * locals.var_sce1_ac) + (assign42530_e47981 * locals.var_sce1_ac_dn6)), (((((((-locals.var_temp_dn7) * locals.var_lambdaf) + (assign42530_e47977 * locals.var_lambdaf_dn7)) * locals.var_k1_ac) + (assign42530_e47979 * locals.var_k1_ac_dn7)) * locals.var_sce1_ac) + (assign42530_e47981 * locals.var_sce1_ac_dn7)), (((((((-locals.var_temp_dn8) * locals.var_lambdaf) + (assign42530_e47977 * locals.var_lambdaf_dn8)) * locals.var_k1_ac) + (assign42530_e47979 * locals.var_k1_ac_dn8)) * locals.var_sce1_ac) + (assign42530_e47981 * locals.var_sce1_ac_dn8)), (((((((-locals.var_temp_dn9) * locals.var_lambdaf) + (assign42530_e47977 * locals.var_lambdaf_dn9)) * locals.var_k1_ac) + (assign42530_e47979 * locals.var_k1_ac_dn9)) * locals.var_sce1_ac) + (assign42530_e47981 * locals.var_sce1_ac_dn9)), );
            locals.var_temp1_rv = 0.0;
        }
        if (locals.var_guard1235 != 0.0) {
            let assign42540_e47988: f64 = (-locals.var_temp);
            let assign42540_e47990: f64 = (assign42540_e47988 * locals.var_lambdab);
            let assign42540_e47992: f64 = (assign42540_e47990 * locals.var_k2_ac);
            let assign42540_e47994: f64 = (assign42540_e47992 * locals.var_sce2_ac);
            (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9, ) = (assign42540_e47994, (((((((-locals.var_temp_dn4) * locals.var_lambdab) + (assign42540_e47988 * locals.var_lambdab_dn4)) * locals.var_k2_ac) + (assign42540_e47990 * locals.var_k2_ac_dn4)) * locals.var_sce2_ac) + (assign42540_e47992 * locals.var_sce2_ac_dn4)), (((((((-locals.var_temp_dn6) * locals.var_lambdab) + (assign42540_e47988 * locals.var_lambdab_dn6)) * locals.var_k2_ac) + (assign42540_e47990 * locals.var_k2_ac_dn6)) * locals.var_sce2_ac) + (assign42540_e47992 * locals.var_sce2_ac_dn6)), (((((((-locals.var_temp_dn7) * locals.var_lambdab) + (assign42540_e47988 * locals.var_lambdab_dn7)) * locals.var_k2_ac) + (assign42540_e47990 * locals.var_k2_ac_dn7)) * locals.var_sce2_ac) + (assign42540_e47992 * locals.var_sce2_ac_dn7)), (((((((-locals.var_temp_dn8) * locals.var_lambdab) + (assign42540_e47988 * locals.var_lambdab_dn8)) * locals.var_k2_ac) + (assign42540_e47990 * locals.var_k2_ac_dn8)) * locals.var_sce2_ac) + (assign42540_e47992 * locals.var_sce2_ac_dn8)), (((((((-locals.var_temp_dn9) * locals.var_lambdab) + (assign42540_e47988 * locals.var_lambdab_dn9)) * locals.var_k2_ac) + (assign42540_e47990 * locals.var_k2_ac_dn9)) * locals.var_sce2_ac) + (assign42540_e47992 * locals.var_sce2_ac_dn9)), );
            locals.var_temp2_rv = 0.0;
        }
        if (locals.var_guard1235 != 0.0) {
            let assign42550_e48001: f64 = (locals.var_xedgefs - locals.var_xeffs);
            let assign42550_e48003: f64 = assign42550_e48001;
            let assign42550_e48006: f64 = (locals.var_xedgefs - locals.var_xeffs);
            let assign42550_e48008: f64 = assign42550_e48006;
            let assign42550_e48011: f64 = (locals.var_xedgefs - locals.var_xeffs);
            let assign42550_e48013: f64 = assign42550_e48011;
            let assign42550_e48014: f64 = (assign42550_e48008 * assign42550_e48013);
            let assign42550_e48016: f64 = (assign42550_e48014 + 1.0);
            let assign42550_e48017: f64 = (assign42550_e48016).sqrt();
            let assign42550_e48018: f64 = (assign42550_e48003 + assign42550_e48017);
            let assign42550_e48019: f64 = (0.5 * assign42550_e48018);
            (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign42550_e48019, (0.5 * ((locals.var_xedgefs_dn4 - locals.var_xeffs_dn4) + ((((locals.var_xedgefs_dn4 - locals.var_xeffs_dn4) * assign42550_e48013) + (assign42550_e48008 * (locals.var_xedgefs_dn4 - locals.var_xeffs_dn4))) / (2.0 * assign42550_e48017)))), (0.5 * ((locals.var_xedgefs_dn6 - locals.var_xeffs_dn6) + ((((locals.var_xedgefs_dn6 - locals.var_xeffs_dn6) * assign42550_e48013) + (assign42550_e48008 * (locals.var_xedgefs_dn6 - locals.var_xeffs_dn6))) / (2.0 * assign42550_e48017)))), (0.5 * ((locals.var_xedgefs_dn7 - locals.var_xeffs_dn7) + ((((locals.var_xedgefs_dn7 - locals.var_xeffs_dn7) * assign42550_e48013) + (assign42550_e48008 * (locals.var_xedgefs_dn7 - locals.var_xeffs_dn7))) / (2.0 * assign42550_e48017)))), (0.5 * ((locals.var_xedgefs_dn8 - locals.var_xeffs_dn8) + ((((locals.var_xedgefs_dn8 - locals.var_xeffs_dn8) * assign42550_e48013) + (assign42550_e48008 * (locals.var_xedgefs_dn8 - locals.var_xeffs_dn8))) / (2.0 * assign42550_e48017)))), (0.5 * ((locals.var_xedgefs_dn9 - locals.var_xeffs_dn9) + ((((locals.var_xedgefs_dn9 - locals.var_xeffs_dn9) * assign42550_e48013) + (assign42550_e48008 * (locals.var_xedgefs_dn9 - locals.var_xeffs_dn9))) / (2.0 * assign42550_e48017)))), );
            locals.var_temp_rv = 0.0;
        }
        if (locals.var_guard1235 != 0.0) {
            let assign42560_e48025: f64 = (locals.var_temp1 * locals.var_temp);
            let assign42560_e48027: f64 = (assign42560_e48025 * locals.var_temp);
            let assign42560_e48030: f64 = (locals.var_xedgefs - locals.var_xstars);
            let assign42560_e48031: f64 = (assign42560_e48027 / assign42560_e48030);
            (locals.var_qgsif, locals.var_qgsif_dn4, locals.var_qgsif_dn6, locals.var_qgsif_dn7, locals.var_qgsif_dn8, locals.var_qgsif_dn9, ) = (assign42560_e48031, (((((((locals.var_temp1_dn4 * locals.var_temp) + (locals.var_temp1 * locals.var_temp_dn4)) * locals.var_temp) + (assign42560_e48025 * locals.var_temp_dn4)) * assign42560_e48030) - (assign42560_e48027 * (locals.var_xedgefs_dn4 - locals.var_xstars_dn4))) / (assign42560_e48030 * assign42560_e48030)), (((((((locals.var_temp1_dn6 * locals.var_temp) + (locals.var_temp1 * locals.var_temp_dn6)) * locals.var_temp) + (assign42560_e48025 * locals.var_temp_dn6)) * assign42560_e48030) - (assign42560_e48027 * (locals.var_xedgefs_dn6 - locals.var_xstars_dn6))) / (assign42560_e48030 * assign42560_e48030)), (((((((locals.var_temp1_dn7 * locals.var_temp) + (locals.var_temp1 * locals.var_temp_dn7)) * locals.var_temp) + (assign42560_e48025 * locals.var_temp_dn7)) * assign42560_e48030) - (assign42560_e48027 * (locals.var_xedgefs_dn7 - locals.var_xstars_dn7))) / (assign42560_e48030 * assign42560_e48030)), (((((((locals.var_temp1_dn8 * locals.var_temp) + (locals.var_temp1 * locals.var_temp_dn8)) * locals.var_temp) + (assign42560_e48025 * locals.var_temp_dn8)) * assign42560_e48030) - (assign42560_e48027 * (locals.var_xedgefs_dn8 - locals.var_xstars_dn8))) / (assign42560_e48030 * assign42560_e48030)), (((((((locals.var_temp1_dn9 * locals.var_temp) + (locals.var_temp1 * locals.var_temp_dn9)) * locals.var_temp) + (assign42560_e48025 * locals.var_temp_dn9)) * assign42560_e48030) - (assign42560_e48027 * (locals.var_xedgefs_dn9 - locals.var_xstars_dn9))) / (assign42560_e48030 * assign42560_e48030)), );
            locals.var_qgsif_rv = 0.0;
        }
        if (locals.var_guard1235 != 0.0) {
            let assign42570_e48038: f64 = (locals.var_xedgefd - locals.var_xeffd);
            let assign42570_e48040: f64 = assign42570_e48038;
            let assign42570_e48043: f64 = (locals.var_xedgefd - locals.var_xeffd);
            let assign42570_e48045: f64 = assign42570_e48043;
            let assign42570_e48048: f64 = (locals.var_xedgefd - locals.var_xeffd);
            let assign42570_e48050: f64 = assign42570_e48048;
            let assign42570_e48051: f64 = (assign42570_e48045 * assign42570_e48050);
            let assign42570_e48053: f64 = (assign42570_e48051 + 1.0);
            let assign42570_e48054: f64 = (assign42570_e48053).sqrt();
            let assign42570_e48055: f64 = (assign42570_e48040 + assign42570_e48054);
            let assign42570_e48056: f64 = (0.5 * assign42570_e48055);
            (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign42570_e48056, (0.5 * ((locals.var_xedgefd_dn4 - locals.var_xeffd_dn4) + ((((locals.var_xedgefd_dn4 - locals.var_xeffd_dn4) * assign42570_e48050) + (assign42570_e48045 * (locals.var_xedgefd_dn4 - locals.var_xeffd_dn4))) / (2.0 * assign42570_e48054)))), (0.5 * ((locals.var_xedgefd_dn6 - locals.var_xeffd_dn6) + ((((locals.var_xedgefd_dn6 - locals.var_xeffd_dn6) * assign42570_e48050) + (assign42570_e48045 * (locals.var_xedgefd_dn6 - locals.var_xeffd_dn6))) / (2.0 * assign42570_e48054)))), (0.5 * ((locals.var_xedgefd_dn7 - locals.var_xeffd_dn7) + ((((locals.var_xedgefd_dn7 - locals.var_xeffd_dn7) * assign42570_e48050) + (assign42570_e48045 * (locals.var_xedgefd_dn7 - locals.var_xeffd_dn7))) / (2.0 * assign42570_e48054)))), (0.5 * ((locals.var_xedgefd_dn8 - locals.var_xeffd_dn8) + ((((locals.var_xedgefd_dn8 - locals.var_xeffd_dn8) * assign42570_e48050) + (assign42570_e48045 * (locals.var_xedgefd_dn8 - locals.var_xeffd_dn8))) / (2.0 * assign42570_e48054)))), (0.5 * ((locals.var_xedgefd_dn9 - locals.var_xeffd_dn9) + ((((locals.var_xedgefd_dn9 - locals.var_xeffd_dn9) * assign42570_e48050) + (assign42570_e48045 * (locals.var_xedgefd_dn9 - locals.var_xeffd_dn9))) / (2.0 * assign42570_e48054)))), );
            locals.var_temp_rv = 0.0;
        }
        if (locals.var_guard1235 != 0.0) {
            let assign42580_e48062: f64 = (locals.var_temp1 * locals.var_temp);
            let assign42580_e48064: f64 = (assign42580_e48062 * locals.var_temp);
            let assign42580_e48067: f64 = (locals.var_xedgefd - locals.var_xstard);
            let assign42580_e48068: f64 = (assign42580_e48064 / assign42580_e48067);
            (locals.var_qgdif, locals.var_qgdif_dn4, locals.var_qgdif_dn6, locals.var_qgdif_dn7, locals.var_qgdif_dn8, locals.var_qgdif_dn9, ) = (assign42580_e48068, (((((((locals.var_temp1_dn4 * locals.var_temp) + (locals.var_temp1 * locals.var_temp_dn4)) * locals.var_temp) + (assign42580_e48062 * locals.var_temp_dn4)) * assign42580_e48067) - (assign42580_e48064 * (locals.var_xedgefd_dn4 - locals.var_xstard_dn4))) / (assign42580_e48067 * assign42580_e48067)), (((((((locals.var_temp1_dn6 * locals.var_temp) + (locals.var_temp1 * locals.var_temp_dn6)) * locals.var_temp) + (assign42580_e48062 * locals.var_temp_dn6)) * assign42580_e48067) - (assign42580_e48064 * (locals.var_xedgefd_dn6 - locals.var_xstard_dn6))) / (assign42580_e48067 * assign42580_e48067)), (((((((locals.var_temp1_dn7 * locals.var_temp) + (locals.var_temp1 * locals.var_temp_dn7)) * locals.var_temp) + (assign42580_e48062 * locals.var_temp_dn7)) * assign42580_e48067) - (assign42580_e48064 * (locals.var_xedgefd_dn7 - locals.var_xstard_dn7))) / (assign42580_e48067 * assign42580_e48067)), (((((((locals.var_temp1_dn8 * locals.var_temp) + (locals.var_temp1 * locals.var_temp_dn8)) * locals.var_temp) + (assign42580_e48062 * locals.var_temp_dn8)) * assign42580_e48067) - (assign42580_e48064 * (locals.var_xedgefd_dn8 - locals.var_xstard_dn8))) / (assign42580_e48067 * assign42580_e48067)), (((((((locals.var_temp1_dn9 * locals.var_temp) + (locals.var_temp1 * locals.var_temp_dn9)) * locals.var_temp) + (assign42580_e48062 * locals.var_temp_dn9)) * assign42580_e48067) - (assign42580_e48064 * (locals.var_xedgefd_dn9 - locals.var_xstard_dn9))) / (assign42580_e48067 * assign42580_e48067)), );
            locals.var_qgdif_rv = 0.0;
        }
        if (locals.var_guard1235 != 0.0) {
            let assign42590_e48075: f64 = (locals.var_xedgebs - locals.var_xeffs);
            let assign42590_e48077: f64 = assign42590_e48075;
            let assign42590_e48080: f64 = (locals.var_xedgebs - locals.var_xeffs);
            let assign42590_e48082: f64 = assign42590_e48080;
            let assign42590_e48085: f64 = (locals.var_xedgebs - locals.var_xeffs);
            let assign42590_e48087: f64 = assign42590_e48085;
            let assign42590_e48088: f64 = (assign42590_e48082 * assign42590_e48087);
            let assign42590_e48090: f64 = (assign42590_e48088 + 1.0);
            let assign42590_e48091: f64 = (assign42590_e48090).sqrt();
            let assign42590_e48092: f64 = (assign42590_e48077 + assign42590_e48091);
            let assign42590_e48093: f64 = (0.5 * assign42590_e48092);
            (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign42590_e48093, (0.5 * ((locals.var_xedgebs_dn4 - locals.var_xeffs_dn4) + ((((locals.var_xedgebs_dn4 - locals.var_xeffs_dn4) * assign42590_e48087) + (assign42590_e48082 * (locals.var_xedgebs_dn4 - locals.var_xeffs_dn4))) / (2.0 * assign42590_e48091)))), (0.5 * ((locals.var_xedgebs_dn6 - locals.var_xeffs_dn6) + ((((locals.var_xedgebs_dn6 - locals.var_xeffs_dn6) * assign42590_e48087) + (assign42590_e48082 * (locals.var_xedgebs_dn6 - locals.var_xeffs_dn6))) / (2.0 * assign42590_e48091)))), (0.5 * ((locals.var_xedgebs_dn7 - locals.var_xeffs_dn7) + ((((locals.var_xedgebs_dn7 - locals.var_xeffs_dn7) * assign42590_e48087) + (assign42590_e48082 * (locals.var_xedgebs_dn7 - locals.var_xeffs_dn7))) / (2.0 * assign42590_e48091)))), (0.5 * ((locals.var_xedgebs_dn8 - locals.var_xeffs_dn8) + ((((locals.var_xedgebs_dn8 - locals.var_xeffs_dn8) * assign42590_e48087) + (assign42590_e48082 * (locals.var_xedgebs_dn8 - locals.var_xeffs_dn8))) / (2.0 * assign42590_e48091)))), (0.5 * ((locals.var_xedgebs_dn9 - locals.var_xeffs_dn9) + ((((locals.var_xedgebs_dn9 - locals.var_xeffs_dn9) * assign42590_e48087) + (assign42590_e48082 * (locals.var_xedgebs_dn9 - locals.var_xeffs_dn9))) / (2.0 * assign42590_e48091)))), );
            locals.var_temp_rv = 0.0;
        }
        if (locals.var_guard1235 != 0.0) {
            let assign42600_e48099: f64 = (locals.var_temp2 * locals.var_temp);
            let assign42600_e48101: f64 = (assign42600_e48099 * locals.var_temp);
            let assign42600_e48104: f64 = (locals.var_xedgebs - locals.var_xstars);
            let assign42600_e48105: f64 = (assign42600_e48101 / assign42600_e48104);
            (locals.var_qbsif, locals.var_qbsif_dn4, locals.var_qbsif_dn6, locals.var_qbsif_dn7, locals.var_qbsif_dn8, locals.var_qbsif_dn9, ) = (assign42600_e48105, (((((((locals.var_temp2_dn4 * locals.var_temp) + (locals.var_temp2 * locals.var_temp_dn4)) * locals.var_temp) + (assign42600_e48099 * locals.var_temp_dn4)) * assign42600_e48104) - (assign42600_e48101 * (locals.var_xedgebs_dn4 - locals.var_xstars_dn4))) / (assign42600_e48104 * assign42600_e48104)), (((((((locals.var_temp2_dn6 * locals.var_temp) + (locals.var_temp2 * locals.var_temp_dn6)) * locals.var_temp) + (assign42600_e48099 * locals.var_temp_dn6)) * assign42600_e48104) - (assign42600_e48101 * (locals.var_xedgebs_dn6 - locals.var_xstars_dn6))) / (assign42600_e48104 * assign42600_e48104)), (((((((locals.var_temp2_dn7 * locals.var_temp) + (locals.var_temp2 * locals.var_temp_dn7)) * locals.var_temp) + (assign42600_e48099 * locals.var_temp_dn7)) * assign42600_e48104) - (assign42600_e48101 * (locals.var_xedgebs_dn7 - locals.var_xstars_dn7))) / (assign42600_e48104 * assign42600_e48104)), (((((((locals.var_temp2_dn8 * locals.var_temp) + (locals.var_temp2 * locals.var_temp_dn8)) * locals.var_temp) + (assign42600_e48099 * locals.var_temp_dn8)) * assign42600_e48104) - (assign42600_e48101 * (locals.var_xedgebs_dn8 - locals.var_xstars_dn8))) / (assign42600_e48104 * assign42600_e48104)), (((((((locals.var_temp2_dn9 * locals.var_temp) + (locals.var_temp2 * locals.var_temp_dn9)) * locals.var_temp) + (assign42600_e48099 * locals.var_temp_dn9)) * assign42600_e48104) - (assign42600_e48101 * (locals.var_xedgebs_dn9 - locals.var_xstars_dn9))) / (assign42600_e48104 * assign42600_e48104)), );
            locals.var_qbsif_rv = 0.0;
        }
        if (locals.var_guard1235 != 0.0) {
            let assign42610_e48112: f64 = (locals.var_xedgebd - locals.var_xeffd);
            let assign42610_e48114: f64 = assign42610_e48112;
            let assign42610_e48117: f64 = (locals.var_xedgebd - locals.var_xeffd);
            let assign42610_e48119: f64 = assign42610_e48117;
            let assign42610_e48122: f64 = (locals.var_xedgebd - locals.var_xeffd);
            let assign42610_e48124: f64 = assign42610_e48122;
            let assign42610_e48125: f64 = (assign42610_e48119 * assign42610_e48124);
            let assign42610_e48127: f64 = (assign42610_e48125 + 1.0);
            let assign42610_e48128: f64 = (assign42610_e48127).sqrt();
            let assign42610_e48129: f64 = (assign42610_e48114 + assign42610_e48128);
            let assign42610_e48130: f64 = (0.5 * assign42610_e48129);
            (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign42610_e48130, (0.5 * ((locals.var_xedgebd_dn4 - locals.var_xeffd_dn4) + ((((locals.var_xedgebd_dn4 - locals.var_xeffd_dn4) * assign42610_e48124) + (assign42610_e48119 * (locals.var_xedgebd_dn4 - locals.var_xeffd_dn4))) / (2.0 * assign42610_e48128)))), (0.5 * ((locals.var_xedgebd_dn6 - locals.var_xeffd_dn6) + ((((locals.var_xedgebd_dn6 - locals.var_xeffd_dn6) * assign42610_e48124) + (assign42610_e48119 * (locals.var_xedgebd_dn6 - locals.var_xeffd_dn6))) / (2.0 * assign42610_e48128)))), (0.5 * ((locals.var_xedgebd_dn7 - locals.var_xeffd_dn7) + ((((locals.var_xedgebd_dn7 - locals.var_xeffd_dn7) * assign42610_e48124) + (assign42610_e48119 * (locals.var_xedgebd_dn7 - locals.var_xeffd_dn7))) / (2.0 * assign42610_e48128)))), (0.5 * ((locals.var_xedgebd_dn8 - locals.var_xeffd_dn8) + ((((locals.var_xedgebd_dn8 - locals.var_xeffd_dn8) * assign42610_e48124) + (assign42610_e48119 * (locals.var_xedgebd_dn8 - locals.var_xeffd_dn8))) / (2.0 * assign42610_e48128)))), (0.5 * ((locals.var_xedgebd_dn9 - locals.var_xeffd_dn9) + ((((locals.var_xedgebd_dn9 - locals.var_xeffd_dn9) * assign42610_e48124) + (assign42610_e48119 * (locals.var_xedgebd_dn9 - locals.var_xeffd_dn9))) / (2.0 * assign42610_e48128)))), );
            locals.var_temp_rv = 0.0;
        }
        if (locals.var_guard1235 != 0.0) {
            let assign42620_e48136: f64 = (locals.var_temp2 * locals.var_temp);
            let assign42620_e48138: f64 = (assign42620_e48136 * locals.var_temp);
            let assign42620_e48141: f64 = (locals.var_xedgebd - locals.var_xstard);
            let assign42620_e48142: f64 = (assign42620_e48138 / assign42620_e48141);
            (locals.var_qbdif, locals.var_qbdif_dn4, locals.var_qbdif_dn6, locals.var_qbdif_dn7, locals.var_qbdif_dn8, locals.var_qbdif_dn9, ) = (assign42620_e48142, (((((((locals.var_temp2_dn4 * locals.var_temp) + (locals.var_temp2 * locals.var_temp_dn4)) * locals.var_temp) + (assign42620_e48136 * locals.var_temp_dn4)) * assign42620_e48141) - (assign42620_e48138 * (locals.var_xedgebd_dn4 - locals.var_xstard_dn4))) / (assign42620_e48141 * assign42620_e48141)), (((((((locals.var_temp2_dn6 * locals.var_temp) + (locals.var_temp2 * locals.var_temp_dn6)) * locals.var_temp) + (assign42620_e48136 * locals.var_temp_dn6)) * assign42620_e48141) - (assign42620_e48138 * (locals.var_xedgebd_dn6 - locals.var_xstard_dn6))) / (assign42620_e48141 * assign42620_e48141)), (((((((locals.var_temp2_dn7 * locals.var_temp) + (locals.var_temp2 * locals.var_temp_dn7)) * locals.var_temp) + (assign42620_e48136 * locals.var_temp_dn7)) * assign42620_e48141) - (assign42620_e48138 * (locals.var_xedgebd_dn7 - locals.var_xstard_dn7))) / (assign42620_e48141 * assign42620_e48141)), (((((((locals.var_temp2_dn8 * locals.var_temp) + (locals.var_temp2 * locals.var_temp_dn8)) * locals.var_temp) + (assign42620_e48136 * locals.var_temp_dn8)) * assign42620_e48141) - (assign42620_e48138 * (locals.var_xedgebd_dn8 - locals.var_xstard_dn8))) / (assign42620_e48141 * assign42620_e48141)), (((((((locals.var_temp2_dn9 * locals.var_temp) + (locals.var_temp2 * locals.var_temp_dn9)) * locals.var_temp) + (assign42620_e48136 * locals.var_temp_dn9)) * assign42620_e48141) - (assign42620_e48138 * (locals.var_xedgebd_dn9 - locals.var_xstard_dn9))) / (assign42620_e48141 * assign42620_e48141)), );
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
        let assign42670_e48167: f64 = (locals.var_cfr_i * locals.var_vgsu);
        (locals.var_qgse, locals.var_qgse_dn4, locals.var_qgse_dn6, locals.var_qgse_dn7, locals.var_qgse_dn8, locals.var_qgse_dn9, ) = (assign42670_e48167, (locals.var_cfr_i_dn4 * locals.var_vgsu), ((locals.var_cfr_i_dn6 * locals.var_vgsu) + (locals.var_cfr_i * locals.var_vgsu_dn6)), (locals.var_cfr_i_dn7 * locals.var_vgsu), (locals.var_cfr_i_dn8 * locals.var_vgsu), ((locals.var_cfr_i_dn9 * locals.var_vgsu) + (locals.var_cfr_i * locals.var_vgsu_dn9)), );
        locals.var_qgse_rv = 0.0;
        let assign42680_e48170: f64 = (locals.var_cfrd_i * locals.var_vgdu);
        (locals.var_qgde, locals.var_qgde_dn4, locals.var_qgde_dn6, locals.var_qgde_dn7, locals.var_qgde_dn8, locals.var_qgde_dn9, ) = (assign42680_e48170, (locals.var_cfrd_i_dn4 * locals.var_vgdu), ((locals.var_cfrd_i_dn6 * locals.var_vgdu) + (locals.var_cfrd_i * locals.var_vgdu_dn6)), ((locals.var_cfrd_i_dn7 * locals.var_vgdu) + (locals.var_cfrd_i * locals.var_vgdu_dn7)), (locals.var_cfrd_i_dn8 * locals.var_vgdu), ((locals.var_cfrd_i_dn9 * locals.var_vgdu) + (locals.var_cfrd_i * locals.var_vgdu_dn9)), );
        locals.var_qgde_rv = 0.0;
        let assign42690_e48175: f64 = (locals.var_covdl_i * locals.var_dleff_ac);
        let assign42690_e48179: f64 = (locals.var_covdlb_i * locals.var_xg20shift_ac);
        let assign42690_e48180: f64 = (1.0 - assign42690_e48179);
        let assign42690_e48181: f64 = (assign42690_e48175 * assign42690_e48180);
        let assign42690_e48182: f64 = (1.0 - assign42690_e48181);
        let assign42690_e48184: f64 = assign42690_e48182;
        let assign42690_e48188: f64 = (locals.var_covdl_i * locals.var_dleff_ac);
        let assign42690_e48192: f64 = (locals.var_covdlb_i * locals.var_xg20shift_ac);
        let assign42690_e48193: f64 = (1.0 - assign42690_e48192);
        let assign42690_e48194: f64 = (assign42690_e48188 * assign42690_e48193);
        let assign42690_e48195: f64 = (1.0 - assign42690_e48194);
        let assign42690_e48197: f64 = assign42690_e48195;
        let assign42690_e48201: f64 = (locals.var_covdl_i * locals.var_dleff_ac);
        let assign42690_e48205: f64 = (locals.var_covdlb_i * locals.var_xg20shift_ac);
        let assign42690_e48206: f64 = (1.0 - assign42690_e48205);
        let assign42690_e48207: f64 = (assign42690_e48201 * assign42690_e48206);
        let assign42690_e48208: f64 = (1.0 - assign42690_e48207);
        let assign42690_e48210: f64 = assign42690_e48208;
        let assign42690_e48211: f64 = (assign42690_e48197 * assign42690_e48210);
        let assign42690_e48213: f64 = (assign42690_e48211 + 0.2);
        let assign42690_e48214: f64 = (assign42690_e48213).sqrt();
        let assign42690_e48215: f64 = (assign42690_e48184 + assign42690_e48214);
        let assign42690_e48216: f64 = (0.5 * assign42690_e48215);
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign42690_e48216, (0.5 * ((-(((locals.var_covdl_i * locals.var_dleff_ac_dn4) * assign42690_e48180) + (assign42690_e48175 * (-(locals.var_covdlb_i * locals.var_xg20shift_ac_dn4))))) + ((((-(((locals.var_covdl_i * locals.var_dleff_ac_dn4) * assign42690_e48193) + (assign42690_e48188 * (-(locals.var_covdlb_i * locals.var_xg20shift_ac_dn4))))) * assign42690_e48210) + (assign42690_e48197 * (-(((locals.var_covdl_i * locals.var_dleff_ac_dn4) * assign42690_e48206) + (assign42690_e48201 * (-(locals.var_covdlb_i * locals.var_xg20shift_ac_dn4))))))) / (2.0 * assign42690_e48214)))), (0.5 * ((-(((locals.var_covdl_i * locals.var_dleff_ac_dn6) * assign42690_e48180) + (assign42690_e48175 * (-(locals.var_covdlb_i * locals.var_xg20shift_ac_dn6))))) + ((((-(((locals.var_covdl_i * locals.var_dleff_ac_dn6) * assign42690_e48193) + (assign42690_e48188 * (-(locals.var_covdlb_i * locals.var_xg20shift_ac_dn6))))) * assign42690_e48210) + (assign42690_e48197 * (-(((locals.var_covdl_i * locals.var_dleff_ac_dn6) * assign42690_e48206) + (assign42690_e48201 * (-(locals.var_covdlb_i * locals.var_xg20shift_ac_dn6))))))) / (2.0 * assign42690_e48214)))), (0.5 * ((-(((locals.var_covdl_i * locals.var_dleff_ac_dn7) * assign42690_e48180) + (assign42690_e48175 * (-(locals.var_covdlb_i * locals.var_xg20shift_ac_dn7))))) + ((((-(((locals.var_covdl_i * locals.var_dleff_ac_dn7) * assign42690_e48193) + (assign42690_e48188 * (-(locals.var_covdlb_i * locals.var_xg20shift_ac_dn7))))) * assign42690_e48210) + (assign42690_e48197 * (-(((locals.var_covdl_i * locals.var_dleff_ac_dn7) * assign42690_e48206) + (assign42690_e48201 * (-(locals.var_covdlb_i * locals.var_xg20shift_ac_dn7))))))) / (2.0 * assign42690_e48214)))), (0.5 * ((-(((locals.var_covdl_i * locals.var_dleff_ac_dn8) * assign42690_e48180) + (assign42690_e48175 * (-(locals.var_covdlb_i * locals.var_xg20shift_ac_dn8))))) + ((((-(((locals.var_covdl_i * locals.var_dleff_ac_dn8) * assign42690_e48193) + (assign42690_e48188 * (-(locals.var_covdlb_i * locals.var_xg20shift_ac_dn8))))) * assign42690_e48210) + (assign42690_e48197 * (-(((locals.var_covdl_i * locals.var_dleff_ac_dn8) * assign42690_e48206) + (assign42690_e48201 * (-(locals.var_covdlb_i * locals.var_xg20shift_ac_dn8))))))) / (2.0 * assign42690_e48214)))), (0.5 * ((-(((locals.var_covdl_i * locals.var_dleff_ac_dn9) * assign42690_e48180) + (assign42690_e48175 * (-(locals.var_covdlb_i * locals.var_xg20shift_ac_dn9))))) + ((((-(((locals.var_covdl_i * locals.var_dleff_ac_dn9) * assign42690_e48193) + (assign42690_e48188 * (-(locals.var_covdlb_i * locals.var_xg20shift_ac_dn9))))) * assign42690_e48210) + (assign42690_e48197 * (-(((locals.var_covdl_i * locals.var_dleff_ac_dn9) * assign42690_e48206) + (assign42690_e48201 * (-(locals.var_covdlb_i * locals.var_xg20shift_ac_dn9))))))) / (2.0 * assign42690_e48214)))), );
        locals.var_temp_rv = 0.0;
        let assign42700_e48219: f64 = (locals.var_cov_i * locals.var_vovscv);
        let assign42700_e48221: f64 = (assign42700_e48219 * locals.var_temp);
        (locals.var_qovs, locals.var_qovs_dn4, locals.var_qovs_dn6, locals.var_qovs_dn7, locals.var_qovs_dn8, locals.var_qovs_dn9, ) = (assign42700_e48221, ((((locals.var_cov_i_dn4 * locals.var_vovscv) + (locals.var_cov_i * locals.var_vovscv_dn4)) * locals.var_temp) + (assign42700_e48219 * locals.var_temp_dn4)), ((((locals.var_cov_i_dn6 * locals.var_vovscv) + (locals.var_cov_i * locals.var_vovscv_dn6)) * locals.var_temp) + (assign42700_e48219 * locals.var_temp_dn6)), ((((locals.var_cov_i_dn7 * locals.var_vovscv) + (locals.var_cov_i * locals.var_vovscv_dn7)) * locals.var_temp) + (assign42700_e48219 * locals.var_temp_dn7)), ((((locals.var_cov_i_dn8 * locals.var_vovscv) + (locals.var_cov_i * locals.var_vovscv_dn8)) * locals.var_temp) + (assign42700_e48219 * locals.var_temp_dn8)), ((((locals.var_cov_i_dn9 * locals.var_vovscv) + (locals.var_cov_i * locals.var_vovscv_dn9)) * locals.var_temp) + (assign42700_e48219 * locals.var_temp_dn9)), );
        locals.var_qovs_rv = 0.0;
        let assign42710_e48224: f64 = (locals.var_covd_i * locals.var_vovdcv);
        let assign42710_e48226: f64 = (assign42710_e48224 * locals.var_temp);
        (locals.var_qovd, locals.var_qovd_dn4, locals.var_qovd_dn6, locals.var_qovd_dn7, locals.var_qovd_dn8, locals.var_qovd_dn9, ) = (assign42710_e48226, ((((locals.var_covd_i_dn4 * locals.var_vovdcv) + (locals.var_covd_i * locals.var_vovdcv_dn4)) * locals.var_temp) + (assign42710_e48224 * locals.var_temp_dn4)), ((((locals.var_covd_i_dn6 * locals.var_vovdcv) + (locals.var_covd_i * locals.var_vovdcv_dn6)) * locals.var_temp) + (assign42710_e48224 * locals.var_temp_dn6)), ((((locals.var_covd_i_dn7 * locals.var_vovdcv) + (locals.var_covd_i * locals.var_vovdcv_dn7)) * locals.var_temp) + (assign42710_e48224 * locals.var_temp_dn7)), ((((locals.var_covd_i_dn8 * locals.var_vovdcv) + (locals.var_covd_i * locals.var_vovdcv_dn8)) * locals.var_temp) + (assign42710_e48224 * locals.var_temp_dn8)), ((((locals.var_covd_i_dn9 * locals.var_vovdcv) + (locals.var_covd_i * locals.var_vovdcv_dn9)) * locals.var_temp) + (assign42710_e48224 * locals.var_temp_dn9)), );
        locals.var_qovd_rv = 0.0;
        let assign42720_e48229: f64 = (locals.var_cgbov_i * locals.var_vgb);
        (locals.var_qgbe, locals.var_qgbe_dn4, locals.var_qgbe_dn6, locals.var_qgbe_dn7, locals.var_qgbe_dn8, locals.var_qgbe_dn9, ) = (assign42720_e48229, (locals.var_cgbov_i_dn4 * locals.var_vgb), ((locals.var_cgbov_i_dn6 * locals.var_vgb) + (locals.var_cgbov_i * locals.var_vgb_dn6)), ((locals.var_cgbov_i_dn7 * locals.var_vgb) + (locals.var_cgbov_i * locals.var_vgb_dn7)), ((locals.var_cgbov_i_dn8 * locals.var_vgb) + (locals.var_cgbov_i * locals.var_vgb_dn8)), ((locals.var_cgbov_i_dn9 * locals.var_vgb) + (locals.var_cgbov_i * locals.var_vgb_dn9)), );
        locals.var_qgbe_rv = 0.0;
        let assign42730_e48232: f64 = (locals.var_csd_i * locals.var_vds);
        (locals.var_qdse, locals.var_qdse_dn6, locals.var_qdse_dn7, ) = (assign42730_e48232, (locals.var_csd_i * locals.var_vds_dn6), (locals.var_csd_i * locals.var_vds_dn7), );
        locals.var_qdse_rv = 0.0;
        let assign42740_e48235: f64 = (locals.var_cox2init * locals.var_asource_i);
        let assign42740_e48238: f64 = (locals.var_csdbp_i * locals.var_psource_i);
        let assign42740_e48239: f64 = (assign42740_e48235 + assign42740_e48238);
        let assign42740_e48240: f64 = (-assign42740_e48239);
        let assign42740_e48242: f64 = (assign42740_e48240 * locals.var_vsbu);
        (locals.var_qssub, locals.var_qssub_dn6, locals.var_qssub_dn8, ) = (assign42740_e48242, (assign42740_e48240 * locals.var_vsbu_dn6), (assign42740_e48240 * locals.var_vsbu_dn8), );
        locals.var_qssub_rv = 0.0;
        let assign42750_e48245: f64 = (locals.var_cox2init * locals.var_adrain_i);
        let assign42750_e48248: f64 = (locals.var_csdbp_i * locals.var_pdrain_i);
        let assign42750_e48249: f64 = (assign42750_e48245 + assign42750_e48248);
        let assign42750_e48250: f64 = (-assign42750_e48249);
        let assign42750_e48252: f64 = (assign42750_e48250 * locals.var_vdbu);
        (locals.var_qdsub, locals.var_qdsub_dn6, locals.var_qdsub_dn7, locals.var_qdsub_dn8, ) = (assign42750_e48252, (assign42750_e48250 * locals.var_vdbu_dn6), (assign42750_e48250 * locals.var_vdbu_dn7), (assign42750_e48250 * locals.var_vdbu_dn8), );
        locals.var_qdsub_rv = 0.0;
        let assign42760_e48255: f64 = if locals.var_swshe_i > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1236 = assign42760_e48255;
        locals.var_guard1236_rv = 0.0;
        if (locals.var_guard1236 != 0.0) {
            let assign42770_e48259: f64 = (locals.var_cth_i * locals.var_dtc);
            (locals.var_qth, locals.var_qth_dn4, locals.var_qth_dn6, locals.var_qth_dn7, locals.var_qth_dn8, locals.var_qth_dn9, ) = (assign42770_e48259, ((locals.var_cth_i_dn4 * locals.var_dtc) + (locals.var_cth_i * locals.var_dtc_dn4)), (locals.var_cth_i_dn6 * locals.var_dtc), (locals.var_cth_i_dn7 * locals.var_dtc), (locals.var_cth_i_dn8 * locals.var_dtc), (locals.var_cth_i_dn9 * locals.var_dtc), );
            locals.var_qth_rv = 0.0;
        }
        if (locals.var_guard1236 == 0.0) {
            (locals.var_qth, locals.var_qth_dn4, locals.var_qth_dn6, locals.var_qth_dn7, locals.var_qth_dn8, locals.var_qth_dn9, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_qth_rv = 0.0;
        }
        let assign42910_e48319: f64 = (p.p32 * locals.var_mult_i_int);
        let assign42910_e48321: f64 = (assign42910_e48319 * locals.var_qg);
        (locals.var_qg, locals.var_qg_dn4, locals.var_qg_dn6, locals.var_qg_dn7, locals.var_qg_dn8, locals.var_qg_dn9, ) = (assign42910_e48321, (assign42910_e48319 * locals.var_qg_dn4), (assign42910_e48319 * locals.var_qg_dn6), (assign42910_e48319 * locals.var_qg_dn7), (assign42910_e48319 * locals.var_qg_dn8), (assign42910_e48319 * locals.var_qg_dn9), );
        locals.var_qg_rv = 0.0;
        let assign42920_e48324: f64 = (p.p32 * locals.var_mult_i_int);
        let assign42920_e48326: f64 = (assign42920_e48324 * locals.var_qb);
        (locals.var_qb, locals.var_qb_dn4, locals.var_qb_dn6, locals.var_qb_dn7, locals.var_qb_dn8, locals.var_qb_dn9, ) = (assign42920_e48326, (assign42920_e48324 * locals.var_qb_dn4), (assign42920_e48324 * locals.var_qb_dn6), (assign42920_e48324 * locals.var_qb_dn7), (assign42920_e48324 * locals.var_qb_dn8), (assign42920_e48324 * locals.var_qb_dn9), );
        locals.var_qb_rv = 0.0;
        let assign42930_e48329: f64 = (p.p32 * locals.var_mult_i_int);
        let assign42930_e48331: f64 = (assign42930_e48329 * locals.var_qd);
        (locals.var_qd, locals.var_qd_dn4, locals.var_qd_dn6, locals.var_qd_dn7, locals.var_qd_dn8, locals.var_qd_dn9, ) = (assign42930_e48331, (assign42930_e48329 * locals.var_qd_dn4), (assign42930_e48329 * locals.var_qd_dn6), (assign42930_e48329 * locals.var_qd_dn7), (assign42930_e48329 * locals.var_qd_dn8), (assign42930_e48329 * locals.var_qd_dn9), );
        locals.var_qd_rv = 0.0;
        let assign42940_e48334: f64 = (locals.var_qg + locals.var_qb);
        let assign42940_e48336: f64 = (assign42940_e48334 + locals.var_qd);
        let assign42940_e48337: f64 = (-assign42940_e48336);
        (locals.var_qs, locals.var_qs_dn4, locals.var_qs_dn6, locals.var_qs_dn7, locals.var_qs_dn8, locals.var_qs_dn9, ) = (assign42940_e48337, (-((locals.var_qg_dn4 + locals.var_qb_dn4) + locals.var_qd_dn4)), (-((locals.var_qg_dn6 + locals.var_qb_dn6) + locals.var_qd_dn6)), (-((locals.var_qg_dn7 + locals.var_qb_dn7) + locals.var_qd_dn7)), (-((locals.var_qg_dn8 + locals.var_qb_dn8) + locals.var_qd_dn8)), (-((locals.var_qg_dn9 + locals.var_qb_dn9) + locals.var_qd_dn9)), );
        locals.var_qs_rv = 0.0;
        let assign42950_e48340: f64 = (p.p32 * locals.var_mult_i_int);
        let assign42950_e48342: f64 = (assign42950_e48340 * locals.var_qgsif);
        (locals.var_qgsif, locals.var_qgsif_dn4, locals.var_qgsif_dn6, locals.var_qgsif_dn7, locals.var_qgsif_dn8, locals.var_qgsif_dn9, ) = (assign42950_e48342, (assign42950_e48340 * locals.var_qgsif_dn4), (assign42950_e48340 * locals.var_qgsif_dn6), (assign42950_e48340 * locals.var_qgsif_dn7), (assign42950_e48340 * locals.var_qgsif_dn8), (assign42950_e48340 * locals.var_qgsif_dn9), );
        locals.var_qgsif_rv = 0.0;
        let assign42960_e48345: f64 = (p.p32 * locals.var_mult_i_int);
        let assign42960_e48347: f64 = (assign42960_e48345 * locals.var_qgdif);
        (locals.var_qgdif, locals.var_qgdif_dn4, locals.var_qgdif_dn6, locals.var_qgdif_dn7, locals.var_qgdif_dn8, locals.var_qgdif_dn9, ) = (assign42960_e48347, (assign42960_e48345 * locals.var_qgdif_dn4), (assign42960_e48345 * locals.var_qgdif_dn6), (assign42960_e48345 * locals.var_qgdif_dn7), (assign42960_e48345 * locals.var_qgdif_dn8), (assign42960_e48345 * locals.var_qgdif_dn9), );
        locals.var_qgdif_rv = 0.0;
        let assign42970_e48350: f64 = (p.p32 * locals.var_mult_i_int);
        let assign42970_e48352: f64 = (assign42970_e48350 * locals.var_qbsif);
        (locals.var_qbsif, locals.var_qbsif_dn4, locals.var_qbsif_dn6, locals.var_qbsif_dn7, locals.var_qbsif_dn8, locals.var_qbsif_dn9, ) = (assign42970_e48352, (assign42970_e48350 * locals.var_qbsif_dn4), (assign42970_e48350 * locals.var_qbsif_dn6), (assign42970_e48350 * locals.var_qbsif_dn7), (assign42970_e48350 * locals.var_qbsif_dn8), (assign42970_e48350 * locals.var_qbsif_dn9), );
        locals.var_qbsif_rv = 0.0;
        let assign42980_e48355: f64 = (p.p32 * locals.var_mult_i_int);
        let assign42980_e48357: f64 = (assign42980_e48355 * locals.var_qbdif);
        (locals.var_qbdif, locals.var_qbdif_dn4, locals.var_qbdif_dn6, locals.var_qbdif_dn7, locals.var_qbdif_dn8, locals.var_qbdif_dn9, ) = (assign42980_e48357, (assign42980_e48355 * locals.var_qbdif_dn4), (assign42980_e48355 * locals.var_qbdif_dn6), (assign42980_e48355 * locals.var_qbdif_dn7), (assign42980_e48355 * locals.var_qbdif_dn8), (assign42980_e48355 * locals.var_qbdif_dn9), );
        locals.var_qbdif_rv = 0.0;
        let assign42990_e48360: f64 = (p.p32 * locals.var_mult_i_int);
        let assign42990_e48362: f64 = (assign42990_e48360 * locals.var_qgse);
        (locals.var_qgse, locals.var_qgse_dn4, locals.var_qgse_dn6, locals.var_qgse_dn7, locals.var_qgse_dn8, locals.var_qgse_dn9, ) = (assign42990_e48362, (assign42990_e48360 * locals.var_qgse_dn4), (assign42990_e48360 * locals.var_qgse_dn6), (assign42990_e48360 * locals.var_qgse_dn7), (assign42990_e48360 * locals.var_qgse_dn8), (assign42990_e48360 * locals.var_qgse_dn9), );
        locals.var_qgse_rv = 0.0;
        let assign43000_e48365: f64 = (p.p32 * locals.var_mult_i_int);
        let assign43000_e48367: f64 = (assign43000_e48365 * locals.var_qgde);
        (locals.var_qgde, locals.var_qgde_dn4, locals.var_qgde_dn6, locals.var_qgde_dn7, locals.var_qgde_dn8, locals.var_qgde_dn9, ) = (assign43000_e48367, (assign43000_e48365 * locals.var_qgde_dn4), (assign43000_e48365 * locals.var_qgde_dn6), (assign43000_e48365 * locals.var_qgde_dn7), (assign43000_e48365 * locals.var_qgde_dn8), (assign43000_e48365 * locals.var_qgde_dn9), );
        locals.var_qgde_rv = 0.0;
        let assign43010_e48370: f64 = (p.p32 * locals.var_mult_i_int);
        let assign43010_e48372: f64 = (assign43010_e48370 * locals.var_qovs);
        (locals.var_qovs, locals.var_qovs_dn4, locals.var_qovs_dn6, locals.var_qovs_dn7, locals.var_qovs_dn8, locals.var_qovs_dn9, ) = (assign43010_e48372, (assign43010_e48370 * locals.var_qovs_dn4), (assign43010_e48370 * locals.var_qovs_dn6), (assign43010_e48370 * locals.var_qovs_dn7), (assign43010_e48370 * locals.var_qovs_dn8), (assign43010_e48370 * locals.var_qovs_dn9), );
        locals.var_qovs_rv = 0.0;
        let assign43020_e48375: f64 = (p.p32 * locals.var_mult_i_int);
        let assign43020_e48377: f64 = (assign43020_e48375 * locals.var_qovd);
        (locals.var_qovd, locals.var_qovd_dn4, locals.var_qovd_dn6, locals.var_qovd_dn7, locals.var_qovd_dn8, locals.var_qovd_dn9, ) = (assign43020_e48377, (assign43020_e48375 * locals.var_qovd_dn4), (assign43020_e48375 * locals.var_qovd_dn6), (assign43020_e48375 * locals.var_qovd_dn7), (assign43020_e48375 * locals.var_qovd_dn8), (assign43020_e48375 * locals.var_qovd_dn9), );
        locals.var_qovd_rv = 0.0;
        let assign43030_e48380: f64 = (p.p32 * locals.var_mult_i_int);
        let assign43030_e48382: f64 = (assign43030_e48380 * locals.var_qgbe);
        (locals.var_qgbe, locals.var_qgbe_dn4, locals.var_qgbe_dn6, locals.var_qgbe_dn7, locals.var_qgbe_dn8, locals.var_qgbe_dn9, ) = (assign43030_e48382, (assign43030_e48380 * locals.var_qgbe_dn4), (assign43030_e48380 * locals.var_qgbe_dn6), (assign43030_e48380 * locals.var_qgbe_dn7), (assign43030_e48380 * locals.var_qgbe_dn8), (assign43030_e48380 * locals.var_qgbe_dn9), );
        locals.var_qgbe_rv = 0.0;
        let assign43040_e48385: f64 = (p.p32 * locals.var_mult_i_int);
        let assign43040_e48387: f64 = (assign43040_e48385 * locals.var_qssub);
        (locals.var_qssub, locals.var_qssub_dn6, locals.var_qssub_dn8, ) = (assign43040_e48387, (assign43040_e48385 * locals.var_qssub_dn6), (assign43040_e48385 * locals.var_qssub_dn8), );
        locals.var_qssub_rv = 0.0;
        let assign43050_e48390: f64 = (p.p32 * locals.var_mult_i_int);
        let assign43050_e48392: f64 = (assign43050_e48390 * locals.var_qdsub);
        (locals.var_qdsub, locals.var_qdsub_dn6, locals.var_qdsub_dn7, locals.var_qdsub_dn8, ) = (assign43050_e48392, (assign43050_e48390 * locals.var_qdsub_dn6), (assign43050_e48390 * locals.var_qdsub_dn7), (assign43050_e48390 * locals.var_qdsub_dn8), );
        locals.var_qdsub_rv = 0.0;
        let assign43060_e48395: f64 = (p.p32 * locals.var_mult_i_int);
        let assign43060_e48397: f64 = (assign43060_e48395 * locals.var_qdse);
        (locals.var_qdse, locals.var_qdse_dn6, locals.var_qdse_dn7, ) = (assign43060_e48397, (assign43060_e48395 * locals.var_qdse_dn6), (assign43060_e48395 * locals.var_qdse_dn7), );
        locals.var_qdse_rv = 0.0;
        let assign43070_e48400: f64 = (locals.var_mult_i_int * locals.var_qth);
        (locals.var_qth, locals.var_qth_dn4, locals.var_qth_dn6, locals.var_qth_dn7, locals.var_qth_dn8, locals.var_qth_dn9, ) = (assign43070_e48400, (locals.var_mult_i_int * locals.var_qth_dn4), (locals.var_mult_i_int * locals.var_qth_dn6), (locals.var_mult_i_int * locals.var_qth_dn7), (locals.var_mult_i_int * locals.var_qth_dn8), (locals.var_mult_i_int * locals.var_qth_dn9), );
        locals.var_qth_rv = 0.0;
        let assign43080_e48403: f64 = if locals.var_sigvds < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1245 = assign43080_e48403;
        locals.var_guard1245_rv = 0.0;
        if (locals.var_guard1245 != 0.0) {
            (locals.var_temp_q, locals.var_temp_q_dn4, locals.var_temp_q_dn6, locals.var_temp_q_dn7, locals.var_temp_q_dn8, locals.var_temp_q_dn9, ) = (locals.var_qd, locals.var_qd_dn4, locals.var_qd_dn6, locals.var_qd_dn7, locals.var_qd_dn8, locals.var_qd_dn9, );
            locals.var_temp_q_rv = 0.0;
            (locals.var_qd, locals.var_qd_dn4, locals.var_qd_dn6, locals.var_qd_dn7, locals.var_qd_dn8, locals.var_qd_dn9, ) = (locals.var_qs, locals.var_qs_dn4, locals.var_qs_dn6, locals.var_qs_dn7, locals.var_qs_dn8, locals.var_qs_dn9, );
            locals.var_qd_rv = 0.0;
            (locals.var_qs, locals.var_qs_dn4, locals.var_qs_dn6, locals.var_qs_dn7, locals.var_qs_dn8, locals.var_qs_dn9, ) = (locals.var_temp_q, locals.var_temp_q_dn4, locals.var_temp_q_dn6, locals.var_temp_q_dn7, locals.var_temp_q_dn8, locals.var_temp_q_dn9, );
            locals.var_qs_rv = 0.0;
        }
        if (locals.var_guard1245 != 0.0) {
            let assign43120_e48418: f64 = (-locals.var_qdse);
            (locals.var_qdse, locals.var_qdse_dn6, locals.var_qdse_dn7, ) = (assign43120_e48418, (-locals.var_qdse_dn6), (-locals.var_qdse_dn7), );
            locals.var_qdse_rv = 0.0;
        }
        if (locals.var_guard1245 != 0.0) {
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
        let assign43190_e48447: f64 = (locals.var_csiprime_dc / 1.602176565e-19);
        let assign43190_e48449: f64 = (assign43190_e48447 * locals.var_phit);
        (locals.var_nunit, locals.var_nunit_dn4, locals.var_nunit_dn6, locals.var_nunit_dn7, locals.var_nunit_dn8, locals.var_nunit_dn9, ) = (assign43190_e48449, (((locals.var_csiprime_dc_dn4 / 1.602176565e-19) * locals.var_phit) + (assign43190_e48447 * locals.var_phit_dn4)), (((locals.var_csiprime_dc_dn6 / 1.602176565e-19) * locals.var_phit) + (assign43190_e48447 * locals.var_phit_dn6)), (((locals.var_csiprime_dc_dn7 / 1.602176565e-19) * locals.var_phit) + (assign43190_e48447 * locals.var_phit_dn7)), (((locals.var_csiprime_dc_dn8 / 1.602176565e-19) * locals.var_phit) + (assign43190_e48447 * locals.var_phit_dn8)), (((locals.var_csiprime_dc_dn9 / 1.602176565e-19) * locals.var_phit) + (assign43190_e48447 * locals.var_phit_dn9)), );
        locals.var_nunit_rv = 0.0;
        let assign43200_e48451: f64 = (-0.5);
        let assign43200_e48454: f64 = (locals.var_ds_dc + locals.var_dd_dc);
        let assign43200_e48455: f64 = (assign43200_e48451 * assign43200_e48454);
        (locals.var_dm, locals.var_dm_dn4, locals.var_dm_dn6, locals.var_dm_dn7, locals.var_dm_dn8, locals.var_dm_dn9, ) = (assign43200_e48455, (assign43200_e48451 * (locals.var_ds_dc_dn4 + locals.var_dd_dc_dn4)), (assign43200_e48451 * (locals.var_ds_dc_dn6 + locals.var_dd_dc_dn6)), (assign43200_e48451 * (locals.var_ds_dc_dn7 + locals.var_dd_dc_dn7)), (assign43200_e48451 * (locals.var_ds_dc_dn8 + locals.var_dd_dc_dn8)), (assign43200_e48451 * (locals.var_ds_dc_dn9 + locals.var_dd_dc_dn9)), );
        locals.var_dm_rv = 0.0;
        let assign43210_e48458: f64 = (locals.var_qim_dc + locals.var_dm);
        (locals.var_qimstar, locals.var_qimstar_dn4, locals.var_qimstar_dn6, locals.var_qimstar_dn7, locals.var_qimstar_dn8, locals.var_qimstar_dn9, ) = (assign43210_e48458, (locals.var_qim_dc_dn4 + locals.var_dm_dn4), (locals.var_qim_dc_dn6 + locals.var_dm_dn6), (locals.var_qim_dc_dn7 + locals.var_dm_dn7), (locals.var_qim_dc_dn8 + locals.var_dm_dn8), (locals.var_qim_dc_dn9 + locals.var_dm_dn9), );
        locals.var_qimstar_rv = 0.0;
        let assign43220_e48461: f64 = (locals.var_qim_dc / locals.var_qimstar);
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign43220_e48461, (((locals.var_qim_dc_dn4 * locals.var_qimstar) - (locals.var_qim_dc * locals.var_qimstar_dn4)) / (locals.var_qimstar * locals.var_qimstar)), (((locals.var_qim_dc_dn6 * locals.var_qimstar) - (locals.var_qim_dc * locals.var_qimstar_dn6)) / (locals.var_qimstar * locals.var_qimstar)), (((locals.var_qim_dc_dn7 * locals.var_qimstar) - (locals.var_qim_dc * locals.var_qimstar_dn7)) / (locals.var_qimstar * locals.var_qimstar)), (((locals.var_qim_dc_dn8 * locals.var_qimstar) - (locals.var_qim_dc * locals.var_qimstar_dn8)) / (locals.var_qimstar * locals.var_qimstar)), (((locals.var_qim_dc_dn9 * locals.var_qimstar) - (locals.var_qim_dc * locals.var_qimstar_dn9)) / (locals.var_qimstar * locals.var_qimstar)), );
        locals.var_temp_rv = 0.0;
        let assign43230_e48465: f64 = locals.var_temp;
        let assign43230_e48468: f64 = locals.var_temp;
        let assign43230_e48471: f64 = locals.var_temp;
        let assign43230_e48472: f64 = (assign43230_e48468 * assign43230_e48471);
        let assign43230_e48474: f64 = (assign43230_e48472 + 1e-20);
        let assign43230_e48475: f64 = (assign43230_e48474).sqrt();
        let assign43230_e48476: f64 = (assign43230_e48465 + assign43230_e48475);
        let assign43230_e48477: f64 = (0.5 * assign43230_e48476);
        (locals.var_t1, locals.var_t1_dn4, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, ) = (assign43230_e48477, (0.5 * (locals.var_temp_dn4 + (((locals.var_temp_dn4 * assign43230_e48471) + (assign43230_e48468 * locals.var_temp_dn4)) / (2.0 * assign43230_e48475)))), (0.5 * (locals.var_temp_dn6 + (((locals.var_temp_dn6 * assign43230_e48471) + (assign43230_e48468 * locals.var_temp_dn6)) / (2.0 * assign43230_e48475)))), (0.5 * (locals.var_temp_dn7 + (((locals.var_temp_dn7 * assign43230_e48471) + (assign43230_e48468 * locals.var_temp_dn7)) / (2.0 * assign43230_e48475)))), (0.5 * (locals.var_temp_dn8 + (((locals.var_temp_dn8 * assign43230_e48471) + (assign43230_e48468 * locals.var_temp_dn8)) / (2.0 * assign43230_e48475)))), (0.5 * (locals.var_temp_dn9 + (((locals.var_temp_dn9 * assign43230_e48471) + (assign43230_e48468 * locals.var_temp_dn9)) / (2.0 * assign43230_e48475)))), );
        locals.var_t1_rv = 0.0;
        let assign43240_e48479: f64 = (-0.1666666666667);
        let assign43240_e48481: f64 = (assign43240_e48479 * locals.var_delta_k1q1_dc);
        let assign43240_e48483: f64 = (assign43240_e48481 * locals.var_inv_k1h1_0_dc);
        (locals.var_sqrt_t2, locals.var_sqrt_t2_dn4, locals.var_sqrt_t2_dn6, locals.var_sqrt_t2_dn7, locals.var_sqrt_t2_dn8, locals.var_sqrt_t2_dn9, ) = (assign43240_e48483, (((assign43240_e48479 * locals.var_delta_k1q1_dc_dn4) * locals.var_inv_k1h1_0_dc) + (assign43240_e48481 * locals.var_inv_k1h1_0_dc_dn4)), (((assign43240_e48479 * locals.var_delta_k1q1_dc_dn6) * locals.var_inv_k1h1_0_dc) + (assign43240_e48481 * locals.var_inv_k1h1_0_dc_dn6)), (((assign43240_e48479 * locals.var_delta_k1q1_dc_dn7) * locals.var_inv_k1h1_0_dc) + (assign43240_e48481 * locals.var_inv_k1h1_0_dc_dn7)), (((assign43240_e48479 * locals.var_delta_k1q1_dc_dn8) * locals.var_inv_k1h1_0_dc) + (assign43240_e48481 * locals.var_inv_k1h1_0_dc_dn8)), (((assign43240_e48479 * locals.var_delta_k1q1_dc_dn9) * locals.var_inv_k1h1_0_dc) + (assign43240_e48481 * locals.var_inv_k1h1_0_dc_dn9)), );
        locals.var_sqrt_t2_rv = 0.0;
        let assign43250_e48486: f64 = (locals.var_sqrt_t2 * locals.var_sqrt_t2);
        (locals.var_t2, locals.var_t2_dn4, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, ) = (assign43250_e48486, ((locals.var_sqrt_t2_dn4 * locals.var_sqrt_t2) + (locals.var_sqrt_t2 * locals.var_sqrt_t2_dn4)), ((locals.var_sqrt_t2_dn6 * locals.var_sqrt_t2) + (locals.var_sqrt_t2 * locals.var_sqrt_t2_dn6)), ((locals.var_sqrt_t2_dn7 * locals.var_sqrt_t2) + (locals.var_sqrt_t2 * locals.var_sqrt_t2_dn7)), ((locals.var_sqrt_t2_dn8 * locals.var_sqrt_t2) + (locals.var_sqrt_t2 * locals.var_sqrt_t2_dn8)), ((locals.var_sqrt_t2_dn9 * locals.var_sqrt_t2) + (locals.var_sqrt_t2 * locals.var_sqrt_t2_dn9)), );
        locals.var_t2_rv = 0.0;
        let assign43260_e48489: f64 = (locals.var_hsat_dc - 1.0);
        (locals.var_r, locals.var_r_dn4, locals.var_r_dn6, locals.var_r_dn7, locals.var_r_dn8, locals.var_r_dn9, ) = (assign43260_e48489, locals.var_hsat_dc_dn4, locals.var_hsat_dc_dn6, locals.var_hsat_dc_dn7, locals.var_hsat_dc_dn8, locals.var_hsat_dc_dn9, );
        locals.var_r_rv = 0.0;
        let assign43300_e48519: f64 = (12.0 * locals.var_t2);
        (locals.var_t2x12, locals.var_t2x12_dn4, locals.var_t2x12_dn6, locals.var_t2x12_dn7, locals.var_t2x12_dn8, locals.var_t2x12_dn9, ) = (assign43300_e48519, (12.0 * locals.var_t2_dn4), (12.0 * locals.var_t2_dn6), (12.0 * locals.var_t2_dn7), (12.0 * locals.var_t2_dn8), (12.0 * locals.var_t2_dn9), );
        locals.var_t2x12_rv = 0.0;
        let assign43310_e48522: f64 = (locals.var_t1 + locals.var_t2x12);
        let assign43310_e48526: f64 = (1.0 + locals.var_t1);
        let assign43310_e48527: f64 = (2.0 * assign43310_e48526);
        let assign43310_e48529: f64 = (assign43310_e48527 * locals.var_t2x12);
        let assign43310_e48531: f64 = (assign43310_e48529 * locals.var_r);
        let assign43310_e48532: f64 = (assign43310_e48522 - assign43310_e48531);
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9, ) = (assign43310_e48532, ((locals.var_t1_dn4 + locals.var_t2x12_dn4) - (((((2.0 * locals.var_t1_dn4) * locals.var_t2x12) + (assign43310_e48527 * locals.var_t2x12_dn4)) * locals.var_r) + (assign43310_e48529 * locals.var_r_dn4))), ((locals.var_t1_dn6 + locals.var_t2x12_dn6) - (((((2.0 * locals.var_t1_dn6) * locals.var_t2x12) + (assign43310_e48527 * locals.var_t2x12_dn6)) * locals.var_r) + (assign43310_e48529 * locals.var_r_dn6))), ((locals.var_t1_dn7 + locals.var_t2x12_dn7) - (((((2.0 * locals.var_t1_dn7) * locals.var_t2x12) + (assign43310_e48527 * locals.var_t2x12_dn7)) * locals.var_r) + (assign43310_e48529 * locals.var_r_dn7))), ((locals.var_t1_dn8 + locals.var_t2x12_dn8) - (((((2.0 * locals.var_t1_dn8) * locals.var_t2x12) + (assign43310_e48527 * locals.var_t2x12_dn8)) * locals.var_r) + (assign43310_e48529 * locals.var_r_dn8))), ((locals.var_t1_dn9 + locals.var_t2x12_dn9) - (((((2.0 * locals.var_t1_dn9) * locals.var_t2x12) + (assign43310_e48527 * locals.var_t2x12_dn9)) * locals.var_r) + (assign43310_e48529 * locals.var_r_dn9))), );
        locals.var_temp1_rv = 0.0;
        let assign43320_e48535: f64 = (locals.var_temp1).max(1e-40);
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9, ) = (assign43320_e48535, if locals.var_temp1 >= 1e-40 { locals.var_temp1_dn4 } else { 0.0 }, if locals.var_temp1 >= 1e-40 { locals.var_temp1_dn6 } else { 0.0 }, if locals.var_temp1 >= 1e-40 { locals.var_temp1_dn7 } else { 0.0 }, if locals.var_temp1 >= 1e-40 { locals.var_temp1_dn8 } else { 0.0 }, if locals.var_temp1 >= 1e-40 { locals.var_temp1_dn9 } else { 0.0 }, );
        locals.var_temp2_rv = 0.0;
        let assign43390_e48587: f64 = (locals.var_k1_ac * locals.var_csiprime_ac);
        let assign43390_e48589: f64 = (assign43390_e48587 * locals.var_areaq_i);
        let assign43390_e48591: f64 = (assign43390_e48589 / locals.var_qmfact1_ac);
        (locals.var_cox_qm, locals.var_cox_qm_dn4, locals.var_cox_qm_dn6, locals.var_cox_qm_dn7, locals.var_cox_qm_dn8, locals.var_cox_qm_dn9, ) = (assign43390_e48591, ((((((locals.var_k1_ac_dn4 * locals.var_csiprime_ac) + (locals.var_k1_ac * locals.var_csiprime_ac_dn4)) * locals.var_areaq_i) * locals.var_qmfact1_ac) - (assign43390_e48589 * locals.var_qmfact1_ac_dn4)) / (locals.var_qmfact1_ac * locals.var_qmfact1_ac)), ((((((locals.var_k1_ac_dn6 * locals.var_csiprime_ac) + (locals.var_k1_ac * locals.var_csiprime_ac_dn6)) * locals.var_areaq_i) * locals.var_qmfact1_ac) - (assign43390_e48589 * locals.var_qmfact1_ac_dn6)) / (locals.var_qmfact1_ac * locals.var_qmfact1_ac)), ((((((locals.var_k1_ac_dn7 * locals.var_csiprime_ac) + (locals.var_k1_ac * locals.var_csiprime_ac_dn7)) * locals.var_areaq_i) * locals.var_qmfact1_ac) - (assign43390_e48589 * locals.var_qmfact1_ac_dn7)) / (locals.var_qmfact1_ac * locals.var_qmfact1_ac)), ((((((locals.var_k1_ac_dn8 * locals.var_csiprime_ac) + (locals.var_k1_ac * locals.var_csiprime_ac_dn8)) * locals.var_areaq_i) * locals.var_qmfact1_ac) - (assign43390_e48589 * locals.var_qmfact1_ac_dn8)) / (locals.var_qmfact1_ac * locals.var_qmfact1_ac)), ((((((locals.var_k1_ac_dn9 * locals.var_csiprime_ac) + (locals.var_k1_ac * locals.var_csiprime_ac_dn9)) * locals.var_areaq_i) * locals.var_qmfact1_ac) - (assign43390_e48589 * locals.var_qmfact1_ac_dn9)) / (locals.var_qmfact1_ac * locals.var_qmfact1_ac)), );
        locals.var_cox_qm_rv = 0.0;
        let assign43400_e48594: f64 = (1.0 + locals.var_zsat_ac);
        let assign43400_e48596: f64 = (assign43400_e48594 * locals.var_cox_qm);
        (locals.var_cgeff, locals.var_cgeff_dn4, locals.var_cgeff_dn6, locals.var_cgeff_dn7, locals.var_cgeff_dn8, locals.var_cgeff_dn9, ) = (assign43400_e48596, ((locals.var_zsat_ac_dn4 * locals.var_cox_qm) + (assign43400_e48594 * locals.var_cox_qm_dn4)), ((locals.var_zsat_ac_dn6 * locals.var_cox_qm) + (assign43400_e48594 * locals.var_cox_qm_dn6)), ((locals.var_zsat_ac_dn7 * locals.var_cox_qm) + (assign43400_e48594 * locals.var_cox_qm_dn7)), ((locals.var_zsat_ac_dn8 * locals.var_cox_qm) + (assign43400_e48594 * locals.var_cox_qm_dn8)), ((locals.var_zsat_ac_dn9 * locals.var_cox_qm) + (assign43400_e48594 * locals.var_cox_qm_dn9)), );
        locals.var_cgeff_rv = 0.0;
        let assign43410_e48601: f64 = (0.25 * locals.var_sigvds);
        let assign43410_e48603: f64 = (assign43410_e48601 * locals.var_sqrt_t2);
        let assign43410_e48604: f64 = (0.5 - assign43410_e48603);
        let assign43410_e48605: f64 = (locals.var_cgeff * assign43410_e48604);
        (locals.var_cdgeff, locals.var_cdgeff_dn4, locals.var_cdgeff_dn6, locals.var_cdgeff_dn7, locals.var_cdgeff_dn8, locals.var_cdgeff_dn9, ) = (assign43410_e48605, ((locals.var_cgeff_dn4 * assign43410_e48604) + (locals.var_cgeff * (-(assign43410_e48601 * locals.var_sqrt_t2_dn4)))), ((locals.var_cgeff_dn6 * assign43410_e48604) + (locals.var_cgeff * (-(assign43410_e48601 * locals.var_sqrt_t2_dn6)))), ((locals.var_cgeff_dn7 * assign43410_e48604) + (locals.var_cgeff * (-(assign43410_e48601 * locals.var_sqrt_t2_dn7)))), ((locals.var_cgeff_dn8 * assign43410_e48604) + (locals.var_cgeff * (-(assign43410_e48601 * locals.var_sqrt_t2_dn8)))), ((locals.var_cgeff_dn9 * assign43410_e48604) + (locals.var_cgeff * (-(assign43410_e48601 * locals.var_sqrt_t2_dn9)))), );
        locals.var_cdgeff_rv = 0.0;
        let assign43420_e48608: f64 = (locals.var_cgeff - locals.var_cdgeff);
        (locals.var_csgeff, locals.var_csgeff_dn4, locals.var_csgeff_dn6, locals.var_csgeff_dn7, locals.var_csgeff_dn8, locals.var_csgeff_dn9, ) = (assign43420_e48608, (locals.var_cgeff_dn4 - locals.var_cdgeff_dn4), (locals.var_cgeff_dn6 - locals.var_cdgeff_dn6), (locals.var_cgeff_dn7 - locals.var_cdgeff_dn7), (locals.var_cgeff_dn8 - locals.var_cdgeff_dn8), (locals.var_cgeff_dn9 - locals.var_cdgeff_dn9), );
        locals.var_csgeff_rv = 0.0;
        let assign43450_e48613: f64 = if p.p6 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1279 = assign43450_e48613;
        locals.var_guard1279_rv = 0.0;
        if (locals.var_guard1279 != 0.0) {
            let assign43460_e48617: f64 = (locals.var_t1 / 12.0);
            let assign43460_e48621: f64 = (locals.var_t1 + 0.2);
            let assign43460_e48623: f64 = (assign43460_e48621 - locals.var_t2x12);
            let assign43460_e48624: f64 = (locals.var_t2 * assign43460_e48623);
            let assign43460_e48625: f64 = (assign43460_e48617 - assign43460_e48624);
            let assign43460_e48628: f64 = (1.6 * locals.var_t2);
            let assign43460_e48631: f64 = (locals.var_t1 + 1.0);
            let assign43460_e48633: f64 = (assign43460_e48631 - locals.var_t2x12);
            let assign43460_e48634: f64 = (assign43460_e48628 * assign43460_e48633);
            let assign43460_e48636: f64 = (assign43460_e48634 * locals.var_r);
            let assign43460_e48637: f64 = (assign43460_e48625 - assign43460_e48636);
            (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9, ) = (assign43460_e48637, (((locals.var_t1_dn4 / 12.0) - ((locals.var_t2_dn4 * assign43460_e48623) + (locals.var_t2 * (locals.var_t1_dn4 - locals.var_t2x12_dn4)))) - (((((1.6 * locals.var_t2_dn4) * assign43460_e48633) + (assign43460_e48628 * (locals.var_t1_dn4 - locals.var_t2x12_dn4))) * locals.var_r) + (assign43460_e48634 * locals.var_r_dn4))), (((locals.var_t1_dn6 / 12.0) - ((locals.var_t2_dn6 * assign43460_e48623) + (locals.var_t2 * (locals.var_t1_dn6 - locals.var_t2x12_dn6)))) - (((((1.6 * locals.var_t2_dn6) * assign43460_e48633) + (assign43460_e48628 * (locals.var_t1_dn6 - locals.var_t2x12_dn6))) * locals.var_r) + (assign43460_e48634 * locals.var_r_dn6))), (((locals.var_t1_dn7 / 12.0) - ((locals.var_t2_dn7 * assign43460_e48623) + (locals.var_t2 * (locals.var_t1_dn7 - locals.var_t2x12_dn7)))) - (((((1.6 * locals.var_t2_dn7) * assign43460_e48633) + (assign43460_e48628 * (locals.var_t1_dn7 - locals.var_t2x12_dn7))) * locals.var_r) + (assign43460_e48634 * locals.var_r_dn7))), (((locals.var_t1_dn8 / 12.0) - ((locals.var_t2_dn8 * assign43460_e48623) + (locals.var_t2 * (locals.var_t1_dn8 - locals.var_t2x12_dn8)))) - (((((1.6 * locals.var_t2_dn8) * assign43460_e48633) + (assign43460_e48628 * (locals.var_t1_dn8 - locals.var_t2x12_dn8))) * locals.var_r) + (assign43460_e48634 * locals.var_r_dn8))), (((locals.var_t1_dn9 / 12.0) - ((locals.var_t2_dn9 * assign43460_e48623) + (locals.var_t2 * (locals.var_t1_dn9 - locals.var_t2x12_dn9)))) - (((((1.6 * locals.var_t2_dn9) * assign43460_e48633) + (assign43460_e48628 * (locals.var_t1_dn9 - locals.var_t2x12_dn9))) * locals.var_r) + (assign43460_e48634 * locals.var_r_dn9))), );
            locals.var_temp1_rv = 0.0;
        }
    }
    pub(super) fn stamp_reactive_block_62(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        if (locals.var_guard1279 != 0.0) {
            let assign43470_e48643: f64 = (locals.var_temp1).max(1e-40);
            (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9, ) = (assign43470_e48643, if locals.var_temp1 >= 1e-40 { locals.var_temp1_dn4 } else { 0.0 }, if locals.var_temp1 >= 1e-40 { locals.var_temp1_dn6 } else { 0.0 }, if locals.var_temp1 >= 1e-40 { locals.var_temp1_dn7 } else { 0.0 }, if locals.var_temp1 >= 1e-40 { locals.var_temp1_dn8 } else { 0.0 }, if locals.var_temp1 >= 1e-40 { locals.var_temp1_dn9 } else { 0.0 }, );
            locals.var_temp2_rv = 0.0;
        }
        (locals.var_nstar, locals.var_nstar_dn4, locals.var_nstar_dn6, locals.var_nstar_dn7, locals.var_nstar_dn8, locals.var_nstar_dn9, ) = (locals.var_nunit, locals.var_nunit_dn4, locals.var_nunit_dn6, locals.var_nunit_dn7, locals.var_nunit_dn8, locals.var_nunit_dn9, );
        locals.var_nstar_rv = 0.0;
        let assign43580_e48790: f64 = (locals.var_qim_dc + 1.0);
        let assign43580_e48791: f64 = (locals.var_nunit * assign43580_e48790);
        (locals.var_nmstar, locals.var_nmstar_dn4, locals.var_nmstar_dn6, locals.var_nmstar_dn7, locals.var_nmstar_dn8, locals.var_nmstar_dn9, ) = (assign43580_e48791, ((locals.var_nunit_dn4 * assign43580_e48790) + (locals.var_nunit * locals.var_qim_dc_dn4)), ((locals.var_nunit_dn6 * assign43580_e48790) + (locals.var_nunit * locals.var_qim_dc_dn6)), ((locals.var_nunit_dn7 * assign43580_e48790) + (locals.var_nunit * locals.var_qim_dc_dn7)), ((locals.var_nunit_dn8 * assign43580_e48790) + (locals.var_nunit * locals.var_qim_dc_dn8)), ((locals.var_nunit_dn9 * assign43580_e48790) + (locals.var_nunit * locals.var_qim_dc_dn9)), );
        locals.var_nmstar_rv = 0.0;
        let assign43590_e48795: f64 = (locals.var_qis_dc - locals.var_qid_dc);
        let assign43590_e48796: f64 = (locals.var_nunit * assign43590_e48795);
        (locals.var_deltan, locals.var_deltan_dn4, locals.var_deltan_dn6, locals.var_deltan_dn7, locals.var_deltan_dn8, locals.var_deltan_dn9, ) = (assign43590_e48796, ((locals.var_nunit_dn4 * assign43590_e48795) + (locals.var_nunit * (locals.var_qis_dc_dn4 - locals.var_qid_dc_dn4))), ((locals.var_nunit_dn6 * assign43590_e48795) + (locals.var_nunit * (locals.var_qis_dc_dn6 - locals.var_qid_dc_dn6))), ((locals.var_nunit_dn7 * assign43590_e48795) + (locals.var_nunit * (locals.var_qis_dc_dn7 - locals.var_qid_dc_dn7))), ((locals.var_nunit_dn8 * assign43590_e48795) + (locals.var_nunit * (locals.var_qis_dc_dn8 - locals.var_qid_dc_dn8))), ((locals.var_nunit_dn9 * assign43590_e48795) + (locals.var_nunit * (locals.var_qis_dc_dn9 - locals.var_qid_dc_dn9))), );
        locals.var_deltan_rv = 0.0;
        let assign43600_e48800: f64 = (locals.var_nfb_i * locals.var_nstar);
        let assign43600_e48801: f64 = (locals.var_nfa_i - assign43600_e48800);
        let assign43600_e48804: f64 = (locals.var_nfc_i * locals.var_nstar);
        let assign43600_e48806: f64 = (assign43600_e48804 * locals.var_nstar);
        let assign43600_e48807: f64 = (assign43600_e48801 + assign43600_e48806);
        let assign43600_e48811: f64 = (0.5 * locals.var_deltan);
        let assign43600_e48812: f64 = (locals.var_nmstar + assign43600_e48811);
        let assign43600_e48816: f64 = (0.5 * locals.var_deltan);
        let assign43600_e48817: f64 = (locals.var_nmstar - assign43600_e48816);
        let assign43600_e48818: f64 = (assign43600_e48812 / assign43600_e48817);
        let assign43600_e48819: f64 = (assign43600_e48818).ln();
        let assign43600_e48820: f64 = (assign43600_e48807 * assign43600_e48819);
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9, ) = (assign43600_e48820, ((((-(locals.var_nfb_i * locals.var_nstar_dn4)) + (((locals.var_nfc_i * locals.var_nstar_dn4) * locals.var_nstar) + (assign43600_e48804 * locals.var_nstar_dn4))) * assign43600_e48819) + (assign43600_e48807 * (((((locals.var_nmstar_dn4 + (0.5 * locals.var_deltan_dn4)) * assign43600_e48817) - (assign43600_e48812 * (locals.var_nmstar_dn4 - (0.5 * locals.var_deltan_dn4)))) / (assign43600_e48817 * assign43600_e48817)) / assign43600_e48818))), ((((-(locals.var_nfb_i * locals.var_nstar_dn6)) + (((locals.var_nfc_i * locals.var_nstar_dn6) * locals.var_nstar) + (assign43600_e48804 * locals.var_nstar_dn6))) * assign43600_e48819) + (assign43600_e48807 * (((((locals.var_nmstar_dn6 + (0.5 * locals.var_deltan_dn6)) * assign43600_e48817) - (assign43600_e48812 * (locals.var_nmstar_dn6 - (0.5 * locals.var_deltan_dn6)))) / (assign43600_e48817 * assign43600_e48817)) / assign43600_e48818))), ((((-(locals.var_nfb_i * locals.var_nstar_dn7)) + (((locals.var_nfc_i * locals.var_nstar_dn7) * locals.var_nstar) + (assign43600_e48804 * locals.var_nstar_dn7))) * assign43600_e48819) + (assign43600_e48807 * (((((locals.var_nmstar_dn7 + (0.5 * locals.var_deltan_dn7)) * assign43600_e48817) - (assign43600_e48812 * (locals.var_nmstar_dn7 - (0.5 * locals.var_deltan_dn7)))) / (assign43600_e48817 * assign43600_e48817)) / assign43600_e48818))), ((((-(locals.var_nfb_i * locals.var_nstar_dn8)) + (((locals.var_nfc_i * locals.var_nstar_dn8) * locals.var_nstar) + (assign43600_e48804 * locals.var_nstar_dn8))) * assign43600_e48819) + (assign43600_e48807 * (((((locals.var_nmstar_dn8 + (0.5 * locals.var_deltan_dn8)) * assign43600_e48817) - (assign43600_e48812 * (locals.var_nmstar_dn8 - (0.5 * locals.var_deltan_dn8)))) / (assign43600_e48817 * assign43600_e48817)) / assign43600_e48818))), ((((-(locals.var_nfb_i * locals.var_nstar_dn9)) + (((locals.var_nfc_i * locals.var_nstar_dn9) * locals.var_nstar) + (assign43600_e48804 * locals.var_nstar_dn9))) * assign43600_e48819) + (assign43600_e48807 * (((((locals.var_nmstar_dn9 + (0.5 * locals.var_deltan_dn9)) * assign43600_e48817) - (assign43600_e48812 * (locals.var_nmstar_dn9 - (0.5 * locals.var_deltan_dn9)))) / (assign43600_e48817 * assign43600_e48817)) / assign43600_e48818))), );
        locals.var_temp1_rv = 0.0;
        let assign43610_e48827: f64 = (2.0 * locals.var_nstar);
        let assign43610_e48828: f64 = (locals.var_nmstar - assign43610_e48827);
        let assign43610_e48829: f64 = (locals.var_nfc_i * assign43610_e48828);
        let assign43610_e48830: f64 = (locals.var_nfb_i + assign43610_e48829);
        let assign43610_e48832: f64 = (assign43610_e48830 * locals.var_deltan);
        let assign43610_e48833: f64 = (locals.var_temp1 + assign43610_e48832);
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9, ) = (assign43610_e48833, (locals.var_temp1_dn4 + (((locals.var_nfc_i * (locals.var_nmstar_dn4 - (2.0 * locals.var_nstar_dn4))) * locals.var_deltan) + (assign43610_e48830 * locals.var_deltan_dn4))), (locals.var_temp1_dn6 + (((locals.var_nfc_i * (locals.var_nmstar_dn6 - (2.0 * locals.var_nstar_dn6))) * locals.var_deltan) + (assign43610_e48830 * locals.var_deltan_dn6))), (locals.var_temp1_dn7 + (((locals.var_nfc_i * (locals.var_nmstar_dn7 - (2.0 * locals.var_nstar_dn7))) * locals.var_deltan) + (assign43610_e48830 * locals.var_deltan_dn7))), (locals.var_temp1_dn8 + (((locals.var_nfc_i * (locals.var_nmstar_dn8 - (2.0 * locals.var_nstar_dn8))) * locals.var_deltan) + (assign43610_e48830 * locals.var_deltan_dn8))), (locals.var_temp1_dn9 + (((locals.var_nfc_i * (locals.var_nmstar_dn9 - (2.0 * locals.var_nstar_dn9))) * locals.var_deltan) + (assign43610_e48830 * locals.var_deltan_dn9))), );
        locals.var_temp2_rv = 0.0;
        let assign43620_e48837: f64 = (locals.var_nfe_i * locals.var_esurf1_dc);
        let assign43620_e48840: f64 = (locals.var_nfeb_i * locals.var_esurf2_dc);
        let assign43620_e48841: f64 = (assign43620_e48837 + assign43620_e48840);
        let assign43620_e48844: f64 = (locals.var_qim_dc + 1.0);
        let assign43620_e48845: f64 = (assign43620_e48841 / assign43620_e48844);
        let assign43620_e48846: f64 = (1.0 + assign43620_e48845);
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign43620_e48846, (((((locals.var_nfe_i * locals.var_esurf1_dc_dn4) + (locals.var_nfeb_i * locals.var_esurf2_dc_dn4)) * assign43620_e48844) - (assign43620_e48841 * locals.var_qim_dc_dn4)) / (assign43620_e48844 * assign43620_e48844)), (((((locals.var_nfe_i * locals.var_esurf1_dc_dn6) + (locals.var_nfeb_i * locals.var_esurf2_dc_dn6)) * assign43620_e48844) - (assign43620_e48841 * locals.var_qim_dc_dn6)) / (assign43620_e48844 * assign43620_e48844)), (((((locals.var_nfe_i * locals.var_esurf1_dc_dn7) + (locals.var_nfeb_i * locals.var_esurf2_dc_dn7)) * assign43620_e48844) - (assign43620_e48841 * locals.var_qim_dc_dn7)) / (assign43620_e48844 * assign43620_e48844)), (((((locals.var_nfe_i * locals.var_esurf1_dc_dn8) + (locals.var_nfeb_i * locals.var_esurf2_dc_dn8)) * assign43620_e48844) - (assign43620_e48841 * locals.var_qim_dc_dn8)) / (assign43620_e48844 * assign43620_e48844)), (((((locals.var_nfe_i * locals.var_esurf1_dc_dn9) + (locals.var_nfeb_i * locals.var_esurf2_dc_dn9)) * assign43620_e48844) - (assign43620_e48841 * locals.var_qim_dc_dn9)) / (assign43620_e48844 * assign43620_e48844)), );
        locals.var_temp_rv = 0.0;
        let assign43630_e48850: f64 = (locals.var_temp + 0.01);
        let assign43630_e48853: f64 = (locals.var_temp - 0.01);
        let assign43630_e48856: f64 = (locals.var_temp - 0.01);
        let assign43630_e48857: f64 = (assign43630_e48853 * assign43630_e48856);
        let assign43630_e48859: f64 = (assign43630_e48857 + 0.0001);
        let assign43630_e48860: f64 = (assign43630_e48859).sqrt();
        let assign43630_e48861: f64 = (assign43630_e48850 + assign43630_e48860);
        let assign43630_e48862: f64 = (0.5 * assign43630_e48861);
        (locals.var_temp3, locals.var_temp3_dn4, locals.var_temp3_dn6, locals.var_temp3_dn7, locals.var_temp3_dn8, locals.var_temp3_dn9, ) = (assign43630_e48862, (0.5 * (locals.var_temp_dn4 + (((locals.var_temp_dn4 * assign43630_e48856) + (assign43630_e48853 * locals.var_temp_dn4)) / (2.0 * assign43630_e48860)))), (0.5 * (locals.var_temp_dn6 + (((locals.var_temp_dn6 * assign43630_e48856) + (assign43630_e48853 * locals.var_temp_dn6)) / (2.0 * assign43630_e48860)))), (0.5 * (locals.var_temp_dn7 + (((locals.var_temp_dn7 * assign43630_e48856) + (assign43630_e48853 * locals.var_temp_dn7)) / (2.0 * assign43630_e48860)))), (0.5 * (locals.var_temp_dn8 + (((locals.var_temp_dn8 * assign43630_e48856) + (assign43630_e48853 * locals.var_temp_dn8)) / (2.0 * assign43630_e48860)))), (0.5 * (locals.var_temp_dn9 + (((locals.var_temp_dn9 * assign43630_e48856) + (assign43630_e48853 * locals.var_temp_dn9)) / (2.0 * assign43630_e48860)))), );
        locals.var_temp3_rv = 0.0;
        let assign43640_e48865: f64 = (1.602176565e-19 * locals.var_fact_ids);
        let assign43640_e48867: f64 = (assign43640_e48865 * locals.var_ids);
        let assign43640_e48869: f64 = (assign43640_e48867 / locals.var_gvsat);
        let assign43640_e48871: f64 = (assign43640_e48869 * locals.var_temp2);
        let assign43640_e48873: f64 = (assign43640_e48871 / locals.var_nstar);
        let assign43640_e48875: f64 = (assign43640_e48873 * locals.var_temp3);
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign43640_e48875, (((((((((((((1.602176565e-19 * locals.var_fact_ids_dn4) * locals.var_ids) + (assign43640_e48865 * locals.var_ids_dn4)) * locals.var_gvsat) - (assign43640_e48867 * locals.var_gvsat_dn4)) / (locals.var_gvsat * locals.var_gvsat)) * locals.var_temp2) + (assign43640_e48869 * locals.var_temp2_dn4)) * locals.var_nstar) - (assign43640_e48871 * locals.var_nstar_dn4)) / (locals.var_nstar * locals.var_nstar)) * locals.var_temp3) + (assign43640_e48873 * locals.var_temp3_dn4)), (((((((((((((1.602176565e-19 * locals.var_fact_ids_dn6) * locals.var_ids) + (assign43640_e48865 * locals.var_ids_dn6)) * locals.var_gvsat) - (assign43640_e48867 * locals.var_gvsat_dn6)) / (locals.var_gvsat * locals.var_gvsat)) * locals.var_temp2) + (assign43640_e48869 * locals.var_temp2_dn6)) * locals.var_nstar) - (assign43640_e48871 * locals.var_nstar_dn6)) / (locals.var_nstar * locals.var_nstar)) * locals.var_temp3) + (assign43640_e48873 * locals.var_temp3_dn6)), (((((((((((((1.602176565e-19 * locals.var_fact_ids_dn7) * locals.var_ids) + (assign43640_e48865 * locals.var_ids_dn7)) * locals.var_gvsat) - (assign43640_e48867 * locals.var_gvsat_dn7)) / (locals.var_gvsat * locals.var_gvsat)) * locals.var_temp2) + (assign43640_e48869 * locals.var_temp2_dn7)) * locals.var_nstar) - (assign43640_e48871 * locals.var_nstar_dn7)) / (locals.var_nstar * locals.var_nstar)) * locals.var_temp3) + (assign43640_e48873 * locals.var_temp3_dn7)), (((((((((((((1.602176565e-19 * locals.var_fact_ids_dn8) * locals.var_ids) + (assign43640_e48865 * locals.var_ids_dn8)) * locals.var_gvsat) - (assign43640_e48867 * locals.var_gvsat_dn8)) / (locals.var_gvsat * locals.var_gvsat)) * locals.var_temp2) + (assign43640_e48869 * locals.var_temp2_dn8)) * locals.var_nstar) - (assign43640_e48871 * locals.var_nstar_dn8)) / (locals.var_nstar * locals.var_nstar)) * locals.var_temp3) + (assign43640_e48873 * locals.var_temp3_dn8)), (((((((((((((1.602176565e-19 * locals.var_fact_ids_dn9) * locals.var_ids) + (assign43640_e48865 * locals.var_ids_dn9)) * locals.var_gvsat) - (assign43640_e48867 * locals.var_gvsat_dn9)) / (locals.var_gvsat * locals.var_gvsat)) * locals.var_temp2) + (assign43640_e48869 * locals.var_temp2_dn9)) * locals.var_nstar) - (assign43640_e48871 * locals.var_nstar_dn9)) / (locals.var_nstar * locals.var_nstar)) * locals.var_temp3) + (assign43640_e48873 * locals.var_temp3_dn9)), );
        locals.var_temp_rv = 0.0;
        let assign43710_e48935: f64 = (locals.var_tkd * 8.617332384961e-5);
        let assign43710_e48936: f64 = (1.0 / assign43710_e48935);
        (locals.var_inv_phit0_op, locals.var_inv_phit0_op_dn4, locals.var_inv_phit0_op_dn6, locals.var_inv_phit0_op_dn7, locals.var_inv_phit0_op_dn8, locals.var_inv_phit0_op_dn9, ) = (assign43710_e48936, (-((locals.var_tkd_dn4 * 8.617332384961e-5) / (assign43710_e48935 * assign43710_e48935))), (-((locals.var_tkd_dn6 * 8.617332384961e-5) / (assign43710_e48935 * assign43710_e48935))), (-((locals.var_tkd_dn7 * 8.617332384961e-5) / (assign43710_e48935 * assign43710_e48935))), (-((locals.var_tkd_dn8 * 8.617332384961e-5) / (assign43710_e48935 * assign43710_e48935))), (-((locals.var_tkd_dn9 * 8.617332384961e-5) / (assign43710_e48935 * assign43710_e48935))), );
        locals.var_inv_phit0_op_rv = 0.0;
        let assign43720_e48940: f64 = (0.000473 * locals.var_tkd);
        let assign43720_e48942: f64 = (assign43720_e48940 * locals.var_tkd);
        let assign43720_e48945: f64 = (636.0 + locals.var_tkd);
        let assign43720_e48946: f64 = (assign43720_e48942 / assign43720_e48945);
        let assign43720_e48947: f64 = (1.17 - assign43720_e48946);
        (locals.var_egsi_op, locals.var_egsi_op_dn4, locals.var_egsi_op_dn6, locals.var_egsi_op_dn7, locals.var_egsi_op_dn8, locals.var_egsi_op_dn9, ) = (assign43720_e48947, (-((((((0.000473 * locals.var_tkd_dn4) * locals.var_tkd) + (assign43720_e48940 * locals.var_tkd_dn4)) * assign43720_e48945) - (assign43720_e48942 * locals.var_tkd_dn4)) / (assign43720_e48945 * assign43720_e48945))), (-((((((0.000473 * locals.var_tkd_dn6) * locals.var_tkd) + (assign43720_e48940 * locals.var_tkd_dn6)) * assign43720_e48945) - (assign43720_e48942 * locals.var_tkd_dn6)) / (assign43720_e48945 * assign43720_e48945))), (-((((((0.000473 * locals.var_tkd_dn7) * locals.var_tkd) + (assign43720_e48940 * locals.var_tkd_dn7)) * assign43720_e48945) - (assign43720_e48942 * locals.var_tkd_dn7)) / (assign43720_e48945 * assign43720_e48945))), (-((((((0.000473 * locals.var_tkd_dn8) * locals.var_tkd) + (assign43720_e48940 * locals.var_tkd_dn8)) * assign43720_e48945) - (assign43720_e48942 * locals.var_tkd_dn8)) / (assign43720_e48945 * assign43720_e48945))), (-((((((0.000473 * locals.var_tkd_dn9) * locals.var_tkd) + (assign43720_e48940 * locals.var_tkd_dn9)) * assign43720_e48945) - (assign43720_e48942 * locals.var_tkd_dn9)) / (assign43720_e48945 * assign43720_e48945))), );
        locals.var_egsi_op_rv = 0.0;
        let assign43730_e48951: f64 = (0.0004774 * locals.var_tkd);
        let assign43730_e48953: f64 = (assign43730_e48951 * locals.var_tkd);
        let assign43730_e48956: f64 = (235.0 + locals.var_tkd);
        let assign43730_e48957: f64 = (assign43730_e48953 / assign43730_e48956);
        let assign43730_e48958: f64 = (0.744 - assign43730_e48957);
        (locals.var_egge_op, locals.var_egge_op_dn4, locals.var_egge_op_dn6, locals.var_egge_op_dn7, locals.var_egge_op_dn8, locals.var_egge_op_dn9, ) = (assign43730_e48958, (-((((((0.0004774 * locals.var_tkd_dn4) * locals.var_tkd) + (assign43730_e48951 * locals.var_tkd_dn4)) * assign43730_e48956) - (assign43730_e48953 * locals.var_tkd_dn4)) / (assign43730_e48956 * assign43730_e48956))), (-((((((0.0004774 * locals.var_tkd_dn6) * locals.var_tkd) + (assign43730_e48951 * locals.var_tkd_dn6)) * assign43730_e48956) - (assign43730_e48953 * locals.var_tkd_dn6)) / (assign43730_e48956 * assign43730_e48956))), (-((((((0.0004774 * locals.var_tkd_dn7) * locals.var_tkd) + (assign43730_e48951 * locals.var_tkd_dn7)) * assign43730_e48956) - (assign43730_e48953 * locals.var_tkd_dn7)) / (assign43730_e48956 * assign43730_e48956))), (-((((((0.0004774 * locals.var_tkd_dn8) * locals.var_tkd) + (assign43730_e48951 * locals.var_tkd_dn8)) * assign43730_e48956) - (assign43730_e48953 * locals.var_tkd_dn8)) / (assign43730_e48956 * assign43730_e48956))), (-((((((0.0004774 * locals.var_tkd_dn9) * locals.var_tkd) + (assign43730_e48951 * locals.var_tkd_dn9)) * assign43730_e48956) - (assign43730_e48953 * locals.var_tkd_dn9)) / (assign43730_e48956 * assign43730_e48956))), );
        locals.var_egge_op_rv = 0.0;
        let assign43740_e48961: f64 = (locals.var_egge_op - locals.var_egsi_op);
        let assign43740_e48963: f64 = (-0.4);
        let assign43740_e48965: f64 = (assign43740_e48963 * locals.var_one_m_xge);
        let assign43740_e48966: f64 = (assign43740_e48961 + assign43740_e48965);
        let assign43740_e48968: f64 = (assign43740_e48966 * locals.var_xge_i);
        (locals.var_deg_op, locals.var_deg_op_dn4, locals.var_deg_op_dn6, locals.var_deg_op_dn7, locals.var_deg_op_dn8, locals.var_deg_op_dn9, ) = (assign43740_e48968, ((locals.var_egge_op_dn4 - locals.var_egsi_op_dn4) * locals.var_xge_i), ((locals.var_egge_op_dn6 - locals.var_egsi_op_dn6) * locals.var_xge_i), ((locals.var_egge_op_dn7 - locals.var_egsi_op_dn7) * locals.var_xge_i), ((locals.var_egge_op_dn8 - locals.var_egsi_op_dn8) * locals.var_xge_i), ((locals.var_egge_op_dn9 - locals.var_egsi_op_dn9) * locals.var_xge_i), );
        locals.var_deg_op_rv = 0.0;
        let assign43750_e48971: f64 = (locals.var_egsi_op + locals.var_deg_op);
        (locals.var_eg_op, locals.var_eg_op_dn4, locals.var_eg_op_dn6, locals.var_eg_op_dn7, locals.var_eg_op_dn8, locals.var_eg_op_dn9, ) = (assign43750_e48971, (locals.var_egsi_op_dn4 + locals.var_deg_op_dn4), (locals.var_egsi_op_dn6 + locals.var_deg_op_dn6), (locals.var_egsi_op_dn7 + locals.var_deg_op_dn7), (locals.var_egsi_op_dn8 + locals.var_deg_op_dn8), (locals.var_egsi_op_dn9 + locals.var_deg_op_dn9), );
        locals.var_eg_op_rv = 0.0;
        let assign43760_e48974: f64 = (0.5 * locals.var_eg_op);
        let assign43760_e48976: f64 = (assign43760_e48974 * locals.var_inv_phit0_op);
        (locals.var_eg_2phit0_op, locals.var_eg_2phit0_op_dn4, locals.var_eg_2phit0_op_dn6, locals.var_eg_2phit0_op_dn7, locals.var_eg_2phit0_op_dn8, locals.var_eg_2phit0_op_dn9, ) = (assign43760_e48976, (((0.5 * locals.var_eg_op_dn4) * locals.var_inv_phit0_op) + (assign43760_e48974 * locals.var_inv_phit0_op_dn4)), (((0.5 * locals.var_eg_op_dn6) * locals.var_inv_phit0_op) + (assign43760_e48974 * locals.var_inv_phit0_op_dn6)), (((0.5 * locals.var_eg_op_dn7) * locals.var_inv_phit0_op) + (assign43760_e48974 * locals.var_inv_phit0_op_dn7)), (((0.5 * locals.var_eg_op_dn8) * locals.var_inv_phit0_op) + (assign43760_e48974 * locals.var_inv_phit0_op_dn8)), (((0.5 * locals.var_eg_op_dn9) * locals.var_inv_phit0_op) + (assign43760_e48974 * locals.var_inv_phit0_op_dn9)), );
        locals.var_eg_2phit0_op_rv = 0.0;
        let assign43770_e48979: f64 = (0.05 * locals.var_xge_i);
        let assign43770_e48982: f64 = (0.5 * locals.var_deg_op);
        let assign43770_e48983: f64 = (assign43770_e48979 - assign43770_e48982);
        (locals.var_dvfbch_op, locals.var_dvfbch_op_dn4, locals.var_dvfbch_op_dn6, locals.var_dvfbch_op_dn7, locals.var_dvfbch_op_dn8, locals.var_dvfbch_op_dn9, ) = (assign43770_e48983, (-(0.5 * locals.var_deg_op_dn4)), (-(0.5 * locals.var_deg_op_dn6)), (-(0.5 * locals.var_deg_op_dn7)), (-(0.5 * locals.var_deg_op_dn8)), (-(0.5 * locals.var_deg_op_dn9)), );
        locals.var_dvfbch_op_rv = 0.0;
        let assign43780_e48986: f64 = (locals.var_tkd * 0.0033333333333);
        let assign43780_e48987: f64 = (assign43780_e48986).sqrt();
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign43780_e48987, ((locals.var_tkd_dn4 * 0.0033333333333) / (2.0 * assign43780_e48987)), ((locals.var_tkd_dn6 * 0.0033333333333) / (2.0 * assign43780_e48987)), ((locals.var_tkd_dn7 * 0.0033333333333) / (2.0 * assign43780_e48987)), ((locals.var_tkd_dn8 * 0.0033333333333) / (2.0 * assign43780_e48987)), ((locals.var_tkd_dn9 * 0.0033333333333) / (2.0 * assign43780_e48987)), );
        locals.var_temp_rv = 0.0;
        let assign43790_e48990: f64 = (4.05e25 * locals.var_temp);
        let assign43790_e48992: f64 = (assign43790_e48990 * locals.var_temp);
        let assign43790_e48994: f64 = (assign43790_e48992 * locals.var_temp);
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9, ) = (assign43790_e48994, (((((4.05e25 * locals.var_temp_dn4) * locals.var_temp) + (assign43790_e48990 * locals.var_temp_dn4)) * locals.var_temp) + (assign43790_e48992 * locals.var_temp_dn4)), (((((4.05e25 * locals.var_temp_dn6) * locals.var_temp) + (assign43790_e48990 * locals.var_temp_dn6)) * locals.var_temp) + (assign43790_e48992 * locals.var_temp_dn6)), (((((4.05e25 * locals.var_temp_dn7) * locals.var_temp) + (assign43790_e48990 * locals.var_temp_dn7)) * locals.var_temp) + (assign43790_e48992 * locals.var_temp_dn7)), (((((4.05e25 * locals.var_temp_dn8) * locals.var_temp) + (assign43790_e48990 * locals.var_temp_dn8)) * locals.var_temp) + (assign43790_e48992 * locals.var_temp_dn8)), (((((4.05e25 * locals.var_temp_dn9) * locals.var_temp) + (assign43790_e48990 * locals.var_temp_dn9)) * locals.var_temp) + (assign43790_e48992 * locals.var_temp_dn9)), );
        locals.var_temp1_rv = 0.0;
        let assign43800_e48997: f64 = (locals.var_temp1 * locals.var_niratio);
        (locals.var_neff_op, locals.var_neff_op_dn4, locals.var_neff_op_dn6, locals.var_neff_op_dn7, locals.var_neff_op_dn8, locals.var_neff_op_dn9, ) = (assign43800_e48997, (locals.var_temp1_dn4 * locals.var_niratio), (locals.var_temp1_dn6 * locals.var_niratio), (locals.var_temp1_dn7 * locals.var_niratio), (locals.var_temp1_dn8 * locals.var_niratio), (locals.var_temp1_dn9 * locals.var_niratio), );
        locals.var_neff_op_rv = 0.0;
        let assign43810_e49002: f64 = (locals.var_ct_i * locals.var_tkr);
        let assign43810_e49004: f64 = (assign43810_e49002 / locals.var_tkd);
        let assign43810_e49005: f64 = (1.0 + assign43810_e49004);
        let assign43810_e49006: f64 = (locals.var_inv_phit0_op / assign43810_e49005);
        (locals.var_inv_phit_op, locals.var_inv_phit_op_dn4, locals.var_inv_phit_op_dn6, locals.var_inv_phit_op_dn7, locals.var_inv_phit_op_dn8, locals.var_inv_phit_op_dn9, ) = (assign43810_e49006, (((locals.var_inv_phit0_op_dn4 * assign43810_e49005) - (locals.var_inv_phit0_op * (-((assign43810_e49002 * locals.var_tkd_dn4) / (locals.var_tkd * locals.var_tkd))))) / (assign43810_e49005 * assign43810_e49005)), (((locals.var_inv_phit0_op_dn6 * assign43810_e49005) - (locals.var_inv_phit0_op * (-((assign43810_e49002 * locals.var_tkd_dn6) / (locals.var_tkd * locals.var_tkd))))) / (assign43810_e49005 * assign43810_e49005)), (((locals.var_inv_phit0_op_dn7 * assign43810_e49005) - (locals.var_inv_phit0_op * (-((assign43810_e49002 * locals.var_tkd_dn7) / (locals.var_tkd * locals.var_tkd))))) / (assign43810_e49005 * assign43810_e49005)), (((locals.var_inv_phit0_op_dn8 * assign43810_e49005) - (locals.var_inv_phit0_op * (-((assign43810_e49002 * locals.var_tkd_dn8) / (locals.var_tkd * locals.var_tkd))))) / (assign43810_e49005 * assign43810_e49005)), (((locals.var_inv_phit0_op_dn9 * assign43810_e49005) - (locals.var_inv_phit0_op * (-((assign43810_e49002 * locals.var_tkd_dn9) / (locals.var_tkd * locals.var_tkd))))) / (assign43810_e49005 * assign43810_e49005)), );
        locals.var_inv_phit_op_rv = 0.0;
        let assign43820_e49009: f64 = (2.0 * 1.602176565e-19);
        let assign43820_e49011: f64 = (assign43820_e49009 * locals.var_neff_op);
        let assign43820_e49013: f64 = (assign43820_e49011 * locals.var_epsch);
        let assign43820_e49015: f64 = (assign43820_e49013 * locals.var_inv_phit_op);
        (locals.var_a0_csisq_op, locals.var_a0_csisq_op_dn4, locals.var_a0_csisq_op_dn6, locals.var_a0_csisq_op_dn7, locals.var_a0_csisq_op_dn8, locals.var_a0_csisq_op_dn9, ) = (assign43820_e49015, ((((assign43820_e49009 * locals.var_neff_op_dn4) * locals.var_epsch) * locals.var_inv_phit_op) + (assign43820_e49013 * locals.var_inv_phit_op_dn4)), ((((assign43820_e49009 * locals.var_neff_op_dn6) * locals.var_epsch) * locals.var_inv_phit_op) + (assign43820_e49013 * locals.var_inv_phit_op_dn6)), ((((assign43820_e49009 * locals.var_neff_op_dn7) * locals.var_epsch) * locals.var_inv_phit_op) + (assign43820_e49013 * locals.var_inv_phit_op_dn7)), ((((assign43820_e49009 * locals.var_neff_op_dn8) * locals.var_epsch) * locals.var_inv_phit_op) + (assign43820_e49013 * locals.var_inv_phit_op_dn8)), ((((assign43820_e49009 * locals.var_neff_op_dn9) * locals.var_epsch) * locals.var_inv_phit_op) + (assign43820_e49013 * locals.var_inv_phit_op_dn9)), );
        locals.var_a0_csisq_op_rv = 0.0;
        let assign43830_e49018: f64 = (locals.var_csiprime_0 * locals.var_csiprime_0);
        let assign43830_e49020: f64 = (assign43830_e49018 / locals.var_a0_csisq_op);
        let assign43830_e49021: f64 = (assign43830_e49020).ln();
        let assign43830_e49023: f64 = (assign43830_e49021 - 0.6931471805599);
        let assign43830_e49025: f64 = (assign43830_e49023 + locals.var_eg_2phit0_op);
        (locals.var_xth_1d_op, locals.var_xth_1d_op_dn4, locals.var_xth_1d_op_dn6, locals.var_xth_1d_op_dn7, locals.var_xth_1d_op_dn8, locals.var_xth_1d_op_dn9, ) = (assign43830_e49025, (((-((assign43830_e49018 * locals.var_a0_csisq_op_dn4) / (locals.var_a0_csisq_op * locals.var_a0_csisq_op))) / assign43830_e49020) + locals.var_eg_2phit0_op_dn4), (((-((assign43830_e49018 * locals.var_a0_csisq_op_dn6) / (locals.var_a0_csisq_op * locals.var_a0_csisq_op))) / assign43830_e49020) + locals.var_eg_2phit0_op_dn6), (((-((assign43830_e49018 * locals.var_a0_csisq_op_dn7) / (locals.var_a0_csisq_op * locals.var_a0_csisq_op))) / assign43830_e49020) + locals.var_eg_2phit0_op_dn7), (((-((assign43830_e49018 * locals.var_a0_csisq_op_dn8) / (locals.var_a0_csisq_op * locals.var_a0_csisq_op))) / assign43830_e49020) + locals.var_eg_2phit0_op_dn8), (((-((assign43830_e49018 * locals.var_a0_csisq_op_dn9) / (locals.var_a0_csisq_op * locals.var_a0_csisq_op))) / assign43830_e49020) + locals.var_eg_2phit0_op_dn9), );
        locals.var_xth_1d_op_rv = 0.0;
        let assign43840_e49028: f64 = (0.5 * 1.602176565e-19);
        let assign43840_e49030: f64 = (assign43840_e49028 * locals.var_nsddc_i);
        let assign43840_e49032: f64 = (assign43840_e49030 * locals.var_tsi_i);
        let assign43840_e49035: f64 = (locals.var_cox1prime + locals.var_cox2prime);
        let assign43840_e49036: f64 = (assign43840_e49032 / assign43840_e49035);
        let assign43840_e49038: f64 = (assign43840_e49036 * locals.var_inv_phit_op);
        (locals.var_xsddep_op, locals.var_xsddep_op_dn4, locals.var_xsddep_op_dn6, locals.var_xsddep_op_dn7, locals.var_xsddep_op_dn8, locals.var_xsddep_op_dn9, ) = (assign43840_e49038, (assign43840_e49036 * locals.var_inv_phit_op_dn4), (assign43840_e49036 * locals.var_inv_phit_op_dn6), (assign43840_e49036 * locals.var_inv_phit_op_dn7), (assign43840_e49036 * locals.var_inv_phit_op_dn8), (assign43840_e49036 * locals.var_inv_phit_op_dn9), );
        locals.var_xsddep_op_rv = 0.0;
        let assign43850_e49041: f64 = (locals.var_cfd_i * locals.var_inv_phit_op);
        (locals.var_xd0_op, locals.var_xd0_op_dn4, locals.var_xd0_op_dn6, locals.var_xd0_op_dn7, locals.var_xd0_op_dn8, locals.var_xd0_op_dn9, ) = (assign43850_e49041, (locals.var_cfd_i * locals.var_inv_phit_op_dn4), (locals.var_cfd_i * locals.var_inv_phit_op_dn6), (locals.var_cfd_i * locals.var_inv_phit_op_dn7), (locals.var_cfd_i * locals.var_inv_phit_op_dn8), (locals.var_cfd_i * locals.var_inv_phit_op_dn9), );
        locals.var_xd0_op_rv = 0.0;
        (locals.var_qq_op, locals.var_qq_op_dn4, locals.var_qq_op_dn6, locals.var_qq_op_dn7, locals.var_qq_op_dn8, locals.var_qq_op_dn9, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
        locals.var_qq_op_rv = 0.0;
        (locals.var_dvfbpdep_op, locals.var_dvfbpdep_op_dn4, locals.var_dvfbpdep_op_dn6, locals.var_dvfbpdep_op_dn7, locals.var_dvfbpdep_op_dn8, locals.var_dvfbpdep_op_dn9, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
        locals.var_dvfbpdep_op_rv = 0.0;
        let assign43880_e49046: f64 = if p.p9 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1350 = assign43880_e49046;
        locals.var_guard1350_rv = 0.0;
        if (locals.var_guard1350 != 0.0) {
            let assign43890_e49050: f64 = (1.0 / locals.var_inv_phit0_op);
            let assign43890_e49053: f64 = (locals.var_np_i / locals.var_neff_poly);
            let assign43890_e49054: f64 = (assign43890_e49053).ln();
            let assign43890_e49055: f64 = (assign43890_e49050 * assign43890_e49054);
            (locals.var_dvfbpdep_op, locals.var_dvfbpdep_op_dn4, locals.var_dvfbpdep_op_dn6, locals.var_dvfbpdep_op_dn7, locals.var_dvfbpdep_op_dn8, locals.var_dvfbpdep_op_dn9, ) = (assign43890_e49055, (((-(locals.var_inv_phit0_op_dn4 / (locals.var_inv_phit0_op * locals.var_inv_phit0_op))) * assign43890_e49054) + (assign43890_e49050 * ((((locals.var_np_i_dn4 * locals.var_neff_poly) - (locals.var_np_i * locals.var_neff_poly_dn4)) / (locals.var_neff_poly * locals.var_neff_poly)) / assign43890_e49053))), (((-(locals.var_inv_phit0_op_dn6 / (locals.var_inv_phit0_op * locals.var_inv_phit0_op))) * assign43890_e49054) + (assign43890_e49050 * ((((locals.var_np_i_dn6 * locals.var_neff_poly) - (locals.var_np_i * locals.var_neff_poly_dn6)) / (locals.var_neff_poly * locals.var_neff_poly)) / assign43890_e49053))), (((-(locals.var_inv_phit0_op_dn7 / (locals.var_inv_phit0_op * locals.var_inv_phit0_op))) * assign43890_e49054) + (assign43890_e49050 * ((((locals.var_np_i_dn7 * locals.var_neff_poly) - (locals.var_np_i * locals.var_neff_poly_dn7)) / (locals.var_neff_poly * locals.var_neff_poly)) / assign43890_e49053))), (((-(locals.var_inv_phit0_op_dn8 / (locals.var_inv_phit0_op * locals.var_inv_phit0_op))) * assign43890_e49054) + (assign43890_e49050 * ((((locals.var_np_i_dn8 * locals.var_neff_poly) - (locals.var_np_i * locals.var_neff_poly_dn8)) / (locals.var_neff_poly * locals.var_neff_poly)) / assign43890_e49053))), (((-(locals.var_inv_phit0_op_dn9 / (locals.var_inv_phit0_op * locals.var_inv_phit0_op))) * assign43890_e49054) + (assign43890_e49050 * ((((locals.var_np_i_dn9 * locals.var_neff_poly) - (locals.var_np_i * locals.var_neff_poly_dn9)) / (locals.var_neff_poly * locals.var_neff_poly)) / assign43890_e49053))), );
            locals.var_dvfbpdep_op_rv = 0.0;
        }
        let assign43900_e49060: f64 = if p.p13 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1351 = assign43900_e49060;
        locals.var_guard1351_rv = 0.0;
        let assign43910_e49063: f64 = 1.0;
        let assign43910_e49064: f64 = if p.p14 == assign43910_e49063 { 1.0 } else { 0.0 };
        locals.var_guard1352 = assign43910_e49064;
        locals.var_guard1352_rv = 0.0;
        if ((locals.var_guard1351 != 0.0) && (locals.var_guard1352 != 0.0)) {
            let assign43920_e49070: f64 = (0.4 * p.p13);
            let assign43920_e49072: f64 = (assign43920_e49070 * 1.27520989);
            let assign43920_e49074: f64 = (-0.3333333333333);
            let assign43920_e49077: f64 = (locals.var_tsisq / locals.var_inv_phit_op);
            let assign43920_e49078: f64 = (assign43920_e49077).ln();
            let assign43920_e49079: f64 = (assign43920_e49074 * assign43920_e49078);
            let assign43920_e49080: f64 = (assign43920_e49079).exp();
            let assign43920_e49081: f64 = (assign43920_e49072 * assign43920_e49080);
            (locals.var_qq_op, locals.var_qq_op_dn4, locals.var_qq_op_dn6, locals.var_qq_op_dn7, locals.var_qq_op_dn8, locals.var_qq_op_dn9, ) = (assign43920_e49081, (assign43920_e49072 * (assign43920_e49080 * (assign43920_e49074 * ((-((locals.var_tsisq * locals.var_inv_phit_op_dn4) / (locals.var_inv_phit_op * locals.var_inv_phit_op))) / assign43920_e49077)))), (assign43920_e49072 * (assign43920_e49080 * (assign43920_e49074 * ((-((locals.var_tsisq * locals.var_inv_phit_op_dn6) / (locals.var_inv_phit_op * locals.var_inv_phit_op))) / assign43920_e49077)))), (assign43920_e49072 * (assign43920_e49080 * (assign43920_e49074 * ((-((locals.var_tsisq * locals.var_inv_phit_op_dn7) / (locals.var_inv_phit_op * locals.var_inv_phit_op))) / assign43920_e49077)))), (assign43920_e49072 * (assign43920_e49080 * (assign43920_e49074 * ((-((locals.var_tsisq * locals.var_inv_phit_op_dn8) / (locals.var_inv_phit_op * locals.var_inv_phit_op))) / assign43920_e49077)))), (assign43920_e49072 * (assign43920_e49080 * (assign43920_e49074 * ((-((locals.var_tsisq * locals.var_inv_phit_op_dn9) / (locals.var_inv_phit_op * locals.var_inv_phit_op))) / assign43920_e49077)))), );
            locals.var_qq_op_rv = 0.0;
        }
        if ((locals.var_guard1351 != 0.0) && (locals.var_guard1352 == 0.0)) {
            let assign43930_e49090: f64 = (0.4 * p.p13);
            let assign43930_e49092: f64 = (assign43930_e49090 * 1.5412087);
            let assign43930_e49094: f64 = (-0.3333333333333);
            let assign43930_e49097: f64 = (locals.var_tsisq / locals.var_inv_phit_op);
            let assign43930_e49098: f64 = (assign43930_e49097).ln();
            let assign43930_e49099: f64 = (assign43930_e49094 * assign43930_e49098);
            let assign43930_e49100: f64 = (assign43930_e49099).exp();
            let assign43930_e49101: f64 = (assign43930_e49092 * assign43930_e49100);
            (locals.var_qq_op, locals.var_qq_op_dn4, locals.var_qq_op_dn6, locals.var_qq_op_dn7, locals.var_qq_op_dn8, locals.var_qq_op_dn9, ) = (assign43930_e49101, (assign43930_e49092 * (assign43930_e49100 * (assign43930_e49094 * ((-((locals.var_tsisq * locals.var_inv_phit_op_dn4) / (locals.var_inv_phit_op * locals.var_inv_phit_op))) / assign43930_e49097)))), (assign43930_e49092 * (assign43930_e49100 * (assign43930_e49094 * ((-((locals.var_tsisq * locals.var_inv_phit_op_dn6) / (locals.var_inv_phit_op * locals.var_inv_phit_op))) / assign43930_e49097)))), (assign43930_e49092 * (assign43930_e49100 * (assign43930_e49094 * ((-((locals.var_tsisq * locals.var_inv_phit_op_dn7) / (locals.var_inv_phit_op * locals.var_inv_phit_op))) / assign43930_e49097)))), (assign43930_e49092 * (assign43930_e49100 * (assign43930_e49094 * ((-((locals.var_tsisq * locals.var_inv_phit_op_dn8) / (locals.var_inv_phit_op * locals.var_inv_phit_op))) / assign43930_e49097)))), (assign43930_e49092 * (assign43930_e49100 * (assign43930_e49094 * ((-((locals.var_tsisq * locals.var_inv_phit_op_dn9) / (locals.var_inv_phit_op * locals.var_inv_phit_op))) / assign43930_e49097)))), );
            locals.var_qq_op_rv = 0.0;
        }
        let assign43940_e49106: f64 = (locals.var_vds * locals.var_inv_phit_op);
        (locals.var_xd_op, locals.var_xd_op_dn4, locals.var_xd_op_dn6, locals.var_xd_op_dn7, locals.var_xd_op_dn8, locals.var_xd_op_dn9, ) = (assign43940_e49106, (locals.var_vds * locals.var_inv_phit_op_dn4), ((locals.var_vds_dn6 * locals.var_inv_phit_op) + (locals.var_vds * locals.var_inv_phit_op_dn6)), ((locals.var_vds_dn7 * locals.var_inv_phit_op) + (locals.var_vds * locals.var_inv_phit_op_dn7)), (locals.var_vds * locals.var_inv_phit_op_dn8), (locals.var_vds * locals.var_inv_phit_op_dn9), );
        locals.var_xd_op_rv = 0.0;
        let assign43950_e49109: f64 = (locals.var_vds * locals.var_vds);
        let assign43950_e49111: f64 = (assign43950_e49109 + 0.01);
        let assign43950_e49112: f64 = (assign43950_e49111).sqrt();
        let assign43950_e49114: f64 = (assign43950_e49112 - 0.1);
        let assign43950_e49116: f64 = (assign43950_e49114 * locals.var_inv_phit_op);
        (locals.var_xdsx_op, locals.var_xdsx_op_dn4, locals.var_xdsx_op_dn6, locals.var_xdsx_op_dn7, locals.var_xdsx_op_dn8, locals.var_xdsx_op_dn9, ) = (assign43950_e49116, (assign43950_e49114 * locals.var_inv_phit_op_dn4), (((((locals.var_vds_dn6 * locals.var_vds) + (locals.var_vds * locals.var_vds_dn6)) / (2.0 * assign43950_e49112)) * locals.var_inv_phit_op) + (assign43950_e49114 * locals.var_inv_phit_op_dn6)), (((((locals.var_vds_dn7 * locals.var_vds) + (locals.var_vds * locals.var_vds_dn7)) / (2.0 * assign43950_e49112)) * locals.var_inv_phit_op) + (assign43950_e49114 * locals.var_inv_phit_op_dn7)), (assign43950_e49114 * locals.var_inv_phit_op_dn8), (assign43950_e49114 * locals.var_inv_phit_op_dn9), );
        locals.var_xdsx_op_rv = 0.0;
        let assign43960_e49120: f64 = (locals.var_xd_op - locals.var_xdsx_op);
        let assign43960_e49121: f64 = (0.5 * assign43960_e49120);
        (locals.var_dxdsx_op, locals.var_dxdsx_op_dn4, locals.var_dxdsx_op_dn6, locals.var_dxdsx_op_dn7, locals.var_dxdsx_op_dn8, locals.var_dxdsx_op_dn9, ) = (assign43960_e49121, (0.5 * (locals.var_xd_op_dn4 - locals.var_xdsx_op_dn4)), (0.5 * (locals.var_xd_op_dn6 - locals.var_xdsx_op_dn6)), (0.5 * (locals.var_xd_op_dn7 - locals.var_xdsx_op_dn7)), (0.5 * (locals.var_xd_op_dn8 - locals.var_xdsx_op_dn8)), (0.5 * (locals.var_xd_op_dn9 - locals.var_xdsx_op_dn9)), );
        locals.var_dxdsx_op_rv = 0.0;
        let assign43970_e49124: f64 = (locals.var_k2_dc / locals.var_k1_dc);
        let assign43970_e49127: f64 = (1.0 + locals.var_k2_dc);
        let assign43970_e49128: f64 = (assign43970_e49124 / assign43970_e49127);
        (locals.var_r1init_op, locals.var_r1init_op_dn4, locals.var_r1init_op_dn6, locals.var_r1init_op_dn7, locals.var_r1init_op_dn8, locals.var_r1init_op_dn9, ) = (assign43970_e49128, ((((((locals.var_k2_dc_dn4 * locals.var_k1_dc) - (locals.var_k2_dc * locals.var_k1_dc_dn4)) / (locals.var_k1_dc * locals.var_k1_dc)) * assign43970_e49127) - (assign43970_e49124 * locals.var_k2_dc_dn4)) / (assign43970_e49127 * assign43970_e49127)), ((((((locals.var_k2_dc_dn6 * locals.var_k1_dc) - (locals.var_k2_dc * locals.var_k1_dc_dn6)) / (locals.var_k1_dc * locals.var_k1_dc)) * assign43970_e49127) - (assign43970_e49124 * locals.var_k2_dc_dn6)) / (assign43970_e49127 * assign43970_e49127)), ((((((locals.var_k2_dc_dn7 * locals.var_k1_dc) - (locals.var_k2_dc * locals.var_k1_dc_dn7)) / (locals.var_k1_dc * locals.var_k1_dc)) * assign43970_e49127) - (assign43970_e49124 * locals.var_k2_dc_dn7)) / (assign43970_e49127 * assign43970_e49127)), ((((((locals.var_k2_dc_dn8 * locals.var_k1_dc) - (locals.var_k2_dc * locals.var_k1_dc_dn8)) / (locals.var_k1_dc * locals.var_k1_dc)) * assign43970_e49127) - (assign43970_e49124 * locals.var_k2_dc_dn8)) / (assign43970_e49127 * assign43970_e49127)), ((((((locals.var_k2_dc_dn9 * locals.var_k1_dc) - (locals.var_k2_dc * locals.var_k1_dc_dn9)) / (locals.var_k1_dc * locals.var_k1_dc)) * assign43970_e49127) - (assign43970_e49124 * locals.var_k2_dc_dn9)) / (assign43970_e49127 * assign43970_e49127)), );
        locals.var_r1init_op_rv = 0.0;
        let assign43980_e49131: f64 = (locals.var_k1_dc / locals.var_k2_dc);
        let assign43980_e49134: f64 = (1.0 + locals.var_k1_dc);
        let assign43980_e49135: f64 = (assign43980_e49131 / assign43980_e49134);
        (locals.var_r2init_op, locals.var_r2init_op_dn4, locals.var_r2init_op_dn6, locals.var_r2init_op_dn7, locals.var_r2init_op_dn8, locals.var_r2init_op_dn9, ) = (assign43980_e49135, ((((((locals.var_k1_dc_dn4 * locals.var_k2_dc) - (locals.var_k1_dc * locals.var_k2_dc_dn4)) / (locals.var_k2_dc * locals.var_k2_dc)) * assign43980_e49134) - (assign43980_e49131 * locals.var_k1_dc_dn4)) / (assign43980_e49134 * assign43980_e49134)), ((((((locals.var_k1_dc_dn6 * locals.var_k2_dc) - (locals.var_k1_dc * locals.var_k2_dc_dn6)) / (locals.var_k2_dc * locals.var_k2_dc)) * assign43980_e49134) - (assign43980_e49131 * locals.var_k1_dc_dn6)) / (assign43980_e49134 * assign43980_e49134)), ((((((locals.var_k1_dc_dn7 * locals.var_k2_dc) - (locals.var_k1_dc * locals.var_k2_dc_dn7)) / (locals.var_k2_dc * locals.var_k2_dc)) * assign43980_e49134) - (assign43980_e49131 * locals.var_k1_dc_dn7)) / (assign43980_e49134 * assign43980_e49134)), ((((((locals.var_k1_dc_dn8 * locals.var_k2_dc) - (locals.var_k1_dc * locals.var_k2_dc_dn8)) / (locals.var_k2_dc * locals.var_k2_dc)) * assign43980_e49134) - (assign43980_e49131 * locals.var_k1_dc_dn8)) / (assign43980_e49134 * assign43980_e49134)), ((((((locals.var_k1_dc_dn9 * locals.var_k2_dc) - (locals.var_k1_dc * locals.var_k2_dc_dn9)) / (locals.var_k2_dc * locals.var_k2_dc)) * assign43980_e49134) - (assign43980_e49131 * locals.var_k1_dc_dn9)) / (assign43980_e49134 * assign43980_e49134)), );
        locals.var_r2init_op_rv = 0.0;
        let assign43990_e49139: f64 = (1.0 + locals.var_r1init_op);
        let assign43990_e49140: f64 = (locals.var_k1_dc * assign43990_e49139);
        let assign43990_e49142: f64 = (assign43990_e49140 * locals.var_diff_min_dc);
        let assign43990_e49144: f64 = (assign43990_e49142 / locals.var_a0_dc);
        let assign43990_e49145: f64 = (assign43990_e49144).ln();
        let assign43990_e49147: f64 = (assign43990_e49145 + 2.0);
        (locals.var_x1init_op, locals.var_x1init_op_dn4, locals.var_x1init_op_dn6, locals.var_x1init_op_dn7, locals.var_x1init_op_dn8, locals.var_x1init_op_dn9, ) = (assign43990_e49147, ((((((((locals.var_k1_dc_dn4 * assign43990_e49139) + (locals.var_k1_dc * locals.var_r1init_op_dn4)) * locals.var_diff_min_dc) + (assign43990_e49140 * locals.var_diff_min_dc_dn4)) * locals.var_a0_dc) - (assign43990_e49142 * locals.var_a0_dc_dn4)) / (locals.var_a0_dc * locals.var_a0_dc)) / assign43990_e49144), ((((((((locals.var_k1_dc_dn6 * assign43990_e49139) + (locals.var_k1_dc * locals.var_r1init_op_dn6)) * locals.var_diff_min_dc) + (assign43990_e49140 * locals.var_diff_min_dc_dn6)) * locals.var_a0_dc) - (assign43990_e49142 * locals.var_a0_dc_dn6)) / (locals.var_a0_dc * locals.var_a0_dc)) / assign43990_e49144), ((((((((locals.var_k1_dc_dn7 * assign43990_e49139) + (locals.var_k1_dc * locals.var_r1init_op_dn7)) * locals.var_diff_min_dc) + (assign43990_e49140 * locals.var_diff_min_dc_dn7)) * locals.var_a0_dc) - (assign43990_e49142 * locals.var_a0_dc_dn7)) / (locals.var_a0_dc * locals.var_a0_dc)) / assign43990_e49144), ((((((((locals.var_k1_dc_dn8 * assign43990_e49139) + (locals.var_k1_dc * locals.var_r1init_op_dn8)) * locals.var_diff_min_dc) + (assign43990_e49140 * locals.var_diff_min_dc_dn8)) * locals.var_a0_dc) - (assign43990_e49142 * locals.var_a0_dc_dn8)) / (locals.var_a0_dc * locals.var_a0_dc)) / assign43990_e49144), ((((((((locals.var_k1_dc_dn9 * assign43990_e49139) + (locals.var_k1_dc * locals.var_r1init_op_dn9)) * locals.var_diff_min_dc) + (assign43990_e49140 * locals.var_diff_min_dc_dn9)) * locals.var_a0_dc) - (assign43990_e49142 * locals.var_a0_dc_dn9)) / (locals.var_a0_dc * locals.var_a0_dc)) / assign43990_e49144), );
        locals.var_x1init_op_rv = 0.0;
        let assign44000_e49151: f64 = (1.0 + locals.var_r2init_op);
        let assign44000_e49152: f64 = (locals.var_k2_dc * assign44000_e49151);
        let assign44000_e49154: f64 = (assign44000_e49152 * locals.var_diff_min_dc);
        let assign44000_e49156: f64 = (assign44000_e49154 / locals.var_a0_dc);
        let assign44000_e49157: f64 = (assign44000_e49156).ln();
        let assign44000_e49159: f64 = (assign44000_e49157 + 2.0);
        (locals.var_x2init_op, locals.var_x2init_op_dn4, locals.var_x2init_op_dn6, locals.var_x2init_op_dn7, locals.var_x2init_op_dn8, locals.var_x2init_op_dn9, ) = (assign44000_e49159, ((((((((locals.var_k2_dc_dn4 * assign44000_e49151) + (locals.var_k2_dc * locals.var_r2init_op_dn4)) * locals.var_diff_min_dc) + (assign44000_e49152 * locals.var_diff_min_dc_dn4)) * locals.var_a0_dc) - (assign44000_e49154 * locals.var_a0_dc_dn4)) / (locals.var_a0_dc * locals.var_a0_dc)) / assign44000_e49156), ((((((((locals.var_k2_dc_dn6 * assign44000_e49151) + (locals.var_k2_dc * locals.var_r2init_op_dn6)) * locals.var_diff_min_dc) + (assign44000_e49152 * locals.var_diff_min_dc_dn6)) * locals.var_a0_dc) - (assign44000_e49154 * locals.var_a0_dc_dn6)) / (locals.var_a0_dc * locals.var_a0_dc)) / assign44000_e49156), ((((((((locals.var_k2_dc_dn7 * assign44000_e49151) + (locals.var_k2_dc * locals.var_r2init_op_dn7)) * locals.var_diff_min_dc) + (assign44000_e49152 * locals.var_diff_min_dc_dn7)) * locals.var_a0_dc) - (assign44000_e49154 * locals.var_a0_dc_dn7)) / (locals.var_a0_dc * locals.var_a0_dc)) / assign44000_e49156), ((((((((locals.var_k2_dc_dn8 * assign44000_e49151) + (locals.var_k2_dc * locals.var_r2init_op_dn8)) * locals.var_diff_min_dc) + (assign44000_e49152 * locals.var_diff_min_dc_dn8)) * locals.var_a0_dc) - (assign44000_e49154 * locals.var_a0_dc_dn8)) / (locals.var_a0_dc * locals.var_a0_dc)) / assign44000_e49156), ((((((((locals.var_k2_dc_dn9 * assign44000_e49151) + (locals.var_k2_dc * locals.var_r2init_op_dn9)) * locals.var_diff_min_dc) + (assign44000_e49152 * locals.var_diff_min_dc_dn9)) * locals.var_a0_dc) - (assign44000_e49154 * locals.var_a0_dc_dn9)) / (locals.var_a0_dc * locals.var_a0_dc)) / assign44000_e49156), );
        locals.var_x2init_op_rv = 0.0;
        let assign44010_e49162: f64 = (1.0 + locals.var_r1init_op);
        let assign44010_e49164: f64 = (assign44010_e49162 * locals.var_x1init_op);
        let assign44010_e49167: f64 = (locals.var_xg2x_dc * locals.var_r1init_op);
        let assign44010_e49168: f64 = (assign44010_e49164 - assign44010_e49167);
        (locals.var_xth1init_op, locals.var_xth1init_op_dn4, locals.var_xth1init_op_dn6, locals.var_xth1init_op_dn7, locals.var_xth1init_op_dn8, locals.var_xth1init_op_dn9, ) = (assign44010_e49168, (((locals.var_r1init_op_dn4 * locals.var_x1init_op) + (assign44010_e49162 * locals.var_x1init_op_dn4)) - ((locals.var_xg2x_dc_dn4 * locals.var_r1init_op) + (locals.var_xg2x_dc * locals.var_r1init_op_dn4))), (((locals.var_r1init_op_dn6 * locals.var_x1init_op) + (assign44010_e49162 * locals.var_x1init_op_dn6)) - ((locals.var_xg2x_dc_dn6 * locals.var_r1init_op) + (locals.var_xg2x_dc * locals.var_r1init_op_dn6))), (((locals.var_r1init_op_dn7 * locals.var_x1init_op) + (assign44010_e49162 * locals.var_x1init_op_dn7)) - ((locals.var_xg2x_dc_dn7 * locals.var_r1init_op) + (locals.var_xg2x_dc * locals.var_r1init_op_dn7))), (((locals.var_r1init_op_dn8 * locals.var_x1init_op) + (assign44010_e49162 * locals.var_x1init_op_dn8)) - ((locals.var_xg2x_dc_dn8 * locals.var_r1init_op) + (locals.var_xg2x_dc * locals.var_r1init_op_dn8))), (((locals.var_r1init_op_dn9 * locals.var_x1init_op) + (assign44010_e49162 * locals.var_x1init_op_dn9)) - ((locals.var_xg2x_dc_dn9 * locals.var_r1init_op) + (locals.var_xg2x_dc * locals.var_r1init_op_dn9))), );
        locals.var_xth1init_op_rv = 0.0;
        let assign44020_e49172: f64 = (1.0 / locals.var_r2init_op);
        let assign44020_e49173: f64 = (1.0 + assign44020_e49172);
        let assign44020_e49175: f64 = (assign44020_e49173 * locals.var_x2init_op);
        let assign44020_e49178: f64 = (locals.var_xg2x_dc / locals.var_r2init_op);
        let assign44020_e49179: f64 = (assign44020_e49175 - assign44020_e49178);
        (locals.var_xth2init_op, locals.var_xth2init_op_dn4, locals.var_xth2init_op_dn6, locals.var_xth2init_op_dn7, locals.var_xth2init_op_dn8, locals.var_xth2init_op_dn9, ) = (assign44020_e49179, ((((-(locals.var_r2init_op_dn4 / (locals.var_r2init_op * locals.var_r2init_op))) * locals.var_x2init_op) + (assign44020_e49173 * locals.var_x2init_op_dn4)) - (((locals.var_xg2x_dc_dn4 * locals.var_r2init_op) - (locals.var_xg2x_dc * locals.var_r2init_op_dn4)) / (locals.var_r2init_op * locals.var_r2init_op))), ((((-(locals.var_r2init_op_dn6 / (locals.var_r2init_op * locals.var_r2init_op))) * locals.var_x2init_op) + (assign44020_e49173 * locals.var_x2init_op_dn6)) - (((locals.var_xg2x_dc_dn6 * locals.var_r2init_op) - (locals.var_xg2x_dc * locals.var_r2init_op_dn6)) / (locals.var_r2init_op * locals.var_r2init_op))), ((((-(locals.var_r2init_op_dn7 / (locals.var_r2init_op * locals.var_r2init_op))) * locals.var_x2init_op) + (assign44020_e49173 * locals.var_x2init_op_dn7)) - (((locals.var_xg2x_dc_dn7 * locals.var_r2init_op) - (locals.var_xg2x_dc * locals.var_r2init_op_dn7)) / (locals.var_r2init_op * locals.var_r2init_op))), ((((-(locals.var_r2init_op_dn8 / (locals.var_r2init_op * locals.var_r2init_op))) * locals.var_x2init_op) + (assign44020_e49173 * locals.var_x2init_op_dn8)) - (((locals.var_xg2x_dc_dn8 * locals.var_r2init_op) - (locals.var_xg2x_dc * locals.var_r2init_op_dn8)) / (locals.var_r2init_op * locals.var_r2init_op))), ((((-(locals.var_r2init_op_dn9 / (locals.var_r2init_op * locals.var_r2init_op))) * locals.var_x2init_op) + (assign44020_e49173 * locals.var_x2init_op_dn9)) - (((locals.var_xg2x_dc_dn9 * locals.var_r2init_op) - (locals.var_xg2x_dc * locals.var_r2init_op_dn9)) / (locals.var_r2init_op * locals.var_r2init_op))), );
        locals.var_xth2init_op_rv = 0.0;
        let assign44030_e49183: f64 = (locals.var_xth1init_op + locals.var_xth2init_op);
        let assign44030_e49186: f64 = (locals.var_xth1init_op - locals.var_xth2init_op);
        let assign44030_e49189: f64 = (locals.var_xth1init_op - locals.var_xth2init_op);
        let assign44030_e49190: f64 = (assign44030_e49186 * assign44030_e49189);
        let assign44030_e49192: f64 = (assign44030_e49190 + 38.0);
        let assign44030_e49193: f64 = (assign44030_e49192).sqrt();
        let assign44030_e49194: f64 = (assign44030_e49183 - assign44030_e49193);
        let assign44030_e49195: f64 = (0.5 * assign44030_e49194);
        let assign44030_e49197: f64 = (assign44030_e49195 - locals.var_xg2_dc);
        let assign44030_e49199: f64 = (assign44030_e49197 / locals.var_cic1_i);
        let assign44030_e49201: f64 = (assign44030_e49199 + locals.var_xg2_dc);
        (locals.var_xg1thinit_op, locals.var_xg1thinit_op_dn4, locals.var_xg1thinit_op_dn6, locals.var_xg1thinit_op_dn7, locals.var_xg1thinit_op_dn8, locals.var_xg1thinit_op_dn9, ) = (assign44030_e49201, ((((0.5 * ((locals.var_xth1init_op_dn4 + locals.var_xth2init_op_dn4) - ((((locals.var_xth1init_op_dn4 - locals.var_xth2init_op_dn4) * assign44030_e49189) + (assign44030_e49186 * (locals.var_xth1init_op_dn4 - locals.var_xth2init_op_dn4))) / (2.0 * assign44030_e49193)))) - locals.var_xg2_dc_dn4) / locals.var_cic1_i) + locals.var_xg2_dc_dn4), ((((0.5 * ((locals.var_xth1init_op_dn6 + locals.var_xth2init_op_dn6) - ((((locals.var_xth1init_op_dn6 - locals.var_xth2init_op_dn6) * assign44030_e49189) + (assign44030_e49186 * (locals.var_xth1init_op_dn6 - locals.var_xth2init_op_dn6))) / (2.0 * assign44030_e49193)))) - locals.var_xg2_dc_dn6) / locals.var_cic1_i) + locals.var_xg2_dc_dn6), ((((0.5 * ((locals.var_xth1init_op_dn7 + locals.var_xth2init_op_dn7) - ((((locals.var_xth1init_op_dn7 - locals.var_xth2init_op_dn7) * assign44030_e49189) + (assign44030_e49186 * (locals.var_xth1init_op_dn7 - locals.var_xth2init_op_dn7))) / (2.0 * assign44030_e49193)))) - locals.var_xg2_dc_dn7) / locals.var_cic1_i) + locals.var_xg2_dc_dn7), ((((0.5 * ((locals.var_xth1init_op_dn8 + locals.var_xth2init_op_dn8) - ((((locals.var_xth1init_op_dn8 - locals.var_xth2init_op_dn8) * assign44030_e49189) + (assign44030_e49186 * (locals.var_xth1init_op_dn8 - locals.var_xth2init_op_dn8))) / (2.0 * assign44030_e49193)))) - locals.var_xg2_dc_dn8) / locals.var_cic1_i) + locals.var_xg2_dc_dn8), ((((0.5 * ((locals.var_xth1init_op_dn9 + locals.var_xth2init_op_dn9) - ((((locals.var_xth1init_op_dn9 - locals.var_xth2init_op_dn9) * assign44030_e49189) + (assign44030_e49186 * (locals.var_xth1init_op_dn9 - locals.var_xth2init_op_dn9))) / (2.0 * assign44030_e49193)))) - locals.var_xg2_dc_dn9) / locals.var_cic1_i) + locals.var_xg2_dc_dn9), );
        locals.var_xg1thinit_op_rv = 0.0;
        let assign44040_e49205: f64 = (locals.var_xg1thinit_op - locals.var_xedge_dc);
        let assign44040_e49207: f64 = (assign44040_e49205 / locals.var_sce1_dc);
        let assign44040_e49209: f64 = (assign44040_e49207 - locals.var_dxg1_dibl_dc);
        let assign44040_e49211: f64 = (assign44040_e49209 + locals.var_xedge_dc);
        let assign44040_e49212: f64 = (locals.var_phit * assign44040_e49211);
        let assign44040_e49214: f64 = (assign44040_e49212 + locals.var_vfb1_i);
        (locals.var_vthinit_op, locals.var_vthinit_op_dn4, locals.var_vthinit_op_dn6, locals.var_vthinit_op_dn7, locals.var_vthinit_op_dn8, locals.var_vthinit_op_dn9, ) = (assign44040_e49214, (((locals.var_phit_dn4 * assign44040_e49211) + (locals.var_phit * ((((((locals.var_xg1thinit_op_dn4 - locals.var_xedge_dc_dn4) * locals.var_sce1_dc) - (assign44040_e49205 * locals.var_sce1_dc_dn4)) / (locals.var_sce1_dc * locals.var_sce1_dc)) - locals.var_dxg1_dibl_dc_dn4) + locals.var_xedge_dc_dn4))) + locals.var_vfb1_i_dn4), (((locals.var_phit_dn6 * assign44040_e49211) + (locals.var_phit * ((((((locals.var_xg1thinit_op_dn6 - locals.var_xedge_dc_dn6) * locals.var_sce1_dc) - (assign44040_e49205 * locals.var_sce1_dc_dn6)) / (locals.var_sce1_dc * locals.var_sce1_dc)) - locals.var_dxg1_dibl_dc_dn6) + locals.var_xedge_dc_dn6))) + locals.var_vfb1_i_dn6), (((locals.var_phit_dn7 * assign44040_e49211) + (locals.var_phit * ((((((locals.var_xg1thinit_op_dn7 - locals.var_xedge_dc_dn7) * locals.var_sce1_dc) - (assign44040_e49205 * locals.var_sce1_dc_dn7)) / (locals.var_sce1_dc * locals.var_sce1_dc)) - locals.var_dxg1_dibl_dc_dn7) + locals.var_xedge_dc_dn7))) + locals.var_vfb1_i_dn7), (((locals.var_phit_dn8 * assign44040_e49211) + (locals.var_phit * ((((((locals.var_xg1thinit_op_dn8 - locals.var_xedge_dc_dn8) * locals.var_sce1_dc) - (assign44040_e49205 * locals.var_sce1_dc_dn8)) / (locals.var_sce1_dc * locals.var_sce1_dc)) - locals.var_dxg1_dibl_dc_dn8) + locals.var_xedge_dc_dn8))) + locals.var_vfb1_i_dn8), (((locals.var_phit_dn9 * assign44040_e49211) + (locals.var_phit * ((((((locals.var_xg1thinit_op_dn9 - locals.var_xedge_dc_dn9) * locals.var_sce1_dc) - (assign44040_e49205 * locals.var_sce1_dc_dn9)) / (locals.var_sce1_dc * locals.var_sce1_dc)) - locals.var_dxg1_dibl_dc_dn9) + locals.var_xedge_dc_dn9))) + locals.var_vfb1_i_dn9), );
        locals.var_vthinit_op_rv = 0.0;
        let assign44050_e49218: f64 = (locals.var_tkd - locals.var_tkr);
        let assign44050_e49219: f64 = (locals.var_stcf_i * assign44050_e49218);
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign44050_e49219, ((locals.var_stcf_i_dn4 * assign44050_e49218) + (locals.var_stcf_i * locals.var_tkd_dn4)), ((locals.var_stcf_i_dn6 * assign44050_e49218) + (locals.var_stcf_i * locals.var_tkd_dn6)), ((locals.var_stcf_i_dn7 * assign44050_e49218) + (locals.var_stcf_i * locals.var_tkd_dn7)), ((locals.var_stcf_i_dn8 * assign44050_e49218) + (locals.var_stcf_i * locals.var_tkd_dn8)), ((locals.var_stcf_i_dn9 * assign44050_e49218) + (locals.var_stcf_i * locals.var_tkd_dn9)), );
        locals.var_temp_rv = 0.0;
        let assign44080_e49228: f64 = (p.p14 * locals.var_stvfb_i);
        let assign44080_e49231: f64 = (locals.var_tkd - locals.var_tkr);
        let assign44080_e49232: f64 = (assign44080_e49228 * assign44080_e49231);
        let assign44080_e49234: f64 = (assign44080_e49232 + locals.var_dvfbqm);
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign44080_e49234, (assign44080_e49228 * locals.var_tkd_dn4), (assign44080_e49228 * locals.var_tkd_dn6), (assign44080_e49228 * locals.var_tkd_dn7), (assign44080_e49228 * locals.var_tkd_dn8), (assign44080_e49228 * locals.var_tkd_dn9), );
        locals.var_temp_rv = 0.0;
        let assign44090_e49238: f64 = (locals.var_vfb1_t + locals.var_dvfbch_op);
        let assign44090_e49240: f64 = (assign44090_e49238 + locals.var_dvfb1nch);
        let assign44090_e49241: f64 = (p.p14 * assign44090_e49240);
        let assign44090_e49243: f64 = (assign44090_e49241 + locals.var_temp);
        let assign44090_e49245: f64 = (assign44090_e49243 + p.p34);
        let assign44090_e49247: f64 = (assign44090_e49245 - locals.var_dvfbpdep_op);
        (locals.var_vfb1_op, locals.var_vfb1_op_dn4, locals.var_vfb1_op_dn6, locals.var_vfb1_op_dn7, locals.var_vfb1_op_dn8, locals.var_vfb1_op_dn9, ) = (assign44090_e49247, (((p.p14 * ((locals.var_vfb1_t_dn4 + locals.var_dvfbch_op_dn4) + locals.var_dvfb1nch_dn4)) + locals.var_temp_dn4) - locals.var_dvfbpdep_op_dn4), (((p.p14 * ((locals.var_vfb1_t_dn6 + locals.var_dvfbch_op_dn6) + locals.var_dvfb1nch_dn6)) + locals.var_temp_dn6) - locals.var_dvfbpdep_op_dn6), (((p.p14 * ((locals.var_vfb1_t_dn7 + locals.var_dvfbch_op_dn7) + locals.var_dvfb1nch_dn7)) + locals.var_temp_dn7) - locals.var_dvfbpdep_op_dn7), (((p.p14 * ((locals.var_vfb1_t_dn8 + locals.var_dvfbch_op_dn8) + locals.var_dvfb1nch_dn8)) + locals.var_temp_dn8) - locals.var_dvfbpdep_op_dn8), (((p.p14 * ((locals.var_vfb1_t_dn9 + locals.var_dvfbch_op_dn9) + locals.var_dvfb1nch_dn9)) + locals.var_temp_dn9) - locals.var_dvfbpdep_op_dn9), );
        locals.var_vfb1_op_rv = 0.0;
        let assign44100_e49251: f64 = (locals.var_vfb2_t + locals.var_dvfbch_op);
        let assign44100_e49253: f64 = (assign44100_e49251 + locals.var_dvfb2nch);
        let assign44100_e49254: f64 = (p.p14 * assign44100_e49253);
        let assign44100_e49256: f64 = (assign44100_e49254 + locals.var_temp);
        (locals.var_vfb2_op, locals.var_vfb2_op_dn4, locals.var_vfb2_op_dn6, locals.var_vfb2_op_dn7, locals.var_vfb2_op_dn8, locals.var_vfb2_op_dn9, ) = (assign44100_e49256, ((p.p14 * ((locals.var_vfb2_t_dn4 + locals.var_dvfbch_op_dn4) + locals.var_dvfb2nch_dn4)) + locals.var_temp_dn4), ((p.p14 * ((locals.var_vfb2_t_dn6 + locals.var_dvfbch_op_dn6) + locals.var_dvfb2nch_dn6)) + locals.var_temp_dn6), ((p.p14 * ((locals.var_vfb2_t_dn7 + locals.var_dvfbch_op_dn7) + locals.var_dvfb2nch_dn7)) + locals.var_temp_dn7), ((p.p14 * ((locals.var_vfb2_t_dn8 + locals.var_dvfbch_op_dn8) + locals.var_dvfb2nch_dn8)) + locals.var_temp_dn8), ((p.p14 * ((locals.var_vfb2_t_dn9 + locals.var_dvfbch_op_dn9) + locals.var_dvfb2nch_dn9)) + locals.var_temp_dn9), );
        locals.var_vfb2_op_rv = 0.0;
        let assign44110_e49259: f64 = (locals.var_vthinit_op - locals.var_vfb1_op);
        let assign44110_e49261: f64 = (assign44110_e49259 * locals.var_inv_phit_op);
        let assign44110_e49263: f64 = (assign44110_e49261 - locals.var_dxdsx_op);
        (locals.var_xg10_op, locals.var_xg10_op_dn4, locals.var_xg10_op_dn6, locals.var_xg10_op_dn7, locals.var_xg10_op_dn8, locals.var_xg10_op_dn9, ) = (assign44110_e49263, ((((locals.var_vthinit_op_dn4 - locals.var_vfb1_op_dn4) * locals.var_inv_phit_op) + (assign44110_e49259 * locals.var_inv_phit_op_dn4)) - locals.var_dxdsx_op_dn4), ((((locals.var_vthinit_op_dn6 - locals.var_vfb1_op_dn6) * locals.var_inv_phit_op) + (assign44110_e49259 * locals.var_inv_phit_op_dn6)) - locals.var_dxdsx_op_dn6), ((((locals.var_vthinit_op_dn7 - locals.var_vfb1_op_dn7) * locals.var_inv_phit_op) + (assign44110_e49259 * locals.var_inv_phit_op_dn7)) - locals.var_dxdsx_op_dn7), ((((locals.var_vthinit_op_dn8 - locals.var_vfb1_op_dn8) * locals.var_inv_phit_op) + (assign44110_e49259 * locals.var_inv_phit_op_dn8)) - locals.var_dxdsx_op_dn8), ((((locals.var_vthinit_op_dn9 - locals.var_vfb1_op_dn9) * locals.var_inv_phit_op) + (assign44110_e49259 * locals.var_inv_phit_op_dn9)) - locals.var_dxdsx_op_dn9), );
        locals.var_xg10_op_rv = 0.0;
        let assign44120_e49265: f64 = (-locals.var_vsb);
        let assign44120_e49267: f64 = (assign44120_e49265 - locals.var_vfb2_op);
        let assign44120_e49269: f64 = (assign44120_e49267 * locals.var_inv_phit_op);
        let assign44120_e49271: f64 = (assign44120_e49269 - locals.var_dxdsx_op);
        (locals.var_xg20_op, locals.var_xg20_op_dn4, locals.var_xg20_op_dn6, locals.var_xg20_op_dn7, locals.var_xg20_op_dn8, locals.var_xg20_op_dn9, ) = (assign44120_e49271, ((((-locals.var_vfb2_op_dn4) * locals.var_inv_phit_op) + (assign44120_e49267 * locals.var_inv_phit_op_dn4)) - locals.var_dxdsx_op_dn4), (((((-locals.var_vsb_dn6) - locals.var_vfb2_op_dn6) * locals.var_inv_phit_op) + (assign44120_e49267 * locals.var_inv_phit_op_dn6)) - locals.var_dxdsx_op_dn6), (((((-locals.var_vsb_dn7) - locals.var_vfb2_op_dn7) * locals.var_inv_phit_op) + (assign44120_e49267 * locals.var_inv_phit_op_dn7)) - locals.var_dxdsx_op_dn7), (((((-locals.var_vsb_dn8) - locals.var_vfb2_op_dn8) * locals.var_inv_phit_op) + (assign44120_e49267 * locals.var_inv_phit_op_dn8)) - locals.var_dxdsx_op_dn8), ((((-locals.var_vfb2_op_dn9) * locals.var_inv_phit_op) + (assign44120_e49267 * locals.var_inv_phit_op_dn9)) - locals.var_dxdsx_op_dn9), );
        locals.var_xg20_op_rv = 0.0;
        let assign44130_e49274: f64 = if p.p2 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1353 = assign44130_e49274;
        locals.var_guard1353_rv = 0.0;
        if (locals.var_guard1353 != 0.0) {
            let assign44140_e49278: f64 = (p.p14 * locals.var_typesub_i);
            let assign44140_e49281: f64 = (locals.var_xg10_op - locals.var_xg20_op);
            let assign44140_e49282: f64 = (assign44140_e49278 * assign44140_e49281);
            let assign44140_e49284: f64 = (assign44140_e49282 / locals.var_gfsub);
            (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign44140_e49284, ((((assign44140_e49278 * (locals.var_xg10_op_dn4 - locals.var_xg20_op_dn4)) * locals.var_gfsub) - (assign44140_e49282 * locals.var_gfsub_dn4)) / (locals.var_gfsub * locals.var_gfsub)), ((((assign44140_e49278 * (locals.var_xg10_op_dn6 - locals.var_xg20_op_dn6)) * locals.var_gfsub) - (assign44140_e49282 * locals.var_gfsub_dn6)) / (locals.var_gfsub * locals.var_gfsub)), ((((assign44140_e49278 * (locals.var_xg10_op_dn7 - locals.var_xg20_op_dn7)) * locals.var_gfsub) - (assign44140_e49282 * locals.var_gfsub_dn7)) / (locals.var_gfsub * locals.var_gfsub)), ((((assign44140_e49278 * (locals.var_xg10_op_dn8 - locals.var_xg20_op_dn8)) * locals.var_gfsub) - (assign44140_e49282 * locals.var_gfsub_dn8)) / (locals.var_gfsub * locals.var_gfsub)), ((((assign44140_e49278 * (locals.var_xg10_op_dn9 - locals.var_xg20_op_dn9)) * locals.var_gfsub) - (assign44140_e49282 * locals.var_gfsub_dn9)) / (locals.var_gfsub * locals.var_gfsub)), );
            locals.var_temp_rv = 0.0;
        }
        let assign44150_e49289: f64 = if locals.var_temp < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1354 = assign44150_e49289;
        locals.var_guard1354_rv = 0.0;
        if ((locals.var_guard1353 != 0.0) && (locals.var_guard1354 != 0.0)) {
            let assign44160_e49294: f64 = (-2.0);
            let assign44160_e49297: f64 = (1.0 - locals.var_temp);
            let assign44160_e49298: f64 = (assign44160_e49297).ln();
            let assign44160_e49299: f64 = (assign44160_e49294 * assign44160_e49298);
            (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9, ) = (assign44160_e49299, (assign44160_e49294 * ((-locals.var_temp_dn4) / assign44160_e49297)), (assign44160_e49294 * ((-locals.var_temp_dn6) / assign44160_e49297)), (assign44160_e49294 * ((-locals.var_temp_dn7) / assign44160_e49297)), (assign44160_e49294 * ((-locals.var_temp_dn8) / assign44160_e49297)), (assign44160_e49294 * ((-locals.var_temp_dn9) / assign44160_e49297)), );
            locals.var_temp1_rv = 0.0;
        }
        if ((locals.var_guard1353 != 0.0) && (locals.var_guard1354 == 0.0)) {
            let assign44170_e49308: f64 = (locals.var_temp * locals.var_temp);
            let assign44170_e49312: f64 = (2.0 * locals.var_temp);
            let assign44170_e49314: f64 = (assign44170_e49312 / locals.var_gfsub);
            let assign44170_e49315: f64 = (1.0 + assign44170_e49314);
            let assign44170_e49316: f64 = (assign44170_e49308 / assign44170_e49315);
            (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9, ) = (assign44170_e49316, (((((locals.var_temp_dn4 * locals.var_temp) + (locals.var_temp * locals.var_temp_dn4)) * assign44170_e49315) - (assign44170_e49308 * ((((2.0 * locals.var_temp_dn4) * locals.var_gfsub) - (assign44170_e49312 * locals.var_gfsub_dn4)) / (locals.var_gfsub * locals.var_gfsub)))) / (assign44170_e49315 * assign44170_e49315)), (((((locals.var_temp_dn6 * locals.var_temp) + (locals.var_temp * locals.var_temp_dn6)) * assign44170_e49315) - (assign44170_e49308 * ((((2.0 * locals.var_temp_dn6) * locals.var_gfsub) - (assign44170_e49312 * locals.var_gfsub_dn6)) / (locals.var_gfsub * locals.var_gfsub)))) / (assign44170_e49315 * assign44170_e49315)), (((((locals.var_temp_dn7 * locals.var_temp) + (locals.var_temp * locals.var_temp_dn7)) * assign44170_e49315) - (assign44170_e49308 * ((((2.0 * locals.var_temp_dn7) * locals.var_gfsub) - (assign44170_e49312 * locals.var_gfsub_dn7)) / (locals.var_gfsub * locals.var_gfsub)))) / (assign44170_e49315 * assign44170_e49315)), (((((locals.var_temp_dn8 * locals.var_temp) + (locals.var_temp * locals.var_temp_dn8)) * assign44170_e49315) - (assign44170_e49308 * ((((2.0 * locals.var_temp_dn8) * locals.var_gfsub) - (assign44170_e49312 * locals.var_gfsub_dn8)) / (locals.var_gfsub * locals.var_gfsub)))) / (assign44170_e49315 * assign44170_e49315)), (((((locals.var_temp_dn9 * locals.var_temp) + (locals.var_temp * locals.var_temp_dn9)) * assign44170_e49315) - (assign44170_e49308 * ((((2.0 * locals.var_temp_dn9) * locals.var_gfsub) - (assign44170_e49312 * locals.var_gfsub_dn9)) / (locals.var_gfsub * locals.var_gfsub)))) / (assign44170_e49315 * assign44170_e49315)), );
            locals.var_temp1_rv = 0.0;
        }
        if (locals.var_guard1353 != 0.0) {
            let assign44180_e49323: f64 = (p.p14 * locals.var_typesub_i);
            let assign44180_e49325: f64 = (assign44180_e49323 * locals.var_temp1);
            let assign44180_e49326: f64 = (locals.var_xg20_op + assign44180_e49325);
            (locals.var_xg2eff_op, locals.var_xg2eff_op_dn4, locals.var_xg2eff_op_dn6, locals.var_xg2eff_op_dn7, locals.var_xg2eff_op_dn8, locals.var_xg2eff_op_dn9, ) = (assign44180_e49326, (locals.var_xg20_op_dn4 + (assign44180_e49323 * locals.var_temp1_dn4)), (locals.var_xg20_op_dn6 + (assign44180_e49323 * locals.var_temp1_dn6)), (locals.var_xg20_op_dn7 + (assign44180_e49323 * locals.var_temp1_dn7)), (locals.var_xg20_op_dn8 + (assign44180_e49323 * locals.var_temp1_dn8)), (locals.var_xg20_op_dn9 + (assign44180_e49323 * locals.var_temp1_dn9)), );
            locals.var_xg2eff_op_rv = 0.0;
        }
        if (locals.var_guard1353 == 0.0) {
            (locals.var_xg2eff_op, locals.var_xg2eff_op_dn4, locals.var_xg2eff_op_dn6, locals.var_xg2eff_op_dn7, locals.var_xg2eff_op_dn8, locals.var_xg2eff_op_dn9, ) = (locals.var_xg20_op, locals.var_xg20_op_dn4, locals.var_xg20_op_dn6, locals.var_xg20_op_dn7, locals.var_xg20_op_dn8, locals.var_xg20_op_dn9, );
            locals.var_xg2eff_op_rv = 0.0;
        }
        let assign44200_e49337: f64 = (locals.var_xg10_op - locals.var_xg2eff_op);
        let assign44200_e49338: f64 = (locals.var_keq_1d * assign44200_e49337);
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign44200_e49338, (locals.var_keq_1d * (locals.var_xg10_op_dn4 - locals.var_xg2eff_op_dn4)), (locals.var_keq_1d * (locals.var_xg10_op_dn6 - locals.var_xg2eff_op_dn6)), (locals.var_keq_1d * (locals.var_xg10_op_dn7 - locals.var_xg2eff_op_dn7)), (locals.var_keq_1d * (locals.var_xg10_op_dn8 - locals.var_xg2eff_op_dn8)), (locals.var_keq_1d * (locals.var_xg10_op_dn9 - locals.var_xg2eff_op_dn9)), );
        locals.var_temp_rv = 0.0;
        let assign44210_e49341: f64 = if p.p13 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1355 = assign44210_e49341;
        locals.var_guard1355_rv = 0.0;
        if (locals.var_guard1355 != 0.0) {
            let assign44220_e49346: f64 = (locals.var_temp + locals.var_emin);
            let assign44220_e49349: f64 = (locals.var_temp - locals.var_emin);
            let assign44220_e49352: f64 = (locals.var_temp - locals.var_emin);
            let assign44220_e49353: f64 = (assign44220_e49349 * assign44220_e49352);
            let assign44220_e49356: f64 = (locals.var_emin * locals.var_emin);
            let assign44220_e49357: f64 = (assign44220_e49353 + assign44220_e49356);
            let assign44220_e49358: f64 = (assign44220_e49357).sqrt();
            let assign44220_e49359: f64 = (assign44220_e49346 + assign44220_e49358);
            let assign44220_e49360: f64 = (0.5 * assign44220_e49359);
            (locals.var_e1_op, locals.var_e1_op_dn4, locals.var_e1_op_dn6, locals.var_e1_op_dn7, locals.var_e1_op_dn8, locals.var_e1_op_dn9, ) = (assign44220_e49360, (0.5 * ((locals.var_temp_dn4 + locals.var_emin_dn4) + (((((locals.var_temp_dn4 - locals.var_emin_dn4) * assign44220_e49352) + (assign44220_e49349 * (locals.var_temp_dn4 - locals.var_emin_dn4))) + ((locals.var_emin_dn4 * locals.var_emin) + (locals.var_emin * locals.var_emin_dn4))) / (2.0 * assign44220_e49358)))), (0.5 * ((locals.var_temp_dn6 + locals.var_emin_dn6) + (((((locals.var_temp_dn6 - locals.var_emin_dn6) * assign44220_e49352) + (assign44220_e49349 * (locals.var_temp_dn6 - locals.var_emin_dn6))) + ((locals.var_emin_dn6 * locals.var_emin) + (locals.var_emin * locals.var_emin_dn6))) / (2.0 * assign44220_e49358)))), (0.5 * ((locals.var_temp_dn7 + locals.var_emin_dn7) + (((((locals.var_temp_dn7 - locals.var_emin_dn7) * assign44220_e49352) + (assign44220_e49349 * (locals.var_temp_dn7 - locals.var_emin_dn7))) + ((locals.var_emin_dn7 * locals.var_emin) + (locals.var_emin * locals.var_emin_dn7))) / (2.0 * assign44220_e49358)))), (0.5 * ((locals.var_temp_dn8 + locals.var_emin_dn8) + (((((locals.var_temp_dn8 - locals.var_emin_dn8) * assign44220_e49352) + (assign44220_e49349 * (locals.var_temp_dn8 - locals.var_emin_dn8))) + ((locals.var_emin_dn8 * locals.var_emin) + (locals.var_emin * locals.var_emin_dn8))) / (2.0 * assign44220_e49358)))), (0.5 * ((locals.var_temp_dn9 + locals.var_emin_dn9) + (((((locals.var_temp_dn9 - locals.var_emin_dn9) * assign44220_e49352) + (assign44220_e49349 * (locals.var_temp_dn9 - locals.var_emin_dn9))) + ((locals.var_emin_dn9 * locals.var_emin) + (locals.var_emin * locals.var_emin_dn9))) / (2.0 * assign44220_e49358)))), );
            locals.var_e1_op_rv = 0.0;
        }
        if (locals.var_guard1355 != 0.0) {
            let assign44230_e49366: f64 = (-locals.var_temp);
            let assign44230_e49368: f64 = (assign44230_e49366 + locals.var_emin);
            let assign44230_e49370: f64 = (-locals.var_temp);
            let assign44230_e49372: f64 = (assign44230_e49370 - locals.var_emin);
            let assign44230_e49374: f64 = (-locals.var_temp);
            let assign44230_e49376: f64 = (assign44230_e49374 - locals.var_emin);
            let assign44230_e49377: f64 = (assign44230_e49372 * assign44230_e49376);
            let assign44230_e49380: f64 = (locals.var_emin * locals.var_emin);
            let assign44230_e49381: f64 = (assign44230_e49377 + assign44230_e49380);
            let assign44230_e49382: f64 = (assign44230_e49381).sqrt();
            let assign44230_e49383: f64 = (assign44230_e49368 + assign44230_e49382);
            let assign44230_e49384: f64 = (0.5 * assign44230_e49383);
            (locals.var_e2_op, locals.var_e2_op_dn4, locals.var_e2_op_dn6, locals.var_e2_op_dn7, locals.var_e2_op_dn8, locals.var_e2_op_dn9, ) = (assign44230_e49384, (0.5 * (((-locals.var_temp_dn4) + locals.var_emin_dn4) + ((((((-locals.var_temp_dn4) - locals.var_emin_dn4) * assign44230_e49376) + (assign44230_e49372 * ((-locals.var_temp_dn4) - locals.var_emin_dn4))) + ((locals.var_emin_dn4 * locals.var_emin) + (locals.var_emin * locals.var_emin_dn4))) / (2.0 * assign44230_e49382)))), (0.5 * (((-locals.var_temp_dn6) + locals.var_emin_dn6) + ((((((-locals.var_temp_dn6) - locals.var_emin_dn6) * assign44230_e49376) + (assign44230_e49372 * ((-locals.var_temp_dn6) - locals.var_emin_dn6))) + ((locals.var_emin_dn6 * locals.var_emin) + (locals.var_emin * locals.var_emin_dn6))) / (2.0 * assign44230_e49382)))), (0.5 * (((-locals.var_temp_dn7) + locals.var_emin_dn7) + ((((((-locals.var_temp_dn7) - locals.var_emin_dn7) * assign44230_e49376) + (assign44230_e49372 * ((-locals.var_temp_dn7) - locals.var_emin_dn7))) + ((locals.var_emin_dn7 * locals.var_emin) + (locals.var_emin * locals.var_emin_dn7))) / (2.0 * assign44230_e49382)))), (0.5 * (((-locals.var_temp_dn8) + locals.var_emin_dn8) + ((((((-locals.var_temp_dn8) - locals.var_emin_dn8) * assign44230_e49376) + (assign44230_e49372 * ((-locals.var_temp_dn8) - locals.var_emin_dn8))) + ((locals.var_emin_dn8 * locals.var_emin) + (locals.var_emin * locals.var_emin_dn8))) / (2.0 * assign44230_e49382)))), (0.5 * (((-locals.var_temp_dn9) + locals.var_emin_dn9) + ((((((-locals.var_temp_dn9) - locals.var_emin_dn9) * assign44230_e49376) + (assign44230_e49372 * ((-locals.var_temp_dn9) - locals.var_emin_dn9))) + ((locals.var_emin_dn9 * locals.var_emin) + (locals.var_emin * locals.var_emin_dn9))) / (2.0 * assign44230_e49382)))), );
            locals.var_e2_op_rv = 0.0;
        }
        if (locals.var_guard1355 != 0.0) {
            let assign44240_e49390: f64 = (-0.3333333333333);
            let assign44240_e49392: f64 = (locals.var_e1_op).ln();
            let assign44240_e49393: f64 = (assign44240_e49390 * assign44240_e49392);
            let assign44240_e49394: f64 = (assign44240_e49393).exp();
            let assign44240_e49395: f64 = (locals.var_qq_op * assign44240_e49394);
            (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9, ) = (assign44240_e49395, ((locals.var_qq_op_dn4 * assign44240_e49394) + (locals.var_qq_op * (assign44240_e49394 * (assign44240_e49390 * (locals.var_e1_op_dn4 / locals.var_e1_op))))), ((locals.var_qq_op_dn6 * assign44240_e49394) + (locals.var_qq_op * (assign44240_e49394 * (assign44240_e49390 * (locals.var_e1_op_dn6 / locals.var_e1_op))))), ((locals.var_qq_op_dn7 * assign44240_e49394) + (locals.var_qq_op * (assign44240_e49394 * (assign44240_e49390 * (locals.var_e1_op_dn7 / locals.var_e1_op))))), ((locals.var_qq_op_dn8 * assign44240_e49394) + (locals.var_qq_op * (assign44240_e49394 * (assign44240_e49390 * (locals.var_e1_op_dn8 / locals.var_e1_op))))), ((locals.var_qq_op_dn9 * assign44240_e49394) + (locals.var_qq_op * (assign44240_e49394 * (assign44240_e49390 * (locals.var_e1_op_dn9 / locals.var_e1_op))))), );
            locals.var_temp1_rv = 0.0;
        }
        if (locals.var_guard1355 != 0.0) {
            let assign44250_e49401: f64 = (-0.3333333333333);
            let assign44250_e49403: f64 = (locals.var_e2_op).ln();
            let assign44250_e49404: f64 = (assign44250_e49401 * assign44250_e49403);
            let assign44250_e49405: f64 = (assign44250_e49404).exp();
            let assign44250_e49406: f64 = (locals.var_qq_op * assign44250_e49405);
            (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9, ) = (assign44250_e49406, ((locals.var_qq_op_dn4 * assign44250_e49405) + (locals.var_qq_op * (assign44250_e49405 * (assign44250_e49401 * (locals.var_e2_op_dn4 / locals.var_e2_op))))), ((locals.var_qq_op_dn6 * assign44250_e49405) + (locals.var_qq_op * (assign44250_e49405 * (assign44250_e49401 * (locals.var_e2_op_dn6 / locals.var_e2_op))))), ((locals.var_qq_op_dn7 * assign44250_e49405) + (locals.var_qq_op * (assign44250_e49405 * (assign44250_e49401 * (locals.var_e2_op_dn7 / locals.var_e2_op))))), ((locals.var_qq_op_dn8 * assign44250_e49405) + (locals.var_qq_op * (assign44250_e49405 * (assign44250_e49401 * (locals.var_e2_op_dn8 / locals.var_e2_op))))), ((locals.var_qq_op_dn9 * assign44250_e49405) + (locals.var_qq_op * (assign44250_e49405 * (assign44250_e49401 * (locals.var_e2_op_dn9 / locals.var_e2_op))))), );
            locals.var_temp2_rv = 0.0;
        }
        if (locals.var_guard1355 != 0.0) {
            let assign44260_e49412: f64 = (1.0 - locals.var_temp1);
            let assign44260_e49414: f64 = (assign44260_e49412 - locals.var_temp2);
            (locals.var_temp3, locals.var_temp3_dn4, locals.var_temp3_dn6, locals.var_temp3_dn7, locals.var_temp3_dn8, locals.var_temp3_dn9, ) = (assign44260_e49414, ((-locals.var_temp1_dn4) - locals.var_temp2_dn4), ((-locals.var_temp1_dn6) - locals.var_temp2_dn6), ((-locals.var_temp1_dn7) - locals.var_temp2_dn7), ((-locals.var_temp1_dn8) - locals.var_temp2_dn8), ((-locals.var_temp1_dn9) - locals.var_temp2_dn9), );
            locals.var_temp3_rv = 0.0;
        }
        if (locals.var_guard1355 != 0.0) {
            let assign44280_e49426: f64 = (locals.var_k1_1d * locals.var_temp3);
            let assign44280_e49430: f64 = (locals.var_k1_1d * locals.var_temp1);
            let assign44280_e49431: f64 = (1.0 + assign44280_e49430);
            let assign44280_e49432: f64 = (assign44280_e49426 / assign44280_e49431);
            (locals.var_k1_1d_qm_op, locals.var_k1_1d_qm_op_dn4, locals.var_k1_1d_qm_op_dn6, locals.var_k1_1d_qm_op_dn7, locals.var_k1_1d_qm_op_dn8, locals.var_k1_1d_qm_op_dn9, ) = (assign44280_e49432, ((((locals.var_k1_1d * locals.var_temp3_dn4) * assign44280_e49431) - (assign44280_e49426 * (locals.var_k1_1d * locals.var_temp1_dn4))) / (assign44280_e49431 * assign44280_e49431)), ((((locals.var_k1_1d * locals.var_temp3_dn6) * assign44280_e49431) - (assign44280_e49426 * (locals.var_k1_1d * locals.var_temp1_dn6))) / (assign44280_e49431 * assign44280_e49431)), ((((locals.var_k1_1d * locals.var_temp3_dn7) * assign44280_e49431) - (assign44280_e49426 * (locals.var_k1_1d * locals.var_temp1_dn7))) / (assign44280_e49431 * assign44280_e49431)), ((((locals.var_k1_1d * locals.var_temp3_dn8) * assign44280_e49431) - (assign44280_e49426 * (locals.var_k1_1d * locals.var_temp1_dn8))) / (assign44280_e49431 * assign44280_e49431)), ((((locals.var_k1_1d * locals.var_temp3_dn9) * assign44280_e49431) - (assign44280_e49426 * (locals.var_k1_1d * locals.var_temp1_dn9))) / (assign44280_e49431 * assign44280_e49431)), );
            locals.var_k1_1d_qm_op_rv = 0.0;
        }
        if (locals.var_guard1355 != 0.0) {
            let assign44290_e49438: f64 = (locals.var_k2_1d * locals.var_temp3);
            let assign44290_e49442: f64 = (locals.var_k2_1d * locals.var_temp2);
            let assign44290_e49443: f64 = (1.0 + assign44290_e49442);
            let assign44290_e49444: f64 = (assign44290_e49438 / assign44290_e49443);
            (locals.var_k2_1d_qm_op, locals.var_k2_1d_qm_op_dn4, locals.var_k2_1d_qm_op_dn6, locals.var_k2_1d_qm_op_dn7, locals.var_k2_1d_qm_op_dn8, locals.var_k2_1d_qm_op_dn9, ) = (assign44290_e49444, ((((locals.var_k2_1d * locals.var_temp3_dn4) * assign44290_e49443) - (assign44290_e49438 * (locals.var_k2_1d * locals.var_temp2_dn4))) / (assign44290_e49443 * assign44290_e49443)), ((((locals.var_k2_1d * locals.var_temp3_dn6) * assign44290_e49443) - (assign44290_e49438 * (locals.var_k2_1d * locals.var_temp2_dn6))) / (assign44290_e49443 * assign44290_e49443)), ((((locals.var_k2_1d * locals.var_temp3_dn7) * assign44290_e49443) - (assign44290_e49438 * (locals.var_k2_1d * locals.var_temp2_dn7))) / (assign44290_e49443 * assign44290_e49443)), ((((locals.var_k2_1d * locals.var_temp3_dn8) * assign44290_e49443) - (assign44290_e49438 * (locals.var_k2_1d * locals.var_temp2_dn8))) / (assign44290_e49443 * assign44290_e49443)), ((((locals.var_k2_1d * locals.var_temp3_dn9) * assign44290_e49443) - (assign44290_e49438 * (locals.var_k2_1d * locals.var_temp2_dn9))) / (assign44290_e49443 * assign44290_e49443)), );
            locals.var_k2_1d_qm_op_rv = 0.0;
        }
        if (locals.var_guard1355 != 0.0) {
            let assign44300_e49452: f64 = (1.0 / locals.var_k1_1d_qm_op);
            let assign44300_e49453: f64 = (1.0 + assign44300_e49452);
            let assign44300_e49456: f64 = (1.0 / locals.var_k2_1d_qm_op);
            let assign44300_e49457: f64 = (assign44300_e49453 + assign44300_e49456);
            let assign44300_e49458: f64 = (1.0 / assign44300_e49457);
            (locals.var_keq_1d_qm_op, locals.var_keq_1d_qm_op_dn4, locals.var_keq_1d_qm_op_dn6, locals.var_keq_1d_qm_op_dn7, locals.var_keq_1d_qm_op_dn8, locals.var_keq_1d_qm_op_dn9, ) = (assign44300_e49458, (-(((-(locals.var_k1_1d_qm_op_dn4 / (locals.var_k1_1d_qm_op * locals.var_k1_1d_qm_op))) + (-(locals.var_k2_1d_qm_op_dn4 / (locals.var_k2_1d_qm_op * locals.var_k2_1d_qm_op)))) / (assign44300_e49457 * assign44300_e49457))), (-(((-(locals.var_k1_1d_qm_op_dn6 / (locals.var_k1_1d_qm_op * locals.var_k1_1d_qm_op))) + (-(locals.var_k2_1d_qm_op_dn6 / (locals.var_k2_1d_qm_op * locals.var_k2_1d_qm_op)))) / (assign44300_e49457 * assign44300_e49457))), (-(((-(locals.var_k1_1d_qm_op_dn7 / (locals.var_k1_1d_qm_op * locals.var_k1_1d_qm_op))) + (-(locals.var_k2_1d_qm_op_dn7 / (locals.var_k2_1d_qm_op * locals.var_k2_1d_qm_op)))) / (assign44300_e49457 * assign44300_e49457))), (-(((-(locals.var_k1_1d_qm_op_dn8 / (locals.var_k1_1d_qm_op * locals.var_k1_1d_qm_op))) + (-(locals.var_k2_1d_qm_op_dn8 / (locals.var_k2_1d_qm_op * locals.var_k2_1d_qm_op)))) / (assign44300_e49457 * assign44300_e49457))), (-(((-(locals.var_k1_1d_qm_op_dn9 / (locals.var_k1_1d_qm_op * locals.var_k1_1d_qm_op))) + (-(locals.var_k2_1d_qm_op_dn9 / (locals.var_k2_1d_qm_op * locals.var_k2_1d_qm_op)))) / (assign44300_e49457 * assign44300_e49457))), );
            locals.var_keq_1d_qm_op_rv = 0.0;
        }
        if (locals.var_guard1355 == 0.0) {
            (locals.var_k1_1d_qm_op, locals.var_k1_1d_qm_op_dn4, locals.var_k1_1d_qm_op_dn6, locals.var_k1_1d_qm_op_dn7, locals.var_k1_1d_qm_op_dn8, locals.var_k1_1d_qm_op_dn9, ) = (locals.var_k1_1d, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_k1_1d_qm_op_rv = 0.0;
            (locals.var_k2_1d_qm_op, locals.var_k2_1d_qm_op_dn4, locals.var_k2_1d_qm_op_dn6, locals.var_k2_1d_qm_op_dn7, locals.var_k2_1d_qm_op_dn8, locals.var_k2_1d_qm_op_dn9, ) = (locals.var_k2_1d, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_k2_1d_qm_op_rv = 0.0;
            (locals.var_keq_1d_qm_op, locals.var_keq_1d_qm_op_dn4, locals.var_keq_1d_qm_op_dn6, locals.var_keq_1d_qm_op_dn7, locals.var_keq_1d_qm_op_dn8, locals.var_keq_1d_qm_op_dn9, ) = (locals.var_keq_1d, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_keq_1d_qm_op_rv = 0.0;
        }
        let assign44350_e49484: f64 = (locals.var_xg10_op - locals.var_xg2eff_op);
        let assign44350_e49485: f64 = (locals.var_keq_1d_qm_op * assign44350_e49484);
        (locals.var_dx_wi_1d_op, locals.var_dx_wi_1d_op_dn4, locals.var_dx_wi_1d_op_dn6, locals.var_dx_wi_1d_op_dn7, locals.var_dx_wi_1d_op_dn8, locals.var_dx_wi_1d_op_dn9, ) = (assign44350_e49485, ((locals.var_keq_1d_qm_op_dn4 * assign44350_e49484) + (locals.var_keq_1d_qm_op * (locals.var_xg10_op_dn4 - locals.var_xg2eff_op_dn4))), ((locals.var_keq_1d_qm_op_dn6 * assign44350_e49484) + (locals.var_keq_1d_qm_op * (locals.var_xg10_op_dn6 - locals.var_xg2eff_op_dn6))), ((locals.var_keq_1d_qm_op_dn7 * assign44350_e49484) + (locals.var_keq_1d_qm_op * (locals.var_xg10_op_dn7 - locals.var_xg2eff_op_dn7))), ((locals.var_keq_1d_qm_op_dn8 * assign44350_e49484) + (locals.var_keq_1d_qm_op * (locals.var_xg10_op_dn8 - locals.var_xg2eff_op_dn8))), ((locals.var_keq_1d_qm_op_dn9 * assign44350_e49484) + (locals.var_keq_1d_qm_op * (locals.var_xg10_op_dn9 - locals.var_xg2eff_op_dn9))), );
        locals.var_dx_wi_1d_op_rv = 0.0;
    }
    pub(super) fn stamp_reactive_block_63(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign44360_e49488: f64 = if locals.var_dx_wi_1d_op > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1356 = assign44360_e49488;
        locals.var_guard1356_rv = 0.0;
        let assign44370_e49490: f64 = (-locals.var_dx_wi_1d_op);
        let assign44370_e49492: f64 = if assign44370_e49490 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard1357 = assign44370_e49492;
        locals.var_guard1357_rv = 0.0;
        if ((locals.var_guard1356 != 0.0) && (locals.var_guard1357 != 0.0)) {
            let assign44380_e49498: f64 = (-locals.var_dx_wi_1d_op);
            let assign44380_e49499: f64 = (assign44380_e49498).exp();
            let assign44380_e49500: f64 = (1.0 + assign44380_e49499);
            let assign44380_e49501: f64 = (assign44380_e49500).ln();
            (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign44380_e49501, ((assign44380_e49499 * (-locals.var_dx_wi_1d_op_dn4)) / assign44380_e49500), ((assign44380_e49499 * (-locals.var_dx_wi_1d_op_dn6)) / assign44380_e49500), ((assign44380_e49499 * (-locals.var_dx_wi_1d_op_dn7)) / assign44380_e49500), ((assign44380_e49499 * (-locals.var_dx_wi_1d_op_dn8)) / assign44380_e49500), ((assign44380_e49499 * (-locals.var_dx_wi_1d_op_dn9)) / assign44380_e49500), );
            locals.var_temp_rv = 0.0;
        }
        if ((locals.var_guard1356 != 0.0) && (locals.var_guard1357 == 0.0)) {
            let assign44390_e49509: f64 = (-locals.var_dx_wi_1d_op);
            (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign44390_e49509, (-locals.var_dx_wi_1d_op_dn4), (-locals.var_dx_wi_1d_op_dn6), (-locals.var_dx_wi_1d_op_dn7), (-locals.var_dx_wi_1d_op_dn8), (-locals.var_dx_wi_1d_op_dn9), );
            locals.var_temp_rv = 0.0;
        }
        if (locals.var_guard1356 != 0.0) {
            let assign44400_e49516: f64 = (locals.var_dx_wi_1d_op / locals.var_k1_1d_qm_op);
            let assign44400_e49517: f64 = (locals.var_xg10_op - assign44400_e49516);
            let assign44400_e49519: f64 = (assign44400_e49517 + locals.var_temp);
            let assign44400_e49521: f64 = (assign44400_e49519 - 0.6931471805599);
            (locals.var_x_wi_1d_op, locals.var_x_wi_1d_op_dn4, locals.var_x_wi_1d_op_dn6, locals.var_x_wi_1d_op_dn7, locals.var_x_wi_1d_op_dn8, locals.var_x_wi_1d_op_dn9, ) = (assign44400_e49521, ((locals.var_xg10_op_dn4 - (((locals.var_dx_wi_1d_op_dn4 * locals.var_k1_1d_qm_op) - (locals.var_dx_wi_1d_op * locals.var_k1_1d_qm_op_dn4)) / (locals.var_k1_1d_qm_op * locals.var_k1_1d_qm_op))) + locals.var_temp_dn4), ((locals.var_xg10_op_dn6 - (((locals.var_dx_wi_1d_op_dn6 * locals.var_k1_1d_qm_op) - (locals.var_dx_wi_1d_op * locals.var_k1_1d_qm_op_dn6)) / (locals.var_k1_1d_qm_op * locals.var_k1_1d_qm_op))) + locals.var_temp_dn6), ((locals.var_xg10_op_dn7 - (((locals.var_dx_wi_1d_op_dn7 * locals.var_k1_1d_qm_op) - (locals.var_dx_wi_1d_op * locals.var_k1_1d_qm_op_dn7)) / (locals.var_k1_1d_qm_op * locals.var_k1_1d_qm_op))) + locals.var_temp_dn7), ((locals.var_xg10_op_dn8 - (((locals.var_dx_wi_1d_op_dn8 * locals.var_k1_1d_qm_op) - (locals.var_dx_wi_1d_op * locals.var_k1_1d_qm_op_dn8)) / (locals.var_k1_1d_qm_op * locals.var_k1_1d_qm_op))) + locals.var_temp_dn8), ((locals.var_xg10_op_dn9 - (((locals.var_dx_wi_1d_op_dn9 * locals.var_k1_1d_qm_op) - (locals.var_dx_wi_1d_op * locals.var_k1_1d_qm_op_dn9)) / (locals.var_k1_1d_qm_op * locals.var_k1_1d_qm_op))) + locals.var_temp_dn9), );
            locals.var_x_wi_1d_op_rv = 0.0;
        }
        let assign44410_e49526: f64 = if locals.var_dx_wi_1d_op < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard1358 = assign44410_e49526;
        locals.var_guard1358_rv = 0.0;
        if ((locals.var_guard1356 == 0.0) && (locals.var_guard1358 != 0.0)) {
            let assign44420_e49533: f64 = (locals.var_dx_wi_1d_op).exp();
            let assign44420_e49534: f64 = (1.0 + assign44420_e49533);
            let assign44420_e49535: f64 = (assign44420_e49534).ln();
            (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign44420_e49535, ((assign44420_e49533 * locals.var_dx_wi_1d_op_dn4) / assign44420_e49534), ((assign44420_e49533 * locals.var_dx_wi_1d_op_dn6) / assign44420_e49534), ((assign44420_e49533 * locals.var_dx_wi_1d_op_dn7) / assign44420_e49534), ((assign44420_e49533 * locals.var_dx_wi_1d_op_dn8) / assign44420_e49534), ((assign44420_e49533 * locals.var_dx_wi_1d_op_dn9) / assign44420_e49534), );
            locals.var_temp_rv = 0.0;
        }
        if ((locals.var_guard1356 == 0.0) && (locals.var_guard1358 == 0.0)) {
            (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (locals.var_dx_wi_1d_op, locals.var_dx_wi_1d_op_dn4, locals.var_dx_wi_1d_op_dn6, locals.var_dx_wi_1d_op_dn7, locals.var_dx_wi_1d_op_dn8, locals.var_dx_wi_1d_op_dn9, );
            locals.var_temp_rv = 0.0;
        }
        if (locals.var_guard1356 == 0.0) {
            let assign44440_e49551: f64 = (locals.var_dx_wi_1d_op / locals.var_k2_1d_qm_op);
            let assign44440_e49552: f64 = (locals.var_xg2eff_op + assign44440_e49551);
            let assign44440_e49554: f64 = (assign44440_e49552 + locals.var_temp);
            let assign44440_e49556: f64 = (assign44440_e49554 - 0.6931471805599);
            (locals.var_x_wi_1d_op, locals.var_x_wi_1d_op_dn4, locals.var_x_wi_1d_op_dn6, locals.var_x_wi_1d_op_dn7, locals.var_x_wi_1d_op_dn8, locals.var_x_wi_1d_op_dn9, ) = (assign44440_e49556, ((locals.var_xg2eff_op_dn4 + (((locals.var_dx_wi_1d_op_dn4 * locals.var_k2_1d_qm_op) - (locals.var_dx_wi_1d_op * locals.var_k2_1d_qm_op_dn4)) / (locals.var_k2_1d_qm_op * locals.var_k2_1d_qm_op))) + locals.var_temp_dn4), ((locals.var_xg2eff_op_dn6 + (((locals.var_dx_wi_1d_op_dn6 * locals.var_k2_1d_qm_op) - (locals.var_dx_wi_1d_op * locals.var_k2_1d_qm_op_dn6)) / (locals.var_k2_1d_qm_op * locals.var_k2_1d_qm_op))) + locals.var_temp_dn6), ((locals.var_xg2eff_op_dn7 + (((locals.var_dx_wi_1d_op_dn7 * locals.var_k2_1d_qm_op) - (locals.var_dx_wi_1d_op * locals.var_k2_1d_qm_op_dn7)) / (locals.var_k2_1d_qm_op * locals.var_k2_1d_qm_op))) + locals.var_temp_dn7), ((locals.var_xg2eff_op_dn8 + (((locals.var_dx_wi_1d_op_dn8 * locals.var_k2_1d_qm_op) - (locals.var_dx_wi_1d_op * locals.var_k2_1d_qm_op_dn8)) / (locals.var_k2_1d_qm_op * locals.var_k2_1d_qm_op))) + locals.var_temp_dn8), ((locals.var_xg2eff_op_dn9 + (((locals.var_dx_wi_1d_op_dn9 * locals.var_k2_1d_qm_op) - (locals.var_dx_wi_1d_op * locals.var_k2_1d_qm_op_dn9)) / (locals.var_k2_1d_qm_op * locals.var_k2_1d_qm_op))) + locals.var_temp_dn9), );
            locals.var_x_wi_1d_op_rv = 0.0;
        }
        let assign44450_e49562: f64 = (locals.var_x_wi_1d_op + locals.var_xth_1d_op);
        let assign44450_e49565: f64 = (locals.var_x_wi_1d_op - locals.var_xth_1d_op);
        let assign44450_e49568: f64 = (locals.var_x_wi_1d_op - locals.var_xth_1d_op);
        let assign44450_e49569: f64 = (assign44450_e49565 * assign44450_e49568);
        let assign44450_e49571: f64 = (assign44450_e49569 + 4.0);
        let assign44450_e49572: f64 = (assign44450_e49571).sqrt();
        let assign44450_e49573: f64 = (assign44450_e49562 - assign44450_e49572);
        let assign44450_e49574: f64 = (0.5 * assign44450_e49573);
        (locals.var_x_1d_op, locals.var_x_1d_op_dn4, locals.var_x_1d_op_dn6, locals.var_x_1d_op_dn7, locals.var_x_1d_op_dn8, locals.var_x_1d_op_dn9, ) = (assign44450_e49574, (0.5 * ((locals.var_x_wi_1d_op_dn4 + locals.var_xth_1d_op_dn4) - ((((locals.var_x_wi_1d_op_dn4 - locals.var_xth_1d_op_dn4) * assign44450_e49568) + (assign44450_e49565 * (locals.var_x_wi_1d_op_dn4 - locals.var_xth_1d_op_dn4))) / (2.0 * assign44450_e49572)))), (0.5 * ((locals.var_x_wi_1d_op_dn6 + locals.var_xth_1d_op_dn6) - ((((locals.var_x_wi_1d_op_dn6 - locals.var_xth_1d_op_dn6) * assign44450_e49568) + (assign44450_e49565 * (locals.var_x_wi_1d_op_dn6 - locals.var_xth_1d_op_dn6))) / (2.0 * assign44450_e49572)))), (0.5 * ((locals.var_x_wi_1d_op_dn7 + locals.var_xth_1d_op_dn7) - ((((locals.var_x_wi_1d_op_dn7 - locals.var_xth_1d_op_dn7) * assign44450_e49568) + (assign44450_e49565 * (locals.var_x_wi_1d_op_dn7 - locals.var_xth_1d_op_dn7))) / (2.0 * assign44450_e49572)))), (0.5 * ((locals.var_x_wi_1d_op_dn8 + locals.var_xth_1d_op_dn8) - ((((locals.var_x_wi_1d_op_dn8 - locals.var_xth_1d_op_dn8) * assign44450_e49568) + (assign44450_e49565 * (locals.var_x_wi_1d_op_dn8 - locals.var_xth_1d_op_dn8))) / (2.0 * assign44450_e49572)))), (0.5 * ((locals.var_x_wi_1d_op_dn9 + locals.var_xth_1d_op_dn9) - ((((locals.var_x_wi_1d_op_dn9 - locals.var_xth_1d_op_dn9) * assign44450_e49568) + (assign44450_e49565 * (locals.var_x_wi_1d_op_dn9 - locals.var_xth_1d_op_dn9))) / (2.0 * assign44450_e49572)))), );
        locals.var_x_1d_op_rv = 0.0;
        let assign44460_e49579: f64 = (locals.var_xth_1d_op - locals.var_x_1d_op);
        let assign44460_e49580: f64 = (2.0 * assign44460_e49579);
        let assign44460_e49582: f64 = (assign44460_e49580 / locals.var_xsddep_op);
        let assign44460_e49583: f64 = (1.0 + assign44460_e49582);
        let assign44460_e49584: f64 = (assign44460_e49583).sqrt();
        let assign44460_e49586: f64 = (assign44460_e49584 - 1.0);
        (locals.var_dleff_op, locals.var_dleff_op_dn4, locals.var_dleff_op_dn6, locals.var_dleff_op_dn7, locals.var_dleff_op_dn8, locals.var_dleff_op_dn9, ) = (assign44460_e49586, (((((2.0 * (locals.var_xth_1d_op_dn4 - locals.var_x_1d_op_dn4)) * locals.var_xsddep_op) - (assign44460_e49580 * locals.var_xsddep_op_dn4)) / (locals.var_xsddep_op * locals.var_xsddep_op)) / (2.0 * assign44460_e49584)), (((((2.0 * (locals.var_xth_1d_op_dn6 - locals.var_x_1d_op_dn6)) * locals.var_xsddep_op) - (assign44460_e49580 * locals.var_xsddep_op_dn6)) / (locals.var_xsddep_op * locals.var_xsddep_op)) / (2.0 * assign44460_e49584)), (((((2.0 * (locals.var_xth_1d_op_dn7 - locals.var_x_1d_op_dn7)) * locals.var_xsddep_op) - (assign44460_e49580 * locals.var_xsddep_op_dn7)) / (locals.var_xsddep_op * locals.var_xsddep_op)) / (2.0 * assign44460_e49584)), (((((2.0 * (locals.var_xth_1d_op_dn8 - locals.var_x_1d_op_dn8)) * locals.var_xsddep_op) - (assign44460_e49580 * locals.var_xsddep_op_dn8)) / (locals.var_xsddep_op * locals.var_xsddep_op)) / (2.0 * assign44460_e49584)), (((((2.0 * (locals.var_xth_1d_op_dn9 - locals.var_x_1d_op_dn9)) * locals.var_xsddep_op) - (assign44460_e49580 * locals.var_xsddep_op_dn9)) / (locals.var_xsddep_op * locals.var_xsddep_op)) / (2.0 * assign44460_e49584)), );
        locals.var_dleff_op_rv = 0.0;
        let assign44480_e49596: f64 = (locals.var_pscedlb_i * locals.var_xg20_op);
        let assign44480_e49597: f64 = (1.0 + assign44480_e49596);
        let assign44480_e49599: f64 = (assign44480_e49597 + 0.5);
        let assign44480_e49603: f64 = (locals.var_pscedlb_i * locals.var_xg20_op);
        let assign44480_e49604: f64 = (1.0 + assign44480_e49603);
        let assign44480_e49606: f64 = (assign44480_e49604 - 0.5);
        let assign44480_e49610: f64 = (locals.var_pscedlb_i * locals.var_xg20_op);
        let assign44480_e49611: f64 = (1.0 + assign44480_e49610);
        let assign44480_e49613: f64 = (assign44480_e49611 - 0.5);
        let assign44480_e49614: f64 = (assign44480_e49606 * assign44480_e49613);
        let assign44480_e49616: f64 = (assign44480_e49614 + 0.01);
        let assign44480_e49617: f64 = (assign44480_e49616).sqrt();
        let assign44480_e49618: f64 = (assign44480_e49599 + assign44480_e49617);
        let assign44480_e49619: f64 = (0.5 * assign44480_e49618);
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign44480_e49619, (0.5 * ((locals.var_pscedlb_i * locals.var_xg20_op_dn4) + ((((locals.var_pscedlb_i * locals.var_xg20_op_dn4) * assign44480_e49613) + (assign44480_e49606 * (locals.var_pscedlb_i * locals.var_xg20_op_dn4))) / (2.0 * assign44480_e49617)))), (0.5 * ((locals.var_pscedlb_i * locals.var_xg20_op_dn6) + ((((locals.var_pscedlb_i * locals.var_xg20_op_dn6) * assign44480_e49613) + (assign44480_e49606 * (locals.var_pscedlb_i * locals.var_xg20_op_dn6))) / (2.0 * assign44480_e49617)))), (0.5 * ((locals.var_pscedlb_i * locals.var_xg20_op_dn7) + ((((locals.var_pscedlb_i * locals.var_xg20_op_dn7) * assign44480_e49613) + (assign44480_e49606 * (locals.var_pscedlb_i * locals.var_xg20_op_dn7))) / (2.0 * assign44480_e49617)))), (0.5 * ((locals.var_pscedlb_i * locals.var_xg20_op_dn8) + ((((locals.var_pscedlb_i * locals.var_xg20_op_dn8) * assign44480_e49613) + (assign44480_e49606 * (locals.var_pscedlb_i * locals.var_xg20_op_dn8))) / (2.0 * assign44480_e49617)))), (0.5 * ((locals.var_pscedlb_i * locals.var_xg20_op_dn9) + ((((locals.var_pscedlb_i * locals.var_xg20_op_dn9) * assign44480_e49613) + (assign44480_e49606 * (locals.var_pscedlb_i * locals.var_xg20_op_dn9))) / (2.0 * assign44480_e49617)))), );
        locals.var_temp_rv = 0.0;
        let assign44510_e49636: f64 = (2.0 * locals.var_xd0_op);
        let assign44510_e49640: f64 = (locals.var_xdsx_op / locals.var_xd0_op);
        let assign44510_e49641: f64 = (1.0 + assign44510_e49640);
        let assign44510_e49642: f64 = (assign44510_e49641).sqrt();
        let assign44510_e49644: f64 = (assign44510_e49642 - 1.0);
        let assign44510_e49645: f64 = (assign44510_e49636 * assign44510_e49644);
        let assign44510_e49649: f64 = (locals.var_cfdl_i * locals.var_dleff_op);
        let assign44510_e49650: f64 = (1.0 + assign44510_e49649);
        let assign44510_e49651: f64 = (assign44510_e49645 * assign44510_e49650);
        let assign44510_e49655: f64 = (locals.var_cfdlb_i * locals.var_xg20_op);
        let assign44510_e49656: f64 = (1.0 + assign44510_e49655);
        let assign44510_e49657: f64 = (assign44510_e49651 * assign44510_e49656);
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign44510_e49657, (((((((2.0 * locals.var_xd0_op_dn4) * assign44510_e49644) + (assign44510_e49636 * ((((locals.var_xdsx_op_dn4 * locals.var_xd0_op) - (locals.var_xdsx_op * locals.var_xd0_op_dn4)) / (locals.var_xd0_op * locals.var_xd0_op)) / (2.0 * assign44510_e49642)))) * assign44510_e49650) + (assign44510_e49645 * (locals.var_cfdl_i * locals.var_dleff_op_dn4))) * assign44510_e49656) + (assign44510_e49651 * (locals.var_cfdlb_i * locals.var_xg20_op_dn4))), (((((((2.0 * locals.var_xd0_op_dn6) * assign44510_e49644) + (assign44510_e49636 * ((((locals.var_xdsx_op_dn6 * locals.var_xd0_op) - (locals.var_xdsx_op * locals.var_xd0_op_dn6)) / (locals.var_xd0_op * locals.var_xd0_op)) / (2.0 * assign44510_e49642)))) * assign44510_e49650) + (assign44510_e49645 * (locals.var_cfdl_i * locals.var_dleff_op_dn6))) * assign44510_e49656) + (assign44510_e49651 * (locals.var_cfdlb_i * locals.var_xg20_op_dn6))), (((((((2.0 * locals.var_xd0_op_dn7) * assign44510_e49644) + (assign44510_e49636 * ((((locals.var_xdsx_op_dn7 * locals.var_xd0_op) - (locals.var_xdsx_op * locals.var_xd0_op_dn7)) / (locals.var_xd0_op * locals.var_xd0_op)) / (2.0 * assign44510_e49642)))) * assign44510_e49650) + (assign44510_e49645 * (locals.var_cfdl_i * locals.var_dleff_op_dn7))) * assign44510_e49656) + (assign44510_e49651 * (locals.var_cfdlb_i * locals.var_xg20_op_dn7))), (((((((2.0 * locals.var_xd0_op_dn8) * assign44510_e49644) + (assign44510_e49636 * ((((locals.var_xdsx_op_dn8 * locals.var_xd0_op) - (locals.var_xdsx_op * locals.var_xd0_op_dn8)) / (locals.var_xd0_op * locals.var_xd0_op)) / (2.0 * assign44510_e49642)))) * assign44510_e49650) + (assign44510_e49645 * (locals.var_cfdl_i * locals.var_dleff_op_dn8))) * assign44510_e49656) + (assign44510_e49651 * (locals.var_cfdlb_i * locals.var_xg20_op_dn8))), (((((((2.0 * locals.var_xd0_op_dn9) * assign44510_e49644) + (assign44510_e49636 * ((((locals.var_xdsx_op_dn9 * locals.var_xd0_op) - (locals.var_xdsx_op * locals.var_xd0_op_dn9)) / (locals.var_xd0_op * locals.var_xd0_op)) / (2.0 * assign44510_e49642)))) * assign44510_e49650) + (assign44510_e49645 * (locals.var_cfdl_i * locals.var_dleff_op_dn9))) * assign44510_e49656) + (assign44510_e49651 * (locals.var_cfdlb_i * locals.var_xg20_op_dn9))), );
        locals.var_temp_rv = 0.0;
        let assign44750_e49840: f64 = if p.p11 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1360 = assign44750_e49840;
        locals.var_guard1360_rv = 0.0;
        if (locals.var_guard1360 != 0.0) {
            let assign44760_e49844: f64 = (locals.var_k2_ac / locals.var_k1_ac);
            let assign44760_e49847: f64 = (1.0 + locals.var_k2_ac);
            let assign44760_e49848: f64 = (assign44760_e49844 / assign44760_e49847);
            (locals.var_r1init_op, locals.var_r1init_op_dn4, locals.var_r1init_op_dn6, locals.var_r1init_op_dn7, locals.var_r1init_op_dn8, locals.var_r1init_op_dn9, ) = (assign44760_e49848, ((((((locals.var_k2_ac_dn4 * locals.var_k1_ac) - (locals.var_k2_ac * locals.var_k1_ac_dn4)) / (locals.var_k1_ac * locals.var_k1_ac)) * assign44760_e49847) - (assign44760_e49844 * locals.var_k2_ac_dn4)) / (assign44760_e49847 * assign44760_e49847)), ((((((locals.var_k2_ac_dn6 * locals.var_k1_ac) - (locals.var_k2_ac * locals.var_k1_ac_dn6)) / (locals.var_k1_ac * locals.var_k1_ac)) * assign44760_e49847) - (assign44760_e49844 * locals.var_k2_ac_dn6)) / (assign44760_e49847 * assign44760_e49847)), ((((((locals.var_k2_ac_dn7 * locals.var_k1_ac) - (locals.var_k2_ac * locals.var_k1_ac_dn7)) / (locals.var_k1_ac * locals.var_k1_ac)) * assign44760_e49847) - (assign44760_e49844 * locals.var_k2_ac_dn7)) / (assign44760_e49847 * assign44760_e49847)), ((((((locals.var_k2_ac_dn8 * locals.var_k1_ac) - (locals.var_k2_ac * locals.var_k1_ac_dn8)) / (locals.var_k1_ac * locals.var_k1_ac)) * assign44760_e49847) - (assign44760_e49844 * locals.var_k2_ac_dn8)) / (assign44760_e49847 * assign44760_e49847)), ((((((locals.var_k2_ac_dn9 * locals.var_k1_ac) - (locals.var_k2_ac * locals.var_k1_ac_dn9)) / (locals.var_k1_ac * locals.var_k1_ac)) * assign44760_e49847) - (assign44760_e49844 * locals.var_k2_ac_dn9)) / (assign44760_e49847 * assign44760_e49847)), );
            locals.var_r1init_op_rv = 0.0;
        }
        if (locals.var_guard1360 != 0.0) {
            let assign44770_e49854: f64 = (locals.var_k1_ac / locals.var_k2_ac);
            let assign44770_e49857: f64 = (1.0 + locals.var_k1_ac);
            let assign44770_e49858: f64 = (assign44770_e49854 / assign44770_e49857);
            (locals.var_r2init_op, locals.var_r2init_op_dn4, locals.var_r2init_op_dn6, locals.var_r2init_op_dn7, locals.var_r2init_op_dn8, locals.var_r2init_op_dn9, ) = (assign44770_e49858, ((((((locals.var_k1_ac_dn4 * locals.var_k2_ac) - (locals.var_k1_ac * locals.var_k2_ac_dn4)) / (locals.var_k2_ac * locals.var_k2_ac)) * assign44770_e49857) - (assign44770_e49854 * locals.var_k1_ac_dn4)) / (assign44770_e49857 * assign44770_e49857)), ((((((locals.var_k1_ac_dn6 * locals.var_k2_ac) - (locals.var_k1_ac * locals.var_k2_ac_dn6)) / (locals.var_k2_ac * locals.var_k2_ac)) * assign44770_e49857) - (assign44770_e49854 * locals.var_k1_ac_dn6)) / (assign44770_e49857 * assign44770_e49857)), ((((((locals.var_k1_ac_dn7 * locals.var_k2_ac) - (locals.var_k1_ac * locals.var_k2_ac_dn7)) / (locals.var_k2_ac * locals.var_k2_ac)) * assign44770_e49857) - (assign44770_e49854 * locals.var_k1_ac_dn7)) / (assign44770_e49857 * assign44770_e49857)), ((((((locals.var_k1_ac_dn8 * locals.var_k2_ac) - (locals.var_k1_ac * locals.var_k2_ac_dn8)) / (locals.var_k2_ac * locals.var_k2_ac)) * assign44770_e49857) - (assign44770_e49854 * locals.var_k1_ac_dn8)) / (assign44770_e49857 * assign44770_e49857)), ((((((locals.var_k1_ac_dn9 * locals.var_k2_ac) - (locals.var_k1_ac * locals.var_k2_ac_dn9)) / (locals.var_k2_ac * locals.var_k2_ac)) * assign44770_e49857) - (assign44770_e49854 * locals.var_k1_ac_dn9)) / (assign44770_e49857 * assign44770_e49857)), );
            locals.var_r2init_op_rv = 0.0;
        }
        if (locals.var_guard1360 != 0.0) {
            let assign44780_e49865: f64 = (1.0 + locals.var_r1init_op);
            let assign44780_e49866: f64 = (locals.var_k1_ac * assign44780_e49865);
            let assign44780_e49868: f64 = (assign44780_e49866 * locals.var_diff_min_ac);
            let assign44780_e49870: f64 = (assign44780_e49868 / locals.var_a0_ac);
            let assign44780_e49871: f64 = (assign44780_e49870).ln();
            let assign44780_e49873: f64 = (assign44780_e49871 + 2.0);
            (locals.var_x1init_op, locals.var_x1init_op_dn4, locals.var_x1init_op_dn6, locals.var_x1init_op_dn7, locals.var_x1init_op_dn8, locals.var_x1init_op_dn9, ) = (assign44780_e49873, ((((((((locals.var_k1_ac_dn4 * assign44780_e49865) + (locals.var_k1_ac * locals.var_r1init_op_dn4)) * locals.var_diff_min_ac) + (assign44780_e49866 * locals.var_diff_min_ac_dn4)) * locals.var_a0_ac) - (assign44780_e49868 * locals.var_a0_ac_dn4)) / (locals.var_a0_ac * locals.var_a0_ac)) / assign44780_e49870), ((((((((locals.var_k1_ac_dn6 * assign44780_e49865) + (locals.var_k1_ac * locals.var_r1init_op_dn6)) * locals.var_diff_min_ac) + (assign44780_e49866 * locals.var_diff_min_ac_dn6)) * locals.var_a0_ac) - (assign44780_e49868 * locals.var_a0_ac_dn6)) / (locals.var_a0_ac * locals.var_a0_ac)) / assign44780_e49870), ((((((((locals.var_k1_ac_dn7 * assign44780_e49865) + (locals.var_k1_ac * locals.var_r1init_op_dn7)) * locals.var_diff_min_ac) + (assign44780_e49866 * locals.var_diff_min_ac_dn7)) * locals.var_a0_ac) - (assign44780_e49868 * locals.var_a0_ac_dn7)) / (locals.var_a0_ac * locals.var_a0_ac)) / assign44780_e49870), ((((((((locals.var_k1_ac_dn8 * assign44780_e49865) + (locals.var_k1_ac * locals.var_r1init_op_dn8)) * locals.var_diff_min_ac) + (assign44780_e49866 * locals.var_diff_min_ac_dn8)) * locals.var_a0_ac) - (assign44780_e49868 * locals.var_a0_ac_dn8)) / (locals.var_a0_ac * locals.var_a0_ac)) / assign44780_e49870), ((((((((locals.var_k1_ac_dn9 * assign44780_e49865) + (locals.var_k1_ac * locals.var_r1init_op_dn9)) * locals.var_diff_min_ac) + (assign44780_e49866 * locals.var_diff_min_ac_dn9)) * locals.var_a0_ac) - (assign44780_e49868 * locals.var_a0_ac_dn9)) / (locals.var_a0_ac * locals.var_a0_ac)) / assign44780_e49870), );
            locals.var_x1init_op_rv = 0.0;
        }
        if (locals.var_guard1360 != 0.0) {
            let assign44790_e49880: f64 = (1.0 + locals.var_r2init_op);
            let assign44790_e49881: f64 = (locals.var_k2_ac * assign44790_e49880);
            let assign44790_e49883: f64 = (assign44790_e49881 * locals.var_diff_min_ac);
            let assign44790_e49885: f64 = (assign44790_e49883 / locals.var_a0_ac);
            let assign44790_e49886: f64 = (assign44790_e49885).ln();
            let assign44790_e49888: f64 = (assign44790_e49886 + 2.0);
            (locals.var_x2init_op, locals.var_x2init_op_dn4, locals.var_x2init_op_dn6, locals.var_x2init_op_dn7, locals.var_x2init_op_dn8, locals.var_x2init_op_dn9, ) = (assign44790_e49888, ((((((((locals.var_k2_ac_dn4 * assign44790_e49880) + (locals.var_k2_ac * locals.var_r2init_op_dn4)) * locals.var_diff_min_ac) + (assign44790_e49881 * locals.var_diff_min_ac_dn4)) * locals.var_a0_ac) - (assign44790_e49883 * locals.var_a0_ac_dn4)) / (locals.var_a0_ac * locals.var_a0_ac)) / assign44790_e49885), ((((((((locals.var_k2_ac_dn6 * assign44790_e49880) + (locals.var_k2_ac * locals.var_r2init_op_dn6)) * locals.var_diff_min_ac) + (assign44790_e49881 * locals.var_diff_min_ac_dn6)) * locals.var_a0_ac) - (assign44790_e49883 * locals.var_a0_ac_dn6)) / (locals.var_a0_ac * locals.var_a0_ac)) / assign44790_e49885), ((((((((locals.var_k2_ac_dn7 * assign44790_e49880) + (locals.var_k2_ac * locals.var_r2init_op_dn7)) * locals.var_diff_min_ac) + (assign44790_e49881 * locals.var_diff_min_ac_dn7)) * locals.var_a0_ac) - (assign44790_e49883 * locals.var_a0_ac_dn7)) / (locals.var_a0_ac * locals.var_a0_ac)) / assign44790_e49885), ((((((((locals.var_k2_ac_dn8 * assign44790_e49880) + (locals.var_k2_ac * locals.var_r2init_op_dn8)) * locals.var_diff_min_ac) + (assign44790_e49881 * locals.var_diff_min_ac_dn8)) * locals.var_a0_ac) - (assign44790_e49883 * locals.var_a0_ac_dn8)) / (locals.var_a0_ac * locals.var_a0_ac)) / assign44790_e49885), ((((((((locals.var_k2_ac_dn9 * assign44790_e49880) + (locals.var_k2_ac * locals.var_r2init_op_dn9)) * locals.var_diff_min_ac) + (assign44790_e49881 * locals.var_diff_min_ac_dn9)) * locals.var_a0_ac) - (assign44790_e49883 * locals.var_a0_ac_dn9)) / (locals.var_a0_ac * locals.var_a0_ac)) / assign44790_e49885), );
            locals.var_x2init_op_rv = 0.0;
        }
        if (locals.var_guard1360 != 0.0) {
            let assign44800_e49894: f64 = (1.0 + locals.var_r1init_op);
            let assign44800_e49896: f64 = (assign44800_e49894 * locals.var_x1init_op);
            let assign44800_e49899: f64 = (locals.var_xg2x_ac * locals.var_r1init_op);
            let assign44800_e49900: f64 = (assign44800_e49896 - assign44800_e49899);
            (locals.var_xth1init_op, locals.var_xth1init_op_dn4, locals.var_xth1init_op_dn6, locals.var_xth1init_op_dn7, locals.var_xth1init_op_dn8, locals.var_xth1init_op_dn9, ) = (assign44800_e49900, (((locals.var_r1init_op_dn4 * locals.var_x1init_op) + (assign44800_e49894 * locals.var_x1init_op_dn4)) - ((locals.var_xg2x_ac_dn4 * locals.var_r1init_op) + (locals.var_xg2x_ac * locals.var_r1init_op_dn4))), (((locals.var_r1init_op_dn6 * locals.var_x1init_op) + (assign44800_e49894 * locals.var_x1init_op_dn6)) - ((locals.var_xg2x_ac_dn6 * locals.var_r1init_op) + (locals.var_xg2x_ac * locals.var_r1init_op_dn6))), (((locals.var_r1init_op_dn7 * locals.var_x1init_op) + (assign44800_e49894 * locals.var_x1init_op_dn7)) - ((locals.var_xg2x_ac_dn7 * locals.var_r1init_op) + (locals.var_xg2x_ac * locals.var_r1init_op_dn7))), (((locals.var_r1init_op_dn8 * locals.var_x1init_op) + (assign44800_e49894 * locals.var_x1init_op_dn8)) - ((locals.var_xg2x_ac_dn8 * locals.var_r1init_op) + (locals.var_xg2x_ac * locals.var_r1init_op_dn8))), (((locals.var_r1init_op_dn9 * locals.var_x1init_op) + (assign44800_e49894 * locals.var_x1init_op_dn9)) - ((locals.var_xg2x_ac_dn9 * locals.var_r1init_op) + (locals.var_xg2x_ac * locals.var_r1init_op_dn9))), );
            locals.var_xth1init_op_rv = 0.0;
        }
        if (locals.var_guard1360 != 0.0) {
            let assign44810_e49907: f64 = (1.0 / locals.var_r2init_op);
            let assign44810_e49908: f64 = (1.0 + assign44810_e49907);
            let assign44810_e49910: f64 = (assign44810_e49908 * locals.var_x2init_op);
            let assign44810_e49913: f64 = (locals.var_xg2x_ac / locals.var_r2init_op);
            let assign44810_e49914: f64 = (assign44810_e49910 - assign44810_e49913);
            (locals.var_xth2init_op, locals.var_xth2init_op_dn4, locals.var_xth2init_op_dn6, locals.var_xth2init_op_dn7, locals.var_xth2init_op_dn8, locals.var_xth2init_op_dn9, ) = (assign44810_e49914, ((((-(locals.var_r2init_op_dn4 / (locals.var_r2init_op * locals.var_r2init_op))) * locals.var_x2init_op) + (assign44810_e49908 * locals.var_x2init_op_dn4)) - (((locals.var_xg2x_ac_dn4 * locals.var_r2init_op) - (locals.var_xg2x_ac * locals.var_r2init_op_dn4)) / (locals.var_r2init_op * locals.var_r2init_op))), ((((-(locals.var_r2init_op_dn6 / (locals.var_r2init_op * locals.var_r2init_op))) * locals.var_x2init_op) + (assign44810_e49908 * locals.var_x2init_op_dn6)) - (((locals.var_xg2x_ac_dn6 * locals.var_r2init_op) - (locals.var_xg2x_ac * locals.var_r2init_op_dn6)) / (locals.var_r2init_op * locals.var_r2init_op))), ((((-(locals.var_r2init_op_dn7 / (locals.var_r2init_op * locals.var_r2init_op))) * locals.var_x2init_op) + (assign44810_e49908 * locals.var_x2init_op_dn7)) - (((locals.var_xg2x_ac_dn7 * locals.var_r2init_op) - (locals.var_xg2x_ac * locals.var_r2init_op_dn7)) / (locals.var_r2init_op * locals.var_r2init_op))), ((((-(locals.var_r2init_op_dn8 / (locals.var_r2init_op * locals.var_r2init_op))) * locals.var_x2init_op) + (assign44810_e49908 * locals.var_x2init_op_dn8)) - (((locals.var_xg2x_ac_dn8 * locals.var_r2init_op) - (locals.var_xg2x_ac * locals.var_r2init_op_dn8)) / (locals.var_r2init_op * locals.var_r2init_op))), ((((-(locals.var_r2init_op_dn9 / (locals.var_r2init_op * locals.var_r2init_op))) * locals.var_x2init_op) + (assign44810_e49908 * locals.var_x2init_op_dn9)) - (((locals.var_xg2x_ac_dn9 * locals.var_r2init_op) - (locals.var_xg2x_ac * locals.var_r2init_op_dn9)) / (locals.var_r2init_op * locals.var_r2init_op))), );
            locals.var_xth2init_op_rv = 0.0;
        }
        if (locals.var_guard1360 != 0.0) {
            let assign44820_e49921: f64 = (locals.var_xth1init_op + locals.var_xth2init_op);
            let assign44820_e49924: f64 = (locals.var_xth1init_op - locals.var_xth2init_op);
            let assign44820_e49927: f64 = (locals.var_xth1init_op - locals.var_xth2init_op);
            let assign44820_e49928: f64 = (assign44820_e49924 * assign44820_e49927);
            let assign44820_e49930: f64 = (assign44820_e49928 + 38.0);
            let assign44820_e49931: f64 = (assign44820_e49930).sqrt();
            let assign44820_e49932: f64 = (assign44820_e49921 - assign44820_e49931);
            let assign44820_e49933: f64 = (0.5 * assign44820_e49932);
            let assign44820_e49935: f64 = (assign44820_e49933 - locals.var_xg2_ac);
            let assign44820_e49937: f64 = (assign44820_e49935 / locals.var_cic1_i);
            let assign44820_e49939: f64 = (assign44820_e49937 + locals.var_xg2_ac);
            (locals.var_xg1thinit_op, locals.var_xg1thinit_op_dn4, locals.var_xg1thinit_op_dn6, locals.var_xg1thinit_op_dn7, locals.var_xg1thinit_op_dn8, locals.var_xg1thinit_op_dn9, ) = (assign44820_e49939, ((((0.5 * ((locals.var_xth1init_op_dn4 + locals.var_xth2init_op_dn4) - ((((locals.var_xth1init_op_dn4 - locals.var_xth2init_op_dn4) * assign44820_e49927) + (assign44820_e49924 * (locals.var_xth1init_op_dn4 - locals.var_xth2init_op_dn4))) / (2.0 * assign44820_e49931)))) - locals.var_xg2_ac_dn4) / locals.var_cic1_i) + locals.var_xg2_ac_dn4), ((((0.5 * ((locals.var_xth1init_op_dn6 + locals.var_xth2init_op_dn6) - ((((locals.var_xth1init_op_dn6 - locals.var_xth2init_op_dn6) * assign44820_e49927) + (assign44820_e49924 * (locals.var_xth1init_op_dn6 - locals.var_xth2init_op_dn6))) / (2.0 * assign44820_e49931)))) - locals.var_xg2_ac_dn6) / locals.var_cic1_i) + locals.var_xg2_ac_dn6), ((((0.5 * ((locals.var_xth1init_op_dn7 + locals.var_xth2init_op_dn7) - ((((locals.var_xth1init_op_dn7 - locals.var_xth2init_op_dn7) * assign44820_e49927) + (assign44820_e49924 * (locals.var_xth1init_op_dn7 - locals.var_xth2init_op_dn7))) / (2.0 * assign44820_e49931)))) - locals.var_xg2_ac_dn7) / locals.var_cic1_i) + locals.var_xg2_ac_dn7), ((((0.5 * ((locals.var_xth1init_op_dn8 + locals.var_xth2init_op_dn8) - ((((locals.var_xth1init_op_dn8 - locals.var_xth2init_op_dn8) * assign44820_e49927) + (assign44820_e49924 * (locals.var_xth1init_op_dn8 - locals.var_xth2init_op_dn8))) / (2.0 * assign44820_e49931)))) - locals.var_xg2_ac_dn8) / locals.var_cic1_i) + locals.var_xg2_ac_dn8), ((((0.5 * ((locals.var_xth1init_op_dn9 + locals.var_xth2init_op_dn9) - ((((locals.var_xth1init_op_dn9 - locals.var_xth2init_op_dn9) * assign44820_e49927) + (assign44820_e49924 * (locals.var_xth1init_op_dn9 - locals.var_xth2init_op_dn9))) / (2.0 * assign44820_e49931)))) - locals.var_xg2_ac_dn9) / locals.var_cic1_i) + locals.var_xg2_ac_dn9), );
            locals.var_xg1thinit_op_rv = 0.0;
        }
        if (locals.var_guard1360 != 0.0) {
            let assign44830_e49946: f64 = (locals.var_xg1thinit_op - locals.var_xedge_ac);
            let assign44830_e49948: f64 = (assign44830_e49946 / locals.var_sce1_ac);
            let assign44830_e49950: f64 = (assign44830_e49948 - locals.var_dxg1_dibl_ac);
            let assign44830_e49952: f64 = (assign44830_e49950 + locals.var_xedge_ac);
            let assign44830_e49953: f64 = (locals.var_phit * assign44830_e49952);
            let assign44830_e49955: f64 = (assign44830_e49953 + locals.var_vfbac1_i);
            (locals.var_vthinit_op, locals.var_vthinit_op_dn4, locals.var_vthinit_op_dn6, locals.var_vthinit_op_dn7, locals.var_vthinit_op_dn8, locals.var_vthinit_op_dn9, ) = (assign44830_e49955, (((locals.var_phit_dn4 * assign44830_e49952) + (locals.var_phit * ((((((locals.var_xg1thinit_op_dn4 - locals.var_xedge_ac_dn4) * locals.var_sce1_ac) - (assign44830_e49946 * locals.var_sce1_ac_dn4)) / (locals.var_sce1_ac * locals.var_sce1_ac)) - locals.var_dxg1_dibl_ac_dn4) + locals.var_xedge_ac_dn4))) + locals.var_vfbac1_i_dn4), (((locals.var_phit_dn6 * assign44830_e49952) + (locals.var_phit * ((((((locals.var_xg1thinit_op_dn6 - locals.var_xedge_ac_dn6) * locals.var_sce1_ac) - (assign44830_e49946 * locals.var_sce1_ac_dn6)) / (locals.var_sce1_ac * locals.var_sce1_ac)) - locals.var_dxg1_dibl_ac_dn6) + locals.var_xedge_ac_dn6))) + locals.var_vfbac1_i_dn6), (((locals.var_phit_dn7 * assign44830_e49952) + (locals.var_phit * ((((((locals.var_xg1thinit_op_dn7 - locals.var_xedge_ac_dn7) * locals.var_sce1_ac) - (assign44830_e49946 * locals.var_sce1_ac_dn7)) / (locals.var_sce1_ac * locals.var_sce1_ac)) - locals.var_dxg1_dibl_ac_dn7) + locals.var_xedge_ac_dn7))) + locals.var_vfbac1_i_dn7), (((locals.var_phit_dn8 * assign44830_e49952) + (locals.var_phit * ((((((locals.var_xg1thinit_op_dn8 - locals.var_xedge_ac_dn8) * locals.var_sce1_ac) - (assign44830_e49946 * locals.var_sce1_ac_dn8)) / (locals.var_sce1_ac * locals.var_sce1_ac)) - locals.var_dxg1_dibl_ac_dn8) + locals.var_xedge_ac_dn8))) + locals.var_vfbac1_i_dn8), (((locals.var_phit_dn9 * assign44830_e49952) + (locals.var_phit * ((((((locals.var_xg1thinit_op_dn9 - locals.var_xedge_ac_dn9) * locals.var_sce1_ac) - (assign44830_e49946 * locals.var_sce1_ac_dn9)) / (locals.var_sce1_ac * locals.var_sce1_ac)) - locals.var_dxg1_dibl_ac_dn9) + locals.var_xedge_ac_dn9))) + locals.var_vfbac1_i_dn9), );
            locals.var_vthinit_op_rv = 0.0;
        }
        if (locals.var_guard1360 != 0.0) {
            let assign44840_e49962: f64 = (locals.var_tkd - locals.var_tkr);
            let assign44840_e49963: f64 = (locals.var_stcf_i * assign44840_e49962);
            (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign44840_e49963, ((locals.var_stcf_i_dn4 * assign44840_e49962) + (locals.var_stcf_i * locals.var_tkd_dn4)), ((locals.var_stcf_i_dn6 * assign44840_e49962) + (locals.var_stcf_i * locals.var_tkd_dn6)), ((locals.var_stcf_i_dn7 * assign44840_e49962) + (locals.var_stcf_i * locals.var_tkd_dn7)), ((locals.var_stcf_i_dn8 * assign44840_e49962) + (locals.var_stcf_i * locals.var_tkd_dn8)), ((locals.var_stcf_i_dn9 * assign44840_e49962) + (locals.var_stcf_i * locals.var_tkd_dn9)), );
            locals.var_temp_rv = 0.0;
        }
        if (locals.var_guard1360 != 0.0) {
            let assign44870_e49981: f64 = (p.p14 * locals.var_stvfb_i);
            let assign44870_e49984: f64 = (locals.var_tkd - locals.var_tkr);
            let assign44870_e49985: f64 = (assign44870_e49981 * assign44870_e49984);
            let assign44870_e49987: f64 = (assign44870_e49985 + locals.var_dvfbqm);
            (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign44870_e49987, (assign44870_e49981 * locals.var_tkd_dn4), (assign44870_e49981 * locals.var_tkd_dn6), (assign44870_e49981 * locals.var_tkd_dn7), (assign44870_e49981 * locals.var_tkd_dn8), (assign44870_e49981 * locals.var_tkd_dn9), );
            locals.var_temp_rv = 0.0;
        }
        if (locals.var_guard1360 != 0.0) {
            let assign44880_e49994: f64 = (locals.var_vfbac1_t + locals.var_dvfbch_op);
            let assign44880_e49996: f64 = (assign44880_e49994 + locals.var_dvfb1nch);
            let assign44880_e49997: f64 = (p.p14 * assign44880_e49996);
            let assign44880_e49999: f64 = (assign44880_e49997 + locals.var_temp);
            let assign44880_e50001: f64 = (assign44880_e49999 + p.p34);
            let assign44880_e50003: f64 = (assign44880_e50001 - locals.var_dvfbpdep_op);
            (locals.var_vfb1_op, locals.var_vfb1_op_dn4, locals.var_vfb1_op_dn6, locals.var_vfb1_op_dn7, locals.var_vfb1_op_dn8, locals.var_vfb1_op_dn9, ) = (assign44880_e50003, (((p.p14 * ((locals.var_vfbac1_t_dn4 + locals.var_dvfbch_op_dn4) + locals.var_dvfb1nch_dn4)) + locals.var_temp_dn4) - locals.var_dvfbpdep_op_dn4), (((p.p14 * ((locals.var_vfbac1_t_dn6 + locals.var_dvfbch_op_dn6) + locals.var_dvfb1nch_dn6)) + locals.var_temp_dn6) - locals.var_dvfbpdep_op_dn6), (((p.p14 * ((locals.var_vfbac1_t_dn7 + locals.var_dvfbch_op_dn7) + locals.var_dvfb1nch_dn7)) + locals.var_temp_dn7) - locals.var_dvfbpdep_op_dn7), (((p.p14 * ((locals.var_vfbac1_t_dn8 + locals.var_dvfbch_op_dn8) + locals.var_dvfb1nch_dn8)) + locals.var_temp_dn8) - locals.var_dvfbpdep_op_dn8), (((p.p14 * ((locals.var_vfbac1_t_dn9 + locals.var_dvfbch_op_dn9) + locals.var_dvfb1nch_dn9)) + locals.var_temp_dn9) - locals.var_dvfbpdep_op_dn9), );
            locals.var_vfb1_op_rv = 0.0;
        }
        if (locals.var_guard1360 != 0.0) {
            let assign44890_e50010: f64 = (locals.var_vfbac2_t + locals.var_dvfbch_op);
            let assign44890_e50012: f64 = (assign44890_e50010 + locals.var_dvfb2nch);
            let assign44890_e50013: f64 = (p.p14 * assign44890_e50012);
            let assign44890_e50015: f64 = (assign44890_e50013 + locals.var_temp);
            (locals.var_vfb2_op, locals.var_vfb2_op_dn4, locals.var_vfb2_op_dn6, locals.var_vfb2_op_dn7, locals.var_vfb2_op_dn8, locals.var_vfb2_op_dn9, ) = (assign44890_e50015, ((p.p14 * ((locals.var_vfbac2_t_dn4 + locals.var_dvfbch_op_dn4) + locals.var_dvfb2nch_dn4)) + locals.var_temp_dn4), ((p.p14 * ((locals.var_vfbac2_t_dn6 + locals.var_dvfbch_op_dn6) + locals.var_dvfb2nch_dn6)) + locals.var_temp_dn6), ((p.p14 * ((locals.var_vfbac2_t_dn7 + locals.var_dvfbch_op_dn7) + locals.var_dvfb2nch_dn7)) + locals.var_temp_dn7), ((p.p14 * ((locals.var_vfbac2_t_dn8 + locals.var_dvfbch_op_dn8) + locals.var_dvfb2nch_dn8)) + locals.var_temp_dn8), ((p.p14 * ((locals.var_vfbac2_t_dn9 + locals.var_dvfbch_op_dn9) + locals.var_dvfb2nch_dn9)) + locals.var_temp_dn9), );
            locals.var_vfb2_op_rv = 0.0;
        }
        if (locals.var_guard1360 != 0.0) {
            let assign44900_e50021: f64 = (locals.var_vthinit_op - locals.var_vfb1_op);
            let assign44900_e50023: f64 = (assign44900_e50021 * locals.var_inv_phit_op);
            let assign44900_e50025: f64 = (assign44900_e50023 - locals.var_dxdsx_op);
            (locals.var_xg10_op, locals.var_xg10_op_dn4, locals.var_xg10_op_dn6, locals.var_xg10_op_dn7, locals.var_xg10_op_dn8, locals.var_xg10_op_dn9, ) = (assign44900_e50025, ((((locals.var_vthinit_op_dn4 - locals.var_vfb1_op_dn4) * locals.var_inv_phit_op) + (assign44900_e50021 * locals.var_inv_phit_op_dn4)) - locals.var_dxdsx_op_dn4), ((((locals.var_vthinit_op_dn6 - locals.var_vfb1_op_dn6) * locals.var_inv_phit_op) + (assign44900_e50021 * locals.var_inv_phit_op_dn6)) - locals.var_dxdsx_op_dn6), ((((locals.var_vthinit_op_dn7 - locals.var_vfb1_op_dn7) * locals.var_inv_phit_op) + (assign44900_e50021 * locals.var_inv_phit_op_dn7)) - locals.var_dxdsx_op_dn7), ((((locals.var_vthinit_op_dn8 - locals.var_vfb1_op_dn8) * locals.var_inv_phit_op) + (assign44900_e50021 * locals.var_inv_phit_op_dn8)) - locals.var_dxdsx_op_dn8), ((((locals.var_vthinit_op_dn9 - locals.var_vfb1_op_dn9) * locals.var_inv_phit_op) + (assign44900_e50021 * locals.var_inv_phit_op_dn9)) - locals.var_dxdsx_op_dn9), );
            locals.var_xg10_op_rv = 0.0;
        }
        if (locals.var_guard1360 != 0.0) {
            let assign44910_e50030: f64 = (-locals.var_vsb);
            let assign44910_e50032: f64 = (assign44910_e50030 - locals.var_vfb2_op);
            let assign44910_e50034: f64 = (assign44910_e50032 * locals.var_inv_phit_op);
            let assign44910_e50036: f64 = (assign44910_e50034 - locals.var_dxdsx_op);
            (locals.var_xg20_op, locals.var_xg20_op_dn4, locals.var_xg20_op_dn6, locals.var_xg20_op_dn7, locals.var_xg20_op_dn8, locals.var_xg20_op_dn9, ) = (assign44910_e50036, ((((-locals.var_vfb2_op_dn4) * locals.var_inv_phit_op) + (assign44910_e50032 * locals.var_inv_phit_op_dn4)) - locals.var_dxdsx_op_dn4), (((((-locals.var_vsb_dn6) - locals.var_vfb2_op_dn6) * locals.var_inv_phit_op) + (assign44910_e50032 * locals.var_inv_phit_op_dn6)) - locals.var_dxdsx_op_dn6), (((((-locals.var_vsb_dn7) - locals.var_vfb2_op_dn7) * locals.var_inv_phit_op) + (assign44910_e50032 * locals.var_inv_phit_op_dn7)) - locals.var_dxdsx_op_dn7), (((((-locals.var_vsb_dn8) - locals.var_vfb2_op_dn8) * locals.var_inv_phit_op) + (assign44910_e50032 * locals.var_inv_phit_op_dn8)) - locals.var_dxdsx_op_dn8), ((((-locals.var_vfb2_op_dn9) * locals.var_inv_phit_op) + (assign44910_e50032 * locals.var_inv_phit_op_dn9)) - locals.var_dxdsx_op_dn9), );
            locals.var_xg20_op_rv = 0.0;
        }
        let assign44920_e50041: f64 = if p.p2 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1361 = assign44920_e50041;
        locals.var_guard1361_rv = 0.0;
        if ((locals.var_guard1360 != 0.0) && (locals.var_guard1361 != 0.0)) {
            let assign44930_e50047: f64 = (p.p14 * locals.var_typesub_i);
            let assign44930_e50050: f64 = (locals.var_xg10_op - locals.var_xg20_op);
            let assign44930_e50051: f64 = (assign44930_e50047 * assign44930_e50050);
            let assign44930_e50053: f64 = (assign44930_e50051 / locals.var_gfsub);
            (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign44930_e50053, ((((assign44930_e50047 * (locals.var_xg10_op_dn4 - locals.var_xg20_op_dn4)) * locals.var_gfsub) - (assign44930_e50051 * locals.var_gfsub_dn4)) / (locals.var_gfsub * locals.var_gfsub)), ((((assign44930_e50047 * (locals.var_xg10_op_dn6 - locals.var_xg20_op_dn6)) * locals.var_gfsub) - (assign44930_e50051 * locals.var_gfsub_dn6)) / (locals.var_gfsub * locals.var_gfsub)), ((((assign44930_e50047 * (locals.var_xg10_op_dn7 - locals.var_xg20_op_dn7)) * locals.var_gfsub) - (assign44930_e50051 * locals.var_gfsub_dn7)) / (locals.var_gfsub * locals.var_gfsub)), ((((assign44930_e50047 * (locals.var_xg10_op_dn8 - locals.var_xg20_op_dn8)) * locals.var_gfsub) - (assign44930_e50051 * locals.var_gfsub_dn8)) / (locals.var_gfsub * locals.var_gfsub)), ((((assign44930_e50047 * (locals.var_xg10_op_dn9 - locals.var_xg20_op_dn9)) * locals.var_gfsub) - (assign44930_e50051 * locals.var_gfsub_dn9)) / (locals.var_gfsub * locals.var_gfsub)), );
            locals.var_temp_rv = 0.0;
        }
        let assign44940_e50058: f64 = if locals.var_temp < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1362 = assign44940_e50058;
        locals.var_guard1362_rv = 0.0;
        if (((locals.var_guard1360 != 0.0) && (locals.var_guard1361 != 0.0)) && (locals.var_guard1362 != 0.0)) {
            let assign44950_e50065: f64 = (-2.0);
            let assign44950_e50068: f64 = (1.0 - locals.var_temp);
            let assign44950_e50069: f64 = (assign44950_e50068).ln();
            let assign44950_e50070: f64 = (assign44950_e50065 * assign44950_e50069);
            (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9, ) = (assign44950_e50070, (assign44950_e50065 * ((-locals.var_temp_dn4) / assign44950_e50068)), (assign44950_e50065 * ((-locals.var_temp_dn6) / assign44950_e50068)), (assign44950_e50065 * ((-locals.var_temp_dn7) / assign44950_e50068)), (assign44950_e50065 * ((-locals.var_temp_dn8) / assign44950_e50068)), (assign44950_e50065 * ((-locals.var_temp_dn9) / assign44950_e50068)), );
            locals.var_temp1_rv = 0.0;
        }
        if (((locals.var_guard1360 != 0.0) && (locals.var_guard1361 != 0.0)) && (locals.var_guard1362 == 0.0)) {
            let assign44960_e50081: f64 = (locals.var_temp * locals.var_temp);
            let assign44960_e50085: f64 = (2.0 * locals.var_temp);
            let assign44960_e50087: f64 = (assign44960_e50085 / locals.var_gfsub);
            let assign44960_e50088: f64 = (1.0 + assign44960_e50087);
            let assign44960_e50089: f64 = (assign44960_e50081 / assign44960_e50088);
            (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9, ) = (assign44960_e50089, (((((locals.var_temp_dn4 * locals.var_temp) + (locals.var_temp * locals.var_temp_dn4)) * assign44960_e50088) - (assign44960_e50081 * ((((2.0 * locals.var_temp_dn4) * locals.var_gfsub) - (assign44960_e50085 * locals.var_gfsub_dn4)) / (locals.var_gfsub * locals.var_gfsub)))) / (assign44960_e50088 * assign44960_e50088)), (((((locals.var_temp_dn6 * locals.var_temp) + (locals.var_temp * locals.var_temp_dn6)) * assign44960_e50088) - (assign44960_e50081 * ((((2.0 * locals.var_temp_dn6) * locals.var_gfsub) - (assign44960_e50085 * locals.var_gfsub_dn6)) / (locals.var_gfsub * locals.var_gfsub)))) / (assign44960_e50088 * assign44960_e50088)), (((((locals.var_temp_dn7 * locals.var_temp) + (locals.var_temp * locals.var_temp_dn7)) * assign44960_e50088) - (assign44960_e50081 * ((((2.0 * locals.var_temp_dn7) * locals.var_gfsub) - (assign44960_e50085 * locals.var_gfsub_dn7)) / (locals.var_gfsub * locals.var_gfsub)))) / (assign44960_e50088 * assign44960_e50088)), (((((locals.var_temp_dn8 * locals.var_temp) + (locals.var_temp * locals.var_temp_dn8)) * assign44960_e50088) - (assign44960_e50081 * ((((2.0 * locals.var_temp_dn8) * locals.var_gfsub) - (assign44960_e50085 * locals.var_gfsub_dn8)) / (locals.var_gfsub * locals.var_gfsub)))) / (assign44960_e50088 * assign44960_e50088)), (((((locals.var_temp_dn9 * locals.var_temp) + (locals.var_temp * locals.var_temp_dn9)) * assign44960_e50088) - (assign44960_e50081 * ((((2.0 * locals.var_temp_dn9) * locals.var_gfsub) - (assign44960_e50085 * locals.var_gfsub_dn9)) / (locals.var_gfsub * locals.var_gfsub)))) / (assign44960_e50088 * assign44960_e50088)), );
            locals.var_temp1_rv = 0.0;
        }
        if ((locals.var_guard1360 != 0.0) && (locals.var_guard1361 != 0.0)) {
            let assign44970_e50098: f64 = (p.p14 * locals.var_typesub_i);
            let assign44970_e50100: f64 = (assign44970_e50098 * locals.var_temp1);
            let assign44970_e50101: f64 = (locals.var_xg20_op + assign44970_e50100);
            (locals.var_xg2eff_op, locals.var_xg2eff_op_dn4, locals.var_xg2eff_op_dn6, locals.var_xg2eff_op_dn7, locals.var_xg2eff_op_dn8, locals.var_xg2eff_op_dn9, ) = (assign44970_e50101, (locals.var_xg20_op_dn4 + (assign44970_e50098 * locals.var_temp1_dn4)), (locals.var_xg20_op_dn6 + (assign44970_e50098 * locals.var_temp1_dn6)), (locals.var_xg20_op_dn7 + (assign44970_e50098 * locals.var_temp1_dn7)), (locals.var_xg20_op_dn8 + (assign44970_e50098 * locals.var_temp1_dn8)), (locals.var_xg20_op_dn9 + (assign44970_e50098 * locals.var_temp1_dn9)), );
            locals.var_xg2eff_op_rv = 0.0;
        }
        if ((locals.var_guard1360 != 0.0) && (locals.var_guard1361 == 0.0)) {
            (locals.var_xg2eff_op, locals.var_xg2eff_op_dn4, locals.var_xg2eff_op_dn6, locals.var_xg2eff_op_dn7, locals.var_xg2eff_op_dn8, locals.var_xg2eff_op_dn9, ) = (locals.var_xg20_op, locals.var_xg20_op_dn4, locals.var_xg20_op_dn6, locals.var_xg20_op_dn7, locals.var_xg20_op_dn8, locals.var_xg20_op_dn9, );
            locals.var_xg2eff_op_rv = 0.0;
        }
        if (locals.var_guard1360 != 0.0) {
            let assign44990_e50115: f64 = (locals.var_xg10_op - locals.var_xg2eff_op);
            let assign44990_e50116: f64 = (locals.var_keq_1d * assign44990_e50115);
            (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign44990_e50116, (locals.var_keq_1d * (locals.var_xg10_op_dn4 - locals.var_xg2eff_op_dn4)), (locals.var_keq_1d * (locals.var_xg10_op_dn6 - locals.var_xg2eff_op_dn6)), (locals.var_keq_1d * (locals.var_xg10_op_dn7 - locals.var_xg2eff_op_dn7)), (locals.var_keq_1d * (locals.var_xg10_op_dn8 - locals.var_xg2eff_op_dn8)), (locals.var_keq_1d * (locals.var_xg10_op_dn9 - locals.var_xg2eff_op_dn9)), );
            locals.var_temp_rv = 0.0;
        }
        let assign45000_e50121: f64 = if p.p13 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1363 = assign45000_e50121;
        locals.var_guard1363_rv = 0.0;
        if ((locals.var_guard1360 != 0.0) && (locals.var_guard1363 != 0.0)) {
            let assign45010_e50128: f64 = (locals.var_temp + locals.var_emin);
            let assign45010_e50131: f64 = (locals.var_temp - locals.var_emin);
            let assign45010_e50134: f64 = (locals.var_temp - locals.var_emin);
            let assign45010_e50135: f64 = (assign45010_e50131 * assign45010_e50134);
            let assign45010_e50138: f64 = (locals.var_emin * locals.var_emin);
            let assign45010_e50139: f64 = (assign45010_e50135 + assign45010_e50138);
            let assign45010_e50140: f64 = (assign45010_e50139).sqrt();
            let assign45010_e50141: f64 = (assign45010_e50128 + assign45010_e50140);
            let assign45010_e50142: f64 = (0.5 * assign45010_e50141);
            (locals.var_e1_op, locals.var_e1_op_dn4, locals.var_e1_op_dn6, locals.var_e1_op_dn7, locals.var_e1_op_dn8, locals.var_e1_op_dn9, ) = (assign45010_e50142, (0.5 * ((locals.var_temp_dn4 + locals.var_emin_dn4) + (((((locals.var_temp_dn4 - locals.var_emin_dn4) * assign45010_e50134) + (assign45010_e50131 * (locals.var_temp_dn4 - locals.var_emin_dn4))) + ((locals.var_emin_dn4 * locals.var_emin) + (locals.var_emin * locals.var_emin_dn4))) / (2.0 * assign45010_e50140)))), (0.5 * ((locals.var_temp_dn6 + locals.var_emin_dn6) + (((((locals.var_temp_dn6 - locals.var_emin_dn6) * assign45010_e50134) + (assign45010_e50131 * (locals.var_temp_dn6 - locals.var_emin_dn6))) + ((locals.var_emin_dn6 * locals.var_emin) + (locals.var_emin * locals.var_emin_dn6))) / (2.0 * assign45010_e50140)))), (0.5 * ((locals.var_temp_dn7 + locals.var_emin_dn7) + (((((locals.var_temp_dn7 - locals.var_emin_dn7) * assign45010_e50134) + (assign45010_e50131 * (locals.var_temp_dn7 - locals.var_emin_dn7))) + ((locals.var_emin_dn7 * locals.var_emin) + (locals.var_emin * locals.var_emin_dn7))) / (2.0 * assign45010_e50140)))), (0.5 * ((locals.var_temp_dn8 + locals.var_emin_dn8) + (((((locals.var_temp_dn8 - locals.var_emin_dn8) * assign45010_e50134) + (assign45010_e50131 * (locals.var_temp_dn8 - locals.var_emin_dn8))) + ((locals.var_emin_dn8 * locals.var_emin) + (locals.var_emin * locals.var_emin_dn8))) / (2.0 * assign45010_e50140)))), (0.5 * ((locals.var_temp_dn9 + locals.var_emin_dn9) + (((((locals.var_temp_dn9 - locals.var_emin_dn9) * assign45010_e50134) + (assign45010_e50131 * (locals.var_temp_dn9 - locals.var_emin_dn9))) + ((locals.var_emin_dn9 * locals.var_emin) + (locals.var_emin * locals.var_emin_dn9))) / (2.0 * assign45010_e50140)))), );
            locals.var_e1_op_rv = 0.0;
        }
        if ((locals.var_guard1360 != 0.0) && (locals.var_guard1363 != 0.0)) {
            let assign45020_e50150: f64 = (-locals.var_temp);
            let assign45020_e50152: f64 = (assign45020_e50150 + locals.var_emin);
            let assign45020_e50154: f64 = (-locals.var_temp);
            let assign45020_e50156: f64 = (assign45020_e50154 - locals.var_emin);
            let assign45020_e50158: f64 = (-locals.var_temp);
            let assign45020_e50160: f64 = (assign45020_e50158 - locals.var_emin);
            let assign45020_e50161: f64 = (assign45020_e50156 * assign45020_e50160);
            let assign45020_e50164: f64 = (locals.var_emin * locals.var_emin);
            let assign45020_e50165: f64 = (assign45020_e50161 + assign45020_e50164);
            let assign45020_e50166: f64 = (assign45020_e50165).sqrt();
            let assign45020_e50167: f64 = (assign45020_e50152 + assign45020_e50166);
            let assign45020_e50168: f64 = (0.5 * assign45020_e50167);
            (locals.var_e2_op, locals.var_e2_op_dn4, locals.var_e2_op_dn6, locals.var_e2_op_dn7, locals.var_e2_op_dn8, locals.var_e2_op_dn9, ) = (assign45020_e50168, (0.5 * (((-locals.var_temp_dn4) + locals.var_emin_dn4) + ((((((-locals.var_temp_dn4) - locals.var_emin_dn4) * assign45020_e50160) + (assign45020_e50156 * ((-locals.var_temp_dn4) - locals.var_emin_dn4))) + ((locals.var_emin_dn4 * locals.var_emin) + (locals.var_emin * locals.var_emin_dn4))) / (2.0 * assign45020_e50166)))), (0.5 * (((-locals.var_temp_dn6) + locals.var_emin_dn6) + ((((((-locals.var_temp_dn6) - locals.var_emin_dn6) * assign45020_e50160) + (assign45020_e50156 * ((-locals.var_temp_dn6) - locals.var_emin_dn6))) + ((locals.var_emin_dn6 * locals.var_emin) + (locals.var_emin * locals.var_emin_dn6))) / (2.0 * assign45020_e50166)))), (0.5 * (((-locals.var_temp_dn7) + locals.var_emin_dn7) + ((((((-locals.var_temp_dn7) - locals.var_emin_dn7) * assign45020_e50160) + (assign45020_e50156 * ((-locals.var_temp_dn7) - locals.var_emin_dn7))) + ((locals.var_emin_dn7 * locals.var_emin) + (locals.var_emin * locals.var_emin_dn7))) / (2.0 * assign45020_e50166)))), (0.5 * (((-locals.var_temp_dn8) + locals.var_emin_dn8) + ((((((-locals.var_temp_dn8) - locals.var_emin_dn8) * assign45020_e50160) + (assign45020_e50156 * ((-locals.var_temp_dn8) - locals.var_emin_dn8))) + ((locals.var_emin_dn8 * locals.var_emin) + (locals.var_emin * locals.var_emin_dn8))) / (2.0 * assign45020_e50166)))), (0.5 * (((-locals.var_temp_dn9) + locals.var_emin_dn9) + ((((((-locals.var_temp_dn9) - locals.var_emin_dn9) * assign45020_e50160) + (assign45020_e50156 * ((-locals.var_temp_dn9) - locals.var_emin_dn9))) + ((locals.var_emin_dn9 * locals.var_emin) + (locals.var_emin * locals.var_emin_dn9))) / (2.0 * assign45020_e50166)))), );
            locals.var_e2_op_rv = 0.0;
        }
        if ((locals.var_guard1360 != 0.0) && (locals.var_guard1363 != 0.0)) {
            let assign45030_e50176: f64 = (-0.3333333333333);
            let assign45030_e50178: f64 = (locals.var_e1_op).ln();
            let assign45030_e50179: f64 = (assign45030_e50176 * assign45030_e50178);
            let assign45030_e50180: f64 = (assign45030_e50179).exp();
            let assign45030_e50181: f64 = (locals.var_qq_op * assign45030_e50180);
            (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9, ) = (assign45030_e50181, ((locals.var_qq_op_dn4 * assign45030_e50180) + (locals.var_qq_op * (assign45030_e50180 * (assign45030_e50176 * (locals.var_e1_op_dn4 / locals.var_e1_op))))), ((locals.var_qq_op_dn6 * assign45030_e50180) + (locals.var_qq_op * (assign45030_e50180 * (assign45030_e50176 * (locals.var_e1_op_dn6 / locals.var_e1_op))))), ((locals.var_qq_op_dn7 * assign45030_e50180) + (locals.var_qq_op * (assign45030_e50180 * (assign45030_e50176 * (locals.var_e1_op_dn7 / locals.var_e1_op))))), ((locals.var_qq_op_dn8 * assign45030_e50180) + (locals.var_qq_op * (assign45030_e50180 * (assign45030_e50176 * (locals.var_e1_op_dn8 / locals.var_e1_op))))), ((locals.var_qq_op_dn9 * assign45030_e50180) + (locals.var_qq_op * (assign45030_e50180 * (assign45030_e50176 * (locals.var_e1_op_dn9 / locals.var_e1_op))))), );
            locals.var_temp1_rv = 0.0;
        }
        if ((locals.var_guard1360 != 0.0) && (locals.var_guard1363 != 0.0)) {
            let assign45040_e50189: f64 = (-0.3333333333333);
            let assign45040_e50191: f64 = (locals.var_e2_op).ln();
            let assign45040_e50192: f64 = (assign45040_e50189 * assign45040_e50191);
            let assign45040_e50193: f64 = (assign45040_e50192).exp();
            let assign45040_e50194: f64 = (locals.var_qq_op * assign45040_e50193);
            (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9, ) = (assign45040_e50194, ((locals.var_qq_op_dn4 * assign45040_e50193) + (locals.var_qq_op * (assign45040_e50193 * (assign45040_e50189 * (locals.var_e2_op_dn4 / locals.var_e2_op))))), ((locals.var_qq_op_dn6 * assign45040_e50193) + (locals.var_qq_op * (assign45040_e50193 * (assign45040_e50189 * (locals.var_e2_op_dn6 / locals.var_e2_op))))), ((locals.var_qq_op_dn7 * assign45040_e50193) + (locals.var_qq_op * (assign45040_e50193 * (assign45040_e50189 * (locals.var_e2_op_dn7 / locals.var_e2_op))))), ((locals.var_qq_op_dn8 * assign45040_e50193) + (locals.var_qq_op * (assign45040_e50193 * (assign45040_e50189 * (locals.var_e2_op_dn8 / locals.var_e2_op))))), ((locals.var_qq_op_dn9 * assign45040_e50193) + (locals.var_qq_op * (assign45040_e50193 * (assign45040_e50189 * (locals.var_e2_op_dn9 / locals.var_e2_op))))), );
            locals.var_temp2_rv = 0.0;
        }
        if ((locals.var_guard1360 != 0.0) && (locals.var_guard1363 != 0.0)) {
            let assign45050_e50202: f64 = (1.0 - locals.var_temp1);
            let assign45050_e50204: f64 = (assign45050_e50202 - locals.var_temp2);
            (locals.var_temp3, locals.var_temp3_dn4, locals.var_temp3_dn6, locals.var_temp3_dn7, locals.var_temp3_dn8, locals.var_temp3_dn9, ) = (assign45050_e50204, ((-locals.var_temp1_dn4) - locals.var_temp2_dn4), ((-locals.var_temp1_dn6) - locals.var_temp2_dn6), ((-locals.var_temp1_dn7) - locals.var_temp2_dn7), ((-locals.var_temp1_dn8) - locals.var_temp2_dn8), ((-locals.var_temp1_dn9) - locals.var_temp2_dn9), );
            locals.var_temp3_rv = 0.0;
        }
        if ((locals.var_guard1360 != 0.0) && (locals.var_guard1363 != 0.0)) {
            let assign45070_e50220: f64 = (locals.var_k1_1d * locals.var_temp3);
            let assign45070_e50224: f64 = (locals.var_k1_1d * locals.var_temp1);
            let assign45070_e50225: f64 = (1.0 + assign45070_e50224);
            let assign45070_e50226: f64 = (assign45070_e50220 / assign45070_e50225);
            (locals.var_k1_1d_qm_op, locals.var_k1_1d_qm_op_dn4, locals.var_k1_1d_qm_op_dn6, locals.var_k1_1d_qm_op_dn7, locals.var_k1_1d_qm_op_dn8, locals.var_k1_1d_qm_op_dn9, ) = (assign45070_e50226, ((((locals.var_k1_1d * locals.var_temp3_dn4) * assign45070_e50225) - (assign45070_e50220 * (locals.var_k1_1d * locals.var_temp1_dn4))) / (assign45070_e50225 * assign45070_e50225)), ((((locals.var_k1_1d * locals.var_temp3_dn6) * assign45070_e50225) - (assign45070_e50220 * (locals.var_k1_1d * locals.var_temp1_dn6))) / (assign45070_e50225 * assign45070_e50225)), ((((locals.var_k1_1d * locals.var_temp3_dn7) * assign45070_e50225) - (assign45070_e50220 * (locals.var_k1_1d * locals.var_temp1_dn7))) / (assign45070_e50225 * assign45070_e50225)), ((((locals.var_k1_1d * locals.var_temp3_dn8) * assign45070_e50225) - (assign45070_e50220 * (locals.var_k1_1d * locals.var_temp1_dn8))) / (assign45070_e50225 * assign45070_e50225)), ((((locals.var_k1_1d * locals.var_temp3_dn9) * assign45070_e50225) - (assign45070_e50220 * (locals.var_k1_1d * locals.var_temp1_dn9))) / (assign45070_e50225 * assign45070_e50225)), );
            locals.var_k1_1d_qm_op_rv = 0.0;
        }
        if ((locals.var_guard1360 != 0.0) && (locals.var_guard1363 != 0.0)) {
            let assign45080_e50234: f64 = (locals.var_k2_1d * locals.var_temp3);
            let assign45080_e50238: f64 = (locals.var_k2_1d * locals.var_temp2);
            let assign45080_e50239: f64 = (1.0 + assign45080_e50238);
            let assign45080_e50240: f64 = (assign45080_e50234 / assign45080_e50239);
            (locals.var_k2_1d_qm_op, locals.var_k2_1d_qm_op_dn4, locals.var_k2_1d_qm_op_dn6, locals.var_k2_1d_qm_op_dn7, locals.var_k2_1d_qm_op_dn8, locals.var_k2_1d_qm_op_dn9, ) = (assign45080_e50240, ((((locals.var_k2_1d * locals.var_temp3_dn4) * assign45080_e50239) - (assign45080_e50234 * (locals.var_k2_1d * locals.var_temp2_dn4))) / (assign45080_e50239 * assign45080_e50239)), ((((locals.var_k2_1d * locals.var_temp3_dn6) * assign45080_e50239) - (assign45080_e50234 * (locals.var_k2_1d * locals.var_temp2_dn6))) / (assign45080_e50239 * assign45080_e50239)), ((((locals.var_k2_1d * locals.var_temp3_dn7) * assign45080_e50239) - (assign45080_e50234 * (locals.var_k2_1d * locals.var_temp2_dn7))) / (assign45080_e50239 * assign45080_e50239)), ((((locals.var_k2_1d * locals.var_temp3_dn8) * assign45080_e50239) - (assign45080_e50234 * (locals.var_k2_1d * locals.var_temp2_dn8))) / (assign45080_e50239 * assign45080_e50239)), ((((locals.var_k2_1d * locals.var_temp3_dn9) * assign45080_e50239) - (assign45080_e50234 * (locals.var_k2_1d * locals.var_temp2_dn9))) / (assign45080_e50239 * assign45080_e50239)), );
            locals.var_k2_1d_qm_op_rv = 0.0;
        }
        if ((locals.var_guard1360 != 0.0) && (locals.var_guard1363 != 0.0)) {
            let assign45090_e50250: f64 = (1.0 / locals.var_k1_1d_qm_op);
            let assign45090_e50251: f64 = (1.0 + assign45090_e50250);
            let assign45090_e50254: f64 = (1.0 / locals.var_k2_1d_qm_op);
            let assign45090_e50255: f64 = (assign45090_e50251 + assign45090_e50254);
            let assign45090_e50256: f64 = (1.0 / assign45090_e50255);
            (locals.var_keq_1d_qm_op, locals.var_keq_1d_qm_op_dn4, locals.var_keq_1d_qm_op_dn6, locals.var_keq_1d_qm_op_dn7, locals.var_keq_1d_qm_op_dn8, locals.var_keq_1d_qm_op_dn9, ) = (assign45090_e50256, (-(((-(locals.var_k1_1d_qm_op_dn4 / (locals.var_k1_1d_qm_op * locals.var_k1_1d_qm_op))) + (-(locals.var_k2_1d_qm_op_dn4 / (locals.var_k2_1d_qm_op * locals.var_k2_1d_qm_op)))) / (assign45090_e50255 * assign45090_e50255))), (-(((-(locals.var_k1_1d_qm_op_dn6 / (locals.var_k1_1d_qm_op * locals.var_k1_1d_qm_op))) + (-(locals.var_k2_1d_qm_op_dn6 / (locals.var_k2_1d_qm_op * locals.var_k2_1d_qm_op)))) / (assign45090_e50255 * assign45090_e50255))), (-(((-(locals.var_k1_1d_qm_op_dn7 / (locals.var_k1_1d_qm_op * locals.var_k1_1d_qm_op))) + (-(locals.var_k2_1d_qm_op_dn7 / (locals.var_k2_1d_qm_op * locals.var_k2_1d_qm_op)))) / (assign45090_e50255 * assign45090_e50255))), (-(((-(locals.var_k1_1d_qm_op_dn8 / (locals.var_k1_1d_qm_op * locals.var_k1_1d_qm_op))) + (-(locals.var_k2_1d_qm_op_dn8 / (locals.var_k2_1d_qm_op * locals.var_k2_1d_qm_op)))) / (assign45090_e50255 * assign45090_e50255))), (-(((-(locals.var_k1_1d_qm_op_dn9 / (locals.var_k1_1d_qm_op * locals.var_k1_1d_qm_op))) + (-(locals.var_k2_1d_qm_op_dn9 / (locals.var_k2_1d_qm_op * locals.var_k2_1d_qm_op)))) / (assign45090_e50255 * assign45090_e50255))), );
            locals.var_keq_1d_qm_op_rv = 0.0;
        }
        if ((locals.var_guard1360 != 0.0) && (locals.var_guard1363 == 0.0)) {
            (locals.var_k1_1d_qm_op, locals.var_k1_1d_qm_op_dn4, locals.var_k1_1d_qm_op_dn6, locals.var_k1_1d_qm_op_dn7, locals.var_k1_1d_qm_op_dn8, locals.var_k1_1d_qm_op_dn9, ) = (locals.var_k1_1d, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_k1_1d_qm_op_rv = 0.0;
            (locals.var_k2_1d_qm_op, locals.var_k2_1d_qm_op_dn4, locals.var_k2_1d_qm_op_dn6, locals.var_k2_1d_qm_op_dn7, locals.var_k2_1d_qm_op_dn8, locals.var_k2_1d_qm_op_dn9, ) = (locals.var_k2_1d, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_k2_1d_qm_op_rv = 0.0;
            (locals.var_keq_1d_qm_op, locals.var_keq_1d_qm_op_dn4, locals.var_keq_1d_qm_op_dn6, locals.var_keq_1d_qm_op_dn7, locals.var_keq_1d_qm_op_dn8, locals.var_keq_1d_qm_op_dn9, ) = (locals.var_keq_1d, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_keq_1d_qm_op_rv = 0.0;
        }
        if (locals.var_guard1360 != 0.0) {
            let assign45140_e50291: f64 = (locals.var_xg10_op - locals.var_xg2eff_op);
            let assign45140_e50292: f64 = (locals.var_keq_1d_qm_op * assign45140_e50291);
            (locals.var_dx_wi_1d_op, locals.var_dx_wi_1d_op_dn4, locals.var_dx_wi_1d_op_dn6, locals.var_dx_wi_1d_op_dn7, locals.var_dx_wi_1d_op_dn8, locals.var_dx_wi_1d_op_dn9, ) = (assign45140_e50292, ((locals.var_keq_1d_qm_op_dn4 * assign45140_e50291) + (locals.var_keq_1d_qm_op * (locals.var_xg10_op_dn4 - locals.var_xg2eff_op_dn4))), ((locals.var_keq_1d_qm_op_dn6 * assign45140_e50291) + (locals.var_keq_1d_qm_op * (locals.var_xg10_op_dn6 - locals.var_xg2eff_op_dn6))), ((locals.var_keq_1d_qm_op_dn7 * assign45140_e50291) + (locals.var_keq_1d_qm_op * (locals.var_xg10_op_dn7 - locals.var_xg2eff_op_dn7))), ((locals.var_keq_1d_qm_op_dn8 * assign45140_e50291) + (locals.var_keq_1d_qm_op * (locals.var_xg10_op_dn8 - locals.var_xg2eff_op_dn8))), ((locals.var_keq_1d_qm_op_dn9 * assign45140_e50291) + (locals.var_keq_1d_qm_op * (locals.var_xg10_op_dn9 - locals.var_xg2eff_op_dn9))), );
            locals.var_dx_wi_1d_op_rv = 0.0;
        }
        let assign45150_e50297: f64 = if locals.var_dx_wi_1d_op > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1364 = assign45150_e50297;
        locals.var_guard1364_rv = 0.0;
        let assign45160_e50299: f64 = (-locals.var_dx_wi_1d_op);
        let assign45160_e50301: f64 = if assign45160_e50299 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard1365 = assign45160_e50301;
        locals.var_guard1365_rv = 0.0;
        if (((locals.var_guard1360 != 0.0) && (locals.var_guard1364 != 0.0)) && (locals.var_guard1365 != 0.0)) {
            let assign45170_e50309: f64 = (-locals.var_dx_wi_1d_op);
            let assign45170_e50310: f64 = (assign45170_e50309).exp();
            let assign45170_e50311: f64 = (1.0 + assign45170_e50310);
            let assign45170_e50312: f64 = (assign45170_e50311).ln();
            (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign45170_e50312, ((assign45170_e50310 * (-locals.var_dx_wi_1d_op_dn4)) / assign45170_e50311), ((assign45170_e50310 * (-locals.var_dx_wi_1d_op_dn6)) / assign45170_e50311), ((assign45170_e50310 * (-locals.var_dx_wi_1d_op_dn7)) / assign45170_e50311), ((assign45170_e50310 * (-locals.var_dx_wi_1d_op_dn8)) / assign45170_e50311), ((assign45170_e50310 * (-locals.var_dx_wi_1d_op_dn9)) / assign45170_e50311), );
            locals.var_temp_rv = 0.0;
        }
        if (((locals.var_guard1360 != 0.0) && (locals.var_guard1364 != 0.0)) && (locals.var_guard1365 == 0.0)) {
            let assign45180_e50322: f64 = (-locals.var_dx_wi_1d_op);
            (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign45180_e50322, (-locals.var_dx_wi_1d_op_dn4), (-locals.var_dx_wi_1d_op_dn6), (-locals.var_dx_wi_1d_op_dn7), (-locals.var_dx_wi_1d_op_dn8), (-locals.var_dx_wi_1d_op_dn9), );
            locals.var_temp_rv = 0.0;
        }
        if ((locals.var_guard1360 != 0.0) && (locals.var_guard1364 != 0.0)) {
            let assign45190_e50331: f64 = (locals.var_dx_wi_1d_op / locals.var_k1_1d_qm_op);
            let assign45190_e50332: f64 = (locals.var_xg10_op - assign45190_e50331);
            let assign45190_e50334: f64 = (assign45190_e50332 + locals.var_temp);
            let assign45190_e50336: f64 = (assign45190_e50334 - 0.6931471805599);
            (locals.var_x_wi_1d_op, locals.var_x_wi_1d_op_dn4, locals.var_x_wi_1d_op_dn6, locals.var_x_wi_1d_op_dn7, locals.var_x_wi_1d_op_dn8, locals.var_x_wi_1d_op_dn9, ) = (assign45190_e50336, ((locals.var_xg10_op_dn4 - (((locals.var_dx_wi_1d_op_dn4 * locals.var_k1_1d_qm_op) - (locals.var_dx_wi_1d_op * locals.var_k1_1d_qm_op_dn4)) / (locals.var_k1_1d_qm_op * locals.var_k1_1d_qm_op))) + locals.var_temp_dn4), ((locals.var_xg10_op_dn6 - (((locals.var_dx_wi_1d_op_dn6 * locals.var_k1_1d_qm_op) - (locals.var_dx_wi_1d_op * locals.var_k1_1d_qm_op_dn6)) / (locals.var_k1_1d_qm_op * locals.var_k1_1d_qm_op))) + locals.var_temp_dn6), ((locals.var_xg10_op_dn7 - (((locals.var_dx_wi_1d_op_dn7 * locals.var_k1_1d_qm_op) - (locals.var_dx_wi_1d_op * locals.var_k1_1d_qm_op_dn7)) / (locals.var_k1_1d_qm_op * locals.var_k1_1d_qm_op))) + locals.var_temp_dn7), ((locals.var_xg10_op_dn8 - (((locals.var_dx_wi_1d_op_dn8 * locals.var_k1_1d_qm_op) - (locals.var_dx_wi_1d_op * locals.var_k1_1d_qm_op_dn8)) / (locals.var_k1_1d_qm_op * locals.var_k1_1d_qm_op))) + locals.var_temp_dn8), ((locals.var_xg10_op_dn9 - (((locals.var_dx_wi_1d_op_dn9 * locals.var_k1_1d_qm_op) - (locals.var_dx_wi_1d_op * locals.var_k1_1d_qm_op_dn9)) / (locals.var_k1_1d_qm_op * locals.var_k1_1d_qm_op))) + locals.var_temp_dn9), );
            locals.var_x_wi_1d_op_rv = 0.0;
        }
        let assign45200_e50341: f64 = if locals.var_dx_wi_1d_op < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard1366 = assign45200_e50341;
        locals.var_guard1366_rv = 0.0;
        if (((locals.var_guard1360 != 0.0) && (locals.var_guard1364 == 0.0)) && (locals.var_guard1366 != 0.0)) {
            let assign45210_e50350: f64 = (locals.var_dx_wi_1d_op).exp();
            let assign45210_e50351: f64 = (1.0 + assign45210_e50350);
            let assign45210_e50352: f64 = (assign45210_e50351).ln();
            (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign45210_e50352, ((assign45210_e50350 * locals.var_dx_wi_1d_op_dn4) / assign45210_e50351), ((assign45210_e50350 * locals.var_dx_wi_1d_op_dn6) / assign45210_e50351), ((assign45210_e50350 * locals.var_dx_wi_1d_op_dn7) / assign45210_e50351), ((assign45210_e50350 * locals.var_dx_wi_1d_op_dn8) / assign45210_e50351), ((assign45210_e50350 * locals.var_dx_wi_1d_op_dn9) / assign45210_e50351), );
            locals.var_temp_rv = 0.0;
        }
        if (((locals.var_guard1360 != 0.0) && (locals.var_guard1364 == 0.0)) && (locals.var_guard1366 == 0.0)) {
            (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (locals.var_dx_wi_1d_op, locals.var_dx_wi_1d_op_dn4, locals.var_dx_wi_1d_op_dn6, locals.var_dx_wi_1d_op_dn7, locals.var_dx_wi_1d_op_dn8, locals.var_dx_wi_1d_op_dn9, );
            locals.var_temp_rv = 0.0;
        }
        if ((locals.var_guard1360 != 0.0) && (locals.var_guard1364 == 0.0)) {
            let assign45230_e50372: f64 = (locals.var_dx_wi_1d_op / locals.var_k2_1d_qm_op);
            let assign45230_e50373: f64 = (locals.var_xg2eff_op + assign45230_e50372);
            let assign45230_e50375: f64 = (assign45230_e50373 + locals.var_temp);
            let assign45230_e50377: f64 = (assign45230_e50375 - 0.6931471805599);
            (locals.var_x_wi_1d_op, locals.var_x_wi_1d_op_dn4, locals.var_x_wi_1d_op_dn6, locals.var_x_wi_1d_op_dn7, locals.var_x_wi_1d_op_dn8, locals.var_x_wi_1d_op_dn9, ) = (assign45230_e50377, ((locals.var_xg2eff_op_dn4 + (((locals.var_dx_wi_1d_op_dn4 * locals.var_k2_1d_qm_op) - (locals.var_dx_wi_1d_op * locals.var_k2_1d_qm_op_dn4)) / (locals.var_k2_1d_qm_op * locals.var_k2_1d_qm_op))) + locals.var_temp_dn4), ((locals.var_xg2eff_op_dn6 + (((locals.var_dx_wi_1d_op_dn6 * locals.var_k2_1d_qm_op) - (locals.var_dx_wi_1d_op * locals.var_k2_1d_qm_op_dn6)) / (locals.var_k2_1d_qm_op * locals.var_k2_1d_qm_op))) + locals.var_temp_dn6), ((locals.var_xg2eff_op_dn7 + (((locals.var_dx_wi_1d_op_dn7 * locals.var_k2_1d_qm_op) - (locals.var_dx_wi_1d_op * locals.var_k2_1d_qm_op_dn7)) / (locals.var_k2_1d_qm_op * locals.var_k2_1d_qm_op))) + locals.var_temp_dn7), ((locals.var_xg2eff_op_dn8 + (((locals.var_dx_wi_1d_op_dn8 * locals.var_k2_1d_qm_op) - (locals.var_dx_wi_1d_op * locals.var_k2_1d_qm_op_dn8)) / (locals.var_k2_1d_qm_op * locals.var_k2_1d_qm_op))) + locals.var_temp_dn8), ((locals.var_xg2eff_op_dn9 + (((locals.var_dx_wi_1d_op_dn9 * locals.var_k2_1d_qm_op) - (locals.var_dx_wi_1d_op * locals.var_k2_1d_qm_op_dn9)) / (locals.var_k2_1d_qm_op * locals.var_k2_1d_qm_op))) + locals.var_temp_dn9), );
            locals.var_x_wi_1d_op_rv = 0.0;
        }
        if (locals.var_guard1360 != 0.0) {
            let assign45240_e50384: f64 = (locals.var_x_wi_1d_op + locals.var_xth_1d_op);
            let assign45240_e50387: f64 = (locals.var_x_wi_1d_op - locals.var_xth_1d_op);
            let assign45240_e50390: f64 = (locals.var_x_wi_1d_op - locals.var_xth_1d_op);
            let assign45240_e50391: f64 = (assign45240_e50387 * assign45240_e50390);
            let assign45240_e50393: f64 = (assign45240_e50391 + 4.0);
            let assign45240_e50394: f64 = (assign45240_e50393).sqrt();
            let assign45240_e50395: f64 = (assign45240_e50384 - assign45240_e50394);
            let assign45240_e50396: f64 = (0.5 * assign45240_e50395);
            (locals.var_x_1d_op, locals.var_x_1d_op_dn4, locals.var_x_1d_op_dn6, locals.var_x_1d_op_dn7, locals.var_x_1d_op_dn8, locals.var_x_1d_op_dn9, ) = (assign45240_e50396, (0.5 * ((locals.var_x_wi_1d_op_dn4 + locals.var_xth_1d_op_dn4) - ((((locals.var_x_wi_1d_op_dn4 - locals.var_xth_1d_op_dn4) * assign45240_e50390) + (assign45240_e50387 * (locals.var_x_wi_1d_op_dn4 - locals.var_xth_1d_op_dn4))) / (2.0 * assign45240_e50394)))), (0.5 * ((locals.var_x_wi_1d_op_dn6 + locals.var_xth_1d_op_dn6) - ((((locals.var_x_wi_1d_op_dn6 - locals.var_xth_1d_op_dn6) * assign45240_e50390) + (assign45240_e50387 * (locals.var_x_wi_1d_op_dn6 - locals.var_xth_1d_op_dn6))) / (2.0 * assign45240_e50394)))), (0.5 * ((locals.var_x_wi_1d_op_dn7 + locals.var_xth_1d_op_dn7) - ((((locals.var_x_wi_1d_op_dn7 - locals.var_xth_1d_op_dn7) * assign45240_e50390) + (assign45240_e50387 * (locals.var_x_wi_1d_op_dn7 - locals.var_xth_1d_op_dn7))) / (2.0 * assign45240_e50394)))), (0.5 * ((locals.var_x_wi_1d_op_dn8 + locals.var_xth_1d_op_dn8) - ((((locals.var_x_wi_1d_op_dn8 - locals.var_xth_1d_op_dn8) * assign45240_e50390) + (assign45240_e50387 * (locals.var_x_wi_1d_op_dn8 - locals.var_xth_1d_op_dn8))) / (2.0 * assign45240_e50394)))), (0.5 * ((locals.var_x_wi_1d_op_dn9 + locals.var_xth_1d_op_dn9) - ((((locals.var_x_wi_1d_op_dn9 - locals.var_xth_1d_op_dn9) * assign45240_e50390) + (assign45240_e50387 * (locals.var_x_wi_1d_op_dn9 - locals.var_xth_1d_op_dn9))) / (2.0 * assign45240_e50394)))), );
            locals.var_x_1d_op_rv = 0.0;
        }
        if (locals.var_guard1360 != 0.0) {
            let assign45250_e50404: f64 = (locals.var_xth_1d_op - locals.var_x_1d_op);
            let assign45250_e50405: f64 = (2.0 * assign45250_e50404);
            let assign45250_e50407: f64 = (assign45250_e50405 / locals.var_xsddep_op);
            let assign45250_e50408: f64 = (1.0 + assign45250_e50407);
            let assign45250_e50409: f64 = (assign45250_e50408).sqrt();
            let assign45250_e50411: f64 = (assign45250_e50409 - 1.0);
            (locals.var_dleff_op, locals.var_dleff_op_dn4, locals.var_dleff_op_dn6, locals.var_dleff_op_dn7, locals.var_dleff_op_dn8, locals.var_dleff_op_dn9, ) = (assign45250_e50411, (((((2.0 * (locals.var_xth_1d_op_dn4 - locals.var_x_1d_op_dn4)) * locals.var_xsddep_op) - (assign45250_e50405 * locals.var_xsddep_op_dn4)) / (locals.var_xsddep_op * locals.var_xsddep_op)) / (2.0 * assign45250_e50409)), (((((2.0 * (locals.var_xth_1d_op_dn6 - locals.var_x_1d_op_dn6)) * locals.var_xsddep_op) - (assign45250_e50405 * locals.var_xsddep_op_dn6)) / (locals.var_xsddep_op * locals.var_xsddep_op)) / (2.0 * assign45250_e50409)), (((((2.0 * (locals.var_xth_1d_op_dn7 - locals.var_x_1d_op_dn7)) * locals.var_xsddep_op) - (assign45250_e50405 * locals.var_xsddep_op_dn7)) / (locals.var_xsddep_op * locals.var_xsddep_op)) / (2.0 * assign45250_e50409)), (((((2.0 * (locals.var_xth_1d_op_dn8 - locals.var_x_1d_op_dn8)) * locals.var_xsddep_op) - (assign45250_e50405 * locals.var_xsddep_op_dn8)) / (locals.var_xsddep_op * locals.var_xsddep_op)) / (2.0 * assign45250_e50409)), (((((2.0 * (locals.var_xth_1d_op_dn9 - locals.var_x_1d_op_dn9)) * locals.var_xsddep_op) - (assign45250_e50405 * locals.var_xsddep_op_dn9)) / (locals.var_xsddep_op * locals.var_xsddep_op)) / (2.0 * assign45250_e50409)), );
            locals.var_dleff_op_rv = 0.0;
        }
    }
    pub(super) fn stamp_reactive_block_64(
        locals: &mut StampLocals,
    ) {
        if (locals.var_guard1360 != 0.0) {
            let assign45270_e50427: f64 = (locals.var_pscedlb_i * locals.var_xg20_op);
            let assign45270_e50428: f64 = (1.0 + assign45270_e50427);
            let assign45270_e50430: f64 = (assign45270_e50428 + 0.5);
            let assign45270_e50434: f64 = (locals.var_pscedlb_i * locals.var_xg20_op);
            let assign45270_e50435: f64 = (1.0 + assign45270_e50434);
            let assign45270_e50437: f64 = (assign45270_e50435 - 0.5);
            let assign45270_e50441: f64 = (locals.var_pscedlb_i * locals.var_xg20_op);
            let assign45270_e50442: f64 = (1.0 + assign45270_e50441);
            let assign45270_e50444: f64 = (assign45270_e50442 - 0.5);
            let assign45270_e50445: f64 = (assign45270_e50437 * assign45270_e50444);
            let assign45270_e50447: f64 = (assign45270_e50445 + 0.01);
            let assign45270_e50448: f64 = (assign45270_e50447).sqrt();
            let assign45270_e50449: f64 = (assign45270_e50430 + assign45270_e50448);
            let assign45270_e50450: f64 = (0.5 * assign45270_e50449);
            (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign45270_e50450, (0.5 * ((locals.var_pscedlb_i * locals.var_xg20_op_dn4) + ((((locals.var_pscedlb_i * locals.var_xg20_op_dn4) * assign45270_e50444) + (assign45270_e50437 * (locals.var_pscedlb_i * locals.var_xg20_op_dn4))) / (2.0 * assign45270_e50448)))), (0.5 * ((locals.var_pscedlb_i * locals.var_xg20_op_dn6) + ((((locals.var_pscedlb_i * locals.var_xg20_op_dn6) * assign45270_e50444) + (assign45270_e50437 * (locals.var_pscedlb_i * locals.var_xg20_op_dn6))) / (2.0 * assign45270_e50448)))), (0.5 * ((locals.var_pscedlb_i * locals.var_xg20_op_dn7) + ((((locals.var_pscedlb_i * locals.var_xg20_op_dn7) * assign45270_e50444) + (assign45270_e50437 * (locals.var_pscedlb_i * locals.var_xg20_op_dn7))) / (2.0 * assign45270_e50448)))), (0.5 * ((locals.var_pscedlb_i * locals.var_xg20_op_dn8) + ((((locals.var_pscedlb_i * locals.var_xg20_op_dn8) * assign45270_e50444) + (assign45270_e50437 * (locals.var_pscedlb_i * locals.var_xg20_op_dn8))) / (2.0 * assign45270_e50448)))), (0.5 * ((locals.var_pscedlb_i * locals.var_xg20_op_dn9) + ((((locals.var_pscedlb_i * locals.var_xg20_op_dn9) * assign45270_e50444) + (assign45270_e50437 * (locals.var_pscedlb_i * locals.var_xg20_op_dn9))) / (2.0 * assign45270_e50448)))), );
            locals.var_temp_rv = 0.0;
        }
        if (locals.var_guard1360 != 0.0) {
            let assign45300_e50476: f64 = (2.0 * locals.var_xd0_op);
            let assign45300_e50480: f64 = (locals.var_xdsx_op / locals.var_xd0_op);
            let assign45300_e50481: f64 = (1.0 + assign45300_e50480);
            let assign45300_e50482: f64 = (assign45300_e50481).sqrt();
            let assign45300_e50484: f64 = (assign45300_e50482 - 1.0);
            let assign45300_e50485: f64 = (assign45300_e50476 * assign45300_e50484);
            let assign45300_e50489: f64 = (locals.var_cfdl_i * locals.var_dleff_op);
            let assign45300_e50490: f64 = (1.0 + assign45300_e50489);
            let assign45300_e50491: f64 = (assign45300_e50485 * assign45300_e50490);
            let assign45300_e50495: f64 = (locals.var_cfdlb_i * locals.var_xg20_op);
            let assign45300_e50496: f64 = (1.0 + assign45300_e50495);
            let assign45300_e50497: f64 = (assign45300_e50491 * assign45300_e50496);
            (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign45300_e50497, (((((((2.0 * locals.var_xd0_op_dn4) * assign45300_e50484) + (assign45300_e50476 * ((((locals.var_xdsx_op_dn4 * locals.var_xd0_op) - (locals.var_xdsx_op * locals.var_xd0_op_dn4)) / (locals.var_xd0_op * locals.var_xd0_op)) / (2.0 * assign45300_e50482)))) * assign45300_e50490) + (assign45300_e50485 * (locals.var_cfdl_i * locals.var_dleff_op_dn4))) * assign45300_e50496) + (assign45300_e50491 * (locals.var_cfdlb_i * locals.var_xg20_op_dn4))), (((((((2.0 * locals.var_xd0_op_dn6) * assign45300_e50484) + (assign45300_e50476 * ((((locals.var_xdsx_op_dn6 * locals.var_xd0_op) - (locals.var_xdsx_op * locals.var_xd0_op_dn6)) / (locals.var_xd0_op * locals.var_xd0_op)) / (2.0 * assign45300_e50482)))) * assign45300_e50490) + (assign45300_e50485 * (locals.var_cfdl_i * locals.var_dleff_op_dn6))) * assign45300_e50496) + (assign45300_e50491 * (locals.var_cfdlb_i * locals.var_xg20_op_dn6))), (((((((2.0 * locals.var_xd0_op_dn7) * assign45300_e50484) + (assign45300_e50476 * ((((locals.var_xdsx_op_dn7 * locals.var_xd0_op) - (locals.var_xdsx_op * locals.var_xd0_op_dn7)) / (locals.var_xd0_op * locals.var_xd0_op)) / (2.0 * assign45300_e50482)))) * assign45300_e50490) + (assign45300_e50485 * (locals.var_cfdl_i * locals.var_dleff_op_dn7))) * assign45300_e50496) + (assign45300_e50491 * (locals.var_cfdlb_i * locals.var_xg20_op_dn7))), (((((((2.0 * locals.var_xd0_op_dn8) * assign45300_e50484) + (assign45300_e50476 * ((((locals.var_xdsx_op_dn8 * locals.var_xd0_op) - (locals.var_xdsx_op * locals.var_xd0_op_dn8)) / (locals.var_xd0_op * locals.var_xd0_op)) / (2.0 * assign45300_e50482)))) * assign45300_e50490) + (assign45300_e50485 * (locals.var_cfdl_i * locals.var_dleff_op_dn8))) * assign45300_e50496) + (assign45300_e50491 * (locals.var_cfdlb_i * locals.var_xg20_op_dn8))), (((((((2.0 * locals.var_xd0_op_dn9) * assign45300_e50484) + (assign45300_e50476 * ((((locals.var_xdsx_op_dn9 * locals.var_xd0_op) - (locals.var_xdsx_op * locals.var_xd0_op_dn9)) / (locals.var_xd0_op * locals.var_xd0_op)) / (2.0 * assign45300_e50482)))) * assign45300_e50490) + (assign45300_e50485 * (locals.var_cfdl_i * locals.var_dleff_op_dn9))) * assign45300_e50496) + (assign45300_e50491 * (locals.var_cfdlb_i * locals.var_xg20_op_dn9))), );
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
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv1 = ctx.node_voltage(nodes[1]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let (eq0_e500, eq0_e500_d_n4, eq0_e500_d_n6, eq0_e500_d_n7, eq0_e500_d_n8, eq0_e500_d_n9,) = {
    if (locals.var_guard1239 != 0.0) {
        let eq0_e498: f64 = (p.p14 * locals.var_idse);
        let eq0_e498_d_n4: f64 = (p.p14 * locals.var_idse_dn4);
        let eq0_e498_d_n6: f64 = (p.p14 * locals.var_idse_dn6);
        let eq0_e498_d_n7: f64 = (p.p14 * locals.var_idse_dn7);
        let eq0_e498_d_n8: f64 = (p.p14 * locals.var_idse_dn8);
        let eq0_e498_d_n9: f64 = (p.p14 * locals.var_idse_dn9);
        (eq0_e498, eq0_e498_d_n4, eq0_e498_d_n6, eq0_e498_d_n7, eq0_e498_d_n8, eq0_e498_d_n9,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq0_value: f64 = eq0_e500;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(6),
            Some(7),
            multiplicity * (eq0_value),
            [4, 6, 7, 8, 9],
            [multiplicity * (eq0_e500_d_n4), multiplicity * (eq0_e500_d_n6), multiplicity * (eq0_e500_d_n7), multiplicity * (eq0_e500_d_n8), multiplicity * (eq0_e500_d_n9)],
            [],
            [],
            1.0,
        );
        let (eq1_e507, eq1_e507_d_n4, eq1_e507_d_n6, eq1_e507_d_n7, eq1_e507_d_n8, eq1_e507_d_n9,) = {
    if (locals.var_guard1239 == 0.0) {
        let eq1_e505: f64 = (p.p14 * locals.var_idse);
        let eq1_e505_d_n4: f64 = (p.p14 * locals.var_idse_dn4);
        let eq1_e505_d_n6: f64 = (p.p14 * locals.var_idse_dn6);
        let eq1_e505_d_n7: f64 = (p.p14 * locals.var_idse_dn7);
        let eq1_e505_d_n8: f64 = (p.p14 * locals.var_idse_dn8);
        let eq1_e505_d_n9: f64 = (p.p14 * locals.var_idse_dn9);
        (eq1_e505, eq1_e505_d_n4, eq1_e505_d_n6, eq1_e505_d_n7, eq1_e505_d_n8, eq1_e505_d_n9,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq1_value: f64 = eq1_e507;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(7),
            Some(6),
            multiplicity * (eq1_value),
            [4, 6, 7, 8, 9],
            [multiplicity * (eq1_e507_d_n4), multiplicity * (eq1_e507_d_n6), multiplicity * (eq1_e507_d_n7), multiplicity * (eq1_e507_d_n8), multiplicity * (eq1_e507_d_n9)],
            [],
            [],
            1.0,
        );
        let eq2_e511: f64 = (locals.var_igidle - locals.var_igisle);
        let eq2_e511_d_n4: f64 = (locals.var_igidle_dn4 - locals.var_igisle_dn4);
        let eq2_e511_d_n6: f64 = (locals.var_igidle_dn6 - locals.var_igisle_dn6);
        let eq2_e511_d_n7: f64 = (locals.var_igidle_dn7 - locals.var_igisle_dn7);
        let eq2_e511_d_n8: f64 = (locals.var_igidle_dn8 - locals.var_igisle_dn8);
        let eq2_e511_d_n9: f64 = (locals.var_igidle_dn9 - locals.var_igisle_dn9);
        let eq2_e512: f64 = (p.p14 * eq2_e511);
        let eq2_e512_d_n4: f64 = (p.p14 * eq2_e511_d_n4);
        let eq2_e512_d_n6: f64 = (p.p14 * eq2_e511_d_n6);
        let eq2_e512_d_n7: f64 = (p.p14 * eq2_e511_d_n7);
        let eq2_e512_d_n8: f64 = (p.p14 * eq2_e511_d_n8);
        let eq2_e512_d_n9: f64 = (p.p14 * eq2_e511_d_n9);
        let eq2_value: f64 = eq2_e512;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(7),
            Some(6),
            multiplicity * (eq2_value),
            [4, 6, 7, 8, 9],
            [multiplicity * (eq2_e512_d_n4), multiplicity * (eq2_e512_d_n6), multiplicity * (eq2_e512_d_n7), multiplicity * (eq2_e512_d_n8), multiplicity * (eq2_e512_d_n9)],
            [],
            [],
            1.0,
        );
        let eq3_e515: f64 = (p.p14 * locals.var_igse);
        let eq3_e515_d_n4: f64 = (p.p14 * locals.var_igse_dn4);
        let eq3_e515_d_n6: f64 = (p.p14 * locals.var_igse_dn6);
        let eq3_e515_d_n7: f64 = (p.p14 * locals.var_igse_dn7);
        let eq3_e515_d_n8: f64 = (p.p14 * locals.var_igse_dn8);
        let eq3_e515_d_n9: f64 = (p.p14 * locals.var_igse_dn9);
        let eq3_value: f64 = eq3_e515;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(9),
            Some(6),
            multiplicity * (eq3_value),
            [4, 6, 7, 8, 9],
            [multiplicity * (eq3_e515_d_n4), multiplicity * (eq3_e515_d_n6), multiplicity * (eq3_e515_d_n7), multiplicity * (eq3_e515_d_n8), multiplicity * (eq3_e515_d_n9)],
            [],
            [],
            1.0,
        );
        let eq4_e518: f64 = (p.p14 * locals.var_igde);
        let eq4_e518_d_n4: f64 = (p.p14 * locals.var_igde_dn4);
        let eq4_e518_d_n6: f64 = (p.p14 * locals.var_igde_dn6);
        let eq4_e518_d_n7: f64 = (p.p14 * locals.var_igde_dn7);
        let eq4_e518_d_n8: f64 = (p.p14 * locals.var_igde_dn8);
        let eq4_e518_d_n9: f64 = (p.p14 * locals.var_igde_dn9);
        let eq4_value: f64 = eq4_e518;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(9),
            Some(7),
            multiplicity * (eq4_value),
            [4, 6, 7, 8, 9],
            [multiplicity * (eq4_e518_d_n4), multiplicity * (eq4_e518_d_n6), multiplicity * (eq4_e518_d_n7), multiplicity * (eq4_e518_d_n8), multiplicity * (eq4_e518_d_n9)],
            [],
            [],
            1.0,
        );
        let eq8_e524: f64 = (p.p31 * locals.var_gmin);
        let eq8_e526: f64 = (eq8_e524 * (nv7 - nv6));
        let eq8_value: f64 = eq8_e526;
        stamper.stamp_current_node2_local(
            Some(7),
            Some(6),
            multiplicity * (eq8_value),
            6,
            multiplicity * ((-eq8_e524)),
            7,
            multiplicity * (eq8_e524),
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
        let eq10_value: f64 = locals.var_ithrce;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(4),
            None,
            multiplicity * (eq10_value),
            [4, 6, 7, 8, 9],
            [multiplicity * (locals.var_ithrce_dn4), multiplicity * (locals.var_ithrce_dn6), multiplicity * (locals.var_ithrce_dn7), multiplicity * (locals.var_ithrce_dn8), multiplicity * (locals.var_ithrce_dn9)],
            [],
            [],
            1.0,
        );
        let (eq11_e538, eq11_e538_d_n1, eq11_e538_d_n9,) = {
    if (locals.var_guard1240 != 0.0) {
        let eq11_e532: f64 = (p.p31 * locals.var_mult_i_int);
        let eq11_e534: f64 = (eq11_e532 * locals.var_ggate);
        let eq11_e536: f64 = (eq11_e534 * (nv1 - nv9));
        (eq11_e536, eq11_e534, (-eq11_e534),)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        let eq11_value: f64 = eq11_e538;
        stamper.stamp_current_node2_local(
            Some(1),
            Some(9),
            multiplicity * (eq11_value),
            1,
            multiplicity * (eq11_e538_d_n1),
            9,
            multiplicity * (eq11_e538_d_n9),
        );
        let (eq13_e553,) = {
    if (locals.var_guard1240 == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq13_value: f64 = eq13_e553;
        stamper.stamp_potential_const_local(
            0,
            eq13_value,
        );
        let (eq14_e563, eq14_e563_d_n2, eq14_e563_d_n6,) = {
    if (locals.var_guard1241 != 0.0) {
        let eq14_e557: f64 = (p.p31 * locals.var_mult_i_int);
        let eq14_e559: f64 = (eq14_e557 * locals.var_gsource);
        let eq14_e561: f64 = (eq14_e559 * (nv2 - nv6));
        (eq14_e561, eq14_e559, (-eq14_e559),)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        let eq14_value: f64 = eq14_e563;
        stamper.stamp_current_node2_local(
            Some(2),
            Some(6),
            multiplicity * (eq14_value),
            2,
            multiplicity * (eq14_e563_d_n2),
            6,
            multiplicity * (eq14_e563_d_n6),
        );
        let (eq16_e578,) = {
    if (locals.var_guard1241 == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq16_value: f64 = eq16_e578;
        stamper.stamp_potential_const_local(
            1,
            eq16_value,
        );
        let (eq17_e588, eq17_e588_d_n0, eq17_e588_d_n7,) = {
    if (locals.var_guard1242 != 0.0) {
        let eq17_e582: f64 = (p.p31 * locals.var_mult_i_int);
        let eq17_e584: f64 = (eq17_e582 * locals.var_gdrain);
        let eq17_e586: f64 = (eq17_e584 * (nv0 - nv7));
        (eq17_e586, eq17_e584, (-eq17_e584),)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        let eq17_value: f64 = eq17_e588;
        stamper.stamp_current_node2_local(
            Some(0),
            Some(7),
            multiplicity * (eq17_value),
            0,
            multiplicity * (eq17_e588_d_n0),
            7,
            multiplicity * (eq17_e588_d_n7),
        );
        let (eq19_e603,) = {
    if (locals.var_guard1242 == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq19_value: f64 = eq19_e603;
        stamper.stamp_potential_const_local(
            2,
            eq19_value,
        );
        let (eq20_e613, eq20_e613_d_n3, eq20_e613_d_n8,) = {
    if (locals.var_guard1243 != 0.0) {
        let eq20_e607: f64 = (p.p31 * locals.var_mult_i_int);
        let eq20_e609: f64 = (eq20_e607 * locals.var_gwell);
        let eq20_e611: f64 = (eq20_e609 * (nv3 - nv8));
        (eq20_e611, eq20_e609, (-eq20_e609),)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        let eq20_value: f64 = eq20_e613;
        stamper.stamp_current_node2_local(
            Some(3),
            Some(8),
            multiplicity * (eq20_value),
            3,
            multiplicity * (eq20_e613_d_n3),
            8,
            multiplicity * (eq20_e613_d_n8),
        );
        let (eq22_e628,) = {
    if (locals.var_guard1243 == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq22_value: f64 = eq22_e628;
        stamper.stamp_potential_const_local(
            3,
            eq22_value,
        );
        let eq23_e631: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, locals.var_qb);
        let eq23_e633: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 1, locals.var_qssub);
        let eq23_e634: f64 = (eq23_e631 + eq23_e633);
        let eq23_e634_d_n6: f64 = ((locals.var_qb_dn6 * ddt_scale) + (locals.var_qssub_dn6 * ddt_scale));
        let eq23_e634_d_n8: f64 = ((locals.var_qb_dn8 * ddt_scale) + (locals.var_qssub_dn8 * ddt_scale));
        let eq23_e636: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 2, locals.var_qbsif);
        let eq23_e637: f64 = (eq23_e634 + eq23_e636);
        let eq23_e637_d_n4: f64 = ((locals.var_qb_dn4 * ddt_scale) + (locals.var_qbsif_dn4 * ddt_scale));
        let eq23_e637_d_n6: f64 = (eq23_e634_d_n6 + (locals.var_qbsif_dn6 * ddt_scale));
        let eq23_e637_d_n7: f64 = ((locals.var_qb_dn7 * ddt_scale) + (locals.var_qbsif_dn7 * ddt_scale));
        let eq23_e637_d_n8: f64 = (eq23_e634_d_n8 + (locals.var_qbsif_dn8 * ddt_scale));
        let eq23_e637_d_n9: f64 = ((locals.var_qb_dn9 * ddt_scale) + (locals.var_qbsif_dn9 * ddt_scale));
        let eq23_e638: f64 = (p.p14 * eq23_e637);
        let eq23_e638_d_n4: f64 = (p.p14 * eq23_e637_d_n4);
        let eq23_e638_d_n6: f64 = (p.p14 * eq23_e637_d_n6);
        let eq23_e638_d_n7: f64 = (p.p14 * eq23_e637_d_n7);
        let eq23_e638_d_n8: f64 = (p.p14 * eq23_e637_d_n8);
        let eq23_e638_d_n9: f64 = (p.p14 * eq23_e637_d_n9);
        let eq23_value: f64 = eq23_e638;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(8),
            Some(6),
            multiplicity * (eq23_value),
            [4, 6, 7, 8, 9],
            [multiplicity * (eq23_e638_d_n4), multiplicity * (eq23_e638_d_n6), multiplicity * (eq23_e638_d_n7), multiplicity * (eq23_e638_d_n8), multiplicity * (eq23_e638_d_n9)],
            [],
            [],
            1.0,
        );
        let eq24_e641: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 3, locals.var_qgde);
        let eq24_e643: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 4, locals.var_qovd);
        let eq24_e644: f64 = (eq24_e641 + eq24_e643);
        let eq24_e644_d_n4: f64 = ((locals.var_qgde_dn4 * ddt_scale) + (locals.var_qovd_dn4 * ddt_scale));
        let eq24_e644_d_n6: f64 = ((locals.var_qgde_dn6 * ddt_scale) + (locals.var_qovd_dn6 * ddt_scale));
        let eq24_e644_d_n7: f64 = ((locals.var_qgde_dn7 * ddt_scale) + (locals.var_qovd_dn7 * ddt_scale));
        let eq24_e644_d_n8: f64 = ((locals.var_qgde_dn8 * ddt_scale) + (locals.var_qovd_dn8 * ddt_scale));
        let eq24_e644_d_n9: f64 = ((locals.var_qgde_dn9 * ddt_scale) + (locals.var_qovd_dn9 * ddt_scale));
        let eq24_e646: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 5, locals.var_qgdif);
        let eq24_e647: f64 = (eq24_e644 + eq24_e646);
        let eq24_e647_d_n4: f64 = (eq24_e644_d_n4 + (locals.var_qgdif_dn4 * ddt_scale));
        let eq24_e647_d_n6: f64 = (eq24_e644_d_n6 + (locals.var_qgdif_dn6 * ddt_scale));
        let eq24_e647_d_n7: f64 = (eq24_e644_d_n7 + (locals.var_qgdif_dn7 * ddt_scale));
        let eq24_e647_d_n8: f64 = (eq24_e644_d_n8 + (locals.var_qgdif_dn8 * ddt_scale));
        let eq24_e647_d_n9: f64 = (eq24_e644_d_n9 + (locals.var_qgdif_dn9 * ddt_scale));
        let eq24_e648: f64 = (p.p14 * eq24_e647);
        let eq24_e648_d_n4: f64 = (p.p14 * eq24_e647_d_n4);
        let eq24_e648_d_n6: f64 = (p.p14 * eq24_e647_d_n6);
        let eq24_e648_d_n7: f64 = (p.p14 * eq24_e647_d_n7);
        let eq24_e648_d_n8: f64 = (p.p14 * eq24_e647_d_n8);
        let eq24_e648_d_n9: f64 = (p.p14 * eq24_e647_d_n9);
        let eq24_value: f64 = eq24_e648;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(9),
            Some(7),
            multiplicity * (eq24_value),
            [4, 6, 7, 8, 9],
            [multiplicity * (eq24_e648_d_n4), multiplicity * (eq24_e648_d_n6), multiplicity * (eq24_e648_d_n7), multiplicity * (eq24_e648_d_n8), multiplicity * (eq24_e648_d_n9)],
            [],
            [],
            1.0,
        );
        let eq25_e651: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 6, locals.var_qdsub);
        let eq25_e653: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 7, locals.var_qbdif);
        let eq25_e654: f64 = (eq25_e651 + eq25_e653);
        let eq25_e654_d_n6: f64 = ((locals.var_qdsub_dn6 * ddt_scale) + (locals.var_qbdif_dn6 * ddt_scale));
        let eq25_e654_d_n7: f64 = ((locals.var_qdsub_dn7 * ddt_scale) + (locals.var_qbdif_dn7 * ddt_scale));
        let eq25_e654_d_n8: f64 = ((locals.var_qdsub_dn8 * ddt_scale) + (locals.var_qbdif_dn8 * ddt_scale));
        let eq25_e655: f64 = (p.p14 * eq25_e654);
        let eq25_e655_d_n4: f64 = (p.p14 * (locals.var_qbdif_dn4 * ddt_scale));
        let eq25_e655_d_n6: f64 = (p.p14 * eq25_e654_d_n6);
        let eq25_e655_d_n7: f64 = (p.p14 * eq25_e654_d_n7);
        let eq25_e655_d_n8: f64 = (p.p14 * eq25_e654_d_n8);
        let eq25_e655_d_n9: f64 = (p.p14 * (locals.var_qbdif_dn9 * ddt_scale));
        let eq25_value: f64 = eq25_e655;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(8),
            Some(7),
            multiplicity * (eq25_value),
            [4, 6, 7, 8, 9],
            [multiplicity * (eq25_e655_d_n4), multiplicity * (eq25_e655_d_n6), multiplicity * (eq25_e655_d_n7), multiplicity * (eq25_e655_d_n8), multiplicity * (eq25_e655_d_n9)],
            [],
            [],
            1.0,
        );
        let eq26_e658: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 8, locals.var_qgbe);
        let eq26_e659: f64 = (p.p14 * eq26_e658);
        let eq26_e659_d_n4: f64 = (p.p14 * (locals.var_qgbe_dn4 * ddt_scale));
        let eq26_e659_d_n6: f64 = (p.p14 * (locals.var_qgbe_dn6 * ddt_scale));
        let eq26_e659_d_n7: f64 = (p.p14 * (locals.var_qgbe_dn7 * ddt_scale));
        let eq26_e659_d_n8: f64 = (p.p14 * (locals.var_qgbe_dn8 * ddt_scale));
        let eq26_e659_d_n9: f64 = (p.p14 * (locals.var_qgbe_dn9 * ddt_scale));
        let eq26_value: f64 = eq26_e659;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(9),
            Some(8),
            multiplicity * (eq26_value),
            [4, 6, 7, 8, 9],
            [multiplicity * (eq26_e659_d_n4), multiplicity * (eq26_e659_d_n6), multiplicity * (eq26_e659_d_n7), multiplicity * (eq26_e659_d_n8), multiplicity * (eq26_e659_d_n9)],
            [],
            [],
            1.0,
        );
        let eq27_e662: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 9, locals.var_qg);
        let eq27_e664: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 10, locals.var_qgse);
        let eq27_e665: f64 = (eq27_e662 + eq27_e664);
        let eq27_e665_d_n4: f64 = ((locals.var_qg_dn4 * ddt_scale) + (locals.var_qgse_dn4 * ddt_scale));
        let eq27_e665_d_n6: f64 = ((locals.var_qg_dn6 * ddt_scale) + (locals.var_qgse_dn6 * ddt_scale));
        let eq27_e665_d_n7: f64 = ((locals.var_qg_dn7 * ddt_scale) + (locals.var_qgse_dn7 * ddt_scale));
        let eq27_e665_d_n8: f64 = ((locals.var_qg_dn8 * ddt_scale) + (locals.var_qgse_dn8 * ddt_scale));
        let eq27_e665_d_n9: f64 = ((locals.var_qg_dn9 * ddt_scale) + (locals.var_qgse_dn9 * ddt_scale));
        let eq27_e667: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 11, locals.var_qovs);
        let eq27_e668: f64 = (eq27_e665 + eq27_e667);
        let eq27_e668_d_n4: f64 = (eq27_e665_d_n4 + (locals.var_qovs_dn4 * ddt_scale));
        let eq27_e668_d_n6: f64 = (eq27_e665_d_n6 + (locals.var_qovs_dn6 * ddt_scale));
        let eq27_e668_d_n7: f64 = (eq27_e665_d_n7 + (locals.var_qovs_dn7 * ddt_scale));
        let eq27_e668_d_n8: f64 = (eq27_e665_d_n8 + (locals.var_qovs_dn8 * ddt_scale));
        let eq27_e668_d_n9: f64 = (eq27_e665_d_n9 + (locals.var_qovs_dn9 * ddt_scale));
        let eq27_e670: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 12, locals.var_qgsif);
        let eq27_e671: f64 = (eq27_e668 + eq27_e670);
        let eq27_e671_d_n4: f64 = (eq27_e668_d_n4 + (locals.var_qgsif_dn4 * ddt_scale));
        let eq27_e671_d_n6: f64 = (eq27_e668_d_n6 + (locals.var_qgsif_dn6 * ddt_scale));
        let eq27_e671_d_n7: f64 = (eq27_e668_d_n7 + (locals.var_qgsif_dn7 * ddt_scale));
        let eq27_e671_d_n8: f64 = (eq27_e668_d_n8 + (locals.var_qgsif_dn8 * ddt_scale));
        let eq27_e671_d_n9: f64 = (eq27_e668_d_n9 + (locals.var_qgsif_dn9 * ddt_scale));
        let eq27_e672: f64 = (p.p14 * eq27_e671);
        let eq27_e672_d_n4: f64 = (p.p14 * eq27_e671_d_n4);
        let eq27_e672_d_n6: f64 = (p.p14 * eq27_e671_d_n6);
        let eq27_e672_d_n7: f64 = (p.p14 * eq27_e671_d_n7);
        let eq27_e672_d_n8: f64 = (p.p14 * eq27_e671_d_n8);
        let eq27_e672_d_n9: f64 = (p.p14 * eq27_e671_d_n9);
        let eq27_value: f64 = eq27_e672;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(9),
            Some(6),
            multiplicity * (eq27_value),
            [4, 6, 7, 8, 9],
            [multiplicity * (eq27_e672_d_n4), multiplicity * (eq27_e672_d_n6), multiplicity * (eq27_e672_d_n7), multiplicity * (eq27_e672_d_n8), multiplicity * (eq27_e672_d_n9)],
            [],
            [],
            1.0,
        );
        let eq28_e675: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 13, locals.var_qd);
        let eq28_e677: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 14, locals.var_qdse);
        let eq28_e678: f64 = (eq28_e675 + eq28_e677);
        let eq28_e678_d_n6: f64 = ((locals.var_qd_dn6 * ddt_scale) + (locals.var_qdse_dn6 * ddt_scale));
        let eq28_e678_d_n7: f64 = ((locals.var_qd_dn7 * ddt_scale) + (locals.var_qdse_dn7 * ddt_scale));
        let eq28_e679: f64 = (p.p14 * eq28_e678);
        let eq28_e679_d_n4: f64 = (p.p14 * (locals.var_qd_dn4 * ddt_scale));
        let eq28_e679_d_n6: f64 = (p.p14 * eq28_e678_d_n6);
        let eq28_e679_d_n7: f64 = (p.p14 * eq28_e678_d_n7);
        let eq28_e679_d_n8: f64 = (p.p14 * (locals.var_qd_dn8 * ddt_scale));
        let eq28_e679_d_n9: f64 = (p.p14 * (locals.var_qd_dn9 * ddt_scale));
        let eq28_value: f64 = eq28_e679;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(7),
            Some(6),
            multiplicity * (eq28_value),
            [4, 6, 7, 8, 9],
            [multiplicity * (eq28_e679_d_n4), multiplicity * (eq28_e679_d_n6), multiplicity * (eq28_e679_d_n7), multiplicity * (eq28_e679_d_n8), multiplicity * (eq28_e679_d_n9)],
            [],
            [],
            1.0,
        );
        let eq29_e681: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 15, locals.var_qth);
        let eq29_value: f64 = eq29_e681;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(4),
            None,
            multiplicity * (eq29_value),
            [4, 6, 7, 8, 9],
            [multiplicity * ((locals.var_qth_dn4 * ddt_scale)), multiplicity * ((locals.var_qth_dn6 * ddt_scale)), multiplicity * ((locals.var_qth_dn7 * ddt_scale)), multiplicity * ((locals.var_qth_dn8 * ddt_scale)), multiplicity * ((locals.var_qth_dn9 * ddt_scale))],
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
        let eq31_e687: f64 = (locals.var_gsig * (nv5 - 0.0));
        let eq31_e687_d_n4: f64 = (locals.var_gsig_dn4 * (nv5 - 0.0));
        let eq31_e687_d_n6: f64 = (locals.var_gsig_dn6 * (nv5 - 0.0));
        let eq31_e687_d_n7: f64 = (locals.var_gsig_dn7 * (nv5 - 0.0));
        let eq31_e687_d_n8: f64 = (locals.var_gsig_dn8 * (nv5 - 0.0));
        let eq31_e687_d_n9: f64 = (locals.var_gsig_dn9 * (nv5 - 0.0));
        let eq31_value: f64 = eq31_e687;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(5),
            None,
            multiplicity * (eq31_value),
            [4, 5, 6, 7, 8, 9],
            [multiplicity * (eq31_e687_d_n4), multiplicity * (locals.var_gsig), multiplicity * (eq31_e687_d_n6), multiplicity * (eq31_e687_d_n7), multiplicity * (eq31_e687_d_n8), multiplicity * (eq31_e687_d_n9)],
            [],
            [],
            1.0,
        );
        let eq32_e690: f64 = (locals.var_cgeff * (nv5 - 0.0));
        let eq32_e690_d_n4: f64 = (locals.var_cgeff_dn4 * (nv5 - 0.0));
        let eq32_e690_d_n6: f64 = (locals.var_cgeff_dn6 * (nv5 - 0.0));
        let eq32_e690_d_n7: f64 = (locals.var_cgeff_dn7 * (nv5 - 0.0));
        let eq32_e690_d_n8: f64 = (locals.var_cgeff_dn8 * (nv5 - 0.0));
        let eq32_e690_d_n9: f64 = (locals.var_cgeff_dn9 * (nv5 - 0.0));
        let eq32_e691: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 16, eq32_e690);
        let eq32_value: f64 = eq32_e691;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(5),
            None,
            multiplicity * (eq32_value),
            [4, 5, 6, 7, 8, 9],
            [multiplicity * ((eq32_e690_d_n4 * ddt_scale)), multiplicity * ((locals.var_cgeff * ddt_scale)), multiplicity * ((eq32_e690_d_n6 * ddt_scale)), multiplicity * ((eq32_e690_d_n7 * ddt_scale)), multiplicity * ((eq32_e690_d_n8 * ddt_scale)), multiplicity * ((eq32_e690_d_n9 * ddt_scale))],
            [],
            [],
            1.0,
        );
        let eq33_e693: f64 = (-locals.var_csgeff);
        let eq33_e695: f64 = (eq33_e693 * (nv5 - 0.0));
        let eq33_e695_d_n4: f64 = ((-locals.var_csgeff_dn4) * (nv5 - 0.0));
        let eq33_e695_d_n6: f64 = ((-locals.var_csgeff_dn6) * (nv5 - 0.0));
        let eq33_e695_d_n7: f64 = ((-locals.var_csgeff_dn7) * (nv5 - 0.0));
        let eq33_e695_d_n8: f64 = ((-locals.var_csgeff_dn8) * (nv5 - 0.0));
        let eq33_e695_d_n9: f64 = ((-locals.var_csgeff_dn9) * (nv5 - 0.0));
        let eq33_e696: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 17, eq33_e695);
        let eq33_value: f64 = eq33_e696;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(9),
            Some(6),
            multiplicity * (eq33_value),
            [4, 5, 6, 7, 8, 9],
            [multiplicity * ((eq33_e695_d_n4 * ddt_scale)), multiplicity * ((eq33_e693 * ddt_scale)), multiplicity * ((eq33_e695_d_n6 * ddt_scale)), multiplicity * ((eq33_e695_d_n7 * ddt_scale)), multiplicity * ((eq33_e695_d_n8 * ddt_scale)), multiplicity * ((eq33_e695_d_n9 * ddt_scale))],
            [],
            [],
            1.0,
        );
        let eq34_e698: f64 = (-locals.var_cdgeff);
        let eq34_e700: f64 = (eq34_e698 * (nv5 - 0.0));
        let eq34_e700_d_n4: f64 = ((-locals.var_cdgeff_dn4) * (nv5 - 0.0));
        let eq34_e700_d_n6: f64 = ((-locals.var_cdgeff_dn6) * (nv5 - 0.0));
        let eq34_e700_d_n7: f64 = ((-locals.var_cdgeff_dn7) * (nv5 - 0.0));
        let eq34_e700_d_n8: f64 = ((-locals.var_cdgeff_dn8) * (nv5 - 0.0));
        let eq34_e700_d_n9: f64 = ((-locals.var_cdgeff_dn9) * (nv5 - 0.0));
        let eq34_e701: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 18, eq34_e700);
        let eq34_value: f64 = eq34_e701;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(9),
            Some(7),
            multiplicity * (eq34_value),
            [4, 5, 6, 7, 8, 9],
            [multiplicity * ((eq34_e700_d_n4 * ddt_scale)), multiplicity * ((eq34_e698 * ddt_scale)), multiplicity * ((eq34_e700_d_n6 * ddt_scale)), multiplicity * ((eq34_e700_d_n7 * ddt_scale)), multiplicity * ((eq34_e700_d_n8 * ddt_scale)), multiplicity * ((eq34_e700_d_n9 * ddt_scale))],
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
        let eq23_e631_q: f64 = locals.var_qb;
        let eq23_e633_q: f64 = locals.var_qssub;
        let eq23_e634: f64 = (locals.var_qb + locals.var_qssub);
        let eq23_e634_d_n6: f64 = (locals.var_qb_dn6 + locals.var_qssub_dn6);
        let eq23_e634_d_n8: f64 = (locals.var_qb_dn8 + locals.var_qssub_dn8);
        let eq23_e634_q: f64 = (eq23_e631_q + eq23_e633_q);
        let eq23_e636_q: f64 = locals.var_qbsif;
        let eq23_e637: f64 = (eq23_e634 + locals.var_qbsif);
        let eq23_e637_d_n4: f64 = (locals.var_qb_dn4 + locals.var_qbsif_dn4);
        let eq23_e637_d_n6: f64 = (eq23_e634_d_n6 + locals.var_qbsif_dn6);
        let eq23_e637_d_n7: f64 = (locals.var_qb_dn7 + locals.var_qbsif_dn7);
        let eq23_e637_d_n8: f64 = (eq23_e634_d_n8 + locals.var_qbsif_dn8);
        let eq23_e637_d_n9: f64 = (locals.var_qb_dn9 + locals.var_qbsif_dn9);
        let eq23_e637_q: f64 = (eq23_e634_q + eq23_e636_q);
        let eq23_e638: f64 = (p.p14 * eq23_e637);
        let eq23_e638_d_n4: f64 = (p.p14 * eq23_e637_d_n4);
        let eq23_e638_d_n6: f64 = (p.p14 * eq23_e637_d_n6);
        let eq23_e638_d_n7: f64 = (p.p14 * eq23_e637_d_n7);
        let eq23_e638_d_n8: f64 = (p.p14 * eq23_e637_d_n8);
        let eq23_e638_d_n9: f64 = (p.p14 * eq23_e637_d_n9);
        let eq23_e638_q: f64 = (p.p14 * eq23_e637_q);
        let eq23_reactive_node_derivatives: [f64; 10] = [0.0, 0.0, 0.0, 0.0, eq23_e638_d_n4, 0.0, eq23_e638_d_n6, eq23_e638_d_n7, eq23_e638_d_n8, eq23_e638_d_n9];
        let eq23_reactive_branch_derivatives: [f64; 4] = [0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[6]),
            nodes,
            &eq23_reactive_node_derivatives,
            branches,
            &eq23_reactive_branch_derivatives,
            multiplicity,
        );
        let eq24_e641_q: f64 = locals.var_qgde;
        let eq24_e643_q: f64 = locals.var_qovd;
        let eq24_e644: f64 = (locals.var_qgde + locals.var_qovd);
        let eq24_e644_d_n4: f64 = (locals.var_qgde_dn4 + locals.var_qovd_dn4);
        let eq24_e644_d_n6: f64 = (locals.var_qgde_dn6 + locals.var_qovd_dn6);
        let eq24_e644_d_n7: f64 = (locals.var_qgde_dn7 + locals.var_qovd_dn7);
        let eq24_e644_d_n8: f64 = (locals.var_qgde_dn8 + locals.var_qovd_dn8);
        let eq24_e644_d_n9: f64 = (locals.var_qgde_dn9 + locals.var_qovd_dn9);
        let eq24_e644_q: f64 = (eq24_e641_q + eq24_e643_q);
        let eq24_e646_q: f64 = locals.var_qgdif;
        let eq24_e647: f64 = (eq24_e644 + locals.var_qgdif);
        let eq24_e647_d_n4: f64 = (eq24_e644_d_n4 + locals.var_qgdif_dn4);
        let eq24_e647_d_n6: f64 = (eq24_e644_d_n6 + locals.var_qgdif_dn6);
        let eq24_e647_d_n7: f64 = (eq24_e644_d_n7 + locals.var_qgdif_dn7);
        let eq24_e647_d_n8: f64 = (eq24_e644_d_n8 + locals.var_qgdif_dn8);
        let eq24_e647_d_n9: f64 = (eq24_e644_d_n9 + locals.var_qgdif_dn9);
        let eq24_e647_q: f64 = (eq24_e644_q + eq24_e646_q);
        let eq24_e648: f64 = (p.p14 * eq24_e647);
        let eq24_e648_d_n4: f64 = (p.p14 * eq24_e647_d_n4);
        let eq24_e648_d_n6: f64 = (p.p14 * eq24_e647_d_n6);
        let eq24_e648_d_n7: f64 = (p.p14 * eq24_e647_d_n7);
        let eq24_e648_d_n8: f64 = (p.p14 * eq24_e647_d_n8);
        let eq24_e648_d_n9: f64 = (p.p14 * eq24_e647_d_n9);
        let eq24_e648_q: f64 = (p.p14 * eq24_e647_q);
        let eq24_reactive_node_derivatives: [f64; 10] = [0.0, 0.0, 0.0, 0.0, eq24_e648_d_n4, 0.0, eq24_e648_d_n6, eq24_e648_d_n7, eq24_e648_d_n8, eq24_e648_d_n9];
        let eq24_reactive_branch_derivatives: [f64; 4] = [0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[7]),
            nodes,
            &eq24_reactive_node_derivatives,
            branches,
            &eq24_reactive_branch_derivatives,
            multiplicity,
        );
        let eq25_e651_q: f64 = locals.var_qdsub;
        let eq25_e653_q: f64 = locals.var_qbdif;
        let eq25_e654: f64 = (locals.var_qdsub + locals.var_qbdif);
        let eq25_e654_d_n6: f64 = (locals.var_qdsub_dn6 + locals.var_qbdif_dn6);
        let eq25_e654_d_n7: f64 = (locals.var_qdsub_dn7 + locals.var_qbdif_dn7);
        let eq25_e654_d_n8: f64 = (locals.var_qdsub_dn8 + locals.var_qbdif_dn8);
        let eq25_e654_q: f64 = (eq25_e651_q + eq25_e653_q);
        let eq25_e655: f64 = (p.p14 * eq25_e654);
        let eq25_e655_d_n4: f64 = (p.p14 * locals.var_qbdif_dn4);
        let eq25_e655_d_n6: f64 = (p.p14 * eq25_e654_d_n6);
        let eq25_e655_d_n7: f64 = (p.p14 * eq25_e654_d_n7);
        let eq25_e655_d_n8: f64 = (p.p14 * eq25_e654_d_n8);
        let eq25_e655_d_n9: f64 = (p.p14 * locals.var_qbdif_dn9);
        let eq25_e655_q: f64 = (p.p14 * eq25_e654_q);
        let eq25_reactive_node_derivatives: [f64; 10] = [0.0, 0.0, 0.0, 0.0, eq25_e655_d_n4, 0.0, eq25_e655_d_n6, eq25_e655_d_n7, eq25_e655_d_n8, eq25_e655_d_n9];
        let eq25_reactive_branch_derivatives: [f64; 4] = [0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[7]),
            nodes,
            &eq25_reactive_node_derivatives,
            branches,
            &eq25_reactive_branch_derivatives,
            multiplicity,
        );
        let eq26_e658_q: f64 = locals.var_qgbe;
        let eq26_e659: f64 = (p.p14 * locals.var_qgbe);
        let eq26_e659_d_n4: f64 = (p.p14 * locals.var_qgbe_dn4);
        let eq26_e659_d_n6: f64 = (p.p14 * locals.var_qgbe_dn6);
        let eq26_e659_d_n7: f64 = (p.p14 * locals.var_qgbe_dn7);
        let eq26_e659_d_n8: f64 = (p.p14 * locals.var_qgbe_dn8);
        let eq26_e659_d_n9: f64 = (p.p14 * locals.var_qgbe_dn9);
        let eq26_e659_q: f64 = (p.p14 * eq26_e658_q);
        let eq26_reactive_node_derivatives: [f64; 10] = [0.0, 0.0, 0.0, 0.0, eq26_e659_d_n4, 0.0, eq26_e659_d_n6, eq26_e659_d_n7, eq26_e659_d_n8, eq26_e659_d_n9];
        let eq26_reactive_branch_derivatives: [f64; 4] = [0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[8]),
            nodes,
            &eq26_reactive_node_derivatives,
            branches,
            &eq26_reactive_branch_derivatives,
            multiplicity,
        );
        let eq27_e662_q: f64 = locals.var_qg;
        let eq27_e664_q: f64 = locals.var_qgse;
        let eq27_e665: f64 = (locals.var_qg + locals.var_qgse);
        let eq27_e665_d_n4: f64 = (locals.var_qg_dn4 + locals.var_qgse_dn4);
        let eq27_e665_d_n6: f64 = (locals.var_qg_dn6 + locals.var_qgse_dn6);
        let eq27_e665_d_n7: f64 = (locals.var_qg_dn7 + locals.var_qgse_dn7);
        let eq27_e665_d_n8: f64 = (locals.var_qg_dn8 + locals.var_qgse_dn8);
        let eq27_e665_d_n9: f64 = (locals.var_qg_dn9 + locals.var_qgse_dn9);
        let eq27_e665_q: f64 = (eq27_e662_q + eq27_e664_q);
        let eq27_e667_q: f64 = locals.var_qovs;
        let eq27_e668: f64 = (eq27_e665 + locals.var_qovs);
        let eq27_e668_d_n4: f64 = (eq27_e665_d_n4 + locals.var_qovs_dn4);
        let eq27_e668_d_n6: f64 = (eq27_e665_d_n6 + locals.var_qovs_dn6);
        let eq27_e668_d_n7: f64 = (eq27_e665_d_n7 + locals.var_qovs_dn7);
        let eq27_e668_d_n8: f64 = (eq27_e665_d_n8 + locals.var_qovs_dn8);
        let eq27_e668_d_n9: f64 = (eq27_e665_d_n9 + locals.var_qovs_dn9);
        let eq27_e668_q: f64 = (eq27_e665_q + eq27_e667_q);
        let eq27_e670_q: f64 = locals.var_qgsif;
        let eq27_e671: f64 = (eq27_e668 + locals.var_qgsif);
        let eq27_e671_d_n4: f64 = (eq27_e668_d_n4 + locals.var_qgsif_dn4);
        let eq27_e671_d_n6: f64 = (eq27_e668_d_n6 + locals.var_qgsif_dn6);
        let eq27_e671_d_n7: f64 = (eq27_e668_d_n7 + locals.var_qgsif_dn7);
        let eq27_e671_d_n8: f64 = (eq27_e668_d_n8 + locals.var_qgsif_dn8);
        let eq27_e671_d_n9: f64 = (eq27_e668_d_n9 + locals.var_qgsif_dn9);
        let eq27_e671_q: f64 = (eq27_e668_q + eq27_e670_q);
        let eq27_e672: f64 = (p.p14 * eq27_e671);
        let eq27_e672_d_n4: f64 = (p.p14 * eq27_e671_d_n4);
        let eq27_e672_d_n6: f64 = (p.p14 * eq27_e671_d_n6);
        let eq27_e672_d_n7: f64 = (p.p14 * eq27_e671_d_n7);
        let eq27_e672_d_n8: f64 = (p.p14 * eq27_e671_d_n8);
        let eq27_e672_d_n9: f64 = (p.p14 * eq27_e671_d_n9);
        let eq27_e672_q: f64 = (p.p14 * eq27_e671_q);
        let eq27_reactive_node_derivatives: [f64; 10] = [0.0, 0.0, 0.0, 0.0, eq27_e672_d_n4, 0.0, eq27_e672_d_n6, eq27_e672_d_n7, eq27_e672_d_n8, eq27_e672_d_n9];
        let eq27_reactive_branch_derivatives: [f64; 4] = [0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[6]),
            nodes,
            &eq27_reactive_node_derivatives,
            branches,
            &eq27_reactive_branch_derivatives,
            multiplicity,
        );
        let eq28_e675_q: f64 = locals.var_qd;
        let eq28_e677_q: f64 = locals.var_qdse;
        let eq28_e678: f64 = (locals.var_qd + locals.var_qdse);
        let eq28_e678_d_n6: f64 = (locals.var_qd_dn6 + locals.var_qdse_dn6);
        let eq28_e678_d_n7: f64 = (locals.var_qd_dn7 + locals.var_qdse_dn7);
        let eq28_e678_q: f64 = (eq28_e675_q + eq28_e677_q);
        let eq28_e679: f64 = (p.p14 * eq28_e678);
        let eq28_e679_d_n4: f64 = (p.p14 * locals.var_qd_dn4);
        let eq28_e679_d_n6: f64 = (p.p14 * eq28_e678_d_n6);
        let eq28_e679_d_n7: f64 = (p.p14 * eq28_e678_d_n7);
        let eq28_e679_d_n8: f64 = (p.p14 * locals.var_qd_dn8);
        let eq28_e679_d_n9: f64 = (p.p14 * locals.var_qd_dn9);
        let eq28_e679_q: f64 = (p.p14 * eq28_e678_q);
        let eq28_reactive_node_derivatives: [f64; 10] = [0.0, 0.0, 0.0, 0.0, eq28_e679_d_n4, 0.0, eq28_e679_d_n6, eq28_e679_d_n7, eq28_e679_d_n8, eq28_e679_d_n9];
        let eq28_reactive_branch_derivatives: [f64; 4] = [0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[6]),
            nodes,
            &eq28_reactive_node_derivatives,
            branches,
            &eq28_reactive_branch_derivatives,
            multiplicity,
        );
        let eq29_e681_q: f64 = locals.var_qth;
        let eq29_reactive_node_derivatives: [f64; 10] = [0.0, 0.0, 0.0, 0.0, locals.var_qth_dn4, 0.0, locals.var_qth_dn6, locals.var_qth_dn7, locals.var_qth_dn8, locals.var_qth_dn9];
        let eq29_reactive_branch_derivatives: [f64; 4] = [0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[4]),
            None,
            nodes,
            &eq29_reactive_node_derivatives,
            branches,
            &eq29_reactive_branch_derivatives,
            multiplicity,
        );
        let eq32_e690: f64 = (locals.var_cgeff * (nv5 - 0.0));
        let eq32_e690_d_n4: f64 = (locals.var_cgeff_dn4 * (nv5 - 0.0));
        let eq32_e690_d_n6: f64 = (locals.var_cgeff_dn6 * (nv5 - 0.0));
        let eq32_e690_d_n7: f64 = (locals.var_cgeff_dn7 * (nv5 - 0.0));
        let eq32_e690_d_n8: f64 = (locals.var_cgeff_dn8 * (nv5 - 0.0));
        let eq32_e690_d_n9: f64 = (locals.var_cgeff_dn9 * (nv5 - 0.0));
        let eq32_e691_q: f64 = eq32_e690;
        let eq32_reactive_node_derivatives: [f64; 10] = [0.0, 0.0, 0.0, 0.0, eq32_e690_d_n4, locals.var_cgeff, eq32_e690_d_n6, eq32_e690_d_n7, eq32_e690_d_n8, eq32_e690_d_n9];
        let eq32_reactive_branch_derivatives: [f64; 4] = [0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            None,
            nodes,
            &eq32_reactive_node_derivatives,
            branches,
            &eq32_reactive_branch_derivatives,
            multiplicity,
        );
        let eq33_e693: f64 = (-locals.var_csgeff);
        let eq33_e695: f64 = (eq33_e693 * (nv5 - 0.0));
        let eq33_e695_d_n4: f64 = ((-locals.var_csgeff_dn4) * (nv5 - 0.0));
        let eq33_e695_d_n6: f64 = ((-locals.var_csgeff_dn6) * (nv5 - 0.0));
        let eq33_e695_d_n7: f64 = ((-locals.var_csgeff_dn7) * (nv5 - 0.0));
        let eq33_e695_d_n8: f64 = ((-locals.var_csgeff_dn8) * (nv5 - 0.0));
        let eq33_e695_d_n9: f64 = ((-locals.var_csgeff_dn9) * (nv5 - 0.0));
        let eq33_e696_q: f64 = eq33_e695;
        let eq33_reactive_node_derivatives: [f64; 10] = [0.0, 0.0, 0.0, 0.0, eq33_e695_d_n4, eq33_e693, eq33_e695_d_n6, eq33_e695_d_n7, eq33_e695_d_n8, eq33_e695_d_n9];
        let eq33_reactive_branch_derivatives: [f64; 4] = [0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[6]),
            nodes,
            &eq33_reactive_node_derivatives,
            branches,
            &eq33_reactive_branch_derivatives,
            multiplicity,
        );
        let eq34_e698: f64 = (-locals.var_cdgeff);
        let eq34_e700: f64 = (eq34_e698 * (nv5 - 0.0));
        let eq34_e700_d_n4: f64 = ((-locals.var_cdgeff_dn4) * (nv5 - 0.0));
        let eq34_e700_d_n6: f64 = ((-locals.var_cdgeff_dn6) * (nv5 - 0.0));
        let eq34_e700_d_n7: f64 = ((-locals.var_cdgeff_dn7) * (nv5 - 0.0));
        let eq34_e700_d_n8: f64 = ((-locals.var_cdgeff_dn8) * (nv5 - 0.0));
        let eq34_e700_d_n9: f64 = ((-locals.var_cdgeff_dn9) * (nv5 - 0.0));
        let eq34_e701_q: f64 = eq34_e700;
        let eq34_reactive_node_derivatives: [f64; 10] = [0.0, 0.0, 0.0, 0.0, eq34_e700_d_n4, eq34_e698, eq34_e700_d_n6, eq34_e700_d_n7, eq34_e700_d_n8, eq34_e700_d_n9];
        let eq34_reactive_branch_derivatives: [f64; 4] = [0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[7]),
            nodes,
            &eq34_reactive_node_derivatives,
            branches,
            &eq34_reactive_branch_derivatives,
            multiplicity,
        );
    }
}
