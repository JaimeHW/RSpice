#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_13(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign7210_e8604: f64 = if (((p.p9 == 3.0) || (p.p9 == 4.0)) || (p.p9 == 6.0)) { 1.0 } else { 0.0 };
        locals.var_guard134 = assign7210_e8604;
        locals.var_guard134_rv = 0.0;

        let assign7220_e8607: f64 = if locals.var_nuendd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard135 = assign7220_e8607;
        locals.var_guard135_rv = 0.0;

        let (assign7230_e8628,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard87 != 0.0) && (locals.var_guard86 == 0.0))) && (locals.var_guard120 == 0.0)) && (locals.var_guard132 != 0.0)) && (locals.var_guard133 != 0.0)) && (locals.var_guard135 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7230_e8628;
        locals.var_rend_rv = 0.0;

        let (assign7240_e8656,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard87 != 0.0) && (locals.var_guard86 == 0.0))) && (locals.var_guard120 == 0.0)) && (locals.var_guard132 != 0.0)) && (locals.var_guard133 != 0.0)) && (locals.var_guard135 == 0.0)) {
        let assign7240_e8650: f64 = (p.p438 * locals.var_dmcgeff);
        let assign7240_e8653: f64 = (locals.var_weff * locals.var_nuendd);
        let assign7240_e8654: f64 = (assign7240_e8650 / assign7240_e8653);
        (assign7240_e8654,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7240_e8656;
        locals.var_rend_rv = 0.0;

        let assign7260_e8666: f64 = if ((locals.var_nuendd == 0.0) || (locals.var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard137 = assign7260_e8666;
        locals.var_guard137_rv = 0.0;

        let (assign7270_e8690,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard87 != 0.0) && (locals.var_guard86 == 0.0))) && (locals.var_guard120 == 0.0)) && (locals.var_guard132 != 0.0)) && ((locals.var_guard134 != 0.0) && (locals.var_guard133 == 0.0))) && (locals.var_guard137 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7270_e8690;
        locals.var_rend_rv = 0.0;

        let (assign7280_e8723,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard87 != 0.0) && (locals.var_guard86 == 0.0))) && (locals.var_guard120 == 0.0)) && (locals.var_guard132 != 0.0)) && ((locals.var_guard134 != 0.0) && (locals.var_guard133 == 0.0))) && (locals.var_guard137 == 0.0)) {
        let assign7280_e8715: f64 = (p.p438 * locals.var_weff);
        let assign7280_e8718: f64 = (6.0 * locals.var_nuendd);
        let assign7280_e8720: f64 = (assign7280_e8718 * locals.var_dmcgeff);
        let assign7280_e8721: f64 = (assign7280_e8715 / assign7280_e8720);
        (assign7280_e8721,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7280_e8723;
        locals.var_rend_rv = 0.0;

        let (assign7290_e8745,) = {
    if ((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard87 != 0.0) && (locals.var_guard86 == 0.0))) && (locals.var_guard120 == 0.0)) && (locals.var_guard132 != 0.0)) && (!((locals.var_guard133 != 0.0) || (locals.var_guard134 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7290_e8745;
        locals.var_rend_rv = 0.0;

        let assign7300_e8756: f64 = if (((p.p9 == 1.0) || (p.p9 == 3.0)) || (p.p9 == 7.0)) { 1.0 } else { 0.0 };
        locals.var_guard138 = assign7300_e8756;
        locals.var_guard138_rv = 0.0;

        let assign7310_e8767: f64 = if (((p.p9 == 2.0) || (p.p9 == 4.0)) || (p.p9 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard139 = assign7310_e8767;
        locals.var_guard139_rv = 0.0;

        let assign7320_e8770: f64 = if locals.var_nuendd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard140 = assign7320_e8770;
        locals.var_guard140_rv = 0.0;

        let (assign7330_e8792,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard87 != 0.0) && (locals.var_guard86 == 0.0))) && (locals.var_guard120 == 0.0)) && (locals.var_guard132 == 0.0)) && (locals.var_guard138 != 0.0)) && (locals.var_guard140 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7330_e8792;
        locals.var_rend_rv = 0.0;

        let (assign7340_e8821,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard87 != 0.0) && (locals.var_guard86 == 0.0))) && (locals.var_guard120 == 0.0)) && (locals.var_guard132 == 0.0)) && (locals.var_guard138 != 0.0)) && (locals.var_guard140 == 0.0)) {
        let assign7340_e8815: f64 = (p.p438 * locals.var_dmcgeff);
        let assign7340_e8818: f64 = (locals.var_weff * locals.var_nuendd);
        let assign7340_e8819: f64 = (assign7340_e8815 / assign7340_e8818);
        (assign7340_e8819,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7340_e8821;
        locals.var_rend_rv = 0.0;

        let assign7360_e8831: f64 = if ((locals.var_nuendd == 0.0) || (locals.var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard142 = assign7360_e8831;
        locals.var_guard142_rv = 0.0;

        let (assign7370_e8856,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard87 != 0.0) && (locals.var_guard86 == 0.0))) && (locals.var_guard120 == 0.0)) && (locals.var_guard132 == 0.0)) && ((locals.var_guard139 != 0.0) && (locals.var_guard138 == 0.0))) && (locals.var_guard142 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7370_e8856;
        locals.var_rend_rv = 0.0;

        let (assign7380_e8890,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard87 != 0.0) && (locals.var_guard86 == 0.0))) && (locals.var_guard120 == 0.0)) && (locals.var_guard132 == 0.0)) && ((locals.var_guard139 != 0.0) && (locals.var_guard138 == 0.0))) && (locals.var_guard142 == 0.0)) {
        let assign7380_e8882: f64 = (p.p438 * locals.var_weff);
        let assign7380_e8885: f64 = (6.0 * locals.var_nuendd);
        let assign7380_e8887: f64 = (assign7380_e8885 * locals.var_dmcgeff);
        let assign7380_e8888: f64 = (assign7380_e8882 / assign7380_e8887);
        (assign7380_e8888,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7380_e8890;
        locals.var_rend_rv = 0.0;

        let (assign7390_e8913,) = {
    if ((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard87 != 0.0) && (locals.var_guard86 == 0.0))) && (locals.var_guard120 == 0.0)) && (locals.var_guard132 == 0.0)) && (!((locals.var_guard138 != 0.0) || (locals.var_guard139 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7390_e8913;
        locals.var_rend_rv = 0.0;

        let assign7400_e8916: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard143 = assign7400_e8916;
        locals.var_guard143_rv = 0.0;

        let assign7410_e8919: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard144 = assign7410_e8919;
        locals.var_guard144_rv = 0.0;

        let assign7420_e8930: f64 = if (((p.p9 == 1.0) || (p.p9 == 2.0)) || (p.p9 == 5.0)) { 1.0 } else { 0.0 };
        locals.var_guard145 = assign7420_e8930;
        locals.var_guard145_rv = 0.0;

        let assign7430_e8941: f64 = if (((p.p9 == 3.0) || (p.p9 == 4.0)) || (p.p9 == 6.0)) { 1.0 } else { 0.0 };
        locals.var_guard146 = assign7430_e8941;
        locals.var_guard146_rv = 0.0;

        let assign7440_e8944: f64 = if locals.var_nuends == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard147 = assign7440_e8944;
        locals.var_guard147_rv = 0.0;

        let (assign7450_e8966,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard88 != 0.0) && (!((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0))))) && (locals.var_guard143 != 0.0)) && (locals.var_guard144 != 0.0)) && (locals.var_guard145 != 0.0)) && (locals.var_guard147 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7450_e8966;
        locals.var_rend_rv = 0.0;

        let (assign7460_e8995,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard88 != 0.0) && (!((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0))))) && (locals.var_guard143 != 0.0)) && (locals.var_guard144 != 0.0)) && (locals.var_guard145 != 0.0)) && (locals.var_guard147 == 0.0)) {
        let assign7460_e8989: f64 = (p.p438 * locals.var_dmcgeff);
        let assign7460_e8992: f64 = (locals.var_weff * locals.var_nuends);
        let assign7460_e8993: f64 = (assign7460_e8989 / assign7460_e8992);
        (assign7460_e8993,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7460_e8995;
        locals.var_rend_rv = 0.0;

        let assign7480_e9005: f64 = if ((locals.var_nuends == 0.0) || (locals.var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard149 = assign7480_e9005;
        locals.var_guard149_rv = 0.0;

        let (assign7490_e9030,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard88 != 0.0) && (!((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0))))) && (locals.var_guard143 != 0.0)) && (locals.var_guard144 != 0.0)) && ((locals.var_guard146 != 0.0) && (locals.var_guard145 == 0.0))) && (locals.var_guard149 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7490_e9030;
        locals.var_rend_rv = 0.0;

        let (assign7500_e9064,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard88 != 0.0) && (!((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0))))) && (locals.var_guard143 != 0.0)) && (locals.var_guard144 != 0.0)) && ((locals.var_guard146 != 0.0) && (locals.var_guard145 == 0.0))) && (locals.var_guard149 == 0.0)) {
        let assign7500_e9056: f64 = (p.p438 * locals.var_weff);
        let assign7500_e9059: f64 = (6.0 * locals.var_nuends);
        let assign7500_e9061: f64 = (assign7500_e9059 * locals.var_dmcgeff);
        let assign7500_e9062: f64 = (assign7500_e9056 / assign7500_e9061);
        (assign7500_e9062,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7500_e9064;
        locals.var_rend_rv = 0.0;

        let (assign7510_e9087,) = {
    if ((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard88 != 0.0) && (!((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0))))) && (locals.var_guard143 != 0.0)) && (locals.var_guard144 != 0.0)) && (!((locals.var_guard145 != 0.0) || (locals.var_guard146 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7510_e9087;
        locals.var_rend_rv = 0.0;

        let assign7520_e9098: f64 = if (((p.p9 == 1.0) || (p.p9 == 3.0)) || (p.p9 == 7.0)) { 1.0 } else { 0.0 };
        locals.var_guard150 = assign7520_e9098;
        locals.var_guard150_rv = 0.0;

        let assign7530_e9109: f64 = if (((p.p9 == 2.0) || (p.p9 == 4.0)) || (p.p9 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard151 = assign7530_e9109;
        locals.var_guard151_rv = 0.0;

        let assign7540_e9112: f64 = if locals.var_nuends == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard152 = assign7540_e9112;
        locals.var_guard152_rv = 0.0;

        let (assign7550_e9135,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard88 != 0.0) && (!((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0))))) && (locals.var_guard143 != 0.0)) && (locals.var_guard144 == 0.0)) && (locals.var_guard150 != 0.0)) && (locals.var_guard152 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7550_e9135;
        locals.var_rend_rv = 0.0;

        let (assign7560_e9165,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard88 != 0.0) && (!((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0))))) && (locals.var_guard143 != 0.0)) && (locals.var_guard144 == 0.0)) && (locals.var_guard150 != 0.0)) && (locals.var_guard152 == 0.0)) {
        let assign7560_e9159: f64 = (p.p438 * locals.var_dmcgeff);
        let assign7560_e9162: f64 = (locals.var_weff * locals.var_nuends);
        let assign7560_e9163: f64 = (assign7560_e9159 / assign7560_e9162);
        (assign7560_e9163,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7560_e9165;
        locals.var_rend_rv = 0.0;

        let assign7580_e9175: f64 = if ((locals.var_nuends == 0.0) || (locals.var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard154 = assign7580_e9175;
        locals.var_guard154_rv = 0.0;

        let (assign7590_e9201,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard88 != 0.0) && (!((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0))))) && (locals.var_guard143 != 0.0)) && (locals.var_guard144 == 0.0)) && ((locals.var_guard151 != 0.0) && (locals.var_guard150 == 0.0))) && (locals.var_guard154 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7590_e9201;
        locals.var_rend_rv = 0.0;

        let (assign7600_e9236,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard88 != 0.0) && (!((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0))))) && (locals.var_guard143 != 0.0)) && (locals.var_guard144 == 0.0)) && ((locals.var_guard151 != 0.0) && (locals.var_guard150 == 0.0))) && (locals.var_guard154 == 0.0)) {
        let assign7600_e9228: f64 = (p.p438 * locals.var_weff);
        let assign7600_e9231: f64 = (6.0 * locals.var_nuends);
        let assign7600_e9233: f64 = (assign7600_e9231 * locals.var_dmcgeff);
        let assign7600_e9234: f64 = (assign7600_e9228 / assign7600_e9233);
        (assign7600_e9234,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7600_e9236;
        locals.var_rend_rv = 0.0;

        let (assign7610_e9260,) = {
    if ((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard88 != 0.0) && (!((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0))))) && (locals.var_guard143 != 0.0)) && (locals.var_guard144 == 0.0)) && (!((locals.var_guard150 != 0.0) || (locals.var_guard151 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7610_e9260;
        locals.var_rend_rv = 0.0;

        let assign7620_e9263: f64 = if 0.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard155 = assign7620_e9263;
        locals.var_guard155_rv = 0.0;

        let assign7630_e9274: f64 = if (((p.p9 == 1.0) || (p.p9 == 2.0)) || (p.p9 == 5.0)) { 1.0 } else { 0.0 };
        locals.var_guard156 = assign7630_e9274;
        locals.var_guard156_rv = 0.0;

        let assign7640_e9285: f64 = if (((p.p9 == 3.0) || (p.p9 == 4.0)) || (p.p9 == 6.0)) { 1.0 } else { 0.0 };
        locals.var_guard157 = assign7640_e9285;
        locals.var_guard157_rv = 0.0;

        let assign7650_e9288: f64 = if locals.var_nuendd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard158 = assign7650_e9288;
        locals.var_guard158_rv = 0.0;

        let (assign7660_e9311,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard88 != 0.0) && (!((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0))))) && (locals.var_guard143 == 0.0)) && (locals.var_guard155 != 0.0)) && (locals.var_guard156 != 0.0)) && (locals.var_guard158 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7660_e9311;
        locals.var_rend_rv = 0.0;

        let (assign7670_e9341,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard88 != 0.0) && (!((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0))))) && (locals.var_guard143 == 0.0)) && (locals.var_guard155 != 0.0)) && (locals.var_guard156 != 0.0)) && (locals.var_guard158 == 0.0)) {
        let assign7670_e9335: f64 = (p.p438 * locals.var_dmcgeff);
        let assign7670_e9338: f64 = (locals.var_weff * locals.var_nuendd);
        let assign7670_e9339: f64 = (assign7670_e9335 / assign7670_e9338);
        (assign7670_e9339,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7670_e9341;
        locals.var_rend_rv = 0.0;

        let assign7690_e9352: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign7690_e9355: f64 = if ((locals.var_nuendd == 0.0) || (assign7690_e9352 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard160 = assign7690_e9355;
        locals.var_guard160_rv = 0.0;

        let (assign7700_e9381,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard88 != 0.0) && (!((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0))))) && (locals.var_guard143 == 0.0)) && (locals.var_guard155 != 0.0)) && ((locals.var_guard157 != 0.0) && (locals.var_guard156 == 0.0))) && (locals.var_guard160 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7700_e9381;
        locals.var_rend_rv = 0.0;

        let (assign7710_e9418,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard88 != 0.0) && (!((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0))))) && (locals.var_guard143 == 0.0)) && (locals.var_guard155 != 0.0)) && ((locals.var_guard157 != 0.0) && (locals.var_guard156 == 0.0))) && (locals.var_guard160 == 0.0)) {
        let assign7710_e9408: f64 = (p.p438 * locals.var_weff);
        let assign7710_e9411: f64 = (3.0 * locals.var_nuendd);
        let assign7710_e9414: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign7710_e9415: f64 = (assign7710_e9411 * assign7710_e9414);
        let assign7710_e9416: f64 = (assign7710_e9408 / assign7710_e9415);
        (assign7710_e9416,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7710_e9418;
        locals.var_rend_rv = 0.0;

        let (assign7720_e9442,) = {
    if ((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard88 != 0.0) && (!((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0))))) && (locals.var_guard143 == 0.0)) && (locals.var_guard155 != 0.0)) && (!((locals.var_guard156 != 0.0) || (locals.var_guard157 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7720_e9442;
        locals.var_rend_rv = 0.0;

        let assign7730_e9453: f64 = if (((p.p9 == 1.0) || (p.p9 == 3.0)) || (p.p9 == 7.0)) { 1.0 } else { 0.0 };
        locals.var_guard161 = assign7730_e9453;
        locals.var_guard161_rv = 0.0;

        let assign7740_e9464: f64 = if (((p.p9 == 2.0) || (p.p9 == 4.0)) || (p.p9 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard162 = assign7740_e9464;
        locals.var_guard162_rv = 0.0;

        let assign7750_e9467: f64 = if locals.var_nuendd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard163 = assign7750_e9467;
        locals.var_guard163_rv = 0.0;

        let (assign7760_e9491,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard88 != 0.0) && (!((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0))))) && (locals.var_guard143 == 0.0)) && (locals.var_guard155 == 0.0)) && (locals.var_guard161 != 0.0)) && (locals.var_guard163 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7760_e9491;
        locals.var_rend_rv = 0.0;

        let (assign7770_e9522,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard88 != 0.0) && (!((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0))))) && (locals.var_guard143 == 0.0)) && (locals.var_guard155 == 0.0)) && (locals.var_guard161 != 0.0)) && (locals.var_guard163 == 0.0)) {
        let assign7770_e9516: f64 = (p.p438 * locals.var_dmcgeff);
        let assign7770_e9519: f64 = (locals.var_weff * locals.var_nuendd);
        let assign7770_e9520: f64 = (assign7770_e9516 / assign7770_e9519);
        (assign7770_e9520,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7770_e9522;
        locals.var_rend_rv = 0.0;

        let assign7790_e9533: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign7790_e9536: f64 = if ((locals.var_nuendd == 0.0) || (assign7790_e9533 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard165 = assign7790_e9536;
        locals.var_guard165_rv = 0.0;

        let (assign7800_e9563,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard88 != 0.0) && (!((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0))))) && (locals.var_guard143 == 0.0)) && (locals.var_guard155 == 0.0)) && ((locals.var_guard162 != 0.0) && (locals.var_guard161 == 0.0))) && (locals.var_guard165 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7800_e9563;
        locals.var_rend_rv = 0.0;

        let (assign7810_e9601,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard88 != 0.0) && (!((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0))))) && (locals.var_guard143 == 0.0)) && (locals.var_guard155 == 0.0)) && ((locals.var_guard162 != 0.0) && (locals.var_guard161 == 0.0))) && (locals.var_guard165 == 0.0)) {
        let assign7810_e9591: f64 = (p.p438 * locals.var_weff);
        let assign7810_e9594: f64 = (3.0 * locals.var_nuendd);
        let assign7810_e9597: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign7810_e9598: f64 = (assign7810_e9594 * assign7810_e9597);
        let assign7810_e9599: f64 = (assign7810_e9591 / assign7810_e9598);
        (assign7810_e9599,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7810_e9601;
        locals.var_rend_rv = 0.0;

        let (assign7820_e9626,) = {
    if ((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard88 != 0.0) && (!((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0))))) && (locals.var_guard143 == 0.0)) && (locals.var_guard155 == 0.0)) && (!((locals.var_guard161 != 0.0) || (locals.var_guard162 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7820_e9626;
        locals.var_rend_rv = 0.0;

        let assign7830_e9629: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard166 = assign7830_e9629;
        locals.var_guard166_rv = 0.0;

        let assign7840_e9632: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard167 = assign7840_e9632;
        locals.var_guard167_rv = 0.0;

        let assign7850_e9643: f64 = if (((p.p9 == 1.0) || (p.p9 == 2.0)) || (p.p9 == 5.0)) { 1.0 } else { 0.0 };
        locals.var_guard168 = assign7850_e9643;
        locals.var_guard168_rv = 0.0;

        let assign7860_e9654: f64 = if (((p.p9 == 3.0) || (p.p9 == 4.0)) || (p.p9 == 6.0)) { 1.0 } else { 0.0 };
        locals.var_guard169 = assign7860_e9654;
        locals.var_guard169_rv = 0.0;

        let assign7870_e9657: f64 = if locals.var_nuends == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard170 = assign7870_e9657;
        locals.var_guard170_rv = 0.0;

        let (assign7880_e9681,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard89 != 0.0) && (!(((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0))))) && (locals.var_guard166 != 0.0)) && (locals.var_guard167 != 0.0)) && (locals.var_guard168 != 0.0)) && (locals.var_guard170 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7880_e9681;
        locals.var_rend_rv = 0.0;

        let (assign7890_e9712,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard89 != 0.0) && (!(((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0))))) && (locals.var_guard166 != 0.0)) && (locals.var_guard167 != 0.0)) && (locals.var_guard168 != 0.0)) && (locals.var_guard170 == 0.0)) {
        let assign7890_e9706: f64 = (p.p438 * locals.var_dmcgeff);
        let assign7890_e9709: f64 = (locals.var_weff * locals.var_nuends);
        let assign7890_e9710: f64 = (assign7890_e9706 / assign7890_e9709);
        (assign7890_e9710,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7890_e9712;
        locals.var_rend_rv = 0.0;

        let assign7910_e9722: f64 = if ((locals.var_nuends == 0.0) || (locals.var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard172 = assign7910_e9722;
        locals.var_guard172_rv = 0.0;

        let (assign7920_e9749,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard89 != 0.0) && (!(((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0))))) && (locals.var_guard166 != 0.0)) && (locals.var_guard167 != 0.0)) && ((locals.var_guard169 != 0.0) && (locals.var_guard168 == 0.0))) && (locals.var_guard172 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7920_e9749;
        locals.var_rend_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_14(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign7930_e9785,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard89 != 0.0) && (!(((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0))))) && (locals.var_guard166 != 0.0)) && (locals.var_guard167 != 0.0)) && ((locals.var_guard169 != 0.0) && (locals.var_guard168 == 0.0))) && (locals.var_guard172 == 0.0)) {
        let assign7930_e9777: f64 = (p.p438 * locals.var_weff);
        let assign7930_e9780: f64 = (6.0 * locals.var_nuends);
        let assign7930_e9782: f64 = (assign7930_e9780 * locals.var_dmcgeff);
        let assign7930_e9783: f64 = (assign7930_e9777 / assign7930_e9782);
        (assign7930_e9783,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7930_e9785;
        locals.var_rend_rv = 0.0;

        let (assign7940_e9810,) = {
    if ((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard89 != 0.0) && (!(((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0))))) && (locals.var_guard166 != 0.0)) && (locals.var_guard167 != 0.0)) && (!((locals.var_guard168 != 0.0) || (locals.var_guard169 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7940_e9810;
        locals.var_rend_rv = 0.0;

        let assign7950_e9821: f64 = if (((p.p9 == 1.0) || (p.p9 == 3.0)) || (p.p9 == 7.0)) { 1.0 } else { 0.0 };
        locals.var_guard173 = assign7950_e9821;
        locals.var_guard173_rv = 0.0;

        let assign7960_e9832: f64 = if (((p.p9 == 2.0) || (p.p9 == 4.0)) || (p.p9 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard174 = assign7960_e9832;
        locals.var_guard174_rv = 0.0;

        let assign7970_e9835: f64 = if locals.var_nuends == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard175 = assign7970_e9835;
        locals.var_guard175_rv = 0.0;

        let (assign7980_e9860,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard89 != 0.0) && (!(((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0))))) && (locals.var_guard166 != 0.0)) && (locals.var_guard167 == 0.0)) && (locals.var_guard173 != 0.0)) && (locals.var_guard175 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7980_e9860;
        locals.var_rend_rv = 0.0;

        let (assign7990_e9892,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard89 != 0.0) && (!(((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0))))) && (locals.var_guard166 != 0.0)) && (locals.var_guard167 == 0.0)) && (locals.var_guard173 != 0.0)) && (locals.var_guard175 == 0.0)) {
        let assign7990_e9886: f64 = (p.p438 * locals.var_dmcgeff);
        let assign7990_e9889: f64 = (locals.var_weff * locals.var_nuends);
        let assign7990_e9890: f64 = (assign7990_e9886 / assign7990_e9889);
        (assign7990_e9890,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7990_e9892;
        locals.var_rend_rv = 0.0;

        let assign8010_e9902: f64 = if ((locals.var_nuends == 0.0) || (locals.var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard177 = assign8010_e9902;
        locals.var_guard177_rv = 0.0;

        let (assign8020_e9930,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard89 != 0.0) && (!(((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0))))) && (locals.var_guard166 != 0.0)) && (locals.var_guard167 == 0.0)) && ((locals.var_guard174 != 0.0) && (locals.var_guard173 == 0.0))) && (locals.var_guard177 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign8020_e9930;
        locals.var_rend_rv = 0.0;

        let (assign8030_e9967,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard89 != 0.0) && (!(((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0))))) && (locals.var_guard166 != 0.0)) && (locals.var_guard167 == 0.0)) && ((locals.var_guard174 != 0.0) && (locals.var_guard173 == 0.0))) && (locals.var_guard177 == 0.0)) {
        let assign8030_e9959: f64 = (p.p438 * locals.var_weff);
        let assign8030_e9962: f64 = (6.0 * locals.var_nuends);
        let assign8030_e9964: f64 = (assign8030_e9962 * locals.var_dmcgeff);
        let assign8030_e9965: f64 = (assign8030_e9959 / assign8030_e9964);
        (assign8030_e9965,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign8030_e9967;
        locals.var_rend_rv = 0.0;

        let (assign8040_e9993,) = {
    if ((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard89 != 0.0) && (!(((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0))))) && (locals.var_guard166 != 0.0)) && (locals.var_guard167 == 0.0)) && (!((locals.var_guard173 != 0.0) || (locals.var_guard174 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign8040_e9993;
        locals.var_rend_rv = 0.0;

        let assign8050_e9996: f64 = if 0.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard178 = assign8050_e9996;
        locals.var_guard178_rv = 0.0;

        let assign8060_e10007: f64 = if (((p.p9 == 1.0) || (p.p9 == 2.0)) || (p.p9 == 5.0)) { 1.0 } else { 0.0 };
        locals.var_guard179 = assign8060_e10007;
        locals.var_guard179_rv = 0.0;

        let assign8070_e10018: f64 = if (((p.p9 == 3.0) || (p.p9 == 4.0)) || (p.p9 == 6.0)) { 1.0 } else { 0.0 };
        locals.var_guard180 = assign8070_e10018;
        locals.var_guard180_rv = 0.0;

        let assign8080_e10021: f64 = if locals.var_nuendd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard181 = assign8080_e10021;
        locals.var_guard181_rv = 0.0;

        let (assign8090_e10046,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard89 != 0.0) && (!(((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0))))) && (locals.var_guard166 == 0.0)) && (locals.var_guard178 != 0.0)) && (locals.var_guard179 != 0.0)) && (locals.var_guard181 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign8090_e10046;
        locals.var_rend_rv = 0.0;

        let (assign8100_e10078,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard89 != 0.0) && (!(((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0))))) && (locals.var_guard166 == 0.0)) && (locals.var_guard178 != 0.0)) && (locals.var_guard179 != 0.0)) && (locals.var_guard181 == 0.0)) {
        let assign8100_e10072: f64 = (p.p438 * locals.var_dmcgeff);
        let assign8100_e10075: f64 = (locals.var_weff * locals.var_nuendd);
        let assign8100_e10076: f64 = (assign8100_e10072 / assign8100_e10075);
        (assign8100_e10076,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign8100_e10078;
        locals.var_rend_rv = 0.0;

        let assign8120_e10088: f64 = if ((locals.var_nuendd == 0.0) || (locals.var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard183 = assign8120_e10088;
        locals.var_guard183_rv = 0.0;

        let (assign8130_e10116,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard89 != 0.0) && (!(((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0))))) && (locals.var_guard166 == 0.0)) && (locals.var_guard178 != 0.0)) && ((locals.var_guard180 != 0.0) && (locals.var_guard179 == 0.0))) && (locals.var_guard183 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign8130_e10116;
        locals.var_rend_rv = 0.0;

        let (assign8140_e10153,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard89 != 0.0) && (!(((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0))))) && (locals.var_guard166 == 0.0)) && (locals.var_guard178 != 0.0)) && ((locals.var_guard180 != 0.0) && (locals.var_guard179 == 0.0))) && (locals.var_guard183 == 0.0)) {
        let assign8140_e10145: f64 = (p.p438 * locals.var_weff);
        let assign8140_e10148: f64 = (6.0 * locals.var_nuendd);
        let assign8140_e10150: f64 = (assign8140_e10148 * locals.var_dmcgeff);
        let assign8140_e10151: f64 = (assign8140_e10145 / assign8140_e10150);
        (assign8140_e10151,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign8140_e10153;
        locals.var_rend_rv = 0.0;

        let (assign8150_e10179,) = {
    if ((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard89 != 0.0) && (!(((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0))))) && (locals.var_guard166 == 0.0)) && (locals.var_guard178 != 0.0)) && (!((locals.var_guard179 != 0.0) || (locals.var_guard180 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign8150_e10179;
        locals.var_rend_rv = 0.0;

        let assign8160_e10190: f64 = if (((p.p9 == 1.0) || (p.p9 == 3.0)) || (p.p9 == 7.0)) { 1.0 } else { 0.0 };
        locals.var_guard184 = assign8160_e10190;
        locals.var_guard184_rv = 0.0;

        let assign8170_e10201: f64 = if (((p.p9 == 2.0) || (p.p9 == 4.0)) || (p.p9 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard185 = assign8170_e10201;
        locals.var_guard185_rv = 0.0;

        let assign8180_e10204: f64 = if locals.var_nuendd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard186 = assign8180_e10204;
        locals.var_guard186_rv = 0.0;

        let (assign8190_e10230,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard89 != 0.0) && (!(((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0))))) && (locals.var_guard166 == 0.0)) && (locals.var_guard178 == 0.0)) && (locals.var_guard184 != 0.0)) && (locals.var_guard186 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign8190_e10230;
        locals.var_rend_rv = 0.0;

        let (assign8200_e10263,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard89 != 0.0) && (!(((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0))))) && (locals.var_guard166 == 0.0)) && (locals.var_guard178 == 0.0)) && (locals.var_guard184 != 0.0)) && (locals.var_guard186 == 0.0)) {
        let assign8200_e10257: f64 = (p.p438 * locals.var_dmcgeff);
        let assign8200_e10260: f64 = (locals.var_weff * locals.var_nuendd);
        let assign8200_e10261: f64 = (assign8200_e10257 / assign8200_e10260);
        (assign8200_e10261,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign8200_e10263;
        locals.var_rend_rv = 0.0;

        let assign8220_e10273: f64 = if ((locals.var_nuendd == 0.0) || (locals.var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard188 = assign8220_e10273;
        locals.var_guard188_rv = 0.0;

        let (assign8230_e10302,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard89 != 0.0) && (!(((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0))))) && (locals.var_guard166 == 0.0)) && (locals.var_guard178 == 0.0)) && ((locals.var_guard185 != 0.0) && (locals.var_guard184 == 0.0))) && (locals.var_guard188 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign8230_e10302;
        locals.var_rend_rv = 0.0;

        let (assign8240_e10340,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard89 != 0.0) && (!(((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0))))) && (locals.var_guard166 == 0.0)) && (locals.var_guard178 == 0.0)) && ((locals.var_guard185 != 0.0) && (locals.var_guard184 == 0.0))) && (locals.var_guard188 == 0.0)) {
        let assign8240_e10332: f64 = (p.p438 * locals.var_weff);
        let assign8240_e10335: f64 = (6.0 * locals.var_nuendd);
        let assign8240_e10337: f64 = (assign8240_e10335 * locals.var_dmcgeff);
        let assign8240_e10338: f64 = (assign8240_e10332 / assign8240_e10337);
        (assign8240_e10338,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign8240_e10340;
        locals.var_rend_rv = 0.0;

        let (assign8250_e10367,) = {
    if ((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard89 != 0.0) && (!(((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0))))) && (locals.var_guard166 == 0.0)) && (locals.var_guard178 == 0.0)) && (!((locals.var_guard184 != 0.0) || (locals.var_guard185 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign8250_e10367;
        locals.var_rend_rv = 0.0;

        let assign8260_e10370: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard189 = assign8260_e10370;
        locals.var_guard189_rv = 0.0;

        let assign8270_e10373: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard190 = assign8270_e10373;
        locals.var_guard190_rv = 0.0;

        let assign8280_e10384: f64 = if (((p.p9 == 1.0) || (p.p9 == 2.0)) || (p.p9 == 5.0)) { 1.0 } else { 0.0 };
        locals.var_guard191 = assign8280_e10384;
        locals.var_guard191_rv = 0.0;

        let assign8290_e10395: f64 = if (((p.p9 == 3.0) || (p.p9 == 4.0)) || (p.p9 == 6.0)) { 1.0 } else { 0.0 };
        locals.var_guard192 = assign8290_e10395;
        locals.var_guard192_rv = 0.0;

        let assign8300_e10398: f64 = if locals.var_nuends == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard193 = assign8300_e10398;
        locals.var_guard193_rv = 0.0;

        let (assign8310_e10424,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard90 != 0.0) && (!((((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0))))) && (locals.var_guard189 != 0.0)) && (locals.var_guard190 != 0.0)) && (locals.var_guard191 != 0.0)) && (locals.var_guard193 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign8310_e10424;
        locals.var_rend_rv = 0.0;

        let (assign8320_e10457,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard90 != 0.0) && (!((((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0))))) && (locals.var_guard189 != 0.0)) && (locals.var_guard190 != 0.0)) && (locals.var_guard191 != 0.0)) && (locals.var_guard193 == 0.0)) {
        let assign8320_e10451: f64 = (p.p438 * locals.var_dmcgeff);
        let assign8320_e10454: f64 = (locals.var_weff * locals.var_nuends);
        let assign8320_e10455: f64 = (assign8320_e10451 / assign8320_e10454);
        (assign8320_e10455,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign8320_e10457;
        locals.var_rend_rv = 0.0;

        let assign8340_e10468: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign8340_e10471: f64 = if ((locals.var_nuends == 0.0) || (assign8340_e10468 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard195 = assign8340_e10471;
        locals.var_guard195_rv = 0.0;

        let (assign8350_e10500,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard90 != 0.0) && (!((((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0))))) && (locals.var_guard189 != 0.0)) && (locals.var_guard190 != 0.0)) && ((locals.var_guard192 != 0.0) && (locals.var_guard191 == 0.0))) && (locals.var_guard195 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign8350_e10500;
        locals.var_rend_rv = 0.0;

        let (assign8360_e10540,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard90 != 0.0) && (!((((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0))))) && (locals.var_guard189 != 0.0)) && (locals.var_guard190 != 0.0)) && ((locals.var_guard192 != 0.0) && (locals.var_guard191 == 0.0))) && (locals.var_guard195 == 0.0)) {
        let assign8360_e10530: f64 = (p.p438 * locals.var_weff);
        let assign8360_e10533: f64 = (3.0 * locals.var_nuends);
        let assign8360_e10536: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign8360_e10537: f64 = (assign8360_e10533 * assign8360_e10536);
        let assign8360_e10538: f64 = (assign8360_e10530 / assign8360_e10537);
        (assign8360_e10538,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign8360_e10540;
        locals.var_rend_rv = 0.0;

        let (assign8370_e10567,) = {
    if ((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard90 != 0.0) && (!((((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0))))) && (locals.var_guard189 != 0.0)) && (locals.var_guard190 != 0.0)) && (!((locals.var_guard191 != 0.0) || (locals.var_guard192 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign8370_e10567;
        locals.var_rend_rv = 0.0;

        let assign8380_e10578: f64 = if (((p.p9 == 1.0) || (p.p9 == 3.0)) || (p.p9 == 7.0)) { 1.0 } else { 0.0 };
        locals.var_guard196 = assign8380_e10578;
        locals.var_guard196_rv = 0.0;

        let assign8390_e10589: f64 = if (((p.p9 == 2.0) || (p.p9 == 4.0)) || (p.p9 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard197 = assign8390_e10589;
        locals.var_guard197_rv = 0.0;

        let assign8400_e10592: f64 = if locals.var_nuends == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard198 = assign8400_e10592;
        locals.var_guard198_rv = 0.0;

        let (assign8410_e10619,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard90 != 0.0) && (!((((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0))))) && (locals.var_guard189 != 0.0)) && (locals.var_guard190 == 0.0)) && (locals.var_guard196 != 0.0)) && (locals.var_guard198 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign8410_e10619;
        locals.var_rend_rv = 0.0;

        let (assign8420_e10653,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard90 != 0.0) && (!((((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0))))) && (locals.var_guard189 != 0.0)) && (locals.var_guard190 == 0.0)) && (locals.var_guard196 != 0.0)) && (locals.var_guard198 == 0.0)) {
        let assign8420_e10647: f64 = (p.p438 * locals.var_dmcgeff);
        let assign8420_e10650: f64 = (locals.var_weff * locals.var_nuends);
        let assign8420_e10651: f64 = (assign8420_e10647 / assign8420_e10650);
        (assign8420_e10651,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign8420_e10653;
        locals.var_rend_rv = 0.0;

        let assign8440_e10664: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign8440_e10667: f64 = if ((locals.var_nuends == 0.0) || (assign8440_e10664 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard200 = assign8440_e10667;
        locals.var_guard200_rv = 0.0;

        let (assign8450_e10697,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard90 != 0.0) && (!((((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0))))) && (locals.var_guard189 != 0.0)) && (locals.var_guard190 == 0.0)) && ((locals.var_guard197 != 0.0) && (locals.var_guard196 == 0.0))) && (locals.var_guard200 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign8450_e10697;
        locals.var_rend_rv = 0.0;

        let (assign8460_e10738,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard90 != 0.0) && (!((((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0))))) && (locals.var_guard189 != 0.0)) && (locals.var_guard190 == 0.0)) && ((locals.var_guard197 != 0.0) && (locals.var_guard196 == 0.0))) && (locals.var_guard200 == 0.0)) {
        let assign8460_e10728: f64 = (p.p438 * locals.var_weff);
        let assign8460_e10731: f64 = (3.0 * locals.var_nuends);
        let assign8460_e10734: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign8460_e10735: f64 = (assign8460_e10731 * assign8460_e10734);
        let assign8460_e10736: f64 = (assign8460_e10728 / assign8460_e10735);
        (assign8460_e10736,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign8460_e10738;
        locals.var_rend_rv = 0.0;

        let (assign8470_e10766,) = {
    if ((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard90 != 0.0) && (!((((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0))))) && (locals.var_guard189 != 0.0)) && (locals.var_guard190 == 0.0)) && (!((locals.var_guard196 != 0.0) || (locals.var_guard197 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign8470_e10766;
        locals.var_rend_rv = 0.0;

        let (assign8480_e10791,) = {
    if ((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard90 != 0.0) && (!((((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0))))) && (locals.var_guard189 == 0.0)) {
        let assign8480_e10787: f64 = (p.p438 * locals.var_dmdgeff);
        let assign8480_e10789: f64 = (assign8480_e10787 / locals.var_weff);
        (assign8480_e10789,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign8480_e10791;
        locals.var_rend_rv = 0.0;

        let assign8490_e10794: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard201 = assign8490_e10794;
        locals.var_guard201_rv = 0.0;

        let assign8500_e10797: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard202 = assign8500_e10797;
        locals.var_guard202_rv = 0.0;

        let assign8510_e10808: f64 = if (((p.p9 == 1.0) || (p.p9 == 2.0)) || (p.p9 == 5.0)) { 1.0 } else { 0.0 };
        locals.var_guard203 = assign8510_e10808;
        locals.var_guard203_rv = 0.0;

        let assign8520_e10819: f64 = if (((p.p9 == 3.0) || (p.p9 == 4.0)) || (p.p9 == 6.0)) { 1.0 } else { 0.0 };
        locals.var_guard204 = assign8520_e10819;
        locals.var_guard204_rv = 0.0;

        let assign8530_e10822: f64 = if locals.var_nuends == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard205 = assign8530_e10822;
        locals.var_guard205_rv = 0.0;

        let (assign8540_e10850,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard91 != 0.0) && (!(((((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0))))) && (locals.var_guard201 != 0.0)) && (locals.var_guard202 != 0.0)) && (locals.var_guard203 != 0.0)) && (locals.var_guard205 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign8540_e10850;
        locals.var_rend_rv = 0.0;

        let (assign8550_e10885,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard91 != 0.0) && (!(((((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0))))) && (locals.var_guard201 != 0.0)) && (locals.var_guard202 != 0.0)) && (locals.var_guard203 != 0.0)) && (locals.var_guard205 == 0.0)) {
        let assign8550_e10879: f64 = (p.p438 * locals.var_dmcgeff);
        let assign8550_e10882: f64 = (locals.var_weff * locals.var_nuends);
        let assign8550_e10883: f64 = (assign8550_e10879 / assign8550_e10882);
        (assign8550_e10883,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign8550_e10885;
        locals.var_rend_rv = 0.0;

        let assign8570_e10895: f64 = if ((locals.var_nuends == 0.0) || (locals.var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard207 = assign8570_e10895;
        locals.var_guard207_rv = 0.0;

        let (assign8580_e10926,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard91 != 0.0) && (!(((((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0))))) && (locals.var_guard201 != 0.0)) && (locals.var_guard202 != 0.0)) && ((locals.var_guard204 != 0.0) && (locals.var_guard203 == 0.0))) && (locals.var_guard207 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign8580_e10926;
        locals.var_rend_rv = 0.0;

        let (assign8590_e10966,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard91 != 0.0) && (!(((((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0))))) && (locals.var_guard201 != 0.0)) && (locals.var_guard202 != 0.0)) && ((locals.var_guard204 != 0.0) && (locals.var_guard203 == 0.0))) && (locals.var_guard207 == 0.0)) {
        let assign8590_e10958: f64 = (p.p438 * locals.var_weff);
        let assign8590_e10961: f64 = (6.0 * locals.var_nuends);
        let assign8590_e10963: f64 = (assign8590_e10961 * locals.var_dmcgeff);
        let assign8590_e10964: f64 = (assign8590_e10958 / assign8590_e10963);
        (assign8590_e10964,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign8590_e10966;
        locals.var_rend_rv = 0.0;

        let (assign8600_e10995,) = {
    if ((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard91 != 0.0) && (!(((((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0))))) && (locals.var_guard201 != 0.0)) && (locals.var_guard202 != 0.0)) && (!((locals.var_guard203 != 0.0) || (locals.var_guard204 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign8600_e10995;
        locals.var_rend_rv = 0.0;

        let assign8610_e11006: f64 = if (((p.p9 == 1.0) || (p.p9 == 3.0)) || (p.p9 == 7.0)) { 1.0 } else { 0.0 };
        locals.var_guard208 = assign8610_e11006;
        locals.var_guard208_rv = 0.0;

        let assign8620_e11017: f64 = if (((p.p9 == 2.0) || (p.p9 == 4.0)) || (p.p9 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard209 = assign8620_e11017;
        locals.var_guard209_rv = 0.0;

        let assign8630_e11020: f64 = if locals.var_nuends == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard210 = assign8630_e11020;
        locals.var_guard210_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_15(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign8640_e11049,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard91 != 0.0) && (!(((((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0))))) && (locals.var_guard201 != 0.0)) && (locals.var_guard202 == 0.0)) && (locals.var_guard208 != 0.0)) && (locals.var_guard210 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign8640_e11049;
        locals.var_rend_rv = 0.0;

        let (assign8650_e11085,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard91 != 0.0) && (!(((((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0))))) && (locals.var_guard201 != 0.0)) && (locals.var_guard202 == 0.0)) && (locals.var_guard208 != 0.0)) && (locals.var_guard210 == 0.0)) {
        let assign8650_e11079: f64 = (p.p438 * locals.var_dmcgeff);
        let assign8650_e11082: f64 = (locals.var_weff * locals.var_nuends);
        let assign8650_e11083: f64 = (assign8650_e11079 / assign8650_e11082);
        (assign8650_e11083,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign8650_e11085;
        locals.var_rend_rv = 0.0;

        let assign8670_e11095: f64 = if ((locals.var_nuends == 0.0) || (locals.var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard212 = assign8670_e11095;
        locals.var_guard212_rv = 0.0;

        let (assign8680_e11127,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard91 != 0.0) && (!(((((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0))))) && (locals.var_guard201 != 0.0)) && (locals.var_guard202 == 0.0)) && ((locals.var_guard209 != 0.0) && (locals.var_guard208 == 0.0))) && (locals.var_guard212 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign8680_e11127;
        locals.var_rend_rv = 0.0;

        let (assign8690_e11168,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard91 != 0.0) && (!(((((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0))))) && (locals.var_guard201 != 0.0)) && (locals.var_guard202 == 0.0)) && ((locals.var_guard209 != 0.0) && (locals.var_guard208 == 0.0))) && (locals.var_guard212 == 0.0)) {
        let assign8690_e11160: f64 = (p.p438 * locals.var_weff);
        let assign8690_e11163: f64 = (6.0 * locals.var_nuends);
        let assign8690_e11165: f64 = (assign8690_e11163 * locals.var_dmcgeff);
        let assign8690_e11166: f64 = (assign8690_e11160 / assign8690_e11165);
        (assign8690_e11166,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign8690_e11168;
        locals.var_rend_rv = 0.0;

        let (assign8700_e11198,) = {
    if ((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard91 != 0.0) && (!(((((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0))))) && (locals.var_guard201 != 0.0)) && (locals.var_guard202 == 0.0)) && (!((locals.var_guard208 != 0.0) || (locals.var_guard209 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign8700_e11198;
        locals.var_rend_rv = 0.0;

        let assign8710_e11201: f64 = if locals.var_nuendd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard213 = assign8710_e11201;
        locals.var_guard213_rv = 0.0;

        let (assign8720_e11226,) = {
    if (((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard91 != 0.0) && (!(((((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0))))) && (locals.var_guard201 == 0.0)) && (locals.var_guard213 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign8720_e11226;
        locals.var_rend_rv = 0.0;

        let (assign8730_e11258,) = {
    if (((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard91 != 0.0) && (!(((((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0))))) && (locals.var_guard201 == 0.0)) && (locals.var_guard213 == 0.0)) {
        let assign8730_e11252: f64 = (p.p438 * locals.var_dmdgeff);
        let assign8730_e11255: f64 = (locals.var_weff * locals.var_nuendd);
        let assign8730_e11256: f64 = (assign8730_e11252 / assign8730_e11255);
        (assign8730_e11256,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign8730_e11258;
        locals.var_rend_rv = 0.0;

        let assign8740_e11261: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard214 = assign8740_e11261;
        locals.var_guard214_rv = 0.0;

        let (assign8750_e11289,) = {
    if ((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard92 != 0.0) && (!((((((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0)) || (locals.var_guard91 != 0.0))))) && (locals.var_guard214 != 0.0)) {
        let assign8750_e11285: f64 = (p.p438 * locals.var_dmdgeff);
        let assign8750_e11287: f64 = (assign8750_e11285 / locals.var_weff);
        (assign8750_e11287,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign8750_e11289;
        locals.var_rend_rv = 0.0;

        let assign8760_e11292: f64 = if 0.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard215 = assign8760_e11292;
        locals.var_guard215_rv = 0.0;

        let assign8770_e11303: f64 = if (((p.p9 == 1.0) || (p.p9 == 2.0)) || (p.p9 == 5.0)) { 1.0 } else { 0.0 };
        locals.var_guard216 = assign8770_e11303;
        locals.var_guard216_rv = 0.0;

        let assign8780_e11314: f64 = if (((p.p9 == 3.0) || (p.p9 == 4.0)) || (p.p9 == 6.0)) { 1.0 } else { 0.0 };
        locals.var_guard217 = assign8780_e11314;
        locals.var_guard217_rv = 0.0;

        let assign8790_e11317: f64 = if locals.var_nuendd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard218 = assign8790_e11317;
        locals.var_guard218_rv = 0.0;

        let (assign8800_e11348,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard92 != 0.0) && (!((((((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0)) || (locals.var_guard91 != 0.0))))) && (locals.var_guard214 == 0.0)) && (locals.var_guard215 != 0.0)) && (locals.var_guard216 != 0.0)) && (locals.var_guard218 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign8800_e11348;
        locals.var_rend_rv = 0.0;

        let (assign8810_e11386,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard92 != 0.0) && (!((((((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0)) || (locals.var_guard91 != 0.0))))) && (locals.var_guard214 == 0.0)) && (locals.var_guard215 != 0.0)) && (locals.var_guard216 != 0.0)) && (locals.var_guard218 == 0.0)) {
        let assign8810_e11380: f64 = (p.p438 * locals.var_dmcgeff);
        let assign8810_e11383: f64 = (locals.var_weff * locals.var_nuendd);
        let assign8810_e11384: f64 = (assign8810_e11380 / assign8810_e11383);
        (assign8810_e11384,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign8810_e11386;
        locals.var_rend_rv = 0.0;

        let assign8830_e11397: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign8830_e11400: f64 = if ((locals.var_nuendd == 0.0) || (assign8830_e11397 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard220 = assign8830_e11400;
        locals.var_guard220_rv = 0.0;

        let (assign8840_e11434,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard92 != 0.0) && (!((((((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0)) || (locals.var_guard91 != 0.0))))) && (locals.var_guard214 == 0.0)) && (locals.var_guard215 != 0.0)) && ((locals.var_guard217 != 0.0) && (locals.var_guard216 == 0.0))) && (locals.var_guard220 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign8840_e11434;
        locals.var_rend_rv = 0.0;

        let (assign8850_e11479,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard92 != 0.0) && (!((((((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0)) || (locals.var_guard91 != 0.0))))) && (locals.var_guard214 == 0.0)) && (locals.var_guard215 != 0.0)) && ((locals.var_guard217 != 0.0) && (locals.var_guard216 == 0.0))) && (locals.var_guard220 == 0.0)) {
        let assign8850_e11469: f64 = (p.p438 * locals.var_weff);
        let assign8850_e11472: f64 = (3.0 * locals.var_nuendd);
        let assign8850_e11475: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign8850_e11476: f64 = (assign8850_e11472 * assign8850_e11475);
        let assign8850_e11477: f64 = (assign8850_e11469 / assign8850_e11476);
        (assign8850_e11477,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign8850_e11479;
        locals.var_rend_rv = 0.0;

        let (assign8860_e11511,) = {
    if ((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard92 != 0.0) && (!((((((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0)) || (locals.var_guard91 != 0.0))))) && (locals.var_guard214 == 0.0)) && (locals.var_guard215 != 0.0)) && (!((locals.var_guard216 != 0.0) || (locals.var_guard217 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign8860_e11511;
        locals.var_rend_rv = 0.0;

        let assign8870_e11522: f64 = if (((p.p9 == 1.0) || (p.p9 == 3.0)) || (p.p9 == 7.0)) { 1.0 } else { 0.0 };
        locals.var_guard221 = assign8870_e11522;
        locals.var_guard221_rv = 0.0;

        let assign8880_e11533: f64 = if (((p.p9 == 2.0) || (p.p9 == 4.0)) || (p.p9 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard222 = assign8880_e11533;
        locals.var_guard222_rv = 0.0;

        let assign8890_e11536: f64 = if locals.var_nuendd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard223 = assign8890_e11536;
        locals.var_guard223_rv = 0.0;

        let (assign8900_e11568,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard92 != 0.0) && (!((((((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0)) || (locals.var_guard91 != 0.0))))) && (locals.var_guard214 == 0.0)) && (locals.var_guard215 == 0.0)) && (locals.var_guard221 != 0.0)) && (locals.var_guard223 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign8900_e11568;
        locals.var_rend_rv = 0.0;

        let (assign8910_e11607,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard92 != 0.0) && (!((((((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0)) || (locals.var_guard91 != 0.0))))) && (locals.var_guard214 == 0.0)) && (locals.var_guard215 == 0.0)) && (locals.var_guard221 != 0.0)) && (locals.var_guard223 == 0.0)) {
        let assign8910_e11601: f64 = (p.p438 * locals.var_dmcgeff);
        let assign8910_e11604: f64 = (locals.var_weff * locals.var_nuendd);
        let assign8910_e11605: f64 = (assign8910_e11601 / assign8910_e11604);
        (assign8910_e11605,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign8910_e11607;
        locals.var_rend_rv = 0.0;

        let assign8930_e11618: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign8930_e11621: f64 = if ((locals.var_nuendd == 0.0) || (assign8930_e11618 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard225 = assign8930_e11621;
        locals.var_guard225_rv = 0.0;

        let (assign8940_e11656,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard92 != 0.0) && (!((((((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0)) || (locals.var_guard91 != 0.0))))) && (locals.var_guard214 == 0.0)) && (locals.var_guard215 == 0.0)) && ((locals.var_guard222 != 0.0) && (locals.var_guard221 == 0.0))) && (locals.var_guard225 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign8940_e11656;
        locals.var_rend_rv = 0.0;

        let (assign8950_e11702,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard92 != 0.0) && (!((((((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0)) || (locals.var_guard91 != 0.0))))) && (locals.var_guard214 == 0.0)) && (locals.var_guard215 == 0.0)) && ((locals.var_guard222 != 0.0) && (locals.var_guard221 == 0.0))) && (locals.var_guard225 == 0.0)) {
        let assign8950_e11692: f64 = (p.p438 * locals.var_weff);
        let assign8950_e11695: f64 = (3.0 * locals.var_nuendd);
        let assign8950_e11698: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign8950_e11699: f64 = (assign8950_e11695 * assign8950_e11698);
        let assign8950_e11700: f64 = (assign8950_e11692 / assign8950_e11699);
        (assign8950_e11700,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign8950_e11702;
        locals.var_rend_rv = 0.0;

        let (assign8960_e11735,) = {
    if ((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard92 != 0.0) && (!((((((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0)) || (locals.var_guard91 != 0.0))))) && (locals.var_guard214 == 0.0)) && (locals.var_guard215 == 0.0)) && (!((locals.var_guard221 != 0.0) || (locals.var_guard222 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign8960_e11735;
        locals.var_rend_rv = 0.0;

        let assign8970_e11738: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard226 = assign8970_e11738;
        locals.var_guard226_rv = 0.0;

        let assign8980_e11741: f64 = if locals.var_nuends == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard227 = assign8980_e11741;
        locals.var_guard227_rv = 0.0;

        let (assign8990_e11769,) = {
    if (((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard93 != 0.0) && (!(((((((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0)) || (locals.var_guard91 != 0.0)) || (locals.var_guard92 != 0.0))))) && (locals.var_guard226 != 0.0)) && (locals.var_guard227 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign8990_e11769;
        locals.var_rend_rv = 0.0;

        let (assign9000_e11804,) = {
    if (((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard93 != 0.0) && (!(((((((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0)) || (locals.var_guard91 != 0.0)) || (locals.var_guard92 != 0.0))))) && (locals.var_guard226 != 0.0)) && (locals.var_guard227 == 0.0)) {
        let assign9000_e11798: f64 = (p.p438 * locals.var_dmdgeff);
        let assign9000_e11801: f64 = (locals.var_weff * locals.var_nuends);
        let assign9000_e11802: f64 = (assign9000_e11798 / assign9000_e11801);
        (assign9000_e11802,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign9000_e11804;
        locals.var_rend_rv = 0.0;

        let assign9010_e11807: f64 = if 0.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard228 = assign9010_e11807;
        locals.var_guard228_rv = 0.0;

        let assign9020_e11818: f64 = if (((p.p9 == 1.0) || (p.p9 == 2.0)) || (p.p9 == 5.0)) { 1.0 } else { 0.0 };
        locals.var_guard229 = assign9020_e11818;
        locals.var_guard229_rv = 0.0;

        let assign9030_e11829: f64 = if (((p.p9 == 3.0) || (p.p9 == 4.0)) || (p.p9 == 6.0)) { 1.0 } else { 0.0 };
        locals.var_guard230 = assign9030_e11829;
        locals.var_guard230_rv = 0.0;

        let assign9040_e11832: f64 = if locals.var_nuendd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard231 = assign9040_e11832;
        locals.var_guard231_rv = 0.0;

        let (assign9050_e11865,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard93 != 0.0) && (!(((((((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0)) || (locals.var_guard91 != 0.0)) || (locals.var_guard92 != 0.0))))) && (locals.var_guard226 == 0.0)) && (locals.var_guard228 != 0.0)) && (locals.var_guard229 != 0.0)) && (locals.var_guard231 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign9050_e11865;
        locals.var_rend_rv = 0.0;

        let (assign9060_e11905,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard93 != 0.0) && (!(((((((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0)) || (locals.var_guard91 != 0.0)) || (locals.var_guard92 != 0.0))))) && (locals.var_guard226 == 0.0)) && (locals.var_guard228 != 0.0)) && (locals.var_guard229 != 0.0)) && (locals.var_guard231 == 0.0)) {
        let assign9060_e11899: f64 = (p.p438 * locals.var_dmcgeff);
        let assign9060_e11902: f64 = (locals.var_weff * locals.var_nuendd);
        let assign9060_e11903: f64 = (assign9060_e11899 / assign9060_e11902);
        (assign9060_e11903,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign9060_e11905;
        locals.var_rend_rv = 0.0;

        let assign9080_e11915: f64 = if ((locals.var_nuendd == 0.0) || (locals.var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard233 = assign9080_e11915;
        locals.var_guard233_rv = 0.0;

        let (assign9090_e11951,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard93 != 0.0) && (!(((((((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0)) || (locals.var_guard91 != 0.0)) || (locals.var_guard92 != 0.0))))) && (locals.var_guard226 == 0.0)) && (locals.var_guard228 != 0.0)) && ((locals.var_guard230 != 0.0) && (locals.var_guard229 == 0.0))) && (locals.var_guard233 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign9090_e11951;
        locals.var_rend_rv = 0.0;

        let (assign9100_e11996,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard93 != 0.0) && (!(((((((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0)) || (locals.var_guard91 != 0.0)) || (locals.var_guard92 != 0.0))))) && (locals.var_guard226 == 0.0)) && (locals.var_guard228 != 0.0)) && ((locals.var_guard230 != 0.0) && (locals.var_guard229 == 0.0))) && (locals.var_guard233 == 0.0)) {
        let assign9100_e11988: f64 = (p.p438 * locals.var_weff);
        let assign9100_e11991: f64 = (6.0 * locals.var_nuendd);
        let assign9100_e11993: f64 = (assign9100_e11991 * locals.var_dmcgeff);
        let assign9100_e11994: f64 = (assign9100_e11988 / assign9100_e11993);
        (assign9100_e11994,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign9100_e11996;
        locals.var_rend_rv = 0.0;

        let (assign9110_e12030,) = {
    if ((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard93 != 0.0) && (!(((((((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0)) || (locals.var_guard91 != 0.0)) || (locals.var_guard92 != 0.0))))) && (locals.var_guard226 == 0.0)) && (locals.var_guard228 != 0.0)) && (!((locals.var_guard229 != 0.0) || (locals.var_guard230 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign9110_e12030;
        locals.var_rend_rv = 0.0;

        let assign9120_e12041: f64 = if (((p.p9 == 1.0) || (p.p9 == 3.0)) || (p.p9 == 7.0)) { 1.0 } else { 0.0 };
        locals.var_guard234 = assign9120_e12041;
        locals.var_guard234_rv = 0.0;

        let assign9130_e12052: f64 = if (((p.p9 == 2.0) || (p.p9 == 4.0)) || (p.p9 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard235 = assign9130_e12052;
        locals.var_guard235_rv = 0.0;

        let assign9140_e12055: f64 = if locals.var_nuendd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard236 = assign9140_e12055;
        locals.var_guard236_rv = 0.0;

        let (assign9150_e12089,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard93 != 0.0) && (!(((((((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0)) || (locals.var_guard91 != 0.0)) || (locals.var_guard92 != 0.0))))) && (locals.var_guard226 == 0.0)) && (locals.var_guard228 == 0.0)) && (locals.var_guard234 != 0.0)) && (locals.var_guard236 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign9150_e12089;
        locals.var_rend_rv = 0.0;

        let (assign9160_e12130,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard93 != 0.0) && (!(((((((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0)) || (locals.var_guard91 != 0.0)) || (locals.var_guard92 != 0.0))))) && (locals.var_guard226 == 0.0)) && (locals.var_guard228 == 0.0)) && (locals.var_guard234 != 0.0)) && (locals.var_guard236 == 0.0)) {
        let assign9160_e12124: f64 = (p.p438 * locals.var_dmcgeff);
        let assign9160_e12127: f64 = (locals.var_weff * locals.var_nuendd);
        let assign9160_e12128: f64 = (assign9160_e12124 / assign9160_e12127);
        (assign9160_e12128,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign9160_e12130;
        locals.var_rend_rv = 0.0;

        let assign9180_e12140: f64 = if ((locals.var_nuendd == 0.0) || (locals.var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard238 = assign9180_e12140;
        locals.var_guard238_rv = 0.0;

        let (assign9190_e12177,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard93 != 0.0) && (!(((((((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0)) || (locals.var_guard91 != 0.0)) || (locals.var_guard92 != 0.0))))) && (locals.var_guard226 == 0.0)) && (locals.var_guard228 == 0.0)) && ((locals.var_guard235 != 0.0) && (locals.var_guard234 == 0.0))) && (locals.var_guard238 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign9190_e12177;
        locals.var_rend_rv = 0.0;

        let (assign9200_e12223,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard93 != 0.0) && (!(((((((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0)) || (locals.var_guard91 != 0.0)) || (locals.var_guard92 != 0.0))))) && (locals.var_guard226 == 0.0)) && (locals.var_guard228 == 0.0)) && ((locals.var_guard235 != 0.0) && (locals.var_guard234 == 0.0))) && (locals.var_guard238 == 0.0)) {
        let assign9200_e12215: f64 = (p.p438 * locals.var_weff);
        let assign9200_e12218: f64 = (6.0 * locals.var_nuendd);
        let assign9200_e12220: f64 = (assign9200_e12218 * locals.var_dmcgeff);
        let assign9200_e12221: f64 = (assign9200_e12215 / assign9200_e12220);
        (assign9200_e12221,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign9200_e12223;
        locals.var_rend_rv = 0.0;

        let (assign9210_e12258,) = {
    if ((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard93 != 0.0) && (!(((((((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0)) || (locals.var_guard91 != 0.0)) || (locals.var_guard92 != 0.0))))) && (locals.var_guard226 == 0.0)) && (locals.var_guard228 == 0.0)) && (!((locals.var_guard234 != 0.0) || (locals.var_guard235 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign9210_e12258;
        locals.var_rend_rv = 0.0;

        let (assign9220_e12288,) = {
    if (((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard94 != 0.0) && (!((((((((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0)) || (locals.var_guard91 != 0.0)) || (locals.var_guard92 != 0.0)) || (locals.var_guard93 != 0.0))))) {
        let assign9220_e12284: f64 = (p.p438 * locals.var_dmdgeff);
        let assign9220_e12286: f64 = (assign9220_e12284 / locals.var_weff);
        (assign9220_e12286,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign9220_e12288;
        locals.var_rend_rv = 0.0;

        let assign9230_e12291: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard239 = assign9230_e12291;
        locals.var_guard239_rv = 0.0;

        let (assign9240_e12327,) = {
    if ((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard95 != 0.0) && (!(((((((((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0)) || (locals.var_guard91 != 0.0)) || (locals.var_guard92 != 0.0)) || (locals.var_guard93 != 0.0)) || (locals.var_guard94 != 0.0))))) && (locals.var_guard239 != 0.0)) {
        let assign9240_e12321: f64 = (0.5 * p.p438);
        let assign9240_e12323: f64 = (assign9240_e12321 * locals.var_dmcgeff);
        let assign9240_e12325: f64 = (assign9240_e12323 / locals.var_weff);
        (assign9240_e12325,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign9240_e12327;
        locals.var_rend_rv = 0.0;

        let assign9250_e12330: f64 = if p.p2 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard240 = assign9250_e12330;
        locals.var_guard240_rv = 0.0;

        let (assign9260_e12362,) = {
    if (((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard95 != 0.0) && (!(((((((((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0)) || (locals.var_guard91 != 0.0)) || (locals.var_guard92 != 0.0)) || (locals.var_guard93 != 0.0)) || (locals.var_guard94 != 0.0))))) && (locals.var_guard239 != 0.0)) && (locals.var_guard240 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rint,)
    }
};
        locals.var_rint = assign9260_e12362;
        locals.var_rint_rv = 0.0;

        let (assign9270_e12403,) = {
    if (((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard95 != 0.0) && (!(((((((((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0)) || (locals.var_guard91 != 0.0)) || (locals.var_guard92 != 0.0)) || (locals.var_guard93 != 0.0)) || (locals.var_guard94 != 0.0))))) && (locals.var_guard239 != 0.0)) && (locals.var_guard240 == 0.0)) {
        let assign9270_e12395: f64 = (p.p438 * locals.var_dmcgeff);
        let assign9270_e12399: f64 = (p.p2 - 2.0);
        let assign9270_e12400: f64 = (locals.var_weff * assign9270_e12399);
        let assign9270_e12401: f64 = (assign9270_e12395 / assign9270_e12400);
        (assign9270_e12401,)
    } else {
        (locals.var_rint,)
    }
};
        locals.var_rint = assign9270_e12403;
        locals.var_rint_rv = 0.0;

        let (assign9280_e12434,) = {
    if ((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard95 != 0.0) && (!(((((((((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0)) || (locals.var_guard91 != 0.0)) || (locals.var_guard92 != 0.0)) || (locals.var_guard93 != 0.0)) || (locals.var_guard94 != 0.0))))) && (locals.var_guard239 == 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign9280_e12434;
        locals.var_rend_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_16(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        locals: &mut StampLocals,
    ) {
        let (assign9290_e12471,) = {
    if ((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard95 != 0.0) && (!(((((((((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0)) || (locals.var_guard91 != 0.0)) || (locals.var_guard92 != 0.0)) || (locals.var_guard93 != 0.0)) || (locals.var_guard94 != 0.0))))) && (locals.var_guard239 == 0.0)) {
        let assign9290_e12465: f64 = (p.p438 * locals.var_dmcgeff);
        let assign9290_e12468: f64 = (locals.var_weff * p.p2);
        let assign9290_e12469: f64 = (assign9290_e12465 / assign9290_e12468);
        (assign9290_e12469,)
    } else {
        (locals.var_rint,)
    }
};
        locals.var_rint = assign9290_e12471;
        locals.var_rint_rv = 0.0;

        let assign9300_e12474: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard241 = assign9300_e12474;
        locals.var_guard241_rv = 0.0;

        let (assign9310_e12506,) = {
    if ((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard96 != 0.0) && (!((((((((((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0)) || (locals.var_guard91 != 0.0)) || (locals.var_guard92 != 0.0)) || (locals.var_guard93 != 0.0)) || (locals.var_guard94 != 0.0)) || (locals.var_guard95 != 0.0))))) && (locals.var_guard241 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign9310_e12506;
        locals.var_rend_rv = 0.0;

        let (assign9320_e12544,) = {
    if ((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard96 != 0.0) && (!((((((((((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0)) || (locals.var_guard91 != 0.0)) || (locals.var_guard92 != 0.0)) || (locals.var_guard93 != 0.0)) || (locals.var_guard94 != 0.0)) || (locals.var_guard95 != 0.0))))) && (locals.var_guard241 != 0.0)) {
        let assign9320_e12538: f64 = (p.p438 * locals.var_dmcgeff);
        let assign9320_e12541: f64 = (locals.var_weff * p.p2);
        let assign9320_e12542: f64 = (assign9320_e12538 / assign9320_e12541);
        (assign9320_e12542,)
    } else {
        (locals.var_rint,)
    }
};
        locals.var_rint = assign9320_e12544;
        locals.var_rint_rv = 0.0;

        let (assign9330_e12583,) = {
    if ((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard96 != 0.0) && (!((((((((((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0)) || (locals.var_guard91 != 0.0)) || (locals.var_guard92 != 0.0)) || (locals.var_guard93 != 0.0)) || (locals.var_guard94 != 0.0)) || (locals.var_guard95 != 0.0))))) && (locals.var_guard241 == 0.0)) {
        let assign9330_e12577: f64 = (0.5 * p.p438);
        let assign9330_e12579: f64 = (assign9330_e12577 * locals.var_dmcgeff);
        let assign9330_e12581: f64 = (assign9330_e12579 / locals.var_weff);
        (assign9330_e12581,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign9330_e12583;
        locals.var_rend_rv = 0.0;

        let assign9340_e12586: f64 = if p.p2 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard242 = assign9340_e12586;
        locals.var_guard242_rv = 0.0;

        let (assign9350_e12621,) = {
    if (((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard96 != 0.0) && (!((((((((((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0)) || (locals.var_guard91 != 0.0)) || (locals.var_guard92 != 0.0)) || (locals.var_guard93 != 0.0)) || (locals.var_guard94 != 0.0)) || (locals.var_guard95 != 0.0))))) && (locals.var_guard241 == 0.0)) && (locals.var_guard242 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rint,)
    }
};
        locals.var_rint = assign9350_e12621;
        locals.var_rint_rv = 0.0;

        let (assign9360_e12665,) = {
    if (((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard96 != 0.0) && (!((((((((((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0)) || (locals.var_guard91 != 0.0)) || (locals.var_guard92 != 0.0)) || (locals.var_guard93 != 0.0)) || (locals.var_guard94 != 0.0)) || (locals.var_guard95 != 0.0))))) && (locals.var_guard241 == 0.0)) && (locals.var_guard242 == 0.0)) {
        let assign9360_e12657: f64 = (p.p438 * locals.var_dmcgeff);
        let assign9360_e12661: f64 = (p.p2 - 2.0);
        let assign9360_e12662: f64 = (locals.var_weff * assign9360_e12661);
        let assign9360_e12663: f64 = (assign9360_e12657 / assign9360_e12662);
        (assign9360_e12663,)
    } else {
        (locals.var_rint,)
    }
};
        locals.var_rint = assign9360_e12665;
        locals.var_rint_rv = 0.0;

        let (assign9370_e12695,) = {
    if (((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && (!(((((((((((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0)) || (locals.var_guard91 != 0.0)) || (locals.var_guard92 != 0.0)) || (locals.var_guard93 != 0.0)) || (locals.var_guard94 != 0.0)) || (locals.var_guard95 != 0.0)) || (locals.var_guard96 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rint,)
    }
};
        locals.var_rint = assign9370_e12695;
        locals.var_rint_rv = 0.0;

        let assign9380_e12698: f64 = if locals.var_rint <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard243 = assign9380_e12698;
        locals.var_guard243_rv = 0.0;

        let (assign9390_e12707,) = {
    if (((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && (locals.var_guard243 != 0.0)) {
        (locals.var_rend,)
    } else {
        (locals.var_rsourcegeo,)
    }
};
        locals.var_rsourcegeo = assign9390_e12707;
        locals.var_rsourcegeo_rv = 0.0;

        let assign9400_e12710: f64 = if locals.var_rend <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard244 = assign9400_e12710;
        locals.var_guard244_rv = 0.0;

        let (assign9410_e12722,) = {
    if ((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && (locals.var_guard243 == 0.0)) && (locals.var_guard244 != 0.0)) {
        (locals.var_rint,)
    } else {
        (locals.var_rsourcegeo,)
    }
};
        locals.var_rsourcegeo = assign9410_e12722;
        locals.var_rsourcegeo_rv = 0.0;

        let (assign9420_e12741,) = {
    if ((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && (locals.var_guard243 == 0.0)) && (locals.var_guard244 == 0.0)) {
        let assign9420_e12735: f64 = (locals.var_rint * locals.var_rend);
        let assign9420_e12738: f64 = (locals.var_rint + locals.var_rend);
        let assign9420_e12739: f64 = (assign9420_e12735 / assign9420_e12738);
        (assign9420_e12739,)
    } else {
        (locals.var_rsourcegeo,)
    }
};
        locals.var_rsourcegeo = assign9420_e12741;
        locals.var_rsourcegeo_rv = 0.0;

        let (assign9440_e12752,) = {
    if ((locals.var_guard78 == 0.0) && (locals.var_guard79 == 0.0)) {
        (0.0,)
    } else {
        (locals.var_rsourcegeo,)
    }
};
        locals.var_rsourcegeo = assign9440_e12752;
        locals.var_rsourcegeo_rv = 0.0;

        let assign9450_e12754: f64 = if param_given[4] { 1.0 } else { 0.0 };
        locals.var_guard246 = assign9450_e12754;
        locals.var_guard246_rv = 0.0;

        let (assign9460_e12760,) = {
    if (locals.var_guard246 != 0.0) {
        let assign9460_e12758: f64 = (p.p438 * p.p4);
        (assign9460_e12758,)
    } else {
        (locals.var_rdraingeo,)
    }
};
        locals.var_rdraingeo = assign9460_e12760;
        locals.var_rdraingeo_rv = 0.0;

        let assign9470_e12767: f64 = if ((p.p9 > 0.0) && (p.p438 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard247 = assign9470_e12767;
        locals.var_guard247_rv = 0.0;

        let assign9480_e12770: f64 = if p.p8 < 9.0 { 1.0 } else { 0.0 };
        locals.var_guard248 = assign9480_e12770;
        locals.var_guard248_rv = 0.0;

        let assign9490_e12773: f64 = (p.p2 % 2.0);
        let assign9490_e12775: f64 = if assign9490_e12773 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard249 = assign9490_e12775;
        locals.var_guard249_rv = 0.0;

        let (assign9500_e12786,) = {
    if ((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && (locals.var_guard248 != 0.0)) && (locals.var_guard249 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_nuendd,)
    }
};
        locals.var_nuendd = assign9500_e12786;
        locals.var_nuendd_rv = 0.0;

        let (assign9510_e12797,) = {
    if ((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && (locals.var_guard248 != 0.0)) && (locals.var_guard249 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_nuends,)
    }
};
        locals.var_nuends = assign9510_e12797;
        locals.var_nuends_rv = 0.0;

        let (assign9520_e12816,) = {
    if ((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && (locals.var_guard248 != 0.0)) && (locals.var_guard249 != 0.0)) {
        let assign9520_e12809: f64 = (p.p2 - 1.0);
        let assign9520_e12811: f64 = (assign9520_e12809 / 2.0);
        let assign9520_e12813: f64 = (assign9520_e12811).max(0.0);
        let assign9520_e12814: f64 = (2.0 * assign9520_e12813);
        (assign9520_e12814,)
    } else {
        (locals.var_nuintd,)
    }
};
        locals.var_nuintd = assign9520_e12816;
        locals.var_nuintd_rv = 0.0;

        let (assign9530_e12827,) = {
    if ((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && (locals.var_guard248 != 0.0)) && (locals.var_guard249 != 0.0)) {
        (locals.var_nuintd,)
    } else {
        (locals.var_nuints,)
    }
};
        locals.var_nuints = assign9530_e12827;
        locals.var_nuints_rv = 0.0;

        let assign9540_e12830: f64 = if p.p6 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard250 = assign9540_e12830;
        locals.var_guard250_rv = 0.0;

        let (assign9550_e12844,) = {
    if (((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && (locals.var_guard248 != 0.0)) && (locals.var_guard249 == 0.0)) && (locals.var_guard250 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_nuendd,)
    }
};
        locals.var_nuendd = assign9550_e12844;
        locals.var_nuendd_rv = 0.0;

        let (assign9560_e12866,) = {
    if (((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && (locals.var_guard248 != 0.0)) && (locals.var_guard249 == 0.0)) && (locals.var_guard250 != 0.0)) {
        let assign9560_e12859: f64 = (p.p2 / 2.0);
        let assign9560_e12861: f64 = (assign9560_e12859 - 1.0);
        let assign9560_e12863: f64 = (assign9560_e12861).max(0.0);
        let assign9560_e12864: f64 = (2.0 * assign9560_e12863);
        (assign9560_e12864,)
    } else {
        (locals.var_nuintd,)
    }
};
        locals.var_nuintd = assign9560_e12866;
        locals.var_nuintd_rv = 0.0;

        let (assign9570_e12880,) = {
    if (((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && (locals.var_guard248 != 0.0)) && (locals.var_guard249 == 0.0)) && (locals.var_guard250 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_nuends,)
    }
};
        locals.var_nuends = assign9570_e12880;
        locals.var_nuends_rv = 0.0;

        let (assign9580_e12894,) = {
    if (((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && (locals.var_guard248 != 0.0)) && (locals.var_guard249 == 0.0)) && (locals.var_guard250 != 0.0)) {
        (p.p2,)
    } else {
        (locals.var_nuints,)
    }
};
        locals.var_nuints = assign9580_e12894;
        locals.var_nuints_rv = 0.0;

        let (assign9590_e12909,) = {
    if (((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && (locals.var_guard248 != 0.0)) && (locals.var_guard249 == 0.0)) && (locals.var_guard250 == 0.0)) {
        (0.0,)
    } else {
        (locals.var_nuendd,)
    }
};
        locals.var_nuendd = assign9590_e12909;
        locals.var_nuendd_rv = 0.0;

        let (assign9600_e12924,) = {
    if (((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && (locals.var_guard248 != 0.0)) && (locals.var_guard249 == 0.0)) && (locals.var_guard250 == 0.0)) {
        (p.p2,)
    } else {
        (locals.var_nuintd,)
    }
};
        locals.var_nuintd = assign9600_e12924;
        locals.var_nuintd_rv = 0.0;

        let (assign9610_e12939,) = {
    if (((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && (locals.var_guard248 != 0.0)) && (locals.var_guard249 == 0.0)) && (locals.var_guard250 == 0.0)) {
        (2.0,)
    } else {
        (locals.var_nuends,)
    }
};
        locals.var_nuends = assign9610_e12939;
        locals.var_nuends_rv = 0.0;

        let (assign9620_e12962,) = {
    if (((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && (locals.var_guard248 != 0.0)) && (locals.var_guard249 == 0.0)) && (locals.var_guard250 == 0.0)) {
        let assign9620_e12955: f64 = (p.p2 / 2.0);
        let assign9620_e12957: f64 = (assign9620_e12955 - 1.0);
        let assign9620_e12959: f64 = (assign9620_e12957).max(0.0);
        let assign9620_e12960: f64 = (2.0 * assign9620_e12959);
        (assign9620_e12960,)
    } else {
        (locals.var_nuints,)
    }
};
        locals.var_nuints = assign9620_e12962;
        locals.var_nuints_rv = 0.0;

        let assign9630_e12965: f64 = if 0.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard251 = assign9630_e12965;
        locals.var_guard251_rv = 0.0;

        let assign9640_e12968: f64 = if locals.var_nuints == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard252 = assign9640_e12968;
        locals.var_guard252_rv = 0.0;

        let (assign9650_e12981,) = {
    if (((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && (locals.var_guard248 != 0.0)) && (locals.var_guard251 != 0.0)) && (locals.var_guard252 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rint,)
    }
};
        locals.var_rint = assign9650_e12981;
        locals.var_rint_rv = 0.0;

        let (assign9660_e13001,) = {
    if (((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && (locals.var_guard248 != 0.0)) && (locals.var_guard251 != 0.0)) && (locals.var_guard252 == 0.0)) {
        let assign9660_e12995: f64 = (p.p438 * locals.var_dmcgeff);
        let assign9660_e12998: f64 = (locals.var_weff * locals.var_nuints);
        let assign9660_e12999: f64 = (assign9660_e12995 / assign9660_e12998);
        (assign9660_e12999,)
    } else {
        (locals.var_rint,)
    }
};
        locals.var_rint = assign9660_e13001;
        locals.var_rint_rv = 0.0;

        let assign9670_e13004: f64 = if locals.var_nuintd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard253 = assign9670_e13004;
        locals.var_guard253_rv = 0.0;

        let (assign9680_e13018,) = {
    if (((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && (locals.var_guard248 != 0.0)) && (locals.var_guard251 == 0.0)) && (locals.var_guard253 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rint,)
    }
};
        locals.var_rint = assign9680_e13018;
        locals.var_rint_rv = 0.0;

        let (assign9690_e13039,) = {
    if (((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && (locals.var_guard248 != 0.0)) && (locals.var_guard251 == 0.0)) && (locals.var_guard253 == 0.0)) {
        let assign9690_e13033: f64 = (p.p438 * locals.var_dmcgeff);
        let assign9690_e13036: f64 = (locals.var_weff * locals.var_nuintd);
        let assign9690_e13037: f64 = (assign9690_e13033 / assign9690_e13036);
        (assign9690_e13037,)
    } else {
        (locals.var_rint,)
    }
};
        locals.var_rint = assign9690_e13039;
        locals.var_rint_rv = 0.0;

        let assign9700_e13042: f64 = if p.p8 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard254 = assign9700_e13042;
        locals.var_guard254_rv = 0.0;

        let assign9710_e13045: f64 = if p.p8 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard255 = assign9710_e13045;
        locals.var_guard255_rv = 0.0;

        let assign9720_e13048: f64 = if p.p8 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard256 = assign9720_e13048;
        locals.var_guard256_rv = 0.0;

        let assign9730_e13051: f64 = if p.p8 == 3.0 { 1.0 } else { 0.0 };
        locals.var_guard257 = assign9730_e13051;
        locals.var_guard257_rv = 0.0;

        let assign9740_e13054: f64 = if p.p8 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard258 = assign9740_e13054;
        locals.var_guard258_rv = 0.0;

        let assign9750_e13057: f64 = if p.p8 == 5.0 { 1.0 } else { 0.0 };
        locals.var_guard259 = assign9750_e13057;
        locals.var_guard259_rv = 0.0;

        let assign9760_e13060: f64 = if p.p8 == 6.0 { 1.0 } else { 0.0 };
        locals.var_guard260 = assign9760_e13060;
        locals.var_guard260_rv = 0.0;

        let assign9770_e13063: f64 = if p.p8 == 7.0 { 1.0 } else { 0.0 };
        locals.var_guard261 = assign9770_e13063;
        locals.var_guard261_rv = 0.0;

        let assign9780_e13066: f64 = if p.p8 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard262 = assign9780_e13066;
        locals.var_guard262_rv = 0.0;

        let assign9790_e13069: f64 = if p.p8 == 9.0 { 1.0 } else { 0.0 };
        locals.var_guard263 = assign9790_e13069;
        locals.var_guard263_rv = 0.0;

        let assign9800_e13072: f64 = if p.p8 == 10.0 { 1.0 } else { 0.0 };
        locals.var_guard264 = assign9800_e13072;
        locals.var_guard264_rv = 0.0;

        let assign9810_e13075: f64 = if 0.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard265 = assign9810_e13075;
        locals.var_guard265_rv = 0.0;

        let assign9820_e13078: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard266 = assign9820_e13078;
        locals.var_guard266_rv = 0.0;

        let assign9830_e13089: f64 = if (((p.p9 == 1.0) || (p.p9 == 2.0)) || (p.p9 == 5.0)) { 1.0 } else { 0.0 };
        locals.var_guard267 = assign9830_e13089;
        locals.var_guard267_rv = 0.0;

        let assign9840_e13100: f64 = if (((p.p9 == 3.0) || (p.p9 == 4.0)) || (p.p9 == 6.0)) { 1.0 } else { 0.0 };
        locals.var_guard268 = assign9840_e13100;
        locals.var_guard268_rv = 0.0;

        let assign9850_e13103: f64 = if locals.var_nuends == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard269 = assign9850_e13103;
        locals.var_guard269_rv = 0.0;

        let (assign9860_e13120,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && (locals.var_guard254 != 0.0)) && (locals.var_guard265 != 0.0)) && (locals.var_guard266 != 0.0)) && (locals.var_guard267 != 0.0)) && (locals.var_guard269 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign9860_e13120;
        locals.var_rend_rv = 0.0;

        let (assign9870_e13144,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && (locals.var_guard254 != 0.0)) && (locals.var_guard265 != 0.0)) && (locals.var_guard266 != 0.0)) && (locals.var_guard267 != 0.0)) && (locals.var_guard269 == 0.0)) {
        let assign9870_e13138: f64 = (p.p438 * locals.var_dmcgeff);
        let assign9870_e13141: f64 = (locals.var_weff * locals.var_nuends);
        let assign9870_e13142: f64 = (assign9870_e13138 / assign9870_e13141);
        (assign9870_e13142,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign9870_e13144;
        locals.var_rend_rv = 0.0;

        let assign9890_e13155: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign9890_e13158: f64 = if ((locals.var_nuends == 0.0) || (assign9890_e13155 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard271 = assign9890_e13158;
        locals.var_guard271_rv = 0.0;

        let (assign9900_e13178,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && (locals.var_guard254 != 0.0)) && (locals.var_guard265 != 0.0)) && (locals.var_guard266 != 0.0)) && ((locals.var_guard268 != 0.0) && (locals.var_guard267 == 0.0))) && (locals.var_guard271 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign9900_e13178;
        locals.var_rend_rv = 0.0;

        let (assign9910_e13209,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && (locals.var_guard254 != 0.0)) && (locals.var_guard265 != 0.0)) && (locals.var_guard266 != 0.0)) && ((locals.var_guard268 != 0.0) && (locals.var_guard267 == 0.0))) && (locals.var_guard271 == 0.0)) {
        let assign9910_e13199: f64 = (p.p438 * locals.var_weff);
        let assign9910_e13202: f64 = (3.0 * locals.var_nuends);
        let assign9910_e13205: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign9910_e13206: f64 = (assign9910_e13202 * assign9910_e13205);
        let assign9910_e13207: f64 = (assign9910_e13199 / assign9910_e13206);
        (assign9910_e13207,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign9910_e13209;
        locals.var_rend_rv = 0.0;

        let (assign9920_e13227,) = {
    if ((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && (locals.var_guard254 != 0.0)) && (locals.var_guard265 != 0.0)) && (locals.var_guard266 != 0.0)) && (!((locals.var_guard267 != 0.0) || (locals.var_guard268 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign9920_e13227;
        locals.var_rend_rv = 0.0;

        let assign9930_e13238: f64 = if (((p.p9 == 1.0) || (p.p9 == 3.0)) || (p.p9 == 7.0)) { 1.0 } else { 0.0 };
        locals.var_guard272 = assign9930_e13238;
        locals.var_guard272_rv = 0.0;

        let assign9940_e13249: f64 = if (((p.p9 == 2.0) || (p.p9 == 4.0)) || (p.p9 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard273 = assign9940_e13249;
        locals.var_guard273_rv = 0.0;

        let assign9950_e13252: f64 = if locals.var_nuends == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard274 = assign9950_e13252;
        locals.var_guard274_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_17(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign9960_e13270,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && (locals.var_guard254 != 0.0)) && (locals.var_guard265 != 0.0)) && (locals.var_guard266 == 0.0)) && (locals.var_guard272 != 0.0)) && (locals.var_guard274 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign9960_e13270;
        locals.var_rend_rv = 0.0;

        let (assign9970_e13295,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && (locals.var_guard254 != 0.0)) && (locals.var_guard265 != 0.0)) && (locals.var_guard266 == 0.0)) && (locals.var_guard272 != 0.0)) && (locals.var_guard274 == 0.0)) {
        let assign9970_e13289: f64 = (p.p438 * locals.var_dmcgeff);
        let assign9970_e13292: f64 = (locals.var_weff * locals.var_nuends);
        let assign9970_e13293: f64 = (assign9970_e13289 / assign9970_e13292);
        (assign9970_e13293,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign9970_e13295;
        locals.var_rend_rv = 0.0;

        let assign9990_e13306: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign9990_e13309: f64 = if ((locals.var_nuends == 0.0) || (assign9990_e13306 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard276 = assign9990_e13309;
        locals.var_guard276_rv = 0.0;

        let (assign10000_e13330,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && (locals.var_guard254 != 0.0)) && (locals.var_guard265 != 0.0)) && (locals.var_guard266 == 0.0)) && ((locals.var_guard273 != 0.0) && (locals.var_guard272 == 0.0))) && (locals.var_guard276 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10000_e13330;
        locals.var_rend_rv = 0.0;

        let (assign10010_e13362,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && (locals.var_guard254 != 0.0)) && (locals.var_guard265 != 0.0)) && (locals.var_guard266 == 0.0)) && ((locals.var_guard273 != 0.0) && (locals.var_guard272 == 0.0))) && (locals.var_guard276 == 0.0)) {
        let assign10010_e13352: f64 = (p.p438 * locals.var_weff);
        let assign10010_e13355: f64 = (3.0 * locals.var_nuends);
        let assign10010_e13358: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign10010_e13359: f64 = (assign10010_e13355 * assign10010_e13358);
        let assign10010_e13360: f64 = (assign10010_e13352 / assign10010_e13359);
        (assign10010_e13360,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10010_e13362;
        locals.var_rend_rv = 0.0;

        let (assign10020_e13381,) = {
    if ((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && (locals.var_guard254 != 0.0)) && (locals.var_guard265 != 0.0)) && (locals.var_guard266 == 0.0)) && (!((locals.var_guard272 != 0.0) || (locals.var_guard273 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10020_e13381;
        locals.var_rend_rv = 0.0;

        let assign10030_e13384: f64 = if 0.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard277 = assign10030_e13384;
        locals.var_guard277_rv = 0.0;

        let assign10040_e13395: f64 = if (((p.p9 == 1.0) || (p.p9 == 2.0)) || (p.p9 == 5.0)) { 1.0 } else { 0.0 };
        locals.var_guard278 = assign10040_e13395;
        locals.var_guard278_rv = 0.0;

        let assign10050_e13406: f64 = if (((p.p9 == 3.0) || (p.p9 == 4.0)) || (p.p9 == 6.0)) { 1.0 } else { 0.0 };
        locals.var_guard279 = assign10050_e13406;
        locals.var_guard279_rv = 0.0;

        let assign10060_e13409: f64 = if locals.var_nuendd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard280 = assign10060_e13409;
        locals.var_guard280_rv = 0.0;

        let (assign10070_e13427,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && (locals.var_guard254 != 0.0)) && (locals.var_guard265 == 0.0)) && (locals.var_guard277 != 0.0)) && (locals.var_guard278 != 0.0)) && (locals.var_guard280 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10070_e13427;
        locals.var_rend_rv = 0.0;

        let (assign10080_e13452,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && (locals.var_guard254 != 0.0)) && (locals.var_guard265 == 0.0)) && (locals.var_guard277 != 0.0)) && (locals.var_guard278 != 0.0)) && (locals.var_guard280 == 0.0)) {
        let assign10080_e13446: f64 = (p.p438 * locals.var_dmcgeff);
        let assign10080_e13449: f64 = (locals.var_weff * locals.var_nuendd);
        let assign10080_e13450: f64 = (assign10080_e13446 / assign10080_e13449);
        (assign10080_e13450,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10080_e13452;
        locals.var_rend_rv = 0.0;

        let assign10100_e13463: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign10100_e13466: f64 = if ((locals.var_nuendd == 0.0) || (assign10100_e13463 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard282 = assign10100_e13466;
        locals.var_guard282_rv = 0.0;

        let (assign10110_e13487,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && (locals.var_guard254 != 0.0)) && (locals.var_guard265 == 0.0)) && (locals.var_guard277 != 0.0)) && ((locals.var_guard279 != 0.0) && (locals.var_guard278 == 0.0))) && (locals.var_guard282 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10110_e13487;
        locals.var_rend_rv = 0.0;

        let (assign10120_e13519,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && (locals.var_guard254 != 0.0)) && (locals.var_guard265 == 0.0)) && (locals.var_guard277 != 0.0)) && ((locals.var_guard279 != 0.0) && (locals.var_guard278 == 0.0))) && (locals.var_guard282 == 0.0)) {
        let assign10120_e13509: f64 = (p.p438 * locals.var_weff);
        let assign10120_e13512: f64 = (3.0 * locals.var_nuendd);
        let assign10120_e13515: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign10120_e13516: f64 = (assign10120_e13512 * assign10120_e13515);
        let assign10120_e13517: f64 = (assign10120_e13509 / assign10120_e13516);
        (assign10120_e13517,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10120_e13519;
        locals.var_rend_rv = 0.0;

        let (assign10130_e13538,) = {
    if ((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && (locals.var_guard254 != 0.0)) && (locals.var_guard265 == 0.0)) && (locals.var_guard277 != 0.0)) && (!((locals.var_guard278 != 0.0) || (locals.var_guard279 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10130_e13538;
        locals.var_rend_rv = 0.0;

        let assign10140_e13549: f64 = if (((p.p9 == 1.0) || (p.p9 == 3.0)) || (p.p9 == 7.0)) { 1.0 } else { 0.0 };
        locals.var_guard283 = assign10140_e13549;
        locals.var_guard283_rv = 0.0;

        let assign10150_e13560: f64 = if (((p.p9 == 2.0) || (p.p9 == 4.0)) || (p.p9 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard284 = assign10150_e13560;
        locals.var_guard284_rv = 0.0;

        let assign10160_e13563: f64 = if locals.var_nuendd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard285 = assign10160_e13563;
        locals.var_guard285_rv = 0.0;

        let (assign10170_e13582,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && (locals.var_guard254 != 0.0)) && (locals.var_guard265 == 0.0)) && (locals.var_guard277 == 0.0)) && (locals.var_guard283 != 0.0)) && (locals.var_guard285 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10170_e13582;
        locals.var_rend_rv = 0.0;

        let (assign10180_e13608,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && (locals.var_guard254 != 0.0)) && (locals.var_guard265 == 0.0)) && (locals.var_guard277 == 0.0)) && (locals.var_guard283 != 0.0)) && (locals.var_guard285 == 0.0)) {
        let assign10180_e13602: f64 = (p.p438 * locals.var_dmcgeff);
        let assign10180_e13605: f64 = (locals.var_weff * locals.var_nuendd);
        let assign10180_e13606: f64 = (assign10180_e13602 / assign10180_e13605);
        (assign10180_e13606,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10180_e13608;
        locals.var_rend_rv = 0.0;

        let assign10200_e13619: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign10200_e13622: f64 = if ((locals.var_nuendd == 0.0) || (assign10200_e13619 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard287 = assign10200_e13622;
        locals.var_guard287_rv = 0.0;

        let (assign10210_e13644,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && (locals.var_guard254 != 0.0)) && (locals.var_guard265 == 0.0)) && (locals.var_guard277 == 0.0)) && ((locals.var_guard284 != 0.0) && (locals.var_guard283 == 0.0))) && (locals.var_guard287 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10210_e13644;
        locals.var_rend_rv = 0.0;

        let (assign10220_e13677,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && (locals.var_guard254 != 0.0)) && (locals.var_guard265 == 0.0)) && (locals.var_guard277 == 0.0)) && ((locals.var_guard284 != 0.0) && (locals.var_guard283 == 0.0))) && (locals.var_guard287 == 0.0)) {
        let assign10220_e13667: f64 = (p.p438 * locals.var_weff);
        let assign10220_e13670: f64 = (3.0 * locals.var_nuendd);
        let assign10220_e13673: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign10220_e13674: f64 = (assign10220_e13670 * assign10220_e13673);
        let assign10220_e13675: f64 = (assign10220_e13667 / assign10220_e13674);
        (assign10220_e13675,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10220_e13677;
        locals.var_rend_rv = 0.0;

        let (assign10230_e13697,) = {
    if ((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && (locals.var_guard254 != 0.0)) && (locals.var_guard265 == 0.0)) && (locals.var_guard277 == 0.0)) && (!((locals.var_guard283 != 0.0) || (locals.var_guard284 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10230_e13697;
        locals.var_rend_rv = 0.0;

        let assign10240_e13700: f64 = if 0.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard288 = assign10240_e13700;
        locals.var_guard288_rv = 0.0;

        let assign10250_e13703: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard289 = assign10250_e13703;
        locals.var_guard289_rv = 0.0;

        let assign10260_e13714: f64 = if (((p.p9 == 1.0) || (p.p9 == 2.0)) || (p.p9 == 5.0)) { 1.0 } else { 0.0 };
        locals.var_guard290 = assign10260_e13714;
        locals.var_guard290_rv = 0.0;

        let assign10270_e13725: f64 = if (((p.p9 == 3.0) || (p.p9 == 4.0)) || (p.p9 == 6.0)) { 1.0 } else { 0.0 };
        locals.var_guard291 = assign10270_e13725;
        locals.var_guard291_rv = 0.0;

        let assign10280_e13728: f64 = if locals.var_nuends == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard292 = assign10280_e13728;
        locals.var_guard292_rv = 0.0;

        let (assign10290_e13748,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard255 != 0.0) && (locals.var_guard254 == 0.0))) && (locals.var_guard288 != 0.0)) && (locals.var_guard289 != 0.0)) && (locals.var_guard290 != 0.0)) && (locals.var_guard292 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10290_e13748;
        locals.var_rend_rv = 0.0;

        let (assign10300_e13775,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard255 != 0.0) && (locals.var_guard254 == 0.0))) && (locals.var_guard288 != 0.0)) && (locals.var_guard289 != 0.0)) && (locals.var_guard290 != 0.0)) && (locals.var_guard292 == 0.0)) {
        let assign10300_e13769: f64 = (p.p438 * locals.var_dmcgeff);
        let assign10300_e13772: f64 = (locals.var_weff * locals.var_nuends);
        let assign10300_e13773: f64 = (assign10300_e13769 / assign10300_e13772);
        (assign10300_e13773,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10300_e13775;
        locals.var_rend_rv = 0.0;

        let assign10320_e13786: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign10320_e13789: f64 = if ((locals.var_nuends == 0.0) || (assign10320_e13786 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard294 = assign10320_e13789;
        locals.var_guard294_rv = 0.0;

        let (assign10330_e13812,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard255 != 0.0) && (locals.var_guard254 == 0.0))) && (locals.var_guard288 != 0.0)) && (locals.var_guard289 != 0.0)) && ((locals.var_guard291 != 0.0) && (locals.var_guard290 == 0.0))) && (locals.var_guard294 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10330_e13812;
        locals.var_rend_rv = 0.0;

        let (assign10340_e13846,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard255 != 0.0) && (locals.var_guard254 == 0.0))) && (locals.var_guard288 != 0.0)) && (locals.var_guard289 != 0.0)) && ((locals.var_guard291 != 0.0) && (locals.var_guard290 == 0.0))) && (locals.var_guard294 == 0.0)) {
        let assign10340_e13836: f64 = (p.p438 * locals.var_weff);
        let assign10340_e13839: f64 = (3.0 * locals.var_nuends);
        let assign10340_e13842: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign10340_e13843: f64 = (assign10340_e13839 * assign10340_e13842);
        let assign10340_e13844: f64 = (assign10340_e13836 / assign10340_e13843);
        (assign10340_e13844,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10340_e13846;
        locals.var_rend_rv = 0.0;

        let (assign10350_e13867,) = {
    if ((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard255 != 0.0) && (locals.var_guard254 == 0.0))) && (locals.var_guard288 != 0.0)) && (locals.var_guard289 != 0.0)) && (!((locals.var_guard290 != 0.0) || (locals.var_guard291 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10350_e13867;
        locals.var_rend_rv = 0.0;

        let assign10360_e13878: f64 = if (((p.p9 == 1.0) || (p.p9 == 3.0)) || (p.p9 == 7.0)) { 1.0 } else { 0.0 };
        locals.var_guard295 = assign10360_e13878;
        locals.var_guard295_rv = 0.0;

        let assign10370_e13889: f64 = if (((p.p9 == 2.0) || (p.p9 == 4.0)) || (p.p9 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard296 = assign10370_e13889;
        locals.var_guard296_rv = 0.0;

        let assign10380_e13892: f64 = if locals.var_nuends == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard297 = assign10380_e13892;
        locals.var_guard297_rv = 0.0;

        let (assign10390_e13913,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard255 != 0.0) && (locals.var_guard254 == 0.0))) && (locals.var_guard288 != 0.0)) && (locals.var_guard289 == 0.0)) && (locals.var_guard295 != 0.0)) && (locals.var_guard297 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10390_e13913;
        locals.var_rend_rv = 0.0;

        let (assign10400_e13941,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard255 != 0.0) && (locals.var_guard254 == 0.0))) && (locals.var_guard288 != 0.0)) && (locals.var_guard289 == 0.0)) && (locals.var_guard295 != 0.0)) && (locals.var_guard297 == 0.0)) {
        let assign10400_e13935: f64 = (p.p438 * locals.var_dmcgeff);
        let assign10400_e13938: f64 = (locals.var_weff * locals.var_nuends);
        let assign10400_e13939: f64 = (assign10400_e13935 / assign10400_e13938);
        (assign10400_e13939,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10400_e13941;
        locals.var_rend_rv = 0.0;

        let assign10420_e13952: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign10420_e13955: f64 = if ((locals.var_nuends == 0.0) || (assign10420_e13952 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard299 = assign10420_e13955;
        locals.var_guard299_rv = 0.0;

        let (assign10430_e13979,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard255 != 0.0) && (locals.var_guard254 == 0.0))) && (locals.var_guard288 != 0.0)) && (locals.var_guard289 == 0.0)) && ((locals.var_guard296 != 0.0) && (locals.var_guard295 == 0.0))) && (locals.var_guard299 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10430_e13979;
        locals.var_rend_rv = 0.0;

        let (assign10440_e14014,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard255 != 0.0) && (locals.var_guard254 == 0.0))) && (locals.var_guard288 != 0.0)) && (locals.var_guard289 == 0.0)) && ((locals.var_guard296 != 0.0) && (locals.var_guard295 == 0.0))) && (locals.var_guard299 == 0.0)) {
        let assign10440_e14004: f64 = (p.p438 * locals.var_weff);
        let assign10440_e14007: f64 = (3.0 * locals.var_nuends);
        let assign10440_e14010: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign10440_e14011: f64 = (assign10440_e14007 * assign10440_e14010);
        let assign10440_e14012: f64 = (assign10440_e14004 / assign10440_e14011);
        (assign10440_e14012,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10440_e14014;
        locals.var_rend_rv = 0.0;

        let (assign10450_e14036,) = {
    if ((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard255 != 0.0) && (locals.var_guard254 == 0.0))) && (locals.var_guard288 != 0.0)) && (locals.var_guard289 == 0.0)) && (!((locals.var_guard295 != 0.0) || (locals.var_guard296 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10450_e14036;
        locals.var_rend_rv = 0.0;

        let assign10460_e14039: f64 = if 0.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard300 = assign10460_e14039;
        locals.var_guard300_rv = 0.0;

        let assign10470_e14050: f64 = if (((p.p9 == 1.0) || (p.p9 == 2.0)) || (p.p9 == 5.0)) { 1.0 } else { 0.0 };
        locals.var_guard301 = assign10470_e14050;
        locals.var_guard301_rv = 0.0;

        let assign10480_e14061: f64 = if (((p.p9 == 3.0) || (p.p9 == 4.0)) || (p.p9 == 6.0)) { 1.0 } else { 0.0 };
        locals.var_guard302 = assign10480_e14061;
        locals.var_guard302_rv = 0.0;

        let assign10490_e14064: f64 = if locals.var_nuendd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard303 = assign10490_e14064;
        locals.var_guard303_rv = 0.0;

        let (assign10500_e14085,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard255 != 0.0) && (locals.var_guard254 == 0.0))) && (locals.var_guard288 == 0.0)) && (locals.var_guard300 != 0.0)) && (locals.var_guard301 != 0.0)) && (locals.var_guard303 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10500_e14085;
        locals.var_rend_rv = 0.0;

        let (assign10510_e14113,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard255 != 0.0) && (locals.var_guard254 == 0.0))) && (locals.var_guard288 == 0.0)) && (locals.var_guard300 != 0.0)) && (locals.var_guard301 != 0.0)) && (locals.var_guard303 == 0.0)) {
        let assign10510_e14107: f64 = (p.p438 * locals.var_dmcgeff);
        let assign10510_e14110: f64 = (locals.var_weff * locals.var_nuendd);
        let assign10510_e14111: f64 = (assign10510_e14107 / assign10510_e14110);
        (assign10510_e14111,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10510_e14113;
        locals.var_rend_rv = 0.0;

        let assign10530_e14123: f64 = if ((locals.var_nuendd == 0.0) || (locals.var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard305 = assign10530_e14123;
        locals.var_guard305_rv = 0.0;

        let (assign10540_e14147,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard255 != 0.0) && (locals.var_guard254 == 0.0))) && (locals.var_guard288 == 0.0)) && (locals.var_guard300 != 0.0)) && ((locals.var_guard302 != 0.0) && (locals.var_guard301 == 0.0))) && (locals.var_guard305 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10540_e14147;
        locals.var_rend_rv = 0.0;

        let (assign10550_e14180,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard255 != 0.0) && (locals.var_guard254 == 0.0))) && (locals.var_guard288 == 0.0)) && (locals.var_guard300 != 0.0)) && ((locals.var_guard302 != 0.0) && (locals.var_guard301 == 0.0))) && (locals.var_guard305 == 0.0)) {
        let assign10550_e14172: f64 = (p.p438 * locals.var_weff);
        let assign10550_e14175: f64 = (6.0 * locals.var_nuendd);
        let assign10550_e14177: f64 = (assign10550_e14175 * locals.var_dmcgeff);
        let assign10550_e14178: f64 = (assign10550_e14172 / assign10550_e14177);
        (assign10550_e14178,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10550_e14180;
        locals.var_rend_rv = 0.0;

        let (assign10560_e14202,) = {
    if ((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard255 != 0.0) && (locals.var_guard254 == 0.0))) && (locals.var_guard288 == 0.0)) && (locals.var_guard300 != 0.0)) && (!((locals.var_guard301 != 0.0) || (locals.var_guard302 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10560_e14202;
        locals.var_rend_rv = 0.0;

        let assign10570_e14213: f64 = if (((p.p9 == 1.0) || (p.p9 == 3.0)) || (p.p9 == 7.0)) { 1.0 } else { 0.0 };
        locals.var_guard306 = assign10570_e14213;
        locals.var_guard306_rv = 0.0;

        let assign10580_e14224: f64 = if (((p.p9 == 2.0) || (p.p9 == 4.0)) || (p.p9 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard307 = assign10580_e14224;
        locals.var_guard307_rv = 0.0;

        let assign10590_e14227: f64 = if locals.var_nuendd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard308 = assign10590_e14227;
        locals.var_guard308_rv = 0.0;

        let (assign10600_e14249,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard255 != 0.0) && (locals.var_guard254 == 0.0))) && (locals.var_guard288 == 0.0)) && (locals.var_guard300 == 0.0)) && (locals.var_guard306 != 0.0)) && (locals.var_guard308 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10600_e14249;
        locals.var_rend_rv = 0.0;

        let (assign10610_e14278,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard255 != 0.0) && (locals.var_guard254 == 0.0))) && (locals.var_guard288 == 0.0)) && (locals.var_guard300 == 0.0)) && (locals.var_guard306 != 0.0)) && (locals.var_guard308 == 0.0)) {
        let assign10610_e14272: f64 = (p.p438 * locals.var_dmcgeff);
        let assign10610_e14275: f64 = (locals.var_weff * locals.var_nuendd);
        let assign10610_e14276: f64 = (assign10610_e14272 / assign10610_e14275);
        (assign10610_e14276,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10610_e14278;
        locals.var_rend_rv = 0.0;

        let assign10630_e14288: f64 = if ((locals.var_nuendd == 0.0) || (locals.var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard310 = assign10630_e14288;
        locals.var_guard310_rv = 0.0;

        let (assign10640_e14313,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard255 != 0.0) && (locals.var_guard254 == 0.0))) && (locals.var_guard288 == 0.0)) && (locals.var_guard300 == 0.0)) && ((locals.var_guard307 != 0.0) && (locals.var_guard306 == 0.0))) && (locals.var_guard310 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10640_e14313;
        locals.var_rend_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_18(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign10650_e14347,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard255 != 0.0) && (locals.var_guard254 == 0.0))) && (locals.var_guard288 == 0.0)) && (locals.var_guard300 == 0.0)) && ((locals.var_guard307 != 0.0) && (locals.var_guard306 == 0.0))) && (locals.var_guard310 == 0.0)) {
        let assign10650_e14339: f64 = (p.p438 * locals.var_weff);
        let assign10650_e14342: f64 = (6.0 * locals.var_nuendd);
        let assign10650_e14344: f64 = (assign10650_e14342 * locals.var_dmcgeff);
        let assign10650_e14345: f64 = (assign10650_e14339 / assign10650_e14344);
        (assign10650_e14345,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10650_e14347;
        locals.var_rend_rv = 0.0;

        let (assign10660_e14370,) = {
    if ((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard255 != 0.0) && (locals.var_guard254 == 0.0))) && (locals.var_guard288 == 0.0)) && (locals.var_guard300 == 0.0)) && (!((locals.var_guard306 != 0.0) || (locals.var_guard307 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10660_e14370;
        locals.var_rend_rv = 0.0;

        let assign10670_e14373: f64 = if 0.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard311 = assign10670_e14373;
        locals.var_guard311_rv = 0.0;

        let assign10680_e14376: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard312 = assign10680_e14376;
        locals.var_guard312_rv = 0.0;

        let assign10690_e14387: f64 = if (((p.p9 == 1.0) || (p.p9 == 2.0)) || (p.p9 == 5.0)) { 1.0 } else { 0.0 };
        locals.var_guard313 = assign10690_e14387;
        locals.var_guard313_rv = 0.0;

        let assign10700_e14398: f64 = if (((p.p9 == 3.0) || (p.p9 == 4.0)) || (p.p9 == 6.0)) { 1.0 } else { 0.0 };
        locals.var_guard314 = assign10700_e14398;
        locals.var_guard314_rv = 0.0;

        let assign10710_e14401: f64 = if locals.var_nuends == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard315 = assign10710_e14401;
        locals.var_guard315_rv = 0.0;

        let (assign10720_e14423,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard256 != 0.0) && (!((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0))))) && (locals.var_guard311 != 0.0)) && (locals.var_guard312 != 0.0)) && (locals.var_guard313 != 0.0)) && (locals.var_guard315 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10720_e14423;
        locals.var_rend_rv = 0.0;

        let (assign10730_e14452,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard256 != 0.0) && (!((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0))))) && (locals.var_guard311 != 0.0)) && (locals.var_guard312 != 0.0)) && (locals.var_guard313 != 0.0)) && (locals.var_guard315 == 0.0)) {
        let assign10730_e14446: f64 = (p.p438 * locals.var_dmcgeff);
        let assign10730_e14449: f64 = (locals.var_weff * locals.var_nuends);
        let assign10730_e14450: f64 = (assign10730_e14446 / assign10730_e14449);
        (assign10730_e14450,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10730_e14452;
        locals.var_rend_rv = 0.0;

        let assign10750_e14462: f64 = if ((locals.var_nuends == 0.0) || (locals.var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard317 = assign10750_e14462;
        locals.var_guard317_rv = 0.0;

        let (assign10760_e14487,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard256 != 0.0) && (!((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0))))) && (locals.var_guard311 != 0.0)) && (locals.var_guard312 != 0.0)) && ((locals.var_guard314 != 0.0) && (locals.var_guard313 == 0.0))) && (locals.var_guard317 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10760_e14487;
        locals.var_rend_rv = 0.0;

        let (assign10770_e14521,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard256 != 0.0) && (!((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0))))) && (locals.var_guard311 != 0.0)) && (locals.var_guard312 != 0.0)) && ((locals.var_guard314 != 0.0) && (locals.var_guard313 == 0.0))) && (locals.var_guard317 == 0.0)) {
        let assign10770_e14513: f64 = (p.p438 * locals.var_weff);
        let assign10770_e14516: f64 = (6.0 * locals.var_nuends);
        let assign10770_e14518: f64 = (assign10770_e14516 * locals.var_dmcgeff);
        let assign10770_e14519: f64 = (assign10770_e14513 / assign10770_e14518);
        (assign10770_e14519,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10770_e14521;
        locals.var_rend_rv = 0.0;

        let (assign10780_e14544,) = {
    if ((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard256 != 0.0) && (!((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0))))) && (locals.var_guard311 != 0.0)) && (locals.var_guard312 != 0.0)) && (!((locals.var_guard313 != 0.0) || (locals.var_guard314 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10780_e14544;
        locals.var_rend_rv = 0.0;

        let assign10790_e14555: f64 = if (((p.p9 == 1.0) || (p.p9 == 3.0)) || (p.p9 == 7.0)) { 1.0 } else { 0.0 };
        locals.var_guard318 = assign10790_e14555;
        locals.var_guard318_rv = 0.0;

        let assign10800_e14566: f64 = if (((p.p9 == 2.0) || (p.p9 == 4.0)) || (p.p9 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard319 = assign10800_e14566;
        locals.var_guard319_rv = 0.0;

        let assign10810_e14569: f64 = if locals.var_nuends == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard320 = assign10810_e14569;
        locals.var_guard320_rv = 0.0;

        let (assign10820_e14592,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard256 != 0.0) && (!((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0))))) && (locals.var_guard311 != 0.0)) && (locals.var_guard312 == 0.0)) && (locals.var_guard318 != 0.0)) && (locals.var_guard320 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10820_e14592;
        locals.var_rend_rv = 0.0;

        let (assign10830_e14622,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard256 != 0.0) && (!((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0))))) && (locals.var_guard311 != 0.0)) && (locals.var_guard312 == 0.0)) && (locals.var_guard318 != 0.0)) && (locals.var_guard320 == 0.0)) {
        let assign10830_e14616: f64 = (p.p438 * locals.var_dmcgeff);
        let assign10830_e14619: f64 = (locals.var_weff * locals.var_nuends);
        let assign10830_e14620: f64 = (assign10830_e14616 / assign10830_e14619);
        (assign10830_e14620,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10830_e14622;
        locals.var_rend_rv = 0.0;

        let assign10850_e14632: f64 = if ((locals.var_nuends == 0.0) || (locals.var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard322 = assign10850_e14632;
        locals.var_guard322_rv = 0.0;

        let (assign10860_e14658,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard256 != 0.0) && (!((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0))))) && (locals.var_guard311 != 0.0)) && (locals.var_guard312 == 0.0)) && ((locals.var_guard319 != 0.0) && (locals.var_guard318 == 0.0))) && (locals.var_guard322 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10860_e14658;
        locals.var_rend_rv = 0.0;

        let (assign10870_e14693,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard256 != 0.0) && (!((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0))))) && (locals.var_guard311 != 0.0)) && (locals.var_guard312 == 0.0)) && ((locals.var_guard319 != 0.0) && (locals.var_guard318 == 0.0))) && (locals.var_guard322 == 0.0)) {
        let assign10870_e14685: f64 = (p.p438 * locals.var_weff);
        let assign10870_e14688: f64 = (6.0 * locals.var_nuends);
        let assign10870_e14690: f64 = (assign10870_e14688 * locals.var_dmcgeff);
        let assign10870_e14691: f64 = (assign10870_e14685 / assign10870_e14690);
        (assign10870_e14691,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10870_e14693;
        locals.var_rend_rv = 0.0;

        let (assign10880_e14717,) = {
    if ((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard256 != 0.0) && (!((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0))))) && (locals.var_guard311 != 0.0)) && (locals.var_guard312 == 0.0)) && (!((locals.var_guard318 != 0.0) || (locals.var_guard319 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10880_e14717;
        locals.var_rend_rv = 0.0;

        let assign10890_e14720: f64 = if 0.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard323 = assign10890_e14720;
        locals.var_guard323_rv = 0.0;

        let assign10900_e14731: f64 = if (((p.p9 == 1.0) || (p.p9 == 2.0)) || (p.p9 == 5.0)) { 1.0 } else { 0.0 };
        locals.var_guard324 = assign10900_e14731;
        locals.var_guard324_rv = 0.0;

        let assign10910_e14742: f64 = if (((p.p9 == 3.0) || (p.p9 == 4.0)) || (p.p9 == 6.0)) { 1.0 } else { 0.0 };
        locals.var_guard325 = assign10910_e14742;
        locals.var_guard325_rv = 0.0;

        let assign10920_e14745: f64 = if locals.var_nuendd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard326 = assign10920_e14745;
        locals.var_guard326_rv = 0.0;

        let (assign10930_e14768,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard256 != 0.0) && (!((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0))))) && (locals.var_guard311 == 0.0)) && (locals.var_guard323 != 0.0)) && (locals.var_guard324 != 0.0)) && (locals.var_guard326 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10930_e14768;
        locals.var_rend_rv = 0.0;

        let (assign10940_e14798,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard256 != 0.0) && (!((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0))))) && (locals.var_guard311 == 0.0)) && (locals.var_guard323 != 0.0)) && (locals.var_guard324 != 0.0)) && (locals.var_guard326 == 0.0)) {
        let assign10940_e14792: f64 = (p.p438 * locals.var_dmcgeff);
        let assign10940_e14795: f64 = (locals.var_weff * locals.var_nuendd);
        let assign10940_e14796: f64 = (assign10940_e14792 / assign10940_e14795);
        (assign10940_e14796,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10940_e14798;
        locals.var_rend_rv = 0.0;

        let assign10960_e14809: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign10960_e14812: f64 = if ((locals.var_nuendd == 0.0) || (assign10960_e14809 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard328 = assign10960_e14812;
        locals.var_guard328_rv = 0.0;

        let (assign10970_e14838,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard256 != 0.0) && (!((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0))))) && (locals.var_guard311 == 0.0)) && (locals.var_guard323 != 0.0)) && ((locals.var_guard325 != 0.0) && (locals.var_guard324 == 0.0))) && (locals.var_guard328 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10970_e14838;
        locals.var_rend_rv = 0.0;

        let (assign10980_e14875,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard256 != 0.0) && (!((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0))))) && (locals.var_guard311 == 0.0)) && (locals.var_guard323 != 0.0)) && ((locals.var_guard325 != 0.0) && (locals.var_guard324 == 0.0))) && (locals.var_guard328 == 0.0)) {
        let assign10980_e14865: f64 = (p.p438 * locals.var_weff);
        let assign10980_e14868: f64 = (3.0 * locals.var_nuendd);
        let assign10980_e14871: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign10980_e14872: f64 = (assign10980_e14868 * assign10980_e14871);
        let assign10980_e14873: f64 = (assign10980_e14865 / assign10980_e14872);
        (assign10980_e14873,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10980_e14875;
        locals.var_rend_rv = 0.0;

        let (assign10990_e14899,) = {
    if ((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard256 != 0.0) && (!((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0))))) && (locals.var_guard311 == 0.0)) && (locals.var_guard323 != 0.0)) && (!((locals.var_guard324 != 0.0) || (locals.var_guard325 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10990_e14899;
        locals.var_rend_rv = 0.0;

        let assign11000_e14910: f64 = if (((p.p9 == 1.0) || (p.p9 == 3.0)) || (p.p9 == 7.0)) { 1.0 } else { 0.0 };
        locals.var_guard329 = assign11000_e14910;
        locals.var_guard329_rv = 0.0;

        let assign11010_e14921: f64 = if (((p.p9 == 2.0) || (p.p9 == 4.0)) || (p.p9 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard330 = assign11010_e14921;
        locals.var_guard330_rv = 0.0;

        let assign11020_e14924: f64 = if locals.var_nuendd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard331 = assign11020_e14924;
        locals.var_guard331_rv = 0.0;

        let (assign11030_e14948,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard256 != 0.0) && (!((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0))))) && (locals.var_guard311 == 0.0)) && (locals.var_guard323 == 0.0)) && (locals.var_guard329 != 0.0)) && (locals.var_guard331 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11030_e14948;
        locals.var_rend_rv = 0.0;

        let (assign11040_e14979,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard256 != 0.0) && (!((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0))))) && (locals.var_guard311 == 0.0)) && (locals.var_guard323 == 0.0)) && (locals.var_guard329 != 0.0)) && (locals.var_guard331 == 0.0)) {
        let assign11040_e14973: f64 = (p.p438 * locals.var_dmcgeff);
        let assign11040_e14976: f64 = (locals.var_weff * locals.var_nuendd);
        let assign11040_e14977: f64 = (assign11040_e14973 / assign11040_e14976);
        (assign11040_e14977,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11040_e14979;
        locals.var_rend_rv = 0.0;

        let assign11060_e14990: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign11060_e14993: f64 = if ((locals.var_nuendd == 0.0) || (assign11060_e14990 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard333 = assign11060_e14993;
        locals.var_guard333_rv = 0.0;

        let (assign11070_e15020,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard256 != 0.0) && (!((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0))))) && (locals.var_guard311 == 0.0)) && (locals.var_guard323 == 0.0)) && ((locals.var_guard330 != 0.0) && (locals.var_guard329 == 0.0))) && (locals.var_guard333 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11070_e15020;
        locals.var_rend_rv = 0.0;

        let (assign11080_e15058,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard256 != 0.0) && (!((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0))))) && (locals.var_guard311 == 0.0)) && (locals.var_guard323 == 0.0)) && ((locals.var_guard330 != 0.0) && (locals.var_guard329 == 0.0))) && (locals.var_guard333 == 0.0)) {
        let assign11080_e15048: f64 = (p.p438 * locals.var_weff);
        let assign11080_e15051: f64 = (3.0 * locals.var_nuendd);
        let assign11080_e15054: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign11080_e15055: f64 = (assign11080_e15051 * assign11080_e15054);
        let assign11080_e15056: f64 = (assign11080_e15048 / assign11080_e15055);
        (assign11080_e15056,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11080_e15058;
        locals.var_rend_rv = 0.0;

        let (assign11090_e15083,) = {
    if ((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard256 != 0.0) && (!((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0))))) && (locals.var_guard311 == 0.0)) && (locals.var_guard323 == 0.0)) && (!((locals.var_guard329 != 0.0) || (locals.var_guard330 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11090_e15083;
        locals.var_rend_rv = 0.0;

        let assign11100_e15086: f64 = if 0.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard334 = assign11100_e15086;
        locals.var_guard334_rv = 0.0;

        let assign11110_e15089: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard335 = assign11110_e15089;
        locals.var_guard335_rv = 0.0;

        let assign11120_e15100: f64 = if (((p.p9 == 1.0) || (p.p9 == 2.0)) || (p.p9 == 5.0)) { 1.0 } else { 0.0 };
        locals.var_guard336 = assign11120_e15100;
        locals.var_guard336_rv = 0.0;

        let assign11130_e15111: f64 = if (((p.p9 == 3.0) || (p.p9 == 4.0)) || (p.p9 == 6.0)) { 1.0 } else { 0.0 };
        locals.var_guard337 = assign11130_e15111;
        locals.var_guard337_rv = 0.0;

        let assign11140_e15114: f64 = if locals.var_nuends == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard338 = assign11140_e15114;
        locals.var_guard338_rv = 0.0;

        let (assign11150_e15138,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard257 != 0.0) && (!(((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0))))) && (locals.var_guard334 != 0.0)) && (locals.var_guard335 != 0.0)) && (locals.var_guard336 != 0.0)) && (locals.var_guard338 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11150_e15138;
        locals.var_rend_rv = 0.0;

        let (assign11160_e15169,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard257 != 0.0) && (!(((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0))))) && (locals.var_guard334 != 0.0)) && (locals.var_guard335 != 0.0)) && (locals.var_guard336 != 0.0)) && (locals.var_guard338 == 0.0)) {
        let assign11160_e15163: f64 = (p.p438 * locals.var_dmcgeff);
        let assign11160_e15166: f64 = (locals.var_weff * locals.var_nuends);
        let assign11160_e15167: f64 = (assign11160_e15163 / assign11160_e15166);
        (assign11160_e15167,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11160_e15169;
        locals.var_rend_rv = 0.0;

        let assign11180_e15179: f64 = if ((locals.var_nuends == 0.0) || (locals.var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard340 = assign11180_e15179;
        locals.var_guard340_rv = 0.0;

        let (assign11190_e15206,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard257 != 0.0) && (!(((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0))))) && (locals.var_guard334 != 0.0)) && (locals.var_guard335 != 0.0)) && ((locals.var_guard337 != 0.0) && (locals.var_guard336 == 0.0))) && (locals.var_guard340 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11190_e15206;
        locals.var_rend_rv = 0.0;

        let (assign11200_e15242,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard257 != 0.0) && (!(((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0))))) && (locals.var_guard334 != 0.0)) && (locals.var_guard335 != 0.0)) && ((locals.var_guard337 != 0.0) && (locals.var_guard336 == 0.0))) && (locals.var_guard340 == 0.0)) {
        let assign11200_e15234: f64 = (p.p438 * locals.var_weff);
        let assign11200_e15237: f64 = (6.0 * locals.var_nuends);
        let assign11200_e15239: f64 = (assign11200_e15237 * locals.var_dmcgeff);
        let assign11200_e15240: f64 = (assign11200_e15234 / assign11200_e15239);
        (assign11200_e15240,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11200_e15242;
        locals.var_rend_rv = 0.0;

        let (assign11210_e15267,) = {
    if ((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard257 != 0.0) && (!(((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0))))) && (locals.var_guard334 != 0.0)) && (locals.var_guard335 != 0.0)) && (!((locals.var_guard336 != 0.0) || (locals.var_guard337 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11210_e15267;
        locals.var_rend_rv = 0.0;

        let assign11220_e15278: f64 = if (((p.p9 == 1.0) || (p.p9 == 3.0)) || (p.p9 == 7.0)) { 1.0 } else { 0.0 };
        locals.var_guard341 = assign11220_e15278;
        locals.var_guard341_rv = 0.0;

        let assign11230_e15289: f64 = if (((p.p9 == 2.0) || (p.p9 == 4.0)) || (p.p9 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard342 = assign11230_e15289;
        locals.var_guard342_rv = 0.0;

        let assign11240_e15292: f64 = if locals.var_nuends == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard343 = assign11240_e15292;
        locals.var_guard343_rv = 0.0;

        let (assign11250_e15317,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard257 != 0.0) && (!(((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0))))) && (locals.var_guard334 != 0.0)) && (locals.var_guard335 == 0.0)) && (locals.var_guard341 != 0.0)) && (locals.var_guard343 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11250_e15317;
        locals.var_rend_rv = 0.0;

        let (assign11260_e15349,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard257 != 0.0) && (!(((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0))))) && (locals.var_guard334 != 0.0)) && (locals.var_guard335 == 0.0)) && (locals.var_guard341 != 0.0)) && (locals.var_guard343 == 0.0)) {
        let assign11260_e15343: f64 = (p.p438 * locals.var_dmcgeff);
        let assign11260_e15346: f64 = (locals.var_weff * locals.var_nuends);
        let assign11260_e15347: f64 = (assign11260_e15343 / assign11260_e15346);
        (assign11260_e15347,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11260_e15349;
        locals.var_rend_rv = 0.0;

        let assign11280_e15359: f64 = if ((locals.var_nuends == 0.0) || (locals.var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard345 = assign11280_e15359;
        locals.var_guard345_rv = 0.0;

        let (assign11290_e15387,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard257 != 0.0) && (!(((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0))))) && (locals.var_guard334 != 0.0)) && (locals.var_guard335 == 0.0)) && ((locals.var_guard342 != 0.0) && (locals.var_guard341 == 0.0))) && (locals.var_guard345 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11290_e15387;
        locals.var_rend_rv = 0.0;

        let (assign11300_e15424,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard257 != 0.0) && (!(((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0))))) && (locals.var_guard334 != 0.0)) && (locals.var_guard335 == 0.0)) && ((locals.var_guard342 != 0.0) && (locals.var_guard341 == 0.0))) && (locals.var_guard345 == 0.0)) {
        let assign11300_e15416: f64 = (p.p438 * locals.var_weff);
        let assign11300_e15419: f64 = (6.0 * locals.var_nuends);
        let assign11300_e15421: f64 = (assign11300_e15419 * locals.var_dmcgeff);
        let assign11300_e15422: f64 = (assign11300_e15416 / assign11300_e15421);
        (assign11300_e15422,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11300_e15424;
        locals.var_rend_rv = 0.0;

        let (assign11310_e15450,) = {
    if ((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard257 != 0.0) && (!(((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0))))) && (locals.var_guard334 != 0.0)) && (locals.var_guard335 == 0.0)) && (!((locals.var_guard341 != 0.0) || (locals.var_guard342 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11310_e15450;
        locals.var_rend_rv = 0.0;

        let assign11320_e15453: f64 = if 0.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard346 = assign11320_e15453;
        locals.var_guard346_rv = 0.0;

        let assign11330_e15464: f64 = if (((p.p9 == 1.0) || (p.p9 == 2.0)) || (p.p9 == 5.0)) { 1.0 } else { 0.0 };
        locals.var_guard347 = assign11330_e15464;
        locals.var_guard347_rv = 0.0;

        let assign11340_e15475: f64 = if (((p.p9 == 3.0) || (p.p9 == 4.0)) || (p.p9 == 6.0)) { 1.0 } else { 0.0 };
        locals.var_guard348 = assign11340_e15475;
        locals.var_guard348_rv = 0.0;

        let assign11350_e15478: f64 = if locals.var_nuendd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard349 = assign11350_e15478;
        locals.var_guard349_rv = 0.0;

        let (assign11360_e15503,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard257 != 0.0) && (!(((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0))))) && (locals.var_guard334 == 0.0)) && (locals.var_guard346 != 0.0)) && (locals.var_guard347 != 0.0)) && (locals.var_guard349 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11360_e15503;
        locals.var_rend_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_19(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign11370_e15535,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard257 != 0.0) && (!(((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0))))) && (locals.var_guard334 == 0.0)) && (locals.var_guard346 != 0.0)) && (locals.var_guard347 != 0.0)) && (locals.var_guard349 == 0.0)) {
        let assign11370_e15529: f64 = (p.p438 * locals.var_dmcgeff);
        let assign11370_e15532: f64 = (locals.var_weff * locals.var_nuendd);
        let assign11370_e15533: f64 = (assign11370_e15529 / assign11370_e15532);
        (assign11370_e15533,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11370_e15535;
        locals.var_rend_rv = 0.0;

        let assign11390_e15545: f64 = if ((locals.var_nuendd == 0.0) || (locals.var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard351 = assign11390_e15545;
        locals.var_guard351_rv = 0.0;

        let (assign11400_e15573,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard257 != 0.0) && (!(((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0))))) && (locals.var_guard334 == 0.0)) && (locals.var_guard346 != 0.0)) && ((locals.var_guard348 != 0.0) && (locals.var_guard347 == 0.0))) && (locals.var_guard351 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11400_e15573;
        locals.var_rend_rv = 0.0;

        let (assign11410_e15610,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard257 != 0.0) && (!(((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0))))) && (locals.var_guard334 == 0.0)) && (locals.var_guard346 != 0.0)) && ((locals.var_guard348 != 0.0) && (locals.var_guard347 == 0.0))) && (locals.var_guard351 == 0.0)) {
        let assign11410_e15602: f64 = (p.p438 * locals.var_weff);
        let assign11410_e15605: f64 = (6.0 * locals.var_nuendd);
        let assign11410_e15607: f64 = (assign11410_e15605 * locals.var_dmcgeff);
        let assign11410_e15608: f64 = (assign11410_e15602 / assign11410_e15607);
        (assign11410_e15608,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11410_e15610;
        locals.var_rend_rv = 0.0;

        let (assign11420_e15636,) = {
    if ((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard257 != 0.0) && (!(((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0))))) && (locals.var_guard334 == 0.0)) && (locals.var_guard346 != 0.0)) && (!((locals.var_guard347 != 0.0) || (locals.var_guard348 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11420_e15636;
        locals.var_rend_rv = 0.0;

        let assign11430_e15647: f64 = if (((p.p9 == 1.0) || (p.p9 == 3.0)) || (p.p9 == 7.0)) { 1.0 } else { 0.0 };
        locals.var_guard352 = assign11430_e15647;
        locals.var_guard352_rv = 0.0;

        let assign11440_e15658: f64 = if (((p.p9 == 2.0) || (p.p9 == 4.0)) || (p.p9 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard353 = assign11440_e15658;
        locals.var_guard353_rv = 0.0;

        let assign11450_e15661: f64 = if locals.var_nuendd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard354 = assign11450_e15661;
        locals.var_guard354_rv = 0.0;

        let (assign11460_e15687,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard257 != 0.0) && (!(((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0))))) && (locals.var_guard334 == 0.0)) && (locals.var_guard346 == 0.0)) && (locals.var_guard352 != 0.0)) && (locals.var_guard354 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11460_e15687;
        locals.var_rend_rv = 0.0;

        let (assign11470_e15720,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard257 != 0.0) && (!(((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0))))) && (locals.var_guard334 == 0.0)) && (locals.var_guard346 == 0.0)) && (locals.var_guard352 != 0.0)) && (locals.var_guard354 == 0.0)) {
        let assign11470_e15714: f64 = (p.p438 * locals.var_dmcgeff);
        let assign11470_e15717: f64 = (locals.var_weff * locals.var_nuendd);
        let assign11470_e15718: f64 = (assign11470_e15714 / assign11470_e15717);
        (assign11470_e15718,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11470_e15720;
        locals.var_rend_rv = 0.0;

        let assign11490_e15730: f64 = if ((locals.var_nuendd == 0.0) || (locals.var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard356 = assign11490_e15730;
        locals.var_guard356_rv = 0.0;

        let (assign11500_e15759,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard257 != 0.0) && (!(((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0))))) && (locals.var_guard334 == 0.0)) && (locals.var_guard346 == 0.0)) && ((locals.var_guard353 != 0.0) && (locals.var_guard352 == 0.0))) && (locals.var_guard356 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11500_e15759;
        locals.var_rend_rv = 0.0;

        let (assign11510_e15797,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard257 != 0.0) && (!(((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0))))) && (locals.var_guard334 == 0.0)) && (locals.var_guard346 == 0.0)) && ((locals.var_guard353 != 0.0) && (locals.var_guard352 == 0.0))) && (locals.var_guard356 == 0.0)) {
        let assign11510_e15789: f64 = (p.p438 * locals.var_weff);
        let assign11510_e15792: f64 = (6.0 * locals.var_nuendd);
        let assign11510_e15794: f64 = (assign11510_e15792 * locals.var_dmcgeff);
        let assign11510_e15795: f64 = (assign11510_e15789 / assign11510_e15794);
        (assign11510_e15795,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11510_e15797;
        locals.var_rend_rv = 0.0;

        let (assign11520_e15824,) = {
    if ((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard257 != 0.0) && (!(((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0))))) && (locals.var_guard334 == 0.0)) && (locals.var_guard346 == 0.0)) && (!((locals.var_guard352 != 0.0) || (locals.var_guard353 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11520_e15824;
        locals.var_rend_rv = 0.0;

        let assign11530_e15827: f64 = if 0.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard357 = assign11530_e15827;
        locals.var_guard357_rv = 0.0;

        let assign11540_e15830: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard358 = assign11540_e15830;
        locals.var_guard358_rv = 0.0;

        let assign11550_e15841: f64 = if (((p.p9 == 1.0) || (p.p9 == 2.0)) || (p.p9 == 5.0)) { 1.0 } else { 0.0 };
        locals.var_guard359 = assign11550_e15841;
        locals.var_guard359_rv = 0.0;

        let assign11560_e15852: f64 = if (((p.p9 == 3.0) || (p.p9 == 4.0)) || (p.p9 == 6.0)) { 1.0 } else { 0.0 };
        locals.var_guard360 = assign11560_e15852;
        locals.var_guard360_rv = 0.0;

        let assign11570_e15855: f64 = if locals.var_nuends == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard361 = assign11570_e15855;
        locals.var_guard361_rv = 0.0;

        let (assign11580_e15881,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard258 != 0.0) && (!((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0))))) && (locals.var_guard357 != 0.0)) && (locals.var_guard358 != 0.0)) && (locals.var_guard359 != 0.0)) && (locals.var_guard361 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11580_e15881;
        locals.var_rend_rv = 0.0;

        let (assign11590_e15914,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard258 != 0.0) && (!((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0))))) && (locals.var_guard357 != 0.0)) && (locals.var_guard358 != 0.0)) && (locals.var_guard359 != 0.0)) && (locals.var_guard361 == 0.0)) {
        let assign11590_e15908: f64 = (p.p438 * locals.var_dmcgeff);
        let assign11590_e15911: f64 = (locals.var_weff * locals.var_nuends);
        let assign11590_e15912: f64 = (assign11590_e15908 / assign11590_e15911);
        (assign11590_e15912,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11590_e15914;
        locals.var_rend_rv = 0.0;

        let assign11610_e15925: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign11610_e15928: f64 = if ((locals.var_nuends == 0.0) || (assign11610_e15925 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard363 = assign11610_e15928;
        locals.var_guard363_rv = 0.0;

        let (assign11620_e15957,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard258 != 0.0) && (!((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0))))) && (locals.var_guard357 != 0.0)) && (locals.var_guard358 != 0.0)) && ((locals.var_guard360 != 0.0) && (locals.var_guard359 == 0.0))) && (locals.var_guard363 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11620_e15957;
        locals.var_rend_rv = 0.0;

        let (assign11630_e15997,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard258 != 0.0) && (!((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0))))) && (locals.var_guard357 != 0.0)) && (locals.var_guard358 != 0.0)) && ((locals.var_guard360 != 0.0) && (locals.var_guard359 == 0.0))) && (locals.var_guard363 == 0.0)) {
        let assign11630_e15987: f64 = (p.p438 * locals.var_weff);
        let assign11630_e15990: f64 = (3.0 * locals.var_nuends);
        let assign11630_e15993: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign11630_e15994: f64 = (assign11630_e15990 * assign11630_e15993);
        let assign11630_e15995: f64 = (assign11630_e15987 / assign11630_e15994);
        (assign11630_e15995,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11630_e15997;
        locals.var_rend_rv = 0.0;

        let (assign11640_e16024,) = {
    if ((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard258 != 0.0) && (!((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0))))) && (locals.var_guard357 != 0.0)) && (locals.var_guard358 != 0.0)) && (!((locals.var_guard359 != 0.0) || (locals.var_guard360 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11640_e16024;
        locals.var_rend_rv = 0.0;

        let assign11650_e16035: f64 = if (((p.p9 == 1.0) || (p.p9 == 3.0)) || (p.p9 == 7.0)) { 1.0 } else { 0.0 };
        locals.var_guard364 = assign11650_e16035;
        locals.var_guard364_rv = 0.0;

        let assign11660_e16046: f64 = if (((p.p9 == 2.0) || (p.p9 == 4.0)) || (p.p9 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard365 = assign11660_e16046;
        locals.var_guard365_rv = 0.0;

        let assign11670_e16049: f64 = if locals.var_nuends == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard366 = assign11670_e16049;
        locals.var_guard366_rv = 0.0;

        let (assign11680_e16076,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard258 != 0.0) && (!((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0))))) && (locals.var_guard357 != 0.0)) && (locals.var_guard358 == 0.0)) && (locals.var_guard364 != 0.0)) && (locals.var_guard366 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11680_e16076;
        locals.var_rend_rv = 0.0;

        let (assign11690_e16110,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard258 != 0.0) && (!((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0))))) && (locals.var_guard357 != 0.0)) && (locals.var_guard358 == 0.0)) && (locals.var_guard364 != 0.0)) && (locals.var_guard366 == 0.0)) {
        let assign11690_e16104: f64 = (p.p438 * locals.var_dmcgeff);
        let assign11690_e16107: f64 = (locals.var_weff * locals.var_nuends);
        let assign11690_e16108: f64 = (assign11690_e16104 / assign11690_e16107);
        (assign11690_e16108,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11690_e16110;
        locals.var_rend_rv = 0.0;

        let assign11710_e16121: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign11710_e16124: f64 = if ((locals.var_nuends == 0.0) || (assign11710_e16121 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard368 = assign11710_e16124;
        locals.var_guard368_rv = 0.0;

        let (assign11720_e16154,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard258 != 0.0) && (!((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0))))) && (locals.var_guard357 != 0.0)) && (locals.var_guard358 == 0.0)) && ((locals.var_guard365 != 0.0) && (locals.var_guard364 == 0.0))) && (locals.var_guard368 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11720_e16154;
        locals.var_rend_rv = 0.0;

        let (assign11730_e16195,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard258 != 0.0) && (!((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0))))) && (locals.var_guard357 != 0.0)) && (locals.var_guard358 == 0.0)) && ((locals.var_guard365 != 0.0) && (locals.var_guard364 == 0.0))) && (locals.var_guard368 == 0.0)) {
        let assign11730_e16185: f64 = (p.p438 * locals.var_weff);
        let assign11730_e16188: f64 = (3.0 * locals.var_nuends);
        let assign11730_e16191: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign11730_e16192: f64 = (assign11730_e16188 * assign11730_e16191);
        let assign11730_e16193: f64 = (assign11730_e16185 / assign11730_e16192);
        (assign11730_e16193,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11730_e16195;
        locals.var_rend_rv = 0.0;

        let (assign11740_e16223,) = {
    if ((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard258 != 0.0) && (!((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0))))) && (locals.var_guard357 != 0.0)) && (locals.var_guard358 == 0.0)) && (!((locals.var_guard364 != 0.0) || (locals.var_guard365 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11740_e16223;
        locals.var_rend_rv = 0.0;

        let (assign11750_e16248,) = {
    if ((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard258 != 0.0) && (!((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0))))) && (locals.var_guard357 == 0.0)) {
        let assign11750_e16244: f64 = (p.p438 * locals.var_dmdgeff);
        let assign11750_e16246: f64 = (assign11750_e16244 / locals.var_weff);
        (assign11750_e16246,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11750_e16248;
        locals.var_rend_rv = 0.0;

        let assign11760_e16251: f64 = if 0.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard369 = assign11760_e16251;
        locals.var_guard369_rv = 0.0;

        let assign11770_e16254: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard370 = assign11770_e16254;
        locals.var_guard370_rv = 0.0;

        let assign11780_e16265: f64 = if (((p.p9 == 1.0) || (p.p9 == 2.0)) || (p.p9 == 5.0)) { 1.0 } else { 0.0 };
        locals.var_guard371 = assign11780_e16265;
        locals.var_guard371_rv = 0.0;

        let assign11790_e16276: f64 = if (((p.p9 == 3.0) || (p.p9 == 4.0)) || (p.p9 == 6.0)) { 1.0 } else { 0.0 };
        locals.var_guard372 = assign11790_e16276;
        locals.var_guard372_rv = 0.0;

        let assign11800_e16279: f64 = if locals.var_nuends == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard373 = assign11800_e16279;
        locals.var_guard373_rv = 0.0;

        let (assign11810_e16307,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard259 != 0.0) && (!(((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0))))) && (locals.var_guard369 != 0.0)) && (locals.var_guard370 != 0.0)) && (locals.var_guard371 != 0.0)) && (locals.var_guard373 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11810_e16307;
        locals.var_rend_rv = 0.0;

        let (assign11820_e16342,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard259 != 0.0) && (!(((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0))))) && (locals.var_guard369 != 0.0)) && (locals.var_guard370 != 0.0)) && (locals.var_guard371 != 0.0)) && (locals.var_guard373 == 0.0)) {
        let assign11820_e16336: f64 = (p.p438 * locals.var_dmcgeff);
        let assign11820_e16339: f64 = (locals.var_weff * locals.var_nuends);
        let assign11820_e16340: f64 = (assign11820_e16336 / assign11820_e16339);
        (assign11820_e16340,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11820_e16342;
        locals.var_rend_rv = 0.0;

        let assign11840_e16352: f64 = if ((locals.var_nuends == 0.0) || (locals.var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard375 = assign11840_e16352;
        locals.var_guard375_rv = 0.0;

        let (assign11850_e16383,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard259 != 0.0) && (!(((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0))))) && (locals.var_guard369 != 0.0)) && (locals.var_guard370 != 0.0)) && ((locals.var_guard372 != 0.0) && (locals.var_guard371 == 0.0))) && (locals.var_guard375 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11850_e16383;
        locals.var_rend_rv = 0.0;

        let (assign11860_e16423,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard259 != 0.0) && (!(((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0))))) && (locals.var_guard369 != 0.0)) && (locals.var_guard370 != 0.0)) && ((locals.var_guard372 != 0.0) && (locals.var_guard371 == 0.0))) && (locals.var_guard375 == 0.0)) {
        let assign11860_e16415: f64 = (p.p438 * locals.var_weff);
        let assign11860_e16418: f64 = (6.0 * locals.var_nuends);
        let assign11860_e16420: f64 = (assign11860_e16418 * locals.var_dmcgeff);
        let assign11860_e16421: f64 = (assign11860_e16415 / assign11860_e16420);
        (assign11860_e16421,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11860_e16423;
        locals.var_rend_rv = 0.0;

        let (assign11870_e16452,) = {
    if ((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard259 != 0.0) && (!(((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0))))) && (locals.var_guard369 != 0.0)) && (locals.var_guard370 != 0.0)) && (!((locals.var_guard371 != 0.0) || (locals.var_guard372 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11870_e16452;
        locals.var_rend_rv = 0.0;

        let assign11880_e16463: f64 = if (((p.p9 == 1.0) || (p.p9 == 3.0)) || (p.p9 == 7.0)) { 1.0 } else { 0.0 };
        locals.var_guard376 = assign11880_e16463;
        locals.var_guard376_rv = 0.0;

        let assign11890_e16474: f64 = if (((p.p9 == 2.0) || (p.p9 == 4.0)) || (p.p9 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard377 = assign11890_e16474;
        locals.var_guard377_rv = 0.0;

        let assign11900_e16477: f64 = if locals.var_nuends == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard378 = assign11900_e16477;
        locals.var_guard378_rv = 0.0;

        let (assign11910_e16506,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard259 != 0.0) && (!(((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0))))) && (locals.var_guard369 != 0.0)) && (locals.var_guard370 == 0.0)) && (locals.var_guard376 != 0.0)) && (locals.var_guard378 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11910_e16506;
        locals.var_rend_rv = 0.0;

        let (assign11920_e16542,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard259 != 0.0) && (!(((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0))))) && (locals.var_guard369 != 0.0)) && (locals.var_guard370 == 0.0)) && (locals.var_guard376 != 0.0)) && (locals.var_guard378 == 0.0)) {
        let assign11920_e16536: f64 = (p.p438 * locals.var_dmcgeff);
        let assign11920_e16539: f64 = (locals.var_weff * locals.var_nuends);
        let assign11920_e16540: f64 = (assign11920_e16536 / assign11920_e16539);
        (assign11920_e16540,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11920_e16542;
        locals.var_rend_rv = 0.0;

        let assign11940_e16552: f64 = if ((locals.var_nuends == 0.0) || (locals.var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard380 = assign11940_e16552;
        locals.var_guard380_rv = 0.0;

        let (assign11950_e16584,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard259 != 0.0) && (!(((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0))))) && (locals.var_guard369 != 0.0)) && (locals.var_guard370 == 0.0)) && ((locals.var_guard377 != 0.0) && (locals.var_guard376 == 0.0))) && (locals.var_guard380 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11950_e16584;
        locals.var_rend_rv = 0.0;

        let (assign11960_e16625,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard259 != 0.0) && (!(((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0))))) && (locals.var_guard369 != 0.0)) && (locals.var_guard370 == 0.0)) && ((locals.var_guard377 != 0.0) && (locals.var_guard376 == 0.0))) && (locals.var_guard380 == 0.0)) {
        let assign11960_e16617: f64 = (p.p438 * locals.var_weff);
        let assign11960_e16620: f64 = (6.0 * locals.var_nuends);
        let assign11960_e16622: f64 = (assign11960_e16620 * locals.var_dmcgeff);
        let assign11960_e16623: f64 = (assign11960_e16617 / assign11960_e16622);
        (assign11960_e16623,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11960_e16625;
        locals.var_rend_rv = 0.0;

        let (assign11970_e16655,) = {
    if ((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard259 != 0.0) && (!(((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0))))) && (locals.var_guard369 != 0.0)) && (locals.var_guard370 == 0.0)) && (!((locals.var_guard376 != 0.0) || (locals.var_guard377 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11970_e16655;
        locals.var_rend_rv = 0.0;

        let assign11980_e16658: f64 = if locals.var_nuendd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard381 = assign11980_e16658;
        locals.var_guard381_rv = 0.0;

        let (assign11990_e16683,) = {
    if (((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard259 != 0.0) && (!(((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0))))) && (locals.var_guard369 == 0.0)) && (locals.var_guard381 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11990_e16683;
        locals.var_rend_rv = 0.0;

        let (assign12000_e16715,) = {
    if (((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard259 != 0.0) && (!(((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0))))) && (locals.var_guard369 == 0.0)) && (locals.var_guard381 == 0.0)) {
        let assign12000_e16709: f64 = (p.p438 * locals.var_dmdgeff);
        let assign12000_e16712: f64 = (locals.var_weff * locals.var_nuendd);
        let assign12000_e16713: f64 = (assign12000_e16709 / assign12000_e16712);
        (assign12000_e16713,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign12000_e16715;
        locals.var_rend_rv = 0.0;

        let assign12010_e16718: f64 = if 0.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard382 = assign12010_e16718;
        locals.var_guard382_rv = 0.0;

        let (assign12020_e16746,) = {
    if ((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard260 != 0.0) && (!((((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0))))) && (locals.var_guard382 != 0.0)) {
        let assign12020_e16742: f64 = (p.p438 * locals.var_dmdgeff);
        let assign12020_e16744: f64 = (assign12020_e16742 / locals.var_weff);
        (assign12020_e16744,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign12020_e16746;
        locals.var_rend_rv = 0.0;

        let assign12030_e16749: f64 = if 0.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard383 = assign12030_e16749;
        locals.var_guard383_rv = 0.0;

        let assign12040_e16760: f64 = if (((p.p9 == 1.0) || (p.p9 == 2.0)) || (p.p9 == 5.0)) { 1.0 } else { 0.0 };
        locals.var_guard384 = assign12040_e16760;
        locals.var_guard384_rv = 0.0;

        let assign12050_e16771: f64 = if (((p.p9 == 3.0) || (p.p9 == 4.0)) || (p.p9 == 6.0)) { 1.0 } else { 0.0 };
        locals.var_guard385 = assign12050_e16771;
        locals.var_guard385_rv = 0.0;

        let assign12060_e16774: f64 = if locals.var_nuendd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard386 = assign12060_e16774;
        locals.var_guard386_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_20(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign12070_e16805,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard260 != 0.0) && (!((((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0))))) && (locals.var_guard382 == 0.0)) && (locals.var_guard383 != 0.0)) && (locals.var_guard384 != 0.0)) && (locals.var_guard386 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign12070_e16805;
        locals.var_rend_rv = 0.0;

        let (assign12080_e16843,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard260 != 0.0) && (!((((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0))))) && (locals.var_guard382 == 0.0)) && (locals.var_guard383 != 0.0)) && (locals.var_guard384 != 0.0)) && (locals.var_guard386 == 0.0)) {
        let assign12080_e16837: f64 = (p.p438 * locals.var_dmcgeff);
        let assign12080_e16840: f64 = (locals.var_weff * locals.var_nuendd);
        let assign12080_e16841: f64 = (assign12080_e16837 / assign12080_e16840);
        (assign12080_e16841,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign12080_e16843;
        locals.var_rend_rv = 0.0;

        let assign12100_e16854: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign12100_e16857: f64 = if ((locals.var_nuendd == 0.0) || (assign12100_e16854 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard388 = assign12100_e16857;
        locals.var_guard388_rv = 0.0;

        let (assign12110_e16891,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard260 != 0.0) && (!((((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0))))) && (locals.var_guard382 == 0.0)) && (locals.var_guard383 != 0.0)) && ((locals.var_guard385 != 0.0) && (locals.var_guard384 == 0.0))) && (locals.var_guard388 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign12110_e16891;
        locals.var_rend_rv = 0.0;

        let (assign12120_e16936,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard260 != 0.0) && (!((((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0))))) && (locals.var_guard382 == 0.0)) && (locals.var_guard383 != 0.0)) && ((locals.var_guard385 != 0.0) && (locals.var_guard384 == 0.0))) && (locals.var_guard388 == 0.0)) {
        let assign12120_e16926: f64 = (p.p438 * locals.var_weff);
        let assign12120_e16929: f64 = (3.0 * locals.var_nuendd);
        let assign12120_e16932: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign12120_e16933: f64 = (assign12120_e16929 * assign12120_e16932);
        let assign12120_e16934: f64 = (assign12120_e16926 / assign12120_e16933);
        (assign12120_e16934,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign12120_e16936;
        locals.var_rend_rv = 0.0;

        let (assign12130_e16968,) = {
    if ((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard260 != 0.0) && (!((((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0))))) && (locals.var_guard382 == 0.0)) && (locals.var_guard383 != 0.0)) && (!((locals.var_guard384 != 0.0) || (locals.var_guard385 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign12130_e16968;
        locals.var_rend_rv = 0.0;

        let assign12140_e16979: f64 = if (((p.p9 == 1.0) || (p.p9 == 3.0)) || (p.p9 == 7.0)) { 1.0 } else { 0.0 };
        locals.var_guard389 = assign12140_e16979;
        locals.var_guard389_rv = 0.0;

        let assign12150_e16990: f64 = if (((p.p9 == 2.0) || (p.p9 == 4.0)) || (p.p9 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard390 = assign12150_e16990;
        locals.var_guard390_rv = 0.0;

        let assign12160_e16993: f64 = if locals.var_nuendd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard391 = assign12160_e16993;
        locals.var_guard391_rv = 0.0;

        let (assign12170_e17025,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard260 != 0.0) && (!((((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0))))) && (locals.var_guard382 == 0.0)) && (locals.var_guard383 == 0.0)) && (locals.var_guard389 != 0.0)) && (locals.var_guard391 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign12170_e17025;
        locals.var_rend_rv = 0.0;

        let (assign12180_e17064,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard260 != 0.0) && (!((((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0))))) && (locals.var_guard382 == 0.0)) && (locals.var_guard383 == 0.0)) && (locals.var_guard389 != 0.0)) && (locals.var_guard391 == 0.0)) {
        let assign12180_e17058: f64 = (p.p438 * locals.var_dmcgeff);
        let assign12180_e17061: f64 = (locals.var_weff * locals.var_nuendd);
        let assign12180_e17062: f64 = (assign12180_e17058 / assign12180_e17061);
        (assign12180_e17062,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign12180_e17064;
        locals.var_rend_rv = 0.0;

        let assign12200_e17075: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign12200_e17078: f64 = if ((locals.var_nuendd == 0.0) || (assign12200_e17075 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard393 = assign12200_e17078;
        locals.var_guard393_rv = 0.0;

        let (assign12210_e17113,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard260 != 0.0) && (!((((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0))))) && (locals.var_guard382 == 0.0)) && (locals.var_guard383 == 0.0)) && ((locals.var_guard390 != 0.0) && (locals.var_guard389 == 0.0))) && (locals.var_guard393 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign12210_e17113;
        locals.var_rend_rv = 0.0;

        let (assign12220_e17159,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard260 != 0.0) && (!((((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0))))) && (locals.var_guard382 == 0.0)) && (locals.var_guard383 == 0.0)) && ((locals.var_guard390 != 0.0) && (locals.var_guard389 == 0.0))) && (locals.var_guard393 == 0.0)) {
        let assign12220_e17149: f64 = (p.p438 * locals.var_weff);
        let assign12220_e17152: f64 = (3.0 * locals.var_nuendd);
        let assign12220_e17155: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign12220_e17156: f64 = (assign12220_e17152 * assign12220_e17155);
        let assign12220_e17157: f64 = (assign12220_e17149 / assign12220_e17156);
        (assign12220_e17157,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign12220_e17159;
        locals.var_rend_rv = 0.0;

        let (assign12230_e17192,) = {
    if ((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard260 != 0.0) && (!((((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0))))) && (locals.var_guard382 == 0.0)) && (locals.var_guard383 == 0.0)) && (!((locals.var_guard389 != 0.0) || (locals.var_guard390 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign12230_e17192;
        locals.var_rend_rv = 0.0;

        let assign12240_e17195: f64 = if 0.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard394 = assign12240_e17195;
        locals.var_guard394_rv = 0.0;

        let assign12250_e17198: f64 = if locals.var_nuends == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard395 = assign12250_e17198;
        locals.var_guard395_rv = 0.0;

        let (assign12260_e17226,) = {
    if (((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard261 != 0.0) && (!(((((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0)) || (locals.var_guard260 != 0.0))))) && (locals.var_guard394 != 0.0)) && (locals.var_guard395 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign12260_e17226;
        locals.var_rend_rv = 0.0;

        let (assign12270_e17261,) = {
    if (((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard261 != 0.0) && (!(((((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0)) || (locals.var_guard260 != 0.0))))) && (locals.var_guard394 != 0.0)) && (locals.var_guard395 == 0.0)) {
        let assign12270_e17255: f64 = (p.p438 * locals.var_dmdgeff);
        let assign12270_e17258: f64 = (locals.var_weff * locals.var_nuends);
        let assign12270_e17259: f64 = (assign12270_e17255 / assign12270_e17258);
        (assign12270_e17259,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign12270_e17261;
        locals.var_rend_rv = 0.0;

        let assign12280_e17264: f64 = if 0.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard396 = assign12280_e17264;
        locals.var_guard396_rv = 0.0;

        let assign12290_e17275: f64 = if (((p.p9 == 1.0) || (p.p9 == 2.0)) || (p.p9 == 5.0)) { 1.0 } else { 0.0 };
        locals.var_guard397 = assign12290_e17275;
        locals.var_guard397_rv = 0.0;

        let assign12300_e17286: f64 = if (((p.p9 == 3.0) || (p.p9 == 4.0)) || (p.p9 == 6.0)) { 1.0 } else { 0.0 };
        locals.var_guard398 = assign12300_e17286;
        locals.var_guard398_rv = 0.0;

        let assign12310_e17289: f64 = if locals.var_nuendd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard399 = assign12310_e17289;
        locals.var_guard399_rv = 0.0;

        let (assign12320_e17322,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard261 != 0.0) && (!(((((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0)) || (locals.var_guard260 != 0.0))))) && (locals.var_guard394 == 0.0)) && (locals.var_guard396 != 0.0)) && (locals.var_guard397 != 0.0)) && (locals.var_guard399 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign12320_e17322;
        locals.var_rend_rv = 0.0;

        let (assign12330_e17362,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard261 != 0.0) && (!(((((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0)) || (locals.var_guard260 != 0.0))))) && (locals.var_guard394 == 0.0)) && (locals.var_guard396 != 0.0)) && (locals.var_guard397 != 0.0)) && (locals.var_guard399 == 0.0)) {
        let assign12330_e17356: f64 = (p.p438 * locals.var_dmcgeff);
        let assign12330_e17359: f64 = (locals.var_weff * locals.var_nuendd);
        let assign12330_e17360: f64 = (assign12330_e17356 / assign12330_e17359);
        (assign12330_e17360,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign12330_e17362;
        locals.var_rend_rv = 0.0;

        let assign12350_e17372: f64 = if ((locals.var_nuendd == 0.0) || (locals.var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard401 = assign12350_e17372;
        locals.var_guard401_rv = 0.0;

        let (assign12360_e17408,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard261 != 0.0) && (!(((((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0)) || (locals.var_guard260 != 0.0))))) && (locals.var_guard394 == 0.0)) && (locals.var_guard396 != 0.0)) && ((locals.var_guard398 != 0.0) && (locals.var_guard397 == 0.0))) && (locals.var_guard401 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign12360_e17408;
        locals.var_rend_rv = 0.0;

        let (assign12370_e17453,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard261 != 0.0) && (!(((((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0)) || (locals.var_guard260 != 0.0))))) && (locals.var_guard394 == 0.0)) && (locals.var_guard396 != 0.0)) && ((locals.var_guard398 != 0.0) && (locals.var_guard397 == 0.0))) && (locals.var_guard401 == 0.0)) {
        let assign12370_e17445: f64 = (p.p438 * locals.var_weff);
        let assign12370_e17448: f64 = (6.0 * locals.var_nuendd);
        let assign12370_e17450: f64 = (assign12370_e17448 * locals.var_dmcgeff);
        let assign12370_e17451: f64 = (assign12370_e17445 / assign12370_e17450);
        (assign12370_e17451,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign12370_e17453;
        locals.var_rend_rv = 0.0;

        let (assign12380_e17487,) = {
    if ((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard261 != 0.0) && (!(((((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0)) || (locals.var_guard260 != 0.0))))) && (locals.var_guard394 == 0.0)) && (locals.var_guard396 != 0.0)) && (!((locals.var_guard397 != 0.0) || (locals.var_guard398 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign12380_e17487;
        locals.var_rend_rv = 0.0;

        let assign12390_e17498: f64 = if (((p.p9 == 1.0) || (p.p9 == 3.0)) || (p.p9 == 7.0)) { 1.0 } else { 0.0 };
        locals.var_guard402 = assign12390_e17498;
        locals.var_guard402_rv = 0.0;

        let assign12400_e17509: f64 = if (((p.p9 == 2.0) || (p.p9 == 4.0)) || (p.p9 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard403 = assign12400_e17509;
        locals.var_guard403_rv = 0.0;

        let assign12410_e17512: f64 = if locals.var_nuendd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard404 = assign12410_e17512;
        locals.var_guard404_rv = 0.0;

        let (assign12420_e17546,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard261 != 0.0) && (!(((((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0)) || (locals.var_guard260 != 0.0))))) && (locals.var_guard394 == 0.0)) && (locals.var_guard396 == 0.0)) && (locals.var_guard402 != 0.0)) && (locals.var_guard404 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign12420_e17546;
        locals.var_rend_rv = 0.0;

        let (assign12430_e17587,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard261 != 0.0) && (!(((((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0)) || (locals.var_guard260 != 0.0))))) && (locals.var_guard394 == 0.0)) && (locals.var_guard396 == 0.0)) && (locals.var_guard402 != 0.0)) && (locals.var_guard404 == 0.0)) {
        let assign12430_e17581: f64 = (p.p438 * locals.var_dmcgeff);
        let assign12430_e17584: f64 = (locals.var_weff * locals.var_nuendd);
        let assign12430_e17585: f64 = (assign12430_e17581 / assign12430_e17584);
        (assign12430_e17585,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign12430_e17587;
        locals.var_rend_rv = 0.0;

        let assign12450_e17597: f64 = if ((locals.var_nuendd == 0.0) || (locals.var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard406 = assign12450_e17597;
        locals.var_guard406_rv = 0.0;

        let (assign12460_e17634,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard261 != 0.0) && (!(((((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0)) || (locals.var_guard260 != 0.0))))) && (locals.var_guard394 == 0.0)) && (locals.var_guard396 == 0.0)) && ((locals.var_guard403 != 0.0) && (locals.var_guard402 == 0.0))) && (locals.var_guard406 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign12460_e17634;
        locals.var_rend_rv = 0.0;

        let (assign12470_e17680,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard261 != 0.0) && (!(((((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0)) || (locals.var_guard260 != 0.0))))) && (locals.var_guard394 == 0.0)) && (locals.var_guard396 == 0.0)) && ((locals.var_guard403 != 0.0) && (locals.var_guard402 == 0.0))) && (locals.var_guard406 == 0.0)) {
        let assign12470_e17672: f64 = (p.p438 * locals.var_weff);
        let assign12470_e17675: f64 = (6.0 * locals.var_nuendd);
        let assign12470_e17677: f64 = (assign12470_e17675 * locals.var_dmcgeff);
        let assign12470_e17678: f64 = (assign12470_e17672 / assign12470_e17677);
        (assign12470_e17678,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign12470_e17680;
        locals.var_rend_rv = 0.0;

        let (assign12480_e17715,) = {
    if ((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard261 != 0.0) && (!(((((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0)) || (locals.var_guard260 != 0.0))))) && (locals.var_guard394 == 0.0)) && (locals.var_guard396 == 0.0)) && (!((locals.var_guard402 != 0.0) || (locals.var_guard403 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign12480_e17715;
        locals.var_rend_rv = 0.0;

        let (assign12490_e17745,) = {
    if (((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard262 != 0.0) && (!((((((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0)) || (locals.var_guard260 != 0.0)) || (locals.var_guard261 != 0.0))))) {
        let assign12490_e17741: f64 = (p.p438 * locals.var_dmdgeff);
        let assign12490_e17743: f64 = (assign12490_e17741 / locals.var_weff);
        (assign12490_e17743,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign12490_e17745;
        locals.var_rend_rv = 0.0;

        let assign12500_e17748: f64 = if 0.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard407 = assign12500_e17748;
        locals.var_guard407_rv = 0.0;

        let (assign12510_e17784,) = {
    if ((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard263 != 0.0) && (!(((((((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0)) || (locals.var_guard260 != 0.0)) || (locals.var_guard261 != 0.0)) || (locals.var_guard262 != 0.0))))) && (locals.var_guard407 != 0.0)) {
        let assign12510_e17778: f64 = (0.5 * p.p438);
        let assign12510_e17780: f64 = (assign12510_e17778 * locals.var_dmcgeff);
        let assign12510_e17782: f64 = (assign12510_e17780 / locals.var_weff);
        (assign12510_e17782,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign12510_e17784;
        locals.var_rend_rv = 0.0;

        let assign12520_e17787: f64 = if p.p2 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard408 = assign12520_e17787;
        locals.var_guard408_rv = 0.0;

        let (assign12530_e17819,) = {
    if (((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard263 != 0.0) && (!(((((((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0)) || (locals.var_guard260 != 0.0)) || (locals.var_guard261 != 0.0)) || (locals.var_guard262 != 0.0))))) && (locals.var_guard407 != 0.0)) && (locals.var_guard408 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rint,)
    }
};
        locals.var_rint = assign12530_e17819;
        locals.var_rint_rv = 0.0;

        let (assign12540_e17860,) = {
    if (((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard263 != 0.0) && (!(((((((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0)) || (locals.var_guard260 != 0.0)) || (locals.var_guard261 != 0.0)) || (locals.var_guard262 != 0.0))))) && (locals.var_guard407 != 0.0)) && (locals.var_guard408 == 0.0)) {
        let assign12540_e17852: f64 = (p.p438 * locals.var_dmcgeff);
        let assign12540_e17856: f64 = (p.p2 - 2.0);
        let assign12540_e17857: f64 = (locals.var_weff * assign12540_e17856);
        let assign12540_e17858: f64 = (assign12540_e17852 / assign12540_e17857);
        (assign12540_e17858,)
    } else {
        (locals.var_rint,)
    }
};
        locals.var_rint = assign12540_e17860;
        locals.var_rint_rv = 0.0;

        let (assign12550_e17891,) = {
    if ((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard263 != 0.0) && (!(((((((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0)) || (locals.var_guard260 != 0.0)) || (locals.var_guard261 != 0.0)) || (locals.var_guard262 != 0.0))))) && (locals.var_guard407 == 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign12550_e17891;
        locals.var_rend_rv = 0.0;

        let (assign12560_e17928,) = {
    if ((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard263 != 0.0) && (!(((((((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0)) || (locals.var_guard260 != 0.0)) || (locals.var_guard261 != 0.0)) || (locals.var_guard262 != 0.0))))) && (locals.var_guard407 == 0.0)) {
        let assign12560_e17922: f64 = (p.p438 * locals.var_dmcgeff);
        let assign12560_e17925: f64 = (locals.var_weff * p.p2);
        let assign12560_e17926: f64 = (assign12560_e17922 / assign12560_e17925);
        (assign12560_e17926,)
    } else {
        (locals.var_rint,)
    }
};
        locals.var_rint = assign12560_e17928;
        locals.var_rint_rv = 0.0;

        let assign12570_e17931: f64 = if 0.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard409 = assign12570_e17931;
        locals.var_guard409_rv = 0.0;

        let (assign12580_e17963,) = {
    if ((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard264 != 0.0) && (!((((((((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0)) || (locals.var_guard260 != 0.0)) || (locals.var_guard261 != 0.0)) || (locals.var_guard262 != 0.0)) || (locals.var_guard263 != 0.0))))) && (locals.var_guard409 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign12580_e17963;
        locals.var_rend_rv = 0.0;

        let (assign12590_e18001,) = {
    if ((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard264 != 0.0) && (!((((((((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0)) || (locals.var_guard260 != 0.0)) || (locals.var_guard261 != 0.0)) || (locals.var_guard262 != 0.0)) || (locals.var_guard263 != 0.0))))) && (locals.var_guard409 != 0.0)) {
        let assign12590_e17995: f64 = (p.p438 * locals.var_dmcgeff);
        let assign12590_e17998: f64 = (locals.var_weff * p.p2);
        let assign12590_e17999: f64 = (assign12590_e17995 / assign12590_e17998);
        (assign12590_e17999,)
    } else {
        (locals.var_rint,)
    }
};
        locals.var_rint = assign12590_e18001;
        locals.var_rint_rv = 0.0;

        let (assign12600_e18040,) = {
    if ((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard264 != 0.0) && (!((((((((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0)) || (locals.var_guard260 != 0.0)) || (locals.var_guard261 != 0.0)) || (locals.var_guard262 != 0.0)) || (locals.var_guard263 != 0.0))))) && (locals.var_guard409 == 0.0)) {
        let assign12600_e18034: f64 = (0.5 * p.p438);
        let assign12600_e18036: f64 = (assign12600_e18034 * locals.var_dmcgeff);
        let assign12600_e18038: f64 = (assign12600_e18036 / locals.var_weff);
        (assign12600_e18038,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign12600_e18040;
        locals.var_rend_rv = 0.0;

        let assign12610_e18043: f64 = if p.p2 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard410 = assign12610_e18043;
        locals.var_guard410_rv = 0.0;

        let (assign12620_e18078,) = {
    if (((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard264 != 0.0) && (!((((((((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0)) || (locals.var_guard260 != 0.0)) || (locals.var_guard261 != 0.0)) || (locals.var_guard262 != 0.0)) || (locals.var_guard263 != 0.0))))) && (locals.var_guard409 == 0.0)) && (locals.var_guard410 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rint,)
    }
};
        locals.var_rint = assign12620_e18078;
        locals.var_rint_rv = 0.0;

        let (assign12630_e18122,) = {
    if (((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard264 != 0.0) && (!((((((((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0)) || (locals.var_guard260 != 0.0)) || (locals.var_guard261 != 0.0)) || (locals.var_guard262 != 0.0)) || (locals.var_guard263 != 0.0))))) && (locals.var_guard409 == 0.0)) && (locals.var_guard410 == 0.0)) {
        let assign12630_e18114: f64 = (p.p438 * locals.var_dmcgeff);
        let assign12630_e18118: f64 = (p.p2 - 2.0);
        let assign12630_e18119: f64 = (locals.var_weff * assign12630_e18118);
        let assign12630_e18120: f64 = (assign12630_e18114 / assign12630_e18119);
        (assign12630_e18120,)
    } else {
        (locals.var_rint,)
    }
};
        locals.var_rint = assign12630_e18122;
        locals.var_rint_rv = 0.0;

        let (assign12640_e18152,) = {
    if (((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && (!(((((((((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0)) || (locals.var_guard260 != 0.0)) || (locals.var_guard261 != 0.0)) || (locals.var_guard262 != 0.0)) || (locals.var_guard263 != 0.0)) || (locals.var_guard264 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rint,)
    }
};
        locals.var_rint = assign12640_e18152;
        locals.var_rint_rv = 0.0;

        let assign12650_e18155: f64 = if locals.var_rint <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard411 = assign12650_e18155;
        locals.var_guard411_rv = 0.0;

        let (assign12660_e18164,) = {
    if (((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && (locals.var_guard411 != 0.0)) {
        (locals.var_rend,)
    } else {
        (locals.var_rdraingeo,)
    }
};
        locals.var_rdraingeo = assign12660_e18164;
        locals.var_rdraingeo_rv = 0.0;

        let assign12670_e18167: f64 = if locals.var_rend <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard412 = assign12670_e18167;
        locals.var_guard412_rv = 0.0;

        let (assign12680_e18179,) = {
    if ((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && (locals.var_guard411 == 0.0)) && (locals.var_guard412 != 0.0)) {
        (locals.var_rint,)
    } else {
        (locals.var_rdraingeo,)
    }
};
        locals.var_rdraingeo = assign12680_e18179;
        locals.var_rdraingeo_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_21(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        locals: &mut StampLocals,
    ) {
        let ctx_temp = ctx.temperature();
        let nv4 = ctx.node_voltage(nodes[4]);
        let nv5 = ctx.node_voltage(nodes[5]);
        let (assign12690_e18198,) = {
    if ((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && (locals.var_guard411 == 0.0)) && (locals.var_guard412 == 0.0)) {
        let assign12690_e18192: f64 = (locals.var_rint * locals.var_rend);
        let assign12690_e18195: f64 = (locals.var_rint + locals.var_rend);
        let assign12690_e18196: f64 = (assign12690_e18192 / assign12690_e18195);
        (assign12690_e18196,)
    } else {
        (locals.var_rdraingeo,)
    }
};
        locals.var_rdraingeo = assign12690_e18198;
        locals.var_rdraingeo_rv = 0.0;

        let (assign12710_e18209,) = {
    if ((locals.var_guard246 == 0.0) && (locals.var_guard247 == 0.0)) {
        (0.0,)
    } else {
        (locals.var_rdraingeo,)
    }
};
        locals.var_rdraingeo = assign12710_e18209;
        locals.var_rdraingeo_rv = 0.0;

        let assign12720_e18212: f64 = if p.p33 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard414 = assign12720_e18212;
        locals.var_guard414_rv = 0.0;

        let assign12730_e18215: f64 = if locals.var_rsourcegeo < p.p1347 { 1.0 } else { 0.0 };
        locals.var_guard415 = assign12730_e18215;
        locals.var_guard415_rv = 0.0;

        let (assign12740_e18221,) = {
    if ((locals.var_guard414 != 0.0) && (locals.var_guard415 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rsourcegeo,)
    }
};
        locals.var_rsourcegeo = assign12740_e18221;
        locals.var_rsourcegeo_rv = 0.0;

        let assign12750_e18224: f64 = if locals.var_rdraingeo < p.p1347 { 1.0 } else { 0.0 };
        locals.var_guard416 = assign12750_e18224;
        locals.var_guard416_rv = 0.0;

        let (assign12760_e18230,) = {
    if ((locals.var_guard414 != 0.0) && (locals.var_guard416 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rdraingeo,)
    }
};
        locals.var_rdraingeo = assign12760_e18230;
        locals.var_rdraingeo_rv = 0.0;

        let assign12770_e18233: f64 = if locals.var_rsourcegeo <= p.p1347 { 1.0 } else { 0.0 };
        locals.var_guard417 = assign12770_e18233;
        locals.var_guard417_rv = 0.0;

        let (assign12780_e18240,) = {
    if ((locals.var_guard414 == 0.0) && (locals.var_guard417 != 0.0)) {
        (p.p1347,)
    } else {
        (locals.var_rsourcegeo,)
    }
};
        locals.var_rsourcegeo = assign12780_e18240;
        locals.var_rsourcegeo_rv = 0.0;

        let assign12790_e18243: f64 = if locals.var_rdraingeo <= p.p1347 { 1.0 } else { 0.0 };
        locals.var_guard418 = assign12790_e18243;
        locals.var_guard418_rv = 0.0;

        let (assign12800_e18250,) = {
    if ((locals.var_guard414 == 0.0) && (locals.var_guard418 != 0.0)) {
        (p.p1347,)
    } else {
        (locals.var_rdraingeo,)
    }
};
        locals.var_rdraingeo = assign12800_e18250;
        locals.var_rdraingeo_rv = 0.0;

        let assign12810_e18253: f64 = if p.p33 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard419 = assign12810_e18253;
        locals.var_guard419_rv = 0.0;

        let assign12820_e18256: f64 = if locals.var_rswmin_i <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard420 = assign12820_e18256;
        locals.var_guard420_rv = 0.0;

        let (assign12830_e18262,) = {
    if ((locals.var_guard419 != 0.0) && (locals.var_guard420 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rswmin_i,)
    }
};
        locals.var_rswmin_i = assign12830_e18262;
        locals.var_rswmin_i_rv = 0.0;

        let assign12840_e18265: f64 = if locals.var_rdwmin_i <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard421 = assign12840_e18265;
        locals.var_guard421_rv = 0.0;

        let (assign12850_e18271,) = {
    if ((locals.var_guard419 != 0.0) && (locals.var_guard421 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rdwmin_i,)
    }
};
        locals.var_rdwmin_i = assign12850_e18271;
        locals.var_rdwmin_i_rv = 0.0;

        let assign12860_e18274: f64 = if locals.var_rsw_i <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard422 = assign12860_e18274;
        locals.var_guard422_rv = 0.0;

        let (assign12870_e18280,) = {
    if ((locals.var_guard419 != 0.0) && (locals.var_guard422 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rsw_i,)
    }
};
        locals.var_rsw_i = assign12870_e18280;
        locals.var_rsw_i_rv = 0.0;

        let assign12880_e18283: f64 = if locals.var_rdw_i <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard423 = assign12880_e18283;
        locals.var_guard423_rv = 0.0;

        let (assign12890_e18289,) = {
    if ((locals.var_guard419 != 0.0) && (locals.var_guard423 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rdw_i,)
    }
};
        locals.var_rdw_i = assign12890_e18289;
        locals.var_rdw_i_rv = 0.0;

        let assign12900_e18292: f64 = if locals.var_rdswmin_i <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard424 = assign12900_e18292;
        locals.var_guard424_rv = 0.0;

        let (assign12910_e18299,) = {
    if ((locals.var_guard419 == 0.0) && (locals.var_guard424 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rdswmin_i,)
    }
};
        locals.var_rdswmin_i = assign12910_e18299;
        locals.var_rdswmin_i_rv = 0.0;

        let assign12920_e18302: f64 = if locals.var_rdsw_i <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard425 = assign12920_e18302;
        locals.var_guard425_rv = 0.0;

        let (assign12930_e18309,) = {
    if ((locals.var_guard419 == 0.0) && (locals.var_guard425 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rdsw_i,)
    }
};
        locals.var_rdsw_i = assign12930_e18309;
        locals.var_rdsw_i_rv = 0.0;

        let assign12940_e18314: f64 = (locals.var_weffcj / 3.0);
        let assign12940_e18316: f64 = (assign12940_e18314 / p.p22);
        let assign12940_e18317: f64 = (p.p21 + assign12940_e18316);
        let assign12940_e18318: f64 = (p.p900 * assign12940_e18317);
        let assign12940_e18321: f64 = (p.p22 * p.p2);
        let assign12940_e18324: f64 = (locals.var_lnew - p.p899);
        let assign12940_e18325: f64 = (assign12940_e18321 * assign12940_e18324);
        let assign12940_e18326: f64 = (assign12940_e18318 / assign12940_e18325);
        locals.var_grgeltd = assign12940_e18326;
        locals.var_grgeltd_rv = 0.0;

        let assign12950_e18329: f64 = if locals.var_grgeltd > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard426 = assign12950_e18329;
        locals.var_guard426_rv = 0.0;

        let (assign12960_e18335,) = {
    if (locals.var_guard426 != 0.0) {
        let assign12960_e18333: f64 = (1.0 / locals.var_grgeltd);
        (assign12960_e18333,)
    } else {
        (locals.var_grgeltd,)
    }
};
        locals.var_grgeltd = assign12960_e18335;
        locals.var_grgeltd_rv = 0.0;

        let (assign12970_e18340,) = {
    if (locals.var_guard426 == 0.0) {
        (1000.0,)
    } else {
        (locals.var_grgeltd,)
    }
};
        locals.var_grgeltd = assign12970_e18340;
        locals.var_grgeltd_rv = 0.0;

        let assign12990_e18346: f64 = (p.p76 * p.p76);
        locals.var_t0 = assign12990_e18346;
        locals.var_t0_dn3 = 0.0;
        locals.var_t0_dn4 = 0.0;
        locals.var_t0_dn5 = 0.0;
        locals.var_t0_dn6 = 0.0;
        locals.var_t0_dn7 = 0.0;
        locals.var_t0_dn8 = 0.0;
        locals.var_t0_dn9 = 0.0;
        locals.var_t0_dn10 = 0.0;
        locals.var_t0_dn11 = 0.0;
        locals.var_t0_rv = 0.0;

        let assign13000_e18349: f64 = (p.p76 * locals.var_poxedge_i);
        locals.var_t1 = assign13000_e18349;
        locals.var_t1_dn3 = 0.0;
        locals.var_t1_dn4 = 0.0;
        locals.var_t1_dn5 = 0.0;
        locals.var_t1_dn6 = 0.0;
        locals.var_t1_dn7 = 0.0;
        locals.var_t1_dn8 = 0.0;
        locals.var_t1_dn9 = 0.0;
        locals.var_t1_dn10 = 0.0;
        locals.var_t1_dn11 = 0.0;
        locals.var_t1_rv = 0.0;

        let assign13010_e18352: f64 = (locals.var_t1 * locals.var_t1);
        locals.var_t2 = assign13010_e18352;
        locals.var_t2_dn3 = ((locals.var_t1_dn3 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn3));
        locals.var_t2_dn4 = ((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4));
        locals.var_t2_dn5 = ((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5));
        locals.var_t2_dn6 = ((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6));
        locals.var_t2_dn7 = ((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7));
        locals.var_t2_dn8 = ((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8));
        locals.var_t2_dn9 = ((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9));
        locals.var_t2_dn10 = ((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10));
        locals.var_t2_dn11 = ((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11));
        locals.var_t2_rv = 0.0;

        let assign13020_e18356: f64 = (p.p722 / p.p76);
        let assign13020_e18358: f64 = (assign13020_e18356).max(1e-38);
        let assign13020_e18359: f64 = (assign13020_e18358).ln();
        let assign13020_e18360: f64 = (locals.var_ntox_i * assign13020_e18359);
        let assign13020_e18361: f64 = { let limited_exp_arg = assign13020_e18360; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign13020_e18363: f64 = (assign13020_e18361 / locals.var_t0);
        locals.var_toxratio = assign13020_e18363;
        locals.var_toxratio_dn3 = (-((assign13020_e18361 * locals.var_t0_dn3) / (locals.var_t0 * locals.var_t0)));
        locals.var_toxratio_dn4 = (-((assign13020_e18361 * locals.var_t0_dn4) / (locals.var_t0 * locals.var_t0)));
        locals.var_toxratio_dn5 = (-((assign13020_e18361 * locals.var_t0_dn5) / (locals.var_t0 * locals.var_t0)));
        locals.var_toxratio_dn6 = (-((assign13020_e18361 * locals.var_t0_dn6) / (locals.var_t0 * locals.var_t0)));
        locals.var_toxratio_dn7 = (-((assign13020_e18361 * locals.var_t0_dn7) / (locals.var_t0 * locals.var_t0)));
        locals.var_toxratio_dn8 = (-((assign13020_e18361 * locals.var_t0_dn8) / (locals.var_t0 * locals.var_t0)));
        locals.var_toxratio_dn9 = (-((assign13020_e18361 * locals.var_t0_dn9) / (locals.var_t0 * locals.var_t0)));
        locals.var_toxratio_dn10 = (-((assign13020_e18361 * locals.var_t0_dn10) / (locals.var_t0 * locals.var_t0)));
        locals.var_toxratio_dn11 = (-((assign13020_e18361 * locals.var_t0_dn11) / (locals.var_t0 * locals.var_t0)));
        locals.var_toxratio_rv = 0.0;

        let (assign13050_e18386,) = {
    if (p.p30 == 1.0) {
        (p.p705,)
    } else {
        (p.p704,)
    }
};
        locals.var_bechvb = assign13050_e18386;
        locals.var_bechvb_rv = 0.0;

        let assign13080_e18406: f64 = (-locals.var_bechvb);
        let assign13080_e18408: f64 = (assign13080_e18406 * p.p76);
        let assign13080_e18410: f64 = (assign13080_e18408 * locals.var_poxedge_i);
        locals.var_bechvbedge = assign13080_e18410;
        locals.var_bechvbedge_rv = 0.0;

        let assign13100_e18425: f64 = (-locals.var_bechvb);
        let assign13100_e18427: f64 = (assign13100_e18425 * p.p76);
        locals.var_bechvb = assign13100_e18427;
        locals.var_bechvb_rv = 0.0;

        let assign13110_e18430: f64 = (p.p1101 + locals.var_weff);
        locals.var_weff_sh = assign13110_e18430;
        locals.var_weff_sh_rv = 0.0;

        let assign13150_e18459: f64 = if (((p.p41 != 0.0) && (p.p1099 > 0.0)) && (locals.var_weff_sh > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard431 = assign13150_e18459;
        locals.var_guard431_rv = 0.0;

        let (assign13160_e18467,) = {
    if (locals.var_guard431 != 0.0) {
        let assign13160_e18463: f64 = (locals.var_weff_sh * p.p2);
        let assign13160_e18465: f64 = (assign13160_e18463 / p.p1099);
        (assign13160_e18465,)
    } else {
        (locals.var_gth,)
    }
};
        locals.var_gth = assign13160_e18467;
        locals.var_gth_rv = 0.0;

        let (assign13170_e18475,) = {
    if (locals.var_guard431 != 0.0) {
        let assign13170_e18471: f64 = (p.p1100 * locals.var_weff_sh);
        let assign13170_e18473: f64 = (assign13170_e18471 * p.p2);
        (assign13170_e18473,)
    } else {
        (locals.var_cth,)
    }
};
        locals.var_cth = assign13170_e18475;
        locals.var_cth_rv = 0.0;

        let (assign13180_e18480,) = {
    if (locals.var_guard431 == 0.0) {
        (1.0,)
    } else {
        (locals.var_gth,)
    }
};
        locals.var_gth = assign13180_e18480;
        locals.var_gth_rv = 0.0;

        let (assign13190_e18485,) = {
    if (locals.var_guard431 == 0.0) {
        (0.0,)
    } else {
        (locals.var_cth,)
    }
};
        locals.var_cth = assign13190_e18485;
        locals.var_cth_rv = 0.0;

        let assign13200_e18488: f64 = (-273.15);
        let assign13200_e18489: f64 = if p.p1028 <= assign13200_e18488 { 1.0 } else { 0.0 };
        locals.var_guard432 = assign13200_e18489;
        locals.var_guard432_rv = 0.0;

        let (assign13210_e18495, assign13210_e18495_d_n3, assign13210_e18495_d_n4, assign13210_e18495_d_n5, assign13210_e18495_d_n6, assign13210_e18495_d_n7, assign13210_e18495_d_n8, assign13210_e18495_d_n9, assign13210_e18495_d_n10, assign13210_e18495_d_n11,) = {
    if (locals.var_guard432 != 0.0) {
        let assign13210_e18493: f64 = (300.15 - 273.15);
        (assign13210_e18493, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign13210_e18495;
        locals.var_t0_dn3 = assign13210_e18495_d_n3;
        locals.var_t0_dn4 = assign13210_e18495_d_n4;
        locals.var_t0_dn5 = assign13210_e18495_d_n5;
        locals.var_t0_dn6 = assign13210_e18495_d_n6;
        locals.var_t0_dn7 = assign13210_e18495_d_n7;
        locals.var_t0_dn8 = assign13210_e18495_d_n8;
        locals.var_t0_dn9 = assign13210_e18495_d_n9;
        locals.var_t0_dn10 = assign13210_e18495_d_n10;
        locals.var_t0_dn11 = assign13210_e18495_d_n11;
        locals.var_t0_rv = 0.0;

        let (assign13220_e18499,) = {
    if (locals.var_guard432 != 0.0) {
        (300.15,)
    } else {
        (locals.var_tnom,)
    }
};
        locals.var_tnom = assign13220_e18499;
        locals.var_tnom_rv = 0.0;

        let (assign13230_e18506,) = {
    if (locals.var_guard432 == 0.0) {
        let assign13230_e18504: f64 = (p.p1028 + 273.15);
        (assign13230_e18504,)
    } else {
        (locals.var_tnom,)
    }
};
        locals.var_tnom = assign13230_e18506;
        locals.var_tnom_rv = 0.0;

        let assign13240_e18507: f64 = ctx_temp;
        let assign13240_e18509: f64 = (assign13240_e18507 + p.p23);
        locals.var_devtemp = assign13240_e18509;
        locals.var_devtemp_dn4 = 0.0;
        locals.var_devtemp_dn5 = 0.0;
        locals.var_devtemp_rv = 0.0;

        let assign13250_e18516: f64 = if ((p.p41 != 0.0) && (p.p1099 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard433 = assign13250_e18516;
        locals.var_guard433_rv = 0.0;

        let assign13260_e18523: f64 = if ((p.p40 != 0.0) && (!true)) { 1.0 } else { 0.0 };
        locals.var_guard434 = assign13260_e18523;
        locals.var_guard434_rv = 0.0;

        let assign13270_e18525: f64 = 1.0;
        locals.var_guard435 = assign13270_e18525;
        locals.var_guard435_rv = 0.0;

        let (assign13280_e18533, assign13280_e18533_d_n4, assign13280_e18533_d_n5,) = {
    if (((locals.var_guard433 != 0.0) && (locals.var_guard434 != 0.0)) && (locals.var_guard435 != 0.0)) {
        ((nv4 - 0.0), 1.0, 0.0,)
    } else {
        (locals.var_deltemp1, locals.var_deltemp1_dn4, locals.var_deltemp1_dn5,)
    }
};
        locals.var_deltemp1 = assign13280_e18533;
        locals.var_deltemp1_dn4 = assign13280_e18533_d_n4;
        locals.var_deltemp1_dn5 = assign13280_e18533_d_n5;
        locals.var_deltemp1_rv = 0.0;

        let (assign13290_e18542, assign13290_e18542_d_n4, assign13290_e18542_d_n5,) = {
    if (((locals.var_guard433 != 0.0) && (locals.var_guard434 != 0.0)) && (locals.var_guard435 == 0.0)) {
        ((nv5 - 0.0), 0.0, 1.0,)
    } else {
        (locals.var_deltemp1, locals.var_deltemp1_dn4, locals.var_deltemp1_dn5,)
    }
};
        locals.var_deltemp1 = assign13290_e18542;
        locals.var_deltemp1_dn4 = assign13290_e18542_d_n4;
        locals.var_deltemp1_dn5 = assign13290_e18542_d_n5;
        locals.var_deltemp1_rv = 0.0;

        let (assign13300_e18549, assign13300_e18549_d_n4, assign13300_e18549_d_n5,) = {
    if ((locals.var_guard433 != 0.0) && (locals.var_guard434 == 0.0)) {
        ((nv5 - 0.0), 0.0, 1.0,)
    } else {
        (locals.var_deltemp1, locals.var_deltemp1_dn4, locals.var_deltemp1_dn5,)
    }
};
        locals.var_deltemp1 = assign13300_e18549;
        locals.var_deltemp1_dn4 = assign13300_e18549_d_n4;
        locals.var_deltemp1_dn5 = assign13300_e18549_d_n5;
        locals.var_deltemp1_rv = 0.0;

        let (assign13310_e18554, assign13310_e18554_d_n4, assign13310_e18554_d_n5,) = {
    if (locals.var_guard433 == 0.0) {
        (0.0, 0.0, 0.0,)
    } else {
        (locals.var_deltemp1, locals.var_deltemp1_dn4, locals.var_deltemp1_dn5,)
    }
};
        locals.var_deltemp1 = assign13310_e18554;
        locals.var_deltemp1_dn4 = assign13310_e18554_d_n4;
        locals.var_deltemp1_dn5 = assign13310_e18554_d_n5;
        locals.var_deltemp1_rv = 0.0;

        let assign13320_e18557: f64 = (locals.var_deltemp1 + locals.var_devtemp);
        locals.var_devtemp = assign13320_e18557;
        locals.var_devtemp_dn4 = (locals.var_deltemp1_dn4 + locals.var_devtemp_dn4);
        locals.var_devtemp_dn5 = (locals.var_deltemp1_dn5 + locals.var_devtemp_dn5);
        locals.var_devtemp_rv = 0.0;

        let assign13360_e18565: f64 = (locals.var_kboq * locals.var_devtemp);
        locals.var_vt = assign13360_e18565;
        locals.var_vt_dn4 = (locals.var_kboq * locals.var_devtemp_dn4);
        locals.var_vt_dn5 = (locals.var_kboq * locals.var_devtemp_dn5);
        locals.var_vt_rv = 0.0;

        let assign13370_e18568: f64 = (1.0 / locals.var_vt);
        locals.var_inv_vt = assign13370_e18568;
        locals.var_inv_vt_dn4 = (-(locals.var_vt_dn4 / (locals.var_vt * locals.var_vt)));
        locals.var_inv_vt_dn5 = (-(locals.var_vt_dn5 / (locals.var_vt * locals.var_vt)));
        locals.var_inv_vt_rv = 0.0;

        let assign13380_e18571: f64 = (locals.var_devtemp / locals.var_tnom);
        locals.var_tratio = assign13380_e18571;
        locals.var_tratio_dn4 = (locals.var_devtemp_dn4 / locals.var_tnom);
        locals.var_tratio_dn5 = (locals.var_devtemp_dn5 / locals.var_tnom);
        locals.var_tratio_rv = 0.0;

        let assign13390_e18574: f64 = (locals.var_devtemp - locals.var_tnom);
        locals.var_deltemp = assign13390_e18574;
        locals.var_deltemp_dn4 = locals.var_devtemp_dn4;
        locals.var_deltemp_dn5 = locals.var_devtemp_dn5;
        locals.var_deltemp_rv = 0.0;

        let assign13400_e18577: f64 = (locals.var_kboq * locals.var_devtemp);
        locals.var_vtm = assign13400_e18577;
        locals.var_vtm_dn4 = (locals.var_kboq * locals.var_devtemp_dn4);
        locals.var_vtm_dn5 = (locals.var_kboq * locals.var_devtemp_dn5);
        locals.var_vtm_rv = 0.0;

        let assign13410_e18580: f64 = (locals.var_kboq * locals.var_tnom);
        locals.var_vtm0 = assign13410_e18580;
        locals.var_vtm0_rv = 0.0;

        let assign13420_e18584: f64 = (p.p1029 * locals.var_devtemp);
        let assign13420_e18586: f64 = (assign13420_e18584 * locals.var_devtemp);
        let assign13420_e18589: f64 = (locals.var_devtemp + p.p1030);
        let assign13420_e18590: f64 = (assign13420_e18586 / assign13420_e18589);
        let assign13420_e18591: f64 = (p.p108 - assign13420_e18590);
        locals.var_eg = assign13420_e18591;
        locals.var_eg_dn4 = (-((((((p.p1029 * locals.var_devtemp_dn4) * locals.var_devtemp) + (assign13420_e18584 * locals.var_devtemp_dn4)) * assign13420_e18589) - (assign13420_e18586 * locals.var_devtemp_dn4)) / (assign13420_e18589 * assign13420_e18589)));
        locals.var_eg_dn5 = (-((((((p.p1029 * locals.var_devtemp_dn5) * locals.var_devtemp) + (assign13420_e18584 * locals.var_devtemp_dn5)) * assign13420_e18589) - (assign13420_e18586 * locals.var_devtemp_dn5)) / (assign13420_e18589 * assign13420_e18589)));
        locals.var_eg_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_22(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let __rspice_inv_cse_0: f64 = 1.0 / locals.var_tnom;
        let assign13430_e18594: f64 = (locals.var_devtemp * __rspice_inv_cse_0);
        let assign13430_e18597: f64 = (locals.var_devtemp * __rspice_inv_cse_0);
        let assign13430_e18598: f64 = (assign13430_e18597).sqrt();
        let assign13430_e18599: f64 = (assign13430_e18594 * assign13430_e18598);
        locals.var_t1 = assign13430_e18599;
        locals.var_t1_dn3 = 0.0;
        locals.var_t1_dn4 = (((locals.var_devtemp_dn4 / locals.var_tnom) * assign13430_e18598) + (assign13430_e18594 * ((locals.var_devtemp_dn4 / locals.var_tnom) / (2.0 * assign13430_e18598))));
        locals.var_t1_dn5 = (((locals.var_devtemp_dn5 / locals.var_tnom) * assign13430_e18598) + (assign13430_e18594 * ((locals.var_devtemp_dn5 / locals.var_tnom) / (2.0 * assign13430_e18598))));
        locals.var_t1_dn6 = 0.0;
        locals.var_t1_dn7 = 0.0;
        locals.var_t1_dn8 = 0.0;
        locals.var_t1_dn9 = 0.0;
        locals.var_t1_dn10 = 0.0;
        locals.var_t1_dn11 = 0.0;
        locals.var_t1_rv = 0.0;

        let assign13440_e18602: f64 = (p.p107 * locals.var_t1);
        let assign13440_e18606: f64 = (2.0 * locals.var_vtm0);
        let assign13440_e18607: f64 = (locals.var_eg / assign13440_e18606);
        let assign13440_e18611: f64 = (2.0 * locals.var_vtm);
        let assign13440_e18612: f64 = (locals.var_eg / assign13440_e18611);
        let assign13440_e18613: f64 = (assign13440_e18607 - assign13440_e18612);
        let assign13440_e18614: f64 = { let limited_exp_arg = assign13440_e18613; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign13440_e18615: f64 = (assign13440_e18602 * assign13440_e18614);
        locals.var_ni = assign13440_e18615;
        locals.var_ni_dn3 = ((p.p107 * locals.var_t1_dn3) * assign13440_e18614);
        locals.var_ni_dn4 = (((p.p107 * locals.var_t1_dn4) * assign13440_e18614) + (assign13440_e18602 * ({ let limited_exp_arg = assign13440_e18613; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_eg_dn4 / assign13440_e18606) - (((locals.var_eg_dn4 * assign13440_e18611) - (locals.var_eg * (2.0 * locals.var_vtm_dn4))) / (assign13440_e18611 * assign13440_e18611))))));
        locals.var_ni_dn5 = (((p.p107 * locals.var_t1_dn5) * assign13440_e18614) + (assign13440_e18602 * ({ let limited_exp_arg = assign13440_e18613; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_eg_dn5 / assign13440_e18606) - (((locals.var_eg_dn5 * assign13440_e18611) - (locals.var_eg * (2.0 * locals.var_vtm_dn5))) / (assign13440_e18611 * assign13440_e18611))))));
        locals.var_ni_dn6 = ((p.p107 * locals.var_t1_dn6) * assign13440_e18614);
        locals.var_ni_dn7 = ((p.p107 * locals.var_t1_dn7) * assign13440_e18614);
        locals.var_ni_dn8 = ((p.p107 * locals.var_t1_dn8) * assign13440_e18614);
        locals.var_ni_dn9 = ((p.p107 * locals.var_t1_dn9) * assign13440_e18614);
        locals.var_ni_dn10 = ((p.p107 * locals.var_t1_dn10) * assign13440_e18614);
        locals.var_ni_dn11 = ((p.p107 * locals.var_t1_dn11) * assign13440_e18614);
        locals.var_ni_rv = 0.0;

        let assign13450_e18626: f64 = if (((p.p41 != 0.0) && (p.p1099 > 0.0)) && (locals.var_weff_sh > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard436 = assign13450_e18626;
        locals.var_guard436_rv = 0.0;

        let (assign13460_e18635, assign13460_e18635_d_n3, assign13460_e18635_d_n4, assign13460_e18635_d_n5, assign13460_e18635_d_n6, assign13460_e18635_d_n7, assign13460_e18635_d_n8, assign13460_e18635_d_n9, assign13460_e18635_d_n10, assign13460_e18635_d_n11,) = {
    if (locals.var_guard436 != 0.0) {
        let assign13460_e18630: f64 = (locals.var_ndep_i / locals.var_ni);
        let assign13460_e18632: f64 = (assign13460_e18630).max(1e-38);
        let assign13460_e18633: f64 = (assign13460_e18632).ln();
        (assign13460_e18633, (if assign13460_e18630 >= 1e-38 { (((locals.var_ndep_i_dn3 * locals.var_ni) - (locals.var_ndep_i * locals.var_ni_dn3)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign13460_e18632), (if assign13460_e18630 >= 1e-38 { (((locals.var_ndep_i_dn4 * locals.var_ni) - (locals.var_ndep_i * locals.var_ni_dn4)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign13460_e18632), (if assign13460_e18630 >= 1e-38 { (((locals.var_ndep_i_dn5 * locals.var_ni) - (locals.var_ndep_i * locals.var_ni_dn5)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign13460_e18632), (if assign13460_e18630 >= 1e-38 { (((locals.var_ndep_i_dn6 * locals.var_ni) - (locals.var_ndep_i * locals.var_ni_dn6)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign13460_e18632), (if assign13460_e18630 >= 1e-38 { (((locals.var_ndep_i_dn7 * locals.var_ni) - (locals.var_ndep_i * locals.var_ni_dn7)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign13460_e18632), (if assign13460_e18630 >= 1e-38 { (((locals.var_ndep_i_dn8 * locals.var_ni) - (locals.var_ndep_i * locals.var_ni_dn8)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign13460_e18632), (if assign13460_e18630 >= 1e-38 { (((locals.var_ndep_i_dn9 * locals.var_ni) - (locals.var_ndep_i * locals.var_ni_dn9)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign13460_e18632), (if assign13460_e18630 >= 1e-38 { (((locals.var_ndep_i_dn10 * locals.var_ni) - (locals.var_ndep_i * locals.var_ni_dn10)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign13460_e18632), (if assign13460_e18630 >= 1e-38 { (((locals.var_ndep_i_dn11 * locals.var_ni) - (locals.var_ndep_i * locals.var_ni_dn11)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign13460_e18632),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign13460_e18635;
        locals.var_t0_dn3 = assign13460_e18635_d_n3;
        locals.var_t0_dn4 = assign13460_e18635_d_n4;
        locals.var_t0_dn5 = assign13460_e18635_d_n5;
        locals.var_t0_dn6 = assign13460_e18635_d_n6;
        locals.var_t0_dn7 = assign13460_e18635_d_n7;
        locals.var_t0_dn8 = assign13460_e18635_d_n8;
        locals.var_t0_dn9 = assign13460_e18635_d_n9;
        locals.var_t0_dn10 = assign13460_e18635_d_n10;
        locals.var_t0_dn11 = assign13460_e18635_d_n11;
        locals.var_t0_rv = 0.0;

        let (assign13470_e18644, assign13470_e18644_d_n3, assign13470_e18644_d_n4, assign13470_e18644_d_n5, assign13470_e18644_d_n6, assign13470_e18644_d_n7, assign13470_e18644_d_n8, assign13470_e18644_d_n9, assign13470_e18644_d_n10, assign13470_e18644_d_n11,) = {
    if (locals.var_guard436 != 0.0) {
        let assign13470_e18639: f64 = (locals.var_t0 * locals.var_t0);
        let assign13470_e18641: f64 = (assign13470_e18639 + 1e-6);
        let assign13470_e18642: f64 = (assign13470_e18641).sqrt();
        (assign13470_e18642, (((locals.var_t0_dn3 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn3)) / (2.0 * assign13470_e18642)), (((locals.var_t0_dn4 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn4)) / (2.0 * assign13470_e18642)), (((locals.var_t0_dn5 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn5)) / (2.0 * assign13470_e18642)), (((locals.var_t0_dn6 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn6)) / (2.0 * assign13470_e18642)), (((locals.var_t0_dn7 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn7)) / (2.0 * assign13470_e18642)), (((locals.var_t0_dn8 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn8)) / (2.0 * assign13470_e18642)), (((locals.var_t0_dn9 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn9)) / (2.0 * assign13470_e18642)), (((locals.var_t0_dn10 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn10)) / (2.0 * assign13470_e18642)), (((locals.var_t0_dn11 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn11)) / (2.0 * assign13470_e18642)),)
    } else {
        (locals.var_phib, locals.var_phib_dn3, locals.var_phib_dn4, locals.var_phib_dn5, locals.var_phib_dn6, locals.var_phib_dn7, locals.var_phib_dn8, locals.var_phib_dn9, locals.var_phib_dn10, locals.var_phib_dn11,)
    }
};
        locals.var_phib = assign13470_e18644;
        locals.var_phib_dn3 = assign13470_e18644_d_n3;
        locals.var_phib_dn4 = assign13470_e18644_d_n4;
        locals.var_phib_dn5 = assign13470_e18644_d_n5;
        locals.var_phib_dn6 = assign13470_e18644_d_n6;
        locals.var_phib_dn7 = assign13470_e18644_d_n7;
        locals.var_phib_dn8 = assign13470_e18644_d_n8;
        locals.var_phib_dn9 = assign13470_e18644_d_n9;
        locals.var_phib_dn10 = assign13470_e18644_d_n10;
        locals.var_phib_dn11 = assign13470_e18644_d_n11;
        locals.var_phib_rv = 0.0;

        let (assign13480_e18654, assign13480_e18654_d_n3, assign13480_e18654_d_n4, assign13480_e18654_d_n5, assign13480_e18654_d_n6, assign13480_e18654_d_n7, assign13480_e18654_d_n8, assign13480_e18654_d_n9, assign13480_e18654_d_n10, assign13480_e18654_d_n11,) = {
    if (locals.var_guard436 == 0.0) {
        let assign13480_e18649: f64 = (locals.var_ndep_i / locals.var_ni);
        let assign13480_e18651: f64 = (assign13480_e18649).max(1e-38);
        let assign13480_e18652: f64 = (assign13480_e18651).ln();
        (assign13480_e18652, (if assign13480_e18649 >= 1e-38 { (((locals.var_ndep_i_dn3 * locals.var_ni) - (locals.var_ndep_i * locals.var_ni_dn3)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign13480_e18651), (if assign13480_e18649 >= 1e-38 { (((locals.var_ndep_i_dn4 * locals.var_ni) - (locals.var_ndep_i * locals.var_ni_dn4)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign13480_e18651), (if assign13480_e18649 >= 1e-38 { (((locals.var_ndep_i_dn5 * locals.var_ni) - (locals.var_ndep_i * locals.var_ni_dn5)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign13480_e18651), (if assign13480_e18649 >= 1e-38 { (((locals.var_ndep_i_dn6 * locals.var_ni) - (locals.var_ndep_i * locals.var_ni_dn6)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign13480_e18651), (if assign13480_e18649 >= 1e-38 { (((locals.var_ndep_i_dn7 * locals.var_ni) - (locals.var_ndep_i * locals.var_ni_dn7)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign13480_e18651), (if assign13480_e18649 >= 1e-38 { (((locals.var_ndep_i_dn8 * locals.var_ni) - (locals.var_ndep_i * locals.var_ni_dn8)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign13480_e18651), (if assign13480_e18649 >= 1e-38 { (((locals.var_ndep_i_dn9 * locals.var_ni) - (locals.var_ndep_i * locals.var_ni_dn9)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign13480_e18651), (if assign13480_e18649 >= 1e-38 { (((locals.var_ndep_i_dn10 * locals.var_ni) - (locals.var_ndep_i * locals.var_ni_dn10)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign13480_e18651), (if assign13480_e18649 >= 1e-38 { (((locals.var_ndep_i_dn11 * locals.var_ni) - (locals.var_ndep_i * locals.var_ni_dn11)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign13480_e18651),)
    } else {
        (locals.var_phib, locals.var_phib_dn3, locals.var_phib_dn4, locals.var_phib_dn5, locals.var_phib_dn6, locals.var_phib_dn7, locals.var_phib_dn8, locals.var_phib_dn9, locals.var_phib_dn10, locals.var_phib_dn11,)
    }
};
        locals.var_phib = assign13480_e18654;
        locals.var_phib_dn3 = assign13480_e18654_d_n3;
        locals.var_phib_dn4 = assign13480_e18654_d_n4;
        locals.var_phib_dn5 = assign13480_e18654_d_n5;
        locals.var_phib_dn6 = assign13480_e18654_d_n6;
        locals.var_phib_dn7 = assign13480_e18654_d_n7;
        locals.var_phib_dn8 = assign13480_e18654_d_n8;
        locals.var_phib_dn9 = assign13480_e18654_d_n9;
        locals.var_phib_dn10 = assign13480_e18654_d_n10;
        locals.var_phib_dn11 = assign13480_e18654_d_n11;
        locals.var_phib_rv = 0.0;

        let assign13490_e18665: f64 = if (((p.p41 != 0.0) && (p.p1099 > 0.0)) && (locals.var_weff_sh > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard437 = assign13490_e18665;
        locals.var_guard437_rv = 0.0;

        let (assign13500_e18678, assign13500_e18678_d_n3, assign13500_e18678_d_n4, assign13500_e18678_d_n5, assign13500_e18678_d_n6, assign13500_e18678_d_n7, assign13500_e18678_d_n8, assign13500_e18678_d_n9, assign13500_e18678_d_n10, assign13500_e18678_d_n11,) = {
    if (locals.var_guard437 != 0.0) {
        let assign13500_e18669: f64 = (locals.var_ndepedge_i * locals.var_nsd_i);
        let assign13500_e18672: f64 = (locals.var_ni * locals.var_ni);
        let assign13500_e18673: f64 = (assign13500_e18669 / assign13500_e18672);
        let assign13500_e18675: f64 = (assign13500_e18673).max(1e-38);
        let assign13500_e18676: f64 = (assign13500_e18675).ln();
        (assign13500_e18676, (if assign13500_e18673 >= 1e-38 { (-((assign13500_e18669 * ((locals.var_ni_dn3 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn3))) / (assign13500_e18672 * assign13500_e18672))) } else { 0.0 } / assign13500_e18675), (if assign13500_e18673 >= 1e-38 { (-((assign13500_e18669 * ((locals.var_ni_dn4 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn4))) / (assign13500_e18672 * assign13500_e18672))) } else { 0.0 } / assign13500_e18675), (if assign13500_e18673 >= 1e-38 { (-((assign13500_e18669 * ((locals.var_ni_dn5 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn5))) / (assign13500_e18672 * assign13500_e18672))) } else { 0.0 } / assign13500_e18675), (if assign13500_e18673 >= 1e-38 { (-((assign13500_e18669 * ((locals.var_ni_dn6 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn6))) / (assign13500_e18672 * assign13500_e18672))) } else { 0.0 } / assign13500_e18675), (if assign13500_e18673 >= 1e-38 { (-((assign13500_e18669 * ((locals.var_ni_dn7 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn7))) / (assign13500_e18672 * assign13500_e18672))) } else { 0.0 } / assign13500_e18675), (if assign13500_e18673 >= 1e-38 { (-((assign13500_e18669 * ((locals.var_ni_dn8 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn8))) / (assign13500_e18672 * assign13500_e18672))) } else { 0.0 } / assign13500_e18675), (if assign13500_e18673 >= 1e-38 { (-((assign13500_e18669 * ((locals.var_ni_dn9 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn9))) / (assign13500_e18672 * assign13500_e18672))) } else { 0.0 } / assign13500_e18675), (if assign13500_e18673 >= 1e-38 { (-((assign13500_e18669 * ((locals.var_ni_dn10 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn10))) / (assign13500_e18672 * assign13500_e18672))) } else { 0.0 } / assign13500_e18675), (if assign13500_e18673 >= 1e-38 { (-((assign13500_e18669 * ((locals.var_ni_dn11 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn11))) / (assign13500_e18672 * assign13500_e18672))) } else { 0.0 } / assign13500_e18675),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign13500_e18678;
        locals.var_t0_dn3 = assign13500_e18678_d_n3;
        locals.var_t0_dn4 = assign13500_e18678_d_n4;
        locals.var_t0_dn5 = assign13500_e18678_d_n5;
        locals.var_t0_dn6 = assign13500_e18678_d_n6;
        locals.var_t0_dn7 = assign13500_e18678_d_n7;
        locals.var_t0_dn8 = assign13500_e18678_d_n8;
        locals.var_t0_dn9 = assign13500_e18678_d_n9;
        locals.var_t0_dn10 = assign13500_e18678_d_n10;
        locals.var_t0_dn11 = assign13500_e18678_d_n11;
        locals.var_t0_rv = 0.0;

        let (assign13510_e18687, assign13510_e18687_d_n3, assign13510_e18687_d_n4, assign13510_e18687_d_n5, assign13510_e18687_d_n6, assign13510_e18687_d_n7, assign13510_e18687_d_n8, assign13510_e18687_d_n9, assign13510_e18687_d_n10, assign13510_e18687_d_n11,) = {
    if (locals.var_guard437 != 0.0) {
        let assign13510_e18682: f64 = (locals.var_t0 * locals.var_t0);
        let assign13510_e18684: f64 = (assign13510_e18682 + 1e-6);
        let assign13510_e18685: f64 = (assign13510_e18684).sqrt();
        (assign13510_e18685, (((locals.var_t0_dn3 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn3)) / (2.0 * assign13510_e18685)), (((locals.var_t0_dn4 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn4)) / (2.0 * assign13510_e18685)), (((locals.var_t0_dn5 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn5)) / (2.0 * assign13510_e18685)), (((locals.var_t0_dn6 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn6)) / (2.0 * assign13510_e18685)), (((locals.var_t0_dn7 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn7)) / (2.0 * assign13510_e18685)), (((locals.var_t0_dn8 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn8)) / (2.0 * assign13510_e18685)), (((locals.var_t0_dn9 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn9)) / (2.0 * assign13510_e18685)), (((locals.var_t0_dn10 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn10)) / (2.0 * assign13510_e18685)), (((locals.var_t0_dn11 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn11)) / (2.0 * assign13510_e18685)),)
    } else {
        (locals.var_vbi_edge, locals.var_vbi_edge_dn3, locals.var_vbi_edge_dn4, locals.var_vbi_edge_dn5, locals.var_vbi_edge_dn6, locals.var_vbi_edge_dn7, locals.var_vbi_edge_dn8, locals.var_vbi_edge_dn9, locals.var_vbi_edge_dn10, locals.var_vbi_edge_dn11,)
    }
};
        locals.var_vbi_edge = assign13510_e18687;
        locals.var_vbi_edge_dn3 = assign13510_e18687_d_n3;
        locals.var_vbi_edge_dn4 = assign13510_e18687_d_n4;
        locals.var_vbi_edge_dn5 = assign13510_e18687_d_n5;
        locals.var_vbi_edge_dn6 = assign13510_e18687_d_n6;
        locals.var_vbi_edge_dn7 = assign13510_e18687_d_n7;
        locals.var_vbi_edge_dn8 = assign13510_e18687_d_n8;
        locals.var_vbi_edge_dn9 = assign13510_e18687_d_n9;
        locals.var_vbi_edge_dn10 = assign13510_e18687_d_n10;
        locals.var_vbi_edge_dn11 = assign13510_e18687_d_n11;
        locals.var_vbi_edge_rv = 0.0;

        let (assign13520_e18701, assign13520_e18701_d_n3, assign13520_e18701_d_n4, assign13520_e18701_d_n5, assign13520_e18701_d_n6, assign13520_e18701_d_n7, assign13520_e18701_d_n8, assign13520_e18701_d_n9, assign13520_e18701_d_n10, assign13520_e18701_d_n11,) = {
    if (locals.var_guard437 == 0.0) {
        let assign13520_e18692: f64 = (locals.var_ndepedge_i * locals.var_nsd_i);
        let assign13520_e18695: f64 = (locals.var_ni * locals.var_ni);
        let assign13520_e18696: f64 = (assign13520_e18692 / assign13520_e18695);
        let assign13520_e18698: f64 = (assign13520_e18696).max(1e-38);
        let assign13520_e18699: f64 = (assign13520_e18698).ln();
        (assign13520_e18699, (if assign13520_e18696 >= 1e-38 { (-((assign13520_e18692 * ((locals.var_ni_dn3 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn3))) / (assign13520_e18695 * assign13520_e18695))) } else { 0.0 } / assign13520_e18698), (if assign13520_e18696 >= 1e-38 { (-((assign13520_e18692 * ((locals.var_ni_dn4 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn4))) / (assign13520_e18695 * assign13520_e18695))) } else { 0.0 } / assign13520_e18698), (if assign13520_e18696 >= 1e-38 { (-((assign13520_e18692 * ((locals.var_ni_dn5 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn5))) / (assign13520_e18695 * assign13520_e18695))) } else { 0.0 } / assign13520_e18698), (if assign13520_e18696 >= 1e-38 { (-((assign13520_e18692 * ((locals.var_ni_dn6 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn6))) / (assign13520_e18695 * assign13520_e18695))) } else { 0.0 } / assign13520_e18698), (if assign13520_e18696 >= 1e-38 { (-((assign13520_e18692 * ((locals.var_ni_dn7 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn7))) / (assign13520_e18695 * assign13520_e18695))) } else { 0.0 } / assign13520_e18698), (if assign13520_e18696 >= 1e-38 { (-((assign13520_e18692 * ((locals.var_ni_dn8 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn8))) / (assign13520_e18695 * assign13520_e18695))) } else { 0.0 } / assign13520_e18698), (if assign13520_e18696 >= 1e-38 { (-((assign13520_e18692 * ((locals.var_ni_dn9 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn9))) / (assign13520_e18695 * assign13520_e18695))) } else { 0.0 } / assign13520_e18698), (if assign13520_e18696 >= 1e-38 { (-((assign13520_e18692 * ((locals.var_ni_dn10 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn10))) / (assign13520_e18695 * assign13520_e18695))) } else { 0.0 } / assign13520_e18698), (if assign13520_e18696 >= 1e-38 { (-((assign13520_e18692 * ((locals.var_ni_dn11 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn11))) / (assign13520_e18695 * assign13520_e18695))) } else { 0.0 } / assign13520_e18698),)
    } else {
        (locals.var_vbi_edge, locals.var_vbi_edge_dn3, locals.var_vbi_edge_dn4, locals.var_vbi_edge_dn5, locals.var_vbi_edge_dn6, locals.var_vbi_edge_dn7, locals.var_vbi_edge_dn8, locals.var_vbi_edge_dn9, locals.var_vbi_edge_dn10, locals.var_vbi_edge_dn11,)
    }
};
        locals.var_vbi_edge = assign13520_e18701;
        locals.var_vbi_edge_dn3 = assign13520_e18701_d_n3;
        locals.var_vbi_edge_dn4 = assign13520_e18701_d_n4;
        locals.var_vbi_edge_dn5 = assign13520_e18701_d_n5;
        locals.var_vbi_edge_dn6 = assign13520_e18701_d_n6;
        locals.var_vbi_edge_dn7 = assign13520_e18701_d_n7;
        locals.var_vbi_edge_dn8 = assign13520_e18701_d_n8;
        locals.var_vbi_edge_dn9 = assign13520_e18701_d_n9;
        locals.var_vbi_edge_dn10 = assign13520_e18701_d_n10;
        locals.var_vbi_edge_dn11 = assign13520_e18701_d_n11;
        locals.var_vbi_edge_rv = 0.0;

        let assign13530_e18704: f64 = if locals.var_ngate_i > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard438 = assign13530_e18704;
        locals.var_guard438_rv = 0.0;

        let (assign13540_e18720, assign13540_e18720_d_n4, assign13540_e18720_d_n5,) = {
    if (locals.var_guard438 != 0.0) {
        let assign13540_e18707: f64 = (-locals.var_devsign);
        let assign13540_e18709: f64 = (assign13540_e18707 * locals.var_vt);
        let assign13540_e18712: f64 = (locals.var_ngate_i / locals.var_nsd_i);
        let assign13540_e18714: f64 = (assign13540_e18712).max(1e-38);
        let assign13540_e18715: f64 = (assign13540_e18714).ln();
        let assign13540_e18716: f64 = (assign13540_e18709 * assign13540_e18715);
        let assign13540_e18718: f64 = (assign13540_e18716 + p.p5);
        (assign13540_e18718, ((assign13540_e18707 * locals.var_vt_dn4) * assign13540_e18715), ((assign13540_e18707 * locals.var_vt_dn5) * assign13540_e18715),)
    } else {
        (locals.var_vfbsdr, locals.var_vfbsdr_dn4, locals.var_vfbsdr_dn5,)
    }
};
        locals.var_vfbsdr = assign13540_e18720;
        locals.var_vfbsdr_dn4 = assign13540_e18720_d_n4;
        locals.var_vfbsdr_dn5 = assign13540_e18720_d_n5;
        locals.var_vfbsdr_rv = 0.0;

        let (assign13550_e18725, assign13550_e18725_d_n4, assign13550_e18725_d_n5,) = {
    if (locals.var_guard438 == 0.0) {
        (0.0, 0.0, 0.0,)
    } else {
        (locals.var_vfbsdr, locals.var_vfbsdr_dn4, locals.var_vfbsdr_dn5,)
    }
};
        locals.var_vfbsdr = assign13550_e18725;
        locals.var_vfbsdr_dn4 = assign13550_e18725_d_n4;
        locals.var_vfbsdr_dn5 = assign13550_e18725_d_n5;
        locals.var_vfbsdr_rv = 0.0;

        let assign13660_e18777: f64 = (locals.var_vt * locals.var_phib);
        let assign13660_e18778: f64 = (0.4 + assign13660_e18777);
        let assign13660_e18780: f64 = (assign13660_e18778 + locals.var_phin_i);
        let assign13660_e18782: f64 = (assign13660_e18780).max(0.4);
        locals.var_phist = assign13660_e18782;
        locals.var_phist_dn3 = if assign13660_e18780 >= 0.4 { (locals.var_vt * locals.var_phib_dn3) } else { 0.0 };
        locals.var_phist_dn4 = if assign13660_e18780 >= 0.4 { ((locals.var_vt_dn4 * locals.var_phib) + (locals.var_vt * locals.var_phib_dn4)) } else { 0.0 };
        locals.var_phist_dn5 = if assign13660_e18780 >= 0.4 { ((locals.var_vt_dn5 * locals.var_phib) + (locals.var_vt * locals.var_phib_dn5)) } else { 0.0 };
        locals.var_phist_dn6 = if assign13660_e18780 >= 0.4 { (locals.var_vt * locals.var_phib_dn6) } else { 0.0 };
        locals.var_phist_dn7 = if assign13660_e18780 >= 0.4 { (locals.var_vt * locals.var_phib_dn7) } else { 0.0 };
        locals.var_phist_dn8 = if assign13660_e18780 >= 0.4 { (locals.var_vt * locals.var_phib_dn8) } else { 0.0 };
        locals.var_phist_dn9 = if assign13660_e18780 >= 0.4 { (locals.var_vt * locals.var_phib_dn9) } else { 0.0 };
        locals.var_phist_dn10 = if assign13660_e18780 >= 0.4 { (locals.var_vt * locals.var_phib_dn10) } else { 0.0 };
        locals.var_phist_dn11 = if assign13660_e18780 >= 0.4 { (locals.var_vt * locals.var_phib_dn11) } else { 0.0 };
        locals.var_phist_rv = 0.0;

        let assign13670_e18784: f64 = (locals.var_phist).sqrt();
        locals.var_sqrtphist = assign13670_e18784;
        locals.var_sqrtphist_dn3 = (locals.var_phist_dn3 / (2.0 * assign13670_e18784));
        locals.var_sqrtphist_dn4 = (locals.var_phist_dn4 / (2.0 * assign13670_e18784));
        locals.var_sqrtphist_dn5 = (locals.var_phist_dn5 / (2.0 * assign13670_e18784));
        locals.var_sqrtphist_dn6 = (locals.var_phist_dn6 / (2.0 * assign13670_e18784));
        locals.var_sqrtphist_dn7 = (locals.var_phist_dn7 / (2.0 * assign13670_e18784));
        locals.var_sqrtphist_dn8 = (locals.var_phist_dn8 / (2.0 * assign13670_e18784));
        locals.var_sqrtphist_dn9 = (locals.var_phist_dn9 / (2.0 * assign13670_e18784));
        locals.var_sqrtphist_dn10 = (locals.var_phist_dn10 / (2.0 * assign13670_e18784));
        locals.var_sqrtphist_dn11 = (locals.var_phist_dn11 / (2.0 * assign13670_e18784));
        locals.var_sqrtphist_rv = 0.0;

        let assign13680_e18787: f64 = (2.0 * locals.var_epssi);
        let assign13680_e18790: f64 = (1.602176462e-19 * locals.var_ndep_i);
        let assign13680_e18791: f64 = (assign13680_e18787 / assign13680_e18790);
        let assign13680_e18792: f64 = (assign13680_e18791).sqrt();
        locals.var_t1dep = assign13680_e18792;
        locals.var_t1dep_dn3 = ((-((assign13680_e18787 * (1.602176462e-19 * locals.var_ndep_i_dn3)) / (assign13680_e18790 * assign13680_e18790))) / (2.0 * assign13680_e18792));
        locals.var_t1dep_dn4 = ((-((assign13680_e18787 * (1.602176462e-19 * locals.var_ndep_i_dn4)) / (assign13680_e18790 * assign13680_e18790))) / (2.0 * assign13680_e18792));
        locals.var_t1dep_dn5 = ((-((assign13680_e18787 * (1.602176462e-19 * locals.var_ndep_i_dn5)) / (assign13680_e18790 * assign13680_e18790))) / (2.0 * assign13680_e18792));
        locals.var_t1dep_dn6 = ((-((assign13680_e18787 * (1.602176462e-19 * locals.var_ndep_i_dn6)) / (assign13680_e18790 * assign13680_e18790))) / (2.0 * assign13680_e18792));
        locals.var_t1dep_dn7 = ((-((assign13680_e18787 * (1.602176462e-19 * locals.var_ndep_i_dn7)) / (assign13680_e18790 * assign13680_e18790))) / (2.0 * assign13680_e18792));
        locals.var_t1dep_dn8 = ((-((assign13680_e18787 * (1.602176462e-19 * locals.var_ndep_i_dn8)) / (assign13680_e18790 * assign13680_e18790))) / (2.0 * assign13680_e18792));
        locals.var_t1dep_dn9 = ((-((assign13680_e18787 * (1.602176462e-19 * locals.var_ndep_i_dn9)) / (assign13680_e18790 * assign13680_e18790))) / (2.0 * assign13680_e18792));
        locals.var_t1dep_dn10 = ((-((assign13680_e18787 * (1.602176462e-19 * locals.var_ndep_i_dn10)) / (assign13680_e18790 * assign13680_e18790))) / (2.0 * assign13680_e18792));
        locals.var_t1dep_dn11 = ((-((assign13680_e18787 * (1.602176462e-19 * locals.var_ndep_i_dn11)) / (assign13680_e18790 * assign13680_e18790))) / (2.0 * assign13680_e18792));
        locals.var_t1dep_rv = 0.0;

        let assign13690_e18795: f64 = (locals.var_epssi / locals.var_epsox);
        let assign13690_e18797: f64 = (assign13690_e18795 * p.p76);
        let assign13690_e18799: f64 = (assign13690_e18797 * locals.var_xj_i);
        let assign13690_e18800: f64 = (assign13690_e18799).sqrt();
        locals.var_litl = assign13690_e18800;
        locals.var_litl_rv = 0.0;

        let assign13700_e18807: f64 = (locals.var_tratio - 1.0);
        let assign13700_e18808: f64 = (p.p1031 * assign13700_e18807);
        let assign13700_e18809: f64 = (1.0 + assign13700_e18808);
        let assign13700_e18814: f64 = (locals.var_tratio - 1.0);
        let assign13700_e18815: f64 = (p.p1031 * assign13700_e18814);
        let assign13700_e18816: f64 = (1.0 + assign13700_e18815);
        let assign13700_e18821: f64 = (locals.var_tratio - 1.0);
        let assign13700_e18822: f64 = (p.p1031 * assign13700_e18821);
        let assign13700_e18823: f64 = (1.0 + assign13700_e18822);
        let assign13700_e18824: f64 = (assign13700_e18816 * assign13700_e18823);
        let assign13700_e18827: f64 = (4.0 * 0.001);
        let assign13700_e18829: f64 = (assign13700_e18827 * 0.001);
        let assign13700_e18830: f64 = (assign13700_e18824 + assign13700_e18829);
        let assign13700_e18831: f64 = (assign13700_e18830).sqrt();
        let assign13700_e18832: f64 = (assign13700_e18809 + assign13700_e18831);
        let assign13700_e18833: f64 = (0.5 * assign13700_e18832);
        let assign13700_e18834: f64 = (locals.var_nfactor_i * assign13700_e18833);
        locals.var_nfactor_t = assign13700_e18834;
        locals.var_nfactor_t_dn3 = (locals.var_nfactor_i_dn3 * assign13700_e18833);
        locals.var_nfactor_t_dn4 = ((locals.var_nfactor_i_dn4 * assign13700_e18833) + (locals.var_nfactor_i * (0.5 * ((p.p1031 * locals.var_tratio_dn4) + ((((p.p1031 * locals.var_tratio_dn4) * assign13700_e18823) + (assign13700_e18816 * (p.p1031 * locals.var_tratio_dn4))) / (2.0 * assign13700_e18831))))));
        locals.var_nfactor_t_dn5 = ((locals.var_nfactor_i_dn5 * assign13700_e18833) + (locals.var_nfactor_i * (0.5 * ((p.p1031 * locals.var_tratio_dn5) + ((((p.p1031 * locals.var_tratio_dn5) * assign13700_e18823) + (assign13700_e18816 * (p.p1031 * locals.var_tratio_dn5))) / (2.0 * assign13700_e18831))))));
        locals.var_nfactor_t_dn6 = (locals.var_nfactor_i_dn6 * assign13700_e18833);
        locals.var_nfactor_t_dn7 = (locals.var_nfactor_i_dn7 * assign13700_e18833);
        locals.var_nfactor_t_dn8 = (locals.var_nfactor_i_dn8 * assign13700_e18833);
        locals.var_nfactor_t_dn9 = (locals.var_nfactor_i_dn9 * assign13700_e18833);
        locals.var_nfactor_t_dn10 = (locals.var_nfactor_i_dn10 * assign13700_e18833);
        locals.var_nfactor_t_dn11 = (locals.var_nfactor_i_dn11 * assign13700_e18833);
        locals.var_nfactor_t_rv = 0.0;

        let assign13710_e18840: f64 = (locals.var_tratio - 1.0);
        let assign13710_e18841: f64 = (p.p1059 * assign13710_e18840);
        let assign13710_e18842: f64 = (1.0 + assign13710_e18841);
        let assign13710_e18843: f64 = (locals.var_eta0_i * assign13710_e18842);
        locals.var_eta0_t = assign13710_e18843;
        locals.var_eta0_t_dn3 = (locals.var_eta0_i_dn3 * assign13710_e18842);
        locals.var_eta0_t_dn4 = ((locals.var_eta0_i_dn4 * assign13710_e18842) + (locals.var_eta0_i * (p.p1059 * locals.var_tratio_dn4)));
        locals.var_eta0_t_dn5 = ((locals.var_eta0_i_dn5 * assign13710_e18842) + (locals.var_eta0_i * (p.p1059 * locals.var_tratio_dn5)));
        locals.var_eta0_t_dn6 = (locals.var_eta0_i_dn6 * assign13710_e18842);
        locals.var_eta0_t_dn7 = (locals.var_eta0_i_dn7 * assign13710_e18842);
        locals.var_eta0_t_dn8 = (locals.var_eta0_i_dn8 * assign13710_e18842);
        locals.var_eta0_t_dn9 = (locals.var_eta0_i_dn9 * assign13710_e18842);
        locals.var_eta0_t_dn10 = (locals.var_eta0_i_dn10 * assign13710_e18842);
        locals.var_eta0_t_dn11 = (locals.var_eta0_i_dn11 * assign13710_e18842);
        locals.var_eta0_t_rv = 0.0;

        let assign13720_e18846: f64 = if p.p35 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard449 = assign13720_e18846;
        locals.var_guard449_rv = 0.0;

        let (assign13730_e18858, assign13730_e18858_d_n3, assign13730_e18858_d_n4, assign13730_e18858_d_n5, assign13730_e18858_d_n6, assign13730_e18858_d_n7, assign13730_e18858_d_n8, assign13730_e18858_d_n9, assign13730_e18858_d_n10, assign13730_e18858_d_n11,) = {
    if (locals.var_guard449 != 0.0) {
        let assign13730_e18853: f64 = (locals.var_tratio - 1.0);
        let assign13730_e18854: f64 = (p.p1059 * assign13730_e18853);
        let assign13730_e18855: f64 = (1.0 + assign13730_e18854);
        let assign13730_e18856: f64 = (locals.var_eta0r_i * assign13730_e18855);
        (assign13730_e18856, (locals.var_eta0r_i_dn3 * assign13730_e18855), ((locals.var_eta0r_i_dn4 * assign13730_e18855) + (locals.var_eta0r_i * (p.p1059 * locals.var_tratio_dn4))), ((locals.var_eta0r_i_dn5 * assign13730_e18855) + (locals.var_eta0r_i * (p.p1059 * locals.var_tratio_dn5))), (locals.var_eta0r_i_dn6 * assign13730_e18855), (locals.var_eta0r_i_dn7 * assign13730_e18855), (locals.var_eta0r_i_dn8 * assign13730_e18855), (locals.var_eta0r_i_dn9 * assign13730_e18855), (locals.var_eta0r_i_dn10 * assign13730_e18855), (locals.var_eta0r_i_dn11 * assign13730_e18855),)
    } else {
        (locals.var_eta0r_t, locals.var_eta0r_t_dn3, locals.var_eta0r_t_dn4, locals.var_eta0r_t_dn5, locals.var_eta0r_t_dn6, locals.var_eta0r_t_dn7, locals.var_eta0r_t_dn8, locals.var_eta0r_t_dn9, locals.var_eta0r_t_dn10, locals.var_eta0r_t_dn11,)
    }
};
        locals.var_eta0r_t = assign13730_e18858;
        locals.var_eta0r_t_dn3 = assign13730_e18858_d_n3;
        locals.var_eta0r_t_dn4 = assign13730_e18858_d_n4;
        locals.var_eta0r_t_dn5 = assign13730_e18858_d_n5;
        locals.var_eta0r_t_dn6 = assign13730_e18858_d_n6;
        locals.var_eta0r_t_dn7 = assign13730_e18858_d_n7;
        locals.var_eta0r_t_dn8 = assign13730_e18858_d_n8;
        locals.var_eta0r_t_dn9 = assign13730_e18858_d_n9;
        locals.var_eta0r_t_dn10 = assign13730_e18858_d_n10;
        locals.var_eta0r_t_dn11 = assign13730_e18858_d_n11;
        locals.var_eta0r_t_rv = 0.0;

        let (assign13740_e18868,) = {
    if (p.p30 != 1.0) {
        let assign13740_e18864: f64 = (0.3333333333333333 * p.p347);
        (assign13740_e18864,)
    } else {
        let assign13740_e18867: f64 = (0.5 * p.p347);
        (assign13740_e18867,)
    }
};
        locals.var_eta_mu = assign13740_e18868;
        locals.var_eta_mu_rv = 0.0;

        let assign13750_e18872: f64 = (locals.var_tratio).powf(locals.var_ute_i);
        let assign13750_e18873: f64 = (locals.var_u0_i * assign13750_e18872);
        locals.var_u0_t = assign13750_e18873;
        locals.var_u0_t_dn3 = 0.0;
        locals.var_u0_t_dn4 = (locals.var_u0_i * if 0.0 == 0.0 && ((locals.var_ute_i) as f64).is_finite() && ((locals.var_ute_i) as f64).fract() == 0.0 { if locals.var_ute_i == 0.0 { 0.0 } else { (locals.var_ute_i * ((locals.var_tratio).powf(locals.var_ute_i - 1.0) * locals.var_tratio_dn4)) } } else { (assign13750_e18872 * (locals.var_ute_i * (locals.var_tratio_dn4 / locals.var_tratio))) });
        locals.var_u0_t_dn5 = (locals.var_u0_i * if 0.0 == 0.0 && ((locals.var_ute_i) as f64).is_finite() && ((locals.var_ute_i) as f64).fract() == 0.0 { if locals.var_ute_i == 0.0 { 0.0 } else { (locals.var_ute_i * ((locals.var_tratio).powf(locals.var_ute_i - 1.0) * locals.var_tratio_dn5)) } } else { (assign13750_e18872 * (locals.var_ute_i * (locals.var_tratio_dn5 / locals.var_tratio))) });
        locals.var_u0_t_dn6 = 0.0;
        locals.var_u0_t_dn7 = 0.0;
        locals.var_u0_t_dn8 = 0.0;
        locals.var_u0_t_dn9 = 0.0;
        locals.var_u0_t_dn10 = 0.0;
        locals.var_u0_t_dn11 = 0.0;
        locals.var_u0_t_rv = 0.0;

        let assign13760_e18879: f64 = (locals.var_ua1_i * locals.var_deltemp);
        let assign13760_e18880: f64 = (1.0 + assign13760_e18879);
        let assign13760_e18882: f64 = (assign13760_e18880 - 1e-6);
        let assign13760_e18886: f64 = (locals.var_ua1_i * locals.var_deltemp);
        let assign13760_e18887: f64 = (1.0 + assign13760_e18886);
        let assign13760_e18889: f64 = (assign13760_e18887 - 1e-6);
        let assign13760_e18893: f64 = (locals.var_ua1_i * locals.var_deltemp);
        let assign13760_e18894: f64 = (1.0 + assign13760_e18893);
        let assign13760_e18896: f64 = (assign13760_e18894 - 1e-6);
        let assign13760_e18897: f64 = (assign13760_e18889 * assign13760_e18896);
        let assign13760_e18900: f64 = (4.0 * 0.001);
        let assign13760_e18902: f64 = (assign13760_e18900 * 0.001);
        let assign13760_e18903: f64 = (assign13760_e18897 + assign13760_e18902);
        let assign13760_e18904: f64 = (assign13760_e18903).sqrt();
        let assign13760_e18905: f64 = (assign13760_e18882 + assign13760_e18904);
        let assign13760_e18906: f64 = (0.5 * assign13760_e18905);
        let assign13760_e18907: f64 = (locals.var_ua_i * assign13760_e18906);
        locals.var_ua_t = assign13760_e18907;
        locals.var_ua_t_dn3 = (locals.var_ua_i_dn3 * assign13760_e18906);
        locals.var_ua_t_dn4 = ((locals.var_ua_i_dn4 * assign13760_e18906) + (locals.var_ua_i * (0.5 * ((locals.var_ua1_i * locals.var_deltemp_dn4) + ((((locals.var_ua1_i * locals.var_deltemp_dn4) * assign13760_e18896) + (assign13760_e18889 * (locals.var_ua1_i * locals.var_deltemp_dn4))) / (2.0 * assign13760_e18904))))));
        locals.var_ua_t_dn5 = ((locals.var_ua_i_dn5 * assign13760_e18906) + (locals.var_ua_i * (0.5 * ((locals.var_ua1_i * locals.var_deltemp_dn5) + ((((locals.var_ua1_i * locals.var_deltemp_dn5) * assign13760_e18896) + (assign13760_e18889 * (locals.var_ua1_i * locals.var_deltemp_dn5))) / (2.0 * assign13760_e18904))))));
        locals.var_ua_t_dn6 = (locals.var_ua_i_dn6 * assign13760_e18906);
        locals.var_ua_t_dn7 = (locals.var_ua_i_dn7 * assign13760_e18906);
        locals.var_ua_t_dn8 = (locals.var_ua_i_dn8 * assign13760_e18906);
        locals.var_ua_t_dn9 = (locals.var_ua_i_dn9 * assign13760_e18906);
        locals.var_ua_t_dn10 = (locals.var_ua_i_dn10 * assign13760_e18906);
        locals.var_ua_t_dn11 = (locals.var_ua_i_dn11 * assign13760_e18906);
        locals.var_ua_t_rv = 0.0;

        let assign13770_e18913: f64 = (locals.var_uc1_i * locals.var_deltemp);
        let assign13770_e18914: f64 = (1.0 + assign13770_e18913);
        let assign13770_e18916: f64 = (assign13770_e18914 - 1e-6);
        let assign13770_e18920: f64 = (locals.var_uc1_i * locals.var_deltemp);
        let assign13770_e18921: f64 = (1.0 + assign13770_e18920);
        let assign13770_e18923: f64 = (assign13770_e18921 - 1e-6);
        let assign13770_e18927: f64 = (locals.var_uc1_i * locals.var_deltemp);
        let assign13770_e18928: f64 = (1.0 + assign13770_e18927);
        let assign13770_e18930: f64 = (assign13770_e18928 - 1e-6);
        let assign13770_e18931: f64 = (assign13770_e18923 * assign13770_e18930);
        let assign13770_e18934: f64 = (4.0 * 0.001);
        let assign13770_e18936: f64 = (assign13770_e18934 * 0.001);
        let assign13770_e18937: f64 = (assign13770_e18931 + assign13770_e18936);
        let assign13770_e18938: f64 = (assign13770_e18937).sqrt();
        let assign13770_e18939: f64 = (assign13770_e18916 + assign13770_e18938);
        let assign13770_e18940: f64 = (0.5 * assign13770_e18939);
        let assign13770_e18941: f64 = (locals.var_uc_i * assign13770_e18940);
        locals.var_uc_t = assign13770_e18941;
        locals.var_uc_t_dn3 = (locals.var_uc_i_dn3 * assign13770_e18940);
        locals.var_uc_t_dn4 = ((locals.var_uc_i_dn4 * assign13770_e18940) + (locals.var_uc_i * (0.5 * ((locals.var_uc1_i * locals.var_deltemp_dn4) + ((((locals.var_uc1_i * locals.var_deltemp_dn4) * assign13770_e18930) + (assign13770_e18923 * (locals.var_uc1_i * locals.var_deltemp_dn4))) / (2.0 * assign13770_e18938))))));
        locals.var_uc_t_dn5 = ((locals.var_uc_i_dn5 * assign13770_e18940) + (locals.var_uc_i * (0.5 * ((locals.var_uc1_i * locals.var_deltemp_dn5) + ((((locals.var_uc1_i * locals.var_deltemp_dn5) * assign13770_e18930) + (assign13770_e18923 * (locals.var_uc1_i * locals.var_deltemp_dn5))) / (2.0 * assign13770_e18938))))));
        locals.var_uc_t_dn6 = (locals.var_uc_i_dn6 * assign13770_e18940);
        locals.var_uc_t_dn7 = (locals.var_uc_i_dn7 * assign13770_e18940);
        locals.var_uc_t_dn8 = (locals.var_uc_i_dn8 * assign13770_e18940);
        locals.var_uc_t_dn9 = (locals.var_uc_i_dn9 * assign13770_e18940);
        locals.var_uc_t_dn10 = (locals.var_uc_i_dn10 * assign13770_e18940);
        locals.var_uc_t_dn11 = (locals.var_uc_i_dn11 * assign13770_e18940);
        locals.var_uc_t_rv = 0.0;

        let assign13780_e18945: f64 = (locals.var_tratio).powf(locals.var_ud1_i);
        let assign13780_e18946: f64 = (locals.var_ud_i * assign13780_e18945);
        locals.var_ud_t = assign13780_e18946;
        locals.var_ud_t_dn3 = (locals.var_ud_i_dn3 * assign13780_e18945);
        locals.var_ud_t_dn4 = ((locals.var_ud_i_dn4 * assign13780_e18945) + (locals.var_ud_i * if 0.0 == 0.0 && ((locals.var_ud1_i) as f64).is_finite() && ((locals.var_ud1_i) as f64).fract() == 0.0 { if locals.var_ud1_i == 0.0 { 0.0 } else { (locals.var_ud1_i * ((locals.var_tratio).powf(locals.var_ud1_i - 1.0) * locals.var_tratio_dn4)) } } else { (assign13780_e18945 * (locals.var_ud1_i * (locals.var_tratio_dn4 / locals.var_tratio))) }));
        locals.var_ud_t_dn5 = ((locals.var_ud_i_dn5 * assign13780_e18945) + (locals.var_ud_i * if 0.0 == 0.0 && ((locals.var_ud1_i) as f64).is_finite() && ((locals.var_ud1_i) as f64).fract() == 0.0 { if locals.var_ud1_i == 0.0 { 0.0 } else { (locals.var_ud1_i * ((locals.var_tratio).powf(locals.var_ud1_i - 1.0) * locals.var_tratio_dn5)) } } else { (assign13780_e18945 * (locals.var_ud1_i * (locals.var_tratio_dn5 / locals.var_tratio))) }));
        locals.var_ud_t_dn6 = (locals.var_ud_i_dn6 * assign13780_e18945);
        locals.var_ud_t_dn7 = (locals.var_ud_i_dn7 * assign13780_e18945);
        locals.var_ud_t_dn8 = (locals.var_ud_i_dn8 * assign13780_e18945);
        locals.var_ud_t_dn9 = (locals.var_ud_i_dn9 * assign13780_e18945);
        locals.var_ud_t_dn10 = (locals.var_ud_i_dn10 * assign13780_e18945);
        locals.var_ud_t_dn11 = (locals.var_ud_i_dn11 * assign13780_e18945);
        locals.var_ud_t_rv = 0.0;

        let assign13790_e18950: f64 = (locals.var_tratio).powf(locals.var_ucste_i);
        let assign13790_e18951: f64 = (locals.var_ucs_i * assign13790_e18950);
        locals.var_ucs_t = assign13790_e18951;
        locals.var_ucs_t_dn4 = (locals.var_ucs_i * if 0.0 == 0.0 && ((locals.var_ucste_i) as f64).is_finite() && ((locals.var_ucste_i) as f64).fract() == 0.0 { if locals.var_ucste_i == 0.0 { 0.0 } else { (locals.var_ucste_i * ((locals.var_tratio).powf(locals.var_ucste_i - 1.0) * locals.var_tratio_dn4)) } } else { (assign13790_e18950 * (locals.var_ucste_i * (locals.var_tratio_dn4 / locals.var_tratio))) });
        locals.var_ucs_t_dn5 = (locals.var_ucs_i * if 0.0 == 0.0 && ((locals.var_ucste_i) as f64).is_finite() && ((locals.var_ucste_i) as f64).fract() == 0.0 { if locals.var_ucste_i == 0.0 { 0.0 } else { (locals.var_ucste_i * ((locals.var_tratio).powf(locals.var_ucste_i - 1.0) * locals.var_tratio_dn5)) } } else { (assign13790_e18950 * (locals.var_ucste_i * (locals.var_tratio_dn5 / locals.var_tratio))) });
        locals.var_ucs_t_rv = 0.0;

        let assign13800_e18958: f64 = (locals.var_tratio - 1.0);
        let assign13800_e18959: f64 = (locals.var_eu1_i * assign13800_e18958);
        let assign13800_e18960: f64 = (1.0 + assign13800_e18959);
        let assign13800_e18965: f64 = (locals.var_tratio - 1.0);
        let assign13800_e18966: f64 = (locals.var_eu1_i * assign13800_e18965);
        let assign13800_e18967: f64 = (1.0 + assign13800_e18966);
        let assign13800_e18972: f64 = (locals.var_tratio - 1.0);
        let assign13800_e18973: f64 = (locals.var_eu1_i * assign13800_e18972);
        let assign13800_e18974: f64 = (1.0 + assign13800_e18973);
        let assign13800_e18975: f64 = (assign13800_e18967 * assign13800_e18974);
        let assign13800_e18978: f64 = (4.0 * 0.001);
        let assign13800_e18980: f64 = (assign13800_e18978 * 0.001);
        let assign13800_e18981: f64 = (assign13800_e18975 + assign13800_e18980);
        let assign13800_e18982: f64 = (assign13800_e18981).sqrt();
        let assign13800_e18983: f64 = (assign13800_e18960 + assign13800_e18982);
        let assign13800_e18984: f64 = (0.5 * assign13800_e18983);
        let assign13800_e18985: f64 = (locals.var_eu_i * assign13800_e18984);
        locals.var_eu_t = assign13800_e18985;
        locals.var_eu_t_dn3 = (locals.var_eu_i_dn3 * assign13800_e18984);
        locals.var_eu_t_dn4 = ((locals.var_eu_i_dn4 * assign13800_e18984) + (locals.var_eu_i * (0.5 * ((locals.var_eu1_i * locals.var_tratio_dn4) + ((((locals.var_eu1_i * locals.var_tratio_dn4) * assign13800_e18974) + (assign13800_e18967 * (locals.var_eu1_i * locals.var_tratio_dn4))) / (2.0 * assign13800_e18982))))));
        locals.var_eu_t_dn5 = ((locals.var_eu_i_dn5 * assign13800_e18984) + (locals.var_eu_i * (0.5 * ((locals.var_eu1_i * locals.var_tratio_dn5) + ((((locals.var_eu1_i * locals.var_tratio_dn5) * assign13800_e18974) + (assign13800_e18967 * (locals.var_eu1_i * locals.var_tratio_dn5))) / (2.0 * assign13800_e18982))))));
        locals.var_eu_t_dn6 = (locals.var_eu_i_dn6 * assign13800_e18984);
        locals.var_eu_t_dn7 = (locals.var_eu_i_dn7 * assign13800_e18984);
        locals.var_eu_t_dn8 = (locals.var_eu_i_dn8 * assign13800_e18984);
        locals.var_eu_t_dn9 = (locals.var_eu_i_dn9 * assign13800_e18984);
        locals.var_eu_t_dn10 = (locals.var_eu_i_dn10 * assign13800_e18984);
        locals.var_eu_t_dn11 = (locals.var_eu_i_dn11 * assign13800_e18984);
        locals.var_eu_t_rv = 0.0;

        let assign13810_e18988: f64 = if p.p35 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard450 = assign13810_e18988;
        locals.var_guard450_rv = 0.0;

        let (assign13820_e18996, assign13820_e18996_d_n4, assign13820_e18996_d_n5,) = {
    if (locals.var_guard450 != 0.0) {
        let assign13820_e18993: f64 = (locals.var_tratio).powf(locals.var_ute_i);
        let assign13820_e18994: f64 = (locals.var_u0r_i * assign13820_e18993);
        (assign13820_e18994, (locals.var_u0r_i * if 0.0 == 0.0 && ((locals.var_ute_i) as f64).is_finite() && ((locals.var_ute_i) as f64).fract() == 0.0 { if locals.var_ute_i == 0.0 { 0.0 } else { (locals.var_ute_i * ((locals.var_tratio).powf(locals.var_ute_i - 1.0) * locals.var_tratio_dn4)) } } else { (assign13820_e18993 * (locals.var_ute_i * (locals.var_tratio_dn4 / locals.var_tratio))) }), (locals.var_u0r_i * if 0.0 == 0.0 && ((locals.var_ute_i) as f64).is_finite() && ((locals.var_ute_i) as f64).fract() == 0.0 { if locals.var_ute_i == 0.0 { 0.0 } else { (locals.var_ute_i * ((locals.var_tratio).powf(locals.var_ute_i - 1.0) * locals.var_tratio_dn5)) } } else { (assign13820_e18993 * (locals.var_ute_i * (locals.var_tratio_dn5 / locals.var_tratio))) }),)
    } else {
        (locals.var_u0r_t, locals.var_u0r_t_dn4, locals.var_u0r_t_dn5,)
    }
};
        locals.var_u0r_t = assign13820_e18996;
        locals.var_u0r_t_dn4 = assign13820_e18996_d_n4;
        locals.var_u0r_t_dn5 = assign13820_e18996_d_n5;
        locals.var_u0r_t_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_23(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign13830_e19033, assign13830_e19033_d_n3, assign13830_e19033_d_n4, assign13830_e19033_d_n5, assign13830_e19033_d_n6, assign13830_e19033_d_n7, assign13830_e19033_d_n8, assign13830_e19033_d_n9, assign13830_e19033_d_n10, assign13830_e19033_d_n11,) = {
    if (locals.var_guard450 != 0.0) {
        let assign13830_e19003: f64 = (locals.var_ua1_i * locals.var_deltemp);
        let assign13830_e19004: f64 = (1.0 + assign13830_e19003);
        let assign13830_e19006: f64 = (assign13830_e19004 - 1e-6);
        let assign13830_e19010: f64 = (locals.var_ua1_i * locals.var_deltemp);
        let assign13830_e19011: f64 = (1.0 + assign13830_e19010);
        let assign13830_e19013: f64 = (assign13830_e19011 - 1e-6);
        let assign13830_e19017: f64 = (locals.var_ua1_i * locals.var_deltemp);
        let assign13830_e19018: f64 = (1.0 + assign13830_e19017);
        let assign13830_e19020: f64 = (assign13830_e19018 - 1e-6);
        let assign13830_e19021: f64 = (assign13830_e19013 * assign13830_e19020);
        let assign13830_e19024: f64 = (4.0 * 0.001);
        let assign13830_e19026: f64 = (assign13830_e19024 * 0.001);
        let assign13830_e19027: f64 = (assign13830_e19021 + assign13830_e19026);
        let assign13830_e19028: f64 = (assign13830_e19027).sqrt();
        let assign13830_e19029: f64 = (assign13830_e19006 + assign13830_e19028);
        let assign13830_e19030: f64 = (0.5 * assign13830_e19029);
        let assign13830_e19031: f64 = (locals.var_uar_i * assign13830_e19030);
        (assign13830_e19031, (locals.var_uar_i_dn3 * assign13830_e19030), ((locals.var_uar_i_dn4 * assign13830_e19030) + (locals.var_uar_i * (0.5 * ((locals.var_ua1_i * locals.var_deltemp_dn4) + ((((locals.var_ua1_i * locals.var_deltemp_dn4) * assign13830_e19020) + (assign13830_e19013 * (locals.var_ua1_i * locals.var_deltemp_dn4))) / (2.0 * assign13830_e19028)))))), ((locals.var_uar_i_dn5 * assign13830_e19030) + (locals.var_uar_i * (0.5 * ((locals.var_ua1_i * locals.var_deltemp_dn5) + ((((locals.var_ua1_i * locals.var_deltemp_dn5) * assign13830_e19020) + (assign13830_e19013 * (locals.var_ua1_i * locals.var_deltemp_dn5))) / (2.0 * assign13830_e19028)))))), (locals.var_uar_i_dn6 * assign13830_e19030), (locals.var_uar_i_dn7 * assign13830_e19030), (locals.var_uar_i_dn8 * assign13830_e19030), (locals.var_uar_i_dn9 * assign13830_e19030), (locals.var_uar_i_dn10 * assign13830_e19030), (locals.var_uar_i_dn11 * assign13830_e19030),)
    } else {
        (locals.var_uar_t, locals.var_uar_t_dn3, locals.var_uar_t_dn4, locals.var_uar_t_dn5, locals.var_uar_t_dn6, locals.var_uar_t_dn7, locals.var_uar_t_dn8, locals.var_uar_t_dn9, locals.var_uar_t_dn10, locals.var_uar_t_dn11,)
    }
};
        locals.var_uar_t = assign13830_e19033;
        locals.var_uar_t_dn3 = assign13830_e19033_d_n3;
        locals.var_uar_t_dn4 = assign13830_e19033_d_n4;
        locals.var_uar_t_dn5 = assign13830_e19033_d_n5;
        locals.var_uar_t_dn6 = assign13830_e19033_d_n6;
        locals.var_uar_t_dn7 = assign13830_e19033_d_n7;
        locals.var_uar_t_dn8 = assign13830_e19033_d_n8;
        locals.var_uar_t_dn9 = assign13830_e19033_d_n9;
        locals.var_uar_t_dn10 = assign13830_e19033_d_n10;
        locals.var_uar_t_dn11 = assign13830_e19033_d_n11;
        locals.var_uar_t_rv = 0.0;

        let (assign13840_e19070, assign13840_e19070_d_n3, assign13840_e19070_d_n4, assign13840_e19070_d_n5, assign13840_e19070_d_n6, assign13840_e19070_d_n7, assign13840_e19070_d_n8, assign13840_e19070_d_n9, assign13840_e19070_d_n10, assign13840_e19070_d_n11,) = {
    if (locals.var_guard450 != 0.0) {
        let assign13840_e19040: f64 = (locals.var_uc1_i * locals.var_deltemp);
        let assign13840_e19041: f64 = (1.0 + assign13840_e19040);
        let assign13840_e19043: f64 = (assign13840_e19041 - 1e-6);
        let assign13840_e19047: f64 = (locals.var_uc1_i * locals.var_deltemp);
        let assign13840_e19048: f64 = (1.0 + assign13840_e19047);
        let assign13840_e19050: f64 = (assign13840_e19048 - 1e-6);
        let assign13840_e19054: f64 = (locals.var_uc1_i * locals.var_deltemp);
        let assign13840_e19055: f64 = (1.0 + assign13840_e19054);
        let assign13840_e19057: f64 = (assign13840_e19055 - 1e-6);
        let assign13840_e19058: f64 = (assign13840_e19050 * assign13840_e19057);
        let assign13840_e19061: f64 = (4.0 * 0.001);
        let assign13840_e19063: f64 = (assign13840_e19061 * 0.001);
        let assign13840_e19064: f64 = (assign13840_e19058 + assign13840_e19063);
        let assign13840_e19065: f64 = (assign13840_e19064).sqrt();
        let assign13840_e19066: f64 = (assign13840_e19043 + assign13840_e19065);
        let assign13840_e19067: f64 = (0.5 * assign13840_e19066);
        let assign13840_e19068: f64 = (locals.var_ucr_i * assign13840_e19067);
        (assign13840_e19068, (locals.var_ucr_i_dn3 * assign13840_e19067), ((locals.var_ucr_i_dn4 * assign13840_e19067) + (locals.var_ucr_i * (0.5 * ((locals.var_uc1_i * locals.var_deltemp_dn4) + ((((locals.var_uc1_i * locals.var_deltemp_dn4) * assign13840_e19057) + (assign13840_e19050 * (locals.var_uc1_i * locals.var_deltemp_dn4))) / (2.0 * assign13840_e19065)))))), ((locals.var_ucr_i_dn5 * assign13840_e19067) + (locals.var_ucr_i * (0.5 * ((locals.var_uc1_i * locals.var_deltemp_dn5) + ((((locals.var_uc1_i * locals.var_deltemp_dn5) * assign13840_e19057) + (assign13840_e19050 * (locals.var_uc1_i * locals.var_deltemp_dn5))) / (2.0 * assign13840_e19065)))))), (locals.var_ucr_i_dn6 * assign13840_e19067), (locals.var_ucr_i_dn7 * assign13840_e19067), (locals.var_ucr_i_dn8 * assign13840_e19067), (locals.var_ucr_i_dn9 * assign13840_e19067), (locals.var_ucr_i_dn10 * assign13840_e19067), (locals.var_ucr_i_dn11 * assign13840_e19067),)
    } else {
        (locals.var_ucr_t, locals.var_ucr_t_dn3, locals.var_ucr_t_dn4, locals.var_ucr_t_dn5, locals.var_ucr_t_dn6, locals.var_ucr_t_dn7, locals.var_ucr_t_dn8, locals.var_ucr_t_dn9, locals.var_ucr_t_dn10, locals.var_ucr_t_dn11,)
    }
};
        locals.var_ucr_t = assign13840_e19070;
        locals.var_ucr_t_dn3 = assign13840_e19070_d_n3;
        locals.var_ucr_t_dn4 = assign13840_e19070_d_n4;
        locals.var_ucr_t_dn5 = assign13840_e19070_d_n5;
        locals.var_ucr_t_dn6 = assign13840_e19070_d_n6;
        locals.var_ucr_t_dn7 = assign13840_e19070_d_n7;
        locals.var_ucr_t_dn8 = assign13840_e19070_d_n8;
        locals.var_ucr_t_dn9 = assign13840_e19070_d_n9;
        locals.var_ucr_t_dn10 = assign13840_e19070_d_n10;
        locals.var_ucr_t_dn11 = assign13840_e19070_d_n11;
        locals.var_ucr_t_rv = 0.0;

        let (assign13850_e19078, assign13850_e19078_d_n3, assign13850_e19078_d_n4, assign13850_e19078_d_n5, assign13850_e19078_d_n6, assign13850_e19078_d_n7, assign13850_e19078_d_n8, assign13850_e19078_d_n9, assign13850_e19078_d_n10, assign13850_e19078_d_n11,) = {
    if (locals.var_guard450 != 0.0) {
        let assign13850_e19075: f64 = (locals.var_tratio).powf(locals.var_ud1_i);
        let assign13850_e19076: f64 = (locals.var_udr_i * assign13850_e19075);
        (assign13850_e19076, (locals.var_udr_i_dn3 * assign13850_e19075), ((locals.var_udr_i_dn4 * assign13850_e19075) + (locals.var_udr_i * if 0.0 == 0.0 && ((locals.var_ud1_i) as f64).is_finite() && ((locals.var_ud1_i) as f64).fract() == 0.0 { if locals.var_ud1_i == 0.0 { 0.0 } else { (locals.var_ud1_i * ((locals.var_tratio).powf(locals.var_ud1_i - 1.0) * locals.var_tratio_dn4)) } } else { (assign13850_e19075 * (locals.var_ud1_i * (locals.var_tratio_dn4 / locals.var_tratio))) })), ((locals.var_udr_i_dn5 * assign13850_e19075) + (locals.var_udr_i * if 0.0 == 0.0 && ((locals.var_ud1_i) as f64).is_finite() && ((locals.var_ud1_i) as f64).fract() == 0.0 { if locals.var_ud1_i == 0.0 { 0.0 } else { (locals.var_ud1_i * ((locals.var_tratio).powf(locals.var_ud1_i - 1.0) * locals.var_tratio_dn5)) } } else { (assign13850_e19075 * (locals.var_ud1_i * (locals.var_tratio_dn5 / locals.var_tratio))) })), (locals.var_udr_i_dn6 * assign13850_e19075), (locals.var_udr_i_dn7 * assign13850_e19075), (locals.var_udr_i_dn8 * assign13850_e19075), (locals.var_udr_i_dn9 * assign13850_e19075), (locals.var_udr_i_dn10 * assign13850_e19075), (locals.var_udr_i_dn11 * assign13850_e19075),)
    } else {
        (locals.var_udr_t, locals.var_udr_t_dn3, locals.var_udr_t_dn4, locals.var_udr_t_dn5, locals.var_udr_t_dn6, locals.var_udr_t_dn7, locals.var_udr_t_dn8, locals.var_udr_t_dn9, locals.var_udr_t_dn10, locals.var_udr_t_dn11,)
    }
};
        locals.var_udr_t = assign13850_e19078;
        locals.var_udr_t_dn3 = assign13850_e19078_d_n3;
        locals.var_udr_t_dn4 = assign13850_e19078_d_n4;
        locals.var_udr_t_dn5 = assign13850_e19078_d_n5;
        locals.var_udr_t_dn6 = assign13850_e19078_d_n6;
        locals.var_udr_t_dn7 = assign13850_e19078_d_n7;
        locals.var_udr_t_dn8 = assign13850_e19078_d_n8;
        locals.var_udr_t_dn9 = assign13850_e19078_d_n9;
        locals.var_udr_t_dn10 = assign13850_e19078_d_n10;
        locals.var_udr_t_dn11 = assign13850_e19078_d_n11;
        locals.var_udr_t_rv = 0.0;

        let (assign13860_e19086, assign13860_e19086_d_n4, assign13860_e19086_d_n5,) = {
    if (locals.var_guard450 != 0.0) {
        let assign13860_e19083: f64 = (locals.var_tratio).powf(locals.var_ucste_i);
        let assign13860_e19084: f64 = (locals.var_ucsr_i * assign13860_e19083);
        (assign13860_e19084, (locals.var_ucsr_i * if 0.0 == 0.0 && ((locals.var_ucste_i) as f64).is_finite() && ((locals.var_ucste_i) as f64).fract() == 0.0 { if locals.var_ucste_i == 0.0 { 0.0 } else { (locals.var_ucste_i * ((locals.var_tratio).powf(locals.var_ucste_i - 1.0) * locals.var_tratio_dn4)) } } else { (assign13860_e19083 * (locals.var_ucste_i * (locals.var_tratio_dn4 / locals.var_tratio))) }), (locals.var_ucsr_i * if 0.0 == 0.0 && ((locals.var_ucste_i) as f64).is_finite() && ((locals.var_ucste_i) as f64).fract() == 0.0 { if locals.var_ucste_i == 0.0 { 0.0 } else { (locals.var_ucste_i * ((locals.var_tratio).powf(locals.var_ucste_i - 1.0) * locals.var_tratio_dn5)) } } else { (assign13860_e19083 * (locals.var_ucste_i * (locals.var_tratio_dn5 / locals.var_tratio))) }),)
    } else {
        (locals.var_ucsr_t, locals.var_ucsr_t_dn4, locals.var_ucsr_t_dn5,)
    }
};
        locals.var_ucsr_t = assign13860_e19086;
        locals.var_ucsr_t_dn4 = assign13860_e19086_d_n4;
        locals.var_ucsr_t_dn5 = assign13860_e19086_d_n5;
        locals.var_ucsr_t_rv = 0.0;

        let assign13870_e19089: f64 = (locals.var_tratio).powf(locals.var_prt_i);
        locals.var_rdstemp = assign13870_e19089;
        locals.var_rdstemp_dn4 = if 0.0 == 0.0 && ((locals.var_prt_i) as f64).is_finite() && ((locals.var_prt_i) as f64).fract() == 0.0 { if locals.var_prt_i == 0.0 { 0.0 } else { (locals.var_prt_i * ((locals.var_tratio).powf(locals.var_prt_i - 1.0) * locals.var_tratio_dn4)) } } else { (assign13870_e19089 * (locals.var_prt_i * (locals.var_tratio_dn4 / locals.var_tratio))) };
        locals.var_rdstemp_dn5 = if 0.0 == 0.0 && ((locals.var_prt_i) as f64).is_finite() && ((locals.var_prt_i) as f64).fract() == 0.0 { if locals.var_prt_i == 0.0 { 0.0 } else { (locals.var_prt_i * ((locals.var_tratio).powf(locals.var_prt_i - 1.0) * locals.var_tratio_dn5)) } } else { (assign13870_e19089 * (locals.var_prt_i * (locals.var_tratio_dn5 / locals.var_tratio))) };
        locals.var_rdstemp_rv = 0.0;

        let assign13880_e19093: f64 = (-locals.var_at_i);
        let assign13880_e19094: f64 = (locals.var_tratio).powf(assign13880_e19093);
        let assign13880_e19095: f64 = (locals.var_vsat_i * assign13880_e19094);
        locals.var_vsat_t = assign13880_e19095;
        locals.var_vsat_t_dn3 = (locals.var_vsat_i_dn3 * assign13880_e19094);
        locals.var_vsat_t_dn4 = ((locals.var_vsat_i_dn4 * assign13880_e19094) + (locals.var_vsat_i * if 0.0 == 0.0 && ((assign13880_e19093) as f64).is_finite() && ((assign13880_e19093) as f64).fract() == 0.0 { if assign13880_e19093 == 0.0 { 0.0 } else { (assign13880_e19093 * ((locals.var_tratio).powf(assign13880_e19093 - 1.0) * locals.var_tratio_dn4)) } } else { (assign13880_e19094 * (assign13880_e19093 * (locals.var_tratio_dn4 / locals.var_tratio))) }));
        locals.var_vsat_t_dn5 = ((locals.var_vsat_i_dn5 * assign13880_e19094) + (locals.var_vsat_i * if 0.0 == 0.0 && ((assign13880_e19093) as f64).is_finite() && ((assign13880_e19093) as f64).fract() == 0.0 { if assign13880_e19093 == 0.0 { 0.0 } else { (assign13880_e19093 * ((locals.var_tratio).powf(assign13880_e19093 - 1.0) * locals.var_tratio_dn5)) } } else { (assign13880_e19094 * (assign13880_e19093 * (locals.var_tratio_dn5 / locals.var_tratio))) }));
        locals.var_vsat_t_dn6 = (locals.var_vsat_i_dn6 * assign13880_e19094);
        locals.var_vsat_t_dn7 = (locals.var_vsat_i_dn7 * assign13880_e19094);
        locals.var_vsat_t_dn8 = (locals.var_vsat_i_dn8 * assign13880_e19094);
        locals.var_vsat_t_dn9 = (locals.var_vsat_i_dn9 * assign13880_e19094);
        locals.var_vsat_t_dn10 = (locals.var_vsat_i_dn10 * assign13880_e19094);
        locals.var_vsat_t_dn11 = (locals.var_vsat_i_dn11 * assign13880_e19094);
        locals.var_vsat_t_rv = 0.0;

        let assign13890_e19098: f64 = if locals.var_vsat_t < 100.0 { 1.0 } else { 0.0 };
        locals.var_guard451 = assign13890_e19098;
        locals.var_guard451_rv = 0.0;

        let (assign13900_e19102, assign13900_e19102_d_n3, assign13900_e19102_d_n4, assign13900_e19102_d_n5, assign13900_e19102_d_n6, assign13900_e19102_d_n7, assign13900_e19102_d_n8, assign13900_e19102_d_n9, assign13900_e19102_d_n10, assign13900_e19102_d_n11,) = {
    if (locals.var_guard451 != 0.0) {
        (100.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vsat_t, locals.var_vsat_t_dn3, locals.var_vsat_t_dn4, locals.var_vsat_t_dn5, locals.var_vsat_t_dn6, locals.var_vsat_t_dn7, locals.var_vsat_t_dn8, locals.var_vsat_t_dn9, locals.var_vsat_t_dn10, locals.var_vsat_t_dn11,)
    }
};
        locals.var_vsat_t = assign13900_e19102;
        locals.var_vsat_t_dn3 = assign13900_e19102_d_n3;
        locals.var_vsat_t_dn4 = assign13900_e19102_d_n4;
        locals.var_vsat_t_dn5 = assign13900_e19102_d_n5;
        locals.var_vsat_t_dn6 = assign13900_e19102_d_n6;
        locals.var_vsat_t_dn7 = assign13900_e19102_d_n7;
        locals.var_vsat_t_dn8 = assign13900_e19102_d_n8;
        locals.var_vsat_t_dn9 = assign13900_e19102_d_n9;
        locals.var_vsat_t_dn10 = assign13900_e19102_d_n10;
        locals.var_vsat_t_dn11 = assign13900_e19102_d_n11;
        locals.var_vsat_t_rv = 0.0;

        let assign13910_e19105: f64 = if p.p35 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard452 = assign13910_e19105;
        locals.var_guard452_rv = 0.0;

        let (assign13920_e19114, assign13920_e19114_d_n3, assign13920_e19114_d_n4, assign13920_e19114_d_n5, assign13920_e19114_d_n6, assign13920_e19114_d_n7, assign13920_e19114_d_n8, assign13920_e19114_d_n9, assign13920_e19114_d_n10, assign13920_e19114_d_n11,) = {
    if (locals.var_guard452 != 0.0) {
        let assign13920_e19110: f64 = (-locals.var_at_i);
        let assign13920_e19111: f64 = (locals.var_tratio).powf(assign13920_e19110);
        let assign13920_e19112: f64 = (locals.var_vsatr_i * assign13920_e19111);
        (assign13920_e19112, (locals.var_vsatr_i_dn3 * assign13920_e19111), ((locals.var_vsatr_i_dn4 * assign13920_e19111) + (locals.var_vsatr_i * if 0.0 == 0.0 && ((assign13920_e19110) as f64).is_finite() && ((assign13920_e19110) as f64).fract() == 0.0 { if assign13920_e19110 == 0.0 { 0.0 } else { (assign13920_e19110 * ((locals.var_tratio).powf(assign13920_e19110 - 1.0) * locals.var_tratio_dn4)) } } else { (assign13920_e19111 * (assign13920_e19110 * (locals.var_tratio_dn4 / locals.var_tratio))) })), ((locals.var_vsatr_i_dn5 * assign13920_e19111) + (locals.var_vsatr_i * if 0.0 == 0.0 && ((assign13920_e19110) as f64).is_finite() && ((assign13920_e19110) as f64).fract() == 0.0 { if assign13920_e19110 == 0.0 { 0.0 } else { (assign13920_e19110 * ((locals.var_tratio).powf(assign13920_e19110 - 1.0) * locals.var_tratio_dn5)) } } else { (assign13920_e19111 * (assign13920_e19110 * (locals.var_tratio_dn5 / locals.var_tratio))) })), (locals.var_vsatr_i_dn6 * assign13920_e19111), (locals.var_vsatr_i_dn7 * assign13920_e19111), (locals.var_vsatr_i_dn8 * assign13920_e19111), (locals.var_vsatr_i_dn9 * assign13920_e19111), (locals.var_vsatr_i_dn10 * assign13920_e19111), (locals.var_vsatr_i_dn11 * assign13920_e19111),)
    } else {
        (locals.var_vsatr_t, locals.var_vsatr_t_dn3, locals.var_vsatr_t_dn4, locals.var_vsatr_t_dn5, locals.var_vsatr_t_dn6, locals.var_vsatr_t_dn7, locals.var_vsatr_t_dn8, locals.var_vsatr_t_dn9, locals.var_vsatr_t_dn10, locals.var_vsatr_t_dn11,)
    }
};
        locals.var_vsatr_t = assign13920_e19114;
        locals.var_vsatr_t_dn3 = assign13920_e19114_d_n3;
        locals.var_vsatr_t_dn4 = assign13920_e19114_d_n4;
        locals.var_vsatr_t_dn5 = assign13920_e19114_d_n5;
        locals.var_vsatr_t_dn6 = assign13920_e19114_d_n6;
        locals.var_vsatr_t_dn7 = assign13920_e19114_d_n7;
        locals.var_vsatr_t_dn8 = assign13920_e19114_d_n8;
        locals.var_vsatr_t_dn9 = assign13920_e19114_d_n9;
        locals.var_vsatr_t_dn10 = assign13920_e19114_d_n10;
        locals.var_vsatr_t_dn11 = assign13920_e19114_d_n11;
        locals.var_vsatr_t_rv = 0.0;

        let assign13930_e19117: f64 = if locals.var_vsatr_t < 100.0 { 1.0 } else { 0.0 };
        locals.var_guard453 = assign13930_e19117;
        locals.var_guard453_rv = 0.0;

        let (assign13940_e19123, assign13940_e19123_d_n3, assign13940_e19123_d_n4, assign13940_e19123_d_n5, assign13940_e19123_d_n6, assign13940_e19123_d_n7, assign13940_e19123_d_n8, assign13940_e19123_d_n9, assign13940_e19123_d_n10, assign13940_e19123_d_n11,) = {
    if ((locals.var_guard452 != 0.0) && (locals.var_guard453 != 0.0)) {
        (100.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vsatr_t, locals.var_vsatr_t_dn3, locals.var_vsatr_t_dn4, locals.var_vsatr_t_dn5, locals.var_vsatr_t_dn6, locals.var_vsatr_t_dn7, locals.var_vsatr_t_dn8, locals.var_vsatr_t_dn9, locals.var_vsatr_t_dn10, locals.var_vsatr_t_dn11,)
    }
};
        locals.var_vsatr_t = assign13940_e19123;
        locals.var_vsatr_t_dn3 = assign13940_e19123_d_n3;
        locals.var_vsatr_t_dn4 = assign13940_e19123_d_n4;
        locals.var_vsatr_t_dn5 = assign13940_e19123_d_n5;
        locals.var_vsatr_t_dn6 = assign13940_e19123_d_n6;
        locals.var_vsatr_t_dn7 = assign13940_e19123_d_n7;
        locals.var_vsatr_t_dn8 = assign13940_e19123_d_n8;
        locals.var_vsatr_t_dn9 = assign13940_e19123_d_n9;
        locals.var_vsatr_t_dn10 = assign13940_e19123_d_n10;
        locals.var_vsatr_t_dn11 = assign13940_e19123_d_n11;
        locals.var_vsatr_t_rv = 0.0;

        let assign13950_e19127: f64 = (-locals.var_at_i);
        let assign13950_e19128: f64 = (locals.var_tratio).powf(assign13950_e19127);
        let assign13950_e19129: f64 = (locals.var_vsatcv_i * assign13950_e19128);
        locals.var_vsatcv_t = assign13950_e19129;
        locals.var_vsatcv_t_dn3 = (locals.var_vsatcv_i_dn3 * assign13950_e19128);
        locals.var_vsatcv_t_dn4 = ((locals.var_vsatcv_i_dn4 * assign13950_e19128) + (locals.var_vsatcv_i * if 0.0 == 0.0 && ((assign13950_e19127) as f64).is_finite() && ((assign13950_e19127) as f64).fract() == 0.0 { if assign13950_e19127 == 0.0 { 0.0 } else { (assign13950_e19127 * ((locals.var_tratio).powf(assign13950_e19127 - 1.0) * locals.var_tratio_dn4)) } } else { (assign13950_e19128 * (assign13950_e19127 * (locals.var_tratio_dn4 / locals.var_tratio))) }));
        locals.var_vsatcv_t_dn5 = ((locals.var_vsatcv_i_dn5 * assign13950_e19128) + (locals.var_vsatcv_i * if 0.0 == 0.0 && ((assign13950_e19127) as f64).is_finite() && ((assign13950_e19127) as f64).fract() == 0.0 { if assign13950_e19127 == 0.0 { 0.0 } else { (assign13950_e19127 * ((locals.var_tratio).powf(assign13950_e19127 - 1.0) * locals.var_tratio_dn5)) } } else { (assign13950_e19128 * (assign13950_e19127 * (locals.var_tratio_dn5 / locals.var_tratio))) }));
        locals.var_vsatcv_t_dn6 = (locals.var_vsatcv_i_dn6 * assign13950_e19128);
        locals.var_vsatcv_t_dn7 = (locals.var_vsatcv_i_dn7 * assign13950_e19128);
        locals.var_vsatcv_t_dn8 = (locals.var_vsatcv_i_dn8 * assign13950_e19128);
        locals.var_vsatcv_t_dn9 = (locals.var_vsatcv_i_dn9 * assign13950_e19128);
        locals.var_vsatcv_t_dn10 = (locals.var_vsatcv_i_dn10 * assign13950_e19128);
        locals.var_vsatcv_t_dn11 = (locals.var_vsatcv_i_dn11 * assign13950_e19128);
        locals.var_vsatcv_t_rv = 0.0;

        let assign13960_e19132: f64 = if locals.var_vsatcv_t < 100.0 { 1.0 } else { 0.0 };
        locals.var_guard454 = assign13960_e19132;
        locals.var_guard454_rv = 0.0;

        let (assign13970_e19136, assign13970_e19136_d_n3, assign13970_e19136_d_n4, assign13970_e19136_d_n5, assign13970_e19136_d_n6, assign13970_e19136_d_n7, assign13970_e19136_d_n8, assign13970_e19136_d_n9, assign13970_e19136_d_n10, assign13970_e19136_d_n11,) = {
    if (locals.var_guard454 != 0.0) {
        (100.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vsatcv_t, locals.var_vsatcv_t_dn3, locals.var_vsatcv_t_dn4, locals.var_vsatcv_t_dn5, locals.var_vsatcv_t_dn6, locals.var_vsatcv_t_dn7, locals.var_vsatcv_t_dn8, locals.var_vsatcv_t_dn9, locals.var_vsatcv_t_dn10, locals.var_vsatcv_t_dn11,)
    }
};
        locals.var_vsatcv_t = assign13970_e19136;
        locals.var_vsatcv_t_dn3 = assign13970_e19136_d_n3;
        locals.var_vsatcv_t_dn4 = assign13970_e19136_d_n4;
        locals.var_vsatcv_t_dn5 = assign13970_e19136_d_n5;
        locals.var_vsatcv_t_dn6 = assign13970_e19136_d_n6;
        locals.var_vsatcv_t_dn7 = assign13970_e19136_d_n7;
        locals.var_vsatcv_t_dn8 = assign13970_e19136_d_n8;
        locals.var_vsatcv_t_dn9 = assign13970_e19136_d_n9;
        locals.var_vsatcv_t_dn10 = assign13970_e19136_d_n10;
        locals.var_vsatcv_t_dn11 = assign13970_e19136_d_n11;
        locals.var_vsatcv_t_rv = 0.0;

        let assign13980_e19141: f64 = (1.0 / locals.var_delta_i);
        let assign13980_e19145: f64 = (p.p1069 * locals.var_deltemp);
        let assign13980_e19146: f64 = (1.0 + assign13980_e19145);
        let assign13980_e19147: f64 = (assign13980_e19141 * assign13980_e19146);
        let assign13980_e19149: f64 = (assign13980_e19147 - 2.0);
        let assign13980_e19152: f64 = (1.0 / locals.var_delta_i);
        let assign13980_e19156: f64 = (p.p1069 * locals.var_deltemp);
        let assign13980_e19157: f64 = (1.0 + assign13980_e19156);
        let assign13980_e19158: f64 = (assign13980_e19152 * assign13980_e19157);
        let assign13980_e19160: f64 = (assign13980_e19158 - 2.0);
        let assign13980_e19163: f64 = (1.0 / locals.var_delta_i);
        let assign13980_e19167: f64 = (p.p1069 * locals.var_deltemp);
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
        locals.var_delta_t = assign13980_e19184;
        locals.var_delta_t_dn3 = (-((0.5 * (((-(locals.var_delta_i_dn3 / (locals.var_delta_i * locals.var_delta_i))) * assign13980_e19146) + (((((-(locals.var_delta_i_dn3 / (locals.var_delta_i * locals.var_delta_i))) * assign13980_e19157) * assign13980_e19171) + (assign13980_e19160 * ((-(locals.var_delta_i_dn3 / (locals.var_delta_i * locals.var_delta_i))) * assign13980_e19168))) / (2.0 * assign13980_e19179)))) / (assign13980_e19183 * assign13980_e19183)));
        locals.var_delta_t_dn4 = (-((0.5 * ((((-(locals.var_delta_i_dn4 / (locals.var_delta_i * locals.var_delta_i))) * assign13980_e19146) + (assign13980_e19141 * (p.p1069 * locals.var_deltemp_dn4))) + ((((((-(locals.var_delta_i_dn4 / (locals.var_delta_i * locals.var_delta_i))) * assign13980_e19157) + (assign13980_e19152 * (p.p1069 * locals.var_deltemp_dn4))) * assign13980_e19171) + (assign13980_e19160 * (((-(locals.var_delta_i_dn4 / (locals.var_delta_i * locals.var_delta_i))) * assign13980_e19168) + (assign13980_e19163 * (p.p1069 * locals.var_deltemp_dn4))))) / (2.0 * assign13980_e19179)))) / (assign13980_e19183 * assign13980_e19183)));
        locals.var_delta_t_dn5 = (-((0.5 * ((((-(locals.var_delta_i_dn5 / (locals.var_delta_i * locals.var_delta_i))) * assign13980_e19146) + (assign13980_e19141 * (p.p1069 * locals.var_deltemp_dn5))) + ((((((-(locals.var_delta_i_dn5 / (locals.var_delta_i * locals.var_delta_i))) * assign13980_e19157) + (assign13980_e19152 * (p.p1069 * locals.var_deltemp_dn5))) * assign13980_e19171) + (assign13980_e19160 * (((-(locals.var_delta_i_dn5 / (locals.var_delta_i * locals.var_delta_i))) * assign13980_e19168) + (assign13980_e19163 * (p.p1069 * locals.var_deltemp_dn5))))) / (2.0 * assign13980_e19179)))) / (assign13980_e19183 * assign13980_e19183)));
        locals.var_delta_t_dn6 = (-((0.5 * (((-(locals.var_delta_i_dn6 / (locals.var_delta_i * locals.var_delta_i))) * assign13980_e19146) + (((((-(locals.var_delta_i_dn6 / (locals.var_delta_i * locals.var_delta_i))) * assign13980_e19157) * assign13980_e19171) + (assign13980_e19160 * ((-(locals.var_delta_i_dn6 / (locals.var_delta_i * locals.var_delta_i))) * assign13980_e19168))) / (2.0 * assign13980_e19179)))) / (assign13980_e19183 * assign13980_e19183)));
        locals.var_delta_t_dn7 = (-((0.5 * (((-(locals.var_delta_i_dn7 / (locals.var_delta_i * locals.var_delta_i))) * assign13980_e19146) + (((((-(locals.var_delta_i_dn7 / (locals.var_delta_i * locals.var_delta_i))) * assign13980_e19157) * assign13980_e19171) + (assign13980_e19160 * ((-(locals.var_delta_i_dn7 / (locals.var_delta_i * locals.var_delta_i))) * assign13980_e19168))) / (2.0 * assign13980_e19179)))) / (assign13980_e19183 * assign13980_e19183)));
        locals.var_delta_t_dn8 = (-((0.5 * (((-(locals.var_delta_i_dn8 / (locals.var_delta_i * locals.var_delta_i))) * assign13980_e19146) + (((((-(locals.var_delta_i_dn8 / (locals.var_delta_i * locals.var_delta_i))) * assign13980_e19157) * assign13980_e19171) + (assign13980_e19160 * ((-(locals.var_delta_i_dn8 / (locals.var_delta_i * locals.var_delta_i))) * assign13980_e19168))) / (2.0 * assign13980_e19179)))) / (assign13980_e19183 * assign13980_e19183)));
        locals.var_delta_t_dn9 = (-((0.5 * (((-(locals.var_delta_i_dn9 / (locals.var_delta_i * locals.var_delta_i))) * assign13980_e19146) + (((((-(locals.var_delta_i_dn9 / (locals.var_delta_i * locals.var_delta_i))) * assign13980_e19157) * assign13980_e19171) + (assign13980_e19160 * ((-(locals.var_delta_i_dn9 / (locals.var_delta_i * locals.var_delta_i))) * assign13980_e19168))) / (2.0 * assign13980_e19179)))) / (assign13980_e19183 * assign13980_e19183)));
        locals.var_delta_t_dn10 = (-((0.5 * (((-(locals.var_delta_i_dn10 / (locals.var_delta_i * locals.var_delta_i))) * assign13980_e19146) + (((((-(locals.var_delta_i_dn10 / (locals.var_delta_i * locals.var_delta_i))) * assign13980_e19157) * assign13980_e19171) + (assign13980_e19160 * ((-(locals.var_delta_i_dn10 / (locals.var_delta_i * locals.var_delta_i))) * assign13980_e19168))) / (2.0 * assign13980_e19179)))) / (assign13980_e19183 * assign13980_e19183)));
        locals.var_delta_t_dn11 = (-((0.5 * (((-(locals.var_delta_i_dn11 / (locals.var_delta_i * locals.var_delta_i))) * assign13980_e19146) + (((((-(locals.var_delta_i_dn11 / (locals.var_delta_i * locals.var_delta_i))) * assign13980_e19157) * assign13980_e19171) + (assign13980_e19160 * ((-(locals.var_delta_i_dn11 / (locals.var_delta_i * locals.var_delta_i))) * assign13980_e19168))) / (2.0 * assign13980_e19179)))) / (assign13980_e19183 * assign13980_e19183)));
        locals.var_delta_t_rv = 0.0;

        let assign13990_e19190: f64 = (locals.var_ptwgt_i * locals.var_deltemp);
        let assign13990_e19191: f64 = (1.0 - assign13990_e19190);
        let assign13990_e19193: f64 = (assign13990_e19191 - 1e-6);
        let assign13990_e19197: f64 = (locals.var_ptwgt_i * locals.var_deltemp);
        let assign13990_e19198: f64 = (1.0 - assign13990_e19197);
        let assign13990_e19200: f64 = (assign13990_e19198 - 1e-6);
        let assign13990_e19204: f64 = (locals.var_ptwgt_i * locals.var_deltemp);
        let assign13990_e19205: f64 = (1.0 - assign13990_e19204);
        let assign13990_e19207: f64 = (assign13990_e19205 - 1e-6);
        let assign13990_e19208: f64 = (assign13990_e19200 * assign13990_e19207);
        let assign13990_e19211: f64 = (4.0 * 0.001);
        let assign13990_e19213: f64 = (assign13990_e19211 * 0.001);
        let assign13990_e19214: f64 = (assign13990_e19208 + assign13990_e19213);
        let assign13990_e19215: f64 = (assign13990_e19214).sqrt();
        let assign13990_e19216: f64 = (assign13990_e19193 + assign13990_e19215);
        let assign13990_e19217: f64 = (0.5 * assign13990_e19216);
        let assign13990_e19218: f64 = (locals.var_ptwg_i * assign13990_e19217);
        locals.var_ptwg_t = assign13990_e19218;
        locals.var_ptwg_t_dn3 = (locals.var_ptwg_i_dn3 * assign13990_e19217);
        locals.var_ptwg_t_dn4 = ((locals.var_ptwg_i_dn4 * assign13990_e19217) + (locals.var_ptwg_i * (0.5 * ((-(locals.var_ptwgt_i * locals.var_deltemp_dn4)) + ((((-(locals.var_ptwgt_i * locals.var_deltemp_dn4)) * assign13990_e19207) + (assign13990_e19200 * (-(locals.var_ptwgt_i * locals.var_deltemp_dn4)))) / (2.0 * assign13990_e19215))))));
        locals.var_ptwg_t_dn5 = ((locals.var_ptwg_i_dn5 * assign13990_e19217) + (locals.var_ptwg_i * (0.5 * ((-(locals.var_ptwgt_i * locals.var_deltemp_dn5)) + ((((-(locals.var_ptwgt_i * locals.var_deltemp_dn5)) * assign13990_e19207) + (assign13990_e19200 * (-(locals.var_ptwgt_i * locals.var_deltemp_dn5)))) / (2.0 * assign13990_e19215))))));
        locals.var_ptwg_t_dn6 = (locals.var_ptwg_i_dn6 * assign13990_e19217);
        locals.var_ptwg_t_dn7 = (locals.var_ptwg_i_dn7 * assign13990_e19217);
        locals.var_ptwg_t_dn8 = (locals.var_ptwg_i_dn8 * assign13990_e19217);
        locals.var_ptwg_t_dn9 = (locals.var_ptwg_i_dn9 * assign13990_e19217);
        locals.var_ptwg_t_dn10 = (locals.var_ptwg_i_dn10 * assign13990_e19217);
        locals.var_ptwg_t_dn11 = (locals.var_ptwg_i_dn11 * assign13990_e19217);
        locals.var_ptwg_t_rv = 0.0;

        let assign14000_e19221: f64 = if p.p35 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard455 = assign14000_e19221;
        locals.var_guard455_rv = 0.0;

        let (assign14010_e19258, assign14010_e19258_d_n3, assign14010_e19258_d_n4, assign14010_e19258_d_n5, assign14010_e19258_d_n6, assign14010_e19258_d_n7, assign14010_e19258_d_n8, assign14010_e19258_d_n9, assign14010_e19258_d_n10, assign14010_e19258_d_n11,) = {
    if (locals.var_guard455 != 0.0) {
        let assign14010_e19228: f64 = (locals.var_ptwgt_i * locals.var_deltemp);
        let assign14010_e19229: f64 = (1.0 - assign14010_e19228);
        let assign14010_e19231: f64 = (assign14010_e19229 - 1e-6);
        let assign14010_e19235: f64 = (locals.var_ptwgt_i * locals.var_deltemp);
        let assign14010_e19236: f64 = (1.0 - assign14010_e19235);
        let assign14010_e19238: f64 = (assign14010_e19236 - 1e-6);
        let assign14010_e19242: f64 = (locals.var_ptwgt_i * locals.var_deltemp);
        let assign14010_e19243: f64 = (1.0 - assign14010_e19242);
        let assign14010_e19245: f64 = (assign14010_e19243 - 1e-6);
        let assign14010_e19246: f64 = (assign14010_e19238 * assign14010_e19245);
        let assign14010_e19249: f64 = (4.0 * 0.001);
        let assign14010_e19251: f64 = (assign14010_e19249 * 0.001);
        let assign14010_e19252: f64 = (assign14010_e19246 + assign14010_e19251);
        let assign14010_e19253: f64 = (assign14010_e19252).sqrt();
        let assign14010_e19254: f64 = (assign14010_e19231 + assign14010_e19253);
        let assign14010_e19255: f64 = (0.5 * assign14010_e19254);
        let assign14010_e19256: f64 = (locals.var_ptwgr_i * assign14010_e19255);
        (assign14010_e19256, (locals.var_ptwgr_i_dn3 * assign14010_e19255), ((locals.var_ptwgr_i_dn4 * assign14010_e19255) + (locals.var_ptwgr_i * (0.5 * ((-(locals.var_ptwgt_i * locals.var_deltemp_dn4)) + ((((-(locals.var_ptwgt_i * locals.var_deltemp_dn4)) * assign14010_e19245) + (assign14010_e19238 * (-(locals.var_ptwgt_i * locals.var_deltemp_dn4)))) / (2.0 * assign14010_e19253)))))), ((locals.var_ptwgr_i_dn5 * assign14010_e19255) + (locals.var_ptwgr_i * (0.5 * ((-(locals.var_ptwgt_i * locals.var_deltemp_dn5)) + ((((-(locals.var_ptwgt_i * locals.var_deltemp_dn5)) * assign14010_e19245) + (assign14010_e19238 * (-(locals.var_ptwgt_i * locals.var_deltemp_dn5)))) / (2.0 * assign14010_e19253)))))), (locals.var_ptwgr_i_dn6 * assign14010_e19255), (locals.var_ptwgr_i_dn7 * assign14010_e19255), (locals.var_ptwgr_i_dn8 * assign14010_e19255), (locals.var_ptwgr_i_dn9 * assign14010_e19255), (locals.var_ptwgr_i_dn10 * assign14010_e19255), (locals.var_ptwgr_i_dn11 * assign14010_e19255),)
    } else {
        (locals.var_ptwgr_t, locals.var_ptwgr_t_dn3, locals.var_ptwgr_t_dn4, locals.var_ptwgr_t_dn5, locals.var_ptwgr_t_dn6, locals.var_ptwgr_t_dn7, locals.var_ptwgr_t_dn8, locals.var_ptwgr_t_dn9, locals.var_ptwgr_t_dn10, locals.var_ptwgr_t_dn11,)
    }
};
        locals.var_ptwgr_t = assign14010_e19258;
        locals.var_ptwgr_t_dn3 = assign14010_e19258_d_n3;
        locals.var_ptwgr_t_dn4 = assign14010_e19258_d_n4;
        locals.var_ptwgr_t_dn5 = assign14010_e19258_d_n5;
        locals.var_ptwgr_t_dn6 = assign14010_e19258_d_n6;
        locals.var_ptwgr_t_dn7 = assign14010_e19258_d_n7;
        locals.var_ptwgr_t_dn8 = assign14010_e19258_d_n8;
        locals.var_ptwgr_t_dn9 = assign14010_e19258_d_n9;
        locals.var_ptwgr_t_dn10 = assign14010_e19258_d_n10;
        locals.var_ptwgr_t_dn11 = assign14010_e19258_d_n11;
        locals.var_ptwgr_t_rv = 0.0;

        let assign14020_e19264: f64 = (locals.var_a11_i * locals.var_deltemp);
        let assign14020_e19265: f64 = (1.0 + assign14020_e19264);
        let assign14020_e19267: f64 = (assign14020_e19265 - 1e-6);
        let assign14020_e19271: f64 = (locals.var_a11_i * locals.var_deltemp);
        let assign14020_e19272: f64 = (1.0 + assign14020_e19271);
        let assign14020_e19274: f64 = (assign14020_e19272 - 1e-6);
        let assign14020_e19278: f64 = (locals.var_a11_i * locals.var_deltemp);
        let assign14020_e19279: f64 = (1.0 + assign14020_e19278);
        let assign14020_e19281: f64 = (assign14020_e19279 - 1e-6);
        let assign14020_e19282: f64 = (assign14020_e19274 * assign14020_e19281);
        let assign14020_e19285: f64 = (4.0 * 0.001);
        let assign14020_e19287: f64 = (assign14020_e19285 * 0.001);
        let assign14020_e19288: f64 = (assign14020_e19282 + assign14020_e19287);
        let assign14020_e19289: f64 = (assign14020_e19288).sqrt();
        let assign14020_e19290: f64 = (assign14020_e19267 + assign14020_e19289);
        let assign14020_e19291: f64 = (0.5 * assign14020_e19290);
        let assign14020_e19292: f64 = (locals.var_a1_i * assign14020_e19291);
        locals.var_a1_t = assign14020_e19292;
        locals.var_a1_t_dn4 = (locals.var_a1_i * (0.5 * ((locals.var_a11_i * locals.var_deltemp_dn4) + ((((locals.var_a11_i * locals.var_deltemp_dn4) * assign14020_e19281) + (assign14020_e19274 * (locals.var_a11_i * locals.var_deltemp_dn4))) / (2.0 * assign14020_e19289)))));
        locals.var_a1_t_dn5 = (locals.var_a1_i * (0.5 * ((locals.var_a11_i * locals.var_deltemp_dn5) + ((((locals.var_a11_i * locals.var_deltemp_dn5) * assign14020_e19281) + (assign14020_e19274 * (locals.var_a11_i * locals.var_deltemp_dn5))) / (2.0 * assign14020_e19289)))));
        locals.var_a1_t_rv = 0.0;

        let assign14030_e19298: f64 = (locals.var_a21_i * locals.var_deltemp);
        let assign14030_e19299: f64 = (1.0 + assign14030_e19298);
        let assign14030_e19301: f64 = (assign14030_e19299 - 1e-6);
        let assign14030_e19305: f64 = (locals.var_a21_i * locals.var_deltemp);
        let assign14030_e19306: f64 = (1.0 + assign14030_e19305);
        let assign14030_e19308: f64 = (assign14030_e19306 - 1e-6);
        let assign14030_e19312: f64 = (locals.var_a21_i * locals.var_deltemp);
        let assign14030_e19313: f64 = (1.0 + assign14030_e19312);
        let assign14030_e19315: f64 = (assign14030_e19313 - 1e-6);
        let assign14030_e19316: f64 = (assign14030_e19308 * assign14030_e19315);
        let assign14030_e19319: f64 = (4.0 * 0.001);
        let assign14030_e19321: f64 = (assign14030_e19319 * 0.001);
        let assign14030_e19322: f64 = (assign14030_e19316 + assign14030_e19321);
        let assign14030_e19323: f64 = (assign14030_e19322).sqrt();
        let assign14030_e19324: f64 = (assign14030_e19301 + assign14030_e19323);
        let assign14030_e19325: f64 = (0.5 * assign14030_e19324);
        let assign14030_e19326: f64 = (locals.var_a2_i * assign14030_e19325);
        locals.var_a2_t = assign14030_e19326;
        locals.var_a2_t_dn4 = (locals.var_a2_i * (0.5 * ((locals.var_a21_i * locals.var_deltemp_dn4) + ((((locals.var_a21_i * locals.var_deltemp_dn4) * assign14030_e19315) + (assign14030_e19308 * (locals.var_a21_i * locals.var_deltemp_dn4))) / (2.0 * assign14030_e19323)))));
        locals.var_a2_t_dn5 = (locals.var_a2_i * (0.5 * ((locals.var_a21_i * locals.var_deltemp_dn5) + ((((locals.var_a21_i * locals.var_deltemp_dn5) * assign14030_e19315) + (assign14030_e19308 * (locals.var_a21_i * locals.var_deltemp_dn5))) / (2.0 * assign14030_e19323)))));
        locals.var_a2_t_rv = 0.0;

        let assign14040_e19330: f64 = (locals.var_tratio).powf(locals.var_iit_i);
        let assign14040_e19331: f64 = (locals.var_beta0_i * assign14040_e19330);
        locals.var_beta0_t = assign14040_e19331;
        locals.var_beta0_t_dn4 = (locals.var_beta0_i * if 0.0 == 0.0 && ((locals.var_iit_i) as f64).is_finite() && ((locals.var_iit_i) as f64).fract() == 0.0 { if locals.var_iit_i == 0.0 { 0.0 } else { (locals.var_iit_i * ((locals.var_tratio).powf(locals.var_iit_i - 1.0) * locals.var_tratio_dn4)) } } else { (assign14040_e19330 * (locals.var_iit_i * (locals.var_tratio_dn4 / locals.var_tratio))) });
        locals.var_beta0_t_dn5 = (locals.var_beta0_i * if 0.0 == 0.0 && ((locals.var_iit_i) as f64).is_finite() && ((locals.var_iit_i) as f64).fract() == 0.0 { if locals.var_iit_i == 0.0 { 0.0 } else { (locals.var_iit_i * ((locals.var_tratio).powf(locals.var_iit_i - 1.0) * locals.var_tratio_dn5)) } } else { (assign14040_e19330 * (locals.var_iit_i * (locals.var_tratio_dn5 / locals.var_tratio))) });
        locals.var_beta0_t_rv = 0.0;

        let assign14050_e19336: f64 = (locals.var_tratio - 1.0);
        let assign14050_e19337: f64 = (locals.var_bgidl1_i * assign14050_e19336);
        let assign14050_e19338: f64 = (locals.var_bgidl_i + assign14050_e19337);
        locals.var_bgidl_t = assign14050_e19338;
        locals.var_bgidl_t_dn4 = (locals.var_bgidl1_i * locals.var_tratio_dn4);
        locals.var_bgidl_t_dn5 = (locals.var_bgidl1_i * locals.var_tratio_dn5);
        locals.var_bgidl_t_rv = 0.0;

        let assign14060_e19343: f64 = (locals.var_tratio - 1.0);
        let assign14060_e19344: f64 = (locals.var_bgisl1_i * assign14060_e19343);
        let assign14060_e19345: f64 = (locals.var_bgisl_i + assign14060_e19344);
        locals.var_bgisl_t = assign14060_e19345;
        locals.var_bgisl_t_dn4 = (locals.var_bgisl1_i * locals.var_tratio_dn4);
        locals.var_bgisl_t_dn5 = (locals.var_bgisl1_i * locals.var_tratio_dn5);
        locals.var_bgisl_t_rv = 0.0;

        let assign14080_e19358: f64 = (locals.var_k01_i * locals.var_deltemp);
        let assign14080_e19359: f64 = (1.0 + assign14080_e19358);
        let assign14080_e19361: f64 = (assign14080_e19359 - 1e-6);
        let assign14080_e19365: f64 = (locals.var_k01_i * locals.var_deltemp);
        let assign14080_e19366: f64 = (1.0 + assign14080_e19365);
        let assign14080_e19368: f64 = (assign14080_e19366 - 1e-6);
        let assign14080_e19372: f64 = (locals.var_k01_i * locals.var_deltemp);
        let assign14080_e19373: f64 = (1.0 + assign14080_e19372);
        let assign14080_e19375: f64 = (assign14080_e19373 - 1e-6);
        let assign14080_e19376: f64 = (assign14080_e19368 * assign14080_e19375);
        let assign14080_e19379: f64 = (4.0 * 0.001);
        let assign14080_e19381: f64 = (assign14080_e19379 * 0.001);
        let assign14080_e19382: f64 = (assign14080_e19376 + assign14080_e19381);
        let assign14080_e19383: f64 = (assign14080_e19382).sqrt();
        let assign14080_e19384: f64 = (assign14080_e19361 + assign14080_e19383);
        let assign14080_e19385: f64 = (0.5 * assign14080_e19384);
        let assign14080_e19386: f64 = (locals.var_k0_i * assign14080_e19385);
        locals.var_k0_t = assign14080_e19386;
        locals.var_k0_t_dn4 = (locals.var_k0_i * (0.5 * ((locals.var_k01_i * locals.var_deltemp_dn4) + ((((locals.var_k01_i * locals.var_deltemp_dn4) * assign14080_e19375) + (assign14080_e19368 * (locals.var_k01_i * locals.var_deltemp_dn4))) / (2.0 * assign14080_e19383)))));
        locals.var_k0_t_dn5 = (locals.var_k0_i * (0.5 * ((locals.var_k01_i * locals.var_deltemp_dn5) + ((((locals.var_k01_i * locals.var_deltemp_dn5) * assign14080_e19375) + (assign14080_e19368 * (locals.var_k01_i * locals.var_deltemp_dn5))) / (2.0 * assign14080_e19383)))));
        locals.var_k0_t_rv = 0.0;

        let assign14090_e19392: f64 = (locals.var_m01_i * locals.var_deltemp);
        let assign14090_e19393: f64 = (1.0 + assign14090_e19392);
        let assign14090_e19395: f64 = (assign14090_e19393 - 1e-6);
        let assign14090_e19399: f64 = (locals.var_m01_i * locals.var_deltemp);
        let assign14090_e19400: f64 = (1.0 + assign14090_e19399);
        let assign14090_e19402: f64 = (assign14090_e19400 - 1e-6);
        let assign14090_e19406: f64 = (locals.var_m01_i * locals.var_deltemp);
        let assign14090_e19407: f64 = (1.0 + assign14090_e19406);
        let assign14090_e19409: f64 = (assign14090_e19407 - 1e-6);
        let assign14090_e19410: f64 = (assign14090_e19402 * assign14090_e19409);
        let assign14090_e19413: f64 = (4.0 * 0.001);
        let assign14090_e19415: f64 = (assign14090_e19413 * 0.001);
        let assign14090_e19416: f64 = (assign14090_e19410 + assign14090_e19415);
        let assign14090_e19417: f64 = (assign14090_e19416).sqrt();
        let assign14090_e19418: f64 = (assign14090_e19395 + assign14090_e19417);
        let assign14090_e19419: f64 = (0.5 * assign14090_e19418);
        let assign14090_e19420: f64 = (locals.var_m0_i * assign14090_e19419);
        locals.var_m0_t = assign14090_e19420;
        locals.var_m0_t_dn4 = (locals.var_m0_i * (0.5 * ((locals.var_m01_i * locals.var_deltemp_dn4) + ((((locals.var_m01_i * locals.var_deltemp_dn4) * assign14090_e19409) + (assign14090_e19402 * (locals.var_m01_i * locals.var_deltemp_dn4))) / (2.0 * assign14090_e19417)))));
        locals.var_m0_t_dn5 = (locals.var_m0_i * (0.5 * ((locals.var_m01_i * locals.var_deltemp_dn5) + ((((locals.var_m01_i * locals.var_deltemp_dn5) * assign14090_e19409) + (assign14090_e19402 * (locals.var_m01_i * locals.var_deltemp_dn5))) / (2.0 * assign14090_e19417)))));
        locals.var_m0_t_rv = 0.0;

        let assign14100_e19426: f64 = (locals.var_c01_i * locals.var_deltemp);
        let assign14100_e19427: f64 = (1.0 + assign14100_e19426);
        let assign14100_e19429: f64 = (assign14100_e19427 - 1e-6);
        let assign14100_e19433: f64 = (locals.var_c01_i * locals.var_deltemp);
        let assign14100_e19434: f64 = (1.0 + assign14100_e19433);
        let assign14100_e19436: f64 = (assign14100_e19434 - 1e-6);
        let assign14100_e19440: f64 = (locals.var_c01_i * locals.var_deltemp);
        let assign14100_e19441: f64 = (1.0 + assign14100_e19440);
        let assign14100_e19443: f64 = (assign14100_e19441 - 1e-6);
        let assign14100_e19444: f64 = (assign14100_e19436 * assign14100_e19443);
        let assign14100_e19447: f64 = (4.0 * 0.001);
        let assign14100_e19449: f64 = (assign14100_e19447 * 0.001);
        let assign14100_e19450: f64 = (assign14100_e19444 + assign14100_e19449);
        let assign14100_e19451: f64 = (assign14100_e19450).sqrt();
        let assign14100_e19452: f64 = (assign14100_e19429 + assign14100_e19451);
        let assign14100_e19453: f64 = (0.5 * assign14100_e19452);
        let assign14100_e19454: f64 = (locals.var_c0_i * assign14100_e19453);
        locals.var_c0_t = assign14100_e19454;
        locals.var_c0_t_dn4 = (locals.var_c0_i * (0.5 * ((locals.var_c01_i * locals.var_deltemp_dn4) + ((((locals.var_c01_i * locals.var_deltemp_dn4) * assign14100_e19443) + (assign14100_e19436 * (locals.var_c01_i * locals.var_deltemp_dn4))) / (2.0 * assign14100_e19451)))));
        locals.var_c0_t_dn5 = (locals.var_c0_i * (0.5 * ((locals.var_c01_i * locals.var_deltemp_dn5) + ((((locals.var_c01_i * locals.var_deltemp_dn5) * assign14100_e19443) + (assign14100_e19436 * (locals.var_c01_i * locals.var_deltemp_dn5))) / (2.0 * assign14100_e19451)))));
        locals.var_c0_t_rv = 0.0;

        let assign14110_e19460: f64 = (locals.var_c0si1_i * locals.var_deltemp);
        let assign14110_e19461: f64 = (1.0 + assign14110_e19460);
        let assign14110_e19463: f64 = (assign14110_e19461 - 1e-6);
        let assign14110_e19467: f64 = (locals.var_c0si1_i * locals.var_deltemp);
        let assign14110_e19468: f64 = (1.0 + assign14110_e19467);
        let assign14110_e19470: f64 = (assign14110_e19468 - 1e-6);
        let assign14110_e19474: f64 = (locals.var_c0si1_i * locals.var_deltemp);
        let assign14110_e19475: f64 = (1.0 + assign14110_e19474);
        let assign14110_e19477: f64 = (assign14110_e19475 - 1e-6);
        let assign14110_e19478: f64 = (assign14110_e19470 * assign14110_e19477);
        let assign14110_e19481: f64 = (4.0 * 0.001);
        let assign14110_e19483: f64 = (assign14110_e19481 * 0.001);
        let assign14110_e19484: f64 = (assign14110_e19478 + assign14110_e19483);
        let assign14110_e19485: f64 = (assign14110_e19484).sqrt();
        let assign14110_e19486: f64 = (assign14110_e19463 + assign14110_e19485);
        let assign14110_e19487: f64 = (0.5 * assign14110_e19486);
        let assign14110_e19488: f64 = (locals.var_c0si_i * assign14110_e19487);
        locals.var_c0si_t = assign14110_e19488;
        locals.var_c0si_t_dn4 = (locals.var_c0si_i * (0.5 * ((locals.var_c0si1_i * locals.var_deltemp_dn4) + ((((locals.var_c0si1_i * locals.var_deltemp_dn4) * assign14110_e19477) + (assign14110_e19470 * (locals.var_c0si1_i * locals.var_deltemp_dn4))) / (2.0 * assign14110_e19485)))));
        locals.var_c0si_t_dn5 = (locals.var_c0si_i * (0.5 * ((locals.var_c0si1_i * locals.var_deltemp_dn5) + ((((locals.var_c0si1_i * locals.var_deltemp_dn5) * assign14110_e19477) + (assign14110_e19470 * (locals.var_c0si1_i * locals.var_deltemp_dn5))) / (2.0 * assign14110_e19485)))));
        locals.var_c0si_t_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_24(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign14120_e19494: f64 = (locals.var_c0sisat1_i * locals.var_deltemp);
        let assign14120_e19495: f64 = (1.0 + assign14120_e19494);
        let assign14120_e19497: f64 = (assign14120_e19495 - 1e-6);
        let assign14120_e19501: f64 = (locals.var_c0sisat1_i * locals.var_deltemp);
        let assign14120_e19502: f64 = (1.0 + assign14120_e19501);
        let assign14120_e19504: f64 = (assign14120_e19502 - 1e-6);
        let assign14120_e19508: f64 = (locals.var_c0sisat1_i * locals.var_deltemp);
        let assign14120_e19509: f64 = (1.0 + assign14120_e19508);
        let assign14120_e19511: f64 = (assign14120_e19509 - 1e-6);
        let assign14120_e19512: f64 = (assign14120_e19504 * assign14120_e19511);
        let assign14120_e19515: f64 = (4.0 * 0.001);
        let assign14120_e19517: f64 = (assign14120_e19515 * 0.001);
        let assign14120_e19518: f64 = (assign14120_e19512 + assign14120_e19517);
        let assign14120_e19519: f64 = (assign14120_e19518).sqrt();
        let assign14120_e19520: f64 = (assign14120_e19497 + assign14120_e19519);
        let assign14120_e19521: f64 = (0.5 * assign14120_e19520);
        let assign14120_e19522: f64 = (locals.var_c0sisat_i * assign14120_e19521);
        locals.var_c0sisat_t = assign14120_e19522;
        locals.var_c0sisat_t_dn4 = (locals.var_c0sisat_i * (0.5 * ((locals.var_c0sisat1_i * locals.var_deltemp_dn4) + ((((locals.var_c0sisat1_i * locals.var_deltemp_dn4) * assign14120_e19511) + (assign14120_e19504 * (locals.var_c0sisat1_i * locals.var_deltemp_dn4))) / (2.0 * assign14120_e19519)))));
        locals.var_c0sisat_t_dn5 = (locals.var_c0sisat_i * (0.5 * ((locals.var_c0sisat1_i * locals.var_deltemp_dn5) + ((((locals.var_c0sisat1_i * locals.var_deltemp_dn5) * assign14120_e19511) + (assign14120_e19504 * (locals.var_c0sisat1_i * locals.var_deltemp_dn5))) / (2.0 * assign14120_e19519)))));
        locals.var_c0sisat_t_rv = 0.0;

        let assign14130_e19528: f64 = (p.p1093 * locals.var_deltemp);
        let assign14130_e19529: f64 = (1.0 + assign14130_e19528);
        let assign14130_e19531: f64 = (assign14130_e19529 - 1e-6);
        let assign14130_e19535: f64 = (p.p1093 * locals.var_deltemp);
        let assign14130_e19536: f64 = (1.0 + assign14130_e19535);
        let assign14130_e19538: f64 = (assign14130_e19536 - 1e-6);
        let assign14130_e19542: f64 = (p.p1093 * locals.var_deltemp);
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
        locals.var_cjs_t = assign14130_e19556;
        locals.var_cjs_t_dn4 = (p.p901 * (0.5 * ((p.p1093 * locals.var_deltemp_dn4) + ((((p.p1093 * locals.var_deltemp_dn4) * assign14130_e19545) + (assign14130_e19538 * (p.p1093 * locals.var_deltemp_dn4))) / (2.0 * assign14130_e19553)))));
        locals.var_cjs_t_dn5 = (p.p901 * (0.5 * ((p.p1093 * locals.var_deltemp_dn5) + ((((p.p1093 * locals.var_deltemp_dn5) * assign14130_e19545) + (assign14130_e19538 * (p.p1093 * locals.var_deltemp_dn5))) / (2.0 * assign14130_e19553)))));
        locals.var_cjs_t_rv = 0.0;

        let assign14140_e19562: f64 = (p.p1093 * locals.var_deltemp);
        let assign14140_e19563: f64 = (1.0 + assign14140_e19562);
        let assign14140_e19565: f64 = (assign14140_e19563 - 1e-6);
        let assign14140_e19569: f64 = (p.p1093 * locals.var_deltemp);
        let assign14140_e19570: f64 = (1.0 + assign14140_e19569);
        let assign14140_e19572: f64 = (assign14140_e19570 - 1e-6);
        let assign14140_e19576: f64 = (p.p1093 * locals.var_deltemp);
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
        locals.var_cjd_t = assign14140_e19590;
        locals.var_cjd_t_dn4 = (p.p902 * (0.5 * ((p.p1093 * locals.var_deltemp_dn4) + ((((p.p1093 * locals.var_deltemp_dn4) * assign14140_e19579) + (assign14140_e19572 * (p.p1093 * locals.var_deltemp_dn4))) / (2.0 * assign14140_e19587)))));
        locals.var_cjd_t_dn5 = (p.p902 * (0.5 * ((p.p1093 * locals.var_deltemp_dn5) + ((((p.p1093 * locals.var_deltemp_dn5) * assign14140_e19579) + (assign14140_e19572 * (p.p1093 * locals.var_deltemp_dn5))) / (2.0 * assign14140_e19587)))));
        locals.var_cjd_t_rv = 0.0;

        let assign14150_e19596: f64 = (p.p1094 * locals.var_deltemp);
        let assign14150_e19597: f64 = (1.0 + assign14150_e19596);
        let assign14150_e19599: f64 = (assign14150_e19597 - 1e-6);
        let assign14150_e19603: f64 = (p.p1094 * locals.var_deltemp);
        let assign14150_e19604: f64 = (1.0 + assign14150_e19603);
        let assign14150_e19606: f64 = (assign14150_e19604 - 1e-6);
        let assign14150_e19610: f64 = (p.p1094 * locals.var_deltemp);
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
        locals.var_cjsws_t = assign14150_e19624;
        locals.var_cjsws_t_dn4 = (p.p903 * (0.5 * ((p.p1094 * locals.var_deltemp_dn4) + ((((p.p1094 * locals.var_deltemp_dn4) * assign14150_e19613) + (assign14150_e19606 * (p.p1094 * locals.var_deltemp_dn4))) / (2.0 * assign14150_e19621)))));
        locals.var_cjsws_t_dn5 = (p.p903 * (0.5 * ((p.p1094 * locals.var_deltemp_dn5) + ((((p.p1094 * locals.var_deltemp_dn5) * assign14150_e19613) + (assign14150_e19606 * (p.p1094 * locals.var_deltemp_dn5))) / (2.0 * assign14150_e19621)))));
        locals.var_cjsws_t_rv = 0.0;

        let assign14160_e19630: f64 = (p.p1094 * locals.var_deltemp);
        let assign14160_e19631: f64 = (1.0 + assign14160_e19630);
        let assign14160_e19633: f64 = (assign14160_e19631 - 1e-6);
        let assign14160_e19637: f64 = (p.p1094 * locals.var_deltemp);
        let assign14160_e19638: f64 = (1.0 + assign14160_e19637);
        let assign14160_e19640: f64 = (assign14160_e19638 - 1e-6);
        let assign14160_e19644: f64 = (p.p1094 * locals.var_deltemp);
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
        locals.var_cjswd_t = assign14160_e19658;
        locals.var_cjswd_t_dn4 = (p.p904 * (0.5 * ((p.p1094 * locals.var_deltemp_dn4) + ((((p.p1094 * locals.var_deltemp_dn4) * assign14160_e19647) + (assign14160_e19640 * (p.p1094 * locals.var_deltemp_dn4))) / (2.0 * assign14160_e19655)))));
        locals.var_cjswd_t_dn5 = (p.p904 * (0.5 * ((p.p1094 * locals.var_deltemp_dn5) + ((((p.p1094 * locals.var_deltemp_dn5) * assign14160_e19647) + (assign14160_e19640 * (p.p1094 * locals.var_deltemp_dn5))) / (2.0 * assign14160_e19655)))));
        locals.var_cjswd_t_rv = 0.0;

        let assign14170_e19664: f64 = (p.p1095 * locals.var_deltemp);
        let assign14170_e19665: f64 = (1.0 + assign14170_e19664);
        let assign14170_e19667: f64 = (assign14170_e19665 - 1e-6);
        let assign14170_e19671: f64 = (p.p1095 * locals.var_deltemp);
        let assign14170_e19672: f64 = (1.0 + assign14170_e19671);
        let assign14170_e19674: f64 = (assign14170_e19672 - 1e-6);
        let assign14170_e19678: f64 = (p.p1095 * locals.var_deltemp);
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
        locals.var_cjswgs_t = assign14170_e19692;
        locals.var_cjswgs_t_dn4 = (p.p905 * (0.5 * ((p.p1095 * locals.var_deltemp_dn4) + ((((p.p1095 * locals.var_deltemp_dn4) * assign14170_e19681) + (assign14170_e19674 * (p.p1095 * locals.var_deltemp_dn4))) / (2.0 * assign14170_e19689)))));
        locals.var_cjswgs_t_dn5 = (p.p905 * (0.5 * ((p.p1095 * locals.var_deltemp_dn5) + ((((p.p1095 * locals.var_deltemp_dn5) * assign14170_e19681) + (assign14170_e19674 * (p.p1095 * locals.var_deltemp_dn5))) / (2.0 * assign14170_e19689)))));
        locals.var_cjswgs_t_rv = 0.0;

        let assign14180_e19698: f64 = (p.p1095 * locals.var_deltemp);
        let assign14180_e19699: f64 = (1.0 + assign14180_e19698);
        let assign14180_e19701: f64 = (assign14180_e19699 - 1e-6);
        let assign14180_e19705: f64 = (p.p1095 * locals.var_deltemp);
        let assign14180_e19706: f64 = (1.0 + assign14180_e19705);
        let assign14180_e19708: f64 = (assign14180_e19706 - 1e-6);
        let assign14180_e19712: f64 = (p.p1095 * locals.var_deltemp);
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
        locals.var_cjswgd_t = assign14180_e19726;
        locals.var_cjswgd_t_dn4 = (p.p906 * (0.5 * ((p.p1095 * locals.var_deltemp_dn4) + ((((p.p1095 * locals.var_deltemp_dn4) * assign14180_e19715) + (assign14180_e19708 * (p.p1095 * locals.var_deltemp_dn4))) / (2.0 * assign14180_e19723)))));
        locals.var_cjswgd_t_dn5 = (p.p906 * (0.5 * ((p.p1095 * locals.var_deltemp_dn5) + ((((p.p1095 * locals.var_deltemp_dn5) * assign14180_e19715) + (assign14180_e19708 * (p.p1095 * locals.var_deltemp_dn5))) / (2.0 * assign14180_e19723)))));
        locals.var_cjswgd_t_rv = 0.0;

        let assign14190_e19731: f64 = (p.p1096 * locals.var_deltemp);
        let assign14190_e19732: f64 = (p.p907 - assign14190_e19731);
        let assign14190_e19734: f64 = (assign14190_e19732 - 0.01);
        let assign14190_e19738: f64 = (p.p1096 * locals.var_deltemp);
        let assign14190_e19739: f64 = (p.p907 - assign14190_e19738);
        let assign14190_e19741: f64 = (assign14190_e19739 - 0.01);
        let assign14190_e19745: f64 = (p.p1096 * locals.var_deltemp);
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
        locals.var_pbs_t = assign14190_e19760;
        locals.var_pbs_t_dn4 = (0.5 * ((-(p.p1096 * locals.var_deltemp_dn4)) + ((((-(p.p1096 * locals.var_deltemp_dn4)) * assign14190_e19748) + (assign14190_e19741 * (-(p.p1096 * locals.var_deltemp_dn4)))) / (2.0 * assign14190_e19756))));
        locals.var_pbs_t_dn5 = (0.5 * ((-(p.p1096 * locals.var_deltemp_dn5)) + ((((-(p.p1096 * locals.var_deltemp_dn5)) * assign14190_e19748) + (assign14190_e19741 * (-(p.p1096 * locals.var_deltemp_dn5)))) / (2.0 * assign14190_e19756))));
        locals.var_pbs_t_rv = 0.0;

        let assign14200_e19765: f64 = (p.p1096 * locals.var_deltemp);
        let assign14200_e19766: f64 = (p.p908 - assign14200_e19765);
        let assign14200_e19768: f64 = (assign14200_e19766 - 0.01);
        let assign14200_e19772: f64 = (p.p1096 * locals.var_deltemp);
        let assign14200_e19773: f64 = (p.p908 - assign14200_e19772);
        let assign14200_e19775: f64 = (assign14200_e19773 - 0.01);
        let assign14200_e19779: f64 = (p.p1096 * locals.var_deltemp);
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
        locals.var_pbd_t = assign14200_e19794;
        locals.var_pbd_t_dn4 = (0.5 * ((-(p.p1096 * locals.var_deltemp_dn4)) + ((((-(p.p1096 * locals.var_deltemp_dn4)) * assign14200_e19782) + (assign14200_e19775 * (-(p.p1096 * locals.var_deltemp_dn4)))) / (2.0 * assign14200_e19790))));
        locals.var_pbd_t_dn5 = (0.5 * ((-(p.p1096 * locals.var_deltemp_dn5)) + ((((-(p.p1096 * locals.var_deltemp_dn5)) * assign14200_e19782) + (assign14200_e19775 * (-(p.p1096 * locals.var_deltemp_dn5)))) / (2.0 * assign14200_e19790))));
        locals.var_pbd_t_rv = 0.0;

        let assign14210_e19799: f64 = (p.p1097 * locals.var_deltemp);
        let assign14210_e19800: f64 = (p.p909 - assign14210_e19799);
        let assign14210_e19802: f64 = (assign14210_e19800 - 0.01);
        let assign14210_e19806: f64 = (p.p1097 * locals.var_deltemp);
        let assign14210_e19807: f64 = (p.p909 - assign14210_e19806);
        let assign14210_e19809: f64 = (assign14210_e19807 - 0.01);
        let assign14210_e19813: f64 = (p.p1097 * locals.var_deltemp);
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
        locals.var_pbsws_t = assign14210_e19828;
        locals.var_pbsws_t_dn4 = (0.5 * ((-(p.p1097 * locals.var_deltemp_dn4)) + ((((-(p.p1097 * locals.var_deltemp_dn4)) * assign14210_e19816) + (assign14210_e19809 * (-(p.p1097 * locals.var_deltemp_dn4)))) / (2.0 * assign14210_e19824))));
        locals.var_pbsws_t_dn5 = (0.5 * ((-(p.p1097 * locals.var_deltemp_dn5)) + ((((-(p.p1097 * locals.var_deltemp_dn5)) * assign14210_e19816) + (assign14210_e19809 * (-(p.p1097 * locals.var_deltemp_dn5)))) / (2.0 * assign14210_e19824))));
        locals.var_pbsws_t_rv = 0.0;

        let assign14220_e19833: f64 = (p.p1097 * locals.var_deltemp);
        let assign14220_e19834: f64 = (p.p910 - assign14220_e19833);
        let assign14220_e19836: f64 = (assign14220_e19834 - 0.01);
        let assign14220_e19840: f64 = (p.p1097 * locals.var_deltemp);
        let assign14220_e19841: f64 = (p.p910 - assign14220_e19840);
        let assign14220_e19843: f64 = (assign14220_e19841 - 0.01);
        let assign14220_e19847: f64 = (p.p1097 * locals.var_deltemp);
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
        locals.var_pbswd_t = assign14220_e19862;
        locals.var_pbswd_t_dn4 = (0.5 * ((-(p.p1097 * locals.var_deltemp_dn4)) + ((((-(p.p1097 * locals.var_deltemp_dn4)) * assign14220_e19850) + (assign14220_e19843 * (-(p.p1097 * locals.var_deltemp_dn4)))) / (2.0 * assign14220_e19858))));
        locals.var_pbswd_t_dn5 = (0.5 * ((-(p.p1097 * locals.var_deltemp_dn5)) + ((((-(p.p1097 * locals.var_deltemp_dn5)) * assign14220_e19850) + (assign14220_e19843 * (-(p.p1097 * locals.var_deltemp_dn5)))) / (2.0 * assign14220_e19858))));
        locals.var_pbswd_t_rv = 0.0;

        let assign14230_e19867: f64 = (p.p1098 * locals.var_deltemp);
        let assign14230_e19868: f64 = (p.p911 - assign14230_e19867);
        let assign14230_e19870: f64 = (assign14230_e19868 - 0.01);
        let assign14230_e19874: f64 = (p.p1098 * locals.var_deltemp);
        let assign14230_e19875: f64 = (p.p911 - assign14230_e19874);
        let assign14230_e19877: f64 = (assign14230_e19875 - 0.01);
        let assign14230_e19881: f64 = (p.p1098 * locals.var_deltemp);
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
        locals.var_pbswgs_t = assign14230_e19896;
        locals.var_pbswgs_t_dn4 = (0.5 * ((-(p.p1098 * locals.var_deltemp_dn4)) + ((((-(p.p1098 * locals.var_deltemp_dn4)) * assign14230_e19884) + (assign14230_e19877 * (-(p.p1098 * locals.var_deltemp_dn4)))) / (2.0 * assign14230_e19892))));
        locals.var_pbswgs_t_dn5 = (0.5 * ((-(p.p1098 * locals.var_deltemp_dn5)) + ((((-(p.p1098 * locals.var_deltemp_dn5)) * assign14230_e19884) + (assign14230_e19877 * (-(p.p1098 * locals.var_deltemp_dn5)))) / (2.0 * assign14230_e19892))));
        locals.var_pbswgs_t_rv = 0.0;

        let assign14240_e19901: f64 = (p.p1098 * locals.var_deltemp);
        let assign14240_e19902: f64 = (p.p912 - assign14240_e19901);
        let assign14240_e19904: f64 = (assign14240_e19902 - 0.01);
        let assign14240_e19908: f64 = (p.p1098 * locals.var_deltemp);
        let assign14240_e19909: f64 = (p.p912 - assign14240_e19908);
        let assign14240_e19911: f64 = (assign14240_e19909 - 0.01);
        let assign14240_e19915: f64 = (p.p1098 * locals.var_deltemp);
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
        locals.var_pbswgd_t = assign14240_e19930;
        locals.var_pbswgd_t_dn4 = (0.5 * ((-(p.p1098 * locals.var_deltemp_dn4)) + ((((-(p.p1098 * locals.var_deltemp_dn4)) * assign14240_e19918) + (assign14240_e19911 * (-(p.p1098 * locals.var_deltemp_dn4)))) / (2.0 * assign14240_e19926))));
        locals.var_pbswgd_t_dn5 = (0.5 * ((-(p.p1098 * locals.var_deltemp_dn5)) + ((((-(p.p1098 * locals.var_deltemp_dn5)) * assign14240_e19918) + (assign14240_e19911 * (-(p.p1098 * locals.var_deltemp_dn5)))) / (2.0 * assign14240_e19926))));
        locals.var_pbswgd_t_rv = 0.0;

        let assign14250_e19933: f64 = if p.p8 < 9.0 { 1.0 } else { 0.0 };
        locals.var_guard456 = assign14250_e19933;
        locals.var_guard456_rv = 0.0;

        let assign14260_e19936: f64 = (p.p2 % 2.0);
        let assign14260_e19938: f64 = if assign14260_e19936 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard457 = assign14260_e19938;
        locals.var_guard457_rv = 0.0;

        let (assign14270_e19944,) = {
    if ((locals.var_guard456 != 0.0) && (locals.var_guard457 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_nuendd,)
    }
};
        locals.var_nuendd = assign14270_e19944;
        locals.var_nuendd_rv = 0.0;

        let (assign14280_e19950,) = {
    if ((locals.var_guard456 != 0.0) && (locals.var_guard457 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_nuends,)
    }
};
        locals.var_nuends = assign14280_e19950;
        locals.var_nuends_rv = 0.0;

        let (assign14290_e19964,) = {
    if ((locals.var_guard456 != 0.0) && (locals.var_guard457 != 0.0)) {
        let assign14290_e19957: f64 = (p.p2 - 1.0);
        let assign14290_e19959: f64 = (assign14290_e19957 / 2.0);
        let assign14290_e19961: f64 = (assign14290_e19959).max(0.0);
        let assign14290_e19962: f64 = (2.0 * assign14290_e19961);
        (assign14290_e19962,)
    } else {
        (locals.var_nuintd,)
    }
};
        locals.var_nuintd = assign14290_e19964;
        locals.var_nuintd_rv = 0.0;

        let (assign14300_e19970,) = {
    if ((locals.var_guard456 != 0.0) && (locals.var_guard457 != 0.0)) {
        (locals.var_nuintd,)
    } else {
        (locals.var_nuints,)
    }
};
        locals.var_nuints = assign14300_e19970;
        locals.var_nuints_rv = 0.0;

        let assign14310_e19973: f64 = if p.p6 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard458 = assign14310_e19973;
        locals.var_guard458_rv = 0.0;

        let (assign14320_e19982,) = {
    if (((locals.var_guard456 != 0.0) && (locals.var_guard457 == 0.0)) && (locals.var_guard458 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_nuendd,)
    }
};
        locals.var_nuendd = assign14320_e19982;
        locals.var_nuendd_rv = 0.0;

        let (assign14330_e19999,) = {
    if (((locals.var_guard456 != 0.0) && (locals.var_guard457 == 0.0)) && (locals.var_guard458 != 0.0)) {
        let assign14330_e19992: f64 = (p.p2 / 2.0);
        let assign14330_e19994: f64 = (assign14330_e19992 - 1.0);
        let assign14330_e19996: f64 = (assign14330_e19994).max(0.0);
        let assign14330_e19997: f64 = (2.0 * assign14330_e19996);
        (assign14330_e19997,)
    } else {
        (locals.var_nuintd,)
    }
};
        locals.var_nuintd = assign14330_e19999;
        locals.var_nuintd_rv = 0.0;

        let (assign14340_e20008,) = {
    if (((locals.var_guard456 != 0.0) && (locals.var_guard457 == 0.0)) && (locals.var_guard458 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_nuends,)
    }
};
        locals.var_nuends = assign14340_e20008;
        locals.var_nuends_rv = 0.0;

        let (assign14350_e20017,) = {
    if (((locals.var_guard456 != 0.0) && (locals.var_guard457 == 0.0)) && (locals.var_guard458 != 0.0)) {
        (p.p2,)
    } else {
        (locals.var_nuints,)
    }
};
        locals.var_nuints = assign14350_e20017;
        locals.var_nuints_rv = 0.0;

        let (assign14360_e20027,) = {
    if (((locals.var_guard456 != 0.0) && (locals.var_guard457 == 0.0)) && (locals.var_guard458 == 0.0)) {
        (0.0,)
    } else {
        (locals.var_nuendd,)
    }
};
        locals.var_nuendd = assign14360_e20027;
        locals.var_nuendd_rv = 0.0;

        let (assign14370_e20037,) = {
    if (((locals.var_guard456 != 0.0) && (locals.var_guard457 == 0.0)) && (locals.var_guard458 == 0.0)) {
        (p.p2,)
    } else {
        (locals.var_nuintd,)
    }
};
        locals.var_nuintd = assign14370_e20037;
        locals.var_nuintd_rv = 0.0;

        let (assign14380_e20047,) = {
    if (((locals.var_guard456 != 0.0) && (locals.var_guard457 == 0.0)) && (locals.var_guard458 == 0.0)) {
        (2.0,)
    } else {
        (locals.var_nuends,)
    }
};
        locals.var_nuends = assign14380_e20047;
        locals.var_nuends_rv = 0.0;

        let (assign14390_e20065,) = {
    if (((locals.var_guard456 != 0.0) && (locals.var_guard457 == 0.0)) && (locals.var_guard458 == 0.0)) {
        let assign14390_e20058: f64 = (p.p2 / 2.0);
        let assign14390_e20060: f64 = (assign14390_e20058 - 1.0);
        let assign14390_e20062: f64 = (assign14390_e20060).max(0.0);
        let assign14390_e20063: f64 = (2.0 * assign14390_e20062);
        (assign14390_e20063,)
    } else {
        (locals.var_nuints,)
    }
};
        locals.var_nuints = assign14390_e20065;
        locals.var_nuints_rv = 0.0;

        let assign14400_e20068: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        locals.var_t0 = assign14400_e20068;
        locals.var_t0_dn3 = 0.0;
        locals.var_t0_dn4 = 0.0;
        locals.var_t0_dn5 = 0.0;
        locals.var_t0_dn6 = 0.0;
        locals.var_t0_dn7 = 0.0;
        locals.var_t0_dn8 = 0.0;
        locals.var_t0_dn9 = 0.0;
        locals.var_t0_dn10 = 0.0;
        locals.var_t0_dn11 = 0.0;
        locals.var_t0_rv = 0.0;

        let assign14410_e20071: f64 = (locals.var_dmcgeff + locals.var_dmcgeff);
        locals.var_t1 = assign14410_e20071;
        locals.var_t1_dn3 = 0.0;
        locals.var_t1_dn4 = 0.0;
        locals.var_t1_dn5 = 0.0;
        locals.var_t1_dn6 = 0.0;
        locals.var_t1_dn7 = 0.0;
        locals.var_t1_dn8 = 0.0;
        locals.var_t1_dn9 = 0.0;
        locals.var_t1_dn10 = 0.0;
        locals.var_t1_dn11 = 0.0;
        locals.var_t1_rv = 0.0;

        let assign14420_e20074: f64 = (locals.var_dmdgeff + locals.var_dmdgeff);
        locals.var_t2 = assign14420_e20074;
        locals.var_t2_dn3 = 0.0;
        locals.var_t2_dn4 = 0.0;
        locals.var_t2_dn5 = 0.0;
        locals.var_t2_dn6 = 0.0;
        locals.var_t2_dn7 = 0.0;
        locals.var_t2_dn8 = 0.0;
        locals.var_t2_dn9 = 0.0;
        locals.var_t2_dn10 = 0.0;
        locals.var_t2_dn11 = 0.0;
        locals.var_t2_rv = 0.0;

        let assign14430_e20077: f64 = (locals.var_t0 + locals.var_t0);
        let assign14430_e20079: f64 = (assign14430_e20077 + locals.var_weffcj);
        locals.var_psiso = assign14430_e20079;
        locals.var_psiso_dn3 = (locals.var_t0_dn3 + locals.var_t0_dn3);
        locals.var_psiso_dn4 = (locals.var_t0_dn4 + locals.var_t0_dn4);
        locals.var_psiso_dn5 = (locals.var_t0_dn5 + locals.var_t0_dn5);
        locals.var_psiso_dn6 = (locals.var_t0_dn6 + locals.var_t0_dn6);
        locals.var_psiso_dn7 = (locals.var_t0_dn7 + locals.var_t0_dn7);
        locals.var_psiso_dn8 = (locals.var_t0_dn8 + locals.var_t0_dn8);
        locals.var_psiso_dn9 = (locals.var_t0_dn9 + locals.var_t0_dn9);
        locals.var_psiso_dn10 = (locals.var_t0_dn10 + locals.var_t0_dn10);
        locals.var_psiso_dn11 = (locals.var_t0_dn11 + locals.var_t0_dn11);
        locals.var_psiso_rv = 0.0;

        let assign14440_e20082: f64 = (locals.var_t0 + locals.var_t0);
        let assign14440_e20084: f64 = (assign14440_e20082 + locals.var_weffcj);
        locals.var_pdiso = assign14440_e20084;
        locals.var_pdiso_dn3 = (locals.var_t0_dn3 + locals.var_t0_dn3);
        locals.var_pdiso_dn4 = (locals.var_t0_dn4 + locals.var_t0_dn4);
        locals.var_pdiso_dn5 = (locals.var_t0_dn5 + locals.var_t0_dn5);
        locals.var_pdiso_dn6 = (locals.var_t0_dn6 + locals.var_t0_dn6);
        locals.var_pdiso_dn7 = (locals.var_t0_dn7 + locals.var_t0_dn7);
        locals.var_pdiso_dn8 = (locals.var_t0_dn8 + locals.var_t0_dn8);
        locals.var_pdiso_dn9 = (locals.var_t0_dn9 + locals.var_t0_dn9);
        locals.var_pdiso_dn10 = (locals.var_t0_dn10 + locals.var_t0_dn10);
        locals.var_pdiso_dn11 = (locals.var_t0_dn11 + locals.var_t0_dn11);
        locals.var_pdiso_rv = 0.0;

        locals.var_pssha = locals.var_t1;
        locals.var_pssha_dn3 = locals.var_t1_dn3;
        locals.var_pssha_dn4 = locals.var_t1_dn4;
        locals.var_pssha_dn5 = locals.var_t1_dn5;
        locals.var_pssha_dn6 = locals.var_t1_dn6;
        locals.var_pssha_dn7 = locals.var_t1_dn7;
        locals.var_pssha_dn8 = locals.var_t1_dn8;
        locals.var_pssha_dn9 = locals.var_t1_dn9;
        locals.var_pssha_dn10 = locals.var_t1_dn10;
        locals.var_pssha_dn11 = locals.var_t1_dn11;
        locals.var_pssha_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_25(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        locals.var_pdsha = locals.var_t1;
        locals.var_pdsha_dn3 = locals.var_t1_dn3;
        locals.var_pdsha_dn4 = locals.var_t1_dn4;
        locals.var_pdsha_dn5 = locals.var_t1_dn5;
        locals.var_pdsha_dn6 = locals.var_t1_dn6;
        locals.var_pdsha_dn7 = locals.var_t1_dn7;
        locals.var_pdsha_dn8 = locals.var_t1_dn8;
        locals.var_pdsha_dn9 = locals.var_t1_dn9;
        locals.var_pdsha_dn10 = locals.var_t1_dn10;
        locals.var_pdsha_dn11 = locals.var_t1_dn11;
        locals.var_pdsha_rv = 0.0;

        locals.var_psmer = locals.var_t2;
        locals.var_psmer_dn3 = locals.var_t2_dn3;
        locals.var_psmer_dn4 = locals.var_t2_dn4;
        locals.var_psmer_dn5 = locals.var_t2_dn5;
        locals.var_psmer_dn6 = locals.var_t2_dn6;
        locals.var_psmer_dn7 = locals.var_t2_dn7;
        locals.var_psmer_dn8 = locals.var_t2_dn8;
        locals.var_psmer_dn9 = locals.var_t2_dn9;
        locals.var_psmer_dn10 = locals.var_t2_dn10;
        locals.var_psmer_dn11 = locals.var_t2_dn11;
        locals.var_psmer_rv = 0.0;

        locals.var_pdmer = locals.var_t2;
        locals.var_pdmer_dn3 = locals.var_t2_dn3;
        locals.var_pdmer_dn4 = locals.var_t2_dn4;
        locals.var_pdmer_dn5 = locals.var_t2_dn5;
        locals.var_pdmer_dn6 = locals.var_t2_dn6;
        locals.var_pdmer_dn7 = locals.var_t2_dn7;
        locals.var_pdmer_dn8 = locals.var_t2_dn8;
        locals.var_pdmer_dn9 = locals.var_t2_dn9;
        locals.var_pdmer_dn10 = locals.var_t2_dn10;
        locals.var_pdmer_dn11 = locals.var_t2_dn11;
        locals.var_pdmer_rv = 0.0;

        let assign14490_e20091: f64 = (locals.var_t0 * locals.var_weffcj);
        locals.var_asiso = assign14490_e20091;
        locals.var_asiso_dn3 = (locals.var_t0_dn3 * locals.var_weffcj);
        locals.var_asiso_dn4 = (locals.var_t0_dn4 * locals.var_weffcj);
        locals.var_asiso_dn5 = (locals.var_t0_dn5 * locals.var_weffcj);
        locals.var_asiso_dn6 = (locals.var_t0_dn6 * locals.var_weffcj);
        locals.var_asiso_dn7 = (locals.var_t0_dn7 * locals.var_weffcj);
        locals.var_asiso_dn8 = (locals.var_t0_dn8 * locals.var_weffcj);
        locals.var_asiso_dn9 = (locals.var_t0_dn9 * locals.var_weffcj);
        locals.var_asiso_dn10 = (locals.var_t0_dn10 * locals.var_weffcj);
        locals.var_asiso_dn11 = (locals.var_t0_dn11 * locals.var_weffcj);
        locals.var_asiso_rv = 0.0;

        let assign14500_e20094: f64 = (locals.var_t0 * locals.var_weffcj);
        locals.var_adiso = assign14500_e20094;
        locals.var_adiso_dn3 = (locals.var_t0_dn3 * locals.var_weffcj);
        locals.var_adiso_dn4 = (locals.var_t0_dn4 * locals.var_weffcj);
        locals.var_adiso_dn5 = (locals.var_t0_dn5 * locals.var_weffcj);
        locals.var_adiso_dn6 = (locals.var_t0_dn6 * locals.var_weffcj);
        locals.var_adiso_dn7 = (locals.var_t0_dn7 * locals.var_weffcj);
        locals.var_adiso_dn8 = (locals.var_t0_dn8 * locals.var_weffcj);
        locals.var_adiso_dn9 = (locals.var_t0_dn9 * locals.var_weffcj);
        locals.var_adiso_dn10 = (locals.var_t0_dn10 * locals.var_weffcj);
        locals.var_adiso_dn11 = (locals.var_t0_dn11 * locals.var_weffcj);
        locals.var_adiso_rv = 0.0;

        let assign14510_e20097: f64 = (locals.var_dmcgeff * locals.var_weffcj);
        locals.var_assha = assign14510_e20097;
        locals.var_assha_rv = 0.0;

        let assign14520_e20100: f64 = (locals.var_dmcgeff * locals.var_weffcj);
        locals.var_adsha = assign14520_e20100;
        locals.var_adsha_rv = 0.0;

        let assign14530_e20103: f64 = (locals.var_dmdgeff * locals.var_weffcj);
        locals.var_asmer = assign14530_e20103;
        locals.var_asmer_rv = 0.0;

        let assign14540_e20106: f64 = (locals.var_dmdgeff * locals.var_weffcj);
        locals.var_admer = assign14540_e20106;
        locals.var_admer_rv = 0.0;

        let assign14550_e20109: f64 = if p.p8 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard459 = assign14550_e20109;
        locals.var_guard459_rv = 0.0;

        let assign14560_e20112: f64 = if p.p8 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard460 = assign14560_e20112;
        locals.var_guard460_rv = 0.0;

        let assign14570_e20115: f64 = if p.p8 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard461 = assign14570_e20115;
        locals.var_guard461_rv = 0.0;

        let assign14580_e20118: f64 = if p.p8 == 3.0 { 1.0 } else { 0.0 };
        locals.var_guard462 = assign14580_e20118;
        locals.var_guard462_rv = 0.0;

        let assign14590_e20121: f64 = if p.p8 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard463 = assign14590_e20121;
        locals.var_guard463_rv = 0.0;

        let assign14600_e20124: f64 = if p.p8 == 5.0 { 1.0 } else { 0.0 };
        locals.var_guard464 = assign14600_e20124;
        locals.var_guard464_rv = 0.0;

        let assign14610_e20127: f64 = if p.p8 == 6.0 { 1.0 } else { 0.0 };
        locals.var_guard465 = assign14610_e20127;
        locals.var_guard465_rv = 0.0;

        let assign14620_e20130: f64 = if p.p8 == 7.0 { 1.0 } else { 0.0 };
        locals.var_guard466 = assign14620_e20130;
        locals.var_guard466_rv = 0.0;

        let assign14630_e20133: f64 = if p.p8 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard467 = assign14630_e20133;
        locals.var_guard467_rv = 0.0;

        let assign14640_e20136: f64 = if p.p8 == 9.0 { 1.0 } else { 0.0 };
        locals.var_guard468 = assign14640_e20136;
        locals.var_guard468_rv = 0.0;

        let assign14650_e20139: f64 = if p.p8 == 10.0 { 1.0 } else { 0.0 };
        locals.var_guard469 = assign14650_e20139;
        locals.var_guard469_rv = 0.0;

        let (assign14660_e20149, assign14660_e20149_d_n3, assign14660_e20149_d_n4, assign14660_e20149_d_n5, assign14660_e20149_d_n6, assign14660_e20149_d_n7, assign14660_e20149_d_n8, assign14660_e20149_d_n9, assign14660_e20149_d_n10, assign14660_e20149_d_n11,) = {
    if (locals.var_guard459 != 0.0) {
        let assign14660_e20143: f64 = (locals.var_nuends * locals.var_psiso);
        let assign14660_e20146: f64 = (locals.var_nuints * locals.var_pssha);
        let assign14660_e20147: f64 = (assign14660_e20143 + assign14660_e20146);
        (assign14660_e20147, ((locals.var_nuends * locals.var_psiso_dn3) + (locals.var_nuints * locals.var_pssha_dn3)), ((locals.var_nuends * locals.var_psiso_dn4) + (locals.var_nuints * locals.var_pssha_dn4)), ((locals.var_nuends * locals.var_psiso_dn5) + (locals.var_nuints * locals.var_pssha_dn5)), ((locals.var_nuends * locals.var_psiso_dn6) + (locals.var_nuints * locals.var_pssha_dn6)), ((locals.var_nuends * locals.var_psiso_dn7) + (locals.var_nuints * locals.var_pssha_dn7)), ((locals.var_nuends * locals.var_psiso_dn8) + (locals.var_nuints * locals.var_pssha_dn8)), ((locals.var_nuends * locals.var_psiso_dn9) + (locals.var_nuints * locals.var_pssha_dn9)), ((locals.var_nuends * locals.var_psiso_dn10) + (locals.var_nuints * locals.var_pssha_dn10)), ((locals.var_nuends * locals.var_psiso_dn11) + (locals.var_nuints * locals.var_pssha_dn11)),)
    } else {
        (locals.var_temp_pseff, locals.var_temp_pseff_dn3, locals.var_temp_pseff_dn4, locals.var_temp_pseff_dn5, locals.var_temp_pseff_dn6, locals.var_temp_pseff_dn7, locals.var_temp_pseff_dn8, locals.var_temp_pseff_dn9, locals.var_temp_pseff_dn10, locals.var_temp_pseff_dn11,)
    }
};
        locals.var_temp_pseff = assign14660_e20149;
        locals.var_temp_pseff_dn3 = assign14660_e20149_d_n3;
        locals.var_temp_pseff_dn4 = assign14660_e20149_d_n4;
        locals.var_temp_pseff_dn5 = assign14660_e20149_d_n5;
        locals.var_temp_pseff_dn6 = assign14660_e20149_d_n6;
        locals.var_temp_pseff_dn7 = assign14660_e20149_d_n7;
        locals.var_temp_pseff_dn8 = assign14660_e20149_d_n8;
        locals.var_temp_pseff_dn9 = assign14660_e20149_d_n9;
        locals.var_temp_pseff_dn10 = assign14660_e20149_d_n10;
        locals.var_temp_pseff_dn11 = assign14660_e20149_d_n11;
        locals.var_temp_pseff_rv = 0.0;

        let (assign14670_e20159, assign14670_e20159_d_n3, assign14670_e20159_d_n4, assign14670_e20159_d_n5, assign14670_e20159_d_n6, assign14670_e20159_d_n7, assign14670_e20159_d_n8, assign14670_e20159_d_n9, assign14670_e20159_d_n10, assign14670_e20159_d_n11,) = {
    if (locals.var_guard459 != 0.0) {
        let assign14670_e20153: f64 = (locals.var_nuendd * locals.var_pdiso);
        let assign14670_e20156: f64 = (locals.var_nuintd * locals.var_pdsha);
        let assign14670_e20157: f64 = (assign14670_e20153 + assign14670_e20156);
        (assign14670_e20157, ((locals.var_nuendd * locals.var_pdiso_dn3) + (locals.var_nuintd * locals.var_pdsha_dn3)), ((locals.var_nuendd * locals.var_pdiso_dn4) + (locals.var_nuintd * locals.var_pdsha_dn4)), ((locals.var_nuendd * locals.var_pdiso_dn5) + (locals.var_nuintd * locals.var_pdsha_dn5)), ((locals.var_nuendd * locals.var_pdiso_dn6) + (locals.var_nuintd * locals.var_pdsha_dn6)), ((locals.var_nuendd * locals.var_pdiso_dn7) + (locals.var_nuintd * locals.var_pdsha_dn7)), ((locals.var_nuendd * locals.var_pdiso_dn8) + (locals.var_nuintd * locals.var_pdsha_dn8)), ((locals.var_nuendd * locals.var_pdiso_dn9) + (locals.var_nuintd * locals.var_pdsha_dn9)), ((locals.var_nuendd * locals.var_pdiso_dn10) + (locals.var_nuintd * locals.var_pdsha_dn10)), ((locals.var_nuendd * locals.var_pdiso_dn11) + (locals.var_nuintd * locals.var_pdsha_dn11)),)
    } else {
        (locals.var_temp_pdeff, locals.var_temp_pdeff_dn3, locals.var_temp_pdeff_dn4, locals.var_temp_pdeff_dn5, locals.var_temp_pdeff_dn6, locals.var_temp_pdeff_dn7, locals.var_temp_pdeff_dn8, locals.var_temp_pdeff_dn9, locals.var_temp_pdeff_dn10, locals.var_temp_pdeff_dn11,)
    }
};
        locals.var_temp_pdeff = assign14670_e20159;
        locals.var_temp_pdeff_dn3 = assign14670_e20159_d_n3;
        locals.var_temp_pdeff_dn4 = assign14670_e20159_d_n4;
        locals.var_temp_pdeff_dn5 = assign14670_e20159_d_n5;
        locals.var_temp_pdeff_dn6 = assign14670_e20159_d_n6;
        locals.var_temp_pdeff_dn7 = assign14670_e20159_d_n7;
        locals.var_temp_pdeff_dn8 = assign14670_e20159_d_n8;
        locals.var_temp_pdeff_dn9 = assign14670_e20159_d_n9;
        locals.var_temp_pdeff_dn10 = assign14670_e20159_d_n10;
        locals.var_temp_pdeff_dn11 = assign14670_e20159_d_n11;
        locals.var_temp_pdeff_rv = 0.0;

        let (assign14680_e20169, assign14680_e20169_d_n3, assign14680_e20169_d_n4, assign14680_e20169_d_n5, assign14680_e20169_d_n6, assign14680_e20169_d_n7, assign14680_e20169_d_n8, assign14680_e20169_d_n9, assign14680_e20169_d_n10, assign14680_e20169_d_n11,) = {
    if (locals.var_guard459 != 0.0) {
        let assign14680_e20163: f64 = (locals.var_nuends * locals.var_asiso);
        let assign14680_e20166: f64 = (locals.var_nuints * locals.var_assha);
        let assign14680_e20167: f64 = (assign14680_e20163 + assign14680_e20166);
        (assign14680_e20167, (locals.var_nuends * locals.var_asiso_dn3), (locals.var_nuends * locals.var_asiso_dn4), (locals.var_nuends * locals.var_asiso_dn5), (locals.var_nuends * locals.var_asiso_dn6), (locals.var_nuends * locals.var_asiso_dn7), (locals.var_nuends * locals.var_asiso_dn8), (locals.var_nuends * locals.var_asiso_dn9), (locals.var_nuends * locals.var_asiso_dn10), (locals.var_nuends * locals.var_asiso_dn11),)
    } else {
        (locals.var_temp_aseff, locals.var_temp_aseff_dn3, locals.var_temp_aseff_dn4, locals.var_temp_aseff_dn5, locals.var_temp_aseff_dn6, locals.var_temp_aseff_dn7, locals.var_temp_aseff_dn8, locals.var_temp_aseff_dn9, locals.var_temp_aseff_dn10, locals.var_temp_aseff_dn11,)
    }
};
        locals.var_temp_aseff = assign14680_e20169;
        locals.var_temp_aseff_dn3 = assign14680_e20169_d_n3;
        locals.var_temp_aseff_dn4 = assign14680_e20169_d_n4;
        locals.var_temp_aseff_dn5 = assign14680_e20169_d_n5;
        locals.var_temp_aseff_dn6 = assign14680_e20169_d_n6;
        locals.var_temp_aseff_dn7 = assign14680_e20169_d_n7;
        locals.var_temp_aseff_dn8 = assign14680_e20169_d_n8;
        locals.var_temp_aseff_dn9 = assign14680_e20169_d_n9;
        locals.var_temp_aseff_dn10 = assign14680_e20169_d_n10;
        locals.var_temp_aseff_dn11 = assign14680_e20169_d_n11;
        locals.var_temp_aseff_rv = 0.0;

        let (assign14690_e20179, assign14690_e20179_d_n3, assign14690_e20179_d_n4, assign14690_e20179_d_n5, assign14690_e20179_d_n6, assign14690_e20179_d_n7, assign14690_e20179_d_n8, assign14690_e20179_d_n9, assign14690_e20179_d_n10, assign14690_e20179_d_n11,) = {
    if (locals.var_guard459 != 0.0) {
        let assign14690_e20173: f64 = (locals.var_nuendd * locals.var_adiso);
        let assign14690_e20176: f64 = (locals.var_nuintd * locals.var_adsha);
        let assign14690_e20177: f64 = (assign14690_e20173 + assign14690_e20176);
        (assign14690_e20177, (locals.var_nuendd * locals.var_adiso_dn3), (locals.var_nuendd * locals.var_adiso_dn4), (locals.var_nuendd * locals.var_adiso_dn5), (locals.var_nuendd * locals.var_adiso_dn6), (locals.var_nuendd * locals.var_adiso_dn7), (locals.var_nuendd * locals.var_adiso_dn8), (locals.var_nuendd * locals.var_adiso_dn9), (locals.var_nuendd * locals.var_adiso_dn10), (locals.var_nuendd * locals.var_adiso_dn11),)
    } else {
        (locals.var_temp_adeff, locals.var_temp_adeff_dn3, locals.var_temp_adeff_dn4, locals.var_temp_adeff_dn5, locals.var_temp_adeff_dn6, locals.var_temp_adeff_dn7, locals.var_temp_adeff_dn8, locals.var_temp_adeff_dn9, locals.var_temp_adeff_dn10, locals.var_temp_adeff_dn11,)
    }
};
        locals.var_temp_adeff = assign14690_e20179;
        locals.var_temp_adeff_dn3 = assign14690_e20179_d_n3;
        locals.var_temp_adeff_dn4 = assign14690_e20179_d_n4;
        locals.var_temp_adeff_dn5 = assign14690_e20179_d_n5;
        locals.var_temp_adeff_dn6 = assign14690_e20179_d_n6;
        locals.var_temp_adeff_dn7 = assign14690_e20179_d_n7;
        locals.var_temp_adeff_dn8 = assign14690_e20179_d_n8;
        locals.var_temp_adeff_dn9 = assign14690_e20179_d_n9;
        locals.var_temp_adeff_dn10 = assign14690_e20179_d_n10;
        locals.var_temp_adeff_dn11 = assign14690_e20179_d_n11;
        locals.var_temp_adeff_rv = 0.0;

        let (assign14700_e20192, assign14700_e20192_d_n3, assign14700_e20192_d_n4, assign14700_e20192_d_n5, assign14700_e20192_d_n6, assign14700_e20192_d_n7, assign14700_e20192_d_n8, assign14700_e20192_d_n9, assign14700_e20192_d_n10, assign14700_e20192_d_n11,) = {
    if ((locals.var_guard460 != 0.0) && (locals.var_guard459 == 0.0)) {
        let assign14700_e20186: f64 = (locals.var_nuends * locals.var_psiso);
        let assign14700_e20189: f64 = (locals.var_nuints * locals.var_pssha);
        let assign14700_e20190: f64 = (assign14700_e20186 + assign14700_e20189);
        (assign14700_e20190, ((locals.var_nuends * locals.var_psiso_dn3) + (locals.var_nuints * locals.var_pssha_dn3)), ((locals.var_nuends * locals.var_psiso_dn4) + (locals.var_nuints * locals.var_pssha_dn4)), ((locals.var_nuends * locals.var_psiso_dn5) + (locals.var_nuints * locals.var_pssha_dn5)), ((locals.var_nuends * locals.var_psiso_dn6) + (locals.var_nuints * locals.var_pssha_dn6)), ((locals.var_nuends * locals.var_psiso_dn7) + (locals.var_nuints * locals.var_pssha_dn7)), ((locals.var_nuends * locals.var_psiso_dn8) + (locals.var_nuints * locals.var_pssha_dn8)), ((locals.var_nuends * locals.var_psiso_dn9) + (locals.var_nuints * locals.var_pssha_dn9)), ((locals.var_nuends * locals.var_psiso_dn10) + (locals.var_nuints * locals.var_pssha_dn10)), ((locals.var_nuends * locals.var_psiso_dn11) + (locals.var_nuints * locals.var_pssha_dn11)),)
    } else {
        (locals.var_temp_pseff, locals.var_temp_pseff_dn3, locals.var_temp_pseff_dn4, locals.var_temp_pseff_dn5, locals.var_temp_pseff_dn6, locals.var_temp_pseff_dn7, locals.var_temp_pseff_dn8, locals.var_temp_pseff_dn9, locals.var_temp_pseff_dn10, locals.var_temp_pseff_dn11,)
    }
};
        locals.var_temp_pseff = assign14700_e20192;
        locals.var_temp_pseff_dn3 = assign14700_e20192_d_n3;
        locals.var_temp_pseff_dn4 = assign14700_e20192_d_n4;
        locals.var_temp_pseff_dn5 = assign14700_e20192_d_n5;
        locals.var_temp_pseff_dn6 = assign14700_e20192_d_n6;
        locals.var_temp_pseff_dn7 = assign14700_e20192_d_n7;
        locals.var_temp_pseff_dn8 = assign14700_e20192_d_n8;
        locals.var_temp_pseff_dn9 = assign14700_e20192_d_n9;
        locals.var_temp_pseff_dn10 = assign14700_e20192_d_n10;
        locals.var_temp_pseff_dn11 = assign14700_e20192_d_n11;
        locals.var_temp_pseff_rv = 0.0;

        let (assign14710_e20203, assign14710_e20203_d_n3, assign14710_e20203_d_n4, assign14710_e20203_d_n5, assign14710_e20203_d_n6, assign14710_e20203_d_n7, assign14710_e20203_d_n8, assign14710_e20203_d_n9, assign14710_e20203_d_n10, assign14710_e20203_d_n11,) = {
    if ((locals.var_guard460 != 0.0) && (locals.var_guard459 == 0.0)) {
        let assign14710_e20199: f64 = (locals.var_nuendd + locals.var_nuintd);
        let assign14710_e20201: f64 = (assign14710_e20199 * locals.var_pdsha);
        (assign14710_e20201, (assign14710_e20199 * locals.var_pdsha_dn3), (assign14710_e20199 * locals.var_pdsha_dn4), (assign14710_e20199 * locals.var_pdsha_dn5), (assign14710_e20199 * locals.var_pdsha_dn6), (assign14710_e20199 * locals.var_pdsha_dn7), (assign14710_e20199 * locals.var_pdsha_dn8), (assign14710_e20199 * locals.var_pdsha_dn9), (assign14710_e20199 * locals.var_pdsha_dn10), (assign14710_e20199 * locals.var_pdsha_dn11),)
    } else {
        (locals.var_temp_pdeff, locals.var_temp_pdeff_dn3, locals.var_temp_pdeff_dn4, locals.var_temp_pdeff_dn5, locals.var_temp_pdeff_dn6, locals.var_temp_pdeff_dn7, locals.var_temp_pdeff_dn8, locals.var_temp_pdeff_dn9, locals.var_temp_pdeff_dn10, locals.var_temp_pdeff_dn11,)
    }
};
        locals.var_temp_pdeff = assign14710_e20203;
        locals.var_temp_pdeff_dn3 = assign14710_e20203_d_n3;
        locals.var_temp_pdeff_dn4 = assign14710_e20203_d_n4;
        locals.var_temp_pdeff_dn5 = assign14710_e20203_d_n5;
        locals.var_temp_pdeff_dn6 = assign14710_e20203_d_n6;
        locals.var_temp_pdeff_dn7 = assign14710_e20203_d_n7;
        locals.var_temp_pdeff_dn8 = assign14710_e20203_d_n8;
        locals.var_temp_pdeff_dn9 = assign14710_e20203_d_n9;
        locals.var_temp_pdeff_dn10 = assign14710_e20203_d_n10;
        locals.var_temp_pdeff_dn11 = assign14710_e20203_d_n11;
        locals.var_temp_pdeff_rv = 0.0;

        let (assign14720_e20216, assign14720_e20216_d_n3, assign14720_e20216_d_n4, assign14720_e20216_d_n5, assign14720_e20216_d_n6, assign14720_e20216_d_n7, assign14720_e20216_d_n8, assign14720_e20216_d_n9, assign14720_e20216_d_n10, assign14720_e20216_d_n11,) = {
    if ((locals.var_guard460 != 0.0) && (locals.var_guard459 == 0.0)) {
        let assign14720_e20210: f64 = (locals.var_nuends * locals.var_asiso);
        let assign14720_e20213: f64 = (locals.var_nuints * locals.var_assha);
        let assign14720_e20214: f64 = (assign14720_e20210 + assign14720_e20213);
        (assign14720_e20214, (locals.var_nuends * locals.var_asiso_dn3), (locals.var_nuends * locals.var_asiso_dn4), (locals.var_nuends * locals.var_asiso_dn5), (locals.var_nuends * locals.var_asiso_dn6), (locals.var_nuends * locals.var_asiso_dn7), (locals.var_nuends * locals.var_asiso_dn8), (locals.var_nuends * locals.var_asiso_dn9), (locals.var_nuends * locals.var_asiso_dn10), (locals.var_nuends * locals.var_asiso_dn11),)
    } else {
        (locals.var_temp_aseff, locals.var_temp_aseff_dn3, locals.var_temp_aseff_dn4, locals.var_temp_aseff_dn5, locals.var_temp_aseff_dn6, locals.var_temp_aseff_dn7, locals.var_temp_aseff_dn8, locals.var_temp_aseff_dn9, locals.var_temp_aseff_dn10, locals.var_temp_aseff_dn11,)
    }
};
        locals.var_temp_aseff = assign14720_e20216;
        locals.var_temp_aseff_dn3 = assign14720_e20216_d_n3;
        locals.var_temp_aseff_dn4 = assign14720_e20216_d_n4;
        locals.var_temp_aseff_dn5 = assign14720_e20216_d_n5;
        locals.var_temp_aseff_dn6 = assign14720_e20216_d_n6;
        locals.var_temp_aseff_dn7 = assign14720_e20216_d_n7;
        locals.var_temp_aseff_dn8 = assign14720_e20216_d_n8;
        locals.var_temp_aseff_dn9 = assign14720_e20216_d_n9;
        locals.var_temp_aseff_dn10 = assign14720_e20216_d_n10;
        locals.var_temp_aseff_dn11 = assign14720_e20216_d_n11;
        locals.var_temp_aseff_rv = 0.0;

        let (assign14730_e20227, assign14730_e20227_d_n3, assign14730_e20227_d_n4, assign14730_e20227_d_n5, assign14730_e20227_d_n6, assign14730_e20227_d_n7, assign14730_e20227_d_n8, assign14730_e20227_d_n9, assign14730_e20227_d_n10, assign14730_e20227_d_n11,) = {
    if ((locals.var_guard460 != 0.0) && (locals.var_guard459 == 0.0)) {
        let assign14730_e20223: f64 = (locals.var_nuendd + locals.var_nuintd);
        let assign14730_e20225: f64 = (assign14730_e20223 * locals.var_adsha);
        (assign14730_e20225, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp_adeff, locals.var_temp_adeff_dn3, locals.var_temp_adeff_dn4, locals.var_temp_adeff_dn5, locals.var_temp_adeff_dn6, locals.var_temp_adeff_dn7, locals.var_temp_adeff_dn8, locals.var_temp_adeff_dn9, locals.var_temp_adeff_dn10, locals.var_temp_adeff_dn11,)
    }
};
        locals.var_temp_adeff = assign14730_e20227;
        locals.var_temp_adeff_dn3 = assign14730_e20227_d_n3;
        locals.var_temp_adeff_dn4 = assign14730_e20227_d_n4;
        locals.var_temp_adeff_dn5 = assign14730_e20227_d_n5;
        locals.var_temp_adeff_dn6 = assign14730_e20227_d_n6;
        locals.var_temp_adeff_dn7 = assign14730_e20227_d_n7;
        locals.var_temp_adeff_dn8 = assign14730_e20227_d_n8;
        locals.var_temp_adeff_dn9 = assign14730_e20227_d_n9;
        locals.var_temp_adeff_dn10 = assign14730_e20227_d_n10;
        locals.var_temp_adeff_dn11 = assign14730_e20227_d_n11;
        locals.var_temp_adeff_rv = 0.0;

        let (assign14740_e20240, assign14740_e20240_d_n3, assign14740_e20240_d_n4, assign14740_e20240_d_n5, assign14740_e20240_d_n6, assign14740_e20240_d_n7, assign14740_e20240_d_n8, assign14740_e20240_d_n9, assign14740_e20240_d_n10, assign14740_e20240_d_n11,) = {
    if ((locals.var_guard461 != 0.0) && (!((locals.var_guard459 != 0.0) || (locals.var_guard460 != 0.0)))) {
        let assign14740_e20236: f64 = (locals.var_nuends + locals.var_nuints);
        let assign14740_e20238: f64 = (assign14740_e20236 * locals.var_pssha);
        (assign14740_e20238, (assign14740_e20236 * locals.var_pssha_dn3), (assign14740_e20236 * locals.var_pssha_dn4), (assign14740_e20236 * locals.var_pssha_dn5), (assign14740_e20236 * locals.var_pssha_dn6), (assign14740_e20236 * locals.var_pssha_dn7), (assign14740_e20236 * locals.var_pssha_dn8), (assign14740_e20236 * locals.var_pssha_dn9), (assign14740_e20236 * locals.var_pssha_dn10), (assign14740_e20236 * locals.var_pssha_dn11),)
    } else {
        (locals.var_temp_pseff, locals.var_temp_pseff_dn3, locals.var_temp_pseff_dn4, locals.var_temp_pseff_dn5, locals.var_temp_pseff_dn6, locals.var_temp_pseff_dn7, locals.var_temp_pseff_dn8, locals.var_temp_pseff_dn9, locals.var_temp_pseff_dn10, locals.var_temp_pseff_dn11,)
    }
};
        locals.var_temp_pseff = assign14740_e20240;
        locals.var_temp_pseff_dn3 = assign14740_e20240_d_n3;
        locals.var_temp_pseff_dn4 = assign14740_e20240_d_n4;
        locals.var_temp_pseff_dn5 = assign14740_e20240_d_n5;
        locals.var_temp_pseff_dn6 = assign14740_e20240_d_n6;
        locals.var_temp_pseff_dn7 = assign14740_e20240_d_n7;
        locals.var_temp_pseff_dn8 = assign14740_e20240_d_n8;
        locals.var_temp_pseff_dn9 = assign14740_e20240_d_n9;
        locals.var_temp_pseff_dn10 = assign14740_e20240_d_n10;
        locals.var_temp_pseff_dn11 = assign14740_e20240_d_n11;
        locals.var_temp_pseff_rv = 0.0;

        let (assign14750_e20255, assign14750_e20255_d_n3, assign14750_e20255_d_n4, assign14750_e20255_d_n5, assign14750_e20255_d_n6, assign14750_e20255_d_n7, assign14750_e20255_d_n8, assign14750_e20255_d_n9, assign14750_e20255_d_n10, assign14750_e20255_d_n11,) = {
    if ((locals.var_guard461 != 0.0) && (!((locals.var_guard459 != 0.0) || (locals.var_guard460 != 0.0)))) {
        let assign14750_e20249: f64 = (locals.var_nuendd * locals.var_pdiso);
        let assign14750_e20252: f64 = (locals.var_nuintd * locals.var_pdsha);
        let assign14750_e20253: f64 = (assign14750_e20249 + assign14750_e20252);
        (assign14750_e20253, ((locals.var_nuendd * locals.var_pdiso_dn3) + (locals.var_nuintd * locals.var_pdsha_dn3)), ((locals.var_nuendd * locals.var_pdiso_dn4) + (locals.var_nuintd * locals.var_pdsha_dn4)), ((locals.var_nuendd * locals.var_pdiso_dn5) + (locals.var_nuintd * locals.var_pdsha_dn5)), ((locals.var_nuendd * locals.var_pdiso_dn6) + (locals.var_nuintd * locals.var_pdsha_dn6)), ((locals.var_nuendd * locals.var_pdiso_dn7) + (locals.var_nuintd * locals.var_pdsha_dn7)), ((locals.var_nuendd * locals.var_pdiso_dn8) + (locals.var_nuintd * locals.var_pdsha_dn8)), ((locals.var_nuendd * locals.var_pdiso_dn9) + (locals.var_nuintd * locals.var_pdsha_dn9)), ((locals.var_nuendd * locals.var_pdiso_dn10) + (locals.var_nuintd * locals.var_pdsha_dn10)), ((locals.var_nuendd * locals.var_pdiso_dn11) + (locals.var_nuintd * locals.var_pdsha_dn11)),)
    } else {
        (locals.var_temp_pdeff, locals.var_temp_pdeff_dn3, locals.var_temp_pdeff_dn4, locals.var_temp_pdeff_dn5, locals.var_temp_pdeff_dn6, locals.var_temp_pdeff_dn7, locals.var_temp_pdeff_dn8, locals.var_temp_pdeff_dn9, locals.var_temp_pdeff_dn10, locals.var_temp_pdeff_dn11,)
    }
};
        locals.var_temp_pdeff = assign14750_e20255;
        locals.var_temp_pdeff_dn3 = assign14750_e20255_d_n3;
        locals.var_temp_pdeff_dn4 = assign14750_e20255_d_n4;
        locals.var_temp_pdeff_dn5 = assign14750_e20255_d_n5;
        locals.var_temp_pdeff_dn6 = assign14750_e20255_d_n6;
        locals.var_temp_pdeff_dn7 = assign14750_e20255_d_n7;
        locals.var_temp_pdeff_dn8 = assign14750_e20255_d_n8;
        locals.var_temp_pdeff_dn9 = assign14750_e20255_d_n9;
        locals.var_temp_pdeff_dn10 = assign14750_e20255_d_n10;
        locals.var_temp_pdeff_dn11 = assign14750_e20255_d_n11;
        locals.var_temp_pdeff_rv = 0.0;

        let (assign14760_e20268, assign14760_e20268_d_n3, assign14760_e20268_d_n4, assign14760_e20268_d_n5, assign14760_e20268_d_n6, assign14760_e20268_d_n7, assign14760_e20268_d_n8, assign14760_e20268_d_n9, assign14760_e20268_d_n10, assign14760_e20268_d_n11,) = {
    if ((locals.var_guard461 != 0.0) && (!((locals.var_guard459 != 0.0) || (locals.var_guard460 != 0.0)))) {
        let assign14760_e20264: f64 = (locals.var_nuends + locals.var_nuints);
        let assign14760_e20266: f64 = (assign14760_e20264 * locals.var_assha);
        (assign14760_e20266, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp_aseff, locals.var_temp_aseff_dn3, locals.var_temp_aseff_dn4, locals.var_temp_aseff_dn5, locals.var_temp_aseff_dn6, locals.var_temp_aseff_dn7, locals.var_temp_aseff_dn8, locals.var_temp_aseff_dn9, locals.var_temp_aseff_dn10, locals.var_temp_aseff_dn11,)
    }
};
        locals.var_temp_aseff = assign14760_e20268;
        locals.var_temp_aseff_dn3 = assign14760_e20268_d_n3;
        locals.var_temp_aseff_dn4 = assign14760_e20268_d_n4;
        locals.var_temp_aseff_dn5 = assign14760_e20268_d_n5;
        locals.var_temp_aseff_dn6 = assign14760_e20268_d_n6;
        locals.var_temp_aseff_dn7 = assign14760_e20268_d_n7;
        locals.var_temp_aseff_dn8 = assign14760_e20268_d_n8;
        locals.var_temp_aseff_dn9 = assign14760_e20268_d_n9;
        locals.var_temp_aseff_dn10 = assign14760_e20268_d_n10;
        locals.var_temp_aseff_dn11 = assign14760_e20268_d_n11;
        locals.var_temp_aseff_rv = 0.0;

        let (assign14770_e20283, assign14770_e20283_d_n3, assign14770_e20283_d_n4, assign14770_e20283_d_n5, assign14770_e20283_d_n6, assign14770_e20283_d_n7, assign14770_e20283_d_n8, assign14770_e20283_d_n9, assign14770_e20283_d_n10, assign14770_e20283_d_n11,) = {
    if ((locals.var_guard461 != 0.0) && (!((locals.var_guard459 != 0.0) || (locals.var_guard460 != 0.0)))) {
        let assign14770_e20277: f64 = (locals.var_nuendd * locals.var_adiso);
        let assign14770_e20280: f64 = (locals.var_nuintd * locals.var_adsha);
        let assign14770_e20281: f64 = (assign14770_e20277 + assign14770_e20280);
        (assign14770_e20281, (locals.var_nuendd * locals.var_adiso_dn3), (locals.var_nuendd * locals.var_adiso_dn4), (locals.var_nuendd * locals.var_adiso_dn5), (locals.var_nuendd * locals.var_adiso_dn6), (locals.var_nuendd * locals.var_adiso_dn7), (locals.var_nuendd * locals.var_adiso_dn8), (locals.var_nuendd * locals.var_adiso_dn9), (locals.var_nuendd * locals.var_adiso_dn10), (locals.var_nuendd * locals.var_adiso_dn11),)
    } else {
        (locals.var_temp_adeff, locals.var_temp_adeff_dn3, locals.var_temp_adeff_dn4, locals.var_temp_adeff_dn5, locals.var_temp_adeff_dn6, locals.var_temp_adeff_dn7, locals.var_temp_adeff_dn8, locals.var_temp_adeff_dn9, locals.var_temp_adeff_dn10, locals.var_temp_adeff_dn11,)
    }
};
        locals.var_temp_adeff = assign14770_e20283;
        locals.var_temp_adeff_dn3 = assign14770_e20283_d_n3;
        locals.var_temp_adeff_dn4 = assign14770_e20283_d_n4;
        locals.var_temp_adeff_dn5 = assign14770_e20283_d_n5;
        locals.var_temp_adeff_dn6 = assign14770_e20283_d_n6;
        locals.var_temp_adeff_dn7 = assign14770_e20283_d_n7;
        locals.var_temp_adeff_dn8 = assign14770_e20283_d_n8;
        locals.var_temp_adeff_dn9 = assign14770_e20283_d_n9;
        locals.var_temp_adeff_dn10 = assign14770_e20283_d_n10;
        locals.var_temp_adeff_dn11 = assign14770_e20283_d_n11;
        locals.var_temp_adeff_rv = 0.0;

        let (assign14780_e20298, assign14780_e20298_d_n3, assign14780_e20298_d_n4, assign14780_e20298_d_n5, assign14780_e20298_d_n6, assign14780_e20298_d_n7, assign14780_e20298_d_n8, assign14780_e20298_d_n9, assign14780_e20298_d_n10, assign14780_e20298_d_n11,) = {
    if ((locals.var_guard462 != 0.0) && (!(((locals.var_guard459 != 0.0) || (locals.var_guard460 != 0.0)) || (locals.var_guard461 != 0.0)))) {
        let assign14780_e20294: f64 = (locals.var_nuends + locals.var_nuints);
        let assign14780_e20296: f64 = (assign14780_e20294 * locals.var_pssha);
        (assign14780_e20296, (assign14780_e20294 * locals.var_pssha_dn3), (assign14780_e20294 * locals.var_pssha_dn4), (assign14780_e20294 * locals.var_pssha_dn5), (assign14780_e20294 * locals.var_pssha_dn6), (assign14780_e20294 * locals.var_pssha_dn7), (assign14780_e20294 * locals.var_pssha_dn8), (assign14780_e20294 * locals.var_pssha_dn9), (assign14780_e20294 * locals.var_pssha_dn10), (assign14780_e20294 * locals.var_pssha_dn11),)
    } else {
        (locals.var_temp_pseff, locals.var_temp_pseff_dn3, locals.var_temp_pseff_dn4, locals.var_temp_pseff_dn5, locals.var_temp_pseff_dn6, locals.var_temp_pseff_dn7, locals.var_temp_pseff_dn8, locals.var_temp_pseff_dn9, locals.var_temp_pseff_dn10, locals.var_temp_pseff_dn11,)
    }
};
        locals.var_temp_pseff = assign14780_e20298;
        locals.var_temp_pseff_dn3 = assign14780_e20298_d_n3;
        locals.var_temp_pseff_dn4 = assign14780_e20298_d_n4;
        locals.var_temp_pseff_dn5 = assign14780_e20298_d_n5;
        locals.var_temp_pseff_dn6 = assign14780_e20298_d_n6;
        locals.var_temp_pseff_dn7 = assign14780_e20298_d_n7;
        locals.var_temp_pseff_dn8 = assign14780_e20298_d_n8;
        locals.var_temp_pseff_dn9 = assign14780_e20298_d_n9;
        locals.var_temp_pseff_dn10 = assign14780_e20298_d_n10;
        locals.var_temp_pseff_dn11 = assign14780_e20298_d_n11;
        locals.var_temp_pseff_rv = 0.0;

        let (assign14790_e20313, assign14790_e20313_d_n3, assign14790_e20313_d_n4, assign14790_e20313_d_n5, assign14790_e20313_d_n6, assign14790_e20313_d_n7, assign14790_e20313_d_n8, assign14790_e20313_d_n9, assign14790_e20313_d_n10, assign14790_e20313_d_n11,) = {
    if ((locals.var_guard462 != 0.0) && (!(((locals.var_guard459 != 0.0) || (locals.var_guard460 != 0.0)) || (locals.var_guard461 != 0.0)))) {
        let assign14790_e20309: f64 = (locals.var_nuendd + locals.var_nuintd);
        let assign14790_e20311: f64 = (assign14790_e20309 * locals.var_pdsha);
        (assign14790_e20311, (assign14790_e20309 * locals.var_pdsha_dn3), (assign14790_e20309 * locals.var_pdsha_dn4), (assign14790_e20309 * locals.var_pdsha_dn5), (assign14790_e20309 * locals.var_pdsha_dn6), (assign14790_e20309 * locals.var_pdsha_dn7), (assign14790_e20309 * locals.var_pdsha_dn8), (assign14790_e20309 * locals.var_pdsha_dn9), (assign14790_e20309 * locals.var_pdsha_dn10), (assign14790_e20309 * locals.var_pdsha_dn11),)
    } else {
        (locals.var_temp_pdeff, locals.var_temp_pdeff_dn3, locals.var_temp_pdeff_dn4, locals.var_temp_pdeff_dn5, locals.var_temp_pdeff_dn6, locals.var_temp_pdeff_dn7, locals.var_temp_pdeff_dn8, locals.var_temp_pdeff_dn9, locals.var_temp_pdeff_dn10, locals.var_temp_pdeff_dn11,)
    }
};
        locals.var_temp_pdeff = assign14790_e20313;
        locals.var_temp_pdeff_dn3 = assign14790_e20313_d_n3;
        locals.var_temp_pdeff_dn4 = assign14790_e20313_d_n4;
        locals.var_temp_pdeff_dn5 = assign14790_e20313_d_n5;
        locals.var_temp_pdeff_dn6 = assign14790_e20313_d_n6;
        locals.var_temp_pdeff_dn7 = assign14790_e20313_d_n7;
        locals.var_temp_pdeff_dn8 = assign14790_e20313_d_n8;
        locals.var_temp_pdeff_dn9 = assign14790_e20313_d_n9;
        locals.var_temp_pdeff_dn10 = assign14790_e20313_d_n10;
        locals.var_temp_pdeff_dn11 = assign14790_e20313_d_n11;
        locals.var_temp_pdeff_rv = 0.0;

        let (assign14800_e20328, assign14800_e20328_d_n3, assign14800_e20328_d_n4, assign14800_e20328_d_n5, assign14800_e20328_d_n6, assign14800_e20328_d_n7, assign14800_e20328_d_n8, assign14800_e20328_d_n9, assign14800_e20328_d_n10, assign14800_e20328_d_n11,) = {
    if ((locals.var_guard462 != 0.0) && (!(((locals.var_guard459 != 0.0) || (locals.var_guard460 != 0.0)) || (locals.var_guard461 != 0.0)))) {
        let assign14800_e20324: f64 = (locals.var_nuends + locals.var_nuints);
        let assign14800_e20326: f64 = (assign14800_e20324 * locals.var_assha);
        (assign14800_e20326, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp_aseff, locals.var_temp_aseff_dn3, locals.var_temp_aseff_dn4, locals.var_temp_aseff_dn5, locals.var_temp_aseff_dn6, locals.var_temp_aseff_dn7, locals.var_temp_aseff_dn8, locals.var_temp_aseff_dn9, locals.var_temp_aseff_dn10, locals.var_temp_aseff_dn11,)
    }
};
        locals.var_temp_aseff = assign14800_e20328;
        locals.var_temp_aseff_dn3 = assign14800_e20328_d_n3;
        locals.var_temp_aseff_dn4 = assign14800_e20328_d_n4;
        locals.var_temp_aseff_dn5 = assign14800_e20328_d_n5;
        locals.var_temp_aseff_dn6 = assign14800_e20328_d_n6;
        locals.var_temp_aseff_dn7 = assign14800_e20328_d_n7;
        locals.var_temp_aseff_dn8 = assign14800_e20328_d_n8;
        locals.var_temp_aseff_dn9 = assign14800_e20328_d_n9;
        locals.var_temp_aseff_dn10 = assign14800_e20328_d_n10;
        locals.var_temp_aseff_dn11 = assign14800_e20328_d_n11;
        locals.var_temp_aseff_rv = 0.0;

        let (assign14810_e20343, assign14810_e20343_d_n3, assign14810_e20343_d_n4, assign14810_e20343_d_n5, assign14810_e20343_d_n6, assign14810_e20343_d_n7, assign14810_e20343_d_n8, assign14810_e20343_d_n9, assign14810_e20343_d_n10, assign14810_e20343_d_n11,) = {
    if ((locals.var_guard462 != 0.0) && (!(((locals.var_guard459 != 0.0) || (locals.var_guard460 != 0.0)) || (locals.var_guard461 != 0.0)))) {
        let assign14810_e20339: f64 = (locals.var_nuendd + locals.var_nuintd);
        let assign14810_e20341: f64 = (assign14810_e20339 * locals.var_adsha);
        (assign14810_e20341, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp_adeff, locals.var_temp_adeff_dn3, locals.var_temp_adeff_dn4, locals.var_temp_adeff_dn5, locals.var_temp_adeff_dn6, locals.var_temp_adeff_dn7, locals.var_temp_adeff_dn8, locals.var_temp_adeff_dn9, locals.var_temp_adeff_dn10, locals.var_temp_adeff_dn11,)
    }
};
        locals.var_temp_adeff = assign14810_e20343;
        locals.var_temp_adeff_dn3 = assign14810_e20343_d_n3;
        locals.var_temp_adeff_dn4 = assign14810_e20343_d_n4;
        locals.var_temp_adeff_dn5 = assign14810_e20343_d_n5;
        locals.var_temp_adeff_dn6 = assign14810_e20343_d_n6;
        locals.var_temp_adeff_dn7 = assign14810_e20343_d_n7;
        locals.var_temp_adeff_dn8 = assign14810_e20343_d_n8;
        locals.var_temp_adeff_dn9 = assign14810_e20343_d_n9;
        locals.var_temp_adeff_dn10 = assign14810_e20343_d_n10;
        locals.var_temp_adeff_dn11 = assign14810_e20343_d_n11;
        locals.var_temp_adeff_rv = 0.0;

        let (assign14820_e20362, assign14820_e20362_d_n3, assign14820_e20362_d_n4, assign14820_e20362_d_n5, assign14820_e20362_d_n6, assign14820_e20362_d_n7, assign14820_e20362_d_n8, assign14820_e20362_d_n9, assign14820_e20362_d_n10, assign14820_e20362_d_n11,) = {
    if ((locals.var_guard463 != 0.0) && (!((((locals.var_guard459 != 0.0) || (locals.var_guard460 != 0.0)) || (locals.var_guard461 != 0.0)) || (locals.var_guard462 != 0.0)))) {
        let assign14820_e20356: f64 = (locals.var_nuends * locals.var_psiso);
        let assign14820_e20359: f64 = (locals.var_nuints * locals.var_pssha);
        let assign14820_e20360: f64 = (assign14820_e20356 + assign14820_e20359);
        (assign14820_e20360, ((locals.var_nuends * locals.var_psiso_dn3) + (locals.var_nuints * locals.var_pssha_dn3)), ((locals.var_nuends * locals.var_psiso_dn4) + (locals.var_nuints * locals.var_pssha_dn4)), ((locals.var_nuends * locals.var_psiso_dn5) + (locals.var_nuints * locals.var_pssha_dn5)), ((locals.var_nuends * locals.var_psiso_dn6) + (locals.var_nuints * locals.var_pssha_dn6)), ((locals.var_nuends * locals.var_psiso_dn7) + (locals.var_nuints * locals.var_pssha_dn7)), ((locals.var_nuends * locals.var_psiso_dn8) + (locals.var_nuints * locals.var_pssha_dn8)), ((locals.var_nuends * locals.var_psiso_dn9) + (locals.var_nuints * locals.var_pssha_dn9)), ((locals.var_nuends * locals.var_psiso_dn10) + (locals.var_nuints * locals.var_pssha_dn10)), ((locals.var_nuends * locals.var_psiso_dn11) + (locals.var_nuints * locals.var_pssha_dn11)),)
    } else {
        (locals.var_temp_pseff, locals.var_temp_pseff_dn3, locals.var_temp_pseff_dn4, locals.var_temp_pseff_dn5, locals.var_temp_pseff_dn6, locals.var_temp_pseff_dn7, locals.var_temp_pseff_dn8, locals.var_temp_pseff_dn9, locals.var_temp_pseff_dn10, locals.var_temp_pseff_dn11,)
    }
};
        locals.var_temp_pseff = assign14820_e20362;
        locals.var_temp_pseff_dn3 = assign14820_e20362_d_n3;
        locals.var_temp_pseff_dn4 = assign14820_e20362_d_n4;
        locals.var_temp_pseff_dn5 = assign14820_e20362_d_n5;
        locals.var_temp_pseff_dn6 = assign14820_e20362_d_n6;
        locals.var_temp_pseff_dn7 = assign14820_e20362_d_n7;
        locals.var_temp_pseff_dn8 = assign14820_e20362_d_n8;
        locals.var_temp_pseff_dn9 = assign14820_e20362_d_n9;
        locals.var_temp_pseff_dn10 = assign14820_e20362_d_n10;
        locals.var_temp_pseff_dn11 = assign14820_e20362_d_n11;
        locals.var_temp_pseff_rv = 0.0;

        let (assign14830_e20381, assign14830_e20381_d_n3, assign14830_e20381_d_n4, assign14830_e20381_d_n5, assign14830_e20381_d_n6, assign14830_e20381_d_n7, assign14830_e20381_d_n8, assign14830_e20381_d_n9, assign14830_e20381_d_n10, assign14830_e20381_d_n11,) = {
    if ((locals.var_guard463 != 0.0) && (!((((locals.var_guard459 != 0.0) || (locals.var_guard460 != 0.0)) || (locals.var_guard461 != 0.0)) || (locals.var_guard462 != 0.0)))) {
        let assign14830_e20375: f64 = (locals.var_nuendd * locals.var_pdmer);
        let assign14830_e20378: f64 = (locals.var_nuintd * locals.var_pdsha);
        let assign14830_e20379: f64 = (assign14830_e20375 + assign14830_e20378);
        (assign14830_e20379, ((locals.var_nuendd * locals.var_pdmer_dn3) + (locals.var_nuintd * locals.var_pdsha_dn3)), ((locals.var_nuendd * locals.var_pdmer_dn4) + (locals.var_nuintd * locals.var_pdsha_dn4)), ((locals.var_nuendd * locals.var_pdmer_dn5) + (locals.var_nuintd * locals.var_pdsha_dn5)), ((locals.var_nuendd * locals.var_pdmer_dn6) + (locals.var_nuintd * locals.var_pdsha_dn6)), ((locals.var_nuendd * locals.var_pdmer_dn7) + (locals.var_nuintd * locals.var_pdsha_dn7)), ((locals.var_nuendd * locals.var_pdmer_dn8) + (locals.var_nuintd * locals.var_pdsha_dn8)), ((locals.var_nuendd * locals.var_pdmer_dn9) + (locals.var_nuintd * locals.var_pdsha_dn9)), ((locals.var_nuendd * locals.var_pdmer_dn10) + (locals.var_nuintd * locals.var_pdsha_dn10)), ((locals.var_nuendd * locals.var_pdmer_dn11) + (locals.var_nuintd * locals.var_pdsha_dn11)),)
    } else {
        (locals.var_temp_pdeff, locals.var_temp_pdeff_dn3, locals.var_temp_pdeff_dn4, locals.var_temp_pdeff_dn5, locals.var_temp_pdeff_dn6, locals.var_temp_pdeff_dn7, locals.var_temp_pdeff_dn8, locals.var_temp_pdeff_dn9, locals.var_temp_pdeff_dn10, locals.var_temp_pdeff_dn11,)
    }
};
        locals.var_temp_pdeff = assign14830_e20381;
        locals.var_temp_pdeff_dn3 = assign14830_e20381_d_n3;
        locals.var_temp_pdeff_dn4 = assign14830_e20381_d_n4;
        locals.var_temp_pdeff_dn5 = assign14830_e20381_d_n5;
        locals.var_temp_pdeff_dn6 = assign14830_e20381_d_n6;
        locals.var_temp_pdeff_dn7 = assign14830_e20381_d_n7;
        locals.var_temp_pdeff_dn8 = assign14830_e20381_d_n8;
        locals.var_temp_pdeff_dn9 = assign14830_e20381_d_n9;
        locals.var_temp_pdeff_dn10 = assign14830_e20381_d_n10;
        locals.var_temp_pdeff_dn11 = assign14830_e20381_d_n11;
        locals.var_temp_pdeff_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_26(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign14840_e20400, assign14840_e20400_d_n3, assign14840_e20400_d_n4, assign14840_e20400_d_n5, assign14840_e20400_d_n6, assign14840_e20400_d_n7, assign14840_e20400_d_n8, assign14840_e20400_d_n9, assign14840_e20400_d_n10, assign14840_e20400_d_n11,) = {
    if ((locals.var_guard463 != 0.0) && (!((((locals.var_guard459 != 0.0) || (locals.var_guard460 != 0.0)) || (locals.var_guard461 != 0.0)) || (locals.var_guard462 != 0.0)))) {
        let assign14840_e20394: f64 = (locals.var_nuends * locals.var_asiso);
        let assign14840_e20397: f64 = (locals.var_nuints * locals.var_assha);
        let assign14840_e20398: f64 = (assign14840_e20394 + assign14840_e20397);
        (assign14840_e20398, (locals.var_nuends * locals.var_asiso_dn3), (locals.var_nuends * locals.var_asiso_dn4), (locals.var_nuends * locals.var_asiso_dn5), (locals.var_nuends * locals.var_asiso_dn6), (locals.var_nuends * locals.var_asiso_dn7), (locals.var_nuends * locals.var_asiso_dn8), (locals.var_nuends * locals.var_asiso_dn9), (locals.var_nuends * locals.var_asiso_dn10), (locals.var_nuends * locals.var_asiso_dn11),)
    } else {
        (locals.var_temp_aseff, locals.var_temp_aseff_dn3, locals.var_temp_aseff_dn4, locals.var_temp_aseff_dn5, locals.var_temp_aseff_dn6, locals.var_temp_aseff_dn7, locals.var_temp_aseff_dn8, locals.var_temp_aseff_dn9, locals.var_temp_aseff_dn10, locals.var_temp_aseff_dn11,)
    }
};
        locals.var_temp_aseff = assign14840_e20400;
        locals.var_temp_aseff_dn3 = assign14840_e20400_d_n3;
        locals.var_temp_aseff_dn4 = assign14840_e20400_d_n4;
        locals.var_temp_aseff_dn5 = assign14840_e20400_d_n5;
        locals.var_temp_aseff_dn6 = assign14840_e20400_d_n6;
        locals.var_temp_aseff_dn7 = assign14840_e20400_d_n7;
        locals.var_temp_aseff_dn8 = assign14840_e20400_d_n8;
        locals.var_temp_aseff_dn9 = assign14840_e20400_d_n9;
        locals.var_temp_aseff_dn10 = assign14840_e20400_d_n10;
        locals.var_temp_aseff_dn11 = assign14840_e20400_d_n11;
        locals.var_temp_aseff_rv = 0.0;

        let (assign14850_e20419, assign14850_e20419_d_n3, assign14850_e20419_d_n4, assign14850_e20419_d_n5, assign14850_e20419_d_n6, assign14850_e20419_d_n7, assign14850_e20419_d_n8, assign14850_e20419_d_n9, assign14850_e20419_d_n10, assign14850_e20419_d_n11,) = {
    if ((locals.var_guard463 != 0.0) && (!((((locals.var_guard459 != 0.0) || (locals.var_guard460 != 0.0)) || (locals.var_guard461 != 0.0)) || (locals.var_guard462 != 0.0)))) {
        let assign14850_e20413: f64 = (locals.var_nuendd * locals.var_admer);
        let assign14850_e20416: f64 = (locals.var_nuintd * locals.var_adsha);
        let assign14850_e20417: f64 = (assign14850_e20413 + assign14850_e20416);
        (assign14850_e20417, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp_adeff, locals.var_temp_adeff_dn3, locals.var_temp_adeff_dn4, locals.var_temp_adeff_dn5, locals.var_temp_adeff_dn6, locals.var_temp_adeff_dn7, locals.var_temp_adeff_dn8, locals.var_temp_adeff_dn9, locals.var_temp_adeff_dn10, locals.var_temp_adeff_dn11,)
    }
};
        locals.var_temp_adeff = assign14850_e20419;
        locals.var_temp_adeff_dn3 = assign14850_e20419_d_n3;
        locals.var_temp_adeff_dn4 = assign14850_e20419_d_n4;
        locals.var_temp_adeff_dn5 = assign14850_e20419_d_n5;
        locals.var_temp_adeff_dn6 = assign14850_e20419_d_n6;
        locals.var_temp_adeff_dn7 = assign14850_e20419_d_n7;
        locals.var_temp_adeff_dn8 = assign14850_e20419_d_n8;
        locals.var_temp_adeff_dn9 = assign14850_e20419_d_n9;
        locals.var_temp_adeff_dn10 = assign14850_e20419_d_n10;
        locals.var_temp_adeff_dn11 = assign14850_e20419_d_n11;
        locals.var_temp_adeff_rv = 0.0;

        let (assign14860_e20438, assign14860_e20438_d_n3, assign14860_e20438_d_n4, assign14860_e20438_d_n5, assign14860_e20438_d_n6, assign14860_e20438_d_n7, assign14860_e20438_d_n8, assign14860_e20438_d_n9, assign14860_e20438_d_n10, assign14860_e20438_d_n11,) = {
    if ((locals.var_guard464 != 0.0) && (!(((((locals.var_guard459 != 0.0) || (locals.var_guard460 != 0.0)) || (locals.var_guard461 != 0.0)) || (locals.var_guard462 != 0.0)) || (locals.var_guard463 != 0.0)))) {
        let assign14860_e20434: f64 = (locals.var_nuends + locals.var_nuints);
        let assign14860_e20436: f64 = (assign14860_e20434 * locals.var_pssha);
        (assign14860_e20436, (assign14860_e20434 * locals.var_pssha_dn3), (assign14860_e20434 * locals.var_pssha_dn4), (assign14860_e20434 * locals.var_pssha_dn5), (assign14860_e20434 * locals.var_pssha_dn6), (assign14860_e20434 * locals.var_pssha_dn7), (assign14860_e20434 * locals.var_pssha_dn8), (assign14860_e20434 * locals.var_pssha_dn9), (assign14860_e20434 * locals.var_pssha_dn10), (assign14860_e20434 * locals.var_pssha_dn11),)
    } else {
        (locals.var_temp_pseff, locals.var_temp_pseff_dn3, locals.var_temp_pseff_dn4, locals.var_temp_pseff_dn5, locals.var_temp_pseff_dn6, locals.var_temp_pseff_dn7, locals.var_temp_pseff_dn8, locals.var_temp_pseff_dn9, locals.var_temp_pseff_dn10, locals.var_temp_pseff_dn11,)
    }
};
        locals.var_temp_pseff = assign14860_e20438;
        locals.var_temp_pseff_dn3 = assign14860_e20438_d_n3;
        locals.var_temp_pseff_dn4 = assign14860_e20438_d_n4;
        locals.var_temp_pseff_dn5 = assign14860_e20438_d_n5;
        locals.var_temp_pseff_dn6 = assign14860_e20438_d_n6;
        locals.var_temp_pseff_dn7 = assign14860_e20438_d_n7;
        locals.var_temp_pseff_dn8 = assign14860_e20438_d_n8;
        locals.var_temp_pseff_dn9 = assign14860_e20438_d_n9;
        locals.var_temp_pseff_dn10 = assign14860_e20438_d_n10;
        locals.var_temp_pseff_dn11 = assign14860_e20438_d_n11;
        locals.var_temp_pseff_rv = 0.0;

        let (assign14870_e20459, assign14870_e20459_d_n3, assign14870_e20459_d_n4, assign14870_e20459_d_n5, assign14870_e20459_d_n6, assign14870_e20459_d_n7, assign14870_e20459_d_n8, assign14870_e20459_d_n9, assign14870_e20459_d_n10, assign14870_e20459_d_n11,) = {
    if ((locals.var_guard464 != 0.0) && (!(((((locals.var_guard459 != 0.0) || (locals.var_guard460 != 0.0)) || (locals.var_guard461 != 0.0)) || (locals.var_guard462 != 0.0)) || (locals.var_guard463 != 0.0)))) {
        let assign14870_e20453: f64 = (locals.var_nuendd * locals.var_pdmer);
        let assign14870_e20456: f64 = (locals.var_nuintd * locals.var_pdsha);
        let assign14870_e20457: f64 = (assign14870_e20453 + assign14870_e20456);
        (assign14870_e20457, ((locals.var_nuendd * locals.var_pdmer_dn3) + (locals.var_nuintd * locals.var_pdsha_dn3)), ((locals.var_nuendd * locals.var_pdmer_dn4) + (locals.var_nuintd * locals.var_pdsha_dn4)), ((locals.var_nuendd * locals.var_pdmer_dn5) + (locals.var_nuintd * locals.var_pdsha_dn5)), ((locals.var_nuendd * locals.var_pdmer_dn6) + (locals.var_nuintd * locals.var_pdsha_dn6)), ((locals.var_nuendd * locals.var_pdmer_dn7) + (locals.var_nuintd * locals.var_pdsha_dn7)), ((locals.var_nuendd * locals.var_pdmer_dn8) + (locals.var_nuintd * locals.var_pdsha_dn8)), ((locals.var_nuendd * locals.var_pdmer_dn9) + (locals.var_nuintd * locals.var_pdsha_dn9)), ((locals.var_nuendd * locals.var_pdmer_dn10) + (locals.var_nuintd * locals.var_pdsha_dn10)), ((locals.var_nuendd * locals.var_pdmer_dn11) + (locals.var_nuintd * locals.var_pdsha_dn11)),)
    } else {
        (locals.var_temp_pdeff, locals.var_temp_pdeff_dn3, locals.var_temp_pdeff_dn4, locals.var_temp_pdeff_dn5, locals.var_temp_pdeff_dn6, locals.var_temp_pdeff_dn7, locals.var_temp_pdeff_dn8, locals.var_temp_pdeff_dn9, locals.var_temp_pdeff_dn10, locals.var_temp_pdeff_dn11,)
    }
};
        locals.var_temp_pdeff = assign14870_e20459;
        locals.var_temp_pdeff_dn3 = assign14870_e20459_d_n3;
        locals.var_temp_pdeff_dn4 = assign14870_e20459_d_n4;
        locals.var_temp_pdeff_dn5 = assign14870_e20459_d_n5;
        locals.var_temp_pdeff_dn6 = assign14870_e20459_d_n6;
        locals.var_temp_pdeff_dn7 = assign14870_e20459_d_n7;
        locals.var_temp_pdeff_dn8 = assign14870_e20459_d_n8;
        locals.var_temp_pdeff_dn9 = assign14870_e20459_d_n9;
        locals.var_temp_pdeff_dn10 = assign14870_e20459_d_n10;
        locals.var_temp_pdeff_dn11 = assign14870_e20459_d_n11;
        locals.var_temp_pdeff_rv = 0.0;

        let (assign14880_e20478, assign14880_e20478_d_n3, assign14880_e20478_d_n4, assign14880_e20478_d_n5, assign14880_e20478_d_n6, assign14880_e20478_d_n7, assign14880_e20478_d_n8, assign14880_e20478_d_n9, assign14880_e20478_d_n10, assign14880_e20478_d_n11,) = {
    if ((locals.var_guard464 != 0.0) && (!(((((locals.var_guard459 != 0.0) || (locals.var_guard460 != 0.0)) || (locals.var_guard461 != 0.0)) || (locals.var_guard462 != 0.0)) || (locals.var_guard463 != 0.0)))) {
        let assign14880_e20474: f64 = (locals.var_nuends + locals.var_nuints);
        let assign14880_e20476: f64 = (assign14880_e20474 * locals.var_assha);
        (assign14880_e20476, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp_aseff, locals.var_temp_aseff_dn3, locals.var_temp_aseff_dn4, locals.var_temp_aseff_dn5, locals.var_temp_aseff_dn6, locals.var_temp_aseff_dn7, locals.var_temp_aseff_dn8, locals.var_temp_aseff_dn9, locals.var_temp_aseff_dn10, locals.var_temp_aseff_dn11,)
    }
};
        locals.var_temp_aseff = assign14880_e20478;
        locals.var_temp_aseff_dn3 = assign14880_e20478_d_n3;
        locals.var_temp_aseff_dn4 = assign14880_e20478_d_n4;
        locals.var_temp_aseff_dn5 = assign14880_e20478_d_n5;
        locals.var_temp_aseff_dn6 = assign14880_e20478_d_n6;
        locals.var_temp_aseff_dn7 = assign14880_e20478_d_n7;
        locals.var_temp_aseff_dn8 = assign14880_e20478_d_n8;
        locals.var_temp_aseff_dn9 = assign14880_e20478_d_n9;
        locals.var_temp_aseff_dn10 = assign14880_e20478_d_n10;
        locals.var_temp_aseff_dn11 = assign14880_e20478_d_n11;
        locals.var_temp_aseff_rv = 0.0;

        let (assign14890_e20499, assign14890_e20499_d_n3, assign14890_e20499_d_n4, assign14890_e20499_d_n5, assign14890_e20499_d_n6, assign14890_e20499_d_n7, assign14890_e20499_d_n8, assign14890_e20499_d_n9, assign14890_e20499_d_n10, assign14890_e20499_d_n11,) = {
    if ((locals.var_guard464 != 0.0) && (!(((((locals.var_guard459 != 0.0) || (locals.var_guard460 != 0.0)) || (locals.var_guard461 != 0.0)) || (locals.var_guard462 != 0.0)) || (locals.var_guard463 != 0.0)))) {
        let assign14890_e20493: f64 = (locals.var_nuendd * locals.var_admer);
        let assign14890_e20496: f64 = (locals.var_nuintd * locals.var_adsha);
        let assign14890_e20497: f64 = (assign14890_e20493 + assign14890_e20496);
        (assign14890_e20497, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp_adeff, locals.var_temp_adeff_dn3, locals.var_temp_adeff_dn4, locals.var_temp_adeff_dn5, locals.var_temp_adeff_dn6, locals.var_temp_adeff_dn7, locals.var_temp_adeff_dn8, locals.var_temp_adeff_dn9, locals.var_temp_adeff_dn10, locals.var_temp_adeff_dn11,)
    }
};
        locals.var_temp_adeff = assign14890_e20499;
        locals.var_temp_adeff_dn3 = assign14890_e20499_d_n3;
        locals.var_temp_adeff_dn4 = assign14890_e20499_d_n4;
        locals.var_temp_adeff_dn5 = assign14890_e20499_d_n5;
        locals.var_temp_adeff_dn6 = assign14890_e20499_d_n6;
        locals.var_temp_adeff_dn7 = assign14890_e20499_d_n7;
        locals.var_temp_adeff_dn8 = assign14890_e20499_d_n8;
        locals.var_temp_adeff_dn9 = assign14890_e20499_d_n9;
        locals.var_temp_adeff_dn10 = assign14890_e20499_d_n10;
        locals.var_temp_adeff_dn11 = assign14890_e20499_d_n11;
        locals.var_temp_adeff_rv = 0.0;

        let (assign14900_e20522, assign14900_e20522_d_n3, assign14900_e20522_d_n4, assign14900_e20522_d_n5, assign14900_e20522_d_n6, assign14900_e20522_d_n7, assign14900_e20522_d_n8, assign14900_e20522_d_n9, assign14900_e20522_d_n10, assign14900_e20522_d_n11,) = {
    if ((locals.var_guard465 != 0.0) && (!((((((locals.var_guard459 != 0.0) || (locals.var_guard460 != 0.0)) || (locals.var_guard461 != 0.0)) || (locals.var_guard462 != 0.0)) || (locals.var_guard463 != 0.0)) || (locals.var_guard464 != 0.0)))) {
        let assign14900_e20516: f64 = (locals.var_nuends * locals.var_psmer);
        let assign14900_e20519: f64 = (locals.var_nuints * locals.var_pssha);
        let assign14900_e20520: f64 = (assign14900_e20516 + assign14900_e20519);
        (assign14900_e20520, ((locals.var_nuends * locals.var_psmer_dn3) + (locals.var_nuints * locals.var_pssha_dn3)), ((locals.var_nuends * locals.var_psmer_dn4) + (locals.var_nuints * locals.var_pssha_dn4)), ((locals.var_nuends * locals.var_psmer_dn5) + (locals.var_nuints * locals.var_pssha_dn5)), ((locals.var_nuends * locals.var_psmer_dn6) + (locals.var_nuints * locals.var_pssha_dn6)), ((locals.var_nuends * locals.var_psmer_dn7) + (locals.var_nuints * locals.var_pssha_dn7)), ((locals.var_nuends * locals.var_psmer_dn8) + (locals.var_nuints * locals.var_pssha_dn8)), ((locals.var_nuends * locals.var_psmer_dn9) + (locals.var_nuints * locals.var_pssha_dn9)), ((locals.var_nuends * locals.var_psmer_dn10) + (locals.var_nuints * locals.var_pssha_dn10)), ((locals.var_nuends * locals.var_psmer_dn11) + (locals.var_nuints * locals.var_pssha_dn11)),)
    } else {
        (locals.var_temp_pseff, locals.var_temp_pseff_dn3, locals.var_temp_pseff_dn4, locals.var_temp_pseff_dn5, locals.var_temp_pseff_dn6, locals.var_temp_pseff_dn7, locals.var_temp_pseff_dn8, locals.var_temp_pseff_dn9, locals.var_temp_pseff_dn10, locals.var_temp_pseff_dn11,)
    }
};
        locals.var_temp_pseff = assign14900_e20522;
        locals.var_temp_pseff_dn3 = assign14900_e20522_d_n3;
        locals.var_temp_pseff_dn4 = assign14900_e20522_d_n4;
        locals.var_temp_pseff_dn5 = assign14900_e20522_d_n5;
        locals.var_temp_pseff_dn6 = assign14900_e20522_d_n6;
        locals.var_temp_pseff_dn7 = assign14900_e20522_d_n7;
        locals.var_temp_pseff_dn8 = assign14900_e20522_d_n8;
        locals.var_temp_pseff_dn9 = assign14900_e20522_d_n9;
        locals.var_temp_pseff_dn10 = assign14900_e20522_d_n10;
        locals.var_temp_pseff_dn11 = assign14900_e20522_d_n11;
        locals.var_temp_pseff_rv = 0.0;

        let (assign14910_e20545, assign14910_e20545_d_n3, assign14910_e20545_d_n4, assign14910_e20545_d_n5, assign14910_e20545_d_n6, assign14910_e20545_d_n7, assign14910_e20545_d_n8, assign14910_e20545_d_n9, assign14910_e20545_d_n10, assign14910_e20545_d_n11,) = {
    if ((locals.var_guard465 != 0.0) && (!((((((locals.var_guard459 != 0.0) || (locals.var_guard460 != 0.0)) || (locals.var_guard461 != 0.0)) || (locals.var_guard462 != 0.0)) || (locals.var_guard463 != 0.0)) || (locals.var_guard464 != 0.0)))) {
        let assign14910_e20539: f64 = (locals.var_nuendd * locals.var_pdiso);
        let assign14910_e20542: f64 = (locals.var_nuintd * locals.var_pdsha);
        let assign14910_e20543: f64 = (assign14910_e20539 + assign14910_e20542);
        (assign14910_e20543, ((locals.var_nuendd * locals.var_pdiso_dn3) + (locals.var_nuintd * locals.var_pdsha_dn3)), ((locals.var_nuendd * locals.var_pdiso_dn4) + (locals.var_nuintd * locals.var_pdsha_dn4)), ((locals.var_nuendd * locals.var_pdiso_dn5) + (locals.var_nuintd * locals.var_pdsha_dn5)), ((locals.var_nuendd * locals.var_pdiso_dn6) + (locals.var_nuintd * locals.var_pdsha_dn6)), ((locals.var_nuendd * locals.var_pdiso_dn7) + (locals.var_nuintd * locals.var_pdsha_dn7)), ((locals.var_nuendd * locals.var_pdiso_dn8) + (locals.var_nuintd * locals.var_pdsha_dn8)), ((locals.var_nuendd * locals.var_pdiso_dn9) + (locals.var_nuintd * locals.var_pdsha_dn9)), ((locals.var_nuendd * locals.var_pdiso_dn10) + (locals.var_nuintd * locals.var_pdsha_dn10)), ((locals.var_nuendd * locals.var_pdiso_dn11) + (locals.var_nuintd * locals.var_pdsha_dn11)),)
    } else {
        (locals.var_temp_pdeff, locals.var_temp_pdeff_dn3, locals.var_temp_pdeff_dn4, locals.var_temp_pdeff_dn5, locals.var_temp_pdeff_dn6, locals.var_temp_pdeff_dn7, locals.var_temp_pdeff_dn8, locals.var_temp_pdeff_dn9, locals.var_temp_pdeff_dn10, locals.var_temp_pdeff_dn11,)
    }
};
        locals.var_temp_pdeff = assign14910_e20545;
        locals.var_temp_pdeff_dn3 = assign14910_e20545_d_n3;
        locals.var_temp_pdeff_dn4 = assign14910_e20545_d_n4;
        locals.var_temp_pdeff_dn5 = assign14910_e20545_d_n5;
        locals.var_temp_pdeff_dn6 = assign14910_e20545_d_n6;
        locals.var_temp_pdeff_dn7 = assign14910_e20545_d_n7;
        locals.var_temp_pdeff_dn8 = assign14910_e20545_d_n8;
        locals.var_temp_pdeff_dn9 = assign14910_e20545_d_n9;
        locals.var_temp_pdeff_dn10 = assign14910_e20545_d_n10;
        locals.var_temp_pdeff_dn11 = assign14910_e20545_d_n11;
        locals.var_temp_pdeff_rv = 0.0;

        let (assign14920_e20568, assign14920_e20568_d_n3, assign14920_e20568_d_n4, assign14920_e20568_d_n5, assign14920_e20568_d_n6, assign14920_e20568_d_n7, assign14920_e20568_d_n8, assign14920_e20568_d_n9, assign14920_e20568_d_n10, assign14920_e20568_d_n11,) = {
    if ((locals.var_guard465 != 0.0) && (!((((((locals.var_guard459 != 0.0) || (locals.var_guard460 != 0.0)) || (locals.var_guard461 != 0.0)) || (locals.var_guard462 != 0.0)) || (locals.var_guard463 != 0.0)) || (locals.var_guard464 != 0.0)))) {
        let assign14920_e20562: f64 = (locals.var_nuends * locals.var_asmer);
        let assign14920_e20565: f64 = (locals.var_nuints * locals.var_assha);
        let assign14920_e20566: f64 = (assign14920_e20562 + assign14920_e20565);
        (assign14920_e20566, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp_aseff, locals.var_temp_aseff_dn3, locals.var_temp_aseff_dn4, locals.var_temp_aseff_dn5, locals.var_temp_aseff_dn6, locals.var_temp_aseff_dn7, locals.var_temp_aseff_dn8, locals.var_temp_aseff_dn9, locals.var_temp_aseff_dn10, locals.var_temp_aseff_dn11,)
    }
};
        locals.var_temp_aseff = assign14920_e20568;
        locals.var_temp_aseff_dn3 = assign14920_e20568_d_n3;
        locals.var_temp_aseff_dn4 = assign14920_e20568_d_n4;
        locals.var_temp_aseff_dn5 = assign14920_e20568_d_n5;
        locals.var_temp_aseff_dn6 = assign14920_e20568_d_n6;
        locals.var_temp_aseff_dn7 = assign14920_e20568_d_n7;
        locals.var_temp_aseff_dn8 = assign14920_e20568_d_n8;
        locals.var_temp_aseff_dn9 = assign14920_e20568_d_n9;
        locals.var_temp_aseff_dn10 = assign14920_e20568_d_n10;
        locals.var_temp_aseff_dn11 = assign14920_e20568_d_n11;
        locals.var_temp_aseff_rv = 0.0;

        let (assign14930_e20591, assign14930_e20591_d_n3, assign14930_e20591_d_n4, assign14930_e20591_d_n5, assign14930_e20591_d_n6, assign14930_e20591_d_n7, assign14930_e20591_d_n8, assign14930_e20591_d_n9, assign14930_e20591_d_n10, assign14930_e20591_d_n11,) = {
    if ((locals.var_guard465 != 0.0) && (!((((((locals.var_guard459 != 0.0) || (locals.var_guard460 != 0.0)) || (locals.var_guard461 != 0.0)) || (locals.var_guard462 != 0.0)) || (locals.var_guard463 != 0.0)) || (locals.var_guard464 != 0.0)))) {
        let assign14930_e20585: f64 = (locals.var_nuendd * locals.var_adiso);
        let assign14930_e20588: f64 = (locals.var_nuintd * locals.var_adsha);
        let assign14930_e20589: f64 = (assign14930_e20585 + assign14930_e20588);
        (assign14930_e20589, (locals.var_nuendd * locals.var_adiso_dn3), (locals.var_nuendd * locals.var_adiso_dn4), (locals.var_nuendd * locals.var_adiso_dn5), (locals.var_nuendd * locals.var_adiso_dn6), (locals.var_nuendd * locals.var_adiso_dn7), (locals.var_nuendd * locals.var_adiso_dn8), (locals.var_nuendd * locals.var_adiso_dn9), (locals.var_nuendd * locals.var_adiso_dn10), (locals.var_nuendd * locals.var_adiso_dn11),)
    } else {
        (locals.var_temp_adeff, locals.var_temp_adeff_dn3, locals.var_temp_adeff_dn4, locals.var_temp_adeff_dn5, locals.var_temp_adeff_dn6, locals.var_temp_adeff_dn7, locals.var_temp_adeff_dn8, locals.var_temp_adeff_dn9, locals.var_temp_adeff_dn10, locals.var_temp_adeff_dn11,)
    }
};
        locals.var_temp_adeff = assign14930_e20591;
        locals.var_temp_adeff_dn3 = assign14930_e20591_d_n3;
        locals.var_temp_adeff_dn4 = assign14930_e20591_d_n4;
        locals.var_temp_adeff_dn5 = assign14930_e20591_d_n5;
        locals.var_temp_adeff_dn6 = assign14930_e20591_d_n6;
        locals.var_temp_adeff_dn7 = assign14930_e20591_d_n7;
        locals.var_temp_adeff_dn8 = assign14930_e20591_d_n8;
        locals.var_temp_adeff_dn9 = assign14930_e20591_d_n9;
        locals.var_temp_adeff_dn10 = assign14930_e20591_d_n10;
        locals.var_temp_adeff_dn11 = assign14930_e20591_d_n11;
        locals.var_temp_adeff_rv = 0.0;

        let (assign14940_e20616, assign14940_e20616_d_n3, assign14940_e20616_d_n4, assign14940_e20616_d_n5, assign14940_e20616_d_n6, assign14940_e20616_d_n7, assign14940_e20616_d_n8, assign14940_e20616_d_n9, assign14940_e20616_d_n10, assign14940_e20616_d_n11,) = {
    if ((locals.var_guard466 != 0.0) && (!(((((((locals.var_guard459 != 0.0) || (locals.var_guard460 != 0.0)) || (locals.var_guard461 != 0.0)) || (locals.var_guard462 != 0.0)) || (locals.var_guard463 != 0.0)) || (locals.var_guard464 != 0.0)) || (locals.var_guard465 != 0.0)))) {
        let assign14940_e20610: f64 = (locals.var_nuends * locals.var_psmer);
        let assign14940_e20613: f64 = (locals.var_nuints * locals.var_pssha);
        let assign14940_e20614: f64 = (assign14940_e20610 + assign14940_e20613);
        (assign14940_e20614, ((locals.var_nuends * locals.var_psmer_dn3) + (locals.var_nuints * locals.var_pssha_dn3)), ((locals.var_nuends * locals.var_psmer_dn4) + (locals.var_nuints * locals.var_pssha_dn4)), ((locals.var_nuends * locals.var_psmer_dn5) + (locals.var_nuints * locals.var_pssha_dn5)), ((locals.var_nuends * locals.var_psmer_dn6) + (locals.var_nuints * locals.var_pssha_dn6)), ((locals.var_nuends * locals.var_psmer_dn7) + (locals.var_nuints * locals.var_pssha_dn7)), ((locals.var_nuends * locals.var_psmer_dn8) + (locals.var_nuints * locals.var_pssha_dn8)), ((locals.var_nuends * locals.var_psmer_dn9) + (locals.var_nuints * locals.var_pssha_dn9)), ((locals.var_nuends * locals.var_psmer_dn10) + (locals.var_nuints * locals.var_pssha_dn10)), ((locals.var_nuends * locals.var_psmer_dn11) + (locals.var_nuints * locals.var_pssha_dn11)),)
    } else {
        (locals.var_temp_pseff, locals.var_temp_pseff_dn3, locals.var_temp_pseff_dn4, locals.var_temp_pseff_dn5, locals.var_temp_pseff_dn6, locals.var_temp_pseff_dn7, locals.var_temp_pseff_dn8, locals.var_temp_pseff_dn9, locals.var_temp_pseff_dn10, locals.var_temp_pseff_dn11,)
    }
};
        locals.var_temp_pseff = assign14940_e20616;
        locals.var_temp_pseff_dn3 = assign14940_e20616_d_n3;
        locals.var_temp_pseff_dn4 = assign14940_e20616_d_n4;
        locals.var_temp_pseff_dn5 = assign14940_e20616_d_n5;
        locals.var_temp_pseff_dn6 = assign14940_e20616_d_n6;
        locals.var_temp_pseff_dn7 = assign14940_e20616_d_n7;
        locals.var_temp_pseff_dn8 = assign14940_e20616_d_n8;
        locals.var_temp_pseff_dn9 = assign14940_e20616_d_n9;
        locals.var_temp_pseff_dn10 = assign14940_e20616_d_n10;
        locals.var_temp_pseff_dn11 = assign14940_e20616_d_n11;
        locals.var_temp_pseff_rv = 0.0;

        let (assign14950_e20639, assign14950_e20639_d_n3, assign14950_e20639_d_n4, assign14950_e20639_d_n5, assign14950_e20639_d_n6, assign14950_e20639_d_n7, assign14950_e20639_d_n8, assign14950_e20639_d_n9, assign14950_e20639_d_n10, assign14950_e20639_d_n11,) = {
    if ((locals.var_guard466 != 0.0) && (!(((((((locals.var_guard459 != 0.0) || (locals.var_guard460 != 0.0)) || (locals.var_guard461 != 0.0)) || (locals.var_guard462 != 0.0)) || (locals.var_guard463 != 0.0)) || (locals.var_guard464 != 0.0)) || (locals.var_guard465 != 0.0)))) {
        let assign14950_e20635: f64 = (locals.var_nuendd + locals.var_nuintd);
        let assign14950_e20637: f64 = (assign14950_e20635 * locals.var_pdsha);
        (assign14950_e20637, (assign14950_e20635 * locals.var_pdsha_dn3), (assign14950_e20635 * locals.var_pdsha_dn4), (assign14950_e20635 * locals.var_pdsha_dn5), (assign14950_e20635 * locals.var_pdsha_dn6), (assign14950_e20635 * locals.var_pdsha_dn7), (assign14950_e20635 * locals.var_pdsha_dn8), (assign14950_e20635 * locals.var_pdsha_dn9), (assign14950_e20635 * locals.var_pdsha_dn10), (assign14950_e20635 * locals.var_pdsha_dn11),)
    } else {
        (locals.var_temp_pdeff, locals.var_temp_pdeff_dn3, locals.var_temp_pdeff_dn4, locals.var_temp_pdeff_dn5, locals.var_temp_pdeff_dn6, locals.var_temp_pdeff_dn7, locals.var_temp_pdeff_dn8, locals.var_temp_pdeff_dn9, locals.var_temp_pdeff_dn10, locals.var_temp_pdeff_dn11,)
    }
};
        locals.var_temp_pdeff = assign14950_e20639;
        locals.var_temp_pdeff_dn3 = assign14950_e20639_d_n3;
        locals.var_temp_pdeff_dn4 = assign14950_e20639_d_n4;
        locals.var_temp_pdeff_dn5 = assign14950_e20639_d_n5;
        locals.var_temp_pdeff_dn6 = assign14950_e20639_d_n6;
        locals.var_temp_pdeff_dn7 = assign14950_e20639_d_n7;
        locals.var_temp_pdeff_dn8 = assign14950_e20639_d_n8;
        locals.var_temp_pdeff_dn9 = assign14950_e20639_d_n9;
        locals.var_temp_pdeff_dn10 = assign14950_e20639_d_n10;
        locals.var_temp_pdeff_dn11 = assign14950_e20639_d_n11;
        locals.var_temp_pdeff_rv = 0.0;

        let (assign14960_e20664, assign14960_e20664_d_n3, assign14960_e20664_d_n4, assign14960_e20664_d_n5, assign14960_e20664_d_n6, assign14960_e20664_d_n7, assign14960_e20664_d_n8, assign14960_e20664_d_n9, assign14960_e20664_d_n10, assign14960_e20664_d_n11,) = {
    if ((locals.var_guard466 != 0.0) && (!(((((((locals.var_guard459 != 0.0) || (locals.var_guard460 != 0.0)) || (locals.var_guard461 != 0.0)) || (locals.var_guard462 != 0.0)) || (locals.var_guard463 != 0.0)) || (locals.var_guard464 != 0.0)) || (locals.var_guard465 != 0.0)))) {
        let assign14960_e20658: f64 = (locals.var_nuends * locals.var_asmer);
        let assign14960_e20661: f64 = (locals.var_nuints * locals.var_assha);
        let assign14960_e20662: f64 = (assign14960_e20658 + assign14960_e20661);
        (assign14960_e20662, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp_aseff, locals.var_temp_aseff_dn3, locals.var_temp_aseff_dn4, locals.var_temp_aseff_dn5, locals.var_temp_aseff_dn6, locals.var_temp_aseff_dn7, locals.var_temp_aseff_dn8, locals.var_temp_aseff_dn9, locals.var_temp_aseff_dn10, locals.var_temp_aseff_dn11,)
    }
};
        locals.var_temp_aseff = assign14960_e20664;
        locals.var_temp_aseff_dn3 = assign14960_e20664_d_n3;
        locals.var_temp_aseff_dn4 = assign14960_e20664_d_n4;
        locals.var_temp_aseff_dn5 = assign14960_e20664_d_n5;
        locals.var_temp_aseff_dn6 = assign14960_e20664_d_n6;
        locals.var_temp_aseff_dn7 = assign14960_e20664_d_n7;
        locals.var_temp_aseff_dn8 = assign14960_e20664_d_n8;
        locals.var_temp_aseff_dn9 = assign14960_e20664_d_n9;
        locals.var_temp_aseff_dn10 = assign14960_e20664_d_n10;
        locals.var_temp_aseff_dn11 = assign14960_e20664_d_n11;
        locals.var_temp_aseff_rv = 0.0;

        let (assign14970_e20687, assign14970_e20687_d_n3, assign14970_e20687_d_n4, assign14970_e20687_d_n5, assign14970_e20687_d_n6, assign14970_e20687_d_n7, assign14970_e20687_d_n8, assign14970_e20687_d_n9, assign14970_e20687_d_n10, assign14970_e20687_d_n11,) = {
    if ((locals.var_guard466 != 0.0) && (!(((((((locals.var_guard459 != 0.0) || (locals.var_guard460 != 0.0)) || (locals.var_guard461 != 0.0)) || (locals.var_guard462 != 0.0)) || (locals.var_guard463 != 0.0)) || (locals.var_guard464 != 0.0)) || (locals.var_guard465 != 0.0)))) {
        let assign14970_e20683: f64 = (locals.var_nuendd + locals.var_nuintd);
        let assign14970_e20685: f64 = (assign14970_e20683 * locals.var_adsha);
        (assign14970_e20685, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp_adeff, locals.var_temp_adeff_dn3, locals.var_temp_adeff_dn4, locals.var_temp_adeff_dn5, locals.var_temp_adeff_dn6, locals.var_temp_adeff_dn7, locals.var_temp_adeff_dn8, locals.var_temp_adeff_dn9, locals.var_temp_adeff_dn10, locals.var_temp_adeff_dn11,)
    }
};
        locals.var_temp_adeff = assign14970_e20687;
        locals.var_temp_adeff_dn3 = assign14970_e20687_d_n3;
        locals.var_temp_adeff_dn4 = assign14970_e20687_d_n4;
        locals.var_temp_adeff_dn5 = assign14970_e20687_d_n5;
        locals.var_temp_adeff_dn6 = assign14970_e20687_d_n6;
        locals.var_temp_adeff_dn7 = assign14970_e20687_d_n7;
        locals.var_temp_adeff_dn8 = assign14970_e20687_d_n8;
        locals.var_temp_adeff_dn9 = assign14970_e20687_d_n9;
        locals.var_temp_adeff_dn10 = assign14970_e20687_d_n10;
        locals.var_temp_adeff_dn11 = assign14970_e20687_d_n11;
        locals.var_temp_adeff_rv = 0.0;

        let (assign14980_e20714, assign14980_e20714_d_n3, assign14980_e20714_d_n4, assign14980_e20714_d_n5, assign14980_e20714_d_n6, assign14980_e20714_d_n7, assign14980_e20714_d_n8, assign14980_e20714_d_n9, assign14980_e20714_d_n10, assign14980_e20714_d_n11,) = {
    if ((locals.var_guard467 != 0.0) && (!((((((((locals.var_guard459 != 0.0) || (locals.var_guard460 != 0.0)) || (locals.var_guard461 != 0.0)) || (locals.var_guard462 != 0.0)) || (locals.var_guard463 != 0.0)) || (locals.var_guard464 != 0.0)) || (locals.var_guard465 != 0.0)) || (locals.var_guard466 != 0.0)))) {
        let assign14980_e20708: f64 = (locals.var_nuends * locals.var_psmer);
        let assign14980_e20711: f64 = (locals.var_nuints * locals.var_pssha);
        let assign14980_e20712: f64 = (assign14980_e20708 + assign14980_e20711);
        (assign14980_e20712, ((locals.var_nuends * locals.var_psmer_dn3) + (locals.var_nuints * locals.var_pssha_dn3)), ((locals.var_nuends * locals.var_psmer_dn4) + (locals.var_nuints * locals.var_pssha_dn4)), ((locals.var_nuends * locals.var_psmer_dn5) + (locals.var_nuints * locals.var_pssha_dn5)), ((locals.var_nuends * locals.var_psmer_dn6) + (locals.var_nuints * locals.var_pssha_dn6)), ((locals.var_nuends * locals.var_psmer_dn7) + (locals.var_nuints * locals.var_pssha_dn7)), ((locals.var_nuends * locals.var_psmer_dn8) + (locals.var_nuints * locals.var_pssha_dn8)), ((locals.var_nuends * locals.var_psmer_dn9) + (locals.var_nuints * locals.var_pssha_dn9)), ((locals.var_nuends * locals.var_psmer_dn10) + (locals.var_nuints * locals.var_pssha_dn10)), ((locals.var_nuends * locals.var_psmer_dn11) + (locals.var_nuints * locals.var_pssha_dn11)),)
    } else {
        (locals.var_temp_pseff, locals.var_temp_pseff_dn3, locals.var_temp_pseff_dn4, locals.var_temp_pseff_dn5, locals.var_temp_pseff_dn6, locals.var_temp_pseff_dn7, locals.var_temp_pseff_dn8, locals.var_temp_pseff_dn9, locals.var_temp_pseff_dn10, locals.var_temp_pseff_dn11,)
    }
};
        locals.var_temp_pseff = assign14980_e20714;
        locals.var_temp_pseff_dn3 = assign14980_e20714_d_n3;
        locals.var_temp_pseff_dn4 = assign14980_e20714_d_n4;
        locals.var_temp_pseff_dn5 = assign14980_e20714_d_n5;
        locals.var_temp_pseff_dn6 = assign14980_e20714_d_n6;
        locals.var_temp_pseff_dn7 = assign14980_e20714_d_n7;
        locals.var_temp_pseff_dn8 = assign14980_e20714_d_n8;
        locals.var_temp_pseff_dn9 = assign14980_e20714_d_n9;
        locals.var_temp_pseff_dn10 = assign14980_e20714_d_n10;
        locals.var_temp_pseff_dn11 = assign14980_e20714_d_n11;
        locals.var_temp_pseff_rv = 0.0;

        let (assign14990_e20741, assign14990_e20741_d_n3, assign14990_e20741_d_n4, assign14990_e20741_d_n5, assign14990_e20741_d_n6, assign14990_e20741_d_n7, assign14990_e20741_d_n8, assign14990_e20741_d_n9, assign14990_e20741_d_n10, assign14990_e20741_d_n11,) = {
    if ((locals.var_guard467 != 0.0) && (!((((((((locals.var_guard459 != 0.0) || (locals.var_guard460 != 0.0)) || (locals.var_guard461 != 0.0)) || (locals.var_guard462 != 0.0)) || (locals.var_guard463 != 0.0)) || (locals.var_guard464 != 0.0)) || (locals.var_guard465 != 0.0)) || (locals.var_guard466 != 0.0)))) {
        let assign14990_e20735: f64 = (locals.var_nuendd * locals.var_pdmer);
        let assign14990_e20738: f64 = (locals.var_nuintd * locals.var_pdsha);
        let assign14990_e20739: f64 = (assign14990_e20735 + assign14990_e20738);
        (assign14990_e20739, ((locals.var_nuendd * locals.var_pdmer_dn3) + (locals.var_nuintd * locals.var_pdsha_dn3)), ((locals.var_nuendd * locals.var_pdmer_dn4) + (locals.var_nuintd * locals.var_pdsha_dn4)), ((locals.var_nuendd * locals.var_pdmer_dn5) + (locals.var_nuintd * locals.var_pdsha_dn5)), ((locals.var_nuendd * locals.var_pdmer_dn6) + (locals.var_nuintd * locals.var_pdsha_dn6)), ((locals.var_nuendd * locals.var_pdmer_dn7) + (locals.var_nuintd * locals.var_pdsha_dn7)), ((locals.var_nuendd * locals.var_pdmer_dn8) + (locals.var_nuintd * locals.var_pdsha_dn8)), ((locals.var_nuendd * locals.var_pdmer_dn9) + (locals.var_nuintd * locals.var_pdsha_dn9)), ((locals.var_nuendd * locals.var_pdmer_dn10) + (locals.var_nuintd * locals.var_pdsha_dn10)), ((locals.var_nuendd * locals.var_pdmer_dn11) + (locals.var_nuintd * locals.var_pdsha_dn11)),)
    } else {
        (locals.var_temp_pdeff, locals.var_temp_pdeff_dn3, locals.var_temp_pdeff_dn4, locals.var_temp_pdeff_dn5, locals.var_temp_pdeff_dn6, locals.var_temp_pdeff_dn7, locals.var_temp_pdeff_dn8, locals.var_temp_pdeff_dn9, locals.var_temp_pdeff_dn10, locals.var_temp_pdeff_dn11,)
    }
};
        locals.var_temp_pdeff = assign14990_e20741;
        locals.var_temp_pdeff_dn3 = assign14990_e20741_d_n3;
        locals.var_temp_pdeff_dn4 = assign14990_e20741_d_n4;
        locals.var_temp_pdeff_dn5 = assign14990_e20741_d_n5;
        locals.var_temp_pdeff_dn6 = assign14990_e20741_d_n6;
        locals.var_temp_pdeff_dn7 = assign14990_e20741_d_n7;
        locals.var_temp_pdeff_dn8 = assign14990_e20741_d_n8;
        locals.var_temp_pdeff_dn9 = assign14990_e20741_d_n9;
        locals.var_temp_pdeff_dn10 = assign14990_e20741_d_n10;
        locals.var_temp_pdeff_dn11 = assign14990_e20741_d_n11;
        locals.var_temp_pdeff_rv = 0.0;

        let (assign15000_e20768, assign15000_e20768_d_n3, assign15000_e20768_d_n4, assign15000_e20768_d_n5, assign15000_e20768_d_n6, assign15000_e20768_d_n7, assign15000_e20768_d_n8, assign15000_e20768_d_n9, assign15000_e20768_d_n10, assign15000_e20768_d_n11,) = {
    if ((locals.var_guard467 != 0.0) && (!((((((((locals.var_guard459 != 0.0) || (locals.var_guard460 != 0.0)) || (locals.var_guard461 != 0.0)) || (locals.var_guard462 != 0.0)) || (locals.var_guard463 != 0.0)) || (locals.var_guard464 != 0.0)) || (locals.var_guard465 != 0.0)) || (locals.var_guard466 != 0.0)))) {
        let assign15000_e20762: f64 = (locals.var_nuends * locals.var_asmer);
        let assign15000_e20765: f64 = (locals.var_nuints * locals.var_assha);
        let assign15000_e20766: f64 = (assign15000_e20762 + assign15000_e20765);
        (assign15000_e20766, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp_aseff, locals.var_temp_aseff_dn3, locals.var_temp_aseff_dn4, locals.var_temp_aseff_dn5, locals.var_temp_aseff_dn6, locals.var_temp_aseff_dn7, locals.var_temp_aseff_dn8, locals.var_temp_aseff_dn9, locals.var_temp_aseff_dn10, locals.var_temp_aseff_dn11,)
    }
};
        locals.var_temp_aseff = assign15000_e20768;
        locals.var_temp_aseff_dn3 = assign15000_e20768_d_n3;
        locals.var_temp_aseff_dn4 = assign15000_e20768_d_n4;
        locals.var_temp_aseff_dn5 = assign15000_e20768_d_n5;
        locals.var_temp_aseff_dn6 = assign15000_e20768_d_n6;
        locals.var_temp_aseff_dn7 = assign15000_e20768_d_n7;
        locals.var_temp_aseff_dn8 = assign15000_e20768_d_n8;
        locals.var_temp_aseff_dn9 = assign15000_e20768_d_n9;
        locals.var_temp_aseff_dn10 = assign15000_e20768_d_n10;
        locals.var_temp_aseff_dn11 = assign15000_e20768_d_n11;
        locals.var_temp_aseff_rv = 0.0;

        let (assign15010_e20795, assign15010_e20795_d_n3, assign15010_e20795_d_n4, assign15010_e20795_d_n5, assign15010_e20795_d_n6, assign15010_e20795_d_n7, assign15010_e20795_d_n8, assign15010_e20795_d_n9, assign15010_e20795_d_n10, assign15010_e20795_d_n11,) = {
    if ((locals.var_guard467 != 0.0) && (!((((((((locals.var_guard459 != 0.0) || (locals.var_guard460 != 0.0)) || (locals.var_guard461 != 0.0)) || (locals.var_guard462 != 0.0)) || (locals.var_guard463 != 0.0)) || (locals.var_guard464 != 0.0)) || (locals.var_guard465 != 0.0)) || (locals.var_guard466 != 0.0)))) {
        let assign15010_e20789: f64 = (locals.var_nuendd * locals.var_admer);
        let assign15010_e20792: f64 = (locals.var_nuintd * locals.var_adsha);
        let assign15010_e20793: f64 = (assign15010_e20789 + assign15010_e20792);
        (assign15010_e20793, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp_adeff, locals.var_temp_adeff_dn3, locals.var_temp_adeff_dn4, locals.var_temp_adeff_dn5, locals.var_temp_adeff_dn6, locals.var_temp_adeff_dn7, locals.var_temp_adeff_dn8, locals.var_temp_adeff_dn9, locals.var_temp_adeff_dn10, locals.var_temp_adeff_dn11,)
    }
};
        locals.var_temp_adeff = assign15010_e20795;
        locals.var_temp_adeff_dn3 = assign15010_e20795_d_n3;
        locals.var_temp_adeff_dn4 = assign15010_e20795_d_n4;
        locals.var_temp_adeff_dn5 = assign15010_e20795_d_n5;
        locals.var_temp_adeff_dn6 = assign15010_e20795_d_n6;
        locals.var_temp_adeff_dn7 = assign15010_e20795_d_n7;
        locals.var_temp_adeff_dn8 = assign15010_e20795_d_n8;
        locals.var_temp_adeff_dn9 = assign15010_e20795_d_n9;
        locals.var_temp_adeff_dn10 = assign15010_e20795_d_n10;
        locals.var_temp_adeff_dn11 = assign15010_e20795_d_n11;
        locals.var_temp_adeff_rv = 0.0;

        let (assign15020_e20824, assign15020_e20824_d_n3, assign15020_e20824_d_n4, assign15020_e20824_d_n5, assign15020_e20824_d_n6, assign15020_e20824_d_n7, assign15020_e20824_d_n8, assign15020_e20824_d_n9, assign15020_e20824_d_n10, assign15020_e20824_d_n11,) = {
    if ((locals.var_guard468 != 0.0) && (!(((((((((locals.var_guard459 != 0.0) || (locals.var_guard460 != 0.0)) || (locals.var_guard461 != 0.0)) || (locals.var_guard462 != 0.0)) || (locals.var_guard463 != 0.0)) || (locals.var_guard464 != 0.0)) || (locals.var_guard465 != 0.0)) || (locals.var_guard466 != 0.0)) || (locals.var_guard467 != 0.0)))) {
        let assign15020_e20819: f64 = (p.p2 - 1.0);
        let assign15020_e20821: f64 = (assign15020_e20819 * locals.var_pssha);
        let assign15020_e20822: f64 = (locals.var_psiso + assign15020_e20821);
        (assign15020_e20822, (locals.var_psiso_dn3 + (assign15020_e20819 * locals.var_pssha_dn3)), (locals.var_psiso_dn4 + (assign15020_e20819 * locals.var_pssha_dn4)), (locals.var_psiso_dn5 + (assign15020_e20819 * locals.var_pssha_dn5)), (locals.var_psiso_dn6 + (assign15020_e20819 * locals.var_pssha_dn6)), (locals.var_psiso_dn7 + (assign15020_e20819 * locals.var_pssha_dn7)), (locals.var_psiso_dn8 + (assign15020_e20819 * locals.var_pssha_dn8)), (locals.var_psiso_dn9 + (assign15020_e20819 * locals.var_pssha_dn9)), (locals.var_psiso_dn10 + (assign15020_e20819 * locals.var_pssha_dn10)), (locals.var_psiso_dn11 + (assign15020_e20819 * locals.var_pssha_dn11)),)
    } else {
        (locals.var_temp_pseff, locals.var_temp_pseff_dn3, locals.var_temp_pseff_dn4, locals.var_temp_pseff_dn5, locals.var_temp_pseff_dn6, locals.var_temp_pseff_dn7, locals.var_temp_pseff_dn8, locals.var_temp_pseff_dn9, locals.var_temp_pseff_dn10, locals.var_temp_pseff_dn11,)
    }
};
        locals.var_temp_pseff = assign15020_e20824;
        locals.var_temp_pseff_dn3 = assign15020_e20824_d_n3;
        locals.var_temp_pseff_dn4 = assign15020_e20824_d_n4;
        locals.var_temp_pseff_dn5 = assign15020_e20824_d_n5;
        locals.var_temp_pseff_dn6 = assign15020_e20824_d_n6;
        locals.var_temp_pseff_dn7 = assign15020_e20824_d_n7;
        locals.var_temp_pseff_dn8 = assign15020_e20824_d_n8;
        locals.var_temp_pseff_dn9 = assign15020_e20824_d_n9;
        locals.var_temp_pseff_dn10 = assign15020_e20824_d_n10;
        locals.var_temp_pseff_dn11 = assign15020_e20824_d_n11;
        locals.var_temp_pseff_rv = 0.0;

        let (assign15030_e20849, assign15030_e20849_d_n3, assign15030_e20849_d_n4, assign15030_e20849_d_n5, assign15030_e20849_d_n6, assign15030_e20849_d_n7, assign15030_e20849_d_n8, assign15030_e20849_d_n9, assign15030_e20849_d_n10, assign15030_e20849_d_n11,) = {
    if ((locals.var_guard468 != 0.0) && (!(((((((((locals.var_guard459 != 0.0) || (locals.var_guard460 != 0.0)) || (locals.var_guard461 != 0.0)) || (locals.var_guard462 != 0.0)) || (locals.var_guard463 != 0.0)) || (locals.var_guard464 != 0.0)) || (locals.var_guard465 != 0.0)) || (locals.var_guard466 != 0.0)) || (locals.var_guard467 != 0.0)))) {
        let assign15030_e20847: f64 = (p.p2 * locals.var_pdsha);
        (assign15030_e20847, (p.p2 * locals.var_pdsha_dn3), (p.p2 * locals.var_pdsha_dn4), (p.p2 * locals.var_pdsha_dn5), (p.p2 * locals.var_pdsha_dn6), (p.p2 * locals.var_pdsha_dn7), (p.p2 * locals.var_pdsha_dn8), (p.p2 * locals.var_pdsha_dn9), (p.p2 * locals.var_pdsha_dn10), (p.p2 * locals.var_pdsha_dn11),)
    } else {
        (locals.var_temp_pdeff, locals.var_temp_pdeff_dn3, locals.var_temp_pdeff_dn4, locals.var_temp_pdeff_dn5, locals.var_temp_pdeff_dn6, locals.var_temp_pdeff_dn7, locals.var_temp_pdeff_dn8, locals.var_temp_pdeff_dn9, locals.var_temp_pdeff_dn10, locals.var_temp_pdeff_dn11,)
    }
};
        locals.var_temp_pdeff = assign15030_e20849;
        locals.var_temp_pdeff_dn3 = assign15030_e20849_d_n3;
        locals.var_temp_pdeff_dn4 = assign15030_e20849_d_n4;
        locals.var_temp_pdeff_dn5 = assign15030_e20849_d_n5;
        locals.var_temp_pdeff_dn6 = assign15030_e20849_d_n6;
        locals.var_temp_pdeff_dn7 = assign15030_e20849_d_n7;
        locals.var_temp_pdeff_dn8 = assign15030_e20849_d_n8;
        locals.var_temp_pdeff_dn9 = assign15030_e20849_d_n9;
        locals.var_temp_pdeff_dn10 = assign15030_e20849_d_n10;
        locals.var_temp_pdeff_dn11 = assign15030_e20849_d_n11;
        locals.var_temp_pdeff_rv = 0.0;

        let (assign15040_e20878, assign15040_e20878_d_n3, assign15040_e20878_d_n4, assign15040_e20878_d_n5, assign15040_e20878_d_n6, assign15040_e20878_d_n7, assign15040_e20878_d_n8, assign15040_e20878_d_n9, assign15040_e20878_d_n10, assign15040_e20878_d_n11,) = {
    if ((locals.var_guard468 != 0.0) && (!(((((((((locals.var_guard459 != 0.0) || (locals.var_guard460 != 0.0)) || (locals.var_guard461 != 0.0)) || (locals.var_guard462 != 0.0)) || (locals.var_guard463 != 0.0)) || (locals.var_guard464 != 0.0)) || (locals.var_guard465 != 0.0)) || (locals.var_guard466 != 0.0)) || (locals.var_guard467 != 0.0)))) {
        let assign15040_e20873: f64 = (p.p2 - 1.0);
        let assign15040_e20875: f64 = (assign15040_e20873 * locals.var_assha);
        let assign15040_e20876: f64 = (locals.var_asiso + assign15040_e20875);
        (assign15040_e20876, locals.var_asiso_dn3, locals.var_asiso_dn4, locals.var_asiso_dn5, locals.var_asiso_dn6, locals.var_asiso_dn7, locals.var_asiso_dn8, locals.var_asiso_dn9, locals.var_asiso_dn10, locals.var_asiso_dn11,)
    } else {
        (locals.var_temp_aseff, locals.var_temp_aseff_dn3, locals.var_temp_aseff_dn4, locals.var_temp_aseff_dn5, locals.var_temp_aseff_dn6, locals.var_temp_aseff_dn7, locals.var_temp_aseff_dn8, locals.var_temp_aseff_dn9, locals.var_temp_aseff_dn10, locals.var_temp_aseff_dn11,)
    }
};
        locals.var_temp_aseff = assign15040_e20878;
        locals.var_temp_aseff_dn3 = assign15040_e20878_d_n3;
        locals.var_temp_aseff_dn4 = assign15040_e20878_d_n4;
        locals.var_temp_aseff_dn5 = assign15040_e20878_d_n5;
        locals.var_temp_aseff_dn6 = assign15040_e20878_d_n6;
        locals.var_temp_aseff_dn7 = assign15040_e20878_d_n7;
        locals.var_temp_aseff_dn8 = assign15040_e20878_d_n8;
        locals.var_temp_aseff_dn9 = assign15040_e20878_d_n9;
        locals.var_temp_aseff_dn10 = assign15040_e20878_d_n10;
        locals.var_temp_aseff_dn11 = assign15040_e20878_d_n11;
        locals.var_temp_aseff_rv = 0.0;

        let (assign15050_e20903, assign15050_e20903_d_n3, assign15050_e20903_d_n4, assign15050_e20903_d_n5, assign15050_e20903_d_n6, assign15050_e20903_d_n7, assign15050_e20903_d_n8, assign15050_e20903_d_n9, assign15050_e20903_d_n10, assign15050_e20903_d_n11,) = {
    if ((locals.var_guard468 != 0.0) && (!(((((((((locals.var_guard459 != 0.0) || (locals.var_guard460 != 0.0)) || (locals.var_guard461 != 0.0)) || (locals.var_guard462 != 0.0)) || (locals.var_guard463 != 0.0)) || (locals.var_guard464 != 0.0)) || (locals.var_guard465 != 0.0)) || (locals.var_guard466 != 0.0)) || (locals.var_guard467 != 0.0)))) {
        let assign15050_e20901: f64 = (p.p2 * locals.var_adsha);
        (assign15050_e20901, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp_adeff, locals.var_temp_adeff_dn3, locals.var_temp_adeff_dn4, locals.var_temp_adeff_dn5, locals.var_temp_adeff_dn6, locals.var_temp_adeff_dn7, locals.var_temp_adeff_dn8, locals.var_temp_adeff_dn9, locals.var_temp_adeff_dn10, locals.var_temp_adeff_dn11,)
    }
};
        locals.var_temp_adeff = assign15050_e20903;
        locals.var_temp_adeff_dn3 = assign15050_e20903_d_n3;
        locals.var_temp_adeff_dn4 = assign15050_e20903_d_n4;
        locals.var_temp_adeff_dn5 = assign15050_e20903_d_n5;
        locals.var_temp_adeff_dn6 = assign15050_e20903_d_n6;
        locals.var_temp_adeff_dn7 = assign15050_e20903_d_n7;
        locals.var_temp_adeff_dn8 = assign15050_e20903_d_n8;
        locals.var_temp_adeff_dn9 = assign15050_e20903_d_n9;
        locals.var_temp_adeff_dn10 = assign15050_e20903_d_n10;
        locals.var_temp_adeff_dn11 = assign15050_e20903_d_n11;
        locals.var_temp_adeff_rv = 0.0;

        let (assign15060_e20930, assign15060_e20930_d_n3, assign15060_e20930_d_n4, assign15060_e20930_d_n5, assign15060_e20930_d_n6, assign15060_e20930_d_n7, assign15060_e20930_d_n8, assign15060_e20930_d_n9, assign15060_e20930_d_n10, assign15060_e20930_d_n11,) = {
    if ((locals.var_guard469 != 0.0) && (!((((((((((locals.var_guard459 != 0.0) || (locals.var_guard460 != 0.0)) || (locals.var_guard461 != 0.0)) || (locals.var_guard462 != 0.0)) || (locals.var_guard463 != 0.0)) || (locals.var_guard464 != 0.0)) || (locals.var_guard465 != 0.0)) || (locals.var_guard466 != 0.0)) || (locals.var_guard467 != 0.0)) || (locals.var_guard468 != 0.0)))) {
        let assign15060_e20928: f64 = (p.p2 * locals.var_pssha);
        (assign15060_e20928, (p.p2 * locals.var_pssha_dn3), (p.p2 * locals.var_pssha_dn4), (p.p2 * locals.var_pssha_dn5), (p.p2 * locals.var_pssha_dn6), (p.p2 * locals.var_pssha_dn7), (p.p2 * locals.var_pssha_dn8), (p.p2 * locals.var_pssha_dn9), (p.p2 * locals.var_pssha_dn10), (p.p2 * locals.var_pssha_dn11),)
    } else {
        (locals.var_temp_pseff, locals.var_temp_pseff_dn3, locals.var_temp_pseff_dn4, locals.var_temp_pseff_dn5, locals.var_temp_pseff_dn6, locals.var_temp_pseff_dn7, locals.var_temp_pseff_dn8, locals.var_temp_pseff_dn9, locals.var_temp_pseff_dn10, locals.var_temp_pseff_dn11,)
    }
};
        locals.var_temp_pseff = assign15060_e20930;
        locals.var_temp_pseff_dn3 = assign15060_e20930_d_n3;
        locals.var_temp_pseff_dn4 = assign15060_e20930_d_n4;
        locals.var_temp_pseff_dn5 = assign15060_e20930_d_n5;
        locals.var_temp_pseff_dn6 = assign15060_e20930_d_n6;
        locals.var_temp_pseff_dn7 = assign15060_e20930_d_n7;
        locals.var_temp_pseff_dn8 = assign15060_e20930_d_n8;
        locals.var_temp_pseff_dn9 = assign15060_e20930_d_n9;
        locals.var_temp_pseff_dn10 = assign15060_e20930_d_n10;
        locals.var_temp_pseff_dn11 = assign15060_e20930_d_n11;
        locals.var_temp_pseff_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_27(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        locals: &mut StampLocals,
    ) {
        let (assign15070_e20961, assign15070_e20961_d_n3, assign15070_e20961_d_n4, assign15070_e20961_d_n5, assign15070_e20961_d_n6, assign15070_e20961_d_n7, assign15070_e20961_d_n8, assign15070_e20961_d_n9, assign15070_e20961_d_n10, assign15070_e20961_d_n11,) = {
    if ((locals.var_guard469 != 0.0) && (!((((((((((locals.var_guard459 != 0.0) || (locals.var_guard460 != 0.0)) || (locals.var_guard461 != 0.0)) || (locals.var_guard462 != 0.0)) || (locals.var_guard463 != 0.0)) || (locals.var_guard464 != 0.0)) || (locals.var_guard465 != 0.0)) || (locals.var_guard466 != 0.0)) || (locals.var_guard467 != 0.0)) || (locals.var_guard468 != 0.0)))) {
        let assign15070_e20956: f64 = (p.p2 - 1.0);
        let assign15070_e20958: f64 = (assign15070_e20956 * locals.var_pdsha);
        let assign15070_e20959: f64 = (locals.var_pdiso + assign15070_e20958);
        (assign15070_e20959, (locals.var_pdiso_dn3 + (assign15070_e20956 * locals.var_pdsha_dn3)), (locals.var_pdiso_dn4 + (assign15070_e20956 * locals.var_pdsha_dn4)), (locals.var_pdiso_dn5 + (assign15070_e20956 * locals.var_pdsha_dn5)), (locals.var_pdiso_dn6 + (assign15070_e20956 * locals.var_pdsha_dn6)), (locals.var_pdiso_dn7 + (assign15070_e20956 * locals.var_pdsha_dn7)), (locals.var_pdiso_dn8 + (assign15070_e20956 * locals.var_pdsha_dn8)), (locals.var_pdiso_dn9 + (assign15070_e20956 * locals.var_pdsha_dn9)), (locals.var_pdiso_dn10 + (assign15070_e20956 * locals.var_pdsha_dn10)), (locals.var_pdiso_dn11 + (assign15070_e20956 * locals.var_pdsha_dn11)),)
    } else {
        (locals.var_temp_pdeff, locals.var_temp_pdeff_dn3, locals.var_temp_pdeff_dn4, locals.var_temp_pdeff_dn5, locals.var_temp_pdeff_dn6, locals.var_temp_pdeff_dn7, locals.var_temp_pdeff_dn8, locals.var_temp_pdeff_dn9, locals.var_temp_pdeff_dn10, locals.var_temp_pdeff_dn11,)
    }
};
        locals.var_temp_pdeff = assign15070_e20961;
        locals.var_temp_pdeff_dn3 = assign15070_e20961_d_n3;
        locals.var_temp_pdeff_dn4 = assign15070_e20961_d_n4;
        locals.var_temp_pdeff_dn5 = assign15070_e20961_d_n5;
        locals.var_temp_pdeff_dn6 = assign15070_e20961_d_n6;
        locals.var_temp_pdeff_dn7 = assign15070_e20961_d_n7;
        locals.var_temp_pdeff_dn8 = assign15070_e20961_d_n8;
        locals.var_temp_pdeff_dn9 = assign15070_e20961_d_n9;
        locals.var_temp_pdeff_dn10 = assign15070_e20961_d_n10;
        locals.var_temp_pdeff_dn11 = assign15070_e20961_d_n11;
        locals.var_temp_pdeff_rv = 0.0;

        let (assign15080_e20988, assign15080_e20988_d_n3, assign15080_e20988_d_n4, assign15080_e20988_d_n5, assign15080_e20988_d_n6, assign15080_e20988_d_n7, assign15080_e20988_d_n8, assign15080_e20988_d_n9, assign15080_e20988_d_n10, assign15080_e20988_d_n11,) = {
    if ((locals.var_guard469 != 0.0) && (!((((((((((locals.var_guard459 != 0.0) || (locals.var_guard460 != 0.0)) || (locals.var_guard461 != 0.0)) || (locals.var_guard462 != 0.0)) || (locals.var_guard463 != 0.0)) || (locals.var_guard464 != 0.0)) || (locals.var_guard465 != 0.0)) || (locals.var_guard466 != 0.0)) || (locals.var_guard467 != 0.0)) || (locals.var_guard468 != 0.0)))) {
        let assign15080_e20986: f64 = (p.p2 * locals.var_assha);
        (assign15080_e20986, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp_aseff, locals.var_temp_aseff_dn3, locals.var_temp_aseff_dn4, locals.var_temp_aseff_dn5, locals.var_temp_aseff_dn6, locals.var_temp_aseff_dn7, locals.var_temp_aseff_dn8, locals.var_temp_aseff_dn9, locals.var_temp_aseff_dn10, locals.var_temp_aseff_dn11,)
    }
};
        locals.var_temp_aseff = assign15080_e20988;
        locals.var_temp_aseff_dn3 = assign15080_e20988_d_n3;
        locals.var_temp_aseff_dn4 = assign15080_e20988_d_n4;
        locals.var_temp_aseff_dn5 = assign15080_e20988_d_n5;
        locals.var_temp_aseff_dn6 = assign15080_e20988_d_n6;
        locals.var_temp_aseff_dn7 = assign15080_e20988_d_n7;
        locals.var_temp_aseff_dn8 = assign15080_e20988_d_n8;
        locals.var_temp_aseff_dn9 = assign15080_e20988_d_n9;
        locals.var_temp_aseff_dn10 = assign15080_e20988_d_n10;
        locals.var_temp_aseff_dn11 = assign15080_e20988_d_n11;
        locals.var_temp_aseff_rv = 0.0;

        let (assign15090_e21019, assign15090_e21019_d_n3, assign15090_e21019_d_n4, assign15090_e21019_d_n5, assign15090_e21019_d_n6, assign15090_e21019_d_n7, assign15090_e21019_d_n8, assign15090_e21019_d_n9, assign15090_e21019_d_n10, assign15090_e21019_d_n11,) = {
    if ((locals.var_guard469 != 0.0) && (!((((((((((locals.var_guard459 != 0.0) || (locals.var_guard460 != 0.0)) || (locals.var_guard461 != 0.0)) || (locals.var_guard462 != 0.0)) || (locals.var_guard463 != 0.0)) || (locals.var_guard464 != 0.0)) || (locals.var_guard465 != 0.0)) || (locals.var_guard466 != 0.0)) || (locals.var_guard467 != 0.0)) || (locals.var_guard468 != 0.0)))) {
        let assign15090_e21014: f64 = (p.p2 - 1.0);
        let assign15090_e21016: f64 = (assign15090_e21014 * locals.var_adsha);
        let assign15090_e21017: f64 = (locals.var_adiso + assign15090_e21016);
        (assign15090_e21017, locals.var_adiso_dn3, locals.var_adiso_dn4, locals.var_adiso_dn5, locals.var_adiso_dn6, locals.var_adiso_dn7, locals.var_adiso_dn8, locals.var_adiso_dn9, locals.var_adiso_dn10, locals.var_adiso_dn11,)
    } else {
        (locals.var_temp_adeff, locals.var_temp_adeff_dn3, locals.var_temp_adeff_dn4, locals.var_temp_adeff_dn5, locals.var_temp_adeff_dn6, locals.var_temp_adeff_dn7, locals.var_temp_adeff_dn8, locals.var_temp_adeff_dn9, locals.var_temp_adeff_dn10, locals.var_temp_adeff_dn11,)
    }
};
        locals.var_temp_adeff = assign15090_e21019;
        locals.var_temp_adeff_dn3 = assign15090_e21019_d_n3;
        locals.var_temp_adeff_dn4 = assign15090_e21019_d_n4;
        locals.var_temp_adeff_dn5 = assign15090_e21019_d_n5;
        locals.var_temp_adeff_dn6 = assign15090_e21019_d_n6;
        locals.var_temp_adeff_dn7 = assign15090_e21019_d_n7;
        locals.var_temp_adeff_dn8 = assign15090_e21019_d_n8;
        locals.var_temp_adeff_dn9 = assign15090_e21019_d_n9;
        locals.var_temp_adeff_dn10 = assign15090_e21019_d_n10;
        locals.var_temp_adeff_dn11 = assign15090_e21019_d_n11;
        locals.var_temp_adeff_rv = 0.0;

        let (assign15100_e21044, assign15100_e21044_d_n3, assign15100_e21044_d_n4, assign15100_e21044_d_n5, assign15100_e21044_d_n6, assign15100_e21044_d_n7, assign15100_e21044_d_n8, assign15100_e21044_d_n9, assign15100_e21044_d_n10, assign15100_e21044_d_n11,) = {
    if (!(((((((((((locals.var_guard459 != 0.0) || (locals.var_guard460 != 0.0)) || (locals.var_guard461 != 0.0)) || (locals.var_guard462 != 0.0)) || (locals.var_guard463 != 0.0)) || (locals.var_guard464 != 0.0)) || (locals.var_guard465 != 0.0)) || (locals.var_guard466 != 0.0)) || (locals.var_guard467 != 0.0)) || (locals.var_guard468 != 0.0)) || (locals.var_guard469 != 0.0))) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp_pseff, locals.var_temp_pseff_dn3, locals.var_temp_pseff_dn4, locals.var_temp_pseff_dn5, locals.var_temp_pseff_dn6, locals.var_temp_pseff_dn7, locals.var_temp_pseff_dn8, locals.var_temp_pseff_dn9, locals.var_temp_pseff_dn10, locals.var_temp_pseff_dn11,)
    }
};
        locals.var_temp_pseff = assign15100_e21044;
        locals.var_temp_pseff_dn3 = assign15100_e21044_d_n3;
        locals.var_temp_pseff_dn4 = assign15100_e21044_d_n4;
        locals.var_temp_pseff_dn5 = assign15100_e21044_d_n5;
        locals.var_temp_pseff_dn6 = assign15100_e21044_d_n6;
        locals.var_temp_pseff_dn7 = assign15100_e21044_d_n7;
        locals.var_temp_pseff_dn8 = assign15100_e21044_d_n8;
        locals.var_temp_pseff_dn9 = assign15100_e21044_d_n9;
        locals.var_temp_pseff_dn10 = assign15100_e21044_d_n10;
        locals.var_temp_pseff_dn11 = assign15100_e21044_d_n11;
        locals.var_temp_pseff_rv = 0.0;

        let (assign15110_e21069, assign15110_e21069_d_n3, assign15110_e21069_d_n4, assign15110_e21069_d_n5, assign15110_e21069_d_n6, assign15110_e21069_d_n7, assign15110_e21069_d_n8, assign15110_e21069_d_n9, assign15110_e21069_d_n10, assign15110_e21069_d_n11,) = {
    if (!(((((((((((locals.var_guard459 != 0.0) || (locals.var_guard460 != 0.0)) || (locals.var_guard461 != 0.0)) || (locals.var_guard462 != 0.0)) || (locals.var_guard463 != 0.0)) || (locals.var_guard464 != 0.0)) || (locals.var_guard465 != 0.0)) || (locals.var_guard466 != 0.0)) || (locals.var_guard467 != 0.0)) || (locals.var_guard468 != 0.0)) || (locals.var_guard469 != 0.0))) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp_pdeff, locals.var_temp_pdeff_dn3, locals.var_temp_pdeff_dn4, locals.var_temp_pdeff_dn5, locals.var_temp_pdeff_dn6, locals.var_temp_pdeff_dn7, locals.var_temp_pdeff_dn8, locals.var_temp_pdeff_dn9, locals.var_temp_pdeff_dn10, locals.var_temp_pdeff_dn11,)
    }
};
        locals.var_temp_pdeff = assign15110_e21069;
        locals.var_temp_pdeff_dn3 = assign15110_e21069_d_n3;
        locals.var_temp_pdeff_dn4 = assign15110_e21069_d_n4;
        locals.var_temp_pdeff_dn5 = assign15110_e21069_d_n5;
        locals.var_temp_pdeff_dn6 = assign15110_e21069_d_n6;
        locals.var_temp_pdeff_dn7 = assign15110_e21069_d_n7;
        locals.var_temp_pdeff_dn8 = assign15110_e21069_d_n8;
        locals.var_temp_pdeff_dn9 = assign15110_e21069_d_n9;
        locals.var_temp_pdeff_dn10 = assign15110_e21069_d_n10;
        locals.var_temp_pdeff_dn11 = assign15110_e21069_d_n11;
        locals.var_temp_pdeff_rv = 0.0;

        let (assign15120_e21094, assign15120_e21094_d_n3, assign15120_e21094_d_n4, assign15120_e21094_d_n5, assign15120_e21094_d_n6, assign15120_e21094_d_n7, assign15120_e21094_d_n8, assign15120_e21094_d_n9, assign15120_e21094_d_n10, assign15120_e21094_d_n11,) = {
    if (!(((((((((((locals.var_guard459 != 0.0) || (locals.var_guard460 != 0.0)) || (locals.var_guard461 != 0.0)) || (locals.var_guard462 != 0.0)) || (locals.var_guard463 != 0.0)) || (locals.var_guard464 != 0.0)) || (locals.var_guard465 != 0.0)) || (locals.var_guard466 != 0.0)) || (locals.var_guard467 != 0.0)) || (locals.var_guard468 != 0.0)) || (locals.var_guard469 != 0.0))) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp_aseff, locals.var_temp_aseff_dn3, locals.var_temp_aseff_dn4, locals.var_temp_aseff_dn5, locals.var_temp_aseff_dn6, locals.var_temp_aseff_dn7, locals.var_temp_aseff_dn8, locals.var_temp_aseff_dn9, locals.var_temp_aseff_dn10, locals.var_temp_aseff_dn11,)
    }
};
        locals.var_temp_aseff = assign15120_e21094;
        locals.var_temp_aseff_dn3 = assign15120_e21094_d_n3;
        locals.var_temp_aseff_dn4 = assign15120_e21094_d_n4;
        locals.var_temp_aseff_dn5 = assign15120_e21094_d_n5;
        locals.var_temp_aseff_dn6 = assign15120_e21094_d_n6;
        locals.var_temp_aseff_dn7 = assign15120_e21094_d_n7;
        locals.var_temp_aseff_dn8 = assign15120_e21094_d_n8;
        locals.var_temp_aseff_dn9 = assign15120_e21094_d_n9;
        locals.var_temp_aseff_dn10 = assign15120_e21094_d_n10;
        locals.var_temp_aseff_dn11 = assign15120_e21094_d_n11;
        locals.var_temp_aseff_rv = 0.0;

        let (assign15130_e21119, assign15130_e21119_d_n3, assign15130_e21119_d_n4, assign15130_e21119_d_n5, assign15130_e21119_d_n6, assign15130_e21119_d_n7, assign15130_e21119_d_n8, assign15130_e21119_d_n9, assign15130_e21119_d_n10, assign15130_e21119_d_n11,) = {
    if (!(((((((((((locals.var_guard459 != 0.0) || (locals.var_guard460 != 0.0)) || (locals.var_guard461 != 0.0)) || (locals.var_guard462 != 0.0)) || (locals.var_guard463 != 0.0)) || (locals.var_guard464 != 0.0)) || (locals.var_guard465 != 0.0)) || (locals.var_guard466 != 0.0)) || (locals.var_guard467 != 0.0)) || (locals.var_guard468 != 0.0)) || (locals.var_guard469 != 0.0))) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp_adeff, locals.var_temp_adeff_dn3, locals.var_temp_adeff_dn4, locals.var_temp_adeff_dn5, locals.var_temp_adeff_dn6, locals.var_temp_adeff_dn7, locals.var_temp_adeff_dn8, locals.var_temp_adeff_dn9, locals.var_temp_adeff_dn10, locals.var_temp_adeff_dn11,)
    }
};
        locals.var_temp_adeff = assign15130_e21119;
        locals.var_temp_adeff_dn3 = assign15130_e21119_d_n3;
        locals.var_temp_adeff_dn4 = assign15130_e21119_d_n4;
        locals.var_temp_adeff_dn5 = assign15130_e21119_d_n5;
        locals.var_temp_adeff_dn6 = assign15130_e21119_d_n6;
        locals.var_temp_adeff_dn7 = assign15130_e21119_d_n7;
        locals.var_temp_adeff_dn8 = assign15130_e21119_d_n8;
        locals.var_temp_adeff_dn9 = assign15130_e21119_d_n9;
        locals.var_temp_adeff_dn10 = assign15130_e21119_d_n10;
        locals.var_temp_adeff_dn11 = assign15130_e21119_d_n11;
        locals.var_temp_adeff_rv = 0.0;

        let assign15140_e21121: f64 = if param_given[17] { 1.0 } else { 0.0 };
        locals.var_guard470 = assign15140_e21121;
        locals.var_guard470_rv = 0.0;

        let (assign15150_e21129, assign15150_e21129_d_n3, assign15150_e21129_d_n4, assign15150_e21129_d_n5, assign15150_e21129_d_n6, assign15150_e21129_d_n7, assign15150_e21129_d_n8, assign15150_e21129_d_n9, assign15150_e21129_d_n10, assign15150_e21129_d_n11,) = {
    if (locals.var_guard470 != 0.0) {
        let assign15150_e21125: f64 = (p.p17 * p.p50);
        let assign15150_e21127: f64 = (assign15150_e21125 * p.p49);
        (assign15150_e21127, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_aseff, locals.var_aseff_dn3, locals.var_aseff_dn4, locals.var_aseff_dn5, locals.var_aseff_dn6, locals.var_aseff_dn7, locals.var_aseff_dn8, locals.var_aseff_dn9, locals.var_aseff_dn10, locals.var_aseff_dn11,)
    }
};
        locals.var_aseff = assign15150_e21129;
        locals.var_aseff_dn3 = assign15150_e21129_d_n3;
        locals.var_aseff_dn4 = assign15150_e21129_d_n4;
        locals.var_aseff_dn5 = assign15150_e21129_d_n5;
        locals.var_aseff_dn6 = assign15150_e21129_d_n6;
        locals.var_aseff_dn7 = assign15150_e21129_d_n7;
        locals.var_aseff_dn8 = assign15150_e21129_d_n8;
        locals.var_aseff_dn9 = assign15150_e21129_d_n9;
        locals.var_aseff_dn10 = assign15150_e21129_d_n10;
        locals.var_aseff_dn11 = assign15150_e21129_d_n11;
        locals.var_aseff_rv = 0.0;

        let (assign15160_e21134, assign15160_e21134_d_n3, assign15160_e21134_d_n4, assign15160_e21134_d_n5, assign15160_e21134_d_n6, assign15160_e21134_d_n7, assign15160_e21134_d_n8, assign15160_e21134_d_n9, assign15160_e21134_d_n10, assign15160_e21134_d_n11,) = {
    if (locals.var_guard470 == 0.0) {
        (locals.var_temp_aseff, locals.var_temp_aseff_dn3, locals.var_temp_aseff_dn4, locals.var_temp_aseff_dn5, locals.var_temp_aseff_dn6, locals.var_temp_aseff_dn7, locals.var_temp_aseff_dn8, locals.var_temp_aseff_dn9, locals.var_temp_aseff_dn10, locals.var_temp_aseff_dn11,)
    } else {
        (locals.var_aseff, locals.var_aseff_dn3, locals.var_aseff_dn4, locals.var_aseff_dn5, locals.var_aseff_dn6, locals.var_aseff_dn7, locals.var_aseff_dn8, locals.var_aseff_dn9, locals.var_aseff_dn10, locals.var_aseff_dn11,)
    }
};
        locals.var_aseff = assign15160_e21134;
        locals.var_aseff_dn3 = assign15160_e21134_d_n3;
        locals.var_aseff_dn4 = assign15160_e21134_d_n4;
        locals.var_aseff_dn5 = assign15160_e21134_d_n5;
        locals.var_aseff_dn6 = assign15160_e21134_d_n6;
        locals.var_aseff_dn7 = assign15160_e21134_d_n7;
        locals.var_aseff_dn8 = assign15160_e21134_d_n8;
        locals.var_aseff_dn9 = assign15160_e21134_d_n9;
        locals.var_aseff_dn10 = assign15160_e21134_d_n10;
        locals.var_aseff_dn11 = assign15160_e21134_d_n11;
        locals.var_aseff_rv = 0.0;

        let assign15170_e21137: f64 = if locals.var_aseff < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard471 = assign15170_e21137;
        locals.var_guard471_rv = 0.0;

        let (assign15180_e21141, assign15180_e21141_d_n3, assign15180_e21141_d_n4, assign15180_e21141_d_n5, assign15180_e21141_d_n6, assign15180_e21141_d_n7, assign15180_e21141_d_n8, assign15180_e21141_d_n9, assign15180_e21141_d_n10, assign15180_e21141_d_n11,) = {
    if (locals.var_guard471 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_aseff, locals.var_aseff_dn3, locals.var_aseff_dn4, locals.var_aseff_dn5, locals.var_aseff_dn6, locals.var_aseff_dn7, locals.var_aseff_dn8, locals.var_aseff_dn9, locals.var_aseff_dn10, locals.var_aseff_dn11,)
    }
};
        locals.var_aseff = assign15180_e21141;
        locals.var_aseff_dn3 = assign15180_e21141_d_n3;
        locals.var_aseff_dn4 = assign15180_e21141_d_n4;
        locals.var_aseff_dn5 = assign15180_e21141_d_n5;
        locals.var_aseff_dn6 = assign15180_e21141_d_n6;
        locals.var_aseff_dn7 = assign15180_e21141_d_n7;
        locals.var_aseff_dn8 = assign15180_e21141_d_n8;
        locals.var_aseff_dn9 = assign15180_e21141_d_n9;
        locals.var_aseff_dn10 = assign15180_e21141_d_n10;
        locals.var_aseff_dn11 = assign15180_e21141_d_n11;
        locals.var_aseff_rv = 0.0;

        let assign15190_e21143: f64 = if param_given[18] { 1.0 } else { 0.0 };
        locals.var_guard472 = assign15190_e21143;
        locals.var_guard472_rv = 0.0;

        let (assign15200_e21151, assign15200_e21151_d_n3, assign15200_e21151_d_n4, assign15200_e21151_d_n5, assign15200_e21151_d_n6, assign15200_e21151_d_n7, assign15200_e21151_d_n8, assign15200_e21151_d_n9, assign15200_e21151_d_n10, assign15200_e21151_d_n11,) = {
    if (locals.var_guard472 != 0.0) {
        let assign15200_e21147: f64 = (p.p18 * p.p50);
        let assign15200_e21149: f64 = (assign15200_e21147 * p.p49);
        (assign15200_e21149, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_adeff, locals.var_adeff_dn3, locals.var_adeff_dn4, locals.var_adeff_dn5, locals.var_adeff_dn6, locals.var_adeff_dn7, locals.var_adeff_dn8, locals.var_adeff_dn9, locals.var_adeff_dn10, locals.var_adeff_dn11,)
    }
};
        locals.var_adeff = assign15200_e21151;
        locals.var_adeff_dn3 = assign15200_e21151_d_n3;
        locals.var_adeff_dn4 = assign15200_e21151_d_n4;
        locals.var_adeff_dn5 = assign15200_e21151_d_n5;
        locals.var_adeff_dn6 = assign15200_e21151_d_n6;
        locals.var_adeff_dn7 = assign15200_e21151_d_n7;
        locals.var_adeff_dn8 = assign15200_e21151_d_n8;
        locals.var_adeff_dn9 = assign15200_e21151_d_n9;
        locals.var_adeff_dn10 = assign15200_e21151_d_n10;
        locals.var_adeff_dn11 = assign15200_e21151_d_n11;
        locals.var_adeff_rv = 0.0;

        let (assign15210_e21156, assign15210_e21156_d_n3, assign15210_e21156_d_n4, assign15210_e21156_d_n5, assign15210_e21156_d_n6, assign15210_e21156_d_n7, assign15210_e21156_d_n8, assign15210_e21156_d_n9, assign15210_e21156_d_n10, assign15210_e21156_d_n11,) = {
    if (locals.var_guard472 == 0.0) {
        (locals.var_temp_adeff, locals.var_temp_adeff_dn3, locals.var_temp_adeff_dn4, locals.var_temp_adeff_dn5, locals.var_temp_adeff_dn6, locals.var_temp_adeff_dn7, locals.var_temp_adeff_dn8, locals.var_temp_adeff_dn9, locals.var_temp_adeff_dn10, locals.var_temp_adeff_dn11,)
    } else {
        (locals.var_adeff, locals.var_adeff_dn3, locals.var_adeff_dn4, locals.var_adeff_dn5, locals.var_adeff_dn6, locals.var_adeff_dn7, locals.var_adeff_dn8, locals.var_adeff_dn9, locals.var_adeff_dn10, locals.var_adeff_dn11,)
    }
};
        locals.var_adeff = assign15210_e21156;
        locals.var_adeff_dn3 = assign15210_e21156_d_n3;
        locals.var_adeff_dn4 = assign15210_e21156_d_n4;
        locals.var_adeff_dn5 = assign15210_e21156_d_n5;
        locals.var_adeff_dn6 = assign15210_e21156_d_n6;
        locals.var_adeff_dn7 = assign15210_e21156_d_n7;
        locals.var_adeff_dn8 = assign15210_e21156_d_n8;
        locals.var_adeff_dn9 = assign15210_e21156_d_n9;
        locals.var_adeff_dn10 = assign15210_e21156_d_n10;
        locals.var_adeff_dn11 = assign15210_e21156_d_n11;
        locals.var_adeff_rv = 0.0;

        let assign15220_e21159: f64 = if locals.var_adeff < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard473 = assign15220_e21159;
        locals.var_guard473_rv = 0.0;

        let (assign15230_e21163, assign15230_e21163_d_n3, assign15230_e21163_d_n4, assign15230_e21163_d_n5, assign15230_e21163_d_n6, assign15230_e21163_d_n7, assign15230_e21163_d_n8, assign15230_e21163_d_n9, assign15230_e21163_d_n10, assign15230_e21163_d_n11,) = {
    if (locals.var_guard473 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_adeff, locals.var_adeff_dn3, locals.var_adeff_dn4, locals.var_adeff_dn5, locals.var_adeff_dn6, locals.var_adeff_dn7, locals.var_adeff_dn8, locals.var_adeff_dn9, locals.var_adeff_dn10, locals.var_adeff_dn11,)
    }
};
        locals.var_adeff = assign15230_e21163;
        locals.var_adeff_dn3 = assign15230_e21163_d_n3;
        locals.var_adeff_dn4 = assign15230_e21163_d_n4;
        locals.var_adeff_dn5 = assign15230_e21163_d_n5;
        locals.var_adeff_dn6 = assign15230_e21163_d_n6;
        locals.var_adeff_dn7 = assign15230_e21163_d_n7;
        locals.var_adeff_dn8 = assign15230_e21163_d_n8;
        locals.var_adeff_dn9 = assign15230_e21163_d_n9;
        locals.var_adeff_dn10 = assign15230_e21163_d_n10;
        locals.var_adeff_dn11 = assign15230_e21163_d_n11;
        locals.var_adeff_rv = 0.0;

        let assign15240_e21165: f64 = if param_given[19] { 1.0 } else { 0.0 };
        locals.var_guard474 = assign15240_e21165;
        locals.var_guard474_rv = 0.0;

        let assign15250_e21168: f64 = if p.p926 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard475 = assign15250_e21168;
        locals.var_guard475_rv = 0.0;

        let (assign15260_e21176, assign15260_e21176_d_n3, assign15260_e21176_d_n4, assign15260_e21176_d_n5, assign15260_e21176_d_n6, assign15260_e21176_d_n7, assign15260_e21176_d_n8, assign15260_e21176_d_n9, assign15260_e21176_d_n10, assign15260_e21176_d_n11,) = {
    if ((locals.var_guard474 != 0.0) && (locals.var_guard475 != 0.0)) {
        let assign15260_e21174: f64 = (p.p19 * p.p50);
        (assign15260_e21174, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pseff, locals.var_pseff_dn3, locals.var_pseff_dn4, locals.var_pseff_dn5, locals.var_pseff_dn6, locals.var_pseff_dn7, locals.var_pseff_dn8, locals.var_pseff_dn9, locals.var_pseff_dn10, locals.var_pseff_dn11,)
    }
};
        locals.var_pseff = assign15260_e21176;
        locals.var_pseff_dn3 = assign15260_e21176_d_n3;
        locals.var_pseff_dn4 = assign15260_e21176_d_n4;
        locals.var_pseff_dn5 = assign15260_e21176_d_n5;
        locals.var_pseff_dn6 = assign15260_e21176_d_n6;
        locals.var_pseff_dn7 = assign15260_e21176_d_n7;
        locals.var_pseff_dn8 = assign15260_e21176_d_n8;
        locals.var_pseff_dn9 = assign15260_e21176_d_n9;
        locals.var_pseff_dn10 = assign15260_e21176_d_n10;
        locals.var_pseff_dn11 = assign15260_e21176_d_n11;
        locals.var_pseff_rv = 0.0;

        let (assign15270_e21191, assign15270_e21191_d_n3, assign15270_e21191_d_n4, assign15270_e21191_d_n5, assign15270_e21191_d_n6, assign15270_e21191_d_n7, assign15270_e21191_d_n8, assign15270_e21191_d_n9, assign15270_e21191_d_n10, assign15270_e21191_d_n11,) = {
    if ((locals.var_guard474 != 0.0) && (locals.var_guard475 == 0.0)) {
        let assign15270_e21183: f64 = (p.p19 * p.p50);
        let assign15270_e21186: f64 = (locals.var_weffcj * p.p2);
        let assign15270_e21187: f64 = (assign15270_e21183 - assign15270_e21186);
        let assign15270_e21189: f64 = (assign15270_e21187).max(0.0);
        (assign15270_e21189, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pseff, locals.var_pseff_dn3, locals.var_pseff_dn4, locals.var_pseff_dn5, locals.var_pseff_dn6, locals.var_pseff_dn7, locals.var_pseff_dn8, locals.var_pseff_dn9, locals.var_pseff_dn10, locals.var_pseff_dn11,)
    }
};
        locals.var_pseff = assign15270_e21191;
        locals.var_pseff_dn3 = assign15270_e21191_d_n3;
        locals.var_pseff_dn4 = assign15270_e21191_d_n4;
        locals.var_pseff_dn5 = assign15270_e21191_d_n5;
        locals.var_pseff_dn6 = assign15270_e21191_d_n6;
        locals.var_pseff_dn7 = assign15270_e21191_d_n7;
        locals.var_pseff_dn8 = assign15270_e21191_d_n8;
        locals.var_pseff_dn9 = assign15270_e21191_d_n9;
        locals.var_pseff_dn10 = assign15270_e21191_d_n10;
        locals.var_pseff_dn11 = assign15270_e21191_d_n11;
        locals.var_pseff_rv = 0.0;

        let (assign15280_e21196, assign15280_e21196_d_n3, assign15280_e21196_d_n4, assign15280_e21196_d_n5, assign15280_e21196_d_n6, assign15280_e21196_d_n7, assign15280_e21196_d_n8, assign15280_e21196_d_n9, assign15280_e21196_d_n10, assign15280_e21196_d_n11,) = {
    if (locals.var_guard474 == 0.0) {
        (locals.var_temp_pseff, locals.var_temp_pseff_dn3, locals.var_temp_pseff_dn4, locals.var_temp_pseff_dn5, locals.var_temp_pseff_dn6, locals.var_temp_pseff_dn7, locals.var_temp_pseff_dn8, locals.var_temp_pseff_dn9, locals.var_temp_pseff_dn10, locals.var_temp_pseff_dn11,)
    } else {
        (locals.var_pseff, locals.var_pseff_dn3, locals.var_pseff_dn4, locals.var_pseff_dn5, locals.var_pseff_dn6, locals.var_pseff_dn7, locals.var_pseff_dn8, locals.var_pseff_dn9, locals.var_pseff_dn10, locals.var_pseff_dn11,)
    }
};
        locals.var_pseff = assign15280_e21196;
        locals.var_pseff_dn3 = assign15280_e21196_d_n3;
        locals.var_pseff_dn4 = assign15280_e21196_d_n4;
        locals.var_pseff_dn5 = assign15280_e21196_d_n5;
        locals.var_pseff_dn6 = assign15280_e21196_d_n6;
        locals.var_pseff_dn7 = assign15280_e21196_d_n7;
        locals.var_pseff_dn8 = assign15280_e21196_d_n8;
        locals.var_pseff_dn9 = assign15280_e21196_d_n9;
        locals.var_pseff_dn10 = assign15280_e21196_d_n10;
        locals.var_pseff_dn11 = assign15280_e21196_d_n11;
        locals.var_pseff_rv = 0.0;

        let assign15290_e21199: f64 = if locals.var_pseff < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard476 = assign15290_e21199;
        locals.var_guard476_rv = 0.0;

        let (assign15300_e21206, assign15300_e21206_d_n3, assign15300_e21206_d_n4, assign15300_e21206_d_n5, assign15300_e21206_d_n6, assign15300_e21206_d_n7, assign15300_e21206_d_n8, assign15300_e21206_d_n9, assign15300_e21206_d_n10, assign15300_e21206_d_n11,) = {
    if ((locals.var_guard474 == 0.0) && (locals.var_guard476 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pseff, locals.var_pseff_dn3, locals.var_pseff_dn4, locals.var_pseff_dn5, locals.var_pseff_dn6, locals.var_pseff_dn7, locals.var_pseff_dn8, locals.var_pseff_dn9, locals.var_pseff_dn10, locals.var_pseff_dn11,)
    }
};
        locals.var_pseff = assign15300_e21206;
        locals.var_pseff_dn3 = assign15300_e21206_d_n3;
        locals.var_pseff_dn4 = assign15300_e21206_d_n4;
        locals.var_pseff_dn5 = assign15300_e21206_d_n5;
        locals.var_pseff_dn6 = assign15300_e21206_d_n6;
        locals.var_pseff_dn7 = assign15300_e21206_d_n7;
        locals.var_pseff_dn8 = assign15300_e21206_d_n8;
        locals.var_pseff_dn9 = assign15300_e21206_d_n9;
        locals.var_pseff_dn10 = assign15300_e21206_d_n10;
        locals.var_pseff_dn11 = assign15300_e21206_d_n11;
        locals.var_pseff_rv = 0.0;

        let assign15310_e21208: f64 = if param_given[20] { 1.0 } else { 0.0 };
        locals.var_guard477 = assign15310_e21208;
        locals.var_guard477_rv = 0.0;

        let assign15320_e21211: f64 = if p.p926 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard478 = assign15320_e21211;
        locals.var_guard478_rv = 0.0;

        let (assign15330_e21219, assign15330_e21219_d_n3, assign15330_e21219_d_n4, assign15330_e21219_d_n5, assign15330_e21219_d_n6, assign15330_e21219_d_n7, assign15330_e21219_d_n8, assign15330_e21219_d_n9, assign15330_e21219_d_n10, assign15330_e21219_d_n11,) = {
    if ((locals.var_guard477 != 0.0) && (locals.var_guard478 != 0.0)) {
        let assign15330_e21217: f64 = (p.p20 * p.p50);
        (assign15330_e21217, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pdeff, locals.var_pdeff_dn3, locals.var_pdeff_dn4, locals.var_pdeff_dn5, locals.var_pdeff_dn6, locals.var_pdeff_dn7, locals.var_pdeff_dn8, locals.var_pdeff_dn9, locals.var_pdeff_dn10, locals.var_pdeff_dn11,)
    }
};
        locals.var_pdeff = assign15330_e21219;
        locals.var_pdeff_dn3 = assign15330_e21219_d_n3;
        locals.var_pdeff_dn4 = assign15330_e21219_d_n4;
        locals.var_pdeff_dn5 = assign15330_e21219_d_n5;
        locals.var_pdeff_dn6 = assign15330_e21219_d_n6;
        locals.var_pdeff_dn7 = assign15330_e21219_d_n7;
        locals.var_pdeff_dn8 = assign15330_e21219_d_n8;
        locals.var_pdeff_dn9 = assign15330_e21219_d_n9;
        locals.var_pdeff_dn10 = assign15330_e21219_d_n10;
        locals.var_pdeff_dn11 = assign15330_e21219_d_n11;
        locals.var_pdeff_rv = 0.0;

        let (assign15340_e21234, assign15340_e21234_d_n3, assign15340_e21234_d_n4, assign15340_e21234_d_n5, assign15340_e21234_d_n6, assign15340_e21234_d_n7, assign15340_e21234_d_n8, assign15340_e21234_d_n9, assign15340_e21234_d_n10, assign15340_e21234_d_n11,) = {
    if ((locals.var_guard477 != 0.0) && (locals.var_guard478 == 0.0)) {
        let assign15340_e21226: f64 = (p.p20 * p.p50);
        let assign15340_e21229: f64 = (locals.var_weffcj * p.p2);
        let assign15340_e21230: f64 = (assign15340_e21226 - assign15340_e21229);
        let assign15340_e21232: f64 = (assign15340_e21230).max(0.0);
        (assign15340_e21232, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pdeff, locals.var_pdeff_dn3, locals.var_pdeff_dn4, locals.var_pdeff_dn5, locals.var_pdeff_dn6, locals.var_pdeff_dn7, locals.var_pdeff_dn8, locals.var_pdeff_dn9, locals.var_pdeff_dn10, locals.var_pdeff_dn11,)
    }
};
        locals.var_pdeff = assign15340_e21234;
        locals.var_pdeff_dn3 = assign15340_e21234_d_n3;
        locals.var_pdeff_dn4 = assign15340_e21234_d_n4;
        locals.var_pdeff_dn5 = assign15340_e21234_d_n5;
        locals.var_pdeff_dn6 = assign15340_e21234_d_n6;
        locals.var_pdeff_dn7 = assign15340_e21234_d_n7;
        locals.var_pdeff_dn8 = assign15340_e21234_d_n8;
        locals.var_pdeff_dn9 = assign15340_e21234_d_n9;
        locals.var_pdeff_dn10 = assign15340_e21234_d_n10;
        locals.var_pdeff_dn11 = assign15340_e21234_d_n11;
        locals.var_pdeff_rv = 0.0;

        let (assign15350_e21239, assign15350_e21239_d_n3, assign15350_e21239_d_n4, assign15350_e21239_d_n5, assign15350_e21239_d_n6, assign15350_e21239_d_n7, assign15350_e21239_d_n8, assign15350_e21239_d_n9, assign15350_e21239_d_n10, assign15350_e21239_d_n11,) = {
    if (locals.var_guard477 == 0.0) {
        (locals.var_temp_pdeff, locals.var_temp_pdeff_dn3, locals.var_temp_pdeff_dn4, locals.var_temp_pdeff_dn5, locals.var_temp_pdeff_dn6, locals.var_temp_pdeff_dn7, locals.var_temp_pdeff_dn8, locals.var_temp_pdeff_dn9, locals.var_temp_pdeff_dn10, locals.var_temp_pdeff_dn11,)
    } else {
        (locals.var_pdeff, locals.var_pdeff_dn3, locals.var_pdeff_dn4, locals.var_pdeff_dn5, locals.var_pdeff_dn6, locals.var_pdeff_dn7, locals.var_pdeff_dn8, locals.var_pdeff_dn9, locals.var_pdeff_dn10, locals.var_pdeff_dn11,)
    }
};
        locals.var_pdeff = assign15350_e21239;
        locals.var_pdeff_dn3 = assign15350_e21239_d_n3;
        locals.var_pdeff_dn4 = assign15350_e21239_d_n4;
        locals.var_pdeff_dn5 = assign15350_e21239_d_n5;
        locals.var_pdeff_dn6 = assign15350_e21239_d_n6;
        locals.var_pdeff_dn7 = assign15350_e21239_d_n7;
        locals.var_pdeff_dn8 = assign15350_e21239_d_n8;
        locals.var_pdeff_dn9 = assign15350_e21239_d_n9;
        locals.var_pdeff_dn10 = assign15350_e21239_d_n10;
        locals.var_pdeff_dn11 = assign15350_e21239_d_n11;
        locals.var_pdeff_rv = 0.0;

        let assign15360_e21242: f64 = if locals.var_pdeff < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard479 = assign15360_e21242;
        locals.var_guard479_rv = 0.0;

        let (assign15370_e21249, assign15370_e21249_d_n3, assign15370_e21249_d_n4, assign15370_e21249_d_n5, assign15370_e21249_d_n6, assign15370_e21249_d_n7, assign15370_e21249_d_n8, assign15370_e21249_d_n9, assign15370_e21249_d_n10, assign15370_e21249_d_n11,) = {
    if ((locals.var_guard477 == 0.0) && (locals.var_guard479 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pdeff, locals.var_pdeff_dn3, locals.var_pdeff_dn4, locals.var_pdeff_dn5, locals.var_pdeff_dn6, locals.var_pdeff_dn7, locals.var_pdeff_dn8, locals.var_pdeff_dn9, locals.var_pdeff_dn10, locals.var_pdeff_dn11,)
    }
};
        locals.var_pdeff = assign15370_e21249;
        locals.var_pdeff_dn3 = assign15370_e21249_d_n3;
        locals.var_pdeff_dn4 = assign15370_e21249_d_n4;
        locals.var_pdeff_dn5 = assign15370_e21249_d_n5;
        locals.var_pdeff_dn6 = assign15370_e21249_d_n6;
        locals.var_pdeff_dn7 = assign15370_e21249_d_n7;
        locals.var_pdeff_dn8 = assign15370_e21249_d_n8;
        locals.var_pdeff_dn9 = assign15370_e21249_d_n9;
        locals.var_pdeff_dn10 = assign15370_e21249_d_n10;
        locals.var_pdeff_dn11 = assign15370_e21249_d_n11;
        locals.var_pdeff_rv = 0.0;

        let assign15380_e21268: f64 = if (((p.p10 > 0.0) && (p.p11 > 0.0)) && ((p.p2 == 1.0) || ((p.p2 > 1.0) && (p.p12 > 0.0)))) { 1.0 } else { 0.0 };
        locals.var_guard480 = assign15380_e21268;
        locals.var_guard480_rv = 0.0;

        let (assign15390_e21274, assign15390_e21274_d_n3, assign15390_e21274_d_n4, assign15390_e21274_d_n5, assign15390_e21274_d_n6, assign15390_e21274_d_n7, assign15390_e21274_d_n8, assign15390_e21274_d_n9, assign15390_e21274_d_n10, assign15390_e21274_d_n11,) = {
    if (locals.var_guard480 != 0.0) {
        let assign15390_e21272: f64 = (locals.var_lnew).powf(p.p1111);
        (assign15390_e21272, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign15390_e21274;
        locals.var_t0_dn3 = assign15390_e21274_d_n3;
        locals.var_t0_dn4 = assign15390_e21274_d_n4;
        locals.var_t0_dn5 = assign15390_e21274_d_n5;
        locals.var_t0_dn6 = assign15390_e21274_d_n6;
        locals.var_t0_dn7 = assign15390_e21274_d_n7;
        locals.var_t0_dn8 = assign15390_e21274_d_n8;
        locals.var_t0_dn9 = assign15390_e21274_d_n9;
        locals.var_t0_dn10 = assign15390_e21274_d_n10;
        locals.var_t0_dn11 = assign15390_e21274_d_n11;
        locals.var_t0_rv = 0.0;

        let (assign15400_e21280,) = {
    if (locals.var_guard480 != 0.0) {
        let assign15400_e21278: f64 = (locals.var_wnew + p.p1104);
        (assign15400_e21278,)
    } else {
        (locals.var_w_tmp_stress,)
    }
};
        locals.var_w_tmp_stress = assign15400_e21280;
        locals.var_w_tmp_stress_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_28(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign15410_e21286, assign15410_e21286_d_n3, assign15410_e21286_d_n4, assign15410_e21286_d_n5, assign15410_e21286_d_n6, assign15410_e21286_d_n7, assign15410_e21286_d_n8, assign15410_e21286_d_n9, assign15410_e21286_d_n10, assign15410_e21286_d_n11,) = {
    if (locals.var_guard480 != 0.0) {
        let assign15410_e21284: f64 = (locals.var_w_tmp_stress).powf(p.p1112);
        (assign15410_e21284, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign15410_e21286;
        locals.var_t1_dn3 = assign15410_e21286_d_n3;
        locals.var_t1_dn4 = assign15410_e21286_d_n4;
        locals.var_t1_dn5 = assign15410_e21286_d_n5;
        locals.var_t1_dn6 = assign15410_e21286_d_n6;
        locals.var_t1_dn7 = assign15410_e21286_d_n7;
        locals.var_t1_dn8 = assign15410_e21286_d_n8;
        locals.var_t1_dn9 = assign15410_e21286_d_n9;
        locals.var_t1_dn10 = assign15410_e21286_d_n10;
        locals.var_t1_dn11 = assign15410_e21286_d_n11;
        locals.var_t1_rv = 0.0;

        let (assign15420_e21302, assign15420_e21302_d_n3, assign15420_e21302_d_n4, assign15420_e21302_d_n5, assign15420_e21302_d_n6, assign15420_e21302_d_n7, assign15420_e21302_d_n8, assign15420_e21302_d_n9, assign15420_e21302_d_n10, assign15420_e21302_d_n11,) = {
    if (locals.var_guard480 != 0.0) {
        let assign15420_e21290: f64 = (p.p1108 / locals.var_t0);
        let assign15420_e21293: f64 = (p.p1109 / locals.var_t1);
        let assign15420_e21294: f64 = (assign15420_e21290 + assign15420_e21293);
        let assign15420_e21298: f64 = (locals.var_t0 * locals.var_t1);
        let assign15420_e21299: f64 = (p.p1110 / assign15420_e21298);
        let assign15420_e21300: f64 = (assign15420_e21294 + assign15420_e21299);
        (assign15420_e21300, (((-((p.p1108 * locals.var_t0_dn3) / (locals.var_t0 * locals.var_t0))) + (-((p.p1109 * locals.var_t1_dn3) / (locals.var_t1 * locals.var_t1)))) + (-((p.p1110 * ((locals.var_t0_dn3 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn3))) / (assign15420_e21298 * assign15420_e21298)))), (((-((p.p1108 * locals.var_t0_dn4) / (locals.var_t0 * locals.var_t0))) + (-((p.p1109 * locals.var_t1_dn4) / (locals.var_t1 * locals.var_t1)))) + (-((p.p1110 * ((locals.var_t0_dn4 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn4))) / (assign15420_e21298 * assign15420_e21298)))), (((-((p.p1108 * locals.var_t0_dn5) / (locals.var_t0 * locals.var_t0))) + (-((p.p1109 * locals.var_t1_dn5) / (locals.var_t1 * locals.var_t1)))) + (-((p.p1110 * ((locals.var_t0_dn5 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn5))) / (assign15420_e21298 * assign15420_e21298)))), (((-((p.p1108 * locals.var_t0_dn6) / (locals.var_t0 * locals.var_t0))) + (-((p.p1109 * locals.var_t1_dn6) / (locals.var_t1 * locals.var_t1)))) + (-((p.p1110 * ((locals.var_t0_dn6 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn6))) / (assign15420_e21298 * assign15420_e21298)))), (((-((p.p1108 * locals.var_t0_dn7) / (locals.var_t0 * locals.var_t0))) + (-((p.p1109 * locals.var_t1_dn7) / (locals.var_t1 * locals.var_t1)))) + (-((p.p1110 * ((locals.var_t0_dn7 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn7))) / (assign15420_e21298 * assign15420_e21298)))), (((-((p.p1108 * locals.var_t0_dn8) / (locals.var_t0 * locals.var_t0))) + (-((p.p1109 * locals.var_t1_dn8) / (locals.var_t1 * locals.var_t1)))) + (-((p.p1110 * ((locals.var_t0_dn8 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn8))) / (assign15420_e21298 * assign15420_e21298)))), (((-((p.p1108 * locals.var_t0_dn9) / (locals.var_t0 * locals.var_t0))) + (-((p.p1109 * locals.var_t1_dn9) / (locals.var_t1 * locals.var_t1)))) + (-((p.p1110 * ((locals.var_t0_dn9 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn9))) / (assign15420_e21298 * assign15420_e21298)))), (((-((p.p1108 * locals.var_t0_dn10) / (locals.var_t0 * locals.var_t0))) + (-((p.p1109 * locals.var_t1_dn10) / (locals.var_t1 * locals.var_t1)))) + (-((p.p1110 * ((locals.var_t0_dn10 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn10))) / (assign15420_e21298 * assign15420_e21298)))), (((-((p.p1108 * locals.var_t0_dn11) / (locals.var_t0 * locals.var_t0))) + (-((p.p1109 * locals.var_t1_dn11) / (locals.var_t1 * locals.var_t1)))) + (-((p.p1110 * ((locals.var_t0_dn11 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn11))) / (assign15420_e21298 * assign15420_e21298)))),)
    } else {
        (locals.var_tmp1_stress, locals.var_tmp1_stress_dn3, locals.var_tmp1_stress_dn4, locals.var_tmp1_stress_dn5, locals.var_tmp1_stress_dn6, locals.var_tmp1_stress_dn7, locals.var_tmp1_stress_dn8, locals.var_tmp1_stress_dn9, locals.var_tmp1_stress_dn10, locals.var_tmp1_stress_dn11,)
    }
};
        locals.var_tmp1_stress = assign15420_e21302;
        locals.var_tmp1_stress_dn3 = assign15420_e21302_d_n3;
        locals.var_tmp1_stress_dn4 = assign15420_e21302_d_n4;
        locals.var_tmp1_stress_dn5 = assign15420_e21302_d_n5;
        locals.var_tmp1_stress_dn6 = assign15420_e21302_d_n6;
        locals.var_tmp1_stress_dn7 = assign15420_e21302_d_n7;
        locals.var_tmp1_stress_dn8 = assign15420_e21302_d_n8;
        locals.var_tmp1_stress_dn9 = assign15420_e21302_d_n9;
        locals.var_tmp1_stress_dn10 = assign15420_e21302_d_n10;
        locals.var_tmp1_stress_dn11 = assign15420_e21302_d_n11;
        locals.var_tmp1_stress_rv = 0.0;

        let (assign15430_e21308, assign15430_e21308_d_n3, assign15430_e21308_d_n4, assign15430_e21308_d_n5, assign15430_e21308_d_n6, assign15430_e21308_d_n7, assign15430_e21308_d_n8, assign15430_e21308_d_n9, assign15430_e21308_d_n10, assign15430_e21308_d_n11,) = {
    if (locals.var_guard480 != 0.0) {
        let assign15430_e21306: f64 = (1.0 + locals.var_tmp1_stress);
        (assign15430_e21306, locals.var_tmp1_stress_dn3, locals.var_tmp1_stress_dn4, locals.var_tmp1_stress_dn5, locals.var_tmp1_stress_dn6, locals.var_tmp1_stress_dn7, locals.var_tmp1_stress_dn8, locals.var_tmp1_stress_dn9, locals.var_tmp1_stress_dn10, locals.var_tmp1_stress_dn11,)
    } else {
        (locals.var_kstress_u0, locals.var_kstress_u0_dn3, locals.var_kstress_u0_dn4, locals.var_kstress_u0_dn5, locals.var_kstress_u0_dn6, locals.var_kstress_u0_dn7, locals.var_kstress_u0_dn8, locals.var_kstress_u0_dn9, locals.var_kstress_u0_dn10, locals.var_kstress_u0_dn11,)
    }
};
        locals.var_kstress_u0 = assign15430_e21308;
        locals.var_kstress_u0_dn3 = assign15430_e21308_d_n3;
        locals.var_kstress_u0_dn4 = assign15430_e21308_d_n4;
        locals.var_kstress_u0_dn5 = assign15430_e21308_d_n5;
        locals.var_kstress_u0_dn6 = assign15430_e21308_d_n6;
        locals.var_kstress_u0_dn7 = assign15430_e21308_d_n7;
        locals.var_kstress_u0_dn8 = assign15430_e21308_d_n8;
        locals.var_kstress_u0_dn9 = assign15430_e21308_d_n9;
        locals.var_kstress_u0_dn10 = assign15430_e21308_d_n10;
        locals.var_kstress_u0_dn11 = assign15430_e21308_d_n11;
        locals.var_kstress_u0_rv = 0.0;

        let (assign15440_e21314, assign15440_e21314_d_n3, assign15440_e21314_d_n4, assign15440_e21314_d_n5, assign15440_e21314_d_n6, assign15440_e21314_d_n7, assign15440_e21314_d_n8, assign15440_e21314_d_n9, assign15440_e21314_d_n10, assign15440_e21314_d_n11,) = {
    if (locals.var_guard480 != 0.0) {
        let assign15440_e21312: f64 = (locals.var_lnew).powf(p.p1117);
        (assign15440_e21312, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign15440_e21314;
        locals.var_t0_dn3 = assign15440_e21314_d_n3;
        locals.var_t0_dn4 = assign15440_e21314_d_n4;
        locals.var_t0_dn5 = assign15440_e21314_d_n5;
        locals.var_t0_dn6 = assign15440_e21314_d_n6;
        locals.var_t0_dn7 = assign15440_e21314_d_n7;
        locals.var_t0_dn8 = assign15440_e21314_d_n8;
        locals.var_t0_dn9 = assign15440_e21314_d_n9;
        locals.var_t0_dn10 = assign15440_e21314_d_n10;
        locals.var_t0_dn11 = assign15440_e21314_d_n11;
        locals.var_t0_rv = 0.0;

        let (assign15450_e21320, assign15450_e21320_d_n3, assign15450_e21320_d_n4, assign15450_e21320_d_n5, assign15450_e21320_d_n6, assign15450_e21320_d_n7, assign15450_e21320_d_n8, assign15450_e21320_d_n9, assign15450_e21320_d_n10, assign15450_e21320_d_n11,) = {
    if (locals.var_guard480 != 0.0) {
        let assign15450_e21318: f64 = (locals.var_w_tmp_stress).powf(p.p1118);
        (assign15450_e21318, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign15450_e21320;
        locals.var_t1_dn3 = assign15450_e21320_d_n3;
        locals.var_t1_dn4 = assign15450_e21320_d_n4;
        locals.var_t1_dn5 = assign15450_e21320_d_n5;
        locals.var_t1_dn6 = assign15450_e21320_d_n6;
        locals.var_t1_dn7 = assign15450_e21320_d_n7;
        locals.var_t1_dn8 = assign15450_e21320_d_n8;
        locals.var_t1_dn9 = assign15450_e21320_d_n9;
        locals.var_t1_dn10 = assign15450_e21320_d_n10;
        locals.var_t1_dn11 = assign15450_e21320_d_n11;
        locals.var_t1_rv = 0.0;

        let (assign15460_e21336, assign15460_e21336_d_n3, assign15460_e21336_d_n4, assign15460_e21336_d_n5, assign15460_e21336_d_n6, assign15460_e21336_d_n7, assign15460_e21336_d_n8, assign15460_e21336_d_n9, assign15460_e21336_d_n10, assign15460_e21336_d_n11,) = {
    if (locals.var_guard480 != 0.0) {
        let assign15460_e21324: f64 = (p.p1114 / locals.var_t0);
        let assign15460_e21327: f64 = (p.p1115 / locals.var_t1);
        let assign15460_e21328: f64 = (assign15460_e21324 + assign15460_e21327);
        let assign15460_e21332: f64 = (locals.var_t0 * locals.var_t1);
        let assign15460_e21333: f64 = (p.p1116 / assign15460_e21332);
        let assign15460_e21334: f64 = (assign15460_e21328 + assign15460_e21333);
        (assign15460_e21334, (((-((p.p1114 * locals.var_t0_dn3) / (locals.var_t0 * locals.var_t0))) + (-((p.p1115 * locals.var_t1_dn3) / (locals.var_t1 * locals.var_t1)))) + (-((p.p1116 * ((locals.var_t0_dn3 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn3))) / (assign15460_e21332 * assign15460_e21332)))), (((-((p.p1114 * locals.var_t0_dn4) / (locals.var_t0 * locals.var_t0))) + (-((p.p1115 * locals.var_t1_dn4) / (locals.var_t1 * locals.var_t1)))) + (-((p.p1116 * ((locals.var_t0_dn4 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn4))) / (assign15460_e21332 * assign15460_e21332)))), (((-((p.p1114 * locals.var_t0_dn5) / (locals.var_t0 * locals.var_t0))) + (-((p.p1115 * locals.var_t1_dn5) / (locals.var_t1 * locals.var_t1)))) + (-((p.p1116 * ((locals.var_t0_dn5 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn5))) / (assign15460_e21332 * assign15460_e21332)))), (((-((p.p1114 * locals.var_t0_dn6) / (locals.var_t0 * locals.var_t0))) + (-((p.p1115 * locals.var_t1_dn6) / (locals.var_t1 * locals.var_t1)))) + (-((p.p1116 * ((locals.var_t0_dn6 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn6))) / (assign15460_e21332 * assign15460_e21332)))), (((-((p.p1114 * locals.var_t0_dn7) / (locals.var_t0 * locals.var_t0))) + (-((p.p1115 * locals.var_t1_dn7) / (locals.var_t1 * locals.var_t1)))) + (-((p.p1116 * ((locals.var_t0_dn7 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn7))) / (assign15460_e21332 * assign15460_e21332)))), (((-((p.p1114 * locals.var_t0_dn8) / (locals.var_t0 * locals.var_t0))) + (-((p.p1115 * locals.var_t1_dn8) / (locals.var_t1 * locals.var_t1)))) + (-((p.p1116 * ((locals.var_t0_dn8 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn8))) / (assign15460_e21332 * assign15460_e21332)))), (((-((p.p1114 * locals.var_t0_dn9) / (locals.var_t0 * locals.var_t0))) + (-((p.p1115 * locals.var_t1_dn9) / (locals.var_t1 * locals.var_t1)))) + (-((p.p1116 * ((locals.var_t0_dn9 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn9))) / (assign15460_e21332 * assign15460_e21332)))), (((-((p.p1114 * locals.var_t0_dn10) / (locals.var_t0 * locals.var_t0))) + (-((p.p1115 * locals.var_t1_dn10) / (locals.var_t1 * locals.var_t1)))) + (-((p.p1116 * ((locals.var_t0_dn10 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn10))) / (assign15460_e21332 * assign15460_e21332)))), (((-((p.p1114 * locals.var_t0_dn11) / (locals.var_t0 * locals.var_t0))) + (-((p.p1115 * locals.var_t1_dn11) / (locals.var_t1 * locals.var_t1)))) + (-((p.p1116 * ((locals.var_t0_dn11 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn11))) / (assign15460_e21332 * assign15460_e21332)))),)
    } else {
        (locals.var_tmp1_stress_vth, locals.var_tmp1_stress_vth_dn3, locals.var_tmp1_stress_vth_dn4, locals.var_tmp1_stress_vth_dn5, locals.var_tmp1_stress_vth_dn6, locals.var_tmp1_stress_vth_dn7, locals.var_tmp1_stress_vth_dn8, locals.var_tmp1_stress_vth_dn9, locals.var_tmp1_stress_vth_dn10, locals.var_tmp1_stress_vth_dn11,)
    }
};
        locals.var_tmp1_stress_vth = assign15460_e21336;
        locals.var_tmp1_stress_vth_dn3 = assign15460_e21336_d_n3;
        locals.var_tmp1_stress_vth_dn4 = assign15460_e21336_d_n4;
        locals.var_tmp1_stress_vth_dn5 = assign15460_e21336_d_n5;
        locals.var_tmp1_stress_vth_dn6 = assign15460_e21336_d_n6;
        locals.var_tmp1_stress_vth_dn7 = assign15460_e21336_d_n7;
        locals.var_tmp1_stress_vth_dn8 = assign15460_e21336_d_n8;
        locals.var_tmp1_stress_vth_dn9 = assign15460_e21336_d_n9;
        locals.var_tmp1_stress_vth_dn10 = assign15460_e21336_d_n10;
        locals.var_tmp1_stress_vth_dn11 = assign15460_e21336_d_n11;
        locals.var_tmp1_stress_vth_rv = 0.0;

        let (assign15470_e21342, assign15470_e21342_d_n3, assign15470_e21342_d_n4, assign15470_e21342_d_n5, assign15470_e21342_d_n6, assign15470_e21342_d_n7, assign15470_e21342_d_n8, assign15470_e21342_d_n9, assign15470_e21342_d_n10, assign15470_e21342_d_n11,) = {
    if (locals.var_guard480 != 0.0) {
        let assign15470_e21340: f64 = (1.0 + locals.var_tmp1_stress_vth);
        (assign15470_e21340, locals.var_tmp1_stress_vth_dn3, locals.var_tmp1_stress_vth_dn4, locals.var_tmp1_stress_vth_dn5, locals.var_tmp1_stress_vth_dn6, locals.var_tmp1_stress_vth_dn7, locals.var_tmp1_stress_vth_dn8, locals.var_tmp1_stress_vth_dn9, locals.var_tmp1_stress_vth_dn10, locals.var_tmp1_stress_vth_dn11,)
    } else {
        (locals.var_kstress_vth0, locals.var_kstress_vth0_dn3, locals.var_kstress_vth0_dn4, locals.var_kstress_vth0_dn5, locals.var_kstress_vth0_dn6, locals.var_kstress_vth0_dn7, locals.var_kstress_vth0_dn8, locals.var_kstress_vth0_dn9, locals.var_kstress_vth0_dn10, locals.var_kstress_vth0_dn11,)
    }
};
        locals.var_kstress_vth0 = assign15470_e21342;
        locals.var_kstress_vth0_dn3 = assign15470_e21342_d_n3;
        locals.var_kstress_vth0_dn4 = assign15470_e21342_d_n4;
        locals.var_kstress_vth0_dn5 = assign15470_e21342_d_n5;
        locals.var_kstress_vth0_dn6 = assign15470_e21342_d_n6;
        locals.var_kstress_vth0_dn7 = assign15470_e21342_d_n7;
        locals.var_kstress_vth0_dn8 = assign15470_e21342_d_n8;
        locals.var_kstress_vth0_dn9 = assign15470_e21342_d_n9;
        locals.var_kstress_vth0_dn10 = assign15470_e21342_d_n10;
        locals.var_kstress_vth0_dn11 = assign15470_e21342_d_n11;
        locals.var_kstress_vth0_rv = 0.0;

        let (assign15480_e21348, assign15480_e21348_d_n3, assign15480_e21348_d_n4, assign15480_e21348_d_n5, assign15480_e21348_d_n6, assign15480_e21348_d_n7, assign15480_e21348_d_n8, assign15480_e21348_d_n9, assign15480_e21348_d_n10, assign15480_e21348_d_n11,) = {
    if (locals.var_guard480 != 0.0) {
        let assign15480_e21346: f64 = (locals.var_tratio - 1.0);
        (assign15480_e21346, 0.0, locals.var_tratio_dn4, locals.var_tratio_dn5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign15480_e21348;
        locals.var_t0_dn3 = assign15480_e21348_d_n3;
        locals.var_t0_dn4 = assign15480_e21348_d_n4;
        locals.var_t0_dn5 = assign15480_e21348_d_n5;
        locals.var_t0_dn6 = assign15480_e21348_d_n6;
        locals.var_t0_dn7 = assign15480_e21348_d_n7;
        locals.var_t0_dn8 = assign15480_e21348_d_n8;
        locals.var_t0_dn9 = assign15480_e21348_d_n9;
        locals.var_t0_dn10 = assign15480_e21348_d_n10;
        locals.var_t0_dn11 = assign15480_e21348_d_n11;
        locals.var_t0_rv = 0.0;

        let (assign15490_e21360, assign15490_e21360_d_n3, assign15490_e21360_d_n4, assign15490_e21360_d_n5, assign15490_e21360_d_n6, assign15490_e21360_d_n7, assign15490_e21360_d_n8, assign15490_e21360_d_n9, assign15490_e21360_d_n10, assign15490_e21360_d_n11,) = {
    if (locals.var_guard480 != 0.0) {
        let assign15490_e21354: f64 = (p.p1107 * locals.var_t0);
        let assign15490_e21355: f64 = (1.0 + assign15490_e21354);
        let assign15490_e21356: f64 = (locals.var_kstress_u0 * assign15490_e21355);
        let assign15490_e21358: f64 = (assign15490_e21356 + 1e-9);
        (assign15490_e21358, ((locals.var_kstress_u0_dn3 * assign15490_e21355) + (locals.var_kstress_u0 * (p.p1107 * locals.var_t0_dn3))), ((locals.var_kstress_u0_dn4 * assign15490_e21355) + (locals.var_kstress_u0 * (p.p1107 * locals.var_t0_dn4))), ((locals.var_kstress_u0_dn5 * assign15490_e21355) + (locals.var_kstress_u0 * (p.p1107 * locals.var_t0_dn5))), ((locals.var_kstress_u0_dn6 * assign15490_e21355) + (locals.var_kstress_u0 * (p.p1107 * locals.var_t0_dn6))), ((locals.var_kstress_u0_dn7 * assign15490_e21355) + (locals.var_kstress_u0 * (p.p1107 * locals.var_t0_dn7))), ((locals.var_kstress_u0_dn8 * assign15490_e21355) + (locals.var_kstress_u0 * (p.p1107 * locals.var_t0_dn8))), ((locals.var_kstress_u0_dn9 * assign15490_e21355) + (locals.var_kstress_u0 * (p.p1107 * locals.var_t0_dn9))), ((locals.var_kstress_u0_dn10 * assign15490_e21355) + (locals.var_kstress_u0 * (p.p1107 * locals.var_t0_dn10))), ((locals.var_kstress_u0_dn11 * assign15490_e21355) + (locals.var_kstress_u0 * (p.p1107 * locals.var_t0_dn11))),)
    } else {
        (locals.var_ku0_temp, locals.var_ku0_temp_dn3, locals.var_ku0_temp_dn4, locals.var_ku0_temp_dn5, locals.var_ku0_temp_dn6, locals.var_ku0_temp_dn7, locals.var_ku0_temp_dn8, locals.var_ku0_temp_dn9, locals.var_ku0_temp_dn10, locals.var_ku0_temp_dn11,)
    }
};
        locals.var_ku0_temp = assign15490_e21360;
        locals.var_ku0_temp_dn3 = assign15490_e21360_d_n3;
        locals.var_ku0_temp_dn4 = assign15490_e21360_d_n4;
        locals.var_ku0_temp_dn5 = assign15490_e21360_d_n5;
        locals.var_ku0_temp_dn6 = assign15490_e21360_d_n6;
        locals.var_ku0_temp_dn7 = assign15490_e21360_d_n7;
        locals.var_ku0_temp_dn8 = assign15490_e21360_d_n8;
        locals.var_ku0_temp_dn9 = assign15490_e21360_d_n9;
        locals.var_ku0_temp_dn10 = assign15490_e21360_d_n10;
        locals.var_ku0_temp_dn11 = assign15490_e21360_d_n11;
        locals.var_ku0_temp_rv = 0.0;

        let (assign15500_e21364,) = {
    if (locals.var_guard480 != 0.0) {
        (0.0,)
    } else {
        (locals.var_i,)
    }
};
        locals.var_i = assign15500_e21364;
        locals.var_i_rv = 0.0;

        let mut assign15510_loop_guard: usize = 0;
        while {
            let assign15510_cond_e21369: f64 = if ((locals.var_guard480 != 0.0) && (locals.var_i < p.p2)) { 1.0 } else { 0.0 };
            assign15510_cond_e21369 != 0.0
        } {
            assign15510_loop_guard += 1;
            assert!(assign15510_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign15510_body0_e21387, assign15510_body0_e21387_d_n3, assign15510_body0_e21387_d_n4, assign15510_body0_e21387_d_n5, assign15510_body0_e21387_d_n6, assign15510_body0_e21387_d_n7, assign15510_body0_e21387_d_n8, assign15510_body0_e21387_d_n9, assign15510_body0_e21387_d_n10, assign15510_body0_e21387_d_n11,) = {
    if (locals.var_guard480 != 0.0) {
        let assign15510_body0_e21373: f64 = (1.0 / p.p2);
        let assign15510_body0_e21377: f64 = (0.5 * locals.var_l_mult);
        let assign15510_body0_e21378: f64 = (p.p10 + assign15510_body0_e21377);
        let assign15510_body0_e21382: f64 = (p.p12 + locals.var_l_mult);
        let assign15510_body0_e21383: f64 = (locals.var_i * assign15510_body0_e21382);
        let assign15510_body0_e21384: f64 = (assign15510_body0_e21378 + assign15510_body0_e21383);
        let assign15510_body0_e21385: f64 = (assign15510_body0_e21373 / assign15510_body0_e21384);
        (assign15510_body0_e21385, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
            locals.var_t0 = assign15510_body0_e21387;
            locals.var_t0_dn3 = assign15510_body0_e21387_d_n3;
            locals.var_t0_dn4 = assign15510_body0_e21387_d_n4;
            locals.var_t0_dn5 = assign15510_body0_e21387_d_n5;
            locals.var_t0_dn6 = assign15510_body0_e21387_d_n6;
            locals.var_t0_dn7 = assign15510_body0_e21387_d_n7;
            locals.var_t0_dn8 = assign15510_body0_e21387_d_n8;
            locals.var_t0_dn9 = assign15510_body0_e21387_d_n9;
            locals.var_t0_dn10 = assign15510_body0_e21387_d_n10;
            locals.var_t0_dn11 = assign15510_body0_e21387_d_n11;
            locals.var_t0_rv = 0.0;
            let (assign15510_body1_e21405, assign15510_body1_e21405_d_n3, assign15510_body1_e21405_d_n4, assign15510_body1_e21405_d_n5, assign15510_body1_e21405_d_n6, assign15510_body1_e21405_d_n7, assign15510_body1_e21405_d_n8, assign15510_body1_e21405_d_n9, assign15510_body1_e21405_d_n10, assign15510_body1_e21405_d_n11,) = {
    if (locals.var_guard480 != 0.0) {
        let assign15510_body1_e21391: f64 = (1.0 / p.p2);
        let assign15510_body1_e21395: f64 = (0.5 * locals.var_l_mult);
        let assign15510_body1_e21396: f64 = (p.p11 + assign15510_body1_e21395);
        let assign15510_body1_e21400: f64 = (p.p12 + locals.var_l_mult);
        let assign15510_body1_e21401: f64 = (locals.var_i * assign15510_body1_e21400);
        let assign15510_body1_e21402: f64 = (assign15510_body1_e21396 + assign15510_body1_e21401);
        let assign15510_body1_e21403: f64 = (assign15510_body1_e21391 / assign15510_body1_e21402);
        (assign15510_body1_e21403, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
            locals.var_t1 = assign15510_body1_e21405;
            locals.var_t1_dn3 = assign15510_body1_e21405_d_n3;
            locals.var_t1_dn4 = assign15510_body1_e21405_d_n4;
            locals.var_t1_dn5 = assign15510_body1_e21405_d_n5;
            locals.var_t1_dn6 = assign15510_body1_e21405_d_n6;
            locals.var_t1_dn7 = assign15510_body1_e21405_d_n7;
            locals.var_t1_dn8 = assign15510_body1_e21405_d_n8;
            locals.var_t1_dn9 = assign15510_body1_e21405_d_n9;
            locals.var_t1_dn10 = assign15510_body1_e21405_d_n10;
            locals.var_t1_dn11 = assign15510_body1_e21405_d_n11;
            locals.var_t1_rv = 0.0;
            let (assign15510_body2_e21411, assign15510_body2_e21411_d_n3, assign15510_body2_e21411_d_n4, assign15510_body2_e21411_d_n5, assign15510_body2_e21411_d_n6, assign15510_body2_e21411_d_n7, assign15510_body2_e21411_d_n8, assign15510_body2_e21411_d_n9, assign15510_body2_e21411_d_n10, assign15510_body2_e21411_d_n11,) = {
    if (locals.var_guard480 != 0.0) {
        let assign15510_body2_e21409: f64 = (locals.var_inv_sa + locals.var_t0);
        (assign15510_body2_e21409, (locals.var_inv_sa_dn3 + locals.var_t0_dn3), (locals.var_inv_sa_dn4 + locals.var_t0_dn4), (locals.var_inv_sa_dn5 + locals.var_t0_dn5), (locals.var_inv_sa_dn6 + locals.var_t0_dn6), (locals.var_inv_sa_dn7 + locals.var_t0_dn7), (locals.var_inv_sa_dn8 + locals.var_t0_dn8), (locals.var_inv_sa_dn9 + locals.var_t0_dn9), (locals.var_inv_sa_dn10 + locals.var_t0_dn10), (locals.var_inv_sa_dn11 + locals.var_t0_dn11),)
    } else {
        (locals.var_inv_sa, locals.var_inv_sa_dn3, locals.var_inv_sa_dn4, locals.var_inv_sa_dn5, locals.var_inv_sa_dn6, locals.var_inv_sa_dn7, locals.var_inv_sa_dn8, locals.var_inv_sa_dn9, locals.var_inv_sa_dn10, locals.var_inv_sa_dn11,)
    }
};
            locals.var_inv_sa = assign15510_body2_e21411;
            locals.var_inv_sa_dn3 = assign15510_body2_e21411_d_n3;
            locals.var_inv_sa_dn4 = assign15510_body2_e21411_d_n4;
            locals.var_inv_sa_dn5 = assign15510_body2_e21411_d_n5;
            locals.var_inv_sa_dn6 = assign15510_body2_e21411_d_n6;
            locals.var_inv_sa_dn7 = assign15510_body2_e21411_d_n7;
            locals.var_inv_sa_dn8 = assign15510_body2_e21411_d_n8;
            locals.var_inv_sa_dn9 = assign15510_body2_e21411_d_n9;
            locals.var_inv_sa_dn10 = assign15510_body2_e21411_d_n10;
            locals.var_inv_sa_dn11 = assign15510_body2_e21411_d_n11;
            locals.var_inv_sa_rv = 0.0;
            let (assign15510_body3_e21417, assign15510_body3_e21417_d_n3, assign15510_body3_e21417_d_n4, assign15510_body3_e21417_d_n5, assign15510_body3_e21417_d_n6, assign15510_body3_e21417_d_n7, assign15510_body3_e21417_d_n8, assign15510_body3_e21417_d_n9, assign15510_body3_e21417_d_n10, assign15510_body3_e21417_d_n11,) = {
    if (locals.var_guard480 != 0.0) {
        let assign15510_body3_e21415: f64 = (locals.var_inv_sb + locals.var_t1);
        (assign15510_body3_e21415, (locals.var_inv_sb_dn3 + locals.var_t1_dn3), (locals.var_inv_sb_dn4 + locals.var_t1_dn4), (locals.var_inv_sb_dn5 + locals.var_t1_dn5), (locals.var_inv_sb_dn6 + locals.var_t1_dn6), (locals.var_inv_sb_dn7 + locals.var_t1_dn7), (locals.var_inv_sb_dn8 + locals.var_t1_dn8), (locals.var_inv_sb_dn9 + locals.var_t1_dn9), (locals.var_inv_sb_dn10 + locals.var_t1_dn10), (locals.var_inv_sb_dn11 + locals.var_t1_dn11),)
    } else {
        (locals.var_inv_sb, locals.var_inv_sb_dn3, locals.var_inv_sb_dn4, locals.var_inv_sb_dn5, locals.var_inv_sb_dn6, locals.var_inv_sb_dn7, locals.var_inv_sb_dn8, locals.var_inv_sb_dn9, locals.var_inv_sb_dn10, locals.var_inv_sb_dn11,)
    }
};
            locals.var_inv_sb = assign15510_body3_e21417;
            locals.var_inv_sb_dn3 = assign15510_body3_e21417_d_n3;
            locals.var_inv_sb_dn4 = assign15510_body3_e21417_d_n4;
            locals.var_inv_sb_dn5 = assign15510_body3_e21417_d_n5;
            locals.var_inv_sb_dn6 = assign15510_body3_e21417_d_n6;
            locals.var_inv_sb_dn7 = assign15510_body3_e21417_d_n7;
            locals.var_inv_sb_dn8 = assign15510_body3_e21417_d_n8;
            locals.var_inv_sb_dn9 = assign15510_body3_e21417_d_n9;
            locals.var_inv_sb_dn10 = assign15510_body3_e21417_d_n10;
            locals.var_inv_sb_dn11 = assign15510_body3_e21417_d_n11;
            locals.var_inv_sb_rv = 0.0;
            let (assign15510_body4_e21423,) = {
    if (locals.var_guard480 != 0.0) {
        let assign15510_body4_e21421: f64 = (locals.var_i + 1.0);
        (assign15510_body4_e21421,)
    } else {
        (locals.var_i,)
    }
};
            locals.var_i = assign15510_body4_e21423;
            locals.var_i_rv = 0.0;
        }

        let (assign15520_e21433,) = {
    if (locals.var_guard480 != 0.0) {
        let assign15520_e21429: f64 = (0.5 * locals.var_l_mult);
        let assign15520_e21430: f64 = (p.p1102 + assign15520_e21429);
        let assign15520_e21431: f64 = (1.0 / assign15520_e21430);
        (assign15520_e21431,)
    } else {
        (locals.var_inv_saref,)
    }
};
        locals.var_inv_saref = assign15520_e21433;
        locals.var_inv_saref_rv = 0.0;

        let (assign15530_e21443,) = {
    if (locals.var_guard480 != 0.0) {
        let assign15530_e21439: f64 = (0.5 * locals.var_l_mult);
        let assign15530_e21440: f64 = (p.p1103 + assign15530_e21439);
        let assign15530_e21441: f64 = (1.0 / assign15530_e21440);
        (assign15530_e21441,)
    } else {
        (locals.var_inv_sbref,)
    }
};
        locals.var_inv_sbref = assign15530_e21443;
        locals.var_inv_sbref_rv = 0.0;

        let (assign15540_e21449,) = {
    if (locals.var_guard480 != 0.0) {
        let assign15540_e21447: f64 = (locals.var_inv_saref + locals.var_inv_sbref);
        (assign15540_e21447,)
    } else {
        (locals.var_inv_odref,)
    }
};
        locals.var_inv_odref = assign15540_e21449;
        locals.var_inv_odref_rv = 0.0;

        let (assign15550_e21457, assign15550_e21457_d_n3, assign15550_e21457_d_n4, assign15550_e21457_d_n5, assign15550_e21457_d_n6, assign15550_e21457_d_n7, assign15550_e21457_d_n8, assign15550_e21457_d_n9, assign15550_e21457_d_n10, assign15550_e21457_d_n11,) = {
    if (locals.var_guard480 != 0.0) {
        let assign15550_e21453: f64 = (p.p1105 / locals.var_ku0_temp);
        let assign15550_e21455: f64 = (assign15550_e21453 * locals.var_inv_odref);
        (assign15550_e21455, ((-((p.p1105 * locals.var_ku0_temp_dn3) / (locals.var_ku0_temp * locals.var_ku0_temp))) * locals.var_inv_odref), ((-((p.p1105 * locals.var_ku0_temp_dn4) / (locals.var_ku0_temp * locals.var_ku0_temp))) * locals.var_inv_odref), ((-((p.p1105 * locals.var_ku0_temp_dn5) / (locals.var_ku0_temp * locals.var_ku0_temp))) * locals.var_inv_odref), ((-((p.p1105 * locals.var_ku0_temp_dn6) / (locals.var_ku0_temp * locals.var_ku0_temp))) * locals.var_inv_odref), ((-((p.p1105 * locals.var_ku0_temp_dn7) / (locals.var_ku0_temp * locals.var_ku0_temp))) * locals.var_inv_odref), ((-((p.p1105 * locals.var_ku0_temp_dn8) / (locals.var_ku0_temp * locals.var_ku0_temp))) * locals.var_inv_odref), ((-((p.p1105 * locals.var_ku0_temp_dn9) / (locals.var_ku0_temp * locals.var_ku0_temp))) * locals.var_inv_odref), ((-((p.p1105 * locals.var_ku0_temp_dn10) / (locals.var_ku0_temp * locals.var_ku0_temp))) * locals.var_inv_odref), ((-((p.p1105 * locals.var_ku0_temp_dn11) / (locals.var_ku0_temp * locals.var_ku0_temp))) * locals.var_inv_odref),)
    } else {
        (locals.var_rho_ref, locals.var_rho_ref_dn3, locals.var_rho_ref_dn4, locals.var_rho_ref_dn5, locals.var_rho_ref_dn6, locals.var_rho_ref_dn7, locals.var_rho_ref_dn8, locals.var_rho_ref_dn9, locals.var_rho_ref_dn10, locals.var_rho_ref_dn11,)
    }
};
        locals.var_rho_ref = assign15550_e21457;
        locals.var_rho_ref_dn3 = assign15550_e21457_d_n3;
        locals.var_rho_ref_dn4 = assign15550_e21457_d_n4;
        locals.var_rho_ref_dn5 = assign15550_e21457_d_n5;
        locals.var_rho_ref_dn6 = assign15550_e21457_d_n6;
        locals.var_rho_ref_dn7 = assign15550_e21457_d_n7;
        locals.var_rho_ref_dn8 = assign15550_e21457_d_n8;
        locals.var_rho_ref_dn9 = assign15550_e21457_d_n9;
        locals.var_rho_ref_dn10 = assign15550_e21457_d_n10;
        locals.var_rho_ref_dn11 = assign15550_e21457_d_n11;
        locals.var_rho_ref_rv = 0.0;

        let (assign15560_e21463, assign15560_e21463_d_n3, assign15560_e21463_d_n4, assign15560_e21463_d_n5, assign15560_e21463_d_n6, assign15560_e21463_d_n7, assign15560_e21463_d_n8, assign15560_e21463_d_n9, assign15560_e21463_d_n10, assign15560_e21463_d_n11,) = {
    if (locals.var_guard480 != 0.0) {
        let assign15560_e21461: f64 = (locals.var_inv_sa + locals.var_inv_sb);
        (assign15560_e21461, (locals.var_inv_sa_dn3 + locals.var_inv_sb_dn3), (locals.var_inv_sa_dn4 + locals.var_inv_sb_dn4), (locals.var_inv_sa_dn5 + locals.var_inv_sb_dn5), (locals.var_inv_sa_dn6 + locals.var_inv_sb_dn6), (locals.var_inv_sa_dn7 + locals.var_inv_sb_dn7), (locals.var_inv_sa_dn8 + locals.var_inv_sb_dn8), (locals.var_inv_sa_dn9 + locals.var_inv_sb_dn9), (locals.var_inv_sa_dn10 + locals.var_inv_sb_dn10), (locals.var_inv_sa_dn11 + locals.var_inv_sb_dn11),)
    } else {
        (locals.var_inv_od, locals.var_inv_od_dn3, locals.var_inv_od_dn4, locals.var_inv_od_dn5, locals.var_inv_od_dn6, locals.var_inv_od_dn7, locals.var_inv_od_dn8, locals.var_inv_od_dn9, locals.var_inv_od_dn10, locals.var_inv_od_dn11,)
    }
};
        locals.var_inv_od = assign15560_e21463;
        locals.var_inv_od_dn3 = assign15560_e21463_d_n3;
        locals.var_inv_od_dn4 = assign15560_e21463_d_n4;
        locals.var_inv_od_dn5 = assign15560_e21463_d_n5;
        locals.var_inv_od_dn6 = assign15560_e21463_d_n6;
        locals.var_inv_od_dn7 = assign15560_e21463_d_n7;
        locals.var_inv_od_dn8 = assign15560_e21463_d_n8;
        locals.var_inv_od_dn9 = assign15560_e21463_d_n9;
        locals.var_inv_od_dn10 = assign15560_e21463_d_n10;
        locals.var_inv_od_dn11 = assign15560_e21463_d_n11;
        locals.var_inv_od_rv = 0.0;

        let (assign15570_e21471, assign15570_e21471_d_n3, assign15570_e21471_d_n4, assign15570_e21471_d_n5, assign15570_e21471_d_n6, assign15570_e21471_d_n7, assign15570_e21471_d_n8, assign15570_e21471_d_n9, assign15570_e21471_d_n10, assign15570_e21471_d_n11,) = {
    if (locals.var_guard480 != 0.0) {
        let assign15570_e21467: f64 = (p.p1105 / locals.var_ku0_temp);
        let assign15570_e21469: f64 = (assign15570_e21467 * locals.var_inv_od);
        (assign15570_e21469, (((-((p.p1105 * locals.var_ku0_temp_dn3) / (locals.var_ku0_temp * locals.var_ku0_temp))) * locals.var_inv_od) + (assign15570_e21467 * locals.var_inv_od_dn3)), (((-((p.p1105 * locals.var_ku0_temp_dn4) / (locals.var_ku0_temp * locals.var_ku0_temp))) * locals.var_inv_od) + (assign15570_e21467 * locals.var_inv_od_dn4)), (((-((p.p1105 * locals.var_ku0_temp_dn5) / (locals.var_ku0_temp * locals.var_ku0_temp))) * locals.var_inv_od) + (assign15570_e21467 * locals.var_inv_od_dn5)), (((-((p.p1105 * locals.var_ku0_temp_dn6) / (locals.var_ku0_temp * locals.var_ku0_temp))) * locals.var_inv_od) + (assign15570_e21467 * locals.var_inv_od_dn6)), (((-((p.p1105 * locals.var_ku0_temp_dn7) / (locals.var_ku0_temp * locals.var_ku0_temp))) * locals.var_inv_od) + (assign15570_e21467 * locals.var_inv_od_dn7)), (((-((p.p1105 * locals.var_ku0_temp_dn8) / (locals.var_ku0_temp * locals.var_ku0_temp))) * locals.var_inv_od) + (assign15570_e21467 * locals.var_inv_od_dn8)), (((-((p.p1105 * locals.var_ku0_temp_dn9) / (locals.var_ku0_temp * locals.var_ku0_temp))) * locals.var_inv_od) + (assign15570_e21467 * locals.var_inv_od_dn9)), (((-((p.p1105 * locals.var_ku0_temp_dn10) / (locals.var_ku0_temp * locals.var_ku0_temp))) * locals.var_inv_od) + (assign15570_e21467 * locals.var_inv_od_dn10)), (((-((p.p1105 * locals.var_ku0_temp_dn11) / (locals.var_ku0_temp * locals.var_ku0_temp))) * locals.var_inv_od) + (assign15570_e21467 * locals.var_inv_od_dn11)),)
    } else {
        (locals.var_rho, locals.var_rho_dn3, locals.var_rho_dn4, locals.var_rho_dn5, locals.var_rho_dn6, locals.var_rho_dn7, locals.var_rho_dn8, locals.var_rho_dn9, locals.var_rho_dn10, locals.var_rho_dn11,)
    }
};
        locals.var_rho = assign15570_e21471;
        locals.var_rho_dn3 = assign15570_e21471_d_n3;
        locals.var_rho_dn4 = assign15570_e21471_d_n4;
        locals.var_rho_dn5 = assign15570_e21471_d_n5;
        locals.var_rho_dn6 = assign15570_e21471_d_n6;
        locals.var_rho_dn7 = assign15570_e21471_d_n7;
        locals.var_rho_dn8 = assign15570_e21471_d_n8;
        locals.var_rho_dn9 = assign15570_e21471_d_n9;
        locals.var_rho_dn10 = assign15570_e21471_d_n10;
        locals.var_rho_dn11 = assign15570_e21471_d_n11;
        locals.var_rho_rv = 0.0;

        let (assign15580_e21481, assign15580_e21481_d_n3, assign15580_e21481_d_n4, assign15580_e21481_d_n5, assign15580_e21481_d_n6, assign15580_e21481_d_n7, assign15580_e21481_d_n8, assign15580_e21481_d_n9, assign15580_e21481_d_n10, assign15580_e21481_d_n11,) = {
    if (locals.var_guard480 != 0.0) {
        let assign15580_e21475: f64 = (1.0 + locals.var_rho);
        let assign15580_e21478: f64 = (1.0 + locals.var_rho_ref);
        let assign15580_e21479: f64 = (assign15580_e21475 / assign15580_e21478);
        (assign15580_e21479, (((locals.var_rho_dn3 * assign15580_e21478) - (assign15580_e21475 * locals.var_rho_ref_dn3)) / (assign15580_e21478 * assign15580_e21478)), (((locals.var_rho_dn4 * assign15580_e21478) - (assign15580_e21475 * locals.var_rho_ref_dn4)) / (assign15580_e21478 * assign15580_e21478)), (((locals.var_rho_dn5 * assign15580_e21478) - (assign15580_e21475 * locals.var_rho_ref_dn5)) / (assign15580_e21478 * assign15580_e21478)), (((locals.var_rho_dn6 * assign15580_e21478) - (assign15580_e21475 * locals.var_rho_ref_dn6)) / (assign15580_e21478 * assign15580_e21478)), (((locals.var_rho_dn7 * assign15580_e21478) - (assign15580_e21475 * locals.var_rho_ref_dn7)) / (assign15580_e21478 * assign15580_e21478)), (((locals.var_rho_dn8 * assign15580_e21478) - (assign15580_e21475 * locals.var_rho_ref_dn8)) / (assign15580_e21478 * assign15580_e21478)), (((locals.var_rho_dn9 * assign15580_e21478) - (assign15580_e21475 * locals.var_rho_ref_dn9)) / (assign15580_e21478 * assign15580_e21478)), (((locals.var_rho_dn10 * assign15580_e21478) - (assign15580_e21475 * locals.var_rho_ref_dn10)) / (assign15580_e21478 * assign15580_e21478)), (((locals.var_rho_dn11 * assign15580_e21478) - (assign15580_e21475 * locals.var_rho_ref_dn11)) / (assign15580_e21478 * assign15580_e21478)),)
    } else {
        (locals.var_mu0_mult, locals.var_mu0_mult_dn3, locals.var_mu0_mult_dn4, locals.var_mu0_mult_dn5, locals.var_mu0_mult_dn6, locals.var_mu0_mult_dn7, locals.var_mu0_mult_dn8, locals.var_mu0_mult_dn9, locals.var_mu0_mult_dn10, locals.var_mu0_mult_dn11,)
    }
};
        locals.var_mu0_mult = assign15580_e21481;
        locals.var_mu0_mult_dn3 = assign15580_e21481_d_n3;
        locals.var_mu0_mult_dn4 = assign15580_e21481_d_n4;
        locals.var_mu0_mult_dn5 = assign15580_e21481_d_n5;
        locals.var_mu0_mult_dn6 = assign15580_e21481_d_n6;
        locals.var_mu0_mult_dn7 = assign15580_e21481_d_n7;
        locals.var_mu0_mult_dn8 = assign15580_e21481_d_n8;
        locals.var_mu0_mult_dn9 = assign15580_e21481_d_n9;
        locals.var_mu0_mult_dn10 = assign15580_e21481_d_n10;
        locals.var_mu0_mult_dn11 = assign15580_e21481_d_n11;
        locals.var_mu0_mult_rv = 0.0;

        let (assign15590_e21495, assign15590_e21495_d_n3, assign15590_e21495_d_n4, assign15590_e21495_d_n5, assign15590_e21495_d_n6, assign15590_e21495_d_n7, assign15590_e21495_d_n8, assign15590_e21495_d_n9, assign15590_e21495_d_n10, assign15590_e21495_d_n11,) = {
    if (locals.var_guard480 != 0.0) {
        let assign15590_e21486: f64 = (locals.var_rho * p.p1106);
        let assign15590_e21487: f64 = (1.0 + assign15590_e21486);
        let assign15590_e21491: f64 = (locals.var_rho_ref * p.p1106);
        let assign15590_e21492: f64 = (1.0 + assign15590_e21491);
        let assign15590_e21493: f64 = (assign15590_e21487 / assign15590_e21492);
        (assign15590_e21493, ((((locals.var_rho_dn3 * p.p1106) * assign15590_e21492) - (assign15590_e21487 * (locals.var_rho_ref_dn3 * p.p1106))) / (assign15590_e21492 * assign15590_e21492)), ((((locals.var_rho_dn4 * p.p1106) * assign15590_e21492) - (assign15590_e21487 * (locals.var_rho_ref_dn4 * p.p1106))) / (assign15590_e21492 * assign15590_e21492)), ((((locals.var_rho_dn5 * p.p1106) * assign15590_e21492) - (assign15590_e21487 * (locals.var_rho_ref_dn5 * p.p1106))) / (assign15590_e21492 * assign15590_e21492)), ((((locals.var_rho_dn6 * p.p1106) * assign15590_e21492) - (assign15590_e21487 * (locals.var_rho_ref_dn6 * p.p1106))) / (assign15590_e21492 * assign15590_e21492)), ((((locals.var_rho_dn7 * p.p1106) * assign15590_e21492) - (assign15590_e21487 * (locals.var_rho_ref_dn7 * p.p1106))) / (assign15590_e21492 * assign15590_e21492)), ((((locals.var_rho_dn8 * p.p1106) * assign15590_e21492) - (assign15590_e21487 * (locals.var_rho_ref_dn8 * p.p1106))) / (assign15590_e21492 * assign15590_e21492)), ((((locals.var_rho_dn9 * p.p1106) * assign15590_e21492) - (assign15590_e21487 * (locals.var_rho_ref_dn9 * p.p1106))) / (assign15590_e21492 * assign15590_e21492)), ((((locals.var_rho_dn10 * p.p1106) * assign15590_e21492) - (assign15590_e21487 * (locals.var_rho_ref_dn10 * p.p1106))) / (assign15590_e21492 * assign15590_e21492)), ((((locals.var_rho_dn11 * p.p1106) * assign15590_e21492) - (assign15590_e21487 * (locals.var_rho_ref_dn11 * p.p1106))) / (assign15590_e21492 * assign15590_e21492)),)
    } else {
        (locals.var_vsat_mult, locals.var_vsat_mult_dn3, locals.var_vsat_mult_dn4, locals.var_vsat_mult_dn5, locals.var_vsat_mult_dn6, locals.var_vsat_mult_dn7, locals.var_vsat_mult_dn8, locals.var_vsat_mult_dn9, locals.var_vsat_mult_dn10, locals.var_vsat_mult_dn11,)
    }
};
        locals.var_vsat_mult = assign15590_e21495;
        locals.var_vsat_mult_dn3 = assign15590_e21495_d_n3;
        locals.var_vsat_mult_dn4 = assign15590_e21495_d_n4;
        locals.var_vsat_mult_dn5 = assign15590_e21495_d_n5;
        locals.var_vsat_mult_dn6 = assign15590_e21495_d_n6;
        locals.var_vsat_mult_dn7 = assign15590_e21495_d_n7;
        locals.var_vsat_mult_dn8 = assign15590_e21495_d_n8;
        locals.var_vsat_mult_dn9 = assign15590_e21495_d_n9;
        locals.var_vsat_mult_dn10 = assign15590_e21495_d_n10;
        locals.var_vsat_mult_dn11 = assign15590_e21495_d_n11;
        locals.var_vsat_mult_rv = 0.0;

        let (assign15600_e21505, assign15600_e21505_d_n3, assign15600_e21505_d_n4, assign15600_e21505_d_n5, assign15600_e21505_d_n6, assign15600_e21505_d_n7, assign15600_e21505_d_n8, assign15600_e21505_d_n9, assign15600_e21505_d_n10, assign15600_e21505_d_n11,) = {
    if (locals.var_guard480 != 0.0) {
        let assign15600_e21499: f64 = (p.p1113 / locals.var_kstress_vth0);
        let assign15600_e21502: f64 = (locals.var_inv_od - locals.var_inv_odref);
        let assign15600_e21503: f64 = (assign15600_e21499 * assign15600_e21502);
        (assign15600_e21503, (((-((p.p1113 * locals.var_kstress_vth0_dn3) / (locals.var_kstress_vth0 * locals.var_kstress_vth0))) * assign15600_e21502) + (assign15600_e21499 * locals.var_inv_od_dn3)), (((-((p.p1113 * locals.var_kstress_vth0_dn4) / (locals.var_kstress_vth0 * locals.var_kstress_vth0))) * assign15600_e21502) + (assign15600_e21499 * locals.var_inv_od_dn4)), (((-((p.p1113 * locals.var_kstress_vth0_dn5) / (locals.var_kstress_vth0 * locals.var_kstress_vth0))) * assign15600_e21502) + (assign15600_e21499 * locals.var_inv_od_dn5)), (((-((p.p1113 * locals.var_kstress_vth0_dn6) / (locals.var_kstress_vth0 * locals.var_kstress_vth0))) * assign15600_e21502) + (assign15600_e21499 * locals.var_inv_od_dn6)), (((-((p.p1113 * locals.var_kstress_vth0_dn7) / (locals.var_kstress_vth0 * locals.var_kstress_vth0))) * assign15600_e21502) + (assign15600_e21499 * locals.var_inv_od_dn7)), (((-((p.p1113 * locals.var_kstress_vth0_dn8) / (locals.var_kstress_vth0 * locals.var_kstress_vth0))) * assign15600_e21502) + (assign15600_e21499 * locals.var_inv_od_dn8)), (((-((p.p1113 * locals.var_kstress_vth0_dn9) / (locals.var_kstress_vth0 * locals.var_kstress_vth0))) * assign15600_e21502) + (assign15600_e21499 * locals.var_inv_od_dn9)), (((-((p.p1113 * locals.var_kstress_vth0_dn10) / (locals.var_kstress_vth0 * locals.var_kstress_vth0))) * assign15600_e21502) + (assign15600_e21499 * locals.var_inv_od_dn10)), (((-((p.p1113 * locals.var_kstress_vth0_dn11) / (locals.var_kstress_vth0 * locals.var_kstress_vth0))) * assign15600_e21502) + (assign15600_e21499 * locals.var_inv_od_dn11)),)
    } else {
        (locals.var_vth0_stress, locals.var_vth0_stress_dn3, locals.var_vth0_stress_dn4, locals.var_vth0_stress_dn5, locals.var_vth0_stress_dn6, locals.var_vth0_stress_dn7, locals.var_vth0_stress_dn8, locals.var_vth0_stress_dn9, locals.var_vth0_stress_dn10, locals.var_vth0_stress_dn11,)
    }
};
        locals.var_vth0_stress = assign15600_e21505;
        locals.var_vth0_stress_dn3 = assign15600_e21505_d_n3;
        locals.var_vth0_stress_dn4 = assign15600_e21505_d_n4;
        locals.var_vth0_stress_dn5 = assign15600_e21505_d_n5;
        locals.var_vth0_stress_dn6 = assign15600_e21505_d_n6;
        locals.var_vth0_stress_dn7 = assign15600_e21505_d_n7;
        locals.var_vth0_stress_dn8 = assign15600_e21505_d_n8;
        locals.var_vth0_stress_dn9 = assign15600_e21505_d_n9;
        locals.var_vth0_stress_dn10 = assign15600_e21505_d_n10;
        locals.var_vth0_stress_dn11 = assign15600_e21505_d_n11;
        locals.var_vth0_stress_rv = 0.0;

        let (assign15610_e21517, assign15610_e21517_d_n3, assign15610_e21517_d_n4, assign15610_e21517_d_n5, assign15610_e21517_d_n6, assign15610_e21517_d_n7, assign15610_e21517_d_n8, assign15610_e21517_d_n9, assign15610_e21517_d_n10, assign15610_e21517_d_n11,) = {
    if (locals.var_guard480 != 0.0) {
        let assign15610_e21510: f64 = (locals.var_kstress_vth0).powf(p.p1120);
        let assign15610_e21511: f64 = (p.p1119 / assign15610_e21510);
        let assign15610_e21514: f64 = (locals.var_inv_od - locals.var_inv_odref);
        let assign15610_e21515: f64 = (assign15610_e21511 * assign15610_e21514);
        (assign15610_e21515, (((-((p.p1119 * if 0.0 == 0.0 && ((p.p1120) as f64).is_finite() && ((p.p1120) as f64).fract() == 0.0 { if p.p1120 == 0.0 { 0.0 } else { (p.p1120 * ((locals.var_kstress_vth0).powf(p.p1120 - 1.0) * locals.var_kstress_vth0_dn3)) } } else { (assign15610_e21510 * (p.p1120 * (locals.var_kstress_vth0_dn3 / locals.var_kstress_vth0))) }) / (assign15610_e21510 * assign15610_e21510))) * assign15610_e21514) + (assign15610_e21511 * locals.var_inv_od_dn3)), (((-((p.p1119 * if 0.0 == 0.0 && ((p.p1120) as f64).is_finite() && ((p.p1120) as f64).fract() == 0.0 { if p.p1120 == 0.0 { 0.0 } else { (p.p1120 * ((locals.var_kstress_vth0).powf(p.p1120 - 1.0) * locals.var_kstress_vth0_dn4)) } } else { (assign15610_e21510 * (p.p1120 * (locals.var_kstress_vth0_dn4 / locals.var_kstress_vth0))) }) / (assign15610_e21510 * assign15610_e21510))) * assign15610_e21514) + (assign15610_e21511 * locals.var_inv_od_dn4)), (((-((p.p1119 * if 0.0 == 0.0 && ((p.p1120) as f64).is_finite() && ((p.p1120) as f64).fract() == 0.0 { if p.p1120 == 0.0 { 0.0 } else { (p.p1120 * ((locals.var_kstress_vth0).powf(p.p1120 - 1.0) * locals.var_kstress_vth0_dn5)) } } else { (assign15610_e21510 * (p.p1120 * (locals.var_kstress_vth0_dn5 / locals.var_kstress_vth0))) }) / (assign15610_e21510 * assign15610_e21510))) * assign15610_e21514) + (assign15610_e21511 * locals.var_inv_od_dn5)), (((-((p.p1119 * if 0.0 == 0.0 && ((p.p1120) as f64).is_finite() && ((p.p1120) as f64).fract() == 0.0 { if p.p1120 == 0.0 { 0.0 } else { (p.p1120 * ((locals.var_kstress_vth0).powf(p.p1120 - 1.0) * locals.var_kstress_vth0_dn6)) } } else { (assign15610_e21510 * (p.p1120 * (locals.var_kstress_vth0_dn6 / locals.var_kstress_vth0))) }) / (assign15610_e21510 * assign15610_e21510))) * assign15610_e21514) + (assign15610_e21511 * locals.var_inv_od_dn6)), (((-((p.p1119 * if 0.0 == 0.0 && ((p.p1120) as f64).is_finite() && ((p.p1120) as f64).fract() == 0.0 { if p.p1120 == 0.0 { 0.0 } else { (p.p1120 * ((locals.var_kstress_vth0).powf(p.p1120 - 1.0) * locals.var_kstress_vth0_dn7)) } } else { (assign15610_e21510 * (p.p1120 * (locals.var_kstress_vth0_dn7 / locals.var_kstress_vth0))) }) / (assign15610_e21510 * assign15610_e21510))) * assign15610_e21514) + (assign15610_e21511 * locals.var_inv_od_dn7)), (((-((p.p1119 * if 0.0 == 0.0 && ((p.p1120) as f64).is_finite() && ((p.p1120) as f64).fract() == 0.0 { if p.p1120 == 0.0 { 0.0 } else { (p.p1120 * ((locals.var_kstress_vth0).powf(p.p1120 - 1.0) * locals.var_kstress_vth0_dn8)) } } else { (assign15610_e21510 * (p.p1120 * (locals.var_kstress_vth0_dn8 / locals.var_kstress_vth0))) }) / (assign15610_e21510 * assign15610_e21510))) * assign15610_e21514) + (assign15610_e21511 * locals.var_inv_od_dn8)), (((-((p.p1119 * if 0.0 == 0.0 && ((p.p1120) as f64).is_finite() && ((p.p1120) as f64).fract() == 0.0 { if p.p1120 == 0.0 { 0.0 } else { (p.p1120 * ((locals.var_kstress_vth0).powf(p.p1120 - 1.0) * locals.var_kstress_vth0_dn9)) } } else { (assign15610_e21510 * (p.p1120 * (locals.var_kstress_vth0_dn9 / locals.var_kstress_vth0))) }) / (assign15610_e21510 * assign15610_e21510))) * assign15610_e21514) + (assign15610_e21511 * locals.var_inv_od_dn9)), (((-((p.p1119 * if 0.0 == 0.0 && ((p.p1120) as f64).is_finite() && ((p.p1120) as f64).fract() == 0.0 { if p.p1120 == 0.0 { 0.0 } else { (p.p1120 * ((locals.var_kstress_vth0).powf(p.p1120 - 1.0) * locals.var_kstress_vth0_dn10)) } } else { (assign15610_e21510 * (p.p1120 * (locals.var_kstress_vth0_dn10 / locals.var_kstress_vth0))) }) / (assign15610_e21510 * assign15610_e21510))) * assign15610_e21514) + (assign15610_e21511 * locals.var_inv_od_dn10)), (((-((p.p1119 * if 0.0 == 0.0 && ((p.p1120) as f64).is_finite() && ((p.p1120) as f64).fract() == 0.0 { if p.p1120 == 0.0 { 0.0 } else { (p.p1120 * ((locals.var_kstress_vth0).powf(p.p1120 - 1.0) * locals.var_kstress_vth0_dn11)) } } else { (assign15610_e21510 * (p.p1120 * (locals.var_kstress_vth0_dn11 / locals.var_kstress_vth0))) }) / (assign15610_e21510 * assign15610_e21510))) * assign15610_e21514) + (assign15610_e21511 * locals.var_inv_od_dn11)),)
    } else {
        (locals.var_k2_stress, locals.var_k2_stress_dn3, locals.var_k2_stress_dn4, locals.var_k2_stress_dn5, locals.var_k2_stress_dn6, locals.var_k2_stress_dn7, locals.var_k2_stress_dn8, locals.var_k2_stress_dn9, locals.var_k2_stress_dn10, locals.var_k2_stress_dn11,)
    }
};
        locals.var_k2_stress = assign15610_e21517;
        locals.var_k2_stress_dn3 = assign15610_e21517_d_n3;
        locals.var_k2_stress_dn4 = assign15610_e21517_d_n4;
        locals.var_k2_stress_dn5 = assign15610_e21517_d_n5;
        locals.var_k2_stress_dn6 = assign15610_e21517_d_n6;
        locals.var_k2_stress_dn7 = assign15610_e21517_d_n7;
        locals.var_k2_stress_dn8 = assign15610_e21517_d_n8;
        locals.var_k2_stress_dn9 = assign15610_e21517_d_n9;
        locals.var_k2_stress_dn10 = assign15610_e21517_d_n10;
        locals.var_k2_stress_dn11 = assign15610_e21517_d_n11;
        locals.var_k2_stress_rv = 0.0;

    }
}
