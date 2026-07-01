#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_12(
        p: &Parameters,
        var_guard153: f64,
        var_guard154: f64,
        var_guard169: f64,
        var_guard31: f64,
        var_ndigat_i: f64,
        var_nfagat_i: f64,
        var_nfasti_i: f64,
        var_nin: f64,
        var_phitdinv: f64,
        var_vmax: f64,
        var_dfn_sl_slot: &mut f64,
        var_dfn_sl_dn0_slot: &mut f64,
        var_dfn_sl_dn2_slot: &mut f64,
        var_dfn_sl_rv_slot: &mut f64,
        var_dfn_su_slot: &mut f64,
        var_dfn_su_dn0_slot: &mut f64,
        var_dfn_su_dn2_slot: &mut f64,
        var_dfn_su_rv_slot: &mut f64,
        var_dnj1_dv_slot: &mut f64,
        var_dnj1_dv_dn0_slot: &mut f64,
        var_dnj1_dv_dn2_slot: &mut f64,
        var_dnj1_dv_rv_slot: &mut f64,
        var_dvmax_over_phitd_dv_slot: &mut f64,
        var_dvmax_over_phitd_dv_dn0_slot: &mut f64,
        var_dvmax_over_phitd_dv_dn2_slot: &mut f64,
        var_dvmax_over_phitd_dv_rv_slot: &mut f64,
        var_guard172_slot: &mut f64,
        var_guard172_rv_slot: &mut f64,
        var_nj0_slot: &mut f64,
        var_nj0_dn0_slot: &mut f64,
        var_nj0_dn2_slot: &mut f64,
        var_nj0_rv_slot: &mut f64,
        var_nj1_slot: &mut f64,
        var_nj1_dn0_slot: &mut f64,
        var_nj1_dn2_slot: &mut f64,
        var_nj1_rv_slot: &mut f64,
        var_nja10_slot: &mut f64,
        var_nja10_dn0_slot: &mut f64,
        var_nja10_dn2_slot: &mut f64,
        var_nja10_rv_slot: &mut f64,
        var_nja11_slot: &mut f64,
        var_nja11_dn0_slot: &mut f64,
        var_nja11_dn2_slot: &mut f64,
        var_nja11_rv_slot: &mut f64,
        var_pnn0_slot: &mut f64,
        var_pnn0_rv_slot: &mut f64,
        var_tmf1_slot: &mut f64,
        var_tmf1_dn0_slot: &mut f64,
        var_tmf1_dn2_slot: &mut f64,
        var_tmf1_rv_slot: &mut f64,
        var_tmf2_slot: &mut f64,
        var_tmf2_dn0_slot: &mut f64,
        var_tmf2_dn2_slot: &mut f64,
        var_tmf2_rv_slot: &mut f64,
        var_vha1_slot: &mut f64,
        var_vha1_rv_slot: &mut f64,
    ) {
        let mut var_dfn_sl: f64 = *var_dfn_sl_slot;
        let mut var_dfn_sl_dn0: f64 = *var_dfn_sl_dn0_slot;
        let mut var_dfn_sl_dn2: f64 = *var_dfn_sl_dn2_slot;
        let mut var_dfn_sl_rv: f64 = *var_dfn_sl_rv_slot;
        let mut var_dfn_su: f64 = *var_dfn_su_slot;
        let mut var_dfn_su_dn0: f64 = *var_dfn_su_dn0_slot;
        let mut var_dfn_su_dn2: f64 = *var_dfn_su_dn2_slot;
        let mut var_dfn_su_rv: f64 = *var_dfn_su_rv_slot;
        let mut var_dnj1_dv: f64 = *var_dnj1_dv_slot;
        let mut var_dnj1_dv_dn0: f64 = *var_dnj1_dv_dn0_slot;
        let mut var_dnj1_dv_dn2: f64 = *var_dnj1_dv_dn2_slot;
        let mut var_dnj1_dv_rv: f64 = *var_dnj1_dv_rv_slot;
        let mut var_dvmax_over_phitd_dv: f64 = *var_dvmax_over_phitd_dv_slot;
        let mut var_dvmax_over_phitd_dv_dn0: f64 = *var_dvmax_over_phitd_dv_dn0_slot;
        let mut var_dvmax_over_phitd_dv_dn2: f64 = *var_dvmax_over_phitd_dv_dn2_slot;
        let mut var_dvmax_over_phitd_dv_rv: f64 = *var_dvmax_over_phitd_dv_rv_slot;
        let mut var_guard172: f64 = *var_guard172_slot;
        let mut var_guard172_rv: f64 = *var_guard172_rv_slot;
        let mut var_nj0: f64 = *var_nj0_slot;
        let mut var_nj0_dn0: f64 = *var_nj0_dn0_slot;
        let mut var_nj0_dn2: f64 = *var_nj0_dn2_slot;
        let mut var_nj0_rv: f64 = *var_nj0_rv_slot;
        let mut var_nj1: f64 = *var_nj1_slot;
        let mut var_nj1_dn0: f64 = *var_nj1_dn0_slot;
        let mut var_nj1_dn2: f64 = *var_nj1_dn2_slot;
        let mut var_nj1_rv: f64 = *var_nj1_rv_slot;
        let mut var_nja10: f64 = *var_nja10_slot;
        let mut var_nja10_dn0: f64 = *var_nja10_dn0_slot;
        let mut var_nja10_dn2: f64 = *var_nja10_dn2_slot;
        let mut var_nja10_rv: f64 = *var_nja10_rv_slot;
        let mut var_nja11: f64 = *var_nja11_slot;
        let mut var_nja11_dn0: f64 = *var_nja11_dn0_slot;
        let mut var_nja11_dn2: f64 = *var_nja11_dn2_slot;
        let mut var_nja11_rv: f64 = *var_nja11_rv_slot;
        let mut var_pnn0: f64 = *var_pnn0_slot;
        let mut var_pnn0_rv: f64 = *var_pnn0_rv_slot;
        let mut var_tmf1: f64 = *var_tmf1_slot;
        let mut var_tmf1_dn0: f64 = *var_tmf1_dn0_slot;
        let mut var_tmf1_dn2: f64 = *var_tmf1_dn2_slot;
        let mut var_tmf1_rv: f64 = *var_tmf1_rv_slot;
        let mut var_tmf2: f64 = *var_tmf2_slot;
        let mut var_tmf2_dn0: f64 = *var_tmf2_dn0_slot;
        let mut var_tmf2_dn2: f64 = *var_tmf2_dn2_slot;
        let mut var_tmf2_rv: f64 = *var_tmf2_rv_slot;
        let mut var_vha1: f64 = *var_vha1_slot;
        let mut var_vha1_rv: f64 = *var_vha1_rv_slot;

        let (assign10480_e13350, assign10480_e13350_d_n0, assign10480_e13350_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 == 0.0)) && (var_guard169 != 0.0)) {
        let assign10480_e13345: f64 = (var_tmf1 * var_tmf1);
        let assign10480_e13347: f64 = (assign10480_e13345 + var_tmf2);
        let assign10480_e13348: f64 = (assign10480_e13347).sqrt();
        (assign10480_e13348, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign10480_e13348)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign10480_e13348)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign10480_e13350;
        var_tmf2_dn0 = assign10480_e13350_d_n0;
        var_tmf2_dn2 = assign10480_e13350_d_n2;
        var_tmf2_rv = 0.0;

        let (assign10490_e13367, assign10490_e13367_d_n0, assign10490_e13367_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 == 0.0)) && (var_guard169 != 0.0)) {
        let assign10490_e13363: f64 = (var_tmf1 + var_tmf2);
        let assign10490_e13364: f64 = (0.5 * assign10490_e13363);
        let assign10490_e13365: f64 = (var_nfasti_i + assign10490_e13364);
        (assign10490_e13365, (0.5 * (var_tmf1_dn0 + var_tmf2_dn0)), (0.5 * (var_tmf1_dn2 + var_tmf2_dn2)),)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn2,)
    }
};
        var_nj0 = assign10490_e13367;
        var_nj0_dn0 = assign10490_e13367_d_n0;
        var_nj0_dn2 = assign10490_e13367_d_n2;
        var_nj0_rv = 0.0;

        let (assign10500_e13382, assign10500_e13382_d_n0, assign10500_e13382_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 == 0.0)) && (var_guard169 != 0.0)) {
        let assign10500_e13378: f64 = (p.p86 * var_dfn_su);
        let assign10500_e13380: f64 = (assign10500_e13378 * var_dfn_sl);
        (assign10500_e13380, (((p.p86 * var_dfn_su_dn0) * var_dfn_sl) + (assign10500_e13378 * var_dfn_sl_dn0)), (((p.p86 * var_dfn_su_dn2) * var_dfn_sl) + (assign10500_e13378 * var_dfn_sl_dn2)),)
    } else {
        (var_dnj1_dv, var_dnj1_dv_dn0, var_dnj1_dv_dn2,)
    }
};
        var_dnj1_dv = assign10500_e13382;
        var_dnj1_dv_dn0 = assign10500_e13382_d_n0;
        var_dnj1_dv_dn2 = assign10500_e13382_d_n2;
        var_dnj1_dv_rv = 0.0;

        let (assign10510_e13394, assign10510_e13394_d_n0, assign10510_e13394_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 == 0.0)) && (var_guard169 == 0.0)) {
        (var_nfasti_i, 0.0, 0.0,)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn2,)
    }
};
        var_nj0 = assign10510_e13394;
        var_nj0_dn0 = assign10510_e13394_d_n0;
        var_nj0_dn2 = assign10510_e13394_d_n2;
        var_nj0_rv = 0.0;

        let (assign10520_e13406, assign10520_e13406_d_n0, assign10520_e13406_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 == 0.0)) && (var_guard169 == 0.0)) {
        (var_nfasti_i, 0.0, 0.0,)
    } else {
        (var_nj1, var_nj1_dn0, var_nj1_dn2,)
    }
};
        var_nj1 = assign10520_e13406;
        var_nj1_dn0 = assign10520_e13406_d_n0;
        var_nj1_dn2 = assign10520_e13406_d_n2;
        var_nj1_rv = 0.0;

        let (assign10530_e13418, assign10530_e13418_d_n0, assign10530_e13418_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 == 0.0)) && (var_guard169 == 0.0)) {
        (0.0, 0.0, 0.0,)
    } else {
        (var_dnj1_dv, var_dnj1_dv_dn0, var_dnj1_dv_dn2,)
    }
};
        var_dnj1_dv = assign10530_e13418;
        var_dnj1_dv_dn0 = assign10530_e13418_d_n0;
        var_dnj1_dv_dn2 = assign10530_e13418_d_n2;
        var_dnj1_dv_rv = 0.0;

        let (assign10590_e13667, assign10590_e13667_d_n0, assign10590_e13667_d_n2,) = {
    if (((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 == 0.0)) {
        let assign10590_e13651: f64 = (var_vmax * var_dnj1_dv);
        let assign10590_e13652: f64 = (var_nj1 - assign10590_e13651);
        let assign10590_e13655: f64 = (var_nj1 * var_nj1);
        let assign10590_e13656: f64 = (assign10590_e13652 / assign10590_e13655);
        let assign10590_e13659: f64 = (var_vha1 * var_dnj1_dv);
        let assign10590_e13662: f64 = (var_nj0 * p.p85);
        let assign10590_e13663: f64 = (assign10590_e13659 / assign10590_e13662);
        let assign10590_e13664: f64 = (assign10590_e13656 + assign10590_e13663);
        let assign10590_e13665: f64 = (var_phitdinv * assign10590_e13664);
        (assign10590_e13665, (var_phitdinv * (((((var_nj1_dn0 - (var_vmax * var_dnj1_dv_dn0)) * assign10590_e13655) - (assign10590_e13652 * ((var_nj1_dn0 * var_nj1) + (var_nj1 * var_nj1_dn0)))) / (assign10590_e13655 * assign10590_e13655)) + ((((var_vha1 * var_dnj1_dv_dn0) * assign10590_e13662) - (assign10590_e13659 * (var_nj0_dn0 * p.p85))) / (assign10590_e13662 * assign10590_e13662)))), (var_phitdinv * (((((var_nj1_dn2 - (var_vmax * var_dnj1_dv_dn2)) * assign10590_e13655) - (assign10590_e13652 * ((var_nj1_dn2 * var_nj1) + (var_nj1 * var_nj1_dn2)))) / (assign10590_e13655 * assign10590_e13655)) + ((((var_vha1 * var_dnj1_dv_dn2) * assign10590_e13662) - (assign10590_e13659 * (var_nj0_dn2 * p.p85))) / (assign10590_e13662 * assign10590_e13662)))),)
    } else {
        (var_dvmax_over_phitd_dv, var_dvmax_over_phitd_dv_dn0, var_dvmax_over_phitd_dv_dn2,)
    }
};
        var_dvmax_over_phitd_dv = assign10590_e13667;
        var_dvmax_over_phitd_dv_dn0 = assign10590_e13667_d_n0;
        var_dvmax_over_phitd_dv_dn2 = assign10590_e13667_d_n2;
        var_dvmax_over_phitd_dv_rv = 0.0;

        let (assign10610_e13697,) = {
    if (((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 == 0.0)) {
        let assign10610_e13693: f64 = (var_nin * var_nin);
        let assign10610_e13695: f64 = (assign10610_e13693 / var_ndigat_i);
        (assign10610_e13695,)
    } else {
        (var_pnn0,)
    }
};
        var_pnn0 = assign10610_e13697;
        var_pnn0_rv = 0.0;

        let (assign10620_e13713,) = {
    if (((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 == 0.0)) {
        let assign10620_e13706: f64 = (var_nfagat_i / var_phitdinv);
        let assign10620_e13709: f64 = (var_ndigat_i / var_pnn0);
        let assign10620_e13710: f64 = (assign10620_e13709).ln();
        let assign10620_e13711: f64 = (assign10620_e13706 * assign10620_e13710);
        (assign10620_e13711,)
    } else {
        (var_vha1,)
    }
};
        var_vha1 = assign10620_e13713;
        var_vha1_rv = 0.0;

        let assign10630_e13716: f64 = if var_nfagat_i < p.p85 { 1.0 } else { 0.0 };
        var_guard172 = assign10630_e13716;
        var_guard172_rv = 0.0;

        let (assign10640_e13733, assign10640_e13733_d_n0, assign10640_e13733_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 == 0.0)) && (var_guard172 != 0.0)) {
        let assign10640_e13728: f64 = (var_vmax - var_vha1);
        let assign10640_e13729: f64 = (p.p86 * assign10640_e13728);
        let assign10640_e13731: f64 = (assign10640_e13729 + var_nfagat_i);
        (assign10640_e13731, 0.0, 0.0,)
    } else {
        (var_nja10, var_nja10_dn0, var_nja10_dn2,)
    }
};
        var_nja10 = assign10640_e13733;
        var_nja10_dn0 = assign10640_e13733_d_n0;
        var_nja10_dn2 = assign10640_e13733_d_n2;
        var_nja10_rv = 0.0;

        let (assign10650_e13748, assign10650_e13748_d_n0, assign10650_e13748_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 == 0.0)) && (var_guard172 != 0.0)) {
        let assign10650_e13745: f64 = (p.p86 * var_vha1);
        let assign10650_e13746: f64 = (var_nfagat_i - assign10650_e13745);
        (assign10650_e13746, 0.0, 0.0,)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn2,)
    }
};
        var_nj0 = assign10650_e13748;
        var_nj0_dn0 = assign10650_e13748_d_n0;
        var_nj0_dn2 = assign10650_e13748_d_n2;
        var_nj0_rv = 0.0;

        let (assign10660_e13763, assign10660_e13763_d_n0, assign10660_e13763_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 == 0.0)) && (var_guard172 != 0.0)) {
        let assign10660_e13759: f64 = (p.p85 - var_nja10);
        let assign10660_e13761: f64 = (assign10660_e13759 - 0.01);
        (assign10660_e13761, (-var_nja10_dn0), (-var_nja10_dn2),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2,)
    }
};
        var_tmf1 = assign10660_e13763;
        var_tmf1_dn0 = assign10660_e13763_d_n0;
        var_tmf1_dn2 = assign10660_e13763_d_n2;
        var_tmf1_rv = 0.0;

        let (assign10670_e13778, assign10670_e13778_d_n0, assign10670_e13778_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 == 0.0)) && (var_guard172 != 0.0)) {
        let assign10670_e13774: f64 = (4.0 * p.p85);
        let assign10670_e13776: f64 = (assign10670_e13774 * 0.01);
        (assign10670_e13776, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign10670_e13778;
        var_tmf2_dn0 = assign10670_e13778_d_n0;
        var_tmf2_dn2 = assign10670_e13778_d_n2;
        var_tmf2_rv = 0.0;

        let (assign10680_e13795, assign10680_e13795_d_n0, assign10680_e13795_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 == 0.0)) && (var_guard172 != 0.0)) {
        let (assign10680_e13793, assign10680_e13793_d_n0, assign10680_e13793_d_n2,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
            } else {
                let assign10680_e13792: f64 = (-var_tmf2);
                (assign10680_e13792, (-var_tmf2_dn0), (-var_tmf2_dn2),)
            }
        };
        (assign10680_e13793, assign10680_e13793_d_n0, assign10680_e13793_d_n2,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign10680_e13795;
        var_tmf2_dn0 = assign10680_e13795_d_n0;
        var_tmf2_dn2 = assign10680_e13795_d_n2;
        var_tmf2_rv = 0.0;

        let (assign10690_e13811, assign10690_e13811_d_n0, assign10690_e13811_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 == 0.0)) && (var_guard172 != 0.0)) {
        let assign10690_e13806: f64 = (var_tmf1 * var_tmf1);
        let assign10690_e13808: f64 = (assign10690_e13806 + var_tmf2);
        let assign10690_e13809: f64 = (assign10690_e13808).sqrt();
        (assign10690_e13809, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign10690_e13809)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign10690_e13809)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign10690_e13811;
        var_tmf2_dn0 = assign10690_e13811_d_n0;
        var_tmf2_dn2 = assign10690_e13811_d_n2;
        var_tmf2_rv = 0.0;

        let (assign10700_e13828, assign10700_e13828_d_n0, assign10700_e13828_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 == 0.0)) && (var_guard172 != 0.0)) {
        let assign10700_e13824: f64 = (var_tmf1 / var_tmf2);
        let assign10700_e13825: f64 = (1.0 + assign10700_e13824);
        let assign10700_e13826: f64 = (0.5 * assign10700_e13825);
        (assign10700_e13826, (0.5 * (((var_tmf1_dn0 * var_tmf2) - (var_tmf1 * var_tmf2_dn0)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn2 * var_tmf2) - (var_tmf1 * var_tmf2_dn2)) / (var_tmf2 * var_tmf2))),)
    } else {
        (var_dfn_su, var_dfn_su_dn0, var_dfn_su_dn2,)
    }
};
        var_dfn_su = assign10700_e13828;
        var_dfn_su_dn0 = assign10700_e13828_d_n0;
        var_dfn_su_dn2 = assign10700_e13828_d_n2;
        var_dfn_su_rv = 0.0;

        let (assign10710_e13845, assign10710_e13845_d_n0, assign10710_e13845_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 == 0.0)) && (var_guard172 != 0.0)) {
        let assign10710_e13841: f64 = (var_tmf1 + var_tmf2);
        let assign10710_e13842: f64 = (0.5 * assign10710_e13841);
        let assign10710_e13843: f64 = (p.p85 - assign10710_e13842);
        (assign10710_e13843, (-(0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (-(0.5 * (var_tmf1_dn2 + var_tmf2_dn2))),)
    } else {
        (var_nja11, var_nja11_dn0, var_nja11_dn2,)
    }
};
        var_nja11 = assign10710_e13845;
        var_nja11_dn0 = assign10710_e13845_d_n0;
        var_nja11_dn2 = assign10710_e13845_d_n2;
        var_nja11_rv = 0.0;

        let (assign10720_e13860, assign10720_e13860_d_n0, assign10720_e13860_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 == 0.0)) && (var_guard172 != 0.0)) {
        let assign10720_e13856: f64 = (var_nja11 - var_nfagat_i);
        let assign10720_e13858: f64 = (assign10720_e13856 - 0.01);
        (assign10720_e13858, var_nja11_dn0, var_nja11_dn2,)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2,)
    }
};
        var_tmf1 = assign10720_e13860;
        var_tmf1_dn0 = assign10720_e13860_d_n0;
        var_tmf1_dn2 = assign10720_e13860_d_n2;
        var_tmf1_rv = 0.0;

        let (assign10730_e13875, assign10730_e13875_d_n0, assign10730_e13875_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 == 0.0)) && (var_guard172 != 0.0)) {
        let assign10730_e13871: f64 = (4.0 * var_nfagat_i);
        let assign10730_e13873: f64 = (assign10730_e13871 * 0.01);
        (assign10730_e13873, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign10730_e13875;
        var_tmf2_dn0 = assign10730_e13875_d_n0;
        var_tmf2_dn2 = assign10730_e13875_d_n2;
        var_tmf2_rv = 0.0;

        let (assign10740_e13892, assign10740_e13892_d_n0, assign10740_e13892_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 == 0.0)) && (var_guard172 != 0.0)) {
        let (assign10740_e13890, assign10740_e13890_d_n0, assign10740_e13890_d_n2,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
            } else {
                let assign10740_e13889: f64 = (-var_tmf2);
                (assign10740_e13889, (-var_tmf2_dn0), (-var_tmf2_dn2),)
            }
        };
        (assign10740_e13890, assign10740_e13890_d_n0, assign10740_e13890_d_n2,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign10740_e13892;
        var_tmf2_dn0 = assign10740_e13892_d_n0;
        var_tmf2_dn2 = assign10740_e13892_d_n2;
        var_tmf2_rv = 0.0;

        let (assign10750_e13908, assign10750_e13908_d_n0, assign10750_e13908_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 == 0.0)) && (var_guard172 != 0.0)) {
        let assign10750_e13903: f64 = (var_tmf1 * var_tmf1);
        let assign10750_e13905: f64 = (assign10750_e13903 + var_tmf2);
        let assign10750_e13906: f64 = (assign10750_e13905).sqrt();
        (assign10750_e13906, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign10750_e13906)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign10750_e13906)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign10750_e13908;
        var_tmf2_dn0 = assign10750_e13908_d_n0;
        var_tmf2_dn2 = assign10750_e13908_d_n2;
        var_tmf2_rv = 0.0;

        let (assign10760_e13925, assign10760_e13925_d_n0, assign10760_e13925_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 == 0.0)) && (var_guard172 != 0.0)) {
        let assign10760_e13921: f64 = (var_tmf1 / var_tmf2);
        let assign10760_e13922: f64 = (1.0 + assign10760_e13921);
        let assign10760_e13923: f64 = (0.5 * assign10760_e13922);
        (assign10760_e13923, (0.5 * (((var_tmf1_dn0 * var_tmf2) - (var_tmf1 * var_tmf2_dn0)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn2 * var_tmf2) - (var_tmf1 * var_tmf2_dn2)) / (var_tmf2 * var_tmf2))),)
    } else {
        (var_dfn_sl, var_dfn_sl_dn0, var_dfn_sl_dn2,)
    }
};
        var_dfn_sl = assign10760_e13925;
        var_dfn_sl_dn0 = assign10760_e13925_d_n0;
        var_dfn_sl_dn2 = assign10760_e13925_d_n2;
        var_dfn_sl_rv = 0.0;

        let (assign10770_e13942, assign10770_e13942_d_n0, assign10770_e13942_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 == 0.0)) && (var_guard172 != 0.0)) {
        let assign10770_e13938: f64 = (var_tmf1 + var_tmf2);
        let assign10770_e13939: f64 = (0.5 * assign10770_e13938);
        let assign10770_e13940: f64 = (var_nfagat_i + assign10770_e13939);
        (assign10770_e13940, (0.5 * (var_tmf1_dn0 + var_tmf2_dn0)), (0.5 * (var_tmf1_dn2 + var_tmf2_dn2)),)
    } else {
        (var_nj1, var_nj1_dn0, var_nj1_dn2,)
    }
};
        var_nj1 = assign10770_e13942;
        var_nj1_dn0 = assign10770_e13942_d_n0;
        var_nj1_dn2 = assign10770_e13942_d_n2;
        var_nj1_rv = 0.0;

        let (assign10780_e13957, assign10780_e13957_d_n0, assign10780_e13957_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 == 0.0)) && (var_guard172 != 0.0)) {
        let assign10780_e13953: f64 = (p.p85 - var_nj0);
        let assign10780_e13955: f64 = (assign10780_e13953 - 0.01);
        (assign10780_e13955, (-var_nj0_dn0), (-var_nj0_dn2),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2,)
    }
};
        var_tmf1 = assign10780_e13957;
        var_tmf1_dn0 = assign10780_e13957_d_n0;
        var_tmf1_dn2 = assign10780_e13957_d_n2;
        var_tmf1_rv = 0.0;

        let (assign10790_e13972, assign10790_e13972_d_n0, assign10790_e13972_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 == 0.0)) && (var_guard172 != 0.0)) {
        let assign10790_e13968: f64 = (4.0 * p.p85);
        let assign10790_e13970: f64 = (assign10790_e13968 * 0.01);
        (assign10790_e13970, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign10790_e13972;
        var_tmf2_dn0 = assign10790_e13972_d_n0;
        var_tmf2_dn2 = assign10790_e13972_d_n2;
        var_tmf2_rv = 0.0;

        let (assign10800_e13989, assign10800_e13989_d_n0, assign10800_e13989_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 == 0.0)) && (var_guard172 != 0.0)) {
        let (assign10800_e13987, assign10800_e13987_d_n0, assign10800_e13987_d_n2,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
            } else {
                let assign10800_e13986: f64 = (-var_tmf2);
                (assign10800_e13986, (-var_tmf2_dn0), (-var_tmf2_dn2),)
            }
        };
        (assign10800_e13987, assign10800_e13987_d_n0, assign10800_e13987_d_n2,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign10800_e13989;
        var_tmf2_dn0 = assign10800_e13989_d_n0;
        var_tmf2_dn2 = assign10800_e13989_d_n2;
        var_tmf2_rv = 0.0;

        let (assign10810_e14005, assign10810_e14005_d_n0, assign10810_e14005_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 == 0.0)) && (var_guard172 != 0.0)) {
        let assign10810_e14000: f64 = (var_tmf1 * var_tmf1);
        let assign10810_e14002: f64 = (assign10810_e14000 + var_tmf2);
        let assign10810_e14003: f64 = (assign10810_e14002).sqrt();
        (assign10810_e14003, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign10810_e14003)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign10810_e14003)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign10810_e14005;
        var_tmf2_dn0 = assign10810_e14005_d_n0;
        var_tmf2_dn2 = assign10810_e14005_d_n2;
        var_tmf2_rv = 0.0;

        let (assign10820_e14022, assign10820_e14022_d_n0, assign10820_e14022_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 == 0.0)) && (var_guard172 != 0.0)) {
        let assign10820_e14018: f64 = (var_tmf1 + var_tmf2);
        let assign10820_e14019: f64 = (0.5 * assign10820_e14018);
        let assign10820_e14020: f64 = (p.p85 - assign10820_e14019);
        (assign10820_e14020, (-(0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (-(0.5 * (var_tmf1_dn2 + var_tmf2_dn2))),)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn2,)
    }
};
        var_nj0 = assign10820_e14022;
        var_nj0_dn0 = assign10820_e14022_d_n0;
        var_nj0_dn2 = assign10820_e14022_d_n2;
        var_nj0_rv = 0.0;

        let (assign10830_e14037, assign10830_e14037_d_n0, assign10830_e14037_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 == 0.0)) && (var_guard172 != 0.0)) {
        let assign10830_e14033: f64 = (var_nj0 - var_nfagat_i);
        let assign10830_e14035: f64 = (assign10830_e14033 - 0.01);
        (assign10830_e14035, var_nj0_dn0, var_nj0_dn2,)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2,)
    }
};
        var_tmf1 = assign10830_e14037;
        var_tmf1_dn0 = assign10830_e14037_d_n0;
        var_tmf1_dn2 = assign10830_e14037_d_n2;
        var_tmf1_rv = 0.0;

        let (assign10840_e14052, assign10840_e14052_d_n0, assign10840_e14052_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 == 0.0)) && (var_guard172 != 0.0)) {
        let assign10840_e14048: f64 = (4.0 * var_nfagat_i);
        let assign10840_e14050: f64 = (assign10840_e14048 * 0.01);
        (assign10840_e14050, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign10840_e14052;
        var_tmf2_dn0 = assign10840_e14052_d_n0;
        var_tmf2_dn2 = assign10840_e14052_d_n2;
        var_tmf2_rv = 0.0;

        let (assign10850_e14069, assign10850_e14069_d_n0, assign10850_e14069_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 == 0.0)) && (var_guard172 != 0.0)) {
        let (assign10850_e14067, assign10850_e14067_d_n0, assign10850_e14067_d_n2,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
            } else {
                let assign10850_e14066: f64 = (-var_tmf2);
                (assign10850_e14066, (-var_tmf2_dn0), (-var_tmf2_dn2),)
            }
        };
        (assign10850_e14067, assign10850_e14067_d_n0, assign10850_e14067_d_n2,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign10850_e14069;
        var_tmf2_dn0 = assign10850_e14069_d_n0;
        var_tmf2_dn2 = assign10850_e14069_d_n2;
        var_tmf2_rv = 0.0;

        let (assign10860_e14085, assign10860_e14085_d_n0, assign10860_e14085_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 == 0.0)) && (var_guard172 != 0.0)) {
        let assign10860_e14080: f64 = (var_tmf1 * var_tmf1);
        let assign10860_e14082: f64 = (assign10860_e14080 + var_tmf2);
        let assign10860_e14083: f64 = (assign10860_e14082).sqrt();
        (assign10860_e14083, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign10860_e14083)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign10860_e14083)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign10860_e14085;
        var_tmf2_dn0 = assign10860_e14085_d_n0;
        var_tmf2_dn2 = assign10860_e14085_d_n2;
        var_tmf2_rv = 0.0;

        let (assign10870_e14102, assign10870_e14102_d_n0, assign10870_e14102_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 == 0.0)) && (var_guard172 != 0.0)) {
        let assign10870_e14098: f64 = (var_tmf1 + var_tmf2);
        let assign10870_e14099: f64 = (0.5 * assign10870_e14098);
        let assign10870_e14100: f64 = (var_nfagat_i + assign10870_e14099);
        (assign10870_e14100, (0.5 * (var_tmf1_dn0 + var_tmf2_dn0)), (0.5 * (var_tmf1_dn2 + var_tmf2_dn2)),)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn2,)
    }
};
        var_nj0 = assign10870_e14102;
        var_nj0_dn0 = assign10870_e14102_d_n0;
        var_nj0_dn2 = assign10870_e14102_d_n2;
        var_nj0_rv = 0.0;

        *var_dfn_sl_slot = var_dfn_sl;
        *var_dfn_sl_dn0_slot = var_dfn_sl_dn0;
        *var_dfn_sl_dn2_slot = var_dfn_sl_dn2;
        *var_dfn_sl_rv_slot = var_dfn_sl_rv;
        *var_dfn_su_slot = var_dfn_su;
        *var_dfn_su_dn0_slot = var_dfn_su_dn0;
        *var_dfn_su_dn2_slot = var_dfn_su_dn2;
        *var_dfn_su_rv_slot = var_dfn_su_rv;
        *var_dnj1_dv_slot = var_dnj1_dv;
        *var_dnj1_dv_dn0_slot = var_dnj1_dv_dn0;
        *var_dnj1_dv_dn2_slot = var_dnj1_dv_dn2;
        *var_dnj1_dv_rv_slot = var_dnj1_dv_rv;
        *var_dvmax_over_phitd_dv_slot = var_dvmax_over_phitd_dv;
        *var_dvmax_over_phitd_dv_dn0_slot = var_dvmax_over_phitd_dv_dn0;
        *var_dvmax_over_phitd_dv_dn2_slot = var_dvmax_over_phitd_dv_dn2;
        *var_dvmax_over_phitd_dv_rv_slot = var_dvmax_over_phitd_dv_rv;
        *var_guard172_slot = var_guard172;
        *var_guard172_rv_slot = var_guard172_rv;
        *var_nj0_slot = var_nj0;
        *var_nj0_dn0_slot = var_nj0_dn0;
        *var_nj0_dn2_slot = var_nj0_dn2;
        *var_nj0_rv_slot = var_nj0_rv;
        *var_nj1_slot = var_nj1;
        *var_nj1_dn0_slot = var_nj1_dn0;
        *var_nj1_dn2_slot = var_nj1_dn2;
        *var_nj1_rv_slot = var_nj1_rv;
        *var_nja10_slot = var_nja10;
        *var_nja10_dn0_slot = var_nja10_dn0;
        *var_nja10_dn2_slot = var_nja10_dn2;
        *var_nja10_rv_slot = var_nja10_rv;
        *var_nja11_slot = var_nja11;
        *var_nja11_dn0_slot = var_nja11_dn0;
        *var_nja11_dn2_slot = var_nja11_dn2;
        *var_nja11_rv_slot = var_nja11_rv;
        *var_pnn0_slot = var_pnn0;
        *var_pnn0_rv_slot = var_pnn0_rv;
        *var_tmf1_slot = var_tmf1;
        *var_tmf1_dn0_slot = var_tmf1_dn0;
        *var_tmf1_dn2_slot = var_tmf1_dn2;
        *var_tmf1_rv_slot = var_tmf1_rv;
        *var_tmf2_slot = var_tmf2;
        *var_tmf2_dn0_slot = var_tmf2_dn0;
        *var_tmf2_dn2_slot = var_tmf2_dn2;
        *var_tmf2_rv_slot = var_tmf2_rv;
        *var_vha1_slot = var_vha1;
        *var_vha1_rv_slot = var_vha1_rv;
    }

    pub(super) fn stamp_reactive_block_13(
        p: &Parameters,
        var_ab_i: f64,
        var_dfn_sl: f64,
        var_dfn_sl_dn0: f64,
        var_dfn_sl_dn2: f64,
        var_dfn_su: f64,
        var_dfn_su_dn0: f64,
        var_dfn_su_dn2: f64,
        var_guard153: f64,
        var_guard154: f64,
        var_guard172: f64,
        var_guard31: f64,
        var_lg_i: f64,
        var_ls_i: f64,
        var_ndibot_i: f64,
        var_nfabot_i: f64,
        var_nfagat_i: f64,
        var_nin: f64,
        var_phitdinv: f64,
        var_v3: f64,
        var_vmax: f64,
        var_dnj1_dv_slot: &mut f64,
        var_dnj1_dv_dn0_slot: &mut f64,
        var_dnj1_dv_dn2_slot: &mut f64,
        var_dnj1_dv_rv_slot: &mut f64,
        var_dvmax_over_phitd_dv_slot: &mut f64,
        var_dvmax_over_phitd_dv_dn0_slot: &mut f64,
        var_dvmax_over_phitd_dv_dn2_slot: &mut f64,
        var_dvmax_over_phitd_dv_rv_slot: &mut f64,
        var_guard230_slot: &mut f64,
        var_guard230_rv_slot: &mut f64,
        var_guard231_slot: &mut f64,
        var_guard231_rv_slot: &mut f64,
        var_guard234_slot: &mut f64,
        var_guard234_rv_slot: &mut f64,
        var_guard235_slot: &mut f64,
        var_guard235_rv_slot: &mut f64,
        var_idmultbot_slot: &mut f64,
        var_idmultbot_dn0_slot: &mut f64,
        var_idmultbot_dn2_slot: &mut f64,
        var_idmultbot_rv_slot: &mut f64,
        var_nj0_slot: &mut f64,
        var_nj0_dn0_slot: &mut f64,
        var_nj0_dn2_slot: &mut f64,
        var_nj0_rv_slot: &mut f64,
        var_nj1_slot: &mut f64,
        var_nj1_dn0_slot: &mut f64,
        var_nj1_dn2_slot: &mut f64,
        var_nj1_rv_slot: &mut f64,
        var_nja10_slot: &mut f64,
        var_nja10_dn0_slot: &mut f64,
        var_nja10_dn2_slot: &mut f64,
        var_nja10_rv_slot: &mut f64,
        var_nja11_slot: &mut f64,
        var_nja11_dn0_slot: &mut f64,
        var_nja11_dn2_slot: &mut f64,
        var_nja11_rv_slot: &mut f64,
        var_pnn0_slot: &mut f64,
        var_pnn0_rv_slot: &mut f64,
        var_tmf1_slot: &mut f64,
        var_tmf1_dn0_slot: &mut f64,
        var_tmf1_dn2_slot: &mut f64,
        var_tmf1_rv_slot: &mut f64,
        var_tmf2_slot: &mut f64,
        var_tmf2_dn0_slot: &mut f64,
        var_tmf2_dn2_slot: &mut f64,
        var_tmf2_rv_slot: &mut f64,
        var_vha1_slot: &mut f64,
        var_vha1_rv_slot: &mut f64,
    ) {
        let mut var_dnj1_dv: f64 = *var_dnj1_dv_slot;
        let mut var_dnj1_dv_dn0: f64 = *var_dnj1_dv_dn0_slot;
        let mut var_dnj1_dv_dn2: f64 = *var_dnj1_dv_dn2_slot;
        let mut var_dnj1_dv_rv: f64 = *var_dnj1_dv_rv_slot;
        let mut var_dvmax_over_phitd_dv: f64 = *var_dvmax_over_phitd_dv_slot;
        let mut var_dvmax_over_phitd_dv_dn0: f64 = *var_dvmax_over_phitd_dv_dn0_slot;
        let mut var_dvmax_over_phitd_dv_dn2: f64 = *var_dvmax_over_phitd_dv_dn2_slot;
        let mut var_dvmax_over_phitd_dv_rv: f64 = *var_dvmax_over_phitd_dv_rv_slot;
        let mut var_guard230: f64 = *var_guard230_slot;
        let mut var_guard230_rv: f64 = *var_guard230_rv_slot;
        let mut var_guard231: f64 = *var_guard231_slot;
        let mut var_guard231_rv: f64 = *var_guard231_rv_slot;
        let mut var_guard234: f64 = *var_guard234_slot;
        let mut var_guard234_rv: f64 = *var_guard234_rv_slot;
        let mut var_guard235: f64 = *var_guard235_slot;
        let mut var_guard235_rv: f64 = *var_guard235_rv_slot;
        let mut var_idmultbot: f64 = *var_idmultbot_slot;
        let mut var_idmultbot_dn0: f64 = *var_idmultbot_dn0_slot;
        let mut var_idmultbot_dn2: f64 = *var_idmultbot_dn2_slot;
        let mut var_idmultbot_rv: f64 = *var_idmultbot_rv_slot;
        let mut var_nj0: f64 = *var_nj0_slot;
        let mut var_nj0_dn0: f64 = *var_nj0_dn0_slot;
        let mut var_nj0_dn2: f64 = *var_nj0_dn2_slot;
        let mut var_nj0_rv: f64 = *var_nj0_rv_slot;
        let mut var_nj1: f64 = *var_nj1_slot;
        let mut var_nj1_dn0: f64 = *var_nj1_dn0_slot;
        let mut var_nj1_dn2: f64 = *var_nj1_dn2_slot;
        let mut var_nj1_rv: f64 = *var_nj1_rv_slot;
        let mut var_nja10: f64 = *var_nja10_slot;
        let mut var_nja10_dn0: f64 = *var_nja10_dn0_slot;
        let mut var_nja10_dn2: f64 = *var_nja10_dn2_slot;
        let mut var_nja10_rv: f64 = *var_nja10_rv_slot;
        let mut var_nja11: f64 = *var_nja11_slot;
        let mut var_nja11_dn0: f64 = *var_nja11_dn0_slot;
        let mut var_nja11_dn2: f64 = *var_nja11_dn2_slot;
        let mut var_nja11_rv: f64 = *var_nja11_rv_slot;
        let mut var_pnn0: f64 = *var_pnn0_slot;
        let mut var_pnn0_rv: f64 = *var_pnn0_rv_slot;
        let mut var_tmf1: f64 = *var_tmf1_slot;
        let mut var_tmf1_dn0: f64 = *var_tmf1_dn0_slot;
        let mut var_tmf1_dn2: f64 = *var_tmf1_dn2_slot;
        let mut var_tmf1_rv: f64 = *var_tmf1_rv_slot;
        let mut var_tmf2: f64 = *var_tmf2_slot;
        let mut var_tmf2_dn0: f64 = *var_tmf2_dn0_slot;
        let mut var_tmf2_dn2: f64 = *var_tmf2_dn2_slot;
        let mut var_tmf2_rv: f64 = *var_tmf2_rv_slot;
        let mut var_vha1: f64 = *var_vha1_slot;
        let mut var_vha1_rv: f64 = *var_vha1_rv_slot;

        let (assign10880_e14117, assign10880_e14117_d_n0, assign10880_e14117_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 == 0.0)) && (var_guard172 != 0.0)) {
        let assign10880_e14113: f64 = (p.p86 * var_dfn_su);
        let assign10880_e14115: f64 = (assign10880_e14113 * var_dfn_sl);
        (assign10880_e14115, (((p.p86 * var_dfn_su_dn0) * var_dfn_sl) + (assign10880_e14113 * var_dfn_sl_dn0)), (((p.p86 * var_dfn_su_dn2) * var_dfn_sl) + (assign10880_e14113 * var_dfn_sl_dn2)),)
    } else {
        (var_dnj1_dv, var_dnj1_dv_dn0, var_dnj1_dv_dn2,)
    }
};
        var_dnj1_dv = assign10880_e14117;
        var_dnj1_dv_dn0 = assign10880_e14117_d_n0;
        var_dnj1_dv_dn2 = assign10880_e14117_d_n2;
        var_dnj1_dv_rv = 0.0;

        let (assign10890_e14129, assign10890_e14129_d_n0, assign10890_e14129_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 == 0.0)) && (var_guard172 == 0.0)) {
        (var_nfagat_i, 0.0, 0.0,)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn2,)
    }
};
        var_nj0 = assign10890_e14129;
        var_nj0_dn0 = assign10890_e14129_d_n0;
        var_nj0_dn2 = assign10890_e14129_d_n2;
        var_nj0_rv = 0.0;

        let (assign10900_e14141, assign10900_e14141_d_n0, assign10900_e14141_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 == 0.0)) && (var_guard172 == 0.0)) {
        (var_nfagat_i, 0.0, 0.0,)
    } else {
        (var_nj1, var_nj1_dn0, var_nj1_dn2,)
    }
};
        var_nj1 = assign10900_e14141;
        var_nj1_dn0 = assign10900_e14141_d_n0;
        var_nj1_dn2 = assign10900_e14141_d_n2;
        var_nj1_rv = 0.0;

        let (assign10910_e14153, assign10910_e14153_d_n0, assign10910_e14153_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 == 0.0)) && (var_guard172 == 0.0)) {
        (0.0, 0.0, 0.0,)
    } else {
        (var_dnj1_dv, var_dnj1_dv_dn0, var_dnj1_dv_dn2,)
    }
};
        var_dnj1_dv = assign10910_e14153;
        var_dnj1_dv_dn0 = assign10910_e14153_d_n0;
        var_dnj1_dv_dn2 = assign10910_e14153_d_n2;
        var_dnj1_dv_rv = 0.0;

        let (assign10970_e14402, assign10970_e14402_d_n0, assign10970_e14402_d_n2,) = {
    if (((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 == 0.0)) {
        let assign10970_e14386: f64 = (var_vmax * var_dnj1_dv);
        let assign10970_e14387: f64 = (var_nj1 - assign10970_e14386);
        let assign10970_e14390: f64 = (var_nj1 * var_nj1);
        let assign10970_e14391: f64 = (assign10970_e14387 / assign10970_e14390);
        let assign10970_e14394: f64 = (var_vha1 * var_dnj1_dv);
        let assign10970_e14397: f64 = (var_nj0 * p.p85);
        let assign10970_e14398: f64 = (assign10970_e14394 / assign10970_e14397);
        let assign10970_e14399: f64 = (assign10970_e14391 + assign10970_e14398);
        let assign10970_e14400: f64 = (var_phitdinv * assign10970_e14399);
        (assign10970_e14400, (var_phitdinv * (((((var_nj1_dn0 - (var_vmax * var_dnj1_dv_dn0)) * assign10970_e14390) - (assign10970_e14387 * ((var_nj1_dn0 * var_nj1) + (var_nj1 * var_nj1_dn0)))) / (assign10970_e14390 * assign10970_e14390)) + ((((var_vha1 * var_dnj1_dv_dn0) * assign10970_e14397) - (assign10970_e14394 * (var_nj0_dn0 * p.p85))) / (assign10970_e14397 * assign10970_e14397)))), (var_phitdinv * (((((var_nj1_dn2 - (var_vmax * var_dnj1_dv_dn2)) * assign10970_e14390) - (assign10970_e14387 * ((var_nj1_dn2 * var_nj1) + (var_nj1 * var_nj1_dn2)))) / (assign10970_e14390 * assign10970_e14390)) + ((((var_vha1 * var_dnj1_dv_dn2) * assign10970_e14397) - (assign10970_e14394 * (var_nj0_dn2 * p.p85))) / (assign10970_e14397 * assign10970_e14397)))),)
    } else {
        (var_dvmax_over_phitd_dv, var_dvmax_over_phitd_dv_dn0, var_dvmax_over_phitd_dv_dn2,)
    }
};
        var_dvmax_over_phitd_dv = assign10970_e14402;
        var_dvmax_over_phitd_dv_dn0 = assign10970_e14402_d_n0;
        var_dvmax_over_phitd_dv_dn2 = assign10970_e14402_d_n2;
        var_dvmax_over_phitd_dv_rv = 0.0;

        let (assign10990_e14427, assign10990_e14427_d_n0, assign10990_e14427_d_n2,) = {
    if ((var_guard31 != 0.0) && (var_guard153 != 0.0)) {
        let assign10990_e14425: f64 = (var_idmultbot - 1.0);
        (assign10990_e14425, var_idmultbot_dn0, var_idmultbot_dn2,)
    } else {
        (var_idmultbot, var_idmultbot_dn0, var_idmultbot_dn2,)
    }
};
        var_idmultbot = assign10990_e14427;
        var_idmultbot_dn0 = assign10990_e14427_d_n0;
        var_idmultbot_dn2 = assign10990_e14427_d_n2;
        var_idmultbot_rv = 0.0;

        let (assign11100_e14600, assign11100_e14600_d_n0, assign11100_e14600_d_n2,) = {
    if ((var_guard31 != 0.0) && (var_guard153 == 0.0)) {
        (0.0, 0.0, 0.0,)
    } else {
        (var_idmultbot, var_idmultbot_dn0, var_idmultbot_dn2,)
    }
};
        var_idmultbot = assign11100_e14600;
        var_idmultbot_dn0 = assign11100_e14600_d_n0;
        var_idmultbot_dn2 = assign11100_e14600_d_n2;
        var_idmultbot_rv = 0.0;

        let assign13630_e18150: f64 = if (!(((var_ab_i == 0.0) && (var_ls_i == 0.0)) && (var_lg_i == 0.0))) { 1.0 } else { 0.0 };
        var_guard230 = assign13630_e18150;
        var_guard230_rv = 0.0;

        let assign13710_e18222: f64 = if var_v3 < var_vmax { 1.0 } else { 0.0 };
        var_guard231 = assign13710_e18222;
        var_guard231_rv = 0.0;

        let (assign13770_e18363,) = {
    if (((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 != 0.0)) {
        let assign13770_e18359: f64 = (var_nin * var_nin);
        let assign13770_e18361: f64 = (assign13770_e18359 / var_ndibot_i);
        (assign13770_e18361,)
    } else {
        (var_pnn0,)
    }
};
        var_pnn0 = assign13770_e18363;
        var_pnn0_rv = 0.0;

        let (assign13780_e18378,) = {
    if (((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 != 0.0)) {
        let assign13780_e18371: f64 = (var_nfabot_i / var_phitdinv);
        let assign13780_e18374: f64 = (var_ndibot_i / var_pnn0);
        let assign13780_e18375: f64 = (assign13780_e18374).ln();
        let assign13780_e18376: f64 = (assign13780_e18371 * assign13780_e18375);
        (assign13780_e18376,)
    } else {
        (var_vha1,)
    }
};
        var_vha1 = assign13780_e18378;
        var_vha1_rv = 0.0;

        let assign13790_e18381: f64 = if var_nfabot_i < p.p85 { 1.0 } else { 0.0 };
        var_guard234 = assign13790_e18381;
        var_guard234_rv = 0.0;

        let (assign13800_e18397, assign13800_e18397_d_n0, assign13800_e18397_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 != 0.0)) && (var_guard234 != 0.0)) {
        let assign13800_e18392: f64 = (var_v3 - var_vha1);
        let assign13800_e18393: f64 = (p.p86 * assign13800_e18392);
        let assign13800_e18395: f64 = (assign13800_e18393 + var_nfabot_i);
        (assign13800_e18395, 0.0, 0.0,)
    } else {
        (var_nja10, var_nja10_dn0, var_nja10_dn2,)
    }
};
        var_nja10 = assign13800_e18397;
        var_nja10_dn0 = assign13800_e18397_d_n0;
        var_nja10_dn2 = assign13800_e18397_d_n2;
        var_nja10_rv = 0.0;

        let (assign13810_e18411, assign13810_e18411_d_n0, assign13810_e18411_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 != 0.0)) && (var_guard234 != 0.0)) {
        let assign13810_e18408: f64 = (p.p86 * var_vha1);
        let assign13810_e18409: f64 = (var_nfabot_i - assign13810_e18408);
        (assign13810_e18409, 0.0, 0.0,)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn2,)
    }
};
        var_nj0 = assign13810_e18411;
        var_nj0_dn0 = assign13810_e18411_d_n0;
        var_nj0_dn2 = assign13810_e18411_d_n2;
        var_nj0_rv = 0.0;

        let (assign13820_e18425, assign13820_e18425_d_n0, assign13820_e18425_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 != 0.0)) && (var_guard234 != 0.0)) {
        let assign13820_e18421: f64 = (p.p85 - var_nja10);
        let assign13820_e18423: f64 = (assign13820_e18421 - 0.01);
        (assign13820_e18423, (-var_nja10_dn0), (-var_nja10_dn2),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2,)
    }
};
        var_tmf1 = assign13820_e18425;
        var_tmf1_dn0 = assign13820_e18425_d_n0;
        var_tmf1_dn2 = assign13820_e18425_d_n2;
        var_tmf1_rv = 0.0;

        let (assign13830_e18439, assign13830_e18439_d_n0, assign13830_e18439_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 != 0.0)) && (var_guard234 != 0.0)) {
        let assign13830_e18435: f64 = (4.0 * p.p85);
        let assign13830_e18437: f64 = (assign13830_e18435 * 0.01);
        (assign13830_e18437, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign13830_e18439;
        var_tmf2_dn0 = assign13830_e18439_d_n0;
        var_tmf2_dn2 = assign13830_e18439_d_n2;
        var_tmf2_rv = 0.0;

        let (assign13840_e18455, assign13840_e18455_d_n0, assign13840_e18455_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 != 0.0)) && (var_guard234 != 0.0)) {
        let (assign13840_e18453, assign13840_e18453_d_n0, assign13840_e18453_d_n2,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
            } else {
                let assign13840_e18452: f64 = (-var_tmf2);
                (assign13840_e18452, (-var_tmf2_dn0), (-var_tmf2_dn2),)
            }
        };
        (assign13840_e18453, assign13840_e18453_d_n0, assign13840_e18453_d_n2,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign13840_e18455;
        var_tmf2_dn0 = assign13840_e18455_d_n0;
        var_tmf2_dn2 = assign13840_e18455_d_n2;
        var_tmf2_rv = 0.0;

        let (assign13850_e18470, assign13850_e18470_d_n0, assign13850_e18470_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 != 0.0)) && (var_guard234 != 0.0)) {
        let assign13850_e18465: f64 = (var_tmf1 * var_tmf1);
        let assign13850_e18467: f64 = (assign13850_e18465 + var_tmf2);
        let assign13850_e18468: f64 = (assign13850_e18467).sqrt();
        (assign13850_e18468, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign13850_e18468)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign13850_e18468)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign13850_e18470;
        var_tmf2_dn0 = assign13850_e18470_d_n0;
        var_tmf2_dn2 = assign13850_e18470_d_n2;
        var_tmf2_rv = 0.0;

        let (assign13860_e18486, assign13860_e18486_d_n0, assign13860_e18486_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 != 0.0)) && (var_guard234 != 0.0)) {
        let assign13860_e18482: f64 = (var_tmf1 + var_tmf2);
        let assign13860_e18483: f64 = (0.5 * assign13860_e18482);
        let assign13860_e18484: f64 = (p.p85 - assign13860_e18483);
        (assign13860_e18484, (-(0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (-(0.5 * (var_tmf1_dn2 + var_tmf2_dn2))),)
    } else {
        (var_nja11, var_nja11_dn0, var_nja11_dn2,)
    }
};
        var_nja11 = assign13860_e18486;
        var_nja11_dn0 = assign13860_e18486_d_n0;
        var_nja11_dn2 = assign13860_e18486_d_n2;
        var_nja11_rv = 0.0;

        let (assign13870_e18500, assign13870_e18500_d_n0, assign13870_e18500_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 != 0.0)) && (var_guard234 != 0.0)) {
        let assign13870_e18496: f64 = (var_nja11 - var_nfabot_i);
        let assign13870_e18498: f64 = (assign13870_e18496 - 0.01);
        (assign13870_e18498, var_nja11_dn0, var_nja11_dn2,)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2,)
    }
};
        var_tmf1 = assign13870_e18500;
        var_tmf1_dn0 = assign13870_e18500_d_n0;
        var_tmf1_dn2 = assign13870_e18500_d_n2;
        var_tmf1_rv = 0.0;

        let (assign13880_e18514, assign13880_e18514_d_n0, assign13880_e18514_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 != 0.0)) && (var_guard234 != 0.0)) {
        let assign13880_e18510: f64 = (4.0 * var_nfabot_i);
        let assign13880_e18512: f64 = (assign13880_e18510 * 0.01);
        (assign13880_e18512, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign13880_e18514;
        var_tmf2_dn0 = assign13880_e18514_d_n0;
        var_tmf2_dn2 = assign13880_e18514_d_n2;
        var_tmf2_rv = 0.0;

        let (assign13890_e18530, assign13890_e18530_d_n0, assign13890_e18530_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 != 0.0)) && (var_guard234 != 0.0)) {
        let (assign13890_e18528, assign13890_e18528_d_n0, assign13890_e18528_d_n2,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
            } else {
                let assign13890_e18527: f64 = (-var_tmf2);
                (assign13890_e18527, (-var_tmf2_dn0), (-var_tmf2_dn2),)
            }
        };
        (assign13890_e18528, assign13890_e18528_d_n0, assign13890_e18528_d_n2,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign13890_e18530;
        var_tmf2_dn0 = assign13890_e18530_d_n0;
        var_tmf2_dn2 = assign13890_e18530_d_n2;
        var_tmf2_rv = 0.0;

        let (assign13900_e18545, assign13900_e18545_d_n0, assign13900_e18545_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 != 0.0)) && (var_guard234 != 0.0)) {
        let assign13900_e18540: f64 = (var_tmf1 * var_tmf1);
        let assign13900_e18542: f64 = (assign13900_e18540 + var_tmf2);
        let assign13900_e18543: f64 = (assign13900_e18542).sqrt();
        (assign13900_e18543, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign13900_e18543)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign13900_e18543)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign13900_e18545;
        var_tmf2_dn0 = assign13900_e18545_d_n0;
        var_tmf2_dn2 = assign13900_e18545_d_n2;
        var_tmf2_rv = 0.0;

        let (assign13910_e18561, assign13910_e18561_d_n0, assign13910_e18561_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 != 0.0)) && (var_guard234 != 0.0)) {
        let assign13910_e18557: f64 = (var_tmf1 + var_tmf2);
        let assign13910_e18558: f64 = (0.5 * assign13910_e18557);
        let assign13910_e18559: f64 = (var_nfabot_i + assign13910_e18558);
        (assign13910_e18559, (0.5 * (var_tmf1_dn0 + var_tmf2_dn0)), (0.5 * (var_tmf1_dn2 + var_tmf2_dn2)),)
    } else {
        (var_nj1, var_nj1_dn0, var_nj1_dn2,)
    }
};
        var_nj1 = assign13910_e18561;
        var_nj1_dn0 = assign13910_e18561_d_n0;
        var_nj1_dn2 = assign13910_e18561_d_n2;
        var_nj1_rv = 0.0;

        let (assign13920_e18575, assign13920_e18575_d_n0, assign13920_e18575_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 != 0.0)) && (var_guard234 != 0.0)) {
        let assign13920_e18571: f64 = (p.p85 - var_nj0);
        let assign13920_e18573: f64 = (assign13920_e18571 - 0.01);
        (assign13920_e18573, (-var_nj0_dn0), (-var_nj0_dn2),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2,)
    }
};
        var_tmf1 = assign13920_e18575;
        var_tmf1_dn0 = assign13920_e18575_d_n0;
        var_tmf1_dn2 = assign13920_e18575_d_n2;
        var_tmf1_rv = 0.0;

        let (assign13930_e18589, assign13930_e18589_d_n0, assign13930_e18589_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 != 0.0)) && (var_guard234 != 0.0)) {
        let assign13930_e18585: f64 = (4.0 * p.p85);
        let assign13930_e18587: f64 = (assign13930_e18585 * 0.01);
        (assign13930_e18587, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign13930_e18589;
        var_tmf2_dn0 = assign13930_e18589_d_n0;
        var_tmf2_dn2 = assign13930_e18589_d_n2;
        var_tmf2_rv = 0.0;

        let (assign13940_e18605, assign13940_e18605_d_n0, assign13940_e18605_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 != 0.0)) && (var_guard234 != 0.0)) {
        let (assign13940_e18603, assign13940_e18603_d_n0, assign13940_e18603_d_n2,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
            } else {
                let assign13940_e18602: f64 = (-var_tmf2);
                (assign13940_e18602, (-var_tmf2_dn0), (-var_tmf2_dn2),)
            }
        };
        (assign13940_e18603, assign13940_e18603_d_n0, assign13940_e18603_d_n2,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign13940_e18605;
        var_tmf2_dn0 = assign13940_e18605_d_n0;
        var_tmf2_dn2 = assign13940_e18605_d_n2;
        var_tmf2_rv = 0.0;

        let (assign13950_e18620, assign13950_e18620_d_n0, assign13950_e18620_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 != 0.0)) && (var_guard234 != 0.0)) {
        let assign13950_e18615: f64 = (var_tmf1 * var_tmf1);
        let assign13950_e18617: f64 = (assign13950_e18615 + var_tmf2);
        let assign13950_e18618: f64 = (assign13950_e18617).sqrt();
        (assign13950_e18618, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign13950_e18618)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign13950_e18618)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign13950_e18620;
        var_tmf2_dn0 = assign13950_e18620_d_n0;
        var_tmf2_dn2 = assign13950_e18620_d_n2;
        var_tmf2_rv = 0.0;

        let (assign13960_e18636, assign13960_e18636_d_n0, assign13960_e18636_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 != 0.0)) && (var_guard234 != 0.0)) {
        let assign13960_e18632: f64 = (var_tmf1 + var_tmf2);
        let assign13960_e18633: f64 = (0.5 * assign13960_e18632);
        let assign13960_e18634: f64 = (p.p85 - assign13960_e18633);
        (assign13960_e18634, (-(0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (-(0.5 * (var_tmf1_dn2 + var_tmf2_dn2))),)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn2,)
    }
};
        var_nj0 = assign13960_e18636;
        var_nj0_dn0 = assign13960_e18636_d_n0;
        var_nj0_dn2 = assign13960_e18636_d_n2;
        var_nj0_rv = 0.0;

        let (assign13970_e18650, assign13970_e18650_d_n0, assign13970_e18650_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 != 0.0)) && (var_guard234 != 0.0)) {
        let assign13970_e18646: f64 = (var_nj0 - var_nfabot_i);
        let assign13970_e18648: f64 = (assign13970_e18646 - 0.01);
        (assign13970_e18648, var_nj0_dn0, var_nj0_dn2,)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2,)
    }
};
        var_tmf1 = assign13970_e18650;
        var_tmf1_dn0 = assign13970_e18650_d_n0;
        var_tmf1_dn2 = assign13970_e18650_d_n2;
        var_tmf1_rv = 0.0;

        let (assign13980_e18664, assign13980_e18664_d_n0, assign13980_e18664_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 != 0.0)) && (var_guard234 != 0.0)) {
        let assign13980_e18660: f64 = (4.0 * var_nfabot_i);
        let assign13980_e18662: f64 = (assign13980_e18660 * 0.01);
        (assign13980_e18662, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign13980_e18664;
        var_tmf2_dn0 = assign13980_e18664_d_n0;
        var_tmf2_dn2 = assign13980_e18664_d_n2;
        var_tmf2_rv = 0.0;

        let (assign13990_e18680, assign13990_e18680_d_n0, assign13990_e18680_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 != 0.0)) && (var_guard234 != 0.0)) {
        let (assign13990_e18678, assign13990_e18678_d_n0, assign13990_e18678_d_n2,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
            } else {
                let assign13990_e18677: f64 = (-var_tmf2);
                (assign13990_e18677, (-var_tmf2_dn0), (-var_tmf2_dn2),)
            }
        };
        (assign13990_e18678, assign13990_e18678_d_n0, assign13990_e18678_d_n2,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign13990_e18680;
        var_tmf2_dn0 = assign13990_e18680_d_n0;
        var_tmf2_dn2 = assign13990_e18680_d_n2;
        var_tmf2_rv = 0.0;

        let (assign14000_e18695, assign14000_e18695_d_n0, assign14000_e18695_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 != 0.0)) && (var_guard234 != 0.0)) {
        let assign14000_e18690: f64 = (var_tmf1 * var_tmf1);
        let assign14000_e18692: f64 = (assign14000_e18690 + var_tmf2);
        let assign14000_e18693: f64 = (assign14000_e18692).sqrt();
        (assign14000_e18693, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign14000_e18693)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign14000_e18693)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign14000_e18695;
        var_tmf2_dn0 = assign14000_e18695_d_n0;
        var_tmf2_dn2 = assign14000_e18695_d_n2;
        var_tmf2_rv = 0.0;

        let (assign14010_e18711, assign14010_e18711_d_n0, assign14010_e18711_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 != 0.0)) && (var_guard234 != 0.0)) {
        let assign14010_e18707: f64 = (var_tmf1 + var_tmf2);
        let assign14010_e18708: f64 = (0.5 * assign14010_e18707);
        let assign14010_e18709: f64 = (var_nfabot_i + assign14010_e18708);
        (assign14010_e18709, (0.5 * (var_tmf1_dn0 + var_tmf2_dn0)), (0.5 * (var_tmf1_dn2 + var_tmf2_dn2)),)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn2,)
    }
};
        var_nj0 = assign14010_e18711;
        var_nj0_dn0 = assign14010_e18711_d_n0;
        var_nj0_dn2 = assign14010_e18711_d_n2;
        var_nj0_rv = 0.0;

        let (assign14020_e18722, assign14020_e18722_d_n0, assign14020_e18722_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 != 0.0)) && (var_guard234 == 0.0)) {
        (var_nfabot_i, 0.0, 0.0,)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn2,)
    }
};
        var_nj0 = assign14020_e18722;
        var_nj0_dn0 = assign14020_e18722_d_n0;
        var_nj0_dn2 = assign14020_e18722_d_n2;
        var_nj0_rv = 0.0;

        let (assign14030_e18733, assign14030_e18733_d_n0, assign14030_e18733_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 != 0.0)) && (var_guard234 == 0.0)) {
        (var_nfabot_i, 0.0, 0.0,)
    } else {
        (var_nj1, var_nj1_dn0, var_nj1_dn2,)
    }
};
        var_nj1 = assign14030_e18733;
        var_nj1_dn0 = assign14030_e18733_d_n0;
        var_nj1_dn2 = assign14030_e18733_d_n2;
        var_nj1_rv = 0.0;

        let assign14040_e18737: f64 = (var_v3 / var_nj1);
        let assign14040_e18741: f64 = (var_nj1 - var_nj0);
        let assign14040_e18742: f64 = (var_vha1 * assign14040_e18741);
        let assign14040_e18745: f64 = (var_nj0 * p.p85);
        let assign14040_e18746: f64 = (assign14040_e18742 / assign14040_e18745);
        let assign14040_e18747: f64 = (assign14040_e18737 + assign14040_e18746);
        let assign14040_e18748: f64 = (var_phitdinv * assign14040_e18747);
        let assign14040_e18749: f64 = (assign14040_e18748).abs();
        let assign14040_e18751: f64 = if assign14040_e18749 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard235 = assign14040_e18751;
        var_guard235_rv = 0.0;

        *var_dnj1_dv_slot = var_dnj1_dv;
        *var_dnj1_dv_dn0_slot = var_dnj1_dv_dn0;
        *var_dnj1_dv_dn2_slot = var_dnj1_dv_dn2;
        *var_dnj1_dv_rv_slot = var_dnj1_dv_rv;
        *var_dvmax_over_phitd_dv_slot = var_dvmax_over_phitd_dv;
        *var_dvmax_over_phitd_dv_dn0_slot = var_dvmax_over_phitd_dv_dn0;
        *var_dvmax_over_phitd_dv_dn2_slot = var_dvmax_over_phitd_dv_dn2;
        *var_dvmax_over_phitd_dv_rv_slot = var_dvmax_over_phitd_dv_rv;
        *var_guard230_slot = var_guard230;
        *var_guard230_rv_slot = var_guard230_rv;
        *var_guard231_slot = var_guard231;
        *var_guard231_rv_slot = var_guard231_rv;
        *var_guard234_slot = var_guard234;
        *var_guard234_rv_slot = var_guard234_rv;
        *var_guard235_slot = var_guard235;
        *var_guard235_rv_slot = var_guard235_rv;
        *var_idmultbot_slot = var_idmultbot;
        *var_idmultbot_dn0_slot = var_idmultbot_dn0;
        *var_idmultbot_dn2_slot = var_idmultbot_dn2;
        *var_idmultbot_rv_slot = var_idmultbot_rv;
        *var_nj0_slot = var_nj0;
        *var_nj0_dn0_slot = var_nj0_dn0;
        *var_nj0_dn2_slot = var_nj0_dn2;
        *var_nj0_rv_slot = var_nj0_rv;
        *var_nj1_slot = var_nj1;
        *var_nj1_dn0_slot = var_nj1_dn0;
        *var_nj1_dn2_slot = var_nj1_dn2;
        *var_nj1_rv_slot = var_nj1_rv;
        *var_nja10_slot = var_nja10;
        *var_nja10_dn0_slot = var_nja10_dn0;
        *var_nja10_dn2_slot = var_nja10_dn2;
        *var_nja10_rv_slot = var_nja10_rv;
        *var_nja11_slot = var_nja11;
        *var_nja11_dn0_slot = var_nja11_dn0;
        *var_nja11_dn2_slot = var_nja11_dn2;
        *var_nja11_rv_slot = var_nja11_rv;
        *var_pnn0_slot = var_pnn0;
        *var_pnn0_rv_slot = var_pnn0_rv;
        *var_tmf1_slot = var_tmf1;
        *var_tmf1_dn0_slot = var_tmf1_dn0;
        *var_tmf1_dn2_slot = var_tmf1_dn2;
        *var_tmf1_rv_slot = var_tmf1_rv;
        *var_tmf2_slot = var_tmf2;
        *var_tmf2_dn0_slot = var_tmf2_dn0;
        *var_tmf2_dn2_slot = var_tmf2_dn2;
        *var_tmf2_rv_slot = var_tmf2_rv;
        *var_vha1_slot = var_vha1;
        *var_vha1_rv_slot = var_vha1_rv;
    }

    pub(super) fn stamp_reactive_block_14(
        p: &Parameters,
        var_guard230: f64,
        var_guard231: f64,
        var_guard235: f64,
        var_guard31: f64,
        var_ndisti_i: f64,
        var_nfasti_i: f64,
        var_nin: f64,
        var_phitdinv: f64,
        var_v3: f64,
        var_guard236_slot: &mut f64,
        var_guard236_rv_slot: &mut f64,
        var_guard237_slot: &mut f64,
        var_guard237_rv_slot: &mut f64,
        var_idmultbot_slot: &mut f64,
        var_idmultbot_dn0_slot: &mut f64,
        var_idmultbot_dn2_slot: &mut f64,
        var_idmultbot_rv_slot: &mut f64,
        var_nj0_slot: &mut f64,
        var_nj0_dn0_slot: &mut f64,
        var_nj0_dn2_slot: &mut f64,
        var_nj0_rv_slot: &mut f64,
        var_nj1_slot: &mut f64,
        var_nj1_dn0_slot: &mut f64,
        var_nj1_dn2_slot: &mut f64,
        var_nj1_rv_slot: &mut f64,
        var_nja10_slot: &mut f64,
        var_nja10_dn0_slot: &mut f64,
        var_nja10_dn2_slot: &mut f64,
        var_nja10_rv_slot: &mut f64,
        var_nja11_slot: &mut f64,
        var_nja11_dn0_slot: &mut f64,
        var_nja11_dn2_slot: &mut f64,
        var_nja11_rv_slot: &mut f64,
        var_pnn0_slot: &mut f64,
        var_pnn0_rv_slot: &mut f64,
        var_tmf1_slot: &mut f64,
        var_tmf1_dn0_slot: &mut f64,
        var_tmf1_dn2_slot: &mut f64,
        var_tmf1_rv_slot: &mut f64,
        var_tmf2_slot: &mut f64,
        var_tmf2_dn0_slot: &mut f64,
        var_tmf2_dn2_slot: &mut f64,
        var_tmf2_rv_slot: &mut f64,
        var_vha1_slot: &mut f64,
        var_vha1_rv_slot: &mut f64,
    ) {
        let mut var_guard236: f64 = *var_guard236_slot;
        let mut var_guard236_rv: f64 = *var_guard236_rv_slot;
        let mut var_guard237: f64 = *var_guard237_slot;
        let mut var_guard237_rv: f64 = *var_guard237_rv_slot;
        let mut var_idmultbot: f64 = *var_idmultbot_slot;
        let mut var_idmultbot_dn0: f64 = *var_idmultbot_dn0_slot;
        let mut var_idmultbot_dn2: f64 = *var_idmultbot_dn2_slot;
        let mut var_idmultbot_rv: f64 = *var_idmultbot_rv_slot;
        let mut var_nj0: f64 = *var_nj0_slot;
        let mut var_nj0_dn0: f64 = *var_nj0_dn0_slot;
        let mut var_nj0_dn2: f64 = *var_nj0_dn2_slot;
        let mut var_nj0_rv: f64 = *var_nj0_rv_slot;
        let mut var_nj1: f64 = *var_nj1_slot;
        let mut var_nj1_dn0: f64 = *var_nj1_dn0_slot;
        let mut var_nj1_dn2: f64 = *var_nj1_dn2_slot;
        let mut var_nj1_rv: f64 = *var_nj1_rv_slot;
        let mut var_nja10: f64 = *var_nja10_slot;
        let mut var_nja10_dn0: f64 = *var_nja10_dn0_slot;
        let mut var_nja10_dn2: f64 = *var_nja10_dn2_slot;
        let mut var_nja10_rv: f64 = *var_nja10_rv_slot;
        let mut var_nja11: f64 = *var_nja11_slot;
        let mut var_nja11_dn0: f64 = *var_nja11_dn0_slot;
        let mut var_nja11_dn2: f64 = *var_nja11_dn2_slot;
        let mut var_nja11_rv: f64 = *var_nja11_rv_slot;
        let mut var_pnn0: f64 = *var_pnn0_slot;
        let mut var_pnn0_rv: f64 = *var_pnn0_rv_slot;
        let mut var_tmf1: f64 = *var_tmf1_slot;
        let mut var_tmf1_dn0: f64 = *var_tmf1_dn0_slot;
        let mut var_tmf1_dn2: f64 = *var_tmf1_dn2_slot;
        let mut var_tmf1_rv: f64 = *var_tmf1_rv_slot;
        let mut var_tmf2: f64 = *var_tmf2_slot;
        let mut var_tmf2_dn0: f64 = *var_tmf2_dn0_slot;
        let mut var_tmf2_dn2: f64 = *var_tmf2_dn2_slot;
        let mut var_tmf2_rv: f64 = *var_tmf2_rv_slot;
        let mut var_vha1: f64 = *var_vha1_slot;
        let mut var_vha1_rv: f64 = *var_vha1_rv_slot;

        let (assign14050_e18776, assign14050_e18776_d_n0, assign14050_e18776_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 != 0.0)) && (var_guard235 != 0.0)) {
        let assign14050_e18762: f64 = (var_v3 / var_nj1);
        let assign14050_e18766: f64 = (var_nj1 - var_nj0);
        let assign14050_e18767: f64 = (var_vha1 * assign14050_e18766);
        let assign14050_e18770: f64 = (var_nj0 * p.p85);
        let assign14050_e18771: f64 = (assign14050_e18767 / assign14050_e18770);
        let assign14050_e18772: f64 = (assign14050_e18762 + assign14050_e18771);
        let assign14050_e18773: f64 = (var_phitdinv * assign14050_e18772);
        let assign14050_e18774: f64 = (assign14050_e18773).exp();
        (assign14050_e18774, (assign14050_e18774 * (var_phitdinv * ((-((var_v3 * var_nj1_dn0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn0 - var_nj0_dn0)) * assign14050_e18770) - (assign14050_e18767 * (var_nj0_dn0 * p.p85))) / (assign14050_e18770 * assign14050_e18770))))), (assign14050_e18774 * (var_phitdinv * ((-((var_v3 * var_nj1_dn2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn2 - var_nj0_dn2)) * assign14050_e18770) - (assign14050_e18767 * (var_nj0_dn2 * p.p85))) / (assign14050_e18770 * assign14050_e18770))))),)
    } else {
        (var_idmultbot, var_idmultbot_dn0, var_idmultbot_dn2,)
    }
};
        var_idmultbot = assign14050_e18776;
        var_idmultbot_dn0 = assign14050_e18776_d_n0;
        var_idmultbot_dn2 = assign14050_e18776_d_n2;
        var_idmultbot_rv = 0.0;

        let assign14060_e18780: f64 = (var_v3 / var_nj1);
        let assign14060_e18784: f64 = (var_nj1 - var_nj0);
        let assign14060_e18785: f64 = (var_vha1 * assign14060_e18784);
        let assign14060_e18788: f64 = (var_nj0 * p.p85);
        let assign14060_e18789: f64 = (assign14060_e18785 / assign14060_e18788);
        let assign14060_e18790: f64 = (assign14060_e18780 + assign14060_e18789);
        let assign14060_e18791: f64 = (var_phitdinv * assign14060_e18790);
        let assign14060_e18793: f64 = (-230.25850929940458);
        let assign14060_e18794: f64 = if assign14060_e18791 < assign14060_e18793 { 1.0 } else { 0.0 };
        var_guard236 = assign14060_e18794;
        var_guard236_rv = 0.0;

        let (assign14070_e18874, assign14070_e18874_d_n0, assign14070_e18874_d_n2,) = {
    if (((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 != 0.0)) && (var_guard235 == 0.0)) && (var_guard236 != 0.0)) {
        let assign14070_e18808: f64 = (-230.25850929940458);
        let assign14070_e18812: f64 = (var_v3 / var_nj1);
        let assign14070_e18816: f64 = (var_nj1 - var_nj0);
        let assign14070_e18817: f64 = (var_vha1 * assign14070_e18816);
        let assign14070_e18820: f64 = (var_nj0 * p.p85);
        let assign14070_e18821: f64 = (assign14070_e18817 / assign14070_e18820);
        let assign14070_e18822: f64 = (assign14070_e18812 + assign14070_e18821);
        let assign14070_e18823: f64 = (var_phitdinv * assign14070_e18822);
        let assign14070_e18824: f64 = (assign14070_e18808 - assign14070_e18823);
        let assign14070_e18828: f64 = (-230.25850929940458);
        let assign14070_e18832: f64 = (var_v3 / var_nj1);
        let assign14070_e18836: f64 = (var_nj1 - var_nj0);
        let assign14070_e18837: f64 = (var_vha1 * assign14070_e18836);
        let assign14070_e18840: f64 = (var_nj0 * p.p85);
        let assign14070_e18841: f64 = (assign14070_e18837 / assign14070_e18840);
        let assign14070_e18842: f64 = (assign14070_e18832 + assign14070_e18841);
        let assign14070_e18843: f64 = (var_phitdinv * assign14070_e18842);
        let assign14070_e18844: f64 = (assign14070_e18828 - assign14070_e18843);
        let assign14070_e18847: f64 = (-230.25850929940458);
        let assign14070_e18851: f64 = (var_v3 / var_nj1);
        let assign14070_e18855: f64 = (var_nj1 - var_nj0);
        let assign14070_e18856: f64 = (var_vha1 * assign14070_e18855);
        let assign14070_e18859: f64 = (var_nj0 * p.p85);
        let assign14070_e18860: f64 = (assign14070_e18856 / assign14070_e18859);
        let assign14070_e18861: f64 = (assign14070_e18851 + assign14070_e18860);
        let assign14070_e18862: f64 = (var_phitdinv * assign14070_e18861);
        let assign14070_e18863: f64 = (assign14070_e18847 - assign14070_e18862);
        let assign14070_e18865: f64 = (assign14070_e18863 * 0.3333333333333333);
        let assign14070_e18866: f64 = (1.0 + assign14070_e18865);
        let assign14070_e18867: f64 = (assign14070_e18844 * assign14070_e18866);
        let assign14070_e18868: f64 = (0.5 * assign14070_e18867);
        let assign14070_e18869: f64 = (1.0 + assign14070_e18868);
        let assign14070_e18870: f64 = (assign14070_e18824 * assign14070_e18869);
        let assign14070_e18871: f64 = (1.0 + assign14070_e18870);
        let assign14070_e18872: f64 = (1e-100 / assign14070_e18871);
        (assign14070_e18872, (-((1e-100 * (((-(var_phitdinv * ((-((var_v3 * var_nj1_dn0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn0 - var_nj0_dn0)) * assign14070_e18820) - (assign14070_e18817 * (var_nj0_dn0 * p.p85))) / (assign14070_e18820 * assign14070_e18820))))) * assign14070_e18869) + (assign14070_e18824 * (0.5 * (((-(var_phitdinv * ((-((var_v3 * var_nj1_dn0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn0 - var_nj0_dn0)) * assign14070_e18840) - (assign14070_e18837 * (var_nj0_dn0 * p.p85))) / (assign14070_e18840 * assign14070_e18840))))) * assign14070_e18866) + (assign14070_e18844 * ((-(var_phitdinv * ((-((var_v3 * var_nj1_dn0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn0 - var_nj0_dn0)) * assign14070_e18859) - (assign14070_e18856 * (var_nj0_dn0 * p.p85))) / (assign14070_e18859 * assign14070_e18859))))) * 0.3333333333333333))))))) / (assign14070_e18871 * assign14070_e18871))), (-((1e-100 * (((-(var_phitdinv * ((-((var_v3 * var_nj1_dn2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn2 - var_nj0_dn2)) * assign14070_e18820) - (assign14070_e18817 * (var_nj0_dn2 * p.p85))) / (assign14070_e18820 * assign14070_e18820))))) * assign14070_e18869) + (assign14070_e18824 * (0.5 * (((-(var_phitdinv * ((-((var_v3 * var_nj1_dn2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn2 - var_nj0_dn2)) * assign14070_e18840) - (assign14070_e18837 * (var_nj0_dn2 * p.p85))) / (assign14070_e18840 * assign14070_e18840))))) * assign14070_e18866) + (assign14070_e18844 * ((-(var_phitdinv * ((-((var_v3 * var_nj1_dn2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn2 - var_nj0_dn2)) * assign14070_e18859) - (assign14070_e18856 * (var_nj0_dn2 * p.p85))) / (assign14070_e18859 * assign14070_e18859))))) * 0.3333333333333333))))))) / (assign14070_e18871 * assign14070_e18871))),)
    } else {
        (var_idmultbot, var_idmultbot_dn0, var_idmultbot_dn2,)
    }
};
        var_idmultbot = assign14070_e18874;
        var_idmultbot_dn0 = assign14070_e18874_d_n0;
        var_idmultbot_dn2 = assign14070_e18874_d_n2;
        var_idmultbot_rv = 0.0;

        let (assign14080_e18952, assign14080_e18952_d_n0, assign14080_e18952_d_n2,) = {
    if (((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 != 0.0)) && (var_guard235 == 0.0)) && (var_guard236 == 0.0)) {
        let assign14080_e18891: f64 = (var_v3 / var_nj1);
        let assign14080_e18895: f64 = (var_nj1 - var_nj0);
        let assign14080_e18896: f64 = (var_vha1 * assign14080_e18895);
        let assign14080_e18899: f64 = (var_nj0 * p.p85);
        let assign14080_e18900: f64 = (assign14080_e18896 / assign14080_e18899);
        let assign14080_e18901: f64 = (assign14080_e18891 + assign14080_e18900);
        let assign14080_e18902: f64 = (var_phitdinv * assign14080_e18901);
        let assign14080_e18904: f64 = (assign14080_e18902 - 230.25850929940458);
        let assign14080_e18910: f64 = (var_v3 / var_nj1);
        let assign14080_e18914: f64 = (var_nj1 - var_nj0);
        let assign14080_e18915: f64 = (var_vha1 * assign14080_e18914);
        let assign14080_e18918: f64 = (var_nj0 * p.p85);
        let assign14080_e18919: f64 = (assign14080_e18915 / assign14080_e18918);
        let assign14080_e18920: f64 = (assign14080_e18910 + assign14080_e18919);
        let assign14080_e18921: f64 = (var_phitdinv * assign14080_e18920);
        let assign14080_e18923: f64 = (assign14080_e18921 - 230.25850929940458);
        let assign14080_e18928: f64 = (var_v3 / var_nj1);
        let assign14080_e18932: f64 = (var_nj1 - var_nj0);
        let assign14080_e18933: f64 = (var_vha1 * assign14080_e18932);
        let assign14080_e18936: f64 = (var_nj0 * p.p85);
        let assign14080_e18937: f64 = (assign14080_e18933 / assign14080_e18936);
        let assign14080_e18938: f64 = (assign14080_e18928 + assign14080_e18937);
        let assign14080_e18939: f64 = (var_phitdinv * assign14080_e18938);
        let assign14080_e18941: f64 = (assign14080_e18939 - 230.25850929940458);
        let assign14080_e18943: f64 = (assign14080_e18941 * 0.3333333333333333);
        let assign14080_e18944: f64 = (1.0 + assign14080_e18943);
        let assign14080_e18945: f64 = (assign14080_e18923 * assign14080_e18944);
        let assign14080_e18946: f64 = (0.5 * assign14080_e18945);
        let assign14080_e18947: f64 = (1.0 + assign14080_e18946);
        let assign14080_e18948: f64 = (assign14080_e18904 * assign14080_e18947);
        let assign14080_e18949: f64 = (1.0 + assign14080_e18948);
        let assign14080_e18950: f64 = (1e100 * assign14080_e18949);
        (assign14080_e18950, (1e100 * (((var_phitdinv * ((-((var_v3 * var_nj1_dn0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn0 - var_nj0_dn0)) * assign14080_e18899) - (assign14080_e18896 * (var_nj0_dn0 * p.p85))) / (assign14080_e18899 * assign14080_e18899)))) * assign14080_e18947) + (assign14080_e18904 * (0.5 * (((var_phitdinv * ((-((var_v3 * var_nj1_dn0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn0 - var_nj0_dn0)) * assign14080_e18918) - (assign14080_e18915 * (var_nj0_dn0 * p.p85))) / (assign14080_e18918 * assign14080_e18918)))) * assign14080_e18944) + (assign14080_e18923 * ((var_phitdinv * ((-((var_v3 * var_nj1_dn0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn0 - var_nj0_dn0)) * assign14080_e18936) - (assign14080_e18933 * (var_nj0_dn0 * p.p85))) / (assign14080_e18936 * assign14080_e18936)))) * 0.3333333333333333))))))), (1e100 * (((var_phitdinv * ((-((var_v3 * var_nj1_dn2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn2 - var_nj0_dn2)) * assign14080_e18899) - (assign14080_e18896 * (var_nj0_dn2 * p.p85))) / (assign14080_e18899 * assign14080_e18899)))) * assign14080_e18947) + (assign14080_e18904 * (0.5 * (((var_phitdinv * ((-((var_v3 * var_nj1_dn2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn2 - var_nj0_dn2)) * assign14080_e18918) - (assign14080_e18915 * (var_nj0_dn2 * p.p85))) / (assign14080_e18918 * assign14080_e18918)))) * assign14080_e18944) + (assign14080_e18923 * ((var_phitdinv * ((-((var_v3 * var_nj1_dn2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn2 - var_nj0_dn2)) * assign14080_e18936) - (assign14080_e18933 * (var_nj0_dn2 * p.p85))) / (assign14080_e18936 * assign14080_e18936)))) * 0.3333333333333333))))))),)
    } else {
        (var_idmultbot, var_idmultbot_dn0, var_idmultbot_dn2,)
    }
};
        var_idmultbot = assign14080_e18952;
        var_idmultbot_dn0 = assign14080_e18952_d_n0;
        var_idmultbot_dn2 = assign14080_e18952_d_n2;
        var_idmultbot_rv = 0.0;

        let (assign14090_e18964,) = {
    if (((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 != 0.0)) {
        let assign14090_e18960: f64 = (var_nin * var_nin);
        let assign14090_e18962: f64 = (assign14090_e18960 / var_ndisti_i);
        (assign14090_e18962,)
    } else {
        (var_pnn0,)
    }
};
        var_pnn0 = assign14090_e18964;
        var_pnn0_rv = 0.0;

        let (assign14100_e18979,) = {
    if (((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 != 0.0)) {
        let assign14100_e18972: f64 = (var_nfasti_i / var_phitdinv);
        let assign14100_e18975: f64 = (var_ndisti_i / var_pnn0);
        let assign14100_e18976: f64 = (assign14100_e18975).ln();
        let assign14100_e18977: f64 = (assign14100_e18972 * assign14100_e18976);
        (assign14100_e18977,)
    } else {
        (var_vha1,)
    }
};
        var_vha1 = assign14100_e18979;
        var_vha1_rv = 0.0;

        let assign14110_e18982: f64 = if var_nfasti_i < p.p85 { 1.0 } else { 0.0 };
        var_guard237 = assign14110_e18982;
        var_guard237_rv = 0.0;

        let (assign14120_e18998, assign14120_e18998_d_n0, assign14120_e18998_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 != 0.0)) && (var_guard237 != 0.0)) {
        let assign14120_e18993: f64 = (var_v3 - var_vha1);
        let assign14120_e18994: f64 = (p.p86 * assign14120_e18993);
        let assign14120_e18996: f64 = (assign14120_e18994 + var_nfasti_i);
        (assign14120_e18996, 0.0, 0.0,)
    } else {
        (var_nja10, var_nja10_dn0, var_nja10_dn2,)
    }
};
        var_nja10 = assign14120_e18998;
        var_nja10_dn0 = assign14120_e18998_d_n0;
        var_nja10_dn2 = assign14120_e18998_d_n2;
        var_nja10_rv = 0.0;

        let (assign14130_e19012, assign14130_e19012_d_n0, assign14130_e19012_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 != 0.0)) && (var_guard237 != 0.0)) {
        let assign14130_e19009: f64 = (p.p86 * var_vha1);
        let assign14130_e19010: f64 = (var_nfasti_i - assign14130_e19009);
        (assign14130_e19010, 0.0, 0.0,)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn2,)
    }
};
        var_nj0 = assign14130_e19012;
        var_nj0_dn0 = assign14130_e19012_d_n0;
        var_nj0_dn2 = assign14130_e19012_d_n2;
        var_nj0_rv = 0.0;

        let (assign14140_e19026, assign14140_e19026_d_n0, assign14140_e19026_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 != 0.0)) && (var_guard237 != 0.0)) {
        let assign14140_e19022: f64 = (p.p85 - var_nja10);
        let assign14140_e19024: f64 = (assign14140_e19022 - 0.01);
        (assign14140_e19024, (-var_nja10_dn0), (-var_nja10_dn2),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2,)
    }
};
        var_tmf1 = assign14140_e19026;
        var_tmf1_dn0 = assign14140_e19026_d_n0;
        var_tmf1_dn2 = assign14140_e19026_d_n2;
        var_tmf1_rv = 0.0;

        let (assign14150_e19040, assign14150_e19040_d_n0, assign14150_e19040_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 != 0.0)) && (var_guard237 != 0.0)) {
        let assign14150_e19036: f64 = (4.0 * p.p85);
        let assign14150_e19038: f64 = (assign14150_e19036 * 0.01);
        (assign14150_e19038, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign14150_e19040;
        var_tmf2_dn0 = assign14150_e19040_d_n0;
        var_tmf2_dn2 = assign14150_e19040_d_n2;
        var_tmf2_rv = 0.0;

        let (assign14160_e19056, assign14160_e19056_d_n0, assign14160_e19056_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 != 0.0)) && (var_guard237 != 0.0)) {
        let (assign14160_e19054, assign14160_e19054_d_n0, assign14160_e19054_d_n2,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
            } else {
                let assign14160_e19053: f64 = (-var_tmf2);
                (assign14160_e19053, (-var_tmf2_dn0), (-var_tmf2_dn2),)
            }
        };
        (assign14160_e19054, assign14160_e19054_d_n0, assign14160_e19054_d_n2,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign14160_e19056;
        var_tmf2_dn0 = assign14160_e19056_d_n0;
        var_tmf2_dn2 = assign14160_e19056_d_n2;
        var_tmf2_rv = 0.0;

        let (assign14170_e19071, assign14170_e19071_d_n0, assign14170_e19071_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 != 0.0)) && (var_guard237 != 0.0)) {
        let assign14170_e19066: f64 = (var_tmf1 * var_tmf1);
        let assign14170_e19068: f64 = (assign14170_e19066 + var_tmf2);
        let assign14170_e19069: f64 = (assign14170_e19068).sqrt();
        (assign14170_e19069, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign14170_e19069)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign14170_e19069)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign14170_e19071;
        var_tmf2_dn0 = assign14170_e19071_d_n0;
        var_tmf2_dn2 = assign14170_e19071_d_n2;
        var_tmf2_rv = 0.0;

        let (assign14180_e19087, assign14180_e19087_d_n0, assign14180_e19087_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 != 0.0)) && (var_guard237 != 0.0)) {
        let assign14180_e19083: f64 = (var_tmf1 + var_tmf2);
        let assign14180_e19084: f64 = (0.5 * assign14180_e19083);
        let assign14180_e19085: f64 = (p.p85 - assign14180_e19084);
        (assign14180_e19085, (-(0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (-(0.5 * (var_tmf1_dn2 + var_tmf2_dn2))),)
    } else {
        (var_nja11, var_nja11_dn0, var_nja11_dn2,)
    }
};
        var_nja11 = assign14180_e19087;
        var_nja11_dn0 = assign14180_e19087_d_n0;
        var_nja11_dn2 = assign14180_e19087_d_n2;
        var_nja11_rv = 0.0;

        let (assign14190_e19101, assign14190_e19101_d_n0, assign14190_e19101_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 != 0.0)) && (var_guard237 != 0.0)) {
        let assign14190_e19097: f64 = (var_nja11 - var_nfasti_i);
        let assign14190_e19099: f64 = (assign14190_e19097 - 0.01);
        (assign14190_e19099, var_nja11_dn0, var_nja11_dn2,)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2,)
    }
};
        var_tmf1 = assign14190_e19101;
        var_tmf1_dn0 = assign14190_e19101_d_n0;
        var_tmf1_dn2 = assign14190_e19101_d_n2;
        var_tmf1_rv = 0.0;

        let (assign14200_e19115, assign14200_e19115_d_n0, assign14200_e19115_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 != 0.0)) && (var_guard237 != 0.0)) {
        let assign14200_e19111: f64 = (4.0 * var_nfasti_i);
        let assign14200_e19113: f64 = (assign14200_e19111 * 0.01);
        (assign14200_e19113, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign14200_e19115;
        var_tmf2_dn0 = assign14200_e19115_d_n0;
        var_tmf2_dn2 = assign14200_e19115_d_n2;
        var_tmf2_rv = 0.0;

        let (assign14210_e19131, assign14210_e19131_d_n0, assign14210_e19131_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 != 0.0)) && (var_guard237 != 0.0)) {
        let (assign14210_e19129, assign14210_e19129_d_n0, assign14210_e19129_d_n2,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
            } else {
                let assign14210_e19128: f64 = (-var_tmf2);
                (assign14210_e19128, (-var_tmf2_dn0), (-var_tmf2_dn2),)
            }
        };
        (assign14210_e19129, assign14210_e19129_d_n0, assign14210_e19129_d_n2,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign14210_e19131;
        var_tmf2_dn0 = assign14210_e19131_d_n0;
        var_tmf2_dn2 = assign14210_e19131_d_n2;
        var_tmf2_rv = 0.0;

        let (assign14220_e19146, assign14220_e19146_d_n0, assign14220_e19146_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 != 0.0)) && (var_guard237 != 0.0)) {
        let assign14220_e19141: f64 = (var_tmf1 * var_tmf1);
        let assign14220_e19143: f64 = (assign14220_e19141 + var_tmf2);
        let assign14220_e19144: f64 = (assign14220_e19143).sqrt();
        (assign14220_e19144, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign14220_e19144)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign14220_e19144)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign14220_e19146;
        var_tmf2_dn0 = assign14220_e19146_d_n0;
        var_tmf2_dn2 = assign14220_e19146_d_n2;
        var_tmf2_rv = 0.0;

        let (assign14230_e19162, assign14230_e19162_d_n0, assign14230_e19162_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 != 0.0)) && (var_guard237 != 0.0)) {
        let assign14230_e19158: f64 = (var_tmf1 + var_tmf2);
        let assign14230_e19159: f64 = (0.5 * assign14230_e19158);
        let assign14230_e19160: f64 = (var_nfasti_i + assign14230_e19159);
        (assign14230_e19160, (0.5 * (var_tmf1_dn0 + var_tmf2_dn0)), (0.5 * (var_tmf1_dn2 + var_tmf2_dn2)),)
    } else {
        (var_nj1, var_nj1_dn0, var_nj1_dn2,)
    }
};
        var_nj1 = assign14230_e19162;
        var_nj1_dn0 = assign14230_e19162_d_n0;
        var_nj1_dn2 = assign14230_e19162_d_n2;
        var_nj1_rv = 0.0;

        let (assign14240_e19176, assign14240_e19176_d_n0, assign14240_e19176_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 != 0.0)) && (var_guard237 != 0.0)) {
        let assign14240_e19172: f64 = (p.p85 - var_nj0);
        let assign14240_e19174: f64 = (assign14240_e19172 - 0.01);
        (assign14240_e19174, (-var_nj0_dn0), (-var_nj0_dn2),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2,)
    }
};
        var_tmf1 = assign14240_e19176;
        var_tmf1_dn0 = assign14240_e19176_d_n0;
        var_tmf1_dn2 = assign14240_e19176_d_n2;
        var_tmf1_rv = 0.0;

        let (assign14250_e19190, assign14250_e19190_d_n0, assign14250_e19190_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 != 0.0)) && (var_guard237 != 0.0)) {
        let assign14250_e19186: f64 = (4.0 * p.p85);
        let assign14250_e19188: f64 = (assign14250_e19186 * 0.01);
        (assign14250_e19188, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign14250_e19190;
        var_tmf2_dn0 = assign14250_e19190_d_n0;
        var_tmf2_dn2 = assign14250_e19190_d_n2;
        var_tmf2_rv = 0.0;

        let (assign14260_e19206, assign14260_e19206_d_n0, assign14260_e19206_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 != 0.0)) && (var_guard237 != 0.0)) {
        let (assign14260_e19204, assign14260_e19204_d_n0, assign14260_e19204_d_n2,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
            } else {
                let assign14260_e19203: f64 = (-var_tmf2);
                (assign14260_e19203, (-var_tmf2_dn0), (-var_tmf2_dn2),)
            }
        };
        (assign14260_e19204, assign14260_e19204_d_n0, assign14260_e19204_d_n2,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign14260_e19206;
        var_tmf2_dn0 = assign14260_e19206_d_n0;
        var_tmf2_dn2 = assign14260_e19206_d_n2;
        var_tmf2_rv = 0.0;

        let (assign14270_e19221, assign14270_e19221_d_n0, assign14270_e19221_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 != 0.0)) && (var_guard237 != 0.0)) {
        let assign14270_e19216: f64 = (var_tmf1 * var_tmf1);
        let assign14270_e19218: f64 = (assign14270_e19216 + var_tmf2);
        let assign14270_e19219: f64 = (assign14270_e19218).sqrt();
        (assign14270_e19219, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign14270_e19219)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign14270_e19219)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign14270_e19221;
        var_tmf2_dn0 = assign14270_e19221_d_n0;
        var_tmf2_dn2 = assign14270_e19221_d_n2;
        var_tmf2_rv = 0.0;

        let (assign14280_e19237, assign14280_e19237_d_n0, assign14280_e19237_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 != 0.0)) && (var_guard237 != 0.0)) {
        let assign14280_e19233: f64 = (var_tmf1 + var_tmf2);
        let assign14280_e19234: f64 = (0.5 * assign14280_e19233);
        let assign14280_e19235: f64 = (p.p85 - assign14280_e19234);
        (assign14280_e19235, (-(0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (-(0.5 * (var_tmf1_dn2 + var_tmf2_dn2))),)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn2,)
    }
};
        var_nj0 = assign14280_e19237;
        var_nj0_dn0 = assign14280_e19237_d_n0;
        var_nj0_dn2 = assign14280_e19237_d_n2;
        var_nj0_rv = 0.0;

        let (assign14290_e19251, assign14290_e19251_d_n0, assign14290_e19251_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 != 0.0)) && (var_guard237 != 0.0)) {
        let assign14290_e19247: f64 = (var_nj0 - var_nfasti_i);
        let assign14290_e19249: f64 = (assign14290_e19247 - 0.01);
        (assign14290_e19249, var_nj0_dn0, var_nj0_dn2,)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2,)
    }
};
        var_tmf1 = assign14290_e19251;
        var_tmf1_dn0 = assign14290_e19251_d_n0;
        var_tmf1_dn2 = assign14290_e19251_d_n2;
        var_tmf1_rv = 0.0;

        let (assign14300_e19265, assign14300_e19265_d_n0, assign14300_e19265_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 != 0.0)) && (var_guard237 != 0.0)) {
        let assign14300_e19261: f64 = (4.0 * var_nfasti_i);
        let assign14300_e19263: f64 = (assign14300_e19261 * 0.01);
        (assign14300_e19263, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign14300_e19265;
        var_tmf2_dn0 = assign14300_e19265_d_n0;
        var_tmf2_dn2 = assign14300_e19265_d_n2;
        var_tmf2_rv = 0.0;

        let (assign14310_e19281, assign14310_e19281_d_n0, assign14310_e19281_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 != 0.0)) && (var_guard237 != 0.0)) {
        let (assign14310_e19279, assign14310_e19279_d_n0, assign14310_e19279_d_n2,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
            } else {
                let assign14310_e19278: f64 = (-var_tmf2);
                (assign14310_e19278, (-var_tmf2_dn0), (-var_tmf2_dn2),)
            }
        };
        (assign14310_e19279, assign14310_e19279_d_n0, assign14310_e19279_d_n2,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign14310_e19281;
        var_tmf2_dn0 = assign14310_e19281_d_n0;
        var_tmf2_dn2 = assign14310_e19281_d_n2;
        var_tmf2_rv = 0.0;

        let (assign14320_e19296, assign14320_e19296_d_n0, assign14320_e19296_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 != 0.0)) && (var_guard237 != 0.0)) {
        let assign14320_e19291: f64 = (var_tmf1 * var_tmf1);
        let assign14320_e19293: f64 = (assign14320_e19291 + var_tmf2);
        let assign14320_e19294: f64 = (assign14320_e19293).sqrt();
        (assign14320_e19294, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign14320_e19294)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign14320_e19294)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign14320_e19296;
        var_tmf2_dn0 = assign14320_e19296_d_n0;
        var_tmf2_dn2 = assign14320_e19296_d_n2;
        var_tmf2_rv = 0.0;

        let (assign14330_e19312, assign14330_e19312_d_n0, assign14330_e19312_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 != 0.0)) && (var_guard237 != 0.0)) {
        let assign14330_e19308: f64 = (var_tmf1 + var_tmf2);
        let assign14330_e19309: f64 = (0.5 * assign14330_e19308);
        let assign14330_e19310: f64 = (var_nfasti_i + assign14330_e19309);
        (assign14330_e19310, (0.5 * (var_tmf1_dn0 + var_tmf2_dn0)), (0.5 * (var_tmf1_dn2 + var_tmf2_dn2)),)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn2,)
    }
};
        var_nj0 = assign14330_e19312;
        var_nj0_dn0 = assign14330_e19312_d_n0;
        var_nj0_dn2 = assign14330_e19312_d_n2;
        var_nj0_rv = 0.0;

        let (assign14340_e19323, assign14340_e19323_d_n0, assign14340_e19323_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 != 0.0)) && (var_guard237 == 0.0)) {
        (var_nfasti_i, 0.0, 0.0,)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn2,)
    }
};
        var_nj0 = assign14340_e19323;
        var_nj0_dn0 = assign14340_e19323_d_n0;
        var_nj0_dn2 = assign14340_e19323_d_n2;
        var_nj0_rv = 0.0;

        *var_guard236_slot = var_guard236;
        *var_guard236_rv_slot = var_guard236_rv;
        *var_guard237_slot = var_guard237;
        *var_guard237_rv_slot = var_guard237_rv;
        *var_idmultbot_slot = var_idmultbot;
        *var_idmultbot_dn0_slot = var_idmultbot_dn0;
        *var_idmultbot_dn2_slot = var_idmultbot_dn2;
        *var_idmultbot_rv_slot = var_idmultbot_rv;
        *var_nj0_slot = var_nj0;
        *var_nj0_dn0_slot = var_nj0_dn0;
        *var_nj0_dn2_slot = var_nj0_dn2;
        *var_nj0_rv_slot = var_nj0_rv;
        *var_nj1_slot = var_nj1;
        *var_nj1_dn0_slot = var_nj1_dn0;
        *var_nj1_dn2_slot = var_nj1_dn2;
        *var_nj1_rv_slot = var_nj1_rv;
        *var_nja10_slot = var_nja10;
        *var_nja10_dn0_slot = var_nja10_dn0;
        *var_nja10_dn2_slot = var_nja10_dn2;
        *var_nja10_rv_slot = var_nja10_rv;
        *var_nja11_slot = var_nja11;
        *var_nja11_dn0_slot = var_nja11_dn0;
        *var_nja11_dn2_slot = var_nja11_dn2;
        *var_nja11_rv_slot = var_nja11_rv;
        *var_pnn0_slot = var_pnn0;
        *var_pnn0_rv_slot = var_pnn0_rv;
        *var_tmf1_slot = var_tmf1;
        *var_tmf1_dn0_slot = var_tmf1_dn0;
        *var_tmf1_dn2_slot = var_tmf1_dn2;
        *var_tmf1_rv_slot = var_tmf1_rv;
        *var_tmf2_slot = var_tmf2;
        *var_tmf2_dn0_slot = var_tmf2_dn0;
        *var_tmf2_dn2_slot = var_tmf2_dn2;
        *var_tmf2_rv_slot = var_tmf2_rv;
        *var_vha1_slot = var_vha1;
        *var_vha1_rv_slot = var_vha1_rv;
    }

    pub(super) fn stamp_reactive_block_15(
        p: &Parameters,
        var_guard230: f64,
        var_guard231: f64,
        var_guard237: f64,
        var_guard31: f64,
        var_ndibot_i: f64,
        var_ndigat_i: f64,
        var_nfabot_i: f64,
        var_nfagat_i: f64,
        var_nfasti_i: f64,
        var_nin: f64,
        var_phitdinv: f64,
        var_v3: f64,
        var_vmax: f64,
        var_guard240_slot: &mut f64,
        var_guard240_rv_slot: &mut f64,
        var_guard243_slot: &mut f64,
        var_guard243_rv_slot: &mut f64,
        var_nj0_slot: &mut f64,
        var_nj0_dn0_slot: &mut f64,
        var_nj0_dn2_slot: &mut f64,
        var_nj0_rv_slot: &mut f64,
        var_nj1_slot: &mut f64,
        var_nj1_dn0_slot: &mut f64,
        var_nj1_dn2_slot: &mut f64,
        var_nj1_rv_slot: &mut f64,
        var_nja10_slot: &mut f64,
        var_nja10_dn0_slot: &mut f64,
        var_nja10_dn2_slot: &mut f64,
        var_nja10_rv_slot: &mut f64,
        var_nja11_slot: &mut f64,
        var_nja11_dn0_slot: &mut f64,
        var_nja11_dn2_slot: &mut f64,
        var_nja11_rv_slot: &mut f64,
        var_pnn0_slot: &mut f64,
        var_pnn0_rv_slot: &mut f64,
        var_tmf1_slot: &mut f64,
        var_tmf1_dn0_slot: &mut f64,
        var_tmf1_dn2_slot: &mut f64,
        var_tmf1_rv_slot: &mut f64,
        var_tmf2_slot: &mut f64,
        var_tmf2_dn0_slot: &mut f64,
        var_tmf2_dn2_slot: &mut f64,
        var_tmf2_rv_slot: &mut f64,
        var_vha1_slot: &mut f64,
        var_vha1_rv_slot: &mut f64,
    ) {
        let mut var_guard240: f64 = *var_guard240_slot;
        let mut var_guard240_rv: f64 = *var_guard240_rv_slot;
        let mut var_guard243: f64 = *var_guard243_slot;
        let mut var_guard243_rv: f64 = *var_guard243_rv_slot;
        let mut var_nj0: f64 = *var_nj0_slot;
        let mut var_nj0_dn0: f64 = *var_nj0_dn0_slot;
        let mut var_nj0_dn2: f64 = *var_nj0_dn2_slot;
        let mut var_nj0_rv: f64 = *var_nj0_rv_slot;
        let mut var_nj1: f64 = *var_nj1_slot;
        let mut var_nj1_dn0: f64 = *var_nj1_dn0_slot;
        let mut var_nj1_dn2: f64 = *var_nj1_dn2_slot;
        let mut var_nj1_rv: f64 = *var_nj1_rv_slot;
        let mut var_nja10: f64 = *var_nja10_slot;
        let mut var_nja10_dn0: f64 = *var_nja10_dn0_slot;
        let mut var_nja10_dn2: f64 = *var_nja10_dn2_slot;
        let mut var_nja10_rv: f64 = *var_nja10_rv_slot;
        let mut var_nja11: f64 = *var_nja11_slot;
        let mut var_nja11_dn0: f64 = *var_nja11_dn0_slot;
        let mut var_nja11_dn2: f64 = *var_nja11_dn2_slot;
        let mut var_nja11_rv: f64 = *var_nja11_rv_slot;
        let mut var_pnn0: f64 = *var_pnn0_slot;
        let mut var_pnn0_rv: f64 = *var_pnn0_rv_slot;
        let mut var_tmf1: f64 = *var_tmf1_slot;
        let mut var_tmf1_dn0: f64 = *var_tmf1_dn0_slot;
        let mut var_tmf1_dn2: f64 = *var_tmf1_dn2_slot;
        let mut var_tmf1_rv: f64 = *var_tmf1_rv_slot;
        let mut var_tmf2: f64 = *var_tmf2_slot;
        let mut var_tmf2_dn0: f64 = *var_tmf2_dn0_slot;
        let mut var_tmf2_dn2: f64 = *var_tmf2_dn2_slot;
        let mut var_tmf2_rv: f64 = *var_tmf2_rv_slot;
        let mut var_vha1: f64 = *var_vha1_slot;
        let mut var_vha1_rv: f64 = *var_vha1_rv_slot;

        let (assign14350_e19334, assign14350_e19334_d_n0, assign14350_e19334_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 != 0.0)) && (var_guard237 == 0.0)) {
        (var_nfasti_i, 0.0, 0.0,)
    } else {
        (var_nj1, var_nj1_dn0, var_nj1_dn2,)
    }
};
        var_nj1 = assign14350_e19334;
        var_nj1_dn0 = assign14350_e19334_d_n0;
        var_nj1_dn2 = assign14350_e19334_d_n2;
        var_nj1_rv = 0.0;

        let (assign14410_e19565,) = {
    if (((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 != 0.0)) {
        let assign14410_e19561: f64 = (var_nin * var_nin);
        let assign14410_e19563: f64 = (assign14410_e19561 / var_ndigat_i);
        (assign14410_e19563,)
    } else {
        (var_pnn0,)
    }
};
        var_pnn0 = assign14410_e19565;
        var_pnn0_rv = 0.0;

        let (assign14420_e19580,) = {
    if (((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 != 0.0)) {
        let assign14420_e19573: f64 = (var_nfagat_i / var_phitdinv);
        let assign14420_e19576: f64 = (var_ndigat_i / var_pnn0);
        let assign14420_e19577: f64 = (assign14420_e19576).ln();
        let assign14420_e19578: f64 = (assign14420_e19573 * assign14420_e19577);
        (assign14420_e19578,)
    } else {
        (var_vha1,)
    }
};
        var_vha1 = assign14420_e19580;
        var_vha1_rv = 0.0;

        let assign14430_e19583: f64 = if var_nfagat_i < p.p85 { 1.0 } else { 0.0 };
        var_guard240 = assign14430_e19583;
        var_guard240_rv = 0.0;

        let (assign14440_e19599, assign14440_e19599_d_n0, assign14440_e19599_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 != 0.0)) && (var_guard240 != 0.0)) {
        let assign14440_e19594: f64 = (var_v3 - var_vha1);
        let assign14440_e19595: f64 = (p.p86 * assign14440_e19594);
        let assign14440_e19597: f64 = (assign14440_e19595 + var_nfagat_i);
        (assign14440_e19597, 0.0, 0.0,)
    } else {
        (var_nja10, var_nja10_dn0, var_nja10_dn2,)
    }
};
        var_nja10 = assign14440_e19599;
        var_nja10_dn0 = assign14440_e19599_d_n0;
        var_nja10_dn2 = assign14440_e19599_d_n2;
        var_nja10_rv = 0.0;

        let (assign14450_e19613, assign14450_e19613_d_n0, assign14450_e19613_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 != 0.0)) && (var_guard240 != 0.0)) {
        let assign14450_e19610: f64 = (p.p86 * var_vha1);
        let assign14450_e19611: f64 = (var_nfagat_i - assign14450_e19610);
        (assign14450_e19611, 0.0, 0.0,)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn2,)
    }
};
        var_nj0 = assign14450_e19613;
        var_nj0_dn0 = assign14450_e19613_d_n0;
        var_nj0_dn2 = assign14450_e19613_d_n2;
        var_nj0_rv = 0.0;

        let (assign14460_e19627, assign14460_e19627_d_n0, assign14460_e19627_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 != 0.0)) && (var_guard240 != 0.0)) {
        let assign14460_e19623: f64 = (p.p85 - var_nja10);
        let assign14460_e19625: f64 = (assign14460_e19623 - 0.01);
        (assign14460_e19625, (-var_nja10_dn0), (-var_nja10_dn2),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2,)
    }
};
        var_tmf1 = assign14460_e19627;
        var_tmf1_dn0 = assign14460_e19627_d_n0;
        var_tmf1_dn2 = assign14460_e19627_d_n2;
        var_tmf1_rv = 0.0;

        let (assign14470_e19641, assign14470_e19641_d_n0, assign14470_e19641_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 != 0.0)) && (var_guard240 != 0.0)) {
        let assign14470_e19637: f64 = (4.0 * p.p85);
        let assign14470_e19639: f64 = (assign14470_e19637 * 0.01);
        (assign14470_e19639, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign14470_e19641;
        var_tmf2_dn0 = assign14470_e19641_d_n0;
        var_tmf2_dn2 = assign14470_e19641_d_n2;
        var_tmf2_rv = 0.0;

        let (assign14480_e19657, assign14480_e19657_d_n0, assign14480_e19657_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 != 0.0)) && (var_guard240 != 0.0)) {
        let (assign14480_e19655, assign14480_e19655_d_n0, assign14480_e19655_d_n2,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
            } else {
                let assign14480_e19654: f64 = (-var_tmf2);
                (assign14480_e19654, (-var_tmf2_dn0), (-var_tmf2_dn2),)
            }
        };
        (assign14480_e19655, assign14480_e19655_d_n0, assign14480_e19655_d_n2,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign14480_e19657;
        var_tmf2_dn0 = assign14480_e19657_d_n0;
        var_tmf2_dn2 = assign14480_e19657_d_n2;
        var_tmf2_rv = 0.0;

        let (assign14490_e19672, assign14490_e19672_d_n0, assign14490_e19672_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 != 0.0)) && (var_guard240 != 0.0)) {
        let assign14490_e19667: f64 = (var_tmf1 * var_tmf1);
        let assign14490_e19669: f64 = (assign14490_e19667 + var_tmf2);
        let assign14490_e19670: f64 = (assign14490_e19669).sqrt();
        (assign14490_e19670, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign14490_e19670)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign14490_e19670)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign14490_e19672;
        var_tmf2_dn0 = assign14490_e19672_d_n0;
        var_tmf2_dn2 = assign14490_e19672_d_n2;
        var_tmf2_rv = 0.0;

        let (assign14500_e19688, assign14500_e19688_d_n0, assign14500_e19688_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 != 0.0)) && (var_guard240 != 0.0)) {
        let assign14500_e19684: f64 = (var_tmf1 + var_tmf2);
        let assign14500_e19685: f64 = (0.5 * assign14500_e19684);
        let assign14500_e19686: f64 = (p.p85 - assign14500_e19685);
        (assign14500_e19686, (-(0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (-(0.5 * (var_tmf1_dn2 + var_tmf2_dn2))),)
    } else {
        (var_nja11, var_nja11_dn0, var_nja11_dn2,)
    }
};
        var_nja11 = assign14500_e19688;
        var_nja11_dn0 = assign14500_e19688_d_n0;
        var_nja11_dn2 = assign14500_e19688_d_n2;
        var_nja11_rv = 0.0;

        let (assign14510_e19702, assign14510_e19702_d_n0, assign14510_e19702_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 != 0.0)) && (var_guard240 != 0.0)) {
        let assign14510_e19698: f64 = (var_nja11 - var_nfagat_i);
        let assign14510_e19700: f64 = (assign14510_e19698 - 0.01);
        (assign14510_e19700, var_nja11_dn0, var_nja11_dn2,)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2,)
    }
};
        var_tmf1 = assign14510_e19702;
        var_tmf1_dn0 = assign14510_e19702_d_n0;
        var_tmf1_dn2 = assign14510_e19702_d_n2;
        var_tmf1_rv = 0.0;

        let (assign14520_e19716, assign14520_e19716_d_n0, assign14520_e19716_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 != 0.0)) && (var_guard240 != 0.0)) {
        let assign14520_e19712: f64 = (4.0 * var_nfagat_i);
        let assign14520_e19714: f64 = (assign14520_e19712 * 0.01);
        (assign14520_e19714, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign14520_e19716;
        var_tmf2_dn0 = assign14520_e19716_d_n0;
        var_tmf2_dn2 = assign14520_e19716_d_n2;
        var_tmf2_rv = 0.0;

        let (assign14530_e19732, assign14530_e19732_d_n0, assign14530_e19732_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 != 0.0)) && (var_guard240 != 0.0)) {
        let (assign14530_e19730, assign14530_e19730_d_n0, assign14530_e19730_d_n2,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
            } else {
                let assign14530_e19729: f64 = (-var_tmf2);
                (assign14530_e19729, (-var_tmf2_dn0), (-var_tmf2_dn2),)
            }
        };
        (assign14530_e19730, assign14530_e19730_d_n0, assign14530_e19730_d_n2,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign14530_e19732;
        var_tmf2_dn0 = assign14530_e19732_d_n0;
        var_tmf2_dn2 = assign14530_e19732_d_n2;
        var_tmf2_rv = 0.0;

        let (assign14540_e19747, assign14540_e19747_d_n0, assign14540_e19747_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 != 0.0)) && (var_guard240 != 0.0)) {
        let assign14540_e19742: f64 = (var_tmf1 * var_tmf1);
        let assign14540_e19744: f64 = (assign14540_e19742 + var_tmf2);
        let assign14540_e19745: f64 = (assign14540_e19744).sqrt();
        (assign14540_e19745, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign14540_e19745)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign14540_e19745)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign14540_e19747;
        var_tmf2_dn0 = assign14540_e19747_d_n0;
        var_tmf2_dn2 = assign14540_e19747_d_n2;
        var_tmf2_rv = 0.0;

        let (assign14550_e19763, assign14550_e19763_d_n0, assign14550_e19763_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 != 0.0)) && (var_guard240 != 0.0)) {
        let assign14550_e19759: f64 = (var_tmf1 + var_tmf2);
        let assign14550_e19760: f64 = (0.5 * assign14550_e19759);
        let assign14550_e19761: f64 = (var_nfagat_i + assign14550_e19760);
        (assign14550_e19761, (0.5 * (var_tmf1_dn0 + var_tmf2_dn0)), (0.5 * (var_tmf1_dn2 + var_tmf2_dn2)),)
    } else {
        (var_nj1, var_nj1_dn0, var_nj1_dn2,)
    }
};
        var_nj1 = assign14550_e19763;
        var_nj1_dn0 = assign14550_e19763_d_n0;
        var_nj1_dn2 = assign14550_e19763_d_n2;
        var_nj1_rv = 0.0;

        let (assign14560_e19777, assign14560_e19777_d_n0, assign14560_e19777_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 != 0.0)) && (var_guard240 != 0.0)) {
        let assign14560_e19773: f64 = (p.p85 - var_nj0);
        let assign14560_e19775: f64 = (assign14560_e19773 - 0.01);
        (assign14560_e19775, (-var_nj0_dn0), (-var_nj0_dn2),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2,)
    }
};
        var_tmf1 = assign14560_e19777;
        var_tmf1_dn0 = assign14560_e19777_d_n0;
        var_tmf1_dn2 = assign14560_e19777_d_n2;
        var_tmf1_rv = 0.0;

        let (assign14570_e19791, assign14570_e19791_d_n0, assign14570_e19791_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 != 0.0)) && (var_guard240 != 0.0)) {
        let assign14570_e19787: f64 = (4.0 * p.p85);
        let assign14570_e19789: f64 = (assign14570_e19787 * 0.01);
        (assign14570_e19789, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign14570_e19791;
        var_tmf2_dn0 = assign14570_e19791_d_n0;
        var_tmf2_dn2 = assign14570_e19791_d_n2;
        var_tmf2_rv = 0.0;

        let (assign14580_e19807, assign14580_e19807_d_n0, assign14580_e19807_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 != 0.0)) && (var_guard240 != 0.0)) {
        let (assign14580_e19805, assign14580_e19805_d_n0, assign14580_e19805_d_n2,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
            } else {
                let assign14580_e19804: f64 = (-var_tmf2);
                (assign14580_e19804, (-var_tmf2_dn0), (-var_tmf2_dn2),)
            }
        };
        (assign14580_e19805, assign14580_e19805_d_n0, assign14580_e19805_d_n2,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign14580_e19807;
        var_tmf2_dn0 = assign14580_e19807_d_n0;
        var_tmf2_dn2 = assign14580_e19807_d_n2;
        var_tmf2_rv = 0.0;

        let (assign14590_e19822, assign14590_e19822_d_n0, assign14590_e19822_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 != 0.0)) && (var_guard240 != 0.0)) {
        let assign14590_e19817: f64 = (var_tmf1 * var_tmf1);
        let assign14590_e19819: f64 = (assign14590_e19817 + var_tmf2);
        let assign14590_e19820: f64 = (assign14590_e19819).sqrt();
        (assign14590_e19820, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign14590_e19820)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign14590_e19820)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign14590_e19822;
        var_tmf2_dn0 = assign14590_e19822_d_n0;
        var_tmf2_dn2 = assign14590_e19822_d_n2;
        var_tmf2_rv = 0.0;

        let (assign14600_e19838, assign14600_e19838_d_n0, assign14600_e19838_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 != 0.0)) && (var_guard240 != 0.0)) {
        let assign14600_e19834: f64 = (var_tmf1 + var_tmf2);
        let assign14600_e19835: f64 = (0.5 * assign14600_e19834);
        let assign14600_e19836: f64 = (p.p85 - assign14600_e19835);
        (assign14600_e19836, (-(0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (-(0.5 * (var_tmf1_dn2 + var_tmf2_dn2))),)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn2,)
    }
};
        var_nj0 = assign14600_e19838;
        var_nj0_dn0 = assign14600_e19838_d_n0;
        var_nj0_dn2 = assign14600_e19838_d_n2;
        var_nj0_rv = 0.0;

        let (assign14610_e19852, assign14610_e19852_d_n0, assign14610_e19852_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 != 0.0)) && (var_guard240 != 0.0)) {
        let assign14610_e19848: f64 = (var_nj0 - var_nfagat_i);
        let assign14610_e19850: f64 = (assign14610_e19848 - 0.01);
        (assign14610_e19850, var_nj0_dn0, var_nj0_dn2,)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2,)
    }
};
        var_tmf1 = assign14610_e19852;
        var_tmf1_dn0 = assign14610_e19852_d_n0;
        var_tmf1_dn2 = assign14610_e19852_d_n2;
        var_tmf1_rv = 0.0;

        let (assign14620_e19866, assign14620_e19866_d_n0, assign14620_e19866_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 != 0.0)) && (var_guard240 != 0.0)) {
        let assign14620_e19862: f64 = (4.0 * var_nfagat_i);
        let assign14620_e19864: f64 = (assign14620_e19862 * 0.01);
        (assign14620_e19864, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign14620_e19866;
        var_tmf2_dn0 = assign14620_e19866_d_n0;
        var_tmf2_dn2 = assign14620_e19866_d_n2;
        var_tmf2_rv = 0.0;

        let (assign14630_e19882, assign14630_e19882_d_n0, assign14630_e19882_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 != 0.0)) && (var_guard240 != 0.0)) {
        let (assign14630_e19880, assign14630_e19880_d_n0, assign14630_e19880_d_n2,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
            } else {
                let assign14630_e19879: f64 = (-var_tmf2);
                (assign14630_e19879, (-var_tmf2_dn0), (-var_tmf2_dn2),)
            }
        };
        (assign14630_e19880, assign14630_e19880_d_n0, assign14630_e19880_d_n2,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign14630_e19882;
        var_tmf2_dn0 = assign14630_e19882_d_n0;
        var_tmf2_dn2 = assign14630_e19882_d_n2;
        var_tmf2_rv = 0.0;

        let (assign14640_e19897, assign14640_e19897_d_n0, assign14640_e19897_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 != 0.0)) && (var_guard240 != 0.0)) {
        let assign14640_e19892: f64 = (var_tmf1 * var_tmf1);
        let assign14640_e19894: f64 = (assign14640_e19892 + var_tmf2);
        let assign14640_e19895: f64 = (assign14640_e19894).sqrt();
        (assign14640_e19895, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign14640_e19895)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign14640_e19895)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign14640_e19897;
        var_tmf2_dn0 = assign14640_e19897_d_n0;
        var_tmf2_dn2 = assign14640_e19897_d_n2;
        var_tmf2_rv = 0.0;

        let (assign14650_e19913, assign14650_e19913_d_n0, assign14650_e19913_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 != 0.0)) && (var_guard240 != 0.0)) {
        let assign14650_e19909: f64 = (var_tmf1 + var_tmf2);
        let assign14650_e19910: f64 = (0.5 * assign14650_e19909);
        let assign14650_e19911: f64 = (var_nfagat_i + assign14650_e19910);
        (assign14650_e19911, (0.5 * (var_tmf1_dn0 + var_tmf2_dn0)), (0.5 * (var_tmf1_dn2 + var_tmf2_dn2)),)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn2,)
    }
};
        var_nj0 = assign14650_e19913;
        var_nj0_dn0 = assign14650_e19913_d_n0;
        var_nj0_dn2 = assign14650_e19913_d_n2;
        var_nj0_rv = 0.0;

        let (assign14660_e19924, assign14660_e19924_d_n0, assign14660_e19924_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 != 0.0)) && (var_guard240 == 0.0)) {
        (var_nfagat_i, 0.0, 0.0,)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn2,)
    }
};
        var_nj0 = assign14660_e19924;
        var_nj0_dn0 = assign14660_e19924_d_n0;
        var_nj0_dn2 = assign14660_e19924_d_n2;
        var_nj0_rv = 0.0;

        let (assign14670_e19935, assign14670_e19935_d_n0, assign14670_e19935_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 != 0.0)) && (var_guard240 == 0.0)) {
        (var_nfagat_i, 0.0, 0.0,)
    } else {
        (var_nj1, var_nj1_dn0, var_nj1_dn2,)
    }
};
        var_nj1 = assign14670_e19935;
        var_nj1_dn0 = assign14670_e19935_d_n0;
        var_nj1_dn2 = assign14670_e19935_d_n2;
        var_nj1_rv = 0.0;

        let (assign14740_e20185,) = {
    if (((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) {
        let assign14740_e20181: f64 = (var_nin * var_nin);
        let assign14740_e20183: f64 = (assign14740_e20181 / var_ndibot_i);
        (assign14740_e20183,)
    } else {
        (var_pnn0,)
    }
};
        var_pnn0 = assign14740_e20185;
        var_pnn0_rv = 0.0;

        let (assign14750_e20201,) = {
    if (((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) {
        let assign14750_e20194: f64 = (var_nfabot_i / var_phitdinv);
        let assign14750_e20197: f64 = (var_ndibot_i / var_pnn0);
        let assign14750_e20198: f64 = (assign14750_e20197).ln();
        let assign14750_e20199: f64 = (assign14750_e20194 * assign14750_e20198);
        (assign14750_e20199,)
    } else {
        (var_vha1,)
    }
};
        var_vha1 = assign14750_e20201;
        var_vha1_rv = 0.0;

        let assign14760_e20204: f64 = if var_nfabot_i < p.p85 { 1.0 } else { 0.0 };
        var_guard243 = assign14760_e20204;
        var_guard243_rv = 0.0;

        let (assign14770_e20221, assign14770_e20221_d_n0, assign14770_e20221_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard243 != 0.0)) {
        let assign14770_e20216: f64 = (var_vmax - var_vha1);
        let assign14770_e20217: f64 = (p.p86 * assign14770_e20216);
        let assign14770_e20219: f64 = (assign14770_e20217 + var_nfabot_i);
        (assign14770_e20219, 0.0, 0.0,)
    } else {
        (var_nja10, var_nja10_dn0, var_nja10_dn2,)
    }
};
        var_nja10 = assign14770_e20221;
        var_nja10_dn0 = assign14770_e20221_d_n0;
        var_nja10_dn2 = assign14770_e20221_d_n2;
        var_nja10_rv = 0.0;

        let (assign14780_e20236, assign14780_e20236_d_n0, assign14780_e20236_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard243 != 0.0)) {
        let assign14780_e20233: f64 = (p.p86 * var_vha1);
        let assign14780_e20234: f64 = (var_nfabot_i - assign14780_e20233);
        (assign14780_e20234, 0.0, 0.0,)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn2,)
    }
};
        var_nj0 = assign14780_e20236;
        var_nj0_dn0 = assign14780_e20236_d_n0;
        var_nj0_dn2 = assign14780_e20236_d_n2;
        var_nj0_rv = 0.0;

        let (assign14790_e20251, assign14790_e20251_d_n0, assign14790_e20251_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard243 != 0.0)) {
        let assign14790_e20247: f64 = (p.p85 - var_nja10);
        let assign14790_e20249: f64 = (assign14790_e20247 - 0.01);
        (assign14790_e20249, (-var_nja10_dn0), (-var_nja10_dn2),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2,)
    }
};
        var_tmf1 = assign14790_e20251;
        var_tmf1_dn0 = assign14790_e20251_d_n0;
        var_tmf1_dn2 = assign14790_e20251_d_n2;
        var_tmf1_rv = 0.0;

        let (assign14800_e20266, assign14800_e20266_d_n0, assign14800_e20266_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard243 != 0.0)) {
        let assign14800_e20262: f64 = (4.0 * p.p85);
        let assign14800_e20264: f64 = (assign14800_e20262 * 0.01);
        (assign14800_e20264, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign14800_e20266;
        var_tmf2_dn0 = assign14800_e20266_d_n0;
        var_tmf2_dn2 = assign14800_e20266_d_n2;
        var_tmf2_rv = 0.0;

        *var_guard240_slot = var_guard240;
        *var_guard240_rv_slot = var_guard240_rv;
        *var_guard243_slot = var_guard243;
        *var_guard243_rv_slot = var_guard243_rv;
        *var_nj0_slot = var_nj0;
        *var_nj0_dn0_slot = var_nj0_dn0;
        *var_nj0_dn2_slot = var_nj0_dn2;
        *var_nj0_rv_slot = var_nj0_rv;
        *var_nj1_slot = var_nj1;
        *var_nj1_dn0_slot = var_nj1_dn0;
        *var_nj1_dn2_slot = var_nj1_dn2;
        *var_nj1_rv_slot = var_nj1_rv;
        *var_nja10_slot = var_nja10;
        *var_nja10_dn0_slot = var_nja10_dn0;
        *var_nja10_dn2_slot = var_nja10_dn2;
        *var_nja10_rv_slot = var_nja10_rv;
        *var_nja11_slot = var_nja11;
        *var_nja11_dn0_slot = var_nja11_dn0;
        *var_nja11_dn2_slot = var_nja11_dn2;
        *var_nja11_rv_slot = var_nja11_rv;
        *var_pnn0_slot = var_pnn0;
        *var_pnn0_rv_slot = var_pnn0_rv;
        *var_tmf1_slot = var_tmf1;
        *var_tmf1_dn0_slot = var_tmf1_dn0;
        *var_tmf1_dn2_slot = var_tmf1_dn2;
        *var_tmf1_rv_slot = var_tmf1_rv;
        *var_tmf2_slot = var_tmf2;
        *var_tmf2_dn0_slot = var_tmf2_dn0;
        *var_tmf2_dn2_slot = var_tmf2_dn2;
        *var_tmf2_rv_slot = var_tmf2_rv;
        *var_vha1_slot = var_vha1;
        *var_vha1_rv_slot = var_vha1_rv;
    }

    pub(super) fn stamp_reactive_block_16(
        p: &Parameters,
        var_guard230: f64,
        var_guard231: f64,
        var_guard243: f64,
        var_guard31: f64,
        var_nfabot_i: f64,
        var_phitdinv: f64,
        var_vha1: f64,
        var_vmax: f64,
        var_dfn_sl_slot: &mut f64,
        var_dfn_sl_dn0_slot: &mut f64,
        var_dfn_sl_dn2_slot: &mut f64,
        var_dfn_sl_rv_slot: &mut f64,
        var_dfn_su_slot: &mut f64,
        var_dfn_su_dn0_slot: &mut f64,
        var_dfn_su_dn2_slot: &mut f64,
        var_dfn_su_rv_slot: &mut f64,
        var_dnj1_dv_slot: &mut f64,
        var_dnj1_dv_dn0_slot: &mut f64,
        var_dnj1_dv_dn2_slot: &mut f64,
        var_dnj1_dv_rv_slot: &mut f64,
        var_exp_vmax_over_phitd_bot_slot: &mut f64,
        var_exp_vmax_over_phitd_bot_dn0_slot: &mut f64,
        var_exp_vmax_over_phitd_bot_dn2_slot: &mut f64,
        var_exp_vmax_over_phitd_bot_rv_slot: &mut f64,
        var_guard244_slot: &mut f64,
        var_guard244_rv_slot: &mut f64,
        var_guard245_slot: &mut f64,
        var_guard245_rv_slot: &mut f64,
        var_nj0_slot: &mut f64,
        var_nj0_dn0_slot: &mut f64,
        var_nj0_dn2_slot: &mut f64,
        var_nj0_rv_slot: &mut f64,
        var_nj1_slot: &mut f64,
        var_nj1_dn0_slot: &mut f64,
        var_nj1_dn2_slot: &mut f64,
        var_nj1_rv_slot: &mut f64,
        var_nja11_slot: &mut f64,
        var_nja11_dn0_slot: &mut f64,
        var_nja11_dn2_slot: &mut f64,
        var_nja11_rv_slot: &mut f64,
        var_tmf1_slot: &mut f64,
        var_tmf1_dn0_slot: &mut f64,
        var_tmf1_dn2_slot: &mut f64,
        var_tmf1_rv_slot: &mut f64,
        var_tmf2_slot: &mut f64,
        var_tmf2_dn0_slot: &mut f64,
        var_tmf2_dn2_slot: &mut f64,
        var_tmf2_rv_slot: &mut f64,
    ) {
        let mut var_dfn_sl: f64 = *var_dfn_sl_slot;
        let mut var_dfn_sl_dn0: f64 = *var_dfn_sl_dn0_slot;
        let mut var_dfn_sl_dn2: f64 = *var_dfn_sl_dn2_slot;
        let mut var_dfn_sl_rv: f64 = *var_dfn_sl_rv_slot;
        let mut var_dfn_su: f64 = *var_dfn_su_slot;
        let mut var_dfn_su_dn0: f64 = *var_dfn_su_dn0_slot;
        let mut var_dfn_su_dn2: f64 = *var_dfn_su_dn2_slot;
        let mut var_dfn_su_rv: f64 = *var_dfn_su_rv_slot;
        let mut var_dnj1_dv: f64 = *var_dnj1_dv_slot;
        let mut var_dnj1_dv_dn0: f64 = *var_dnj1_dv_dn0_slot;
        let mut var_dnj1_dv_dn2: f64 = *var_dnj1_dv_dn2_slot;
        let mut var_dnj1_dv_rv: f64 = *var_dnj1_dv_rv_slot;
        let mut var_exp_vmax_over_phitd_bot: f64 = *var_exp_vmax_over_phitd_bot_slot;
        let mut var_exp_vmax_over_phitd_bot_dn0: f64 = *var_exp_vmax_over_phitd_bot_dn0_slot;
        let mut var_exp_vmax_over_phitd_bot_dn2: f64 = *var_exp_vmax_over_phitd_bot_dn2_slot;
        let mut var_exp_vmax_over_phitd_bot_rv: f64 = *var_exp_vmax_over_phitd_bot_rv_slot;
        let mut var_guard244: f64 = *var_guard244_slot;
        let mut var_guard244_rv: f64 = *var_guard244_rv_slot;
        let mut var_guard245: f64 = *var_guard245_slot;
        let mut var_guard245_rv: f64 = *var_guard245_rv_slot;
        let mut var_nj0: f64 = *var_nj0_slot;
        let mut var_nj0_dn0: f64 = *var_nj0_dn0_slot;
        let mut var_nj0_dn2: f64 = *var_nj0_dn2_slot;
        let mut var_nj0_rv: f64 = *var_nj0_rv_slot;
        let mut var_nj1: f64 = *var_nj1_slot;
        let mut var_nj1_dn0: f64 = *var_nj1_dn0_slot;
        let mut var_nj1_dn2: f64 = *var_nj1_dn2_slot;
        let mut var_nj1_rv: f64 = *var_nj1_rv_slot;
        let mut var_nja11: f64 = *var_nja11_slot;
        let mut var_nja11_dn0: f64 = *var_nja11_dn0_slot;
        let mut var_nja11_dn2: f64 = *var_nja11_dn2_slot;
        let mut var_nja11_rv: f64 = *var_nja11_rv_slot;
        let mut var_tmf1: f64 = *var_tmf1_slot;
        let mut var_tmf1_dn0: f64 = *var_tmf1_dn0_slot;
        let mut var_tmf1_dn2: f64 = *var_tmf1_dn2_slot;
        let mut var_tmf1_rv: f64 = *var_tmf1_rv_slot;
        let mut var_tmf2: f64 = *var_tmf2_slot;
        let mut var_tmf2_dn0: f64 = *var_tmf2_dn0_slot;
        let mut var_tmf2_dn2: f64 = *var_tmf2_dn2_slot;
        let mut var_tmf2_rv: f64 = *var_tmf2_rv_slot;

        let (assign14810_e20283, assign14810_e20283_d_n0, assign14810_e20283_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard243 != 0.0)) {
        let (assign14810_e20281, assign14810_e20281_d_n0, assign14810_e20281_d_n2,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
            } else {
                let assign14810_e20280: f64 = (-var_tmf2);
                (assign14810_e20280, (-var_tmf2_dn0), (-var_tmf2_dn2),)
            }
        };
        (assign14810_e20281, assign14810_e20281_d_n0, assign14810_e20281_d_n2,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign14810_e20283;
        var_tmf2_dn0 = assign14810_e20283_d_n0;
        var_tmf2_dn2 = assign14810_e20283_d_n2;
        var_tmf2_rv = 0.0;

        let (assign14820_e20299, assign14820_e20299_d_n0, assign14820_e20299_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard243 != 0.0)) {
        let assign14820_e20294: f64 = (var_tmf1 * var_tmf1);
        let assign14820_e20296: f64 = (assign14820_e20294 + var_tmf2);
        let assign14820_e20297: f64 = (assign14820_e20296).sqrt();
        (assign14820_e20297, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign14820_e20297)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign14820_e20297)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign14820_e20299;
        var_tmf2_dn0 = assign14820_e20299_d_n0;
        var_tmf2_dn2 = assign14820_e20299_d_n2;
        var_tmf2_rv = 0.0;

        let (assign14830_e20316, assign14830_e20316_d_n0, assign14830_e20316_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard243 != 0.0)) {
        let assign14830_e20312: f64 = (var_tmf1 / var_tmf2);
        let assign14830_e20313: f64 = (1.0 + assign14830_e20312);
        let assign14830_e20314: f64 = (0.5 * assign14830_e20313);
        (assign14830_e20314, (0.5 * (((var_tmf1_dn0 * var_tmf2) - (var_tmf1 * var_tmf2_dn0)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn2 * var_tmf2) - (var_tmf1 * var_tmf2_dn2)) / (var_tmf2 * var_tmf2))),)
    } else {
        (var_dfn_su, var_dfn_su_dn0, var_dfn_su_dn2,)
    }
};
        var_dfn_su = assign14830_e20316;
        var_dfn_su_dn0 = assign14830_e20316_d_n0;
        var_dfn_su_dn2 = assign14830_e20316_d_n2;
        var_dfn_su_rv = 0.0;

        let (assign14840_e20333, assign14840_e20333_d_n0, assign14840_e20333_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard243 != 0.0)) {
        let assign14840_e20329: f64 = (var_tmf1 + var_tmf2);
        let assign14840_e20330: f64 = (0.5 * assign14840_e20329);
        let assign14840_e20331: f64 = (p.p85 - assign14840_e20330);
        (assign14840_e20331, (-(0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (-(0.5 * (var_tmf1_dn2 + var_tmf2_dn2))),)
    } else {
        (var_nja11, var_nja11_dn0, var_nja11_dn2,)
    }
};
        var_nja11 = assign14840_e20333;
        var_nja11_dn0 = assign14840_e20333_d_n0;
        var_nja11_dn2 = assign14840_e20333_d_n2;
        var_nja11_rv = 0.0;

        let (assign14850_e20348, assign14850_e20348_d_n0, assign14850_e20348_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard243 != 0.0)) {
        let assign14850_e20344: f64 = (var_nja11 - var_nfabot_i);
        let assign14850_e20346: f64 = (assign14850_e20344 - 0.01);
        (assign14850_e20346, var_nja11_dn0, var_nja11_dn2,)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2,)
    }
};
        var_tmf1 = assign14850_e20348;
        var_tmf1_dn0 = assign14850_e20348_d_n0;
        var_tmf1_dn2 = assign14850_e20348_d_n2;
        var_tmf1_rv = 0.0;

        let (assign14860_e20363, assign14860_e20363_d_n0, assign14860_e20363_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard243 != 0.0)) {
        let assign14860_e20359: f64 = (4.0 * var_nfabot_i);
        let assign14860_e20361: f64 = (assign14860_e20359 * 0.01);
        (assign14860_e20361, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign14860_e20363;
        var_tmf2_dn0 = assign14860_e20363_d_n0;
        var_tmf2_dn2 = assign14860_e20363_d_n2;
        var_tmf2_rv = 0.0;

        let (assign14870_e20380, assign14870_e20380_d_n0, assign14870_e20380_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard243 != 0.0)) {
        let (assign14870_e20378, assign14870_e20378_d_n0, assign14870_e20378_d_n2,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
            } else {
                let assign14870_e20377: f64 = (-var_tmf2);
                (assign14870_e20377, (-var_tmf2_dn0), (-var_tmf2_dn2),)
            }
        };
        (assign14870_e20378, assign14870_e20378_d_n0, assign14870_e20378_d_n2,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign14870_e20380;
        var_tmf2_dn0 = assign14870_e20380_d_n0;
        var_tmf2_dn2 = assign14870_e20380_d_n2;
        var_tmf2_rv = 0.0;

        let (assign14880_e20396, assign14880_e20396_d_n0, assign14880_e20396_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard243 != 0.0)) {
        let assign14880_e20391: f64 = (var_tmf1 * var_tmf1);
        let assign14880_e20393: f64 = (assign14880_e20391 + var_tmf2);
        let assign14880_e20394: f64 = (assign14880_e20393).sqrt();
        (assign14880_e20394, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign14880_e20394)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign14880_e20394)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign14880_e20396;
        var_tmf2_dn0 = assign14880_e20396_d_n0;
        var_tmf2_dn2 = assign14880_e20396_d_n2;
        var_tmf2_rv = 0.0;

        let (assign14890_e20413, assign14890_e20413_d_n0, assign14890_e20413_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard243 != 0.0)) {
        let assign14890_e20409: f64 = (var_tmf1 / var_tmf2);
        let assign14890_e20410: f64 = (1.0 + assign14890_e20409);
        let assign14890_e20411: f64 = (0.5 * assign14890_e20410);
        (assign14890_e20411, (0.5 * (((var_tmf1_dn0 * var_tmf2) - (var_tmf1 * var_tmf2_dn0)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn2 * var_tmf2) - (var_tmf1 * var_tmf2_dn2)) / (var_tmf2 * var_tmf2))),)
    } else {
        (var_dfn_sl, var_dfn_sl_dn0, var_dfn_sl_dn2,)
    }
};
        var_dfn_sl = assign14890_e20413;
        var_dfn_sl_dn0 = assign14890_e20413_d_n0;
        var_dfn_sl_dn2 = assign14890_e20413_d_n2;
        var_dfn_sl_rv = 0.0;

        let (assign14900_e20430, assign14900_e20430_d_n0, assign14900_e20430_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard243 != 0.0)) {
        let assign14900_e20426: f64 = (var_tmf1 + var_tmf2);
        let assign14900_e20427: f64 = (0.5 * assign14900_e20426);
        let assign14900_e20428: f64 = (var_nfabot_i + assign14900_e20427);
        (assign14900_e20428, (0.5 * (var_tmf1_dn0 + var_tmf2_dn0)), (0.5 * (var_tmf1_dn2 + var_tmf2_dn2)),)
    } else {
        (var_nj1, var_nj1_dn0, var_nj1_dn2,)
    }
};
        var_nj1 = assign14900_e20430;
        var_nj1_dn0 = assign14900_e20430_d_n0;
        var_nj1_dn2 = assign14900_e20430_d_n2;
        var_nj1_rv = 0.0;

        let (assign14910_e20445, assign14910_e20445_d_n0, assign14910_e20445_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard243 != 0.0)) {
        let assign14910_e20441: f64 = (p.p85 - var_nj0);
        let assign14910_e20443: f64 = (assign14910_e20441 - 0.01);
        (assign14910_e20443, (-var_nj0_dn0), (-var_nj0_dn2),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2,)
    }
};
        var_tmf1 = assign14910_e20445;
        var_tmf1_dn0 = assign14910_e20445_d_n0;
        var_tmf1_dn2 = assign14910_e20445_d_n2;
        var_tmf1_rv = 0.0;

        let (assign14920_e20460, assign14920_e20460_d_n0, assign14920_e20460_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard243 != 0.0)) {
        let assign14920_e20456: f64 = (4.0 * p.p85);
        let assign14920_e20458: f64 = (assign14920_e20456 * 0.01);
        (assign14920_e20458, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign14920_e20460;
        var_tmf2_dn0 = assign14920_e20460_d_n0;
        var_tmf2_dn2 = assign14920_e20460_d_n2;
        var_tmf2_rv = 0.0;

        let (assign14930_e20477, assign14930_e20477_d_n0, assign14930_e20477_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard243 != 0.0)) {
        let (assign14930_e20475, assign14930_e20475_d_n0, assign14930_e20475_d_n2,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
            } else {
                let assign14930_e20474: f64 = (-var_tmf2);
                (assign14930_e20474, (-var_tmf2_dn0), (-var_tmf2_dn2),)
            }
        };
        (assign14930_e20475, assign14930_e20475_d_n0, assign14930_e20475_d_n2,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign14930_e20477;
        var_tmf2_dn0 = assign14930_e20477_d_n0;
        var_tmf2_dn2 = assign14930_e20477_d_n2;
        var_tmf2_rv = 0.0;

        let (assign14940_e20493, assign14940_e20493_d_n0, assign14940_e20493_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard243 != 0.0)) {
        let assign14940_e20488: f64 = (var_tmf1 * var_tmf1);
        let assign14940_e20490: f64 = (assign14940_e20488 + var_tmf2);
        let assign14940_e20491: f64 = (assign14940_e20490).sqrt();
        (assign14940_e20491, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign14940_e20491)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign14940_e20491)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign14940_e20493;
        var_tmf2_dn0 = assign14940_e20493_d_n0;
        var_tmf2_dn2 = assign14940_e20493_d_n2;
        var_tmf2_rv = 0.0;

        let (assign14950_e20510, assign14950_e20510_d_n0, assign14950_e20510_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard243 != 0.0)) {
        let assign14950_e20506: f64 = (var_tmf1 + var_tmf2);
        let assign14950_e20507: f64 = (0.5 * assign14950_e20506);
        let assign14950_e20508: f64 = (p.p85 - assign14950_e20507);
        (assign14950_e20508, (-(0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (-(0.5 * (var_tmf1_dn2 + var_tmf2_dn2))),)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn2,)
    }
};
        var_nj0 = assign14950_e20510;
        var_nj0_dn0 = assign14950_e20510_d_n0;
        var_nj0_dn2 = assign14950_e20510_d_n2;
        var_nj0_rv = 0.0;

        let (assign14960_e20525, assign14960_e20525_d_n0, assign14960_e20525_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard243 != 0.0)) {
        let assign14960_e20521: f64 = (var_nj0 - var_nfabot_i);
        let assign14960_e20523: f64 = (assign14960_e20521 - 0.01);
        (assign14960_e20523, var_nj0_dn0, var_nj0_dn2,)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2,)
    }
};
        var_tmf1 = assign14960_e20525;
        var_tmf1_dn0 = assign14960_e20525_d_n0;
        var_tmf1_dn2 = assign14960_e20525_d_n2;
        var_tmf1_rv = 0.0;

        let (assign14970_e20540, assign14970_e20540_d_n0, assign14970_e20540_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard243 != 0.0)) {
        let assign14970_e20536: f64 = (4.0 * var_nfabot_i);
        let assign14970_e20538: f64 = (assign14970_e20536 * 0.01);
        (assign14970_e20538, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign14970_e20540;
        var_tmf2_dn0 = assign14970_e20540_d_n0;
        var_tmf2_dn2 = assign14970_e20540_d_n2;
        var_tmf2_rv = 0.0;

        let (assign14980_e20557, assign14980_e20557_d_n0, assign14980_e20557_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard243 != 0.0)) {
        let (assign14980_e20555, assign14980_e20555_d_n0, assign14980_e20555_d_n2,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
            } else {
                let assign14980_e20554: f64 = (-var_tmf2);
                (assign14980_e20554, (-var_tmf2_dn0), (-var_tmf2_dn2),)
            }
        };
        (assign14980_e20555, assign14980_e20555_d_n0, assign14980_e20555_d_n2,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign14980_e20557;
        var_tmf2_dn0 = assign14980_e20557_d_n0;
        var_tmf2_dn2 = assign14980_e20557_d_n2;
        var_tmf2_rv = 0.0;

        let (assign14990_e20573, assign14990_e20573_d_n0, assign14990_e20573_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard243 != 0.0)) {
        let assign14990_e20568: f64 = (var_tmf1 * var_tmf1);
        let assign14990_e20570: f64 = (assign14990_e20568 + var_tmf2);
        let assign14990_e20571: f64 = (assign14990_e20570).sqrt();
        (assign14990_e20571, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign14990_e20571)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign14990_e20571)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign14990_e20573;
        var_tmf2_dn0 = assign14990_e20573_d_n0;
        var_tmf2_dn2 = assign14990_e20573_d_n2;
        var_tmf2_rv = 0.0;

        let (assign15000_e20590, assign15000_e20590_d_n0, assign15000_e20590_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard243 != 0.0)) {
        let assign15000_e20586: f64 = (var_tmf1 + var_tmf2);
        let assign15000_e20587: f64 = (0.5 * assign15000_e20586);
        let assign15000_e20588: f64 = (var_nfabot_i + assign15000_e20587);
        (assign15000_e20588, (0.5 * (var_tmf1_dn0 + var_tmf2_dn0)), (0.5 * (var_tmf1_dn2 + var_tmf2_dn2)),)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn2,)
    }
};
        var_nj0 = assign15000_e20590;
        var_nj0_dn0 = assign15000_e20590_d_n0;
        var_nj0_dn2 = assign15000_e20590_d_n2;
        var_nj0_rv = 0.0;

        let (assign15010_e20605, assign15010_e20605_d_n0, assign15010_e20605_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard243 != 0.0)) {
        let assign15010_e20601: f64 = (p.p86 * var_dfn_su);
        let assign15010_e20603: f64 = (assign15010_e20601 * var_dfn_sl);
        (assign15010_e20603, (((p.p86 * var_dfn_su_dn0) * var_dfn_sl) + (assign15010_e20601 * var_dfn_sl_dn0)), (((p.p86 * var_dfn_su_dn2) * var_dfn_sl) + (assign15010_e20601 * var_dfn_sl_dn2)),)
    } else {
        (var_dnj1_dv, var_dnj1_dv_dn0, var_dnj1_dv_dn2,)
    }
};
        var_dnj1_dv = assign15010_e20605;
        var_dnj1_dv_dn0 = assign15010_e20605_d_n0;
        var_dnj1_dv_dn2 = assign15010_e20605_d_n2;
        var_dnj1_dv_rv = 0.0;

        let (assign15020_e20617, assign15020_e20617_d_n0, assign15020_e20617_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard243 == 0.0)) {
        (var_nfabot_i, 0.0, 0.0,)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn2,)
    }
};
        var_nj0 = assign15020_e20617;
        var_nj0_dn0 = assign15020_e20617_d_n0;
        var_nj0_dn2 = assign15020_e20617_d_n2;
        var_nj0_rv = 0.0;

        let (assign15030_e20629, assign15030_e20629_d_n0, assign15030_e20629_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard243 == 0.0)) {
        (var_nfabot_i, 0.0, 0.0,)
    } else {
        (var_nj1, var_nj1_dn0, var_nj1_dn2,)
    }
};
        var_nj1 = assign15030_e20629;
        var_nj1_dn0 = assign15030_e20629_d_n0;
        var_nj1_dn2 = assign15030_e20629_d_n2;
        var_nj1_rv = 0.0;

        let (assign15040_e20641, assign15040_e20641_d_n0, assign15040_e20641_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard243 == 0.0)) {
        (0.0, 0.0, 0.0,)
    } else {
        (var_dnj1_dv, var_dnj1_dv_dn0, var_dnj1_dv_dn2,)
    }
};
        var_dnj1_dv = assign15040_e20641;
        var_dnj1_dv_dn0 = assign15040_e20641_d_n0;
        var_dnj1_dv_dn2 = assign15040_e20641_d_n2;
        var_dnj1_dv_rv = 0.0;

        let assign15050_e20645: f64 = (var_vmax / var_nj1);
        let assign15050_e20649: f64 = (var_nj1 - var_nj0);
        let assign15050_e20650: f64 = (var_vha1 * assign15050_e20649);
        let assign15050_e20653: f64 = (var_nj0 * p.p85);
        let assign15050_e20654: f64 = (assign15050_e20650 / assign15050_e20653);
        let assign15050_e20655: f64 = (assign15050_e20645 + assign15050_e20654);
        let assign15050_e20656: f64 = (var_phitdinv * assign15050_e20655);
        let assign15050_e20657: f64 = (assign15050_e20656).abs();
        let assign15050_e20659: f64 = if assign15050_e20657 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard244 = assign15050_e20659;
        var_guard244_rv = 0.0;

        let (assign15060_e20685, assign15060_e20685_d_n0, assign15060_e20685_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard244 != 0.0)) {
        let assign15060_e20671: f64 = (var_vmax / var_nj1);
        let assign15060_e20675: f64 = (var_nj1 - var_nj0);
        let assign15060_e20676: f64 = (var_vha1 * assign15060_e20675);
        let assign15060_e20679: f64 = (var_nj0 * p.p85);
        let assign15060_e20680: f64 = (assign15060_e20676 / assign15060_e20679);
        let assign15060_e20681: f64 = (assign15060_e20671 + assign15060_e20680);
        let assign15060_e20682: f64 = (var_phitdinv * assign15060_e20681);
        let assign15060_e20683: f64 = (assign15060_e20682).exp();
        (assign15060_e20683, (assign15060_e20683 * (var_phitdinv * ((-((var_vmax * var_nj1_dn0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn0 - var_nj0_dn0)) * assign15060_e20679) - (assign15060_e20676 * (var_nj0_dn0 * p.p85))) / (assign15060_e20679 * assign15060_e20679))))), (assign15060_e20683 * (var_phitdinv * ((-((var_vmax * var_nj1_dn2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn2 - var_nj0_dn2)) * assign15060_e20679) - (assign15060_e20676 * (var_nj0_dn2 * p.p85))) / (assign15060_e20679 * assign15060_e20679))))),)
    } else {
        (var_exp_vmax_over_phitd_bot, var_exp_vmax_over_phitd_bot_dn0, var_exp_vmax_over_phitd_bot_dn2,)
    }
};
        var_exp_vmax_over_phitd_bot = assign15060_e20685;
        var_exp_vmax_over_phitd_bot_dn0 = assign15060_e20685_d_n0;
        var_exp_vmax_over_phitd_bot_dn2 = assign15060_e20685_d_n2;
        var_exp_vmax_over_phitd_bot_rv = 0.0;

        let assign15070_e20689: f64 = (var_vmax / var_nj1);
        let assign15070_e20693: f64 = (var_nj1 - var_nj0);
        let assign15070_e20694: f64 = (var_vha1 * assign15070_e20693);
        let assign15070_e20697: f64 = (var_nj0 * p.p85);
        let assign15070_e20698: f64 = (assign15070_e20694 / assign15070_e20697);
        let assign15070_e20699: f64 = (assign15070_e20689 + assign15070_e20698);
        let assign15070_e20700: f64 = (var_phitdinv * assign15070_e20699);
        let assign15070_e20702: f64 = (-230.25850929940458);
        let assign15070_e20703: f64 = if assign15070_e20700 < assign15070_e20702 { 1.0 } else { 0.0 };
        var_guard245 = assign15070_e20703;
        var_guard245_rv = 0.0;

        let (assign15080_e20784, assign15080_e20784_d_n0, assign15080_e20784_d_n2,) = {
    if (((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard244 == 0.0)) && (var_guard245 != 0.0)) {
        let assign15080_e20718: f64 = (-230.25850929940458);
        let assign15080_e20722: f64 = (var_vmax / var_nj1);
        let assign15080_e20726: f64 = (var_nj1 - var_nj0);
        let assign15080_e20727: f64 = (var_vha1 * assign15080_e20726);
        let assign15080_e20730: f64 = (var_nj0 * p.p85);
        let assign15080_e20731: f64 = (assign15080_e20727 / assign15080_e20730);
        let assign15080_e20732: f64 = (assign15080_e20722 + assign15080_e20731);
        let assign15080_e20733: f64 = (var_phitdinv * assign15080_e20732);
        let assign15080_e20734: f64 = (assign15080_e20718 - assign15080_e20733);
        let assign15080_e20738: f64 = (-230.25850929940458);
        let assign15080_e20742: f64 = (var_vmax / var_nj1);
        let assign15080_e20746: f64 = (var_nj1 - var_nj0);
        let assign15080_e20747: f64 = (var_vha1 * assign15080_e20746);
        let assign15080_e20750: f64 = (var_nj0 * p.p85);
        let assign15080_e20751: f64 = (assign15080_e20747 / assign15080_e20750);
        let assign15080_e20752: f64 = (assign15080_e20742 + assign15080_e20751);
        let assign15080_e20753: f64 = (var_phitdinv * assign15080_e20752);
        let assign15080_e20754: f64 = (assign15080_e20738 - assign15080_e20753);
        let assign15080_e20757: f64 = (-230.25850929940458);
        let assign15080_e20761: f64 = (var_vmax / var_nj1);
        let assign15080_e20765: f64 = (var_nj1 - var_nj0);
        let assign15080_e20766: f64 = (var_vha1 * assign15080_e20765);
        let assign15080_e20769: f64 = (var_nj0 * p.p85);
        let assign15080_e20770: f64 = (assign15080_e20766 / assign15080_e20769);
        let assign15080_e20771: f64 = (assign15080_e20761 + assign15080_e20770);
        let assign15080_e20772: f64 = (var_phitdinv * assign15080_e20771);
        let assign15080_e20773: f64 = (assign15080_e20757 - assign15080_e20772);
        let assign15080_e20775: f64 = (assign15080_e20773 * 0.3333333333333333);
        let assign15080_e20776: f64 = (1.0 + assign15080_e20775);
        let assign15080_e20777: f64 = (assign15080_e20754 * assign15080_e20776);
        let assign15080_e20778: f64 = (0.5 * assign15080_e20777);
        let assign15080_e20779: f64 = (1.0 + assign15080_e20778);
        let assign15080_e20780: f64 = (assign15080_e20734 * assign15080_e20779);
        let assign15080_e20781: f64 = (1.0 + assign15080_e20780);
        let assign15080_e20782: f64 = (1e-100 / assign15080_e20781);
        (assign15080_e20782, (-((1e-100 * (((-(var_phitdinv * ((-((var_vmax * var_nj1_dn0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn0 - var_nj0_dn0)) * assign15080_e20730) - (assign15080_e20727 * (var_nj0_dn0 * p.p85))) / (assign15080_e20730 * assign15080_e20730))))) * assign15080_e20779) + (assign15080_e20734 * (0.5 * (((-(var_phitdinv * ((-((var_vmax * var_nj1_dn0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn0 - var_nj0_dn0)) * assign15080_e20750) - (assign15080_e20747 * (var_nj0_dn0 * p.p85))) / (assign15080_e20750 * assign15080_e20750))))) * assign15080_e20776) + (assign15080_e20754 * ((-(var_phitdinv * ((-((var_vmax * var_nj1_dn0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn0 - var_nj0_dn0)) * assign15080_e20769) - (assign15080_e20766 * (var_nj0_dn0 * p.p85))) / (assign15080_e20769 * assign15080_e20769))))) * 0.3333333333333333))))))) / (assign15080_e20781 * assign15080_e20781))), (-((1e-100 * (((-(var_phitdinv * ((-((var_vmax * var_nj1_dn2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn2 - var_nj0_dn2)) * assign15080_e20730) - (assign15080_e20727 * (var_nj0_dn2 * p.p85))) / (assign15080_e20730 * assign15080_e20730))))) * assign15080_e20779) + (assign15080_e20734 * (0.5 * (((-(var_phitdinv * ((-((var_vmax * var_nj1_dn2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn2 - var_nj0_dn2)) * assign15080_e20750) - (assign15080_e20747 * (var_nj0_dn2 * p.p85))) / (assign15080_e20750 * assign15080_e20750))))) * assign15080_e20776) + (assign15080_e20754 * ((-(var_phitdinv * ((-((var_vmax * var_nj1_dn2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn2 - var_nj0_dn2)) * assign15080_e20769) - (assign15080_e20766 * (var_nj0_dn2 * p.p85))) / (assign15080_e20769 * assign15080_e20769))))) * 0.3333333333333333))))))) / (assign15080_e20781 * assign15080_e20781))),)
    } else {
        (var_exp_vmax_over_phitd_bot, var_exp_vmax_over_phitd_bot_dn0, var_exp_vmax_over_phitd_bot_dn2,)
    }
};
        var_exp_vmax_over_phitd_bot = assign15080_e20784;
        var_exp_vmax_over_phitd_bot_dn0 = assign15080_e20784_d_n0;
        var_exp_vmax_over_phitd_bot_dn2 = assign15080_e20784_d_n2;
        var_exp_vmax_over_phitd_bot_rv = 0.0;

        let (assign15090_e20863, assign15090_e20863_d_n0, assign15090_e20863_d_n2,) = {
    if (((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard244 == 0.0)) && (var_guard245 == 0.0)) {
        let assign15090_e20802: f64 = (var_vmax / var_nj1);
        let assign15090_e20806: f64 = (var_nj1 - var_nj0);
        let assign15090_e20807: f64 = (var_vha1 * assign15090_e20806);
        let assign15090_e20810: f64 = (var_nj0 * p.p85);
        let assign15090_e20811: f64 = (assign15090_e20807 / assign15090_e20810);
        let assign15090_e20812: f64 = (assign15090_e20802 + assign15090_e20811);
        let assign15090_e20813: f64 = (var_phitdinv * assign15090_e20812);
        let assign15090_e20815: f64 = (assign15090_e20813 - 230.25850929940458);
        let assign15090_e20821: f64 = (var_vmax / var_nj1);
        let assign15090_e20825: f64 = (var_nj1 - var_nj0);
        let assign15090_e20826: f64 = (var_vha1 * assign15090_e20825);
        let assign15090_e20829: f64 = (var_nj0 * p.p85);
        let assign15090_e20830: f64 = (assign15090_e20826 / assign15090_e20829);
        let assign15090_e20831: f64 = (assign15090_e20821 + assign15090_e20830);
        let assign15090_e20832: f64 = (var_phitdinv * assign15090_e20831);
        let assign15090_e20834: f64 = (assign15090_e20832 - 230.25850929940458);
        let assign15090_e20839: f64 = (var_vmax / var_nj1);
        let assign15090_e20843: f64 = (var_nj1 - var_nj0);
        let assign15090_e20844: f64 = (var_vha1 * assign15090_e20843);
        let assign15090_e20847: f64 = (var_nj0 * p.p85);
        let assign15090_e20848: f64 = (assign15090_e20844 / assign15090_e20847);
        let assign15090_e20849: f64 = (assign15090_e20839 + assign15090_e20848);
        let assign15090_e20850: f64 = (var_phitdinv * assign15090_e20849);
        let assign15090_e20852: f64 = (assign15090_e20850 - 230.25850929940458);
        let assign15090_e20854: f64 = (assign15090_e20852 * 0.3333333333333333);
        let assign15090_e20855: f64 = (1.0 + assign15090_e20854);
        let assign15090_e20856: f64 = (assign15090_e20834 * assign15090_e20855);
        let assign15090_e20857: f64 = (0.5 * assign15090_e20856);
        let assign15090_e20858: f64 = (1.0 + assign15090_e20857);
        let assign15090_e20859: f64 = (assign15090_e20815 * assign15090_e20858);
        let assign15090_e20860: f64 = (1.0 + assign15090_e20859);
        let assign15090_e20861: f64 = (1e100 * assign15090_e20860);
        (assign15090_e20861, (1e100 * (((var_phitdinv * ((-((var_vmax * var_nj1_dn0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn0 - var_nj0_dn0)) * assign15090_e20810) - (assign15090_e20807 * (var_nj0_dn0 * p.p85))) / (assign15090_e20810 * assign15090_e20810)))) * assign15090_e20858) + (assign15090_e20815 * (0.5 * (((var_phitdinv * ((-((var_vmax * var_nj1_dn0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn0 - var_nj0_dn0)) * assign15090_e20829) - (assign15090_e20826 * (var_nj0_dn0 * p.p85))) / (assign15090_e20829 * assign15090_e20829)))) * assign15090_e20855) + (assign15090_e20834 * ((var_phitdinv * ((-((var_vmax * var_nj1_dn0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn0 - var_nj0_dn0)) * assign15090_e20847) - (assign15090_e20844 * (var_nj0_dn0 * p.p85))) / (assign15090_e20847 * assign15090_e20847)))) * 0.3333333333333333))))))), (1e100 * (((var_phitdinv * ((-((var_vmax * var_nj1_dn2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn2 - var_nj0_dn2)) * assign15090_e20810) - (assign15090_e20807 * (var_nj0_dn2 * p.p85))) / (assign15090_e20810 * assign15090_e20810)))) * assign15090_e20858) + (assign15090_e20815 * (0.5 * (((var_phitdinv * ((-((var_vmax * var_nj1_dn2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn2 - var_nj0_dn2)) * assign15090_e20829) - (assign15090_e20826 * (var_nj0_dn2 * p.p85))) / (assign15090_e20829 * assign15090_e20829)))) * assign15090_e20855) + (assign15090_e20834 * ((var_phitdinv * ((-((var_vmax * var_nj1_dn2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn2 - var_nj0_dn2)) * assign15090_e20847) - (assign15090_e20844 * (var_nj0_dn2 * p.p85))) / (assign15090_e20847 * assign15090_e20847)))) * 0.3333333333333333))))))),)
    } else {
        (var_exp_vmax_over_phitd_bot, var_exp_vmax_over_phitd_bot_dn0, var_exp_vmax_over_phitd_bot_dn2,)
    }
};
        var_exp_vmax_over_phitd_bot = assign15090_e20863;
        var_exp_vmax_over_phitd_bot_dn0 = assign15090_e20863_d_n0;
        var_exp_vmax_over_phitd_bot_dn2 = assign15090_e20863_d_n2;
        var_exp_vmax_over_phitd_bot_rv = 0.0;

        *var_dfn_sl_slot = var_dfn_sl;
        *var_dfn_sl_dn0_slot = var_dfn_sl_dn0;
        *var_dfn_sl_dn2_slot = var_dfn_sl_dn2;
        *var_dfn_sl_rv_slot = var_dfn_sl_rv;
        *var_dfn_su_slot = var_dfn_su;
        *var_dfn_su_dn0_slot = var_dfn_su_dn0;
        *var_dfn_su_dn2_slot = var_dfn_su_dn2;
        *var_dfn_su_rv_slot = var_dfn_su_rv;
        *var_dnj1_dv_slot = var_dnj1_dv;
        *var_dnj1_dv_dn0_slot = var_dnj1_dv_dn0;
        *var_dnj1_dv_dn2_slot = var_dnj1_dv_dn2;
        *var_dnj1_dv_rv_slot = var_dnj1_dv_rv;
        *var_exp_vmax_over_phitd_bot_slot = var_exp_vmax_over_phitd_bot;
        *var_exp_vmax_over_phitd_bot_dn0_slot = var_exp_vmax_over_phitd_bot_dn0;
        *var_exp_vmax_over_phitd_bot_dn2_slot = var_exp_vmax_over_phitd_bot_dn2;
        *var_exp_vmax_over_phitd_bot_rv_slot = var_exp_vmax_over_phitd_bot_rv;
        *var_guard244_slot = var_guard244;
        *var_guard244_rv_slot = var_guard244_rv;
        *var_guard245_slot = var_guard245;
        *var_guard245_rv_slot = var_guard245_rv;
        *var_nj0_slot = var_nj0;
        *var_nj0_dn0_slot = var_nj0_dn0;
        *var_nj0_dn2_slot = var_nj0_dn2;
        *var_nj0_rv_slot = var_nj0_rv;
        *var_nj1_slot = var_nj1;
        *var_nj1_dn0_slot = var_nj1_dn0;
        *var_nj1_dn2_slot = var_nj1_dn2;
        *var_nj1_rv_slot = var_nj1_rv;
        *var_nja11_slot = var_nja11;
        *var_nja11_dn0_slot = var_nja11_dn0;
        *var_nja11_dn2_slot = var_nja11_dn2;
        *var_nja11_rv_slot = var_nja11_rv;
        *var_tmf1_slot = var_tmf1;
        *var_tmf1_dn0_slot = var_tmf1_dn0;
        *var_tmf1_dn2_slot = var_tmf1_dn2;
        *var_tmf1_rv_slot = var_tmf1_rv;
        *var_tmf2_slot = var_tmf2;
        *var_tmf2_dn0_slot = var_tmf2_dn0;
        *var_tmf2_dn2_slot = var_tmf2_dn2;
        *var_tmf2_rv_slot = var_tmf2_rv;
    }

    pub(super) fn stamp_reactive_block_17(
        p: &Parameters,
        var_exp_vmax_over_phitd_bot: f64,
        var_exp_vmax_over_phitd_bot_dn0: f64,
        var_exp_vmax_over_phitd_bot_dn2: f64,
        var_guard230: f64,
        var_guard231: f64,
        var_guard31: f64,
        var_ndisti_i: f64,
        var_nfasti_i: f64,
        var_nin: f64,
        var_phitdinv: f64,
        var_v3: f64,
        var_vmax: f64,
        var_dfn_sl_slot: &mut f64,
        var_dfn_sl_dn0_slot: &mut f64,
        var_dfn_sl_dn2_slot: &mut f64,
        var_dfn_sl_rv_slot: &mut f64,
        var_dfn_su_slot: &mut f64,
        var_dfn_su_dn0_slot: &mut f64,
        var_dfn_su_dn2_slot: &mut f64,
        var_dfn_su_rv_slot: &mut f64,
        var_dnj1_dv_slot: &mut f64,
        var_dnj1_dv_dn0_slot: &mut f64,
        var_dnj1_dv_dn2_slot: &mut f64,
        var_dnj1_dv_rv_slot: &mut f64,
        var_dvmax_over_phitd_dv_slot: &mut f64,
        var_dvmax_over_phitd_dv_dn0_slot: &mut f64,
        var_dvmax_over_phitd_dv_dn2_slot: &mut f64,
        var_dvmax_over_phitd_dv_rv_slot: &mut f64,
        var_guard246_slot: &mut f64,
        var_guard246_rv_slot: &mut f64,
        var_idmultbot_slot: &mut f64,
        var_idmultbot_dn0_slot: &mut f64,
        var_idmultbot_dn2_slot: &mut f64,
        var_idmultbot_rv_slot: &mut f64,
        var_nj0_slot: &mut f64,
        var_nj0_dn0_slot: &mut f64,
        var_nj0_dn2_slot: &mut f64,
        var_nj0_rv_slot: &mut f64,
        var_nj1_slot: &mut f64,
        var_nj1_dn0_slot: &mut f64,
        var_nj1_dn2_slot: &mut f64,
        var_nj1_rv_slot: &mut f64,
        var_nja10_slot: &mut f64,
        var_nja10_dn0_slot: &mut f64,
        var_nja10_dn2_slot: &mut f64,
        var_nja10_rv_slot: &mut f64,
        var_nja11_slot: &mut f64,
        var_nja11_dn0_slot: &mut f64,
        var_nja11_dn2_slot: &mut f64,
        var_nja11_rv_slot: &mut f64,
        var_pnn0_slot: &mut f64,
        var_pnn0_rv_slot: &mut f64,
        var_tmf1_slot: &mut f64,
        var_tmf1_dn0_slot: &mut f64,
        var_tmf1_dn2_slot: &mut f64,
        var_tmf1_rv_slot: &mut f64,
        var_tmf2_slot: &mut f64,
        var_tmf2_dn0_slot: &mut f64,
        var_tmf2_dn2_slot: &mut f64,
        var_tmf2_rv_slot: &mut f64,
        var_vha1_slot: &mut f64,
        var_vha1_rv_slot: &mut f64,
    ) {
        let mut var_dfn_sl: f64 = *var_dfn_sl_slot;
        let mut var_dfn_sl_dn0: f64 = *var_dfn_sl_dn0_slot;
        let mut var_dfn_sl_dn2: f64 = *var_dfn_sl_dn2_slot;
        let mut var_dfn_sl_rv: f64 = *var_dfn_sl_rv_slot;
        let mut var_dfn_su: f64 = *var_dfn_su_slot;
        let mut var_dfn_su_dn0: f64 = *var_dfn_su_dn0_slot;
        let mut var_dfn_su_dn2: f64 = *var_dfn_su_dn2_slot;
        let mut var_dfn_su_rv: f64 = *var_dfn_su_rv_slot;
        let mut var_dnj1_dv: f64 = *var_dnj1_dv_slot;
        let mut var_dnj1_dv_dn0: f64 = *var_dnj1_dv_dn0_slot;
        let mut var_dnj1_dv_dn2: f64 = *var_dnj1_dv_dn2_slot;
        let mut var_dnj1_dv_rv: f64 = *var_dnj1_dv_rv_slot;
        let mut var_dvmax_over_phitd_dv: f64 = *var_dvmax_over_phitd_dv_slot;
        let mut var_dvmax_over_phitd_dv_dn0: f64 = *var_dvmax_over_phitd_dv_dn0_slot;
        let mut var_dvmax_over_phitd_dv_dn2: f64 = *var_dvmax_over_phitd_dv_dn2_slot;
        let mut var_dvmax_over_phitd_dv_rv: f64 = *var_dvmax_over_phitd_dv_rv_slot;
        let mut var_guard246: f64 = *var_guard246_slot;
        let mut var_guard246_rv: f64 = *var_guard246_rv_slot;
        let mut var_idmultbot: f64 = *var_idmultbot_slot;
        let mut var_idmultbot_dn0: f64 = *var_idmultbot_dn0_slot;
        let mut var_idmultbot_dn2: f64 = *var_idmultbot_dn2_slot;
        let mut var_idmultbot_rv: f64 = *var_idmultbot_rv_slot;
        let mut var_nj0: f64 = *var_nj0_slot;
        let mut var_nj0_dn0: f64 = *var_nj0_dn0_slot;
        let mut var_nj0_dn2: f64 = *var_nj0_dn2_slot;
        let mut var_nj0_rv: f64 = *var_nj0_rv_slot;
        let mut var_nj1: f64 = *var_nj1_slot;
        let mut var_nj1_dn0: f64 = *var_nj1_dn0_slot;
        let mut var_nj1_dn2: f64 = *var_nj1_dn2_slot;
        let mut var_nj1_rv: f64 = *var_nj1_rv_slot;
        let mut var_nja10: f64 = *var_nja10_slot;
        let mut var_nja10_dn0: f64 = *var_nja10_dn0_slot;
        let mut var_nja10_dn2: f64 = *var_nja10_dn2_slot;
        let mut var_nja10_rv: f64 = *var_nja10_rv_slot;
        let mut var_nja11: f64 = *var_nja11_slot;
        let mut var_nja11_dn0: f64 = *var_nja11_dn0_slot;
        let mut var_nja11_dn2: f64 = *var_nja11_dn2_slot;
        let mut var_nja11_rv: f64 = *var_nja11_rv_slot;
        let mut var_pnn0: f64 = *var_pnn0_slot;
        let mut var_pnn0_rv: f64 = *var_pnn0_rv_slot;
        let mut var_tmf1: f64 = *var_tmf1_slot;
        let mut var_tmf1_dn0: f64 = *var_tmf1_dn0_slot;
        let mut var_tmf1_dn2: f64 = *var_tmf1_dn2_slot;
        let mut var_tmf1_rv: f64 = *var_tmf1_rv_slot;
        let mut var_tmf2: f64 = *var_tmf2_slot;
        let mut var_tmf2_dn0: f64 = *var_tmf2_dn0_slot;
        let mut var_tmf2_dn2: f64 = *var_tmf2_dn2_slot;
        let mut var_tmf2_rv: f64 = *var_tmf2_rv_slot;
        let mut var_vha1: f64 = *var_vha1_slot;
        let mut var_vha1_rv: f64 = *var_vha1_rv_slot;

        let (assign15100_e20890, assign15100_e20890_d_n0, assign15100_e20890_d_n2,) = {
    if (((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) {
        let assign15100_e20874: f64 = (var_vmax * var_dnj1_dv);
        let assign15100_e20875: f64 = (var_nj1 - assign15100_e20874);
        let assign15100_e20878: f64 = (var_nj1 * var_nj1);
        let assign15100_e20879: f64 = (assign15100_e20875 / assign15100_e20878);
        let assign15100_e20882: f64 = (var_vha1 * var_dnj1_dv);
        let assign15100_e20885: f64 = (var_nj0 * p.p85);
        let assign15100_e20886: f64 = (assign15100_e20882 / assign15100_e20885);
        let assign15100_e20887: f64 = (assign15100_e20879 + assign15100_e20886);
        let assign15100_e20888: f64 = (var_phitdinv * assign15100_e20887);
        (assign15100_e20888, (var_phitdinv * (((((var_nj1_dn0 - (var_vmax * var_dnj1_dv_dn0)) * assign15100_e20878) - (assign15100_e20875 * ((var_nj1_dn0 * var_nj1) + (var_nj1 * var_nj1_dn0)))) / (assign15100_e20878 * assign15100_e20878)) + ((((var_vha1 * var_dnj1_dv_dn0) * assign15100_e20885) - (assign15100_e20882 * (var_nj0_dn0 * p.p85))) / (assign15100_e20885 * assign15100_e20885)))), (var_phitdinv * (((((var_nj1_dn2 - (var_vmax * var_dnj1_dv_dn2)) * assign15100_e20878) - (assign15100_e20875 * ((var_nj1_dn2 * var_nj1) + (var_nj1 * var_nj1_dn2)))) / (assign15100_e20878 * assign15100_e20878)) + ((((var_vha1 * var_dnj1_dv_dn2) * assign15100_e20885) - (assign15100_e20882 * (var_nj0_dn2 * p.p85))) / (assign15100_e20885 * assign15100_e20885)))),)
    } else {
        (var_dvmax_over_phitd_dv, var_dvmax_over_phitd_dv_dn0, var_dvmax_over_phitd_dv_dn2,)
    }
};
        var_dvmax_over_phitd_dv = assign15100_e20890;
        var_dvmax_over_phitd_dv_dn0 = assign15100_e20890_d_n0;
        var_dvmax_over_phitd_dv_dn2 = assign15100_e20890_d_n2;
        var_dvmax_over_phitd_dv_rv = 0.0;

        let (assign15110_e20907, assign15110_e20907_d_n0, assign15110_e20907_d_n2,) = {
    if (((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) {
        let assign15110_e20900: f64 = (var_v3 - var_vmax);
        let assign15110_e20902: f64 = (assign15110_e20900 * var_dvmax_over_phitd_dv);
        let assign15110_e20903: f64 = (1.0 + assign15110_e20902);
        let assign15110_e20905: f64 = (assign15110_e20903 * var_exp_vmax_over_phitd_bot);
        (assign15110_e20905, (((assign15110_e20900 * var_dvmax_over_phitd_dv_dn0) * var_exp_vmax_over_phitd_bot) + (assign15110_e20903 * var_exp_vmax_over_phitd_bot_dn0)), (((assign15110_e20900 * var_dvmax_over_phitd_dv_dn2) * var_exp_vmax_over_phitd_bot) + (assign15110_e20903 * var_exp_vmax_over_phitd_bot_dn2)),)
    } else {
        (var_idmultbot, var_idmultbot_dn0, var_idmultbot_dn2,)
    }
};
        var_idmultbot = assign15110_e20907;
        var_idmultbot_dn0 = assign15110_e20907_d_n0;
        var_idmultbot_dn2 = assign15110_e20907_d_n2;
        var_idmultbot_rv = 0.0;

        let (assign15120_e20920,) = {
    if (((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) {
        let assign15120_e20916: f64 = (var_nin * var_nin);
        let assign15120_e20918: f64 = (assign15120_e20916 / var_ndisti_i);
        (assign15120_e20918,)
    } else {
        (var_pnn0,)
    }
};
        var_pnn0 = assign15120_e20920;
        var_pnn0_rv = 0.0;

        let (assign15130_e20936,) = {
    if (((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) {
        let assign15130_e20929: f64 = (var_nfasti_i / var_phitdinv);
        let assign15130_e20932: f64 = (var_ndisti_i / var_pnn0);
        let assign15130_e20933: f64 = (assign15130_e20932).ln();
        let assign15130_e20934: f64 = (assign15130_e20929 * assign15130_e20933);
        (assign15130_e20934,)
    } else {
        (var_vha1,)
    }
};
        var_vha1 = assign15130_e20936;
        var_vha1_rv = 0.0;

        let assign15140_e20939: f64 = if var_nfasti_i < p.p85 { 1.0 } else { 0.0 };
        var_guard246 = assign15140_e20939;
        var_guard246_rv = 0.0;

        let (assign15150_e20956, assign15150_e20956_d_n0, assign15150_e20956_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard246 != 0.0)) {
        let assign15150_e20951: f64 = (var_vmax - var_vha1);
        let assign15150_e20952: f64 = (p.p86 * assign15150_e20951);
        let assign15150_e20954: f64 = (assign15150_e20952 + var_nfasti_i);
        (assign15150_e20954, 0.0, 0.0,)
    } else {
        (var_nja10, var_nja10_dn0, var_nja10_dn2,)
    }
};
        var_nja10 = assign15150_e20956;
        var_nja10_dn0 = assign15150_e20956_d_n0;
        var_nja10_dn2 = assign15150_e20956_d_n2;
        var_nja10_rv = 0.0;

        let (assign15160_e20971, assign15160_e20971_d_n0, assign15160_e20971_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard246 != 0.0)) {
        let assign15160_e20968: f64 = (p.p86 * var_vha1);
        let assign15160_e20969: f64 = (var_nfasti_i - assign15160_e20968);
        (assign15160_e20969, 0.0, 0.0,)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn2,)
    }
};
        var_nj0 = assign15160_e20971;
        var_nj0_dn0 = assign15160_e20971_d_n0;
        var_nj0_dn2 = assign15160_e20971_d_n2;
        var_nj0_rv = 0.0;

        let (assign15170_e20986, assign15170_e20986_d_n0, assign15170_e20986_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard246 != 0.0)) {
        let assign15170_e20982: f64 = (p.p85 - var_nja10);
        let assign15170_e20984: f64 = (assign15170_e20982 - 0.01);
        (assign15170_e20984, (-var_nja10_dn0), (-var_nja10_dn2),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2,)
    }
};
        var_tmf1 = assign15170_e20986;
        var_tmf1_dn0 = assign15170_e20986_d_n0;
        var_tmf1_dn2 = assign15170_e20986_d_n2;
        var_tmf1_rv = 0.0;

        let (assign15180_e21001, assign15180_e21001_d_n0, assign15180_e21001_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard246 != 0.0)) {
        let assign15180_e20997: f64 = (4.0 * p.p85);
        let assign15180_e20999: f64 = (assign15180_e20997 * 0.01);
        (assign15180_e20999, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign15180_e21001;
        var_tmf2_dn0 = assign15180_e21001_d_n0;
        var_tmf2_dn2 = assign15180_e21001_d_n2;
        var_tmf2_rv = 0.0;

        let (assign15190_e21018, assign15190_e21018_d_n0, assign15190_e21018_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard246 != 0.0)) {
        let (assign15190_e21016, assign15190_e21016_d_n0, assign15190_e21016_d_n2,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
            } else {
                let assign15190_e21015: f64 = (-var_tmf2);
                (assign15190_e21015, (-var_tmf2_dn0), (-var_tmf2_dn2),)
            }
        };
        (assign15190_e21016, assign15190_e21016_d_n0, assign15190_e21016_d_n2,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign15190_e21018;
        var_tmf2_dn0 = assign15190_e21018_d_n0;
        var_tmf2_dn2 = assign15190_e21018_d_n2;
        var_tmf2_rv = 0.0;

        let (assign15200_e21034, assign15200_e21034_d_n0, assign15200_e21034_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard246 != 0.0)) {
        let assign15200_e21029: f64 = (var_tmf1 * var_tmf1);
        let assign15200_e21031: f64 = (assign15200_e21029 + var_tmf2);
        let assign15200_e21032: f64 = (assign15200_e21031).sqrt();
        (assign15200_e21032, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign15200_e21032)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign15200_e21032)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign15200_e21034;
        var_tmf2_dn0 = assign15200_e21034_d_n0;
        var_tmf2_dn2 = assign15200_e21034_d_n2;
        var_tmf2_rv = 0.0;

        let (assign15210_e21051, assign15210_e21051_d_n0, assign15210_e21051_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard246 != 0.0)) {
        let assign15210_e21047: f64 = (var_tmf1 / var_tmf2);
        let assign15210_e21048: f64 = (1.0 + assign15210_e21047);
        let assign15210_e21049: f64 = (0.5 * assign15210_e21048);
        (assign15210_e21049, (0.5 * (((var_tmf1_dn0 * var_tmf2) - (var_tmf1 * var_tmf2_dn0)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn2 * var_tmf2) - (var_tmf1 * var_tmf2_dn2)) / (var_tmf2 * var_tmf2))),)
    } else {
        (var_dfn_su, var_dfn_su_dn0, var_dfn_su_dn2,)
    }
};
        var_dfn_su = assign15210_e21051;
        var_dfn_su_dn0 = assign15210_e21051_d_n0;
        var_dfn_su_dn2 = assign15210_e21051_d_n2;
        var_dfn_su_rv = 0.0;

        let (assign15220_e21068, assign15220_e21068_d_n0, assign15220_e21068_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard246 != 0.0)) {
        let assign15220_e21064: f64 = (var_tmf1 + var_tmf2);
        let assign15220_e21065: f64 = (0.5 * assign15220_e21064);
        let assign15220_e21066: f64 = (p.p85 - assign15220_e21065);
        (assign15220_e21066, (-(0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (-(0.5 * (var_tmf1_dn2 + var_tmf2_dn2))),)
    } else {
        (var_nja11, var_nja11_dn0, var_nja11_dn2,)
    }
};
        var_nja11 = assign15220_e21068;
        var_nja11_dn0 = assign15220_e21068_d_n0;
        var_nja11_dn2 = assign15220_e21068_d_n2;
        var_nja11_rv = 0.0;

        let (assign15230_e21083, assign15230_e21083_d_n0, assign15230_e21083_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard246 != 0.0)) {
        let assign15230_e21079: f64 = (var_nja11 - var_nfasti_i);
        let assign15230_e21081: f64 = (assign15230_e21079 - 0.01);
        (assign15230_e21081, var_nja11_dn0, var_nja11_dn2,)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2,)
    }
};
        var_tmf1 = assign15230_e21083;
        var_tmf1_dn0 = assign15230_e21083_d_n0;
        var_tmf1_dn2 = assign15230_e21083_d_n2;
        var_tmf1_rv = 0.0;

        let (assign15240_e21098, assign15240_e21098_d_n0, assign15240_e21098_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard246 != 0.0)) {
        let assign15240_e21094: f64 = (4.0 * var_nfasti_i);
        let assign15240_e21096: f64 = (assign15240_e21094 * 0.01);
        (assign15240_e21096, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign15240_e21098;
        var_tmf2_dn0 = assign15240_e21098_d_n0;
        var_tmf2_dn2 = assign15240_e21098_d_n2;
        var_tmf2_rv = 0.0;

        let (assign15250_e21115, assign15250_e21115_d_n0, assign15250_e21115_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard246 != 0.0)) {
        let (assign15250_e21113, assign15250_e21113_d_n0, assign15250_e21113_d_n2,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
            } else {
                let assign15250_e21112: f64 = (-var_tmf2);
                (assign15250_e21112, (-var_tmf2_dn0), (-var_tmf2_dn2),)
            }
        };
        (assign15250_e21113, assign15250_e21113_d_n0, assign15250_e21113_d_n2,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign15250_e21115;
        var_tmf2_dn0 = assign15250_e21115_d_n0;
        var_tmf2_dn2 = assign15250_e21115_d_n2;
        var_tmf2_rv = 0.0;

        let (assign15260_e21131, assign15260_e21131_d_n0, assign15260_e21131_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard246 != 0.0)) {
        let assign15260_e21126: f64 = (var_tmf1 * var_tmf1);
        let assign15260_e21128: f64 = (assign15260_e21126 + var_tmf2);
        let assign15260_e21129: f64 = (assign15260_e21128).sqrt();
        (assign15260_e21129, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign15260_e21129)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign15260_e21129)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign15260_e21131;
        var_tmf2_dn0 = assign15260_e21131_d_n0;
        var_tmf2_dn2 = assign15260_e21131_d_n2;
        var_tmf2_rv = 0.0;

        let (assign15270_e21148, assign15270_e21148_d_n0, assign15270_e21148_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard246 != 0.0)) {
        let assign15270_e21144: f64 = (var_tmf1 / var_tmf2);
        let assign15270_e21145: f64 = (1.0 + assign15270_e21144);
        let assign15270_e21146: f64 = (0.5 * assign15270_e21145);
        (assign15270_e21146, (0.5 * (((var_tmf1_dn0 * var_tmf2) - (var_tmf1 * var_tmf2_dn0)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn2 * var_tmf2) - (var_tmf1 * var_tmf2_dn2)) / (var_tmf2 * var_tmf2))),)
    } else {
        (var_dfn_sl, var_dfn_sl_dn0, var_dfn_sl_dn2,)
    }
};
        var_dfn_sl = assign15270_e21148;
        var_dfn_sl_dn0 = assign15270_e21148_d_n0;
        var_dfn_sl_dn2 = assign15270_e21148_d_n2;
        var_dfn_sl_rv = 0.0;

        let (assign15280_e21165, assign15280_e21165_d_n0, assign15280_e21165_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard246 != 0.0)) {
        let assign15280_e21161: f64 = (var_tmf1 + var_tmf2);
        let assign15280_e21162: f64 = (0.5 * assign15280_e21161);
        let assign15280_e21163: f64 = (var_nfasti_i + assign15280_e21162);
        (assign15280_e21163, (0.5 * (var_tmf1_dn0 + var_tmf2_dn0)), (0.5 * (var_tmf1_dn2 + var_tmf2_dn2)),)
    } else {
        (var_nj1, var_nj1_dn0, var_nj1_dn2,)
    }
};
        var_nj1 = assign15280_e21165;
        var_nj1_dn0 = assign15280_e21165_d_n0;
        var_nj1_dn2 = assign15280_e21165_d_n2;
        var_nj1_rv = 0.0;

        let (assign15290_e21180, assign15290_e21180_d_n0, assign15290_e21180_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard246 != 0.0)) {
        let assign15290_e21176: f64 = (p.p85 - var_nj0);
        let assign15290_e21178: f64 = (assign15290_e21176 - 0.01);
        (assign15290_e21178, (-var_nj0_dn0), (-var_nj0_dn2),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2,)
    }
};
        var_tmf1 = assign15290_e21180;
        var_tmf1_dn0 = assign15290_e21180_d_n0;
        var_tmf1_dn2 = assign15290_e21180_d_n2;
        var_tmf1_rv = 0.0;

        let (assign15300_e21195, assign15300_e21195_d_n0, assign15300_e21195_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard246 != 0.0)) {
        let assign15300_e21191: f64 = (4.0 * p.p85);
        let assign15300_e21193: f64 = (assign15300_e21191 * 0.01);
        (assign15300_e21193, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign15300_e21195;
        var_tmf2_dn0 = assign15300_e21195_d_n0;
        var_tmf2_dn2 = assign15300_e21195_d_n2;
        var_tmf2_rv = 0.0;

        let (assign15310_e21212, assign15310_e21212_d_n0, assign15310_e21212_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard246 != 0.0)) {
        let (assign15310_e21210, assign15310_e21210_d_n0, assign15310_e21210_d_n2,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
            } else {
                let assign15310_e21209: f64 = (-var_tmf2);
                (assign15310_e21209, (-var_tmf2_dn0), (-var_tmf2_dn2),)
            }
        };
        (assign15310_e21210, assign15310_e21210_d_n0, assign15310_e21210_d_n2,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign15310_e21212;
        var_tmf2_dn0 = assign15310_e21212_d_n0;
        var_tmf2_dn2 = assign15310_e21212_d_n2;
        var_tmf2_rv = 0.0;

        let (assign15320_e21228, assign15320_e21228_d_n0, assign15320_e21228_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard246 != 0.0)) {
        let assign15320_e21223: f64 = (var_tmf1 * var_tmf1);
        let assign15320_e21225: f64 = (assign15320_e21223 + var_tmf2);
        let assign15320_e21226: f64 = (assign15320_e21225).sqrt();
        (assign15320_e21226, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign15320_e21226)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign15320_e21226)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign15320_e21228;
        var_tmf2_dn0 = assign15320_e21228_d_n0;
        var_tmf2_dn2 = assign15320_e21228_d_n2;
        var_tmf2_rv = 0.0;

        let (assign15330_e21245, assign15330_e21245_d_n0, assign15330_e21245_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard246 != 0.0)) {
        let assign15330_e21241: f64 = (var_tmf1 + var_tmf2);
        let assign15330_e21242: f64 = (0.5 * assign15330_e21241);
        let assign15330_e21243: f64 = (p.p85 - assign15330_e21242);
        (assign15330_e21243, (-(0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (-(0.5 * (var_tmf1_dn2 + var_tmf2_dn2))),)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn2,)
    }
};
        var_nj0 = assign15330_e21245;
        var_nj0_dn0 = assign15330_e21245_d_n0;
        var_nj0_dn2 = assign15330_e21245_d_n2;
        var_nj0_rv = 0.0;

        let (assign15340_e21260, assign15340_e21260_d_n0, assign15340_e21260_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard246 != 0.0)) {
        let assign15340_e21256: f64 = (var_nj0 - var_nfasti_i);
        let assign15340_e21258: f64 = (assign15340_e21256 - 0.01);
        (assign15340_e21258, var_nj0_dn0, var_nj0_dn2,)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2,)
    }
};
        var_tmf1 = assign15340_e21260;
        var_tmf1_dn0 = assign15340_e21260_d_n0;
        var_tmf1_dn2 = assign15340_e21260_d_n2;
        var_tmf1_rv = 0.0;

        let (assign15350_e21275, assign15350_e21275_d_n0, assign15350_e21275_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard246 != 0.0)) {
        let assign15350_e21271: f64 = (4.0 * var_nfasti_i);
        let assign15350_e21273: f64 = (assign15350_e21271 * 0.01);
        (assign15350_e21273, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign15350_e21275;
        var_tmf2_dn0 = assign15350_e21275_d_n0;
        var_tmf2_dn2 = assign15350_e21275_d_n2;
        var_tmf2_rv = 0.0;

        let (assign15360_e21292, assign15360_e21292_d_n0, assign15360_e21292_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard246 != 0.0)) {
        let (assign15360_e21290, assign15360_e21290_d_n0, assign15360_e21290_d_n2,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
            } else {
                let assign15360_e21289: f64 = (-var_tmf2);
                (assign15360_e21289, (-var_tmf2_dn0), (-var_tmf2_dn2),)
            }
        };
        (assign15360_e21290, assign15360_e21290_d_n0, assign15360_e21290_d_n2,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign15360_e21292;
        var_tmf2_dn0 = assign15360_e21292_d_n0;
        var_tmf2_dn2 = assign15360_e21292_d_n2;
        var_tmf2_rv = 0.0;

        let (assign15370_e21308, assign15370_e21308_d_n0, assign15370_e21308_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard246 != 0.0)) {
        let assign15370_e21303: f64 = (var_tmf1 * var_tmf1);
        let assign15370_e21305: f64 = (assign15370_e21303 + var_tmf2);
        let assign15370_e21306: f64 = (assign15370_e21305).sqrt();
        (assign15370_e21306, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign15370_e21306)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign15370_e21306)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign15370_e21308;
        var_tmf2_dn0 = assign15370_e21308_d_n0;
        var_tmf2_dn2 = assign15370_e21308_d_n2;
        var_tmf2_rv = 0.0;

        let (assign15380_e21325, assign15380_e21325_d_n0, assign15380_e21325_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard246 != 0.0)) {
        let assign15380_e21321: f64 = (var_tmf1 + var_tmf2);
        let assign15380_e21322: f64 = (0.5 * assign15380_e21321);
        let assign15380_e21323: f64 = (var_nfasti_i + assign15380_e21322);
        (assign15380_e21323, (0.5 * (var_tmf1_dn0 + var_tmf2_dn0)), (0.5 * (var_tmf1_dn2 + var_tmf2_dn2)),)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn2,)
    }
};
        var_nj0 = assign15380_e21325;
        var_nj0_dn0 = assign15380_e21325_d_n0;
        var_nj0_dn2 = assign15380_e21325_d_n2;
        var_nj0_rv = 0.0;

        let (assign15390_e21340, assign15390_e21340_d_n0, assign15390_e21340_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard246 != 0.0)) {
        let assign15390_e21336: f64 = (p.p86 * var_dfn_su);
        let assign15390_e21338: f64 = (assign15390_e21336 * var_dfn_sl);
        (assign15390_e21338, (((p.p86 * var_dfn_su_dn0) * var_dfn_sl) + (assign15390_e21336 * var_dfn_sl_dn0)), (((p.p86 * var_dfn_su_dn2) * var_dfn_sl) + (assign15390_e21336 * var_dfn_sl_dn2)),)
    } else {
        (var_dnj1_dv, var_dnj1_dv_dn0, var_dnj1_dv_dn2,)
    }
};
        var_dnj1_dv = assign15390_e21340;
        var_dnj1_dv_dn0 = assign15390_e21340_d_n0;
        var_dnj1_dv_dn2 = assign15390_e21340_d_n2;
        var_dnj1_dv_rv = 0.0;

        let (assign15400_e21352, assign15400_e21352_d_n0, assign15400_e21352_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard246 == 0.0)) {
        (var_nfasti_i, 0.0, 0.0,)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn2,)
    }
};
        var_nj0 = assign15400_e21352;
        var_nj0_dn0 = assign15400_e21352_d_n0;
        var_nj0_dn2 = assign15400_e21352_d_n2;
        var_nj0_rv = 0.0;

        let (assign15410_e21364, assign15410_e21364_d_n0, assign15410_e21364_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard246 == 0.0)) {
        (var_nfasti_i, 0.0, 0.0,)
    } else {
        (var_nj1, var_nj1_dn0, var_nj1_dn2,)
    }
};
        var_nj1 = assign15410_e21364;
        var_nj1_dn0 = assign15410_e21364_d_n0;
        var_nj1_dn2 = assign15410_e21364_d_n2;
        var_nj1_rv = 0.0;

        let (assign15420_e21376, assign15420_e21376_d_n0, assign15420_e21376_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard246 == 0.0)) {
        (0.0, 0.0, 0.0,)
    } else {
        (var_dnj1_dv, var_dnj1_dv_dn0, var_dnj1_dv_dn2,)
    }
};
        var_dnj1_dv = assign15420_e21376;
        var_dnj1_dv_dn0 = assign15420_e21376_d_n0;
        var_dnj1_dv_dn2 = assign15420_e21376_d_n2;
        var_dnj1_dv_rv = 0.0;

        let (assign15480_e21625, assign15480_e21625_d_n0, assign15480_e21625_d_n2,) = {
    if (((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) {
        let assign15480_e21609: f64 = (var_vmax * var_dnj1_dv);
        let assign15480_e21610: f64 = (var_nj1 - assign15480_e21609);
        let assign15480_e21613: f64 = (var_nj1 * var_nj1);
        let assign15480_e21614: f64 = (assign15480_e21610 / assign15480_e21613);
        let assign15480_e21617: f64 = (var_vha1 * var_dnj1_dv);
        let assign15480_e21620: f64 = (var_nj0 * p.p85);
        let assign15480_e21621: f64 = (assign15480_e21617 / assign15480_e21620);
        let assign15480_e21622: f64 = (assign15480_e21614 + assign15480_e21621);
        let assign15480_e21623: f64 = (var_phitdinv * assign15480_e21622);
        (assign15480_e21623, (var_phitdinv * (((((var_nj1_dn0 - (var_vmax * var_dnj1_dv_dn0)) * assign15480_e21613) - (assign15480_e21610 * ((var_nj1_dn0 * var_nj1) + (var_nj1 * var_nj1_dn0)))) / (assign15480_e21613 * assign15480_e21613)) + ((((var_vha1 * var_dnj1_dv_dn0) * assign15480_e21620) - (assign15480_e21617 * (var_nj0_dn0 * p.p85))) / (assign15480_e21620 * assign15480_e21620)))), (var_phitdinv * (((((var_nj1_dn2 - (var_vmax * var_dnj1_dv_dn2)) * assign15480_e21613) - (assign15480_e21610 * ((var_nj1_dn2 * var_nj1) + (var_nj1 * var_nj1_dn2)))) / (assign15480_e21613 * assign15480_e21613)) + ((((var_vha1 * var_dnj1_dv_dn2) * assign15480_e21620) - (assign15480_e21617 * (var_nj0_dn2 * p.p85))) / (assign15480_e21620 * assign15480_e21620)))),)
    } else {
        (var_dvmax_over_phitd_dv, var_dvmax_over_phitd_dv_dn0, var_dvmax_over_phitd_dv_dn2,)
    }
};
        var_dvmax_over_phitd_dv = assign15480_e21625;
        var_dvmax_over_phitd_dv_dn0 = assign15480_e21625_d_n0;
        var_dvmax_over_phitd_dv_dn2 = assign15480_e21625_d_n2;
        var_dvmax_over_phitd_dv_rv = 0.0;

        *var_dfn_sl_slot = var_dfn_sl;
        *var_dfn_sl_dn0_slot = var_dfn_sl_dn0;
        *var_dfn_sl_dn2_slot = var_dfn_sl_dn2;
        *var_dfn_sl_rv_slot = var_dfn_sl_rv;
        *var_dfn_su_slot = var_dfn_su;
        *var_dfn_su_dn0_slot = var_dfn_su_dn0;
        *var_dfn_su_dn2_slot = var_dfn_su_dn2;
        *var_dfn_su_rv_slot = var_dfn_su_rv;
        *var_dnj1_dv_slot = var_dnj1_dv;
        *var_dnj1_dv_dn0_slot = var_dnj1_dv_dn0;
        *var_dnj1_dv_dn2_slot = var_dnj1_dv_dn2;
        *var_dnj1_dv_rv_slot = var_dnj1_dv_rv;
        *var_dvmax_over_phitd_dv_slot = var_dvmax_over_phitd_dv;
        *var_dvmax_over_phitd_dv_dn0_slot = var_dvmax_over_phitd_dv_dn0;
        *var_dvmax_over_phitd_dv_dn2_slot = var_dvmax_over_phitd_dv_dn2;
        *var_dvmax_over_phitd_dv_rv_slot = var_dvmax_over_phitd_dv_rv;
        *var_guard246_slot = var_guard246;
        *var_guard246_rv_slot = var_guard246_rv;
        *var_idmultbot_slot = var_idmultbot;
        *var_idmultbot_dn0_slot = var_idmultbot_dn0;
        *var_idmultbot_dn2_slot = var_idmultbot_dn2;
        *var_idmultbot_rv_slot = var_idmultbot_rv;
        *var_nj0_slot = var_nj0;
        *var_nj0_dn0_slot = var_nj0_dn0;
        *var_nj0_dn2_slot = var_nj0_dn2;
        *var_nj0_rv_slot = var_nj0_rv;
        *var_nj1_slot = var_nj1;
        *var_nj1_dn0_slot = var_nj1_dn0;
        *var_nj1_dn2_slot = var_nj1_dn2;
        *var_nj1_rv_slot = var_nj1_rv;
        *var_nja10_slot = var_nja10;
        *var_nja10_dn0_slot = var_nja10_dn0;
        *var_nja10_dn2_slot = var_nja10_dn2;
        *var_nja10_rv_slot = var_nja10_rv;
        *var_nja11_slot = var_nja11;
        *var_nja11_dn0_slot = var_nja11_dn0;
        *var_nja11_dn2_slot = var_nja11_dn2;
        *var_nja11_rv_slot = var_nja11_rv;
        *var_pnn0_slot = var_pnn0;
        *var_pnn0_rv_slot = var_pnn0_rv;
        *var_tmf1_slot = var_tmf1;
        *var_tmf1_dn0_slot = var_tmf1_dn0;
        *var_tmf1_dn2_slot = var_tmf1_dn2;
        *var_tmf1_rv_slot = var_tmf1_rv;
        *var_tmf2_slot = var_tmf2;
        *var_tmf2_dn0_slot = var_tmf2_dn0;
        *var_tmf2_dn2_slot = var_tmf2_dn2;
        *var_tmf2_rv_slot = var_tmf2_rv;
        *var_vha1_slot = var_vha1;
        *var_vha1_rv_slot = var_vha1_rv;
    }

    pub(super) fn stamp_reactive_block_18(
        p: &Parameters,
        var_ab_i: f64,
        var_guard230: f64,
        var_guard231: f64,
        var_guard31: f64,
        var_lg_i: f64,
        var_ls_i: f64,
        var_ndigat_i: f64,
        var_nfagat_i: f64,
        var_nin: f64,
        var_phitdinv: f64,
        var_v4: f64,
        var_vmax: f64,
        var_dfn_sl_slot: &mut f64,
        var_dfn_sl_dn0_slot: &mut f64,
        var_dfn_sl_dn2_slot: &mut f64,
        var_dfn_sl_rv_slot: &mut f64,
        var_dfn_su_slot: &mut f64,
        var_dfn_su_dn0_slot: &mut f64,
        var_dfn_su_dn2_slot: &mut f64,
        var_dfn_su_rv_slot: &mut f64,
        var_dnj1_dv_slot: &mut f64,
        var_dnj1_dv_dn0_slot: &mut f64,
        var_dnj1_dv_dn2_slot: &mut f64,
        var_dnj1_dv_rv_slot: &mut f64,
        var_dvmax_over_phitd_dv_slot: &mut f64,
        var_dvmax_over_phitd_dv_dn0_slot: &mut f64,
        var_dvmax_over_phitd_dv_dn2_slot: &mut f64,
        var_dvmax_over_phitd_dv_rv_slot: &mut f64,
        var_guard249_slot: &mut f64,
        var_guard249_rv_slot: &mut f64,
        var_guard307_slot: &mut f64,
        var_guard307_rv_slot: &mut f64,
        var_guard308_slot: &mut f64,
        var_guard308_rv_slot: &mut f64,
        var_idmultbot_slot: &mut f64,
        var_idmultbot_dn0_slot: &mut f64,
        var_idmultbot_dn2_slot: &mut f64,
        var_idmultbot_rv_slot: &mut f64,
        var_nj0_slot: &mut f64,
        var_nj0_dn0_slot: &mut f64,
        var_nj0_dn2_slot: &mut f64,
        var_nj0_rv_slot: &mut f64,
        var_nj1_slot: &mut f64,
        var_nj1_dn0_slot: &mut f64,
        var_nj1_dn2_slot: &mut f64,
        var_nj1_rv_slot: &mut f64,
        var_nja10_slot: &mut f64,
        var_nja10_dn0_slot: &mut f64,
        var_nja10_dn2_slot: &mut f64,
        var_nja10_rv_slot: &mut f64,
        var_nja11_slot: &mut f64,
        var_nja11_dn0_slot: &mut f64,
        var_nja11_dn2_slot: &mut f64,
        var_nja11_rv_slot: &mut f64,
        var_pnn0_slot: &mut f64,
        var_pnn0_rv_slot: &mut f64,
        var_tmf1_slot: &mut f64,
        var_tmf1_dn0_slot: &mut f64,
        var_tmf1_dn2_slot: &mut f64,
        var_tmf1_rv_slot: &mut f64,
        var_tmf2_slot: &mut f64,
        var_tmf2_dn0_slot: &mut f64,
        var_tmf2_dn2_slot: &mut f64,
        var_tmf2_rv_slot: &mut f64,
        var_vha1_slot: &mut f64,
        var_vha1_rv_slot: &mut f64,
    ) {
        let mut var_dfn_sl: f64 = *var_dfn_sl_slot;
        let mut var_dfn_sl_dn0: f64 = *var_dfn_sl_dn0_slot;
        let mut var_dfn_sl_dn2: f64 = *var_dfn_sl_dn2_slot;
        let mut var_dfn_sl_rv: f64 = *var_dfn_sl_rv_slot;
        let mut var_dfn_su: f64 = *var_dfn_su_slot;
        let mut var_dfn_su_dn0: f64 = *var_dfn_su_dn0_slot;
        let mut var_dfn_su_dn2: f64 = *var_dfn_su_dn2_slot;
        let mut var_dfn_su_rv: f64 = *var_dfn_su_rv_slot;
        let mut var_dnj1_dv: f64 = *var_dnj1_dv_slot;
        let mut var_dnj1_dv_dn0: f64 = *var_dnj1_dv_dn0_slot;
        let mut var_dnj1_dv_dn2: f64 = *var_dnj1_dv_dn2_slot;
        let mut var_dnj1_dv_rv: f64 = *var_dnj1_dv_rv_slot;
        let mut var_dvmax_over_phitd_dv: f64 = *var_dvmax_over_phitd_dv_slot;
        let mut var_dvmax_over_phitd_dv_dn0: f64 = *var_dvmax_over_phitd_dv_dn0_slot;
        let mut var_dvmax_over_phitd_dv_dn2: f64 = *var_dvmax_over_phitd_dv_dn2_slot;
        let mut var_dvmax_over_phitd_dv_rv: f64 = *var_dvmax_over_phitd_dv_rv_slot;
        let mut var_guard249: f64 = *var_guard249_slot;
        let mut var_guard249_rv: f64 = *var_guard249_rv_slot;
        let mut var_guard307: f64 = *var_guard307_slot;
        let mut var_guard307_rv: f64 = *var_guard307_rv_slot;
        let mut var_guard308: f64 = *var_guard308_slot;
        let mut var_guard308_rv: f64 = *var_guard308_rv_slot;
        let mut var_idmultbot: f64 = *var_idmultbot_slot;
        let mut var_idmultbot_dn0: f64 = *var_idmultbot_dn0_slot;
        let mut var_idmultbot_dn2: f64 = *var_idmultbot_dn2_slot;
        let mut var_idmultbot_rv: f64 = *var_idmultbot_rv_slot;
        let mut var_nj0: f64 = *var_nj0_slot;
        let mut var_nj0_dn0: f64 = *var_nj0_dn0_slot;
        let mut var_nj0_dn2: f64 = *var_nj0_dn2_slot;
        let mut var_nj0_rv: f64 = *var_nj0_rv_slot;
        let mut var_nj1: f64 = *var_nj1_slot;
        let mut var_nj1_dn0: f64 = *var_nj1_dn0_slot;
        let mut var_nj1_dn2: f64 = *var_nj1_dn2_slot;
        let mut var_nj1_rv: f64 = *var_nj1_rv_slot;
        let mut var_nja10: f64 = *var_nja10_slot;
        let mut var_nja10_dn0: f64 = *var_nja10_dn0_slot;
        let mut var_nja10_dn2: f64 = *var_nja10_dn2_slot;
        let mut var_nja10_rv: f64 = *var_nja10_rv_slot;
        let mut var_nja11: f64 = *var_nja11_slot;
        let mut var_nja11_dn0: f64 = *var_nja11_dn0_slot;
        let mut var_nja11_dn2: f64 = *var_nja11_dn2_slot;
        let mut var_nja11_rv: f64 = *var_nja11_rv_slot;
        let mut var_pnn0: f64 = *var_pnn0_slot;
        let mut var_pnn0_rv: f64 = *var_pnn0_rv_slot;
        let mut var_tmf1: f64 = *var_tmf1_slot;
        let mut var_tmf1_dn0: f64 = *var_tmf1_dn0_slot;
        let mut var_tmf1_dn2: f64 = *var_tmf1_dn2_slot;
        let mut var_tmf1_rv: f64 = *var_tmf1_rv_slot;
        let mut var_tmf2: f64 = *var_tmf2_slot;
        let mut var_tmf2_dn0: f64 = *var_tmf2_dn0_slot;
        let mut var_tmf2_dn2: f64 = *var_tmf2_dn2_slot;
        let mut var_tmf2_rv: f64 = *var_tmf2_rv_slot;
        let mut var_vha1: f64 = *var_vha1_slot;
        let mut var_vha1_rv: f64 = *var_vha1_rv_slot;

        let (assign15500_e21655,) = {
    if (((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) {
        let assign15500_e21651: f64 = (var_nin * var_nin);
        let assign15500_e21653: f64 = (assign15500_e21651 / var_ndigat_i);
        (assign15500_e21653,)
    } else {
        (var_pnn0,)
    }
};
        var_pnn0 = assign15500_e21655;
        var_pnn0_rv = 0.0;

        let (assign15510_e21671,) = {
    if (((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) {
        let assign15510_e21664: f64 = (var_nfagat_i / var_phitdinv);
        let assign15510_e21667: f64 = (var_ndigat_i / var_pnn0);
        let assign15510_e21668: f64 = (assign15510_e21667).ln();
        let assign15510_e21669: f64 = (assign15510_e21664 * assign15510_e21668);
        (assign15510_e21669,)
    } else {
        (var_vha1,)
    }
};
        var_vha1 = assign15510_e21671;
        var_vha1_rv = 0.0;

        let assign15520_e21674: f64 = if var_nfagat_i < p.p85 { 1.0 } else { 0.0 };
        var_guard249 = assign15520_e21674;
        var_guard249_rv = 0.0;

        let (assign15530_e21691, assign15530_e21691_d_n0, assign15530_e21691_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard249 != 0.0)) {
        let assign15530_e21686: f64 = (var_vmax - var_vha1);
        let assign15530_e21687: f64 = (p.p86 * assign15530_e21686);
        let assign15530_e21689: f64 = (assign15530_e21687 + var_nfagat_i);
        (assign15530_e21689, 0.0, 0.0,)
    } else {
        (var_nja10, var_nja10_dn0, var_nja10_dn2,)
    }
};
        var_nja10 = assign15530_e21691;
        var_nja10_dn0 = assign15530_e21691_d_n0;
        var_nja10_dn2 = assign15530_e21691_d_n2;
        var_nja10_rv = 0.0;

        let (assign15540_e21706, assign15540_e21706_d_n0, assign15540_e21706_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard249 != 0.0)) {
        let assign15540_e21703: f64 = (p.p86 * var_vha1);
        let assign15540_e21704: f64 = (var_nfagat_i - assign15540_e21703);
        (assign15540_e21704, 0.0, 0.0,)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn2,)
    }
};
        var_nj0 = assign15540_e21706;
        var_nj0_dn0 = assign15540_e21706_d_n0;
        var_nj0_dn2 = assign15540_e21706_d_n2;
        var_nj0_rv = 0.0;

        let (assign15550_e21721, assign15550_e21721_d_n0, assign15550_e21721_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard249 != 0.0)) {
        let assign15550_e21717: f64 = (p.p85 - var_nja10);
        let assign15550_e21719: f64 = (assign15550_e21717 - 0.01);
        (assign15550_e21719, (-var_nja10_dn0), (-var_nja10_dn2),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2,)
    }
};
        var_tmf1 = assign15550_e21721;
        var_tmf1_dn0 = assign15550_e21721_d_n0;
        var_tmf1_dn2 = assign15550_e21721_d_n2;
        var_tmf1_rv = 0.0;

        let (assign15560_e21736, assign15560_e21736_d_n0, assign15560_e21736_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard249 != 0.0)) {
        let assign15560_e21732: f64 = (4.0 * p.p85);
        let assign15560_e21734: f64 = (assign15560_e21732 * 0.01);
        (assign15560_e21734, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign15560_e21736;
        var_tmf2_dn0 = assign15560_e21736_d_n0;
        var_tmf2_dn2 = assign15560_e21736_d_n2;
        var_tmf2_rv = 0.0;

        let (assign15570_e21753, assign15570_e21753_d_n0, assign15570_e21753_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard249 != 0.0)) {
        let (assign15570_e21751, assign15570_e21751_d_n0, assign15570_e21751_d_n2,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
            } else {
                let assign15570_e21750: f64 = (-var_tmf2);
                (assign15570_e21750, (-var_tmf2_dn0), (-var_tmf2_dn2),)
            }
        };
        (assign15570_e21751, assign15570_e21751_d_n0, assign15570_e21751_d_n2,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign15570_e21753;
        var_tmf2_dn0 = assign15570_e21753_d_n0;
        var_tmf2_dn2 = assign15570_e21753_d_n2;
        var_tmf2_rv = 0.0;

        let (assign15580_e21769, assign15580_e21769_d_n0, assign15580_e21769_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard249 != 0.0)) {
        let assign15580_e21764: f64 = (var_tmf1 * var_tmf1);
        let assign15580_e21766: f64 = (assign15580_e21764 + var_tmf2);
        let assign15580_e21767: f64 = (assign15580_e21766).sqrt();
        (assign15580_e21767, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign15580_e21767)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign15580_e21767)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign15580_e21769;
        var_tmf2_dn0 = assign15580_e21769_d_n0;
        var_tmf2_dn2 = assign15580_e21769_d_n2;
        var_tmf2_rv = 0.0;

        let (assign15590_e21786, assign15590_e21786_d_n0, assign15590_e21786_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard249 != 0.0)) {
        let assign15590_e21782: f64 = (var_tmf1 / var_tmf2);
        let assign15590_e21783: f64 = (1.0 + assign15590_e21782);
        let assign15590_e21784: f64 = (0.5 * assign15590_e21783);
        (assign15590_e21784, (0.5 * (((var_tmf1_dn0 * var_tmf2) - (var_tmf1 * var_tmf2_dn0)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn2 * var_tmf2) - (var_tmf1 * var_tmf2_dn2)) / (var_tmf2 * var_tmf2))),)
    } else {
        (var_dfn_su, var_dfn_su_dn0, var_dfn_su_dn2,)
    }
};
        var_dfn_su = assign15590_e21786;
        var_dfn_su_dn0 = assign15590_e21786_d_n0;
        var_dfn_su_dn2 = assign15590_e21786_d_n2;
        var_dfn_su_rv = 0.0;

        let (assign15600_e21803, assign15600_e21803_d_n0, assign15600_e21803_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard249 != 0.0)) {
        let assign15600_e21799: f64 = (var_tmf1 + var_tmf2);
        let assign15600_e21800: f64 = (0.5 * assign15600_e21799);
        let assign15600_e21801: f64 = (p.p85 - assign15600_e21800);
        (assign15600_e21801, (-(0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (-(0.5 * (var_tmf1_dn2 + var_tmf2_dn2))),)
    } else {
        (var_nja11, var_nja11_dn0, var_nja11_dn2,)
    }
};
        var_nja11 = assign15600_e21803;
        var_nja11_dn0 = assign15600_e21803_d_n0;
        var_nja11_dn2 = assign15600_e21803_d_n2;
        var_nja11_rv = 0.0;

        let (assign15610_e21818, assign15610_e21818_d_n0, assign15610_e21818_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard249 != 0.0)) {
        let assign15610_e21814: f64 = (var_nja11 - var_nfagat_i);
        let assign15610_e21816: f64 = (assign15610_e21814 - 0.01);
        (assign15610_e21816, var_nja11_dn0, var_nja11_dn2,)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2,)
    }
};
        var_tmf1 = assign15610_e21818;
        var_tmf1_dn0 = assign15610_e21818_d_n0;
        var_tmf1_dn2 = assign15610_e21818_d_n2;
        var_tmf1_rv = 0.0;

        let (assign15620_e21833, assign15620_e21833_d_n0, assign15620_e21833_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard249 != 0.0)) {
        let assign15620_e21829: f64 = (4.0 * var_nfagat_i);
        let assign15620_e21831: f64 = (assign15620_e21829 * 0.01);
        (assign15620_e21831, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign15620_e21833;
        var_tmf2_dn0 = assign15620_e21833_d_n0;
        var_tmf2_dn2 = assign15620_e21833_d_n2;
        var_tmf2_rv = 0.0;

        let (assign15630_e21850, assign15630_e21850_d_n0, assign15630_e21850_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard249 != 0.0)) {
        let (assign15630_e21848, assign15630_e21848_d_n0, assign15630_e21848_d_n2,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
            } else {
                let assign15630_e21847: f64 = (-var_tmf2);
                (assign15630_e21847, (-var_tmf2_dn0), (-var_tmf2_dn2),)
            }
        };
        (assign15630_e21848, assign15630_e21848_d_n0, assign15630_e21848_d_n2,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign15630_e21850;
        var_tmf2_dn0 = assign15630_e21850_d_n0;
        var_tmf2_dn2 = assign15630_e21850_d_n2;
        var_tmf2_rv = 0.0;

        let (assign15640_e21866, assign15640_e21866_d_n0, assign15640_e21866_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard249 != 0.0)) {
        let assign15640_e21861: f64 = (var_tmf1 * var_tmf1);
        let assign15640_e21863: f64 = (assign15640_e21861 + var_tmf2);
        let assign15640_e21864: f64 = (assign15640_e21863).sqrt();
        (assign15640_e21864, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign15640_e21864)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign15640_e21864)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign15640_e21866;
        var_tmf2_dn0 = assign15640_e21866_d_n0;
        var_tmf2_dn2 = assign15640_e21866_d_n2;
        var_tmf2_rv = 0.0;

        let (assign15650_e21883, assign15650_e21883_d_n0, assign15650_e21883_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard249 != 0.0)) {
        let assign15650_e21879: f64 = (var_tmf1 / var_tmf2);
        let assign15650_e21880: f64 = (1.0 + assign15650_e21879);
        let assign15650_e21881: f64 = (0.5 * assign15650_e21880);
        (assign15650_e21881, (0.5 * (((var_tmf1_dn0 * var_tmf2) - (var_tmf1 * var_tmf2_dn0)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn2 * var_tmf2) - (var_tmf1 * var_tmf2_dn2)) / (var_tmf2 * var_tmf2))),)
    } else {
        (var_dfn_sl, var_dfn_sl_dn0, var_dfn_sl_dn2,)
    }
};
        var_dfn_sl = assign15650_e21883;
        var_dfn_sl_dn0 = assign15650_e21883_d_n0;
        var_dfn_sl_dn2 = assign15650_e21883_d_n2;
        var_dfn_sl_rv = 0.0;

        let (assign15660_e21900, assign15660_e21900_d_n0, assign15660_e21900_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard249 != 0.0)) {
        let assign15660_e21896: f64 = (var_tmf1 + var_tmf2);
        let assign15660_e21897: f64 = (0.5 * assign15660_e21896);
        let assign15660_e21898: f64 = (var_nfagat_i + assign15660_e21897);
        (assign15660_e21898, (0.5 * (var_tmf1_dn0 + var_tmf2_dn0)), (0.5 * (var_tmf1_dn2 + var_tmf2_dn2)),)
    } else {
        (var_nj1, var_nj1_dn0, var_nj1_dn2,)
    }
};
        var_nj1 = assign15660_e21900;
        var_nj1_dn0 = assign15660_e21900_d_n0;
        var_nj1_dn2 = assign15660_e21900_d_n2;
        var_nj1_rv = 0.0;

        let (assign15670_e21915, assign15670_e21915_d_n0, assign15670_e21915_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard249 != 0.0)) {
        let assign15670_e21911: f64 = (p.p85 - var_nj0);
        let assign15670_e21913: f64 = (assign15670_e21911 - 0.01);
        (assign15670_e21913, (-var_nj0_dn0), (-var_nj0_dn2),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2,)
    }
};
        var_tmf1 = assign15670_e21915;
        var_tmf1_dn0 = assign15670_e21915_d_n0;
        var_tmf1_dn2 = assign15670_e21915_d_n2;
        var_tmf1_rv = 0.0;

        let (assign15680_e21930, assign15680_e21930_d_n0, assign15680_e21930_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard249 != 0.0)) {
        let assign15680_e21926: f64 = (4.0 * p.p85);
        let assign15680_e21928: f64 = (assign15680_e21926 * 0.01);
        (assign15680_e21928, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign15680_e21930;
        var_tmf2_dn0 = assign15680_e21930_d_n0;
        var_tmf2_dn2 = assign15680_e21930_d_n2;
        var_tmf2_rv = 0.0;

        let (assign15690_e21947, assign15690_e21947_d_n0, assign15690_e21947_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard249 != 0.0)) {
        let (assign15690_e21945, assign15690_e21945_d_n0, assign15690_e21945_d_n2,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
            } else {
                let assign15690_e21944: f64 = (-var_tmf2);
                (assign15690_e21944, (-var_tmf2_dn0), (-var_tmf2_dn2),)
            }
        };
        (assign15690_e21945, assign15690_e21945_d_n0, assign15690_e21945_d_n2,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign15690_e21947;
        var_tmf2_dn0 = assign15690_e21947_d_n0;
        var_tmf2_dn2 = assign15690_e21947_d_n2;
        var_tmf2_rv = 0.0;

        let (assign15700_e21963, assign15700_e21963_d_n0, assign15700_e21963_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard249 != 0.0)) {
        let assign15700_e21958: f64 = (var_tmf1 * var_tmf1);
        let assign15700_e21960: f64 = (assign15700_e21958 + var_tmf2);
        let assign15700_e21961: f64 = (assign15700_e21960).sqrt();
        (assign15700_e21961, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign15700_e21961)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign15700_e21961)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign15700_e21963;
        var_tmf2_dn0 = assign15700_e21963_d_n0;
        var_tmf2_dn2 = assign15700_e21963_d_n2;
        var_tmf2_rv = 0.0;

        let (assign15710_e21980, assign15710_e21980_d_n0, assign15710_e21980_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard249 != 0.0)) {
        let assign15710_e21976: f64 = (var_tmf1 + var_tmf2);
        let assign15710_e21977: f64 = (0.5 * assign15710_e21976);
        let assign15710_e21978: f64 = (p.p85 - assign15710_e21977);
        (assign15710_e21978, (-(0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (-(0.5 * (var_tmf1_dn2 + var_tmf2_dn2))),)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn2,)
    }
};
        var_nj0 = assign15710_e21980;
        var_nj0_dn0 = assign15710_e21980_d_n0;
        var_nj0_dn2 = assign15710_e21980_d_n2;
        var_nj0_rv = 0.0;

        let (assign15720_e21995, assign15720_e21995_d_n0, assign15720_e21995_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard249 != 0.0)) {
        let assign15720_e21991: f64 = (var_nj0 - var_nfagat_i);
        let assign15720_e21993: f64 = (assign15720_e21991 - 0.01);
        (assign15720_e21993, var_nj0_dn0, var_nj0_dn2,)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2,)
    }
};
        var_tmf1 = assign15720_e21995;
        var_tmf1_dn0 = assign15720_e21995_d_n0;
        var_tmf1_dn2 = assign15720_e21995_d_n2;
        var_tmf1_rv = 0.0;

        let (assign15730_e22010, assign15730_e22010_d_n0, assign15730_e22010_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard249 != 0.0)) {
        let assign15730_e22006: f64 = (4.0 * var_nfagat_i);
        let assign15730_e22008: f64 = (assign15730_e22006 * 0.01);
        (assign15730_e22008, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign15730_e22010;
        var_tmf2_dn0 = assign15730_e22010_d_n0;
        var_tmf2_dn2 = assign15730_e22010_d_n2;
        var_tmf2_rv = 0.0;

        let (assign15740_e22027, assign15740_e22027_d_n0, assign15740_e22027_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard249 != 0.0)) {
        let (assign15740_e22025, assign15740_e22025_d_n0, assign15740_e22025_d_n2,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
            } else {
                let assign15740_e22024: f64 = (-var_tmf2);
                (assign15740_e22024, (-var_tmf2_dn0), (-var_tmf2_dn2),)
            }
        };
        (assign15740_e22025, assign15740_e22025_d_n0, assign15740_e22025_d_n2,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign15740_e22027;
        var_tmf2_dn0 = assign15740_e22027_d_n0;
        var_tmf2_dn2 = assign15740_e22027_d_n2;
        var_tmf2_rv = 0.0;

        let (assign15750_e22043, assign15750_e22043_d_n0, assign15750_e22043_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard249 != 0.0)) {
        let assign15750_e22038: f64 = (var_tmf1 * var_tmf1);
        let assign15750_e22040: f64 = (assign15750_e22038 + var_tmf2);
        let assign15750_e22041: f64 = (assign15750_e22040).sqrt();
        (assign15750_e22041, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign15750_e22041)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign15750_e22041)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign15750_e22043;
        var_tmf2_dn0 = assign15750_e22043_d_n0;
        var_tmf2_dn2 = assign15750_e22043_d_n2;
        var_tmf2_rv = 0.0;

        let (assign15760_e22060, assign15760_e22060_d_n0, assign15760_e22060_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard249 != 0.0)) {
        let assign15760_e22056: f64 = (var_tmf1 + var_tmf2);
        let assign15760_e22057: f64 = (0.5 * assign15760_e22056);
        let assign15760_e22058: f64 = (var_nfagat_i + assign15760_e22057);
        (assign15760_e22058, (0.5 * (var_tmf1_dn0 + var_tmf2_dn0)), (0.5 * (var_tmf1_dn2 + var_tmf2_dn2)),)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn2,)
    }
};
        var_nj0 = assign15760_e22060;
        var_nj0_dn0 = assign15760_e22060_d_n0;
        var_nj0_dn2 = assign15760_e22060_d_n2;
        var_nj0_rv = 0.0;

        let (assign15770_e22075, assign15770_e22075_d_n0, assign15770_e22075_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard249 != 0.0)) {
        let assign15770_e22071: f64 = (p.p86 * var_dfn_su);
        let assign15770_e22073: f64 = (assign15770_e22071 * var_dfn_sl);
        (assign15770_e22073, (((p.p86 * var_dfn_su_dn0) * var_dfn_sl) + (assign15770_e22071 * var_dfn_sl_dn0)), (((p.p86 * var_dfn_su_dn2) * var_dfn_sl) + (assign15770_e22071 * var_dfn_sl_dn2)),)
    } else {
        (var_dnj1_dv, var_dnj1_dv_dn0, var_dnj1_dv_dn2,)
    }
};
        var_dnj1_dv = assign15770_e22075;
        var_dnj1_dv_dn0 = assign15770_e22075_d_n0;
        var_dnj1_dv_dn2 = assign15770_e22075_d_n2;
        var_dnj1_dv_rv = 0.0;

        let (assign15780_e22087, assign15780_e22087_d_n0, assign15780_e22087_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard249 == 0.0)) {
        (var_nfagat_i, 0.0, 0.0,)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn2,)
    }
};
        var_nj0 = assign15780_e22087;
        var_nj0_dn0 = assign15780_e22087_d_n0;
        var_nj0_dn2 = assign15780_e22087_d_n2;
        var_nj0_rv = 0.0;

        let (assign15790_e22099, assign15790_e22099_d_n0, assign15790_e22099_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard249 == 0.0)) {
        (var_nfagat_i, 0.0, 0.0,)
    } else {
        (var_nj1, var_nj1_dn0, var_nj1_dn2,)
    }
};
        var_nj1 = assign15790_e22099;
        var_nj1_dn0 = assign15790_e22099_d_n0;
        var_nj1_dn2 = assign15790_e22099_d_n2;
        var_nj1_rv = 0.0;

        let (assign15800_e22111, assign15800_e22111_d_n0, assign15800_e22111_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard249 == 0.0)) {
        (0.0, 0.0, 0.0,)
    } else {
        (var_dnj1_dv, var_dnj1_dv_dn0, var_dnj1_dv_dn2,)
    }
};
        var_dnj1_dv = assign15800_e22111;
        var_dnj1_dv_dn0 = assign15800_e22111_d_n0;
        var_dnj1_dv_dn2 = assign15800_e22111_d_n2;
        var_dnj1_dv_rv = 0.0;

        let (assign15860_e22360, assign15860_e22360_d_n0, assign15860_e22360_d_n2,) = {
    if (((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) {
        let assign15860_e22344: f64 = (var_vmax * var_dnj1_dv);
        let assign15860_e22345: f64 = (var_nj1 - assign15860_e22344);
        let assign15860_e22348: f64 = (var_nj1 * var_nj1);
        let assign15860_e22349: f64 = (assign15860_e22345 / assign15860_e22348);
        let assign15860_e22352: f64 = (var_vha1 * var_dnj1_dv);
        let assign15860_e22355: f64 = (var_nj0 * p.p85);
        let assign15860_e22356: f64 = (assign15860_e22352 / assign15860_e22355);
        let assign15860_e22357: f64 = (assign15860_e22349 + assign15860_e22356);
        let assign15860_e22358: f64 = (var_phitdinv * assign15860_e22357);
        (assign15860_e22358, (var_phitdinv * (((((var_nj1_dn0 - (var_vmax * var_dnj1_dv_dn0)) * assign15860_e22348) - (assign15860_e22345 * ((var_nj1_dn0 * var_nj1) + (var_nj1 * var_nj1_dn0)))) / (assign15860_e22348 * assign15860_e22348)) + ((((var_vha1 * var_dnj1_dv_dn0) * assign15860_e22355) - (assign15860_e22352 * (var_nj0_dn0 * p.p85))) / (assign15860_e22355 * assign15860_e22355)))), (var_phitdinv * (((((var_nj1_dn2 - (var_vmax * var_dnj1_dv_dn2)) * assign15860_e22348) - (assign15860_e22345 * ((var_nj1_dn2 * var_nj1) + (var_nj1 * var_nj1_dn2)))) / (assign15860_e22348 * assign15860_e22348)) + ((((var_vha1 * var_dnj1_dv_dn2) * assign15860_e22355) - (assign15860_e22352 * (var_nj0_dn2 * p.p85))) / (assign15860_e22355 * assign15860_e22355)))),)
    } else {
        (var_dvmax_over_phitd_dv, var_dvmax_over_phitd_dv_dn0, var_dvmax_over_phitd_dv_dn2,)
    }
};
        var_dvmax_over_phitd_dv = assign15860_e22360;
        var_dvmax_over_phitd_dv_dn0 = assign15860_e22360_d_n0;
        var_dvmax_over_phitd_dv_dn2 = assign15860_e22360_d_n2;
        var_dvmax_over_phitd_dv_rv = 0.0;

        let (assign15880_e22385, assign15880_e22385_d_n0, assign15880_e22385_d_n2,) = {
    if ((var_guard31 != 0.0) && (var_guard230 != 0.0)) {
        let assign15880_e22383: f64 = (var_idmultbot - 1.0);
        (assign15880_e22383, var_idmultbot_dn0, var_idmultbot_dn2,)
    } else {
        (var_idmultbot, var_idmultbot_dn0, var_idmultbot_dn2,)
    }
};
        var_idmultbot = assign15880_e22385;
        var_idmultbot_dn0 = assign15880_e22385_d_n0;
        var_idmultbot_dn2 = assign15880_e22385_d_n2;
        var_idmultbot_rv = 0.0;

        let (assign15990_e22558, assign15990_e22558_d_n0, assign15990_e22558_d_n2,) = {
    if ((var_guard31 != 0.0) && (var_guard230 == 0.0)) {
        (0.0, 0.0, 0.0,)
    } else {
        (var_idmultbot, var_idmultbot_dn0, var_idmultbot_dn2,)
    }
};
        var_idmultbot = assign15990_e22558;
        var_idmultbot_dn0 = assign15990_e22558_d_n0;
        var_idmultbot_dn2 = assign15990_e22558_d_n2;
        var_idmultbot_rv = 0.0;

        let assign18520_e26108: f64 = if (!(((var_ab_i == 0.0) && (var_ls_i == 0.0)) && (var_lg_i == 0.0))) { 1.0 } else { 0.0 };
        var_guard307 = assign18520_e26108;
        var_guard307_rv = 0.0;

        let assign18600_e26180: f64 = if var_v4 < var_vmax { 1.0 } else { 0.0 };
        var_guard308 = assign18600_e26180;
        var_guard308_rv = 0.0;

        *var_dfn_sl_slot = var_dfn_sl;
        *var_dfn_sl_dn0_slot = var_dfn_sl_dn0;
        *var_dfn_sl_dn2_slot = var_dfn_sl_dn2;
        *var_dfn_sl_rv_slot = var_dfn_sl_rv;
        *var_dfn_su_slot = var_dfn_su;
        *var_dfn_su_dn0_slot = var_dfn_su_dn0;
        *var_dfn_su_dn2_slot = var_dfn_su_dn2;
        *var_dfn_su_rv_slot = var_dfn_su_rv;
        *var_dnj1_dv_slot = var_dnj1_dv;
        *var_dnj1_dv_dn0_slot = var_dnj1_dv_dn0;
        *var_dnj1_dv_dn2_slot = var_dnj1_dv_dn2;
        *var_dnj1_dv_rv_slot = var_dnj1_dv_rv;
        *var_dvmax_over_phitd_dv_slot = var_dvmax_over_phitd_dv;
        *var_dvmax_over_phitd_dv_dn0_slot = var_dvmax_over_phitd_dv_dn0;
        *var_dvmax_over_phitd_dv_dn2_slot = var_dvmax_over_phitd_dv_dn2;
        *var_dvmax_over_phitd_dv_rv_slot = var_dvmax_over_phitd_dv_rv;
        *var_guard249_slot = var_guard249;
        *var_guard249_rv_slot = var_guard249_rv;
        *var_guard307_slot = var_guard307;
        *var_guard307_rv_slot = var_guard307_rv;
        *var_guard308_slot = var_guard308;
        *var_guard308_rv_slot = var_guard308_rv;
        *var_idmultbot_slot = var_idmultbot;
        *var_idmultbot_dn0_slot = var_idmultbot_dn0;
        *var_idmultbot_dn2_slot = var_idmultbot_dn2;
        *var_idmultbot_rv_slot = var_idmultbot_rv;
        *var_nj0_slot = var_nj0;
        *var_nj0_dn0_slot = var_nj0_dn0;
        *var_nj0_dn2_slot = var_nj0_dn2;
        *var_nj0_rv_slot = var_nj0_rv;
        *var_nj1_slot = var_nj1;
        *var_nj1_dn0_slot = var_nj1_dn0;
        *var_nj1_dn2_slot = var_nj1_dn2;
        *var_nj1_rv_slot = var_nj1_rv;
        *var_nja10_slot = var_nja10;
        *var_nja10_dn0_slot = var_nja10_dn0;
        *var_nja10_dn2_slot = var_nja10_dn2;
        *var_nja10_rv_slot = var_nja10_rv;
        *var_nja11_slot = var_nja11;
        *var_nja11_dn0_slot = var_nja11_dn0;
        *var_nja11_dn2_slot = var_nja11_dn2;
        *var_nja11_rv_slot = var_nja11_rv;
        *var_pnn0_slot = var_pnn0;
        *var_pnn0_rv_slot = var_pnn0_rv;
        *var_tmf1_slot = var_tmf1;
        *var_tmf1_dn0_slot = var_tmf1_dn0;
        *var_tmf1_dn2_slot = var_tmf1_dn2;
        *var_tmf1_rv_slot = var_tmf1_rv;
        *var_tmf2_slot = var_tmf2;
        *var_tmf2_dn0_slot = var_tmf2_dn0;
        *var_tmf2_dn2_slot = var_tmf2_dn2;
        *var_tmf2_rv_slot = var_tmf2_rv;
        *var_vha1_slot = var_vha1;
        *var_vha1_rv_slot = var_vha1_rv;
    }

    pub(super) fn stamp_reactive_block_19(
        p: &Parameters,
        var_guard307: f64,
        var_guard308: f64,
        var_guard31: f64,
        var_ndibot_i: f64,
        var_nfabot_i: f64,
        var_nin: f64,
        var_phitdinv: f64,
        var_v4: f64,
        var_guard311_slot: &mut f64,
        var_guard311_rv_slot: &mut f64,
        var_guard312_slot: &mut f64,
        var_guard312_rv_slot: &mut f64,
        var_guard313_slot: &mut f64,
        var_guard313_rv_slot: &mut f64,
        var_idmultbot_slot: &mut f64,
        var_idmultbot_dn0_slot: &mut f64,
        var_idmultbot_dn2_slot: &mut f64,
        var_idmultbot_rv_slot: &mut f64,
        var_nj0_slot: &mut f64,
        var_nj0_dn0_slot: &mut f64,
        var_nj0_dn2_slot: &mut f64,
        var_nj0_rv_slot: &mut f64,
        var_nj1_slot: &mut f64,
        var_nj1_dn0_slot: &mut f64,
        var_nj1_dn2_slot: &mut f64,
        var_nj1_rv_slot: &mut f64,
        var_nja10_slot: &mut f64,
        var_nja10_dn0_slot: &mut f64,
        var_nja10_dn2_slot: &mut f64,
        var_nja10_rv_slot: &mut f64,
        var_nja11_slot: &mut f64,
        var_nja11_dn0_slot: &mut f64,
        var_nja11_dn2_slot: &mut f64,
        var_nja11_rv_slot: &mut f64,
        var_pnn0_slot: &mut f64,
        var_pnn0_rv_slot: &mut f64,
        var_tmf1_slot: &mut f64,
        var_tmf1_dn0_slot: &mut f64,
        var_tmf1_dn2_slot: &mut f64,
        var_tmf1_rv_slot: &mut f64,
        var_tmf2_slot: &mut f64,
        var_tmf2_dn0_slot: &mut f64,
        var_tmf2_dn2_slot: &mut f64,
        var_tmf2_rv_slot: &mut f64,
        var_vha1_slot: &mut f64,
        var_vha1_rv_slot: &mut f64,
    ) {
        let mut var_guard311: f64 = *var_guard311_slot;
        let mut var_guard311_rv: f64 = *var_guard311_rv_slot;
        let mut var_guard312: f64 = *var_guard312_slot;
        let mut var_guard312_rv: f64 = *var_guard312_rv_slot;
        let mut var_guard313: f64 = *var_guard313_slot;
        let mut var_guard313_rv: f64 = *var_guard313_rv_slot;
        let mut var_idmultbot: f64 = *var_idmultbot_slot;
        let mut var_idmultbot_dn0: f64 = *var_idmultbot_dn0_slot;
        let mut var_idmultbot_dn2: f64 = *var_idmultbot_dn2_slot;
        let mut var_idmultbot_rv: f64 = *var_idmultbot_rv_slot;
        let mut var_nj0: f64 = *var_nj0_slot;
        let mut var_nj0_dn0: f64 = *var_nj0_dn0_slot;
        let mut var_nj0_dn2: f64 = *var_nj0_dn2_slot;
        let mut var_nj0_rv: f64 = *var_nj0_rv_slot;
        let mut var_nj1: f64 = *var_nj1_slot;
        let mut var_nj1_dn0: f64 = *var_nj1_dn0_slot;
        let mut var_nj1_dn2: f64 = *var_nj1_dn2_slot;
        let mut var_nj1_rv: f64 = *var_nj1_rv_slot;
        let mut var_nja10: f64 = *var_nja10_slot;
        let mut var_nja10_dn0: f64 = *var_nja10_dn0_slot;
        let mut var_nja10_dn2: f64 = *var_nja10_dn2_slot;
        let mut var_nja10_rv: f64 = *var_nja10_rv_slot;
        let mut var_nja11: f64 = *var_nja11_slot;
        let mut var_nja11_dn0: f64 = *var_nja11_dn0_slot;
        let mut var_nja11_dn2: f64 = *var_nja11_dn2_slot;
        let mut var_nja11_rv: f64 = *var_nja11_rv_slot;
        let mut var_pnn0: f64 = *var_pnn0_slot;
        let mut var_pnn0_rv: f64 = *var_pnn0_rv_slot;
        let mut var_tmf1: f64 = *var_tmf1_slot;
        let mut var_tmf1_dn0: f64 = *var_tmf1_dn0_slot;
        let mut var_tmf1_dn2: f64 = *var_tmf1_dn2_slot;
        let mut var_tmf1_rv: f64 = *var_tmf1_rv_slot;
        let mut var_tmf2: f64 = *var_tmf2_slot;
        let mut var_tmf2_dn0: f64 = *var_tmf2_dn0_slot;
        let mut var_tmf2_dn2: f64 = *var_tmf2_dn2_slot;
        let mut var_tmf2_rv: f64 = *var_tmf2_rv_slot;
        let mut var_vha1: f64 = *var_vha1_slot;
        let mut var_vha1_rv: f64 = *var_vha1_rv_slot;

        let (assign18660_e26321,) = {
    if (((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 != 0.0)) {
        let assign18660_e26317: f64 = (var_nin * var_nin);
        let assign18660_e26319: f64 = (assign18660_e26317 / var_ndibot_i);
        (assign18660_e26319,)
    } else {
        (var_pnn0,)
    }
};
        var_pnn0 = assign18660_e26321;
        var_pnn0_rv = 0.0;

        let (assign18670_e26336,) = {
    if (((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 != 0.0)) {
        let assign18670_e26329: f64 = (var_nfabot_i / var_phitdinv);
        let assign18670_e26332: f64 = (var_ndibot_i / var_pnn0);
        let assign18670_e26333: f64 = (assign18670_e26332).ln();
        let assign18670_e26334: f64 = (assign18670_e26329 * assign18670_e26333);
        (assign18670_e26334,)
    } else {
        (var_vha1,)
    }
};
        var_vha1 = assign18670_e26336;
        var_vha1_rv = 0.0;

        let assign18680_e26339: f64 = if var_nfabot_i < p.p85 { 1.0 } else { 0.0 };
        var_guard311 = assign18680_e26339;
        var_guard311_rv = 0.0;

        let (assign18690_e26355, assign18690_e26355_d_n0, assign18690_e26355_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 != 0.0)) && (var_guard311 != 0.0)) {
        let assign18690_e26350: f64 = (var_v4 - var_vha1);
        let assign18690_e26351: f64 = (p.p86 * assign18690_e26350);
        let assign18690_e26353: f64 = (assign18690_e26351 + var_nfabot_i);
        (assign18690_e26353, 0.0, 0.0,)
    } else {
        (var_nja10, var_nja10_dn0, var_nja10_dn2,)
    }
};
        var_nja10 = assign18690_e26355;
        var_nja10_dn0 = assign18690_e26355_d_n0;
        var_nja10_dn2 = assign18690_e26355_d_n2;
        var_nja10_rv = 0.0;

        let (assign18700_e26369, assign18700_e26369_d_n0, assign18700_e26369_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 != 0.0)) && (var_guard311 != 0.0)) {
        let assign18700_e26366: f64 = (p.p86 * var_vha1);
        let assign18700_e26367: f64 = (var_nfabot_i - assign18700_e26366);
        (assign18700_e26367, 0.0, 0.0,)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn2,)
    }
};
        var_nj0 = assign18700_e26369;
        var_nj0_dn0 = assign18700_e26369_d_n0;
        var_nj0_dn2 = assign18700_e26369_d_n2;
        var_nj0_rv = 0.0;

        let (assign18710_e26383, assign18710_e26383_d_n0, assign18710_e26383_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 != 0.0)) && (var_guard311 != 0.0)) {
        let assign18710_e26379: f64 = (p.p85 - var_nja10);
        let assign18710_e26381: f64 = (assign18710_e26379 - 0.01);
        (assign18710_e26381, (-var_nja10_dn0), (-var_nja10_dn2),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2,)
    }
};
        var_tmf1 = assign18710_e26383;
        var_tmf1_dn0 = assign18710_e26383_d_n0;
        var_tmf1_dn2 = assign18710_e26383_d_n2;
        var_tmf1_rv = 0.0;

        let (assign18720_e26397, assign18720_e26397_d_n0, assign18720_e26397_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 != 0.0)) && (var_guard311 != 0.0)) {
        let assign18720_e26393: f64 = (4.0 * p.p85);
        let assign18720_e26395: f64 = (assign18720_e26393 * 0.01);
        (assign18720_e26395, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign18720_e26397;
        var_tmf2_dn0 = assign18720_e26397_d_n0;
        var_tmf2_dn2 = assign18720_e26397_d_n2;
        var_tmf2_rv = 0.0;

        let (assign18730_e26413, assign18730_e26413_d_n0, assign18730_e26413_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 != 0.0)) && (var_guard311 != 0.0)) {
        let (assign18730_e26411, assign18730_e26411_d_n0, assign18730_e26411_d_n2,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
            } else {
                let assign18730_e26410: f64 = (-var_tmf2);
                (assign18730_e26410, (-var_tmf2_dn0), (-var_tmf2_dn2),)
            }
        };
        (assign18730_e26411, assign18730_e26411_d_n0, assign18730_e26411_d_n2,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign18730_e26413;
        var_tmf2_dn0 = assign18730_e26413_d_n0;
        var_tmf2_dn2 = assign18730_e26413_d_n2;
        var_tmf2_rv = 0.0;

        let (assign18740_e26428, assign18740_e26428_d_n0, assign18740_e26428_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 != 0.0)) && (var_guard311 != 0.0)) {
        let assign18740_e26423: f64 = (var_tmf1 * var_tmf1);
        let assign18740_e26425: f64 = (assign18740_e26423 + var_tmf2);
        let assign18740_e26426: f64 = (assign18740_e26425).sqrt();
        (assign18740_e26426, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign18740_e26426)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign18740_e26426)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign18740_e26428;
        var_tmf2_dn0 = assign18740_e26428_d_n0;
        var_tmf2_dn2 = assign18740_e26428_d_n2;
        var_tmf2_rv = 0.0;

        let (assign18750_e26444, assign18750_e26444_d_n0, assign18750_e26444_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 != 0.0)) && (var_guard311 != 0.0)) {
        let assign18750_e26440: f64 = (var_tmf1 + var_tmf2);
        let assign18750_e26441: f64 = (0.5 * assign18750_e26440);
        let assign18750_e26442: f64 = (p.p85 - assign18750_e26441);
        (assign18750_e26442, (-(0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (-(0.5 * (var_tmf1_dn2 + var_tmf2_dn2))),)
    } else {
        (var_nja11, var_nja11_dn0, var_nja11_dn2,)
    }
};
        var_nja11 = assign18750_e26444;
        var_nja11_dn0 = assign18750_e26444_d_n0;
        var_nja11_dn2 = assign18750_e26444_d_n2;
        var_nja11_rv = 0.0;

        let (assign18760_e26458, assign18760_e26458_d_n0, assign18760_e26458_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 != 0.0)) && (var_guard311 != 0.0)) {
        let assign18760_e26454: f64 = (var_nja11 - var_nfabot_i);
        let assign18760_e26456: f64 = (assign18760_e26454 - 0.01);
        (assign18760_e26456, var_nja11_dn0, var_nja11_dn2,)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2,)
    }
};
        var_tmf1 = assign18760_e26458;
        var_tmf1_dn0 = assign18760_e26458_d_n0;
        var_tmf1_dn2 = assign18760_e26458_d_n2;
        var_tmf1_rv = 0.0;

        let (assign18770_e26472, assign18770_e26472_d_n0, assign18770_e26472_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 != 0.0)) && (var_guard311 != 0.0)) {
        let assign18770_e26468: f64 = (4.0 * var_nfabot_i);
        let assign18770_e26470: f64 = (assign18770_e26468 * 0.01);
        (assign18770_e26470, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign18770_e26472;
        var_tmf2_dn0 = assign18770_e26472_d_n0;
        var_tmf2_dn2 = assign18770_e26472_d_n2;
        var_tmf2_rv = 0.0;

        let (assign18780_e26488, assign18780_e26488_d_n0, assign18780_e26488_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 != 0.0)) && (var_guard311 != 0.0)) {
        let (assign18780_e26486, assign18780_e26486_d_n0, assign18780_e26486_d_n2,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
            } else {
                let assign18780_e26485: f64 = (-var_tmf2);
                (assign18780_e26485, (-var_tmf2_dn0), (-var_tmf2_dn2),)
            }
        };
        (assign18780_e26486, assign18780_e26486_d_n0, assign18780_e26486_d_n2,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign18780_e26488;
        var_tmf2_dn0 = assign18780_e26488_d_n0;
        var_tmf2_dn2 = assign18780_e26488_d_n2;
        var_tmf2_rv = 0.0;

        let (assign18790_e26503, assign18790_e26503_d_n0, assign18790_e26503_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 != 0.0)) && (var_guard311 != 0.0)) {
        let assign18790_e26498: f64 = (var_tmf1 * var_tmf1);
        let assign18790_e26500: f64 = (assign18790_e26498 + var_tmf2);
        let assign18790_e26501: f64 = (assign18790_e26500).sqrt();
        (assign18790_e26501, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign18790_e26501)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign18790_e26501)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign18790_e26503;
        var_tmf2_dn0 = assign18790_e26503_d_n0;
        var_tmf2_dn2 = assign18790_e26503_d_n2;
        var_tmf2_rv = 0.0;

        let (assign18800_e26519, assign18800_e26519_d_n0, assign18800_e26519_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 != 0.0)) && (var_guard311 != 0.0)) {
        let assign18800_e26515: f64 = (var_tmf1 + var_tmf2);
        let assign18800_e26516: f64 = (0.5 * assign18800_e26515);
        let assign18800_e26517: f64 = (var_nfabot_i + assign18800_e26516);
        (assign18800_e26517, (0.5 * (var_tmf1_dn0 + var_tmf2_dn0)), (0.5 * (var_tmf1_dn2 + var_tmf2_dn2)),)
    } else {
        (var_nj1, var_nj1_dn0, var_nj1_dn2,)
    }
};
        var_nj1 = assign18800_e26519;
        var_nj1_dn0 = assign18800_e26519_d_n0;
        var_nj1_dn2 = assign18800_e26519_d_n2;
        var_nj1_rv = 0.0;

        let (assign18810_e26533, assign18810_e26533_d_n0, assign18810_e26533_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 != 0.0)) && (var_guard311 != 0.0)) {
        let assign18810_e26529: f64 = (p.p85 - var_nj0);
        let assign18810_e26531: f64 = (assign18810_e26529 - 0.01);
        (assign18810_e26531, (-var_nj0_dn0), (-var_nj0_dn2),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2,)
    }
};
        var_tmf1 = assign18810_e26533;
        var_tmf1_dn0 = assign18810_e26533_d_n0;
        var_tmf1_dn2 = assign18810_e26533_d_n2;
        var_tmf1_rv = 0.0;

        let (assign18820_e26547, assign18820_e26547_d_n0, assign18820_e26547_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 != 0.0)) && (var_guard311 != 0.0)) {
        let assign18820_e26543: f64 = (4.0 * p.p85);
        let assign18820_e26545: f64 = (assign18820_e26543 * 0.01);
        (assign18820_e26545, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign18820_e26547;
        var_tmf2_dn0 = assign18820_e26547_d_n0;
        var_tmf2_dn2 = assign18820_e26547_d_n2;
        var_tmf2_rv = 0.0;

        let (assign18830_e26563, assign18830_e26563_d_n0, assign18830_e26563_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 != 0.0)) && (var_guard311 != 0.0)) {
        let (assign18830_e26561, assign18830_e26561_d_n0, assign18830_e26561_d_n2,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
            } else {
                let assign18830_e26560: f64 = (-var_tmf2);
                (assign18830_e26560, (-var_tmf2_dn0), (-var_tmf2_dn2),)
            }
        };
        (assign18830_e26561, assign18830_e26561_d_n0, assign18830_e26561_d_n2,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign18830_e26563;
        var_tmf2_dn0 = assign18830_e26563_d_n0;
        var_tmf2_dn2 = assign18830_e26563_d_n2;
        var_tmf2_rv = 0.0;

        let (assign18840_e26578, assign18840_e26578_d_n0, assign18840_e26578_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 != 0.0)) && (var_guard311 != 0.0)) {
        let assign18840_e26573: f64 = (var_tmf1 * var_tmf1);
        let assign18840_e26575: f64 = (assign18840_e26573 + var_tmf2);
        let assign18840_e26576: f64 = (assign18840_e26575).sqrt();
        (assign18840_e26576, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign18840_e26576)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign18840_e26576)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign18840_e26578;
        var_tmf2_dn0 = assign18840_e26578_d_n0;
        var_tmf2_dn2 = assign18840_e26578_d_n2;
        var_tmf2_rv = 0.0;

        let (assign18850_e26594, assign18850_e26594_d_n0, assign18850_e26594_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 != 0.0)) && (var_guard311 != 0.0)) {
        let assign18850_e26590: f64 = (var_tmf1 + var_tmf2);
        let assign18850_e26591: f64 = (0.5 * assign18850_e26590);
        let assign18850_e26592: f64 = (p.p85 - assign18850_e26591);
        (assign18850_e26592, (-(0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (-(0.5 * (var_tmf1_dn2 + var_tmf2_dn2))),)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn2,)
    }
};
        var_nj0 = assign18850_e26594;
        var_nj0_dn0 = assign18850_e26594_d_n0;
        var_nj0_dn2 = assign18850_e26594_d_n2;
        var_nj0_rv = 0.0;

        let (assign18860_e26608, assign18860_e26608_d_n0, assign18860_e26608_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 != 0.0)) && (var_guard311 != 0.0)) {
        let assign18860_e26604: f64 = (var_nj0 - var_nfabot_i);
        let assign18860_e26606: f64 = (assign18860_e26604 - 0.01);
        (assign18860_e26606, var_nj0_dn0, var_nj0_dn2,)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2,)
    }
};
        var_tmf1 = assign18860_e26608;
        var_tmf1_dn0 = assign18860_e26608_d_n0;
        var_tmf1_dn2 = assign18860_e26608_d_n2;
        var_tmf1_rv = 0.0;

        let (assign18870_e26622, assign18870_e26622_d_n0, assign18870_e26622_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 != 0.0)) && (var_guard311 != 0.0)) {
        let assign18870_e26618: f64 = (4.0 * var_nfabot_i);
        let assign18870_e26620: f64 = (assign18870_e26618 * 0.01);
        (assign18870_e26620, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign18870_e26622;
        var_tmf2_dn0 = assign18870_e26622_d_n0;
        var_tmf2_dn2 = assign18870_e26622_d_n2;
        var_tmf2_rv = 0.0;

        let (assign18880_e26638, assign18880_e26638_d_n0, assign18880_e26638_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 != 0.0)) && (var_guard311 != 0.0)) {
        let (assign18880_e26636, assign18880_e26636_d_n0, assign18880_e26636_d_n2,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
            } else {
                let assign18880_e26635: f64 = (-var_tmf2);
                (assign18880_e26635, (-var_tmf2_dn0), (-var_tmf2_dn2),)
            }
        };
        (assign18880_e26636, assign18880_e26636_d_n0, assign18880_e26636_d_n2,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign18880_e26638;
        var_tmf2_dn0 = assign18880_e26638_d_n0;
        var_tmf2_dn2 = assign18880_e26638_d_n2;
        var_tmf2_rv = 0.0;

        let (assign18890_e26653, assign18890_e26653_d_n0, assign18890_e26653_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 != 0.0)) && (var_guard311 != 0.0)) {
        let assign18890_e26648: f64 = (var_tmf1 * var_tmf1);
        let assign18890_e26650: f64 = (assign18890_e26648 + var_tmf2);
        let assign18890_e26651: f64 = (assign18890_e26650).sqrt();
        (assign18890_e26651, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign18890_e26651)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign18890_e26651)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign18890_e26653;
        var_tmf2_dn0 = assign18890_e26653_d_n0;
        var_tmf2_dn2 = assign18890_e26653_d_n2;
        var_tmf2_rv = 0.0;

        let (assign18900_e26669, assign18900_e26669_d_n0, assign18900_e26669_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 != 0.0)) && (var_guard311 != 0.0)) {
        let assign18900_e26665: f64 = (var_tmf1 + var_tmf2);
        let assign18900_e26666: f64 = (0.5 * assign18900_e26665);
        let assign18900_e26667: f64 = (var_nfabot_i + assign18900_e26666);
        (assign18900_e26667, (0.5 * (var_tmf1_dn0 + var_tmf2_dn0)), (0.5 * (var_tmf1_dn2 + var_tmf2_dn2)),)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn2,)
    }
};
        var_nj0 = assign18900_e26669;
        var_nj0_dn0 = assign18900_e26669_d_n0;
        var_nj0_dn2 = assign18900_e26669_d_n2;
        var_nj0_rv = 0.0;

        let (assign18910_e26680, assign18910_e26680_d_n0, assign18910_e26680_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 != 0.0)) && (var_guard311 == 0.0)) {
        (var_nfabot_i, 0.0, 0.0,)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn2,)
    }
};
        var_nj0 = assign18910_e26680;
        var_nj0_dn0 = assign18910_e26680_d_n0;
        var_nj0_dn2 = assign18910_e26680_d_n2;
        var_nj0_rv = 0.0;

        let (assign18920_e26691, assign18920_e26691_d_n0, assign18920_e26691_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 != 0.0)) && (var_guard311 == 0.0)) {
        (var_nfabot_i, 0.0, 0.0,)
    } else {
        (var_nj1, var_nj1_dn0, var_nj1_dn2,)
    }
};
        var_nj1 = assign18920_e26691;
        var_nj1_dn0 = assign18920_e26691_d_n0;
        var_nj1_dn2 = assign18920_e26691_d_n2;
        var_nj1_rv = 0.0;

        let assign18930_e26695: f64 = (var_v4 / var_nj1);
        let assign18930_e26699: f64 = (var_nj1 - var_nj0);
        let assign18930_e26700: f64 = (var_vha1 * assign18930_e26699);
        let assign18930_e26703: f64 = (var_nj0 * p.p85);
        let assign18930_e26704: f64 = (assign18930_e26700 / assign18930_e26703);
        let assign18930_e26705: f64 = (assign18930_e26695 + assign18930_e26704);
        let assign18930_e26706: f64 = (var_phitdinv * assign18930_e26705);
        let assign18930_e26707: f64 = (assign18930_e26706).abs();
        let assign18930_e26709: f64 = if assign18930_e26707 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard312 = assign18930_e26709;
        var_guard312_rv = 0.0;

        let (assign18940_e26734, assign18940_e26734_d_n0, assign18940_e26734_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 != 0.0)) && (var_guard312 != 0.0)) {
        let assign18940_e26720: f64 = (var_v4 / var_nj1);
        let assign18940_e26724: f64 = (var_nj1 - var_nj0);
        let assign18940_e26725: f64 = (var_vha1 * assign18940_e26724);
        let assign18940_e26728: f64 = (var_nj0 * p.p85);
        let assign18940_e26729: f64 = (assign18940_e26725 / assign18940_e26728);
        let assign18940_e26730: f64 = (assign18940_e26720 + assign18940_e26729);
        let assign18940_e26731: f64 = (var_phitdinv * assign18940_e26730);
        let assign18940_e26732: f64 = (assign18940_e26731).exp();
        (assign18940_e26732, (assign18940_e26732 * (var_phitdinv * ((-((var_v4 * var_nj1_dn0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn0 - var_nj0_dn0)) * assign18940_e26728) - (assign18940_e26725 * (var_nj0_dn0 * p.p85))) / (assign18940_e26728 * assign18940_e26728))))), (assign18940_e26732 * (var_phitdinv * ((-((var_v4 * var_nj1_dn2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn2 - var_nj0_dn2)) * assign18940_e26728) - (assign18940_e26725 * (var_nj0_dn2 * p.p85))) / (assign18940_e26728 * assign18940_e26728))))),)
    } else {
        (var_idmultbot, var_idmultbot_dn0, var_idmultbot_dn2,)
    }
};
        var_idmultbot = assign18940_e26734;
        var_idmultbot_dn0 = assign18940_e26734_d_n0;
        var_idmultbot_dn2 = assign18940_e26734_d_n2;
        var_idmultbot_rv = 0.0;

        let assign18950_e26738: f64 = (var_v4 / var_nj1);
        let assign18950_e26742: f64 = (var_nj1 - var_nj0);
        let assign18950_e26743: f64 = (var_vha1 * assign18950_e26742);
        let assign18950_e26746: f64 = (var_nj0 * p.p85);
        let assign18950_e26747: f64 = (assign18950_e26743 / assign18950_e26746);
        let assign18950_e26748: f64 = (assign18950_e26738 + assign18950_e26747);
        let assign18950_e26749: f64 = (var_phitdinv * assign18950_e26748);
        let assign18950_e26751: f64 = (-230.25850929940458);
        let assign18950_e26752: f64 = if assign18950_e26749 < assign18950_e26751 { 1.0 } else { 0.0 };
        var_guard313 = assign18950_e26752;
        var_guard313_rv = 0.0;

        let (assign18960_e26832, assign18960_e26832_d_n0, assign18960_e26832_d_n2,) = {
    if (((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 != 0.0)) && (var_guard312 == 0.0)) && (var_guard313 != 0.0)) {
        let assign18960_e26766: f64 = (-230.25850929940458);
        let assign18960_e26770: f64 = (var_v4 / var_nj1);
        let assign18960_e26774: f64 = (var_nj1 - var_nj0);
        let assign18960_e26775: f64 = (var_vha1 * assign18960_e26774);
        let assign18960_e26778: f64 = (var_nj0 * p.p85);
        let assign18960_e26779: f64 = (assign18960_e26775 / assign18960_e26778);
        let assign18960_e26780: f64 = (assign18960_e26770 + assign18960_e26779);
        let assign18960_e26781: f64 = (var_phitdinv * assign18960_e26780);
        let assign18960_e26782: f64 = (assign18960_e26766 - assign18960_e26781);
        let assign18960_e26786: f64 = (-230.25850929940458);
        let assign18960_e26790: f64 = (var_v4 / var_nj1);
        let assign18960_e26794: f64 = (var_nj1 - var_nj0);
        let assign18960_e26795: f64 = (var_vha1 * assign18960_e26794);
        let assign18960_e26798: f64 = (var_nj0 * p.p85);
        let assign18960_e26799: f64 = (assign18960_e26795 / assign18960_e26798);
        let assign18960_e26800: f64 = (assign18960_e26790 + assign18960_e26799);
        let assign18960_e26801: f64 = (var_phitdinv * assign18960_e26800);
        let assign18960_e26802: f64 = (assign18960_e26786 - assign18960_e26801);
        let assign18960_e26805: f64 = (-230.25850929940458);
        let assign18960_e26809: f64 = (var_v4 / var_nj1);
        let assign18960_e26813: f64 = (var_nj1 - var_nj0);
        let assign18960_e26814: f64 = (var_vha1 * assign18960_e26813);
        let assign18960_e26817: f64 = (var_nj0 * p.p85);
        let assign18960_e26818: f64 = (assign18960_e26814 / assign18960_e26817);
        let assign18960_e26819: f64 = (assign18960_e26809 + assign18960_e26818);
        let assign18960_e26820: f64 = (var_phitdinv * assign18960_e26819);
        let assign18960_e26821: f64 = (assign18960_e26805 - assign18960_e26820);
        let assign18960_e26823: f64 = (assign18960_e26821 * 0.3333333333333333);
        let assign18960_e26824: f64 = (1.0 + assign18960_e26823);
        let assign18960_e26825: f64 = (assign18960_e26802 * assign18960_e26824);
        let assign18960_e26826: f64 = (0.5 * assign18960_e26825);
        let assign18960_e26827: f64 = (1.0 + assign18960_e26826);
        let assign18960_e26828: f64 = (assign18960_e26782 * assign18960_e26827);
        let assign18960_e26829: f64 = (1.0 + assign18960_e26828);
        let assign18960_e26830: f64 = (1e-100 / assign18960_e26829);
        (assign18960_e26830, (-((1e-100 * (((-(var_phitdinv * ((-((var_v4 * var_nj1_dn0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn0 - var_nj0_dn0)) * assign18960_e26778) - (assign18960_e26775 * (var_nj0_dn0 * p.p85))) / (assign18960_e26778 * assign18960_e26778))))) * assign18960_e26827) + (assign18960_e26782 * (0.5 * (((-(var_phitdinv * ((-((var_v4 * var_nj1_dn0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn0 - var_nj0_dn0)) * assign18960_e26798) - (assign18960_e26795 * (var_nj0_dn0 * p.p85))) / (assign18960_e26798 * assign18960_e26798))))) * assign18960_e26824) + (assign18960_e26802 * ((-(var_phitdinv * ((-((var_v4 * var_nj1_dn0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn0 - var_nj0_dn0)) * assign18960_e26817) - (assign18960_e26814 * (var_nj0_dn0 * p.p85))) / (assign18960_e26817 * assign18960_e26817))))) * 0.3333333333333333))))))) / (assign18960_e26829 * assign18960_e26829))), (-((1e-100 * (((-(var_phitdinv * ((-((var_v4 * var_nj1_dn2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn2 - var_nj0_dn2)) * assign18960_e26778) - (assign18960_e26775 * (var_nj0_dn2 * p.p85))) / (assign18960_e26778 * assign18960_e26778))))) * assign18960_e26827) + (assign18960_e26782 * (0.5 * (((-(var_phitdinv * ((-((var_v4 * var_nj1_dn2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn2 - var_nj0_dn2)) * assign18960_e26798) - (assign18960_e26795 * (var_nj0_dn2 * p.p85))) / (assign18960_e26798 * assign18960_e26798))))) * assign18960_e26824) + (assign18960_e26802 * ((-(var_phitdinv * ((-((var_v4 * var_nj1_dn2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn2 - var_nj0_dn2)) * assign18960_e26817) - (assign18960_e26814 * (var_nj0_dn2 * p.p85))) / (assign18960_e26817 * assign18960_e26817))))) * 0.3333333333333333))))))) / (assign18960_e26829 * assign18960_e26829))),)
    } else {
        (var_idmultbot, var_idmultbot_dn0, var_idmultbot_dn2,)
    }
};
        var_idmultbot = assign18960_e26832;
        var_idmultbot_dn0 = assign18960_e26832_d_n0;
        var_idmultbot_dn2 = assign18960_e26832_d_n2;
        var_idmultbot_rv = 0.0;

        *var_guard311_slot = var_guard311;
        *var_guard311_rv_slot = var_guard311_rv;
        *var_guard312_slot = var_guard312;
        *var_guard312_rv_slot = var_guard312_rv;
        *var_guard313_slot = var_guard313;
        *var_guard313_rv_slot = var_guard313_rv;
        *var_idmultbot_slot = var_idmultbot;
        *var_idmultbot_dn0_slot = var_idmultbot_dn0;
        *var_idmultbot_dn2_slot = var_idmultbot_dn2;
        *var_idmultbot_rv_slot = var_idmultbot_rv;
        *var_nj0_slot = var_nj0;
        *var_nj0_dn0_slot = var_nj0_dn0;
        *var_nj0_dn2_slot = var_nj0_dn2;
        *var_nj0_rv_slot = var_nj0_rv;
        *var_nj1_slot = var_nj1;
        *var_nj1_dn0_slot = var_nj1_dn0;
        *var_nj1_dn2_slot = var_nj1_dn2;
        *var_nj1_rv_slot = var_nj1_rv;
        *var_nja10_slot = var_nja10;
        *var_nja10_dn0_slot = var_nja10_dn0;
        *var_nja10_dn2_slot = var_nja10_dn2;
        *var_nja10_rv_slot = var_nja10_rv;
        *var_nja11_slot = var_nja11;
        *var_nja11_dn0_slot = var_nja11_dn0;
        *var_nja11_dn2_slot = var_nja11_dn2;
        *var_nja11_rv_slot = var_nja11_rv;
        *var_pnn0_slot = var_pnn0;
        *var_pnn0_rv_slot = var_pnn0_rv;
        *var_tmf1_slot = var_tmf1;
        *var_tmf1_dn0_slot = var_tmf1_dn0;
        *var_tmf1_dn2_slot = var_tmf1_dn2;
        *var_tmf1_rv_slot = var_tmf1_rv;
        *var_tmf2_slot = var_tmf2;
        *var_tmf2_dn0_slot = var_tmf2_dn0;
        *var_tmf2_dn2_slot = var_tmf2_dn2;
        *var_tmf2_rv_slot = var_tmf2_rv;
        *var_vha1_slot = var_vha1;
        *var_vha1_rv_slot = var_vha1_rv;
    }

    pub(super) fn stamp_reactive_block_20(
        p: &Parameters,
        var_guard307: f64,
        var_guard308: f64,
        var_guard31: f64,
        var_guard312: f64,
        var_guard313: f64,
        var_ndigat_i: f64,
        var_ndisti_i: f64,
        var_nfagat_i: f64,
        var_nfasti_i: f64,
        var_nin: f64,
        var_phitdinv: f64,
        var_v4: f64,
        var_guard314_slot: &mut f64,
        var_guard314_rv_slot: &mut f64,
        var_guard317_slot: &mut f64,
        var_guard317_rv_slot: &mut f64,
        var_idmultbot_slot: &mut f64,
        var_idmultbot_dn0_slot: &mut f64,
        var_idmultbot_dn2_slot: &mut f64,
        var_idmultbot_rv_slot: &mut f64,
        var_nj0_slot: &mut f64,
        var_nj0_dn0_slot: &mut f64,
        var_nj0_dn2_slot: &mut f64,
        var_nj0_rv_slot: &mut f64,
        var_nj1_slot: &mut f64,
        var_nj1_dn0_slot: &mut f64,
        var_nj1_dn2_slot: &mut f64,
        var_nj1_rv_slot: &mut f64,
        var_nja10_slot: &mut f64,
        var_nja10_dn0_slot: &mut f64,
        var_nja10_dn2_slot: &mut f64,
        var_nja10_rv_slot: &mut f64,
        var_nja11_slot: &mut f64,
        var_nja11_dn0_slot: &mut f64,
        var_nja11_dn2_slot: &mut f64,
        var_nja11_rv_slot: &mut f64,
        var_pnn0_slot: &mut f64,
        var_pnn0_rv_slot: &mut f64,
        var_tmf1_slot: &mut f64,
        var_tmf1_dn0_slot: &mut f64,
        var_tmf1_dn2_slot: &mut f64,
        var_tmf1_rv_slot: &mut f64,
        var_tmf2_slot: &mut f64,
        var_tmf2_dn0_slot: &mut f64,
        var_tmf2_dn2_slot: &mut f64,
        var_tmf2_rv_slot: &mut f64,
        var_vha1_slot: &mut f64,
        var_vha1_rv_slot: &mut f64,
    ) {
        let mut var_guard314: f64 = *var_guard314_slot;
        let mut var_guard314_rv: f64 = *var_guard314_rv_slot;
        let mut var_guard317: f64 = *var_guard317_slot;
        let mut var_guard317_rv: f64 = *var_guard317_rv_slot;
        let mut var_idmultbot: f64 = *var_idmultbot_slot;
        let mut var_idmultbot_dn0: f64 = *var_idmultbot_dn0_slot;
        let mut var_idmultbot_dn2: f64 = *var_idmultbot_dn2_slot;
        let mut var_idmultbot_rv: f64 = *var_idmultbot_rv_slot;
        let mut var_nj0: f64 = *var_nj0_slot;
        let mut var_nj0_dn0: f64 = *var_nj0_dn0_slot;
        let mut var_nj0_dn2: f64 = *var_nj0_dn2_slot;
        let mut var_nj0_rv: f64 = *var_nj0_rv_slot;
        let mut var_nj1: f64 = *var_nj1_slot;
        let mut var_nj1_dn0: f64 = *var_nj1_dn0_slot;
        let mut var_nj1_dn2: f64 = *var_nj1_dn2_slot;
        let mut var_nj1_rv: f64 = *var_nj1_rv_slot;
        let mut var_nja10: f64 = *var_nja10_slot;
        let mut var_nja10_dn0: f64 = *var_nja10_dn0_slot;
        let mut var_nja10_dn2: f64 = *var_nja10_dn2_slot;
        let mut var_nja10_rv: f64 = *var_nja10_rv_slot;
        let mut var_nja11: f64 = *var_nja11_slot;
        let mut var_nja11_dn0: f64 = *var_nja11_dn0_slot;
        let mut var_nja11_dn2: f64 = *var_nja11_dn2_slot;
        let mut var_nja11_rv: f64 = *var_nja11_rv_slot;
        let mut var_pnn0: f64 = *var_pnn0_slot;
        let mut var_pnn0_rv: f64 = *var_pnn0_rv_slot;
        let mut var_tmf1: f64 = *var_tmf1_slot;
        let mut var_tmf1_dn0: f64 = *var_tmf1_dn0_slot;
        let mut var_tmf1_dn2: f64 = *var_tmf1_dn2_slot;
        let mut var_tmf1_rv: f64 = *var_tmf1_rv_slot;
        let mut var_tmf2: f64 = *var_tmf2_slot;
        let mut var_tmf2_dn0: f64 = *var_tmf2_dn0_slot;
        let mut var_tmf2_dn2: f64 = *var_tmf2_dn2_slot;
        let mut var_tmf2_rv: f64 = *var_tmf2_rv_slot;
        let mut var_vha1: f64 = *var_vha1_slot;
        let mut var_vha1_rv: f64 = *var_vha1_rv_slot;

        let (assign18970_e26910, assign18970_e26910_d_n0, assign18970_e26910_d_n2,) = {
    if (((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 != 0.0)) && (var_guard312 == 0.0)) && (var_guard313 == 0.0)) {
        let assign18970_e26849: f64 = (var_v4 / var_nj1);
        let assign18970_e26853: f64 = (var_nj1 - var_nj0);
        let assign18970_e26854: f64 = (var_vha1 * assign18970_e26853);
        let assign18970_e26857: f64 = (var_nj0 * p.p85);
        let assign18970_e26858: f64 = (assign18970_e26854 / assign18970_e26857);
        let assign18970_e26859: f64 = (assign18970_e26849 + assign18970_e26858);
        let assign18970_e26860: f64 = (var_phitdinv * assign18970_e26859);
        let assign18970_e26862: f64 = (assign18970_e26860 - 230.25850929940458);
        let assign18970_e26868: f64 = (var_v4 / var_nj1);
        let assign18970_e26872: f64 = (var_nj1 - var_nj0);
        let assign18970_e26873: f64 = (var_vha1 * assign18970_e26872);
        let assign18970_e26876: f64 = (var_nj0 * p.p85);
        let assign18970_e26877: f64 = (assign18970_e26873 / assign18970_e26876);
        let assign18970_e26878: f64 = (assign18970_e26868 + assign18970_e26877);
        let assign18970_e26879: f64 = (var_phitdinv * assign18970_e26878);
        let assign18970_e26881: f64 = (assign18970_e26879 - 230.25850929940458);
        let assign18970_e26886: f64 = (var_v4 / var_nj1);
        let assign18970_e26890: f64 = (var_nj1 - var_nj0);
        let assign18970_e26891: f64 = (var_vha1 * assign18970_e26890);
        let assign18970_e26894: f64 = (var_nj0 * p.p85);
        let assign18970_e26895: f64 = (assign18970_e26891 / assign18970_e26894);
        let assign18970_e26896: f64 = (assign18970_e26886 + assign18970_e26895);
        let assign18970_e26897: f64 = (var_phitdinv * assign18970_e26896);
        let assign18970_e26899: f64 = (assign18970_e26897 - 230.25850929940458);
        let assign18970_e26901: f64 = (assign18970_e26899 * 0.3333333333333333);
        let assign18970_e26902: f64 = (1.0 + assign18970_e26901);
        let assign18970_e26903: f64 = (assign18970_e26881 * assign18970_e26902);
        let assign18970_e26904: f64 = (0.5 * assign18970_e26903);
        let assign18970_e26905: f64 = (1.0 + assign18970_e26904);
        let assign18970_e26906: f64 = (assign18970_e26862 * assign18970_e26905);
        let assign18970_e26907: f64 = (1.0 + assign18970_e26906);
        let assign18970_e26908: f64 = (1e100 * assign18970_e26907);
        (assign18970_e26908, (1e100 * (((var_phitdinv * ((-((var_v4 * var_nj1_dn0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn0 - var_nj0_dn0)) * assign18970_e26857) - (assign18970_e26854 * (var_nj0_dn0 * p.p85))) / (assign18970_e26857 * assign18970_e26857)))) * assign18970_e26905) + (assign18970_e26862 * (0.5 * (((var_phitdinv * ((-((var_v4 * var_nj1_dn0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn0 - var_nj0_dn0)) * assign18970_e26876) - (assign18970_e26873 * (var_nj0_dn0 * p.p85))) / (assign18970_e26876 * assign18970_e26876)))) * assign18970_e26902) + (assign18970_e26881 * ((var_phitdinv * ((-((var_v4 * var_nj1_dn0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn0 - var_nj0_dn0)) * assign18970_e26894) - (assign18970_e26891 * (var_nj0_dn0 * p.p85))) / (assign18970_e26894 * assign18970_e26894)))) * 0.3333333333333333))))))), (1e100 * (((var_phitdinv * ((-((var_v4 * var_nj1_dn2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn2 - var_nj0_dn2)) * assign18970_e26857) - (assign18970_e26854 * (var_nj0_dn2 * p.p85))) / (assign18970_e26857 * assign18970_e26857)))) * assign18970_e26905) + (assign18970_e26862 * (0.5 * (((var_phitdinv * ((-((var_v4 * var_nj1_dn2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn2 - var_nj0_dn2)) * assign18970_e26876) - (assign18970_e26873 * (var_nj0_dn2 * p.p85))) / (assign18970_e26876 * assign18970_e26876)))) * assign18970_e26902) + (assign18970_e26881 * ((var_phitdinv * ((-((var_v4 * var_nj1_dn2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn2 - var_nj0_dn2)) * assign18970_e26894) - (assign18970_e26891 * (var_nj0_dn2 * p.p85))) / (assign18970_e26894 * assign18970_e26894)))) * 0.3333333333333333))))))),)
    } else {
        (var_idmultbot, var_idmultbot_dn0, var_idmultbot_dn2,)
    }
};
        var_idmultbot = assign18970_e26910;
        var_idmultbot_dn0 = assign18970_e26910_d_n0;
        var_idmultbot_dn2 = assign18970_e26910_d_n2;
        var_idmultbot_rv = 0.0;

        let (assign18980_e26922,) = {
    if (((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 != 0.0)) {
        let assign18980_e26918: f64 = (var_nin * var_nin);
        let assign18980_e26920: f64 = (assign18980_e26918 / var_ndisti_i);
        (assign18980_e26920,)
    } else {
        (var_pnn0,)
    }
};
        var_pnn0 = assign18980_e26922;
        var_pnn0_rv = 0.0;

        let (assign18990_e26937,) = {
    if (((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 != 0.0)) {
        let assign18990_e26930: f64 = (var_nfasti_i / var_phitdinv);
        let assign18990_e26933: f64 = (var_ndisti_i / var_pnn0);
        let assign18990_e26934: f64 = (assign18990_e26933).ln();
        let assign18990_e26935: f64 = (assign18990_e26930 * assign18990_e26934);
        (assign18990_e26935,)
    } else {
        (var_vha1,)
    }
};
        var_vha1 = assign18990_e26937;
        var_vha1_rv = 0.0;

        let assign19000_e26940: f64 = if var_nfasti_i < p.p85 { 1.0 } else { 0.0 };
        var_guard314 = assign19000_e26940;
        var_guard314_rv = 0.0;

        let (assign19010_e26956, assign19010_e26956_d_n0, assign19010_e26956_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 != 0.0)) && (var_guard314 != 0.0)) {
        let assign19010_e26951: f64 = (var_v4 - var_vha1);
        let assign19010_e26952: f64 = (p.p86 * assign19010_e26951);
        let assign19010_e26954: f64 = (assign19010_e26952 + var_nfasti_i);
        (assign19010_e26954, 0.0, 0.0,)
    } else {
        (var_nja10, var_nja10_dn0, var_nja10_dn2,)
    }
};
        var_nja10 = assign19010_e26956;
        var_nja10_dn0 = assign19010_e26956_d_n0;
        var_nja10_dn2 = assign19010_e26956_d_n2;
        var_nja10_rv = 0.0;

        let (assign19020_e26970, assign19020_e26970_d_n0, assign19020_e26970_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 != 0.0)) && (var_guard314 != 0.0)) {
        let assign19020_e26967: f64 = (p.p86 * var_vha1);
        let assign19020_e26968: f64 = (var_nfasti_i - assign19020_e26967);
        (assign19020_e26968, 0.0, 0.0,)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn2,)
    }
};
        var_nj0 = assign19020_e26970;
        var_nj0_dn0 = assign19020_e26970_d_n0;
        var_nj0_dn2 = assign19020_e26970_d_n2;
        var_nj0_rv = 0.0;

        let (assign19030_e26984, assign19030_e26984_d_n0, assign19030_e26984_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 != 0.0)) && (var_guard314 != 0.0)) {
        let assign19030_e26980: f64 = (p.p85 - var_nja10);
        let assign19030_e26982: f64 = (assign19030_e26980 - 0.01);
        (assign19030_e26982, (-var_nja10_dn0), (-var_nja10_dn2),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2,)
    }
};
        var_tmf1 = assign19030_e26984;
        var_tmf1_dn0 = assign19030_e26984_d_n0;
        var_tmf1_dn2 = assign19030_e26984_d_n2;
        var_tmf1_rv = 0.0;

        let (assign19040_e26998, assign19040_e26998_d_n0, assign19040_e26998_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 != 0.0)) && (var_guard314 != 0.0)) {
        let assign19040_e26994: f64 = (4.0 * p.p85);
        let assign19040_e26996: f64 = (assign19040_e26994 * 0.01);
        (assign19040_e26996, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign19040_e26998;
        var_tmf2_dn0 = assign19040_e26998_d_n0;
        var_tmf2_dn2 = assign19040_e26998_d_n2;
        var_tmf2_rv = 0.0;

        let (assign19050_e27014, assign19050_e27014_d_n0, assign19050_e27014_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 != 0.0)) && (var_guard314 != 0.0)) {
        let (assign19050_e27012, assign19050_e27012_d_n0, assign19050_e27012_d_n2,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
            } else {
                let assign19050_e27011: f64 = (-var_tmf2);
                (assign19050_e27011, (-var_tmf2_dn0), (-var_tmf2_dn2),)
            }
        };
        (assign19050_e27012, assign19050_e27012_d_n0, assign19050_e27012_d_n2,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign19050_e27014;
        var_tmf2_dn0 = assign19050_e27014_d_n0;
        var_tmf2_dn2 = assign19050_e27014_d_n2;
        var_tmf2_rv = 0.0;

        let (assign19060_e27029, assign19060_e27029_d_n0, assign19060_e27029_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 != 0.0)) && (var_guard314 != 0.0)) {
        let assign19060_e27024: f64 = (var_tmf1 * var_tmf1);
        let assign19060_e27026: f64 = (assign19060_e27024 + var_tmf2);
        let assign19060_e27027: f64 = (assign19060_e27026).sqrt();
        (assign19060_e27027, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign19060_e27027)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign19060_e27027)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign19060_e27029;
        var_tmf2_dn0 = assign19060_e27029_d_n0;
        var_tmf2_dn2 = assign19060_e27029_d_n2;
        var_tmf2_rv = 0.0;

        let (assign19070_e27045, assign19070_e27045_d_n0, assign19070_e27045_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 != 0.0)) && (var_guard314 != 0.0)) {
        let assign19070_e27041: f64 = (var_tmf1 + var_tmf2);
        let assign19070_e27042: f64 = (0.5 * assign19070_e27041);
        let assign19070_e27043: f64 = (p.p85 - assign19070_e27042);
        (assign19070_e27043, (-(0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (-(0.5 * (var_tmf1_dn2 + var_tmf2_dn2))),)
    } else {
        (var_nja11, var_nja11_dn0, var_nja11_dn2,)
    }
};
        var_nja11 = assign19070_e27045;
        var_nja11_dn0 = assign19070_e27045_d_n0;
        var_nja11_dn2 = assign19070_e27045_d_n2;
        var_nja11_rv = 0.0;

        let (assign19080_e27059, assign19080_e27059_d_n0, assign19080_e27059_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 != 0.0)) && (var_guard314 != 0.0)) {
        let assign19080_e27055: f64 = (var_nja11 - var_nfasti_i);
        let assign19080_e27057: f64 = (assign19080_e27055 - 0.01);
        (assign19080_e27057, var_nja11_dn0, var_nja11_dn2,)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2,)
    }
};
        var_tmf1 = assign19080_e27059;
        var_tmf1_dn0 = assign19080_e27059_d_n0;
        var_tmf1_dn2 = assign19080_e27059_d_n2;
        var_tmf1_rv = 0.0;

        let (assign19090_e27073, assign19090_e27073_d_n0, assign19090_e27073_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 != 0.0)) && (var_guard314 != 0.0)) {
        let assign19090_e27069: f64 = (4.0 * var_nfasti_i);
        let assign19090_e27071: f64 = (assign19090_e27069 * 0.01);
        (assign19090_e27071, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign19090_e27073;
        var_tmf2_dn0 = assign19090_e27073_d_n0;
        var_tmf2_dn2 = assign19090_e27073_d_n2;
        var_tmf2_rv = 0.0;

        let (assign19100_e27089, assign19100_e27089_d_n0, assign19100_e27089_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 != 0.0)) && (var_guard314 != 0.0)) {
        let (assign19100_e27087, assign19100_e27087_d_n0, assign19100_e27087_d_n2,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
            } else {
                let assign19100_e27086: f64 = (-var_tmf2);
                (assign19100_e27086, (-var_tmf2_dn0), (-var_tmf2_dn2),)
            }
        };
        (assign19100_e27087, assign19100_e27087_d_n0, assign19100_e27087_d_n2,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign19100_e27089;
        var_tmf2_dn0 = assign19100_e27089_d_n0;
        var_tmf2_dn2 = assign19100_e27089_d_n2;
        var_tmf2_rv = 0.0;

        let (assign19110_e27104, assign19110_e27104_d_n0, assign19110_e27104_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 != 0.0)) && (var_guard314 != 0.0)) {
        let assign19110_e27099: f64 = (var_tmf1 * var_tmf1);
        let assign19110_e27101: f64 = (assign19110_e27099 + var_tmf2);
        let assign19110_e27102: f64 = (assign19110_e27101).sqrt();
        (assign19110_e27102, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign19110_e27102)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign19110_e27102)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign19110_e27104;
        var_tmf2_dn0 = assign19110_e27104_d_n0;
        var_tmf2_dn2 = assign19110_e27104_d_n2;
        var_tmf2_rv = 0.0;

        let (assign19120_e27120, assign19120_e27120_d_n0, assign19120_e27120_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 != 0.0)) && (var_guard314 != 0.0)) {
        let assign19120_e27116: f64 = (var_tmf1 + var_tmf2);
        let assign19120_e27117: f64 = (0.5 * assign19120_e27116);
        let assign19120_e27118: f64 = (var_nfasti_i + assign19120_e27117);
        (assign19120_e27118, (0.5 * (var_tmf1_dn0 + var_tmf2_dn0)), (0.5 * (var_tmf1_dn2 + var_tmf2_dn2)),)
    } else {
        (var_nj1, var_nj1_dn0, var_nj1_dn2,)
    }
};
        var_nj1 = assign19120_e27120;
        var_nj1_dn0 = assign19120_e27120_d_n0;
        var_nj1_dn2 = assign19120_e27120_d_n2;
        var_nj1_rv = 0.0;

        let (assign19130_e27134, assign19130_e27134_d_n0, assign19130_e27134_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 != 0.0)) && (var_guard314 != 0.0)) {
        let assign19130_e27130: f64 = (p.p85 - var_nj0);
        let assign19130_e27132: f64 = (assign19130_e27130 - 0.01);
        (assign19130_e27132, (-var_nj0_dn0), (-var_nj0_dn2),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2,)
    }
};
        var_tmf1 = assign19130_e27134;
        var_tmf1_dn0 = assign19130_e27134_d_n0;
        var_tmf1_dn2 = assign19130_e27134_d_n2;
        var_tmf1_rv = 0.0;

        let (assign19140_e27148, assign19140_e27148_d_n0, assign19140_e27148_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 != 0.0)) && (var_guard314 != 0.0)) {
        let assign19140_e27144: f64 = (4.0 * p.p85);
        let assign19140_e27146: f64 = (assign19140_e27144 * 0.01);
        (assign19140_e27146, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign19140_e27148;
        var_tmf2_dn0 = assign19140_e27148_d_n0;
        var_tmf2_dn2 = assign19140_e27148_d_n2;
        var_tmf2_rv = 0.0;

        let (assign19150_e27164, assign19150_e27164_d_n0, assign19150_e27164_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 != 0.0)) && (var_guard314 != 0.0)) {
        let (assign19150_e27162, assign19150_e27162_d_n0, assign19150_e27162_d_n2,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
            } else {
                let assign19150_e27161: f64 = (-var_tmf2);
                (assign19150_e27161, (-var_tmf2_dn0), (-var_tmf2_dn2),)
            }
        };
        (assign19150_e27162, assign19150_e27162_d_n0, assign19150_e27162_d_n2,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign19150_e27164;
        var_tmf2_dn0 = assign19150_e27164_d_n0;
        var_tmf2_dn2 = assign19150_e27164_d_n2;
        var_tmf2_rv = 0.0;

        let (assign19160_e27179, assign19160_e27179_d_n0, assign19160_e27179_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 != 0.0)) && (var_guard314 != 0.0)) {
        let assign19160_e27174: f64 = (var_tmf1 * var_tmf1);
        let assign19160_e27176: f64 = (assign19160_e27174 + var_tmf2);
        let assign19160_e27177: f64 = (assign19160_e27176).sqrt();
        (assign19160_e27177, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign19160_e27177)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign19160_e27177)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign19160_e27179;
        var_tmf2_dn0 = assign19160_e27179_d_n0;
        var_tmf2_dn2 = assign19160_e27179_d_n2;
        var_tmf2_rv = 0.0;

        let (assign19170_e27195, assign19170_e27195_d_n0, assign19170_e27195_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 != 0.0)) && (var_guard314 != 0.0)) {
        let assign19170_e27191: f64 = (var_tmf1 + var_tmf2);
        let assign19170_e27192: f64 = (0.5 * assign19170_e27191);
        let assign19170_e27193: f64 = (p.p85 - assign19170_e27192);
        (assign19170_e27193, (-(0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (-(0.5 * (var_tmf1_dn2 + var_tmf2_dn2))),)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn2,)
    }
};
        var_nj0 = assign19170_e27195;
        var_nj0_dn0 = assign19170_e27195_d_n0;
        var_nj0_dn2 = assign19170_e27195_d_n2;
        var_nj0_rv = 0.0;

        let (assign19180_e27209, assign19180_e27209_d_n0, assign19180_e27209_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 != 0.0)) && (var_guard314 != 0.0)) {
        let assign19180_e27205: f64 = (var_nj0 - var_nfasti_i);
        let assign19180_e27207: f64 = (assign19180_e27205 - 0.01);
        (assign19180_e27207, var_nj0_dn0, var_nj0_dn2,)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2,)
    }
};
        var_tmf1 = assign19180_e27209;
        var_tmf1_dn0 = assign19180_e27209_d_n0;
        var_tmf1_dn2 = assign19180_e27209_d_n2;
        var_tmf1_rv = 0.0;

        let (assign19190_e27223, assign19190_e27223_d_n0, assign19190_e27223_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 != 0.0)) && (var_guard314 != 0.0)) {
        let assign19190_e27219: f64 = (4.0 * var_nfasti_i);
        let assign19190_e27221: f64 = (assign19190_e27219 * 0.01);
        (assign19190_e27221, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign19190_e27223;
        var_tmf2_dn0 = assign19190_e27223_d_n0;
        var_tmf2_dn2 = assign19190_e27223_d_n2;
        var_tmf2_rv = 0.0;

        let (assign19200_e27239, assign19200_e27239_d_n0, assign19200_e27239_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 != 0.0)) && (var_guard314 != 0.0)) {
        let (assign19200_e27237, assign19200_e27237_d_n0, assign19200_e27237_d_n2,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
            } else {
                let assign19200_e27236: f64 = (-var_tmf2);
                (assign19200_e27236, (-var_tmf2_dn0), (-var_tmf2_dn2),)
            }
        };
        (assign19200_e27237, assign19200_e27237_d_n0, assign19200_e27237_d_n2,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign19200_e27239;
        var_tmf2_dn0 = assign19200_e27239_d_n0;
        var_tmf2_dn2 = assign19200_e27239_d_n2;
        var_tmf2_rv = 0.0;

        let (assign19210_e27254, assign19210_e27254_d_n0, assign19210_e27254_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 != 0.0)) && (var_guard314 != 0.0)) {
        let assign19210_e27249: f64 = (var_tmf1 * var_tmf1);
        let assign19210_e27251: f64 = (assign19210_e27249 + var_tmf2);
        let assign19210_e27252: f64 = (assign19210_e27251).sqrt();
        (assign19210_e27252, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign19210_e27252)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign19210_e27252)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign19210_e27254;
        var_tmf2_dn0 = assign19210_e27254_d_n0;
        var_tmf2_dn2 = assign19210_e27254_d_n2;
        var_tmf2_rv = 0.0;

        let (assign19220_e27270, assign19220_e27270_d_n0, assign19220_e27270_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 != 0.0)) && (var_guard314 != 0.0)) {
        let assign19220_e27266: f64 = (var_tmf1 + var_tmf2);
        let assign19220_e27267: f64 = (0.5 * assign19220_e27266);
        let assign19220_e27268: f64 = (var_nfasti_i + assign19220_e27267);
        (assign19220_e27268, (0.5 * (var_tmf1_dn0 + var_tmf2_dn0)), (0.5 * (var_tmf1_dn2 + var_tmf2_dn2)),)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn2,)
    }
};
        var_nj0 = assign19220_e27270;
        var_nj0_dn0 = assign19220_e27270_d_n0;
        var_nj0_dn2 = assign19220_e27270_d_n2;
        var_nj0_rv = 0.0;

        let (assign19230_e27281, assign19230_e27281_d_n0, assign19230_e27281_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 != 0.0)) && (var_guard314 == 0.0)) {
        (var_nfasti_i, 0.0, 0.0,)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn2,)
    }
};
        var_nj0 = assign19230_e27281;
        var_nj0_dn0 = assign19230_e27281_d_n0;
        var_nj0_dn2 = assign19230_e27281_d_n2;
        var_nj0_rv = 0.0;

        let (assign19240_e27292, assign19240_e27292_d_n0, assign19240_e27292_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 != 0.0)) && (var_guard314 == 0.0)) {
        (var_nfasti_i, 0.0, 0.0,)
    } else {
        (var_nj1, var_nj1_dn0, var_nj1_dn2,)
    }
};
        var_nj1 = assign19240_e27292;
        var_nj1_dn0 = assign19240_e27292_d_n0;
        var_nj1_dn2 = assign19240_e27292_d_n2;
        var_nj1_rv = 0.0;

        let (assign19300_e27523,) = {
    if (((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 != 0.0)) {
        let assign19300_e27519: f64 = (var_nin * var_nin);
        let assign19300_e27521: f64 = (assign19300_e27519 / var_ndigat_i);
        (assign19300_e27521,)
    } else {
        (var_pnn0,)
    }
};
        var_pnn0 = assign19300_e27523;
        var_pnn0_rv = 0.0;

        let (assign19310_e27538,) = {
    if (((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 != 0.0)) {
        let assign19310_e27531: f64 = (var_nfagat_i / var_phitdinv);
        let assign19310_e27534: f64 = (var_ndigat_i / var_pnn0);
        let assign19310_e27535: f64 = (assign19310_e27534).ln();
        let assign19310_e27536: f64 = (assign19310_e27531 * assign19310_e27535);
        (assign19310_e27536,)
    } else {
        (var_vha1,)
    }
};
        var_vha1 = assign19310_e27538;
        var_vha1_rv = 0.0;

        let assign19320_e27541: f64 = if var_nfagat_i < p.p85 { 1.0 } else { 0.0 };
        var_guard317 = assign19320_e27541;
        var_guard317_rv = 0.0;

        let (assign19330_e27557, assign19330_e27557_d_n0, assign19330_e27557_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 != 0.0)) && (var_guard317 != 0.0)) {
        let assign19330_e27552: f64 = (var_v4 - var_vha1);
        let assign19330_e27553: f64 = (p.p86 * assign19330_e27552);
        let assign19330_e27555: f64 = (assign19330_e27553 + var_nfagat_i);
        (assign19330_e27555, 0.0, 0.0,)
    } else {
        (var_nja10, var_nja10_dn0, var_nja10_dn2,)
    }
};
        var_nja10 = assign19330_e27557;
        var_nja10_dn0 = assign19330_e27557_d_n0;
        var_nja10_dn2 = assign19330_e27557_d_n2;
        var_nja10_rv = 0.0;

        let (assign19340_e27571, assign19340_e27571_d_n0, assign19340_e27571_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 != 0.0)) && (var_guard317 != 0.0)) {
        let assign19340_e27568: f64 = (p.p86 * var_vha1);
        let assign19340_e27569: f64 = (var_nfagat_i - assign19340_e27568);
        (assign19340_e27569, 0.0, 0.0,)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn2,)
    }
};
        var_nj0 = assign19340_e27571;
        var_nj0_dn0 = assign19340_e27571_d_n0;
        var_nj0_dn2 = assign19340_e27571_d_n2;
        var_nj0_rv = 0.0;

        let (assign19350_e27585, assign19350_e27585_d_n0, assign19350_e27585_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 != 0.0)) && (var_guard317 != 0.0)) {
        let assign19350_e27581: f64 = (p.p85 - var_nja10);
        let assign19350_e27583: f64 = (assign19350_e27581 - 0.01);
        (assign19350_e27583, (-var_nja10_dn0), (-var_nja10_dn2),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2,)
    }
};
        var_tmf1 = assign19350_e27585;
        var_tmf1_dn0 = assign19350_e27585_d_n0;
        var_tmf1_dn2 = assign19350_e27585_d_n2;
        var_tmf1_rv = 0.0;

        *var_guard314_slot = var_guard314;
        *var_guard314_rv_slot = var_guard314_rv;
        *var_guard317_slot = var_guard317;
        *var_guard317_rv_slot = var_guard317_rv;
        *var_idmultbot_slot = var_idmultbot;
        *var_idmultbot_dn0_slot = var_idmultbot_dn0;
        *var_idmultbot_dn2_slot = var_idmultbot_dn2;
        *var_idmultbot_rv_slot = var_idmultbot_rv;
        *var_nj0_slot = var_nj0;
        *var_nj0_dn0_slot = var_nj0_dn0;
        *var_nj0_dn2_slot = var_nj0_dn2;
        *var_nj0_rv_slot = var_nj0_rv;
        *var_nj1_slot = var_nj1;
        *var_nj1_dn0_slot = var_nj1_dn0;
        *var_nj1_dn2_slot = var_nj1_dn2;
        *var_nj1_rv_slot = var_nj1_rv;
        *var_nja10_slot = var_nja10;
        *var_nja10_dn0_slot = var_nja10_dn0;
        *var_nja10_dn2_slot = var_nja10_dn2;
        *var_nja10_rv_slot = var_nja10_rv;
        *var_nja11_slot = var_nja11;
        *var_nja11_dn0_slot = var_nja11_dn0;
        *var_nja11_dn2_slot = var_nja11_dn2;
        *var_nja11_rv_slot = var_nja11_rv;
        *var_pnn0_slot = var_pnn0;
        *var_pnn0_rv_slot = var_pnn0_rv;
        *var_tmf1_slot = var_tmf1;
        *var_tmf1_dn0_slot = var_tmf1_dn0;
        *var_tmf1_dn2_slot = var_tmf1_dn2;
        *var_tmf1_rv_slot = var_tmf1_rv;
        *var_tmf2_slot = var_tmf2;
        *var_tmf2_dn0_slot = var_tmf2_dn0;
        *var_tmf2_dn2_slot = var_tmf2_dn2;
        *var_tmf2_rv_slot = var_tmf2_rv;
        *var_vha1_slot = var_vha1;
        *var_vha1_rv_slot = var_vha1_rv;
    }

    pub(super) fn stamp_reactive_block_21(
        p: &Parameters,
        var_guard307: f64,
        var_guard308: f64,
        var_guard31: f64,
        var_guard317: f64,
        var_ndibot_i: f64,
        var_nfabot_i: f64,
        var_nfagat_i: f64,
        var_nin: f64,
        var_phitdinv: f64,
        var_vmax: f64,
        var_dfn_su_slot: &mut f64,
        var_dfn_su_dn0_slot: &mut f64,
        var_dfn_su_dn2_slot: &mut f64,
        var_dfn_su_rv_slot: &mut f64,
        var_guard320_slot: &mut f64,
        var_guard320_rv_slot: &mut f64,
        var_nj0_slot: &mut f64,
        var_nj0_dn0_slot: &mut f64,
        var_nj0_dn2_slot: &mut f64,
        var_nj0_rv_slot: &mut f64,
        var_nj1_slot: &mut f64,
        var_nj1_dn0_slot: &mut f64,
        var_nj1_dn2_slot: &mut f64,
        var_nj1_rv_slot: &mut f64,
        var_nja10_slot: &mut f64,
        var_nja10_dn0_slot: &mut f64,
        var_nja10_dn2_slot: &mut f64,
        var_nja10_rv_slot: &mut f64,
        var_nja11_slot: &mut f64,
        var_nja11_dn0_slot: &mut f64,
        var_nja11_dn2_slot: &mut f64,
        var_nja11_rv_slot: &mut f64,
        var_pnn0_slot: &mut f64,
        var_pnn0_rv_slot: &mut f64,
        var_tmf1_slot: &mut f64,
        var_tmf1_dn0_slot: &mut f64,
        var_tmf1_dn2_slot: &mut f64,
        var_tmf1_rv_slot: &mut f64,
        var_tmf2_slot: &mut f64,
        var_tmf2_dn0_slot: &mut f64,
        var_tmf2_dn2_slot: &mut f64,
        var_tmf2_rv_slot: &mut f64,
        var_vha1_slot: &mut f64,
        var_vha1_rv_slot: &mut f64,
    ) {
        let mut var_dfn_su: f64 = *var_dfn_su_slot;
        let mut var_dfn_su_dn0: f64 = *var_dfn_su_dn0_slot;
        let mut var_dfn_su_dn2: f64 = *var_dfn_su_dn2_slot;
        let mut var_dfn_su_rv: f64 = *var_dfn_su_rv_slot;
        let mut var_guard320: f64 = *var_guard320_slot;
        let mut var_guard320_rv: f64 = *var_guard320_rv_slot;
        let mut var_nj0: f64 = *var_nj0_slot;
        let mut var_nj0_dn0: f64 = *var_nj0_dn0_slot;
        let mut var_nj0_dn2: f64 = *var_nj0_dn2_slot;
        let mut var_nj0_rv: f64 = *var_nj0_rv_slot;
        let mut var_nj1: f64 = *var_nj1_slot;
        let mut var_nj1_dn0: f64 = *var_nj1_dn0_slot;
        let mut var_nj1_dn2: f64 = *var_nj1_dn2_slot;
        let mut var_nj1_rv: f64 = *var_nj1_rv_slot;
        let mut var_nja10: f64 = *var_nja10_slot;
        let mut var_nja10_dn0: f64 = *var_nja10_dn0_slot;
        let mut var_nja10_dn2: f64 = *var_nja10_dn2_slot;
        let mut var_nja10_rv: f64 = *var_nja10_rv_slot;
        let mut var_nja11: f64 = *var_nja11_slot;
        let mut var_nja11_dn0: f64 = *var_nja11_dn0_slot;
        let mut var_nja11_dn2: f64 = *var_nja11_dn2_slot;
        let mut var_nja11_rv: f64 = *var_nja11_rv_slot;
        let mut var_pnn0: f64 = *var_pnn0_slot;
        let mut var_pnn0_rv: f64 = *var_pnn0_rv_slot;
        let mut var_tmf1: f64 = *var_tmf1_slot;
        let mut var_tmf1_dn0: f64 = *var_tmf1_dn0_slot;
        let mut var_tmf1_dn2: f64 = *var_tmf1_dn2_slot;
        let mut var_tmf1_rv: f64 = *var_tmf1_rv_slot;
        let mut var_tmf2: f64 = *var_tmf2_slot;
        let mut var_tmf2_dn0: f64 = *var_tmf2_dn0_slot;
        let mut var_tmf2_dn2: f64 = *var_tmf2_dn2_slot;
        let mut var_tmf2_rv: f64 = *var_tmf2_rv_slot;
        let mut var_vha1: f64 = *var_vha1_slot;
        let mut var_vha1_rv: f64 = *var_vha1_rv_slot;

        let (assign19360_e27599, assign19360_e27599_d_n0, assign19360_e27599_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 != 0.0)) && (var_guard317 != 0.0)) {
        let assign19360_e27595: f64 = (4.0 * p.p85);
        let assign19360_e27597: f64 = (assign19360_e27595 * 0.01);
        (assign19360_e27597, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign19360_e27599;
        var_tmf2_dn0 = assign19360_e27599_d_n0;
        var_tmf2_dn2 = assign19360_e27599_d_n2;
        var_tmf2_rv = 0.0;

        let (assign19370_e27615, assign19370_e27615_d_n0, assign19370_e27615_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 != 0.0)) && (var_guard317 != 0.0)) {
        let (assign19370_e27613, assign19370_e27613_d_n0, assign19370_e27613_d_n2,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
            } else {
                let assign19370_e27612: f64 = (-var_tmf2);
                (assign19370_e27612, (-var_tmf2_dn0), (-var_tmf2_dn2),)
            }
        };
        (assign19370_e27613, assign19370_e27613_d_n0, assign19370_e27613_d_n2,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign19370_e27615;
        var_tmf2_dn0 = assign19370_e27615_d_n0;
        var_tmf2_dn2 = assign19370_e27615_d_n2;
        var_tmf2_rv = 0.0;

        let (assign19380_e27630, assign19380_e27630_d_n0, assign19380_e27630_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 != 0.0)) && (var_guard317 != 0.0)) {
        let assign19380_e27625: f64 = (var_tmf1 * var_tmf1);
        let assign19380_e27627: f64 = (assign19380_e27625 + var_tmf2);
        let assign19380_e27628: f64 = (assign19380_e27627).sqrt();
        (assign19380_e27628, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign19380_e27628)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign19380_e27628)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign19380_e27630;
        var_tmf2_dn0 = assign19380_e27630_d_n0;
        var_tmf2_dn2 = assign19380_e27630_d_n2;
        var_tmf2_rv = 0.0;

        let (assign19390_e27646, assign19390_e27646_d_n0, assign19390_e27646_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 != 0.0)) && (var_guard317 != 0.0)) {
        let assign19390_e27642: f64 = (var_tmf1 + var_tmf2);
        let assign19390_e27643: f64 = (0.5 * assign19390_e27642);
        let assign19390_e27644: f64 = (p.p85 - assign19390_e27643);
        (assign19390_e27644, (-(0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (-(0.5 * (var_tmf1_dn2 + var_tmf2_dn2))),)
    } else {
        (var_nja11, var_nja11_dn0, var_nja11_dn2,)
    }
};
        var_nja11 = assign19390_e27646;
        var_nja11_dn0 = assign19390_e27646_d_n0;
        var_nja11_dn2 = assign19390_e27646_d_n2;
        var_nja11_rv = 0.0;

        let (assign19400_e27660, assign19400_e27660_d_n0, assign19400_e27660_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 != 0.0)) && (var_guard317 != 0.0)) {
        let assign19400_e27656: f64 = (var_nja11 - var_nfagat_i);
        let assign19400_e27658: f64 = (assign19400_e27656 - 0.01);
        (assign19400_e27658, var_nja11_dn0, var_nja11_dn2,)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2,)
    }
};
        var_tmf1 = assign19400_e27660;
        var_tmf1_dn0 = assign19400_e27660_d_n0;
        var_tmf1_dn2 = assign19400_e27660_d_n2;
        var_tmf1_rv = 0.0;

        let (assign19410_e27674, assign19410_e27674_d_n0, assign19410_e27674_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 != 0.0)) && (var_guard317 != 0.0)) {
        let assign19410_e27670: f64 = (4.0 * var_nfagat_i);
        let assign19410_e27672: f64 = (assign19410_e27670 * 0.01);
        (assign19410_e27672, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign19410_e27674;
        var_tmf2_dn0 = assign19410_e27674_d_n0;
        var_tmf2_dn2 = assign19410_e27674_d_n2;
        var_tmf2_rv = 0.0;

        let (assign19420_e27690, assign19420_e27690_d_n0, assign19420_e27690_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 != 0.0)) && (var_guard317 != 0.0)) {
        let (assign19420_e27688, assign19420_e27688_d_n0, assign19420_e27688_d_n2,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
            } else {
                let assign19420_e27687: f64 = (-var_tmf2);
                (assign19420_e27687, (-var_tmf2_dn0), (-var_tmf2_dn2),)
            }
        };
        (assign19420_e27688, assign19420_e27688_d_n0, assign19420_e27688_d_n2,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign19420_e27690;
        var_tmf2_dn0 = assign19420_e27690_d_n0;
        var_tmf2_dn2 = assign19420_e27690_d_n2;
        var_tmf2_rv = 0.0;

        let (assign19430_e27705, assign19430_e27705_d_n0, assign19430_e27705_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 != 0.0)) && (var_guard317 != 0.0)) {
        let assign19430_e27700: f64 = (var_tmf1 * var_tmf1);
        let assign19430_e27702: f64 = (assign19430_e27700 + var_tmf2);
        let assign19430_e27703: f64 = (assign19430_e27702).sqrt();
        (assign19430_e27703, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign19430_e27703)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign19430_e27703)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign19430_e27705;
        var_tmf2_dn0 = assign19430_e27705_d_n0;
        var_tmf2_dn2 = assign19430_e27705_d_n2;
        var_tmf2_rv = 0.0;

        let (assign19440_e27721, assign19440_e27721_d_n0, assign19440_e27721_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 != 0.0)) && (var_guard317 != 0.0)) {
        let assign19440_e27717: f64 = (var_tmf1 + var_tmf2);
        let assign19440_e27718: f64 = (0.5 * assign19440_e27717);
        let assign19440_e27719: f64 = (var_nfagat_i + assign19440_e27718);
        (assign19440_e27719, (0.5 * (var_tmf1_dn0 + var_tmf2_dn0)), (0.5 * (var_tmf1_dn2 + var_tmf2_dn2)),)
    } else {
        (var_nj1, var_nj1_dn0, var_nj1_dn2,)
    }
};
        var_nj1 = assign19440_e27721;
        var_nj1_dn0 = assign19440_e27721_d_n0;
        var_nj1_dn2 = assign19440_e27721_d_n2;
        var_nj1_rv = 0.0;

        let (assign19450_e27735, assign19450_e27735_d_n0, assign19450_e27735_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 != 0.0)) && (var_guard317 != 0.0)) {
        let assign19450_e27731: f64 = (p.p85 - var_nj0);
        let assign19450_e27733: f64 = (assign19450_e27731 - 0.01);
        (assign19450_e27733, (-var_nj0_dn0), (-var_nj0_dn2),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2,)
    }
};
        var_tmf1 = assign19450_e27735;
        var_tmf1_dn0 = assign19450_e27735_d_n0;
        var_tmf1_dn2 = assign19450_e27735_d_n2;
        var_tmf1_rv = 0.0;

        let (assign19460_e27749, assign19460_e27749_d_n0, assign19460_e27749_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 != 0.0)) && (var_guard317 != 0.0)) {
        let assign19460_e27745: f64 = (4.0 * p.p85);
        let assign19460_e27747: f64 = (assign19460_e27745 * 0.01);
        (assign19460_e27747, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign19460_e27749;
        var_tmf2_dn0 = assign19460_e27749_d_n0;
        var_tmf2_dn2 = assign19460_e27749_d_n2;
        var_tmf2_rv = 0.0;

        let (assign19470_e27765, assign19470_e27765_d_n0, assign19470_e27765_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 != 0.0)) && (var_guard317 != 0.0)) {
        let (assign19470_e27763, assign19470_e27763_d_n0, assign19470_e27763_d_n2,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
            } else {
                let assign19470_e27762: f64 = (-var_tmf2);
                (assign19470_e27762, (-var_tmf2_dn0), (-var_tmf2_dn2),)
            }
        };
        (assign19470_e27763, assign19470_e27763_d_n0, assign19470_e27763_d_n2,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign19470_e27765;
        var_tmf2_dn0 = assign19470_e27765_d_n0;
        var_tmf2_dn2 = assign19470_e27765_d_n2;
        var_tmf2_rv = 0.0;

        let (assign19480_e27780, assign19480_e27780_d_n0, assign19480_e27780_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 != 0.0)) && (var_guard317 != 0.0)) {
        let assign19480_e27775: f64 = (var_tmf1 * var_tmf1);
        let assign19480_e27777: f64 = (assign19480_e27775 + var_tmf2);
        let assign19480_e27778: f64 = (assign19480_e27777).sqrt();
        (assign19480_e27778, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign19480_e27778)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign19480_e27778)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign19480_e27780;
        var_tmf2_dn0 = assign19480_e27780_d_n0;
        var_tmf2_dn2 = assign19480_e27780_d_n2;
        var_tmf2_rv = 0.0;

        let (assign19490_e27796, assign19490_e27796_d_n0, assign19490_e27796_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 != 0.0)) && (var_guard317 != 0.0)) {
        let assign19490_e27792: f64 = (var_tmf1 + var_tmf2);
        let assign19490_e27793: f64 = (0.5 * assign19490_e27792);
        let assign19490_e27794: f64 = (p.p85 - assign19490_e27793);
        (assign19490_e27794, (-(0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (-(0.5 * (var_tmf1_dn2 + var_tmf2_dn2))),)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn2,)
    }
};
        var_nj0 = assign19490_e27796;
        var_nj0_dn0 = assign19490_e27796_d_n0;
        var_nj0_dn2 = assign19490_e27796_d_n2;
        var_nj0_rv = 0.0;

        let (assign19500_e27810, assign19500_e27810_d_n0, assign19500_e27810_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 != 0.0)) && (var_guard317 != 0.0)) {
        let assign19500_e27806: f64 = (var_nj0 - var_nfagat_i);
        let assign19500_e27808: f64 = (assign19500_e27806 - 0.01);
        (assign19500_e27808, var_nj0_dn0, var_nj0_dn2,)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2,)
    }
};
        var_tmf1 = assign19500_e27810;
        var_tmf1_dn0 = assign19500_e27810_d_n0;
        var_tmf1_dn2 = assign19500_e27810_d_n2;
        var_tmf1_rv = 0.0;

        let (assign19510_e27824, assign19510_e27824_d_n0, assign19510_e27824_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 != 0.0)) && (var_guard317 != 0.0)) {
        let assign19510_e27820: f64 = (4.0 * var_nfagat_i);
        let assign19510_e27822: f64 = (assign19510_e27820 * 0.01);
        (assign19510_e27822, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign19510_e27824;
        var_tmf2_dn0 = assign19510_e27824_d_n0;
        var_tmf2_dn2 = assign19510_e27824_d_n2;
        var_tmf2_rv = 0.0;

        let (assign19520_e27840, assign19520_e27840_d_n0, assign19520_e27840_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 != 0.0)) && (var_guard317 != 0.0)) {
        let (assign19520_e27838, assign19520_e27838_d_n0, assign19520_e27838_d_n2,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
            } else {
                let assign19520_e27837: f64 = (-var_tmf2);
                (assign19520_e27837, (-var_tmf2_dn0), (-var_tmf2_dn2),)
            }
        };
        (assign19520_e27838, assign19520_e27838_d_n0, assign19520_e27838_d_n2,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign19520_e27840;
        var_tmf2_dn0 = assign19520_e27840_d_n0;
        var_tmf2_dn2 = assign19520_e27840_d_n2;
        var_tmf2_rv = 0.0;

        let (assign19530_e27855, assign19530_e27855_d_n0, assign19530_e27855_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 != 0.0)) && (var_guard317 != 0.0)) {
        let assign19530_e27850: f64 = (var_tmf1 * var_tmf1);
        let assign19530_e27852: f64 = (assign19530_e27850 + var_tmf2);
        let assign19530_e27853: f64 = (assign19530_e27852).sqrt();
        (assign19530_e27853, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign19530_e27853)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign19530_e27853)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign19530_e27855;
        var_tmf2_dn0 = assign19530_e27855_d_n0;
        var_tmf2_dn2 = assign19530_e27855_d_n2;
        var_tmf2_rv = 0.0;

        let (assign19540_e27871, assign19540_e27871_d_n0, assign19540_e27871_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 != 0.0)) && (var_guard317 != 0.0)) {
        let assign19540_e27867: f64 = (var_tmf1 + var_tmf2);
        let assign19540_e27868: f64 = (0.5 * assign19540_e27867);
        let assign19540_e27869: f64 = (var_nfagat_i + assign19540_e27868);
        (assign19540_e27869, (0.5 * (var_tmf1_dn0 + var_tmf2_dn0)), (0.5 * (var_tmf1_dn2 + var_tmf2_dn2)),)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn2,)
    }
};
        var_nj0 = assign19540_e27871;
        var_nj0_dn0 = assign19540_e27871_d_n0;
        var_nj0_dn2 = assign19540_e27871_d_n2;
        var_nj0_rv = 0.0;

        let (assign19550_e27882, assign19550_e27882_d_n0, assign19550_e27882_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 != 0.0)) && (var_guard317 == 0.0)) {
        (var_nfagat_i, 0.0, 0.0,)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn2,)
    }
};
        var_nj0 = assign19550_e27882;
        var_nj0_dn0 = assign19550_e27882_d_n0;
        var_nj0_dn2 = assign19550_e27882_d_n2;
        var_nj0_rv = 0.0;

        let (assign19560_e27893, assign19560_e27893_d_n0, assign19560_e27893_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 != 0.0)) && (var_guard317 == 0.0)) {
        (var_nfagat_i, 0.0, 0.0,)
    } else {
        (var_nj1, var_nj1_dn0, var_nj1_dn2,)
    }
};
        var_nj1 = assign19560_e27893;
        var_nj1_dn0 = assign19560_e27893_d_n0;
        var_nj1_dn2 = assign19560_e27893_d_n2;
        var_nj1_rv = 0.0;

        let (assign19630_e28143,) = {
    if (((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) {
        let assign19630_e28139: f64 = (var_nin * var_nin);
        let assign19630_e28141: f64 = (assign19630_e28139 / var_ndibot_i);
        (assign19630_e28141,)
    } else {
        (var_pnn0,)
    }
};
        var_pnn0 = assign19630_e28143;
        var_pnn0_rv = 0.0;

        let (assign19640_e28159,) = {
    if (((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) {
        let assign19640_e28152: f64 = (var_nfabot_i / var_phitdinv);
        let assign19640_e28155: f64 = (var_ndibot_i / var_pnn0);
        let assign19640_e28156: f64 = (assign19640_e28155).ln();
        let assign19640_e28157: f64 = (assign19640_e28152 * assign19640_e28156);
        (assign19640_e28157,)
    } else {
        (var_vha1,)
    }
};
        var_vha1 = assign19640_e28159;
        var_vha1_rv = 0.0;

        let assign19650_e28162: f64 = if var_nfabot_i < p.p85 { 1.0 } else { 0.0 };
        var_guard320 = assign19650_e28162;
        var_guard320_rv = 0.0;

        let (assign19660_e28179, assign19660_e28179_d_n0, assign19660_e28179_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard320 != 0.0)) {
        let assign19660_e28174: f64 = (var_vmax - var_vha1);
        let assign19660_e28175: f64 = (p.p86 * assign19660_e28174);
        let assign19660_e28177: f64 = (assign19660_e28175 + var_nfabot_i);
        (assign19660_e28177, 0.0, 0.0,)
    } else {
        (var_nja10, var_nja10_dn0, var_nja10_dn2,)
    }
};
        var_nja10 = assign19660_e28179;
        var_nja10_dn0 = assign19660_e28179_d_n0;
        var_nja10_dn2 = assign19660_e28179_d_n2;
        var_nja10_rv = 0.0;

        let (assign19670_e28194, assign19670_e28194_d_n0, assign19670_e28194_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard320 != 0.0)) {
        let assign19670_e28191: f64 = (p.p86 * var_vha1);
        let assign19670_e28192: f64 = (var_nfabot_i - assign19670_e28191);
        (assign19670_e28192, 0.0, 0.0,)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn2,)
    }
};
        var_nj0 = assign19670_e28194;
        var_nj0_dn0 = assign19670_e28194_d_n0;
        var_nj0_dn2 = assign19670_e28194_d_n2;
        var_nj0_rv = 0.0;

        let (assign19680_e28209, assign19680_e28209_d_n0, assign19680_e28209_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard320 != 0.0)) {
        let assign19680_e28205: f64 = (p.p85 - var_nja10);
        let assign19680_e28207: f64 = (assign19680_e28205 - 0.01);
        (assign19680_e28207, (-var_nja10_dn0), (-var_nja10_dn2),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2,)
    }
};
        var_tmf1 = assign19680_e28209;
        var_tmf1_dn0 = assign19680_e28209_d_n0;
        var_tmf1_dn2 = assign19680_e28209_d_n2;
        var_tmf1_rv = 0.0;

        let (assign19690_e28224, assign19690_e28224_d_n0, assign19690_e28224_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard320 != 0.0)) {
        let assign19690_e28220: f64 = (4.0 * p.p85);
        let assign19690_e28222: f64 = (assign19690_e28220 * 0.01);
        (assign19690_e28222, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign19690_e28224;
        var_tmf2_dn0 = assign19690_e28224_d_n0;
        var_tmf2_dn2 = assign19690_e28224_d_n2;
        var_tmf2_rv = 0.0;

        let (assign19700_e28241, assign19700_e28241_d_n0, assign19700_e28241_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard320 != 0.0)) {
        let (assign19700_e28239, assign19700_e28239_d_n0, assign19700_e28239_d_n2,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
            } else {
                let assign19700_e28238: f64 = (-var_tmf2);
                (assign19700_e28238, (-var_tmf2_dn0), (-var_tmf2_dn2),)
            }
        };
        (assign19700_e28239, assign19700_e28239_d_n0, assign19700_e28239_d_n2,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign19700_e28241;
        var_tmf2_dn0 = assign19700_e28241_d_n0;
        var_tmf2_dn2 = assign19700_e28241_d_n2;
        var_tmf2_rv = 0.0;

        let (assign19710_e28257, assign19710_e28257_d_n0, assign19710_e28257_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard320 != 0.0)) {
        let assign19710_e28252: f64 = (var_tmf1 * var_tmf1);
        let assign19710_e28254: f64 = (assign19710_e28252 + var_tmf2);
        let assign19710_e28255: f64 = (assign19710_e28254).sqrt();
        (assign19710_e28255, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign19710_e28255)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign19710_e28255)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign19710_e28257;
        var_tmf2_dn0 = assign19710_e28257_d_n0;
        var_tmf2_dn2 = assign19710_e28257_d_n2;
        var_tmf2_rv = 0.0;

        let (assign19720_e28274, assign19720_e28274_d_n0, assign19720_e28274_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard320 != 0.0)) {
        let assign19720_e28270: f64 = (var_tmf1 / var_tmf2);
        let assign19720_e28271: f64 = (1.0 + assign19720_e28270);
        let assign19720_e28272: f64 = (0.5 * assign19720_e28271);
        (assign19720_e28272, (0.5 * (((var_tmf1_dn0 * var_tmf2) - (var_tmf1 * var_tmf2_dn0)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn2 * var_tmf2) - (var_tmf1 * var_tmf2_dn2)) / (var_tmf2 * var_tmf2))),)
    } else {
        (var_dfn_su, var_dfn_su_dn0, var_dfn_su_dn2,)
    }
};
        var_dfn_su = assign19720_e28274;
        var_dfn_su_dn0 = assign19720_e28274_d_n0;
        var_dfn_su_dn2 = assign19720_e28274_d_n2;
        var_dfn_su_rv = 0.0;

        let (assign19730_e28291, assign19730_e28291_d_n0, assign19730_e28291_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard320 != 0.0)) {
        let assign19730_e28287: f64 = (var_tmf1 + var_tmf2);
        let assign19730_e28288: f64 = (0.5 * assign19730_e28287);
        let assign19730_e28289: f64 = (p.p85 - assign19730_e28288);
        (assign19730_e28289, (-(0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (-(0.5 * (var_tmf1_dn2 + var_tmf2_dn2))),)
    } else {
        (var_nja11, var_nja11_dn0, var_nja11_dn2,)
    }
};
        var_nja11 = assign19730_e28291;
        var_nja11_dn0 = assign19730_e28291_d_n0;
        var_nja11_dn2 = assign19730_e28291_d_n2;
        var_nja11_rv = 0.0;

        let (assign19740_e28306, assign19740_e28306_d_n0, assign19740_e28306_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard320 != 0.0)) {
        let assign19740_e28302: f64 = (var_nja11 - var_nfabot_i);
        let assign19740_e28304: f64 = (assign19740_e28302 - 0.01);
        (assign19740_e28304, var_nja11_dn0, var_nja11_dn2,)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2,)
    }
};
        var_tmf1 = assign19740_e28306;
        var_tmf1_dn0 = assign19740_e28306_d_n0;
        var_tmf1_dn2 = assign19740_e28306_d_n2;
        var_tmf1_rv = 0.0;

        let (assign19750_e28321, assign19750_e28321_d_n0, assign19750_e28321_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard320 != 0.0)) {
        let assign19750_e28317: f64 = (4.0 * var_nfabot_i);
        let assign19750_e28319: f64 = (assign19750_e28317 * 0.01);
        (assign19750_e28319, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign19750_e28321;
        var_tmf2_dn0 = assign19750_e28321_d_n0;
        var_tmf2_dn2 = assign19750_e28321_d_n2;
        var_tmf2_rv = 0.0;

        *var_dfn_su_slot = var_dfn_su;
        *var_dfn_su_dn0_slot = var_dfn_su_dn0;
        *var_dfn_su_dn2_slot = var_dfn_su_dn2;
        *var_dfn_su_rv_slot = var_dfn_su_rv;
        *var_guard320_slot = var_guard320;
        *var_guard320_rv_slot = var_guard320_rv;
        *var_nj0_slot = var_nj0;
        *var_nj0_dn0_slot = var_nj0_dn0;
        *var_nj0_dn2_slot = var_nj0_dn2;
        *var_nj0_rv_slot = var_nj0_rv;
        *var_nj1_slot = var_nj1;
        *var_nj1_dn0_slot = var_nj1_dn0;
        *var_nj1_dn2_slot = var_nj1_dn2;
        *var_nj1_rv_slot = var_nj1_rv;
        *var_nja10_slot = var_nja10;
        *var_nja10_dn0_slot = var_nja10_dn0;
        *var_nja10_dn2_slot = var_nja10_dn2;
        *var_nja10_rv_slot = var_nja10_rv;
        *var_nja11_slot = var_nja11;
        *var_nja11_dn0_slot = var_nja11_dn0;
        *var_nja11_dn2_slot = var_nja11_dn2;
        *var_nja11_rv_slot = var_nja11_rv;
        *var_pnn0_slot = var_pnn0;
        *var_pnn0_rv_slot = var_pnn0_rv;
        *var_tmf1_slot = var_tmf1;
        *var_tmf1_dn0_slot = var_tmf1_dn0;
        *var_tmf1_dn2_slot = var_tmf1_dn2;
        *var_tmf1_rv_slot = var_tmf1_rv;
        *var_tmf2_slot = var_tmf2;
        *var_tmf2_dn0_slot = var_tmf2_dn0;
        *var_tmf2_dn2_slot = var_tmf2_dn2;
        *var_tmf2_rv_slot = var_tmf2_rv;
        *var_vha1_slot = var_vha1;
        *var_vha1_rv_slot = var_vha1_rv;
    }

    pub(super) fn stamp_reactive_block_22(
        p: &Parameters,
        var_dfn_su: f64,
        var_dfn_su_dn0: f64,
        var_dfn_su_dn2: f64,
        var_guard307: f64,
        var_guard308: f64,
        var_guard31: f64,
        var_guard320: f64,
        var_ndisti_i: f64,
        var_nfabot_i: f64,
        var_nfasti_i: f64,
        var_nin: f64,
        var_phitdinv: f64,
        var_v4: f64,
        var_vmax: f64,
        var_dfn_sl_slot: &mut f64,
        var_dfn_sl_dn0_slot: &mut f64,
        var_dfn_sl_dn2_slot: &mut f64,
        var_dfn_sl_rv_slot: &mut f64,
        var_dnj1_dv_slot: &mut f64,
        var_dnj1_dv_dn0_slot: &mut f64,
        var_dnj1_dv_dn2_slot: &mut f64,
        var_dnj1_dv_rv_slot: &mut f64,
        var_dvmax_over_phitd_dv_slot: &mut f64,
        var_dvmax_over_phitd_dv_dn0_slot: &mut f64,
        var_dvmax_over_phitd_dv_dn2_slot: &mut f64,
        var_dvmax_over_phitd_dv_rv_slot: &mut f64,
        var_exp_vmax_over_phitd_bot_slot: &mut f64,
        var_exp_vmax_over_phitd_bot_dn0_slot: &mut f64,
        var_exp_vmax_over_phitd_bot_dn2_slot: &mut f64,
        var_exp_vmax_over_phitd_bot_rv_slot: &mut f64,
        var_guard321_slot: &mut f64,
        var_guard321_rv_slot: &mut f64,
        var_guard322_slot: &mut f64,
        var_guard322_rv_slot: &mut f64,
        var_guard323_slot: &mut f64,
        var_guard323_rv_slot: &mut f64,
        var_idmultbot_slot: &mut f64,
        var_idmultbot_dn0_slot: &mut f64,
        var_idmultbot_dn2_slot: &mut f64,
        var_idmultbot_rv_slot: &mut f64,
        var_nj0_slot: &mut f64,
        var_nj0_dn0_slot: &mut f64,
        var_nj0_dn2_slot: &mut f64,
        var_nj0_rv_slot: &mut f64,
        var_nj1_slot: &mut f64,
        var_nj1_dn0_slot: &mut f64,
        var_nj1_dn2_slot: &mut f64,
        var_nj1_rv_slot: &mut f64,
        var_nja10_slot: &mut f64,
        var_nja10_dn0_slot: &mut f64,
        var_nja10_dn2_slot: &mut f64,
        var_nja10_rv_slot: &mut f64,
        var_pnn0_slot: &mut f64,
        var_pnn0_rv_slot: &mut f64,
        var_tmf1_slot: &mut f64,
        var_tmf1_dn0_slot: &mut f64,
        var_tmf1_dn2_slot: &mut f64,
        var_tmf1_rv_slot: &mut f64,
        var_tmf2_slot: &mut f64,
        var_tmf2_dn0_slot: &mut f64,
        var_tmf2_dn2_slot: &mut f64,
        var_tmf2_rv_slot: &mut f64,
        var_vha1_slot: &mut f64,
        var_vha1_rv_slot: &mut f64,
    ) {
        let mut var_dfn_sl: f64 = *var_dfn_sl_slot;
        let mut var_dfn_sl_dn0: f64 = *var_dfn_sl_dn0_slot;
        let mut var_dfn_sl_dn2: f64 = *var_dfn_sl_dn2_slot;
        let mut var_dfn_sl_rv: f64 = *var_dfn_sl_rv_slot;
        let mut var_dnj1_dv: f64 = *var_dnj1_dv_slot;
        let mut var_dnj1_dv_dn0: f64 = *var_dnj1_dv_dn0_slot;
        let mut var_dnj1_dv_dn2: f64 = *var_dnj1_dv_dn2_slot;
        let mut var_dnj1_dv_rv: f64 = *var_dnj1_dv_rv_slot;
        let mut var_dvmax_over_phitd_dv: f64 = *var_dvmax_over_phitd_dv_slot;
        let mut var_dvmax_over_phitd_dv_dn0: f64 = *var_dvmax_over_phitd_dv_dn0_slot;
        let mut var_dvmax_over_phitd_dv_dn2: f64 = *var_dvmax_over_phitd_dv_dn2_slot;
        let mut var_dvmax_over_phitd_dv_rv: f64 = *var_dvmax_over_phitd_dv_rv_slot;
        let mut var_exp_vmax_over_phitd_bot: f64 = *var_exp_vmax_over_phitd_bot_slot;
        let mut var_exp_vmax_over_phitd_bot_dn0: f64 = *var_exp_vmax_over_phitd_bot_dn0_slot;
        let mut var_exp_vmax_over_phitd_bot_dn2: f64 = *var_exp_vmax_over_phitd_bot_dn2_slot;
        let mut var_exp_vmax_over_phitd_bot_rv: f64 = *var_exp_vmax_over_phitd_bot_rv_slot;
        let mut var_guard321: f64 = *var_guard321_slot;
        let mut var_guard321_rv: f64 = *var_guard321_rv_slot;
        let mut var_guard322: f64 = *var_guard322_slot;
        let mut var_guard322_rv: f64 = *var_guard322_rv_slot;
        let mut var_guard323: f64 = *var_guard323_slot;
        let mut var_guard323_rv: f64 = *var_guard323_rv_slot;
        let mut var_idmultbot: f64 = *var_idmultbot_slot;
        let mut var_idmultbot_dn0: f64 = *var_idmultbot_dn0_slot;
        let mut var_idmultbot_dn2: f64 = *var_idmultbot_dn2_slot;
        let mut var_idmultbot_rv: f64 = *var_idmultbot_rv_slot;
        let mut var_nj0: f64 = *var_nj0_slot;
        let mut var_nj0_dn0: f64 = *var_nj0_dn0_slot;
        let mut var_nj0_dn2: f64 = *var_nj0_dn2_slot;
        let mut var_nj0_rv: f64 = *var_nj0_rv_slot;
        let mut var_nj1: f64 = *var_nj1_slot;
        let mut var_nj1_dn0: f64 = *var_nj1_dn0_slot;
        let mut var_nj1_dn2: f64 = *var_nj1_dn2_slot;
        let mut var_nj1_rv: f64 = *var_nj1_rv_slot;
        let mut var_nja10: f64 = *var_nja10_slot;
        let mut var_nja10_dn0: f64 = *var_nja10_dn0_slot;
        let mut var_nja10_dn2: f64 = *var_nja10_dn2_slot;
        let mut var_nja10_rv: f64 = *var_nja10_rv_slot;
        let mut var_pnn0: f64 = *var_pnn0_slot;
        let mut var_pnn0_rv: f64 = *var_pnn0_rv_slot;
        let mut var_tmf1: f64 = *var_tmf1_slot;
        let mut var_tmf1_dn0: f64 = *var_tmf1_dn0_slot;
        let mut var_tmf1_dn2: f64 = *var_tmf1_dn2_slot;
        let mut var_tmf1_rv: f64 = *var_tmf1_rv_slot;
        let mut var_tmf2: f64 = *var_tmf2_slot;
        let mut var_tmf2_dn0: f64 = *var_tmf2_dn0_slot;
        let mut var_tmf2_dn2: f64 = *var_tmf2_dn2_slot;
        let mut var_tmf2_rv: f64 = *var_tmf2_rv_slot;
        let mut var_vha1: f64 = *var_vha1_slot;
        let mut var_vha1_rv: f64 = *var_vha1_rv_slot;

        let (assign19760_e28338, assign19760_e28338_d_n0, assign19760_e28338_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard320 != 0.0)) {
        let (assign19760_e28336, assign19760_e28336_d_n0, assign19760_e28336_d_n2,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
            } else {
                let assign19760_e28335: f64 = (-var_tmf2);
                (assign19760_e28335, (-var_tmf2_dn0), (-var_tmf2_dn2),)
            }
        };
        (assign19760_e28336, assign19760_e28336_d_n0, assign19760_e28336_d_n2,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign19760_e28338;
        var_tmf2_dn0 = assign19760_e28338_d_n0;
        var_tmf2_dn2 = assign19760_e28338_d_n2;
        var_tmf2_rv = 0.0;

        let (assign19770_e28354, assign19770_e28354_d_n0, assign19770_e28354_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard320 != 0.0)) {
        let assign19770_e28349: f64 = (var_tmf1 * var_tmf1);
        let assign19770_e28351: f64 = (assign19770_e28349 + var_tmf2);
        let assign19770_e28352: f64 = (assign19770_e28351).sqrt();
        (assign19770_e28352, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign19770_e28352)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign19770_e28352)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign19770_e28354;
        var_tmf2_dn0 = assign19770_e28354_d_n0;
        var_tmf2_dn2 = assign19770_e28354_d_n2;
        var_tmf2_rv = 0.0;

        let (assign19780_e28371, assign19780_e28371_d_n0, assign19780_e28371_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard320 != 0.0)) {
        let assign19780_e28367: f64 = (var_tmf1 / var_tmf2);
        let assign19780_e28368: f64 = (1.0 + assign19780_e28367);
        let assign19780_e28369: f64 = (0.5 * assign19780_e28368);
        (assign19780_e28369, (0.5 * (((var_tmf1_dn0 * var_tmf2) - (var_tmf1 * var_tmf2_dn0)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn2 * var_tmf2) - (var_tmf1 * var_tmf2_dn2)) / (var_tmf2 * var_tmf2))),)
    } else {
        (var_dfn_sl, var_dfn_sl_dn0, var_dfn_sl_dn2,)
    }
};
        var_dfn_sl = assign19780_e28371;
        var_dfn_sl_dn0 = assign19780_e28371_d_n0;
        var_dfn_sl_dn2 = assign19780_e28371_d_n2;
        var_dfn_sl_rv = 0.0;

        let (assign19790_e28388, assign19790_e28388_d_n0, assign19790_e28388_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard320 != 0.0)) {
        let assign19790_e28384: f64 = (var_tmf1 + var_tmf2);
        let assign19790_e28385: f64 = (0.5 * assign19790_e28384);
        let assign19790_e28386: f64 = (var_nfabot_i + assign19790_e28385);
        (assign19790_e28386, (0.5 * (var_tmf1_dn0 + var_tmf2_dn0)), (0.5 * (var_tmf1_dn2 + var_tmf2_dn2)),)
    } else {
        (var_nj1, var_nj1_dn0, var_nj1_dn2,)
    }
};
        var_nj1 = assign19790_e28388;
        var_nj1_dn0 = assign19790_e28388_d_n0;
        var_nj1_dn2 = assign19790_e28388_d_n2;
        var_nj1_rv = 0.0;

        let (assign19800_e28403, assign19800_e28403_d_n0, assign19800_e28403_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard320 != 0.0)) {
        let assign19800_e28399: f64 = (p.p85 - var_nj0);
        let assign19800_e28401: f64 = (assign19800_e28399 - 0.01);
        (assign19800_e28401, (-var_nj0_dn0), (-var_nj0_dn2),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2,)
    }
};
        var_tmf1 = assign19800_e28403;
        var_tmf1_dn0 = assign19800_e28403_d_n0;
        var_tmf1_dn2 = assign19800_e28403_d_n2;
        var_tmf1_rv = 0.0;

        let (assign19810_e28418, assign19810_e28418_d_n0, assign19810_e28418_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard320 != 0.0)) {
        let assign19810_e28414: f64 = (4.0 * p.p85);
        let assign19810_e28416: f64 = (assign19810_e28414 * 0.01);
        (assign19810_e28416, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign19810_e28418;
        var_tmf2_dn0 = assign19810_e28418_d_n0;
        var_tmf2_dn2 = assign19810_e28418_d_n2;
        var_tmf2_rv = 0.0;

        let (assign19820_e28435, assign19820_e28435_d_n0, assign19820_e28435_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard320 != 0.0)) {
        let (assign19820_e28433, assign19820_e28433_d_n0, assign19820_e28433_d_n2,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
            } else {
                let assign19820_e28432: f64 = (-var_tmf2);
                (assign19820_e28432, (-var_tmf2_dn0), (-var_tmf2_dn2),)
            }
        };
        (assign19820_e28433, assign19820_e28433_d_n0, assign19820_e28433_d_n2,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign19820_e28435;
        var_tmf2_dn0 = assign19820_e28435_d_n0;
        var_tmf2_dn2 = assign19820_e28435_d_n2;
        var_tmf2_rv = 0.0;

        let (assign19830_e28451, assign19830_e28451_d_n0, assign19830_e28451_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard320 != 0.0)) {
        let assign19830_e28446: f64 = (var_tmf1 * var_tmf1);
        let assign19830_e28448: f64 = (assign19830_e28446 + var_tmf2);
        let assign19830_e28449: f64 = (assign19830_e28448).sqrt();
        (assign19830_e28449, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign19830_e28449)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign19830_e28449)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign19830_e28451;
        var_tmf2_dn0 = assign19830_e28451_d_n0;
        var_tmf2_dn2 = assign19830_e28451_d_n2;
        var_tmf2_rv = 0.0;

        let (assign19840_e28468, assign19840_e28468_d_n0, assign19840_e28468_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard320 != 0.0)) {
        let assign19840_e28464: f64 = (var_tmf1 + var_tmf2);
        let assign19840_e28465: f64 = (0.5 * assign19840_e28464);
        let assign19840_e28466: f64 = (p.p85 - assign19840_e28465);
        (assign19840_e28466, (-(0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (-(0.5 * (var_tmf1_dn2 + var_tmf2_dn2))),)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn2,)
    }
};
        var_nj0 = assign19840_e28468;
        var_nj0_dn0 = assign19840_e28468_d_n0;
        var_nj0_dn2 = assign19840_e28468_d_n2;
        var_nj0_rv = 0.0;

        let (assign19850_e28483, assign19850_e28483_d_n0, assign19850_e28483_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard320 != 0.0)) {
        let assign19850_e28479: f64 = (var_nj0 - var_nfabot_i);
        let assign19850_e28481: f64 = (assign19850_e28479 - 0.01);
        (assign19850_e28481, var_nj0_dn0, var_nj0_dn2,)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2,)
    }
};
        var_tmf1 = assign19850_e28483;
        var_tmf1_dn0 = assign19850_e28483_d_n0;
        var_tmf1_dn2 = assign19850_e28483_d_n2;
        var_tmf1_rv = 0.0;

        let (assign19860_e28498, assign19860_e28498_d_n0, assign19860_e28498_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard320 != 0.0)) {
        let assign19860_e28494: f64 = (4.0 * var_nfabot_i);
        let assign19860_e28496: f64 = (assign19860_e28494 * 0.01);
        (assign19860_e28496, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign19860_e28498;
        var_tmf2_dn0 = assign19860_e28498_d_n0;
        var_tmf2_dn2 = assign19860_e28498_d_n2;
        var_tmf2_rv = 0.0;

        let (assign19870_e28515, assign19870_e28515_d_n0, assign19870_e28515_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard320 != 0.0)) {
        let (assign19870_e28513, assign19870_e28513_d_n0, assign19870_e28513_d_n2,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
            } else {
                let assign19870_e28512: f64 = (-var_tmf2);
                (assign19870_e28512, (-var_tmf2_dn0), (-var_tmf2_dn2),)
            }
        };
        (assign19870_e28513, assign19870_e28513_d_n0, assign19870_e28513_d_n2,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign19870_e28515;
        var_tmf2_dn0 = assign19870_e28515_d_n0;
        var_tmf2_dn2 = assign19870_e28515_d_n2;
        var_tmf2_rv = 0.0;

        let (assign19880_e28531, assign19880_e28531_d_n0, assign19880_e28531_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard320 != 0.0)) {
        let assign19880_e28526: f64 = (var_tmf1 * var_tmf1);
        let assign19880_e28528: f64 = (assign19880_e28526 + var_tmf2);
        let assign19880_e28529: f64 = (assign19880_e28528).sqrt();
        (assign19880_e28529, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign19880_e28529)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign19880_e28529)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign19880_e28531;
        var_tmf2_dn0 = assign19880_e28531_d_n0;
        var_tmf2_dn2 = assign19880_e28531_d_n2;
        var_tmf2_rv = 0.0;

        let (assign19890_e28548, assign19890_e28548_d_n0, assign19890_e28548_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard320 != 0.0)) {
        let assign19890_e28544: f64 = (var_tmf1 + var_tmf2);
        let assign19890_e28545: f64 = (0.5 * assign19890_e28544);
        let assign19890_e28546: f64 = (var_nfabot_i + assign19890_e28545);
        (assign19890_e28546, (0.5 * (var_tmf1_dn0 + var_tmf2_dn0)), (0.5 * (var_tmf1_dn2 + var_tmf2_dn2)),)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn2,)
    }
};
        var_nj0 = assign19890_e28548;
        var_nj0_dn0 = assign19890_e28548_d_n0;
        var_nj0_dn2 = assign19890_e28548_d_n2;
        var_nj0_rv = 0.0;

        let (assign19900_e28563, assign19900_e28563_d_n0, assign19900_e28563_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard320 != 0.0)) {
        let assign19900_e28559: f64 = (p.p86 * var_dfn_su);
        let assign19900_e28561: f64 = (assign19900_e28559 * var_dfn_sl);
        (assign19900_e28561, (((p.p86 * var_dfn_su_dn0) * var_dfn_sl) + (assign19900_e28559 * var_dfn_sl_dn0)), (((p.p86 * var_dfn_su_dn2) * var_dfn_sl) + (assign19900_e28559 * var_dfn_sl_dn2)),)
    } else {
        (var_dnj1_dv, var_dnj1_dv_dn0, var_dnj1_dv_dn2,)
    }
};
        var_dnj1_dv = assign19900_e28563;
        var_dnj1_dv_dn0 = assign19900_e28563_d_n0;
        var_dnj1_dv_dn2 = assign19900_e28563_d_n2;
        var_dnj1_dv_rv = 0.0;

        let (assign19910_e28575, assign19910_e28575_d_n0, assign19910_e28575_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard320 == 0.0)) {
        (var_nfabot_i, 0.0, 0.0,)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn2,)
    }
};
        var_nj0 = assign19910_e28575;
        var_nj0_dn0 = assign19910_e28575_d_n0;
        var_nj0_dn2 = assign19910_e28575_d_n2;
        var_nj0_rv = 0.0;

        let (assign19920_e28587, assign19920_e28587_d_n0, assign19920_e28587_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard320 == 0.0)) {
        (var_nfabot_i, 0.0, 0.0,)
    } else {
        (var_nj1, var_nj1_dn0, var_nj1_dn2,)
    }
};
        var_nj1 = assign19920_e28587;
        var_nj1_dn0 = assign19920_e28587_d_n0;
        var_nj1_dn2 = assign19920_e28587_d_n2;
        var_nj1_rv = 0.0;

        let (assign19930_e28599, assign19930_e28599_d_n0, assign19930_e28599_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard320 == 0.0)) {
        (0.0, 0.0, 0.0,)
    } else {
        (var_dnj1_dv, var_dnj1_dv_dn0, var_dnj1_dv_dn2,)
    }
};
        var_dnj1_dv = assign19930_e28599;
        var_dnj1_dv_dn0 = assign19930_e28599_d_n0;
        var_dnj1_dv_dn2 = assign19930_e28599_d_n2;
        var_dnj1_dv_rv = 0.0;

        let assign19940_e28603: f64 = (var_vmax / var_nj1);
        let assign19940_e28607: f64 = (var_nj1 - var_nj0);
        let assign19940_e28608: f64 = (var_vha1 * assign19940_e28607);
        let assign19940_e28611: f64 = (var_nj0 * p.p85);
        let assign19940_e28612: f64 = (assign19940_e28608 / assign19940_e28611);
        let assign19940_e28613: f64 = (assign19940_e28603 + assign19940_e28612);
        let assign19940_e28614: f64 = (var_phitdinv * assign19940_e28613);
        let assign19940_e28615: f64 = (assign19940_e28614).abs();
        let assign19940_e28617: f64 = if assign19940_e28615 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard321 = assign19940_e28617;
        var_guard321_rv = 0.0;

        let (assign19950_e28643, assign19950_e28643_d_n0, assign19950_e28643_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard321 != 0.0)) {
        let assign19950_e28629: f64 = (var_vmax / var_nj1);
        let assign19950_e28633: f64 = (var_nj1 - var_nj0);
        let assign19950_e28634: f64 = (var_vha1 * assign19950_e28633);
        let assign19950_e28637: f64 = (var_nj0 * p.p85);
        let assign19950_e28638: f64 = (assign19950_e28634 / assign19950_e28637);
        let assign19950_e28639: f64 = (assign19950_e28629 + assign19950_e28638);
        let assign19950_e28640: f64 = (var_phitdinv * assign19950_e28639);
        let assign19950_e28641: f64 = (assign19950_e28640).exp();
        (assign19950_e28641, (assign19950_e28641 * (var_phitdinv * ((-((var_vmax * var_nj1_dn0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn0 - var_nj0_dn0)) * assign19950_e28637) - (assign19950_e28634 * (var_nj0_dn0 * p.p85))) / (assign19950_e28637 * assign19950_e28637))))), (assign19950_e28641 * (var_phitdinv * ((-((var_vmax * var_nj1_dn2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn2 - var_nj0_dn2)) * assign19950_e28637) - (assign19950_e28634 * (var_nj0_dn2 * p.p85))) / (assign19950_e28637 * assign19950_e28637))))),)
    } else {
        (var_exp_vmax_over_phitd_bot, var_exp_vmax_over_phitd_bot_dn0, var_exp_vmax_over_phitd_bot_dn2,)
    }
};
        var_exp_vmax_over_phitd_bot = assign19950_e28643;
        var_exp_vmax_over_phitd_bot_dn0 = assign19950_e28643_d_n0;
        var_exp_vmax_over_phitd_bot_dn2 = assign19950_e28643_d_n2;
        var_exp_vmax_over_phitd_bot_rv = 0.0;

        let assign19960_e28647: f64 = (var_vmax / var_nj1);
        let assign19960_e28651: f64 = (var_nj1 - var_nj0);
        let assign19960_e28652: f64 = (var_vha1 * assign19960_e28651);
        let assign19960_e28655: f64 = (var_nj0 * p.p85);
        let assign19960_e28656: f64 = (assign19960_e28652 / assign19960_e28655);
        let assign19960_e28657: f64 = (assign19960_e28647 + assign19960_e28656);
        let assign19960_e28658: f64 = (var_phitdinv * assign19960_e28657);
        let assign19960_e28660: f64 = (-230.25850929940458);
        let assign19960_e28661: f64 = if assign19960_e28658 < assign19960_e28660 { 1.0 } else { 0.0 };
        var_guard322 = assign19960_e28661;
        var_guard322_rv = 0.0;

        let (assign19970_e28742, assign19970_e28742_d_n0, assign19970_e28742_d_n2,) = {
    if (((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard321 == 0.0)) && (var_guard322 != 0.0)) {
        let assign19970_e28676: f64 = (-230.25850929940458);
        let assign19970_e28680: f64 = (var_vmax / var_nj1);
        let assign19970_e28684: f64 = (var_nj1 - var_nj0);
        let assign19970_e28685: f64 = (var_vha1 * assign19970_e28684);
        let assign19970_e28688: f64 = (var_nj0 * p.p85);
        let assign19970_e28689: f64 = (assign19970_e28685 / assign19970_e28688);
        let assign19970_e28690: f64 = (assign19970_e28680 + assign19970_e28689);
        let assign19970_e28691: f64 = (var_phitdinv * assign19970_e28690);
        let assign19970_e28692: f64 = (assign19970_e28676 - assign19970_e28691);
        let assign19970_e28696: f64 = (-230.25850929940458);
        let assign19970_e28700: f64 = (var_vmax / var_nj1);
        let assign19970_e28704: f64 = (var_nj1 - var_nj0);
        let assign19970_e28705: f64 = (var_vha1 * assign19970_e28704);
        let assign19970_e28708: f64 = (var_nj0 * p.p85);
        let assign19970_e28709: f64 = (assign19970_e28705 / assign19970_e28708);
        let assign19970_e28710: f64 = (assign19970_e28700 + assign19970_e28709);
        let assign19970_e28711: f64 = (var_phitdinv * assign19970_e28710);
        let assign19970_e28712: f64 = (assign19970_e28696 - assign19970_e28711);
        let assign19970_e28715: f64 = (-230.25850929940458);
        let assign19970_e28719: f64 = (var_vmax / var_nj1);
        let assign19970_e28723: f64 = (var_nj1 - var_nj0);
        let assign19970_e28724: f64 = (var_vha1 * assign19970_e28723);
        let assign19970_e28727: f64 = (var_nj0 * p.p85);
        let assign19970_e28728: f64 = (assign19970_e28724 / assign19970_e28727);
        let assign19970_e28729: f64 = (assign19970_e28719 + assign19970_e28728);
        let assign19970_e28730: f64 = (var_phitdinv * assign19970_e28729);
        let assign19970_e28731: f64 = (assign19970_e28715 - assign19970_e28730);
        let assign19970_e28733: f64 = (assign19970_e28731 * 0.3333333333333333);
        let assign19970_e28734: f64 = (1.0 + assign19970_e28733);
        let assign19970_e28735: f64 = (assign19970_e28712 * assign19970_e28734);
        let assign19970_e28736: f64 = (0.5 * assign19970_e28735);
        let assign19970_e28737: f64 = (1.0 + assign19970_e28736);
        let assign19970_e28738: f64 = (assign19970_e28692 * assign19970_e28737);
        let assign19970_e28739: f64 = (1.0 + assign19970_e28738);
        let assign19970_e28740: f64 = (1e-100 / assign19970_e28739);
        (assign19970_e28740, (-((1e-100 * (((-(var_phitdinv * ((-((var_vmax * var_nj1_dn0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn0 - var_nj0_dn0)) * assign19970_e28688) - (assign19970_e28685 * (var_nj0_dn0 * p.p85))) / (assign19970_e28688 * assign19970_e28688))))) * assign19970_e28737) + (assign19970_e28692 * (0.5 * (((-(var_phitdinv * ((-((var_vmax * var_nj1_dn0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn0 - var_nj0_dn0)) * assign19970_e28708) - (assign19970_e28705 * (var_nj0_dn0 * p.p85))) / (assign19970_e28708 * assign19970_e28708))))) * assign19970_e28734) + (assign19970_e28712 * ((-(var_phitdinv * ((-((var_vmax * var_nj1_dn0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn0 - var_nj0_dn0)) * assign19970_e28727) - (assign19970_e28724 * (var_nj0_dn0 * p.p85))) / (assign19970_e28727 * assign19970_e28727))))) * 0.3333333333333333))))))) / (assign19970_e28739 * assign19970_e28739))), (-((1e-100 * (((-(var_phitdinv * ((-((var_vmax * var_nj1_dn2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn2 - var_nj0_dn2)) * assign19970_e28688) - (assign19970_e28685 * (var_nj0_dn2 * p.p85))) / (assign19970_e28688 * assign19970_e28688))))) * assign19970_e28737) + (assign19970_e28692 * (0.5 * (((-(var_phitdinv * ((-((var_vmax * var_nj1_dn2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn2 - var_nj0_dn2)) * assign19970_e28708) - (assign19970_e28705 * (var_nj0_dn2 * p.p85))) / (assign19970_e28708 * assign19970_e28708))))) * assign19970_e28734) + (assign19970_e28712 * ((-(var_phitdinv * ((-((var_vmax * var_nj1_dn2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn2 - var_nj0_dn2)) * assign19970_e28727) - (assign19970_e28724 * (var_nj0_dn2 * p.p85))) / (assign19970_e28727 * assign19970_e28727))))) * 0.3333333333333333))))))) / (assign19970_e28739 * assign19970_e28739))),)
    } else {
        (var_exp_vmax_over_phitd_bot, var_exp_vmax_over_phitd_bot_dn0, var_exp_vmax_over_phitd_bot_dn2,)
    }
};
        var_exp_vmax_over_phitd_bot = assign19970_e28742;
        var_exp_vmax_over_phitd_bot_dn0 = assign19970_e28742_d_n0;
        var_exp_vmax_over_phitd_bot_dn2 = assign19970_e28742_d_n2;
        var_exp_vmax_over_phitd_bot_rv = 0.0;

        let (assign19980_e28821, assign19980_e28821_d_n0, assign19980_e28821_d_n2,) = {
    if (((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard321 == 0.0)) && (var_guard322 == 0.0)) {
        let assign19980_e28760: f64 = (var_vmax / var_nj1);
        let assign19980_e28764: f64 = (var_nj1 - var_nj0);
        let assign19980_e28765: f64 = (var_vha1 * assign19980_e28764);
        let assign19980_e28768: f64 = (var_nj0 * p.p85);
        let assign19980_e28769: f64 = (assign19980_e28765 / assign19980_e28768);
        let assign19980_e28770: f64 = (assign19980_e28760 + assign19980_e28769);
        let assign19980_e28771: f64 = (var_phitdinv * assign19980_e28770);
        let assign19980_e28773: f64 = (assign19980_e28771 - 230.25850929940458);
        let assign19980_e28779: f64 = (var_vmax / var_nj1);
        let assign19980_e28783: f64 = (var_nj1 - var_nj0);
        let assign19980_e28784: f64 = (var_vha1 * assign19980_e28783);
        let assign19980_e28787: f64 = (var_nj0 * p.p85);
        let assign19980_e28788: f64 = (assign19980_e28784 / assign19980_e28787);
        let assign19980_e28789: f64 = (assign19980_e28779 + assign19980_e28788);
        let assign19980_e28790: f64 = (var_phitdinv * assign19980_e28789);
        let assign19980_e28792: f64 = (assign19980_e28790 - 230.25850929940458);
        let assign19980_e28797: f64 = (var_vmax / var_nj1);
        let assign19980_e28801: f64 = (var_nj1 - var_nj0);
        let assign19980_e28802: f64 = (var_vha1 * assign19980_e28801);
        let assign19980_e28805: f64 = (var_nj0 * p.p85);
        let assign19980_e28806: f64 = (assign19980_e28802 / assign19980_e28805);
        let assign19980_e28807: f64 = (assign19980_e28797 + assign19980_e28806);
        let assign19980_e28808: f64 = (var_phitdinv * assign19980_e28807);
        let assign19980_e28810: f64 = (assign19980_e28808 - 230.25850929940458);
        let assign19980_e28812: f64 = (assign19980_e28810 * 0.3333333333333333);
        let assign19980_e28813: f64 = (1.0 + assign19980_e28812);
        let assign19980_e28814: f64 = (assign19980_e28792 * assign19980_e28813);
        let assign19980_e28815: f64 = (0.5 * assign19980_e28814);
        let assign19980_e28816: f64 = (1.0 + assign19980_e28815);
        let assign19980_e28817: f64 = (assign19980_e28773 * assign19980_e28816);
        let assign19980_e28818: f64 = (1.0 + assign19980_e28817);
        let assign19980_e28819: f64 = (1e100 * assign19980_e28818);
        (assign19980_e28819, (1e100 * (((var_phitdinv * ((-((var_vmax * var_nj1_dn0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn0 - var_nj0_dn0)) * assign19980_e28768) - (assign19980_e28765 * (var_nj0_dn0 * p.p85))) / (assign19980_e28768 * assign19980_e28768)))) * assign19980_e28816) + (assign19980_e28773 * (0.5 * (((var_phitdinv * ((-((var_vmax * var_nj1_dn0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn0 - var_nj0_dn0)) * assign19980_e28787) - (assign19980_e28784 * (var_nj0_dn0 * p.p85))) / (assign19980_e28787 * assign19980_e28787)))) * assign19980_e28813) + (assign19980_e28792 * ((var_phitdinv * ((-((var_vmax * var_nj1_dn0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn0 - var_nj0_dn0)) * assign19980_e28805) - (assign19980_e28802 * (var_nj0_dn0 * p.p85))) / (assign19980_e28805 * assign19980_e28805)))) * 0.3333333333333333))))))), (1e100 * (((var_phitdinv * ((-((var_vmax * var_nj1_dn2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn2 - var_nj0_dn2)) * assign19980_e28768) - (assign19980_e28765 * (var_nj0_dn2 * p.p85))) / (assign19980_e28768 * assign19980_e28768)))) * assign19980_e28816) + (assign19980_e28773 * (0.5 * (((var_phitdinv * ((-((var_vmax * var_nj1_dn2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn2 - var_nj0_dn2)) * assign19980_e28787) - (assign19980_e28784 * (var_nj0_dn2 * p.p85))) / (assign19980_e28787 * assign19980_e28787)))) * assign19980_e28813) + (assign19980_e28792 * ((var_phitdinv * ((-((var_vmax * var_nj1_dn2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn2 - var_nj0_dn2)) * assign19980_e28805) - (assign19980_e28802 * (var_nj0_dn2 * p.p85))) / (assign19980_e28805 * assign19980_e28805)))) * 0.3333333333333333))))))),)
    } else {
        (var_exp_vmax_over_phitd_bot, var_exp_vmax_over_phitd_bot_dn0, var_exp_vmax_over_phitd_bot_dn2,)
    }
};
        var_exp_vmax_over_phitd_bot = assign19980_e28821;
        var_exp_vmax_over_phitd_bot_dn0 = assign19980_e28821_d_n0;
        var_exp_vmax_over_phitd_bot_dn2 = assign19980_e28821_d_n2;
        var_exp_vmax_over_phitd_bot_rv = 0.0;

        let (assign19990_e28848, assign19990_e28848_d_n0, assign19990_e28848_d_n2,) = {
    if (((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) {
        let assign19990_e28832: f64 = (var_vmax * var_dnj1_dv);
        let assign19990_e28833: f64 = (var_nj1 - assign19990_e28832);
        let assign19990_e28836: f64 = (var_nj1 * var_nj1);
        let assign19990_e28837: f64 = (assign19990_e28833 / assign19990_e28836);
        let assign19990_e28840: f64 = (var_vha1 * var_dnj1_dv);
        let assign19990_e28843: f64 = (var_nj0 * p.p85);
        let assign19990_e28844: f64 = (assign19990_e28840 / assign19990_e28843);
        let assign19990_e28845: f64 = (assign19990_e28837 + assign19990_e28844);
        let assign19990_e28846: f64 = (var_phitdinv * assign19990_e28845);
        (assign19990_e28846, (var_phitdinv * (((((var_nj1_dn0 - (var_vmax * var_dnj1_dv_dn0)) * assign19990_e28836) - (assign19990_e28833 * ((var_nj1_dn0 * var_nj1) + (var_nj1 * var_nj1_dn0)))) / (assign19990_e28836 * assign19990_e28836)) + ((((var_vha1 * var_dnj1_dv_dn0) * assign19990_e28843) - (assign19990_e28840 * (var_nj0_dn0 * p.p85))) / (assign19990_e28843 * assign19990_e28843)))), (var_phitdinv * (((((var_nj1_dn2 - (var_vmax * var_dnj1_dv_dn2)) * assign19990_e28836) - (assign19990_e28833 * ((var_nj1_dn2 * var_nj1) + (var_nj1 * var_nj1_dn2)))) / (assign19990_e28836 * assign19990_e28836)) + ((((var_vha1 * var_dnj1_dv_dn2) * assign19990_e28843) - (assign19990_e28840 * (var_nj0_dn2 * p.p85))) / (assign19990_e28843 * assign19990_e28843)))),)
    } else {
        (var_dvmax_over_phitd_dv, var_dvmax_over_phitd_dv_dn0, var_dvmax_over_phitd_dv_dn2,)
    }
};
        var_dvmax_over_phitd_dv = assign19990_e28848;
        var_dvmax_over_phitd_dv_dn0 = assign19990_e28848_d_n0;
        var_dvmax_over_phitd_dv_dn2 = assign19990_e28848_d_n2;
        var_dvmax_over_phitd_dv_rv = 0.0;

        let (assign20000_e28865, assign20000_e28865_d_n0, assign20000_e28865_d_n2,) = {
    if (((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) {
        let assign20000_e28858: f64 = (var_v4 - var_vmax);
        let assign20000_e28860: f64 = (assign20000_e28858 * var_dvmax_over_phitd_dv);
        let assign20000_e28861: f64 = (1.0 + assign20000_e28860);
        let assign20000_e28863: f64 = (assign20000_e28861 * var_exp_vmax_over_phitd_bot);
        (assign20000_e28863, (((assign20000_e28858 * var_dvmax_over_phitd_dv_dn0) * var_exp_vmax_over_phitd_bot) + (assign20000_e28861 * var_exp_vmax_over_phitd_bot_dn0)), (((assign20000_e28858 * var_dvmax_over_phitd_dv_dn2) * var_exp_vmax_over_phitd_bot) + (assign20000_e28861 * var_exp_vmax_over_phitd_bot_dn2)),)
    } else {
        (var_idmultbot, var_idmultbot_dn0, var_idmultbot_dn2,)
    }
};
        var_idmultbot = assign20000_e28865;
        var_idmultbot_dn0 = assign20000_e28865_d_n0;
        var_idmultbot_dn2 = assign20000_e28865_d_n2;
        var_idmultbot_rv = 0.0;

        let (assign20010_e28878,) = {
    if (((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) {
        let assign20010_e28874: f64 = (var_nin * var_nin);
        let assign20010_e28876: f64 = (assign20010_e28874 / var_ndisti_i);
        (assign20010_e28876,)
    } else {
        (var_pnn0,)
    }
};
        var_pnn0 = assign20010_e28878;
        var_pnn0_rv = 0.0;

        let (assign20020_e28894,) = {
    if (((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) {
        let assign20020_e28887: f64 = (var_nfasti_i / var_phitdinv);
        let assign20020_e28890: f64 = (var_ndisti_i / var_pnn0);
        let assign20020_e28891: f64 = (assign20020_e28890).ln();
        let assign20020_e28892: f64 = (assign20020_e28887 * assign20020_e28891);
        (assign20020_e28892,)
    } else {
        (var_vha1,)
    }
};
        var_vha1 = assign20020_e28894;
        var_vha1_rv = 0.0;

        let assign20030_e28897: f64 = if var_nfasti_i < p.p85 { 1.0 } else { 0.0 };
        var_guard323 = assign20030_e28897;
        var_guard323_rv = 0.0;

        let (assign20040_e28914, assign20040_e28914_d_n0, assign20040_e28914_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard323 != 0.0)) {
        let assign20040_e28909: f64 = (var_vmax - var_vha1);
        let assign20040_e28910: f64 = (p.p86 * assign20040_e28909);
        let assign20040_e28912: f64 = (assign20040_e28910 + var_nfasti_i);
        (assign20040_e28912, 0.0, 0.0,)
    } else {
        (var_nja10, var_nja10_dn0, var_nja10_dn2,)
    }
};
        var_nja10 = assign20040_e28914;
        var_nja10_dn0 = assign20040_e28914_d_n0;
        var_nja10_dn2 = assign20040_e28914_d_n2;
        var_nja10_rv = 0.0;

        let (assign20050_e28929, assign20050_e28929_d_n0, assign20050_e28929_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard323 != 0.0)) {
        let assign20050_e28926: f64 = (p.p86 * var_vha1);
        let assign20050_e28927: f64 = (var_nfasti_i - assign20050_e28926);
        (assign20050_e28927, 0.0, 0.0,)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn2,)
    }
};
        var_nj0 = assign20050_e28929;
        var_nj0_dn0 = assign20050_e28929_d_n0;
        var_nj0_dn2 = assign20050_e28929_d_n2;
        var_nj0_rv = 0.0;

        *var_dfn_sl_slot = var_dfn_sl;
        *var_dfn_sl_dn0_slot = var_dfn_sl_dn0;
        *var_dfn_sl_dn2_slot = var_dfn_sl_dn2;
        *var_dfn_sl_rv_slot = var_dfn_sl_rv;
        *var_dnj1_dv_slot = var_dnj1_dv;
        *var_dnj1_dv_dn0_slot = var_dnj1_dv_dn0;
        *var_dnj1_dv_dn2_slot = var_dnj1_dv_dn2;
        *var_dnj1_dv_rv_slot = var_dnj1_dv_rv;
        *var_dvmax_over_phitd_dv_slot = var_dvmax_over_phitd_dv;
        *var_dvmax_over_phitd_dv_dn0_slot = var_dvmax_over_phitd_dv_dn0;
        *var_dvmax_over_phitd_dv_dn2_slot = var_dvmax_over_phitd_dv_dn2;
        *var_dvmax_over_phitd_dv_rv_slot = var_dvmax_over_phitd_dv_rv;
        *var_exp_vmax_over_phitd_bot_slot = var_exp_vmax_over_phitd_bot;
        *var_exp_vmax_over_phitd_bot_dn0_slot = var_exp_vmax_over_phitd_bot_dn0;
        *var_exp_vmax_over_phitd_bot_dn2_slot = var_exp_vmax_over_phitd_bot_dn2;
        *var_exp_vmax_over_phitd_bot_rv_slot = var_exp_vmax_over_phitd_bot_rv;
        *var_guard321_slot = var_guard321;
        *var_guard321_rv_slot = var_guard321_rv;
        *var_guard322_slot = var_guard322;
        *var_guard322_rv_slot = var_guard322_rv;
        *var_guard323_slot = var_guard323;
        *var_guard323_rv_slot = var_guard323_rv;
        *var_idmultbot_slot = var_idmultbot;
        *var_idmultbot_dn0_slot = var_idmultbot_dn0;
        *var_idmultbot_dn2_slot = var_idmultbot_dn2;
        *var_idmultbot_rv_slot = var_idmultbot_rv;
        *var_nj0_slot = var_nj0;
        *var_nj0_dn0_slot = var_nj0_dn0;
        *var_nj0_dn2_slot = var_nj0_dn2;
        *var_nj0_rv_slot = var_nj0_rv;
        *var_nj1_slot = var_nj1;
        *var_nj1_dn0_slot = var_nj1_dn0;
        *var_nj1_dn2_slot = var_nj1_dn2;
        *var_nj1_rv_slot = var_nj1_rv;
        *var_nja10_slot = var_nja10;
        *var_nja10_dn0_slot = var_nja10_dn0;
        *var_nja10_dn2_slot = var_nja10_dn2;
        *var_nja10_rv_slot = var_nja10_rv;
        *var_pnn0_slot = var_pnn0;
        *var_pnn0_rv_slot = var_pnn0_rv;
        *var_tmf1_slot = var_tmf1;
        *var_tmf1_dn0_slot = var_tmf1_dn0;
        *var_tmf1_dn2_slot = var_tmf1_dn2;
        *var_tmf1_rv_slot = var_tmf1_rv;
        *var_tmf2_slot = var_tmf2;
        *var_tmf2_dn0_slot = var_tmf2_dn0;
        *var_tmf2_dn2_slot = var_tmf2_dn2;
        *var_tmf2_rv_slot = var_tmf2_rv;
        *var_vha1_slot = var_vha1;
        *var_vha1_rv_slot = var_vha1_rv;
    }

    pub(super) fn stamp_reactive_block_23(
        p: &Parameters,
        var_guard307: f64,
        var_guard308: f64,
        var_guard31: f64,
        var_guard323: f64,
        var_ndigat_i: f64,
        var_nfagat_i: f64,
        var_nfasti_i: f64,
        var_nin: f64,
        var_phitdinv: f64,
        var_vmax: f64,
        var_dfn_sl_slot: &mut f64,
        var_dfn_sl_dn0_slot: &mut f64,
        var_dfn_sl_dn2_slot: &mut f64,
        var_dfn_sl_rv_slot: &mut f64,
        var_dfn_su_slot: &mut f64,
        var_dfn_su_dn0_slot: &mut f64,
        var_dfn_su_dn2_slot: &mut f64,
        var_dfn_su_rv_slot: &mut f64,
        var_dnj1_dv_slot: &mut f64,
        var_dnj1_dv_dn0_slot: &mut f64,
        var_dnj1_dv_dn2_slot: &mut f64,
        var_dnj1_dv_rv_slot: &mut f64,
        var_dvmax_over_phitd_dv_slot: &mut f64,
        var_dvmax_over_phitd_dv_dn0_slot: &mut f64,
        var_dvmax_over_phitd_dv_dn2_slot: &mut f64,
        var_dvmax_over_phitd_dv_rv_slot: &mut f64,
        var_guard326_slot: &mut f64,
        var_guard326_rv_slot: &mut f64,
        var_nj0_slot: &mut f64,
        var_nj0_dn0_slot: &mut f64,
        var_nj0_dn2_slot: &mut f64,
        var_nj0_rv_slot: &mut f64,
        var_nj1_slot: &mut f64,
        var_nj1_dn0_slot: &mut f64,
        var_nj1_dn2_slot: &mut f64,
        var_nj1_rv_slot: &mut f64,
        var_nja10_slot: &mut f64,
        var_nja10_dn0_slot: &mut f64,
        var_nja10_dn2_slot: &mut f64,
        var_nja10_rv_slot: &mut f64,
        var_nja11_slot: &mut f64,
        var_nja11_dn0_slot: &mut f64,
        var_nja11_dn2_slot: &mut f64,
        var_nja11_rv_slot: &mut f64,
        var_pnn0_slot: &mut f64,
        var_pnn0_rv_slot: &mut f64,
        var_tmf1_slot: &mut f64,
        var_tmf1_dn0_slot: &mut f64,
        var_tmf1_dn2_slot: &mut f64,
        var_tmf1_rv_slot: &mut f64,
        var_tmf2_slot: &mut f64,
        var_tmf2_dn0_slot: &mut f64,
        var_tmf2_dn2_slot: &mut f64,
        var_tmf2_rv_slot: &mut f64,
        var_vha1_slot: &mut f64,
        var_vha1_rv_slot: &mut f64,
    ) {
        let mut var_dfn_sl: f64 = *var_dfn_sl_slot;
        let mut var_dfn_sl_dn0: f64 = *var_dfn_sl_dn0_slot;
        let mut var_dfn_sl_dn2: f64 = *var_dfn_sl_dn2_slot;
        let mut var_dfn_sl_rv: f64 = *var_dfn_sl_rv_slot;
        let mut var_dfn_su: f64 = *var_dfn_su_slot;
        let mut var_dfn_su_dn0: f64 = *var_dfn_su_dn0_slot;
        let mut var_dfn_su_dn2: f64 = *var_dfn_su_dn2_slot;
        let mut var_dfn_su_rv: f64 = *var_dfn_su_rv_slot;
        let mut var_dnj1_dv: f64 = *var_dnj1_dv_slot;
        let mut var_dnj1_dv_dn0: f64 = *var_dnj1_dv_dn0_slot;
        let mut var_dnj1_dv_dn2: f64 = *var_dnj1_dv_dn2_slot;
        let mut var_dnj1_dv_rv: f64 = *var_dnj1_dv_rv_slot;
        let mut var_dvmax_over_phitd_dv: f64 = *var_dvmax_over_phitd_dv_slot;
        let mut var_dvmax_over_phitd_dv_dn0: f64 = *var_dvmax_over_phitd_dv_dn0_slot;
        let mut var_dvmax_over_phitd_dv_dn2: f64 = *var_dvmax_over_phitd_dv_dn2_slot;
        let mut var_dvmax_over_phitd_dv_rv: f64 = *var_dvmax_over_phitd_dv_rv_slot;
        let mut var_guard326: f64 = *var_guard326_slot;
        let mut var_guard326_rv: f64 = *var_guard326_rv_slot;
        let mut var_nj0: f64 = *var_nj0_slot;
        let mut var_nj0_dn0: f64 = *var_nj0_dn0_slot;
        let mut var_nj0_dn2: f64 = *var_nj0_dn2_slot;
        let mut var_nj0_rv: f64 = *var_nj0_rv_slot;
        let mut var_nj1: f64 = *var_nj1_slot;
        let mut var_nj1_dn0: f64 = *var_nj1_dn0_slot;
        let mut var_nj1_dn2: f64 = *var_nj1_dn2_slot;
        let mut var_nj1_rv: f64 = *var_nj1_rv_slot;
        let mut var_nja10: f64 = *var_nja10_slot;
        let mut var_nja10_dn0: f64 = *var_nja10_dn0_slot;
        let mut var_nja10_dn2: f64 = *var_nja10_dn2_slot;
        let mut var_nja10_rv: f64 = *var_nja10_rv_slot;
        let mut var_nja11: f64 = *var_nja11_slot;
        let mut var_nja11_dn0: f64 = *var_nja11_dn0_slot;
        let mut var_nja11_dn2: f64 = *var_nja11_dn2_slot;
        let mut var_nja11_rv: f64 = *var_nja11_rv_slot;
        let mut var_pnn0: f64 = *var_pnn0_slot;
        let mut var_pnn0_rv: f64 = *var_pnn0_rv_slot;
        let mut var_tmf1: f64 = *var_tmf1_slot;
        let mut var_tmf1_dn0: f64 = *var_tmf1_dn0_slot;
        let mut var_tmf1_dn2: f64 = *var_tmf1_dn2_slot;
        let mut var_tmf1_rv: f64 = *var_tmf1_rv_slot;
        let mut var_tmf2: f64 = *var_tmf2_slot;
        let mut var_tmf2_dn0: f64 = *var_tmf2_dn0_slot;
        let mut var_tmf2_dn2: f64 = *var_tmf2_dn2_slot;
        let mut var_tmf2_rv: f64 = *var_tmf2_rv_slot;
        let mut var_vha1: f64 = *var_vha1_slot;
        let mut var_vha1_rv: f64 = *var_vha1_rv_slot;

        let (assign20060_e28944, assign20060_e28944_d_n0, assign20060_e28944_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard323 != 0.0)) {
        let assign20060_e28940: f64 = (p.p85 - var_nja10);
        let assign20060_e28942: f64 = (assign20060_e28940 - 0.01);
        (assign20060_e28942, (-var_nja10_dn0), (-var_nja10_dn2),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2,)
    }
};
        var_tmf1 = assign20060_e28944;
        var_tmf1_dn0 = assign20060_e28944_d_n0;
        var_tmf1_dn2 = assign20060_e28944_d_n2;
        var_tmf1_rv = 0.0;

        let (assign20070_e28959, assign20070_e28959_d_n0, assign20070_e28959_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard323 != 0.0)) {
        let assign20070_e28955: f64 = (4.0 * p.p85);
        let assign20070_e28957: f64 = (assign20070_e28955 * 0.01);
        (assign20070_e28957, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign20070_e28959;
        var_tmf2_dn0 = assign20070_e28959_d_n0;
        var_tmf2_dn2 = assign20070_e28959_d_n2;
        var_tmf2_rv = 0.0;

        let (assign20080_e28976, assign20080_e28976_d_n0, assign20080_e28976_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard323 != 0.0)) {
        let (assign20080_e28974, assign20080_e28974_d_n0, assign20080_e28974_d_n2,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
            } else {
                let assign20080_e28973: f64 = (-var_tmf2);
                (assign20080_e28973, (-var_tmf2_dn0), (-var_tmf2_dn2),)
            }
        };
        (assign20080_e28974, assign20080_e28974_d_n0, assign20080_e28974_d_n2,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign20080_e28976;
        var_tmf2_dn0 = assign20080_e28976_d_n0;
        var_tmf2_dn2 = assign20080_e28976_d_n2;
        var_tmf2_rv = 0.0;

        let (assign20090_e28992, assign20090_e28992_d_n0, assign20090_e28992_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard323 != 0.0)) {
        let assign20090_e28987: f64 = (var_tmf1 * var_tmf1);
        let assign20090_e28989: f64 = (assign20090_e28987 + var_tmf2);
        let assign20090_e28990: f64 = (assign20090_e28989).sqrt();
        (assign20090_e28990, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign20090_e28990)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign20090_e28990)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign20090_e28992;
        var_tmf2_dn0 = assign20090_e28992_d_n0;
        var_tmf2_dn2 = assign20090_e28992_d_n2;
        var_tmf2_rv = 0.0;

        let (assign20100_e29009, assign20100_e29009_d_n0, assign20100_e29009_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard323 != 0.0)) {
        let assign20100_e29005: f64 = (var_tmf1 / var_tmf2);
        let assign20100_e29006: f64 = (1.0 + assign20100_e29005);
        let assign20100_e29007: f64 = (0.5 * assign20100_e29006);
        (assign20100_e29007, (0.5 * (((var_tmf1_dn0 * var_tmf2) - (var_tmf1 * var_tmf2_dn0)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn2 * var_tmf2) - (var_tmf1 * var_tmf2_dn2)) / (var_tmf2 * var_tmf2))),)
    } else {
        (var_dfn_su, var_dfn_su_dn0, var_dfn_su_dn2,)
    }
};
        var_dfn_su = assign20100_e29009;
        var_dfn_su_dn0 = assign20100_e29009_d_n0;
        var_dfn_su_dn2 = assign20100_e29009_d_n2;
        var_dfn_su_rv = 0.0;

        let (assign20110_e29026, assign20110_e29026_d_n0, assign20110_e29026_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard323 != 0.0)) {
        let assign20110_e29022: f64 = (var_tmf1 + var_tmf2);
        let assign20110_e29023: f64 = (0.5 * assign20110_e29022);
        let assign20110_e29024: f64 = (p.p85 - assign20110_e29023);
        (assign20110_e29024, (-(0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (-(0.5 * (var_tmf1_dn2 + var_tmf2_dn2))),)
    } else {
        (var_nja11, var_nja11_dn0, var_nja11_dn2,)
    }
};
        var_nja11 = assign20110_e29026;
        var_nja11_dn0 = assign20110_e29026_d_n0;
        var_nja11_dn2 = assign20110_e29026_d_n2;
        var_nja11_rv = 0.0;

        let (assign20120_e29041, assign20120_e29041_d_n0, assign20120_e29041_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard323 != 0.0)) {
        let assign20120_e29037: f64 = (var_nja11 - var_nfasti_i);
        let assign20120_e29039: f64 = (assign20120_e29037 - 0.01);
        (assign20120_e29039, var_nja11_dn0, var_nja11_dn2,)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2,)
    }
};
        var_tmf1 = assign20120_e29041;
        var_tmf1_dn0 = assign20120_e29041_d_n0;
        var_tmf1_dn2 = assign20120_e29041_d_n2;
        var_tmf1_rv = 0.0;

        let (assign20130_e29056, assign20130_e29056_d_n0, assign20130_e29056_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard323 != 0.0)) {
        let assign20130_e29052: f64 = (4.0 * var_nfasti_i);
        let assign20130_e29054: f64 = (assign20130_e29052 * 0.01);
        (assign20130_e29054, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign20130_e29056;
        var_tmf2_dn0 = assign20130_e29056_d_n0;
        var_tmf2_dn2 = assign20130_e29056_d_n2;
        var_tmf2_rv = 0.0;

        let (assign20140_e29073, assign20140_e29073_d_n0, assign20140_e29073_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard323 != 0.0)) {
        let (assign20140_e29071, assign20140_e29071_d_n0, assign20140_e29071_d_n2,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
            } else {
                let assign20140_e29070: f64 = (-var_tmf2);
                (assign20140_e29070, (-var_tmf2_dn0), (-var_tmf2_dn2),)
            }
        };
        (assign20140_e29071, assign20140_e29071_d_n0, assign20140_e29071_d_n2,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign20140_e29073;
        var_tmf2_dn0 = assign20140_e29073_d_n0;
        var_tmf2_dn2 = assign20140_e29073_d_n2;
        var_tmf2_rv = 0.0;

        let (assign20150_e29089, assign20150_e29089_d_n0, assign20150_e29089_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard323 != 0.0)) {
        let assign20150_e29084: f64 = (var_tmf1 * var_tmf1);
        let assign20150_e29086: f64 = (assign20150_e29084 + var_tmf2);
        let assign20150_e29087: f64 = (assign20150_e29086).sqrt();
        (assign20150_e29087, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign20150_e29087)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign20150_e29087)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign20150_e29089;
        var_tmf2_dn0 = assign20150_e29089_d_n0;
        var_tmf2_dn2 = assign20150_e29089_d_n2;
        var_tmf2_rv = 0.0;

        let (assign20160_e29106, assign20160_e29106_d_n0, assign20160_e29106_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard323 != 0.0)) {
        let assign20160_e29102: f64 = (var_tmf1 / var_tmf2);
        let assign20160_e29103: f64 = (1.0 + assign20160_e29102);
        let assign20160_e29104: f64 = (0.5 * assign20160_e29103);
        (assign20160_e29104, (0.5 * (((var_tmf1_dn0 * var_tmf2) - (var_tmf1 * var_tmf2_dn0)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn2 * var_tmf2) - (var_tmf1 * var_tmf2_dn2)) / (var_tmf2 * var_tmf2))),)
    } else {
        (var_dfn_sl, var_dfn_sl_dn0, var_dfn_sl_dn2,)
    }
};
        var_dfn_sl = assign20160_e29106;
        var_dfn_sl_dn0 = assign20160_e29106_d_n0;
        var_dfn_sl_dn2 = assign20160_e29106_d_n2;
        var_dfn_sl_rv = 0.0;

        let (assign20170_e29123, assign20170_e29123_d_n0, assign20170_e29123_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard323 != 0.0)) {
        let assign20170_e29119: f64 = (var_tmf1 + var_tmf2);
        let assign20170_e29120: f64 = (0.5 * assign20170_e29119);
        let assign20170_e29121: f64 = (var_nfasti_i + assign20170_e29120);
        (assign20170_e29121, (0.5 * (var_tmf1_dn0 + var_tmf2_dn0)), (0.5 * (var_tmf1_dn2 + var_tmf2_dn2)),)
    } else {
        (var_nj1, var_nj1_dn0, var_nj1_dn2,)
    }
};
        var_nj1 = assign20170_e29123;
        var_nj1_dn0 = assign20170_e29123_d_n0;
        var_nj1_dn2 = assign20170_e29123_d_n2;
        var_nj1_rv = 0.0;

        let (assign20180_e29138, assign20180_e29138_d_n0, assign20180_e29138_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard323 != 0.0)) {
        let assign20180_e29134: f64 = (p.p85 - var_nj0);
        let assign20180_e29136: f64 = (assign20180_e29134 - 0.01);
        (assign20180_e29136, (-var_nj0_dn0), (-var_nj0_dn2),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2,)
    }
};
        var_tmf1 = assign20180_e29138;
        var_tmf1_dn0 = assign20180_e29138_d_n0;
        var_tmf1_dn2 = assign20180_e29138_d_n2;
        var_tmf1_rv = 0.0;

        let (assign20190_e29153, assign20190_e29153_d_n0, assign20190_e29153_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard323 != 0.0)) {
        let assign20190_e29149: f64 = (4.0 * p.p85);
        let assign20190_e29151: f64 = (assign20190_e29149 * 0.01);
        (assign20190_e29151, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign20190_e29153;
        var_tmf2_dn0 = assign20190_e29153_d_n0;
        var_tmf2_dn2 = assign20190_e29153_d_n2;
        var_tmf2_rv = 0.0;

        let (assign20200_e29170, assign20200_e29170_d_n0, assign20200_e29170_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard323 != 0.0)) {
        let (assign20200_e29168, assign20200_e29168_d_n0, assign20200_e29168_d_n2,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
            } else {
                let assign20200_e29167: f64 = (-var_tmf2);
                (assign20200_e29167, (-var_tmf2_dn0), (-var_tmf2_dn2),)
            }
        };
        (assign20200_e29168, assign20200_e29168_d_n0, assign20200_e29168_d_n2,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign20200_e29170;
        var_tmf2_dn0 = assign20200_e29170_d_n0;
        var_tmf2_dn2 = assign20200_e29170_d_n2;
        var_tmf2_rv = 0.0;

        let (assign20210_e29186, assign20210_e29186_d_n0, assign20210_e29186_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard323 != 0.0)) {
        let assign20210_e29181: f64 = (var_tmf1 * var_tmf1);
        let assign20210_e29183: f64 = (assign20210_e29181 + var_tmf2);
        let assign20210_e29184: f64 = (assign20210_e29183).sqrt();
        (assign20210_e29184, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign20210_e29184)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign20210_e29184)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign20210_e29186;
        var_tmf2_dn0 = assign20210_e29186_d_n0;
        var_tmf2_dn2 = assign20210_e29186_d_n2;
        var_tmf2_rv = 0.0;

        let (assign20220_e29203, assign20220_e29203_d_n0, assign20220_e29203_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard323 != 0.0)) {
        let assign20220_e29199: f64 = (var_tmf1 + var_tmf2);
        let assign20220_e29200: f64 = (0.5 * assign20220_e29199);
        let assign20220_e29201: f64 = (p.p85 - assign20220_e29200);
        (assign20220_e29201, (-(0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (-(0.5 * (var_tmf1_dn2 + var_tmf2_dn2))),)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn2,)
    }
};
        var_nj0 = assign20220_e29203;
        var_nj0_dn0 = assign20220_e29203_d_n0;
        var_nj0_dn2 = assign20220_e29203_d_n2;
        var_nj0_rv = 0.0;

        let (assign20230_e29218, assign20230_e29218_d_n0, assign20230_e29218_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard323 != 0.0)) {
        let assign20230_e29214: f64 = (var_nj0 - var_nfasti_i);
        let assign20230_e29216: f64 = (assign20230_e29214 - 0.01);
        (assign20230_e29216, var_nj0_dn0, var_nj0_dn2,)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2,)
    }
};
        var_tmf1 = assign20230_e29218;
        var_tmf1_dn0 = assign20230_e29218_d_n0;
        var_tmf1_dn2 = assign20230_e29218_d_n2;
        var_tmf1_rv = 0.0;

        let (assign20240_e29233, assign20240_e29233_d_n0, assign20240_e29233_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard323 != 0.0)) {
        let assign20240_e29229: f64 = (4.0 * var_nfasti_i);
        let assign20240_e29231: f64 = (assign20240_e29229 * 0.01);
        (assign20240_e29231, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign20240_e29233;
        var_tmf2_dn0 = assign20240_e29233_d_n0;
        var_tmf2_dn2 = assign20240_e29233_d_n2;
        var_tmf2_rv = 0.0;

        let (assign20250_e29250, assign20250_e29250_d_n0, assign20250_e29250_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard323 != 0.0)) {
        let (assign20250_e29248, assign20250_e29248_d_n0, assign20250_e29248_d_n2,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
            } else {
                let assign20250_e29247: f64 = (-var_tmf2);
                (assign20250_e29247, (-var_tmf2_dn0), (-var_tmf2_dn2),)
            }
        };
        (assign20250_e29248, assign20250_e29248_d_n0, assign20250_e29248_d_n2,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign20250_e29250;
        var_tmf2_dn0 = assign20250_e29250_d_n0;
        var_tmf2_dn2 = assign20250_e29250_d_n2;
        var_tmf2_rv = 0.0;

        let (assign20260_e29266, assign20260_e29266_d_n0, assign20260_e29266_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard323 != 0.0)) {
        let assign20260_e29261: f64 = (var_tmf1 * var_tmf1);
        let assign20260_e29263: f64 = (assign20260_e29261 + var_tmf2);
        let assign20260_e29264: f64 = (assign20260_e29263).sqrt();
        (assign20260_e29264, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign20260_e29264)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign20260_e29264)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign20260_e29266;
        var_tmf2_dn0 = assign20260_e29266_d_n0;
        var_tmf2_dn2 = assign20260_e29266_d_n2;
        var_tmf2_rv = 0.0;

        let (assign20270_e29283, assign20270_e29283_d_n0, assign20270_e29283_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard323 != 0.0)) {
        let assign20270_e29279: f64 = (var_tmf1 + var_tmf2);
        let assign20270_e29280: f64 = (0.5 * assign20270_e29279);
        let assign20270_e29281: f64 = (var_nfasti_i + assign20270_e29280);
        (assign20270_e29281, (0.5 * (var_tmf1_dn0 + var_tmf2_dn0)), (0.5 * (var_tmf1_dn2 + var_tmf2_dn2)),)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn2,)
    }
};
        var_nj0 = assign20270_e29283;
        var_nj0_dn0 = assign20270_e29283_d_n0;
        var_nj0_dn2 = assign20270_e29283_d_n2;
        var_nj0_rv = 0.0;

        let (assign20280_e29298, assign20280_e29298_d_n0, assign20280_e29298_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard323 != 0.0)) {
        let assign20280_e29294: f64 = (p.p86 * var_dfn_su);
        let assign20280_e29296: f64 = (assign20280_e29294 * var_dfn_sl);
        (assign20280_e29296, (((p.p86 * var_dfn_su_dn0) * var_dfn_sl) + (assign20280_e29294 * var_dfn_sl_dn0)), (((p.p86 * var_dfn_su_dn2) * var_dfn_sl) + (assign20280_e29294 * var_dfn_sl_dn2)),)
    } else {
        (var_dnj1_dv, var_dnj1_dv_dn0, var_dnj1_dv_dn2,)
    }
};
        var_dnj1_dv = assign20280_e29298;
        var_dnj1_dv_dn0 = assign20280_e29298_d_n0;
        var_dnj1_dv_dn2 = assign20280_e29298_d_n2;
        var_dnj1_dv_rv = 0.0;

        let (assign20290_e29310, assign20290_e29310_d_n0, assign20290_e29310_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard323 == 0.0)) {
        (var_nfasti_i, 0.0, 0.0,)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn2,)
    }
};
        var_nj0 = assign20290_e29310;
        var_nj0_dn0 = assign20290_e29310_d_n0;
        var_nj0_dn2 = assign20290_e29310_d_n2;
        var_nj0_rv = 0.0;

        let (assign20300_e29322, assign20300_e29322_d_n0, assign20300_e29322_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard323 == 0.0)) {
        (var_nfasti_i, 0.0, 0.0,)
    } else {
        (var_nj1, var_nj1_dn0, var_nj1_dn2,)
    }
};
        var_nj1 = assign20300_e29322;
        var_nj1_dn0 = assign20300_e29322_d_n0;
        var_nj1_dn2 = assign20300_e29322_d_n2;
        var_nj1_rv = 0.0;

        let (assign20310_e29334, assign20310_e29334_d_n0, assign20310_e29334_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard323 == 0.0)) {
        (0.0, 0.0, 0.0,)
    } else {
        (var_dnj1_dv, var_dnj1_dv_dn0, var_dnj1_dv_dn2,)
    }
};
        var_dnj1_dv = assign20310_e29334;
        var_dnj1_dv_dn0 = assign20310_e29334_d_n0;
        var_dnj1_dv_dn2 = assign20310_e29334_d_n2;
        var_dnj1_dv_rv = 0.0;

        let (assign20370_e29583, assign20370_e29583_d_n0, assign20370_e29583_d_n2,) = {
    if (((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) {
        let assign20370_e29567: f64 = (var_vmax * var_dnj1_dv);
        let assign20370_e29568: f64 = (var_nj1 - assign20370_e29567);
        let assign20370_e29571: f64 = (var_nj1 * var_nj1);
        let assign20370_e29572: f64 = (assign20370_e29568 / assign20370_e29571);
        let assign20370_e29575: f64 = (var_vha1 * var_dnj1_dv);
        let assign20370_e29578: f64 = (var_nj0 * p.p85);
        let assign20370_e29579: f64 = (assign20370_e29575 / assign20370_e29578);
        let assign20370_e29580: f64 = (assign20370_e29572 + assign20370_e29579);
        let assign20370_e29581: f64 = (var_phitdinv * assign20370_e29580);
        (assign20370_e29581, (var_phitdinv * (((((var_nj1_dn0 - (var_vmax * var_dnj1_dv_dn0)) * assign20370_e29571) - (assign20370_e29568 * ((var_nj1_dn0 * var_nj1) + (var_nj1 * var_nj1_dn0)))) / (assign20370_e29571 * assign20370_e29571)) + ((((var_vha1 * var_dnj1_dv_dn0) * assign20370_e29578) - (assign20370_e29575 * (var_nj0_dn0 * p.p85))) / (assign20370_e29578 * assign20370_e29578)))), (var_phitdinv * (((((var_nj1_dn2 - (var_vmax * var_dnj1_dv_dn2)) * assign20370_e29571) - (assign20370_e29568 * ((var_nj1_dn2 * var_nj1) + (var_nj1 * var_nj1_dn2)))) / (assign20370_e29571 * assign20370_e29571)) + ((((var_vha1 * var_dnj1_dv_dn2) * assign20370_e29578) - (assign20370_e29575 * (var_nj0_dn2 * p.p85))) / (assign20370_e29578 * assign20370_e29578)))),)
    } else {
        (var_dvmax_over_phitd_dv, var_dvmax_over_phitd_dv_dn0, var_dvmax_over_phitd_dv_dn2,)
    }
};
        var_dvmax_over_phitd_dv = assign20370_e29583;
        var_dvmax_over_phitd_dv_dn0 = assign20370_e29583_d_n0;
        var_dvmax_over_phitd_dv_dn2 = assign20370_e29583_d_n2;
        var_dvmax_over_phitd_dv_rv = 0.0;

        let (assign20390_e29613,) = {
    if (((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) {
        let assign20390_e29609: f64 = (var_nin * var_nin);
        let assign20390_e29611: f64 = (assign20390_e29609 / var_ndigat_i);
        (assign20390_e29611,)
    } else {
        (var_pnn0,)
    }
};
        var_pnn0 = assign20390_e29613;
        var_pnn0_rv = 0.0;

        let (assign20400_e29629,) = {
    if (((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) {
        let assign20400_e29622: f64 = (var_nfagat_i / var_phitdinv);
        let assign20400_e29625: f64 = (var_ndigat_i / var_pnn0);
        let assign20400_e29626: f64 = (assign20400_e29625).ln();
        let assign20400_e29627: f64 = (assign20400_e29622 * assign20400_e29626);
        (assign20400_e29627,)
    } else {
        (var_vha1,)
    }
};
        var_vha1 = assign20400_e29629;
        var_vha1_rv = 0.0;

        let assign20410_e29632: f64 = if var_nfagat_i < p.p85 { 1.0 } else { 0.0 };
        var_guard326 = assign20410_e29632;
        var_guard326_rv = 0.0;

        let (assign20420_e29649, assign20420_e29649_d_n0, assign20420_e29649_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard326 != 0.0)) {
        let assign20420_e29644: f64 = (var_vmax - var_vha1);
        let assign20420_e29645: f64 = (p.p86 * assign20420_e29644);
        let assign20420_e29647: f64 = (assign20420_e29645 + var_nfagat_i);
        (assign20420_e29647, 0.0, 0.0,)
    } else {
        (var_nja10, var_nja10_dn0, var_nja10_dn2,)
    }
};
        var_nja10 = assign20420_e29649;
        var_nja10_dn0 = assign20420_e29649_d_n0;
        var_nja10_dn2 = assign20420_e29649_d_n2;
        var_nja10_rv = 0.0;

        let (assign20430_e29664, assign20430_e29664_d_n0, assign20430_e29664_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard326 != 0.0)) {
        let assign20430_e29661: f64 = (p.p86 * var_vha1);
        let assign20430_e29662: f64 = (var_nfagat_i - assign20430_e29661);
        (assign20430_e29662, 0.0, 0.0,)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn2,)
    }
};
        var_nj0 = assign20430_e29664;
        var_nj0_dn0 = assign20430_e29664_d_n0;
        var_nj0_dn2 = assign20430_e29664_d_n2;
        var_nj0_rv = 0.0;

        let (assign20440_e29679, assign20440_e29679_d_n0, assign20440_e29679_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard326 != 0.0)) {
        let assign20440_e29675: f64 = (p.p85 - var_nja10);
        let assign20440_e29677: f64 = (assign20440_e29675 - 0.01);
        (assign20440_e29677, (-var_nja10_dn0), (-var_nja10_dn2),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2,)
    }
};
        var_tmf1 = assign20440_e29679;
        var_tmf1_dn0 = assign20440_e29679_d_n0;
        var_tmf1_dn2 = assign20440_e29679_d_n2;
        var_tmf1_rv = 0.0;

        let (assign20450_e29694, assign20450_e29694_d_n0, assign20450_e29694_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard326 != 0.0)) {
        let assign20450_e29690: f64 = (4.0 * p.p85);
        let assign20450_e29692: f64 = (assign20450_e29690 * 0.01);
        (assign20450_e29692, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign20450_e29694;
        var_tmf2_dn0 = assign20450_e29694_d_n0;
        var_tmf2_dn2 = assign20450_e29694_d_n2;
        var_tmf2_rv = 0.0;

        *var_dfn_sl_slot = var_dfn_sl;
        *var_dfn_sl_dn0_slot = var_dfn_sl_dn0;
        *var_dfn_sl_dn2_slot = var_dfn_sl_dn2;
        *var_dfn_sl_rv_slot = var_dfn_sl_rv;
        *var_dfn_su_slot = var_dfn_su;
        *var_dfn_su_dn0_slot = var_dfn_su_dn0;
        *var_dfn_su_dn2_slot = var_dfn_su_dn2;
        *var_dfn_su_rv_slot = var_dfn_su_rv;
        *var_dnj1_dv_slot = var_dnj1_dv;
        *var_dnj1_dv_dn0_slot = var_dnj1_dv_dn0;
        *var_dnj1_dv_dn2_slot = var_dnj1_dv_dn2;
        *var_dnj1_dv_rv_slot = var_dnj1_dv_rv;
        *var_dvmax_over_phitd_dv_slot = var_dvmax_over_phitd_dv;
        *var_dvmax_over_phitd_dv_dn0_slot = var_dvmax_over_phitd_dv_dn0;
        *var_dvmax_over_phitd_dv_dn2_slot = var_dvmax_over_phitd_dv_dn2;
        *var_dvmax_over_phitd_dv_rv_slot = var_dvmax_over_phitd_dv_rv;
        *var_guard326_slot = var_guard326;
        *var_guard326_rv_slot = var_guard326_rv;
        *var_nj0_slot = var_nj0;
        *var_nj0_dn0_slot = var_nj0_dn0;
        *var_nj0_dn2_slot = var_nj0_dn2;
        *var_nj0_rv_slot = var_nj0_rv;
        *var_nj1_slot = var_nj1;
        *var_nj1_dn0_slot = var_nj1_dn0;
        *var_nj1_dn2_slot = var_nj1_dn2;
        *var_nj1_rv_slot = var_nj1_rv;
        *var_nja10_slot = var_nja10;
        *var_nja10_dn0_slot = var_nja10_dn0;
        *var_nja10_dn2_slot = var_nja10_dn2;
        *var_nja10_rv_slot = var_nja10_rv;
        *var_nja11_slot = var_nja11;
        *var_nja11_dn0_slot = var_nja11_dn0;
        *var_nja11_dn2_slot = var_nja11_dn2;
        *var_nja11_rv_slot = var_nja11_rv;
        *var_pnn0_slot = var_pnn0;
        *var_pnn0_rv_slot = var_pnn0_rv;
        *var_tmf1_slot = var_tmf1;
        *var_tmf1_dn0_slot = var_tmf1_dn0;
        *var_tmf1_dn2_slot = var_tmf1_dn2;
        *var_tmf1_rv_slot = var_tmf1_rv;
        *var_tmf2_slot = var_tmf2;
        *var_tmf2_dn0_slot = var_tmf2_dn0;
        *var_tmf2_dn2_slot = var_tmf2_dn2;
        *var_tmf2_rv_slot = var_tmf2_rv;
        *var_vha1_slot = var_vha1;
        *var_vha1_rv_slot = var_vha1_rv;
    }

    pub(super) fn stamp_reactive_block_24(
        p: &Parameters,
        var_ab_i: f64,
        var_guard307: f64,
        var_guard308: f64,
        var_guard31: f64,
        var_guard326: f64,
        var_lg_i: f64,
        var_ls_i: f64,
        var_ndibot_i: f64,
        var_nfabot_i: f64,
        var_nfagat_i: f64,
        var_nin: f64,
        var_phitdinv: f64,
        var_v5: f64,
        var_vmax: f64,
        var_dfn_sl_slot: &mut f64,
        var_dfn_sl_dn0_slot: &mut f64,
        var_dfn_sl_dn2_slot: &mut f64,
        var_dfn_sl_rv_slot: &mut f64,
        var_dfn_su_slot: &mut f64,
        var_dfn_su_dn0_slot: &mut f64,
        var_dfn_su_dn2_slot: &mut f64,
        var_dfn_su_rv_slot: &mut f64,
        var_dnj1_dv_slot: &mut f64,
        var_dnj1_dv_dn0_slot: &mut f64,
        var_dnj1_dv_dn2_slot: &mut f64,
        var_dnj1_dv_rv_slot: &mut f64,
        var_dvmax_over_phitd_dv_slot: &mut f64,
        var_dvmax_over_phitd_dv_dn0_slot: &mut f64,
        var_dvmax_over_phitd_dv_dn2_slot: &mut f64,
        var_dvmax_over_phitd_dv_rv_slot: &mut f64,
        var_guard384_slot: &mut f64,
        var_guard384_rv_slot: &mut f64,
        var_guard385_slot: &mut f64,
        var_guard385_rv_slot: &mut f64,
        var_guard388_slot: &mut f64,
        var_guard388_rv_slot: &mut f64,
        var_idmultbot_slot: &mut f64,
        var_idmultbot_dn0_slot: &mut f64,
        var_idmultbot_dn2_slot: &mut f64,
        var_idmultbot_rv_slot: &mut f64,
        var_nj0_slot: &mut f64,
        var_nj0_dn0_slot: &mut f64,
        var_nj0_dn2_slot: &mut f64,
        var_nj0_rv_slot: &mut f64,
        var_nj1_slot: &mut f64,
        var_nj1_dn0_slot: &mut f64,
        var_nj1_dn2_slot: &mut f64,
        var_nj1_rv_slot: &mut f64,
        var_nja10_slot: &mut f64,
        var_nja10_dn0_slot: &mut f64,
        var_nja10_dn2_slot: &mut f64,
        var_nja10_rv_slot: &mut f64,
        var_nja11_slot: &mut f64,
        var_nja11_dn0_slot: &mut f64,
        var_nja11_dn2_slot: &mut f64,
        var_nja11_rv_slot: &mut f64,
        var_pnn0_slot: &mut f64,
        var_pnn0_rv_slot: &mut f64,
        var_tmf1_slot: &mut f64,
        var_tmf1_dn0_slot: &mut f64,
        var_tmf1_dn2_slot: &mut f64,
        var_tmf1_rv_slot: &mut f64,
        var_tmf2_slot: &mut f64,
        var_tmf2_dn0_slot: &mut f64,
        var_tmf2_dn2_slot: &mut f64,
        var_tmf2_rv_slot: &mut f64,
        var_vha1_slot: &mut f64,
        var_vha1_rv_slot: &mut f64,
    ) {
        let mut var_dfn_sl: f64 = *var_dfn_sl_slot;
        let mut var_dfn_sl_dn0: f64 = *var_dfn_sl_dn0_slot;
        let mut var_dfn_sl_dn2: f64 = *var_dfn_sl_dn2_slot;
        let mut var_dfn_sl_rv: f64 = *var_dfn_sl_rv_slot;
        let mut var_dfn_su: f64 = *var_dfn_su_slot;
        let mut var_dfn_su_dn0: f64 = *var_dfn_su_dn0_slot;
        let mut var_dfn_su_dn2: f64 = *var_dfn_su_dn2_slot;
        let mut var_dfn_su_rv: f64 = *var_dfn_su_rv_slot;
        let mut var_dnj1_dv: f64 = *var_dnj1_dv_slot;
        let mut var_dnj1_dv_dn0: f64 = *var_dnj1_dv_dn0_slot;
        let mut var_dnj1_dv_dn2: f64 = *var_dnj1_dv_dn2_slot;
        let mut var_dnj1_dv_rv: f64 = *var_dnj1_dv_rv_slot;
        let mut var_dvmax_over_phitd_dv: f64 = *var_dvmax_over_phitd_dv_slot;
        let mut var_dvmax_over_phitd_dv_dn0: f64 = *var_dvmax_over_phitd_dv_dn0_slot;
        let mut var_dvmax_over_phitd_dv_dn2: f64 = *var_dvmax_over_phitd_dv_dn2_slot;
        let mut var_dvmax_over_phitd_dv_rv: f64 = *var_dvmax_over_phitd_dv_rv_slot;
        let mut var_guard384: f64 = *var_guard384_slot;
        let mut var_guard384_rv: f64 = *var_guard384_rv_slot;
        let mut var_guard385: f64 = *var_guard385_slot;
        let mut var_guard385_rv: f64 = *var_guard385_rv_slot;
        let mut var_guard388: f64 = *var_guard388_slot;
        let mut var_guard388_rv: f64 = *var_guard388_rv_slot;
        let mut var_idmultbot: f64 = *var_idmultbot_slot;
        let mut var_idmultbot_dn0: f64 = *var_idmultbot_dn0_slot;
        let mut var_idmultbot_dn2: f64 = *var_idmultbot_dn2_slot;
        let mut var_idmultbot_rv: f64 = *var_idmultbot_rv_slot;
        let mut var_nj0: f64 = *var_nj0_slot;
        let mut var_nj0_dn0: f64 = *var_nj0_dn0_slot;
        let mut var_nj0_dn2: f64 = *var_nj0_dn2_slot;
        let mut var_nj0_rv: f64 = *var_nj0_rv_slot;
        let mut var_nj1: f64 = *var_nj1_slot;
        let mut var_nj1_dn0: f64 = *var_nj1_dn0_slot;
        let mut var_nj1_dn2: f64 = *var_nj1_dn2_slot;
        let mut var_nj1_rv: f64 = *var_nj1_rv_slot;
        let mut var_nja10: f64 = *var_nja10_slot;
        let mut var_nja10_dn0: f64 = *var_nja10_dn0_slot;
        let mut var_nja10_dn2: f64 = *var_nja10_dn2_slot;
        let mut var_nja10_rv: f64 = *var_nja10_rv_slot;
        let mut var_nja11: f64 = *var_nja11_slot;
        let mut var_nja11_dn0: f64 = *var_nja11_dn0_slot;
        let mut var_nja11_dn2: f64 = *var_nja11_dn2_slot;
        let mut var_nja11_rv: f64 = *var_nja11_rv_slot;
        let mut var_pnn0: f64 = *var_pnn0_slot;
        let mut var_pnn0_rv: f64 = *var_pnn0_rv_slot;
        let mut var_tmf1: f64 = *var_tmf1_slot;
        let mut var_tmf1_dn0: f64 = *var_tmf1_dn0_slot;
        let mut var_tmf1_dn2: f64 = *var_tmf1_dn2_slot;
        let mut var_tmf1_rv: f64 = *var_tmf1_rv_slot;
        let mut var_tmf2: f64 = *var_tmf2_slot;
        let mut var_tmf2_dn0: f64 = *var_tmf2_dn0_slot;
        let mut var_tmf2_dn2: f64 = *var_tmf2_dn2_slot;
        let mut var_tmf2_rv: f64 = *var_tmf2_rv_slot;
        let mut var_vha1: f64 = *var_vha1_slot;
        let mut var_vha1_rv: f64 = *var_vha1_rv_slot;

        let (assign20460_e29711, assign20460_e29711_d_n0, assign20460_e29711_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard326 != 0.0)) {
        let (assign20460_e29709, assign20460_e29709_d_n0, assign20460_e29709_d_n2,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
            } else {
                let assign20460_e29708: f64 = (-var_tmf2);
                (assign20460_e29708, (-var_tmf2_dn0), (-var_tmf2_dn2),)
            }
        };
        (assign20460_e29709, assign20460_e29709_d_n0, assign20460_e29709_d_n2,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign20460_e29711;
        var_tmf2_dn0 = assign20460_e29711_d_n0;
        var_tmf2_dn2 = assign20460_e29711_d_n2;
        var_tmf2_rv = 0.0;

        let (assign20470_e29727, assign20470_e29727_d_n0, assign20470_e29727_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard326 != 0.0)) {
        let assign20470_e29722: f64 = (var_tmf1 * var_tmf1);
        let assign20470_e29724: f64 = (assign20470_e29722 + var_tmf2);
        let assign20470_e29725: f64 = (assign20470_e29724).sqrt();
        (assign20470_e29725, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign20470_e29725)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign20470_e29725)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign20470_e29727;
        var_tmf2_dn0 = assign20470_e29727_d_n0;
        var_tmf2_dn2 = assign20470_e29727_d_n2;
        var_tmf2_rv = 0.0;

        let (assign20480_e29744, assign20480_e29744_d_n0, assign20480_e29744_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard326 != 0.0)) {
        let assign20480_e29740: f64 = (var_tmf1 / var_tmf2);
        let assign20480_e29741: f64 = (1.0 + assign20480_e29740);
        let assign20480_e29742: f64 = (0.5 * assign20480_e29741);
        (assign20480_e29742, (0.5 * (((var_tmf1_dn0 * var_tmf2) - (var_tmf1 * var_tmf2_dn0)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn2 * var_tmf2) - (var_tmf1 * var_tmf2_dn2)) / (var_tmf2 * var_tmf2))),)
    } else {
        (var_dfn_su, var_dfn_su_dn0, var_dfn_su_dn2,)
    }
};
        var_dfn_su = assign20480_e29744;
        var_dfn_su_dn0 = assign20480_e29744_d_n0;
        var_dfn_su_dn2 = assign20480_e29744_d_n2;
        var_dfn_su_rv = 0.0;

        let (assign20490_e29761, assign20490_e29761_d_n0, assign20490_e29761_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard326 != 0.0)) {
        let assign20490_e29757: f64 = (var_tmf1 + var_tmf2);
        let assign20490_e29758: f64 = (0.5 * assign20490_e29757);
        let assign20490_e29759: f64 = (p.p85 - assign20490_e29758);
        (assign20490_e29759, (-(0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (-(0.5 * (var_tmf1_dn2 + var_tmf2_dn2))),)
    } else {
        (var_nja11, var_nja11_dn0, var_nja11_dn2,)
    }
};
        var_nja11 = assign20490_e29761;
        var_nja11_dn0 = assign20490_e29761_d_n0;
        var_nja11_dn2 = assign20490_e29761_d_n2;
        var_nja11_rv = 0.0;

        let (assign20500_e29776, assign20500_e29776_d_n0, assign20500_e29776_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard326 != 0.0)) {
        let assign20500_e29772: f64 = (var_nja11 - var_nfagat_i);
        let assign20500_e29774: f64 = (assign20500_e29772 - 0.01);
        (assign20500_e29774, var_nja11_dn0, var_nja11_dn2,)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2,)
    }
};
        var_tmf1 = assign20500_e29776;
        var_tmf1_dn0 = assign20500_e29776_d_n0;
        var_tmf1_dn2 = assign20500_e29776_d_n2;
        var_tmf1_rv = 0.0;

        let (assign20510_e29791, assign20510_e29791_d_n0, assign20510_e29791_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard326 != 0.0)) {
        let assign20510_e29787: f64 = (4.0 * var_nfagat_i);
        let assign20510_e29789: f64 = (assign20510_e29787 * 0.01);
        (assign20510_e29789, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign20510_e29791;
        var_tmf2_dn0 = assign20510_e29791_d_n0;
        var_tmf2_dn2 = assign20510_e29791_d_n2;
        var_tmf2_rv = 0.0;

        let (assign20520_e29808, assign20520_e29808_d_n0, assign20520_e29808_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard326 != 0.0)) {
        let (assign20520_e29806, assign20520_e29806_d_n0, assign20520_e29806_d_n2,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
            } else {
                let assign20520_e29805: f64 = (-var_tmf2);
                (assign20520_e29805, (-var_tmf2_dn0), (-var_tmf2_dn2),)
            }
        };
        (assign20520_e29806, assign20520_e29806_d_n0, assign20520_e29806_d_n2,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign20520_e29808;
        var_tmf2_dn0 = assign20520_e29808_d_n0;
        var_tmf2_dn2 = assign20520_e29808_d_n2;
        var_tmf2_rv = 0.0;

        let (assign20530_e29824, assign20530_e29824_d_n0, assign20530_e29824_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard326 != 0.0)) {
        let assign20530_e29819: f64 = (var_tmf1 * var_tmf1);
        let assign20530_e29821: f64 = (assign20530_e29819 + var_tmf2);
        let assign20530_e29822: f64 = (assign20530_e29821).sqrt();
        (assign20530_e29822, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign20530_e29822)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign20530_e29822)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign20530_e29824;
        var_tmf2_dn0 = assign20530_e29824_d_n0;
        var_tmf2_dn2 = assign20530_e29824_d_n2;
        var_tmf2_rv = 0.0;

        let (assign20540_e29841, assign20540_e29841_d_n0, assign20540_e29841_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard326 != 0.0)) {
        let assign20540_e29837: f64 = (var_tmf1 / var_tmf2);
        let assign20540_e29838: f64 = (1.0 + assign20540_e29837);
        let assign20540_e29839: f64 = (0.5 * assign20540_e29838);
        (assign20540_e29839, (0.5 * (((var_tmf1_dn0 * var_tmf2) - (var_tmf1 * var_tmf2_dn0)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn2 * var_tmf2) - (var_tmf1 * var_tmf2_dn2)) / (var_tmf2 * var_tmf2))),)
    } else {
        (var_dfn_sl, var_dfn_sl_dn0, var_dfn_sl_dn2,)
    }
};
        var_dfn_sl = assign20540_e29841;
        var_dfn_sl_dn0 = assign20540_e29841_d_n0;
        var_dfn_sl_dn2 = assign20540_e29841_d_n2;
        var_dfn_sl_rv = 0.0;

        let (assign20550_e29858, assign20550_e29858_d_n0, assign20550_e29858_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard326 != 0.0)) {
        let assign20550_e29854: f64 = (var_tmf1 + var_tmf2);
        let assign20550_e29855: f64 = (0.5 * assign20550_e29854);
        let assign20550_e29856: f64 = (var_nfagat_i + assign20550_e29855);
        (assign20550_e29856, (0.5 * (var_tmf1_dn0 + var_tmf2_dn0)), (0.5 * (var_tmf1_dn2 + var_tmf2_dn2)),)
    } else {
        (var_nj1, var_nj1_dn0, var_nj1_dn2,)
    }
};
        var_nj1 = assign20550_e29858;
        var_nj1_dn0 = assign20550_e29858_d_n0;
        var_nj1_dn2 = assign20550_e29858_d_n2;
        var_nj1_rv = 0.0;

        let (assign20560_e29873, assign20560_e29873_d_n0, assign20560_e29873_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard326 != 0.0)) {
        let assign20560_e29869: f64 = (p.p85 - var_nj0);
        let assign20560_e29871: f64 = (assign20560_e29869 - 0.01);
        (assign20560_e29871, (-var_nj0_dn0), (-var_nj0_dn2),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2,)
    }
};
        var_tmf1 = assign20560_e29873;
        var_tmf1_dn0 = assign20560_e29873_d_n0;
        var_tmf1_dn2 = assign20560_e29873_d_n2;
        var_tmf1_rv = 0.0;

        let (assign20570_e29888, assign20570_e29888_d_n0, assign20570_e29888_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard326 != 0.0)) {
        let assign20570_e29884: f64 = (4.0 * p.p85);
        let assign20570_e29886: f64 = (assign20570_e29884 * 0.01);
        (assign20570_e29886, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign20570_e29888;
        var_tmf2_dn0 = assign20570_e29888_d_n0;
        var_tmf2_dn2 = assign20570_e29888_d_n2;
        var_tmf2_rv = 0.0;

        let (assign20580_e29905, assign20580_e29905_d_n0, assign20580_e29905_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard326 != 0.0)) {
        let (assign20580_e29903, assign20580_e29903_d_n0, assign20580_e29903_d_n2,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
            } else {
                let assign20580_e29902: f64 = (-var_tmf2);
                (assign20580_e29902, (-var_tmf2_dn0), (-var_tmf2_dn2),)
            }
        };
        (assign20580_e29903, assign20580_e29903_d_n0, assign20580_e29903_d_n2,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign20580_e29905;
        var_tmf2_dn0 = assign20580_e29905_d_n0;
        var_tmf2_dn2 = assign20580_e29905_d_n2;
        var_tmf2_rv = 0.0;

        let (assign20590_e29921, assign20590_e29921_d_n0, assign20590_e29921_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard326 != 0.0)) {
        let assign20590_e29916: f64 = (var_tmf1 * var_tmf1);
        let assign20590_e29918: f64 = (assign20590_e29916 + var_tmf2);
        let assign20590_e29919: f64 = (assign20590_e29918).sqrt();
        (assign20590_e29919, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign20590_e29919)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign20590_e29919)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign20590_e29921;
        var_tmf2_dn0 = assign20590_e29921_d_n0;
        var_tmf2_dn2 = assign20590_e29921_d_n2;
        var_tmf2_rv = 0.0;

        let (assign20600_e29938, assign20600_e29938_d_n0, assign20600_e29938_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard326 != 0.0)) {
        let assign20600_e29934: f64 = (var_tmf1 + var_tmf2);
        let assign20600_e29935: f64 = (0.5 * assign20600_e29934);
        let assign20600_e29936: f64 = (p.p85 - assign20600_e29935);
        (assign20600_e29936, (-(0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (-(0.5 * (var_tmf1_dn2 + var_tmf2_dn2))),)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn2,)
    }
};
        var_nj0 = assign20600_e29938;
        var_nj0_dn0 = assign20600_e29938_d_n0;
        var_nj0_dn2 = assign20600_e29938_d_n2;
        var_nj0_rv = 0.0;

        let (assign20610_e29953, assign20610_e29953_d_n0, assign20610_e29953_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard326 != 0.0)) {
        let assign20610_e29949: f64 = (var_nj0 - var_nfagat_i);
        let assign20610_e29951: f64 = (assign20610_e29949 - 0.01);
        (assign20610_e29951, var_nj0_dn0, var_nj0_dn2,)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2,)
    }
};
        var_tmf1 = assign20610_e29953;
        var_tmf1_dn0 = assign20610_e29953_d_n0;
        var_tmf1_dn2 = assign20610_e29953_d_n2;
        var_tmf1_rv = 0.0;

        let (assign20620_e29968, assign20620_e29968_d_n0, assign20620_e29968_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard326 != 0.0)) {
        let assign20620_e29964: f64 = (4.0 * var_nfagat_i);
        let assign20620_e29966: f64 = (assign20620_e29964 * 0.01);
        (assign20620_e29966, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign20620_e29968;
        var_tmf2_dn0 = assign20620_e29968_d_n0;
        var_tmf2_dn2 = assign20620_e29968_d_n2;
        var_tmf2_rv = 0.0;

        let (assign20630_e29985, assign20630_e29985_d_n0, assign20630_e29985_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard326 != 0.0)) {
        let (assign20630_e29983, assign20630_e29983_d_n0, assign20630_e29983_d_n2,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
            } else {
                let assign20630_e29982: f64 = (-var_tmf2);
                (assign20630_e29982, (-var_tmf2_dn0), (-var_tmf2_dn2),)
            }
        };
        (assign20630_e29983, assign20630_e29983_d_n0, assign20630_e29983_d_n2,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign20630_e29985;
        var_tmf2_dn0 = assign20630_e29985_d_n0;
        var_tmf2_dn2 = assign20630_e29985_d_n2;
        var_tmf2_rv = 0.0;

        let (assign20640_e30001, assign20640_e30001_d_n0, assign20640_e30001_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard326 != 0.0)) {
        let assign20640_e29996: f64 = (var_tmf1 * var_tmf1);
        let assign20640_e29998: f64 = (assign20640_e29996 + var_tmf2);
        let assign20640_e29999: f64 = (assign20640_e29998).sqrt();
        (assign20640_e29999, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign20640_e29999)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign20640_e29999)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign20640_e30001;
        var_tmf2_dn0 = assign20640_e30001_d_n0;
        var_tmf2_dn2 = assign20640_e30001_d_n2;
        var_tmf2_rv = 0.0;

        let (assign20650_e30018, assign20650_e30018_d_n0, assign20650_e30018_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard326 != 0.0)) {
        let assign20650_e30014: f64 = (var_tmf1 + var_tmf2);
        let assign20650_e30015: f64 = (0.5 * assign20650_e30014);
        let assign20650_e30016: f64 = (var_nfagat_i + assign20650_e30015);
        (assign20650_e30016, (0.5 * (var_tmf1_dn0 + var_tmf2_dn0)), (0.5 * (var_tmf1_dn2 + var_tmf2_dn2)),)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn2,)
    }
};
        var_nj0 = assign20650_e30018;
        var_nj0_dn0 = assign20650_e30018_d_n0;
        var_nj0_dn2 = assign20650_e30018_d_n2;
        var_nj0_rv = 0.0;

        let (assign20660_e30033, assign20660_e30033_d_n0, assign20660_e30033_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard326 != 0.0)) {
        let assign20660_e30029: f64 = (p.p86 * var_dfn_su);
        let assign20660_e30031: f64 = (assign20660_e30029 * var_dfn_sl);
        (assign20660_e30031, (((p.p86 * var_dfn_su_dn0) * var_dfn_sl) + (assign20660_e30029 * var_dfn_sl_dn0)), (((p.p86 * var_dfn_su_dn2) * var_dfn_sl) + (assign20660_e30029 * var_dfn_sl_dn2)),)
    } else {
        (var_dnj1_dv, var_dnj1_dv_dn0, var_dnj1_dv_dn2,)
    }
};
        var_dnj1_dv = assign20660_e30033;
        var_dnj1_dv_dn0 = assign20660_e30033_d_n0;
        var_dnj1_dv_dn2 = assign20660_e30033_d_n2;
        var_dnj1_dv_rv = 0.0;

        let (assign20670_e30045, assign20670_e30045_d_n0, assign20670_e30045_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard326 == 0.0)) {
        (var_nfagat_i, 0.0, 0.0,)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn2,)
    }
};
        var_nj0 = assign20670_e30045;
        var_nj0_dn0 = assign20670_e30045_d_n0;
        var_nj0_dn2 = assign20670_e30045_d_n2;
        var_nj0_rv = 0.0;

        let (assign20680_e30057, assign20680_e30057_d_n0, assign20680_e30057_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard326 == 0.0)) {
        (var_nfagat_i, 0.0, 0.0,)
    } else {
        (var_nj1, var_nj1_dn0, var_nj1_dn2,)
    }
};
        var_nj1 = assign20680_e30057;
        var_nj1_dn0 = assign20680_e30057_d_n0;
        var_nj1_dn2 = assign20680_e30057_d_n2;
        var_nj1_rv = 0.0;

        let (assign20690_e30069, assign20690_e30069_d_n0, assign20690_e30069_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard326 == 0.0)) {
        (0.0, 0.0, 0.0,)
    } else {
        (var_dnj1_dv, var_dnj1_dv_dn0, var_dnj1_dv_dn2,)
    }
};
        var_dnj1_dv = assign20690_e30069;
        var_dnj1_dv_dn0 = assign20690_e30069_d_n0;
        var_dnj1_dv_dn2 = assign20690_e30069_d_n2;
        var_dnj1_dv_rv = 0.0;

        let (assign20750_e30318, assign20750_e30318_d_n0, assign20750_e30318_d_n2,) = {
    if (((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) {
        let assign20750_e30302: f64 = (var_vmax * var_dnj1_dv);
        let assign20750_e30303: f64 = (var_nj1 - assign20750_e30302);
        let assign20750_e30306: f64 = (var_nj1 * var_nj1);
        let assign20750_e30307: f64 = (assign20750_e30303 / assign20750_e30306);
        let assign20750_e30310: f64 = (var_vha1 * var_dnj1_dv);
        let assign20750_e30313: f64 = (var_nj0 * p.p85);
        let assign20750_e30314: f64 = (assign20750_e30310 / assign20750_e30313);
        let assign20750_e30315: f64 = (assign20750_e30307 + assign20750_e30314);
        let assign20750_e30316: f64 = (var_phitdinv * assign20750_e30315);
        (assign20750_e30316, (var_phitdinv * (((((var_nj1_dn0 - (var_vmax * var_dnj1_dv_dn0)) * assign20750_e30306) - (assign20750_e30303 * ((var_nj1_dn0 * var_nj1) + (var_nj1 * var_nj1_dn0)))) / (assign20750_e30306 * assign20750_e30306)) + ((((var_vha1 * var_dnj1_dv_dn0) * assign20750_e30313) - (assign20750_e30310 * (var_nj0_dn0 * p.p85))) / (assign20750_e30313 * assign20750_e30313)))), (var_phitdinv * (((((var_nj1_dn2 - (var_vmax * var_dnj1_dv_dn2)) * assign20750_e30306) - (assign20750_e30303 * ((var_nj1_dn2 * var_nj1) + (var_nj1 * var_nj1_dn2)))) / (assign20750_e30306 * assign20750_e30306)) + ((((var_vha1 * var_dnj1_dv_dn2) * assign20750_e30313) - (assign20750_e30310 * (var_nj0_dn2 * p.p85))) / (assign20750_e30313 * assign20750_e30313)))),)
    } else {
        (var_dvmax_over_phitd_dv, var_dvmax_over_phitd_dv_dn0, var_dvmax_over_phitd_dv_dn2,)
    }
};
        var_dvmax_over_phitd_dv = assign20750_e30318;
        var_dvmax_over_phitd_dv_dn0 = assign20750_e30318_d_n0;
        var_dvmax_over_phitd_dv_dn2 = assign20750_e30318_d_n2;
        var_dvmax_over_phitd_dv_rv = 0.0;

        let (assign20770_e30343, assign20770_e30343_d_n0, assign20770_e30343_d_n2,) = {
    if ((var_guard31 != 0.0) && (var_guard307 != 0.0)) {
        let assign20770_e30341: f64 = (var_idmultbot - 1.0);
        (assign20770_e30341, var_idmultbot_dn0, var_idmultbot_dn2,)
    } else {
        (var_idmultbot, var_idmultbot_dn0, var_idmultbot_dn2,)
    }
};
        var_idmultbot = assign20770_e30343;
        var_idmultbot_dn0 = assign20770_e30343_d_n0;
        var_idmultbot_dn2 = assign20770_e30343_d_n2;
        var_idmultbot_rv = 0.0;

        let (assign20880_e30516, assign20880_e30516_d_n0, assign20880_e30516_d_n2,) = {
    if ((var_guard31 != 0.0) && (var_guard307 == 0.0)) {
        (0.0, 0.0, 0.0,)
    } else {
        (var_idmultbot, var_idmultbot_dn0, var_idmultbot_dn2,)
    }
};
        var_idmultbot = assign20880_e30516;
        var_idmultbot_dn0 = assign20880_e30516_d_n0;
        var_idmultbot_dn2 = assign20880_e30516_d_n2;
        var_idmultbot_rv = 0.0;

        let assign23410_e34066: f64 = if (!(((var_ab_i == 0.0) && (var_ls_i == 0.0)) && (var_lg_i == 0.0))) { 1.0 } else { 0.0 };
        var_guard384 = assign23410_e34066;
        var_guard384_rv = 0.0;

        let assign23490_e34138: f64 = if var_v5 < var_vmax { 1.0 } else { 0.0 };
        var_guard385 = assign23490_e34138;
        var_guard385_rv = 0.0;

        let (assign23550_e34279,) = {
    if (((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 != 0.0)) {
        let assign23550_e34275: f64 = (var_nin * var_nin);
        let assign23550_e34277: f64 = (assign23550_e34275 / var_ndibot_i);
        (assign23550_e34277,)
    } else {
        (var_pnn0,)
    }
};
        var_pnn0 = assign23550_e34279;
        var_pnn0_rv = 0.0;

        let (assign23560_e34294,) = {
    if (((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 != 0.0)) {
        let assign23560_e34287: f64 = (var_nfabot_i / var_phitdinv);
        let assign23560_e34290: f64 = (var_ndibot_i / var_pnn0);
        let assign23560_e34291: f64 = (assign23560_e34290).ln();
        let assign23560_e34292: f64 = (assign23560_e34287 * assign23560_e34291);
        (assign23560_e34292,)
    } else {
        (var_vha1,)
    }
};
        var_vha1 = assign23560_e34294;
        var_vha1_rv = 0.0;

        let assign23570_e34297: f64 = if var_nfabot_i < p.p85 { 1.0 } else { 0.0 };
        var_guard388 = assign23570_e34297;
        var_guard388_rv = 0.0;

        let (assign23580_e34313, assign23580_e34313_d_n0, assign23580_e34313_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 != 0.0)) && (var_guard388 != 0.0)) {
        let assign23580_e34308: f64 = (var_v5 - var_vha1);
        let assign23580_e34309: f64 = (p.p86 * assign23580_e34308);
        let assign23580_e34311: f64 = (assign23580_e34309 + var_nfabot_i);
        (assign23580_e34311, 0.0, 0.0,)
    } else {
        (var_nja10, var_nja10_dn0, var_nja10_dn2,)
    }
};
        var_nja10 = assign23580_e34313;
        var_nja10_dn0 = assign23580_e34313_d_n0;
        var_nja10_dn2 = assign23580_e34313_d_n2;
        var_nja10_rv = 0.0;

        let (assign23590_e34327, assign23590_e34327_d_n0, assign23590_e34327_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 != 0.0)) && (var_guard388 != 0.0)) {
        let assign23590_e34324: f64 = (p.p86 * var_vha1);
        let assign23590_e34325: f64 = (var_nfabot_i - assign23590_e34324);
        (assign23590_e34325, 0.0, 0.0,)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn2,)
    }
};
        var_nj0 = assign23590_e34327;
        var_nj0_dn0 = assign23590_e34327_d_n0;
        var_nj0_dn2 = assign23590_e34327_d_n2;
        var_nj0_rv = 0.0;

        let (assign23600_e34341, assign23600_e34341_d_n0, assign23600_e34341_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 != 0.0)) && (var_guard388 != 0.0)) {
        let assign23600_e34337: f64 = (p.p85 - var_nja10);
        let assign23600_e34339: f64 = (assign23600_e34337 - 0.01);
        (assign23600_e34339, (-var_nja10_dn0), (-var_nja10_dn2),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2,)
    }
};
        var_tmf1 = assign23600_e34341;
        var_tmf1_dn0 = assign23600_e34341_d_n0;
        var_tmf1_dn2 = assign23600_e34341_d_n2;
        var_tmf1_rv = 0.0;

        let (assign23610_e34355, assign23610_e34355_d_n0, assign23610_e34355_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 != 0.0)) && (var_guard388 != 0.0)) {
        let assign23610_e34351: f64 = (4.0 * p.p85);
        let assign23610_e34353: f64 = (assign23610_e34351 * 0.01);
        (assign23610_e34353, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign23610_e34355;
        var_tmf2_dn0 = assign23610_e34355_d_n0;
        var_tmf2_dn2 = assign23610_e34355_d_n2;
        var_tmf2_rv = 0.0;

        *var_dfn_sl_slot = var_dfn_sl;
        *var_dfn_sl_dn0_slot = var_dfn_sl_dn0;
        *var_dfn_sl_dn2_slot = var_dfn_sl_dn2;
        *var_dfn_sl_rv_slot = var_dfn_sl_rv;
        *var_dfn_su_slot = var_dfn_su;
        *var_dfn_su_dn0_slot = var_dfn_su_dn0;
        *var_dfn_su_dn2_slot = var_dfn_su_dn2;
        *var_dfn_su_rv_slot = var_dfn_su_rv;
        *var_dnj1_dv_slot = var_dnj1_dv;
        *var_dnj1_dv_dn0_slot = var_dnj1_dv_dn0;
        *var_dnj1_dv_dn2_slot = var_dnj1_dv_dn2;
        *var_dnj1_dv_rv_slot = var_dnj1_dv_rv;
        *var_dvmax_over_phitd_dv_slot = var_dvmax_over_phitd_dv;
        *var_dvmax_over_phitd_dv_dn0_slot = var_dvmax_over_phitd_dv_dn0;
        *var_dvmax_over_phitd_dv_dn2_slot = var_dvmax_over_phitd_dv_dn2;
        *var_dvmax_over_phitd_dv_rv_slot = var_dvmax_over_phitd_dv_rv;
        *var_guard384_slot = var_guard384;
        *var_guard384_rv_slot = var_guard384_rv;
        *var_guard385_slot = var_guard385;
        *var_guard385_rv_slot = var_guard385_rv;
        *var_guard388_slot = var_guard388;
        *var_guard388_rv_slot = var_guard388_rv;
        *var_idmultbot_slot = var_idmultbot;
        *var_idmultbot_dn0_slot = var_idmultbot_dn0;
        *var_idmultbot_dn2_slot = var_idmultbot_dn2;
        *var_idmultbot_rv_slot = var_idmultbot_rv;
        *var_nj0_slot = var_nj0;
        *var_nj0_dn0_slot = var_nj0_dn0;
        *var_nj0_dn2_slot = var_nj0_dn2;
        *var_nj0_rv_slot = var_nj0_rv;
        *var_nj1_slot = var_nj1;
        *var_nj1_dn0_slot = var_nj1_dn0;
        *var_nj1_dn2_slot = var_nj1_dn2;
        *var_nj1_rv_slot = var_nj1_rv;
        *var_nja10_slot = var_nja10;
        *var_nja10_dn0_slot = var_nja10_dn0;
        *var_nja10_dn2_slot = var_nja10_dn2;
        *var_nja10_rv_slot = var_nja10_rv;
        *var_nja11_slot = var_nja11;
        *var_nja11_dn0_slot = var_nja11_dn0;
        *var_nja11_dn2_slot = var_nja11_dn2;
        *var_nja11_rv_slot = var_nja11_rv;
        *var_pnn0_slot = var_pnn0;
        *var_pnn0_rv_slot = var_pnn0_rv;
        *var_tmf1_slot = var_tmf1;
        *var_tmf1_dn0_slot = var_tmf1_dn0;
        *var_tmf1_dn2_slot = var_tmf1_dn2;
        *var_tmf1_rv_slot = var_tmf1_rv;
        *var_tmf2_slot = var_tmf2;
        *var_tmf2_dn0_slot = var_tmf2_dn0;
        *var_tmf2_dn2_slot = var_tmf2_dn2;
        *var_tmf2_rv_slot = var_tmf2_rv;
        *var_vha1_slot = var_vha1;
        *var_vha1_rv_slot = var_vha1_rv;
    }

    pub(super) fn stamp_reactive_block_25(
        p: &Parameters,
        var_guard31: f64,
        var_guard384: f64,
        var_guard385: f64,
        var_guard388: f64,
        var_ndisti_i: f64,
        var_nfabot_i: f64,
        var_nfasti_i: f64,
        var_nin: f64,
        var_phitdinv: f64,
        var_v5: f64,
        var_guard389_slot: &mut f64,
        var_guard389_rv_slot: &mut f64,
        var_guard390_slot: &mut f64,
        var_guard390_rv_slot: &mut f64,
        var_guard391_slot: &mut f64,
        var_guard391_rv_slot: &mut f64,
        var_idmultbot_slot: &mut f64,
        var_idmultbot_dn0_slot: &mut f64,
        var_idmultbot_dn2_slot: &mut f64,
        var_idmultbot_rv_slot: &mut f64,
        var_nj0_slot: &mut f64,
        var_nj0_dn0_slot: &mut f64,
        var_nj0_dn2_slot: &mut f64,
        var_nj0_rv_slot: &mut f64,
        var_nj1_slot: &mut f64,
        var_nj1_dn0_slot: &mut f64,
        var_nj1_dn2_slot: &mut f64,
        var_nj1_rv_slot: &mut f64,
        var_nja10_slot: &mut f64,
        var_nja10_dn0_slot: &mut f64,
        var_nja10_dn2_slot: &mut f64,
        var_nja10_rv_slot: &mut f64,
        var_nja11_slot: &mut f64,
        var_nja11_dn0_slot: &mut f64,
        var_nja11_dn2_slot: &mut f64,
        var_nja11_rv_slot: &mut f64,
        var_pnn0_slot: &mut f64,
        var_pnn0_rv_slot: &mut f64,
        var_tmf1_slot: &mut f64,
        var_tmf1_dn0_slot: &mut f64,
        var_tmf1_dn2_slot: &mut f64,
        var_tmf1_rv_slot: &mut f64,
        var_tmf2_slot: &mut f64,
        var_tmf2_dn0_slot: &mut f64,
        var_tmf2_dn2_slot: &mut f64,
        var_tmf2_rv_slot: &mut f64,
        var_vha1_slot: &mut f64,
        var_vha1_rv_slot: &mut f64,
    ) {
        let mut var_guard389: f64 = *var_guard389_slot;
        let mut var_guard389_rv: f64 = *var_guard389_rv_slot;
        let mut var_guard390: f64 = *var_guard390_slot;
        let mut var_guard390_rv: f64 = *var_guard390_rv_slot;
        let mut var_guard391: f64 = *var_guard391_slot;
        let mut var_guard391_rv: f64 = *var_guard391_rv_slot;
        let mut var_idmultbot: f64 = *var_idmultbot_slot;
        let mut var_idmultbot_dn0: f64 = *var_idmultbot_dn0_slot;
        let mut var_idmultbot_dn2: f64 = *var_idmultbot_dn2_slot;
        let mut var_idmultbot_rv: f64 = *var_idmultbot_rv_slot;
        let mut var_nj0: f64 = *var_nj0_slot;
        let mut var_nj0_dn0: f64 = *var_nj0_dn0_slot;
        let mut var_nj0_dn2: f64 = *var_nj0_dn2_slot;
        let mut var_nj0_rv: f64 = *var_nj0_rv_slot;
        let mut var_nj1: f64 = *var_nj1_slot;
        let mut var_nj1_dn0: f64 = *var_nj1_dn0_slot;
        let mut var_nj1_dn2: f64 = *var_nj1_dn2_slot;
        let mut var_nj1_rv: f64 = *var_nj1_rv_slot;
        let mut var_nja10: f64 = *var_nja10_slot;
        let mut var_nja10_dn0: f64 = *var_nja10_dn0_slot;
        let mut var_nja10_dn2: f64 = *var_nja10_dn2_slot;
        let mut var_nja10_rv: f64 = *var_nja10_rv_slot;
        let mut var_nja11: f64 = *var_nja11_slot;
        let mut var_nja11_dn0: f64 = *var_nja11_dn0_slot;
        let mut var_nja11_dn2: f64 = *var_nja11_dn2_slot;
        let mut var_nja11_rv: f64 = *var_nja11_rv_slot;
        let mut var_pnn0: f64 = *var_pnn0_slot;
        let mut var_pnn0_rv: f64 = *var_pnn0_rv_slot;
        let mut var_tmf1: f64 = *var_tmf1_slot;
        let mut var_tmf1_dn0: f64 = *var_tmf1_dn0_slot;
        let mut var_tmf1_dn2: f64 = *var_tmf1_dn2_slot;
        let mut var_tmf1_rv: f64 = *var_tmf1_rv_slot;
        let mut var_tmf2: f64 = *var_tmf2_slot;
        let mut var_tmf2_dn0: f64 = *var_tmf2_dn0_slot;
        let mut var_tmf2_dn2: f64 = *var_tmf2_dn2_slot;
        let mut var_tmf2_rv: f64 = *var_tmf2_rv_slot;
        let mut var_vha1: f64 = *var_vha1_slot;
        let mut var_vha1_rv: f64 = *var_vha1_rv_slot;

        let (assign23620_e34371, assign23620_e34371_d_n0, assign23620_e34371_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 != 0.0)) && (var_guard388 != 0.0)) {
        let (assign23620_e34369, assign23620_e34369_d_n0, assign23620_e34369_d_n2,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
            } else {
                let assign23620_e34368: f64 = (-var_tmf2);
                (assign23620_e34368, (-var_tmf2_dn0), (-var_tmf2_dn2),)
            }
        };
        (assign23620_e34369, assign23620_e34369_d_n0, assign23620_e34369_d_n2,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign23620_e34371;
        var_tmf2_dn0 = assign23620_e34371_d_n0;
        var_tmf2_dn2 = assign23620_e34371_d_n2;
        var_tmf2_rv = 0.0;

        let (assign23630_e34386, assign23630_e34386_d_n0, assign23630_e34386_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 != 0.0)) && (var_guard388 != 0.0)) {
        let assign23630_e34381: f64 = (var_tmf1 * var_tmf1);
        let assign23630_e34383: f64 = (assign23630_e34381 + var_tmf2);
        let assign23630_e34384: f64 = (assign23630_e34383).sqrt();
        (assign23630_e34384, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign23630_e34384)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign23630_e34384)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign23630_e34386;
        var_tmf2_dn0 = assign23630_e34386_d_n0;
        var_tmf2_dn2 = assign23630_e34386_d_n2;
        var_tmf2_rv = 0.0;

        let (assign23640_e34402, assign23640_e34402_d_n0, assign23640_e34402_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 != 0.0)) && (var_guard388 != 0.0)) {
        let assign23640_e34398: f64 = (var_tmf1 + var_tmf2);
        let assign23640_e34399: f64 = (0.5 * assign23640_e34398);
        let assign23640_e34400: f64 = (p.p85 - assign23640_e34399);
        (assign23640_e34400, (-(0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (-(0.5 * (var_tmf1_dn2 + var_tmf2_dn2))),)
    } else {
        (var_nja11, var_nja11_dn0, var_nja11_dn2,)
    }
};
        var_nja11 = assign23640_e34402;
        var_nja11_dn0 = assign23640_e34402_d_n0;
        var_nja11_dn2 = assign23640_e34402_d_n2;
        var_nja11_rv = 0.0;

        let (assign23650_e34416, assign23650_e34416_d_n0, assign23650_e34416_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 != 0.0)) && (var_guard388 != 0.0)) {
        let assign23650_e34412: f64 = (var_nja11 - var_nfabot_i);
        let assign23650_e34414: f64 = (assign23650_e34412 - 0.01);
        (assign23650_e34414, var_nja11_dn0, var_nja11_dn2,)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2,)
    }
};
        var_tmf1 = assign23650_e34416;
        var_tmf1_dn0 = assign23650_e34416_d_n0;
        var_tmf1_dn2 = assign23650_e34416_d_n2;
        var_tmf1_rv = 0.0;

        let (assign23660_e34430, assign23660_e34430_d_n0, assign23660_e34430_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 != 0.0)) && (var_guard388 != 0.0)) {
        let assign23660_e34426: f64 = (4.0 * var_nfabot_i);
        let assign23660_e34428: f64 = (assign23660_e34426 * 0.01);
        (assign23660_e34428, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign23660_e34430;
        var_tmf2_dn0 = assign23660_e34430_d_n0;
        var_tmf2_dn2 = assign23660_e34430_d_n2;
        var_tmf2_rv = 0.0;

        let (assign23670_e34446, assign23670_e34446_d_n0, assign23670_e34446_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 != 0.0)) && (var_guard388 != 0.0)) {
        let (assign23670_e34444, assign23670_e34444_d_n0, assign23670_e34444_d_n2,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
            } else {
                let assign23670_e34443: f64 = (-var_tmf2);
                (assign23670_e34443, (-var_tmf2_dn0), (-var_tmf2_dn2),)
            }
        };
        (assign23670_e34444, assign23670_e34444_d_n0, assign23670_e34444_d_n2,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign23670_e34446;
        var_tmf2_dn0 = assign23670_e34446_d_n0;
        var_tmf2_dn2 = assign23670_e34446_d_n2;
        var_tmf2_rv = 0.0;

        let (assign23680_e34461, assign23680_e34461_d_n0, assign23680_e34461_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 != 0.0)) && (var_guard388 != 0.0)) {
        let assign23680_e34456: f64 = (var_tmf1 * var_tmf1);
        let assign23680_e34458: f64 = (assign23680_e34456 + var_tmf2);
        let assign23680_e34459: f64 = (assign23680_e34458).sqrt();
        (assign23680_e34459, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign23680_e34459)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign23680_e34459)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign23680_e34461;
        var_tmf2_dn0 = assign23680_e34461_d_n0;
        var_tmf2_dn2 = assign23680_e34461_d_n2;
        var_tmf2_rv = 0.0;

        let (assign23690_e34477, assign23690_e34477_d_n0, assign23690_e34477_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 != 0.0)) && (var_guard388 != 0.0)) {
        let assign23690_e34473: f64 = (var_tmf1 + var_tmf2);
        let assign23690_e34474: f64 = (0.5 * assign23690_e34473);
        let assign23690_e34475: f64 = (var_nfabot_i + assign23690_e34474);
        (assign23690_e34475, (0.5 * (var_tmf1_dn0 + var_tmf2_dn0)), (0.5 * (var_tmf1_dn2 + var_tmf2_dn2)),)
    } else {
        (var_nj1, var_nj1_dn0, var_nj1_dn2,)
    }
};
        var_nj1 = assign23690_e34477;
        var_nj1_dn0 = assign23690_e34477_d_n0;
        var_nj1_dn2 = assign23690_e34477_d_n2;
        var_nj1_rv = 0.0;

        let (assign23700_e34491, assign23700_e34491_d_n0, assign23700_e34491_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 != 0.0)) && (var_guard388 != 0.0)) {
        let assign23700_e34487: f64 = (p.p85 - var_nj0);
        let assign23700_e34489: f64 = (assign23700_e34487 - 0.01);
        (assign23700_e34489, (-var_nj0_dn0), (-var_nj0_dn2),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2,)
    }
};
        var_tmf1 = assign23700_e34491;
        var_tmf1_dn0 = assign23700_e34491_d_n0;
        var_tmf1_dn2 = assign23700_e34491_d_n2;
        var_tmf1_rv = 0.0;

        let (assign23710_e34505, assign23710_e34505_d_n0, assign23710_e34505_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 != 0.0)) && (var_guard388 != 0.0)) {
        let assign23710_e34501: f64 = (4.0 * p.p85);
        let assign23710_e34503: f64 = (assign23710_e34501 * 0.01);
        (assign23710_e34503, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign23710_e34505;
        var_tmf2_dn0 = assign23710_e34505_d_n0;
        var_tmf2_dn2 = assign23710_e34505_d_n2;
        var_tmf2_rv = 0.0;

        let (assign23720_e34521, assign23720_e34521_d_n0, assign23720_e34521_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 != 0.0)) && (var_guard388 != 0.0)) {
        let (assign23720_e34519, assign23720_e34519_d_n0, assign23720_e34519_d_n2,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
            } else {
                let assign23720_e34518: f64 = (-var_tmf2);
                (assign23720_e34518, (-var_tmf2_dn0), (-var_tmf2_dn2),)
            }
        };
        (assign23720_e34519, assign23720_e34519_d_n0, assign23720_e34519_d_n2,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign23720_e34521;
        var_tmf2_dn0 = assign23720_e34521_d_n0;
        var_tmf2_dn2 = assign23720_e34521_d_n2;
        var_tmf2_rv = 0.0;

        let (assign23730_e34536, assign23730_e34536_d_n0, assign23730_e34536_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 != 0.0)) && (var_guard388 != 0.0)) {
        let assign23730_e34531: f64 = (var_tmf1 * var_tmf1);
        let assign23730_e34533: f64 = (assign23730_e34531 + var_tmf2);
        let assign23730_e34534: f64 = (assign23730_e34533).sqrt();
        (assign23730_e34534, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign23730_e34534)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign23730_e34534)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign23730_e34536;
        var_tmf2_dn0 = assign23730_e34536_d_n0;
        var_tmf2_dn2 = assign23730_e34536_d_n2;
        var_tmf2_rv = 0.0;

        let (assign23740_e34552, assign23740_e34552_d_n0, assign23740_e34552_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 != 0.0)) && (var_guard388 != 0.0)) {
        let assign23740_e34548: f64 = (var_tmf1 + var_tmf2);
        let assign23740_e34549: f64 = (0.5 * assign23740_e34548);
        let assign23740_e34550: f64 = (p.p85 - assign23740_e34549);
        (assign23740_e34550, (-(0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (-(0.5 * (var_tmf1_dn2 + var_tmf2_dn2))),)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn2,)
    }
};
        var_nj0 = assign23740_e34552;
        var_nj0_dn0 = assign23740_e34552_d_n0;
        var_nj0_dn2 = assign23740_e34552_d_n2;
        var_nj0_rv = 0.0;

        let (assign23750_e34566, assign23750_e34566_d_n0, assign23750_e34566_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 != 0.0)) && (var_guard388 != 0.0)) {
        let assign23750_e34562: f64 = (var_nj0 - var_nfabot_i);
        let assign23750_e34564: f64 = (assign23750_e34562 - 0.01);
        (assign23750_e34564, var_nj0_dn0, var_nj0_dn2,)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2,)
    }
};
        var_tmf1 = assign23750_e34566;
        var_tmf1_dn0 = assign23750_e34566_d_n0;
        var_tmf1_dn2 = assign23750_e34566_d_n2;
        var_tmf1_rv = 0.0;

        let (assign23760_e34580, assign23760_e34580_d_n0, assign23760_e34580_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 != 0.0)) && (var_guard388 != 0.0)) {
        let assign23760_e34576: f64 = (4.0 * var_nfabot_i);
        let assign23760_e34578: f64 = (assign23760_e34576 * 0.01);
        (assign23760_e34578, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign23760_e34580;
        var_tmf2_dn0 = assign23760_e34580_d_n0;
        var_tmf2_dn2 = assign23760_e34580_d_n2;
        var_tmf2_rv = 0.0;

        let (assign23770_e34596, assign23770_e34596_d_n0, assign23770_e34596_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 != 0.0)) && (var_guard388 != 0.0)) {
        let (assign23770_e34594, assign23770_e34594_d_n0, assign23770_e34594_d_n2,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
            } else {
                let assign23770_e34593: f64 = (-var_tmf2);
                (assign23770_e34593, (-var_tmf2_dn0), (-var_tmf2_dn2),)
            }
        };
        (assign23770_e34594, assign23770_e34594_d_n0, assign23770_e34594_d_n2,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign23770_e34596;
        var_tmf2_dn0 = assign23770_e34596_d_n0;
        var_tmf2_dn2 = assign23770_e34596_d_n2;
        var_tmf2_rv = 0.0;

        let (assign23780_e34611, assign23780_e34611_d_n0, assign23780_e34611_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 != 0.0)) && (var_guard388 != 0.0)) {
        let assign23780_e34606: f64 = (var_tmf1 * var_tmf1);
        let assign23780_e34608: f64 = (assign23780_e34606 + var_tmf2);
        let assign23780_e34609: f64 = (assign23780_e34608).sqrt();
        (assign23780_e34609, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign23780_e34609)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign23780_e34609)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign23780_e34611;
        var_tmf2_dn0 = assign23780_e34611_d_n0;
        var_tmf2_dn2 = assign23780_e34611_d_n2;
        var_tmf2_rv = 0.0;

        let (assign23790_e34627, assign23790_e34627_d_n0, assign23790_e34627_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 != 0.0)) && (var_guard388 != 0.0)) {
        let assign23790_e34623: f64 = (var_tmf1 + var_tmf2);
        let assign23790_e34624: f64 = (0.5 * assign23790_e34623);
        let assign23790_e34625: f64 = (var_nfabot_i + assign23790_e34624);
        (assign23790_e34625, (0.5 * (var_tmf1_dn0 + var_tmf2_dn0)), (0.5 * (var_tmf1_dn2 + var_tmf2_dn2)),)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn2,)
    }
};
        var_nj0 = assign23790_e34627;
        var_nj0_dn0 = assign23790_e34627_d_n0;
        var_nj0_dn2 = assign23790_e34627_d_n2;
        var_nj0_rv = 0.0;

        let (assign23800_e34638, assign23800_e34638_d_n0, assign23800_e34638_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 != 0.0)) && (var_guard388 == 0.0)) {
        (var_nfabot_i, 0.0, 0.0,)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn2,)
    }
};
        var_nj0 = assign23800_e34638;
        var_nj0_dn0 = assign23800_e34638_d_n0;
        var_nj0_dn2 = assign23800_e34638_d_n2;
        var_nj0_rv = 0.0;

        let (assign23810_e34649, assign23810_e34649_d_n0, assign23810_e34649_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 != 0.0)) && (var_guard388 == 0.0)) {
        (var_nfabot_i, 0.0, 0.0,)
    } else {
        (var_nj1, var_nj1_dn0, var_nj1_dn2,)
    }
};
        var_nj1 = assign23810_e34649;
        var_nj1_dn0 = assign23810_e34649_d_n0;
        var_nj1_dn2 = assign23810_e34649_d_n2;
        var_nj1_rv = 0.0;

        let assign23820_e34653: f64 = (var_v5 / var_nj1);
        let assign23820_e34657: f64 = (var_nj1 - var_nj0);
        let assign23820_e34658: f64 = (var_vha1 * assign23820_e34657);
        let assign23820_e34661: f64 = (var_nj0 * p.p85);
        let assign23820_e34662: f64 = (assign23820_e34658 / assign23820_e34661);
        let assign23820_e34663: f64 = (assign23820_e34653 + assign23820_e34662);
        let assign23820_e34664: f64 = (var_phitdinv * assign23820_e34663);
        let assign23820_e34665: f64 = (assign23820_e34664).abs();
        let assign23820_e34667: f64 = if assign23820_e34665 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard389 = assign23820_e34667;
        var_guard389_rv = 0.0;

        let (assign23830_e34692, assign23830_e34692_d_n0, assign23830_e34692_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 != 0.0)) && (var_guard389 != 0.0)) {
        let assign23830_e34678: f64 = (var_v5 / var_nj1);
        let assign23830_e34682: f64 = (var_nj1 - var_nj0);
        let assign23830_e34683: f64 = (var_vha1 * assign23830_e34682);
        let assign23830_e34686: f64 = (var_nj0 * p.p85);
        let assign23830_e34687: f64 = (assign23830_e34683 / assign23830_e34686);
        let assign23830_e34688: f64 = (assign23830_e34678 + assign23830_e34687);
        let assign23830_e34689: f64 = (var_phitdinv * assign23830_e34688);
        let assign23830_e34690: f64 = (assign23830_e34689).exp();
        (assign23830_e34690, (assign23830_e34690 * (var_phitdinv * ((-((var_v5 * var_nj1_dn0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn0 - var_nj0_dn0)) * assign23830_e34686) - (assign23830_e34683 * (var_nj0_dn0 * p.p85))) / (assign23830_e34686 * assign23830_e34686))))), (assign23830_e34690 * (var_phitdinv * ((-((var_v5 * var_nj1_dn2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn2 - var_nj0_dn2)) * assign23830_e34686) - (assign23830_e34683 * (var_nj0_dn2 * p.p85))) / (assign23830_e34686 * assign23830_e34686))))),)
    } else {
        (var_idmultbot, var_idmultbot_dn0, var_idmultbot_dn2,)
    }
};
        var_idmultbot = assign23830_e34692;
        var_idmultbot_dn0 = assign23830_e34692_d_n0;
        var_idmultbot_dn2 = assign23830_e34692_d_n2;
        var_idmultbot_rv = 0.0;

        let assign23840_e34696: f64 = (var_v5 / var_nj1);
        let assign23840_e34700: f64 = (var_nj1 - var_nj0);
        let assign23840_e34701: f64 = (var_vha1 * assign23840_e34700);
        let assign23840_e34704: f64 = (var_nj0 * p.p85);
        let assign23840_e34705: f64 = (assign23840_e34701 / assign23840_e34704);
        let assign23840_e34706: f64 = (assign23840_e34696 + assign23840_e34705);
        let assign23840_e34707: f64 = (var_phitdinv * assign23840_e34706);
        let assign23840_e34709: f64 = (-230.25850929940458);
        let assign23840_e34710: f64 = if assign23840_e34707 < assign23840_e34709 { 1.0 } else { 0.0 };
        var_guard390 = assign23840_e34710;
        var_guard390_rv = 0.0;

        let (assign23850_e34790, assign23850_e34790_d_n0, assign23850_e34790_d_n2,) = {
    if (((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 != 0.0)) && (var_guard389 == 0.0)) && (var_guard390 != 0.0)) {
        let assign23850_e34724: f64 = (-230.25850929940458);
        let assign23850_e34728: f64 = (var_v5 / var_nj1);
        let assign23850_e34732: f64 = (var_nj1 - var_nj0);
        let assign23850_e34733: f64 = (var_vha1 * assign23850_e34732);
        let assign23850_e34736: f64 = (var_nj0 * p.p85);
        let assign23850_e34737: f64 = (assign23850_e34733 / assign23850_e34736);
        let assign23850_e34738: f64 = (assign23850_e34728 + assign23850_e34737);
        let assign23850_e34739: f64 = (var_phitdinv * assign23850_e34738);
        let assign23850_e34740: f64 = (assign23850_e34724 - assign23850_e34739);
        let assign23850_e34744: f64 = (-230.25850929940458);
        let assign23850_e34748: f64 = (var_v5 / var_nj1);
        let assign23850_e34752: f64 = (var_nj1 - var_nj0);
        let assign23850_e34753: f64 = (var_vha1 * assign23850_e34752);
        let assign23850_e34756: f64 = (var_nj0 * p.p85);
        let assign23850_e34757: f64 = (assign23850_e34753 / assign23850_e34756);
        let assign23850_e34758: f64 = (assign23850_e34748 + assign23850_e34757);
        let assign23850_e34759: f64 = (var_phitdinv * assign23850_e34758);
        let assign23850_e34760: f64 = (assign23850_e34744 - assign23850_e34759);
        let assign23850_e34763: f64 = (-230.25850929940458);
        let assign23850_e34767: f64 = (var_v5 / var_nj1);
        let assign23850_e34771: f64 = (var_nj1 - var_nj0);
        let assign23850_e34772: f64 = (var_vha1 * assign23850_e34771);
        let assign23850_e34775: f64 = (var_nj0 * p.p85);
        let assign23850_e34776: f64 = (assign23850_e34772 / assign23850_e34775);
        let assign23850_e34777: f64 = (assign23850_e34767 + assign23850_e34776);
        let assign23850_e34778: f64 = (var_phitdinv * assign23850_e34777);
        let assign23850_e34779: f64 = (assign23850_e34763 - assign23850_e34778);
        let assign23850_e34781: f64 = (assign23850_e34779 * 0.3333333333333333);
        let assign23850_e34782: f64 = (1.0 + assign23850_e34781);
        let assign23850_e34783: f64 = (assign23850_e34760 * assign23850_e34782);
        let assign23850_e34784: f64 = (0.5 * assign23850_e34783);
        let assign23850_e34785: f64 = (1.0 + assign23850_e34784);
        let assign23850_e34786: f64 = (assign23850_e34740 * assign23850_e34785);
        let assign23850_e34787: f64 = (1.0 + assign23850_e34786);
        let assign23850_e34788: f64 = (1e-100 / assign23850_e34787);
        (assign23850_e34788, (-((1e-100 * (((-(var_phitdinv * ((-((var_v5 * var_nj1_dn0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn0 - var_nj0_dn0)) * assign23850_e34736) - (assign23850_e34733 * (var_nj0_dn0 * p.p85))) / (assign23850_e34736 * assign23850_e34736))))) * assign23850_e34785) + (assign23850_e34740 * (0.5 * (((-(var_phitdinv * ((-((var_v5 * var_nj1_dn0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn0 - var_nj0_dn0)) * assign23850_e34756) - (assign23850_e34753 * (var_nj0_dn0 * p.p85))) / (assign23850_e34756 * assign23850_e34756))))) * assign23850_e34782) + (assign23850_e34760 * ((-(var_phitdinv * ((-((var_v5 * var_nj1_dn0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn0 - var_nj0_dn0)) * assign23850_e34775) - (assign23850_e34772 * (var_nj0_dn0 * p.p85))) / (assign23850_e34775 * assign23850_e34775))))) * 0.3333333333333333))))))) / (assign23850_e34787 * assign23850_e34787))), (-((1e-100 * (((-(var_phitdinv * ((-((var_v5 * var_nj1_dn2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn2 - var_nj0_dn2)) * assign23850_e34736) - (assign23850_e34733 * (var_nj0_dn2 * p.p85))) / (assign23850_e34736 * assign23850_e34736))))) * assign23850_e34785) + (assign23850_e34740 * (0.5 * (((-(var_phitdinv * ((-((var_v5 * var_nj1_dn2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn2 - var_nj0_dn2)) * assign23850_e34756) - (assign23850_e34753 * (var_nj0_dn2 * p.p85))) / (assign23850_e34756 * assign23850_e34756))))) * assign23850_e34782) + (assign23850_e34760 * ((-(var_phitdinv * ((-((var_v5 * var_nj1_dn2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn2 - var_nj0_dn2)) * assign23850_e34775) - (assign23850_e34772 * (var_nj0_dn2 * p.p85))) / (assign23850_e34775 * assign23850_e34775))))) * 0.3333333333333333))))))) / (assign23850_e34787 * assign23850_e34787))),)
    } else {
        (var_idmultbot, var_idmultbot_dn0, var_idmultbot_dn2,)
    }
};
        var_idmultbot = assign23850_e34790;
        var_idmultbot_dn0 = assign23850_e34790_d_n0;
        var_idmultbot_dn2 = assign23850_e34790_d_n2;
        var_idmultbot_rv = 0.0;

        let (assign23860_e34868, assign23860_e34868_d_n0, assign23860_e34868_d_n2,) = {
    if (((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 != 0.0)) && (var_guard389 == 0.0)) && (var_guard390 == 0.0)) {
        let assign23860_e34807: f64 = (var_v5 / var_nj1);
        let assign23860_e34811: f64 = (var_nj1 - var_nj0);
        let assign23860_e34812: f64 = (var_vha1 * assign23860_e34811);
        let assign23860_e34815: f64 = (var_nj0 * p.p85);
        let assign23860_e34816: f64 = (assign23860_e34812 / assign23860_e34815);
        let assign23860_e34817: f64 = (assign23860_e34807 + assign23860_e34816);
        let assign23860_e34818: f64 = (var_phitdinv * assign23860_e34817);
        let assign23860_e34820: f64 = (assign23860_e34818 - 230.25850929940458);
        let assign23860_e34826: f64 = (var_v5 / var_nj1);
        let assign23860_e34830: f64 = (var_nj1 - var_nj0);
        let assign23860_e34831: f64 = (var_vha1 * assign23860_e34830);
        let assign23860_e34834: f64 = (var_nj0 * p.p85);
        let assign23860_e34835: f64 = (assign23860_e34831 / assign23860_e34834);
        let assign23860_e34836: f64 = (assign23860_e34826 + assign23860_e34835);
        let assign23860_e34837: f64 = (var_phitdinv * assign23860_e34836);
        let assign23860_e34839: f64 = (assign23860_e34837 - 230.25850929940458);
        let assign23860_e34844: f64 = (var_v5 / var_nj1);
        let assign23860_e34848: f64 = (var_nj1 - var_nj0);
        let assign23860_e34849: f64 = (var_vha1 * assign23860_e34848);
        let assign23860_e34852: f64 = (var_nj0 * p.p85);
        let assign23860_e34853: f64 = (assign23860_e34849 / assign23860_e34852);
        let assign23860_e34854: f64 = (assign23860_e34844 + assign23860_e34853);
        let assign23860_e34855: f64 = (var_phitdinv * assign23860_e34854);
        let assign23860_e34857: f64 = (assign23860_e34855 - 230.25850929940458);
        let assign23860_e34859: f64 = (assign23860_e34857 * 0.3333333333333333);
        let assign23860_e34860: f64 = (1.0 + assign23860_e34859);
        let assign23860_e34861: f64 = (assign23860_e34839 * assign23860_e34860);
        let assign23860_e34862: f64 = (0.5 * assign23860_e34861);
        let assign23860_e34863: f64 = (1.0 + assign23860_e34862);
        let assign23860_e34864: f64 = (assign23860_e34820 * assign23860_e34863);
        let assign23860_e34865: f64 = (1.0 + assign23860_e34864);
        let assign23860_e34866: f64 = (1e100 * assign23860_e34865);
        (assign23860_e34866, (1e100 * (((var_phitdinv * ((-((var_v5 * var_nj1_dn0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn0 - var_nj0_dn0)) * assign23860_e34815) - (assign23860_e34812 * (var_nj0_dn0 * p.p85))) / (assign23860_e34815 * assign23860_e34815)))) * assign23860_e34863) + (assign23860_e34820 * (0.5 * (((var_phitdinv * ((-((var_v5 * var_nj1_dn0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn0 - var_nj0_dn0)) * assign23860_e34834) - (assign23860_e34831 * (var_nj0_dn0 * p.p85))) / (assign23860_e34834 * assign23860_e34834)))) * assign23860_e34860) + (assign23860_e34839 * ((var_phitdinv * ((-((var_v5 * var_nj1_dn0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn0 - var_nj0_dn0)) * assign23860_e34852) - (assign23860_e34849 * (var_nj0_dn0 * p.p85))) / (assign23860_e34852 * assign23860_e34852)))) * 0.3333333333333333))))))), (1e100 * (((var_phitdinv * ((-((var_v5 * var_nj1_dn2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn2 - var_nj0_dn2)) * assign23860_e34815) - (assign23860_e34812 * (var_nj0_dn2 * p.p85))) / (assign23860_e34815 * assign23860_e34815)))) * assign23860_e34863) + (assign23860_e34820 * (0.5 * (((var_phitdinv * ((-((var_v5 * var_nj1_dn2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn2 - var_nj0_dn2)) * assign23860_e34834) - (assign23860_e34831 * (var_nj0_dn2 * p.p85))) / (assign23860_e34834 * assign23860_e34834)))) * assign23860_e34860) + (assign23860_e34839 * ((var_phitdinv * ((-((var_v5 * var_nj1_dn2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn2 - var_nj0_dn2)) * assign23860_e34852) - (assign23860_e34849 * (var_nj0_dn2 * p.p85))) / (assign23860_e34852 * assign23860_e34852)))) * 0.3333333333333333))))))),)
    } else {
        (var_idmultbot, var_idmultbot_dn0, var_idmultbot_dn2,)
    }
};
        var_idmultbot = assign23860_e34868;
        var_idmultbot_dn0 = assign23860_e34868_d_n0;
        var_idmultbot_dn2 = assign23860_e34868_d_n2;
        var_idmultbot_rv = 0.0;

        let (assign23870_e34880,) = {
    if (((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 != 0.0)) {
        let assign23870_e34876: f64 = (var_nin * var_nin);
        let assign23870_e34878: f64 = (assign23870_e34876 / var_ndisti_i);
        (assign23870_e34878,)
    } else {
        (var_pnn0,)
    }
};
        var_pnn0 = assign23870_e34880;
        var_pnn0_rv = 0.0;

        let (assign23880_e34895,) = {
    if (((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 != 0.0)) {
        let assign23880_e34888: f64 = (var_nfasti_i / var_phitdinv);
        let assign23880_e34891: f64 = (var_ndisti_i / var_pnn0);
        let assign23880_e34892: f64 = (assign23880_e34891).ln();
        let assign23880_e34893: f64 = (assign23880_e34888 * assign23880_e34892);
        (assign23880_e34893,)
    } else {
        (var_vha1,)
    }
};
        var_vha1 = assign23880_e34895;
        var_vha1_rv = 0.0;

        let assign23890_e34898: f64 = if var_nfasti_i < p.p85 { 1.0 } else { 0.0 };
        var_guard391 = assign23890_e34898;
        var_guard391_rv = 0.0;

        let (assign23900_e34914, assign23900_e34914_d_n0, assign23900_e34914_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 != 0.0)) && (var_guard391 != 0.0)) {
        let assign23900_e34909: f64 = (var_v5 - var_vha1);
        let assign23900_e34910: f64 = (p.p86 * assign23900_e34909);
        let assign23900_e34912: f64 = (assign23900_e34910 + var_nfasti_i);
        (assign23900_e34912, 0.0, 0.0,)
    } else {
        (var_nja10, var_nja10_dn0, var_nja10_dn2,)
    }
};
        var_nja10 = assign23900_e34914;
        var_nja10_dn0 = assign23900_e34914_d_n0;
        var_nja10_dn2 = assign23900_e34914_d_n2;
        var_nja10_rv = 0.0;

        let (assign23910_e34928, assign23910_e34928_d_n0, assign23910_e34928_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 != 0.0)) && (var_guard391 != 0.0)) {
        let assign23910_e34925: f64 = (p.p86 * var_vha1);
        let assign23910_e34926: f64 = (var_nfasti_i - assign23910_e34925);
        (assign23910_e34926, 0.0, 0.0,)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn2,)
    }
};
        var_nj0 = assign23910_e34928;
        var_nj0_dn0 = assign23910_e34928_d_n0;
        var_nj0_dn2 = assign23910_e34928_d_n2;
        var_nj0_rv = 0.0;

        *var_guard389_slot = var_guard389;
        *var_guard389_rv_slot = var_guard389_rv;
        *var_guard390_slot = var_guard390;
        *var_guard390_rv_slot = var_guard390_rv;
        *var_guard391_slot = var_guard391;
        *var_guard391_rv_slot = var_guard391_rv;
        *var_idmultbot_slot = var_idmultbot;
        *var_idmultbot_dn0_slot = var_idmultbot_dn0;
        *var_idmultbot_dn2_slot = var_idmultbot_dn2;
        *var_idmultbot_rv_slot = var_idmultbot_rv;
        *var_nj0_slot = var_nj0;
        *var_nj0_dn0_slot = var_nj0_dn0;
        *var_nj0_dn2_slot = var_nj0_dn2;
        *var_nj0_rv_slot = var_nj0_rv;
        *var_nj1_slot = var_nj1;
        *var_nj1_dn0_slot = var_nj1_dn0;
        *var_nj1_dn2_slot = var_nj1_dn2;
        *var_nj1_rv_slot = var_nj1_rv;
        *var_nja10_slot = var_nja10;
        *var_nja10_dn0_slot = var_nja10_dn0;
        *var_nja10_dn2_slot = var_nja10_dn2;
        *var_nja10_rv_slot = var_nja10_rv;
        *var_nja11_slot = var_nja11;
        *var_nja11_dn0_slot = var_nja11_dn0;
        *var_nja11_dn2_slot = var_nja11_dn2;
        *var_nja11_rv_slot = var_nja11_rv;
        *var_pnn0_slot = var_pnn0;
        *var_pnn0_rv_slot = var_pnn0_rv;
        *var_tmf1_slot = var_tmf1;
        *var_tmf1_dn0_slot = var_tmf1_dn0;
        *var_tmf1_dn2_slot = var_tmf1_dn2;
        *var_tmf1_rv_slot = var_tmf1_rv;
        *var_tmf2_slot = var_tmf2;
        *var_tmf2_dn0_slot = var_tmf2_dn0;
        *var_tmf2_dn2_slot = var_tmf2_dn2;
        *var_tmf2_rv_slot = var_tmf2_rv;
        *var_vha1_slot = var_vha1;
        *var_vha1_rv_slot = var_vha1_rv;
    }

    pub(super) fn stamp_reactive_block_26(
        p: &Parameters,
        var_guard31: f64,
        var_guard384: f64,
        var_guard385: f64,
        var_guard391: f64,
        var_ndigat_i: f64,
        var_nfagat_i: f64,
        var_nfasti_i: f64,
        var_nin: f64,
        var_phitdinv: f64,
        var_v5: f64,
        var_guard394_slot: &mut f64,
        var_guard394_rv_slot: &mut f64,
        var_nj0_slot: &mut f64,
        var_nj0_dn0_slot: &mut f64,
        var_nj0_dn2_slot: &mut f64,
        var_nj0_rv_slot: &mut f64,
        var_nj1_slot: &mut f64,
        var_nj1_dn0_slot: &mut f64,
        var_nj1_dn2_slot: &mut f64,
        var_nj1_rv_slot: &mut f64,
        var_nja10_slot: &mut f64,
        var_nja10_dn0_slot: &mut f64,
        var_nja10_dn2_slot: &mut f64,
        var_nja10_rv_slot: &mut f64,
        var_nja11_slot: &mut f64,
        var_nja11_dn0_slot: &mut f64,
        var_nja11_dn2_slot: &mut f64,
        var_nja11_rv_slot: &mut f64,
        var_pnn0_slot: &mut f64,
        var_pnn0_rv_slot: &mut f64,
        var_tmf1_slot: &mut f64,
        var_tmf1_dn0_slot: &mut f64,
        var_tmf1_dn2_slot: &mut f64,
        var_tmf1_rv_slot: &mut f64,
        var_tmf2_slot: &mut f64,
        var_tmf2_dn0_slot: &mut f64,
        var_tmf2_dn2_slot: &mut f64,
        var_tmf2_rv_slot: &mut f64,
        var_vha1_slot: &mut f64,
        var_vha1_rv_slot: &mut f64,
    ) {
        let mut var_guard394: f64 = *var_guard394_slot;
        let mut var_guard394_rv: f64 = *var_guard394_rv_slot;
        let mut var_nj0: f64 = *var_nj0_slot;
        let mut var_nj0_dn0: f64 = *var_nj0_dn0_slot;
        let mut var_nj0_dn2: f64 = *var_nj0_dn2_slot;
        let mut var_nj0_rv: f64 = *var_nj0_rv_slot;
        let mut var_nj1: f64 = *var_nj1_slot;
        let mut var_nj1_dn0: f64 = *var_nj1_dn0_slot;
        let mut var_nj1_dn2: f64 = *var_nj1_dn2_slot;
        let mut var_nj1_rv: f64 = *var_nj1_rv_slot;
        let mut var_nja10: f64 = *var_nja10_slot;
        let mut var_nja10_dn0: f64 = *var_nja10_dn0_slot;
        let mut var_nja10_dn2: f64 = *var_nja10_dn2_slot;
        let mut var_nja10_rv: f64 = *var_nja10_rv_slot;
        let mut var_nja11: f64 = *var_nja11_slot;
        let mut var_nja11_dn0: f64 = *var_nja11_dn0_slot;
        let mut var_nja11_dn2: f64 = *var_nja11_dn2_slot;
        let mut var_nja11_rv: f64 = *var_nja11_rv_slot;
        let mut var_pnn0: f64 = *var_pnn0_slot;
        let mut var_pnn0_rv: f64 = *var_pnn0_rv_slot;
        let mut var_tmf1: f64 = *var_tmf1_slot;
        let mut var_tmf1_dn0: f64 = *var_tmf1_dn0_slot;
        let mut var_tmf1_dn2: f64 = *var_tmf1_dn2_slot;
        let mut var_tmf1_rv: f64 = *var_tmf1_rv_slot;
        let mut var_tmf2: f64 = *var_tmf2_slot;
        let mut var_tmf2_dn0: f64 = *var_tmf2_dn0_slot;
        let mut var_tmf2_dn2: f64 = *var_tmf2_dn2_slot;
        let mut var_tmf2_rv: f64 = *var_tmf2_rv_slot;
        let mut var_vha1: f64 = *var_vha1_slot;
        let mut var_vha1_rv: f64 = *var_vha1_rv_slot;

        let (assign23920_e34942, assign23920_e34942_d_n0, assign23920_e34942_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 != 0.0)) && (var_guard391 != 0.0)) {
        let assign23920_e34938: f64 = (p.p85 - var_nja10);
        let assign23920_e34940: f64 = (assign23920_e34938 - 0.01);
        (assign23920_e34940, (-var_nja10_dn0), (-var_nja10_dn2),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2,)
    }
};
        var_tmf1 = assign23920_e34942;
        var_tmf1_dn0 = assign23920_e34942_d_n0;
        var_tmf1_dn2 = assign23920_e34942_d_n2;
        var_tmf1_rv = 0.0;

        let (assign23930_e34956, assign23930_e34956_d_n0, assign23930_e34956_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 != 0.0)) && (var_guard391 != 0.0)) {
        let assign23930_e34952: f64 = (4.0 * p.p85);
        let assign23930_e34954: f64 = (assign23930_e34952 * 0.01);
        (assign23930_e34954, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign23930_e34956;
        var_tmf2_dn0 = assign23930_e34956_d_n0;
        var_tmf2_dn2 = assign23930_e34956_d_n2;
        var_tmf2_rv = 0.0;

        let (assign23940_e34972, assign23940_e34972_d_n0, assign23940_e34972_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 != 0.0)) && (var_guard391 != 0.0)) {
        let (assign23940_e34970, assign23940_e34970_d_n0, assign23940_e34970_d_n2,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
            } else {
                let assign23940_e34969: f64 = (-var_tmf2);
                (assign23940_e34969, (-var_tmf2_dn0), (-var_tmf2_dn2),)
            }
        };
        (assign23940_e34970, assign23940_e34970_d_n0, assign23940_e34970_d_n2,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign23940_e34972;
        var_tmf2_dn0 = assign23940_e34972_d_n0;
        var_tmf2_dn2 = assign23940_e34972_d_n2;
        var_tmf2_rv = 0.0;

        let (assign23950_e34987, assign23950_e34987_d_n0, assign23950_e34987_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 != 0.0)) && (var_guard391 != 0.0)) {
        let assign23950_e34982: f64 = (var_tmf1 * var_tmf1);
        let assign23950_e34984: f64 = (assign23950_e34982 + var_tmf2);
        let assign23950_e34985: f64 = (assign23950_e34984).sqrt();
        (assign23950_e34985, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign23950_e34985)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign23950_e34985)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign23950_e34987;
        var_tmf2_dn0 = assign23950_e34987_d_n0;
        var_tmf2_dn2 = assign23950_e34987_d_n2;
        var_tmf2_rv = 0.0;

        let (assign23960_e35003, assign23960_e35003_d_n0, assign23960_e35003_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 != 0.0)) && (var_guard391 != 0.0)) {
        let assign23960_e34999: f64 = (var_tmf1 + var_tmf2);
        let assign23960_e35000: f64 = (0.5 * assign23960_e34999);
        let assign23960_e35001: f64 = (p.p85 - assign23960_e35000);
        (assign23960_e35001, (-(0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (-(0.5 * (var_tmf1_dn2 + var_tmf2_dn2))),)
    } else {
        (var_nja11, var_nja11_dn0, var_nja11_dn2,)
    }
};
        var_nja11 = assign23960_e35003;
        var_nja11_dn0 = assign23960_e35003_d_n0;
        var_nja11_dn2 = assign23960_e35003_d_n2;
        var_nja11_rv = 0.0;

        let (assign23970_e35017, assign23970_e35017_d_n0, assign23970_e35017_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 != 0.0)) && (var_guard391 != 0.0)) {
        let assign23970_e35013: f64 = (var_nja11 - var_nfasti_i);
        let assign23970_e35015: f64 = (assign23970_e35013 - 0.01);
        (assign23970_e35015, var_nja11_dn0, var_nja11_dn2,)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2,)
    }
};
        var_tmf1 = assign23970_e35017;
        var_tmf1_dn0 = assign23970_e35017_d_n0;
        var_tmf1_dn2 = assign23970_e35017_d_n2;
        var_tmf1_rv = 0.0;

        let (assign23980_e35031, assign23980_e35031_d_n0, assign23980_e35031_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 != 0.0)) && (var_guard391 != 0.0)) {
        let assign23980_e35027: f64 = (4.0 * var_nfasti_i);
        let assign23980_e35029: f64 = (assign23980_e35027 * 0.01);
        (assign23980_e35029, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign23980_e35031;
        var_tmf2_dn0 = assign23980_e35031_d_n0;
        var_tmf2_dn2 = assign23980_e35031_d_n2;
        var_tmf2_rv = 0.0;

        let (assign23990_e35047, assign23990_e35047_d_n0, assign23990_e35047_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 != 0.0)) && (var_guard391 != 0.0)) {
        let (assign23990_e35045, assign23990_e35045_d_n0, assign23990_e35045_d_n2,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
            } else {
                let assign23990_e35044: f64 = (-var_tmf2);
                (assign23990_e35044, (-var_tmf2_dn0), (-var_tmf2_dn2),)
            }
        };
        (assign23990_e35045, assign23990_e35045_d_n0, assign23990_e35045_d_n2,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign23990_e35047;
        var_tmf2_dn0 = assign23990_e35047_d_n0;
        var_tmf2_dn2 = assign23990_e35047_d_n2;
        var_tmf2_rv = 0.0;

        let (assign24000_e35062, assign24000_e35062_d_n0, assign24000_e35062_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 != 0.0)) && (var_guard391 != 0.0)) {
        let assign24000_e35057: f64 = (var_tmf1 * var_tmf1);
        let assign24000_e35059: f64 = (assign24000_e35057 + var_tmf2);
        let assign24000_e35060: f64 = (assign24000_e35059).sqrt();
        (assign24000_e35060, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign24000_e35060)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign24000_e35060)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign24000_e35062;
        var_tmf2_dn0 = assign24000_e35062_d_n0;
        var_tmf2_dn2 = assign24000_e35062_d_n2;
        var_tmf2_rv = 0.0;

        let (assign24010_e35078, assign24010_e35078_d_n0, assign24010_e35078_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 != 0.0)) && (var_guard391 != 0.0)) {
        let assign24010_e35074: f64 = (var_tmf1 + var_tmf2);
        let assign24010_e35075: f64 = (0.5 * assign24010_e35074);
        let assign24010_e35076: f64 = (var_nfasti_i + assign24010_e35075);
        (assign24010_e35076, (0.5 * (var_tmf1_dn0 + var_tmf2_dn0)), (0.5 * (var_tmf1_dn2 + var_tmf2_dn2)),)
    } else {
        (var_nj1, var_nj1_dn0, var_nj1_dn2,)
    }
};
        var_nj1 = assign24010_e35078;
        var_nj1_dn0 = assign24010_e35078_d_n0;
        var_nj1_dn2 = assign24010_e35078_d_n2;
        var_nj1_rv = 0.0;

        let (assign24020_e35092, assign24020_e35092_d_n0, assign24020_e35092_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 != 0.0)) && (var_guard391 != 0.0)) {
        let assign24020_e35088: f64 = (p.p85 - var_nj0);
        let assign24020_e35090: f64 = (assign24020_e35088 - 0.01);
        (assign24020_e35090, (-var_nj0_dn0), (-var_nj0_dn2),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2,)
    }
};
        var_tmf1 = assign24020_e35092;
        var_tmf1_dn0 = assign24020_e35092_d_n0;
        var_tmf1_dn2 = assign24020_e35092_d_n2;
        var_tmf1_rv = 0.0;

        let (assign24030_e35106, assign24030_e35106_d_n0, assign24030_e35106_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 != 0.0)) && (var_guard391 != 0.0)) {
        let assign24030_e35102: f64 = (4.0 * p.p85);
        let assign24030_e35104: f64 = (assign24030_e35102 * 0.01);
        (assign24030_e35104, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign24030_e35106;
        var_tmf2_dn0 = assign24030_e35106_d_n0;
        var_tmf2_dn2 = assign24030_e35106_d_n2;
        var_tmf2_rv = 0.0;

        let (assign24040_e35122, assign24040_e35122_d_n0, assign24040_e35122_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 != 0.0)) && (var_guard391 != 0.0)) {
        let (assign24040_e35120, assign24040_e35120_d_n0, assign24040_e35120_d_n2,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
            } else {
                let assign24040_e35119: f64 = (-var_tmf2);
                (assign24040_e35119, (-var_tmf2_dn0), (-var_tmf2_dn2),)
            }
        };
        (assign24040_e35120, assign24040_e35120_d_n0, assign24040_e35120_d_n2,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign24040_e35122;
        var_tmf2_dn0 = assign24040_e35122_d_n0;
        var_tmf2_dn2 = assign24040_e35122_d_n2;
        var_tmf2_rv = 0.0;

        let (assign24050_e35137, assign24050_e35137_d_n0, assign24050_e35137_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 != 0.0)) && (var_guard391 != 0.0)) {
        let assign24050_e35132: f64 = (var_tmf1 * var_tmf1);
        let assign24050_e35134: f64 = (assign24050_e35132 + var_tmf2);
        let assign24050_e35135: f64 = (assign24050_e35134).sqrt();
        (assign24050_e35135, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign24050_e35135)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign24050_e35135)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign24050_e35137;
        var_tmf2_dn0 = assign24050_e35137_d_n0;
        var_tmf2_dn2 = assign24050_e35137_d_n2;
        var_tmf2_rv = 0.0;

        let (assign24060_e35153, assign24060_e35153_d_n0, assign24060_e35153_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 != 0.0)) && (var_guard391 != 0.0)) {
        let assign24060_e35149: f64 = (var_tmf1 + var_tmf2);
        let assign24060_e35150: f64 = (0.5 * assign24060_e35149);
        let assign24060_e35151: f64 = (p.p85 - assign24060_e35150);
        (assign24060_e35151, (-(0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (-(0.5 * (var_tmf1_dn2 + var_tmf2_dn2))),)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn2,)
    }
};
        var_nj0 = assign24060_e35153;
        var_nj0_dn0 = assign24060_e35153_d_n0;
        var_nj0_dn2 = assign24060_e35153_d_n2;
        var_nj0_rv = 0.0;

        let (assign24070_e35167, assign24070_e35167_d_n0, assign24070_e35167_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 != 0.0)) && (var_guard391 != 0.0)) {
        let assign24070_e35163: f64 = (var_nj0 - var_nfasti_i);
        let assign24070_e35165: f64 = (assign24070_e35163 - 0.01);
        (assign24070_e35165, var_nj0_dn0, var_nj0_dn2,)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2,)
    }
};
        var_tmf1 = assign24070_e35167;
        var_tmf1_dn0 = assign24070_e35167_d_n0;
        var_tmf1_dn2 = assign24070_e35167_d_n2;
        var_tmf1_rv = 0.0;

        let (assign24080_e35181, assign24080_e35181_d_n0, assign24080_e35181_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 != 0.0)) && (var_guard391 != 0.0)) {
        let assign24080_e35177: f64 = (4.0 * var_nfasti_i);
        let assign24080_e35179: f64 = (assign24080_e35177 * 0.01);
        (assign24080_e35179, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign24080_e35181;
        var_tmf2_dn0 = assign24080_e35181_d_n0;
        var_tmf2_dn2 = assign24080_e35181_d_n2;
        var_tmf2_rv = 0.0;

        let (assign24090_e35197, assign24090_e35197_d_n0, assign24090_e35197_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 != 0.0)) && (var_guard391 != 0.0)) {
        let (assign24090_e35195, assign24090_e35195_d_n0, assign24090_e35195_d_n2,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
            } else {
                let assign24090_e35194: f64 = (-var_tmf2);
                (assign24090_e35194, (-var_tmf2_dn0), (-var_tmf2_dn2),)
            }
        };
        (assign24090_e35195, assign24090_e35195_d_n0, assign24090_e35195_d_n2,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign24090_e35197;
        var_tmf2_dn0 = assign24090_e35197_d_n0;
        var_tmf2_dn2 = assign24090_e35197_d_n2;
        var_tmf2_rv = 0.0;

        let (assign24100_e35212, assign24100_e35212_d_n0, assign24100_e35212_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 != 0.0)) && (var_guard391 != 0.0)) {
        let assign24100_e35207: f64 = (var_tmf1 * var_tmf1);
        let assign24100_e35209: f64 = (assign24100_e35207 + var_tmf2);
        let assign24100_e35210: f64 = (assign24100_e35209).sqrt();
        (assign24100_e35210, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign24100_e35210)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign24100_e35210)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign24100_e35212;
        var_tmf2_dn0 = assign24100_e35212_d_n0;
        var_tmf2_dn2 = assign24100_e35212_d_n2;
        var_tmf2_rv = 0.0;

        let (assign24110_e35228, assign24110_e35228_d_n0, assign24110_e35228_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 != 0.0)) && (var_guard391 != 0.0)) {
        let assign24110_e35224: f64 = (var_tmf1 + var_tmf2);
        let assign24110_e35225: f64 = (0.5 * assign24110_e35224);
        let assign24110_e35226: f64 = (var_nfasti_i + assign24110_e35225);
        (assign24110_e35226, (0.5 * (var_tmf1_dn0 + var_tmf2_dn0)), (0.5 * (var_tmf1_dn2 + var_tmf2_dn2)),)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn2,)
    }
};
        var_nj0 = assign24110_e35228;
        var_nj0_dn0 = assign24110_e35228_d_n0;
        var_nj0_dn2 = assign24110_e35228_d_n2;
        var_nj0_rv = 0.0;

        let (assign24120_e35239, assign24120_e35239_d_n0, assign24120_e35239_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 != 0.0)) && (var_guard391 == 0.0)) {
        (var_nfasti_i, 0.0, 0.0,)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn2,)
    }
};
        var_nj0 = assign24120_e35239;
        var_nj0_dn0 = assign24120_e35239_d_n0;
        var_nj0_dn2 = assign24120_e35239_d_n2;
        var_nj0_rv = 0.0;

        let (assign24130_e35250, assign24130_e35250_d_n0, assign24130_e35250_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 != 0.0)) && (var_guard391 == 0.0)) {
        (var_nfasti_i, 0.0, 0.0,)
    } else {
        (var_nj1, var_nj1_dn0, var_nj1_dn2,)
    }
};
        var_nj1 = assign24130_e35250;
        var_nj1_dn0 = assign24130_e35250_d_n0;
        var_nj1_dn2 = assign24130_e35250_d_n2;
        var_nj1_rv = 0.0;

        let (assign24190_e35481,) = {
    if (((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 != 0.0)) {
        let assign24190_e35477: f64 = (var_nin * var_nin);
        let assign24190_e35479: f64 = (assign24190_e35477 / var_ndigat_i);
        (assign24190_e35479,)
    } else {
        (var_pnn0,)
    }
};
        var_pnn0 = assign24190_e35481;
        var_pnn0_rv = 0.0;

        let (assign24200_e35496,) = {
    if (((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 != 0.0)) {
        let assign24200_e35489: f64 = (var_nfagat_i / var_phitdinv);
        let assign24200_e35492: f64 = (var_ndigat_i / var_pnn0);
        let assign24200_e35493: f64 = (assign24200_e35492).ln();
        let assign24200_e35494: f64 = (assign24200_e35489 * assign24200_e35493);
        (assign24200_e35494,)
    } else {
        (var_vha1,)
    }
};
        var_vha1 = assign24200_e35496;
        var_vha1_rv = 0.0;

        let assign24210_e35499: f64 = if var_nfagat_i < p.p85 { 1.0 } else { 0.0 };
        var_guard394 = assign24210_e35499;
        var_guard394_rv = 0.0;

        let (assign24220_e35515, assign24220_e35515_d_n0, assign24220_e35515_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 != 0.0)) && (var_guard394 != 0.0)) {
        let assign24220_e35510: f64 = (var_v5 - var_vha1);
        let assign24220_e35511: f64 = (p.p86 * assign24220_e35510);
        let assign24220_e35513: f64 = (assign24220_e35511 + var_nfagat_i);
        (assign24220_e35513, 0.0, 0.0,)
    } else {
        (var_nja10, var_nja10_dn0, var_nja10_dn2,)
    }
};
        var_nja10 = assign24220_e35515;
        var_nja10_dn0 = assign24220_e35515_d_n0;
        var_nja10_dn2 = assign24220_e35515_d_n2;
        var_nja10_rv = 0.0;

        let (assign24230_e35529, assign24230_e35529_d_n0, assign24230_e35529_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 != 0.0)) && (var_guard394 != 0.0)) {
        let assign24230_e35526: f64 = (p.p86 * var_vha1);
        let assign24230_e35527: f64 = (var_nfagat_i - assign24230_e35526);
        (assign24230_e35527, 0.0, 0.0,)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn2,)
    }
};
        var_nj0 = assign24230_e35529;
        var_nj0_dn0 = assign24230_e35529_d_n0;
        var_nj0_dn2 = assign24230_e35529_d_n2;
        var_nj0_rv = 0.0;

        let (assign24240_e35543, assign24240_e35543_d_n0, assign24240_e35543_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 != 0.0)) && (var_guard394 != 0.0)) {
        let assign24240_e35539: f64 = (p.p85 - var_nja10);
        let assign24240_e35541: f64 = (assign24240_e35539 - 0.01);
        (assign24240_e35541, (-var_nja10_dn0), (-var_nja10_dn2),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2,)
    }
};
        var_tmf1 = assign24240_e35543;
        var_tmf1_dn0 = assign24240_e35543_d_n0;
        var_tmf1_dn2 = assign24240_e35543_d_n2;
        var_tmf1_rv = 0.0;

        let (assign24250_e35557, assign24250_e35557_d_n0, assign24250_e35557_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 != 0.0)) && (var_guard394 != 0.0)) {
        let assign24250_e35553: f64 = (4.0 * p.p85);
        let assign24250_e35555: f64 = (assign24250_e35553 * 0.01);
        (assign24250_e35555, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign24250_e35557;
        var_tmf2_dn0 = assign24250_e35557_d_n0;
        var_tmf2_dn2 = assign24250_e35557_d_n2;
        var_tmf2_rv = 0.0;

        let (assign24260_e35573, assign24260_e35573_d_n0, assign24260_e35573_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 != 0.0)) && (var_guard394 != 0.0)) {
        let (assign24260_e35571, assign24260_e35571_d_n0, assign24260_e35571_d_n2,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
            } else {
                let assign24260_e35570: f64 = (-var_tmf2);
                (assign24260_e35570, (-var_tmf2_dn0), (-var_tmf2_dn2),)
            }
        };
        (assign24260_e35571, assign24260_e35571_d_n0, assign24260_e35571_d_n2,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign24260_e35573;
        var_tmf2_dn0 = assign24260_e35573_d_n0;
        var_tmf2_dn2 = assign24260_e35573_d_n2;
        var_tmf2_rv = 0.0;

        let (assign24270_e35588, assign24270_e35588_d_n0, assign24270_e35588_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 != 0.0)) && (var_guard394 != 0.0)) {
        let assign24270_e35583: f64 = (var_tmf1 * var_tmf1);
        let assign24270_e35585: f64 = (assign24270_e35583 + var_tmf2);
        let assign24270_e35586: f64 = (assign24270_e35585).sqrt();
        (assign24270_e35586, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign24270_e35586)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign24270_e35586)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign24270_e35588;
        var_tmf2_dn0 = assign24270_e35588_d_n0;
        var_tmf2_dn2 = assign24270_e35588_d_n2;
        var_tmf2_rv = 0.0;

        let (assign24280_e35604, assign24280_e35604_d_n0, assign24280_e35604_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 != 0.0)) && (var_guard394 != 0.0)) {
        let assign24280_e35600: f64 = (var_tmf1 + var_tmf2);
        let assign24280_e35601: f64 = (0.5 * assign24280_e35600);
        let assign24280_e35602: f64 = (p.p85 - assign24280_e35601);
        (assign24280_e35602, (-(0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (-(0.5 * (var_tmf1_dn2 + var_tmf2_dn2))),)
    } else {
        (var_nja11, var_nja11_dn0, var_nja11_dn2,)
    }
};
        var_nja11 = assign24280_e35604;
        var_nja11_dn0 = assign24280_e35604_d_n0;
        var_nja11_dn2 = assign24280_e35604_d_n2;
        var_nja11_rv = 0.0;

        let (assign24290_e35618, assign24290_e35618_d_n0, assign24290_e35618_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 != 0.0)) && (var_guard394 != 0.0)) {
        let assign24290_e35614: f64 = (var_nja11 - var_nfagat_i);
        let assign24290_e35616: f64 = (assign24290_e35614 - 0.01);
        (assign24290_e35616, var_nja11_dn0, var_nja11_dn2,)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2,)
    }
};
        var_tmf1 = assign24290_e35618;
        var_tmf1_dn0 = assign24290_e35618_d_n0;
        var_tmf1_dn2 = assign24290_e35618_d_n2;
        var_tmf1_rv = 0.0;

        let (assign24300_e35632, assign24300_e35632_d_n0, assign24300_e35632_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 != 0.0)) && (var_guard394 != 0.0)) {
        let assign24300_e35628: f64 = (4.0 * var_nfagat_i);
        let assign24300_e35630: f64 = (assign24300_e35628 * 0.01);
        (assign24300_e35630, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign24300_e35632;
        var_tmf2_dn0 = assign24300_e35632_d_n0;
        var_tmf2_dn2 = assign24300_e35632_d_n2;
        var_tmf2_rv = 0.0;

        *var_guard394_slot = var_guard394;
        *var_guard394_rv_slot = var_guard394_rv;
        *var_nj0_slot = var_nj0;
        *var_nj0_dn0_slot = var_nj0_dn0;
        *var_nj0_dn2_slot = var_nj0_dn2;
        *var_nj0_rv_slot = var_nj0_rv;
        *var_nj1_slot = var_nj1;
        *var_nj1_dn0_slot = var_nj1_dn0;
        *var_nj1_dn2_slot = var_nj1_dn2;
        *var_nj1_rv_slot = var_nj1_rv;
        *var_nja10_slot = var_nja10;
        *var_nja10_dn0_slot = var_nja10_dn0;
        *var_nja10_dn2_slot = var_nja10_dn2;
        *var_nja10_rv_slot = var_nja10_rv;
        *var_nja11_slot = var_nja11;
        *var_nja11_dn0_slot = var_nja11_dn0;
        *var_nja11_dn2_slot = var_nja11_dn2;
        *var_nja11_rv_slot = var_nja11_rv;
        *var_pnn0_slot = var_pnn0;
        *var_pnn0_rv_slot = var_pnn0_rv;
        *var_tmf1_slot = var_tmf1;
        *var_tmf1_dn0_slot = var_tmf1_dn0;
        *var_tmf1_dn2_slot = var_tmf1_dn2;
        *var_tmf1_rv_slot = var_tmf1_rv;
        *var_tmf2_slot = var_tmf2;
        *var_tmf2_dn0_slot = var_tmf2_dn0;
        *var_tmf2_dn2_slot = var_tmf2_dn2;
        *var_tmf2_rv_slot = var_tmf2_rv;
        *var_vha1_slot = var_vha1;
        *var_vha1_rv_slot = var_vha1_rv;
    }

    pub(super) fn stamp_reactive_block_27(
        p: &Parameters,
        var_guard31: f64,
        var_guard384: f64,
        var_guard385: f64,
        var_guard394: f64,
        var_ndibot_i: f64,
        var_nfabot_i: f64,
        var_nfagat_i: f64,
        var_nin: f64,
        var_phitdinv: f64,
        var_vmax: f64,
        var_dfn_sl_slot: &mut f64,
        var_dfn_sl_dn0_slot: &mut f64,
        var_dfn_sl_dn2_slot: &mut f64,
        var_dfn_sl_rv_slot: &mut f64,
        var_dfn_su_slot: &mut f64,
        var_dfn_su_dn0_slot: &mut f64,
        var_dfn_su_dn2_slot: &mut f64,
        var_dfn_su_rv_slot: &mut f64,
        var_guard397_slot: &mut f64,
        var_guard397_rv_slot: &mut f64,
        var_nj0_slot: &mut f64,
        var_nj0_dn0_slot: &mut f64,
        var_nj0_dn2_slot: &mut f64,
        var_nj0_rv_slot: &mut f64,
        var_nj1_slot: &mut f64,
        var_nj1_dn0_slot: &mut f64,
        var_nj1_dn2_slot: &mut f64,
        var_nj1_rv_slot: &mut f64,
        var_nja10_slot: &mut f64,
        var_nja10_dn0_slot: &mut f64,
        var_nja10_dn2_slot: &mut f64,
        var_nja10_rv_slot: &mut f64,
        var_nja11_slot: &mut f64,
        var_nja11_dn0_slot: &mut f64,
        var_nja11_dn2_slot: &mut f64,
        var_nja11_rv_slot: &mut f64,
        var_pnn0_slot: &mut f64,
        var_pnn0_rv_slot: &mut f64,
        var_tmf1_slot: &mut f64,
        var_tmf1_dn0_slot: &mut f64,
        var_tmf1_dn2_slot: &mut f64,
        var_tmf1_rv_slot: &mut f64,
        var_tmf2_slot: &mut f64,
        var_tmf2_dn0_slot: &mut f64,
        var_tmf2_dn2_slot: &mut f64,
        var_tmf2_rv_slot: &mut f64,
        var_vha1_slot: &mut f64,
        var_vha1_rv_slot: &mut f64,
    ) {
        let mut var_dfn_sl: f64 = *var_dfn_sl_slot;
        let mut var_dfn_sl_dn0: f64 = *var_dfn_sl_dn0_slot;
        let mut var_dfn_sl_dn2: f64 = *var_dfn_sl_dn2_slot;
        let mut var_dfn_sl_rv: f64 = *var_dfn_sl_rv_slot;
        let mut var_dfn_su: f64 = *var_dfn_su_slot;
        let mut var_dfn_su_dn0: f64 = *var_dfn_su_dn0_slot;
        let mut var_dfn_su_dn2: f64 = *var_dfn_su_dn2_slot;
        let mut var_dfn_su_rv: f64 = *var_dfn_su_rv_slot;
        let mut var_guard397: f64 = *var_guard397_slot;
        let mut var_guard397_rv: f64 = *var_guard397_rv_slot;
        let mut var_nj0: f64 = *var_nj0_slot;
        let mut var_nj0_dn0: f64 = *var_nj0_dn0_slot;
        let mut var_nj0_dn2: f64 = *var_nj0_dn2_slot;
        let mut var_nj0_rv: f64 = *var_nj0_rv_slot;
        let mut var_nj1: f64 = *var_nj1_slot;
        let mut var_nj1_dn0: f64 = *var_nj1_dn0_slot;
        let mut var_nj1_dn2: f64 = *var_nj1_dn2_slot;
        let mut var_nj1_rv: f64 = *var_nj1_rv_slot;
        let mut var_nja10: f64 = *var_nja10_slot;
        let mut var_nja10_dn0: f64 = *var_nja10_dn0_slot;
        let mut var_nja10_dn2: f64 = *var_nja10_dn2_slot;
        let mut var_nja10_rv: f64 = *var_nja10_rv_slot;
        let mut var_nja11: f64 = *var_nja11_slot;
        let mut var_nja11_dn0: f64 = *var_nja11_dn0_slot;
        let mut var_nja11_dn2: f64 = *var_nja11_dn2_slot;
        let mut var_nja11_rv: f64 = *var_nja11_rv_slot;
        let mut var_pnn0: f64 = *var_pnn0_slot;
        let mut var_pnn0_rv: f64 = *var_pnn0_rv_slot;
        let mut var_tmf1: f64 = *var_tmf1_slot;
        let mut var_tmf1_dn0: f64 = *var_tmf1_dn0_slot;
        let mut var_tmf1_dn2: f64 = *var_tmf1_dn2_slot;
        let mut var_tmf1_rv: f64 = *var_tmf1_rv_slot;
        let mut var_tmf2: f64 = *var_tmf2_slot;
        let mut var_tmf2_dn0: f64 = *var_tmf2_dn0_slot;
        let mut var_tmf2_dn2: f64 = *var_tmf2_dn2_slot;
        let mut var_tmf2_rv: f64 = *var_tmf2_rv_slot;
        let mut var_vha1: f64 = *var_vha1_slot;
        let mut var_vha1_rv: f64 = *var_vha1_rv_slot;

        let (assign24310_e35648, assign24310_e35648_d_n0, assign24310_e35648_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 != 0.0)) && (var_guard394 != 0.0)) {
        let (assign24310_e35646, assign24310_e35646_d_n0, assign24310_e35646_d_n2,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
            } else {
                let assign24310_e35645: f64 = (-var_tmf2);
                (assign24310_e35645, (-var_tmf2_dn0), (-var_tmf2_dn2),)
            }
        };
        (assign24310_e35646, assign24310_e35646_d_n0, assign24310_e35646_d_n2,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign24310_e35648;
        var_tmf2_dn0 = assign24310_e35648_d_n0;
        var_tmf2_dn2 = assign24310_e35648_d_n2;
        var_tmf2_rv = 0.0;

        let (assign24320_e35663, assign24320_e35663_d_n0, assign24320_e35663_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 != 0.0)) && (var_guard394 != 0.0)) {
        let assign24320_e35658: f64 = (var_tmf1 * var_tmf1);
        let assign24320_e35660: f64 = (assign24320_e35658 + var_tmf2);
        let assign24320_e35661: f64 = (assign24320_e35660).sqrt();
        (assign24320_e35661, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign24320_e35661)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign24320_e35661)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign24320_e35663;
        var_tmf2_dn0 = assign24320_e35663_d_n0;
        var_tmf2_dn2 = assign24320_e35663_d_n2;
        var_tmf2_rv = 0.0;

        let (assign24330_e35679, assign24330_e35679_d_n0, assign24330_e35679_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 != 0.0)) && (var_guard394 != 0.0)) {
        let assign24330_e35675: f64 = (var_tmf1 + var_tmf2);
        let assign24330_e35676: f64 = (0.5 * assign24330_e35675);
        let assign24330_e35677: f64 = (var_nfagat_i + assign24330_e35676);
        (assign24330_e35677, (0.5 * (var_tmf1_dn0 + var_tmf2_dn0)), (0.5 * (var_tmf1_dn2 + var_tmf2_dn2)),)
    } else {
        (var_nj1, var_nj1_dn0, var_nj1_dn2,)
    }
};
        var_nj1 = assign24330_e35679;
        var_nj1_dn0 = assign24330_e35679_d_n0;
        var_nj1_dn2 = assign24330_e35679_d_n2;
        var_nj1_rv = 0.0;

        let (assign24340_e35693, assign24340_e35693_d_n0, assign24340_e35693_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 != 0.0)) && (var_guard394 != 0.0)) {
        let assign24340_e35689: f64 = (p.p85 - var_nj0);
        let assign24340_e35691: f64 = (assign24340_e35689 - 0.01);
        (assign24340_e35691, (-var_nj0_dn0), (-var_nj0_dn2),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2,)
    }
};
        var_tmf1 = assign24340_e35693;
        var_tmf1_dn0 = assign24340_e35693_d_n0;
        var_tmf1_dn2 = assign24340_e35693_d_n2;
        var_tmf1_rv = 0.0;

        let (assign24350_e35707, assign24350_e35707_d_n0, assign24350_e35707_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 != 0.0)) && (var_guard394 != 0.0)) {
        let assign24350_e35703: f64 = (4.0 * p.p85);
        let assign24350_e35705: f64 = (assign24350_e35703 * 0.01);
        (assign24350_e35705, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign24350_e35707;
        var_tmf2_dn0 = assign24350_e35707_d_n0;
        var_tmf2_dn2 = assign24350_e35707_d_n2;
        var_tmf2_rv = 0.0;

        let (assign24360_e35723, assign24360_e35723_d_n0, assign24360_e35723_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 != 0.0)) && (var_guard394 != 0.0)) {
        let (assign24360_e35721, assign24360_e35721_d_n0, assign24360_e35721_d_n2,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
            } else {
                let assign24360_e35720: f64 = (-var_tmf2);
                (assign24360_e35720, (-var_tmf2_dn0), (-var_tmf2_dn2),)
            }
        };
        (assign24360_e35721, assign24360_e35721_d_n0, assign24360_e35721_d_n2,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign24360_e35723;
        var_tmf2_dn0 = assign24360_e35723_d_n0;
        var_tmf2_dn2 = assign24360_e35723_d_n2;
        var_tmf2_rv = 0.0;

        let (assign24370_e35738, assign24370_e35738_d_n0, assign24370_e35738_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 != 0.0)) && (var_guard394 != 0.0)) {
        let assign24370_e35733: f64 = (var_tmf1 * var_tmf1);
        let assign24370_e35735: f64 = (assign24370_e35733 + var_tmf2);
        let assign24370_e35736: f64 = (assign24370_e35735).sqrt();
        (assign24370_e35736, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign24370_e35736)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign24370_e35736)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign24370_e35738;
        var_tmf2_dn0 = assign24370_e35738_d_n0;
        var_tmf2_dn2 = assign24370_e35738_d_n2;
        var_tmf2_rv = 0.0;

        let (assign24380_e35754, assign24380_e35754_d_n0, assign24380_e35754_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 != 0.0)) && (var_guard394 != 0.0)) {
        let assign24380_e35750: f64 = (var_tmf1 + var_tmf2);
        let assign24380_e35751: f64 = (0.5 * assign24380_e35750);
        let assign24380_e35752: f64 = (p.p85 - assign24380_e35751);
        (assign24380_e35752, (-(0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (-(0.5 * (var_tmf1_dn2 + var_tmf2_dn2))),)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn2,)
    }
};
        var_nj0 = assign24380_e35754;
        var_nj0_dn0 = assign24380_e35754_d_n0;
        var_nj0_dn2 = assign24380_e35754_d_n2;
        var_nj0_rv = 0.0;

        let (assign24390_e35768, assign24390_e35768_d_n0, assign24390_e35768_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 != 0.0)) && (var_guard394 != 0.0)) {
        let assign24390_e35764: f64 = (var_nj0 - var_nfagat_i);
        let assign24390_e35766: f64 = (assign24390_e35764 - 0.01);
        (assign24390_e35766, var_nj0_dn0, var_nj0_dn2,)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2,)
    }
};
        var_tmf1 = assign24390_e35768;
        var_tmf1_dn0 = assign24390_e35768_d_n0;
        var_tmf1_dn2 = assign24390_e35768_d_n2;
        var_tmf1_rv = 0.0;

        let (assign24400_e35782, assign24400_e35782_d_n0, assign24400_e35782_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 != 0.0)) && (var_guard394 != 0.0)) {
        let assign24400_e35778: f64 = (4.0 * var_nfagat_i);
        let assign24400_e35780: f64 = (assign24400_e35778 * 0.01);
        (assign24400_e35780, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign24400_e35782;
        var_tmf2_dn0 = assign24400_e35782_d_n0;
        var_tmf2_dn2 = assign24400_e35782_d_n2;
        var_tmf2_rv = 0.0;

        let (assign24410_e35798, assign24410_e35798_d_n0, assign24410_e35798_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 != 0.0)) && (var_guard394 != 0.0)) {
        let (assign24410_e35796, assign24410_e35796_d_n0, assign24410_e35796_d_n2,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
            } else {
                let assign24410_e35795: f64 = (-var_tmf2);
                (assign24410_e35795, (-var_tmf2_dn0), (-var_tmf2_dn2),)
            }
        };
        (assign24410_e35796, assign24410_e35796_d_n0, assign24410_e35796_d_n2,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign24410_e35798;
        var_tmf2_dn0 = assign24410_e35798_d_n0;
        var_tmf2_dn2 = assign24410_e35798_d_n2;
        var_tmf2_rv = 0.0;

        let (assign24420_e35813, assign24420_e35813_d_n0, assign24420_e35813_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 != 0.0)) && (var_guard394 != 0.0)) {
        let assign24420_e35808: f64 = (var_tmf1 * var_tmf1);
        let assign24420_e35810: f64 = (assign24420_e35808 + var_tmf2);
        let assign24420_e35811: f64 = (assign24420_e35810).sqrt();
        (assign24420_e35811, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign24420_e35811)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign24420_e35811)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign24420_e35813;
        var_tmf2_dn0 = assign24420_e35813_d_n0;
        var_tmf2_dn2 = assign24420_e35813_d_n2;
        var_tmf2_rv = 0.0;

        let (assign24430_e35829, assign24430_e35829_d_n0, assign24430_e35829_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 != 0.0)) && (var_guard394 != 0.0)) {
        let assign24430_e35825: f64 = (var_tmf1 + var_tmf2);
        let assign24430_e35826: f64 = (0.5 * assign24430_e35825);
        let assign24430_e35827: f64 = (var_nfagat_i + assign24430_e35826);
        (assign24430_e35827, (0.5 * (var_tmf1_dn0 + var_tmf2_dn0)), (0.5 * (var_tmf1_dn2 + var_tmf2_dn2)),)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn2,)
    }
};
        var_nj0 = assign24430_e35829;
        var_nj0_dn0 = assign24430_e35829_d_n0;
        var_nj0_dn2 = assign24430_e35829_d_n2;
        var_nj0_rv = 0.0;

        let (assign24440_e35840, assign24440_e35840_d_n0, assign24440_e35840_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 != 0.0)) && (var_guard394 == 0.0)) {
        (var_nfagat_i, 0.0, 0.0,)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn2,)
    }
};
        var_nj0 = assign24440_e35840;
        var_nj0_dn0 = assign24440_e35840_d_n0;
        var_nj0_dn2 = assign24440_e35840_d_n2;
        var_nj0_rv = 0.0;

        let (assign24450_e35851, assign24450_e35851_d_n0, assign24450_e35851_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 != 0.0)) && (var_guard394 == 0.0)) {
        (var_nfagat_i, 0.0, 0.0,)
    } else {
        (var_nj1, var_nj1_dn0, var_nj1_dn2,)
    }
};
        var_nj1 = assign24450_e35851;
        var_nj1_dn0 = assign24450_e35851_d_n0;
        var_nj1_dn2 = assign24450_e35851_d_n2;
        var_nj1_rv = 0.0;

        let (assign24520_e36101,) = {
    if (((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) {
        let assign24520_e36097: f64 = (var_nin * var_nin);
        let assign24520_e36099: f64 = (assign24520_e36097 / var_ndibot_i);
        (assign24520_e36099,)
    } else {
        (var_pnn0,)
    }
};
        var_pnn0 = assign24520_e36101;
        var_pnn0_rv = 0.0;

        let (assign24530_e36117,) = {
    if (((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) {
        let assign24530_e36110: f64 = (var_nfabot_i / var_phitdinv);
        let assign24530_e36113: f64 = (var_ndibot_i / var_pnn0);
        let assign24530_e36114: f64 = (assign24530_e36113).ln();
        let assign24530_e36115: f64 = (assign24530_e36110 * assign24530_e36114);
        (assign24530_e36115,)
    } else {
        (var_vha1,)
    }
};
        var_vha1 = assign24530_e36117;
        var_vha1_rv = 0.0;

        let assign24540_e36120: f64 = if var_nfabot_i < p.p85 { 1.0 } else { 0.0 };
        var_guard397 = assign24540_e36120;
        var_guard397_rv = 0.0;

        let (assign24550_e36137, assign24550_e36137_d_n0, assign24550_e36137_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard397 != 0.0)) {
        let assign24550_e36132: f64 = (var_vmax - var_vha1);
        let assign24550_e36133: f64 = (p.p86 * assign24550_e36132);
        let assign24550_e36135: f64 = (assign24550_e36133 + var_nfabot_i);
        (assign24550_e36135, 0.0, 0.0,)
    } else {
        (var_nja10, var_nja10_dn0, var_nja10_dn2,)
    }
};
        var_nja10 = assign24550_e36137;
        var_nja10_dn0 = assign24550_e36137_d_n0;
        var_nja10_dn2 = assign24550_e36137_d_n2;
        var_nja10_rv = 0.0;

        let (assign24560_e36152, assign24560_e36152_d_n0, assign24560_e36152_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard397 != 0.0)) {
        let assign24560_e36149: f64 = (p.p86 * var_vha1);
        let assign24560_e36150: f64 = (var_nfabot_i - assign24560_e36149);
        (assign24560_e36150, 0.0, 0.0,)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn2,)
    }
};
        var_nj0 = assign24560_e36152;
        var_nj0_dn0 = assign24560_e36152_d_n0;
        var_nj0_dn2 = assign24560_e36152_d_n2;
        var_nj0_rv = 0.0;

        let (assign24570_e36167, assign24570_e36167_d_n0, assign24570_e36167_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard397 != 0.0)) {
        let assign24570_e36163: f64 = (p.p85 - var_nja10);
        let assign24570_e36165: f64 = (assign24570_e36163 - 0.01);
        (assign24570_e36165, (-var_nja10_dn0), (-var_nja10_dn2),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2,)
    }
};
        var_tmf1 = assign24570_e36167;
        var_tmf1_dn0 = assign24570_e36167_d_n0;
        var_tmf1_dn2 = assign24570_e36167_d_n2;
        var_tmf1_rv = 0.0;

        let (assign24580_e36182, assign24580_e36182_d_n0, assign24580_e36182_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard397 != 0.0)) {
        let assign24580_e36178: f64 = (4.0 * p.p85);
        let assign24580_e36180: f64 = (assign24580_e36178 * 0.01);
        (assign24580_e36180, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign24580_e36182;
        var_tmf2_dn0 = assign24580_e36182_d_n0;
        var_tmf2_dn2 = assign24580_e36182_d_n2;
        var_tmf2_rv = 0.0;

        let (assign24590_e36199, assign24590_e36199_d_n0, assign24590_e36199_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard397 != 0.0)) {
        let (assign24590_e36197, assign24590_e36197_d_n0, assign24590_e36197_d_n2,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
            } else {
                let assign24590_e36196: f64 = (-var_tmf2);
                (assign24590_e36196, (-var_tmf2_dn0), (-var_tmf2_dn2),)
            }
        };
        (assign24590_e36197, assign24590_e36197_d_n0, assign24590_e36197_d_n2,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign24590_e36199;
        var_tmf2_dn0 = assign24590_e36199_d_n0;
        var_tmf2_dn2 = assign24590_e36199_d_n2;
        var_tmf2_rv = 0.0;

        let (assign24600_e36215, assign24600_e36215_d_n0, assign24600_e36215_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard397 != 0.0)) {
        let assign24600_e36210: f64 = (var_tmf1 * var_tmf1);
        let assign24600_e36212: f64 = (assign24600_e36210 + var_tmf2);
        let assign24600_e36213: f64 = (assign24600_e36212).sqrt();
        (assign24600_e36213, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign24600_e36213)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign24600_e36213)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign24600_e36215;
        var_tmf2_dn0 = assign24600_e36215_d_n0;
        var_tmf2_dn2 = assign24600_e36215_d_n2;
        var_tmf2_rv = 0.0;

        let (assign24610_e36232, assign24610_e36232_d_n0, assign24610_e36232_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard397 != 0.0)) {
        let assign24610_e36228: f64 = (var_tmf1 / var_tmf2);
        let assign24610_e36229: f64 = (1.0 + assign24610_e36228);
        let assign24610_e36230: f64 = (0.5 * assign24610_e36229);
        (assign24610_e36230, (0.5 * (((var_tmf1_dn0 * var_tmf2) - (var_tmf1 * var_tmf2_dn0)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn2 * var_tmf2) - (var_tmf1 * var_tmf2_dn2)) / (var_tmf2 * var_tmf2))),)
    } else {
        (var_dfn_su, var_dfn_su_dn0, var_dfn_su_dn2,)
    }
};
        var_dfn_su = assign24610_e36232;
        var_dfn_su_dn0 = assign24610_e36232_d_n0;
        var_dfn_su_dn2 = assign24610_e36232_d_n2;
        var_dfn_su_rv = 0.0;

        let (assign24620_e36249, assign24620_e36249_d_n0, assign24620_e36249_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard397 != 0.0)) {
        let assign24620_e36245: f64 = (var_tmf1 + var_tmf2);
        let assign24620_e36246: f64 = (0.5 * assign24620_e36245);
        let assign24620_e36247: f64 = (p.p85 - assign24620_e36246);
        (assign24620_e36247, (-(0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (-(0.5 * (var_tmf1_dn2 + var_tmf2_dn2))),)
    } else {
        (var_nja11, var_nja11_dn0, var_nja11_dn2,)
    }
};
        var_nja11 = assign24620_e36249;
        var_nja11_dn0 = assign24620_e36249_d_n0;
        var_nja11_dn2 = assign24620_e36249_d_n2;
        var_nja11_rv = 0.0;

        let (assign24630_e36264, assign24630_e36264_d_n0, assign24630_e36264_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard397 != 0.0)) {
        let assign24630_e36260: f64 = (var_nja11 - var_nfabot_i);
        let assign24630_e36262: f64 = (assign24630_e36260 - 0.01);
        (assign24630_e36262, var_nja11_dn0, var_nja11_dn2,)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2,)
    }
};
        var_tmf1 = assign24630_e36264;
        var_tmf1_dn0 = assign24630_e36264_d_n0;
        var_tmf1_dn2 = assign24630_e36264_d_n2;
        var_tmf1_rv = 0.0;

        let (assign24640_e36279, assign24640_e36279_d_n0, assign24640_e36279_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard397 != 0.0)) {
        let assign24640_e36275: f64 = (4.0 * var_nfabot_i);
        let assign24640_e36277: f64 = (assign24640_e36275 * 0.01);
        (assign24640_e36277, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign24640_e36279;
        var_tmf2_dn0 = assign24640_e36279_d_n0;
        var_tmf2_dn2 = assign24640_e36279_d_n2;
        var_tmf2_rv = 0.0;

        let (assign24650_e36296, assign24650_e36296_d_n0, assign24650_e36296_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard397 != 0.0)) {
        let (assign24650_e36294, assign24650_e36294_d_n0, assign24650_e36294_d_n2,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
            } else {
                let assign24650_e36293: f64 = (-var_tmf2);
                (assign24650_e36293, (-var_tmf2_dn0), (-var_tmf2_dn2),)
            }
        };
        (assign24650_e36294, assign24650_e36294_d_n0, assign24650_e36294_d_n2,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign24650_e36296;
        var_tmf2_dn0 = assign24650_e36296_d_n0;
        var_tmf2_dn2 = assign24650_e36296_d_n2;
        var_tmf2_rv = 0.0;

        let (assign24660_e36312, assign24660_e36312_d_n0, assign24660_e36312_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard397 != 0.0)) {
        let assign24660_e36307: f64 = (var_tmf1 * var_tmf1);
        let assign24660_e36309: f64 = (assign24660_e36307 + var_tmf2);
        let assign24660_e36310: f64 = (assign24660_e36309).sqrt();
        (assign24660_e36310, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign24660_e36310)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign24660_e36310)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign24660_e36312;
        var_tmf2_dn0 = assign24660_e36312_d_n0;
        var_tmf2_dn2 = assign24660_e36312_d_n2;
        var_tmf2_rv = 0.0;

        let (assign24670_e36329, assign24670_e36329_d_n0, assign24670_e36329_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard397 != 0.0)) {
        let assign24670_e36325: f64 = (var_tmf1 / var_tmf2);
        let assign24670_e36326: f64 = (1.0 + assign24670_e36325);
        let assign24670_e36327: f64 = (0.5 * assign24670_e36326);
        (assign24670_e36327, (0.5 * (((var_tmf1_dn0 * var_tmf2) - (var_tmf1 * var_tmf2_dn0)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn2 * var_tmf2) - (var_tmf1 * var_tmf2_dn2)) / (var_tmf2 * var_tmf2))),)
    } else {
        (var_dfn_sl, var_dfn_sl_dn0, var_dfn_sl_dn2,)
    }
};
        var_dfn_sl = assign24670_e36329;
        var_dfn_sl_dn0 = assign24670_e36329_d_n0;
        var_dfn_sl_dn2 = assign24670_e36329_d_n2;
        var_dfn_sl_rv = 0.0;

        let (assign24680_e36346, assign24680_e36346_d_n0, assign24680_e36346_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard397 != 0.0)) {
        let assign24680_e36342: f64 = (var_tmf1 + var_tmf2);
        let assign24680_e36343: f64 = (0.5 * assign24680_e36342);
        let assign24680_e36344: f64 = (var_nfabot_i + assign24680_e36343);
        (assign24680_e36344, (0.5 * (var_tmf1_dn0 + var_tmf2_dn0)), (0.5 * (var_tmf1_dn2 + var_tmf2_dn2)),)
    } else {
        (var_nj1, var_nj1_dn0, var_nj1_dn2,)
    }
};
        var_nj1 = assign24680_e36346;
        var_nj1_dn0 = assign24680_e36346_d_n0;
        var_nj1_dn2 = assign24680_e36346_d_n2;
        var_nj1_rv = 0.0;

        let (assign24690_e36361, assign24690_e36361_d_n0, assign24690_e36361_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard397 != 0.0)) {
        let assign24690_e36357: f64 = (p.p85 - var_nj0);
        let assign24690_e36359: f64 = (assign24690_e36357 - 0.01);
        (assign24690_e36359, (-var_nj0_dn0), (-var_nj0_dn2),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2,)
    }
};
        var_tmf1 = assign24690_e36361;
        var_tmf1_dn0 = assign24690_e36361_d_n0;
        var_tmf1_dn2 = assign24690_e36361_d_n2;
        var_tmf1_rv = 0.0;

        let (assign24700_e36376, assign24700_e36376_d_n0, assign24700_e36376_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard397 != 0.0)) {
        let assign24700_e36372: f64 = (4.0 * p.p85);
        let assign24700_e36374: f64 = (assign24700_e36372 * 0.01);
        (assign24700_e36374, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign24700_e36376;
        var_tmf2_dn0 = assign24700_e36376_d_n0;
        var_tmf2_dn2 = assign24700_e36376_d_n2;
        var_tmf2_rv = 0.0;

        *var_dfn_sl_slot = var_dfn_sl;
        *var_dfn_sl_dn0_slot = var_dfn_sl_dn0;
        *var_dfn_sl_dn2_slot = var_dfn_sl_dn2;
        *var_dfn_sl_rv_slot = var_dfn_sl_rv;
        *var_dfn_su_slot = var_dfn_su;
        *var_dfn_su_dn0_slot = var_dfn_su_dn0;
        *var_dfn_su_dn2_slot = var_dfn_su_dn2;
        *var_dfn_su_rv_slot = var_dfn_su_rv;
        *var_guard397_slot = var_guard397;
        *var_guard397_rv_slot = var_guard397_rv;
        *var_nj0_slot = var_nj0;
        *var_nj0_dn0_slot = var_nj0_dn0;
        *var_nj0_dn2_slot = var_nj0_dn2;
        *var_nj0_rv_slot = var_nj0_rv;
        *var_nj1_slot = var_nj1;
        *var_nj1_dn0_slot = var_nj1_dn0;
        *var_nj1_dn2_slot = var_nj1_dn2;
        *var_nj1_rv_slot = var_nj1_rv;
        *var_nja10_slot = var_nja10;
        *var_nja10_dn0_slot = var_nja10_dn0;
        *var_nja10_dn2_slot = var_nja10_dn2;
        *var_nja10_rv_slot = var_nja10_rv;
        *var_nja11_slot = var_nja11;
        *var_nja11_dn0_slot = var_nja11_dn0;
        *var_nja11_dn2_slot = var_nja11_dn2;
        *var_nja11_rv_slot = var_nja11_rv;
        *var_pnn0_slot = var_pnn0;
        *var_pnn0_rv_slot = var_pnn0_rv;
        *var_tmf1_slot = var_tmf1;
        *var_tmf1_dn0_slot = var_tmf1_dn0;
        *var_tmf1_dn2_slot = var_tmf1_dn2;
        *var_tmf1_rv_slot = var_tmf1_rv;
        *var_tmf2_slot = var_tmf2;
        *var_tmf2_dn0_slot = var_tmf2_dn0;
        *var_tmf2_dn2_slot = var_tmf2_dn2;
        *var_tmf2_rv_slot = var_tmf2_rv;
        *var_vha1_slot = var_vha1;
        *var_vha1_rv_slot = var_vha1_rv;
    }
}
