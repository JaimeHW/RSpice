#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_13(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        var_guard109: f64,
        var_guard36: f64,
        var_iae: f64,
        var_iilcv: f64,
        var_iiwcv: f64,
        var_iiwecv: f64,
        var_ile: f64,
        var_ile2: f64,
        var_iwe: f64,
        var_le: f64,
        var_plparam_i: f64,
        var_plwparam_i: f64,
        var_poparam_i: f64,
        var_pwparam_i: f64,
        var_we_edge: f64,
        var_alp1ac_p_slot: &mut f64,
        var_alp1ac_p_rv_slot: &mut f64,
        var_alpac_p_slot: &mut f64,
        var_alpac_p_rv_slot: &mut f64,
        var_axac_p_slot: &mut f64,
        var_axac_p_rv_slot: &mut f64,
        var_betnedge_p_slot: &mut f64,
        var_betnedge_p_rv_slot: &mut f64,
        var_cfbedge_p_slot: &mut f64,
        var_cfbedge_p_rv_slot: &mut f64,
        var_cfdedge_p_slot: &mut f64,
        var_cfdedge_p_rv_slot: &mut f64,
        var_cfedge_p_slot: &mut f64,
        var_cfedge_p_rv_slot: &mut f64,
        var_cfr_p_slot: &mut f64,
        var_cfr_p_rv_slot: &mut f64,
        var_cfrd_p_slot: &mut f64,
        var_cfrd_p_rv_slot: &mut f64,
        var_cgbov_p_slot: &mut f64,
        var_cgbov_p_rv_slot: &mut f64,
        var_cgov_p_slot: &mut f64,
        var_cgov_p_rv_slot: &mut f64,
        var_cgovd_p_slot: &mut f64,
        var_cgovd_p_rv_slot: &mut f64,
        var_cinr_p_slot: &mut f64,
        var_cinr_p_rv_slot: &mut f64,
        var_cinrd_p_slot: &mut f64,
        var_cinrd_p_rv_slot: &mut f64,
        var_ctedge_p_slot: &mut f64,
        var_ctedge_p_rv_slot: &mut f64,
        var_dphibedge_p_slot: &mut f64,
        var_dphibedge_p_rv_slot: &mut f64,
        var_guard114_slot: &mut f64,
        var_guard114_rv_slot: &mut f64,
        var_guard115_slot: &mut f64,
        var_guard115_rv_slot: &mut f64,
        var_guard116_slot: &mut f64,
        var_guard116_rv_slot: &mut f64,
        var_guard117_slot: &mut f64,
        var_guard117_rv_slot: &mut f64,
        var_guard118_slot: &mut f64,
        var_guard118_rv_slot: &mut f64,
        var_guard119_slot: &mut f64,
        var_guard119_rv_slot: &mut f64,
        var_guard120_slot: &mut f64,
        var_guard120_rv_slot: &mut f64,
        var_guard121_slot: &mut f64,
        var_guard121_rv_slot: &mut f64,
        var_guard122_slot: &mut f64,
        var_guard122_rv_slot: &mut f64,
        var_guard127_slot: &mut f64,
        var_guard127_rv_slot: &mut f64,
        var_guard128_slot: &mut f64,
        var_guard128_rv_slot: &mut f64,
        var_guard129_slot: &mut f64,
        var_guard129_rv_slot: &mut f64,
        var_guard130_slot: &mut f64,
        var_guard130_rv_slot: &mut f64,
        var_guard131_slot: &mut f64,
        var_guard131_rv_slot: &mut f64,
        var_guard132_slot: &mut f64,
        var_guard132_rv_slot: &mut f64,
        var_guard133_slot: &mut f64,
        var_guard133_rv_slot: &mut f64,
        var_guard134_slot: &mut f64,
        var_guard134_rv_slot: &mut f64,
        var_guard135_slot: &mut f64,
        var_guard135_rv_slot: &mut f64,
        var_guard136_slot: &mut f64,
        var_guard136_rv_slot: &mut f64,
        var_guard137_slot: &mut f64,
        var_guard137_rv_slot: &mut f64,
        var_guard138_slot: &mut f64,
        var_guard138_rv_slot: &mut f64,
        var_guard139_slot: &mut f64,
        var_guard139_rv_slot: &mut f64,
        var_kvsatac_i_slot: &mut f64,
        var_kvsatac_i_rv_slot: &mut f64,
        var_loop__slot: &mut f64,
        var_loop__rv_slot: &mut f64,
        var_neffedge_p_slot: &mut f64,
        var_neffedge_p_rv_slot: &mut f64,
        var_pscebedge_p_slot: &mut f64,
        var_pscebedge_p_rv_slot: &mut f64,
        var_pscededge_p_slot: &mut f64,
        var_pscededge_p_rv_slot: &mut f64,
        var_psceedge_p_slot: &mut f64,
        var_psceedge_p_rv_slot: &mut f64,
        var_stbetedge_p_slot: &mut f64,
        var_stbetedge_p_rv_slot: &mut f64,
        var_stvfbedge_p_slot: &mut f64,
        var_stvfbedge_p_rv_slot: &mut f64,
        var_tmpa_slot: &mut f64,
        var_tmpa_rv_slot: &mut f64,
        var_tmpb_slot: &mut f64,
        var_tmpb_rv_slot: &mut f64,
        var_vfbedge_p_slot: &mut f64,
        var_vfbedge_p_rv_slot: &mut f64,
    ) {
        let mut var_alp1ac_p: f64 = *var_alp1ac_p_slot;
        let mut var_alp1ac_p_rv: f64 = *var_alp1ac_p_rv_slot;
        let mut var_alpac_p: f64 = *var_alpac_p_slot;
        let mut var_alpac_p_rv: f64 = *var_alpac_p_rv_slot;
        let mut var_axac_p: f64 = *var_axac_p_slot;
        let mut var_axac_p_rv: f64 = *var_axac_p_rv_slot;
        let mut var_betnedge_p: f64 = *var_betnedge_p_slot;
        let mut var_betnedge_p_rv: f64 = *var_betnedge_p_rv_slot;
        let mut var_cfbedge_p: f64 = *var_cfbedge_p_slot;
        let mut var_cfbedge_p_rv: f64 = *var_cfbedge_p_rv_slot;
        let mut var_cfdedge_p: f64 = *var_cfdedge_p_slot;
        let mut var_cfdedge_p_rv: f64 = *var_cfdedge_p_rv_slot;
        let mut var_cfedge_p: f64 = *var_cfedge_p_slot;
        let mut var_cfedge_p_rv: f64 = *var_cfedge_p_rv_slot;
        let mut var_cfr_p: f64 = *var_cfr_p_slot;
        let mut var_cfr_p_rv: f64 = *var_cfr_p_rv_slot;
        let mut var_cfrd_p: f64 = *var_cfrd_p_slot;
        let mut var_cfrd_p_rv: f64 = *var_cfrd_p_rv_slot;
        let mut var_cgbov_p: f64 = *var_cgbov_p_slot;
        let mut var_cgbov_p_rv: f64 = *var_cgbov_p_rv_slot;
        let mut var_cgov_p: f64 = *var_cgov_p_slot;
        let mut var_cgov_p_rv: f64 = *var_cgov_p_rv_slot;
        let mut var_cgovd_p: f64 = *var_cgovd_p_slot;
        let mut var_cgovd_p_rv: f64 = *var_cgovd_p_rv_slot;
        let mut var_cinr_p: f64 = *var_cinr_p_slot;
        let mut var_cinr_p_rv: f64 = *var_cinr_p_rv_slot;
        let mut var_cinrd_p: f64 = *var_cinrd_p_slot;
        let mut var_cinrd_p_rv: f64 = *var_cinrd_p_rv_slot;
        let mut var_ctedge_p: f64 = *var_ctedge_p_slot;
        let mut var_ctedge_p_rv: f64 = *var_ctedge_p_rv_slot;
        let mut var_dphibedge_p: f64 = *var_dphibedge_p_slot;
        let mut var_dphibedge_p_rv: f64 = *var_dphibedge_p_rv_slot;
        let mut var_guard114: f64 = *var_guard114_slot;
        let mut var_guard114_rv: f64 = *var_guard114_rv_slot;
        let mut var_guard115: f64 = *var_guard115_slot;
        let mut var_guard115_rv: f64 = *var_guard115_rv_slot;
        let mut var_guard116: f64 = *var_guard116_slot;
        let mut var_guard116_rv: f64 = *var_guard116_rv_slot;
        let mut var_guard117: f64 = *var_guard117_slot;
        let mut var_guard117_rv: f64 = *var_guard117_rv_slot;
        let mut var_guard118: f64 = *var_guard118_slot;
        let mut var_guard118_rv: f64 = *var_guard118_rv_slot;
        let mut var_guard119: f64 = *var_guard119_slot;
        let mut var_guard119_rv: f64 = *var_guard119_rv_slot;
        let mut var_guard120: f64 = *var_guard120_slot;
        let mut var_guard120_rv: f64 = *var_guard120_rv_slot;
        let mut var_guard121: f64 = *var_guard121_slot;
        let mut var_guard121_rv: f64 = *var_guard121_rv_slot;
        let mut var_guard122: f64 = *var_guard122_slot;
        let mut var_guard122_rv: f64 = *var_guard122_rv_slot;
        let mut var_guard127: f64 = *var_guard127_slot;
        let mut var_guard127_rv: f64 = *var_guard127_rv_slot;
        let mut var_guard128: f64 = *var_guard128_slot;
        let mut var_guard128_rv: f64 = *var_guard128_rv_slot;
        let mut var_guard129: f64 = *var_guard129_slot;
        let mut var_guard129_rv: f64 = *var_guard129_rv_slot;
        let mut var_guard130: f64 = *var_guard130_slot;
        let mut var_guard130_rv: f64 = *var_guard130_rv_slot;
        let mut var_guard131: f64 = *var_guard131_slot;
        let mut var_guard131_rv: f64 = *var_guard131_rv_slot;
        let mut var_guard132: f64 = *var_guard132_slot;
        let mut var_guard132_rv: f64 = *var_guard132_rv_slot;
        let mut var_guard133: f64 = *var_guard133_slot;
        let mut var_guard133_rv: f64 = *var_guard133_rv_slot;
        let mut var_guard134: f64 = *var_guard134_slot;
        let mut var_guard134_rv: f64 = *var_guard134_rv_slot;
        let mut var_guard135: f64 = *var_guard135_slot;
        let mut var_guard135_rv: f64 = *var_guard135_rv_slot;
        let mut var_guard136: f64 = *var_guard136_slot;
        let mut var_guard136_rv: f64 = *var_guard136_rv_slot;
        let mut var_guard137: f64 = *var_guard137_slot;
        let mut var_guard137_rv: f64 = *var_guard137_rv_slot;
        let mut var_guard138: f64 = *var_guard138_slot;
        let mut var_guard138_rv: f64 = *var_guard138_rv_slot;
        let mut var_guard139: f64 = *var_guard139_slot;
        let mut var_guard139_rv: f64 = *var_guard139_rv_slot;
        let mut var_kvsatac_i: f64 = *var_kvsatac_i_slot;
        let mut var_kvsatac_i_rv: f64 = *var_kvsatac_i_rv_slot;
        let mut var_loop_: f64 = *var_loop__slot;
        let mut var_loop__rv: f64 = *var_loop__rv_slot;
        let mut var_neffedge_p: f64 = *var_neffedge_p_slot;
        let mut var_neffedge_p_rv: f64 = *var_neffedge_p_rv_slot;
        let mut var_pscebedge_p: f64 = *var_pscebedge_p_slot;
        let mut var_pscebedge_p_rv: f64 = *var_pscebedge_p_rv_slot;
        let mut var_pscededge_p: f64 = *var_pscededge_p_slot;
        let mut var_pscededge_p_rv: f64 = *var_pscededge_p_rv_slot;
        let mut var_psceedge_p: f64 = *var_psceedge_p_slot;
        let mut var_psceedge_p_rv: f64 = *var_psceedge_p_rv_slot;
        let mut var_stbetedge_p: f64 = *var_stbetedge_p_slot;
        let mut var_stbetedge_p_rv: f64 = *var_stbetedge_p_rv_slot;
        let mut var_stvfbedge_p: f64 = *var_stvfbedge_p_slot;
        let mut var_stvfbedge_p_rv: f64 = *var_stvfbedge_p_rv_slot;
        let mut var_tmpa: f64 = *var_tmpa_slot;
        let mut var_tmpa_rv: f64 = *var_tmpa_rv_slot;
        let mut var_tmpb: f64 = *var_tmpb_slot;
        let mut var_tmpb_rv: f64 = *var_tmpb_rv_slot;
        let mut var_vfbedge_p: f64 = *var_vfbedge_p_slot;
        let mut var_vfbedge_p_rv: f64 = *var_vfbedge_p_rv_slot;

        let (assign8500_e7804,) = {
    if ((var_guard36 != 0.0) && (var_guard109 != 0.0)) {
        let assign8500_e7792: f64 = (var_plparam_i * var_ile);
        let assign8500_e7793: f64 = (var_poparam_i + assign8500_e7792);
        let assign8500_e7796: f64 = (var_pwparam_i * var_iwe);
        let assign8500_e7797: f64 = (assign8500_e7793 + assign8500_e7796);
        let assign8500_e7800: f64 = (var_plwparam_i * var_iae);
        let assign8500_e7801: f64 = (assign8500_e7797 + assign8500_e7800);
        let assign8500_e7802: f64 = assign8500_e7801;
        (assign8500_e7802,)
    } else {
        (var_axac_p,)
    }
};
        var_axac_p = assign8500_e7804;
        var_axac_p_rv = 0.0;

        let assign8510_e7823: f64 = if (((param_given[668] || param_given[669]) || param_given[670]) || param_given[671]) { 1.0 } else { 0.0 };
        var_guard114 = assign8510_e7823;
        var_guard114_rv = 0.0;

        let (assign8520_e7843,) = {
    if ((var_guard36 != 0.0) && (var_guard114 != 0.0)) {
        let assign8520_e7831: f64 = (p.p669 * var_ile);
        let assign8520_e7832: f64 = (p.p668 + assign8520_e7831);
        let assign8520_e7835: f64 = (p.p670 * var_iwe);
        let assign8520_e7836: f64 = (assign8520_e7832 + assign8520_e7835);
        let assign8520_e7839: f64 = (p.p671 * var_iae);
        let assign8520_e7840: f64 = (assign8520_e7836 + assign8520_e7839);
        let assign8520_e7841: f64 = (var_ile * assign8520_e7840);
        (assign8520_e7841,)
    } else {
        (var_alpac_p,)
    }
};
        var_alpac_p = assign8520_e7843;
        var_alpac_p_rv = 0.0;

        let assign8530_e7862: f64 = if (((param_given[672] || param_given[673]) || param_given[674]) || param_given[675]) { 1.0 } else { 0.0 };
        var_guard115 = assign8530_e7862;
        var_guard115_rv = 0.0;

        let (assign8540_e7882,) = {
    if ((var_guard36 != 0.0) && (var_guard115 != 0.0)) {
        let assign8540_e7870: f64 = (p.p673 * var_ile);
        let assign8540_e7871: f64 = (p.p672 + assign8540_e7870);
        let assign8540_e7874: f64 = (p.p674 * var_iwe);
        let assign8540_e7875: f64 = (assign8540_e7871 + assign8540_e7874);
        let assign8540_e7878: f64 = (p.p675 * var_iae);
        let assign8540_e7879: f64 = (assign8540_e7875 + assign8540_e7878);
        let assign8540_e7880: f64 = (var_ile * assign8540_e7879);
        (assign8540_e7880,)
    } else {
        (var_alp1ac_p,)
    }
};
        var_alp1ac_p = assign8540_e7882;
        var_alp1ac_p_rv = 0.0;

        let assign8550_e7901: f64 = if (((param_given[676] || param_given[677]) || param_given[678]) || param_given[679]) { 1.0 } else { 0.0 };
        var_guard116 = assign8550_e7901;
        var_guard116_rv = 0.0;

        let (assign8560_e7921,) = {
    if ((var_guard36 != 0.0) && (var_guard116 != 0.0)) {
        let assign8560_e7909: f64 = (p.p677 * var_ile);
        let assign8560_e7910: f64 = (p.p676 + assign8560_e7909);
        let assign8560_e7913: f64 = (p.p678 * var_iwe);
        let assign8560_e7914: f64 = (assign8560_e7910 + assign8560_e7913);
        let assign8560_e7917: f64 = (p.p679 * var_iae);
        let assign8560_e7918: f64 = (assign8560_e7914 + assign8560_e7917);
        let assign8560_e7919: f64 = (var_iiwecv * assign8560_e7918);
        (assign8560_e7919,)
    } else {
        (var_cgov_p,)
    }
};
        var_cgov_p = assign8560_e7921;
        var_cgov_p_rv = 0.0;

        let assign8570_e7940: f64 = if (((param_given[680] || param_given[681]) || param_given[682]) || param_given[683]) { 1.0 } else { 0.0 };
        var_guard117 = assign8570_e7940;
        var_guard117_rv = 0.0;

        let (assign8580_e7960,) = {
    if ((var_guard36 != 0.0) && (var_guard117 != 0.0)) {
        let assign8580_e7948: f64 = (p.p681 * var_ile);
        let assign8580_e7949: f64 = (p.p680 + assign8580_e7948);
        let assign8580_e7952: f64 = (p.p682 * var_iwe);
        let assign8580_e7953: f64 = (assign8580_e7949 + assign8580_e7952);
        let assign8580_e7956: f64 = (p.p683 * var_iae);
        let assign8580_e7957: f64 = (assign8580_e7953 + assign8580_e7956);
        let assign8580_e7958: f64 = (var_iiwecv * assign8580_e7957);
        (assign8580_e7958,)
    } else {
        (var_cgovd_p,)
    }
};
        var_cgovd_p = assign8580_e7960;
        var_cgovd_p_rv = 0.0;

        let assign8590_e7979: f64 = if (((param_given[684] || param_given[685]) || param_given[686]) || param_given[687]) { 1.0 } else { 0.0 };
        var_guard118 = assign8590_e7979;
        var_guard118_rv = 0.0;

        let (assign8600_e7999,) = {
    if ((var_guard36 != 0.0) && (var_guard118 != 0.0)) {
        let assign8600_e7987: f64 = (p.p685 * var_ile);
        let assign8600_e7988: f64 = (p.p684 + assign8600_e7987);
        let assign8600_e7991: f64 = (p.p686 * var_iwe);
        let assign8600_e7992: f64 = (assign8600_e7988 + assign8600_e7991);
        let assign8600_e7995: f64 = (p.p687 * var_iae);
        let assign8600_e7996: f64 = (assign8600_e7992 + assign8600_e7995);
        let assign8600_e7997: f64 = (var_iilcv * assign8600_e7996);
        (assign8600_e7997,)
    } else {
        (var_cgbov_p,)
    }
};
        var_cgbov_p = assign8600_e7999;
        var_cgbov_p_rv = 0.0;

        let assign8610_e8018: f64 = if (((param_given[688] || param_given[689]) || param_given[690]) || param_given[691]) { 1.0 } else { 0.0 };
        var_guard119 = assign8610_e8018;
        var_guard119_rv = 0.0;

        let (assign8620_e8038,) = {
    if ((var_guard36 != 0.0) && (var_guard119 != 0.0)) {
        let assign8620_e8026: f64 = (p.p689 * var_ile);
        let assign8620_e8027: f64 = (p.p688 + assign8620_e8026);
        let assign8620_e8030: f64 = (p.p690 * var_iwe);
        let assign8620_e8031: f64 = (assign8620_e8027 + assign8620_e8030);
        let assign8620_e8034: f64 = (p.p691 * var_iae);
        let assign8620_e8035: f64 = (assign8620_e8031 + assign8620_e8034);
        let assign8620_e8036: f64 = (var_iiwecv * assign8620_e8035);
        (assign8620_e8036,)
    } else {
        (var_cinr_p,)
    }
};
        var_cinr_p = assign8620_e8038;
        var_cinr_p_rv = 0.0;

        let assign8630_e8057: f64 = if (((param_given[692] || param_given[693]) || param_given[694]) || param_given[695]) { 1.0 } else { 0.0 };
        var_guard120 = assign8630_e8057;
        var_guard120_rv = 0.0;

        let (assign8640_e8077,) = {
    if ((var_guard36 != 0.0) && (var_guard120 != 0.0)) {
        let assign8640_e8065: f64 = (p.p693 * var_ile);
        let assign8640_e8066: f64 = (p.p692 + assign8640_e8065);
        let assign8640_e8069: f64 = (p.p694 * var_iwe);
        let assign8640_e8070: f64 = (assign8640_e8066 + assign8640_e8069);
        let assign8640_e8073: f64 = (p.p695 * var_iae);
        let assign8640_e8074: f64 = (assign8640_e8070 + assign8640_e8073);
        let assign8640_e8075: f64 = (var_iiwecv * assign8640_e8074);
        (assign8640_e8075,)
    } else {
        (var_cinrd_p,)
    }
};
        var_cinrd_p = assign8640_e8077;
        var_cinrd_p_rv = 0.0;

        let assign8650_e8096: f64 = if (((param_given[696] || param_given[697]) || param_given[698]) || param_given[699]) { 1.0 } else { 0.0 };
        var_guard121 = assign8650_e8096;
        var_guard121_rv = 0.0;

        let (assign8660_e8116,) = {
    if ((var_guard36 != 0.0) && (var_guard121 != 0.0)) {
        let assign8660_e8104: f64 = (p.p697 * var_ile);
        let assign8660_e8105: f64 = (p.p696 + assign8660_e8104);
        let assign8660_e8108: f64 = (p.p698 * var_iwe);
        let assign8660_e8109: f64 = (assign8660_e8105 + assign8660_e8108);
        let assign8660_e8112: f64 = (p.p699 * var_iae);
        let assign8660_e8113: f64 = (assign8660_e8109 + assign8660_e8112);
        let assign8660_e8114: f64 = (var_iiwcv * assign8660_e8113);
        (assign8660_e8114,)
    } else {
        (var_cfr_p,)
    }
};
        var_cfr_p = assign8660_e8116;
        var_cfr_p_rv = 0.0;

        let assign8670_e8135: f64 = if (((param_given[700] || param_given[701]) || param_given[702]) || param_given[703]) { 1.0 } else { 0.0 };
        var_guard122 = assign8670_e8135;
        var_guard122_rv = 0.0;

        let (assign8680_e8155,) = {
    if ((var_guard36 != 0.0) && (var_guard122 != 0.0)) {
        let assign8680_e8143: f64 = (p.p701 * var_ile);
        let assign8680_e8144: f64 = (p.p700 + assign8680_e8143);
        let assign8680_e8147: f64 = (p.p702 * var_iwe);
        let assign8680_e8148: f64 = (assign8680_e8144 + assign8680_e8147);
        let assign8680_e8151: f64 = (p.p703 * var_iae);
        let assign8680_e8152: f64 = (assign8680_e8148 + assign8680_e8151);
        let assign8680_e8153: f64 = (var_iiwcv * assign8680_e8152);
        (assign8680_e8153,)
    } else {
        (var_cfrd_p,)
    }
};
        var_cfrd_p = assign8680_e8155;
        var_cfrd_p_rv = 0.0;

        let assign8770_e8330: f64 = if (((param_given[720] || param_given[721]) || param_given[722]) || param_given[723]) { 1.0 } else { 0.0 };
        var_guard127 = assign8770_e8330;
        var_guard127_rv = 0.0;

        let (assign8780_e8348,) = {
    if ((var_guard36 != 0.0) && (var_guard127 != 0.0)) {
        let assign8780_e8337: f64 = (p.p721 * var_ile);
        let assign8780_e8338: f64 = (p.p720 + assign8780_e8337);
        let assign8780_e8341: f64 = (p.p722 * var_iwe);
        let assign8780_e8342: f64 = (assign8780_e8338 + assign8780_e8341);
        let assign8780_e8345: f64 = (p.p723 * var_iae);
        let assign8780_e8346: f64 = (assign8780_e8342 + assign8780_e8345);
        (assign8780_e8346,)
    } else {
        (var_vfbedge_p,)
    }
};
        var_vfbedge_p = assign8780_e8348;
        var_vfbedge_p_rv = 0.0;

        let assign8790_e8367: f64 = if (((param_given[724] || param_given[725]) || param_given[726]) || param_given[727]) { 1.0 } else { 0.0 };
        var_guard128 = assign8790_e8367;
        var_guard128_rv = 0.0;

        let (assign8800_e8385,) = {
    if ((var_guard36 != 0.0) && (var_guard128 != 0.0)) {
        let assign8800_e8374: f64 = (p.p725 * var_ile);
        let assign8800_e8375: f64 = (p.p724 + assign8800_e8374);
        let assign8800_e8378: f64 = (p.p726 * var_iwe);
        let assign8800_e8379: f64 = (assign8800_e8375 + assign8800_e8378);
        let assign8800_e8382: f64 = (p.p727 * var_iae);
        let assign8800_e8383: f64 = (assign8800_e8379 + assign8800_e8382);
        (assign8800_e8383,)
    } else {
        (var_stvfbedge_p,)
    }
};
        var_stvfbedge_p = assign8800_e8385;
        var_stvfbedge_p_rv = 0.0;

        let assign8810_e8404: f64 = if (((param_given[728] || param_given[729]) || param_given[730]) || param_given[731]) { 1.0 } else { 0.0 };
        var_guard129 = assign8810_e8404;
        var_guard129_rv = 0.0;

        let (assign8820_e8422,) = {
    if ((var_guard36 != 0.0) && (var_guard129 != 0.0)) {
        let assign8820_e8411: f64 = (p.p729 * var_ile);
        let assign8820_e8412: f64 = (p.p728 + assign8820_e8411);
        let assign8820_e8415: f64 = (p.p730 * var_iwe);
        let assign8820_e8416: f64 = (assign8820_e8412 + assign8820_e8415);
        let assign8820_e8419: f64 = (p.p731 * var_iae);
        let assign8820_e8420: f64 = (assign8820_e8416 + assign8820_e8419);
        (assign8820_e8420,)
    } else {
        (var_dphibedge_p,)
    }
};
        var_dphibedge_p = assign8820_e8422;
        var_dphibedge_p_rv = 0.0;

        let assign8830_e8441: f64 = if (((param_given[732] || param_given[733]) || param_given[734]) || param_given[735]) { 1.0 } else { 0.0 };
        var_guard130 = assign8830_e8441;
        var_guard130_rv = 0.0;

        let (assign8840_e8459,) = {
    if ((var_guard36 != 0.0) && (var_guard130 != 0.0)) {
        let assign8840_e8448: f64 = (p.p733 * var_ile);
        let assign8840_e8449: f64 = (p.p732 + assign8840_e8448);
        let assign8840_e8452: f64 = (p.p734 * var_iwe);
        let assign8840_e8453: f64 = (assign8840_e8449 + assign8840_e8452);
        let assign8840_e8456: f64 = (p.p735 * var_iae);
        let assign8840_e8457: f64 = (assign8840_e8453 + assign8840_e8456);
        (assign8840_e8457,)
    } else {
        (var_neffedge_p,)
    }
};
        var_neffedge_p = assign8840_e8459;
        var_neffedge_p_rv = 0.0;

        let assign8850_e8478: f64 = if (((param_given[736] || param_given[737]) || param_given[738]) || param_given[739]) { 1.0 } else { 0.0 };
        var_guard131 = assign8850_e8478;
        var_guard131_rv = 0.0;

        let (assign8860_e8496,) = {
    if ((var_guard36 != 0.0) && (var_guard131 != 0.0)) {
        let assign8860_e8485: f64 = (p.p737 * var_ile);
        let assign8860_e8486: f64 = (p.p736 + assign8860_e8485);
        let assign8860_e8489: f64 = (p.p738 * var_iwe);
        let assign8860_e8490: f64 = (assign8860_e8486 + assign8860_e8489);
        let assign8860_e8493: f64 = (p.p739 * var_iae);
        let assign8860_e8494: f64 = (assign8860_e8490 + assign8860_e8493);
        (assign8860_e8494,)
    } else {
        (var_ctedge_p,)
    }
};
        var_ctedge_p = assign8860_e8496;
        var_ctedge_p_rv = 0.0;

        let assign8870_e8515: f64 = if (((param_given[740] || param_given[741]) || param_given[742]) || param_given[743]) { 1.0 } else { 0.0 };
        var_guard132 = assign8870_e8515;
        var_guard132_rv = 0.0;

        let (assign8880_e8537,) = {
    if ((var_guard36 != 0.0) && (var_guard132 != 0.0)) {
        let assign8880_e8521: f64 = (var_we_edge / var_le);
        let assign8880_e8525: f64 = (p.p741 * var_ile);
        let assign8880_e8526: f64 = (p.p740 + assign8880_e8525);
        let assign8880_e8529: f64 = (p.p742 * var_iwe);
        let assign8880_e8530: f64 = (assign8880_e8526 + assign8880_e8529);
        let assign8880_e8533: f64 = (p.p743 * var_iae);
        let assign8880_e8534: f64 = (assign8880_e8530 + assign8880_e8533);
        let assign8880_e8535: f64 = (assign8880_e8521 * assign8880_e8534);
        (assign8880_e8535,)
    } else {
        (var_betnedge_p,)
    }
};
        var_betnedge_p = assign8880_e8537;
        var_betnedge_p_rv = 0.0;

        let assign8890_e8556: f64 = if (((param_given[744] || param_given[745]) || param_given[746]) || param_given[747]) { 1.0 } else { 0.0 };
        var_guard133 = assign8890_e8556;
        var_guard133_rv = 0.0;

        let (assign8900_e8574,) = {
    if ((var_guard36 != 0.0) && (var_guard133 != 0.0)) {
        let assign8900_e8563: f64 = (p.p745 * var_ile);
        let assign8900_e8564: f64 = (p.p744 + assign8900_e8563);
        let assign8900_e8567: f64 = (p.p746 * var_iwe);
        let assign8900_e8568: f64 = (assign8900_e8564 + assign8900_e8567);
        let assign8900_e8571: f64 = (p.p747 * var_iae);
        let assign8900_e8572: f64 = (assign8900_e8568 + assign8900_e8571);
        (assign8900_e8572,)
    } else {
        (var_stbetedge_p,)
    }
};
        var_stbetedge_p = assign8900_e8574;
        var_stbetedge_p_rv = 0.0;

        let assign8910_e8593: f64 = if (((param_given[748] || param_given[749]) || param_given[750]) || param_given[751]) { 1.0 } else { 0.0 };
        var_guard134 = assign8910_e8593;
        var_guard134_rv = 0.0;

        let (assign8920_e8613,) = {
    if ((var_guard36 != 0.0) && (var_guard134 != 0.0)) {
        let assign8920_e8601: f64 = (p.p749 * var_ile);
        let assign8920_e8602: f64 = (p.p748 + assign8920_e8601);
        let assign8920_e8605: f64 = (p.p750 * var_iwe);
        let assign8920_e8606: f64 = (assign8920_e8602 + assign8920_e8605);
        let assign8920_e8609: f64 = (p.p751 * var_iae);
        let assign8920_e8610: f64 = (assign8920_e8606 + assign8920_e8609);
        let assign8920_e8611: f64 = (var_ile2 * assign8920_e8610);
        (assign8920_e8611,)
    } else {
        (var_psceedge_p,)
    }
};
        var_psceedge_p = assign8920_e8613;
        var_psceedge_p_rv = 0.0;

        let assign8930_e8632: f64 = if (((param_given[752] || param_given[753]) || param_given[754]) || param_given[755]) { 1.0 } else { 0.0 };
        var_guard135 = assign8930_e8632;
        var_guard135_rv = 0.0;

        let (assign8940_e8650,) = {
    if ((var_guard36 != 0.0) && (var_guard135 != 0.0)) {
        let assign8940_e8639: f64 = (p.p753 * var_ile);
        let assign8940_e8640: f64 = (p.p752 + assign8940_e8639);
        let assign8940_e8643: f64 = (p.p754 * var_iwe);
        let assign8940_e8644: f64 = (assign8940_e8640 + assign8940_e8643);
        let assign8940_e8647: f64 = (p.p755 * var_iae);
        let assign8940_e8648: f64 = (assign8940_e8644 + assign8940_e8647);
        (assign8940_e8648,)
    } else {
        (var_pscebedge_p,)
    }
};
        var_pscebedge_p = assign8940_e8650;
        var_pscebedge_p_rv = 0.0;

        let assign8950_e8669: f64 = if (((param_given[756] || param_given[757]) || param_given[758]) || param_given[759]) { 1.0 } else { 0.0 };
        var_guard136 = assign8950_e8669;
        var_guard136_rv = 0.0;

        let (assign8960_e8687,) = {
    if ((var_guard36 != 0.0) && (var_guard136 != 0.0)) {
        let assign8960_e8676: f64 = (p.p757 * var_ile);
        let assign8960_e8677: f64 = (p.p756 + assign8960_e8676);
        let assign8960_e8680: f64 = (p.p758 * var_iwe);
        let assign8960_e8681: f64 = (assign8960_e8677 + assign8960_e8680);
        let assign8960_e8684: f64 = (p.p759 * var_iae);
        let assign8960_e8685: f64 = (assign8960_e8681 + assign8960_e8684);
        (assign8960_e8685,)
    } else {
        (var_pscededge_p,)
    }
};
        var_pscededge_p = assign8960_e8687;
        var_pscededge_p_rv = 0.0;

        let assign8970_e8706: f64 = if (((param_given[760] || param_given[761]) || param_given[762]) || param_given[763]) { 1.0 } else { 0.0 };
        var_guard137 = assign8970_e8706;
        var_guard137_rv = 0.0;

        let (assign8980_e8726,) = {
    if ((var_guard36 != 0.0) && (var_guard137 != 0.0)) {
        let assign8980_e8714: f64 = (p.p761 * var_ile);
        let assign8980_e8715: f64 = (p.p760 + assign8980_e8714);
        let assign8980_e8718: f64 = (p.p762 * var_iwe);
        let assign8980_e8719: f64 = (assign8980_e8715 + assign8980_e8718);
        let assign8980_e8722: f64 = (p.p763 * var_iae);
        let assign8980_e8723: f64 = (assign8980_e8719 + assign8980_e8722);
        let assign8980_e8724: f64 = (var_ile2 * assign8980_e8723);
        (assign8980_e8724,)
    } else {
        (var_cfedge_p,)
    }
};
        var_cfedge_p = assign8980_e8726;
        var_cfedge_p_rv = 0.0;

        let assign8990_e8745: f64 = if (((param_given[768] || param_given[769]) || param_given[770]) || param_given[771]) { 1.0 } else { 0.0 };
        var_guard138 = assign8990_e8745;
        var_guard138_rv = 0.0;

        let (assign9000_e8763,) = {
    if ((var_guard36 != 0.0) && (var_guard138 != 0.0)) {
        let assign9000_e8752: f64 = (p.p769 * var_ile);
        let assign9000_e8753: f64 = (p.p768 + assign9000_e8752);
        let assign9000_e8756: f64 = (p.p770 * var_iwe);
        let assign9000_e8757: f64 = (assign9000_e8753 + assign9000_e8756);
        let assign9000_e8760: f64 = (p.p771 * var_iae);
        let assign9000_e8761: f64 = (assign9000_e8757 + assign9000_e8760);
        (assign9000_e8761,)
    } else {
        (var_cfdedge_p,)
    }
};
        var_cfdedge_p = assign9000_e8763;
        var_cfdedge_p_rv = 0.0;

        let assign9010_e8782: f64 = if (((param_given[764] || param_given[765]) || param_given[766]) || param_given[767]) { 1.0 } else { 0.0 };
        var_guard139 = assign9010_e8782;
        var_guard139_rv = 0.0;

        let (assign9020_e8800,) = {
    if ((var_guard36 != 0.0) && (var_guard139 != 0.0)) {
        let assign9020_e8789: f64 = (p.p765 * var_ile);
        let assign9020_e8790: f64 = (p.p764 + assign9020_e8789);
        let assign9020_e8793: f64 = (p.p766 * var_iwe);
        let assign9020_e8794: f64 = (assign9020_e8790 + assign9020_e8793);
        let assign9020_e8797: f64 = (p.p767 * var_iae);
        let assign9020_e8798: f64 = (assign9020_e8794 + assign9020_e8797);
        (assign9020_e8798,)
    } else {
        (var_cfbedge_p,)
    }
};
        var_cfbedge_p = assign9020_e8800;
        var_cfbedge_p_rv = 0.0;

        let (assign9090_e8921,) = {
    if (var_guard36 != 0.0) {
        (0.0,)
    } else {
        (var_tmpa,)
    }
};
        var_tmpa = assign9090_e8921;
        var_tmpa_rv = 0.0;

        let (assign9100_e8925,) = {
    if (var_guard36 != 0.0) {
        (0.0,)
    } else {
        (var_tmpb,)
    }
};
        var_tmpb = assign9100_e8925;
        var_tmpb_rv = 0.0;

        let (assign9110_e8929,) = {
    if (var_guard36 != 0.0) {
        (0.0,)
    } else {
        (var_loop_,)
    }
};
        var_loop_ = assign9110_e8929;
        var_loop__rv = 0.0;

        let (assign9120_e8933,) = {
    if (var_guard36 != 0.0) {
        (p.p788,)
    } else {
        (var_kvsatac_i,)
    }
};
        var_kvsatac_i = assign9120_e8933;
        var_kvsatac_i_rv = 0.0;

        *var_alp1ac_p_slot = var_alp1ac_p;
        *var_alp1ac_p_rv_slot = var_alp1ac_p_rv;
        *var_alpac_p_slot = var_alpac_p;
        *var_alpac_p_rv_slot = var_alpac_p_rv;
        *var_axac_p_slot = var_axac_p;
        *var_axac_p_rv_slot = var_axac_p_rv;
        *var_betnedge_p_slot = var_betnedge_p;
        *var_betnedge_p_rv_slot = var_betnedge_p_rv;
        *var_cfbedge_p_slot = var_cfbedge_p;
        *var_cfbedge_p_rv_slot = var_cfbedge_p_rv;
        *var_cfdedge_p_slot = var_cfdedge_p;
        *var_cfdedge_p_rv_slot = var_cfdedge_p_rv;
        *var_cfedge_p_slot = var_cfedge_p;
        *var_cfedge_p_rv_slot = var_cfedge_p_rv;
        *var_cfr_p_slot = var_cfr_p;
        *var_cfr_p_rv_slot = var_cfr_p_rv;
        *var_cfrd_p_slot = var_cfrd_p;
        *var_cfrd_p_rv_slot = var_cfrd_p_rv;
        *var_cgbov_p_slot = var_cgbov_p;
        *var_cgbov_p_rv_slot = var_cgbov_p_rv;
        *var_cgov_p_slot = var_cgov_p;
        *var_cgov_p_rv_slot = var_cgov_p_rv;
        *var_cgovd_p_slot = var_cgovd_p;
        *var_cgovd_p_rv_slot = var_cgovd_p_rv;
        *var_cinr_p_slot = var_cinr_p;
        *var_cinr_p_rv_slot = var_cinr_p_rv;
        *var_cinrd_p_slot = var_cinrd_p;
        *var_cinrd_p_rv_slot = var_cinrd_p_rv;
        *var_ctedge_p_slot = var_ctedge_p;
        *var_ctedge_p_rv_slot = var_ctedge_p_rv;
        *var_dphibedge_p_slot = var_dphibedge_p;
        *var_dphibedge_p_rv_slot = var_dphibedge_p_rv;
        *var_guard114_slot = var_guard114;
        *var_guard114_rv_slot = var_guard114_rv;
        *var_guard115_slot = var_guard115;
        *var_guard115_rv_slot = var_guard115_rv;
        *var_guard116_slot = var_guard116;
        *var_guard116_rv_slot = var_guard116_rv;
        *var_guard117_slot = var_guard117;
        *var_guard117_rv_slot = var_guard117_rv;
        *var_guard118_slot = var_guard118;
        *var_guard118_rv_slot = var_guard118_rv;
        *var_guard119_slot = var_guard119;
        *var_guard119_rv_slot = var_guard119_rv;
        *var_guard120_slot = var_guard120;
        *var_guard120_rv_slot = var_guard120_rv;
        *var_guard121_slot = var_guard121;
        *var_guard121_rv_slot = var_guard121_rv;
        *var_guard122_slot = var_guard122;
        *var_guard122_rv_slot = var_guard122_rv;
        *var_guard127_slot = var_guard127;
        *var_guard127_rv_slot = var_guard127_rv;
        *var_guard128_slot = var_guard128;
        *var_guard128_rv_slot = var_guard128_rv;
        *var_guard129_slot = var_guard129;
        *var_guard129_rv_slot = var_guard129_rv;
        *var_guard130_slot = var_guard130;
        *var_guard130_rv_slot = var_guard130_rv;
        *var_guard131_slot = var_guard131;
        *var_guard131_rv_slot = var_guard131_rv;
        *var_guard132_slot = var_guard132;
        *var_guard132_rv_slot = var_guard132_rv;
        *var_guard133_slot = var_guard133;
        *var_guard133_rv_slot = var_guard133_rv;
        *var_guard134_slot = var_guard134;
        *var_guard134_rv_slot = var_guard134_rv;
        *var_guard135_slot = var_guard135;
        *var_guard135_rv_slot = var_guard135_rv;
        *var_guard136_slot = var_guard136;
        *var_guard136_rv_slot = var_guard136_rv;
        *var_guard137_slot = var_guard137;
        *var_guard137_rv_slot = var_guard137_rv;
        *var_guard138_slot = var_guard138;
        *var_guard138_rv_slot = var_guard138_rv;
        *var_guard139_slot = var_guard139;
        *var_guard139_rv_slot = var_guard139_rv;
        *var_kvsatac_i_slot = var_kvsatac_i;
        *var_kvsatac_i_rv_slot = var_kvsatac_i_rv;
        *var_loop__slot = var_loop_;
        *var_loop__rv_slot = var_loop__rv;
        *var_neffedge_p_slot = var_neffedge_p;
        *var_neffedge_p_rv_slot = var_neffedge_p_rv;
        *var_pscebedge_p_slot = var_pscebedge_p;
        *var_pscebedge_p_rv_slot = var_pscebedge_p_rv;
        *var_pscededge_p_slot = var_pscededge_p;
        *var_pscededge_p_rv_slot = var_pscededge_p_rv;
        *var_psceedge_p_slot = var_psceedge_p;
        *var_psceedge_p_rv_slot = var_psceedge_p_rv;
        *var_stbetedge_p_slot = var_stbetedge_p;
        *var_stbetedge_p_rv_slot = var_stbetedge_p_rv;
        *var_stvfbedge_p_slot = var_stvfbedge_p;
        *var_stvfbedge_p_rv_slot = var_stvfbedge_p_rv;
        *var_tmpa_slot = var_tmpa;
        *var_tmpa_rv_slot = var_tmpa_rv;
        *var_tmpb_slot = var_tmpb;
        *var_tmpb_rv_slot = var_tmpb_rv;
        *var_vfbedge_p_slot = var_vfbedge_p;
        *var_vfbedge_p_rv_slot = var_vfbedge_p_rv;
    }

    pub(super) fn stamp_reactive_block_14(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        var_dellps: f64,
        var_delwod: f64,
        var_guard36: f64,
        var_invnf: f64,
        var_l_i: f64,
        var_nf_i: f64,
        var_rta: f64,
        var_sa_i: f64,
        var_sb_i: f64,
        var_sc_i: f64,
        var_scc_i: f64,
        var_sd_i: f64,
        var_w_i: f64,
        var_betn_p_slot: &mut f64,
        var_betn_p_rv_slot: &mut f64,
        var_betnedge_p_slot: &mut f64,
        var_betnedge_p_rv_slot: &mut f64,
        var_cf_p_slot: &mut f64,
        var_cf_p_rv_slot: &mut f64,
        var_cfedge_p_slot: &mut f64,
        var_cfedge_p_rv_slot: &mut f64,
        var_guard143_slot: &mut f64,
        var_guard143_rv_slot: &mut f64,
        var_guard144_slot: &mut f64,
        var_guard144_rv_slot: &mut f64,
        var_guard145_slot: &mut f64,
        var_guard145_rv_slot: &mut f64,
        var_guard146_slot: &mut f64,
        var_guard146_rv_slot: &mut f64,
        var_invsa_slot: &mut f64,
        var_invsa_rv_slot: &mut f64,
        var_invsaref_slot: &mut f64,
        var_invsaref_rv_slot: &mut f64,
        var_invsb_slot: &mut f64,
        var_invsb_rv_slot: &mut f64,
        var_invsbref_slot: &mut f64,
        var_invsbref_rv_slot: &mut f64,
        var_kstressu0_slot: &mut f64,
        var_kstressu0_rv_slot: &mut f64,
        var_kstressvth0_slot: &mut f64,
        var_kstressvth0_rv_slot: &mut f64,
        var_kvsatac_i_slot: &mut f64,
        var_kvsatac_i_rv_slot: &mut f64,
        var_loop__slot: &mut f64,
        var_loop__rv_slot: &mut f64,
        var_lx_slot: &mut f64,
        var_lx_rv_slot: &mut f64,
        var_rhobeta_slot: &mut f64,
        var_rhobeta_rv_slot: &mut f64,
        var_rhobetaref_slot: &mut f64,
        var_rhobetaref_rv_slot: &mut f64,
        var_sca_i_slot: &mut f64,
        var_sca_i_rv_slot: &mut f64,
        var_scb_i_slot: &mut f64,
        var_scb_i_rv_slot: &mut f64,
        var_temp0_slot: &mut f64,
        var_temp00_slot: &mut f64,
        var_temp00_rv_slot: &mut f64,
        var_temp0_rv_slot: &mut f64,
        var_templ_slot: &mut f64,
        var_templ_rv_slot: &mut f64,
        var_tempw_slot: &mut f64,
        var_tempw_rv_slot: &mut f64,
        var_thesat_p_slot: &mut f64,
        var_thesat_p_rv_slot: &mut f64,
        var_thesatac_p_slot: &mut f64,
        var_thesatac_p_rv_slot: &mut f64,
        var_tmpa_slot: &mut f64,
        var_tmpa_rv_slot: &mut f64,
        var_tmpb_slot: &mut f64,
        var_tmpb_rv_slot: &mut f64,
        var_vfb_p_slot: &mut f64,
        var_vfb_p_rv_slot: &mut f64,
        var_vfbedge_p_slot: &mut f64,
        var_vfbedge_p_rv_slot: &mut f64,
        var_wx_slot: &mut f64,
        var_wx_rv_slot: &mut f64,
    ) {
        let mut var_betn_p: f64 = *var_betn_p_slot;
        let mut var_betn_p_rv: f64 = *var_betn_p_rv_slot;
        let mut var_betnedge_p: f64 = *var_betnedge_p_slot;
        let mut var_betnedge_p_rv: f64 = *var_betnedge_p_rv_slot;
        let mut var_cf_p: f64 = *var_cf_p_slot;
        let mut var_cf_p_rv: f64 = *var_cf_p_rv_slot;
        let mut var_cfedge_p: f64 = *var_cfedge_p_slot;
        let mut var_cfedge_p_rv: f64 = *var_cfedge_p_rv_slot;
        let mut var_guard143: f64 = *var_guard143_slot;
        let mut var_guard143_rv: f64 = *var_guard143_rv_slot;
        let mut var_guard144: f64 = *var_guard144_slot;
        let mut var_guard144_rv: f64 = *var_guard144_rv_slot;
        let mut var_guard145: f64 = *var_guard145_slot;
        let mut var_guard145_rv: f64 = *var_guard145_rv_slot;
        let mut var_guard146: f64 = *var_guard146_slot;
        let mut var_guard146_rv: f64 = *var_guard146_rv_slot;
        let mut var_invsa: f64 = *var_invsa_slot;
        let mut var_invsa_rv: f64 = *var_invsa_rv_slot;
        let mut var_invsaref: f64 = *var_invsaref_slot;
        let mut var_invsaref_rv: f64 = *var_invsaref_rv_slot;
        let mut var_invsb: f64 = *var_invsb_slot;
        let mut var_invsb_rv: f64 = *var_invsb_rv_slot;
        let mut var_invsbref: f64 = *var_invsbref_slot;
        let mut var_invsbref_rv: f64 = *var_invsbref_rv_slot;
        let mut var_kstressu0: f64 = *var_kstressu0_slot;
        let mut var_kstressu0_rv: f64 = *var_kstressu0_rv_slot;
        let mut var_kstressvth0: f64 = *var_kstressvth0_slot;
        let mut var_kstressvth0_rv: f64 = *var_kstressvth0_rv_slot;
        let mut var_kvsatac_i: f64 = *var_kvsatac_i_slot;
        let mut var_kvsatac_i_rv: f64 = *var_kvsatac_i_rv_slot;
        let mut var_loop_: f64 = *var_loop__slot;
        let mut var_loop__rv: f64 = *var_loop__rv_slot;
        let mut var_lx: f64 = *var_lx_slot;
        let mut var_lx_rv: f64 = *var_lx_rv_slot;
        let mut var_rhobeta: f64 = *var_rhobeta_slot;
        let mut var_rhobeta_rv: f64 = *var_rhobeta_rv_slot;
        let mut var_rhobetaref: f64 = *var_rhobetaref_slot;
        let mut var_rhobetaref_rv: f64 = *var_rhobetaref_rv_slot;
        let mut var_sca_i: f64 = *var_sca_i_slot;
        let mut var_sca_i_rv: f64 = *var_sca_i_rv_slot;
        let mut var_scb_i: f64 = *var_scb_i_slot;
        let mut var_scb_i_rv: f64 = *var_scb_i_rv_slot;
        let mut var_temp0: f64 = *var_temp0_slot;
        let mut var_temp00: f64 = *var_temp00_slot;
        let mut var_temp00_rv: f64 = *var_temp00_rv_slot;
        let mut var_temp0_rv: f64 = *var_temp0_rv_slot;
        let mut var_templ: f64 = *var_templ_slot;
        let mut var_templ_rv: f64 = *var_templ_rv_slot;
        let mut var_tempw: f64 = *var_tempw_slot;
        let mut var_tempw_rv: f64 = *var_tempw_rv_slot;
        let mut var_thesat_p: f64 = *var_thesat_p_slot;
        let mut var_thesat_p_rv: f64 = *var_thesat_p_rv_slot;
        let mut var_thesatac_p: f64 = *var_thesatac_p_slot;
        let mut var_thesatac_p_rv: f64 = *var_thesatac_p_rv_slot;
        let mut var_tmpa: f64 = *var_tmpa_slot;
        let mut var_tmpa_rv: f64 = *var_tmpa_rv_slot;
        let mut var_tmpb: f64 = *var_tmpb_slot;
        let mut var_tmpb_rv: f64 = *var_tmpb_rv_slot;
        let mut var_vfb_p: f64 = *var_vfb_p_slot;
        let mut var_vfb_p_rv: f64 = *var_vfb_p_rv_slot;
        let mut var_vfbedge_p: f64 = *var_vfbedge_p_slot;
        let mut var_vfbedge_p_rv: f64 = *var_vfbedge_p_rv_slot;
        let mut var_wx: f64 = *var_wx_slot;
        let mut var_wx_rv: f64 = *var_wx_rv_slot;

        let assign9130_e8935: f64 = if param_given[789] { 1.0 } else { 0.0 };
        let assign9130_e8937: f64 = if assign9130_e8935 == 1.0 { 1.0 } else { 0.0 };
        var_guard143 = assign9130_e8937;
        var_guard143_rv = 0.0;

        let (assign9140_e8943,) = {
    if ((var_guard36 != 0.0) && (var_guard143 != 0.0)) {
        (p.p789,)
    } else {
        (var_kvsatac_i,)
    }
};
        var_kvsatac_i = assign9140_e8943;
        var_kvsatac_i_rv = 0.0;

        let assign9150_e8962: f64 = if (((var_sa_i > 0.0) && (var_sb_i > 0.0)) && ((var_nf_i == 1.0) || ((var_nf_i > 1.0) && (var_sd_i > 0.0)))) { 1.0 } else { 0.0 };
        var_guard144 = assign9150_e8962;
        var_guard144_rv = 0.0;

        let mut assign9160_loop_guard: usize = 0;
        while {
            let assign9160_cond_e8969: f64 = (var_nf_i - 0.5);
            let assign9160_cond_e8971: f64 = if (((var_guard36 != 0.0) && (var_guard144 != 0.0)) && (var_loop_ < assign9160_cond_e8969)) { 1.0 } else { 0.0 };
            assign9160_cond_e8971 != 0.0
        } {
            assign9160_loop_guard += 1;
            assert!(assign9160_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign9160_body0_e8991,) = {
    if ((var_guard36 != 0.0) && (var_guard144 != 0.0)) {
        let assign9160_body0_e8980: f64 = (0.5 * var_l_i);
        let assign9160_body0_e8981: f64 = (var_sa_i + assign9160_body0_e8980);
        let assign9160_body0_e8985: f64 = (var_sd_i + var_l_i);
        let assign9160_body0_e8986: f64 = (var_loop_ * assign9160_body0_e8985);
        let assign9160_body0_e8987: f64 = (assign9160_body0_e8981 + assign9160_body0_e8986);
        let assign9160_body0_e8988: f64 = (1.0 / assign9160_body0_e8987);
        let assign9160_body0_e8989: f64 = (var_tmpa + assign9160_body0_e8988);
        (assign9160_body0_e8989,)
    } else {
        (var_tmpa,)
    }
};
            var_tmpa = assign9160_body0_e8991;
            var_tmpa_rv = 0.0;
            let (assign9160_body1_e9011,) = {
    if ((var_guard36 != 0.0) && (var_guard144 != 0.0)) {
        let assign9160_body1_e9000: f64 = (0.5 * var_l_i);
        let assign9160_body1_e9001: f64 = (var_sb_i + assign9160_body1_e9000);
        let assign9160_body1_e9005: f64 = (var_sd_i + var_l_i);
        let assign9160_body1_e9006: f64 = (var_loop_ * assign9160_body1_e9005);
        let assign9160_body1_e9007: f64 = (assign9160_body1_e9001 + assign9160_body1_e9006);
        let assign9160_body1_e9008: f64 = (1.0 / assign9160_body1_e9007);
        let assign9160_body1_e9009: f64 = (var_tmpb + assign9160_body1_e9008);
        (assign9160_body1_e9009,)
    } else {
        (var_tmpb,)
    }
};
            var_tmpb = assign9160_body1_e9011;
            var_tmpb_rv = 0.0;
            let (assign9160_body2_e9019,) = {
    if ((var_guard36 != 0.0) && (var_guard144 != 0.0)) {
        let assign9160_body2_e9017: f64 = (var_loop_ + 1.0);
        (assign9160_body2_e9017,)
    } else {
        (var_loop_,)
    }
};
            var_loop_ = assign9160_body2_e9019;
            var_loop__rv = 0.0;
        }

        let (assign9170_e9027,) = {
    if ((var_guard36 != 0.0) && (var_guard144 != 0.0)) {
        let assign9170_e9025: f64 = (var_tmpa * var_invnf);
        (assign9170_e9025,)
    } else {
        (var_invsa,)
    }
};
        var_invsa = assign9170_e9027;
        var_invsa_rv = 0.0;

        let (assign9180_e9035,) = {
    if ((var_guard36 != 0.0) && (var_guard144 != 0.0)) {
        let assign9180_e9033: f64 = (var_tmpb * var_invnf);
        (assign9180_e9033,)
    } else {
        (var_invsb,)
    }
};
        var_invsb = assign9180_e9035;
        var_invsb_rv = 0.0;

        let (assign9190_e9047,) = {
    if ((var_guard36 != 0.0) && (var_guard144 != 0.0)) {
        let assign9190_e9043: f64 = (0.5 * var_l_i);
        let assign9190_e9044: f64 = (p.p784 + assign9190_e9043);
        let assign9190_e9045: f64 = (1.0 / assign9190_e9044);
        (assign9190_e9045,)
    } else {
        (var_invsaref,)
    }
};
        var_invsaref = assign9190_e9047;
        var_invsaref_rv = 0.0;

        let (assign9200_e9059,) = {
    if ((var_guard36 != 0.0) && (var_guard144 != 0.0)) {
        let assign9200_e9055: f64 = (0.5 * var_l_i);
        let assign9200_e9056: f64 = (p.p785 + assign9200_e9055);
        let assign9200_e9057: f64 = (1.0 / assign9200_e9056);
        (assign9200_e9057,)
    } else {
        (var_invsbref,)
    }
};
        var_invsbref = assign9200_e9059;
        var_invsbref_rv = 0.0;

        let (assign9210_e9074,) = {
    if ((var_guard36 != 0.0) && (var_guard144 != 0.0)) {
        let assign9210_e9065: f64 = (var_l_i + var_dellps);
        let (assign9210_e9072,) = {
            if (assign9210_e9065 > 1e-9) {
                let assign9210_e9070: f64 = (var_l_i + var_dellps);
                (assign9210_e9070,)
            } else {
                (1e-9,)
            }
        };
        (assign9210_e9072,)
    } else {
        (var_lx,)
    }
};
        var_lx = assign9210_e9074;
        var_lx_rv = 0.0;

        let (assign9220_e9093,) = {
    if ((var_guard36 != 0.0) && (var_guard144 != 0.0)) {
        let assign9220_e9080: f64 = (var_w_i + var_delwod);
        let assign9220_e9082: f64 = (assign9220_e9080 + p.p786);
        let (assign9220_e9091,) = {
            if (assign9220_e9082 > 1e-9) {
                let assign9220_e9087: f64 = (var_w_i + var_delwod);
                let assign9220_e9089: f64 = (assign9220_e9087 + p.p786);
                (assign9220_e9089,)
            } else {
                (1e-9,)
            }
        };
        (assign9220_e9091,)
    } else {
        (var_wx,)
    }
};
        var_wx = assign9220_e9093;
        var_wx_rv = 0.0;

        let (assign9230_e9103,) = {
    if ((var_guard36 != 0.0) && (var_guard144 != 0.0)) {
        let assign9230_e9100: f64 = (var_lx).powf(p.p794);
        let assign9230_e9101: f64 = (1.0 / assign9230_e9100);
        (assign9230_e9101,)
    } else {
        (var_templ,)
    }
};
        var_templ = assign9230_e9103;
        var_templ_rv = 0.0;

        let (assign9240_e9113,) = {
    if ((var_guard36 != 0.0) && (var_guard144 != 0.0)) {
        let assign9240_e9110: f64 = (var_wx).powf(p.p795);
        let assign9240_e9111: f64 = (1.0 / assign9240_e9110);
        (assign9240_e9111,)
    } else {
        (var_tempw,)
    }
};
        var_tempw = assign9240_e9113;
        var_tempw_rv = 0.0;

        let (assign9250_e9141,) = {
    if ((var_guard36 != 0.0) && (var_guard144 != 0.0)) {
        let assign9250_e9120: f64 = (p.p791 * var_templ);
        let assign9250_e9121: f64 = (1.0 + assign9250_e9120);
        let assign9250_e9124: f64 = (p.p792 * var_tempw);
        let assign9250_e9125: f64 = (assign9250_e9121 + assign9250_e9124);
        let assign9250_e9128: f64 = (p.p793 * var_templ);
        let assign9250_e9130: f64 = (assign9250_e9128 * var_tempw);
        let assign9250_e9131: f64 = (assign9250_e9125 + assign9250_e9130);
        let assign9250_e9136: f64 = (var_rta - 1.0);
        let assign9250_e9137: f64 = (p.p790 * assign9250_e9136);
        let assign9250_e9138: f64 = (1.0 + assign9250_e9137);
        let assign9250_e9139: f64 = (assign9250_e9131 * assign9250_e9138);
        (assign9250_e9139,)
    } else {
        (var_kstressu0,)
    }
};
        var_kstressu0 = assign9250_e9141;
        var_kstressu0_rv = 0.0;

        let (assign9260_e9153,) = {
    if ((var_guard36 != 0.0) && (var_guard144 != 0.0)) {
        let assign9260_e9148: f64 = (var_invsa + var_invsb);
        let assign9260_e9149: f64 = (p.p787 * assign9260_e9148);
        let assign9260_e9151: f64 = (assign9260_e9149 / var_kstressu0);
        (assign9260_e9151,)
    } else {
        (var_rhobeta,)
    }
};
        var_rhobeta = assign9260_e9153;
        var_rhobeta_rv = 0.0;

        let (assign9270_e9165,) = {
    if ((var_guard36 != 0.0) && (var_guard144 != 0.0)) {
        let assign9270_e9160: f64 = (var_invsaref + var_invsbref);
        let assign9270_e9161: f64 = (p.p787 * assign9270_e9160);
        let assign9270_e9163: f64 = (assign9270_e9161 / var_kstressu0);
        (assign9270_e9163,)
    } else {
        (var_rhobetaref,)
    }
};
        var_rhobetaref = assign9270_e9165;
        var_rhobetaref_rv = 0.0;

        let (assign9280_e9175,) = {
    if ((var_guard36 != 0.0) && (var_guard144 != 0.0)) {
        let assign9280_e9172: f64 = (var_lx).powf(p.p800);
        let assign9280_e9173: f64 = (1.0 / assign9280_e9172);
        (assign9280_e9173,)
    } else {
        (var_templ,)
    }
};
        var_templ = assign9280_e9175;
        var_templ_rv = 0.0;

        let (assign9290_e9185,) = {
    if ((var_guard36 != 0.0) && (var_guard144 != 0.0)) {
        let assign9290_e9182: f64 = (var_wx).powf(p.p801);
        let assign9290_e9183: f64 = (1.0 / assign9290_e9182);
        (assign9290_e9183,)
    } else {
        (var_tempw,)
    }
};
        var_tempw = assign9290_e9185;
        var_tempw_rv = 0.0;

        let (assign9300_e9205,) = {
    if ((var_guard36 != 0.0) && (var_guard144 != 0.0)) {
        let assign9300_e9192: f64 = (p.p797 * var_templ);
        let assign9300_e9193: f64 = (1.0 + assign9300_e9192);
        let assign9300_e9196: f64 = (p.p798 * var_tempw);
        let assign9300_e9197: f64 = (assign9300_e9193 + assign9300_e9196);
        let assign9300_e9200: f64 = (p.p799 * var_templ);
        let assign9300_e9202: f64 = (assign9300_e9200 * var_tempw);
        let assign9300_e9203: f64 = (assign9300_e9197 + assign9300_e9202);
        (assign9300_e9203,)
    } else {
        (var_kstressvth0,)
    }
};
        var_kstressvth0 = assign9300_e9205;
        var_kstressvth0_rv = 0.0;

        let (assign9310_e9217,) = {
    if ((var_guard36 != 0.0) && (var_guard144 != 0.0)) {
        let assign9310_e9211: f64 = (var_invsa + var_invsb);
        let assign9310_e9213: f64 = (assign9310_e9211 - var_invsaref);
        let assign9310_e9215: f64 = (assign9310_e9213 - var_invsbref);
        (assign9310_e9215,)
    } else {
        (var_temp0,)
    }
};
        var_temp0 = assign9310_e9217;
        var_temp0_rv = 0.0;

        let (assign9320_e9229,) = {
    if ((var_guard36 != 0.0) && (var_guard144 != 0.0)) {
        let assign9320_e9223: f64 = (1.0 + var_rhobeta);
        let assign9320_e9226: f64 = (1.0 + var_rhobetaref);
        let assign9320_e9227: f64 = (assign9320_e9223 / assign9320_e9226);
        (assign9320_e9227,)
    } else {
        (var_temp00,)
    }
};
        var_temp00 = assign9320_e9229;
        var_temp00_rv = 0.0;

        let (assign9330_e9237,) = {
    if ((var_guard36 != 0.0) && (var_guard144 != 0.0)) {
        let assign9330_e9235: f64 = (var_betn_p * var_temp00);
        (assign9330_e9235,)
    } else {
        (var_betn_p,)
    }
};
        var_betn_p = assign9330_e9237;
        var_betn_p_rv = 0.0;

        let (assign9340_e9257,) = {
    if ((var_guard36 != 0.0) && (var_guard144 != 0.0)) {
        let assign9340_e9243: f64 = (var_thesat_p * var_temp00);
        let assign9340_e9247: f64 = (p.p788 * var_rhobetaref);
        let assign9340_e9248: f64 = (1.0 + assign9340_e9247);
        let assign9340_e9249: f64 = (assign9340_e9243 * assign9340_e9248);
        let assign9340_e9253: f64 = (p.p788 * var_rhobeta);
        let assign9340_e9254: f64 = (1.0 + assign9340_e9253);
        let assign9340_e9255: f64 = (assign9340_e9249 / assign9340_e9254);
        (assign9340_e9255,)
    } else {
        (var_thesat_p,)
    }
};
        var_thesat_p = assign9340_e9257;
        var_thesat_p_rv = 0.0;

        let (assign9350_e9277,) = {
    if ((var_guard36 != 0.0) && (var_guard144 != 0.0)) {
        let assign9350_e9263: f64 = (var_thesatac_p * var_temp00);
        let assign9350_e9267: f64 = (var_kvsatac_i * var_rhobetaref);
        let assign9350_e9268: f64 = (1.0 + assign9350_e9267);
        let assign9350_e9269: f64 = (assign9350_e9263 * assign9350_e9268);
        let assign9350_e9273: f64 = (var_kvsatac_i * var_rhobeta);
        let assign9350_e9274: f64 = (1.0 + assign9350_e9273);
        let assign9350_e9275: f64 = (assign9350_e9269 / assign9350_e9274);
        (assign9350_e9275,)
    } else {
        (var_thesatac_p,)
    }
};
        var_thesatac_p = assign9350_e9277;
        var_thesatac_p_rv = 0.0;

        let (assign9360_e9285,) = {
    if ((var_guard36 != 0.0) && (var_guard144 != 0.0)) {
        let assign9360_e9283: f64 = (var_betnedge_p * var_temp00);
        (assign9360_e9283,)
    } else {
        (var_betnedge_p,)
    }
};
        var_betnedge_p = assign9360_e9285;
        var_betnedge_p_rv = 0.0;

        let (assign9370_e9295,) = {
    if ((var_guard36 != 0.0) && (var_guard144 != 0.0)) {
        let assign9370_e9291: f64 = (p.p796 * var_temp0);
        let assign9370_e9293: f64 = (assign9370_e9291 / var_kstressvth0);
        (assign9370_e9293,)
    } else {
        (var_temp00,)
    }
};
        var_temp00 = assign9370_e9295;
        var_temp00_rv = 0.0;

        let (assign9380_e9303,) = {
    if ((var_guard36 != 0.0) && (var_guard144 != 0.0)) {
        let assign9380_e9301: f64 = (var_vfb_p + var_temp00);
        (assign9380_e9301,)
    } else {
        (var_vfb_p,)
    }
};
        var_vfb_p = assign9380_e9303;
        var_vfb_p_rv = 0.0;

        let (assign9390_e9311,) = {
    if ((var_guard36 != 0.0) && (var_guard144 != 0.0)) {
        let assign9390_e9309: f64 = (var_vfbedge_p + var_temp00);
        (assign9390_e9309,)
    } else {
        (var_vfbedge_p,)
    }
};
        var_vfbedge_p = assign9390_e9311;
        var_vfbedge_p_rv = 0.0;

        let (assign9400_e9323,) = {
    if ((var_guard36 != 0.0) && (var_guard144 != 0.0)) {
        let assign9400_e9317: f64 = (p.p802 * var_temp0);
        let assign9400_e9320: f64 = (var_kstressvth0).powf(p.p803);
        let assign9400_e9321: f64 = (assign9400_e9317 / assign9400_e9320);
        (assign9400_e9321,)
    } else {
        (var_temp00,)
    }
};
        var_temp00 = assign9400_e9323;
        var_temp00_rv = 0.0;

        let (assign9410_e9331,) = {
    if ((var_guard36 != 0.0) && (var_guard144 != 0.0)) {
        let assign9410_e9329: f64 = (var_cf_p + var_temp00);
        (assign9410_e9329,)
    } else {
        (var_cf_p,)
    }
};
        var_cf_p = assign9410_e9331;
        var_cf_p_rv = 0.0;

        let (assign9420_e9339,) = {
    if ((var_guard36 != 0.0) && (var_guard144 != 0.0)) {
        let assign9420_e9337: f64 = (var_cfedge_p + var_temp00);
        (assign9420_e9337,)
    } else {
        (var_cfedge_p,)
    }
};
        var_cfedge_p = assign9420_e9339;
        var_cfedge_p_rv = 0.0;

        let assign9430_e9354: f64 = if ((((var_sca_i > 0.0) || (var_scb_i > 0.0)) || (var_scc_i > 0.0)) || (var_sc_i > 0.0)) { 1.0 } else { 0.0 };
        var_guard145 = assign9430_e9354;
        var_guard145_rv = 0.0;

        let assign9440_e9365: f64 = if (((var_sca_i == 0.0) && (var_scb_i == 0.0)) && (var_scc_i == 0.0)) { 1.0 } else { 0.0 };
        var_guard146 = assign9440_e9365;
        var_guard146_rv = 0.0;

        let (assign9450_e9375,) = {
    if (((var_guard36 != 0.0) && (var_guard145 != 0.0)) && (var_guard146 != 0.0)) {
        let assign9450_e9373: f64 = (var_sc_i + var_w_i);
        (assign9450_e9373,)
    } else {
        (var_temp0,)
    }
};
        var_temp0 = assign9450_e9375;
        var_temp0_rv = 0.0;

        let (assign9460_e9385,) = {
    if (((var_guard36 != 0.0) && (var_guard145 != 0.0)) && (var_guard146 != 0.0)) {
        let assign9460_e9383: f64 = (1.0 / p.p804);
        (assign9460_e9383,)
    } else {
        (var_temp00,)
    }
};
        var_temp00 = assign9460_e9385;
        var_temp00_rv = 0.0;

        let (assign9470_e9399,) = {
    if (((var_guard36 != 0.0) && (var_guard145 != 0.0)) && (var_guard146 != 0.0)) {
        let assign9470_e9393: f64 = (p.p804 * p.p804);
        let assign9470_e9396: f64 = (var_sc_i * var_temp0);
        let assign9470_e9397: f64 = (assign9470_e9393 / assign9470_e9396);
        (assign9470_e9397,)
    } else {
        (var_sca_i,)
    }
};
        var_sca_i = assign9470_e9399;
        var_sca_i_rv = 0.0;

        let (assign9480_e9439,) = {
    if (((var_guard36 != 0.0) && (var_guard145 != 0.0)) && (var_guard146 != 0.0)) {
        let assign9480_e9407: f64 = (0.1 * var_sc_i);
        let assign9480_e9410: f64 = (0.01 * p.p804);
        let assign9480_e9411: f64 = (assign9480_e9407 + assign9480_e9410);
        let assign9480_e9413: f64 = (-10.0);
        let assign9480_e9415: f64 = (assign9480_e9413 * var_sc_i);
        let assign9480_e9417: f64 = (assign9480_e9415 * var_temp00);
        let assign9480_e9418: f64 = (assign9480_e9417).exp();
        let assign9480_e9419: f64 = (assign9480_e9411 * assign9480_e9418);
        let assign9480_e9422: f64 = (0.1 * var_temp0);
        let assign9480_e9425: f64 = (0.01 * p.p804);
        let assign9480_e9426: f64 = (assign9480_e9422 + assign9480_e9425);
        let assign9480_e9428: f64 = (-10.0);
        let assign9480_e9430: f64 = (assign9480_e9428 * var_temp0);
        let assign9480_e9432: f64 = (assign9480_e9430 * var_temp00);
        let assign9480_e9433: f64 = (assign9480_e9432).exp();
        let assign9480_e9434: f64 = (assign9480_e9426 * assign9480_e9433);
        let assign9480_e9435: f64 = (assign9480_e9419 - assign9480_e9434);
        let assign9480_e9437: f64 = (assign9480_e9435 / var_w_i);
        (assign9480_e9437,)
    } else {
        (var_scb_i,)
    }
};
        var_scb_i = assign9480_e9439;
        var_scb_i_rv = 0.0;

        *var_betn_p_slot = var_betn_p;
        *var_betn_p_rv_slot = var_betn_p_rv;
        *var_betnedge_p_slot = var_betnedge_p;
        *var_betnedge_p_rv_slot = var_betnedge_p_rv;
        *var_cf_p_slot = var_cf_p;
        *var_cf_p_rv_slot = var_cf_p_rv;
        *var_cfedge_p_slot = var_cfedge_p;
        *var_cfedge_p_rv_slot = var_cfedge_p_rv;
        *var_guard143_slot = var_guard143;
        *var_guard143_rv_slot = var_guard143_rv;
        *var_guard144_slot = var_guard144;
        *var_guard144_rv_slot = var_guard144_rv;
        *var_guard145_slot = var_guard145;
        *var_guard145_rv_slot = var_guard145_rv;
        *var_guard146_slot = var_guard146;
        *var_guard146_rv_slot = var_guard146_rv;
        *var_invsa_slot = var_invsa;
        *var_invsa_rv_slot = var_invsa_rv;
        *var_invsaref_slot = var_invsaref;
        *var_invsaref_rv_slot = var_invsaref_rv;
        *var_invsb_slot = var_invsb;
        *var_invsb_rv_slot = var_invsb_rv;
        *var_invsbref_slot = var_invsbref;
        *var_invsbref_rv_slot = var_invsbref_rv;
        *var_kstressu0_slot = var_kstressu0;
        *var_kstressu0_rv_slot = var_kstressu0_rv;
        *var_kstressvth0_slot = var_kstressvth0;
        *var_kstressvth0_rv_slot = var_kstressvth0_rv;
        *var_kvsatac_i_slot = var_kvsatac_i;
        *var_kvsatac_i_rv_slot = var_kvsatac_i_rv;
        *var_loop__slot = var_loop_;
        *var_loop__rv_slot = var_loop__rv;
        *var_lx_slot = var_lx;
        *var_lx_rv_slot = var_lx_rv;
        *var_rhobeta_slot = var_rhobeta;
        *var_rhobeta_rv_slot = var_rhobeta_rv;
        *var_rhobetaref_slot = var_rhobetaref;
        *var_rhobetaref_rv_slot = var_rhobetaref_rv;
        *var_sca_i_slot = var_sca_i;
        *var_sca_i_rv_slot = var_sca_i_rv;
        *var_scb_i_slot = var_scb_i;
        *var_scb_i_rv_slot = var_scb_i_rv;
        *var_temp0_slot = var_temp0;
        *var_temp00_slot = var_temp00;
        *var_temp00_rv_slot = var_temp00_rv;
        *var_temp0_rv_slot = var_temp0_rv;
        *var_templ_slot = var_templ;
        *var_templ_rv_slot = var_templ_rv;
        *var_tempw_slot = var_tempw;
        *var_tempw_rv_slot = var_tempw_rv;
        *var_thesat_p_slot = var_thesat_p;
        *var_thesat_p_rv_slot = var_thesat_p_rv;
        *var_thesatac_p_slot = var_thesatac_p;
        *var_thesatac_p_rv_slot = var_thesatac_p_rv;
        *var_tmpa_slot = var_tmpa;
        *var_tmpa_rv_slot = var_tmpa_rv;
        *var_tmpb_slot = var_tmpb;
        *var_tmpb_rv_slot = var_tmpb_rv;
        *var_vfb_p_slot = var_vfb_p;
        *var_vfb_p_rv_slot = var_vfb_p_rv;
        *var_vfbedge_p_slot = var_vfbedge_p;
        *var_vfbedge_p_rv_slot = var_vfbedge_p_rv;
        *var_wx_slot = var_wx;
        *var_wx_rv_slot = var_wx_rv;
    }

    pub(super) fn stamp_reactive_block_15(
        p: &Parameters,
        var_ax_p: f64,
        var_cf_p: f64,
        var_cfb_p: f64,
        var_cfd_p: f64,
        var_cs_p: f64,
        var_ct_p: f64,
        var_ctb_p: f64,
        var_ctg_p: f64,
        var_dphib_p: f64,
        var_dvsbnud_p: f64,
        var_epsrox_p: f64,
        var_feta_p: f64,
        var_gfacnud_p: f64,
        var_guard145: f64,
        var_guard146: f64,
        var_guard36: f64,
        var_kuowe: f64,
        var_kvthowe: f64,
        var_mue_p: f64,
        var_neff_p: f64,
        var_nov_p: f64,
        var_novd_p: f64,
        var_np_p: f64,
        var_psce_p: f64,
        var_psceb_p: f64,
        var_psced_p: f64,
        var_rs_p: f64,
        var_rsb_p: f64,
        var_rsg_p: f64,
        var_sc_i: f64,
        var_sca_i: f64,
        var_scb_i: f64,
        var_st2vfb_p: f64,
        var_stbet_p: f64,
        var_stcs_p: f64,
        var_stct_p: f64,
        var_stmue_p: f64,
        var_strs_p: f64,
        var_stthecs_p: f64,
        var_stthemu_p: f64,
        var_stthesat_p: f64,
        var_stvfb_p: f64,
        var_stxcor_p: f64,
        var_temp00: f64,
        var_thecs_p: f64,
        var_themu_p: f64,
        var_thesat_p: f64,
        var_thesatb_p: f64,
        var_thesatg_p: f64,
        var_thesatt_p: f64,
        var_tox_p: f64,
        var_toxov_p: f64,
        var_toxovd_p: f64,
        var_vsbnud_p: f64,
        var_w_i: f64,
        var_xcor_p: f64,
        var_ax_i_slot: &mut f64,
        var_ax_i_rv_slot: &mut f64,
        var_betn_i_slot: &mut f64,
        var_betn_i_rv_slot: &mut f64,
        var_betn_p_slot: &mut f64,
        var_betn_p_rv_slot: &mut f64,
        var_betnedge_p_slot: &mut f64,
        var_betnedge_p_rv_slot: &mut f64,
        var_cf_i_slot: &mut f64,
        var_cf_i_rv_slot: &mut f64,
        var_cfb_i_slot: &mut f64,
        var_cfb_i_rv_slot: &mut f64,
        var_cfd_i_slot: &mut f64,
        var_cfd_i_rv_slot: &mut f64,
        var_cs_i_slot: &mut f64,
        var_cs_i_rv_slot: &mut f64,
        var_ct_i_slot: &mut f64,
        var_ct_i_rv_slot: &mut f64,
        var_ctb_i_slot: &mut f64,
        var_ctb_i_rv_slot: &mut f64,
        var_ctg_i_slot: &mut f64,
        var_ctg_i_rv_slot: &mut f64,
        var_dphib_i_slot: &mut f64,
        var_dphib_i_rv_slot: &mut f64,
        var_dvsbnud_i_slot: &mut f64,
        var_dvsbnud_i_rv_slot: &mut f64,
        var_epsrox_i_slot: &mut f64,
        var_epsrox_i_rv_slot: &mut f64,
        var_feta_i_slot: &mut f64,
        var_feta_i_rv_slot: &mut f64,
        var_gfacnud_i_slot: &mut f64,
        var_gfacnud_i_rv_slot: &mut f64,
        var_mue_i_slot: &mut f64,
        var_mue_i_rv_slot: &mut f64,
        var_neff_i_slot: &mut f64,
        var_neff_i_rv_slot: &mut f64,
        var_nov_i_slot: &mut f64,
        var_nov_i_rv_slot: &mut f64,
        var_novd_i_slot: &mut f64,
        var_novd_i_rv_slot: &mut f64,
        var_np_i_slot: &mut f64,
        var_np_i_rv_slot: &mut f64,
        var_psce_i_slot: &mut f64,
        var_psce_i_rv_slot: &mut f64,
        var_psceb_i_slot: &mut f64,
        var_psceb_i_rv_slot: &mut f64,
        var_psced_i_slot: &mut f64,
        var_psced_i_rv_slot: &mut f64,
        var_rs_i_slot: &mut f64,
        var_rs_i_rv_slot: &mut f64,
        var_rsb_i_slot: &mut f64,
        var_rsb_i_rv_slot: &mut f64,
        var_rsg_i_slot: &mut f64,
        var_rsg_i_rv_slot: &mut f64,
        var_scc_i_slot: &mut f64,
        var_scc_i_rv_slot: &mut f64,
        var_st2vfb_i_slot: &mut f64,
        var_st2vfb_i_rv_slot: &mut f64,
        var_stbet_i_slot: &mut f64,
        var_stbet_i_rv_slot: &mut f64,
        var_stcs_i_slot: &mut f64,
        var_stcs_i_rv_slot: &mut f64,
        var_stct_i_slot: &mut f64,
        var_stct_i_rv_slot: &mut f64,
        var_stmue_i_slot: &mut f64,
        var_stmue_i_rv_slot: &mut f64,
        var_strs_i_slot: &mut f64,
        var_strs_i_rv_slot: &mut f64,
        var_stthecs_i_slot: &mut f64,
        var_stthecs_i_rv_slot: &mut f64,
        var_stthemu_i_slot: &mut f64,
        var_stthemu_i_rv_slot: &mut f64,
        var_stthesat_i_slot: &mut f64,
        var_stthesat_i_rv_slot: &mut f64,
        var_stvfb_i_slot: &mut f64,
        var_stvfb_i_rv_slot: &mut f64,
        var_stxcor_i_slot: &mut f64,
        var_stxcor_i_rv_slot: &mut f64,
        var_temp0_slot: &mut f64,
        var_temp0_rv_slot: &mut f64,
        var_thecs_i_slot: &mut f64,
        var_thecs_i_rv_slot: &mut f64,
        var_themu_i_slot: &mut f64,
        var_themu_i_rv_slot: &mut f64,
        var_thesat_i_slot: &mut f64,
        var_thesat_i_rv_slot: &mut f64,
        var_thesatb_i_slot: &mut f64,
        var_thesatb_i_rv_slot: &mut f64,
        var_thesatg_i_slot: &mut f64,
        var_thesatg_i_rv_slot: &mut f64,
        var_thesatt_i_slot: &mut f64,
        var_thesatt_i_rv_slot: &mut f64,
        var_tox_i_slot: &mut f64,
        var_tox_i_rv_slot: &mut f64,
        var_toxov_i_slot: &mut f64,
        var_toxov_i_rv_slot: &mut f64,
        var_toxovd_i_slot: &mut f64,
        var_toxovd_i_rv_slot: &mut f64,
        var_vfb_i_slot: &mut f64,
        var_vfb_i_rv_slot: &mut f64,
        var_vfb_p_slot: &mut f64,
        var_vfb_p_rv_slot: &mut f64,
        var_vfbedge_p_slot: &mut f64,
        var_vfbedge_p_rv_slot: &mut f64,
        var_vsbnud_i_slot: &mut f64,
        var_vsbnud_i_rv_slot: &mut f64,
        var_xcor_i_slot: &mut f64,
        var_xcor_i_rv_slot: &mut f64,
    ) {
        let mut var_ax_i: f64 = *var_ax_i_slot;
        let mut var_ax_i_rv: f64 = *var_ax_i_rv_slot;
        let mut var_betn_i: f64 = *var_betn_i_slot;
        let mut var_betn_i_rv: f64 = *var_betn_i_rv_slot;
        let mut var_betn_p: f64 = *var_betn_p_slot;
        let mut var_betn_p_rv: f64 = *var_betn_p_rv_slot;
        let mut var_betnedge_p: f64 = *var_betnedge_p_slot;
        let mut var_betnedge_p_rv: f64 = *var_betnedge_p_rv_slot;
        let mut var_cf_i: f64 = *var_cf_i_slot;
        let mut var_cf_i_rv: f64 = *var_cf_i_rv_slot;
        let mut var_cfb_i: f64 = *var_cfb_i_slot;
        let mut var_cfb_i_rv: f64 = *var_cfb_i_rv_slot;
        let mut var_cfd_i: f64 = *var_cfd_i_slot;
        let mut var_cfd_i_rv: f64 = *var_cfd_i_rv_slot;
        let mut var_cs_i: f64 = *var_cs_i_slot;
        let mut var_cs_i_rv: f64 = *var_cs_i_rv_slot;
        let mut var_ct_i: f64 = *var_ct_i_slot;
        let mut var_ct_i_rv: f64 = *var_ct_i_rv_slot;
        let mut var_ctb_i: f64 = *var_ctb_i_slot;
        let mut var_ctb_i_rv: f64 = *var_ctb_i_rv_slot;
        let mut var_ctg_i: f64 = *var_ctg_i_slot;
        let mut var_ctg_i_rv: f64 = *var_ctg_i_rv_slot;
        let mut var_dphib_i: f64 = *var_dphib_i_slot;
        let mut var_dphib_i_rv: f64 = *var_dphib_i_rv_slot;
        let mut var_dvsbnud_i: f64 = *var_dvsbnud_i_slot;
        let mut var_dvsbnud_i_rv: f64 = *var_dvsbnud_i_rv_slot;
        let mut var_epsrox_i: f64 = *var_epsrox_i_slot;
        let mut var_epsrox_i_rv: f64 = *var_epsrox_i_rv_slot;
        let mut var_feta_i: f64 = *var_feta_i_slot;
        let mut var_feta_i_rv: f64 = *var_feta_i_rv_slot;
        let mut var_gfacnud_i: f64 = *var_gfacnud_i_slot;
        let mut var_gfacnud_i_rv: f64 = *var_gfacnud_i_rv_slot;
        let mut var_mue_i: f64 = *var_mue_i_slot;
        let mut var_mue_i_rv: f64 = *var_mue_i_rv_slot;
        let mut var_neff_i: f64 = *var_neff_i_slot;
        let mut var_neff_i_rv: f64 = *var_neff_i_rv_slot;
        let mut var_nov_i: f64 = *var_nov_i_slot;
        let mut var_nov_i_rv: f64 = *var_nov_i_rv_slot;
        let mut var_novd_i: f64 = *var_novd_i_slot;
        let mut var_novd_i_rv: f64 = *var_novd_i_rv_slot;
        let mut var_np_i: f64 = *var_np_i_slot;
        let mut var_np_i_rv: f64 = *var_np_i_rv_slot;
        let mut var_psce_i: f64 = *var_psce_i_slot;
        let mut var_psce_i_rv: f64 = *var_psce_i_rv_slot;
        let mut var_psceb_i: f64 = *var_psceb_i_slot;
        let mut var_psceb_i_rv: f64 = *var_psceb_i_rv_slot;
        let mut var_psced_i: f64 = *var_psced_i_slot;
        let mut var_psced_i_rv: f64 = *var_psced_i_rv_slot;
        let mut var_rs_i: f64 = *var_rs_i_slot;
        let mut var_rs_i_rv: f64 = *var_rs_i_rv_slot;
        let mut var_rsb_i: f64 = *var_rsb_i_slot;
        let mut var_rsb_i_rv: f64 = *var_rsb_i_rv_slot;
        let mut var_rsg_i: f64 = *var_rsg_i_slot;
        let mut var_rsg_i_rv: f64 = *var_rsg_i_rv_slot;
        let mut var_scc_i: f64 = *var_scc_i_slot;
        let mut var_scc_i_rv: f64 = *var_scc_i_rv_slot;
        let mut var_st2vfb_i: f64 = *var_st2vfb_i_slot;
        let mut var_st2vfb_i_rv: f64 = *var_st2vfb_i_rv_slot;
        let mut var_stbet_i: f64 = *var_stbet_i_slot;
        let mut var_stbet_i_rv: f64 = *var_stbet_i_rv_slot;
        let mut var_stcs_i: f64 = *var_stcs_i_slot;
        let mut var_stcs_i_rv: f64 = *var_stcs_i_rv_slot;
        let mut var_stct_i: f64 = *var_stct_i_slot;
        let mut var_stct_i_rv: f64 = *var_stct_i_rv_slot;
        let mut var_stmue_i: f64 = *var_stmue_i_slot;
        let mut var_stmue_i_rv: f64 = *var_stmue_i_rv_slot;
        let mut var_strs_i: f64 = *var_strs_i_slot;
        let mut var_strs_i_rv: f64 = *var_strs_i_rv_slot;
        let mut var_stthecs_i: f64 = *var_stthecs_i_slot;
        let mut var_stthecs_i_rv: f64 = *var_stthecs_i_rv_slot;
        let mut var_stthemu_i: f64 = *var_stthemu_i_slot;
        let mut var_stthemu_i_rv: f64 = *var_stthemu_i_rv_slot;
        let mut var_stthesat_i: f64 = *var_stthesat_i_slot;
        let mut var_stthesat_i_rv: f64 = *var_stthesat_i_rv_slot;
        let mut var_stvfb_i: f64 = *var_stvfb_i_slot;
        let mut var_stvfb_i_rv: f64 = *var_stvfb_i_rv_slot;
        let mut var_stxcor_i: f64 = *var_stxcor_i_slot;
        let mut var_stxcor_i_rv: f64 = *var_stxcor_i_rv_slot;
        let mut var_temp0: f64 = *var_temp0_slot;
        let mut var_temp0_rv: f64 = *var_temp0_rv_slot;
        let mut var_thecs_i: f64 = *var_thecs_i_slot;
        let mut var_thecs_i_rv: f64 = *var_thecs_i_rv_slot;
        let mut var_themu_i: f64 = *var_themu_i_slot;
        let mut var_themu_i_rv: f64 = *var_themu_i_rv_slot;
        let mut var_thesat_i: f64 = *var_thesat_i_slot;
        let mut var_thesat_i_rv: f64 = *var_thesat_i_rv_slot;
        let mut var_thesatb_i: f64 = *var_thesatb_i_slot;
        let mut var_thesatb_i_rv: f64 = *var_thesatb_i_rv_slot;
        let mut var_thesatg_i: f64 = *var_thesatg_i_slot;
        let mut var_thesatg_i_rv: f64 = *var_thesatg_i_rv_slot;
        let mut var_thesatt_i: f64 = *var_thesatt_i_slot;
        let mut var_thesatt_i_rv: f64 = *var_thesatt_i_rv_slot;
        let mut var_tox_i: f64 = *var_tox_i_slot;
        let mut var_tox_i_rv: f64 = *var_tox_i_rv_slot;
        let mut var_toxov_i: f64 = *var_toxov_i_slot;
        let mut var_toxov_i_rv: f64 = *var_toxov_i_rv_slot;
        let mut var_toxovd_i: f64 = *var_toxovd_i_slot;
        let mut var_toxovd_i_rv: f64 = *var_toxovd_i_rv_slot;
        let mut var_vfb_i: f64 = *var_vfb_i_slot;
        let mut var_vfb_i_rv: f64 = *var_vfb_i_rv_slot;
        let mut var_vfb_p: f64 = *var_vfb_p_slot;
        let mut var_vfb_p_rv: f64 = *var_vfb_p_rv_slot;
        let mut var_vfbedge_p: f64 = *var_vfbedge_p_slot;
        let mut var_vfbedge_p_rv: f64 = *var_vfbedge_p_rv_slot;
        let mut var_vsbnud_i: f64 = *var_vsbnud_i_slot;
        let mut var_vsbnud_i_rv: f64 = *var_vsbnud_i_rv_slot;
        let mut var_xcor_i: f64 = *var_xcor_i_slot;
        let mut var_xcor_i_rv: f64 = *var_xcor_i_rv_slot;

        let (assign9490_e9479,) = {
    if (((var_guard36 != 0.0) && (var_guard145 != 0.0)) && (var_guard146 != 0.0)) {
        let assign9490_e9447: f64 = (0.05 * var_sc_i);
        let assign9490_e9450: f64 = (0.0025 * p.p804);
        let assign9490_e9451: f64 = (assign9490_e9447 + assign9490_e9450);
        let assign9490_e9453: f64 = (-20.0);
        let assign9490_e9455: f64 = (assign9490_e9453 * var_sc_i);
        let assign9490_e9457: f64 = (assign9490_e9455 * var_temp00);
        let assign9490_e9458: f64 = (assign9490_e9457).exp();
        let assign9490_e9459: f64 = (assign9490_e9451 * assign9490_e9458);
        let assign9490_e9462: f64 = (0.05 * var_temp0);
        let assign9490_e9465: f64 = (0.0025 * p.p804);
        let assign9490_e9466: f64 = (assign9490_e9462 + assign9490_e9465);
        let assign9490_e9468: f64 = (-20.0);
        let assign9490_e9470: f64 = (assign9490_e9468 * var_temp0);
        let assign9490_e9472: f64 = (assign9490_e9470 * var_temp00);
        let assign9490_e9473: f64 = (assign9490_e9472).exp();
        let assign9490_e9474: f64 = (assign9490_e9466 * assign9490_e9473);
        let assign9490_e9475: f64 = (assign9490_e9459 - assign9490_e9474);
        let assign9490_e9477: f64 = (assign9490_e9475 / var_w_i);
        (assign9490_e9477,)
    } else {
        (var_scc_i,)
    }
};
        var_scc_i = assign9490_e9479;
        var_scc_i_rv = 0.0;

        let (assign9500_e9493,) = {
    if ((var_guard36 != 0.0) && (var_guard145 != 0.0)) {
        let assign9500_e9486: f64 = (p.p805 * var_scb_i);
        let assign9500_e9487: f64 = (var_sca_i + assign9500_e9486);
        let assign9500_e9490: f64 = (p.p806 * var_scc_i);
        let assign9500_e9491: f64 = (assign9500_e9487 + assign9500_e9490);
        (assign9500_e9491,)
    } else {
        (var_temp0,)
    }
};
        var_temp0 = assign9500_e9493;
        var_temp0_rv = 0.0;

        let (assign9510_e9503,) = {
    if ((var_guard36 != 0.0) && (var_guard145 != 0.0)) {
        let assign9510_e9500: f64 = (var_kvthowe * var_temp0);
        let assign9510_e9501: f64 = (var_vfb_p + assign9510_e9500);
        (assign9510_e9501,)
    } else {
        (var_vfb_p,)
    }
};
        var_vfb_p = assign9510_e9503;
        var_vfb_p_rv = 0.0;

        let (assign9520_e9515,) = {
    if ((var_guard36 != 0.0) && (var_guard145 != 0.0)) {
        let assign9520_e9511: f64 = (var_kuowe * var_temp0);
        let assign9520_e9512: f64 = (1.0 + assign9520_e9511);
        let assign9520_e9513: f64 = (var_betn_p * assign9520_e9512);
        (assign9520_e9513,)
    } else {
        (var_betn_p,)
    }
};
        var_betn_p = assign9520_e9515;
        var_betn_p_rv = 0.0;

        let (assign9530_e9525,) = {
    if ((var_guard36 != 0.0) && (var_guard145 != 0.0)) {
        let assign9530_e9522: f64 = (var_kvthowe * var_temp0);
        let assign9530_e9523: f64 = (var_vfbedge_p + assign9530_e9522);
        (assign9530_e9523,)
    } else {
        (var_vfbedge_p,)
    }
};
        var_vfbedge_p = assign9530_e9525;
        var_vfbedge_p_rv = 0.0;

        let (assign9540_e9537,) = {
    if ((var_guard36 != 0.0) && (var_guard145 != 0.0)) {
        let assign9540_e9533: f64 = (var_kuowe * var_temp0);
        let assign9540_e9534: f64 = (1.0 + assign9540_e9533);
        let assign9540_e9535: f64 = (var_betnedge_p * assign9540_e9534);
        (assign9540_e9535,)
    } else {
        (var_betnedge_p,)
    }
};
        var_betnedge_p = assign9540_e9537;
        var_betnedge_p_rv = 0.0;

        var_vfb_i = var_vfb_p;
        var_vfb_i_rv = 0.0;

        var_stvfb_i = var_stvfb_p;
        var_stvfb_i_rv = 0.0;

        var_st2vfb_i = var_st2vfb_p;
        var_st2vfb_i_rv = 0.0;

        var_tox_i = var_tox_p;
        var_tox_i_rv = 0.0;

        var_epsrox_i = var_epsrox_p;
        var_epsrox_i_rv = 0.0;

        let (assign9600_e9553,) = {
    if (var_neff_p > 1e20) {
        let (assign9600_e9551,) = {
            if (var_neff_p < 1e26) {
                (var_neff_p,)
            } else {
                (1e26,)
            }
        };
        (assign9600_e9551,)
    } else {
        (1e20,)
    }
};
        var_neff_i = assign9600_e9553;
        var_neff_i_rv = 0.0;

        let (assign9610_e9559,) = {
    if (var_gfacnud_p > 0.01) {
        (var_gfacnud_p,)
    } else {
        (0.01,)
    }
};
        var_gfacnud_i = assign9610_e9559;
        var_gfacnud_i_rv = 0.0;

        let (assign9620_e9565,) = {
    if (var_vsbnud_p > 0.0) {
        (var_vsbnud_p,)
    } else {
        (0.0,)
    }
};
        var_vsbnud_i = assign9620_e9565;
        var_vsbnud_i_rv = 0.0;

        var_dvsbnud_i = var_dvsbnud_p;
        var_dvsbnud_i_rv = 0.0;

        var_dphib_i = var_dphib_p;
        var_dphib_i_rv = 0.0;

        let (assign9650_e9573,) = {
    if (var_np_p > 0.0) {
        (var_np_p,)
    } else {
        (0.0,)
    }
};
        var_np_i = assign9650_e9573;
        var_np_i_rv = 0.0;

        var_toxov_i = var_toxov_p;
        var_toxov_i_rv = 0.0;

        var_toxovd_i = var_toxovd_p;
        var_toxovd_i_rv = 0.0;

        let (assign9680_e9586,) = {
    if (var_nov_p > 1e23) {
        let (assign9680_e9584,) = {
            if (var_nov_p < 1e27) {
                (var_nov_p,)
            } else {
                (1e27,)
            }
        };
        (assign9680_e9584,)
    } else {
        (1e23,)
    }
};
        var_nov_i = assign9680_e9586;
        var_nov_i_rv = 0.0;

        let (assign9690_e9597,) = {
    if (var_novd_p > 1e23) {
        let (assign9690_e9595,) = {
            if (var_novd_p < 1e27) {
                (var_novd_p,)
            } else {
                (1e27,)
            }
        };
        (assign9690_e9595,)
    } else {
        (1e23,)
    }
};
        var_novd_i = assign9690_e9597;
        var_novd_i_rv = 0.0;

        let (assign9700_e9603,) = {
    if (var_ct_p > 0.0) {
        (var_ct_p,)
    } else {
        (0.0,)
    }
};
        var_ct_i = assign9700_e9603;
        var_ct_i_rv = 0.0;

        let (assign9710_e9614,) = {
    if (var_ctb_p > 0.0) {
        let (assign9710_e9612,) = {
            if (var_ctb_p < 0.5) {
                (var_ctb_p,)
            } else {
                (0.5,)
            }
        };
        (assign9710_e9612,)
    } else {
        (0.0,)
    }
};
        var_ctb_i = assign9710_e9614;
        var_ctb_i_rv = 0.0;

        let (assign9720_e9625,) = {
    if (var_ctg_p > 0.0) {
        let (assign9720_e9623,) = {
            if (var_ctg_p < 1.0) {
                (var_ctg_p,)
            } else {
                (1.0,)
            }
        };
        (assign9720_e9623,)
    } else {
        (0.0,)
    }
};
        var_ctg_i = assign9720_e9625;
        var_ctg_i_rv = 0.0;

        var_stct_i = var_stct_p;
        var_stct_i_rv = 0.0;

        let (assign9740_e9632,) = {
    if (var_cf_p > 0.0) {
        (var_cf_p,)
    } else {
        (0.0,)
    }
};
        var_cf_i = assign9740_e9632;
        var_cf_i_rv = 0.0;

        let (assign9750_e9643,) = {
    if (var_cfb_p > 0.0) {
        let (assign9750_e9641,) = {
            if (var_cfb_p < 1.0) {
                (var_cfb_p,)
            } else {
                (1.0,)
            }
        };
        (assign9750_e9641,)
    } else {
        (0.0,)
    }
};
        var_cfb_i = assign9750_e9643;
        var_cfb_i_rv = 0.0;

        let (assign9760_e9649,) = {
    if (var_cfd_p > 0.0) {
        (var_cfd_p,)
    } else {
        (0.0,)
    }
};
        var_cfd_i = assign9760_e9649;
        var_cfd_i_rv = 0.0;

        let (assign9770_e9655,) = {
    if (var_psce_p > 0.0) {
        (var_psce_p,)
    } else {
        (0.0,)
    }
};
        var_psce_i = assign9770_e9655;
        var_psce_i_rv = 0.0;

        let (assign9780_e9666,) = {
    if (var_psceb_p > 0.0) {
        let (assign9780_e9664,) = {
            if (var_psceb_p < 1.0) {
                (var_psceb_p,)
            } else {
                (1.0,)
            }
        };
        (assign9780_e9664,)
    } else {
        (0.0,)
    }
};
        var_psceb_i = assign9780_e9666;
        var_psceb_i_rv = 0.0;

        let (assign9790_e9672,) = {
    if (var_psced_p > 0.0) {
        (var_psced_p,)
    } else {
        (0.0,)
    }
};
        var_psced_i = assign9790_e9672;
        var_psced_i_rv = 0.0;

        let (assign9800_e9678,) = {
    if (var_betn_p > 0.0) {
        (var_betn_p,)
    } else {
        (0.0,)
    }
};
        var_betn_i = assign9800_e9678;
        var_betn_i_rv = 0.0;

        var_stbet_i = var_stbet_p;
        var_stbet_i_rv = 0.0;

        let (assign9820_e9685,) = {
    if (var_mue_p > 0.0) {
        (var_mue_p,)
    } else {
        (0.0,)
    }
};
        var_mue_i = assign9820_e9685;
        var_mue_i_rv = 0.0;

        var_stmue_i = var_stmue_p;
        var_stmue_i_rv = 0.0;

        let (assign9840_e9692,) = {
    if (var_themu_p > 0.0) {
        (var_themu_p,)
    } else {
        (0.0,)
    }
};
        var_themu_i = assign9840_e9692;
        var_themu_i_rv = 0.0;

        var_stthemu_i = var_stthemu_p;
        var_stthemu_i_rv = 0.0;

        let (assign9860_e9699,) = {
    if (var_cs_p > 0.0) {
        (var_cs_p,)
    } else {
        (0.0,)
    }
};
        var_cs_i = assign9860_e9699;
        var_cs_i_rv = 0.0;

        var_stcs_i = var_stcs_p;
        var_stcs_i_rv = 0.0;

        let (assign9880_e9706,) = {
    if (var_thecs_p > 0.0) {
        (var_thecs_p,)
    } else {
        (0.0,)
    }
};
        var_thecs_i = assign9880_e9706;
        var_thecs_i_rv = 0.0;

        var_stthecs_i = var_stthecs_p;
        var_stthecs_i_rv = 0.0;

        let (assign9900_e9713,) = {
    if (var_xcor_p > 0.0) {
        (var_xcor_p,)
    } else {
        (0.0,)
    }
};
        var_xcor_i = assign9900_e9713;
        var_xcor_i_rv = 0.0;

        var_stxcor_i = var_stxcor_p;
        var_stxcor_i_rv = 0.0;

        var_feta_i = var_feta_p;
        var_feta_i_rv = 0.0;

        let (assign9930_e9721,) = {
    if (var_rs_p > 0.0) {
        (var_rs_p,)
    } else {
        (0.0,)
    }
};
        var_rs_i = assign9930_e9721;
        var_rs_i_rv = 0.0;

        var_strs_i = var_strs_p;
        var_strs_i_rv = 0.0;

        let assign9950_e9725: f64 = (-0.5);
        let (assign9950_e9735,) = {
    if (var_rsb_p > assign9950_e9725) {
        let (assign9950_e9732,) = {
            if (var_rsb_p < 1.0) {
                (var_rsb_p,)
            } else {
                (1.0,)
            }
        };
        (assign9950_e9732,)
    } else {
        let assign9950_e9734: f64 = (-0.5);
        (assign9950_e9734,)
    }
};
        var_rsb_i = assign9950_e9735;
        var_rsb_i_rv = 0.0;

        let assign9960_e9738: f64 = (-0.5);
        let (assign9960_e9743,) = {
    if (var_rsg_p > assign9960_e9738) {
        (var_rsg_p,)
    } else {
        let assign9960_e9742: f64 = (-0.5);
        (assign9960_e9742,)
    }
};
        var_rsg_i = assign9960_e9743;
        var_rsg_i_rv = 0.0;

        let (assign9970_e9749,) = {
    if (var_thesat_p > 0.0) {
        (var_thesat_p,)
    } else {
        (0.0,)
    }
};
        var_thesat_i = assign9970_e9749;
        var_thesat_i_rv = 0.0;

        var_stthesat_i = var_stthesat_p;
        var_stthesat_i_rv = 0.0;

        let assign9990_e9753: f64 = (-0.5);
        let (assign9990_e9763,) = {
    if (var_thesatb_p > assign9990_e9753) {
        let (assign9990_e9760,) = {
            if (var_thesatb_p < 1.0) {
                (var_thesatb_p,)
            } else {
                (1.0,)
            }
        };
        (assign9990_e9760,)
    } else {
        let assign9990_e9762: f64 = (-0.5);
        (assign9990_e9762,)
    }
};
        var_thesatb_i = assign9990_e9763;
        var_thesatb_i_rv = 0.0;

        let assign10000_e9766: f64 = (-0.5);
        let (assign10000_e9771,) = {
    if (var_thesatg_p > assign10000_e9766) {
        (var_thesatg_p,)
    } else {
        let assign10000_e9770: f64 = (-0.5);
        (assign10000_e9770,)
    }
};
        var_thesatg_i = assign10000_e9771;
        var_thesatg_i_rv = 0.0;

        let (assign10010_e9777,) = {
    if (var_thesatt_p > 0.01) {
        (var_thesatt_p,)
    } else {
        (0.01,)
    }
};
        var_thesatt_i = assign10010_e9777;
        var_thesatt_i_rv = 0.0;

        let (assign10020_e9783,) = {
    if (var_ax_p > 2.0) {
        (var_ax_p,)
    } else {
        (2.0,)
    }
};
        var_ax_i = assign10020_e9783;
        var_ax_i_rv = 0.0;

        *var_ax_i_slot = var_ax_i;
        *var_ax_i_rv_slot = var_ax_i_rv;
        *var_betn_i_slot = var_betn_i;
        *var_betn_i_rv_slot = var_betn_i_rv;
        *var_betn_p_slot = var_betn_p;
        *var_betn_p_rv_slot = var_betn_p_rv;
        *var_betnedge_p_slot = var_betnedge_p;
        *var_betnedge_p_rv_slot = var_betnedge_p_rv;
        *var_cf_i_slot = var_cf_i;
        *var_cf_i_rv_slot = var_cf_i_rv;
        *var_cfb_i_slot = var_cfb_i;
        *var_cfb_i_rv_slot = var_cfb_i_rv;
        *var_cfd_i_slot = var_cfd_i;
        *var_cfd_i_rv_slot = var_cfd_i_rv;
        *var_cs_i_slot = var_cs_i;
        *var_cs_i_rv_slot = var_cs_i_rv;
        *var_ct_i_slot = var_ct_i;
        *var_ct_i_rv_slot = var_ct_i_rv;
        *var_ctb_i_slot = var_ctb_i;
        *var_ctb_i_rv_slot = var_ctb_i_rv;
        *var_ctg_i_slot = var_ctg_i;
        *var_ctg_i_rv_slot = var_ctg_i_rv;
        *var_dphib_i_slot = var_dphib_i;
        *var_dphib_i_rv_slot = var_dphib_i_rv;
        *var_dvsbnud_i_slot = var_dvsbnud_i;
        *var_dvsbnud_i_rv_slot = var_dvsbnud_i_rv;
        *var_epsrox_i_slot = var_epsrox_i;
        *var_epsrox_i_rv_slot = var_epsrox_i_rv;
        *var_feta_i_slot = var_feta_i;
        *var_feta_i_rv_slot = var_feta_i_rv;
        *var_gfacnud_i_slot = var_gfacnud_i;
        *var_gfacnud_i_rv_slot = var_gfacnud_i_rv;
        *var_mue_i_slot = var_mue_i;
        *var_mue_i_rv_slot = var_mue_i_rv;
        *var_neff_i_slot = var_neff_i;
        *var_neff_i_rv_slot = var_neff_i_rv;
        *var_nov_i_slot = var_nov_i;
        *var_nov_i_rv_slot = var_nov_i_rv;
        *var_novd_i_slot = var_novd_i;
        *var_novd_i_rv_slot = var_novd_i_rv;
        *var_np_i_slot = var_np_i;
        *var_np_i_rv_slot = var_np_i_rv;
        *var_psce_i_slot = var_psce_i;
        *var_psce_i_rv_slot = var_psce_i_rv;
        *var_psceb_i_slot = var_psceb_i;
        *var_psceb_i_rv_slot = var_psceb_i_rv;
        *var_psced_i_slot = var_psced_i;
        *var_psced_i_rv_slot = var_psced_i_rv;
        *var_rs_i_slot = var_rs_i;
        *var_rs_i_rv_slot = var_rs_i_rv;
        *var_rsb_i_slot = var_rsb_i;
        *var_rsb_i_rv_slot = var_rsb_i_rv;
        *var_rsg_i_slot = var_rsg_i;
        *var_rsg_i_rv_slot = var_rsg_i_rv;
        *var_scc_i_slot = var_scc_i;
        *var_scc_i_rv_slot = var_scc_i_rv;
        *var_st2vfb_i_slot = var_st2vfb_i;
        *var_st2vfb_i_rv_slot = var_st2vfb_i_rv;
        *var_stbet_i_slot = var_stbet_i;
        *var_stbet_i_rv_slot = var_stbet_i_rv;
        *var_stcs_i_slot = var_stcs_i;
        *var_stcs_i_rv_slot = var_stcs_i_rv;
        *var_stct_i_slot = var_stct_i;
        *var_stct_i_rv_slot = var_stct_i_rv;
        *var_stmue_i_slot = var_stmue_i;
        *var_stmue_i_rv_slot = var_stmue_i_rv;
        *var_strs_i_slot = var_strs_i;
        *var_strs_i_rv_slot = var_strs_i_rv;
        *var_stthecs_i_slot = var_stthecs_i;
        *var_stthecs_i_rv_slot = var_stthecs_i_rv;
        *var_stthemu_i_slot = var_stthemu_i;
        *var_stthemu_i_rv_slot = var_stthemu_i_rv;
        *var_stthesat_i_slot = var_stthesat_i;
        *var_stthesat_i_rv_slot = var_stthesat_i_rv;
        *var_stvfb_i_slot = var_stvfb_i;
        *var_stvfb_i_rv_slot = var_stvfb_i_rv;
        *var_stxcor_i_slot = var_stxcor_i;
        *var_stxcor_i_rv_slot = var_stxcor_i_rv;
        *var_temp0_slot = var_temp0;
        *var_temp0_rv_slot = var_temp0_rv;
        *var_thecs_i_slot = var_thecs_i;
        *var_thecs_i_rv_slot = var_thecs_i_rv;
        *var_themu_i_slot = var_themu_i;
        *var_themu_i_rv_slot = var_themu_i_rv;
        *var_thesat_i_slot = var_thesat_i;
        *var_thesat_i_rv_slot = var_thesat_i_rv;
        *var_thesatb_i_slot = var_thesatb_i;
        *var_thesatb_i_rv_slot = var_thesatb_i_rv;
        *var_thesatg_i_slot = var_thesatg_i;
        *var_thesatg_i_rv_slot = var_thesatg_i_rv;
        *var_thesatt_i_slot = var_thesatt_i;
        *var_thesatt_i_rv_slot = var_thesatt_i_rv;
        *var_tox_i_slot = var_tox_i;
        *var_tox_i_rv_slot = var_tox_i_rv;
        *var_toxov_i_slot = var_toxov_i;
        *var_toxov_i_rv_slot = var_toxov_i_rv;
        *var_toxovd_i_slot = var_toxovd_i;
        *var_toxovd_i_rv_slot = var_toxovd_i_rv;
        *var_vfb_i_slot = var_vfb_i;
        *var_vfb_i_rv_slot = var_vfb_i_rv;
        *var_vfb_p_slot = var_vfb_p;
        *var_vfb_p_rv_slot = var_vfb_p_rv;
        *var_vfbedge_p_slot = var_vfbedge_p;
        *var_vfbedge_p_rv_slot = var_vfbedge_p_rv;
        *var_vsbnud_i_slot = var_vsbnud_i;
        *var_vsbnud_i_rv_slot = var_vsbnud_i_rv;
        *var_xcor_i_slot = var_xcor_i;
        *var_xcor_i_rv_slot = var_xcor_i_rv;
    }

    pub(super) fn stamp_reactive_block_16(
        p: &Parameters,
        var_a1_p: f64,
        var_a2_p: f64,
        var_a3_p: f64,
        var_a4_p: f64,
        var_agidl_p: f64,
        var_agidld_p: f64,
        var_alp1_p: f64,
        var_alp1ac_p: f64,
        var_alp2_p: f64,
        var_alp_p: f64,
        var_alpac_p: f64,
        var_axac_p: f64,
        var_axinr_p: f64,
        var_betnedge_p: f64,
        var_bgidl_p: f64,
        var_bgidld_p: f64,
        var_cfbedge_p: f64,
        var_cfdedge_p: f64,
        var_cfedge_p: f64,
        var_cfr_p: f64,
        var_cfrd_p: f64,
        var_cgbov_p: f64,
        var_cgidl_p: f64,
        var_cgidld_p: f64,
        var_cgov_p: f64,
        var_cgovaccg_p: f64,
        var_cgovd_p: f64,
        var_chib_p: f64,
        var_cinr_p: f64,
        var_cinrd_p: f64,
        var_cox_p: f64,
        var_ctedge_p: f64,
        var_delvtac_p: f64,
        var_dphibedge_p: f64,
        var_dvfbinr_p: f64,
        var_facneffac_p: f64,
        var_fcgovacc_p: f64,
        var_fcgovaccd_p: f64,
        var_fcinracc_p: f64,
        var_fcinrdep_p: f64,
        var_fnt_p: f64,
        var_gc2_p: f64,
        var_gc2ov_p: f64,
        var_gc2ovd_p: f64,
        var_gc3_p: f64,
        var_gc3ov_p: f64,
        var_gc3ovd_p: f64,
        var_gco_p: f64,
        var_iginv_p: f64,
        var_igov_p: f64,
        var_igovd_p: f64,
        var_imaxii_p: f64,
        var_neffedge_p: f64,
        var_nf_i: f64,
        var_nov_i: f64,
        var_pscebedge_p: f64,
        var_pscededge_p: f64,
        var_psceedge_p: f64,
        var_sta2_p: f64,
        var_stbetedge_p: f64,
        var_stbgidl_p: f64,
        var_stbgidld_p: f64,
        var_stig_p: f64,
        var_stvfbedge_p: f64,
        var_thesatac_p: f64,
        var_toxov_i: f64,
        var_vfbedge_p: f64,
        var_vp_p: f64,
        var_a1_i_slot: &mut f64,
        var_a1_i_rv_slot: &mut f64,
        var_a2_i_slot: &mut f64,
        var_a2_i_rv_slot: &mut f64,
        var_a3_i_slot: &mut f64,
        var_a3_i_rv_slot: &mut f64,
        var_a4_i_slot: &mut f64,
        var_a4_i_rv_slot: &mut f64,
        var_agidl_i_slot: &mut f64,
        var_agidl_i_rv_slot: &mut f64,
        var_agidld_i_slot: &mut f64,
        var_agidld_i_rv_slot: &mut f64,
        var_alp1_i_slot: &mut f64,
        var_alp1_i_rv_slot: &mut f64,
        var_alp1ac_i_slot: &mut f64,
        var_alp1ac_i_rv_slot: &mut f64,
        var_alp2_i_slot: &mut f64,
        var_alp2_i_rv_slot: &mut f64,
        var_alp_i_slot: &mut f64,
        var_alp_i_rv_slot: &mut f64,
        var_alpac_i_slot: &mut f64,
        var_alpac_i_rv_slot: &mut f64,
        var_axac_i_slot: &mut f64,
        var_axac_i_rv_slot: &mut f64,
        var_axinr_i_slot: &mut f64,
        var_axinr_i_rv_slot: &mut f64,
        var_betnedge_i_slot: &mut f64,
        var_betnedge_i_rv_slot: &mut f64,
        var_bgidl_i_slot: &mut f64,
        var_bgidl_i_rv_slot: &mut f64,
        var_bgidld_i_slot: &mut f64,
        var_bgidld_i_rv_slot: &mut f64,
        var_cfbedge_i_slot: &mut f64,
        var_cfbedge_i_rv_slot: &mut f64,
        var_cfdedge_i_slot: &mut f64,
        var_cfdedge_i_rv_slot: &mut f64,
        var_cfedge_i_slot: &mut f64,
        var_cfedge_i_rv_slot: &mut f64,
        var_cfr_i_slot: &mut f64,
        var_cfr_i_rv_slot: &mut f64,
        var_cfrd_i_slot: &mut f64,
        var_cfrd_i_rv_slot: &mut f64,
        var_cgbov_i_slot: &mut f64,
        var_cgbov_i_rv_slot: &mut f64,
        var_cgidl_i_slot: &mut f64,
        var_cgidl_i_rv_slot: &mut f64,
        var_cgidld_i_slot: &mut f64,
        var_cgidld_i_rv_slot: &mut f64,
        var_cgov_i_slot: &mut f64,
        var_cgov_i_rv_slot: &mut f64,
        var_cgovaccg_i_slot: &mut f64,
        var_cgovaccg_i_rv_slot: &mut f64,
        var_cgovd_i_slot: &mut f64,
        var_cgovd_i_rv_slot: &mut f64,
        var_chib_i_slot: &mut f64,
        var_chib_i_rv_slot: &mut f64,
        var_cinr_i_slot: &mut f64,
        var_cinr_i_rv_slot: &mut f64,
        var_cinrd_i_slot: &mut f64,
        var_cinrd_i_rv_slot: &mut f64,
        var_cox_i_slot: &mut f64,
        var_cox_i_rv_slot: &mut f64,
        var_ctedge_i_slot: &mut f64,
        var_ctedge_i_rv_slot: &mut f64,
        var_delvtac_i_slot: &mut f64,
        var_delvtac_i_rv_slot: &mut f64,
        var_delvto_i_slot: &mut f64,
        var_delvto_i_rv_slot: &mut f64,
        var_delvtoedge_i_slot: &mut f64,
        var_delvtoedge_i_rv_slot: &mut f64,
        var_dphibedge_i_slot: &mut f64,
        var_dphibedge_i_rv_slot: &mut f64,
        var_dvfbinr_i_slot: &mut f64,
        var_dvfbinr_i_rv_slot: &mut f64,
        var_facneffac_i_slot: &mut f64,
        var_facneffac_i_rv_slot: &mut f64,
        var_factuo_i_slot: &mut f64,
        var_factuo_i_rv_slot: &mut f64,
        var_factuoedge_i_slot: &mut f64,
        var_factuoedge_i_rv_slot: &mut f64,
        var_fcgovacc_i_slot: &mut f64,
        var_fcgovacc_i_rv_slot: &mut f64,
        var_fcgovaccd_i_slot: &mut f64,
        var_fcgovaccd_i_rv_slot: &mut f64,
        var_fcinracc_i_slot: &mut f64,
        var_fcinracc_i_rv_slot: &mut f64,
        var_fcinrdep_i_slot: &mut f64,
        var_fcinrdep_i_rv_slot: &mut f64,
        var_fnt_i_slot: &mut f64,
        var_fnt_i_rv_slot: &mut f64,
        var_gc2_i_slot: &mut f64,
        var_gc2_i_rv_slot: &mut f64,
        var_gc2ov_i_slot: &mut f64,
        var_gc2ov_i_rv_slot: &mut f64,
        var_gc2ovd_i_slot: &mut f64,
        var_gc2ovd_i_rv_slot: &mut f64,
        var_gc3_i_slot: &mut f64,
        var_gc3_i_rv_slot: &mut f64,
        var_gc3ov_i_slot: &mut f64,
        var_gc3ov_i_rv_slot: &mut f64,
        var_gc3ovd_i_slot: &mut f64,
        var_gc3ovd_i_rv_slot: &mut f64,
        var_gco_i_slot: &mut f64,
        var_gco_i_rv_slot: &mut f64,
        var_guard147_slot: &mut f64,
        var_guard147_rv_slot: &mut f64,
        var_iginv_i_slot: &mut f64,
        var_iginv_i_rv_slot: &mut f64,
        var_igov_i_slot: &mut f64,
        var_igov_i_rv_slot: &mut f64,
        var_igovd_i_slot: &mut f64,
        var_igovd_i_rv_slot: &mut f64,
        var_imaxii_i_slot: &mut f64,
        var_imaxii_i_rv_slot: &mut f64,
        var_mult_inst_slot: &mut f64,
        var_mult_inst_rv_slot: &mut f64,
        var_neffedge_i_slot: &mut f64,
        var_neffedge_i_rv_slot: &mut f64,
        var_novd_i_slot: &mut f64,
        var_novd_i_rv_slot: &mut f64,
        var_pscebedge_i_slot: &mut f64,
        var_pscebedge_i_rv_slot: &mut f64,
        var_pscededge_i_slot: &mut f64,
        var_pscededge_i_rv_slot: &mut f64,
        var_psceedge_i_slot: &mut f64,
        var_psceedge_i_rv_slot: &mut f64,
        var_sta2_i_slot: &mut f64,
        var_sta2_i_rv_slot: &mut f64,
        var_stbetedge_i_slot: &mut f64,
        var_stbetedge_i_rv_slot: &mut f64,
        var_stbgidl_i_slot: &mut f64,
        var_stbgidl_i_rv_slot: &mut f64,
        var_stbgidld_i_slot: &mut f64,
        var_stbgidld_i_rv_slot: &mut f64,
        var_stig_i_slot: &mut f64,
        var_stig_i_rv_slot: &mut f64,
        var_stvfbedge_i_slot: &mut f64,
        var_stvfbedge_i_rv_slot: &mut f64,
        var_thesatac_i_slot: &mut f64,
        var_thesatac_i_rv_slot: &mut f64,
        var_toxovd_i_slot: &mut f64,
        var_toxovd_i_rv_slot: &mut f64,
        var_vfbedge_i_slot: &mut f64,
        var_vfbedge_i_rv_slot: &mut f64,
        var_vp_i_slot: &mut f64,
        var_vp_i_rv_slot: &mut f64,
    ) {
        let mut var_a1_i: f64 = *var_a1_i_slot;
        let mut var_a1_i_rv: f64 = *var_a1_i_rv_slot;
        let mut var_a2_i: f64 = *var_a2_i_slot;
        let mut var_a2_i_rv: f64 = *var_a2_i_rv_slot;
        let mut var_a3_i: f64 = *var_a3_i_slot;
        let mut var_a3_i_rv: f64 = *var_a3_i_rv_slot;
        let mut var_a4_i: f64 = *var_a4_i_slot;
        let mut var_a4_i_rv: f64 = *var_a4_i_rv_slot;
        let mut var_agidl_i: f64 = *var_agidl_i_slot;
        let mut var_agidl_i_rv: f64 = *var_agidl_i_rv_slot;
        let mut var_agidld_i: f64 = *var_agidld_i_slot;
        let mut var_agidld_i_rv: f64 = *var_agidld_i_rv_slot;
        let mut var_alp1_i: f64 = *var_alp1_i_slot;
        let mut var_alp1_i_rv: f64 = *var_alp1_i_rv_slot;
        let mut var_alp1ac_i: f64 = *var_alp1ac_i_slot;
        let mut var_alp1ac_i_rv: f64 = *var_alp1ac_i_rv_slot;
        let mut var_alp2_i: f64 = *var_alp2_i_slot;
        let mut var_alp2_i_rv: f64 = *var_alp2_i_rv_slot;
        let mut var_alp_i: f64 = *var_alp_i_slot;
        let mut var_alp_i_rv: f64 = *var_alp_i_rv_slot;
        let mut var_alpac_i: f64 = *var_alpac_i_slot;
        let mut var_alpac_i_rv: f64 = *var_alpac_i_rv_slot;
        let mut var_axac_i: f64 = *var_axac_i_slot;
        let mut var_axac_i_rv: f64 = *var_axac_i_rv_slot;
        let mut var_axinr_i: f64 = *var_axinr_i_slot;
        let mut var_axinr_i_rv: f64 = *var_axinr_i_rv_slot;
        let mut var_betnedge_i: f64 = *var_betnedge_i_slot;
        let mut var_betnedge_i_rv: f64 = *var_betnedge_i_rv_slot;
        let mut var_bgidl_i: f64 = *var_bgidl_i_slot;
        let mut var_bgidl_i_rv: f64 = *var_bgidl_i_rv_slot;
        let mut var_bgidld_i: f64 = *var_bgidld_i_slot;
        let mut var_bgidld_i_rv: f64 = *var_bgidld_i_rv_slot;
        let mut var_cfbedge_i: f64 = *var_cfbedge_i_slot;
        let mut var_cfbedge_i_rv: f64 = *var_cfbedge_i_rv_slot;
        let mut var_cfdedge_i: f64 = *var_cfdedge_i_slot;
        let mut var_cfdedge_i_rv: f64 = *var_cfdedge_i_rv_slot;
        let mut var_cfedge_i: f64 = *var_cfedge_i_slot;
        let mut var_cfedge_i_rv: f64 = *var_cfedge_i_rv_slot;
        let mut var_cfr_i: f64 = *var_cfr_i_slot;
        let mut var_cfr_i_rv: f64 = *var_cfr_i_rv_slot;
        let mut var_cfrd_i: f64 = *var_cfrd_i_slot;
        let mut var_cfrd_i_rv: f64 = *var_cfrd_i_rv_slot;
        let mut var_cgbov_i: f64 = *var_cgbov_i_slot;
        let mut var_cgbov_i_rv: f64 = *var_cgbov_i_rv_slot;
        let mut var_cgidl_i: f64 = *var_cgidl_i_slot;
        let mut var_cgidl_i_rv: f64 = *var_cgidl_i_rv_slot;
        let mut var_cgidld_i: f64 = *var_cgidld_i_slot;
        let mut var_cgidld_i_rv: f64 = *var_cgidld_i_rv_slot;
        let mut var_cgov_i: f64 = *var_cgov_i_slot;
        let mut var_cgov_i_rv: f64 = *var_cgov_i_rv_slot;
        let mut var_cgovaccg_i: f64 = *var_cgovaccg_i_slot;
        let mut var_cgovaccg_i_rv: f64 = *var_cgovaccg_i_rv_slot;
        let mut var_cgovd_i: f64 = *var_cgovd_i_slot;
        let mut var_cgovd_i_rv: f64 = *var_cgovd_i_rv_slot;
        let mut var_chib_i: f64 = *var_chib_i_slot;
        let mut var_chib_i_rv: f64 = *var_chib_i_rv_slot;
        let mut var_cinr_i: f64 = *var_cinr_i_slot;
        let mut var_cinr_i_rv: f64 = *var_cinr_i_rv_slot;
        let mut var_cinrd_i: f64 = *var_cinrd_i_slot;
        let mut var_cinrd_i_rv: f64 = *var_cinrd_i_rv_slot;
        let mut var_cox_i: f64 = *var_cox_i_slot;
        let mut var_cox_i_rv: f64 = *var_cox_i_rv_slot;
        let mut var_ctedge_i: f64 = *var_ctedge_i_slot;
        let mut var_ctedge_i_rv: f64 = *var_ctedge_i_rv_slot;
        let mut var_delvtac_i: f64 = *var_delvtac_i_slot;
        let mut var_delvtac_i_rv: f64 = *var_delvtac_i_rv_slot;
        let mut var_delvto_i: f64 = *var_delvto_i_slot;
        let mut var_delvto_i_rv: f64 = *var_delvto_i_rv_slot;
        let mut var_delvtoedge_i: f64 = *var_delvtoedge_i_slot;
        let mut var_delvtoedge_i_rv: f64 = *var_delvtoedge_i_rv_slot;
        let mut var_dphibedge_i: f64 = *var_dphibedge_i_slot;
        let mut var_dphibedge_i_rv: f64 = *var_dphibedge_i_rv_slot;
        let mut var_dvfbinr_i: f64 = *var_dvfbinr_i_slot;
        let mut var_dvfbinr_i_rv: f64 = *var_dvfbinr_i_rv_slot;
        let mut var_facneffac_i: f64 = *var_facneffac_i_slot;
        let mut var_facneffac_i_rv: f64 = *var_facneffac_i_rv_slot;
        let mut var_factuo_i: f64 = *var_factuo_i_slot;
        let mut var_factuo_i_rv: f64 = *var_factuo_i_rv_slot;
        let mut var_factuoedge_i: f64 = *var_factuoedge_i_slot;
        let mut var_factuoedge_i_rv: f64 = *var_factuoedge_i_rv_slot;
        let mut var_fcgovacc_i: f64 = *var_fcgovacc_i_slot;
        let mut var_fcgovacc_i_rv: f64 = *var_fcgovacc_i_rv_slot;
        let mut var_fcgovaccd_i: f64 = *var_fcgovaccd_i_slot;
        let mut var_fcgovaccd_i_rv: f64 = *var_fcgovaccd_i_rv_slot;
        let mut var_fcinracc_i: f64 = *var_fcinracc_i_slot;
        let mut var_fcinracc_i_rv: f64 = *var_fcinracc_i_rv_slot;
        let mut var_fcinrdep_i: f64 = *var_fcinrdep_i_slot;
        let mut var_fcinrdep_i_rv: f64 = *var_fcinrdep_i_rv_slot;
        let mut var_fnt_i: f64 = *var_fnt_i_slot;
        let mut var_fnt_i_rv: f64 = *var_fnt_i_rv_slot;
        let mut var_gc2_i: f64 = *var_gc2_i_slot;
        let mut var_gc2_i_rv: f64 = *var_gc2_i_rv_slot;
        let mut var_gc2ov_i: f64 = *var_gc2ov_i_slot;
        let mut var_gc2ov_i_rv: f64 = *var_gc2ov_i_rv_slot;
        let mut var_gc2ovd_i: f64 = *var_gc2ovd_i_slot;
        let mut var_gc2ovd_i_rv: f64 = *var_gc2ovd_i_rv_slot;
        let mut var_gc3_i: f64 = *var_gc3_i_slot;
        let mut var_gc3_i_rv: f64 = *var_gc3_i_rv_slot;
        let mut var_gc3ov_i: f64 = *var_gc3ov_i_slot;
        let mut var_gc3ov_i_rv: f64 = *var_gc3ov_i_rv_slot;
        let mut var_gc3ovd_i: f64 = *var_gc3ovd_i_slot;
        let mut var_gc3ovd_i_rv: f64 = *var_gc3ovd_i_rv_slot;
        let mut var_gco_i: f64 = *var_gco_i_slot;
        let mut var_gco_i_rv: f64 = *var_gco_i_rv_slot;
        let mut var_guard147: f64 = *var_guard147_slot;
        let mut var_guard147_rv: f64 = *var_guard147_rv_slot;
        let mut var_iginv_i: f64 = *var_iginv_i_slot;
        let mut var_iginv_i_rv: f64 = *var_iginv_i_rv_slot;
        let mut var_igov_i: f64 = *var_igov_i_slot;
        let mut var_igov_i_rv: f64 = *var_igov_i_rv_slot;
        let mut var_igovd_i: f64 = *var_igovd_i_slot;
        let mut var_igovd_i_rv: f64 = *var_igovd_i_rv_slot;
        let mut var_imaxii_i: f64 = *var_imaxii_i_slot;
        let mut var_imaxii_i_rv: f64 = *var_imaxii_i_rv_slot;
        let mut var_mult_inst: f64 = *var_mult_inst_slot;
        let mut var_mult_inst_rv: f64 = *var_mult_inst_rv_slot;
        let mut var_neffedge_i: f64 = *var_neffedge_i_slot;
        let mut var_neffedge_i_rv: f64 = *var_neffedge_i_rv_slot;
        let mut var_novd_i: f64 = *var_novd_i_slot;
        let mut var_novd_i_rv: f64 = *var_novd_i_rv_slot;
        let mut var_pscebedge_i: f64 = *var_pscebedge_i_slot;
        let mut var_pscebedge_i_rv: f64 = *var_pscebedge_i_rv_slot;
        let mut var_pscededge_i: f64 = *var_pscededge_i_slot;
        let mut var_pscededge_i_rv: f64 = *var_pscededge_i_rv_slot;
        let mut var_psceedge_i: f64 = *var_psceedge_i_slot;
        let mut var_psceedge_i_rv: f64 = *var_psceedge_i_rv_slot;
        let mut var_sta2_i: f64 = *var_sta2_i_slot;
        let mut var_sta2_i_rv: f64 = *var_sta2_i_rv_slot;
        let mut var_stbetedge_i: f64 = *var_stbetedge_i_slot;
        let mut var_stbetedge_i_rv: f64 = *var_stbetedge_i_rv_slot;
        let mut var_stbgidl_i: f64 = *var_stbgidl_i_slot;
        let mut var_stbgidl_i_rv: f64 = *var_stbgidl_i_rv_slot;
        let mut var_stbgidld_i: f64 = *var_stbgidld_i_slot;
        let mut var_stbgidld_i_rv: f64 = *var_stbgidld_i_rv_slot;
        let mut var_stig_i: f64 = *var_stig_i_slot;
        let mut var_stig_i_rv: f64 = *var_stig_i_rv_slot;
        let mut var_stvfbedge_i: f64 = *var_stvfbedge_i_slot;
        let mut var_stvfbedge_i_rv: f64 = *var_stvfbedge_i_rv_slot;
        let mut var_thesatac_i: f64 = *var_thesatac_i_slot;
        let mut var_thesatac_i_rv: f64 = *var_thesatac_i_rv_slot;
        let mut var_toxovd_i: f64 = *var_toxovd_i_slot;
        let mut var_toxovd_i_rv: f64 = *var_toxovd_i_rv_slot;
        let mut var_vfbedge_i: f64 = *var_vfbedge_i_slot;
        let mut var_vfbedge_i_rv: f64 = *var_vfbedge_i_rv_slot;
        let mut var_vp_i: f64 = *var_vp_i_slot;
        let mut var_vp_i_rv: f64 = *var_vp_i_rv_slot;

        let (assign10030_e9789,) = {
    if (var_alp_p > 0.0) {
        (var_alp_p,)
    } else {
        (0.0,)
    }
};
        var_alp_i = assign10030_e9789;
        var_alp_i_rv = 0.0;

        let (assign10040_e9795,) = {
    if (var_alp1_p > 0.0) {
        (var_alp1_p,)
    } else {
        (0.0,)
    }
};
        var_alp1_i = assign10040_e9795;
        var_alp1_i_rv = 0.0;

        let (assign10050_e9801,) = {
    if (var_alp2_p > 0.0) {
        (var_alp2_p,)
    } else {
        (0.0,)
    }
};
        var_alp2_i = assign10050_e9801;
        var_alp2_i_rv = 0.0;

        var_vp_i = var_vp_p;
        var_vp_i_rv = 0.0;

        let (assign10070_e9808,) = {
    if (var_a1_p > 0.0) {
        (var_a1_p,)
    } else {
        (0.0,)
    }
};
        var_a1_i = assign10070_e9808;
        var_a1_i_rv = 0.0;

        var_a2_i = var_a2_p;
        var_a2_i_rv = 0.0;

        var_sta2_i = var_sta2_p;
        var_sta2_i_rv = 0.0;

        let (assign10100_e9816,) = {
    if (var_a3_p > 0.0) {
        (var_a3_p,)
    } else {
        (0.0,)
    }
};
        var_a3_i = assign10100_e9816;
        var_a3_i_rv = 0.0;

        let (assign10110_e9822,) = {
    if (var_a4_p > 0.0) {
        (var_a4_p,)
    } else {
        (0.0,)
    }
};
        var_a4_i = assign10110_e9822;
        var_a4_i_rv = 0.0;

        let (assign10120_e9828,) = {
    if (var_imaxii_p > 1e-12) {
        (var_imaxii_p,)
    } else {
        (1e-12,)
    }
};
        var_imaxii_i = assign10120_e9828;
        var_imaxii_i_rv = 0.0;

        var_gco_i = var_gco_p;
        var_gco_i_rv = 0.0;

        let (assign10140_e9835,) = {
    if (var_iginv_p > 0.0) {
        (var_iginv_p,)
    } else {
        (0.0,)
    }
};
        var_iginv_i = assign10140_e9835;
        var_iginv_i_rv = 0.0;

        let (assign10150_e9841,) = {
    if (var_igov_p > 0.0) {
        (var_igov_p,)
    } else {
        (0.0,)
    }
};
        var_igov_i = assign10150_e9841;
        var_igov_i_rv = 0.0;

        let (assign10160_e9847,) = {
    if (var_igovd_p > 0.0) {
        (var_igovd_p,)
    } else {
        (0.0,)
    }
};
        var_igovd_i = assign10160_e9847;
        var_igovd_i_rv = 0.0;

        var_stig_i = var_stig_p;
        var_stig_i_rv = 0.0;

        var_gc2_i = var_gc2_p;
        var_gc2_i_rv = 0.0;

        var_gc3_i = var_gc3_p;
        var_gc3_i_rv = 0.0;

        var_gc2ov_i = var_gc2ov_p;
        var_gc2ov_i_rv = 0.0;

        var_gc3ov_i = var_gc3ov_p;
        var_gc3ov_i_rv = 0.0;

        var_gc2ovd_i = var_gc2ovd_p;
        var_gc2ovd_i_rv = 0.0;

        var_gc3ovd_i = var_gc3ovd_p;
        var_gc3ovd_i_rv = 0.0;

        var_chib_i = var_chib_p;
        var_chib_i_rv = 0.0;

        let (assign10250_e9861,) = {
    if (var_agidl_p > 0.0) {
        (var_agidl_p,)
    } else {
        (0.0,)
    }
};
        var_agidl_i = assign10250_e9861;
        var_agidl_i_rv = 0.0;

        let (assign10260_e9867,) = {
    if (var_agidld_p > 0.0) {
        (var_agidld_p,)
    } else {
        (0.0,)
    }
};
        var_agidld_i = assign10260_e9867;
        var_agidld_i_rv = 0.0;

        var_bgidl_i = var_bgidl_p;
        var_bgidl_i_rv = 0.0;

        var_bgidld_i = var_bgidld_p;
        var_bgidld_i_rv = 0.0;

        var_stbgidl_i = var_stbgidl_p;
        var_stbgidl_i_rv = 0.0;

        var_stbgidld_i = var_stbgidld_p;
        var_stbgidld_i_rv = 0.0;

        var_cgidl_i = var_cgidl_p;
        var_cgidl_i_rv = 0.0;

        var_cgidld_i = var_cgidld_p;
        var_cgidld_i_rv = 0.0;

        let (assign10330_e9879,) = {
    if (var_cox_p > 0.0) {
        (var_cox_p,)
    } else {
        (0.0,)
    }
};
        var_cox_i = assign10330_e9879;
        var_cox_i_rv = 0.0;

        var_delvtac_i = var_delvtac_p;
        var_delvtac_i_rv = 0.0;

        let (assign10350_e9886,) = {
    if (var_facneffac_p > 0.0) {
        (var_facneffac_p,)
    } else {
        (0.0,)
    }
};
        var_facneffac_i = assign10350_e9886;
        var_facneffac_i_rv = 0.0;

        let (assign10360_e9892,) = {
    if (var_thesatac_p > 0.0) {
        (var_thesatac_p,)
    } else {
        (0.0,)
    }
};
        var_thesatac_i = assign10360_e9892;
        var_thesatac_i_rv = 0.0;

        let (assign10370_e9898,) = {
    if (var_axac_p > 2.0) {
        (var_axac_p,)
    } else {
        (2.0,)
    }
};
        var_axac_i = assign10370_e9898;
        var_axac_i_rv = 0.0;

        var_alpac_i = var_alpac_p;
        var_alpac_i_rv = 0.0;

        let (assign10390_e9905,) = {
    if (var_alp1ac_p > 0.0) {
        (var_alp1ac_p,)
    } else {
        (0.0,)
    }
};
        var_alp1ac_i = assign10390_e9905;
        var_alp1ac_i_rv = 0.0;

        let (assign10400_e9911,) = {
    if (var_cgov_p > 0.0) {
        (var_cgov_p,)
    } else {
        (0.0,)
    }
};
        var_cgov_i = assign10400_e9911;
        var_cgov_i_rv = 0.0;

        let (assign10410_e9917,) = {
    if (var_cgovd_p > 0.0) {
        (var_cgovd_p,)
    } else {
        (0.0,)
    }
};
        var_cgovd_i = assign10410_e9917;
        var_cgovd_i_rv = 0.0;

        var_fcgovacc_i = var_fcgovacc_p;
        var_fcgovacc_i_rv = 0.0;

        var_fcgovaccd_i = var_fcgovaccd_p;
        var_fcgovaccd_i_rv = 0.0;

        var_cgovaccg_i = var_cgovaccg_p;
        var_cgovaccg_i_rv = 0.0;

        let (assign10450_e9926,) = {
    if (var_cgbov_p > 0.0) {
        (var_cgbov_p,)
    } else {
        (0.0,)
    }
};
        var_cgbov_i = assign10450_e9926;
        var_cgbov_i_rv = 0.0;

        let (assign10460_e9932,) = {
    if (var_cinr_p > 0.0) {
        (var_cinr_p,)
    } else {
        (0.0,)
    }
};
        var_cinr_i = assign10460_e9932;
        var_cinr_i_rv = 0.0;

        let (assign10470_e9938,) = {
    if (var_cinrd_p > 0.0) {
        (var_cinrd_p,)
    } else {
        (0.0,)
    }
};
        var_cinrd_i = assign10470_e9938;
        var_cinrd_i_rv = 0.0;

        var_dvfbinr_i = var_dvfbinr_p;
        var_dvfbinr_i_rv = 0.0;

        var_fcinrdep_i = var_fcinrdep_p;
        var_fcinrdep_i_rv = 0.0;

        var_fcinracc_i = var_fcinracc_p;
        var_fcinracc_i_rv = 0.0;

        var_axinr_i = var_axinr_p;
        var_axinr_i_rv = 0.0;

        let (assign10520_e9948,) = {
    if (var_cfr_p > 0.0) {
        (var_cfr_p,)
    } else {
        (0.0,)
    }
};
        var_cfr_i = assign10520_e9948;
        var_cfr_i_rv = 0.0;

        let (assign10530_e9954,) = {
    if (var_cfrd_p > 0.0) {
        (var_cfrd_p,)
    } else {
        (0.0,)
    }
};
        var_cfrd_i = assign10530_e9954;
        var_cfrd_i_rv = 0.0;

        var_fnt_i = var_fnt_p;
        var_fnt_i_rv = 0.0;

        var_vfbedge_i = var_vfbedge_p;
        var_vfbedge_i_rv = 0.0;

        var_stvfbedge_i = var_stvfbedge_p;
        var_stvfbedge_i_rv = 0.0;

        var_dphibedge_i = var_dphibedge_p;
        var_dphibedge_i_rv = 0.0;

        let (assign10630_e9994,) = {
    if (var_neffedge_p > 1e20) {
        let (assign10630_e9992,) = {
            if (var_neffedge_p < 1e26) {
                (var_neffedge_p,)
            } else {
                (1e26,)
            }
        };
        (assign10630_e9992,)
    } else {
        (1e20,)
    }
};
        var_neffedge_i = assign10630_e9994;
        var_neffedge_i_rv = 0.0;

        let (assign10640_e10000,) = {
    if (var_ctedge_p > 0.0) {
        (var_ctedge_p,)
    } else {
        (0.0,)
    }
};
        var_ctedge_i = assign10640_e10000;
        var_ctedge_i_rv = 0.0;

        let (assign10650_e10006,) = {
    if (var_betnedge_p > 0.0) {
        (var_betnedge_p,)
    } else {
        (0.0,)
    }
};
        var_betnedge_i = assign10650_e10006;
        var_betnedge_i_rv = 0.0;

        var_stbetedge_i = var_stbetedge_p;
        var_stbetedge_i_rv = 0.0;

        let (assign10670_e10013,) = {
    if (var_psceedge_p > 0.0) {
        (var_psceedge_p,)
    } else {
        (0.0,)
    }
};
        var_psceedge_i = assign10670_e10013;
        var_psceedge_i_rv = 0.0;

        let (assign10680_e10024,) = {
    if (var_pscebedge_p > 0.0) {
        let (assign10680_e10022,) = {
            if (var_pscebedge_p < 1.0) {
                (var_pscebedge_p,)
            } else {
                (1.0,)
            }
        };
        (assign10680_e10022,)
    } else {
        (0.0,)
    }
};
        var_pscebedge_i = assign10680_e10024;
        var_pscebedge_i_rv = 0.0;

        let (assign10690_e10030,) = {
    if (var_pscededge_p > 0.0) {
        (var_pscededge_p,)
    } else {
        (0.0,)
    }
};
        var_pscededge_i = assign10690_e10030;
        var_pscededge_i_rv = 0.0;

        let (assign10700_e10036,) = {
    if (var_cfedge_p > 0.0) {
        (var_cfedge_p,)
    } else {
        (0.0,)
    }
};
        var_cfedge_i = assign10700_e10036;
        var_cfedge_i_rv = 0.0;

        let (assign10710_e10047,) = {
    if (var_cfbedge_p > 0.0) {
        let (assign10710_e10045,) = {
            if (var_cfbedge_p < 1.0) {
                (var_cfbedge_p,)
            } else {
                (1.0,)
            }
        };
        (assign10710_e10045,)
    } else {
        (0.0,)
    }
};
        var_cfbedge_i = assign10710_e10047;
        var_cfbedge_i_rv = 0.0;

        let (assign10720_e10053,) = {
    if (var_cfdedge_p > 0.0) {
        (var_cfdedge_p,)
    } else {
        (0.0,)
    }
};
        var_cfdedge_i = assign10720_e10053;
        var_cfdedge_i_rv = 0.0;

        let assign10850_e10088: f64 = (p.p31 * var_nf_i);
        let (assign10850_e10095,) = {
    if (assign10850_e10088 > 0.0) {
        let assign10850_e10093: f64 = (p.p31 * var_nf_i);
        (assign10850_e10093,)
    } else {
        (0.0,)
    }
};
        var_mult_inst = assign10850_e10095;
        var_mult_inst_rv = 0.0;

        var_factuo_i = p.p16;
        var_factuo_i_rv = 0.0;

        var_delvto_i = p.p15;
        var_delvto_i_rv = 0.0;

        var_factuoedge_i = p.p18;
        var_factuoedge_i_rv = 0.0;

        var_delvtoedge_i = p.p17;
        var_delvtoedge_i_rv = 0.0;

        let assign10900_e10102: f64 = if p.p44 == 0.0 { 1.0 } else { 0.0 };
        var_guard147 = assign10900_e10102;
        var_guard147_rv = 0.0;

        let (assign10910_e10106,) = {
    if (var_guard147 != 0.0) {
        (var_toxov_i,)
    } else {
        (var_toxovd_i,)
    }
};
        var_toxovd_i = assign10910_e10106;
        var_toxovd_i_rv = 0.0;

        let (assign10920_e10110,) = {
    if (var_guard147 != 0.0) {
        (var_nov_i,)
    } else {
        (var_novd_i,)
    }
};
        var_novd_i = assign10920_e10110;
        var_novd_i_rv = 0.0;

        let (assign10930_e10114,) = {
    if (var_guard147 != 0.0) {
        (var_agidl_i,)
    } else {
        (var_agidld_i,)
    }
};
        var_agidld_i = assign10930_e10114;
        var_agidld_i_rv = 0.0;

        *var_a1_i_slot = var_a1_i;
        *var_a1_i_rv_slot = var_a1_i_rv;
        *var_a2_i_slot = var_a2_i;
        *var_a2_i_rv_slot = var_a2_i_rv;
        *var_a3_i_slot = var_a3_i;
        *var_a3_i_rv_slot = var_a3_i_rv;
        *var_a4_i_slot = var_a4_i;
        *var_a4_i_rv_slot = var_a4_i_rv;
        *var_agidl_i_slot = var_agidl_i;
        *var_agidl_i_rv_slot = var_agidl_i_rv;
        *var_agidld_i_slot = var_agidld_i;
        *var_agidld_i_rv_slot = var_agidld_i_rv;
        *var_alp1_i_slot = var_alp1_i;
        *var_alp1_i_rv_slot = var_alp1_i_rv;
        *var_alp1ac_i_slot = var_alp1ac_i;
        *var_alp1ac_i_rv_slot = var_alp1ac_i_rv;
        *var_alp2_i_slot = var_alp2_i;
        *var_alp2_i_rv_slot = var_alp2_i_rv;
        *var_alp_i_slot = var_alp_i;
        *var_alp_i_rv_slot = var_alp_i_rv;
        *var_alpac_i_slot = var_alpac_i;
        *var_alpac_i_rv_slot = var_alpac_i_rv;
        *var_axac_i_slot = var_axac_i;
        *var_axac_i_rv_slot = var_axac_i_rv;
        *var_axinr_i_slot = var_axinr_i;
        *var_axinr_i_rv_slot = var_axinr_i_rv;
        *var_betnedge_i_slot = var_betnedge_i;
        *var_betnedge_i_rv_slot = var_betnedge_i_rv;
        *var_bgidl_i_slot = var_bgidl_i;
        *var_bgidl_i_rv_slot = var_bgidl_i_rv;
        *var_bgidld_i_slot = var_bgidld_i;
        *var_bgidld_i_rv_slot = var_bgidld_i_rv;
        *var_cfbedge_i_slot = var_cfbedge_i;
        *var_cfbedge_i_rv_slot = var_cfbedge_i_rv;
        *var_cfdedge_i_slot = var_cfdedge_i;
        *var_cfdedge_i_rv_slot = var_cfdedge_i_rv;
        *var_cfedge_i_slot = var_cfedge_i;
        *var_cfedge_i_rv_slot = var_cfedge_i_rv;
        *var_cfr_i_slot = var_cfr_i;
        *var_cfr_i_rv_slot = var_cfr_i_rv;
        *var_cfrd_i_slot = var_cfrd_i;
        *var_cfrd_i_rv_slot = var_cfrd_i_rv;
        *var_cgbov_i_slot = var_cgbov_i;
        *var_cgbov_i_rv_slot = var_cgbov_i_rv;
        *var_cgidl_i_slot = var_cgidl_i;
        *var_cgidl_i_rv_slot = var_cgidl_i_rv;
        *var_cgidld_i_slot = var_cgidld_i;
        *var_cgidld_i_rv_slot = var_cgidld_i_rv;
        *var_cgov_i_slot = var_cgov_i;
        *var_cgov_i_rv_slot = var_cgov_i_rv;
        *var_cgovaccg_i_slot = var_cgovaccg_i;
        *var_cgovaccg_i_rv_slot = var_cgovaccg_i_rv;
        *var_cgovd_i_slot = var_cgovd_i;
        *var_cgovd_i_rv_slot = var_cgovd_i_rv;
        *var_chib_i_slot = var_chib_i;
        *var_chib_i_rv_slot = var_chib_i_rv;
        *var_cinr_i_slot = var_cinr_i;
        *var_cinr_i_rv_slot = var_cinr_i_rv;
        *var_cinrd_i_slot = var_cinrd_i;
        *var_cinrd_i_rv_slot = var_cinrd_i_rv;
        *var_cox_i_slot = var_cox_i;
        *var_cox_i_rv_slot = var_cox_i_rv;
        *var_ctedge_i_slot = var_ctedge_i;
        *var_ctedge_i_rv_slot = var_ctedge_i_rv;
        *var_delvtac_i_slot = var_delvtac_i;
        *var_delvtac_i_rv_slot = var_delvtac_i_rv;
        *var_delvto_i_slot = var_delvto_i;
        *var_delvto_i_rv_slot = var_delvto_i_rv;
        *var_delvtoedge_i_slot = var_delvtoedge_i;
        *var_delvtoedge_i_rv_slot = var_delvtoedge_i_rv;
        *var_dphibedge_i_slot = var_dphibedge_i;
        *var_dphibedge_i_rv_slot = var_dphibedge_i_rv;
        *var_dvfbinr_i_slot = var_dvfbinr_i;
        *var_dvfbinr_i_rv_slot = var_dvfbinr_i_rv;
        *var_facneffac_i_slot = var_facneffac_i;
        *var_facneffac_i_rv_slot = var_facneffac_i_rv;
        *var_factuo_i_slot = var_factuo_i;
        *var_factuo_i_rv_slot = var_factuo_i_rv;
        *var_factuoedge_i_slot = var_factuoedge_i;
        *var_factuoedge_i_rv_slot = var_factuoedge_i_rv;
        *var_fcgovacc_i_slot = var_fcgovacc_i;
        *var_fcgovacc_i_rv_slot = var_fcgovacc_i_rv;
        *var_fcgovaccd_i_slot = var_fcgovaccd_i;
        *var_fcgovaccd_i_rv_slot = var_fcgovaccd_i_rv;
        *var_fcinracc_i_slot = var_fcinracc_i;
        *var_fcinracc_i_rv_slot = var_fcinracc_i_rv;
        *var_fcinrdep_i_slot = var_fcinrdep_i;
        *var_fcinrdep_i_rv_slot = var_fcinrdep_i_rv;
        *var_fnt_i_slot = var_fnt_i;
        *var_fnt_i_rv_slot = var_fnt_i_rv;
        *var_gc2_i_slot = var_gc2_i;
        *var_gc2_i_rv_slot = var_gc2_i_rv;
        *var_gc2ov_i_slot = var_gc2ov_i;
        *var_gc2ov_i_rv_slot = var_gc2ov_i_rv;
        *var_gc2ovd_i_slot = var_gc2ovd_i;
        *var_gc2ovd_i_rv_slot = var_gc2ovd_i_rv;
        *var_gc3_i_slot = var_gc3_i;
        *var_gc3_i_rv_slot = var_gc3_i_rv;
        *var_gc3ov_i_slot = var_gc3ov_i;
        *var_gc3ov_i_rv_slot = var_gc3ov_i_rv;
        *var_gc3ovd_i_slot = var_gc3ovd_i;
        *var_gc3ovd_i_rv_slot = var_gc3ovd_i_rv;
        *var_gco_i_slot = var_gco_i;
        *var_gco_i_rv_slot = var_gco_i_rv;
        *var_guard147_slot = var_guard147;
        *var_guard147_rv_slot = var_guard147_rv;
        *var_iginv_i_slot = var_iginv_i;
        *var_iginv_i_rv_slot = var_iginv_i_rv;
        *var_igov_i_slot = var_igov_i;
        *var_igov_i_rv_slot = var_igov_i_rv;
        *var_igovd_i_slot = var_igovd_i;
        *var_igovd_i_rv_slot = var_igovd_i_rv;
        *var_imaxii_i_slot = var_imaxii_i;
        *var_imaxii_i_rv_slot = var_imaxii_i_rv;
        *var_mult_inst_slot = var_mult_inst;
        *var_mult_inst_rv_slot = var_mult_inst_rv;
        *var_neffedge_i_slot = var_neffedge_i;
        *var_neffedge_i_rv_slot = var_neffedge_i_rv;
        *var_novd_i_slot = var_novd_i;
        *var_novd_i_rv_slot = var_novd_i_rv;
        *var_pscebedge_i_slot = var_pscebedge_i;
        *var_pscebedge_i_rv_slot = var_pscebedge_i_rv;
        *var_pscededge_i_slot = var_pscededge_i;
        *var_pscededge_i_rv_slot = var_pscededge_i_rv;
        *var_psceedge_i_slot = var_psceedge_i;
        *var_psceedge_i_rv_slot = var_psceedge_i_rv;
        *var_sta2_i_slot = var_sta2_i;
        *var_sta2_i_rv_slot = var_sta2_i_rv;
        *var_stbetedge_i_slot = var_stbetedge_i;
        *var_stbetedge_i_rv_slot = var_stbetedge_i_rv;
        *var_stbgidl_i_slot = var_stbgidl_i;
        *var_stbgidl_i_rv_slot = var_stbgidl_i_rv;
        *var_stbgidld_i_slot = var_stbgidld_i;
        *var_stbgidld_i_rv_slot = var_stbgidld_i_rv;
        *var_stig_i_slot = var_stig_i;
        *var_stig_i_rv_slot = var_stig_i_rv;
        *var_stvfbedge_i_slot = var_stvfbedge_i;
        *var_stvfbedge_i_rv_slot = var_stvfbedge_i_rv;
        *var_thesatac_i_slot = var_thesatac_i;
        *var_thesatac_i_rv_slot = var_thesatac_i_rv;
        *var_toxovd_i_slot = var_toxovd_i;
        *var_toxovd_i_rv_slot = var_toxovd_i_rv;
        *var_vfbedge_i_slot = var_vfbedge_i;
        *var_vfbedge_i_rv_slot = var_vfbedge_i_rv;
        *var_vp_i_slot = var_vp_i;
        *var_vp_i_rv_slot = var_vp_i_rv;
    }

    pub(super) fn stamp_reactive_block_17(
        p: &Parameters,
        var_ax_i: f64,
        var_axac_i: f64,
        var_bgidl_i: f64,
        var_cfr_i: f64,
        var_cgidl_i: f64,
        var_cgov_i: f64,
        var_cgovaccg_i: f64,
        var_chnl_type: f64,
        var_cinr_i: f64,
        var_dphib_i: f64,
        var_eg: f64,
        var_epsrox_i: f64,
        var_epssi: f64,
        var_facneffac_i: f64,
        var_fcgovacc_i: f64,
        var_feta_i: f64,
        var_gc2ov_i: f64,
        var_gc3ov_i: f64,
        var_guard147: f64,
        var_igov_i: f64,
        var_inv_phita: f64,
        var_neff_i: f64,
        var_nov_i: f64,
        var_novd_i: f64,
        var_phibfac: f64,
        var_phit: f64,
        var_stbgidl_i: f64,
        var_tox_i: f64,
        var_toxov_i: f64,
        var_toxovd_i: f64,
        var_vp_i: f64,
        var_ar_slot: &mut f64,
        var_ar_rv_slot: &mut f64,
        var_arac_slot: &mut f64,
        var_arac_rv_slot: &mut f64,
        var_bgidld_i_slot: &mut f64,
        var_bgidld_i_rv_slot: &mut f64,
        var_cfrd_i_slot: &mut f64,
        var_cfrd_i_rv_slot: &mut f64,
        var_cgidld_i_slot: &mut f64,
        var_cgidld_i_rv_slot: &mut f64,
        var_cgovd_i_slot: &mut f64,
        var_cgovd_i_rv_slot: &mut f64,
        var_cinrd_i_slot: &mut f64,
        var_cinrd_i_rv_slot: &mut f64,
        var_cox_over_q_slot: &mut f64,
        var_cox_over_q_rv_slot: &mut f64,
        var_coxovprime_slot: &mut f64,
        var_coxovprime_d_slot: &mut f64,
        var_coxovprime_d_rv_slot: &mut f64,
        var_coxovprime_rv_slot: &mut f64,
        var_coxprime_slot: &mut f64,
        var_coxprime_rv_slot: &mut f64,
        var_dxgb_ov_d_slot: &mut f64,
        var_dxgb_ov_d_rv_slot: &mut f64,
        var_dxgb_ov_s_slot: &mut f64,
        var_dxgb_ov_s_rv_slot: &mut f64,
        var_dxgb_ov_th_slot: &mut f64,
        var_dxgb_ov_th_rv_slot: &mut f64,
        var_e_eff0_slot: &mut f64,
        var_e_eff0_rv_slot: &mut f64,
        var_epsox_slot: &mut f64,
        var_epsox_rv_slot: &mut f64,
        var_eta_mu_slot: &mut f64,
        var_eta_mu1_slot: &mut f64,
        var_eta_mu1_rv_slot: &mut f64,
        var_eta_mu_rv_slot: &mut f64,
        var_fcgovaccd_i_slot: &mut f64,
        var_fcgovaccd_i_rv_slot: &mut f64,
        var_gc2ovd_i_slot: &mut f64,
        var_gc2ovd_i_rv_slot: &mut f64,
        var_gc3ovd_i_slot: &mut f64,
        var_gc3ovd_i_rv_slot: &mut f64,
        var_gov2_d_slot: &mut f64,
        var_gov2_d_rv_slot: &mut f64,
        var_gov2_s_slot: &mut f64,
        var_gov2_s_rv_slot: &mut f64,
        var_gov_d_slot: &mut f64,
        var_gov_d_rv_slot: &mut f64,
        var_gov_s_slot: &mut f64,
        var_gov_s_rv_slot: &mut f64,
        var_guard148_slot: &mut f64,
        var_guard148_rv_slot: &mut f64,
        var_guard149_slot: &mut f64,
        var_guard149_rv_slot: &mut f64,
        var_guard150_slot: &mut f64,
        var_guard150_rv_slot: &mut f64,
        var_guard151_slot: &mut f64,
        var_guard151_rv_slot: &mut f64,
        var_guard152_slot: &mut f64,
        var_guard152_rv_slot: &mut f64,
        var_guard153_slot: &mut f64,
        var_guard153_rv_slot: &mut f64,
        var_guard154_slot: &mut f64,
        var_guard154_rv_slot: &mut f64,
        var_guard155_slot: &mut f64,
        var_guard155_rv_slot: &mut f64,
        var_guard156_slot: &mut f64,
        var_guard156_rv_slot: &mut f64,
        var_igovd_i_slot: &mut f64,
        var_igovd_i_rv_slot: &mut f64,
        var_inv_gov_slot: &mut f64,
        var_inv_gov_rv_slot: &mut f64,
        var_inv_vp_slot: &mut f64,
        var_inv_vp_rv_slot: &mut f64,
        var_neffac_i_slot: &mut f64,
        var_neffac_i_rv_slot: &mut f64,
        var_phib_dc_slot: &mut f64,
        var_phib_dc_rv_slot: &mut f64,
        var_qq_slot: &mut f64,
        var_qq_rv_slot: &mut f64,
        var_sp_ov_a_d_slot: &mut f64,
        var_sp_ov_a_d_rv_slot: &mut f64,
        var_sp_ov_a_s_slot: &mut f64,
        var_sp_ov_a_s_rv_slot: &mut f64,
        var_sp_ov_delta_slot: &mut f64,
        var_sp_ov_delta1_d_slot: &mut f64,
        var_sp_ov_delta1_d_rv_slot: &mut f64,
        var_sp_ov_delta1_s_slot: &mut f64,
        var_sp_ov_delta1_s_rv_slot: &mut f64,
        var_sp_ov_delta_rv_slot: &mut f64,
        var_sp_ov_eps_slot: &mut f64,
        var_sp_ov_eps2_d_slot: &mut f64,
        var_sp_ov_eps2_d_rv_slot: &mut f64,
        var_sp_ov_eps2_s_slot: &mut f64,
        var_sp_ov_eps2_s_rv_slot: &mut f64,
        var_sp_ov_eps_rv_slot: &mut f64,
        var_stbgidld_i_slot: &mut f64,
        var_stbgidld_i_rv_slot: &mut f64,
        var_temp_slot: &mut f64,
        var_temp_rv_slot: &mut f64,
        var_tox_sq_slot: &mut f64,
        var_tox_sq_rv_slot: &mut f64,
    ) {
        let mut var_ar: f64 = *var_ar_slot;
        let mut var_ar_rv: f64 = *var_ar_rv_slot;
        let mut var_arac: f64 = *var_arac_slot;
        let mut var_arac_rv: f64 = *var_arac_rv_slot;
        let mut var_bgidld_i: f64 = *var_bgidld_i_slot;
        let mut var_bgidld_i_rv: f64 = *var_bgidld_i_rv_slot;
        let mut var_cfrd_i: f64 = *var_cfrd_i_slot;
        let mut var_cfrd_i_rv: f64 = *var_cfrd_i_rv_slot;
        let mut var_cgidld_i: f64 = *var_cgidld_i_slot;
        let mut var_cgidld_i_rv: f64 = *var_cgidld_i_rv_slot;
        let mut var_cgovd_i: f64 = *var_cgovd_i_slot;
        let mut var_cgovd_i_rv: f64 = *var_cgovd_i_rv_slot;
        let mut var_cinrd_i: f64 = *var_cinrd_i_slot;
        let mut var_cinrd_i_rv: f64 = *var_cinrd_i_rv_slot;
        let mut var_cox_over_q: f64 = *var_cox_over_q_slot;
        let mut var_cox_over_q_rv: f64 = *var_cox_over_q_rv_slot;
        let mut var_coxovprime: f64 = *var_coxovprime_slot;
        let mut var_coxovprime_d: f64 = *var_coxovprime_d_slot;
        let mut var_coxovprime_d_rv: f64 = *var_coxovprime_d_rv_slot;
        let mut var_coxovprime_rv: f64 = *var_coxovprime_rv_slot;
        let mut var_coxprime: f64 = *var_coxprime_slot;
        let mut var_coxprime_rv: f64 = *var_coxprime_rv_slot;
        let mut var_dxgb_ov_d: f64 = *var_dxgb_ov_d_slot;
        let mut var_dxgb_ov_d_rv: f64 = *var_dxgb_ov_d_rv_slot;
        let mut var_dxgb_ov_s: f64 = *var_dxgb_ov_s_slot;
        let mut var_dxgb_ov_s_rv: f64 = *var_dxgb_ov_s_rv_slot;
        let mut var_dxgb_ov_th: f64 = *var_dxgb_ov_th_slot;
        let mut var_dxgb_ov_th_rv: f64 = *var_dxgb_ov_th_rv_slot;
        let mut var_e_eff0: f64 = *var_e_eff0_slot;
        let mut var_e_eff0_rv: f64 = *var_e_eff0_rv_slot;
        let mut var_epsox: f64 = *var_epsox_slot;
        let mut var_epsox_rv: f64 = *var_epsox_rv_slot;
        let mut var_eta_mu: f64 = *var_eta_mu_slot;
        let mut var_eta_mu1: f64 = *var_eta_mu1_slot;
        let mut var_eta_mu1_rv: f64 = *var_eta_mu1_rv_slot;
        let mut var_eta_mu_rv: f64 = *var_eta_mu_rv_slot;
        let mut var_fcgovaccd_i: f64 = *var_fcgovaccd_i_slot;
        let mut var_fcgovaccd_i_rv: f64 = *var_fcgovaccd_i_rv_slot;
        let mut var_gc2ovd_i: f64 = *var_gc2ovd_i_slot;
        let mut var_gc2ovd_i_rv: f64 = *var_gc2ovd_i_rv_slot;
        let mut var_gc3ovd_i: f64 = *var_gc3ovd_i_slot;
        let mut var_gc3ovd_i_rv: f64 = *var_gc3ovd_i_rv_slot;
        let mut var_gov2_d: f64 = *var_gov2_d_slot;
        let mut var_gov2_d_rv: f64 = *var_gov2_d_rv_slot;
        let mut var_gov2_s: f64 = *var_gov2_s_slot;
        let mut var_gov2_s_rv: f64 = *var_gov2_s_rv_slot;
        let mut var_gov_d: f64 = *var_gov_d_slot;
        let mut var_gov_d_rv: f64 = *var_gov_d_rv_slot;
        let mut var_gov_s: f64 = *var_gov_s_slot;
        let mut var_gov_s_rv: f64 = *var_gov_s_rv_slot;
        let mut var_guard148: f64 = *var_guard148_slot;
        let mut var_guard148_rv: f64 = *var_guard148_rv_slot;
        let mut var_guard149: f64 = *var_guard149_slot;
        let mut var_guard149_rv: f64 = *var_guard149_rv_slot;
        let mut var_guard150: f64 = *var_guard150_slot;
        let mut var_guard150_rv: f64 = *var_guard150_rv_slot;
        let mut var_guard151: f64 = *var_guard151_slot;
        let mut var_guard151_rv: f64 = *var_guard151_rv_slot;
        let mut var_guard152: f64 = *var_guard152_slot;
        let mut var_guard152_rv: f64 = *var_guard152_rv_slot;
        let mut var_guard153: f64 = *var_guard153_slot;
        let mut var_guard153_rv: f64 = *var_guard153_rv_slot;
        let mut var_guard154: f64 = *var_guard154_slot;
        let mut var_guard154_rv: f64 = *var_guard154_rv_slot;
        let mut var_guard155: f64 = *var_guard155_slot;
        let mut var_guard155_rv: f64 = *var_guard155_rv_slot;
        let mut var_guard156: f64 = *var_guard156_slot;
        let mut var_guard156_rv: f64 = *var_guard156_rv_slot;
        let mut var_igovd_i: f64 = *var_igovd_i_slot;
        let mut var_igovd_i_rv: f64 = *var_igovd_i_rv_slot;
        let mut var_inv_gov: f64 = *var_inv_gov_slot;
        let mut var_inv_gov_rv: f64 = *var_inv_gov_rv_slot;
        let mut var_inv_vp: f64 = *var_inv_vp_slot;
        let mut var_inv_vp_rv: f64 = *var_inv_vp_rv_slot;
        let mut var_neffac_i: f64 = *var_neffac_i_slot;
        let mut var_neffac_i_rv: f64 = *var_neffac_i_rv_slot;
        let mut var_phib_dc: f64 = *var_phib_dc_slot;
        let mut var_phib_dc_rv: f64 = *var_phib_dc_rv_slot;
        let mut var_qq: f64 = *var_qq_slot;
        let mut var_qq_rv: f64 = *var_qq_rv_slot;
        let mut var_sp_ov_a_d: f64 = *var_sp_ov_a_d_slot;
        let mut var_sp_ov_a_d_rv: f64 = *var_sp_ov_a_d_rv_slot;
        let mut var_sp_ov_a_s: f64 = *var_sp_ov_a_s_slot;
        let mut var_sp_ov_a_s_rv: f64 = *var_sp_ov_a_s_rv_slot;
        let mut var_sp_ov_delta: f64 = *var_sp_ov_delta_slot;
        let mut var_sp_ov_delta1_d: f64 = *var_sp_ov_delta1_d_slot;
        let mut var_sp_ov_delta1_d_rv: f64 = *var_sp_ov_delta1_d_rv_slot;
        let mut var_sp_ov_delta1_s: f64 = *var_sp_ov_delta1_s_slot;
        let mut var_sp_ov_delta1_s_rv: f64 = *var_sp_ov_delta1_s_rv_slot;
        let mut var_sp_ov_delta_rv: f64 = *var_sp_ov_delta_rv_slot;
        let mut var_sp_ov_eps: f64 = *var_sp_ov_eps_slot;
        let mut var_sp_ov_eps2_d: f64 = *var_sp_ov_eps2_d_slot;
        let mut var_sp_ov_eps2_d_rv: f64 = *var_sp_ov_eps2_d_rv_slot;
        let mut var_sp_ov_eps2_s: f64 = *var_sp_ov_eps2_s_slot;
        let mut var_sp_ov_eps2_s_rv: f64 = *var_sp_ov_eps2_s_rv_slot;
        let mut var_sp_ov_eps_rv: f64 = *var_sp_ov_eps_rv_slot;
        let mut var_stbgidld_i: f64 = *var_stbgidld_i_slot;
        let mut var_stbgidld_i_rv: f64 = *var_stbgidld_i_rv_slot;
        let mut var_temp: f64 = *var_temp_slot;
        let mut var_temp_rv: f64 = *var_temp_rv_slot;
        let mut var_tox_sq: f64 = *var_tox_sq_slot;
        let mut var_tox_sq_rv: f64 = *var_tox_sq_rv_slot;

        let (assign10940_e10118,) = {
    if (var_guard147 != 0.0) {
        (var_bgidl_i,)
    } else {
        (var_bgidld_i,)
    }
};
        var_bgidld_i = assign10940_e10118;
        var_bgidld_i_rv = 0.0;

        let (assign10950_e10122,) = {
    if (var_guard147 != 0.0) {
        (var_stbgidl_i,)
    } else {
        (var_stbgidld_i,)
    }
};
        var_stbgidld_i = assign10950_e10122;
        var_stbgidld_i_rv = 0.0;

        let (assign10960_e10126,) = {
    if (var_guard147 != 0.0) {
        (var_cgidl_i,)
    } else {
        (var_cgidld_i,)
    }
};
        var_cgidld_i = assign10960_e10126;
        var_cgidld_i_rv = 0.0;

        let (assign10970_e10130,) = {
    if (var_guard147 != 0.0) {
        (var_igov_i,)
    } else {
        (var_igovd_i,)
    }
};
        var_igovd_i = assign10970_e10130;
        var_igovd_i_rv = 0.0;

        let (assign10980_e10134,) = {
    if (var_guard147 != 0.0) {
        (var_gc2ov_i,)
    } else {
        (var_gc2ovd_i,)
    }
};
        var_gc2ovd_i = assign10980_e10134;
        var_gc2ovd_i_rv = 0.0;

        let (assign10990_e10138,) = {
    if (var_guard147 != 0.0) {
        (var_gc3ov_i,)
    } else {
        (var_gc3ovd_i,)
    }
};
        var_gc3ovd_i = assign10990_e10138;
        var_gc3ovd_i_rv = 0.0;

        let (assign11000_e10142,) = {
    if (var_guard147 != 0.0) {
        (var_cgov_i,)
    } else {
        (var_cgovd_i,)
    }
};
        var_cgovd_i = assign11000_e10142;
        var_cgovd_i_rv = 0.0;

        let (assign11010_e10146,) = {
    if (var_guard147 != 0.0) {
        (var_fcgovacc_i,)
    } else {
        (var_fcgovaccd_i,)
    }
};
        var_fcgovaccd_i = assign11010_e10146;
        var_fcgovaccd_i_rv = 0.0;

        let (assign11020_e10150,) = {
    if (var_guard147 != 0.0) {
        (var_cinr_i,)
    } else {
        (var_cinrd_i,)
    }
};
        var_cinrd_i = assign11020_e10150;
        var_cinrd_i_rv = 0.0;

        let (assign11030_e10154,) = {
    if (var_guard147 != 0.0) {
        (var_cfr_i,)
    } else {
        (var_cfrd_i,)
    }
};
        var_cfrd_i = assign11030_e10154;
        var_cfrd_i_rv = 0.0;

        let assign11040_e10157: f64 = (8.8541878176e-12 * var_epsrox_i);
        var_epsox = assign11040_e10157;
        var_epsox_rv = 0.0;

        let assign11050_e10160: f64 = (var_epsox / var_tox_i);
        var_coxprime = assign11050_e10160;
        var_coxprime_rv = 0.0;

        let assign11060_e10163: f64 = (var_tox_i * var_tox_i);
        var_tox_sq = assign11060_e10163;
        var_tox_sq_rv = 0.0;

        let assign11070_e10166: f64 = (var_coxprime / 1.6021918e-19);
        var_cox_over_q = assign11070_e10166;
        var_cox_over_q_rv = 0.0;

        let assign11080_e10169: f64 = (var_facneffac_i * var_neff_i);
        var_neffac_i = assign11080_e10169;
        var_neffac_i_rv = 0.0;

        let (assign11090_e10180,) = {
    if (var_neffac_i > 1e20) {
        let (assign11090_e10178,) = {
            if (var_neffac_i < 1e26) {
                (var_neffac_i,)
            } else {
                (1e26,)
            }
        };
        (assign11090_e10178,)
    } else {
        (1e20,)
    }
};
        var_neffac_i = assign11090_e10180;
        var_neffac_i_rv = 0.0;

        var_qq = 0.0;
        var_qq_rv = 0.0;

        let assign11110_e10184: f64 = if p.p51 > 0.0 { 1.0 } else { 0.0 };
        var_guard148 = assign11110_e10184;
        var_guard148_rv = 0.0;

        let (assign11120_e10196,) = {
    if (var_guard148 != 0.0) {
        let assign11120_e10188: f64 = (0.4 * 5.951993);
        let assign11120_e10190: f64 = (assign11120_e10188 * p.p51);
        let assign11120_e10193: f64 = (var_coxprime).powf(0.6666666666666666);
        let assign11120_e10194: f64 = (assign11120_e10190 * assign11120_e10193);
        (assign11120_e10194,)
    } else {
        (var_qq,)
    }
};
        var_qq = assign11120_e10196;
        var_qq_rv = 0.0;

        let assign11130_e10199: f64 = (-1.0);
        let assign11130_e10200: f64 = if var_chnl_type == assign11130_e10199 { 1.0 } else { 0.0 };
        var_guard149 = assign11130_e10200;
        var_guard149_rv = 0.0;

        let (assign11140_e10210,) = {
    if ((var_guard148 != 0.0) && (var_guard149 != 0.0)) {
        let assign11140_e10206: f64 = (7.448711 / 5.951993);
        let assign11140_e10208: f64 = (assign11140_e10206 * var_qq);
        (assign11140_e10208,)
    } else {
        (var_qq,)
    }
};
        var_qq = assign11140_e10210;
        var_qq_rv = 0.0;

        let assign11150_e10213: f64 = (1e-8 * var_coxprime);
        let assign11150_e10215: f64 = (assign11150_e10213 / var_epssi);
        var_e_eff0 = assign11150_e10215;
        var_e_eff0_rv = 0.0;

        let assign11160_e10218: f64 = (0.5 * var_feta_i);
        var_eta_mu = assign11160_e10218;
        var_eta_mu_rv = 0.0;

        var_eta_mu1 = 0.5;
        var_eta_mu1_rv = 0.0;

        let assign11180_e10222: f64 = (-1.0);
        let assign11180_e10223: f64 = if var_chnl_type == assign11180_e10222 { 1.0 } else { 0.0 };
        var_guard150 = assign11180_e10223;
        var_guard150_rv = 0.0;

        let (assign11190_e10229,) = {
    if (var_guard150 != 0.0) {
        let assign11190_e10227: f64 = (0.3333333333333333 * var_feta_i);
        (assign11190_e10227,)
    } else {
        (var_eta_mu,)
    }
};
        var_eta_mu = assign11190_e10229;
        var_eta_mu_rv = 0.0;

        let (assign11200_e10233,) = {
    if (var_guard150 != 0.0) {
        (0.3333333333333333,)
    } else {
        (var_eta_mu1,)
    }
};
        var_eta_mu1 = assign11200_e10233;
        var_eta_mu1_rv = 0.0;

        let assign11210_e10236: f64 = (-2.0);
        let assign11210_e10238: f64 = (assign11210_e10236 / var_ax_i);
        let assign11210_e10240: f64 = (assign11210_e10238 + 1.0);
        let assign11210_e10241: f64 = (2.0_f64).powf(assign11210_e10240);
        let assign11210_e10243: f64 = (assign11210_e10241 - 1.0);
        var_temp = assign11210_e10243;
        var_temp_rv = 0.0;

        let assign11220_e10246: f64 = (var_temp - 1.0);
        let assign11220_e10249: f64 = (var_temp - 1.0);
        let assign11220_e10250: f64 = (assign11220_e10246 * assign11220_e10249);
        let assign11220_e10253: f64 = (4.0 * var_temp);
        let (assign11220_e10260,) = {
    if (assign11220_e10253 > 0.0001) {
        let assign11220_e10258: f64 = (4.0 * var_temp);
        (assign11220_e10258,)
    } else {
        (0.0001,)
    }
};
        let assign11220_e10261: f64 = (assign11220_e10250 / assign11220_e10260);
        var_ar = assign11220_e10261;
        var_ar_rv = 0.0;

        let assign11230_e10264: f64 = (-2.0);
        let assign11230_e10266: f64 = (assign11230_e10264 / var_axac_i);
        let assign11230_e10268: f64 = (assign11230_e10266 + 1.0);
        let assign11230_e10269: f64 = (2.0_f64).powf(assign11230_e10268);
        let assign11230_e10271: f64 = (assign11230_e10269 - 1.0);
        var_temp = assign11230_e10271;
        var_temp_rv = 0.0;

        let assign11240_e10274: f64 = (var_temp - 1.0);
        let assign11240_e10277: f64 = (var_temp - 1.0);
        let assign11240_e10278: f64 = (assign11240_e10274 * assign11240_e10277);
        let assign11240_e10281: f64 = (4.0 * var_temp);
        let (assign11240_e10288,) = {
    if (assign11240_e10281 > 0.0001) {
        let assign11240_e10286: f64 = (4.0 * var_temp);
        (assign11240_e10286,)
    } else {
        (0.0001,)
    }
};
        let assign11240_e10289: f64 = (assign11240_e10278 / assign11240_e10288);
        var_arac = assign11240_e10289;
        var_arac_rv = 0.0;

        let assign11250_e10292: f64 = (1.0 / var_vp_i);
        var_inv_vp = assign11250_e10292;
        var_inv_vp_rv = 0.0;

        let assign11260_e10295: f64 = (var_epsox / var_toxov_i);
        var_coxovprime = assign11260_e10295;
        var_coxovprime_rv = 0.0;

        let assign11270_e10298: f64 = (var_epsox / var_toxovd_i);
        var_coxovprime_d = assign11270_e10298;
        var_coxovprime_d_rv = 0.0;

        let assign11280_e10301: f64 = (2.0 * 1.6021918e-19);
        let assign11280_e10303: f64 = (assign11280_e10301 * var_nov_i);
        let assign11280_e10305: f64 = (assign11280_e10303 * var_epssi);
        let assign11280_e10307: f64 = (assign11280_e10305 * var_inv_phita);
        let assign11280_e10308: f64 = (assign11280_e10307).sqrt();
        let assign11280_e10310: f64 = (assign11280_e10308 / var_coxovprime);
        var_gov_s = assign11280_e10310;
        var_gov_s_rv = 0.0;

        let assign11290_e10313: f64 = (2.0 * 1.6021918e-19);
        let assign11290_e10315: f64 = (assign11290_e10313 * var_novd_i);
        let assign11290_e10317: f64 = (assign11290_e10315 * var_epssi);
        let assign11290_e10319: f64 = (assign11290_e10317 * var_inv_phita);
        let assign11290_e10320: f64 = (assign11290_e10319).sqrt();
        let assign11290_e10322: f64 = (assign11290_e10320 / var_coxovprime_d);
        var_gov_d = assign11290_e10322;
        var_gov_d_rv = 0.0;

        let assign11300_e10325: f64 = (var_gov_s * var_gov_s);
        var_gov2_s = assign11300_e10325;
        var_gov2_s_rv = 0.0;

        let assign11310_e10328: f64 = (var_gov_d * var_gov_d);
        var_gov2_d = assign11310_e10328;
        var_gov2_d_rv = 0.0;

        let assign11320_e10331: f64 = (var_cgovaccg_i * 0.005);
        let assign11320_e10333: f64 = (assign11320_e10331 * var_inv_phita);
        let assign11320_e10334: f64 = (assign11320_e10333).exp();
        let assign11320_e10336: f64 = (assign11320_e10334 - 1.0);
        let assign11320_e10337: f64 = (assign11320_e10336).ln();
        let assign11320_e10339: f64 = (assign11320_e10337 / var_cgovaccg_i);
        let assign11320_e10342: f64 = (0.005 * var_inv_phita);
        let assign11320_e10343: f64 = (assign11320_e10342).exp();
        let assign11320_e10345: f64 = (assign11320_e10343 - 1.0);
        let assign11320_e10346: f64 = (assign11320_e10345).ln();
        let assign11320_e10347: f64 = (assign11320_e10339 - assign11320_e10346);
        var_dxgb_ov_th = assign11320_e10347;
        var_dxgb_ov_th_rv = 0.0;

        let assign11330_e10350: f64 = (0.5 * var_gov_s);
        let assign11330_e10351: f64 = (assign11330_e10350).ln();
        let assign11330_e10353: f64 = (assign11330_e10351 + var_dxgb_ov_th);
        var_dxgb_ov_s = assign11330_e10353;
        var_dxgb_ov_s_rv = 0.0;

        let assign11340_e10356: f64 = (0.5 * var_gov_d);
        let assign11340_e10357: f64 = (assign11340_e10356).ln();
        let assign11340_e10359: f64 = (assign11340_e10357 + var_dxgb_ov_th);
        var_dxgb_ov_d = assign11340_e10359;
        var_dxgb_ov_d_rv = 0.0;

        let assign11350_e10362: f64 = (1.0 / var_gov_s);
        var_inv_gov = assign11350_e10362;
        var_inv_gov_rv = 0.0;

        let assign11360_e10365: f64 = (3.1 * var_gov_s);
        let assign11360_e10367: f64 = (assign11360_e10365 + 8.5);
        var_sp_ov_eps = assign11360_e10367;
        var_sp_ov_eps_rv = 0.0;

        let assign11370_e10370: f64 = (var_sp_ov_eps * var_sp_ov_eps);
        var_sp_ov_eps2_s = assign11370_e10370;
        var_sp_ov_eps2_s_rv = 0.0;

        let assign11380_e10373: f64 = (0.5 * var_sp_ov_eps);
        var_sp_ov_delta = assign11380_e10373;
        var_sp_ov_delta_rv = 0.0;

        let assign11390_e10376: f64 = if var_inv_gov < 0.06 { 1.0 } else { 0.0 };
        var_guard151 = assign11390_e10376;
        var_guard151_rv = 0.0;

        let (assign11400_e10382,) = {
    if (var_guard151 != 0.0) {
        let assign11400_e10380: f64 = (64.0 * var_inv_gov);
        (assign11400_e10380,)
    } else {
        (var_sp_ov_a_s,)
    }
};
        var_sp_ov_a_s = assign11400_e10382;
        var_sp_ov_a_s_rv = 0.0;

        let assign11410_e10385: f64 = if var_inv_gov <= 0.45 { 1.0 } else { 0.0 };
        var_guard152 = assign11410_e10385;
        var_guard152_rv = 0.0;

        let (assign11420_e10396,) = {
    if ((var_guard151 == 0.0) && (var_guard152 != 0.0)) {
        let assign11420_e10392: f64 = (22.0 * var_inv_gov);
        let assign11420_e10394: f64 = (assign11420_e10392 + 3.0);
        (assign11420_e10394,)
    } else {
        (var_sp_ov_a_s,)
    }
};
        var_sp_ov_a_s = assign11420_e10396;
        var_sp_ov_a_s_rv = 0.0;

        let assign11430_e10399: f64 = if var_inv_gov <= 1.6 { 1.0 } else { 0.0 };
        var_guard153 = assign11430_e10399;
        var_guard153_rv = 0.0;

        let (assign11440_e10414,) = {
    if (((var_guard151 == 0.0) && (var_guard152 == 0.0)) && (var_guard153 != 0.0)) {
        let assign11440_e10408: f64 = (-7.2);
        let assign11440_e10410: f64 = (assign11440_e10408 * var_inv_gov);
        let assign11440_e10412: f64 = (assign11440_e10410 + 15.5);
        (assign11440_e10412,)
    } else {
        (var_sp_ov_a_s,)
    }
};
        var_sp_ov_a_s = assign11440_e10414;
        var_sp_ov_a_s_rv = 0.0;

        let (assign11450_e10425,) = {
    if (((var_guard151 == 0.0) && (var_guard152 == 0.0)) && (var_guard153 == 0.0)) {
        (var_gov_s,)
    } else {
        (var_sp_ov_a_s,)
    }
};
        var_sp_ov_a_s = assign11450_e10425;
        var_sp_ov_a_s_rv = 0.0;

        let assign11460_e10429: f64 = (var_gov2_s * 0.5);
        let assign11460_e10430: f64 = (var_sp_ov_delta + assign11460_e10429);
        let assign11460_e10435: f64 = (var_gov2_s * 0.25);
        let assign11460_e10436: f64 = (var_sp_ov_delta + assign11460_e10435);
        let assign11460_e10438: f64 = (assign11460_e10436 + var_sp_ov_a_s);
        let assign11460_e10439: f64 = (assign11460_e10438).sqrt();
        let assign11460_e10440: f64 = (var_gov_s * assign11460_e10439);
        let assign11460_e10441: f64 = (assign11460_e10430 - assign11460_e10440);
        var_sp_ov_delta1_s = assign11460_e10441;
        var_sp_ov_delta1_s_rv = 0.0;

        let assign11470_e10444: f64 = (1.0 / var_gov_d);
        var_inv_gov = assign11470_e10444;
        var_inv_gov_rv = 0.0;

        let assign11480_e10447: f64 = (3.1 * var_gov_d);
        let assign11480_e10449: f64 = (assign11480_e10447 + 8.5);
        var_sp_ov_eps = assign11480_e10449;
        var_sp_ov_eps_rv = 0.0;

        let assign11490_e10452: f64 = (var_sp_ov_eps * var_sp_ov_eps);
        var_sp_ov_eps2_d = assign11490_e10452;
        var_sp_ov_eps2_d_rv = 0.0;

        let assign11500_e10455: f64 = (0.5 * var_sp_ov_eps);
        var_sp_ov_delta = assign11500_e10455;
        var_sp_ov_delta_rv = 0.0;

        let assign11510_e10458: f64 = if var_inv_gov < 0.06 { 1.0 } else { 0.0 };
        var_guard154 = assign11510_e10458;
        var_guard154_rv = 0.0;

        let (assign11520_e10464,) = {
    if (var_guard154 != 0.0) {
        let assign11520_e10462: f64 = (64.0 * var_inv_gov);
        (assign11520_e10462,)
    } else {
        (var_sp_ov_a_d,)
    }
};
        var_sp_ov_a_d = assign11520_e10464;
        var_sp_ov_a_d_rv = 0.0;

        let assign11530_e10467: f64 = if var_inv_gov <= 0.45 { 1.0 } else { 0.0 };
        var_guard155 = assign11530_e10467;
        var_guard155_rv = 0.0;

        let (assign11540_e10478,) = {
    if ((var_guard154 == 0.0) && (var_guard155 != 0.0)) {
        let assign11540_e10474: f64 = (22.0 * var_inv_gov);
        let assign11540_e10476: f64 = (assign11540_e10474 + 3.0);
        (assign11540_e10476,)
    } else {
        (var_sp_ov_a_d,)
    }
};
        var_sp_ov_a_d = assign11540_e10478;
        var_sp_ov_a_d_rv = 0.0;

        let assign11550_e10481: f64 = if var_inv_gov <= 1.6 { 1.0 } else { 0.0 };
        var_guard156 = assign11550_e10481;
        var_guard156_rv = 0.0;

        let (assign11560_e10496,) = {
    if (((var_guard154 == 0.0) && (var_guard155 == 0.0)) && (var_guard156 != 0.0)) {
        let assign11560_e10490: f64 = (-7.2);
        let assign11560_e10492: f64 = (assign11560_e10490 * var_inv_gov);
        let assign11560_e10494: f64 = (assign11560_e10492 + 15.5);
        (assign11560_e10494,)
    } else {
        (var_sp_ov_a_d,)
    }
};
        var_sp_ov_a_d = assign11560_e10496;
        var_sp_ov_a_d_rv = 0.0;

        let (assign11570_e10507,) = {
    if (((var_guard154 == 0.0) && (var_guard155 == 0.0)) && (var_guard156 == 0.0)) {
        (var_gov_d,)
    } else {
        (var_sp_ov_a_d,)
    }
};
        var_sp_ov_a_d = assign11570_e10507;
        var_sp_ov_a_d_rv = 0.0;

        let assign11580_e10511: f64 = (var_gov2_d * 0.5);
        let assign11580_e10512: f64 = (var_sp_ov_delta + assign11580_e10511);
        let assign11580_e10517: f64 = (var_gov2_d * 0.25);
        let assign11580_e10518: f64 = (var_sp_ov_delta + assign11580_e10517);
        let assign11580_e10520: f64 = (assign11580_e10518 + var_sp_ov_a_d);
        let assign11580_e10521: f64 = (assign11580_e10520).sqrt();
        let assign11580_e10522: f64 = (var_gov_d * assign11580_e10521);
        let assign11580_e10523: f64 = (assign11580_e10512 - assign11580_e10522);
        var_sp_ov_delta1_d = assign11580_e10523;
        var_sp_ov_delta1_d_rv = 0.0;

        let assign11590_e10526: f64 = (var_eg + var_dphib_i);
        let assign11590_e10529: f64 = (2.0 * var_phit);
        let assign11590_e10533: f64 = (-0.75);
        let assign11590_e10534: f64 = (var_phibfac).powf(assign11590_e10533);
        let assign11590_e10535: f64 = (var_neff_i * assign11590_e10534);
        let assign11590_e10537: f64 = (assign11590_e10535 * 4e-26);
        let assign11590_e10538: f64 = (assign11590_e10537).ln();
        let assign11590_e10539: f64 = (assign11590_e10529 * assign11590_e10538);
        let assign11590_e10540: f64 = (assign11590_e10526 + assign11590_e10539);
        var_phib_dc = assign11590_e10540;
        var_phib_dc_rv = 0.0;

        *var_ar_slot = var_ar;
        *var_ar_rv_slot = var_ar_rv;
        *var_arac_slot = var_arac;
        *var_arac_rv_slot = var_arac_rv;
        *var_bgidld_i_slot = var_bgidld_i;
        *var_bgidld_i_rv_slot = var_bgidld_i_rv;
        *var_cfrd_i_slot = var_cfrd_i;
        *var_cfrd_i_rv_slot = var_cfrd_i_rv;
        *var_cgidld_i_slot = var_cgidld_i;
        *var_cgidld_i_rv_slot = var_cgidld_i_rv;
        *var_cgovd_i_slot = var_cgovd_i;
        *var_cgovd_i_rv_slot = var_cgovd_i_rv;
        *var_cinrd_i_slot = var_cinrd_i;
        *var_cinrd_i_rv_slot = var_cinrd_i_rv;
        *var_cox_over_q_slot = var_cox_over_q;
        *var_cox_over_q_rv_slot = var_cox_over_q_rv;
        *var_coxovprime_slot = var_coxovprime;
        *var_coxovprime_d_slot = var_coxovprime_d;
        *var_coxovprime_d_rv_slot = var_coxovprime_d_rv;
        *var_coxovprime_rv_slot = var_coxovprime_rv;
        *var_coxprime_slot = var_coxprime;
        *var_coxprime_rv_slot = var_coxprime_rv;
        *var_dxgb_ov_d_slot = var_dxgb_ov_d;
        *var_dxgb_ov_d_rv_slot = var_dxgb_ov_d_rv;
        *var_dxgb_ov_s_slot = var_dxgb_ov_s;
        *var_dxgb_ov_s_rv_slot = var_dxgb_ov_s_rv;
        *var_dxgb_ov_th_slot = var_dxgb_ov_th;
        *var_dxgb_ov_th_rv_slot = var_dxgb_ov_th_rv;
        *var_e_eff0_slot = var_e_eff0;
        *var_e_eff0_rv_slot = var_e_eff0_rv;
        *var_epsox_slot = var_epsox;
        *var_epsox_rv_slot = var_epsox_rv;
        *var_eta_mu_slot = var_eta_mu;
        *var_eta_mu1_slot = var_eta_mu1;
        *var_eta_mu1_rv_slot = var_eta_mu1_rv;
        *var_eta_mu_rv_slot = var_eta_mu_rv;
        *var_fcgovaccd_i_slot = var_fcgovaccd_i;
        *var_fcgovaccd_i_rv_slot = var_fcgovaccd_i_rv;
        *var_gc2ovd_i_slot = var_gc2ovd_i;
        *var_gc2ovd_i_rv_slot = var_gc2ovd_i_rv;
        *var_gc3ovd_i_slot = var_gc3ovd_i;
        *var_gc3ovd_i_rv_slot = var_gc3ovd_i_rv;
        *var_gov2_d_slot = var_gov2_d;
        *var_gov2_d_rv_slot = var_gov2_d_rv;
        *var_gov2_s_slot = var_gov2_s;
        *var_gov2_s_rv_slot = var_gov2_s_rv;
        *var_gov_d_slot = var_gov_d;
        *var_gov_d_rv_slot = var_gov_d_rv;
        *var_gov_s_slot = var_gov_s;
        *var_gov_s_rv_slot = var_gov_s_rv;
        *var_guard148_slot = var_guard148;
        *var_guard148_rv_slot = var_guard148_rv;
        *var_guard149_slot = var_guard149;
        *var_guard149_rv_slot = var_guard149_rv;
        *var_guard150_slot = var_guard150;
        *var_guard150_rv_slot = var_guard150_rv;
        *var_guard151_slot = var_guard151;
        *var_guard151_rv_slot = var_guard151_rv;
        *var_guard152_slot = var_guard152;
        *var_guard152_rv_slot = var_guard152_rv;
        *var_guard153_slot = var_guard153;
        *var_guard153_rv_slot = var_guard153_rv;
        *var_guard154_slot = var_guard154;
        *var_guard154_rv_slot = var_guard154_rv;
        *var_guard155_slot = var_guard155;
        *var_guard155_rv_slot = var_guard155_rv;
        *var_guard156_slot = var_guard156;
        *var_guard156_rv_slot = var_guard156_rv;
        *var_igovd_i_slot = var_igovd_i;
        *var_igovd_i_rv_slot = var_igovd_i_rv;
        *var_inv_gov_slot = var_inv_gov;
        *var_inv_gov_rv_slot = var_inv_gov_rv;
        *var_inv_vp_slot = var_inv_vp;
        *var_inv_vp_rv_slot = var_inv_vp_rv;
        *var_neffac_i_slot = var_neffac_i;
        *var_neffac_i_rv_slot = var_neffac_i_rv;
        *var_phib_dc_slot = var_phib_dc;
        *var_phib_dc_rv_slot = var_phib_dc_rv;
        *var_qq_slot = var_qq;
        *var_qq_rv_slot = var_qq_rv;
        *var_sp_ov_a_d_slot = var_sp_ov_a_d;
        *var_sp_ov_a_d_rv_slot = var_sp_ov_a_d_rv;
        *var_sp_ov_a_s_slot = var_sp_ov_a_s;
        *var_sp_ov_a_s_rv_slot = var_sp_ov_a_s_rv;
        *var_sp_ov_delta_slot = var_sp_ov_delta;
        *var_sp_ov_delta1_d_slot = var_sp_ov_delta1_d;
        *var_sp_ov_delta1_d_rv_slot = var_sp_ov_delta1_d_rv;
        *var_sp_ov_delta1_s_slot = var_sp_ov_delta1_s;
        *var_sp_ov_delta1_s_rv_slot = var_sp_ov_delta1_s_rv;
        *var_sp_ov_delta_rv_slot = var_sp_ov_delta_rv;
        *var_sp_ov_eps_slot = var_sp_ov_eps;
        *var_sp_ov_eps2_d_slot = var_sp_ov_eps2_d;
        *var_sp_ov_eps2_d_rv_slot = var_sp_ov_eps2_d_rv;
        *var_sp_ov_eps2_s_slot = var_sp_ov_eps2_s;
        *var_sp_ov_eps2_s_rv_slot = var_sp_ov_eps2_s_rv;
        *var_sp_ov_eps_rv_slot = var_sp_ov_eps_rv;
        *var_stbgidld_i_slot = var_stbgidld_i;
        *var_stbgidld_i_rv_slot = var_stbgidld_i_rv;
        *var_temp_slot = var_temp;
        *var_temp_rv_slot = var_temp_rv;
        *var_tox_sq_slot = var_tox_sq;
        *var_tox_sq_rv_slot = var_tox_sq_rv;
    }

    pub(super) fn stamp_reactive_block_18(
        p: &Parameters,
        var_a2_i: f64,
        var_betn_i: f64,
        var_betnedge_i: f64,
        var_coxprime: f64,
        var_cs_i: f64,
        var_ct_i: f64,
        var_ctedge_i: f64,
        var_ctg_i: f64,
        var_delt: f64,
        var_delvtac_i: f64,
        var_delvto_i: f64,
        var_delvtoedge_i: f64,
        var_dphib_i: f64,
        var_dvsbnud_i: f64,
        var_eg: f64,
        var_epssi: f64,
        var_factuo_i: f64,
        var_factuoedge_i: f64,
        var_fnt_i: f64,
        var_inv_phit: f64,
        var_ln_rtn: f64,
        var_mue_i: f64,
        var_neff_i: f64,
        var_neffac_i: f64,
        var_np_i: f64,
        var_phibfac: f64,
        var_phit: f64,
        var_qq: f64,
        var_rs_i: f64,
        var_rtn: f64,
        var_st2vfb_i: f64,
        var_sta2_i: f64,
        var_stbet_i: f64,
        var_stbetedge_i: f64,
        var_stcs_i: f64,
        var_stct_i: f64,
        var_stmue_i: f64,
        var_strs_i: f64,
        var_stthecs_i: f64,
        var_stthemu_i: f64,
        var_stthesat_i: f64,
        var_stvfb_i: f64,
        var_stvfbedge_i: f64,
        var_stxcor_i: f64,
        var_thecs_i: f64,
        var_themu_i: f64,
        var_thesat_i: f64,
        var_thesatac_i: f64,
        var_tkd: f64,
        var_tox_sq: f64,
        var_vfb_i: f64,
        var_vfbedge_i: f64,
        var_vsbnud_i: f64,
        var_xcor_i: f64,
        var_a2_t_slot: &mut f64,
        var_a2_t_rv_slot: &mut f64,
        var_alpha_b_slot: &mut f64,
        var_alpha_b_rv_slot: &mut f64,
        var_aphi_ac_slot: &mut f64,
        var_aphi_ac_rv_slot: &mut f64,
        var_aphi_dc_slot: &mut f64,
        var_aphi_dc_rv_slot: &mut f64,
        var_arg2max_slot: &mut f64,
        var_arg2max_rv_slot: &mut f64,
        var_bet_i_slot: &mut f64,
        var_bet_i_rv_slot: &mut f64,
        var_betedge_i_slot: &mut f64,
        var_betedge_i_rv_slot: &mut f64,
        var_betn_t_slot: &mut f64,
        var_betn_t_rv_slot: &mut f64,
        var_betnedge_t_slot: &mut f64,
        var_betnedge_t_rv_slot: &mut f64,
        var_bphi_ac_slot: &mut f64,
        var_bphi_ac_rv_slot: &mut f64,
        var_bphi_dc_slot: &mut f64,
        var_bphi_dc_rv_slot: &mut f64,
        var_cs_t_slot: &mut f64,
        var_cs_t_rv_slot: &mut f64,
        var_ct_t_slot: &mut f64,
        var_ct_t_rv_slot: &mut f64,
        var_ctg_t_slot: &mut f64,
        var_ctg_t_rv_slot: &mut f64,
        var_dphibq_slot: &mut f64,
        var_dphibq_rv_slot: &mut f64,
        var_g_0_ac_slot: &mut f64,
        var_g_0_ac_rv_slot: &mut f64,
        var_g_0_dc_slot: &mut f64,
        var_g_0_dc_rv_slot: &mut f64,
        var_guard157_slot: &mut f64,
        var_guard157_rv_slot: &mut f64,
        var_guard158_slot: &mut f64,
        var_guard158_rv_slot: &mut f64,
        var_guard159_slot: &mut f64,
        var_guard159_rv_slot: &mut f64,
        var_guard160_slot: &mut f64,
        var_guard160_rv_slot: &mut f64,
        var_kp_slot: &mut f64,
        var_kp_rv_slot: &mut f64,
        var_mue_t_slot: &mut f64,
        var_mue_t_rv_slot: &mut f64,
        var_np_slot: &mut f64,
        var_np_rv_slot: &mut f64,
        var_nt_slot: &mut f64,
        var_nt_rv_slot: &mut f64,
        var_phib_ac_slot: &mut f64,
        var_phib_ac_rv_slot: &mut f64,
        var_phib_dc_slot: &mut f64,
        var_phib_dc_rv_slot: &mut f64,
        var_phit0edge_slot: &mut f64,
        var_phit0edge_rv_slot: &mut f64,
        var_phix1_ac_slot: &mut f64,
        var_phix1_ac_rv_slot: &mut f64,
        var_phix1_dc_slot: &mut f64,
        var_phix1_dc_rv_slot: &mut f64,
        var_phix2_slot: &mut f64,
        var_phix2_rv_slot: &mut f64,
        var_phix_ac_slot: &mut f64,
        var_phix_ac_rv_slot: &mut f64,
        var_phix_dc_slot: &mut f64,
        var_phix_dc_rv_slot: &mut f64,
        var_qb0_slot: &mut f64,
        var_qb0_rv_slot: &mut f64,
        var_qlim2_slot: &mut f64,
        var_qlim2_rv_slot: &mut f64,
        var_rs_t_slot: &mut f64,
        var_rs_t_rv_slot: &mut f64,
        var_sqrt_phib_dc_slot: &mut f64,
        var_sqrt_phib_dc_rv_slot: &mut f64,
        var_tf_bet_slot: &mut f64,
        var_tf_bet_rv_slot: &mut f64,
        var_tf_betedge_slot: &mut f64,
        var_tf_betedge_rv_slot: &mut f64,
        var_tf_cs_slot: &mut f64,
        var_tf_cs_rv_slot: &mut f64,
        var_tf_ct_slot: &mut f64,
        var_tf_ct_rv_slot: &mut f64,
        var_tf_mue_slot: &mut f64,
        var_tf_mue_rv_slot: &mut f64,
        var_tf_ther_slot: &mut f64,
        var_tf_ther_rv_slot: &mut f64,
        var_tf_thesat_slot: &mut f64,
        var_tf_thesat_rv_slot: &mut f64,
        var_tf_xcor_slot: &mut f64,
        var_tf_xcor_rv_slot: &mut f64,
        var_thecs_t_slot: &mut f64,
        var_thecs_t_rv_slot: &mut f64,
        var_themu_t_slot: &mut f64,
        var_themu_t_rv_slot: &mut f64,
        var_ther_i_slot: &mut f64,
        var_ther_i_rv_slot: &mut f64,
        var_thesat_t_slot: &mut f64,
        var_thesat_t_rv_slot: &mut f64,
        var_thesatac_t_slot: &mut f64,
        var_thesatac_t_rv_slot: &mut f64,
        var_us1_slot: &mut f64,
        var_us1_rv_slot: &mut f64,
        var_us21_slot: &mut f64,
        var_us21_rv_slot: &mut f64,
        var_vfb_t_slot: &mut f64,
        var_vfb_t_rv_slot: &mut f64,
        var_vfbedge_t_slot: &mut f64,
        var_vfbedge_t_rv_slot: &mut f64,
        var_xcor_t_slot: &mut f64,
        var_xcor_t_rv_slot: &mut f64,
    ) {
        let mut var_a2_t: f64 = *var_a2_t_slot;
        let mut var_a2_t_rv: f64 = *var_a2_t_rv_slot;
        let mut var_alpha_b: f64 = *var_alpha_b_slot;
        let mut var_alpha_b_rv: f64 = *var_alpha_b_rv_slot;
        let mut var_aphi_ac: f64 = *var_aphi_ac_slot;
        let mut var_aphi_ac_rv: f64 = *var_aphi_ac_rv_slot;
        let mut var_aphi_dc: f64 = *var_aphi_dc_slot;
        let mut var_aphi_dc_rv: f64 = *var_aphi_dc_rv_slot;
        let mut var_arg2max: f64 = *var_arg2max_slot;
        let mut var_arg2max_rv: f64 = *var_arg2max_rv_slot;
        let mut var_bet_i: f64 = *var_bet_i_slot;
        let mut var_bet_i_rv: f64 = *var_bet_i_rv_slot;
        let mut var_betedge_i: f64 = *var_betedge_i_slot;
        let mut var_betedge_i_rv: f64 = *var_betedge_i_rv_slot;
        let mut var_betn_t: f64 = *var_betn_t_slot;
        let mut var_betn_t_rv: f64 = *var_betn_t_rv_slot;
        let mut var_betnedge_t: f64 = *var_betnedge_t_slot;
        let mut var_betnedge_t_rv: f64 = *var_betnedge_t_rv_slot;
        let mut var_bphi_ac: f64 = *var_bphi_ac_slot;
        let mut var_bphi_ac_rv: f64 = *var_bphi_ac_rv_slot;
        let mut var_bphi_dc: f64 = *var_bphi_dc_slot;
        let mut var_bphi_dc_rv: f64 = *var_bphi_dc_rv_slot;
        let mut var_cs_t: f64 = *var_cs_t_slot;
        let mut var_cs_t_rv: f64 = *var_cs_t_rv_slot;
        let mut var_ct_t: f64 = *var_ct_t_slot;
        let mut var_ct_t_rv: f64 = *var_ct_t_rv_slot;
        let mut var_ctg_t: f64 = *var_ctg_t_slot;
        let mut var_ctg_t_rv: f64 = *var_ctg_t_rv_slot;
        let mut var_dphibq: f64 = *var_dphibq_slot;
        let mut var_dphibq_rv: f64 = *var_dphibq_rv_slot;
        let mut var_g_0_ac: f64 = *var_g_0_ac_slot;
        let mut var_g_0_ac_rv: f64 = *var_g_0_ac_rv_slot;
        let mut var_g_0_dc: f64 = *var_g_0_dc_slot;
        let mut var_g_0_dc_rv: f64 = *var_g_0_dc_rv_slot;
        let mut var_guard157: f64 = *var_guard157_slot;
        let mut var_guard157_rv: f64 = *var_guard157_rv_slot;
        let mut var_guard158: f64 = *var_guard158_slot;
        let mut var_guard158_rv: f64 = *var_guard158_rv_slot;
        let mut var_guard159: f64 = *var_guard159_slot;
        let mut var_guard159_rv: f64 = *var_guard159_rv_slot;
        let mut var_guard160: f64 = *var_guard160_slot;
        let mut var_guard160_rv: f64 = *var_guard160_rv_slot;
        let mut var_kp: f64 = *var_kp_slot;
        let mut var_kp_rv: f64 = *var_kp_rv_slot;
        let mut var_mue_t: f64 = *var_mue_t_slot;
        let mut var_mue_t_rv: f64 = *var_mue_t_rv_slot;
        let mut var_np: f64 = *var_np_slot;
        let mut var_np_rv: f64 = *var_np_rv_slot;
        let mut var_nt: f64 = *var_nt_slot;
        let mut var_nt_rv: f64 = *var_nt_rv_slot;
        let mut var_phib_ac: f64 = *var_phib_ac_slot;
        let mut var_phib_ac_rv: f64 = *var_phib_ac_rv_slot;
        let mut var_phib_dc: f64 = *var_phib_dc_slot;
        let mut var_phib_dc_rv: f64 = *var_phib_dc_rv_slot;
        let mut var_phit0edge: f64 = *var_phit0edge_slot;
        let mut var_phit0edge_rv: f64 = *var_phit0edge_rv_slot;
        let mut var_phix1_ac: f64 = *var_phix1_ac_slot;
        let mut var_phix1_ac_rv: f64 = *var_phix1_ac_rv_slot;
        let mut var_phix1_dc: f64 = *var_phix1_dc_slot;
        let mut var_phix1_dc_rv: f64 = *var_phix1_dc_rv_slot;
        let mut var_phix2: f64 = *var_phix2_slot;
        let mut var_phix2_rv: f64 = *var_phix2_rv_slot;
        let mut var_phix_ac: f64 = *var_phix_ac_slot;
        let mut var_phix_ac_rv: f64 = *var_phix_ac_rv_slot;
        let mut var_phix_dc: f64 = *var_phix_dc_slot;
        let mut var_phix_dc_rv: f64 = *var_phix_dc_rv_slot;
        let mut var_qb0: f64 = *var_qb0_slot;
        let mut var_qb0_rv: f64 = *var_qb0_rv_slot;
        let mut var_qlim2: f64 = *var_qlim2_slot;
        let mut var_qlim2_rv: f64 = *var_qlim2_rv_slot;
        let mut var_rs_t: f64 = *var_rs_t_slot;
        let mut var_rs_t_rv: f64 = *var_rs_t_rv_slot;
        let mut var_sqrt_phib_dc: f64 = *var_sqrt_phib_dc_slot;
        let mut var_sqrt_phib_dc_rv: f64 = *var_sqrt_phib_dc_rv_slot;
        let mut var_tf_bet: f64 = *var_tf_bet_slot;
        let mut var_tf_bet_rv: f64 = *var_tf_bet_rv_slot;
        let mut var_tf_betedge: f64 = *var_tf_betedge_slot;
        let mut var_tf_betedge_rv: f64 = *var_tf_betedge_rv_slot;
        let mut var_tf_cs: f64 = *var_tf_cs_slot;
        let mut var_tf_cs_rv: f64 = *var_tf_cs_rv_slot;
        let mut var_tf_ct: f64 = *var_tf_ct_slot;
        let mut var_tf_ct_rv: f64 = *var_tf_ct_rv_slot;
        let mut var_tf_mue: f64 = *var_tf_mue_slot;
        let mut var_tf_mue_rv: f64 = *var_tf_mue_rv_slot;
        let mut var_tf_ther: f64 = *var_tf_ther_slot;
        let mut var_tf_ther_rv: f64 = *var_tf_ther_rv_slot;
        let mut var_tf_thesat: f64 = *var_tf_thesat_slot;
        let mut var_tf_thesat_rv: f64 = *var_tf_thesat_rv_slot;
        let mut var_tf_xcor: f64 = *var_tf_xcor_slot;
        let mut var_tf_xcor_rv: f64 = *var_tf_xcor_rv_slot;
        let mut var_thecs_t: f64 = *var_thecs_t_slot;
        let mut var_thecs_t_rv: f64 = *var_thecs_t_rv_slot;
        let mut var_themu_t: f64 = *var_themu_t_slot;
        let mut var_themu_t_rv: f64 = *var_themu_t_rv_slot;
        let mut var_ther_i: f64 = *var_ther_i_slot;
        let mut var_ther_i_rv: f64 = *var_ther_i_rv_slot;
        let mut var_thesat_t: f64 = *var_thesat_t_slot;
        let mut var_thesat_t_rv: f64 = *var_thesat_t_rv_slot;
        let mut var_thesatac_t: f64 = *var_thesatac_t_slot;
        let mut var_thesatac_t_rv: f64 = *var_thesatac_t_rv_slot;
        let mut var_us1: f64 = *var_us1_slot;
        let mut var_us1_rv: f64 = *var_us1_rv_slot;
        let mut var_us21: f64 = *var_us21_slot;
        let mut var_us21_rv: f64 = *var_us21_rv_slot;
        let mut var_vfb_t: f64 = *var_vfb_t_slot;
        let mut var_vfb_t_rv: f64 = *var_vfb_t_rv_slot;
        let mut var_vfbedge_t: f64 = *var_vfbedge_t_slot;
        let mut var_vfbedge_t_rv: f64 = *var_vfbedge_t_rv_slot;
        let mut var_xcor_t: f64 = *var_xcor_t_slot;
        let mut var_xcor_t_rv: f64 = *var_xcor_t_rv_slot;

        let (assign11600_e10546,) = {
    if (var_phib_dc > 0.05) {
        (var_phib_dc,)
    } else {
        (0.05,)
    }
};
        var_phib_dc = assign11600_e10546;
        var_phib_dc_rv = 0.0;

        let assign11610_e10549: f64 = (2.0 * 1.6021918e-19);
        let assign11610_e10551: f64 = (assign11610_e10549 * var_neff_i);
        let assign11610_e10553: f64 = (assign11610_e10551 * var_epssi);
        let assign11610_e10555: f64 = (assign11610_e10553 * var_inv_phit);
        let assign11610_e10556: f64 = (assign11610_e10555).sqrt();
        let assign11610_e10558: f64 = (assign11610_e10556 / var_coxprime);
        var_g_0_dc = assign11610_e10558;
        var_g_0_dc_rv = 0.0;

        var_kp = 0.0;
        var_kp_rv = 0.0;

        var_np = 0.0;
        var_np_rv = 0.0;

        let assign11640_e10563: f64 = if var_np_i > 0.0 { 1.0 } else { 0.0 };
        var_guard157 = assign11640_e10563;
        var_guard157_rv = 0.0;

        let (assign11650_e10569,) = {
    if (var_guard157 != 0.0) {
        let assign11650_e10567: f64 = (80000000.0 / var_tox_sq);
        (assign11650_e10567,)
    } else {
        (var_arg2max,)
    }
};
        var_arg2max = assign11650_e10569;
        var_arg2max_rv = 0.0;

        let (assign11660_e10578,) = {
    if (var_guard157 != 0.0) {
        let (assign11660_e10576,) = {
            if (var_np_i > var_arg2max) {
                (var_np_i,)
            } else {
                (var_arg2max,)
            }
        };
        (assign11660_e10576,)
    } else {
        (var_np,)
    }
};
        var_np = assign11660_e10578;
        var_np_rv = 0.0;

        let (assign11670_e10587,) = {
    if (var_guard157 != 0.0) {
        let (assign11670_e10585,) = {
            if (5e24 > var_np) {
                (5e24,)
            } else {
                (var_np,)
            }
        };
        (assign11670_e10585,)
    } else {
        (var_np,)
    }
};
        var_np = assign11670_e10587;
        var_np_rv = 0.0;

        let (assign11680_e10603,) = {
    if (var_guard157 != 0.0) {
        let assign11680_e10591: f64 = (2.0 * var_coxprime);
        let assign11680_e10593: f64 = (assign11680_e10591 * var_coxprime);
        let assign11680_e10595: f64 = (assign11680_e10593 * var_phit);
        let assign11680_e10598: f64 = (1.6021918e-19 * var_np);
        let assign11680_e10600: f64 = (assign11680_e10598 * var_epssi);
        let assign11680_e10601: f64 = (assign11680_e10595 / assign11680_e10600);
        (assign11680_e10601,)
    } else {
        (var_kp,)
    }
};
        var_kp = assign11680_e10603;
        var_kp_rv = 0.0;

        let assign11690_e10606: f64 = (100.0 * var_phit);
        let assign11690_e10608: f64 = (assign11690_e10606 * var_phit);
        var_qlim2 = assign11690_e10608;
        var_qlim2_rv = 0.0;

        let assign11700_e10611: f64 = if p.p51 > 0.0 { 1.0 } else { 0.0 };
        var_guard158 = assign11700_e10611;
        var_guard158_rv = 0.0;

        let (assign11710_e10622,) = {
    if (var_guard158 != 0.0) {
        let assign11710_e10615: f64 = (var_phit * var_g_0_dc);
        let assign11710_e10617: f64 = (assign11710_e10615 * var_g_0_dc);
        let assign11710_e10619: f64 = (assign11710_e10617 * var_phib_dc);
        let assign11710_e10620: f64 = (assign11710_e10619).sqrt();
        (assign11710_e10620,)
    } else {
        (var_qb0,)
    }
};
        var_qb0 = assign11710_e10622;
        var_qb0_rv = 0.0;

        let (assign11720_e10632,) = {
    if (var_guard158 != 0.0) {
        let assign11720_e10626: f64 = (0.75 * var_qq);
        let assign11720_e10629: f64 = (var_qb0).powf(0.6666666666666666);
        let assign11720_e10630: f64 = (assign11720_e10626 * assign11720_e10629);
        (assign11720_e10630,)
    } else {
        (var_dphibq,)
    }
};
        var_dphibq = assign11720_e10632;
        var_dphibq_rv = 0.0;

        let (assign11730_e10638,) = {
    if (var_guard158 != 0.0) {
        let assign11730_e10636: f64 = (var_phib_dc + var_dphibq);
        (assign11730_e10636,)
    } else {
        (var_phib_dc,)
    }
};
        var_phib_dc = assign11730_e10638;
        var_phib_dc_rv = 0.0;

        let (assign11740_e10652,) = {
    if (var_guard158 != 0.0) {
        let assign11740_e10644: f64 = (2.0 * 0.6666666666666666);
        let assign11740_e10646: f64 = (assign11740_e10644 * var_dphibq);
        let assign11740_e10648: f64 = (assign11740_e10646 / var_qb0);
        let assign11740_e10649: f64 = (1.0 + assign11740_e10648);
        let assign11740_e10650: f64 = (var_g_0_dc * assign11740_e10649);
        (assign11740_e10650,)
    } else {
        (var_g_0_dc,)
    }
};
        var_g_0_dc = assign11740_e10652;
        var_g_0_dc_rv = 0.0;

        let assign11750_e10654: f64 = (var_phib_dc).sqrt();
        var_sqrt_phib_dc = assign11750_e10654;
        var_sqrt_phib_dc_rv = 0.0;

        let assign11760_e10657: f64 = (0.95 * var_phib_dc);
        var_phix_dc = assign11760_e10657;
        var_phix_dc_rv = 0.0;

        let assign11770_e10660: f64 = (0.0025 * var_phib_dc);
        let assign11770_e10662: f64 = (assign11770_e10660 * var_phib_dc);
        var_aphi_dc = assign11770_e10662;
        var_aphi_dc_rv = 0.0;

        var_bphi_dc = var_aphi_dc;
        var_bphi_dc_rv = 0.0;

        let assign11790_e10666: f64 = (var_bphi_dc).sqrt();
        let assign11790_e10667: f64 = (0.5 * assign11790_e10666);
        var_phix2 = assign11790_e10667;
        var_phix2_rv = 0.0;

        let assign11800_e10671: f64 = (var_phix_dc - var_phix2);
        let assign11800_e10673: f64 = assign11800_e10671;
        let assign11800_e10676: f64 = (var_phix_dc - var_phix2);
        let assign11800_e10678: f64 = assign11800_e10676;
        let assign11800_e10681: f64 = (var_phix_dc - var_phix2);
        let assign11800_e10683: f64 = assign11800_e10681;
        let assign11800_e10684: f64 = (assign11800_e10678 * assign11800_e10683);
        let assign11800_e10686: f64 = (assign11800_e10684 + var_aphi_dc);
        let assign11800_e10687: f64 = (assign11800_e10686).sqrt();
        let assign11800_e10688: f64 = (assign11800_e10673 - assign11800_e10687);
        let assign11800_e10689: f64 = (0.5 * assign11800_e10688);
        var_phix1_dc = assign11800_e10689;
        var_phix1_dc_rv = 0.0;

        let assign11810_e10693: f64 = (var_phib_dc + var_eg);
        let assign11810_e10694: f64 = (0.5 * assign11810_e10693);
        var_alpha_b = assign11810_e10694;
        var_alpha_b_rv = 0.0;

        let assign11820_e10697: f64 = (var_vsbnud_i + var_phib_dc);
        let assign11820_e10698: f64 = (assign11820_e10697).sqrt();
        let assign11820_e10700: f64 = (assign11820_e10698 - var_sqrt_phib_dc);
        var_us1 = assign11820_e10700;
        var_us1_rv = 0.0;

        let assign11830_e10703: f64 = (var_vsbnud_i + var_dvsbnud_i);
        let assign11830_e10705: f64 = (assign11830_e10703 + var_phib_dc);
        let assign11830_e10706: f64 = (assign11830_e10705).sqrt();
        let assign11830_e10708: f64 = (assign11830_e10706 - var_sqrt_phib_dc);
        let assign11830_e10710: f64 = (assign11830_e10708 - var_us1);
        var_us21 = assign11830_e10710;
        var_us21_rv = 0.0;

        let assign11840_e10713: f64 = (var_eg + var_dphib_i);
        let assign11840_e10715: f64 = (assign11840_e10713 + var_delvtac_i);
        let assign11840_e10718: f64 = (2.0 * var_phit);
        let assign11840_e10722: f64 = (-0.75);
        let assign11840_e10723: f64 = (var_phibfac).powf(assign11840_e10722);
        let assign11840_e10724: f64 = (var_neffac_i * assign11840_e10723);
        let assign11840_e10726: f64 = (assign11840_e10724 * 4e-26);
        let assign11840_e10727: f64 = (assign11840_e10726).ln();
        let assign11840_e10728: f64 = (assign11840_e10718 * assign11840_e10727);
        let assign11840_e10729: f64 = (assign11840_e10715 + assign11840_e10728);
        var_phib_ac = assign11840_e10729;
        var_phib_ac_rv = 0.0;

        let (assign11850_e10735,) = {
    if (var_phib_ac > 0.05) {
        (var_phib_ac,)
    } else {
        (0.05,)
    }
};
        var_phib_ac = assign11850_e10735;
        var_phib_ac_rv = 0.0;

        let assign11860_e10738: f64 = (2.0 * 1.6021918e-19);
        let assign11860_e10740: f64 = (assign11860_e10738 * var_neffac_i);
        let assign11860_e10742: f64 = (assign11860_e10740 * var_epssi);
        let assign11860_e10744: f64 = (assign11860_e10742 * var_inv_phit);
        let assign11860_e10745: f64 = (assign11860_e10744).sqrt();
        let assign11860_e10747: f64 = (assign11860_e10745 / var_coxprime);
        var_g_0_ac = assign11860_e10747;
        var_g_0_ac_rv = 0.0;

        let assign11870_e10750: f64 = if p.p51 > 0.0 { 1.0 } else { 0.0 };
        var_guard159 = assign11870_e10750;
        var_guard159_rv = 0.0;

        let (assign11880_e10761,) = {
    if (var_guard159 != 0.0) {
        let assign11880_e10754: f64 = (var_phit * var_g_0_ac);
        let assign11880_e10756: f64 = (assign11880_e10754 * var_g_0_ac);
        let assign11880_e10758: f64 = (assign11880_e10756 * var_phib_ac);
        let assign11880_e10759: f64 = (assign11880_e10758).sqrt();
        (assign11880_e10759,)
    } else {
        (var_qb0,)
    }
};
        var_qb0 = assign11880_e10761;
        var_qb0_rv = 0.0;

        let (assign11890_e10771,) = {
    if (var_guard159 != 0.0) {
        let assign11890_e10765: f64 = (0.75 * var_qq);
        let assign11890_e10768: f64 = (var_qb0).powf(0.6666666666666666);
        let assign11890_e10769: f64 = (assign11890_e10765 * assign11890_e10768);
        (assign11890_e10769,)
    } else {
        (var_dphibq,)
    }
};
        var_dphibq = assign11890_e10771;
        var_dphibq_rv = 0.0;

        let (assign11900_e10777,) = {
    if (var_guard159 != 0.0) {
        let assign11900_e10775: f64 = (var_phib_ac + var_dphibq);
        (assign11900_e10775,)
    } else {
        (var_phib_ac,)
    }
};
        var_phib_ac = assign11900_e10777;
        var_phib_ac_rv = 0.0;

        let (assign11910_e10791,) = {
    if (var_guard159 != 0.0) {
        let assign11910_e10783: f64 = (2.0 * 0.6666666666666666);
        let assign11910_e10785: f64 = (assign11910_e10783 * var_dphibq);
        let assign11910_e10787: f64 = (assign11910_e10785 / var_qb0);
        let assign11910_e10788: f64 = (1.0 + assign11910_e10787);
        let assign11910_e10789: f64 = (var_g_0_ac * assign11910_e10788);
        (assign11910_e10789,)
    } else {
        (var_g_0_ac,)
    }
};
        var_g_0_ac = assign11910_e10791;
        var_g_0_ac_rv = 0.0;

        let assign11920_e10794: f64 = (0.95 * var_phib_ac);
        var_phix_ac = assign11920_e10794;
        var_phix_ac_rv = 0.0;

        let assign11930_e10797: f64 = (0.0025 * var_phib_ac);
        let assign11930_e10799: f64 = (assign11930_e10797 * var_phib_ac);
        var_aphi_ac = assign11930_e10799;
        var_aphi_ac_rv = 0.0;

        var_bphi_ac = var_aphi_ac;
        var_bphi_ac_rv = 0.0;

        let assign11950_e10803: f64 = (var_bphi_ac).sqrt();
        let assign11950_e10804: f64 = (0.5 * assign11950_e10803);
        var_phix2 = assign11950_e10804;
        var_phix2_rv = 0.0;

        let assign11960_e10808: f64 = (var_phix_ac - var_phix2);
        let assign11960_e10810: f64 = assign11960_e10808;
        let assign11960_e10813: f64 = (var_phix_ac - var_phix2);
        let assign11960_e10815: f64 = assign11960_e10813;
        let assign11960_e10818: f64 = (var_phix_ac - var_phix2);
        let assign11960_e10820: f64 = assign11960_e10818;
        let assign11960_e10821: f64 = (assign11960_e10815 * assign11960_e10820);
        let assign11960_e10823: f64 = (assign11960_e10821 + var_aphi_ac);
        let assign11960_e10824: f64 = (assign11960_e10823).sqrt();
        let assign11960_e10825: f64 = (assign11960_e10810 - assign11960_e10824);
        let assign11960_e10826: f64 = (0.5 * assign11960_e10825);
        var_phix1_ac = assign11960_e10826;
        var_phix1_ac_rv = 0.0;

        let assign11970_e10830: f64 = (var_stvfb_i * var_delt);
        let assign11970_e10834: f64 = (var_st2vfb_i * var_delt);
        let assign11970_e10835: f64 = (1.0 + assign11970_e10834);
        let assign11970_e10836: f64 = (assign11970_e10830 * assign11970_e10835);
        let assign11970_e10837: f64 = (var_vfb_i + assign11970_e10836);
        let assign11970_e10839: f64 = (assign11970_e10837 + var_delvto_i);
        var_vfb_t = assign11970_e10839;
        var_vfb_t_rv = 0.0;

        let assign11980_e10842: f64 = (var_stct_i * var_ln_rtn);
        let assign11980_e10843: f64 = (assign11980_e10842).exp();
        var_tf_ct = assign11980_e10843;
        var_tf_ct_rv = 0.0;

        let assign11990_e10846: f64 = (var_ct_i * var_tf_ct);
        var_ct_t = assign11990_e10846;
        var_ct_t_rv = 0.0;

        let assign12000_e10849: f64 = (var_ctg_i / var_rtn);
        var_ctg_t = assign12000_e10849;
        var_ctg_t_rv = 0.0;

        let assign12010_e10852: f64 = (var_stbet_i * var_ln_rtn);
        let assign12010_e10853: f64 = (assign12010_e10852).exp();
        var_tf_bet = assign12010_e10853;
        var_tf_bet_rv = 0.0;

        let assign12020_e10856: f64 = (var_betn_i * var_tf_bet);
        var_betn_t = assign12020_e10856;
        var_betn_t_rv = 0.0;

        let assign12030_e10859: f64 = (var_factuo_i * var_betn_t);
        let assign12030_e10861: f64 = (assign12030_e10859 * var_coxprime);
        var_bet_i = assign12030_e10861;
        var_bet_i_rv = 0.0;

        let assign12040_e10865: f64 = (var_stthemu_i * var_ln_rtn);
        let assign12040_e10866: f64 = (assign12040_e10865).exp();
        let assign12040_e10867: f64 = (var_themu_i * assign12040_e10866);
        var_themu_t = assign12040_e10867;
        var_themu_t_rv = 0.0;

        let assign12050_e10870: f64 = (var_stmue_i * var_ln_rtn);
        let assign12050_e10871: f64 = (assign12050_e10870).exp();
        var_tf_mue = assign12050_e10871;
        var_tf_mue_rv = 0.0;

        let assign12060_e10874: f64 = (var_mue_i * var_tf_mue);
        var_mue_t = assign12060_e10874;
        var_mue_t_rv = 0.0;

        let assign12070_e10878: f64 = (var_stthecs_i * var_ln_rtn);
        let assign12070_e10879: f64 = (assign12070_e10878).exp();
        let assign12070_e10880: f64 = (var_thecs_i * assign12070_e10879);
        var_thecs_t = assign12070_e10880;
        var_thecs_t_rv = 0.0;

        let assign12080_e10883: f64 = (var_stcs_i * var_ln_rtn);
        let assign12080_e10884: f64 = (assign12080_e10883).exp();
        var_tf_cs = assign12080_e10884;
        var_tf_cs_rv = 0.0;

        let assign12090_e10887: f64 = (var_cs_i * var_tf_cs);
        var_cs_t = assign12090_e10887;
        var_cs_t_rv = 0.0;

        let assign12100_e10890: f64 = (var_stxcor_i * var_ln_rtn);
        let assign12100_e10891: f64 = (assign12100_e10890).exp();
        var_tf_xcor = assign12100_e10891;
        var_tf_xcor_rv = 0.0;

        let assign12110_e10894: f64 = (var_xcor_i * var_tf_xcor);
        var_xcor_t = assign12110_e10894;
        var_xcor_t_rv = 0.0;

        let assign12120_e10897: f64 = (var_strs_i * var_ln_rtn);
        let assign12120_e10898: f64 = (assign12120_e10897).exp();
        var_tf_ther = assign12120_e10898;
        var_tf_ther_rv = 0.0;

        let assign12130_e10901: f64 = (var_rs_i * var_tf_ther);
        var_rs_t = assign12130_e10901;
        var_rs_t_rv = 0.0;

        let assign12140_e10904: f64 = (2.0 * var_bet_i);
        let assign12140_e10906: f64 = (assign12140_e10904 * var_rs_t);
        var_ther_i = assign12140_e10906;
        var_ther_i_rv = 0.0;

        let assign12150_e10909: f64 = (var_stthesat_i * var_ln_rtn);
        let assign12150_e10910: f64 = (assign12150_e10909).exp();
        var_tf_thesat = assign12150_e10910;
        var_tf_thesat_rv = 0.0;

        let assign12160_e10913: f64 = (var_thesat_i * var_tf_thesat);
        var_thesat_t = assign12160_e10913;
        var_thesat_t_rv = 0.0;

        let assign12170_e10916: f64 = (var_thesatac_i * var_tf_thesat);
        var_thesatac_t = assign12170_e10916;
        var_thesatac_t_rv = 0.0;

        let assign12180_e10919: f64 = (-var_sta2_i);
        let assign12180_e10921: f64 = (assign12180_e10919 * var_ln_rtn);
        let assign12180_e10922: f64 = (assign12180_e10921).exp();
        let assign12180_e10923: f64 = (var_a2_i * assign12180_e10922);
        var_a2_t = assign12180_e10923;
        var_a2_t_rv = 0.0;

        let assign12190_e10926: f64 = (var_fnt_i * 4.0);
        let assign12190_e10928: f64 = (assign12190_e10926 * 1.3806505e-23);
        let assign12190_e10930: f64 = (assign12190_e10928 * var_tkd);
        var_nt = assign12190_e10930;
        var_nt_rv = 0.0;

        let assign12210_e10944: f64 = if ((p.p46 != 0.0) && (var_betnedge_i > 0.0)) { 1.0 } else { 0.0 };
        var_guard160 = assign12210_e10944;
        var_guard160_rv = 0.0;

        let (assign12220_e10954,) = {
    if (var_guard160 != 0.0) {
        let assign12220_e10949: f64 = (var_stvfbedge_i * var_delt);
        let assign12220_e10950: f64 = (var_vfbedge_i + assign12220_e10949);
        let assign12220_e10952: f64 = (assign12220_e10950 + var_delvtoedge_i);
        (assign12220_e10952,)
    } else {
        (var_vfbedge_t,)
    }
};
        var_vfbedge_t = assign12220_e10954;
        var_vfbedge_t_rv = 0.0;

        let (assign12230_e10961,) = {
    if (var_guard160 != 0.0) {
        let assign12230_e10958: f64 = (var_stbetedge_i * var_ln_rtn);
        let assign12230_e10959: f64 = (assign12230_e10958).exp();
        (assign12230_e10959,)
    } else {
        (var_tf_betedge,)
    }
};
        var_tf_betedge = assign12230_e10961;
        var_tf_betedge_rv = 0.0;

        let (assign12240_e10967,) = {
    if (var_guard160 != 0.0) {
        let assign12240_e10965: f64 = (var_betnedge_i * var_tf_betedge);
        (assign12240_e10965,)
    } else {
        (var_betnedge_t,)
    }
};
        var_betnedge_t = assign12240_e10967;
        var_betnedge_t_rv = 0.0;

        let (assign12250_e10975,) = {
    if (var_guard160 != 0.0) {
        let assign12250_e10971: f64 = (var_factuoedge_i * var_betnedge_t);
        let assign12250_e10973: f64 = (assign12250_e10971 * var_coxprime);
        (assign12250_e10973,)
    } else {
        (var_betedge_i,)
    }
};
        var_betedge_i = assign12250_e10975;
        var_betedge_i_rv = 0.0;

        let (assign12260_e10985,) = {
    if (var_guard160 != 0.0) {
        let assign12260_e10981: f64 = (var_ctedge_i * var_rtn);
        let assign12260_e10982: f64 = (1.0 + assign12260_e10981);
        let assign12260_e10983: f64 = (var_phit * assign12260_e10982);
        (assign12260_e10983,)
    } else {
        (var_phit0edge,)
    }
};
        var_phit0edge = assign12260_e10985;
        var_phit0edge_rv = 0.0;

        *var_a2_t_slot = var_a2_t;
        *var_a2_t_rv_slot = var_a2_t_rv;
        *var_alpha_b_slot = var_alpha_b;
        *var_alpha_b_rv_slot = var_alpha_b_rv;
        *var_aphi_ac_slot = var_aphi_ac;
        *var_aphi_ac_rv_slot = var_aphi_ac_rv;
        *var_aphi_dc_slot = var_aphi_dc;
        *var_aphi_dc_rv_slot = var_aphi_dc_rv;
        *var_arg2max_slot = var_arg2max;
        *var_arg2max_rv_slot = var_arg2max_rv;
        *var_bet_i_slot = var_bet_i;
        *var_bet_i_rv_slot = var_bet_i_rv;
        *var_betedge_i_slot = var_betedge_i;
        *var_betedge_i_rv_slot = var_betedge_i_rv;
        *var_betn_t_slot = var_betn_t;
        *var_betn_t_rv_slot = var_betn_t_rv;
        *var_betnedge_t_slot = var_betnedge_t;
        *var_betnedge_t_rv_slot = var_betnedge_t_rv;
        *var_bphi_ac_slot = var_bphi_ac;
        *var_bphi_ac_rv_slot = var_bphi_ac_rv;
        *var_bphi_dc_slot = var_bphi_dc;
        *var_bphi_dc_rv_slot = var_bphi_dc_rv;
        *var_cs_t_slot = var_cs_t;
        *var_cs_t_rv_slot = var_cs_t_rv;
        *var_ct_t_slot = var_ct_t;
        *var_ct_t_rv_slot = var_ct_t_rv;
        *var_ctg_t_slot = var_ctg_t;
        *var_ctg_t_rv_slot = var_ctg_t_rv;
        *var_dphibq_slot = var_dphibq;
        *var_dphibq_rv_slot = var_dphibq_rv;
        *var_g_0_ac_slot = var_g_0_ac;
        *var_g_0_ac_rv_slot = var_g_0_ac_rv;
        *var_g_0_dc_slot = var_g_0_dc;
        *var_g_0_dc_rv_slot = var_g_0_dc_rv;
        *var_guard157_slot = var_guard157;
        *var_guard157_rv_slot = var_guard157_rv;
        *var_guard158_slot = var_guard158;
        *var_guard158_rv_slot = var_guard158_rv;
        *var_guard159_slot = var_guard159;
        *var_guard159_rv_slot = var_guard159_rv;
        *var_guard160_slot = var_guard160;
        *var_guard160_rv_slot = var_guard160_rv;
        *var_kp_slot = var_kp;
        *var_kp_rv_slot = var_kp_rv;
        *var_mue_t_slot = var_mue_t;
        *var_mue_t_rv_slot = var_mue_t_rv;
        *var_np_slot = var_np;
        *var_np_rv_slot = var_np_rv;
        *var_nt_slot = var_nt;
        *var_nt_rv_slot = var_nt_rv;
        *var_phib_ac_slot = var_phib_ac;
        *var_phib_ac_rv_slot = var_phib_ac_rv;
        *var_phib_dc_slot = var_phib_dc;
        *var_phib_dc_rv_slot = var_phib_dc_rv;
        *var_phit0edge_slot = var_phit0edge;
        *var_phit0edge_rv_slot = var_phit0edge_rv;
        *var_phix1_ac_slot = var_phix1_ac;
        *var_phix1_ac_rv_slot = var_phix1_ac_rv;
        *var_phix1_dc_slot = var_phix1_dc;
        *var_phix1_dc_rv_slot = var_phix1_dc_rv;
        *var_phix2_slot = var_phix2;
        *var_phix2_rv_slot = var_phix2_rv;
        *var_phix_ac_slot = var_phix_ac;
        *var_phix_ac_rv_slot = var_phix_ac_rv;
        *var_phix_dc_slot = var_phix_dc;
        *var_phix_dc_rv_slot = var_phix_dc_rv;
        *var_qb0_slot = var_qb0;
        *var_qb0_rv_slot = var_qb0_rv;
        *var_qlim2_slot = var_qlim2;
        *var_qlim2_rv_slot = var_qlim2_rv;
        *var_rs_t_slot = var_rs_t;
        *var_rs_t_rv_slot = var_rs_t_rv;
        *var_sqrt_phib_dc_slot = var_sqrt_phib_dc;
        *var_sqrt_phib_dc_rv_slot = var_sqrt_phib_dc_rv;
        *var_tf_bet_slot = var_tf_bet;
        *var_tf_bet_rv_slot = var_tf_bet_rv;
        *var_tf_betedge_slot = var_tf_betedge;
        *var_tf_betedge_rv_slot = var_tf_betedge_rv;
        *var_tf_cs_slot = var_tf_cs;
        *var_tf_cs_rv_slot = var_tf_cs_rv;
        *var_tf_ct_slot = var_tf_ct;
        *var_tf_ct_rv_slot = var_tf_ct_rv;
        *var_tf_mue_slot = var_tf_mue;
        *var_tf_mue_rv_slot = var_tf_mue_rv;
        *var_tf_ther_slot = var_tf_ther;
        *var_tf_ther_rv_slot = var_tf_ther_rv;
        *var_tf_thesat_slot = var_tf_thesat;
        *var_tf_thesat_rv_slot = var_tf_thesat_rv;
        *var_tf_xcor_slot = var_tf_xcor;
        *var_tf_xcor_rv_slot = var_tf_xcor_rv;
        *var_thecs_t_slot = var_thecs_t;
        *var_thecs_t_rv_slot = var_thecs_t_rv;
        *var_themu_t_slot = var_themu_t;
        *var_themu_t_rv_slot = var_themu_t_rv;
        *var_ther_i_slot = var_ther_i;
        *var_ther_i_rv_slot = var_ther_i_rv;
        *var_thesat_t_slot = var_thesat_t;
        *var_thesat_t_rv_slot = var_thesat_t_rv;
        *var_thesatac_t_slot = var_thesatac_t;
        *var_thesatac_t_rv_slot = var_thesatac_t_rv;
        *var_us1_slot = var_us1;
        *var_us1_rv_slot = var_us1_rv;
        *var_us21_slot = var_us21;
        *var_us21_rv_slot = var_us21_rv;
        *var_vfb_t_slot = var_vfb_t;
        *var_vfb_t_rv_slot = var_vfb_t_rv;
        *var_vfbedge_t_slot = var_vfbedge_t;
        *var_vfbedge_t_rv_slot = var_vfbedge_t_rv;
        *var_xcor_t_slot = var_xcor_t;
        *var_xcor_t_rv_slot = var_xcor_t_rv;
    }

    pub(super) fn stamp_reactive_block_19(
        p: &Parameters,
        var_abdrain_i: f64,
        var_absource_i: f64,
        var_axinr_i: f64,
        var_bgidl_i: f64,
        var_bgidld_i: f64,
        var_chib_i: f64,
        var_coxprime: f64,
        var_delta: f64,
        var_dphibedge_i: f64,
        var_eg: f64,
        var_epssi: f64,
        var_fcinracc_i: f64,
        var_gc2_i: f64,
        var_gc2ov_i: f64,
        var_gc2ovd_i: f64,
        var_gc3_i: f64,
        var_gc3ov_i: f64,
        var_gc3ovd_i: f64,
        var_guard160: f64,
        var_inv_phit: f64,
        var_invnf: f64,
        var_jw_i: f64,
        var_lgdrain_i: f64,
        var_lgsource_i: f64,
        var_lsdrain_i: f64,
        var_lssource_i: f64,
        var_neffedge_i: f64,
        var_phibfac: f64,
        var_phit: f64,
        var_rta: f64,
        var_stbgidl_i: f64,
        var_stbgidld_i: f64,
        var_stig_i: f64,
        var_tox_i: f64,
        var_toxov_i: f64,
        var_toxovd_i: f64,
        var_we: f64,
        var_abd_i_slot: &mut f64,
        var_abd_i_rv_slot: &mut f64,
        var_abs_i_slot: &mut f64,
        var_abs_i_rv_slot: &mut f64,
        var_ainr_slot: &mut f64,
        var_ainr_rv_slot: &mut f64,
        var_aphiedge_slot: &mut f64,
        var_aphiedge_rv_slot: &mut f64,
        var_b_fact_slot: &mut f64,
        var_b_fact_rv_slot: &mut f64,
        var_bch_slot: &mut f64,
        var_bch_rv_slot: &mut f64,
        var_betedge_i_slot: &mut f64,
        var_betedge_i_rv_slot: &mut f64,
        var_betnedge_t_slot: &mut f64,
        var_betnedge_t_rv_slot: &mut f64,
        var_bgidl_t_slot: &mut f64,
        var_bgidl_t_rv_slot: &mut f64,
        var_bgidld_t_slot: &mut f64,
        var_bgidld_t_rv_slot: &mut f64,
        var_bgidlds_slot: &mut f64,
        var_bgidlds_rv_slot: &mut f64,
        var_bgidls_slot: &mut f64,
        var_bgidls_rv_slot: &mut f64,
        var_bov_slot: &mut f64,
        var_bov_d_slot: &mut f64,
        var_bov_d_rv_slot: &mut f64,
        var_bov_rv_slot: &mut f64,
        var_bphiedge_slot: &mut f64,
        var_bphiedge_rv_slot: &mut f64,
        var_gcq_slot: &mut f64,
        var_gcq_rv_slot: &mut f64,
        var_gcqov_slot: &mut f64,
        var_gcqov_rv_slot: &mut f64,
        var_gcqovd_slot: &mut f64,
        var_gcqovd_rv_slot: &mut f64,
        var_gfedge_slot: &mut f64,
        var_gfedge2_slot: &mut f64,
        var_gfedge2_rv_slot: &mut f64,
        var_gfedge_rv_slot: &mut f64,
        var_guard161_slot: &mut f64,
        var_guard161_rv_slot: &mut f64,
        var_guard162_slot: &mut f64,
        var_guard162_rv_slot: &mut f64,
        var_guard163_slot: &mut f64,
        var_guard163_rv_slot: &mut f64,
        var_guard164_slot: &mut f64,
        var_guard164_rv_slot: &mut f64,
        var_guard172_slot: &mut f64,
        var_guard172_rv_slot: &mut f64,
        var_guard173_slot: &mut f64,
        var_guard173_rv_slot: &mut f64,
        var_iginv_i_slot: &mut f64,
        var_iginv_i_rv_slot: &mut f64,
        var_igov_i_slot: &mut f64,
        var_igov_i_rv_slot: &mut f64,
        var_igovd_i_slot: &mut f64,
        var_igovd_i_rv_slot: &mut f64,
        var_inv_chib_slot: &mut f64,
        var_inv_chib_rv_slot: &mut f64,
        var_jwcorr_slot: &mut f64,
        var_jwcorr_rv_slot: &mut f64,
        var_jww_slot: &mut f64,
        var_jww_rv_slot: &mut f64,
        var_lgd_i_slot: &mut f64,
        var_lgd_i_rv_slot: &mut f64,
        var_lgs_i_slot: &mut f64,
        var_lgs_i_rv_slot: &mut f64,
        var_lngfedge2_slot: &mut f64,
        var_lngfedge2_rv_slot: &mut f64,
        var_lsd_i_slot: &mut f64,
        var_lsd_i_rv_slot: &mut f64,
        var_lss_i_slot: &mut f64,
        var_lss_i_rv_slot: &mut f64,
        var_phibedge_slot: &mut f64,
        var_phibedge_rv_slot: &mut f64,
        var_phit0edge_slot: &mut f64,
        var_phit0edge_rv_slot: &mut f64,
        var_phix1edge_slot: &mut f64,
        var_phix1edge_rv_slot: &mut f64,
        var_phix2edge_slot: &mut f64,
        var_phix2edge_rv_slot: &mut f64,
        var_phixedge_slot: &mut f64,
        var_phixedge_rv_slot: &mut f64,
        var_tf_betedge_slot: &mut f64,
        var_tf_betedge_rv_slot: &mut f64,
        var_tf_ig_slot: &mut f64,
        var_tf_ig_rv_slot: &mut f64,
        var_vfbedge_t_slot: &mut f64,
        var_vfbedge_t_rv_slot: &mut f64,
        var_vinr_max_slot: &mut f64,
        var_vinr_max_rv_slot: &mut f64,
    ) {
        let mut var_abd_i: f64 = *var_abd_i_slot;
        let mut var_abd_i_rv: f64 = *var_abd_i_rv_slot;
        let mut var_abs_i: f64 = *var_abs_i_slot;
        let mut var_abs_i_rv: f64 = *var_abs_i_rv_slot;
        let mut var_ainr: f64 = *var_ainr_slot;
        let mut var_ainr_rv: f64 = *var_ainr_rv_slot;
        let mut var_aphiedge: f64 = *var_aphiedge_slot;
        let mut var_aphiedge_rv: f64 = *var_aphiedge_rv_slot;
        let mut var_b_fact: f64 = *var_b_fact_slot;
        let mut var_b_fact_rv: f64 = *var_b_fact_rv_slot;
        let mut var_bch: f64 = *var_bch_slot;
        let mut var_bch_rv: f64 = *var_bch_rv_slot;
        let mut var_betedge_i: f64 = *var_betedge_i_slot;
        let mut var_betedge_i_rv: f64 = *var_betedge_i_rv_slot;
        let mut var_betnedge_t: f64 = *var_betnedge_t_slot;
        let mut var_betnedge_t_rv: f64 = *var_betnedge_t_rv_slot;
        let mut var_bgidl_t: f64 = *var_bgidl_t_slot;
        let mut var_bgidl_t_rv: f64 = *var_bgidl_t_rv_slot;
        let mut var_bgidld_t: f64 = *var_bgidld_t_slot;
        let mut var_bgidld_t_rv: f64 = *var_bgidld_t_rv_slot;
        let mut var_bgidlds: f64 = *var_bgidlds_slot;
        let mut var_bgidlds_rv: f64 = *var_bgidlds_rv_slot;
        let mut var_bgidls: f64 = *var_bgidls_slot;
        let mut var_bgidls_rv: f64 = *var_bgidls_rv_slot;
        let mut var_bov: f64 = *var_bov_slot;
        let mut var_bov_d: f64 = *var_bov_d_slot;
        let mut var_bov_d_rv: f64 = *var_bov_d_rv_slot;
        let mut var_bov_rv: f64 = *var_bov_rv_slot;
        let mut var_bphiedge: f64 = *var_bphiedge_slot;
        let mut var_bphiedge_rv: f64 = *var_bphiedge_rv_slot;
        let mut var_gcq: f64 = *var_gcq_slot;
        let mut var_gcq_rv: f64 = *var_gcq_rv_slot;
        let mut var_gcqov: f64 = *var_gcqov_slot;
        let mut var_gcqov_rv: f64 = *var_gcqov_rv_slot;
        let mut var_gcqovd: f64 = *var_gcqovd_slot;
        let mut var_gcqovd_rv: f64 = *var_gcqovd_rv_slot;
        let mut var_gfedge: f64 = *var_gfedge_slot;
        let mut var_gfedge2: f64 = *var_gfedge2_slot;
        let mut var_gfedge2_rv: f64 = *var_gfedge2_rv_slot;
        let mut var_gfedge_rv: f64 = *var_gfedge_rv_slot;
        let mut var_guard161: f64 = *var_guard161_slot;
        let mut var_guard161_rv: f64 = *var_guard161_rv_slot;
        let mut var_guard162: f64 = *var_guard162_slot;
        let mut var_guard162_rv: f64 = *var_guard162_rv_slot;
        let mut var_guard163: f64 = *var_guard163_slot;
        let mut var_guard163_rv: f64 = *var_guard163_rv_slot;
        let mut var_guard164: f64 = *var_guard164_slot;
        let mut var_guard164_rv: f64 = *var_guard164_rv_slot;
        let mut var_guard172: f64 = *var_guard172_slot;
        let mut var_guard172_rv: f64 = *var_guard172_rv_slot;
        let mut var_guard173: f64 = *var_guard173_slot;
        let mut var_guard173_rv: f64 = *var_guard173_rv_slot;
        let mut var_iginv_i: f64 = *var_iginv_i_slot;
        let mut var_iginv_i_rv: f64 = *var_iginv_i_rv_slot;
        let mut var_igov_i: f64 = *var_igov_i_slot;
        let mut var_igov_i_rv: f64 = *var_igov_i_rv_slot;
        let mut var_igovd_i: f64 = *var_igovd_i_slot;
        let mut var_igovd_i_rv: f64 = *var_igovd_i_rv_slot;
        let mut var_inv_chib: f64 = *var_inv_chib_slot;
        let mut var_inv_chib_rv: f64 = *var_inv_chib_rv_slot;
        let mut var_jwcorr: f64 = *var_jwcorr_slot;
        let mut var_jwcorr_rv: f64 = *var_jwcorr_rv_slot;
        let mut var_jww: f64 = *var_jww_slot;
        let mut var_jww_rv: f64 = *var_jww_rv_slot;
        let mut var_lgd_i: f64 = *var_lgd_i_slot;
        let mut var_lgd_i_rv: f64 = *var_lgd_i_rv_slot;
        let mut var_lgs_i: f64 = *var_lgs_i_slot;
        let mut var_lgs_i_rv: f64 = *var_lgs_i_rv_slot;
        let mut var_lngfedge2: f64 = *var_lngfedge2_slot;
        let mut var_lngfedge2_rv: f64 = *var_lngfedge2_rv_slot;
        let mut var_lsd_i: f64 = *var_lsd_i_slot;
        let mut var_lsd_i_rv: f64 = *var_lsd_i_rv_slot;
        let mut var_lss_i: f64 = *var_lss_i_slot;
        let mut var_lss_i_rv: f64 = *var_lss_i_rv_slot;
        let mut var_phibedge: f64 = *var_phibedge_slot;
        let mut var_phibedge_rv: f64 = *var_phibedge_rv_slot;
        let mut var_phit0edge: f64 = *var_phit0edge_slot;
        let mut var_phit0edge_rv: f64 = *var_phit0edge_rv_slot;
        let mut var_phix1edge: f64 = *var_phix1edge_slot;
        let mut var_phix1edge_rv: f64 = *var_phix1edge_rv_slot;
        let mut var_phix2edge: f64 = *var_phix2edge_slot;
        let mut var_phix2edge_rv: f64 = *var_phix2edge_rv_slot;
        let mut var_phixedge: f64 = *var_phixedge_slot;
        let mut var_phixedge_rv: f64 = *var_phixedge_rv_slot;
        let mut var_tf_betedge: f64 = *var_tf_betedge_slot;
        let mut var_tf_betedge_rv: f64 = *var_tf_betedge_rv_slot;
        let mut var_tf_ig: f64 = *var_tf_ig_slot;
        let mut var_tf_ig_rv: f64 = *var_tf_ig_rv_slot;
        let mut var_vfbedge_t: f64 = *var_vfbedge_t_slot;
        let mut var_vfbedge_t_rv: f64 = *var_vfbedge_t_rv_slot;
        let mut var_vinr_max: f64 = *var_vinr_max_slot;
        let mut var_vinr_max_rv: f64 = *var_vinr_max_rv_slot;

        let (assign12270_e11005,) = {
    if (var_guard160 != 0.0) {
        let assign12270_e10989: f64 = (var_eg + var_dphibedge_i);
        let assign12270_e10992: f64 = (2.0 * var_phit0edge);
        let assign12270_e10996: f64 = (-0.75);
        let assign12270_e10997: f64 = (var_phibfac).powf(assign12270_e10996);
        let assign12270_e10998: f64 = (var_neffedge_i * assign12270_e10997);
        let assign12270_e11000: f64 = (assign12270_e10998 * 4e-26);
        let assign12270_e11001: f64 = (assign12270_e11000).ln();
        let assign12270_e11002: f64 = (assign12270_e10992 * assign12270_e11001);
        let assign12270_e11003: f64 = (assign12270_e10989 + assign12270_e11002);
        (assign12270_e11003,)
    } else {
        (var_phibedge,)
    }
};
        var_phibedge = assign12270_e11005;
        var_phibedge_rv = 0.0;

        let (assign12280_e11014,) = {
    if (var_guard160 != 0.0) {
        let (assign12280_e11012,) = {
            if (var_phibedge > 0.05) {
                (var_phibedge,)
            } else {
                (0.05,)
            }
        };
        (assign12280_e11012,)
    } else {
        (var_phibedge,)
    }
};
        var_phibedge = assign12280_e11014;
        var_phibedge_rv = 0.0;

        let (assign12290_e11029,) = {
    if (var_guard160 != 0.0) {
        let assign12290_e11018: f64 = (2.0 * 1.6021918e-19);
        let assign12290_e11020: f64 = (assign12290_e11018 * var_neffedge_i);
        let assign12290_e11022: f64 = (assign12290_e11020 * var_epssi);
        let assign12290_e11024: f64 = (assign12290_e11022 * var_inv_phit);
        let assign12290_e11025: f64 = (assign12290_e11024).sqrt();
        let assign12290_e11027: f64 = (assign12290_e11025 / var_coxprime);
        (assign12290_e11027,)
    } else {
        (var_gfedge,)
    }
};
        var_gfedge = assign12290_e11029;
        var_gfedge_rv = 0.0;

        let (assign12300_e11035,) = {
    if (var_guard160 != 0.0) {
        let assign12300_e11033: f64 = (var_gfedge * var_gfedge);
        (assign12300_e11033,)
    } else {
        (var_gfedge2,)
    }
};
        var_gfedge2 = assign12300_e11035;
        var_gfedge2_rv = 0.0;

        let (assign12310_e11040,) = {
    if (var_guard160 != 0.0) {
        let assign12310_e11038: f64 = (var_gfedge2).ln();
        (assign12310_e11038,)
    } else {
        (var_lngfedge2,)
    }
};
        var_lngfedge2 = assign12310_e11040;
        var_lngfedge2_rv = 0.0;

        let (assign12320_e11046,) = {
    if (var_guard160 != 0.0) {
        let assign12320_e11044: f64 = (0.95 * var_phibedge);
        (assign12320_e11044,)
    } else {
        (var_phixedge,)
    }
};
        var_phixedge = assign12320_e11046;
        var_phixedge_rv = 0.0;

        let (assign12330_e11054,) = {
    if (var_guard160 != 0.0) {
        let assign12330_e11050: f64 = (0.0025 * var_phibedge);
        let assign12330_e11052: f64 = (assign12330_e11050 * var_phibedge);
        (assign12330_e11052,)
    } else {
        (var_aphiedge,)
    }
};
        var_aphiedge = assign12330_e11054;
        var_aphiedge_rv = 0.0;

        let (assign12340_e11058,) = {
    if (var_guard160 != 0.0) {
        (var_aphiedge,)
    } else {
        (var_bphiedge,)
    }
};
        var_bphiedge = assign12340_e11058;
        var_bphiedge_rv = 0.0;

        let (assign12350_e11065,) = {
    if (var_guard160 != 0.0) {
        let assign12350_e11062: f64 = (var_bphiedge).sqrt();
        let assign12350_e11063: f64 = (0.5 * assign12350_e11062);
        (assign12350_e11063,)
    } else {
        (var_phix2edge,)
    }
};
        var_phix2edge = assign12350_e11065;
        var_phix2edge_rv = 0.0;

        let (assign12360_e11090,) = {
    if (var_guard160 != 0.0) {
        let assign12360_e11070: f64 = (var_phixedge - var_phix2edge);
        let assign12360_e11072: f64 = assign12360_e11070;
        let assign12360_e11075: f64 = (var_phixedge - var_phix2edge);
        let assign12360_e11077: f64 = assign12360_e11075;
        let assign12360_e11080: f64 = (var_phixedge - var_phix2edge);
        let assign12360_e11082: f64 = assign12360_e11080;
        let assign12360_e11083: f64 = (assign12360_e11077 * assign12360_e11082);
        let assign12360_e11085: f64 = (assign12360_e11083 + var_aphiedge);
        let assign12360_e11086: f64 = (assign12360_e11085).sqrt();
        let assign12360_e11087: f64 = (assign12360_e11072 - assign12360_e11086);
        let assign12360_e11088: f64 = (0.5 * assign12360_e11087);
        (assign12360_e11088,)
    } else {
        (var_phix1edge,)
    }
};
        var_phix1edge = assign12360_e11090;
        var_phix1edge_rv = 0.0;

        let (assign12390_e11115,) = {
    if (var_guard160 == 0.0) {
        (0.0,)
    } else {
        (var_vfbedge_t,)
    }
};
        var_vfbedge_t = assign12390_e11115;
        var_vfbedge_t_rv = 0.0;

        let (assign12400_e11120,) = {
    if (var_guard160 == 0.0) {
        (1.0,)
    } else {
        (var_tf_betedge,)
    }
};
        var_tf_betedge = assign12400_e11120;
        var_tf_betedge_rv = 0.0;

        let (assign12410_e11125,) = {
    if (var_guard160 == 0.0) {
        (0.0,)
    } else {
        (var_betnedge_t,)
    }
};
        var_betnedge_t = assign12410_e11125;
        var_betnedge_t_rv = 0.0;

        let (assign12420_e11130,) = {
    if (var_guard160 == 0.0) {
        (0.0,)
    } else {
        (var_betedge_i,)
    }
};
        var_betedge_i = assign12420_e11130;
        var_betedge_i_rv = 0.0;

        let (assign12430_e11135,) = {
    if (var_guard160 == 0.0) {
        (var_phit,)
    } else {
        (var_phit0edge,)
    }
};
        var_phit0edge = assign12430_e11135;
        var_phit0edge_rv = 0.0;

        let (assign12440_e11140,) = {
    if (var_guard160 == 0.0) {
        (0.0,)
    } else {
        (var_phibedge,)
    }
};
        var_phibedge = assign12440_e11140;
        var_phibedge_rv = 0.0;

        let (assign12450_e11145,) = {
    if (var_guard160 == 0.0) {
        (1.0,)
    } else {
        (var_gfedge,)
    }
};
        var_gfedge = assign12450_e11145;
        var_gfedge_rv = 0.0;

        let (assign12460_e11150,) = {
    if (var_guard160 == 0.0) {
        (1.0,)
    } else {
        (var_gfedge2,)
    }
};
        var_gfedge2 = assign12460_e11150;
        var_gfedge2_rv = 0.0;

        let (assign12470_e11155,) = {
    if (var_guard160 == 0.0) {
        (0.0,)
    } else {
        (var_lngfedge2,)
    }
};
        var_lngfedge2 = assign12470_e11155;
        var_lngfedge2_rv = 0.0;

        let (assign12480_e11160,) = {
    if (var_guard160 == 0.0) {
        (0.0,)
    } else {
        (var_phixedge,)
    }
};
        var_phixedge = assign12480_e11160;
        var_phixedge_rv = 0.0;

        let (assign12490_e11165,) = {
    if (var_guard160 == 0.0) {
        (0.0,)
    } else {
        (var_aphiedge,)
    }
};
        var_aphiedge = assign12490_e11165;
        var_aphiedge_rv = 0.0;

        let (assign12500_e11170,) = {
    if (var_guard160 == 0.0) {
        (0.0,)
    } else {
        (var_bphiedge,)
    }
};
        var_bphiedge = assign12500_e11170;
        var_bphiedge_rv = 0.0;

        let (assign12510_e11175,) = {
    if (var_guard160 == 0.0) {
        (0.0,)
    } else {
        (var_phix2edge,)
    }
};
        var_phix2edge = assign12510_e11175;
        var_phix2edge_rv = 0.0;

        let (assign12520_e11180,) = {
    if (var_guard160 == 0.0) {
        (0.0,)
    } else {
        (var_phix1edge,)
    }
};
        var_phix1edge = assign12520_e11180;
        var_phix1edge_rv = 0.0;

        let assign12550_e11193: f64 = (1.0 / var_chib_i);
        var_inv_chib = assign12550_e11193;
        var_inv_chib_rv = 0.0;

        let assign12560_e11196: f64 = (4.0 * 0.3333333333333333);
        let assign12560_e11199: f64 = (2.0 * 1.6021918e-19);
        let assign12560_e11201: f64 = (assign12560_e11199 * 9.1093826e-31);
        let assign12560_e11203: f64 = (assign12560_e11201 * var_chib_i);
        let assign12560_e11204: f64 = (assign12560_e11203).sqrt();
        let assign12560_e11205: f64 = (assign12560_e11196 * assign12560_e11204);
        let assign12560_e11207: f64 = (assign12560_e11205 / 1.05457168e-34);
        var_b_fact = assign12560_e11207;
        var_b_fact_rv = 0.0;

        let assign12570_e11210: f64 = (var_b_fact * var_tox_i);
        var_bch = assign12570_e11210;
        var_bch_rv = 0.0;

        let assign12580_e11213: f64 = (var_b_fact * var_toxov_i);
        var_bov = assign12580_e11213;
        var_bov_rv = 0.0;

        let assign12590_e11216: f64 = (var_b_fact * var_toxovd_i);
        var_bov_d = assign12590_e11216;
        var_bov_d_rv = 0.0;

        var_gcq = 0.0;
        var_gcq_rv = 0.0;

        let assign12610_e11220: f64 = if var_gc3_i < 0.0 { 1.0 } else { 0.0 };
        var_guard161 = assign12610_e11220;
        var_guard161_rv = 0.0;

        let (assign12620_e11229,) = {
    if (var_guard161 != 0.0) {
        let assign12620_e11223: f64 = (-0.495);
        let assign12620_e11225: f64 = (assign12620_e11223 * var_gc2_i);
        let assign12620_e11227: f64 = (assign12620_e11225 / var_gc3_i);
        (assign12620_e11227,)
    } else {
        (var_gcq,)
    }
};
        var_gcq = assign12620_e11229;
        var_gcq_rv = 0.0;

        var_gcqov = 0.0;
        var_gcqov_rv = 0.0;

        let assign12640_e11233: f64 = if var_gc3ov_i < 0.0 { 1.0 } else { 0.0 };
        var_guard162 = assign12640_e11233;
        var_guard162_rv = 0.0;

        let (assign12650_e11242,) = {
    if (var_guard162 != 0.0) {
        let assign12650_e11236: f64 = (-0.495);
        let assign12650_e11238: f64 = (assign12650_e11236 * var_gc2ov_i);
        let assign12650_e11240: f64 = (assign12650_e11238 / var_gc3ov_i);
        (assign12650_e11240,)
    } else {
        (var_gcqov,)
    }
};
        var_gcqov = assign12650_e11242;
        var_gcqov_rv = 0.0;

        let assign12660_e11245: f64 = if var_gc3ovd_i < 0.0 { 1.0 } else { 0.0 };
        var_guard163 = assign12660_e11245;
        var_guard163_rv = 0.0;

        let (assign12670_e11254,) = {
    if (var_guard163 != 0.0) {
        let assign12670_e11248: f64 = (-0.495);
        let assign12670_e11250: f64 = (assign12670_e11248 * var_gc2ovd_i);
        let assign12670_e11252: f64 = (assign12670_e11250 / var_gc3ovd_i);
        (assign12670_e11252,)
    } else {
        (var_gcqovd,)
    }
};
        var_gcqovd = assign12670_e11254;
        var_gcqovd_rv = 0.0;

        let assign12680_e11257: f64 = (var_rta).powf(var_stig_i);
        var_tf_ig = assign12680_e11257;
        var_tf_ig_rv = 0.0;

        let assign12690_e11260: f64 = (var_iginv_i * var_tf_ig);
        var_iginv_i = assign12690_e11260;
        var_iginv_i_rv = 0.0;

        let assign12700_e11263: f64 = (var_igov_i * var_tf_ig);
        var_igov_i = assign12700_e11263;
        var_igov_i_rv = 0.0;

        let assign12710_e11266: f64 = (var_igovd_i * var_tf_ig);
        var_igovd_i = assign12710_e11266;
        var_igovd_i_rv = 0.0;

        let assign12740_e11284: f64 = (var_stbgidl_i * var_delta);
        let assign12740_e11285: f64 = (1.0 + assign12740_e11284);
        let (assign12740_e11294,) = {
    if (assign12740_e11285 > 0.0) {
        let assign12740_e11291: f64 = (var_stbgidl_i * var_delta);
        let assign12740_e11292: f64 = (1.0 + assign12740_e11291);
        (assign12740_e11292,)
    } else {
        (0.0,)
    }
};
        var_b_fact = assign12740_e11294;
        var_b_fact_rv = 0.0;

        let assign12750_e11297: f64 = (var_bgidl_i * var_b_fact);
        var_bgidl_t = assign12750_e11297;
        var_bgidl_t_rv = 0.0;

        let assign12760_e11300: f64 = (var_bgidl_t * var_toxov_i);
        let assign12760_e11302: f64 = (assign12760_e11300 * 500000000.0);
        var_bgidls = assign12760_e11302;
        var_bgidls_rv = 0.0;

        let assign12770_e11306: f64 = (var_stbgidld_i * var_delta);
        let assign12770_e11307: f64 = (1.0 + assign12770_e11306);
        let (assign12770_e11316,) = {
    if (assign12770_e11307 > 0.0) {
        let assign12770_e11313: f64 = (var_stbgidld_i * var_delta);
        let assign12770_e11314: f64 = (1.0 + assign12770_e11313);
        (assign12770_e11314,)
    } else {
        (0.0,)
    }
};
        var_b_fact = assign12770_e11316;
        var_b_fact_rv = 0.0;

        let assign12780_e11319: f64 = (var_bgidld_i * var_b_fact);
        var_bgidld_t = assign12780_e11319;
        var_bgidld_t_rv = 0.0;

        let assign12790_e11322: f64 = (var_bgidld_t * var_toxovd_i);
        let assign12790_e11324: f64 = (assign12790_e11322 * 500000000.0);
        var_bgidlds = assign12790_e11324;
        var_bgidlds_rv = 0.0;

        var_vinr_max = 0.0;
        var_vinr_max_rv = 0.0;

        let assign12810_e11328: f64 = if var_fcinracc_i > 1e-10 { 1.0 } else { 0.0 };
        var_guard164 = assign12810_e11328;
        var_guard164_rv = 0.0;

        let (assign12820_e11334,) = {
    if (var_guard164 != 0.0) {
        let assign12820_e11332: f64 = (0.75 / var_fcinracc_i);
        (assign12820_e11332,)
    } else {
        (var_vinr_max,)
    }
};
        var_vinr_max = assign12820_e11334;
        var_vinr_max_rv = 0.0;

        let assign12830_e11337: f64 = (var_axinr_i * var_axinr_i);
        var_ainr = assign12830_e11337;
        var_ainr_rv = 0.0;

        let assign13060_e11443: f64 = (var_absource_i * var_invnf);
        var_abs_i = assign13060_e11443;
        var_abs_i_rv = 0.0;

        let assign13070_e11446: f64 = (var_lssource_i * var_invnf);
        var_lss_i = assign13070_e11446;
        var_lss_i_rv = 0.0;

        let assign13080_e11449: f64 = (var_lgsource_i * var_invnf);
        var_lgs_i = assign13080_e11449;
        var_lgs_i_rv = 0.0;

        let assign13090_e11452: f64 = (var_abdrain_i * var_invnf);
        var_abd_i = assign13090_e11452;
        var_abd_i_rv = 0.0;

        let assign13100_e11455: f64 = (var_lsdrain_i * var_invnf);
        var_lsd_i = assign13100_e11455;
        var_lsd_i_rv = 0.0;

        let assign13110_e11458: f64 = (var_lgdrain_i * var_invnf);
        var_lgd_i = assign13110_e11458;
        var_lgd_i_rv = 0.0;

        var_jwcorr = 0.0;
        var_jwcorr_rv = 0.0;

        let assign13130_e11462: f64 = if p.p43 == 3.0 { 1.0 } else { 0.0 };
        var_guard172 = assign13130_e11462;
        var_guard172_rv = 0.0;

        let (assign13140_e11466,) = {
    if (var_guard172 != 0.0) {
        (1.0,)
    } else {
        (var_jwcorr,)
    }
};
        var_jwcorr = assign13140_e11466;
        var_jwcorr_rv = 0.0;

        var_jww = var_we;
        var_jww_rv = 0.0;

        let assign13160_e11470: f64 = if p.p39 == 0.0 { 1.0 } else { 0.0 };
        var_guard173 = assign13160_e11470;
        var_guard173_rv = 0.0;

        let (assign13170_e11479,) = {
    if (var_guard173 != 0.0) {
        let (assign13170_e11477,) = {
            if (var_jw_i > 0.0) {
                (var_jw_i,)
            } else {
                (0.0,)
            }
        };
        (assign13170_e11477,)
    } else {
        (var_jww,)
    }
};
        var_jww = assign13170_e11479;
        var_jww_rv = 0.0;

        *var_abd_i_slot = var_abd_i;
        *var_abd_i_rv_slot = var_abd_i_rv;
        *var_abs_i_slot = var_abs_i;
        *var_abs_i_rv_slot = var_abs_i_rv;
        *var_ainr_slot = var_ainr;
        *var_ainr_rv_slot = var_ainr_rv;
        *var_aphiedge_slot = var_aphiedge;
        *var_aphiedge_rv_slot = var_aphiedge_rv;
        *var_b_fact_slot = var_b_fact;
        *var_b_fact_rv_slot = var_b_fact_rv;
        *var_bch_slot = var_bch;
        *var_bch_rv_slot = var_bch_rv;
        *var_betedge_i_slot = var_betedge_i;
        *var_betedge_i_rv_slot = var_betedge_i_rv;
        *var_betnedge_t_slot = var_betnedge_t;
        *var_betnedge_t_rv_slot = var_betnedge_t_rv;
        *var_bgidl_t_slot = var_bgidl_t;
        *var_bgidl_t_rv_slot = var_bgidl_t_rv;
        *var_bgidld_t_slot = var_bgidld_t;
        *var_bgidld_t_rv_slot = var_bgidld_t_rv;
        *var_bgidlds_slot = var_bgidlds;
        *var_bgidlds_rv_slot = var_bgidlds_rv;
        *var_bgidls_slot = var_bgidls;
        *var_bgidls_rv_slot = var_bgidls_rv;
        *var_bov_slot = var_bov;
        *var_bov_d_slot = var_bov_d;
        *var_bov_d_rv_slot = var_bov_d_rv;
        *var_bov_rv_slot = var_bov_rv;
        *var_bphiedge_slot = var_bphiedge;
        *var_bphiedge_rv_slot = var_bphiedge_rv;
        *var_gcq_slot = var_gcq;
        *var_gcq_rv_slot = var_gcq_rv;
        *var_gcqov_slot = var_gcqov;
        *var_gcqov_rv_slot = var_gcqov_rv;
        *var_gcqovd_slot = var_gcqovd;
        *var_gcqovd_rv_slot = var_gcqovd_rv;
        *var_gfedge_slot = var_gfedge;
        *var_gfedge2_slot = var_gfedge2;
        *var_gfedge2_rv_slot = var_gfedge2_rv;
        *var_gfedge_rv_slot = var_gfedge_rv;
        *var_guard161_slot = var_guard161;
        *var_guard161_rv_slot = var_guard161_rv;
        *var_guard162_slot = var_guard162;
        *var_guard162_rv_slot = var_guard162_rv;
        *var_guard163_slot = var_guard163;
        *var_guard163_rv_slot = var_guard163_rv;
        *var_guard164_slot = var_guard164;
        *var_guard164_rv_slot = var_guard164_rv;
        *var_guard172_slot = var_guard172;
        *var_guard172_rv_slot = var_guard172_rv;
        *var_guard173_slot = var_guard173;
        *var_guard173_rv_slot = var_guard173_rv;
        *var_iginv_i_slot = var_iginv_i;
        *var_iginv_i_rv_slot = var_iginv_i_rv;
        *var_igov_i_slot = var_igov_i;
        *var_igov_i_rv_slot = var_igov_i_rv;
        *var_igovd_i_slot = var_igovd_i;
        *var_igovd_i_rv_slot = var_igovd_i_rv;
        *var_inv_chib_slot = var_inv_chib;
        *var_inv_chib_rv_slot = var_inv_chib_rv;
        *var_jwcorr_slot = var_jwcorr;
        *var_jwcorr_rv_slot = var_jwcorr_rv;
        *var_jww_slot = var_jww;
        *var_jww_rv_slot = var_jww_rv;
        *var_lgd_i_slot = var_lgd_i;
        *var_lgd_i_rv_slot = var_lgd_i_rv;
        *var_lgs_i_slot = var_lgs_i;
        *var_lgs_i_rv_slot = var_lgs_i_rv;
        *var_lngfedge2_slot = var_lngfedge2;
        *var_lngfedge2_rv_slot = var_lngfedge2_rv;
        *var_lsd_i_slot = var_lsd_i;
        *var_lsd_i_rv_slot = var_lsd_i_rv;
        *var_lss_i_slot = var_lss_i;
        *var_lss_i_rv_slot = var_lss_i_rv;
        *var_phibedge_slot = var_phibedge;
        *var_phibedge_rv_slot = var_phibedge_rv;
        *var_phit0edge_slot = var_phit0edge;
        *var_phit0edge_rv_slot = var_phit0edge_rv;
        *var_phix1edge_slot = var_phix1edge;
        *var_phix1edge_rv_slot = var_phix1edge_rv;
        *var_phix2edge_slot = var_phix2edge;
        *var_phix2edge_rv_slot = var_phix2edge_rv;
        *var_phixedge_slot = var_phixedge;
        *var_phixedge_rv_slot = var_phixedge_rv;
        *var_tf_betedge_slot = var_tf_betedge;
        *var_tf_betedge_rv_slot = var_tf_betedge_rv;
        *var_tf_ig_slot = var_tf_ig;
        *var_tf_ig_rv_slot = var_tf_ig_rv;
        *var_vfbedge_t_slot = var_vfbedge_t;
        *var_vfbedge_t_rv_slot = var_vfbedge_t_rv;
        *var_vinr_max_slot = var_vinr_max;
        *var_vinr_max_rv_slot = var_vinr_max_rv;
    }

    pub(super) fn stamp_reactive_block_20(
        p: &Parameters,
        var_ad_i: f64,
        var_as_i: f64,
        var_idsatbot: f64,
        var_idsatgat: f64,
        var_idsatsti: f64,
        var_invnf: f64,
        var_jwcorr: f64,
        var_jww: f64,
        var_pd_i: f64,
        var_phitd: f64,
        var_phitdinv: f64,
        var_ps_i: f64,
        var_vbibot: f64,
        var_vbigat: f64,
        var_vbisti: f64,
        var_abd_i_slot: &mut f64,
        var_abd_i_rv_slot: &mut f64,
        var_abdrain_i_slot: &mut f64,
        var_abdrain_i_rv_slot: &mut f64,
        var_abs_i_slot: &mut f64,
        var_abs_i_rv_slot: &mut f64,
        var_absource_i_slot: &mut f64,
        var_absource_i_rv_slot: &mut f64,
        var_exp_vmax_over_phitd_d_slot: &mut f64,
        var_exp_vmax_over_phitd_d_rv_slot: &mut f64,
        var_exp_vmax_over_phitd_s_slot: &mut f64,
        var_exp_vmax_over_phitd_s_rv_slot: &mut f64,
        var_guard174_slot: &mut f64,
        var_guard174_rv_slot: &mut f64,
        var_guard175_slot: &mut f64,
        var_guard175_rv_slot: &mut f64,
        var_guard176_slot: &mut f64,
        var_guard176_rv_slot: &mut f64,
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
        var_lgd_i_slot: &mut f64,
        var_lgd_i_rv_slot: &mut f64,
        var_lgdrain_i_slot: &mut f64,
        var_lgdrain_i_rv_slot: &mut f64,
        var_lgs_i_slot: &mut f64,
        var_lgs_i_rv_slot: &mut f64,
        var_lgsource_i_slot: &mut f64,
        var_lgsource_i_rv_slot: &mut f64,
        var_lsd_i_slot: &mut f64,
        var_lsd_i_rv_slot: &mut f64,
        var_lsdrain_i_slot: &mut f64,
        var_lsdrain_i_rv_slot: &mut f64,
        var_lss_i_slot: &mut f64,
        var_lss_i_rv_slot: &mut f64,
        var_lssource_i_slot: &mut f64,
        var_lssource_i_rv_slot: &mut f64,
        var_vbbtlim_d_slot: &mut f64,
        var_vbbtlim_d_rv_slot: &mut f64,
        var_vbbtlim_s_slot: &mut f64,
        var_vbbtlim_s_rv_slot: &mut f64,
        var_vbibot2_slot: &mut f64,
        var_vbibot2_rv_slot: &mut f64,
        var_vbigat2_slot: &mut f64,
        var_vbigat2_rv_slot: &mut f64,
        var_vbimin_d_slot: &mut f64,
        var_vbimin_d_rv_slot: &mut f64,
        var_vbimin_s_slot: &mut f64,
        var_vbimin_s_rv_slot: &mut f64,
        var_vbisti2_slot: &mut f64,
        var_vbisti2_rv_slot: &mut f64,
        var_vch_d_slot: &mut f64,
        var_vch_d_rv_slot: &mut f64,
        var_vch_s_slot: &mut f64,
        var_vch_s_rv_slot: &mut f64,
        var_vfmin_d_slot: &mut f64,
        var_vfmin_d_rv_slot: &mut f64,
        var_vfmin_s_slot: &mut f64,
        var_vfmin_s_rv_slot: &mut f64,
        var_vmax_d_slot: &mut f64,
        var_vmax_d_rv_slot: &mut f64,
        var_vmax_s_slot: &mut f64,
        var_vmax_s_rv_slot: &mut f64,
        var_vmaxbot_slot: &mut f64,
        var_vmaxbot_rv_slot: &mut f64,
        var_vmaxgat_slot: &mut f64,
        var_vmaxgat_rv_slot: &mut f64,
        var_vmaxsti_slot: &mut f64,
        var_vmaxsti_rv_slot: &mut f64,
        var_zflagbot_d_slot: &mut f64,
        var_zflagbot_d_rv_slot: &mut f64,
        var_zflagbot_s_slot: &mut f64,
        var_zflagbot_s_rv_slot: &mut f64,
        var_zflaggat_d_slot: &mut f64,
        var_zflaggat_d_rv_slot: &mut f64,
        var_zflaggat_s_slot: &mut f64,
        var_zflaggat_s_rv_slot: &mut f64,
        var_zflagsti_d_slot: &mut f64,
        var_zflagsti_d_rv_slot: &mut f64,
        var_zflagsti_s_slot: &mut f64,
        var_zflagsti_s_rv_slot: &mut f64,
        var_zfrac_slot: &mut f64,
        var_zfrac_rv_slot: &mut f64,
    ) {
        let mut var_abd_i: f64 = *var_abd_i_slot;
        let mut var_abd_i_rv: f64 = *var_abd_i_rv_slot;
        let mut var_abdrain_i: f64 = *var_abdrain_i_slot;
        let mut var_abdrain_i_rv: f64 = *var_abdrain_i_rv_slot;
        let mut var_abs_i: f64 = *var_abs_i_slot;
        let mut var_abs_i_rv: f64 = *var_abs_i_rv_slot;
        let mut var_absource_i: f64 = *var_absource_i_slot;
        let mut var_absource_i_rv: f64 = *var_absource_i_rv_slot;
        let mut var_exp_vmax_over_phitd_d: f64 = *var_exp_vmax_over_phitd_d_slot;
        let mut var_exp_vmax_over_phitd_d_rv: f64 = *var_exp_vmax_over_phitd_d_rv_slot;
        let mut var_exp_vmax_over_phitd_s: f64 = *var_exp_vmax_over_phitd_s_slot;
        let mut var_exp_vmax_over_phitd_s_rv: f64 = *var_exp_vmax_over_phitd_s_rv_slot;
        let mut var_guard174: f64 = *var_guard174_slot;
        let mut var_guard174_rv: f64 = *var_guard174_rv_slot;
        let mut var_guard175: f64 = *var_guard175_slot;
        let mut var_guard175_rv: f64 = *var_guard175_rv_slot;
        let mut var_guard176: f64 = *var_guard176_slot;
        let mut var_guard176_rv: f64 = *var_guard176_rv_slot;
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
        let mut var_lgd_i: f64 = *var_lgd_i_slot;
        let mut var_lgd_i_rv: f64 = *var_lgd_i_rv_slot;
        let mut var_lgdrain_i: f64 = *var_lgdrain_i_slot;
        let mut var_lgdrain_i_rv: f64 = *var_lgdrain_i_rv_slot;
        let mut var_lgs_i: f64 = *var_lgs_i_slot;
        let mut var_lgs_i_rv: f64 = *var_lgs_i_rv_slot;
        let mut var_lgsource_i: f64 = *var_lgsource_i_slot;
        let mut var_lgsource_i_rv: f64 = *var_lgsource_i_rv_slot;
        let mut var_lsd_i: f64 = *var_lsd_i_slot;
        let mut var_lsd_i_rv: f64 = *var_lsd_i_rv_slot;
        let mut var_lsdrain_i: f64 = *var_lsdrain_i_slot;
        let mut var_lsdrain_i_rv: f64 = *var_lsdrain_i_rv_slot;
        let mut var_lss_i: f64 = *var_lss_i_slot;
        let mut var_lss_i_rv: f64 = *var_lss_i_rv_slot;
        let mut var_lssource_i: f64 = *var_lssource_i_slot;
        let mut var_lssource_i_rv: f64 = *var_lssource_i_rv_slot;
        let mut var_vbbtlim_d: f64 = *var_vbbtlim_d_slot;
        let mut var_vbbtlim_d_rv: f64 = *var_vbbtlim_d_rv_slot;
        let mut var_vbbtlim_s: f64 = *var_vbbtlim_s_slot;
        let mut var_vbbtlim_s_rv: f64 = *var_vbbtlim_s_rv_slot;
        let mut var_vbibot2: f64 = *var_vbibot2_slot;
        let mut var_vbibot2_rv: f64 = *var_vbibot2_rv_slot;
        let mut var_vbigat2: f64 = *var_vbigat2_slot;
        let mut var_vbigat2_rv: f64 = *var_vbigat2_rv_slot;
        let mut var_vbimin_d: f64 = *var_vbimin_d_slot;
        let mut var_vbimin_d_rv: f64 = *var_vbimin_d_rv_slot;
        let mut var_vbimin_s: f64 = *var_vbimin_s_slot;
        let mut var_vbimin_s_rv: f64 = *var_vbimin_s_rv_slot;
        let mut var_vbisti2: f64 = *var_vbisti2_slot;
        let mut var_vbisti2_rv: f64 = *var_vbisti2_rv_slot;
        let mut var_vch_d: f64 = *var_vch_d_slot;
        let mut var_vch_d_rv: f64 = *var_vch_d_rv_slot;
        let mut var_vch_s: f64 = *var_vch_s_slot;
        let mut var_vch_s_rv: f64 = *var_vch_s_rv_slot;
        let mut var_vfmin_d: f64 = *var_vfmin_d_slot;
        let mut var_vfmin_d_rv: f64 = *var_vfmin_d_rv_slot;
        let mut var_vfmin_s: f64 = *var_vfmin_s_slot;
        let mut var_vfmin_s_rv: f64 = *var_vfmin_s_rv_slot;
        let mut var_vmax_d: f64 = *var_vmax_d_slot;
        let mut var_vmax_d_rv: f64 = *var_vmax_d_rv_slot;
        let mut var_vmax_s: f64 = *var_vmax_s_slot;
        let mut var_vmax_s_rv: f64 = *var_vmax_s_rv_slot;
        let mut var_vmaxbot: f64 = *var_vmaxbot_slot;
        let mut var_vmaxbot_rv: f64 = *var_vmaxbot_rv_slot;
        let mut var_vmaxgat: f64 = *var_vmaxgat_slot;
        let mut var_vmaxgat_rv: f64 = *var_vmaxgat_rv_slot;
        let mut var_vmaxsti: f64 = *var_vmaxsti_slot;
        let mut var_vmaxsti_rv: f64 = *var_vmaxsti_rv_slot;
        let mut var_zflagbot_d: f64 = *var_zflagbot_d_slot;
        let mut var_zflagbot_d_rv: f64 = *var_zflagbot_d_rv_slot;
        let mut var_zflagbot_s: f64 = *var_zflagbot_s_slot;
        let mut var_zflagbot_s_rv: f64 = *var_zflagbot_s_rv_slot;
        let mut var_zflaggat_d: f64 = *var_zflaggat_d_slot;
        let mut var_zflaggat_d_rv: f64 = *var_zflaggat_d_rv_slot;
        let mut var_zflaggat_s: f64 = *var_zflaggat_s_slot;
        let mut var_zflaggat_s_rv: f64 = *var_zflaggat_s_rv_slot;
        let mut var_zflagsti_d: f64 = *var_zflagsti_d_slot;
        let mut var_zflagsti_d_rv: f64 = *var_zflagsti_d_rv_slot;
        let mut var_zflagsti_s: f64 = *var_zflagsti_s_slot;
        let mut var_zflagsti_s_rv: f64 = *var_zflagsti_s_rv_slot;
        let mut var_zfrac: f64 = *var_zfrac_slot;
        let mut var_zfrac_rv: f64 = *var_zfrac_rv_slot;

        let assign13180_e11486: f64 = if ((p.p43 == 2.0) || (p.p43 == 3.0)) { 1.0 } else { 0.0 };
        var_guard174 = assign13180_e11486;
        var_guard174_rv = 0.0;

        let (assign13190_e11492,) = {
    if (var_guard174 != 0.0) {
        let assign13190_e11490: f64 = (var_as_i * var_invnf);
        (assign13190_e11490,)
    } else {
        (var_abs_i,)
    }
};
        var_abs_i = assign13190_e11492;
        var_abs_i_rv = 0.0;

        let (assign13200_e11502,) = {
    if (var_guard174 != 0.0) {
        let assign13200_e11496: f64 = (var_ps_i * var_invnf);
        let assign13200_e11499: f64 = (var_jwcorr * var_jww);
        let assign13200_e11500: f64 = (assign13200_e11496 - assign13200_e11499);
        (assign13200_e11500,)
    } else {
        (var_lss_i,)
    }
};
        var_lss_i = assign13200_e11502;
        var_lss_i_rv = 0.0;

        let (assign13210_e11506,) = {
    if (var_guard174 != 0.0) {
        (var_jww,)
    } else {
        (var_lgs_i,)
    }
};
        var_lgs_i = assign13210_e11506;
        var_lgs_i_rv = 0.0;

        let (assign13220_e11512,) = {
    if (var_guard174 != 0.0) {
        let assign13220_e11510: f64 = (var_ad_i * var_invnf);
        (assign13220_e11510,)
    } else {
        (var_abd_i,)
    }
};
        var_abd_i = assign13220_e11512;
        var_abd_i_rv = 0.0;

        let (assign13230_e11522,) = {
    if (var_guard174 != 0.0) {
        let assign13230_e11516: f64 = (var_pd_i * var_invnf);
        let assign13230_e11519: f64 = (var_jwcorr * var_jww);
        let assign13230_e11520: f64 = (assign13230_e11516 - assign13230_e11519);
        (assign13230_e11520,)
    } else {
        (var_lsd_i,)
    }
};
        var_lsd_i = assign13230_e11522;
        var_lsd_i_rv = 0.0;

        let (assign13240_e11526,) = {
    if (var_guard174 != 0.0) {
        (var_jww,)
    } else {
        (var_lgd_i,)
    }
};
        var_lgd_i = assign13240_e11526;
        var_lgd_i_rv = 0.0;

        let assign13250_e11537: f64 = if (((p.p43 == 1.0) || (p.p43 == 2.0)) || (p.p43 == 3.0)) { 1.0 } else { 0.0 };
        var_guard175 = assign13250_e11537;
        var_guard175_rv = 0.0;

        let (assign13260_e11546,) = {
    if (var_guard175 != 0.0) {
        let (assign13260_e11544,) = {
            if (var_abs_i > 0.0) {
                (var_abs_i,)
            } else {
                (0.0,)
            }
        };
        (assign13260_e11544,)
    } else {
        (var_absource_i,)
    }
};
        var_absource_i = assign13260_e11546;
        var_absource_i_rv = 0.0;

        let (assign13270_e11555,) = {
    if (var_guard175 != 0.0) {
        let (assign13270_e11553,) = {
            if (var_lss_i > 0.0) {
                (var_lss_i,)
            } else {
                (0.0,)
            }
        };
        (assign13270_e11553,)
    } else {
        (var_lssource_i,)
    }
};
        var_lssource_i = assign13270_e11555;
        var_lssource_i_rv = 0.0;

        let (assign13280_e11564,) = {
    if (var_guard175 != 0.0) {
        let (assign13280_e11562,) = {
            if (var_lgs_i > 0.0) {
                (var_lgs_i,)
            } else {
                (0.0,)
            }
        };
        (assign13280_e11562,)
    } else {
        (var_lgsource_i,)
    }
};
        var_lgsource_i = assign13280_e11564;
        var_lgsource_i_rv = 0.0;

        let (assign13290_e11573,) = {
    if (var_guard175 != 0.0) {
        let (assign13290_e11571,) = {
            if (var_abd_i > 0.0) {
                (var_abd_i,)
            } else {
                (0.0,)
            }
        };
        (assign13290_e11571,)
    } else {
        (var_abdrain_i,)
    }
};
        var_abdrain_i = assign13290_e11573;
        var_abdrain_i_rv = 0.0;

        let (assign13300_e11582,) = {
    if (var_guard175 != 0.0) {
        let (assign13300_e11580,) = {
            if (var_lsd_i > 0.0) {
                (var_lsd_i,)
            } else {
                (0.0,)
            }
        };
        (assign13300_e11580,)
    } else {
        (var_lsdrain_i,)
    }
};
        var_lsdrain_i = assign13300_e11582;
        var_lsdrain_i_rv = 0.0;

        let (assign13310_e11591,) = {
    if (var_guard175 != 0.0) {
        let (assign13310_e11589,) = {
            if (var_lgd_i > 0.0) {
                (var_lgd_i,)
            } else {
                (0.0,)
            }
        };
        (assign13310_e11589,)
    } else {
        (var_lgdrain_i,)
    }
};
        var_lgdrain_i = assign13310_e11591;
        var_lgdrain_i_rv = 0.0;

        let (assign13320_e11596,) = {
    if (var_guard175 == 0.0) {
        (0.0,)
    } else {
        (var_absource_i,)
    }
};
        var_absource_i = assign13320_e11596;
        var_absource_i_rv = 0.0;

        let (assign13330_e11601,) = {
    if (var_guard175 == 0.0) {
        (0.0,)
    } else {
        (var_lssource_i,)
    }
};
        var_lssource_i = assign13330_e11601;
        var_lssource_i_rv = 0.0;

        let (assign13340_e11606,) = {
    if (var_guard175 == 0.0) {
        (0.0,)
    } else {
        (var_lgsource_i,)
    }
};
        var_lgsource_i = assign13340_e11606;
        var_lgsource_i_rv = 0.0;

        let (assign13350_e11611,) = {
    if (var_guard175 == 0.0) {
        (0.0,)
    } else {
        (var_abdrain_i,)
    }
};
        var_abdrain_i = assign13350_e11611;
        var_abdrain_i_rv = 0.0;

        let (assign13360_e11616,) = {
    if (var_guard175 == 0.0) {
        (0.0,)
    } else {
        (var_lsdrain_i,)
    }
};
        var_lsdrain_i = assign13360_e11616;
        var_lsdrain_i_rv = 0.0;

        let (assign13370_e11621,) = {
    if (var_guard175 == 0.0) {
        (0.0,)
    } else {
        (var_lgdrain_i,)
    }
};
        var_lgdrain_i = assign13370_e11621;
        var_lgdrain_i_rv = 0.0;

        var_vbimin_s = 0.0;
        var_vbimin_s_rv = 0.0;

        var_vbimin_d = 0.0;
        var_vbimin_d_rv = 0.0;

        var_vfmin_s = 0.0;
        var_vfmin_s_rv = 0.0;

        var_vfmin_d = 0.0;
        var_vfmin_d_rv = 0.0;

        var_vch_s = 0.0;
        var_vch_s_rv = 0.0;

        var_vch_d = 0.0;
        var_vch_d_rv = 0.0;

        var_vbbtlim_s = 0.0;
        var_vbbtlim_s_rv = 0.0;

        var_vbbtlim_d = 0.0;
        var_vbbtlim_d_rv = 0.0;

        var_vmax_s = 0.0;
        var_vmax_s_rv = 0.0;

        var_vmax_d = 0.0;
        var_vmax_d_rv = 0.0;

        var_exp_vmax_over_phitd_s = 0.0;
        var_exp_vmax_over_phitd_s_rv = 0.0;

        var_exp_vmax_over_phitd_d = 0.0;
        var_exp_vmax_over_phitd_d_rv = 0.0;

        var_zflagbot_s = 1.0;
        var_zflagbot_s_rv = 0.0;

        var_zflagbot_d = 1.0;
        var_zflagbot_d_rv = 0.0;

        var_zflagsti_s = 1.0;
        var_zflagsti_s_rv = 0.0;

        var_zflagsti_d = 1.0;
        var_zflagsti_d_rv = 0.0;

        var_zflaggat_s = 1.0;
        var_zflaggat_s_rv = 0.0;

        var_zflaggat_d = 1.0;
        var_zflaggat_d_rv = 0.0;

        var_zfrac = 0.0;
        var_zfrac_rv = 0.0;

        let assign13940_e11680: f64 = if p.p43 > 0.0 { 1.0 } else { 0.0 };
        var_guard176 = assign13940_e11680;
        var_guard176_rv = 0.0;

        let assign13950_e11683: f64 = (var_idsatbot * var_absource_i);
        let assign13950_e11685: f64 = if assign13950_e11683 > 0.0 { 1.0 } else { 0.0 };
        var_guard177 = assign13950_e11685;
        var_guard177_rv = 0.0;

        let (assign13960_e11700,) = {
    if ((var_guard176 != 0.0) && (var_guard177 != 0.0)) {
        let assign13960_e11693: f64 = (var_idsatbot * var_absource_i);
        let assign13960_e11694: f64 = (p.p815 / assign13960_e11693);
        let assign13960_e11696: f64 = (assign13960_e11694 + 1.0);
        let assign13960_e11697: f64 = (assign13960_e11696).ln();
        let assign13960_e11698: f64 = (var_phitd * assign13960_e11697);
        (assign13960_e11698,)
    } else {
        (var_vmaxbot,)
    }
};
        var_vmaxbot = assign13960_e11700;
        var_vmaxbot_rv = 0.0;

        let (assign13970_e11707,) = {
    if ((var_guard176 != 0.0) && (var_guard177 == 0.0)) {
        (100000000.0,)
    } else {
        (var_vmaxbot,)
    }
};
        var_vmaxbot = assign13970_e11707;
        var_vmaxbot_rv = 0.0;

        let assign13980_e11710: f64 = (var_idsatsti * var_lssource_i);
        let assign13980_e11712: f64 = if assign13980_e11710 > 0.0 { 1.0 } else { 0.0 };
        var_guard178 = assign13980_e11712;
        var_guard178_rv = 0.0;

        let (assign13990_e11727,) = {
    if ((var_guard176 != 0.0) && (var_guard178 != 0.0)) {
        let assign13990_e11720: f64 = (var_idsatsti * var_lssource_i);
        let assign13990_e11721: f64 = (p.p815 / assign13990_e11720);
        let assign13990_e11723: f64 = (assign13990_e11721 + 1.0);
        let assign13990_e11724: f64 = (assign13990_e11723).ln();
        let assign13990_e11725: f64 = (var_phitd * assign13990_e11724);
        (assign13990_e11725,)
    } else {
        (var_vmaxsti,)
    }
};
        var_vmaxsti = assign13990_e11727;
        var_vmaxsti_rv = 0.0;

        let (assign14000_e11734,) = {
    if ((var_guard176 != 0.0) && (var_guard178 == 0.0)) {
        (100000000.0,)
    } else {
        (var_vmaxsti,)
    }
};
        var_vmaxsti = assign14000_e11734;
        var_vmaxsti_rv = 0.0;

        let assign14010_e11737: f64 = (var_idsatgat * var_lgsource_i);
        let assign14010_e11739: f64 = if assign14010_e11737 > 0.0 { 1.0 } else { 0.0 };
        var_guard179 = assign14010_e11739;
        var_guard179_rv = 0.0;

        let (assign14020_e11754,) = {
    if ((var_guard176 != 0.0) && (var_guard179 != 0.0)) {
        let assign14020_e11747: f64 = (var_idsatgat * var_lgsource_i);
        let assign14020_e11748: f64 = (p.p815 / assign14020_e11747);
        let assign14020_e11750: f64 = (assign14020_e11748 + 1.0);
        let assign14020_e11751: f64 = (assign14020_e11750).ln();
        let assign14020_e11752: f64 = (var_phitd * assign14020_e11751);
        (assign14020_e11752,)
    } else {
        (var_vmaxgat,)
    }
};
        var_vmaxgat = assign14020_e11754;
        var_vmaxgat_rv = 0.0;

        let (assign14030_e11761,) = {
    if ((var_guard176 != 0.0) && (var_guard179 == 0.0)) {
        (100000000.0,)
    } else {
        (var_vmaxgat,)
    }
};
        var_vmaxgat = assign14030_e11761;
        var_vmaxgat_rv = 0.0;

        let (assign14040_e11769,) = {
    if (var_guard176 != 0.0) {
        let assign14040_e11765: f64 = (var_vmaxbot).min(var_vmaxsti);
        let assign14040_e11767: f64 = (assign14040_e11765).min(var_vmaxgat);
        (assign14040_e11767,)
    } else {
        (var_vmax_s,)
    }
};
        var_vmax_s = assign14040_e11769;
        var_vmax_s_rv = 0.0;

        let assign14050_e11772: f64 = (var_vmax_s * var_phitdinv);
        let assign14050_e11773: f64 = (assign14050_e11772).abs();
        let assign14050_e11775: f64 = if assign14050_e11773 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard180 = assign14050_e11775;
        var_guard180_rv = 0.0;

        let (assign14060_e11784,) = {
    if ((var_guard176 != 0.0) && (var_guard180 != 0.0)) {
        let assign14060_e11781: f64 = (var_vmax_s * var_phitdinv);
        let assign14060_e11782: f64 = (assign14060_e11781).exp();
        (assign14060_e11782,)
    } else {
        (var_exp_vmax_over_phitd_s,)
    }
};
        var_exp_vmax_over_phitd_s = assign14060_e11784;
        var_exp_vmax_over_phitd_s_rv = 0.0;

        let assign14070_e11787: f64 = (var_vmax_s * var_phitdinv);
        let assign14070_e11789: f64 = if assign14070_e11787 < 0.0 { 1.0 } else { 0.0 };
        var_guard181 = assign14070_e11789;
        var_guard181_rv = 0.0;

        let (assign14080_e11829,) = {
    if (((var_guard176 != 0.0) && (var_guard180 == 0.0)) && (var_guard181 != 0.0)) {
        let assign14080_e11799: f64 = (-230.25850929940458);
        let assign14080_e11802: f64 = (var_vmax_s * var_phitdinv);
        let assign14080_e11803: f64 = (assign14080_e11799 - assign14080_e11802);
        let assign14080_e11807: f64 = (-230.25850929940458);
        let assign14080_e11810: f64 = (var_vmax_s * var_phitdinv);
        let assign14080_e11811: f64 = (assign14080_e11807 - assign14080_e11810);
        let assign14080_e11814: f64 = (-230.25850929940458);
        let assign14080_e11817: f64 = (var_vmax_s * var_phitdinv);
        let assign14080_e11818: f64 = (assign14080_e11814 - assign14080_e11817);
        let assign14080_e11820: f64 = (assign14080_e11818 * 0.3333333333333333);
        let assign14080_e11821: f64 = (1.0 + assign14080_e11820);
        let assign14080_e11822: f64 = (assign14080_e11811 * assign14080_e11821);
        let assign14080_e11823: f64 = (0.5 * assign14080_e11822);
        let assign14080_e11824: f64 = (1.0 + assign14080_e11823);
        let assign14080_e11825: f64 = (assign14080_e11803 * assign14080_e11824);
        let assign14080_e11826: f64 = (1.0 + assign14080_e11825);
        let assign14080_e11827: f64 = (1e-100 / assign14080_e11826);
        (assign14080_e11827,)
    } else {
        (var_exp_vmax_over_phitd_s,)
    }
};
        var_exp_vmax_over_phitd_s = assign14080_e11829;
        var_exp_vmax_over_phitd_s_rv = 0.0;

        let (assign14090_e11867,) = {
    if (((var_guard176 != 0.0) && (var_guard180 == 0.0)) && (var_guard181 == 0.0)) {
        let assign14090_e11841: f64 = (var_vmax_s * var_phitdinv);
        let assign14090_e11843: f64 = (assign14090_e11841 - 230.25850929940458);
        let assign14090_e11848: f64 = (var_vmax_s * var_phitdinv);
        let assign14090_e11850: f64 = (assign14090_e11848 - 230.25850929940458);
        let assign14090_e11854: f64 = (var_vmax_s * var_phitdinv);
        let assign14090_e11856: f64 = (assign14090_e11854 - 230.25850929940458);
        let assign14090_e11858: f64 = (assign14090_e11856 * 0.3333333333333333);
        let assign14090_e11859: f64 = (1.0 + assign14090_e11858);
        let assign14090_e11860: f64 = (assign14090_e11850 * assign14090_e11859);
        let assign14090_e11861: f64 = (0.5 * assign14090_e11860);
        let assign14090_e11862: f64 = (1.0 + assign14090_e11861);
        let assign14090_e11863: f64 = (assign14090_e11843 * assign14090_e11862);
        let assign14090_e11864: f64 = (1.0 + assign14090_e11863);
        let assign14090_e11865: f64 = (1e100 * assign14090_e11864);
        (assign14090_e11865,)
    } else {
        (var_exp_vmax_over_phitd_s,)
    }
};
        var_exp_vmax_over_phitd_s = assign14090_e11867;
        var_exp_vmax_over_phitd_s_rv = 0.0;

        let (assign14100_e11871,) = {
    if (var_guard176 != 0.0) {
        (var_vbibot,)
    } else {
        (var_vbibot2,)
    }
};
        var_vbibot2 = assign14100_e11871;
        var_vbibot2_rv = 0.0;

        let (assign14110_e11875,) = {
    if (var_guard176 != 0.0) {
        (var_vbisti,)
    } else {
        (var_vbisti2,)
    }
};
        var_vbisti2 = assign14110_e11875;
        var_vbisti2_rv = 0.0;

        let (assign14120_e11879,) = {
    if (var_guard176 != 0.0) {
        (var_vbigat,)
    } else {
        (var_vbigat2,)
    }
};
        var_vbigat2 = assign14120_e11879;
        var_vbigat2_rv = 0.0;

        *var_abd_i_slot = var_abd_i;
        *var_abd_i_rv_slot = var_abd_i_rv;
        *var_abdrain_i_slot = var_abdrain_i;
        *var_abdrain_i_rv_slot = var_abdrain_i_rv;
        *var_abs_i_slot = var_abs_i;
        *var_abs_i_rv_slot = var_abs_i_rv;
        *var_absource_i_slot = var_absource_i;
        *var_absource_i_rv_slot = var_absource_i_rv;
        *var_exp_vmax_over_phitd_d_slot = var_exp_vmax_over_phitd_d;
        *var_exp_vmax_over_phitd_d_rv_slot = var_exp_vmax_over_phitd_d_rv;
        *var_exp_vmax_over_phitd_s_slot = var_exp_vmax_over_phitd_s;
        *var_exp_vmax_over_phitd_s_rv_slot = var_exp_vmax_over_phitd_s_rv;
        *var_guard174_slot = var_guard174;
        *var_guard174_rv_slot = var_guard174_rv;
        *var_guard175_slot = var_guard175;
        *var_guard175_rv_slot = var_guard175_rv;
        *var_guard176_slot = var_guard176;
        *var_guard176_rv_slot = var_guard176_rv;
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
        *var_lgd_i_slot = var_lgd_i;
        *var_lgd_i_rv_slot = var_lgd_i_rv;
        *var_lgdrain_i_slot = var_lgdrain_i;
        *var_lgdrain_i_rv_slot = var_lgdrain_i_rv;
        *var_lgs_i_slot = var_lgs_i;
        *var_lgs_i_rv_slot = var_lgs_i_rv;
        *var_lgsource_i_slot = var_lgsource_i;
        *var_lgsource_i_rv_slot = var_lgsource_i_rv;
        *var_lsd_i_slot = var_lsd_i;
        *var_lsd_i_rv_slot = var_lsd_i_rv;
        *var_lsdrain_i_slot = var_lsdrain_i;
        *var_lsdrain_i_rv_slot = var_lsdrain_i_rv;
        *var_lss_i_slot = var_lss_i;
        *var_lss_i_rv_slot = var_lss_i_rv;
        *var_lssource_i_slot = var_lssource_i;
        *var_lssource_i_rv_slot = var_lssource_i_rv;
        *var_vbbtlim_d_slot = var_vbbtlim_d;
        *var_vbbtlim_d_rv_slot = var_vbbtlim_d_rv;
        *var_vbbtlim_s_slot = var_vbbtlim_s;
        *var_vbbtlim_s_rv_slot = var_vbbtlim_s_rv;
        *var_vbibot2_slot = var_vbibot2;
        *var_vbibot2_rv_slot = var_vbibot2_rv;
        *var_vbigat2_slot = var_vbigat2;
        *var_vbigat2_rv_slot = var_vbigat2_rv;
        *var_vbimin_d_slot = var_vbimin_d;
        *var_vbimin_d_rv_slot = var_vbimin_d_rv;
        *var_vbimin_s_slot = var_vbimin_s;
        *var_vbimin_s_rv_slot = var_vbimin_s_rv;
        *var_vbisti2_slot = var_vbisti2;
        *var_vbisti2_rv_slot = var_vbisti2_rv;
        *var_vch_d_slot = var_vch_d;
        *var_vch_d_rv_slot = var_vch_d_rv;
        *var_vch_s_slot = var_vch_s;
        *var_vch_s_rv_slot = var_vch_s_rv;
        *var_vfmin_d_slot = var_vfmin_d;
        *var_vfmin_d_rv_slot = var_vfmin_d_rv;
        *var_vfmin_s_slot = var_vfmin_s;
        *var_vfmin_s_rv_slot = var_vfmin_s_rv;
        *var_vmax_d_slot = var_vmax_d;
        *var_vmax_d_rv_slot = var_vmax_d_rv;
        *var_vmax_s_slot = var_vmax_s;
        *var_vmax_s_rv_slot = var_vmax_s_rv;
        *var_vmaxbot_slot = var_vmaxbot;
        *var_vmaxbot_rv_slot = var_vmaxbot_rv;
        *var_vmaxgat_slot = var_vmaxgat;
        *var_vmaxgat_rv_slot = var_vmaxgat_rv;
        *var_vmaxsti_slot = var_vmaxsti;
        *var_vmaxsti_rv_slot = var_vmaxsti_rv;
        *var_zflagbot_d_slot = var_zflagbot_d;
        *var_zflagbot_d_rv_slot = var_zflagbot_d_rv;
        *var_zflagbot_s_slot = var_zflagbot_s;
        *var_zflagbot_s_rv_slot = var_zflagbot_s_rv;
        *var_zflaggat_d_slot = var_zflaggat_d;
        *var_zflaggat_d_rv_slot = var_zflaggat_d_rv;
        *var_zflaggat_s_slot = var_zflaggat_s;
        *var_zflaggat_s_rv_slot = var_zflaggat_s_rv;
        *var_zflagsti_d_slot = var_zflagsti_d;
        *var_zflagsti_d_rv_slot = var_zflagsti_d_rv;
        *var_zflagsti_s_slot = var_zflagsti_s;
        *var_zflagsti_s_rv_slot = var_zflagsti_s_rv;
        *var_zfrac_slot = var_zfrac;
        *var_zfrac_rv_slot = var_zfrac_rv;
    }

    pub(super) fn stamp_reactive_block_21(
        p: &Parameters,
        var_abdrain_i: f64,
        var_absource_i: f64,
        var_guard176: f64,
        var_idsatbot_d: f64,
        var_idsatgat_d: f64,
        var_idsatsti_d: f64,
        var_lgdrain_i: f64,
        var_lgsource_i: f64,
        var_lsdrain_i: f64,
        var_lssource_i: f64,
        var_pbotd_i: f64,
        var_pgatd_i: f64,
        var_phitd: f64,
        var_phitdinv: f64,
        var_pstid_i: f64,
        var_vbibot: f64,
        var_vbibot_d: f64,
        var_vbigat: f64,
        var_vbigat_d: f64,
        var_vbirbotd_i: f64,
        var_vbirgatd_i: f64,
        var_vbirstid_i: f64,
        var_vbisti: f64,
        var_vbisti_d: f64,
        var_exp_vmax_over_phitd_d_slot: &mut f64,
        var_exp_vmax_over_phitd_d_rv_slot: &mut f64,
        var_guard182_slot: &mut f64,
        var_guard182_rv_slot: &mut f64,
        var_guard183_slot: &mut f64,
        var_guard183_rv_slot: &mut f64,
        var_guard184_slot: &mut f64,
        var_guard184_rv_slot: &mut f64,
        var_guard185_slot: &mut f64,
        var_guard185_rv_slot: &mut f64,
        var_guard186_slot: &mut f64,
        var_guard186_rv_slot: &mut f64,
        var_guard187_slot: &mut f64,
        var_guard187_rv_slot: &mut f64,
        var_guard188_slot: &mut f64,
        var_guard188_rv_slot: &mut f64,
        var_guard189_slot: &mut f64,
        var_guard189_rv_slot: &mut f64,
        var_guard190_slot: &mut f64,
        var_guard190_rv_slot: &mut f64,
        var_pbot2_slot: &mut f64,
        var_pbot2_rv_slot: &mut f64,
        var_pgat2_slot: &mut f64,
        var_pgat2_rv_slot: &mut f64,
        var_pmax_slot: &mut f64,
        var_pmax_rv_slot: &mut f64,
        var_psti2_slot: &mut f64,
        var_psti2_rv_slot: &mut f64,
        var_vbbtlim_s_slot: &mut f64,
        var_vbbtlim_s_rv_slot: &mut f64,
        var_vbibot2_slot: &mut f64,
        var_vbibot2_rv_slot: &mut f64,
        var_vbibot2r_slot: &mut f64,
        var_vbibot2r_rv_slot: &mut f64,
        var_vbigat2_slot: &mut f64,
        var_vbigat2_rv_slot: &mut f64,
        var_vbigat2r_slot: &mut f64,
        var_vbigat2r_rv_slot: &mut f64,
        var_vbimin_s_slot: &mut f64,
        var_vbimin_s_rv_slot: &mut f64,
        var_vbisti2_slot: &mut f64,
        var_vbisti2_rv_slot: &mut f64,
        var_vbisti2r_slot: &mut f64,
        var_vbisti2r_rv_slot: &mut f64,
        var_vch_s_slot: &mut f64,
        var_vch_s_rv_slot: &mut f64,
        var_vfmin_s_slot: &mut f64,
        var_vfmin_s_rv_slot: &mut f64,
        var_vmax_d_slot: &mut f64,
        var_vmax_d_rv_slot: &mut f64,
        var_vmaxbot_slot: &mut f64,
        var_vmaxbot_rv_slot: &mut f64,
        var_vmaxgat_slot: &mut f64,
        var_vmaxgat_rv_slot: &mut f64,
        var_vmaxsti_slot: &mut f64,
        var_vmaxsti_rv_slot: &mut f64,
    ) {
        let mut var_exp_vmax_over_phitd_d: f64 = *var_exp_vmax_over_phitd_d_slot;
        let mut var_exp_vmax_over_phitd_d_rv: f64 = *var_exp_vmax_over_phitd_d_rv_slot;
        let mut var_guard182: f64 = *var_guard182_slot;
        let mut var_guard182_rv: f64 = *var_guard182_rv_slot;
        let mut var_guard183: f64 = *var_guard183_slot;
        let mut var_guard183_rv: f64 = *var_guard183_rv_slot;
        let mut var_guard184: f64 = *var_guard184_slot;
        let mut var_guard184_rv: f64 = *var_guard184_rv_slot;
        let mut var_guard185: f64 = *var_guard185_slot;
        let mut var_guard185_rv: f64 = *var_guard185_rv_slot;
        let mut var_guard186: f64 = *var_guard186_slot;
        let mut var_guard186_rv: f64 = *var_guard186_rv_slot;
        let mut var_guard187: f64 = *var_guard187_slot;
        let mut var_guard187_rv: f64 = *var_guard187_rv_slot;
        let mut var_guard188: f64 = *var_guard188_slot;
        let mut var_guard188_rv: f64 = *var_guard188_rv_slot;
        let mut var_guard189: f64 = *var_guard189_slot;
        let mut var_guard189_rv: f64 = *var_guard189_rv_slot;
        let mut var_guard190: f64 = *var_guard190_slot;
        let mut var_guard190_rv: f64 = *var_guard190_rv_slot;
        let mut var_pbot2: f64 = *var_pbot2_slot;
        let mut var_pbot2_rv: f64 = *var_pbot2_rv_slot;
        let mut var_pgat2: f64 = *var_pgat2_slot;
        let mut var_pgat2_rv: f64 = *var_pgat2_rv_slot;
        let mut var_pmax: f64 = *var_pmax_slot;
        let mut var_pmax_rv: f64 = *var_pmax_rv_slot;
        let mut var_psti2: f64 = *var_psti2_slot;
        let mut var_psti2_rv: f64 = *var_psti2_rv_slot;
        let mut var_vbbtlim_s: f64 = *var_vbbtlim_s_slot;
        let mut var_vbbtlim_s_rv: f64 = *var_vbbtlim_s_rv_slot;
        let mut var_vbibot2: f64 = *var_vbibot2_slot;
        let mut var_vbibot2_rv: f64 = *var_vbibot2_rv_slot;
        let mut var_vbibot2r: f64 = *var_vbibot2r_slot;
        let mut var_vbibot2r_rv: f64 = *var_vbibot2r_rv_slot;
        let mut var_vbigat2: f64 = *var_vbigat2_slot;
        let mut var_vbigat2_rv: f64 = *var_vbigat2_rv_slot;
        let mut var_vbigat2r: f64 = *var_vbigat2r_slot;
        let mut var_vbigat2r_rv: f64 = *var_vbigat2r_rv_slot;
        let mut var_vbimin_s: f64 = *var_vbimin_s_slot;
        let mut var_vbimin_s_rv: f64 = *var_vbimin_s_rv_slot;
        let mut var_vbisti2: f64 = *var_vbisti2_slot;
        let mut var_vbisti2_rv: f64 = *var_vbisti2_rv_slot;
        let mut var_vbisti2r: f64 = *var_vbisti2r_slot;
        let mut var_vbisti2r_rv: f64 = *var_vbisti2r_rv_slot;
        let mut var_vch_s: f64 = *var_vch_s_slot;
        let mut var_vch_s_rv: f64 = *var_vch_s_rv_slot;
        let mut var_vfmin_s: f64 = *var_vfmin_s_slot;
        let mut var_vfmin_s_rv: f64 = *var_vfmin_s_rv_slot;
        let mut var_vmax_d: f64 = *var_vmax_d_slot;
        let mut var_vmax_d_rv: f64 = *var_vmax_d_rv_slot;
        let mut var_vmaxbot: f64 = *var_vmaxbot_slot;
        let mut var_vmaxbot_rv: f64 = *var_vmaxbot_rv_slot;
        let mut var_vmaxgat: f64 = *var_vmaxgat_slot;
        let mut var_vmaxgat_rv: f64 = *var_vmaxgat_rv_slot;
        let mut var_vmaxsti: f64 = *var_vmaxsti_slot;
        let mut var_vmaxsti_rv: f64 = *var_vmaxsti_rv_slot;

        let (assign14130_e11883,) = {
    if (var_guard176 != 0.0) {
        (p.p824,)
    } else {
        (var_pbot2,)
    }
};
        var_pbot2 = assign14130_e11883;
        var_pbot2_rv = 0.0;

        let (assign14140_e11887,) = {
    if (var_guard176 != 0.0) {
        (p.p825,)
    } else {
        (var_psti2,)
    }
};
        var_psti2 = assign14140_e11887;
        var_psti2_rv = 0.0;

        let (assign14150_e11891,) = {
    if (var_guard176 != 0.0) {
        (p.p826,)
    } else {
        (var_pgat2,)
    }
};
        var_pgat2 = assign14150_e11891;
        var_pgat2_rv = 0.0;

        let (assign14160_e11895,) = {
    if (var_guard176 != 0.0) {
        (p.p821,)
    } else {
        (var_vbibot2r,)
    }
};
        var_vbibot2r = assign14160_e11895;
        var_vbibot2r_rv = 0.0;

        let (assign14170_e11899,) = {
    if (var_guard176 != 0.0) {
        (p.p822,)
    } else {
        (var_vbisti2r,)
    }
};
        var_vbisti2r = assign14170_e11899;
        var_vbisti2r_rv = 0.0;

        let (assign14180_e11903,) = {
    if (var_guard176 != 0.0) {
        (p.p823,)
    } else {
        (var_vbigat2r,)
    }
};
        var_vbigat2r = assign14180_e11903;
        var_vbigat2r_rv = 0.0;

        let assign14190_e11906: f64 = if var_absource_i == 0.0 { 1.0 } else { 0.0 };
        var_guard182 = assign14190_e11906;
        var_guard182_rv = 0.0;

        let (assign14200_e11914,) = {
    if ((var_guard176 != 0.0) && (var_guard182 != 0.0)) {
        let assign14200_e11912: f64 = (var_vbisti + var_vbigat);
        (assign14200_e11912,)
    } else {
        (var_vbibot2,)
    }
};
        var_vbibot2 = assign14200_e11914;
        var_vbibot2_rv = 0.0;

        let (assign14210_e11924,) = {
    if ((var_guard176 != 0.0) && (var_guard182 != 0.0)) {
        let assign14210_e11921: f64 = (p.p825).min(p.p826);
        let assign14210_e11922: f64 = (0.9 * assign14210_e11921);
        (assign14210_e11922,)
    } else {
        (var_pbot2,)
    }
};
        var_pbot2 = assign14210_e11924;
        var_pbot2_rv = 0.0;

        let (assign14220_e11932,) = {
    if ((var_guard176 != 0.0) && (var_guard182 != 0.0)) {
        let assign14220_e11930: f64 = (p.p822 + p.p823);
        (assign14220_e11930,)
    } else {
        (var_vbibot2r,)
    }
};
        var_vbibot2r = assign14220_e11932;
        var_vbibot2r_rv = 0.0;

        let assign14230_e11935: f64 = if var_lssource_i == 0.0 { 1.0 } else { 0.0 };
        var_guard183 = assign14230_e11935;
        var_guard183_rv = 0.0;

        let (assign14240_e11943,) = {
    if ((var_guard176 != 0.0) && (var_guard183 != 0.0)) {
        let assign14240_e11941: f64 = (var_vbibot + var_vbigat);
        (assign14240_e11941,)
    } else {
        (var_vbisti2,)
    }
};
        var_vbisti2 = assign14240_e11943;
        var_vbisti2_rv = 0.0;

        let (assign14250_e11953,) = {
    if ((var_guard176 != 0.0) && (var_guard183 != 0.0)) {
        let assign14250_e11950: f64 = (p.p824).min(p.p826);
        let assign14250_e11951: f64 = (0.9 * assign14250_e11950);
        (assign14250_e11951,)
    } else {
        (var_psti2,)
    }
};
        var_psti2 = assign14250_e11953;
        var_psti2_rv = 0.0;

        let (assign14260_e11961,) = {
    if ((var_guard176 != 0.0) && (var_guard183 != 0.0)) {
        let assign14260_e11959: f64 = (p.p821 + p.p823);
        (assign14260_e11959,)
    } else {
        (var_vbisti2r,)
    }
};
        var_vbisti2r = assign14260_e11961;
        var_vbisti2r_rv = 0.0;

        let assign14270_e11964: f64 = if var_lgsource_i == 0.0 { 1.0 } else { 0.0 };
        var_guard184 = assign14270_e11964;
        var_guard184_rv = 0.0;

        let (assign14280_e11972,) = {
    if ((var_guard176 != 0.0) && (var_guard184 != 0.0)) {
        let assign14280_e11970: f64 = (var_vbibot + var_vbisti);
        (assign14280_e11970,)
    } else {
        (var_vbigat2,)
    }
};
        var_vbigat2 = assign14280_e11972;
        var_vbigat2_rv = 0.0;

        let (assign14290_e11982,) = {
    if ((var_guard176 != 0.0) && (var_guard184 != 0.0)) {
        let assign14290_e11979: f64 = (p.p824).min(p.p825);
        let assign14290_e11980: f64 = (0.9 * assign14290_e11979);
        (assign14290_e11980,)
    } else {
        (var_pgat2,)
    }
};
        var_pgat2 = assign14290_e11982;
        var_pgat2_rv = 0.0;

        let (assign14300_e11990,) = {
    if ((var_guard176 != 0.0) && (var_guard184 != 0.0)) {
        let assign14300_e11988: f64 = (p.p821 + p.p822);
        (assign14300_e11988,)
    } else {
        (var_vbigat2r,)
    }
};
        var_vbigat2r = assign14300_e11990;
        var_vbigat2r_rv = 0.0;

        let (assign14310_e11998,) = {
    if (var_guard176 != 0.0) {
        let assign14310_e11994: f64 = (var_vbibot2).min(var_vbisti2);
        let assign14310_e11996: f64 = (assign14310_e11994).min(var_vbigat2);
        (assign14310_e11996,)
    } else {
        (var_vbimin_s,)
    }
};
        var_vbimin_s = assign14310_e11998;
        var_vbimin_s_rv = 0.0;

        let (assign14320_e12004,) = {
    if (var_guard176 != 0.0) {
        let assign14320_e12002: f64 = (var_vbimin_s * 0.1);
        (assign14320_e12002,)
    } else {
        (var_vch_s,)
    }
};
        var_vch_s = assign14320_e12004;
        var_vch_s_rv = 0.0;

        let (assign14330_e12012,) = {
    if (var_guard176 != 0.0) {
        let assign14330_e12008: f64 = (var_pbot2).max(var_psti2);
        let assign14330_e12010: f64 = (assign14330_e12008).max(var_pgat2);
        (assign14330_e12010,)
    } else {
        (var_pmax,)
    }
};
        var_pmax = assign14330_e12012;
        var_pmax_rv = 0.0;

        let (assign14340_e12025,) = {
    if (var_guard176 != 0.0) {
        let assign14340_e12018: f64 = (-1.0);
        let assign14340_e12020: f64 = (assign14340_e12018 / var_pmax);
        let assign14340_e12021: f64 = (2.0_f64).powf(assign14340_e12020);
        let assign14340_e12022: f64 = (1.0 - assign14340_e12021);
        let assign14340_e12023: f64 = (var_vbimin_s * assign14340_e12022);
        (assign14340_e12023,)
    } else {
        (var_vfmin_s,)
    }
};
        var_vfmin_s = assign14340_e12025;
        var_vfmin_s_rv = 0.0;

        let (assign14350_e12035,) = {
    if (var_guard176 != 0.0) {
        let assign14350_e12029: f64 = (var_vbibot2r).min(var_vbisti2r);
        let assign14350_e12031: f64 = (assign14350_e12029).min(var_vbigat2r);
        let assign14350_e12033: f64 = (assign14350_e12031 - 0.05);
        (assign14350_e12033,)
    } else {
        (var_vbbtlim_s,)
    }
};
        var_vbbtlim_s = assign14350_e12035;
        var_vbbtlim_s_rv = 0.0;

        let assign14360_e12038: f64 = (var_idsatbot_d * var_abdrain_i);
        let assign14360_e12040: f64 = if assign14360_e12038 > 0.0 { 1.0 } else { 0.0 };
        var_guard185 = assign14360_e12040;
        var_guard185_rv = 0.0;

        let (assign14370_e12055,) = {
    if ((var_guard176 != 0.0) && (var_guard185 != 0.0)) {
        let assign14370_e12048: f64 = (var_idsatbot_d * var_abdrain_i);
        let assign14370_e12049: f64 = (p.p815 / assign14370_e12048);
        let assign14370_e12051: f64 = (assign14370_e12049 + 1.0);
        let assign14370_e12052: f64 = (assign14370_e12051).ln();
        let assign14370_e12053: f64 = (var_phitd * assign14370_e12052);
        (assign14370_e12053,)
    } else {
        (var_vmaxbot,)
    }
};
        var_vmaxbot = assign14370_e12055;
        var_vmaxbot_rv = 0.0;

        let (assign14380_e12062,) = {
    if ((var_guard176 != 0.0) && (var_guard185 == 0.0)) {
        (100000000.0,)
    } else {
        (var_vmaxbot,)
    }
};
        var_vmaxbot = assign14380_e12062;
        var_vmaxbot_rv = 0.0;

        let assign14390_e12065: f64 = (var_idsatsti_d * var_lsdrain_i);
        let assign14390_e12067: f64 = if assign14390_e12065 > 0.0 { 1.0 } else { 0.0 };
        var_guard186 = assign14390_e12067;
        var_guard186_rv = 0.0;

        let (assign14400_e12082,) = {
    if ((var_guard176 != 0.0) && (var_guard186 != 0.0)) {
        let assign14400_e12075: f64 = (var_idsatsti_d * var_lsdrain_i);
        let assign14400_e12076: f64 = (p.p815 / assign14400_e12075);
        let assign14400_e12078: f64 = (assign14400_e12076 + 1.0);
        let assign14400_e12079: f64 = (assign14400_e12078).ln();
        let assign14400_e12080: f64 = (var_phitd * assign14400_e12079);
        (assign14400_e12080,)
    } else {
        (var_vmaxsti,)
    }
};
        var_vmaxsti = assign14400_e12082;
        var_vmaxsti_rv = 0.0;

        let (assign14410_e12089,) = {
    if ((var_guard176 != 0.0) && (var_guard186 == 0.0)) {
        (100000000.0,)
    } else {
        (var_vmaxsti,)
    }
};
        var_vmaxsti = assign14410_e12089;
        var_vmaxsti_rv = 0.0;

        let assign14420_e12092: f64 = (var_idsatgat_d * var_lgdrain_i);
        let assign14420_e12094: f64 = if assign14420_e12092 > 0.0 { 1.0 } else { 0.0 };
        var_guard187 = assign14420_e12094;
        var_guard187_rv = 0.0;

        let (assign14430_e12109,) = {
    if ((var_guard176 != 0.0) && (var_guard187 != 0.0)) {
        let assign14430_e12102: f64 = (var_idsatgat_d * var_lgdrain_i);
        let assign14430_e12103: f64 = (p.p815 / assign14430_e12102);
        let assign14430_e12105: f64 = (assign14430_e12103 + 1.0);
        let assign14430_e12106: f64 = (assign14430_e12105).ln();
        let assign14430_e12107: f64 = (var_phitd * assign14430_e12106);
        (assign14430_e12107,)
    } else {
        (var_vmaxgat,)
    }
};
        var_vmaxgat = assign14430_e12109;
        var_vmaxgat_rv = 0.0;

        let (assign14440_e12116,) = {
    if ((var_guard176 != 0.0) && (var_guard187 == 0.0)) {
        (100000000.0,)
    } else {
        (var_vmaxgat,)
    }
};
        var_vmaxgat = assign14440_e12116;
        var_vmaxgat_rv = 0.0;

        let (assign14450_e12124,) = {
    if (var_guard176 != 0.0) {
        let assign14450_e12120: f64 = (var_vmaxbot).min(var_vmaxsti);
        let assign14450_e12122: f64 = (assign14450_e12120).min(var_vmaxgat);
        (assign14450_e12122,)
    } else {
        (var_vmax_d,)
    }
};
        var_vmax_d = assign14450_e12124;
        var_vmax_d_rv = 0.0;

        let assign14460_e12127: f64 = (var_vmax_d * var_phitdinv);
        let assign14460_e12128: f64 = (assign14460_e12127).abs();
        let assign14460_e12130: f64 = if assign14460_e12128 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard188 = assign14460_e12130;
        var_guard188_rv = 0.0;

        let (assign14470_e12139,) = {
    if ((var_guard176 != 0.0) && (var_guard188 != 0.0)) {
        let assign14470_e12136: f64 = (var_vmax_d * var_phitdinv);
        let assign14470_e12137: f64 = (assign14470_e12136).exp();
        (assign14470_e12137,)
    } else {
        (var_exp_vmax_over_phitd_d,)
    }
};
        var_exp_vmax_over_phitd_d = assign14470_e12139;
        var_exp_vmax_over_phitd_d_rv = 0.0;

        let assign14480_e12142: f64 = (var_vmax_d * var_phitdinv);
        let assign14480_e12144: f64 = if assign14480_e12142 < 0.0 { 1.0 } else { 0.0 };
        var_guard189 = assign14480_e12144;
        var_guard189_rv = 0.0;

        let (assign14490_e12184,) = {
    if (((var_guard176 != 0.0) && (var_guard188 == 0.0)) && (var_guard189 != 0.0)) {
        let assign14490_e12154: f64 = (-230.25850929940458);
        let assign14490_e12157: f64 = (var_vmax_d * var_phitdinv);
        let assign14490_e12158: f64 = (assign14490_e12154 - assign14490_e12157);
        let assign14490_e12162: f64 = (-230.25850929940458);
        let assign14490_e12165: f64 = (var_vmax_d * var_phitdinv);
        let assign14490_e12166: f64 = (assign14490_e12162 - assign14490_e12165);
        let assign14490_e12169: f64 = (-230.25850929940458);
        let assign14490_e12172: f64 = (var_vmax_d * var_phitdinv);
        let assign14490_e12173: f64 = (assign14490_e12169 - assign14490_e12172);
        let assign14490_e12175: f64 = (assign14490_e12173 * 0.3333333333333333);
        let assign14490_e12176: f64 = (1.0 + assign14490_e12175);
        let assign14490_e12177: f64 = (assign14490_e12166 * assign14490_e12176);
        let assign14490_e12178: f64 = (0.5 * assign14490_e12177);
        let assign14490_e12179: f64 = (1.0 + assign14490_e12178);
        let assign14490_e12180: f64 = (assign14490_e12158 * assign14490_e12179);
        let assign14490_e12181: f64 = (1.0 + assign14490_e12180);
        let assign14490_e12182: f64 = (1e-100 / assign14490_e12181);
        (assign14490_e12182,)
    } else {
        (var_exp_vmax_over_phitd_d,)
    }
};
        var_exp_vmax_over_phitd_d = assign14490_e12184;
        var_exp_vmax_over_phitd_d_rv = 0.0;

        let (assign14500_e12222,) = {
    if (((var_guard176 != 0.0) && (var_guard188 == 0.0)) && (var_guard189 == 0.0)) {
        let assign14500_e12196: f64 = (var_vmax_d * var_phitdinv);
        let assign14500_e12198: f64 = (assign14500_e12196 - 230.25850929940458);
        let assign14500_e12203: f64 = (var_vmax_d * var_phitdinv);
        let assign14500_e12205: f64 = (assign14500_e12203 - 230.25850929940458);
        let assign14500_e12209: f64 = (var_vmax_d * var_phitdinv);
        let assign14500_e12211: f64 = (assign14500_e12209 - 230.25850929940458);
        let assign14500_e12213: f64 = (assign14500_e12211 * 0.3333333333333333);
        let assign14500_e12214: f64 = (1.0 + assign14500_e12213);
        let assign14500_e12215: f64 = (assign14500_e12205 * assign14500_e12214);
        let assign14500_e12216: f64 = (0.5 * assign14500_e12215);
        let assign14500_e12217: f64 = (1.0 + assign14500_e12216);
        let assign14500_e12218: f64 = (assign14500_e12198 * assign14500_e12217);
        let assign14500_e12219: f64 = (1.0 + assign14500_e12218);
        let assign14500_e12220: f64 = (1e100 * assign14500_e12219);
        (assign14500_e12220,)
    } else {
        (var_exp_vmax_over_phitd_d,)
    }
};
        var_exp_vmax_over_phitd_d = assign14500_e12222;
        var_exp_vmax_over_phitd_d_rv = 0.0;

        let (assign14510_e12226,) = {
    if (var_guard176 != 0.0) {
        (var_vbibot_d,)
    } else {
        (var_vbibot2,)
    }
};
        var_vbibot2 = assign14510_e12226;
        var_vbibot2_rv = 0.0;

        let (assign14520_e12230,) = {
    if (var_guard176 != 0.0) {
        (var_vbisti_d,)
    } else {
        (var_vbisti2,)
    }
};
        var_vbisti2 = assign14520_e12230;
        var_vbisti2_rv = 0.0;

        let (assign14530_e12234,) = {
    if (var_guard176 != 0.0) {
        (var_vbigat_d,)
    } else {
        (var_vbigat2,)
    }
};
        var_vbigat2 = assign14530_e12234;
        var_vbigat2_rv = 0.0;

        let (assign14540_e12238,) = {
    if (var_guard176 != 0.0) {
        (var_pbotd_i,)
    } else {
        (var_pbot2,)
    }
};
        var_pbot2 = assign14540_e12238;
        var_pbot2_rv = 0.0;

        let (assign14550_e12242,) = {
    if (var_guard176 != 0.0) {
        (var_pstid_i,)
    } else {
        (var_psti2,)
    }
};
        var_psti2 = assign14550_e12242;
        var_psti2_rv = 0.0;

        let (assign14560_e12246,) = {
    if (var_guard176 != 0.0) {
        (var_pgatd_i,)
    } else {
        (var_pgat2,)
    }
};
        var_pgat2 = assign14560_e12246;
        var_pgat2_rv = 0.0;

        let (assign14570_e12250,) = {
    if (var_guard176 != 0.0) {
        (var_vbirbotd_i,)
    } else {
        (var_vbibot2r,)
    }
};
        var_vbibot2r = assign14570_e12250;
        var_vbibot2r_rv = 0.0;

        let (assign14580_e12254,) = {
    if (var_guard176 != 0.0) {
        (var_vbirstid_i,)
    } else {
        (var_vbisti2r,)
    }
};
        var_vbisti2r = assign14580_e12254;
        var_vbisti2r_rv = 0.0;

        let (assign14590_e12258,) = {
    if (var_guard176 != 0.0) {
        (var_vbirgatd_i,)
    } else {
        (var_vbigat2r,)
    }
};
        var_vbigat2r = assign14590_e12258;
        var_vbigat2r_rv = 0.0;

        let assign14600_e12261: f64 = if var_abdrain_i == 0.0 { 1.0 } else { 0.0 };
        var_guard190 = assign14600_e12261;
        var_guard190_rv = 0.0;

        *var_exp_vmax_over_phitd_d_slot = var_exp_vmax_over_phitd_d;
        *var_exp_vmax_over_phitd_d_rv_slot = var_exp_vmax_over_phitd_d_rv;
        *var_guard182_slot = var_guard182;
        *var_guard182_rv_slot = var_guard182_rv;
        *var_guard183_slot = var_guard183;
        *var_guard183_rv_slot = var_guard183_rv;
        *var_guard184_slot = var_guard184;
        *var_guard184_rv_slot = var_guard184_rv;
        *var_guard185_slot = var_guard185;
        *var_guard185_rv_slot = var_guard185_rv;
        *var_guard186_slot = var_guard186;
        *var_guard186_rv_slot = var_guard186_rv;
        *var_guard187_slot = var_guard187;
        *var_guard187_rv_slot = var_guard187_rv;
        *var_guard188_slot = var_guard188;
        *var_guard188_rv_slot = var_guard188_rv;
        *var_guard189_slot = var_guard189;
        *var_guard189_rv_slot = var_guard189_rv;
        *var_guard190_slot = var_guard190;
        *var_guard190_rv_slot = var_guard190_rv;
        *var_pbot2_slot = var_pbot2;
        *var_pbot2_rv_slot = var_pbot2_rv;
        *var_pgat2_slot = var_pgat2;
        *var_pgat2_rv_slot = var_pgat2_rv;
        *var_pmax_slot = var_pmax;
        *var_pmax_rv_slot = var_pmax_rv;
        *var_psti2_slot = var_psti2;
        *var_psti2_rv_slot = var_psti2_rv;
        *var_vbbtlim_s_slot = var_vbbtlim_s;
        *var_vbbtlim_s_rv_slot = var_vbbtlim_s_rv;
        *var_vbibot2_slot = var_vbibot2;
        *var_vbibot2_rv_slot = var_vbibot2_rv;
        *var_vbibot2r_slot = var_vbibot2r;
        *var_vbibot2r_rv_slot = var_vbibot2r_rv;
        *var_vbigat2_slot = var_vbigat2;
        *var_vbigat2_rv_slot = var_vbigat2_rv;
        *var_vbigat2r_slot = var_vbigat2r;
        *var_vbigat2r_rv_slot = var_vbigat2r_rv;
        *var_vbimin_s_slot = var_vbimin_s;
        *var_vbimin_s_rv_slot = var_vbimin_s_rv;
        *var_vbisti2_slot = var_vbisti2;
        *var_vbisti2_rv_slot = var_vbisti2_rv;
        *var_vbisti2r_slot = var_vbisti2r;
        *var_vbisti2r_rv_slot = var_vbisti2r_rv;
        *var_vch_s_slot = var_vch_s;
        *var_vch_s_rv_slot = var_vch_s_rv;
        *var_vfmin_s_slot = var_vfmin_s;
        *var_vfmin_s_rv_slot = var_vfmin_s_rv;
        *var_vmax_d_slot = var_vmax_d;
        *var_vmax_d_rv_slot = var_vmax_d_rv;
        *var_vmaxbot_slot = var_vmaxbot;
        *var_vmaxbot_rv_slot = var_vmaxbot_rv;
        *var_vmaxgat_slot = var_vmaxgat;
        *var_vmaxgat_rv_slot = var_vmaxgat_rv;
        *var_vmaxsti_slot = var_vmaxsti;
        *var_vmaxsti_rv_slot = var_vmaxsti_rv;
    }

    pub(super) fn stamp_reactive_block_22(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        var_abdrain_i: f64,
        var_absource_i: f64,
        var_chnl_type: f64,
        var_cjobot: f64,
        var_cjobot_d: f64,
        var_cjogat: f64,
        var_cjogat_d: f64,
        var_cjosti: f64,
        var_cjosti_d: f64,
        var_fjunqd_i: f64,
        var_guard176: f64,
        var_guard190: f64,
        var_inv_phita: f64,
        var_lgdrain_i: f64,
        var_lgsource_i: f64,
        var_lsdrain_i: f64,
        var_lssource_i: f64,
        var_pbotd_i: f64,
        var_pgatd_i: f64,
        var_pstid_i: f64,
        var_swjunexp_i: f64,
        var_vbibot_d: f64,
        var_vbigat_d: f64,
        var_vbirbotd_i: f64,
        var_vbirgatd_i: f64,
        var_vbirstid_i: f64,
        var_vbisti_d: f64,
        var_guard1011_slot: &mut f64,
        var_guard1011_rv_slot: &mut f64,
        var_guard191_slot: &mut f64,
        var_guard191_rv_slot: &mut f64,
        var_guard192_slot: &mut f64,
        var_guard192_rv_slot: &mut f64,
        var_guard193_slot: &mut f64,
        var_guard193_rv_slot: &mut f64,
        var_guard528_slot: &mut f64,
        var_guard528_rv_slot: &mut f64,
        var_guard529_slot: &mut f64,
        var_guard529_rv_slot: &mut f64,
        var_guard530_slot: &mut f64,
        var_guard530_rv_slot: &mut f64,
        var_guard818_slot: &mut f64,
        var_guard818_rv_slot: &mut f64,
        var_guard819_slot: &mut f64,
        var_guard819_rv_slot: &mut f64,
        var_guard820_slot: &mut f64,
        var_guard820_rv_slot: &mut f64,
        var_pbot2_slot: &mut f64,
        var_pbot2_rv_slot: &mut f64,
        var_pgat2_slot: &mut f64,
        var_pgat2_rv_slot: &mut f64,
        var_pmax_slot: &mut f64,
        var_pmax_rv_slot: &mut f64,
        var_psti2_slot: &mut f64,
        var_psti2_rv_slot: &mut f64,
        var_temp1_slot: &mut f64,
        var_temp1_dn5_slot: &mut f64,
        var_temp1_dn6_slot: &mut f64,
        var_temp1_dn7_slot: &mut f64,
        var_temp1_dn8_slot: &mut f64,
        var_temp1_rv_slot: &mut f64,
        var_temp2_slot: &mut f64,
        var_temp2_dn5_slot: &mut f64,
        var_temp2_dn6_slot: &mut f64,
        var_temp2_dn7_slot: &mut f64,
        var_temp2_dn8_slot: &mut f64,
        var_temp2_rv_slot: &mut f64,
        var_temp__blk936_slot: &mut f64,
        var_temp__blk936_dn5_slot: &mut f64,
        var_temp__blk936_dn6_slot: &mut f64,
        var_temp__blk936_dn7_slot: &mut f64,
        var_temp__blk936_dn8_slot: &mut f64,
        var_temp__blk936_rv_slot: &mut f64,
        var_v_ds_slot: &mut f64,
        var_v_ds_dn6_slot: &mut f64,
        var_v_ds_dn7_slot: &mut f64,
        var_v_ds_rv_slot: &mut f64,
        var_v_gs_slot: &mut f64,
        var_v_gs_dn5_slot: &mut f64,
        var_v_gs_dn6_slot: &mut f64,
        var_v_gs_dn7_slot: &mut f64,
        var_v_gs_rv_slot: &mut f64,
        var_v_sb_slot: &mut f64,
        var_v_sb_dn6_slot: &mut f64,
        var_v_sb_dn7_slot: &mut f64,
        var_v_sb_dn8_slot: &mut f64,
        var_v_sb_rv_slot: &mut f64,
        var_vbbtlim_d_slot: &mut f64,
        var_vbbtlim_d_rv_slot: &mut f64,
        var_vbibot2_slot: &mut f64,
        var_vbibot2_rv_slot: &mut f64,
        var_vbibot2r_slot: &mut f64,
        var_vbibot2r_rv_slot: &mut f64,
        var_vbigat2_slot: &mut f64,
        var_vbigat2_rv_slot: &mut f64,
        var_vbigat2r_slot: &mut f64,
        var_vbigat2r_rv_slot: &mut f64,
        var_vbimin_d_slot: &mut f64,
        var_vbimin_d_rv_slot: &mut f64,
        var_vbisti2_slot: &mut f64,
        var_vbisti2_rv_slot: &mut f64,
        var_vbisti2r_slot: &mut f64,
        var_vbisti2r_rv_slot: &mut f64,
        var_vch_d_slot: &mut f64,
        var_vch_d_rv_slot: &mut f64,
        var_vdbprime_slot: &mut f64,
        var_vdbprime_dn6_slot: &mut f64,
        var_vdbprime_dn7_slot: &mut f64,
        var_vdbprime_dn8_slot: &mut f64,
        var_vdbprime_rv_slot: &mut f64,
        var_vfmin_d_slot: &mut f64,
        var_vfmin_d_rv_slot: &mut f64,
        var_vgb_slot: &mut f64,
        var_vgb_dn5_slot: &mut f64,
        var_vgb_dn6_slot: &mut f64,
        var_vgb_dn7_slot: &mut f64,
        var_vgb_dn8_slot: &mut f64,
        var_vgb_rv_slot: &mut f64,
        var_vgdprime_slot: &mut f64,
        var_vgdprime_dn5_slot: &mut f64,
        var_vgdprime_dn6_slot: &mut f64,
        var_vgdprime_dn7_slot: &mut f64,
        var_vgdprime_rv_slot: &mut f64,
        var_vgsprime_slot: &mut f64,
        var_vgsprime_dn5_slot: &mut f64,
        var_vgsprime_dn6_slot: &mut f64,
        var_vgsprime_dn7_slot: &mut f64,
        var_vgsprime_rv_slot: &mut f64,
        var_vjun_d_slot: &mut f64,
        var_vjun_d_dn11_slot: &mut f64,
        var_vjun_d_dn7_slot: &mut f64,
        var_vjun_d_rv_slot: &mut f64,
        var_vjun_s_slot: &mut f64,
        var_vjun_s_dn10_slot: &mut f64,
        var_vjun_s_dn6_slot: &mut f64,
        var_vjun_s_rv_slot: &mut f64,
        var_vsbprime_slot: &mut f64,
        var_vsbprime_dn6_slot: &mut f64,
        var_vsbprime_dn7_slot: &mut f64,
        var_vsbprime_dn8_slot: &mut f64,
        var_vsbprime_rv_slot: &mut f64,
        var_xgd_ov_slot: &mut f64,
        var_xgd_ov_dn5_slot: &mut f64,
        var_xgd_ov_dn6_slot: &mut f64,
        var_xgd_ov_dn7_slot: &mut f64,
        var_xgd_ov_rv_slot: &mut f64,
        var_xgs_ov_slot: &mut f64,
        var_xgs_ov_dn5_slot: &mut f64,
        var_xgs_ov_dn6_slot: &mut f64,
        var_xgs_ov_dn7_slot: &mut f64,
        var_xgs_ov_rv_slot: &mut f64,
        var_zflagbot_d_slot: &mut f64,
        var_zflagbot_d_rv_slot: &mut f64,
        var_zflagbot_s_slot: &mut f64,
        var_zflagbot_s_rv_slot: &mut f64,
        var_zflaggat_d_slot: &mut f64,
        var_zflaggat_d_rv_slot: &mut f64,
        var_zflaggat_s_slot: &mut f64,
        var_zflaggat_s_rv_slot: &mut f64,
        var_zflagsti_d_slot: &mut f64,
        var_zflagsti_d_rv_slot: &mut f64,
        var_zflagsti_s_slot: &mut f64,
        var_zflagsti_s_rv_slot: &mut f64,
        var_zfrac_slot: &mut f64,
        var_zfrac_rv_slot: &mut f64,
    ) {
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let nv10 = ctx.node_voltage(nodes[10]);
        let nv11 = ctx.node_voltage(nodes[11]);
        let mut var_guard1011: f64 = *var_guard1011_slot;
        let mut var_guard1011_rv: f64 = *var_guard1011_rv_slot;
        let mut var_guard191: f64 = *var_guard191_slot;
        let mut var_guard191_rv: f64 = *var_guard191_rv_slot;
        let mut var_guard192: f64 = *var_guard192_slot;
        let mut var_guard192_rv: f64 = *var_guard192_rv_slot;
        let mut var_guard193: f64 = *var_guard193_slot;
        let mut var_guard193_rv: f64 = *var_guard193_rv_slot;
        let mut var_guard528: f64 = *var_guard528_slot;
        let mut var_guard528_rv: f64 = *var_guard528_rv_slot;
        let mut var_guard529: f64 = *var_guard529_slot;
        let mut var_guard529_rv: f64 = *var_guard529_rv_slot;
        let mut var_guard530: f64 = *var_guard530_slot;
        let mut var_guard530_rv: f64 = *var_guard530_rv_slot;
        let mut var_guard818: f64 = *var_guard818_slot;
        let mut var_guard818_rv: f64 = *var_guard818_rv_slot;
        let mut var_guard819: f64 = *var_guard819_slot;
        let mut var_guard819_rv: f64 = *var_guard819_rv_slot;
        let mut var_guard820: f64 = *var_guard820_slot;
        let mut var_guard820_rv: f64 = *var_guard820_rv_slot;
        let mut var_pbot2: f64 = *var_pbot2_slot;
        let mut var_pbot2_rv: f64 = *var_pbot2_rv_slot;
        let mut var_pgat2: f64 = *var_pgat2_slot;
        let mut var_pgat2_rv: f64 = *var_pgat2_rv_slot;
        let mut var_pmax: f64 = *var_pmax_slot;
        let mut var_pmax_rv: f64 = *var_pmax_rv_slot;
        let mut var_psti2: f64 = *var_psti2_slot;
        let mut var_psti2_rv: f64 = *var_psti2_rv_slot;
        let mut var_temp1: f64 = *var_temp1_slot;
        let mut var_temp1_dn5: f64 = *var_temp1_dn5_slot;
        let mut var_temp1_dn6: f64 = *var_temp1_dn6_slot;
        let mut var_temp1_dn7: f64 = *var_temp1_dn7_slot;
        let mut var_temp1_dn8: f64 = *var_temp1_dn8_slot;
        let mut var_temp1_rv: f64 = *var_temp1_rv_slot;
        let mut var_temp2: f64 = *var_temp2_slot;
        let mut var_temp2_dn5: f64 = *var_temp2_dn5_slot;
        let mut var_temp2_dn6: f64 = *var_temp2_dn6_slot;
        let mut var_temp2_dn7: f64 = *var_temp2_dn7_slot;
        let mut var_temp2_dn8: f64 = *var_temp2_dn8_slot;
        let mut var_temp2_rv: f64 = *var_temp2_rv_slot;
        let mut var_temp__blk936: f64 = *var_temp__blk936_slot;
        let mut var_temp__blk936_dn5: f64 = *var_temp__blk936_dn5_slot;
        let mut var_temp__blk936_dn6: f64 = *var_temp__blk936_dn6_slot;
        let mut var_temp__blk936_dn7: f64 = *var_temp__blk936_dn7_slot;
        let mut var_temp__blk936_dn8: f64 = *var_temp__blk936_dn8_slot;
        let mut var_temp__blk936_rv: f64 = *var_temp__blk936_rv_slot;
        let mut var_v_ds: f64 = *var_v_ds_slot;
        let mut var_v_ds_dn6: f64 = *var_v_ds_dn6_slot;
        let mut var_v_ds_dn7: f64 = *var_v_ds_dn7_slot;
        let mut var_v_ds_rv: f64 = *var_v_ds_rv_slot;
        let mut var_v_gs: f64 = *var_v_gs_slot;
        let mut var_v_gs_dn5: f64 = *var_v_gs_dn5_slot;
        let mut var_v_gs_dn6: f64 = *var_v_gs_dn6_slot;
        let mut var_v_gs_dn7: f64 = *var_v_gs_dn7_slot;
        let mut var_v_gs_rv: f64 = *var_v_gs_rv_slot;
        let mut var_v_sb: f64 = *var_v_sb_slot;
        let mut var_v_sb_dn6: f64 = *var_v_sb_dn6_slot;
        let mut var_v_sb_dn7: f64 = *var_v_sb_dn7_slot;
        let mut var_v_sb_dn8: f64 = *var_v_sb_dn8_slot;
        let mut var_v_sb_rv: f64 = *var_v_sb_rv_slot;
        let mut var_vbbtlim_d: f64 = *var_vbbtlim_d_slot;
        let mut var_vbbtlim_d_rv: f64 = *var_vbbtlim_d_rv_slot;
        let mut var_vbibot2: f64 = *var_vbibot2_slot;
        let mut var_vbibot2_rv: f64 = *var_vbibot2_rv_slot;
        let mut var_vbibot2r: f64 = *var_vbibot2r_slot;
        let mut var_vbibot2r_rv: f64 = *var_vbibot2r_rv_slot;
        let mut var_vbigat2: f64 = *var_vbigat2_slot;
        let mut var_vbigat2_rv: f64 = *var_vbigat2_rv_slot;
        let mut var_vbigat2r: f64 = *var_vbigat2r_slot;
        let mut var_vbigat2r_rv: f64 = *var_vbigat2r_rv_slot;
        let mut var_vbimin_d: f64 = *var_vbimin_d_slot;
        let mut var_vbimin_d_rv: f64 = *var_vbimin_d_rv_slot;
        let mut var_vbisti2: f64 = *var_vbisti2_slot;
        let mut var_vbisti2_rv: f64 = *var_vbisti2_rv_slot;
        let mut var_vbisti2r: f64 = *var_vbisti2r_slot;
        let mut var_vbisti2r_rv: f64 = *var_vbisti2r_rv_slot;
        let mut var_vch_d: f64 = *var_vch_d_slot;
        let mut var_vch_d_rv: f64 = *var_vch_d_rv_slot;
        let mut var_vdbprime: f64 = *var_vdbprime_slot;
        let mut var_vdbprime_dn6: f64 = *var_vdbprime_dn6_slot;
        let mut var_vdbprime_dn7: f64 = *var_vdbprime_dn7_slot;
        let mut var_vdbprime_dn8: f64 = *var_vdbprime_dn8_slot;
        let mut var_vdbprime_rv: f64 = *var_vdbprime_rv_slot;
        let mut var_vfmin_d: f64 = *var_vfmin_d_slot;
        let mut var_vfmin_d_rv: f64 = *var_vfmin_d_rv_slot;
        let mut var_vgb: f64 = *var_vgb_slot;
        let mut var_vgb_dn5: f64 = *var_vgb_dn5_slot;
        let mut var_vgb_dn6: f64 = *var_vgb_dn6_slot;
        let mut var_vgb_dn7: f64 = *var_vgb_dn7_slot;
        let mut var_vgb_dn8: f64 = *var_vgb_dn8_slot;
        let mut var_vgb_rv: f64 = *var_vgb_rv_slot;
        let mut var_vgdprime: f64 = *var_vgdprime_slot;
        let mut var_vgdprime_dn5: f64 = *var_vgdprime_dn5_slot;
        let mut var_vgdprime_dn6: f64 = *var_vgdprime_dn6_slot;
        let mut var_vgdprime_dn7: f64 = *var_vgdprime_dn7_slot;
        let mut var_vgdprime_rv: f64 = *var_vgdprime_rv_slot;
        let mut var_vgsprime: f64 = *var_vgsprime_slot;
        let mut var_vgsprime_dn5: f64 = *var_vgsprime_dn5_slot;
        let mut var_vgsprime_dn6: f64 = *var_vgsprime_dn6_slot;
        let mut var_vgsprime_dn7: f64 = *var_vgsprime_dn7_slot;
        let mut var_vgsprime_rv: f64 = *var_vgsprime_rv_slot;
        let mut var_vjun_d: f64 = *var_vjun_d_slot;
        let mut var_vjun_d_dn11: f64 = *var_vjun_d_dn11_slot;
        let mut var_vjun_d_dn7: f64 = *var_vjun_d_dn7_slot;
        let mut var_vjun_d_rv: f64 = *var_vjun_d_rv_slot;
        let mut var_vjun_s: f64 = *var_vjun_s_slot;
        let mut var_vjun_s_dn10: f64 = *var_vjun_s_dn10_slot;
        let mut var_vjun_s_dn6: f64 = *var_vjun_s_dn6_slot;
        let mut var_vjun_s_rv: f64 = *var_vjun_s_rv_slot;
        let mut var_vsbprime: f64 = *var_vsbprime_slot;
        let mut var_vsbprime_dn6: f64 = *var_vsbprime_dn6_slot;
        let mut var_vsbprime_dn7: f64 = *var_vsbprime_dn7_slot;
        let mut var_vsbprime_dn8: f64 = *var_vsbprime_dn8_slot;
        let mut var_vsbprime_rv: f64 = *var_vsbprime_rv_slot;
        let mut var_xgd_ov: f64 = *var_xgd_ov_slot;
        let mut var_xgd_ov_dn5: f64 = *var_xgd_ov_dn5_slot;
        let mut var_xgd_ov_dn6: f64 = *var_xgd_ov_dn6_slot;
        let mut var_xgd_ov_dn7: f64 = *var_xgd_ov_dn7_slot;
        let mut var_xgd_ov_rv: f64 = *var_xgd_ov_rv_slot;
        let mut var_xgs_ov: f64 = *var_xgs_ov_slot;
        let mut var_xgs_ov_dn5: f64 = *var_xgs_ov_dn5_slot;
        let mut var_xgs_ov_dn6: f64 = *var_xgs_ov_dn6_slot;
        let mut var_xgs_ov_dn7: f64 = *var_xgs_ov_dn7_slot;
        let mut var_xgs_ov_rv: f64 = *var_xgs_ov_rv_slot;
        let mut var_zflagbot_d: f64 = *var_zflagbot_d_slot;
        let mut var_zflagbot_d_rv: f64 = *var_zflagbot_d_rv_slot;
        let mut var_zflagbot_s: f64 = *var_zflagbot_s_slot;
        let mut var_zflagbot_s_rv: f64 = *var_zflagbot_s_rv_slot;
        let mut var_zflaggat_d: f64 = *var_zflaggat_d_slot;
        let mut var_zflaggat_d_rv: f64 = *var_zflaggat_d_rv_slot;
        let mut var_zflaggat_s: f64 = *var_zflaggat_s_slot;
        let mut var_zflaggat_s_rv: f64 = *var_zflaggat_s_rv_slot;
        let mut var_zflagsti_d: f64 = *var_zflagsti_d_slot;
        let mut var_zflagsti_d_rv: f64 = *var_zflagsti_d_rv_slot;
        let mut var_zflagsti_s: f64 = *var_zflagsti_s_slot;
        let mut var_zflagsti_s_rv: f64 = *var_zflagsti_s_rv_slot;
        let mut var_zfrac: f64 = *var_zfrac_slot;
        let mut var_zfrac_rv: f64 = *var_zfrac_rv_slot;

        let (assign14610_e12269,) = {
    if ((var_guard176 != 0.0) && (var_guard190 != 0.0)) {
        let assign14610_e12267: f64 = (var_vbisti_d + var_vbigat_d);
        (assign14610_e12267,)
    } else {
        (var_vbibot2,)
    }
};
        var_vbibot2 = assign14610_e12269;
        var_vbibot2_rv = 0.0;

        let (assign14620_e12279,) = {
    if ((var_guard176 != 0.0) && (var_guard190 != 0.0)) {
        let assign14620_e12276: f64 = (var_pstid_i).min(var_pgatd_i);
        let assign14620_e12277: f64 = (0.9 * assign14620_e12276);
        (assign14620_e12277,)
    } else {
        (var_pbot2,)
    }
};
        var_pbot2 = assign14620_e12279;
        var_pbot2_rv = 0.0;

        let (assign14630_e12287,) = {
    if ((var_guard176 != 0.0) && (var_guard190 != 0.0)) {
        let assign14630_e12285: f64 = (var_vbirstid_i + var_vbirgatd_i);
        (assign14630_e12285,)
    } else {
        (var_vbibot2r,)
    }
};
        var_vbibot2r = assign14630_e12287;
        var_vbibot2r_rv = 0.0;

        let assign14640_e12290: f64 = if var_lsdrain_i == 0.0 { 1.0 } else { 0.0 };
        var_guard191 = assign14640_e12290;
        var_guard191_rv = 0.0;

        let (assign14650_e12298,) = {
    if ((var_guard176 != 0.0) && (var_guard191 != 0.0)) {
        let assign14650_e12296: f64 = (var_vbibot_d + var_vbigat_d);
        (assign14650_e12296,)
    } else {
        (var_vbisti2,)
    }
};
        var_vbisti2 = assign14650_e12298;
        var_vbisti2_rv = 0.0;

        let (assign14660_e12308,) = {
    if ((var_guard176 != 0.0) && (var_guard191 != 0.0)) {
        let assign14660_e12305: f64 = (var_pbotd_i).min(var_pgatd_i);
        let assign14660_e12306: f64 = (0.9 * assign14660_e12305);
        (assign14660_e12306,)
    } else {
        (var_psti2,)
    }
};
        var_psti2 = assign14660_e12308;
        var_psti2_rv = 0.0;

        let (assign14670_e12316,) = {
    if ((var_guard176 != 0.0) && (var_guard191 != 0.0)) {
        let assign14670_e12314: f64 = (var_vbirbotd_i + var_vbirgatd_i);
        (assign14670_e12314,)
    } else {
        (var_vbisti2r,)
    }
};
        var_vbisti2r = assign14670_e12316;
        var_vbisti2r_rv = 0.0;

        let assign14680_e12319: f64 = if var_lgdrain_i == 0.0 { 1.0 } else { 0.0 };
        var_guard192 = assign14680_e12319;
        var_guard192_rv = 0.0;

        let (assign14690_e12327,) = {
    if ((var_guard176 != 0.0) && (var_guard192 != 0.0)) {
        let assign14690_e12325: f64 = (var_vbibot_d + var_vbisti_d);
        (assign14690_e12325,)
    } else {
        (var_vbigat2,)
    }
};
        var_vbigat2 = assign14690_e12327;
        var_vbigat2_rv = 0.0;

        let (assign14700_e12337,) = {
    if ((var_guard176 != 0.0) && (var_guard192 != 0.0)) {
        let assign14700_e12334: f64 = (var_pbotd_i).min(var_pstid_i);
        let assign14700_e12335: f64 = (0.9 * assign14700_e12334);
        (assign14700_e12335,)
    } else {
        (var_pgat2,)
    }
};
        var_pgat2 = assign14700_e12337;
        var_pgat2_rv = 0.0;

        let (assign14710_e12345,) = {
    if ((var_guard176 != 0.0) && (var_guard192 != 0.0)) {
        let assign14710_e12343: f64 = (var_vbirbotd_i + var_vbirstid_i);
        (assign14710_e12343,)
    } else {
        (var_vbigat2r,)
    }
};
        var_vbigat2r = assign14710_e12345;
        var_vbigat2r_rv = 0.0;

        let (assign14720_e12353,) = {
    if (var_guard176 != 0.0) {
        let assign14720_e12349: f64 = (var_vbibot2).min(var_vbisti2);
        let assign14720_e12351: f64 = (assign14720_e12349).min(var_vbigat2);
        (assign14720_e12351,)
    } else {
        (var_vbimin_d,)
    }
};
        var_vbimin_d = assign14720_e12353;
        var_vbimin_d_rv = 0.0;

        let (assign14730_e12359,) = {
    if (var_guard176 != 0.0) {
        let assign14730_e12357: f64 = (var_vbimin_d * 0.1);
        (assign14730_e12357,)
    } else {
        (var_vch_d,)
    }
};
        var_vch_d = assign14730_e12359;
        var_vch_d_rv = 0.0;

        let (assign14740_e12367,) = {
    if (var_guard176 != 0.0) {
        let assign14740_e12363: f64 = (var_pbot2).max(var_psti2);
        let assign14740_e12365: f64 = (assign14740_e12363).max(var_pgat2);
        (assign14740_e12365,)
    } else {
        (var_pmax,)
    }
};
        var_pmax = assign14740_e12367;
        var_pmax_rv = 0.0;

        let (assign14750_e12380,) = {
    if (var_guard176 != 0.0) {
        let assign14750_e12373: f64 = (-1.0);
        let assign14750_e12375: f64 = (assign14750_e12373 / var_pmax);
        let assign14750_e12376: f64 = (2.0_f64).powf(assign14750_e12375);
        let assign14750_e12377: f64 = (1.0 - assign14750_e12376);
        let assign14750_e12378: f64 = (var_vbimin_d * assign14750_e12377);
        (assign14750_e12378,)
    } else {
        (var_vfmin_d,)
    }
};
        var_vfmin_d = assign14750_e12380;
        var_vfmin_d_rv = 0.0;

        let (assign14760_e12390,) = {
    if (var_guard176 != 0.0) {
        let assign14760_e12384: f64 = (var_vbibot2r).min(var_vbisti2r);
        let assign14760_e12386: f64 = (assign14760_e12384).min(var_vbigat2r);
        let assign14760_e12388: f64 = (assign14760_e12386 - 0.05);
        (assign14760_e12388,)
    } else {
        (var_vbbtlim_d,)
    }
};
        var_vbbtlim_d = assign14760_e12390;
        var_vbbtlim_d_rv = 0.0;

        let assign14770_e12393: f64 = if var_swjunexp_i == 1.0 { 1.0 } else { 0.0 };
        var_guard193 = assign14770_e12393;
        var_guard193_rv = 0.0;

        let (assign27590_e32914,) = {
    if ((var_guard176 != 0.0) && (var_guard193 != 0.0)) {
        let assign27590_e32903: f64 = (var_absource_i * var_cjobot);
        let assign27590_e32906: f64 = (var_lssource_i * var_cjosti);
        let assign27590_e32907: f64 = (assign27590_e32903 + assign27590_e32906);
        let assign27590_e32910: f64 = (var_lgsource_i * var_cjogat);
        let assign27590_e32911: f64 = (assign27590_e32907 + assign27590_e32910);
        let assign27590_e32912: f64 = (p.p922 * assign27590_e32911);
        (assign27590_e32912,)
    } else {
        (var_zfrac,)
    }
};
        var_zfrac = assign27590_e32914;
        var_zfrac_rv = 0.0;

        let assign27600_e32917: f64 = (var_absource_i * var_cjobot);
        let assign27600_e32919: f64 = if assign27600_e32917 <= var_zfrac { 1.0 } else { 0.0 };
        var_guard528 = assign27600_e32919;
        var_guard528_rv = 0.0;

        let (assign27610_e32927,) = {
    if (((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard528 != 0.0)) {
        (0.0,)
    } else {
        (var_zflagbot_s,)
    }
};
        var_zflagbot_s = assign27610_e32927;
        var_zflagbot_s_rv = 0.0;

        let assign27620_e32930: f64 = (var_lssource_i * var_cjosti);
        let assign27620_e32932: f64 = if assign27620_e32930 <= var_zfrac { 1.0 } else { 0.0 };
        var_guard529 = assign27620_e32932;
        var_guard529_rv = 0.0;

        let (assign27630_e32940,) = {
    if (((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard529 != 0.0)) {
        (0.0,)
    } else {
        (var_zflagsti_s,)
    }
};
        var_zflagsti_s = assign27630_e32940;
        var_zflagsti_s_rv = 0.0;

        let assign27640_e32943: f64 = (var_lgsource_i * var_cjogat);
        let assign27640_e32945: f64 = if assign27640_e32943 <= var_zfrac { 1.0 } else { 0.0 };
        var_guard530 = assign27640_e32945;
        var_guard530_rv = 0.0;

        let (assign27650_e32953,) = {
    if (((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard530 != 0.0)) {
        (0.0,)
    } else {
        (var_zflaggat_s,)
    }
};
        var_zflaggat_s = assign27650_e32953;
        var_zflaggat_s_rv = 0.0;

        let (assign40120_e53307,) = {
    if ((var_guard176 != 0.0) && (var_guard193 != 0.0)) {
        let assign40120_e53296: f64 = (var_abdrain_i * var_cjobot_d);
        let assign40120_e53299: f64 = (var_lsdrain_i * var_cjosti_d);
        let assign40120_e53300: f64 = (assign40120_e53296 + assign40120_e53299);
        let assign40120_e53303: f64 = (var_lgdrain_i * var_cjogat_d);
        let assign40120_e53304: f64 = (assign40120_e53300 + assign40120_e53303);
        let assign40120_e53305: f64 = (var_fjunqd_i * assign40120_e53304);
        (assign40120_e53305,)
    } else {
        (var_zfrac,)
    }
};
        var_zfrac = assign40120_e53307;
        var_zfrac_rv = 0.0;

        let assign40130_e53310: f64 = (var_abdrain_i * var_cjobot_d);
        let assign40130_e53312: f64 = if assign40130_e53310 <= var_zfrac { 1.0 } else { 0.0 };
        var_guard818 = assign40130_e53312;
        var_guard818_rv = 0.0;

        let (assign40140_e53320,) = {
    if (((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard818 != 0.0)) {
        (0.0,)
    } else {
        (var_zflagbot_d,)
    }
};
        var_zflagbot_d = assign40140_e53320;
        var_zflagbot_d_rv = 0.0;

        let assign40150_e53323: f64 = (var_lsdrain_i * var_cjosti_d);
        let assign40150_e53325: f64 = if assign40150_e53323 <= var_zfrac { 1.0 } else { 0.0 };
        var_guard819 = assign40150_e53325;
        var_guard819_rv = 0.0;

        let (assign40160_e53333,) = {
    if (((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard819 != 0.0)) {
        (0.0,)
    } else {
        (var_zflagsti_d,)
    }
};
        var_zflagsti_d = assign40160_e53333;
        var_zflagsti_d_rv = 0.0;

        let assign40170_e53336: f64 = (var_lgdrain_i * var_cjogat_d);
        let assign40170_e53338: f64 = if assign40170_e53336 <= var_zfrac { 1.0 } else { 0.0 };
        var_guard820 = assign40170_e53338;
        var_guard820_rv = 0.0;

        let (assign40180_e53346,) = {
    if (((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard820 != 0.0)) {
        (0.0,)
    } else {
        (var_zflaggat_d,)
    }
};
        var_zflaggat_d = assign40180_e53346;
        var_zflaggat_d_rv = 0.0;

        var_temp__blk936 = 0.0;
        var_temp__blk936_dn5 = 0.0;
        var_temp__blk936_dn6 = 0.0;
        var_temp__blk936_dn7 = 0.0;
        var_temp__blk936_dn8 = 0.0;
        var_temp__blk936_rv = 0.0;

        var_temp1 = 0.0;
        var_temp1_dn5 = 0.0;
        var_temp1_dn6 = 0.0;
        var_temp1_dn7 = 0.0;
        var_temp1_dn8 = 0.0;
        var_temp1_rv = 0.0;

        var_temp2 = 0.0;
        var_temp2_dn5 = 0.0;
        var_temp2_dn6 = 0.0;
        var_temp2_dn7 = 0.0;
        var_temp2_dn8 = 0.0;
        var_temp2_rv = 0.0;

        let assign40320_e53455: f64 = 1.0;
        let assign40320_e53456: f64 = if var_chnl_type == assign40320_e53455 { 1.0 } else { 0.0 };
        var_guard1011 = assign40320_e53456;
        var_guard1011_rv = 0.0;

        let (assign40330_e53460, assign40330_e53460_d_n5, assign40330_e53460_d_n6, assign40330_e53460_d_n7,) = {
    if (var_guard1011 != 0.0) {
        ((nv5 - nv6), 1.0, -1.0, 0.0,)
    } else {
        (var_v_gs, var_v_gs_dn5, var_v_gs_dn6, var_v_gs_dn7,)
    }
};
        var_v_gs = assign40330_e53460;
        var_v_gs_dn5 = assign40330_e53460_d_n5;
        var_v_gs_dn6 = assign40330_e53460_d_n6;
        var_v_gs_dn7 = assign40330_e53460_d_n7;
        var_v_gs_rv = 0.0;

        let (assign40340_e53464, assign40340_e53464_d_n6, assign40340_e53464_d_n7,) = {
    if (var_guard1011 != 0.0) {
        ((nv7 - nv6), -1.0, 1.0,)
    } else {
        (var_v_ds, var_v_ds_dn6, var_v_ds_dn7,)
    }
};
        var_v_ds = assign40340_e53464;
        var_v_ds_dn6 = assign40340_e53464_d_n6;
        var_v_ds_dn7 = assign40340_e53464_d_n7;
        var_v_ds_rv = 0.0;

        let (assign40350_e53468, assign40350_e53468_d_n6, assign40350_e53468_d_n7, assign40350_e53468_d_n8,) = {
    if (var_guard1011 != 0.0) {
        ((nv6 - nv8), 1.0, 0.0, -1.0,)
    } else {
        (var_v_sb, var_v_sb_dn6, var_v_sb_dn7, var_v_sb_dn8,)
    }
};
        var_v_sb = assign40350_e53468;
        var_v_sb_dn6 = assign40350_e53468_d_n6;
        var_v_sb_dn7 = assign40350_e53468_d_n7;
        var_v_sb_dn8 = assign40350_e53468_d_n8;
        var_v_sb_rv = 0.0;

        let (assign40360_e53473, assign40360_e53473_d_n6, assign40360_e53473_d_n10,) = {
    if (var_guard1011 != 0.0) {
        let assign40360_e53471: f64 = (-(nv6 - nv10));
        (assign40360_e53471, (-1.0), 1.0,)
    } else {
        (var_vjun_s, var_vjun_s_dn6, var_vjun_s_dn10,)
    }
};
        var_vjun_s = assign40360_e53473;
        var_vjun_s_dn6 = assign40360_e53473_d_n6;
        var_vjun_s_dn10 = assign40360_e53473_d_n10;
        var_vjun_s_rv = 0.0;

        let (assign40370_e53478, assign40370_e53478_d_n7, assign40370_e53478_d_n11,) = {
    if (var_guard1011 != 0.0) {
        let assign40370_e53476: f64 = (-(nv7 - nv11));
        (assign40370_e53476, (-1.0), 1.0,)
    } else {
        (var_vjun_d, var_vjun_d_dn7, var_vjun_d_dn11,)
    }
};
        var_vjun_d = assign40370_e53478;
        var_vjun_d_dn7 = assign40370_e53478_d_n7;
        var_vjun_d_dn11 = assign40370_e53478_d_n11;
        var_vjun_d_rv = 0.0;

        let (assign40380_e53484, assign40380_e53484_d_n5, assign40380_e53484_d_n6, assign40380_e53484_d_n7,) = {
    if (var_guard1011 == 0.0) {
        let assign40380_e53482: f64 = (-(nv5 - nv6));
        (assign40380_e53482, (-1.0), 1.0, 0.0,)
    } else {
        (var_v_gs, var_v_gs_dn5, var_v_gs_dn6, var_v_gs_dn7,)
    }
};
        var_v_gs = assign40380_e53484;
        var_v_gs_dn5 = assign40380_e53484_d_n5;
        var_v_gs_dn6 = assign40380_e53484_d_n6;
        var_v_gs_dn7 = assign40380_e53484_d_n7;
        var_v_gs_rv = 0.0;

        let (assign40390_e53490, assign40390_e53490_d_n6, assign40390_e53490_d_n7,) = {
    if (var_guard1011 == 0.0) {
        let assign40390_e53488: f64 = (-(nv7 - nv6));
        (assign40390_e53488, 1.0, (-1.0),)
    } else {
        (var_v_ds, var_v_ds_dn6, var_v_ds_dn7,)
    }
};
        var_v_ds = assign40390_e53490;
        var_v_ds_dn6 = assign40390_e53490_d_n6;
        var_v_ds_dn7 = assign40390_e53490_d_n7;
        var_v_ds_rv = 0.0;

        let (assign40400_e53496, assign40400_e53496_d_n6, assign40400_e53496_d_n7, assign40400_e53496_d_n8,) = {
    if (var_guard1011 == 0.0) {
        let assign40400_e53494: f64 = (-(nv6 - nv8));
        (assign40400_e53494, (-1.0), 0.0, 1.0,)
    } else {
        (var_v_sb, var_v_sb_dn6, var_v_sb_dn7, var_v_sb_dn8,)
    }
};
        var_v_sb = assign40400_e53496;
        var_v_sb_dn6 = assign40400_e53496_d_n6;
        var_v_sb_dn7 = assign40400_e53496_d_n7;
        var_v_sb_dn8 = assign40400_e53496_d_n8;
        var_v_sb_rv = 0.0;

        let (assign40410_e53501, assign40410_e53501_d_n6, assign40410_e53501_d_n10,) = {
    if (var_guard1011 == 0.0) {
        ((nv6 - nv10), 1.0, -1.0,)
    } else {
        (var_vjun_s, var_vjun_s_dn6, var_vjun_s_dn10,)
    }
};
        var_vjun_s = assign40410_e53501;
        var_vjun_s_dn6 = assign40410_e53501_d_n6;
        var_vjun_s_dn10 = assign40410_e53501_d_n10;
        var_vjun_s_rv = 0.0;

        let (assign40420_e53506, assign40420_e53506_d_n7, assign40420_e53506_d_n11,) = {
    if (var_guard1011 == 0.0) {
        ((nv7 - nv11), 1.0, -1.0,)
    } else {
        (var_vjun_d, var_vjun_d_dn7, var_vjun_d_dn11,)
    }
};
        var_vjun_d = assign40420_e53506;
        var_vjun_d_dn7 = assign40420_e53506_d_n7;
        var_vjun_d_dn11 = assign40420_e53506_d_n11;
        var_vjun_d_rv = 0.0;

        let assign40430_e53509: f64 = (var_v_gs + var_v_sb);
        var_vgb = assign40430_e53509;
        var_vgb_dn5 = var_v_gs_dn5;
        var_vgb_dn6 = (var_v_gs_dn6 + var_v_sb_dn6);
        var_vgb_dn7 = (var_v_gs_dn7 + var_v_sb_dn7);
        var_vgb_dn8 = var_v_sb_dn8;
        var_vgb_rv = 0.0;

        var_vgsprime = var_v_gs;
        var_vgsprime_dn5 = var_v_gs_dn5;
        var_vgsprime_dn6 = var_v_gs_dn6;
        var_vgsprime_dn7 = var_v_gs_dn7;
        var_vgsprime_rv = 0.0;

        var_vsbprime = var_v_sb;
        var_vsbprime_dn6 = var_v_sb_dn6;
        var_vsbprime_dn7 = var_v_sb_dn7;
        var_vsbprime_dn8 = var_v_sb_dn8;
        var_vsbprime_rv = 0.0;

        let assign40460_e53514: f64 = (var_v_ds + var_v_sb);
        var_vdbprime = assign40460_e53514;
        var_vdbprime_dn6 = (var_v_ds_dn6 + var_v_sb_dn6);
        var_vdbprime_dn7 = (var_v_ds_dn7 + var_v_sb_dn7);
        var_vdbprime_dn8 = var_v_sb_dn8;
        var_vdbprime_rv = 0.0;

        let assign40470_e53517: f64 = (var_v_gs - var_v_ds);
        var_vgdprime = assign40470_e53517;
        var_vgdprime_dn5 = var_v_gs_dn5;
        var_vgdprime_dn6 = (var_v_gs_dn6 - var_v_ds_dn6);
        var_vgdprime_dn7 = (var_v_gs_dn7 - var_v_ds_dn7);
        var_vgdprime_rv = 0.0;

        let assign40480_e53519: f64 = (-var_vgsprime);
        let assign40480_e53521: f64 = (assign40480_e53519 * var_inv_phita);
        var_xgs_ov = assign40480_e53521;
        var_xgs_ov_dn5 = ((-var_vgsprime_dn5) * var_inv_phita);
        var_xgs_ov_dn6 = ((-var_vgsprime_dn6) * var_inv_phita);
        var_xgs_ov_dn7 = ((-var_vgsprime_dn7) * var_inv_phita);
        var_xgs_ov_rv = 0.0;

        let assign40490_e53523: f64 = (-var_vgdprime);
        let assign40490_e53525: f64 = (assign40490_e53523 * var_inv_phita);
        var_xgd_ov = assign40490_e53525;
        var_xgd_ov_dn5 = ((-var_vgdprime_dn5) * var_inv_phita);
        var_xgd_ov_dn6 = ((-var_vgdprime_dn6) * var_inv_phita);
        var_xgd_ov_dn7 = ((-var_vgdprime_dn7) * var_inv_phita);
        var_xgd_ov_rv = 0.0;

        *var_guard1011_slot = var_guard1011;
        *var_guard1011_rv_slot = var_guard1011_rv;
        *var_guard191_slot = var_guard191;
        *var_guard191_rv_slot = var_guard191_rv;
        *var_guard192_slot = var_guard192;
        *var_guard192_rv_slot = var_guard192_rv;
        *var_guard193_slot = var_guard193;
        *var_guard193_rv_slot = var_guard193_rv;
        *var_guard528_slot = var_guard528;
        *var_guard528_rv_slot = var_guard528_rv;
        *var_guard529_slot = var_guard529;
        *var_guard529_rv_slot = var_guard529_rv;
        *var_guard530_slot = var_guard530;
        *var_guard530_rv_slot = var_guard530_rv;
        *var_guard818_slot = var_guard818;
        *var_guard818_rv_slot = var_guard818_rv;
        *var_guard819_slot = var_guard819;
        *var_guard819_rv_slot = var_guard819_rv;
        *var_guard820_slot = var_guard820;
        *var_guard820_rv_slot = var_guard820_rv;
        *var_pbot2_slot = var_pbot2;
        *var_pbot2_rv_slot = var_pbot2_rv;
        *var_pgat2_slot = var_pgat2;
        *var_pgat2_rv_slot = var_pgat2_rv;
        *var_pmax_slot = var_pmax;
        *var_pmax_rv_slot = var_pmax_rv;
        *var_psti2_slot = var_psti2;
        *var_psti2_rv_slot = var_psti2_rv;
        *var_temp1_slot = var_temp1;
        *var_temp1_dn5_slot = var_temp1_dn5;
        *var_temp1_dn6_slot = var_temp1_dn6;
        *var_temp1_dn7_slot = var_temp1_dn7;
        *var_temp1_dn8_slot = var_temp1_dn8;
        *var_temp1_rv_slot = var_temp1_rv;
        *var_temp2_slot = var_temp2;
        *var_temp2_dn5_slot = var_temp2_dn5;
        *var_temp2_dn6_slot = var_temp2_dn6;
        *var_temp2_dn7_slot = var_temp2_dn7;
        *var_temp2_dn8_slot = var_temp2_dn8;
        *var_temp2_rv_slot = var_temp2_rv;
        *var_temp__blk936_slot = var_temp__blk936;
        *var_temp__blk936_dn5_slot = var_temp__blk936_dn5;
        *var_temp__blk936_dn6_slot = var_temp__blk936_dn6;
        *var_temp__blk936_dn7_slot = var_temp__blk936_dn7;
        *var_temp__blk936_dn8_slot = var_temp__blk936_dn8;
        *var_temp__blk936_rv_slot = var_temp__blk936_rv;
        *var_v_ds_slot = var_v_ds;
        *var_v_ds_dn6_slot = var_v_ds_dn6;
        *var_v_ds_dn7_slot = var_v_ds_dn7;
        *var_v_ds_rv_slot = var_v_ds_rv;
        *var_v_gs_slot = var_v_gs;
        *var_v_gs_dn5_slot = var_v_gs_dn5;
        *var_v_gs_dn6_slot = var_v_gs_dn6;
        *var_v_gs_dn7_slot = var_v_gs_dn7;
        *var_v_gs_rv_slot = var_v_gs_rv;
        *var_v_sb_slot = var_v_sb;
        *var_v_sb_dn6_slot = var_v_sb_dn6;
        *var_v_sb_dn7_slot = var_v_sb_dn7;
        *var_v_sb_dn8_slot = var_v_sb_dn8;
        *var_v_sb_rv_slot = var_v_sb_rv;
        *var_vbbtlim_d_slot = var_vbbtlim_d;
        *var_vbbtlim_d_rv_slot = var_vbbtlim_d_rv;
        *var_vbibot2_slot = var_vbibot2;
        *var_vbibot2_rv_slot = var_vbibot2_rv;
        *var_vbibot2r_slot = var_vbibot2r;
        *var_vbibot2r_rv_slot = var_vbibot2r_rv;
        *var_vbigat2_slot = var_vbigat2;
        *var_vbigat2_rv_slot = var_vbigat2_rv;
        *var_vbigat2r_slot = var_vbigat2r;
        *var_vbigat2r_rv_slot = var_vbigat2r_rv;
        *var_vbimin_d_slot = var_vbimin_d;
        *var_vbimin_d_rv_slot = var_vbimin_d_rv;
        *var_vbisti2_slot = var_vbisti2;
        *var_vbisti2_rv_slot = var_vbisti2_rv;
        *var_vbisti2r_slot = var_vbisti2r;
        *var_vbisti2r_rv_slot = var_vbisti2r_rv;
        *var_vch_d_slot = var_vch_d;
        *var_vch_d_rv_slot = var_vch_d_rv;
        *var_vdbprime_slot = var_vdbprime;
        *var_vdbprime_dn6_slot = var_vdbprime_dn6;
        *var_vdbprime_dn7_slot = var_vdbprime_dn7;
        *var_vdbprime_dn8_slot = var_vdbprime_dn8;
        *var_vdbprime_rv_slot = var_vdbprime_rv;
        *var_vfmin_d_slot = var_vfmin_d;
        *var_vfmin_d_rv_slot = var_vfmin_d_rv;
        *var_vgb_slot = var_vgb;
        *var_vgb_dn5_slot = var_vgb_dn5;
        *var_vgb_dn6_slot = var_vgb_dn6;
        *var_vgb_dn7_slot = var_vgb_dn7;
        *var_vgb_dn8_slot = var_vgb_dn8;
        *var_vgb_rv_slot = var_vgb_rv;
        *var_vgdprime_slot = var_vgdprime;
        *var_vgdprime_dn5_slot = var_vgdprime_dn5;
        *var_vgdprime_dn6_slot = var_vgdprime_dn6;
        *var_vgdprime_dn7_slot = var_vgdprime_dn7;
        *var_vgdprime_rv_slot = var_vgdprime_rv;
        *var_vgsprime_slot = var_vgsprime;
        *var_vgsprime_dn5_slot = var_vgsprime_dn5;
        *var_vgsprime_dn6_slot = var_vgsprime_dn6;
        *var_vgsprime_dn7_slot = var_vgsprime_dn7;
        *var_vgsprime_rv_slot = var_vgsprime_rv;
        *var_vjun_d_slot = var_vjun_d;
        *var_vjun_d_dn11_slot = var_vjun_d_dn11;
        *var_vjun_d_dn7_slot = var_vjun_d_dn7;
        *var_vjun_d_rv_slot = var_vjun_d_rv;
        *var_vjun_s_slot = var_vjun_s;
        *var_vjun_s_dn10_slot = var_vjun_s_dn10;
        *var_vjun_s_dn6_slot = var_vjun_s_dn6;
        *var_vjun_s_rv_slot = var_vjun_s_rv;
        *var_vsbprime_slot = var_vsbprime;
        *var_vsbprime_dn6_slot = var_vsbprime_dn6;
        *var_vsbprime_dn7_slot = var_vsbprime_dn7;
        *var_vsbprime_dn8_slot = var_vsbprime_dn8;
        *var_vsbprime_rv_slot = var_vsbprime_rv;
        *var_xgd_ov_slot = var_xgd_ov;
        *var_xgd_ov_dn5_slot = var_xgd_ov_dn5;
        *var_xgd_ov_dn6_slot = var_xgd_ov_dn6;
        *var_xgd_ov_dn7_slot = var_xgd_ov_dn7;
        *var_xgd_ov_rv_slot = var_xgd_ov_rv;
        *var_xgs_ov_slot = var_xgs_ov;
        *var_xgs_ov_dn5_slot = var_xgs_ov_dn5;
        *var_xgs_ov_dn6_slot = var_xgs_ov_dn6;
        *var_xgs_ov_dn7_slot = var_xgs_ov_dn7;
        *var_xgs_ov_rv_slot = var_xgs_ov_rv;
        *var_zflagbot_d_slot = var_zflagbot_d;
        *var_zflagbot_d_rv_slot = var_zflagbot_d_rv;
        *var_zflagbot_s_slot = var_zflagbot_s;
        *var_zflagbot_s_rv_slot = var_zflagbot_s_rv;
        *var_zflaggat_d_slot = var_zflaggat_d;
        *var_zflaggat_d_rv_slot = var_zflaggat_d_rv;
        *var_zflaggat_s_slot = var_zflaggat_s;
        *var_zflaggat_s_rv_slot = var_zflaggat_s_rv;
        *var_zflagsti_d_slot = var_zflagsti_d;
        *var_zflagsti_d_rv_slot = var_zflagsti_d_rv;
        *var_zflagsti_s_slot = var_zflagsti_s;
        *var_zflagsti_s_rv_slot = var_zflagsti_s_rv;
        *var_zfrac_slot = var_zfrac;
        *var_zfrac_rv_slot = var_zfrac_rv;
    }

    pub(super) fn stamp_reactive_block_23(
        p: &Parameters,
        var_aphi_dc: f64,
        var_ar: f64,
        var_bphi_dc: f64,
        var_ctb_i: f64,
        var_ctg_i: f64,
        var_g_0_dc: f64,
        var_gfacnud_i: f64,
        var_inv_phit: f64,
        var_inv_phita: f64,
        var_phib_dc: f64,
        var_phix1_dc: f64,
        var_phix_dc: f64,
        var_sqrt_phib_dc: f64,
        var_thesat_t: f64,
        var_us1: f64,
        var_us21: f64,
        var_vfb_t: f64,
        var_vgb: f64,
        var_vgb_dn5: f64,
        var_vgb_dn6: f64,
        var_vgb_dn7: f64,
        var_vgb_dn8: f64,
        var_aphi_slot: &mut f64,
        var_aphi_rv_slot: &mut f64,
        var_arloc_slot: &mut f64,
        var_arloc_rv_slot: &mut f64,
        var_dctg_slot: &mut f64,
        var_dctg_dn5_slot: &mut f64,
        var_dctg_dn6_slot: &mut f64,
        var_dctg_dn7_slot: &mut f64,
        var_dctg_dn8_slot: &mut f64,
        var_dctg_rv_slot: &mut f64,
        var_dvbstar_slot: &mut f64,
        var_dvbstar_dc_slot: &mut f64,
        var_dvbstar_dc_dn5_slot: &mut f64,
        var_dvbstar_dc_dn6_slot: &mut f64,
        var_dvbstar_dc_dn7_slot: &mut f64,
        var_dvbstar_dc_dn8_slot: &mut f64,
        var_dvbstar_dc_rv_slot: &mut f64,
        var_dvbstar_dn5_slot: &mut f64,
        var_dvbstar_dn6_slot: &mut f64,
        var_dvbstar_dn7_slot: &mut f64,
        var_dvbstar_dn8_slot: &mut f64,
        var_dvbstar_rv_slot: &mut f64,
        var_g_0_slot: &mut f64,
        var_g_0_rv_slot: &mut f64,
        var_guard1012_slot: &mut f64,
        var_guard1012_rv_slot: &mut f64,
        var_guard1172_slot: &mut f64,
        var_guard1172_rv_slot: &mut f64,
        var_guard1173_slot: &mut f64,
        var_guard1173_rv_slot: &mut f64,
        var_phib_slot: &mut f64,
        var_phib_rv_slot: &mut f64,
        var_sigvds_slot: &mut f64,
        var_sigvds_rv_slot: &mut f64,
        var_temp1_slot: &mut f64,
        var_temp1_dn5_slot: &mut f64,
        var_temp1_dn6_slot: &mut f64,
        var_temp1_dn7_slot: &mut f64,
        var_temp1_dn8_slot: &mut f64,
        var_temp1_rv_slot: &mut f64,
        var_temp2_slot: &mut f64,
        var_temp2_dn5_slot: &mut f64,
        var_temp2_dn6_slot: &mut f64,
        var_temp2_dn7_slot: &mut f64,
        var_temp2_dn8_slot: &mut f64,
        var_temp2_rv_slot: &mut f64,
        var_temp__blk936_slot: &mut f64,
        var_temp__blk936_dn5_slot: &mut f64,
        var_temp__blk936_dn6_slot: &mut f64,
        var_temp__blk936_dn7_slot: &mut f64,
        var_temp__blk936_dn8_slot: &mut f64,
        var_temp__blk936_rv_slot: &mut f64,
        var_thesatloc_slot: &mut f64,
        var_thesatloc_rv_slot: &mut f64,
        var_us_slot: &mut f64,
        var_us_dn5_slot: &mut f64,
        var_us_dn6_slot: &mut f64,
        var_us_dn7_slot: &mut f64,
        var_us_dn8_slot: &mut f64,
        var_us_rv_slot: &mut f64,
        var_usnew_slot: &mut f64,
        var_usnew_dn5_slot: &mut f64,
        var_usnew_dn6_slot: &mut f64,
        var_usnew_dn7_slot: &mut f64,
        var_usnew_dn8_slot: &mut f64,
        var_usnew_rv_slot: &mut f64,
        var_v_db_slot: &mut f64,
        var_v_db_dn6_slot: &mut f64,
        var_v_db_dn7_slot: &mut f64,
        var_v_db_dn8_slot: &mut f64,
        var_v_db_rv_slot: &mut f64,
        var_v_ds_slot: &mut f64,
        var_v_ds_dn6_slot: &mut f64,
        var_v_ds_dn7_slot: &mut f64,
        var_v_ds_rv_slot: &mut f64,
        var_v_gs_slot: &mut f64,
        var_v_gs_dn5_slot: &mut f64,
        var_v_gs_dn6_slot: &mut f64,
        var_v_gs_dn7_slot: &mut f64,
        var_v_gs_rv_slot: &mut f64,
        var_v_sb_slot: &mut f64,
        var_v_sb_dn6_slot: &mut f64,
        var_v_sb_dn7_slot: &mut f64,
        var_v_sb_dn8_slot: &mut f64,
        var_v_sb_rv_slot: &mut f64,
        var_v_xb_slot: &mut f64,
        var_v_xb_dc_tmp_slot: &mut f64,
        var_v_xb_dc_tmp_dn6_slot: &mut f64,
        var_v_xb_dc_tmp_dn7_slot: &mut f64,
        var_v_xb_dc_tmp_dn8_slot: &mut f64,
        var_v_xb_dc_tmp_rv_slot: &mut f64,
        var_v_xb_dn6_slot: &mut f64,
        var_v_xb_dn7_slot: &mut f64,
        var_v_xb_dn8_slot: &mut f64,
        var_v_xb_rv_slot: &mut f64,
        var_vdsx_slot: &mut f64,
        var_vdsx_dn6_slot: &mut f64,
        var_vdsx_dn7_slot: &mut f64,
        var_vdsx_rv_slot: &mut f64,
        var_vgb1_slot: &mut f64,
        var_vgb1_dn5_slot: &mut f64,
        var_vgb1_dn6_slot: &mut f64,
        var_vgb1_dn7_slot: &mut f64,
        var_vgb1_dn8_slot: &mut f64,
        var_vgb1_rv_slot: &mut f64,
        var_vmb_slot: &mut f64,
        var_vmb_dn5_slot: &mut f64,
        var_vmb_dn6_slot: &mut f64,
        var_vmb_dn7_slot: &mut f64,
        var_vmb_dn8_slot: &mut f64,
        var_vmb_rv_slot: &mut f64,
        var_vmbnew_slot: &mut f64,
        var_vmbnew_dn5_slot: &mut f64,
        var_vmbnew_dn6_slot: &mut f64,
        var_vmbnew_dn7_slot: &mut f64,
        var_vmbnew_dn8_slot: &mut f64,
        var_vmbnew_rv_slot: &mut f64,
        var_vsbstar_slot: &mut f64,
        var_vsbstar_dc_slot: &mut f64,
        var_vsbstar_dc_dn5_slot: &mut f64,
        var_vsbstar_dc_dn6_slot: &mut f64,
        var_vsbstar_dc_dn7_slot: &mut f64,
        var_vsbstar_dc_dn8_slot: &mut f64,
        var_vsbstar_dc_rv_slot: &mut f64,
        var_vsbstar_dc_tmp_slot: &mut f64,
        var_vsbstar_dc_tmp_dn5_slot: &mut f64,
        var_vsbstar_dc_tmp_dn6_slot: &mut f64,
        var_vsbstar_dc_tmp_dn7_slot: &mut f64,
        var_vsbstar_dc_tmp_dn8_slot: &mut f64,
        var_vsbstar_dc_tmp_rv_slot: &mut f64,
        var_vsbstar_dn5_slot: &mut f64,
        var_vsbstar_dn6_slot: &mut f64,
        var_vsbstar_dn7_slot: &mut f64,
        var_vsbstar_dn8_slot: &mut f64,
        var_vsbstar_rv_slot: &mut f64,
        var_vsbx_slot: &mut f64,
        var_vsbx_dn5_slot: &mut f64,
        var_vsbx_dn6_slot: &mut f64,
        var_vsbx_dn7_slot: &mut f64,
        var_vsbx_dn8_slot: &mut f64,
        var_vsbx_rv_slot: &mut f64,
        var_xbct_slot: &mut f64,
        var_xbct_rv_slot: &mut f64,
        var_xctmax_slot: &mut f64,
        var_xctmax_rv_slot: &mut f64,
        var_xgb_ov_slot: &mut f64,
        var_xgb_ov_dn5_slot: &mut f64,
        var_xgb_ov_dn6_slot: &mut f64,
        var_xgb_ov_dn7_slot: &mut f64,
        var_xgb_ov_dn8_slot: &mut f64,
        var_xgb_ov_rv_slot: &mut f64,
        var_xgct_slot: &mut f64,
        var_xgct_dn5_slot: &mut f64,
        var_xgct_dn6_slot: &mut f64,
        var_xgct_dn7_slot: &mut f64,
        var_xgct_dn8_slot: &mut f64,
        var_xgct_rv_slot: &mut f64,
        var_xmict_slot: &mut f64,
        var_xmict_dn5_slot: &mut f64,
        var_xmict_dn6_slot: &mut f64,
        var_xmict_dn7_slot: &mut f64,
        var_xmict_dn8_slot: &mut f64,
        var_xmict_rv_slot: &mut f64,
        var_xnct_slot: &mut f64,
        var_xnct_dn5_slot: &mut f64,
        var_xnct_dn6_slot: &mut f64,
        var_xnct_dn7_slot: &mut f64,
        var_xnct_dn8_slot: &mut f64,
        var_xnct_rv_slot: &mut f64,
        var_xsbstar_slot: &mut f64,
        var_xsbstar_dn5_slot: &mut f64,
        var_xsbstar_dn6_slot: &mut f64,
        var_xsbstar_dn7_slot: &mut f64,
        var_xsbstar_dn8_slot: &mut f64,
        var_xsbstar_rv_slot: &mut f64,
        var_xwict_slot: &mut f64,
        var_xwict_dn5_slot: &mut f64,
        var_xwict_dn6_slot: &mut f64,
        var_xwict_dn7_slot: &mut f64,
        var_xwict_dn8_slot: &mut f64,
        var_xwict_rv_slot: &mut f64,
    ) {
        let mut var_aphi: f64 = *var_aphi_slot;
        let mut var_aphi_rv: f64 = *var_aphi_rv_slot;
        let mut var_arloc: f64 = *var_arloc_slot;
        let mut var_arloc_rv: f64 = *var_arloc_rv_slot;
        let mut var_dctg: f64 = *var_dctg_slot;
        let mut var_dctg_dn5: f64 = *var_dctg_dn5_slot;
        let mut var_dctg_dn6: f64 = *var_dctg_dn6_slot;
        let mut var_dctg_dn7: f64 = *var_dctg_dn7_slot;
        let mut var_dctg_dn8: f64 = *var_dctg_dn8_slot;
        let mut var_dctg_rv: f64 = *var_dctg_rv_slot;
        let mut var_dvbstar: f64 = *var_dvbstar_slot;
        let mut var_dvbstar_dc: f64 = *var_dvbstar_dc_slot;
        let mut var_dvbstar_dc_dn5: f64 = *var_dvbstar_dc_dn5_slot;
        let mut var_dvbstar_dc_dn6: f64 = *var_dvbstar_dc_dn6_slot;
        let mut var_dvbstar_dc_dn7: f64 = *var_dvbstar_dc_dn7_slot;
        let mut var_dvbstar_dc_dn8: f64 = *var_dvbstar_dc_dn8_slot;
        let mut var_dvbstar_dc_rv: f64 = *var_dvbstar_dc_rv_slot;
        let mut var_dvbstar_dn5: f64 = *var_dvbstar_dn5_slot;
        let mut var_dvbstar_dn6: f64 = *var_dvbstar_dn6_slot;
        let mut var_dvbstar_dn7: f64 = *var_dvbstar_dn7_slot;
        let mut var_dvbstar_dn8: f64 = *var_dvbstar_dn8_slot;
        let mut var_dvbstar_rv: f64 = *var_dvbstar_rv_slot;
        let mut var_g_0: f64 = *var_g_0_slot;
        let mut var_g_0_rv: f64 = *var_g_0_rv_slot;
        let mut var_guard1012: f64 = *var_guard1012_slot;
        let mut var_guard1012_rv: f64 = *var_guard1012_rv_slot;
        let mut var_guard1172: f64 = *var_guard1172_slot;
        let mut var_guard1172_rv: f64 = *var_guard1172_rv_slot;
        let mut var_guard1173: f64 = *var_guard1173_slot;
        let mut var_guard1173_rv: f64 = *var_guard1173_rv_slot;
        let mut var_phib: f64 = *var_phib_slot;
        let mut var_phib_rv: f64 = *var_phib_rv_slot;
        let mut var_sigvds: f64 = *var_sigvds_slot;
        let mut var_sigvds_rv: f64 = *var_sigvds_rv_slot;
        let mut var_temp1: f64 = *var_temp1_slot;
        let mut var_temp1_dn5: f64 = *var_temp1_dn5_slot;
        let mut var_temp1_dn6: f64 = *var_temp1_dn6_slot;
        let mut var_temp1_dn7: f64 = *var_temp1_dn7_slot;
        let mut var_temp1_dn8: f64 = *var_temp1_dn8_slot;
        let mut var_temp1_rv: f64 = *var_temp1_rv_slot;
        let mut var_temp2: f64 = *var_temp2_slot;
        let mut var_temp2_dn5: f64 = *var_temp2_dn5_slot;
        let mut var_temp2_dn6: f64 = *var_temp2_dn6_slot;
        let mut var_temp2_dn7: f64 = *var_temp2_dn7_slot;
        let mut var_temp2_dn8: f64 = *var_temp2_dn8_slot;
        let mut var_temp2_rv: f64 = *var_temp2_rv_slot;
        let mut var_temp__blk936: f64 = *var_temp__blk936_slot;
        let mut var_temp__blk936_dn5: f64 = *var_temp__blk936_dn5_slot;
        let mut var_temp__blk936_dn6: f64 = *var_temp__blk936_dn6_slot;
        let mut var_temp__blk936_dn7: f64 = *var_temp__blk936_dn7_slot;
        let mut var_temp__blk936_dn8: f64 = *var_temp__blk936_dn8_slot;
        let mut var_temp__blk936_rv: f64 = *var_temp__blk936_rv_slot;
        let mut var_thesatloc: f64 = *var_thesatloc_slot;
        let mut var_thesatloc_rv: f64 = *var_thesatloc_rv_slot;
        let mut var_us: f64 = *var_us_slot;
        let mut var_us_dn5: f64 = *var_us_dn5_slot;
        let mut var_us_dn6: f64 = *var_us_dn6_slot;
        let mut var_us_dn7: f64 = *var_us_dn7_slot;
        let mut var_us_dn8: f64 = *var_us_dn8_slot;
        let mut var_us_rv: f64 = *var_us_rv_slot;
        let mut var_usnew: f64 = *var_usnew_slot;
        let mut var_usnew_dn5: f64 = *var_usnew_dn5_slot;
        let mut var_usnew_dn6: f64 = *var_usnew_dn6_slot;
        let mut var_usnew_dn7: f64 = *var_usnew_dn7_slot;
        let mut var_usnew_dn8: f64 = *var_usnew_dn8_slot;
        let mut var_usnew_rv: f64 = *var_usnew_rv_slot;
        let mut var_v_db: f64 = *var_v_db_slot;
        let mut var_v_db_dn6: f64 = *var_v_db_dn6_slot;
        let mut var_v_db_dn7: f64 = *var_v_db_dn7_slot;
        let mut var_v_db_dn8: f64 = *var_v_db_dn8_slot;
        let mut var_v_db_rv: f64 = *var_v_db_rv_slot;
        let mut var_v_ds: f64 = *var_v_ds_slot;
        let mut var_v_ds_dn6: f64 = *var_v_ds_dn6_slot;
        let mut var_v_ds_dn7: f64 = *var_v_ds_dn7_slot;
        let mut var_v_ds_rv: f64 = *var_v_ds_rv_slot;
        let mut var_v_gs: f64 = *var_v_gs_slot;
        let mut var_v_gs_dn5: f64 = *var_v_gs_dn5_slot;
        let mut var_v_gs_dn6: f64 = *var_v_gs_dn6_slot;
        let mut var_v_gs_dn7: f64 = *var_v_gs_dn7_slot;
        let mut var_v_gs_rv: f64 = *var_v_gs_rv_slot;
        let mut var_v_sb: f64 = *var_v_sb_slot;
        let mut var_v_sb_dn6: f64 = *var_v_sb_dn6_slot;
        let mut var_v_sb_dn7: f64 = *var_v_sb_dn7_slot;
        let mut var_v_sb_dn8: f64 = *var_v_sb_dn8_slot;
        let mut var_v_sb_rv: f64 = *var_v_sb_rv_slot;
        let mut var_v_xb: f64 = *var_v_xb_slot;
        let mut var_v_xb_dc_tmp: f64 = *var_v_xb_dc_tmp_slot;
        let mut var_v_xb_dc_tmp_dn6: f64 = *var_v_xb_dc_tmp_dn6_slot;
        let mut var_v_xb_dc_tmp_dn7: f64 = *var_v_xb_dc_tmp_dn7_slot;
        let mut var_v_xb_dc_tmp_dn8: f64 = *var_v_xb_dc_tmp_dn8_slot;
        let mut var_v_xb_dc_tmp_rv: f64 = *var_v_xb_dc_tmp_rv_slot;
        let mut var_v_xb_dn6: f64 = *var_v_xb_dn6_slot;
        let mut var_v_xb_dn7: f64 = *var_v_xb_dn7_slot;
        let mut var_v_xb_dn8: f64 = *var_v_xb_dn8_slot;
        let mut var_v_xb_rv: f64 = *var_v_xb_rv_slot;
        let mut var_vdsx: f64 = *var_vdsx_slot;
        let mut var_vdsx_dn6: f64 = *var_vdsx_dn6_slot;
        let mut var_vdsx_dn7: f64 = *var_vdsx_dn7_slot;
        let mut var_vdsx_rv: f64 = *var_vdsx_rv_slot;
        let mut var_vgb1: f64 = *var_vgb1_slot;
        let mut var_vgb1_dn5: f64 = *var_vgb1_dn5_slot;
        let mut var_vgb1_dn6: f64 = *var_vgb1_dn6_slot;
        let mut var_vgb1_dn7: f64 = *var_vgb1_dn7_slot;
        let mut var_vgb1_dn8: f64 = *var_vgb1_dn8_slot;
        let mut var_vgb1_rv: f64 = *var_vgb1_rv_slot;
        let mut var_vmb: f64 = *var_vmb_slot;
        let mut var_vmb_dn5: f64 = *var_vmb_dn5_slot;
        let mut var_vmb_dn6: f64 = *var_vmb_dn6_slot;
        let mut var_vmb_dn7: f64 = *var_vmb_dn7_slot;
        let mut var_vmb_dn8: f64 = *var_vmb_dn8_slot;
        let mut var_vmb_rv: f64 = *var_vmb_rv_slot;
        let mut var_vmbnew: f64 = *var_vmbnew_slot;
        let mut var_vmbnew_dn5: f64 = *var_vmbnew_dn5_slot;
        let mut var_vmbnew_dn6: f64 = *var_vmbnew_dn6_slot;
        let mut var_vmbnew_dn7: f64 = *var_vmbnew_dn7_slot;
        let mut var_vmbnew_dn8: f64 = *var_vmbnew_dn8_slot;
        let mut var_vmbnew_rv: f64 = *var_vmbnew_rv_slot;
        let mut var_vsbstar: f64 = *var_vsbstar_slot;
        let mut var_vsbstar_dc: f64 = *var_vsbstar_dc_slot;
        let mut var_vsbstar_dc_dn5: f64 = *var_vsbstar_dc_dn5_slot;
        let mut var_vsbstar_dc_dn6: f64 = *var_vsbstar_dc_dn6_slot;
        let mut var_vsbstar_dc_dn7: f64 = *var_vsbstar_dc_dn7_slot;
        let mut var_vsbstar_dc_dn8: f64 = *var_vsbstar_dc_dn8_slot;
        let mut var_vsbstar_dc_rv: f64 = *var_vsbstar_dc_rv_slot;
        let mut var_vsbstar_dc_tmp: f64 = *var_vsbstar_dc_tmp_slot;
        let mut var_vsbstar_dc_tmp_dn5: f64 = *var_vsbstar_dc_tmp_dn5_slot;
        let mut var_vsbstar_dc_tmp_dn6: f64 = *var_vsbstar_dc_tmp_dn6_slot;
        let mut var_vsbstar_dc_tmp_dn7: f64 = *var_vsbstar_dc_tmp_dn7_slot;
        let mut var_vsbstar_dc_tmp_dn8: f64 = *var_vsbstar_dc_tmp_dn8_slot;
        let mut var_vsbstar_dc_tmp_rv: f64 = *var_vsbstar_dc_tmp_rv_slot;
        let mut var_vsbstar_dn5: f64 = *var_vsbstar_dn5_slot;
        let mut var_vsbstar_dn6: f64 = *var_vsbstar_dn6_slot;
        let mut var_vsbstar_dn7: f64 = *var_vsbstar_dn7_slot;
        let mut var_vsbstar_dn8: f64 = *var_vsbstar_dn8_slot;
        let mut var_vsbstar_rv: f64 = *var_vsbstar_rv_slot;
        let mut var_vsbx: f64 = *var_vsbx_slot;
        let mut var_vsbx_dn5: f64 = *var_vsbx_dn5_slot;
        let mut var_vsbx_dn6: f64 = *var_vsbx_dn6_slot;
        let mut var_vsbx_dn7: f64 = *var_vsbx_dn7_slot;
        let mut var_vsbx_dn8: f64 = *var_vsbx_dn8_slot;
        let mut var_vsbx_rv: f64 = *var_vsbx_rv_slot;
        let mut var_xbct: f64 = *var_xbct_slot;
        let mut var_xbct_rv: f64 = *var_xbct_rv_slot;
        let mut var_xctmax: f64 = *var_xctmax_slot;
        let mut var_xctmax_rv: f64 = *var_xctmax_rv_slot;
        let mut var_xgb_ov: f64 = *var_xgb_ov_slot;
        let mut var_xgb_ov_dn5: f64 = *var_xgb_ov_dn5_slot;
        let mut var_xgb_ov_dn6: f64 = *var_xgb_ov_dn6_slot;
        let mut var_xgb_ov_dn7: f64 = *var_xgb_ov_dn7_slot;
        let mut var_xgb_ov_dn8: f64 = *var_xgb_ov_dn8_slot;
        let mut var_xgb_ov_rv: f64 = *var_xgb_ov_rv_slot;
        let mut var_xgct: f64 = *var_xgct_slot;
        let mut var_xgct_dn5: f64 = *var_xgct_dn5_slot;
        let mut var_xgct_dn6: f64 = *var_xgct_dn6_slot;
        let mut var_xgct_dn7: f64 = *var_xgct_dn7_slot;
        let mut var_xgct_dn8: f64 = *var_xgct_dn8_slot;
        let mut var_xgct_rv: f64 = *var_xgct_rv_slot;
        let mut var_xmict: f64 = *var_xmict_slot;
        let mut var_xmict_dn5: f64 = *var_xmict_dn5_slot;
        let mut var_xmict_dn6: f64 = *var_xmict_dn6_slot;
        let mut var_xmict_dn7: f64 = *var_xmict_dn7_slot;
        let mut var_xmict_dn8: f64 = *var_xmict_dn8_slot;
        let mut var_xmict_rv: f64 = *var_xmict_rv_slot;
        let mut var_xnct: f64 = *var_xnct_slot;
        let mut var_xnct_dn5: f64 = *var_xnct_dn5_slot;
        let mut var_xnct_dn6: f64 = *var_xnct_dn6_slot;
        let mut var_xnct_dn7: f64 = *var_xnct_dn7_slot;
        let mut var_xnct_dn8: f64 = *var_xnct_dn8_slot;
        let mut var_xnct_rv: f64 = *var_xnct_rv_slot;
        let mut var_xsbstar: f64 = *var_xsbstar_slot;
        let mut var_xsbstar_dn5: f64 = *var_xsbstar_dn5_slot;
        let mut var_xsbstar_dn6: f64 = *var_xsbstar_dn6_slot;
        let mut var_xsbstar_dn7: f64 = *var_xsbstar_dn7_slot;
        let mut var_xsbstar_dn8: f64 = *var_xsbstar_dn8_slot;
        let mut var_xsbstar_rv: f64 = *var_xsbstar_rv_slot;
        let mut var_xwict: f64 = *var_xwict_slot;
        let mut var_xwict_dn5: f64 = *var_xwict_dn5_slot;
        let mut var_xwict_dn6: f64 = *var_xwict_dn6_slot;
        let mut var_xwict_dn7: f64 = *var_xwict_dn7_slot;
        let mut var_xwict_dn8: f64 = *var_xwict_dn8_slot;
        let mut var_xwict_rv: f64 = *var_xwict_rv_slot;

        let assign40500_e53528: f64 = (var_vgb - var_vfb_t);
        let assign40500_e53529: f64 = (-assign40500_e53528);
        let assign40500_e53531: f64 = (assign40500_e53529 * var_inv_phita);
        var_xgb_ov = assign40500_e53531;
        var_xgb_ov_dn5 = ((-var_vgb_dn5) * var_inv_phita);
        var_xgb_ov_dn6 = ((-var_vgb_dn6) * var_inv_phita);
        var_xgb_ov_dn7 = ((-var_vgb_dn7) * var_inv_phita);
        var_xgb_ov_dn8 = ((-var_vgb_dn8) * var_inv_phita);
        var_xgb_ov_rv = 0.0;

        var_sigvds = 1.0;
        var_sigvds_rv = 0.0;

        let assign40520_e53535: f64 = if var_v_ds < 0.0 { 1.0 } else { 0.0 };
        var_guard1012 = assign40520_e53535;
        var_guard1012_rv = 0.0;

        let (assign40530_e53540,) = {
    if (var_guard1012 != 0.0) {
        let assign40530_e53538: f64 = (-1.0);
        (assign40530_e53538,)
    } else {
        (var_sigvds,)
    }
};
        var_sigvds = assign40530_e53540;
        var_sigvds_rv = 0.0;

        let (assign40540_e53546, assign40540_e53546_d_n5, assign40540_e53546_d_n6, assign40540_e53546_d_n7,) = {
    if (var_guard1012 != 0.0) {
        let assign40540_e53544: f64 = (var_v_gs - var_v_ds);
        (assign40540_e53544, var_v_gs_dn5, (var_v_gs_dn6 - var_v_ds_dn6), (var_v_gs_dn7 - var_v_ds_dn7),)
    } else {
        (var_v_gs, var_v_gs_dn5, var_v_gs_dn6, var_v_gs_dn7,)
    }
};
        var_v_gs = assign40540_e53546;
        var_v_gs_dn5 = assign40540_e53546_d_n5;
        var_v_gs_dn6 = assign40540_e53546_d_n6;
        var_v_gs_dn7 = assign40540_e53546_d_n7;
        var_v_gs_rv = 0.0;

        let (assign40550_e53552, assign40550_e53552_d_n6, assign40550_e53552_d_n7, assign40550_e53552_d_n8,) = {
    if (var_guard1012 != 0.0) {
        let assign40550_e53550: f64 = (var_v_sb + var_v_ds);
        (assign40550_e53550, (var_v_sb_dn6 + var_v_ds_dn6), (var_v_sb_dn7 + var_v_ds_dn7), var_v_sb_dn8,)
    } else {
        (var_v_sb, var_v_sb_dn6, var_v_sb_dn7, var_v_sb_dn8,)
    }
};
        var_v_sb = assign40550_e53552;
        var_v_sb_dn6 = assign40550_e53552_d_n6;
        var_v_sb_dn7 = assign40550_e53552_d_n7;
        var_v_sb_dn8 = assign40550_e53552_d_n8;
        var_v_sb_rv = 0.0;

        let (assign40560_e53557, assign40560_e53557_d_n6, assign40560_e53557_d_n7,) = {
    if (var_guard1012 != 0.0) {
        let assign40560_e53555: f64 = (-var_v_ds);
        (assign40560_e53555, (-var_v_ds_dn6), (-var_v_ds_dn7),)
    } else {
        (var_v_ds, var_v_ds_dn6, var_v_ds_dn7,)
    }
};
        var_v_ds = assign40560_e53557;
        var_v_ds_dn6 = assign40560_e53557_d_n6;
        var_v_ds_dn7 = assign40560_e53557_d_n7;
        var_v_ds_rv = 0.0;

        let assign40570_e53560: f64 = (var_v_ds + var_v_sb);
        var_v_db = assign40570_e53560;
        var_v_db_dn6 = (var_v_ds_dn6 + var_v_sb_dn6);
        var_v_db_dn7 = (var_v_ds_dn7 + var_v_sb_dn7);
        var_v_db_dn8 = var_v_sb_dn8;
        var_v_db_rv = 0.0;

        let assign40580_e53563: f64 = (var_v_ds * var_v_ds);
        let assign40580_e53566: f64 = (var_v_ds * var_v_ds);
        let assign40580_e53568: f64 = (assign40580_e53566 + 0.01);
        let assign40580_e53569: f64 = (assign40580_e53568).sqrt();
        let assign40580_e53571: f64 = (assign40580_e53569 + 0.1);
        let assign40580_e53572: f64 = (assign40580_e53563 / assign40580_e53571);
        var_vdsx = assign40580_e53572;
        var_vdsx_dn6 = (((((var_v_ds_dn6 * var_v_ds) + (var_v_ds * var_v_ds_dn6)) * assign40580_e53571) - (assign40580_e53563 * (((var_v_ds_dn6 * var_v_ds) + (var_v_ds * var_v_ds_dn6)) / (2.0 * assign40580_e53569)))) / (assign40580_e53571 * assign40580_e53571));
        var_vdsx_dn7 = (((((var_v_ds_dn7 * var_v_ds) + (var_v_ds * var_v_ds_dn7)) * assign40580_e53571) - (assign40580_e53563 * (((var_v_ds_dn7 * var_v_ds) + (var_v_ds * var_v_ds_dn7)) / (2.0 * assign40580_e53569)))) / (assign40580_e53571 * assign40580_e53571));
        var_vdsx_rv = 0.0;

        let assign40590_e53576: f64 = (var_v_db + var_v_sb);
        let assign40590_e53579: f64 = (var_v_db - var_v_sb);
        let assign40590_e53582: f64 = (var_v_db - var_v_sb);
        let assign40590_e53583: f64 = (assign40590_e53579 * assign40590_e53582);
        let assign40590_e53585: f64 = (assign40590_e53583 + var_bphi_dc);
        let assign40590_e53586: f64 = (assign40590_e53585).sqrt();
        let assign40590_e53587: f64 = (assign40590_e53576 - assign40590_e53586);
        let assign40590_e53588: f64 = (0.5 * assign40590_e53587);
        let assign40590_e53590: f64 = (assign40590_e53588 + var_phix_dc);
        var_v_xb = assign40590_e53590;
        var_v_xb_dn6 = (0.5 * ((var_v_db_dn6 + var_v_sb_dn6) - ((((var_v_db_dn6 - var_v_sb_dn6) * assign40590_e53582) + (assign40590_e53579 * (var_v_db_dn6 - var_v_sb_dn6))) / (2.0 * assign40590_e53586))));
        var_v_xb_dn7 = (0.5 * ((var_v_db_dn7 + var_v_sb_dn7) - ((((var_v_db_dn7 - var_v_sb_dn7) * assign40590_e53582) + (assign40590_e53579 * (var_v_db_dn7 - var_v_sb_dn7))) / (2.0 * assign40590_e53586))));
        var_v_xb_dn8 = (0.5 * ((var_v_db_dn8 + var_v_sb_dn8) - ((((var_v_db_dn8 - var_v_sb_dn8) * assign40590_e53582) + (assign40590_e53579 * (var_v_db_dn8 - var_v_sb_dn8))) / (2.0 * assign40590_e53586))));
        var_v_xb_rv = 0.0;

        var_v_xb_dc_tmp = var_v_xb;
        var_v_xb_dc_tmp_dn6 = var_v_xb_dn6;
        var_v_xb_dc_tmp_dn7 = var_v_xb_dn7;
        var_v_xb_dc_tmp_dn8 = var_v_xb_dn8;
        var_v_xb_dc_tmp_rv = 0.0;

        let assign40610_e53596: f64 = var_v_xb;
        let assign40610_e53599: f64 = var_v_xb;
        let assign40610_e53602: f64 = var_v_xb;
        let assign40610_e53603: f64 = (assign40610_e53599 * assign40610_e53602);
        let assign40610_e53605: f64 = (assign40610_e53603 + var_aphi_dc);
        let assign40610_e53606: f64 = (assign40610_e53605).sqrt();
        let assign40610_e53607: f64 = (assign40610_e53596 - assign40610_e53606);
        let assign40610_e53608: f64 = (0.5 * assign40610_e53607);
        let assign40610_e53609: f64 = (var_v_sb - assign40610_e53608);
        let assign40610_e53611: f64 = (assign40610_e53609 + var_phix1_dc);
        var_vsbstar_dc = assign40610_e53611;
        var_vsbstar_dc_dn5 = 0.0;
        var_vsbstar_dc_dn6 = (var_v_sb_dn6 - (0.5 * (var_v_xb_dn6 - (((var_v_xb_dn6 * assign40610_e53602) + (assign40610_e53599 * var_v_xb_dn6)) / (2.0 * assign40610_e53606)))));
        var_vsbstar_dc_dn7 = (var_v_sb_dn7 - (0.5 * (var_v_xb_dn7 - (((var_v_xb_dn7 * assign40610_e53602) + (assign40610_e53599 * var_v_xb_dn7)) / (2.0 * assign40610_e53606)))));
        var_vsbstar_dc_dn8 = (var_v_sb_dn8 - (0.5 * (var_v_xb_dn8 - (((var_v_xb_dn8 * assign40610_e53602) + (assign40610_e53599 * var_v_xb_dn8)) / (2.0 * assign40610_e53606)))));
        var_vsbstar_dc_rv = 0.0;

        var_vsbstar_dc_tmp = var_vsbstar_dc;
        var_vsbstar_dc_tmp_dn5 = var_vsbstar_dc_dn5;
        var_vsbstar_dc_tmp_dn6 = var_vsbstar_dc_dn6;
        var_vsbstar_dc_tmp_dn7 = var_vsbstar_dc_dn7;
        var_vsbstar_dc_tmp_dn8 = var_vsbstar_dc_dn8;
        var_vsbstar_dc_tmp_rv = 0.0;

        var_dvbstar_dc = 0.0;
        var_dvbstar_dc_dn5 = 0.0;
        var_dvbstar_dc_dn6 = 0.0;
        var_dvbstar_dc_dn7 = 0.0;
        var_dvbstar_dc_dn8 = 0.0;
        var_dvbstar_dc_rv = 0.0;

        let assign40640_e53620: f64 = if ((p.p45 != 0.0) && (var_gfacnud_i != 1.0)) { 1.0 } else { 0.0 };
        var_guard1172 = assign40640_e53620;
        var_guard1172_rv = 0.0;

        let (assign40650_e53630, assign40650_e53630_d_n5, assign40650_e53630_d_n6, assign40650_e53630_d_n7, assign40650_e53630_d_n8,) = {
    if (var_guard1172 != 0.0) {
        let assign40650_e53626: f64 = (var_v_ds - var_vdsx);
        let assign40650_e53627: f64 = (0.5 * assign40650_e53626);
        let assign40650_e53628: f64 = (var_vsbstar_dc + assign40650_e53627);
        (assign40650_e53628, var_vsbstar_dc_dn5, (var_vsbstar_dc_dn6 + (0.5 * (var_v_ds_dn6 - var_vdsx_dn6))), (var_vsbstar_dc_dn7 + (0.5 * (var_v_ds_dn7 - var_vdsx_dn7))), var_vsbstar_dc_dn8,)
    } else {
        (var_vmb, var_vmb_dn5, var_vmb_dn6, var_vmb_dn7, var_vmb_dn8,)
    }
};
        var_vmb = assign40650_e53630;
        var_vmb_dn5 = assign40650_e53630_d_n5;
        var_vmb_dn6 = assign40650_e53630_d_n6;
        var_vmb_dn7 = assign40650_e53630_d_n7;
        var_vmb_dn8 = assign40650_e53630_d_n8;
        var_vmb_rv = 0.0;

        let (assign40660_e53639, assign40660_e53639_d_n5, assign40660_e53639_d_n6, assign40660_e53639_d_n7, assign40660_e53639_d_n8,) = {
    if (var_guard1172 != 0.0) {
        let assign40660_e53634: f64 = (var_vmb + var_phib_dc);
        let assign40660_e53635: f64 = (assign40660_e53634).sqrt();
        let assign40660_e53637: f64 = (assign40660_e53635 - var_sqrt_phib_dc);
        (assign40660_e53637, (var_vmb_dn5 / (2.0 * assign40660_e53635)), (var_vmb_dn6 / (2.0 * assign40660_e53635)), (var_vmb_dn7 / (2.0 * assign40660_e53635)), (var_vmb_dn8 / (2.0 * assign40660_e53635)),)
    } else {
        (var_us, var_us_dn5, var_us_dn6, var_us_dn7, var_us_dn8,)
    }
};
        var_us = assign40660_e53639;
        var_us_dn5 = assign40660_e53639_d_n5;
        var_us_dn6 = assign40660_e53639_d_n6;
        var_us_dn7 = assign40660_e53639_d_n7;
        var_us_dn8 = assign40660_e53639_d_n8;
        var_us_rv = 0.0;

        let (assign40670_e53651, assign40670_e53651_d_n5, assign40670_e53651_d_n6, assign40670_e53651_d_n7, assign40670_e53651_d_n8,) = {
    if (var_guard1172 != 0.0) {
        let assign40670_e53644: f64 = (var_us - var_us1);
        let assign40670_e53645: f64 = (2.0 * assign40670_e53644);
        let assign40670_e53647: f64 = (assign40670_e53645 / var_us21);
        let assign40670_e53649: f64 = (assign40670_e53647 - 1.0);
        (assign40670_e53649, ((2.0 * var_us_dn5) / var_us21), ((2.0 * var_us_dn6) / var_us21), ((2.0 * var_us_dn7) / var_us21), ((2.0 * var_us_dn8) / var_us21),)
    } else {
        (var_temp__blk936, var_temp__blk936_dn5, var_temp__blk936_dn6, var_temp__blk936_dn7, var_temp__blk936_dn8,)
    }
};
        var_temp__blk936 = assign40670_e53651;
        var_temp__blk936_dn5 = assign40670_e53651_d_n5;
        var_temp__blk936_dn6 = assign40670_e53651_d_n6;
        var_temp__blk936_dn7 = assign40670_e53651_d_n7;
        var_temp__blk936_dn8 = assign40670_e53651_d_n8;
        var_temp__blk936_rv = 0.0;

        let (assign40680_e53672, assign40680_e53672_d_n5, assign40680_e53672_d_n6, assign40680_e53672_d_n7, assign40680_e53672_d_n8,) = {
    if (var_guard1172 != 0.0) {
        let assign40680_e53657: f64 = (1.0 - var_gfacnud_i);
        let assign40680_e53658: f64 = (0.25 * assign40680_e53657);
        let assign40680_e53660: f64 = (assign40680_e53658 * var_us21);
        let assign40680_e53664: f64 = (var_temp__blk936 * var_temp__blk936);
        let assign40680_e53666: f64 = (assign40680_e53664 + 0.4804530139182);
        let assign40680_e53667: f64 = (assign40680_e53666).sqrt();
        let assign40680_e53668: f64 = (var_temp__blk936 + assign40680_e53667);
        let assign40680_e53669: f64 = (assign40680_e53660 * assign40680_e53668);
        let assign40680_e53670: f64 = (var_us - assign40680_e53669);
        (assign40680_e53670, (var_us_dn5 - (assign40680_e53660 * (var_temp__blk936_dn5 + (((var_temp__blk936_dn5 * var_temp__blk936) + (var_temp__blk936 * var_temp__blk936_dn5)) / (2.0 * assign40680_e53667))))), (var_us_dn6 - (assign40680_e53660 * (var_temp__blk936_dn6 + (((var_temp__blk936_dn6 * var_temp__blk936) + (var_temp__blk936 * var_temp__blk936_dn6)) / (2.0 * assign40680_e53667))))), (var_us_dn7 - (assign40680_e53660 * (var_temp__blk936_dn7 + (((var_temp__blk936_dn7 * var_temp__blk936) + (var_temp__blk936 * var_temp__blk936_dn7)) / (2.0 * assign40680_e53667))))), (var_us_dn8 - (assign40680_e53660 * (var_temp__blk936_dn8 + (((var_temp__blk936_dn8 * var_temp__blk936) + (var_temp__blk936 * var_temp__blk936_dn8)) / (2.0 * assign40680_e53667))))),)
    } else {
        (var_usnew, var_usnew_dn5, var_usnew_dn6, var_usnew_dn7, var_usnew_dn8,)
    }
};
        var_usnew = assign40680_e53672;
        var_usnew_dn5 = assign40680_e53672_d_n5;
        var_usnew_dn6 = assign40680_e53672_d_n6;
        var_usnew_dn7 = assign40680_e53672_d_n7;
        var_usnew_dn8 = assign40680_e53672_d_n8;
        var_usnew_rv = 0.0;

        let (assign40690_e53684, assign40690_e53684_d_n5, assign40690_e53684_d_n6, assign40690_e53684_d_n7, assign40690_e53684_d_n8,) = {
    if (var_guard1172 != 0.0) {
        let assign40690_e53676: f64 = (var_usnew * var_usnew);
        let assign40690_e53679: f64 = (2.0 * var_sqrt_phib_dc);
        let assign40690_e53681: f64 = (assign40690_e53679 * var_usnew);
        let assign40690_e53682: f64 = (assign40690_e53676 + assign40690_e53681);
        (assign40690_e53682, (((var_usnew_dn5 * var_usnew) + (var_usnew * var_usnew_dn5)) + (assign40690_e53679 * var_usnew_dn5)), (((var_usnew_dn6 * var_usnew) + (var_usnew * var_usnew_dn6)) + (assign40690_e53679 * var_usnew_dn6)), (((var_usnew_dn7 * var_usnew) + (var_usnew * var_usnew_dn7)) + (assign40690_e53679 * var_usnew_dn7)), (((var_usnew_dn8 * var_usnew) + (var_usnew * var_usnew_dn8)) + (assign40690_e53679 * var_usnew_dn8)),)
    } else {
        (var_vmbnew, var_vmbnew_dn5, var_vmbnew_dn6, var_vmbnew_dn7, var_vmbnew_dn8,)
    }
};
        var_vmbnew = assign40690_e53684;
        var_vmbnew_dn5 = assign40690_e53684_d_n5;
        var_vmbnew_dn6 = assign40690_e53684_d_n6;
        var_vmbnew_dn7 = assign40690_e53684_d_n7;
        var_vmbnew_dn8 = assign40690_e53684_d_n8;
        var_vmbnew_rv = 0.0;

        let (assign40700_e53694, assign40700_e53694_d_n5, assign40700_e53694_d_n6, assign40700_e53694_d_n7, assign40700_e53694_d_n8,) = {
    if (var_guard1172 != 0.0) {
        let assign40700_e53690: f64 = (var_v_ds - var_vdsx);
        let assign40700_e53691: f64 = (0.5 * assign40700_e53690);
        let assign40700_e53692: f64 = (var_vmbnew - assign40700_e53691);
        (assign40700_e53692, var_vmbnew_dn5, (var_vmbnew_dn6 - (0.5 * (var_v_ds_dn6 - var_vdsx_dn6))), (var_vmbnew_dn7 - (0.5 * (var_v_ds_dn7 - var_vdsx_dn7))), var_vmbnew_dn8,)
    } else {
        (var_vsbstar_dc, var_vsbstar_dc_dn5, var_vsbstar_dc_dn6, var_vsbstar_dc_dn7, var_vsbstar_dc_dn8,)
    }
};
        var_vsbstar_dc = assign40700_e53694;
        var_vsbstar_dc_dn5 = assign40700_e53694_d_n5;
        var_vsbstar_dc_dn6 = assign40700_e53694_d_n6;
        var_vsbstar_dc_dn7 = assign40700_e53694_d_n7;
        var_vsbstar_dc_dn8 = assign40700_e53694_d_n8;
        var_vsbstar_dc_rv = 0.0;

        let (assign40710_e53700, assign40710_e53700_d_n5, assign40710_e53700_d_n6, assign40710_e53700_d_n7, assign40710_e53700_d_n8,) = {
    if (var_guard1172 != 0.0) {
        let assign40710_e53698: f64 = (var_vsbstar_dc_tmp - var_vsbstar_dc);
        (assign40710_e53698, (var_vsbstar_dc_tmp_dn5 - var_vsbstar_dc_dn5), (var_vsbstar_dc_tmp_dn6 - var_vsbstar_dc_dn6), (var_vsbstar_dc_tmp_dn7 - var_vsbstar_dc_dn7), (var_vsbstar_dc_tmp_dn8 - var_vsbstar_dc_dn8),)
    } else {
        (var_dvbstar_dc, var_dvbstar_dc_dn5, var_dvbstar_dc_dn6, var_dvbstar_dc_dn7, var_dvbstar_dc_dn8,)
    }
};
        var_dvbstar_dc = assign40710_e53700;
        var_dvbstar_dc_dn5 = assign40710_e53700_d_n5;
        var_dvbstar_dc_dn6 = assign40710_e53700_d_n6;
        var_dvbstar_dc_dn7 = assign40710_e53700_d_n7;
        var_dvbstar_dc_dn8 = assign40710_e53700_d_n8;
        var_dvbstar_dc_rv = 0.0;

        var_phib = var_phib_dc;
        var_phib_rv = 0.0;

        var_aphi = var_aphi_dc;
        var_aphi_rv = 0.0;

        var_g_0 = var_g_0_dc;
        var_g_0_rv = 0.0;

        var_vsbstar = var_vsbstar_dc;
        var_vsbstar_dn5 = var_vsbstar_dc_dn5;
        var_vsbstar_dn6 = var_vsbstar_dc_dn6;
        var_vsbstar_dn7 = var_vsbstar_dc_dn7;
        var_vsbstar_dn8 = var_vsbstar_dc_dn8;
        var_vsbstar_rv = 0.0;

        var_dvbstar = var_dvbstar_dc;
        var_dvbstar_dn5 = var_dvbstar_dc_dn5;
        var_dvbstar_dn6 = var_dvbstar_dc_dn6;
        var_dvbstar_dn7 = var_dvbstar_dc_dn7;
        var_dvbstar_dn8 = var_dvbstar_dc_dn8;
        var_dvbstar_rv = 0.0;

        var_thesatloc = var_thesat_t;
        var_thesatloc_rv = 0.0;

        var_arloc = var_ar;
        var_arloc_rv = 0.0;

        let assign40790_e53710: f64 = (var_vgb - var_dvbstar);
        let assign40790_e53712: f64 = (assign40790_e53710 - var_vfb_t);
        var_vgb1 = assign40790_e53712;
        var_vgb1_dn5 = (var_vgb_dn5 - var_dvbstar_dn5);
        var_vgb1_dn6 = (var_vgb_dn6 - var_dvbstar_dn6);
        var_vgb1_dn7 = (var_vgb_dn7 - var_dvbstar_dn7);
        var_vgb1_dn8 = (var_vgb_dn8 - var_dvbstar_dn8);
        var_vgb1_rv = 0.0;

        let assign40800_e53717: f64 = (var_v_ds - var_vdsx);
        let assign40800_e53718: f64 = (0.5 * assign40800_e53717);
        let assign40800_e53719: f64 = (var_vsbstar + assign40800_e53718);
        var_vsbx = assign40800_e53719;
        var_vsbx_dn5 = var_vsbstar_dn5;
        var_vsbx_dn6 = (var_vsbstar_dn6 + (0.5 * (var_v_ds_dn6 - var_vdsx_dn6)));
        var_vsbx_dn7 = (var_vsbstar_dn7 + (0.5 * (var_v_ds_dn7 - var_vdsx_dn7)));
        var_vsbx_dn8 = var_vsbstar_dn8;
        var_vsbx_rv = 0.0;

        var_dctg = 1.0;
        var_dctg_dn5 = 0.0;
        var_dctg_dn6 = 0.0;
        var_dctg_dn7 = 0.0;
        var_dctg_dn8 = 0.0;
        var_dctg_rv = 0.0;

        let assign40820_e53723: f64 = if var_ctg_i > 0.0 { 1.0 } else { 0.0 };
        var_guard1173 = assign40820_e53723;
        var_guard1173_rv = 0.0;

        let (assign40830_e53729,) = {
    if (var_guard1173 != 0.0) {
        let assign40830_e53727: f64 = (var_phib * var_inv_phit);
        (assign40830_e53727,)
    } else {
        (var_xbct,)
    }
};
        var_xbct = assign40830_e53729;
        var_xbct_rv = 0.0;

        let (assign40840_e53735, assign40840_e53735_d_n5, assign40840_e53735_d_n6, assign40840_e53735_d_n7, assign40840_e53735_d_n8,) = {
    if (var_guard1173 != 0.0) {
        let assign40840_e53733: f64 = (var_vsbx * var_inv_phit);
        (assign40840_e53733, (var_vsbx_dn5 * var_inv_phit), (var_vsbx_dn6 * var_inv_phit), (var_vsbx_dn7 * var_inv_phit), (var_vsbx_dn8 * var_inv_phit),)
    } else {
        (var_xsbstar, var_xsbstar_dn5, var_xsbstar_dn6, var_xsbstar_dn7, var_xsbstar_dn8,)
    }
};
        var_xsbstar = assign40840_e53735;
        var_xsbstar_dn5 = assign40840_e53735_d_n5;
        var_xsbstar_dn6 = assign40840_e53735_d_n6;
        var_xsbstar_dn7 = assign40840_e53735_d_n7;
        var_xsbstar_dn8 = assign40840_e53735_d_n8;
        var_xsbstar_rv = 0.0;

        let (assign40850_e53741, assign40850_e53741_d_n5, assign40850_e53741_d_n6, assign40850_e53741_d_n7, assign40850_e53741_d_n8,) = {
    if (var_guard1173 != 0.0) {
        let assign40850_e53739: f64 = (var_vgb1 * var_inv_phit);
        (assign40850_e53739, (var_vgb1_dn5 * var_inv_phit), (var_vgb1_dn6 * var_inv_phit), (var_vgb1_dn7 * var_inv_phit), (var_vgb1_dn8 * var_inv_phit),)
    } else {
        (var_xgct, var_xgct_dn5, var_xgct_dn6, var_xgct_dn7, var_xgct_dn8,)
    }
};
        var_xgct = assign40850_e53741;
        var_xgct_dn5 = assign40850_e53741_d_n5;
        var_xgct_dn6 = assign40850_e53741_d_n6;
        var_xgct_dn7 = assign40850_e53741_d_n7;
        var_xgct_dn8 = assign40850_e53741_d_n8;
        var_xgct_rv = 0.0;

        let (assign40860_e53752, assign40860_e53752_d_n5, assign40860_e53752_d_n6, assign40860_e53752_d_n7, assign40860_e53752_d_n8,) = {
    if (var_guard1173 != 0.0) {
        let assign40860_e53746: f64 = (0.5 * var_g_0);
        let assign40860_e53748: f64 = (var_xbct).sqrt();
        let assign40860_e53749: f64 = (assign40860_e53746 / assign40860_e53748);
        let assign40860_e53750: f64 = (1.0 + assign40860_e53749);
        (assign40860_e53750, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_temp1, var_temp1_dn5, var_temp1_dn6, var_temp1_dn7, var_temp1_dn8,)
    }
};
        var_temp1 = assign40860_e53752;
        var_temp1_dn5 = assign40860_e53752_d_n5;
        var_temp1_dn6 = assign40860_e53752_d_n6;
        var_temp1_dn7 = assign40860_e53752_d_n7;
        var_temp1_dn8 = assign40860_e53752_d_n8;
        var_temp1_rv = 0.0;

        let (assign40870_e53761, assign40870_e53761_d_n5, assign40870_e53761_d_n6, assign40870_e53761_d_n7, assign40870_e53761_d_n8,) = {
    if (var_guard1173 != 0.0) {
        let assign40870_e53757: f64 = (var_xbct).sqrt();
        let assign40870_e53758: f64 = (var_g_0 * assign40870_e53757);
        let assign40870_e53759: f64 = (var_xbct + assign40870_e53758);
        (assign40870_e53759, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_temp2, var_temp2_dn5, var_temp2_dn6, var_temp2_dn7, var_temp2_dn8,)
    }
};
        var_temp2 = assign40870_e53761;
        var_temp2_dn5 = assign40870_e53761_d_n5;
        var_temp2_dn6 = assign40870_e53761_d_n6;
        var_temp2_dn7 = assign40870_e53761_d_n7;
        var_temp2_dn8 = assign40870_e53761_d_n8;
        var_temp2_rv = 0.0;

        let (assign40880_e53779, assign40880_e53779_d_n5, assign40880_e53779_d_n6, assign40880_e53779_d_n7, assign40880_e53779_d_n8,) = {
    if (var_guard1173 != 0.0) {
        let assign40880_e53765: f64 = (var_xgct - var_temp2);
        let assign40880_e53767: f64 = (assign40880_e53765 / var_temp1);
        let assign40880_e53770: f64 = (0.5 * var_xbct);
        let assign40880_e53771: f64 = (assign40880_e53767 + assign40880_e53770);
        let assign40880_e53774: f64 = (1.0 + var_ctb_i);
        let assign40880_e53776: f64 = (assign40880_e53774 * var_xsbstar);
        let assign40880_e53777: f64 = (assign40880_e53771 - assign40880_e53776);
        (assign40880_e53777, (((((var_xgct_dn5 - var_temp2_dn5) * var_temp1) - (assign40880_e53765 * var_temp1_dn5)) / (var_temp1 * var_temp1)) - (assign40880_e53774 * var_xsbstar_dn5)), (((((var_xgct_dn6 - var_temp2_dn6) * var_temp1) - (assign40880_e53765 * var_temp1_dn6)) / (var_temp1 * var_temp1)) - (assign40880_e53774 * var_xsbstar_dn6)), (((((var_xgct_dn7 - var_temp2_dn7) * var_temp1) - (assign40880_e53765 * var_temp1_dn7)) / (var_temp1 * var_temp1)) - (assign40880_e53774 * var_xsbstar_dn7)), (((((var_xgct_dn8 - var_temp2_dn8) * var_temp1) - (assign40880_e53765 * var_temp1_dn8)) / (var_temp1 * var_temp1)) - (assign40880_e53774 * var_xsbstar_dn8)),)
    } else {
        (var_xwict, var_xwict_dn5, var_xwict_dn6, var_xwict_dn7, var_xwict_dn8,)
    }
};
        var_xwict = assign40880_e53779;
        var_xwict_dn5 = assign40880_e53779_d_n5;
        var_xwict_dn6 = assign40880_e53779_d_n6;
        var_xwict_dn7 = assign40880_e53779_d_n7;
        var_xwict_dn8 = assign40880_e53779_d_n8;
        var_xwict_rv = 0.0;

        let (assign40890_e53787,) = {
    if (var_guard1173 != 0.0) {
        let assign40890_e53783: f64 = (0.5 * var_xbct);
        let assign40890_e53785: f64 = (assign40890_e53783 + 2.0);
        (assign40890_e53785,)
    } else {
        (var_xctmax,)
    }
};
        var_xctmax = assign40890_e53787;
        var_xctmax_rv = 0.0;

        let (assign40900_e53793, assign40900_e53793_d_n5, assign40900_e53793_d_n6, assign40900_e53793_d_n7, assign40900_e53793_d_n8,) = {
    if (var_guard1173 != 0.0) {
        let assign40900_e53791: f64 = (var_xbct + var_xsbstar);
        (assign40900_e53791, var_xsbstar_dn5, var_xsbstar_dn6, var_xsbstar_dn7, var_xsbstar_dn8,)
    } else {
        (var_xnct, var_xnct_dn5, var_xnct_dn6, var_xnct_dn7, var_xnct_dn8,)
    }
};
        var_xnct = assign40900_e53793;
        var_xnct_dn5 = assign40900_e53793_d_n5;
        var_xnct_dn6 = assign40900_e53793_d_n6;
        var_xnct_dn7 = assign40900_e53793_d_n7;
        var_xnct_dn8 = assign40900_e53793_d_n8;
        var_xnct_rv = 0.0;

        let (assign40910_e53814, assign40910_e53814_d_n5, assign40910_e53814_d_n6, assign40910_e53814_d_n7, assign40910_e53814_d_n8,) = {
    if (var_guard1173 != 0.0) {
        let assign40910_e53797: f64 = (var_xgct - var_xnct);
        let assign40910_e53800: f64 = (var_xnct).sqrt();
        let assign40910_e53801: f64 = (var_g_0 * assign40910_e53800);
        let assign40910_e53802: f64 = (assign40910_e53797 - assign40910_e53801);
        let assign40910_e53806: f64 = (var_xbct / var_g_0);
        let assign40910_e53808: f64 = (var_xbct).sqrt();
        let assign40910_e53809: f64 = (assign40910_e53806 + assign40910_e53808);
        let assign40910_e53810: f64 = (assign40910_e53809).ln();
        let assign40910_e53811: f64 = (2.0 * assign40910_e53810);
        let assign40910_e53812: f64 = (assign40910_e53802 - assign40910_e53811);
        (assign40910_e53812, ((var_xgct_dn5 - var_xnct_dn5) - (var_g_0 * (var_xnct_dn5 / (2.0 * assign40910_e53800)))), ((var_xgct_dn6 - var_xnct_dn6) - (var_g_0 * (var_xnct_dn6 / (2.0 * assign40910_e53800)))), ((var_xgct_dn7 - var_xnct_dn7) - (var_g_0 * (var_xnct_dn7 / (2.0 * assign40910_e53800)))), ((var_xgct_dn8 - var_xnct_dn8) - (var_g_0 * (var_xnct_dn8 / (2.0 * assign40910_e53800)))),)
    } else {
        (var_temp1, var_temp1_dn5, var_temp1_dn6, var_temp1_dn7, var_temp1_dn8,)
    }
};
        var_temp1 = assign40910_e53814;
        var_temp1_dn5 = assign40910_e53814_d_n5;
        var_temp1_dn6 = assign40910_e53814_d_n6;
        var_temp1_dn7 = assign40910_e53814_d_n7;
        var_temp1_dn8 = assign40910_e53814_d_n8;
        var_temp1_rv = 0.0;

        let (assign40920_e53822, assign40920_e53822_d_n5, assign40920_e53822_d_n6, assign40920_e53822_d_n7, assign40920_e53822_d_n8,) = {
    if (var_guard1173 != 0.0) {
        let assign40920_e53818: f64 = (2.0 * var_temp1);
        let assign40920_e53820: f64 = (assign40920_e53818 + var_xctmax);
        (assign40920_e53820, (2.0 * var_temp1_dn5), (2.0 * var_temp1_dn6), (2.0 * var_temp1_dn7), (2.0 * var_temp1_dn8),)
    } else {
        (var_xmict, var_xmict_dn5, var_xmict_dn6, var_xmict_dn7, var_xmict_dn8,)
    }
};
        var_xmict = assign40920_e53822;
        var_xmict_dn5 = assign40920_e53822_d_n5;
        var_xmict_dn6 = assign40920_e53822_d_n6;
        var_xmict_dn7 = assign40920_e53822_d_n7;
        var_xmict_dn8 = assign40920_e53822_d_n8;
        var_xmict_rv = 0.0;

        *var_aphi_slot = var_aphi;
        *var_aphi_rv_slot = var_aphi_rv;
        *var_arloc_slot = var_arloc;
        *var_arloc_rv_slot = var_arloc_rv;
        *var_dctg_slot = var_dctg;
        *var_dctg_dn5_slot = var_dctg_dn5;
        *var_dctg_dn6_slot = var_dctg_dn6;
        *var_dctg_dn7_slot = var_dctg_dn7;
        *var_dctg_dn8_slot = var_dctg_dn8;
        *var_dctg_rv_slot = var_dctg_rv;
        *var_dvbstar_slot = var_dvbstar;
        *var_dvbstar_dc_slot = var_dvbstar_dc;
        *var_dvbstar_dc_dn5_slot = var_dvbstar_dc_dn5;
        *var_dvbstar_dc_dn6_slot = var_dvbstar_dc_dn6;
        *var_dvbstar_dc_dn7_slot = var_dvbstar_dc_dn7;
        *var_dvbstar_dc_dn8_slot = var_dvbstar_dc_dn8;
        *var_dvbstar_dc_rv_slot = var_dvbstar_dc_rv;
        *var_dvbstar_dn5_slot = var_dvbstar_dn5;
        *var_dvbstar_dn6_slot = var_dvbstar_dn6;
        *var_dvbstar_dn7_slot = var_dvbstar_dn7;
        *var_dvbstar_dn8_slot = var_dvbstar_dn8;
        *var_dvbstar_rv_slot = var_dvbstar_rv;
        *var_g_0_slot = var_g_0;
        *var_g_0_rv_slot = var_g_0_rv;
        *var_guard1012_slot = var_guard1012;
        *var_guard1012_rv_slot = var_guard1012_rv;
        *var_guard1172_slot = var_guard1172;
        *var_guard1172_rv_slot = var_guard1172_rv;
        *var_guard1173_slot = var_guard1173;
        *var_guard1173_rv_slot = var_guard1173_rv;
        *var_phib_slot = var_phib;
        *var_phib_rv_slot = var_phib_rv;
        *var_sigvds_slot = var_sigvds;
        *var_sigvds_rv_slot = var_sigvds_rv;
        *var_temp1_slot = var_temp1;
        *var_temp1_dn5_slot = var_temp1_dn5;
        *var_temp1_dn6_slot = var_temp1_dn6;
        *var_temp1_dn7_slot = var_temp1_dn7;
        *var_temp1_dn8_slot = var_temp1_dn8;
        *var_temp1_rv_slot = var_temp1_rv;
        *var_temp2_slot = var_temp2;
        *var_temp2_dn5_slot = var_temp2_dn5;
        *var_temp2_dn6_slot = var_temp2_dn6;
        *var_temp2_dn7_slot = var_temp2_dn7;
        *var_temp2_dn8_slot = var_temp2_dn8;
        *var_temp2_rv_slot = var_temp2_rv;
        *var_temp__blk936_slot = var_temp__blk936;
        *var_temp__blk936_dn5_slot = var_temp__blk936_dn5;
        *var_temp__blk936_dn6_slot = var_temp__blk936_dn6;
        *var_temp__blk936_dn7_slot = var_temp__blk936_dn7;
        *var_temp__blk936_dn8_slot = var_temp__blk936_dn8;
        *var_temp__blk936_rv_slot = var_temp__blk936_rv;
        *var_thesatloc_slot = var_thesatloc;
        *var_thesatloc_rv_slot = var_thesatloc_rv;
        *var_us_slot = var_us;
        *var_us_dn5_slot = var_us_dn5;
        *var_us_dn6_slot = var_us_dn6;
        *var_us_dn7_slot = var_us_dn7;
        *var_us_dn8_slot = var_us_dn8;
        *var_us_rv_slot = var_us_rv;
        *var_usnew_slot = var_usnew;
        *var_usnew_dn5_slot = var_usnew_dn5;
        *var_usnew_dn6_slot = var_usnew_dn6;
        *var_usnew_dn7_slot = var_usnew_dn7;
        *var_usnew_dn8_slot = var_usnew_dn8;
        *var_usnew_rv_slot = var_usnew_rv;
        *var_v_db_slot = var_v_db;
        *var_v_db_dn6_slot = var_v_db_dn6;
        *var_v_db_dn7_slot = var_v_db_dn7;
        *var_v_db_dn8_slot = var_v_db_dn8;
        *var_v_db_rv_slot = var_v_db_rv;
        *var_v_ds_slot = var_v_ds;
        *var_v_ds_dn6_slot = var_v_ds_dn6;
        *var_v_ds_dn7_slot = var_v_ds_dn7;
        *var_v_ds_rv_slot = var_v_ds_rv;
        *var_v_gs_slot = var_v_gs;
        *var_v_gs_dn5_slot = var_v_gs_dn5;
        *var_v_gs_dn6_slot = var_v_gs_dn6;
        *var_v_gs_dn7_slot = var_v_gs_dn7;
        *var_v_gs_rv_slot = var_v_gs_rv;
        *var_v_sb_slot = var_v_sb;
        *var_v_sb_dn6_slot = var_v_sb_dn6;
        *var_v_sb_dn7_slot = var_v_sb_dn7;
        *var_v_sb_dn8_slot = var_v_sb_dn8;
        *var_v_sb_rv_slot = var_v_sb_rv;
        *var_v_xb_slot = var_v_xb;
        *var_v_xb_dc_tmp_slot = var_v_xb_dc_tmp;
        *var_v_xb_dc_tmp_dn6_slot = var_v_xb_dc_tmp_dn6;
        *var_v_xb_dc_tmp_dn7_slot = var_v_xb_dc_tmp_dn7;
        *var_v_xb_dc_tmp_dn8_slot = var_v_xb_dc_tmp_dn8;
        *var_v_xb_dc_tmp_rv_slot = var_v_xb_dc_tmp_rv;
        *var_v_xb_dn6_slot = var_v_xb_dn6;
        *var_v_xb_dn7_slot = var_v_xb_dn7;
        *var_v_xb_dn8_slot = var_v_xb_dn8;
        *var_v_xb_rv_slot = var_v_xb_rv;
        *var_vdsx_slot = var_vdsx;
        *var_vdsx_dn6_slot = var_vdsx_dn6;
        *var_vdsx_dn7_slot = var_vdsx_dn7;
        *var_vdsx_rv_slot = var_vdsx_rv;
        *var_vgb1_slot = var_vgb1;
        *var_vgb1_dn5_slot = var_vgb1_dn5;
        *var_vgb1_dn6_slot = var_vgb1_dn6;
        *var_vgb1_dn7_slot = var_vgb1_dn7;
        *var_vgb1_dn8_slot = var_vgb1_dn8;
        *var_vgb1_rv_slot = var_vgb1_rv;
        *var_vmb_slot = var_vmb;
        *var_vmb_dn5_slot = var_vmb_dn5;
        *var_vmb_dn6_slot = var_vmb_dn6;
        *var_vmb_dn7_slot = var_vmb_dn7;
        *var_vmb_dn8_slot = var_vmb_dn8;
        *var_vmb_rv_slot = var_vmb_rv;
        *var_vmbnew_slot = var_vmbnew;
        *var_vmbnew_dn5_slot = var_vmbnew_dn5;
        *var_vmbnew_dn6_slot = var_vmbnew_dn6;
        *var_vmbnew_dn7_slot = var_vmbnew_dn7;
        *var_vmbnew_dn8_slot = var_vmbnew_dn8;
        *var_vmbnew_rv_slot = var_vmbnew_rv;
        *var_vsbstar_slot = var_vsbstar;
        *var_vsbstar_dc_slot = var_vsbstar_dc;
        *var_vsbstar_dc_dn5_slot = var_vsbstar_dc_dn5;
        *var_vsbstar_dc_dn6_slot = var_vsbstar_dc_dn6;
        *var_vsbstar_dc_dn7_slot = var_vsbstar_dc_dn7;
        *var_vsbstar_dc_dn8_slot = var_vsbstar_dc_dn8;
        *var_vsbstar_dc_rv_slot = var_vsbstar_dc_rv;
        *var_vsbstar_dc_tmp_slot = var_vsbstar_dc_tmp;
        *var_vsbstar_dc_tmp_dn5_slot = var_vsbstar_dc_tmp_dn5;
        *var_vsbstar_dc_tmp_dn6_slot = var_vsbstar_dc_tmp_dn6;
        *var_vsbstar_dc_tmp_dn7_slot = var_vsbstar_dc_tmp_dn7;
        *var_vsbstar_dc_tmp_dn8_slot = var_vsbstar_dc_tmp_dn8;
        *var_vsbstar_dc_tmp_rv_slot = var_vsbstar_dc_tmp_rv;
        *var_vsbstar_dn5_slot = var_vsbstar_dn5;
        *var_vsbstar_dn6_slot = var_vsbstar_dn6;
        *var_vsbstar_dn7_slot = var_vsbstar_dn7;
        *var_vsbstar_dn8_slot = var_vsbstar_dn8;
        *var_vsbstar_rv_slot = var_vsbstar_rv;
        *var_vsbx_slot = var_vsbx;
        *var_vsbx_dn5_slot = var_vsbx_dn5;
        *var_vsbx_dn6_slot = var_vsbx_dn6;
        *var_vsbx_dn7_slot = var_vsbx_dn7;
        *var_vsbx_dn8_slot = var_vsbx_dn8;
        *var_vsbx_rv_slot = var_vsbx_rv;
        *var_xbct_slot = var_xbct;
        *var_xbct_rv_slot = var_xbct_rv;
        *var_xctmax_slot = var_xctmax;
        *var_xctmax_rv_slot = var_xctmax_rv;
        *var_xgb_ov_slot = var_xgb_ov;
        *var_xgb_ov_dn5_slot = var_xgb_ov_dn5;
        *var_xgb_ov_dn6_slot = var_xgb_ov_dn6;
        *var_xgb_ov_dn7_slot = var_xgb_ov_dn7;
        *var_xgb_ov_dn8_slot = var_xgb_ov_dn8;
        *var_xgb_ov_rv_slot = var_xgb_ov_rv;
        *var_xgct_slot = var_xgct;
        *var_xgct_dn5_slot = var_xgct_dn5;
        *var_xgct_dn6_slot = var_xgct_dn6;
        *var_xgct_dn7_slot = var_xgct_dn7;
        *var_xgct_dn8_slot = var_xgct_dn8;
        *var_xgct_rv_slot = var_xgct_rv;
        *var_xmict_slot = var_xmict;
        *var_xmict_dn5_slot = var_xmict_dn5;
        *var_xmict_dn6_slot = var_xmict_dn6;
        *var_xmict_dn7_slot = var_xmict_dn7;
        *var_xmict_dn8_slot = var_xmict_dn8;
        *var_xmict_rv_slot = var_xmict_rv;
        *var_xnct_slot = var_xnct;
        *var_xnct_dn5_slot = var_xnct_dn5;
        *var_xnct_dn6_slot = var_xnct_dn6;
        *var_xnct_dn7_slot = var_xnct_dn7;
        *var_xnct_dn8_slot = var_xnct_dn8;
        *var_xnct_rv_slot = var_xnct_rv;
        *var_xsbstar_slot = var_xsbstar;
        *var_xsbstar_dn5_slot = var_xsbstar_dn5;
        *var_xsbstar_dn6_slot = var_xsbstar_dn6;
        *var_xsbstar_dn7_slot = var_xsbstar_dn7;
        *var_xsbstar_dn8_slot = var_xsbstar_dn8;
        *var_xsbstar_rv_slot = var_xsbstar_rv;
        *var_xwict_slot = var_xwict;
        *var_xwict_dn5_slot = var_xwict_dn5;
        *var_xwict_dn6_slot = var_xwict_dn6;
        *var_xwict_dn7_slot = var_xwict_dn7;
        *var_xwict_dn8_slot = var_xwict_dn8;
        *var_xwict_rv_slot = var_xwict_rv;
    }

    pub(super) fn stamp_reactive_block_24(
        p: &Parameters,
        var_aphi: f64,
        var_cf_i: f64,
        var_cfb_i: f64,
        var_cfd_i: f64,
        var_ct_t: f64,
        var_ctg_t: f64,
        var_g_0: f64,
        var_guard1173: f64,
        var_phib: f64,
        var_phit: f64,
        var_psce_i: f64,
        var_psceb_i: f64,
        var_psced_i: f64,
        var_v_xb: f64,
        var_v_xb_dn6: f64,
        var_v_xb_dn7: f64,
        var_v_xb_dn8: f64,
        var_vdsx: f64,
        var_vdsx_dn6: f64,
        var_vdsx_dn7: f64,
        var_vgb1: f64,
        var_vgb1_dn5: f64,
        var_vgb1_dn6: f64,
        var_vgb1_dn7: f64,
        var_vgb1_dn8: f64,
        var_vsbstar: f64,
        var_vsbstar_dn5: f64,
        var_vsbstar_dn6: f64,
        var_vsbstar_dn7: f64,
        var_vsbstar_dn8: f64,
        var_vsbx: f64,
        var_vsbx_dn5: f64,
        var_vsbx_dn6: f64,
        var_vsbx_dn7: f64,
        var_vsbx_dn8: f64,
        var_xctmax: f64,
        var_xgct: f64,
        var_xgct_dn5: f64,
        var_xgct_dn6: f64,
        var_xgct_dn7: f64,
        var_xgct_dn8: f64,
        var_xmict: f64,
        var_xmict_dn5: f64,
        var_xmict_dn6: f64,
        var_xmict_dn7: f64,
        var_xmict_dn8: f64,
        var_xsbstar: f64,
        var_xsbstar_dn5: f64,
        var_xsbstar_dn6: f64,
        var_xsbstar_dn7: f64,
        var_xsbstar_dn8: f64,
        var_xwict: f64,
        var_xwict_dn5: f64,
        var_xwict_dn6: f64,
        var_xwict_dn7: f64,
        var_xwict_dn8: f64,
        var_ct_fact_slot: &mut f64,
        var_ct_fact_dn5_slot: &mut f64,
        var_ct_fact_dn6_slot: &mut f64,
        var_ct_fact_dn7_slot: &mut f64,
        var_ct_fact_dn8_slot: &mut f64,
        var_ct_fact_rv_slot: &mut f64,
        var_dctg_slot: &mut f64,
        var_dctg_dn5_slot: &mut f64,
        var_dctg_dn6_slot: &mut f64,
        var_dctg_dn7_slot: &mut f64,
        var_dctg_dn8_slot: &mut f64,
        var_dctg_rv_slot: &mut f64,
        var_delphib_slot: &mut f64,
        var_delphib_dn5_slot: &mut f64,
        var_delphib_dn6_slot: &mut f64,
        var_delphib_dn7_slot: &mut f64,
        var_delphib_dn8_slot: &mut f64,
        var_delphib_rv_slot: &mut f64,
        var_delta_ns_slot: &mut f64,
        var_delta_ns_dn5_slot: &mut f64,
        var_delta_ns_dn6_slot: &mut f64,
        var_delta_ns_dn7_slot: &mut f64,
        var_delta_ns_dn8_slot: &mut f64,
        var_delta_ns_rv_slot: &mut f64,
        var_delxb_slot: &mut f64,
        var_delxb_dn5_slot: &mut f64,
        var_delxb_dn6_slot: &mut f64,
        var_delxb_dn7_slot: &mut f64,
        var_delxb_dn8_slot: &mut f64,
        var_delxb_rv_slot: &mut f64,
        var_dphit1_slot: &mut f64,
        var_dphit1_dn5_slot: &mut f64,
        var_dphit1_dn6_slot: &mut f64,
        var_dphit1_dn7_slot: &mut f64,
        var_dphit1_dn8_slot: &mut f64,
        var_dphit1_rv_slot: &mut f64,
        var_gf_slot: &mut f64,
        var_gf2_slot: &mut f64,
        var_gf2_dn5_slot: &mut f64,
        var_gf2_dn6_slot: &mut f64,
        var_gf2_dn7_slot: &mut f64,
        var_gf2_dn8_slot: &mut f64,
        var_gf2_rv_slot: &mut f64,
        var_gf_dn5_slot: &mut f64,
        var_gf_dn6_slot: &mut f64,
        var_gf_dn7_slot: &mut f64,
        var_gf_dn8_slot: &mut f64,
        var_gf_rv_slot: &mut f64,
        var_guard1174_slot: &mut f64,
        var_guard1174_rv_slot: &mut f64,
        var_guard1175_slot: &mut f64,
        var_guard1175_rv_slot: &mut f64,
        var_guard1176_slot: &mut f64,
        var_guard1176_rv_slot: &mut f64,
        var_guard1177_slot: &mut f64,
        var_guard1177_rv_slot: &mut f64,
        var_inv_gf2_slot: &mut f64,
        var_inv_gf2_dn5_slot: &mut f64,
        var_inv_gf2_dn6_slot: &mut f64,
        var_inv_gf2_dn7_slot: &mut f64,
        var_inv_gf2_dn8_slot: &mut f64,
        var_inv_gf2_rv_slot: &mut f64,
        var_inv_phit1_slot: &mut f64,
        var_inv_phit1_dn5_slot: &mut f64,
        var_inv_phit1_dn6_slot: &mut f64,
        var_inv_phit1_dn7_slot: &mut f64,
        var_inv_phit1_dn8_slot: &mut f64,
        var_inv_phit1_rv_slot: &mut f64,
        var_nscr_slot: &mut f64,
        var_nscr_dn5_slot: &mut f64,
        var_nscr_dn6_slot: &mut f64,
        var_nscr_dn7_slot: &mut f64,
        var_nscr_dn8_slot: &mut f64,
        var_nscr_rv_slot: &mut f64,
        var_phit1_slot: &mut f64,
        var_phit1_dn5_slot: &mut f64,
        var_phit1_dn6_slot: &mut f64,
        var_phit1_dn7_slot: &mut f64,
        var_phit1_dn8_slot: &mut f64,
        var_phit1_rv_slot: &mut f64,
        var_phitct_slot: &mut f64,
        var_phitct_dn5_slot: &mut f64,
        var_phitct_dn6_slot: &mut f64,
        var_phitct_dn7_slot: &mut f64,
        var_phitct_dn8_slot: &mut f64,
        var_phitct_rv_slot: &mut f64,
        var_temp1_slot: &mut f64,
        var_temp1_dn5_slot: &mut f64,
        var_temp1_dn6_slot: &mut f64,
        var_temp1_dn7_slot: &mut f64,
        var_temp1_dn8_slot: &mut f64,
        var_temp1_rv_slot: &mut f64,
        var_temp2_slot: &mut f64,
        var_temp2_dn5_slot: &mut f64,
        var_temp2_dn6_slot: &mut f64,
        var_temp2_dn7_slot: &mut f64,
        var_temp2_dn8_slot: &mut f64,
        var_temp2_rv_slot: &mut f64,
        var_temp__blk936_slot: &mut f64,
        var_temp__blk936_dn5_slot: &mut f64,
        var_temp__blk936_dn6_slot: &mut f64,
        var_temp__blk936_dn7_slot: &mut f64,
        var_temp__blk936_dn8_slot: &mut f64,
        var_temp__blk936_rv_slot: &mut f64,
        var_ux_slot: &mut f64,
        var_ux_dn5_slot: &mut f64,
        var_ux_dn6_slot: &mut f64,
        var_ux_dn7_slot: &mut f64,
        var_ux_dn8_slot: &mut f64,
        var_ux_rv_slot: &mut f64,
        var_vdsp_slot: &mut f64,
        var_vdsp_dn6_slot: &mut f64,
        var_vdsp_dn7_slot: &mut f64,
        var_vdsp_rv_slot: &mut f64,
        var_xb_slot: &mut f64,
        var_xb_dn5_slot: &mut f64,
        var_xb_dn6_slot: &mut f64,
        var_xb_dn7_slot: &mut f64,
        var_xb_dn8_slot: &mut f64,
        var_xb_rv_slot: &mut f64,
        var_xct_slot: &mut f64,
        var_xct_dn5_slot: &mut f64,
        var_xct_dn6_slot: &mut f64,
        var_xct_dn7_slot: &mut f64,
        var_xct_dn8_slot: &mut f64,
        var_xct_rv_slot: &mut f64,
        var_xg_slot: &mut f64,
        var_xg_dn5_slot: &mut f64,
        var_xg_dn6_slot: &mut f64,
        var_xg_dn7_slot: &mut f64,
        var_xg_dn8_slot: &mut f64,
        var_xg_rv_slot: &mut f64,
        var_xgtscr_slot: &mut f64,
        var_xgtscr_dn5_slot: &mut f64,
        var_xgtscr_dn6_slot: &mut f64,
        var_xgtscr_dn7_slot: &mut f64,
        var_xgtscr_dn8_slot: &mut f64,
        var_xgtscr_rv_slot: &mut f64,
        var_xn_s_slot: &mut f64,
        var_xn_s_dn5_slot: &mut f64,
        var_xn_s_dn6_slot: &mut f64,
        var_xn_s_dn7_slot: &mut f64,
        var_xn_s_dn8_slot: &mut f64,
        var_xn_s_rv_slot: &mut f64,
        var_xno_s_slot: &mut f64,
        var_xno_s_dn5_slot: &mut f64,
        var_xno_s_dn6_slot: &mut f64,
        var_xno_s_dn7_slot: &mut f64,
        var_xno_s_dn8_slot: &mut f64,
        var_xno_s_rv_slot: &mut f64,
        var_xsubct_slot: &mut f64,
        var_xsubct_dn5_slot: &mut f64,
        var_xsubct_dn6_slot: &mut f64,
        var_xsubct_dn7_slot: &mut f64,
        var_xsubct_dn8_slot: &mut f64,
        var_xsubct_rv_slot: &mut f64,
        var_xthscr_slot: &mut f64,
        var_xthscr_dn5_slot: &mut f64,
        var_xthscr_dn6_slot: &mut f64,
        var_xthscr_dn7_slot: &mut f64,
        var_xthscr_dn8_slot: &mut f64,
        var_xthscr_rv_slot: &mut f64,
    ) {
        let mut var_ct_fact: f64 = *var_ct_fact_slot;
        let mut var_ct_fact_dn5: f64 = *var_ct_fact_dn5_slot;
        let mut var_ct_fact_dn6: f64 = *var_ct_fact_dn6_slot;
        let mut var_ct_fact_dn7: f64 = *var_ct_fact_dn7_slot;
        let mut var_ct_fact_dn8: f64 = *var_ct_fact_dn8_slot;
        let mut var_ct_fact_rv: f64 = *var_ct_fact_rv_slot;
        let mut var_dctg: f64 = *var_dctg_slot;
        let mut var_dctg_dn5: f64 = *var_dctg_dn5_slot;
        let mut var_dctg_dn6: f64 = *var_dctg_dn6_slot;
        let mut var_dctg_dn7: f64 = *var_dctg_dn7_slot;
        let mut var_dctg_dn8: f64 = *var_dctg_dn8_slot;
        let mut var_dctg_rv: f64 = *var_dctg_rv_slot;
        let mut var_delphib: f64 = *var_delphib_slot;
        let mut var_delphib_dn5: f64 = *var_delphib_dn5_slot;
        let mut var_delphib_dn6: f64 = *var_delphib_dn6_slot;
        let mut var_delphib_dn7: f64 = *var_delphib_dn7_slot;
        let mut var_delphib_dn8: f64 = *var_delphib_dn8_slot;
        let mut var_delphib_rv: f64 = *var_delphib_rv_slot;
        let mut var_delta_ns: f64 = *var_delta_ns_slot;
        let mut var_delta_ns_dn5: f64 = *var_delta_ns_dn5_slot;
        let mut var_delta_ns_dn6: f64 = *var_delta_ns_dn6_slot;
        let mut var_delta_ns_dn7: f64 = *var_delta_ns_dn7_slot;
        let mut var_delta_ns_dn8: f64 = *var_delta_ns_dn8_slot;
        let mut var_delta_ns_rv: f64 = *var_delta_ns_rv_slot;
        let mut var_delxb: f64 = *var_delxb_slot;
        let mut var_delxb_dn5: f64 = *var_delxb_dn5_slot;
        let mut var_delxb_dn6: f64 = *var_delxb_dn6_slot;
        let mut var_delxb_dn7: f64 = *var_delxb_dn7_slot;
        let mut var_delxb_dn8: f64 = *var_delxb_dn8_slot;
        let mut var_delxb_rv: f64 = *var_delxb_rv_slot;
        let mut var_dphit1: f64 = *var_dphit1_slot;
        let mut var_dphit1_dn5: f64 = *var_dphit1_dn5_slot;
        let mut var_dphit1_dn6: f64 = *var_dphit1_dn6_slot;
        let mut var_dphit1_dn7: f64 = *var_dphit1_dn7_slot;
        let mut var_dphit1_dn8: f64 = *var_dphit1_dn8_slot;
        let mut var_dphit1_rv: f64 = *var_dphit1_rv_slot;
        let mut var_gf: f64 = *var_gf_slot;
        let mut var_gf2: f64 = *var_gf2_slot;
        let mut var_gf2_dn5: f64 = *var_gf2_dn5_slot;
        let mut var_gf2_dn6: f64 = *var_gf2_dn6_slot;
        let mut var_gf2_dn7: f64 = *var_gf2_dn7_slot;
        let mut var_gf2_dn8: f64 = *var_gf2_dn8_slot;
        let mut var_gf2_rv: f64 = *var_gf2_rv_slot;
        let mut var_gf_dn5: f64 = *var_gf_dn5_slot;
        let mut var_gf_dn6: f64 = *var_gf_dn6_slot;
        let mut var_gf_dn7: f64 = *var_gf_dn7_slot;
        let mut var_gf_dn8: f64 = *var_gf_dn8_slot;
        let mut var_gf_rv: f64 = *var_gf_rv_slot;
        let mut var_guard1174: f64 = *var_guard1174_slot;
        let mut var_guard1174_rv: f64 = *var_guard1174_rv_slot;
        let mut var_guard1175: f64 = *var_guard1175_slot;
        let mut var_guard1175_rv: f64 = *var_guard1175_rv_slot;
        let mut var_guard1176: f64 = *var_guard1176_slot;
        let mut var_guard1176_rv: f64 = *var_guard1176_rv_slot;
        let mut var_guard1177: f64 = *var_guard1177_slot;
        let mut var_guard1177_rv: f64 = *var_guard1177_rv_slot;
        let mut var_inv_gf2: f64 = *var_inv_gf2_slot;
        let mut var_inv_gf2_dn5: f64 = *var_inv_gf2_dn5_slot;
        let mut var_inv_gf2_dn6: f64 = *var_inv_gf2_dn6_slot;
        let mut var_inv_gf2_dn7: f64 = *var_inv_gf2_dn7_slot;
        let mut var_inv_gf2_dn8: f64 = *var_inv_gf2_dn8_slot;
        let mut var_inv_gf2_rv: f64 = *var_inv_gf2_rv_slot;
        let mut var_inv_phit1: f64 = *var_inv_phit1_slot;
        let mut var_inv_phit1_dn5: f64 = *var_inv_phit1_dn5_slot;
        let mut var_inv_phit1_dn6: f64 = *var_inv_phit1_dn6_slot;
        let mut var_inv_phit1_dn7: f64 = *var_inv_phit1_dn7_slot;
        let mut var_inv_phit1_dn8: f64 = *var_inv_phit1_dn8_slot;
        let mut var_inv_phit1_rv: f64 = *var_inv_phit1_rv_slot;
        let mut var_nscr: f64 = *var_nscr_slot;
        let mut var_nscr_dn5: f64 = *var_nscr_dn5_slot;
        let mut var_nscr_dn6: f64 = *var_nscr_dn6_slot;
        let mut var_nscr_dn7: f64 = *var_nscr_dn7_slot;
        let mut var_nscr_dn8: f64 = *var_nscr_dn8_slot;
        let mut var_nscr_rv: f64 = *var_nscr_rv_slot;
        let mut var_phit1: f64 = *var_phit1_slot;
        let mut var_phit1_dn5: f64 = *var_phit1_dn5_slot;
        let mut var_phit1_dn6: f64 = *var_phit1_dn6_slot;
        let mut var_phit1_dn7: f64 = *var_phit1_dn7_slot;
        let mut var_phit1_dn8: f64 = *var_phit1_dn8_slot;
        let mut var_phit1_rv: f64 = *var_phit1_rv_slot;
        let mut var_phitct: f64 = *var_phitct_slot;
        let mut var_phitct_dn5: f64 = *var_phitct_dn5_slot;
        let mut var_phitct_dn6: f64 = *var_phitct_dn6_slot;
        let mut var_phitct_dn7: f64 = *var_phitct_dn7_slot;
        let mut var_phitct_dn8: f64 = *var_phitct_dn8_slot;
        let mut var_phitct_rv: f64 = *var_phitct_rv_slot;
        let mut var_temp1: f64 = *var_temp1_slot;
        let mut var_temp1_dn5: f64 = *var_temp1_dn5_slot;
        let mut var_temp1_dn6: f64 = *var_temp1_dn6_slot;
        let mut var_temp1_dn7: f64 = *var_temp1_dn7_slot;
        let mut var_temp1_dn8: f64 = *var_temp1_dn8_slot;
        let mut var_temp1_rv: f64 = *var_temp1_rv_slot;
        let mut var_temp2: f64 = *var_temp2_slot;
        let mut var_temp2_dn5: f64 = *var_temp2_dn5_slot;
        let mut var_temp2_dn6: f64 = *var_temp2_dn6_slot;
        let mut var_temp2_dn7: f64 = *var_temp2_dn7_slot;
        let mut var_temp2_dn8: f64 = *var_temp2_dn8_slot;
        let mut var_temp2_rv: f64 = *var_temp2_rv_slot;
        let mut var_temp__blk936: f64 = *var_temp__blk936_slot;
        let mut var_temp__blk936_dn5: f64 = *var_temp__blk936_dn5_slot;
        let mut var_temp__blk936_dn6: f64 = *var_temp__blk936_dn6_slot;
        let mut var_temp__blk936_dn7: f64 = *var_temp__blk936_dn7_slot;
        let mut var_temp__blk936_dn8: f64 = *var_temp__blk936_dn8_slot;
        let mut var_temp__blk936_rv: f64 = *var_temp__blk936_rv_slot;
        let mut var_ux: f64 = *var_ux_slot;
        let mut var_ux_dn5: f64 = *var_ux_dn5_slot;
        let mut var_ux_dn6: f64 = *var_ux_dn6_slot;
        let mut var_ux_dn7: f64 = *var_ux_dn7_slot;
        let mut var_ux_dn8: f64 = *var_ux_dn8_slot;
        let mut var_ux_rv: f64 = *var_ux_rv_slot;
        let mut var_vdsp: f64 = *var_vdsp_slot;
        let mut var_vdsp_dn6: f64 = *var_vdsp_dn6_slot;
        let mut var_vdsp_dn7: f64 = *var_vdsp_dn7_slot;
        let mut var_vdsp_rv: f64 = *var_vdsp_rv_slot;
        let mut var_xb: f64 = *var_xb_slot;
        let mut var_xb_dn5: f64 = *var_xb_dn5_slot;
        let mut var_xb_dn6: f64 = *var_xb_dn6_slot;
        let mut var_xb_dn7: f64 = *var_xb_dn7_slot;
        let mut var_xb_dn8: f64 = *var_xb_dn8_slot;
        let mut var_xb_rv: f64 = *var_xb_rv_slot;
        let mut var_xct: f64 = *var_xct_slot;
        let mut var_xct_dn5: f64 = *var_xct_dn5_slot;
        let mut var_xct_dn6: f64 = *var_xct_dn6_slot;
        let mut var_xct_dn7: f64 = *var_xct_dn7_slot;
        let mut var_xct_dn8: f64 = *var_xct_dn8_slot;
        let mut var_xct_rv: f64 = *var_xct_rv_slot;
        let mut var_xg: f64 = *var_xg_slot;
        let mut var_xg_dn5: f64 = *var_xg_dn5_slot;
        let mut var_xg_dn6: f64 = *var_xg_dn6_slot;
        let mut var_xg_dn7: f64 = *var_xg_dn7_slot;
        let mut var_xg_dn8: f64 = *var_xg_dn8_slot;
        let mut var_xg_rv: f64 = *var_xg_rv_slot;
        let mut var_xgtscr: f64 = *var_xgtscr_slot;
        let mut var_xgtscr_dn5: f64 = *var_xgtscr_dn5_slot;
        let mut var_xgtscr_dn6: f64 = *var_xgtscr_dn6_slot;
        let mut var_xgtscr_dn7: f64 = *var_xgtscr_dn7_slot;
        let mut var_xgtscr_dn8: f64 = *var_xgtscr_dn8_slot;
        let mut var_xgtscr_rv: f64 = *var_xgtscr_rv_slot;
        let mut var_xn_s: f64 = *var_xn_s_slot;
        let mut var_xn_s_dn5: f64 = *var_xn_s_dn5_slot;
        let mut var_xn_s_dn6: f64 = *var_xn_s_dn6_slot;
        let mut var_xn_s_dn7: f64 = *var_xn_s_dn7_slot;
        let mut var_xn_s_dn8: f64 = *var_xn_s_dn8_slot;
        let mut var_xn_s_rv: f64 = *var_xn_s_rv_slot;
        let mut var_xno_s: f64 = *var_xno_s_slot;
        let mut var_xno_s_dn5: f64 = *var_xno_s_dn5_slot;
        let mut var_xno_s_dn6: f64 = *var_xno_s_dn6_slot;
        let mut var_xno_s_dn7: f64 = *var_xno_s_dn7_slot;
        let mut var_xno_s_dn8: f64 = *var_xno_s_dn8_slot;
        let mut var_xno_s_rv: f64 = *var_xno_s_rv_slot;
        let mut var_xsubct: f64 = *var_xsubct_slot;
        let mut var_xsubct_dn5: f64 = *var_xsubct_dn5_slot;
        let mut var_xsubct_dn6: f64 = *var_xsubct_dn6_slot;
        let mut var_xsubct_dn7: f64 = *var_xsubct_dn7_slot;
        let mut var_xsubct_dn8: f64 = *var_xsubct_dn8_slot;
        let mut var_xsubct_rv: f64 = *var_xsubct_rv_slot;
        let mut var_xthscr: f64 = *var_xthscr_slot;
        let mut var_xthscr_dn5: f64 = *var_xthscr_dn5_slot;
        let mut var_xthscr_dn6: f64 = *var_xthscr_dn6_slot;
        let mut var_xthscr_dn7: f64 = *var_xthscr_dn7_slot;
        let mut var_xthscr_dn8: f64 = *var_xthscr_dn8_slot;
        let mut var_xthscr_rv: f64 = *var_xthscr_rv_slot;

        let (assign40930_e53841, assign40930_e53841_d_n5, assign40930_e53841_d_n6, assign40930_e53841_d_n7, assign40930_e53841_d_n8,) = {
    if (var_guard1173 != 0.0) {
        let assign40930_e53827: f64 = (var_xwict + var_xmict);
        let assign40930_e53830: f64 = (var_xwict - var_xmict);
        let assign40930_e53833: f64 = (var_xwict - var_xmict);
        let assign40930_e53834: f64 = (assign40930_e53830 * assign40930_e53833);
        let assign40930_e53836: f64 = (assign40930_e53834 + 20.0);
        let assign40930_e53837: f64 = (assign40930_e53836).sqrt();
        let assign40930_e53838: f64 = (assign40930_e53827 + assign40930_e53837);
        let assign40930_e53839: f64 = (0.5 * assign40930_e53838);
        (assign40930_e53839, (0.5 * ((var_xwict_dn5 + var_xmict_dn5) + ((((var_xwict_dn5 - var_xmict_dn5) * assign40930_e53833) + (assign40930_e53830 * (var_xwict_dn5 - var_xmict_dn5))) / (2.0 * assign40930_e53837)))), (0.5 * ((var_xwict_dn6 + var_xmict_dn6) + ((((var_xwict_dn6 - var_xmict_dn6) * assign40930_e53833) + (assign40930_e53830 * (var_xwict_dn6 - var_xmict_dn6))) / (2.0 * assign40930_e53837)))), (0.5 * ((var_xwict_dn7 + var_xmict_dn7) + ((((var_xwict_dn7 - var_xmict_dn7) * assign40930_e53833) + (assign40930_e53830 * (var_xwict_dn7 - var_xmict_dn7))) / (2.0 * assign40930_e53837)))), (0.5 * ((var_xwict_dn8 + var_xmict_dn8) + ((((var_xwict_dn8 - var_xmict_dn8) * assign40930_e53833) + (assign40930_e53830 * (var_xwict_dn8 - var_xmict_dn8))) / (2.0 * assign40930_e53837)))),)
    } else {
        (var_temp1, var_temp1_dn5, var_temp1_dn6, var_temp1_dn7, var_temp1_dn8,)
    }
};
        var_temp1 = assign40930_e53841;
        var_temp1_dn5 = assign40930_e53841_d_n5;
        var_temp1_dn6 = assign40930_e53841_d_n6;
        var_temp1_dn7 = assign40930_e53841_d_n7;
        var_temp1_dn8 = assign40930_e53841_d_n8;
        var_temp1_rv = 0.0;

        let (assign40940_e53851, assign40940_e53851_d_n5, assign40940_e53851_d_n6, assign40940_e53851_d_n7, assign40940_e53851_d_n8,) = {
    if (var_guard1173 != 0.0) {
        let assign40940_e53846: f64 = (var_xgct - var_xsbstar);
        let assign40940_e53847: f64 = (2.0 * assign40940_e53846);
        let assign40940_e53849: f64 = (assign40940_e53847 - var_xctmax);
        (assign40940_e53849, (2.0 * (var_xgct_dn5 - var_xsbstar_dn5)), (2.0 * (var_xgct_dn6 - var_xsbstar_dn6)), (2.0 * (var_xgct_dn7 - var_xsbstar_dn7)), (2.0 * (var_xgct_dn8 - var_xsbstar_dn8)),)
    } else {
        (var_temp2, var_temp2_dn5, var_temp2_dn6, var_temp2_dn7, var_temp2_dn8,)
    }
};
        var_temp2 = assign40940_e53851;
        var_temp2_dn5 = assign40940_e53851_d_n5;
        var_temp2_dn6 = assign40940_e53851_d_n6;
        var_temp2_dn7 = assign40940_e53851_d_n7;
        var_temp2_dn8 = assign40940_e53851_d_n8;
        var_temp2_rv = 0.0;

        let (assign40950_e53870, assign40950_e53870_d_n5, assign40950_e53870_d_n6, assign40950_e53870_d_n7, assign40950_e53870_d_n8,) = {
    if (var_guard1173 != 0.0) {
        let assign40950_e53856: f64 = (var_temp1 + var_temp2);
        let assign40950_e53859: f64 = (var_temp1 - var_temp2);
        let assign40950_e53862: f64 = (var_temp1 - var_temp2);
        let assign40950_e53863: f64 = (assign40950_e53859 * assign40950_e53862);
        let assign40950_e53865: f64 = (assign40950_e53863 + 20.0);
        let assign40950_e53866: f64 = (assign40950_e53865).sqrt();
        let assign40950_e53867: f64 = (assign40950_e53856 - assign40950_e53866);
        let assign40950_e53868: f64 = (0.5 * assign40950_e53867);
        (assign40950_e53868, (0.5 * ((var_temp1_dn5 + var_temp2_dn5) - ((((var_temp1_dn5 - var_temp2_dn5) * assign40950_e53862) + (assign40950_e53859 * (var_temp1_dn5 - var_temp2_dn5))) / (2.0 * assign40950_e53866)))), (0.5 * ((var_temp1_dn6 + var_temp2_dn6) - ((((var_temp1_dn6 - var_temp2_dn6) * assign40950_e53862) + (assign40950_e53859 * (var_temp1_dn6 - var_temp2_dn6))) / (2.0 * assign40950_e53866)))), (0.5 * ((var_temp1_dn7 + var_temp2_dn7) - ((((var_temp1_dn7 - var_temp2_dn7) * assign40950_e53862) + (assign40950_e53859 * (var_temp1_dn7 - var_temp2_dn7))) / (2.0 * assign40950_e53866)))), (0.5 * ((var_temp1_dn8 + var_temp2_dn8) - ((((var_temp1_dn8 - var_temp2_dn8) * assign40950_e53862) + (assign40950_e53859 * (var_temp1_dn8 - var_temp2_dn8))) / (2.0 * assign40950_e53866)))),)
    } else {
        (var_xsubct, var_xsubct_dn5, var_xsubct_dn6, var_xsubct_dn7, var_xsubct_dn8,)
    }
};
        var_xsubct = assign40950_e53870;
        var_xsubct_dn5 = assign40950_e53870_d_n5;
        var_xsubct_dn6 = assign40950_e53870_d_n6;
        var_xsubct_dn7 = assign40950_e53870_d_n7;
        var_xsubct_dn8 = assign40950_e53870_d_n8;
        var_xsubct_rv = 0.0;

        let (assign40960_e53889, assign40960_e53889_d_n5, assign40960_e53889_d_n6, assign40960_e53889_d_n7, assign40960_e53889_d_n8,) = {
    if (var_guard1173 != 0.0) {
        let assign40960_e53875: f64 = (var_xsubct + var_xctmax);
        let assign40960_e53878: f64 = (var_xsubct - var_xctmax);
        let assign40960_e53881: f64 = (var_xsubct - var_xctmax);
        let assign40960_e53882: f64 = (assign40960_e53878 * assign40960_e53881);
        let assign40960_e53884: f64 = (assign40960_e53882 + 5.0);
        let assign40960_e53885: f64 = (assign40960_e53884).sqrt();
        let assign40960_e53886: f64 = (assign40960_e53875 - assign40960_e53885);
        let assign40960_e53887: f64 = (0.5 * assign40960_e53886);
        (assign40960_e53887, (0.5 * (var_xsubct_dn5 - (((var_xsubct_dn5 * assign40960_e53881) + (assign40960_e53878 * var_xsubct_dn5)) / (2.0 * assign40960_e53885)))), (0.5 * (var_xsubct_dn6 - (((var_xsubct_dn6 * assign40960_e53881) + (assign40960_e53878 * var_xsubct_dn6)) / (2.0 * assign40960_e53885)))), (0.5 * (var_xsubct_dn7 - (((var_xsubct_dn7 * assign40960_e53881) + (assign40960_e53878 * var_xsubct_dn7)) / (2.0 * assign40960_e53885)))), (0.5 * (var_xsubct_dn8 - (((var_xsubct_dn8 * assign40960_e53881) + (assign40960_e53878 * var_xsubct_dn8)) / (2.0 * assign40960_e53885)))),)
    } else {
        (var_temp1, var_temp1_dn5, var_temp1_dn6, var_temp1_dn7, var_temp1_dn8,)
    }
};
        var_temp1 = assign40960_e53889;
        var_temp1_dn5 = assign40960_e53889_d_n5;
        var_temp1_dn6 = assign40960_e53889_d_n6;
        var_temp1_dn7 = assign40960_e53889_d_n7;
        var_temp1_dn8 = assign40960_e53889_d_n8;
        var_temp1_rv = 0.0;

        let (assign40970_e53911, assign40970_e53911_d_n5, assign40970_e53911_d_n6, assign40970_e53911_d_n7, assign40970_e53911_d_n8,) = {
    if (var_guard1173 != 0.0) {
        let assign40970_e53894: f64 = (-var_xctmax);
        let assign40970_e53895: f64 = (var_temp1 + assign40970_e53894);
        let assign40970_e53898: f64 = (-var_xctmax);
        let assign40970_e53899: f64 = (var_temp1 - assign40970_e53898);
        let assign40970_e53902: f64 = (-var_xctmax);
        let assign40970_e53903: f64 = (var_temp1 - assign40970_e53902);
        let assign40970_e53904: f64 = (assign40970_e53899 * assign40970_e53903);
        let assign40970_e53906: f64 = (assign40970_e53904 + 20.0);
        let assign40970_e53907: f64 = (assign40970_e53906).sqrt();
        let assign40970_e53908: f64 = (assign40970_e53895 + assign40970_e53907);
        let assign40970_e53909: f64 = (0.5 * assign40970_e53908);
        (assign40970_e53909, (0.5 * (var_temp1_dn5 + (((var_temp1_dn5 * assign40970_e53903) + (assign40970_e53899 * var_temp1_dn5)) / (2.0 * assign40970_e53907)))), (0.5 * (var_temp1_dn6 + (((var_temp1_dn6 * assign40970_e53903) + (assign40970_e53899 * var_temp1_dn6)) / (2.0 * assign40970_e53907)))), (0.5 * (var_temp1_dn7 + (((var_temp1_dn7 * assign40970_e53903) + (assign40970_e53899 * var_temp1_dn7)) / (2.0 * assign40970_e53907)))), (0.5 * (var_temp1_dn8 + (((var_temp1_dn8 * assign40970_e53903) + (assign40970_e53899 * var_temp1_dn8)) / (2.0 * assign40970_e53907)))),)
    } else {
        (var_xct, var_xct_dn5, var_xct_dn6, var_xct_dn7, var_xct_dn8,)
    }
};
        var_xct = assign40970_e53911;
        var_xct_dn5 = assign40970_e53911_d_n5;
        var_xct_dn6 = assign40970_e53911_d_n6;
        var_xct_dn7 = assign40970_e53911_d_n7;
        var_xct_dn8 = assign40970_e53911_d_n8;
        var_xct_rv = 0.0;

        let (assign40980_e53921, assign40980_e53921_d_n5, assign40980_e53921_d_n6, assign40980_e53921_d_n7, assign40980_e53921_d_n8,) = {
    if (var_guard1173 != 0.0) {
        let assign40980_e53916: f64 = (var_xct / var_xctmax);
        let assign40980_e53918: f64 = (assign40980_e53916 + 1.0);
        let assign40980_e53919: f64 = (var_ctg_t * assign40980_e53918);
        (assign40980_e53919, (var_ctg_t * (var_xct_dn5 / var_xctmax)), (var_ctg_t * (var_xct_dn6 / var_xctmax)), (var_ctg_t * (var_xct_dn7 / var_xctmax)), (var_ctg_t * (var_xct_dn8 / var_xctmax)),)
    } else {
        (var_temp2, var_temp2_dn5, var_temp2_dn6, var_temp2_dn7, var_temp2_dn8,)
    }
};
        var_temp2 = assign40980_e53921;
        var_temp2_dn5 = assign40980_e53921_d_n5;
        var_temp2_dn6 = assign40980_e53921_d_n6;
        var_temp2_dn7 = assign40980_e53921_d_n7;
        var_temp2_dn8 = assign40980_e53921_d_n8;
        var_temp2_rv = 0.0;

        let assign40990_e53924: f64 = (-230.25850929940458);
        let assign40990_e53925: f64 = if var_temp2 > assign40990_e53924 { 1.0 } else { 0.0 };
        var_guard1174 = assign40990_e53925;
        var_guard1174_rv = 0.0;

        let (assign41000_e53932, assign41000_e53932_d_n5, assign41000_e53932_d_n6, assign41000_e53932_d_n7, assign41000_e53932_d_n8,) = {
    if ((var_guard1173 != 0.0) && (var_guard1174 != 0.0)) {
        let assign41000_e53930: f64 = (var_temp2).exp();
        (assign41000_e53930, (assign41000_e53930 * var_temp2_dn5), (assign41000_e53930 * var_temp2_dn6), (assign41000_e53930 * var_temp2_dn7), (assign41000_e53930 * var_temp2_dn8),)
    } else {
        (var_dctg, var_dctg_dn5, var_dctg_dn6, var_dctg_dn7, var_dctg_dn8,)
    }
};
        var_dctg = assign41000_e53932;
        var_dctg_dn5 = assign41000_e53932_d_n5;
        var_dctg_dn6 = assign41000_e53932_d_n6;
        var_dctg_dn7 = assign41000_e53932_d_n7;
        var_dctg_dn8 = assign41000_e53932_d_n8;
        var_dctg_rv = 0.0;

        let (assign41010_e53964, assign41010_e53964_d_n5, assign41010_e53964_d_n6, assign41010_e53964_d_n7, assign41010_e53964_d_n8,) = {
    if ((var_guard1173 != 0.0) && (var_guard1174 == 0.0)) {
        let assign41010_e53940: f64 = (-230.25850929940458);
        let assign41010_e53942: f64 = (assign41010_e53940 - var_temp2);
        let assign41010_e53946: f64 = (-230.25850929940458);
        let assign41010_e53948: f64 = (assign41010_e53946 - var_temp2);
        let assign41010_e53951: f64 = (-230.25850929940458);
        let assign41010_e53953: f64 = (assign41010_e53951 - var_temp2);
        let assign41010_e53955: f64 = (assign41010_e53953 * 0.3333333333333333);
        let assign41010_e53956: f64 = (1.0 + assign41010_e53955);
        let assign41010_e53957: f64 = (assign41010_e53948 * assign41010_e53956);
        let assign41010_e53958: f64 = (0.5 * assign41010_e53957);
        let assign41010_e53959: f64 = (1.0 + assign41010_e53958);
        let assign41010_e53960: f64 = (assign41010_e53942 * assign41010_e53959);
        let assign41010_e53961: f64 = (1.0 + assign41010_e53960);
        let assign41010_e53962: f64 = (1e-100 / assign41010_e53961);
        (assign41010_e53962, (-((1e-100 * (((-var_temp2_dn5) * assign41010_e53959) + (assign41010_e53942 * (0.5 * (((-var_temp2_dn5) * assign41010_e53956) + (assign41010_e53948 * ((-var_temp2_dn5) * 0.3333333333333333))))))) / (assign41010_e53961 * assign41010_e53961))), (-((1e-100 * (((-var_temp2_dn6) * assign41010_e53959) + (assign41010_e53942 * (0.5 * (((-var_temp2_dn6) * assign41010_e53956) + (assign41010_e53948 * ((-var_temp2_dn6) * 0.3333333333333333))))))) / (assign41010_e53961 * assign41010_e53961))), (-((1e-100 * (((-var_temp2_dn7) * assign41010_e53959) + (assign41010_e53942 * (0.5 * (((-var_temp2_dn7) * assign41010_e53956) + (assign41010_e53948 * ((-var_temp2_dn7) * 0.3333333333333333))))))) / (assign41010_e53961 * assign41010_e53961))), (-((1e-100 * (((-var_temp2_dn8) * assign41010_e53959) + (assign41010_e53942 * (0.5 * (((-var_temp2_dn8) * assign41010_e53956) + (assign41010_e53948 * ((-var_temp2_dn8) * 0.3333333333333333))))))) / (assign41010_e53961 * assign41010_e53961))),)
    } else {
        (var_dctg, var_dctg_dn5, var_dctg_dn6, var_dctg_dn7, var_dctg_dn8,)
    }
};
        var_dctg = assign41010_e53964;
        var_dctg_dn5 = assign41010_e53964_d_n5;
        var_dctg_dn6 = assign41010_e53964_d_n6;
        var_dctg_dn7 = assign41010_e53964_d_n7;
        var_dctg_dn8 = assign41010_e53964_d_n8;
        var_dctg_rv = 0.0;

        let assign41020_e53968: f64 = (var_ct_t * var_dctg);
        let assign41020_e53969: f64 = (1.0 + assign41020_e53968);
        var_ct_fact = assign41020_e53969;
        var_ct_fact_dn5 = (var_ct_t * var_dctg_dn5);
        var_ct_fact_dn6 = (var_ct_t * var_dctg_dn6);
        var_ct_fact_dn7 = (var_ct_t * var_dctg_dn7);
        var_ct_fact_dn8 = (var_ct_t * var_dctg_dn8);
        var_ct_fact_rv = 0.0;

        let assign41030_e53972: f64 = (var_phit * var_ct_fact);
        var_phitct = assign41030_e53972;
        var_phitct_dn5 = (var_phit * var_ct_fact_dn5);
        var_phitct_dn6 = (var_phit * var_ct_fact_dn6);
        var_phitct_dn7 = (var_phit * var_ct_fact_dn7);
        var_phitct_dn8 = (var_phit * var_ct_fact_dn8);
        var_phitct_rv = 0.0;

        let assign41040_e53977: f64 = (var_psced_i * var_vdsx);
        let assign41040_e53978: f64 = (1.0 + assign41040_e53977);
        let assign41040_e53979: f64 = (var_psce_i * assign41040_e53978);
        let assign41040_e53983: f64 = (var_psceb_i * var_vsbx);
        let assign41040_e53984: f64 = (1.0 + assign41040_e53983);
        let assign41040_e53985: f64 = (assign41040_e53979 * assign41040_e53984);
        var_dphit1 = assign41040_e53985;
        var_dphit1_dn5 = (assign41040_e53979 * (var_psceb_i * var_vsbx_dn5));
        var_dphit1_dn6 = (((var_psce_i * (var_psced_i * var_vdsx_dn6)) * assign41040_e53984) + (assign41040_e53979 * (var_psceb_i * var_vsbx_dn6)));
        var_dphit1_dn7 = (((var_psce_i * (var_psced_i * var_vdsx_dn7)) * assign41040_e53984) + (assign41040_e53979 * (var_psceb_i * var_vsbx_dn7)));
        var_dphit1_dn8 = (assign41040_e53979 * (var_psceb_i * var_vsbx_dn8));
        var_dphit1_rv = 0.0;

        let assign41050_e53989: f64 = (1.0 + var_dphit1);
        let assign41050_e53990: f64 = (var_phitct * assign41050_e53989);
        var_phit1 = assign41050_e53990;
        var_phit1_dn5 = ((var_phitct_dn5 * assign41050_e53989) + (var_phitct * var_dphit1_dn5));
        var_phit1_dn6 = ((var_phitct_dn6 * assign41050_e53989) + (var_phitct * var_dphit1_dn6));
        var_phit1_dn7 = ((var_phitct_dn7 * assign41050_e53989) + (var_phitct * var_dphit1_dn7));
        var_phit1_dn8 = ((var_phitct_dn8 * assign41050_e53989) + (var_phitct * var_dphit1_dn8));
        var_phit1_rv = 0.0;

        let assign41060_e53993: f64 = (1.0 / var_phit1);
        var_inv_phit1 = assign41060_e53993;
        var_inv_phit1_dn5 = (-(var_phit1_dn5 / (var_phit1 * var_phit1)));
        var_inv_phit1_dn6 = (-(var_phit1_dn6 / (var_phit1 * var_phit1)));
        var_inv_phit1_dn7 = (-(var_phit1_dn7 / (var_phit1 * var_phit1)));
        var_inv_phit1_dn8 = (-(var_phit1_dn8 / (var_phit1 * var_phit1)));
        var_inv_phit1_rv = 0.0;

        let assign41070_e53997: f64 = (var_phit * var_inv_phit1);
        let assign41070_e53998: f64 = (assign41070_e53997).sqrt();
        let assign41070_e53999: f64 = (var_g_0 * assign41070_e53998);
        var_gf = assign41070_e53999;
        var_gf_dn5 = (var_g_0 * ((var_phit * var_inv_phit1_dn5) / (2.0 * assign41070_e53998)));
        var_gf_dn6 = (var_g_0 * ((var_phit * var_inv_phit1_dn6) / (2.0 * assign41070_e53998)));
        var_gf_dn7 = (var_g_0 * ((var_phit * var_inv_phit1_dn7) / (2.0 * assign41070_e53998)));
        var_gf_dn8 = (var_g_0 * ((var_phit * var_inv_phit1_dn8) / (2.0 * assign41070_e53998)));
        var_gf_rv = 0.0;

        let assign41080_e54002: f64 = (var_gf * var_gf);
        var_gf2 = assign41080_e54002;
        var_gf2_dn5 = ((var_gf_dn5 * var_gf) + (var_gf * var_gf_dn5));
        var_gf2_dn6 = ((var_gf_dn6 * var_gf) + (var_gf * var_gf_dn6));
        var_gf2_dn7 = ((var_gf_dn7 * var_gf) + (var_gf * var_gf_dn7));
        var_gf2_dn8 = ((var_gf_dn8 * var_gf) + (var_gf * var_gf_dn8));
        var_gf2_rv = 0.0;

        let assign41090_e54005: f64 = (1.0 / var_gf2);
        var_inv_gf2 = assign41090_e54005;
        var_inv_gf2_dn5 = (-(var_gf2_dn5 / (var_gf2 * var_gf2)));
        var_inv_gf2_dn6 = (-(var_gf2_dn6 / (var_gf2 * var_gf2)));
        var_inv_gf2_dn7 = (-(var_gf2_dn7 / (var_gf2 * var_gf2)));
        var_inv_gf2_dn8 = (-(var_gf2_dn8 / (var_gf2 * var_gf2)));
        var_inv_gf2_rv = 0.0;

        let assign41100_e54008: f64 = (var_vsbstar * var_inv_phit1);
        var_ux = assign41100_e54008;
        var_ux_dn5 = ((var_vsbstar_dn5 * var_inv_phit1) + (var_vsbstar * var_inv_phit1_dn5));
        var_ux_dn6 = ((var_vsbstar_dn6 * var_inv_phit1) + (var_vsbstar * var_inv_phit1_dn6));
        var_ux_dn7 = ((var_vsbstar_dn7 * var_inv_phit1) + (var_vsbstar * var_inv_phit1_dn7));
        var_ux_dn8 = ((var_vsbstar_dn8 * var_inv_phit1) + (var_vsbstar * var_inv_phit1_dn8));
        var_ux_rv = 0.0;

        let assign41110_e54011: f64 = (var_vgb1 * var_inv_phit1);
        var_xg = assign41110_e54011;
        var_xg_dn5 = ((var_vgb1_dn5 * var_inv_phit1) + (var_vgb1 * var_inv_phit1_dn5));
        var_xg_dn6 = ((var_vgb1_dn6 * var_inv_phit1) + (var_vgb1 * var_inv_phit1_dn6));
        var_xg_dn7 = ((var_vgb1_dn7 * var_inv_phit1) + (var_vgb1 * var_inv_phit1_dn7));
        var_xg_dn8 = ((var_vgb1_dn8 * var_inv_phit1) + (var_vgb1 * var_inv_phit1_dn8));
        var_xg_rv = 0.0;

        let assign41120_e54014: f64 = (2.0 * var_vdsx);
        let assign41120_e54019: f64 = (var_cfd_i * var_vdsx);
        let assign41120_e54020: f64 = (1.0 + assign41120_e54019);
        let assign41120_e54021: f64 = (assign41120_e54020).sqrt();
        let assign41120_e54022: f64 = (1.0 + assign41120_e54021);
        let assign41120_e54023: f64 = (assign41120_e54014 / assign41120_e54022);
        var_vdsp = assign41120_e54023;
        var_vdsp_dn6 = ((((2.0 * var_vdsx_dn6) * assign41120_e54022) - (assign41120_e54014 * ((var_cfd_i * var_vdsx_dn6) / (2.0 * assign41120_e54021)))) / (assign41120_e54022 * assign41120_e54022));
        var_vdsp_dn7 = ((((2.0 * var_vdsx_dn7) * assign41120_e54022) - (assign41120_e54014 * ((var_cfd_i * var_vdsx_dn7) / (2.0 * assign41120_e54021)))) / (assign41120_e54022 * assign41120_e54022));
        var_vdsp_rv = 0.0;

        let assign41130_e54026: f64 = (var_cf_i * var_vdsp);
        let assign41130_e54030: f64 = (var_cfb_i * var_vsbx);
        let assign41130_e54031: f64 = (1.0 + assign41130_e54030);
        let assign41130_e54032: f64 = (assign41130_e54026 * assign41130_e54031);
        var_delphib = assign41130_e54032;
        var_delphib_dn5 = (assign41130_e54026 * (var_cfb_i * var_vsbx_dn5));
        var_delphib_dn6 = (((var_cf_i * var_vdsp_dn6) * assign41130_e54031) + (assign41130_e54026 * (var_cfb_i * var_vsbx_dn6)));
        var_delphib_dn7 = (((var_cf_i * var_vdsp_dn7) * assign41130_e54031) + (assign41130_e54026 * (var_cfb_i * var_vsbx_dn7)));
        var_delphib_dn8 = (assign41130_e54026 * (var_cfb_i * var_vsbx_dn8));
        var_delphib_rv = 0.0;

        let assign41140_e54035: f64 = (var_phib * var_inv_phit1);
        var_xb = assign41140_e54035;
        var_xb_dn5 = (var_phib * var_inv_phit1_dn5);
        var_xb_dn6 = (var_phib * var_inv_phit1_dn6);
        var_xb_dn7 = (var_phib * var_inv_phit1_dn7);
        var_xb_dn8 = (var_phib * var_inv_phit1_dn8);
        var_xb_rv = 0.0;

        let assign41150_e54038: f64 = (var_v_xb * var_v_xb);
        let assign41150_e54040: f64 = (assign41150_e54038 + var_aphi);
        let assign41150_e54041: f64 = (assign41150_e54040).sqrt();
        var_temp1 = assign41150_e54041;
        var_temp1_dn5 = 0.0;
        var_temp1_dn6 = (((var_v_xb_dn6 * var_v_xb) + (var_v_xb * var_v_xb_dn6)) / (2.0 * assign41150_e54041));
        var_temp1_dn7 = (((var_v_xb_dn7 * var_v_xb) + (var_v_xb * var_v_xb_dn7)) / (2.0 * assign41150_e54041));
        var_temp1_dn8 = (((var_v_xb_dn8 * var_v_xb) + (var_v_xb * var_v_xb_dn8)) / (2.0 * assign41150_e54041));
        var_temp1_rv = 0.0;

        let assign41160_e54044: f64 = (var_v_xb - var_delphib);
        let assign41160_e54047: f64 = (var_v_xb - var_delphib);
        let assign41160_e54048: f64 = (assign41160_e54044 * assign41160_e54047);
        let assign41160_e54050: f64 = (assign41160_e54048 + var_aphi);
        let assign41160_e54051: f64 = (assign41160_e54050).sqrt();
        var_temp2 = assign41160_e54051;
        var_temp2_dn5 = ((((-var_delphib_dn5) * assign41160_e54047) + (assign41160_e54044 * (-var_delphib_dn5))) / (2.0 * assign41160_e54051));
        var_temp2_dn6 = ((((var_v_xb_dn6 - var_delphib_dn6) * assign41160_e54047) + (assign41160_e54044 * (var_v_xb_dn6 - var_delphib_dn6))) / (2.0 * assign41160_e54051));
        var_temp2_dn7 = ((((var_v_xb_dn7 - var_delphib_dn7) * assign41160_e54047) + (assign41160_e54044 * (var_v_xb_dn7 - var_delphib_dn7))) / (2.0 * assign41160_e54051));
        var_temp2_dn8 = ((((var_v_xb_dn8 - var_delphib_dn8) * assign41160_e54047) + (assign41160_e54044 * (var_v_xb_dn8 - var_delphib_dn8))) / (2.0 * assign41160_e54051));
        var_temp2_rv = 0.0;

        let assign41170_e54054: f64 = (0.5 * var_inv_phit1);
        let assign41170_e54057: f64 = (var_delphib + var_temp1);
        let assign41170_e54059: f64 = (assign41170_e54057 - var_temp2);
        let assign41170_e54060: f64 = (assign41170_e54054 * assign41170_e54059);
        var_delxb = assign41170_e54060;
        var_delxb_dn5 = (((0.5 * var_inv_phit1_dn5) * assign41170_e54059) + (assign41170_e54054 * ((var_delphib_dn5 + var_temp1_dn5) - var_temp2_dn5)));
        var_delxb_dn6 = (((0.5 * var_inv_phit1_dn6) * assign41170_e54059) + (assign41170_e54054 * ((var_delphib_dn6 + var_temp1_dn6) - var_temp2_dn6)));
        var_delxb_dn7 = (((0.5 * var_inv_phit1_dn7) * assign41170_e54059) + (assign41170_e54054 * ((var_delphib_dn7 + var_temp1_dn7) - var_temp2_dn7)));
        var_delxb_dn8 = (((0.5 * var_inv_phit1_dn8) * assign41170_e54059) + (assign41170_e54054 * ((var_delphib_dn8 + var_temp1_dn8) - var_temp2_dn8)));
        var_delxb_rv = 0.0;

        let assign41180_e54063: f64 = (var_xb + var_ux);
        var_xno_s = assign41180_e54063;
        var_xno_s_dn5 = (var_xb_dn5 + var_ux_dn5);
        var_xno_s_dn6 = (var_xb_dn6 + var_ux_dn6);
        var_xno_s_dn7 = (var_xb_dn7 + var_ux_dn7);
        var_xno_s_dn8 = (var_xb_dn8 + var_ux_dn8);
        var_xno_s_rv = 0.0;

        let assign41190_e54066: f64 = (var_xno_s - var_delxb);
        var_xn_s = assign41190_e54066;
        var_xn_s_dn5 = (var_xno_s_dn5 - var_delxb_dn5);
        var_xn_s_dn6 = (var_xno_s_dn6 - var_delxb_dn6);
        var_xn_s_dn7 = (var_xno_s_dn7 - var_delxb_dn7);
        var_xn_s_dn8 = (var_xno_s_dn8 - var_delxb_dn8);
        var_xn_s_rv = 0.0;

        let assign41200_e54069: f64 = if p.p45 > 0.0 { 1.0 } else { 0.0 };
        var_guard1175 = assign41200_e54069;
        var_guard1175_rv = 0.0;

        let assign41210_e54071: f64 = (var_xn_s).abs();
        let assign41210_e54073: f64 = if assign41210_e54071 < 1e-5 { 1.0 } else { 0.0 };
        var_guard1176 = assign41210_e54073;
        var_guard1176_rv = 0.0;

        let (assign41220_e54093, assign41220_e54093_d_n5, assign41220_e54093_d_n6, assign41220_e54093_d_n7, assign41220_e54093_d_n8,) = {
    if ((var_guard1175 != 0.0) && (var_guard1176 != 0.0)) {
        let assign41220_e54082: f64 = (0.5 * var_xn_s);
        let assign41220_e54086: f64 = (0.3125 * var_xn_s);
        let assign41220_e54087: f64 = (1.0 - assign41220_e54086);
        let assign41220_e54088: f64 = (assign41220_e54082 * assign41220_e54087);
        let assign41220_e54089: f64 = (1.0 - assign41220_e54088);
        let assign41220_e54090: f64 = (var_gf * assign41220_e54089);
        let assign41220_e54091: f64 = (1.0 + assign41220_e54090);
        (assign41220_e54091, ((var_gf_dn5 * assign41220_e54089) + (var_gf * (-(((0.5 * var_xn_s_dn5) * assign41220_e54087) + (assign41220_e54082 * (-(0.3125 * var_xn_s_dn5))))))), ((var_gf_dn6 * assign41220_e54089) + (var_gf * (-(((0.5 * var_xn_s_dn6) * assign41220_e54087) + (assign41220_e54082 * (-(0.3125 * var_xn_s_dn6))))))), ((var_gf_dn7 * assign41220_e54089) + (var_gf * (-(((0.5 * var_xn_s_dn7) * assign41220_e54087) + (assign41220_e54082 * (-(0.3125 * var_xn_s_dn7))))))), ((var_gf_dn8 * assign41220_e54089) + (var_gf * (-(((0.5 * var_xn_s_dn8) * assign41220_e54087) + (assign41220_e54082 * (-(0.3125 * var_xn_s_dn8))))))),)
    } else {
        (var_nscr, var_nscr_dn5, var_nscr_dn6, var_nscr_dn7, var_nscr_dn8,)
    }
};
        var_nscr = assign41220_e54093;
        var_nscr_dn5 = assign41220_e54093_d_n5;
        var_nscr_dn6 = assign41220_e54093_d_n6;
        var_nscr_dn7 = assign41220_e54093_d_n7;
        var_nscr_dn8 = assign41220_e54093_d_n8;
        var_nscr_rv = 0.0;

        let assign41230_e54096: f64 = if var_xn_s < 460.51701859880916 { 1.0 } else { 0.0 };
        var_guard1177 = assign41230_e54096;
        var_guard1177_rv = 0.0;

        let (assign41240_e54107, assign41240_e54107_d_n5, assign41240_e54107_d_n6, assign41240_e54107_d_n7, assign41240_e54107_d_n8,) = {
    if (((var_guard1175 != 0.0) && (var_guard1176 == 0.0)) && (var_guard1177 != 0.0)) {
        let assign41240_e54104: f64 = (-var_xn_s);
        let assign41240_e54105: f64 = (assign41240_e54104).exp();
        (assign41240_e54105, (assign41240_e54105 * (-var_xn_s_dn5)), (assign41240_e54105 * (-var_xn_s_dn6)), (assign41240_e54105 * (-var_xn_s_dn7)), (assign41240_e54105 * (-var_xn_s_dn8)),)
    } else {
        (var_delta_ns, var_delta_ns_dn5, var_delta_ns_dn6, var_delta_ns_dn7, var_delta_ns_dn8,)
    }
};
        var_delta_ns = assign41240_e54107;
        var_delta_ns_dn5 = assign41240_e54107_d_n5;
        var_delta_ns_dn6 = assign41240_e54107_d_n6;
        var_delta_ns_dn7 = assign41240_e54107_d_n7;
        var_delta_ns_dn8 = assign41240_e54107_d_n8;
        var_delta_ns_rv = 0.0;

        let (assign41250_e54139, assign41250_e54139_d_n5, assign41250_e54139_d_n6, assign41250_e54139_d_n7, assign41250_e54139_d_n8,) = {
    if (((var_guard1175 != 0.0) && (var_guard1176 == 0.0)) && (var_guard1177 == 0.0)) {
        let assign41250_e54119: f64 = (var_xn_s - 460.51701859880916);
        let assign41250_e54124: f64 = (var_xn_s - 460.51701859880916);
        let assign41250_e54128: f64 = (var_xn_s - 460.51701859880916);
        let assign41250_e54130: f64 = (assign41250_e54128 * 0.3333333333333333);
        let assign41250_e54131: f64 = (1.0 + assign41250_e54130);
        let assign41250_e54132: f64 = (assign41250_e54124 * assign41250_e54131);
        let assign41250_e54133: f64 = (0.5 * assign41250_e54132);
        let assign41250_e54134: f64 = (1.0 + assign41250_e54133);
        let assign41250_e54135: f64 = (assign41250_e54119 * assign41250_e54134);
        let assign41250_e54136: f64 = (1.0 + assign41250_e54135);
        let assign41250_e54137: f64 = (1e-200 / assign41250_e54136);
        (assign41250_e54137, (-((1e-200 * ((var_xn_s_dn5 * assign41250_e54134) + (assign41250_e54119 * (0.5 * ((var_xn_s_dn5 * assign41250_e54131) + (assign41250_e54124 * (var_xn_s_dn5 * 0.3333333333333333))))))) / (assign41250_e54136 * assign41250_e54136))), (-((1e-200 * ((var_xn_s_dn6 * assign41250_e54134) + (assign41250_e54119 * (0.5 * ((var_xn_s_dn6 * assign41250_e54131) + (assign41250_e54124 * (var_xn_s_dn6 * 0.3333333333333333))))))) / (assign41250_e54136 * assign41250_e54136))), (-((1e-200 * ((var_xn_s_dn7 * assign41250_e54134) + (assign41250_e54119 * (0.5 * ((var_xn_s_dn7 * assign41250_e54131) + (assign41250_e54124 * (var_xn_s_dn7 * 0.3333333333333333))))))) / (assign41250_e54136 * assign41250_e54136))), (-((1e-200 * ((var_xn_s_dn8 * assign41250_e54134) + (assign41250_e54119 * (0.5 * ((var_xn_s_dn8 * assign41250_e54131) + (assign41250_e54124 * (var_xn_s_dn8 * 0.3333333333333333))))))) / (assign41250_e54136 * assign41250_e54136))),)
    } else {
        (var_delta_ns, var_delta_ns_dn5, var_delta_ns_dn6, var_delta_ns_dn7, var_delta_ns_dn8,)
    }
};
        var_delta_ns = assign41250_e54139;
        var_delta_ns_dn5 = assign41250_e54139_d_n5;
        var_delta_ns_dn6 = assign41250_e54139_d_n6;
        var_delta_ns_dn7 = assign41250_e54139_d_n7;
        var_delta_ns_dn8 = assign41250_e54139_d_n8;
        var_delta_ns_rv = 0.0;

        let (assign41260_e54152, assign41260_e54152_d_n5, assign41260_e54152_d_n6, assign41260_e54152_d_n7, assign41260_e54152_d_n8,) = {
    if ((var_guard1175 != 0.0) && (var_guard1176 == 0.0)) {
        let (assign41260_e54150,) = {
            if (var_xn_s > 0.0) {
                (1.0,)
            } else {
                let assign41260_e54149: f64 = (-1.0);
                (assign41260_e54149,)
            }
        };
        (assign41260_e54150, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_temp__blk936, var_temp__blk936_dn5, var_temp__blk936_dn6, var_temp__blk936_dn7, var_temp__blk936_dn8,)
    }
};
        var_temp__blk936 = assign41260_e54152;
        var_temp__blk936_dn5 = assign41260_e54152_d_n5;
        var_temp__blk936_dn6 = assign41260_e54152_d_n6;
        var_temp__blk936_dn7 = assign41260_e54152_d_n7;
        var_temp__blk936_dn8 = assign41260_e54152_d_n8;
        var_temp__blk936_rv = 0.0;

        let (assign41270_e54180, assign41270_e54180_d_n5, assign41270_e54180_d_n6, assign41270_e54180_d_n7, assign41270_e54180_d_n8,) = {
    if ((var_guard1175 != 0.0) && (var_guard1176 == 0.0)) {
        let assign41270_e54160: f64 = (var_temp__blk936 * var_gf);
        let assign41270_e54165: f64 = (1.0 - var_xn_s);
        let assign41270_e54166: f64 = (var_delta_ns * assign41270_e54165);
        let assign41270_e54167: f64 = (1.0 - assign41270_e54166);
        let assign41270_e54168: f64 = (assign41270_e54160 * assign41270_e54167);
        let assign41270_e54173: f64 = (1.0 - var_delta_ns);
        let assign41270_e54174: f64 = (var_xn_s * assign41270_e54173);
        let assign41270_e54175: f64 = (assign41270_e54174).sqrt();
        let assign41270_e54176: f64 = (2.0 * assign41270_e54175);
        let assign41270_e54177: f64 = (assign41270_e54168 / assign41270_e54176);
        let assign41270_e54178: f64 = (1.0 + assign41270_e54177);
        (assign41270_e54178, (((((((var_temp__blk936_dn5 * var_gf) + (var_temp__blk936 * var_gf_dn5)) * assign41270_e54167) + (assign41270_e54160 * (-((var_delta_ns_dn5 * assign41270_e54165) + (var_delta_ns * (-var_xn_s_dn5)))))) * assign41270_e54176) - (assign41270_e54168 * (2.0 * (((var_xn_s_dn5 * assign41270_e54173) + (var_xn_s * (-var_delta_ns_dn5))) / (2.0 * assign41270_e54175))))) / (assign41270_e54176 * assign41270_e54176)), (((((((var_temp__blk936_dn6 * var_gf) + (var_temp__blk936 * var_gf_dn6)) * assign41270_e54167) + (assign41270_e54160 * (-((var_delta_ns_dn6 * assign41270_e54165) + (var_delta_ns * (-var_xn_s_dn6)))))) * assign41270_e54176) - (assign41270_e54168 * (2.0 * (((var_xn_s_dn6 * assign41270_e54173) + (var_xn_s * (-var_delta_ns_dn6))) / (2.0 * assign41270_e54175))))) / (assign41270_e54176 * assign41270_e54176)), (((((((var_temp__blk936_dn7 * var_gf) + (var_temp__blk936 * var_gf_dn7)) * assign41270_e54167) + (assign41270_e54160 * (-((var_delta_ns_dn7 * assign41270_e54165) + (var_delta_ns * (-var_xn_s_dn7)))))) * assign41270_e54176) - (assign41270_e54168 * (2.0 * (((var_xn_s_dn7 * assign41270_e54173) + (var_xn_s * (-var_delta_ns_dn7))) / (2.0 * assign41270_e54175))))) / (assign41270_e54176 * assign41270_e54176)), (((((((var_temp__blk936_dn8 * var_gf) + (var_temp__blk936 * var_gf_dn8)) * assign41270_e54167) + (assign41270_e54160 * (-((var_delta_ns_dn8 * assign41270_e54165) + (var_delta_ns * (-var_xn_s_dn8)))))) * assign41270_e54176) - (assign41270_e54168 * (2.0 * (((var_xn_s_dn8 * assign41270_e54173) + (var_xn_s * (-var_delta_ns_dn8))) / (2.0 * assign41270_e54175))))) / (assign41270_e54176 * assign41270_e54176)),)
    } else {
        (var_nscr, var_nscr_dn5, var_nscr_dn6, var_nscr_dn7, var_nscr_dn8,)
    }
};
        var_nscr = assign41270_e54180;
        var_nscr_dn5 = assign41270_e54180_d_n5;
        var_nscr_dn6 = assign41270_e54180_d_n6;
        var_nscr_dn7 = assign41270_e54180_d_n7;
        var_nscr_dn8 = assign41270_e54180_d_n8;
        var_nscr_rv = 0.0;

        let (assign41280_e54192, assign41280_e54192_d_n5, assign41280_e54192_d_n6, assign41280_e54192_d_n7, assign41280_e54192_d_n8,) = {
    if (var_guard1175 == 0.0) {
        let assign41280_e54186: f64 = (0.5 * var_gf);
        let assign41280_e54188: f64 = (var_xn_s).sqrt();
        let assign41280_e54189: f64 = (assign41280_e54186 / assign41280_e54188);
        let assign41280_e54190: f64 = (1.0 + assign41280_e54189);
        (assign41280_e54190, ((((0.5 * var_gf_dn5) * assign41280_e54188) - (assign41280_e54186 * (var_xn_s_dn5 / (2.0 * assign41280_e54188)))) / (assign41280_e54188 * assign41280_e54188)), ((((0.5 * var_gf_dn6) * assign41280_e54188) - (assign41280_e54186 * (var_xn_s_dn6 / (2.0 * assign41280_e54188)))) / (assign41280_e54188 * assign41280_e54188)), ((((0.5 * var_gf_dn7) * assign41280_e54188) - (assign41280_e54186 * (var_xn_s_dn7 / (2.0 * assign41280_e54188)))) / (assign41280_e54188 * assign41280_e54188)), ((((0.5 * var_gf_dn8) * assign41280_e54188) - (assign41280_e54186 * (var_xn_s_dn8 / (2.0 * assign41280_e54188)))) / (assign41280_e54188 * assign41280_e54188)),)
    } else {
        (var_nscr, var_nscr_dn5, var_nscr_dn6, var_nscr_dn7, var_nscr_dn8,)
    }
};
        var_nscr = assign41280_e54192;
        var_nscr_dn5 = assign41280_e54192_d_n5;
        var_nscr_dn6 = assign41280_e54192_d_n6;
        var_nscr_dn7 = assign41280_e54192_d_n7;
        var_nscr_dn8 = assign41280_e54192_d_n8;
        var_nscr_rv = 0.0;

        let assign41290_e54196: f64 = (var_xn_s).sqrt();
        let assign41290_e54197: f64 = (var_gf * assign41290_e54196);
        let assign41290_e54198: f64 = (var_xn_s + assign41290_e54197);
        let assign41290_e54202: f64 = (var_nscr - 1.0);
        let assign41290_e54203: f64 = (assign41290_e54202).ln();
        let assign41290_e54204: f64 = (var_nscr * assign41290_e54203);
        let assign41290_e54205: f64 = (assign41290_e54198 - assign41290_e54204);
        var_xthscr = assign41290_e54205;
        var_xthscr_dn5 = ((var_xn_s_dn5 + ((var_gf_dn5 * assign41290_e54196) + (var_gf * (var_xn_s_dn5 / (2.0 * assign41290_e54196))))) - ((var_nscr_dn5 * assign41290_e54203) + (var_nscr * (var_nscr_dn5 / assign41290_e54202))));
        var_xthscr_dn6 = ((var_xn_s_dn6 + ((var_gf_dn6 * assign41290_e54196) + (var_gf * (var_xn_s_dn6 / (2.0 * assign41290_e54196))))) - ((var_nscr_dn6 * assign41290_e54203) + (var_nscr * (var_nscr_dn6 / assign41290_e54202))));
        var_xthscr_dn7 = ((var_xn_s_dn7 + ((var_gf_dn7 * assign41290_e54196) + (var_gf * (var_xn_s_dn7 / (2.0 * assign41290_e54196))))) - ((var_nscr_dn7 * assign41290_e54203) + (var_nscr * (var_nscr_dn7 / assign41290_e54202))));
        var_xthscr_dn8 = ((var_xn_s_dn8 + ((var_gf_dn8 * assign41290_e54196) + (var_gf * (var_xn_s_dn8 / (2.0 * assign41290_e54196))))) - ((var_nscr_dn8 * assign41290_e54203) + (var_nscr * (var_nscr_dn8 / assign41290_e54202))));
        var_xthscr_rv = 0.0;

        let assign41300_e54208: f64 = (var_xg - var_xthscr);
        let assign41300_e54210: f64 = (assign41300_e54208 / var_nscr);
        var_xgtscr = assign41300_e54210;
        var_xgtscr_dn5 = ((((var_xg_dn5 - var_xthscr_dn5) * var_nscr) - (assign41300_e54208 * var_nscr_dn5)) / (var_nscr * var_nscr));
        var_xgtscr_dn6 = ((((var_xg_dn6 - var_xthscr_dn6) * var_nscr) - (assign41300_e54208 * var_nscr_dn6)) / (var_nscr * var_nscr));
        var_xgtscr_dn7 = ((((var_xg_dn7 - var_xthscr_dn7) * var_nscr) - (assign41300_e54208 * var_nscr_dn7)) / (var_nscr * var_nscr));
        var_xgtscr_dn8 = ((((var_xg_dn8 - var_xthscr_dn8) * var_nscr) - (assign41300_e54208 * var_nscr_dn8)) / (var_nscr * var_nscr));
        var_xgtscr_rv = 0.0;

        *var_ct_fact_slot = var_ct_fact;
        *var_ct_fact_dn5_slot = var_ct_fact_dn5;
        *var_ct_fact_dn6_slot = var_ct_fact_dn6;
        *var_ct_fact_dn7_slot = var_ct_fact_dn7;
        *var_ct_fact_dn8_slot = var_ct_fact_dn8;
        *var_ct_fact_rv_slot = var_ct_fact_rv;
        *var_dctg_slot = var_dctg;
        *var_dctg_dn5_slot = var_dctg_dn5;
        *var_dctg_dn6_slot = var_dctg_dn6;
        *var_dctg_dn7_slot = var_dctg_dn7;
        *var_dctg_dn8_slot = var_dctg_dn8;
        *var_dctg_rv_slot = var_dctg_rv;
        *var_delphib_slot = var_delphib;
        *var_delphib_dn5_slot = var_delphib_dn5;
        *var_delphib_dn6_slot = var_delphib_dn6;
        *var_delphib_dn7_slot = var_delphib_dn7;
        *var_delphib_dn8_slot = var_delphib_dn8;
        *var_delphib_rv_slot = var_delphib_rv;
        *var_delta_ns_slot = var_delta_ns;
        *var_delta_ns_dn5_slot = var_delta_ns_dn5;
        *var_delta_ns_dn6_slot = var_delta_ns_dn6;
        *var_delta_ns_dn7_slot = var_delta_ns_dn7;
        *var_delta_ns_dn8_slot = var_delta_ns_dn8;
        *var_delta_ns_rv_slot = var_delta_ns_rv;
        *var_delxb_slot = var_delxb;
        *var_delxb_dn5_slot = var_delxb_dn5;
        *var_delxb_dn6_slot = var_delxb_dn6;
        *var_delxb_dn7_slot = var_delxb_dn7;
        *var_delxb_dn8_slot = var_delxb_dn8;
        *var_delxb_rv_slot = var_delxb_rv;
        *var_dphit1_slot = var_dphit1;
        *var_dphit1_dn5_slot = var_dphit1_dn5;
        *var_dphit1_dn6_slot = var_dphit1_dn6;
        *var_dphit1_dn7_slot = var_dphit1_dn7;
        *var_dphit1_dn8_slot = var_dphit1_dn8;
        *var_dphit1_rv_slot = var_dphit1_rv;
        *var_gf_slot = var_gf;
        *var_gf2_slot = var_gf2;
        *var_gf2_dn5_slot = var_gf2_dn5;
        *var_gf2_dn6_slot = var_gf2_dn6;
        *var_gf2_dn7_slot = var_gf2_dn7;
        *var_gf2_dn8_slot = var_gf2_dn8;
        *var_gf2_rv_slot = var_gf2_rv;
        *var_gf_dn5_slot = var_gf_dn5;
        *var_gf_dn6_slot = var_gf_dn6;
        *var_gf_dn7_slot = var_gf_dn7;
        *var_gf_dn8_slot = var_gf_dn8;
        *var_gf_rv_slot = var_gf_rv;
        *var_guard1174_slot = var_guard1174;
        *var_guard1174_rv_slot = var_guard1174_rv;
        *var_guard1175_slot = var_guard1175;
        *var_guard1175_rv_slot = var_guard1175_rv;
        *var_guard1176_slot = var_guard1176;
        *var_guard1176_rv_slot = var_guard1176_rv;
        *var_guard1177_slot = var_guard1177;
        *var_guard1177_rv_slot = var_guard1177_rv;
        *var_inv_gf2_slot = var_inv_gf2;
        *var_inv_gf2_dn5_slot = var_inv_gf2_dn5;
        *var_inv_gf2_dn6_slot = var_inv_gf2_dn6;
        *var_inv_gf2_dn7_slot = var_inv_gf2_dn7;
        *var_inv_gf2_dn8_slot = var_inv_gf2_dn8;
        *var_inv_gf2_rv_slot = var_inv_gf2_rv;
        *var_inv_phit1_slot = var_inv_phit1;
        *var_inv_phit1_dn5_slot = var_inv_phit1_dn5;
        *var_inv_phit1_dn6_slot = var_inv_phit1_dn6;
        *var_inv_phit1_dn7_slot = var_inv_phit1_dn7;
        *var_inv_phit1_dn8_slot = var_inv_phit1_dn8;
        *var_inv_phit1_rv_slot = var_inv_phit1_rv;
        *var_nscr_slot = var_nscr;
        *var_nscr_dn5_slot = var_nscr_dn5;
        *var_nscr_dn6_slot = var_nscr_dn6;
        *var_nscr_dn7_slot = var_nscr_dn7;
        *var_nscr_dn8_slot = var_nscr_dn8;
        *var_nscr_rv_slot = var_nscr_rv;
        *var_phit1_slot = var_phit1;
        *var_phit1_dn5_slot = var_phit1_dn5;
        *var_phit1_dn6_slot = var_phit1_dn6;
        *var_phit1_dn7_slot = var_phit1_dn7;
        *var_phit1_dn8_slot = var_phit1_dn8;
        *var_phit1_rv_slot = var_phit1_rv;
        *var_phitct_slot = var_phitct;
        *var_phitct_dn5_slot = var_phitct_dn5;
        *var_phitct_dn6_slot = var_phitct_dn6;
        *var_phitct_dn7_slot = var_phitct_dn7;
        *var_phitct_dn8_slot = var_phitct_dn8;
        *var_phitct_rv_slot = var_phitct_rv;
        *var_temp1_slot = var_temp1;
        *var_temp1_dn5_slot = var_temp1_dn5;
        *var_temp1_dn6_slot = var_temp1_dn6;
        *var_temp1_dn7_slot = var_temp1_dn7;
        *var_temp1_dn8_slot = var_temp1_dn8;
        *var_temp1_rv_slot = var_temp1_rv;
        *var_temp2_slot = var_temp2;
        *var_temp2_dn5_slot = var_temp2_dn5;
        *var_temp2_dn6_slot = var_temp2_dn6;
        *var_temp2_dn7_slot = var_temp2_dn7;
        *var_temp2_dn8_slot = var_temp2_dn8;
        *var_temp2_rv_slot = var_temp2_rv;
        *var_temp__blk936_slot = var_temp__blk936;
        *var_temp__blk936_dn5_slot = var_temp__blk936_dn5;
        *var_temp__blk936_dn6_slot = var_temp__blk936_dn6;
        *var_temp__blk936_dn7_slot = var_temp__blk936_dn7;
        *var_temp__blk936_dn8_slot = var_temp__blk936_dn8;
        *var_temp__blk936_rv_slot = var_temp__blk936_rv;
        *var_ux_slot = var_ux;
        *var_ux_dn5_slot = var_ux_dn5;
        *var_ux_dn6_slot = var_ux_dn6;
        *var_ux_dn7_slot = var_ux_dn7;
        *var_ux_dn8_slot = var_ux_dn8;
        *var_ux_rv_slot = var_ux_rv;
        *var_vdsp_slot = var_vdsp;
        *var_vdsp_dn6_slot = var_vdsp_dn6;
        *var_vdsp_dn7_slot = var_vdsp_dn7;
        *var_vdsp_rv_slot = var_vdsp_rv;
        *var_xb_slot = var_xb;
        *var_xb_dn5_slot = var_xb_dn5;
        *var_xb_dn6_slot = var_xb_dn6;
        *var_xb_dn7_slot = var_xb_dn7;
        *var_xb_dn8_slot = var_xb_dn8;
        *var_xb_rv_slot = var_xb_rv;
        *var_xct_slot = var_xct;
        *var_xct_dn5_slot = var_xct_dn5;
        *var_xct_dn6_slot = var_xct_dn6;
        *var_xct_dn7_slot = var_xct_dn7;
        *var_xct_dn8_slot = var_xct_dn8;
        *var_xct_rv_slot = var_xct_rv;
        *var_xg_slot = var_xg;
        *var_xg_dn5_slot = var_xg_dn5;
        *var_xg_dn6_slot = var_xg_dn6;
        *var_xg_dn7_slot = var_xg_dn7;
        *var_xg_dn8_slot = var_xg_dn8;
        *var_xg_rv_slot = var_xg_rv;
        *var_xgtscr_slot = var_xgtscr;
        *var_xgtscr_dn5_slot = var_xgtscr_dn5;
        *var_xgtscr_dn6_slot = var_xgtscr_dn6;
        *var_xgtscr_dn7_slot = var_xgtscr_dn7;
        *var_xgtscr_dn8_slot = var_xgtscr_dn8;
        *var_xgtscr_rv_slot = var_xgtscr_rv;
        *var_xn_s_slot = var_xn_s;
        *var_xn_s_dn5_slot = var_xn_s_dn5;
        *var_xn_s_dn6_slot = var_xn_s_dn6;
        *var_xn_s_dn7_slot = var_xn_s_dn7;
        *var_xn_s_dn8_slot = var_xn_s_dn8;
        *var_xn_s_rv_slot = var_xn_s_rv;
        *var_xno_s_slot = var_xno_s;
        *var_xno_s_dn5_slot = var_xno_s_dn5;
        *var_xno_s_dn6_slot = var_xno_s_dn6;
        *var_xno_s_dn7_slot = var_xno_s_dn7;
        *var_xno_s_dn8_slot = var_xno_s_dn8;
        *var_xno_s_rv_slot = var_xno_s_rv;
        *var_xsubct_slot = var_xsubct;
        *var_xsubct_dn5_slot = var_xsubct_dn5;
        *var_xsubct_dn6_slot = var_xsubct_dn6;
        *var_xsubct_dn7_slot = var_xsubct_dn7;
        *var_xsubct_dn8_slot = var_xsubct_dn8;
        *var_xsubct_rv_slot = var_xsubct_rv;
        *var_xthscr_slot = var_xthscr;
        *var_xthscr_dn5_slot = var_xthscr_dn5;
        *var_xthscr_dn6_slot = var_xthscr_dn6;
        *var_xthscr_dn7_slot = var_xthscr_dn7;
        *var_xthscr_dn8_slot = var_xthscr_dn8;
        *var_xthscr_rv_slot = var_xthscr_rv;
    }

    pub(super) fn stamp_reactive_block_25(
        var_delxb: f64,
        var_delxb_dn5: f64,
        var_delxb_dn6: f64,
        var_delxb_dn7: f64,
        var_delxb_dn8: f64,
        var_gf: f64,
        var_gf2: f64,
        var_gf2_dn5: f64,
        var_gf2_dn6: f64,
        var_gf2_dn7: f64,
        var_gf2_dn8: f64,
        var_gf_dn5: f64,
        var_gf_dn6: f64,
        var_gf_dn7: f64,
        var_gf_dn8: f64,
        var_nscr: f64,
        var_nscr_dn5: f64,
        var_nscr_dn6: f64,
        var_nscr_dn7: f64,
        var_nscr_dn8: f64,
        var_xg: f64,
        var_xg_dn5: f64,
        var_xg_dn6: f64,
        var_xg_dn7: f64,
        var_xg_dn8: f64,
        var_xgtscr: f64,
        var_xgtscr_dn5: f64,
        var_xgtscr_dn6: f64,
        var_xgtscr_dn7: f64,
        var_xgtscr_dn8: f64,
        var_xno_s: f64,
        var_xno_s_dn5: f64,
        var_xno_s_dn6: f64,
        var_xno_s_dn7: f64,
        var_xno_s_dn8: f64,
        var_delta_ns_slot: &mut f64,
        var_delta_ns_dn5_slot: &mut f64,
        var_delta_ns_dn6_slot: &mut f64,
        var_delta_ns_dn7_slot: &mut f64,
        var_delta_ns_dn8_slot: &mut f64,
        var_delta_ns_rv_slot: &mut f64,
        var_dscr0_slot: &mut f64,
        var_dscr0_dn5_slot: &mut f64,
        var_dscr0_dn6_slot: &mut f64,
        var_dscr0_dn7_slot: &mut f64,
        var_dscr0_dn8_slot: &mut f64,
        var_dscr0_rv_slot: &mut f64,
        var_fscr_slot: &mut f64,
        var_fscr_dn5_slot: &mut f64,
        var_fscr_dn6_slot: &mut f64,
        var_fscr_dn7_slot: &mut f64,
        var_fscr_dn8_slot: &mut f64,
        var_fscr_rv_slot: &mut f64,
        var_guard1178_slot: &mut f64,
        var_guard1178_rv_slot: &mut f64,
        var_guard1179_slot: &mut f64,
        var_guard1179_rv_slot: &mut f64,
        var_guard1180_slot: &mut f64,
        var_guard1180_rv_slot: &mut f64,
        var_guard1181_slot: &mut f64,
        var_guard1181_rv_slot: &mut f64,
        var_guard1182_slot: &mut f64,
        var_guard1182_rv_slot: &mut f64,
        var_guard1183_slot: &mut f64,
        var_guard1183_rv_slot: &mut f64,
        var_inv_xi_slot: &mut f64,
        var_inv_xi_dn5_slot: &mut f64,
        var_inv_xi_dn6_slot: &mut f64,
        var_inv_xi_dn7_slot: &mut f64,
        var_inv_xi_dn8_slot: &mut f64,
        var_inv_xi_rv_slot: &mut f64,
        var_margin_slot: &mut f64,
        var_margin_dn5_slot: &mut f64,
        var_margin_dn6_slot: &mut f64,
        var_margin_dn7_slot: &mut f64,
        var_margin_dn8_slot: &mut f64,
        var_margin_rv_slot: &mut f64,
        var_qbscr_slot: &mut f64,
        var_qbscr_dn5_slot: &mut f64,
        var_qbscr_dn6_slot: &mut f64,
        var_qbscr_dn7_slot: &mut f64,
        var_qbscr_dn8_slot: &mut f64,
        var_qbscr_rv_slot: &mut f64,
        var_qiscr_slot: &mut f64,
        var_qiscr0_slot: &mut f64,
        var_qiscr0_dn5_slot: &mut f64,
        var_qiscr0_dn6_slot: &mut f64,
        var_qiscr0_dn7_slot: &mut f64,
        var_qiscr0_dn8_slot: &mut f64,
        var_qiscr0_rv_slot: &mut f64,
        var_qiscr0si_slot: &mut f64,
        var_qiscr0si_dn5_slot: &mut f64,
        var_qiscr0si_dn6_slot: &mut f64,
        var_qiscr0si_dn7_slot: &mut f64,
        var_qiscr0si_dn8_slot: &mut f64,
        var_qiscr0si_rv_slot: &mut f64,
        var_qiscr_dn5_slot: &mut f64,
        var_qiscr_dn6_slot: &mut f64,
        var_qiscr_dn7_slot: &mut f64,
        var_qiscr_dn8_slot: &mut f64,
        var_qiscr_rv_slot: &mut f64,
        var_sp_s_eta_slot: &mut f64,
        var_sp_s_eta_dn5_slot: &mut f64,
        var_sp_s_eta_dn6_slot: &mut f64,
        var_sp_s_eta_dn7_slot: &mut f64,
        var_sp_s_eta_dn8_slot: &mut f64,
        var_sp_s_eta_rv_slot: &mut f64,
        var_sp_s_temp_slot: &mut f64,
        var_sp_s_temp1_slot: &mut f64,
        var_sp_s_temp1_dn5_slot: &mut f64,
        var_sp_s_temp1_dn6_slot: &mut f64,
        var_sp_s_temp1_dn7_slot: &mut f64,
        var_sp_s_temp1_dn8_slot: &mut f64,
        var_sp_s_temp1_rv_slot: &mut f64,
        var_sp_s_temp_dn5_slot: &mut f64,
        var_sp_s_temp_dn6_slot: &mut f64,
        var_sp_s_temp_dn7_slot: &mut f64,
        var_sp_s_temp_dn8_slot: &mut f64,
        var_sp_s_temp_rv_slot: &mut f64,
        var_sp_s_x1_slot: &mut f64,
        var_sp_s_x1_dn5_slot: &mut f64,
        var_sp_s_x1_dn6_slot: &mut f64,
        var_sp_s_x1_dn7_slot: &mut f64,
        var_sp_s_x1_dn8_slot: &mut f64,
        var_sp_s_x1_rv_slot: &mut f64,
        var_sp_s_yg_slot: &mut f64,
        var_sp_s_yg_dn5_slot: &mut f64,
        var_sp_s_yg_dn6_slot: &mut f64,
        var_sp_s_yg_dn7_slot: &mut f64,
        var_sp_s_yg_dn8_slot: &mut f64,
        var_sp_s_yg_rv_slot: &mut f64,
        var_sp_s_ysub_slot: &mut f64,
        var_sp_s_ysub_dn5_slot: &mut f64,
        var_sp_s_ysub_dn6_slot: &mut f64,
        var_sp_s_ysub_dn7_slot: &mut f64,
        var_sp_s_ysub_dn8_slot: &mut f64,
        var_sp_s_ysub_rv_slot: &mut f64,
        var_temp__blk936_slot: &mut f64,
        var_temp__blk936_dn5_slot: &mut f64,
        var_temp__blk936_dn6_slot: &mut f64,
        var_temp__blk936_dn7_slot: &mut f64,
        var_temp__blk936_dn8_slot: &mut f64,
        var_temp__blk936_rv_slot: &mut f64,
        var_x_s_slot: &mut f64,
        var_x_s_dn5_slot: &mut f64,
        var_x_s_dn6_slot: &mut f64,
        var_x_s_dn7_slot: &mut f64,
        var_x_s_dn8_slot: &mut f64,
        var_x_s_rv_slot: &mut f64,
        var_xgtscr0_slot: &mut f64,
        var_xgtscr0_dn5_slot: &mut f64,
        var_xgtscr0_dn6_slot: &mut f64,
        var_xgtscr0_dn7_slot: &mut f64,
        var_xgtscr0_dn8_slot: &mut f64,
        var_xgtscr0_rv_slot: &mut f64,
        var_xi_slot: &mut f64,
        var_xi_dn5_slot: &mut f64,
        var_xi_dn6_slot: &mut f64,
        var_xi_dn7_slot: &mut f64,
        var_xi_dn8_slot: &mut f64,
        var_xi_rv_slot: &mut f64,
        var_xn_s_slot: &mut f64,
        var_xn_s_dn5_slot: &mut f64,
        var_xn_s_dn6_slot: &mut f64,
        var_xn_s_dn7_slot: &mut f64,
        var_xn_s_dn8_slot: &mut f64,
        var_xn_s_rv_slot: &mut f64,
    ) {
        let mut var_delta_ns: f64 = *var_delta_ns_slot;
        let mut var_delta_ns_dn5: f64 = *var_delta_ns_dn5_slot;
        let mut var_delta_ns_dn6: f64 = *var_delta_ns_dn6_slot;
        let mut var_delta_ns_dn7: f64 = *var_delta_ns_dn7_slot;
        let mut var_delta_ns_dn8: f64 = *var_delta_ns_dn8_slot;
        let mut var_delta_ns_rv: f64 = *var_delta_ns_rv_slot;
        let mut var_dscr0: f64 = *var_dscr0_slot;
        let mut var_dscr0_dn5: f64 = *var_dscr0_dn5_slot;
        let mut var_dscr0_dn6: f64 = *var_dscr0_dn6_slot;
        let mut var_dscr0_dn7: f64 = *var_dscr0_dn7_slot;
        let mut var_dscr0_dn8: f64 = *var_dscr0_dn8_slot;
        let mut var_dscr0_rv: f64 = *var_dscr0_rv_slot;
        let mut var_fscr: f64 = *var_fscr_slot;
        let mut var_fscr_dn5: f64 = *var_fscr_dn5_slot;
        let mut var_fscr_dn6: f64 = *var_fscr_dn6_slot;
        let mut var_fscr_dn7: f64 = *var_fscr_dn7_slot;
        let mut var_fscr_dn8: f64 = *var_fscr_dn8_slot;
        let mut var_fscr_rv: f64 = *var_fscr_rv_slot;
        let mut var_guard1178: f64 = *var_guard1178_slot;
        let mut var_guard1178_rv: f64 = *var_guard1178_rv_slot;
        let mut var_guard1179: f64 = *var_guard1179_slot;
        let mut var_guard1179_rv: f64 = *var_guard1179_rv_slot;
        let mut var_guard1180: f64 = *var_guard1180_slot;
        let mut var_guard1180_rv: f64 = *var_guard1180_rv_slot;
        let mut var_guard1181: f64 = *var_guard1181_slot;
        let mut var_guard1181_rv: f64 = *var_guard1181_rv_slot;
        let mut var_guard1182: f64 = *var_guard1182_slot;
        let mut var_guard1182_rv: f64 = *var_guard1182_rv_slot;
        let mut var_guard1183: f64 = *var_guard1183_slot;
        let mut var_guard1183_rv: f64 = *var_guard1183_rv_slot;
        let mut var_inv_xi: f64 = *var_inv_xi_slot;
        let mut var_inv_xi_dn5: f64 = *var_inv_xi_dn5_slot;
        let mut var_inv_xi_dn6: f64 = *var_inv_xi_dn6_slot;
        let mut var_inv_xi_dn7: f64 = *var_inv_xi_dn7_slot;
        let mut var_inv_xi_dn8: f64 = *var_inv_xi_dn8_slot;
        let mut var_inv_xi_rv: f64 = *var_inv_xi_rv_slot;
        let mut var_margin: f64 = *var_margin_slot;
        let mut var_margin_dn5: f64 = *var_margin_dn5_slot;
        let mut var_margin_dn6: f64 = *var_margin_dn6_slot;
        let mut var_margin_dn7: f64 = *var_margin_dn7_slot;
        let mut var_margin_dn8: f64 = *var_margin_dn8_slot;
        let mut var_margin_rv: f64 = *var_margin_rv_slot;
        let mut var_qbscr: f64 = *var_qbscr_slot;
        let mut var_qbscr_dn5: f64 = *var_qbscr_dn5_slot;
        let mut var_qbscr_dn6: f64 = *var_qbscr_dn6_slot;
        let mut var_qbscr_dn7: f64 = *var_qbscr_dn7_slot;
        let mut var_qbscr_dn8: f64 = *var_qbscr_dn8_slot;
        let mut var_qbscr_rv: f64 = *var_qbscr_rv_slot;
        let mut var_qiscr: f64 = *var_qiscr_slot;
        let mut var_qiscr0: f64 = *var_qiscr0_slot;
        let mut var_qiscr0_dn5: f64 = *var_qiscr0_dn5_slot;
        let mut var_qiscr0_dn6: f64 = *var_qiscr0_dn6_slot;
        let mut var_qiscr0_dn7: f64 = *var_qiscr0_dn7_slot;
        let mut var_qiscr0_dn8: f64 = *var_qiscr0_dn8_slot;
        let mut var_qiscr0_rv: f64 = *var_qiscr0_rv_slot;
        let mut var_qiscr0si: f64 = *var_qiscr0si_slot;
        let mut var_qiscr0si_dn5: f64 = *var_qiscr0si_dn5_slot;
        let mut var_qiscr0si_dn6: f64 = *var_qiscr0si_dn6_slot;
        let mut var_qiscr0si_dn7: f64 = *var_qiscr0si_dn7_slot;
        let mut var_qiscr0si_dn8: f64 = *var_qiscr0si_dn8_slot;
        let mut var_qiscr0si_rv: f64 = *var_qiscr0si_rv_slot;
        let mut var_qiscr_dn5: f64 = *var_qiscr_dn5_slot;
        let mut var_qiscr_dn6: f64 = *var_qiscr_dn6_slot;
        let mut var_qiscr_dn7: f64 = *var_qiscr_dn7_slot;
        let mut var_qiscr_dn8: f64 = *var_qiscr_dn8_slot;
        let mut var_qiscr_rv: f64 = *var_qiscr_rv_slot;
        let mut var_sp_s_eta: f64 = *var_sp_s_eta_slot;
        let mut var_sp_s_eta_dn5: f64 = *var_sp_s_eta_dn5_slot;
        let mut var_sp_s_eta_dn6: f64 = *var_sp_s_eta_dn6_slot;
        let mut var_sp_s_eta_dn7: f64 = *var_sp_s_eta_dn7_slot;
        let mut var_sp_s_eta_dn8: f64 = *var_sp_s_eta_dn8_slot;
        let mut var_sp_s_eta_rv: f64 = *var_sp_s_eta_rv_slot;
        let mut var_sp_s_temp: f64 = *var_sp_s_temp_slot;
        let mut var_sp_s_temp1: f64 = *var_sp_s_temp1_slot;
        let mut var_sp_s_temp1_dn5: f64 = *var_sp_s_temp1_dn5_slot;
        let mut var_sp_s_temp1_dn6: f64 = *var_sp_s_temp1_dn6_slot;
        let mut var_sp_s_temp1_dn7: f64 = *var_sp_s_temp1_dn7_slot;
        let mut var_sp_s_temp1_dn8: f64 = *var_sp_s_temp1_dn8_slot;
        let mut var_sp_s_temp1_rv: f64 = *var_sp_s_temp1_rv_slot;
        let mut var_sp_s_temp_dn5: f64 = *var_sp_s_temp_dn5_slot;
        let mut var_sp_s_temp_dn6: f64 = *var_sp_s_temp_dn6_slot;
        let mut var_sp_s_temp_dn7: f64 = *var_sp_s_temp_dn7_slot;
        let mut var_sp_s_temp_dn8: f64 = *var_sp_s_temp_dn8_slot;
        let mut var_sp_s_temp_rv: f64 = *var_sp_s_temp_rv_slot;
        let mut var_sp_s_x1: f64 = *var_sp_s_x1_slot;
        let mut var_sp_s_x1_dn5: f64 = *var_sp_s_x1_dn5_slot;
        let mut var_sp_s_x1_dn6: f64 = *var_sp_s_x1_dn6_slot;
        let mut var_sp_s_x1_dn7: f64 = *var_sp_s_x1_dn7_slot;
        let mut var_sp_s_x1_dn8: f64 = *var_sp_s_x1_dn8_slot;
        let mut var_sp_s_x1_rv: f64 = *var_sp_s_x1_rv_slot;
        let mut var_sp_s_yg: f64 = *var_sp_s_yg_slot;
        let mut var_sp_s_yg_dn5: f64 = *var_sp_s_yg_dn5_slot;
        let mut var_sp_s_yg_dn6: f64 = *var_sp_s_yg_dn6_slot;
        let mut var_sp_s_yg_dn7: f64 = *var_sp_s_yg_dn7_slot;
        let mut var_sp_s_yg_dn8: f64 = *var_sp_s_yg_dn8_slot;
        let mut var_sp_s_yg_rv: f64 = *var_sp_s_yg_rv_slot;
        let mut var_sp_s_ysub: f64 = *var_sp_s_ysub_slot;
        let mut var_sp_s_ysub_dn5: f64 = *var_sp_s_ysub_dn5_slot;
        let mut var_sp_s_ysub_dn6: f64 = *var_sp_s_ysub_dn6_slot;
        let mut var_sp_s_ysub_dn7: f64 = *var_sp_s_ysub_dn7_slot;
        let mut var_sp_s_ysub_dn8: f64 = *var_sp_s_ysub_dn8_slot;
        let mut var_sp_s_ysub_rv: f64 = *var_sp_s_ysub_rv_slot;
        let mut var_temp__blk936: f64 = *var_temp__blk936_slot;
        let mut var_temp__blk936_dn5: f64 = *var_temp__blk936_dn5_slot;
        let mut var_temp__blk936_dn6: f64 = *var_temp__blk936_dn6_slot;
        let mut var_temp__blk936_dn7: f64 = *var_temp__blk936_dn7_slot;
        let mut var_temp__blk936_dn8: f64 = *var_temp__blk936_dn8_slot;
        let mut var_temp__blk936_rv: f64 = *var_temp__blk936_rv_slot;
        let mut var_x_s: f64 = *var_x_s_slot;
        let mut var_x_s_dn5: f64 = *var_x_s_dn5_slot;
        let mut var_x_s_dn6: f64 = *var_x_s_dn6_slot;
        let mut var_x_s_dn7: f64 = *var_x_s_dn7_slot;
        let mut var_x_s_dn8: f64 = *var_x_s_dn8_slot;
        let mut var_x_s_rv: f64 = *var_x_s_rv_slot;
        let mut var_xgtscr0: f64 = *var_xgtscr0_slot;
        let mut var_xgtscr0_dn5: f64 = *var_xgtscr0_dn5_slot;
        let mut var_xgtscr0_dn6: f64 = *var_xgtscr0_dn6_slot;
        let mut var_xgtscr0_dn7: f64 = *var_xgtscr0_dn7_slot;
        let mut var_xgtscr0_dn8: f64 = *var_xgtscr0_dn8_slot;
        let mut var_xgtscr0_rv: f64 = *var_xgtscr0_rv_slot;
        let mut var_xi: f64 = *var_xi_slot;
        let mut var_xi_dn5: f64 = *var_xi_dn5_slot;
        let mut var_xi_dn6: f64 = *var_xi_dn6_slot;
        let mut var_xi_dn7: f64 = *var_xi_dn7_slot;
        let mut var_xi_dn8: f64 = *var_xi_dn8_slot;
        let mut var_xi_rv: f64 = *var_xi_rv_slot;
        let mut var_xn_s: f64 = *var_xn_s_slot;
        let mut var_xn_s_dn5: f64 = *var_xn_s_dn5_slot;
        let mut var_xn_s_dn6: f64 = *var_xn_s_dn6_slot;
        let mut var_xn_s_dn7: f64 = *var_xn_s_dn7_slot;
        let mut var_xn_s_dn8: f64 = *var_xn_s_dn8_slot;
        let mut var_xn_s_rv: f64 = *var_xn_s_rv_slot;

        let assign41310_e54213: f64 = (0.5 * var_gf2);
        let assign41310_e54217: f64 = (8.0 / var_gf2);
        let assign41310_e54218: f64 = (1.0 + assign41310_e54217);
        let assign41310_e54219: f64 = (assign41310_e54218).sqrt();
        let assign41310_e54221: f64 = (assign41310_e54219 - 1.0);
        let assign41310_e54222: f64 = (assign41310_e54213 * assign41310_e54221);
        var_qbscr = assign41310_e54222;
        var_qbscr_dn5 = (((0.5 * var_gf2_dn5) * assign41310_e54221) + (assign41310_e54213 * ((-((8.0 * var_gf2_dn5) / (var_gf2 * var_gf2))) / (2.0 * assign41310_e54219))));
        var_qbscr_dn6 = (((0.5 * var_gf2_dn6) * assign41310_e54221) + (assign41310_e54213 * ((-((8.0 * var_gf2_dn6) / (var_gf2 * var_gf2))) / (2.0 * assign41310_e54219))));
        var_qbscr_dn7 = (((0.5 * var_gf2_dn7) * assign41310_e54221) + (assign41310_e54213 * ((-((8.0 * var_gf2_dn7) / (var_gf2 * var_gf2))) / (2.0 * assign41310_e54219))));
        var_qbscr_dn8 = (((0.5 * var_gf2_dn8) * assign41310_e54221) + (assign41310_e54213 * ((-((8.0 * var_gf2_dn8) / (var_gf2 * var_gf2))) / (2.0 * assign41310_e54219))));
        var_qbscr_rv = 0.0;

        var_qiscr = 0.0;
        var_qiscr_dn5 = 0.0;
        var_qiscr_dn6 = 0.0;
        var_qiscr_dn7 = 0.0;
        var_qiscr_dn8 = 0.0;
        var_qiscr_rv = 0.0;

        var_fscr = 1.0;
        var_fscr_dn5 = 0.0;
        var_fscr_dn6 = 0.0;
        var_fscr_dn7 = 0.0;
        var_fscr_dn8 = 0.0;
        var_fscr_rv = 0.0;

        let assign41340_e54227: f64 = (-30.0);
        let assign41340_e54228: f64 = if var_xgtscr > assign41340_e54227 { 1.0 } else { 0.0 };
        var_guard1178 = assign41340_e54228;
        var_guard1178_rv = 0.0;

        let (assign41350_e54236, assign41350_e54236_d_n5, assign41350_e54236_d_n6, assign41350_e54236_d_n7, assign41350_e54236_d_n8,) = {
    if (var_guard1178 != 0.0) {
        let assign41350_e54232: f64 = (var_nscr * var_xgtscr);
        let assign41350_e54234: f64 = (assign41350_e54232 - 1.0);
        (assign41350_e54234, ((var_nscr_dn5 * var_xgtscr) + (var_nscr * var_xgtscr_dn5)), ((var_nscr_dn6 * var_xgtscr) + (var_nscr * var_xgtscr_dn6)), ((var_nscr_dn7 * var_xgtscr) + (var_nscr * var_xgtscr_dn7)), ((var_nscr_dn8 * var_xgtscr) + (var_nscr * var_xgtscr_dn8)),)
    } else {
        (var_xgtscr0, var_xgtscr0_dn5, var_xgtscr0_dn6, var_xgtscr0_dn7, var_xgtscr0_dn8,)
    }
};
        var_xgtscr0 = assign41350_e54236;
        var_xgtscr0_dn5 = assign41350_e54236_d_n5;
        var_xgtscr0_dn6 = assign41350_e54236_d_n6;
        var_xgtscr0_dn7 = assign41350_e54236_d_n7;
        var_xgtscr0_dn8 = assign41350_e54236_d_n8;
        var_xgtscr0_rv = 0.0;

        let (assign41360_e54249, assign41360_e54249_d_n5, assign41360_e54249_d_n6, assign41360_e54249_d_n7, assign41360_e54249_d_n8,) = {
    if (var_guard1178 != 0.0) {
        let assign41360_e54242: f64 = (var_xgtscr0 * var_xgtscr0);
        let assign41360_e54244: f64 = (assign41360_e54242 + 10.0);
        let assign41360_e54245: f64 = (assign41360_e54244).sqrt();
        let assign41360_e54246: f64 = (var_xgtscr0 + assign41360_e54245);
        let assign41360_e54247: f64 = (0.5 * assign41360_e54246);
        (assign41360_e54247, (0.5 * (var_xgtscr0_dn5 + (((var_xgtscr0_dn5 * var_xgtscr0) + (var_xgtscr0 * var_xgtscr0_dn5)) / (2.0 * assign41360_e54245)))), (0.5 * (var_xgtscr0_dn6 + (((var_xgtscr0_dn6 * var_xgtscr0) + (var_xgtscr0 * var_xgtscr0_dn6)) / (2.0 * assign41360_e54245)))), (0.5 * (var_xgtscr0_dn7 + (((var_xgtscr0_dn7 * var_xgtscr0) + (var_xgtscr0 * var_xgtscr0_dn7)) / (2.0 * assign41360_e54245)))), (0.5 * (var_xgtscr0_dn8 + (((var_xgtscr0_dn8 * var_xgtscr0) + (var_xgtscr0 * var_xgtscr0_dn8)) / (2.0 * assign41360_e54245)))),)
    } else {
        (var_temp__blk936, var_temp__blk936_dn5, var_temp__blk936_dn6, var_temp__blk936_dn7, var_temp__blk936_dn8,)
    }
};
        var_temp__blk936 = assign41360_e54249;
        var_temp__blk936_dn5 = assign41360_e54249_d_n5;
        var_temp__blk936_dn6 = assign41360_e54249_d_n6;
        var_temp__blk936_dn7 = assign41360_e54249_d_n7;
        var_temp__blk936_dn8 = assign41360_e54249_d_n8;
        var_temp__blk936_rv = 0.0;

        let (assign41370_e54256, assign41370_e54256_d_n5, assign41370_e54256_d_n6, assign41370_e54256_d_n7, assign41370_e54256_d_n8,) = {
    if (var_guard1178 != 0.0) {
        let assign41370_e54253: f64 = (var_temp__blk936).ln();
        let assign41370_e54254: f64 = (var_xgtscr - assign41370_e54253);
        (assign41370_e54254, (var_xgtscr_dn5 - (var_temp__blk936_dn5 / var_temp__blk936)), (var_xgtscr_dn6 - (var_temp__blk936_dn6 / var_temp__blk936)), (var_xgtscr_dn7 - (var_temp__blk936_dn7 / var_temp__blk936)), (var_xgtscr_dn8 - (var_temp__blk936_dn8 / var_temp__blk936)),)
    } else {
        (var_qiscr0si, var_qiscr0si_dn5, var_qiscr0si_dn6, var_qiscr0si_dn7, var_qiscr0si_dn8,)
    }
};
        var_qiscr0si = assign41370_e54256;
        var_qiscr0si_dn5 = assign41370_e54256_d_n5;
        var_qiscr0si_dn6 = assign41370_e54256_d_n6;
        var_qiscr0si_dn7 = assign41370_e54256_d_n7;
        var_qiscr0si_dn8 = assign41370_e54256_d_n8;
        var_qiscr0si_rv = 0.0;

        let (assign41380_e54269, assign41380_e54269_d_n5, assign41380_e54269_d_n6, assign41380_e54269_d_n7, assign41380_e54269_d_n8,) = {
    if (var_guard1178 != 0.0) {
        let assign41380_e54262: f64 = (var_qiscr0si * var_qiscr0si);
        let assign41380_e54264: f64 = (assign41380_e54262 + 2.0);
        let assign41380_e54265: f64 = (assign41380_e54264).sqrt();
        let assign41380_e54266: f64 = (var_qiscr0si + assign41380_e54265);
        let assign41380_e54267: f64 = (0.5 * assign41380_e54266);
        (assign41380_e54267, (0.5 * (var_qiscr0si_dn5 + (((var_qiscr0si_dn5 * var_qiscr0si) + (var_qiscr0si * var_qiscr0si_dn5)) / (2.0 * assign41380_e54265)))), (0.5 * (var_qiscr0si_dn6 + (((var_qiscr0si_dn6 * var_qiscr0si) + (var_qiscr0si * var_qiscr0si_dn6)) / (2.0 * assign41380_e54265)))), (0.5 * (var_qiscr0si_dn7 + (((var_qiscr0si_dn7 * var_qiscr0si) + (var_qiscr0si * var_qiscr0si_dn7)) / (2.0 * assign41380_e54265)))), (0.5 * (var_qiscr0si_dn8 + (((var_qiscr0si_dn8 * var_qiscr0si) + (var_qiscr0si * var_qiscr0si_dn8)) / (2.0 * assign41380_e54265)))),)
    } else {
        (var_qiscr0, var_qiscr0_dn5, var_qiscr0_dn6, var_qiscr0_dn7, var_qiscr0_dn8,)
    }
};
        var_qiscr0 = assign41380_e54269;
        var_qiscr0_dn5 = assign41380_e54269_d_n5;
        var_qiscr0_dn6 = assign41380_e54269_d_n6;
        var_qiscr0_dn7 = assign41380_e54269_d_n7;
        var_qiscr0_dn8 = assign41380_e54269_d_n8;
        var_qiscr0_rv = 0.0;

        let assign41390_e54272: f64 = (var_xgtscr - var_qiscr0);
        let assign41390_e54274: f64 = if assign41390_e54272 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard1179 = assign41390_e54274;
        var_guard1179_rv = 0.0;

        let (assign41400_e54283, assign41400_e54283_d_n5, assign41400_e54283_d_n6, assign41400_e54283_d_n7, assign41400_e54283_d_n8,) = {
    if ((var_guard1178 != 0.0) && (var_guard1179 != 0.0)) {
        let assign41400_e54280: f64 = (var_xgtscr - var_qiscr0);
        let assign41400_e54281: f64 = (assign41400_e54280).exp();
        (assign41400_e54281, (assign41400_e54281 * (var_xgtscr_dn5 - var_qiscr0_dn5)), (assign41400_e54281 * (var_xgtscr_dn6 - var_qiscr0_dn6)), (assign41400_e54281 * (var_xgtscr_dn7 - var_qiscr0_dn7)), (assign41400_e54281 * (var_xgtscr_dn8 - var_qiscr0_dn8)),)
    } else {
        (var_temp__blk936, var_temp__blk936_dn5, var_temp__blk936_dn6, var_temp__blk936_dn7, var_temp__blk936_dn8,)
    }
};
        var_temp__blk936 = assign41400_e54283;
        var_temp__blk936_dn5 = assign41400_e54283_d_n5;
        var_temp__blk936_dn6 = assign41400_e54283_d_n6;
        var_temp__blk936_dn7 = assign41400_e54283_d_n7;
        var_temp__blk936_dn8 = assign41400_e54283_d_n8;
        var_temp__blk936_rv = 0.0;

        let (assign41410_e54318, assign41410_e54318_d_n5, assign41410_e54318_d_n6, assign41410_e54318_d_n7, assign41410_e54318_d_n8,) = {
    if ((var_guard1178 != 0.0) && (var_guard1179 == 0.0)) {
        let assign41410_e54292: f64 = (var_xgtscr - var_qiscr0);
        let assign41410_e54294: f64 = (assign41410_e54292 - 230.25850929940458);
        let assign41410_e54299: f64 = (var_xgtscr - var_qiscr0);
        let assign41410_e54301: f64 = (assign41410_e54299 - 230.25850929940458);
        let assign41410_e54305: f64 = (var_xgtscr - var_qiscr0);
        let assign41410_e54307: f64 = (assign41410_e54305 - 230.25850929940458);
        let assign41410_e54309: f64 = (assign41410_e54307 * 0.3333333333333333);
        let assign41410_e54310: f64 = (1.0 + assign41410_e54309);
        let assign41410_e54311: f64 = (assign41410_e54301 * assign41410_e54310);
        let assign41410_e54312: f64 = (0.5 * assign41410_e54311);
        let assign41410_e54313: f64 = (1.0 + assign41410_e54312);
        let assign41410_e54314: f64 = (assign41410_e54294 * assign41410_e54313);
        let assign41410_e54315: f64 = (1.0 + assign41410_e54314);
        let assign41410_e54316: f64 = (1e100 * assign41410_e54315);
        (assign41410_e54316, (1e100 * (((var_xgtscr_dn5 - var_qiscr0_dn5) * assign41410_e54313) + (assign41410_e54294 * (0.5 * (((var_xgtscr_dn5 - var_qiscr0_dn5) * assign41410_e54310) + (assign41410_e54301 * ((var_xgtscr_dn5 - var_qiscr0_dn5) * 0.3333333333333333))))))), (1e100 * (((var_xgtscr_dn6 - var_qiscr0_dn6) * assign41410_e54313) + (assign41410_e54294 * (0.5 * (((var_xgtscr_dn6 - var_qiscr0_dn6) * assign41410_e54310) + (assign41410_e54301 * ((var_xgtscr_dn6 - var_qiscr0_dn6) * 0.3333333333333333))))))), (1e100 * (((var_xgtscr_dn7 - var_qiscr0_dn7) * assign41410_e54313) + (assign41410_e54294 * (0.5 * (((var_xgtscr_dn7 - var_qiscr0_dn7) * assign41410_e54310) + (assign41410_e54301 * ((var_xgtscr_dn7 - var_qiscr0_dn7) * 0.3333333333333333))))))), (1e100 * (((var_xgtscr_dn8 - var_qiscr0_dn8) * assign41410_e54313) + (assign41410_e54294 * (0.5 * (((var_xgtscr_dn8 - var_qiscr0_dn8) * assign41410_e54310) + (assign41410_e54301 * ((var_xgtscr_dn8 - var_qiscr0_dn8) * 0.3333333333333333))))))),)
    } else {
        (var_temp__blk936, var_temp__blk936_dn5, var_temp__blk936_dn6, var_temp__blk936_dn7, var_temp__blk936_dn8,)
    }
};
        var_temp__blk936 = assign41410_e54318;
        var_temp__blk936_dn5 = assign41410_e54318_d_n5;
        var_temp__blk936_dn6 = assign41410_e54318_d_n6;
        var_temp__blk936_dn7 = assign41410_e54318_d_n7;
        var_temp__blk936_dn8 = assign41410_e54318_d_n8;
        var_temp__blk936_rv = 0.0;

        let (assign41420_e54324, assign41420_e54324_d_n5, assign41420_e54324_d_n6, assign41420_e54324_d_n7, assign41420_e54324_d_n8,) = {
    if (var_guard1178 != 0.0) {
        let assign41420_e54322: f64 = (var_temp__blk936 / var_nscr);
        (assign41420_e54322, (((var_temp__blk936_dn5 * var_nscr) - (var_temp__blk936 * var_nscr_dn5)) / (var_nscr * var_nscr)), (((var_temp__blk936_dn6 * var_nscr) - (var_temp__blk936 * var_nscr_dn6)) / (var_nscr * var_nscr)), (((var_temp__blk936_dn7 * var_nscr) - (var_temp__blk936 * var_nscr_dn7)) / (var_nscr * var_nscr)), (((var_temp__blk936_dn8 * var_nscr) - (var_temp__blk936 * var_nscr_dn8)) / (var_nscr * var_nscr)),)
    } else {
        (var_dscr0, var_dscr0_dn5, var_dscr0_dn6, var_dscr0_dn7, var_dscr0_dn8,)
    }
};
        var_dscr0 = assign41420_e54324;
        var_dscr0_dn5 = assign41420_e54324_d_n5;
        var_dscr0_dn6 = assign41420_e54324_d_n6;
        var_dscr0_dn7 = assign41420_e54324_d_n7;
        var_dscr0_dn8 = assign41420_e54324_d_n8;
        var_dscr0_rv = 0.0;

        let (assign41430_e54334, assign41430_e54334_d_n5, assign41430_e54334_d_n6, assign41430_e54334_d_n7, assign41430_e54334_d_n8,) = {
    if (var_guard1178 != 0.0) {
        let assign41430_e54329: f64 = (var_qiscr0 + 1.0);
        let assign41430_e54330: f64 = (2.0 * assign41430_e54329);
        let assign41430_e54332: f64 = (assign41430_e54330 - var_dscr0);
        (assign41430_e54332, ((2.0 * var_qiscr0_dn5) - var_dscr0_dn5), ((2.0 * var_qiscr0_dn6) - var_dscr0_dn6), ((2.0 * var_qiscr0_dn7) - var_dscr0_dn7), ((2.0 * var_qiscr0_dn8) - var_dscr0_dn8),)
    } else {
        (var_temp__blk936, var_temp__blk936_dn5, var_temp__blk936_dn6, var_temp__blk936_dn7, var_temp__blk936_dn8,)
    }
};
        var_temp__blk936 = assign41430_e54334;
        var_temp__blk936_dn5 = assign41430_e54334_d_n5;
        var_temp__blk936_dn6 = assign41430_e54334_d_n6;
        var_temp__blk936_dn7 = assign41430_e54334_d_n7;
        var_temp__blk936_dn8 = assign41430_e54334_d_n8;
        var_temp__blk936_rv = 0.0;

        let assign41440_e54337: f64 = if var_dscr0 > 1e-6 { 1.0 } else { 0.0 };
        var_guard1180 = assign41440_e54337;
        var_guard1180_rv = 0.0;

        let (assign41450_e54358, assign41450_e54358_d_n5, assign41450_e54358_d_n6, assign41450_e54358_d_n7, assign41450_e54358_d_n8,) = {
    if ((var_guard1178 != 0.0) && (var_guard1180 != 0.0)) {
        let assign41450_e54346: f64 = (var_dscr0 * var_temp__blk936);
        let assign41450_e54347: f64 = (1.0 + assign41450_e54346);
        let assign41450_e54348: f64 = (assign41450_e54347).sqrt();
        let assign41450_e54350: f64 = (assign41450_e54348 - 1.0);
        let assign41450_e54352: f64 = (assign41450_e54350 / var_dscr0);
        let assign41450_e54353: f64 = (var_qiscr0 - assign41450_e54352);
        let assign41450_e54355: f64 = (assign41450_e54353 + 1.0);
        let assign41450_e54356: f64 = (var_nscr * assign41450_e54355);
        (assign41450_e54356, ((var_nscr_dn5 * assign41450_e54355) + (var_nscr * (var_qiscr0_dn5 - ((((((var_dscr0_dn5 * var_temp__blk936) + (var_dscr0 * var_temp__blk936_dn5)) / (2.0 * assign41450_e54348)) * var_dscr0) - (assign41450_e54350 * var_dscr0_dn5)) / (var_dscr0 * var_dscr0))))), ((var_nscr_dn6 * assign41450_e54355) + (var_nscr * (var_qiscr0_dn6 - ((((((var_dscr0_dn6 * var_temp__blk936) + (var_dscr0 * var_temp__blk936_dn6)) / (2.0 * assign41450_e54348)) * var_dscr0) - (assign41450_e54350 * var_dscr0_dn6)) / (var_dscr0 * var_dscr0))))), ((var_nscr_dn7 * assign41450_e54355) + (var_nscr * (var_qiscr0_dn7 - ((((((var_dscr0_dn7 * var_temp__blk936) + (var_dscr0 * var_temp__blk936_dn7)) / (2.0 * assign41450_e54348)) * var_dscr0) - (assign41450_e54350 * var_dscr0_dn7)) / (var_dscr0 * var_dscr0))))), ((var_nscr_dn8 * assign41450_e54355) + (var_nscr * (var_qiscr0_dn8 - ((((((var_dscr0_dn8 * var_temp__blk936) + (var_dscr0 * var_temp__blk936_dn8)) / (2.0 * assign41450_e54348)) * var_dscr0) - (assign41450_e54350 * var_dscr0_dn8)) / (var_dscr0 * var_dscr0))))),)
    } else {
        (var_qiscr, var_qiscr_dn5, var_qiscr_dn6, var_qiscr_dn7, var_qiscr_dn8,)
    }
};
        var_qiscr = assign41450_e54358;
        var_qiscr_dn5 = assign41450_e54358_d_n5;
        var_qiscr_dn6 = assign41450_e54358_d_n6;
        var_qiscr_dn7 = assign41450_e54358_d_n7;
        var_qiscr_dn8 = assign41450_e54358_d_n8;
        var_qiscr_rv = 0.0;

        let (assign41460_e54377, assign41460_e54377_d_n5, assign41460_e54377_d_n6, assign41460_e54377_d_n7, assign41460_e54377_d_n8,) = {
    if ((var_guard1178 != 0.0) && (var_guard1180 == 0.0)) {
        let assign41460_e54365: f64 = (var_nscr * 0.5);
        let assign41460_e54367: f64 = (assign41460_e54365 * var_dscr0);
        let assign41460_e54371: f64 = (0.25 * var_temp__blk936);
        let assign41460_e54373: f64 = (assign41460_e54371 * var_temp__blk936);
        let assign41460_e54374: f64 = (1.0 + assign41460_e54373);
        let assign41460_e54375: f64 = (assign41460_e54367 * assign41460_e54374);
        (assign41460_e54375, (((((var_nscr_dn5 * 0.5) * var_dscr0) + (assign41460_e54365 * var_dscr0_dn5)) * assign41460_e54374) + (assign41460_e54367 * (((0.25 * var_temp__blk936_dn5) * var_temp__blk936) + (assign41460_e54371 * var_temp__blk936_dn5)))), (((((var_nscr_dn6 * 0.5) * var_dscr0) + (assign41460_e54365 * var_dscr0_dn6)) * assign41460_e54374) + (assign41460_e54367 * (((0.25 * var_temp__blk936_dn6) * var_temp__blk936) + (assign41460_e54371 * var_temp__blk936_dn6)))), (((((var_nscr_dn7 * 0.5) * var_dscr0) + (assign41460_e54365 * var_dscr0_dn7)) * assign41460_e54374) + (assign41460_e54367 * (((0.25 * var_temp__blk936_dn7) * var_temp__blk936) + (assign41460_e54371 * var_temp__blk936_dn7)))), (((((var_nscr_dn8 * 0.5) * var_dscr0) + (assign41460_e54365 * var_dscr0_dn8)) * assign41460_e54374) + (assign41460_e54367 * (((0.25 * var_temp__blk936_dn8) * var_temp__blk936) + (assign41460_e54371 * var_temp__blk936_dn8)))),)
    } else {
        (var_qiscr, var_qiscr_dn5, var_qiscr_dn6, var_qiscr_dn7, var_qiscr_dn8,)
    }
};
        var_qiscr = assign41460_e54377;
        var_qiscr_dn5 = assign41460_e54377_d_n5;
        var_qiscr_dn6 = assign41460_e54377_d_n6;
        var_qiscr_dn7 = assign41460_e54377_d_n7;
        var_qiscr_dn8 = assign41460_e54377_d_n8;
        var_qiscr_rv = 0.0;

        let (assign41470_e54402, assign41470_e54402_d_n5, assign41470_e54402_d_n6, assign41470_e54402_d_n7, assign41470_e54402_d_n8,) = {
    if (var_guard1178 != 0.0) {
        let assign41470_e54382: f64 = (var_xg - var_qiscr);
        let assign41470_e54384: f64 = (assign41470_e54382 + 2.0);
        let assign41470_e54387: f64 = (var_xg - var_qiscr);
        let assign41470_e54389: f64 = (assign41470_e54387 - 2.0);
        let assign41470_e54392: f64 = (var_xg - var_qiscr);
        let assign41470_e54394: f64 = (assign41470_e54392 - 2.0);
        let assign41470_e54395: f64 = (assign41470_e54389 * assign41470_e54394);
        let assign41470_e54397: f64 = (assign41470_e54395 + 1.0);
        let assign41470_e54398: f64 = (assign41470_e54397).sqrt();
        let assign41470_e54399: f64 = (assign41470_e54384 + assign41470_e54398);
        let assign41470_e54400: f64 = (0.5 * assign41470_e54399);
        (assign41470_e54400, (0.5 * ((var_xg_dn5 - var_qiscr_dn5) + ((((var_xg_dn5 - var_qiscr_dn5) * assign41470_e54394) + (assign41470_e54389 * (var_xg_dn5 - var_qiscr_dn5))) / (2.0 * assign41470_e54398)))), (0.5 * ((var_xg_dn6 - var_qiscr_dn6) + ((((var_xg_dn6 - var_qiscr_dn6) * assign41470_e54394) + (assign41470_e54389 * (var_xg_dn6 - var_qiscr_dn6))) / (2.0 * assign41470_e54398)))), (0.5 * ((var_xg_dn7 - var_qiscr_dn7) + ((((var_xg_dn7 - var_qiscr_dn7) * assign41470_e54394) + (assign41470_e54389 * (var_xg_dn7 - var_qiscr_dn7))) / (2.0 * assign41470_e54398)))), (0.5 * ((var_xg_dn8 - var_qiscr_dn8) + ((((var_xg_dn8 - var_qiscr_dn8) * assign41470_e54394) + (assign41470_e54389 * (var_xg_dn8 - var_qiscr_dn8))) / (2.0 * assign41470_e54398)))),)
    } else {
        (var_temp__blk936, var_temp__blk936_dn5, var_temp__blk936_dn6, var_temp__blk936_dn7, var_temp__blk936_dn8,)
    }
};
        var_temp__blk936 = assign41470_e54402;
        var_temp__blk936_dn5 = assign41470_e54402_d_n5;
        var_temp__blk936_dn6 = assign41470_e54402_d_n6;
        var_temp__blk936_dn7 = assign41470_e54402_d_n7;
        var_temp__blk936_dn8 = assign41470_e54402_d_n8;
        var_temp__blk936_rv = 0.0;

        let (assign41480_e54419, assign41480_e54419_d_n5, assign41480_e54419_d_n6, assign41480_e54419_d_n7, assign41480_e54419_d_n8,) = {
    if (var_guard1178 != 0.0) {
        let assign41480_e54406: f64 = (0.5 * var_gf2);
        let assign41480_e54410: f64 = (4.0 / var_gf2);
        let assign41480_e54412: f64 = (assign41480_e54410 * var_temp__blk936);
        let assign41480_e54413: f64 = (1.0 + assign41480_e54412);
        let assign41480_e54414: f64 = (assign41480_e54413).sqrt();
        let assign41480_e54416: f64 = (assign41480_e54414 - 1.0);
        let assign41480_e54417: f64 = (assign41480_e54406 * assign41480_e54416);
        (assign41480_e54417, (((0.5 * var_gf2_dn5) * assign41480_e54416) + (assign41480_e54406 * ((((-((4.0 * var_gf2_dn5) / (var_gf2 * var_gf2))) * var_temp__blk936) + (assign41480_e54410 * var_temp__blk936_dn5)) / (2.0 * assign41480_e54414)))), (((0.5 * var_gf2_dn6) * assign41480_e54416) + (assign41480_e54406 * ((((-((4.0 * var_gf2_dn6) / (var_gf2 * var_gf2))) * var_temp__blk936) + (assign41480_e54410 * var_temp__blk936_dn6)) / (2.0 * assign41480_e54414)))), (((0.5 * var_gf2_dn7) * assign41480_e54416) + (assign41480_e54406 * ((((-((4.0 * var_gf2_dn7) / (var_gf2 * var_gf2))) * var_temp__blk936) + (assign41480_e54410 * var_temp__blk936_dn7)) / (2.0 * assign41480_e54414)))), (((0.5 * var_gf2_dn8) * assign41480_e54416) + (assign41480_e54406 * ((((-((4.0 * var_gf2_dn8) / (var_gf2 * var_gf2))) * var_temp__blk936) + (assign41480_e54410 * var_temp__blk936_dn8)) / (2.0 * assign41480_e54414)))),)
    } else {
        (var_qbscr, var_qbscr_dn5, var_qbscr_dn6, var_qbscr_dn7, var_qbscr_dn8,)
    }
};
        var_qbscr = assign41480_e54419;
        var_qbscr_dn5 = assign41480_e54419_d_n5;
        var_qbscr_dn6 = assign41480_e54419_d_n6;
        var_qbscr_dn7 = assign41480_e54419_d_n7;
        var_qbscr_dn8 = assign41480_e54419_d_n8;
        var_qbscr_rv = 0.0;

        let (assign41490_e54427, assign41490_e54427_d_n5, assign41490_e54427_d_n6, assign41490_e54427_d_n7, assign41490_e54427_d_n8,) = {
    if (var_guard1178 != 0.0) {
        let assign41490_e54424: f64 = (var_qbscr + var_qiscr);
        let assign41490_e54425: f64 = (var_qbscr / assign41490_e54424);
        (assign41490_e54425, (((var_qbscr_dn5 * assign41490_e54424) - (var_qbscr * (var_qbscr_dn5 + var_qiscr_dn5))) / (assign41490_e54424 * assign41490_e54424)), (((var_qbscr_dn6 * assign41490_e54424) - (var_qbscr * (var_qbscr_dn6 + var_qiscr_dn6))) / (assign41490_e54424 * assign41490_e54424)), (((var_qbscr_dn7 * assign41490_e54424) - (var_qbscr * (var_qbscr_dn7 + var_qiscr_dn7))) / (assign41490_e54424 * assign41490_e54424)), (((var_qbscr_dn8 * assign41490_e54424) - (var_qbscr * (var_qbscr_dn8 + var_qiscr_dn8))) / (assign41490_e54424 * assign41490_e54424)),)
    } else {
        (var_fscr, var_fscr_dn5, var_fscr_dn6, var_fscr_dn7, var_fscr_dn8,)
    }
};
        var_fscr = assign41490_e54427;
        var_fscr_dn5 = assign41490_e54427_d_n5;
        var_fscr_dn6 = assign41490_e54427_d_n6;
        var_fscr_dn7 = assign41490_e54427_d_n7;
        var_fscr_dn8 = assign41490_e54427_d_n8;
        var_fscr_rv = 0.0;

        let (assign41500_e54435, assign41500_e54435_d_n5, assign41500_e54435_d_n6, assign41500_e54435_d_n7, assign41500_e54435_d_n8,) = {
    if (var_guard1178 != 0.0) {
        let assign41500_e54432: f64 = (var_fscr * var_delxb);
        let assign41500_e54433: f64 = (var_xno_s - assign41500_e54432);
        (assign41500_e54433, (var_xno_s_dn5 - ((var_fscr_dn5 * var_delxb) + (var_fscr * var_delxb_dn5))), (var_xno_s_dn6 - ((var_fscr_dn6 * var_delxb) + (var_fscr * var_delxb_dn6))), (var_xno_s_dn7 - ((var_fscr_dn7 * var_delxb) + (var_fscr * var_delxb_dn7))), (var_xno_s_dn8 - ((var_fscr_dn8 * var_delxb) + (var_fscr * var_delxb_dn8))),)
    } else {
        (var_xn_s, var_xn_s_dn5, var_xn_s_dn6, var_xn_s_dn7, var_xn_s_dn8,)
    }
};
        var_xn_s = assign41500_e54435;
        var_xn_s_dn5 = assign41500_e54435_d_n5;
        var_xn_s_dn6 = assign41500_e54435_d_n6;
        var_xn_s_dn7 = assign41500_e54435_d_n7;
        var_xn_s_dn8 = assign41500_e54435_d_n8;
        var_xn_s_rv = 0.0;

        let assign41510_e54439: f64 = (var_gf * 0.7071067811865475);
        let assign41510_e54440: f64 = (1.0 + assign41510_e54439);
        var_xi = assign41510_e54440;
        var_xi_dn5 = (var_gf_dn5 * 0.7071067811865475);
        var_xi_dn6 = (var_gf_dn6 * 0.7071067811865475);
        var_xi_dn7 = (var_gf_dn7 * 0.7071067811865475);
        var_xi_dn8 = (var_gf_dn8 * 0.7071067811865475);
        var_xi_rv = 0.0;

        let assign41520_e54443: f64 = (1e-5 * var_xi);
        var_margin = assign41520_e54443;
        var_margin_dn5 = (1e-5 * var_xi_dn5);
        var_margin_dn6 = (1e-5 * var_xi_dn6);
        var_margin_dn7 = (1e-5 * var_xi_dn7);
        var_margin_dn8 = (1e-5 * var_xi_dn8);
        var_margin_rv = 0.0;

        let assign41530_e54446: f64 = (1.0 / var_xi);
        var_inv_xi = assign41530_e54446;
        var_inv_xi_dn5 = (-(var_xi_dn5 / (var_xi * var_xi)));
        var_inv_xi_dn6 = (-(var_xi_dn6 / (var_xi * var_xi)));
        var_inv_xi_dn7 = (-(var_xi_dn7 / (var_xi * var_xi)));
        var_inv_xi_dn8 = (-(var_xi_dn8 / (var_xi * var_xi)));
        var_inv_xi_rv = 0.0;

        var_sp_s_x1 = 0.0;
        var_sp_s_x1_dn5 = 0.0;
        var_sp_s_x1_dn6 = 0.0;
        var_sp_s_x1_dn7 = 0.0;
        var_sp_s_x1_dn8 = 0.0;
        var_sp_s_x1_rv = 0.0;

        var_x_s = 0.0;
        var_x_s_dn5 = 0.0;
        var_x_s_dn6 = 0.0;
        var_x_s_dn7 = 0.0;
        var_x_s_dn8 = 0.0;
        var_x_s_rv = 0.0;

        let assign41560_e54451: f64 = if var_xn_s < 460.51701859880916 { 1.0 } else { 0.0 };
        var_guard1181 = assign41560_e54451;
        var_guard1181_rv = 0.0;

        let (assign41570_e54457, assign41570_e54457_d_n5, assign41570_e54457_d_n6, assign41570_e54457_d_n7, assign41570_e54457_d_n8,) = {
    if (var_guard1181 != 0.0) {
        let assign41570_e54454: f64 = (-var_xn_s);
        let assign41570_e54455: f64 = (assign41570_e54454).exp();
        (assign41570_e54455, (assign41570_e54455 * (-var_xn_s_dn5)), (assign41570_e54455 * (-var_xn_s_dn6)), (assign41570_e54455 * (-var_xn_s_dn7)), (assign41570_e54455 * (-var_xn_s_dn8)),)
    } else {
        (var_delta_ns, var_delta_ns_dn5, var_delta_ns_dn6, var_delta_ns_dn7, var_delta_ns_dn8,)
    }
};
        var_delta_ns = assign41570_e54457;
        var_delta_ns_dn5 = assign41570_e54457_d_n5;
        var_delta_ns_dn6 = assign41570_e54457_d_n6;
        var_delta_ns_dn7 = assign41570_e54457_d_n7;
        var_delta_ns_dn8 = assign41570_e54457_d_n8;
        var_delta_ns_rv = 0.0;

        let (assign41580_e54484, assign41580_e54484_d_n5, assign41580_e54484_d_n6, assign41580_e54484_d_n7, assign41580_e54484_d_n8,) = {
    if (var_guard1181 == 0.0) {
        let assign41580_e54464: f64 = (var_xn_s - 460.51701859880916);
        let assign41580_e54469: f64 = (var_xn_s - 460.51701859880916);
        let assign41580_e54473: f64 = (var_xn_s - 460.51701859880916);
        let assign41580_e54475: f64 = (assign41580_e54473 * 0.3333333333333333);
        let assign41580_e54476: f64 = (1.0 + assign41580_e54475);
        let assign41580_e54477: f64 = (assign41580_e54469 * assign41580_e54476);
        let assign41580_e54478: f64 = (0.5 * assign41580_e54477);
        let assign41580_e54479: f64 = (1.0 + assign41580_e54478);
        let assign41580_e54480: f64 = (assign41580_e54464 * assign41580_e54479);
        let assign41580_e54481: f64 = (1.0 + assign41580_e54480);
        let assign41580_e54482: f64 = (1e-200 / assign41580_e54481);
        (assign41580_e54482, (-((1e-200 * ((var_xn_s_dn5 * assign41580_e54479) + (assign41580_e54464 * (0.5 * ((var_xn_s_dn5 * assign41580_e54476) + (assign41580_e54469 * (var_xn_s_dn5 * 0.3333333333333333))))))) / (assign41580_e54481 * assign41580_e54481))), (-((1e-200 * ((var_xn_s_dn6 * assign41580_e54479) + (assign41580_e54464 * (0.5 * ((var_xn_s_dn6 * assign41580_e54476) + (assign41580_e54469 * (var_xn_s_dn6 * 0.3333333333333333))))))) / (assign41580_e54481 * assign41580_e54481))), (-((1e-200 * ((var_xn_s_dn7 * assign41580_e54479) + (assign41580_e54464 * (0.5 * ((var_xn_s_dn7 * assign41580_e54476) + (assign41580_e54469 * (var_xn_s_dn7 * 0.3333333333333333))))))) / (assign41580_e54481 * assign41580_e54481))), (-((1e-200 * ((var_xn_s_dn8 * assign41580_e54479) + (assign41580_e54464 * (0.5 * ((var_xn_s_dn8 * assign41580_e54476) + (assign41580_e54469 * (var_xn_s_dn8 * 0.3333333333333333))))))) / (assign41580_e54481 * assign41580_e54481))),)
    } else {
        (var_delta_ns, var_delta_ns_dn5, var_delta_ns_dn6, var_delta_ns_dn7, var_delta_ns_dn8,)
    }
};
        var_delta_ns = assign41580_e54484;
        var_delta_ns_dn5 = assign41580_e54484_d_n5;
        var_delta_ns_dn6 = assign41580_e54484_d_n6;
        var_delta_ns_dn7 = assign41580_e54484_d_n7;
        var_delta_ns_dn8 = assign41580_e54484_d_n8;
        var_delta_ns_rv = 0.0;

        let assign41590_e54486: f64 = (var_xg).abs();
        let assign41590_e54488: f64 = if assign41590_e54486 <= var_margin { 1.0 } else { 0.0 };
        var_guard1182 = assign41590_e54488;
        var_guard1182_rv = 0.0;

        let (assign41600_e54498, assign41600_e54498_d_n5, assign41600_e54498_d_n6, assign41600_e54498_d_n7, assign41600_e54498_d_n8,) = {
    if (var_guard1182 != 0.0) {
        let assign41600_e54492: f64 = (var_inv_xi * var_inv_xi);
        let assign41600_e54494: f64 = (assign41600_e54492 * 0.16666666666666666);
        let assign41600_e54496: f64 = (assign41600_e54494 * 0.7071067811865475);
        (assign41600_e54496, ((((var_inv_xi_dn5 * var_inv_xi) + (var_inv_xi * var_inv_xi_dn5)) * 0.16666666666666666) * 0.7071067811865475), ((((var_inv_xi_dn6 * var_inv_xi) + (var_inv_xi * var_inv_xi_dn6)) * 0.16666666666666666) * 0.7071067811865475), ((((var_inv_xi_dn7 * var_inv_xi) + (var_inv_xi * var_inv_xi_dn7)) * 0.16666666666666666) * 0.7071067811865475), ((((var_inv_xi_dn8 * var_inv_xi) + (var_inv_xi * var_inv_xi_dn8)) * 0.16666666666666666) * 0.7071067811865475),)
    } else {
        (var_sp_s_temp1, var_sp_s_temp1_dn5, var_sp_s_temp1_dn6, var_sp_s_temp1_dn7, var_sp_s_temp1_dn8,)
    }
};
        var_sp_s_temp1 = assign41600_e54498;
        var_sp_s_temp1_dn5 = assign41600_e54498_d_n5;
        var_sp_s_temp1_dn6 = assign41600_e54498_d_n6;
        var_sp_s_temp1_dn7 = assign41600_e54498_d_n7;
        var_sp_s_temp1_dn8 = assign41600_e54498_d_n8;
        var_sp_s_temp1_rv = 0.0;

        let (assign41610_e54516, assign41610_e54516_d_n5, assign41610_e54516_d_n6, assign41610_e54516_d_n7, assign41610_e54516_d_n8,) = {
    if (var_guard1182 != 0.0) {
        let assign41610_e54502: f64 = (var_xg * var_inv_xi);
        let assign41610_e54507: f64 = (1.0 - var_delta_ns);
        let assign41610_e54508: f64 = (var_xg * assign41610_e54507);
        let assign41610_e54510: f64 = (assign41610_e54508 * var_gf);
        let assign41610_e54512: f64 = (assign41610_e54510 * var_sp_s_temp1);
        let assign41610_e54513: f64 = (1.0 + assign41610_e54512);
        let assign41610_e54514: f64 = (assign41610_e54502 * assign41610_e54513);
        (assign41610_e54514, ((((var_xg_dn5 * var_inv_xi) + (var_xg * var_inv_xi_dn5)) * assign41610_e54513) + (assign41610_e54502 * ((((((var_xg_dn5 * assign41610_e54507) + (var_xg * (-var_delta_ns_dn5))) * var_gf) + (assign41610_e54508 * var_gf_dn5)) * var_sp_s_temp1) + (assign41610_e54510 * var_sp_s_temp1_dn5)))), ((((var_xg_dn6 * var_inv_xi) + (var_xg * var_inv_xi_dn6)) * assign41610_e54513) + (assign41610_e54502 * ((((((var_xg_dn6 * assign41610_e54507) + (var_xg * (-var_delta_ns_dn6))) * var_gf) + (assign41610_e54508 * var_gf_dn6)) * var_sp_s_temp1) + (assign41610_e54510 * var_sp_s_temp1_dn6)))), ((((var_xg_dn7 * var_inv_xi) + (var_xg * var_inv_xi_dn7)) * assign41610_e54513) + (assign41610_e54502 * ((((((var_xg_dn7 * assign41610_e54507) + (var_xg * (-var_delta_ns_dn7))) * var_gf) + (assign41610_e54508 * var_gf_dn7)) * var_sp_s_temp1) + (assign41610_e54510 * var_sp_s_temp1_dn7)))), ((((var_xg_dn8 * var_inv_xi) + (var_xg * var_inv_xi_dn8)) * assign41610_e54513) + (assign41610_e54502 * ((((((var_xg_dn8 * assign41610_e54507) + (var_xg * (-var_delta_ns_dn8))) * var_gf) + (assign41610_e54508 * var_gf_dn8)) * var_sp_s_temp1) + (assign41610_e54510 * var_sp_s_temp1_dn8)))),)
    } else {
        (var_x_s, var_x_s_dn5, var_x_s_dn6, var_x_s_dn7, var_x_s_dn8,)
    }
};
        var_x_s = assign41610_e54516;
        var_x_s_dn5 = assign41610_e54516_d_n5;
        var_x_s_dn6 = assign41610_e54516_d_n6;
        var_x_s_dn7 = assign41610_e54516_d_n7;
        var_x_s_dn8 = assign41610_e54516_d_n8;
        var_x_s_rv = 0.0;

        let assign41620_e54519: f64 = (-var_margin);
        let assign41620_e54520: f64 = if var_xg < assign41620_e54519 { 1.0 } else { 0.0 };
        var_guard1183 = assign41620_e54520;
        var_guard1183_rv = 0.0;

        let (assign41630_e54528, assign41630_e54528_d_n5, assign41630_e54528_d_n6, assign41630_e54528_d_n7, assign41630_e54528_d_n8,) = {
    if ((var_guard1182 == 0.0) && (var_guard1183 != 0.0)) {
        let assign41630_e54526: f64 = (-var_xg);
        (assign41630_e54526, (-var_xg_dn5), (-var_xg_dn6), (-var_xg_dn7), (-var_xg_dn8),)
    } else {
        (var_sp_s_yg, var_sp_s_yg_dn5, var_sp_s_yg_dn6, var_sp_s_yg_dn7, var_sp_s_yg_dn8,)
    }
};
        var_sp_s_yg = assign41630_e54528;
        var_sp_s_yg_dn5 = assign41630_e54528_d_n5;
        var_sp_s_yg_dn6 = assign41630_e54528_d_n6;
        var_sp_s_yg_dn7 = assign41630_e54528_d_n7;
        var_sp_s_yg_dn8 = assign41630_e54528_d_n8;
        var_sp_s_yg_rv = 0.0;

        let (assign41640_e54539, assign41640_e54539_d_n5, assign41640_e54539_d_n6, assign41640_e54539_d_n7, assign41640_e54539_d_n8,) = {
    if ((var_guard1182 == 0.0) && (var_guard1183 != 0.0)) {
        let assign41640_e54536: f64 = (var_sp_s_yg * var_inv_xi);
        let assign41640_e54537: f64 = (1.25 * assign41640_e54536);
        (assign41640_e54537, (1.25 * ((var_sp_s_yg_dn5 * var_inv_xi) + (var_sp_s_yg * var_inv_xi_dn5))), (1.25 * ((var_sp_s_yg_dn6 * var_inv_xi) + (var_sp_s_yg * var_inv_xi_dn6))), (1.25 * ((var_sp_s_yg_dn7 * var_inv_xi) + (var_sp_s_yg * var_inv_xi_dn7))), (1.25 * ((var_sp_s_yg_dn8 * var_inv_xi) + (var_sp_s_yg * var_inv_xi_dn8))),)
    } else {
        (var_sp_s_ysub, var_sp_s_ysub_dn5, var_sp_s_ysub_dn6, var_sp_s_ysub_dn7, var_sp_s_ysub_dn8,)
    }
};
        var_sp_s_ysub = assign41640_e54539;
        var_sp_s_ysub_dn5 = assign41640_e54539_d_n5;
        var_sp_s_ysub_dn6 = assign41640_e54539_d_n6;
        var_sp_s_ysub_dn7 = assign41640_e54539_d_n7;
        var_sp_s_ysub_dn8 = assign41640_e54539_d_n8;
        var_sp_s_ysub_rv = 0.0;

        let (assign41650_e54561, assign41650_e54561_d_n5, assign41650_e54561_d_n6, assign41650_e54561_d_n7, assign41650_e54561_d_n8,) = {
    if ((var_guard1182 == 0.0) && (var_guard1183 != 0.0)) {
        let assign41650_e54547: f64 = (var_sp_s_ysub + 10.0);
        let assign41650_e54550: f64 = (var_sp_s_ysub - 6.0);
        let assign41650_e54553: f64 = (var_sp_s_ysub - 6.0);
        let assign41650_e54554: f64 = (assign41650_e54550 * assign41650_e54553);
        let assign41650_e54556: f64 = (assign41650_e54554 + 64.0);
        let assign41650_e54557: f64 = (assign41650_e54556).sqrt();
        let assign41650_e54558: f64 = (assign41650_e54547 - assign41650_e54557);
        let assign41650_e54559: f64 = (0.5 * assign41650_e54558);
        (assign41650_e54559, (0.5 * (var_sp_s_ysub_dn5 - (((var_sp_s_ysub_dn5 * assign41650_e54553) + (assign41650_e54550 * var_sp_s_ysub_dn5)) / (2.0 * assign41650_e54557)))), (0.5 * (var_sp_s_ysub_dn6 - (((var_sp_s_ysub_dn6 * assign41650_e54553) + (assign41650_e54550 * var_sp_s_ysub_dn6)) / (2.0 * assign41650_e54557)))), (0.5 * (var_sp_s_ysub_dn7 - (((var_sp_s_ysub_dn7 * assign41650_e54553) + (assign41650_e54550 * var_sp_s_ysub_dn7)) / (2.0 * assign41650_e54557)))), (0.5 * (var_sp_s_ysub_dn8 - (((var_sp_s_ysub_dn8 * assign41650_e54553) + (assign41650_e54550 * var_sp_s_ysub_dn8)) / (2.0 * assign41650_e54557)))),)
    } else {
        (var_sp_s_eta, var_sp_s_eta_dn5, var_sp_s_eta_dn6, var_sp_s_eta_dn7, var_sp_s_eta_dn8,)
    }
};
        var_sp_s_eta = assign41650_e54561;
        var_sp_s_eta_dn5 = assign41650_e54561_d_n5;
        var_sp_s_eta_dn6 = assign41650_e54561_d_n6;
        var_sp_s_eta_dn7 = assign41650_e54561_d_n7;
        var_sp_s_eta_dn8 = assign41650_e54561_d_n8;
        var_sp_s_eta_rv = 0.0;

        let (assign41660_e54570, assign41660_e54570_d_n5, assign41660_e54570_d_n6, assign41660_e54570_d_n7, assign41660_e54570_d_n8,) = {
    if ((var_guard1182 == 0.0) && (var_guard1183 != 0.0)) {
        let assign41660_e54568: f64 = (var_sp_s_yg - var_sp_s_eta);
        (assign41660_e54568, (var_sp_s_yg_dn5 - var_sp_s_eta_dn5), (var_sp_s_yg_dn6 - var_sp_s_eta_dn6), (var_sp_s_yg_dn7 - var_sp_s_eta_dn7), (var_sp_s_yg_dn8 - var_sp_s_eta_dn8),)
    } else {
        (var_sp_s_temp, var_sp_s_temp_dn5, var_sp_s_temp_dn6, var_sp_s_temp_dn7, var_sp_s_temp_dn8,)
    }
};
        var_sp_s_temp = assign41660_e54570;
        var_sp_s_temp_dn5 = assign41660_e54570_d_n5;
        var_sp_s_temp_dn6 = assign41660_e54570_d_n6;
        var_sp_s_temp_dn7 = assign41660_e54570_d_n7;
        var_sp_s_temp_dn8 = assign41660_e54570_d_n8;
        var_sp_s_temp_rv = 0.0;

        *var_delta_ns_slot = var_delta_ns;
        *var_delta_ns_dn5_slot = var_delta_ns_dn5;
        *var_delta_ns_dn6_slot = var_delta_ns_dn6;
        *var_delta_ns_dn7_slot = var_delta_ns_dn7;
        *var_delta_ns_dn8_slot = var_delta_ns_dn8;
        *var_delta_ns_rv_slot = var_delta_ns_rv;
        *var_dscr0_slot = var_dscr0;
        *var_dscr0_dn5_slot = var_dscr0_dn5;
        *var_dscr0_dn6_slot = var_dscr0_dn6;
        *var_dscr0_dn7_slot = var_dscr0_dn7;
        *var_dscr0_dn8_slot = var_dscr0_dn8;
        *var_dscr0_rv_slot = var_dscr0_rv;
        *var_fscr_slot = var_fscr;
        *var_fscr_dn5_slot = var_fscr_dn5;
        *var_fscr_dn6_slot = var_fscr_dn6;
        *var_fscr_dn7_slot = var_fscr_dn7;
        *var_fscr_dn8_slot = var_fscr_dn8;
        *var_fscr_rv_slot = var_fscr_rv;
        *var_guard1178_slot = var_guard1178;
        *var_guard1178_rv_slot = var_guard1178_rv;
        *var_guard1179_slot = var_guard1179;
        *var_guard1179_rv_slot = var_guard1179_rv;
        *var_guard1180_slot = var_guard1180;
        *var_guard1180_rv_slot = var_guard1180_rv;
        *var_guard1181_slot = var_guard1181;
        *var_guard1181_rv_slot = var_guard1181_rv;
        *var_guard1182_slot = var_guard1182;
        *var_guard1182_rv_slot = var_guard1182_rv;
        *var_guard1183_slot = var_guard1183;
        *var_guard1183_rv_slot = var_guard1183_rv;
        *var_inv_xi_slot = var_inv_xi;
        *var_inv_xi_dn5_slot = var_inv_xi_dn5;
        *var_inv_xi_dn6_slot = var_inv_xi_dn6;
        *var_inv_xi_dn7_slot = var_inv_xi_dn7;
        *var_inv_xi_dn8_slot = var_inv_xi_dn8;
        *var_inv_xi_rv_slot = var_inv_xi_rv;
        *var_margin_slot = var_margin;
        *var_margin_dn5_slot = var_margin_dn5;
        *var_margin_dn6_slot = var_margin_dn6;
        *var_margin_dn7_slot = var_margin_dn7;
        *var_margin_dn8_slot = var_margin_dn8;
        *var_margin_rv_slot = var_margin_rv;
        *var_qbscr_slot = var_qbscr;
        *var_qbscr_dn5_slot = var_qbscr_dn5;
        *var_qbscr_dn6_slot = var_qbscr_dn6;
        *var_qbscr_dn7_slot = var_qbscr_dn7;
        *var_qbscr_dn8_slot = var_qbscr_dn8;
        *var_qbscr_rv_slot = var_qbscr_rv;
        *var_qiscr_slot = var_qiscr;
        *var_qiscr0_slot = var_qiscr0;
        *var_qiscr0_dn5_slot = var_qiscr0_dn5;
        *var_qiscr0_dn6_slot = var_qiscr0_dn6;
        *var_qiscr0_dn7_slot = var_qiscr0_dn7;
        *var_qiscr0_dn8_slot = var_qiscr0_dn8;
        *var_qiscr0_rv_slot = var_qiscr0_rv;
        *var_qiscr0si_slot = var_qiscr0si;
        *var_qiscr0si_dn5_slot = var_qiscr0si_dn5;
        *var_qiscr0si_dn6_slot = var_qiscr0si_dn6;
        *var_qiscr0si_dn7_slot = var_qiscr0si_dn7;
        *var_qiscr0si_dn8_slot = var_qiscr0si_dn8;
        *var_qiscr0si_rv_slot = var_qiscr0si_rv;
        *var_qiscr_dn5_slot = var_qiscr_dn5;
        *var_qiscr_dn6_slot = var_qiscr_dn6;
        *var_qiscr_dn7_slot = var_qiscr_dn7;
        *var_qiscr_dn8_slot = var_qiscr_dn8;
        *var_qiscr_rv_slot = var_qiscr_rv;
        *var_sp_s_eta_slot = var_sp_s_eta;
        *var_sp_s_eta_dn5_slot = var_sp_s_eta_dn5;
        *var_sp_s_eta_dn6_slot = var_sp_s_eta_dn6;
        *var_sp_s_eta_dn7_slot = var_sp_s_eta_dn7;
        *var_sp_s_eta_dn8_slot = var_sp_s_eta_dn8;
        *var_sp_s_eta_rv_slot = var_sp_s_eta_rv;
        *var_sp_s_temp_slot = var_sp_s_temp;
        *var_sp_s_temp1_slot = var_sp_s_temp1;
        *var_sp_s_temp1_dn5_slot = var_sp_s_temp1_dn5;
        *var_sp_s_temp1_dn6_slot = var_sp_s_temp1_dn6;
        *var_sp_s_temp1_dn7_slot = var_sp_s_temp1_dn7;
        *var_sp_s_temp1_dn8_slot = var_sp_s_temp1_dn8;
        *var_sp_s_temp1_rv_slot = var_sp_s_temp1_rv;
        *var_sp_s_temp_dn5_slot = var_sp_s_temp_dn5;
        *var_sp_s_temp_dn6_slot = var_sp_s_temp_dn6;
        *var_sp_s_temp_dn7_slot = var_sp_s_temp_dn7;
        *var_sp_s_temp_dn8_slot = var_sp_s_temp_dn8;
        *var_sp_s_temp_rv_slot = var_sp_s_temp_rv;
        *var_sp_s_x1_slot = var_sp_s_x1;
        *var_sp_s_x1_dn5_slot = var_sp_s_x1_dn5;
        *var_sp_s_x1_dn6_slot = var_sp_s_x1_dn6;
        *var_sp_s_x1_dn7_slot = var_sp_s_x1_dn7;
        *var_sp_s_x1_dn8_slot = var_sp_s_x1_dn8;
        *var_sp_s_x1_rv_slot = var_sp_s_x1_rv;
        *var_sp_s_yg_slot = var_sp_s_yg;
        *var_sp_s_yg_dn5_slot = var_sp_s_yg_dn5;
        *var_sp_s_yg_dn6_slot = var_sp_s_yg_dn6;
        *var_sp_s_yg_dn7_slot = var_sp_s_yg_dn7;
        *var_sp_s_yg_dn8_slot = var_sp_s_yg_dn8;
        *var_sp_s_yg_rv_slot = var_sp_s_yg_rv;
        *var_sp_s_ysub_slot = var_sp_s_ysub;
        *var_sp_s_ysub_dn5_slot = var_sp_s_ysub_dn5;
        *var_sp_s_ysub_dn6_slot = var_sp_s_ysub_dn6;
        *var_sp_s_ysub_dn7_slot = var_sp_s_ysub_dn7;
        *var_sp_s_ysub_dn8_slot = var_sp_s_ysub_dn8;
        *var_sp_s_ysub_rv_slot = var_sp_s_ysub_rv;
        *var_temp__blk936_slot = var_temp__blk936;
        *var_temp__blk936_dn5_slot = var_temp__blk936_dn5;
        *var_temp__blk936_dn6_slot = var_temp__blk936_dn6;
        *var_temp__blk936_dn7_slot = var_temp__blk936_dn7;
        *var_temp__blk936_dn8_slot = var_temp__blk936_dn8;
        *var_temp__blk936_rv_slot = var_temp__blk936_rv;
        *var_x_s_slot = var_x_s;
        *var_x_s_dn5_slot = var_x_s_dn5;
        *var_x_s_dn6_slot = var_x_s_dn6;
        *var_x_s_dn7_slot = var_x_s_dn7;
        *var_x_s_dn8_slot = var_x_s_dn8;
        *var_x_s_rv_slot = var_x_s_rv;
        *var_xgtscr0_slot = var_xgtscr0;
        *var_xgtscr0_dn5_slot = var_xgtscr0_dn5;
        *var_xgtscr0_dn6_slot = var_xgtscr0_dn6;
        *var_xgtscr0_dn7_slot = var_xgtscr0_dn7;
        *var_xgtscr0_dn8_slot = var_xgtscr0_dn8;
        *var_xgtscr0_rv_slot = var_xgtscr0_rv;
        *var_xi_slot = var_xi;
        *var_xi_dn5_slot = var_xi_dn5;
        *var_xi_dn6_slot = var_xi_dn6;
        *var_xi_dn7_slot = var_xi_dn7;
        *var_xi_dn8_slot = var_xi_dn8;
        *var_xi_rv_slot = var_xi_rv;
        *var_xn_s_slot = var_xn_s;
        *var_xn_s_dn5_slot = var_xn_s_dn5;
        *var_xn_s_dn6_slot = var_xn_s_dn6;
        *var_xn_s_dn7_slot = var_xn_s_dn7;
        *var_xn_s_dn8_slot = var_xn_s_dn8;
        *var_xn_s_rv_slot = var_xn_s_rv;
    }

    pub(super) fn stamp_reactive_block_26(
        var_delta_ns: f64,
        var_delta_ns_dn5: f64,
        var_delta_ns_dn6: f64,
        var_delta_ns_dn7: f64,
        var_delta_ns_dn8: f64,
        var_gf: f64,
        var_gf2: f64,
        var_gf2_dn5: f64,
        var_gf2_dn6: f64,
        var_gf2_dn7: f64,
        var_gf2_dn8: f64,
        var_gf_dn5: f64,
        var_gf_dn6: f64,
        var_gf_dn7: f64,
        var_gf_dn8: f64,
        var_guard1182: f64,
        var_guard1183: f64,
        var_inv_gf2: f64,
        var_inv_gf2_dn5: f64,
        var_inv_gf2_dn6: f64,
        var_inv_gf2_dn7: f64,
        var_inv_gf2_dn8: f64,
        var_inv_xi: f64,
        var_inv_xi_dn5: f64,
        var_inv_xi_dn6: f64,
        var_inv_xi_dn7: f64,
        var_inv_xi_dn8: f64,
        var_sp_s_eta: f64,
        var_sp_s_eta_dn5: f64,
        var_sp_s_eta_dn6: f64,
        var_sp_s_eta_dn7: f64,
        var_sp_s_eta_dn8: f64,
        var_sp_s_yg: f64,
        var_sp_s_yg_dn5: f64,
        var_sp_s_yg_dn6: f64,
        var_sp_s_yg_dn7: f64,
        var_sp_s_yg_dn8: f64,
        var_xg: f64,
        var_xg_dn5: f64,
        var_xg_dn6: f64,
        var_xg_dn7: f64,
        var_xg_dn8: f64,
        var_xi: f64,
        var_xi_dn5: f64,
        var_xi_dn6: f64,
        var_xi_dn7: f64,
        var_xi_dn8: f64,
        var_guard1184_slot: &mut f64,
        var_guard1184_rv_slot: &mut f64,
        var_guard1185_slot: &mut f64,
        var_guard1185_rv_slot: &mut f64,
        var_mutau_slot: &mut f64,
        var_mutau_dn5_slot: &mut f64,
        var_mutau_dn6_slot: &mut f64,
        var_mutau_dn7_slot: &mut f64,
        var_mutau_dn8_slot: &mut f64,
        var_mutau_rv_slot: &mut f64,
        var_nu_slot: &mut f64,
        var_nu_dn5_slot: &mut f64,
        var_nu_dn6_slot: &mut f64,
        var_nu_dn7_slot: &mut f64,
        var_nu_dn8_slot: &mut f64,
        var_nu_rv_slot: &mut f64,
        var_sp_s_a_slot: &mut f64,
        var_sp_s_a_dn5_slot: &mut f64,
        var_sp_s_a_dn6_slot: &mut f64,
        var_sp_s_a_dn7_slot: &mut f64,
        var_sp_s_a_dn8_slot: &mut f64,
        var_sp_s_a_fac_slot: &mut f64,
        var_sp_s_a_fac_dn5_slot: &mut f64,
        var_sp_s_a_fac_dn6_slot: &mut f64,
        var_sp_s_a_fac_dn7_slot: &mut f64,
        var_sp_s_a_fac_dn8_slot: &mut f64,
        var_sp_s_a_fac_rv_slot: &mut f64,
        var_sp_s_a_rv_slot: &mut f64,
        var_sp_s_c_slot: &mut f64,
        var_sp_s_c_dn5_slot: &mut f64,
        var_sp_s_c_dn6_slot: &mut f64,
        var_sp_s_c_dn7_slot: &mut f64,
        var_sp_s_c_dn8_slot: &mut f64,
        var_sp_s_c_rv_slot: &mut f64,
        var_sp_s_delta0_slot: &mut f64,
        var_sp_s_delta0_dn5_slot: &mut f64,
        var_sp_s_delta0_dn6_slot: &mut f64,
        var_sp_s_delta0_dn7_slot: &mut f64,
        var_sp_s_delta0_dn8_slot: &mut f64,
        var_sp_s_delta0_rv_slot: &mut f64,
        var_sp_s_delta1_slot: &mut f64,
        var_sp_s_delta1_dn5_slot: &mut f64,
        var_sp_s_delta1_dn6_slot: &mut f64,
        var_sp_s_delta1_dn7_slot: &mut f64,
        var_sp_s_delta1_dn8_slot: &mut f64,
        var_sp_s_delta1_rv_slot: &mut f64,
        var_sp_s_pc_slot: &mut f64,
        var_sp_s_pc_dn5_slot: &mut f64,
        var_sp_s_pc_dn6_slot: &mut f64,
        var_sp_s_pc_dn7_slot: &mut f64,
        var_sp_s_pc_dn8_slot: &mut f64,
        var_sp_s_pc_rv_slot: &mut f64,
        var_sp_s_qc_slot: &mut f64,
        var_sp_s_qc_dn5_slot: &mut f64,
        var_sp_s_qc_dn6_slot: &mut f64,
        var_sp_s_qc_dn7_slot: &mut f64,
        var_sp_s_qc_dn8_slot: &mut f64,
        var_sp_s_qc_rv_slot: &mut f64,
        var_sp_s_tau_slot: &mut f64,
        var_sp_s_tau_dn5_slot: &mut f64,
        var_sp_s_tau_dn6_slot: &mut f64,
        var_sp_s_tau_dn7_slot: &mut f64,
        var_sp_s_tau_dn8_slot: &mut f64,
        var_sp_s_tau_rv_slot: &mut f64,
        var_sp_s_temp_slot: &mut f64,
        var_sp_s_temp1_slot: &mut f64,
        var_sp_s_temp1_dn5_slot: &mut f64,
        var_sp_s_temp1_dn6_slot: &mut f64,
        var_sp_s_temp1_dn7_slot: &mut f64,
        var_sp_s_temp1_dn8_slot: &mut f64,
        var_sp_s_temp1_rv_slot: &mut f64,
        var_sp_s_temp_dn5_slot: &mut f64,
        var_sp_s_temp_dn6_slot: &mut f64,
        var_sp_s_temp_dn7_slot: &mut f64,
        var_sp_s_temp_dn8_slot: &mut f64,
        var_sp_s_temp_rv_slot: &mut f64,
        var_sp_s_w_slot: &mut f64,
        var_sp_s_w_dn5_slot: &mut f64,
        var_sp_s_w_dn6_slot: &mut f64,
        var_sp_s_w_dn7_slot: &mut f64,
        var_sp_s_w_dn8_slot: &mut f64,
        var_sp_s_w_rv_slot: &mut f64,
        var_sp_s_xbar_slot: &mut f64,
        var_sp_s_xbar_dn5_slot: &mut f64,
        var_sp_s_xbar_dn6_slot: &mut f64,
        var_sp_s_xbar_dn7_slot: &mut f64,
        var_sp_s_xbar_dn8_slot: &mut f64,
        var_sp_s_xbar_rv_slot: &mut f64,
        var_sp_s_xi0_slot: &mut f64,
        var_sp_s_xi0_dn5_slot: &mut f64,
        var_sp_s_xi0_dn6_slot: &mut f64,
        var_sp_s_xi0_dn7_slot: &mut f64,
        var_sp_s_xi0_dn8_slot: &mut f64,
        var_sp_s_xi0_rv_slot: &mut f64,
        var_sp_s_xi1_slot: &mut f64,
        var_sp_s_xi1_dn5_slot: &mut f64,
        var_sp_s_xi1_dn6_slot: &mut f64,
        var_sp_s_xi1_dn7_slot: &mut f64,
        var_sp_s_xi1_dn8_slot: &mut f64,
        var_sp_s_xi1_rv_slot: &mut f64,
        var_sp_s_xi2_slot: &mut f64,
        var_sp_s_xi2_dn5_slot: &mut f64,
        var_sp_s_xi2_dn6_slot: &mut f64,
        var_sp_s_xi2_dn7_slot: &mut f64,
        var_sp_s_xi2_dn8_slot: &mut f64,
        var_sp_s_xi2_rv_slot: &mut f64,
        var_sp_s_y0_slot: &mut f64,
        var_sp_s_y0_dn5_slot: &mut f64,
        var_sp_s_y0_dn6_slot: &mut f64,
        var_sp_s_y0_dn7_slot: &mut f64,
        var_sp_s_y0_dn8_slot: &mut f64,
        var_sp_s_y0_rv_slot: &mut f64,
        var_sp_xg1_slot: &mut f64,
        var_sp_xg1_dn5_slot: &mut f64,
        var_sp_xg1_dn6_slot: &mut f64,
        var_sp_xg1_dn7_slot: &mut f64,
        var_sp_xg1_dn8_slot: &mut f64,
        var_sp_xg1_rv_slot: &mut f64,
        var_x_s_slot: &mut f64,
        var_x_s_dn5_slot: &mut f64,
        var_x_s_dn6_slot: &mut f64,
        var_x_s_dn7_slot: &mut f64,
        var_x_s_dn8_slot: &mut f64,
        var_x_s_rv_slot: &mut f64,
    ) {
        let mut var_guard1184: f64 = *var_guard1184_slot;
        let mut var_guard1184_rv: f64 = *var_guard1184_rv_slot;
        let mut var_guard1185: f64 = *var_guard1185_slot;
        let mut var_guard1185_rv: f64 = *var_guard1185_rv_slot;
        let mut var_mutau: f64 = *var_mutau_slot;
        let mut var_mutau_dn5: f64 = *var_mutau_dn5_slot;
        let mut var_mutau_dn6: f64 = *var_mutau_dn6_slot;
        let mut var_mutau_dn7: f64 = *var_mutau_dn7_slot;
        let mut var_mutau_dn8: f64 = *var_mutau_dn8_slot;
        let mut var_mutau_rv: f64 = *var_mutau_rv_slot;
        let mut var_nu: f64 = *var_nu_slot;
        let mut var_nu_dn5: f64 = *var_nu_dn5_slot;
        let mut var_nu_dn6: f64 = *var_nu_dn6_slot;
        let mut var_nu_dn7: f64 = *var_nu_dn7_slot;
        let mut var_nu_dn8: f64 = *var_nu_dn8_slot;
        let mut var_nu_rv: f64 = *var_nu_rv_slot;
        let mut var_sp_s_a: f64 = *var_sp_s_a_slot;
        let mut var_sp_s_a_dn5: f64 = *var_sp_s_a_dn5_slot;
        let mut var_sp_s_a_dn6: f64 = *var_sp_s_a_dn6_slot;
        let mut var_sp_s_a_dn7: f64 = *var_sp_s_a_dn7_slot;
        let mut var_sp_s_a_dn8: f64 = *var_sp_s_a_dn8_slot;
        let mut var_sp_s_a_fac: f64 = *var_sp_s_a_fac_slot;
        let mut var_sp_s_a_fac_dn5: f64 = *var_sp_s_a_fac_dn5_slot;
        let mut var_sp_s_a_fac_dn6: f64 = *var_sp_s_a_fac_dn6_slot;
        let mut var_sp_s_a_fac_dn7: f64 = *var_sp_s_a_fac_dn7_slot;
        let mut var_sp_s_a_fac_dn8: f64 = *var_sp_s_a_fac_dn8_slot;
        let mut var_sp_s_a_fac_rv: f64 = *var_sp_s_a_fac_rv_slot;
        let mut var_sp_s_a_rv: f64 = *var_sp_s_a_rv_slot;
        let mut var_sp_s_c: f64 = *var_sp_s_c_slot;
        let mut var_sp_s_c_dn5: f64 = *var_sp_s_c_dn5_slot;
        let mut var_sp_s_c_dn6: f64 = *var_sp_s_c_dn6_slot;
        let mut var_sp_s_c_dn7: f64 = *var_sp_s_c_dn7_slot;
        let mut var_sp_s_c_dn8: f64 = *var_sp_s_c_dn8_slot;
        let mut var_sp_s_c_rv: f64 = *var_sp_s_c_rv_slot;
        let mut var_sp_s_delta0: f64 = *var_sp_s_delta0_slot;
        let mut var_sp_s_delta0_dn5: f64 = *var_sp_s_delta0_dn5_slot;
        let mut var_sp_s_delta0_dn6: f64 = *var_sp_s_delta0_dn6_slot;
        let mut var_sp_s_delta0_dn7: f64 = *var_sp_s_delta0_dn7_slot;
        let mut var_sp_s_delta0_dn8: f64 = *var_sp_s_delta0_dn8_slot;
        let mut var_sp_s_delta0_rv: f64 = *var_sp_s_delta0_rv_slot;
        let mut var_sp_s_delta1: f64 = *var_sp_s_delta1_slot;
        let mut var_sp_s_delta1_dn5: f64 = *var_sp_s_delta1_dn5_slot;
        let mut var_sp_s_delta1_dn6: f64 = *var_sp_s_delta1_dn6_slot;
        let mut var_sp_s_delta1_dn7: f64 = *var_sp_s_delta1_dn7_slot;
        let mut var_sp_s_delta1_dn8: f64 = *var_sp_s_delta1_dn8_slot;
        let mut var_sp_s_delta1_rv: f64 = *var_sp_s_delta1_rv_slot;
        let mut var_sp_s_pc: f64 = *var_sp_s_pc_slot;
        let mut var_sp_s_pc_dn5: f64 = *var_sp_s_pc_dn5_slot;
        let mut var_sp_s_pc_dn6: f64 = *var_sp_s_pc_dn6_slot;
        let mut var_sp_s_pc_dn7: f64 = *var_sp_s_pc_dn7_slot;
        let mut var_sp_s_pc_dn8: f64 = *var_sp_s_pc_dn8_slot;
        let mut var_sp_s_pc_rv: f64 = *var_sp_s_pc_rv_slot;
        let mut var_sp_s_qc: f64 = *var_sp_s_qc_slot;
        let mut var_sp_s_qc_dn5: f64 = *var_sp_s_qc_dn5_slot;
        let mut var_sp_s_qc_dn6: f64 = *var_sp_s_qc_dn6_slot;
        let mut var_sp_s_qc_dn7: f64 = *var_sp_s_qc_dn7_slot;
        let mut var_sp_s_qc_dn8: f64 = *var_sp_s_qc_dn8_slot;
        let mut var_sp_s_qc_rv: f64 = *var_sp_s_qc_rv_slot;
        let mut var_sp_s_tau: f64 = *var_sp_s_tau_slot;
        let mut var_sp_s_tau_dn5: f64 = *var_sp_s_tau_dn5_slot;
        let mut var_sp_s_tau_dn6: f64 = *var_sp_s_tau_dn6_slot;
        let mut var_sp_s_tau_dn7: f64 = *var_sp_s_tau_dn7_slot;
        let mut var_sp_s_tau_dn8: f64 = *var_sp_s_tau_dn8_slot;
        let mut var_sp_s_tau_rv: f64 = *var_sp_s_tau_rv_slot;
        let mut var_sp_s_temp: f64 = *var_sp_s_temp_slot;
        let mut var_sp_s_temp1: f64 = *var_sp_s_temp1_slot;
        let mut var_sp_s_temp1_dn5: f64 = *var_sp_s_temp1_dn5_slot;
        let mut var_sp_s_temp1_dn6: f64 = *var_sp_s_temp1_dn6_slot;
        let mut var_sp_s_temp1_dn7: f64 = *var_sp_s_temp1_dn7_slot;
        let mut var_sp_s_temp1_dn8: f64 = *var_sp_s_temp1_dn8_slot;
        let mut var_sp_s_temp1_rv: f64 = *var_sp_s_temp1_rv_slot;
        let mut var_sp_s_temp_dn5: f64 = *var_sp_s_temp_dn5_slot;
        let mut var_sp_s_temp_dn6: f64 = *var_sp_s_temp_dn6_slot;
        let mut var_sp_s_temp_dn7: f64 = *var_sp_s_temp_dn7_slot;
        let mut var_sp_s_temp_dn8: f64 = *var_sp_s_temp_dn8_slot;
        let mut var_sp_s_temp_rv: f64 = *var_sp_s_temp_rv_slot;
        let mut var_sp_s_w: f64 = *var_sp_s_w_slot;
        let mut var_sp_s_w_dn5: f64 = *var_sp_s_w_dn5_slot;
        let mut var_sp_s_w_dn6: f64 = *var_sp_s_w_dn6_slot;
        let mut var_sp_s_w_dn7: f64 = *var_sp_s_w_dn7_slot;
        let mut var_sp_s_w_dn8: f64 = *var_sp_s_w_dn8_slot;
        let mut var_sp_s_w_rv: f64 = *var_sp_s_w_rv_slot;
        let mut var_sp_s_xbar: f64 = *var_sp_s_xbar_slot;
        let mut var_sp_s_xbar_dn5: f64 = *var_sp_s_xbar_dn5_slot;
        let mut var_sp_s_xbar_dn6: f64 = *var_sp_s_xbar_dn6_slot;
        let mut var_sp_s_xbar_dn7: f64 = *var_sp_s_xbar_dn7_slot;
        let mut var_sp_s_xbar_dn8: f64 = *var_sp_s_xbar_dn8_slot;
        let mut var_sp_s_xbar_rv: f64 = *var_sp_s_xbar_rv_slot;
        let mut var_sp_s_xi0: f64 = *var_sp_s_xi0_slot;
        let mut var_sp_s_xi0_dn5: f64 = *var_sp_s_xi0_dn5_slot;
        let mut var_sp_s_xi0_dn6: f64 = *var_sp_s_xi0_dn6_slot;
        let mut var_sp_s_xi0_dn7: f64 = *var_sp_s_xi0_dn7_slot;
        let mut var_sp_s_xi0_dn8: f64 = *var_sp_s_xi0_dn8_slot;
        let mut var_sp_s_xi0_rv: f64 = *var_sp_s_xi0_rv_slot;
        let mut var_sp_s_xi1: f64 = *var_sp_s_xi1_slot;
        let mut var_sp_s_xi1_dn5: f64 = *var_sp_s_xi1_dn5_slot;
        let mut var_sp_s_xi1_dn6: f64 = *var_sp_s_xi1_dn6_slot;
        let mut var_sp_s_xi1_dn7: f64 = *var_sp_s_xi1_dn7_slot;
        let mut var_sp_s_xi1_dn8: f64 = *var_sp_s_xi1_dn8_slot;
        let mut var_sp_s_xi1_rv: f64 = *var_sp_s_xi1_rv_slot;
        let mut var_sp_s_xi2: f64 = *var_sp_s_xi2_slot;
        let mut var_sp_s_xi2_dn5: f64 = *var_sp_s_xi2_dn5_slot;
        let mut var_sp_s_xi2_dn6: f64 = *var_sp_s_xi2_dn6_slot;
        let mut var_sp_s_xi2_dn7: f64 = *var_sp_s_xi2_dn7_slot;
        let mut var_sp_s_xi2_dn8: f64 = *var_sp_s_xi2_dn8_slot;
        let mut var_sp_s_xi2_rv: f64 = *var_sp_s_xi2_rv_slot;
        let mut var_sp_s_y0: f64 = *var_sp_s_y0_slot;
        let mut var_sp_s_y0_dn5: f64 = *var_sp_s_y0_dn5_slot;
        let mut var_sp_s_y0_dn6: f64 = *var_sp_s_y0_dn6_slot;
        let mut var_sp_s_y0_dn7: f64 = *var_sp_s_y0_dn7_slot;
        let mut var_sp_s_y0_dn8: f64 = *var_sp_s_y0_dn8_slot;
        let mut var_sp_s_y0_rv: f64 = *var_sp_s_y0_rv_slot;
        let mut var_sp_xg1: f64 = *var_sp_xg1_slot;
        let mut var_sp_xg1_dn5: f64 = *var_sp_xg1_dn5_slot;
        let mut var_sp_xg1_dn6: f64 = *var_sp_xg1_dn6_slot;
        let mut var_sp_xg1_dn7: f64 = *var_sp_xg1_dn7_slot;
        let mut var_sp_xg1_dn8: f64 = *var_sp_xg1_dn8_slot;
        let mut var_sp_xg1_rv: f64 = *var_sp_xg1_rv_slot;
        let mut var_x_s: f64 = *var_x_s_slot;
        let mut var_x_s_dn5: f64 = *var_x_s_dn5_slot;
        let mut var_x_s_dn6: f64 = *var_x_s_dn6_slot;
        let mut var_x_s_dn7: f64 = *var_x_s_dn7_slot;
        let mut var_x_s_dn8: f64 = *var_x_s_dn8_slot;
        let mut var_x_s_rv: f64 = *var_x_s_rv_slot;

        let (assign41670_e54585, assign41670_e54585_d_n5, assign41670_e54585_d_n6, assign41670_e54585_d_n7, assign41670_e54585_d_n8,) = {
    if ((var_guard1182 == 0.0) && (var_guard1183 != 0.0)) {
        let assign41670_e54577: f64 = (var_sp_s_temp * var_sp_s_temp);
        let assign41670_e54581: f64 = (var_sp_s_eta + 1.0);
        let assign41670_e54582: f64 = (var_gf2 * assign41670_e54581);
        let assign41670_e54583: f64 = (assign41670_e54577 + assign41670_e54582);
        (assign41670_e54583, (((var_sp_s_temp_dn5 * var_sp_s_temp) + (var_sp_s_temp * var_sp_s_temp_dn5)) + ((var_gf2_dn5 * assign41670_e54581) + (var_gf2 * var_sp_s_eta_dn5))), (((var_sp_s_temp_dn6 * var_sp_s_temp) + (var_sp_s_temp * var_sp_s_temp_dn6)) + ((var_gf2_dn6 * assign41670_e54581) + (var_gf2 * var_sp_s_eta_dn6))), (((var_sp_s_temp_dn7 * var_sp_s_temp) + (var_sp_s_temp * var_sp_s_temp_dn7)) + ((var_gf2_dn7 * assign41670_e54581) + (var_gf2 * var_sp_s_eta_dn7))), (((var_sp_s_temp_dn8 * var_sp_s_temp) + (var_sp_s_temp * var_sp_s_temp_dn8)) + ((var_gf2_dn8 * assign41670_e54581) + (var_gf2 * var_sp_s_eta_dn8))),)
    } else {
        (var_sp_s_a, var_sp_s_a_dn5, var_sp_s_a_dn6, var_sp_s_a_dn7, var_sp_s_a_dn8,)
    }
};
        var_sp_s_a = assign41670_e54585;
        var_sp_s_a_dn5 = assign41670_e54585_d_n5;
        var_sp_s_a_dn6 = assign41670_e54585_d_n6;
        var_sp_s_a_dn7 = assign41670_e54585_d_n7;
        var_sp_s_a_dn8 = assign41670_e54585_d_n8;
        var_sp_s_a_rv = 0.0;

        let (assign41680_e54596, assign41680_e54596_d_n5, assign41680_e54596_d_n6, assign41680_e54596_d_n7, assign41680_e54596_d_n8,) = {
    if ((var_guard1182 == 0.0) && (var_guard1183 != 0.0)) {
        let assign41680_e54592: f64 = (2.0 * var_sp_s_temp);
        let assign41680_e54594: f64 = (assign41680_e54592 - var_gf2);
        (assign41680_e54594, ((2.0 * var_sp_s_temp_dn5) - var_gf2_dn5), ((2.0 * var_sp_s_temp_dn6) - var_gf2_dn6), ((2.0 * var_sp_s_temp_dn7) - var_gf2_dn7), ((2.0 * var_sp_s_temp_dn8) - var_gf2_dn8),)
    } else {
        (var_sp_s_c, var_sp_s_c_dn5, var_sp_s_c_dn6, var_sp_s_c_dn7, var_sp_s_c_dn8,)
    }
};
        var_sp_s_c = assign41680_e54596;
        var_sp_s_c_dn5 = assign41680_e54596_d_n5;
        var_sp_s_c_dn6 = assign41680_e54596_d_n6;
        var_sp_s_c_dn7 = assign41680_e54596_d_n7;
        var_sp_s_c_dn8 = assign41680_e54596_d_n8;
        var_sp_s_c_rv = 0.0;

        let (assign41690_e54609, assign41690_e54609_d_n5, assign41690_e54609_d_n6, assign41690_e54609_d_n7, assign41690_e54609_d_n8,) = {
    if ((var_guard1182 == 0.0) && (var_guard1183 != 0.0)) {
        let assign41690_e54602: f64 = (-var_sp_s_eta);
        let assign41690_e54605: f64 = (var_sp_s_a * var_inv_gf2);
        let assign41690_e54606: f64 = (assign41690_e54605).ln();
        let assign41690_e54607: f64 = (assign41690_e54602 + assign41690_e54606);
        (assign41690_e54607, ((-var_sp_s_eta_dn5) + (((var_sp_s_a_dn5 * var_inv_gf2) + (var_sp_s_a * var_inv_gf2_dn5)) / assign41690_e54605)), ((-var_sp_s_eta_dn6) + (((var_sp_s_a_dn6 * var_inv_gf2) + (var_sp_s_a * var_inv_gf2_dn6)) / assign41690_e54605)), ((-var_sp_s_eta_dn7) + (((var_sp_s_a_dn7 * var_inv_gf2) + (var_sp_s_a * var_inv_gf2_dn7)) / assign41690_e54605)), ((-var_sp_s_eta_dn8) + (((var_sp_s_a_dn8 * var_inv_gf2) + (var_sp_s_a * var_inv_gf2_dn8)) / assign41690_e54605)),)
    } else {
        (var_sp_s_tau, var_sp_s_tau_dn5, var_sp_s_tau_dn6, var_sp_s_tau_dn7, var_sp_s_tau_dn8,)
    }
};
        var_sp_s_tau = assign41690_e54609;
        var_sp_s_tau_dn5 = assign41690_e54609_d_n5;
        var_sp_s_tau_dn6 = assign41690_e54609_d_n6;
        var_sp_s_tau_dn7 = assign41690_e54609_d_n7;
        var_sp_s_tau_dn8 = assign41690_e54609_d_n8;
        var_sp_s_tau_rv = 0.0;

        let (assign41700_e54618, assign41700_e54618_d_n5, assign41700_e54618_d_n6, assign41700_e54618_d_n7, assign41700_e54618_d_n8,) = {
    if ((var_guard1182 == 0.0) && (var_guard1183 != 0.0)) {
        let assign41700_e54616: f64 = (var_sp_s_a + var_sp_s_c);
        (assign41700_e54616, (var_sp_s_a_dn5 + var_sp_s_c_dn5), (var_sp_s_a_dn6 + var_sp_s_c_dn6), (var_sp_s_a_dn7 + var_sp_s_c_dn7), (var_sp_s_a_dn8 + var_sp_s_c_dn8),)
    } else {
        (var_nu, var_nu_dn5, var_nu_dn6, var_nu_dn7, var_nu_dn8,)
    }
};
        var_nu = assign41700_e54618;
        var_nu_dn5 = assign41700_e54618_d_n5;
        var_nu_dn6 = assign41700_e54618_d_n6;
        var_nu_dn7 = assign41700_e54618_d_n7;
        var_nu_dn8 = assign41700_e54618_d_n8;
        var_nu_rv = 0.0;

        let (assign41710_e54637, assign41710_e54637_d_n5, assign41710_e54637_d_n6, assign41710_e54637_d_n7, assign41710_e54637_d_n8,) = {
    if ((var_guard1182 == 0.0) && (var_guard1183 != 0.0)) {
        let assign41710_e54625: f64 = (var_nu * var_nu);
        let assign41710_e54630: f64 = (var_sp_s_c * var_sp_s_c);
        let assign41710_e54631: f64 = (0.5 * assign41710_e54630);
        let assign41710_e54633: f64 = (assign41710_e54631 - var_sp_s_a);
        let assign41710_e54634: f64 = (var_sp_s_tau * assign41710_e54633);
        let assign41710_e54635: f64 = (assign41710_e54625 + assign41710_e54634);
        (assign41710_e54635, (((var_nu_dn5 * var_nu) + (var_nu * var_nu_dn5)) + ((var_sp_s_tau_dn5 * assign41710_e54633) + (var_sp_s_tau * ((0.5 * ((var_sp_s_c_dn5 * var_sp_s_c) + (var_sp_s_c * var_sp_s_c_dn5))) - var_sp_s_a_dn5)))), (((var_nu_dn6 * var_nu) + (var_nu * var_nu_dn6)) + ((var_sp_s_tau_dn6 * assign41710_e54633) + (var_sp_s_tau * ((0.5 * ((var_sp_s_c_dn6 * var_sp_s_c) + (var_sp_s_c * var_sp_s_c_dn6))) - var_sp_s_a_dn6)))), (((var_nu_dn7 * var_nu) + (var_nu * var_nu_dn7)) + ((var_sp_s_tau_dn7 * assign41710_e54633) + (var_sp_s_tau * ((0.5 * ((var_sp_s_c_dn7 * var_sp_s_c) + (var_sp_s_c * var_sp_s_c_dn7))) - var_sp_s_a_dn7)))), (((var_nu_dn8 * var_nu) + (var_nu * var_nu_dn8)) + ((var_sp_s_tau_dn8 * assign41710_e54633) + (var_sp_s_tau * ((0.5 * ((var_sp_s_c_dn8 * var_sp_s_c) + (var_sp_s_c * var_sp_s_c_dn8))) - var_sp_s_a_dn8)))),)
    } else {
        (var_mutau, var_mutau_dn5, var_mutau_dn6, var_mutau_dn7, var_mutau_dn8,)
    }
};
        var_mutau = assign41710_e54637;
        var_mutau_dn5 = assign41710_e54637_d_n5;
        var_mutau_dn6 = assign41710_e54637_d_n6;
        var_mutau_dn7 = assign41710_e54637_d_n7;
        var_mutau_dn8 = assign41710_e54637_d_n8;
        var_mutau_rv = 0.0;

        let (assign41720_e54670, assign41720_e54670_d_n5, assign41720_e54670_d_n6, assign41720_e54670_d_n7, assign41720_e54670_d_n8,) = {
    if ((var_guard1182 == 0.0) && (var_guard1183 != 0.0)) {
        let assign41720_e54645: f64 = (var_sp_s_a * var_nu);
        let assign41720_e54647: f64 = (assign41720_e54645 * var_sp_s_tau);
        let assign41720_e54651: f64 = (var_nu / var_mutau);
        let assign41720_e54653: f64 = (assign41720_e54651 * var_sp_s_tau);
        let assign41720_e54655: f64 = (assign41720_e54653 * var_sp_s_tau);
        let assign41720_e54657: f64 = (assign41720_e54655 * var_sp_s_c);
        let assign41720_e54660: f64 = (var_sp_s_c * var_sp_s_c);
        let assign41720_e54662: f64 = (assign41720_e54660 * 0.3333333333333333);
        let assign41720_e54664: f64 = (assign41720_e54662 - var_sp_s_a);
        let assign41720_e54665: f64 = (assign41720_e54657 * assign41720_e54664);
        let assign41720_e54666: f64 = (var_mutau + assign41720_e54665);
        let assign41720_e54667: f64 = (assign41720_e54647 / assign41720_e54666);
        let assign41720_e54668: f64 = (var_sp_s_eta + assign41720_e54667);
        (assign41720_e54668, (var_sp_s_eta_dn5 + (((((((var_sp_s_a_dn5 * var_nu) + (var_sp_s_a * var_nu_dn5)) * var_sp_s_tau) + (assign41720_e54645 * var_sp_s_tau_dn5)) * assign41720_e54666) - (assign41720_e54647 * (var_mutau_dn5 + (((((((((((var_nu_dn5 * var_mutau) - (var_nu * var_mutau_dn5)) / (var_mutau * var_mutau)) * var_sp_s_tau) + (assign41720_e54651 * var_sp_s_tau_dn5)) * var_sp_s_tau) + (assign41720_e54653 * var_sp_s_tau_dn5)) * var_sp_s_c) + (assign41720_e54655 * var_sp_s_c_dn5)) * assign41720_e54664) + (assign41720_e54657 * ((((var_sp_s_c_dn5 * var_sp_s_c) + (var_sp_s_c * var_sp_s_c_dn5)) * 0.3333333333333333) - var_sp_s_a_dn5)))))) / (assign41720_e54666 * assign41720_e54666))), (var_sp_s_eta_dn6 + (((((((var_sp_s_a_dn6 * var_nu) + (var_sp_s_a * var_nu_dn6)) * var_sp_s_tau) + (assign41720_e54645 * var_sp_s_tau_dn6)) * assign41720_e54666) - (assign41720_e54647 * (var_mutau_dn6 + (((((((((((var_nu_dn6 * var_mutau) - (var_nu * var_mutau_dn6)) / (var_mutau * var_mutau)) * var_sp_s_tau) + (assign41720_e54651 * var_sp_s_tau_dn6)) * var_sp_s_tau) + (assign41720_e54653 * var_sp_s_tau_dn6)) * var_sp_s_c) + (assign41720_e54655 * var_sp_s_c_dn6)) * assign41720_e54664) + (assign41720_e54657 * ((((var_sp_s_c_dn6 * var_sp_s_c) + (var_sp_s_c * var_sp_s_c_dn6)) * 0.3333333333333333) - var_sp_s_a_dn6)))))) / (assign41720_e54666 * assign41720_e54666))), (var_sp_s_eta_dn7 + (((((((var_sp_s_a_dn7 * var_nu) + (var_sp_s_a * var_nu_dn7)) * var_sp_s_tau) + (assign41720_e54645 * var_sp_s_tau_dn7)) * assign41720_e54666) - (assign41720_e54647 * (var_mutau_dn7 + (((((((((((var_nu_dn7 * var_mutau) - (var_nu * var_mutau_dn7)) / (var_mutau * var_mutau)) * var_sp_s_tau) + (assign41720_e54651 * var_sp_s_tau_dn7)) * var_sp_s_tau) + (assign41720_e54653 * var_sp_s_tau_dn7)) * var_sp_s_c) + (assign41720_e54655 * var_sp_s_c_dn7)) * assign41720_e54664) + (assign41720_e54657 * ((((var_sp_s_c_dn7 * var_sp_s_c) + (var_sp_s_c * var_sp_s_c_dn7)) * 0.3333333333333333) - var_sp_s_a_dn7)))))) / (assign41720_e54666 * assign41720_e54666))), (var_sp_s_eta_dn8 + (((((((var_sp_s_a_dn8 * var_nu) + (var_sp_s_a * var_nu_dn8)) * var_sp_s_tau) + (assign41720_e54645 * var_sp_s_tau_dn8)) * assign41720_e54666) - (assign41720_e54647 * (var_mutau_dn8 + (((((((((((var_nu_dn8 * var_mutau) - (var_nu * var_mutau_dn8)) / (var_mutau * var_mutau)) * var_sp_s_tau) + (assign41720_e54651 * var_sp_s_tau_dn8)) * var_sp_s_tau) + (assign41720_e54653 * var_sp_s_tau_dn8)) * var_sp_s_c) + (assign41720_e54655 * var_sp_s_c_dn8)) * assign41720_e54664) + (assign41720_e54657 * ((((var_sp_s_c_dn8 * var_sp_s_c) + (var_sp_s_c * var_sp_s_c_dn8)) * 0.3333333333333333) - var_sp_s_a_dn8)))))) / (assign41720_e54666 * assign41720_e54666))),)
    } else {
        (var_sp_s_y0, var_sp_s_y0_dn5, var_sp_s_y0_dn6, var_sp_s_y0_dn7, var_sp_s_y0_dn8,)
    }
};
        var_sp_s_y0 = assign41720_e54670;
        var_sp_s_y0_dn5 = assign41720_e54670_d_n5;
        var_sp_s_y0_dn6 = assign41720_e54670_d_n6;
        var_sp_s_y0_dn7 = assign41720_e54670_d_n7;
        var_sp_s_y0_dn8 = assign41720_e54670_d_n8;
        var_sp_s_y0_rv = 0.0;

        let assign41730_e54673: f64 = if var_sp_s_y0 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard1184 = assign41730_e54673;
        var_guard1184_rv = 0.0;

        let (assign41740_e54683, assign41740_e54683_d_n5, assign41740_e54683_d_n6, assign41740_e54683_d_n7, assign41740_e54683_d_n8,) = {
    if (((var_guard1182 == 0.0) && (var_guard1183 != 0.0)) && (var_guard1184 != 0.0)) {
        let assign41740_e54681: f64 = (var_sp_s_y0).exp();
        (assign41740_e54681, (assign41740_e54681 * var_sp_s_y0_dn5), (assign41740_e54681 * var_sp_s_y0_dn6), (assign41740_e54681 * var_sp_s_y0_dn7), (assign41740_e54681 * var_sp_s_y0_dn8),)
    } else {
        (var_sp_s_delta0, var_sp_s_delta0_dn5, var_sp_s_delta0_dn6, var_sp_s_delta0_dn7, var_sp_s_delta0_dn8,)
    }
};
        var_sp_s_delta0 = assign41740_e54683;
        var_sp_s_delta0_dn5 = assign41740_e54683_d_n5;
        var_sp_s_delta0_dn6 = assign41740_e54683_d_n6;
        var_sp_s_delta0_dn7 = assign41740_e54683_d_n7;
        var_sp_s_delta0_dn8 = assign41740_e54683_d_n8;
        var_sp_s_delta0_rv = 0.0;

        let (assign41750_e54715, assign41750_e54715_d_n5, assign41750_e54715_d_n6, assign41750_e54715_d_n7, assign41750_e54715_d_n8,) = {
    if (((var_guard1182 == 0.0) && (var_guard1183 != 0.0)) && (var_guard1184 == 0.0)) {
        let assign41750_e54695: f64 = (var_sp_s_y0 - 230.25850929940458);
        let assign41750_e54700: f64 = (var_sp_s_y0 - 230.25850929940458);
        let assign41750_e54704: f64 = (var_sp_s_y0 - 230.25850929940458);
        let assign41750_e54706: f64 = (assign41750_e54704 * 0.3333333333333333);
        let assign41750_e54707: f64 = (1.0 + assign41750_e54706);
        let assign41750_e54708: f64 = (assign41750_e54700 * assign41750_e54707);
        let assign41750_e54709: f64 = (0.5 * assign41750_e54708);
        let assign41750_e54710: f64 = (1.0 + assign41750_e54709);
        let assign41750_e54711: f64 = (assign41750_e54695 * assign41750_e54710);
        let assign41750_e54712: f64 = (1.0 + assign41750_e54711);
        let assign41750_e54713: f64 = (1e100 * assign41750_e54712);
        (assign41750_e54713, (1e100 * ((var_sp_s_y0_dn5 * assign41750_e54710) + (assign41750_e54695 * (0.5 * ((var_sp_s_y0_dn5 * assign41750_e54707) + (assign41750_e54700 * (var_sp_s_y0_dn5 * 0.3333333333333333))))))), (1e100 * ((var_sp_s_y0_dn6 * assign41750_e54710) + (assign41750_e54695 * (0.5 * ((var_sp_s_y0_dn6 * assign41750_e54707) + (assign41750_e54700 * (var_sp_s_y0_dn6 * 0.3333333333333333))))))), (1e100 * ((var_sp_s_y0_dn7 * assign41750_e54710) + (assign41750_e54695 * (0.5 * ((var_sp_s_y0_dn7 * assign41750_e54707) + (assign41750_e54700 * (var_sp_s_y0_dn7 * 0.3333333333333333))))))), (1e100 * ((var_sp_s_y0_dn8 * assign41750_e54710) + (assign41750_e54695 * (0.5 * ((var_sp_s_y0_dn8 * assign41750_e54707) + (assign41750_e54700 * (var_sp_s_y0_dn8 * 0.3333333333333333))))))),)
    } else {
        (var_sp_s_delta0, var_sp_s_delta0_dn5, var_sp_s_delta0_dn6, var_sp_s_delta0_dn7, var_sp_s_delta0_dn8,)
    }
};
        var_sp_s_delta0 = assign41750_e54715;
        var_sp_s_delta0_dn5 = assign41750_e54715_d_n5;
        var_sp_s_delta0_dn6 = assign41750_e54715_d_n6;
        var_sp_s_delta0_dn7 = assign41750_e54715_d_n7;
        var_sp_s_delta0_dn8 = assign41750_e54715_d_n8;
        var_sp_s_delta0_rv = 0.0;

        let (assign41760_e54724, assign41760_e54724_d_n5, assign41760_e54724_d_n6, assign41760_e54724_d_n7, assign41760_e54724_d_n8,) = {
    if ((var_guard1182 == 0.0) && (var_guard1183 != 0.0)) {
        let assign41760_e54722: f64 = (1.0 / var_sp_s_delta0);
        (assign41760_e54722, (-(var_sp_s_delta0_dn5 / (var_sp_s_delta0 * var_sp_s_delta0))), (-(var_sp_s_delta0_dn6 / (var_sp_s_delta0 * var_sp_s_delta0))), (-(var_sp_s_delta0_dn7 / (var_sp_s_delta0 * var_sp_s_delta0))), (-(var_sp_s_delta0_dn8 / (var_sp_s_delta0 * var_sp_s_delta0))),)
    } else {
        (var_sp_s_delta1, var_sp_s_delta1_dn5, var_sp_s_delta1_dn6, var_sp_s_delta1_dn7, var_sp_s_delta1_dn8,)
    }
};
        var_sp_s_delta1 = assign41760_e54724;
        var_sp_s_delta1_dn5 = assign41760_e54724_d_n5;
        var_sp_s_delta1_dn6 = assign41760_e54724_d_n6;
        var_sp_s_delta1_dn7 = assign41760_e54724_d_n7;
        var_sp_s_delta1_dn8 = assign41760_e54724_d_n8;
        var_sp_s_delta1_rv = 0.0;

        let (assign41770_e54737, assign41770_e54737_d_n5, assign41770_e54737_d_n6, assign41770_e54737_d_n7, assign41770_e54737_d_n8,) = {
    if ((var_guard1182 == 0.0) && (var_guard1183 != 0.0)) {
        let assign41770_e54733: f64 = (var_sp_s_y0 * var_sp_s_y0);
        let assign41770_e54734: f64 = (2.0 + assign41770_e54733);
        let assign41770_e54735: f64 = (1.0 / assign41770_e54734);
        (assign41770_e54735, (-(((var_sp_s_y0_dn5 * var_sp_s_y0) + (var_sp_s_y0 * var_sp_s_y0_dn5)) / (assign41770_e54734 * assign41770_e54734))), (-(((var_sp_s_y0_dn6 * var_sp_s_y0) + (var_sp_s_y0 * var_sp_s_y0_dn6)) / (assign41770_e54734 * assign41770_e54734))), (-(((var_sp_s_y0_dn7 * var_sp_s_y0) + (var_sp_s_y0 * var_sp_s_y0_dn7)) / (assign41770_e54734 * assign41770_e54734))), (-(((var_sp_s_y0_dn8 * var_sp_s_y0) + (var_sp_s_y0 * var_sp_s_y0_dn8)) / (assign41770_e54734 * assign41770_e54734))),)
    } else {
        (var_sp_s_temp, var_sp_s_temp_dn5, var_sp_s_temp_dn6, var_sp_s_temp_dn7, var_sp_s_temp_dn8,)
    }
};
        var_sp_s_temp = assign41770_e54737;
        var_sp_s_temp_dn5 = assign41770_e54737_d_n5;
        var_sp_s_temp_dn6 = assign41770_e54737_d_n6;
        var_sp_s_temp_dn7 = assign41770_e54737_d_n7;
        var_sp_s_temp_dn8 = assign41770_e54737_d_n8;
        var_sp_s_temp_rv = 0.0;

        let (assign41780_e54748, assign41780_e54748_d_n5, assign41780_e54748_d_n6, assign41780_e54748_d_n7, assign41780_e54748_d_n8,) = {
    if ((var_guard1182 == 0.0) && (var_guard1183 != 0.0)) {
        let assign41780_e54744: f64 = (var_sp_s_y0 * var_sp_s_y0);
        let assign41780_e54746: f64 = (assign41780_e54744 * var_sp_s_temp);
        (assign41780_e54746, ((((var_sp_s_y0_dn5 * var_sp_s_y0) + (var_sp_s_y0 * var_sp_s_y0_dn5)) * var_sp_s_temp) + (assign41780_e54744 * var_sp_s_temp_dn5)), ((((var_sp_s_y0_dn6 * var_sp_s_y0) + (var_sp_s_y0 * var_sp_s_y0_dn6)) * var_sp_s_temp) + (assign41780_e54744 * var_sp_s_temp_dn6)), ((((var_sp_s_y0_dn7 * var_sp_s_y0) + (var_sp_s_y0 * var_sp_s_y0_dn7)) * var_sp_s_temp) + (assign41780_e54744 * var_sp_s_temp_dn7)), ((((var_sp_s_y0_dn8 * var_sp_s_y0) + (var_sp_s_y0 * var_sp_s_y0_dn8)) * var_sp_s_temp) + (assign41780_e54744 * var_sp_s_temp_dn8)),)
    } else {
        (var_sp_s_xi0, var_sp_s_xi0_dn5, var_sp_s_xi0_dn6, var_sp_s_xi0_dn7, var_sp_s_xi0_dn8,)
    }
};
        var_sp_s_xi0 = assign41780_e54748;
        var_sp_s_xi0_dn5 = assign41780_e54748_d_n5;
        var_sp_s_xi0_dn6 = assign41780_e54748_d_n6;
        var_sp_s_xi0_dn7 = assign41780_e54748_d_n7;
        var_sp_s_xi0_dn8 = assign41780_e54748_d_n8;
        var_sp_s_xi0_rv = 0.0;

        let (assign41790_e54761, assign41790_e54761_d_n5, assign41790_e54761_d_n6, assign41790_e54761_d_n7, assign41790_e54761_d_n8,) = {
    if ((var_guard1182 == 0.0) && (var_guard1183 != 0.0)) {
        let assign41790_e54756: f64 = (var_sp_s_y0 * var_sp_s_temp);
        let assign41790_e54758: f64 = (assign41790_e54756 * var_sp_s_temp);
        let assign41790_e54759: f64 = (4.0 * assign41790_e54758);
        (assign41790_e54759, (4.0 * ((((var_sp_s_y0_dn5 * var_sp_s_temp) + (var_sp_s_y0 * var_sp_s_temp_dn5)) * var_sp_s_temp) + (assign41790_e54756 * var_sp_s_temp_dn5))), (4.0 * ((((var_sp_s_y0_dn6 * var_sp_s_temp) + (var_sp_s_y0 * var_sp_s_temp_dn6)) * var_sp_s_temp) + (assign41790_e54756 * var_sp_s_temp_dn6))), (4.0 * ((((var_sp_s_y0_dn7 * var_sp_s_temp) + (var_sp_s_y0 * var_sp_s_temp_dn7)) * var_sp_s_temp) + (assign41790_e54756 * var_sp_s_temp_dn7))), (4.0 * ((((var_sp_s_y0_dn8 * var_sp_s_temp) + (var_sp_s_y0 * var_sp_s_temp_dn8)) * var_sp_s_temp) + (assign41790_e54756 * var_sp_s_temp_dn8))),)
    } else {
        (var_sp_s_xi1, var_sp_s_xi1_dn5, var_sp_s_xi1_dn6, var_sp_s_xi1_dn7, var_sp_s_xi1_dn8,)
    }
};
        var_sp_s_xi1 = assign41790_e54761;
        var_sp_s_xi1_dn5 = assign41790_e54761_d_n5;
        var_sp_s_xi1_dn6 = assign41790_e54761_d_n6;
        var_sp_s_xi1_dn7 = assign41790_e54761_d_n7;
        var_sp_s_xi1_dn8 = assign41790_e54761_d_n8;
        var_sp_s_xi1_rv = 0.0;

        let (assign41800_e54778, assign41800_e54778_d_n5, assign41800_e54778_d_n6, assign41800_e54778_d_n7, assign41800_e54778_d_n8,) = {
    if ((var_guard1182 == 0.0) && (var_guard1183 != 0.0)) {
        let assign41800_e54768: f64 = (8.0 * var_sp_s_temp);
        let assign41800_e54771: f64 = (12.0 * var_sp_s_xi0);
        let assign41800_e54772: f64 = (assign41800_e54768 - assign41800_e54771);
        let assign41800_e54774: f64 = (assign41800_e54772 * var_sp_s_temp);
        let assign41800_e54776: f64 = (assign41800_e54774 * var_sp_s_temp);
        (assign41800_e54776, ((((((8.0 * var_sp_s_temp_dn5) - (12.0 * var_sp_s_xi0_dn5)) * var_sp_s_temp) + (assign41800_e54772 * var_sp_s_temp_dn5)) * var_sp_s_temp) + (assign41800_e54774 * var_sp_s_temp_dn5)), ((((((8.0 * var_sp_s_temp_dn6) - (12.0 * var_sp_s_xi0_dn6)) * var_sp_s_temp) + (assign41800_e54772 * var_sp_s_temp_dn6)) * var_sp_s_temp) + (assign41800_e54774 * var_sp_s_temp_dn6)), ((((((8.0 * var_sp_s_temp_dn7) - (12.0 * var_sp_s_xi0_dn7)) * var_sp_s_temp) + (assign41800_e54772 * var_sp_s_temp_dn7)) * var_sp_s_temp) + (assign41800_e54774 * var_sp_s_temp_dn7)), ((((((8.0 * var_sp_s_temp_dn8) - (12.0 * var_sp_s_xi0_dn8)) * var_sp_s_temp) + (assign41800_e54772 * var_sp_s_temp_dn8)) * var_sp_s_temp) + (assign41800_e54774 * var_sp_s_temp_dn8)),)
    } else {
        (var_sp_s_xi2, var_sp_s_xi2_dn5, var_sp_s_xi2_dn6, var_sp_s_xi2_dn7, var_sp_s_xi2_dn8,)
    }
};
        var_sp_s_xi2 = assign41800_e54778;
        var_sp_s_xi2_dn5 = assign41800_e54778_d_n5;
        var_sp_s_xi2_dn6 = assign41800_e54778_d_n6;
        var_sp_s_xi2_dn7 = assign41800_e54778_d_n7;
        var_sp_s_xi2_dn8 = assign41800_e54778_d_n8;
        var_sp_s_xi2_rv = 0.0;

        let (assign41810_e54787, assign41810_e54787_d_n5, assign41810_e54787_d_n6, assign41810_e54787_d_n7, assign41810_e54787_d_n8,) = {
    if ((var_guard1182 == 0.0) && (var_guard1183 != 0.0)) {
        let assign41810_e54785: f64 = (var_sp_s_yg - var_sp_s_y0);
        (assign41810_e54785, (var_sp_s_yg_dn5 - var_sp_s_y0_dn5), (var_sp_s_yg_dn6 - var_sp_s_y0_dn6), (var_sp_s_yg_dn7 - var_sp_s_y0_dn7), (var_sp_s_yg_dn8 - var_sp_s_y0_dn8),)
    } else {
        (var_sp_s_temp, var_sp_s_temp_dn5, var_sp_s_temp_dn6, var_sp_s_temp_dn7, var_sp_s_temp_dn8,)
    }
};
        var_sp_s_temp = assign41810_e54787;
        var_sp_s_temp_dn5 = assign41810_e54787_d_n5;
        var_sp_s_temp_dn6 = assign41810_e54787_d_n6;
        var_sp_s_temp_dn7 = assign41810_e54787_d_n7;
        var_sp_s_temp_dn8 = assign41810_e54787_d_n8;
        var_sp_s_temp_rv = 0.0;

        let (assign41820_e54796, assign41820_e54796_d_n5, assign41820_e54796_d_n6, assign41820_e54796_d_n7, assign41820_e54796_d_n8,) = {
    if ((var_guard1182 == 0.0) && (var_guard1183 != 0.0)) {
        let assign41820_e54794: f64 = (var_delta_ns * var_sp_s_delta1);
        (assign41820_e54794, ((var_delta_ns_dn5 * var_sp_s_delta1) + (var_delta_ns * var_sp_s_delta1_dn5)), ((var_delta_ns_dn6 * var_sp_s_delta1) + (var_delta_ns * var_sp_s_delta1_dn6)), ((var_delta_ns_dn7 * var_sp_s_delta1) + (var_delta_ns * var_sp_s_delta1_dn7)), ((var_delta_ns_dn8 * var_sp_s_delta1) + (var_delta_ns * var_sp_s_delta1_dn8)),)
    } else {
        (var_sp_s_temp1, var_sp_s_temp1_dn5, var_sp_s_temp1_dn6, var_sp_s_temp1_dn7, var_sp_s_temp1_dn8,)
    }
};
        var_sp_s_temp1 = assign41820_e54796;
        var_sp_s_temp1_dn5 = assign41820_e54796_d_n5;
        var_sp_s_temp1_dn6 = assign41820_e54796_d_n6;
        var_sp_s_temp1_dn7 = assign41820_e54796_d_n7;
        var_sp_s_temp1_dn8 = assign41820_e54796_d_n8;
        var_sp_s_temp1_rv = 0.0;

        let (assign41830_e54819, assign41830_e54819_d_n5, assign41830_e54819_d_n6, assign41830_e54819_d_n7, assign41830_e54819_d_n8,) = {
    if ((var_guard1182 == 0.0) && (var_guard1183 != 0.0)) {
        let assign41830_e54803: f64 = (2.0 * var_sp_s_temp);
        let assign41830_e54807: f64 = (var_sp_s_delta0 - 1.0);
        let assign41830_e54809: f64 = (assign41830_e54807 - var_sp_s_temp1);
        let assign41830_e54813: f64 = (1.0 - var_sp_s_xi1);
        let assign41830_e54814: f64 = (var_delta_ns * assign41830_e54813);
        let assign41830_e54815: f64 = (assign41830_e54809 + assign41830_e54814);
        let assign41830_e54816: f64 = (var_gf2 * assign41830_e54815);
        let assign41830_e54817: f64 = (assign41830_e54803 + assign41830_e54816);
        (assign41830_e54817, ((2.0 * var_sp_s_temp_dn5) + ((var_gf2_dn5 * assign41830_e54815) + (var_gf2 * ((var_sp_s_delta0_dn5 - var_sp_s_temp1_dn5) + ((var_delta_ns_dn5 * assign41830_e54813) + (var_delta_ns * (-var_sp_s_xi1_dn5))))))), ((2.0 * var_sp_s_temp_dn6) + ((var_gf2_dn6 * assign41830_e54815) + (var_gf2 * ((var_sp_s_delta0_dn6 - var_sp_s_temp1_dn6) + ((var_delta_ns_dn6 * assign41830_e54813) + (var_delta_ns * (-var_sp_s_xi1_dn6))))))), ((2.0 * var_sp_s_temp_dn7) + ((var_gf2_dn7 * assign41830_e54815) + (var_gf2 * ((var_sp_s_delta0_dn7 - var_sp_s_temp1_dn7) + ((var_delta_ns_dn7 * assign41830_e54813) + (var_delta_ns * (-var_sp_s_xi1_dn7))))))), ((2.0 * var_sp_s_temp_dn8) + ((var_gf2_dn8 * assign41830_e54815) + (var_gf2 * ((var_sp_s_delta0_dn8 - var_sp_s_temp1_dn8) + ((var_delta_ns_dn8 * assign41830_e54813) + (var_delta_ns * (-var_sp_s_xi1_dn8))))))),)
    } else {
        (var_sp_s_pc, var_sp_s_pc_dn5, var_sp_s_pc_dn6, var_sp_s_pc_dn7, var_sp_s_pc_dn8,)
    }
};
        var_sp_s_pc = assign41830_e54819;
        var_sp_s_pc_dn5 = assign41830_e54819_d_n5;
        var_sp_s_pc_dn6 = assign41830_e54819_d_n6;
        var_sp_s_pc_dn7 = assign41830_e54819_d_n7;
        var_sp_s_pc_dn8 = assign41830_e54819_d_n8;
        var_sp_s_pc_rv = 0.0;

        let (assign41840_e54846, assign41840_e54846_d_n5, assign41840_e54846_d_n6, assign41840_e54846_d_n7, assign41840_e54846_d_n8,) = {
    if ((var_guard1182 == 0.0) && (var_guard1183 != 0.0)) {
        let assign41840_e54826: f64 = (var_sp_s_temp * var_sp_s_temp);
        let assign41840_e54830: f64 = (var_sp_s_delta0 - var_sp_s_y0);
        let assign41840_e54832: f64 = (assign41840_e54830 - 1.0);
        let assign41840_e54834: f64 = (assign41840_e54832 + var_sp_s_temp1);
        let assign41840_e54838: f64 = (var_sp_s_y0 - 1.0);
        let assign41840_e54840: f64 = (assign41840_e54838 - var_sp_s_xi0);
        let assign41840_e54841: f64 = (var_delta_ns * assign41840_e54840);
        let assign41840_e54842: f64 = (assign41840_e54834 + assign41840_e54841);
        let assign41840_e54843: f64 = (var_gf2 * assign41840_e54842);
        let assign41840_e54844: f64 = (assign41840_e54826 - assign41840_e54843);
        (assign41840_e54844, (((var_sp_s_temp_dn5 * var_sp_s_temp) + (var_sp_s_temp * var_sp_s_temp_dn5)) - ((var_gf2_dn5 * assign41840_e54842) + (var_gf2 * (((var_sp_s_delta0_dn5 - var_sp_s_y0_dn5) + var_sp_s_temp1_dn5) + ((var_delta_ns_dn5 * assign41840_e54840) + (var_delta_ns * (var_sp_s_y0_dn5 - var_sp_s_xi0_dn5))))))), (((var_sp_s_temp_dn6 * var_sp_s_temp) + (var_sp_s_temp * var_sp_s_temp_dn6)) - ((var_gf2_dn6 * assign41840_e54842) + (var_gf2 * (((var_sp_s_delta0_dn6 - var_sp_s_y0_dn6) + var_sp_s_temp1_dn6) + ((var_delta_ns_dn6 * assign41840_e54840) + (var_delta_ns * (var_sp_s_y0_dn6 - var_sp_s_xi0_dn6))))))), (((var_sp_s_temp_dn7 * var_sp_s_temp) + (var_sp_s_temp * var_sp_s_temp_dn7)) - ((var_gf2_dn7 * assign41840_e54842) + (var_gf2 * (((var_sp_s_delta0_dn7 - var_sp_s_y0_dn7) + var_sp_s_temp1_dn7) + ((var_delta_ns_dn7 * assign41840_e54840) + (var_delta_ns * (var_sp_s_y0_dn7 - var_sp_s_xi0_dn7))))))), (((var_sp_s_temp_dn8 * var_sp_s_temp) + (var_sp_s_temp * var_sp_s_temp_dn8)) - ((var_gf2_dn8 * assign41840_e54842) + (var_gf2 * (((var_sp_s_delta0_dn8 - var_sp_s_y0_dn8) + var_sp_s_temp1_dn8) + ((var_delta_ns_dn8 * assign41840_e54840) + (var_delta_ns * (var_sp_s_y0_dn8 - var_sp_s_xi0_dn8))))))),)
    } else {
        (var_sp_s_qc, var_sp_s_qc_dn5, var_sp_s_qc_dn6, var_sp_s_qc_dn7, var_sp_s_qc_dn8,)
    }
};
        var_sp_s_qc = assign41840_e54846;
        var_sp_s_qc_dn5 = assign41840_e54846_d_n5;
        var_sp_s_qc_dn6 = assign41840_e54846_d_n6;
        var_sp_s_qc_dn7 = assign41840_e54846_d_n7;
        var_sp_s_qc_dn8 = assign41840_e54846_d_n8;
        var_sp_s_qc_rv = 0.0;

        let (assign41850_e54863, assign41850_e54863_d_n5, assign41850_e54863_d_n6, assign41850_e54863_d_n7, assign41850_e54863_d_n8,) = {
    if ((var_guard1182 == 0.0) && (var_guard1183 != 0.0)) {
        let assign41850_e54855: f64 = (var_sp_s_delta0 + var_sp_s_temp1);
        let assign41850_e54858: f64 = (var_delta_ns * var_sp_s_xi2);
        let assign41850_e54859: f64 = (assign41850_e54855 - assign41850_e54858);
        let assign41850_e54860: f64 = (var_gf2 * assign41850_e54859);
        let assign41850_e54861: f64 = (2.0 - assign41850_e54860);
        (assign41850_e54861, (-((var_gf2_dn5 * assign41850_e54859) + (var_gf2 * ((var_sp_s_delta0_dn5 + var_sp_s_temp1_dn5) - ((var_delta_ns_dn5 * var_sp_s_xi2) + (var_delta_ns * var_sp_s_xi2_dn5)))))), (-((var_gf2_dn6 * assign41850_e54859) + (var_gf2 * ((var_sp_s_delta0_dn6 + var_sp_s_temp1_dn6) - ((var_delta_ns_dn6 * var_sp_s_xi2) + (var_delta_ns * var_sp_s_xi2_dn6)))))), (-((var_gf2_dn7 * assign41850_e54859) + (var_gf2 * ((var_sp_s_delta0_dn7 + var_sp_s_temp1_dn7) - ((var_delta_ns_dn7 * var_sp_s_xi2) + (var_delta_ns * var_sp_s_xi2_dn7)))))), (-((var_gf2_dn8 * assign41850_e54859) + (var_gf2 * ((var_sp_s_delta0_dn8 + var_sp_s_temp1_dn8) - ((var_delta_ns_dn8 * var_sp_s_xi2) + (var_delta_ns * var_sp_s_xi2_dn8)))))),)
    } else {
        (var_sp_s_temp, var_sp_s_temp_dn5, var_sp_s_temp_dn6, var_sp_s_temp_dn7, var_sp_s_temp_dn8,)
    }
};
        var_sp_s_temp = assign41850_e54863;
        var_sp_s_temp_dn5 = assign41850_e54863_d_n5;
        var_sp_s_temp_dn6 = assign41850_e54863_d_n6;
        var_sp_s_temp_dn7 = assign41850_e54863_d_n7;
        var_sp_s_temp_dn8 = assign41850_e54863_d_n8;
        var_sp_s_temp_rv = 0.0;

        let (assign41860_e54878, assign41860_e54878_d_n5, assign41860_e54878_d_n6, assign41860_e54878_d_n7, assign41860_e54878_d_n8,) = {
    if ((var_guard1182 == 0.0) && (var_guard1183 != 0.0)) {
        let assign41860_e54870: f64 = (var_sp_s_pc * var_sp_s_pc);
        let assign41860_e54874: f64 = (var_sp_s_qc * var_sp_s_temp);
        let assign41860_e54875: f64 = (2.0 * assign41860_e54874);
        let assign41860_e54876: f64 = (assign41860_e54870 - assign41860_e54875);
        (assign41860_e54876, (((var_sp_s_pc_dn5 * var_sp_s_pc) + (var_sp_s_pc * var_sp_s_pc_dn5)) - (2.0 * ((var_sp_s_qc_dn5 * var_sp_s_temp) + (var_sp_s_qc * var_sp_s_temp_dn5)))), (((var_sp_s_pc_dn6 * var_sp_s_pc) + (var_sp_s_pc * var_sp_s_pc_dn6)) - (2.0 * ((var_sp_s_qc_dn6 * var_sp_s_temp) + (var_sp_s_qc * var_sp_s_temp_dn6)))), (((var_sp_s_pc_dn7 * var_sp_s_pc) + (var_sp_s_pc * var_sp_s_pc_dn7)) - (2.0 * ((var_sp_s_qc_dn7 * var_sp_s_temp) + (var_sp_s_qc * var_sp_s_temp_dn7)))), (((var_sp_s_pc_dn8 * var_sp_s_pc) + (var_sp_s_pc * var_sp_s_pc_dn8)) - (2.0 * ((var_sp_s_qc_dn8 * var_sp_s_temp) + (var_sp_s_qc * var_sp_s_temp_dn8)))),)
    } else {
        (var_sp_s_temp, var_sp_s_temp_dn5, var_sp_s_temp_dn6, var_sp_s_temp_dn7, var_sp_s_temp_dn8,)
    }
};
        var_sp_s_temp = assign41860_e54878;
        var_sp_s_temp_dn5 = assign41860_e54878_d_n5;
        var_sp_s_temp_dn6 = assign41860_e54878_d_n6;
        var_sp_s_temp_dn7 = assign41860_e54878_d_n7;
        var_sp_s_temp_dn8 = assign41860_e54878_d_n8;
        var_sp_s_temp_rv = 0.0;

        let (assign41870_e54895, assign41870_e54895_d_n5, assign41870_e54895_d_n6, assign41870_e54895_d_n7, assign41870_e54895_d_n8,) = {
    if ((var_guard1182 == 0.0) && (var_guard1183 != 0.0)) {
        let assign41870_e54884: f64 = (-var_sp_s_y0);
        let assign41870_e54889: f64 = (var_sp_s_temp).sqrt();
        let assign41870_e54890: f64 = (var_sp_s_pc + assign41870_e54889);
        let assign41870_e54891: f64 = (var_sp_s_qc / assign41870_e54890);
        let assign41870_e54892: f64 = (2.0 * assign41870_e54891);
        let assign41870_e54893: f64 = (assign41870_e54884 - assign41870_e54892);
        (assign41870_e54893, ((-var_sp_s_y0_dn5) - (2.0 * (((var_sp_s_qc_dn5 * assign41870_e54890) - (var_sp_s_qc * (var_sp_s_pc_dn5 + (var_sp_s_temp_dn5 / (2.0 * assign41870_e54889))))) / (assign41870_e54890 * assign41870_e54890)))), ((-var_sp_s_y0_dn6) - (2.0 * (((var_sp_s_qc_dn6 * assign41870_e54890) - (var_sp_s_qc * (var_sp_s_pc_dn6 + (var_sp_s_temp_dn6 / (2.0 * assign41870_e54889))))) / (assign41870_e54890 * assign41870_e54890)))), ((-var_sp_s_y0_dn7) - (2.0 * (((var_sp_s_qc_dn7 * assign41870_e54890) - (var_sp_s_qc * (var_sp_s_pc_dn7 + (var_sp_s_temp_dn7 / (2.0 * assign41870_e54889))))) / (assign41870_e54890 * assign41870_e54890)))), ((-var_sp_s_y0_dn8) - (2.0 * (((var_sp_s_qc_dn8 * assign41870_e54890) - (var_sp_s_qc * (var_sp_s_pc_dn8 + (var_sp_s_temp_dn8 / (2.0 * assign41870_e54889))))) / (assign41870_e54890 * assign41870_e54890)))),)
    } else {
        (var_x_s, var_x_s_dn5, var_x_s_dn6, var_x_s_dn7, var_x_s_dn8,)
    }
};
        var_x_s = assign41870_e54895;
        var_x_s_dn5 = assign41870_e54895_d_n5;
        var_x_s_dn6 = assign41870_e54895_d_n6;
        var_x_s_dn7 = assign41870_e54895_d_n7;
        var_x_s_dn8 = assign41870_e54895_d_n8;
        var_x_s_rv = 0.0;

        let (assign41880_e54909, assign41880_e54909_d_n5, assign41880_e54909_d_n6, assign41880_e54909_d_n7, assign41880_e54909_d_n8,) = {
    if ((var_guard1182 == 0.0) && (var_guard1183 == 0.0)) {
        let assign41880_e54905: f64 = (var_gf * 0.7324648775608221);
        let assign41880_e54906: f64 = (1.25 + assign41880_e54905);
        let assign41880_e54907: f64 = (1.0 / assign41880_e54906);
        (assign41880_e54907, (-((var_gf_dn5 * 0.7324648775608221) / (assign41880_e54906 * assign41880_e54906))), (-((var_gf_dn6 * 0.7324648775608221) / (assign41880_e54906 * assign41880_e54906))), (-((var_gf_dn7 * 0.7324648775608221) / (assign41880_e54906 * assign41880_e54906))), (-((var_gf_dn8 * 0.7324648775608221) / (assign41880_e54906 * assign41880_e54906))),)
    } else {
        (var_sp_xg1, var_sp_xg1_dn5, var_sp_xg1_dn6, var_sp_xg1_dn7, var_sp_xg1_dn8,)
    }
};
        var_sp_xg1 = assign41880_e54909;
        var_sp_xg1_dn5 = assign41880_e54909_d_n5;
        var_sp_xg1_dn6 = assign41880_e54909_d_n6;
        var_sp_xg1_dn7 = assign41880_e54909_d_n7;
        var_sp_xg1_dn8 = assign41880_e54909_d_n8;
        var_sp_xg1_rv = 0.0;

        let (assign41890_e54925, assign41890_e54925_d_n5, assign41890_e54925_d_n6, assign41890_e54925_d_n7, assign41890_e54925_d_n8,) = {
    if ((var_guard1182 == 0.0) && (var_guard1183 == 0.0)) {
        let assign41890_e54917: f64 = (var_xi * 1.25);
        let assign41890_e54919: f64 = (assign41890_e54917 * var_sp_xg1);
        let assign41890_e54921: f64 = (assign41890_e54919 - 1.0);
        let assign41890_e54923: f64 = (assign41890_e54921 * var_sp_xg1);
        (assign41890_e54923, (((((var_xi_dn5 * 1.25) * var_sp_xg1) + (assign41890_e54917 * var_sp_xg1_dn5)) * var_sp_xg1) + (assign41890_e54921 * var_sp_xg1_dn5)), (((((var_xi_dn6 * 1.25) * var_sp_xg1) + (assign41890_e54917 * var_sp_xg1_dn6)) * var_sp_xg1) + (assign41890_e54921 * var_sp_xg1_dn6)), (((((var_xi_dn7 * 1.25) * var_sp_xg1) + (assign41890_e54917 * var_sp_xg1_dn7)) * var_sp_xg1) + (assign41890_e54921 * var_sp_xg1_dn7)), (((((var_xi_dn8 * 1.25) * var_sp_xg1) + (assign41890_e54917 * var_sp_xg1_dn8)) * var_sp_xg1) + (assign41890_e54921 * var_sp_xg1_dn8)),)
    } else {
        (var_sp_s_a_fac, var_sp_s_a_fac_dn5, var_sp_s_a_fac_dn6, var_sp_s_a_fac_dn7, var_sp_s_a_fac_dn8,)
    }
};
        var_sp_s_a_fac = assign41890_e54925;
        var_sp_s_a_fac_dn5 = assign41890_e54925_d_n5;
        var_sp_s_a_fac_dn6 = assign41890_e54925_d_n6;
        var_sp_s_a_fac_dn7 = assign41890_e54925_d_n7;
        var_sp_s_a_fac_dn8 = assign41890_e54925_d_n8;
        var_sp_s_a_fac_rv = 0.0;

        let (assign41900_e54941, assign41900_e54941_d_n5, assign41900_e54941_d_n6, assign41900_e54941_d_n7, assign41900_e54941_d_n8,) = {
    if ((var_guard1182 == 0.0) && (var_guard1183 == 0.0)) {
        let assign41900_e54933: f64 = (var_xg * var_inv_xi);
        let assign41900_e54937: f64 = (var_sp_s_a_fac * var_xg);
        let assign41900_e54938: f64 = (1.0 + assign41900_e54937);
        let assign41900_e54939: f64 = (assign41900_e54933 * assign41900_e54938);
        (assign41900_e54939, ((((var_xg_dn5 * var_inv_xi) + (var_xg * var_inv_xi_dn5)) * assign41900_e54938) + (assign41900_e54933 * ((var_sp_s_a_fac_dn5 * var_xg) + (var_sp_s_a_fac * var_xg_dn5)))), ((((var_xg_dn6 * var_inv_xi) + (var_xg * var_inv_xi_dn6)) * assign41900_e54938) + (assign41900_e54933 * ((var_sp_s_a_fac_dn6 * var_xg) + (var_sp_s_a_fac * var_xg_dn6)))), ((((var_xg_dn7 * var_inv_xi) + (var_xg * var_inv_xi_dn7)) * assign41900_e54938) + (assign41900_e54933 * ((var_sp_s_a_fac_dn7 * var_xg) + (var_sp_s_a_fac * var_xg_dn7)))), ((((var_xg_dn8 * var_inv_xi) + (var_xg * var_inv_xi_dn8)) * assign41900_e54938) + (assign41900_e54933 * ((var_sp_s_a_fac_dn8 * var_xg) + (var_sp_s_a_fac * var_xg_dn8)))),)
    } else {
        (var_sp_s_xbar, var_sp_s_xbar_dn5, var_sp_s_xbar_dn6, var_sp_s_xbar_dn7, var_sp_s_xbar_dn8,)
    }
};
        var_sp_s_xbar = assign41900_e54941;
        var_sp_s_xbar_dn5 = assign41900_e54941_d_n5;
        var_sp_s_xbar_dn6 = assign41900_e54941_d_n6;
        var_sp_s_xbar_dn7 = assign41900_e54941_d_n7;
        var_sp_s_xbar_dn8 = assign41900_e54941_d_n8;
        var_sp_s_xbar_rv = 0.0;

        let assign41910_e54943: f64 = (-var_sp_s_xbar);
        let assign41910_e54945: f64 = (-230.25850929940458);
        let assign41910_e54946: f64 = if assign41910_e54943 > assign41910_e54945 { 1.0 } else { 0.0 };
        var_guard1185 = assign41910_e54946;
        var_guard1185_rv = 0.0;

        let (assign41920_e54958, assign41920_e54958_d_n5, assign41920_e54958_d_n6, assign41920_e54958_d_n7, assign41920_e54958_d_n8,) = {
    if (((var_guard1182 == 0.0) && (var_guard1183 == 0.0)) && (var_guard1185 != 0.0)) {
        let assign41920_e54955: f64 = (-var_sp_s_xbar);
        let assign41920_e54956: f64 = (assign41920_e54955).exp();
        (assign41920_e54956, (assign41920_e54956 * (-var_sp_s_xbar_dn5)), (assign41920_e54956 * (-var_sp_s_xbar_dn6)), (assign41920_e54956 * (-var_sp_s_xbar_dn7)), (assign41920_e54956 * (-var_sp_s_xbar_dn8)),)
    } else {
        (var_sp_s_temp, var_sp_s_temp_dn5, var_sp_s_temp_dn6, var_sp_s_temp_dn7, var_sp_s_temp_dn8,)
    }
};
        var_sp_s_temp = assign41920_e54958;
        var_sp_s_temp_dn5 = assign41920_e54958_d_n5;
        var_sp_s_temp_dn6 = assign41920_e54958_d_n6;
        var_sp_s_temp_dn7 = assign41920_e54958_d_n7;
        var_sp_s_temp_dn8 = assign41920_e54958_d_n8;
        var_sp_s_temp_rv = 0.0;

        let (assign41930_e54997, assign41930_e54997_d_n5, assign41930_e54997_d_n6, assign41930_e54997_d_n7, assign41930_e54997_d_n8,) = {
    if (((var_guard1182 == 0.0) && (var_guard1183 == 0.0)) && (var_guard1185 == 0.0)) {
        let assign41930_e54970: f64 = (-230.25850929940458);
        let assign41930_e54972: f64 = (-var_sp_s_xbar);
        let assign41930_e54973: f64 = (assign41930_e54970 - assign41930_e54972);
        let assign41930_e54977: f64 = (-230.25850929940458);
        let assign41930_e54979: f64 = (-var_sp_s_xbar);
        let assign41930_e54980: f64 = (assign41930_e54977 - assign41930_e54979);
        let assign41930_e54983: f64 = (-230.25850929940458);
        let assign41930_e54985: f64 = (-var_sp_s_xbar);
        let assign41930_e54986: f64 = (assign41930_e54983 - assign41930_e54985);
        let assign41930_e54988: f64 = (assign41930_e54986 * 0.3333333333333333);
        let assign41930_e54989: f64 = (1.0 + assign41930_e54988);
        let assign41930_e54990: f64 = (assign41930_e54980 * assign41930_e54989);
        let assign41930_e54991: f64 = (0.5 * assign41930_e54990);
        let assign41930_e54992: f64 = (1.0 + assign41930_e54991);
        let assign41930_e54993: f64 = (assign41930_e54973 * assign41930_e54992);
        let assign41930_e54994: f64 = (1.0 + assign41930_e54993);
        let assign41930_e54995: f64 = (1e-100 / assign41930_e54994);
        (assign41930_e54995, (-((1e-100 * (((-(-var_sp_s_xbar_dn5)) * assign41930_e54992) + (assign41930_e54973 * (0.5 * (((-(-var_sp_s_xbar_dn5)) * assign41930_e54989) + (assign41930_e54980 * ((-(-var_sp_s_xbar_dn5)) * 0.3333333333333333))))))) / (assign41930_e54994 * assign41930_e54994))), (-((1e-100 * (((-(-var_sp_s_xbar_dn6)) * assign41930_e54992) + (assign41930_e54973 * (0.5 * (((-(-var_sp_s_xbar_dn6)) * assign41930_e54989) + (assign41930_e54980 * ((-(-var_sp_s_xbar_dn6)) * 0.3333333333333333))))))) / (assign41930_e54994 * assign41930_e54994))), (-((1e-100 * (((-(-var_sp_s_xbar_dn7)) * assign41930_e54992) + (assign41930_e54973 * (0.5 * (((-(-var_sp_s_xbar_dn7)) * assign41930_e54989) + (assign41930_e54980 * ((-(-var_sp_s_xbar_dn7)) * 0.3333333333333333))))))) / (assign41930_e54994 * assign41930_e54994))), (-((1e-100 * (((-(-var_sp_s_xbar_dn8)) * assign41930_e54992) + (assign41930_e54973 * (0.5 * (((-(-var_sp_s_xbar_dn8)) * assign41930_e54989) + (assign41930_e54980 * ((-(-var_sp_s_xbar_dn8)) * 0.3333333333333333))))))) / (assign41930_e54994 * assign41930_e54994))),)
    } else {
        (var_sp_s_temp, var_sp_s_temp_dn5, var_sp_s_temp_dn6, var_sp_s_temp_dn7, var_sp_s_temp_dn8,)
    }
};
        var_sp_s_temp = assign41930_e54997;
        var_sp_s_temp_dn5 = assign41930_e54997_d_n5;
        var_sp_s_temp_dn6 = assign41930_e54997_d_n6;
        var_sp_s_temp_dn7 = assign41930_e54997_d_n7;
        var_sp_s_temp_dn8 = assign41930_e54997_d_n8;
        var_sp_s_temp_rv = 0.0;

        let (assign41940_e55007, assign41940_e55007_d_n5, assign41940_e55007_d_n6, assign41940_e55007_d_n7, assign41940_e55007_d_n8,) = {
    if ((var_guard1182 == 0.0) && (var_guard1183 == 0.0)) {
        let assign41940_e55005: f64 = (1.0 - var_sp_s_temp);
        (assign41940_e55005, (-var_sp_s_temp_dn5), (-var_sp_s_temp_dn6), (-var_sp_s_temp_dn7), (-var_sp_s_temp_dn8),)
    } else {
        (var_sp_s_w, var_sp_s_w_dn5, var_sp_s_w_dn6, var_sp_s_w_dn7, var_sp_s_w_dn8,)
    }
};
        var_sp_s_w = assign41940_e55007;
        var_sp_s_w_dn5 = assign41940_e55007_d_n5;
        var_sp_s_w_dn6 = assign41940_e55007_d_n6;
        var_sp_s_w_dn7 = assign41940_e55007_d_n7;
        var_sp_s_w_dn8 = assign41940_e55007_d_n8;
        var_sp_s_w_rv = 0.0;

        *var_guard1184_slot = var_guard1184;
        *var_guard1184_rv_slot = var_guard1184_rv;
        *var_guard1185_slot = var_guard1185;
        *var_guard1185_rv_slot = var_guard1185_rv;
        *var_mutau_slot = var_mutau;
        *var_mutau_dn5_slot = var_mutau_dn5;
        *var_mutau_dn6_slot = var_mutau_dn6;
        *var_mutau_dn7_slot = var_mutau_dn7;
        *var_mutau_dn8_slot = var_mutau_dn8;
        *var_mutau_rv_slot = var_mutau_rv;
        *var_nu_slot = var_nu;
        *var_nu_dn5_slot = var_nu_dn5;
        *var_nu_dn6_slot = var_nu_dn6;
        *var_nu_dn7_slot = var_nu_dn7;
        *var_nu_dn8_slot = var_nu_dn8;
        *var_nu_rv_slot = var_nu_rv;
        *var_sp_s_a_slot = var_sp_s_a;
        *var_sp_s_a_dn5_slot = var_sp_s_a_dn5;
        *var_sp_s_a_dn6_slot = var_sp_s_a_dn6;
        *var_sp_s_a_dn7_slot = var_sp_s_a_dn7;
        *var_sp_s_a_dn8_slot = var_sp_s_a_dn8;
        *var_sp_s_a_fac_slot = var_sp_s_a_fac;
        *var_sp_s_a_fac_dn5_slot = var_sp_s_a_fac_dn5;
        *var_sp_s_a_fac_dn6_slot = var_sp_s_a_fac_dn6;
        *var_sp_s_a_fac_dn7_slot = var_sp_s_a_fac_dn7;
        *var_sp_s_a_fac_dn8_slot = var_sp_s_a_fac_dn8;
        *var_sp_s_a_fac_rv_slot = var_sp_s_a_fac_rv;
        *var_sp_s_a_rv_slot = var_sp_s_a_rv;
        *var_sp_s_c_slot = var_sp_s_c;
        *var_sp_s_c_dn5_slot = var_sp_s_c_dn5;
        *var_sp_s_c_dn6_slot = var_sp_s_c_dn6;
        *var_sp_s_c_dn7_slot = var_sp_s_c_dn7;
        *var_sp_s_c_dn8_slot = var_sp_s_c_dn8;
        *var_sp_s_c_rv_slot = var_sp_s_c_rv;
        *var_sp_s_delta0_slot = var_sp_s_delta0;
        *var_sp_s_delta0_dn5_slot = var_sp_s_delta0_dn5;
        *var_sp_s_delta0_dn6_slot = var_sp_s_delta0_dn6;
        *var_sp_s_delta0_dn7_slot = var_sp_s_delta0_dn7;
        *var_sp_s_delta0_dn8_slot = var_sp_s_delta0_dn8;
        *var_sp_s_delta0_rv_slot = var_sp_s_delta0_rv;
        *var_sp_s_delta1_slot = var_sp_s_delta1;
        *var_sp_s_delta1_dn5_slot = var_sp_s_delta1_dn5;
        *var_sp_s_delta1_dn6_slot = var_sp_s_delta1_dn6;
        *var_sp_s_delta1_dn7_slot = var_sp_s_delta1_dn7;
        *var_sp_s_delta1_dn8_slot = var_sp_s_delta1_dn8;
        *var_sp_s_delta1_rv_slot = var_sp_s_delta1_rv;
        *var_sp_s_pc_slot = var_sp_s_pc;
        *var_sp_s_pc_dn5_slot = var_sp_s_pc_dn5;
        *var_sp_s_pc_dn6_slot = var_sp_s_pc_dn6;
        *var_sp_s_pc_dn7_slot = var_sp_s_pc_dn7;
        *var_sp_s_pc_dn8_slot = var_sp_s_pc_dn8;
        *var_sp_s_pc_rv_slot = var_sp_s_pc_rv;
        *var_sp_s_qc_slot = var_sp_s_qc;
        *var_sp_s_qc_dn5_slot = var_sp_s_qc_dn5;
        *var_sp_s_qc_dn6_slot = var_sp_s_qc_dn6;
        *var_sp_s_qc_dn7_slot = var_sp_s_qc_dn7;
        *var_sp_s_qc_dn8_slot = var_sp_s_qc_dn8;
        *var_sp_s_qc_rv_slot = var_sp_s_qc_rv;
        *var_sp_s_tau_slot = var_sp_s_tau;
        *var_sp_s_tau_dn5_slot = var_sp_s_tau_dn5;
        *var_sp_s_tau_dn6_slot = var_sp_s_tau_dn6;
        *var_sp_s_tau_dn7_slot = var_sp_s_tau_dn7;
        *var_sp_s_tau_dn8_slot = var_sp_s_tau_dn8;
        *var_sp_s_tau_rv_slot = var_sp_s_tau_rv;
        *var_sp_s_temp_slot = var_sp_s_temp;
        *var_sp_s_temp1_slot = var_sp_s_temp1;
        *var_sp_s_temp1_dn5_slot = var_sp_s_temp1_dn5;
        *var_sp_s_temp1_dn6_slot = var_sp_s_temp1_dn6;
        *var_sp_s_temp1_dn7_slot = var_sp_s_temp1_dn7;
        *var_sp_s_temp1_dn8_slot = var_sp_s_temp1_dn8;
        *var_sp_s_temp1_rv_slot = var_sp_s_temp1_rv;
        *var_sp_s_temp_dn5_slot = var_sp_s_temp_dn5;
        *var_sp_s_temp_dn6_slot = var_sp_s_temp_dn6;
        *var_sp_s_temp_dn7_slot = var_sp_s_temp_dn7;
        *var_sp_s_temp_dn8_slot = var_sp_s_temp_dn8;
        *var_sp_s_temp_rv_slot = var_sp_s_temp_rv;
        *var_sp_s_w_slot = var_sp_s_w;
        *var_sp_s_w_dn5_slot = var_sp_s_w_dn5;
        *var_sp_s_w_dn6_slot = var_sp_s_w_dn6;
        *var_sp_s_w_dn7_slot = var_sp_s_w_dn7;
        *var_sp_s_w_dn8_slot = var_sp_s_w_dn8;
        *var_sp_s_w_rv_slot = var_sp_s_w_rv;
        *var_sp_s_xbar_slot = var_sp_s_xbar;
        *var_sp_s_xbar_dn5_slot = var_sp_s_xbar_dn5;
        *var_sp_s_xbar_dn6_slot = var_sp_s_xbar_dn6;
        *var_sp_s_xbar_dn7_slot = var_sp_s_xbar_dn7;
        *var_sp_s_xbar_dn8_slot = var_sp_s_xbar_dn8;
        *var_sp_s_xbar_rv_slot = var_sp_s_xbar_rv;
        *var_sp_s_xi0_slot = var_sp_s_xi0;
        *var_sp_s_xi0_dn5_slot = var_sp_s_xi0_dn5;
        *var_sp_s_xi0_dn6_slot = var_sp_s_xi0_dn6;
        *var_sp_s_xi0_dn7_slot = var_sp_s_xi0_dn7;
        *var_sp_s_xi0_dn8_slot = var_sp_s_xi0_dn8;
        *var_sp_s_xi0_rv_slot = var_sp_s_xi0_rv;
        *var_sp_s_xi1_slot = var_sp_s_xi1;
        *var_sp_s_xi1_dn5_slot = var_sp_s_xi1_dn5;
        *var_sp_s_xi1_dn6_slot = var_sp_s_xi1_dn6;
        *var_sp_s_xi1_dn7_slot = var_sp_s_xi1_dn7;
        *var_sp_s_xi1_dn8_slot = var_sp_s_xi1_dn8;
        *var_sp_s_xi1_rv_slot = var_sp_s_xi1_rv;
        *var_sp_s_xi2_slot = var_sp_s_xi2;
        *var_sp_s_xi2_dn5_slot = var_sp_s_xi2_dn5;
        *var_sp_s_xi2_dn6_slot = var_sp_s_xi2_dn6;
        *var_sp_s_xi2_dn7_slot = var_sp_s_xi2_dn7;
        *var_sp_s_xi2_dn8_slot = var_sp_s_xi2_dn8;
        *var_sp_s_xi2_rv_slot = var_sp_s_xi2_rv;
        *var_sp_s_y0_slot = var_sp_s_y0;
        *var_sp_s_y0_dn5_slot = var_sp_s_y0_dn5;
        *var_sp_s_y0_dn6_slot = var_sp_s_y0_dn6;
        *var_sp_s_y0_dn7_slot = var_sp_s_y0_dn7;
        *var_sp_s_y0_dn8_slot = var_sp_s_y0_dn8;
        *var_sp_s_y0_rv_slot = var_sp_s_y0_rv;
        *var_sp_xg1_slot = var_sp_xg1;
        *var_sp_xg1_dn5_slot = var_sp_xg1_dn5;
        *var_sp_xg1_dn6_slot = var_sp_xg1_dn6;
        *var_sp_xg1_dn7_slot = var_sp_xg1_dn7;
        *var_sp_xg1_dn8_slot = var_sp_xg1_dn8;
        *var_sp_xg1_rv_slot = var_sp_xg1_rv;
        *var_x_s_slot = var_x_s;
        *var_x_s_dn5_slot = var_x_s_dn5;
        *var_x_s_dn6_slot = var_x_s_dn6;
        *var_x_s_dn7_slot = var_x_s_dn7;
        *var_x_s_dn8_slot = var_x_s_dn8;
        *var_x_s_rv_slot = var_x_s_rv;
    }

    pub(super) fn stamp_reactive_block_27(
        var_delta_ns: f64,
        var_delta_ns_dn5: f64,
        var_delta_ns_dn6: f64,
        var_delta_ns_dn7: f64,
        var_delta_ns_dn8: f64,
        var_gf: f64,
        var_gf2: f64,
        var_gf2_dn5: f64,
        var_gf2_dn6: f64,
        var_gf2_dn7: f64,
        var_gf2_dn8: f64,
        var_gf_dn5: f64,
        var_gf_dn6: f64,
        var_gf_dn7: f64,
        var_gf_dn8: f64,
        var_guard1182: f64,
        var_guard1183: f64,
        var_sp_s_w: f64,
        var_sp_s_w_dn5: f64,
        var_sp_s_w_dn6: f64,
        var_sp_s_w_dn7: f64,
        var_sp_s_w_dn8: f64,
        var_xg: f64,
        var_xg_dn5: f64,
        var_xg_dn6: f64,
        var_xg_dn7: f64,
        var_xg_dn8: f64,
        var_xn_s: f64,
        var_xn_s_dn5: f64,
        var_xn_s_dn6: f64,
        var_xn_s_dn7: f64,
        var_xn_s_dn8: f64,
        var_guard1186_slot: &mut f64,
        var_guard1186_rv_slot: &mut f64,
        var_guard1187_slot: &mut f64,
        var_guard1187_rv_slot: &mut f64,
        var_mutau_slot: &mut f64,
        var_mutau_dn5_slot: &mut f64,
        var_mutau_dn6_slot: &mut f64,
        var_mutau_dn7_slot: &mut f64,
        var_mutau_dn8_slot: &mut f64,
        var_mutau_rv_slot: &mut f64,
        var_nu_slot: &mut f64,
        var_nu_dn5_slot: &mut f64,
        var_nu_dn6_slot: &mut f64,
        var_nu_dn7_slot: &mut f64,
        var_nu_dn8_slot: &mut f64,
        var_nu_rv_slot: &mut f64,
        var_sp_s_a_slot: &mut f64,
        var_sp_s_a_dn5_slot: &mut f64,
        var_sp_s_a_dn6_slot: &mut f64,
        var_sp_s_a_dn7_slot: &mut f64,
        var_sp_s_a_dn8_slot: &mut f64,
        var_sp_s_a_rv_slot: &mut f64,
        var_sp_s_b_slot: &mut f64,
        var_sp_s_b_dn5_slot: &mut f64,
        var_sp_s_b_dn6_slot: &mut f64,
        var_sp_s_b_dn7_slot: &mut f64,
        var_sp_s_b_dn8_slot: &mut f64,
        var_sp_s_b_rv_slot: &mut f64,
        var_sp_s_bx_slot: &mut f64,
        var_sp_s_bx_dn5_slot: &mut f64,
        var_sp_s_bx_dn6_slot: &mut f64,
        var_sp_s_bx_dn7_slot: &mut f64,
        var_sp_s_bx_dn8_slot: &mut f64,
        var_sp_s_bx_rv_slot: &mut f64,
        var_sp_s_c_slot: &mut f64,
        var_sp_s_c_dn5_slot: &mut f64,
        var_sp_s_c_dn6_slot: &mut f64,
        var_sp_s_c_dn7_slot: &mut f64,
        var_sp_s_c_dn8_slot: &mut f64,
        var_sp_s_c_rv_slot: &mut f64,
        var_sp_s_delta0_slot: &mut f64,
        var_sp_s_delta0_dn5_slot: &mut f64,
        var_sp_s_delta0_dn6_slot: &mut f64,
        var_sp_s_delta0_dn7_slot: &mut f64,
        var_sp_s_delta0_dn8_slot: &mut f64,
        var_sp_s_delta0_rv_slot: &mut f64,
        var_sp_s_delta1_slot: &mut f64,
        var_sp_s_delta1_dn5_slot: &mut f64,
        var_sp_s_delta1_dn6_slot: &mut f64,
        var_sp_s_delta1_dn7_slot: &mut f64,
        var_sp_s_delta1_dn8_slot: &mut f64,
        var_sp_s_delta1_rv_slot: &mut f64,
        var_sp_s_eta_slot: &mut f64,
        var_sp_s_eta_dn5_slot: &mut f64,
        var_sp_s_eta_dn6_slot: &mut f64,
        var_sp_s_eta_dn7_slot: &mut f64,
        var_sp_s_eta_dn8_slot: &mut f64,
        var_sp_s_eta_rv_slot: &mut f64,
        var_sp_s_tau_slot: &mut f64,
        var_sp_s_tau_dn5_slot: &mut f64,
        var_sp_s_tau_dn6_slot: &mut f64,
        var_sp_s_tau_dn7_slot: &mut f64,
        var_sp_s_tau_dn8_slot: &mut f64,
        var_sp_s_tau_rv_slot: &mut f64,
        var_sp_s_temp_slot: &mut f64,
        var_sp_s_temp1_slot: &mut f64,
        var_sp_s_temp1_dn5_slot: &mut f64,
        var_sp_s_temp1_dn6_slot: &mut f64,
        var_sp_s_temp1_dn7_slot: &mut f64,
        var_sp_s_temp1_dn8_slot: &mut f64,
        var_sp_s_temp1_rv_slot: &mut f64,
        var_sp_s_temp2_slot: &mut f64,
        var_sp_s_temp2_dn5_slot: &mut f64,
        var_sp_s_temp2_dn6_slot: &mut f64,
        var_sp_s_temp2_dn7_slot: &mut f64,
        var_sp_s_temp2_dn8_slot: &mut f64,
        var_sp_s_temp2_rv_slot: &mut f64,
        var_sp_s_temp_dn5_slot: &mut f64,
        var_sp_s_temp_dn6_slot: &mut f64,
        var_sp_s_temp_dn7_slot: &mut f64,
        var_sp_s_temp_dn8_slot: &mut f64,
        var_sp_s_temp_rv_slot: &mut f64,
        var_sp_s_x0_slot: &mut f64,
        var_sp_s_x0_dn5_slot: &mut f64,
        var_sp_s_x0_dn6_slot: &mut f64,
        var_sp_s_x0_dn7_slot: &mut f64,
        var_sp_s_x0_dn8_slot: &mut f64,
        var_sp_s_x0_rv_slot: &mut f64,
        var_sp_s_x1_slot: &mut f64,
        var_sp_s_x1_dn5_slot: &mut f64,
        var_sp_s_x1_dn6_slot: &mut f64,
        var_sp_s_x1_dn7_slot: &mut f64,
        var_sp_s_x1_dn8_slot: &mut f64,
        var_sp_s_x1_rv_slot: &mut f64,
        var_sp_s_xi0_slot: &mut f64,
        var_sp_s_xi0_dn5_slot: &mut f64,
        var_sp_s_xi0_dn6_slot: &mut f64,
        var_sp_s_xi0_dn7_slot: &mut f64,
        var_sp_s_xi0_dn8_slot: &mut f64,
        var_sp_s_xi0_rv_slot: &mut f64,
        var_sp_s_xi1_slot: &mut f64,
        var_sp_s_xi1_dn5_slot: &mut f64,
        var_sp_s_xi1_dn6_slot: &mut f64,
        var_sp_s_xi1_dn7_slot: &mut f64,
        var_sp_s_xi1_dn8_slot: &mut f64,
        var_sp_s_xi1_rv_slot: &mut f64,
        var_sp_s_xi2_slot: &mut f64,
        var_sp_s_xi2_dn5_slot: &mut f64,
        var_sp_s_xi2_dn6_slot: &mut f64,
        var_sp_s_xi2_dn7_slot: &mut f64,
        var_sp_s_xi2_dn8_slot: &mut f64,
        var_sp_s_xi2_rv_slot: &mut f64,
    ) {
        let mut var_guard1186: f64 = *var_guard1186_slot;
        let mut var_guard1186_rv: f64 = *var_guard1186_rv_slot;
        let mut var_guard1187: f64 = *var_guard1187_slot;
        let mut var_guard1187_rv: f64 = *var_guard1187_rv_slot;
        let mut var_mutau: f64 = *var_mutau_slot;
        let mut var_mutau_dn5: f64 = *var_mutau_dn5_slot;
        let mut var_mutau_dn6: f64 = *var_mutau_dn6_slot;
        let mut var_mutau_dn7: f64 = *var_mutau_dn7_slot;
        let mut var_mutau_dn8: f64 = *var_mutau_dn8_slot;
        let mut var_mutau_rv: f64 = *var_mutau_rv_slot;
        let mut var_nu: f64 = *var_nu_slot;
        let mut var_nu_dn5: f64 = *var_nu_dn5_slot;
        let mut var_nu_dn6: f64 = *var_nu_dn6_slot;
        let mut var_nu_dn7: f64 = *var_nu_dn7_slot;
        let mut var_nu_dn8: f64 = *var_nu_dn8_slot;
        let mut var_nu_rv: f64 = *var_nu_rv_slot;
        let mut var_sp_s_a: f64 = *var_sp_s_a_slot;
        let mut var_sp_s_a_dn5: f64 = *var_sp_s_a_dn5_slot;
        let mut var_sp_s_a_dn6: f64 = *var_sp_s_a_dn6_slot;
        let mut var_sp_s_a_dn7: f64 = *var_sp_s_a_dn7_slot;
        let mut var_sp_s_a_dn8: f64 = *var_sp_s_a_dn8_slot;
        let mut var_sp_s_a_rv: f64 = *var_sp_s_a_rv_slot;
        let mut var_sp_s_b: f64 = *var_sp_s_b_slot;
        let mut var_sp_s_b_dn5: f64 = *var_sp_s_b_dn5_slot;
        let mut var_sp_s_b_dn6: f64 = *var_sp_s_b_dn6_slot;
        let mut var_sp_s_b_dn7: f64 = *var_sp_s_b_dn7_slot;
        let mut var_sp_s_b_dn8: f64 = *var_sp_s_b_dn8_slot;
        let mut var_sp_s_b_rv: f64 = *var_sp_s_b_rv_slot;
        let mut var_sp_s_bx: f64 = *var_sp_s_bx_slot;
        let mut var_sp_s_bx_dn5: f64 = *var_sp_s_bx_dn5_slot;
        let mut var_sp_s_bx_dn6: f64 = *var_sp_s_bx_dn6_slot;
        let mut var_sp_s_bx_dn7: f64 = *var_sp_s_bx_dn7_slot;
        let mut var_sp_s_bx_dn8: f64 = *var_sp_s_bx_dn8_slot;
        let mut var_sp_s_bx_rv: f64 = *var_sp_s_bx_rv_slot;
        let mut var_sp_s_c: f64 = *var_sp_s_c_slot;
        let mut var_sp_s_c_dn5: f64 = *var_sp_s_c_dn5_slot;
        let mut var_sp_s_c_dn6: f64 = *var_sp_s_c_dn6_slot;
        let mut var_sp_s_c_dn7: f64 = *var_sp_s_c_dn7_slot;
        let mut var_sp_s_c_dn8: f64 = *var_sp_s_c_dn8_slot;
        let mut var_sp_s_c_rv: f64 = *var_sp_s_c_rv_slot;
        let mut var_sp_s_delta0: f64 = *var_sp_s_delta0_slot;
        let mut var_sp_s_delta0_dn5: f64 = *var_sp_s_delta0_dn5_slot;
        let mut var_sp_s_delta0_dn6: f64 = *var_sp_s_delta0_dn6_slot;
        let mut var_sp_s_delta0_dn7: f64 = *var_sp_s_delta0_dn7_slot;
        let mut var_sp_s_delta0_dn8: f64 = *var_sp_s_delta0_dn8_slot;
        let mut var_sp_s_delta0_rv: f64 = *var_sp_s_delta0_rv_slot;
        let mut var_sp_s_delta1: f64 = *var_sp_s_delta1_slot;
        let mut var_sp_s_delta1_dn5: f64 = *var_sp_s_delta1_dn5_slot;
        let mut var_sp_s_delta1_dn6: f64 = *var_sp_s_delta1_dn6_slot;
        let mut var_sp_s_delta1_dn7: f64 = *var_sp_s_delta1_dn7_slot;
        let mut var_sp_s_delta1_dn8: f64 = *var_sp_s_delta1_dn8_slot;
        let mut var_sp_s_delta1_rv: f64 = *var_sp_s_delta1_rv_slot;
        let mut var_sp_s_eta: f64 = *var_sp_s_eta_slot;
        let mut var_sp_s_eta_dn5: f64 = *var_sp_s_eta_dn5_slot;
        let mut var_sp_s_eta_dn6: f64 = *var_sp_s_eta_dn6_slot;
        let mut var_sp_s_eta_dn7: f64 = *var_sp_s_eta_dn7_slot;
        let mut var_sp_s_eta_dn8: f64 = *var_sp_s_eta_dn8_slot;
        let mut var_sp_s_eta_rv: f64 = *var_sp_s_eta_rv_slot;
        let mut var_sp_s_tau: f64 = *var_sp_s_tau_slot;
        let mut var_sp_s_tau_dn5: f64 = *var_sp_s_tau_dn5_slot;
        let mut var_sp_s_tau_dn6: f64 = *var_sp_s_tau_dn6_slot;
        let mut var_sp_s_tau_dn7: f64 = *var_sp_s_tau_dn7_slot;
        let mut var_sp_s_tau_dn8: f64 = *var_sp_s_tau_dn8_slot;
        let mut var_sp_s_tau_rv: f64 = *var_sp_s_tau_rv_slot;
        let mut var_sp_s_temp: f64 = *var_sp_s_temp_slot;
        let mut var_sp_s_temp1: f64 = *var_sp_s_temp1_slot;
        let mut var_sp_s_temp1_dn5: f64 = *var_sp_s_temp1_dn5_slot;
        let mut var_sp_s_temp1_dn6: f64 = *var_sp_s_temp1_dn6_slot;
        let mut var_sp_s_temp1_dn7: f64 = *var_sp_s_temp1_dn7_slot;
        let mut var_sp_s_temp1_dn8: f64 = *var_sp_s_temp1_dn8_slot;
        let mut var_sp_s_temp1_rv: f64 = *var_sp_s_temp1_rv_slot;
        let mut var_sp_s_temp2: f64 = *var_sp_s_temp2_slot;
        let mut var_sp_s_temp2_dn5: f64 = *var_sp_s_temp2_dn5_slot;
        let mut var_sp_s_temp2_dn6: f64 = *var_sp_s_temp2_dn6_slot;
        let mut var_sp_s_temp2_dn7: f64 = *var_sp_s_temp2_dn7_slot;
        let mut var_sp_s_temp2_dn8: f64 = *var_sp_s_temp2_dn8_slot;
        let mut var_sp_s_temp2_rv: f64 = *var_sp_s_temp2_rv_slot;
        let mut var_sp_s_temp_dn5: f64 = *var_sp_s_temp_dn5_slot;
        let mut var_sp_s_temp_dn6: f64 = *var_sp_s_temp_dn6_slot;
        let mut var_sp_s_temp_dn7: f64 = *var_sp_s_temp_dn7_slot;
        let mut var_sp_s_temp_dn8: f64 = *var_sp_s_temp_dn8_slot;
        let mut var_sp_s_temp_rv: f64 = *var_sp_s_temp_rv_slot;
        let mut var_sp_s_x0: f64 = *var_sp_s_x0_slot;
        let mut var_sp_s_x0_dn5: f64 = *var_sp_s_x0_dn5_slot;
        let mut var_sp_s_x0_dn6: f64 = *var_sp_s_x0_dn6_slot;
        let mut var_sp_s_x0_dn7: f64 = *var_sp_s_x0_dn7_slot;
        let mut var_sp_s_x0_dn8: f64 = *var_sp_s_x0_dn8_slot;
        let mut var_sp_s_x0_rv: f64 = *var_sp_s_x0_rv_slot;
        let mut var_sp_s_x1: f64 = *var_sp_s_x1_slot;
        let mut var_sp_s_x1_dn5: f64 = *var_sp_s_x1_dn5_slot;
        let mut var_sp_s_x1_dn6: f64 = *var_sp_s_x1_dn6_slot;
        let mut var_sp_s_x1_dn7: f64 = *var_sp_s_x1_dn7_slot;
        let mut var_sp_s_x1_dn8: f64 = *var_sp_s_x1_dn8_slot;
        let mut var_sp_s_x1_rv: f64 = *var_sp_s_x1_rv_slot;
        let mut var_sp_s_xi0: f64 = *var_sp_s_xi0_slot;
        let mut var_sp_s_xi0_dn5: f64 = *var_sp_s_xi0_dn5_slot;
        let mut var_sp_s_xi0_dn6: f64 = *var_sp_s_xi0_dn6_slot;
        let mut var_sp_s_xi0_dn7: f64 = *var_sp_s_xi0_dn7_slot;
        let mut var_sp_s_xi0_dn8: f64 = *var_sp_s_xi0_dn8_slot;
        let mut var_sp_s_xi0_rv: f64 = *var_sp_s_xi0_rv_slot;
        let mut var_sp_s_xi1: f64 = *var_sp_s_xi1_slot;
        let mut var_sp_s_xi1_dn5: f64 = *var_sp_s_xi1_dn5_slot;
        let mut var_sp_s_xi1_dn6: f64 = *var_sp_s_xi1_dn6_slot;
        let mut var_sp_s_xi1_dn7: f64 = *var_sp_s_xi1_dn7_slot;
        let mut var_sp_s_xi1_dn8: f64 = *var_sp_s_xi1_dn8_slot;
        let mut var_sp_s_xi1_rv: f64 = *var_sp_s_xi1_rv_slot;
        let mut var_sp_s_xi2: f64 = *var_sp_s_xi2_slot;
        let mut var_sp_s_xi2_dn5: f64 = *var_sp_s_xi2_dn5_slot;
        let mut var_sp_s_xi2_dn6: f64 = *var_sp_s_xi2_dn6_slot;
        let mut var_sp_s_xi2_dn7: f64 = *var_sp_s_xi2_dn7_slot;
        let mut var_sp_s_xi2_dn8: f64 = *var_sp_s_xi2_dn8_slot;
        let mut var_sp_s_xi2_rv: f64 = *var_sp_s_xi2_rv_slot;

        let (assign41950_e55030, assign41950_e55030_d_n5, assign41950_e55030_d_n6, assign41950_e55030_d_n7, assign41950_e55030_d_n8,) = {
    if ((var_guard1182 == 0.0) && (var_guard1183 == 0.0)) {
        let assign41950_e55016: f64 = (var_gf2 * 0.5);
        let assign41950_e55017: f64 = (var_xg + assign41950_e55016);
        let assign41950_e55022: f64 = (var_gf2 * 0.25);
        let assign41950_e55023: f64 = (var_xg + assign41950_e55022);
        let assign41950_e55025: f64 = (assign41950_e55023 - var_sp_s_w);
        let assign41950_e55026: f64 = (assign41950_e55025).sqrt();
        let assign41950_e55027: f64 = (var_gf * assign41950_e55026);
        let assign41950_e55028: f64 = (assign41950_e55017 - assign41950_e55027);
        (assign41950_e55028, ((var_xg_dn5 + (var_gf2_dn5 * 0.5)) - ((var_gf_dn5 * assign41950_e55026) + (var_gf * (((var_xg_dn5 + (var_gf2_dn5 * 0.25)) - var_sp_s_w_dn5) / (2.0 * assign41950_e55026))))), ((var_xg_dn6 + (var_gf2_dn6 * 0.5)) - ((var_gf_dn6 * assign41950_e55026) + (var_gf * (((var_xg_dn6 + (var_gf2_dn6 * 0.25)) - var_sp_s_w_dn6) / (2.0 * assign41950_e55026))))), ((var_xg_dn7 + (var_gf2_dn7 * 0.5)) - ((var_gf_dn7 * assign41950_e55026) + (var_gf * (((var_xg_dn7 + (var_gf2_dn7 * 0.25)) - var_sp_s_w_dn7) / (2.0 * assign41950_e55026))))), ((var_xg_dn8 + (var_gf2_dn8 * 0.5)) - ((var_gf_dn8 * assign41950_e55026) + (var_gf * (((var_xg_dn8 + (var_gf2_dn8 * 0.25)) - var_sp_s_w_dn8) / (2.0 * assign41950_e55026))))),)
    } else {
        (var_sp_s_x1, var_sp_s_x1_dn5, var_sp_s_x1_dn6, var_sp_s_x1_dn7, var_sp_s_x1_dn8,)
    }
};
        var_sp_s_x1 = assign41950_e55030;
        var_sp_s_x1_dn5 = assign41950_e55030_d_n5;
        var_sp_s_x1_dn6 = assign41950_e55030_d_n6;
        var_sp_s_x1_dn7 = assign41950_e55030_d_n7;
        var_sp_s_x1_dn8 = assign41950_e55030_d_n8;
        var_sp_s_x1_rv = 0.0;

        let (assign41960_e55040, assign41960_e55040_d_n5, assign41960_e55040_d_n6, assign41960_e55040_d_n7, assign41960_e55040_d_n8,) = {
    if ((var_guard1182 == 0.0) && (var_guard1183 == 0.0)) {
        let assign41960_e55038: f64 = (var_xn_s + 3.0);
        (assign41960_e55038, var_xn_s_dn5, var_xn_s_dn6, var_xn_s_dn7, var_xn_s_dn8,)
    } else {
        (var_sp_s_bx, var_sp_s_bx_dn5, var_sp_s_bx_dn6, var_sp_s_bx_dn7, var_sp_s_bx_dn8,)
    }
};
        var_sp_s_bx = assign41960_e55040;
        var_sp_s_bx_dn5 = assign41960_e55040_d_n5;
        var_sp_s_bx_dn6 = assign41960_e55040_d_n6;
        var_sp_s_bx_dn7 = assign41960_e55040_d_n7;
        var_sp_s_bx_dn8 = assign41960_e55040_d_n8;
        var_sp_s_bx_rv = 0.0;

        let (assign41970_e55074, assign41970_e55074_d_n5, assign41970_e55074_d_n6, assign41970_e55074_d_n7, assign41970_e55074_d_n8,) = {
    if ((var_guard1182 == 0.0) && (var_guard1183 == 0.0)) {
        let assign41970_e55049: f64 = (var_sp_s_x1 + var_sp_s_bx);
        let assign41970_e55052: f64 = (var_sp_s_x1 - var_sp_s_bx);
        let assign41970_e55055: f64 = (var_sp_s_x1 - var_sp_s_bx);
        let assign41970_e55056: f64 = (assign41970_e55052 * assign41970_e55055);
        let assign41970_e55058: f64 = (assign41970_e55056 + 5.0);
        let assign41970_e55059: f64 = (assign41970_e55058).sqrt();
        let assign41970_e55060: f64 = (assign41970_e55049 - assign41970_e55059);
        let assign41970_e55061: f64 = (0.5 * assign41970_e55060);
        let assign41970_e55066: f64 = (var_sp_s_bx * var_sp_s_bx);
        let assign41970_e55068: f64 = (assign41970_e55066 + 5.0);
        let assign41970_e55069: f64 = (assign41970_e55068).sqrt();
        let assign41970_e55070: f64 = (var_sp_s_bx - assign41970_e55069);
        let assign41970_e55071: f64 = (0.5 * assign41970_e55070);
        let assign41970_e55072: f64 = (assign41970_e55061 - assign41970_e55071);
        (assign41970_e55072, ((0.5 * ((var_sp_s_x1_dn5 + var_sp_s_bx_dn5) - ((((var_sp_s_x1_dn5 - var_sp_s_bx_dn5) * assign41970_e55055) + (assign41970_e55052 * (var_sp_s_x1_dn5 - var_sp_s_bx_dn5))) / (2.0 * assign41970_e55059)))) - (0.5 * (var_sp_s_bx_dn5 - (((var_sp_s_bx_dn5 * var_sp_s_bx) + (var_sp_s_bx * var_sp_s_bx_dn5)) / (2.0 * assign41970_e55069))))), ((0.5 * ((var_sp_s_x1_dn6 + var_sp_s_bx_dn6) - ((((var_sp_s_x1_dn6 - var_sp_s_bx_dn6) * assign41970_e55055) + (assign41970_e55052 * (var_sp_s_x1_dn6 - var_sp_s_bx_dn6))) / (2.0 * assign41970_e55059)))) - (0.5 * (var_sp_s_bx_dn6 - (((var_sp_s_bx_dn6 * var_sp_s_bx) + (var_sp_s_bx * var_sp_s_bx_dn6)) / (2.0 * assign41970_e55069))))), ((0.5 * ((var_sp_s_x1_dn7 + var_sp_s_bx_dn7) - ((((var_sp_s_x1_dn7 - var_sp_s_bx_dn7) * assign41970_e55055) + (assign41970_e55052 * (var_sp_s_x1_dn7 - var_sp_s_bx_dn7))) / (2.0 * assign41970_e55059)))) - (0.5 * (var_sp_s_bx_dn7 - (((var_sp_s_bx_dn7 * var_sp_s_bx) + (var_sp_s_bx * var_sp_s_bx_dn7)) / (2.0 * assign41970_e55069))))), ((0.5 * ((var_sp_s_x1_dn8 + var_sp_s_bx_dn8) - ((((var_sp_s_x1_dn8 - var_sp_s_bx_dn8) * assign41970_e55055) + (assign41970_e55052 * (var_sp_s_x1_dn8 - var_sp_s_bx_dn8))) / (2.0 * assign41970_e55059)))) - (0.5 * (var_sp_s_bx_dn8 - (((var_sp_s_bx_dn8 * var_sp_s_bx) + (var_sp_s_bx * var_sp_s_bx_dn8)) / (2.0 * assign41970_e55069))))),)
    } else {
        (var_sp_s_eta, var_sp_s_eta_dn5, var_sp_s_eta_dn6, var_sp_s_eta_dn7, var_sp_s_eta_dn8,)
    }
};
        var_sp_s_eta = assign41970_e55074;
        var_sp_s_eta_dn5 = assign41970_e55074_d_n5;
        var_sp_s_eta_dn6 = assign41970_e55074_d_n6;
        var_sp_s_eta_dn7 = assign41970_e55074_d_n7;
        var_sp_s_eta_dn8 = assign41970_e55074_d_n8;
        var_sp_s_eta_rv = 0.0;

        let (assign41980_e55084, assign41980_e55084_d_n5, assign41980_e55084_d_n6, assign41980_e55084_d_n7, assign41980_e55084_d_n8,) = {
    if ((var_guard1182 == 0.0) && (var_guard1183 == 0.0)) {
        let assign41980_e55082: f64 = (var_xg - var_sp_s_eta);
        (assign41980_e55082, (var_xg_dn5 - var_sp_s_eta_dn5), (var_xg_dn6 - var_sp_s_eta_dn6), (var_xg_dn7 - var_sp_s_eta_dn7), (var_xg_dn8 - var_sp_s_eta_dn8),)
    } else {
        (var_sp_s_temp, var_sp_s_temp_dn5, var_sp_s_temp_dn6, var_sp_s_temp_dn7, var_sp_s_temp_dn8,)
    }
};
        var_sp_s_temp = assign41980_e55084;
        var_sp_s_temp_dn5 = assign41980_e55084_d_n5;
        var_sp_s_temp_dn6 = assign41980_e55084_d_n6;
        var_sp_s_temp_dn7 = assign41980_e55084_d_n7;
        var_sp_s_temp_dn8 = assign41980_e55084_d_n8;
        var_sp_s_temp_rv = 0.0;

        let (assign41990_e55094, assign41990_e55094_d_n5, assign41990_e55094_d_n6, assign41990_e55094_d_n7, assign41990_e55094_d_n8,) = {
    if ((var_guard1182 == 0.0) && (var_guard1183 == 0.0)) {
        let assign41990_e55091: f64 = (-var_sp_s_eta);
        let assign41990_e55092: f64 = (assign41990_e55091).exp();
        (assign41990_e55092, (assign41990_e55092 * (-var_sp_s_eta_dn5)), (assign41990_e55092 * (-var_sp_s_eta_dn6)), (assign41990_e55092 * (-var_sp_s_eta_dn7)), (assign41990_e55092 * (-var_sp_s_eta_dn8)),)
    } else {
        (var_sp_s_temp1, var_sp_s_temp1_dn5, var_sp_s_temp1_dn6, var_sp_s_temp1_dn7, var_sp_s_temp1_dn8,)
    }
};
        var_sp_s_temp1 = assign41990_e55094;
        var_sp_s_temp1_dn5 = assign41990_e55094_d_n5;
        var_sp_s_temp1_dn6 = assign41990_e55094_d_n6;
        var_sp_s_temp1_dn7 = assign41990_e55094_d_n7;
        var_sp_s_temp1_dn8 = assign41990_e55094_d_n8;
        var_sp_s_temp1_rv = 0.0;

        let (assign42000_e55108, assign42000_e55108_d_n5, assign42000_e55108_d_n6, assign42000_e55108_d_n7, assign42000_e55108_d_n8,) = {
    if ((var_guard1182 == 0.0) && (var_guard1183 == 0.0)) {
        let assign42000_e55104: f64 = (var_sp_s_eta * var_sp_s_eta);
        let assign42000_e55105: f64 = (2.0 + assign42000_e55104);
        let assign42000_e55106: f64 = (1.0 / assign42000_e55105);
        (assign42000_e55106, (-(((var_sp_s_eta_dn5 * var_sp_s_eta) + (var_sp_s_eta * var_sp_s_eta_dn5)) / (assign42000_e55105 * assign42000_e55105))), (-(((var_sp_s_eta_dn6 * var_sp_s_eta) + (var_sp_s_eta * var_sp_s_eta_dn6)) / (assign42000_e55105 * assign42000_e55105))), (-(((var_sp_s_eta_dn7 * var_sp_s_eta) + (var_sp_s_eta * var_sp_s_eta_dn7)) / (assign42000_e55105 * assign42000_e55105))), (-(((var_sp_s_eta_dn8 * var_sp_s_eta) + (var_sp_s_eta * var_sp_s_eta_dn8)) / (assign42000_e55105 * assign42000_e55105))),)
    } else {
        (var_sp_s_temp2, var_sp_s_temp2_dn5, var_sp_s_temp2_dn6, var_sp_s_temp2_dn7, var_sp_s_temp2_dn8,)
    }
};
        var_sp_s_temp2 = assign42000_e55108;
        var_sp_s_temp2_dn5 = assign42000_e55108_d_n5;
        var_sp_s_temp2_dn6 = assign42000_e55108_d_n6;
        var_sp_s_temp2_dn7 = assign42000_e55108_d_n7;
        var_sp_s_temp2_dn8 = assign42000_e55108_d_n8;
        var_sp_s_temp2_rv = 0.0;

        let (assign42010_e55120, assign42010_e55120_d_n5, assign42010_e55120_d_n6, assign42010_e55120_d_n7, assign42010_e55120_d_n8,) = {
    if ((var_guard1182 == 0.0) && (var_guard1183 == 0.0)) {
        let assign42010_e55116: f64 = (var_sp_s_eta * var_sp_s_eta);
        let assign42010_e55118: f64 = (assign42010_e55116 * var_sp_s_temp2);
        (assign42010_e55118, ((((var_sp_s_eta_dn5 * var_sp_s_eta) + (var_sp_s_eta * var_sp_s_eta_dn5)) * var_sp_s_temp2) + (assign42010_e55116 * var_sp_s_temp2_dn5)), ((((var_sp_s_eta_dn6 * var_sp_s_eta) + (var_sp_s_eta * var_sp_s_eta_dn6)) * var_sp_s_temp2) + (assign42010_e55116 * var_sp_s_temp2_dn6)), ((((var_sp_s_eta_dn7 * var_sp_s_eta) + (var_sp_s_eta * var_sp_s_eta_dn7)) * var_sp_s_temp2) + (assign42010_e55116 * var_sp_s_temp2_dn7)), ((((var_sp_s_eta_dn8 * var_sp_s_eta) + (var_sp_s_eta * var_sp_s_eta_dn8)) * var_sp_s_temp2) + (assign42010_e55116 * var_sp_s_temp2_dn8)),)
    } else {
        (var_sp_s_xi0, var_sp_s_xi0_dn5, var_sp_s_xi0_dn6, var_sp_s_xi0_dn7, var_sp_s_xi0_dn8,)
    }
};
        var_sp_s_xi0 = assign42010_e55120;
        var_sp_s_xi0_dn5 = assign42010_e55120_d_n5;
        var_sp_s_xi0_dn6 = assign42010_e55120_d_n6;
        var_sp_s_xi0_dn7 = assign42010_e55120_d_n7;
        var_sp_s_xi0_dn8 = assign42010_e55120_d_n8;
        var_sp_s_xi0_rv = 0.0;

        let (assign42020_e55134, assign42020_e55134_d_n5, assign42020_e55134_d_n6, assign42020_e55134_d_n7, assign42020_e55134_d_n8,) = {
    if ((var_guard1182 == 0.0) && (var_guard1183 == 0.0)) {
        let assign42020_e55129: f64 = (var_sp_s_eta * var_sp_s_temp2);
        let assign42020_e55131: f64 = (assign42020_e55129 * var_sp_s_temp2);
        let assign42020_e55132: f64 = (4.0 * assign42020_e55131);
        (assign42020_e55132, (4.0 * ((((var_sp_s_eta_dn5 * var_sp_s_temp2) + (var_sp_s_eta * var_sp_s_temp2_dn5)) * var_sp_s_temp2) + (assign42020_e55129 * var_sp_s_temp2_dn5))), (4.0 * ((((var_sp_s_eta_dn6 * var_sp_s_temp2) + (var_sp_s_eta * var_sp_s_temp2_dn6)) * var_sp_s_temp2) + (assign42020_e55129 * var_sp_s_temp2_dn6))), (4.0 * ((((var_sp_s_eta_dn7 * var_sp_s_temp2) + (var_sp_s_eta * var_sp_s_temp2_dn7)) * var_sp_s_temp2) + (assign42020_e55129 * var_sp_s_temp2_dn7))), (4.0 * ((((var_sp_s_eta_dn8 * var_sp_s_temp2) + (var_sp_s_eta * var_sp_s_temp2_dn8)) * var_sp_s_temp2) + (assign42020_e55129 * var_sp_s_temp2_dn8))),)
    } else {
        (var_sp_s_xi1, var_sp_s_xi1_dn5, var_sp_s_xi1_dn6, var_sp_s_xi1_dn7, var_sp_s_xi1_dn8,)
    }
};
        var_sp_s_xi1 = assign42020_e55134;
        var_sp_s_xi1_dn5 = assign42020_e55134_d_n5;
        var_sp_s_xi1_dn6 = assign42020_e55134_d_n6;
        var_sp_s_xi1_dn7 = assign42020_e55134_d_n7;
        var_sp_s_xi1_dn8 = assign42020_e55134_d_n8;
        var_sp_s_xi1_rv = 0.0;

        let (assign42030_e55152, assign42030_e55152_d_n5, assign42030_e55152_d_n6, assign42030_e55152_d_n7, assign42030_e55152_d_n8,) = {
    if ((var_guard1182 == 0.0) && (var_guard1183 == 0.0)) {
        let assign42030_e55142: f64 = (8.0 * var_sp_s_temp2);
        let assign42030_e55145: f64 = (12.0 * var_sp_s_xi0);
        let assign42030_e55146: f64 = (assign42030_e55142 - assign42030_e55145);
        let assign42030_e55148: f64 = (assign42030_e55146 * var_sp_s_temp2);
        let assign42030_e55150: f64 = (assign42030_e55148 * var_sp_s_temp2);
        (assign42030_e55150, ((((((8.0 * var_sp_s_temp2_dn5) - (12.0 * var_sp_s_xi0_dn5)) * var_sp_s_temp2) + (assign42030_e55146 * var_sp_s_temp2_dn5)) * var_sp_s_temp2) + (assign42030_e55148 * var_sp_s_temp2_dn5)), ((((((8.0 * var_sp_s_temp2_dn6) - (12.0 * var_sp_s_xi0_dn6)) * var_sp_s_temp2) + (assign42030_e55146 * var_sp_s_temp2_dn6)) * var_sp_s_temp2) + (assign42030_e55148 * var_sp_s_temp2_dn6)), ((((((8.0 * var_sp_s_temp2_dn7) - (12.0 * var_sp_s_xi0_dn7)) * var_sp_s_temp2) + (assign42030_e55146 * var_sp_s_temp2_dn7)) * var_sp_s_temp2) + (assign42030_e55148 * var_sp_s_temp2_dn7)), ((((((8.0 * var_sp_s_temp2_dn8) - (12.0 * var_sp_s_xi0_dn8)) * var_sp_s_temp2) + (assign42030_e55146 * var_sp_s_temp2_dn8)) * var_sp_s_temp2) + (assign42030_e55148 * var_sp_s_temp2_dn8)),)
    } else {
        (var_sp_s_xi2, var_sp_s_xi2_dn5, var_sp_s_xi2_dn6, var_sp_s_xi2_dn7, var_sp_s_xi2_dn8,)
    }
};
        var_sp_s_xi2 = assign42030_e55152;
        var_sp_s_xi2_dn5 = assign42030_e55152_d_n5;
        var_sp_s_xi2_dn6 = assign42030_e55152_d_n6;
        var_sp_s_xi2_dn7 = assign42030_e55152_d_n7;
        var_sp_s_xi2_dn8 = assign42030_e55152_d_n8;
        var_sp_s_xi2_rv = 0.0;

        let (assign42040_e55201, assign42040_e55201_d_n5, assign42040_e55201_d_n6, assign42040_e55201_d_n7, assign42040_e55201_d_n8,) = {
    if ((var_guard1182 == 0.0) && (var_guard1183 == 0.0)) {
        let assign42040_e55161: f64 = (var_sp_s_temp * var_sp_s_temp);
        let assign42040_e55165: f64 = (var_sp_s_temp1 + var_sp_s_eta);
        let assign42040_e55167: f64 = (assign42040_e55165 - 1.0);
        let assign42040_e55171: f64 = (var_sp_s_eta + 1.0);
        let assign42040_e55173: f64 = (assign42040_e55171 + var_sp_s_xi0);
        let assign42040_e55174: f64 = (var_delta_ns * assign42040_e55173);
        let assign42040_e55175: f64 = (assign42040_e55167 - assign42040_e55174);
        let assign42040_e55176: f64 = (var_gf2 * assign42040_e55175);
        let assign42040_e55177: f64 = (assign42040_e55161 - assign42040_e55176);
        let (assign42040_e55199, assign42040_e55199_d_n5, assign42040_e55199_d_n6, assign42040_e55199_d_n7, assign42040_e55199_d_n8,) = {
            if (1e-40 > assign42040_e55177) {
                (1e-40, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign42040_e55182: f64 = (var_sp_s_temp * var_sp_s_temp);
                let assign42040_e55186: f64 = (var_sp_s_temp1 + var_sp_s_eta);
                let assign42040_e55188: f64 = (assign42040_e55186 - 1.0);
                let assign42040_e55192: f64 = (var_sp_s_eta + 1.0);
                let assign42040_e55194: f64 = (assign42040_e55192 + var_sp_s_xi0);
                let assign42040_e55195: f64 = (var_delta_ns * assign42040_e55194);
                let assign42040_e55196: f64 = (assign42040_e55188 - assign42040_e55195);
                let assign42040_e55197: f64 = (var_gf2 * assign42040_e55196);
                let assign42040_e55198: f64 = (assign42040_e55182 - assign42040_e55197);
                (assign42040_e55198, (((var_sp_s_temp_dn5 * var_sp_s_temp) + (var_sp_s_temp * var_sp_s_temp_dn5)) - ((var_gf2_dn5 * assign42040_e55196) + (var_gf2 * ((var_sp_s_temp1_dn5 + var_sp_s_eta_dn5) - ((var_delta_ns_dn5 * assign42040_e55194) + (var_delta_ns * (var_sp_s_eta_dn5 + var_sp_s_xi0_dn5))))))), (((var_sp_s_temp_dn6 * var_sp_s_temp) + (var_sp_s_temp * var_sp_s_temp_dn6)) - ((var_gf2_dn6 * assign42040_e55196) + (var_gf2 * ((var_sp_s_temp1_dn6 + var_sp_s_eta_dn6) - ((var_delta_ns_dn6 * assign42040_e55194) + (var_delta_ns * (var_sp_s_eta_dn6 + var_sp_s_xi0_dn6))))))), (((var_sp_s_temp_dn7 * var_sp_s_temp) + (var_sp_s_temp * var_sp_s_temp_dn7)) - ((var_gf2_dn7 * assign42040_e55196) + (var_gf2 * ((var_sp_s_temp1_dn7 + var_sp_s_eta_dn7) - ((var_delta_ns_dn7 * assign42040_e55194) + (var_delta_ns * (var_sp_s_eta_dn7 + var_sp_s_xi0_dn7))))))), (((var_sp_s_temp_dn8 * var_sp_s_temp) + (var_sp_s_temp * var_sp_s_temp_dn8)) - ((var_gf2_dn8 * assign42040_e55196) + (var_gf2 * ((var_sp_s_temp1_dn8 + var_sp_s_eta_dn8) - ((var_delta_ns_dn8 * assign42040_e55194) + (var_delta_ns * (var_sp_s_eta_dn8 + var_sp_s_xi0_dn8))))))),)
            }
        };
        (assign42040_e55199, assign42040_e55199_d_n5, assign42040_e55199_d_n6, assign42040_e55199_d_n7, assign42040_e55199_d_n8,)
    } else {
        (var_sp_s_a, var_sp_s_a_dn5, var_sp_s_a_dn6, var_sp_s_a_dn7, var_sp_s_a_dn8,)
    }
};
        var_sp_s_a = assign42040_e55201;
        var_sp_s_a_dn5 = assign42040_e55201_d_n5;
        var_sp_s_a_dn6 = assign42040_e55201_d_n6;
        var_sp_s_a_dn7 = assign42040_e55201_d_n7;
        var_sp_s_a_dn8 = assign42040_e55201_d_n8;
        var_sp_s_a_rv = 0.0;

        let (assign42050_e55219, assign42050_e55219_d_n5, assign42050_e55219_d_n6, assign42050_e55219_d_n7, assign42050_e55219_d_n8,) = {
    if ((var_guard1182 == 0.0) && (var_guard1183 == 0.0)) {
        let assign42050_e55213: f64 = (var_delta_ns * var_sp_s_xi2);
        let assign42050_e55214: f64 = (var_sp_s_temp1 - assign42050_e55213);
        let assign42050_e55215: f64 = (var_gf2 * assign42050_e55214);
        let assign42050_e55216: f64 = (0.5 * assign42050_e55215);
        let assign42050_e55217: f64 = (1.0 - assign42050_e55216);
        (assign42050_e55217, (-(0.5 * ((var_gf2_dn5 * assign42050_e55214) + (var_gf2 * (var_sp_s_temp1_dn5 - ((var_delta_ns_dn5 * var_sp_s_xi2) + (var_delta_ns * var_sp_s_xi2_dn5))))))), (-(0.5 * ((var_gf2_dn6 * assign42050_e55214) + (var_gf2 * (var_sp_s_temp1_dn6 - ((var_delta_ns_dn6 * var_sp_s_xi2) + (var_delta_ns * var_sp_s_xi2_dn6))))))), (-(0.5 * ((var_gf2_dn7 * assign42050_e55214) + (var_gf2 * (var_sp_s_temp1_dn7 - ((var_delta_ns_dn7 * var_sp_s_xi2) + (var_delta_ns * var_sp_s_xi2_dn7))))))), (-(0.5 * ((var_gf2_dn8 * assign42050_e55214) + (var_gf2 * (var_sp_s_temp1_dn8 - ((var_delta_ns_dn8 * var_sp_s_xi2) + (var_delta_ns * var_sp_s_xi2_dn8))))))),)
    } else {
        (var_sp_s_b, var_sp_s_b_dn5, var_sp_s_b_dn6, var_sp_s_b_dn7, var_sp_s_b_dn8,)
    }
};
        var_sp_s_b = assign42050_e55219;
        var_sp_s_b_dn5 = assign42050_e55219_d_n5;
        var_sp_s_b_dn6 = assign42050_e55219_d_n6;
        var_sp_s_b_dn7 = assign42050_e55219_d_n7;
        var_sp_s_b_dn8 = assign42050_e55219_d_n8;
        var_sp_s_b_rv = 0.0;

        let (assign42060_e55241, assign42060_e55241_d_n5, assign42060_e55241_d_n6, assign42060_e55241_d_n7, assign42060_e55241_d_n8,) = {
    if ((var_guard1182 == 0.0) && (var_guard1183 == 0.0)) {
        let assign42060_e55227: f64 = (2.0 * var_sp_s_temp);
        let assign42060_e55231: f64 = (1.0 - var_sp_s_temp1);
        let assign42060_e55235: f64 = (1.0 + var_sp_s_xi1);
        let assign42060_e55236: f64 = (var_delta_ns * assign42060_e55235);
        let assign42060_e55237: f64 = (assign42060_e55231 - assign42060_e55236);
        let assign42060_e55238: f64 = (var_gf2 * assign42060_e55237);
        let assign42060_e55239: f64 = (assign42060_e55227 + assign42060_e55238);
        (assign42060_e55239, ((2.0 * var_sp_s_temp_dn5) + ((var_gf2_dn5 * assign42060_e55237) + (var_gf2 * ((-var_sp_s_temp1_dn5) - ((var_delta_ns_dn5 * assign42060_e55235) + (var_delta_ns * var_sp_s_xi1_dn5)))))), ((2.0 * var_sp_s_temp_dn6) + ((var_gf2_dn6 * assign42060_e55237) + (var_gf2 * ((-var_sp_s_temp1_dn6) - ((var_delta_ns_dn6 * assign42060_e55235) + (var_delta_ns * var_sp_s_xi1_dn6)))))), ((2.0 * var_sp_s_temp_dn7) + ((var_gf2_dn7 * assign42060_e55237) + (var_gf2 * ((-var_sp_s_temp1_dn7) - ((var_delta_ns_dn7 * assign42060_e55235) + (var_delta_ns * var_sp_s_xi1_dn7)))))), ((2.0 * var_sp_s_temp_dn8) + ((var_gf2_dn8 * assign42060_e55237) + (var_gf2 * ((-var_sp_s_temp1_dn8) - ((var_delta_ns_dn8 * assign42060_e55235) + (var_delta_ns * var_sp_s_xi1_dn8)))))),)
    } else {
        (var_sp_s_c, var_sp_s_c_dn5, var_sp_s_c_dn6, var_sp_s_c_dn7, var_sp_s_c_dn8,)
    }
};
        var_sp_s_c = assign42060_e55241;
        var_sp_s_c_dn5 = assign42060_e55241_d_n5;
        var_sp_s_c_dn6 = assign42060_e55241_d_n6;
        var_sp_s_c_dn7 = assign42060_e55241_d_n7;
        var_sp_s_c_dn8 = assign42060_e55241_d_n8;
        var_sp_s_c_rv = 0.0;

        let (assign42070_e55256, assign42070_e55256_d_n5, assign42070_e55256_d_n6, assign42070_e55256_d_n7, assign42070_e55256_d_n8,) = {
    if ((var_guard1182 == 0.0) && (var_guard1183 == 0.0)) {
        let assign42070_e55249: f64 = (var_xn_s - var_sp_s_eta);
        let assign42070_e55252: f64 = (var_sp_s_a / var_gf2);
        let assign42070_e55253: f64 = (assign42070_e55252).ln();
        let assign42070_e55254: f64 = (assign42070_e55249 + assign42070_e55253);
        (assign42070_e55254, ((var_xn_s_dn5 - var_sp_s_eta_dn5) + ((((var_sp_s_a_dn5 * var_gf2) - (var_sp_s_a * var_gf2_dn5)) / (var_gf2 * var_gf2)) / assign42070_e55252)), ((var_xn_s_dn6 - var_sp_s_eta_dn6) + ((((var_sp_s_a_dn6 * var_gf2) - (var_sp_s_a * var_gf2_dn6)) / (var_gf2 * var_gf2)) / assign42070_e55252)), ((var_xn_s_dn7 - var_sp_s_eta_dn7) + ((((var_sp_s_a_dn7 * var_gf2) - (var_sp_s_a * var_gf2_dn7)) / (var_gf2 * var_gf2)) / assign42070_e55252)), ((var_xn_s_dn8 - var_sp_s_eta_dn8) + ((((var_sp_s_a_dn8 * var_gf2) - (var_sp_s_a * var_gf2_dn8)) / (var_gf2 * var_gf2)) / assign42070_e55252)),)
    } else {
        (var_sp_s_tau, var_sp_s_tau_dn5, var_sp_s_tau_dn6, var_sp_s_tau_dn7, var_sp_s_tau_dn8,)
    }
};
        var_sp_s_tau = assign42070_e55256;
        var_sp_s_tau_dn5 = assign42070_e55256_d_n5;
        var_sp_s_tau_dn6 = assign42070_e55256_d_n6;
        var_sp_s_tau_dn7 = assign42070_e55256_d_n7;
        var_sp_s_tau_dn8 = assign42070_e55256_d_n8;
        var_sp_s_tau_rv = 0.0;

        let (assign42080_e55266, assign42080_e55266_d_n5, assign42080_e55266_d_n6, assign42080_e55266_d_n7, assign42080_e55266_d_n8,) = {
    if ((var_guard1182 == 0.0) && (var_guard1183 == 0.0)) {
        let assign42080_e55264: f64 = (var_sp_s_a + var_sp_s_c);
        (assign42080_e55264, (var_sp_s_a_dn5 + var_sp_s_c_dn5), (var_sp_s_a_dn6 + var_sp_s_c_dn6), (var_sp_s_a_dn7 + var_sp_s_c_dn7), (var_sp_s_a_dn8 + var_sp_s_c_dn8),)
    } else {
        (var_nu, var_nu_dn5, var_nu_dn6, var_nu_dn7, var_nu_dn8,)
    }
};
        var_nu = assign42080_e55266;
        var_nu_dn5 = assign42080_e55266_d_n5;
        var_nu_dn6 = assign42080_e55266_d_n6;
        var_nu_dn7 = assign42080_e55266_d_n7;
        var_nu_dn8 = assign42080_e55266_d_n8;
        var_nu_rv = 0.0;

        let (assign42090_e55288, assign42090_e55288_d_n5, assign42090_e55288_d_n6, assign42090_e55288_d_n7, assign42090_e55288_d_n8,) = {
    if ((var_guard1182 == 0.0) && (var_guard1183 == 0.0)) {
        let assign42090_e55274: f64 = (var_nu * var_nu);
        let assign42090_e55279: f64 = (var_sp_s_c * var_sp_s_c);
        let assign42090_e55280: f64 = (0.5 * assign42090_e55279);
        let assign42090_e55283: f64 = (var_sp_s_a * var_sp_s_b);
        let assign42090_e55284: f64 = (assign42090_e55280 - assign42090_e55283);
        let assign42090_e55285: f64 = (var_sp_s_tau * assign42090_e55284);
        let assign42090_e55286: f64 = (assign42090_e55274 + assign42090_e55285);
        (assign42090_e55286, (((var_nu_dn5 * var_nu) + (var_nu * var_nu_dn5)) + ((var_sp_s_tau_dn5 * assign42090_e55284) + (var_sp_s_tau * ((0.5 * ((var_sp_s_c_dn5 * var_sp_s_c) + (var_sp_s_c * var_sp_s_c_dn5))) - ((var_sp_s_a_dn5 * var_sp_s_b) + (var_sp_s_a * var_sp_s_b_dn5)))))), (((var_nu_dn6 * var_nu) + (var_nu * var_nu_dn6)) + ((var_sp_s_tau_dn6 * assign42090_e55284) + (var_sp_s_tau * ((0.5 * ((var_sp_s_c_dn6 * var_sp_s_c) + (var_sp_s_c * var_sp_s_c_dn6))) - ((var_sp_s_a_dn6 * var_sp_s_b) + (var_sp_s_a * var_sp_s_b_dn6)))))), (((var_nu_dn7 * var_nu) + (var_nu * var_nu_dn7)) + ((var_sp_s_tau_dn7 * assign42090_e55284) + (var_sp_s_tau * ((0.5 * ((var_sp_s_c_dn7 * var_sp_s_c) + (var_sp_s_c * var_sp_s_c_dn7))) - ((var_sp_s_a_dn7 * var_sp_s_b) + (var_sp_s_a * var_sp_s_b_dn7)))))), (((var_nu_dn8 * var_nu) + (var_nu * var_nu_dn8)) + ((var_sp_s_tau_dn8 * assign42090_e55284) + (var_sp_s_tau * ((0.5 * ((var_sp_s_c_dn8 * var_sp_s_c) + (var_sp_s_c * var_sp_s_c_dn8))) - ((var_sp_s_a_dn8 * var_sp_s_b) + (var_sp_s_a * var_sp_s_b_dn8)))))),)
    } else {
        (var_mutau, var_mutau_dn5, var_mutau_dn6, var_mutau_dn7, var_mutau_dn8,)
    }
};
        var_mutau = assign42090_e55288;
        var_mutau_dn5 = assign42090_e55288_d_n5;
        var_mutau_dn6 = assign42090_e55288_d_n6;
        var_mutau_dn7 = assign42090_e55288_d_n7;
        var_mutau_dn8 = assign42090_e55288_d_n8;
        var_mutau_rv = 0.0;

        let (assign42100_e55324, assign42100_e55324_d_n5, assign42100_e55324_d_n6, assign42100_e55324_d_n7, assign42100_e55324_d_n8,) = {
    if ((var_guard1182 == 0.0) && (var_guard1183 == 0.0)) {
        let assign42100_e55297: f64 = (var_sp_s_a * var_nu);
        let assign42100_e55299: f64 = (assign42100_e55297 * var_sp_s_tau);
        let assign42100_e55303: f64 = (var_nu / var_mutau);
        let assign42100_e55305: f64 = (assign42100_e55303 * var_sp_s_tau);
        let assign42100_e55307: f64 = (assign42100_e55305 * var_sp_s_tau);
        let assign42100_e55309: f64 = (assign42100_e55307 * var_sp_s_c);
        let assign42100_e55312: f64 = (var_sp_s_c * var_sp_s_c);
        let assign42100_e55314: f64 = (assign42100_e55312 * 0.3333333333333333);
        let assign42100_e55317: f64 = (var_sp_s_a * var_sp_s_b);
        let assign42100_e55318: f64 = (assign42100_e55314 - assign42100_e55317);
        let assign42100_e55319: f64 = (assign42100_e55309 * assign42100_e55318);
        let assign42100_e55320: f64 = (var_mutau + assign42100_e55319);
        let assign42100_e55321: f64 = (assign42100_e55299 / assign42100_e55320);
        let assign42100_e55322: f64 = (var_sp_s_eta + assign42100_e55321);
        (assign42100_e55322, (var_sp_s_eta_dn5 + (((((((var_sp_s_a_dn5 * var_nu) + (var_sp_s_a * var_nu_dn5)) * var_sp_s_tau) + (assign42100_e55297 * var_sp_s_tau_dn5)) * assign42100_e55320) - (assign42100_e55299 * (var_mutau_dn5 + (((((((((((var_nu_dn5 * var_mutau) - (var_nu * var_mutau_dn5)) / (var_mutau * var_mutau)) * var_sp_s_tau) + (assign42100_e55303 * var_sp_s_tau_dn5)) * var_sp_s_tau) + (assign42100_e55305 * var_sp_s_tau_dn5)) * var_sp_s_c) + (assign42100_e55307 * var_sp_s_c_dn5)) * assign42100_e55318) + (assign42100_e55309 * ((((var_sp_s_c_dn5 * var_sp_s_c) + (var_sp_s_c * var_sp_s_c_dn5)) * 0.3333333333333333) - ((var_sp_s_a_dn5 * var_sp_s_b) + (var_sp_s_a * var_sp_s_b_dn5)))))))) / (assign42100_e55320 * assign42100_e55320))), (var_sp_s_eta_dn6 + (((((((var_sp_s_a_dn6 * var_nu) + (var_sp_s_a * var_nu_dn6)) * var_sp_s_tau) + (assign42100_e55297 * var_sp_s_tau_dn6)) * assign42100_e55320) - (assign42100_e55299 * (var_mutau_dn6 + (((((((((((var_nu_dn6 * var_mutau) - (var_nu * var_mutau_dn6)) / (var_mutau * var_mutau)) * var_sp_s_tau) + (assign42100_e55303 * var_sp_s_tau_dn6)) * var_sp_s_tau) + (assign42100_e55305 * var_sp_s_tau_dn6)) * var_sp_s_c) + (assign42100_e55307 * var_sp_s_c_dn6)) * assign42100_e55318) + (assign42100_e55309 * ((((var_sp_s_c_dn6 * var_sp_s_c) + (var_sp_s_c * var_sp_s_c_dn6)) * 0.3333333333333333) - ((var_sp_s_a_dn6 * var_sp_s_b) + (var_sp_s_a * var_sp_s_b_dn6)))))))) / (assign42100_e55320 * assign42100_e55320))), (var_sp_s_eta_dn7 + (((((((var_sp_s_a_dn7 * var_nu) + (var_sp_s_a * var_nu_dn7)) * var_sp_s_tau) + (assign42100_e55297 * var_sp_s_tau_dn7)) * assign42100_e55320) - (assign42100_e55299 * (var_mutau_dn7 + (((((((((((var_nu_dn7 * var_mutau) - (var_nu * var_mutau_dn7)) / (var_mutau * var_mutau)) * var_sp_s_tau) + (assign42100_e55303 * var_sp_s_tau_dn7)) * var_sp_s_tau) + (assign42100_e55305 * var_sp_s_tau_dn7)) * var_sp_s_c) + (assign42100_e55307 * var_sp_s_c_dn7)) * assign42100_e55318) + (assign42100_e55309 * ((((var_sp_s_c_dn7 * var_sp_s_c) + (var_sp_s_c * var_sp_s_c_dn7)) * 0.3333333333333333) - ((var_sp_s_a_dn7 * var_sp_s_b) + (var_sp_s_a * var_sp_s_b_dn7)))))))) / (assign42100_e55320 * assign42100_e55320))), (var_sp_s_eta_dn8 + (((((((var_sp_s_a_dn8 * var_nu) + (var_sp_s_a * var_nu_dn8)) * var_sp_s_tau) + (assign42100_e55297 * var_sp_s_tau_dn8)) * assign42100_e55320) - (assign42100_e55299 * (var_mutau_dn8 + (((((((((((var_nu_dn8 * var_mutau) - (var_nu * var_mutau_dn8)) / (var_mutau * var_mutau)) * var_sp_s_tau) + (assign42100_e55303 * var_sp_s_tau_dn8)) * var_sp_s_tau) + (assign42100_e55305 * var_sp_s_tau_dn8)) * var_sp_s_c) + (assign42100_e55307 * var_sp_s_c_dn8)) * assign42100_e55318) + (assign42100_e55309 * ((((var_sp_s_c_dn8 * var_sp_s_c) + (var_sp_s_c * var_sp_s_c_dn8)) * 0.3333333333333333) - ((var_sp_s_a_dn8 * var_sp_s_b) + (var_sp_s_a * var_sp_s_b_dn8)))))))) / (assign42100_e55320 * assign42100_e55320))),)
    } else {
        (var_sp_s_x0, var_sp_s_x0_dn5, var_sp_s_x0_dn6, var_sp_s_x0_dn7, var_sp_s_x0_dn8,)
    }
};
        var_sp_s_x0 = assign42100_e55324;
        var_sp_s_x0_dn5 = assign42100_e55324_d_n5;
        var_sp_s_x0_dn6 = assign42100_e55324_d_n6;
        var_sp_s_x0_dn7 = assign42100_e55324_d_n7;
        var_sp_s_x0_dn8 = assign42100_e55324_d_n8;
        var_sp_s_x0_rv = 0.0;

        let assign42110_e55327: f64 = if var_sp_s_x0 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard1186 = assign42110_e55327;
        var_guard1186_rv = 0.0;

        let (assign42120_e55338, assign42120_e55338_d_n5, assign42120_e55338_d_n6, assign42120_e55338_d_n7, assign42120_e55338_d_n8,) = {
    if (((var_guard1182 == 0.0) && (var_guard1183 == 0.0)) && (var_guard1186 != 0.0)) {
        let assign42120_e55336: f64 = (var_sp_s_x0).exp();
        (assign42120_e55336, (assign42120_e55336 * var_sp_s_x0_dn5), (assign42120_e55336 * var_sp_s_x0_dn6), (assign42120_e55336 * var_sp_s_x0_dn7), (assign42120_e55336 * var_sp_s_x0_dn8),)
    } else {
        (var_sp_s_delta0, var_sp_s_delta0_dn5, var_sp_s_delta0_dn6, var_sp_s_delta0_dn7, var_sp_s_delta0_dn8,)
    }
};
        var_sp_s_delta0 = assign42120_e55338;
        var_sp_s_delta0_dn5 = assign42120_e55338_d_n5;
        var_sp_s_delta0_dn6 = assign42120_e55338_d_n6;
        var_sp_s_delta0_dn7 = assign42120_e55338_d_n7;
        var_sp_s_delta0_dn8 = assign42120_e55338_d_n8;
        var_sp_s_delta0_rv = 0.0;

        let (assign42130_e55350, assign42130_e55350_d_n5, assign42130_e55350_d_n6, assign42130_e55350_d_n7, assign42130_e55350_d_n8,) = {
    if (((var_guard1182 == 0.0) && (var_guard1183 == 0.0)) && (var_guard1186 != 0.0)) {
        let assign42130_e55348: f64 = (1.0 / var_sp_s_delta0);
        (assign42130_e55348, (-(var_sp_s_delta0_dn5 / (var_sp_s_delta0 * var_sp_s_delta0))), (-(var_sp_s_delta0_dn6 / (var_sp_s_delta0 * var_sp_s_delta0))), (-(var_sp_s_delta0_dn7 / (var_sp_s_delta0 * var_sp_s_delta0))), (-(var_sp_s_delta0_dn8 / (var_sp_s_delta0 * var_sp_s_delta0))),)
    } else {
        (var_sp_s_delta1, var_sp_s_delta1_dn5, var_sp_s_delta1_dn6, var_sp_s_delta1_dn7, var_sp_s_delta1_dn8,)
    }
};
        var_sp_s_delta1 = assign42130_e55350;
        var_sp_s_delta1_dn5 = assign42130_e55350_d_n5;
        var_sp_s_delta1_dn6 = assign42130_e55350_d_n6;
        var_sp_s_delta1_dn7 = assign42130_e55350_d_n7;
        var_sp_s_delta1_dn8 = assign42130_e55350_d_n8;
        var_sp_s_delta1_rv = 0.0;

        let (assign42140_e55362, assign42140_e55362_d_n5, assign42140_e55362_d_n6, assign42140_e55362_d_n7, assign42140_e55362_d_n8,) = {
    if (((var_guard1182 == 0.0) && (var_guard1183 == 0.0)) && (var_guard1186 != 0.0)) {
        let assign42140_e55360: f64 = (var_delta_ns * var_sp_s_delta0);
        (assign42140_e55360, ((var_delta_ns_dn5 * var_sp_s_delta0) + (var_delta_ns * var_sp_s_delta0_dn5)), ((var_delta_ns_dn6 * var_sp_s_delta0) + (var_delta_ns * var_sp_s_delta0_dn6)), ((var_delta_ns_dn7 * var_sp_s_delta0) + (var_delta_ns * var_sp_s_delta0_dn7)), ((var_delta_ns_dn8 * var_sp_s_delta0) + (var_delta_ns * var_sp_s_delta0_dn8)),)
    } else {
        (var_sp_s_delta0, var_sp_s_delta0_dn5, var_sp_s_delta0_dn6, var_sp_s_delta0_dn7, var_sp_s_delta0_dn8,)
    }
};
        var_sp_s_delta0 = assign42140_e55362;
        var_sp_s_delta0_dn5 = assign42140_e55362_d_n5;
        var_sp_s_delta0_dn6 = assign42140_e55362_d_n6;
        var_sp_s_delta0_dn7 = assign42140_e55362_d_n7;
        var_sp_s_delta0_dn8 = assign42140_e55362_d_n8;
        var_sp_s_delta0_rv = 0.0;

        let assign42150_e55366: f64 = (var_xn_s - 230.25850929940458);
        let assign42150_e55367: f64 = if var_sp_s_x0 > assign42150_e55366 { 1.0 } else { 0.0 };
        var_guard1187 = assign42150_e55367;
        var_guard1187_rv = 0.0;

        let (assign42160_e55383, assign42160_e55383_d_n5, assign42160_e55383_d_n6, assign42160_e55383_d_n7, assign42160_e55383_d_n8,) = {
    if ((((var_guard1182 == 0.0) && (var_guard1183 == 0.0)) && (var_guard1186 == 0.0)) && (var_guard1187 != 0.0)) {
        let assign42160_e55380: f64 = (var_sp_s_x0 - var_xn_s);
        let assign42160_e55381: f64 = (assign42160_e55380).exp();
        (assign42160_e55381, (assign42160_e55381 * (var_sp_s_x0_dn5 - var_xn_s_dn5)), (assign42160_e55381 * (var_sp_s_x0_dn6 - var_xn_s_dn6)), (assign42160_e55381 * (var_sp_s_x0_dn7 - var_xn_s_dn7)), (assign42160_e55381 * (var_sp_s_x0_dn8 - var_xn_s_dn8)),)
    } else {
        (var_sp_s_delta0, var_sp_s_delta0_dn5, var_sp_s_delta0_dn6, var_sp_s_delta0_dn7, var_sp_s_delta0_dn8,)
    }
};
        var_sp_s_delta0 = assign42160_e55383;
        var_sp_s_delta0_dn5 = assign42160_e55383_d_n5;
        var_sp_s_delta0_dn6 = assign42160_e55383_d_n6;
        var_sp_s_delta0_dn7 = assign42160_e55383_d_n7;
        var_sp_s_delta0_dn8 = assign42160_e55383_d_n8;
        var_sp_s_delta0_rv = 0.0;

        let (assign42170_e55398, assign42170_e55398_d_n5, assign42170_e55398_d_n6, assign42170_e55398_d_n7, assign42170_e55398_d_n8,) = {
    if ((((var_guard1182 == 0.0) && (var_guard1183 == 0.0)) && (var_guard1186 == 0.0)) && (var_guard1187 != 0.0)) {
        let assign42170_e55396: f64 = (var_delta_ns / var_sp_s_delta0);
        (assign42170_e55396, (((var_delta_ns_dn5 * var_sp_s_delta0) - (var_delta_ns * var_sp_s_delta0_dn5)) / (var_sp_s_delta0 * var_sp_s_delta0)), (((var_delta_ns_dn6 * var_sp_s_delta0) - (var_delta_ns * var_sp_s_delta0_dn6)) / (var_sp_s_delta0 * var_sp_s_delta0)), (((var_delta_ns_dn7 * var_sp_s_delta0) - (var_delta_ns * var_sp_s_delta0_dn7)) / (var_sp_s_delta0 * var_sp_s_delta0)), (((var_delta_ns_dn8 * var_sp_s_delta0) - (var_delta_ns * var_sp_s_delta0_dn8)) / (var_sp_s_delta0 * var_sp_s_delta0)),)
    } else {
        (var_sp_s_delta1, var_sp_s_delta1_dn5, var_sp_s_delta1_dn6, var_sp_s_delta1_dn7, var_sp_s_delta1_dn8,)
    }
};
        var_sp_s_delta1 = assign42170_e55398;
        var_sp_s_delta1_dn5 = assign42170_e55398_d_n5;
        var_sp_s_delta1_dn6 = assign42170_e55398_d_n6;
        var_sp_s_delta1_dn7 = assign42170_e55398_d_n7;
        var_sp_s_delta1_dn8 = assign42170_e55398_d_n8;
        var_sp_s_delta1_rv = 0.0;

        let (assign42180_e55440, assign42180_e55440_d_n5, assign42180_e55440_d_n6, assign42180_e55440_d_n7, assign42180_e55440_d_n8,) = {
    if ((((var_guard1182 == 0.0) && (var_guard1183 == 0.0)) && (var_guard1186 == 0.0)) && (var_guard1187 == 0.0)) {
        let assign42180_e55414: f64 = (var_xn_s - var_sp_s_x0);
        let assign42180_e55416: f64 = (assign42180_e55414 - 230.25850929940458);
        let assign42180_e55421: f64 = (var_xn_s - var_sp_s_x0);
        let assign42180_e55423: f64 = (assign42180_e55421 - 230.25850929940458);
        let assign42180_e55427: f64 = (var_xn_s - var_sp_s_x0);
        let assign42180_e55429: f64 = (assign42180_e55427 - 230.25850929940458);
        let assign42180_e55431: f64 = (assign42180_e55429 * 0.3333333333333333);
        let assign42180_e55432: f64 = (1.0 + assign42180_e55431);
        let assign42180_e55433: f64 = (assign42180_e55423 * assign42180_e55432);
        let assign42180_e55434: f64 = (0.5 * assign42180_e55433);
        let assign42180_e55435: f64 = (1.0 + assign42180_e55434);
        let assign42180_e55436: f64 = (assign42180_e55416 * assign42180_e55435);
        let assign42180_e55437: f64 = (1.0 + assign42180_e55436);
        let assign42180_e55438: f64 = (1e-100 / assign42180_e55437);
        (assign42180_e55438, (-((1e-100 * (((var_xn_s_dn5 - var_sp_s_x0_dn5) * assign42180_e55435) + (assign42180_e55416 * (0.5 * (((var_xn_s_dn5 - var_sp_s_x0_dn5) * assign42180_e55432) + (assign42180_e55423 * ((var_xn_s_dn5 - var_sp_s_x0_dn5) * 0.3333333333333333))))))) / (assign42180_e55437 * assign42180_e55437))), (-((1e-100 * (((var_xn_s_dn6 - var_sp_s_x0_dn6) * assign42180_e55435) + (assign42180_e55416 * (0.5 * (((var_xn_s_dn6 - var_sp_s_x0_dn6) * assign42180_e55432) + (assign42180_e55423 * ((var_xn_s_dn6 - var_sp_s_x0_dn6) * 0.3333333333333333))))))) / (assign42180_e55437 * assign42180_e55437))), (-((1e-100 * (((var_xn_s_dn7 - var_sp_s_x0_dn7) * assign42180_e55435) + (assign42180_e55416 * (0.5 * (((var_xn_s_dn7 - var_sp_s_x0_dn7) * assign42180_e55432) + (assign42180_e55423 * ((var_xn_s_dn7 - var_sp_s_x0_dn7) * 0.3333333333333333))))))) / (assign42180_e55437 * assign42180_e55437))), (-((1e-100 * (((var_xn_s_dn8 - var_sp_s_x0_dn8) * assign42180_e55435) + (assign42180_e55416 * (0.5 * (((var_xn_s_dn8 - var_sp_s_x0_dn8) * assign42180_e55432) + (assign42180_e55423 * ((var_xn_s_dn8 - var_sp_s_x0_dn8) * 0.3333333333333333))))))) / (assign42180_e55437 * assign42180_e55437))),)
    } else {
        (var_sp_s_delta0, var_sp_s_delta0_dn5, var_sp_s_delta0_dn6, var_sp_s_delta0_dn7, var_sp_s_delta0_dn8,)
    }
};
        var_sp_s_delta0 = assign42180_e55440;
        var_sp_s_delta0_dn5 = assign42180_e55440_d_n5;
        var_sp_s_delta0_dn6 = assign42180_e55440_d_n6;
        var_sp_s_delta0_dn7 = assign42180_e55440_d_n7;
        var_sp_s_delta0_dn8 = assign42180_e55440_d_n8;
        var_sp_s_delta0_rv = 0.0;

        let (assign42190_e55476, assign42190_e55476_d_n5, assign42190_e55476_d_n6, assign42190_e55476_d_n7, assign42190_e55476_d_n8,) = {
    if ((((var_guard1182 == 0.0) && (var_guard1183 == 0.0)) && (var_guard1186 == 0.0)) && (var_guard1187 == 0.0)) {
        let assign42190_e55456: f64 = (var_sp_s_x0 - 230.25850929940458);
        let assign42190_e55461: f64 = (var_sp_s_x0 - 230.25850929940458);
        let assign42190_e55465: f64 = (var_sp_s_x0 - 230.25850929940458);
        let assign42190_e55467: f64 = (assign42190_e55465 * 0.3333333333333333);
        let assign42190_e55468: f64 = (1.0 + assign42190_e55467);
        let assign42190_e55469: f64 = (assign42190_e55461 * assign42190_e55468);
        let assign42190_e55470: f64 = (0.5 * assign42190_e55469);
        let assign42190_e55471: f64 = (1.0 + assign42190_e55470);
        let assign42190_e55472: f64 = (assign42190_e55456 * assign42190_e55471);
        let assign42190_e55473: f64 = (1.0 + assign42190_e55472);
        let assign42190_e55474: f64 = (1e-100 / assign42190_e55473);
        (assign42190_e55474, (-((1e-100 * ((var_sp_s_x0_dn5 * assign42190_e55471) + (assign42190_e55456 * (0.5 * ((var_sp_s_x0_dn5 * assign42190_e55468) + (assign42190_e55461 * (var_sp_s_x0_dn5 * 0.3333333333333333))))))) / (assign42190_e55473 * assign42190_e55473))), (-((1e-100 * ((var_sp_s_x0_dn6 * assign42190_e55471) + (assign42190_e55456 * (0.5 * ((var_sp_s_x0_dn6 * assign42190_e55468) + (assign42190_e55461 * (var_sp_s_x0_dn6 * 0.3333333333333333))))))) / (assign42190_e55473 * assign42190_e55473))), (-((1e-100 * ((var_sp_s_x0_dn7 * assign42190_e55471) + (assign42190_e55456 * (0.5 * ((var_sp_s_x0_dn7 * assign42190_e55468) + (assign42190_e55461 * (var_sp_s_x0_dn7 * 0.3333333333333333))))))) / (assign42190_e55473 * assign42190_e55473))), (-((1e-100 * ((var_sp_s_x0_dn8 * assign42190_e55471) + (assign42190_e55456 * (0.5 * ((var_sp_s_x0_dn8 * assign42190_e55468) + (assign42190_e55461 * (var_sp_s_x0_dn8 * 0.3333333333333333))))))) / (assign42190_e55473 * assign42190_e55473))),)
    } else {
        (var_sp_s_delta1, var_sp_s_delta1_dn5, var_sp_s_delta1_dn6, var_sp_s_delta1_dn7, var_sp_s_delta1_dn8,)
    }
};
        var_sp_s_delta1 = assign42190_e55476;
        var_sp_s_delta1_dn5 = assign42190_e55476_d_n5;
        var_sp_s_delta1_dn6 = assign42190_e55476_d_n6;
        var_sp_s_delta1_dn7 = assign42190_e55476_d_n7;
        var_sp_s_delta1_dn8 = assign42190_e55476_d_n8;
        var_sp_s_delta1_rv = 0.0;

        let (assign42200_e55490, assign42200_e55490_d_n5, assign42200_e55490_d_n6, assign42200_e55490_d_n7, assign42200_e55490_d_n8,) = {
    if ((var_guard1182 == 0.0) && (var_guard1183 == 0.0)) {
        let assign42200_e55486: f64 = (var_sp_s_x0 * var_sp_s_x0);
        let assign42200_e55487: f64 = (2.0 + assign42200_e55486);
        let assign42200_e55488: f64 = (1.0 / assign42200_e55487);
        (assign42200_e55488, (-(((var_sp_s_x0_dn5 * var_sp_s_x0) + (var_sp_s_x0 * var_sp_s_x0_dn5)) / (assign42200_e55487 * assign42200_e55487))), (-(((var_sp_s_x0_dn6 * var_sp_s_x0) + (var_sp_s_x0 * var_sp_s_x0_dn6)) / (assign42200_e55487 * assign42200_e55487))), (-(((var_sp_s_x0_dn7 * var_sp_s_x0) + (var_sp_s_x0 * var_sp_s_x0_dn7)) / (assign42200_e55487 * assign42200_e55487))), (-(((var_sp_s_x0_dn8 * var_sp_s_x0) + (var_sp_s_x0 * var_sp_s_x0_dn8)) / (assign42200_e55487 * assign42200_e55487))),)
    } else {
        (var_sp_s_temp, var_sp_s_temp_dn5, var_sp_s_temp_dn6, var_sp_s_temp_dn7, var_sp_s_temp_dn8,)
    }
};
        var_sp_s_temp = assign42200_e55490;
        var_sp_s_temp_dn5 = assign42200_e55490_d_n5;
        var_sp_s_temp_dn6 = assign42200_e55490_d_n6;
        var_sp_s_temp_dn7 = assign42200_e55490_d_n7;
        var_sp_s_temp_dn8 = assign42200_e55490_d_n8;
        var_sp_s_temp_rv = 0.0;

        let (assign42210_e55502, assign42210_e55502_d_n5, assign42210_e55502_d_n6, assign42210_e55502_d_n7, assign42210_e55502_d_n8,) = {
    if ((var_guard1182 == 0.0) && (var_guard1183 == 0.0)) {
        let assign42210_e55498: f64 = (var_sp_s_x0 * var_sp_s_x0);
        let assign42210_e55500: f64 = (assign42210_e55498 * var_sp_s_temp);
        (assign42210_e55500, ((((var_sp_s_x0_dn5 * var_sp_s_x0) + (var_sp_s_x0 * var_sp_s_x0_dn5)) * var_sp_s_temp) + (assign42210_e55498 * var_sp_s_temp_dn5)), ((((var_sp_s_x0_dn6 * var_sp_s_x0) + (var_sp_s_x0 * var_sp_s_x0_dn6)) * var_sp_s_temp) + (assign42210_e55498 * var_sp_s_temp_dn6)), ((((var_sp_s_x0_dn7 * var_sp_s_x0) + (var_sp_s_x0 * var_sp_s_x0_dn7)) * var_sp_s_temp) + (assign42210_e55498 * var_sp_s_temp_dn7)), ((((var_sp_s_x0_dn8 * var_sp_s_x0) + (var_sp_s_x0 * var_sp_s_x0_dn8)) * var_sp_s_temp) + (assign42210_e55498 * var_sp_s_temp_dn8)),)
    } else {
        (var_sp_s_xi0, var_sp_s_xi0_dn5, var_sp_s_xi0_dn6, var_sp_s_xi0_dn7, var_sp_s_xi0_dn8,)
    }
};
        var_sp_s_xi0 = assign42210_e55502;
        var_sp_s_xi0_dn5 = assign42210_e55502_d_n5;
        var_sp_s_xi0_dn6 = assign42210_e55502_d_n6;
        var_sp_s_xi0_dn7 = assign42210_e55502_d_n7;
        var_sp_s_xi0_dn8 = assign42210_e55502_d_n8;
        var_sp_s_xi0_rv = 0.0;

        *var_guard1186_slot = var_guard1186;
        *var_guard1186_rv_slot = var_guard1186_rv;
        *var_guard1187_slot = var_guard1187;
        *var_guard1187_rv_slot = var_guard1187_rv;
        *var_mutau_slot = var_mutau;
        *var_mutau_dn5_slot = var_mutau_dn5;
        *var_mutau_dn6_slot = var_mutau_dn6;
        *var_mutau_dn7_slot = var_mutau_dn7;
        *var_mutau_dn8_slot = var_mutau_dn8;
        *var_mutau_rv_slot = var_mutau_rv;
        *var_nu_slot = var_nu;
        *var_nu_dn5_slot = var_nu_dn5;
        *var_nu_dn6_slot = var_nu_dn6;
        *var_nu_dn7_slot = var_nu_dn7;
        *var_nu_dn8_slot = var_nu_dn8;
        *var_nu_rv_slot = var_nu_rv;
        *var_sp_s_a_slot = var_sp_s_a;
        *var_sp_s_a_dn5_slot = var_sp_s_a_dn5;
        *var_sp_s_a_dn6_slot = var_sp_s_a_dn6;
        *var_sp_s_a_dn7_slot = var_sp_s_a_dn7;
        *var_sp_s_a_dn8_slot = var_sp_s_a_dn8;
        *var_sp_s_a_rv_slot = var_sp_s_a_rv;
        *var_sp_s_b_slot = var_sp_s_b;
        *var_sp_s_b_dn5_slot = var_sp_s_b_dn5;
        *var_sp_s_b_dn6_slot = var_sp_s_b_dn6;
        *var_sp_s_b_dn7_slot = var_sp_s_b_dn7;
        *var_sp_s_b_dn8_slot = var_sp_s_b_dn8;
        *var_sp_s_b_rv_slot = var_sp_s_b_rv;
        *var_sp_s_bx_slot = var_sp_s_bx;
        *var_sp_s_bx_dn5_slot = var_sp_s_bx_dn5;
        *var_sp_s_bx_dn6_slot = var_sp_s_bx_dn6;
        *var_sp_s_bx_dn7_slot = var_sp_s_bx_dn7;
        *var_sp_s_bx_dn8_slot = var_sp_s_bx_dn8;
        *var_sp_s_bx_rv_slot = var_sp_s_bx_rv;
        *var_sp_s_c_slot = var_sp_s_c;
        *var_sp_s_c_dn5_slot = var_sp_s_c_dn5;
        *var_sp_s_c_dn6_slot = var_sp_s_c_dn6;
        *var_sp_s_c_dn7_slot = var_sp_s_c_dn7;
        *var_sp_s_c_dn8_slot = var_sp_s_c_dn8;
        *var_sp_s_c_rv_slot = var_sp_s_c_rv;
        *var_sp_s_delta0_slot = var_sp_s_delta0;
        *var_sp_s_delta0_dn5_slot = var_sp_s_delta0_dn5;
        *var_sp_s_delta0_dn6_slot = var_sp_s_delta0_dn6;
        *var_sp_s_delta0_dn7_slot = var_sp_s_delta0_dn7;
        *var_sp_s_delta0_dn8_slot = var_sp_s_delta0_dn8;
        *var_sp_s_delta0_rv_slot = var_sp_s_delta0_rv;
        *var_sp_s_delta1_slot = var_sp_s_delta1;
        *var_sp_s_delta1_dn5_slot = var_sp_s_delta1_dn5;
        *var_sp_s_delta1_dn6_slot = var_sp_s_delta1_dn6;
        *var_sp_s_delta1_dn7_slot = var_sp_s_delta1_dn7;
        *var_sp_s_delta1_dn8_slot = var_sp_s_delta1_dn8;
        *var_sp_s_delta1_rv_slot = var_sp_s_delta1_rv;
        *var_sp_s_eta_slot = var_sp_s_eta;
        *var_sp_s_eta_dn5_slot = var_sp_s_eta_dn5;
        *var_sp_s_eta_dn6_slot = var_sp_s_eta_dn6;
        *var_sp_s_eta_dn7_slot = var_sp_s_eta_dn7;
        *var_sp_s_eta_dn8_slot = var_sp_s_eta_dn8;
        *var_sp_s_eta_rv_slot = var_sp_s_eta_rv;
        *var_sp_s_tau_slot = var_sp_s_tau;
        *var_sp_s_tau_dn5_slot = var_sp_s_tau_dn5;
        *var_sp_s_tau_dn6_slot = var_sp_s_tau_dn6;
        *var_sp_s_tau_dn7_slot = var_sp_s_tau_dn7;
        *var_sp_s_tau_dn8_slot = var_sp_s_tau_dn8;
        *var_sp_s_tau_rv_slot = var_sp_s_tau_rv;
        *var_sp_s_temp_slot = var_sp_s_temp;
        *var_sp_s_temp1_slot = var_sp_s_temp1;
        *var_sp_s_temp1_dn5_slot = var_sp_s_temp1_dn5;
        *var_sp_s_temp1_dn6_slot = var_sp_s_temp1_dn6;
        *var_sp_s_temp1_dn7_slot = var_sp_s_temp1_dn7;
        *var_sp_s_temp1_dn8_slot = var_sp_s_temp1_dn8;
        *var_sp_s_temp1_rv_slot = var_sp_s_temp1_rv;
        *var_sp_s_temp2_slot = var_sp_s_temp2;
        *var_sp_s_temp2_dn5_slot = var_sp_s_temp2_dn5;
        *var_sp_s_temp2_dn6_slot = var_sp_s_temp2_dn6;
        *var_sp_s_temp2_dn7_slot = var_sp_s_temp2_dn7;
        *var_sp_s_temp2_dn8_slot = var_sp_s_temp2_dn8;
        *var_sp_s_temp2_rv_slot = var_sp_s_temp2_rv;
        *var_sp_s_temp_dn5_slot = var_sp_s_temp_dn5;
        *var_sp_s_temp_dn6_slot = var_sp_s_temp_dn6;
        *var_sp_s_temp_dn7_slot = var_sp_s_temp_dn7;
        *var_sp_s_temp_dn8_slot = var_sp_s_temp_dn8;
        *var_sp_s_temp_rv_slot = var_sp_s_temp_rv;
        *var_sp_s_x0_slot = var_sp_s_x0;
        *var_sp_s_x0_dn5_slot = var_sp_s_x0_dn5;
        *var_sp_s_x0_dn6_slot = var_sp_s_x0_dn6;
        *var_sp_s_x0_dn7_slot = var_sp_s_x0_dn7;
        *var_sp_s_x0_dn8_slot = var_sp_s_x0_dn8;
        *var_sp_s_x0_rv_slot = var_sp_s_x0_rv;
        *var_sp_s_x1_slot = var_sp_s_x1;
        *var_sp_s_x1_dn5_slot = var_sp_s_x1_dn5;
        *var_sp_s_x1_dn6_slot = var_sp_s_x1_dn6;
        *var_sp_s_x1_dn7_slot = var_sp_s_x1_dn7;
        *var_sp_s_x1_dn8_slot = var_sp_s_x1_dn8;
        *var_sp_s_x1_rv_slot = var_sp_s_x1_rv;
        *var_sp_s_xi0_slot = var_sp_s_xi0;
        *var_sp_s_xi0_dn5_slot = var_sp_s_xi0_dn5;
        *var_sp_s_xi0_dn6_slot = var_sp_s_xi0_dn6;
        *var_sp_s_xi0_dn7_slot = var_sp_s_xi0_dn7;
        *var_sp_s_xi0_dn8_slot = var_sp_s_xi0_dn8;
        *var_sp_s_xi0_rv_slot = var_sp_s_xi0_rv;
        *var_sp_s_xi1_slot = var_sp_s_xi1;
        *var_sp_s_xi1_dn5_slot = var_sp_s_xi1_dn5;
        *var_sp_s_xi1_dn6_slot = var_sp_s_xi1_dn6;
        *var_sp_s_xi1_dn7_slot = var_sp_s_xi1_dn7;
        *var_sp_s_xi1_dn8_slot = var_sp_s_xi1_dn8;
        *var_sp_s_xi1_rv_slot = var_sp_s_xi1_rv;
        *var_sp_s_xi2_slot = var_sp_s_xi2;
        *var_sp_s_xi2_dn5_slot = var_sp_s_xi2_dn5;
        *var_sp_s_xi2_dn6_slot = var_sp_s_xi2_dn6;
        *var_sp_s_xi2_dn7_slot = var_sp_s_xi2_dn7;
        *var_sp_s_xi2_dn8_slot = var_sp_s_xi2_dn8;
        *var_sp_s_xi2_rv_slot = var_sp_s_xi2_rv;
    }

    pub(super) fn stamp_reactive_block_28(
        var_delta_ns: f64,
        var_delta_ns_dn5: f64,
        var_delta_ns_dn6: f64,
        var_delta_ns_dn7: f64,
        var_delta_ns_dn8: f64,
        var_gf2: f64,
        var_gf2_dn5: f64,
        var_gf2_dn6: f64,
        var_gf2_dn7: f64,
        var_gf2_dn8: f64,
        var_guard1182: f64,
        var_guard1183: f64,
        var_phit1: f64,
        var_phit1_dn5: f64,
        var_phit1_dn6: f64,
        var_phit1_dn7: f64,
        var_phit1_dn8: f64,
        var_sp_s_delta0: f64,
        var_sp_s_delta0_dn5: f64,
        var_sp_s_delta0_dn6: f64,
        var_sp_s_delta0_dn7: f64,
        var_sp_s_delta0_dn8: f64,
        var_sp_s_delta1: f64,
        var_sp_s_delta1_dn5: f64,
        var_sp_s_delta1_dn6: f64,
        var_sp_s_delta1_dn7: f64,
        var_sp_s_delta1_dn8: f64,
        var_sp_s_x0: f64,
        var_sp_s_x0_dn5: f64,
        var_sp_s_x0_dn6: f64,
        var_sp_s_x0_dn7: f64,
        var_sp_s_x0_dn8: f64,
        var_sp_s_xi0: f64,
        var_sp_s_xi0_dn5: f64,
        var_sp_s_xi0_dn6: f64,
        var_sp_s_xi0_dn7: f64,
        var_sp_s_xi0_dn8: f64,
        var_xg: f64,
        var_xg_dn5: f64,
        var_xg_dn6: f64,
        var_xg_dn7: f64,
        var_xg_dn8: f64,
        var_xn_s: f64,
        var_xn_s_dn5: f64,
        var_xn_s_dn6: f64,
        var_xn_s_dn7: f64,
        var_xn_s_dn8: f64,
        var_alphas_slot: &mut f64,
        var_alphas_dn5_slot: &mut f64,
        var_alphas_dn6_slot: &mut f64,
        var_alphas_dn7_slot: &mut f64,
        var_alphas_dn8_slot: &mut f64,
        var_alphas_rv_slot: &mut f64,
        var_delta_1s_slot: &mut f64,
        var_delta_1s_dn5_slot: &mut f64,
        var_delta_1s_dn6_slot: &mut f64,
        var_delta_1s_dn7_slot: &mut f64,
        var_delta_1s_dn8_slot: &mut f64,
        var_delta_1s_rv_slot: &mut f64,
        var_ds_slot: &mut f64,
        var_ds_dn5_slot: &mut f64,
        var_ds_dn6_slot: &mut f64,
        var_ds_dn7_slot: &mut f64,
        var_ds_dn8_slot: &mut f64,
        var_ds_rv_slot: &mut f64,
        var_es_slot: &mut f64,
        var_es_dn5_slot: &mut f64,
        var_es_dn6_slot: &mut f64,
        var_es_dn7_slot: &mut f64,
        var_es_dn8_slot: &mut f64,
        var_es_rv_slot: &mut f64,
        var_factheta_slot: &mut f64,
        var_factheta_dn5_slot: &mut f64,
        var_factheta_dn6_slot: &mut f64,
        var_factheta_dn7_slot: &mut f64,
        var_factheta_dn8_slot: &mut f64,
        var_factheta_rv_slot: &mut f64,
        var_gmobs_slot: &mut f64,
        var_gmobs_dn5_slot: &mut f64,
        var_gmobs_dn6_slot: &mut f64,
        var_gmobs_dn7_slot: &mut f64,
        var_gmobs_dn8_slot: &mut f64,
        var_gmobs_rv_slot: &mut f64,
        var_guard1188_slot: &mut f64,
        var_guard1188_rv_slot: &mut f64,
        var_guard1189_slot: &mut f64,
        var_guard1189_rv_slot: &mut f64,
        var_guard1190_slot: &mut f64,
        var_guard1190_rv_slot: &mut f64,
        var_ps_slot: &mut f64,
        var_ps_dn5_slot: &mut f64,
        var_ps_dn6_slot: &mut f64,
        var_ps_dn7_slot: &mut f64,
        var_ps_dn8_slot: &mut f64,
        var_ps_rv_slot: &mut f64,
        var_qbs_slot: &mut f64,
        var_qbs_dn5_slot: &mut f64,
        var_qbs_dn6_slot: &mut f64,
        var_qbs_dn7_slot: &mut f64,
        var_qbs_dn8_slot: &mut f64,
        var_qbs_rv_slot: &mut f64,
        var_qis_slot: &mut f64,
        var_qis_dn5_slot: &mut f64,
        var_qis_dn6_slot: &mut f64,
        var_qis_dn7_slot: &mut f64,
        var_qis_dn8_slot: &mut f64,
        var_qis_rv_slot: &mut f64,
        var_rhob_slot: &mut f64,
        var_rhob_dn5_slot: &mut f64,
        var_rhob_dn6_slot: &mut f64,
        var_rhob_dn7_slot: &mut f64,
        var_rhob_dn8_slot: &mut f64,
        var_rhob_rv_slot: &mut f64,
        var_rhog_slot: &mut f64,
        var_rhog_dn5_slot: &mut f64,
        var_rhog_dn6_slot: &mut f64,
        var_rhog_dn7_slot: &mut f64,
        var_rhog_dn8_slot: &mut f64,
        var_rhog_rv_slot: &mut f64,
        var_rxcor_slot: &mut f64,
        var_rxcor_dn5_slot: &mut f64,
        var_rxcor_dn6_slot: &mut f64,
        var_rxcor_dn7_slot: &mut f64,
        var_rxcor_dn8_slot: &mut f64,
        var_rxcor_rv_slot: &mut f64,
        var_sp_s_pc_slot: &mut f64,
        var_sp_s_pc_dn5_slot: &mut f64,
        var_sp_s_pc_dn6_slot: &mut f64,
        var_sp_s_pc_dn7_slot: &mut f64,
        var_sp_s_pc_dn8_slot: &mut f64,
        var_sp_s_pc_rv_slot: &mut f64,
        var_sp_s_qc_slot: &mut f64,
        var_sp_s_qc_dn5_slot: &mut f64,
        var_sp_s_qc_dn6_slot: &mut f64,
        var_sp_s_qc_dn7_slot: &mut f64,
        var_sp_s_qc_dn8_slot: &mut f64,
        var_sp_s_qc_rv_slot: &mut f64,
        var_sp_s_temp_slot: &mut f64,
        var_sp_s_temp_dn5_slot: &mut f64,
        var_sp_s_temp_dn6_slot: &mut f64,
        var_sp_s_temp_dn7_slot: &mut f64,
        var_sp_s_temp_dn8_slot: &mut f64,
        var_sp_s_temp_rv_slot: &mut f64,
        var_sp_s_xi1_slot: &mut f64,
        var_sp_s_xi1_dn5_slot: &mut f64,
        var_sp_s_xi1_dn6_slot: &mut f64,
        var_sp_s_xi1_dn7_slot: &mut f64,
        var_sp_s_xi1_dn8_slot: &mut f64,
        var_sp_s_xi1_rv_slot: &mut f64,
        var_sp_s_xi2_slot: &mut f64,
        var_sp_s_xi2_dn5_slot: &mut f64,
        var_sp_s_xi2_dn6_slot: &mut f64,
        var_sp_s_xi2_dn7_slot: &mut f64,
        var_sp_s_xi2_dn8_slot: &mut f64,
        var_sp_s_xi2_rv_slot: &mut f64,
        var_sqs_slot: &mut f64,
        var_sqs_dn5_slot: &mut f64,
        var_sqs_dn6_slot: &mut f64,
        var_sqs_dn7_slot: &mut f64,
        var_sqs_dn8_slot: &mut f64,
        var_sqs_rv_slot: &mut f64,
        var_temp__blk936_slot: &mut f64,
        var_temp__blk936_dn5_slot: &mut f64,
        var_temp__blk936_dn6_slot: &mut f64,
        var_temp__blk936_dn7_slot: &mut f64,
        var_temp__blk936_dn8_slot: &mut f64,
        var_temp__blk936_rv_slot: &mut f64,
        var_x_s_slot: &mut f64,
        var_x_s_dn5_slot: &mut f64,
        var_x_s_dn6_slot: &mut f64,
        var_x_s_dn7_slot: &mut f64,
        var_x_s_dn8_slot: &mut f64,
        var_x_s_rv_slot: &mut f64,
        var_xgs_slot: &mut f64,
        var_xgs_dn5_slot: &mut f64,
        var_xgs_dn6_slot: &mut f64,
        var_xgs_dn7_slot: &mut f64,
        var_xgs_dn8_slot: &mut f64,
        var_xgs_rv_slot: &mut f64,
        var_xi0s_slot: &mut f64,
        var_xi0s_dn5_slot: &mut f64,
        var_xi0s_dn6_slot: &mut f64,
        var_xi0s_dn7_slot: &mut f64,
        var_xi0s_dn8_slot: &mut f64,
        var_xi0s_rv_slot: &mut f64,
        var_xi1s_slot: &mut f64,
        var_xi1s_dn5_slot: &mut f64,
        var_xi1s_dn6_slot: &mut f64,
        var_xi1s_dn7_slot: &mut f64,
        var_xi1s_dn8_slot: &mut f64,
        var_xi1s_rv_slot: &mut f64,
        var_xi2s_slot: &mut f64,
        var_xi2s_dn5_slot: &mut f64,
        var_xi2s_dn6_slot: &mut f64,
        var_xi2s_dn7_slot: &mut f64,
        var_xi2s_dn8_slot: &mut f64,
        var_xi2s_rv_slot: &mut f64,
        var_xitsb_slot: &mut f64,
        var_xitsb_dn5_slot: &mut f64,
        var_xitsb_dn6_slot: &mut f64,
        var_xitsb_dn7_slot: &mut f64,
        var_xitsb_dn8_slot: &mut f64,
        var_xitsb_rv_slot: &mut f64,
    ) {
        let mut var_alphas: f64 = *var_alphas_slot;
        let mut var_alphas_dn5: f64 = *var_alphas_dn5_slot;
        let mut var_alphas_dn6: f64 = *var_alphas_dn6_slot;
        let mut var_alphas_dn7: f64 = *var_alphas_dn7_slot;
        let mut var_alphas_dn8: f64 = *var_alphas_dn8_slot;
        let mut var_alphas_rv: f64 = *var_alphas_rv_slot;
        let mut var_delta_1s: f64 = *var_delta_1s_slot;
        let mut var_delta_1s_dn5: f64 = *var_delta_1s_dn5_slot;
        let mut var_delta_1s_dn6: f64 = *var_delta_1s_dn6_slot;
        let mut var_delta_1s_dn7: f64 = *var_delta_1s_dn7_slot;
        let mut var_delta_1s_dn8: f64 = *var_delta_1s_dn8_slot;
        let mut var_delta_1s_rv: f64 = *var_delta_1s_rv_slot;
        let mut var_ds: f64 = *var_ds_slot;
        let mut var_ds_dn5: f64 = *var_ds_dn5_slot;
        let mut var_ds_dn6: f64 = *var_ds_dn6_slot;
        let mut var_ds_dn7: f64 = *var_ds_dn7_slot;
        let mut var_ds_dn8: f64 = *var_ds_dn8_slot;
        let mut var_ds_rv: f64 = *var_ds_rv_slot;
        let mut var_es: f64 = *var_es_slot;
        let mut var_es_dn5: f64 = *var_es_dn5_slot;
        let mut var_es_dn6: f64 = *var_es_dn6_slot;
        let mut var_es_dn7: f64 = *var_es_dn7_slot;
        let mut var_es_dn8: f64 = *var_es_dn8_slot;
        let mut var_es_rv: f64 = *var_es_rv_slot;
        let mut var_factheta: f64 = *var_factheta_slot;
        let mut var_factheta_dn5: f64 = *var_factheta_dn5_slot;
        let mut var_factheta_dn6: f64 = *var_factheta_dn6_slot;
        let mut var_factheta_dn7: f64 = *var_factheta_dn7_slot;
        let mut var_factheta_dn8: f64 = *var_factheta_dn8_slot;
        let mut var_factheta_rv: f64 = *var_factheta_rv_slot;
        let mut var_gmobs: f64 = *var_gmobs_slot;
        let mut var_gmobs_dn5: f64 = *var_gmobs_dn5_slot;
        let mut var_gmobs_dn6: f64 = *var_gmobs_dn6_slot;
        let mut var_gmobs_dn7: f64 = *var_gmobs_dn7_slot;
        let mut var_gmobs_dn8: f64 = *var_gmobs_dn8_slot;
        let mut var_gmobs_rv: f64 = *var_gmobs_rv_slot;
        let mut var_guard1188: f64 = *var_guard1188_slot;
        let mut var_guard1188_rv: f64 = *var_guard1188_rv_slot;
        let mut var_guard1189: f64 = *var_guard1189_slot;
        let mut var_guard1189_rv: f64 = *var_guard1189_rv_slot;
        let mut var_guard1190: f64 = *var_guard1190_slot;
        let mut var_guard1190_rv: f64 = *var_guard1190_rv_slot;
        let mut var_ps: f64 = *var_ps_slot;
        let mut var_ps_dn5: f64 = *var_ps_dn5_slot;
        let mut var_ps_dn6: f64 = *var_ps_dn6_slot;
        let mut var_ps_dn7: f64 = *var_ps_dn7_slot;
        let mut var_ps_dn8: f64 = *var_ps_dn8_slot;
        let mut var_ps_rv: f64 = *var_ps_rv_slot;
        let mut var_qbs: f64 = *var_qbs_slot;
        let mut var_qbs_dn5: f64 = *var_qbs_dn5_slot;
        let mut var_qbs_dn6: f64 = *var_qbs_dn6_slot;
        let mut var_qbs_dn7: f64 = *var_qbs_dn7_slot;
        let mut var_qbs_dn8: f64 = *var_qbs_dn8_slot;
        let mut var_qbs_rv: f64 = *var_qbs_rv_slot;
        let mut var_qis: f64 = *var_qis_slot;
        let mut var_qis_dn5: f64 = *var_qis_dn5_slot;
        let mut var_qis_dn6: f64 = *var_qis_dn6_slot;
        let mut var_qis_dn7: f64 = *var_qis_dn7_slot;
        let mut var_qis_dn8: f64 = *var_qis_dn8_slot;
        let mut var_qis_rv: f64 = *var_qis_rv_slot;
        let mut var_rhob: f64 = *var_rhob_slot;
        let mut var_rhob_dn5: f64 = *var_rhob_dn5_slot;
        let mut var_rhob_dn6: f64 = *var_rhob_dn6_slot;
        let mut var_rhob_dn7: f64 = *var_rhob_dn7_slot;
        let mut var_rhob_dn8: f64 = *var_rhob_dn8_slot;
        let mut var_rhob_rv: f64 = *var_rhob_rv_slot;
        let mut var_rhog: f64 = *var_rhog_slot;
        let mut var_rhog_dn5: f64 = *var_rhog_dn5_slot;
        let mut var_rhog_dn6: f64 = *var_rhog_dn6_slot;
        let mut var_rhog_dn7: f64 = *var_rhog_dn7_slot;
        let mut var_rhog_dn8: f64 = *var_rhog_dn8_slot;
        let mut var_rhog_rv: f64 = *var_rhog_rv_slot;
        let mut var_rxcor: f64 = *var_rxcor_slot;
        let mut var_rxcor_dn5: f64 = *var_rxcor_dn5_slot;
        let mut var_rxcor_dn6: f64 = *var_rxcor_dn6_slot;
        let mut var_rxcor_dn7: f64 = *var_rxcor_dn7_slot;
        let mut var_rxcor_dn8: f64 = *var_rxcor_dn8_slot;
        let mut var_rxcor_rv: f64 = *var_rxcor_rv_slot;
        let mut var_sp_s_pc: f64 = *var_sp_s_pc_slot;
        let mut var_sp_s_pc_dn5: f64 = *var_sp_s_pc_dn5_slot;
        let mut var_sp_s_pc_dn6: f64 = *var_sp_s_pc_dn6_slot;
        let mut var_sp_s_pc_dn7: f64 = *var_sp_s_pc_dn7_slot;
        let mut var_sp_s_pc_dn8: f64 = *var_sp_s_pc_dn8_slot;
        let mut var_sp_s_pc_rv: f64 = *var_sp_s_pc_rv_slot;
        let mut var_sp_s_qc: f64 = *var_sp_s_qc_slot;
        let mut var_sp_s_qc_dn5: f64 = *var_sp_s_qc_dn5_slot;
        let mut var_sp_s_qc_dn6: f64 = *var_sp_s_qc_dn6_slot;
        let mut var_sp_s_qc_dn7: f64 = *var_sp_s_qc_dn7_slot;
        let mut var_sp_s_qc_dn8: f64 = *var_sp_s_qc_dn8_slot;
        let mut var_sp_s_qc_rv: f64 = *var_sp_s_qc_rv_slot;
        let mut var_sp_s_temp: f64 = *var_sp_s_temp_slot;
        let mut var_sp_s_temp_dn5: f64 = *var_sp_s_temp_dn5_slot;
        let mut var_sp_s_temp_dn6: f64 = *var_sp_s_temp_dn6_slot;
        let mut var_sp_s_temp_dn7: f64 = *var_sp_s_temp_dn7_slot;
        let mut var_sp_s_temp_dn8: f64 = *var_sp_s_temp_dn8_slot;
        let mut var_sp_s_temp_rv: f64 = *var_sp_s_temp_rv_slot;
        let mut var_sp_s_xi1: f64 = *var_sp_s_xi1_slot;
        let mut var_sp_s_xi1_dn5: f64 = *var_sp_s_xi1_dn5_slot;
        let mut var_sp_s_xi1_dn6: f64 = *var_sp_s_xi1_dn6_slot;
        let mut var_sp_s_xi1_dn7: f64 = *var_sp_s_xi1_dn7_slot;
        let mut var_sp_s_xi1_dn8: f64 = *var_sp_s_xi1_dn8_slot;
        let mut var_sp_s_xi1_rv: f64 = *var_sp_s_xi1_rv_slot;
        let mut var_sp_s_xi2: f64 = *var_sp_s_xi2_slot;
        let mut var_sp_s_xi2_dn5: f64 = *var_sp_s_xi2_dn5_slot;
        let mut var_sp_s_xi2_dn6: f64 = *var_sp_s_xi2_dn6_slot;
        let mut var_sp_s_xi2_dn7: f64 = *var_sp_s_xi2_dn7_slot;
        let mut var_sp_s_xi2_dn8: f64 = *var_sp_s_xi2_dn8_slot;
        let mut var_sp_s_xi2_rv: f64 = *var_sp_s_xi2_rv_slot;
        let mut var_sqs: f64 = *var_sqs_slot;
        let mut var_sqs_dn5: f64 = *var_sqs_dn5_slot;
        let mut var_sqs_dn6: f64 = *var_sqs_dn6_slot;
        let mut var_sqs_dn7: f64 = *var_sqs_dn7_slot;
        let mut var_sqs_dn8: f64 = *var_sqs_dn8_slot;
        let mut var_sqs_rv: f64 = *var_sqs_rv_slot;
        let mut var_temp__blk936: f64 = *var_temp__blk936_slot;
        let mut var_temp__blk936_dn5: f64 = *var_temp__blk936_dn5_slot;
        let mut var_temp__blk936_dn6: f64 = *var_temp__blk936_dn6_slot;
        let mut var_temp__blk936_dn7: f64 = *var_temp__blk936_dn7_slot;
        let mut var_temp__blk936_dn8: f64 = *var_temp__blk936_dn8_slot;
        let mut var_temp__blk936_rv: f64 = *var_temp__blk936_rv_slot;
        let mut var_x_s: f64 = *var_x_s_slot;
        let mut var_x_s_dn5: f64 = *var_x_s_dn5_slot;
        let mut var_x_s_dn6: f64 = *var_x_s_dn6_slot;
        let mut var_x_s_dn7: f64 = *var_x_s_dn7_slot;
        let mut var_x_s_dn8: f64 = *var_x_s_dn8_slot;
        let mut var_x_s_rv: f64 = *var_x_s_rv_slot;
        let mut var_xgs: f64 = *var_xgs_slot;
        let mut var_xgs_dn5: f64 = *var_xgs_dn5_slot;
        let mut var_xgs_dn6: f64 = *var_xgs_dn6_slot;
        let mut var_xgs_dn7: f64 = *var_xgs_dn7_slot;
        let mut var_xgs_dn8: f64 = *var_xgs_dn8_slot;
        let mut var_xgs_rv: f64 = *var_xgs_rv_slot;
        let mut var_xi0s: f64 = *var_xi0s_slot;
        let mut var_xi0s_dn5: f64 = *var_xi0s_dn5_slot;
        let mut var_xi0s_dn6: f64 = *var_xi0s_dn6_slot;
        let mut var_xi0s_dn7: f64 = *var_xi0s_dn7_slot;
        let mut var_xi0s_dn8: f64 = *var_xi0s_dn8_slot;
        let mut var_xi0s_rv: f64 = *var_xi0s_rv_slot;
        let mut var_xi1s: f64 = *var_xi1s_slot;
        let mut var_xi1s_dn5: f64 = *var_xi1s_dn5_slot;
        let mut var_xi1s_dn6: f64 = *var_xi1s_dn6_slot;
        let mut var_xi1s_dn7: f64 = *var_xi1s_dn7_slot;
        let mut var_xi1s_dn8: f64 = *var_xi1s_dn8_slot;
        let mut var_xi1s_rv: f64 = *var_xi1s_rv_slot;
        let mut var_xi2s: f64 = *var_xi2s_slot;
        let mut var_xi2s_dn5: f64 = *var_xi2s_dn5_slot;
        let mut var_xi2s_dn6: f64 = *var_xi2s_dn6_slot;
        let mut var_xi2s_dn7: f64 = *var_xi2s_dn7_slot;
        let mut var_xi2s_dn8: f64 = *var_xi2s_dn8_slot;
        let mut var_xi2s_rv: f64 = *var_xi2s_rv_slot;
        let mut var_xitsb: f64 = *var_xitsb_slot;
        let mut var_xitsb_dn5: f64 = *var_xitsb_dn5_slot;
        let mut var_xitsb_dn6: f64 = *var_xitsb_dn6_slot;
        let mut var_xitsb_dn7: f64 = *var_xitsb_dn7_slot;
        let mut var_xitsb_dn8: f64 = *var_xitsb_dn8_slot;
        let mut var_xitsb_rv: f64 = *var_xitsb_rv_slot;

        let (assign42220_e55516, assign42220_e55516_d_n5, assign42220_e55516_d_n6, assign42220_e55516_d_n7, assign42220_e55516_d_n8,) = {
    if ((var_guard1182 == 0.0) && (var_guard1183 == 0.0)) {
        let assign42220_e55511: f64 = (var_sp_s_x0 * var_sp_s_temp);
        let assign42220_e55513: f64 = (assign42220_e55511 * var_sp_s_temp);
        let assign42220_e55514: f64 = (4.0 * assign42220_e55513);
        (assign42220_e55514, (4.0 * ((((var_sp_s_x0_dn5 * var_sp_s_temp) + (var_sp_s_x0 * var_sp_s_temp_dn5)) * var_sp_s_temp) + (assign42220_e55511 * var_sp_s_temp_dn5))), (4.0 * ((((var_sp_s_x0_dn6 * var_sp_s_temp) + (var_sp_s_x0 * var_sp_s_temp_dn6)) * var_sp_s_temp) + (assign42220_e55511 * var_sp_s_temp_dn6))), (4.0 * ((((var_sp_s_x0_dn7 * var_sp_s_temp) + (var_sp_s_x0 * var_sp_s_temp_dn7)) * var_sp_s_temp) + (assign42220_e55511 * var_sp_s_temp_dn7))), (4.0 * ((((var_sp_s_x0_dn8 * var_sp_s_temp) + (var_sp_s_x0 * var_sp_s_temp_dn8)) * var_sp_s_temp) + (assign42220_e55511 * var_sp_s_temp_dn8))),)
    } else {
        (var_sp_s_xi1, var_sp_s_xi1_dn5, var_sp_s_xi1_dn6, var_sp_s_xi1_dn7, var_sp_s_xi1_dn8,)
    }
};
        var_sp_s_xi1 = assign42220_e55516;
        var_sp_s_xi1_dn5 = assign42220_e55516_d_n5;
        var_sp_s_xi1_dn6 = assign42220_e55516_d_n6;
        var_sp_s_xi1_dn7 = assign42220_e55516_d_n7;
        var_sp_s_xi1_dn8 = assign42220_e55516_d_n8;
        var_sp_s_xi1_rv = 0.0;

        let (assign42230_e55534, assign42230_e55534_d_n5, assign42230_e55534_d_n6, assign42230_e55534_d_n7, assign42230_e55534_d_n8,) = {
    if ((var_guard1182 == 0.0) && (var_guard1183 == 0.0)) {
        let assign42230_e55524: f64 = (8.0 * var_sp_s_temp);
        let assign42230_e55527: f64 = (12.0 * var_sp_s_xi0);
        let assign42230_e55528: f64 = (assign42230_e55524 - assign42230_e55527);
        let assign42230_e55530: f64 = (assign42230_e55528 * var_sp_s_temp);
        let assign42230_e55532: f64 = (assign42230_e55530 * var_sp_s_temp);
        (assign42230_e55532, ((((((8.0 * var_sp_s_temp_dn5) - (12.0 * var_sp_s_xi0_dn5)) * var_sp_s_temp) + (assign42230_e55528 * var_sp_s_temp_dn5)) * var_sp_s_temp) + (assign42230_e55530 * var_sp_s_temp_dn5)), ((((((8.0 * var_sp_s_temp_dn6) - (12.0 * var_sp_s_xi0_dn6)) * var_sp_s_temp) + (assign42230_e55528 * var_sp_s_temp_dn6)) * var_sp_s_temp) + (assign42230_e55530 * var_sp_s_temp_dn6)), ((((((8.0 * var_sp_s_temp_dn7) - (12.0 * var_sp_s_xi0_dn7)) * var_sp_s_temp) + (assign42230_e55528 * var_sp_s_temp_dn7)) * var_sp_s_temp) + (assign42230_e55530 * var_sp_s_temp_dn7)), ((((((8.0 * var_sp_s_temp_dn8) - (12.0 * var_sp_s_xi0_dn8)) * var_sp_s_temp) + (assign42230_e55528 * var_sp_s_temp_dn8)) * var_sp_s_temp) + (assign42230_e55530 * var_sp_s_temp_dn8)),)
    } else {
        (var_sp_s_xi2, var_sp_s_xi2_dn5, var_sp_s_xi2_dn6, var_sp_s_xi2_dn7, var_sp_s_xi2_dn8,)
    }
};
        var_sp_s_xi2 = assign42230_e55534;
        var_sp_s_xi2_dn5 = assign42230_e55534_d_n5;
        var_sp_s_xi2_dn6 = assign42230_e55534_d_n6;
        var_sp_s_xi2_dn7 = assign42230_e55534_d_n7;
        var_sp_s_xi2_dn8 = assign42230_e55534_d_n8;
        var_sp_s_xi2_rv = 0.0;

        let (assign42240_e55544, assign42240_e55544_d_n5, assign42240_e55544_d_n6, assign42240_e55544_d_n7, assign42240_e55544_d_n8,) = {
    if ((var_guard1182 == 0.0) && (var_guard1183 == 0.0)) {
        let assign42240_e55542: f64 = (var_xg - var_sp_s_x0);
        (assign42240_e55542, (var_xg_dn5 - var_sp_s_x0_dn5), (var_xg_dn6 - var_sp_s_x0_dn6), (var_xg_dn7 - var_sp_s_x0_dn7), (var_xg_dn8 - var_sp_s_x0_dn8),)
    } else {
        (var_sp_s_temp, var_sp_s_temp_dn5, var_sp_s_temp_dn6, var_sp_s_temp_dn7, var_sp_s_temp_dn8,)
    }
};
        var_sp_s_temp = assign42240_e55544;
        var_sp_s_temp_dn5 = assign42240_e55544_d_n5;
        var_sp_s_temp_dn6 = assign42240_e55544_d_n6;
        var_sp_s_temp_dn7 = assign42240_e55544_d_n7;
        var_sp_s_temp_dn8 = assign42240_e55544_d_n8;
        var_sp_s_temp_rv = 0.0;

        let (assign42250_e55568, assign42250_e55568_d_n5, assign42250_e55568_d_n6, assign42250_e55568_d_n7, assign42250_e55568_d_n8,) = {
    if ((var_guard1182 == 0.0) && (var_guard1183 == 0.0)) {
        let assign42250_e55552: f64 = (2.0 * var_sp_s_temp);
        let assign42250_e55556: f64 = (1.0 - var_sp_s_delta1);
        let assign42250_e55558: f64 = (assign42250_e55556 + var_sp_s_delta0);
        let assign42250_e55562: f64 = (1.0 + var_sp_s_xi1);
        let assign42250_e55563: f64 = (var_delta_ns * assign42250_e55562);
        let assign42250_e55564: f64 = (assign42250_e55558 - assign42250_e55563);
        let assign42250_e55565: f64 = (var_gf2 * assign42250_e55564);
        let assign42250_e55566: f64 = (assign42250_e55552 + assign42250_e55565);
        (assign42250_e55566, ((2.0 * var_sp_s_temp_dn5) + ((var_gf2_dn5 * assign42250_e55564) + (var_gf2 * (((-var_sp_s_delta1_dn5) + var_sp_s_delta0_dn5) - ((var_delta_ns_dn5 * assign42250_e55562) + (var_delta_ns * var_sp_s_xi1_dn5)))))), ((2.0 * var_sp_s_temp_dn6) + ((var_gf2_dn6 * assign42250_e55564) + (var_gf2 * (((-var_sp_s_delta1_dn6) + var_sp_s_delta0_dn6) - ((var_delta_ns_dn6 * assign42250_e55562) + (var_delta_ns * var_sp_s_xi1_dn6)))))), ((2.0 * var_sp_s_temp_dn7) + ((var_gf2_dn7 * assign42250_e55564) + (var_gf2 * (((-var_sp_s_delta1_dn7) + var_sp_s_delta0_dn7) - ((var_delta_ns_dn7 * assign42250_e55562) + (var_delta_ns * var_sp_s_xi1_dn7)))))), ((2.0 * var_sp_s_temp_dn8) + ((var_gf2_dn8 * assign42250_e55564) + (var_gf2 * (((-var_sp_s_delta1_dn8) + var_sp_s_delta0_dn8) - ((var_delta_ns_dn8 * assign42250_e55562) + (var_delta_ns * var_sp_s_xi1_dn8)))))),)
    } else {
        (var_sp_s_pc, var_sp_s_pc_dn5, var_sp_s_pc_dn6, var_sp_s_pc_dn7, var_sp_s_pc_dn8,)
    }
};
        var_sp_s_pc = assign42250_e55568;
        var_sp_s_pc_dn5 = assign42250_e55568_d_n5;
        var_sp_s_pc_dn6 = assign42250_e55568_d_n6;
        var_sp_s_pc_dn7 = assign42250_e55568_d_n7;
        var_sp_s_pc_dn8 = assign42250_e55568_d_n8;
        var_sp_s_pc_rv = 0.0;

        let (assign42260_e55596, assign42260_e55596_d_n5, assign42260_e55596_d_n6, assign42260_e55596_d_n7, assign42260_e55596_d_n8,) = {
    if ((var_guard1182 == 0.0) && (var_guard1183 == 0.0)) {
        let assign42260_e55576: f64 = (var_sp_s_temp * var_sp_s_temp);
        let assign42260_e55580: f64 = (var_sp_s_delta1 + var_sp_s_x0);
        let assign42260_e55582: f64 = (assign42260_e55580 - 1.0);
        let assign42260_e55584: f64 = (assign42260_e55582 + var_sp_s_delta0);
        let assign42260_e55588: f64 = (var_sp_s_x0 + 1.0);
        let assign42260_e55590: f64 = (assign42260_e55588 + var_sp_s_xi0);
        let assign42260_e55591: f64 = (var_delta_ns * assign42260_e55590);
        let assign42260_e55592: f64 = (assign42260_e55584 - assign42260_e55591);
        let assign42260_e55593: f64 = (var_gf2 * assign42260_e55592);
        let assign42260_e55594: f64 = (assign42260_e55576 - assign42260_e55593);
        (assign42260_e55594, (((var_sp_s_temp_dn5 * var_sp_s_temp) + (var_sp_s_temp * var_sp_s_temp_dn5)) - ((var_gf2_dn5 * assign42260_e55592) + (var_gf2 * (((var_sp_s_delta1_dn5 + var_sp_s_x0_dn5) + var_sp_s_delta0_dn5) - ((var_delta_ns_dn5 * assign42260_e55590) + (var_delta_ns * (var_sp_s_x0_dn5 + var_sp_s_xi0_dn5))))))), (((var_sp_s_temp_dn6 * var_sp_s_temp) + (var_sp_s_temp * var_sp_s_temp_dn6)) - ((var_gf2_dn6 * assign42260_e55592) + (var_gf2 * (((var_sp_s_delta1_dn6 + var_sp_s_x0_dn6) + var_sp_s_delta0_dn6) - ((var_delta_ns_dn6 * assign42260_e55590) + (var_delta_ns * (var_sp_s_x0_dn6 + var_sp_s_xi0_dn6))))))), (((var_sp_s_temp_dn7 * var_sp_s_temp) + (var_sp_s_temp * var_sp_s_temp_dn7)) - ((var_gf2_dn7 * assign42260_e55592) + (var_gf2 * (((var_sp_s_delta1_dn7 + var_sp_s_x0_dn7) + var_sp_s_delta0_dn7) - ((var_delta_ns_dn7 * assign42260_e55590) + (var_delta_ns * (var_sp_s_x0_dn7 + var_sp_s_xi0_dn7))))))), (((var_sp_s_temp_dn8 * var_sp_s_temp) + (var_sp_s_temp * var_sp_s_temp_dn8)) - ((var_gf2_dn8 * assign42260_e55592) + (var_gf2 * (((var_sp_s_delta1_dn8 + var_sp_s_x0_dn8) + var_sp_s_delta0_dn8) - ((var_delta_ns_dn8 * assign42260_e55590) + (var_delta_ns * (var_sp_s_x0_dn8 + var_sp_s_xi0_dn8))))))),)
    } else {
        (var_sp_s_qc, var_sp_s_qc_dn5, var_sp_s_qc_dn6, var_sp_s_qc_dn7, var_sp_s_qc_dn8,)
    }
};
        var_sp_s_qc = assign42260_e55596;
        var_sp_s_qc_dn5 = assign42260_e55596_d_n5;
        var_sp_s_qc_dn6 = assign42260_e55596_d_n6;
        var_sp_s_qc_dn7 = assign42260_e55596_d_n7;
        var_sp_s_qc_dn8 = assign42260_e55596_d_n8;
        var_sp_s_qc_rv = 0.0;

        let (assign42270_e55614, assign42270_e55614_d_n5, assign42270_e55614_d_n6, assign42270_e55614_d_n7, assign42270_e55614_d_n8,) = {
    if ((var_guard1182 == 0.0) && (var_guard1183 == 0.0)) {
        let assign42270_e55606: f64 = (var_sp_s_delta1 + var_sp_s_delta0);
        let assign42270_e55609: f64 = (var_delta_ns * var_sp_s_xi2);
        let assign42270_e55610: f64 = (assign42270_e55606 - assign42270_e55609);
        let assign42270_e55611: f64 = (var_gf2 * assign42270_e55610);
        let assign42270_e55612: f64 = (2.0 - assign42270_e55611);
        (assign42270_e55612, (-((var_gf2_dn5 * assign42270_e55610) + (var_gf2 * ((var_sp_s_delta1_dn5 + var_sp_s_delta0_dn5) - ((var_delta_ns_dn5 * var_sp_s_xi2) + (var_delta_ns * var_sp_s_xi2_dn5)))))), (-((var_gf2_dn6 * assign42270_e55610) + (var_gf2 * ((var_sp_s_delta1_dn6 + var_sp_s_delta0_dn6) - ((var_delta_ns_dn6 * var_sp_s_xi2) + (var_delta_ns * var_sp_s_xi2_dn6)))))), (-((var_gf2_dn7 * assign42270_e55610) + (var_gf2 * ((var_sp_s_delta1_dn7 + var_sp_s_delta0_dn7) - ((var_delta_ns_dn7 * var_sp_s_xi2) + (var_delta_ns * var_sp_s_xi2_dn7)))))), (-((var_gf2_dn8 * assign42270_e55610) + (var_gf2 * ((var_sp_s_delta1_dn8 + var_sp_s_delta0_dn8) - ((var_delta_ns_dn8 * var_sp_s_xi2) + (var_delta_ns * var_sp_s_xi2_dn8)))))),)
    } else {
        (var_sp_s_temp, var_sp_s_temp_dn5, var_sp_s_temp_dn6, var_sp_s_temp_dn7, var_sp_s_temp_dn8,)
    }
};
        var_sp_s_temp = assign42270_e55614;
        var_sp_s_temp_dn5 = assign42270_e55614_d_n5;
        var_sp_s_temp_dn6 = assign42270_e55614_d_n6;
        var_sp_s_temp_dn7 = assign42270_e55614_d_n7;
        var_sp_s_temp_dn8 = assign42270_e55614_d_n8;
        var_sp_s_temp_rv = 0.0;

        let (assign42280_e55630, assign42280_e55630_d_n5, assign42280_e55630_d_n6, assign42280_e55630_d_n7, assign42280_e55630_d_n8,) = {
    if ((var_guard1182 == 0.0) && (var_guard1183 == 0.0)) {
        let assign42280_e55622: f64 = (var_sp_s_pc * var_sp_s_pc);
        let assign42280_e55626: f64 = (var_sp_s_qc * var_sp_s_temp);
        let assign42280_e55627: f64 = (2.0 * assign42280_e55626);
        let assign42280_e55628: f64 = (assign42280_e55622 - assign42280_e55627);
        (assign42280_e55628, (((var_sp_s_pc_dn5 * var_sp_s_pc) + (var_sp_s_pc * var_sp_s_pc_dn5)) - (2.0 * ((var_sp_s_qc_dn5 * var_sp_s_temp) + (var_sp_s_qc * var_sp_s_temp_dn5)))), (((var_sp_s_pc_dn6 * var_sp_s_pc) + (var_sp_s_pc * var_sp_s_pc_dn6)) - (2.0 * ((var_sp_s_qc_dn6 * var_sp_s_temp) + (var_sp_s_qc * var_sp_s_temp_dn6)))), (((var_sp_s_pc_dn7 * var_sp_s_pc) + (var_sp_s_pc * var_sp_s_pc_dn7)) - (2.0 * ((var_sp_s_qc_dn7 * var_sp_s_temp) + (var_sp_s_qc * var_sp_s_temp_dn7)))), (((var_sp_s_pc_dn8 * var_sp_s_pc) + (var_sp_s_pc * var_sp_s_pc_dn8)) - (2.0 * ((var_sp_s_qc_dn8 * var_sp_s_temp) + (var_sp_s_qc * var_sp_s_temp_dn8)))),)
    } else {
        (var_sp_s_temp, var_sp_s_temp_dn5, var_sp_s_temp_dn6, var_sp_s_temp_dn7, var_sp_s_temp_dn8,)
    }
};
        var_sp_s_temp = assign42280_e55630;
        var_sp_s_temp_dn5 = assign42280_e55630_d_n5;
        var_sp_s_temp_dn6 = assign42280_e55630_d_n6;
        var_sp_s_temp_dn7 = assign42280_e55630_d_n7;
        var_sp_s_temp_dn8 = assign42280_e55630_d_n8;
        var_sp_s_temp_rv = 0.0;

        let (assign42290_e55647, assign42290_e55647_d_n5, assign42290_e55647_d_n6, assign42290_e55647_d_n7, assign42290_e55647_d_n8,) = {
    if ((var_guard1182 == 0.0) && (var_guard1183 == 0.0)) {
        let assign42290_e55641: f64 = (var_sp_s_temp).sqrt();
        let assign42290_e55642: f64 = (var_sp_s_pc + assign42290_e55641);
        let assign42290_e55643: f64 = (var_sp_s_qc / assign42290_e55642);
        let assign42290_e55644: f64 = (2.0 * assign42290_e55643);
        let assign42290_e55645: f64 = (var_sp_s_x0 + assign42290_e55644);
        (assign42290_e55645, (var_sp_s_x0_dn5 + (2.0 * (((var_sp_s_qc_dn5 * assign42290_e55642) - (var_sp_s_qc * (var_sp_s_pc_dn5 + (var_sp_s_temp_dn5 / (2.0 * assign42290_e55641))))) / (assign42290_e55642 * assign42290_e55642)))), (var_sp_s_x0_dn6 + (2.0 * (((var_sp_s_qc_dn6 * assign42290_e55642) - (var_sp_s_qc * (var_sp_s_pc_dn6 + (var_sp_s_temp_dn6 / (2.0 * assign42290_e55641))))) / (assign42290_e55642 * assign42290_e55642)))), (var_sp_s_x0_dn7 + (2.0 * (((var_sp_s_qc_dn7 * assign42290_e55642) - (var_sp_s_qc * (var_sp_s_pc_dn7 + (var_sp_s_temp_dn7 / (2.0 * assign42290_e55641))))) / (assign42290_e55642 * assign42290_e55642)))), (var_sp_s_x0_dn8 + (2.0 * (((var_sp_s_qc_dn8 * assign42290_e55642) - (var_sp_s_qc * (var_sp_s_pc_dn8 + (var_sp_s_temp_dn8 / (2.0 * assign42290_e55641))))) / (assign42290_e55642 * assign42290_e55642)))),)
    } else {
        (var_x_s, var_x_s_dn5, var_x_s_dn6, var_x_s_dn7, var_x_s_dn8,)
    }
};
        var_x_s = assign42290_e55647;
        var_x_s_dn5 = assign42290_e55647_d_n5;
        var_x_s_dn6 = assign42290_e55647_d_n6;
        var_x_s_dn7 = assign42290_e55647_d_n7;
        var_x_s_dn8 = assign42290_e55647_d_n8;
        var_x_s_rv = 0.0;

        var_xi1s = 0.0;
        var_xi1s_dn5 = 0.0;
        var_xi1s_dn6 = 0.0;
        var_xi1s_dn7 = 0.0;
        var_xi1s_dn8 = 0.0;
        var_xi1s_rv = 0.0;

        var_xi2s = 0.0;
        var_xi2s_dn5 = 0.0;
        var_xi2s_dn6 = 0.0;
        var_xi2s_dn7 = 0.0;
        var_xi2s_dn8 = 0.0;
        var_xi2s_rv = 0.0;

        var_delta_1s = 0.0;
        var_delta_1s_dn5 = 0.0;
        var_delta_1s_dn6 = 0.0;
        var_delta_1s_dn7 = 0.0;
        var_delta_1s_dn8 = 0.0;
        var_delta_1s_rv = 0.0;

        var_es = 0.0;
        var_es_dn5 = 0.0;
        var_es_dn6 = 0.0;
        var_es_dn7 = 0.0;
        var_es_dn8 = 0.0;
        var_es_rv = 0.0;

        var_ds = 0.0;
        var_ds_dn5 = 0.0;
        var_ds_dn6 = 0.0;
        var_ds_dn7 = 0.0;
        var_ds_dn8 = 0.0;
        var_ds_rv = 0.0;

        var_ps = 0.0;
        var_ps_dn5 = 0.0;
        var_ps_dn6 = 0.0;
        var_ps_dn7 = 0.0;
        var_ps_dn8 = 0.0;
        var_ps_rv = 0.0;

        var_sqs = 0.0;
        var_sqs_dn5 = 0.0;
        var_sqs_dn6 = 0.0;
        var_sqs_dn7 = 0.0;
        var_sqs_dn8 = 0.0;
        var_sqs_rv = 0.0;

        var_alphas = 1.0;
        var_alphas_dn5 = 0.0;
        var_alphas_dn6 = 0.0;
        var_alphas_dn7 = 0.0;
        var_alphas_dn8 = 0.0;
        var_alphas_rv = 0.0;

        var_rxcor = 1.0;
        var_rxcor_dn5 = 0.0;
        var_rxcor_dn6 = 0.0;
        var_rxcor_dn7 = 0.0;
        var_rxcor_dn8 = 0.0;
        var_rxcor_rv = 0.0;

        let assign42390_e55659: f64 = (var_xg - var_x_s);
        var_xgs = assign42390_e55659;
        var_xgs_dn5 = (var_xg_dn5 - var_x_s_dn5);
        var_xgs_dn6 = (var_xg_dn6 - var_x_s_dn6);
        var_xgs_dn7 = (var_xg_dn7 - var_x_s_dn7);
        var_xgs_dn8 = (var_xg_dn8 - var_x_s_dn8);
        var_xgs_rv = 0.0;

        var_qis = 0.0;
        var_qis_dn5 = 0.0;
        var_qis_dn6 = 0.0;
        var_qis_dn7 = 0.0;
        var_qis_dn8 = 0.0;
        var_qis_rv = 0.0;

        let assign42410_e55663: f64 = (var_phit1 * var_xgs);
        var_qbs = assign42410_e55663;
        var_qbs_dn5 = ((var_phit1_dn5 * var_xgs) + (var_phit1 * var_xgs_dn5));
        var_qbs_dn6 = ((var_phit1_dn6 * var_xgs) + (var_phit1 * var_xgs_dn6));
        var_qbs_dn7 = ((var_phit1_dn7 * var_xgs) + (var_phit1 * var_xgs_dn7));
        var_qbs_dn8 = ((var_phit1_dn8 * var_xgs) + (var_phit1 * var_xgs_dn8));
        var_qbs_rv = 0.0;

        var_rhob = 1.0;
        var_rhob_dn5 = 0.0;
        var_rhob_dn6 = 0.0;
        var_rhob_dn7 = 0.0;
        var_rhob_dn8 = 0.0;
        var_rhob_rv = 0.0;

        var_rhog = 1.0;
        var_rhog_dn5 = 0.0;
        var_rhog_dn6 = 0.0;
        var_rhog_dn7 = 0.0;
        var_rhog_dn8 = 0.0;
        var_rhog_rv = 0.0;

        var_gmobs = 1.0;
        var_gmobs_dn5 = 0.0;
        var_gmobs_dn6 = 0.0;
        var_gmobs_dn7 = 0.0;
        var_gmobs_dn8 = 0.0;
        var_gmobs_rv = 0.0;

        var_xitsb = 1.0;
        var_xitsb_dn5 = 0.0;
        var_xitsb_dn6 = 0.0;
        var_xitsb_dn7 = 0.0;
        var_xitsb_dn8 = 0.0;
        var_xitsb_rv = 0.0;

        var_factheta = 1.0;
        var_factheta_dn5 = 0.0;
        var_factheta_dn6 = 0.0;
        var_factheta_dn7 = 0.0;
        var_factheta_dn8 = 0.0;
        var_factheta_rv = 0.0;

        let assign42470_e55671: f64 = if var_xg > 0.0 { 1.0 } else { 0.0 };
        var_guard1188 = assign42470_e55671;
        var_guard1188_rv = 0.0;

        let (assign42480_e55681, assign42480_e55681_d_n5, assign42480_e55681_d_n6, assign42480_e55681_d_n7, assign42480_e55681_d_n8,) = {
    if (var_guard1188 != 0.0) {
        let assign42480_e55677: f64 = (var_x_s * var_x_s);
        let assign42480_e55678: f64 = (2.0 + assign42480_e55677);
        let assign42480_e55679: f64 = (1.0 / assign42480_e55678);
        (assign42480_e55679, (-(((var_x_s_dn5 * var_x_s) + (var_x_s * var_x_s_dn5)) / (assign42480_e55678 * assign42480_e55678))), (-(((var_x_s_dn6 * var_x_s) + (var_x_s * var_x_s_dn6)) / (assign42480_e55678 * assign42480_e55678))), (-(((var_x_s_dn7 * var_x_s) + (var_x_s * var_x_s_dn7)) / (assign42480_e55678 * assign42480_e55678))), (-(((var_x_s_dn8 * var_x_s) + (var_x_s * var_x_s_dn8)) / (assign42480_e55678 * assign42480_e55678))),)
    } else {
        (var_temp__blk936, var_temp__blk936_dn5, var_temp__blk936_dn6, var_temp__blk936_dn7, var_temp__blk936_dn8,)
    }
};
        var_temp__blk936 = assign42480_e55681;
        var_temp__blk936_dn5 = assign42480_e55681_d_n5;
        var_temp__blk936_dn6 = assign42480_e55681_d_n6;
        var_temp__blk936_dn7 = assign42480_e55681_d_n7;
        var_temp__blk936_dn8 = assign42480_e55681_d_n8;
        var_temp__blk936_rv = 0.0;

        let (assign42490_e55689, assign42490_e55689_d_n5, assign42490_e55689_d_n6, assign42490_e55689_d_n7, assign42490_e55689_d_n8,) = {
    if (var_guard1188 != 0.0) {
        let assign42490_e55685: f64 = (var_x_s * var_x_s);
        let assign42490_e55687: f64 = (assign42490_e55685 * var_temp__blk936);
        (assign42490_e55687, ((((var_x_s_dn5 * var_x_s) + (var_x_s * var_x_s_dn5)) * var_temp__blk936) + (assign42490_e55685 * var_temp__blk936_dn5)), ((((var_x_s_dn6 * var_x_s) + (var_x_s * var_x_s_dn6)) * var_temp__blk936) + (assign42490_e55685 * var_temp__blk936_dn6)), ((((var_x_s_dn7 * var_x_s) + (var_x_s * var_x_s_dn7)) * var_temp__blk936) + (assign42490_e55685 * var_temp__blk936_dn7)), ((((var_x_s_dn8 * var_x_s) + (var_x_s * var_x_s_dn8)) * var_temp__blk936) + (assign42490_e55685 * var_temp__blk936_dn8)),)
    } else {
        (var_xi0s, var_xi0s_dn5, var_xi0s_dn6, var_xi0s_dn7, var_xi0s_dn8,)
    }
};
        var_xi0s = assign42490_e55689;
        var_xi0s_dn5 = assign42490_e55689_d_n5;
        var_xi0s_dn6 = assign42490_e55689_d_n6;
        var_xi0s_dn7 = assign42490_e55689_d_n7;
        var_xi0s_dn8 = assign42490_e55689_d_n8;
        var_xi0s_rv = 0.0;

        let (assign42500_e55699, assign42500_e55699_d_n5, assign42500_e55699_d_n6, assign42500_e55699_d_n7, assign42500_e55699_d_n8,) = {
    if (var_guard1188 != 0.0) {
        let assign42500_e55694: f64 = (var_x_s * var_temp__blk936);
        let assign42500_e55696: f64 = (assign42500_e55694 * var_temp__blk936);
        let assign42500_e55697: f64 = (4.0 * assign42500_e55696);
        (assign42500_e55697, (4.0 * ((((var_x_s_dn5 * var_temp__blk936) + (var_x_s * var_temp__blk936_dn5)) * var_temp__blk936) + (assign42500_e55694 * var_temp__blk936_dn5))), (4.0 * ((((var_x_s_dn6 * var_temp__blk936) + (var_x_s * var_temp__blk936_dn6)) * var_temp__blk936) + (assign42500_e55694 * var_temp__blk936_dn6))), (4.0 * ((((var_x_s_dn7 * var_temp__blk936) + (var_x_s * var_temp__blk936_dn7)) * var_temp__blk936) + (assign42500_e55694 * var_temp__blk936_dn7))), (4.0 * ((((var_x_s_dn8 * var_temp__blk936) + (var_x_s * var_temp__blk936_dn8)) * var_temp__blk936) + (assign42500_e55694 * var_temp__blk936_dn8))),)
    } else {
        (var_xi1s, var_xi1s_dn5, var_xi1s_dn6, var_xi1s_dn7, var_xi1s_dn8,)
    }
};
        var_xi1s = assign42500_e55699;
        var_xi1s_dn5 = assign42500_e55699_d_n5;
        var_xi1s_dn6 = assign42500_e55699_d_n6;
        var_xi1s_dn7 = assign42500_e55699_d_n7;
        var_xi1s_dn8 = assign42500_e55699_d_n8;
        var_xi1s_rv = 0.0;

        let (assign42510_e55713, assign42510_e55713_d_n5, assign42510_e55713_d_n6, assign42510_e55713_d_n7, assign42510_e55713_d_n8,) = {
    if (var_guard1188 != 0.0) {
        let assign42510_e55703: f64 = (8.0 * var_temp__blk936);
        let assign42510_e55706: f64 = (12.0 * var_xi0s);
        let assign42510_e55707: f64 = (assign42510_e55703 - assign42510_e55706);
        let assign42510_e55709: f64 = (assign42510_e55707 * var_temp__blk936);
        let assign42510_e55711: f64 = (assign42510_e55709 * var_temp__blk936);
        (assign42510_e55711, ((((((8.0 * var_temp__blk936_dn5) - (12.0 * var_xi0s_dn5)) * var_temp__blk936) + (assign42510_e55707 * var_temp__blk936_dn5)) * var_temp__blk936) + (assign42510_e55709 * var_temp__blk936_dn5)), ((((((8.0 * var_temp__blk936_dn6) - (12.0 * var_xi0s_dn6)) * var_temp__blk936) + (assign42510_e55707 * var_temp__blk936_dn6)) * var_temp__blk936) + (assign42510_e55709 * var_temp__blk936_dn6)), ((((((8.0 * var_temp__blk936_dn7) - (12.0 * var_xi0s_dn7)) * var_temp__blk936) + (assign42510_e55707 * var_temp__blk936_dn7)) * var_temp__blk936) + (assign42510_e55709 * var_temp__blk936_dn7)), ((((((8.0 * var_temp__blk936_dn8) - (12.0 * var_xi0s_dn8)) * var_temp__blk936) + (assign42510_e55707 * var_temp__blk936_dn8)) * var_temp__blk936) + (assign42510_e55709 * var_temp__blk936_dn8)),)
    } else {
        (var_xi2s, var_xi2s_dn5, var_xi2s_dn6, var_xi2s_dn7, var_xi2s_dn8,)
    }
};
        var_xi2s = assign42510_e55713;
        var_xi2s_dn5 = assign42510_e55713_d_n5;
        var_xi2s_dn6 = assign42510_e55713_d_n6;
        var_xi2s_dn7 = assign42510_e55713_d_n7;
        var_xi2s_dn8 = assign42510_e55713_d_n8;
        var_xi2s_rv = 0.0;

        let (assign42520_e55717, assign42520_e55717_d_n5, assign42520_e55717_d_n6, assign42520_e55717_d_n7, assign42520_e55717_d_n8,) = {
    if (var_guard1188 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_delta_1s, var_delta_1s_dn5, var_delta_1s_dn6, var_delta_1s_dn7, var_delta_1s_dn8,)
    }
};
        var_delta_1s = assign42520_e55717;
        var_delta_1s_dn5 = assign42520_e55717_d_n5;
        var_delta_1s_dn6 = assign42520_e55717_d_n6;
        var_delta_1s_dn7 = assign42520_e55717_d_n7;
        var_delta_1s_dn8 = assign42520_e55717_d_n8;
        var_delta_1s_rv = 0.0;

        let assign42530_e55720: f64 = if var_x_s < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard1189 = assign42530_e55720;
        var_guard1189_rv = 0.0;

        let (assign42540_e55727, assign42540_e55727_d_n5, assign42540_e55727_d_n6, assign42540_e55727_d_n7, assign42540_e55727_d_n8,) = {
    if ((var_guard1188 != 0.0) && (var_guard1189 != 0.0)) {
        let assign42540_e55725: f64 = (var_x_s).exp();
        (assign42540_e55725, (assign42540_e55725 * var_x_s_dn5), (assign42540_e55725 * var_x_s_dn6), (assign42540_e55725 * var_x_s_dn7), (assign42540_e55725 * var_x_s_dn8),)
    } else {
        (var_delta_1s, var_delta_1s_dn5, var_delta_1s_dn6, var_delta_1s_dn7, var_delta_1s_dn8,)
    }
};
        var_delta_1s = assign42540_e55727;
        var_delta_1s_dn5 = assign42540_e55727_d_n5;
        var_delta_1s_dn6 = assign42540_e55727_d_n6;
        var_delta_1s_dn7 = assign42540_e55727_d_n7;
        var_delta_1s_dn8 = assign42540_e55727_d_n8;
        var_delta_1s_rv = 0.0;

        let (assign42550_e55735, assign42550_e55735_d_n5, assign42550_e55735_d_n6, assign42550_e55735_d_n7, assign42550_e55735_d_n8,) = {
    if ((var_guard1188 != 0.0) && (var_guard1189 != 0.0)) {
        let assign42550_e55733: f64 = (1.0 / var_delta_1s);
        (assign42550_e55733, (-(var_delta_1s_dn5 / (var_delta_1s * var_delta_1s))), (-(var_delta_1s_dn6 / (var_delta_1s * var_delta_1s))), (-(var_delta_1s_dn7 / (var_delta_1s * var_delta_1s))), (-(var_delta_1s_dn8 / (var_delta_1s * var_delta_1s))),)
    } else {
        (var_es, var_es_dn5, var_es_dn6, var_es_dn7, var_es_dn8,)
    }
};
        var_es = assign42550_e55735;
        var_es_dn5 = assign42550_e55735_d_n5;
        var_es_dn6 = assign42550_e55735_d_n6;
        var_es_dn7 = assign42550_e55735_d_n7;
        var_es_dn8 = assign42550_e55735_d_n8;
        var_es_rv = 0.0;

        let (assign42560_e55743, assign42560_e55743_d_n5, assign42560_e55743_d_n6, assign42560_e55743_d_n7, assign42560_e55743_d_n8,) = {
    if ((var_guard1188 != 0.0) && (var_guard1189 != 0.0)) {
        let assign42560_e55741: f64 = (var_delta_ns * var_delta_1s);
        (assign42560_e55741, ((var_delta_ns_dn5 * var_delta_1s) + (var_delta_ns * var_delta_1s_dn5)), ((var_delta_ns_dn6 * var_delta_1s) + (var_delta_ns * var_delta_1s_dn6)), ((var_delta_ns_dn7 * var_delta_1s) + (var_delta_ns * var_delta_1s_dn7)), ((var_delta_ns_dn8 * var_delta_1s) + (var_delta_ns * var_delta_1s_dn8)),)
    } else {
        (var_delta_1s, var_delta_1s_dn5, var_delta_1s_dn6, var_delta_1s_dn7, var_delta_1s_dn8,)
    }
};
        var_delta_1s = assign42560_e55743;
        var_delta_1s_dn5 = assign42560_e55743_d_n5;
        var_delta_1s_dn6 = assign42560_e55743_d_n6;
        var_delta_1s_dn7 = assign42560_e55743_d_n7;
        var_delta_1s_dn8 = assign42560_e55743_d_n8;
        var_delta_1s_rv = 0.0;

        let assign42570_e55747: f64 = (var_xn_s - 230.25850929940458);
        let assign42570_e55748: f64 = if var_x_s > assign42570_e55747 { 1.0 } else { 0.0 };
        var_guard1190 = assign42570_e55748;
        var_guard1190_rv = 0.0;

        let (assign42580_e55760, assign42580_e55760_d_n5, assign42580_e55760_d_n6, assign42580_e55760_d_n7, assign42580_e55760_d_n8,) = {
    if (((var_guard1188 != 0.0) && (var_guard1189 == 0.0)) && (var_guard1190 != 0.0)) {
        let assign42580_e55757: f64 = (var_x_s - var_xn_s);
        let assign42580_e55758: f64 = (assign42580_e55757).exp();
        (assign42580_e55758, (assign42580_e55758 * (var_x_s_dn5 - var_xn_s_dn5)), (assign42580_e55758 * (var_x_s_dn6 - var_xn_s_dn6)), (assign42580_e55758 * (var_x_s_dn7 - var_xn_s_dn7)), (assign42580_e55758 * (var_x_s_dn8 - var_xn_s_dn8)),)
    } else {
        (var_delta_1s, var_delta_1s_dn5, var_delta_1s_dn6, var_delta_1s_dn7, var_delta_1s_dn8,)
    }
};
        var_delta_1s = assign42580_e55760;
        var_delta_1s_dn5 = assign42580_e55760_d_n5;
        var_delta_1s_dn6 = assign42580_e55760_d_n6;
        var_delta_1s_dn7 = assign42580_e55760_d_n7;
        var_delta_1s_dn8 = assign42580_e55760_d_n8;
        var_delta_1s_rv = 0.0;

        let (assign42590_e55771, assign42590_e55771_d_n5, assign42590_e55771_d_n6, assign42590_e55771_d_n7, assign42590_e55771_d_n8,) = {
    if (((var_guard1188 != 0.0) && (var_guard1189 == 0.0)) && (var_guard1190 != 0.0)) {
        let assign42590_e55769: f64 = (var_delta_ns / var_delta_1s);
        (assign42590_e55769, (((var_delta_ns_dn5 * var_delta_1s) - (var_delta_ns * var_delta_1s_dn5)) / (var_delta_1s * var_delta_1s)), (((var_delta_ns_dn6 * var_delta_1s) - (var_delta_ns * var_delta_1s_dn6)) / (var_delta_1s * var_delta_1s)), (((var_delta_ns_dn7 * var_delta_1s) - (var_delta_ns * var_delta_1s_dn7)) / (var_delta_1s * var_delta_1s)), (((var_delta_ns_dn8 * var_delta_1s) - (var_delta_ns * var_delta_1s_dn8)) / (var_delta_1s * var_delta_1s)),)
    } else {
        (var_es, var_es_dn5, var_es_dn6, var_es_dn7, var_es_dn8,)
    }
};
        var_es = assign42590_e55771;
        var_es_dn5 = assign42590_e55771_d_n5;
        var_es_dn6 = assign42590_e55771_d_n6;
        var_es_dn7 = assign42590_e55771_d_n7;
        var_es_dn8 = assign42590_e55771_d_n8;
        var_es_rv = 0.0;

        let (assign42600_e55809, assign42600_e55809_d_n5, assign42600_e55809_d_n6, assign42600_e55809_d_n7, assign42600_e55809_d_n8,) = {
    if (((var_guard1188 != 0.0) && (var_guard1189 == 0.0)) && (var_guard1190 == 0.0)) {
        let assign42600_e55783: f64 = (var_xn_s - var_x_s);
        let assign42600_e55785: f64 = (assign42600_e55783 - 230.25850929940458);
        let assign42600_e55790: f64 = (var_xn_s - var_x_s);
        let assign42600_e55792: f64 = (assign42600_e55790 - 230.25850929940458);
        let assign42600_e55796: f64 = (var_xn_s - var_x_s);
        let assign42600_e55798: f64 = (assign42600_e55796 - 230.25850929940458);
        let assign42600_e55800: f64 = (assign42600_e55798 * 0.3333333333333333);
        let assign42600_e55801: f64 = (1.0 + assign42600_e55800);
        let assign42600_e55802: f64 = (assign42600_e55792 * assign42600_e55801);
        let assign42600_e55803: f64 = (0.5 * assign42600_e55802);
        let assign42600_e55804: f64 = (1.0 + assign42600_e55803);
        let assign42600_e55805: f64 = (assign42600_e55785 * assign42600_e55804);
        let assign42600_e55806: f64 = (1.0 + assign42600_e55805);
        let assign42600_e55807: f64 = (1e-100 / assign42600_e55806);
        (assign42600_e55807, (-((1e-100 * (((var_xn_s_dn5 - var_x_s_dn5) * assign42600_e55804) + (assign42600_e55785 * (0.5 * (((var_xn_s_dn5 - var_x_s_dn5) * assign42600_e55801) + (assign42600_e55792 * ((var_xn_s_dn5 - var_x_s_dn5) * 0.3333333333333333))))))) / (assign42600_e55806 * assign42600_e55806))), (-((1e-100 * (((var_xn_s_dn6 - var_x_s_dn6) * assign42600_e55804) + (assign42600_e55785 * (0.5 * (((var_xn_s_dn6 - var_x_s_dn6) * assign42600_e55801) + (assign42600_e55792 * ((var_xn_s_dn6 - var_x_s_dn6) * 0.3333333333333333))))))) / (assign42600_e55806 * assign42600_e55806))), (-((1e-100 * (((var_xn_s_dn7 - var_x_s_dn7) * assign42600_e55804) + (assign42600_e55785 * (0.5 * (((var_xn_s_dn7 - var_x_s_dn7) * assign42600_e55801) + (assign42600_e55792 * ((var_xn_s_dn7 - var_x_s_dn7) * 0.3333333333333333))))))) / (assign42600_e55806 * assign42600_e55806))), (-((1e-100 * (((var_xn_s_dn8 - var_x_s_dn8) * assign42600_e55804) + (assign42600_e55785 * (0.5 * (((var_xn_s_dn8 - var_x_s_dn8) * assign42600_e55801) + (assign42600_e55792 * ((var_xn_s_dn8 - var_x_s_dn8) * 0.3333333333333333))))))) / (assign42600_e55806 * assign42600_e55806))),)
    } else {
        (var_delta_1s, var_delta_1s_dn5, var_delta_1s_dn6, var_delta_1s_dn7, var_delta_1s_dn8,)
    }
};
        var_delta_1s = assign42600_e55809;
        var_delta_1s_dn5 = assign42600_e55809_d_n5;
        var_delta_1s_dn6 = assign42600_e55809_d_n6;
        var_delta_1s_dn7 = assign42600_e55809_d_n7;
        var_delta_1s_dn8 = assign42600_e55809_d_n8;
        var_delta_1s_rv = 0.0;

        let (assign42610_e55841, assign42610_e55841_d_n5, assign42610_e55841_d_n6, assign42610_e55841_d_n7, assign42610_e55841_d_n8,) = {
    if (((var_guard1188 != 0.0) && (var_guard1189 == 0.0)) && (var_guard1190 == 0.0)) {
        let assign42610_e55821: f64 = (var_x_s - 230.25850929940458);
        let assign42610_e55826: f64 = (var_x_s - 230.25850929940458);
        let assign42610_e55830: f64 = (var_x_s - 230.25850929940458);
        let assign42610_e55832: f64 = (assign42610_e55830 * 0.3333333333333333);
        let assign42610_e55833: f64 = (1.0 + assign42610_e55832);
        let assign42610_e55834: f64 = (assign42610_e55826 * assign42610_e55833);
        let assign42610_e55835: f64 = (0.5 * assign42610_e55834);
        let assign42610_e55836: f64 = (1.0 + assign42610_e55835);
        let assign42610_e55837: f64 = (assign42610_e55821 * assign42610_e55836);
        let assign42610_e55838: f64 = (1.0 + assign42610_e55837);
        let assign42610_e55839: f64 = (1e-100 / assign42610_e55838);
        (assign42610_e55839, (-((1e-100 * ((var_x_s_dn5 * assign42610_e55836) + (assign42610_e55821 * (0.5 * ((var_x_s_dn5 * assign42610_e55833) + (assign42610_e55826 * (var_x_s_dn5 * 0.3333333333333333))))))) / (assign42610_e55838 * assign42610_e55838))), (-((1e-100 * ((var_x_s_dn6 * assign42610_e55836) + (assign42610_e55821 * (0.5 * ((var_x_s_dn6 * assign42610_e55833) + (assign42610_e55826 * (var_x_s_dn6 * 0.3333333333333333))))))) / (assign42610_e55838 * assign42610_e55838))), (-((1e-100 * ((var_x_s_dn7 * assign42610_e55836) + (assign42610_e55821 * (0.5 * ((var_x_s_dn7 * assign42610_e55833) + (assign42610_e55826 * (var_x_s_dn7 * 0.3333333333333333))))))) / (assign42610_e55838 * assign42610_e55838))), (-((1e-100 * ((var_x_s_dn8 * assign42610_e55836) + (assign42610_e55821 * (0.5 * ((var_x_s_dn8 * assign42610_e55833) + (assign42610_e55826 * (var_x_s_dn8 * 0.3333333333333333))))))) / (assign42610_e55838 * assign42610_e55838))),)
    } else {
        (var_es, var_es_dn5, var_es_dn6, var_es_dn7, var_es_dn8,)
    }
};
        var_es = assign42610_e55841;
        var_es_dn5 = assign42610_e55841_d_n5;
        var_es_dn6 = assign42610_e55841_d_n6;
        var_es_dn7 = assign42610_e55841_d_n7;
        var_es_dn8 = assign42610_e55841_d_n8;
        var_es_rv = 0.0;

        *var_alphas_slot = var_alphas;
        *var_alphas_dn5_slot = var_alphas_dn5;
        *var_alphas_dn6_slot = var_alphas_dn6;
        *var_alphas_dn7_slot = var_alphas_dn7;
        *var_alphas_dn8_slot = var_alphas_dn8;
        *var_alphas_rv_slot = var_alphas_rv;
        *var_delta_1s_slot = var_delta_1s;
        *var_delta_1s_dn5_slot = var_delta_1s_dn5;
        *var_delta_1s_dn6_slot = var_delta_1s_dn6;
        *var_delta_1s_dn7_slot = var_delta_1s_dn7;
        *var_delta_1s_dn8_slot = var_delta_1s_dn8;
        *var_delta_1s_rv_slot = var_delta_1s_rv;
        *var_ds_slot = var_ds;
        *var_ds_dn5_slot = var_ds_dn5;
        *var_ds_dn6_slot = var_ds_dn6;
        *var_ds_dn7_slot = var_ds_dn7;
        *var_ds_dn8_slot = var_ds_dn8;
        *var_ds_rv_slot = var_ds_rv;
        *var_es_slot = var_es;
        *var_es_dn5_slot = var_es_dn5;
        *var_es_dn6_slot = var_es_dn6;
        *var_es_dn7_slot = var_es_dn7;
        *var_es_dn8_slot = var_es_dn8;
        *var_es_rv_slot = var_es_rv;
        *var_factheta_slot = var_factheta;
        *var_factheta_dn5_slot = var_factheta_dn5;
        *var_factheta_dn6_slot = var_factheta_dn6;
        *var_factheta_dn7_slot = var_factheta_dn7;
        *var_factheta_dn8_slot = var_factheta_dn8;
        *var_factheta_rv_slot = var_factheta_rv;
        *var_gmobs_slot = var_gmobs;
        *var_gmobs_dn5_slot = var_gmobs_dn5;
        *var_gmobs_dn6_slot = var_gmobs_dn6;
        *var_gmobs_dn7_slot = var_gmobs_dn7;
        *var_gmobs_dn8_slot = var_gmobs_dn8;
        *var_gmobs_rv_slot = var_gmobs_rv;
        *var_guard1188_slot = var_guard1188;
        *var_guard1188_rv_slot = var_guard1188_rv;
        *var_guard1189_slot = var_guard1189;
        *var_guard1189_rv_slot = var_guard1189_rv;
        *var_guard1190_slot = var_guard1190;
        *var_guard1190_rv_slot = var_guard1190_rv;
        *var_ps_slot = var_ps;
        *var_ps_dn5_slot = var_ps_dn5;
        *var_ps_dn6_slot = var_ps_dn6;
        *var_ps_dn7_slot = var_ps_dn7;
        *var_ps_dn8_slot = var_ps_dn8;
        *var_ps_rv_slot = var_ps_rv;
        *var_qbs_slot = var_qbs;
        *var_qbs_dn5_slot = var_qbs_dn5;
        *var_qbs_dn6_slot = var_qbs_dn6;
        *var_qbs_dn7_slot = var_qbs_dn7;
        *var_qbs_dn8_slot = var_qbs_dn8;
        *var_qbs_rv_slot = var_qbs_rv;
        *var_qis_slot = var_qis;
        *var_qis_dn5_slot = var_qis_dn5;
        *var_qis_dn6_slot = var_qis_dn6;
        *var_qis_dn7_slot = var_qis_dn7;
        *var_qis_dn8_slot = var_qis_dn8;
        *var_qis_rv_slot = var_qis_rv;
        *var_rhob_slot = var_rhob;
        *var_rhob_dn5_slot = var_rhob_dn5;
        *var_rhob_dn6_slot = var_rhob_dn6;
        *var_rhob_dn7_slot = var_rhob_dn7;
        *var_rhob_dn8_slot = var_rhob_dn8;
        *var_rhob_rv_slot = var_rhob_rv;
        *var_rhog_slot = var_rhog;
        *var_rhog_dn5_slot = var_rhog_dn5;
        *var_rhog_dn6_slot = var_rhog_dn6;
        *var_rhog_dn7_slot = var_rhog_dn7;
        *var_rhog_dn8_slot = var_rhog_dn8;
        *var_rhog_rv_slot = var_rhog_rv;
        *var_rxcor_slot = var_rxcor;
        *var_rxcor_dn5_slot = var_rxcor_dn5;
        *var_rxcor_dn6_slot = var_rxcor_dn6;
        *var_rxcor_dn7_slot = var_rxcor_dn7;
        *var_rxcor_dn8_slot = var_rxcor_dn8;
        *var_rxcor_rv_slot = var_rxcor_rv;
        *var_sp_s_pc_slot = var_sp_s_pc;
        *var_sp_s_pc_dn5_slot = var_sp_s_pc_dn5;
        *var_sp_s_pc_dn6_slot = var_sp_s_pc_dn6;
        *var_sp_s_pc_dn7_slot = var_sp_s_pc_dn7;
        *var_sp_s_pc_dn8_slot = var_sp_s_pc_dn8;
        *var_sp_s_pc_rv_slot = var_sp_s_pc_rv;
        *var_sp_s_qc_slot = var_sp_s_qc;
        *var_sp_s_qc_dn5_slot = var_sp_s_qc_dn5;
        *var_sp_s_qc_dn6_slot = var_sp_s_qc_dn6;
        *var_sp_s_qc_dn7_slot = var_sp_s_qc_dn7;
        *var_sp_s_qc_dn8_slot = var_sp_s_qc_dn8;
        *var_sp_s_qc_rv_slot = var_sp_s_qc_rv;
        *var_sp_s_temp_slot = var_sp_s_temp;
        *var_sp_s_temp_dn5_slot = var_sp_s_temp_dn5;
        *var_sp_s_temp_dn6_slot = var_sp_s_temp_dn6;
        *var_sp_s_temp_dn7_slot = var_sp_s_temp_dn7;
        *var_sp_s_temp_dn8_slot = var_sp_s_temp_dn8;
        *var_sp_s_temp_rv_slot = var_sp_s_temp_rv;
        *var_sp_s_xi1_slot = var_sp_s_xi1;
        *var_sp_s_xi1_dn5_slot = var_sp_s_xi1_dn5;
        *var_sp_s_xi1_dn6_slot = var_sp_s_xi1_dn6;
        *var_sp_s_xi1_dn7_slot = var_sp_s_xi1_dn7;
        *var_sp_s_xi1_dn8_slot = var_sp_s_xi1_dn8;
        *var_sp_s_xi1_rv_slot = var_sp_s_xi1_rv;
        *var_sp_s_xi2_slot = var_sp_s_xi2;
        *var_sp_s_xi2_dn5_slot = var_sp_s_xi2_dn5;
        *var_sp_s_xi2_dn6_slot = var_sp_s_xi2_dn6;
        *var_sp_s_xi2_dn7_slot = var_sp_s_xi2_dn7;
        *var_sp_s_xi2_dn8_slot = var_sp_s_xi2_dn8;
        *var_sp_s_xi2_rv_slot = var_sp_s_xi2_rv;
        *var_sqs_slot = var_sqs;
        *var_sqs_dn5_slot = var_sqs_dn5;
        *var_sqs_dn6_slot = var_sqs_dn6;
        *var_sqs_dn7_slot = var_sqs_dn7;
        *var_sqs_dn8_slot = var_sqs_dn8;
        *var_sqs_rv_slot = var_sqs_rv;
        *var_temp__blk936_slot = var_temp__blk936;
        *var_temp__blk936_dn5_slot = var_temp__blk936_dn5;
        *var_temp__blk936_dn6_slot = var_temp__blk936_dn6;
        *var_temp__blk936_dn7_slot = var_temp__blk936_dn7;
        *var_temp__blk936_dn8_slot = var_temp__blk936_dn8;
        *var_temp__blk936_rv_slot = var_temp__blk936_rv;
        *var_x_s_slot = var_x_s;
        *var_x_s_dn5_slot = var_x_s_dn5;
        *var_x_s_dn6_slot = var_x_s_dn6;
        *var_x_s_dn7_slot = var_x_s_dn7;
        *var_x_s_dn8_slot = var_x_s_dn8;
        *var_x_s_rv_slot = var_x_s_rv;
        *var_xgs_slot = var_xgs;
        *var_xgs_dn5_slot = var_xgs_dn5;
        *var_xgs_dn6_slot = var_xgs_dn6;
        *var_xgs_dn7_slot = var_xgs_dn7;
        *var_xgs_dn8_slot = var_xgs_dn8;
        *var_xgs_rv_slot = var_xgs_rv;
        *var_xi0s_slot = var_xi0s;
        *var_xi0s_dn5_slot = var_xi0s_dn5;
        *var_xi0s_dn6_slot = var_xi0s_dn6;
        *var_xi0s_dn7_slot = var_xi0s_dn7;
        *var_xi0s_dn8_slot = var_xi0s_dn8;
        *var_xi0s_rv_slot = var_xi0s_rv;
        *var_xi1s_slot = var_xi1s;
        *var_xi1s_dn5_slot = var_xi1s_dn5;
        *var_xi1s_dn6_slot = var_xi1s_dn6;
        *var_xi1s_dn7_slot = var_xi1s_dn7;
        *var_xi1s_dn8_slot = var_xi1s_dn8;
        *var_xi1s_rv_slot = var_xi1s_rv;
        *var_xi2s_slot = var_xi2s;
        *var_xi2s_dn5_slot = var_xi2s_dn5;
        *var_xi2s_dn6_slot = var_xi2s_dn6;
        *var_xi2s_dn7_slot = var_xi2s_dn7;
        *var_xi2s_dn8_slot = var_xi2s_dn8;
        *var_xi2s_rv_slot = var_xi2s_rv;
        *var_xitsb_slot = var_xitsb;
        *var_xitsb_dn5_slot = var_xitsb_dn5;
        *var_xitsb_dn6_slot = var_xitsb_dn6;
        *var_xitsb_dn7_slot = var_xitsb_dn7;
        *var_xitsb_dn8_slot = var_xitsb_dn8;
        *var_xitsb_rv_slot = var_xitsb_rv;
    }
}
