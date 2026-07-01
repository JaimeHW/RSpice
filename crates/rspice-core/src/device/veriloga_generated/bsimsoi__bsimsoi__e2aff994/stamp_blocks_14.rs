#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_13(
        p: &Parameters,
        var_dmcgeff: f64,
        var_dmcieff: f64,
        var_guard120: f64,
        var_guard132: f64,
        var_guard133: f64,
        var_guard78: f64,
        var_guard79: f64,
        var_guard86: f64,
        var_guard87: f64,
        var_guard88: f64,
        var_guard89: f64,
        var_nuendd: f64,
        var_nuends: f64,
        var_weff: f64,
        var_guard134_slot: &mut f64,
        var_guard134_rv_slot: &mut f64,
        var_guard135_slot: &mut f64,
        var_guard135_rv_slot: &mut f64,
        var_guard137_slot: &mut f64,
        var_guard137_rv_slot: &mut f64,
        var_guard138_slot: &mut f64,
        var_guard138_rv_slot: &mut f64,
        var_guard139_slot: &mut f64,
        var_guard139_rv_slot: &mut f64,
        var_guard140_slot: &mut f64,
        var_guard140_rv_slot: &mut f64,
        var_guard142_slot: &mut f64,
        var_guard142_rv_slot: &mut f64,
        var_guard143_slot: &mut f64,
        var_guard143_rv_slot: &mut f64,
        var_guard144_slot: &mut f64,
        var_guard144_rv_slot: &mut f64,
        var_guard145_slot: &mut f64,
        var_guard145_rv_slot: &mut f64,
        var_guard146_slot: &mut f64,
        var_guard146_rv_slot: &mut f64,
        var_guard147_slot: &mut f64,
        var_guard147_rv_slot: &mut f64,
        var_guard149_slot: &mut f64,
        var_guard149_rv_slot: &mut f64,
        var_guard150_slot: &mut f64,
        var_guard150_rv_slot: &mut f64,
        var_guard151_slot: &mut f64,
        var_guard151_rv_slot: &mut f64,
        var_guard152_slot: &mut f64,
        var_guard152_rv_slot: &mut f64,
        var_guard154_slot: &mut f64,
        var_guard154_rv_slot: &mut f64,
        var_guard155_slot: &mut f64,
        var_guard155_rv_slot: &mut f64,
        var_guard156_slot: &mut f64,
        var_guard156_rv_slot: &mut f64,
        var_guard157_slot: &mut f64,
        var_guard157_rv_slot: &mut f64,
        var_guard158_slot: &mut f64,
        var_guard158_rv_slot: &mut f64,
        var_guard160_slot: &mut f64,
        var_guard160_rv_slot: &mut f64,
        var_guard161_slot: &mut f64,
        var_guard161_rv_slot: &mut f64,
        var_guard162_slot: &mut f64,
        var_guard162_rv_slot: &mut f64,
        var_guard163_slot: &mut f64,
        var_guard163_rv_slot: &mut f64,
        var_guard165_slot: &mut f64,
        var_guard165_rv_slot: &mut f64,
        var_guard166_slot: &mut f64,
        var_guard166_rv_slot: &mut f64,
        var_guard167_slot: &mut f64,
        var_guard167_rv_slot: &mut f64,
        var_guard168_slot: &mut f64,
        var_guard168_rv_slot: &mut f64,
        var_guard169_slot: &mut f64,
        var_guard169_rv_slot: &mut f64,
        var_guard170_slot: &mut f64,
        var_guard170_rv_slot: &mut f64,
        var_guard172_slot: &mut f64,
        var_guard172_rv_slot: &mut f64,
        var_rend_slot: &mut f64,
        var_rend_rv_slot: &mut f64,
    ) {
        let mut var_guard134: f64 = *var_guard134_slot;
        let mut var_guard134_rv: f64 = *var_guard134_rv_slot;
        let mut var_guard135: f64 = *var_guard135_slot;
        let mut var_guard135_rv: f64 = *var_guard135_rv_slot;
        let mut var_guard137: f64 = *var_guard137_slot;
        let mut var_guard137_rv: f64 = *var_guard137_rv_slot;
        let mut var_guard138: f64 = *var_guard138_slot;
        let mut var_guard138_rv: f64 = *var_guard138_rv_slot;
        let mut var_guard139: f64 = *var_guard139_slot;
        let mut var_guard139_rv: f64 = *var_guard139_rv_slot;
        let mut var_guard140: f64 = *var_guard140_slot;
        let mut var_guard140_rv: f64 = *var_guard140_rv_slot;
        let mut var_guard142: f64 = *var_guard142_slot;
        let mut var_guard142_rv: f64 = *var_guard142_rv_slot;
        let mut var_guard143: f64 = *var_guard143_slot;
        let mut var_guard143_rv: f64 = *var_guard143_rv_slot;
        let mut var_guard144: f64 = *var_guard144_slot;
        let mut var_guard144_rv: f64 = *var_guard144_rv_slot;
        let mut var_guard145: f64 = *var_guard145_slot;
        let mut var_guard145_rv: f64 = *var_guard145_rv_slot;
        let mut var_guard146: f64 = *var_guard146_slot;
        let mut var_guard146_rv: f64 = *var_guard146_rv_slot;
        let mut var_guard147: f64 = *var_guard147_slot;
        let mut var_guard147_rv: f64 = *var_guard147_rv_slot;
        let mut var_guard149: f64 = *var_guard149_slot;
        let mut var_guard149_rv: f64 = *var_guard149_rv_slot;
        let mut var_guard150: f64 = *var_guard150_slot;
        let mut var_guard150_rv: f64 = *var_guard150_rv_slot;
        let mut var_guard151: f64 = *var_guard151_slot;
        let mut var_guard151_rv: f64 = *var_guard151_rv_slot;
        let mut var_guard152: f64 = *var_guard152_slot;
        let mut var_guard152_rv: f64 = *var_guard152_rv_slot;
        let mut var_guard154: f64 = *var_guard154_slot;
        let mut var_guard154_rv: f64 = *var_guard154_rv_slot;
        let mut var_guard155: f64 = *var_guard155_slot;
        let mut var_guard155_rv: f64 = *var_guard155_rv_slot;
        let mut var_guard156: f64 = *var_guard156_slot;
        let mut var_guard156_rv: f64 = *var_guard156_rv_slot;
        let mut var_guard157: f64 = *var_guard157_slot;
        let mut var_guard157_rv: f64 = *var_guard157_rv_slot;
        let mut var_guard158: f64 = *var_guard158_slot;
        let mut var_guard158_rv: f64 = *var_guard158_rv_slot;
        let mut var_guard160: f64 = *var_guard160_slot;
        let mut var_guard160_rv: f64 = *var_guard160_rv_slot;
        let mut var_guard161: f64 = *var_guard161_slot;
        let mut var_guard161_rv: f64 = *var_guard161_rv_slot;
        let mut var_guard162: f64 = *var_guard162_slot;
        let mut var_guard162_rv: f64 = *var_guard162_rv_slot;
        let mut var_guard163: f64 = *var_guard163_slot;
        let mut var_guard163_rv: f64 = *var_guard163_rv_slot;
        let mut var_guard165: f64 = *var_guard165_slot;
        let mut var_guard165_rv: f64 = *var_guard165_rv_slot;
        let mut var_guard166: f64 = *var_guard166_slot;
        let mut var_guard166_rv: f64 = *var_guard166_rv_slot;
        let mut var_guard167: f64 = *var_guard167_slot;
        let mut var_guard167_rv: f64 = *var_guard167_rv_slot;
        let mut var_guard168: f64 = *var_guard168_slot;
        let mut var_guard168_rv: f64 = *var_guard168_rv_slot;
        let mut var_guard169: f64 = *var_guard169_slot;
        let mut var_guard169_rv: f64 = *var_guard169_rv_slot;
        let mut var_guard170: f64 = *var_guard170_slot;
        let mut var_guard170_rv: f64 = *var_guard170_rv_slot;
        let mut var_guard172: f64 = *var_guard172_slot;
        let mut var_guard172_rv: f64 = *var_guard172_rv_slot;
        let mut var_rend: f64 = *var_rend_slot;
        let mut var_rend_rv: f64 = *var_rend_rv_slot;

        let assign7210_e8604: f64 = if (((p.p9 == 3.0) || (p.p9 == 4.0)) || (p.p9 == 6.0)) { 1.0 } else { 0.0 };
        var_guard134 = assign7210_e8604;
        var_guard134_rv = 0.0;

        let assign7220_e8607: f64 = if var_nuendd == 0.0 { 1.0 } else { 0.0 };
        var_guard135 = assign7220_e8607;
        var_guard135_rv = 0.0;

        let (assign7230_e8628,) = {
    if (((((((var_guard78 == 0.0) && (var_guard79 != 0.0)) && ((var_guard87 != 0.0) && (var_guard86 == 0.0))) && (var_guard120 == 0.0)) && (var_guard132 != 0.0)) && (var_guard133 != 0.0)) && (var_guard135 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign7230_e8628;
        var_rend_rv = 0.0;

        let (assign7240_e8656,) = {
    if (((((((var_guard78 == 0.0) && (var_guard79 != 0.0)) && ((var_guard87 != 0.0) && (var_guard86 == 0.0))) && (var_guard120 == 0.0)) && (var_guard132 != 0.0)) && (var_guard133 != 0.0)) && (var_guard135 == 0.0)) {
        let assign7240_e8650: f64 = (p.p438 * var_dmcgeff);
        let assign7240_e8653: f64 = (var_weff * var_nuendd);
        let assign7240_e8654: f64 = (assign7240_e8650 / assign7240_e8653);
        (assign7240_e8654,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign7240_e8656;
        var_rend_rv = 0.0;

        let assign7260_e8666: f64 = if ((var_nuendd == 0.0) || (var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        var_guard137 = assign7260_e8666;
        var_guard137_rv = 0.0;

        let (assign7270_e8690,) = {
    if (((((((var_guard78 == 0.0) && (var_guard79 != 0.0)) && ((var_guard87 != 0.0) && (var_guard86 == 0.0))) && (var_guard120 == 0.0)) && (var_guard132 != 0.0)) && ((var_guard134 != 0.0) && (var_guard133 == 0.0))) && (var_guard137 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign7270_e8690;
        var_rend_rv = 0.0;

        let (assign7280_e8723,) = {
    if (((((((var_guard78 == 0.0) && (var_guard79 != 0.0)) && ((var_guard87 != 0.0) && (var_guard86 == 0.0))) && (var_guard120 == 0.0)) && (var_guard132 != 0.0)) && ((var_guard134 != 0.0) && (var_guard133 == 0.0))) && (var_guard137 == 0.0)) {
        let assign7280_e8715: f64 = (p.p438 * var_weff);
        let assign7280_e8718: f64 = (6.0 * var_nuendd);
        let assign7280_e8720: f64 = (assign7280_e8718 * var_dmcgeff);
        let assign7280_e8721: f64 = (assign7280_e8715 / assign7280_e8720);
        (assign7280_e8721,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign7280_e8723;
        var_rend_rv = 0.0;

        let (assign7290_e8745,) = {
    if ((((((var_guard78 == 0.0) && (var_guard79 != 0.0)) && ((var_guard87 != 0.0) && (var_guard86 == 0.0))) && (var_guard120 == 0.0)) && (var_guard132 != 0.0)) && (!((var_guard133 != 0.0) || (var_guard134 != 0.0)))) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign7290_e8745;
        var_rend_rv = 0.0;

        let assign7300_e8756: f64 = if (((p.p9 == 1.0) || (p.p9 == 3.0)) || (p.p9 == 7.0)) { 1.0 } else { 0.0 };
        var_guard138 = assign7300_e8756;
        var_guard138_rv = 0.0;

        let assign7310_e8767: f64 = if (((p.p9 == 2.0) || (p.p9 == 4.0)) || (p.p9 == 8.0)) { 1.0 } else { 0.0 };
        var_guard139 = assign7310_e8767;
        var_guard139_rv = 0.0;

        let assign7320_e8770: f64 = if var_nuendd == 0.0 { 1.0 } else { 0.0 };
        var_guard140 = assign7320_e8770;
        var_guard140_rv = 0.0;

        let (assign7330_e8792,) = {
    if (((((((var_guard78 == 0.0) && (var_guard79 != 0.0)) && ((var_guard87 != 0.0) && (var_guard86 == 0.0))) && (var_guard120 == 0.0)) && (var_guard132 == 0.0)) && (var_guard138 != 0.0)) && (var_guard140 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign7330_e8792;
        var_rend_rv = 0.0;

        let (assign7340_e8821,) = {
    if (((((((var_guard78 == 0.0) && (var_guard79 != 0.0)) && ((var_guard87 != 0.0) && (var_guard86 == 0.0))) && (var_guard120 == 0.0)) && (var_guard132 == 0.0)) && (var_guard138 != 0.0)) && (var_guard140 == 0.0)) {
        let assign7340_e8815: f64 = (p.p438 * var_dmcgeff);
        let assign7340_e8818: f64 = (var_weff * var_nuendd);
        let assign7340_e8819: f64 = (assign7340_e8815 / assign7340_e8818);
        (assign7340_e8819,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign7340_e8821;
        var_rend_rv = 0.0;

        let assign7360_e8831: f64 = if ((var_nuendd == 0.0) || (var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        var_guard142 = assign7360_e8831;
        var_guard142_rv = 0.0;

        let (assign7370_e8856,) = {
    if (((((((var_guard78 == 0.0) && (var_guard79 != 0.0)) && ((var_guard87 != 0.0) && (var_guard86 == 0.0))) && (var_guard120 == 0.0)) && (var_guard132 == 0.0)) && ((var_guard139 != 0.0) && (var_guard138 == 0.0))) && (var_guard142 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign7370_e8856;
        var_rend_rv = 0.0;

        let (assign7380_e8890,) = {
    if (((((((var_guard78 == 0.0) && (var_guard79 != 0.0)) && ((var_guard87 != 0.0) && (var_guard86 == 0.0))) && (var_guard120 == 0.0)) && (var_guard132 == 0.0)) && ((var_guard139 != 0.0) && (var_guard138 == 0.0))) && (var_guard142 == 0.0)) {
        let assign7380_e8882: f64 = (p.p438 * var_weff);
        let assign7380_e8885: f64 = (6.0 * var_nuendd);
        let assign7380_e8887: f64 = (assign7380_e8885 * var_dmcgeff);
        let assign7380_e8888: f64 = (assign7380_e8882 / assign7380_e8887);
        (assign7380_e8888,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign7380_e8890;
        var_rend_rv = 0.0;

        let (assign7390_e8913,) = {
    if ((((((var_guard78 == 0.0) && (var_guard79 != 0.0)) && ((var_guard87 != 0.0) && (var_guard86 == 0.0))) && (var_guard120 == 0.0)) && (var_guard132 == 0.0)) && (!((var_guard138 != 0.0) || (var_guard139 != 0.0)))) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign7390_e8913;
        var_rend_rv = 0.0;

        let assign7400_e8916: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        var_guard143 = assign7400_e8916;
        var_guard143_rv = 0.0;

        let assign7410_e8919: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        var_guard144 = assign7410_e8919;
        var_guard144_rv = 0.0;

        let assign7420_e8930: f64 = if (((p.p9 == 1.0) || (p.p9 == 2.0)) || (p.p9 == 5.0)) { 1.0 } else { 0.0 };
        var_guard145 = assign7420_e8930;
        var_guard145_rv = 0.0;

        let assign7430_e8941: f64 = if (((p.p9 == 3.0) || (p.p9 == 4.0)) || (p.p9 == 6.0)) { 1.0 } else { 0.0 };
        var_guard146 = assign7430_e8941;
        var_guard146_rv = 0.0;

        let assign7440_e8944: f64 = if var_nuends == 0.0 { 1.0 } else { 0.0 };
        var_guard147 = assign7440_e8944;
        var_guard147_rv = 0.0;

        let (assign7450_e8966,) = {
    if (((((((var_guard78 == 0.0) && (var_guard79 != 0.0)) && ((var_guard88 != 0.0) && (!((var_guard86 != 0.0) || (var_guard87 != 0.0))))) && (var_guard143 != 0.0)) && (var_guard144 != 0.0)) && (var_guard145 != 0.0)) && (var_guard147 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign7450_e8966;
        var_rend_rv = 0.0;

        let (assign7460_e8995,) = {
    if (((((((var_guard78 == 0.0) && (var_guard79 != 0.0)) && ((var_guard88 != 0.0) && (!((var_guard86 != 0.0) || (var_guard87 != 0.0))))) && (var_guard143 != 0.0)) && (var_guard144 != 0.0)) && (var_guard145 != 0.0)) && (var_guard147 == 0.0)) {
        let assign7460_e8989: f64 = (p.p438 * var_dmcgeff);
        let assign7460_e8992: f64 = (var_weff * var_nuends);
        let assign7460_e8993: f64 = (assign7460_e8989 / assign7460_e8992);
        (assign7460_e8993,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign7460_e8995;
        var_rend_rv = 0.0;

        let assign7480_e9005: f64 = if ((var_nuends == 0.0) || (var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        var_guard149 = assign7480_e9005;
        var_guard149_rv = 0.0;

        let (assign7490_e9030,) = {
    if (((((((var_guard78 == 0.0) && (var_guard79 != 0.0)) && ((var_guard88 != 0.0) && (!((var_guard86 != 0.0) || (var_guard87 != 0.0))))) && (var_guard143 != 0.0)) && (var_guard144 != 0.0)) && ((var_guard146 != 0.0) && (var_guard145 == 0.0))) && (var_guard149 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign7490_e9030;
        var_rend_rv = 0.0;

        let (assign7500_e9064,) = {
    if (((((((var_guard78 == 0.0) && (var_guard79 != 0.0)) && ((var_guard88 != 0.0) && (!((var_guard86 != 0.0) || (var_guard87 != 0.0))))) && (var_guard143 != 0.0)) && (var_guard144 != 0.0)) && ((var_guard146 != 0.0) && (var_guard145 == 0.0))) && (var_guard149 == 0.0)) {
        let assign7500_e9056: f64 = (p.p438 * var_weff);
        let assign7500_e9059: f64 = (6.0 * var_nuends);
        let assign7500_e9061: f64 = (assign7500_e9059 * var_dmcgeff);
        let assign7500_e9062: f64 = (assign7500_e9056 / assign7500_e9061);
        (assign7500_e9062,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign7500_e9064;
        var_rend_rv = 0.0;

        let (assign7510_e9087,) = {
    if ((((((var_guard78 == 0.0) && (var_guard79 != 0.0)) && ((var_guard88 != 0.0) && (!((var_guard86 != 0.0) || (var_guard87 != 0.0))))) && (var_guard143 != 0.0)) && (var_guard144 != 0.0)) && (!((var_guard145 != 0.0) || (var_guard146 != 0.0)))) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign7510_e9087;
        var_rend_rv = 0.0;

        let assign7520_e9098: f64 = if (((p.p9 == 1.0) || (p.p9 == 3.0)) || (p.p9 == 7.0)) { 1.0 } else { 0.0 };
        var_guard150 = assign7520_e9098;
        var_guard150_rv = 0.0;

        let assign7530_e9109: f64 = if (((p.p9 == 2.0) || (p.p9 == 4.0)) || (p.p9 == 8.0)) { 1.0 } else { 0.0 };
        var_guard151 = assign7530_e9109;
        var_guard151_rv = 0.0;

        let assign7540_e9112: f64 = if var_nuends == 0.0 { 1.0 } else { 0.0 };
        var_guard152 = assign7540_e9112;
        var_guard152_rv = 0.0;

        let (assign7550_e9135,) = {
    if (((((((var_guard78 == 0.0) && (var_guard79 != 0.0)) && ((var_guard88 != 0.0) && (!((var_guard86 != 0.0) || (var_guard87 != 0.0))))) && (var_guard143 != 0.0)) && (var_guard144 == 0.0)) && (var_guard150 != 0.0)) && (var_guard152 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign7550_e9135;
        var_rend_rv = 0.0;

        let (assign7560_e9165,) = {
    if (((((((var_guard78 == 0.0) && (var_guard79 != 0.0)) && ((var_guard88 != 0.0) && (!((var_guard86 != 0.0) || (var_guard87 != 0.0))))) && (var_guard143 != 0.0)) && (var_guard144 == 0.0)) && (var_guard150 != 0.0)) && (var_guard152 == 0.0)) {
        let assign7560_e9159: f64 = (p.p438 * var_dmcgeff);
        let assign7560_e9162: f64 = (var_weff * var_nuends);
        let assign7560_e9163: f64 = (assign7560_e9159 / assign7560_e9162);
        (assign7560_e9163,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign7560_e9165;
        var_rend_rv = 0.0;

        let assign7580_e9175: f64 = if ((var_nuends == 0.0) || (var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        var_guard154 = assign7580_e9175;
        var_guard154_rv = 0.0;

        let (assign7590_e9201,) = {
    if (((((((var_guard78 == 0.0) && (var_guard79 != 0.0)) && ((var_guard88 != 0.0) && (!((var_guard86 != 0.0) || (var_guard87 != 0.0))))) && (var_guard143 != 0.0)) && (var_guard144 == 0.0)) && ((var_guard151 != 0.0) && (var_guard150 == 0.0))) && (var_guard154 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign7590_e9201;
        var_rend_rv = 0.0;

        let (assign7600_e9236,) = {
    if (((((((var_guard78 == 0.0) && (var_guard79 != 0.0)) && ((var_guard88 != 0.0) && (!((var_guard86 != 0.0) || (var_guard87 != 0.0))))) && (var_guard143 != 0.0)) && (var_guard144 == 0.0)) && ((var_guard151 != 0.0) && (var_guard150 == 0.0))) && (var_guard154 == 0.0)) {
        let assign7600_e9228: f64 = (p.p438 * var_weff);
        let assign7600_e9231: f64 = (6.0 * var_nuends);
        let assign7600_e9233: f64 = (assign7600_e9231 * var_dmcgeff);
        let assign7600_e9234: f64 = (assign7600_e9228 / assign7600_e9233);
        (assign7600_e9234,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign7600_e9236;
        var_rend_rv = 0.0;

        let (assign7610_e9260,) = {
    if ((((((var_guard78 == 0.0) && (var_guard79 != 0.0)) && ((var_guard88 != 0.0) && (!((var_guard86 != 0.0) || (var_guard87 != 0.0))))) && (var_guard143 != 0.0)) && (var_guard144 == 0.0)) && (!((var_guard150 != 0.0) || (var_guard151 != 0.0)))) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign7610_e9260;
        var_rend_rv = 0.0;

        let assign7620_e9263: f64 = if 0.0 == 1.0 { 1.0 } else { 0.0 };
        var_guard155 = assign7620_e9263;
        var_guard155_rv = 0.0;

        let assign7630_e9274: f64 = if (((p.p9 == 1.0) || (p.p9 == 2.0)) || (p.p9 == 5.0)) { 1.0 } else { 0.0 };
        var_guard156 = assign7630_e9274;
        var_guard156_rv = 0.0;

        let assign7640_e9285: f64 = if (((p.p9 == 3.0) || (p.p9 == 4.0)) || (p.p9 == 6.0)) { 1.0 } else { 0.0 };
        var_guard157 = assign7640_e9285;
        var_guard157_rv = 0.0;

        let assign7650_e9288: f64 = if var_nuendd == 0.0 { 1.0 } else { 0.0 };
        var_guard158 = assign7650_e9288;
        var_guard158_rv = 0.0;

        let (assign7660_e9311,) = {
    if (((((((var_guard78 == 0.0) && (var_guard79 != 0.0)) && ((var_guard88 != 0.0) && (!((var_guard86 != 0.0) || (var_guard87 != 0.0))))) && (var_guard143 == 0.0)) && (var_guard155 != 0.0)) && (var_guard156 != 0.0)) && (var_guard158 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign7660_e9311;
        var_rend_rv = 0.0;

        let (assign7670_e9341,) = {
    if (((((((var_guard78 == 0.0) && (var_guard79 != 0.0)) && ((var_guard88 != 0.0) && (!((var_guard86 != 0.0) || (var_guard87 != 0.0))))) && (var_guard143 == 0.0)) && (var_guard155 != 0.0)) && (var_guard156 != 0.0)) && (var_guard158 == 0.0)) {
        let assign7670_e9335: f64 = (p.p438 * var_dmcgeff);
        let assign7670_e9338: f64 = (var_weff * var_nuendd);
        let assign7670_e9339: f64 = (assign7670_e9335 / assign7670_e9338);
        (assign7670_e9339,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign7670_e9341;
        var_rend_rv = 0.0;

        let assign7690_e9352: f64 = (var_dmcgeff + var_dmcieff);
        let assign7690_e9355: f64 = if ((var_nuendd == 0.0) || (assign7690_e9352 == 0.0)) { 1.0 } else { 0.0 };
        var_guard160 = assign7690_e9355;
        var_guard160_rv = 0.0;

        let (assign7700_e9381,) = {
    if (((((((var_guard78 == 0.0) && (var_guard79 != 0.0)) && ((var_guard88 != 0.0) && (!((var_guard86 != 0.0) || (var_guard87 != 0.0))))) && (var_guard143 == 0.0)) && (var_guard155 != 0.0)) && ((var_guard157 != 0.0) && (var_guard156 == 0.0))) && (var_guard160 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign7700_e9381;
        var_rend_rv = 0.0;

        let (assign7710_e9418,) = {
    if (((((((var_guard78 == 0.0) && (var_guard79 != 0.0)) && ((var_guard88 != 0.0) && (!((var_guard86 != 0.0) || (var_guard87 != 0.0))))) && (var_guard143 == 0.0)) && (var_guard155 != 0.0)) && ((var_guard157 != 0.0) && (var_guard156 == 0.0))) && (var_guard160 == 0.0)) {
        let assign7710_e9408: f64 = (p.p438 * var_weff);
        let assign7710_e9411: f64 = (3.0 * var_nuendd);
        let assign7710_e9414: f64 = (var_dmcgeff + var_dmcieff);
        let assign7710_e9415: f64 = (assign7710_e9411 * assign7710_e9414);
        let assign7710_e9416: f64 = (assign7710_e9408 / assign7710_e9415);
        (assign7710_e9416,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign7710_e9418;
        var_rend_rv = 0.0;

        let (assign7720_e9442,) = {
    if ((((((var_guard78 == 0.0) && (var_guard79 != 0.0)) && ((var_guard88 != 0.0) && (!((var_guard86 != 0.0) || (var_guard87 != 0.0))))) && (var_guard143 == 0.0)) && (var_guard155 != 0.0)) && (!((var_guard156 != 0.0) || (var_guard157 != 0.0)))) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign7720_e9442;
        var_rend_rv = 0.0;

        let assign7730_e9453: f64 = if (((p.p9 == 1.0) || (p.p9 == 3.0)) || (p.p9 == 7.0)) { 1.0 } else { 0.0 };
        var_guard161 = assign7730_e9453;
        var_guard161_rv = 0.0;

        let assign7740_e9464: f64 = if (((p.p9 == 2.0) || (p.p9 == 4.0)) || (p.p9 == 8.0)) { 1.0 } else { 0.0 };
        var_guard162 = assign7740_e9464;
        var_guard162_rv = 0.0;

        let assign7750_e9467: f64 = if var_nuendd == 0.0 { 1.0 } else { 0.0 };
        var_guard163 = assign7750_e9467;
        var_guard163_rv = 0.0;

        let (assign7760_e9491,) = {
    if (((((((var_guard78 == 0.0) && (var_guard79 != 0.0)) && ((var_guard88 != 0.0) && (!((var_guard86 != 0.0) || (var_guard87 != 0.0))))) && (var_guard143 == 0.0)) && (var_guard155 == 0.0)) && (var_guard161 != 0.0)) && (var_guard163 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign7760_e9491;
        var_rend_rv = 0.0;

        let (assign7770_e9522,) = {
    if (((((((var_guard78 == 0.0) && (var_guard79 != 0.0)) && ((var_guard88 != 0.0) && (!((var_guard86 != 0.0) || (var_guard87 != 0.0))))) && (var_guard143 == 0.0)) && (var_guard155 == 0.0)) && (var_guard161 != 0.0)) && (var_guard163 == 0.0)) {
        let assign7770_e9516: f64 = (p.p438 * var_dmcgeff);
        let assign7770_e9519: f64 = (var_weff * var_nuendd);
        let assign7770_e9520: f64 = (assign7770_e9516 / assign7770_e9519);
        (assign7770_e9520,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign7770_e9522;
        var_rend_rv = 0.0;

        let assign7790_e9533: f64 = (var_dmcgeff + var_dmcieff);
        let assign7790_e9536: f64 = if ((var_nuendd == 0.0) || (assign7790_e9533 == 0.0)) { 1.0 } else { 0.0 };
        var_guard165 = assign7790_e9536;
        var_guard165_rv = 0.0;

        let (assign7800_e9563,) = {
    if (((((((var_guard78 == 0.0) && (var_guard79 != 0.0)) && ((var_guard88 != 0.0) && (!((var_guard86 != 0.0) || (var_guard87 != 0.0))))) && (var_guard143 == 0.0)) && (var_guard155 == 0.0)) && ((var_guard162 != 0.0) && (var_guard161 == 0.0))) && (var_guard165 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign7800_e9563;
        var_rend_rv = 0.0;

        let (assign7810_e9601,) = {
    if (((((((var_guard78 == 0.0) && (var_guard79 != 0.0)) && ((var_guard88 != 0.0) && (!((var_guard86 != 0.0) || (var_guard87 != 0.0))))) && (var_guard143 == 0.0)) && (var_guard155 == 0.0)) && ((var_guard162 != 0.0) && (var_guard161 == 0.0))) && (var_guard165 == 0.0)) {
        let assign7810_e9591: f64 = (p.p438 * var_weff);
        let assign7810_e9594: f64 = (3.0 * var_nuendd);
        let assign7810_e9597: f64 = (var_dmcgeff + var_dmcieff);
        let assign7810_e9598: f64 = (assign7810_e9594 * assign7810_e9597);
        let assign7810_e9599: f64 = (assign7810_e9591 / assign7810_e9598);
        (assign7810_e9599,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign7810_e9601;
        var_rend_rv = 0.0;

        let (assign7820_e9626,) = {
    if ((((((var_guard78 == 0.0) && (var_guard79 != 0.0)) && ((var_guard88 != 0.0) && (!((var_guard86 != 0.0) || (var_guard87 != 0.0))))) && (var_guard143 == 0.0)) && (var_guard155 == 0.0)) && (!((var_guard161 != 0.0) || (var_guard162 != 0.0)))) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign7820_e9626;
        var_rend_rv = 0.0;

        let assign7830_e9629: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        var_guard166 = assign7830_e9629;
        var_guard166_rv = 0.0;

        let assign7840_e9632: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        var_guard167 = assign7840_e9632;
        var_guard167_rv = 0.0;

        let assign7850_e9643: f64 = if (((p.p9 == 1.0) || (p.p9 == 2.0)) || (p.p9 == 5.0)) { 1.0 } else { 0.0 };
        var_guard168 = assign7850_e9643;
        var_guard168_rv = 0.0;

        let assign7860_e9654: f64 = if (((p.p9 == 3.0) || (p.p9 == 4.0)) || (p.p9 == 6.0)) { 1.0 } else { 0.0 };
        var_guard169 = assign7860_e9654;
        var_guard169_rv = 0.0;

        let assign7870_e9657: f64 = if var_nuends == 0.0 { 1.0 } else { 0.0 };
        var_guard170 = assign7870_e9657;
        var_guard170_rv = 0.0;

        let (assign7880_e9681,) = {
    if (((((((var_guard78 == 0.0) && (var_guard79 != 0.0)) && ((var_guard89 != 0.0) && (!(((var_guard86 != 0.0) || (var_guard87 != 0.0)) || (var_guard88 != 0.0))))) && (var_guard166 != 0.0)) && (var_guard167 != 0.0)) && (var_guard168 != 0.0)) && (var_guard170 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign7880_e9681;
        var_rend_rv = 0.0;

        let (assign7890_e9712,) = {
    if (((((((var_guard78 == 0.0) && (var_guard79 != 0.0)) && ((var_guard89 != 0.0) && (!(((var_guard86 != 0.0) || (var_guard87 != 0.0)) || (var_guard88 != 0.0))))) && (var_guard166 != 0.0)) && (var_guard167 != 0.0)) && (var_guard168 != 0.0)) && (var_guard170 == 0.0)) {
        let assign7890_e9706: f64 = (p.p438 * var_dmcgeff);
        let assign7890_e9709: f64 = (var_weff * var_nuends);
        let assign7890_e9710: f64 = (assign7890_e9706 / assign7890_e9709);
        (assign7890_e9710,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign7890_e9712;
        var_rend_rv = 0.0;

        let assign7910_e9722: f64 = if ((var_nuends == 0.0) || (var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        var_guard172 = assign7910_e9722;
        var_guard172_rv = 0.0;

        let (assign7920_e9749,) = {
    if (((((((var_guard78 == 0.0) && (var_guard79 != 0.0)) && ((var_guard89 != 0.0) && (!(((var_guard86 != 0.0) || (var_guard87 != 0.0)) || (var_guard88 != 0.0))))) && (var_guard166 != 0.0)) && (var_guard167 != 0.0)) && ((var_guard169 != 0.0) && (var_guard168 == 0.0))) && (var_guard172 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign7920_e9749;
        var_rend_rv = 0.0;

        *var_guard134_slot = var_guard134;
        *var_guard134_rv_slot = var_guard134_rv;
        *var_guard135_slot = var_guard135;
        *var_guard135_rv_slot = var_guard135_rv;
        *var_guard137_slot = var_guard137;
        *var_guard137_rv_slot = var_guard137_rv;
        *var_guard138_slot = var_guard138;
        *var_guard138_rv_slot = var_guard138_rv;
        *var_guard139_slot = var_guard139;
        *var_guard139_rv_slot = var_guard139_rv;
        *var_guard140_slot = var_guard140;
        *var_guard140_rv_slot = var_guard140_rv;
        *var_guard142_slot = var_guard142;
        *var_guard142_rv_slot = var_guard142_rv;
        *var_guard143_slot = var_guard143;
        *var_guard143_rv_slot = var_guard143_rv;
        *var_guard144_slot = var_guard144;
        *var_guard144_rv_slot = var_guard144_rv;
        *var_guard145_slot = var_guard145;
        *var_guard145_rv_slot = var_guard145_rv;
        *var_guard146_slot = var_guard146;
        *var_guard146_rv_slot = var_guard146_rv;
        *var_guard147_slot = var_guard147;
        *var_guard147_rv_slot = var_guard147_rv;
        *var_guard149_slot = var_guard149;
        *var_guard149_rv_slot = var_guard149_rv;
        *var_guard150_slot = var_guard150;
        *var_guard150_rv_slot = var_guard150_rv;
        *var_guard151_slot = var_guard151;
        *var_guard151_rv_slot = var_guard151_rv;
        *var_guard152_slot = var_guard152;
        *var_guard152_rv_slot = var_guard152_rv;
        *var_guard154_slot = var_guard154;
        *var_guard154_rv_slot = var_guard154_rv;
        *var_guard155_slot = var_guard155;
        *var_guard155_rv_slot = var_guard155_rv;
        *var_guard156_slot = var_guard156;
        *var_guard156_rv_slot = var_guard156_rv;
        *var_guard157_slot = var_guard157;
        *var_guard157_rv_slot = var_guard157_rv;
        *var_guard158_slot = var_guard158;
        *var_guard158_rv_slot = var_guard158_rv;
        *var_guard160_slot = var_guard160;
        *var_guard160_rv_slot = var_guard160_rv;
        *var_guard161_slot = var_guard161;
        *var_guard161_rv_slot = var_guard161_rv;
        *var_guard162_slot = var_guard162;
        *var_guard162_rv_slot = var_guard162_rv;
        *var_guard163_slot = var_guard163;
        *var_guard163_rv_slot = var_guard163_rv;
        *var_guard165_slot = var_guard165;
        *var_guard165_rv_slot = var_guard165_rv;
        *var_guard166_slot = var_guard166;
        *var_guard166_rv_slot = var_guard166_rv;
        *var_guard167_slot = var_guard167;
        *var_guard167_rv_slot = var_guard167_rv;
        *var_guard168_slot = var_guard168;
        *var_guard168_rv_slot = var_guard168_rv;
        *var_guard169_slot = var_guard169;
        *var_guard169_rv_slot = var_guard169_rv;
        *var_guard170_slot = var_guard170;
        *var_guard170_rv_slot = var_guard170_rv;
        *var_guard172_slot = var_guard172;
        *var_guard172_rv_slot = var_guard172_rv;
        *var_rend_slot = var_rend;
        *var_rend_rv_slot = var_rend_rv;
    }

    pub(super) fn stamp_reactive_block_14(
        p: &Parameters,
        var_dmcgeff: f64,
        var_dmcieff: f64,
        var_dmdgeff: f64,
        var_guard166: f64,
        var_guard167: f64,
        var_guard168: f64,
        var_guard169: f64,
        var_guard172: f64,
        var_guard78: f64,
        var_guard79: f64,
        var_guard86: f64,
        var_guard87: f64,
        var_guard88: f64,
        var_guard89: f64,
        var_guard90: f64,
        var_guard91: f64,
        var_nuendd: f64,
        var_nuends: f64,
        var_weff: f64,
        var_guard173_slot: &mut f64,
        var_guard173_rv_slot: &mut f64,
        var_guard174_slot: &mut f64,
        var_guard174_rv_slot: &mut f64,
        var_guard175_slot: &mut f64,
        var_guard175_rv_slot: &mut f64,
        var_guard177_slot: &mut f64,
        var_guard177_rv_slot: &mut f64,
        var_guard178_slot: &mut f64,
        var_guard178_rv_slot: &mut f64,
        var_guard179_slot: &mut f64,
        var_guard179_rv_slot: &mut f64,
        var_guard180_slot: &mut f64,
        var_guard180_rv_slot: &mut f64,
        var_guard181_slot: &mut f64,
        var_guard181_rv_slot: &mut f64,
        var_guard183_slot: &mut f64,
        var_guard183_rv_slot: &mut f64,
        var_guard184_slot: &mut f64,
        var_guard184_rv_slot: &mut f64,
        var_guard185_slot: &mut f64,
        var_guard185_rv_slot: &mut f64,
        var_guard186_slot: &mut f64,
        var_guard186_rv_slot: &mut f64,
        var_guard188_slot: &mut f64,
        var_guard188_rv_slot: &mut f64,
        var_guard189_slot: &mut f64,
        var_guard189_rv_slot: &mut f64,
        var_guard190_slot: &mut f64,
        var_guard190_rv_slot: &mut f64,
        var_guard191_slot: &mut f64,
        var_guard191_rv_slot: &mut f64,
        var_guard192_slot: &mut f64,
        var_guard192_rv_slot: &mut f64,
        var_guard193_slot: &mut f64,
        var_guard193_rv_slot: &mut f64,
        var_guard195_slot: &mut f64,
        var_guard195_rv_slot: &mut f64,
        var_guard196_slot: &mut f64,
        var_guard196_rv_slot: &mut f64,
        var_guard197_slot: &mut f64,
        var_guard197_rv_slot: &mut f64,
        var_guard198_slot: &mut f64,
        var_guard198_rv_slot: &mut f64,
        var_guard200_slot: &mut f64,
        var_guard200_rv_slot: &mut f64,
        var_guard201_slot: &mut f64,
        var_guard201_rv_slot: &mut f64,
        var_guard202_slot: &mut f64,
        var_guard202_rv_slot: &mut f64,
        var_guard203_slot: &mut f64,
        var_guard203_rv_slot: &mut f64,
        var_guard204_slot: &mut f64,
        var_guard204_rv_slot: &mut f64,
        var_guard205_slot: &mut f64,
        var_guard205_rv_slot: &mut f64,
        var_guard207_slot: &mut f64,
        var_guard207_rv_slot: &mut f64,
        var_guard208_slot: &mut f64,
        var_guard208_rv_slot: &mut f64,
        var_guard209_slot: &mut f64,
        var_guard209_rv_slot: &mut f64,
        var_guard210_slot: &mut f64,
        var_guard210_rv_slot: &mut f64,
        var_rend_slot: &mut f64,
        var_rend_rv_slot: &mut f64,
    ) {
        let mut var_guard173: f64 = *var_guard173_slot;
        let mut var_guard173_rv: f64 = *var_guard173_rv_slot;
        let mut var_guard174: f64 = *var_guard174_slot;
        let mut var_guard174_rv: f64 = *var_guard174_rv_slot;
        let mut var_guard175: f64 = *var_guard175_slot;
        let mut var_guard175_rv: f64 = *var_guard175_rv_slot;
        let mut var_guard177: f64 = *var_guard177_slot;
        let mut var_guard177_rv: f64 = *var_guard177_rv_slot;
        let mut var_guard178: f64 = *var_guard178_slot;
        let mut var_guard178_rv: f64 = *var_guard178_rv_slot;
        let mut var_guard179: f64 = *var_guard179_slot;
        let mut var_guard179_rv: f64 = *var_guard179_rv_slot;
        let mut var_guard180: f64 = *var_guard180_slot;
        let mut var_guard180_rv: f64 = *var_guard180_rv_slot;
        let mut var_guard181: f64 = *var_guard181_slot;
        let mut var_guard181_rv: f64 = *var_guard181_rv_slot;
        let mut var_guard183: f64 = *var_guard183_slot;
        let mut var_guard183_rv: f64 = *var_guard183_rv_slot;
        let mut var_guard184: f64 = *var_guard184_slot;
        let mut var_guard184_rv: f64 = *var_guard184_rv_slot;
        let mut var_guard185: f64 = *var_guard185_slot;
        let mut var_guard185_rv: f64 = *var_guard185_rv_slot;
        let mut var_guard186: f64 = *var_guard186_slot;
        let mut var_guard186_rv: f64 = *var_guard186_rv_slot;
        let mut var_guard188: f64 = *var_guard188_slot;
        let mut var_guard188_rv: f64 = *var_guard188_rv_slot;
        let mut var_guard189: f64 = *var_guard189_slot;
        let mut var_guard189_rv: f64 = *var_guard189_rv_slot;
        let mut var_guard190: f64 = *var_guard190_slot;
        let mut var_guard190_rv: f64 = *var_guard190_rv_slot;
        let mut var_guard191: f64 = *var_guard191_slot;
        let mut var_guard191_rv: f64 = *var_guard191_rv_slot;
        let mut var_guard192: f64 = *var_guard192_slot;
        let mut var_guard192_rv: f64 = *var_guard192_rv_slot;
        let mut var_guard193: f64 = *var_guard193_slot;
        let mut var_guard193_rv: f64 = *var_guard193_rv_slot;
        let mut var_guard195: f64 = *var_guard195_slot;
        let mut var_guard195_rv: f64 = *var_guard195_rv_slot;
        let mut var_guard196: f64 = *var_guard196_slot;
        let mut var_guard196_rv: f64 = *var_guard196_rv_slot;
        let mut var_guard197: f64 = *var_guard197_slot;
        let mut var_guard197_rv: f64 = *var_guard197_rv_slot;
        let mut var_guard198: f64 = *var_guard198_slot;
        let mut var_guard198_rv: f64 = *var_guard198_rv_slot;
        let mut var_guard200: f64 = *var_guard200_slot;
        let mut var_guard200_rv: f64 = *var_guard200_rv_slot;
        let mut var_guard201: f64 = *var_guard201_slot;
        let mut var_guard201_rv: f64 = *var_guard201_rv_slot;
        let mut var_guard202: f64 = *var_guard202_slot;
        let mut var_guard202_rv: f64 = *var_guard202_rv_slot;
        let mut var_guard203: f64 = *var_guard203_slot;
        let mut var_guard203_rv: f64 = *var_guard203_rv_slot;
        let mut var_guard204: f64 = *var_guard204_slot;
        let mut var_guard204_rv: f64 = *var_guard204_rv_slot;
        let mut var_guard205: f64 = *var_guard205_slot;
        let mut var_guard205_rv: f64 = *var_guard205_rv_slot;
        let mut var_guard207: f64 = *var_guard207_slot;
        let mut var_guard207_rv: f64 = *var_guard207_rv_slot;
        let mut var_guard208: f64 = *var_guard208_slot;
        let mut var_guard208_rv: f64 = *var_guard208_rv_slot;
        let mut var_guard209: f64 = *var_guard209_slot;
        let mut var_guard209_rv: f64 = *var_guard209_rv_slot;
        let mut var_guard210: f64 = *var_guard210_slot;
        let mut var_guard210_rv: f64 = *var_guard210_rv_slot;
        let mut var_rend: f64 = *var_rend_slot;
        let mut var_rend_rv: f64 = *var_rend_rv_slot;

        let (assign7930_e9785,) = {
    if (((((((var_guard78 == 0.0) && (var_guard79 != 0.0)) && ((var_guard89 != 0.0) && (!(((var_guard86 != 0.0) || (var_guard87 != 0.0)) || (var_guard88 != 0.0))))) && (var_guard166 != 0.0)) && (var_guard167 != 0.0)) && ((var_guard169 != 0.0) && (var_guard168 == 0.0))) && (var_guard172 == 0.0)) {
        let assign7930_e9777: f64 = (p.p438 * var_weff);
        let assign7930_e9780: f64 = (6.0 * var_nuends);
        let assign7930_e9782: f64 = (assign7930_e9780 * var_dmcgeff);
        let assign7930_e9783: f64 = (assign7930_e9777 / assign7930_e9782);
        (assign7930_e9783,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign7930_e9785;
        var_rend_rv = 0.0;

        let (assign7940_e9810,) = {
    if ((((((var_guard78 == 0.0) && (var_guard79 != 0.0)) && ((var_guard89 != 0.0) && (!(((var_guard86 != 0.0) || (var_guard87 != 0.0)) || (var_guard88 != 0.0))))) && (var_guard166 != 0.0)) && (var_guard167 != 0.0)) && (!((var_guard168 != 0.0) || (var_guard169 != 0.0)))) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign7940_e9810;
        var_rend_rv = 0.0;

        let assign7950_e9821: f64 = if (((p.p9 == 1.0) || (p.p9 == 3.0)) || (p.p9 == 7.0)) { 1.0 } else { 0.0 };
        var_guard173 = assign7950_e9821;
        var_guard173_rv = 0.0;

        let assign7960_e9832: f64 = if (((p.p9 == 2.0) || (p.p9 == 4.0)) || (p.p9 == 8.0)) { 1.0 } else { 0.0 };
        var_guard174 = assign7960_e9832;
        var_guard174_rv = 0.0;

        let assign7970_e9835: f64 = if var_nuends == 0.0 { 1.0 } else { 0.0 };
        var_guard175 = assign7970_e9835;
        var_guard175_rv = 0.0;

        let (assign7980_e9860,) = {
    if (((((((var_guard78 == 0.0) && (var_guard79 != 0.0)) && ((var_guard89 != 0.0) && (!(((var_guard86 != 0.0) || (var_guard87 != 0.0)) || (var_guard88 != 0.0))))) && (var_guard166 != 0.0)) && (var_guard167 == 0.0)) && (var_guard173 != 0.0)) && (var_guard175 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign7980_e9860;
        var_rend_rv = 0.0;

        let (assign7990_e9892,) = {
    if (((((((var_guard78 == 0.0) && (var_guard79 != 0.0)) && ((var_guard89 != 0.0) && (!(((var_guard86 != 0.0) || (var_guard87 != 0.0)) || (var_guard88 != 0.0))))) && (var_guard166 != 0.0)) && (var_guard167 == 0.0)) && (var_guard173 != 0.0)) && (var_guard175 == 0.0)) {
        let assign7990_e9886: f64 = (p.p438 * var_dmcgeff);
        let assign7990_e9889: f64 = (var_weff * var_nuends);
        let assign7990_e9890: f64 = (assign7990_e9886 / assign7990_e9889);
        (assign7990_e9890,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign7990_e9892;
        var_rend_rv = 0.0;

        let assign8010_e9902: f64 = if ((var_nuends == 0.0) || (var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        var_guard177 = assign8010_e9902;
        var_guard177_rv = 0.0;

        let (assign8020_e9930,) = {
    if (((((((var_guard78 == 0.0) && (var_guard79 != 0.0)) && ((var_guard89 != 0.0) && (!(((var_guard86 != 0.0) || (var_guard87 != 0.0)) || (var_guard88 != 0.0))))) && (var_guard166 != 0.0)) && (var_guard167 == 0.0)) && ((var_guard174 != 0.0) && (var_guard173 == 0.0))) && (var_guard177 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign8020_e9930;
        var_rend_rv = 0.0;

        let (assign8030_e9967,) = {
    if (((((((var_guard78 == 0.0) && (var_guard79 != 0.0)) && ((var_guard89 != 0.0) && (!(((var_guard86 != 0.0) || (var_guard87 != 0.0)) || (var_guard88 != 0.0))))) && (var_guard166 != 0.0)) && (var_guard167 == 0.0)) && ((var_guard174 != 0.0) && (var_guard173 == 0.0))) && (var_guard177 == 0.0)) {
        let assign8030_e9959: f64 = (p.p438 * var_weff);
        let assign8030_e9962: f64 = (6.0 * var_nuends);
        let assign8030_e9964: f64 = (assign8030_e9962 * var_dmcgeff);
        let assign8030_e9965: f64 = (assign8030_e9959 / assign8030_e9964);
        (assign8030_e9965,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign8030_e9967;
        var_rend_rv = 0.0;

        let (assign8040_e9993,) = {
    if ((((((var_guard78 == 0.0) && (var_guard79 != 0.0)) && ((var_guard89 != 0.0) && (!(((var_guard86 != 0.0) || (var_guard87 != 0.0)) || (var_guard88 != 0.0))))) && (var_guard166 != 0.0)) && (var_guard167 == 0.0)) && (!((var_guard173 != 0.0) || (var_guard174 != 0.0)))) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign8040_e9993;
        var_rend_rv = 0.0;

        let assign8050_e9996: f64 = if 0.0 == 1.0 { 1.0 } else { 0.0 };
        var_guard178 = assign8050_e9996;
        var_guard178_rv = 0.0;

        let assign8060_e10007: f64 = if (((p.p9 == 1.0) || (p.p9 == 2.0)) || (p.p9 == 5.0)) { 1.0 } else { 0.0 };
        var_guard179 = assign8060_e10007;
        var_guard179_rv = 0.0;

        let assign8070_e10018: f64 = if (((p.p9 == 3.0) || (p.p9 == 4.0)) || (p.p9 == 6.0)) { 1.0 } else { 0.0 };
        var_guard180 = assign8070_e10018;
        var_guard180_rv = 0.0;

        let assign8080_e10021: f64 = if var_nuendd == 0.0 { 1.0 } else { 0.0 };
        var_guard181 = assign8080_e10021;
        var_guard181_rv = 0.0;

        let (assign8090_e10046,) = {
    if (((((((var_guard78 == 0.0) && (var_guard79 != 0.0)) && ((var_guard89 != 0.0) && (!(((var_guard86 != 0.0) || (var_guard87 != 0.0)) || (var_guard88 != 0.0))))) && (var_guard166 == 0.0)) && (var_guard178 != 0.0)) && (var_guard179 != 0.0)) && (var_guard181 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign8090_e10046;
        var_rend_rv = 0.0;

        let (assign8100_e10078,) = {
    if (((((((var_guard78 == 0.0) && (var_guard79 != 0.0)) && ((var_guard89 != 0.0) && (!(((var_guard86 != 0.0) || (var_guard87 != 0.0)) || (var_guard88 != 0.0))))) && (var_guard166 == 0.0)) && (var_guard178 != 0.0)) && (var_guard179 != 0.0)) && (var_guard181 == 0.0)) {
        let assign8100_e10072: f64 = (p.p438 * var_dmcgeff);
        let assign8100_e10075: f64 = (var_weff * var_nuendd);
        let assign8100_e10076: f64 = (assign8100_e10072 / assign8100_e10075);
        (assign8100_e10076,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign8100_e10078;
        var_rend_rv = 0.0;

        let assign8120_e10088: f64 = if ((var_nuendd == 0.0) || (var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        var_guard183 = assign8120_e10088;
        var_guard183_rv = 0.0;

        let (assign8130_e10116,) = {
    if (((((((var_guard78 == 0.0) && (var_guard79 != 0.0)) && ((var_guard89 != 0.0) && (!(((var_guard86 != 0.0) || (var_guard87 != 0.0)) || (var_guard88 != 0.0))))) && (var_guard166 == 0.0)) && (var_guard178 != 0.0)) && ((var_guard180 != 0.0) && (var_guard179 == 0.0))) && (var_guard183 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign8130_e10116;
        var_rend_rv = 0.0;

        let (assign8140_e10153,) = {
    if (((((((var_guard78 == 0.0) && (var_guard79 != 0.0)) && ((var_guard89 != 0.0) && (!(((var_guard86 != 0.0) || (var_guard87 != 0.0)) || (var_guard88 != 0.0))))) && (var_guard166 == 0.0)) && (var_guard178 != 0.0)) && ((var_guard180 != 0.0) && (var_guard179 == 0.0))) && (var_guard183 == 0.0)) {
        let assign8140_e10145: f64 = (p.p438 * var_weff);
        let assign8140_e10148: f64 = (6.0 * var_nuendd);
        let assign8140_e10150: f64 = (assign8140_e10148 * var_dmcgeff);
        let assign8140_e10151: f64 = (assign8140_e10145 / assign8140_e10150);
        (assign8140_e10151,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign8140_e10153;
        var_rend_rv = 0.0;

        let (assign8150_e10179,) = {
    if ((((((var_guard78 == 0.0) && (var_guard79 != 0.0)) && ((var_guard89 != 0.0) && (!(((var_guard86 != 0.0) || (var_guard87 != 0.0)) || (var_guard88 != 0.0))))) && (var_guard166 == 0.0)) && (var_guard178 != 0.0)) && (!((var_guard179 != 0.0) || (var_guard180 != 0.0)))) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign8150_e10179;
        var_rend_rv = 0.0;

        let assign8160_e10190: f64 = if (((p.p9 == 1.0) || (p.p9 == 3.0)) || (p.p9 == 7.0)) { 1.0 } else { 0.0 };
        var_guard184 = assign8160_e10190;
        var_guard184_rv = 0.0;

        let assign8170_e10201: f64 = if (((p.p9 == 2.0) || (p.p9 == 4.0)) || (p.p9 == 8.0)) { 1.0 } else { 0.0 };
        var_guard185 = assign8170_e10201;
        var_guard185_rv = 0.0;

        let assign8180_e10204: f64 = if var_nuendd == 0.0 { 1.0 } else { 0.0 };
        var_guard186 = assign8180_e10204;
        var_guard186_rv = 0.0;

        let (assign8190_e10230,) = {
    if (((((((var_guard78 == 0.0) && (var_guard79 != 0.0)) && ((var_guard89 != 0.0) && (!(((var_guard86 != 0.0) || (var_guard87 != 0.0)) || (var_guard88 != 0.0))))) && (var_guard166 == 0.0)) && (var_guard178 == 0.0)) && (var_guard184 != 0.0)) && (var_guard186 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign8190_e10230;
        var_rend_rv = 0.0;

        let (assign8200_e10263,) = {
    if (((((((var_guard78 == 0.0) && (var_guard79 != 0.0)) && ((var_guard89 != 0.0) && (!(((var_guard86 != 0.0) || (var_guard87 != 0.0)) || (var_guard88 != 0.0))))) && (var_guard166 == 0.0)) && (var_guard178 == 0.0)) && (var_guard184 != 0.0)) && (var_guard186 == 0.0)) {
        let assign8200_e10257: f64 = (p.p438 * var_dmcgeff);
        let assign8200_e10260: f64 = (var_weff * var_nuendd);
        let assign8200_e10261: f64 = (assign8200_e10257 / assign8200_e10260);
        (assign8200_e10261,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign8200_e10263;
        var_rend_rv = 0.0;

        let assign8220_e10273: f64 = if ((var_nuendd == 0.0) || (var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        var_guard188 = assign8220_e10273;
        var_guard188_rv = 0.0;

        let (assign8230_e10302,) = {
    if (((((((var_guard78 == 0.0) && (var_guard79 != 0.0)) && ((var_guard89 != 0.0) && (!(((var_guard86 != 0.0) || (var_guard87 != 0.0)) || (var_guard88 != 0.0))))) && (var_guard166 == 0.0)) && (var_guard178 == 0.0)) && ((var_guard185 != 0.0) && (var_guard184 == 0.0))) && (var_guard188 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign8230_e10302;
        var_rend_rv = 0.0;

        let (assign8240_e10340,) = {
    if (((((((var_guard78 == 0.0) && (var_guard79 != 0.0)) && ((var_guard89 != 0.0) && (!(((var_guard86 != 0.0) || (var_guard87 != 0.0)) || (var_guard88 != 0.0))))) && (var_guard166 == 0.0)) && (var_guard178 == 0.0)) && ((var_guard185 != 0.0) && (var_guard184 == 0.0))) && (var_guard188 == 0.0)) {
        let assign8240_e10332: f64 = (p.p438 * var_weff);
        let assign8240_e10335: f64 = (6.0 * var_nuendd);
        let assign8240_e10337: f64 = (assign8240_e10335 * var_dmcgeff);
        let assign8240_e10338: f64 = (assign8240_e10332 / assign8240_e10337);
        (assign8240_e10338,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign8240_e10340;
        var_rend_rv = 0.0;

        let (assign8250_e10367,) = {
    if ((((((var_guard78 == 0.0) && (var_guard79 != 0.0)) && ((var_guard89 != 0.0) && (!(((var_guard86 != 0.0) || (var_guard87 != 0.0)) || (var_guard88 != 0.0))))) && (var_guard166 == 0.0)) && (var_guard178 == 0.0)) && (!((var_guard184 != 0.0) || (var_guard185 != 0.0)))) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign8250_e10367;
        var_rend_rv = 0.0;

        let assign8260_e10370: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        var_guard189 = assign8260_e10370;
        var_guard189_rv = 0.0;

        let assign8270_e10373: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        var_guard190 = assign8270_e10373;
        var_guard190_rv = 0.0;

        let assign8280_e10384: f64 = if (((p.p9 == 1.0) || (p.p9 == 2.0)) || (p.p9 == 5.0)) { 1.0 } else { 0.0 };
        var_guard191 = assign8280_e10384;
        var_guard191_rv = 0.0;

        let assign8290_e10395: f64 = if (((p.p9 == 3.0) || (p.p9 == 4.0)) || (p.p9 == 6.0)) { 1.0 } else { 0.0 };
        var_guard192 = assign8290_e10395;
        var_guard192_rv = 0.0;

        let assign8300_e10398: f64 = if var_nuends == 0.0 { 1.0 } else { 0.0 };
        var_guard193 = assign8300_e10398;
        var_guard193_rv = 0.0;

        let (assign8310_e10424,) = {
    if (((((((var_guard78 == 0.0) && (var_guard79 != 0.0)) && ((var_guard90 != 0.0) && (!((((var_guard86 != 0.0) || (var_guard87 != 0.0)) || (var_guard88 != 0.0)) || (var_guard89 != 0.0))))) && (var_guard189 != 0.0)) && (var_guard190 != 0.0)) && (var_guard191 != 0.0)) && (var_guard193 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign8310_e10424;
        var_rend_rv = 0.0;

        let (assign8320_e10457,) = {
    if (((((((var_guard78 == 0.0) && (var_guard79 != 0.0)) && ((var_guard90 != 0.0) && (!((((var_guard86 != 0.0) || (var_guard87 != 0.0)) || (var_guard88 != 0.0)) || (var_guard89 != 0.0))))) && (var_guard189 != 0.0)) && (var_guard190 != 0.0)) && (var_guard191 != 0.0)) && (var_guard193 == 0.0)) {
        let assign8320_e10451: f64 = (p.p438 * var_dmcgeff);
        let assign8320_e10454: f64 = (var_weff * var_nuends);
        let assign8320_e10455: f64 = (assign8320_e10451 / assign8320_e10454);
        (assign8320_e10455,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign8320_e10457;
        var_rend_rv = 0.0;

        let assign8340_e10468: f64 = (var_dmcgeff + var_dmcieff);
        let assign8340_e10471: f64 = if ((var_nuends == 0.0) || (assign8340_e10468 == 0.0)) { 1.0 } else { 0.0 };
        var_guard195 = assign8340_e10471;
        var_guard195_rv = 0.0;

        let (assign8350_e10500,) = {
    if (((((((var_guard78 == 0.0) && (var_guard79 != 0.0)) && ((var_guard90 != 0.0) && (!((((var_guard86 != 0.0) || (var_guard87 != 0.0)) || (var_guard88 != 0.0)) || (var_guard89 != 0.0))))) && (var_guard189 != 0.0)) && (var_guard190 != 0.0)) && ((var_guard192 != 0.0) && (var_guard191 == 0.0))) && (var_guard195 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign8350_e10500;
        var_rend_rv = 0.0;

        let (assign8360_e10540,) = {
    if (((((((var_guard78 == 0.0) && (var_guard79 != 0.0)) && ((var_guard90 != 0.0) && (!((((var_guard86 != 0.0) || (var_guard87 != 0.0)) || (var_guard88 != 0.0)) || (var_guard89 != 0.0))))) && (var_guard189 != 0.0)) && (var_guard190 != 0.0)) && ((var_guard192 != 0.0) && (var_guard191 == 0.0))) && (var_guard195 == 0.0)) {
        let assign8360_e10530: f64 = (p.p438 * var_weff);
        let assign8360_e10533: f64 = (3.0 * var_nuends);
        let assign8360_e10536: f64 = (var_dmcgeff + var_dmcieff);
        let assign8360_e10537: f64 = (assign8360_e10533 * assign8360_e10536);
        let assign8360_e10538: f64 = (assign8360_e10530 / assign8360_e10537);
        (assign8360_e10538,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign8360_e10540;
        var_rend_rv = 0.0;

        let (assign8370_e10567,) = {
    if ((((((var_guard78 == 0.0) && (var_guard79 != 0.0)) && ((var_guard90 != 0.0) && (!((((var_guard86 != 0.0) || (var_guard87 != 0.0)) || (var_guard88 != 0.0)) || (var_guard89 != 0.0))))) && (var_guard189 != 0.0)) && (var_guard190 != 0.0)) && (!((var_guard191 != 0.0) || (var_guard192 != 0.0)))) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign8370_e10567;
        var_rend_rv = 0.0;

        let assign8380_e10578: f64 = if (((p.p9 == 1.0) || (p.p9 == 3.0)) || (p.p9 == 7.0)) { 1.0 } else { 0.0 };
        var_guard196 = assign8380_e10578;
        var_guard196_rv = 0.0;

        let assign8390_e10589: f64 = if (((p.p9 == 2.0) || (p.p9 == 4.0)) || (p.p9 == 8.0)) { 1.0 } else { 0.0 };
        var_guard197 = assign8390_e10589;
        var_guard197_rv = 0.0;

        let assign8400_e10592: f64 = if var_nuends == 0.0 { 1.0 } else { 0.0 };
        var_guard198 = assign8400_e10592;
        var_guard198_rv = 0.0;

        let (assign8410_e10619,) = {
    if (((((((var_guard78 == 0.0) && (var_guard79 != 0.0)) && ((var_guard90 != 0.0) && (!((((var_guard86 != 0.0) || (var_guard87 != 0.0)) || (var_guard88 != 0.0)) || (var_guard89 != 0.0))))) && (var_guard189 != 0.0)) && (var_guard190 == 0.0)) && (var_guard196 != 0.0)) && (var_guard198 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign8410_e10619;
        var_rend_rv = 0.0;

        let (assign8420_e10653,) = {
    if (((((((var_guard78 == 0.0) && (var_guard79 != 0.0)) && ((var_guard90 != 0.0) && (!((((var_guard86 != 0.0) || (var_guard87 != 0.0)) || (var_guard88 != 0.0)) || (var_guard89 != 0.0))))) && (var_guard189 != 0.0)) && (var_guard190 == 0.0)) && (var_guard196 != 0.0)) && (var_guard198 == 0.0)) {
        let assign8420_e10647: f64 = (p.p438 * var_dmcgeff);
        let assign8420_e10650: f64 = (var_weff * var_nuends);
        let assign8420_e10651: f64 = (assign8420_e10647 / assign8420_e10650);
        (assign8420_e10651,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign8420_e10653;
        var_rend_rv = 0.0;

        let assign8440_e10664: f64 = (var_dmcgeff + var_dmcieff);
        let assign8440_e10667: f64 = if ((var_nuends == 0.0) || (assign8440_e10664 == 0.0)) { 1.0 } else { 0.0 };
        var_guard200 = assign8440_e10667;
        var_guard200_rv = 0.0;

        let (assign8450_e10697,) = {
    if (((((((var_guard78 == 0.0) && (var_guard79 != 0.0)) && ((var_guard90 != 0.0) && (!((((var_guard86 != 0.0) || (var_guard87 != 0.0)) || (var_guard88 != 0.0)) || (var_guard89 != 0.0))))) && (var_guard189 != 0.0)) && (var_guard190 == 0.0)) && ((var_guard197 != 0.0) && (var_guard196 == 0.0))) && (var_guard200 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign8450_e10697;
        var_rend_rv = 0.0;

        let (assign8460_e10738,) = {
    if (((((((var_guard78 == 0.0) && (var_guard79 != 0.0)) && ((var_guard90 != 0.0) && (!((((var_guard86 != 0.0) || (var_guard87 != 0.0)) || (var_guard88 != 0.0)) || (var_guard89 != 0.0))))) && (var_guard189 != 0.0)) && (var_guard190 == 0.0)) && ((var_guard197 != 0.0) && (var_guard196 == 0.0))) && (var_guard200 == 0.0)) {
        let assign8460_e10728: f64 = (p.p438 * var_weff);
        let assign8460_e10731: f64 = (3.0 * var_nuends);
        let assign8460_e10734: f64 = (var_dmcgeff + var_dmcieff);
        let assign8460_e10735: f64 = (assign8460_e10731 * assign8460_e10734);
        let assign8460_e10736: f64 = (assign8460_e10728 / assign8460_e10735);
        (assign8460_e10736,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign8460_e10738;
        var_rend_rv = 0.0;

        let (assign8470_e10766,) = {
    if ((((((var_guard78 == 0.0) && (var_guard79 != 0.0)) && ((var_guard90 != 0.0) && (!((((var_guard86 != 0.0) || (var_guard87 != 0.0)) || (var_guard88 != 0.0)) || (var_guard89 != 0.0))))) && (var_guard189 != 0.0)) && (var_guard190 == 0.0)) && (!((var_guard196 != 0.0) || (var_guard197 != 0.0)))) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign8470_e10766;
        var_rend_rv = 0.0;

        let (assign8480_e10791,) = {
    if ((((var_guard78 == 0.0) && (var_guard79 != 0.0)) && ((var_guard90 != 0.0) && (!((((var_guard86 != 0.0) || (var_guard87 != 0.0)) || (var_guard88 != 0.0)) || (var_guard89 != 0.0))))) && (var_guard189 == 0.0)) {
        let assign8480_e10787: f64 = (p.p438 * var_dmdgeff);
        let assign8480_e10789: f64 = (assign8480_e10787 / var_weff);
        (assign8480_e10789,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign8480_e10791;
        var_rend_rv = 0.0;

        let assign8490_e10794: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        var_guard201 = assign8490_e10794;
        var_guard201_rv = 0.0;

        let assign8500_e10797: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        var_guard202 = assign8500_e10797;
        var_guard202_rv = 0.0;

        let assign8510_e10808: f64 = if (((p.p9 == 1.0) || (p.p9 == 2.0)) || (p.p9 == 5.0)) { 1.0 } else { 0.0 };
        var_guard203 = assign8510_e10808;
        var_guard203_rv = 0.0;

        let assign8520_e10819: f64 = if (((p.p9 == 3.0) || (p.p9 == 4.0)) || (p.p9 == 6.0)) { 1.0 } else { 0.0 };
        var_guard204 = assign8520_e10819;
        var_guard204_rv = 0.0;

        let assign8530_e10822: f64 = if var_nuends == 0.0 { 1.0 } else { 0.0 };
        var_guard205 = assign8530_e10822;
        var_guard205_rv = 0.0;

        let (assign8540_e10850,) = {
    if (((((((var_guard78 == 0.0) && (var_guard79 != 0.0)) && ((var_guard91 != 0.0) && (!(((((var_guard86 != 0.0) || (var_guard87 != 0.0)) || (var_guard88 != 0.0)) || (var_guard89 != 0.0)) || (var_guard90 != 0.0))))) && (var_guard201 != 0.0)) && (var_guard202 != 0.0)) && (var_guard203 != 0.0)) && (var_guard205 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign8540_e10850;
        var_rend_rv = 0.0;

        let (assign8550_e10885,) = {
    if (((((((var_guard78 == 0.0) && (var_guard79 != 0.0)) && ((var_guard91 != 0.0) && (!(((((var_guard86 != 0.0) || (var_guard87 != 0.0)) || (var_guard88 != 0.0)) || (var_guard89 != 0.0)) || (var_guard90 != 0.0))))) && (var_guard201 != 0.0)) && (var_guard202 != 0.0)) && (var_guard203 != 0.0)) && (var_guard205 == 0.0)) {
        let assign8550_e10879: f64 = (p.p438 * var_dmcgeff);
        let assign8550_e10882: f64 = (var_weff * var_nuends);
        let assign8550_e10883: f64 = (assign8550_e10879 / assign8550_e10882);
        (assign8550_e10883,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign8550_e10885;
        var_rend_rv = 0.0;

        let assign8570_e10895: f64 = if ((var_nuends == 0.0) || (var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        var_guard207 = assign8570_e10895;
        var_guard207_rv = 0.0;

        let (assign8580_e10926,) = {
    if (((((((var_guard78 == 0.0) && (var_guard79 != 0.0)) && ((var_guard91 != 0.0) && (!(((((var_guard86 != 0.0) || (var_guard87 != 0.0)) || (var_guard88 != 0.0)) || (var_guard89 != 0.0)) || (var_guard90 != 0.0))))) && (var_guard201 != 0.0)) && (var_guard202 != 0.0)) && ((var_guard204 != 0.0) && (var_guard203 == 0.0))) && (var_guard207 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign8580_e10926;
        var_rend_rv = 0.0;

        let (assign8590_e10966,) = {
    if (((((((var_guard78 == 0.0) && (var_guard79 != 0.0)) && ((var_guard91 != 0.0) && (!(((((var_guard86 != 0.0) || (var_guard87 != 0.0)) || (var_guard88 != 0.0)) || (var_guard89 != 0.0)) || (var_guard90 != 0.0))))) && (var_guard201 != 0.0)) && (var_guard202 != 0.0)) && ((var_guard204 != 0.0) && (var_guard203 == 0.0))) && (var_guard207 == 0.0)) {
        let assign8590_e10958: f64 = (p.p438 * var_weff);
        let assign8590_e10961: f64 = (6.0 * var_nuends);
        let assign8590_e10963: f64 = (assign8590_e10961 * var_dmcgeff);
        let assign8590_e10964: f64 = (assign8590_e10958 / assign8590_e10963);
        (assign8590_e10964,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign8590_e10966;
        var_rend_rv = 0.0;

        let (assign8600_e10995,) = {
    if ((((((var_guard78 == 0.0) && (var_guard79 != 0.0)) && ((var_guard91 != 0.0) && (!(((((var_guard86 != 0.0) || (var_guard87 != 0.0)) || (var_guard88 != 0.0)) || (var_guard89 != 0.0)) || (var_guard90 != 0.0))))) && (var_guard201 != 0.0)) && (var_guard202 != 0.0)) && (!((var_guard203 != 0.0) || (var_guard204 != 0.0)))) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign8600_e10995;
        var_rend_rv = 0.0;

        let assign8610_e11006: f64 = if (((p.p9 == 1.0) || (p.p9 == 3.0)) || (p.p9 == 7.0)) { 1.0 } else { 0.0 };
        var_guard208 = assign8610_e11006;
        var_guard208_rv = 0.0;

        let assign8620_e11017: f64 = if (((p.p9 == 2.0) || (p.p9 == 4.0)) || (p.p9 == 8.0)) { 1.0 } else { 0.0 };
        var_guard209 = assign8620_e11017;
        var_guard209_rv = 0.0;

        let assign8630_e11020: f64 = if var_nuends == 0.0 { 1.0 } else { 0.0 };
        var_guard210 = assign8630_e11020;
        var_guard210_rv = 0.0;

        *var_guard173_slot = var_guard173;
        *var_guard173_rv_slot = var_guard173_rv;
        *var_guard174_slot = var_guard174;
        *var_guard174_rv_slot = var_guard174_rv;
        *var_guard175_slot = var_guard175;
        *var_guard175_rv_slot = var_guard175_rv;
        *var_guard177_slot = var_guard177;
        *var_guard177_rv_slot = var_guard177_rv;
        *var_guard178_slot = var_guard178;
        *var_guard178_rv_slot = var_guard178_rv;
        *var_guard179_slot = var_guard179;
        *var_guard179_rv_slot = var_guard179_rv;
        *var_guard180_slot = var_guard180;
        *var_guard180_rv_slot = var_guard180_rv;
        *var_guard181_slot = var_guard181;
        *var_guard181_rv_slot = var_guard181_rv;
        *var_guard183_slot = var_guard183;
        *var_guard183_rv_slot = var_guard183_rv;
        *var_guard184_slot = var_guard184;
        *var_guard184_rv_slot = var_guard184_rv;
        *var_guard185_slot = var_guard185;
        *var_guard185_rv_slot = var_guard185_rv;
        *var_guard186_slot = var_guard186;
        *var_guard186_rv_slot = var_guard186_rv;
        *var_guard188_slot = var_guard188;
        *var_guard188_rv_slot = var_guard188_rv;
        *var_guard189_slot = var_guard189;
        *var_guard189_rv_slot = var_guard189_rv;
        *var_guard190_slot = var_guard190;
        *var_guard190_rv_slot = var_guard190_rv;
        *var_guard191_slot = var_guard191;
        *var_guard191_rv_slot = var_guard191_rv;
        *var_guard192_slot = var_guard192;
        *var_guard192_rv_slot = var_guard192_rv;
        *var_guard193_slot = var_guard193;
        *var_guard193_rv_slot = var_guard193_rv;
        *var_guard195_slot = var_guard195;
        *var_guard195_rv_slot = var_guard195_rv;
        *var_guard196_slot = var_guard196;
        *var_guard196_rv_slot = var_guard196_rv;
        *var_guard197_slot = var_guard197;
        *var_guard197_rv_slot = var_guard197_rv;
        *var_guard198_slot = var_guard198;
        *var_guard198_rv_slot = var_guard198_rv;
        *var_guard200_slot = var_guard200;
        *var_guard200_rv_slot = var_guard200_rv;
        *var_guard201_slot = var_guard201;
        *var_guard201_rv_slot = var_guard201_rv;
        *var_guard202_slot = var_guard202;
        *var_guard202_rv_slot = var_guard202_rv;
        *var_guard203_slot = var_guard203;
        *var_guard203_rv_slot = var_guard203_rv;
        *var_guard204_slot = var_guard204;
        *var_guard204_rv_slot = var_guard204_rv;
        *var_guard205_slot = var_guard205;
        *var_guard205_rv_slot = var_guard205_rv;
        *var_guard207_slot = var_guard207;
        *var_guard207_rv_slot = var_guard207_rv;
        *var_guard208_slot = var_guard208;
        *var_guard208_rv_slot = var_guard208_rv;
        *var_guard209_slot = var_guard209;
        *var_guard209_rv_slot = var_guard209_rv;
        *var_guard210_slot = var_guard210;
        *var_guard210_rv_slot = var_guard210_rv;
        *var_rend_slot = var_rend;
        *var_rend_rv_slot = var_rend_rv;
    }

    pub(super) fn stamp_reactive_block_15(
        p: &Parameters,
        var_dmcgeff: f64,
        var_dmcieff: f64,
        var_dmdgeff: f64,
        var_guard201: f64,
        var_guard202: f64,
        var_guard208: f64,
        var_guard209: f64,
        var_guard210: f64,
        var_guard78: f64,
        var_guard79: f64,
        var_guard86: f64,
        var_guard87: f64,
        var_guard88: f64,
        var_guard89: f64,
        var_guard90: f64,
        var_guard91: f64,
        var_guard92: f64,
        var_guard93: f64,
        var_guard94: f64,
        var_guard95: f64,
        var_nuendd: f64,
        var_nuends: f64,
        var_weff: f64,
        var_guard212_slot: &mut f64,
        var_guard212_rv_slot: &mut f64,
        var_guard213_slot: &mut f64,
        var_guard213_rv_slot: &mut f64,
        var_guard214_slot: &mut f64,
        var_guard214_rv_slot: &mut f64,
        var_guard215_slot: &mut f64,
        var_guard215_rv_slot: &mut f64,
        var_guard216_slot: &mut f64,
        var_guard216_rv_slot: &mut f64,
        var_guard217_slot: &mut f64,
        var_guard217_rv_slot: &mut f64,
        var_guard218_slot: &mut f64,
        var_guard218_rv_slot: &mut f64,
        var_guard220_slot: &mut f64,
        var_guard220_rv_slot: &mut f64,
        var_guard221_slot: &mut f64,
        var_guard221_rv_slot: &mut f64,
        var_guard222_slot: &mut f64,
        var_guard222_rv_slot: &mut f64,
        var_guard223_slot: &mut f64,
        var_guard223_rv_slot: &mut f64,
        var_guard225_slot: &mut f64,
        var_guard225_rv_slot: &mut f64,
        var_guard226_slot: &mut f64,
        var_guard226_rv_slot: &mut f64,
        var_guard227_slot: &mut f64,
        var_guard227_rv_slot: &mut f64,
        var_guard228_slot: &mut f64,
        var_guard228_rv_slot: &mut f64,
        var_guard229_slot: &mut f64,
        var_guard229_rv_slot: &mut f64,
        var_guard230_slot: &mut f64,
        var_guard230_rv_slot: &mut f64,
        var_guard231_slot: &mut f64,
        var_guard231_rv_slot: &mut f64,
        var_guard233_slot: &mut f64,
        var_guard233_rv_slot: &mut f64,
        var_guard234_slot: &mut f64,
        var_guard234_rv_slot: &mut f64,
        var_guard235_slot: &mut f64,
        var_guard235_rv_slot: &mut f64,
        var_guard236_slot: &mut f64,
        var_guard236_rv_slot: &mut f64,
        var_guard238_slot: &mut f64,
        var_guard238_rv_slot: &mut f64,
        var_guard239_slot: &mut f64,
        var_guard239_rv_slot: &mut f64,
        var_guard240_slot: &mut f64,
        var_guard240_rv_slot: &mut f64,
        var_rend_slot: &mut f64,
        var_rend_rv_slot: &mut f64,
        var_rint_slot: &mut f64,
        var_rint_rv_slot: &mut f64,
    ) {
        let mut var_guard212: f64 = *var_guard212_slot;
        let mut var_guard212_rv: f64 = *var_guard212_rv_slot;
        let mut var_guard213: f64 = *var_guard213_slot;
        let mut var_guard213_rv: f64 = *var_guard213_rv_slot;
        let mut var_guard214: f64 = *var_guard214_slot;
        let mut var_guard214_rv: f64 = *var_guard214_rv_slot;
        let mut var_guard215: f64 = *var_guard215_slot;
        let mut var_guard215_rv: f64 = *var_guard215_rv_slot;
        let mut var_guard216: f64 = *var_guard216_slot;
        let mut var_guard216_rv: f64 = *var_guard216_rv_slot;
        let mut var_guard217: f64 = *var_guard217_slot;
        let mut var_guard217_rv: f64 = *var_guard217_rv_slot;
        let mut var_guard218: f64 = *var_guard218_slot;
        let mut var_guard218_rv: f64 = *var_guard218_rv_slot;
        let mut var_guard220: f64 = *var_guard220_slot;
        let mut var_guard220_rv: f64 = *var_guard220_rv_slot;
        let mut var_guard221: f64 = *var_guard221_slot;
        let mut var_guard221_rv: f64 = *var_guard221_rv_slot;
        let mut var_guard222: f64 = *var_guard222_slot;
        let mut var_guard222_rv: f64 = *var_guard222_rv_slot;
        let mut var_guard223: f64 = *var_guard223_slot;
        let mut var_guard223_rv: f64 = *var_guard223_rv_slot;
        let mut var_guard225: f64 = *var_guard225_slot;
        let mut var_guard225_rv: f64 = *var_guard225_rv_slot;
        let mut var_guard226: f64 = *var_guard226_slot;
        let mut var_guard226_rv: f64 = *var_guard226_rv_slot;
        let mut var_guard227: f64 = *var_guard227_slot;
        let mut var_guard227_rv: f64 = *var_guard227_rv_slot;
        let mut var_guard228: f64 = *var_guard228_slot;
        let mut var_guard228_rv: f64 = *var_guard228_rv_slot;
        let mut var_guard229: f64 = *var_guard229_slot;
        let mut var_guard229_rv: f64 = *var_guard229_rv_slot;
        let mut var_guard230: f64 = *var_guard230_slot;
        let mut var_guard230_rv: f64 = *var_guard230_rv_slot;
        let mut var_guard231: f64 = *var_guard231_slot;
        let mut var_guard231_rv: f64 = *var_guard231_rv_slot;
        let mut var_guard233: f64 = *var_guard233_slot;
        let mut var_guard233_rv: f64 = *var_guard233_rv_slot;
        let mut var_guard234: f64 = *var_guard234_slot;
        let mut var_guard234_rv: f64 = *var_guard234_rv_slot;
        let mut var_guard235: f64 = *var_guard235_slot;
        let mut var_guard235_rv: f64 = *var_guard235_rv_slot;
        let mut var_guard236: f64 = *var_guard236_slot;
        let mut var_guard236_rv: f64 = *var_guard236_rv_slot;
        let mut var_guard238: f64 = *var_guard238_slot;
        let mut var_guard238_rv: f64 = *var_guard238_rv_slot;
        let mut var_guard239: f64 = *var_guard239_slot;
        let mut var_guard239_rv: f64 = *var_guard239_rv_slot;
        let mut var_guard240: f64 = *var_guard240_slot;
        let mut var_guard240_rv: f64 = *var_guard240_rv_slot;
        let mut var_rend: f64 = *var_rend_slot;
        let mut var_rend_rv: f64 = *var_rend_rv_slot;
        let mut var_rint: f64 = *var_rint_slot;
        let mut var_rint_rv: f64 = *var_rint_rv_slot;

        let (assign8640_e11049,) = {
    if (((((((var_guard78 == 0.0) && (var_guard79 != 0.0)) && ((var_guard91 != 0.0) && (!(((((var_guard86 != 0.0) || (var_guard87 != 0.0)) || (var_guard88 != 0.0)) || (var_guard89 != 0.0)) || (var_guard90 != 0.0))))) && (var_guard201 != 0.0)) && (var_guard202 == 0.0)) && (var_guard208 != 0.0)) && (var_guard210 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign8640_e11049;
        var_rend_rv = 0.0;

        let (assign8650_e11085,) = {
    if (((((((var_guard78 == 0.0) && (var_guard79 != 0.0)) && ((var_guard91 != 0.0) && (!(((((var_guard86 != 0.0) || (var_guard87 != 0.0)) || (var_guard88 != 0.0)) || (var_guard89 != 0.0)) || (var_guard90 != 0.0))))) && (var_guard201 != 0.0)) && (var_guard202 == 0.0)) && (var_guard208 != 0.0)) && (var_guard210 == 0.0)) {
        let assign8650_e11079: f64 = (p.p438 * var_dmcgeff);
        let assign8650_e11082: f64 = (var_weff * var_nuends);
        let assign8650_e11083: f64 = (assign8650_e11079 / assign8650_e11082);
        (assign8650_e11083,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign8650_e11085;
        var_rend_rv = 0.0;

        let assign8670_e11095: f64 = if ((var_nuends == 0.0) || (var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        var_guard212 = assign8670_e11095;
        var_guard212_rv = 0.0;

        let (assign8680_e11127,) = {
    if (((((((var_guard78 == 0.0) && (var_guard79 != 0.0)) && ((var_guard91 != 0.0) && (!(((((var_guard86 != 0.0) || (var_guard87 != 0.0)) || (var_guard88 != 0.0)) || (var_guard89 != 0.0)) || (var_guard90 != 0.0))))) && (var_guard201 != 0.0)) && (var_guard202 == 0.0)) && ((var_guard209 != 0.0) && (var_guard208 == 0.0))) && (var_guard212 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign8680_e11127;
        var_rend_rv = 0.0;

        let (assign8690_e11168,) = {
    if (((((((var_guard78 == 0.0) && (var_guard79 != 0.0)) && ((var_guard91 != 0.0) && (!(((((var_guard86 != 0.0) || (var_guard87 != 0.0)) || (var_guard88 != 0.0)) || (var_guard89 != 0.0)) || (var_guard90 != 0.0))))) && (var_guard201 != 0.0)) && (var_guard202 == 0.0)) && ((var_guard209 != 0.0) && (var_guard208 == 0.0))) && (var_guard212 == 0.0)) {
        let assign8690_e11160: f64 = (p.p438 * var_weff);
        let assign8690_e11163: f64 = (6.0 * var_nuends);
        let assign8690_e11165: f64 = (assign8690_e11163 * var_dmcgeff);
        let assign8690_e11166: f64 = (assign8690_e11160 / assign8690_e11165);
        (assign8690_e11166,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign8690_e11168;
        var_rend_rv = 0.0;

        let (assign8700_e11198,) = {
    if ((((((var_guard78 == 0.0) && (var_guard79 != 0.0)) && ((var_guard91 != 0.0) && (!(((((var_guard86 != 0.0) || (var_guard87 != 0.0)) || (var_guard88 != 0.0)) || (var_guard89 != 0.0)) || (var_guard90 != 0.0))))) && (var_guard201 != 0.0)) && (var_guard202 == 0.0)) && (!((var_guard208 != 0.0) || (var_guard209 != 0.0)))) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign8700_e11198;
        var_rend_rv = 0.0;

        let assign8710_e11201: f64 = if var_nuendd == 0.0 { 1.0 } else { 0.0 };
        var_guard213 = assign8710_e11201;
        var_guard213_rv = 0.0;

        let (assign8720_e11226,) = {
    if (((((var_guard78 == 0.0) && (var_guard79 != 0.0)) && ((var_guard91 != 0.0) && (!(((((var_guard86 != 0.0) || (var_guard87 != 0.0)) || (var_guard88 != 0.0)) || (var_guard89 != 0.0)) || (var_guard90 != 0.0))))) && (var_guard201 == 0.0)) && (var_guard213 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign8720_e11226;
        var_rend_rv = 0.0;

        let (assign8730_e11258,) = {
    if (((((var_guard78 == 0.0) && (var_guard79 != 0.0)) && ((var_guard91 != 0.0) && (!(((((var_guard86 != 0.0) || (var_guard87 != 0.0)) || (var_guard88 != 0.0)) || (var_guard89 != 0.0)) || (var_guard90 != 0.0))))) && (var_guard201 == 0.0)) && (var_guard213 == 0.0)) {
        let assign8730_e11252: f64 = (p.p438 * var_dmdgeff);
        let assign8730_e11255: f64 = (var_weff * var_nuendd);
        let assign8730_e11256: f64 = (assign8730_e11252 / assign8730_e11255);
        (assign8730_e11256,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign8730_e11258;
        var_rend_rv = 0.0;

        let assign8740_e11261: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        var_guard214 = assign8740_e11261;
        var_guard214_rv = 0.0;

        let (assign8750_e11289,) = {
    if ((((var_guard78 == 0.0) && (var_guard79 != 0.0)) && ((var_guard92 != 0.0) && (!((((((var_guard86 != 0.0) || (var_guard87 != 0.0)) || (var_guard88 != 0.0)) || (var_guard89 != 0.0)) || (var_guard90 != 0.0)) || (var_guard91 != 0.0))))) && (var_guard214 != 0.0)) {
        let assign8750_e11285: f64 = (p.p438 * var_dmdgeff);
        let assign8750_e11287: f64 = (assign8750_e11285 / var_weff);
        (assign8750_e11287,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign8750_e11289;
        var_rend_rv = 0.0;

        let assign8760_e11292: f64 = if 0.0 == 1.0 { 1.0 } else { 0.0 };
        var_guard215 = assign8760_e11292;
        var_guard215_rv = 0.0;

        let assign8770_e11303: f64 = if (((p.p9 == 1.0) || (p.p9 == 2.0)) || (p.p9 == 5.0)) { 1.0 } else { 0.0 };
        var_guard216 = assign8770_e11303;
        var_guard216_rv = 0.0;

        let assign8780_e11314: f64 = if (((p.p9 == 3.0) || (p.p9 == 4.0)) || (p.p9 == 6.0)) { 1.0 } else { 0.0 };
        var_guard217 = assign8780_e11314;
        var_guard217_rv = 0.0;

        let assign8790_e11317: f64 = if var_nuendd == 0.0 { 1.0 } else { 0.0 };
        var_guard218 = assign8790_e11317;
        var_guard218_rv = 0.0;

        let (assign8800_e11348,) = {
    if (((((((var_guard78 == 0.0) && (var_guard79 != 0.0)) && ((var_guard92 != 0.0) && (!((((((var_guard86 != 0.0) || (var_guard87 != 0.0)) || (var_guard88 != 0.0)) || (var_guard89 != 0.0)) || (var_guard90 != 0.0)) || (var_guard91 != 0.0))))) && (var_guard214 == 0.0)) && (var_guard215 != 0.0)) && (var_guard216 != 0.0)) && (var_guard218 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign8800_e11348;
        var_rend_rv = 0.0;

        let (assign8810_e11386,) = {
    if (((((((var_guard78 == 0.0) && (var_guard79 != 0.0)) && ((var_guard92 != 0.0) && (!((((((var_guard86 != 0.0) || (var_guard87 != 0.0)) || (var_guard88 != 0.0)) || (var_guard89 != 0.0)) || (var_guard90 != 0.0)) || (var_guard91 != 0.0))))) && (var_guard214 == 0.0)) && (var_guard215 != 0.0)) && (var_guard216 != 0.0)) && (var_guard218 == 0.0)) {
        let assign8810_e11380: f64 = (p.p438 * var_dmcgeff);
        let assign8810_e11383: f64 = (var_weff * var_nuendd);
        let assign8810_e11384: f64 = (assign8810_e11380 / assign8810_e11383);
        (assign8810_e11384,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign8810_e11386;
        var_rend_rv = 0.0;

        let assign8830_e11397: f64 = (var_dmcgeff + var_dmcieff);
        let assign8830_e11400: f64 = if ((var_nuendd == 0.0) || (assign8830_e11397 == 0.0)) { 1.0 } else { 0.0 };
        var_guard220 = assign8830_e11400;
        var_guard220_rv = 0.0;

        let (assign8840_e11434,) = {
    if (((((((var_guard78 == 0.0) && (var_guard79 != 0.0)) && ((var_guard92 != 0.0) && (!((((((var_guard86 != 0.0) || (var_guard87 != 0.0)) || (var_guard88 != 0.0)) || (var_guard89 != 0.0)) || (var_guard90 != 0.0)) || (var_guard91 != 0.0))))) && (var_guard214 == 0.0)) && (var_guard215 != 0.0)) && ((var_guard217 != 0.0) && (var_guard216 == 0.0))) && (var_guard220 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign8840_e11434;
        var_rend_rv = 0.0;

        let (assign8850_e11479,) = {
    if (((((((var_guard78 == 0.0) && (var_guard79 != 0.0)) && ((var_guard92 != 0.0) && (!((((((var_guard86 != 0.0) || (var_guard87 != 0.0)) || (var_guard88 != 0.0)) || (var_guard89 != 0.0)) || (var_guard90 != 0.0)) || (var_guard91 != 0.0))))) && (var_guard214 == 0.0)) && (var_guard215 != 0.0)) && ((var_guard217 != 0.0) && (var_guard216 == 0.0))) && (var_guard220 == 0.0)) {
        let assign8850_e11469: f64 = (p.p438 * var_weff);
        let assign8850_e11472: f64 = (3.0 * var_nuendd);
        let assign8850_e11475: f64 = (var_dmcgeff + var_dmcieff);
        let assign8850_e11476: f64 = (assign8850_e11472 * assign8850_e11475);
        let assign8850_e11477: f64 = (assign8850_e11469 / assign8850_e11476);
        (assign8850_e11477,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign8850_e11479;
        var_rend_rv = 0.0;

        let (assign8860_e11511,) = {
    if ((((((var_guard78 == 0.0) && (var_guard79 != 0.0)) && ((var_guard92 != 0.0) && (!((((((var_guard86 != 0.0) || (var_guard87 != 0.0)) || (var_guard88 != 0.0)) || (var_guard89 != 0.0)) || (var_guard90 != 0.0)) || (var_guard91 != 0.0))))) && (var_guard214 == 0.0)) && (var_guard215 != 0.0)) && (!((var_guard216 != 0.0) || (var_guard217 != 0.0)))) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign8860_e11511;
        var_rend_rv = 0.0;

        let assign8870_e11522: f64 = if (((p.p9 == 1.0) || (p.p9 == 3.0)) || (p.p9 == 7.0)) { 1.0 } else { 0.0 };
        var_guard221 = assign8870_e11522;
        var_guard221_rv = 0.0;

        let assign8880_e11533: f64 = if (((p.p9 == 2.0) || (p.p9 == 4.0)) || (p.p9 == 8.0)) { 1.0 } else { 0.0 };
        var_guard222 = assign8880_e11533;
        var_guard222_rv = 0.0;

        let assign8890_e11536: f64 = if var_nuendd == 0.0 { 1.0 } else { 0.0 };
        var_guard223 = assign8890_e11536;
        var_guard223_rv = 0.0;

        let (assign8900_e11568,) = {
    if (((((((var_guard78 == 0.0) && (var_guard79 != 0.0)) && ((var_guard92 != 0.0) && (!((((((var_guard86 != 0.0) || (var_guard87 != 0.0)) || (var_guard88 != 0.0)) || (var_guard89 != 0.0)) || (var_guard90 != 0.0)) || (var_guard91 != 0.0))))) && (var_guard214 == 0.0)) && (var_guard215 == 0.0)) && (var_guard221 != 0.0)) && (var_guard223 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign8900_e11568;
        var_rend_rv = 0.0;

        let (assign8910_e11607,) = {
    if (((((((var_guard78 == 0.0) && (var_guard79 != 0.0)) && ((var_guard92 != 0.0) && (!((((((var_guard86 != 0.0) || (var_guard87 != 0.0)) || (var_guard88 != 0.0)) || (var_guard89 != 0.0)) || (var_guard90 != 0.0)) || (var_guard91 != 0.0))))) && (var_guard214 == 0.0)) && (var_guard215 == 0.0)) && (var_guard221 != 0.0)) && (var_guard223 == 0.0)) {
        let assign8910_e11601: f64 = (p.p438 * var_dmcgeff);
        let assign8910_e11604: f64 = (var_weff * var_nuendd);
        let assign8910_e11605: f64 = (assign8910_e11601 / assign8910_e11604);
        (assign8910_e11605,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign8910_e11607;
        var_rend_rv = 0.0;

        let assign8930_e11618: f64 = (var_dmcgeff + var_dmcieff);
        let assign8930_e11621: f64 = if ((var_nuendd == 0.0) || (assign8930_e11618 == 0.0)) { 1.0 } else { 0.0 };
        var_guard225 = assign8930_e11621;
        var_guard225_rv = 0.0;

        let (assign8940_e11656,) = {
    if (((((((var_guard78 == 0.0) && (var_guard79 != 0.0)) && ((var_guard92 != 0.0) && (!((((((var_guard86 != 0.0) || (var_guard87 != 0.0)) || (var_guard88 != 0.0)) || (var_guard89 != 0.0)) || (var_guard90 != 0.0)) || (var_guard91 != 0.0))))) && (var_guard214 == 0.0)) && (var_guard215 == 0.0)) && ((var_guard222 != 0.0) && (var_guard221 == 0.0))) && (var_guard225 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign8940_e11656;
        var_rend_rv = 0.0;

        let (assign8950_e11702,) = {
    if (((((((var_guard78 == 0.0) && (var_guard79 != 0.0)) && ((var_guard92 != 0.0) && (!((((((var_guard86 != 0.0) || (var_guard87 != 0.0)) || (var_guard88 != 0.0)) || (var_guard89 != 0.0)) || (var_guard90 != 0.0)) || (var_guard91 != 0.0))))) && (var_guard214 == 0.0)) && (var_guard215 == 0.0)) && ((var_guard222 != 0.0) && (var_guard221 == 0.0))) && (var_guard225 == 0.0)) {
        let assign8950_e11692: f64 = (p.p438 * var_weff);
        let assign8950_e11695: f64 = (3.0 * var_nuendd);
        let assign8950_e11698: f64 = (var_dmcgeff + var_dmcieff);
        let assign8950_e11699: f64 = (assign8950_e11695 * assign8950_e11698);
        let assign8950_e11700: f64 = (assign8950_e11692 / assign8950_e11699);
        (assign8950_e11700,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign8950_e11702;
        var_rend_rv = 0.0;

        let (assign8960_e11735,) = {
    if ((((((var_guard78 == 0.0) && (var_guard79 != 0.0)) && ((var_guard92 != 0.0) && (!((((((var_guard86 != 0.0) || (var_guard87 != 0.0)) || (var_guard88 != 0.0)) || (var_guard89 != 0.0)) || (var_guard90 != 0.0)) || (var_guard91 != 0.0))))) && (var_guard214 == 0.0)) && (var_guard215 == 0.0)) && (!((var_guard221 != 0.0) || (var_guard222 != 0.0)))) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign8960_e11735;
        var_rend_rv = 0.0;

        let assign8970_e11738: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        var_guard226 = assign8970_e11738;
        var_guard226_rv = 0.0;

        let assign8980_e11741: f64 = if var_nuends == 0.0 { 1.0 } else { 0.0 };
        var_guard227 = assign8980_e11741;
        var_guard227_rv = 0.0;

        let (assign8990_e11769,) = {
    if (((((var_guard78 == 0.0) && (var_guard79 != 0.0)) && ((var_guard93 != 0.0) && (!(((((((var_guard86 != 0.0) || (var_guard87 != 0.0)) || (var_guard88 != 0.0)) || (var_guard89 != 0.0)) || (var_guard90 != 0.0)) || (var_guard91 != 0.0)) || (var_guard92 != 0.0))))) && (var_guard226 != 0.0)) && (var_guard227 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign8990_e11769;
        var_rend_rv = 0.0;

        let (assign9000_e11804,) = {
    if (((((var_guard78 == 0.0) && (var_guard79 != 0.0)) && ((var_guard93 != 0.0) && (!(((((((var_guard86 != 0.0) || (var_guard87 != 0.0)) || (var_guard88 != 0.0)) || (var_guard89 != 0.0)) || (var_guard90 != 0.0)) || (var_guard91 != 0.0)) || (var_guard92 != 0.0))))) && (var_guard226 != 0.0)) && (var_guard227 == 0.0)) {
        let assign9000_e11798: f64 = (p.p438 * var_dmdgeff);
        let assign9000_e11801: f64 = (var_weff * var_nuends);
        let assign9000_e11802: f64 = (assign9000_e11798 / assign9000_e11801);
        (assign9000_e11802,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign9000_e11804;
        var_rend_rv = 0.0;

        let assign9010_e11807: f64 = if 0.0 == 1.0 { 1.0 } else { 0.0 };
        var_guard228 = assign9010_e11807;
        var_guard228_rv = 0.0;

        let assign9020_e11818: f64 = if (((p.p9 == 1.0) || (p.p9 == 2.0)) || (p.p9 == 5.0)) { 1.0 } else { 0.0 };
        var_guard229 = assign9020_e11818;
        var_guard229_rv = 0.0;

        let assign9030_e11829: f64 = if (((p.p9 == 3.0) || (p.p9 == 4.0)) || (p.p9 == 6.0)) { 1.0 } else { 0.0 };
        var_guard230 = assign9030_e11829;
        var_guard230_rv = 0.0;

        let assign9040_e11832: f64 = if var_nuendd == 0.0 { 1.0 } else { 0.0 };
        var_guard231 = assign9040_e11832;
        var_guard231_rv = 0.0;

        let (assign9050_e11865,) = {
    if (((((((var_guard78 == 0.0) && (var_guard79 != 0.0)) && ((var_guard93 != 0.0) && (!(((((((var_guard86 != 0.0) || (var_guard87 != 0.0)) || (var_guard88 != 0.0)) || (var_guard89 != 0.0)) || (var_guard90 != 0.0)) || (var_guard91 != 0.0)) || (var_guard92 != 0.0))))) && (var_guard226 == 0.0)) && (var_guard228 != 0.0)) && (var_guard229 != 0.0)) && (var_guard231 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign9050_e11865;
        var_rend_rv = 0.0;

        let (assign9060_e11905,) = {
    if (((((((var_guard78 == 0.0) && (var_guard79 != 0.0)) && ((var_guard93 != 0.0) && (!(((((((var_guard86 != 0.0) || (var_guard87 != 0.0)) || (var_guard88 != 0.0)) || (var_guard89 != 0.0)) || (var_guard90 != 0.0)) || (var_guard91 != 0.0)) || (var_guard92 != 0.0))))) && (var_guard226 == 0.0)) && (var_guard228 != 0.0)) && (var_guard229 != 0.0)) && (var_guard231 == 0.0)) {
        let assign9060_e11899: f64 = (p.p438 * var_dmcgeff);
        let assign9060_e11902: f64 = (var_weff * var_nuendd);
        let assign9060_e11903: f64 = (assign9060_e11899 / assign9060_e11902);
        (assign9060_e11903,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign9060_e11905;
        var_rend_rv = 0.0;

        let assign9080_e11915: f64 = if ((var_nuendd == 0.0) || (var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        var_guard233 = assign9080_e11915;
        var_guard233_rv = 0.0;

        let (assign9090_e11951,) = {
    if (((((((var_guard78 == 0.0) && (var_guard79 != 0.0)) && ((var_guard93 != 0.0) && (!(((((((var_guard86 != 0.0) || (var_guard87 != 0.0)) || (var_guard88 != 0.0)) || (var_guard89 != 0.0)) || (var_guard90 != 0.0)) || (var_guard91 != 0.0)) || (var_guard92 != 0.0))))) && (var_guard226 == 0.0)) && (var_guard228 != 0.0)) && ((var_guard230 != 0.0) && (var_guard229 == 0.0))) && (var_guard233 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign9090_e11951;
        var_rend_rv = 0.0;

        let (assign9100_e11996,) = {
    if (((((((var_guard78 == 0.0) && (var_guard79 != 0.0)) && ((var_guard93 != 0.0) && (!(((((((var_guard86 != 0.0) || (var_guard87 != 0.0)) || (var_guard88 != 0.0)) || (var_guard89 != 0.0)) || (var_guard90 != 0.0)) || (var_guard91 != 0.0)) || (var_guard92 != 0.0))))) && (var_guard226 == 0.0)) && (var_guard228 != 0.0)) && ((var_guard230 != 0.0) && (var_guard229 == 0.0))) && (var_guard233 == 0.0)) {
        let assign9100_e11988: f64 = (p.p438 * var_weff);
        let assign9100_e11991: f64 = (6.0 * var_nuendd);
        let assign9100_e11993: f64 = (assign9100_e11991 * var_dmcgeff);
        let assign9100_e11994: f64 = (assign9100_e11988 / assign9100_e11993);
        (assign9100_e11994,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign9100_e11996;
        var_rend_rv = 0.0;

        let (assign9110_e12030,) = {
    if ((((((var_guard78 == 0.0) && (var_guard79 != 0.0)) && ((var_guard93 != 0.0) && (!(((((((var_guard86 != 0.0) || (var_guard87 != 0.0)) || (var_guard88 != 0.0)) || (var_guard89 != 0.0)) || (var_guard90 != 0.0)) || (var_guard91 != 0.0)) || (var_guard92 != 0.0))))) && (var_guard226 == 0.0)) && (var_guard228 != 0.0)) && (!((var_guard229 != 0.0) || (var_guard230 != 0.0)))) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign9110_e12030;
        var_rend_rv = 0.0;

        let assign9120_e12041: f64 = if (((p.p9 == 1.0) || (p.p9 == 3.0)) || (p.p9 == 7.0)) { 1.0 } else { 0.0 };
        var_guard234 = assign9120_e12041;
        var_guard234_rv = 0.0;

        let assign9130_e12052: f64 = if (((p.p9 == 2.0) || (p.p9 == 4.0)) || (p.p9 == 8.0)) { 1.0 } else { 0.0 };
        var_guard235 = assign9130_e12052;
        var_guard235_rv = 0.0;

        let assign9140_e12055: f64 = if var_nuendd == 0.0 { 1.0 } else { 0.0 };
        var_guard236 = assign9140_e12055;
        var_guard236_rv = 0.0;

        let (assign9150_e12089,) = {
    if (((((((var_guard78 == 0.0) && (var_guard79 != 0.0)) && ((var_guard93 != 0.0) && (!(((((((var_guard86 != 0.0) || (var_guard87 != 0.0)) || (var_guard88 != 0.0)) || (var_guard89 != 0.0)) || (var_guard90 != 0.0)) || (var_guard91 != 0.0)) || (var_guard92 != 0.0))))) && (var_guard226 == 0.0)) && (var_guard228 == 0.0)) && (var_guard234 != 0.0)) && (var_guard236 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign9150_e12089;
        var_rend_rv = 0.0;

        let (assign9160_e12130,) = {
    if (((((((var_guard78 == 0.0) && (var_guard79 != 0.0)) && ((var_guard93 != 0.0) && (!(((((((var_guard86 != 0.0) || (var_guard87 != 0.0)) || (var_guard88 != 0.0)) || (var_guard89 != 0.0)) || (var_guard90 != 0.0)) || (var_guard91 != 0.0)) || (var_guard92 != 0.0))))) && (var_guard226 == 0.0)) && (var_guard228 == 0.0)) && (var_guard234 != 0.0)) && (var_guard236 == 0.0)) {
        let assign9160_e12124: f64 = (p.p438 * var_dmcgeff);
        let assign9160_e12127: f64 = (var_weff * var_nuendd);
        let assign9160_e12128: f64 = (assign9160_e12124 / assign9160_e12127);
        (assign9160_e12128,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign9160_e12130;
        var_rend_rv = 0.0;

        let assign9180_e12140: f64 = if ((var_nuendd == 0.0) || (var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        var_guard238 = assign9180_e12140;
        var_guard238_rv = 0.0;

        let (assign9190_e12177,) = {
    if (((((((var_guard78 == 0.0) && (var_guard79 != 0.0)) && ((var_guard93 != 0.0) && (!(((((((var_guard86 != 0.0) || (var_guard87 != 0.0)) || (var_guard88 != 0.0)) || (var_guard89 != 0.0)) || (var_guard90 != 0.0)) || (var_guard91 != 0.0)) || (var_guard92 != 0.0))))) && (var_guard226 == 0.0)) && (var_guard228 == 0.0)) && ((var_guard235 != 0.0) && (var_guard234 == 0.0))) && (var_guard238 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign9190_e12177;
        var_rend_rv = 0.0;

        let (assign9200_e12223,) = {
    if (((((((var_guard78 == 0.0) && (var_guard79 != 0.0)) && ((var_guard93 != 0.0) && (!(((((((var_guard86 != 0.0) || (var_guard87 != 0.0)) || (var_guard88 != 0.0)) || (var_guard89 != 0.0)) || (var_guard90 != 0.0)) || (var_guard91 != 0.0)) || (var_guard92 != 0.0))))) && (var_guard226 == 0.0)) && (var_guard228 == 0.0)) && ((var_guard235 != 0.0) && (var_guard234 == 0.0))) && (var_guard238 == 0.0)) {
        let assign9200_e12215: f64 = (p.p438 * var_weff);
        let assign9200_e12218: f64 = (6.0 * var_nuendd);
        let assign9200_e12220: f64 = (assign9200_e12218 * var_dmcgeff);
        let assign9200_e12221: f64 = (assign9200_e12215 / assign9200_e12220);
        (assign9200_e12221,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign9200_e12223;
        var_rend_rv = 0.0;

        let (assign9210_e12258,) = {
    if ((((((var_guard78 == 0.0) && (var_guard79 != 0.0)) && ((var_guard93 != 0.0) && (!(((((((var_guard86 != 0.0) || (var_guard87 != 0.0)) || (var_guard88 != 0.0)) || (var_guard89 != 0.0)) || (var_guard90 != 0.0)) || (var_guard91 != 0.0)) || (var_guard92 != 0.0))))) && (var_guard226 == 0.0)) && (var_guard228 == 0.0)) && (!((var_guard234 != 0.0) || (var_guard235 != 0.0)))) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign9210_e12258;
        var_rend_rv = 0.0;

        let (assign9220_e12288,) = {
    if (((var_guard78 == 0.0) && (var_guard79 != 0.0)) && ((var_guard94 != 0.0) && (!((((((((var_guard86 != 0.0) || (var_guard87 != 0.0)) || (var_guard88 != 0.0)) || (var_guard89 != 0.0)) || (var_guard90 != 0.0)) || (var_guard91 != 0.0)) || (var_guard92 != 0.0)) || (var_guard93 != 0.0))))) {
        let assign9220_e12284: f64 = (p.p438 * var_dmdgeff);
        let assign9220_e12286: f64 = (assign9220_e12284 / var_weff);
        (assign9220_e12286,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign9220_e12288;
        var_rend_rv = 0.0;

        let assign9230_e12291: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        var_guard239 = assign9230_e12291;
        var_guard239_rv = 0.0;

        let (assign9240_e12327,) = {
    if ((((var_guard78 == 0.0) && (var_guard79 != 0.0)) && ((var_guard95 != 0.0) && (!(((((((((var_guard86 != 0.0) || (var_guard87 != 0.0)) || (var_guard88 != 0.0)) || (var_guard89 != 0.0)) || (var_guard90 != 0.0)) || (var_guard91 != 0.0)) || (var_guard92 != 0.0)) || (var_guard93 != 0.0)) || (var_guard94 != 0.0))))) && (var_guard239 != 0.0)) {
        let assign9240_e12321: f64 = (0.5 * p.p438);
        let assign9240_e12323: f64 = (assign9240_e12321 * var_dmcgeff);
        let assign9240_e12325: f64 = (assign9240_e12323 / var_weff);
        (assign9240_e12325,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign9240_e12327;
        var_rend_rv = 0.0;

        let assign9250_e12330: f64 = if p.p2 == 2.0 { 1.0 } else { 0.0 };
        var_guard240 = assign9250_e12330;
        var_guard240_rv = 0.0;

        let (assign9260_e12362,) = {
    if (((((var_guard78 == 0.0) && (var_guard79 != 0.0)) && ((var_guard95 != 0.0) && (!(((((((((var_guard86 != 0.0) || (var_guard87 != 0.0)) || (var_guard88 != 0.0)) || (var_guard89 != 0.0)) || (var_guard90 != 0.0)) || (var_guard91 != 0.0)) || (var_guard92 != 0.0)) || (var_guard93 != 0.0)) || (var_guard94 != 0.0))))) && (var_guard239 != 0.0)) && (var_guard240 != 0.0)) {
        (0.0,)
    } else {
        (var_rint,)
    }
};
        var_rint = assign9260_e12362;
        var_rint_rv = 0.0;

        let (assign9270_e12403,) = {
    if (((((var_guard78 == 0.0) && (var_guard79 != 0.0)) && ((var_guard95 != 0.0) && (!(((((((((var_guard86 != 0.0) || (var_guard87 != 0.0)) || (var_guard88 != 0.0)) || (var_guard89 != 0.0)) || (var_guard90 != 0.0)) || (var_guard91 != 0.0)) || (var_guard92 != 0.0)) || (var_guard93 != 0.0)) || (var_guard94 != 0.0))))) && (var_guard239 != 0.0)) && (var_guard240 == 0.0)) {
        let assign9270_e12395: f64 = (p.p438 * var_dmcgeff);
        let assign9270_e12399: f64 = (p.p2 - 2.0);
        let assign9270_e12400: f64 = (var_weff * assign9270_e12399);
        let assign9270_e12401: f64 = (assign9270_e12395 / assign9270_e12400);
        (assign9270_e12401,)
    } else {
        (var_rint,)
    }
};
        var_rint = assign9270_e12403;
        var_rint_rv = 0.0;

        let (assign9280_e12434,) = {
    if ((((var_guard78 == 0.0) && (var_guard79 != 0.0)) && ((var_guard95 != 0.0) && (!(((((((((var_guard86 != 0.0) || (var_guard87 != 0.0)) || (var_guard88 != 0.0)) || (var_guard89 != 0.0)) || (var_guard90 != 0.0)) || (var_guard91 != 0.0)) || (var_guard92 != 0.0)) || (var_guard93 != 0.0)) || (var_guard94 != 0.0))))) && (var_guard239 == 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign9280_e12434;
        var_rend_rv = 0.0;

        *var_guard212_slot = var_guard212;
        *var_guard212_rv_slot = var_guard212_rv;
        *var_guard213_slot = var_guard213;
        *var_guard213_rv_slot = var_guard213_rv;
        *var_guard214_slot = var_guard214;
        *var_guard214_rv_slot = var_guard214_rv;
        *var_guard215_slot = var_guard215;
        *var_guard215_rv_slot = var_guard215_rv;
        *var_guard216_slot = var_guard216;
        *var_guard216_rv_slot = var_guard216_rv;
        *var_guard217_slot = var_guard217;
        *var_guard217_rv_slot = var_guard217_rv;
        *var_guard218_slot = var_guard218;
        *var_guard218_rv_slot = var_guard218_rv;
        *var_guard220_slot = var_guard220;
        *var_guard220_rv_slot = var_guard220_rv;
        *var_guard221_slot = var_guard221;
        *var_guard221_rv_slot = var_guard221_rv;
        *var_guard222_slot = var_guard222;
        *var_guard222_rv_slot = var_guard222_rv;
        *var_guard223_slot = var_guard223;
        *var_guard223_rv_slot = var_guard223_rv;
        *var_guard225_slot = var_guard225;
        *var_guard225_rv_slot = var_guard225_rv;
        *var_guard226_slot = var_guard226;
        *var_guard226_rv_slot = var_guard226_rv;
        *var_guard227_slot = var_guard227;
        *var_guard227_rv_slot = var_guard227_rv;
        *var_guard228_slot = var_guard228;
        *var_guard228_rv_slot = var_guard228_rv;
        *var_guard229_slot = var_guard229;
        *var_guard229_rv_slot = var_guard229_rv;
        *var_guard230_slot = var_guard230;
        *var_guard230_rv_slot = var_guard230_rv;
        *var_guard231_slot = var_guard231;
        *var_guard231_rv_slot = var_guard231_rv;
        *var_guard233_slot = var_guard233;
        *var_guard233_rv_slot = var_guard233_rv;
        *var_guard234_slot = var_guard234;
        *var_guard234_rv_slot = var_guard234_rv;
        *var_guard235_slot = var_guard235;
        *var_guard235_rv_slot = var_guard235_rv;
        *var_guard236_slot = var_guard236;
        *var_guard236_rv_slot = var_guard236_rv;
        *var_guard238_slot = var_guard238;
        *var_guard238_rv_slot = var_guard238_rv;
        *var_guard239_slot = var_guard239;
        *var_guard239_rv_slot = var_guard239_rv;
        *var_guard240_slot = var_guard240;
        *var_guard240_rv_slot = var_guard240_rv;
        *var_rend_slot = var_rend;
        *var_rend_rv_slot = var_rend_rv;
        *var_rint_slot = var_rint;
        *var_rint_rv_slot = var_rint_rv;
    }

    pub(super) fn stamp_reactive_block_16(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        var_dmcgeff: f64,
        var_dmcieff: f64,
        var_guard239: f64,
        var_guard78: f64,
        var_guard79: f64,
        var_guard86: f64,
        var_guard87: f64,
        var_guard88: f64,
        var_guard89: f64,
        var_guard90: f64,
        var_guard91: f64,
        var_guard92: f64,
        var_guard93: f64,
        var_guard94: f64,
        var_guard95: f64,
        var_guard96: f64,
        var_weff: f64,
        var_guard241_slot: &mut f64,
        var_guard241_rv_slot: &mut f64,
        var_guard242_slot: &mut f64,
        var_guard242_rv_slot: &mut f64,
        var_guard243_slot: &mut f64,
        var_guard243_rv_slot: &mut f64,
        var_guard244_slot: &mut f64,
        var_guard244_rv_slot: &mut f64,
        var_guard246_slot: &mut f64,
        var_guard246_rv_slot: &mut f64,
        var_guard247_slot: &mut f64,
        var_guard247_rv_slot: &mut f64,
        var_guard248_slot: &mut f64,
        var_guard248_rv_slot: &mut f64,
        var_guard249_slot: &mut f64,
        var_guard249_rv_slot: &mut f64,
        var_guard250_slot: &mut f64,
        var_guard250_rv_slot: &mut f64,
        var_guard251_slot: &mut f64,
        var_guard251_rv_slot: &mut f64,
        var_guard252_slot: &mut f64,
        var_guard252_rv_slot: &mut f64,
        var_guard253_slot: &mut f64,
        var_guard253_rv_slot: &mut f64,
        var_guard254_slot: &mut f64,
        var_guard254_rv_slot: &mut f64,
        var_guard255_slot: &mut f64,
        var_guard255_rv_slot: &mut f64,
        var_guard256_slot: &mut f64,
        var_guard256_rv_slot: &mut f64,
        var_guard257_slot: &mut f64,
        var_guard257_rv_slot: &mut f64,
        var_guard258_slot: &mut f64,
        var_guard258_rv_slot: &mut f64,
        var_guard259_slot: &mut f64,
        var_guard259_rv_slot: &mut f64,
        var_guard260_slot: &mut f64,
        var_guard260_rv_slot: &mut f64,
        var_guard261_slot: &mut f64,
        var_guard261_rv_slot: &mut f64,
        var_guard262_slot: &mut f64,
        var_guard262_rv_slot: &mut f64,
        var_guard263_slot: &mut f64,
        var_guard263_rv_slot: &mut f64,
        var_guard264_slot: &mut f64,
        var_guard264_rv_slot: &mut f64,
        var_guard265_slot: &mut f64,
        var_guard265_rv_slot: &mut f64,
        var_guard266_slot: &mut f64,
        var_guard266_rv_slot: &mut f64,
        var_guard267_slot: &mut f64,
        var_guard267_rv_slot: &mut f64,
        var_guard268_slot: &mut f64,
        var_guard268_rv_slot: &mut f64,
        var_guard269_slot: &mut f64,
        var_guard269_rv_slot: &mut f64,
        var_guard271_slot: &mut f64,
        var_guard271_rv_slot: &mut f64,
        var_guard272_slot: &mut f64,
        var_guard272_rv_slot: &mut f64,
        var_guard273_slot: &mut f64,
        var_guard273_rv_slot: &mut f64,
        var_guard274_slot: &mut f64,
        var_guard274_rv_slot: &mut f64,
        var_nuendd_slot: &mut f64,
        var_nuendd_rv_slot: &mut f64,
        var_nuends_slot: &mut f64,
        var_nuends_rv_slot: &mut f64,
        var_nuintd_slot: &mut f64,
        var_nuintd_rv_slot: &mut f64,
        var_nuints_slot: &mut f64,
        var_nuints_rv_slot: &mut f64,
        var_rdraingeo_slot: &mut f64,
        var_rdraingeo_rv_slot: &mut f64,
        var_rend_slot: &mut f64,
        var_rend_rv_slot: &mut f64,
        var_rint_slot: &mut f64,
        var_rint_rv_slot: &mut f64,
        var_rsourcegeo_slot: &mut f64,
        var_rsourcegeo_rv_slot: &mut f64,
    ) {
        let mut var_guard241: f64 = *var_guard241_slot;
        let mut var_guard241_rv: f64 = *var_guard241_rv_slot;
        let mut var_guard242: f64 = *var_guard242_slot;
        let mut var_guard242_rv: f64 = *var_guard242_rv_slot;
        let mut var_guard243: f64 = *var_guard243_slot;
        let mut var_guard243_rv: f64 = *var_guard243_rv_slot;
        let mut var_guard244: f64 = *var_guard244_slot;
        let mut var_guard244_rv: f64 = *var_guard244_rv_slot;
        let mut var_guard246: f64 = *var_guard246_slot;
        let mut var_guard246_rv: f64 = *var_guard246_rv_slot;
        let mut var_guard247: f64 = *var_guard247_slot;
        let mut var_guard247_rv: f64 = *var_guard247_rv_slot;
        let mut var_guard248: f64 = *var_guard248_slot;
        let mut var_guard248_rv: f64 = *var_guard248_rv_slot;
        let mut var_guard249: f64 = *var_guard249_slot;
        let mut var_guard249_rv: f64 = *var_guard249_rv_slot;
        let mut var_guard250: f64 = *var_guard250_slot;
        let mut var_guard250_rv: f64 = *var_guard250_rv_slot;
        let mut var_guard251: f64 = *var_guard251_slot;
        let mut var_guard251_rv: f64 = *var_guard251_rv_slot;
        let mut var_guard252: f64 = *var_guard252_slot;
        let mut var_guard252_rv: f64 = *var_guard252_rv_slot;
        let mut var_guard253: f64 = *var_guard253_slot;
        let mut var_guard253_rv: f64 = *var_guard253_rv_slot;
        let mut var_guard254: f64 = *var_guard254_slot;
        let mut var_guard254_rv: f64 = *var_guard254_rv_slot;
        let mut var_guard255: f64 = *var_guard255_slot;
        let mut var_guard255_rv: f64 = *var_guard255_rv_slot;
        let mut var_guard256: f64 = *var_guard256_slot;
        let mut var_guard256_rv: f64 = *var_guard256_rv_slot;
        let mut var_guard257: f64 = *var_guard257_slot;
        let mut var_guard257_rv: f64 = *var_guard257_rv_slot;
        let mut var_guard258: f64 = *var_guard258_slot;
        let mut var_guard258_rv: f64 = *var_guard258_rv_slot;
        let mut var_guard259: f64 = *var_guard259_slot;
        let mut var_guard259_rv: f64 = *var_guard259_rv_slot;
        let mut var_guard260: f64 = *var_guard260_slot;
        let mut var_guard260_rv: f64 = *var_guard260_rv_slot;
        let mut var_guard261: f64 = *var_guard261_slot;
        let mut var_guard261_rv: f64 = *var_guard261_rv_slot;
        let mut var_guard262: f64 = *var_guard262_slot;
        let mut var_guard262_rv: f64 = *var_guard262_rv_slot;
        let mut var_guard263: f64 = *var_guard263_slot;
        let mut var_guard263_rv: f64 = *var_guard263_rv_slot;
        let mut var_guard264: f64 = *var_guard264_slot;
        let mut var_guard264_rv: f64 = *var_guard264_rv_slot;
        let mut var_guard265: f64 = *var_guard265_slot;
        let mut var_guard265_rv: f64 = *var_guard265_rv_slot;
        let mut var_guard266: f64 = *var_guard266_slot;
        let mut var_guard266_rv: f64 = *var_guard266_rv_slot;
        let mut var_guard267: f64 = *var_guard267_slot;
        let mut var_guard267_rv: f64 = *var_guard267_rv_slot;
        let mut var_guard268: f64 = *var_guard268_slot;
        let mut var_guard268_rv: f64 = *var_guard268_rv_slot;
        let mut var_guard269: f64 = *var_guard269_slot;
        let mut var_guard269_rv: f64 = *var_guard269_rv_slot;
        let mut var_guard271: f64 = *var_guard271_slot;
        let mut var_guard271_rv: f64 = *var_guard271_rv_slot;
        let mut var_guard272: f64 = *var_guard272_slot;
        let mut var_guard272_rv: f64 = *var_guard272_rv_slot;
        let mut var_guard273: f64 = *var_guard273_slot;
        let mut var_guard273_rv: f64 = *var_guard273_rv_slot;
        let mut var_guard274: f64 = *var_guard274_slot;
        let mut var_guard274_rv: f64 = *var_guard274_rv_slot;
        let mut var_nuendd: f64 = *var_nuendd_slot;
        let mut var_nuendd_rv: f64 = *var_nuendd_rv_slot;
        let mut var_nuends: f64 = *var_nuends_slot;
        let mut var_nuends_rv: f64 = *var_nuends_rv_slot;
        let mut var_nuintd: f64 = *var_nuintd_slot;
        let mut var_nuintd_rv: f64 = *var_nuintd_rv_slot;
        let mut var_nuints: f64 = *var_nuints_slot;
        let mut var_nuints_rv: f64 = *var_nuints_rv_slot;
        let mut var_rdraingeo: f64 = *var_rdraingeo_slot;
        let mut var_rdraingeo_rv: f64 = *var_rdraingeo_rv_slot;
        let mut var_rend: f64 = *var_rend_slot;
        let mut var_rend_rv: f64 = *var_rend_rv_slot;
        let mut var_rint: f64 = *var_rint_slot;
        let mut var_rint_rv: f64 = *var_rint_rv_slot;
        let mut var_rsourcegeo: f64 = *var_rsourcegeo_slot;
        let mut var_rsourcegeo_rv: f64 = *var_rsourcegeo_rv_slot;

        let (assign9290_e12471,) = {
    if ((((var_guard78 == 0.0) && (var_guard79 != 0.0)) && ((var_guard95 != 0.0) && (!(((((((((var_guard86 != 0.0) || (var_guard87 != 0.0)) || (var_guard88 != 0.0)) || (var_guard89 != 0.0)) || (var_guard90 != 0.0)) || (var_guard91 != 0.0)) || (var_guard92 != 0.0)) || (var_guard93 != 0.0)) || (var_guard94 != 0.0))))) && (var_guard239 == 0.0)) {
        let assign9290_e12465: f64 = (p.p438 * var_dmcgeff);
        let assign9290_e12468: f64 = (var_weff * p.p2);
        let assign9290_e12469: f64 = (assign9290_e12465 / assign9290_e12468);
        (assign9290_e12469,)
    } else {
        (var_rint,)
    }
};
        var_rint = assign9290_e12471;
        var_rint_rv = 0.0;

        let assign9300_e12474: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        var_guard241 = assign9300_e12474;
        var_guard241_rv = 0.0;

        let (assign9310_e12506,) = {
    if ((((var_guard78 == 0.0) && (var_guard79 != 0.0)) && ((var_guard96 != 0.0) && (!((((((((((var_guard86 != 0.0) || (var_guard87 != 0.0)) || (var_guard88 != 0.0)) || (var_guard89 != 0.0)) || (var_guard90 != 0.0)) || (var_guard91 != 0.0)) || (var_guard92 != 0.0)) || (var_guard93 != 0.0)) || (var_guard94 != 0.0)) || (var_guard95 != 0.0))))) && (var_guard241 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign9310_e12506;
        var_rend_rv = 0.0;

        let (assign9320_e12544,) = {
    if ((((var_guard78 == 0.0) && (var_guard79 != 0.0)) && ((var_guard96 != 0.0) && (!((((((((((var_guard86 != 0.0) || (var_guard87 != 0.0)) || (var_guard88 != 0.0)) || (var_guard89 != 0.0)) || (var_guard90 != 0.0)) || (var_guard91 != 0.0)) || (var_guard92 != 0.0)) || (var_guard93 != 0.0)) || (var_guard94 != 0.0)) || (var_guard95 != 0.0))))) && (var_guard241 != 0.0)) {
        let assign9320_e12538: f64 = (p.p438 * var_dmcgeff);
        let assign9320_e12541: f64 = (var_weff * p.p2);
        let assign9320_e12542: f64 = (assign9320_e12538 / assign9320_e12541);
        (assign9320_e12542,)
    } else {
        (var_rint,)
    }
};
        var_rint = assign9320_e12544;
        var_rint_rv = 0.0;

        let (assign9330_e12583,) = {
    if ((((var_guard78 == 0.0) && (var_guard79 != 0.0)) && ((var_guard96 != 0.0) && (!((((((((((var_guard86 != 0.0) || (var_guard87 != 0.0)) || (var_guard88 != 0.0)) || (var_guard89 != 0.0)) || (var_guard90 != 0.0)) || (var_guard91 != 0.0)) || (var_guard92 != 0.0)) || (var_guard93 != 0.0)) || (var_guard94 != 0.0)) || (var_guard95 != 0.0))))) && (var_guard241 == 0.0)) {
        let assign9330_e12577: f64 = (0.5 * p.p438);
        let assign9330_e12579: f64 = (assign9330_e12577 * var_dmcgeff);
        let assign9330_e12581: f64 = (assign9330_e12579 / var_weff);
        (assign9330_e12581,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign9330_e12583;
        var_rend_rv = 0.0;

        let assign9340_e12586: f64 = if p.p2 == 2.0 { 1.0 } else { 0.0 };
        var_guard242 = assign9340_e12586;
        var_guard242_rv = 0.0;

        let (assign9350_e12621,) = {
    if (((((var_guard78 == 0.0) && (var_guard79 != 0.0)) && ((var_guard96 != 0.0) && (!((((((((((var_guard86 != 0.0) || (var_guard87 != 0.0)) || (var_guard88 != 0.0)) || (var_guard89 != 0.0)) || (var_guard90 != 0.0)) || (var_guard91 != 0.0)) || (var_guard92 != 0.0)) || (var_guard93 != 0.0)) || (var_guard94 != 0.0)) || (var_guard95 != 0.0))))) && (var_guard241 == 0.0)) && (var_guard242 != 0.0)) {
        (0.0,)
    } else {
        (var_rint,)
    }
};
        var_rint = assign9350_e12621;
        var_rint_rv = 0.0;

        let (assign9360_e12665,) = {
    if (((((var_guard78 == 0.0) && (var_guard79 != 0.0)) && ((var_guard96 != 0.0) && (!((((((((((var_guard86 != 0.0) || (var_guard87 != 0.0)) || (var_guard88 != 0.0)) || (var_guard89 != 0.0)) || (var_guard90 != 0.0)) || (var_guard91 != 0.0)) || (var_guard92 != 0.0)) || (var_guard93 != 0.0)) || (var_guard94 != 0.0)) || (var_guard95 != 0.0))))) && (var_guard241 == 0.0)) && (var_guard242 == 0.0)) {
        let assign9360_e12657: f64 = (p.p438 * var_dmcgeff);
        let assign9360_e12661: f64 = (p.p2 - 2.0);
        let assign9360_e12662: f64 = (var_weff * assign9360_e12661);
        let assign9360_e12663: f64 = (assign9360_e12657 / assign9360_e12662);
        (assign9360_e12663,)
    } else {
        (var_rint,)
    }
};
        var_rint = assign9360_e12665;
        var_rint_rv = 0.0;

        let (assign9370_e12695,) = {
    if (((var_guard78 == 0.0) && (var_guard79 != 0.0)) && (!(((((((((((var_guard86 != 0.0) || (var_guard87 != 0.0)) || (var_guard88 != 0.0)) || (var_guard89 != 0.0)) || (var_guard90 != 0.0)) || (var_guard91 != 0.0)) || (var_guard92 != 0.0)) || (var_guard93 != 0.0)) || (var_guard94 != 0.0)) || (var_guard95 != 0.0)) || (var_guard96 != 0.0)))) {
        (0.0,)
    } else {
        (var_rint,)
    }
};
        var_rint = assign9370_e12695;
        var_rint_rv = 0.0;

        let assign9380_e12698: f64 = if var_rint <= 0.0 { 1.0 } else { 0.0 };
        var_guard243 = assign9380_e12698;
        var_guard243_rv = 0.0;

        let (assign9390_e12707,) = {
    if (((var_guard78 == 0.0) && (var_guard79 != 0.0)) && (var_guard243 != 0.0)) {
        (var_rend,)
    } else {
        (var_rsourcegeo,)
    }
};
        var_rsourcegeo = assign9390_e12707;
        var_rsourcegeo_rv = 0.0;

        let assign9400_e12710: f64 = if var_rend <= 0.0 { 1.0 } else { 0.0 };
        var_guard244 = assign9400_e12710;
        var_guard244_rv = 0.0;

        let (assign9410_e12722,) = {
    if ((((var_guard78 == 0.0) && (var_guard79 != 0.0)) && (var_guard243 == 0.0)) && (var_guard244 != 0.0)) {
        (var_rint,)
    } else {
        (var_rsourcegeo,)
    }
};
        var_rsourcegeo = assign9410_e12722;
        var_rsourcegeo_rv = 0.0;

        let (assign9420_e12741,) = {
    if ((((var_guard78 == 0.0) && (var_guard79 != 0.0)) && (var_guard243 == 0.0)) && (var_guard244 == 0.0)) {
        let assign9420_e12735: f64 = (var_rint * var_rend);
        let assign9420_e12738: f64 = (var_rint + var_rend);
        let assign9420_e12739: f64 = (assign9420_e12735 / assign9420_e12738);
        (assign9420_e12739,)
    } else {
        (var_rsourcegeo,)
    }
};
        var_rsourcegeo = assign9420_e12741;
        var_rsourcegeo_rv = 0.0;

        let (assign9440_e12752,) = {
    if ((var_guard78 == 0.0) && (var_guard79 == 0.0)) {
        (0.0,)
    } else {
        (var_rsourcegeo,)
    }
};
        var_rsourcegeo = assign9440_e12752;
        var_rsourcegeo_rv = 0.0;

        let assign9450_e12754: f64 = if param_given[4] { 1.0 } else { 0.0 };
        var_guard246 = assign9450_e12754;
        var_guard246_rv = 0.0;

        let (assign9460_e12760,) = {
    if (var_guard246 != 0.0) {
        let assign9460_e12758: f64 = (p.p438 * p.p4);
        (assign9460_e12758,)
    } else {
        (var_rdraingeo,)
    }
};
        var_rdraingeo = assign9460_e12760;
        var_rdraingeo_rv = 0.0;

        let assign9470_e12767: f64 = if ((p.p9 > 0.0) && (p.p438 > 0.0)) { 1.0 } else { 0.0 };
        var_guard247 = assign9470_e12767;
        var_guard247_rv = 0.0;

        let assign9480_e12770: f64 = if p.p8 < 9.0 { 1.0 } else { 0.0 };
        var_guard248 = assign9480_e12770;
        var_guard248_rv = 0.0;

        let assign9490_e12773: f64 = (p.p2 % 2.0);
        let assign9490_e12775: f64 = if assign9490_e12773 != 0.0 { 1.0 } else { 0.0 };
        var_guard249 = assign9490_e12775;
        var_guard249_rv = 0.0;

        let (assign9500_e12786,) = {
    if ((((var_guard246 == 0.0) && (var_guard247 != 0.0)) && (var_guard248 != 0.0)) && (var_guard249 != 0.0)) {
        (1.0,)
    } else {
        (var_nuendd,)
    }
};
        var_nuendd = assign9500_e12786;
        var_nuendd_rv = 0.0;

        let (assign9510_e12797,) = {
    if ((((var_guard246 == 0.0) && (var_guard247 != 0.0)) && (var_guard248 != 0.0)) && (var_guard249 != 0.0)) {
        (1.0,)
    } else {
        (var_nuends,)
    }
};
        var_nuends = assign9510_e12797;
        var_nuends_rv = 0.0;

        let (assign9520_e12816,) = {
    if ((((var_guard246 == 0.0) && (var_guard247 != 0.0)) && (var_guard248 != 0.0)) && (var_guard249 != 0.0)) {
        let assign9520_e12809: f64 = (p.p2 - 1.0);
        let assign9520_e12811: f64 = (assign9520_e12809 / 2.0);
        let assign9520_e12813: f64 = (assign9520_e12811).max(0.0);
        let assign9520_e12814: f64 = (2.0 * assign9520_e12813);
        (assign9520_e12814,)
    } else {
        (var_nuintd,)
    }
};
        var_nuintd = assign9520_e12816;
        var_nuintd_rv = 0.0;

        let (assign9530_e12827,) = {
    if ((((var_guard246 == 0.0) && (var_guard247 != 0.0)) && (var_guard248 != 0.0)) && (var_guard249 != 0.0)) {
        (var_nuintd,)
    } else {
        (var_nuints,)
    }
};
        var_nuints = assign9530_e12827;
        var_nuints_rv = 0.0;

        let assign9540_e12830: f64 = if p.p6 == 1.0 { 1.0 } else { 0.0 };
        var_guard250 = assign9540_e12830;
        var_guard250_rv = 0.0;

        let (assign9550_e12844,) = {
    if (((((var_guard246 == 0.0) && (var_guard247 != 0.0)) && (var_guard248 != 0.0)) && (var_guard249 == 0.0)) && (var_guard250 != 0.0)) {
        (2.0,)
    } else {
        (var_nuendd,)
    }
};
        var_nuendd = assign9550_e12844;
        var_nuendd_rv = 0.0;

        let (assign9560_e12866,) = {
    if (((((var_guard246 == 0.0) && (var_guard247 != 0.0)) && (var_guard248 != 0.0)) && (var_guard249 == 0.0)) && (var_guard250 != 0.0)) {
        let assign9560_e12859: f64 = (p.p2 / 2.0);
        let assign9560_e12861: f64 = (assign9560_e12859 - 1.0);
        let assign9560_e12863: f64 = (assign9560_e12861).max(0.0);
        let assign9560_e12864: f64 = (2.0 * assign9560_e12863);
        (assign9560_e12864,)
    } else {
        (var_nuintd,)
    }
};
        var_nuintd = assign9560_e12866;
        var_nuintd_rv = 0.0;

        let (assign9570_e12880,) = {
    if (((((var_guard246 == 0.0) && (var_guard247 != 0.0)) && (var_guard248 != 0.0)) && (var_guard249 == 0.0)) && (var_guard250 != 0.0)) {
        (0.0,)
    } else {
        (var_nuends,)
    }
};
        var_nuends = assign9570_e12880;
        var_nuends_rv = 0.0;

        let (assign9580_e12894,) = {
    if (((((var_guard246 == 0.0) && (var_guard247 != 0.0)) && (var_guard248 != 0.0)) && (var_guard249 == 0.0)) && (var_guard250 != 0.0)) {
        (p.p2,)
    } else {
        (var_nuints,)
    }
};
        var_nuints = assign9580_e12894;
        var_nuints_rv = 0.0;

        let (assign9590_e12909,) = {
    if (((((var_guard246 == 0.0) && (var_guard247 != 0.0)) && (var_guard248 != 0.0)) && (var_guard249 == 0.0)) && (var_guard250 == 0.0)) {
        (0.0,)
    } else {
        (var_nuendd,)
    }
};
        var_nuendd = assign9590_e12909;
        var_nuendd_rv = 0.0;

        let (assign9600_e12924,) = {
    if (((((var_guard246 == 0.0) && (var_guard247 != 0.0)) && (var_guard248 != 0.0)) && (var_guard249 == 0.0)) && (var_guard250 == 0.0)) {
        (p.p2,)
    } else {
        (var_nuintd,)
    }
};
        var_nuintd = assign9600_e12924;
        var_nuintd_rv = 0.0;

        let (assign9610_e12939,) = {
    if (((((var_guard246 == 0.0) && (var_guard247 != 0.0)) && (var_guard248 != 0.0)) && (var_guard249 == 0.0)) && (var_guard250 == 0.0)) {
        (2.0,)
    } else {
        (var_nuends,)
    }
};
        var_nuends = assign9610_e12939;
        var_nuends_rv = 0.0;

        let (assign9620_e12962,) = {
    if (((((var_guard246 == 0.0) && (var_guard247 != 0.0)) && (var_guard248 != 0.0)) && (var_guard249 == 0.0)) && (var_guard250 == 0.0)) {
        let assign9620_e12955: f64 = (p.p2 / 2.0);
        let assign9620_e12957: f64 = (assign9620_e12955 - 1.0);
        let assign9620_e12959: f64 = (assign9620_e12957).max(0.0);
        let assign9620_e12960: f64 = (2.0 * assign9620_e12959);
        (assign9620_e12960,)
    } else {
        (var_nuints,)
    }
};
        var_nuints = assign9620_e12962;
        var_nuints_rv = 0.0;

        let assign9630_e12965: f64 = if 0.0 == 1.0 { 1.0 } else { 0.0 };
        var_guard251 = assign9630_e12965;
        var_guard251_rv = 0.0;

        let assign9640_e12968: f64 = if var_nuints == 0.0 { 1.0 } else { 0.0 };
        var_guard252 = assign9640_e12968;
        var_guard252_rv = 0.0;

        let (assign9650_e12981,) = {
    if (((((var_guard246 == 0.0) && (var_guard247 != 0.0)) && (var_guard248 != 0.0)) && (var_guard251 != 0.0)) && (var_guard252 != 0.0)) {
        (0.0,)
    } else {
        (var_rint,)
    }
};
        var_rint = assign9650_e12981;
        var_rint_rv = 0.0;

        let (assign9660_e13001,) = {
    if (((((var_guard246 == 0.0) && (var_guard247 != 0.0)) && (var_guard248 != 0.0)) && (var_guard251 != 0.0)) && (var_guard252 == 0.0)) {
        let assign9660_e12995: f64 = (p.p438 * var_dmcgeff);
        let assign9660_e12998: f64 = (var_weff * var_nuints);
        let assign9660_e12999: f64 = (assign9660_e12995 / assign9660_e12998);
        (assign9660_e12999,)
    } else {
        (var_rint,)
    }
};
        var_rint = assign9660_e13001;
        var_rint_rv = 0.0;

        let assign9670_e13004: f64 = if var_nuintd == 0.0 { 1.0 } else { 0.0 };
        var_guard253 = assign9670_e13004;
        var_guard253_rv = 0.0;

        let (assign9680_e13018,) = {
    if (((((var_guard246 == 0.0) && (var_guard247 != 0.0)) && (var_guard248 != 0.0)) && (var_guard251 == 0.0)) && (var_guard253 != 0.0)) {
        (0.0,)
    } else {
        (var_rint,)
    }
};
        var_rint = assign9680_e13018;
        var_rint_rv = 0.0;

        let (assign9690_e13039,) = {
    if (((((var_guard246 == 0.0) && (var_guard247 != 0.0)) && (var_guard248 != 0.0)) && (var_guard251 == 0.0)) && (var_guard253 == 0.0)) {
        let assign9690_e13033: f64 = (p.p438 * var_dmcgeff);
        let assign9690_e13036: f64 = (var_weff * var_nuintd);
        let assign9690_e13037: f64 = (assign9690_e13033 / assign9690_e13036);
        (assign9690_e13037,)
    } else {
        (var_rint,)
    }
};
        var_rint = assign9690_e13039;
        var_rint_rv = 0.0;

        let assign9700_e13042: f64 = if p.p8 == 0.0 { 1.0 } else { 0.0 };
        var_guard254 = assign9700_e13042;
        var_guard254_rv = 0.0;

        let assign9710_e13045: f64 = if p.p8 == 1.0 { 1.0 } else { 0.0 };
        var_guard255 = assign9710_e13045;
        var_guard255_rv = 0.0;

        let assign9720_e13048: f64 = if p.p8 == 2.0 { 1.0 } else { 0.0 };
        var_guard256 = assign9720_e13048;
        var_guard256_rv = 0.0;

        let assign9730_e13051: f64 = if p.p8 == 3.0 { 1.0 } else { 0.0 };
        var_guard257 = assign9730_e13051;
        var_guard257_rv = 0.0;

        let assign9740_e13054: f64 = if p.p8 == 4.0 { 1.0 } else { 0.0 };
        var_guard258 = assign9740_e13054;
        var_guard258_rv = 0.0;

        let assign9750_e13057: f64 = if p.p8 == 5.0 { 1.0 } else { 0.0 };
        var_guard259 = assign9750_e13057;
        var_guard259_rv = 0.0;

        let assign9760_e13060: f64 = if p.p8 == 6.0 { 1.0 } else { 0.0 };
        var_guard260 = assign9760_e13060;
        var_guard260_rv = 0.0;

        let assign9770_e13063: f64 = if p.p8 == 7.0 { 1.0 } else { 0.0 };
        var_guard261 = assign9770_e13063;
        var_guard261_rv = 0.0;

        let assign9780_e13066: f64 = if p.p8 == 8.0 { 1.0 } else { 0.0 };
        var_guard262 = assign9780_e13066;
        var_guard262_rv = 0.0;

        let assign9790_e13069: f64 = if p.p8 == 9.0 { 1.0 } else { 0.0 };
        var_guard263 = assign9790_e13069;
        var_guard263_rv = 0.0;

        let assign9800_e13072: f64 = if p.p8 == 10.0 { 1.0 } else { 0.0 };
        var_guard264 = assign9800_e13072;
        var_guard264_rv = 0.0;

        let assign9810_e13075: f64 = if 0.0 == 1.0 { 1.0 } else { 0.0 };
        var_guard265 = assign9810_e13075;
        var_guard265_rv = 0.0;

        let assign9820_e13078: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        var_guard266 = assign9820_e13078;
        var_guard266_rv = 0.0;

        let assign9830_e13089: f64 = if (((p.p9 == 1.0) || (p.p9 == 2.0)) || (p.p9 == 5.0)) { 1.0 } else { 0.0 };
        var_guard267 = assign9830_e13089;
        var_guard267_rv = 0.0;

        let assign9840_e13100: f64 = if (((p.p9 == 3.0) || (p.p9 == 4.0)) || (p.p9 == 6.0)) { 1.0 } else { 0.0 };
        var_guard268 = assign9840_e13100;
        var_guard268_rv = 0.0;

        let assign9850_e13103: f64 = if var_nuends == 0.0 { 1.0 } else { 0.0 };
        var_guard269 = assign9850_e13103;
        var_guard269_rv = 0.0;

        let (assign9860_e13120,) = {
    if (((((((var_guard246 == 0.0) && (var_guard247 != 0.0)) && (var_guard254 != 0.0)) && (var_guard265 != 0.0)) && (var_guard266 != 0.0)) && (var_guard267 != 0.0)) && (var_guard269 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign9860_e13120;
        var_rend_rv = 0.0;

        let (assign9870_e13144,) = {
    if (((((((var_guard246 == 0.0) && (var_guard247 != 0.0)) && (var_guard254 != 0.0)) && (var_guard265 != 0.0)) && (var_guard266 != 0.0)) && (var_guard267 != 0.0)) && (var_guard269 == 0.0)) {
        let assign9870_e13138: f64 = (p.p438 * var_dmcgeff);
        let assign9870_e13141: f64 = (var_weff * var_nuends);
        let assign9870_e13142: f64 = (assign9870_e13138 / assign9870_e13141);
        (assign9870_e13142,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign9870_e13144;
        var_rend_rv = 0.0;

        let assign9890_e13155: f64 = (var_dmcgeff + var_dmcieff);
        let assign9890_e13158: f64 = if ((var_nuends == 0.0) || (assign9890_e13155 == 0.0)) { 1.0 } else { 0.0 };
        var_guard271 = assign9890_e13158;
        var_guard271_rv = 0.0;

        let (assign9900_e13178,) = {
    if (((((((var_guard246 == 0.0) && (var_guard247 != 0.0)) && (var_guard254 != 0.0)) && (var_guard265 != 0.0)) && (var_guard266 != 0.0)) && ((var_guard268 != 0.0) && (var_guard267 == 0.0))) && (var_guard271 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign9900_e13178;
        var_rend_rv = 0.0;

        let (assign9910_e13209,) = {
    if (((((((var_guard246 == 0.0) && (var_guard247 != 0.0)) && (var_guard254 != 0.0)) && (var_guard265 != 0.0)) && (var_guard266 != 0.0)) && ((var_guard268 != 0.0) && (var_guard267 == 0.0))) && (var_guard271 == 0.0)) {
        let assign9910_e13199: f64 = (p.p438 * var_weff);
        let assign9910_e13202: f64 = (3.0 * var_nuends);
        let assign9910_e13205: f64 = (var_dmcgeff + var_dmcieff);
        let assign9910_e13206: f64 = (assign9910_e13202 * assign9910_e13205);
        let assign9910_e13207: f64 = (assign9910_e13199 / assign9910_e13206);
        (assign9910_e13207,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign9910_e13209;
        var_rend_rv = 0.0;

        let (assign9920_e13227,) = {
    if ((((((var_guard246 == 0.0) && (var_guard247 != 0.0)) && (var_guard254 != 0.0)) && (var_guard265 != 0.0)) && (var_guard266 != 0.0)) && (!((var_guard267 != 0.0) || (var_guard268 != 0.0)))) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign9920_e13227;
        var_rend_rv = 0.0;

        let assign9930_e13238: f64 = if (((p.p9 == 1.0) || (p.p9 == 3.0)) || (p.p9 == 7.0)) { 1.0 } else { 0.0 };
        var_guard272 = assign9930_e13238;
        var_guard272_rv = 0.0;

        let assign9940_e13249: f64 = if (((p.p9 == 2.0) || (p.p9 == 4.0)) || (p.p9 == 8.0)) { 1.0 } else { 0.0 };
        var_guard273 = assign9940_e13249;
        var_guard273_rv = 0.0;

        let assign9950_e13252: f64 = if var_nuends == 0.0 { 1.0 } else { 0.0 };
        var_guard274 = assign9950_e13252;
        var_guard274_rv = 0.0;

        *var_guard241_slot = var_guard241;
        *var_guard241_rv_slot = var_guard241_rv;
        *var_guard242_slot = var_guard242;
        *var_guard242_rv_slot = var_guard242_rv;
        *var_guard243_slot = var_guard243;
        *var_guard243_rv_slot = var_guard243_rv;
        *var_guard244_slot = var_guard244;
        *var_guard244_rv_slot = var_guard244_rv;
        *var_guard246_slot = var_guard246;
        *var_guard246_rv_slot = var_guard246_rv;
        *var_guard247_slot = var_guard247;
        *var_guard247_rv_slot = var_guard247_rv;
        *var_guard248_slot = var_guard248;
        *var_guard248_rv_slot = var_guard248_rv;
        *var_guard249_slot = var_guard249;
        *var_guard249_rv_slot = var_guard249_rv;
        *var_guard250_slot = var_guard250;
        *var_guard250_rv_slot = var_guard250_rv;
        *var_guard251_slot = var_guard251;
        *var_guard251_rv_slot = var_guard251_rv;
        *var_guard252_slot = var_guard252;
        *var_guard252_rv_slot = var_guard252_rv;
        *var_guard253_slot = var_guard253;
        *var_guard253_rv_slot = var_guard253_rv;
        *var_guard254_slot = var_guard254;
        *var_guard254_rv_slot = var_guard254_rv;
        *var_guard255_slot = var_guard255;
        *var_guard255_rv_slot = var_guard255_rv;
        *var_guard256_slot = var_guard256;
        *var_guard256_rv_slot = var_guard256_rv;
        *var_guard257_slot = var_guard257;
        *var_guard257_rv_slot = var_guard257_rv;
        *var_guard258_slot = var_guard258;
        *var_guard258_rv_slot = var_guard258_rv;
        *var_guard259_slot = var_guard259;
        *var_guard259_rv_slot = var_guard259_rv;
        *var_guard260_slot = var_guard260;
        *var_guard260_rv_slot = var_guard260_rv;
        *var_guard261_slot = var_guard261;
        *var_guard261_rv_slot = var_guard261_rv;
        *var_guard262_slot = var_guard262;
        *var_guard262_rv_slot = var_guard262_rv;
        *var_guard263_slot = var_guard263;
        *var_guard263_rv_slot = var_guard263_rv;
        *var_guard264_slot = var_guard264;
        *var_guard264_rv_slot = var_guard264_rv;
        *var_guard265_slot = var_guard265;
        *var_guard265_rv_slot = var_guard265_rv;
        *var_guard266_slot = var_guard266;
        *var_guard266_rv_slot = var_guard266_rv;
        *var_guard267_slot = var_guard267;
        *var_guard267_rv_slot = var_guard267_rv;
        *var_guard268_slot = var_guard268;
        *var_guard268_rv_slot = var_guard268_rv;
        *var_guard269_slot = var_guard269;
        *var_guard269_rv_slot = var_guard269_rv;
        *var_guard271_slot = var_guard271;
        *var_guard271_rv_slot = var_guard271_rv;
        *var_guard272_slot = var_guard272;
        *var_guard272_rv_slot = var_guard272_rv;
        *var_guard273_slot = var_guard273;
        *var_guard273_rv_slot = var_guard273_rv;
        *var_guard274_slot = var_guard274;
        *var_guard274_rv_slot = var_guard274_rv;
        *var_nuendd_slot = var_nuendd;
        *var_nuendd_rv_slot = var_nuendd_rv;
        *var_nuends_slot = var_nuends;
        *var_nuends_rv_slot = var_nuends_rv;
        *var_nuintd_slot = var_nuintd;
        *var_nuintd_rv_slot = var_nuintd_rv;
        *var_nuints_slot = var_nuints;
        *var_nuints_rv_slot = var_nuints_rv;
        *var_rdraingeo_slot = var_rdraingeo;
        *var_rdraingeo_rv_slot = var_rdraingeo_rv;
        *var_rend_slot = var_rend;
        *var_rend_rv_slot = var_rend_rv;
        *var_rint_slot = var_rint;
        *var_rint_rv_slot = var_rint_rv;
        *var_rsourcegeo_slot = var_rsourcegeo;
        *var_rsourcegeo_rv_slot = var_rsourcegeo_rv;
    }

    pub(super) fn stamp_reactive_block_17(
        p: &Parameters,
        var_dmcgeff: f64,
        var_dmcieff: f64,
        var_guard246: f64,
        var_guard247: f64,
        var_guard254: f64,
        var_guard255: f64,
        var_guard265: f64,
        var_guard266: f64,
        var_guard272: f64,
        var_guard273: f64,
        var_guard274: f64,
        var_nuendd: f64,
        var_nuends: f64,
        var_weff: f64,
        var_guard276_slot: &mut f64,
        var_guard276_rv_slot: &mut f64,
        var_guard277_slot: &mut f64,
        var_guard277_rv_slot: &mut f64,
        var_guard278_slot: &mut f64,
        var_guard278_rv_slot: &mut f64,
        var_guard279_slot: &mut f64,
        var_guard279_rv_slot: &mut f64,
        var_guard280_slot: &mut f64,
        var_guard280_rv_slot: &mut f64,
        var_guard282_slot: &mut f64,
        var_guard282_rv_slot: &mut f64,
        var_guard283_slot: &mut f64,
        var_guard283_rv_slot: &mut f64,
        var_guard284_slot: &mut f64,
        var_guard284_rv_slot: &mut f64,
        var_guard285_slot: &mut f64,
        var_guard285_rv_slot: &mut f64,
        var_guard287_slot: &mut f64,
        var_guard287_rv_slot: &mut f64,
        var_guard288_slot: &mut f64,
        var_guard288_rv_slot: &mut f64,
        var_guard289_slot: &mut f64,
        var_guard289_rv_slot: &mut f64,
        var_guard290_slot: &mut f64,
        var_guard290_rv_slot: &mut f64,
        var_guard291_slot: &mut f64,
        var_guard291_rv_slot: &mut f64,
        var_guard292_slot: &mut f64,
        var_guard292_rv_slot: &mut f64,
        var_guard294_slot: &mut f64,
        var_guard294_rv_slot: &mut f64,
        var_guard295_slot: &mut f64,
        var_guard295_rv_slot: &mut f64,
        var_guard296_slot: &mut f64,
        var_guard296_rv_slot: &mut f64,
        var_guard297_slot: &mut f64,
        var_guard297_rv_slot: &mut f64,
        var_guard299_slot: &mut f64,
        var_guard299_rv_slot: &mut f64,
        var_guard300_slot: &mut f64,
        var_guard300_rv_slot: &mut f64,
        var_guard301_slot: &mut f64,
        var_guard301_rv_slot: &mut f64,
        var_guard302_slot: &mut f64,
        var_guard302_rv_slot: &mut f64,
        var_guard303_slot: &mut f64,
        var_guard303_rv_slot: &mut f64,
        var_guard305_slot: &mut f64,
        var_guard305_rv_slot: &mut f64,
        var_guard306_slot: &mut f64,
        var_guard306_rv_slot: &mut f64,
        var_guard307_slot: &mut f64,
        var_guard307_rv_slot: &mut f64,
        var_guard308_slot: &mut f64,
        var_guard308_rv_slot: &mut f64,
        var_guard310_slot: &mut f64,
        var_guard310_rv_slot: &mut f64,
        var_rend_slot: &mut f64,
        var_rend_rv_slot: &mut f64,
    ) {
        let mut var_guard276: f64 = *var_guard276_slot;
        let mut var_guard276_rv: f64 = *var_guard276_rv_slot;
        let mut var_guard277: f64 = *var_guard277_slot;
        let mut var_guard277_rv: f64 = *var_guard277_rv_slot;
        let mut var_guard278: f64 = *var_guard278_slot;
        let mut var_guard278_rv: f64 = *var_guard278_rv_slot;
        let mut var_guard279: f64 = *var_guard279_slot;
        let mut var_guard279_rv: f64 = *var_guard279_rv_slot;
        let mut var_guard280: f64 = *var_guard280_slot;
        let mut var_guard280_rv: f64 = *var_guard280_rv_slot;
        let mut var_guard282: f64 = *var_guard282_slot;
        let mut var_guard282_rv: f64 = *var_guard282_rv_slot;
        let mut var_guard283: f64 = *var_guard283_slot;
        let mut var_guard283_rv: f64 = *var_guard283_rv_slot;
        let mut var_guard284: f64 = *var_guard284_slot;
        let mut var_guard284_rv: f64 = *var_guard284_rv_slot;
        let mut var_guard285: f64 = *var_guard285_slot;
        let mut var_guard285_rv: f64 = *var_guard285_rv_slot;
        let mut var_guard287: f64 = *var_guard287_slot;
        let mut var_guard287_rv: f64 = *var_guard287_rv_slot;
        let mut var_guard288: f64 = *var_guard288_slot;
        let mut var_guard288_rv: f64 = *var_guard288_rv_slot;
        let mut var_guard289: f64 = *var_guard289_slot;
        let mut var_guard289_rv: f64 = *var_guard289_rv_slot;
        let mut var_guard290: f64 = *var_guard290_slot;
        let mut var_guard290_rv: f64 = *var_guard290_rv_slot;
        let mut var_guard291: f64 = *var_guard291_slot;
        let mut var_guard291_rv: f64 = *var_guard291_rv_slot;
        let mut var_guard292: f64 = *var_guard292_slot;
        let mut var_guard292_rv: f64 = *var_guard292_rv_slot;
        let mut var_guard294: f64 = *var_guard294_slot;
        let mut var_guard294_rv: f64 = *var_guard294_rv_slot;
        let mut var_guard295: f64 = *var_guard295_slot;
        let mut var_guard295_rv: f64 = *var_guard295_rv_slot;
        let mut var_guard296: f64 = *var_guard296_slot;
        let mut var_guard296_rv: f64 = *var_guard296_rv_slot;
        let mut var_guard297: f64 = *var_guard297_slot;
        let mut var_guard297_rv: f64 = *var_guard297_rv_slot;
        let mut var_guard299: f64 = *var_guard299_slot;
        let mut var_guard299_rv: f64 = *var_guard299_rv_slot;
        let mut var_guard300: f64 = *var_guard300_slot;
        let mut var_guard300_rv: f64 = *var_guard300_rv_slot;
        let mut var_guard301: f64 = *var_guard301_slot;
        let mut var_guard301_rv: f64 = *var_guard301_rv_slot;
        let mut var_guard302: f64 = *var_guard302_slot;
        let mut var_guard302_rv: f64 = *var_guard302_rv_slot;
        let mut var_guard303: f64 = *var_guard303_slot;
        let mut var_guard303_rv: f64 = *var_guard303_rv_slot;
        let mut var_guard305: f64 = *var_guard305_slot;
        let mut var_guard305_rv: f64 = *var_guard305_rv_slot;
        let mut var_guard306: f64 = *var_guard306_slot;
        let mut var_guard306_rv: f64 = *var_guard306_rv_slot;
        let mut var_guard307: f64 = *var_guard307_slot;
        let mut var_guard307_rv: f64 = *var_guard307_rv_slot;
        let mut var_guard308: f64 = *var_guard308_slot;
        let mut var_guard308_rv: f64 = *var_guard308_rv_slot;
        let mut var_guard310: f64 = *var_guard310_slot;
        let mut var_guard310_rv: f64 = *var_guard310_rv_slot;
        let mut var_rend: f64 = *var_rend_slot;
        let mut var_rend_rv: f64 = *var_rend_rv_slot;

        let (assign9960_e13270,) = {
    if (((((((var_guard246 == 0.0) && (var_guard247 != 0.0)) && (var_guard254 != 0.0)) && (var_guard265 != 0.0)) && (var_guard266 == 0.0)) && (var_guard272 != 0.0)) && (var_guard274 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign9960_e13270;
        var_rend_rv = 0.0;

        let (assign9970_e13295,) = {
    if (((((((var_guard246 == 0.0) && (var_guard247 != 0.0)) && (var_guard254 != 0.0)) && (var_guard265 != 0.0)) && (var_guard266 == 0.0)) && (var_guard272 != 0.0)) && (var_guard274 == 0.0)) {
        let assign9970_e13289: f64 = (p.p438 * var_dmcgeff);
        let assign9970_e13292: f64 = (var_weff * var_nuends);
        let assign9970_e13293: f64 = (assign9970_e13289 / assign9970_e13292);
        (assign9970_e13293,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign9970_e13295;
        var_rend_rv = 0.0;

        let assign9990_e13306: f64 = (var_dmcgeff + var_dmcieff);
        let assign9990_e13309: f64 = if ((var_nuends == 0.0) || (assign9990_e13306 == 0.0)) { 1.0 } else { 0.0 };
        var_guard276 = assign9990_e13309;
        var_guard276_rv = 0.0;

        let (assign10000_e13330,) = {
    if (((((((var_guard246 == 0.0) && (var_guard247 != 0.0)) && (var_guard254 != 0.0)) && (var_guard265 != 0.0)) && (var_guard266 == 0.0)) && ((var_guard273 != 0.0) && (var_guard272 == 0.0))) && (var_guard276 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign10000_e13330;
        var_rend_rv = 0.0;

        let (assign10010_e13362,) = {
    if (((((((var_guard246 == 0.0) && (var_guard247 != 0.0)) && (var_guard254 != 0.0)) && (var_guard265 != 0.0)) && (var_guard266 == 0.0)) && ((var_guard273 != 0.0) && (var_guard272 == 0.0))) && (var_guard276 == 0.0)) {
        let assign10010_e13352: f64 = (p.p438 * var_weff);
        let assign10010_e13355: f64 = (3.0 * var_nuends);
        let assign10010_e13358: f64 = (var_dmcgeff + var_dmcieff);
        let assign10010_e13359: f64 = (assign10010_e13355 * assign10010_e13358);
        let assign10010_e13360: f64 = (assign10010_e13352 / assign10010_e13359);
        (assign10010_e13360,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign10010_e13362;
        var_rend_rv = 0.0;

        let (assign10020_e13381,) = {
    if ((((((var_guard246 == 0.0) && (var_guard247 != 0.0)) && (var_guard254 != 0.0)) && (var_guard265 != 0.0)) && (var_guard266 == 0.0)) && (!((var_guard272 != 0.0) || (var_guard273 != 0.0)))) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign10020_e13381;
        var_rend_rv = 0.0;

        let assign10030_e13384: f64 = if 0.0 == 1.0 { 1.0 } else { 0.0 };
        var_guard277 = assign10030_e13384;
        var_guard277_rv = 0.0;

        let assign10040_e13395: f64 = if (((p.p9 == 1.0) || (p.p9 == 2.0)) || (p.p9 == 5.0)) { 1.0 } else { 0.0 };
        var_guard278 = assign10040_e13395;
        var_guard278_rv = 0.0;

        let assign10050_e13406: f64 = if (((p.p9 == 3.0) || (p.p9 == 4.0)) || (p.p9 == 6.0)) { 1.0 } else { 0.0 };
        var_guard279 = assign10050_e13406;
        var_guard279_rv = 0.0;

        let assign10060_e13409: f64 = if var_nuendd == 0.0 { 1.0 } else { 0.0 };
        var_guard280 = assign10060_e13409;
        var_guard280_rv = 0.0;

        let (assign10070_e13427,) = {
    if (((((((var_guard246 == 0.0) && (var_guard247 != 0.0)) && (var_guard254 != 0.0)) && (var_guard265 == 0.0)) && (var_guard277 != 0.0)) && (var_guard278 != 0.0)) && (var_guard280 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign10070_e13427;
        var_rend_rv = 0.0;

        let (assign10080_e13452,) = {
    if (((((((var_guard246 == 0.0) && (var_guard247 != 0.0)) && (var_guard254 != 0.0)) && (var_guard265 == 0.0)) && (var_guard277 != 0.0)) && (var_guard278 != 0.0)) && (var_guard280 == 0.0)) {
        let assign10080_e13446: f64 = (p.p438 * var_dmcgeff);
        let assign10080_e13449: f64 = (var_weff * var_nuendd);
        let assign10080_e13450: f64 = (assign10080_e13446 / assign10080_e13449);
        (assign10080_e13450,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign10080_e13452;
        var_rend_rv = 0.0;

        let assign10100_e13463: f64 = (var_dmcgeff + var_dmcieff);
        let assign10100_e13466: f64 = if ((var_nuendd == 0.0) || (assign10100_e13463 == 0.0)) { 1.0 } else { 0.0 };
        var_guard282 = assign10100_e13466;
        var_guard282_rv = 0.0;

        let (assign10110_e13487,) = {
    if (((((((var_guard246 == 0.0) && (var_guard247 != 0.0)) && (var_guard254 != 0.0)) && (var_guard265 == 0.0)) && (var_guard277 != 0.0)) && ((var_guard279 != 0.0) && (var_guard278 == 0.0))) && (var_guard282 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign10110_e13487;
        var_rend_rv = 0.0;

        let (assign10120_e13519,) = {
    if (((((((var_guard246 == 0.0) && (var_guard247 != 0.0)) && (var_guard254 != 0.0)) && (var_guard265 == 0.0)) && (var_guard277 != 0.0)) && ((var_guard279 != 0.0) && (var_guard278 == 0.0))) && (var_guard282 == 0.0)) {
        let assign10120_e13509: f64 = (p.p438 * var_weff);
        let assign10120_e13512: f64 = (3.0 * var_nuendd);
        let assign10120_e13515: f64 = (var_dmcgeff + var_dmcieff);
        let assign10120_e13516: f64 = (assign10120_e13512 * assign10120_e13515);
        let assign10120_e13517: f64 = (assign10120_e13509 / assign10120_e13516);
        (assign10120_e13517,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign10120_e13519;
        var_rend_rv = 0.0;

        let (assign10130_e13538,) = {
    if ((((((var_guard246 == 0.0) && (var_guard247 != 0.0)) && (var_guard254 != 0.0)) && (var_guard265 == 0.0)) && (var_guard277 != 0.0)) && (!((var_guard278 != 0.0) || (var_guard279 != 0.0)))) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign10130_e13538;
        var_rend_rv = 0.0;

        let assign10140_e13549: f64 = if (((p.p9 == 1.0) || (p.p9 == 3.0)) || (p.p9 == 7.0)) { 1.0 } else { 0.0 };
        var_guard283 = assign10140_e13549;
        var_guard283_rv = 0.0;

        let assign10150_e13560: f64 = if (((p.p9 == 2.0) || (p.p9 == 4.0)) || (p.p9 == 8.0)) { 1.0 } else { 0.0 };
        var_guard284 = assign10150_e13560;
        var_guard284_rv = 0.0;

        let assign10160_e13563: f64 = if var_nuendd == 0.0 { 1.0 } else { 0.0 };
        var_guard285 = assign10160_e13563;
        var_guard285_rv = 0.0;

        let (assign10170_e13582,) = {
    if (((((((var_guard246 == 0.0) && (var_guard247 != 0.0)) && (var_guard254 != 0.0)) && (var_guard265 == 0.0)) && (var_guard277 == 0.0)) && (var_guard283 != 0.0)) && (var_guard285 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign10170_e13582;
        var_rend_rv = 0.0;

        let (assign10180_e13608,) = {
    if (((((((var_guard246 == 0.0) && (var_guard247 != 0.0)) && (var_guard254 != 0.0)) && (var_guard265 == 0.0)) && (var_guard277 == 0.0)) && (var_guard283 != 0.0)) && (var_guard285 == 0.0)) {
        let assign10180_e13602: f64 = (p.p438 * var_dmcgeff);
        let assign10180_e13605: f64 = (var_weff * var_nuendd);
        let assign10180_e13606: f64 = (assign10180_e13602 / assign10180_e13605);
        (assign10180_e13606,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign10180_e13608;
        var_rend_rv = 0.0;

        let assign10200_e13619: f64 = (var_dmcgeff + var_dmcieff);
        let assign10200_e13622: f64 = if ((var_nuendd == 0.0) || (assign10200_e13619 == 0.0)) { 1.0 } else { 0.0 };
        var_guard287 = assign10200_e13622;
        var_guard287_rv = 0.0;

        let (assign10210_e13644,) = {
    if (((((((var_guard246 == 0.0) && (var_guard247 != 0.0)) && (var_guard254 != 0.0)) && (var_guard265 == 0.0)) && (var_guard277 == 0.0)) && ((var_guard284 != 0.0) && (var_guard283 == 0.0))) && (var_guard287 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign10210_e13644;
        var_rend_rv = 0.0;

        let (assign10220_e13677,) = {
    if (((((((var_guard246 == 0.0) && (var_guard247 != 0.0)) && (var_guard254 != 0.0)) && (var_guard265 == 0.0)) && (var_guard277 == 0.0)) && ((var_guard284 != 0.0) && (var_guard283 == 0.0))) && (var_guard287 == 0.0)) {
        let assign10220_e13667: f64 = (p.p438 * var_weff);
        let assign10220_e13670: f64 = (3.0 * var_nuendd);
        let assign10220_e13673: f64 = (var_dmcgeff + var_dmcieff);
        let assign10220_e13674: f64 = (assign10220_e13670 * assign10220_e13673);
        let assign10220_e13675: f64 = (assign10220_e13667 / assign10220_e13674);
        (assign10220_e13675,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign10220_e13677;
        var_rend_rv = 0.0;

        let (assign10230_e13697,) = {
    if ((((((var_guard246 == 0.0) && (var_guard247 != 0.0)) && (var_guard254 != 0.0)) && (var_guard265 == 0.0)) && (var_guard277 == 0.0)) && (!((var_guard283 != 0.0) || (var_guard284 != 0.0)))) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign10230_e13697;
        var_rend_rv = 0.0;

        let assign10240_e13700: f64 = if 0.0 == 1.0 { 1.0 } else { 0.0 };
        var_guard288 = assign10240_e13700;
        var_guard288_rv = 0.0;

        let assign10250_e13703: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        var_guard289 = assign10250_e13703;
        var_guard289_rv = 0.0;

        let assign10260_e13714: f64 = if (((p.p9 == 1.0) || (p.p9 == 2.0)) || (p.p9 == 5.0)) { 1.0 } else { 0.0 };
        var_guard290 = assign10260_e13714;
        var_guard290_rv = 0.0;

        let assign10270_e13725: f64 = if (((p.p9 == 3.0) || (p.p9 == 4.0)) || (p.p9 == 6.0)) { 1.0 } else { 0.0 };
        var_guard291 = assign10270_e13725;
        var_guard291_rv = 0.0;

        let assign10280_e13728: f64 = if var_nuends == 0.0 { 1.0 } else { 0.0 };
        var_guard292 = assign10280_e13728;
        var_guard292_rv = 0.0;

        let (assign10290_e13748,) = {
    if (((((((var_guard246 == 0.0) && (var_guard247 != 0.0)) && ((var_guard255 != 0.0) && (var_guard254 == 0.0))) && (var_guard288 != 0.0)) && (var_guard289 != 0.0)) && (var_guard290 != 0.0)) && (var_guard292 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign10290_e13748;
        var_rend_rv = 0.0;

        let (assign10300_e13775,) = {
    if (((((((var_guard246 == 0.0) && (var_guard247 != 0.0)) && ((var_guard255 != 0.0) && (var_guard254 == 0.0))) && (var_guard288 != 0.0)) && (var_guard289 != 0.0)) && (var_guard290 != 0.0)) && (var_guard292 == 0.0)) {
        let assign10300_e13769: f64 = (p.p438 * var_dmcgeff);
        let assign10300_e13772: f64 = (var_weff * var_nuends);
        let assign10300_e13773: f64 = (assign10300_e13769 / assign10300_e13772);
        (assign10300_e13773,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign10300_e13775;
        var_rend_rv = 0.0;

        let assign10320_e13786: f64 = (var_dmcgeff + var_dmcieff);
        let assign10320_e13789: f64 = if ((var_nuends == 0.0) || (assign10320_e13786 == 0.0)) { 1.0 } else { 0.0 };
        var_guard294 = assign10320_e13789;
        var_guard294_rv = 0.0;

        let (assign10330_e13812,) = {
    if (((((((var_guard246 == 0.0) && (var_guard247 != 0.0)) && ((var_guard255 != 0.0) && (var_guard254 == 0.0))) && (var_guard288 != 0.0)) && (var_guard289 != 0.0)) && ((var_guard291 != 0.0) && (var_guard290 == 0.0))) && (var_guard294 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign10330_e13812;
        var_rend_rv = 0.0;

        let (assign10340_e13846,) = {
    if (((((((var_guard246 == 0.0) && (var_guard247 != 0.0)) && ((var_guard255 != 0.0) && (var_guard254 == 0.0))) && (var_guard288 != 0.0)) && (var_guard289 != 0.0)) && ((var_guard291 != 0.0) && (var_guard290 == 0.0))) && (var_guard294 == 0.0)) {
        let assign10340_e13836: f64 = (p.p438 * var_weff);
        let assign10340_e13839: f64 = (3.0 * var_nuends);
        let assign10340_e13842: f64 = (var_dmcgeff + var_dmcieff);
        let assign10340_e13843: f64 = (assign10340_e13839 * assign10340_e13842);
        let assign10340_e13844: f64 = (assign10340_e13836 / assign10340_e13843);
        (assign10340_e13844,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign10340_e13846;
        var_rend_rv = 0.0;

        let (assign10350_e13867,) = {
    if ((((((var_guard246 == 0.0) && (var_guard247 != 0.0)) && ((var_guard255 != 0.0) && (var_guard254 == 0.0))) && (var_guard288 != 0.0)) && (var_guard289 != 0.0)) && (!((var_guard290 != 0.0) || (var_guard291 != 0.0)))) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign10350_e13867;
        var_rend_rv = 0.0;

        let assign10360_e13878: f64 = if (((p.p9 == 1.0) || (p.p9 == 3.0)) || (p.p9 == 7.0)) { 1.0 } else { 0.0 };
        var_guard295 = assign10360_e13878;
        var_guard295_rv = 0.0;

        let assign10370_e13889: f64 = if (((p.p9 == 2.0) || (p.p9 == 4.0)) || (p.p9 == 8.0)) { 1.0 } else { 0.0 };
        var_guard296 = assign10370_e13889;
        var_guard296_rv = 0.0;

        let assign10380_e13892: f64 = if var_nuends == 0.0 { 1.0 } else { 0.0 };
        var_guard297 = assign10380_e13892;
        var_guard297_rv = 0.0;

        let (assign10390_e13913,) = {
    if (((((((var_guard246 == 0.0) && (var_guard247 != 0.0)) && ((var_guard255 != 0.0) && (var_guard254 == 0.0))) && (var_guard288 != 0.0)) && (var_guard289 == 0.0)) && (var_guard295 != 0.0)) && (var_guard297 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign10390_e13913;
        var_rend_rv = 0.0;

        let (assign10400_e13941,) = {
    if (((((((var_guard246 == 0.0) && (var_guard247 != 0.0)) && ((var_guard255 != 0.0) && (var_guard254 == 0.0))) && (var_guard288 != 0.0)) && (var_guard289 == 0.0)) && (var_guard295 != 0.0)) && (var_guard297 == 0.0)) {
        let assign10400_e13935: f64 = (p.p438 * var_dmcgeff);
        let assign10400_e13938: f64 = (var_weff * var_nuends);
        let assign10400_e13939: f64 = (assign10400_e13935 / assign10400_e13938);
        (assign10400_e13939,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign10400_e13941;
        var_rend_rv = 0.0;

        let assign10420_e13952: f64 = (var_dmcgeff + var_dmcieff);
        let assign10420_e13955: f64 = if ((var_nuends == 0.0) || (assign10420_e13952 == 0.0)) { 1.0 } else { 0.0 };
        var_guard299 = assign10420_e13955;
        var_guard299_rv = 0.0;

        let (assign10430_e13979,) = {
    if (((((((var_guard246 == 0.0) && (var_guard247 != 0.0)) && ((var_guard255 != 0.0) && (var_guard254 == 0.0))) && (var_guard288 != 0.0)) && (var_guard289 == 0.0)) && ((var_guard296 != 0.0) && (var_guard295 == 0.0))) && (var_guard299 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign10430_e13979;
        var_rend_rv = 0.0;

        let (assign10440_e14014,) = {
    if (((((((var_guard246 == 0.0) && (var_guard247 != 0.0)) && ((var_guard255 != 0.0) && (var_guard254 == 0.0))) && (var_guard288 != 0.0)) && (var_guard289 == 0.0)) && ((var_guard296 != 0.0) && (var_guard295 == 0.0))) && (var_guard299 == 0.0)) {
        let assign10440_e14004: f64 = (p.p438 * var_weff);
        let assign10440_e14007: f64 = (3.0 * var_nuends);
        let assign10440_e14010: f64 = (var_dmcgeff + var_dmcieff);
        let assign10440_e14011: f64 = (assign10440_e14007 * assign10440_e14010);
        let assign10440_e14012: f64 = (assign10440_e14004 / assign10440_e14011);
        (assign10440_e14012,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign10440_e14014;
        var_rend_rv = 0.0;

        let (assign10450_e14036,) = {
    if ((((((var_guard246 == 0.0) && (var_guard247 != 0.0)) && ((var_guard255 != 0.0) && (var_guard254 == 0.0))) && (var_guard288 != 0.0)) && (var_guard289 == 0.0)) && (!((var_guard295 != 0.0) || (var_guard296 != 0.0)))) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign10450_e14036;
        var_rend_rv = 0.0;

        let assign10460_e14039: f64 = if 0.0 == 1.0 { 1.0 } else { 0.0 };
        var_guard300 = assign10460_e14039;
        var_guard300_rv = 0.0;

        let assign10470_e14050: f64 = if (((p.p9 == 1.0) || (p.p9 == 2.0)) || (p.p9 == 5.0)) { 1.0 } else { 0.0 };
        var_guard301 = assign10470_e14050;
        var_guard301_rv = 0.0;

        let assign10480_e14061: f64 = if (((p.p9 == 3.0) || (p.p9 == 4.0)) || (p.p9 == 6.0)) { 1.0 } else { 0.0 };
        var_guard302 = assign10480_e14061;
        var_guard302_rv = 0.0;

        let assign10490_e14064: f64 = if var_nuendd == 0.0 { 1.0 } else { 0.0 };
        var_guard303 = assign10490_e14064;
        var_guard303_rv = 0.0;

        let (assign10500_e14085,) = {
    if (((((((var_guard246 == 0.0) && (var_guard247 != 0.0)) && ((var_guard255 != 0.0) && (var_guard254 == 0.0))) && (var_guard288 == 0.0)) && (var_guard300 != 0.0)) && (var_guard301 != 0.0)) && (var_guard303 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign10500_e14085;
        var_rend_rv = 0.0;

        let (assign10510_e14113,) = {
    if (((((((var_guard246 == 0.0) && (var_guard247 != 0.0)) && ((var_guard255 != 0.0) && (var_guard254 == 0.0))) && (var_guard288 == 0.0)) && (var_guard300 != 0.0)) && (var_guard301 != 0.0)) && (var_guard303 == 0.0)) {
        let assign10510_e14107: f64 = (p.p438 * var_dmcgeff);
        let assign10510_e14110: f64 = (var_weff * var_nuendd);
        let assign10510_e14111: f64 = (assign10510_e14107 / assign10510_e14110);
        (assign10510_e14111,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign10510_e14113;
        var_rend_rv = 0.0;

        let assign10530_e14123: f64 = if ((var_nuendd == 0.0) || (var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        var_guard305 = assign10530_e14123;
        var_guard305_rv = 0.0;

        let (assign10540_e14147,) = {
    if (((((((var_guard246 == 0.0) && (var_guard247 != 0.0)) && ((var_guard255 != 0.0) && (var_guard254 == 0.0))) && (var_guard288 == 0.0)) && (var_guard300 != 0.0)) && ((var_guard302 != 0.0) && (var_guard301 == 0.0))) && (var_guard305 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign10540_e14147;
        var_rend_rv = 0.0;

        let (assign10550_e14180,) = {
    if (((((((var_guard246 == 0.0) && (var_guard247 != 0.0)) && ((var_guard255 != 0.0) && (var_guard254 == 0.0))) && (var_guard288 == 0.0)) && (var_guard300 != 0.0)) && ((var_guard302 != 0.0) && (var_guard301 == 0.0))) && (var_guard305 == 0.0)) {
        let assign10550_e14172: f64 = (p.p438 * var_weff);
        let assign10550_e14175: f64 = (6.0 * var_nuendd);
        let assign10550_e14177: f64 = (assign10550_e14175 * var_dmcgeff);
        let assign10550_e14178: f64 = (assign10550_e14172 / assign10550_e14177);
        (assign10550_e14178,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign10550_e14180;
        var_rend_rv = 0.0;

        let (assign10560_e14202,) = {
    if ((((((var_guard246 == 0.0) && (var_guard247 != 0.0)) && ((var_guard255 != 0.0) && (var_guard254 == 0.0))) && (var_guard288 == 0.0)) && (var_guard300 != 0.0)) && (!((var_guard301 != 0.0) || (var_guard302 != 0.0)))) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign10560_e14202;
        var_rend_rv = 0.0;

        let assign10570_e14213: f64 = if (((p.p9 == 1.0) || (p.p9 == 3.0)) || (p.p9 == 7.0)) { 1.0 } else { 0.0 };
        var_guard306 = assign10570_e14213;
        var_guard306_rv = 0.0;

        let assign10580_e14224: f64 = if (((p.p9 == 2.0) || (p.p9 == 4.0)) || (p.p9 == 8.0)) { 1.0 } else { 0.0 };
        var_guard307 = assign10580_e14224;
        var_guard307_rv = 0.0;

        let assign10590_e14227: f64 = if var_nuendd == 0.0 { 1.0 } else { 0.0 };
        var_guard308 = assign10590_e14227;
        var_guard308_rv = 0.0;

        let (assign10600_e14249,) = {
    if (((((((var_guard246 == 0.0) && (var_guard247 != 0.0)) && ((var_guard255 != 0.0) && (var_guard254 == 0.0))) && (var_guard288 == 0.0)) && (var_guard300 == 0.0)) && (var_guard306 != 0.0)) && (var_guard308 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign10600_e14249;
        var_rend_rv = 0.0;

        let (assign10610_e14278,) = {
    if (((((((var_guard246 == 0.0) && (var_guard247 != 0.0)) && ((var_guard255 != 0.0) && (var_guard254 == 0.0))) && (var_guard288 == 0.0)) && (var_guard300 == 0.0)) && (var_guard306 != 0.0)) && (var_guard308 == 0.0)) {
        let assign10610_e14272: f64 = (p.p438 * var_dmcgeff);
        let assign10610_e14275: f64 = (var_weff * var_nuendd);
        let assign10610_e14276: f64 = (assign10610_e14272 / assign10610_e14275);
        (assign10610_e14276,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign10610_e14278;
        var_rend_rv = 0.0;

        let assign10630_e14288: f64 = if ((var_nuendd == 0.0) || (var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        var_guard310 = assign10630_e14288;
        var_guard310_rv = 0.0;

        let (assign10640_e14313,) = {
    if (((((((var_guard246 == 0.0) && (var_guard247 != 0.0)) && ((var_guard255 != 0.0) && (var_guard254 == 0.0))) && (var_guard288 == 0.0)) && (var_guard300 == 0.0)) && ((var_guard307 != 0.0) && (var_guard306 == 0.0))) && (var_guard310 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign10640_e14313;
        var_rend_rv = 0.0;

        *var_guard276_slot = var_guard276;
        *var_guard276_rv_slot = var_guard276_rv;
        *var_guard277_slot = var_guard277;
        *var_guard277_rv_slot = var_guard277_rv;
        *var_guard278_slot = var_guard278;
        *var_guard278_rv_slot = var_guard278_rv;
        *var_guard279_slot = var_guard279;
        *var_guard279_rv_slot = var_guard279_rv;
        *var_guard280_slot = var_guard280;
        *var_guard280_rv_slot = var_guard280_rv;
        *var_guard282_slot = var_guard282;
        *var_guard282_rv_slot = var_guard282_rv;
        *var_guard283_slot = var_guard283;
        *var_guard283_rv_slot = var_guard283_rv;
        *var_guard284_slot = var_guard284;
        *var_guard284_rv_slot = var_guard284_rv;
        *var_guard285_slot = var_guard285;
        *var_guard285_rv_slot = var_guard285_rv;
        *var_guard287_slot = var_guard287;
        *var_guard287_rv_slot = var_guard287_rv;
        *var_guard288_slot = var_guard288;
        *var_guard288_rv_slot = var_guard288_rv;
        *var_guard289_slot = var_guard289;
        *var_guard289_rv_slot = var_guard289_rv;
        *var_guard290_slot = var_guard290;
        *var_guard290_rv_slot = var_guard290_rv;
        *var_guard291_slot = var_guard291;
        *var_guard291_rv_slot = var_guard291_rv;
        *var_guard292_slot = var_guard292;
        *var_guard292_rv_slot = var_guard292_rv;
        *var_guard294_slot = var_guard294;
        *var_guard294_rv_slot = var_guard294_rv;
        *var_guard295_slot = var_guard295;
        *var_guard295_rv_slot = var_guard295_rv;
        *var_guard296_slot = var_guard296;
        *var_guard296_rv_slot = var_guard296_rv;
        *var_guard297_slot = var_guard297;
        *var_guard297_rv_slot = var_guard297_rv;
        *var_guard299_slot = var_guard299;
        *var_guard299_rv_slot = var_guard299_rv;
        *var_guard300_slot = var_guard300;
        *var_guard300_rv_slot = var_guard300_rv;
        *var_guard301_slot = var_guard301;
        *var_guard301_rv_slot = var_guard301_rv;
        *var_guard302_slot = var_guard302;
        *var_guard302_rv_slot = var_guard302_rv;
        *var_guard303_slot = var_guard303;
        *var_guard303_rv_slot = var_guard303_rv;
        *var_guard305_slot = var_guard305;
        *var_guard305_rv_slot = var_guard305_rv;
        *var_guard306_slot = var_guard306;
        *var_guard306_rv_slot = var_guard306_rv;
        *var_guard307_slot = var_guard307;
        *var_guard307_rv_slot = var_guard307_rv;
        *var_guard308_slot = var_guard308;
        *var_guard308_rv_slot = var_guard308_rv;
        *var_guard310_slot = var_guard310;
        *var_guard310_rv_slot = var_guard310_rv;
        *var_rend_slot = var_rend;
        *var_rend_rv_slot = var_rend_rv;
    }

    pub(super) fn stamp_reactive_block_18(
        p: &Parameters,
        var_dmcgeff: f64,
        var_dmcieff: f64,
        var_guard246: f64,
        var_guard247: f64,
        var_guard254: f64,
        var_guard255: f64,
        var_guard256: f64,
        var_guard257: f64,
        var_guard288: f64,
        var_guard300: f64,
        var_guard306: f64,
        var_guard307: f64,
        var_guard310: f64,
        var_nuendd: f64,
        var_nuends: f64,
        var_weff: f64,
        var_guard311_slot: &mut f64,
        var_guard311_rv_slot: &mut f64,
        var_guard312_slot: &mut f64,
        var_guard312_rv_slot: &mut f64,
        var_guard313_slot: &mut f64,
        var_guard313_rv_slot: &mut f64,
        var_guard314_slot: &mut f64,
        var_guard314_rv_slot: &mut f64,
        var_guard315_slot: &mut f64,
        var_guard315_rv_slot: &mut f64,
        var_guard317_slot: &mut f64,
        var_guard317_rv_slot: &mut f64,
        var_guard318_slot: &mut f64,
        var_guard318_rv_slot: &mut f64,
        var_guard319_slot: &mut f64,
        var_guard319_rv_slot: &mut f64,
        var_guard320_slot: &mut f64,
        var_guard320_rv_slot: &mut f64,
        var_guard322_slot: &mut f64,
        var_guard322_rv_slot: &mut f64,
        var_guard323_slot: &mut f64,
        var_guard323_rv_slot: &mut f64,
        var_guard324_slot: &mut f64,
        var_guard324_rv_slot: &mut f64,
        var_guard325_slot: &mut f64,
        var_guard325_rv_slot: &mut f64,
        var_guard326_slot: &mut f64,
        var_guard326_rv_slot: &mut f64,
        var_guard328_slot: &mut f64,
        var_guard328_rv_slot: &mut f64,
        var_guard329_slot: &mut f64,
        var_guard329_rv_slot: &mut f64,
        var_guard330_slot: &mut f64,
        var_guard330_rv_slot: &mut f64,
        var_guard331_slot: &mut f64,
        var_guard331_rv_slot: &mut f64,
        var_guard333_slot: &mut f64,
        var_guard333_rv_slot: &mut f64,
        var_guard334_slot: &mut f64,
        var_guard334_rv_slot: &mut f64,
        var_guard335_slot: &mut f64,
        var_guard335_rv_slot: &mut f64,
        var_guard336_slot: &mut f64,
        var_guard336_rv_slot: &mut f64,
        var_guard337_slot: &mut f64,
        var_guard337_rv_slot: &mut f64,
        var_guard338_slot: &mut f64,
        var_guard338_rv_slot: &mut f64,
        var_guard340_slot: &mut f64,
        var_guard340_rv_slot: &mut f64,
        var_guard341_slot: &mut f64,
        var_guard341_rv_slot: &mut f64,
        var_guard342_slot: &mut f64,
        var_guard342_rv_slot: &mut f64,
        var_guard343_slot: &mut f64,
        var_guard343_rv_slot: &mut f64,
        var_guard345_slot: &mut f64,
        var_guard345_rv_slot: &mut f64,
        var_guard346_slot: &mut f64,
        var_guard346_rv_slot: &mut f64,
        var_guard347_slot: &mut f64,
        var_guard347_rv_slot: &mut f64,
        var_guard348_slot: &mut f64,
        var_guard348_rv_slot: &mut f64,
        var_guard349_slot: &mut f64,
        var_guard349_rv_slot: &mut f64,
        var_rend_slot: &mut f64,
        var_rend_rv_slot: &mut f64,
    ) {
        let mut var_guard311: f64 = *var_guard311_slot;
        let mut var_guard311_rv: f64 = *var_guard311_rv_slot;
        let mut var_guard312: f64 = *var_guard312_slot;
        let mut var_guard312_rv: f64 = *var_guard312_rv_slot;
        let mut var_guard313: f64 = *var_guard313_slot;
        let mut var_guard313_rv: f64 = *var_guard313_rv_slot;
        let mut var_guard314: f64 = *var_guard314_slot;
        let mut var_guard314_rv: f64 = *var_guard314_rv_slot;
        let mut var_guard315: f64 = *var_guard315_slot;
        let mut var_guard315_rv: f64 = *var_guard315_rv_slot;
        let mut var_guard317: f64 = *var_guard317_slot;
        let mut var_guard317_rv: f64 = *var_guard317_rv_slot;
        let mut var_guard318: f64 = *var_guard318_slot;
        let mut var_guard318_rv: f64 = *var_guard318_rv_slot;
        let mut var_guard319: f64 = *var_guard319_slot;
        let mut var_guard319_rv: f64 = *var_guard319_rv_slot;
        let mut var_guard320: f64 = *var_guard320_slot;
        let mut var_guard320_rv: f64 = *var_guard320_rv_slot;
        let mut var_guard322: f64 = *var_guard322_slot;
        let mut var_guard322_rv: f64 = *var_guard322_rv_slot;
        let mut var_guard323: f64 = *var_guard323_slot;
        let mut var_guard323_rv: f64 = *var_guard323_rv_slot;
        let mut var_guard324: f64 = *var_guard324_slot;
        let mut var_guard324_rv: f64 = *var_guard324_rv_slot;
        let mut var_guard325: f64 = *var_guard325_slot;
        let mut var_guard325_rv: f64 = *var_guard325_rv_slot;
        let mut var_guard326: f64 = *var_guard326_slot;
        let mut var_guard326_rv: f64 = *var_guard326_rv_slot;
        let mut var_guard328: f64 = *var_guard328_slot;
        let mut var_guard328_rv: f64 = *var_guard328_rv_slot;
        let mut var_guard329: f64 = *var_guard329_slot;
        let mut var_guard329_rv: f64 = *var_guard329_rv_slot;
        let mut var_guard330: f64 = *var_guard330_slot;
        let mut var_guard330_rv: f64 = *var_guard330_rv_slot;
        let mut var_guard331: f64 = *var_guard331_slot;
        let mut var_guard331_rv: f64 = *var_guard331_rv_slot;
        let mut var_guard333: f64 = *var_guard333_slot;
        let mut var_guard333_rv: f64 = *var_guard333_rv_slot;
        let mut var_guard334: f64 = *var_guard334_slot;
        let mut var_guard334_rv: f64 = *var_guard334_rv_slot;
        let mut var_guard335: f64 = *var_guard335_slot;
        let mut var_guard335_rv: f64 = *var_guard335_rv_slot;
        let mut var_guard336: f64 = *var_guard336_slot;
        let mut var_guard336_rv: f64 = *var_guard336_rv_slot;
        let mut var_guard337: f64 = *var_guard337_slot;
        let mut var_guard337_rv: f64 = *var_guard337_rv_slot;
        let mut var_guard338: f64 = *var_guard338_slot;
        let mut var_guard338_rv: f64 = *var_guard338_rv_slot;
        let mut var_guard340: f64 = *var_guard340_slot;
        let mut var_guard340_rv: f64 = *var_guard340_rv_slot;
        let mut var_guard341: f64 = *var_guard341_slot;
        let mut var_guard341_rv: f64 = *var_guard341_rv_slot;
        let mut var_guard342: f64 = *var_guard342_slot;
        let mut var_guard342_rv: f64 = *var_guard342_rv_slot;
        let mut var_guard343: f64 = *var_guard343_slot;
        let mut var_guard343_rv: f64 = *var_guard343_rv_slot;
        let mut var_guard345: f64 = *var_guard345_slot;
        let mut var_guard345_rv: f64 = *var_guard345_rv_slot;
        let mut var_guard346: f64 = *var_guard346_slot;
        let mut var_guard346_rv: f64 = *var_guard346_rv_slot;
        let mut var_guard347: f64 = *var_guard347_slot;
        let mut var_guard347_rv: f64 = *var_guard347_rv_slot;
        let mut var_guard348: f64 = *var_guard348_slot;
        let mut var_guard348_rv: f64 = *var_guard348_rv_slot;
        let mut var_guard349: f64 = *var_guard349_slot;
        let mut var_guard349_rv: f64 = *var_guard349_rv_slot;
        let mut var_rend: f64 = *var_rend_slot;
        let mut var_rend_rv: f64 = *var_rend_rv_slot;

        let (assign10650_e14347,) = {
    if (((((((var_guard246 == 0.0) && (var_guard247 != 0.0)) && ((var_guard255 != 0.0) && (var_guard254 == 0.0))) && (var_guard288 == 0.0)) && (var_guard300 == 0.0)) && ((var_guard307 != 0.0) && (var_guard306 == 0.0))) && (var_guard310 == 0.0)) {
        let assign10650_e14339: f64 = (p.p438 * var_weff);
        let assign10650_e14342: f64 = (6.0 * var_nuendd);
        let assign10650_e14344: f64 = (assign10650_e14342 * var_dmcgeff);
        let assign10650_e14345: f64 = (assign10650_e14339 / assign10650_e14344);
        (assign10650_e14345,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign10650_e14347;
        var_rend_rv = 0.0;

        let (assign10660_e14370,) = {
    if ((((((var_guard246 == 0.0) && (var_guard247 != 0.0)) && ((var_guard255 != 0.0) && (var_guard254 == 0.0))) && (var_guard288 == 0.0)) && (var_guard300 == 0.0)) && (!((var_guard306 != 0.0) || (var_guard307 != 0.0)))) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign10660_e14370;
        var_rend_rv = 0.0;

        let assign10670_e14373: f64 = if 0.0 == 1.0 { 1.0 } else { 0.0 };
        var_guard311 = assign10670_e14373;
        var_guard311_rv = 0.0;

        let assign10680_e14376: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        var_guard312 = assign10680_e14376;
        var_guard312_rv = 0.0;

        let assign10690_e14387: f64 = if (((p.p9 == 1.0) || (p.p9 == 2.0)) || (p.p9 == 5.0)) { 1.0 } else { 0.0 };
        var_guard313 = assign10690_e14387;
        var_guard313_rv = 0.0;

        let assign10700_e14398: f64 = if (((p.p9 == 3.0) || (p.p9 == 4.0)) || (p.p9 == 6.0)) { 1.0 } else { 0.0 };
        var_guard314 = assign10700_e14398;
        var_guard314_rv = 0.0;

        let assign10710_e14401: f64 = if var_nuends == 0.0 { 1.0 } else { 0.0 };
        var_guard315 = assign10710_e14401;
        var_guard315_rv = 0.0;

        let (assign10720_e14423,) = {
    if (((((((var_guard246 == 0.0) && (var_guard247 != 0.0)) && ((var_guard256 != 0.0) && (!((var_guard254 != 0.0) || (var_guard255 != 0.0))))) && (var_guard311 != 0.0)) && (var_guard312 != 0.0)) && (var_guard313 != 0.0)) && (var_guard315 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign10720_e14423;
        var_rend_rv = 0.0;

        let (assign10730_e14452,) = {
    if (((((((var_guard246 == 0.0) && (var_guard247 != 0.0)) && ((var_guard256 != 0.0) && (!((var_guard254 != 0.0) || (var_guard255 != 0.0))))) && (var_guard311 != 0.0)) && (var_guard312 != 0.0)) && (var_guard313 != 0.0)) && (var_guard315 == 0.0)) {
        let assign10730_e14446: f64 = (p.p438 * var_dmcgeff);
        let assign10730_e14449: f64 = (var_weff * var_nuends);
        let assign10730_e14450: f64 = (assign10730_e14446 / assign10730_e14449);
        (assign10730_e14450,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign10730_e14452;
        var_rend_rv = 0.0;

        let assign10750_e14462: f64 = if ((var_nuends == 0.0) || (var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        var_guard317 = assign10750_e14462;
        var_guard317_rv = 0.0;

        let (assign10760_e14487,) = {
    if (((((((var_guard246 == 0.0) && (var_guard247 != 0.0)) && ((var_guard256 != 0.0) && (!((var_guard254 != 0.0) || (var_guard255 != 0.0))))) && (var_guard311 != 0.0)) && (var_guard312 != 0.0)) && ((var_guard314 != 0.0) && (var_guard313 == 0.0))) && (var_guard317 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign10760_e14487;
        var_rend_rv = 0.0;

        let (assign10770_e14521,) = {
    if (((((((var_guard246 == 0.0) && (var_guard247 != 0.0)) && ((var_guard256 != 0.0) && (!((var_guard254 != 0.0) || (var_guard255 != 0.0))))) && (var_guard311 != 0.0)) && (var_guard312 != 0.0)) && ((var_guard314 != 0.0) && (var_guard313 == 0.0))) && (var_guard317 == 0.0)) {
        let assign10770_e14513: f64 = (p.p438 * var_weff);
        let assign10770_e14516: f64 = (6.0 * var_nuends);
        let assign10770_e14518: f64 = (assign10770_e14516 * var_dmcgeff);
        let assign10770_e14519: f64 = (assign10770_e14513 / assign10770_e14518);
        (assign10770_e14519,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign10770_e14521;
        var_rend_rv = 0.0;

        let (assign10780_e14544,) = {
    if ((((((var_guard246 == 0.0) && (var_guard247 != 0.0)) && ((var_guard256 != 0.0) && (!((var_guard254 != 0.0) || (var_guard255 != 0.0))))) && (var_guard311 != 0.0)) && (var_guard312 != 0.0)) && (!((var_guard313 != 0.0) || (var_guard314 != 0.0)))) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign10780_e14544;
        var_rend_rv = 0.0;

        let assign10790_e14555: f64 = if (((p.p9 == 1.0) || (p.p9 == 3.0)) || (p.p9 == 7.0)) { 1.0 } else { 0.0 };
        var_guard318 = assign10790_e14555;
        var_guard318_rv = 0.0;

        let assign10800_e14566: f64 = if (((p.p9 == 2.0) || (p.p9 == 4.0)) || (p.p9 == 8.0)) { 1.0 } else { 0.0 };
        var_guard319 = assign10800_e14566;
        var_guard319_rv = 0.0;

        let assign10810_e14569: f64 = if var_nuends == 0.0 { 1.0 } else { 0.0 };
        var_guard320 = assign10810_e14569;
        var_guard320_rv = 0.0;

        let (assign10820_e14592,) = {
    if (((((((var_guard246 == 0.0) && (var_guard247 != 0.0)) && ((var_guard256 != 0.0) && (!((var_guard254 != 0.0) || (var_guard255 != 0.0))))) && (var_guard311 != 0.0)) && (var_guard312 == 0.0)) && (var_guard318 != 0.0)) && (var_guard320 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign10820_e14592;
        var_rend_rv = 0.0;

        let (assign10830_e14622,) = {
    if (((((((var_guard246 == 0.0) && (var_guard247 != 0.0)) && ((var_guard256 != 0.0) && (!((var_guard254 != 0.0) || (var_guard255 != 0.0))))) && (var_guard311 != 0.0)) && (var_guard312 == 0.0)) && (var_guard318 != 0.0)) && (var_guard320 == 0.0)) {
        let assign10830_e14616: f64 = (p.p438 * var_dmcgeff);
        let assign10830_e14619: f64 = (var_weff * var_nuends);
        let assign10830_e14620: f64 = (assign10830_e14616 / assign10830_e14619);
        (assign10830_e14620,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign10830_e14622;
        var_rend_rv = 0.0;

        let assign10850_e14632: f64 = if ((var_nuends == 0.0) || (var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        var_guard322 = assign10850_e14632;
        var_guard322_rv = 0.0;

        let (assign10860_e14658,) = {
    if (((((((var_guard246 == 0.0) && (var_guard247 != 0.0)) && ((var_guard256 != 0.0) && (!((var_guard254 != 0.0) || (var_guard255 != 0.0))))) && (var_guard311 != 0.0)) && (var_guard312 == 0.0)) && ((var_guard319 != 0.0) && (var_guard318 == 0.0))) && (var_guard322 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign10860_e14658;
        var_rend_rv = 0.0;

        let (assign10870_e14693,) = {
    if (((((((var_guard246 == 0.0) && (var_guard247 != 0.0)) && ((var_guard256 != 0.0) && (!((var_guard254 != 0.0) || (var_guard255 != 0.0))))) && (var_guard311 != 0.0)) && (var_guard312 == 0.0)) && ((var_guard319 != 0.0) && (var_guard318 == 0.0))) && (var_guard322 == 0.0)) {
        let assign10870_e14685: f64 = (p.p438 * var_weff);
        let assign10870_e14688: f64 = (6.0 * var_nuends);
        let assign10870_e14690: f64 = (assign10870_e14688 * var_dmcgeff);
        let assign10870_e14691: f64 = (assign10870_e14685 / assign10870_e14690);
        (assign10870_e14691,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign10870_e14693;
        var_rend_rv = 0.0;

        let (assign10880_e14717,) = {
    if ((((((var_guard246 == 0.0) && (var_guard247 != 0.0)) && ((var_guard256 != 0.0) && (!((var_guard254 != 0.0) || (var_guard255 != 0.0))))) && (var_guard311 != 0.0)) && (var_guard312 == 0.0)) && (!((var_guard318 != 0.0) || (var_guard319 != 0.0)))) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign10880_e14717;
        var_rend_rv = 0.0;

        let assign10890_e14720: f64 = if 0.0 == 1.0 { 1.0 } else { 0.0 };
        var_guard323 = assign10890_e14720;
        var_guard323_rv = 0.0;

        let assign10900_e14731: f64 = if (((p.p9 == 1.0) || (p.p9 == 2.0)) || (p.p9 == 5.0)) { 1.0 } else { 0.0 };
        var_guard324 = assign10900_e14731;
        var_guard324_rv = 0.0;

        let assign10910_e14742: f64 = if (((p.p9 == 3.0) || (p.p9 == 4.0)) || (p.p9 == 6.0)) { 1.0 } else { 0.0 };
        var_guard325 = assign10910_e14742;
        var_guard325_rv = 0.0;

        let assign10920_e14745: f64 = if var_nuendd == 0.0 { 1.0 } else { 0.0 };
        var_guard326 = assign10920_e14745;
        var_guard326_rv = 0.0;

        let (assign10930_e14768,) = {
    if (((((((var_guard246 == 0.0) && (var_guard247 != 0.0)) && ((var_guard256 != 0.0) && (!((var_guard254 != 0.0) || (var_guard255 != 0.0))))) && (var_guard311 == 0.0)) && (var_guard323 != 0.0)) && (var_guard324 != 0.0)) && (var_guard326 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign10930_e14768;
        var_rend_rv = 0.0;

        let (assign10940_e14798,) = {
    if (((((((var_guard246 == 0.0) && (var_guard247 != 0.0)) && ((var_guard256 != 0.0) && (!((var_guard254 != 0.0) || (var_guard255 != 0.0))))) && (var_guard311 == 0.0)) && (var_guard323 != 0.0)) && (var_guard324 != 0.0)) && (var_guard326 == 0.0)) {
        let assign10940_e14792: f64 = (p.p438 * var_dmcgeff);
        let assign10940_e14795: f64 = (var_weff * var_nuendd);
        let assign10940_e14796: f64 = (assign10940_e14792 / assign10940_e14795);
        (assign10940_e14796,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign10940_e14798;
        var_rend_rv = 0.0;

        let assign10960_e14809: f64 = (var_dmcgeff + var_dmcieff);
        let assign10960_e14812: f64 = if ((var_nuendd == 0.0) || (assign10960_e14809 == 0.0)) { 1.0 } else { 0.0 };
        var_guard328 = assign10960_e14812;
        var_guard328_rv = 0.0;

        let (assign10970_e14838,) = {
    if (((((((var_guard246 == 0.0) && (var_guard247 != 0.0)) && ((var_guard256 != 0.0) && (!((var_guard254 != 0.0) || (var_guard255 != 0.0))))) && (var_guard311 == 0.0)) && (var_guard323 != 0.0)) && ((var_guard325 != 0.0) && (var_guard324 == 0.0))) && (var_guard328 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign10970_e14838;
        var_rend_rv = 0.0;

        let (assign10980_e14875,) = {
    if (((((((var_guard246 == 0.0) && (var_guard247 != 0.0)) && ((var_guard256 != 0.0) && (!((var_guard254 != 0.0) || (var_guard255 != 0.0))))) && (var_guard311 == 0.0)) && (var_guard323 != 0.0)) && ((var_guard325 != 0.0) && (var_guard324 == 0.0))) && (var_guard328 == 0.0)) {
        let assign10980_e14865: f64 = (p.p438 * var_weff);
        let assign10980_e14868: f64 = (3.0 * var_nuendd);
        let assign10980_e14871: f64 = (var_dmcgeff + var_dmcieff);
        let assign10980_e14872: f64 = (assign10980_e14868 * assign10980_e14871);
        let assign10980_e14873: f64 = (assign10980_e14865 / assign10980_e14872);
        (assign10980_e14873,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign10980_e14875;
        var_rend_rv = 0.0;

        let (assign10990_e14899,) = {
    if ((((((var_guard246 == 0.0) && (var_guard247 != 0.0)) && ((var_guard256 != 0.0) && (!((var_guard254 != 0.0) || (var_guard255 != 0.0))))) && (var_guard311 == 0.0)) && (var_guard323 != 0.0)) && (!((var_guard324 != 0.0) || (var_guard325 != 0.0)))) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign10990_e14899;
        var_rend_rv = 0.0;

        let assign11000_e14910: f64 = if (((p.p9 == 1.0) || (p.p9 == 3.0)) || (p.p9 == 7.0)) { 1.0 } else { 0.0 };
        var_guard329 = assign11000_e14910;
        var_guard329_rv = 0.0;

        let assign11010_e14921: f64 = if (((p.p9 == 2.0) || (p.p9 == 4.0)) || (p.p9 == 8.0)) { 1.0 } else { 0.0 };
        var_guard330 = assign11010_e14921;
        var_guard330_rv = 0.0;

        let assign11020_e14924: f64 = if var_nuendd == 0.0 { 1.0 } else { 0.0 };
        var_guard331 = assign11020_e14924;
        var_guard331_rv = 0.0;

        let (assign11030_e14948,) = {
    if (((((((var_guard246 == 0.0) && (var_guard247 != 0.0)) && ((var_guard256 != 0.0) && (!((var_guard254 != 0.0) || (var_guard255 != 0.0))))) && (var_guard311 == 0.0)) && (var_guard323 == 0.0)) && (var_guard329 != 0.0)) && (var_guard331 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign11030_e14948;
        var_rend_rv = 0.0;

        let (assign11040_e14979,) = {
    if (((((((var_guard246 == 0.0) && (var_guard247 != 0.0)) && ((var_guard256 != 0.0) && (!((var_guard254 != 0.0) || (var_guard255 != 0.0))))) && (var_guard311 == 0.0)) && (var_guard323 == 0.0)) && (var_guard329 != 0.0)) && (var_guard331 == 0.0)) {
        let assign11040_e14973: f64 = (p.p438 * var_dmcgeff);
        let assign11040_e14976: f64 = (var_weff * var_nuendd);
        let assign11040_e14977: f64 = (assign11040_e14973 / assign11040_e14976);
        (assign11040_e14977,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign11040_e14979;
        var_rend_rv = 0.0;

        let assign11060_e14990: f64 = (var_dmcgeff + var_dmcieff);
        let assign11060_e14993: f64 = if ((var_nuendd == 0.0) || (assign11060_e14990 == 0.0)) { 1.0 } else { 0.0 };
        var_guard333 = assign11060_e14993;
        var_guard333_rv = 0.0;

        let (assign11070_e15020,) = {
    if (((((((var_guard246 == 0.0) && (var_guard247 != 0.0)) && ((var_guard256 != 0.0) && (!((var_guard254 != 0.0) || (var_guard255 != 0.0))))) && (var_guard311 == 0.0)) && (var_guard323 == 0.0)) && ((var_guard330 != 0.0) && (var_guard329 == 0.0))) && (var_guard333 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign11070_e15020;
        var_rend_rv = 0.0;

        let (assign11080_e15058,) = {
    if (((((((var_guard246 == 0.0) && (var_guard247 != 0.0)) && ((var_guard256 != 0.0) && (!((var_guard254 != 0.0) || (var_guard255 != 0.0))))) && (var_guard311 == 0.0)) && (var_guard323 == 0.0)) && ((var_guard330 != 0.0) && (var_guard329 == 0.0))) && (var_guard333 == 0.0)) {
        let assign11080_e15048: f64 = (p.p438 * var_weff);
        let assign11080_e15051: f64 = (3.0 * var_nuendd);
        let assign11080_e15054: f64 = (var_dmcgeff + var_dmcieff);
        let assign11080_e15055: f64 = (assign11080_e15051 * assign11080_e15054);
        let assign11080_e15056: f64 = (assign11080_e15048 / assign11080_e15055);
        (assign11080_e15056,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign11080_e15058;
        var_rend_rv = 0.0;

        let (assign11090_e15083,) = {
    if ((((((var_guard246 == 0.0) && (var_guard247 != 0.0)) && ((var_guard256 != 0.0) && (!((var_guard254 != 0.0) || (var_guard255 != 0.0))))) && (var_guard311 == 0.0)) && (var_guard323 == 0.0)) && (!((var_guard329 != 0.0) || (var_guard330 != 0.0)))) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign11090_e15083;
        var_rend_rv = 0.0;

        let assign11100_e15086: f64 = if 0.0 == 1.0 { 1.0 } else { 0.0 };
        var_guard334 = assign11100_e15086;
        var_guard334_rv = 0.0;

        let assign11110_e15089: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        var_guard335 = assign11110_e15089;
        var_guard335_rv = 0.0;

        let assign11120_e15100: f64 = if (((p.p9 == 1.0) || (p.p9 == 2.0)) || (p.p9 == 5.0)) { 1.0 } else { 0.0 };
        var_guard336 = assign11120_e15100;
        var_guard336_rv = 0.0;

        let assign11130_e15111: f64 = if (((p.p9 == 3.0) || (p.p9 == 4.0)) || (p.p9 == 6.0)) { 1.0 } else { 0.0 };
        var_guard337 = assign11130_e15111;
        var_guard337_rv = 0.0;

        let assign11140_e15114: f64 = if var_nuends == 0.0 { 1.0 } else { 0.0 };
        var_guard338 = assign11140_e15114;
        var_guard338_rv = 0.0;

        let (assign11150_e15138,) = {
    if (((((((var_guard246 == 0.0) && (var_guard247 != 0.0)) && ((var_guard257 != 0.0) && (!(((var_guard254 != 0.0) || (var_guard255 != 0.0)) || (var_guard256 != 0.0))))) && (var_guard334 != 0.0)) && (var_guard335 != 0.0)) && (var_guard336 != 0.0)) && (var_guard338 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign11150_e15138;
        var_rend_rv = 0.0;

        let (assign11160_e15169,) = {
    if (((((((var_guard246 == 0.0) && (var_guard247 != 0.0)) && ((var_guard257 != 0.0) && (!(((var_guard254 != 0.0) || (var_guard255 != 0.0)) || (var_guard256 != 0.0))))) && (var_guard334 != 0.0)) && (var_guard335 != 0.0)) && (var_guard336 != 0.0)) && (var_guard338 == 0.0)) {
        let assign11160_e15163: f64 = (p.p438 * var_dmcgeff);
        let assign11160_e15166: f64 = (var_weff * var_nuends);
        let assign11160_e15167: f64 = (assign11160_e15163 / assign11160_e15166);
        (assign11160_e15167,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign11160_e15169;
        var_rend_rv = 0.0;

        let assign11180_e15179: f64 = if ((var_nuends == 0.0) || (var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        var_guard340 = assign11180_e15179;
        var_guard340_rv = 0.0;

        let (assign11190_e15206,) = {
    if (((((((var_guard246 == 0.0) && (var_guard247 != 0.0)) && ((var_guard257 != 0.0) && (!(((var_guard254 != 0.0) || (var_guard255 != 0.0)) || (var_guard256 != 0.0))))) && (var_guard334 != 0.0)) && (var_guard335 != 0.0)) && ((var_guard337 != 0.0) && (var_guard336 == 0.0))) && (var_guard340 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign11190_e15206;
        var_rend_rv = 0.0;

        let (assign11200_e15242,) = {
    if (((((((var_guard246 == 0.0) && (var_guard247 != 0.0)) && ((var_guard257 != 0.0) && (!(((var_guard254 != 0.0) || (var_guard255 != 0.0)) || (var_guard256 != 0.0))))) && (var_guard334 != 0.0)) && (var_guard335 != 0.0)) && ((var_guard337 != 0.0) && (var_guard336 == 0.0))) && (var_guard340 == 0.0)) {
        let assign11200_e15234: f64 = (p.p438 * var_weff);
        let assign11200_e15237: f64 = (6.0 * var_nuends);
        let assign11200_e15239: f64 = (assign11200_e15237 * var_dmcgeff);
        let assign11200_e15240: f64 = (assign11200_e15234 / assign11200_e15239);
        (assign11200_e15240,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign11200_e15242;
        var_rend_rv = 0.0;

        let (assign11210_e15267,) = {
    if ((((((var_guard246 == 0.0) && (var_guard247 != 0.0)) && ((var_guard257 != 0.0) && (!(((var_guard254 != 0.0) || (var_guard255 != 0.0)) || (var_guard256 != 0.0))))) && (var_guard334 != 0.0)) && (var_guard335 != 0.0)) && (!((var_guard336 != 0.0) || (var_guard337 != 0.0)))) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign11210_e15267;
        var_rend_rv = 0.0;

        let assign11220_e15278: f64 = if (((p.p9 == 1.0) || (p.p9 == 3.0)) || (p.p9 == 7.0)) { 1.0 } else { 0.0 };
        var_guard341 = assign11220_e15278;
        var_guard341_rv = 0.0;

        let assign11230_e15289: f64 = if (((p.p9 == 2.0) || (p.p9 == 4.0)) || (p.p9 == 8.0)) { 1.0 } else { 0.0 };
        var_guard342 = assign11230_e15289;
        var_guard342_rv = 0.0;

        let assign11240_e15292: f64 = if var_nuends == 0.0 { 1.0 } else { 0.0 };
        var_guard343 = assign11240_e15292;
        var_guard343_rv = 0.0;

        let (assign11250_e15317,) = {
    if (((((((var_guard246 == 0.0) && (var_guard247 != 0.0)) && ((var_guard257 != 0.0) && (!(((var_guard254 != 0.0) || (var_guard255 != 0.0)) || (var_guard256 != 0.0))))) && (var_guard334 != 0.0)) && (var_guard335 == 0.0)) && (var_guard341 != 0.0)) && (var_guard343 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign11250_e15317;
        var_rend_rv = 0.0;

        let (assign11260_e15349,) = {
    if (((((((var_guard246 == 0.0) && (var_guard247 != 0.0)) && ((var_guard257 != 0.0) && (!(((var_guard254 != 0.0) || (var_guard255 != 0.0)) || (var_guard256 != 0.0))))) && (var_guard334 != 0.0)) && (var_guard335 == 0.0)) && (var_guard341 != 0.0)) && (var_guard343 == 0.0)) {
        let assign11260_e15343: f64 = (p.p438 * var_dmcgeff);
        let assign11260_e15346: f64 = (var_weff * var_nuends);
        let assign11260_e15347: f64 = (assign11260_e15343 / assign11260_e15346);
        (assign11260_e15347,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign11260_e15349;
        var_rend_rv = 0.0;

        let assign11280_e15359: f64 = if ((var_nuends == 0.0) || (var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        var_guard345 = assign11280_e15359;
        var_guard345_rv = 0.0;

        let (assign11290_e15387,) = {
    if (((((((var_guard246 == 0.0) && (var_guard247 != 0.0)) && ((var_guard257 != 0.0) && (!(((var_guard254 != 0.0) || (var_guard255 != 0.0)) || (var_guard256 != 0.0))))) && (var_guard334 != 0.0)) && (var_guard335 == 0.0)) && ((var_guard342 != 0.0) && (var_guard341 == 0.0))) && (var_guard345 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign11290_e15387;
        var_rend_rv = 0.0;

        let (assign11300_e15424,) = {
    if (((((((var_guard246 == 0.0) && (var_guard247 != 0.0)) && ((var_guard257 != 0.0) && (!(((var_guard254 != 0.0) || (var_guard255 != 0.0)) || (var_guard256 != 0.0))))) && (var_guard334 != 0.0)) && (var_guard335 == 0.0)) && ((var_guard342 != 0.0) && (var_guard341 == 0.0))) && (var_guard345 == 0.0)) {
        let assign11300_e15416: f64 = (p.p438 * var_weff);
        let assign11300_e15419: f64 = (6.0 * var_nuends);
        let assign11300_e15421: f64 = (assign11300_e15419 * var_dmcgeff);
        let assign11300_e15422: f64 = (assign11300_e15416 / assign11300_e15421);
        (assign11300_e15422,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign11300_e15424;
        var_rend_rv = 0.0;

        let (assign11310_e15450,) = {
    if ((((((var_guard246 == 0.0) && (var_guard247 != 0.0)) && ((var_guard257 != 0.0) && (!(((var_guard254 != 0.0) || (var_guard255 != 0.0)) || (var_guard256 != 0.0))))) && (var_guard334 != 0.0)) && (var_guard335 == 0.0)) && (!((var_guard341 != 0.0) || (var_guard342 != 0.0)))) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign11310_e15450;
        var_rend_rv = 0.0;

        let assign11320_e15453: f64 = if 0.0 == 1.0 { 1.0 } else { 0.0 };
        var_guard346 = assign11320_e15453;
        var_guard346_rv = 0.0;

        let assign11330_e15464: f64 = if (((p.p9 == 1.0) || (p.p9 == 2.0)) || (p.p9 == 5.0)) { 1.0 } else { 0.0 };
        var_guard347 = assign11330_e15464;
        var_guard347_rv = 0.0;

        let assign11340_e15475: f64 = if (((p.p9 == 3.0) || (p.p9 == 4.0)) || (p.p9 == 6.0)) { 1.0 } else { 0.0 };
        var_guard348 = assign11340_e15475;
        var_guard348_rv = 0.0;

        let assign11350_e15478: f64 = if var_nuendd == 0.0 { 1.0 } else { 0.0 };
        var_guard349 = assign11350_e15478;
        var_guard349_rv = 0.0;

        let (assign11360_e15503,) = {
    if (((((((var_guard246 == 0.0) && (var_guard247 != 0.0)) && ((var_guard257 != 0.0) && (!(((var_guard254 != 0.0) || (var_guard255 != 0.0)) || (var_guard256 != 0.0))))) && (var_guard334 == 0.0)) && (var_guard346 != 0.0)) && (var_guard347 != 0.0)) && (var_guard349 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign11360_e15503;
        var_rend_rv = 0.0;

        *var_guard311_slot = var_guard311;
        *var_guard311_rv_slot = var_guard311_rv;
        *var_guard312_slot = var_guard312;
        *var_guard312_rv_slot = var_guard312_rv;
        *var_guard313_slot = var_guard313;
        *var_guard313_rv_slot = var_guard313_rv;
        *var_guard314_slot = var_guard314;
        *var_guard314_rv_slot = var_guard314_rv;
        *var_guard315_slot = var_guard315;
        *var_guard315_rv_slot = var_guard315_rv;
        *var_guard317_slot = var_guard317;
        *var_guard317_rv_slot = var_guard317_rv;
        *var_guard318_slot = var_guard318;
        *var_guard318_rv_slot = var_guard318_rv;
        *var_guard319_slot = var_guard319;
        *var_guard319_rv_slot = var_guard319_rv;
        *var_guard320_slot = var_guard320;
        *var_guard320_rv_slot = var_guard320_rv;
        *var_guard322_slot = var_guard322;
        *var_guard322_rv_slot = var_guard322_rv;
        *var_guard323_slot = var_guard323;
        *var_guard323_rv_slot = var_guard323_rv;
        *var_guard324_slot = var_guard324;
        *var_guard324_rv_slot = var_guard324_rv;
        *var_guard325_slot = var_guard325;
        *var_guard325_rv_slot = var_guard325_rv;
        *var_guard326_slot = var_guard326;
        *var_guard326_rv_slot = var_guard326_rv;
        *var_guard328_slot = var_guard328;
        *var_guard328_rv_slot = var_guard328_rv;
        *var_guard329_slot = var_guard329;
        *var_guard329_rv_slot = var_guard329_rv;
        *var_guard330_slot = var_guard330;
        *var_guard330_rv_slot = var_guard330_rv;
        *var_guard331_slot = var_guard331;
        *var_guard331_rv_slot = var_guard331_rv;
        *var_guard333_slot = var_guard333;
        *var_guard333_rv_slot = var_guard333_rv;
        *var_guard334_slot = var_guard334;
        *var_guard334_rv_slot = var_guard334_rv;
        *var_guard335_slot = var_guard335;
        *var_guard335_rv_slot = var_guard335_rv;
        *var_guard336_slot = var_guard336;
        *var_guard336_rv_slot = var_guard336_rv;
        *var_guard337_slot = var_guard337;
        *var_guard337_rv_slot = var_guard337_rv;
        *var_guard338_slot = var_guard338;
        *var_guard338_rv_slot = var_guard338_rv;
        *var_guard340_slot = var_guard340;
        *var_guard340_rv_slot = var_guard340_rv;
        *var_guard341_slot = var_guard341;
        *var_guard341_rv_slot = var_guard341_rv;
        *var_guard342_slot = var_guard342;
        *var_guard342_rv_slot = var_guard342_rv;
        *var_guard343_slot = var_guard343;
        *var_guard343_rv_slot = var_guard343_rv;
        *var_guard345_slot = var_guard345;
        *var_guard345_rv_slot = var_guard345_rv;
        *var_guard346_slot = var_guard346;
        *var_guard346_rv_slot = var_guard346_rv;
        *var_guard347_slot = var_guard347;
        *var_guard347_rv_slot = var_guard347_rv;
        *var_guard348_slot = var_guard348;
        *var_guard348_rv_slot = var_guard348_rv;
        *var_guard349_slot = var_guard349;
        *var_guard349_rv_slot = var_guard349_rv;
        *var_rend_slot = var_rend;
        *var_rend_rv_slot = var_rend_rv;
    }

    pub(super) fn stamp_reactive_block_19(
        p: &Parameters,
        var_dmcgeff: f64,
        var_dmcieff: f64,
        var_dmdgeff: f64,
        var_guard246: f64,
        var_guard247: f64,
        var_guard254: f64,
        var_guard255: f64,
        var_guard256: f64,
        var_guard257: f64,
        var_guard258: f64,
        var_guard259: f64,
        var_guard260: f64,
        var_guard334: f64,
        var_guard346: f64,
        var_guard347: f64,
        var_guard348: f64,
        var_guard349: f64,
        var_nuendd: f64,
        var_nuends: f64,
        var_weff: f64,
        var_guard351_slot: &mut f64,
        var_guard351_rv_slot: &mut f64,
        var_guard352_slot: &mut f64,
        var_guard352_rv_slot: &mut f64,
        var_guard353_slot: &mut f64,
        var_guard353_rv_slot: &mut f64,
        var_guard354_slot: &mut f64,
        var_guard354_rv_slot: &mut f64,
        var_guard356_slot: &mut f64,
        var_guard356_rv_slot: &mut f64,
        var_guard357_slot: &mut f64,
        var_guard357_rv_slot: &mut f64,
        var_guard358_slot: &mut f64,
        var_guard358_rv_slot: &mut f64,
        var_guard359_slot: &mut f64,
        var_guard359_rv_slot: &mut f64,
        var_guard360_slot: &mut f64,
        var_guard360_rv_slot: &mut f64,
        var_guard361_slot: &mut f64,
        var_guard361_rv_slot: &mut f64,
        var_guard363_slot: &mut f64,
        var_guard363_rv_slot: &mut f64,
        var_guard364_slot: &mut f64,
        var_guard364_rv_slot: &mut f64,
        var_guard365_slot: &mut f64,
        var_guard365_rv_slot: &mut f64,
        var_guard366_slot: &mut f64,
        var_guard366_rv_slot: &mut f64,
        var_guard368_slot: &mut f64,
        var_guard368_rv_slot: &mut f64,
        var_guard369_slot: &mut f64,
        var_guard369_rv_slot: &mut f64,
        var_guard370_slot: &mut f64,
        var_guard370_rv_slot: &mut f64,
        var_guard371_slot: &mut f64,
        var_guard371_rv_slot: &mut f64,
        var_guard372_slot: &mut f64,
        var_guard372_rv_slot: &mut f64,
        var_guard373_slot: &mut f64,
        var_guard373_rv_slot: &mut f64,
        var_guard375_slot: &mut f64,
        var_guard375_rv_slot: &mut f64,
        var_guard376_slot: &mut f64,
        var_guard376_rv_slot: &mut f64,
        var_guard377_slot: &mut f64,
        var_guard377_rv_slot: &mut f64,
        var_guard378_slot: &mut f64,
        var_guard378_rv_slot: &mut f64,
        var_guard380_slot: &mut f64,
        var_guard380_rv_slot: &mut f64,
        var_guard381_slot: &mut f64,
        var_guard381_rv_slot: &mut f64,
        var_guard382_slot: &mut f64,
        var_guard382_rv_slot: &mut f64,
        var_guard383_slot: &mut f64,
        var_guard383_rv_slot: &mut f64,
        var_guard384_slot: &mut f64,
        var_guard384_rv_slot: &mut f64,
        var_guard385_slot: &mut f64,
        var_guard385_rv_slot: &mut f64,
        var_guard386_slot: &mut f64,
        var_guard386_rv_slot: &mut f64,
        var_rend_slot: &mut f64,
        var_rend_rv_slot: &mut f64,
    ) {
        let mut var_guard351: f64 = *var_guard351_slot;
        let mut var_guard351_rv: f64 = *var_guard351_rv_slot;
        let mut var_guard352: f64 = *var_guard352_slot;
        let mut var_guard352_rv: f64 = *var_guard352_rv_slot;
        let mut var_guard353: f64 = *var_guard353_slot;
        let mut var_guard353_rv: f64 = *var_guard353_rv_slot;
        let mut var_guard354: f64 = *var_guard354_slot;
        let mut var_guard354_rv: f64 = *var_guard354_rv_slot;
        let mut var_guard356: f64 = *var_guard356_slot;
        let mut var_guard356_rv: f64 = *var_guard356_rv_slot;
        let mut var_guard357: f64 = *var_guard357_slot;
        let mut var_guard357_rv: f64 = *var_guard357_rv_slot;
        let mut var_guard358: f64 = *var_guard358_slot;
        let mut var_guard358_rv: f64 = *var_guard358_rv_slot;
        let mut var_guard359: f64 = *var_guard359_slot;
        let mut var_guard359_rv: f64 = *var_guard359_rv_slot;
        let mut var_guard360: f64 = *var_guard360_slot;
        let mut var_guard360_rv: f64 = *var_guard360_rv_slot;
        let mut var_guard361: f64 = *var_guard361_slot;
        let mut var_guard361_rv: f64 = *var_guard361_rv_slot;
        let mut var_guard363: f64 = *var_guard363_slot;
        let mut var_guard363_rv: f64 = *var_guard363_rv_slot;
        let mut var_guard364: f64 = *var_guard364_slot;
        let mut var_guard364_rv: f64 = *var_guard364_rv_slot;
        let mut var_guard365: f64 = *var_guard365_slot;
        let mut var_guard365_rv: f64 = *var_guard365_rv_slot;
        let mut var_guard366: f64 = *var_guard366_slot;
        let mut var_guard366_rv: f64 = *var_guard366_rv_slot;
        let mut var_guard368: f64 = *var_guard368_slot;
        let mut var_guard368_rv: f64 = *var_guard368_rv_slot;
        let mut var_guard369: f64 = *var_guard369_slot;
        let mut var_guard369_rv: f64 = *var_guard369_rv_slot;
        let mut var_guard370: f64 = *var_guard370_slot;
        let mut var_guard370_rv: f64 = *var_guard370_rv_slot;
        let mut var_guard371: f64 = *var_guard371_slot;
        let mut var_guard371_rv: f64 = *var_guard371_rv_slot;
        let mut var_guard372: f64 = *var_guard372_slot;
        let mut var_guard372_rv: f64 = *var_guard372_rv_slot;
        let mut var_guard373: f64 = *var_guard373_slot;
        let mut var_guard373_rv: f64 = *var_guard373_rv_slot;
        let mut var_guard375: f64 = *var_guard375_slot;
        let mut var_guard375_rv: f64 = *var_guard375_rv_slot;
        let mut var_guard376: f64 = *var_guard376_slot;
        let mut var_guard376_rv: f64 = *var_guard376_rv_slot;
        let mut var_guard377: f64 = *var_guard377_slot;
        let mut var_guard377_rv: f64 = *var_guard377_rv_slot;
        let mut var_guard378: f64 = *var_guard378_slot;
        let mut var_guard378_rv: f64 = *var_guard378_rv_slot;
        let mut var_guard380: f64 = *var_guard380_slot;
        let mut var_guard380_rv: f64 = *var_guard380_rv_slot;
        let mut var_guard381: f64 = *var_guard381_slot;
        let mut var_guard381_rv: f64 = *var_guard381_rv_slot;
        let mut var_guard382: f64 = *var_guard382_slot;
        let mut var_guard382_rv: f64 = *var_guard382_rv_slot;
        let mut var_guard383: f64 = *var_guard383_slot;
        let mut var_guard383_rv: f64 = *var_guard383_rv_slot;
        let mut var_guard384: f64 = *var_guard384_slot;
        let mut var_guard384_rv: f64 = *var_guard384_rv_slot;
        let mut var_guard385: f64 = *var_guard385_slot;
        let mut var_guard385_rv: f64 = *var_guard385_rv_slot;
        let mut var_guard386: f64 = *var_guard386_slot;
        let mut var_guard386_rv: f64 = *var_guard386_rv_slot;
        let mut var_rend: f64 = *var_rend_slot;
        let mut var_rend_rv: f64 = *var_rend_rv_slot;

        let (assign11370_e15535,) = {
    if (((((((var_guard246 == 0.0) && (var_guard247 != 0.0)) && ((var_guard257 != 0.0) && (!(((var_guard254 != 0.0) || (var_guard255 != 0.0)) || (var_guard256 != 0.0))))) && (var_guard334 == 0.0)) && (var_guard346 != 0.0)) && (var_guard347 != 0.0)) && (var_guard349 == 0.0)) {
        let assign11370_e15529: f64 = (p.p438 * var_dmcgeff);
        let assign11370_e15532: f64 = (var_weff * var_nuendd);
        let assign11370_e15533: f64 = (assign11370_e15529 / assign11370_e15532);
        (assign11370_e15533,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign11370_e15535;
        var_rend_rv = 0.0;

        let assign11390_e15545: f64 = if ((var_nuendd == 0.0) || (var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        var_guard351 = assign11390_e15545;
        var_guard351_rv = 0.0;

        let (assign11400_e15573,) = {
    if (((((((var_guard246 == 0.0) && (var_guard247 != 0.0)) && ((var_guard257 != 0.0) && (!(((var_guard254 != 0.0) || (var_guard255 != 0.0)) || (var_guard256 != 0.0))))) && (var_guard334 == 0.0)) && (var_guard346 != 0.0)) && ((var_guard348 != 0.0) && (var_guard347 == 0.0))) && (var_guard351 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign11400_e15573;
        var_rend_rv = 0.0;

        let (assign11410_e15610,) = {
    if (((((((var_guard246 == 0.0) && (var_guard247 != 0.0)) && ((var_guard257 != 0.0) && (!(((var_guard254 != 0.0) || (var_guard255 != 0.0)) || (var_guard256 != 0.0))))) && (var_guard334 == 0.0)) && (var_guard346 != 0.0)) && ((var_guard348 != 0.0) && (var_guard347 == 0.0))) && (var_guard351 == 0.0)) {
        let assign11410_e15602: f64 = (p.p438 * var_weff);
        let assign11410_e15605: f64 = (6.0 * var_nuendd);
        let assign11410_e15607: f64 = (assign11410_e15605 * var_dmcgeff);
        let assign11410_e15608: f64 = (assign11410_e15602 / assign11410_e15607);
        (assign11410_e15608,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign11410_e15610;
        var_rend_rv = 0.0;

        let (assign11420_e15636,) = {
    if ((((((var_guard246 == 0.0) && (var_guard247 != 0.0)) && ((var_guard257 != 0.0) && (!(((var_guard254 != 0.0) || (var_guard255 != 0.0)) || (var_guard256 != 0.0))))) && (var_guard334 == 0.0)) && (var_guard346 != 0.0)) && (!((var_guard347 != 0.0) || (var_guard348 != 0.0)))) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign11420_e15636;
        var_rend_rv = 0.0;

        let assign11430_e15647: f64 = if (((p.p9 == 1.0) || (p.p9 == 3.0)) || (p.p9 == 7.0)) { 1.0 } else { 0.0 };
        var_guard352 = assign11430_e15647;
        var_guard352_rv = 0.0;

        let assign11440_e15658: f64 = if (((p.p9 == 2.0) || (p.p9 == 4.0)) || (p.p9 == 8.0)) { 1.0 } else { 0.0 };
        var_guard353 = assign11440_e15658;
        var_guard353_rv = 0.0;

        let assign11450_e15661: f64 = if var_nuendd == 0.0 { 1.0 } else { 0.0 };
        var_guard354 = assign11450_e15661;
        var_guard354_rv = 0.0;

        let (assign11460_e15687,) = {
    if (((((((var_guard246 == 0.0) && (var_guard247 != 0.0)) && ((var_guard257 != 0.0) && (!(((var_guard254 != 0.0) || (var_guard255 != 0.0)) || (var_guard256 != 0.0))))) && (var_guard334 == 0.0)) && (var_guard346 == 0.0)) && (var_guard352 != 0.0)) && (var_guard354 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign11460_e15687;
        var_rend_rv = 0.0;

        let (assign11470_e15720,) = {
    if (((((((var_guard246 == 0.0) && (var_guard247 != 0.0)) && ((var_guard257 != 0.0) && (!(((var_guard254 != 0.0) || (var_guard255 != 0.0)) || (var_guard256 != 0.0))))) && (var_guard334 == 0.0)) && (var_guard346 == 0.0)) && (var_guard352 != 0.0)) && (var_guard354 == 0.0)) {
        let assign11470_e15714: f64 = (p.p438 * var_dmcgeff);
        let assign11470_e15717: f64 = (var_weff * var_nuendd);
        let assign11470_e15718: f64 = (assign11470_e15714 / assign11470_e15717);
        (assign11470_e15718,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign11470_e15720;
        var_rend_rv = 0.0;

        let assign11490_e15730: f64 = if ((var_nuendd == 0.0) || (var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        var_guard356 = assign11490_e15730;
        var_guard356_rv = 0.0;

        let (assign11500_e15759,) = {
    if (((((((var_guard246 == 0.0) && (var_guard247 != 0.0)) && ((var_guard257 != 0.0) && (!(((var_guard254 != 0.0) || (var_guard255 != 0.0)) || (var_guard256 != 0.0))))) && (var_guard334 == 0.0)) && (var_guard346 == 0.0)) && ((var_guard353 != 0.0) && (var_guard352 == 0.0))) && (var_guard356 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign11500_e15759;
        var_rend_rv = 0.0;

        let (assign11510_e15797,) = {
    if (((((((var_guard246 == 0.0) && (var_guard247 != 0.0)) && ((var_guard257 != 0.0) && (!(((var_guard254 != 0.0) || (var_guard255 != 0.0)) || (var_guard256 != 0.0))))) && (var_guard334 == 0.0)) && (var_guard346 == 0.0)) && ((var_guard353 != 0.0) && (var_guard352 == 0.0))) && (var_guard356 == 0.0)) {
        let assign11510_e15789: f64 = (p.p438 * var_weff);
        let assign11510_e15792: f64 = (6.0 * var_nuendd);
        let assign11510_e15794: f64 = (assign11510_e15792 * var_dmcgeff);
        let assign11510_e15795: f64 = (assign11510_e15789 / assign11510_e15794);
        (assign11510_e15795,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign11510_e15797;
        var_rend_rv = 0.0;

        let (assign11520_e15824,) = {
    if ((((((var_guard246 == 0.0) && (var_guard247 != 0.0)) && ((var_guard257 != 0.0) && (!(((var_guard254 != 0.0) || (var_guard255 != 0.0)) || (var_guard256 != 0.0))))) && (var_guard334 == 0.0)) && (var_guard346 == 0.0)) && (!((var_guard352 != 0.0) || (var_guard353 != 0.0)))) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign11520_e15824;
        var_rend_rv = 0.0;

        let assign11530_e15827: f64 = if 0.0 == 1.0 { 1.0 } else { 0.0 };
        var_guard357 = assign11530_e15827;
        var_guard357_rv = 0.0;

        let assign11540_e15830: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        var_guard358 = assign11540_e15830;
        var_guard358_rv = 0.0;

        let assign11550_e15841: f64 = if (((p.p9 == 1.0) || (p.p9 == 2.0)) || (p.p9 == 5.0)) { 1.0 } else { 0.0 };
        var_guard359 = assign11550_e15841;
        var_guard359_rv = 0.0;

        let assign11560_e15852: f64 = if (((p.p9 == 3.0) || (p.p9 == 4.0)) || (p.p9 == 6.0)) { 1.0 } else { 0.0 };
        var_guard360 = assign11560_e15852;
        var_guard360_rv = 0.0;

        let assign11570_e15855: f64 = if var_nuends == 0.0 { 1.0 } else { 0.0 };
        var_guard361 = assign11570_e15855;
        var_guard361_rv = 0.0;

        let (assign11580_e15881,) = {
    if (((((((var_guard246 == 0.0) && (var_guard247 != 0.0)) && ((var_guard258 != 0.0) && (!((((var_guard254 != 0.0) || (var_guard255 != 0.0)) || (var_guard256 != 0.0)) || (var_guard257 != 0.0))))) && (var_guard357 != 0.0)) && (var_guard358 != 0.0)) && (var_guard359 != 0.0)) && (var_guard361 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign11580_e15881;
        var_rend_rv = 0.0;

        let (assign11590_e15914,) = {
    if (((((((var_guard246 == 0.0) && (var_guard247 != 0.0)) && ((var_guard258 != 0.0) && (!((((var_guard254 != 0.0) || (var_guard255 != 0.0)) || (var_guard256 != 0.0)) || (var_guard257 != 0.0))))) && (var_guard357 != 0.0)) && (var_guard358 != 0.0)) && (var_guard359 != 0.0)) && (var_guard361 == 0.0)) {
        let assign11590_e15908: f64 = (p.p438 * var_dmcgeff);
        let assign11590_e15911: f64 = (var_weff * var_nuends);
        let assign11590_e15912: f64 = (assign11590_e15908 / assign11590_e15911);
        (assign11590_e15912,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign11590_e15914;
        var_rend_rv = 0.0;

        let assign11610_e15925: f64 = (var_dmcgeff + var_dmcieff);
        let assign11610_e15928: f64 = if ((var_nuends == 0.0) || (assign11610_e15925 == 0.0)) { 1.0 } else { 0.0 };
        var_guard363 = assign11610_e15928;
        var_guard363_rv = 0.0;

        let (assign11620_e15957,) = {
    if (((((((var_guard246 == 0.0) && (var_guard247 != 0.0)) && ((var_guard258 != 0.0) && (!((((var_guard254 != 0.0) || (var_guard255 != 0.0)) || (var_guard256 != 0.0)) || (var_guard257 != 0.0))))) && (var_guard357 != 0.0)) && (var_guard358 != 0.0)) && ((var_guard360 != 0.0) && (var_guard359 == 0.0))) && (var_guard363 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign11620_e15957;
        var_rend_rv = 0.0;

        let (assign11630_e15997,) = {
    if (((((((var_guard246 == 0.0) && (var_guard247 != 0.0)) && ((var_guard258 != 0.0) && (!((((var_guard254 != 0.0) || (var_guard255 != 0.0)) || (var_guard256 != 0.0)) || (var_guard257 != 0.0))))) && (var_guard357 != 0.0)) && (var_guard358 != 0.0)) && ((var_guard360 != 0.0) && (var_guard359 == 0.0))) && (var_guard363 == 0.0)) {
        let assign11630_e15987: f64 = (p.p438 * var_weff);
        let assign11630_e15990: f64 = (3.0 * var_nuends);
        let assign11630_e15993: f64 = (var_dmcgeff + var_dmcieff);
        let assign11630_e15994: f64 = (assign11630_e15990 * assign11630_e15993);
        let assign11630_e15995: f64 = (assign11630_e15987 / assign11630_e15994);
        (assign11630_e15995,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign11630_e15997;
        var_rend_rv = 0.0;

        let (assign11640_e16024,) = {
    if ((((((var_guard246 == 0.0) && (var_guard247 != 0.0)) && ((var_guard258 != 0.0) && (!((((var_guard254 != 0.0) || (var_guard255 != 0.0)) || (var_guard256 != 0.0)) || (var_guard257 != 0.0))))) && (var_guard357 != 0.0)) && (var_guard358 != 0.0)) && (!((var_guard359 != 0.0) || (var_guard360 != 0.0)))) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign11640_e16024;
        var_rend_rv = 0.0;

        let assign11650_e16035: f64 = if (((p.p9 == 1.0) || (p.p9 == 3.0)) || (p.p9 == 7.0)) { 1.0 } else { 0.0 };
        var_guard364 = assign11650_e16035;
        var_guard364_rv = 0.0;

        let assign11660_e16046: f64 = if (((p.p9 == 2.0) || (p.p9 == 4.0)) || (p.p9 == 8.0)) { 1.0 } else { 0.0 };
        var_guard365 = assign11660_e16046;
        var_guard365_rv = 0.0;

        let assign11670_e16049: f64 = if var_nuends == 0.0 { 1.0 } else { 0.0 };
        var_guard366 = assign11670_e16049;
        var_guard366_rv = 0.0;

        let (assign11680_e16076,) = {
    if (((((((var_guard246 == 0.0) && (var_guard247 != 0.0)) && ((var_guard258 != 0.0) && (!((((var_guard254 != 0.0) || (var_guard255 != 0.0)) || (var_guard256 != 0.0)) || (var_guard257 != 0.0))))) && (var_guard357 != 0.0)) && (var_guard358 == 0.0)) && (var_guard364 != 0.0)) && (var_guard366 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign11680_e16076;
        var_rend_rv = 0.0;

        let (assign11690_e16110,) = {
    if (((((((var_guard246 == 0.0) && (var_guard247 != 0.0)) && ((var_guard258 != 0.0) && (!((((var_guard254 != 0.0) || (var_guard255 != 0.0)) || (var_guard256 != 0.0)) || (var_guard257 != 0.0))))) && (var_guard357 != 0.0)) && (var_guard358 == 0.0)) && (var_guard364 != 0.0)) && (var_guard366 == 0.0)) {
        let assign11690_e16104: f64 = (p.p438 * var_dmcgeff);
        let assign11690_e16107: f64 = (var_weff * var_nuends);
        let assign11690_e16108: f64 = (assign11690_e16104 / assign11690_e16107);
        (assign11690_e16108,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign11690_e16110;
        var_rend_rv = 0.0;

        let assign11710_e16121: f64 = (var_dmcgeff + var_dmcieff);
        let assign11710_e16124: f64 = if ((var_nuends == 0.0) || (assign11710_e16121 == 0.0)) { 1.0 } else { 0.0 };
        var_guard368 = assign11710_e16124;
        var_guard368_rv = 0.0;

        let (assign11720_e16154,) = {
    if (((((((var_guard246 == 0.0) && (var_guard247 != 0.0)) && ((var_guard258 != 0.0) && (!((((var_guard254 != 0.0) || (var_guard255 != 0.0)) || (var_guard256 != 0.0)) || (var_guard257 != 0.0))))) && (var_guard357 != 0.0)) && (var_guard358 == 0.0)) && ((var_guard365 != 0.0) && (var_guard364 == 0.0))) && (var_guard368 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign11720_e16154;
        var_rend_rv = 0.0;

        let (assign11730_e16195,) = {
    if (((((((var_guard246 == 0.0) && (var_guard247 != 0.0)) && ((var_guard258 != 0.0) && (!((((var_guard254 != 0.0) || (var_guard255 != 0.0)) || (var_guard256 != 0.0)) || (var_guard257 != 0.0))))) && (var_guard357 != 0.0)) && (var_guard358 == 0.0)) && ((var_guard365 != 0.0) && (var_guard364 == 0.0))) && (var_guard368 == 0.0)) {
        let assign11730_e16185: f64 = (p.p438 * var_weff);
        let assign11730_e16188: f64 = (3.0 * var_nuends);
        let assign11730_e16191: f64 = (var_dmcgeff + var_dmcieff);
        let assign11730_e16192: f64 = (assign11730_e16188 * assign11730_e16191);
        let assign11730_e16193: f64 = (assign11730_e16185 / assign11730_e16192);
        (assign11730_e16193,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign11730_e16195;
        var_rend_rv = 0.0;

        let (assign11740_e16223,) = {
    if ((((((var_guard246 == 0.0) && (var_guard247 != 0.0)) && ((var_guard258 != 0.0) && (!((((var_guard254 != 0.0) || (var_guard255 != 0.0)) || (var_guard256 != 0.0)) || (var_guard257 != 0.0))))) && (var_guard357 != 0.0)) && (var_guard358 == 0.0)) && (!((var_guard364 != 0.0) || (var_guard365 != 0.0)))) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign11740_e16223;
        var_rend_rv = 0.0;

        let (assign11750_e16248,) = {
    if ((((var_guard246 == 0.0) && (var_guard247 != 0.0)) && ((var_guard258 != 0.0) && (!((((var_guard254 != 0.0) || (var_guard255 != 0.0)) || (var_guard256 != 0.0)) || (var_guard257 != 0.0))))) && (var_guard357 == 0.0)) {
        let assign11750_e16244: f64 = (p.p438 * var_dmdgeff);
        let assign11750_e16246: f64 = (assign11750_e16244 / var_weff);
        (assign11750_e16246,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign11750_e16248;
        var_rend_rv = 0.0;

        let assign11760_e16251: f64 = if 0.0 == 1.0 { 1.0 } else { 0.0 };
        var_guard369 = assign11760_e16251;
        var_guard369_rv = 0.0;

        let assign11770_e16254: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        var_guard370 = assign11770_e16254;
        var_guard370_rv = 0.0;

        let assign11780_e16265: f64 = if (((p.p9 == 1.0) || (p.p9 == 2.0)) || (p.p9 == 5.0)) { 1.0 } else { 0.0 };
        var_guard371 = assign11780_e16265;
        var_guard371_rv = 0.0;

        let assign11790_e16276: f64 = if (((p.p9 == 3.0) || (p.p9 == 4.0)) || (p.p9 == 6.0)) { 1.0 } else { 0.0 };
        var_guard372 = assign11790_e16276;
        var_guard372_rv = 0.0;

        let assign11800_e16279: f64 = if var_nuends == 0.0 { 1.0 } else { 0.0 };
        var_guard373 = assign11800_e16279;
        var_guard373_rv = 0.0;

        let (assign11810_e16307,) = {
    if (((((((var_guard246 == 0.0) && (var_guard247 != 0.0)) && ((var_guard259 != 0.0) && (!(((((var_guard254 != 0.0) || (var_guard255 != 0.0)) || (var_guard256 != 0.0)) || (var_guard257 != 0.0)) || (var_guard258 != 0.0))))) && (var_guard369 != 0.0)) && (var_guard370 != 0.0)) && (var_guard371 != 0.0)) && (var_guard373 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign11810_e16307;
        var_rend_rv = 0.0;

        let (assign11820_e16342,) = {
    if (((((((var_guard246 == 0.0) && (var_guard247 != 0.0)) && ((var_guard259 != 0.0) && (!(((((var_guard254 != 0.0) || (var_guard255 != 0.0)) || (var_guard256 != 0.0)) || (var_guard257 != 0.0)) || (var_guard258 != 0.0))))) && (var_guard369 != 0.0)) && (var_guard370 != 0.0)) && (var_guard371 != 0.0)) && (var_guard373 == 0.0)) {
        let assign11820_e16336: f64 = (p.p438 * var_dmcgeff);
        let assign11820_e16339: f64 = (var_weff * var_nuends);
        let assign11820_e16340: f64 = (assign11820_e16336 / assign11820_e16339);
        (assign11820_e16340,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign11820_e16342;
        var_rend_rv = 0.0;

        let assign11840_e16352: f64 = if ((var_nuends == 0.0) || (var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        var_guard375 = assign11840_e16352;
        var_guard375_rv = 0.0;

        let (assign11850_e16383,) = {
    if (((((((var_guard246 == 0.0) && (var_guard247 != 0.0)) && ((var_guard259 != 0.0) && (!(((((var_guard254 != 0.0) || (var_guard255 != 0.0)) || (var_guard256 != 0.0)) || (var_guard257 != 0.0)) || (var_guard258 != 0.0))))) && (var_guard369 != 0.0)) && (var_guard370 != 0.0)) && ((var_guard372 != 0.0) && (var_guard371 == 0.0))) && (var_guard375 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign11850_e16383;
        var_rend_rv = 0.0;

        let (assign11860_e16423,) = {
    if (((((((var_guard246 == 0.0) && (var_guard247 != 0.0)) && ((var_guard259 != 0.0) && (!(((((var_guard254 != 0.0) || (var_guard255 != 0.0)) || (var_guard256 != 0.0)) || (var_guard257 != 0.0)) || (var_guard258 != 0.0))))) && (var_guard369 != 0.0)) && (var_guard370 != 0.0)) && ((var_guard372 != 0.0) && (var_guard371 == 0.0))) && (var_guard375 == 0.0)) {
        let assign11860_e16415: f64 = (p.p438 * var_weff);
        let assign11860_e16418: f64 = (6.0 * var_nuends);
        let assign11860_e16420: f64 = (assign11860_e16418 * var_dmcgeff);
        let assign11860_e16421: f64 = (assign11860_e16415 / assign11860_e16420);
        (assign11860_e16421,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign11860_e16423;
        var_rend_rv = 0.0;

        let (assign11870_e16452,) = {
    if ((((((var_guard246 == 0.0) && (var_guard247 != 0.0)) && ((var_guard259 != 0.0) && (!(((((var_guard254 != 0.0) || (var_guard255 != 0.0)) || (var_guard256 != 0.0)) || (var_guard257 != 0.0)) || (var_guard258 != 0.0))))) && (var_guard369 != 0.0)) && (var_guard370 != 0.0)) && (!((var_guard371 != 0.0) || (var_guard372 != 0.0)))) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign11870_e16452;
        var_rend_rv = 0.0;

        let assign11880_e16463: f64 = if (((p.p9 == 1.0) || (p.p9 == 3.0)) || (p.p9 == 7.0)) { 1.0 } else { 0.0 };
        var_guard376 = assign11880_e16463;
        var_guard376_rv = 0.0;

        let assign11890_e16474: f64 = if (((p.p9 == 2.0) || (p.p9 == 4.0)) || (p.p9 == 8.0)) { 1.0 } else { 0.0 };
        var_guard377 = assign11890_e16474;
        var_guard377_rv = 0.0;

        let assign11900_e16477: f64 = if var_nuends == 0.0 { 1.0 } else { 0.0 };
        var_guard378 = assign11900_e16477;
        var_guard378_rv = 0.0;

        let (assign11910_e16506,) = {
    if (((((((var_guard246 == 0.0) && (var_guard247 != 0.0)) && ((var_guard259 != 0.0) && (!(((((var_guard254 != 0.0) || (var_guard255 != 0.0)) || (var_guard256 != 0.0)) || (var_guard257 != 0.0)) || (var_guard258 != 0.0))))) && (var_guard369 != 0.0)) && (var_guard370 == 0.0)) && (var_guard376 != 0.0)) && (var_guard378 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign11910_e16506;
        var_rend_rv = 0.0;

        let (assign11920_e16542,) = {
    if (((((((var_guard246 == 0.0) && (var_guard247 != 0.0)) && ((var_guard259 != 0.0) && (!(((((var_guard254 != 0.0) || (var_guard255 != 0.0)) || (var_guard256 != 0.0)) || (var_guard257 != 0.0)) || (var_guard258 != 0.0))))) && (var_guard369 != 0.0)) && (var_guard370 == 0.0)) && (var_guard376 != 0.0)) && (var_guard378 == 0.0)) {
        let assign11920_e16536: f64 = (p.p438 * var_dmcgeff);
        let assign11920_e16539: f64 = (var_weff * var_nuends);
        let assign11920_e16540: f64 = (assign11920_e16536 / assign11920_e16539);
        (assign11920_e16540,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign11920_e16542;
        var_rend_rv = 0.0;

        let assign11940_e16552: f64 = if ((var_nuends == 0.0) || (var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        var_guard380 = assign11940_e16552;
        var_guard380_rv = 0.0;

        let (assign11950_e16584,) = {
    if (((((((var_guard246 == 0.0) && (var_guard247 != 0.0)) && ((var_guard259 != 0.0) && (!(((((var_guard254 != 0.0) || (var_guard255 != 0.0)) || (var_guard256 != 0.0)) || (var_guard257 != 0.0)) || (var_guard258 != 0.0))))) && (var_guard369 != 0.0)) && (var_guard370 == 0.0)) && ((var_guard377 != 0.0) && (var_guard376 == 0.0))) && (var_guard380 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign11950_e16584;
        var_rend_rv = 0.0;

        let (assign11960_e16625,) = {
    if (((((((var_guard246 == 0.0) && (var_guard247 != 0.0)) && ((var_guard259 != 0.0) && (!(((((var_guard254 != 0.0) || (var_guard255 != 0.0)) || (var_guard256 != 0.0)) || (var_guard257 != 0.0)) || (var_guard258 != 0.0))))) && (var_guard369 != 0.0)) && (var_guard370 == 0.0)) && ((var_guard377 != 0.0) && (var_guard376 == 0.0))) && (var_guard380 == 0.0)) {
        let assign11960_e16617: f64 = (p.p438 * var_weff);
        let assign11960_e16620: f64 = (6.0 * var_nuends);
        let assign11960_e16622: f64 = (assign11960_e16620 * var_dmcgeff);
        let assign11960_e16623: f64 = (assign11960_e16617 / assign11960_e16622);
        (assign11960_e16623,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign11960_e16625;
        var_rend_rv = 0.0;

        let (assign11970_e16655,) = {
    if ((((((var_guard246 == 0.0) && (var_guard247 != 0.0)) && ((var_guard259 != 0.0) && (!(((((var_guard254 != 0.0) || (var_guard255 != 0.0)) || (var_guard256 != 0.0)) || (var_guard257 != 0.0)) || (var_guard258 != 0.0))))) && (var_guard369 != 0.0)) && (var_guard370 == 0.0)) && (!((var_guard376 != 0.0) || (var_guard377 != 0.0)))) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign11970_e16655;
        var_rend_rv = 0.0;

        let assign11980_e16658: f64 = if var_nuendd == 0.0 { 1.0 } else { 0.0 };
        var_guard381 = assign11980_e16658;
        var_guard381_rv = 0.0;

        let (assign11990_e16683,) = {
    if (((((var_guard246 == 0.0) && (var_guard247 != 0.0)) && ((var_guard259 != 0.0) && (!(((((var_guard254 != 0.0) || (var_guard255 != 0.0)) || (var_guard256 != 0.0)) || (var_guard257 != 0.0)) || (var_guard258 != 0.0))))) && (var_guard369 == 0.0)) && (var_guard381 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign11990_e16683;
        var_rend_rv = 0.0;

        let (assign12000_e16715,) = {
    if (((((var_guard246 == 0.0) && (var_guard247 != 0.0)) && ((var_guard259 != 0.0) && (!(((((var_guard254 != 0.0) || (var_guard255 != 0.0)) || (var_guard256 != 0.0)) || (var_guard257 != 0.0)) || (var_guard258 != 0.0))))) && (var_guard369 == 0.0)) && (var_guard381 == 0.0)) {
        let assign12000_e16709: f64 = (p.p438 * var_dmdgeff);
        let assign12000_e16712: f64 = (var_weff * var_nuendd);
        let assign12000_e16713: f64 = (assign12000_e16709 / assign12000_e16712);
        (assign12000_e16713,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign12000_e16715;
        var_rend_rv = 0.0;

        let assign12010_e16718: f64 = if 0.0 == 1.0 { 1.0 } else { 0.0 };
        var_guard382 = assign12010_e16718;
        var_guard382_rv = 0.0;

        let (assign12020_e16746,) = {
    if ((((var_guard246 == 0.0) && (var_guard247 != 0.0)) && ((var_guard260 != 0.0) && (!((((((var_guard254 != 0.0) || (var_guard255 != 0.0)) || (var_guard256 != 0.0)) || (var_guard257 != 0.0)) || (var_guard258 != 0.0)) || (var_guard259 != 0.0))))) && (var_guard382 != 0.0)) {
        let assign12020_e16742: f64 = (p.p438 * var_dmdgeff);
        let assign12020_e16744: f64 = (assign12020_e16742 / var_weff);
        (assign12020_e16744,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign12020_e16746;
        var_rend_rv = 0.0;

        let assign12030_e16749: f64 = if 0.0 == 1.0 { 1.0 } else { 0.0 };
        var_guard383 = assign12030_e16749;
        var_guard383_rv = 0.0;

        let assign12040_e16760: f64 = if (((p.p9 == 1.0) || (p.p9 == 2.0)) || (p.p9 == 5.0)) { 1.0 } else { 0.0 };
        var_guard384 = assign12040_e16760;
        var_guard384_rv = 0.0;

        let assign12050_e16771: f64 = if (((p.p9 == 3.0) || (p.p9 == 4.0)) || (p.p9 == 6.0)) { 1.0 } else { 0.0 };
        var_guard385 = assign12050_e16771;
        var_guard385_rv = 0.0;

        let assign12060_e16774: f64 = if var_nuendd == 0.0 { 1.0 } else { 0.0 };
        var_guard386 = assign12060_e16774;
        var_guard386_rv = 0.0;

        *var_guard351_slot = var_guard351;
        *var_guard351_rv_slot = var_guard351_rv;
        *var_guard352_slot = var_guard352;
        *var_guard352_rv_slot = var_guard352_rv;
        *var_guard353_slot = var_guard353;
        *var_guard353_rv_slot = var_guard353_rv;
        *var_guard354_slot = var_guard354;
        *var_guard354_rv_slot = var_guard354_rv;
        *var_guard356_slot = var_guard356;
        *var_guard356_rv_slot = var_guard356_rv;
        *var_guard357_slot = var_guard357;
        *var_guard357_rv_slot = var_guard357_rv;
        *var_guard358_slot = var_guard358;
        *var_guard358_rv_slot = var_guard358_rv;
        *var_guard359_slot = var_guard359;
        *var_guard359_rv_slot = var_guard359_rv;
        *var_guard360_slot = var_guard360;
        *var_guard360_rv_slot = var_guard360_rv;
        *var_guard361_slot = var_guard361;
        *var_guard361_rv_slot = var_guard361_rv;
        *var_guard363_slot = var_guard363;
        *var_guard363_rv_slot = var_guard363_rv;
        *var_guard364_slot = var_guard364;
        *var_guard364_rv_slot = var_guard364_rv;
        *var_guard365_slot = var_guard365;
        *var_guard365_rv_slot = var_guard365_rv;
        *var_guard366_slot = var_guard366;
        *var_guard366_rv_slot = var_guard366_rv;
        *var_guard368_slot = var_guard368;
        *var_guard368_rv_slot = var_guard368_rv;
        *var_guard369_slot = var_guard369;
        *var_guard369_rv_slot = var_guard369_rv;
        *var_guard370_slot = var_guard370;
        *var_guard370_rv_slot = var_guard370_rv;
        *var_guard371_slot = var_guard371;
        *var_guard371_rv_slot = var_guard371_rv;
        *var_guard372_slot = var_guard372;
        *var_guard372_rv_slot = var_guard372_rv;
        *var_guard373_slot = var_guard373;
        *var_guard373_rv_slot = var_guard373_rv;
        *var_guard375_slot = var_guard375;
        *var_guard375_rv_slot = var_guard375_rv;
        *var_guard376_slot = var_guard376;
        *var_guard376_rv_slot = var_guard376_rv;
        *var_guard377_slot = var_guard377;
        *var_guard377_rv_slot = var_guard377_rv;
        *var_guard378_slot = var_guard378;
        *var_guard378_rv_slot = var_guard378_rv;
        *var_guard380_slot = var_guard380;
        *var_guard380_rv_slot = var_guard380_rv;
        *var_guard381_slot = var_guard381;
        *var_guard381_rv_slot = var_guard381_rv;
        *var_guard382_slot = var_guard382;
        *var_guard382_rv_slot = var_guard382_rv;
        *var_guard383_slot = var_guard383;
        *var_guard383_rv_slot = var_guard383_rv;
        *var_guard384_slot = var_guard384;
        *var_guard384_rv_slot = var_guard384_rv;
        *var_guard385_slot = var_guard385;
        *var_guard385_rv_slot = var_guard385_rv;
        *var_guard386_slot = var_guard386;
        *var_guard386_rv_slot = var_guard386_rv;
        *var_rend_slot = var_rend;
        *var_rend_rv_slot = var_rend_rv;
    }

    pub(super) fn stamp_reactive_block_20(
        p: &Parameters,
        var_dmcgeff: f64,
        var_dmcieff: f64,
        var_dmdgeff: f64,
        var_guard246: f64,
        var_guard247: f64,
        var_guard254: f64,
        var_guard255: f64,
        var_guard256: f64,
        var_guard257: f64,
        var_guard258: f64,
        var_guard259: f64,
        var_guard260: f64,
        var_guard261: f64,
        var_guard262: f64,
        var_guard263: f64,
        var_guard264: f64,
        var_guard382: f64,
        var_guard383: f64,
        var_guard384: f64,
        var_guard385: f64,
        var_guard386: f64,
        var_nuendd: f64,
        var_nuends: f64,
        var_weff: f64,
        var_guard388_slot: &mut f64,
        var_guard388_rv_slot: &mut f64,
        var_guard389_slot: &mut f64,
        var_guard389_rv_slot: &mut f64,
        var_guard390_slot: &mut f64,
        var_guard390_rv_slot: &mut f64,
        var_guard391_slot: &mut f64,
        var_guard391_rv_slot: &mut f64,
        var_guard393_slot: &mut f64,
        var_guard393_rv_slot: &mut f64,
        var_guard394_slot: &mut f64,
        var_guard394_rv_slot: &mut f64,
        var_guard395_slot: &mut f64,
        var_guard395_rv_slot: &mut f64,
        var_guard396_slot: &mut f64,
        var_guard396_rv_slot: &mut f64,
        var_guard397_slot: &mut f64,
        var_guard397_rv_slot: &mut f64,
        var_guard398_slot: &mut f64,
        var_guard398_rv_slot: &mut f64,
        var_guard399_slot: &mut f64,
        var_guard399_rv_slot: &mut f64,
        var_guard401_slot: &mut f64,
        var_guard401_rv_slot: &mut f64,
        var_guard402_slot: &mut f64,
        var_guard402_rv_slot: &mut f64,
        var_guard403_slot: &mut f64,
        var_guard403_rv_slot: &mut f64,
        var_guard404_slot: &mut f64,
        var_guard404_rv_slot: &mut f64,
        var_guard406_slot: &mut f64,
        var_guard406_rv_slot: &mut f64,
        var_guard407_slot: &mut f64,
        var_guard407_rv_slot: &mut f64,
        var_guard408_slot: &mut f64,
        var_guard408_rv_slot: &mut f64,
        var_guard409_slot: &mut f64,
        var_guard409_rv_slot: &mut f64,
        var_guard410_slot: &mut f64,
        var_guard410_rv_slot: &mut f64,
        var_guard411_slot: &mut f64,
        var_guard411_rv_slot: &mut f64,
        var_guard412_slot: &mut f64,
        var_guard412_rv_slot: &mut f64,
        var_rdraingeo_slot: &mut f64,
        var_rdraingeo_rv_slot: &mut f64,
        var_rend_slot: &mut f64,
        var_rend_rv_slot: &mut f64,
        var_rint_slot: &mut f64,
        var_rint_rv_slot: &mut f64,
    ) {
        let mut var_guard388: f64 = *var_guard388_slot;
        let mut var_guard388_rv: f64 = *var_guard388_rv_slot;
        let mut var_guard389: f64 = *var_guard389_slot;
        let mut var_guard389_rv: f64 = *var_guard389_rv_slot;
        let mut var_guard390: f64 = *var_guard390_slot;
        let mut var_guard390_rv: f64 = *var_guard390_rv_slot;
        let mut var_guard391: f64 = *var_guard391_slot;
        let mut var_guard391_rv: f64 = *var_guard391_rv_slot;
        let mut var_guard393: f64 = *var_guard393_slot;
        let mut var_guard393_rv: f64 = *var_guard393_rv_slot;
        let mut var_guard394: f64 = *var_guard394_slot;
        let mut var_guard394_rv: f64 = *var_guard394_rv_slot;
        let mut var_guard395: f64 = *var_guard395_slot;
        let mut var_guard395_rv: f64 = *var_guard395_rv_slot;
        let mut var_guard396: f64 = *var_guard396_slot;
        let mut var_guard396_rv: f64 = *var_guard396_rv_slot;
        let mut var_guard397: f64 = *var_guard397_slot;
        let mut var_guard397_rv: f64 = *var_guard397_rv_slot;
        let mut var_guard398: f64 = *var_guard398_slot;
        let mut var_guard398_rv: f64 = *var_guard398_rv_slot;
        let mut var_guard399: f64 = *var_guard399_slot;
        let mut var_guard399_rv: f64 = *var_guard399_rv_slot;
        let mut var_guard401: f64 = *var_guard401_slot;
        let mut var_guard401_rv: f64 = *var_guard401_rv_slot;
        let mut var_guard402: f64 = *var_guard402_slot;
        let mut var_guard402_rv: f64 = *var_guard402_rv_slot;
        let mut var_guard403: f64 = *var_guard403_slot;
        let mut var_guard403_rv: f64 = *var_guard403_rv_slot;
        let mut var_guard404: f64 = *var_guard404_slot;
        let mut var_guard404_rv: f64 = *var_guard404_rv_slot;
        let mut var_guard406: f64 = *var_guard406_slot;
        let mut var_guard406_rv: f64 = *var_guard406_rv_slot;
        let mut var_guard407: f64 = *var_guard407_slot;
        let mut var_guard407_rv: f64 = *var_guard407_rv_slot;
        let mut var_guard408: f64 = *var_guard408_slot;
        let mut var_guard408_rv: f64 = *var_guard408_rv_slot;
        let mut var_guard409: f64 = *var_guard409_slot;
        let mut var_guard409_rv: f64 = *var_guard409_rv_slot;
        let mut var_guard410: f64 = *var_guard410_slot;
        let mut var_guard410_rv: f64 = *var_guard410_rv_slot;
        let mut var_guard411: f64 = *var_guard411_slot;
        let mut var_guard411_rv: f64 = *var_guard411_rv_slot;
        let mut var_guard412: f64 = *var_guard412_slot;
        let mut var_guard412_rv: f64 = *var_guard412_rv_slot;
        let mut var_rdraingeo: f64 = *var_rdraingeo_slot;
        let mut var_rdraingeo_rv: f64 = *var_rdraingeo_rv_slot;
        let mut var_rend: f64 = *var_rend_slot;
        let mut var_rend_rv: f64 = *var_rend_rv_slot;
        let mut var_rint: f64 = *var_rint_slot;
        let mut var_rint_rv: f64 = *var_rint_rv_slot;

        let (assign12070_e16805,) = {
    if (((((((var_guard246 == 0.0) && (var_guard247 != 0.0)) && ((var_guard260 != 0.0) && (!((((((var_guard254 != 0.0) || (var_guard255 != 0.0)) || (var_guard256 != 0.0)) || (var_guard257 != 0.0)) || (var_guard258 != 0.0)) || (var_guard259 != 0.0))))) && (var_guard382 == 0.0)) && (var_guard383 != 0.0)) && (var_guard384 != 0.0)) && (var_guard386 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign12070_e16805;
        var_rend_rv = 0.0;

        let (assign12080_e16843,) = {
    if (((((((var_guard246 == 0.0) && (var_guard247 != 0.0)) && ((var_guard260 != 0.0) && (!((((((var_guard254 != 0.0) || (var_guard255 != 0.0)) || (var_guard256 != 0.0)) || (var_guard257 != 0.0)) || (var_guard258 != 0.0)) || (var_guard259 != 0.0))))) && (var_guard382 == 0.0)) && (var_guard383 != 0.0)) && (var_guard384 != 0.0)) && (var_guard386 == 0.0)) {
        let assign12080_e16837: f64 = (p.p438 * var_dmcgeff);
        let assign12080_e16840: f64 = (var_weff * var_nuendd);
        let assign12080_e16841: f64 = (assign12080_e16837 / assign12080_e16840);
        (assign12080_e16841,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign12080_e16843;
        var_rend_rv = 0.0;

        let assign12100_e16854: f64 = (var_dmcgeff + var_dmcieff);
        let assign12100_e16857: f64 = if ((var_nuendd == 0.0) || (assign12100_e16854 == 0.0)) { 1.0 } else { 0.0 };
        var_guard388 = assign12100_e16857;
        var_guard388_rv = 0.0;

        let (assign12110_e16891,) = {
    if (((((((var_guard246 == 0.0) && (var_guard247 != 0.0)) && ((var_guard260 != 0.0) && (!((((((var_guard254 != 0.0) || (var_guard255 != 0.0)) || (var_guard256 != 0.0)) || (var_guard257 != 0.0)) || (var_guard258 != 0.0)) || (var_guard259 != 0.0))))) && (var_guard382 == 0.0)) && (var_guard383 != 0.0)) && ((var_guard385 != 0.0) && (var_guard384 == 0.0))) && (var_guard388 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign12110_e16891;
        var_rend_rv = 0.0;

        let (assign12120_e16936,) = {
    if (((((((var_guard246 == 0.0) && (var_guard247 != 0.0)) && ((var_guard260 != 0.0) && (!((((((var_guard254 != 0.0) || (var_guard255 != 0.0)) || (var_guard256 != 0.0)) || (var_guard257 != 0.0)) || (var_guard258 != 0.0)) || (var_guard259 != 0.0))))) && (var_guard382 == 0.0)) && (var_guard383 != 0.0)) && ((var_guard385 != 0.0) && (var_guard384 == 0.0))) && (var_guard388 == 0.0)) {
        let assign12120_e16926: f64 = (p.p438 * var_weff);
        let assign12120_e16929: f64 = (3.0 * var_nuendd);
        let assign12120_e16932: f64 = (var_dmcgeff + var_dmcieff);
        let assign12120_e16933: f64 = (assign12120_e16929 * assign12120_e16932);
        let assign12120_e16934: f64 = (assign12120_e16926 / assign12120_e16933);
        (assign12120_e16934,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign12120_e16936;
        var_rend_rv = 0.0;

        let (assign12130_e16968,) = {
    if ((((((var_guard246 == 0.0) && (var_guard247 != 0.0)) && ((var_guard260 != 0.0) && (!((((((var_guard254 != 0.0) || (var_guard255 != 0.0)) || (var_guard256 != 0.0)) || (var_guard257 != 0.0)) || (var_guard258 != 0.0)) || (var_guard259 != 0.0))))) && (var_guard382 == 0.0)) && (var_guard383 != 0.0)) && (!((var_guard384 != 0.0) || (var_guard385 != 0.0)))) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign12130_e16968;
        var_rend_rv = 0.0;

        let assign12140_e16979: f64 = if (((p.p9 == 1.0) || (p.p9 == 3.0)) || (p.p9 == 7.0)) { 1.0 } else { 0.0 };
        var_guard389 = assign12140_e16979;
        var_guard389_rv = 0.0;

        let assign12150_e16990: f64 = if (((p.p9 == 2.0) || (p.p9 == 4.0)) || (p.p9 == 8.0)) { 1.0 } else { 0.0 };
        var_guard390 = assign12150_e16990;
        var_guard390_rv = 0.0;

        let assign12160_e16993: f64 = if var_nuendd == 0.0 { 1.0 } else { 0.0 };
        var_guard391 = assign12160_e16993;
        var_guard391_rv = 0.0;

        let (assign12170_e17025,) = {
    if (((((((var_guard246 == 0.0) && (var_guard247 != 0.0)) && ((var_guard260 != 0.0) && (!((((((var_guard254 != 0.0) || (var_guard255 != 0.0)) || (var_guard256 != 0.0)) || (var_guard257 != 0.0)) || (var_guard258 != 0.0)) || (var_guard259 != 0.0))))) && (var_guard382 == 0.0)) && (var_guard383 == 0.0)) && (var_guard389 != 0.0)) && (var_guard391 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign12170_e17025;
        var_rend_rv = 0.0;

        let (assign12180_e17064,) = {
    if (((((((var_guard246 == 0.0) && (var_guard247 != 0.0)) && ((var_guard260 != 0.0) && (!((((((var_guard254 != 0.0) || (var_guard255 != 0.0)) || (var_guard256 != 0.0)) || (var_guard257 != 0.0)) || (var_guard258 != 0.0)) || (var_guard259 != 0.0))))) && (var_guard382 == 0.0)) && (var_guard383 == 0.0)) && (var_guard389 != 0.0)) && (var_guard391 == 0.0)) {
        let assign12180_e17058: f64 = (p.p438 * var_dmcgeff);
        let assign12180_e17061: f64 = (var_weff * var_nuendd);
        let assign12180_e17062: f64 = (assign12180_e17058 / assign12180_e17061);
        (assign12180_e17062,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign12180_e17064;
        var_rend_rv = 0.0;

        let assign12200_e17075: f64 = (var_dmcgeff + var_dmcieff);
        let assign12200_e17078: f64 = if ((var_nuendd == 0.0) || (assign12200_e17075 == 0.0)) { 1.0 } else { 0.0 };
        var_guard393 = assign12200_e17078;
        var_guard393_rv = 0.0;

        let (assign12210_e17113,) = {
    if (((((((var_guard246 == 0.0) && (var_guard247 != 0.0)) && ((var_guard260 != 0.0) && (!((((((var_guard254 != 0.0) || (var_guard255 != 0.0)) || (var_guard256 != 0.0)) || (var_guard257 != 0.0)) || (var_guard258 != 0.0)) || (var_guard259 != 0.0))))) && (var_guard382 == 0.0)) && (var_guard383 == 0.0)) && ((var_guard390 != 0.0) && (var_guard389 == 0.0))) && (var_guard393 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign12210_e17113;
        var_rend_rv = 0.0;

        let (assign12220_e17159,) = {
    if (((((((var_guard246 == 0.0) && (var_guard247 != 0.0)) && ((var_guard260 != 0.0) && (!((((((var_guard254 != 0.0) || (var_guard255 != 0.0)) || (var_guard256 != 0.0)) || (var_guard257 != 0.0)) || (var_guard258 != 0.0)) || (var_guard259 != 0.0))))) && (var_guard382 == 0.0)) && (var_guard383 == 0.0)) && ((var_guard390 != 0.0) && (var_guard389 == 0.0))) && (var_guard393 == 0.0)) {
        let assign12220_e17149: f64 = (p.p438 * var_weff);
        let assign12220_e17152: f64 = (3.0 * var_nuendd);
        let assign12220_e17155: f64 = (var_dmcgeff + var_dmcieff);
        let assign12220_e17156: f64 = (assign12220_e17152 * assign12220_e17155);
        let assign12220_e17157: f64 = (assign12220_e17149 / assign12220_e17156);
        (assign12220_e17157,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign12220_e17159;
        var_rend_rv = 0.0;

        let (assign12230_e17192,) = {
    if ((((((var_guard246 == 0.0) && (var_guard247 != 0.0)) && ((var_guard260 != 0.0) && (!((((((var_guard254 != 0.0) || (var_guard255 != 0.0)) || (var_guard256 != 0.0)) || (var_guard257 != 0.0)) || (var_guard258 != 0.0)) || (var_guard259 != 0.0))))) && (var_guard382 == 0.0)) && (var_guard383 == 0.0)) && (!((var_guard389 != 0.0) || (var_guard390 != 0.0)))) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign12230_e17192;
        var_rend_rv = 0.0;

        let assign12240_e17195: f64 = if 0.0 == 1.0 { 1.0 } else { 0.0 };
        var_guard394 = assign12240_e17195;
        var_guard394_rv = 0.0;

        let assign12250_e17198: f64 = if var_nuends == 0.0 { 1.0 } else { 0.0 };
        var_guard395 = assign12250_e17198;
        var_guard395_rv = 0.0;

        let (assign12260_e17226,) = {
    if (((((var_guard246 == 0.0) && (var_guard247 != 0.0)) && ((var_guard261 != 0.0) && (!(((((((var_guard254 != 0.0) || (var_guard255 != 0.0)) || (var_guard256 != 0.0)) || (var_guard257 != 0.0)) || (var_guard258 != 0.0)) || (var_guard259 != 0.0)) || (var_guard260 != 0.0))))) && (var_guard394 != 0.0)) && (var_guard395 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign12260_e17226;
        var_rend_rv = 0.0;

        let (assign12270_e17261,) = {
    if (((((var_guard246 == 0.0) && (var_guard247 != 0.0)) && ((var_guard261 != 0.0) && (!(((((((var_guard254 != 0.0) || (var_guard255 != 0.0)) || (var_guard256 != 0.0)) || (var_guard257 != 0.0)) || (var_guard258 != 0.0)) || (var_guard259 != 0.0)) || (var_guard260 != 0.0))))) && (var_guard394 != 0.0)) && (var_guard395 == 0.0)) {
        let assign12270_e17255: f64 = (p.p438 * var_dmdgeff);
        let assign12270_e17258: f64 = (var_weff * var_nuends);
        let assign12270_e17259: f64 = (assign12270_e17255 / assign12270_e17258);
        (assign12270_e17259,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign12270_e17261;
        var_rend_rv = 0.0;

        let assign12280_e17264: f64 = if 0.0 == 1.0 { 1.0 } else { 0.0 };
        var_guard396 = assign12280_e17264;
        var_guard396_rv = 0.0;

        let assign12290_e17275: f64 = if (((p.p9 == 1.0) || (p.p9 == 2.0)) || (p.p9 == 5.0)) { 1.0 } else { 0.0 };
        var_guard397 = assign12290_e17275;
        var_guard397_rv = 0.0;

        let assign12300_e17286: f64 = if (((p.p9 == 3.0) || (p.p9 == 4.0)) || (p.p9 == 6.0)) { 1.0 } else { 0.0 };
        var_guard398 = assign12300_e17286;
        var_guard398_rv = 0.0;

        let assign12310_e17289: f64 = if var_nuendd == 0.0 { 1.0 } else { 0.0 };
        var_guard399 = assign12310_e17289;
        var_guard399_rv = 0.0;

        let (assign12320_e17322,) = {
    if (((((((var_guard246 == 0.0) && (var_guard247 != 0.0)) && ((var_guard261 != 0.0) && (!(((((((var_guard254 != 0.0) || (var_guard255 != 0.0)) || (var_guard256 != 0.0)) || (var_guard257 != 0.0)) || (var_guard258 != 0.0)) || (var_guard259 != 0.0)) || (var_guard260 != 0.0))))) && (var_guard394 == 0.0)) && (var_guard396 != 0.0)) && (var_guard397 != 0.0)) && (var_guard399 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign12320_e17322;
        var_rend_rv = 0.0;

        let (assign12330_e17362,) = {
    if (((((((var_guard246 == 0.0) && (var_guard247 != 0.0)) && ((var_guard261 != 0.0) && (!(((((((var_guard254 != 0.0) || (var_guard255 != 0.0)) || (var_guard256 != 0.0)) || (var_guard257 != 0.0)) || (var_guard258 != 0.0)) || (var_guard259 != 0.0)) || (var_guard260 != 0.0))))) && (var_guard394 == 0.0)) && (var_guard396 != 0.0)) && (var_guard397 != 0.0)) && (var_guard399 == 0.0)) {
        let assign12330_e17356: f64 = (p.p438 * var_dmcgeff);
        let assign12330_e17359: f64 = (var_weff * var_nuendd);
        let assign12330_e17360: f64 = (assign12330_e17356 / assign12330_e17359);
        (assign12330_e17360,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign12330_e17362;
        var_rend_rv = 0.0;

        let assign12350_e17372: f64 = if ((var_nuendd == 0.0) || (var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        var_guard401 = assign12350_e17372;
        var_guard401_rv = 0.0;

        let (assign12360_e17408,) = {
    if (((((((var_guard246 == 0.0) && (var_guard247 != 0.0)) && ((var_guard261 != 0.0) && (!(((((((var_guard254 != 0.0) || (var_guard255 != 0.0)) || (var_guard256 != 0.0)) || (var_guard257 != 0.0)) || (var_guard258 != 0.0)) || (var_guard259 != 0.0)) || (var_guard260 != 0.0))))) && (var_guard394 == 0.0)) && (var_guard396 != 0.0)) && ((var_guard398 != 0.0) && (var_guard397 == 0.0))) && (var_guard401 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign12360_e17408;
        var_rend_rv = 0.0;

        let (assign12370_e17453,) = {
    if (((((((var_guard246 == 0.0) && (var_guard247 != 0.0)) && ((var_guard261 != 0.0) && (!(((((((var_guard254 != 0.0) || (var_guard255 != 0.0)) || (var_guard256 != 0.0)) || (var_guard257 != 0.0)) || (var_guard258 != 0.0)) || (var_guard259 != 0.0)) || (var_guard260 != 0.0))))) && (var_guard394 == 0.0)) && (var_guard396 != 0.0)) && ((var_guard398 != 0.0) && (var_guard397 == 0.0))) && (var_guard401 == 0.0)) {
        let assign12370_e17445: f64 = (p.p438 * var_weff);
        let assign12370_e17448: f64 = (6.0 * var_nuendd);
        let assign12370_e17450: f64 = (assign12370_e17448 * var_dmcgeff);
        let assign12370_e17451: f64 = (assign12370_e17445 / assign12370_e17450);
        (assign12370_e17451,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign12370_e17453;
        var_rend_rv = 0.0;

        let (assign12380_e17487,) = {
    if ((((((var_guard246 == 0.0) && (var_guard247 != 0.0)) && ((var_guard261 != 0.0) && (!(((((((var_guard254 != 0.0) || (var_guard255 != 0.0)) || (var_guard256 != 0.0)) || (var_guard257 != 0.0)) || (var_guard258 != 0.0)) || (var_guard259 != 0.0)) || (var_guard260 != 0.0))))) && (var_guard394 == 0.0)) && (var_guard396 != 0.0)) && (!((var_guard397 != 0.0) || (var_guard398 != 0.0)))) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign12380_e17487;
        var_rend_rv = 0.0;

        let assign12390_e17498: f64 = if (((p.p9 == 1.0) || (p.p9 == 3.0)) || (p.p9 == 7.0)) { 1.0 } else { 0.0 };
        var_guard402 = assign12390_e17498;
        var_guard402_rv = 0.0;

        let assign12400_e17509: f64 = if (((p.p9 == 2.0) || (p.p9 == 4.0)) || (p.p9 == 8.0)) { 1.0 } else { 0.0 };
        var_guard403 = assign12400_e17509;
        var_guard403_rv = 0.0;

        let assign12410_e17512: f64 = if var_nuendd == 0.0 { 1.0 } else { 0.0 };
        var_guard404 = assign12410_e17512;
        var_guard404_rv = 0.0;

        let (assign12420_e17546,) = {
    if (((((((var_guard246 == 0.0) && (var_guard247 != 0.0)) && ((var_guard261 != 0.0) && (!(((((((var_guard254 != 0.0) || (var_guard255 != 0.0)) || (var_guard256 != 0.0)) || (var_guard257 != 0.0)) || (var_guard258 != 0.0)) || (var_guard259 != 0.0)) || (var_guard260 != 0.0))))) && (var_guard394 == 0.0)) && (var_guard396 == 0.0)) && (var_guard402 != 0.0)) && (var_guard404 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign12420_e17546;
        var_rend_rv = 0.0;

        let (assign12430_e17587,) = {
    if (((((((var_guard246 == 0.0) && (var_guard247 != 0.0)) && ((var_guard261 != 0.0) && (!(((((((var_guard254 != 0.0) || (var_guard255 != 0.0)) || (var_guard256 != 0.0)) || (var_guard257 != 0.0)) || (var_guard258 != 0.0)) || (var_guard259 != 0.0)) || (var_guard260 != 0.0))))) && (var_guard394 == 0.0)) && (var_guard396 == 0.0)) && (var_guard402 != 0.0)) && (var_guard404 == 0.0)) {
        let assign12430_e17581: f64 = (p.p438 * var_dmcgeff);
        let assign12430_e17584: f64 = (var_weff * var_nuendd);
        let assign12430_e17585: f64 = (assign12430_e17581 / assign12430_e17584);
        (assign12430_e17585,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign12430_e17587;
        var_rend_rv = 0.0;

        let assign12450_e17597: f64 = if ((var_nuendd == 0.0) || (var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        var_guard406 = assign12450_e17597;
        var_guard406_rv = 0.0;

        let (assign12460_e17634,) = {
    if (((((((var_guard246 == 0.0) && (var_guard247 != 0.0)) && ((var_guard261 != 0.0) && (!(((((((var_guard254 != 0.0) || (var_guard255 != 0.0)) || (var_guard256 != 0.0)) || (var_guard257 != 0.0)) || (var_guard258 != 0.0)) || (var_guard259 != 0.0)) || (var_guard260 != 0.0))))) && (var_guard394 == 0.0)) && (var_guard396 == 0.0)) && ((var_guard403 != 0.0) && (var_guard402 == 0.0))) && (var_guard406 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign12460_e17634;
        var_rend_rv = 0.0;

        let (assign12470_e17680,) = {
    if (((((((var_guard246 == 0.0) && (var_guard247 != 0.0)) && ((var_guard261 != 0.0) && (!(((((((var_guard254 != 0.0) || (var_guard255 != 0.0)) || (var_guard256 != 0.0)) || (var_guard257 != 0.0)) || (var_guard258 != 0.0)) || (var_guard259 != 0.0)) || (var_guard260 != 0.0))))) && (var_guard394 == 0.0)) && (var_guard396 == 0.0)) && ((var_guard403 != 0.0) && (var_guard402 == 0.0))) && (var_guard406 == 0.0)) {
        let assign12470_e17672: f64 = (p.p438 * var_weff);
        let assign12470_e17675: f64 = (6.0 * var_nuendd);
        let assign12470_e17677: f64 = (assign12470_e17675 * var_dmcgeff);
        let assign12470_e17678: f64 = (assign12470_e17672 / assign12470_e17677);
        (assign12470_e17678,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign12470_e17680;
        var_rend_rv = 0.0;

        let (assign12480_e17715,) = {
    if ((((((var_guard246 == 0.0) && (var_guard247 != 0.0)) && ((var_guard261 != 0.0) && (!(((((((var_guard254 != 0.0) || (var_guard255 != 0.0)) || (var_guard256 != 0.0)) || (var_guard257 != 0.0)) || (var_guard258 != 0.0)) || (var_guard259 != 0.0)) || (var_guard260 != 0.0))))) && (var_guard394 == 0.0)) && (var_guard396 == 0.0)) && (!((var_guard402 != 0.0) || (var_guard403 != 0.0)))) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign12480_e17715;
        var_rend_rv = 0.0;

        let (assign12490_e17745,) = {
    if (((var_guard246 == 0.0) && (var_guard247 != 0.0)) && ((var_guard262 != 0.0) && (!((((((((var_guard254 != 0.0) || (var_guard255 != 0.0)) || (var_guard256 != 0.0)) || (var_guard257 != 0.0)) || (var_guard258 != 0.0)) || (var_guard259 != 0.0)) || (var_guard260 != 0.0)) || (var_guard261 != 0.0))))) {
        let assign12490_e17741: f64 = (p.p438 * var_dmdgeff);
        let assign12490_e17743: f64 = (assign12490_e17741 / var_weff);
        (assign12490_e17743,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign12490_e17745;
        var_rend_rv = 0.0;

        let assign12500_e17748: f64 = if 0.0 == 1.0 { 1.0 } else { 0.0 };
        var_guard407 = assign12500_e17748;
        var_guard407_rv = 0.0;

        let (assign12510_e17784,) = {
    if ((((var_guard246 == 0.0) && (var_guard247 != 0.0)) && ((var_guard263 != 0.0) && (!(((((((((var_guard254 != 0.0) || (var_guard255 != 0.0)) || (var_guard256 != 0.0)) || (var_guard257 != 0.0)) || (var_guard258 != 0.0)) || (var_guard259 != 0.0)) || (var_guard260 != 0.0)) || (var_guard261 != 0.0)) || (var_guard262 != 0.0))))) && (var_guard407 != 0.0)) {
        let assign12510_e17778: f64 = (0.5 * p.p438);
        let assign12510_e17780: f64 = (assign12510_e17778 * var_dmcgeff);
        let assign12510_e17782: f64 = (assign12510_e17780 / var_weff);
        (assign12510_e17782,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign12510_e17784;
        var_rend_rv = 0.0;

        let assign12520_e17787: f64 = if p.p2 == 2.0 { 1.0 } else { 0.0 };
        var_guard408 = assign12520_e17787;
        var_guard408_rv = 0.0;

        let (assign12530_e17819,) = {
    if (((((var_guard246 == 0.0) && (var_guard247 != 0.0)) && ((var_guard263 != 0.0) && (!(((((((((var_guard254 != 0.0) || (var_guard255 != 0.0)) || (var_guard256 != 0.0)) || (var_guard257 != 0.0)) || (var_guard258 != 0.0)) || (var_guard259 != 0.0)) || (var_guard260 != 0.0)) || (var_guard261 != 0.0)) || (var_guard262 != 0.0))))) && (var_guard407 != 0.0)) && (var_guard408 != 0.0)) {
        (0.0,)
    } else {
        (var_rint,)
    }
};
        var_rint = assign12530_e17819;
        var_rint_rv = 0.0;

        let (assign12540_e17860,) = {
    if (((((var_guard246 == 0.0) && (var_guard247 != 0.0)) && ((var_guard263 != 0.0) && (!(((((((((var_guard254 != 0.0) || (var_guard255 != 0.0)) || (var_guard256 != 0.0)) || (var_guard257 != 0.0)) || (var_guard258 != 0.0)) || (var_guard259 != 0.0)) || (var_guard260 != 0.0)) || (var_guard261 != 0.0)) || (var_guard262 != 0.0))))) && (var_guard407 != 0.0)) && (var_guard408 == 0.0)) {
        let assign12540_e17852: f64 = (p.p438 * var_dmcgeff);
        let assign12540_e17856: f64 = (p.p2 - 2.0);
        let assign12540_e17857: f64 = (var_weff * assign12540_e17856);
        let assign12540_e17858: f64 = (assign12540_e17852 / assign12540_e17857);
        (assign12540_e17858,)
    } else {
        (var_rint,)
    }
};
        var_rint = assign12540_e17860;
        var_rint_rv = 0.0;

        let (assign12550_e17891,) = {
    if ((((var_guard246 == 0.0) && (var_guard247 != 0.0)) && ((var_guard263 != 0.0) && (!(((((((((var_guard254 != 0.0) || (var_guard255 != 0.0)) || (var_guard256 != 0.0)) || (var_guard257 != 0.0)) || (var_guard258 != 0.0)) || (var_guard259 != 0.0)) || (var_guard260 != 0.0)) || (var_guard261 != 0.0)) || (var_guard262 != 0.0))))) && (var_guard407 == 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign12550_e17891;
        var_rend_rv = 0.0;

        let (assign12560_e17928,) = {
    if ((((var_guard246 == 0.0) && (var_guard247 != 0.0)) && ((var_guard263 != 0.0) && (!(((((((((var_guard254 != 0.0) || (var_guard255 != 0.0)) || (var_guard256 != 0.0)) || (var_guard257 != 0.0)) || (var_guard258 != 0.0)) || (var_guard259 != 0.0)) || (var_guard260 != 0.0)) || (var_guard261 != 0.0)) || (var_guard262 != 0.0))))) && (var_guard407 == 0.0)) {
        let assign12560_e17922: f64 = (p.p438 * var_dmcgeff);
        let assign12560_e17925: f64 = (var_weff * p.p2);
        let assign12560_e17926: f64 = (assign12560_e17922 / assign12560_e17925);
        (assign12560_e17926,)
    } else {
        (var_rint,)
    }
};
        var_rint = assign12560_e17928;
        var_rint_rv = 0.0;

        let assign12570_e17931: f64 = if 0.0 == 1.0 { 1.0 } else { 0.0 };
        var_guard409 = assign12570_e17931;
        var_guard409_rv = 0.0;

        let (assign12580_e17963,) = {
    if ((((var_guard246 == 0.0) && (var_guard247 != 0.0)) && ((var_guard264 != 0.0) && (!((((((((((var_guard254 != 0.0) || (var_guard255 != 0.0)) || (var_guard256 != 0.0)) || (var_guard257 != 0.0)) || (var_guard258 != 0.0)) || (var_guard259 != 0.0)) || (var_guard260 != 0.0)) || (var_guard261 != 0.0)) || (var_guard262 != 0.0)) || (var_guard263 != 0.0))))) && (var_guard409 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign12580_e17963;
        var_rend_rv = 0.0;

        let (assign12590_e18001,) = {
    if ((((var_guard246 == 0.0) && (var_guard247 != 0.0)) && ((var_guard264 != 0.0) && (!((((((((((var_guard254 != 0.0) || (var_guard255 != 0.0)) || (var_guard256 != 0.0)) || (var_guard257 != 0.0)) || (var_guard258 != 0.0)) || (var_guard259 != 0.0)) || (var_guard260 != 0.0)) || (var_guard261 != 0.0)) || (var_guard262 != 0.0)) || (var_guard263 != 0.0))))) && (var_guard409 != 0.0)) {
        let assign12590_e17995: f64 = (p.p438 * var_dmcgeff);
        let assign12590_e17998: f64 = (var_weff * p.p2);
        let assign12590_e17999: f64 = (assign12590_e17995 / assign12590_e17998);
        (assign12590_e17999,)
    } else {
        (var_rint,)
    }
};
        var_rint = assign12590_e18001;
        var_rint_rv = 0.0;

        let (assign12600_e18040,) = {
    if ((((var_guard246 == 0.0) && (var_guard247 != 0.0)) && ((var_guard264 != 0.0) && (!((((((((((var_guard254 != 0.0) || (var_guard255 != 0.0)) || (var_guard256 != 0.0)) || (var_guard257 != 0.0)) || (var_guard258 != 0.0)) || (var_guard259 != 0.0)) || (var_guard260 != 0.0)) || (var_guard261 != 0.0)) || (var_guard262 != 0.0)) || (var_guard263 != 0.0))))) && (var_guard409 == 0.0)) {
        let assign12600_e18034: f64 = (0.5 * p.p438);
        let assign12600_e18036: f64 = (assign12600_e18034 * var_dmcgeff);
        let assign12600_e18038: f64 = (assign12600_e18036 / var_weff);
        (assign12600_e18038,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign12600_e18040;
        var_rend_rv = 0.0;

        let assign12610_e18043: f64 = if p.p2 == 2.0 { 1.0 } else { 0.0 };
        var_guard410 = assign12610_e18043;
        var_guard410_rv = 0.0;

        let (assign12620_e18078,) = {
    if (((((var_guard246 == 0.0) && (var_guard247 != 0.0)) && ((var_guard264 != 0.0) && (!((((((((((var_guard254 != 0.0) || (var_guard255 != 0.0)) || (var_guard256 != 0.0)) || (var_guard257 != 0.0)) || (var_guard258 != 0.0)) || (var_guard259 != 0.0)) || (var_guard260 != 0.0)) || (var_guard261 != 0.0)) || (var_guard262 != 0.0)) || (var_guard263 != 0.0))))) && (var_guard409 == 0.0)) && (var_guard410 != 0.0)) {
        (0.0,)
    } else {
        (var_rint,)
    }
};
        var_rint = assign12620_e18078;
        var_rint_rv = 0.0;

        let (assign12630_e18122,) = {
    if (((((var_guard246 == 0.0) && (var_guard247 != 0.0)) && ((var_guard264 != 0.0) && (!((((((((((var_guard254 != 0.0) || (var_guard255 != 0.0)) || (var_guard256 != 0.0)) || (var_guard257 != 0.0)) || (var_guard258 != 0.0)) || (var_guard259 != 0.0)) || (var_guard260 != 0.0)) || (var_guard261 != 0.0)) || (var_guard262 != 0.0)) || (var_guard263 != 0.0))))) && (var_guard409 == 0.0)) && (var_guard410 == 0.0)) {
        let assign12630_e18114: f64 = (p.p438 * var_dmcgeff);
        let assign12630_e18118: f64 = (p.p2 - 2.0);
        let assign12630_e18119: f64 = (var_weff * assign12630_e18118);
        let assign12630_e18120: f64 = (assign12630_e18114 / assign12630_e18119);
        (assign12630_e18120,)
    } else {
        (var_rint,)
    }
};
        var_rint = assign12630_e18122;
        var_rint_rv = 0.0;

        let (assign12640_e18152,) = {
    if (((var_guard246 == 0.0) && (var_guard247 != 0.0)) && (!(((((((((((var_guard254 != 0.0) || (var_guard255 != 0.0)) || (var_guard256 != 0.0)) || (var_guard257 != 0.0)) || (var_guard258 != 0.0)) || (var_guard259 != 0.0)) || (var_guard260 != 0.0)) || (var_guard261 != 0.0)) || (var_guard262 != 0.0)) || (var_guard263 != 0.0)) || (var_guard264 != 0.0)))) {
        (0.0,)
    } else {
        (var_rint,)
    }
};
        var_rint = assign12640_e18152;
        var_rint_rv = 0.0;

        let assign12650_e18155: f64 = if var_rint <= 0.0 { 1.0 } else { 0.0 };
        var_guard411 = assign12650_e18155;
        var_guard411_rv = 0.0;

        let (assign12660_e18164,) = {
    if (((var_guard246 == 0.0) && (var_guard247 != 0.0)) && (var_guard411 != 0.0)) {
        (var_rend,)
    } else {
        (var_rdraingeo,)
    }
};
        var_rdraingeo = assign12660_e18164;
        var_rdraingeo_rv = 0.0;

        let assign12670_e18167: f64 = if var_rend <= 0.0 { 1.0 } else { 0.0 };
        var_guard412 = assign12670_e18167;
        var_guard412_rv = 0.0;

        let (assign12680_e18179,) = {
    if ((((var_guard246 == 0.0) && (var_guard247 != 0.0)) && (var_guard411 == 0.0)) && (var_guard412 != 0.0)) {
        (var_rint,)
    } else {
        (var_rdraingeo,)
    }
};
        var_rdraingeo = assign12680_e18179;
        var_rdraingeo_rv = 0.0;

        *var_guard388_slot = var_guard388;
        *var_guard388_rv_slot = var_guard388_rv;
        *var_guard389_slot = var_guard389;
        *var_guard389_rv_slot = var_guard389_rv;
        *var_guard390_slot = var_guard390;
        *var_guard390_rv_slot = var_guard390_rv;
        *var_guard391_slot = var_guard391;
        *var_guard391_rv_slot = var_guard391_rv;
        *var_guard393_slot = var_guard393;
        *var_guard393_rv_slot = var_guard393_rv;
        *var_guard394_slot = var_guard394;
        *var_guard394_rv_slot = var_guard394_rv;
        *var_guard395_slot = var_guard395;
        *var_guard395_rv_slot = var_guard395_rv;
        *var_guard396_slot = var_guard396;
        *var_guard396_rv_slot = var_guard396_rv;
        *var_guard397_slot = var_guard397;
        *var_guard397_rv_slot = var_guard397_rv;
        *var_guard398_slot = var_guard398;
        *var_guard398_rv_slot = var_guard398_rv;
        *var_guard399_slot = var_guard399;
        *var_guard399_rv_slot = var_guard399_rv;
        *var_guard401_slot = var_guard401;
        *var_guard401_rv_slot = var_guard401_rv;
        *var_guard402_slot = var_guard402;
        *var_guard402_rv_slot = var_guard402_rv;
        *var_guard403_slot = var_guard403;
        *var_guard403_rv_slot = var_guard403_rv;
        *var_guard404_slot = var_guard404;
        *var_guard404_rv_slot = var_guard404_rv;
        *var_guard406_slot = var_guard406;
        *var_guard406_rv_slot = var_guard406_rv;
        *var_guard407_slot = var_guard407;
        *var_guard407_rv_slot = var_guard407_rv;
        *var_guard408_slot = var_guard408;
        *var_guard408_rv_slot = var_guard408_rv;
        *var_guard409_slot = var_guard409;
        *var_guard409_rv_slot = var_guard409_rv;
        *var_guard410_slot = var_guard410;
        *var_guard410_rv_slot = var_guard410_rv;
        *var_guard411_slot = var_guard411;
        *var_guard411_rv_slot = var_guard411_rv;
        *var_guard412_slot = var_guard412;
        *var_guard412_rv_slot = var_guard412_rv;
        *var_rdraingeo_slot = var_rdraingeo;
        *var_rdraingeo_rv_slot = var_rdraingeo_rv;
        *var_rend_slot = var_rend;
        *var_rend_rv_slot = var_rend_rv;
        *var_rint_slot = var_rint;
        *var_rint_rv_slot = var_rint_rv;
    }

    pub(super) fn stamp_reactive_block_21(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        var_guard246: f64,
        var_guard247: f64,
        var_guard411: f64,
        var_guard412: f64,
        var_kboq: f64,
        var_lnew: f64,
        var_ntox_i: f64,
        var_poxedge_i: f64,
        var_rend: f64,
        var_rint: f64,
        var_weff: f64,
        var_weffcj: f64,
        var_bechvb_slot: &mut f64,
        var_bechvb_rv_slot: &mut f64,
        var_bechvbedge_slot: &mut f64,
        var_bechvbedge_rv_slot: &mut f64,
        var_cth_slot: &mut f64,
        var_cth_rv_slot: &mut f64,
        var_deltemp_slot: &mut f64,
        var_deltemp1_slot: &mut f64,
        var_deltemp1_dn4_slot: &mut f64,
        var_deltemp1_dn5_slot: &mut f64,
        var_deltemp1_rv_slot: &mut f64,
        var_deltemp_dn4_slot: &mut f64,
        var_deltemp_dn5_slot: &mut f64,
        var_deltemp_rv_slot: &mut f64,
        var_devtemp_slot: &mut f64,
        var_devtemp_dn4_slot: &mut f64,
        var_devtemp_dn5_slot: &mut f64,
        var_devtemp_rv_slot: &mut f64,
        var_eg_slot: &mut f64,
        var_eg_dn4_slot: &mut f64,
        var_eg_dn5_slot: &mut f64,
        var_eg_rv_slot: &mut f64,
        var_grgeltd_slot: &mut f64,
        var_grgeltd_rv_slot: &mut f64,
        var_gth_slot: &mut f64,
        var_gth_rv_slot: &mut f64,
        var_guard414_slot: &mut f64,
        var_guard414_rv_slot: &mut f64,
        var_guard415_slot: &mut f64,
        var_guard415_rv_slot: &mut f64,
        var_guard416_slot: &mut f64,
        var_guard416_rv_slot: &mut f64,
        var_guard417_slot: &mut f64,
        var_guard417_rv_slot: &mut f64,
        var_guard418_slot: &mut f64,
        var_guard418_rv_slot: &mut f64,
        var_guard419_slot: &mut f64,
        var_guard419_rv_slot: &mut f64,
        var_guard420_slot: &mut f64,
        var_guard420_rv_slot: &mut f64,
        var_guard421_slot: &mut f64,
        var_guard421_rv_slot: &mut f64,
        var_guard422_slot: &mut f64,
        var_guard422_rv_slot: &mut f64,
        var_guard423_slot: &mut f64,
        var_guard423_rv_slot: &mut f64,
        var_guard424_slot: &mut f64,
        var_guard424_rv_slot: &mut f64,
        var_guard425_slot: &mut f64,
        var_guard425_rv_slot: &mut f64,
        var_guard426_slot: &mut f64,
        var_guard426_rv_slot: &mut f64,
        var_guard431_slot: &mut f64,
        var_guard431_rv_slot: &mut f64,
        var_guard432_slot: &mut f64,
        var_guard432_rv_slot: &mut f64,
        var_guard433_slot: &mut f64,
        var_guard433_rv_slot: &mut f64,
        var_guard434_slot: &mut f64,
        var_guard434_rv_slot: &mut f64,
        var_guard435_slot: &mut f64,
        var_guard435_rv_slot: &mut f64,
        var_inv_vt_slot: &mut f64,
        var_inv_vt_dn4_slot: &mut f64,
        var_inv_vt_dn5_slot: &mut f64,
        var_inv_vt_rv_slot: &mut f64,
        var_rdraingeo_slot: &mut f64,
        var_rdraingeo_rv_slot: &mut f64,
        var_rdsw_i_slot: &mut f64,
        var_rdsw_i_rv_slot: &mut f64,
        var_rdswmin_i_slot: &mut f64,
        var_rdswmin_i_rv_slot: &mut f64,
        var_rdw_i_slot: &mut f64,
        var_rdw_i_rv_slot: &mut f64,
        var_rdwmin_i_slot: &mut f64,
        var_rdwmin_i_rv_slot: &mut f64,
        var_rsourcegeo_slot: &mut f64,
        var_rsourcegeo_rv_slot: &mut f64,
        var_rsw_i_slot: &mut f64,
        var_rsw_i_rv_slot: &mut f64,
        var_rswmin_i_slot: &mut f64,
        var_rswmin_i_rv_slot: &mut f64,
        var_t0_slot: &mut f64,
        var_t0_dn10_slot: &mut f64,
        var_t0_dn11_slot: &mut f64,
        var_t0_dn3_slot: &mut f64,
        var_t0_dn4_slot: &mut f64,
        var_t0_dn5_slot: &mut f64,
        var_t0_dn6_slot: &mut f64,
        var_t0_dn7_slot: &mut f64,
        var_t0_dn8_slot: &mut f64,
        var_t0_dn9_slot: &mut f64,
        var_t0_rv_slot: &mut f64,
        var_t1_slot: &mut f64,
        var_t1_dn10_slot: &mut f64,
        var_t1_dn11_slot: &mut f64,
        var_t1_dn3_slot: &mut f64,
        var_t1_dn4_slot: &mut f64,
        var_t1_dn5_slot: &mut f64,
        var_t1_dn6_slot: &mut f64,
        var_t1_dn7_slot: &mut f64,
        var_t1_dn8_slot: &mut f64,
        var_t1_dn9_slot: &mut f64,
        var_t1_rv_slot: &mut f64,
        var_t2_slot: &mut f64,
        var_t2_dn10_slot: &mut f64,
        var_t2_dn11_slot: &mut f64,
        var_t2_dn3_slot: &mut f64,
        var_t2_dn4_slot: &mut f64,
        var_t2_dn5_slot: &mut f64,
        var_t2_dn6_slot: &mut f64,
        var_t2_dn7_slot: &mut f64,
        var_t2_dn8_slot: &mut f64,
        var_t2_dn9_slot: &mut f64,
        var_t2_rv_slot: &mut f64,
        var_tnom_slot: &mut f64,
        var_tnom_rv_slot: &mut f64,
        var_toxratio_slot: &mut f64,
        var_toxratio_dn10_slot: &mut f64,
        var_toxratio_dn11_slot: &mut f64,
        var_toxratio_dn3_slot: &mut f64,
        var_toxratio_dn4_slot: &mut f64,
        var_toxratio_dn5_slot: &mut f64,
        var_toxratio_dn6_slot: &mut f64,
        var_toxratio_dn7_slot: &mut f64,
        var_toxratio_dn8_slot: &mut f64,
        var_toxratio_dn9_slot: &mut f64,
        var_toxratio_rv_slot: &mut f64,
        var_tratio_slot: &mut f64,
        var_tratio_dn4_slot: &mut f64,
        var_tratio_dn5_slot: &mut f64,
        var_tratio_rv_slot: &mut f64,
        var_vt_slot: &mut f64,
        var_vt_dn4_slot: &mut f64,
        var_vt_dn5_slot: &mut f64,
        var_vt_rv_slot: &mut f64,
        var_vtm_slot: &mut f64,
        var_vtm0_slot: &mut f64,
        var_vtm0_rv_slot: &mut f64,
        var_vtm_dn4_slot: &mut f64,
        var_vtm_dn5_slot: &mut f64,
        var_vtm_rv_slot: &mut f64,
        var_weff_sh_slot: &mut f64,
        var_weff_sh_rv_slot: &mut f64,
    ) {
        let ctx_temp = ctx.temperature();
        let nv4 = ctx.node_voltage(nodes[4]);
        let nv5 = ctx.node_voltage(nodes[5]);
        let mut var_bechvb: f64 = *var_bechvb_slot;
        let mut var_bechvb_rv: f64 = *var_bechvb_rv_slot;
        let mut var_bechvbedge: f64 = *var_bechvbedge_slot;
        let mut var_bechvbedge_rv: f64 = *var_bechvbedge_rv_slot;
        let mut var_cth: f64 = *var_cth_slot;
        let mut var_cth_rv: f64 = *var_cth_rv_slot;
        let mut var_deltemp: f64 = *var_deltemp_slot;
        let mut var_deltemp1: f64 = *var_deltemp1_slot;
        let mut var_deltemp1_dn4: f64 = *var_deltemp1_dn4_slot;
        let mut var_deltemp1_dn5: f64 = *var_deltemp1_dn5_slot;
        let mut var_deltemp1_rv: f64 = *var_deltemp1_rv_slot;
        let mut var_deltemp_dn4: f64 = *var_deltemp_dn4_slot;
        let mut var_deltemp_dn5: f64 = *var_deltemp_dn5_slot;
        let mut var_deltemp_rv: f64 = *var_deltemp_rv_slot;
        let mut var_devtemp: f64 = *var_devtemp_slot;
        let mut var_devtemp_dn4: f64 = *var_devtemp_dn4_slot;
        let mut var_devtemp_dn5: f64 = *var_devtemp_dn5_slot;
        let mut var_devtemp_rv: f64 = *var_devtemp_rv_slot;
        let mut var_eg: f64 = *var_eg_slot;
        let mut var_eg_dn4: f64 = *var_eg_dn4_slot;
        let mut var_eg_dn5: f64 = *var_eg_dn5_slot;
        let mut var_eg_rv: f64 = *var_eg_rv_slot;
        let mut var_grgeltd: f64 = *var_grgeltd_slot;
        let mut var_grgeltd_rv: f64 = *var_grgeltd_rv_slot;
        let mut var_gth: f64 = *var_gth_slot;
        let mut var_gth_rv: f64 = *var_gth_rv_slot;
        let mut var_guard414: f64 = *var_guard414_slot;
        let mut var_guard414_rv: f64 = *var_guard414_rv_slot;
        let mut var_guard415: f64 = *var_guard415_slot;
        let mut var_guard415_rv: f64 = *var_guard415_rv_slot;
        let mut var_guard416: f64 = *var_guard416_slot;
        let mut var_guard416_rv: f64 = *var_guard416_rv_slot;
        let mut var_guard417: f64 = *var_guard417_slot;
        let mut var_guard417_rv: f64 = *var_guard417_rv_slot;
        let mut var_guard418: f64 = *var_guard418_slot;
        let mut var_guard418_rv: f64 = *var_guard418_rv_slot;
        let mut var_guard419: f64 = *var_guard419_slot;
        let mut var_guard419_rv: f64 = *var_guard419_rv_slot;
        let mut var_guard420: f64 = *var_guard420_slot;
        let mut var_guard420_rv: f64 = *var_guard420_rv_slot;
        let mut var_guard421: f64 = *var_guard421_slot;
        let mut var_guard421_rv: f64 = *var_guard421_rv_slot;
        let mut var_guard422: f64 = *var_guard422_slot;
        let mut var_guard422_rv: f64 = *var_guard422_rv_slot;
        let mut var_guard423: f64 = *var_guard423_slot;
        let mut var_guard423_rv: f64 = *var_guard423_rv_slot;
        let mut var_guard424: f64 = *var_guard424_slot;
        let mut var_guard424_rv: f64 = *var_guard424_rv_slot;
        let mut var_guard425: f64 = *var_guard425_slot;
        let mut var_guard425_rv: f64 = *var_guard425_rv_slot;
        let mut var_guard426: f64 = *var_guard426_slot;
        let mut var_guard426_rv: f64 = *var_guard426_rv_slot;
        let mut var_guard431: f64 = *var_guard431_slot;
        let mut var_guard431_rv: f64 = *var_guard431_rv_slot;
        let mut var_guard432: f64 = *var_guard432_slot;
        let mut var_guard432_rv: f64 = *var_guard432_rv_slot;
        let mut var_guard433: f64 = *var_guard433_slot;
        let mut var_guard433_rv: f64 = *var_guard433_rv_slot;
        let mut var_guard434: f64 = *var_guard434_slot;
        let mut var_guard434_rv: f64 = *var_guard434_rv_slot;
        let mut var_guard435: f64 = *var_guard435_slot;
        let mut var_guard435_rv: f64 = *var_guard435_rv_slot;
        let mut var_inv_vt: f64 = *var_inv_vt_slot;
        let mut var_inv_vt_dn4: f64 = *var_inv_vt_dn4_slot;
        let mut var_inv_vt_dn5: f64 = *var_inv_vt_dn5_slot;
        let mut var_inv_vt_rv: f64 = *var_inv_vt_rv_slot;
        let mut var_rdraingeo: f64 = *var_rdraingeo_slot;
        let mut var_rdraingeo_rv: f64 = *var_rdraingeo_rv_slot;
        let mut var_rdsw_i: f64 = *var_rdsw_i_slot;
        let mut var_rdsw_i_rv: f64 = *var_rdsw_i_rv_slot;
        let mut var_rdswmin_i: f64 = *var_rdswmin_i_slot;
        let mut var_rdswmin_i_rv: f64 = *var_rdswmin_i_rv_slot;
        let mut var_rdw_i: f64 = *var_rdw_i_slot;
        let mut var_rdw_i_rv: f64 = *var_rdw_i_rv_slot;
        let mut var_rdwmin_i: f64 = *var_rdwmin_i_slot;
        let mut var_rdwmin_i_rv: f64 = *var_rdwmin_i_rv_slot;
        let mut var_rsourcegeo: f64 = *var_rsourcegeo_slot;
        let mut var_rsourcegeo_rv: f64 = *var_rsourcegeo_rv_slot;
        let mut var_rsw_i: f64 = *var_rsw_i_slot;
        let mut var_rsw_i_rv: f64 = *var_rsw_i_rv_slot;
        let mut var_rswmin_i: f64 = *var_rswmin_i_slot;
        let mut var_rswmin_i_rv: f64 = *var_rswmin_i_rv_slot;
        let mut var_t0: f64 = *var_t0_slot;
        let mut var_t0_dn10: f64 = *var_t0_dn10_slot;
        let mut var_t0_dn11: f64 = *var_t0_dn11_slot;
        let mut var_t0_dn3: f64 = *var_t0_dn3_slot;
        let mut var_t0_dn4: f64 = *var_t0_dn4_slot;
        let mut var_t0_dn5: f64 = *var_t0_dn5_slot;
        let mut var_t0_dn6: f64 = *var_t0_dn6_slot;
        let mut var_t0_dn7: f64 = *var_t0_dn7_slot;
        let mut var_t0_dn8: f64 = *var_t0_dn8_slot;
        let mut var_t0_dn9: f64 = *var_t0_dn9_slot;
        let mut var_t0_rv: f64 = *var_t0_rv_slot;
        let mut var_t1: f64 = *var_t1_slot;
        let mut var_t1_dn10: f64 = *var_t1_dn10_slot;
        let mut var_t1_dn11: f64 = *var_t1_dn11_slot;
        let mut var_t1_dn3: f64 = *var_t1_dn3_slot;
        let mut var_t1_dn4: f64 = *var_t1_dn4_slot;
        let mut var_t1_dn5: f64 = *var_t1_dn5_slot;
        let mut var_t1_dn6: f64 = *var_t1_dn6_slot;
        let mut var_t1_dn7: f64 = *var_t1_dn7_slot;
        let mut var_t1_dn8: f64 = *var_t1_dn8_slot;
        let mut var_t1_dn9: f64 = *var_t1_dn9_slot;
        let mut var_t1_rv: f64 = *var_t1_rv_slot;
        let mut var_t2: f64 = *var_t2_slot;
        let mut var_t2_dn10: f64 = *var_t2_dn10_slot;
        let mut var_t2_dn11: f64 = *var_t2_dn11_slot;
        let mut var_t2_dn3: f64 = *var_t2_dn3_slot;
        let mut var_t2_dn4: f64 = *var_t2_dn4_slot;
        let mut var_t2_dn5: f64 = *var_t2_dn5_slot;
        let mut var_t2_dn6: f64 = *var_t2_dn6_slot;
        let mut var_t2_dn7: f64 = *var_t2_dn7_slot;
        let mut var_t2_dn8: f64 = *var_t2_dn8_slot;
        let mut var_t2_dn9: f64 = *var_t2_dn9_slot;
        let mut var_t2_rv: f64 = *var_t2_rv_slot;
        let mut var_tnom: f64 = *var_tnom_slot;
        let mut var_tnom_rv: f64 = *var_tnom_rv_slot;
        let mut var_toxratio: f64 = *var_toxratio_slot;
        let mut var_toxratio_dn10: f64 = *var_toxratio_dn10_slot;
        let mut var_toxratio_dn11: f64 = *var_toxratio_dn11_slot;
        let mut var_toxratio_dn3: f64 = *var_toxratio_dn3_slot;
        let mut var_toxratio_dn4: f64 = *var_toxratio_dn4_slot;
        let mut var_toxratio_dn5: f64 = *var_toxratio_dn5_slot;
        let mut var_toxratio_dn6: f64 = *var_toxratio_dn6_slot;
        let mut var_toxratio_dn7: f64 = *var_toxratio_dn7_slot;
        let mut var_toxratio_dn8: f64 = *var_toxratio_dn8_slot;
        let mut var_toxratio_dn9: f64 = *var_toxratio_dn9_slot;
        let mut var_toxratio_rv: f64 = *var_toxratio_rv_slot;
        let mut var_tratio: f64 = *var_tratio_slot;
        let mut var_tratio_dn4: f64 = *var_tratio_dn4_slot;
        let mut var_tratio_dn5: f64 = *var_tratio_dn5_slot;
        let mut var_tratio_rv: f64 = *var_tratio_rv_slot;
        let mut var_vt: f64 = *var_vt_slot;
        let mut var_vt_dn4: f64 = *var_vt_dn4_slot;
        let mut var_vt_dn5: f64 = *var_vt_dn5_slot;
        let mut var_vt_rv: f64 = *var_vt_rv_slot;
        let mut var_vtm: f64 = *var_vtm_slot;
        let mut var_vtm0: f64 = *var_vtm0_slot;
        let mut var_vtm0_rv: f64 = *var_vtm0_rv_slot;
        let mut var_vtm_dn4: f64 = *var_vtm_dn4_slot;
        let mut var_vtm_dn5: f64 = *var_vtm_dn5_slot;
        let mut var_vtm_rv: f64 = *var_vtm_rv_slot;
        let mut var_weff_sh: f64 = *var_weff_sh_slot;
        let mut var_weff_sh_rv: f64 = *var_weff_sh_rv_slot;

        let (assign12690_e18198,) = {
    if ((((var_guard246 == 0.0) && (var_guard247 != 0.0)) && (var_guard411 == 0.0)) && (var_guard412 == 0.0)) {
        let assign12690_e18192: f64 = (var_rint * var_rend);
        let assign12690_e18195: f64 = (var_rint + var_rend);
        let assign12690_e18196: f64 = (assign12690_e18192 / assign12690_e18195);
        (assign12690_e18196,)
    } else {
        (var_rdraingeo,)
    }
};
        var_rdraingeo = assign12690_e18198;
        var_rdraingeo_rv = 0.0;

        let (assign12710_e18209,) = {
    if ((var_guard246 == 0.0) && (var_guard247 == 0.0)) {
        (0.0,)
    } else {
        (var_rdraingeo,)
    }
};
        var_rdraingeo = assign12710_e18209;
        var_rdraingeo_rv = 0.0;

        let assign12720_e18212: f64 = if p.p33 == 0.0 { 1.0 } else { 0.0 };
        var_guard414 = assign12720_e18212;
        var_guard414_rv = 0.0;

        let assign12730_e18215: f64 = if var_rsourcegeo < p.p1347 { 1.0 } else { 0.0 };
        var_guard415 = assign12730_e18215;
        var_guard415_rv = 0.0;

        let (assign12740_e18221,) = {
    if ((var_guard414 != 0.0) && (var_guard415 != 0.0)) {
        (0.0,)
    } else {
        (var_rsourcegeo,)
    }
};
        var_rsourcegeo = assign12740_e18221;
        var_rsourcegeo_rv = 0.0;

        let assign12750_e18224: f64 = if var_rdraingeo < p.p1347 { 1.0 } else { 0.0 };
        var_guard416 = assign12750_e18224;
        var_guard416_rv = 0.0;

        let (assign12760_e18230,) = {
    if ((var_guard414 != 0.0) && (var_guard416 != 0.0)) {
        (0.0,)
    } else {
        (var_rdraingeo,)
    }
};
        var_rdraingeo = assign12760_e18230;
        var_rdraingeo_rv = 0.0;

        let assign12770_e18233: f64 = if var_rsourcegeo <= p.p1347 { 1.0 } else { 0.0 };
        var_guard417 = assign12770_e18233;
        var_guard417_rv = 0.0;

        let (assign12780_e18240,) = {
    if ((var_guard414 == 0.0) && (var_guard417 != 0.0)) {
        (p.p1347,)
    } else {
        (var_rsourcegeo,)
    }
};
        var_rsourcegeo = assign12780_e18240;
        var_rsourcegeo_rv = 0.0;

        let assign12790_e18243: f64 = if var_rdraingeo <= p.p1347 { 1.0 } else { 0.0 };
        var_guard418 = assign12790_e18243;
        var_guard418_rv = 0.0;

        let (assign12800_e18250,) = {
    if ((var_guard414 == 0.0) && (var_guard418 != 0.0)) {
        (p.p1347,)
    } else {
        (var_rdraingeo,)
    }
};
        var_rdraingeo = assign12800_e18250;
        var_rdraingeo_rv = 0.0;

        let assign12810_e18253: f64 = if p.p33 == 1.0 { 1.0 } else { 0.0 };
        var_guard419 = assign12810_e18253;
        var_guard419_rv = 0.0;

        let assign12820_e18256: f64 = if var_rswmin_i <= 0.0 { 1.0 } else { 0.0 };
        var_guard420 = assign12820_e18256;
        var_guard420_rv = 0.0;

        let (assign12830_e18262,) = {
    if ((var_guard419 != 0.0) && (var_guard420 != 0.0)) {
        (0.0,)
    } else {
        (var_rswmin_i,)
    }
};
        var_rswmin_i = assign12830_e18262;
        var_rswmin_i_rv = 0.0;

        let assign12840_e18265: f64 = if var_rdwmin_i <= 0.0 { 1.0 } else { 0.0 };
        var_guard421 = assign12840_e18265;
        var_guard421_rv = 0.0;

        let (assign12850_e18271,) = {
    if ((var_guard419 != 0.0) && (var_guard421 != 0.0)) {
        (0.0,)
    } else {
        (var_rdwmin_i,)
    }
};
        var_rdwmin_i = assign12850_e18271;
        var_rdwmin_i_rv = 0.0;

        let assign12860_e18274: f64 = if var_rsw_i <= 0.0 { 1.0 } else { 0.0 };
        var_guard422 = assign12860_e18274;
        var_guard422_rv = 0.0;

        let (assign12870_e18280,) = {
    if ((var_guard419 != 0.0) && (var_guard422 != 0.0)) {
        (0.0,)
    } else {
        (var_rsw_i,)
    }
};
        var_rsw_i = assign12870_e18280;
        var_rsw_i_rv = 0.0;

        let assign12880_e18283: f64 = if var_rdw_i <= 0.0 { 1.0 } else { 0.0 };
        var_guard423 = assign12880_e18283;
        var_guard423_rv = 0.0;

        let (assign12890_e18289,) = {
    if ((var_guard419 != 0.0) && (var_guard423 != 0.0)) {
        (0.0,)
    } else {
        (var_rdw_i,)
    }
};
        var_rdw_i = assign12890_e18289;
        var_rdw_i_rv = 0.0;

        let assign12900_e18292: f64 = if var_rdswmin_i <= 0.0 { 1.0 } else { 0.0 };
        var_guard424 = assign12900_e18292;
        var_guard424_rv = 0.0;

        let (assign12910_e18299,) = {
    if ((var_guard419 == 0.0) && (var_guard424 != 0.0)) {
        (0.0,)
    } else {
        (var_rdswmin_i,)
    }
};
        var_rdswmin_i = assign12910_e18299;
        var_rdswmin_i_rv = 0.0;

        let assign12920_e18302: f64 = if var_rdsw_i <= 0.0 { 1.0 } else { 0.0 };
        var_guard425 = assign12920_e18302;
        var_guard425_rv = 0.0;

        let (assign12930_e18309,) = {
    if ((var_guard419 == 0.0) && (var_guard425 != 0.0)) {
        (0.0,)
    } else {
        (var_rdsw_i,)
    }
};
        var_rdsw_i = assign12930_e18309;
        var_rdsw_i_rv = 0.0;

        let assign12940_e18314: f64 = (var_weffcj / 3.0);
        let assign12940_e18316: f64 = (assign12940_e18314 / p.p22);
        let assign12940_e18317: f64 = (p.p21 + assign12940_e18316);
        let assign12940_e18318: f64 = (p.p900 * assign12940_e18317);
        let assign12940_e18321: f64 = (p.p22 * p.p2);
        let assign12940_e18324: f64 = (var_lnew - p.p899);
        let assign12940_e18325: f64 = (assign12940_e18321 * assign12940_e18324);
        let assign12940_e18326: f64 = (assign12940_e18318 / assign12940_e18325);
        var_grgeltd = assign12940_e18326;
        var_grgeltd_rv = 0.0;

        let assign12950_e18329: f64 = if var_grgeltd > 0.0 { 1.0 } else { 0.0 };
        var_guard426 = assign12950_e18329;
        var_guard426_rv = 0.0;

        let (assign12960_e18335,) = {
    if (var_guard426 != 0.0) {
        let assign12960_e18333: f64 = (1.0 / var_grgeltd);
        (assign12960_e18333,)
    } else {
        (var_grgeltd,)
    }
};
        var_grgeltd = assign12960_e18335;
        var_grgeltd_rv = 0.0;

        let (assign12970_e18340,) = {
    if (var_guard426 == 0.0) {
        (1000.0,)
    } else {
        (var_grgeltd,)
    }
};
        var_grgeltd = assign12970_e18340;
        var_grgeltd_rv = 0.0;

        let assign12990_e18346: f64 = (p.p76 * p.p76);
        var_t0 = assign12990_e18346;
        var_t0_dn3 = 0.0;
        var_t0_dn4 = 0.0;
        var_t0_dn5 = 0.0;
        var_t0_dn6 = 0.0;
        var_t0_dn7 = 0.0;
        var_t0_dn8 = 0.0;
        var_t0_dn9 = 0.0;
        var_t0_dn10 = 0.0;
        var_t0_dn11 = 0.0;
        var_t0_rv = 0.0;

        let assign13000_e18349: f64 = (p.p76 * var_poxedge_i);
        var_t1 = assign13000_e18349;
        var_t1_dn3 = 0.0;
        var_t1_dn4 = 0.0;
        var_t1_dn5 = 0.0;
        var_t1_dn6 = 0.0;
        var_t1_dn7 = 0.0;
        var_t1_dn8 = 0.0;
        var_t1_dn9 = 0.0;
        var_t1_dn10 = 0.0;
        var_t1_dn11 = 0.0;
        var_t1_rv = 0.0;

        let assign13010_e18352: f64 = (var_t1 * var_t1);
        var_t2 = assign13010_e18352;
        var_t2_dn3 = ((var_t1_dn3 * var_t1) + (var_t1 * var_t1_dn3));
        var_t2_dn4 = ((var_t1_dn4 * var_t1) + (var_t1 * var_t1_dn4));
        var_t2_dn5 = ((var_t1_dn5 * var_t1) + (var_t1 * var_t1_dn5));
        var_t2_dn6 = ((var_t1_dn6 * var_t1) + (var_t1 * var_t1_dn6));
        var_t2_dn7 = ((var_t1_dn7 * var_t1) + (var_t1 * var_t1_dn7));
        var_t2_dn8 = ((var_t1_dn8 * var_t1) + (var_t1 * var_t1_dn8));
        var_t2_dn9 = ((var_t1_dn9 * var_t1) + (var_t1 * var_t1_dn9));
        var_t2_dn10 = ((var_t1_dn10 * var_t1) + (var_t1 * var_t1_dn10));
        var_t2_dn11 = ((var_t1_dn11 * var_t1) + (var_t1 * var_t1_dn11));
        var_t2_rv = 0.0;

        let assign13020_e18356: f64 = (p.p722 / p.p76);
        let assign13020_e18358: f64 = (assign13020_e18356).max(1e-38);
        let assign13020_e18359: f64 = (assign13020_e18358).ln();
        let assign13020_e18360: f64 = (var_ntox_i * assign13020_e18359);
        let assign13020_e18361: f64 = { let limited_exp_arg = assign13020_e18360; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign13020_e18363: f64 = (assign13020_e18361 / var_t0);
        var_toxratio = assign13020_e18363;
        var_toxratio_dn3 = (-((assign13020_e18361 * var_t0_dn3) / (var_t0 * var_t0)));
        var_toxratio_dn4 = (-((assign13020_e18361 * var_t0_dn4) / (var_t0 * var_t0)));
        var_toxratio_dn5 = (-((assign13020_e18361 * var_t0_dn5) / (var_t0 * var_t0)));
        var_toxratio_dn6 = (-((assign13020_e18361 * var_t0_dn6) / (var_t0 * var_t0)));
        var_toxratio_dn7 = (-((assign13020_e18361 * var_t0_dn7) / (var_t0 * var_t0)));
        var_toxratio_dn8 = (-((assign13020_e18361 * var_t0_dn8) / (var_t0 * var_t0)));
        var_toxratio_dn9 = (-((assign13020_e18361 * var_t0_dn9) / (var_t0 * var_t0)));
        var_toxratio_dn10 = (-((assign13020_e18361 * var_t0_dn10) / (var_t0 * var_t0)));
        var_toxratio_dn11 = (-((assign13020_e18361 * var_t0_dn11) / (var_t0 * var_t0)));
        var_toxratio_rv = 0.0;

        let (assign13050_e18386,) = {
    if (p.p30 == 1.0) {
        (p.p705,)
    } else {
        (p.p704,)
    }
};
        var_bechvb = assign13050_e18386;
        var_bechvb_rv = 0.0;

        let assign13080_e18406: f64 = (-var_bechvb);
        let assign13080_e18408: f64 = (assign13080_e18406 * p.p76);
        let assign13080_e18410: f64 = (assign13080_e18408 * var_poxedge_i);
        var_bechvbedge = assign13080_e18410;
        var_bechvbedge_rv = 0.0;

        let assign13100_e18425: f64 = (-var_bechvb);
        let assign13100_e18427: f64 = (assign13100_e18425 * p.p76);
        var_bechvb = assign13100_e18427;
        var_bechvb_rv = 0.0;

        let assign13110_e18430: f64 = (p.p1101 + var_weff);
        var_weff_sh = assign13110_e18430;
        var_weff_sh_rv = 0.0;

        let assign13150_e18459: f64 = if (((p.p41 != 0.0) && (p.p1099 > 0.0)) && (var_weff_sh > 0.0)) { 1.0 } else { 0.0 };
        var_guard431 = assign13150_e18459;
        var_guard431_rv = 0.0;

        let (assign13160_e18467,) = {
    if (var_guard431 != 0.0) {
        let assign13160_e18463: f64 = (var_weff_sh * p.p2);
        let assign13160_e18465: f64 = (assign13160_e18463 / p.p1099);
        (assign13160_e18465,)
    } else {
        (var_gth,)
    }
};
        var_gth = assign13160_e18467;
        var_gth_rv = 0.0;

        let (assign13170_e18475,) = {
    if (var_guard431 != 0.0) {
        let assign13170_e18471: f64 = (p.p1100 * var_weff_sh);
        let assign13170_e18473: f64 = (assign13170_e18471 * p.p2);
        (assign13170_e18473,)
    } else {
        (var_cth,)
    }
};
        var_cth = assign13170_e18475;
        var_cth_rv = 0.0;

        let (assign13180_e18480,) = {
    if (var_guard431 == 0.0) {
        (1.0,)
    } else {
        (var_gth,)
    }
};
        var_gth = assign13180_e18480;
        var_gth_rv = 0.0;

        let (assign13190_e18485,) = {
    if (var_guard431 == 0.0) {
        (0.0,)
    } else {
        (var_cth,)
    }
};
        var_cth = assign13190_e18485;
        var_cth_rv = 0.0;

        let assign13200_e18488: f64 = (-273.15);
        let assign13200_e18489: f64 = if p.p1028 <= assign13200_e18488 { 1.0 } else { 0.0 };
        var_guard432 = assign13200_e18489;
        var_guard432_rv = 0.0;

        let (assign13210_e18495, assign13210_e18495_d_n3, assign13210_e18495_d_n4, assign13210_e18495_d_n5, assign13210_e18495_d_n6, assign13210_e18495_d_n7, assign13210_e18495_d_n8, assign13210_e18495_d_n9, assign13210_e18495_d_n10, assign13210_e18495_d_n11,) = {
    if (var_guard432 != 0.0) {
        let assign13210_e18493: f64 = (300.15 - 273.15);
        (assign13210_e18493, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t0, var_t0_dn3, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8, var_t0_dn9, var_t0_dn10, var_t0_dn11,)
    }
};
        var_t0 = assign13210_e18495;
        var_t0_dn3 = assign13210_e18495_d_n3;
        var_t0_dn4 = assign13210_e18495_d_n4;
        var_t0_dn5 = assign13210_e18495_d_n5;
        var_t0_dn6 = assign13210_e18495_d_n6;
        var_t0_dn7 = assign13210_e18495_d_n7;
        var_t0_dn8 = assign13210_e18495_d_n8;
        var_t0_dn9 = assign13210_e18495_d_n9;
        var_t0_dn10 = assign13210_e18495_d_n10;
        var_t0_dn11 = assign13210_e18495_d_n11;
        var_t0_rv = 0.0;

        let (assign13220_e18499,) = {
    if (var_guard432 != 0.0) {
        (300.15,)
    } else {
        (var_tnom,)
    }
};
        var_tnom = assign13220_e18499;
        var_tnom_rv = 0.0;

        let (assign13230_e18506,) = {
    if (var_guard432 == 0.0) {
        let assign13230_e18504: f64 = (p.p1028 + 273.15);
        (assign13230_e18504,)
    } else {
        (var_tnom,)
    }
};
        var_tnom = assign13230_e18506;
        var_tnom_rv = 0.0;

        let assign13240_e18507: f64 = ctx_temp;
        let assign13240_e18509: f64 = (assign13240_e18507 + p.p23);
        var_devtemp = assign13240_e18509;
        var_devtemp_dn4 = 0.0;
        var_devtemp_dn5 = 0.0;
        var_devtemp_rv = 0.0;

        let assign13250_e18516: f64 = if ((p.p41 != 0.0) && (p.p1099 > 0.0)) { 1.0 } else { 0.0 };
        var_guard433 = assign13250_e18516;
        var_guard433_rv = 0.0;

        let assign13260_e18523: f64 = if ((p.p40 != 0.0) && (!true)) { 1.0 } else { 0.0 };
        var_guard434 = assign13260_e18523;
        var_guard434_rv = 0.0;

        let assign13270_e18525: f64 = 1.0;
        var_guard435 = assign13270_e18525;
        var_guard435_rv = 0.0;

        let (assign13280_e18533, assign13280_e18533_d_n4, assign13280_e18533_d_n5,) = {
    if (((var_guard433 != 0.0) && (var_guard434 != 0.0)) && (var_guard435 != 0.0)) {
        ((nv4 - 0.0), 1.0, 0.0,)
    } else {
        (var_deltemp1, var_deltemp1_dn4, var_deltemp1_dn5,)
    }
};
        var_deltemp1 = assign13280_e18533;
        var_deltemp1_dn4 = assign13280_e18533_d_n4;
        var_deltemp1_dn5 = assign13280_e18533_d_n5;
        var_deltemp1_rv = 0.0;

        let (assign13290_e18542, assign13290_e18542_d_n4, assign13290_e18542_d_n5,) = {
    if (((var_guard433 != 0.0) && (var_guard434 != 0.0)) && (var_guard435 == 0.0)) {
        ((nv5 - 0.0), 0.0, 1.0,)
    } else {
        (var_deltemp1, var_deltemp1_dn4, var_deltemp1_dn5,)
    }
};
        var_deltemp1 = assign13290_e18542;
        var_deltemp1_dn4 = assign13290_e18542_d_n4;
        var_deltemp1_dn5 = assign13290_e18542_d_n5;
        var_deltemp1_rv = 0.0;

        let (assign13300_e18549, assign13300_e18549_d_n4, assign13300_e18549_d_n5,) = {
    if ((var_guard433 != 0.0) && (var_guard434 == 0.0)) {
        ((nv5 - 0.0), 0.0, 1.0,)
    } else {
        (var_deltemp1, var_deltemp1_dn4, var_deltemp1_dn5,)
    }
};
        var_deltemp1 = assign13300_e18549;
        var_deltemp1_dn4 = assign13300_e18549_d_n4;
        var_deltemp1_dn5 = assign13300_e18549_d_n5;
        var_deltemp1_rv = 0.0;

        let (assign13310_e18554, assign13310_e18554_d_n4, assign13310_e18554_d_n5,) = {
    if (var_guard433 == 0.0) {
        (0.0, 0.0, 0.0,)
    } else {
        (var_deltemp1, var_deltemp1_dn4, var_deltemp1_dn5,)
    }
};
        var_deltemp1 = assign13310_e18554;
        var_deltemp1_dn4 = assign13310_e18554_d_n4;
        var_deltemp1_dn5 = assign13310_e18554_d_n5;
        var_deltemp1_rv = 0.0;

        let assign13320_e18557: f64 = (var_deltemp1 + var_devtemp);
        var_devtemp = assign13320_e18557;
        var_devtemp_dn4 = (var_deltemp1_dn4 + var_devtemp_dn4);
        var_devtemp_dn5 = (var_deltemp1_dn5 + var_devtemp_dn5);
        var_devtemp_rv = 0.0;

        let assign13360_e18565: f64 = (var_kboq * var_devtemp);
        var_vt = assign13360_e18565;
        var_vt_dn4 = (var_kboq * var_devtemp_dn4);
        var_vt_dn5 = (var_kboq * var_devtemp_dn5);
        var_vt_rv = 0.0;

        let assign13370_e18568: f64 = (1.0 / var_vt);
        var_inv_vt = assign13370_e18568;
        var_inv_vt_dn4 = (-(var_vt_dn4 / (var_vt * var_vt)));
        var_inv_vt_dn5 = (-(var_vt_dn5 / (var_vt * var_vt)));
        var_inv_vt_rv = 0.0;

        let assign13380_e18571: f64 = (var_devtemp / var_tnom);
        var_tratio = assign13380_e18571;
        var_tratio_dn4 = (var_devtemp_dn4 / var_tnom);
        var_tratio_dn5 = (var_devtemp_dn5 / var_tnom);
        var_tratio_rv = 0.0;

        let assign13390_e18574: f64 = (var_devtemp - var_tnom);
        var_deltemp = assign13390_e18574;
        var_deltemp_dn4 = var_devtemp_dn4;
        var_deltemp_dn5 = var_devtemp_dn5;
        var_deltemp_rv = 0.0;

        let assign13400_e18577: f64 = (var_kboq * var_devtemp);
        var_vtm = assign13400_e18577;
        var_vtm_dn4 = (var_kboq * var_devtemp_dn4);
        var_vtm_dn5 = (var_kboq * var_devtemp_dn5);
        var_vtm_rv = 0.0;

        let assign13410_e18580: f64 = (var_kboq * var_tnom);
        var_vtm0 = assign13410_e18580;
        var_vtm0_rv = 0.0;

        let assign13420_e18584: f64 = (p.p1029 * var_devtemp);
        let assign13420_e18586: f64 = (assign13420_e18584 * var_devtemp);
        let assign13420_e18589: f64 = (var_devtemp + p.p1030);
        let assign13420_e18590: f64 = (assign13420_e18586 / assign13420_e18589);
        let assign13420_e18591: f64 = (p.p108 - assign13420_e18590);
        var_eg = assign13420_e18591;
        var_eg_dn4 = (-((((((p.p1029 * var_devtemp_dn4) * var_devtemp) + (assign13420_e18584 * var_devtemp_dn4)) * assign13420_e18589) - (assign13420_e18586 * var_devtemp_dn4)) / (assign13420_e18589 * assign13420_e18589)));
        var_eg_dn5 = (-((((((p.p1029 * var_devtemp_dn5) * var_devtemp) + (assign13420_e18584 * var_devtemp_dn5)) * assign13420_e18589) - (assign13420_e18586 * var_devtemp_dn5)) / (assign13420_e18589 * assign13420_e18589)));
        var_eg_rv = 0.0;

        *var_bechvb_slot = var_bechvb;
        *var_bechvb_rv_slot = var_bechvb_rv;
        *var_bechvbedge_slot = var_bechvbedge;
        *var_bechvbedge_rv_slot = var_bechvbedge_rv;
        *var_cth_slot = var_cth;
        *var_cth_rv_slot = var_cth_rv;
        *var_deltemp_slot = var_deltemp;
        *var_deltemp1_slot = var_deltemp1;
        *var_deltemp1_dn4_slot = var_deltemp1_dn4;
        *var_deltemp1_dn5_slot = var_deltemp1_dn5;
        *var_deltemp1_rv_slot = var_deltemp1_rv;
        *var_deltemp_dn4_slot = var_deltemp_dn4;
        *var_deltemp_dn5_slot = var_deltemp_dn5;
        *var_deltemp_rv_slot = var_deltemp_rv;
        *var_devtemp_slot = var_devtemp;
        *var_devtemp_dn4_slot = var_devtemp_dn4;
        *var_devtemp_dn5_slot = var_devtemp_dn5;
        *var_devtemp_rv_slot = var_devtemp_rv;
        *var_eg_slot = var_eg;
        *var_eg_dn4_slot = var_eg_dn4;
        *var_eg_dn5_slot = var_eg_dn5;
        *var_eg_rv_slot = var_eg_rv;
        *var_grgeltd_slot = var_grgeltd;
        *var_grgeltd_rv_slot = var_grgeltd_rv;
        *var_gth_slot = var_gth;
        *var_gth_rv_slot = var_gth_rv;
        *var_guard414_slot = var_guard414;
        *var_guard414_rv_slot = var_guard414_rv;
        *var_guard415_slot = var_guard415;
        *var_guard415_rv_slot = var_guard415_rv;
        *var_guard416_slot = var_guard416;
        *var_guard416_rv_slot = var_guard416_rv;
        *var_guard417_slot = var_guard417;
        *var_guard417_rv_slot = var_guard417_rv;
        *var_guard418_slot = var_guard418;
        *var_guard418_rv_slot = var_guard418_rv;
        *var_guard419_slot = var_guard419;
        *var_guard419_rv_slot = var_guard419_rv;
        *var_guard420_slot = var_guard420;
        *var_guard420_rv_slot = var_guard420_rv;
        *var_guard421_slot = var_guard421;
        *var_guard421_rv_slot = var_guard421_rv;
        *var_guard422_slot = var_guard422;
        *var_guard422_rv_slot = var_guard422_rv;
        *var_guard423_slot = var_guard423;
        *var_guard423_rv_slot = var_guard423_rv;
        *var_guard424_slot = var_guard424;
        *var_guard424_rv_slot = var_guard424_rv;
        *var_guard425_slot = var_guard425;
        *var_guard425_rv_slot = var_guard425_rv;
        *var_guard426_slot = var_guard426;
        *var_guard426_rv_slot = var_guard426_rv;
        *var_guard431_slot = var_guard431;
        *var_guard431_rv_slot = var_guard431_rv;
        *var_guard432_slot = var_guard432;
        *var_guard432_rv_slot = var_guard432_rv;
        *var_guard433_slot = var_guard433;
        *var_guard433_rv_slot = var_guard433_rv;
        *var_guard434_slot = var_guard434;
        *var_guard434_rv_slot = var_guard434_rv;
        *var_guard435_slot = var_guard435;
        *var_guard435_rv_slot = var_guard435_rv;
        *var_inv_vt_slot = var_inv_vt;
        *var_inv_vt_dn4_slot = var_inv_vt_dn4;
        *var_inv_vt_dn5_slot = var_inv_vt_dn5;
        *var_inv_vt_rv_slot = var_inv_vt_rv;
        *var_rdraingeo_slot = var_rdraingeo;
        *var_rdraingeo_rv_slot = var_rdraingeo_rv;
        *var_rdsw_i_slot = var_rdsw_i;
        *var_rdsw_i_rv_slot = var_rdsw_i_rv;
        *var_rdswmin_i_slot = var_rdswmin_i;
        *var_rdswmin_i_rv_slot = var_rdswmin_i_rv;
        *var_rdw_i_slot = var_rdw_i;
        *var_rdw_i_rv_slot = var_rdw_i_rv;
        *var_rdwmin_i_slot = var_rdwmin_i;
        *var_rdwmin_i_rv_slot = var_rdwmin_i_rv;
        *var_rsourcegeo_slot = var_rsourcegeo;
        *var_rsourcegeo_rv_slot = var_rsourcegeo_rv;
        *var_rsw_i_slot = var_rsw_i;
        *var_rsw_i_rv_slot = var_rsw_i_rv;
        *var_rswmin_i_slot = var_rswmin_i;
        *var_rswmin_i_rv_slot = var_rswmin_i_rv;
        *var_t0_slot = var_t0;
        *var_t0_dn10_slot = var_t0_dn10;
        *var_t0_dn11_slot = var_t0_dn11;
        *var_t0_dn3_slot = var_t0_dn3;
        *var_t0_dn4_slot = var_t0_dn4;
        *var_t0_dn5_slot = var_t0_dn5;
        *var_t0_dn6_slot = var_t0_dn6;
        *var_t0_dn7_slot = var_t0_dn7;
        *var_t0_dn8_slot = var_t0_dn8;
        *var_t0_dn9_slot = var_t0_dn9;
        *var_t0_rv_slot = var_t0_rv;
        *var_t1_slot = var_t1;
        *var_t1_dn10_slot = var_t1_dn10;
        *var_t1_dn11_slot = var_t1_dn11;
        *var_t1_dn3_slot = var_t1_dn3;
        *var_t1_dn4_slot = var_t1_dn4;
        *var_t1_dn5_slot = var_t1_dn5;
        *var_t1_dn6_slot = var_t1_dn6;
        *var_t1_dn7_slot = var_t1_dn7;
        *var_t1_dn8_slot = var_t1_dn8;
        *var_t1_dn9_slot = var_t1_dn9;
        *var_t1_rv_slot = var_t1_rv;
        *var_t2_slot = var_t2;
        *var_t2_dn10_slot = var_t2_dn10;
        *var_t2_dn11_slot = var_t2_dn11;
        *var_t2_dn3_slot = var_t2_dn3;
        *var_t2_dn4_slot = var_t2_dn4;
        *var_t2_dn5_slot = var_t2_dn5;
        *var_t2_dn6_slot = var_t2_dn6;
        *var_t2_dn7_slot = var_t2_dn7;
        *var_t2_dn8_slot = var_t2_dn8;
        *var_t2_dn9_slot = var_t2_dn9;
        *var_t2_rv_slot = var_t2_rv;
        *var_tnom_slot = var_tnom;
        *var_tnom_rv_slot = var_tnom_rv;
        *var_toxratio_slot = var_toxratio;
        *var_toxratio_dn10_slot = var_toxratio_dn10;
        *var_toxratio_dn11_slot = var_toxratio_dn11;
        *var_toxratio_dn3_slot = var_toxratio_dn3;
        *var_toxratio_dn4_slot = var_toxratio_dn4;
        *var_toxratio_dn5_slot = var_toxratio_dn5;
        *var_toxratio_dn6_slot = var_toxratio_dn6;
        *var_toxratio_dn7_slot = var_toxratio_dn7;
        *var_toxratio_dn8_slot = var_toxratio_dn8;
        *var_toxratio_dn9_slot = var_toxratio_dn9;
        *var_toxratio_rv_slot = var_toxratio_rv;
        *var_tratio_slot = var_tratio;
        *var_tratio_dn4_slot = var_tratio_dn4;
        *var_tratio_dn5_slot = var_tratio_dn5;
        *var_tratio_rv_slot = var_tratio_rv;
        *var_vt_slot = var_vt;
        *var_vt_dn4_slot = var_vt_dn4;
        *var_vt_dn5_slot = var_vt_dn5;
        *var_vt_rv_slot = var_vt_rv;
        *var_vtm_slot = var_vtm;
        *var_vtm0_slot = var_vtm0;
        *var_vtm0_rv_slot = var_vtm0_rv;
        *var_vtm_dn4_slot = var_vtm_dn4;
        *var_vtm_dn5_slot = var_vtm_dn5;
        *var_vtm_rv_slot = var_vtm_rv;
        *var_weff_sh_slot = var_weff_sh;
        *var_weff_sh_rv_slot = var_weff_sh_rv;
    }

    pub(super) fn stamp_reactive_block_22(
        p: &Parameters,
        var_deltemp: f64,
        var_deltemp_dn4: f64,
        var_deltemp_dn5: f64,
        var_devsign: f64,
        var_devtemp: f64,
        var_devtemp_dn4: f64,
        var_devtemp_dn5: f64,
        var_eg: f64,
        var_eg_dn4: f64,
        var_eg_dn5: f64,
        var_epsox: f64,
        var_epssi: f64,
        var_eta0_i: f64,
        var_eta0_i_dn10: f64,
        var_eta0_i_dn11: f64,
        var_eta0_i_dn3: f64,
        var_eta0_i_dn4: f64,
        var_eta0_i_dn5: f64,
        var_eta0_i_dn6: f64,
        var_eta0_i_dn7: f64,
        var_eta0_i_dn8: f64,
        var_eta0_i_dn9: f64,
        var_eta0r_i: f64,
        var_eta0r_i_dn10: f64,
        var_eta0r_i_dn11: f64,
        var_eta0r_i_dn3: f64,
        var_eta0r_i_dn4: f64,
        var_eta0r_i_dn5: f64,
        var_eta0r_i_dn6: f64,
        var_eta0r_i_dn7: f64,
        var_eta0r_i_dn8: f64,
        var_eta0r_i_dn9: f64,
        var_eu1_i: f64,
        var_eu_i: f64,
        var_eu_i_dn10: f64,
        var_eu_i_dn11: f64,
        var_eu_i_dn3: f64,
        var_eu_i_dn4: f64,
        var_eu_i_dn5: f64,
        var_eu_i_dn6: f64,
        var_eu_i_dn7: f64,
        var_eu_i_dn8: f64,
        var_eu_i_dn9: f64,
        var_ndep_i: f64,
        var_ndep_i_dn10: f64,
        var_ndep_i_dn11: f64,
        var_ndep_i_dn3: f64,
        var_ndep_i_dn4: f64,
        var_ndep_i_dn5: f64,
        var_ndep_i_dn6: f64,
        var_ndep_i_dn7: f64,
        var_ndep_i_dn8: f64,
        var_ndep_i_dn9: f64,
        var_ndepedge_i: f64,
        var_nfactor_i: f64,
        var_nfactor_i_dn10: f64,
        var_nfactor_i_dn11: f64,
        var_nfactor_i_dn3: f64,
        var_nfactor_i_dn4: f64,
        var_nfactor_i_dn5: f64,
        var_nfactor_i_dn6: f64,
        var_nfactor_i_dn7: f64,
        var_nfactor_i_dn8: f64,
        var_nfactor_i_dn9: f64,
        var_ngate_i: f64,
        var_nsd_i: f64,
        var_phin_i: f64,
        var_tnom: f64,
        var_tratio: f64,
        var_tratio_dn4: f64,
        var_tratio_dn5: f64,
        var_u0_i: f64,
        var_u0r_i: f64,
        var_ua1_i: f64,
        var_ua_i: f64,
        var_ua_i_dn10: f64,
        var_ua_i_dn11: f64,
        var_ua_i_dn3: f64,
        var_ua_i_dn4: f64,
        var_ua_i_dn5: f64,
        var_ua_i_dn6: f64,
        var_ua_i_dn7: f64,
        var_ua_i_dn8: f64,
        var_ua_i_dn9: f64,
        var_uc1_i: f64,
        var_uc_i: f64,
        var_uc_i_dn10: f64,
        var_uc_i_dn11: f64,
        var_uc_i_dn3: f64,
        var_uc_i_dn4: f64,
        var_uc_i_dn5: f64,
        var_uc_i_dn6: f64,
        var_uc_i_dn7: f64,
        var_uc_i_dn8: f64,
        var_uc_i_dn9: f64,
        var_ucs_i: f64,
        var_ucste_i: f64,
        var_ud1_i: f64,
        var_ud_i: f64,
        var_ud_i_dn10: f64,
        var_ud_i_dn11: f64,
        var_ud_i_dn3: f64,
        var_ud_i_dn4: f64,
        var_ud_i_dn5: f64,
        var_ud_i_dn6: f64,
        var_ud_i_dn7: f64,
        var_ud_i_dn8: f64,
        var_ud_i_dn9: f64,
        var_ute_i: f64,
        var_vt: f64,
        var_vt_dn4: f64,
        var_vt_dn5: f64,
        var_vtm: f64,
        var_vtm0: f64,
        var_vtm_dn4: f64,
        var_vtm_dn5: f64,
        var_weff_sh: f64,
        var_xj_i: f64,
        var_eta0_t_slot: &mut f64,
        var_eta0_t_dn10_slot: &mut f64,
        var_eta0_t_dn11_slot: &mut f64,
        var_eta0_t_dn3_slot: &mut f64,
        var_eta0_t_dn4_slot: &mut f64,
        var_eta0_t_dn5_slot: &mut f64,
        var_eta0_t_dn6_slot: &mut f64,
        var_eta0_t_dn7_slot: &mut f64,
        var_eta0_t_dn8_slot: &mut f64,
        var_eta0_t_dn9_slot: &mut f64,
        var_eta0_t_rv_slot: &mut f64,
        var_eta0r_t_slot: &mut f64,
        var_eta0r_t_dn10_slot: &mut f64,
        var_eta0r_t_dn11_slot: &mut f64,
        var_eta0r_t_dn3_slot: &mut f64,
        var_eta0r_t_dn4_slot: &mut f64,
        var_eta0r_t_dn5_slot: &mut f64,
        var_eta0r_t_dn6_slot: &mut f64,
        var_eta0r_t_dn7_slot: &mut f64,
        var_eta0r_t_dn8_slot: &mut f64,
        var_eta0r_t_dn9_slot: &mut f64,
        var_eta0r_t_rv_slot: &mut f64,
        var_eta_mu_slot: &mut f64,
        var_eta_mu_rv_slot: &mut f64,
        var_eu_t_slot: &mut f64,
        var_eu_t_dn10_slot: &mut f64,
        var_eu_t_dn11_slot: &mut f64,
        var_eu_t_dn3_slot: &mut f64,
        var_eu_t_dn4_slot: &mut f64,
        var_eu_t_dn5_slot: &mut f64,
        var_eu_t_dn6_slot: &mut f64,
        var_eu_t_dn7_slot: &mut f64,
        var_eu_t_dn8_slot: &mut f64,
        var_eu_t_dn9_slot: &mut f64,
        var_eu_t_rv_slot: &mut f64,
        var_guard436_slot: &mut f64,
        var_guard436_rv_slot: &mut f64,
        var_guard437_slot: &mut f64,
        var_guard437_rv_slot: &mut f64,
        var_guard438_slot: &mut f64,
        var_guard438_rv_slot: &mut f64,
        var_guard449_slot: &mut f64,
        var_guard449_rv_slot: &mut f64,
        var_guard450_slot: &mut f64,
        var_guard450_rv_slot: &mut f64,
        var_litl_slot: &mut f64,
        var_litl_rv_slot: &mut f64,
        var_nfactor_t_slot: &mut f64,
        var_nfactor_t_dn10_slot: &mut f64,
        var_nfactor_t_dn11_slot: &mut f64,
        var_nfactor_t_dn3_slot: &mut f64,
        var_nfactor_t_dn4_slot: &mut f64,
        var_nfactor_t_dn5_slot: &mut f64,
        var_nfactor_t_dn6_slot: &mut f64,
        var_nfactor_t_dn7_slot: &mut f64,
        var_nfactor_t_dn8_slot: &mut f64,
        var_nfactor_t_dn9_slot: &mut f64,
        var_nfactor_t_rv_slot: &mut f64,
        var_ni_slot: &mut f64,
        var_ni_dn10_slot: &mut f64,
        var_ni_dn11_slot: &mut f64,
        var_ni_dn3_slot: &mut f64,
        var_ni_dn4_slot: &mut f64,
        var_ni_dn5_slot: &mut f64,
        var_ni_dn6_slot: &mut f64,
        var_ni_dn7_slot: &mut f64,
        var_ni_dn8_slot: &mut f64,
        var_ni_dn9_slot: &mut f64,
        var_ni_rv_slot: &mut f64,
        var_phib_slot: &mut f64,
        var_phib_dn10_slot: &mut f64,
        var_phib_dn11_slot: &mut f64,
        var_phib_dn3_slot: &mut f64,
        var_phib_dn4_slot: &mut f64,
        var_phib_dn5_slot: &mut f64,
        var_phib_dn6_slot: &mut f64,
        var_phib_dn7_slot: &mut f64,
        var_phib_dn8_slot: &mut f64,
        var_phib_dn9_slot: &mut f64,
        var_phib_rv_slot: &mut f64,
        var_phist_slot: &mut f64,
        var_phist_dn10_slot: &mut f64,
        var_phist_dn11_slot: &mut f64,
        var_phist_dn3_slot: &mut f64,
        var_phist_dn4_slot: &mut f64,
        var_phist_dn5_slot: &mut f64,
        var_phist_dn6_slot: &mut f64,
        var_phist_dn7_slot: &mut f64,
        var_phist_dn8_slot: &mut f64,
        var_phist_dn9_slot: &mut f64,
        var_phist_rv_slot: &mut f64,
        var_sqrtphist_slot: &mut f64,
        var_sqrtphist_dn10_slot: &mut f64,
        var_sqrtphist_dn11_slot: &mut f64,
        var_sqrtphist_dn3_slot: &mut f64,
        var_sqrtphist_dn4_slot: &mut f64,
        var_sqrtphist_dn5_slot: &mut f64,
        var_sqrtphist_dn6_slot: &mut f64,
        var_sqrtphist_dn7_slot: &mut f64,
        var_sqrtphist_dn8_slot: &mut f64,
        var_sqrtphist_dn9_slot: &mut f64,
        var_sqrtphist_rv_slot: &mut f64,
        var_t0_slot: &mut f64,
        var_t0_dn10_slot: &mut f64,
        var_t0_dn11_slot: &mut f64,
        var_t0_dn3_slot: &mut f64,
        var_t0_dn4_slot: &mut f64,
        var_t0_dn5_slot: &mut f64,
        var_t0_dn6_slot: &mut f64,
        var_t0_dn7_slot: &mut f64,
        var_t0_dn8_slot: &mut f64,
        var_t0_dn9_slot: &mut f64,
        var_t0_rv_slot: &mut f64,
        var_t1_slot: &mut f64,
        var_t1_dn10_slot: &mut f64,
        var_t1_dn11_slot: &mut f64,
        var_t1_dn3_slot: &mut f64,
        var_t1_dn4_slot: &mut f64,
        var_t1_dn5_slot: &mut f64,
        var_t1_dn6_slot: &mut f64,
        var_t1_dn7_slot: &mut f64,
        var_t1_dn8_slot: &mut f64,
        var_t1_dn9_slot: &mut f64,
        var_t1_rv_slot: &mut f64,
        var_t1dep_slot: &mut f64,
        var_t1dep_dn10_slot: &mut f64,
        var_t1dep_dn11_slot: &mut f64,
        var_t1dep_dn3_slot: &mut f64,
        var_t1dep_dn4_slot: &mut f64,
        var_t1dep_dn5_slot: &mut f64,
        var_t1dep_dn6_slot: &mut f64,
        var_t1dep_dn7_slot: &mut f64,
        var_t1dep_dn8_slot: &mut f64,
        var_t1dep_dn9_slot: &mut f64,
        var_t1dep_rv_slot: &mut f64,
        var_u0_t_slot: &mut f64,
        var_u0_t_dn10_slot: &mut f64,
        var_u0_t_dn11_slot: &mut f64,
        var_u0_t_dn3_slot: &mut f64,
        var_u0_t_dn4_slot: &mut f64,
        var_u0_t_dn5_slot: &mut f64,
        var_u0_t_dn6_slot: &mut f64,
        var_u0_t_dn7_slot: &mut f64,
        var_u0_t_dn8_slot: &mut f64,
        var_u0_t_dn9_slot: &mut f64,
        var_u0_t_rv_slot: &mut f64,
        var_u0r_t_slot: &mut f64,
        var_u0r_t_dn4_slot: &mut f64,
        var_u0r_t_dn5_slot: &mut f64,
        var_u0r_t_rv_slot: &mut f64,
        var_ua_t_slot: &mut f64,
        var_ua_t_dn10_slot: &mut f64,
        var_ua_t_dn11_slot: &mut f64,
        var_ua_t_dn3_slot: &mut f64,
        var_ua_t_dn4_slot: &mut f64,
        var_ua_t_dn5_slot: &mut f64,
        var_ua_t_dn6_slot: &mut f64,
        var_ua_t_dn7_slot: &mut f64,
        var_ua_t_dn8_slot: &mut f64,
        var_ua_t_dn9_slot: &mut f64,
        var_ua_t_rv_slot: &mut f64,
        var_uc_t_slot: &mut f64,
        var_uc_t_dn10_slot: &mut f64,
        var_uc_t_dn11_slot: &mut f64,
        var_uc_t_dn3_slot: &mut f64,
        var_uc_t_dn4_slot: &mut f64,
        var_uc_t_dn5_slot: &mut f64,
        var_uc_t_dn6_slot: &mut f64,
        var_uc_t_dn7_slot: &mut f64,
        var_uc_t_dn8_slot: &mut f64,
        var_uc_t_dn9_slot: &mut f64,
        var_uc_t_rv_slot: &mut f64,
        var_ucs_t_slot: &mut f64,
        var_ucs_t_dn4_slot: &mut f64,
        var_ucs_t_dn5_slot: &mut f64,
        var_ucs_t_rv_slot: &mut f64,
        var_ud_t_slot: &mut f64,
        var_ud_t_dn10_slot: &mut f64,
        var_ud_t_dn11_slot: &mut f64,
        var_ud_t_dn3_slot: &mut f64,
        var_ud_t_dn4_slot: &mut f64,
        var_ud_t_dn5_slot: &mut f64,
        var_ud_t_dn6_slot: &mut f64,
        var_ud_t_dn7_slot: &mut f64,
        var_ud_t_dn8_slot: &mut f64,
        var_ud_t_dn9_slot: &mut f64,
        var_ud_t_rv_slot: &mut f64,
        var_vbi_edge_slot: &mut f64,
        var_vbi_edge_dn10_slot: &mut f64,
        var_vbi_edge_dn11_slot: &mut f64,
        var_vbi_edge_dn3_slot: &mut f64,
        var_vbi_edge_dn4_slot: &mut f64,
        var_vbi_edge_dn5_slot: &mut f64,
        var_vbi_edge_dn6_slot: &mut f64,
        var_vbi_edge_dn7_slot: &mut f64,
        var_vbi_edge_dn8_slot: &mut f64,
        var_vbi_edge_dn9_slot: &mut f64,
        var_vbi_edge_rv_slot: &mut f64,
        var_vfbsdr_slot: &mut f64,
        var_vfbsdr_dn4_slot: &mut f64,
        var_vfbsdr_dn5_slot: &mut f64,
        var_vfbsdr_rv_slot: &mut f64,
    ) {
        let mut var_eta0_t: f64 = *var_eta0_t_slot;
        let mut var_eta0_t_dn10: f64 = *var_eta0_t_dn10_slot;
        let mut var_eta0_t_dn11: f64 = *var_eta0_t_dn11_slot;
        let mut var_eta0_t_dn3: f64 = *var_eta0_t_dn3_slot;
        let mut var_eta0_t_dn4: f64 = *var_eta0_t_dn4_slot;
        let mut var_eta0_t_dn5: f64 = *var_eta0_t_dn5_slot;
        let mut var_eta0_t_dn6: f64 = *var_eta0_t_dn6_slot;
        let mut var_eta0_t_dn7: f64 = *var_eta0_t_dn7_slot;
        let mut var_eta0_t_dn8: f64 = *var_eta0_t_dn8_slot;
        let mut var_eta0_t_dn9: f64 = *var_eta0_t_dn9_slot;
        let mut var_eta0_t_rv: f64 = *var_eta0_t_rv_slot;
        let mut var_eta0r_t: f64 = *var_eta0r_t_slot;
        let mut var_eta0r_t_dn10: f64 = *var_eta0r_t_dn10_slot;
        let mut var_eta0r_t_dn11: f64 = *var_eta0r_t_dn11_slot;
        let mut var_eta0r_t_dn3: f64 = *var_eta0r_t_dn3_slot;
        let mut var_eta0r_t_dn4: f64 = *var_eta0r_t_dn4_slot;
        let mut var_eta0r_t_dn5: f64 = *var_eta0r_t_dn5_slot;
        let mut var_eta0r_t_dn6: f64 = *var_eta0r_t_dn6_slot;
        let mut var_eta0r_t_dn7: f64 = *var_eta0r_t_dn7_slot;
        let mut var_eta0r_t_dn8: f64 = *var_eta0r_t_dn8_slot;
        let mut var_eta0r_t_dn9: f64 = *var_eta0r_t_dn9_slot;
        let mut var_eta0r_t_rv: f64 = *var_eta0r_t_rv_slot;
        let mut var_eta_mu: f64 = *var_eta_mu_slot;
        let mut var_eta_mu_rv: f64 = *var_eta_mu_rv_slot;
        let mut var_eu_t: f64 = *var_eu_t_slot;
        let mut var_eu_t_dn10: f64 = *var_eu_t_dn10_slot;
        let mut var_eu_t_dn11: f64 = *var_eu_t_dn11_slot;
        let mut var_eu_t_dn3: f64 = *var_eu_t_dn3_slot;
        let mut var_eu_t_dn4: f64 = *var_eu_t_dn4_slot;
        let mut var_eu_t_dn5: f64 = *var_eu_t_dn5_slot;
        let mut var_eu_t_dn6: f64 = *var_eu_t_dn6_slot;
        let mut var_eu_t_dn7: f64 = *var_eu_t_dn7_slot;
        let mut var_eu_t_dn8: f64 = *var_eu_t_dn8_slot;
        let mut var_eu_t_dn9: f64 = *var_eu_t_dn9_slot;
        let mut var_eu_t_rv: f64 = *var_eu_t_rv_slot;
        let mut var_guard436: f64 = *var_guard436_slot;
        let mut var_guard436_rv: f64 = *var_guard436_rv_slot;
        let mut var_guard437: f64 = *var_guard437_slot;
        let mut var_guard437_rv: f64 = *var_guard437_rv_slot;
        let mut var_guard438: f64 = *var_guard438_slot;
        let mut var_guard438_rv: f64 = *var_guard438_rv_slot;
        let mut var_guard449: f64 = *var_guard449_slot;
        let mut var_guard449_rv: f64 = *var_guard449_rv_slot;
        let mut var_guard450: f64 = *var_guard450_slot;
        let mut var_guard450_rv: f64 = *var_guard450_rv_slot;
        let mut var_litl: f64 = *var_litl_slot;
        let mut var_litl_rv: f64 = *var_litl_rv_slot;
        let mut var_nfactor_t: f64 = *var_nfactor_t_slot;
        let mut var_nfactor_t_dn10: f64 = *var_nfactor_t_dn10_slot;
        let mut var_nfactor_t_dn11: f64 = *var_nfactor_t_dn11_slot;
        let mut var_nfactor_t_dn3: f64 = *var_nfactor_t_dn3_slot;
        let mut var_nfactor_t_dn4: f64 = *var_nfactor_t_dn4_slot;
        let mut var_nfactor_t_dn5: f64 = *var_nfactor_t_dn5_slot;
        let mut var_nfactor_t_dn6: f64 = *var_nfactor_t_dn6_slot;
        let mut var_nfactor_t_dn7: f64 = *var_nfactor_t_dn7_slot;
        let mut var_nfactor_t_dn8: f64 = *var_nfactor_t_dn8_slot;
        let mut var_nfactor_t_dn9: f64 = *var_nfactor_t_dn9_slot;
        let mut var_nfactor_t_rv: f64 = *var_nfactor_t_rv_slot;
        let mut var_ni: f64 = *var_ni_slot;
        let mut var_ni_dn10: f64 = *var_ni_dn10_slot;
        let mut var_ni_dn11: f64 = *var_ni_dn11_slot;
        let mut var_ni_dn3: f64 = *var_ni_dn3_slot;
        let mut var_ni_dn4: f64 = *var_ni_dn4_slot;
        let mut var_ni_dn5: f64 = *var_ni_dn5_slot;
        let mut var_ni_dn6: f64 = *var_ni_dn6_slot;
        let mut var_ni_dn7: f64 = *var_ni_dn7_slot;
        let mut var_ni_dn8: f64 = *var_ni_dn8_slot;
        let mut var_ni_dn9: f64 = *var_ni_dn9_slot;
        let mut var_ni_rv: f64 = *var_ni_rv_slot;
        let mut var_phib: f64 = *var_phib_slot;
        let mut var_phib_dn10: f64 = *var_phib_dn10_slot;
        let mut var_phib_dn11: f64 = *var_phib_dn11_slot;
        let mut var_phib_dn3: f64 = *var_phib_dn3_slot;
        let mut var_phib_dn4: f64 = *var_phib_dn4_slot;
        let mut var_phib_dn5: f64 = *var_phib_dn5_slot;
        let mut var_phib_dn6: f64 = *var_phib_dn6_slot;
        let mut var_phib_dn7: f64 = *var_phib_dn7_slot;
        let mut var_phib_dn8: f64 = *var_phib_dn8_slot;
        let mut var_phib_dn9: f64 = *var_phib_dn9_slot;
        let mut var_phib_rv: f64 = *var_phib_rv_slot;
        let mut var_phist: f64 = *var_phist_slot;
        let mut var_phist_dn10: f64 = *var_phist_dn10_slot;
        let mut var_phist_dn11: f64 = *var_phist_dn11_slot;
        let mut var_phist_dn3: f64 = *var_phist_dn3_slot;
        let mut var_phist_dn4: f64 = *var_phist_dn4_slot;
        let mut var_phist_dn5: f64 = *var_phist_dn5_slot;
        let mut var_phist_dn6: f64 = *var_phist_dn6_slot;
        let mut var_phist_dn7: f64 = *var_phist_dn7_slot;
        let mut var_phist_dn8: f64 = *var_phist_dn8_slot;
        let mut var_phist_dn9: f64 = *var_phist_dn9_slot;
        let mut var_phist_rv: f64 = *var_phist_rv_slot;
        let mut var_sqrtphist: f64 = *var_sqrtphist_slot;
        let mut var_sqrtphist_dn10: f64 = *var_sqrtphist_dn10_slot;
        let mut var_sqrtphist_dn11: f64 = *var_sqrtphist_dn11_slot;
        let mut var_sqrtphist_dn3: f64 = *var_sqrtphist_dn3_slot;
        let mut var_sqrtphist_dn4: f64 = *var_sqrtphist_dn4_slot;
        let mut var_sqrtphist_dn5: f64 = *var_sqrtphist_dn5_slot;
        let mut var_sqrtphist_dn6: f64 = *var_sqrtphist_dn6_slot;
        let mut var_sqrtphist_dn7: f64 = *var_sqrtphist_dn7_slot;
        let mut var_sqrtphist_dn8: f64 = *var_sqrtphist_dn8_slot;
        let mut var_sqrtphist_dn9: f64 = *var_sqrtphist_dn9_slot;
        let mut var_sqrtphist_rv: f64 = *var_sqrtphist_rv_slot;
        let mut var_t0: f64 = *var_t0_slot;
        let mut var_t0_dn10: f64 = *var_t0_dn10_slot;
        let mut var_t0_dn11: f64 = *var_t0_dn11_slot;
        let mut var_t0_dn3: f64 = *var_t0_dn3_slot;
        let mut var_t0_dn4: f64 = *var_t0_dn4_slot;
        let mut var_t0_dn5: f64 = *var_t0_dn5_slot;
        let mut var_t0_dn6: f64 = *var_t0_dn6_slot;
        let mut var_t0_dn7: f64 = *var_t0_dn7_slot;
        let mut var_t0_dn8: f64 = *var_t0_dn8_slot;
        let mut var_t0_dn9: f64 = *var_t0_dn9_slot;
        let mut var_t0_rv: f64 = *var_t0_rv_slot;
        let mut var_t1: f64 = *var_t1_slot;
        let mut var_t1_dn10: f64 = *var_t1_dn10_slot;
        let mut var_t1_dn11: f64 = *var_t1_dn11_slot;
        let mut var_t1_dn3: f64 = *var_t1_dn3_slot;
        let mut var_t1_dn4: f64 = *var_t1_dn4_slot;
        let mut var_t1_dn5: f64 = *var_t1_dn5_slot;
        let mut var_t1_dn6: f64 = *var_t1_dn6_slot;
        let mut var_t1_dn7: f64 = *var_t1_dn7_slot;
        let mut var_t1_dn8: f64 = *var_t1_dn8_slot;
        let mut var_t1_dn9: f64 = *var_t1_dn9_slot;
        let mut var_t1_rv: f64 = *var_t1_rv_slot;
        let mut var_t1dep: f64 = *var_t1dep_slot;
        let mut var_t1dep_dn10: f64 = *var_t1dep_dn10_slot;
        let mut var_t1dep_dn11: f64 = *var_t1dep_dn11_slot;
        let mut var_t1dep_dn3: f64 = *var_t1dep_dn3_slot;
        let mut var_t1dep_dn4: f64 = *var_t1dep_dn4_slot;
        let mut var_t1dep_dn5: f64 = *var_t1dep_dn5_slot;
        let mut var_t1dep_dn6: f64 = *var_t1dep_dn6_slot;
        let mut var_t1dep_dn7: f64 = *var_t1dep_dn7_slot;
        let mut var_t1dep_dn8: f64 = *var_t1dep_dn8_slot;
        let mut var_t1dep_dn9: f64 = *var_t1dep_dn9_slot;
        let mut var_t1dep_rv: f64 = *var_t1dep_rv_slot;
        let mut var_u0_t: f64 = *var_u0_t_slot;
        let mut var_u0_t_dn10: f64 = *var_u0_t_dn10_slot;
        let mut var_u0_t_dn11: f64 = *var_u0_t_dn11_slot;
        let mut var_u0_t_dn3: f64 = *var_u0_t_dn3_slot;
        let mut var_u0_t_dn4: f64 = *var_u0_t_dn4_slot;
        let mut var_u0_t_dn5: f64 = *var_u0_t_dn5_slot;
        let mut var_u0_t_dn6: f64 = *var_u0_t_dn6_slot;
        let mut var_u0_t_dn7: f64 = *var_u0_t_dn7_slot;
        let mut var_u0_t_dn8: f64 = *var_u0_t_dn8_slot;
        let mut var_u0_t_dn9: f64 = *var_u0_t_dn9_slot;
        let mut var_u0_t_rv: f64 = *var_u0_t_rv_slot;
        let mut var_u0r_t: f64 = *var_u0r_t_slot;
        let mut var_u0r_t_dn4: f64 = *var_u0r_t_dn4_slot;
        let mut var_u0r_t_dn5: f64 = *var_u0r_t_dn5_slot;
        let mut var_u0r_t_rv: f64 = *var_u0r_t_rv_slot;
        let mut var_ua_t: f64 = *var_ua_t_slot;
        let mut var_ua_t_dn10: f64 = *var_ua_t_dn10_slot;
        let mut var_ua_t_dn11: f64 = *var_ua_t_dn11_slot;
        let mut var_ua_t_dn3: f64 = *var_ua_t_dn3_slot;
        let mut var_ua_t_dn4: f64 = *var_ua_t_dn4_slot;
        let mut var_ua_t_dn5: f64 = *var_ua_t_dn5_slot;
        let mut var_ua_t_dn6: f64 = *var_ua_t_dn6_slot;
        let mut var_ua_t_dn7: f64 = *var_ua_t_dn7_slot;
        let mut var_ua_t_dn8: f64 = *var_ua_t_dn8_slot;
        let mut var_ua_t_dn9: f64 = *var_ua_t_dn9_slot;
        let mut var_ua_t_rv: f64 = *var_ua_t_rv_slot;
        let mut var_uc_t: f64 = *var_uc_t_slot;
        let mut var_uc_t_dn10: f64 = *var_uc_t_dn10_slot;
        let mut var_uc_t_dn11: f64 = *var_uc_t_dn11_slot;
        let mut var_uc_t_dn3: f64 = *var_uc_t_dn3_slot;
        let mut var_uc_t_dn4: f64 = *var_uc_t_dn4_slot;
        let mut var_uc_t_dn5: f64 = *var_uc_t_dn5_slot;
        let mut var_uc_t_dn6: f64 = *var_uc_t_dn6_slot;
        let mut var_uc_t_dn7: f64 = *var_uc_t_dn7_slot;
        let mut var_uc_t_dn8: f64 = *var_uc_t_dn8_slot;
        let mut var_uc_t_dn9: f64 = *var_uc_t_dn9_slot;
        let mut var_uc_t_rv: f64 = *var_uc_t_rv_slot;
        let mut var_ucs_t: f64 = *var_ucs_t_slot;
        let mut var_ucs_t_dn4: f64 = *var_ucs_t_dn4_slot;
        let mut var_ucs_t_dn5: f64 = *var_ucs_t_dn5_slot;
        let mut var_ucs_t_rv: f64 = *var_ucs_t_rv_slot;
        let mut var_ud_t: f64 = *var_ud_t_slot;
        let mut var_ud_t_dn10: f64 = *var_ud_t_dn10_slot;
        let mut var_ud_t_dn11: f64 = *var_ud_t_dn11_slot;
        let mut var_ud_t_dn3: f64 = *var_ud_t_dn3_slot;
        let mut var_ud_t_dn4: f64 = *var_ud_t_dn4_slot;
        let mut var_ud_t_dn5: f64 = *var_ud_t_dn5_slot;
        let mut var_ud_t_dn6: f64 = *var_ud_t_dn6_slot;
        let mut var_ud_t_dn7: f64 = *var_ud_t_dn7_slot;
        let mut var_ud_t_dn8: f64 = *var_ud_t_dn8_slot;
        let mut var_ud_t_dn9: f64 = *var_ud_t_dn9_slot;
        let mut var_ud_t_rv: f64 = *var_ud_t_rv_slot;
        let mut var_vbi_edge: f64 = *var_vbi_edge_slot;
        let mut var_vbi_edge_dn10: f64 = *var_vbi_edge_dn10_slot;
        let mut var_vbi_edge_dn11: f64 = *var_vbi_edge_dn11_slot;
        let mut var_vbi_edge_dn3: f64 = *var_vbi_edge_dn3_slot;
        let mut var_vbi_edge_dn4: f64 = *var_vbi_edge_dn4_slot;
        let mut var_vbi_edge_dn5: f64 = *var_vbi_edge_dn5_slot;
        let mut var_vbi_edge_dn6: f64 = *var_vbi_edge_dn6_slot;
        let mut var_vbi_edge_dn7: f64 = *var_vbi_edge_dn7_slot;
        let mut var_vbi_edge_dn8: f64 = *var_vbi_edge_dn8_slot;
        let mut var_vbi_edge_dn9: f64 = *var_vbi_edge_dn9_slot;
        let mut var_vbi_edge_rv: f64 = *var_vbi_edge_rv_slot;
        let mut var_vfbsdr: f64 = *var_vfbsdr_slot;
        let mut var_vfbsdr_dn4: f64 = *var_vfbsdr_dn4_slot;
        let mut var_vfbsdr_dn5: f64 = *var_vfbsdr_dn5_slot;
        let mut var_vfbsdr_rv: f64 = *var_vfbsdr_rv_slot;

        let __rspice_inv_cse_0: f64 = 1.0 / var_tnom;
        let assign13430_e18594: f64 = (var_devtemp * __rspice_inv_cse_0);
        let assign13430_e18597: f64 = (var_devtemp * __rspice_inv_cse_0);
        let assign13430_e18598: f64 = (assign13430_e18597).sqrt();
        let assign13430_e18599: f64 = (assign13430_e18594 * assign13430_e18598);
        var_t1 = assign13430_e18599;
        var_t1_dn3 = 0.0;
        var_t1_dn4 = (((var_devtemp_dn4 / var_tnom) * assign13430_e18598) + (assign13430_e18594 * ((var_devtemp_dn4 / var_tnom) / (2.0 * assign13430_e18598))));
        var_t1_dn5 = (((var_devtemp_dn5 / var_tnom) * assign13430_e18598) + (assign13430_e18594 * ((var_devtemp_dn5 / var_tnom) / (2.0 * assign13430_e18598))));
        var_t1_dn6 = 0.0;
        var_t1_dn7 = 0.0;
        var_t1_dn8 = 0.0;
        var_t1_dn9 = 0.0;
        var_t1_dn10 = 0.0;
        var_t1_dn11 = 0.0;
        var_t1_rv = 0.0;

        let assign13440_e18602: f64 = (p.p107 * var_t1);
        let assign13440_e18606: f64 = (2.0 * var_vtm0);
        let assign13440_e18607: f64 = (var_eg / assign13440_e18606);
        let assign13440_e18611: f64 = (2.0 * var_vtm);
        let assign13440_e18612: f64 = (var_eg / assign13440_e18611);
        let assign13440_e18613: f64 = (assign13440_e18607 - assign13440_e18612);
        let assign13440_e18614: f64 = { let limited_exp_arg = assign13440_e18613; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign13440_e18615: f64 = (assign13440_e18602 * assign13440_e18614);
        var_ni = assign13440_e18615;
        var_ni_dn3 = ((p.p107 * var_t1_dn3) * assign13440_e18614);
        var_ni_dn4 = (((p.p107 * var_t1_dn4) * assign13440_e18614) + (assign13440_e18602 * ({ let limited_exp_arg = assign13440_e18613; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((var_eg_dn4 / assign13440_e18606) - (((var_eg_dn4 * assign13440_e18611) - (var_eg * (2.0 * var_vtm_dn4))) / (assign13440_e18611 * assign13440_e18611))))));
        var_ni_dn5 = (((p.p107 * var_t1_dn5) * assign13440_e18614) + (assign13440_e18602 * ({ let limited_exp_arg = assign13440_e18613; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((var_eg_dn5 / assign13440_e18606) - (((var_eg_dn5 * assign13440_e18611) - (var_eg * (2.0 * var_vtm_dn5))) / (assign13440_e18611 * assign13440_e18611))))));
        var_ni_dn6 = ((p.p107 * var_t1_dn6) * assign13440_e18614);
        var_ni_dn7 = ((p.p107 * var_t1_dn7) * assign13440_e18614);
        var_ni_dn8 = ((p.p107 * var_t1_dn8) * assign13440_e18614);
        var_ni_dn9 = ((p.p107 * var_t1_dn9) * assign13440_e18614);
        var_ni_dn10 = ((p.p107 * var_t1_dn10) * assign13440_e18614);
        var_ni_dn11 = ((p.p107 * var_t1_dn11) * assign13440_e18614);
        var_ni_rv = 0.0;

        let assign13450_e18626: f64 = if (((p.p41 != 0.0) && (p.p1099 > 0.0)) && (var_weff_sh > 0.0)) { 1.0 } else { 0.0 };
        var_guard436 = assign13450_e18626;
        var_guard436_rv = 0.0;

        let (assign13460_e18635, assign13460_e18635_d_n3, assign13460_e18635_d_n4, assign13460_e18635_d_n5, assign13460_e18635_d_n6, assign13460_e18635_d_n7, assign13460_e18635_d_n8, assign13460_e18635_d_n9, assign13460_e18635_d_n10, assign13460_e18635_d_n11,) = {
    if (var_guard436 != 0.0) {
        let assign13460_e18630: f64 = (var_ndep_i / var_ni);
        let assign13460_e18632: f64 = (assign13460_e18630).max(1e-38);
        let assign13460_e18633: f64 = (assign13460_e18632).ln();
        (assign13460_e18633, (if assign13460_e18630 >= 1e-38 { (((var_ndep_i_dn3 * var_ni) - (var_ndep_i * var_ni_dn3)) / (var_ni * var_ni)) } else { 0.0 } / assign13460_e18632), (if assign13460_e18630 >= 1e-38 { (((var_ndep_i_dn4 * var_ni) - (var_ndep_i * var_ni_dn4)) / (var_ni * var_ni)) } else { 0.0 } / assign13460_e18632), (if assign13460_e18630 >= 1e-38 { (((var_ndep_i_dn5 * var_ni) - (var_ndep_i * var_ni_dn5)) / (var_ni * var_ni)) } else { 0.0 } / assign13460_e18632), (if assign13460_e18630 >= 1e-38 { (((var_ndep_i_dn6 * var_ni) - (var_ndep_i * var_ni_dn6)) / (var_ni * var_ni)) } else { 0.0 } / assign13460_e18632), (if assign13460_e18630 >= 1e-38 { (((var_ndep_i_dn7 * var_ni) - (var_ndep_i * var_ni_dn7)) / (var_ni * var_ni)) } else { 0.0 } / assign13460_e18632), (if assign13460_e18630 >= 1e-38 { (((var_ndep_i_dn8 * var_ni) - (var_ndep_i * var_ni_dn8)) / (var_ni * var_ni)) } else { 0.0 } / assign13460_e18632), (if assign13460_e18630 >= 1e-38 { (((var_ndep_i_dn9 * var_ni) - (var_ndep_i * var_ni_dn9)) / (var_ni * var_ni)) } else { 0.0 } / assign13460_e18632), (if assign13460_e18630 >= 1e-38 { (((var_ndep_i_dn10 * var_ni) - (var_ndep_i * var_ni_dn10)) / (var_ni * var_ni)) } else { 0.0 } / assign13460_e18632), (if assign13460_e18630 >= 1e-38 { (((var_ndep_i_dn11 * var_ni) - (var_ndep_i * var_ni_dn11)) / (var_ni * var_ni)) } else { 0.0 } / assign13460_e18632),)
    } else {
        (var_t0, var_t0_dn3, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8, var_t0_dn9, var_t0_dn10, var_t0_dn11,)
    }
};
        var_t0 = assign13460_e18635;
        var_t0_dn3 = assign13460_e18635_d_n3;
        var_t0_dn4 = assign13460_e18635_d_n4;
        var_t0_dn5 = assign13460_e18635_d_n5;
        var_t0_dn6 = assign13460_e18635_d_n6;
        var_t0_dn7 = assign13460_e18635_d_n7;
        var_t0_dn8 = assign13460_e18635_d_n8;
        var_t0_dn9 = assign13460_e18635_d_n9;
        var_t0_dn10 = assign13460_e18635_d_n10;
        var_t0_dn11 = assign13460_e18635_d_n11;
        var_t0_rv = 0.0;

        let (assign13470_e18644, assign13470_e18644_d_n3, assign13470_e18644_d_n4, assign13470_e18644_d_n5, assign13470_e18644_d_n6, assign13470_e18644_d_n7, assign13470_e18644_d_n8, assign13470_e18644_d_n9, assign13470_e18644_d_n10, assign13470_e18644_d_n11,) = {
    if (var_guard436 != 0.0) {
        let assign13470_e18639: f64 = (var_t0 * var_t0);
        let assign13470_e18641: f64 = (assign13470_e18639 + 1e-6);
        let assign13470_e18642: f64 = (assign13470_e18641).sqrt();
        (assign13470_e18642, (((var_t0_dn3 * var_t0) + (var_t0 * var_t0_dn3)) / (2.0 * assign13470_e18642)), (((var_t0_dn4 * var_t0) + (var_t0 * var_t0_dn4)) / (2.0 * assign13470_e18642)), (((var_t0_dn5 * var_t0) + (var_t0 * var_t0_dn5)) / (2.0 * assign13470_e18642)), (((var_t0_dn6 * var_t0) + (var_t0 * var_t0_dn6)) / (2.0 * assign13470_e18642)), (((var_t0_dn7 * var_t0) + (var_t0 * var_t0_dn7)) / (2.0 * assign13470_e18642)), (((var_t0_dn8 * var_t0) + (var_t0 * var_t0_dn8)) / (2.0 * assign13470_e18642)), (((var_t0_dn9 * var_t0) + (var_t0 * var_t0_dn9)) / (2.0 * assign13470_e18642)), (((var_t0_dn10 * var_t0) + (var_t0 * var_t0_dn10)) / (2.0 * assign13470_e18642)), (((var_t0_dn11 * var_t0) + (var_t0 * var_t0_dn11)) / (2.0 * assign13470_e18642)),)
    } else {
        (var_phib, var_phib_dn3, var_phib_dn4, var_phib_dn5, var_phib_dn6, var_phib_dn7, var_phib_dn8, var_phib_dn9, var_phib_dn10, var_phib_dn11,)
    }
};
        var_phib = assign13470_e18644;
        var_phib_dn3 = assign13470_e18644_d_n3;
        var_phib_dn4 = assign13470_e18644_d_n4;
        var_phib_dn5 = assign13470_e18644_d_n5;
        var_phib_dn6 = assign13470_e18644_d_n6;
        var_phib_dn7 = assign13470_e18644_d_n7;
        var_phib_dn8 = assign13470_e18644_d_n8;
        var_phib_dn9 = assign13470_e18644_d_n9;
        var_phib_dn10 = assign13470_e18644_d_n10;
        var_phib_dn11 = assign13470_e18644_d_n11;
        var_phib_rv = 0.0;

        let (assign13480_e18654, assign13480_e18654_d_n3, assign13480_e18654_d_n4, assign13480_e18654_d_n5, assign13480_e18654_d_n6, assign13480_e18654_d_n7, assign13480_e18654_d_n8, assign13480_e18654_d_n9, assign13480_e18654_d_n10, assign13480_e18654_d_n11,) = {
    if (var_guard436 == 0.0) {
        let assign13480_e18649: f64 = (var_ndep_i / var_ni);
        let assign13480_e18651: f64 = (assign13480_e18649).max(1e-38);
        let assign13480_e18652: f64 = (assign13480_e18651).ln();
        (assign13480_e18652, (if assign13480_e18649 >= 1e-38 { (((var_ndep_i_dn3 * var_ni) - (var_ndep_i * var_ni_dn3)) / (var_ni * var_ni)) } else { 0.0 } / assign13480_e18651), (if assign13480_e18649 >= 1e-38 { (((var_ndep_i_dn4 * var_ni) - (var_ndep_i * var_ni_dn4)) / (var_ni * var_ni)) } else { 0.0 } / assign13480_e18651), (if assign13480_e18649 >= 1e-38 { (((var_ndep_i_dn5 * var_ni) - (var_ndep_i * var_ni_dn5)) / (var_ni * var_ni)) } else { 0.0 } / assign13480_e18651), (if assign13480_e18649 >= 1e-38 { (((var_ndep_i_dn6 * var_ni) - (var_ndep_i * var_ni_dn6)) / (var_ni * var_ni)) } else { 0.0 } / assign13480_e18651), (if assign13480_e18649 >= 1e-38 { (((var_ndep_i_dn7 * var_ni) - (var_ndep_i * var_ni_dn7)) / (var_ni * var_ni)) } else { 0.0 } / assign13480_e18651), (if assign13480_e18649 >= 1e-38 { (((var_ndep_i_dn8 * var_ni) - (var_ndep_i * var_ni_dn8)) / (var_ni * var_ni)) } else { 0.0 } / assign13480_e18651), (if assign13480_e18649 >= 1e-38 { (((var_ndep_i_dn9 * var_ni) - (var_ndep_i * var_ni_dn9)) / (var_ni * var_ni)) } else { 0.0 } / assign13480_e18651), (if assign13480_e18649 >= 1e-38 { (((var_ndep_i_dn10 * var_ni) - (var_ndep_i * var_ni_dn10)) / (var_ni * var_ni)) } else { 0.0 } / assign13480_e18651), (if assign13480_e18649 >= 1e-38 { (((var_ndep_i_dn11 * var_ni) - (var_ndep_i * var_ni_dn11)) / (var_ni * var_ni)) } else { 0.0 } / assign13480_e18651),)
    } else {
        (var_phib, var_phib_dn3, var_phib_dn4, var_phib_dn5, var_phib_dn6, var_phib_dn7, var_phib_dn8, var_phib_dn9, var_phib_dn10, var_phib_dn11,)
    }
};
        var_phib = assign13480_e18654;
        var_phib_dn3 = assign13480_e18654_d_n3;
        var_phib_dn4 = assign13480_e18654_d_n4;
        var_phib_dn5 = assign13480_e18654_d_n5;
        var_phib_dn6 = assign13480_e18654_d_n6;
        var_phib_dn7 = assign13480_e18654_d_n7;
        var_phib_dn8 = assign13480_e18654_d_n8;
        var_phib_dn9 = assign13480_e18654_d_n9;
        var_phib_dn10 = assign13480_e18654_d_n10;
        var_phib_dn11 = assign13480_e18654_d_n11;
        var_phib_rv = 0.0;

        let assign13490_e18665: f64 = if (((p.p41 != 0.0) && (p.p1099 > 0.0)) && (var_weff_sh > 0.0)) { 1.0 } else { 0.0 };
        var_guard437 = assign13490_e18665;
        var_guard437_rv = 0.0;

        let (assign13500_e18678, assign13500_e18678_d_n3, assign13500_e18678_d_n4, assign13500_e18678_d_n5, assign13500_e18678_d_n6, assign13500_e18678_d_n7, assign13500_e18678_d_n8, assign13500_e18678_d_n9, assign13500_e18678_d_n10, assign13500_e18678_d_n11,) = {
    if (var_guard437 != 0.0) {
        let assign13500_e18669: f64 = (var_ndepedge_i * var_nsd_i);
        let assign13500_e18672: f64 = (var_ni * var_ni);
        let assign13500_e18673: f64 = (assign13500_e18669 / assign13500_e18672);
        let assign13500_e18675: f64 = (assign13500_e18673).max(1e-38);
        let assign13500_e18676: f64 = (assign13500_e18675).ln();
        (assign13500_e18676, (if assign13500_e18673 >= 1e-38 { (-((assign13500_e18669 * ((var_ni_dn3 * var_ni) + (var_ni * var_ni_dn3))) / (assign13500_e18672 * assign13500_e18672))) } else { 0.0 } / assign13500_e18675), (if assign13500_e18673 >= 1e-38 { (-((assign13500_e18669 * ((var_ni_dn4 * var_ni) + (var_ni * var_ni_dn4))) / (assign13500_e18672 * assign13500_e18672))) } else { 0.0 } / assign13500_e18675), (if assign13500_e18673 >= 1e-38 { (-((assign13500_e18669 * ((var_ni_dn5 * var_ni) + (var_ni * var_ni_dn5))) / (assign13500_e18672 * assign13500_e18672))) } else { 0.0 } / assign13500_e18675), (if assign13500_e18673 >= 1e-38 { (-((assign13500_e18669 * ((var_ni_dn6 * var_ni) + (var_ni * var_ni_dn6))) / (assign13500_e18672 * assign13500_e18672))) } else { 0.0 } / assign13500_e18675), (if assign13500_e18673 >= 1e-38 { (-((assign13500_e18669 * ((var_ni_dn7 * var_ni) + (var_ni * var_ni_dn7))) / (assign13500_e18672 * assign13500_e18672))) } else { 0.0 } / assign13500_e18675), (if assign13500_e18673 >= 1e-38 { (-((assign13500_e18669 * ((var_ni_dn8 * var_ni) + (var_ni * var_ni_dn8))) / (assign13500_e18672 * assign13500_e18672))) } else { 0.0 } / assign13500_e18675), (if assign13500_e18673 >= 1e-38 { (-((assign13500_e18669 * ((var_ni_dn9 * var_ni) + (var_ni * var_ni_dn9))) / (assign13500_e18672 * assign13500_e18672))) } else { 0.0 } / assign13500_e18675), (if assign13500_e18673 >= 1e-38 { (-((assign13500_e18669 * ((var_ni_dn10 * var_ni) + (var_ni * var_ni_dn10))) / (assign13500_e18672 * assign13500_e18672))) } else { 0.0 } / assign13500_e18675), (if assign13500_e18673 >= 1e-38 { (-((assign13500_e18669 * ((var_ni_dn11 * var_ni) + (var_ni * var_ni_dn11))) / (assign13500_e18672 * assign13500_e18672))) } else { 0.0 } / assign13500_e18675),)
    } else {
        (var_t0, var_t0_dn3, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8, var_t0_dn9, var_t0_dn10, var_t0_dn11,)
    }
};
        var_t0 = assign13500_e18678;
        var_t0_dn3 = assign13500_e18678_d_n3;
        var_t0_dn4 = assign13500_e18678_d_n4;
        var_t0_dn5 = assign13500_e18678_d_n5;
        var_t0_dn6 = assign13500_e18678_d_n6;
        var_t0_dn7 = assign13500_e18678_d_n7;
        var_t0_dn8 = assign13500_e18678_d_n8;
        var_t0_dn9 = assign13500_e18678_d_n9;
        var_t0_dn10 = assign13500_e18678_d_n10;
        var_t0_dn11 = assign13500_e18678_d_n11;
        var_t0_rv = 0.0;

        let (assign13510_e18687, assign13510_e18687_d_n3, assign13510_e18687_d_n4, assign13510_e18687_d_n5, assign13510_e18687_d_n6, assign13510_e18687_d_n7, assign13510_e18687_d_n8, assign13510_e18687_d_n9, assign13510_e18687_d_n10, assign13510_e18687_d_n11,) = {
    if (var_guard437 != 0.0) {
        let assign13510_e18682: f64 = (var_t0 * var_t0);
        let assign13510_e18684: f64 = (assign13510_e18682 + 1e-6);
        let assign13510_e18685: f64 = (assign13510_e18684).sqrt();
        (assign13510_e18685, (((var_t0_dn3 * var_t0) + (var_t0 * var_t0_dn3)) / (2.0 * assign13510_e18685)), (((var_t0_dn4 * var_t0) + (var_t0 * var_t0_dn4)) / (2.0 * assign13510_e18685)), (((var_t0_dn5 * var_t0) + (var_t0 * var_t0_dn5)) / (2.0 * assign13510_e18685)), (((var_t0_dn6 * var_t0) + (var_t0 * var_t0_dn6)) / (2.0 * assign13510_e18685)), (((var_t0_dn7 * var_t0) + (var_t0 * var_t0_dn7)) / (2.0 * assign13510_e18685)), (((var_t0_dn8 * var_t0) + (var_t0 * var_t0_dn8)) / (2.0 * assign13510_e18685)), (((var_t0_dn9 * var_t0) + (var_t0 * var_t0_dn9)) / (2.0 * assign13510_e18685)), (((var_t0_dn10 * var_t0) + (var_t0 * var_t0_dn10)) / (2.0 * assign13510_e18685)), (((var_t0_dn11 * var_t0) + (var_t0 * var_t0_dn11)) / (2.0 * assign13510_e18685)),)
    } else {
        (var_vbi_edge, var_vbi_edge_dn3, var_vbi_edge_dn4, var_vbi_edge_dn5, var_vbi_edge_dn6, var_vbi_edge_dn7, var_vbi_edge_dn8, var_vbi_edge_dn9, var_vbi_edge_dn10, var_vbi_edge_dn11,)
    }
};
        var_vbi_edge = assign13510_e18687;
        var_vbi_edge_dn3 = assign13510_e18687_d_n3;
        var_vbi_edge_dn4 = assign13510_e18687_d_n4;
        var_vbi_edge_dn5 = assign13510_e18687_d_n5;
        var_vbi_edge_dn6 = assign13510_e18687_d_n6;
        var_vbi_edge_dn7 = assign13510_e18687_d_n7;
        var_vbi_edge_dn8 = assign13510_e18687_d_n8;
        var_vbi_edge_dn9 = assign13510_e18687_d_n9;
        var_vbi_edge_dn10 = assign13510_e18687_d_n10;
        var_vbi_edge_dn11 = assign13510_e18687_d_n11;
        var_vbi_edge_rv = 0.0;

        let (assign13520_e18701, assign13520_e18701_d_n3, assign13520_e18701_d_n4, assign13520_e18701_d_n5, assign13520_e18701_d_n6, assign13520_e18701_d_n7, assign13520_e18701_d_n8, assign13520_e18701_d_n9, assign13520_e18701_d_n10, assign13520_e18701_d_n11,) = {
    if (var_guard437 == 0.0) {
        let assign13520_e18692: f64 = (var_ndepedge_i * var_nsd_i);
        let assign13520_e18695: f64 = (var_ni * var_ni);
        let assign13520_e18696: f64 = (assign13520_e18692 / assign13520_e18695);
        let assign13520_e18698: f64 = (assign13520_e18696).max(1e-38);
        let assign13520_e18699: f64 = (assign13520_e18698).ln();
        (assign13520_e18699, (if assign13520_e18696 >= 1e-38 { (-((assign13520_e18692 * ((var_ni_dn3 * var_ni) + (var_ni * var_ni_dn3))) / (assign13520_e18695 * assign13520_e18695))) } else { 0.0 } / assign13520_e18698), (if assign13520_e18696 >= 1e-38 { (-((assign13520_e18692 * ((var_ni_dn4 * var_ni) + (var_ni * var_ni_dn4))) / (assign13520_e18695 * assign13520_e18695))) } else { 0.0 } / assign13520_e18698), (if assign13520_e18696 >= 1e-38 { (-((assign13520_e18692 * ((var_ni_dn5 * var_ni) + (var_ni * var_ni_dn5))) / (assign13520_e18695 * assign13520_e18695))) } else { 0.0 } / assign13520_e18698), (if assign13520_e18696 >= 1e-38 { (-((assign13520_e18692 * ((var_ni_dn6 * var_ni) + (var_ni * var_ni_dn6))) / (assign13520_e18695 * assign13520_e18695))) } else { 0.0 } / assign13520_e18698), (if assign13520_e18696 >= 1e-38 { (-((assign13520_e18692 * ((var_ni_dn7 * var_ni) + (var_ni * var_ni_dn7))) / (assign13520_e18695 * assign13520_e18695))) } else { 0.0 } / assign13520_e18698), (if assign13520_e18696 >= 1e-38 { (-((assign13520_e18692 * ((var_ni_dn8 * var_ni) + (var_ni * var_ni_dn8))) / (assign13520_e18695 * assign13520_e18695))) } else { 0.0 } / assign13520_e18698), (if assign13520_e18696 >= 1e-38 { (-((assign13520_e18692 * ((var_ni_dn9 * var_ni) + (var_ni * var_ni_dn9))) / (assign13520_e18695 * assign13520_e18695))) } else { 0.0 } / assign13520_e18698), (if assign13520_e18696 >= 1e-38 { (-((assign13520_e18692 * ((var_ni_dn10 * var_ni) + (var_ni * var_ni_dn10))) / (assign13520_e18695 * assign13520_e18695))) } else { 0.0 } / assign13520_e18698), (if assign13520_e18696 >= 1e-38 { (-((assign13520_e18692 * ((var_ni_dn11 * var_ni) + (var_ni * var_ni_dn11))) / (assign13520_e18695 * assign13520_e18695))) } else { 0.0 } / assign13520_e18698),)
    } else {
        (var_vbi_edge, var_vbi_edge_dn3, var_vbi_edge_dn4, var_vbi_edge_dn5, var_vbi_edge_dn6, var_vbi_edge_dn7, var_vbi_edge_dn8, var_vbi_edge_dn9, var_vbi_edge_dn10, var_vbi_edge_dn11,)
    }
};
        var_vbi_edge = assign13520_e18701;
        var_vbi_edge_dn3 = assign13520_e18701_d_n3;
        var_vbi_edge_dn4 = assign13520_e18701_d_n4;
        var_vbi_edge_dn5 = assign13520_e18701_d_n5;
        var_vbi_edge_dn6 = assign13520_e18701_d_n6;
        var_vbi_edge_dn7 = assign13520_e18701_d_n7;
        var_vbi_edge_dn8 = assign13520_e18701_d_n8;
        var_vbi_edge_dn9 = assign13520_e18701_d_n9;
        var_vbi_edge_dn10 = assign13520_e18701_d_n10;
        var_vbi_edge_dn11 = assign13520_e18701_d_n11;
        var_vbi_edge_rv = 0.0;

        let assign13530_e18704: f64 = if var_ngate_i > 0.0 { 1.0 } else { 0.0 };
        var_guard438 = assign13530_e18704;
        var_guard438_rv = 0.0;

        let (assign13540_e18720, assign13540_e18720_d_n4, assign13540_e18720_d_n5,) = {
    if (var_guard438 != 0.0) {
        let assign13540_e18707: f64 = (-var_devsign);
        let assign13540_e18709: f64 = (assign13540_e18707 * var_vt);
        let assign13540_e18712: f64 = (var_ngate_i / var_nsd_i);
        let assign13540_e18714: f64 = (assign13540_e18712).max(1e-38);
        let assign13540_e18715: f64 = (assign13540_e18714).ln();
        let assign13540_e18716: f64 = (assign13540_e18709 * assign13540_e18715);
        let assign13540_e18718: f64 = (assign13540_e18716 + p.p5);
        (assign13540_e18718, ((assign13540_e18707 * var_vt_dn4) * assign13540_e18715), ((assign13540_e18707 * var_vt_dn5) * assign13540_e18715),)
    } else {
        (var_vfbsdr, var_vfbsdr_dn4, var_vfbsdr_dn5,)
    }
};
        var_vfbsdr = assign13540_e18720;
        var_vfbsdr_dn4 = assign13540_e18720_d_n4;
        var_vfbsdr_dn5 = assign13540_e18720_d_n5;
        var_vfbsdr_rv = 0.0;

        let (assign13550_e18725, assign13550_e18725_d_n4, assign13550_e18725_d_n5,) = {
    if (var_guard438 == 0.0) {
        (0.0, 0.0, 0.0,)
    } else {
        (var_vfbsdr, var_vfbsdr_dn4, var_vfbsdr_dn5,)
    }
};
        var_vfbsdr = assign13550_e18725;
        var_vfbsdr_dn4 = assign13550_e18725_d_n4;
        var_vfbsdr_dn5 = assign13550_e18725_d_n5;
        var_vfbsdr_rv = 0.0;

        let assign13660_e18777: f64 = (var_vt * var_phib);
        let assign13660_e18778: f64 = (0.4 + assign13660_e18777);
        let assign13660_e18780: f64 = (assign13660_e18778 + var_phin_i);
        let assign13660_e18782: f64 = (assign13660_e18780).max(0.4);
        var_phist = assign13660_e18782;
        var_phist_dn3 = if assign13660_e18780 >= 0.4 { (var_vt * var_phib_dn3) } else { 0.0 };
        var_phist_dn4 = if assign13660_e18780 >= 0.4 { ((var_vt_dn4 * var_phib) + (var_vt * var_phib_dn4)) } else { 0.0 };
        var_phist_dn5 = if assign13660_e18780 >= 0.4 { ((var_vt_dn5 * var_phib) + (var_vt * var_phib_dn5)) } else { 0.0 };
        var_phist_dn6 = if assign13660_e18780 >= 0.4 { (var_vt * var_phib_dn6) } else { 0.0 };
        var_phist_dn7 = if assign13660_e18780 >= 0.4 { (var_vt * var_phib_dn7) } else { 0.0 };
        var_phist_dn8 = if assign13660_e18780 >= 0.4 { (var_vt * var_phib_dn8) } else { 0.0 };
        var_phist_dn9 = if assign13660_e18780 >= 0.4 { (var_vt * var_phib_dn9) } else { 0.0 };
        var_phist_dn10 = if assign13660_e18780 >= 0.4 { (var_vt * var_phib_dn10) } else { 0.0 };
        var_phist_dn11 = if assign13660_e18780 >= 0.4 { (var_vt * var_phib_dn11) } else { 0.0 };
        var_phist_rv = 0.0;

        let assign13670_e18784: f64 = (var_phist).sqrt();
        var_sqrtphist = assign13670_e18784;
        var_sqrtphist_dn3 = (var_phist_dn3 / (2.0 * assign13670_e18784));
        var_sqrtphist_dn4 = (var_phist_dn4 / (2.0 * assign13670_e18784));
        var_sqrtphist_dn5 = (var_phist_dn5 / (2.0 * assign13670_e18784));
        var_sqrtphist_dn6 = (var_phist_dn6 / (2.0 * assign13670_e18784));
        var_sqrtphist_dn7 = (var_phist_dn7 / (2.0 * assign13670_e18784));
        var_sqrtphist_dn8 = (var_phist_dn8 / (2.0 * assign13670_e18784));
        var_sqrtphist_dn9 = (var_phist_dn9 / (2.0 * assign13670_e18784));
        var_sqrtphist_dn10 = (var_phist_dn10 / (2.0 * assign13670_e18784));
        var_sqrtphist_dn11 = (var_phist_dn11 / (2.0 * assign13670_e18784));
        var_sqrtphist_rv = 0.0;

        let assign13680_e18787: f64 = (2.0 * var_epssi);
        let assign13680_e18790: f64 = (1.602176462e-19 * var_ndep_i);
        let assign13680_e18791: f64 = (assign13680_e18787 / assign13680_e18790);
        let assign13680_e18792: f64 = (assign13680_e18791).sqrt();
        var_t1dep = assign13680_e18792;
        var_t1dep_dn3 = ((-((assign13680_e18787 * (1.602176462e-19 * var_ndep_i_dn3)) / (assign13680_e18790 * assign13680_e18790))) / (2.0 * assign13680_e18792));
        var_t1dep_dn4 = ((-((assign13680_e18787 * (1.602176462e-19 * var_ndep_i_dn4)) / (assign13680_e18790 * assign13680_e18790))) / (2.0 * assign13680_e18792));
        var_t1dep_dn5 = ((-((assign13680_e18787 * (1.602176462e-19 * var_ndep_i_dn5)) / (assign13680_e18790 * assign13680_e18790))) / (2.0 * assign13680_e18792));
        var_t1dep_dn6 = ((-((assign13680_e18787 * (1.602176462e-19 * var_ndep_i_dn6)) / (assign13680_e18790 * assign13680_e18790))) / (2.0 * assign13680_e18792));
        var_t1dep_dn7 = ((-((assign13680_e18787 * (1.602176462e-19 * var_ndep_i_dn7)) / (assign13680_e18790 * assign13680_e18790))) / (2.0 * assign13680_e18792));
        var_t1dep_dn8 = ((-((assign13680_e18787 * (1.602176462e-19 * var_ndep_i_dn8)) / (assign13680_e18790 * assign13680_e18790))) / (2.0 * assign13680_e18792));
        var_t1dep_dn9 = ((-((assign13680_e18787 * (1.602176462e-19 * var_ndep_i_dn9)) / (assign13680_e18790 * assign13680_e18790))) / (2.0 * assign13680_e18792));
        var_t1dep_dn10 = ((-((assign13680_e18787 * (1.602176462e-19 * var_ndep_i_dn10)) / (assign13680_e18790 * assign13680_e18790))) / (2.0 * assign13680_e18792));
        var_t1dep_dn11 = ((-((assign13680_e18787 * (1.602176462e-19 * var_ndep_i_dn11)) / (assign13680_e18790 * assign13680_e18790))) / (2.0 * assign13680_e18792));
        var_t1dep_rv = 0.0;

        let assign13690_e18795: f64 = (var_epssi / var_epsox);
        let assign13690_e18797: f64 = (assign13690_e18795 * p.p76);
        let assign13690_e18799: f64 = (assign13690_e18797 * var_xj_i);
        let assign13690_e18800: f64 = (assign13690_e18799).sqrt();
        var_litl = assign13690_e18800;
        var_litl_rv = 0.0;

        let assign13700_e18807: f64 = (var_tratio - 1.0);
        let assign13700_e18808: f64 = (p.p1031 * assign13700_e18807);
        let assign13700_e18809: f64 = (1.0 + assign13700_e18808);
        let assign13700_e18814: f64 = (var_tratio - 1.0);
        let assign13700_e18815: f64 = (p.p1031 * assign13700_e18814);
        let assign13700_e18816: f64 = (1.0 + assign13700_e18815);
        let assign13700_e18821: f64 = (var_tratio - 1.0);
        let assign13700_e18822: f64 = (p.p1031 * assign13700_e18821);
        let assign13700_e18823: f64 = (1.0 + assign13700_e18822);
        let assign13700_e18824: f64 = (assign13700_e18816 * assign13700_e18823);
        let assign13700_e18827: f64 = (4.0 * 0.001);
        let assign13700_e18829: f64 = (assign13700_e18827 * 0.001);
        let assign13700_e18830: f64 = (assign13700_e18824 + assign13700_e18829);
        let assign13700_e18831: f64 = (assign13700_e18830).sqrt();
        let assign13700_e18832: f64 = (assign13700_e18809 + assign13700_e18831);
        let assign13700_e18833: f64 = (0.5 * assign13700_e18832);
        let assign13700_e18834: f64 = (var_nfactor_i * assign13700_e18833);
        var_nfactor_t = assign13700_e18834;
        var_nfactor_t_dn3 = (var_nfactor_i_dn3 * assign13700_e18833);
        var_nfactor_t_dn4 = ((var_nfactor_i_dn4 * assign13700_e18833) + (var_nfactor_i * (0.5 * ((p.p1031 * var_tratio_dn4) + ((((p.p1031 * var_tratio_dn4) * assign13700_e18823) + (assign13700_e18816 * (p.p1031 * var_tratio_dn4))) / (2.0 * assign13700_e18831))))));
        var_nfactor_t_dn5 = ((var_nfactor_i_dn5 * assign13700_e18833) + (var_nfactor_i * (0.5 * ((p.p1031 * var_tratio_dn5) + ((((p.p1031 * var_tratio_dn5) * assign13700_e18823) + (assign13700_e18816 * (p.p1031 * var_tratio_dn5))) / (2.0 * assign13700_e18831))))));
        var_nfactor_t_dn6 = (var_nfactor_i_dn6 * assign13700_e18833);
        var_nfactor_t_dn7 = (var_nfactor_i_dn7 * assign13700_e18833);
        var_nfactor_t_dn8 = (var_nfactor_i_dn8 * assign13700_e18833);
        var_nfactor_t_dn9 = (var_nfactor_i_dn9 * assign13700_e18833);
        var_nfactor_t_dn10 = (var_nfactor_i_dn10 * assign13700_e18833);
        var_nfactor_t_dn11 = (var_nfactor_i_dn11 * assign13700_e18833);
        var_nfactor_t_rv = 0.0;

        let assign13710_e18840: f64 = (var_tratio - 1.0);
        let assign13710_e18841: f64 = (p.p1059 * assign13710_e18840);
        let assign13710_e18842: f64 = (1.0 + assign13710_e18841);
        let assign13710_e18843: f64 = (var_eta0_i * assign13710_e18842);
        var_eta0_t = assign13710_e18843;
        var_eta0_t_dn3 = (var_eta0_i_dn3 * assign13710_e18842);
        var_eta0_t_dn4 = ((var_eta0_i_dn4 * assign13710_e18842) + (var_eta0_i * (p.p1059 * var_tratio_dn4)));
        var_eta0_t_dn5 = ((var_eta0_i_dn5 * assign13710_e18842) + (var_eta0_i * (p.p1059 * var_tratio_dn5)));
        var_eta0_t_dn6 = (var_eta0_i_dn6 * assign13710_e18842);
        var_eta0_t_dn7 = (var_eta0_i_dn7 * assign13710_e18842);
        var_eta0_t_dn8 = (var_eta0_i_dn8 * assign13710_e18842);
        var_eta0_t_dn9 = (var_eta0_i_dn9 * assign13710_e18842);
        var_eta0_t_dn10 = (var_eta0_i_dn10 * assign13710_e18842);
        var_eta0_t_dn11 = (var_eta0_i_dn11 * assign13710_e18842);
        var_eta0_t_rv = 0.0;

        let assign13720_e18846: f64 = if p.p35 != 0.0 { 1.0 } else { 0.0 };
        var_guard449 = assign13720_e18846;
        var_guard449_rv = 0.0;

        let (assign13730_e18858, assign13730_e18858_d_n3, assign13730_e18858_d_n4, assign13730_e18858_d_n5, assign13730_e18858_d_n6, assign13730_e18858_d_n7, assign13730_e18858_d_n8, assign13730_e18858_d_n9, assign13730_e18858_d_n10, assign13730_e18858_d_n11,) = {
    if (var_guard449 != 0.0) {
        let assign13730_e18853: f64 = (var_tratio - 1.0);
        let assign13730_e18854: f64 = (p.p1059 * assign13730_e18853);
        let assign13730_e18855: f64 = (1.0 + assign13730_e18854);
        let assign13730_e18856: f64 = (var_eta0r_i * assign13730_e18855);
        (assign13730_e18856, (var_eta0r_i_dn3 * assign13730_e18855), ((var_eta0r_i_dn4 * assign13730_e18855) + (var_eta0r_i * (p.p1059 * var_tratio_dn4))), ((var_eta0r_i_dn5 * assign13730_e18855) + (var_eta0r_i * (p.p1059 * var_tratio_dn5))), (var_eta0r_i_dn6 * assign13730_e18855), (var_eta0r_i_dn7 * assign13730_e18855), (var_eta0r_i_dn8 * assign13730_e18855), (var_eta0r_i_dn9 * assign13730_e18855), (var_eta0r_i_dn10 * assign13730_e18855), (var_eta0r_i_dn11 * assign13730_e18855),)
    } else {
        (var_eta0r_t, var_eta0r_t_dn3, var_eta0r_t_dn4, var_eta0r_t_dn5, var_eta0r_t_dn6, var_eta0r_t_dn7, var_eta0r_t_dn8, var_eta0r_t_dn9, var_eta0r_t_dn10, var_eta0r_t_dn11,)
    }
};
        var_eta0r_t = assign13730_e18858;
        var_eta0r_t_dn3 = assign13730_e18858_d_n3;
        var_eta0r_t_dn4 = assign13730_e18858_d_n4;
        var_eta0r_t_dn5 = assign13730_e18858_d_n5;
        var_eta0r_t_dn6 = assign13730_e18858_d_n6;
        var_eta0r_t_dn7 = assign13730_e18858_d_n7;
        var_eta0r_t_dn8 = assign13730_e18858_d_n8;
        var_eta0r_t_dn9 = assign13730_e18858_d_n9;
        var_eta0r_t_dn10 = assign13730_e18858_d_n10;
        var_eta0r_t_dn11 = assign13730_e18858_d_n11;
        var_eta0r_t_rv = 0.0;

        let (assign13740_e18868,) = {
    if (p.p30 != 1.0) {
        let assign13740_e18864: f64 = (0.3333333333333333 * p.p347);
        (assign13740_e18864,)
    } else {
        let assign13740_e18867: f64 = (0.5 * p.p347);
        (assign13740_e18867,)
    }
};
        var_eta_mu = assign13740_e18868;
        var_eta_mu_rv = 0.0;

        let assign13750_e18872: f64 = (var_tratio).powf(var_ute_i);
        let assign13750_e18873: f64 = (var_u0_i * assign13750_e18872);
        var_u0_t = assign13750_e18873;
        var_u0_t_dn3 = 0.0;
        var_u0_t_dn4 = (var_u0_i * if 0.0 == 0.0 && ((var_ute_i) as f64).is_finite() && ((var_ute_i) as f64).fract() == 0.0 { if var_ute_i == 0.0 { 0.0 } else { (var_ute_i * ((var_tratio).powf(var_ute_i - 1.0) * var_tratio_dn4)) } } else { (assign13750_e18872 * (var_ute_i * (var_tratio_dn4 / var_tratio))) });
        var_u0_t_dn5 = (var_u0_i * if 0.0 == 0.0 && ((var_ute_i) as f64).is_finite() && ((var_ute_i) as f64).fract() == 0.0 { if var_ute_i == 0.0 { 0.0 } else { (var_ute_i * ((var_tratio).powf(var_ute_i - 1.0) * var_tratio_dn5)) } } else { (assign13750_e18872 * (var_ute_i * (var_tratio_dn5 / var_tratio))) });
        var_u0_t_dn6 = 0.0;
        var_u0_t_dn7 = 0.0;
        var_u0_t_dn8 = 0.0;
        var_u0_t_dn9 = 0.0;
        var_u0_t_dn10 = 0.0;
        var_u0_t_dn11 = 0.0;
        var_u0_t_rv = 0.0;

        let assign13760_e18879: f64 = (var_ua1_i * var_deltemp);
        let assign13760_e18880: f64 = (1.0 + assign13760_e18879);
        let assign13760_e18882: f64 = (assign13760_e18880 - 1e-6);
        let assign13760_e18886: f64 = (var_ua1_i * var_deltemp);
        let assign13760_e18887: f64 = (1.0 + assign13760_e18886);
        let assign13760_e18889: f64 = (assign13760_e18887 - 1e-6);
        let assign13760_e18893: f64 = (var_ua1_i * var_deltemp);
        let assign13760_e18894: f64 = (1.0 + assign13760_e18893);
        let assign13760_e18896: f64 = (assign13760_e18894 - 1e-6);
        let assign13760_e18897: f64 = (assign13760_e18889 * assign13760_e18896);
        let assign13760_e18900: f64 = (4.0 * 0.001);
        let assign13760_e18902: f64 = (assign13760_e18900 * 0.001);
        let assign13760_e18903: f64 = (assign13760_e18897 + assign13760_e18902);
        let assign13760_e18904: f64 = (assign13760_e18903).sqrt();
        let assign13760_e18905: f64 = (assign13760_e18882 + assign13760_e18904);
        let assign13760_e18906: f64 = (0.5 * assign13760_e18905);
        let assign13760_e18907: f64 = (var_ua_i * assign13760_e18906);
        var_ua_t = assign13760_e18907;
        var_ua_t_dn3 = (var_ua_i_dn3 * assign13760_e18906);
        var_ua_t_dn4 = ((var_ua_i_dn4 * assign13760_e18906) + (var_ua_i * (0.5 * ((var_ua1_i * var_deltemp_dn4) + ((((var_ua1_i * var_deltemp_dn4) * assign13760_e18896) + (assign13760_e18889 * (var_ua1_i * var_deltemp_dn4))) / (2.0 * assign13760_e18904))))));
        var_ua_t_dn5 = ((var_ua_i_dn5 * assign13760_e18906) + (var_ua_i * (0.5 * ((var_ua1_i * var_deltemp_dn5) + ((((var_ua1_i * var_deltemp_dn5) * assign13760_e18896) + (assign13760_e18889 * (var_ua1_i * var_deltemp_dn5))) / (2.0 * assign13760_e18904))))));
        var_ua_t_dn6 = (var_ua_i_dn6 * assign13760_e18906);
        var_ua_t_dn7 = (var_ua_i_dn7 * assign13760_e18906);
        var_ua_t_dn8 = (var_ua_i_dn8 * assign13760_e18906);
        var_ua_t_dn9 = (var_ua_i_dn9 * assign13760_e18906);
        var_ua_t_dn10 = (var_ua_i_dn10 * assign13760_e18906);
        var_ua_t_dn11 = (var_ua_i_dn11 * assign13760_e18906);
        var_ua_t_rv = 0.0;

        let assign13770_e18913: f64 = (var_uc1_i * var_deltemp);
        let assign13770_e18914: f64 = (1.0 + assign13770_e18913);
        let assign13770_e18916: f64 = (assign13770_e18914 - 1e-6);
        let assign13770_e18920: f64 = (var_uc1_i * var_deltemp);
        let assign13770_e18921: f64 = (1.0 + assign13770_e18920);
        let assign13770_e18923: f64 = (assign13770_e18921 - 1e-6);
        let assign13770_e18927: f64 = (var_uc1_i * var_deltemp);
        let assign13770_e18928: f64 = (1.0 + assign13770_e18927);
        let assign13770_e18930: f64 = (assign13770_e18928 - 1e-6);
        let assign13770_e18931: f64 = (assign13770_e18923 * assign13770_e18930);
        let assign13770_e18934: f64 = (4.0 * 0.001);
        let assign13770_e18936: f64 = (assign13770_e18934 * 0.001);
        let assign13770_e18937: f64 = (assign13770_e18931 + assign13770_e18936);
        let assign13770_e18938: f64 = (assign13770_e18937).sqrt();
        let assign13770_e18939: f64 = (assign13770_e18916 + assign13770_e18938);
        let assign13770_e18940: f64 = (0.5 * assign13770_e18939);
        let assign13770_e18941: f64 = (var_uc_i * assign13770_e18940);
        var_uc_t = assign13770_e18941;
        var_uc_t_dn3 = (var_uc_i_dn3 * assign13770_e18940);
        var_uc_t_dn4 = ((var_uc_i_dn4 * assign13770_e18940) + (var_uc_i * (0.5 * ((var_uc1_i * var_deltemp_dn4) + ((((var_uc1_i * var_deltemp_dn4) * assign13770_e18930) + (assign13770_e18923 * (var_uc1_i * var_deltemp_dn4))) / (2.0 * assign13770_e18938))))));
        var_uc_t_dn5 = ((var_uc_i_dn5 * assign13770_e18940) + (var_uc_i * (0.5 * ((var_uc1_i * var_deltemp_dn5) + ((((var_uc1_i * var_deltemp_dn5) * assign13770_e18930) + (assign13770_e18923 * (var_uc1_i * var_deltemp_dn5))) / (2.0 * assign13770_e18938))))));
        var_uc_t_dn6 = (var_uc_i_dn6 * assign13770_e18940);
        var_uc_t_dn7 = (var_uc_i_dn7 * assign13770_e18940);
        var_uc_t_dn8 = (var_uc_i_dn8 * assign13770_e18940);
        var_uc_t_dn9 = (var_uc_i_dn9 * assign13770_e18940);
        var_uc_t_dn10 = (var_uc_i_dn10 * assign13770_e18940);
        var_uc_t_dn11 = (var_uc_i_dn11 * assign13770_e18940);
        var_uc_t_rv = 0.0;

        let assign13780_e18945: f64 = (var_tratio).powf(var_ud1_i);
        let assign13780_e18946: f64 = (var_ud_i * assign13780_e18945);
        var_ud_t = assign13780_e18946;
        var_ud_t_dn3 = (var_ud_i_dn3 * assign13780_e18945);
        var_ud_t_dn4 = ((var_ud_i_dn4 * assign13780_e18945) + (var_ud_i * if 0.0 == 0.0 && ((var_ud1_i) as f64).is_finite() && ((var_ud1_i) as f64).fract() == 0.0 { if var_ud1_i == 0.0 { 0.0 } else { (var_ud1_i * ((var_tratio).powf(var_ud1_i - 1.0) * var_tratio_dn4)) } } else { (assign13780_e18945 * (var_ud1_i * (var_tratio_dn4 / var_tratio))) }));
        var_ud_t_dn5 = ((var_ud_i_dn5 * assign13780_e18945) + (var_ud_i * if 0.0 == 0.0 && ((var_ud1_i) as f64).is_finite() && ((var_ud1_i) as f64).fract() == 0.0 { if var_ud1_i == 0.0 { 0.0 } else { (var_ud1_i * ((var_tratio).powf(var_ud1_i - 1.0) * var_tratio_dn5)) } } else { (assign13780_e18945 * (var_ud1_i * (var_tratio_dn5 / var_tratio))) }));
        var_ud_t_dn6 = (var_ud_i_dn6 * assign13780_e18945);
        var_ud_t_dn7 = (var_ud_i_dn7 * assign13780_e18945);
        var_ud_t_dn8 = (var_ud_i_dn8 * assign13780_e18945);
        var_ud_t_dn9 = (var_ud_i_dn9 * assign13780_e18945);
        var_ud_t_dn10 = (var_ud_i_dn10 * assign13780_e18945);
        var_ud_t_dn11 = (var_ud_i_dn11 * assign13780_e18945);
        var_ud_t_rv = 0.0;

        let assign13790_e18950: f64 = (var_tratio).powf(var_ucste_i);
        let assign13790_e18951: f64 = (var_ucs_i * assign13790_e18950);
        var_ucs_t = assign13790_e18951;
        var_ucs_t_dn4 = (var_ucs_i * if 0.0 == 0.0 && ((var_ucste_i) as f64).is_finite() && ((var_ucste_i) as f64).fract() == 0.0 { if var_ucste_i == 0.0 { 0.0 } else { (var_ucste_i * ((var_tratio).powf(var_ucste_i - 1.0) * var_tratio_dn4)) } } else { (assign13790_e18950 * (var_ucste_i * (var_tratio_dn4 / var_tratio))) });
        var_ucs_t_dn5 = (var_ucs_i * if 0.0 == 0.0 && ((var_ucste_i) as f64).is_finite() && ((var_ucste_i) as f64).fract() == 0.0 { if var_ucste_i == 0.0 { 0.0 } else { (var_ucste_i * ((var_tratio).powf(var_ucste_i - 1.0) * var_tratio_dn5)) } } else { (assign13790_e18950 * (var_ucste_i * (var_tratio_dn5 / var_tratio))) });
        var_ucs_t_rv = 0.0;

        let assign13800_e18958: f64 = (var_tratio - 1.0);
        let assign13800_e18959: f64 = (var_eu1_i * assign13800_e18958);
        let assign13800_e18960: f64 = (1.0 + assign13800_e18959);
        let assign13800_e18965: f64 = (var_tratio - 1.0);
        let assign13800_e18966: f64 = (var_eu1_i * assign13800_e18965);
        let assign13800_e18967: f64 = (1.0 + assign13800_e18966);
        let assign13800_e18972: f64 = (var_tratio - 1.0);
        let assign13800_e18973: f64 = (var_eu1_i * assign13800_e18972);
        let assign13800_e18974: f64 = (1.0 + assign13800_e18973);
        let assign13800_e18975: f64 = (assign13800_e18967 * assign13800_e18974);
        let assign13800_e18978: f64 = (4.0 * 0.001);
        let assign13800_e18980: f64 = (assign13800_e18978 * 0.001);
        let assign13800_e18981: f64 = (assign13800_e18975 + assign13800_e18980);
        let assign13800_e18982: f64 = (assign13800_e18981).sqrt();
        let assign13800_e18983: f64 = (assign13800_e18960 + assign13800_e18982);
        let assign13800_e18984: f64 = (0.5 * assign13800_e18983);
        let assign13800_e18985: f64 = (var_eu_i * assign13800_e18984);
        var_eu_t = assign13800_e18985;
        var_eu_t_dn3 = (var_eu_i_dn3 * assign13800_e18984);
        var_eu_t_dn4 = ((var_eu_i_dn4 * assign13800_e18984) + (var_eu_i * (0.5 * ((var_eu1_i * var_tratio_dn4) + ((((var_eu1_i * var_tratio_dn4) * assign13800_e18974) + (assign13800_e18967 * (var_eu1_i * var_tratio_dn4))) / (2.0 * assign13800_e18982))))));
        var_eu_t_dn5 = ((var_eu_i_dn5 * assign13800_e18984) + (var_eu_i * (0.5 * ((var_eu1_i * var_tratio_dn5) + ((((var_eu1_i * var_tratio_dn5) * assign13800_e18974) + (assign13800_e18967 * (var_eu1_i * var_tratio_dn5))) / (2.0 * assign13800_e18982))))));
        var_eu_t_dn6 = (var_eu_i_dn6 * assign13800_e18984);
        var_eu_t_dn7 = (var_eu_i_dn7 * assign13800_e18984);
        var_eu_t_dn8 = (var_eu_i_dn8 * assign13800_e18984);
        var_eu_t_dn9 = (var_eu_i_dn9 * assign13800_e18984);
        var_eu_t_dn10 = (var_eu_i_dn10 * assign13800_e18984);
        var_eu_t_dn11 = (var_eu_i_dn11 * assign13800_e18984);
        var_eu_t_rv = 0.0;

        let assign13810_e18988: f64 = if p.p35 != 0.0 { 1.0 } else { 0.0 };
        var_guard450 = assign13810_e18988;
        var_guard450_rv = 0.0;

        let (assign13820_e18996, assign13820_e18996_d_n4, assign13820_e18996_d_n5,) = {
    if (var_guard450 != 0.0) {
        let assign13820_e18993: f64 = (var_tratio).powf(var_ute_i);
        let assign13820_e18994: f64 = (var_u0r_i * assign13820_e18993);
        (assign13820_e18994, (var_u0r_i * if 0.0 == 0.0 && ((var_ute_i) as f64).is_finite() && ((var_ute_i) as f64).fract() == 0.0 { if var_ute_i == 0.0 { 0.0 } else { (var_ute_i * ((var_tratio).powf(var_ute_i - 1.0) * var_tratio_dn4)) } } else { (assign13820_e18993 * (var_ute_i * (var_tratio_dn4 / var_tratio))) }), (var_u0r_i * if 0.0 == 0.0 && ((var_ute_i) as f64).is_finite() && ((var_ute_i) as f64).fract() == 0.0 { if var_ute_i == 0.0 { 0.0 } else { (var_ute_i * ((var_tratio).powf(var_ute_i - 1.0) * var_tratio_dn5)) } } else { (assign13820_e18993 * (var_ute_i * (var_tratio_dn5 / var_tratio))) }),)
    } else {
        (var_u0r_t, var_u0r_t_dn4, var_u0r_t_dn5,)
    }
};
        var_u0r_t = assign13820_e18996;
        var_u0r_t_dn4 = assign13820_e18996_d_n4;
        var_u0r_t_dn5 = assign13820_e18996_d_n5;
        var_u0r_t_rv = 0.0;

        *var_eta0_t_slot = var_eta0_t;
        *var_eta0_t_dn10_slot = var_eta0_t_dn10;
        *var_eta0_t_dn11_slot = var_eta0_t_dn11;
        *var_eta0_t_dn3_slot = var_eta0_t_dn3;
        *var_eta0_t_dn4_slot = var_eta0_t_dn4;
        *var_eta0_t_dn5_slot = var_eta0_t_dn5;
        *var_eta0_t_dn6_slot = var_eta0_t_dn6;
        *var_eta0_t_dn7_slot = var_eta0_t_dn7;
        *var_eta0_t_dn8_slot = var_eta0_t_dn8;
        *var_eta0_t_dn9_slot = var_eta0_t_dn9;
        *var_eta0_t_rv_slot = var_eta0_t_rv;
        *var_eta0r_t_slot = var_eta0r_t;
        *var_eta0r_t_dn10_slot = var_eta0r_t_dn10;
        *var_eta0r_t_dn11_slot = var_eta0r_t_dn11;
        *var_eta0r_t_dn3_slot = var_eta0r_t_dn3;
        *var_eta0r_t_dn4_slot = var_eta0r_t_dn4;
        *var_eta0r_t_dn5_slot = var_eta0r_t_dn5;
        *var_eta0r_t_dn6_slot = var_eta0r_t_dn6;
        *var_eta0r_t_dn7_slot = var_eta0r_t_dn7;
        *var_eta0r_t_dn8_slot = var_eta0r_t_dn8;
        *var_eta0r_t_dn9_slot = var_eta0r_t_dn9;
        *var_eta0r_t_rv_slot = var_eta0r_t_rv;
        *var_eta_mu_slot = var_eta_mu;
        *var_eta_mu_rv_slot = var_eta_mu_rv;
        *var_eu_t_slot = var_eu_t;
        *var_eu_t_dn10_slot = var_eu_t_dn10;
        *var_eu_t_dn11_slot = var_eu_t_dn11;
        *var_eu_t_dn3_slot = var_eu_t_dn3;
        *var_eu_t_dn4_slot = var_eu_t_dn4;
        *var_eu_t_dn5_slot = var_eu_t_dn5;
        *var_eu_t_dn6_slot = var_eu_t_dn6;
        *var_eu_t_dn7_slot = var_eu_t_dn7;
        *var_eu_t_dn8_slot = var_eu_t_dn8;
        *var_eu_t_dn9_slot = var_eu_t_dn9;
        *var_eu_t_rv_slot = var_eu_t_rv;
        *var_guard436_slot = var_guard436;
        *var_guard436_rv_slot = var_guard436_rv;
        *var_guard437_slot = var_guard437;
        *var_guard437_rv_slot = var_guard437_rv;
        *var_guard438_slot = var_guard438;
        *var_guard438_rv_slot = var_guard438_rv;
        *var_guard449_slot = var_guard449;
        *var_guard449_rv_slot = var_guard449_rv;
        *var_guard450_slot = var_guard450;
        *var_guard450_rv_slot = var_guard450_rv;
        *var_litl_slot = var_litl;
        *var_litl_rv_slot = var_litl_rv;
        *var_nfactor_t_slot = var_nfactor_t;
        *var_nfactor_t_dn10_slot = var_nfactor_t_dn10;
        *var_nfactor_t_dn11_slot = var_nfactor_t_dn11;
        *var_nfactor_t_dn3_slot = var_nfactor_t_dn3;
        *var_nfactor_t_dn4_slot = var_nfactor_t_dn4;
        *var_nfactor_t_dn5_slot = var_nfactor_t_dn5;
        *var_nfactor_t_dn6_slot = var_nfactor_t_dn6;
        *var_nfactor_t_dn7_slot = var_nfactor_t_dn7;
        *var_nfactor_t_dn8_slot = var_nfactor_t_dn8;
        *var_nfactor_t_dn9_slot = var_nfactor_t_dn9;
        *var_nfactor_t_rv_slot = var_nfactor_t_rv;
        *var_ni_slot = var_ni;
        *var_ni_dn10_slot = var_ni_dn10;
        *var_ni_dn11_slot = var_ni_dn11;
        *var_ni_dn3_slot = var_ni_dn3;
        *var_ni_dn4_slot = var_ni_dn4;
        *var_ni_dn5_slot = var_ni_dn5;
        *var_ni_dn6_slot = var_ni_dn6;
        *var_ni_dn7_slot = var_ni_dn7;
        *var_ni_dn8_slot = var_ni_dn8;
        *var_ni_dn9_slot = var_ni_dn9;
        *var_ni_rv_slot = var_ni_rv;
        *var_phib_slot = var_phib;
        *var_phib_dn10_slot = var_phib_dn10;
        *var_phib_dn11_slot = var_phib_dn11;
        *var_phib_dn3_slot = var_phib_dn3;
        *var_phib_dn4_slot = var_phib_dn4;
        *var_phib_dn5_slot = var_phib_dn5;
        *var_phib_dn6_slot = var_phib_dn6;
        *var_phib_dn7_slot = var_phib_dn7;
        *var_phib_dn8_slot = var_phib_dn8;
        *var_phib_dn9_slot = var_phib_dn9;
        *var_phib_rv_slot = var_phib_rv;
        *var_phist_slot = var_phist;
        *var_phist_dn10_slot = var_phist_dn10;
        *var_phist_dn11_slot = var_phist_dn11;
        *var_phist_dn3_slot = var_phist_dn3;
        *var_phist_dn4_slot = var_phist_dn4;
        *var_phist_dn5_slot = var_phist_dn5;
        *var_phist_dn6_slot = var_phist_dn6;
        *var_phist_dn7_slot = var_phist_dn7;
        *var_phist_dn8_slot = var_phist_dn8;
        *var_phist_dn9_slot = var_phist_dn9;
        *var_phist_rv_slot = var_phist_rv;
        *var_sqrtphist_slot = var_sqrtphist;
        *var_sqrtphist_dn10_slot = var_sqrtphist_dn10;
        *var_sqrtphist_dn11_slot = var_sqrtphist_dn11;
        *var_sqrtphist_dn3_slot = var_sqrtphist_dn3;
        *var_sqrtphist_dn4_slot = var_sqrtphist_dn4;
        *var_sqrtphist_dn5_slot = var_sqrtphist_dn5;
        *var_sqrtphist_dn6_slot = var_sqrtphist_dn6;
        *var_sqrtphist_dn7_slot = var_sqrtphist_dn7;
        *var_sqrtphist_dn8_slot = var_sqrtphist_dn8;
        *var_sqrtphist_dn9_slot = var_sqrtphist_dn9;
        *var_sqrtphist_rv_slot = var_sqrtphist_rv;
        *var_t0_slot = var_t0;
        *var_t0_dn10_slot = var_t0_dn10;
        *var_t0_dn11_slot = var_t0_dn11;
        *var_t0_dn3_slot = var_t0_dn3;
        *var_t0_dn4_slot = var_t0_dn4;
        *var_t0_dn5_slot = var_t0_dn5;
        *var_t0_dn6_slot = var_t0_dn6;
        *var_t0_dn7_slot = var_t0_dn7;
        *var_t0_dn8_slot = var_t0_dn8;
        *var_t0_dn9_slot = var_t0_dn9;
        *var_t0_rv_slot = var_t0_rv;
        *var_t1_slot = var_t1;
        *var_t1_dn10_slot = var_t1_dn10;
        *var_t1_dn11_slot = var_t1_dn11;
        *var_t1_dn3_slot = var_t1_dn3;
        *var_t1_dn4_slot = var_t1_dn4;
        *var_t1_dn5_slot = var_t1_dn5;
        *var_t1_dn6_slot = var_t1_dn6;
        *var_t1_dn7_slot = var_t1_dn7;
        *var_t1_dn8_slot = var_t1_dn8;
        *var_t1_dn9_slot = var_t1_dn9;
        *var_t1_rv_slot = var_t1_rv;
        *var_t1dep_slot = var_t1dep;
        *var_t1dep_dn10_slot = var_t1dep_dn10;
        *var_t1dep_dn11_slot = var_t1dep_dn11;
        *var_t1dep_dn3_slot = var_t1dep_dn3;
        *var_t1dep_dn4_slot = var_t1dep_dn4;
        *var_t1dep_dn5_slot = var_t1dep_dn5;
        *var_t1dep_dn6_slot = var_t1dep_dn6;
        *var_t1dep_dn7_slot = var_t1dep_dn7;
        *var_t1dep_dn8_slot = var_t1dep_dn8;
        *var_t1dep_dn9_slot = var_t1dep_dn9;
        *var_t1dep_rv_slot = var_t1dep_rv;
        *var_u0_t_slot = var_u0_t;
        *var_u0_t_dn10_slot = var_u0_t_dn10;
        *var_u0_t_dn11_slot = var_u0_t_dn11;
        *var_u0_t_dn3_slot = var_u0_t_dn3;
        *var_u0_t_dn4_slot = var_u0_t_dn4;
        *var_u0_t_dn5_slot = var_u0_t_dn5;
        *var_u0_t_dn6_slot = var_u0_t_dn6;
        *var_u0_t_dn7_slot = var_u0_t_dn7;
        *var_u0_t_dn8_slot = var_u0_t_dn8;
        *var_u0_t_dn9_slot = var_u0_t_dn9;
        *var_u0_t_rv_slot = var_u0_t_rv;
        *var_u0r_t_slot = var_u0r_t;
        *var_u0r_t_dn4_slot = var_u0r_t_dn4;
        *var_u0r_t_dn5_slot = var_u0r_t_dn5;
        *var_u0r_t_rv_slot = var_u0r_t_rv;
        *var_ua_t_slot = var_ua_t;
        *var_ua_t_dn10_slot = var_ua_t_dn10;
        *var_ua_t_dn11_slot = var_ua_t_dn11;
        *var_ua_t_dn3_slot = var_ua_t_dn3;
        *var_ua_t_dn4_slot = var_ua_t_dn4;
        *var_ua_t_dn5_slot = var_ua_t_dn5;
        *var_ua_t_dn6_slot = var_ua_t_dn6;
        *var_ua_t_dn7_slot = var_ua_t_dn7;
        *var_ua_t_dn8_slot = var_ua_t_dn8;
        *var_ua_t_dn9_slot = var_ua_t_dn9;
        *var_ua_t_rv_slot = var_ua_t_rv;
        *var_uc_t_slot = var_uc_t;
        *var_uc_t_dn10_slot = var_uc_t_dn10;
        *var_uc_t_dn11_slot = var_uc_t_dn11;
        *var_uc_t_dn3_slot = var_uc_t_dn3;
        *var_uc_t_dn4_slot = var_uc_t_dn4;
        *var_uc_t_dn5_slot = var_uc_t_dn5;
        *var_uc_t_dn6_slot = var_uc_t_dn6;
        *var_uc_t_dn7_slot = var_uc_t_dn7;
        *var_uc_t_dn8_slot = var_uc_t_dn8;
        *var_uc_t_dn9_slot = var_uc_t_dn9;
        *var_uc_t_rv_slot = var_uc_t_rv;
        *var_ucs_t_slot = var_ucs_t;
        *var_ucs_t_dn4_slot = var_ucs_t_dn4;
        *var_ucs_t_dn5_slot = var_ucs_t_dn5;
        *var_ucs_t_rv_slot = var_ucs_t_rv;
        *var_ud_t_slot = var_ud_t;
        *var_ud_t_dn10_slot = var_ud_t_dn10;
        *var_ud_t_dn11_slot = var_ud_t_dn11;
        *var_ud_t_dn3_slot = var_ud_t_dn3;
        *var_ud_t_dn4_slot = var_ud_t_dn4;
        *var_ud_t_dn5_slot = var_ud_t_dn5;
        *var_ud_t_dn6_slot = var_ud_t_dn6;
        *var_ud_t_dn7_slot = var_ud_t_dn7;
        *var_ud_t_dn8_slot = var_ud_t_dn8;
        *var_ud_t_dn9_slot = var_ud_t_dn9;
        *var_ud_t_rv_slot = var_ud_t_rv;
        *var_vbi_edge_slot = var_vbi_edge;
        *var_vbi_edge_dn10_slot = var_vbi_edge_dn10;
        *var_vbi_edge_dn11_slot = var_vbi_edge_dn11;
        *var_vbi_edge_dn3_slot = var_vbi_edge_dn3;
        *var_vbi_edge_dn4_slot = var_vbi_edge_dn4;
        *var_vbi_edge_dn5_slot = var_vbi_edge_dn5;
        *var_vbi_edge_dn6_slot = var_vbi_edge_dn6;
        *var_vbi_edge_dn7_slot = var_vbi_edge_dn7;
        *var_vbi_edge_dn8_slot = var_vbi_edge_dn8;
        *var_vbi_edge_dn9_slot = var_vbi_edge_dn9;
        *var_vbi_edge_rv_slot = var_vbi_edge_rv;
        *var_vfbsdr_slot = var_vfbsdr;
        *var_vfbsdr_dn4_slot = var_vfbsdr_dn4;
        *var_vfbsdr_dn5_slot = var_vfbsdr_dn5;
        *var_vfbsdr_rv_slot = var_vfbsdr_rv;
    }

    pub(super) fn stamp_reactive_block_23(
        p: &Parameters,
        var_a11_i: f64,
        var_a1_i: f64,
        var_a21_i: f64,
        var_a2_i: f64,
        var_at_i: f64,
        var_beta0_i: f64,
        var_bgidl1_i: f64,
        var_bgidl_i: f64,
        var_bgisl1_i: f64,
        var_bgisl_i: f64,
        var_c01_i: f64,
        var_c0_i: f64,
        var_c0si1_i: f64,
        var_c0si_i: f64,
        var_delta_i: f64,
        var_delta_i_dn10: f64,
        var_delta_i_dn11: f64,
        var_delta_i_dn3: f64,
        var_delta_i_dn4: f64,
        var_delta_i_dn5: f64,
        var_delta_i_dn6: f64,
        var_delta_i_dn7: f64,
        var_delta_i_dn8: f64,
        var_delta_i_dn9: f64,
        var_deltemp: f64,
        var_deltemp_dn4: f64,
        var_deltemp_dn5: f64,
        var_guard450: f64,
        var_iit_i: f64,
        var_k01_i: f64,
        var_k0_i: f64,
        var_m01_i: f64,
        var_m0_i: f64,
        var_prt_i: f64,
        var_ptwg_i: f64,
        var_ptwg_i_dn10: f64,
        var_ptwg_i_dn11: f64,
        var_ptwg_i_dn3: f64,
        var_ptwg_i_dn4: f64,
        var_ptwg_i_dn5: f64,
        var_ptwg_i_dn6: f64,
        var_ptwg_i_dn7: f64,
        var_ptwg_i_dn8: f64,
        var_ptwg_i_dn9: f64,
        var_ptwgr_i: f64,
        var_ptwgr_i_dn10: f64,
        var_ptwgr_i_dn11: f64,
        var_ptwgr_i_dn3: f64,
        var_ptwgr_i_dn4: f64,
        var_ptwgr_i_dn5: f64,
        var_ptwgr_i_dn6: f64,
        var_ptwgr_i_dn7: f64,
        var_ptwgr_i_dn8: f64,
        var_ptwgr_i_dn9: f64,
        var_ptwgt_i: f64,
        var_tratio: f64,
        var_tratio_dn4: f64,
        var_tratio_dn5: f64,
        var_ua1_i: f64,
        var_uar_i: f64,
        var_uar_i_dn10: f64,
        var_uar_i_dn11: f64,
        var_uar_i_dn3: f64,
        var_uar_i_dn4: f64,
        var_uar_i_dn5: f64,
        var_uar_i_dn6: f64,
        var_uar_i_dn7: f64,
        var_uar_i_dn8: f64,
        var_uar_i_dn9: f64,
        var_uc1_i: f64,
        var_ucr_i: f64,
        var_ucr_i_dn10: f64,
        var_ucr_i_dn11: f64,
        var_ucr_i_dn3: f64,
        var_ucr_i_dn4: f64,
        var_ucr_i_dn5: f64,
        var_ucr_i_dn6: f64,
        var_ucr_i_dn7: f64,
        var_ucr_i_dn8: f64,
        var_ucr_i_dn9: f64,
        var_ucsr_i: f64,
        var_ucste_i: f64,
        var_ud1_i: f64,
        var_udr_i: f64,
        var_udr_i_dn10: f64,
        var_udr_i_dn11: f64,
        var_udr_i_dn3: f64,
        var_udr_i_dn4: f64,
        var_udr_i_dn5: f64,
        var_udr_i_dn6: f64,
        var_udr_i_dn7: f64,
        var_udr_i_dn8: f64,
        var_udr_i_dn9: f64,
        var_vsat_i: f64,
        var_vsat_i_dn10: f64,
        var_vsat_i_dn11: f64,
        var_vsat_i_dn3: f64,
        var_vsat_i_dn4: f64,
        var_vsat_i_dn5: f64,
        var_vsat_i_dn6: f64,
        var_vsat_i_dn7: f64,
        var_vsat_i_dn8: f64,
        var_vsat_i_dn9: f64,
        var_vsatcv_i: f64,
        var_vsatcv_i_dn10: f64,
        var_vsatcv_i_dn11: f64,
        var_vsatcv_i_dn3: f64,
        var_vsatcv_i_dn4: f64,
        var_vsatcv_i_dn5: f64,
        var_vsatcv_i_dn6: f64,
        var_vsatcv_i_dn7: f64,
        var_vsatcv_i_dn8: f64,
        var_vsatcv_i_dn9: f64,
        var_vsatr_i: f64,
        var_vsatr_i_dn10: f64,
        var_vsatr_i_dn11: f64,
        var_vsatr_i_dn3: f64,
        var_vsatr_i_dn4: f64,
        var_vsatr_i_dn5: f64,
        var_vsatr_i_dn6: f64,
        var_vsatr_i_dn7: f64,
        var_vsatr_i_dn8: f64,
        var_vsatr_i_dn9: f64,
        var_a1_t_slot: &mut f64,
        var_a1_t_dn4_slot: &mut f64,
        var_a1_t_dn5_slot: &mut f64,
        var_a1_t_rv_slot: &mut f64,
        var_a2_t_slot: &mut f64,
        var_a2_t_dn4_slot: &mut f64,
        var_a2_t_dn5_slot: &mut f64,
        var_a2_t_rv_slot: &mut f64,
        var_beta0_t_slot: &mut f64,
        var_beta0_t_dn4_slot: &mut f64,
        var_beta0_t_dn5_slot: &mut f64,
        var_beta0_t_rv_slot: &mut f64,
        var_bgidl_t_slot: &mut f64,
        var_bgidl_t_dn4_slot: &mut f64,
        var_bgidl_t_dn5_slot: &mut f64,
        var_bgidl_t_rv_slot: &mut f64,
        var_bgisl_t_slot: &mut f64,
        var_bgisl_t_dn4_slot: &mut f64,
        var_bgisl_t_dn5_slot: &mut f64,
        var_bgisl_t_rv_slot: &mut f64,
        var_c0_t_slot: &mut f64,
        var_c0_t_dn4_slot: &mut f64,
        var_c0_t_dn5_slot: &mut f64,
        var_c0_t_rv_slot: &mut f64,
        var_c0si_t_slot: &mut f64,
        var_c0si_t_dn4_slot: &mut f64,
        var_c0si_t_dn5_slot: &mut f64,
        var_c0si_t_rv_slot: &mut f64,
        var_delta_t_slot: &mut f64,
        var_delta_t_dn10_slot: &mut f64,
        var_delta_t_dn11_slot: &mut f64,
        var_delta_t_dn3_slot: &mut f64,
        var_delta_t_dn4_slot: &mut f64,
        var_delta_t_dn5_slot: &mut f64,
        var_delta_t_dn6_slot: &mut f64,
        var_delta_t_dn7_slot: &mut f64,
        var_delta_t_dn8_slot: &mut f64,
        var_delta_t_dn9_slot: &mut f64,
        var_delta_t_rv_slot: &mut f64,
        var_guard451_slot: &mut f64,
        var_guard451_rv_slot: &mut f64,
        var_guard452_slot: &mut f64,
        var_guard452_rv_slot: &mut f64,
        var_guard453_slot: &mut f64,
        var_guard453_rv_slot: &mut f64,
        var_guard454_slot: &mut f64,
        var_guard454_rv_slot: &mut f64,
        var_guard455_slot: &mut f64,
        var_guard455_rv_slot: &mut f64,
        var_k0_t_slot: &mut f64,
        var_k0_t_dn4_slot: &mut f64,
        var_k0_t_dn5_slot: &mut f64,
        var_k0_t_rv_slot: &mut f64,
        var_m0_t_slot: &mut f64,
        var_m0_t_dn4_slot: &mut f64,
        var_m0_t_dn5_slot: &mut f64,
        var_m0_t_rv_slot: &mut f64,
        var_ptwg_t_slot: &mut f64,
        var_ptwg_t_dn10_slot: &mut f64,
        var_ptwg_t_dn11_slot: &mut f64,
        var_ptwg_t_dn3_slot: &mut f64,
        var_ptwg_t_dn4_slot: &mut f64,
        var_ptwg_t_dn5_slot: &mut f64,
        var_ptwg_t_dn6_slot: &mut f64,
        var_ptwg_t_dn7_slot: &mut f64,
        var_ptwg_t_dn8_slot: &mut f64,
        var_ptwg_t_dn9_slot: &mut f64,
        var_ptwg_t_rv_slot: &mut f64,
        var_ptwgr_t_slot: &mut f64,
        var_ptwgr_t_dn10_slot: &mut f64,
        var_ptwgr_t_dn11_slot: &mut f64,
        var_ptwgr_t_dn3_slot: &mut f64,
        var_ptwgr_t_dn4_slot: &mut f64,
        var_ptwgr_t_dn5_slot: &mut f64,
        var_ptwgr_t_dn6_slot: &mut f64,
        var_ptwgr_t_dn7_slot: &mut f64,
        var_ptwgr_t_dn8_slot: &mut f64,
        var_ptwgr_t_dn9_slot: &mut f64,
        var_ptwgr_t_rv_slot: &mut f64,
        var_rdstemp_slot: &mut f64,
        var_rdstemp_dn4_slot: &mut f64,
        var_rdstemp_dn5_slot: &mut f64,
        var_rdstemp_rv_slot: &mut f64,
        var_uar_t_slot: &mut f64,
        var_uar_t_dn10_slot: &mut f64,
        var_uar_t_dn11_slot: &mut f64,
        var_uar_t_dn3_slot: &mut f64,
        var_uar_t_dn4_slot: &mut f64,
        var_uar_t_dn5_slot: &mut f64,
        var_uar_t_dn6_slot: &mut f64,
        var_uar_t_dn7_slot: &mut f64,
        var_uar_t_dn8_slot: &mut f64,
        var_uar_t_dn9_slot: &mut f64,
        var_uar_t_rv_slot: &mut f64,
        var_ucr_t_slot: &mut f64,
        var_ucr_t_dn10_slot: &mut f64,
        var_ucr_t_dn11_slot: &mut f64,
        var_ucr_t_dn3_slot: &mut f64,
        var_ucr_t_dn4_slot: &mut f64,
        var_ucr_t_dn5_slot: &mut f64,
        var_ucr_t_dn6_slot: &mut f64,
        var_ucr_t_dn7_slot: &mut f64,
        var_ucr_t_dn8_slot: &mut f64,
        var_ucr_t_dn9_slot: &mut f64,
        var_ucr_t_rv_slot: &mut f64,
        var_ucsr_t_slot: &mut f64,
        var_ucsr_t_dn4_slot: &mut f64,
        var_ucsr_t_dn5_slot: &mut f64,
        var_ucsr_t_rv_slot: &mut f64,
        var_udr_t_slot: &mut f64,
        var_udr_t_dn10_slot: &mut f64,
        var_udr_t_dn11_slot: &mut f64,
        var_udr_t_dn3_slot: &mut f64,
        var_udr_t_dn4_slot: &mut f64,
        var_udr_t_dn5_slot: &mut f64,
        var_udr_t_dn6_slot: &mut f64,
        var_udr_t_dn7_slot: &mut f64,
        var_udr_t_dn8_slot: &mut f64,
        var_udr_t_dn9_slot: &mut f64,
        var_udr_t_rv_slot: &mut f64,
        var_vsat_t_slot: &mut f64,
        var_vsat_t_dn10_slot: &mut f64,
        var_vsat_t_dn11_slot: &mut f64,
        var_vsat_t_dn3_slot: &mut f64,
        var_vsat_t_dn4_slot: &mut f64,
        var_vsat_t_dn5_slot: &mut f64,
        var_vsat_t_dn6_slot: &mut f64,
        var_vsat_t_dn7_slot: &mut f64,
        var_vsat_t_dn8_slot: &mut f64,
        var_vsat_t_dn9_slot: &mut f64,
        var_vsat_t_rv_slot: &mut f64,
        var_vsatcv_t_slot: &mut f64,
        var_vsatcv_t_dn10_slot: &mut f64,
        var_vsatcv_t_dn11_slot: &mut f64,
        var_vsatcv_t_dn3_slot: &mut f64,
        var_vsatcv_t_dn4_slot: &mut f64,
        var_vsatcv_t_dn5_slot: &mut f64,
        var_vsatcv_t_dn6_slot: &mut f64,
        var_vsatcv_t_dn7_slot: &mut f64,
        var_vsatcv_t_dn8_slot: &mut f64,
        var_vsatcv_t_dn9_slot: &mut f64,
        var_vsatcv_t_rv_slot: &mut f64,
        var_vsatr_t_slot: &mut f64,
        var_vsatr_t_dn10_slot: &mut f64,
        var_vsatr_t_dn11_slot: &mut f64,
        var_vsatr_t_dn3_slot: &mut f64,
        var_vsatr_t_dn4_slot: &mut f64,
        var_vsatr_t_dn5_slot: &mut f64,
        var_vsatr_t_dn6_slot: &mut f64,
        var_vsatr_t_dn7_slot: &mut f64,
        var_vsatr_t_dn8_slot: &mut f64,
        var_vsatr_t_dn9_slot: &mut f64,
        var_vsatr_t_rv_slot: &mut f64,
    ) {
        let mut var_a1_t: f64 = *var_a1_t_slot;
        let mut var_a1_t_dn4: f64 = *var_a1_t_dn4_slot;
        let mut var_a1_t_dn5: f64 = *var_a1_t_dn5_slot;
        let mut var_a1_t_rv: f64 = *var_a1_t_rv_slot;
        let mut var_a2_t: f64 = *var_a2_t_slot;
        let mut var_a2_t_dn4: f64 = *var_a2_t_dn4_slot;
        let mut var_a2_t_dn5: f64 = *var_a2_t_dn5_slot;
        let mut var_a2_t_rv: f64 = *var_a2_t_rv_slot;
        let mut var_beta0_t: f64 = *var_beta0_t_slot;
        let mut var_beta0_t_dn4: f64 = *var_beta0_t_dn4_slot;
        let mut var_beta0_t_dn5: f64 = *var_beta0_t_dn5_slot;
        let mut var_beta0_t_rv: f64 = *var_beta0_t_rv_slot;
        let mut var_bgidl_t: f64 = *var_bgidl_t_slot;
        let mut var_bgidl_t_dn4: f64 = *var_bgidl_t_dn4_slot;
        let mut var_bgidl_t_dn5: f64 = *var_bgidl_t_dn5_slot;
        let mut var_bgidl_t_rv: f64 = *var_bgidl_t_rv_slot;
        let mut var_bgisl_t: f64 = *var_bgisl_t_slot;
        let mut var_bgisl_t_dn4: f64 = *var_bgisl_t_dn4_slot;
        let mut var_bgisl_t_dn5: f64 = *var_bgisl_t_dn5_slot;
        let mut var_bgisl_t_rv: f64 = *var_bgisl_t_rv_slot;
        let mut var_c0_t: f64 = *var_c0_t_slot;
        let mut var_c0_t_dn4: f64 = *var_c0_t_dn4_slot;
        let mut var_c0_t_dn5: f64 = *var_c0_t_dn5_slot;
        let mut var_c0_t_rv: f64 = *var_c0_t_rv_slot;
        let mut var_c0si_t: f64 = *var_c0si_t_slot;
        let mut var_c0si_t_dn4: f64 = *var_c0si_t_dn4_slot;
        let mut var_c0si_t_dn5: f64 = *var_c0si_t_dn5_slot;
        let mut var_c0si_t_rv: f64 = *var_c0si_t_rv_slot;
        let mut var_delta_t: f64 = *var_delta_t_slot;
        let mut var_delta_t_dn10: f64 = *var_delta_t_dn10_slot;
        let mut var_delta_t_dn11: f64 = *var_delta_t_dn11_slot;
        let mut var_delta_t_dn3: f64 = *var_delta_t_dn3_slot;
        let mut var_delta_t_dn4: f64 = *var_delta_t_dn4_slot;
        let mut var_delta_t_dn5: f64 = *var_delta_t_dn5_slot;
        let mut var_delta_t_dn6: f64 = *var_delta_t_dn6_slot;
        let mut var_delta_t_dn7: f64 = *var_delta_t_dn7_slot;
        let mut var_delta_t_dn8: f64 = *var_delta_t_dn8_slot;
        let mut var_delta_t_dn9: f64 = *var_delta_t_dn9_slot;
        let mut var_delta_t_rv: f64 = *var_delta_t_rv_slot;
        let mut var_guard451: f64 = *var_guard451_slot;
        let mut var_guard451_rv: f64 = *var_guard451_rv_slot;
        let mut var_guard452: f64 = *var_guard452_slot;
        let mut var_guard452_rv: f64 = *var_guard452_rv_slot;
        let mut var_guard453: f64 = *var_guard453_slot;
        let mut var_guard453_rv: f64 = *var_guard453_rv_slot;
        let mut var_guard454: f64 = *var_guard454_slot;
        let mut var_guard454_rv: f64 = *var_guard454_rv_slot;
        let mut var_guard455: f64 = *var_guard455_slot;
        let mut var_guard455_rv: f64 = *var_guard455_rv_slot;
        let mut var_k0_t: f64 = *var_k0_t_slot;
        let mut var_k0_t_dn4: f64 = *var_k0_t_dn4_slot;
        let mut var_k0_t_dn5: f64 = *var_k0_t_dn5_slot;
        let mut var_k0_t_rv: f64 = *var_k0_t_rv_slot;
        let mut var_m0_t: f64 = *var_m0_t_slot;
        let mut var_m0_t_dn4: f64 = *var_m0_t_dn4_slot;
        let mut var_m0_t_dn5: f64 = *var_m0_t_dn5_slot;
        let mut var_m0_t_rv: f64 = *var_m0_t_rv_slot;
        let mut var_ptwg_t: f64 = *var_ptwg_t_slot;
        let mut var_ptwg_t_dn10: f64 = *var_ptwg_t_dn10_slot;
        let mut var_ptwg_t_dn11: f64 = *var_ptwg_t_dn11_slot;
        let mut var_ptwg_t_dn3: f64 = *var_ptwg_t_dn3_slot;
        let mut var_ptwg_t_dn4: f64 = *var_ptwg_t_dn4_slot;
        let mut var_ptwg_t_dn5: f64 = *var_ptwg_t_dn5_slot;
        let mut var_ptwg_t_dn6: f64 = *var_ptwg_t_dn6_slot;
        let mut var_ptwg_t_dn7: f64 = *var_ptwg_t_dn7_slot;
        let mut var_ptwg_t_dn8: f64 = *var_ptwg_t_dn8_slot;
        let mut var_ptwg_t_dn9: f64 = *var_ptwg_t_dn9_slot;
        let mut var_ptwg_t_rv: f64 = *var_ptwg_t_rv_slot;
        let mut var_ptwgr_t: f64 = *var_ptwgr_t_slot;
        let mut var_ptwgr_t_dn10: f64 = *var_ptwgr_t_dn10_slot;
        let mut var_ptwgr_t_dn11: f64 = *var_ptwgr_t_dn11_slot;
        let mut var_ptwgr_t_dn3: f64 = *var_ptwgr_t_dn3_slot;
        let mut var_ptwgr_t_dn4: f64 = *var_ptwgr_t_dn4_slot;
        let mut var_ptwgr_t_dn5: f64 = *var_ptwgr_t_dn5_slot;
        let mut var_ptwgr_t_dn6: f64 = *var_ptwgr_t_dn6_slot;
        let mut var_ptwgr_t_dn7: f64 = *var_ptwgr_t_dn7_slot;
        let mut var_ptwgr_t_dn8: f64 = *var_ptwgr_t_dn8_slot;
        let mut var_ptwgr_t_dn9: f64 = *var_ptwgr_t_dn9_slot;
        let mut var_ptwgr_t_rv: f64 = *var_ptwgr_t_rv_slot;
        let mut var_rdstemp: f64 = *var_rdstemp_slot;
        let mut var_rdstemp_dn4: f64 = *var_rdstemp_dn4_slot;
        let mut var_rdstemp_dn5: f64 = *var_rdstemp_dn5_slot;
        let mut var_rdstemp_rv: f64 = *var_rdstemp_rv_slot;
        let mut var_uar_t: f64 = *var_uar_t_slot;
        let mut var_uar_t_dn10: f64 = *var_uar_t_dn10_slot;
        let mut var_uar_t_dn11: f64 = *var_uar_t_dn11_slot;
        let mut var_uar_t_dn3: f64 = *var_uar_t_dn3_slot;
        let mut var_uar_t_dn4: f64 = *var_uar_t_dn4_slot;
        let mut var_uar_t_dn5: f64 = *var_uar_t_dn5_slot;
        let mut var_uar_t_dn6: f64 = *var_uar_t_dn6_slot;
        let mut var_uar_t_dn7: f64 = *var_uar_t_dn7_slot;
        let mut var_uar_t_dn8: f64 = *var_uar_t_dn8_slot;
        let mut var_uar_t_dn9: f64 = *var_uar_t_dn9_slot;
        let mut var_uar_t_rv: f64 = *var_uar_t_rv_slot;
        let mut var_ucr_t: f64 = *var_ucr_t_slot;
        let mut var_ucr_t_dn10: f64 = *var_ucr_t_dn10_slot;
        let mut var_ucr_t_dn11: f64 = *var_ucr_t_dn11_slot;
        let mut var_ucr_t_dn3: f64 = *var_ucr_t_dn3_slot;
        let mut var_ucr_t_dn4: f64 = *var_ucr_t_dn4_slot;
        let mut var_ucr_t_dn5: f64 = *var_ucr_t_dn5_slot;
        let mut var_ucr_t_dn6: f64 = *var_ucr_t_dn6_slot;
        let mut var_ucr_t_dn7: f64 = *var_ucr_t_dn7_slot;
        let mut var_ucr_t_dn8: f64 = *var_ucr_t_dn8_slot;
        let mut var_ucr_t_dn9: f64 = *var_ucr_t_dn9_slot;
        let mut var_ucr_t_rv: f64 = *var_ucr_t_rv_slot;
        let mut var_ucsr_t: f64 = *var_ucsr_t_slot;
        let mut var_ucsr_t_dn4: f64 = *var_ucsr_t_dn4_slot;
        let mut var_ucsr_t_dn5: f64 = *var_ucsr_t_dn5_slot;
        let mut var_ucsr_t_rv: f64 = *var_ucsr_t_rv_slot;
        let mut var_udr_t: f64 = *var_udr_t_slot;
        let mut var_udr_t_dn10: f64 = *var_udr_t_dn10_slot;
        let mut var_udr_t_dn11: f64 = *var_udr_t_dn11_slot;
        let mut var_udr_t_dn3: f64 = *var_udr_t_dn3_slot;
        let mut var_udr_t_dn4: f64 = *var_udr_t_dn4_slot;
        let mut var_udr_t_dn5: f64 = *var_udr_t_dn5_slot;
        let mut var_udr_t_dn6: f64 = *var_udr_t_dn6_slot;
        let mut var_udr_t_dn7: f64 = *var_udr_t_dn7_slot;
        let mut var_udr_t_dn8: f64 = *var_udr_t_dn8_slot;
        let mut var_udr_t_dn9: f64 = *var_udr_t_dn9_slot;
        let mut var_udr_t_rv: f64 = *var_udr_t_rv_slot;
        let mut var_vsat_t: f64 = *var_vsat_t_slot;
        let mut var_vsat_t_dn10: f64 = *var_vsat_t_dn10_slot;
        let mut var_vsat_t_dn11: f64 = *var_vsat_t_dn11_slot;
        let mut var_vsat_t_dn3: f64 = *var_vsat_t_dn3_slot;
        let mut var_vsat_t_dn4: f64 = *var_vsat_t_dn4_slot;
        let mut var_vsat_t_dn5: f64 = *var_vsat_t_dn5_slot;
        let mut var_vsat_t_dn6: f64 = *var_vsat_t_dn6_slot;
        let mut var_vsat_t_dn7: f64 = *var_vsat_t_dn7_slot;
        let mut var_vsat_t_dn8: f64 = *var_vsat_t_dn8_slot;
        let mut var_vsat_t_dn9: f64 = *var_vsat_t_dn9_slot;
        let mut var_vsat_t_rv: f64 = *var_vsat_t_rv_slot;
        let mut var_vsatcv_t: f64 = *var_vsatcv_t_slot;
        let mut var_vsatcv_t_dn10: f64 = *var_vsatcv_t_dn10_slot;
        let mut var_vsatcv_t_dn11: f64 = *var_vsatcv_t_dn11_slot;
        let mut var_vsatcv_t_dn3: f64 = *var_vsatcv_t_dn3_slot;
        let mut var_vsatcv_t_dn4: f64 = *var_vsatcv_t_dn4_slot;
        let mut var_vsatcv_t_dn5: f64 = *var_vsatcv_t_dn5_slot;
        let mut var_vsatcv_t_dn6: f64 = *var_vsatcv_t_dn6_slot;
        let mut var_vsatcv_t_dn7: f64 = *var_vsatcv_t_dn7_slot;
        let mut var_vsatcv_t_dn8: f64 = *var_vsatcv_t_dn8_slot;
        let mut var_vsatcv_t_dn9: f64 = *var_vsatcv_t_dn9_slot;
        let mut var_vsatcv_t_rv: f64 = *var_vsatcv_t_rv_slot;
        let mut var_vsatr_t: f64 = *var_vsatr_t_slot;
        let mut var_vsatr_t_dn10: f64 = *var_vsatr_t_dn10_slot;
        let mut var_vsatr_t_dn11: f64 = *var_vsatr_t_dn11_slot;
        let mut var_vsatr_t_dn3: f64 = *var_vsatr_t_dn3_slot;
        let mut var_vsatr_t_dn4: f64 = *var_vsatr_t_dn4_slot;
        let mut var_vsatr_t_dn5: f64 = *var_vsatr_t_dn5_slot;
        let mut var_vsatr_t_dn6: f64 = *var_vsatr_t_dn6_slot;
        let mut var_vsatr_t_dn7: f64 = *var_vsatr_t_dn7_slot;
        let mut var_vsatr_t_dn8: f64 = *var_vsatr_t_dn8_slot;
        let mut var_vsatr_t_dn9: f64 = *var_vsatr_t_dn9_slot;
        let mut var_vsatr_t_rv: f64 = *var_vsatr_t_rv_slot;

        let (assign13830_e19033, assign13830_e19033_d_n3, assign13830_e19033_d_n4, assign13830_e19033_d_n5, assign13830_e19033_d_n6, assign13830_e19033_d_n7, assign13830_e19033_d_n8, assign13830_e19033_d_n9, assign13830_e19033_d_n10, assign13830_e19033_d_n11,) = {
    if (var_guard450 != 0.0) {
        let assign13830_e19003: f64 = (var_ua1_i * var_deltemp);
        let assign13830_e19004: f64 = (1.0 + assign13830_e19003);
        let assign13830_e19006: f64 = (assign13830_e19004 - 1e-6);
        let assign13830_e19010: f64 = (var_ua1_i * var_deltemp);
        let assign13830_e19011: f64 = (1.0 + assign13830_e19010);
        let assign13830_e19013: f64 = (assign13830_e19011 - 1e-6);
        let assign13830_e19017: f64 = (var_ua1_i * var_deltemp);
        let assign13830_e19018: f64 = (1.0 + assign13830_e19017);
        let assign13830_e19020: f64 = (assign13830_e19018 - 1e-6);
        let assign13830_e19021: f64 = (assign13830_e19013 * assign13830_e19020);
        let assign13830_e19024: f64 = (4.0 * 0.001);
        let assign13830_e19026: f64 = (assign13830_e19024 * 0.001);
        let assign13830_e19027: f64 = (assign13830_e19021 + assign13830_e19026);
        let assign13830_e19028: f64 = (assign13830_e19027).sqrt();
        let assign13830_e19029: f64 = (assign13830_e19006 + assign13830_e19028);
        let assign13830_e19030: f64 = (0.5 * assign13830_e19029);
        let assign13830_e19031: f64 = (var_uar_i * assign13830_e19030);
        (assign13830_e19031, (var_uar_i_dn3 * assign13830_e19030), ((var_uar_i_dn4 * assign13830_e19030) + (var_uar_i * (0.5 * ((var_ua1_i * var_deltemp_dn4) + ((((var_ua1_i * var_deltemp_dn4) * assign13830_e19020) + (assign13830_e19013 * (var_ua1_i * var_deltemp_dn4))) / (2.0 * assign13830_e19028)))))), ((var_uar_i_dn5 * assign13830_e19030) + (var_uar_i * (0.5 * ((var_ua1_i * var_deltemp_dn5) + ((((var_ua1_i * var_deltemp_dn5) * assign13830_e19020) + (assign13830_e19013 * (var_ua1_i * var_deltemp_dn5))) / (2.0 * assign13830_e19028)))))), (var_uar_i_dn6 * assign13830_e19030), (var_uar_i_dn7 * assign13830_e19030), (var_uar_i_dn8 * assign13830_e19030), (var_uar_i_dn9 * assign13830_e19030), (var_uar_i_dn10 * assign13830_e19030), (var_uar_i_dn11 * assign13830_e19030),)
    } else {
        (var_uar_t, var_uar_t_dn3, var_uar_t_dn4, var_uar_t_dn5, var_uar_t_dn6, var_uar_t_dn7, var_uar_t_dn8, var_uar_t_dn9, var_uar_t_dn10, var_uar_t_dn11,)
    }
};
        var_uar_t = assign13830_e19033;
        var_uar_t_dn3 = assign13830_e19033_d_n3;
        var_uar_t_dn4 = assign13830_e19033_d_n4;
        var_uar_t_dn5 = assign13830_e19033_d_n5;
        var_uar_t_dn6 = assign13830_e19033_d_n6;
        var_uar_t_dn7 = assign13830_e19033_d_n7;
        var_uar_t_dn8 = assign13830_e19033_d_n8;
        var_uar_t_dn9 = assign13830_e19033_d_n9;
        var_uar_t_dn10 = assign13830_e19033_d_n10;
        var_uar_t_dn11 = assign13830_e19033_d_n11;
        var_uar_t_rv = 0.0;

        let (assign13840_e19070, assign13840_e19070_d_n3, assign13840_e19070_d_n4, assign13840_e19070_d_n5, assign13840_e19070_d_n6, assign13840_e19070_d_n7, assign13840_e19070_d_n8, assign13840_e19070_d_n9, assign13840_e19070_d_n10, assign13840_e19070_d_n11,) = {
    if (var_guard450 != 0.0) {
        let assign13840_e19040: f64 = (var_uc1_i * var_deltemp);
        let assign13840_e19041: f64 = (1.0 + assign13840_e19040);
        let assign13840_e19043: f64 = (assign13840_e19041 - 1e-6);
        let assign13840_e19047: f64 = (var_uc1_i * var_deltemp);
        let assign13840_e19048: f64 = (1.0 + assign13840_e19047);
        let assign13840_e19050: f64 = (assign13840_e19048 - 1e-6);
        let assign13840_e19054: f64 = (var_uc1_i * var_deltemp);
        let assign13840_e19055: f64 = (1.0 + assign13840_e19054);
        let assign13840_e19057: f64 = (assign13840_e19055 - 1e-6);
        let assign13840_e19058: f64 = (assign13840_e19050 * assign13840_e19057);
        let assign13840_e19061: f64 = (4.0 * 0.001);
        let assign13840_e19063: f64 = (assign13840_e19061 * 0.001);
        let assign13840_e19064: f64 = (assign13840_e19058 + assign13840_e19063);
        let assign13840_e19065: f64 = (assign13840_e19064).sqrt();
        let assign13840_e19066: f64 = (assign13840_e19043 + assign13840_e19065);
        let assign13840_e19067: f64 = (0.5 * assign13840_e19066);
        let assign13840_e19068: f64 = (var_ucr_i * assign13840_e19067);
        (assign13840_e19068, (var_ucr_i_dn3 * assign13840_e19067), ((var_ucr_i_dn4 * assign13840_e19067) + (var_ucr_i * (0.5 * ((var_uc1_i * var_deltemp_dn4) + ((((var_uc1_i * var_deltemp_dn4) * assign13840_e19057) + (assign13840_e19050 * (var_uc1_i * var_deltemp_dn4))) / (2.0 * assign13840_e19065)))))), ((var_ucr_i_dn5 * assign13840_e19067) + (var_ucr_i * (0.5 * ((var_uc1_i * var_deltemp_dn5) + ((((var_uc1_i * var_deltemp_dn5) * assign13840_e19057) + (assign13840_e19050 * (var_uc1_i * var_deltemp_dn5))) / (2.0 * assign13840_e19065)))))), (var_ucr_i_dn6 * assign13840_e19067), (var_ucr_i_dn7 * assign13840_e19067), (var_ucr_i_dn8 * assign13840_e19067), (var_ucr_i_dn9 * assign13840_e19067), (var_ucr_i_dn10 * assign13840_e19067), (var_ucr_i_dn11 * assign13840_e19067),)
    } else {
        (var_ucr_t, var_ucr_t_dn3, var_ucr_t_dn4, var_ucr_t_dn5, var_ucr_t_dn6, var_ucr_t_dn7, var_ucr_t_dn8, var_ucr_t_dn9, var_ucr_t_dn10, var_ucr_t_dn11,)
    }
};
        var_ucr_t = assign13840_e19070;
        var_ucr_t_dn3 = assign13840_e19070_d_n3;
        var_ucr_t_dn4 = assign13840_e19070_d_n4;
        var_ucr_t_dn5 = assign13840_e19070_d_n5;
        var_ucr_t_dn6 = assign13840_e19070_d_n6;
        var_ucr_t_dn7 = assign13840_e19070_d_n7;
        var_ucr_t_dn8 = assign13840_e19070_d_n8;
        var_ucr_t_dn9 = assign13840_e19070_d_n9;
        var_ucr_t_dn10 = assign13840_e19070_d_n10;
        var_ucr_t_dn11 = assign13840_e19070_d_n11;
        var_ucr_t_rv = 0.0;

        let (assign13850_e19078, assign13850_e19078_d_n3, assign13850_e19078_d_n4, assign13850_e19078_d_n5, assign13850_e19078_d_n6, assign13850_e19078_d_n7, assign13850_e19078_d_n8, assign13850_e19078_d_n9, assign13850_e19078_d_n10, assign13850_e19078_d_n11,) = {
    if (var_guard450 != 0.0) {
        let assign13850_e19075: f64 = (var_tratio).powf(var_ud1_i);
        let assign13850_e19076: f64 = (var_udr_i * assign13850_e19075);
        (assign13850_e19076, (var_udr_i_dn3 * assign13850_e19075), ((var_udr_i_dn4 * assign13850_e19075) + (var_udr_i * if 0.0 == 0.0 && ((var_ud1_i) as f64).is_finite() && ((var_ud1_i) as f64).fract() == 0.0 { if var_ud1_i == 0.0 { 0.0 } else { (var_ud1_i * ((var_tratio).powf(var_ud1_i - 1.0) * var_tratio_dn4)) } } else { (assign13850_e19075 * (var_ud1_i * (var_tratio_dn4 / var_tratio))) })), ((var_udr_i_dn5 * assign13850_e19075) + (var_udr_i * if 0.0 == 0.0 && ((var_ud1_i) as f64).is_finite() && ((var_ud1_i) as f64).fract() == 0.0 { if var_ud1_i == 0.0 { 0.0 } else { (var_ud1_i * ((var_tratio).powf(var_ud1_i - 1.0) * var_tratio_dn5)) } } else { (assign13850_e19075 * (var_ud1_i * (var_tratio_dn5 / var_tratio))) })), (var_udr_i_dn6 * assign13850_e19075), (var_udr_i_dn7 * assign13850_e19075), (var_udr_i_dn8 * assign13850_e19075), (var_udr_i_dn9 * assign13850_e19075), (var_udr_i_dn10 * assign13850_e19075), (var_udr_i_dn11 * assign13850_e19075),)
    } else {
        (var_udr_t, var_udr_t_dn3, var_udr_t_dn4, var_udr_t_dn5, var_udr_t_dn6, var_udr_t_dn7, var_udr_t_dn8, var_udr_t_dn9, var_udr_t_dn10, var_udr_t_dn11,)
    }
};
        var_udr_t = assign13850_e19078;
        var_udr_t_dn3 = assign13850_e19078_d_n3;
        var_udr_t_dn4 = assign13850_e19078_d_n4;
        var_udr_t_dn5 = assign13850_e19078_d_n5;
        var_udr_t_dn6 = assign13850_e19078_d_n6;
        var_udr_t_dn7 = assign13850_e19078_d_n7;
        var_udr_t_dn8 = assign13850_e19078_d_n8;
        var_udr_t_dn9 = assign13850_e19078_d_n9;
        var_udr_t_dn10 = assign13850_e19078_d_n10;
        var_udr_t_dn11 = assign13850_e19078_d_n11;
        var_udr_t_rv = 0.0;

        let (assign13860_e19086, assign13860_e19086_d_n4, assign13860_e19086_d_n5,) = {
    if (var_guard450 != 0.0) {
        let assign13860_e19083: f64 = (var_tratio).powf(var_ucste_i);
        let assign13860_e19084: f64 = (var_ucsr_i * assign13860_e19083);
        (assign13860_e19084, (var_ucsr_i * if 0.0 == 0.0 && ((var_ucste_i) as f64).is_finite() && ((var_ucste_i) as f64).fract() == 0.0 { if var_ucste_i == 0.0 { 0.0 } else { (var_ucste_i * ((var_tratio).powf(var_ucste_i - 1.0) * var_tratio_dn4)) } } else { (assign13860_e19083 * (var_ucste_i * (var_tratio_dn4 / var_tratio))) }), (var_ucsr_i * if 0.0 == 0.0 && ((var_ucste_i) as f64).is_finite() && ((var_ucste_i) as f64).fract() == 0.0 { if var_ucste_i == 0.0 { 0.0 } else { (var_ucste_i * ((var_tratio).powf(var_ucste_i - 1.0) * var_tratio_dn5)) } } else { (assign13860_e19083 * (var_ucste_i * (var_tratio_dn5 / var_tratio))) }),)
    } else {
        (var_ucsr_t, var_ucsr_t_dn4, var_ucsr_t_dn5,)
    }
};
        var_ucsr_t = assign13860_e19086;
        var_ucsr_t_dn4 = assign13860_e19086_d_n4;
        var_ucsr_t_dn5 = assign13860_e19086_d_n5;
        var_ucsr_t_rv = 0.0;

        let assign13870_e19089: f64 = (var_tratio).powf(var_prt_i);
        var_rdstemp = assign13870_e19089;
        var_rdstemp_dn4 = if 0.0 == 0.0 && ((var_prt_i) as f64).is_finite() && ((var_prt_i) as f64).fract() == 0.0 { if var_prt_i == 0.0 { 0.0 } else { (var_prt_i * ((var_tratio).powf(var_prt_i - 1.0) * var_tratio_dn4)) } } else { (assign13870_e19089 * (var_prt_i * (var_tratio_dn4 / var_tratio))) };
        var_rdstemp_dn5 = if 0.0 == 0.0 && ((var_prt_i) as f64).is_finite() && ((var_prt_i) as f64).fract() == 0.0 { if var_prt_i == 0.0 { 0.0 } else { (var_prt_i * ((var_tratio).powf(var_prt_i - 1.0) * var_tratio_dn5)) } } else { (assign13870_e19089 * (var_prt_i * (var_tratio_dn5 / var_tratio))) };
        var_rdstemp_rv = 0.0;

        let assign13880_e19093: f64 = (-var_at_i);
        let assign13880_e19094: f64 = (var_tratio).powf(assign13880_e19093);
        let assign13880_e19095: f64 = (var_vsat_i * assign13880_e19094);
        var_vsat_t = assign13880_e19095;
        var_vsat_t_dn3 = (var_vsat_i_dn3 * assign13880_e19094);
        var_vsat_t_dn4 = ((var_vsat_i_dn4 * assign13880_e19094) + (var_vsat_i * if 0.0 == 0.0 && ((assign13880_e19093) as f64).is_finite() && ((assign13880_e19093) as f64).fract() == 0.0 { if assign13880_e19093 == 0.0 { 0.0 } else { (assign13880_e19093 * ((var_tratio).powf(assign13880_e19093 - 1.0) * var_tratio_dn4)) } } else { (assign13880_e19094 * (assign13880_e19093 * (var_tratio_dn4 / var_tratio))) }));
        var_vsat_t_dn5 = ((var_vsat_i_dn5 * assign13880_e19094) + (var_vsat_i * if 0.0 == 0.0 && ((assign13880_e19093) as f64).is_finite() && ((assign13880_e19093) as f64).fract() == 0.0 { if assign13880_e19093 == 0.0 { 0.0 } else { (assign13880_e19093 * ((var_tratio).powf(assign13880_e19093 - 1.0) * var_tratio_dn5)) } } else { (assign13880_e19094 * (assign13880_e19093 * (var_tratio_dn5 / var_tratio))) }));
        var_vsat_t_dn6 = (var_vsat_i_dn6 * assign13880_e19094);
        var_vsat_t_dn7 = (var_vsat_i_dn7 * assign13880_e19094);
        var_vsat_t_dn8 = (var_vsat_i_dn8 * assign13880_e19094);
        var_vsat_t_dn9 = (var_vsat_i_dn9 * assign13880_e19094);
        var_vsat_t_dn10 = (var_vsat_i_dn10 * assign13880_e19094);
        var_vsat_t_dn11 = (var_vsat_i_dn11 * assign13880_e19094);
        var_vsat_t_rv = 0.0;

        let assign13890_e19098: f64 = if var_vsat_t < 100.0 { 1.0 } else { 0.0 };
        var_guard451 = assign13890_e19098;
        var_guard451_rv = 0.0;

        let (assign13900_e19102, assign13900_e19102_d_n3, assign13900_e19102_d_n4, assign13900_e19102_d_n5, assign13900_e19102_d_n6, assign13900_e19102_d_n7, assign13900_e19102_d_n8, assign13900_e19102_d_n9, assign13900_e19102_d_n10, assign13900_e19102_d_n11,) = {
    if (var_guard451 != 0.0) {
        (100.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_vsat_t, var_vsat_t_dn3, var_vsat_t_dn4, var_vsat_t_dn5, var_vsat_t_dn6, var_vsat_t_dn7, var_vsat_t_dn8, var_vsat_t_dn9, var_vsat_t_dn10, var_vsat_t_dn11,)
    }
};
        var_vsat_t = assign13900_e19102;
        var_vsat_t_dn3 = assign13900_e19102_d_n3;
        var_vsat_t_dn4 = assign13900_e19102_d_n4;
        var_vsat_t_dn5 = assign13900_e19102_d_n5;
        var_vsat_t_dn6 = assign13900_e19102_d_n6;
        var_vsat_t_dn7 = assign13900_e19102_d_n7;
        var_vsat_t_dn8 = assign13900_e19102_d_n8;
        var_vsat_t_dn9 = assign13900_e19102_d_n9;
        var_vsat_t_dn10 = assign13900_e19102_d_n10;
        var_vsat_t_dn11 = assign13900_e19102_d_n11;
        var_vsat_t_rv = 0.0;

        let assign13910_e19105: f64 = if p.p35 != 0.0 { 1.0 } else { 0.0 };
        var_guard452 = assign13910_e19105;
        var_guard452_rv = 0.0;

        let (assign13920_e19114, assign13920_e19114_d_n3, assign13920_e19114_d_n4, assign13920_e19114_d_n5, assign13920_e19114_d_n6, assign13920_e19114_d_n7, assign13920_e19114_d_n8, assign13920_e19114_d_n9, assign13920_e19114_d_n10, assign13920_e19114_d_n11,) = {
    if (var_guard452 != 0.0) {
        let assign13920_e19110: f64 = (-var_at_i);
        let assign13920_e19111: f64 = (var_tratio).powf(assign13920_e19110);
        let assign13920_e19112: f64 = (var_vsatr_i * assign13920_e19111);
        (assign13920_e19112, (var_vsatr_i_dn3 * assign13920_e19111), ((var_vsatr_i_dn4 * assign13920_e19111) + (var_vsatr_i * if 0.0 == 0.0 && ((assign13920_e19110) as f64).is_finite() && ((assign13920_e19110) as f64).fract() == 0.0 { if assign13920_e19110 == 0.0 { 0.0 } else { (assign13920_e19110 * ((var_tratio).powf(assign13920_e19110 - 1.0) * var_tratio_dn4)) } } else { (assign13920_e19111 * (assign13920_e19110 * (var_tratio_dn4 / var_tratio))) })), ((var_vsatr_i_dn5 * assign13920_e19111) + (var_vsatr_i * if 0.0 == 0.0 && ((assign13920_e19110) as f64).is_finite() && ((assign13920_e19110) as f64).fract() == 0.0 { if assign13920_e19110 == 0.0 { 0.0 } else { (assign13920_e19110 * ((var_tratio).powf(assign13920_e19110 - 1.0) * var_tratio_dn5)) } } else { (assign13920_e19111 * (assign13920_e19110 * (var_tratio_dn5 / var_tratio))) })), (var_vsatr_i_dn6 * assign13920_e19111), (var_vsatr_i_dn7 * assign13920_e19111), (var_vsatr_i_dn8 * assign13920_e19111), (var_vsatr_i_dn9 * assign13920_e19111), (var_vsatr_i_dn10 * assign13920_e19111), (var_vsatr_i_dn11 * assign13920_e19111),)
    } else {
        (var_vsatr_t, var_vsatr_t_dn3, var_vsatr_t_dn4, var_vsatr_t_dn5, var_vsatr_t_dn6, var_vsatr_t_dn7, var_vsatr_t_dn8, var_vsatr_t_dn9, var_vsatr_t_dn10, var_vsatr_t_dn11,)
    }
};
        var_vsatr_t = assign13920_e19114;
        var_vsatr_t_dn3 = assign13920_e19114_d_n3;
        var_vsatr_t_dn4 = assign13920_e19114_d_n4;
        var_vsatr_t_dn5 = assign13920_e19114_d_n5;
        var_vsatr_t_dn6 = assign13920_e19114_d_n6;
        var_vsatr_t_dn7 = assign13920_e19114_d_n7;
        var_vsatr_t_dn8 = assign13920_e19114_d_n8;
        var_vsatr_t_dn9 = assign13920_e19114_d_n9;
        var_vsatr_t_dn10 = assign13920_e19114_d_n10;
        var_vsatr_t_dn11 = assign13920_e19114_d_n11;
        var_vsatr_t_rv = 0.0;

        let assign13930_e19117: f64 = if var_vsatr_t < 100.0 { 1.0 } else { 0.0 };
        var_guard453 = assign13930_e19117;
        var_guard453_rv = 0.0;

        let (assign13940_e19123, assign13940_e19123_d_n3, assign13940_e19123_d_n4, assign13940_e19123_d_n5, assign13940_e19123_d_n6, assign13940_e19123_d_n7, assign13940_e19123_d_n8, assign13940_e19123_d_n9, assign13940_e19123_d_n10, assign13940_e19123_d_n11,) = {
    if ((var_guard452 != 0.0) && (var_guard453 != 0.0)) {
        (100.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_vsatr_t, var_vsatr_t_dn3, var_vsatr_t_dn4, var_vsatr_t_dn5, var_vsatr_t_dn6, var_vsatr_t_dn7, var_vsatr_t_dn8, var_vsatr_t_dn9, var_vsatr_t_dn10, var_vsatr_t_dn11,)
    }
};
        var_vsatr_t = assign13940_e19123;
        var_vsatr_t_dn3 = assign13940_e19123_d_n3;
        var_vsatr_t_dn4 = assign13940_e19123_d_n4;
        var_vsatr_t_dn5 = assign13940_e19123_d_n5;
        var_vsatr_t_dn6 = assign13940_e19123_d_n6;
        var_vsatr_t_dn7 = assign13940_e19123_d_n7;
        var_vsatr_t_dn8 = assign13940_e19123_d_n8;
        var_vsatr_t_dn9 = assign13940_e19123_d_n9;
        var_vsatr_t_dn10 = assign13940_e19123_d_n10;
        var_vsatr_t_dn11 = assign13940_e19123_d_n11;
        var_vsatr_t_rv = 0.0;

        let assign13950_e19127: f64 = (-var_at_i);
        let assign13950_e19128: f64 = (var_tratio).powf(assign13950_e19127);
        let assign13950_e19129: f64 = (var_vsatcv_i * assign13950_e19128);
        var_vsatcv_t = assign13950_e19129;
        var_vsatcv_t_dn3 = (var_vsatcv_i_dn3 * assign13950_e19128);
        var_vsatcv_t_dn4 = ((var_vsatcv_i_dn4 * assign13950_e19128) + (var_vsatcv_i * if 0.0 == 0.0 && ((assign13950_e19127) as f64).is_finite() && ((assign13950_e19127) as f64).fract() == 0.0 { if assign13950_e19127 == 0.0 { 0.0 } else { (assign13950_e19127 * ((var_tratio).powf(assign13950_e19127 - 1.0) * var_tratio_dn4)) } } else { (assign13950_e19128 * (assign13950_e19127 * (var_tratio_dn4 / var_tratio))) }));
        var_vsatcv_t_dn5 = ((var_vsatcv_i_dn5 * assign13950_e19128) + (var_vsatcv_i * if 0.0 == 0.0 && ((assign13950_e19127) as f64).is_finite() && ((assign13950_e19127) as f64).fract() == 0.0 { if assign13950_e19127 == 0.0 { 0.0 } else { (assign13950_e19127 * ((var_tratio).powf(assign13950_e19127 - 1.0) * var_tratio_dn5)) } } else { (assign13950_e19128 * (assign13950_e19127 * (var_tratio_dn5 / var_tratio))) }));
        var_vsatcv_t_dn6 = (var_vsatcv_i_dn6 * assign13950_e19128);
        var_vsatcv_t_dn7 = (var_vsatcv_i_dn7 * assign13950_e19128);
        var_vsatcv_t_dn8 = (var_vsatcv_i_dn8 * assign13950_e19128);
        var_vsatcv_t_dn9 = (var_vsatcv_i_dn9 * assign13950_e19128);
        var_vsatcv_t_dn10 = (var_vsatcv_i_dn10 * assign13950_e19128);
        var_vsatcv_t_dn11 = (var_vsatcv_i_dn11 * assign13950_e19128);
        var_vsatcv_t_rv = 0.0;

        let assign13960_e19132: f64 = if var_vsatcv_t < 100.0 { 1.0 } else { 0.0 };
        var_guard454 = assign13960_e19132;
        var_guard454_rv = 0.0;

        let (assign13970_e19136, assign13970_e19136_d_n3, assign13970_e19136_d_n4, assign13970_e19136_d_n5, assign13970_e19136_d_n6, assign13970_e19136_d_n7, assign13970_e19136_d_n8, assign13970_e19136_d_n9, assign13970_e19136_d_n10, assign13970_e19136_d_n11,) = {
    if (var_guard454 != 0.0) {
        (100.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_vsatcv_t, var_vsatcv_t_dn3, var_vsatcv_t_dn4, var_vsatcv_t_dn5, var_vsatcv_t_dn6, var_vsatcv_t_dn7, var_vsatcv_t_dn8, var_vsatcv_t_dn9, var_vsatcv_t_dn10, var_vsatcv_t_dn11,)
    }
};
        var_vsatcv_t = assign13970_e19136;
        var_vsatcv_t_dn3 = assign13970_e19136_d_n3;
        var_vsatcv_t_dn4 = assign13970_e19136_d_n4;
        var_vsatcv_t_dn5 = assign13970_e19136_d_n5;
        var_vsatcv_t_dn6 = assign13970_e19136_d_n6;
        var_vsatcv_t_dn7 = assign13970_e19136_d_n7;
        var_vsatcv_t_dn8 = assign13970_e19136_d_n8;
        var_vsatcv_t_dn9 = assign13970_e19136_d_n9;
        var_vsatcv_t_dn10 = assign13970_e19136_d_n10;
        var_vsatcv_t_dn11 = assign13970_e19136_d_n11;
        var_vsatcv_t_rv = 0.0;

        let assign13980_e19141: f64 = (1.0 / var_delta_i);
        let assign13980_e19145: f64 = (p.p1069 * var_deltemp);
        let assign13980_e19146: f64 = (1.0 + assign13980_e19145);
        let assign13980_e19147: f64 = (assign13980_e19141 * assign13980_e19146);
        let assign13980_e19149: f64 = (assign13980_e19147 - 2.0);
        let assign13980_e19152: f64 = (1.0 / var_delta_i);
        let assign13980_e19156: f64 = (p.p1069 * var_deltemp);
        let assign13980_e19157: f64 = (1.0 + assign13980_e19156);
        let assign13980_e19158: f64 = (assign13980_e19152 * assign13980_e19157);
        let assign13980_e19160: f64 = (assign13980_e19158 - 2.0);
        let assign13980_e19163: f64 = (1.0 / var_delta_i);
        let assign13980_e19167: f64 = (p.p1069 * var_deltemp);
        let assign13980_e19168: f64 = (1.0 + assign13980_e19167);
        let assign13980_e19169: f64 = (assign13980_e19163 * assign13980_e19168);
        let assign13980_e19171: f64 = (assign13980_e19169 - 2.0);
        let assign13980_e19172: f64 = (assign13980_e19160 * assign13980_e19171);
        let assign13980_e19175: f64 = (4.0 * 0.001);
        let assign13980_e19177: f64 = (assign13980_e19175 * 0.001);
        let assign13980_e19178: f64 = (assign13980_e19172 + assign13980_e19177);
        let assign13980_e19179: f64 = (assign13980_e19178).sqrt();
        let assign13980_e19180: f64 = (assign13980_e19149 + assign13980_e19179);
        let assign13980_e19181: f64 = (0.5 * assign13980_e19180);
        let assign13980_e19183: f64 = (assign13980_e19181 + 2.0);
        let assign13980_e19184: f64 = (1.0 / assign13980_e19183);
        var_delta_t = assign13980_e19184;
        var_delta_t_dn3 = (-((0.5 * (((-(var_delta_i_dn3 / (var_delta_i * var_delta_i))) * assign13980_e19146) + (((((-(var_delta_i_dn3 / (var_delta_i * var_delta_i))) * assign13980_e19157) * assign13980_e19171) + (assign13980_e19160 * ((-(var_delta_i_dn3 / (var_delta_i * var_delta_i))) * assign13980_e19168))) / (2.0 * assign13980_e19179)))) / (assign13980_e19183 * assign13980_e19183)));
        var_delta_t_dn4 = (-((0.5 * ((((-(var_delta_i_dn4 / (var_delta_i * var_delta_i))) * assign13980_e19146) + (assign13980_e19141 * (p.p1069 * var_deltemp_dn4))) + ((((((-(var_delta_i_dn4 / (var_delta_i * var_delta_i))) * assign13980_e19157) + (assign13980_e19152 * (p.p1069 * var_deltemp_dn4))) * assign13980_e19171) + (assign13980_e19160 * (((-(var_delta_i_dn4 / (var_delta_i * var_delta_i))) * assign13980_e19168) + (assign13980_e19163 * (p.p1069 * var_deltemp_dn4))))) / (2.0 * assign13980_e19179)))) / (assign13980_e19183 * assign13980_e19183)));
        var_delta_t_dn5 = (-((0.5 * ((((-(var_delta_i_dn5 / (var_delta_i * var_delta_i))) * assign13980_e19146) + (assign13980_e19141 * (p.p1069 * var_deltemp_dn5))) + ((((((-(var_delta_i_dn5 / (var_delta_i * var_delta_i))) * assign13980_e19157) + (assign13980_e19152 * (p.p1069 * var_deltemp_dn5))) * assign13980_e19171) + (assign13980_e19160 * (((-(var_delta_i_dn5 / (var_delta_i * var_delta_i))) * assign13980_e19168) + (assign13980_e19163 * (p.p1069 * var_deltemp_dn5))))) / (2.0 * assign13980_e19179)))) / (assign13980_e19183 * assign13980_e19183)));
        var_delta_t_dn6 = (-((0.5 * (((-(var_delta_i_dn6 / (var_delta_i * var_delta_i))) * assign13980_e19146) + (((((-(var_delta_i_dn6 / (var_delta_i * var_delta_i))) * assign13980_e19157) * assign13980_e19171) + (assign13980_e19160 * ((-(var_delta_i_dn6 / (var_delta_i * var_delta_i))) * assign13980_e19168))) / (2.0 * assign13980_e19179)))) / (assign13980_e19183 * assign13980_e19183)));
        var_delta_t_dn7 = (-((0.5 * (((-(var_delta_i_dn7 / (var_delta_i * var_delta_i))) * assign13980_e19146) + (((((-(var_delta_i_dn7 / (var_delta_i * var_delta_i))) * assign13980_e19157) * assign13980_e19171) + (assign13980_e19160 * ((-(var_delta_i_dn7 / (var_delta_i * var_delta_i))) * assign13980_e19168))) / (2.0 * assign13980_e19179)))) / (assign13980_e19183 * assign13980_e19183)));
        var_delta_t_dn8 = (-((0.5 * (((-(var_delta_i_dn8 / (var_delta_i * var_delta_i))) * assign13980_e19146) + (((((-(var_delta_i_dn8 / (var_delta_i * var_delta_i))) * assign13980_e19157) * assign13980_e19171) + (assign13980_e19160 * ((-(var_delta_i_dn8 / (var_delta_i * var_delta_i))) * assign13980_e19168))) / (2.0 * assign13980_e19179)))) / (assign13980_e19183 * assign13980_e19183)));
        var_delta_t_dn9 = (-((0.5 * (((-(var_delta_i_dn9 / (var_delta_i * var_delta_i))) * assign13980_e19146) + (((((-(var_delta_i_dn9 / (var_delta_i * var_delta_i))) * assign13980_e19157) * assign13980_e19171) + (assign13980_e19160 * ((-(var_delta_i_dn9 / (var_delta_i * var_delta_i))) * assign13980_e19168))) / (2.0 * assign13980_e19179)))) / (assign13980_e19183 * assign13980_e19183)));
        var_delta_t_dn10 = (-((0.5 * (((-(var_delta_i_dn10 / (var_delta_i * var_delta_i))) * assign13980_e19146) + (((((-(var_delta_i_dn10 / (var_delta_i * var_delta_i))) * assign13980_e19157) * assign13980_e19171) + (assign13980_e19160 * ((-(var_delta_i_dn10 / (var_delta_i * var_delta_i))) * assign13980_e19168))) / (2.0 * assign13980_e19179)))) / (assign13980_e19183 * assign13980_e19183)));
        var_delta_t_dn11 = (-((0.5 * (((-(var_delta_i_dn11 / (var_delta_i * var_delta_i))) * assign13980_e19146) + (((((-(var_delta_i_dn11 / (var_delta_i * var_delta_i))) * assign13980_e19157) * assign13980_e19171) + (assign13980_e19160 * ((-(var_delta_i_dn11 / (var_delta_i * var_delta_i))) * assign13980_e19168))) / (2.0 * assign13980_e19179)))) / (assign13980_e19183 * assign13980_e19183)));
        var_delta_t_rv = 0.0;

        let assign13990_e19190: f64 = (var_ptwgt_i * var_deltemp);
        let assign13990_e19191: f64 = (1.0 - assign13990_e19190);
        let assign13990_e19193: f64 = (assign13990_e19191 - 1e-6);
        let assign13990_e19197: f64 = (var_ptwgt_i * var_deltemp);
        let assign13990_e19198: f64 = (1.0 - assign13990_e19197);
        let assign13990_e19200: f64 = (assign13990_e19198 - 1e-6);
        let assign13990_e19204: f64 = (var_ptwgt_i * var_deltemp);
        let assign13990_e19205: f64 = (1.0 - assign13990_e19204);
        let assign13990_e19207: f64 = (assign13990_e19205 - 1e-6);
        let assign13990_e19208: f64 = (assign13990_e19200 * assign13990_e19207);
        let assign13990_e19211: f64 = (4.0 * 0.001);
        let assign13990_e19213: f64 = (assign13990_e19211 * 0.001);
        let assign13990_e19214: f64 = (assign13990_e19208 + assign13990_e19213);
        let assign13990_e19215: f64 = (assign13990_e19214).sqrt();
        let assign13990_e19216: f64 = (assign13990_e19193 + assign13990_e19215);
        let assign13990_e19217: f64 = (0.5 * assign13990_e19216);
        let assign13990_e19218: f64 = (var_ptwg_i * assign13990_e19217);
        var_ptwg_t = assign13990_e19218;
        var_ptwg_t_dn3 = (var_ptwg_i_dn3 * assign13990_e19217);
        var_ptwg_t_dn4 = ((var_ptwg_i_dn4 * assign13990_e19217) + (var_ptwg_i * (0.5 * ((-(var_ptwgt_i * var_deltemp_dn4)) + ((((-(var_ptwgt_i * var_deltemp_dn4)) * assign13990_e19207) + (assign13990_e19200 * (-(var_ptwgt_i * var_deltemp_dn4)))) / (2.0 * assign13990_e19215))))));
        var_ptwg_t_dn5 = ((var_ptwg_i_dn5 * assign13990_e19217) + (var_ptwg_i * (0.5 * ((-(var_ptwgt_i * var_deltemp_dn5)) + ((((-(var_ptwgt_i * var_deltemp_dn5)) * assign13990_e19207) + (assign13990_e19200 * (-(var_ptwgt_i * var_deltemp_dn5)))) / (2.0 * assign13990_e19215))))));
        var_ptwg_t_dn6 = (var_ptwg_i_dn6 * assign13990_e19217);
        var_ptwg_t_dn7 = (var_ptwg_i_dn7 * assign13990_e19217);
        var_ptwg_t_dn8 = (var_ptwg_i_dn8 * assign13990_e19217);
        var_ptwg_t_dn9 = (var_ptwg_i_dn9 * assign13990_e19217);
        var_ptwg_t_dn10 = (var_ptwg_i_dn10 * assign13990_e19217);
        var_ptwg_t_dn11 = (var_ptwg_i_dn11 * assign13990_e19217);
        var_ptwg_t_rv = 0.0;

        let assign14000_e19221: f64 = if p.p35 != 0.0 { 1.0 } else { 0.0 };
        var_guard455 = assign14000_e19221;
        var_guard455_rv = 0.0;

        let (assign14010_e19258, assign14010_e19258_d_n3, assign14010_e19258_d_n4, assign14010_e19258_d_n5, assign14010_e19258_d_n6, assign14010_e19258_d_n7, assign14010_e19258_d_n8, assign14010_e19258_d_n9, assign14010_e19258_d_n10, assign14010_e19258_d_n11,) = {
    if (var_guard455 != 0.0) {
        let assign14010_e19228: f64 = (var_ptwgt_i * var_deltemp);
        let assign14010_e19229: f64 = (1.0 - assign14010_e19228);
        let assign14010_e19231: f64 = (assign14010_e19229 - 1e-6);
        let assign14010_e19235: f64 = (var_ptwgt_i * var_deltemp);
        let assign14010_e19236: f64 = (1.0 - assign14010_e19235);
        let assign14010_e19238: f64 = (assign14010_e19236 - 1e-6);
        let assign14010_e19242: f64 = (var_ptwgt_i * var_deltemp);
        let assign14010_e19243: f64 = (1.0 - assign14010_e19242);
        let assign14010_e19245: f64 = (assign14010_e19243 - 1e-6);
        let assign14010_e19246: f64 = (assign14010_e19238 * assign14010_e19245);
        let assign14010_e19249: f64 = (4.0 * 0.001);
        let assign14010_e19251: f64 = (assign14010_e19249 * 0.001);
        let assign14010_e19252: f64 = (assign14010_e19246 + assign14010_e19251);
        let assign14010_e19253: f64 = (assign14010_e19252).sqrt();
        let assign14010_e19254: f64 = (assign14010_e19231 + assign14010_e19253);
        let assign14010_e19255: f64 = (0.5 * assign14010_e19254);
        let assign14010_e19256: f64 = (var_ptwgr_i * assign14010_e19255);
        (assign14010_e19256, (var_ptwgr_i_dn3 * assign14010_e19255), ((var_ptwgr_i_dn4 * assign14010_e19255) + (var_ptwgr_i * (0.5 * ((-(var_ptwgt_i * var_deltemp_dn4)) + ((((-(var_ptwgt_i * var_deltemp_dn4)) * assign14010_e19245) + (assign14010_e19238 * (-(var_ptwgt_i * var_deltemp_dn4)))) / (2.0 * assign14010_e19253)))))), ((var_ptwgr_i_dn5 * assign14010_e19255) + (var_ptwgr_i * (0.5 * ((-(var_ptwgt_i * var_deltemp_dn5)) + ((((-(var_ptwgt_i * var_deltemp_dn5)) * assign14010_e19245) + (assign14010_e19238 * (-(var_ptwgt_i * var_deltemp_dn5)))) / (2.0 * assign14010_e19253)))))), (var_ptwgr_i_dn6 * assign14010_e19255), (var_ptwgr_i_dn7 * assign14010_e19255), (var_ptwgr_i_dn8 * assign14010_e19255), (var_ptwgr_i_dn9 * assign14010_e19255), (var_ptwgr_i_dn10 * assign14010_e19255), (var_ptwgr_i_dn11 * assign14010_e19255),)
    } else {
        (var_ptwgr_t, var_ptwgr_t_dn3, var_ptwgr_t_dn4, var_ptwgr_t_dn5, var_ptwgr_t_dn6, var_ptwgr_t_dn7, var_ptwgr_t_dn8, var_ptwgr_t_dn9, var_ptwgr_t_dn10, var_ptwgr_t_dn11,)
    }
};
        var_ptwgr_t = assign14010_e19258;
        var_ptwgr_t_dn3 = assign14010_e19258_d_n3;
        var_ptwgr_t_dn4 = assign14010_e19258_d_n4;
        var_ptwgr_t_dn5 = assign14010_e19258_d_n5;
        var_ptwgr_t_dn6 = assign14010_e19258_d_n6;
        var_ptwgr_t_dn7 = assign14010_e19258_d_n7;
        var_ptwgr_t_dn8 = assign14010_e19258_d_n8;
        var_ptwgr_t_dn9 = assign14010_e19258_d_n9;
        var_ptwgr_t_dn10 = assign14010_e19258_d_n10;
        var_ptwgr_t_dn11 = assign14010_e19258_d_n11;
        var_ptwgr_t_rv = 0.0;

        let assign14020_e19264: f64 = (var_a11_i * var_deltemp);
        let assign14020_e19265: f64 = (1.0 + assign14020_e19264);
        let assign14020_e19267: f64 = (assign14020_e19265 - 1e-6);
        let assign14020_e19271: f64 = (var_a11_i * var_deltemp);
        let assign14020_e19272: f64 = (1.0 + assign14020_e19271);
        let assign14020_e19274: f64 = (assign14020_e19272 - 1e-6);
        let assign14020_e19278: f64 = (var_a11_i * var_deltemp);
        let assign14020_e19279: f64 = (1.0 + assign14020_e19278);
        let assign14020_e19281: f64 = (assign14020_e19279 - 1e-6);
        let assign14020_e19282: f64 = (assign14020_e19274 * assign14020_e19281);
        let assign14020_e19285: f64 = (4.0 * 0.001);
        let assign14020_e19287: f64 = (assign14020_e19285 * 0.001);
        let assign14020_e19288: f64 = (assign14020_e19282 + assign14020_e19287);
        let assign14020_e19289: f64 = (assign14020_e19288).sqrt();
        let assign14020_e19290: f64 = (assign14020_e19267 + assign14020_e19289);
        let assign14020_e19291: f64 = (0.5 * assign14020_e19290);
        let assign14020_e19292: f64 = (var_a1_i * assign14020_e19291);
        var_a1_t = assign14020_e19292;
        var_a1_t_dn4 = (var_a1_i * (0.5 * ((var_a11_i * var_deltemp_dn4) + ((((var_a11_i * var_deltemp_dn4) * assign14020_e19281) + (assign14020_e19274 * (var_a11_i * var_deltemp_dn4))) / (2.0 * assign14020_e19289)))));
        var_a1_t_dn5 = (var_a1_i * (0.5 * ((var_a11_i * var_deltemp_dn5) + ((((var_a11_i * var_deltemp_dn5) * assign14020_e19281) + (assign14020_e19274 * (var_a11_i * var_deltemp_dn5))) / (2.0 * assign14020_e19289)))));
        var_a1_t_rv = 0.0;

        let assign14030_e19298: f64 = (var_a21_i * var_deltemp);
        let assign14030_e19299: f64 = (1.0 + assign14030_e19298);
        let assign14030_e19301: f64 = (assign14030_e19299 - 1e-6);
        let assign14030_e19305: f64 = (var_a21_i * var_deltemp);
        let assign14030_e19306: f64 = (1.0 + assign14030_e19305);
        let assign14030_e19308: f64 = (assign14030_e19306 - 1e-6);
        let assign14030_e19312: f64 = (var_a21_i * var_deltemp);
        let assign14030_e19313: f64 = (1.0 + assign14030_e19312);
        let assign14030_e19315: f64 = (assign14030_e19313 - 1e-6);
        let assign14030_e19316: f64 = (assign14030_e19308 * assign14030_e19315);
        let assign14030_e19319: f64 = (4.0 * 0.001);
        let assign14030_e19321: f64 = (assign14030_e19319 * 0.001);
        let assign14030_e19322: f64 = (assign14030_e19316 + assign14030_e19321);
        let assign14030_e19323: f64 = (assign14030_e19322).sqrt();
        let assign14030_e19324: f64 = (assign14030_e19301 + assign14030_e19323);
        let assign14030_e19325: f64 = (0.5 * assign14030_e19324);
        let assign14030_e19326: f64 = (var_a2_i * assign14030_e19325);
        var_a2_t = assign14030_e19326;
        var_a2_t_dn4 = (var_a2_i * (0.5 * ((var_a21_i * var_deltemp_dn4) + ((((var_a21_i * var_deltemp_dn4) * assign14030_e19315) + (assign14030_e19308 * (var_a21_i * var_deltemp_dn4))) / (2.0 * assign14030_e19323)))));
        var_a2_t_dn5 = (var_a2_i * (0.5 * ((var_a21_i * var_deltemp_dn5) + ((((var_a21_i * var_deltemp_dn5) * assign14030_e19315) + (assign14030_e19308 * (var_a21_i * var_deltemp_dn5))) / (2.0 * assign14030_e19323)))));
        var_a2_t_rv = 0.0;

        let assign14040_e19330: f64 = (var_tratio).powf(var_iit_i);
        let assign14040_e19331: f64 = (var_beta0_i * assign14040_e19330);
        var_beta0_t = assign14040_e19331;
        var_beta0_t_dn4 = (var_beta0_i * if 0.0 == 0.0 && ((var_iit_i) as f64).is_finite() && ((var_iit_i) as f64).fract() == 0.0 { if var_iit_i == 0.0 { 0.0 } else { (var_iit_i * ((var_tratio).powf(var_iit_i - 1.0) * var_tratio_dn4)) } } else { (assign14040_e19330 * (var_iit_i * (var_tratio_dn4 / var_tratio))) });
        var_beta0_t_dn5 = (var_beta0_i * if 0.0 == 0.0 && ((var_iit_i) as f64).is_finite() && ((var_iit_i) as f64).fract() == 0.0 { if var_iit_i == 0.0 { 0.0 } else { (var_iit_i * ((var_tratio).powf(var_iit_i - 1.0) * var_tratio_dn5)) } } else { (assign14040_e19330 * (var_iit_i * (var_tratio_dn5 / var_tratio))) });
        var_beta0_t_rv = 0.0;

        let assign14050_e19336: f64 = (var_tratio - 1.0);
        let assign14050_e19337: f64 = (var_bgidl1_i * assign14050_e19336);
        let assign14050_e19338: f64 = (var_bgidl_i + assign14050_e19337);
        var_bgidl_t = assign14050_e19338;
        var_bgidl_t_dn4 = (var_bgidl1_i * var_tratio_dn4);
        var_bgidl_t_dn5 = (var_bgidl1_i * var_tratio_dn5);
        var_bgidl_t_rv = 0.0;

        let assign14060_e19343: f64 = (var_tratio - 1.0);
        let assign14060_e19344: f64 = (var_bgisl1_i * assign14060_e19343);
        let assign14060_e19345: f64 = (var_bgisl_i + assign14060_e19344);
        var_bgisl_t = assign14060_e19345;
        var_bgisl_t_dn4 = (var_bgisl1_i * var_tratio_dn4);
        var_bgisl_t_dn5 = (var_bgisl1_i * var_tratio_dn5);
        var_bgisl_t_rv = 0.0;

        let assign14080_e19358: f64 = (var_k01_i * var_deltemp);
        let assign14080_e19359: f64 = (1.0 + assign14080_e19358);
        let assign14080_e19361: f64 = (assign14080_e19359 - 1e-6);
        let assign14080_e19365: f64 = (var_k01_i * var_deltemp);
        let assign14080_e19366: f64 = (1.0 + assign14080_e19365);
        let assign14080_e19368: f64 = (assign14080_e19366 - 1e-6);
        let assign14080_e19372: f64 = (var_k01_i * var_deltemp);
        let assign14080_e19373: f64 = (1.0 + assign14080_e19372);
        let assign14080_e19375: f64 = (assign14080_e19373 - 1e-6);
        let assign14080_e19376: f64 = (assign14080_e19368 * assign14080_e19375);
        let assign14080_e19379: f64 = (4.0 * 0.001);
        let assign14080_e19381: f64 = (assign14080_e19379 * 0.001);
        let assign14080_e19382: f64 = (assign14080_e19376 + assign14080_e19381);
        let assign14080_e19383: f64 = (assign14080_e19382).sqrt();
        let assign14080_e19384: f64 = (assign14080_e19361 + assign14080_e19383);
        let assign14080_e19385: f64 = (0.5 * assign14080_e19384);
        let assign14080_e19386: f64 = (var_k0_i * assign14080_e19385);
        var_k0_t = assign14080_e19386;
        var_k0_t_dn4 = (var_k0_i * (0.5 * ((var_k01_i * var_deltemp_dn4) + ((((var_k01_i * var_deltemp_dn4) * assign14080_e19375) + (assign14080_e19368 * (var_k01_i * var_deltemp_dn4))) / (2.0 * assign14080_e19383)))));
        var_k0_t_dn5 = (var_k0_i * (0.5 * ((var_k01_i * var_deltemp_dn5) + ((((var_k01_i * var_deltemp_dn5) * assign14080_e19375) + (assign14080_e19368 * (var_k01_i * var_deltemp_dn5))) / (2.0 * assign14080_e19383)))));
        var_k0_t_rv = 0.0;

        let assign14090_e19392: f64 = (var_m01_i * var_deltemp);
        let assign14090_e19393: f64 = (1.0 + assign14090_e19392);
        let assign14090_e19395: f64 = (assign14090_e19393 - 1e-6);
        let assign14090_e19399: f64 = (var_m01_i * var_deltemp);
        let assign14090_e19400: f64 = (1.0 + assign14090_e19399);
        let assign14090_e19402: f64 = (assign14090_e19400 - 1e-6);
        let assign14090_e19406: f64 = (var_m01_i * var_deltemp);
        let assign14090_e19407: f64 = (1.0 + assign14090_e19406);
        let assign14090_e19409: f64 = (assign14090_e19407 - 1e-6);
        let assign14090_e19410: f64 = (assign14090_e19402 * assign14090_e19409);
        let assign14090_e19413: f64 = (4.0 * 0.001);
        let assign14090_e19415: f64 = (assign14090_e19413 * 0.001);
        let assign14090_e19416: f64 = (assign14090_e19410 + assign14090_e19415);
        let assign14090_e19417: f64 = (assign14090_e19416).sqrt();
        let assign14090_e19418: f64 = (assign14090_e19395 + assign14090_e19417);
        let assign14090_e19419: f64 = (0.5 * assign14090_e19418);
        let assign14090_e19420: f64 = (var_m0_i * assign14090_e19419);
        var_m0_t = assign14090_e19420;
        var_m0_t_dn4 = (var_m0_i * (0.5 * ((var_m01_i * var_deltemp_dn4) + ((((var_m01_i * var_deltemp_dn4) * assign14090_e19409) + (assign14090_e19402 * (var_m01_i * var_deltemp_dn4))) / (2.0 * assign14090_e19417)))));
        var_m0_t_dn5 = (var_m0_i * (0.5 * ((var_m01_i * var_deltemp_dn5) + ((((var_m01_i * var_deltemp_dn5) * assign14090_e19409) + (assign14090_e19402 * (var_m01_i * var_deltemp_dn5))) / (2.0 * assign14090_e19417)))));
        var_m0_t_rv = 0.0;

        let assign14100_e19426: f64 = (var_c01_i * var_deltemp);
        let assign14100_e19427: f64 = (1.0 + assign14100_e19426);
        let assign14100_e19429: f64 = (assign14100_e19427 - 1e-6);
        let assign14100_e19433: f64 = (var_c01_i * var_deltemp);
        let assign14100_e19434: f64 = (1.0 + assign14100_e19433);
        let assign14100_e19436: f64 = (assign14100_e19434 - 1e-6);
        let assign14100_e19440: f64 = (var_c01_i * var_deltemp);
        let assign14100_e19441: f64 = (1.0 + assign14100_e19440);
        let assign14100_e19443: f64 = (assign14100_e19441 - 1e-6);
        let assign14100_e19444: f64 = (assign14100_e19436 * assign14100_e19443);
        let assign14100_e19447: f64 = (4.0 * 0.001);
        let assign14100_e19449: f64 = (assign14100_e19447 * 0.001);
        let assign14100_e19450: f64 = (assign14100_e19444 + assign14100_e19449);
        let assign14100_e19451: f64 = (assign14100_e19450).sqrt();
        let assign14100_e19452: f64 = (assign14100_e19429 + assign14100_e19451);
        let assign14100_e19453: f64 = (0.5 * assign14100_e19452);
        let assign14100_e19454: f64 = (var_c0_i * assign14100_e19453);
        var_c0_t = assign14100_e19454;
        var_c0_t_dn4 = (var_c0_i * (0.5 * ((var_c01_i * var_deltemp_dn4) + ((((var_c01_i * var_deltemp_dn4) * assign14100_e19443) + (assign14100_e19436 * (var_c01_i * var_deltemp_dn4))) / (2.0 * assign14100_e19451)))));
        var_c0_t_dn5 = (var_c0_i * (0.5 * ((var_c01_i * var_deltemp_dn5) + ((((var_c01_i * var_deltemp_dn5) * assign14100_e19443) + (assign14100_e19436 * (var_c01_i * var_deltemp_dn5))) / (2.0 * assign14100_e19451)))));
        var_c0_t_rv = 0.0;

        let assign14110_e19460: f64 = (var_c0si1_i * var_deltemp);
        let assign14110_e19461: f64 = (1.0 + assign14110_e19460);
        let assign14110_e19463: f64 = (assign14110_e19461 - 1e-6);
        let assign14110_e19467: f64 = (var_c0si1_i * var_deltemp);
        let assign14110_e19468: f64 = (1.0 + assign14110_e19467);
        let assign14110_e19470: f64 = (assign14110_e19468 - 1e-6);
        let assign14110_e19474: f64 = (var_c0si1_i * var_deltemp);
        let assign14110_e19475: f64 = (1.0 + assign14110_e19474);
        let assign14110_e19477: f64 = (assign14110_e19475 - 1e-6);
        let assign14110_e19478: f64 = (assign14110_e19470 * assign14110_e19477);
        let assign14110_e19481: f64 = (4.0 * 0.001);
        let assign14110_e19483: f64 = (assign14110_e19481 * 0.001);
        let assign14110_e19484: f64 = (assign14110_e19478 + assign14110_e19483);
        let assign14110_e19485: f64 = (assign14110_e19484).sqrt();
        let assign14110_e19486: f64 = (assign14110_e19463 + assign14110_e19485);
        let assign14110_e19487: f64 = (0.5 * assign14110_e19486);
        let assign14110_e19488: f64 = (var_c0si_i * assign14110_e19487);
        var_c0si_t = assign14110_e19488;
        var_c0si_t_dn4 = (var_c0si_i * (0.5 * ((var_c0si1_i * var_deltemp_dn4) + ((((var_c0si1_i * var_deltemp_dn4) * assign14110_e19477) + (assign14110_e19470 * (var_c0si1_i * var_deltemp_dn4))) / (2.0 * assign14110_e19485)))));
        var_c0si_t_dn5 = (var_c0si_i * (0.5 * ((var_c0si1_i * var_deltemp_dn5) + ((((var_c0si1_i * var_deltemp_dn5) * assign14110_e19477) + (assign14110_e19470 * (var_c0si1_i * var_deltemp_dn5))) / (2.0 * assign14110_e19485)))));
        var_c0si_t_rv = 0.0;

        *var_a1_t_slot = var_a1_t;
        *var_a1_t_dn4_slot = var_a1_t_dn4;
        *var_a1_t_dn5_slot = var_a1_t_dn5;
        *var_a1_t_rv_slot = var_a1_t_rv;
        *var_a2_t_slot = var_a2_t;
        *var_a2_t_dn4_slot = var_a2_t_dn4;
        *var_a2_t_dn5_slot = var_a2_t_dn5;
        *var_a2_t_rv_slot = var_a2_t_rv;
        *var_beta0_t_slot = var_beta0_t;
        *var_beta0_t_dn4_slot = var_beta0_t_dn4;
        *var_beta0_t_dn5_slot = var_beta0_t_dn5;
        *var_beta0_t_rv_slot = var_beta0_t_rv;
        *var_bgidl_t_slot = var_bgidl_t;
        *var_bgidl_t_dn4_slot = var_bgidl_t_dn4;
        *var_bgidl_t_dn5_slot = var_bgidl_t_dn5;
        *var_bgidl_t_rv_slot = var_bgidl_t_rv;
        *var_bgisl_t_slot = var_bgisl_t;
        *var_bgisl_t_dn4_slot = var_bgisl_t_dn4;
        *var_bgisl_t_dn5_slot = var_bgisl_t_dn5;
        *var_bgisl_t_rv_slot = var_bgisl_t_rv;
        *var_c0_t_slot = var_c0_t;
        *var_c0_t_dn4_slot = var_c0_t_dn4;
        *var_c0_t_dn5_slot = var_c0_t_dn5;
        *var_c0_t_rv_slot = var_c0_t_rv;
        *var_c0si_t_slot = var_c0si_t;
        *var_c0si_t_dn4_slot = var_c0si_t_dn4;
        *var_c0si_t_dn5_slot = var_c0si_t_dn5;
        *var_c0si_t_rv_slot = var_c0si_t_rv;
        *var_delta_t_slot = var_delta_t;
        *var_delta_t_dn10_slot = var_delta_t_dn10;
        *var_delta_t_dn11_slot = var_delta_t_dn11;
        *var_delta_t_dn3_slot = var_delta_t_dn3;
        *var_delta_t_dn4_slot = var_delta_t_dn4;
        *var_delta_t_dn5_slot = var_delta_t_dn5;
        *var_delta_t_dn6_slot = var_delta_t_dn6;
        *var_delta_t_dn7_slot = var_delta_t_dn7;
        *var_delta_t_dn8_slot = var_delta_t_dn8;
        *var_delta_t_dn9_slot = var_delta_t_dn9;
        *var_delta_t_rv_slot = var_delta_t_rv;
        *var_guard451_slot = var_guard451;
        *var_guard451_rv_slot = var_guard451_rv;
        *var_guard452_slot = var_guard452;
        *var_guard452_rv_slot = var_guard452_rv;
        *var_guard453_slot = var_guard453;
        *var_guard453_rv_slot = var_guard453_rv;
        *var_guard454_slot = var_guard454;
        *var_guard454_rv_slot = var_guard454_rv;
        *var_guard455_slot = var_guard455;
        *var_guard455_rv_slot = var_guard455_rv;
        *var_k0_t_slot = var_k0_t;
        *var_k0_t_dn4_slot = var_k0_t_dn4;
        *var_k0_t_dn5_slot = var_k0_t_dn5;
        *var_k0_t_rv_slot = var_k0_t_rv;
        *var_m0_t_slot = var_m0_t;
        *var_m0_t_dn4_slot = var_m0_t_dn4;
        *var_m0_t_dn5_slot = var_m0_t_dn5;
        *var_m0_t_rv_slot = var_m0_t_rv;
        *var_ptwg_t_slot = var_ptwg_t;
        *var_ptwg_t_dn10_slot = var_ptwg_t_dn10;
        *var_ptwg_t_dn11_slot = var_ptwg_t_dn11;
        *var_ptwg_t_dn3_slot = var_ptwg_t_dn3;
        *var_ptwg_t_dn4_slot = var_ptwg_t_dn4;
        *var_ptwg_t_dn5_slot = var_ptwg_t_dn5;
        *var_ptwg_t_dn6_slot = var_ptwg_t_dn6;
        *var_ptwg_t_dn7_slot = var_ptwg_t_dn7;
        *var_ptwg_t_dn8_slot = var_ptwg_t_dn8;
        *var_ptwg_t_dn9_slot = var_ptwg_t_dn9;
        *var_ptwg_t_rv_slot = var_ptwg_t_rv;
        *var_ptwgr_t_slot = var_ptwgr_t;
        *var_ptwgr_t_dn10_slot = var_ptwgr_t_dn10;
        *var_ptwgr_t_dn11_slot = var_ptwgr_t_dn11;
        *var_ptwgr_t_dn3_slot = var_ptwgr_t_dn3;
        *var_ptwgr_t_dn4_slot = var_ptwgr_t_dn4;
        *var_ptwgr_t_dn5_slot = var_ptwgr_t_dn5;
        *var_ptwgr_t_dn6_slot = var_ptwgr_t_dn6;
        *var_ptwgr_t_dn7_slot = var_ptwgr_t_dn7;
        *var_ptwgr_t_dn8_slot = var_ptwgr_t_dn8;
        *var_ptwgr_t_dn9_slot = var_ptwgr_t_dn9;
        *var_ptwgr_t_rv_slot = var_ptwgr_t_rv;
        *var_rdstemp_slot = var_rdstemp;
        *var_rdstemp_dn4_slot = var_rdstemp_dn4;
        *var_rdstemp_dn5_slot = var_rdstemp_dn5;
        *var_rdstemp_rv_slot = var_rdstemp_rv;
        *var_uar_t_slot = var_uar_t;
        *var_uar_t_dn10_slot = var_uar_t_dn10;
        *var_uar_t_dn11_slot = var_uar_t_dn11;
        *var_uar_t_dn3_slot = var_uar_t_dn3;
        *var_uar_t_dn4_slot = var_uar_t_dn4;
        *var_uar_t_dn5_slot = var_uar_t_dn5;
        *var_uar_t_dn6_slot = var_uar_t_dn6;
        *var_uar_t_dn7_slot = var_uar_t_dn7;
        *var_uar_t_dn8_slot = var_uar_t_dn8;
        *var_uar_t_dn9_slot = var_uar_t_dn9;
        *var_uar_t_rv_slot = var_uar_t_rv;
        *var_ucr_t_slot = var_ucr_t;
        *var_ucr_t_dn10_slot = var_ucr_t_dn10;
        *var_ucr_t_dn11_slot = var_ucr_t_dn11;
        *var_ucr_t_dn3_slot = var_ucr_t_dn3;
        *var_ucr_t_dn4_slot = var_ucr_t_dn4;
        *var_ucr_t_dn5_slot = var_ucr_t_dn5;
        *var_ucr_t_dn6_slot = var_ucr_t_dn6;
        *var_ucr_t_dn7_slot = var_ucr_t_dn7;
        *var_ucr_t_dn8_slot = var_ucr_t_dn8;
        *var_ucr_t_dn9_slot = var_ucr_t_dn9;
        *var_ucr_t_rv_slot = var_ucr_t_rv;
        *var_ucsr_t_slot = var_ucsr_t;
        *var_ucsr_t_dn4_slot = var_ucsr_t_dn4;
        *var_ucsr_t_dn5_slot = var_ucsr_t_dn5;
        *var_ucsr_t_rv_slot = var_ucsr_t_rv;
        *var_udr_t_slot = var_udr_t;
        *var_udr_t_dn10_slot = var_udr_t_dn10;
        *var_udr_t_dn11_slot = var_udr_t_dn11;
        *var_udr_t_dn3_slot = var_udr_t_dn3;
        *var_udr_t_dn4_slot = var_udr_t_dn4;
        *var_udr_t_dn5_slot = var_udr_t_dn5;
        *var_udr_t_dn6_slot = var_udr_t_dn6;
        *var_udr_t_dn7_slot = var_udr_t_dn7;
        *var_udr_t_dn8_slot = var_udr_t_dn8;
        *var_udr_t_dn9_slot = var_udr_t_dn9;
        *var_udr_t_rv_slot = var_udr_t_rv;
        *var_vsat_t_slot = var_vsat_t;
        *var_vsat_t_dn10_slot = var_vsat_t_dn10;
        *var_vsat_t_dn11_slot = var_vsat_t_dn11;
        *var_vsat_t_dn3_slot = var_vsat_t_dn3;
        *var_vsat_t_dn4_slot = var_vsat_t_dn4;
        *var_vsat_t_dn5_slot = var_vsat_t_dn5;
        *var_vsat_t_dn6_slot = var_vsat_t_dn6;
        *var_vsat_t_dn7_slot = var_vsat_t_dn7;
        *var_vsat_t_dn8_slot = var_vsat_t_dn8;
        *var_vsat_t_dn9_slot = var_vsat_t_dn9;
        *var_vsat_t_rv_slot = var_vsat_t_rv;
        *var_vsatcv_t_slot = var_vsatcv_t;
        *var_vsatcv_t_dn10_slot = var_vsatcv_t_dn10;
        *var_vsatcv_t_dn11_slot = var_vsatcv_t_dn11;
        *var_vsatcv_t_dn3_slot = var_vsatcv_t_dn3;
        *var_vsatcv_t_dn4_slot = var_vsatcv_t_dn4;
        *var_vsatcv_t_dn5_slot = var_vsatcv_t_dn5;
        *var_vsatcv_t_dn6_slot = var_vsatcv_t_dn6;
        *var_vsatcv_t_dn7_slot = var_vsatcv_t_dn7;
        *var_vsatcv_t_dn8_slot = var_vsatcv_t_dn8;
        *var_vsatcv_t_dn9_slot = var_vsatcv_t_dn9;
        *var_vsatcv_t_rv_slot = var_vsatcv_t_rv;
        *var_vsatr_t_slot = var_vsatr_t;
        *var_vsatr_t_dn10_slot = var_vsatr_t_dn10;
        *var_vsatr_t_dn11_slot = var_vsatr_t_dn11;
        *var_vsatr_t_dn3_slot = var_vsatr_t_dn3;
        *var_vsatr_t_dn4_slot = var_vsatr_t_dn4;
        *var_vsatr_t_dn5_slot = var_vsatr_t_dn5;
        *var_vsatr_t_dn6_slot = var_vsatr_t_dn6;
        *var_vsatr_t_dn7_slot = var_vsatr_t_dn7;
        *var_vsatr_t_dn8_slot = var_vsatr_t_dn8;
        *var_vsatr_t_dn9_slot = var_vsatr_t_dn9;
        *var_vsatr_t_rv_slot = var_vsatr_t_rv;
    }

    pub(super) fn stamp_reactive_block_24(
        p: &Parameters,
        var_c0sisat1_i: f64,
        var_c0sisat_i: f64,
        var_deltemp: f64,
        var_deltemp_dn4: f64,
        var_deltemp_dn5: f64,
        var_dmcgeff: f64,
        var_dmcieff: f64,
        var_dmdgeff: f64,
        var_weffcj: f64,
        var_c0sisat_t_slot: &mut f64,
        var_c0sisat_t_dn4_slot: &mut f64,
        var_c0sisat_t_dn5_slot: &mut f64,
        var_c0sisat_t_rv_slot: &mut f64,
        var_cjd_t_slot: &mut f64,
        var_cjd_t_dn4_slot: &mut f64,
        var_cjd_t_dn5_slot: &mut f64,
        var_cjd_t_rv_slot: &mut f64,
        var_cjs_t_slot: &mut f64,
        var_cjs_t_dn4_slot: &mut f64,
        var_cjs_t_dn5_slot: &mut f64,
        var_cjs_t_rv_slot: &mut f64,
        var_cjswd_t_slot: &mut f64,
        var_cjswd_t_dn4_slot: &mut f64,
        var_cjswd_t_dn5_slot: &mut f64,
        var_cjswd_t_rv_slot: &mut f64,
        var_cjswgd_t_slot: &mut f64,
        var_cjswgd_t_dn4_slot: &mut f64,
        var_cjswgd_t_dn5_slot: &mut f64,
        var_cjswgd_t_rv_slot: &mut f64,
        var_cjswgs_t_slot: &mut f64,
        var_cjswgs_t_dn4_slot: &mut f64,
        var_cjswgs_t_dn5_slot: &mut f64,
        var_cjswgs_t_rv_slot: &mut f64,
        var_cjsws_t_slot: &mut f64,
        var_cjsws_t_dn4_slot: &mut f64,
        var_cjsws_t_dn5_slot: &mut f64,
        var_cjsws_t_rv_slot: &mut f64,
        var_guard456_slot: &mut f64,
        var_guard456_rv_slot: &mut f64,
        var_guard457_slot: &mut f64,
        var_guard457_rv_slot: &mut f64,
        var_guard458_slot: &mut f64,
        var_guard458_rv_slot: &mut f64,
        var_nuendd_slot: &mut f64,
        var_nuendd_rv_slot: &mut f64,
        var_nuends_slot: &mut f64,
        var_nuends_rv_slot: &mut f64,
        var_nuintd_slot: &mut f64,
        var_nuintd_rv_slot: &mut f64,
        var_nuints_slot: &mut f64,
        var_nuints_rv_slot: &mut f64,
        var_pbd_t_slot: &mut f64,
        var_pbd_t_dn4_slot: &mut f64,
        var_pbd_t_dn5_slot: &mut f64,
        var_pbd_t_rv_slot: &mut f64,
        var_pbs_t_slot: &mut f64,
        var_pbs_t_dn4_slot: &mut f64,
        var_pbs_t_dn5_slot: &mut f64,
        var_pbs_t_rv_slot: &mut f64,
        var_pbswd_t_slot: &mut f64,
        var_pbswd_t_dn4_slot: &mut f64,
        var_pbswd_t_dn5_slot: &mut f64,
        var_pbswd_t_rv_slot: &mut f64,
        var_pbswgd_t_slot: &mut f64,
        var_pbswgd_t_dn4_slot: &mut f64,
        var_pbswgd_t_dn5_slot: &mut f64,
        var_pbswgd_t_rv_slot: &mut f64,
        var_pbswgs_t_slot: &mut f64,
        var_pbswgs_t_dn4_slot: &mut f64,
        var_pbswgs_t_dn5_slot: &mut f64,
        var_pbswgs_t_rv_slot: &mut f64,
        var_pbsws_t_slot: &mut f64,
        var_pbsws_t_dn4_slot: &mut f64,
        var_pbsws_t_dn5_slot: &mut f64,
        var_pbsws_t_rv_slot: &mut f64,
        var_pdiso_slot: &mut f64,
        var_pdiso_dn10_slot: &mut f64,
        var_pdiso_dn11_slot: &mut f64,
        var_pdiso_dn3_slot: &mut f64,
        var_pdiso_dn4_slot: &mut f64,
        var_pdiso_dn5_slot: &mut f64,
        var_pdiso_dn6_slot: &mut f64,
        var_pdiso_dn7_slot: &mut f64,
        var_pdiso_dn8_slot: &mut f64,
        var_pdiso_dn9_slot: &mut f64,
        var_pdiso_rv_slot: &mut f64,
        var_psiso_slot: &mut f64,
        var_psiso_dn10_slot: &mut f64,
        var_psiso_dn11_slot: &mut f64,
        var_psiso_dn3_slot: &mut f64,
        var_psiso_dn4_slot: &mut f64,
        var_psiso_dn5_slot: &mut f64,
        var_psiso_dn6_slot: &mut f64,
        var_psiso_dn7_slot: &mut f64,
        var_psiso_dn8_slot: &mut f64,
        var_psiso_dn9_slot: &mut f64,
        var_psiso_rv_slot: &mut f64,
        var_pssha_slot: &mut f64,
        var_pssha_dn10_slot: &mut f64,
        var_pssha_dn11_slot: &mut f64,
        var_pssha_dn3_slot: &mut f64,
        var_pssha_dn4_slot: &mut f64,
        var_pssha_dn5_slot: &mut f64,
        var_pssha_dn6_slot: &mut f64,
        var_pssha_dn7_slot: &mut f64,
        var_pssha_dn8_slot: &mut f64,
        var_pssha_dn9_slot: &mut f64,
        var_pssha_rv_slot: &mut f64,
        var_t0_slot: &mut f64,
        var_t0_dn10_slot: &mut f64,
        var_t0_dn11_slot: &mut f64,
        var_t0_dn3_slot: &mut f64,
        var_t0_dn4_slot: &mut f64,
        var_t0_dn5_slot: &mut f64,
        var_t0_dn6_slot: &mut f64,
        var_t0_dn7_slot: &mut f64,
        var_t0_dn8_slot: &mut f64,
        var_t0_dn9_slot: &mut f64,
        var_t0_rv_slot: &mut f64,
        var_t1_slot: &mut f64,
        var_t1_dn10_slot: &mut f64,
        var_t1_dn11_slot: &mut f64,
        var_t1_dn3_slot: &mut f64,
        var_t1_dn4_slot: &mut f64,
        var_t1_dn5_slot: &mut f64,
        var_t1_dn6_slot: &mut f64,
        var_t1_dn7_slot: &mut f64,
        var_t1_dn8_slot: &mut f64,
        var_t1_dn9_slot: &mut f64,
        var_t1_rv_slot: &mut f64,
        var_t2_slot: &mut f64,
        var_t2_dn10_slot: &mut f64,
        var_t2_dn11_slot: &mut f64,
        var_t2_dn3_slot: &mut f64,
        var_t2_dn4_slot: &mut f64,
        var_t2_dn5_slot: &mut f64,
        var_t2_dn6_slot: &mut f64,
        var_t2_dn7_slot: &mut f64,
        var_t2_dn8_slot: &mut f64,
        var_t2_dn9_slot: &mut f64,
        var_t2_rv_slot: &mut f64,
    ) {
        let mut var_c0sisat_t: f64 = *var_c0sisat_t_slot;
        let mut var_c0sisat_t_dn4: f64 = *var_c0sisat_t_dn4_slot;
        let mut var_c0sisat_t_dn5: f64 = *var_c0sisat_t_dn5_slot;
        let mut var_c0sisat_t_rv: f64 = *var_c0sisat_t_rv_slot;
        let mut var_cjd_t: f64 = *var_cjd_t_slot;
        let mut var_cjd_t_dn4: f64 = *var_cjd_t_dn4_slot;
        let mut var_cjd_t_dn5: f64 = *var_cjd_t_dn5_slot;
        let mut var_cjd_t_rv: f64 = *var_cjd_t_rv_slot;
        let mut var_cjs_t: f64 = *var_cjs_t_slot;
        let mut var_cjs_t_dn4: f64 = *var_cjs_t_dn4_slot;
        let mut var_cjs_t_dn5: f64 = *var_cjs_t_dn5_slot;
        let mut var_cjs_t_rv: f64 = *var_cjs_t_rv_slot;
        let mut var_cjswd_t: f64 = *var_cjswd_t_slot;
        let mut var_cjswd_t_dn4: f64 = *var_cjswd_t_dn4_slot;
        let mut var_cjswd_t_dn5: f64 = *var_cjswd_t_dn5_slot;
        let mut var_cjswd_t_rv: f64 = *var_cjswd_t_rv_slot;
        let mut var_cjswgd_t: f64 = *var_cjswgd_t_slot;
        let mut var_cjswgd_t_dn4: f64 = *var_cjswgd_t_dn4_slot;
        let mut var_cjswgd_t_dn5: f64 = *var_cjswgd_t_dn5_slot;
        let mut var_cjswgd_t_rv: f64 = *var_cjswgd_t_rv_slot;
        let mut var_cjswgs_t: f64 = *var_cjswgs_t_slot;
        let mut var_cjswgs_t_dn4: f64 = *var_cjswgs_t_dn4_slot;
        let mut var_cjswgs_t_dn5: f64 = *var_cjswgs_t_dn5_slot;
        let mut var_cjswgs_t_rv: f64 = *var_cjswgs_t_rv_slot;
        let mut var_cjsws_t: f64 = *var_cjsws_t_slot;
        let mut var_cjsws_t_dn4: f64 = *var_cjsws_t_dn4_slot;
        let mut var_cjsws_t_dn5: f64 = *var_cjsws_t_dn5_slot;
        let mut var_cjsws_t_rv: f64 = *var_cjsws_t_rv_slot;
        let mut var_guard456: f64 = *var_guard456_slot;
        let mut var_guard456_rv: f64 = *var_guard456_rv_slot;
        let mut var_guard457: f64 = *var_guard457_slot;
        let mut var_guard457_rv: f64 = *var_guard457_rv_slot;
        let mut var_guard458: f64 = *var_guard458_slot;
        let mut var_guard458_rv: f64 = *var_guard458_rv_slot;
        let mut var_nuendd: f64 = *var_nuendd_slot;
        let mut var_nuendd_rv: f64 = *var_nuendd_rv_slot;
        let mut var_nuends: f64 = *var_nuends_slot;
        let mut var_nuends_rv: f64 = *var_nuends_rv_slot;
        let mut var_nuintd: f64 = *var_nuintd_slot;
        let mut var_nuintd_rv: f64 = *var_nuintd_rv_slot;
        let mut var_nuints: f64 = *var_nuints_slot;
        let mut var_nuints_rv: f64 = *var_nuints_rv_slot;
        let mut var_pbd_t: f64 = *var_pbd_t_slot;
        let mut var_pbd_t_dn4: f64 = *var_pbd_t_dn4_slot;
        let mut var_pbd_t_dn5: f64 = *var_pbd_t_dn5_slot;
        let mut var_pbd_t_rv: f64 = *var_pbd_t_rv_slot;
        let mut var_pbs_t: f64 = *var_pbs_t_slot;
        let mut var_pbs_t_dn4: f64 = *var_pbs_t_dn4_slot;
        let mut var_pbs_t_dn5: f64 = *var_pbs_t_dn5_slot;
        let mut var_pbs_t_rv: f64 = *var_pbs_t_rv_slot;
        let mut var_pbswd_t: f64 = *var_pbswd_t_slot;
        let mut var_pbswd_t_dn4: f64 = *var_pbswd_t_dn4_slot;
        let mut var_pbswd_t_dn5: f64 = *var_pbswd_t_dn5_slot;
        let mut var_pbswd_t_rv: f64 = *var_pbswd_t_rv_slot;
        let mut var_pbswgd_t: f64 = *var_pbswgd_t_slot;
        let mut var_pbswgd_t_dn4: f64 = *var_pbswgd_t_dn4_slot;
        let mut var_pbswgd_t_dn5: f64 = *var_pbswgd_t_dn5_slot;
        let mut var_pbswgd_t_rv: f64 = *var_pbswgd_t_rv_slot;
        let mut var_pbswgs_t: f64 = *var_pbswgs_t_slot;
        let mut var_pbswgs_t_dn4: f64 = *var_pbswgs_t_dn4_slot;
        let mut var_pbswgs_t_dn5: f64 = *var_pbswgs_t_dn5_slot;
        let mut var_pbswgs_t_rv: f64 = *var_pbswgs_t_rv_slot;
        let mut var_pbsws_t: f64 = *var_pbsws_t_slot;
        let mut var_pbsws_t_dn4: f64 = *var_pbsws_t_dn4_slot;
        let mut var_pbsws_t_dn5: f64 = *var_pbsws_t_dn5_slot;
        let mut var_pbsws_t_rv: f64 = *var_pbsws_t_rv_slot;
        let mut var_pdiso: f64 = *var_pdiso_slot;
        let mut var_pdiso_dn10: f64 = *var_pdiso_dn10_slot;
        let mut var_pdiso_dn11: f64 = *var_pdiso_dn11_slot;
        let mut var_pdiso_dn3: f64 = *var_pdiso_dn3_slot;
        let mut var_pdiso_dn4: f64 = *var_pdiso_dn4_slot;
        let mut var_pdiso_dn5: f64 = *var_pdiso_dn5_slot;
        let mut var_pdiso_dn6: f64 = *var_pdiso_dn6_slot;
        let mut var_pdiso_dn7: f64 = *var_pdiso_dn7_slot;
        let mut var_pdiso_dn8: f64 = *var_pdiso_dn8_slot;
        let mut var_pdiso_dn9: f64 = *var_pdiso_dn9_slot;
        let mut var_pdiso_rv: f64 = *var_pdiso_rv_slot;
        let mut var_psiso: f64 = *var_psiso_slot;
        let mut var_psiso_dn10: f64 = *var_psiso_dn10_slot;
        let mut var_psiso_dn11: f64 = *var_psiso_dn11_slot;
        let mut var_psiso_dn3: f64 = *var_psiso_dn3_slot;
        let mut var_psiso_dn4: f64 = *var_psiso_dn4_slot;
        let mut var_psiso_dn5: f64 = *var_psiso_dn5_slot;
        let mut var_psiso_dn6: f64 = *var_psiso_dn6_slot;
        let mut var_psiso_dn7: f64 = *var_psiso_dn7_slot;
        let mut var_psiso_dn8: f64 = *var_psiso_dn8_slot;
        let mut var_psiso_dn9: f64 = *var_psiso_dn9_slot;
        let mut var_psiso_rv: f64 = *var_psiso_rv_slot;
        let mut var_pssha: f64 = *var_pssha_slot;
        let mut var_pssha_dn10: f64 = *var_pssha_dn10_slot;
        let mut var_pssha_dn11: f64 = *var_pssha_dn11_slot;
        let mut var_pssha_dn3: f64 = *var_pssha_dn3_slot;
        let mut var_pssha_dn4: f64 = *var_pssha_dn4_slot;
        let mut var_pssha_dn5: f64 = *var_pssha_dn5_slot;
        let mut var_pssha_dn6: f64 = *var_pssha_dn6_slot;
        let mut var_pssha_dn7: f64 = *var_pssha_dn7_slot;
        let mut var_pssha_dn8: f64 = *var_pssha_dn8_slot;
        let mut var_pssha_dn9: f64 = *var_pssha_dn9_slot;
        let mut var_pssha_rv: f64 = *var_pssha_rv_slot;
        let mut var_t0: f64 = *var_t0_slot;
        let mut var_t0_dn10: f64 = *var_t0_dn10_slot;
        let mut var_t0_dn11: f64 = *var_t0_dn11_slot;
        let mut var_t0_dn3: f64 = *var_t0_dn3_slot;
        let mut var_t0_dn4: f64 = *var_t0_dn4_slot;
        let mut var_t0_dn5: f64 = *var_t0_dn5_slot;
        let mut var_t0_dn6: f64 = *var_t0_dn6_slot;
        let mut var_t0_dn7: f64 = *var_t0_dn7_slot;
        let mut var_t0_dn8: f64 = *var_t0_dn8_slot;
        let mut var_t0_dn9: f64 = *var_t0_dn9_slot;
        let mut var_t0_rv: f64 = *var_t0_rv_slot;
        let mut var_t1: f64 = *var_t1_slot;
        let mut var_t1_dn10: f64 = *var_t1_dn10_slot;
        let mut var_t1_dn11: f64 = *var_t1_dn11_slot;
        let mut var_t1_dn3: f64 = *var_t1_dn3_slot;
        let mut var_t1_dn4: f64 = *var_t1_dn4_slot;
        let mut var_t1_dn5: f64 = *var_t1_dn5_slot;
        let mut var_t1_dn6: f64 = *var_t1_dn6_slot;
        let mut var_t1_dn7: f64 = *var_t1_dn7_slot;
        let mut var_t1_dn8: f64 = *var_t1_dn8_slot;
        let mut var_t1_dn9: f64 = *var_t1_dn9_slot;
        let mut var_t1_rv: f64 = *var_t1_rv_slot;
        let mut var_t2: f64 = *var_t2_slot;
        let mut var_t2_dn10: f64 = *var_t2_dn10_slot;
        let mut var_t2_dn11: f64 = *var_t2_dn11_slot;
        let mut var_t2_dn3: f64 = *var_t2_dn3_slot;
        let mut var_t2_dn4: f64 = *var_t2_dn4_slot;
        let mut var_t2_dn5: f64 = *var_t2_dn5_slot;
        let mut var_t2_dn6: f64 = *var_t2_dn6_slot;
        let mut var_t2_dn7: f64 = *var_t2_dn7_slot;
        let mut var_t2_dn8: f64 = *var_t2_dn8_slot;
        let mut var_t2_dn9: f64 = *var_t2_dn9_slot;
        let mut var_t2_rv: f64 = *var_t2_rv_slot;

        let assign14120_e19494: f64 = (var_c0sisat1_i * var_deltemp);
        let assign14120_e19495: f64 = (1.0 + assign14120_e19494);
        let assign14120_e19497: f64 = (assign14120_e19495 - 1e-6);
        let assign14120_e19501: f64 = (var_c0sisat1_i * var_deltemp);
        let assign14120_e19502: f64 = (1.0 + assign14120_e19501);
        let assign14120_e19504: f64 = (assign14120_e19502 - 1e-6);
        let assign14120_e19508: f64 = (var_c0sisat1_i * var_deltemp);
        let assign14120_e19509: f64 = (1.0 + assign14120_e19508);
        let assign14120_e19511: f64 = (assign14120_e19509 - 1e-6);
        let assign14120_e19512: f64 = (assign14120_e19504 * assign14120_e19511);
        let assign14120_e19515: f64 = (4.0 * 0.001);
        let assign14120_e19517: f64 = (assign14120_e19515 * 0.001);
        let assign14120_e19518: f64 = (assign14120_e19512 + assign14120_e19517);
        let assign14120_e19519: f64 = (assign14120_e19518).sqrt();
        let assign14120_e19520: f64 = (assign14120_e19497 + assign14120_e19519);
        let assign14120_e19521: f64 = (0.5 * assign14120_e19520);
        let assign14120_e19522: f64 = (var_c0sisat_i * assign14120_e19521);
        var_c0sisat_t = assign14120_e19522;
        var_c0sisat_t_dn4 = (var_c0sisat_i * (0.5 * ((var_c0sisat1_i * var_deltemp_dn4) + ((((var_c0sisat1_i * var_deltemp_dn4) * assign14120_e19511) + (assign14120_e19504 * (var_c0sisat1_i * var_deltemp_dn4))) / (2.0 * assign14120_e19519)))));
        var_c0sisat_t_dn5 = (var_c0sisat_i * (0.5 * ((var_c0sisat1_i * var_deltemp_dn5) + ((((var_c0sisat1_i * var_deltemp_dn5) * assign14120_e19511) + (assign14120_e19504 * (var_c0sisat1_i * var_deltemp_dn5))) / (2.0 * assign14120_e19519)))));
        var_c0sisat_t_rv = 0.0;

        let assign14130_e19528: f64 = (p.p1093 * var_deltemp);
        let assign14130_e19529: f64 = (1.0 + assign14130_e19528);
        let assign14130_e19531: f64 = (assign14130_e19529 - 1e-6);
        let assign14130_e19535: f64 = (p.p1093 * var_deltemp);
        let assign14130_e19536: f64 = (1.0 + assign14130_e19535);
        let assign14130_e19538: f64 = (assign14130_e19536 - 1e-6);
        let assign14130_e19542: f64 = (p.p1093 * var_deltemp);
        let assign14130_e19543: f64 = (1.0 + assign14130_e19542);
        let assign14130_e19545: f64 = (assign14130_e19543 - 1e-6);
        let assign14130_e19546: f64 = (assign14130_e19538 * assign14130_e19545);
        let assign14130_e19549: f64 = (4.0 * 0.001);
        let assign14130_e19551: f64 = (assign14130_e19549 * 0.001);
        let assign14130_e19552: f64 = (assign14130_e19546 + assign14130_e19551);
        let assign14130_e19553: f64 = (assign14130_e19552).sqrt();
        let assign14130_e19554: f64 = (assign14130_e19531 + assign14130_e19553);
        let assign14130_e19555: f64 = (0.5 * assign14130_e19554);
        let assign14130_e19556: f64 = (p.p901 * assign14130_e19555);
        var_cjs_t = assign14130_e19556;
        var_cjs_t_dn4 = (p.p901 * (0.5 * ((p.p1093 * var_deltemp_dn4) + ((((p.p1093 * var_deltemp_dn4) * assign14130_e19545) + (assign14130_e19538 * (p.p1093 * var_deltemp_dn4))) / (2.0 * assign14130_e19553)))));
        var_cjs_t_dn5 = (p.p901 * (0.5 * ((p.p1093 * var_deltemp_dn5) + ((((p.p1093 * var_deltemp_dn5) * assign14130_e19545) + (assign14130_e19538 * (p.p1093 * var_deltemp_dn5))) / (2.0 * assign14130_e19553)))));
        var_cjs_t_rv = 0.0;

        let assign14140_e19562: f64 = (p.p1093 * var_deltemp);
        let assign14140_e19563: f64 = (1.0 + assign14140_e19562);
        let assign14140_e19565: f64 = (assign14140_e19563 - 1e-6);
        let assign14140_e19569: f64 = (p.p1093 * var_deltemp);
        let assign14140_e19570: f64 = (1.0 + assign14140_e19569);
        let assign14140_e19572: f64 = (assign14140_e19570 - 1e-6);
        let assign14140_e19576: f64 = (p.p1093 * var_deltemp);
        let assign14140_e19577: f64 = (1.0 + assign14140_e19576);
        let assign14140_e19579: f64 = (assign14140_e19577 - 1e-6);
        let assign14140_e19580: f64 = (assign14140_e19572 * assign14140_e19579);
        let assign14140_e19583: f64 = (4.0 * 0.001);
        let assign14140_e19585: f64 = (assign14140_e19583 * 0.001);
        let assign14140_e19586: f64 = (assign14140_e19580 + assign14140_e19585);
        let assign14140_e19587: f64 = (assign14140_e19586).sqrt();
        let assign14140_e19588: f64 = (assign14140_e19565 + assign14140_e19587);
        let assign14140_e19589: f64 = (0.5 * assign14140_e19588);
        let assign14140_e19590: f64 = (p.p902 * assign14140_e19589);
        var_cjd_t = assign14140_e19590;
        var_cjd_t_dn4 = (p.p902 * (0.5 * ((p.p1093 * var_deltemp_dn4) + ((((p.p1093 * var_deltemp_dn4) * assign14140_e19579) + (assign14140_e19572 * (p.p1093 * var_deltemp_dn4))) / (2.0 * assign14140_e19587)))));
        var_cjd_t_dn5 = (p.p902 * (0.5 * ((p.p1093 * var_deltemp_dn5) + ((((p.p1093 * var_deltemp_dn5) * assign14140_e19579) + (assign14140_e19572 * (p.p1093 * var_deltemp_dn5))) / (2.0 * assign14140_e19587)))));
        var_cjd_t_rv = 0.0;

        let assign14150_e19596: f64 = (p.p1094 * var_deltemp);
        let assign14150_e19597: f64 = (1.0 + assign14150_e19596);
        let assign14150_e19599: f64 = (assign14150_e19597 - 1e-6);
        let assign14150_e19603: f64 = (p.p1094 * var_deltemp);
        let assign14150_e19604: f64 = (1.0 + assign14150_e19603);
        let assign14150_e19606: f64 = (assign14150_e19604 - 1e-6);
        let assign14150_e19610: f64 = (p.p1094 * var_deltemp);
        let assign14150_e19611: f64 = (1.0 + assign14150_e19610);
        let assign14150_e19613: f64 = (assign14150_e19611 - 1e-6);
        let assign14150_e19614: f64 = (assign14150_e19606 * assign14150_e19613);
        let assign14150_e19617: f64 = (4.0 * 0.001);
        let assign14150_e19619: f64 = (assign14150_e19617 * 0.001);
        let assign14150_e19620: f64 = (assign14150_e19614 + assign14150_e19619);
        let assign14150_e19621: f64 = (assign14150_e19620).sqrt();
        let assign14150_e19622: f64 = (assign14150_e19599 + assign14150_e19621);
        let assign14150_e19623: f64 = (0.5 * assign14150_e19622);
        let assign14150_e19624: f64 = (p.p903 * assign14150_e19623);
        var_cjsws_t = assign14150_e19624;
        var_cjsws_t_dn4 = (p.p903 * (0.5 * ((p.p1094 * var_deltemp_dn4) + ((((p.p1094 * var_deltemp_dn4) * assign14150_e19613) + (assign14150_e19606 * (p.p1094 * var_deltemp_dn4))) / (2.0 * assign14150_e19621)))));
        var_cjsws_t_dn5 = (p.p903 * (0.5 * ((p.p1094 * var_deltemp_dn5) + ((((p.p1094 * var_deltemp_dn5) * assign14150_e19613) + (assign14150_e19606 * (p.p1094 * var_deltemp_dn5))) / (2.0 * assign14150_e19621)))));
        var_cjsws_t_rv = 0.0;

        let assign14160_e19630: f64 = (p.p1094 * var_deltemp);
        let assign14160_e19631: f64 = (1.0 + assign14160_e19630);
        let assign14160_e19633: f64 = (assign14160_e19631 - 1e-6);
        let assign14160_e19637: f64 = (p.p1094 * var_deltemp);
        let assign14160_e19638: f64 = (1.0 + assign14160_e19637);
        let assign14160_e19640: f64 = (assign14160_e19638 - 1e-6);
        let assign14160_e19644: f64 = (p.p1094 * var_deltemp);
        let assign14160_e19645: f64 = (1.0 + assign14160_e19644);
        let assign14160_e19647: f64 = (assign14160_e19645 - 1e-6);
        let assign14160_e19648: f64 = (assign14160_e19640 * assign14160_e19647);
        let assign14160_e19651: f64 = (4.0 * 0.001);
        let assign14160_e19653: f64 = (assign14160_e19651 * 0.001);
        let assign14160_e19654: f64 = (assign14160_e19648 + assign14160_e19653);
        let assign14160_e19655: f64 = (assign14160_e19654).sqrt();
        let assign14160_e19656: f64 = (assign14160_e19633 + assign14160_e19655);
        let assign14160_e19657: f64 = (0.5 * assign14160_e19656);
        let assign14160_e19658: f64 = (p.p904 * assign14160_e19657);
        var_cjswd_t = assign14160_e19658;
        var_cjswd_t_dn4 = (p.p904 * (0.5 * ((p.p1094 * var_deltemp_dn4) + ((((p.p1094 * var_deltemp_dn4) * assign14160_e19647) + (assign14160_e19640 * (p.p1094 * var_deltemp_dn4))) / (2.0 * assign14160_e19655)))));
        var_cjswd_t_dn5 = (p.p904 * (0.5 * ((p.p1094 * var_deltemp_dn5) + ((((p.p1094 * var_deltemp_dn5) * assign14160_e19647) + (assign14160_e19640 * (p.p1094 * var_deltemp_dn5))) / (2.0 * assign14160_e19655)))));
        var_cjswd_t_rv = 0.0;

        let assign14170_e19664: f64 = (p.p1095 * var_deltemp);
        let assign14170_e19665: f64 = (1.0 + assign14170_e19664);
        let assign14170_e19667: f64 = (assign14170_e19665 - 1e-6);
        let assign14170_e19671: f64 = (p.p1095 * var_deltemp);
        let assign14170_e19672: f64 = (1.0 + assign14170_e19671);
        let assign14170_e19674: f64 = (assign14170_e19672 - 1e-6);
        let assign14170_e19678: f64 = (p.p1095 * var_deltemp);
        let assign14170_e19679: f64 = (1.0 + assign14170_e19678);
        let assign14170_e19681: f64 = (assign14170_e19679 - 1e-6);
        let assign14170_e19682: f64 = (assign14170_e19674 * assign14170_e19681);
        let assign14170_e19685: f64 = (4.0 * 0.001);
        let assign14170_e19687: f64 = (assign14170_e19685 * 0.001);
        let assign14170_e19688: f64 = (assign14170_e19682 + assign14170_e19687);
        let assign14170_e19689: f64 = (assign14170_e19688).sqrt();
        let assign14170_e19690: f64 = (assign14170_e19667 + assign14170_e19689);
        let assign14170_e19691: f64 = (0.5 * assign14170_e19690);
        let assign14170_e19692: f64 = (p.p905 * assign14170_e19691);
        var_cjswgs_t = assign14170_e19692;
        var_cjswgs_t_dn4 = (p.p905 * (0.5 * ((p.p1095 * var_deltemp_dn4) + ((((p.p1095 * var_deltemp_dn4) * assign14170_e19681) + (assign14170_e19674 * (p.p1095 * var_deltemp_dn4))) / (2.0 * assign14170_e19689)))));
        var_cjswgs_t_dn5 = (p.p905 * (0.5 * ((p.p1095 * var_deltemp_dn5) + ((((p.p1095 * var_deltemp_dn5) * assign14170_e19681) + (assign14170_e19674 * (p.p1095 * var_deltemp_dn5))) / (2.0 * assign14170_e19689)))));
        var_cjswgs_t_rv = 0.0;

        let assign14180_e19698: f64 = (p.p1095 * var_deltemp);
        let assign14180_e19699: f64 = (1.0 + assign14180_e19698);
        let assign14180_e19701: f64 = (assign14180_e19699 - 1e-6);
        let assign14180_e19705: f64 = (p.p1095 * var_deltemp);
        let assign14180_e19706: f64 = (1.0 + assign14180_e19705);
        let assign14180_e19708: f64 = (assign14180_e19706 - 1e-6);
        let assign14180_e19712: f64 = (p.p1095 * var_deltemp);
        let assign14180_e19713: f64 = (1.0 + assign14180_e19712);
        let assign14180_e19715: f64 = (assign14180_e19713 - 1e-6);
        let assign14180_e19716: f64 = (assign14180_e19708 * assign14180_e19715);
        let assign14180_e19719: f64 = (4.0 * 0.001);
        let assign14180_e19721: f64 = (assign14180_e19719 * 0.001);
        let assign14180_e19722: f64 = (assign14180_e19716 + assign14180_e19721);
        let assign14180_e19723: f64 = (assign14180_e19722).sqrt();
        let assign14180_e19724: f64 = (assign14180_e19701 + assign14180_e19723);
        let assign14180_e19725: f64 = (0.5 * assign14180_e19724);
        let assign14180_e19726: f64 = (p.p906 * assign14180_e19725);
        var_cjswgd_t = assign14180_e19726;
        var_cjswgd_t_dn4 = (p.p906 * (0.5 * ((p.p1095 * var_deltemp_dn4) + ((((p.p1095 * var_deltemp_dn4) * assign14180_e19715) + (assign14180_e19708 * (p.p1095 * var_deltemp_dn4))) / (2.0 * assign14180_e19723)))));
        var_cjswgd_t_dn5 = (p.p906 * (0.5 * ((p.p1095 * var_deltemp_dn5) + ((((p.p1095 * var_deltemp_dn5) * assign14180_e19715) + (assign14180_e19708 * (p.p1095 * var_deltemp_dn5))) / (2.0 * assign14180_e19723)))));
        var_cjswgd_t_rv = 0.0;

        let assign14190_e19731: f64 = (p.p1096 * var_deltemp);
        let assign14190_e19732: f64 = (p.p907 - assign14190_e19731);
        let assign14190_e19734: f64 = (assign14190_e19732 - 0.01);
        let assign14190_e19738: f64 = (p.p1096 * var_deltemp);
        let assign14190_e19739: f64 = (p.p907 - assign14190_e19738);
        let assign14190_e19741: f64 = (assign14190_e19739 - 0.01);
        let assign14190_e19745: f64 = (p.p1096 * var_deltemp);
        let assign14190_e19746: f64 = (p.p907 - assign14190_e19745);
        let assign14190_e19748: f64 = (assign14190_e19746 - 0.01);
        let assign14190_e19749: f64 = (assign14190_e19741 * assign14190_e19748);
        let assign14190_e19752: f64 = (4.0 * 0.001);
        let assign14190_e19754: f64 = (assign14190_e19752 * 0.001);
        let assign14190_e19755: f64 = (assign14190_e19749 + assign14190_e19754);
        let assign14190_e19756: f64 = (assign14190_e19755).sqrt();
        let assign14190_e19757: f64 = (assign14190_e19734 + assign14190_e19756);
        let assign14190_e19758: f64 = (0.5 * assign14190_e19757);
        let assign14190_e19760: f64 = (assign14190_e19758 + 0.01);
        var_pbs_t = assign14190_e19760;
        var_pbs_t_dn4 = (0.5 * ((-(p.p1096 * var_deltemp_dn4)) + ((((-(p.p1096 * var_deltemp_dn4)) * assign14190_e19748) + (assign14190_e19741 * (-(p.p1096 * var_deltemp_dn4)))) / (2.0 * assign14190_e19756))));
        var_pbs_t_dn5 = (0.5 * ((-(p.p1096 * var_deltemp_dn5)) + ((((-(p.p1096 * var_deltemp_dn5)) * assign14190_e19748) + (assign14190_e19741 * (-(p.p1096 * var_deltemp_dn5)))) / (2.0 * assign14190_e19756))));
        var_pbs_t_rv = 0.0;

        let assign14200_e19765: f64 = (p.p1096 * var_deltemp);
        let assign14200_e19766: f64 = (p.p908 - assign14200_e19765);
        let assign14200_e19768: f64 = (assign14200_e19766 - 0.01);
        let assign14200_e19772: f64 = (p.p1096 * var_deltemp);
        let assign14200_e19773: f64 = (p.p908 - assign14200_e19772);
        let assign14200_e19775: f64 = (assign14200_e19773 - 0.01);
        let assign14200_e19779: f64 = (p.p1096 * var_deltemp);
        let assign14200_e19780: f64 = (p.p908 - assign14200_e19779);
        let assign14200_e19782: f64 = (assign14200_e19780 - 0.01);
        let assign14200_e19783: f64 = (assign14200_e19775 * assign14200_e19782);
        let assign14200_e19786: f64 = (4.0 * 0.001);
        let assign14200_e19788: f64 = (assign14200_e19786 * 0.001);
        let assign14200_e19789: f64 = (assign14200_e19783 + assign14200_e19788);
        let assign14200_e19790: f64 = (assign14200_e19789).sqrt();
        let assign14200_e19791: f64 = (assign14200_e19768 + assign14200_e19790);
        let assign14200_e19792: f64 = (0.5 * assign14200_e19791);
        let assign14200_e19794: f64 = (assign14200_e19792 + 0.01);
        var_pbd_t = assign14200_e19794;
        var_pbd_t_dn4 = (0.5 * ((-(p.p1096 * var_deltemp_dn4)) + ((((-(p.p1096 * var_deltemp_dn4)) * assign14200_e19782) + (assign14200_e19775 * (-(p.p1096 * var_deltemp_dn4)))) / (2.0 * assign14200_e19790))));
        var_pbd_t_dn5 = (0.5 * ((-(p.p1096 * var_deltemp_dn5)) + ((((-(p.p1096 * var_deltemp_dn5)) * assign14200_e19782) + (assign14200_e19775 * (-(p.p1096 * var_deltemp_dn5)))) / (2.0 * assign14200_e19790))));
        var_pbd_t_rv = 0.0;

        let assign14210_e19799: f64 = (p.p1097 * var_deltemp);
        let assign14210_e19800: f64 = (p.p909 - assign14210_e19799);
        let assign14210_e19802: f64 = (assign14210_e19800 - 0.01);
        let assign14210_e19806: f64 = (p.p1097 * var_deltemp);
        let assign14210_e19807: f64 = (p.p909 - assign14210_e19806);
        let assign14210_e19809: f64 = (assign14210_e19807 - 0.01);
        let assign14210_e19813: f64 = (p.p1097 * var_deltemp);
        let assign14210_e19814: f64 = (p.p909 - assign14210_e19813);
        let assign14210_e19816: f64 = (assign14210_e19814 - 0.01);
        let assign14210_e19817: f64 = (assign14210_e19809 * assign14210_e19816);
        let assign14210_e19820: f64 = (4.0 * 0.001);
        let assign14210_e19822: f64 = (assign14210_e19820 * 0.001);
        let assign14210_e19823: f64 = (assign14210_e19817 + assign14210_e19822);
        let assign14210_e19824: f64 = (assign14210_e19823).sqrt();
        let assign14210_e19825: f64 = (assign14210_e19802 + assign14210_e19824);
        let assign14210_e19826: f64 = (0.5 * assign14210_e19825);
        let assign14210_e19828: f64 = (assign14210_e19826 + 0.01);
        var_pbsws_t = assign14210_e19828;
        var_pbsws_t_dn4 = (0.5 * ((-(p.p1097 * var_deltemp_dn4)) + ((((-(p.p1097 * var_deltemp_dn4)) * assign14210_e19816) + (assign14210_e19809 * (-(p.p1097 * var_deltemp_dn4)))) / (2.0 * assign14210_e19824))));
        var_pbsws_t_dn5 = (0.5 * ((-(p.p1097 * var_deltemp_dn5)) + ((((-(p.p1097 * var_deltemp_dn5)) * assign14210_e19816) + (assign14210_e19809 * (-(p.p1097 * var_deltemp_dn5)))) / (2.0 * assign14210_e19824))));
        var_pbsws_t_rv = 0.0;

        let assign14220_e19833: f64 = (p.p1097 * var_deltemp);
        let assign14220_e19834: f64 = (p.p910 - assign14220_e19833);
        let assign14220_e19836: f64 = (assign14220_e19834 - 0.01);
        let assign14220_e19840: f64 = (p.p1097 * var_deltemp);
        let assign14220_e19841: f64 = (p.p910 - assign14220_e19840);
        let assign14220_e19843: f64 = (assign14220_e19841 - 0.01);
        let assign14220_e19847: f64 = (p.p1097 * var_deltemp);
        let assign14220_e19848: f64 = (p.p910 - assign14220_e19847);
        let assign14220_e19850: f64 = (assign14220_e19848 - 0.01);
        let assign14220_e19851: f64 = (assign14220_e19843 * assign14220_e19850);
        let assign14220_e19854: f64 = (4.0 * 0.001);
        let assign14220_e19856: f64 = (assign14220_e19854 * 0.001);
        let assign14220_e19857: f64 = (assign14220_e19851 + assign14220_e19856);
        let assign14220_e19858: f64 = (assign14220_e19857).sqrt();
        let assign14220_e19859: f64 = (assign14220_e19836 + assign14220_e19858);
        let assign14220_e19860: f64 = (0.5 * assign14220_e19859);
        let assign14220_e19862: f64 = (assign14220_e19860 + 0.01);
        var_pbswd_t = assign14220_e19862;
        var_pbswd_t_dn4 = (0.5 * ((-(p.p1097 * var_deltemp_dn4)) + ((((-(p.p1097 * var_deltemp_dn4)) * assign14220_e19850) + (assign14220_e19843 * (-(p.p1097 * var_deltemp_dn4)))) / (2.0 * assign14220_e19858))));
        var_pbswd_t_dn5 = (0.5 * ((-(p.p1097 * var_deltemp_dn5)) + ((((-(p.p1097 * var_deltemp_dn5)) * assign14220_e19850) + (assign14220_e19843 * (-(p.p1097 * var_deltemp_dn5)))) / (2.0 * assign14220_e19858))));
        var_pbswd_t_rv = 0.0;

        let assign14230_e19867: f64 = (p.p1098 * var_deltemp);
        let assign14230_e19868: f64 = (p.p911 - assign14230_e19867);
        let assign14230_e19870: f64 = (assign14230_e19868 - 0.01);
        let assign14230_e19874: f64 = (p.p1098 * var_deltemp);
        let assign14230_e19875: f64 = (p.p911 - assign14230_e19874);
        let assign14230_e19877: f64 = (assign14230_e19875 - 0.01);
        let assign14230_e19881: f64 = (p.p1098 * var_deltemp);
        let assign14230_e19882: f64 = (p.p911 - assign14230_e19881);
        let assign14230_e19884: f64 = (assign14230_e19882 - 0.01);
        let assign14230_e19885: f64 = (assign14230_e19877 * assign14230_e19884);
        let assign14230_e19888: f64 = (4.0 * 0.001);
        let assign14230_e19890: f64 = (assign14230_e19888 * 0.001);
        let assign14230_e19891: f64 = (assign14230_e19885 + assign14230_e19890);
        let assign14230_e19892: f64 = (assign14230_e19891).sqrt();
        let assign14230_e19893: f64 = (assign14230_e19870 + assign14230_e19892);
        let assign14230_e19894: f64 = (0.5 * assign14230_e19893);
        let assign14230_e19896: f64 = (assign14230_e19894 + 0.01);
        var_pbswgs_t = assign14230_e19896;
        var_pbswgs_t_dn4 = (0.5 * ((-(p.p1098 * var_deltemp_dn4)) + ((((-(p.p1098 * var_deltemp_dn4)) * assign14230_e19884) + (assign14230_e19877 * (-(p.p1098 * var_deltemp_dn4)))) / (2.0 * assign14230_e19892))));
        var_pbswgs_t_dn5 = (0.5 * ((-(p.p1098 * var_deltemp_dn5)) + ((((-(p.p1098 * var_deltemp_dn5)) * assign14230_e19884) + (assign14230_e19877 * (-(p.p1098 * var_deltemp_dn5)))) / (2.0 * assign14230_e19892))));
        var_pbswgs_t_rv = 0.0;

        let assign14240_e19901: f64 = (p.p1098 * var_deltemp);
        let assign14240_e19902: f64 = (p.p912 - assign14240_e19901);
        let assign14240_e19904: f64 = (assign14240_e19902 - 0.01);
        let assign14240_e19908: f64 = (p.p1098 * var_deltemp);
        let assign14240_e19909: f64 = (p.p912 - assign14240_e19908);
        let assign14240_e19911: f64 = (assign14240_e19909 - 0.01);
        let assign14240_e19915: f64 = (p.p1098 * var_deltemp);
        let assign14240_e19916: f64 = (p.p912 - assign14240_e19915);
        let assign14240_e19918: f64 = (assign14240_e19916 - 0.01);
        let assign14240_e19919: f64 = (assign14240_e19911 * assign14240_e19918);
        let assign14240_e19922: f64 = (4.0 * 0.001);
        let assign14240_e19924: f64 = (assign14240_e19922 * 0.001);
        let assign14240_e19925: f64 = (assign14240_e19919 + assign14240_e19924);
        let assign14240_e19926: f64 = (assign14240_e19925).sqrt();
        let assign14240_e19927: f64 = (assign14240_e19904 + assign14240_e19926);
        let assign14240_e19928: f64 = (0.5 * assign14240_e19927);
        let assign14240_e19930: f64 = (assign14240_e19928 + 0.01);
        var_pbswgd_t = assign14240_e19930;
        var_pbswgd_t_dn4 = (0.5 * ((-(p.p1098 * var_deltemp_dn4)) + ((((-(p.p1098 * var_deltemp_dn4)) * assign14240_e19918) + (assign14240_e19911 * (-(p.p1098 * var_deltemp_dn4)))) / (2.0 * assign14240_e19926))));
        var_pbswgd_t_dn5 = (0.5 * ((-(p.p1098 * var_deltemp_dn5)) + ((((-(p.p1098 * var_deltemp_dn5)) * assign14240_e19918) + (assign14240_e19911 * (-(p.p1098 * var_deltemp_dn5)))) / (2.0 * assign14240_e19926))));
        var_pbswgd_t_rv = 0.0;

        let assign14250_e19933: f64 = if p.p8 < 9.0 { 1.0 } else { 0.0 };
        var_guard456 = assign14250_e19933;
        var_guard456_rv = 0.0;

        let assign14260_e19936: f64 = (p.p2 % 2.0);
        let assign14260_e19938: f64 = if assign14260_e19936 != 0.0 { 1.0 } else { 0.0 };
        var_guard457 = assign14260_e19938;
        var_guard457_rv = 0.0;

        let (assign14270_e19944,) = {
    if ((var_guard456 != 0.0) && (var_guard457 != 0.0)) {
        (1.0,)
    } else {
        (var_nuendd,)
    }
};
        var_nuendd = assign14270_e19944;
        var_nuendd_rv = 0.0;

        let (assign14280_e19950,) = {
    if ((var_guard456 != 0.0) && (var_guard457 != 0.0)) {
        (1.0,)
    } else {
        (var_nuends,)
    }
};
        var_nuends = assign14280_e19950;
        var_nuends_rv = 0.0;

        let (assign14290_e19964,) = {
    if ((var_guard456 != 0.0) && (var_guard457 != 0.0)) {
        let assign14290_e19957: f64 = (p.p2 - 1.0);
        let assign14290_e19959: f64 = (assign14290_e19957 / 2.0);
        let assign14290_e19961: f64 = (assign14290_e19959).max(0.0);
        let assign14290_e19962: f64 = (2.0 * assign14290_e19961);
        (assign14290_e19962,)
    } else {
        (var_nuintd,)
    }
};
        var_nuintd = assign14290_e19964;
        var_nuintd_rv = 0.0;

        let (assign14300_e19970,) = {
    if ((var_guard456 != 0.0) && (var_guard457 != 0.0)) {
        (var_nuintd,)
    } else {
        (var_nuints,)
    }
};
        var_nuints = assign14300_e19970;
        var_nuints_rv = 0.0;

        let assign14310_e19973: f64 = if p.p6 == 1.0 { 1.0 } else { 0.0 };
        var_guard458 = assign14310_e19973;
        var_guard458_rv = 0.0;

        let (assign14320_e19982,) = {
    if (((var_guard456 != 0.0) && (var_guard457 == 0.0)) && (var_guard458 != 0.0)) {
        (2.0,)
    } else {
        (var_nuendd,)
    }
};
        var_nuendd = assign14320_e19982;
        var_nuendd_rv = 0.0;

        let (assign14330_e19999,) = {
    if (((var_guard456 != 0.0) && (var_guard457 == 0.0)) && (var_guard458 != 0.0)) {
        let assign14330_e19992: f64 = (p.p2 / 2.0);
        let assign14330_e19994: f64 = (assign14330_e19992 - 1.0);
        let assign14330_e19996: f64 = (assign14330_e19994).max(0.0);
        let assign14330_e19997: f64 = (2.0 * assign14330_e19996);
        (assign14330_e19997,)
    } else {
        (var_nuintd,)
    }
};
        var_nuintd = assign14330_e19999;
        var_nuintd_rv = 0.0;

        let (assign14340_e20008,) = {
    if (((var_guard456 != 0.0) && (var_guard457 == 0.0)) && (var_guard458 != 0.0)) {
        (0.0,)
    } else {
        (var_nuends,)
    }
};
        var_nuends = assign14340_e20008;
        var_nuends_rv = 0.0;

        let (assign14350_e20017,) = {
    if (((var_guard456 != 0.0) && (var_guard457 == 0.0)) && (var_guard458 != 0.0)) {
        (p.p2,)
    } else {
        (var_nuints,)
    }
};
        var_nuints = assign14350_e20017;
        var_nuints_rv = 0.0;

        let (assign14360_e20027,) = {
    if (((var_guard456 != 0.0) && (var_guard457 == 0.0)) && (var_guard458 == 0.0)) {
        (0.0,)
    } else {
        (var_nuendd,)
    }
};
        var_nuendd = assign14360_e20027;
        var_nuendd_rv = 0.0;

        let (assign14370_e20037,) = {
    if (((var_guard456 != 0.0) && (var_guard457 == 0.0)) && (var_guard458 == 0.0)) {
        (p.p2,)
    } else {
        (var_nuintd,)
    }
};
        var_nuintd = assign14370_e20037;
        var_nuintd_rv = 0.0;

        let (assign14380_e20047,) = {
    if (((var_guard456 != 0.0) && (var_guard457 == 0.0)) && (var_guard458 == 0.0)) {
        (2.0,)
    } else {
        (var_nuends,)
    }
};
        var_nuends = assign14380_e20047;
        var_nuends_rv = 0.0;

        let (assign14390_e20065,) = {
    if (((var_guard456 != 0.0) && (var_guard457 == 0.0)) && (var_guard458 == 0.0)) {
        let assign14390_e20058: f64 = (p.p2 / 2.0);
        let assign14390_e20060: f64 = (assign14390_e20058 - 1.0);
        let assign14390_e20062: f64 = (assign14390_e20060).max(0.0);
        let assign14390_e20063: f64 = (2.0 * assign14390_e20062);
        (assign14390_e20063,)
    } else {
        (var_nuints,)
    }
};
        var_nuints = assign14390_e20065;
        var_nuints_rv = 0.0;

        let assign14400_e20068: f64 = (var_dmcgeff + var_dmcieff);
        var_t0 = assign14400_e20068;
        var_t0_dn3 = 0.0;
        var_t0_dn4 = 0.0;
        var_t0_dn5 = 0.0;
        var_t0_dn6 = 0.0;
        var_t0_dn7 = 0.0;
        var_t0_dn8 = 0.0;
        var_t0_dn9 = 0.0;
        var_t0_dn10 = 0.0;
        var_t0_dn11 = 0.0;
        var_t0_rv = 0.0;

        let assign14410_e20071: f64 = (var_dmcgeff + var_dmcgeff);
        var_t1 = assign14410_e20071;
        var_t1_dn3 = 0.0;
        var_t1_dn4 = 0.0;
        var_t1_dn5 = 0.0;
        var_t1_dn6 = 0.0;
        var_t1_dn7 = 0.0;
        var_t1_dn8 = 0.0;
        var_t1_dn9 = 0.0;
        var_t1_dn10 = 0.0;
        var_t1_dn11 = 0.0;
        var_t1_rv = 0.0;

        let assign14420_e20074: f64 = (var_dmdgeff + var_dmdgeff);
        var_t2 = assign14420_e20074;
        var_t2_dn3 = 0.0;
        var_t2_dn4 = 0.0;
        var_t2_dn5 = 0.0;
        var_t2_dn6 = 0.0;
        var_t2_dn7 = 0.0;
        var_t2_dn8 = 0.0;
        var_t2_dn9 = 0.0;
        var_t2_dn10 = 0.0;
        var_t2_dn11 = 0.0;
        var_t2_rv = 0.0;

        let assign14430_e20077: f64 = (var_t0 + var_t0);
        let assign14430_e20079: f64 = (assign14430_e20077 + var_weffcj);
        var_psiso = assign14430_e20079;
        var_psiso_dn3 = (var_t0_dn3 + var_t0_dn3);
        var_psiso_dn4 = (var_t0_dn4 + var_t0_dn4);
        var_psiso_dn5 = (var_t0_dn5 + var_t0_dn5);
        var_psiso_dn6 = (var_t0_dn6 + var_t0_dn6);
        var_psiso_dn7 = (var_t0_dn7 + var_t0_dn7);
        var_psiso_dn8 = (var_t0_dn8 + var_t0_dn8);
        var_psiso_dn9 = (var_t0_dn9 + var_t0_dn9);
        var_psiso_dn10 = (var_t0_dn10 + var_t0_dn10);
        var_psiso_dn11 = (var_t0_dn11 + var_t0_dn11);
        var_psiso_rv = 0.0;

        let assign14440_e20082: f64 = (var_t0 + var_t0);
        let assign14440_e20084: f64 = (assign14440_e20082 + var_weffcj);
        var_pdiso = assign14440_e20084;
        var_pdiso_dn3 = (var_t0_dn3 + var_t0_dn3);
        var_pdiso_dn4 = (var_t0_dn4 + var_t0_dn4);
        var_pdiso_dn5 = (var_t0_dn5 + var_t0_dn5);
        var_pdiso_dn6 = (var_t0_dn6 + var_t0_dn6);
        var_pdiso_dn7 = (var_t0_dn7 + var_t0_dn7);
        var_pdiso_dn8 = (var_t0_dn8 + var_t0_dn8);
        var_pdiso_dn9 = (var_t0_dn9 + var_t0_dn9);
        var_pdiso_dn10 = (var_t0_dn10 + var_t0_dn10);
        var_pdiso_dn11 = (var_t0_dn11 + var_t0_dn11);
        var_pdiso_rv = 0.0;

        var_pssha = var_t1;
        var_pssha_dn3 = var_t1_dn3;
        var_pssha_dn4 = var_t1_dn4;
        var_pssha_dn5 = var_t1_dn5;
        var_pssha_dn6 = var_t1_dn6;
        var_pssha_dn7 = var_t1_dn7;
        var_pssha_dn8 = var_t1_dn8;
        var_pssha_dn9 = var_t1_dn9;
        var_pssha_dn10 = var_t1_dn10;
        var_pssha_dn11 = var_t1_dn11;
        var_pssha_rv = 0.0;

        *var_c0sisat_t_slot = var_c0sisat_t;
        *var_c0sisat_t_dn4_slot = var_c0sisat_t_dn4;
        *var_c0sisat_t_dn5_slot = var_c0sisat_t_dn5;
        *var_c0sisat_t_rv_slot = var_c0sisat_t_rv;
        *var_cjd_t_slot = var_cjd_t;
        *var_cjd_t_dn4_slot = var_cjd_t_dn4;
        *var_cjd_t_dn5_slot = var_cjd_t_dn5;
        *var_cjd_t_rv_slot = var_cjd_t_rv;
        *var_cjs_t_slot = var_cjs_t;
        *var_cjs_t_dn4_slot = var_cjs_t_dn4;
        *var_cjs_t_dn5_slot = var_cjs_t_dn5;
        *var_cjs_t_rv_slot = var_cjs_t_rv;
        *var_cjswd_t_slot = var_cjswd_t;
        *var_cjswd_t_dn4_slot = var_cjswd_t_dn4;
        *var_cjswd_t_dn5_slot = var_cjswd_t_dn5;
        *var_cjswd_t_rv_slot = var_cjswd_t_rv;
        *var_cjswgd_t_slot = var_cjswgd_t;
        *var_cjswgd_t_dn4_slot = var_cjswgd_t_dn4;
        *var_cjswgd_t_dn5_slot = var_cjswgd_t_dn5;
        *var_cjswgd_t_rv_slot = var_cjswgd_t_rv;
        *var_cjswgs_t_slot = var_cjswgs_t;
        *var_cjswgs_t_dn4_slot = var_cjswgs_t_dn4;
        *var_cjswgs_t_dn5_slot = var_cjswgs_t_dn5;
        *var_cjswgs_t_rv_slot = var_cjswgs_t_rv;
        *var_cjsws_t_slot = var_cjsws_t;
        *var_cjsws_t_dn4_slot = var_cjsws_t_dn4;
        *var_cjsws_t_dn5_slot = var_cjsws_t_dn5;
        *var_cjsws_t_rv_slot = var_cjsws_t_rv;
        *var_guard456_slot = var_guard456;
        *var_guard456_rv_slot = var_guard456_rv;
        *var_guard457_slot = var_guard457;
        *var_guard457_rv_slot = var_guard457_rv;
        *var_guard458_slot = var_guard458;
        *var_guard458_rv_slot = var_guard458_rv;
        *var_nuendd_slot = var_nuendd;
        *var_nuendd_rv_slot = var_nuendd_rv;
        *var_nuends_slot = var_nuends;
        *var_nuends_rv_slot = var_nuends_rv;
        *var_nuintd_slot = var_nuintd;
        *var_nuintd_rv_slot = var_nuintd_rv;
        *var_nuints_slot = var_nuints;
        *var_nuints_rv_slot = var_nuints_rv;
        *var_pbd_t_slot = var_pbd_t;
        *var_pbd_t_dn4_slot = var_pbd_t_dn4;
        *var_pbd_t_dn5_slot = var_pbd_t_dn5;
        *var_pbd_t_rv_slot = var_pbd_t_rv;
        *var_pbs_t_slot = var_pbs_t;
        *var_pbs_t_dn4_slot = var_pbs_t_dn4;
        *var_pbs_t_dn5_slot = var_pbs_t_dn5;
        *var_pbs_t_rv_slot = var_pbs_t_rv;
        *var_pbswd_t_slot = var_pbswd_t;
        *var_pbswd_t_dn4_slot = var_pbswd_t_dn4;
        *var_pbswd_t_dn5_slot = var_pbswd_t_dn5;
        *var_pbswd_t_rv_slot = var_pbswd_t_rv;
        *var_pbswgd_t_slot = var_pbswgd_t;
        *var_pbswgd_t_dn4_slot = var_pbswgd_t_dn4;
        *var_pbswgd_t_dn5_slot = var_pbswgd_t_dn5;
        *var_pbswgd_t_rv_slot = var_pbswgd_t_rv;
        *var_pbswgs_t_slot = var_pbswgs_t;
        *var_pbswgs_t_dn4_slot = var_pbswgs_t_dn4;
        *var_pbswgs_t_dn5_slot = var_pbswgs_t_dn5;
        *var_pbswgs_t_rv_slot = var_pbswgs_t_rv;
        *var_pbsws_t_slot = var_pbsws_t;
        *var_pbsws_t_dn4_slot = var_pbsws_t_dn4;
        *var_pbsws_t_dn5_slot = var_pbsws_t_dn5;
        *var_pbsws_t_rv_slot = var_pbsws_t_rv;
        *var_pdiso_slot = var_pdiso;
        *var_pdiso_dn10_slot = var_pdiso_dn10;
        *var_pdiso_dn11_slot = var_pdiso_dn11;
        *var_pdiso_dn3_slot = var_pdiso_dn3;
        *var_pdiso_dn4_slot = var_pdiso_dn4;
        *var_pdiso_dn5_slot = var_pdiso_dn5;
        *var_pdiso_dn6_slot = var_pdiso_dn6;
        *var_pdiso_dn7_slot = var_pdiso_dn7;
        *var_pdiso_dn8_slot = var_pdiso_dn8;
        *var_pdiso_dn9_slot = var_pdiso_dn9;
        *var_pdiso_rv_slot = var_pdiso_rv;
        *var_psiso_slot = var_psiso;
        *var_psiso_dn10_slot = var_psiso_dn10;
        *var_psiso_dn11_slot = var_psiso_dn11;
        *var_psiso_dn3_slot = var_psiso_dn3;
        *var_psiso_dn4_slot = var_psiso_dn4;
        *var_psiso_dn5_slot = var_psiso_dn5;
        *var_psiso_dn6_slot = var_psiso_dn6;
        *var_psiso_dn7_slot = var_psiso_dn7;
        *var_psiso_dn8_slot = var_psiso_dn8;
        *var_psiso_dn9_slot = var_psiso_dn9;
        *var_psiso_rv_slot = var_psiso_rv;
        *var_pssha_slot = var_pssha;
        *var_pssha_dn10_slot = var_pssha_dn10;
        *var_pssha_dn11_slot = var_pssha_dn11;
        *var_pssha_dn3_slot = var_pssha_dn3;
        *var_pssha_dn4_slot = var_pssha_dn4;
        *var_pssha_dn5_slot = var_pssha_dn5;
        *var_pssha_dn6_slot = var_pssha_dn6;
        *var_pssha_dn7_slot = var_pssha_dn7;
        *var_pssha_dn8_slot = var_pssha_dn8;
        *var_pssha_dn9_slot = var_pssha_dn9;
        *var_pssha_rv_slot = var_pssha_rv;
        *var_t0_slot = var_t0;
        *var_t0_dn10_slot = var_t0_dn10;
        *var_t0_dn11_slot = var_t0_dn11;
        *var_t0_dn3_slot = var_t0_dn3;
        *var_t0_dn4_slot = var_t0_dn4;
        *var_t0_dn5_slot = var_t0_dn5;
        *var_t0_dn6_slot = var_t0_dn6;
        *var_t0_dn7_slot = var_t0_dn7;
        *var_t0_dn8_slot = var_t0_dn8;
        *var_t0_dn9_slot = var_t0_dn9;
        *var_t0_rv_slot = var_t0_rv;
        *var_t1_slot = var_t1;
        *var_t1_dn10_slot = var_t1_dn10;
        *var_t1_dn11_slot = var_t1_dn11;
        *var_t1_dn3_slot = var_t1_dn3;
        *var_t1_dn4_slot = var_t1_dn4;
        *var_t1_dn5_slot = var_t1_dn5;
        *var_t1_dn6_slot = var_t1_dn6;
        *var_t1_dn7_slot = var_t1_dn7;
        *var_t1_dn8_slot = var_t1_dn8;
        *var_t1_dn9_slot = var_t1_dn9;
        *var_t1_rv_slot = var_t1_rv;
        *var_t2_slot = var_t2;
        *var_t2_dn10_slot = var_t2_dn10;
        *var_t2_dn11_slot = var_t2_dn11;
        *var_t2_dn3_slot = var_t2_dn3;
        *var_t2_dn4_slot = var_t2_dn4;
        *var_t2_dn5_slot = var_t2_dn5;
        *var_t2_dn6_slot = var_t2_dn6;
        *var_t2_dn7_slot = var_t2_dn7;
        *var_t2_dn8_slot = var_t2_dn8;
        *var_t2_dn9_slot = var_t2_dn9;
        *var_t2_rv_slot = var_t2_rv;
    }

    pub(super) fn stamp_reactive_block_25(
        p: &Parameters,
        var_dmcgeff: f64,
        var_dmdgeff: f64,
        var_nuendd: f64,
        var_nuends: f64,
        var_nuintd: f64,
        var_nuints: f64,
        var_pdiso: f64,
        var_pdiso_dn10: f64,
        var_pdiso_dn11: f64,
        var_pdiso_dn3: f64,
        var_pdiso_dn4: f64,
        var_pdiso_dn5: f64,
        var_pdiso_dn6: f64,
        var_pdiso_dn7: f64,
        var_pdiso_dn8: f64,
        var_pdiso_dn9: f64,
        var_psiso: f64,
        var_psiso_dn10: f64,
        var_psiso_dn11: f64,
        var_psiso_dn3: f64,
        var_psiso_dn4: f64,
        var_psiso_dn5: f64,
        var_psiso_dn6: f64,
        var_psiso_dn7: f64,
        var_psiso_dn8: f64,
        var_psiso_dn9: f64,
        var_pssha: f64,
        var_pssha_dn10: f64,
        var_pssha_dn11: f64,
        var_pssha_dn3: f64,
        var_pssha_dn4: f64,
        var_pssha_dn5: f64,
        var_pssha_dn6: f64,
        var_pssha_dn7: f64,
        var_pssha_dn8: f64,
        var_pssha_dn9: f64,
        var_t0: f64,
        var_t0_dn10: f64,
        var_t0_dn11: f64,
        var_t0_dn3: f64,
        var_t0_dn4: f64,
        var_t0_dn5: f64,
        var_t0_dn6: f64,
        var_t0_dn7: f64,
        var_t0_dn8: f64,
        var_t0_dn9: f64,
        var_t1: f64,
        var_t1_dn10: f64,
        var_t1_dn11: f64,
        var_t1_dn3: f64,
        var_t1_dn4: f64,
        var_t1_dn5: f64,
        var_t1_dn6: f64,
        var_t1_dn7: f64,
        var_t1_dn8: f64,
        var_t1_dn9: f64,
        var_t2: f64,
        var_t2_dn10: f64,
        var_t2_dn11: f64,
        var_t2_dn3: f64,
        var_t2_dn4: f64,
        var_t2_dn5: f64,
        var_t2_dn6: f64,
        var_t2_dn7: f64,
        var_t2_dn8: f64,
        var_t2_dn9: f64,
        var_weffcj: f64,
        var_adiso_slot: &mut f64,
        var_adiso_dn10_slot: &mut f64,
        var_adiso_dn11_slot: &mut f64,
        var_adiso_dn3_slot: &mut f64,
        var_adiso_dn4_slot: &mut f64,
        var_adiso_dn5_slot: &mut f64,
        var_adiso_dn6_slot: &mut f64,
        var_adiso_dn7_slot: &mut f64,
        var_adiso_dn8_slot: &mut f64,
        var_adiso_dn9_slot: &mut f64,
        var_adiso_rv_slot: &mut f64,
        var_admer_slot: &mut f64,
        var_admer_rv_slot: &mut f64,
        var_adsha_slot: &mut f64,
        var_adsha_rv_slot: &mut f64,
        var_asiso_slot: &mut f64,
        var_asiso_dn10_slot: &mut f64,
        var_asiso_dn11_slot: &mut f64,
        var_asiso_dn3_slot: &mut f64,
        var_asiso_dn4_slot: &mut f64,
        var_asiso_dn5_slot: &mut f64,
        var_asiso_dn6_slot: &mut f64,
        var_asiso_dn7_slot: &mut f64,
        var_asiso_dn8_slot: &mut f64,
        var_asiso_dn9_slot: &mut f64,
        var_asiso_rv_slot: &mut f64,
        var_asmer_slot: &mut f64,
        var_asmer_rv_slot: &mut f64,
        var_assha_slot: &mut f64,
        var_assha_rv_slot: &mut f64,
        var_guard459_slot: &mut f64,
        var_guard459_rv_slot: &mut f64,
        var_guard460_slot: &mut f64,
        var_guard460_rv_slot: &mut f64,
        var_guard461_slot: &mut f64,
        var_guard461_rv_slot: &mut f64,
        var_guard462_slot: &mut f64,
        var_guard462_rv_slot: &mut f64,
        var_guard463_slot: &mut f64,
        var_guard463_rv_slot: &mut f64,
        var_guard464_slot: &mut f64,
        var_guard464_rv_slot: &mut f64,
        var_guard465_slot: &mut f64,
        var_guard465_rv_slot: &mut f64,
        var_guard466_slot: &mut f64,
        var_guard466_rv_slot: &mut f64,
        var_guard467_slot: &mut f64,
        var_guard467_rv_slot: &mut f64,
        var_guard468_slot: &mut f64,
        var_guard468_rv_slot: &mut f64,
        var_guard469_slot: &mut f64,
        var_guard469_rv_slot: &mut f64,
        var_pdmer_slot: &mut f64,
        var_pdmer_dn10_slot: &mut f64,
        var_pdmer_dn11_slot: &mut f64,
        var_pdmer_dn3_slot: &mut f64,
        var_pdmer_dn4_slot: &mut f64,
        var_pdmer_dn5_slot: &mut f64,
        var_pdmer_dn6_slot: &mut f64,
        var_pdmer_dn7_slot: &mut f64,
        var_pdmer_dn8_slot: &mut f64,
        var_pdmer_dn9_slot: &mut f64,
        var_pdmer_rv_slot: &mut f64,
        var_pdsha_slot: &mut f64,
        var_pdsha_dn10_slot: &mut f64,
        var_pdsha_dn11_slot: &mut f64,
        var_pdsha_dn3_slot: &mut f64,
        var_pdsha_dn4_slot: &mut f64,
        var_pdsha_dn5_slot: &mut f64,
        var_pdsha_dn6_slot: &mut f64,
        var_pdsha_dn7_slot: &mut f64,
        var_pdsha_dn8_slot: &mut f64,
        var_pdsha_dn9_slot: &mut f64,
        var_pdsha_rv_slot: &mut f64,
        var_psmer_slot: &mut f64,
        var_psmer_dn10_slot: &mut f64,
        var_psmer_dn11_slot: &mut f64,
        var_psmer_dn3_slot: &mut f64,
        var_psmer_dn4_slot: &mut f64,
        var_psmer_dn5_slot: &mut f64,
        var_psmer_dn6_slot: &mut f64,
        var_psmer_dn7_slot: &mut f64,
        var_psmer_dn8_slot: &mut f64,
        var_psmer_dn9_slot: &mut f64,
        var_psmer_rv_slot: &mut f64,
        var_temp_adeff_slot: &mut f64,
        var_temp_adeff_dn10_slot: &mut f64,
        var_temp_adeff_dn11_slot: &mut f64,
        var_temp_adeff_dn3_slot: &mut f64,
        var_temp_adeff_dn4_slot: &mut f64,
        var_temp_adeff_dn5_slot: &mut f64,
        var_temp_adeff_dn6_slot: &mut f64,
        var_temp_adeff_dn7_slot: &mut f64,
        var_temp_adeff_dn8_slot: &mut f64,
        var_temp_adeff_dn9_slot: &mut f64,
        var_temp_adeff_rv_slot: &mut f64,
        var_temp_aseff_slot: &mut f64,
        var_temp_aseff_dn10_slot: &mut f64,
        var_temp_aseff_dn11_slot: &mut f64,
        var_temp_aseff_dn3_slot: &mut f64,
        var_temp_aseff_dn4_slot: &mut f64,
        var_temp_aseff_dn5_slot: &mut f64,
        var_temp_aseff_dn6_slot: &mut f64,
        var_temp_aseff_dn7_slot: &mut f64,
        var_temp_aseff_dn8_slot: &mut f64,
        var_temp_aseff_dn9_slot: &mut f64,
        var_temp_aseff_rv_slot: &mut f64,
        var_temp_pdeff_slot: &mut f64,
        var_temp_pdeff_dn10_slot: &mut f64,
        var_temp_pdeff_dn11_slot: &mut f64,
        var_temp_pdeff_dn3_slot: &mut f64,
        var_temp_pdeff_dn4_slot: &mut f64,
        var_temp_pdeff_dn5_slot: &mut f64,
        var_temp_pdeff_dn6_slot: &mut f64,
        var_temp_pdeff_dn7_slot: &mut f64,
        var_temp_pdeff_dn8_slot: &mut f64,
        var_temp_pdeff_dn9_slot: &mut f64,
        var_temp_pdeff_rv_slot: &mut f64,
        var_temp_pseff_slot: &mut f64,
        var_temp_pseff_dn10_slot: &mut f64,
        var_temp_pseff_dn11_slot: &mut f64,
        var_temp_pseff_dn3_slot: &mut f64,
        var_temp_pseff_dn4_slot: &mut f64,
        var_temp_pseff_dn5_slot: &mut f64,
        var_temp_pseff_dn6_slot: &mut f64,
        var_temp_pseff_dn7_slot: &mut f64,
        var_temp_pseff_dn8_slot: &mut f64,
        var_temp_pseff_dn9_slot: &mut f64,
        var_temp_pseff_rv_slot: &mut f64,
    ) {
        let mut var_adiso: f64 = *var_adiso_slot;
        let mut var_adiso_dn10: f64 = *var_adiso_dn10_slot;
        let mut var_adiso_dn11: f64 = *var_adiso_dn11_slot;
        let mut var_adiso_dn3: f64 = *var_adiso_dn3_slot;
        let mut var_adiso_dn4: f64 = *var_adiso_dn4_slot;
        let mut var_adiso_dn5: f64 = *var_adiso_dn5_slot;
        let mut var_adiso_dn6: f64 = *var_adiso_dn6_slot;
        let mut var_adiso_dn7: f64 = *var_adiso_dn7_slot;
        let mut var_adiso_dn8: f64 = *var_adiso_dn8_slot;
        let mut var_adiso_dn9: f64 = *var_adiso_dn9_slot;
        let mut var_adiso_rv: f64 = *var_adiso_rv_slot;
        let mut var_admer: f64 = *var_admer_slot;
        let mut var_admer_rv: f64 = *var_admer_rv_slot;
        let mut var_adsha: f64 = *var_adsha_slot;
        let mut var_adsha_rv: f64 = *var_adsha_rv_slot;
        let mut var_asiso: f64 = *var_asiso_slot;
        let mut var_asiso_dn10: f64 = *var_asiso_dn10_slot;
        let mut var_asiso_dn11: f64 = *var_asiso_dn11_slot;
        let mut var_asiso_dn3: f64 = *var_asiso_dn3_slot;
        let mut var_asiso_dn4: f64 = *var_asiso_dn4_slot;
        let mut var_asiso_dn5: f64 = *var_asiso_dn5_slot;
        let mut var_asiso_dn6: f64 = *var_asiso_dn6_slot;
        let mut var_asiso_dn7: f64 = *var_asiso_dn7_slot;
        let mut var_asiso_dn8: f64 = *var_asiso_dn8_slot;
        let mut var_asiso_dn9: f64 = *var_asiso_dn9_slot;
        let mut var_asiso_rv: f64 = *var_asiso_rv_slot;
        let mut var_asmer: f64 = *var_asmer_slot;
        let mut var_asmer_rv: f64 = *var_asmer_rv_slot;
        let mut var_assha: f64 = *var_assha_slot;
        let mut var_assha_rv: f64 = *var_assha_rv_slot;
        let mut var_guard459: f64 = *var_guard459_slot;
        let mut var_guard459_rv: f64 = *var_guard459_rv_slot;
        let mut var_guard460: f64 = *var_guard460_slot;
        let mut var_guard460_rv: f64 = *var_guard460_rv_slot;
        let mut var_guard461: f64 = *var_guard461_slot;
        let mut var_guard461_rv: f64 = *var_guard461_rv_slot;
        let mut var_guard462: f64 = *var_guard462_slot;
        let mut var_guard462_rv: f64 = *var_guard462_rv_slot;
        let mut var_guard463: f64 = *var_guard463_slot;
        let mut var_guard463_rv: f64 = *var_guard463_rv_slot;
        let mut var_guard464: f64 = *var_guard464_slot;
        let mut var_guard464_rv: f64 = *var_guard464_rv_slot;
        let mut var_guard465: f64 = *var_guard465_slot;
        let mut var_guard465_rv: f64 = *var_guard465_rv_slot;
        let mut var_guard466: f64 = *var_guard466_slot;
        let mut var_guard466_rv: f64 = *var_guard466_rv_slot;
        let mut var_guard467: f64 = *var_guard467_slot;
        let mut var_guard467_rv: f64 = *var_guard467_rv_slot;
        let mut var_guard468: f64 = *var_guard468_slot;
        let mut var_guard468_rv: f64 = *var_guard468_rv_slot;
        let mut var_guard469: f64 = *var_guard469_slot;
        let mut var_guard469_rv: f64 = *var_guard469_rv_slot;
        let mut var_pdmer: f64 = *var_pdmer_slot;
        let mut var_pdmer_dn10: f64 = *var_pdmer_dn10_slot;
        let mut var_pdmer_dn11: f64 = *var_pdmer_dn11_slot;
        let mut var_pdmer_dn3: f64 = *var_pdmer_dn3_slot;
        let mut var_pdmer_dn4: f64 = *var_pdmer_dn4_slot;
        let mut var_pdmer_dn5: f64 = *var_pdmer_dn5_slot;
        let mut var_pdmer_dn6: f64 = *var_pdmer_dn6_slot;
        let mut var_pdmer_dn7: f64 = *var_pdmer_dn7_slot;
        let mut var_pdmer_dn8: f64 = *var_pdmer_dn8_slot;
        let mut var_pdmer_dn9: f64 = *var_pdmer_dn9_slot;
        let mut var_pdmer_rv: f64 = *var_pdmer_rv_slot;
        let mut var_pdsha: f64 = *var_pdsha_slot;
        let mut var_pdsha_dn10: f64 = *var_pdsha_dn10_slot;
        let mut var_pdsha_dn11: f64 = *var_pdsha_dn11_slot;
        let mut var_pdsha_dn3: f64 = *var_pdsha_dn3_slot;
        let mut var_pdsha_dn4: f64 = *var_pdsha_dn4_slot;
        let mut var_pdsha_dn5: f64 = *var_pdsha_dn5_slot;
        let mut var_pdsha_dn6: f64 = *var_pdsha_dn6_slot;
        let mut var_pdsha_dn7: f64 = *var_pdsha_dn7_slot;
        let mut var_pdsha_dn8: f64 = *var_pdsha_dn8_slot;
        let mut var_pdsha_dn9: f64 = *var_pdsha_dn9_slot;
        let mut var_pdsha_rv: f64 = *var_pdsha_rv_slot;
        let mut var_psmer: f64 = *var_psmer_slot;
        let mut var_psmer_dn10: f64 = *var_psmer_dn10_slot;
        let mut var_psmer_dn11: f64 = *var_psmer_dn11_slot;
        let mut var_psmer_dn3: f64 = *var_psmer_dn3_slot;
        let mut var_psmer_dn4: f64 = *var_psmer_dn4_slot;
        let mut var_psmer_dn5: f64 = *var_psmer_dn5_slot;
        let mut var_psmer_dn6: f64 = *var_psmer_dn6_slot;
        let mut var_psmer_dn7: f64 = *var_psmer_dn7_slot;
        let mut var_psmer_dn8: f64 = *var_psmer_dn8_slot;
        let mut var_psmer_dn9: f64 = *var_psmer_dn9_slot;
        let mut var_psmer_rv: f64 = *var_psmer_rv_slot;
        let mut var_temp_adeff: f64 = *var_temp_adeff_slot;
        let mut var_temp_adeff_dn10: f64 = *var_temp_adeff_dn10_slot;
        let mut var_temp_adeff_dn11: f64 = *var_temp_adeff_dn11_slot;
        let mut var_temp_adeff_dn3: f64 = *var_temp_adeff_dn3_slot;
        let mut var_temp_adeff_dn4: f64 = *var_temp_adeff_dn4_slot;
        let mut var_temp_adeff_dn5: f64 = *var_temp_adeff_dn5_slot;
        let mut var_temp_adeff_dn6: f64 = *var_temp_adeff_dn6_slot;
        let mut var_temp_adeff_dn7: f64 = *var_temp_adeff_dn7_slot;
        let mut var_temp_adeff_dn8: f64 = *var_temp_adeff_dn8_slot;
        let mut var_temp_adeff_dn9: f64 = *var_temp_adeff_dn9_slot;
        let mut var_temp_adeff_rv: f64 = *var_temp_adeff_rv_slot;
        let mut var_temp_aseff: f64 = *var_temp_aseff_slot;
        let mut var_temp_aseff_dn10: f64 = *var_temp_aseff_dn10_slot;
        let mut var_temp_aseff_dn11: f64 = *var_temp_aseff_dn11_slot;
        let mut var_temp_aseff_dn3: f64 = *var_temp_aseff_dn3_slot;
        let mut var_temp_aseff_dn4: f64 = *var_temp_aseff_dn4_slot;
        let mut var_temp_aseff_dn5: f64 = *var_temp_aseff_dn5_slot;
        let mut var_temp_aseff_dn6: f64 = *var_temp_aseff_dn6_slot;
        let mut var_temp_aseff_dn7: f64 = *var_temp_aseff_dn7_slot;
        let mut var_temp_aseff_dn8: f64 = *var_temp_aseff_dn8_slot;
        let mut var_temp_aseff_dn9: f64 = *var_temp_aseff_dn9_slot;
        let mut var_temp_aseff_rv: f64 = *var_temp_aseff_rv_slot;
        let mut var_temp_pdeff: f64 = *var_temp_pdeff_slot;
        let mut var_temp_pdeff_dn10: f64 = *var_temp_pdeff_dn10_slot;
        let mut var_temp_pdeff_dn11: f64 = *var_temp_pdeff_dn11_slot;
        let mut var_temp_pdeff_dn3: f64 = *var_temp_pdeff_dn3_slot;
        let mut var_temp_pdeff_dn4: f64 = *var_temp_pdeff_dn4_slot;
        let mut var_temp_pdeff_dn5: f64 = *var_temp_pdeff_dn5_slot;
        let mut var_temp_pdeff_dn6: f64 = *var_temp_pdeff_dn6_slot;
        let mut var_temp_pdeff_dn7: f64 = *var_temp_pdeff_dn7_slot;
        let mut var_temp_pdeff_dn8: f64 = *var_temp_pdeff_dn8_slot;
        let mut var_temp_pdeff_dn9: f64 = *var_temp_pdeff_dn9_slot;
        let mut var_temp_pdeff_rv: f64 = *var_temp_pdeff_rv_slot;
        let mut var_temp_pseff: f64 = *var_temp_pseff_slot;
        let mut var_temp_pseff_dn10: f64 = *var_temp_pseff_dn10_slot;
        let mut var_temp_pseff_dn11: f64 = *var_temp_pseff_dn11_slot;
        let mut var_temp_pseff_dn3: f64 = *var_temp_pseff_dn3_slot;
        let mut var_temp_pseff_dn4: f64 = *var_temp_pseff_dn4_slot;
        let mut var_temp_pseff_dn5: f64 = *var_temp_pseff_dn5_slot;
        let mut var_temp_pseff_dn6: f64 = *var_temp_pseff_dn6_slot;
        let mut var_temp_pseff_dn7: f64 = *var_temp_pseff_dn7_slot;
        let mut var_temp_pseff_dn8: f64 = *var_temp_pseff_dn8_slot;
        let mut var_temp_pseff_dn9: f64 = *var_temp_pseff_dn9_slot;
        let mut var_temp_pseff_rv: f64 = *var_temp_pseff_rv_slot;

        var_pdsha = var_t1;
        var_pdsha_dn3 = var_t1_dn3;
        var_pdsha_dn4 = var_t1_dn4;
        var_pdsha_dn5 = var_t1_dn5;
        var_pdsha_dn6 = var_t1_dn6;
        var_pdsha_dn7 = var_t1_dn7;
        var_pdsha_dn8 = var_t1_dn8;
        var_pdsha_dn9 = var_t1_dn9;
        var_pdsha_dn10 = var_t1_dn10;
        var_pdsha_dn11 = var_t1_dn11;
        var_pdsha_rv = 0.0;

        var_psmer = var_t2;
        var_psmer_dn3 = var_t2_dn3;
        var_psmer_dn4 = var_t2_dn4;
        var_psmer_dn5 = var_t2_dn5;
        var_psmer_dn6 = var_t2_dn6;
        var_psmer_dn7 = var_t2_dn7;
        var_psmer_dn8 = var_t2_dn8;
        var_psmer_dn9 = var_t2_dn9;
        var_psmer_dn10 = var_t2_dn10;
        var_psmer_dn11 = var_t2_dn11;
        var_psmer_rv = 0.0;

        var_pdmer = var_t2;
        var_pdmer_dn3 = var_t2_dn3;
        var_pdmer_dn4 = var_t2_dn4;
        var_pdmer_dn5 = var_t2_dn5;
        var_pdmer_dn6 = var_t2_dn6;
        var_pdmer_dn7 = var_t2_dn7;
        var_pdmer_dn8 = var_t2_dn8;
        var_pdmer_dn9 = var_t2_dn9;
        var_pdmer_dn10 = var_t2_dn10;
        var_pdmer_dn11 = var_t2_dn11;
        var_pdmer_rv = 0.0;

        let assign14490_e20091: f64 = (var_t0 * var_weffcj);
        var_asiso = assign14490_e20091;
        var_asiso_dn3 = (var_t0_dn3 * var_weffcj);
        var_asiso_dn4 = (var_t0_dn4 * var_weffcj);
        var_asiso_dn5 = (var_t0_dn5 * var_weffcj);
        var_asiso_dn6 = (var_t0_dn6 * var_weffcj);
        var_asiso_dn7 = (var_t0_dn7 * var_weffcj);
        var_asiso_dn8 = (var_t0_dn8 * var_weffcj);
        var_asiso_dn9 = (var_t0_dn9 * var_weffcj);
        var_asiso_dn10 = (var_t0_dn10 * var_weffcj);
        var_asiso_dn11 = (var_t0_dn11 * var_weffcj);
        var_asiso_rv = 0.0;

        let assign14500_e20094: f64 = (var_t0 * var_weffcj);
        var_adiso = assign14500_e20094;
        var_adiso_dn3 = (var_t0_dn3 * var_weffcj);
        var_adiso_dn4 = (var_t0_dn4 * var_weffcj);
        var_adiso_dn5 = (var_t0_dn5 * var_weffcj);
        var_adiso_dn6 = (var_t0_dn6 * var_weffcj);
        var_adiso_dn7 = (var_t0_dn7 * var_weffcj);
        var_adiso_dn8 = (var_t0_dn8 * var_weffcj);
        var_adiso_dn9 = (var_t0_dn9 * var_weffcj);
        var_adiso_dn10 = (var_t0_dn10 * var_weffcj);
        var_adiso_dn11 = (var_t0_dn11 * var_weffcj);
        var_adiso_rv = 0.0;

        let assign14510_e20097: f64 = (var_dmcgeff * var_weffcj);
        var_assha = assign14510_e20097;
        var_assha_rv = 0.0;

        let assign14520_e20100: f64 = (var_dmcgeff * var_weffcj);
        var_adsha = assign14520_e20100;
        var_adsha_rv = 0.0;

        let assign14530_e20103: f64 = (var_dmdgeff * var_weffcj);
        var_asmer = assign14530_e20103;
        var_asmer_rv = 0.0;

        let assign14540_e20106: f64 = (var_dmdgeff * var_weffcj);
        var_admer = assign14540_e20106;
        var_admer_rv = 0.0;

        let assign14550_e20109: f64 = if p.p8 == 0.0 { 1.0 } else { 0.0 };
        var_guard459 = assign14550_e20109;
        var_guard459_rv = 0.0;

        let assign14560_e20112: f64 = if p.p8 == 1.0 { 1.0 } else { 0.0 };
        var_guard460 = assign14560_e20112;
        var_guard460_rv = 0.0;

        let assign14570_e20115: f64 = if p.p8 == 2.0 { 1.0 } else { 0.0 };
        var_guard461 = assign14570_e20115;
        var_guard461_rv = 0.0;

        let assign14580_e20118: f64 = if p.p8 == 3.0 { 1.0 } else { 0.0 };
        var_guard462 = assign14580_e20118;
        var_guard462_rv = 0.0;

        let assign14590_e20121: f64 = if p.p8 == 4.0 { 1.0 } else { 0.0 };
        var_guard463 = assign14590_e20121;
        var_guard463_rv = 0.0;

        let assign14600_e20124: f64 = if p.p8 == 5.0 { 1.0 } else { 0.0 };
        var_guard464 = assign14600_e20124;
        var_guard464_rv = 0.0;

        let assign14610_e20127: f64 = if p.p8 == 6.0 { 1.0 } else { 0.0 };
        var_guard465 = assign14610_e20127;
        var_guard465_rv = 0.0;

        let assign14620_e20130: f64 = if p.p8 == 7.0 { 1.0 } else { 0.0 };
        var_guard466 = assign14620_e20130;
        var_guard466_rv = 0.0;

        let assign14630_e20133: f64 = if p.p8 == 8.0 { 1.0 } else { 0.0 };
        var_guard467 = assign14630_e20133;
        var_guard467_rv = 0.0;

        let assign14640_e20136: f64 = if p.p8 == 9.0 { 1.0 } else { 0.0 };
        var_guard468 = assign14640_e20136;
        var_guard468_rv = 0.0;

        let assign14650_e20139: f64 = if p.p8 == 10.0 { 1.0 } else { 0.0 };
        var_guard469 = assign14650_e20139;
        var_guard469_rv = 0.0;

        let (assign14660_e20149, assign14660_e20149_d_n3, assign14660_e20149_d_n4, assign14660_e20149_d_n5, assign14660_e20149_d_n6, assign14660_e20149_d_n7, assign14660_e20149_d_n8, assign14660_e20149_d_n9, assign14660_e20149_d_n10, assign14660_e20149_d_n11,) = {
    if (var_guard459 != 0.0) {
        let assign14660_e20143: f64 = (var_nuends * var_psiso);
        let assign14660_e20146: f64 = (var_nuints * var_pssha);
        let assign14660_e20147: f64 = (assign14660_e20143 + assign14660_e20146);
        (assign14660_e20147, ((var_nuends * var_psiso_dn3) + (var_nuints * var_pssha_dn3)), ((var_nuends * var_psiso_dn4) + (var_nuints * var_pssha_dn4)), ((var_nuends * var_psiso_dn5) + (var_nuints * var_pssha_dn5)), ((var_nuends * var_psiso_dn6) + (var_nuints * var_pssha_dn6)), ((var_nuends * var_psiso_dn7) + (var_nuints * var_pssha_dn7)), ((var_nuends * var_psiso_dn8) + (var_nuints * var_pssha_dn8)), ((var_nuends * var_psiso_dn9) + (var_nuints * var_pssha_dn9)), ((var_nuends * var_psiso_dn10) + (var_nuints * var_pssha_dn10)), ((var_nuends * var_psiso_dn11) + (var_nuints * var_pssha_dn11)),)
    } else {
        (var_temp_pseff, var_temp_pseff_dn3, var_temp_pseff_dn4, var_temp_pseff_dn5, var_temp_pseff_dn6, var_temp_pseff_dn7, var_temp_pseff_dn8, var_temp_pseff_dn9, var_temp_pseff_dn10, var_temp_pseff_dn11,)
    }
};
        var_temp_pseff = assign14660_e20149;
        var_temp_pseff_dn3 = assign14660_e20149_d_n3;
        var_temp_pseff_dn4 = assign14660_e20149_d_n4;
        var_temp_pseff_dn5 = assign14660_e20149_d_n5;
        var_temp_pseff_dn6 = assign14660_e20149_d_n6;
        var_temp_pseff_dn7 = assign14660_e20149_d_n7;
        var_temp_pseff_dn8 = assign14660_e20149_d_n8;
        var_temp_pseff_dn9 = assign14660_e20149_d_n9;
        var_temp_pseff_dn10 = assign14660_e20149_d_n10;
        var_temp_pseff_dn11 = assign14660_e20149_d_n11;
        var_temp_pseff_rv = 0.0;

        let (assign14670_e20159, assign14670_e20159_d_n3, assign14670_e20159_d_n4, assign14670_e20159_d_n5, assign14670_e20159_d_n6, assign14670_e20159_d_n7, assign14670_e20159_d_n8, assign14670_e20159_d_n9, assign14670_e20159_d_n10, assign14670_e20159_d_n11,) = {
    if (var_guard459 != 0.0) {
        let assign14670_e20153: f64 = (var_nuendd * var_pdiso);
        let assign14670_e20156: f64 = (var_nuintd * var_pdsha);
        let assign14670_e20157: f64 = (assign14670_e20153 + assign14670_e20156);
        (assign14670_e20157, ((var_nuendd * var_pdiso_dn3) + (var_nuintd * var_pdsha_dn3)), ((var_nuendd * var_pdiso_dn4) + (var_nuintd * var_pdsha_dn4)), ((var_nuendd * var_pdiso_dn5) + (var_nuintd * var_pdsha_dn5)), ((var_nuendd * var_pdiso_dn6) + (var_nuintd * var_pdsha_dn6)), ((var_nuendd * var_pdiso_dn7) + (var_nuintd * var_pdsha_dn7)), ((var_nuendd * var_pdiso_dn8) + (var_nuintd * var_pdsha_dn8)), ((var_nuendd * var_pdiso_dn9) + (var_nuintd * var_pdsha_dn9)), ((var_nuendd * var_pdiso_dn10) + (var_nuintd * var_pdsha_dn10)), ((var_nuendd * var_pdiso_dn11) + (var_nuintd * var_pdsha_dn11)),)
    } else {
        (var_temp_pdeff, var_temp_pdeff_dn3, var_temp_pdeff_dn4, var_temp_pdeff_dn5, var_temp_pdeff_dn6, var_temp_pdeff_dn7, var_temp_pdeff_dn8, var_temp_pdeff_dn9, var_temp_pdeff_dn10, var_temp_pdeff_dn11,)
    }
};
        var_temp_pdeff = assign14670_e20159;
        var_temp_pdeff_dn3 = assign14670_e20159_d_n3;
        var_temp_pdeff_dn4 = assign14670_e20159_d_n4;
        var_temp_pdeff_dn5 = assign14670_e20159_d_n5;
        var_temp_pdeff_dn6 = assign14670_e20159_d_n6;
        var_temp_pdeff_dn7 = assign14670_e20159_d_n7;
        var_temp_pdeff_dn8 = assign14670_e20159_d_n8;
        var_temp_pdeff_dn9 = assign14670_e20159_d_n9;
        var_temp_pdeff_dn10 = assign14670_e20159_d_n10;
        var_temp_pdeff_dn11 = assign14670_e20159_d_n11;
        var_temp_pdeff_rv = 0.0;

        let (assign14680_e20169, assign14680_e20169_d_n3, assign14680_e20169_d_n4, assign14680_e20169_d_n5, assign14680_e20169_d_n6, assign14680_e20169_d_n7, assign14680_e20169_d_n8, assign14680_e20169_d_n9, assign14680_e20169_d_n10, assign14680_e20169_d_n11,) = {
    if (var_guard459 != 0.0) {
        let assign14680_e20163: f64 = (var_nuends * var_asiso);
        let assign14680_e20166: f64 = (var_nuints * var_assha);
        let assign14680_e20167: f64 = (assign14680_e20163 + assign14680_e20166);
        (assign14680_e20167, (var_nuends * var_asiso_dn3), (var_nuends * var_asiso_dn4), (var_nuends * var_asiso_dn5), (var_nuends * var_asiso_dn6), (var_nuends * var_asiso_dn7), (var_nuends * var_asiso_dn8), (var_nuends * var_asiso_dn9), (var_nuends * var_asiso_dn10), (var_nuends * var_asiso_dn11),)
    } else {
        (var_temp_aseff, var_temp_aseff_dn3, var_temp_aseff_dn4, var_temp_aseff_dn5, var_temp_aseff_dn6, var_temp_aseff_dn7, var_temp_aseff_dn8, var_temp_aseff_dn9, var_temp_aseff_dn10, var_temp_aseff_dn11,)
    }
};
        var_temp_aseff = assign14680_e20169;
        var_temp_aseff_dn3 = assign14680_e20169_d_n3;
        var_temp_aseff_dn4 = assign14680_e20169_d_n4;
        var_temp_aseff_dn5 = assign14680_e20169_d_n5;
        var_temp_aseff_dn6 = assign14680_e20169_d_n6;
        var_temp_aseff_dn7 = assign14680_e20169_d_n7;
        var_temp_aseff_dn8 = assign14680_e20169_d_n8;
        var_temp_aseff_dn9 = assign14680_e20169_d_n9;
        var_temp_aseff_dn10 = assign14680_e20169_d_n10;
        var_temp_aseff_dn11 = assign14680_e20169_d_n11;
        var_temp_aseff_rv = 0.0;

        let (assign14690_e20179, assign14690_e20179_d_n3, assign14690_e20179_d_n4, assign14690_e20179_d_n5, assign14690_e20179_d_n6, assign14690_e20179_d_n7, assign14690_e20179_d_n8, assign14690_e20179_d_n9, assign14690_e20179_d_n10, assign14690_e20179_d_n11,) = {
    if (var_guard459 != 0.0) {
        let assign14690_e20173: f64 = (var_nuendd * var_adiso);
        let assign14690_e20176: f64 = (var_nuintd * var_adsha);
        let assign14690_e20177: f64 = (assign14690_e20173 + assign14690_e20176);
        (assign14690_e20177, (var_nuendd * var_adiso_dn3), (var_nuendd * var_adiso_dn4), (var_nuendd * var_adiso_dn5), (var_nuendd * var_adiso_dn6), (var_nuendd * var_adiso_dn7), (var_nuendd * var_adiso_dn8), (var_nuendd * var_adiso_dn9), (var_nuendd * var_adiso_dn10), (var_nuendd * var_adiso_dn11),)
    } else {
        (var_temp_adeff, var_temp_adeff_dn3, var_temp_adeff_dn4, var_temp_adeff_dn5, var_temp_adeff_dn6, var_temp_adeff_dn7, var_temp_adeff_dn8, var_temp_adeff_dn9, var_temp_adeff_dn10, var_temp_adeff_dn11,)
    }
};
        var_temp_adeff = assign14690_e20179;
        var_temp_adeff_dn3 = assign14690_e20179_d_n3;
        var_temp_adeff_dn4 = assign14690_e20179_d_n4;
        var_temp_adeff_dn5 = assign14690_e20179_d_n5;
        var_temp_adeff_dn6 = assign14690_e20179_d_n6;
        var_temp_adeff_dn7 = assign14690_e20179_d_n7;
        var_temp_adeff_dn8 = assign14690_e20179_d_n8;
        var_temp_adeff_dn9 = assign14690_e20179_d_n9;
        var_temp_adeff_dn10 = assign14690_e20179_d_n10;
        var_temp_adeff_dn11 = assign14690_e20179_d_n11;
        var_temp_adeff_rv = 0.0;

        let (assign14700_e20192, assign14700_e20192_d_n3, assign14700_e20192_d_n4, assign14700_e20192_d_n5, assign14700_e20192_d_n6, assign14700_e20192_d_n7, assign14700_e20192_d_n8, assign14700_e20192_d_n9, assign14700_e20192_d_n10, assign14700_e20192_d_n11,) = {
    if ((var_guard460 != 0.0) && (var_guard459 == 0.0)) {
        let assign14700_e20186: f64 = (var_nuends * var_psiso);
        let assign14700_e20189: f64 = (var_nuints * var_pssha);
        let assign14700_e20190: f64 = (assign14700_e20186 + assign14700_e20189);
        (assign14700_e20190, ((var_nuends * var_psiso_dn3) + (var_nuints * var_pssha_dn3)), ((var_nuends * var_psiso_dn4) + (var_nuints * var_pssha_dn4)), ((var_nuends * var_psiso_dn5) + (var_nuints * var_pssha_dn5)), ((var_nuends * var_psiso_dn6) + (var_nuints * var_pssha_dn6)), ((var_nuends * var_psiso_dn7) + (var_nuints * var_pssha_dn7)), ((var_nuends * var_psiso_dn8) + (var_nuints * var_pssha_dn8)), ((var_nuends * var_psiso_dn9) + (var_nuints * var_pssha_dn9)), ((var_nuends * var_psiso_dn10) + (var_nuints * var_pssha_dn10)), ((var_nuends * var_psiso_dn11) + (var_nuints * var_pssha_dn11)),)
    } else {
        (var_temp_pseff, var_temp_pseff_dn3, var_temp_pseff_dn4, var_temp_pseff_dn5, var_temp_pseff_dn6, var_temp_pseff_dn7, var_temp_pseff_dn8, var_temp_pseff_dn9, var_temp_pseff_dn10, var_temp_pseff_dn11,)
    }
};
        var_temp_pseff = assign14700_e20192;
        var_temp_pseff_dn3 = assign14700_e20192_d_n3;
        var_temp_pseff_dn4 = assign14700_e20192_d_n4;
        var_temp_pseff_dn5 = assign14700_e20192_d_n5;
        var_temp_pseff_dn6 = assign14700_e20192_d_n6;
        var_temp_pseff_dn7 = assign14700_e20192_d_n7;
        var_temp_pseff_dn8 = assign14700_e20192_d_n8;
        var_temp_pseff_dn9 = assign14700_e20192_d_n9;
        var_temp_pseff_dn10 = assign14700_e20192_d_n10;
        var_temp_pseff_dn11 = assign14700_e20192_d_n11;
        var_temp_pseff_rv = 0.0;

        let (assign14710_e20203, assign14710_e20203_d_n3, assign14710_e20203_d_n4, assign14710_e20203_d_n5, assign14710_e20203_d_n6, assign14710_e20203_d_n7, assign14710_e20203_d_n8, assign14710_e20203_d_n9, assign14710_e20203_d_n10, assign14710_e20203_d_n11,) = {
    if ((var_guard460 != 0.0) && (var_guard459 == 0.0)) {
        let assign14710_e20199: f64 = (var_nuendd + var_nuintd);
        let assign14710_e20201: f64 = (assign14710_e20199 * var_pdsha);
        (assign14710_e20201, (assign14710_e20199 * var_pdsha_dn3), (assign14710_e20199 * var_pdsha_dn4), (assign14710_e20199 * var_pdsha_dn5), (assign14710_e20199 * var_pdsha_dn6), (assign14710_e20199 * var_pdsha_dn7), (assign14710_e20199 * var_pdsha_dn8), (assign14710_e20199 * var_pdsha_dn9), (assign14710_e20199 * var_pdsha_dn10), (assign14710_e20199 * var_pdsha_dn11),)
    } else {
        (var_temp_pdeff, var_temp_pdeff_dn3, var_temp_pdeff_dn4, var_temp_pdeff_dn5, var_temp_pdeff_dn6, var_temp_pdeff_dn7, var_temp_pdeff_dn8, var_temp_pdeff_dn9, var_temp_pdeff_dn10, var_temp_pdeff_dn11,)
    }
};
        var_temp_pdeff = assign14710_e20203;
        var_temp_pdeff_dn3 = assign14710_e20203_d_n3;
        var_temp_pdeff_dn4 = assign14710_e20203_d_n4;
        var_temp_pdeff_dn5 = assign14710_e20203_d_n5;
        var_temp_pdeff_dn6 = assign14710_e20203_d_n6;
        var_temp_pdeff_dn7 = assign14710_e20203_d_n7;
        var_temp_pdeff_dn8 = assign14710_e20203_d_n8;
        var_temp_pdeff_dn9 = assign14710_e20203_d_n9;
        var_temp_pdeff_dn10 = assign14710_e20203_d_n10;
        var_temp_pdeff_dn11 = assign14710_e20203_d_n11;
        var_temp_pdeff_rv = 0.0;

        let (assign14720_e20216, assign14720_e20216_d_n3, assign14720_e20216_d_n4, assign14720_e20216_d_n5, assign14720_e20216_d_n6, assign14720_e20216_d_n7, assign14720_e20216_d_n8, assign14720_e20216_d_n9, assign14720_e20216_d_n10, assign14720_e20216_d_n11,) = {
    if ((var_guard460 != 0.0) && (var_guard459 == 0.0)) {
        let assign14720_e20210: f64 = (var_nuends * var_asiso);
        let assign14720_e20213: f64 = (var_nuints * var_assha);
        let assign14720_e20214: f64 = (assign14720_e20210 + assign14720_e20213);
        (assign14720_e20214, (var_nuends * var_asiso_dn3), (var_nuends * var_asiso_dn4), (var_nuends * var_asiso_dn5), (var_nuends * var_asiso_dn6), (var_nuends * var_asiso_dn7), (var_nuends * var_asiso_dn8), (var_nuends * var_asiso_dn9), (var_nuends * var_asiso_dn10), (var_nuends * var_asiso_dn11),)
    } else {
        (var_temp_aseff, var_temp_aseff_dn3, var_temp_aseff_dn4, var_temp_aseff_dn5, var_temp_aseff_dn6, var_temp_aseff_dn7, var_temp_aseff_dn8, var_temp_aseff_dn9, var_temp_aseff_dn10, var_temp_aseff_dn11,)
    }
};
        var_temp_aseff = assign14720_e20216;
        var_temp_aseff_dn3 = assign14720_e20216_d_n3;
        var_temp_aseff_dn4 = assign14720_e20216_d_n4;
        var_temp_aseff_dn5 = assign14720_e20216_d_n5;
        var_temp_aseff_dn6 = assign14720_e20216_d_n6;
        var_temp_aseff_dn7 = assign14720_e20216_d_n7;
        var_temp_aseff_dn8 = assign14720_e20216_d_n8;
        var_temp_aseff_dn9 = assign14720_e20216_d_n9;
        var_temp_aseff_dn10 = assign14720_e20216_d_n10;
        var_temp_aseff_dn11 = assign14720_e20216_d_n11;
        var_temp_aseff_rv = 0.0;

        let (assign14730_e20227, assign14730_e20227_d_n3, assign14730_e20227_d_n4, assign14730_e20227_d_n5, assign14730_e20227_d_n6, assign14730_e20227_d_n7, assign14730_e20227_d_n8, assign14730_e20227_d_n9, assign14730_e20227_d_n10, assign14730_e20227_d_n11,) = {
    if ((var_guard460 != 0.0) && (var_guard459 == 0.0)) {
        let assign14730_e20223: f64 = (var_nuendd + var_nuintd);
        let assign14730_e20225: f64 = (assign14730_e20223 * var_adsha);
        (assign14730_e20225, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_temp_adeff, var_temp_adeff_dn3, var_temp_adeff_dn4, var_temp_adeff_dn5, var_temp_adeff_dn6, var_temp_adeff_dn7, var_temp_adeff_dn8, var_temp_adeff_dn9, var_temp_adeff_dn10, var_temp_adeff_dn11,)
    }
};
        var_temp_adeff = assign14730_e20227;
        var_temp_adeff_dn3 = assign14730_e20227_d_n3;
        var_temp_adeff_dn4 = assign14730_e20227_d_n4;
        var_temp_adeff_dn5 = assign14730_e20227_d_n5;
        var_temp_adeff_dn6 = assign14730_e20227_d_n6;
        var_temp_adeff_dn7 = assign14730_e20227_d_n7;
        var_temp_adeff_dn8 = assign14730_e20227_d_n8;
        var_temp_adeff_dn9 = assign14730_e20227_d_n9;
        var_temp_adeff_dn10 = assign14730_e20227_d_n10;
        var_temp_adeff_dn11 = assign14730_e20227_d_n11;
        var_temp_adeff_rv = 0.0;

        let (assign14740_e20240, assign14740_e20240_d_n3, assign14740_e20240_d_n4, assign14740_e20240_d_n5, assign14740_e20240_d_n6, assign14740_e20240_d_n7, assign14740_e20240_d_n8, assign14740_e20240_d_n9, assign14740_e20240_d_n10, assign14740_e20240_d_n11,) = {
    if ((var_guard461 != 0.0) && (!((var_guard459 != 0.0) || (var_guard460 != 0.0)))) {
        let assign14740_e20236: f64 = (var_nuends + var_nuints);
        let assign14740_e20238: f64 = (assign14740_e20236 * var_pssha);
        (assign14740_e20238, (assign14740_e20236 * var_pssha_dn3), (assign14740_e20236 * var_pssha_dn4), (assign14740_e20236 * var_pssha_dn5), (assign14740_e20236 * var_pssha_dn6), (assign14740_e20236 * var_pssha_dn7), (assign14740_e20236 * var_pssha_dn8), (assign14740_e20236 * var_pssha_dn9), (assign14740_e20236 * var_pssha_dn10), (assign14740_e20236 * var_pssha_dn11),)
    } else {
        (var_temp_pseff, var_temp_pseff_dn3, var_temp_pseff_dn4, var_temp_pseff_dn5, var_temp_pseff_dn6, var_temp_pseff_dn7, var_temp_pseff_dn8, var_temp_pseff_dn9, var_temp_pseff_dn10, var_temp_pseff_dn11,)
    }
};
        var_temp_pseff = assign14740_e20240;
        var_temp_pseff_dn3 = assign14740_e20240_d_n3;
        var_temp_pseff_dn4 = assign14740_e20240_d_n4;
        var_temp_pseff_dn5 = assign14740_e20240_d_n5;
        var_temp_pseff_dn6 = assign14740_e20240_d_n6;
        var_temp_pseff_dn7 = assign14740_e20240_d_n7;
        var_temp_pseff_dn8 = assign14740_e20240_d_n8;
        var_temp_pseff_dn9 = assign14740_e20240_d_n9;
        var_temp_pseff_dn10 = assign14740_e20240_d_n10;
        var_temp_pseff_dn11 = assign14740_e20240_d_n11;
        var_temp_pseff_rv = 0.0;

        let (assign14750_e20255, assign14750_e20255_d_n3, assign14750_e20255_d_n4, assign14750_e20255_d_n5, assign14750_e20255_d_n6, assign14750_e20255_d_n7, assign14750_e20255_d_n8, assign14750_e20255_d_n9, assign14750_e20255_d_n10, assign14750_e20255_d_n11,) = {
    if ((var_guard461 != 0.0) && (!((var_guard459 != 0.0) || (var_guard460 != 0.0)))) {
        let assign14750_e20249: f64 = (var_nuendd * var_pdiso);
        let assign14750_e20252: f64 = (var_nuintd * var_pdsha);
        let assign14750_e20253: f64 = (assign14750_e20249 + assign14750_e20252);
        (assign14750_e20253, ((var_nuendd * var_pdiso_dn3) + (var_nuintd * var_pdsha_dn3)), ((var_nuendd * var_pdiso_dn4) + (var_nuintd * var_pdsha_dn4)), ((var_nuendd * var_pdiso_dn5) + (var_nuintd * var_pdsha_dn5)), ((var_nuendd * var_pdiso_dn6) + (var_nuintd * var_pdsha_dn6)), ((var_nuendd * var_pdiso_dn7) + (var_nuintd * var_pdsha_dn7)), ((var_nuendd * var_pdiso_dn8) + (var_nuintd * var_pdsha_dn8)), ((var_nuendd * var_pdiso_dn9) + (var_nuintd * var_pdsha_dn9)), ((var_nuendd * var_pdiso_dn10) + (var_nuintd * var_pdsha_dn10)), ((var_nuendd * var_pdiso_dn11) + (var_nuintd * var_pdsha_dn11)),)
    } else {
        (var_temp_pdeff, var_temp_pdeff_dn3, var_temp_pdeff_dn4, var_temp_pdeff_dn5, var_temp_pdeff_dn6, var_temp_pdeff_dn7, var_temp_pdeff_dn8, var_temp_pdeff_dn9, var_temp_pdeff_dn10, var_temp_pdeff_dn11,)
    }
};
        var_temp_pdeff = assign14750_e20255;
        var_temp_pdeff_dn3 = assign14750_e20255_d_n3;
        var_temp_pdeff_dn4 = assign14750_e20255_d_n4;
        var_temp_pdeff_dn5 = assign14750_e20255_d_n5;
        var_temp_pdeff_dn6 = assign14750_e20255_d_n6;
        var_temp_pdeff_dn7 = assign14750_e20255_d_n7;
        var_temp_pdeff_dn8 = assign14750_e20255_d_n8;
        var_temp_pdeff_dn9 = assign14750_e20255_d_n9;
        var_temp_pdeff_dn10 = assign14750_e20255_d_n10;
        var_temp_pdeff_dn11 = assign14750_e20255_d_n11;
        var_temp_pdeff_rv = 0.0;

        let (assign14760_e20268, assign14760_e20268_d_n3, assign14760_e20268_d_n4, assign14760_e20268_d_n5, assign14760_e20268_d_n6, assign14760_e20268_d_n7, assign14760_e20268_d_n8, assign14760_e20268_d_n9, assign14760_e20268_d_n10, assign14760_e20268_d_n11,) = {
    if ((var_guard461 != 0.0) && (!((var_guard459 != 0.0) || (var_guard460 != 0.0)))) {
        let assign14760_e20264: f64 = (var_nuends + var_nuints);
        let assign14760_e20266: f64 = (assign14760_e20264 * var_assha);
        (assign14760_e20266, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_temp_aseff, var_temp_aseff_dn3, var_temp_aseff_dn4, var_temp_aseff_dn5, var_temp_aseff_dn6, var_temp_aseff_dn7, var_temp_aseff_dn8, var_temp_aseff_dn9, var_temp_aseff_dn10, var_temp_aseff_dn11,)
    }
};
        var_temp_aseff = assign14760_e20268;
        var_temp_aseff_dn3 = assign14760_e20268_d_n3;
        var_temp_aseff_dn4 = assign14760_e20268_d_n4;
        var_temp_aseff_dn5 = assign14760_e20268_d_n5;
        var_temp_aseff_dn6 = assign14760_e20268_d_n6;
        var_temp_aseff_dn7 = assign14760_e20268_d_n7;
        var_temp_aseff_dn8 = assign14760_e20268_d_n8;
        var_temp_aseff_dn9 = assign14760_e20268_d_n9;
        var_temp_aseff_dn10 = assign14760_e20268_d_n10;
        var_temp_aseff_dn11 = assign14760_e20268_d_n11;
        var_temp_aseff_rv = 0.0;

        let (assign14770_e20283, assign14770_e20283_d_n3, assign14770_e20283_d_n4, assign14770_e20283_d_n5, assign14770_e20283_d_n6, assign14770_e20283_d_n7, assign14770_e20283_d_n8, assign14770_e20283_d_n9, assign14770_e20283_d_n10, assign14770_e20283_d_n11,) = {
    if ((var_guard461 != 0.0) && (!((var_guard459 != 0.0) || (var_guard460 != 0.0)))) {
        let assign14770_e20277: f64 = (var_nuendd * var_adiso);
        let assign14770_e20280: f64 = (var_nuintd * var_adsha);
        let assign14770_e20281: f64 = (assign14770_e20277 + assign14770_e20280);
        (assign14770_e20281, (var_nuendd * var_adiso_dn3), (var_nuendd * var_adiso_dn4), (var_nuendd * var_adiso_dn5), (var_nuendd * var_adiso_dn6), (var_nuendd * var_adiso_dn7), (var_nuendd * var_adiso_dn8), (var_nuendd * var_adiso_dn9), (var_nuendd * var_adiso_dn10), (var_nuendd * var_adiso_dn11),)
    } else {
        (var_temp_adeff, var_temp_adeff_dn3, var_temp_adeff_dn4, var_temp_adeff_dn5, var_temp_adeff_dn6, var_temp_adeff_dn7, var_temp_adeff_dn8, var_temp_adeff_dn9, var_temp_adeff_dn10, var_temp_adeff_dn11,)
    }
};
        var_temp_adeff = assign14770_e20283;
        var_temp_adeff_dn3 = assign14770_e20283_d_n3;
        var_temp_adeff_dn4 = assign14770_e20283_d_n4;
        var_temp_adeff_dn5 = assign14770_e20283_d_n5;
        var_temp_adeff_dn6 = assign14770_e20283_d_n6;
        var_temp_adeff_dn7 = assign14770_e20283_d_n7;
        var_temp_adeff_dn8 = assign14770_e20283_d_n8;
        var_temp_adeff_dn9 = assign14770_e20283_d_n9;
        var_temp_adeff_dn10 = assign14770_e20283_d_n10;
        var_temp_adeff_dn11 = assign14770_e20283_d_n11;
        var_temp_adeff_rv = 0.0;

        let (assign14780_e20298, assign14780_e20298_d_n3, assign14780_e20298_d_n4, assign14780_e20298_d_n5, assign14780_e20298_d_n6, assign14780_e20298_d_n7, assign14780_e20298_d_n8, assign14780_e20298_d_n9, assign14780_e20298_d_n10, assign14780_e20298_d_n11,) = {
    if ((var_guard462 != 0.0) && (!(((var_guard459 != 0.0) || (var_guard460 != 0.0)) || (var_guard461 != 0.0)))) {
        let assign14780_e20294: f64 = (var_nuends + var_nuints);
        let assign14780_e20296: f64 = (assign14780_e20294 * var_pssha);
        (assign14780_e20296, (assign14780_e20294 * var_pssha_dn3), (assign14780_e20294 * var_pssha_dn4), (assign14780_e20294 * var_pssha_dn5), (assign14780_e20294 * var_pssha_dn6), (assign14780_e20294 * var_pssha_dn7), (assign14780_e20294 * var_pssha_dn8), (assign14780_e20294 * var_pssha_dn9), (assign14780_e20294 * var_pssha_dn10), (assign14780_e20294 * var_pssha_dn11),)
    } else {
        (var_temp_pseff, var_temp_pseff_dn3, var_temp_pseff_dn4, var_temp_pseff_dn5, var_temp_pseff_dn6, var_temp_pseff_dn7, var_temp_pseff_dn8, var_temp_pseff_dn9, var_temp_pseff_dn10, var_temp_pseff_dn11,)
    }
};
        var_temp_pseff = assign14780_e20298;
        var_temp_pseff_dn3 = assign14780_e20298_d_n3;
        var_temp_pseff_dn4 = assign14780_e20298_d_n4;
        var_temp_pseff_dn5 = assign14780_e20298_d_n5;
        var_temp_pseff_dn6 = assign14780_e20298_d_n6;
        var_temp_pseff_dn7 = assign14780_e20298_d_n7;
        var_temp_pseff_dn8 = assign14780_e20298_d_n8;
        var_temp_pseff_dn9 = assign14780_e20298_d_n9;
        var_temp_pseff_dn10 = assign14780_e20298_d_n10;
        var_temp_pseff_dn11 = assign14780_e20298_d_n11;
        var_temp_pseff_rv = 0.0;

        let (assign14790_e20313, assign14790_e20313_d_n3, assign14790_e20313_d_n4, assign14790_e20313_d_n5, assign14790_e20313_d_n6, assign14790_e20313_d_n7, assign14790_e20313_d_n8, assign14790_e20313_d_n9, assign14790_e20313_d_n10, assign14790_e20313_d_n11,) = {
    if ((var_guard462 != 0.0) && (!(((var_guard459 != 0.0) || (var_guard460 != 0.0)) || (var_guard461 != 0.0)))) {
        let assign14790_e20309: f64 = (var_nuendd + var_nuintd);
        let assign14790_e20311: f64 = (assign14790_e20309 * var_pdsha);
        (assign14790_e20311, (assign14790_e20309 * var_pdsha_dn3), (assign14790_e20309 * var_pdsha_dn4), (assign14790_e20309 * var_pdsha_dn5), (assign14790_e20309 * var_pdsha_dn6), (assign14790_e20309 * var_pdsha_dn7), (assign14790_e20309 * var_pdsha_dn8), (assign14790_e20309 * var_pdsha_dn9), (assign14790_e20309 * var_pdsha_dn10), (assign14790_e20309 * var_pdsha_dn11),)
    } else {
        (var_temp_pdeff, var_temp_pdeff_dn3, var_temp_pdeff_dn4, var_temp_pdeff_dn5, var_temp_pdeff_dn6, var_temp_pdeff_dn7, var_temp_pdeff_dn8, var_temp_pdeff_dn9, var_temp_pdeff_dn10, var_temp_pdeff_dn11,)
    }
};
        var_temp_pdeff = assign14790_e20313;
        var_temp_pdeff_dn3 = assign14790_e20313_d_n3;
        var_temp_pdeff_dn4 = assign14790_e20313_d_n4;
        var_temp_pdeff_dn5 = assign14790_e20313_d_n5;
        var_temp_pdeff_dn6 = assign14790_e20313_d_n6;
        var_temp_pdeff_dn7 = assign14790_e20313_d_n7;
        var_temp_pdeff_dn8 = assign14790_e20313_d_n8;
        var_temp_pdeff_dn9 = assign14790_e20313_d_n9;
        var_temp_pdeff_dn10 = assign14790_e20313_d_n10;
        var_temp_pdeff_dn11 = assign14790_e20313_d_n11;
        var_temp_pdeff_rv = 0.0;

        let (assign14800_e20328, assign14800_e20328_d_n3, assign14800_e20328_d_n4, assign14800_e20328_d_n5, assign14800_e20328_d_n6, assign14800_e20328_d_n7, assign14800_e20328_d_n8, assign14800_e20328_d_n9, assign14800_e20328_d_n10, assign14800_e20328_d_n11,) = {
    if ((var_guard462 != 0.0) && (!(((var_guard459 != 0.0) || (var_guard460 != 0.0)) || (var_guard461 != 0.0)))) {
        let assign14800_e20324: f64 = (var_nuends + var_nuints);
        let assign14800_e20326: f64 = (assign14800_e20324 * var_assha);
        (assign14800_e20326, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_temp_aseff, var_temp_aseff_dn3, var_temp_aseff_dn4, var_temp_aseff_dn5, var_temp_aseff_dn6, var_temp_aseff_dn7, var_temp_aseff_dn8, var_temp_aseff_dn9, var_temp_aseff_dn10, var_temp_aseff_dn11,)
    }
};
        var_temp_aseff = assign14800_e20328;
        var_temp_aseff_dn3 = assign14800_e20328_d_n3;
        var_temp_aseff_dn4 = assign14800_e20328_d_n4;
        var_temp_aseff_dn5 = assign14800_e20328_d_n5;
        var_temp_aseff_dn6 = assign14800_e20328_d_n6;
        var_temp_aseff_dn7 = assign14800_e20328_d_n7;
        var_temp_aseff_dn8 = assign14800_e20328_d_n8;
        var_temp_aseff_dn9 = assign14800_e20328_d_n9;
        var_temp_aseff_dn10 = assign14800_e20328_d_n10;
        var_temp_aseff_dn11 = assign14800_e20328_d_n11;
        var_temp_aseff_rv = 0.0;

        let (assign14810_e20343, assign14810_e20343_d_n3, assign14810_e20343_d_n4, assign14810_e20343_d_n5, assign14810_e20343_d_n6, assign14810_e20343_d_n7, assign14810_e20343_d_n8, assign14810_e20343_d_n9, assign14810_e20343_d_n10, assign14810_e20343_d_n11,) = {
    if ((var_guard462 != 0.0) && (!(((var_guard459 != 0.0) || (var_guard460 != 0.0)) || (var_guard461 != 0.0)))) {
        let assign14810_e20339: f64 = (var_nuendd + var_nuintd);
        let assign14810_e20341: f64 = (assign14810_e20339 * var_adsha);
        (assign14810_e20341, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_temp_adeff, var_temp_adeff_dn3, var_temp_adeff_dn4, var_temp_adeff_dn5, var_temp_adeff_dn6, var_temp_adeff_dn7, var_temp_adeff_dn8, var_temp_adeff_dn9, var_temp_adeff_dn10, var_temp_adeff_dn11,)
    }
};
        var_temp_adeff = assign14810_e20343;
        var_temp_adeff_dn3 = assign14810_e20343_d_n3;
        var_temp_adeff_dn4 = assign14810_e20343_d_n4;
        var_temp_adeff_dn5 = assign14810_e20343_d_n5;
        var_temp_adeff_dn6 = assign14810_e20343_d_n6;
        var_temp_adeff_dn7 = assign14810_e20343_d_n7;
        var_temp_adeff_dn8 = assign14810_e20343_d_n8;
        var_temp_adeff_dn9 = assign14810_e20343_d_n9;
        var_temp_adeff_dn10 = assign14810_e20343_d_n10;
        var_temp_adeff_dn11 = assign14810_e20343_d_n11;
        var_temp_adeff_rv = 0.0;

        let (assign14820_e20362, assign14820_e20362_d_n3, assign14820_e20362_d_n4, assign14820_e20362_d_n5, assign14820_e20362_d_n6, assign14820_e20362_d_n7, assign14820_e20362_d_n8, assign14820_e20362_d_n9, assign14820_e20362_d_n10, assign14820_e20362_d_n11,) = {
    if ((var_guard463 != 0.0) && (!((((var_guard459 != 0.0) || (var_guard460 != 0.0)) || (var_guard461 != 0.0)) || (var_guard462 != 0.0)))) {
        let assign14820_e20356: f64 = (var_nuends * var_psiso);
        let assign14820_e20359: f64 = (var_nuints * var_pssha);
        let assign14820_e20360: f64 = (assign14820_e20356 + assign14820_e20359);
        (assign14820_e20360, ((var_nuends * var_psiso_dn3) + (var_nuints * var_pssha_dn3)), ((var_nuends * var_psiso_dn4) + (var_nuints * var_pssha_dn4)), ((var_nuends * var_psiso_dn5) + (var_nuints * var_pssha_dn5)), ((var_nuends * var_psiso_dn6) + (var_nuints * var_pssha_dn6)), ((var_nuends * var_psiso_dn7) + (var_nuints * var_pssha_dn7)), ((var_nuends * var_psiso_dn8) + (var_nuints * var_pssha_dn8)), ((var_nuends * var_psiso_dn9) + (var_nuints * var_pssha_dn9)), ((var_nuends * var_psiso_dn10) + (var_nuints * var_pssha_dn10)), ((var_nuends * var_psiso_dn11) + (var_nuints * var_pssha_dn11)),)
    } else {
        (var_temp_pseff, var_temp_pseff_dn3, var_temp_pseff_dn4, var_temp_pseff_dn5, var_temp_pseff_dn6, var_temp_pseff_dn7, var_temp_pseff_dn8, var_temp_pseff_dn9, var_temp_pseff_dn10, var_temp_pseff_dn11,)
    }
};
        var_temp_pseff = assign14820_e20362;
        var_temp_pseff_dn3 = assign14820_e20362_d_n3;
        var_temp_pseff_dn4 = assign14820_e20362_d_n4;
        var_temp_pseff_dn5 = assign14820_e20362_d_n5;
        var_temp_pseff_dn6 = assign14820_e20362_d_n6;
        var_temp_pseff_dn7 = assign14820_e20362_d_n7;
        var_temp_pseff_dn8 = assign14820_e20362_d_n8;
        var_temp_pseff_dn9 = assign14820_e20362_d_n9;
        var_temp_pseff_dn10 = assign14820_e20362_d_n10;
        var_temp_pseff_dn11 = assign14820_e20362_d_n11;
        var_temp_pseff_rv = 0.0;

        let (assign14830_e20381, assign14830_e20381_d_n3, assign14830_e20381_d_n4, assign14830_e20381_d_n5, assign14830_e20381_d_n6, assign14830_e20381_d_n7, assign14830_e20381_d_n8, assign14830_e20381_d_n9, assign14830_e20381_d_n10, assign14830_e20381_d_n11,) = {
    if ((var_guard463 != 0.0) && (!((((var_guard459 != 0.0) || (var_guard460 != 0.0)) || (var_guard461 != 0.0)) || (var_guard462 != 0.0)))) {
        let assign14830_e20375: f64 = (var_nuendd * var_pdmer);
        let assign14830_e20378: f64 = (var_nuintd * var_pdsha);
        let assign14830_e20379: f64 = (assign14830_e20375 + assign14830_e20378);
        (assign14830_e20379, ((var_nuendd * var_pdmer_dn3) + (var_nuintd * var_pdsha_dn3)), ((var_nuendd * var_pdmer_dn4) + (var_nuintd * var_pdsha_dn4)), ((var_nuendd * var_pdmer_dn5) + (var_nuintd * var_pdsha_dn5)), ((var_nuendd * var_pdmer_dn6) + (var_nuintd * var_pdsha_dn6)), ((var_nuendd * var_pdmer_dn7) + (var_nuintd * var_pdsha_dn7)), ((var_nuendd * var_pdmer_dn8) + (var_nuintd * var_pdsha_dn8)), ((var_nuendd * var_pdmer_dn9) + (var_nuintd * var_pdsha_dn9)), ((var_nuendd * var_pdmer_dn10) + (var_nuintd * var_pdsha_dn10)), ((var_nuendd * var_pdmer_dn11) + (var_nuintd * var_pdsha_dn11)),)
    } else {
        (var_temp_pdeff, var_temp_pdeff_dn3, var_temp_pdeff_dn4, var_temp_pdeff_dn5, var_temp_pdeff_dn6, var_temp_pdeff_dn7, var_temp_pdeff_dn8, var_temp_pdeff_dn9, var_temp_pdeff_dn10, var_temp_pdeff_dn11,)
    }
};
        var_temp_pdeff = assign14830_e20381;
        var_temp_pdeff_dn3 = assign14830_e20381_d_n3;
        var_temp_pdeff_dn4 = assign14830_e20381_d_n4;
        var_temp_pdeff_dn5 = assign14830_e20381_d_n5;
        var_temp_pdeff_dn6 = assign14830_e20381_d_n6;
        var_temp_pdeff_dn7 = assign14830_e20381_d_n7;
        var_temp_pdeff_dn8 = assign14830_e20381_d_n8;
        var_temp_pdeff_dn9 = assign14830_e20381_d_n9;
        var_temp_pdeff_dn10 = assign14830_e20381_d_n10;
        var_temp_pdeff_dn11 = assign14830_e20381_d_n11;
        var_temp_pdeff_rv = 0.0;

        *var_adiso_slot = var_adiso;
        *var_adiso_dn10_slot = var_adiso_dn10;
        *var_adiso_dn11_slot = var_adiso_dn11;
        *var_adiso_dn3_slot = var_adiso_dn3;
        *var_adiso_dn4_slot = var_adiso_dn4;
        *var_adiso_dn5_slot = var_adiso_dn5;
        *var_adiso_dn6_slot = var_adiso_dn6;
        *var_adiso_dn7_slot = var_adiso_dn7;
        *var_adiso_dn8_slot = var_adiso_dn8;
        *var_adiso_dn9_slot = var_adiso_dn9;
        *var_adiso_rv_slot = var_adiso_rv;
        *var_admer_slot = var_admer;
        *var_admer_rv_slot = var_admer_rv;
        *var_adsha_slot = var_adsha;
        *var_adsha_rv_slot = var_adsha_rv;
        *var_asiso_slot = var_asiso;
        *var_asiso_dn10_slot = var_asiso_dn10;
        *var_asiso_dn11_slot = var_asiso_dn11;
        *var_asiso_dn3_slot = var_asiso_dn3;
        *var_asiso_dn4_slot = var_asiso_dn4;
        *var_asiso_dn5_slot = var_asiso_dn5;
        *var_asiso_dn6_slot = var_asiso_dn6;
        *var_asiso_dn7_slot = var_asiso_dn7;
        *var_asiso_dn8_slot = var_asiso_dn8;
        *var_asiso_dn9_slot = var_asiso_dn9;
        *var_asiso_rv_slot = var_asiso_rv;
        *var_asmer_slot = var_asmer;
        *var_asmer_rv_slot = var_asmer_rv;
        *var_assha_slot = var_assha;
        *var_assha_rv_slot = var_assha_rv;
        *var_guard459_slot = var_guard459;
        *var_guard459_rv_slot = var_guard459_rv;
        *var_guard460_slot = var_guard460;
        *var_guard460_rv_slot = var_guard460_rv;
        *var_guard461_slot = var_guard461;
        *var_guard461_rv_slot = var_guard461_rv;
        *var_guard462_slot = var_guard462;
        *var_guard462_rv_slot = var_guard462_rv;
        *var_guard463_slot = var_guard463;
        *var_guard463_rv_slot = var_guard463_rv;
        *var_guard464_slot = var_guard464;
        *var_guard464_rv_slot = var_guard464_rv;
        *var_guard465_slot = var_guard465;
        *var_guard465_rv_slot = var_guard465_rv;
        *var_guard466_slot = var_guard466;
        *var_guard466_rv_slot = var_guard466_rv;
        *var_guard467_slot = var_guard467;
        *var_guard467_rv_slot = var_guard467_rv;
        *var_guard468_slot = var_guard468;
        *var_guard468_rv_slot = var_guard468_rv;
        *var_guard469_slot = var_guard469;
        *var_guard469_rv_slot = var_guard469_rv;
        *var_pdmer_slot = var_pdmer;
        *var_pdmer_dn10_slot = var_pdmer_dn10;
        *var_pdmer_dn11_slot = var_pdmer_dn11;
        *var_pdmer_dn3_slot = var_pdmer_dn3;
        *var_pdmer_dn4_slot = var_pdmer_dn4;
        *var_pdmer_dn5_slot = var_pdmer_dn5;
        *var_pdmer_dn6_slot = var_pdmer_dn6;
        *var_pdmer_dn7_slot = var_pdmer_dn7;
        *var_pdmer_dn8_slot = var_pdmer_dn8;
        *var_pdmer_dn9_slot = var_pdmer_dn9;
        *var_pdmer_rv_slot = var_pdmer_rv;
        *var_pdsha_slot = var_pdsha;
        *var_pdsha_dn10_slot = var_pdsha_dn10;
        *var_pdsha_dn11_slot = var_pdsha_dn11;
        *var_pdsha_dn3_slot = var_pdsha_dn3;
        *var_pdsha_dn4_slot = var_pdsha_dn4;
        *var_pdsha_dn5_slot = var_pdsha_dn5;
        *var_pdsha_dn6_slot = var_pdsha_dn6;
        *var_pdsha_dn7_slot = var_pdsha_dn7;
        *var_pdsha_dn8_slot = var_pdsha_dn8;
        *var_pdsha_dn9_slot = var_pdsha_dn9;
        *var_pdsha_rv_slot = var_pdsha_rv;
        *var_psmer_slot = var_psmer;
        *var_psmer_dn10_slot = var_psmer_dn10;
        *var_psmer_dn11_slot = var_psmer_dn11;
        *var_psmer_dn3_slot = var_psmer_dn3;
        *var_psmer_dn4_slot = var_psmer_dn4;
        *var_psmer_dn5_slot = var_psmer_dn5;
        *var_psmer_dn6_slot = var_psmer_dn6;
        *var_psmer_dn7_slot = var_psmer_dn7;
        *var_psmer_dn8_slot = var_psmer_dn8;
        *var_psmer_dn9_slot = var_psmer_dn9;
        *var_psmer_rv_slot = var_psmer_rv;
        *var_temp_adeff_slot = var_temp_adeff;
        *var_temp_adeff_dn10_slot = var_temp_adeff_dn10;
        *var_temp_adeff_dn11_slot = var_temp_adeff_dn11;
        *var_temp_adeff_dn3_slot = var_temp_adeff_dn3;
        *var_temp_adeff_dn4_slot = var_temp_adeff_dn4;
        *var_temp_adeff_dn5_slot = var_temp_adeff_dn5;
        *var_temp_adeff_dn6_slot = var_temp_adeff_dn6;
        *var_temp_adeff_dn7_slot = var_temp_adeff_dn7;
        *var_temp_adeff_dn8_slot = var_temp_adeff_dn8;
        *var_temp_adeff_dn9_slot = var_temp_adeff_dn9;
        *var_temp_adeff_rv_slot = var_temp_adeff_rv;
        *var_temp_aseff_slot = var_temp_aseff;
        *var_temp_aseff_dn10_slot = var_temp_aseff_dn10;
        *var_temp_aseff_dn11_slot = var_temp_aseff_dn11;
        *var_temp_aseff_dn3_slot = var_temp_aseff_dn3;
        *var_temp_aseff_dn4_slot = var_temp_aseff_dn4;
        *var_temp_aseff_dn5_slot = var_temp_aseff_dn5;
        *var_temp_aseff_dn6_slot = var_temp_aseff_dn6;
        *var_temp_aseff_dn7_slot = var_temp_aseff_dn7;
        *var_temp_aseff_dn8_slot = var_temp_aseff_dn8;
        *var_temp_aseff_dn9_slot = var_temp_aseff_dn9;
        *var_temp_aseff_rv_slot = var_temp_aseff_rv;
        *var_temp_pdeff_slot = var_temp_pdeff;
        *var_temp_pdeff_dn10_slot = var_temp_pdeff_dn10;
        *var_temp_pdeff_dn11_slot = var_temp_pdeff_dn11;
        *var_temp_pdeff_dn3_slot = var_temp_pdeff_dn3;
        *var_temp_pdeff_dn4_slot = var_temp_pdeff_dn4;
        *var_temp_pdeff_dn5_slot = var_temp_pdeff_dn5;
        *var_temp_pdeff_dn6_slot = var_temp_pdeff_dn6;
        *var_temp_pdeff_dn7_slot = var_temp_pdeff_dn7;
        *var_temp_pdeff_dn8_slot = var_temp_pdeff_dn8;
        *var_temp_pdeff_dn9_slot = var_temp_pdeff_dn9;
        *var_temp_pdeff_rv_slot = var_temp_pdeff_rv;
        *var_temp_pseff_slot = var_temp_pseff;
        *var_temp_pseff_dn10_slot = var_temp_pseff_dn10;
        *var_temp_pseff_dn11_slot = var_temp_pseff_dn11;
        *var_temp_pseff_dn3_slot = var_temp_pseff_dn3;
        *var_temp_pseff_dn4_slot = var_temp_pseff_dn4;
        *var_temp_pseff_dn5_slot = var_temp_pseff_dn5;
        *var_temp_pseff_dn6_slot = var_temp_pseff_dn6;
        *var_temp_pseff_dn7_slot = var_temp_pseff_dn7;
        *var_temp_pseff_dn8_slot = var_temp_pseff_dn8;
        *var_temp_pseff_dn9_slot = var_temp_pseff_dn9;
        *var_temp_pseff_rv_slot = var_temp_pseff_rv;
    }

    pub(super) fn stamp_reactive_block_26(
        p: &Parameters,
        var_adiso: f64,
        var_adiso_dn10: f64,
        var_adiso_dn11: f64,
        var_adiso_dn3: f64,
        var_adiso_dn4: f64,
        var_adiso_dn5: f64,
        var_adiso_dn6: f64,
        var_adiso_dn7: f64,
        var_adiso_dn8: f64,
        var_adiso_dn9: f64,
        var_admer: f64,
        var_adsha: f64,
        var_asiso: f64,
        var_asiso_dn10: f64,
        var_asiso_dn11: f64,
        var_asiso_dn3: f64,
        var_asiso_dn4: f64,
        var_asiso_dn5: f64,
        var_asiso_dn6: f64,
        var_asiso_dn7: f64,
        var_asiso_dn8: f64,
        var_asiso_dn9: f64,
        var_asmer: f64,
        var_assha: f64,
        var_guard459: f64,
        var_guard460: f64,
        var_guard461: f64,
        var_guard462: f64,
        var_guard463: f64,
        var_guard464: f64,
        var_guard465: f64,
        var_guard466: f64,
        var_guard467: f64,
        var_guard468: f64,
        var_guard469: f64,
        var_nuendd: f64,
        var_nuends: f64,
        var_nuintd: f64,
        var_nuints: f64,
        var_pdiso: f64,
        var_pdiso_dn10: f64,
        var_pdiso_dn11: f64,
        var_pdiso_dn3: f64,
        var_pdiso_dn4: f64,
        var_pdiso_dn5: f64,
        var_pdiso_dn6: f64,
        var_pdiso_dn7: f64,
        var_pdiso_dn8: f64,
        var_pdiso_dn9: f64,
        var_pdmer: f64,
        var_pdmer_dn10: f64,
        var_pdmer_dn11: f64,
        var_pdmer_dn3: f64,
        var_pdmer_dn4: f64,
        var_pdmer_dn5: f64,
        var_pdmer_dn6: f64,
        var_pdmer_dn7: f64,
        var_pdmer_dn8: f64,
        var_pdmer_dn9: f64,
        var_pdsha: f64,
        var_pdsha_dn10: f64,
        var_pdsha_dn11: f64,
        var_pdsha_dn3: f64,
        var_pdsha_dn4: f64,
        var_pdsha_dn5: f64,
        var_pdsha_dn6: f64,
        var_pdsha_dn7: f64,
        var_pdsha_dn8: f64,
        var_pdsha_dn9: f64,
        var_psiso: f64,
        var_psiso_dn10: f64,
        var_psiso_dn11: f64,
        var_psiso_dn3: f64,
        var_psiso_dn4: f64,
        var_psiso_dn5: f64,
        var_psiso_dn6: f64,
        var_psiso_dn7: f64,
        var_psiso_dn8: f64,
        var_psiso_dn9: f64,
        var_psmer: f64,
        var_psmer_dn10: f64,
        var_psmer_dn11: f64,
        var_psmer_dn3: f64,
        var_psmer_dn4: f64,
        var_psmer_dn5: f64,
        var_psmer_dn6: f64,
        var_psmer_dn7: f64,
        var_psmer_dn8: f64,
        var_psmer_dn9: f64,
        var_pssha: f64,
        var_pssha_dn10: f64,
        var_pssha_dn11: f64,
        var_pssha_dn3: f64,
        var_pssha_dn4: f64,
        var_pssha_dn5: f64,
        var_pssha_dn6: f64,
        var_pssha_dn7: f64,
        var_pssha_dn8: f64,
        var_pssha_dn9: f64,
        var_temp_adeff_slot: &mut f64,
        var_temp_adeff_dn10_slot: &mut f64,
        var_temp_adeff_dn11_slot: &mut f64,
        var_temp_adeff_dn3_slot: &mut f64,
        var_temp_adeff_dn4_slot: &mut f64,
        var_temp_adeff_dn5_slot: &mut f64,
        var_temp_adeff_dn6_slot: &mut f64,
        var_temp_adeff_dn7_slot: &mut f64,
        var_temp_adeff_dn8_slot: &mut f64,
        var_temp_adeff_dn9_slot: &mut f64,
        var_temp_adeff_rv_slot: &mut f64,
        var_temp_aseff_slot: &mut f64,
        var_temp_aseff_dn10_slot: &mut f64,
        var_temp_aseff_dn11_slot: &mut f64,
        var_temp_aseff_dn3_slot: &mut f64,
        var_temp_aseff_dn4_slot: &mut f64,
        var_temp_aseff_dn5_slot: &mut f64,
        var_temp_aseff_dn6_slot: &mut f64,
        var_temp_aseff_dn7_slot: &mut f64,
        var_temp_aseff_dn8_slot: &mut f64,
        var_temp_aseff_dn9_slot: &mut f64,
        var_temp_aseff_rv_slot: &mut f64,
        var_temp_pdeff_slot: &mut f64,
        var_temp_pdeff_dn10_slot: &mut f64,
        var_temp_pdeff_dn11_slot: &mut f64,
        var_temp_pdeff_dn3_slot: &mut f64,
        var_temp_pdeff_dn4_slot: &mut f64,
        var_temp_pdeff_dn5_slot: &mut f64,
        var_temp_pdeff_dn6_slot: &mut f64,
        var_temp_pdeff_dn7_slot: &mut f64,
        var_temp_pdeff_dn8_slot: &mut f64,
        var_temp_pdeff_dn9_slot: &mut f64,
        var_temp_pdeff_rv_slot: &mut f64,
        var_temp_pseff_slot: &mut f64,
        var_temp_pseff_dn10_slot: &mut f64,
        var_temp_pseff_dn11_slot: &mut f64,
        var_temp_pseff_dn3_slot: &mut f64,
        var_temp_pseff_dn4_slot: &mut f64,
        var_temp_pseff_dn5_slot: &mut f64,
        var_temp_pseff_dn6_slot: &mut f64,
        var_temp_pseff_dn7_slot: &mut f64,
        var_temp_pseff_dn8_slot: &mut f64,
        var_temp_pseff_dn9_slot: &mut f64,
        var_temp_pseff_rv_slot: &mut f64,
    ) {
        let mut var_temp_adeff: f64 = *var_temp_adeff_slot;
        let mut var_temp_adeff_dn10: f64 = *var_temp_adeff_dn10_slot;
        let mut var_temp_adeff_dn11: f64 = *var_temp_adeff_dn11_slot;
        let mut var_temp_adeff_dn3: f64 = *var_temp_adeff_dn3_slot;
        let mut var_temp_adeff_dn4: f64 = *var_temp_adeff_dn4_slot;
        let mut var_temp_adeff_dn5: f64 = *var_temp_adeff_dn5_slot;
        let mut var_temp_adeff_dn6: f64 = *var_temp_adeff_dn6_slot;
        let mut var_temp_adeff_dn7: f64 = *var_temp_adeff_dn7_slot;
        let mut var_temp_adeff_dn8: f64 = *var_temp_adeff_dn8_slot;
        let mut var_temp_adeff_dn9: f64 = *var_temp_adeff_dn9_slot;
        let mut var_temp_adeff_rv: f64 = *var_temp_adeff_rv_slot;
        let mut var_temp_aseff: f64 = *var_temp_aseff_slot;
        let mut var_temp_aseff_dn10: f64 = *var_temp_aseff_dn10_slot;
        let mut var_temp_aseff_dn11: f64 = *var_temp_aseff_dn11_slot;
        let mut var_temp_aseff_dn3: f64 = *var_temp_aseff_dn3_slot;
        let mut var_temp_aseff_dn4: f64 = *var_temp_aseff_dn4_slot;
        let mut var_temp_aseff_dn5: f64 = *var_temp_aseff_dn5_slot;
        let mut var_temp_aseff_dn6: f64 = *var_temp_aseff_dn6_slot;
        let mut var_temp_aseff_dn7: f64 = *var_temp_aseff_dn7_slot;
        let mut var_temp_aseff_dn8: f64 = *var_temp_aseff_dn8_slot;
        let mut var_temp_aseff_dn9: f64 = *var_temp_aseff_dn9_slot;
        let mut var_temp_aseff_rv: f64 = *var_temp_aseff_rv_slot;
        let mut var_temp_pdeff: f64 = *var_temp_pdeff_slot;
        let mut var_temp_pdeff_dn10: f64 = *var_temp_pdeff_dn10_slot;
        let mut var_temp_pdeff_dn11: f64 = *var_temp_pdeff_dn11_slot;
        let mut var_temp_pdeff_dn3: f64 = *var_temp_pdeff_dn3_slot;
        let mut var_temp_pdeff_dn4: f64 = *var_temp_pdeff_dn4_slot;
        let mut var_temp_pdeff_dn5: f64 = *var_temp_pdeff_dn5_slot;
        let mut var_temp_pdeff_dn6: f64 = *var_temp_pdeff_dn6_slot;
        let mut var_temp_pdeff_dn7: f64 = *var_temp_pdeff_dn7_slot;
        let mut var_temp_pdeff_dn8: f64 = *var_temp_pdeff_dn8_slot;
        let mut var_temp_pdeff_dn9: f64 = *var_temp_pdeff_dn9_slot;
        let mut var_temp_pdeff_rv: f64 = *var_temp_pdeff_rv_slot;
        let mut var_temp_pseff: f64 = *var_temp_pseff_slot;
        let mut var_temp_pseff_dn10: f64 = *var_temp_pseff_dn10_slot;
        let mut var_temp_pseff_dn11: f64 = *var_temp_pseff_dn11_slot;
        let mut var_temp_pseff_dn3: f64 = *var_temp_pseff_dn3_slot;
        let mut var_temp_pseff_dn4: f64 = *var_temp_pseff_dn4_slot;
        let mut var_temp_pseff_dn5: f64 = *var_temp_pseff_dn5_slot;
        let mut var_temp_pseff_dn6: f64 = *var_temp_pseff_dn6_slot;
        let mut var_temp_pseff_dn7: f64 = *var_temp_pseff_dn7_slot;
        let mut var_temp_pseff_dn8: f64 = *var_temp_pseff_dn8_slot;
        let mut var_temp_pseff_dn9: f64 = *var_temp_pseff_dn9_slot;
        let mut var_temp_pseff_rv: f64 = *var_temp_pseff_rv_slot;

        let (assign14840_e20400, assign14840_e20400_d_n3, assign14840_e20400_d_n4, assign14840_e20400_d_n5, assign14840_e20400_d_n6, assign14840_e20400_d_n7, assign14840_e20400_d_n8, assign14840_e20400_d_n9, assign14840_e20400_d_n10, assign14840_e20400_d_n11,) = {
    if ((var_guard463 != 0.0) && (!((((var_guard459 != 0.0) || (var_guard460 != 0.0)) || (var_guard461 != 0.0)) || (var_guard462 != 0.0)))) {
        let assign14840_e20394: f64 = (var_nuends * var_asiso);
        let assign14840_e20397: f64 = (var_nuints * var_assha);
        let assign14840_e20398: f64 = (assign14840_e20394 + assign14840_e20397);
        (assign14840_e20398, (var_nuends * var_asiso_dn3), (var_nuends * var_asiso_dn4), (var_nuends * var_asiso_dn5), (var_nuends * var_asiso_dn6), (var_nuends * var_asiso_dn7), (var_nuends * var_asiso_dn8), (var_nuends * var_asiso_dn9), (var_nuends * var_asiso_dn10), (var_nuends * var_asiso_dn11),)
    } else {
        (var_temp_aseff, var_temp_aseff_dn3, var_temp_aseff_dn4, var_temp_aseff_dn5, var_temp_aseff_dn6, var_temp_aseff_dn7, var_temp_aseff_dn8, var_temp_aseff_dn9, var_temp_aseff_dn10, var_temp_aseff_dn11,)
    }
};
        var_temp_aseff = assign14840_e20400;
        var_temp_aseff_dn3 = assign14840_e20400_d_n3;
        var_temp_aseff_dn4 = assign14840_e20400_d_n4;
        var_temp_aseff_dn5 = assign14840_e20400_d_n5;
        var_temp_aseff_dn6 = assign14840_e20400_d_n6;
        var_temp_aseff_dn7 = assign14840_e20400_d_n7;
        var_temp_aseff_dn8 = assign14840_e20400_d_n8;
        var_temp_aseff_dn9 = assign14840_e20400_d_n9;
        var_temp_aseff_dn10 = assign14840_e20400_d_n10;
        var_temp_aseff_dn11 = assign14840_e20400_d_n11;
        var_temp_aseff_rv = 0.0;

        let (assign14850_e20419, assign14850_e20419_d_n3, assign14850_e20419_d_n4, assign14850_e20419_d_n5, assign14850_e20419_d_n6, assign14850_e20419_d_n7, assign14850_e20419_d_n8, assign14850_e20419_d_n9, assign14850_e20419_d_n10, assign14850_e20419_d_n11,) = {
    if ((var_guard463 != 0.0) && (!((((var_guard459 != 0.0) || (var_guard460 != 0.0)) || (var_guard461 != 0.0)) || (var_guard462 != 0.0)))) {
        let assign14850_e20413: f64 = (var_nuendd * var_admer);
        let assign14850_e20416: f64 = (var_nuintd * var_adsha);
        let assign14850_e20417: f64 = (assign14850_e20413 + assign14850_e20416);
        (assign14850_e20417, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_temp_adeff, var_temp_adeff_dn3, var_temp_adeff_dn4, var_temp_adeff_dn5, var_temp_adeff_dn6, var_temp_adeff_dn7, var_temp_adeff_dn8, var_temp_adeff_dn9, var_temp_adeff_dn10, var_temp_adeff_dn11,)
    }
};
        var_temp_adeff = assign14850_e20419;
        var_temp_adeff_dn3 = assign14850_e20419_d_n3;
        var_temp_adeff_dn4 = assign14850_e20419_d_n4;
        var_temp_adeff_dn5 = assign14850_e20419_d_n5;
        var_temp_adeff_dn6 = assign14850_e20419_d_n6;
        var_temp_adeff_dn7 = assign14850_e20419_d_n7;
        var_temp_adeff_dn8 = assign14850_e20419_d_n8;
        var_temp_adeff_dn9 = assign14850_e20419_d_n9;
        var_temp_adeff_dn10 = assign14850_e20419_d_n10;
        var_temp_adeff_dn11 = assign14850_e20419_d_n11;
        var_temp_adeff_rv = 0.0;

        let (assign14860_e20438, assign14860_e20438_d_n3, assign14860_e20438_d_n4, assign14860_e20438_d_n5, assign14860_e20438_d_n6, assign14860_e20438_d_n7, assign14860_e20438_d_n8, assign14860_e20438_d_n9, assign14860_e20438_d_n10, assign14860_e20438_d_n11,) = {
    if ((var_guard464 != 0.0) && (!(((((var_guard459 != 0.0) || (var_guard460 != 0.0)) || (var_guard461 != 0.0)) || (var_guard462 != 0.0)) || (var_guard463 != 0.0)))) {
        let assign14860_e20434: f64 = (var_nuends + var_nuints);
        let assign14860_e20436: f64 = (assign14860_e20434 * var_pssha);
        (assign14860_e20436, (assign14860_e20434 * var_pssha_dn3), (assign14860_e20434 * var_pssha_dn4), (assign14860_e20434 * var_pssha_dn5), (assign14860_e20434 * var_pssha_dn6), (assign14860_e20434 * var_pssha_dn7), (assign14860_e20434 * var_pssha_dn8), (assign14860_e20434 * var_pssha_dn9), (assign14860_e20434 * var_pssha_dn10), (assign14860_e20434 * var_pssha_dn11),)
    } else {
        (var_temp_pseff, var_temp_pseff_dn3, var_temp_pseff_dn4, var_temp_pseff_dn5, var_temp_pseff_dn6, var_temp_pseff_dn7, var_temp_pseff_dn8, var_temp_pseff_dn9, var_temp_pseff_dn10, var_temp_pseff_dn11,)
    }
};
        var_temp_pseff = assign14860_e20438;
        var_temp_pseff_dn3 = assign14860_e20438_d_n3;
        var_temp_pseff_dn4 = assign14860_e20438_d_n4;
        var_temp_pseff_dn5 = assign14860_e20438_d_n5;
        var_temp_pseff_dn6 = assign14860_e20438_d_n6;
        var_temp_pseff_dn7 = assign14860_e20438_d_n7;
        var_temp_pseff_dn8 = assign14860_e20438_d_n8;
        var_temp_pseff_dn9 = assign14860_e20438_d_n9;
        var_temp_pseff_dn10 = assign14860_e20438_d_n10;
        var_temp_pseff_dn11 = assign14860_e20438_d_n11;
        var_temp_pseff_rv = 0.0;

        let (assign14870_e20459, assign14870_e20459_d_n3, assign14870_e20459_d_n4, assign14870_e20459_d_n5, assign14870_e20459_d_n6, assign14870_e20459_d_n7, assign14870_e20459_d_n8, assign14870_e20459_d_n9, assign14870_e20459_d_n10, assign14870_e20459_d_n11,) = {
    if ((var_guard464 != 0.0) && (!(((((var_guard459 != 0.0) || (var_guard460 != 0.0)) || (var_guard461 != 0.0)) || (var_guard462 != 0.0)) || (var_guard463 != 0.0)))) {
        let assign14870_e20453: f64 = (var_nuendd * var_pdmer);
        let assign14870_e20456: f64 = (var_nuintd * var_pdsha);
        let assign14870_e20457: f64 = (assign14870_e20453 + assign14870_e20456);
        (assign14870_e20457, ((var_nuendd * var_pdmer_dn3) + (var_nuintd * var_pdsha_dn3)), ((var_nuendd * var_pdmer_dn4) + (var_nuintd * var_pdsha_dn4)), ((var_nuendd * var_pdmer_dn5) + (var_nuintd * var_pdsha_dn5)), ((var_nuendd * var_pdmer_dn6) + (var_nuintd * var_pdsha_dn6)), ((var_nuendd * var_pdmer_dn7) + (var_nuintd * var_pdsha_dn7)), ((var_nuendd * var_pdmer_dn8) + (var_nuintd * var_pdsha_dn8)), ((var_nuendd * var_pdmer_dn9) + (var_nuintd * var_pdsha_dn9)), ((var_nuendd * var_pdmer_dn10) + (var_nuintd * var_pdsha_dn10)), ((var_nuendd * var_pdmer_dn11) + (var_nuintd * var_pdsha_dn11)),)
    } else {
        (var_temp_pdeff, var_temp_pdeff_dn3, var_temp_pdeff_dn4, var_temp_pdeff_dn5, var_temp_pdeff_dn6, var_temp_pdeff_dn7, var_temp_pdeff_dn8, var_temp_pdeff_dn9, var_temp_pdeff_dn10, var_temp_pdeff_dn11,)
    }
};
        var_temp_pdeff = assign14870_e20459;
        var_temp_pdeff_dn3 = assign14870_e20459_d_n3;
        var_temp_pdeff_dn4 = assign14870_e20459_d_n4;
        var_temp_pdeff_dn5 = assign14870_e20459_d_n5;
        var_temp_pdeff_dn6 = assign14870_e20459_d_n6;
        var_temp_pdeff_dn7 = assign14870_e20459_d_n7;
        var_temp_pdeff_dn8 = assign14870_e20459_d_n8;
        var_temp_pdeff_dn9 = assign14870_e20459_d_n9;
        var_temp_pdeff_dn10 = assign14870_e20459_d_n10;
        var_temp_pdeff_dn11 = assign14870_e20459_d_n11;
        var_temp_pdeff_rv = 0.0;

        let (assign14880_e20478, assign14880_e20478_d_n3, assign14880_e20478_d_n4, assign14880_e20478_d_n5, assign14880_e20478_d_n6, assign14880_e20478_d_n7, assign14880_e20478_d_n8, assign14880_e20478_d_n9, assign14880_e20478_d_n10, assign14880_e20478_d_n11,) = {
    if ((var_guard464 != 0.0) && (!(((((var_guard459 != 0.0) || (var_guard460 != 0.0)) || (var_guard461 != 0.0)) || (var_guard462 != 0.0)) || (var_guard463 != 0.0)))) {
        let assign14880_e20474: f64 = (var_nuends + var_nuints);
        let assign14880_e20476: f64 = (assign14880_e20474 * var_assha);
        (assign14880_e20476, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_temp_aseff, var_temp_aseff_dn3, var_temp_aseff_dn4, var_temp_aseff_dn5, var_temp_aseff_dn6, var_temp_aseff_dn7, var_temp_aseff_dn8, var_temp_aseff_dn9, var_temp_aseff_dn10, var_temp_aseff_dn11,)
    }
};
        var_temp_aseff = assign14880_e20478;
        var_temp_aseff_dn3 = assign14880_e20478_d_n3;
        var_temp_aseff_dn4 = assign14880_e20478_d_n4;
        var_temp_aseff_dn5 = assign14880_e20478_d_n5;
        var_temp_aseff_dn6 = assign14880_e20478_d_n6;
        var_temp_aseff_dn7 = assign14880_e20478_d_n7;
        var_temp_aseff_dn8 = assign14880_e20478_d_n8;
        var_temp_aseff_dn9 = assign14880_e20478_d_n9;
        var_temp_aseff_dn10 = assign14880_e20478_d_n10;
        var_temp_aseff_dn11 = assign14880_e20478_d_n11;
        var_temp_aseff_rv = 0.0;

        let (assign14890_e20499, assign14890_e20499_d_n3, assign14890_e20499_d_n4, assign14890_e20499_d_n5, assign14890_e20499_d_n6, assign14890_e20499_d_n7, assign14890_e20499_d_n8, assign14890_e20499_d_n9, assign14890_e20499_d_n10, assign14890_e20499_d_n11,) = {
    if ((var_guard464 != 0.0) && (!(((((var_guard459 != 0.0) || (var_guard460 != 0.0)) || (var_guard461 != 0.0)) || (var_guard462 != 0.0)) || (var_guard463 != 0.0)))) {
        let assign14890_e20493: f64 = (var_nuendd * var_admer);
        let assign14890_e20496: f64 = (var_nuintd * var_adsha);
        let assign14890_e20497: f64 = (assign14890_e20493 + assign14890_e20496);
        (assign14890_e20497, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_temp_adeff, var_temp_adeff_dn3, var_temp_adeff_dn4, var_temp_adeff_dn5, var_temp_adeff_dn6, var_temp_adeff_dn7, var_temp_adeff_dn8, var_temp_adeff_dn9, var_temp_adeff_dn10, var_temp_adeff_dn11,)
    }
};
        var_temp_adeff = assign14890_e20499;
        var_temp_adeff_dn3 = assign14890_e20499_d_n3;
        var_temp_adeff_dn4 = assign14890_e20499_d_n4;
        var_temp_adeff_dn5 = assign14890_e20499_d_n5;
        var_temp_adeff_dn6 = assign14890_e20499_d_n6;
        var_temp_adeff_dn7 = assign14890_e20499_d_n7;
        var_temp_adeff_dn8 = assign14890_e20499_d_n8;
        var_temp_adeff_dn9 = assign14890_e20499_d_n9;
        var_temp_adeff_dn10 = assign14890_e20499_d_n10;
        var_temp_adeff_dn11 = assign14890_e20499_d_n11;
        var_temp_adeff_rv = 0.0;

        let (assign14900_e20522, assign14900_e20522_d_n3, assign14900_e20522_d_n4, assign14900_e20522_d_n5, assign14900_e20522_d_n6, assign14900_e20522_d_n7, assign14900_e20522_d_n8, assign14900_e20522_d_n9, assign14900_e20522_d_n10, assign14900_e20522_d_n11,) = {
    if ((var_guard465 != 0.0) && (!((((((var_guard459 != 0.0) || (var_guard460 != 0.0)) || (var_guard461 != 0.0)) || (var_guard462 != 0.0)) || (var_guard463 != 0.0)) || (var_guard464 != 0.0)))) {
        let assign14900_e20516: f64 = (var_nuends * var_psmer);
        let assign14900_e20519: f64 = (var_nuints * var_pssha);
        let assign14900_e20520: f64 = (assign14900_e20516 + assign14900_e20519);
        (assign14900_e20520, ((var_nuends * var_psmer_dn3) + (var_nuints * var_pssha_dn3)), ((var_nuends * var_psmer_dn4) + (var_nuints * var_pssha_dn4)), ((var_nuends * var_psmer_dn5) + (var_nuints * var_pssha_dn5)), ((var_nuends * var_psmer_dn6) + (var_nuints * var_pssha_dn6)), ((var_nuends * var_psmer_dn7) + (var_nuints * var_pssha_dn7)), ((var_nuends * var_psmer_dn8) + (var_nuints * var_pssha_dn8)), ((var_nuends * var_psmer_dn9) + (var_nuints * var_pssha_dn9)), ((var_nuends * var_psmer_dn10) + (var_nuints * var_pssha_dn10)), ((var_nuends * var_psmer_dn11) + (var_nuints * var_pssha_dn11)),)
    } else {
        (var_temp_pseff, var_temp_pseff_dn3, var_temp_pseff_dn4, var_temp_pseff_dn5, var_temp_pseff_dn6, var_temp_pseff_dn7, var_temp_pseff_dn8, var_temp_pseff_dn9, var_temp_pseff_dn10, var_temp_pseff_dn11,)
    }
};
        var_temp_pseff = assign14900_e20522;
        var_temp_pseff_dn3 = assign14900_e20522_d_n3;
        var_temp_pseff_dn4 = assign14900_e20522_d_n4;
        var_temp_pseff_dn5 = assign14900_e20522_d_n5;
        var_temp_pseff_dn6 = assign14900_e20522_d_n6;
        var_temp_pseff_dn7 = assign14900_e20522_d_n7;
        var_temp_pseff_dn8 = assign14900_e20522_d_n8;
        var_temp_pseff_dn9 = assign14900_e20522_d_n9;
        var_temp_pseff_dn10 = assign14900_e20522_d_n10;
        var_temp_pseff_dn11 = assign14900_e20522_d_n11;
        var_temp_pseff_rv = 0.0;

        let (assign14910_e20545, assign14910_e20545_d_n3, assign14910_e20545_d_n4, assign14910_e20545_d_n5, assign14910_e20545_d_n6, assign14910_e20545_d_n7, assign14910_e20545_d_n8, assign14910_e20545_d_n9, assign14910_e20545_d_n10, assign14910_e20545_d_n11,) = {
    if ((var_guard465 != 0.0) && (!((((((var_guard459 != 0.0) || (var_guard460 != 0.0)) || (var_guard461 != 0.0)) || (var_guard462 != 0.0)) || (var_guard463 != 0.0)) || (var_guard464 != 0.0)))) {
        let assign14910_e20539: f64 = (var_nuendd * var_pdiso);
        let assign14910_e20542: f64 = (var_nuintd * var_pdsha);
        let assign14910_e20543: f64 = (assign14910_e20539 + assign14910_e20542);
        (assign14910_e20543, ((var_nuendd * var_pdiso_dn3) + (var_nuintd * var_pdsha_dn3)), ((var_nuendd * var_pdiso_dn4) + (var_nuintd * var_pdsha_dn4)), ((var_nuendd * var_pdiso_dn5) + (var_nuintd * var_pdsha_dn5)), ((var_nuendd * var_pdiso_dn6) + (var_nuintd * var_pdsha_dn6)), ((var_nuendd * var_pdiso_dn7) + (var_nuintd * var_pdsha_dn7)), ((var_nuendd * var_pdiso_dn8) + (var_nuintd * var_pdsha_dn8)), ((var_nuendd * var_pdiso_dn9) + (var_nuintd * var_pdsha_dn9)), ((var_nuendd * var_pdiso_dn10) + (var_nuintd * var_pdsha_dn10)), ((var_nuendd * var_pdiso_dn11) + (var_nuintd * var_pdsha_dn11)),)
    } else {
        (var_temp_pdeff, var_temp_pdeff_dn3, var_temp_pdeff_dn4, var_temp_pdeff_dn5, var_temp_pdeff_dn6, var_temp_pdeff_dn7, var_temp_pdeff_dn8, var_temp_pdeff_dn9, var_temp_pdeff_dn10, var_temp_pdeff_dn11,)
    }
};
        var_temp_pdeff = assign14910_e20545;
        var_temp_pdeff_dn3 = assign14910_e20545_d_n3;
        var_temp_pdeff_dn4 = assign14910_e20545_d_n4;
        var_temp_pdeff_dn5 = assign14910_e20545_d_n5;
        var_temp_pdeff_dn6 = assign14910_e20545_d_n6;
        var_temp_pdeff_dn7 = assign14910_e20545_d_n7;
        var_temp_pdeff_dn8 = assign14910_e20545_d_n8;
        var_temp_pdeff_dn9 = assign14910_e20545_d_n9;
        var_temp_pdeff_dn10 = assign14910_e20545_d_n10;
        var_temp_pdeff_dn11 = assign14910_e20545_d_n11;
        var_temp_pdeff_rv = 0.0;

        let (assign14920_e20568, assign14920_e20568_d_n3, assign14920_e20568_d_n4, assign14920_e20568_d_n5, assign14920_e20568_d_n6, assign14920_e20568_d_n7, assign14920_e20568_d_n8, assign14920_e20568_d_n9, assign14920_e20568_d_n10, assign14920_e20568_d_n11,) = {
    if ((var_guard465 != 0.0) && (!((((((var_guard459 != 0.0) || (var_guard460 != 0.0)) || (var_guard461 != 0.0)) || (var_guard462 != 0.0)) || (var_guard463 != 0.0)) || (var_guard464 != 0.0)))) {
        let assign14920_e20562: f64 = (var_nuends * var_asmer);
        let assign14920_e20565: f64 = (var_nuints * var_assha);
        let assign14920_e20566: f64 = (assign14920_e20562 + assign14920_e20565);
        (assign14920_e20566, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_temp_aseff, var_temp_aseff_dn3, var_temp_aseff_dn4, var_temp_aseff_dn5, var_temp_aseff_dn6, var_temp_aseff_dn7, var_temp_aseff_dn8, var_temp_aseff_dn9, var_temp_aseff_dn10, var_temp_aseff_dn11,)
    }
};
        var_temp_aseff = assign14920_e20568;
        var_temp_aseff_dn3 = assign14920_e20568_d_n3;
        var_temp_aseff_dn4 = assign14920_e20568_d_n4;
        var_temp_aseff_dn5 = assign14920_e20568_d_n5;
        var_temp_aseff_dn6 = assign14920_e20568_d_n6;
        var_temp_aseff_dn7 = assign14920_e20568_d_n7;
        var_temp_aseff_dn8 = assign14920_e20568_d_n8;
        var_temp_aseff_dn9 = assign14920_e20568_d_n9;
        var_temp_aseff_dn10 = assign14920_e20568_d_n10;
        var_temp_aseff_dn11 = assign14920_e20568_d_n11;
        var_temp_aseff_rv = 0.0;

        let (assign14930_e20591, assign14930_e20591_d_n3, assign14930_e20591_d_n4, assign14930_e20591_d_n5, assign14930_e20591_d_n6, assign14930_e20591_d_n7, assign14930_e20591_d_n8, assign14930_e20591_d_n9, assign14930_e20591_d_n10, assign14930_e20591_d_n11,) = {
    if ((var_guard465 != 0.0) && (!((((((var_guard459 != 0.0) || (var_guard460 != 0.0)) || (var_guard461 != 0.0)) || (var_guard462 != 0.0)) || (var_guard463 != 0.0)) || (var_guard464 != 0.0)))) {
        let assign14930_e20585: f64 = (var_nuendd * var_adiso);
        let assign14930_e20588: f64 = (var_nuintd * var_adsha);
        let assign14930_e20589: f64 = (assign14930_e20585 + assign14930_e20588);
        (assign14930_e20589, (var_nuendd * var_adiso_dn3), (var_nuendd * var_adiso_dn4), (var_nuendd * var_adiso_dn5), (var_nuendd * var_adiso_dn6), (var_nuendd * var_adiso_dn7), (var_nuendd * var_adiso_dn8), (var_nuendd * var_adiso_dn9), (var_nuendd * var_adiso_dn10), (var_nuendd * var_adiso_dn11),)
    } else {
        (var_temp_adeff, var_temp_adeff_dn3, var_temp_adeff_dn4, var_temp_adeff_dn5, var_temp_adeff_dn6, var_temp_adeff_dn7, var_temp_adeff_dn8, var_temp_adeff_dn9, var_temp_adeff_dn10, var_temp_adeff_dn11,)
    }
};
        var_temp_adeff = assign14930_e20591;
        var_temp_adeff_dn3 = assign14930_e20591_d_n3;
        var_temp_adeff_dn4 = assign14930_e20591_d_n4;
        var_temp_adeff_dn5 = assign14930_e20591_d_n5;
        var_temp_adeff_dn6 = assign14930_e20591_d_n6;
        var_temp_adeff_dn7 = assign14930_e20591_d_n7;
        var_temp_adeff_dn8 = assign14930_e20591_d_n8;
        var_temp_adeff_dn9 = assign14930_e20591_d_n9;
        var_temp_adeff_dn10 = assign14930_e20591_d_n10;
        var_temp_adeff_dn11 = assign14930_e20591_d_n11;
        var_temp_adeff_rv = 0.0;

        let (assign14940_e20616, assign14940_e20616_d_n3, assign14940_e20616_d_n4, assign14940_e20616_d_n5, assign14940_e20616_d_n6, assign14940_e20616_d_n7, assign14940_e20616_d_n8, assign14940_e20616_d_n9, assign14940_e20616_d_n10, assign14940_e20616_d_n11,) = {
    if ((var_guard466 != 0.0) && (!(((((((var_guard459 != 0.0) || (var_guard460 != 0.0)) || (var_guard461 != 0.0)) || (var_guard462 != 0.0)) || (var_guard463 != 0.0)) || (var_guard464 != 0.0)) || (var_guard465 != 0.0)))) {
        let assign14940_e20610: f64 = (var_nuends * var_psmer);
        let assign14940_e20613: f64 = (var_nuints * var_pssha);
        let assign14940_e20614: f64 = (assign14940_e20610 + assign14940_e20613);
        (assign14940_e20614, ((var_nuends * var_psmer_dn3) + (var_nuints * var_pssha_dn3)), ((var_nuends * var_psmer_dn4) + (var_nuints * var_pssha_dn4)), ((var_nuends * var_psmer_dn5) + (var_nuints * var_pssha_dn5)), ((var_nuends * var_psmer_dn6) + (var_nuints * var_pssha_dn6)), ((var_nuends * var_psmer_dn7) + (var_nuints * var_pssha_dn7)), ((var_nuends * var_psmer_dn8) + (var_nuints * var_pssha_dn8)), ((var_nuends * var_psmer_dn9) + (var_nuints * var_pssha_dn9)), ((var_nuends * var_psmer_dn10) + (var_nuints * var_pssha_dn10)), ((var_nuends * var_psmer_dn11) + (var_nuints * var_pssha_dn11)),)
    } else {
        (var_temp_pseff, var_temp_pseff_dn3, var_temp_pseff_dn4, var_temp_pseff_dn5, var_temp_pseff_dn6, var_temp_pseff_dn7, var_temp_pseff_dn8, var_temp_pseff_dn9, var_temp_pseff_dn10, var_temp_pseff_dn11,)
    }
};
        var_temp_pseff = assign14940_e20616;
        var_temp_pseff_dn3 = assign14940_e20616_d_n3;
        var_temp_pseff_dn4 = assign14940_e20616_d_n4;
        var_temp_pseff_dn5 = assign14940_e20616_d_n5;
        var_temp_pseff_dn6 = assign14940_e20616_d_n6;
        var_temp_pseff_dn7 = assign14940_e20616_d_n7;
        var_temp_pseff_dn8 = assign14940_e20616_d_n8;
        var_temp_pseff_dn9 = assign14940_e20616_d_n9;
        var_temp_pseff_dn10 = assign14940_e20616_d_n10;
        var_temp_pseff_dn11 = assign14940_e20616_d_n11;
        var_temp_pseff_rv = 0.0;

        let (assign14950_e20639, assign14950_e20639_d_n3, assign14950_e20639_d_n4, assign14950_e20639_d_n5, assign14950_e20639_d_n6, assign14950_e20639_d_n7, assign14950_e20639_d_n8, assign14950_e20639_d_n9, assign14950_e20639_d_n10, assign14950_e20639_d_n11,) = {
    if ((var_guard466 != 0.0) && (!(((((((var_guard459 != 0.0) || (var_guard460 != 0.0)) || (var_guard461 != 0.0)) || (var_guard462 != 0.0)) || (var_guard463 != 0.0)) || (var_guard464 != 0.0)) || (var_guard465 != 0.0)))) {
        let assign14950_e20635: f64 = (var_nuendd + var_nuintd);
        let assign14950_e20637: f64 = (assign14950_e20635 * var_pdsha);
        (assign14950_e20637, (assign14950_e20635 * var_pdsha_dn3), (assign14950_e20635 * var_pdsha_dn4), (assign14950_e20635 * var_pdsha_dn5), (assign14950_e20635 * var_pdsha_dn6), (assign14950_e20635 * var_pdsha_dn7), (assign14950_e20635 * var_pdsha_dn8), (assign14950_e20635 * var_pdsha_dn9), (assign14950_e20635 * var_pdsha_dn10), (assign14950_e20635 * var_pdsha_dn11),)
    } else {
        (var_temp_pdeff, var_temp_pdeff_dn3, var_temp_pdeff_dn4, var_temp_pdeff_dn5, var_temp_pdeff_dn6, var_temp_pdeff_dn7, var_temp_pdeff_dn8, var_temp_pdeff_dn9, var_temp_pdeff_dn10, var_temp_pdeff_dn11,)
    }
};
        var_temp_pdeff = assign14950_e20639;
        var_temp_pdeff_dn3 = assign14950_e20639_d_n3;
        var_temp_pdeff_dn4 = assign14950_e20639_d_n4;
        var_temp_pdeff_dn5 = assign14950_e20639_d_n5;
        var_temp_pdeff_dn6 = assign14950_e20639_d_n6;
        var_temp_pdeff_dn7 = assign14950_e20639_d_n7;
        var_temp_pdeff_dn8 = assign14950_e20639_d_n8;
        var_temp_pdeff_dn9 = assign14950_e20639_d_n9;
        var_temp_pdeff_dn10 = assign14950_e20639_d_n10;
        var_temp_pdeff_dn11 = assign14950_e20639_d_n11;
        var_temp_pdeff_rv = 0.0;

        let (assign14960_e20664, assign14960_e20664_d_n3, assign14960_e20664_d_n4, assign14960_e20664_d_n5, assign14960_e20664_d_n6, assign14960_e20664_d_n7, assign14960_e20664_d_n8, assign14960_e20664_d_n9, assign14960_e20664_d_n10, assign14960_e20664_d_n11,) = {
    if ((var_guard466 != 0.0) && (!(((((((var_guard459 != 0.0) || (var_guard460 != 0.0)) || (var_guard461 != 0.0)) || (var_guard462 != 0.0)) || (var_guard463 != 0.0)) || (var_guard464 != 0.0)) || (var_guard465 != 0.0)))) {
        let assign14960_e20658: f64 = (var_nuends * var_asmer);
        let assign14960_e20661: f64 = (var_nuints * var_assha);
        let assign14960_e20662: f64 = (assign14960_e20658 + assign14960_e20661);
        (assign14960_e20662, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_temp_aseff, var_temp_aseff_dn3, var_temp_aseff_dn4, var_temp_aseff_dn5, var_temp_aseff_dn6, var_temp_aseff_dn7, var_temp_aseff_dn8, var_temp_aseff_dn9, var_temp_aseff_dn10, var_temp_aseff_dn11,)
    }
};
        var_temp_aseff = assign14960_e20664;
        var_temp_aseff_dn3 = assign14960_e20664_d_n3;
        var_temp_aseff_dn4 = assign14960_e20664_d_n4;
        var_temp_aseff_dn5 = assign14960_e20664_d_n5;
        var_temp_aseff_dn6 = assign14960_e20664_d_n6;
        var_temp_aseff_dn7 = assign14960_e20664_d_n7;
        var_temp_aseff_dn8 = assign14960_e20664_d_n8;
        var_temp_aseff_dn9 = assign14960_e20664_d_n9;
        var_temp_aseff_dn10 = assign14960_e20664_d_n10;
        var_temp_aseff_dn11 = assign14960_e20664_d_n11;
        var_temp_aseff_rv = 0.0;

        let (assign14970_e20687, assign14970_e20687_d_n3, assign14970_e20687_d_n4, assign14970_e20687_d_n5, assign14970_e20687_d_n6, assign14970_e20687_d_n7, assign14970_e20687_d_n8, assign14970_e20687_d_n9, assign14970_e20687_d_n10, assign14970_e20687_d_n11,) = {
    if ((var_guard466 != 0.0) && (!(((((((var_guard459 != 0.0) || (var_guard460 != 0.0)) || (var_guard461 != 0.0)) || (var_guard462 != 0.0)) || (var_guard463 != 0.0)) || (var_guard464 != 0.0)) || (var_guard465 != 0.0)))) {
        let assign14970_e20683: f64 = (var_nuendd + var_nuintd);
        let assign14970_e20685: f64 = (assign14970_e20683 * var_adsha);
        (assign14970_e20685, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_temp_adeff, var_temp_adeff_dn3, var_temp_adeff_dn4, var_temp_adeff_dn5, var_temp_adeff_dn6, var_temp_adeff_dn7, var_temp_adeff_dn8, var_temp_adeff_dn9, var_temp_adeff_dn10, var_temp_adeff_dn11,)
    }
};
        var_temp_adeff = assign14970_e20687;
        var_temp_adeff_dn3 = assign14970_e20687_d_n3;
        var_temp_adeff_dn4 = assign14970_e20687_d_n4;
        var_temp_adeff_dn5 = assign14970_e20687_d_n5;
        var_temp_adeff_dn6 = assign14970_e20687_d_n6;
        var_temp_adeff_dn7 = assign14970_e20687_d_n7;
        var_temp_adeff_dn8 = assign14970_e20687_d_n8;
        var_temp_adeff_dn9 = assign14970_e20687_d_n9;
        var_temp_adeff_dn10 = assign14970_e20687_d_n10;
        var_temp_adeff_dn11 = assign14970_e20687_d_n11;
        var_temp_adeff_rv = 0.0;

        let (assign14980_e20714, assign14980_e20714_d_n3, assign14980_e20714_d_n4, assign14980_e20714_d_n5, assign14980_e20714_d_n6, assign14980_e20714_d_n7, assign14980_e20714_d_n8, assign14980_e20714_d_n9, assign14980_e20714_d_n10, assign14980_e20714_d_n11,) = {
    if ((var_guard467 != 0.0) && (!((((((((var_guard459 != 0.0) || (var_guard460 != 0.0)) || (var_guard461 != 0.0)) || (var_guard462 != 0.0)) || (var_guard463 != 0.0)) || (var_guard464 != 0.0)) || (var_guard465 != 0.0)) || (var_guard466 != 0.0)))) {
        let assign14980_e20708: f64 = (var_nuends * var_psmer);
        let assign14980_e20711: f64 = (var_nuints * var_pssha);
        let assign14980_e20712: f64 = (assign14980_e20708 + assign14980_e20711);
        (assign14980_e20712, ((var_nuends * var_psmer_dn3) + (var_nuints * var_pssha_dn3)), ((var_nuends * var_psmer_dn4) + (var_nuints * var_pssha_dn4)), ((var_nuends * var_psmer_dn5) + (var_nuints * var_pssha_dn5)), ((var_nuends * var_psmer_dn6) + (var_nuints * var_pssha_dn6)), ((var_nuends * var_psmer_dn7) + (var_nuints * var_pssha_dn7)), ((var_nuends * var_psmer_dn8) + (var_nuints * var_pssha_dn8)), ((var_nuends * var_psmer_dn9) + (var_nuints * var_pssha_dn9)), ((var_nuends * var_psmer_dn10) + (var_nuints * var_pssha_dn10)), ((var_nuends * var_psmer_dn11) + (var_nuints * var_pssha_dn11)),)
    } else {
        (var_temp_pseff, var_temp_pseff_dn3, var_temp_pseff_dn4, var_temp_pseff_dn5, var_temp_pseff_dn6, var_temp_pseff_dn7, var_temp_pseff_dn8, var_temp_pseff_dn9, var_temp_pseff_dn10, var_temp_pseff_dn11,)
    }
};
        var_temp_pseff = assign14980_e20714;
        var_temp_pseff_dn3 = assign14980_e20714_d_n3;
        var_temp_pseff_dn4 = assign14980_e20714_d_n4;
        var_temp_pseff_dn5 = assign14980_e20714_d_n5;
        var_temp_pseff_dn6 = assign14980_e20714_d_n6;
        var_temp_pseff_dn7 = assign14980_e20714_d_n7;
        var_temp_pseff_dn8 = assign14980_e20714_d_n8;
        var_temp_pseff_dn9 = assign14980_e20714_d_n9;
        var_temp_pseff_dn10 = assign14980_e20714_d_n10;
        var_temp_pseff_dn11 = assign14980_e20714_d_n11;
        var_temp_pseff_rv = 0.0;

        let (assign14990_e20741, assign14990_e20741_d_n3, assign14990_e20741_d_n4, assign14990_e20741_d_n5, assign14990_e20741_d_n6, assign14990_e20741_d_n7, assign14990_e20741_d_n8, assign14990_e20741_d_n9, assign14990_e20741_d_n10, assign14990_e20741_d_n11,) = {
    if ((var_guard467 != 0.0) && (!((((((((var_guard459 != 0.0) || (var_guard460 != 0.0)) || (var_guard461 != 0.0)) || (var_guard462 != 0.0)) || (var_guard463 != 0.0)) || (var_guard464 != 0.0)) || (var_guard465 != 0.0)) || (var_guard466 != 0.0)))) {
        let assign14990_e20735: f64 = (var_nuendd * var_pdmer);
        let assign14990_e20738: f64 = (var_nuintd * var_pdsha);
        let assign14990_e20739: f64 = (assign14990_e20735 + assign14990_e20738);
        (assign14990_e20739, ((var_nuendd * var_pdmer_dn3) + (var_nuintd * var_pdsha_dn3)), ((var_nuendd * var_pdmer_dn4) + (var_nuintd * var_pdsha_dn4)), ((var_nuendd * var_pdmer_dn5) + (var_nuintd * var_pdsha_dn5)), ((var_nuendd * var_pdmer_dn6) + (var_nuintd * var_pdsha_dn6)), ((var_nuendd * var_pdmer_dn7) + (var_nuintd * var_pdsha_dn7)), ((var_nuendd * var_pdmer_dn8) + (var_nuintd * var_pdsha_dn8)), ((var_nuendd * var_pdmer_dn9) + (var_nuintd * var_pdsha_dn9)), ((var_nuendd * var_pdmer_dn10) + (var_nuintd * var_pdsha_dn10)), ((var_nuendd * var_pdmer_dn11) + (var_nuintd * var_pdsha_dn11)),)
    } else {
        (var_temp_pdeff, var_temp_pdeff_dn3, var_temp_pdeff_dn4, var_temp_pdeff_dn5, var_temp_pdeff_dn6, var_temp_pdeff_dn7, var_temp_pdeff_dn8, var_temp_pdeff_dn9, var_temp_pdeff_dn10, var_temp_pdeff_dn11,)
    }
};
        var_temp_pdeff = assign14990_e20741;
        var_temp_pdeff_dn3 = assign14990_e20741_d_n3;
        var_temp_pdeff_dn4 = assign14990_e20741_d_n4;
        var_temp_pdeff_dn5 = assign14990_e20741_d_n5;
        var_temp_pdeff_dn6 = assign14990_e20741_d_n6;
        var_temp_pdeff_dn7 = assign14990_e20741_d_n7;
        var_temp_pdeff_dn8 = assign14990_e20741_d_n8;
        var_temp_pdeff_dn9 = assign14990_e20741_d_n9;
        var_temp_pdeff_dn10 = assign14990_e20741_d_n10;
        var_temp_pdeff_dn11 = assign14990_e20741_d_n11;
        var_temp_pdeff_rv = 0.0;

        let (assign15000_e20768, assign15000_e20768_d_n3, assign15000_e20768_d_n4, assign15000_e20768_d_n5, assign15000_e20768_d_n6, assign15000_e20768_d_n7, assign15000_e20768_d_n8, assign15000_e20768_d_n9, assign15000_e20768_d_n10, assign15000_e20768_d_n11,) = {
    if ((var_guard467 != 0.0) && (!((((((((var_guard459 != 0.0) || (var_guard460 != 0.0)) || (var_guard461 != 0.0)) || (var_guard462 != 0.0)) || (var_guard463 != 0.0)) || (var_guard464 != 0.0)) || (var_guard465 != 0.0)) || (var_guard466 != 0.0)))) {
        let assign15000_e20762: f64 = (var_nuends * var_asmer);
        let assign15000_e20765: f64 = (var_nuints * var_assha);
        let assign15000_e20766: f64 = (assign15000_e20762 + assign15000_e20765);
        (assign15000_e20766, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_temp_aseff, var_temp_aseff_dn3, var_temp_aseff_dn4, var_temp_aseff_dn5, var_temp_aseff_dn6, var_temp_aseff_dn7, var_temp_aseff_dn8, var_temp_aseff_dn9, var_temp_aseff_dn10, var_temp_aseff_dn11,)
    }
};
        var_temp_aseff = assign15000_e20768;
        var_temp_aseff_dn3 = assign15000_e20768_d_n3;
        var_temp_aseff_dn4 = assign15000_e20768_d_n4;
        var_temp_aseff_dn5 = assign15000_e20768_d_n5;
        var_temp_aseff_dn6 = assign15000_e20768_d_n6;
        var_temp_aseff_dn7 = assign15000_e20768_d_n7;
        var_temp_aseff_dn8 = assign15000_e20768_d_n8;
        var_temp_aseff_dn9 = assign15000_e20768_d_n9;
        var_temp_aseff_dn10 = assign15000_e20768_d_n10;
        var_temp_aseff_dn11 = assign15000_e20768_d_n11;
        var_temp_aseff_rv = 0.0;

        let (assign15010_e20795, assign15010_e20795_d_n3, assign15010_e20795_d_n4, assign15010_e20795_d_n5, assign15010_e20795_d_n6, assign15010_e20795_d_n7, assign15010_e20795_d_n8, assign15010_e20795_d_n9, assign15010_e20795_d_n10, assign15010_e20795_d_n11,) = {
    if ((var_guard467 != 0.0) && (!((((((((var_guard459 != 0.0) || (var_guard460 != 0.0)) || (var_guard461 != 0.0)) || (var_guard462 != 0.0)) || (var_guard463 != 0.0)) || (var_guard464 != 0.0)) || (var_guard465 != 0.0)) || (var_guard466 != 0.0)))) {
        let assign15010_e20789: f64 = (var_nuendd * var_admer);
        let assign15010_e20792: f64 = (var_nuintd * var_adsha);
        let assign15010_e20793: f64 = (assign15010_e20789 + assign15010_e20792);
        (assign15010_e20793, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_temp_adeff, var_temp_adeff_dn3, var_temp_adeff_dn4, var_temp_adeff_dn5, var_temp_adeff_dn6, var_temp_adeff_dn7, var_temp_adeff_dn8, var_temp_adeff_dn9, var_temp_adeff_dn10, var_temp_adeff_dn11,)
    }
};
        var_temp_adeff = assign15010_e20795;
        var_temp_adeff_dn3 = assign15010_e20795_d_n3;
        var_temp_adeff_dn4 = assign15010_e20795_d_n4;
        var_temp_adeff_dn5 = assign15010_e20795_d_n5;
        var_temp_adeff_dn6 = assign15010_e20795_d_n6;
        var_temp_adeff_dn7 = assign15010_e20795_d_n7;
        var_temp_adeff_dn8 = assign15010_e20795_d_n8;
        var_temp_adeff_dn9 = assign15010_e20795_d_n9;
        var_temp_adeff_dn10 = assign15010_e20795_d_n10;
        var_temp_adeff_dn11 = assign15010_e20795_d_n11;
        var_temp_adeff_rv = 0.0;

        let (assign15020_e20824, assign15020_e20824_d_n3, assign15020_e20824_d_n4, assign15020_e20824_d_n5, assign15020_e20824_d_n6, assign15020_e20824_d_n7, assign15020_e20824_d_n8, assign15020_e20824_d_n9, assign15020_e20824_d_n10, assign15020_e20824_d_n11,) = {
    if ((var_guard468 != 0.0) && (!(((((((((var_guard459 != 0.0) || (var_guard460 != 0.0)) || (var_guard461 != 0.0)) || (var_guard462 != 0.0)) || (var_guard463 != 0.0)) || (var_guard464 != 0.0)) || (var_guard465 != 0.0)) || (var_guard466 != 0.0)) || (var_guard467 != 0.0)))) {
        let assign15020_e20819: f64 = (p.p2 - 1.0);
        let assign15020_e20821: f64 = (assign15020_e20819 * var_pssha);
        let assign15020_e20822: f64 = (var_psiso + assign15020_e20821);
        (assign15020_e20822, (var_psiso_dn3 + (assign15020_e20819 * var_pssha_dn3)), (var_psiso_dn4 + (assign15020_e20819 * var_pssha_dn4)), (var_psiso_dn5 + (assign15020_e20819 * var_pssha_dn5)), (var_psiso_dn6 + (assign15020_e20819 * var_pssha_dn6)), (var_psiso_dn7 + (assign15020_e20819 * var_pssha_dn7)), (var_psiso_dn8 + (assign15020_e20819 * var_pssha_dn8)), (var_psiso_dn9 + (assign15020_e20819 * var_pssha_dn9)), (var_psiso_dn10 + (assign15020_e20819 * var_pssha_dn10)), (var_psiso_dn11 + (assign15020_e20819 * var_pssha_dn11)),)
    } else {
        (var_temp_pseff, var_temp_pseff_dn3, var_temp_pseff_dn4, var_temp_pseff_dn5, var_temp_pseff_dn6, var_temp_pseff_dn7, var_temp_pseff_dn8, var_temp_pseff_dn9, var_temp_pseff_dn10, var_temp_pseff_dn11,)
    }
};
        var_temp_pseff = assign15020_e20824;
        var_temp_pseff_dn3 = assign15020_e20824_d_n3;
        var_temp_pseff_dn4 = assign15020_e20824_d_n4;
        var_temp_pseff_dn5 = assign15020_e20824_d_n5;
        var_temp_pseff_dn6 = assign15020_e20824_d_n6;
        var_temp_pseff_dn7 = assign15020_e20824_d_n7;
        var_temp_pseff_dn8 = assign15020_e20824_d_n8;
        var_temp_pseff_dn9 = assign15020_e20824_d_n9;
        var_temp_pseff_dn10 = assign15020_e20824_d_n10;
        var_temp_pseff_dn11 = assign15020_e20824_d_n11;
        var_temp_pseff_rv = 0.0;

        let (assign15030_e20849, assign15030_e20849_d_n3, assign15030_e20849_d_n4, assign15030_e20849_d_n5, assign15030_e20849_d_n6, assign15030_e20849_d_n7, assign15030_e20849_d_n8, assign15030_e20849_d_n9, assign15030_e20849_d_n10, assign15030_e20849_d_n11,) = {
    if ((var_guard468 != 0.0) && (!(((((((((var_guard459 != 0.0) || (var_guard460 != 0.0)) || (var_guard461 != 0.0)) || (var_guard462 != 0.0)) || (var_guard463 != 0.0)) || (var_guard464 != 0.0)) || (var_guard465 != 0.0)) || (var_guard466 != 0.0)) || (var_guard467 != 0.0)))) {
        let assign15030_e20847: f64 = (p.p2 * var_pdsha);
        (assign15030_e20847, (p.p2 * var_pdsha_dn3), (p.p2 * var_pdsha_dn4), (p.p2 * var_pdsha_dn5), (p.p2 * var_pdsha_dn6), (p.p2 * var_pdsha_dn7), (p.p2 * var_pdsha_dn8), (p.p2 * var_pdsha_dn9), (p.p2 * var_pdsha_dn10), (p.p2 * var_pdsha_dn11),)
    } else {
        (var_temp_pdeff, var_temp_pdeff_dn3, var_temp_pdeff_dn4, var_temp_pdeff_dn5, var_temp_pdeff_dn6, var_temp_pdeff_dn7, var_temp_pdeff_dn8, var_temp_pdeff_dn9, var_temp_pdeff_dn10, var_temp_pdeff_dn11,)
    }
};
        var_temp_pdeff = assign15030_e20849;
        var_temp_pdeff_dn3 = assign15030_e20849_d_n3;
        var_temp_pdeff_dn4 = assign15030_e20849_d_n4;
        var_temp_pdeff_dn5 = assign15030_e20849_d_n5;
        var_temp_pdeff_dn6 = assign15030_e20849_d_n6;
        var_temp_pdeff_dn7 = assign15030_e20849_d_n7;
        var_temp_pdeff_dn8 = assign15030_e20849_d_n8;
        var_temp_pdeff_dn9 = assign15030_e20849_d_n9;
        var_temp_pdeff_dn10 = assign15030_e20849_d_n10;
        var_temp_pdeff_dn11 = assign15030_e20849_d_n11;
        var_temp_pdeff_rv = 0.0;

        let (assign15040_e20878, assign15040_e20878_d_n3, assign15040_e20878_d_n4, assign15040_e20878_d_n5, assign15040_e20878_d_n6, assign15040_e20878_d_n7, assign15040_e20878_d_n8, assign15040_e20878_d_n9, assign15040_e20878_d_n10, assign15040_e20878_d_n11,) = {
    if ((var_guard468 != 0.0) && (!(((((((((var_guard459 != 0.0) || (var_guard460 != 0.0)) || (var_guard461 != 0.0)) || (var_guard462 != 0.0)) || (var_guard463 != 0.0)) || (var_guard464 != 0.0)) || (var_guard465 != 0.0)) || (var_guard466 != 0.0)) || (var_guard467 != 0.0)))) {
        let assign15040_e20873: f64 = (p.p2 - 1.0);
        let assign15040_e20875: f64 = (assign15040_e20873 * var_assha);
        let assign15040_e20876: f64 = (var_asiso + assign15040_e20875);
        (assign15040_e20876, var_asiso_dn3, var_asiso_dn4, var_asiso_dn5, var_asiso_dn6, var_asiso_dn7, var_asiso_dn8, var_asiso_dn9, var_asiso_dn10, var_asiso_dn11,)
    } else {
        (var_temp_aseff, var_temp_aseff_dn3, var_temp_aseff_dn4, var_temp_aseff_dn5, var_temp_aseff_dn6, var_temp_aseff_dn7, var_temp_aseff_dn8, var_temp_aseff_dn9, var_temp_aseff_dn10, var_temp_aseff_dn11,)
    }
};
        var_temp_aseff = assign15040_e20878;
        var_temp_aseff_dn3 = assign15040_e20878_d_n3;
        var_temp_aseff_dn4 = assign15040_e20878_d_n4;
        var_temp_aseff_dn5 = assign15040_e20878_d_n5;
        var_temp_aseff_dn6 = assign15040_e20878_d_n6;
        var_temp_aseff_dn7 = assign15040_e20878_d_n7;
        var_temp_aseff_dn8 = assign15040_e20878_d_n8;
        var_temp_aseff_dn9 = assign15040_e20878_d_n9;
        var_temp_aseff_dn10 = assign15040_e20878_d_n10;
        var_temp_aseff_dn11 = assign15040_e20878_d_n11;
        var_temp_aseff_rv = 0.0;

        let (assign15050_e20903, assign15050_e20903_d_n3, assign15050_e20903_d_n4, assign15050_e20903_d_n5, assign15050_e20903_d_n6, assign15050_e20903_d_n7, assign15050_e20903_d_n8, assign15050_e20903_d_n9, assign15050_e20903_d_n10, assign15050_e20903_d_n11,) = {
    if ((var_guard468 != 0.0) && (!(((((((((var_guard459 != 0.0) || (var_guard460 != 0.0)) || (var_guard461 != 0.0)) || (var_guard462 != 0.0)) || (var_guard463 != 0.0)) || (var_guard464 != 0.0)) || (var_guard465 != 0.0)) || (var_guard466 != 0.0)) || (var_guard467 != 0.0)))) {
        let assign15050_e20901: f64 = (p.p2 * var_adsha);
        (assign15050_e20901, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_temp_adeff, var_temp_adeff_dn3, var_temp_adeff_dn4, var_temp_adeff_dn5, var_temp_adeff_dn6, var_temp_adeff_dn7, var_temp_adeff_dn8, var_temp_adeff_dn9, var_temp_adeff_dn10, var_temp_adeff_dn11,)
    }
};
        var_temp_adeff = assign15050_e20903;
        var_temp_adeff_dn3 = assign15050_e20903_d_n3;
        var_temp_adeff_dn4 = assign15050_e20903_d_n4;
        var_temp_adeff_dn5 = assign15050_e20903_d_n5;
        var_temp_adeff_dn6 = assign15050_e20903_d_n6;
        var_temp_adeff_dn7 = assign15050_e20903_d_n7;
        var_temp_adeff_dn8 = assign15050_e20903_d_n8;
        var_temp_adeff_dn9 = assign15050_e20903_d_n9;
        var_temp_adeff_dn10 = assign15050_e20903_d_n10;
        var_temp_adeff_dn11 = assign15050_e20903_d_n11;
        var_temp_adeff_rv = 0.0;

        let (assign15060_e20930, assign15060_e20930_d_n3, assign15060_e20930_d_n4, assign15060_e20930_d_n5, assign15060_e20930_d_n6, assign15060_e20930_d_n7, assign15060_e20930_d_n8, assign15060_e20930_d_n9, assign15060_e20930_d_n10, assign15060_e20930_d_n11,) = {
    if ((var_guard469 != 0.0) && (!((((((((((var_guard459 != 0.0) || (var_guard460 != 0.0)) || (var_guard461 != 0.0)) || (var_guard462 != 0.0)) || (var_guard463 != 0.0)) || (var_guard464 != 0.0)) || (var_guard465 != 0.0)) || (var_guard466 != 0.0)) || (var_guard467 != 0.0)) || (var_guard468 != 0.0)))) {
        let assign15060_e20928: f64 = (p.p2 * var_pssha);
        (assign15060_e20928, (p.p2 * var_pssha_dn3), (p.p2 * var_pssha_dn4), (p.p2 * var_pssha_dn5), (p.p2 * var_pssha_dn6), (p.p2 * var_pssha_dn7), (p.p2 * var_pssha_dn8), (p.p2 * var_pssha_dn9), (p.p2 * var_pssha_dn10), (p.p2 * var_pssha_dn11),)
    } else {
        (var_temp_pseff, var_temp_pseff_dn3, var_temp_pseff_dn4, var_temp_pseff_dn5, var_temp_pseff_dn6, var_temp_pseff_dn7, var_temp_pseff_dn8, var_temp_pseff_dn9, var_temp_pseff_dn10, var_temp_pseff_dn11,)
    }
};
        var_temp_pseff = assign15060_e20930;
        var_temp_pseff_dn3 = assign15060_e20930_d_n3;
        var_temp_pseff_dn4 = assign15060_e20930_d_n4;
        var_temp_pseff_dn5 = assign15060_e20930_d_n5;
        var_temp_pseff_dn6 = assign15060_e20930_d_n6;
        var_temp_pseff_dn7 = assign15060_e20930_d_n7;
        var_temp_pseff_dn8 = assign15060_e20930_d_n8;
        var_temp_pseff_dn9 = assign15060_e20930_d_n9;
        var_temp_pseff_dn10 = assign15060_e20930_d_n10;
        var_temp_pseff_dn11 = assign15060_e20930_d_n11;
        var_temp_pseff_rv = 0.0;

        *var_temp_adeff_slot = var_temp_adeff;
        *var_temp_adeff_dn10_slot = var_temp_adeff_dn10;
        *var_temp_adeff_dn11_slot = var_temp_adeff_dn11;
        *var_temp_adeff_dn3_slot = var_temp_adeff_dn3;
        *var_temp_adeff_dn4_slot = var_temp_adeff_dn4;
        *var_temp_adeff_dn5_slot = var_temp_adeff_dn5;
        *var_temp_adeff_dn6_slot = var_temp_adeff_dn6;
        *var_temp_adeff_dn7_slot = var_temp_adeff_dn7;
        *var_temp_adeff_dn8_slot = var_temp_adeff_dn8;
        *var_temp_adeff_dn9_slot = var_temp_adeff_dn9;
        *var_temp_adeff_rv_slot = var_temp_adeff_rv;
        *var_temp_aseff_slot = var_temp_aseff;
        *var_temp_aseff_dn10_slot = var_temp_aseff_dn10;
        *var_temp_aseff_dn11_slot = var_temp_aseff_dn11;
        *var_temp_aseff_dn3_slot = var_temp_aseff_dn3;
        *var_temp_aseff_dn4_slot = var_temp_aseff_dn4;
        *var_temp_aseff_dn5_slot = var_temp_aseff_dn5;
        *var_temp_aseff_dn6_slot = var_temp_aseff_dn6;
        *var_temp_aseff_dn7_slot = var_temp_aseff_dn7;
        *var_temp_aseff_dn8_slot = var_temp_aseff_dn8;
        *var_temp_aseff_dn9_slot = var_temp_aseff_dn9;
        *var_temp_aseff_rv_slot = var_temp_aseff_rv;
        *var_temp_pdeff_slot = var_temp_pdeff;
        *var_temp_pdeff_dn10_slot = var_temp_pdeff_dn10;
        *var_temp_pdeff_dn11_slot = var_temp_pdeff_dn11;
        *var_temp_pdeff_dn3_slot = var_temp_pdeff_dn3;
        *var_temp_pdeff_dn4_slot = var_temp_pdeff_dn4;
        *var_temp_pdeff_dn5_slot = var_temp_pdeff_dn5;
        *var_temp_pdeff_dn6_slot = var_temp_pdeff_dn6;
        *var_temp_pdeff_dn7_slot = var_temp_pdeff_dn7;
        *var_temp_pdeff_dn8_slot = var_temp_pdeff_dn8;
        *var_temp_pdeff_dn9_slot = var_temp_pdeff_dn9;
        *var_temp_pdeff_rv_slot = var_temp_pdeff_rv;
        *var_temp_pseff_slot = var_temp_pseff;
        *var_temp_pseff_dn10_slot = var_temp_pseff_dn10;
        *var_temp_pseff_dn11_slot = var_temp_pseff_dn11;
        *var_temp_pseff_dn3_slot = var_temp_pseff_dn3;
        *var_temp_pseff_dn4_slot = var_temp_pseff_dn4;
        *var_temp_pseff_dn5_slot = var_temp_pseff_dn5;
        *var_temp_pseff_dn6_slot = var_temp_pseff_dn6;
        *var_temp_pseff_dn7_slot = var_temp_pseff_dn7;
        *var_temp_pseff_dn8_slot = var_temp_pseff_dn8;
        *var_temp_pseff_dn9_slot = var_temp_pseff_dn9;
        *var_temp_pseff_rv_slot = var_temp_pseff_rv;
    }

    pub(super) fn stamp_reactive_block_27(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        var_adiso: f64,
        var_adiso_dn10: f64,
        var_adiso_dn11: f64,
        var_adiso_dn3: f64,
        var_adiso_dn4: f64,
        var_adiso_dn5: f64,
        var_adiso_dn6: f64,
        var_adiso_dn7: f64,
        var_adiso_dn8: f64,
        var_adiso_dn9: f64,
        var_adsha: f64,
        var_assha: f64,
        var_guard459: f64,
        var_guard460: f64,
        var_guard461: f64,
        var_guard462: f64,
        var_guard463: f64,
        var_guard464: f64,
        var_guard465: f64,
        var_guard466: f64,
        var_guard467: f64,
        var_guard468: f64,
        var_guard469: f64,
        var_lnew: f64,
        var_pdiso: f64,
        var_pdiso_dn10: f64,
        var_pdiso_dn11: f64,
        var_pdiso_dn3: f64,
        var_pdiso_dn4: f64,
        var_pdiso_dn5: f64,
        var_pdiso_dn6: f64,
        var_pdiso_dn7: f64,
        var_pdiso_dn8: f64,
        var_pdiso_dn9: f64,
        var_pdsha: f64,
        var_pdsha_dn10: f64,
        var_pdsha_dn11: f64,
        var_pdsha_dn3: f64,
        var_pdsha_dn4: f64,
        var_pdsha_dn5: f64,
        var_pdsha_dn6: f64,
        var_pdsha_dn7: f64,
        var_pdsha_dn8: f64,
        var_pdsha_dn9: f64,
        var_weffcj: f64,
        var_wnew: f64,
        var_adeff_slot: &mut f64,
        var_adeff_dn10_slot: &mut f64,
        var_adeff_dn11_slot: &mut f64,
        var_adeff_dn3_slot: &mut f64,
        var_adeff_dn4_slot: &mut f64,
        var_adeff_dn5_slot: &mut f64,
        var_adeff_dn6_slot: &mut f64,
        var_adeff_dn7_slot: &mut f64,
        var_adeff_dn8_slot: &mut f64,
        var_adeff_dn9_slot: &mut f64,
        var_adeff_rv_slot: &mut f64,
        var_aseff_slot: &mut f64,
        var_aseff_dn10_slot: &mut f64,
        var_aseff_dn11_slot: &mut f64,
        var_aseff_dn3_slot: &mut f64,
        var_aseff_dn4_slot: &mut f64,
        var_aseff_dn5_slot: &mut f64,
        var_aseff_dn6_slot: &mut f64,
        var_aseff_dn7_slot: &mut f64,
        var_aseff_dn8_slot: &mut f64,
        var_aseff_dn9_slot: &mut f64,
        var_aseff_rv_slot: &mut f64,
        var_guard470_slot: &mut f64,
        var_guard470_rv_slot: &mut f64,
        var_guard471_slot: &mut f64,
        var_guard471_rv_slot: &mut f64,
        var_guard472_slot: &mut f64,
        var_guard472_rv_slot: &mut f64,
        var_guard473_slot: &mut f64,
        var_guard473_rv_slot: &mut f64,
        var_guard474_slot: &mut f64,
        var_guard474_rv_slot: &mut f64,
        var_guard475_slot: &mut f64,
        var_guard475_rv_slot: &mut f64,
        var_guard476_slot: &mut f64,
        var_guard476_rv_slot: &mut f64,
        var_guard477_slot: &mut f64,
        var_guard477_rv_slot: &mut f64,
        var_guard478_slot: &mut f64,
        var_guard478_rv_slot: &mut f64,
        var_guard479_slot: &mut f64,
        var_guard479_rv_slot: &mut f64,
        var_guard480_slot: &mut f64,
        var_guard480_rv_slot: &mut f64,
        var_pdeff_slot: &mut f64,
        var_pdeff_dn10_slot: &mut f64,
        var_pdeff_dn11_slot: &mut f64,
        var_pdeff_dn3_slot: &mut f64,
        var_pdeff_dn4_slot: &mut f64,
        var_pdeff_dn5_slot: &mut f64,
        var_pdeff_dn6_slot: &mut f64,
        var_pdeff_dn7_slot: &mut f64,
        var_pdeff_dn8_slot: &mut f64,
        var_pdeff_dn9_slot: &mut f64,
        var_pdeff_rv_slot: &mut f64,
        var_pseff_slot: &mut f64,
        var_pseff_dn10_slot: &mut f64,
        var_pseff_dn11_slot: &mut f64,
        var_pseff_dn3_slot: &mut f64,
        var_pseff_dn4_slot: &mut f64,
        var_pseff_dn5_slot: &mut f64,
        var_pseff_dn6_slot: &mut f64,
        var_pseff_dn7_slot: &mut f64,
        var_pseff_dn8_slot: &mut f64,
        var_pseff_dn9_slot: &mut f64,
        var_pseff_rv_slot: &mut f64,
        var_t0_slot: &mut f64,
        var_t0_dn10_slot: &mut f64,
        var_t0_dn11_slot: &mut f64,
        var_t0_dn3_slot: &mut f64,
        var_t0_dn4_slot: &mut f64,
        var_t0_dn5_slot: &mut f64,
        var_t0_dn6_slot: &mut f64,
        var_t0_dn7_slot: &mut f64,
        var_t0_dn8_slot: &mut f64,
        var_t0_dn9_slot: &mut f64,
        var_t0_rv_slot: &mut f64,
        var_temp_adeff_slot: &mut f64,
        var_temp_adeff_dn10_slot: &mut f64,
        var_temp_adeff_dn11_slot: &mut f64,
        var_temp_adeff_dn3_slot: &mut f64,
        var_temp_adeff_dn4_slot: &mut f64,
        var_temp_adeff_dn5_slot: &mut f64,
        var_temp_adeff_dn6_slot: &mut f64,
        var_temp_adeff_dn7_slot: &mut f64,
        var_temp_adeff_dn8_slot: &mut f64,
        var_temp_adeff_dn9_slot: &mut f64,
        var_temp_adeff_rv_slot: &mut f64,
        var_temp_aseff_slot: &mut f64,
        var_temp_aseff_dn10_slot: &mut f64,
        var_temp_aseff_dn11_slot: &mut f64,
        var_temp_aseff_dn3_slot: &mut f64,
        var_temp_aseff_dn4_slot: &mut f64,
        var_temp_aseff_dn5_slot: &mut f64,
        var_temp_aseff_dn6_slot: &mut f64,
        var_temp_aseff_dn7_slot: &mut f64,
        var_temp_aseff_dn8_slot: &mut f64,
        var_temp_aseff_dn9_slot: &mut f64,
        var_temp_aseff_rv_slot: &mut f64,
        var_temp_pdeff_slot: &mut f64,
        var_temp_pdeff_dn10_slot: &mut f64,
        var_temp_pdeff_dn11_slot: &mut f64,
        var_temp_pdeff_dn3_slot: &mut f64,
        var_temp_pdeff_dn4_slot: &mut f64,
        var_temp_pdeff_dn5_slot: &mut f64,
        var_temp_pdeff_dn6_slot: &mut f64,
        var_temp_pdeff_dn7_slot: &mut f64,
        var_temp_pdeff_dn8_slot: &mut f64,
        var_temp_pdeff_dn9_slot: &mut f64,
        var_temp_pdeff_rv_slot: &mut f64,
        var_temp_pseff_slot: &mut f64,
        var_temp_pseff_dn10_slot: &mut f64,
        var_temp_pseff_dn11_slot: &mut f64,
        var_temp_pseff_dn3_slot: &mut f64,
        var_temp_pseff_dn4_slot: &mut f64,
        var_temp_pseff_dn5_slot: &mut f64,
        var_temp_pseff_dn6_slot: &mut f64,
        var_temp_pseff_dn7_slot: &mut f64,
        var_temp_pseff_dn8_slot: &mut f64,
        var_temp_pseff_dn9_slot: &mut f64,
        var_temp_pseff_rv_slot: &mut f64,
        var_w_tmp_stress_slot: &mut f64,
        var_w_tmp_stress_rv_slot: &mut f64,
    ) {
        let mut var_adeff: f64 = *var_adeff_slot;
        let mut var_adeff_dn10: f64 = *var_adeff_dn10_slot;
        let mut var_adeff_dn11: f64 = *var_adeff_dn11_slot;
        let mut var_adeff_dn3: f64 = *var_adeff_dn3_slot;
        let mut var_adeff_dn4: f64 = *var_adeff_dn4_slot;
        let mut var_adeff_dn5: f64 = *var_adeff_dn5_slot;
        let mut var_adeff_dn6: f64 = *var_adeff_dn6_slot;
        let mut var_adeff_dn7: f64 = *var_adeff_dn7_slot;
        let mut var_adeff_dn8: f64 = *var_adeff_dn8_slot;
        let mut var_adeff_dn9: f64 = *var_adeff_dn9_slot;
        let mut var_adeff_rv: f64 = *var_adeff_rv_slot;
        let mut var_aseff: f64 = *var_aseff_slot;
        let mut var_aseff_dn10: f64 = *var_aseff_dn10_slot;
        let mut var_aseff_dn11: f64 = *var_aseff_dn11_slot;
        let mut var_aseff_dn3: f64 = *var_aseff_dn3_slot;
        let mut var_aseff_dn4: f64 = *var_aseff_dn4_slot;
        let mut var_aseff_dn5: f64 = *var_aseff_dn5_slot;
        let mut var_aseff_dn6: f64 = *var_aseff_dn6_slot;
        let mut var_aseff_dn7: f64 = *var_aseff_dn7_slot;
        let mut var_aseff_dn8: f64 = *var_aseff_dn8_slot;
        let mut var_aseff_dn9: f64 = *var_aseff_dn9_slot;
        let mut var_aseff_rv: f64 = *var_aseff_rv_slot;
        let mut var_guard470: f64 = *var_guard470_slot;
        let mut var_guard470_rv: f64 = *var_guard470_rv_slot;
        let mut var_guard471: f64 = *var_guard471_slot;
        let mut var_guard471_rv: f64 = *var_guard471_rv_slot;
        let mut var_guard472: f64 = *var_guard472_slot;
        let mut var_guard472_rv: f64 = *var_guard472_rv_slot;
        let mut var_guard473: f64 = *var_guard473_slot;
        let mut var_guard473_rv: f64 = *var_guard473_rv_slot;
        let mut var_guard474: f64 = *var_guard474_slot;
        let mut var_guard474_rv: f64 = *var_guard474_rv_slot;
        let mut var_guard475: f64 = *var_guard475_slot;
        let mut var_guard475_rv: f64 = *var_guard475_rv_slot;
        let mut var_guard476: f64 = *var_guard476_slot;
        let mut var_guard476_rv: f64 = *var_guard476_rv_slot;
        let mut var_guard477: f64 = *var_guard477_slot;
        let mut var_guard477_rv: f64 = *var_guard477_rv_slot;
        let mut var_guard478: f64 = *var_guard478_slot;
        let mut var_guard478_rv: f64 = *var_guard478_rv_slot;
        let mut var_guard479: f64 = *var_guard479_slot;
        let mut var_guard479_rv: f64 = *var_guard479_rv_slot;
        let mut var_guard480: f64 = *var_guard480_slot;
        let mut var_guard480_rv: f64 = *var_guard480_rv_slot;
        let mut var_pdeff: f64 = *var_pdeff_slot;
        let mut var_pdeff_dn10: f64 = *var_pdeff_dn10_slot;
        let mut var_pdeff_dn11: f64 = *var_pdeff_dn11_slot;
        let mut var_pdeff_dn3: f64 = *var_pdeff_dn3_slot;
        let mut var_pdeff_dn4: f64 = *var_pdeff_dn4_slot;
        let mut var_pdeff_dn5: f64 = *var_pdeff_dn5_slot;
        let mut var_pdeff_dn6: f64 = *var_pdeff_dn6_slot;
        let mut var_pdeff_dn7: f64 = *var_pdeff_dn7_slot;
        let mut var_pdeff_dn8: f64 = *var_pdeff_dn8_slot;
        let mut var_pdeff_dn9: f64 = *var_pdeff_dn9_slot;
        let mut var_pdeff_rv: f64 = *var_pdeff_rv_slot;
        let mut var_pseff: f64 = *var_pseff_slot;
        let mut var_pseff_dn10: f64 = *var_pseff_dn10_slot;
        let mut var_pseff_dn11: f64 = *var_pseff_dn11_slot;
        let mut var_pseff_dn3: f64 = *var_pseff_dn3_slot;
        let mut var_pseff_dn4: f64 = *var_pseff_dn4_slot;
        let mut var_pseff_dn5: f64 = *var_pseff_dn5_slot;
        let mut var_pseff_dn6: f64 = *var_pseff_dn6_slot;
        let mut var_pseff_dn7: f64 = *var_pseff_dn7_slot;
        let mut var_pseff_dn8: f64 = *var_pseff_dn8_slot;
        let mut var_pseff_dn9: f64 = *var_pseff_dn9_slot;
        let mut var_pseff_rv: f64 = *var_pseff_rv_slot;
        let mut var_t0: f64 = *var_t0_slot;
        let mut var_t0_dn10: f64 = *var_t0_dn10_slot;
        let mut var_t0_dn11: f64 = *var_t0_dn11_slot;
        let mut var_t0_dn3: f64 = *var_t0_dn3_slot;
        let mut var_t0_dn4: f64 = *var_t0_dn4_slot;
        let mut var_t0_dn5: f64 = *var_t0_dn5_slot;
        let mut var_t0_dn6: f64 = *var_t0_dn6_slot;
        let mut var_t0_dn7: f64 = *var_t0_dn7_slot;
        let mut var_t0_dn8: f64 = *var_t0_dn8_slot;
        let mut var_t0_dn9: f64 = *var_t0_dn9_slot;
        let mut var_t0_rv: f64 = *var_t0_rv_slot;
        let mut var_temp_adeff: f64 = *var_temp_adeff_slot;
        let mut var_temp_adeff_dn10: f64 = *var_temp_adeff_dn10_slot;
        let mut var_temp_adeff_dn11: f64 = *var_temp_adeff_dn11_slot;
        let mut var_temp_adeff_dn3: f64 = *var_temp_adeff_dn3_slot;
        let mut var_temp_adeff_dn4: f64 = *var_temp_adeff_dn4_slot;
        let mut var_temp_adeff_dn5: f64 = *var_temp_adeff_dn5_slot;
        let mut var_temp_adeff_dn6: f64 = *var_temp_adeff_dn6_slot;
        let mut var_temp_adeff_dn7: f64 = *var_temp_adeff_dn7_slot;
        let mut var_temp_adeff_dn8: f64 = *var_temp_adeff_dn8_slot;
        let mut var_temp_adeff_dn9: f64 = *var_temp_adeff_dn9_slot;
        let mut var_temp_adeff_rv: f64 = *var_temp_adeff_rv_slot;
        let mut var_temp_aseff: f64 = *var_temp_aseff_slot;
        let mut var_temp_aseff_dn10: f64 = *var_temp_aseff_dn10_slot;
        let mut var_temp_aseff_dn11: f64 = *var_temp_aseff_dn11_slot;
        let mut var_temp_aseff_dn3: f64 = *var_temp_aseff_dn3_slot;
        let mut var_temp_aseff_dn4: f64 = *var_temp_aseff_dn4_slot;
        let mut var_temp_aseff_dn5: f64 = *var_temp_aseff_dn5_slot;
        let mut var_temp_aseff_dn6: f64 = *var_temp_aseff_dn6_slot;
        let mut var_temp_aseff_dn7: f64 = *var_temp_aseff_dn7_slot;
        let mut var_temp_aseff_dn8: f64 = *var_temp_aseff_dn8_slot;
        let mut var_temp_aseff_dn9: f64 = *var_temp_aseff_dn9_slot;
        let mut var_temp_aseff_rv: f64 = *var_temp_aseff_rv_slot;
        let mut var_temp_pdeff: f64 = *var_temp_pdeff_slot;
        let mut var_temp_pdeff_dn10: f64 = *var_temp_pdeff_dn10_slot;
        let mut var_temp_pdeff_dn11: f64 = *var_temp_pdeff_dn11_slot;
        let mut var_temp_pdeff_dn3: f64 = *var_temp_pdeff_dn3_slot;
        let mut var_temp_pdeff_dn4: f64 = *var_temp_pdeff_dn4_slot;
        let mut var_temp_pdeff_dn5: f64 = *var_temp_pdeff_dn5_slot;
        let mut var_temp_pdeff_dn6: f64 = *var_temp_pdeff_dn6_slot;
        let mut var_temp_pdeff_dn7: f64 = *var_temp_pdeff_dn7_slot;
        let mut var_temp_pdeff_dn8: f64 = *var_temp_pdeff_dn8_slot;
        let mut var_temp_pdeff_dn9: f64 = *var_temp_pdeff_dn9_slot;
        let mut var_temp_pdeff_rv: f64 = *var_temp_pdeff_rv_slot;
        let mut var_temp_pseff: f64 = *var_temp_pseff_slot;
        let mut var_temp_pseff_dn10: f64 = *var_temp_pseff_dn10_slot;
        let mut var_temp_pseff_dn11: f64 = *var_temp_pseff_dn11_slot;
        let mut var_temp_pseff_dn3: f64 = *var_temp_pseff_dn3_slot;
        let mut var_temp_pseff_dn4: f64 = *var_temp_pseff_dn4_slot;
        let mut var_temp_pseff_dn5: f64 = *var_temp_pseff_dn5_slot;
        let mut var_temp_pseff_dn6: f64 = *var_temp_pseff_dn6_slot;
        let mut var_temp_pseff_dn7: f64 = *var_temp_pseff_dn7_slot;
        let mut var_temp_pseff_dn8: f64 = *var_temp_pseff_dn8_slot;
        let mut var_temp_pseff_dn9: f64 = *var_temp_pseff_dn9_slot;
        let mut var_temp_pseff_rv: f64 = *var_temp_pseff_rv_slot;
        let mut var_w_tmp_stress: f64 = *var_w_tmp_stress_slot;
        let mut var_w_tmp_stress_rv: f64 = *var_w_tmp_stress_rv_slot;

        let (assign15070_e20961, assign15070_e20961_d_n3, assign15070_e20961_d_n4, assign15070_e20961_d_n5, assign15070_e20961_d_n6, assign15070_e20961_d_n7, assign15070_e20961_d_n8, assign15070_e20961_d_n9, assign15070_e20961_d_n10, assign15070_e20961_d_n11,) = {
    if ((var_guard469 != 0.0) && (!((((((((((var_guard459 != 0.0) || (var_guard460 != 0.0)) || (var_guard461 != 0.0)) || (var_guard462 != 0.0)) || (var_guard463 != 0.0)) || (var_guard464 != 0.0)) || (var_guard465 != 0.0)) || (var_guard466 != 0.0)) || (var_guard467 != 0.0)) || (var_guard468 != 0.0)))) {
        let assign15070_e20956: f64 = (p.p2 - 1.0);
        let assign15070_e20958: f64 = (assign15070_e20956 * var_pdsha);
        let assign15070_e20959: f64 = (var_pdiso + assign15070_e20958);
        (assign15070_e20959, (var_pdiso_dn3 + (assign15070_e20956 * var_pdsha_dn3)), (var_pdiso_dn4 + (assign15070_e20956 * var_pdsha_dn4)), (var_pdiso_dn5 + (assign15070_e20956 * var_pdsha_dn5)), (var_pdiso_dn6 + (assign15070_e20956 * var_pdsha_dn6)), (var_pdiso_dn7 + (assign15070_e20956 * var_pdsha_dn7)), (var_pdiso_dn8 + (assign15070_e20956 * var_pdsha_dn8)), (var_pdiso_dn9 + (assign15070_e20956 * var_pdsha_dn9)), (var_pdiso_dn10 + (assign15070_e20956 * var_pdsha_dn10)), (var_pdiso_dn11 + (assign15070_e20956 * var_pdsha_dn11)),)
    } else {
        (var_temp_pdeff, var_temp_pdeff_dn3, var_temp_pdeff_dn4, var_temp_pdeff_dn5, var_temp_pdeff_dn6, var_temp_pdeff_dn7, var_temp_pdeff_dn8, var_temp_pdeff_dn9, var_temp_pdeff_dn10, var_temp_pdeff_dn11,)
    }
};
        var_temp_pdeff = assign15070_e20961;
        var_temp_pdeff_dn3 = assign15070_e20961_d_n3;
        var_temp_pdeff_dn4 = assign15070_e20961_d_n4;
        var_temp_pdeff_dn5 = assign15070_e20961_d_n5;
        var_temp_pdeff_dn6 = assign15070_e20961_d_n6;
        var_temp_pdeff_dn7 = assign15070_e20961_d_n7;
        var_temp_pdeff_dn8 = assign15070_e20961_d_n8;
        var_temp_pdeff_dn9 = assign15070_e20961_d_n9;
        var_temp_pdeff_dn10 = assign15070_e20961_d_n10;
        var_temp_pdeff_dn11 = assign15070_e20961_d_n11;
        var_temp_pdeff_rv = 0.0;

        let (assign15080_e20988, assign15080_e20988_d_n3, assign15080_e20988_d_n4, assign15080_e20988_d_n5, assign15080_e20988_d_n6, assign15080_e20988_d_n7, assign15080_e20988_d_n8, assign15080_e20988_d_n9, assign15080_e20988_d_n10, assign15080_e20988_d_n11,) = {
    if ((var_guard469 != 0.0) && (!((((((((((var_guard459 != 0.0) || (var_guard460 != 0.0)) || (var_guard461 != 0.0)) || (var_guard462 != 0.0)) || (var_guard463 != 0.0)) || (var_guard464 != 0.0)) || (var_guard465 != 0.0)) || (var_guard466 != 0.0)) || (var_guard467 != 0.0)) || (var_guard468 != 0.0)))) {
        let assign15080_e20986: f64 = (p.p2 * var_assha);
        (assign15080_e20986, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_temp_aseff, var_temp_aseff_dn3, var_temp_aseff_dn4, var_temp_aseff_dn5, var_temp_aseff_dn6, var_temp_aseff_dn7, var_temp_aseff_dn8, var_temp_aseff_dn9, var_temp_aseff_dn10, var_temp_aseff_dn11,)
    }
};
        var_temp_aseff = assign15080_e20988;
        var_temp_aseff_dn3 = assign15080_e20988_d_n3;
        var_temp_aseff_dn4 = assign15080_e20988_d_n4;
        var_temp_aseff_dn5 = assign15080_e20988_d_n5;
        var_temp_aseff_dn6 = assign15080_e20988_d_n6;
        var_temp_aseff_dn7 = assign15080_e20988_d_n7;
        var_temp_aseff_dn8 = assign15080_e20988_d_n8;
        var_temp_aseff_dn9 = assign15080_e20988_d_n9;
        var_temp_aseff_dn10 = assign15080_e20988_d_n10;
        var_temp_aseff_dn11 = assign15080_e20988_d_n11;
        var_temp_aseff_rv = 0.0;

        let (assign15090_e21019, assign15090_e21019_d_n3, assign15090_e21019_d_n4, assign15090_e21019_d_n5, assign15090_e21019_d_n6, assign15090_e21019_d_n7, assign15090_e21019_d_n8, assign15090_e21019_d_n9, assign15090_e21019_d_n10, assign15090_e21019_d_n11,) = {
    if ((var_guard469 != 0.0) && (!((((((((((var_guard459 != 0.0) || (var_guard460 != 0.0)) || (var_guard461 != 0.0)) || (var_guard462 != 0.0)) || (var_guard463 != 0.0)) || (var_guard464 != 0.0)) || (var_guard465 != 0.0)) || (var_guard466 != 0.0)) || (var_guard467 != 0.0)) || (var_guard468 != 0.0)))) {
        let assign15090_e21014: f64 = (p.p2 - 1.0);
        let assign15090_e21016: f64 = (assign15090_e21014 * var_adsha);
        let assign15090_e21017: f64 = (var_adiso + assign15090_e21016);
        (assign15090_e21017, var_adiso_dn3, var_adiso_dn4, var_adiso_dn5, var_adiso_dn6, var_adiso_dn7, var_adiso_dn8, var_adiso_dn9, var_adiso_dn10, var_adiso_dn11,)
    } else {
        (var_temp_adeff, var_temp_adeff_dn3, var_temp_adeff_dn4, var_temp_adeff_dn5, var_temp_adeff_dn6, var_temp_adeff_dn7, var_temp_adeff_dn8, var_temp_adeff_dn9, var_temp_adeff_dn10, var_temp_adeff_dn11,)
    }
};
        var_temp_adeff = assign15090_e21019;
        var_temp_adeff_dn3 = assign15090_e21019_d_n3;
        var_temp_adeff_dn4 = assign15090_e21019_d_n4;
        var_temp_adeff_dn5 = assign15090_e21019_d_n5;
        var_temp_adeff_dn6 = assign15090_e21019_d_n6;
        var_temp_adeff_dn7 = assign15090_e21019_d_n7;
        var_temp_adeff_dn8 = assign15090_e21019_d_n8;
        var_temp_adeff_dn9 = assign15090_e21019_d_n9;
        var_temp_adeff_dn10 = assign15090_e21019_d_n10;
        var_temp_adeff_dn11 = assign15090_e21019_d_n11;
        var_temp_adeff_rv = 0.0;

        let (assign15100_e21044, assign15100_e21044_d_n3, assign15100_e21044_d_n4, assign15100_e21044_d_n5, assign15100_e21044_d_n6, assign15100_e21044_d_n7, assign15100_e21044_d_n8, assign15100_e21044_d_n9, assign15100_e21044_d_n10, assign15100_e21044_d_n11,) = {
    if (!(((((((((((var_guard459 != 0.0) || (var_guard460 != 0.0)) || (var_guard461 != 0.0)) || (var_guard462 != 0.0)) || (var_guard463 != 0.0)) || (var_guard464 != 0.0)) || (var_guard465 != 0.0)) || (var_guard466 != 0.0)) || (var_guard467 != 0.0)) || (var_guard468 != 0.0)) || (var_guard469 != 0.0))) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_temp_pseff, var_temp_pseff_dn3, var_temp_pseff_dn4, var_temp_pseff_dn5, var_temp_pseff_dn6, var_temp_pseff_dn7, var_temp_pseff_dn8, var_temp_pseff_dn9, var_temp_pseff_dn10, var_temp_pseff_dn11,)
    }
};
        var_temp_pseff = assign15100_e21044;
        var_temp_pseff_dn3 = assign15100_e21044_d_n3;
        var_temp_pseff_dn4 = assign15100_e21044_d_n4;
        var_temp_pseff_dn5 = assign15100_e21044_d_n5;
        var_temp_pseff_dn6 = assign15100_e21044_d_n6;
        var_temp_pseff_dn7 = assign15100_e21044_d_n7;
        var_temp_pseff_dn8 = assign15100_e21044_d_n8;
        var_temp_pseff_dn9 = assign15100_e21044_d_n9;
        var_temp_pseff_dn10 = assign15100_e21044_d_n10;
        var_temp_pseff_dn11 = assign15100_e21044_d_n11;
        var_temp_pseff_rv = 0.0;

        let (assign15110_e21069, assign15110_e21069_d_n3, assign15110_e21069_d_n4, assign15110_e21069_d_n5, assign15110_e21069_d_n6, assign15110_e21069_d_n7, assign15110_e21069_d_n8, assign15110_e21069_d_n9, assign15110_e21069_d_n10, assign15110_e21069_d_n11,) = {
    if (!(((((((((((var_guard459 != 0.0) || (var_guard460 != 0.0)) || (var_guard461 != 0.0)) || (var_guard462 != 0.0)) || (var_guard463 != 0.0)) || (var_guard464 != 0.0)) || (var_guard465 != 0.0)) || (var_guard466 != 0.0)) || (var_guard467 != 0.0)) || (var_guard468 != 0.0)) || (var_guard469 != 0.0))) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_temp_pdeff, var_temp_pdeff_dn3, var_temp_pdeff_dn4, var_temp_pdeff_dn5, var_temp_pdeff_dn6, var_temp_pdeff_dn7, var_temp_pdeff_dn8, var_temp_pdeff_dn9, var_temp_pdeff_dn10, var_temp_pdeff_dn11,)
    }
};
        var_temp_pdeff = assign15110_e21069;
        var_temp_pdeff_dn3 = assign15110_e21069_d_n3;
        var_temp_pdeff_dn4 = assign15110_e21069_d_n4;
        var_temp_pdeff_dn5 = assign15110_e21069_d_n5;
        var_temp_pdeff_dn6 = assign15110_e21069_d_n6;
        var_temp_pdeff_dn7 = assign15110_e21069_d_n7;
        var_temp_pdeff_dn8 = assign15110_e21069_d_n8;
        var_temp_pdeff_dn9 = assign15110_e21069_d_n9;
        var_temp_pdeff_dn10 = assign15110_e21069_d_n10;
        var_temp_pdeff_dn11 = assign15110_e21069_d_n11;
        var_temp_pdeff_rv = 0.0;

        let (assign15120_e21094, assign15120_e21094_d_n3, assign15120_e21094_d_n4, assign15120_e21094_d_n5, assign15120_e21094_d_n6, assign15120_e21094_d_n7, assign15120_e21094_d_n8, assign15120_e21094_d_n9, assign15120_e21094_d_n10, assign15120_e21094_d_n11,) = {
    if (!(((((((((((var_guard459 != 0.0) || (var_guard460 != 0.0)) || (var_guard461 != 0.0)) || (var_guard462 != 0.0)) || (var_guard463 != 0.0)) || (var_guard464 != 0.0)) || (var_guard465 != 0.0)) || (var_guard466 != 0.0)) || (var_guard467 != 0.0)) || (var_guard468 != 0.0)) || (var_guard469 != 0.0))) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_temp_aseff, var_temp_aseff_dn3, var_temp_aseff_dn4, var_temp_aseff_dn5, var_temp_aseff_dn6, var_temp_aseff_dn7, var_temp_aseff_dn8, var_temp_aseff_dn9, var_temp_aseff_dn10, var_temp_aseff_dn11,)
    }
};
        var_temp_aseff = assign15120_e21094;
        var_temp_aseff_dn3 = assign15120_e21094_d_n3;
        var_temp_aseff_dn4 = assign15120_e21094_d_n4;
        var_temp_aseff_dn5 = assign15120_e21094_d_n5;
        var_temp_aseff_dn6 = assign15120_e21094_d_n6;
        var_temp_aseff_dn7 = assign15120_e21094_d_n7;
        var_temp_aseff_dn8 = assign15120_e21094_d_n8;
        var_temp_aseff_dn9 = assign15120_e21094_d_n9;
        var_temp_aseff_dn10 = assign15120_e21094_d_n10;
        var_temp_aseff_dn11 = assign15120_e21094_d_n11;
        var_temp_aseff_rv = 0.0;

        let (assign15130_e21119, assign15130_e21119_d_n3, assign15130_e21119_d_n4, assign15130_e21119_d_n5, assign15130_e21119_d_n6, assign15130_e21119_d_n7, assign15130_e21119_d_n8, assign15130_e21119_d_n9, assign15130_e21119_d_n10, assign15130_e21119_d_n11,) = {
    if (!(((((((((((var_guard459 != 0.0) || (var_guard460 != 0.0)) || (var_guard461 != 0.0)) || (var_guard462 != 0.0)) || (var_guard463 != 0.0)) || (var_guard464 != 0.0)) || (var_guard465 != 0.0)) || (var_guard466 != 0.0)) || (var_guard467 != 0.0)) || (var_guard468 != 0.0)) || (var_guard469 != 0.0))) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_temp_adeff, var_temp_adeff_dn3, var_temp_adeff_dn4, var_temp_adeff_dn5, var_temp_adeff_dn6, var_temp_adeff_dn7, var_temp_adeff_dn8, var_temp_adeff_dn9, var_temp_adeff_dn10, var_temp_adeff_dn11,)
    }
};
        var_temp_adeff = assign15130_e21119;
        var_temp_adeff_dn3 = assign15130_e21119_d_n3;
        var_temp_adeff_dn4 = assign15130_e21119_d_n4;
        var_temp_adeff_dn5 = assign15130_e21119_d_n5;
        var_temp_adeff_dn6 = assign15130_e21119_d_n6;
        var_temp_adeff_dn7 = assign15130_e21119_d_n7;
        var_temp_adeff_dn8 = assign15130_e21119_d_n8;
        var_temp_adeff_dn9 = assign15130_e21119_d_n9;
        var_temp_adeff_dn10 = assign15130_e21119_d_n10;
        var_temp_adeff_dn11 = assign15130_e21119_d_n11;
        var_temp_adeff_rv = 0.0;

        let assign15140_e21121: f64 = if param_given[17] { 1.0 } else { 0.0 };
        var_guard470 = assign15140_e21121;
        var_guard470_rv = 0.0;

        let (assign15150_e21129, assign15150_e21129_d_n3, assign15150_e21129_d_n4, assign15150_e21129_d_n5, assign15150_e21129_d_n6, assign15150_e21129_d_n7, assign15150_e21129_d_n8, assign15150_e21129_d_n9, assign15150_e21129_d_n10, assign15150_e21129_d_n11,) = {
    if (var_guard470 != 0.0) {
        let assign15150_e21125: f64 = (p.p17 * p.p50);
        let assign15150_e21127: f64 = (assign15150_e21125 * p.p49);
        (assign15150_e21127, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_aseff, var_aseff_dn3, var_aseff_dn4, var_aseff_dn5, var_aseff_dn6, var_aseff_dn7, var_aseff_dn8, var_aseff_dn9, var_aseff_dn10, var_aseff_dn11,)
    }
};
        var_aseff = assign15150_e21129;
        var_aseff_dn3 = assign15150_e21129_d_n3;
        var_aseff_dn4 = assign15150_e21129_d_n4;
        var_aseff_dn5 = assign15150_e21129_d_n5;
        var_aseff_dn6 = assign15150_e21129_d_n6;
        var_aseff_dn7 = assign15150_e21129_d_n7;
        var_aseff_dn8 = assign15150_e21129_d_n8;
        var_aseff_dn9 = assign15150_e21129_d_n9;
        var_aseff_dn10 = assign15150_e21129_d_n10;
        var_aseff_dn11 = assign15150_e21129_d_n11;
        var_aseff_rv = 0.0;

        let (assign15160_e21134, assign15160_e21134_d_n3, assign15160_e21134_d_n4, assign15160_e21134_d_n5, assign15160_e21134_d_n6, assign15160_e21134_d_n7, assign15160_e21134_d_n8, assign15160_e21134_d_n9, assign15160_e21134_d_n10, assign15160_e21134_d_n11,) = {
    if (var_guard470 == 0.0) {
        (var_temp_aseff, var_temp_aseff_dn3, var_temp_aseff_dn4, var_temp_aseff_dn5, var_temp_aseff_dn6, var_temp_aseff_dn7, var_temp_aseff_dn8, var_temp_aseff_dn9, var_temp_aseff_dn10, var_temp_aseff_dn11,)
    } else {
        (var_aseff, var_aseff_dn3, var_aseff_dn4, var_aseff_dn5, var_aseff_dn6, var_aseff_dn7, var_aseff_dn8, var_aseff_dn9, var_aseff_dn10, var_aseff_dn11,)
    }
};
        var_aseff = assign15160_e21134;
        var_aseff_dn3 = assign15160_e21134_d_n3;
        var_aseff_dn4 = assign15160_e21134_d_n4;
        var_aseff_dn5 = assign15160_e21134_d_n5;
        var_aseff_dn6 = assign15160_e21134_d_n6;
        var_aseff_dn7 = assign15160_e21134_d_n7;
        var_aseff_dn8 = assign15160_e21134_d_n8;
        var_aseff_dn9 = assign15160_e21134_d_n9;
        var_aseff_dn10 = assign15160_e21134_d_n10;
        var_aseff_dn11 = assign15160_e21134_d_n11;
        var_aseff_rv = 0.0;

        let assign15170_e21137: f64 = if var_aseff < 0.0 { 1.0 } else { 0.0 };
        var_guard471 = assign15170_e21137;
        var_guard471_rv = 0.0;

        let (assign15180_e21141, assign15180_e21141_d_n3, assign15180_e21141_d_n4, assign15180_e21141_d_n5, assign15180_e21141_d_n6, assign15180_e21141_d_n7, assign15180_e21141_d_n8, assign15180_e21141_d_n9, assign15180_e21141_d_n10, assign15180_e21141_d_n11,) = {
    if (var_guard471 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_aseff, var_aseff_dn3, var_aseff_dn4, var_aseff_dn5, var_aseff_dn6, var_aseff_dn7, var_aseff_dn8, var_aseff_dn9, var_aseff_dn10, var_aseff_dn11,)
    }
};
        var_aseff = assign15180_e21141;
        var_aseff_dn3 = assign15180_e21141_d_n3;
        var_aseff_dn4 = assign15180_e21141_d_n4;
        var_aseff_dn5 = assign15180_e21141_d_n5;
        var_aseff_dn6 = assign15180_e21141_d_n6;
        var_aseff_dn7 = assign15180_e21141_d_n7;
        var_aseff_dn8 = assign15180_e21141_d_n8;
        var_aseff_dn9 = assign15180_e21141_d_n9;
        var_aseff_dn10 = assign15180_e21141_d_n10;
        var_aseff_dn11 = assign15180_e21141_d_n11;
        var_aseff_rv = 0.0;

        let assign15190_e21143: f64 = if param_given[18] { 1.0 } else { 0.0 };
        var_guard472 = assign15190_e21143;
        var_guard472_rv = 0.0;

        let (assign15200_e21151, assign15200_e21151_d_n3, assign15200_e21151_d_n4, assign15200_e21151_d_n5, assign15200_e21151_d_n6, assign15200_e21151_d_n7, assign15200_e21151_d_n8, assign15200_e21151_d_n9, assign15200_e21151_d_n10, assign15200_e21151_d_n11,) = {
    if (var_guard472 != 0.0) {
        let assign15200_e21147: f64 = (p.p18 * p.p50);
        let assign15200_e21149: f64 = (assign15200_e21147 * p.p49);
        (assign15200_e21149, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_adeff, var_adeff_dn3, var_adeff_dn4, var_adeff_dn5, var_adeff_dn6, var_adeff_dn7, var_adeff_dn8, var_adeff_dn9, var_adeff_dn10, var_adeff_dn11,)
    }
};
        var_adeff = assign15200_e21151;
        var_adeff_dn3 = assign15200_e21151_d_n3;
        var_adeff_dn4 = assign15200_e21151_d_n4;
        var_adeff_dn5 = assign15200_e21151_d_n5;
        var_adeff_dn6 = assign15200_e21151_d_n6;
        var_adeff_dn7 = assign15200_e21151_d_n7;
        var_adeff_dn8 = assign15200_e21151_d_n8;
        var_adeff_dn9 = assign15200_e21151_d_n9;
        var_adeff_dn10 = assign15200_e21151_d_n10;
        var_adeff_dn11 = assign15200_e21151_d_n11;
        var_adeff_rv = 0.0;

        let (assign15210_e21156, assign15210_e21156_d_n3, assign15210_e21156_d_n4, assign15210_e21156_d_n5, assign15210_e21156_d_n6, assign15210_e21156_d_n7, assign15210_e21156_d_n8, assign15210_e21156_d_n9, assign15210_e21156_d_n10, assign15210_e21156_d_n11,) = {
    if (var_guard472 == 0.0) {
        (var_temp_adeff, var_temp_adeff_dn3, var_temp_adeff_dn4, var_temp_adeff_dn5, var_temp_adeff_dn6, var_temp_adeff_dn7, var_temp_adeff_dn8, var_temp_adeff_dn9, var_temp_adeff_dn10, var_temp_adeff_dn11,)
    } else {
        (var_adeff, var_adeff_dn3, var_adeff_dn4, var_adeff_dn5, var_adeff_dn6, var_adeff_dn7, var_adeff_dn8, var_adeff_dn9, var_adeff_dn10, var_adeff_dn11,)
    }
};
        var_adeff = assign15210_e21156;
        var_adeff_dn3 = assign15210_e21156_d_n3;
        var_adeff_dn4 = assign15210_e21156_d_n4;
        var_adeff_dn5 = assign15210_e21156_d_n5;
        var_adeff_dn6 = assign15210_e21156_d_n6;
        var_adeff_dn7 = assign15210_e21156_d_n7;
        var_adeff_dn8 = assign15210_e21156_d_n8;
        var_adeff_dn9 = assign15210_e21156_d_n9;
        var_adeff_dn10 = assign15210_e21156_d_n10;
        var_adeff_dn11 = assign15210_e21156_d_n11;
        var_adeff_rv = 0.0;

        let assign15220_e21159: f64 = if var_adeff < 0.0 { 1.0 } else { 0.0 };
        var_guard473 = assign15220_e21159;
        var_guard473_rv = 0.0;

        let (assign15230_e21163, assign15230_e21163_d_n3, assign15230_e21163_d_n4, assign15230_e21163_d_n5, assign15230_e21163_d_n6, assign15230_e21163_d_n7, assign15230_e21163_d_n8, assign15230_e21163_d_n9, assign15230_e21163_d_n10, assign15230_e21163_d_n11,) = {
    if (var_guard473 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_adeff, var_adeff_dn3, var_adeff_dn4, var_adeff_dn5, var_adeff_dn6, var_adeff_dn7, var_adeff_dn8, var_adeff_dn9, var_adeff_dn10, var_adeff_dn11,)
    }
};
        var_adeff = assign15230_e21163;
        var_adeff_dn3 = assign15230_e21163_d_n3;
        var_adeff_dn4 = assign15230_e21163_d_n4;
        var_adeff_dn5 = assign15230_e21163_d_n5;
        var_adeff_dn6 = assign15230_e21163_d_n6;
        var_adeff_dn7 = assign15230_e21163_d_n7;
        var_adeff_dn8 = assign15230_e21163_d_n8;
        var_adeff_dn9 = assign15230_e21163_d_n9;
        var_adeff_dn10 = assign15230_e21163_d_n10;
        var_adeff_dn11 = assign15230_e21163_d_n11;
        var_adeff_rv = 0.0;

        let assign15240_e21165: f64 = if param_given[19] { 1.0 } else { 0.0 };
        var_guard474 = assign15240_e21165;
        var_guard474_rv = 0.0;

        let assign15250_e21168: f64 = if p.p926 == 0.0 { 1.0 } else { 0.0 };
        var_guard475 = assign15250_e21168;
        var_guard475_rv = 0.0;

        let (assign15260_e21176, assign15260_e21176_d_n3, assign15260_e21176_d_n4, assign15260_e21176_d_n5, assign15260_e21176_d_n6, assign15260_e21176_d_n7, assign15260_e21176_d_n8, assign15260_e21176_d_n9, assign15260_e21176_d_n10, assign15260_e21176_d_n11,) = {
    if ((var_guard474 != 0.0) && (var_guard475 != 0.0)) {
        let assign15260_e21174: f64 = (p.p19 * p.p50);
        (assign15260_e21174, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_pseff, var_pseff_dn3, var_pseff_dn4, var_pseff_dn5, var_pseff_dn6, var_pseff_dn7, var_pseff_dn8, var_pseff_dn9, var_pseff_dn10, var_pseff_dn11,)
    }
};
        var_pseff = assign15260_e21176;
        var_pseff_dn3 = assign15260_e21176_d_n3;
        var_pseff_dn4 = assign15260_e21176_d_n4;
        var_pseff_dn5 = assign15260_e21176_d_n5;
        var_pseff_dn6 = assign15260_e21176_d_n6;
        var_pseff_dn7 = assign15260_e21176_d_n7;
        var_pseff_dn8 = assign15260_e21176_d_n8;
        var_pseff_dn9 = assign15260_e21176_d_n9;
        var_pseff_dn10 = assign15260_e21176_d_n10;
        var_pseff_dn11 = assign15260_e21176_d_n11;
        var_pseff_rv = 0.0;

        let (assign15270_e21191, assign15270_e21191_d_n3, assign15270_e21191_d_n4, assign15270_e21191_d_n5, assign15270_e21191_d_n6, assign15270_e21191_d_n7, assign15270_e21191_d_n8, assign15270_e21191_d_n9, assign15270_e21191_d_n10, assign15270_e21191_d_n11,) = {
    if ((var_guard474 != 0.0) && (var_guard475 == 0.0)) {
        let assign15270_e21183: f64 = (p.p19 * p.p50);
        let assign15270_e21186: f64 = (var_weffcj * p.p2);
        let assign15270_e21187: f64 = (assign15270_e21183 - assign15270_e21186);
        let assign15270_e21189: f64 = (assign15270_e21187).max(0.0);
        (assign15270_e21189, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_pseff, var_pseff_dn3, var_pseff_dn4, var_pseff_dn5, var_pseff_dn6, var_pseff_dn7, var_pseff_dn8, var_pseff_dn9, var_pseff_dn10, var_pseff_dn11,)
    }
};
        var_pseff = assign15270_e21191;
        var_pseff_dn3 = assign15270_e21191_d_n3;
        var_pseff_dn4 = assign15270_e21191_d_n4;
        var_pseff_dn5 = assign15270_e21191_d_n5;
        var_pseff_dn6 = assign15270_e21191_d_n6;
        var_pseff_dn7 = assign15270_e21191_d_n7;
        var_pseff_dn8 = assign15270_e21191_d_n8;
        var_pseff_dn9 = assign15270_e21191_d_n9;
        var_pseff_dn10 = assign15270_e21191_d_n10;
        var_pseff_dn11 = assign15270_e21191_d_n11;
        var_pseff_rv = 0.0;

        let (assign15280_e21196, assign15280_e21196_d_n3, assign15280_e21196_d_n4, assign15280_e21196_d_n5, assign15280_e21196_d_n6, assign15280_e21196_d_n7, assign15280_e21196_d_n8, assign15280_e21196_d_n9, assign15280_e21196_d_n10, assign15280_e21196_d_n11,) = {
    if (var_guard474 == 0.0) {
        (var_temp_pseff, var_temp_pseff_dn3, var_temp_pseff_dn4, var_temp_pseff_dn5, var_temp_pseff_dn6, var_temp_pseff_dn7, var_temp_pseff_dn8, var_temp_pseff_dn9, var_temp_pseff_dn10, var_temp_pseff_dn11,)
    } else {
        (var_pseff, var_pseff_dn3, var_pseff_dn4, var_pseff_dn5, var_pseff_dn6, var_pseff_dn7, var_pseff_dn8, var_pseff_dn9, var_pseff_dn10, var_pseff_dn11,)
    }
};
        var_pseff = assign15280_e21196;
        var_pseff_dn3 = assign15280_e21196_d_n3;
        var_pseff_dn4 = assign15280_e21196_d_n4;
        var_pseff_dn5 = assign15280_e21196_d_n5;
        var_pseff_dn6 = assign15280_e21196_d_n6;
        var_pseff_dn7 = assign15280_e21196_d_n7;
        var_pseff_dn8 = assign15280_e21196_d_n8;
        var_pseff_dn9 = assign15280_e21196_d_n9;
        var_pseff_dn10 = assign15280_e21196_d_n10;
        var_pseff_dn11 = assign15280_e21196_d_n11;
        var_pseff_rv = 0.0;

        let assign15290_e21199: f64 = if var_pseff < 0.0 { 1.0 } else { 0.0 };
        var_guard476 = assign15290_e21199;
        var_guard476_rv = 0.0;

        let (assign15300_e21206, assign15300_e21206_d_n3, assign15300_e21206_d_n4, assign15300_e21206_d_n5, assign15300_e21206_d_n6, assign15300_e21206_d_n7, assign15300_e21206_d_n8, assign15300_e21206_d_n9, assign15300_e21206_d_n10, assign15300_e21206_d_n11,) = {
    if ((var_guard474 == 0.0) && (var_guard476 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_pseff, var_pseff_dn3, var_pseff_dn4, var_pseff_dn5, var_pseff_dn6, var_pseff_dn7, var_pseff_dn8, var_pseff_dn9, var_pseff_dn10, var_pseff_dn11,)
    }
};
        var_pseff = assign15300_e21206;
        var_pseff_dn3 = assign15300_e21206_d_n3;
        var_pseff_dn4 = assign15300_e21206_d_n4;
        var_pseff_dn5 = assign15300_e21206_d_n5;
        var_pseff_dn6 = assign15300_e21206_d_n6;
        var_pseff_dn7 = assign15300_e21206_d_n7;
        var_pseff_dn8 = assign15300_e21206_d_n8;
        var_pseff_dn9 = assign15300_e21206_d_n9;
        var_pseff_dn10 = assign15300_e21206_d_n10;
        var_pseff_dn11 = assign15300_e21206_d_n11;
        var_pseff_rv = 0.0;

        let assign15310_e21208: f64 = if param_given[20] { 1.0 } else { 0.0 };
        var_guard477 = assign15310_e21208;
        var_guard477_rv = 0.0;

        let assign15320_e21211: f64 = if p.p926 == 0.0 { 1.0 } else { 0.0 };
        var_guard478 = assign15320_e21211;
        var_guard478_rv = 0.0;

        let (assign15330_e21219, assign15330_e21219_d_n3, assign15330_e21219_d_n4, assign15330_e21219_d_n5, assign15330_e21219_d_n6, assign15330_e21219_d_n7, assign15330_e21219_d_n8, assign15330_e21219_d_n9, assign15330_e21219_d_n10, assign15330_e21219_d_n11,) = {
    if ((var_guard477 != 0.0) && (var_guard478 != 0.0)) {
        let assign15330_e21217: f64 = (p.p20 * p.p50);
        (assign15330_e21217, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_pdeff, var_pdeff_dn3, var_pdeff_dn4, var_pdeff_dn5, var_pdeff_dn6, var_pdeff_dn7, var_pdeff_dn8, var_pdeff_dn9, var_pdeff_dn10, var_pdeff_dn11,)
    }
};
        var_pdeff = assign15330_e21219;
        var_pdeff_dn3 = assign15330_e21219_d_n3;
        var_pdeff_dn4 = assign15330_e21219_d_n4;
        var_pdeff_dn5 = assign15330_e21219_d_n5;
        var_pdeff_dn6 = assign15330_e21219_d_n6;
        var_pdeff_dn7 = assign15330_e21219_d_n7;
        var_pdeff_dn8 = assign15330_e21219_d_n8;
        var_pdeff_dn9 = assign15330_e21219_d_n9;
        var_pdeff_dn10 = assign15330_e21219_d_n10;
        var_pdeff_dn11 = assign15330_e21219_d_n11;
        var_pdeff_rv = 0.0;

        let (assign15340_e21234, assign15340_e21234_d_n3, assign15340_e21234_d_n4, assign15340_e21234_d_n5, assign15340_e21234_d_n6, assign15340_e21234_d_n7, assign15340_e21234_d_n8, assign15340_e21234_d_n9, assign15340_e21234_d_n10, assign15340_e21234_d_n11,) = {
    if ((var_guard477 != 0.0) && (var_guard478 == 0.0)) {
        let assign15340_e21226: f64 = (p.p20 * p.p50);
        let assign15340_e21229: f64 = (var_weffcj * p.p2);
        let assign15340_e21230: f64 = (assign15340_e21226 - assign15340_e21229);
        let assign15340_e21232: f64 = (assign15340_e21230).max(0.0);
        (assign15340_e21232, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_pdeff, var_pdeff_dn3, var_pdeff_dn4, var_pdeff_dn5, var_pdeff_dn6, var_pdeff_dn7, var_pdeff_dn8, var_pdeff_dn9, var_pdeff_dn10, var_pdeff_dn11,)
    }
};
        var_pdeff = assign15340_e21234;
        var_pdeff_dn3 = assign15340_e21234_d_n3;
        var_pdeff_dn4 = assign15340_e21234_d_n4;
        var_pdeff_dn5 = assign15340_e21234_d_n5;
        var_pdeff_dn6 = assign15340_e21234_d_n6;
        var_pdeff_dn7 = assign15340_e21234_d_n7;
        var_pdeff_dn8 = assign15340_e21234_d_n8;
        var_pdeff_dn9 = assign15340_e21234_d_n9;
        var_pdeff_dn10 = assign15340_e21234_d_n10;
        var_pdeff_dn11 = assign15340_e21234_d_n11;
        var_pdeff_rv = 0.0;

        let (assign15350_e21239, assign15350_e21239_d_n3, assign15350_e21239_d_n4, assign15350_e21239_d_n5, assign15350_e21239_d_n6, assign15350_e21239_d_n7, assign15350_e21239_d_n8, assign15350_e21239_d_n9, assign15350_e21239_d_n10, assign15350_e21239_d_n11,) = {
    if (var_guard477 == 0.0) {
        (var_temp_pdeff, var_temp_pdeff_dn3, var_temp_pdeff_dn4, var_temp_pdeff_dn5, var_temp_pdeff_dn6, var_temp_pdeff_dn7, var_temp_pdeff_dn8, var_temp_pdeff_dn9, var_temp_pdeff_dn10, var_temp_pdeff_dn11,)
    } else {
        (var_pdeff, var_pdeff_dn3, var_pdeff_dn4, var_pdeff_dn5, var_pdeff_dn6, var_pdeff_dn7, var_pdeff_dn8, var_pdeff_dn9, var_pdeff_dn10, var_pdeff_dn11,)
    }
};
        var_pdeff = assign15350_e21239;
        var_pdeff_dn3 = assign15350_e21239_d_n3;
        var_pdeff_dn4 = assign15350_e21239_d_n4;
        var_pdeff_dn5 = assign15350_e21239_d_n5;
        var_pdeff_dn6 = assign15350_e21239_d_n6;
        var_pdeff_dn7 = assign15350_e21239_d_n7;
        var_pdeff_dn8 = assign15350_e21239_d_n8;
        var_pdeff_dn9 = assign15350_e21239_d_n9;
        var_pdeff_dn10 = assign15350_e21239_d_n10;
        var_pdeff_dn11 = assign15350_e21239_d_n11;
        var_pdeff_rv = 0.0;

        let assign15360_e21242: f64 = if var_pdeff < 0.0 { 1.0 } else { 0.0 };
        var_guard479 = assign15360_e21242;
        var_guard479_rv = 0.0;

        let (assign15370_e21249, assign15370_e21249_d_n3, assign15370_e21249_d_n4, assign15370_e21249_d_n5, assign15370_e21249_d_n6, assign15370_e21249_d_n7, assign15370_e21249_d_n8, assign15370_e21249_d_n9, assign15370_e21249_d_n10, assign15370_e21249_d_n11,) = {
    if ((var_guard477 == 0.0) && (var_guard479 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_pdeff, var_pdeff_dn3, var_pdeff_dn4, var_pdeff_dn5, var_pdeff_dn6, var_pdeff_dn7, var_pdeff_dn8, var_pdeff_dn9, var_pdeff_dn10, var_pdeff_dn11,)
    }
};
        var_pdeff = assign15370_e21249;
        var_pdeff_dn3 = assign15370_e21249_d_n3;
        var_pdeff_dn4 = assign15370_e21249_d_n4;
        var_pdeff_dn5 = assign15370_e21249_d_n5;
        var_pdeff_dn6 = assign15370_e21249_d_n6;
        var_pdeff_dn7 = assign15370_e21249_d_n7;
        var_pdeff_dn8 = assign15370_e21249_d_n8;
        var_pdeff_dn9 = assign15370_e21249_d_n9;
        var_pdeff_dn10 = assign15370_e21249_d_n10;
        var_pdeff_dn11 = assign15370_e21249_d_n11;
        var_pdeff_rv = 0.0;

        let assign15380_e21268: f64 = if (((p.p10 > 0.0) && (p.p11 > 0.0)) && ((p.p2 == 1.0) || ((p.p2 > 1.0) && (p.p12 > 0.0)))) { 1.0 } else { 0.0 };
        var_guard480 = assign15380_e21268;
        var_guard480_rv = 0.0;

        let (assign15390_e21274, assign15390_e21274_d_n3, assign15390_e21274_d_n4, assign15390_e21274_d_n5, assign15390_e21274_d_n6, assign15390_e21274_d_n7, assign15390_e21274_d_n8, assign15390_e21274_d_n9, assign15390_e21274_d_n10, assign15390_e21274_d_n11,) = {
    if (var_guard480 != 0.0) {
        let assign15390_e21272: f64 = (var_lnew).powf(p.p1111);
        (assign15390_e21272, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t0, var_t0_dn3, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8, var_t0_dn9, var_t0_dn10, var_t0_dn11,)
    }
};
        var_t0 = assign15390_e21274;
        var_t0_dn3 = assign15390_e21274_d_n3;
        var_t0_dn4 = assign15390_e21274_d_n4;
        var_t0_dn5 = assign15390_e21274_d_n5;
        var_t0_dn6 = assign15390_e21274_d_n6;
        var_t0_dn7 = assign15390_e21274_d_n7;
        var_t0_dn8 = assign15390_e21274_d_n8;
        var_t0_dn9 = assign15390_e21274_d_n9;
        var_t0_dn10 = assign15390_e21274_d_n10;
        var_t0_dn11 = assign15390_e21274_d_n11;
        var_t0_rv = 0.0;

        let (assign15400_e21280,) = {
    if (var_guard480 != 0.0) {
        let assign15400_e21278: f64 = (var_wnew + p.p1104);
        (assign15400_e21278,)
    } else {
        (var_w_tmp_stress,)
    }
};
        var_w_tmp_stress = assign15400_e21280;
        var_w_tmp_stress_rv = 0.0;

        *var_adeff_slot = var_adeff;
        *var_adeff_dn10_slot = var_adeff_dn10;
        *var_adeff_dn11_slot = var_adeff_dn11;
        *var_adeff_dn3_slot = var_adeff_dn3;
        *var_adeff_dn4_slot = var_adeff_dn4;
        *var_adeff_dn5_slot = var_adeff_dn5;
        *var_adeff_dn6_slot = var_adeff_dn6;
        *var_adeff_dn7_slot = var_adeff_dn7;
        *var_adeff_dn8_slot = var_adeff_dn8;
        *var_adeff_dn9_slot = var_adeff_dn9;
        *var_adeff_rv_slot = var_adeff_rv;
        *var_aseff_slot = var_aseff;
        *var_aseff_dn10_slot = var_aseff_dn10;
        *var_aseff_dn11_slot = var_aseff_dn11;
        *var_aseff_dn3_slot = var_aseff_dn3;
        *var_aseff_dn4_slot = var_aseff_dn4;
        *var_aseff_dn5_slot = var_aseff_dn5;
        *var_aseff_dn6_slot = var_aseff_dn6;
        *var_aseff_dn7_slot = var_aseff_dn7;
        *var_aseff_dn8_slot = var_aseff_dn8;
        *var_aseff_dn9_slot = var_aseff_dn9;
        *var_aseff_rv_slot = var_aseff_rv;
        *var_guard470_slot = var_guard470;
        *var_guard470_rv_slot = var_guard470_rv;
        *var_guard471_slot = var_guard471;
        *var_guard471_rv_slot = var_guard471_rv;
        *var_guard472_slot = var_guard472;
        *var_guard472_rv_slot = var_guard472_rv;
        *var_guard473_slot = var_guard473;
        *var_guard473_rv_slot = var_guard473_rv;
        *var_guard474_slot = var_guard474;
        *var_guard474_rv_slot = var_guard474_rv;
        *var_guard475_slot = var_guard475;
        *var_guard475_rv_slot = var_guard475_rv;
        *var_guard476_slot = var_guard476;
        *var_guard476_rv_slot = var_guard476_rv;
        *var_guard477_slot = var_guard477;
        *var_guard477_rv_slot = var_guard477_rv;
        *var_guard478_slot = var_guard478;
        *var_guard478_rv_slot = var_guard478_rv;
        *var_guard479_slot = var_guard479;
        *var_guard479_rv_slot = var_guard479_rv;
        *var_guard480_slot = var_guard480;
        *var_guard480_rv_slot = var_guard480_rv;
        *var_pdeff_slot = var_pdeff;
        *var_pdeff_dn10_slot = var_pdeff_dn10;
        *var_pdeff_dn11_slot = var_pdeff_dn11;
        *var_pdeff_dn3_slot = var_pdeff_dn3;
        *var_pdeff_dn4_slot = var_pdeff_dn4;
        *var_pdeff_dn5_slot = var_pdeff_dn5;
        *var_pdeff_dn6_slot = var_pdeff_dn6;
        *var_pdeff_dn7_slot = var_pdeff_dn7;
        *var_pdeff_dn8_slot = var_pdeff_dn8;
        *var_pdeff_dn9_slot = var_pdeff_dn9;
        *var_pdeff_rv_slot = var_pdeff_rv;
        *var_pseff_slot = var_pseff;
        *var_pseff_dn10_slot = var_pseff_dn10;
        *var_pseff_dn11_slot = var_pseff_dn11;
        *var_pseff_dn3_slot = var_pseff_dn3;
        *var_pseff_dn4_slot = var_pseff_dn4;
        *var_pseff_dn5_slot = var_pseff_dn5;
        *var_pseff_dn6_slot = var_pseff_dn6;
        *var_pseff_dn7_slot = var_pseff_dn7;
        *var_pseff_dn8_slot = var_pseff_dn8;
        *var_pseff_dn9_slot = var_pseff_dn9;
        *var_pseff_rv_slot = var_pseff_rv;
        *var_t0_slot = var_t0;
        *var_t0_dn10_slot = var_t0_dn10;
        *var_t0_dn11_slot = var_t0_dn11;
        *var_t0_dn3_slot = var_t0_dn3;
        *var_t0_dn4_slot = var_t0_dn4;
        *var_t0_dn5_slot = var_t0_dn5;
        *var_t0_dn6_slot = var_t0_dn6;
        *var_t0_dn7_slot = var_t0_dn7;
        *var_t0_dn8_slot = var_t0_dn8;
        *var_t0_dn9_slot = var_t0_dn9;
        *var_t0_rv_slot = var_t0_rv;
        *var_temp_adeff_slot = var_temp_adeff;
        *var_temp_adeff_dn10_slot = var_temp_adeff_dn10;
        *var_temp_adeff_dn11_slot = var_temp_adeff_dn11;
        *var_temp_adeff_dn3_slot = var_temp_adeff_dn3;
        *var_temp_adeff_dn4_slot = var_temp_adeff_dn4;
        *var_temp_adeff_dn5_slot = var_temp_adeff_dn5;
        *var_temp_adeff_dn6_slot = var_temp_adeff_dn6;
        *var_temp_adeff_dn7_slot = var_temp_adeff_dn7;
        *var_temp_adeff_dn8_slot = var_temp_adeff_dn8;
        *var_temp_adeff_dn9_slot = var_temp_adeff_dn9;
        *var_temp_adeff_rv_slot = var_temp_adeff_rv;
        *var_temp_aseff_slot = var_temp_aseff;
        *var_temp_aseff_dn10_slot = var_temp_aseff_dn10;
        *var_temp_aseff_dn11_slot = var_temp_aseff_dn11;
        *var_temp_aseff_dn3_slot = var_temp_aseff_dn3;
        *var_temp_aseff_dn4_slot = var_temp_aseff_dn4;
        *var_temp_aseff_dn5_slot = var_temp_aseff_dn5;
        *var_temp_aseff_dn6_slot = var_temp_aseff_dn6;
        *var_temp_aseff_dn7_slot = var_temp_aseff_dn7;
        *var_temp_aseff_dn8_slot = var_temp_aseff_dn8;
        *var_temp_aseff_dn9_slot = var_temp_aseff_dn9;
        *var_temp_aseff_rv_slot = var_temp_aseff_rv;
        *var_temp_pdeff_slot = var_temp_pdeff;
        *var_temp_pdeff_dn10_slot = var_temp_pdeff_dn10;
        *var_temp_pdeff_dn11_slot = var_temp_pdeff_dn11;
        *var_temp_pdeff_dn3_slot = var_temp_pdeff_dn3;
        *var_temp_pdeff_dn4_slot = var_temp_pdeff_dn4;
        *var_temp_pdeff_dn5_slot = var_temp_pdeff_dn5;
        *var_temp_pdeff_dn6_slot = var_temp_pdeff_dn6;
        *var_temp_pdeff_dn7_slot = var_temp_pdeff_dn7;
        *var_temp_pdeff_dn8_slot = var_temp_pdeff_dn8;
        *var_temp_pdeff_dn9_slot = var_temp_pdeff_dn9;
        *var_temp_pdeff_rv_slot = var_temp_pdeff_rv;
        *var_temp_pseff_slot = var_temp_pseff;
        *var_temp_pseff_dn10_slot = var_temp_pseff_dn10;
        *var_temp_pseff_dn11_slot = var_temp_pseff_dn11;
        *var_temp_pseff_dn3_slot = var_temp_pseff_dn3;
        *var_temp_pseff_dn4_slot = var_temp_pseff_dn4;
        *var_temp_pseff_dn5_slot = var_temp_pseff_dn5;
        *var_temp_pseff_dn6_slot = var_temp_pseff_dn6;
        *var_temp_pseff_dn7_slot = var_temp_pseff_dn7;
        *var_temp_pseff_dn8_slot = var_temp_pseff_dn8;
        *var_temp_pseff_dn9_slot = var_temp_pseff_dn9;
        *var_temp_pseff_rv_slot = var_temp_pseff_rv;
        *var_w_tmp_stress_slot = var_w_tmp_stress;
        *var_w_tmp_stress_rv_slot = var_w_tmp_stress_rv;
    }

    pub(super) fn stamp_reactive_block_28(
        p: &Parameters,
        var_guard480: f64,
        var_l_mult: f64,
        var_lnew: f64,
        var_tratio: f64,
        var_tratio_dn4: f64,
        var_tratio_dn5: f64,
        var_w_tmp_stress: f64,
        var_i_slot: &mut f64,
        var_i_rv_slot: &mut f64,
        var_inv_od_slot: &mut f64,
        var_inv_od_dn10_slot: &mut f64,
        var_inv_od_dn11_slot: &mut f64,
        var_inv_od_dn3_slot: &mut f64,
        var_inv_od_dn4_slot: &mut f64,
        var_inv_od_dn5_slot: &mut f64,
        var_inv_od_dn6_slot: &mut f64,
        var_inv_od_dn7_slot: &mut f64,
        var_inv_od_dn8_slot: &mut f64,
        var_inv_od_dn9_slot: &mut f64,
        var_inv_od_rv_slot: &mut f64,
        var_inv_odref_slot: &mut f64,
        var_inv_odref_rv_slot: &mut f64,
        var_inv_sa_slot: &mut f64,
        var_inv_sa_dn10_slot: &mut f64,
        var_inv_sa_dn11_slot: &mut f64,
        var_inv_sa_dn3_slot: &mut f64,
        var_inv_sa_dn4_slot: &mut f64,
        var_inv_sa_dn5_slot: &mut f64,
        var_inv_sa_dn6_slot: &mut f64,
        var_inv_sa_dn7_slot: &mut f64,
        var_inv_sa_dn8_slot: &mut f64,
        var_inv_sa_dn9_slot: &mut f64,
        var_inv_sa_rv_slot: &mut f64,
        var_inv_saref_slot: &mut f64,
        var_inv_saref_rv_slot: &mut f64,
        var_inv_sb_slot: &mut f64,
        var_inv_sb_dn10_slot: &mut f64,
        var_inv_sb_dn11_slot: &mut f64,
        var_inv_sb_dn3_slot: &mut f64,
        var_inv_sb_dn4_slot: &mut f64,
        var_inv_sb_dn5_slot: &mut f64,
        var_inv_sb_dn6_slot: &mut f64,
        var_inv_sb_dn7_slot: &mut f64,
        var_inv_sb_dn8_slot: &mut f64,
        var_inv_sb_dn9_slot: &mut f64,
        var_inv_sb_rv_slot: &mut f64,
        var_inv_sbref_slot: &mut f64,
        var_inv_sbref_rv_slot: &mut f64,
        var_k2_stress_slot: &mut f64,
        var_k2_stress_dn10_slot: &mut f64,
        var_k2_stress_dn11_slot: &mut f64,
        var_k2_stress_dn3_slot: &mut f64,
        var_k2_stress_dn4_slot: &mut f64,
        var_k2_stress_dn5_slot: &mut f64,
        var_k2_stress_dn6_slot: &mut f64,
        var_k2_stress_dn7_slot: &mut f64,
        var_k2_stress_dn8_slot: &mut f64,
        var_k2_stress_dn9_slot: &mut f64,
        var_k2_stress_rv_slot: &mut f64,
        var_kstress_u0_slot: &mut f64,
        var_kstress_u0_dn10_slot: &mut f64,
        var_kstress_u0_dn11_slot: &mut f64,
        var_kstress_u0_dn3_slot: &mut f64,
        var_kstress_u0_dn4_slot: &mut f64,
        var_kstress_u0_dn5_slot: &mut f64,
        var_kstress_u0_dn6_slot: &mut f64,
        var_kstress_u0_dn7_slot: &mut f64,
        var_kstress_u0_dn8_slot: &mut f64,
        var_kstress_u0_dn9_slot: &mut f64,
        var_kstress_u0_rv_slot: &mut f64,
        var_kstress_vth0_slot: &mut f64,
        var_kstress_vth0_dn10_slot: &mut f64,
        var_kstress_vth0_dn11_slot: &mut f64,
        var_kstress_vth0_dn3_slot: &mut f64,
        var_kstress_vth0_dn4_slot: &mut f64,
        var_kstress_vth0_dn5_slot: &mut f64,
        var_kstress_vth0_dn6_slot: &mut f64,
        var_kstress_vth0_dn7_slot: &mut f64,
        var_kstress_vth0_dn8_slot: &mut f64,
        var_kstress_vth0_dn9_slot: &mut f64,
        var_kstress_vth0_rv_slot: &mut f64,
        var_ku0_temp_slot: &mut f64,
        var_ku0_temp_dn10_slot: &mut f64,
        var_ku0_temp_dn11_slot: &mut f64,
        var_ku0_temp_dn3_slot: &mut f64,
        var_ku0_temp_dn4_slot: &mut f64,
        var_ku0_temp_dn5_slot: &mut f64,
        var_ku0_temp_dn6_slot: &mut f64,
        var_ku0_temp_dn7_slot: &mut f64,
        var_ku0_temp_dn8_slot: &mut f64,
        var_ku0_temp_dn9_slot: &mut f64,
        var_ku0_temp_rv_slot: &mut f64,
        var_mu0_mult_slot: &mut f64,
        var_mu0_mult_dn10_slot: &mut f64,
        var_mu0_mult_dn11_slot: &mut f64,
        var_mu0_mult_dn3_slot: &mut f64,
        var_mu0_mult_dn4_slot: &mut f64,
        var_mu0_mult_dn5_slot: &mut f64,
        var_mu0_mult_dn6_slot: &mut f64,
        var_mu0_mult_dn7_slot: &mut f64,
        var_mu0_mult_dn8_slot: &mut f64,
        var_mu0_mult_dn9_slot: &mut f64,
        var_mu0_mult_rv_slot: &mut f64,
        var_rho_slot: &mut f64,
        var_rho_dn10_slot: &mut f64,
        var_rho_dn11_slot: &mut f64,
        var_rho_dn3_slot: &mut f64,
        var_rho_dn4_slot: &mut f64,
        var_rho_dn5_slot: &mut f64,
        var_rho_dn6_slot: &mut f64,
        var_rho_dn7_slot: &mut f64,
        var_rho_dn8_slot: &mut f64,
        var_rho_dn9_slot: &mut f64,
        var_rho_ref_slot: &mut f64,
        var_rho_ref_dn10_slot: &mut f64,
        var_rho_ref_dn11_slot: &mut f64,
        var_rho_ref_dn3_slot: &mut f64,
        var_rho_ref_dn4_slot: &mut f64,
        var_rho_ref_dn5_slot: &mut f64,
        var_rho_ref_dn6_slot: &mut f64,
        var_rho_ref_dn7_slot: &mut f64,
        var_rho_ref_dn8_slot: &mut f64,
        var_rho_ref_dn9_slot: &mut f64,
        var_rho_ref_rv_slot: &mut f64,
        var_rho_rv_slot: &mut f64,
        var_t0_slot: &mut f64,
        var_t0_dn10_slot: &mut f64,
        var_t0_dn11_slot: &mut f64,
        var_t0_dn3_slot: &mut f64,
        var_t0_dn4_slot: &mut f64,
        var_t0_dn5_slot: &mut f64,
        var_t0_dn6_slot: &mut f64,
        var_t0_dn7_slot: &mut f64,
        var_t0_dn8_slot: &mut f64,
        var_t0_dn9_slot: &mut f64,
        var_t0_rv_slot: &mut f64,
        var_t1_slot: &mut f64,
        var_t1_dn10_slot: &mut f64,
        var_t1_dn11_slot: &mut f64,
        var_t1_dn3_slot: &mut f64,
        var_t1_dn4_slot: &mut f64,
        var_t1_dn5_slot: &mut f64,
        var_t1_dn6_slot: &mut f64,
        var_t1_dn7_slot: &mut f64,
        var_t1_dn8_slot: &mut f64,
        var_t1_dn9_slot: &mut f64,
        var_t1_rv_slot: &mut f64,
        var_tmp1_stress_slot: &mut f64,
        var_tmp1_stress_dn10_slot: &mut f64,
        var_tmp1_stress_dn11_slot: &mut f64,
        var_tmp1_stress_dn3_slot: &mut f64,
        var_tmp1_stress_dn4_slot: &mut f64,
        var_tmp1_stress_dn5_slot: &mut f64,
        var_tmp1_stress_dn6_slot: &mut f64,
        var_tmp1_stress_dn7_slot: &mut f64,
        var_tmp1_stress_dn8_slot: &mut f64,
        var_tmp1_stress_dn9_slot: &mut f64,
        var_tmp1_stress_rv_slot: &mut f64,
        var_tmp1_stress_vth_slot: &mut f64,
        var_tmp1_stress_vth_dn10_slot: &mut f64,
        var_tmp1_stress_vth_dn11_slot: &mut f64,
        var_tmp1_stress_vth_dn3_slot: &mut f64,
        var_tmp1_stress_vth_dn4_slot: &mut f64,
        var_tmp1_stress_vth_dn5_slot: &mut f64,
        var_tmp1_stress_vth_dn6_slot: &mut f64,
        var_tmp1_stress_vth_dn7_slot: &mut f64,
        var_tmp1_stress_vth_dn8_slot: &mut f64,
        var_tmp1_stress_vth_dn9_slot: &mut f64,
        var_tmp1_stress_vth_rv_slot: &mut f64,
        var_vsat_mult_slot: &mut f64,
        var_vsat_mult_dn10_slot: &mut f64,
        var_vsat_mult_dn11_slot: &mut f64,
        var_vsat_mult_dn3_slot: &mut f64,
        var_vsat_mult_dn4_slot: &mut f64,
        var_vsat_mult_dn5_slot: &mut f64,
        var_vsat_mult_dn6_slot: &mut f64,
        var_vsat_mult_dn7_slot: &mut f64,
        var_vsat_mult_dn8_slot: &mut f64,
        var_vsat_mult_dn9_slot: &mut f64,
        var_vsat_mult_rv_slot: &mut f64,
        var_vth0_stress_slot: &mut f64,
        var_vth0_stress_dn10_slot: &mut f64,
        var_vth0_stress_dn11_slot: &mut f64,
        var_vth0_stress_dn3_slot: &mut f64,
        var_vth0_stress_dn4_slot: &mut f64,
        var_vth0_stress_dn5_slot: &mut f64,
        var_vth0_stress_dn6_slot: &mut f64,
        var_vth0_stress_dn7_slot: &mut f64,
        var_vth0_stress_dn8_slot: &mut f64,
        var_vth0_stress_dn9_slot: &mut f64,
        var_vth0_stress_rv_slot: &mut f64,
    ) {
        let mut var_i: f64 = *var_i_slot;
        let mut var_i_rv: f64 = *var_i_rv_slot;
        let mut var_inv_od: f64 = *var_inv_od_slot;
        let mut var_inv_od_dn10: f64 = *var_inv_od_dn10_slot;
        let mut var_inv_od_dn11: f64 = *var_inv_od_dn11_slot;
        let mut var_inv_od_dn3: f64 = *var_inv_od_dn3_slot;
        let mut var_inv_od_dn4: f64 = *var_inv_od_dn4_slot;
        let mut var_inv_od_dn5: f64 = *var_inv_od_dn5_slot;
        let mut var_inv_od_dn6: f64 = *var_inv_od_dn6_slot;
        let mut var_inv_od_dn7: f64 = *var_inv_od_dn7_slot;
        let mut var_inv_od_dn8: f64 = *var_inv_od_dn8_slot;
        let mut var_inv_od_dn9: f64 = *var_inv_od_dn9_slot;
        let mut var_inv_od_rv: f64 = *var_inv_od_rv_slot;
        let mut var_inv_odref: f64 = *var_inv_odref_slot;
        let mut var_inv_odref_rv: f64 = *var_inv_odref_rv_slot;
        let mut var_inv_sa: f64 = *var_inv_sa_slot;
        let mut var_inv_sa_dn10: f64 = *var_inv_sa_dn10_slot;
        let mut var_inv_sa_dn11: f64 = *var_inv_sa_dn11_slot;
        let mut var_inv_sa_dn3: f64 = *var_inv_sa_dn3_slot;
        let mut var_inv_sa_dn4: f64 = *var_inv_sa_dn4_slot;
        let mut var_inv_sa_dn5: f64 = *var_inv_sa_dn5_slot;
        let mut var_inv_sa_dn6: f64 = *var_inv_sa_dn6_slot;
        let mut var_inv_sa_dn7: f64 = *var_inv_sa_dn7_slot;
        let mut var_inv_sa_dn8: f64 = *var_inv_sa_dn8_slot;
        let mut var_inv_sa_dn9: f64 = *var_inv_sa_dn9_slot;
        let mut var_inv_sa_rv: f64 = *var_inv_sa_rv_slot;
        let mut var_inv_saref: f64 = *var_inv_saref_slot;
        let mut var_inv_saref_rv: f64 = *var_inv_saref_rv_slot;
        let mut var_inv_sb: f64 = *var_inv_sb_slot;
        let mut var_inv_sb_dn10: f64 = *var_inv_sb_dn10_slot;
        let mut var_inv_sb_dn11: f64 = *var_inv_sb_dn11_slot;
        let mut var_inv_sb_dn3: f64 = *var_inv_sb_dn3_slot;
        let mut var_inv_sb_dn4: f64 = *var_inv_sb_dn4_slot;
        let mut var_inv_sb_dn5: f64 = *var_inv_sb_dn5_slot;
        let mut var_inv_sb_dn6: f64 = *var_inv_sb_dn6_slot;
        let mut var_inv_sb_dn7: f64 = *var_inv_sb_dn7_slot;
        let mut var_inv_sb_dn8: f64 = *var_inv_sb_dn8_slot;
        let mut var_inv_sb_dn9: f64 = *var_inv_sb_dn9_slot;
        let mut var_inv_sb_rv: f64 = *var_inv_sb_rv_slot;
        let mut var_inv_sbref: f64 = *var_inv_sbref_slot;
        let mut var_inv_sbref_rv: f64 = *var_inv_sbref_rv_slot;
        let mut var_k2_stress: f64 = *var_k2_stress_slot;
        let mut var_k2_stress_dn10: f64 = *var_k2_stress_dn10_slot;
        let mut var_k2_stress_dn11: f64 = *var_k2_stress_dn11_slot;
        let mut var_k2_stress_dn3: f64 = *var_k2_stress_dn3_slot;
        let mut var_k2_stress_dn4: f64 = *var_k2_stress_dn4_slot;
        let mut var_k2_stress_dn5: f64 = *var_k2_stress_dn5_slot;
        let mut var_k2_stress_dn6: f64 = *var_k2_stress_dn6_slot;
        let mut var_k2_stress_dn7: f64 = *var_k2_stress_dn7_slot;
        let mut var_k2_stress_dn8: f64 = *var_k2_stress_dn8_slot;
        let mut var_k2_stress_dn9: f64 = *var_k2_stress_dn9_slot;
        let mut var_k2_stress_rv: f64 = *var_k2_stress_rv_slot;
        let mut var_kstress_u0: f64 = *var_kstress_u0_slot;
        let mut var_kstress_u0_dn10: f64 = *var_kstress_u0_dn10_slot;
        let mut var_kstress_u0_dn11: f64 = *var_kstress_u0_dn11_slot;
        let mut var_kstress_u0_dn3: f64 = *var_kstress_u0_dn3_slot;
        let mut var_kstress_u0_dn4: f64 = *var_kstress_u0_dn4_slot;
        let mut var_kstress_u0_dn5: f64 = *var_kstress_u0_dn5_slot;
        let mut var_kstress_u0_dn6: f64 = *var_kstress_u0_dn6_slot;
        let mut var_kstress_u0_dn7: f64 = *var_kstress_u0_dn7_slot;
        let mut var_kstress_u0_dn8: f64 = *var_kstress_u0_dn8_slot;
        let mut var_kstress_u0_dn9: f64 = *var_kstress_u0_dn9_slot;
        let mut var_kstress_u0_rv: f64 = *var_kstress_u0_rv_slot;
        let mut var_kstress_vth0: f64 = *var_kstress_vth0_slot;
        let mut var_kstress_vth0_dn10: f64 = *var_kstress_vth0_dn10_slot;
        let mut var_kstress_vth0_dn11: f64 = *var_kstress_vth0_dn11_slot;
        let mut var_kstress_vth0_dn3: f64 = *var_kstress_vth0_dn3_slot;
        let mut var_kstress_vth0_dn4: f64 = *var_kstress_vth0_dn4_slot;
        let mut var_kstress_vth0_dn5: f64 = *var_kstress_vth0_dn5_slot;
        let mut var_kstress_vth0_dn6: f64 = *var_kstress_vth0_dn6_slot;
        let mut var_kstress_vth0_dn7: f64 = *var_kstress_vth0_dn7_slot;
        let mut var_kstress_vth0_dn8: f64 = *var_kstress_vth0_dn8_slot;
        let mut var_kstress_vth0_dn9: f64 = *var_kstress_vth0_dn9_slot;
        let mut var_kstress_vth0_rv: f64 = *var_kstress_vth0_rv_slot;
        let mut var_ku0_temp: f64 = *var_ku0_temp_slot;
        let mut var_ku0_temp_dn10: f64 = *var_ku0_temp_dn10_slot;
        let mut var_ku0_temp_dn11: f64 = *var_ku0_temp_dn11_slot;
        let mut var_ku0_temp_dn3: f64 = *var_ku0_temp_dn3_slot;
        let mut var_ku0_temp_dn4: f64 = *var_ku0_temp_dn4_slot;
        let mut var_ku0_temp_dn5: f64 = *var_ku0_temp_dn5_slot;
        let mut var_ku0_temp_dn6: f64 = *var_ku0_temp_dn6_slot;
        let mut var_ku0_temp_dn7: f64 = *var_ku0_temp_dn7_slot;
        let mut var_ku0_temp_dn8: f64 = *var_ku0_temp_dn8_slot;
        let mut var_ku0_temp_dn9: f64 = *var_ku0_temp_dn9_slot;
        let mut var_ku0_temp_rv: f64 = *var_ku0_temp_rv_slot;
        let mut var_mu0_mult: f64 = *var_mu0_mult_slot;
        let mut var_mu0_mult_dn10: f64 = *var_mu0_mult_dn10_slot;
        let mut var_mu0_mult_dn11: f64 = *var_mu0_mult_dn11_slot;
        let mut var_mu0_mult_dn3: f64 = *var_mu0_mult_dn3_slot;
        let mut var_mu0_mult_dn4: f64 = *var_mu0_mult_dn4_slot;
        let mut var_mu0_mult_dn5: f64 = *var_mu0_mult_dn5_slot;
        let mut var_mu0_mult_dn6: f64 = *var_mu0_mult_dn6_slot;
        let mut var_mu0_mult_dn7: f64 = *var_mu0_mult_dn7_slot;
        let mut var_mu0_mult_dn8: f64 = *var_mu0_mult_dn8_slot;
        let mut var_mu0_mult_dn9: f64 = *var_mu0_mult_dn9_slot;
        let mut var_mu0_mult_rv: f64 = *var_mu0_mult_rv_slot;
        let mut var_rho: f64 = *var_rho_slot;
        let mut var_rho_dn10: f64 = *var_rho_dn10_slot;
        let mut var_rho_dn11: f64 = *var_rho_dn11_slot;
        let mut var_rho_dn3: f64 = *var_rho_dn3_slot;
        let mut var_rho_dn4: f64 = *var_rho_dn4_slot;
        let mut var_rho_dn5: f64 = *var_rho_dn5_slot;
        let mut var_rho_dn6: f64 = *var_rho_dn6_slot;
        let mut var_rho_dn7: f64 = *var_rho_dn7_slot;
        let mut var_rho_dn8: f64 = *var_rho_dn8_slot;
        let mut var_rho_dn9: f64 = *var_rho_dn9_slot;
        let mut var_rho_ref: f64 = *var_rho_ref_slot;
        let mut var_rho_ref_dn10: f64 = *var_rho_ref_dn10_slot;
        let mut var_rho_ref_dn11: f64 = *var_rho_ref_dn11_slot;
        let mut var_rho_ref_dn3: f64 = *var_rho_ref_dn3_slot;
        let mut var_rho_ref_dn4: f64 = *var_rho_ref_dn4_slot;
        let mut var_rho_ref_dn5: f64 = *var_rho_ref_dn5_slot;
        let mut var_rho_ref_dn6: f64 = *var_rho_ref_dn6_slot;
        let mut var_rho_ref_dn7: f64 = *var_rho_ref_dn7_slot;
        let mut var_rho_ref_dn8: f64 = *var_rho_ref_dn8_slot;
        let mut var_rho_ref_dn9: f64 = *var_rho_ref_dn9_slot;
        let mut var_rho_ref_rv: f64 = *var_rho_ref_rv_slot;
        let mut var_rho_rv: f64 = *var_rho_rv_slot;
        let mut var_t0: f64 = *var_t0_slot;
        let mut var_t0_dn10: f64 = *var_t0_dn10_slot;
        let mut var_t0_dn11: f64 = *var_t0_dn11_slot;
        let mut var_t0_dn3: f64 = *var_t0_dn3_slot;
        let mut var_t0_dn4: f64 = *var_t0_dn4_slot;
        let mut var_t0_dn5: f64 = *var_t0_dn5_slot;
        let mut var_t0_dn6: f64 = *var_t0_dn6_slot;
        let mut var_t0_dn7: f64 = *var_t0_dn7_slot;
        let mut var_t0_dn8: f64 = *var_t0_dn8_slot;
        let mut var_t0_dn9: f64 = *var_t0_dn9_slot;
        let mut var_t0_rv: f64 = *var_t0_rv_slot;
        let mut var_t1: f64 = *var_t1_slot;
        let mut var_t1_dn10: f64 = *var_t1_dn10_slot;
        let mut var_t1_dn11: f64 = *var_t1_dn11_slot;
        let mut var_t1_dn3: f64 = *var_t1_dn3_slot;
        let mut var_t1_dn4: f64 = *var_t1_dn4_slot;
        let mut var_t1_dn5: f64 = *var_t1_dn5_slot;
        let mut var_t1_dn6: f64 = *var_t1_dn6_slot;
        let mut var_t1_dn7: f64 = *var_t1_dn7_slot;
        let mut var_t1_dn8: f64 = *var_t1_dn8_slot;
        let mut var_t1_dn9: f64 = *var_t1_dn9_slot;
        let mut var_t1_rv: f64 = *var_t1_rv_slot;
        let mut var_tmp1_stress: f64 = *var_tmp1_stress_slot;
        let mut var_tmp1_stress_dn10: f64 = *var_tmp1_stress_dn10_slot;
        let mut var_tmp1_stress_dn11: f64 = *var_tmp1_stress_dn11_slot;
        let mut var_tmp1_stress_dn3: f64 = *var_tmp1_stress_dn3_slot;
        let mut var_tmp1_stress_dn4: f64 = *var_tmp1_stress_dn4_slot;
        let mut var_tmp1_stress_dn5: f64 = *var_tmp1_stress_dn5_slot;
        let mut var_tmp1_stress_dn6: f64 = *var_tmp1_stress_dn6_slot;
        let mut var_tmp1_stress_dn7: f64 = *var_tmp1_stress_dn7_slot;
        let mut var_tmp1_stress_dn8: f64 = *var_tmp1_stress_dn8_slot;
        let mut var_tmp1_stress_dn9: f64 = *var_tmp1_stress_dn9_slot;
        let mut var_tmp1_stress_rv: f64 = *var_tmp1_stress_rv_slot;
        let mut var_tmp1_stress_vth: f64 = *var_tmp1_stress_vth_slot;
        let mut var_tmp1_stress_vth_dn10: f64 = *var_tmp1_stress_vth_dn10_slot;
        let mut var_tmp1_stress_vth_dn11: f64 = *var_tmp1_stress_vth_dn11_slot;
        let mut var_tmp1_stress_vth_dn3: f64 = *var_tmp1_stress_vth_dn3_slot;
        let mut var_tmp1_stress_vth_dn4: f64 = *var_tmp1_stress_vth_dn4_slot;
        let mut var_tmp1_stress_vth_dn5: f64 = *var_tmp1_stress_vth_dn5_slot;
        let mut var_tmp1_stress_vth_dn6: f64 = *var_tmp1_stress_vth_dn6_slot;
        let mut var_tmp1_stress_vth_dn7: f64 = *var_tmp1_stress_vth_dn7_slot;
        let mut var_tmp1_stress_vth_dn8: f64 = *var_tmp1_stress_vth_dn8_slot;
        let mut var_tmp1_stress_vth_dn9: f64 = *var_tmp1_stress_vth_dn9_slot;
        let mut var_tmp1_stress_vth_rv: f64 = *var_tmp1_stress_vth_rv_slot;
        let mut var_vsat_mult: f64 = *var_vsat_mult_slot;
        let mut var_vsat_mult_dn10: f64 = *var_vsat_mult_dn10_slot;
        let mut var_vsat_mult_dn11: f64 = *var_vsat_mult_dn11_slot;
        let mut var_vsat_mult_dn3: f64 = *var_vsat_mult_dn3_slot;
        let mut var_vsat_mult_dn4: f64 = *var_vsat_mult_dn4_slot;
        let mut var_vsat_mult_dn5: f64 = *var_vsat_mult_dn5_slot;
        let mut var_vsat_mult_dn6: f64 = *var_vsat_mult_dn6_slot;
        let mut var_vsat_mult_dn7: f64 = *var_vsat_mult_dn7_slot;
        let mut var_vsat_mult_dn8: f64 = *var_vsat_mult_dn8_slot;
        let mut var_vsat_mult_dn9: f64 = *var_vsat_mult_dn9_slot;
        let mut var_vsat_mult_rv: f64 = *var_vsat_mult_rv_slot;
        let mut var_vth0_stress: f64 = *var_vth0_stress_slot;
        let mut var_vth0_stress_dn10: f64 = *var_vth0_stress_dn10_slot;
        let mut var_vth0_stress_dn11: f64 = *var_vth0_stress_dn11_slot;
        let mut var_vth0_stress_dn3: f64 = *var_vth0_stress_dn3_slot;
        let mut var_vth0_stress_dn4: f64 = *var_vth0_stress_dn4_slot;
        let mut var_vth0_stress_dn5: f64 = *var_vth0_stress_dn5_slot;
        let mut var_vth0_stress_dn6: f64 = *var_vth0_stress_dn6_slot;
        let mut var_vth0_stress_dn7: f64 = *var_vth0_stress_dn7_slot;
        let mut var_vth0_stress_dn8: f64 = *var_vth0_stress_dn8_slot;
        let mut var_vth0_stress_dn9: f64 = *var_vth0_stress_dn9_slot;
        let mut var_vth0_stress_rv: f64 = *var_vth0_stress_rv_slot;

        let (assign15410_e21286, assign15410_e21286_d_n3, assign15410_e21286_d_n4, assign15410_e21286_d_n5, assign15410_e21286_d_n6, assign15410_e21286_d_n7, assign15410_e21286_d_n8, assign15410_e21286_d_n9, assign15410_e21286_d_n10, assign15410_e21286_d_n11,) = {
    if (var_guard480 != 0.0) {
        let assign15410_e21284: f64 = (var_w_tmp_stress).powf(p.p1112);
        (assign15410_e21284, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t1, var_t1_dn3, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_dn9, var_t1_dn10, var_t1_dn11,)
    }
};
        var_t1 = assign15410_e21286;
        var_t1_dn3 = assign15410_e21286_d_n3;
        var_t1_dn4 = assign15410_e21286_d_n4;
        var_t1_dn5 = assign15410_e21286_d_n5;
        var_t1_dn6 = assign15410_e21286_d_n6;
        var_t1_dn7 = assign15410_e21286_d_n7;
        var_t1_dn8 = assign15410_e21286_d_n8;
        var_t1_dn9 = assign15410_e21286_d_n9;
        var_t1_dn10 = assign15410_e21286_d_n10;
        var_t1_dn11 = assign15410_e21286_d_n11;
        var_t1_rv = 0.0;

        let (assign15420_e21302, assign15420_e21302_d_n3, assign15420_e21302_d_n4, assign15420_e21302_d_n5, assign15420_e21302_d_n6, assign15420_e21302_d_n7, assign15420_e21302_d_n8, assign15420_e21302_d_n9, assign15420_e21302_d_n10, assign15420_e21302_d_n11,) = {
    if (var_guard480 != 0.0) {
        let assign15420_e21290: f64 = (p.p1108 / var_t0);
        let assign15420_e21293: f64 = (p.p1109 / var_t1);
        let assign15420_e21294: f64 = (assign15420_e21290 + assign15420_e21293);
        let assign15420_e21298: f64 = (var_t0 * var_t1);
        let assign15420_e21299: f64 = (p.p1110 / assign15420_e21298);
        let assign15420_e21300: f64 = (assign15420_e21294 + assign15420_e21299);
        (assign15420_e21300, (((-((p.p1108 * var_t0_dn3) / (var_t0 * var_t0))) + (-((p.p1109 * var_t1_dn3) / (var_t1 * var_t1)))) + (-((p.p1110 * ((var_t0_dn3 * var_t1) + (var_t0 * var_t1_dn3))) / (assign15420_e21298 * assign15420_e21298)))), (((-((p.p1108 * var_t0_dn4) / (var_t0 * var_t0))) + (-((p.p1109 * var_t1_dn4) / (var_t1 * var_t1)))) + (-((p.p1110 * ((var_t0_dn4 * var_t1) + (var_t0 * var_t1_dn4))) / (assign15420_e21298 * assign15420_e21298)))), (((-((p.p1108 * var_t0_dn5) / (var_t0 * var_t0))) + (-((p.p1109 * var_t1_dn5) / (var_t1 * var_t1)))) + (-((p.p1110 * ((var_t0_dn5 * var_t1) + (var_t0 * var_t1_dn5))) / (assign15420_e21298 * assign15420_e21298)))), (((-((p.p1108 * var_t0_dn6) / (var_t0 * var_t0))) + (-((p.p1109 * var_t1_dn6) / (var_t1 * var_t1)))) + (-((p.p1110 * ((var_t0_dn6 * var_t1) + (var_t0 * var_t1_dn6))) / (assign15420_e21298 * assign15420_e21298)))), (((-((p.p1108 * var_t0_dn7) / (var_t0 * var_t0))) + (-((p.p1109 * var_t1_dn7) / (var_t1 * var_t1)))) + (-((p.p1110 * ((var_t0_dn7 * var_t1) + (var_t0 * var_t1_dn7))) / (assign15420_e21298 * assign15420_e21298)))), (((-((p.p1108 * var_t0_dn8) / (var_t0 * var_t0))) + (-((p.p1109 * var_t1_dn8) / (var_t1 * var_t1)))) + (-((p.p1110 * ((var_t0_dn8 * var_t1) + (var_t0 * var_t1_dn8))) / (assign15420_e21298 * assign15420_e21298)))), (((-((p.p1108 * var_t0_dn9) / (var_t0 * var_t0))) + (-((p.p1109 * var_t1_dn9) / (var_t1 * var_t1)))) + (-((p.p1110 * ((var_t0_dn9 * var_t1) + (var_t0 * var_t1_dn9))) / (assign15420_e21298 * assign15420_e21298)))), (((-((p.p1108 * var_t0_dn10) / (var_t0 * var_t0))) + (-((p.p1109 * var_t1_dn10) / (var_t1 * var_t1)))) + (-((p.p1110 * ((var_t0_dn10 * var_t1) + (var_t0 * var_t1_dn10))) / (assign15420_e21298 * assign15420_e21298)))), (((-((p.p1108 * var_t0_dn11) / (var_t0 * var_t0))) + (-((p.p1109 * var_t1_dn11) / (var_t1 * var_t1)))) + (-((p.p1110 * ((var_t0_dn11 * var_t1) + (var_t0 * var_t1_dn11))) / (assign15420_e21298 * assign15420_e21298)))),)
    } else {
        (var_tmp1_stress, var_tmp1_stress_dn3, var_tmp1_stress_dn4, var_tmp1_stress_dn5, var_tmp1_stress_dn6, var_tmp1_stress_dn7, var_tmp1_stress_dn8, var_tmp1_stress_dn9, var_tmp1_stress_dn10, var_tmp1_stress_dn11,)
    }
};
        var_tmp1_stress = assign15420_e21302;
        var_tmp1_stress_dn3 = assign15420_e21302_d_n3;
        var_tmp1_stress_dn4 = assign15420_e21302_d_n4;
        var_tmp1_stress_dn5 = assign15420_e21302_d_n5;
        var_tmp1_stress_dn6 = assign15420_e21302_d_n6;
        var_tmp1_stress_dn7 = assign15420_e21302_d_n7;
        var_tmp1_stress_dn8 = assign15420_e21302_d_n8;
        var_tmp1_stress_dn9 = assign15420_e21302_d_n9;
        var_tmp1_stress_dn10 = assign15420_e21302_d_n10;
        var_tmp1_stress_dn11 = assign15420_e21302_d_n11;
        var_tmp1_stress_rv = 0.0;

        let (assign15430_e21308, assign15430_e21308_d_n3, assign15430_e21308_d_n4, assign15430_e21308_d_n5, assign15430_e21308_d_n6, assign15430_e21308_d_n7, assign15430_e21308_d_n8, assign15430_e21308_d_n9, assign15430_e21308_d_n10, assign15430_e21308_d_n11,) = {
    if (var_guard480 != 0.0) {
        let assign15430_e21306: f64 = (1.0 + var_tmp1_stress);
        (assign15430_e21306, var_tmp1_stress_dn3, var_tmp1_stress_dn4, var_tmp1_stress_dn5, var_tmp1_stress_dn6, var_tmp1_stress_dn7, var_tmp1_stress_dn8, var_tmp1_stress_dn9, var_tmp1_stress_dn10, var_tmp1_stress_dn11,)
    } else {
        (var_kstress_u0, var_kstress_u0_dn3, var_kstress_u0_dn4, var_kstress_u0_dn5, var_kstress_u0_dn6, var_kstress_u0_dn7, var_kstress_u0_dn8, var_kstress_u0_dn9, var_kstress_u0_dn10, var_kstress_u0_dn11,)
    }
};
        var_kstress_u0 = assign15430_e21308;
        var_kstress_u0_dn3 = assign15430_e21308_d_n3;
        var_kstress_u0_dn4 = assign15430_e21308_d_n4;
        var_kstress_u0_dn5 = assign15430_e21308_d_n5;
        var_kstress_u0_dn6 = assign15430_e21308_d_n6;
        var_kstress_u0_dn7 = assign15430_e21308_d_n7;
        var_kstress_u0_dn8 = assign15430_e21308_d_n8;
        var_kstress_u0_dn9 = assign15430_e21308_d_n9;
        var_kstress_u0_dn10 = assign15430_e21308_d_n10;
        var_kstress_u0_dn11 = assign15430_e21308_d_n11;
        var_kstress_u0_rv = 0.0;

        let (assign15440_e21314, assign15440_e21314_d_n3, assign15440_e21314_d_n4, assign15440_e21314_d_n5, assign15440_e21314_d_n6, assign15440_e21314_d_n7, assign15440_e21314_d_n8, assign15440_e21314_d_n9, assign15440_e21314_d_n10, assign15440_e21314_d_n11,) = {
    if (var_guard480 != 0.0) {
        let assign15440_e21312: f64 = (var_lnew).powf(p.p1117);
        (assign15440_e21312, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t0, var_t0_dn3, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8, var_t0_dn9, var_t0_dn10, var_t0_dn11,)
    }
};
        var_t0 = assign15440_e21314;
        var_t0_dn3 = assign15440_e21314_d_n3;
        var_t0_dn4 = assign15440_e21314_d_n4;
        var_t0_dn5 = assign15440_e21314_d_n5;
        var_t0_dn6 = assign15440_e21314_d_n6;
        var_t0_dn7 = assign15440_e21314_d_n7;
        var_t0_dn8 = assign15440_e21314_d_n8;
        var_t0_dn9 = assign15440_e21314_d_n9;
        var_t0_dn10 = assign15440_e21314_d_n10;
        var_t0_dn11 = assign15440_e21314_d_n11;
        var_t0_rv = 0.0;

        let (assign15450_e21320, assign15450_e21320_d_n3, assign15450_e21320_d_n4, assign15450_e21320_d_n5, assign15450_e21320_d_n6, assign15450_e21320_d_n7, assign15450_e21320_d_n8, assign15450_e21320_d_n9, assign15450_e21320_d_n10, assign15450_e21320_d_n11,) = {
    if (var_guard480 != 0.0) {
        let assign15450_e21318: f64 = (var_w_tmp_stress).powf(p.p1118);
        (assign15450_e21318, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t1, var_t1_dn3, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_dn9, var_t1_dn10, var_t1_dn11,)
    }
};
        var_t1 = assign15450_e21320;
        var_t1_dn3 = assign15450_e21320_d_n3;
        var_t1_dn4 = assign15450_e21320_d_n4;
        var_t1_dn5 = assign15450_e21320_d_n5;
        var_t1_dn6 = assign15450_e21320_d_n6;
        var_t1_dn7 = assign15450_e21320_d_n7;
        var_t1_dn8 = assign15450_e21320_d_n8;
        var_t1_dn9 = assign15450_e21320_d_n9;
        var_t1_dn10 = assign15450_e21320_d_n10;
        var_t1_dn11 = assign15450_e21320_d_n11;
        var_t1_rv = 0.0;

        let (assign15460_e21336, assign15460_e21336_d_n3, assign15460_e21336_d_n4, assign15460_e21336_d_n5, assign15460_e21336_d_n6, assign15460_e21336_d_n7, assign15460_e21336_d_n8, assign15460_e21336_d_n9, assign15460_e21336_d_n10, assign15460_e21336_d_n11,) = {
    if (var_guard480 != 0.0) {
        let assign15460_e21324: f64 = (p.p1114 / var_t0);
        let assign15460_e21327: f64 = (p.p1115 / var_t1);
        let assign15460_e21328: f64 = (assign15460_e21324 + assign15460_e21327);
        let assign15460_e21332: f64 = (var_t0 * var_t1);
        let assign15460_e21333: f64 = (p.p1116 / assign15460_e21332);
        let assign15460_e21334: f64 = (assign15460_e21328 + assign15460_e21333);
        (assign15460_e21334, (((-((p.p1114 * var_t0_dn3) / (var_t0 * var_t0))) + (-((p.p1115 * var_t1_dn3) / (var_t1 * var_t1)))) + (-((p.p1116 * ((var_t0_dn3 * var_t1) + (var_t0 * var_t1_dn3))) / (assign15460_e21332 * assign15460_e21332)))), (((-((p.p1114 * var_t0_dn4) / (var_t0 * var_t0))) + (-((p.p1115 * var_t1_dn4) / (var_t1 * var_t1)))) + (-((p.p1116 * ((var_t0_dn4 * var_t1) + (var_t0 * var_t1_dn4))) / (assign15460_e21332 * assign15460_e21332)))), (((-((p.p1114 * var_t0_dn5) / (var_t0 * var_t0))) + (-((p.p1115 * var_t1_dn5) / (var_t1 * var_t1)))) + (-((p.p1116 * ((var_t0_dn5 * var_t1) + (var_t0 * var_t1_dn5))) / (assign15460_e21332 * assign15460_e21332)))), (((-((p.p1114 * var_t0_dn6) / (var_t0 * var_t0))) + (-((p.p1115 * var_t1_dn6) / (var_t1 * var_t1)))) + (-((p.p1116 * ((var_t0_dn6 * var_t1) + (var_t0 * var_t1_dn6))) / (assign15460_e21332 * assign15460_e21332)))), (((-((p.p1114 * var_t0_dn7) / (var_t0 * var_t0))) + (-((p.p1115 * var_t1_dn7) / (var_t1 * var_t1)))) + (-((p.p1116 * ((var_t0_dn7 * var_t1) + (var_t0 * var_t1_dn7))) / (assign15460_e21332 * assign15460_e21332)))), (((-((p.p1114 * var_t0_dn8) / (var_t0 * var_t0))) + (-((p.p1115 * var_t1_dn8) / (var_t1 * var_t1)))) + (-((p.p1116 * ((var_t0_dn8 * var_t1) + (var_t0 * var_t1_dn8))) / (assign15460_e21332 * assign15460_e21332)))), (((-((p.p1114 * var_t0_dn9) / (var_t0 * var_t0))) + (-((p.p1115 * var_t1_dn9) / (var_t1 * var_t1)))) + (-((p.p1116 * ((var_t0_dn9 * var_t1) + (var_t0 * var_t1_dn9))) / (assign15460_e21332 * assign15460_e21332)))), (((-((p.p1114 * var_t0_dn10) / (var_t0 * var_t0))) + (-((p.p1115 * var_t1_dn10) / (var_t1 * var_t1)))) + (-((p.p1116 * ((var_t0_dn10 * var_t1) + (var_t0 * var_t1_dn10))) / (assign15460_e21332 * assign15460_e21332)))), (((-((p.p1114 * var_t0_dn11) / (var_t0 * var_t0))) + (-((p.p1115 * var_t1_dn11) / (var_t1 * var_t1)))) + (-((p.p1116 * ((var_t0_dn11 * var_t1) + (var_t0 * var_t1_dn11))) / (assign15460_e21332 * assign15460_e21332)))),)
    } else {
        (var_tmp1_stress_vth, var_tmp1_stress_vth_dn3, var_tmp1_stress_vth_dn4, var_tmp1_stress_vth_dn5, var_tmp1_stress_vth_dn6, var_tmp1_stress_vth_dn7, var_tmp1_stress_vth_dn8, var_tmp1_stress_vth_dn9, var_tmp1_stress_vth_dn10, var_tmp1_stress_vth_dn11,)
    }
};
        var_tmp1_stress_vth = assign15460_e21336;
        var_tmp1_stress_vth_dn3 = assign15460_e21336_d_n3;
        var_tmp1_stress_vth_dn4 = assign15460_e21336_d_n4;
        var_tmp1_stress_vth_dn5 = assign15460_e21336_d_n5;
        var_tmp1_stress_vth_dn6 = assign15460_e21336_d_n6;
        var_tmp1_stress_vth_dn7 = assign15460_e21336_d_n7;
        var_tmp1_stress_vth_dn8 = assign15460_e21336_d_n8;
        var_tmp1_stress_vth_dn9 = assign15460_e21336_d_n9;
        var_tmp1_stress_vth_dn10 = assign15460_e21336_d_n10;
        var_tmp1_stress_vth_dn11 = assign15460_e21336_d_n11;
        var_tmp1_stress_vth_rv = 0.0;

        let (assign15470_e21342, assign15470_e21342_d_n3, assign15470_e21342_d_n4, assign15470_e21342_d_n5, assign15470_e21342_d_n6, assign15470_e21342_d_n7, assign15470_e21342_d_n8, assign15470_e21342_d_n9, assign15470_e21342_d_n10, assign15470_e21342_d_n11,) = {
    if (var_guard480 != 0.0) {
        let assign15470_e21340: f64 = (1.0 + var_tmp1_stress_vth);
        (assign15470_e21340, var_tmp1_stress_vth_dn3, var_tmp1_stress_vth_dn4, var_tmp1_stress_vth_dn5, var_tmp1_stress_vth_dn6, var_tmp1_stress_vth_dn7, var_tmp1_stress_vth_dn8, var_tmp1_stress_vth_dn9, var_tmp1_stress_vth_dn10, var_tmp1_stress_vth_dn11,)
    } else {
        (var_kstress_vth0, var_kstress_vth0_dn3, var_kstress_vth0_dn4, var_kstress_vth0_dn5, var_kstress_vth0_dn6, var_kstress_vth0_dn7, var_kstress_vth0_dn8, var_kstress_vth0_dn9, var_kstress_vth0_dn10, var_kstress_vth0_dn11,)
    }
};
        var_kstress_vth0 = assign15470_e21342;
        var_kstress_vth0_dn3 = assign15470_e21342_d_n3;
        var_kstress_vth0_dn4 = assign15470_e21342_d_n4;
        var_kstress_vth0_dn5 = assign15470_e21342_d_n5;
        var_kstress_vth0_dn6 = assign15470_e21342_d_n6;
        var_kstress_vth0_dn7 = assign15470_e21342_d_n7;
        var_kstress_vth0_dn8 = assign15470_e21342_d_n8;
        var_kstress_vth0_dn9 = assign15470_e21342_d_n9;
        var_kstress_vth0_dn10 = assign15470_e21342_d_n10;
        var_kstress_vth0_dn11 = assign15470_e21342_d_n11;
        var_kstress_vth0_rv = 0.0;

        let (assign15480_e21348, assign15480_e21348_d_n3, assign15480_e21348_d_n4, assign15480_e21348_d_n5, assign15480_e21348_d_n6, assign15480_e21348_d_n7, assign15480_e21348_d_n8, assign15480_e21348_d_n9, assign15480_e21348_d_n10, assign15480_e21348_d_n11,) = {
    if (var_guard480 != 0.0) {
        let assign15480_e21346: f64 = (var_tratio - 1.0);
        (assign15480_e21346, 0.0, var_tratio_dn4, var_tratio_dn5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t0, var_t0_dn3, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8, var_t0_dn9, var_t0_dn10, var_t0_dn11,)
    }
};
        var_t0 = assign15480_e21348;
        var_t0_dn3 = assign15480_e21348_d_n3;
        var_t0_dn4 = assign15480_e21348_d_n4;
        var_t0_dn5 = assign15480_e21348_d_n5;
        var_t0_dn6 = assign15480_e21348_d_n6;
        var_t0_dn7 = assign15480_e21348_d_n7;
        var_t0_dn8 = assign15480_e21348_d_n8;
        var_t0_dn9 = assign15480_e21348_d_n9;
        var_t0_dn10 = assign15480_e21348_d_n10;
        var_t0_dn11 = assign15480_e21348_d_n11;
        var_t0_rv = 0.0;

        let (assign15490_e21360, assign15490_e21360_d_n3, assign15490_e21360_d_n4, assign15490_e21360_d_n5, assign15490_e21360_d_n6, assign15490_e21360_d_n7, assign15490_e21360_d_n8, assign15490_e21360_d_n9, assign15490_e21360_d_n10, assign15490_e21360_d_n11,) = {
    if (var_guard480 != 0.0) {
        let assign15490_e21354: f64 = (p.p1107 * var_t0);
        let assign15490_e21355: f64 = (1.0 + assign15490_e21354);
        let assign15490_e21356: f64 = (var_kstress_u0 * assign15490_e21355);
        let assign15490_e21358: f64 = (assign15490_e21356 + 1e-9);
        (assign15490_e21358, ((var_kstress_u0_dn3 * assign15490_e21355) + (var_kstress_u0 * (p.p1107 * var_t0_dn3))), ((var_kstress_u0_dn4 * assign15490_e21355) + (var_kstress_u0 * (p.p1107 * var_t0_dn4))), ((var_kstress_u0_dn5 * assign15490_e21355) + (var_kstress_u0 * (p.p1107 * var_t0_dn5))), ((var_kstress_u0_dn6 * assign15490_e21355) + (var_kstress_u0 * (p.p1107 * var_t0_dn6))), ((var_kstress_u0_dn7 * assign15490_e21355) + (var_kstress_u0 * (p.p1107 * var_t0_dn7))), ((var_kstress_u0_dn8 * assign15490_e21355) + (var_kstress_u0 * (p.p1107 * var_t0_dn8))), ((var_kstress_u0_dn9 * assign15490_e21355) + (var_kstress_u0 * (p.p1107 * var_t0_dn9))), ((var_kstress_u0_dn10 * assign15490_e21355) + (var_kstress_u0 * (p.p1107 * var_t0_dn10))), ((var_kstress_u0_dn11 * assign15490_e21355) + (var_kstress_u0 * (p.p1107 * var_t0_dn11))),)
    } else {
        (var_ku0_temp, var_ku0_temp_dn3, var_ku0_temp_dn4, var_ku0_temp_dn5, var_ku0_temp_dn6, var_ku0_temp_dn7, var_ku0_temp_dn8, var_ku0_temp_dn9, var_ku0_temp_dn10, var_ku0_temp_dn11,)
    }
};
        var_ku0_temp = assign15490_e21360;
        var_ku0_temp_dn3 = assign15490_e21360_d_n3;
        var_ku0_temp_dn4 = assign15490_e21360_d_n4;
        var_ku0_temp_dn5 = assign15490_e21360_d_n5;
        var_ku0_temp_dn6 = assign15490_e21360_d_n6;
        var_ku0_temp_dn7 = assign15490_e21360_d_n7;
        var_ku0_temp_dn8 = assign15490_e21360_d_n8;
        var_ku0_temp_dn9 = assign15490_e21360_d_n9;
        var_ku0_temp_dn10 = assign15490_e21360_d_n10;
        var_ku0_temp_dn11 = assign15490_e21360_d_n11;
        var_ku0_temp_rv = 0.0;

        let (assign15500_e21364,) = {
    if (var_guard480 != 0.0) {
        (0.0,)
    } else {
        (var_i,)
    }
};
        var_i = assign15500_e21364;
        var_i_rv = 0.0;

        let mut assign15510_loop_guard: usize = 0;
        while {
            let assign15510_cond_e21369: f64 = if ((var_guard480 != 0.0) && (var_i < p.p2)) { 1.0 } else { 0.0 };
            assign15510_cond_e21369 != 0.0
        } {
            assign15510_loop_guard += 1;
            assert!(assign15510_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign15510_body0_e21387, assign15510_body0_e21387_d_n3, assign15510_body0_e21387_d_n4, assign15510_body0_e21387_d_n5, assign15510_body0_e21387_d_n6, assign15510_body0_e21387_d_n7, assign15510_body0_e21387_d_n8, assign15510_body0_e21387_d_n9, assign15510_body0_e21387_d_n10, assign15510_body0_e21387_d_n11,) = {
    if (var_guard480 != 0.0) {
        let assign15510_body0_e21373: f64 = (1.0 / p.p2);
        let assign15510_body0_e21377: f64 = (0.5 * var_l_mult);
        let assign15510_body0_e21378: f64 = (p.p10 + assign15510_body0_e21377);
        let assign15510_body0_e21382: f64 = (p.p12 + var_l_mult);
        let assign15510_body0_e21383: f64 = (var_i * assign15510_body0_e21382);
        let assign15510_body0_e21384: f64 = (assign15510_body0_e21378 + assign15510_body0_e21383);
        let assign15510_body0_e21385: f64 = (assign15510_body0_e21373 / assign15510_body0_e21384);
        (assign15510_body0_e21385, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t0, var_t0_dn3, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8, var_t0_dn9, var_t0_dn10, var_t0_dn11,)
    }
};
            var_t0 = assign15510_body0_e21387;
            var_t0_dn3 = assign15510_body0_e21387_d_n3;
            var_t0_dn4 = assign15510_body0_e21387_d_n4;
            var_t0_dn5 = assign15510_body0_e21387_d_n5;
            var_t0_dn6 = assign15510_body0_e21387_d_n6;
            var_t0_dn7 = assign15510_body0_e21387_d_n7;
            var_t0_dn8 = assign15510_body0_e21387_d_n8;
            var_t0_dn9 = assign15510_body0_e21387_d_n9;
            var_t0_dn10 = assign15510_body0_e21387_d_n10;
            var_t0_dn11 = assign15510_body0_e21387_d_n11;
            var_t0_rv = 0.0;
            let (assign15510_body1_e21405, assign15510_body1_e21405_d_n3, assign15510_body1_e21405_d_n4, assign15510_body1_e21405_d_n5, assign15510_body1_e21405_d_n6, assign15510_body1_e21405_d_n7, assign15510_body1_e21405_d_n8, assign15510_body1_e21405_d_n9, assign15510_body1_e21405_d_n10, assign15510_body1_e21405_d_n11,) = {
    if (var_guard480 != 0.0) {
        let assign15510_body1_e21391: f64 = (1.0 / p.p2);
        let assign15510_body1_e21395: f64 = (0.5 * var_l_mult);
        let assign15510_body1_e21396: f64 = (p.p11 + assign15510_body1_e21395);
        let assign15510_body1_e21400: f64 = (p.p12 + var_l_mult);
        let assign15510_body1_e21401: f64 = (var_i * assign15510_body1_e21400);
        let assign15510_body1_e21402: f64 = (assign15510_body1_e21396 + assign15510_body1_e21401);
        let assign15510_body1_e21403: f64 = (assign15510_body1_e21391 / assign15510_body1_e21402);
        (assign15510_body1_e21403, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t1, var_t1_dn3, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_dn9, var_t1_dn10, var_t1_dn11,)
    }
};
            var_t1 = assign15510_body1_e21405;
            var_t1_dn3 = assign15510_body1_e21405_d_n3;
            var_t1_dn4 = assign15510_body1_e21405_d_n4;
            var_t1_dn5 = assign15510_body1_e21405_d_n5;
            var_t1_dn6 = assign15510_body1_e21405_d_n6;
            var_t1_dn7 = assign15510_body1_e21405_d_n7;
            var_t1_dn8 = assign15510_body1_e21405_d_n8;
            var_t1_dn9 = assign15510_body1_e21405_d_n9;
            var_t1_dn10 = assign15510_body1_e21405_d_n10;
            var_t1_dn11 = assign15510_body1_e21405_d_n11;
            var_t1_rv = 0.0;
            let (assign15510_body2_e21411, assign15510_body2_e21411_d_n3, assign15510_body2_e21411_d_n4, assign15510_body2_e21411_d_n5, assign15510_body2_e21411_d_n6, assign15510_body2_e21411_d_n7, assign15510_body2_e21411_d_n8, assign15510_body2_e21411_d_n9, assign15510_body2_e21411_d_n10, assign15510_body2_e21411_d_n11,) = {
    if (var_guard480 != 0.0) {
        let assign15510_body2_e21409: f64 = (var_inv_sa + var_t0);
        (assign15510_body2_e21409, (var_inv_sa_dn3 + var_t0_dn3), (var_inv_sa_dn4 + var_t0_dn4), (var_inv_sa_dn5 + var_t0_dn5), (var_inv_sa_dn6 + var_t0_dn6), (var_inv_sa_dn7 + var_t0_dn7), (var_inv_sa_dn8 + var_t0_dn8), (var_inv_sa_dn9 + var_t0_dn9), (var_inv_sa_dn10 + var_t0_dn10), (var_inv_sa_dn11 + var_t0_dn11),)
    } else {
        (var_inv_sa, var_inv_sa_dn3, var_inv_sa_dn4, var_inv_sa_dn5, var_inv_sa_dn6, var_inv_sa_dn7, var_inv_sa_dn8, var_inv_sa_dn9, var_inv_sa_dn10, var_inv_sa_dn11,)
    }
};
            var_inv_sa = assign15510_body2_e21411;
            var_inv_sa_dn3 = assign15510_body2_e21411_d_n3;
            var_inv_sa_dn4 = assign15510_body2_e21411_d_n4;
            var_inv_sa_dn5 = assign15510_body2_e21411_d_n5;
            var_inv_sa_dn6 = assign15510_body2_e21411_d_n6;
            var_inv_sa_dn7 = assign15510_body2_e21411_d_n7;
            var_inv_sa_dn8 = assign15510_body2_e21411_d_n8;
            var_inv_sa_dn9 = assign15510_body2_e21411_d_n9;
            var_inv_sa_dn10 = assign15510_body2_e21411_d_n10;
            var_inv_sa_dn11 = assign15510_body2_e21411_d_n11;
            var_inv_sa_rv = 0.0;
            let (assign15510_body3_e21417, assign15510_body3_e21417_d_n3, assign15510_body3_e21417_d_n4, assign15510_body3_e21417_d_n5, assign15510_body3_e21417_d_n6, assign15510_body3_e21417_d_n7, assign15510_body3_e21417_d_n8, assign15510_body3_e21417_d_n9, assign15510_body3_e21417_d_n10, assign15510_body3_e21417_d_n11,) = {
    if (var_guard480 != 0.0) {
        let assign15510_body3_e21415: f64 = (var_inv_sb + var_t1);
        (assign15510_body3_e21415, (var_inv_sb_dn3 + var_t1_dn3), (var_inv_sb_dn4 + var_t1_dn4), (var_inv_sb_dn5 + var_t1_dn5), (var_inv_sb_dn6 + var_t1_dn6), (var_inv_sb_dn7 + var_t1_dn7), (var_inv_sb_dn8 + var_t1_dn8), (var_inv_sb_dn9 + var_t1_dn9), (var_inv_sb_dn10 + var_t1_dn10), (var_inv_sb_dn11 + var_t1_dn11),)
    } else {
        (var_inv_sb, var_inv_sb_dn3, var_inv_sb_dn4, var_inv_sb_dn5, var_inv_sb_dn6, var_inv_sb_dn7, var_inv_sb_dn8, var_inv_sb_dn9, var_inv_sb_dn10, var_inv_sb_dn11,)
    }
};
            var_inv_sb = assign15510_body3_e21417;
            var_inv_sb_dn3 = assign15510_body3_e21417_d_n3;
            var_inv_sb_dn4 = assign15510_body3_e21417_d_n4;
            var_inv_sb_dn5 = assign15510_body3_e21417_d_n5;
            var_inv_sb_dn6 = assign15510_body3_e21417_d_n6;
            var_inv_sb_dn7 = assign15510_body3_e21417_d_n7;
            var_inv_sb_dn8 = assign15510_body3_e21417_d_n8;
            var_inv_sb_dn9 = assign15510_body3_e21417_d_n9;
            var_inv_sb_dn10 = assign15510_body3_e21417_d_n10;
            var_inv_sb_dn11 = assign15510_body3_e21417_d_n11;
            var_inv_sb_rv = 0.0;
            let (assign15510_body4_e21423,) = {
    if (var_guard480 != 0.0) {
        let assign15510_body4_e21421: f64 = (var_i + 1.0);
        (assign15510_body4_e21421,)
    } else {
        (var_i,)
    }
};
            var_i = assign15510_body4_e21423;
            var_i_rv = 0.0;
        }

        let (assign15520_e21433,) = {
    if (var_guard480 != 0.0) {
        let assign15520_e21429: f64 = (0.5 * var_l_mult);
        let assign15520_e21430: f64 = (p.p1102 + assign15520_e21429);
        let assign15520_e21431: f64 = (1.0 / assign15520_e21430);
        (assign15520_e21431,)
    } else {
        (var_inv_saref,)
    }
};
        var_inv_saref = assign15520_e21433;
        var_inv_saref_rv = 0.0;

        let (assign15530_e21443,) = {
    if (var_guard480 != 0.0) {
        let assign15530_e21439: f64 = (0.5 * var_l_mult);
        let assign15530_e21440: f64 = (p.p1103 + assign15530_e21439);
        let assign15530_e21441: f64 = (1.0 / assign15530_e21440);
        (assign15530_e21441,)
    } else {
        (var_inv_sbref,)
    }
};
        var_inv_sbref = assign15530_e21443;
        var_inv_sbref_rv = 0.0;

        let (assign15540_e21449,) = {
    if (var_guard480 != 0.0) {
        let assign15540_e21447: f64 = (var_inv_saref + var_inv_sbref);
        (assign15540_e21447,)
    } else {
        (var_inv_odref,)
    }
};
        var_inv_odref = assign15540_e21449;
        var_inv_odref_rv = 0.0;

        let (assign15550_e21457, assign15550_e21457_d_n3, assign15550_e21457_d_n4, assign15550_e21457_d_n5, assign15550_e21457_d_n6, assign15550_e21457_d_n7, assign15550_e21457_d_n8, assign15550_e21457_d_n9, assign15550_e21457_d_n10, assign15550_e21457_d_n11,) = {
    if (var_guard480 != 0.0) {
        let assign15550_e21453: f64 = (p.p1105 / var_ku0_temp);
        let assign15550_e21455: f64 = (assign15550_e21453 * var_inv_odref);
        (assign15550_e21455, ((-((p.p1105 * var_ku0_temp_dn3) / (var_ku0_temp * var_ku0_temp))) * var_inv_odref), ((-((p.p1105 * var_ku0_temp_dn4) / (var_ku0_temp * var_ku0_temp))) * var_inv_odref), ((-((p.p1105 * var_ku0_temp_dn5) / (var_ku0_temp * var_ku0_temp))) * var_inv_odref), ((-((p.p1105 * var_ku0_temp_dn6) / (var_ku0_temp * var_ku0_temp))) * var_inv_odref), ((-((p.p1105 * var_ku0_temp_dn7) / (var_ku0_temp * var_ku0_temp))) * var_inv_odref), ((-((p.p1105 * var_ku0_temp_dn8) / (var_ku0_temp * var_ku0_temp))) * var_inv_odref), ((-((p.p1105 * var_ku0_temp_dn9) / (var_ku0_temp * var_ku0_temp))) * var_inv_odref), ((-((p.p1105 * var_ku0_temp_dn10) / (var_ku0_temp * var_ku0_temp))) * var_inv_odref), ((-((p.p1105 * var_ku0_temp_dn11) / (var_ku0_temp * var_ku0_temp))) * var_inv_odref),)
    } else {
        (var_rho_ref, var_rho_ref_dn3, var_rho_ref_dn4, var_rho_ref_dn5, var_rho_ref_dn6, var_rho_ref_dn7, var_rho_ref_dn8, var_rho_ref_dn9, var_rho_ref_dn10, var_rho_ref_dn11,)
    }
};
        var_rho_ref = assign15550_e21457;
        var_rho_ref_dn3 = assign15550_e21457_d_n3;
        var_rho_ref_dn4 = assign15550_e21457_d_n4;
        var_rho_ref_dn5 = assign15550_e21457_d_n5;
        var_rho_ref_dn6 = assign15550_e21457_d_n6;
        var_rho_ref_dn7 = assign15550_e21457_d_n7;
        var_rho_ref_dn8 = assign15550_e21457_d_n8;
        var_rho_ref_dn9 = assign15550_e21457_d_n9;
        var_rho_ref_dn10 = assign15550_e21457_d_n10;
        var_rho_ref_dn11 = assign15550_e21457_d_n11;
        var_rho_ref_rv = 0.0;

        let (assign15560_e21463, assign15560_e21463_d_n3, assign15560_e21463_d_n4, assign15560_e21463_d_n5, assign15560_e21463_d_n6, assign15560_e21463_d_n7, assign15560_e21463_d_n8, assign15560_e21463_d_n9, assign15560_e21463_d_n10, assign15560_e21463_d_n11,) = {
    if (var_guard480 != 0.0) {
        let assign15560_e21461: f64 = (var_inv_sa + var_inv_sb);
        (assign15560_e21461, (var_inv_sa_dn3 + var_inv_sb_dn3), (var_inv_sa_dn4 + var_inv_sb_dn4), (var_inv_sa_dn5 + var_inv_sb_dn5), (var_inv_sa_dn6 + var_inv_sb_dn6), (var_inv_sa_dn7 + var_inv_sb_dn7), (var_inv_sa_dn8 + var_inv_sb_dn8), (var_inv_sa_dn9 + var_inv_sb_dn9), (var_inv_sa_dn10 + var_inv_sb_dn10), (var_inv_sa_dn11 + var_inv_sb_dn11),)
    } else {
        (var_inv_od, var_inv_od_dn3, var_inv_od_dn4, var_inv_od_dn5, var_inv_od_dn6, var_inv_od_dn7, var_inv_od_dn8, var_inv_od_dn9, var_inv_od_dn10, var_inv_od_dn11,)
    }
};
        var_inv_od = assign15560_e21463;
        var_inv_od_dn3 = assign15560_e21463_d_n3;
        var_inv_od_dn4 = assign15560_e21463_d_n4;
        var_inv_od_dn5 = assign15560_e21463_d_n5;
        var_inv_od_dn6 = assign15560_e21463_d_n6;
        var_inv_od_dn7 = assign15560_e21463_d_n7;
        var_inv_od_dn8 = assign15560_e21463_d_n8;
        var_inv_od_dn9 = assign15560_e21463_d_n9;
        var_inv_od_dn10 = assign15560_e21463_d_n10;
        var_inv_od_dn11 = assign15560_e21463_d_n11;
        var_inv_od_rv = 0.0;

        let (assign15570_e21471, assign15570_e21471_d_n3, assign15570_e21471_d_n4, assign15570_e21471_d_n5, assign15570_e21471_d_n6, assign15570_e21471_d_n7, assign15570_e21471_d_n8, assign15570_e21471_d_n9, assign15570_e21471_d_n10, assign15570_e21471_d_n11,) = {
    if (var_guard480 != 0.0) {
        let assign15570_e21467: f64 = (p.p1105 / var_ku0_temp);
        let assign15570_e21469: f64 = (assign15570_e21467 * var_inv_od);
        (assign15570_e21469, (((-((p.p1105 * var_ku0_temp_dn3) / (var_ku0_temp * var_ku0_temp))) * var_inv_od) + (assign15570_e21467 * var_inv_od_dn3)), (((-((p.p1105 * var_ku0_temp_dn4) / (var_ku0_temp * var_ku0_temp))) * var_inv_od) + (assign15570_e21467 * var_inv_od_dn4)), (((-((p.p1105 * var_ku0_temp_dn5) / (var_ku0_temp * var_ku0_temp))) * var_inv_od) + (assign15570_e21467 * var_inv_od_dn5)), (((-((p.p1105 * var_ku0_temp_dn6) / (var_ku0_temp * var_ku0_temp))) * var_inv_od) + (assign15570_e21467 * var_inv_od_dn6)), (((-((p.p1105 * var_ku0_temp_dn7) / (var_ku0_temp * var_ku0_temp))) * var_inv_od) + (assign15570_e21467 * var_inv_od_dn7)), (((-((p.p1105 * var_ku0_temp_dn8) / (var_ku0_temp * var_ku0_temp))) * var_inv_od) + (assign15570_e21467 * var_inv_od_dn8)), (((-((p.p1105 * var_ku0_temp_dn9) / (var_ku0_temp * var_ku0_temp))) * var_inv_od) + (assign15570_e21467 * var_inv_od_dn9)), (((-((p.p1105 * var_ku0_temp_dn10) / (var_ku0_temp * var_ku0_temp))) * var_inv_od) + (assign15570_e21467 * var_inv_od_dn10)), (((-((p.p1105 * var_ku0_temp_dn11) / (var_ku0_temp * var_ku0_temp))) * var_inv_od) + (assign15570_e21467 * var_inv_od_dn11)),)
    } else {
        (var_rho, var_rho_dn3, var_rho_dn4, var_rho_dn5, var_rho_dn6, var_rho_dn7, var_rho_dn8, var_rho_dn9, var_rho_dn10, var_rho_dn11,)
    }
};
        var_rho = assign15570_e21471;
        var_rho_dn3 = assign15570_e21471_d_n3;
        var_rho_dn4 = assign15570_e21471_d_n4;
        var_rho_dn5 = assign15570_e21471_d_n5;
        var_rho_dn6 = assign15570_e21471_d_n6;
        var_rho_dn7 = assign15570_e21471_d_n7;
        var_rho_dn8 = assign15570_e21471_d_n8;
        var_rho_dn9 = assign15570_e21471_d_n9;
        var_rho_dn10 = assign15570_e21471_d_n10;
        var_rho_dn11 = assign15570_e21471_d_n11;
        var_rho_rv = 0.0;

        let (assign15580_e21481, assign15580_e21481_d_n3, assign15580_e21481_d_n4, assign15580_e21481_d_n5, assign15580_e21481_d_n6, assign15580_e21481_d_n7, assign15580_e21481_d_n8, assign15580_e21481_d_n9, assign15580_e21481_d_n10, assign15580_e21481_d_n11,) = {
    if (var_guard480 != 0.0) {
        let assign15580_e21475: f64 = (1.0 + var_rho);
        let assign15580_e21478: f64 = (1.0 + var_rho_ref);
        let assign15580_e21479: f64 = (assign15580_e21475 / assign15580_e21478);
        (assign15580_e21479, (((var_rho_dn3 * assign15580_e21478) - (assign15580_e21475 * var_rho_ref_dn3)) / (assign15580_e21478 * assign15580_e21478)), (((var_rho_dn4 * assign15580_e21478) - (assign15580_e21475 * var_rho_ref_dn4)) / (assign15580_e21478 * assign15580_e21478)), (((var_rho_dn5 * assign15580_e21478) - (assign15580_e21475 * var_rho_ref_dn5)) / (assign15580_e21478 * assign15580_e21478)), (((var_rho_dn6 * assign15580_e21478) - (assign15580_e21475 * var_rho_ref_dn6)) / (assign15580_e21478 * assign15580_e21478)), (((var_rho_dn7 * assign15580_e21478) - (assign15580_e21475 * var_rho_ref_dn7)) / (assign15580_e21478 * assign15580_e21478)), (((var_rho_dn8 * assign15580_e21478) - (assign15580_e21475 * var_rho_ref_dn8)) / (assign15580_e21478 * assign15580_e21478)), (((var_rho_dn9 * assign15580_e21478) - (assign15580_e21475 * var_rho_ref_dn9)) / (assign15580_e21478 * assign15580_e21478)), (((var_rho_dn10 * assign15580_e21478) - (assign15580_e21475 * var_rho_ref_dn10)) / (assign15580_e21478 * assign15580_e21478)), (((var_rho_dn11 * assign15580_e21478) - (assign15580_e21475 * var_rho_ref_dn11)) / (assign15580_e21478 * assign15580_e21478)),)
    } else {
        (var_mu0_mult, var_mu0_mult_dn3, var_mu0_mult_dn4, var_mu0_mult_dn5, var_mu0_mult_dn6, var_mu0_mult_dn7, var_mu0_mult_dn8, var_mu0_mult_dn9, var_mu0_mult_dn10, var_mu0_mult_dn11,)
    }
};
        var_mu0_mult = assign15580_e21481;
        var_mu0_mult_dn3 = assign15580_e21481_d_n3;
        var_mu0_mult_dn4 = assign15580_e21481_d_n4;
        var_mu0_mult_dn5 = assign15580_e21481_d_n5;
        var_mu0_mult_dn6 = assign15580_e21481_d_n6;
        var_mu0_mult_dn7 = assign15580_e21481_d_n7;
        var_mu0_mult_dn8 = assign15580_e21481_d_n8;
        var_mu0_mult_dn9 = assign15580_e21481_d_n9;
        var_mu0_mult_dn10 = assign15580_e21481_d_n10;
        var_mu0_mult_dn11 = assign15580_e21481_d_n11;
        var_mu0_mult_rv = 0.0;

        let (assign15590_e21495, assign15590_e21495_d_n3, assign15590_e21495_d_n4, assign15590_e21495_d_n5, assign15590_e21495_d_n6, assign15590_e21495_d_n7, assign15590_e21495_d_n8, assign15590_e21495_d_n9, assign15590_e21495_d_n10, assign15590_e21495_d_n11,) = {
    if (var_guard480 != 0.0) {
        let assign15590_e21486: f64 = (var_rho * p.p1106);
        let assign15590_e21487: f64 = (1.0 + assign15590_e21486);
        let assign15590_e21491: f64 = (var_rho_ref * p.p1106);
        let assign15590_e21492: f64 = (1.0 + assign15590_e21491);
        let assign15590_e21493: f64 = (assign15590_e21487 / assign15590_e21492);
        (assign15590_e21493, ((((var_rho_dn3 * p.p1106) * assign15590_e21492) - (assign15590_e21487 * (var_rho_ref_dn3 * p.p1106))) / (assign15590_e21492 * assign15590_e21492)), ((((var_rho_dn4 * p.p1106) * assign15590_e21492) - (assign15590_e21487 * (var_rho_ref_dn4 * p.p1106))) / (assign15590_e21492 * assign15590_e21492)), ((((var_rho_dn5 * p.p1106) * assign15590_e21492) - (assign15590_e21487 * (var_rho_ref_dn5 * p.p1106))) / (assign15590_e21492 * assign15590_e21492)), ((((var_rho_dn6 * p.p1106) * assign15590_e21492) - (assign15590_e21487 * (var_rho_ref_dn6 * p.p1106))) / (assign15590_e21492 * assign15590_e21492)), ((((var_rho_dn7 * p.p1106) * assign15590_e21492) - (assign15590_e21487 * (var_rho_ref_dn7 * p.p1106))) / (assign15590_e21492 * assign15590_e21492)), ((((var_rho_dn8 * p.p1106) * assign15590_e21492) - (assign15590_e21487 * (var_rho_ref_dn8 * p.p1106))) / (assign15590_e21492 * assign15590_e21492)), ((((var_rho_dn9 * p.p1106) * assign15590_e21492) - (assign15590_e21487 * (var_rho_ref_dn9 * p.p1106))) / (assign15590_e21492 * assign15590_e21492)), ((((var_rho_dn10 * p.p1106) * assign15590_e21492) - (assign15590_e21487 * (var_rho_ref_dn10 * p.p1106))) / (assign15590_e21492 * assign15590_e21492)), ((((var_rho_dn11 * p.p1106) * assign15590_e21492) - (assign15590_e21487 * (var_rho_ref_dn11 * p.p1106))) / (assign15590_e21492 * assign15590_e21492)),)
    } else {
        (var_vsat_mult, var_vsat_mult_dn3, var_vsat_mult_dn4, var_vsat_mult_dn5, var_vsat_mult_dn6, var_vsat_mult_dn7, var_vsat_mult_dn8, var_vsat_mult_dn9, var_vsat_mult_dn10, var_vsat_mult_dn11,)
    }
};
        var_vsat_mult = assign15590_e21495;
        var_vsat_mult_dn3 = assign15590_e21495_d_n3;
        var_vsat_mult_dn4 = assign15590_e21495_d_n4;
        var_vsat_mult_dn5 = assign15590_e21495_d_n5;
        var_vsat_mult_dn6 = assign15590_e21495_d_n6;
        var_vsat_mult_dn7 = assign15590_e21495_d_n7;
        var_vsat_mult_dn8 = assign15590_e21495_d_n8;
        var_vsat_mult_dn9 = assign15590_e21495_d_n9;
        var_vsat_mult_dn10 = assign15590_e21495_d_n10;
        var_vsat_mult_dn11 = assign15590_e21495_d_n11;
        var_vsat_mult_rv = 0.0;

        let (assign15600_e21505, assign15600_e21505_d_n3, assign15600_e21505_d_n4, assign15600_e21505_d_n5, assign15600_e21505_d_n6, assign15600_e21505_d_n7, assign15600_e21505_d_n8, assign15600_e21505_d_n9, assign15600_e21505_d_n10, assign15600_e21505_d_n11,) = {
    if (var_guard480 != 0.0) {
        let assign15600_e21499: f64 = (p.p1113 / var_kstress_vth0);
        let assign15600_e21502: f64 = (var_inv_od - var_inv_odref);
        let assign15600_e21503: f64 = (assign15600_e21499 * assign15600_e21502);
        (assign15600_e21503, (((-((p.p1113 * var_kstress_vth0_dn3) / (var_kstress_vth0 * var_kstress_vth0))) * assign15600_e21502) + (assign15600_e21499 * var_inv_od_dn3)), (((-((p.p1113 * var_kstress_vth0_dn4) / (var_kstress_vth0 * var_kstress_vth0))) * assign15600_e21502) + (assign15600_e21499 * var_inv_od_dn4)), (((-((p.p1113 * var_kstress_vth0_dn5) / (var_kstress_vth0 * var_kstress_vth0))) * assign15600_e21502) + (assign15600_e21499 * var_inv_od_dn5)), (((-((p.p1113 * var_kstress_vth0_dn6) / (var_kstress_vth0 * var_kstress_vth0))) * assign15600_e21502) + (assign15600_e21499 * var_inv_od_dn6)), (((-((p.p1113 * var_kstress_vth0_dn7) / (var_kstress_vth0 * var_kstress_vth0))) * assign15600_e21502) + (assign15600_e21499 * var_inv_od_dn7)), (((-((p.p1113 * var_kstress_vth0_dn8) / (var_kstress_vth0 * var_kstress_vth0))) * assign15600_e21502) + (assign15600_e21499 * var_inv_od_dn8)), (((-((p.p1113 * var_kstress_vth0_dn9) / (var_kstress_vth0 * var_kstress_vth0))) * assign15600_e21502) + (assign15600_e21499 * var_inv_od_dn9)), (((-((p.p1113 * var_kstress_vth0_dn10) / (var_kstress_vth0 * var_kstress_vth0))) * assign15600_e21502) + (assign15600_e21499 * var_inv_od_dn10)), (((-((p.p1113 * var_kstress_vth0_dn11) / (var_kstress_vth0 * var_kstress_vth0))) * assign15600_e21502) + (assign15600_e21499 * var_inv_od_dn11)),)
    } else {
        (var_vth0_stress, var_vth0_stress_dn3, var_vth0_stress_dn4, var_vth0_stress_dn5, var_vth0_stress_dn6, var_vth0_stress_dn7, var_vth0_stress_dn8, var_vth0_stress_dn9, var_vth0_stress_dn10, var_vth0_stress_dn11,)
    }
};
        var_vth0_stress = assign15600_e21505;
        var_vth0_stress_dn3 = assign15600_e21505_d_n3;
        var_vth0_stress_dn4 = assign15600_e21505_d_n4;
        var_vth0_stress_dn5 = assign15600_e21505_d_n5;
        var_vth0_stress_dn6 = assign15600_e21505_d_n6;
        var_vth0_stress_dn7 = assign15600_e21505_d_n7;
        var_vth0_stress_dn8 = assign15600_e21505_d_n8;
        var_vth0_stress_dn9 = assign15600_e21505_d_n9;
        var_vth0_stress_dn10 = assign15600_e21505_d_n10;
        var_vth0_stress_dn11 = assign15600_e21505_d_n11;
        var_vth0_stress_rv = 0.0;

        let (assign15610_e21517, assign15610_e21517_d_n3, assign15610_e21517_d_n4, assign15610_e21517_d_n5, assign15610_e21517_d_n6, assign15610_e21517_d_n7, assign15610_e21517_d_n8, assign15610_e21517_d_n9, assign15610_e21517_d_n10, assign15610_e21517_d_n11,) = {
    if (var_guard480 != 0.0) {
        let assign15610_e21510: f64 = (var_kstress_vth0).powf(p.p1120);
        let assign15610_e21511: f64 = (p.p1119 / assign15610_e21510);
        let assign15610_e21514: f64 = (var_inv_od - var_inv_odref);
        let assign15610_e21515: f64 = (assign15610_e21511 * assign15610_e21514);
        (assign15610_e21515, (((-((p.p1119 * if 0.0 == 0.0 && ((p.p1120) as f64).is_finite() && ((p.p1120) as f64).fract() == 0.0 { if p.p1120 == 0.0 { 0.0 } else { (p.p1120 * ((var_kstress_vth0).powf(p.p1120 - 1.0) * var_kstress_vth0_dn3)) } } else { (assign15610_e21510 * (p.p1120 * (var_kstress_vth0_dn3 / var_kstress_vth0))) }) / (assign15610_e21510 * assign15610_e21510))) * assign15610_e21514) + (assign15610_e21511 * var_inv_od_dn3)), (((-((p.p1119 * if 0.0 == 0.0 && ((p.p1120) as f64).is_finite() && ((p.p1120) as f64).fract() == 0.0 { if p.p1120 == 0.0 { 0.0 } else { (p.p1120 * ((var_kstress_vth0).powf(p.p1120 - 1.0) * var_kstress_vth0_dn4)) } } else { (assign15610_e21510 * (p.p1120 * (var_kstress_vth0_dn4 / var_kstress_vth0))) }) / (assign15610_e21510 * assign15610_e21510))) * assign15610_e21514) + (assign15610_e21511 * var_inv_od_dn4)), (((-((p.p1119 * if 0.0 == 0.0 && ((p.p1120) as f64).is_finite() && ((p.p1120) as f64).fract() == 0.0 { if p.p1120 == 0.0 { 0.0 } else { (p.p1120 * ((var_kstress_vth0).powf(p.p1120 - 1.0) * var_kstress_vth0_dn5)) } } else { (assign15610_e21510 * (p.p1120 * (var_kstress_vth0_dn5 / var_kstress_vth0))) }) / (assign15610_e21510 * assign15610_e21510))) * assign15610_e21514) + (assign15610_e21511 * var_inv_od_dn5)), (((-((p.p1119 * if 0.0 == 0.0 && ((p.p1120) as f64).is_finite() && ((p.p1120) as f64).fract() == 0.0 { if p.p1120 == 0.0 { 0.0 } else { (p.p1120 * ((var_kstress_vth0).powf(p.p1120 - 1.0) * var_kstress_vth0_dn6)) } } else { (assign15610_e21510 * (p.p1120 * (var_kstress_vth0_dn6 / var_kstress_vth0))) }) / (assign15610_e21510 * assign15610_e21510))) * assign15610_e21514) + (assign15610_e21511 * var_inv_od_dn6)), (((-((p.p1119 * if 0.0 == 0.0 && ((p.p1120) as f64).is_finite() && ((p.p1120) as f64).fract() == 0.0 { if p.p1120 == 0.0 { 0.0 } else { (p.p1120 * ((var_kstress_vth0).powf(p.p1120 - 1.0) * var_kstress_vth0_dn7)) } } else { (assign15610_e21510 * (p.p1120 * (var_kstress_vth0_dn7 / var_kstress_vth0))) }) / (assign15610_e21510 * assign15610_e21510))) * assign15610_e21514) + (assign15610_e21511 * var_inv_od_dn7)), (((-((p.p1119 * if 0.0 == 0.0 && ((p.p1120) as f64).is_finite() && ((p.p1120) as f64).fract() == 0.0 { if p.p1120 == 0.0 { 0.0 } else { (p.p1120 * ((var_kstress_vth0).powf(p.p1120 - 1.0) * var_kstress_vth0_dn8)) } } else { (assign15610_e21510 * (p.p1120 * (var_kstress_vth0_dn8 / var_kstress_vth0))) }) / (assign15610_e21510 * assign15610_e21510))) * assign15610_e21514) + (assign15610_e21511 * var_inv_od_dn8)), (((-((p.p1119 * if 0.0 == 0.0 && ((p.p1120) as f64).is_finite() && ((p.p1120) as f64).fract() == 0.0 { if p.p1120 == 0.0 { 0.0 } else { (p.p1120 * ((var_kstress_vth0).powf(p.p1120 - 1.0) * var_kstress_vth0_dn9)) } } else { (assign15610_e21510 * (p.p1120 * (var_kstress_vth0_dn9 / var_kstress_vth0))) }) / (assign15610_e21510 * assign15610_e21510))) * assign15610_e21514) + (assign15610_e21511 * var_inv_od_dn9)), (((-((p.p1119 * if 0.0 == 0.0 && ((p.p1120) as f64).is_finite() && ((p.p1120) as f64).fract() == 0.0 { if p.p1120 == 0.0 { 0.0 } else { (p.p1120 * ((var_kstress_vth0).powf(p.p1120 - 1.0) * var_kstress_vth0_dn10)) } } else { (assign15610_e21510 * (p.p1120 * (var_kstress_vth0_dn10 / var_kstress_vth0))) }) / (assign15610_e21510 * assign15610_e21510))) * assign15610_e21514) + (assign15610_e21511 * var_inv_od_dn10)), (((-((p.p1119 * if 0.0 == 0.0 && ((p.p1120) as f64).is_finite() && ((p.p1120) as f64).fract() == 0.0 { if p.p1120 == 0.0 { 0.0 } else { (p.p1120 * ((var_kstress_vth0).powf(p.p1120 - 1.0) * var_kstress_vth0_dn11)) } } else { (assign15610_e21510 * (p.p1120 * (var_kstress_vth0_dn11 / var_kstress_vth0))) }) / (assign15610_e21510 * assign15610_e21510))) * assign15610_e21514) + (assign15610_e21511 * var_inv_od_dn11)),)
    } else {
        (var_k2_stress, var_k2_stress_dn3, var_k2_stress_dn4, var_k2_stress_dn5, var_k2_stress_dn6, var_k2_stress_dn7, var_k2_stress_dn8, var_k2_stress_dn9, var_k2_stress_dn10, var_k2_stress_dn11,)
    }
};
        var_k2_stress = assign15610_e21517;
        var_k2_stress_dn3 = assign15610_e21517_d_n3;
        var_k2_stress_dn4 = assign15610_e21517_d_n4;
        var_k2_stress_dn5 = assign15610_e21517_d_n5;
        var_k2_stress_dn6 = assign15610_e21517_d_n6;
        var_k2_stress_dn7 = assign15610_e21517_d_n7;
        var_k2_stress_dn8 = assign15610_e21517_d_n8;
        var_k2_stress_dn9 = assign15610_e21517_d_n9;
        var_k2_stress_dn10 = assign15610_e21517_d_n10;
        var_k2_stress_dn11 = assign15610_e21517_d_n11;
        var_k2_stress_rv = 0.0;

        *var_i_slot = var_i;
        *var_i_rv_slot = var_i_rv;
        *var_inv_od_slot = var_inv_od;
        *var_inv_od_dn10_slot = var_inv_od_dn10;
        *var_inv_od_dn11_slot = var_inv_od_dn11;
        *var_inv_od_dn3_slot = var_inv_od_dn3;
        *var_inv_od_dn4_slot = var_inv_od_dn4;
        *var_inv_od_dn5_slot = var_inv_od_dn5;
        *var_inv_od_dn6_slot = var_inv_od_dn6;
        *var_inv_od_dn7_slot = var_inv_od_dn7;
        *var_inv_od_dn8_slot = var_inv_od_dn8;
        *var_inv_od_dn9_slot = var_inv_od_dn9;
        *var_inv_od_rv_slot = var_inv_od_rv;
        *var_inv_odref_slot = var_inv_odref;
        *var_inv_odref_rv_slot = var_inv_odref_rv;
        *var_inv_sa_slot = var_inv_sa;
        *var_inv_sa_dn10_slot = var_inv_sa_dn10;
        *var_inv_sa_dn11_slot = var_inv_sa_dn11;
        *var_inv_sa_dn3_slot = var_inv_sa_dn3;
        *var_inv_sa_dn4_slot = var_inv_sa_dn4;
        *var_inv_sa_dn5_slot = var_inv_sa_dn5;
        *var_inv_sa_dn6_slot = var_inv_sa_dn6;
        *var_inv_sa_dn7_slot = var_inv_sa_dn7;
        *var_inv_sa_dn8_slot = var_inv_sa_dn8;
        *var_inv_sa_dn9_slot = var_inv_sa_dn9;
        *var_inv_sa_rv_slot = var_inv_sa_rv;
        *var_inv_saref_slot = var_inv_saref;
        *var_inv_saref_rv_slot = var_inv_saref_rv;
        *var_inv_sb_slot = var_inv_sb;
        *var_inv_sb_dn10_slot = var_inv_sb_dn10;
        *var_inv_sb_dn11_slot = var_inv_sb_dn11;
        *var_inv_sb_dn3_slot = var_inv_sb_dn3;
        *var_inv_sb_dn4_slot = var_inv_sb_dn4;
        *var_inv_sb_dn5_slot = var_inv_sb_dn5;
        *var_inv_sb_dn6_slot = var_inv_sb_dn6;
        *var_inv_sb_dn7_slot = var_inv_sb_dn7;
        *var_inv_sb_dn8_slot = var_inv_sb_dn8;
        *var_inv_sb_dn9_slot = var_inv_sb_dn9;
        *var_inv_sb_rv_slot = var_inv_sb_rv;
        *var_inv_sbref_slot = var_inv_sbref;
        *var_inv_sbref_rv_slot = var_inv_sbref_rv;
        *var_k2_stress_slot = var_k2_stress;
        *var_k2_stress_dn10_slot = var_k2_stress_dn10;
        *var_k2_stress_dn11_slot = var_k2_stress_dn11;
        *var_k2_stress_dn3_slot = var_k2_stress_dn3;
        *var_k2_stress_dn4_slot = var_k2_stress_dn4;
        *var_k2_stress_dn5_slot = var_k2_stress_dn5;
        *var_k2_stress_dn6_slot = var_k2_stress_dn6;
        *var_k2_stress_dn7_slot = var_k2_stress_dn7;
        *var_k2_stress_dn8_slot = var_k2_stress_dn8;
        *var_k2_stress_dn9_slot = var_k2_stress_dn9;
        *var_k2_stress_rv_slot = var_k2_stress_rv;
        *var_kstress_u0_slot = var_kstress_u0;
        *var_kstress_u0_dn10_slot = var_kstress_u0_dn10;
        *var_kstress_u0_dn11_slot = var_kstress_u0_dn11;
        *var_kstress_u0_dn3_slot = var_kstress_u0_dn3;
        *var_kstress_u0_dn4_slot = var_kstress_u0_dn4;
        *var_kstress_u0_dn5_slot = var_kstress_u0_dn5;
        *var_kstress_u0_dn6_slot = var_kstress_u0_dn6;
        *var_kstress_u0_dn7_slot = var_kstress_u0_dn7;
        *var_kstress_u0_dn8_slot = var_kstress_u0_dn8;
        *var_kstress_u0_dn9_slot = var_kstress_u0_dn9;
        *var_kstress_u0_rv_slot = var_kstress_u0_rv;
        *var_kstress_vth0_slot = var_kstress_vth0;
        *var_kstress_vth0_dn10_slot = var_kstress_vth0_dn10;
        *var_kstress_vth0_dn11_slot = var_kstress_vth0_dn11;
        *var_kstress_vth0_dn3_slot = var_kstress_vth0_dn3;
        *var_kstress_vth0_dn4_slot = var_kstress_vth0_dn4;
        *var_kstress_vth0_dn5_slot = var_kstress_vth0_dn5;
        *var_kstress_vth0_dn6_slot = var_kstress_vth0_dn6;
        *var_kstress_vth0_dn7_slot = var_kstress_vth0_dn7;
        *var_kstress_vth0_dn8_slot = var_kstress_vth0_dn8;
        *var_kstress_vth0_dn9_slot = var_kstress_vth0_dn9;
        *var_kstress_vth0_rv_slot = var_kstress_vth0_rv;
        *var_ku0_temp_slot = var_ku0_temp;
        *var_ku0_temp_dn10_slot = var_ku0_temp_dn10;
        *var_ku0_temp_dn11_slot = var_ku0_temp_dn11;
        *var_ku0_temp_dn3_slot = var_ku0_temp_dn3;
        *var_ku0_temp_dn4_slot = var_ku0_temp_dn4;
        *var_ku0_temp_dn5_slot = var_ku0_temp_dn5;
        *var_ku0_temp_dn6_slot = var_ku0_temp_dn6;
        *var_ku0_temp_dn7_slot = var_ku0_temp_dn7;
        *var_ku0_temp_dn8_slot = var_ku0_temp_dn8;
        *var_ku0_temp_dn9_slot = var_ku0_temp_dn9;
        *var_ku0_temp_rv_slot = var_ku0_temp_rv;
        *var_mu0_mult_slot = var_mu0_mult;
        *var_mu0_mult_dn10_slot = var_mu0_mult_dn10;
        *var_mu0_mult_dn11_slot = var_mu0_mult_dn11;
        *var_mu0_mult_dn3_slot = var_mu0_mult_dn3;
        *var_mu0_mult_dn4_slot = var_mu0_mult_dn4;
        *var_mu0_mult_dn5_slot = var_mu0_mult_dn5;
        *var_mu0_mult_dn6_slot = var_mu0_mult_dn6;
        *var_mu0_mult_dn7_slot = var_mu0_mult_dn7;
        *var_mu0_mult_dn8_slot = var_mu0_mult_dn8;
        *var_mu0_mult_dn9_slot = var_mu0_mult_dn9;
        *var_mu0_mult_rv_slot = var_mu0_mult_rv;
        *var_rho_slot = var_rho;
        *var_rho_dn10_slot = var_rho_dn10;
        *var_rho_dn11_slot = var_rho_dn11;
        *var_rho_dn3_slot = var_rho_dn3;
        *var_rho_dn4_slot = var_rho_dn4;
        *var_rho_dn5_slot = var_rho_dn5;
        *var_rho_dn6_slot = var_rho_dn6;
        *var_rho_dn7_slot = var_rho_dn7;
        *var_rho_dn8_slot = var_rho_dn8;
        *var_rho_dn9_slot = var_rho_dn9;
        *var_rho_ref_slot = var_rho_ref;
        *var_rho_ref_dn10_slot = var_rho_ref_dn10;
        *var_rho_ref_dn11_slot = var_rho_ref_dn11;
        *var_rho_ref_dn3_slot = var_rho_ref_dn3;
        *var_rho_ref_dn4_slot = var_rho_ref_dn4;
        *var_rho_ref_dn5_slot = var_rho_ref_dn5;
        *var_rho_ref_dn6_slot = var_rho_ref_dn6;
        *var_rho_ref_dn7_slot = var_rho_ref_dn7;
        *var_rho_ref_dn8_slot = var_rho_ref_dn8;
        *var_rho_ref_dn9_slot = var_rho_ref_dn9;
        *var_rho_ref_rv_slot = var_rho_ref_rv;
        *var_rho_rv_slot = var_rho_rv;
        *var_t0_slot = var_t0;
        *var_t0_dn10_slot = var_t0_dn10;
        *var_t0_dn11_slot = var_t0_dn11;
        *var_t0_dn3_slot = var_t0_dn3;
        *var_t0_dn4_slot = var_t0_dn4;
        *var_t0_dn5_slot = var_t0_dn5;
        *var_t0_dn6_slot = var_t0_dn6;
        *var_t0_dn7_slot = var_t0_dn7;
        *var_t0_dn8_slot = var_t0_dn8;
        *var_t0_dn9_slot = var_t0_dn9;
        *var_t0_rv_slot = var_t0_rv;
        *var_t1_slot = var_t1;
        *var_t1_dn10_slot = var_t1_dn10;
        *var_t1_dn11_slot = var_t1_dn11;
        *var_t1_dn3_slot = var_t1_dn3;
        *var_t1_dn4_slot = var_t1_dn4;
        *var_t1_dn5_slot = var_t1_dn5;
        *var_t1_dn6_slot = var_t1_dn6;
        *var_t1_dn7_slot = var_t1_dn7;
        *var_t1_dn8_slot = var_t1_dn8;
        *var_t1_dn9_slot = var_t1_dn9;
        *var_t1_rv_slot = var_t1_rv;
        *var_tmp1_stress_slot = var_tmp1_stress;
        *var_tmp1_stress_dn10_slot = var_tmp1_stress_dn10;
        *var_tmp1_stress_dn11_slot = var_tmp1_stress_dn11;
        *var_tmp1_stress_dn3_slot = var_tmp1_stress_dn3;
        *var_tmp1_stress_dn4_slot = var_tmp1_stress_dn4;
        *var_tmp1_stress_dn5_slot = var_tmp1_stress_dn5;
        *var_tmp1_stress_dn6_slot = var_tmp1_stress_dn6;
        *var_tmp1_stress_dn7_slot = var_tmp1_stress_dn7;
        *var_tmp1_stress_dn8_slot = var_tmp1_stress_dn8;
        *var_tmp1_stress_dn9_slot = var_tmp1_stress_dn9;
        *var_tmp1_stress_rv_slot = var_tmp1_stress_rv;
        *var_tmp1_stress_vth_slot = var_tmp1_stress_vth;
        *var_tmp1_stress_vth_dn10_slot = var_tmp1_stress_vth_dn10;
        *var_tmp1_stress_vth_dn11_slot = var_tmp1_stress_vth_dn11;
        *var_tmp1_stress_vth_dn3_slot = var_tmp1_stress_vth_dn3;
        *var_tmp1_stress_vth_dn4_slot = var_tmp1_stress_vth_dn4;
        *var_tmp1_stress_vth_dn5_slot = var_tmp1_stress_vth_dn5;
        *var_tmp1_stress_vth_dn6_slot = var_tmp1_stress_vth_dn6;
        *var_tmp1_stress_vth_dn7_slot = var_tmp1_stress_vth_dn7;
        *var_tmp1_stress_vth_dn8_slot = var_tmp1_stress_vth_dn8;
        *var_tmp1_stress_vth_dn9_slot = var_tmp1_stress_vth_dn9;
        *var_tmp1_stress_vth_rv_slot = var_tmp1_stress_vth_rv;
        *var_vsat_mult_slot = var_vsat_mult;
        *var_vsat_mult_dn10_slot = var_vsat_mult_dn10;
        *var_vsat_mult_dn11_slot = var_vsat_mult_dn11;
        *var_vsat_mult_dn3_slot = var_vsat_mult_dn3;
        *var_vsat_mult_dn4_slot = var_vsat_mult_dn4;
        *var_vsat_mult_dn5_slot = var_vsat_mult_dn5;
        *var_vsat_mult_dn6_slot = var_vsat_mult_dn6;
        *var_vsat_mult_dn7_slot = var_vsat_mult_dn7;
        *var_vsat_mult_dn8_slot = var_vsat_mult_dn8;
        *var_vsat_mult_dn9_slot = var_vsat_mult_dn9;
        *var_vsat_mult_rv_slot = var_vsat_mult_rv;
        *var_vth0_stress_slot = var_vth0_stress;
        *var_vth0_stress_dn10_slot = var_vth0_stress_dn10;
        *var_vth0_stress_dn11_slot = var_vth0_stress_dn11;
        *var_vth0_stress_dn3_slot = var_vth0_stress_dn3;
        *var_vth0_stress_dn4_slot = var_vth0_stress_dn4;
        *var_vth0_stress_dn5_slot = var_vth0_stress_dn5;
        *var_vth0_stress_dn6_slot = var_vth0_stress_dn6;
        *var_vth0_stress_dn7_slot = var_vth0_stress_dn7;
        *var_vth0_stress_dn8_slot = var_vth0_stress_dn8;
        *var_vth0_stress_dn9_slot = var_vth0_stress_dn9;
        *var_vth0_stress_rv_slot = var_vth0_stress_rv;
    }
}
