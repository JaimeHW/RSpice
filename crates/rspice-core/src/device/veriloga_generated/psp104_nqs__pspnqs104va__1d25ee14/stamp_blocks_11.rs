#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_8(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        locals: &mut StampLocals,
    ) {
        let (assign8490_e7857,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard109 != 0.0)) {
        (p.p574,)
    } else {
        (locals.var_plwparam_i,)
    }
};
        locals.var_plwparam_i = assign8490_e7857;
        locals.var_plwparam_i_rv = 0.0;

        let assign8500_e7859: f64 = if param_given[666] { 1.0 } else { 0.0 };
        let assign8500_e7861: f64 = if assign8500_e7859 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard113 = assign8500_e7861;
        locals.var_guard113_rv = 0.0;

        let (assign8510_e7869,) = {
    if (((locals.var_guard41 != 0.0) && (locals.var_guard109 != 0.0)) && (locals.var_guard113 != 0.0)) {
        (p.p666,)
    } else {
        (locals.var_plwparam_i,)
    }
};
        locals.var_plwparam_i = assign8510_e7869;
        locals.var_plwparam_i_rv = 0.0;

        let (assign8520_e7889,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard109 != 0.0)) {
        let assign8520_e7877: f64 = (locals.var_plparam_i * locals.var_ile);
        let assign8520_e7878: f64 = (locals.var_poparam_i + assign8520_e7877);
        let assign8520_e7881: f64 = (locals.var_pwparam_i * locals.var_iwe);
        let assign8520_e7882: f64 = (assign8520_e7878 + assign8520_e7881);
        let assign8520_e7885: f64 = (locals.var_plwparam_i * locals.var_iae);
        let assign8520_e7886: f64 = (assign8520_e7882 + assign8520_e7885);
        let assign8520_e7887: f64 = (locals.var_ile * assign8520_e7886);
        (assign8520_e7887,)
    } else {
        (locals.var_thesatac_p,)
    }
};
        locals.var_thesatac_p = assign8520_e7889;
        locals.var_thesatac_p_rv = 0.0;

        let assign8530_e7928: f64 = if (((((((param_given[667] || param_given[668]) || param_given[669]) || param_given[670]) || param_given[587]) || param_given[588]) || param_given[589]) || param_given[590]) { 1.0 } else { 0.0 };
        locals.var_guard114 = assign8530_e7928;
        locals.var_guard114_rv = 0.0;

        let (assign8540_e7934,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard114 != 0.0)) {
        (p.p587,)
    } else {
        (locals.var_poparam_i,)
    }
};
        locals.var_poparam_i = assign8540_e7934;
        locals.var_poparam_i_rv = 0.0;

        let assign8550_e7936: f64 = if param_given[667] { 1.0 } else { 0.0 };
        let assign8550_e7938: f64 = if assign8550_e7936 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard115 = assign8550_e7938;
        locals.var_guard115_rv = 0.0;

        let (assign8560_e7946,) = {
    if (((locals.var_guard41 != 0.0) && (locals.var_guard114 != 0.0)) && (locals.var_guard115 != 0.0)) {
        (p.p667,)
    } else {
        (locals.var_poparam_i,)
    }
};
        locals.var_poparam_i = assign8560_e7946;
        locals.var_poparam_i_rv = 0.0;

        let (assign8570_e7952,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard114 != 0.0)) {
        (p.p588,)
    } else {
        (locals.var_plparam_i,)
    }
};
        locals.var_plparam_i = assign8570_e7952;
        locals.var_plparam_i_rv = 0.0;

        let assign8580_e7954: f64 = if param_given[668] { 1.0 } else { 0.0 };
        let assign8580_e7956: f64 = if assign8580_e7954 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard116 = assign8580_e7956;
        locals.var_guard116_rv = 0.0;

        let (assign8590_e7964,) = {
    if (((locals.var_guard41 != 0.0) && (locals.var_guard114 != 0.0)) && (locals.var_guard116 != 0.0)) {
        (p.p668,)
    } else {
        (locals.var_plparam_i,)
    }
};
        locals.var_plparam_i = assign8590_e7964;
        locals.var_plparam_i_rv = 0.0;

        let (assign8600_e7970,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard114 != 0.0)) {
        (p.p589,)
    } else {
        (locals.var_pwparam_i,)
    }
};
        locals.var_pwparam_i = assign8600_e7970;
        locals.var_pwparam_i_rv = 0.0;

        let assign8610_e7972: f64 = if param_given[669] { 1.0 } else { 0.0 };
        let assign8610_e7974: f64 = if assign8610_e7972 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard117 = assign8610_e7974;
        locals.var_guard117_rv = 0.0;

        let (assign8620_e7982,) = {
    if (((locals.var_guard41 != 0.0) && (locals.var_guard114 != 0.0)) && (locals.var_guard117 != 0.0)) {
        (p.p669,)
    } else {
        (locals.var_pwparam_i,)
    }
};
        locals.var_pwparam_i = assign8620_e7982;
        locals.var_pwparam_i_rv = 0.0;

        let (assign8630_e7988,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard114 != 0.0)) {
        (p.p590,)
    } else {
        (locals.var_plwparam_i,)
    }
};
        locals.var_plwparam_i = assign8630_e7988;
        locals.var_plwparam_i_rv = 0.0;

        let assign8640_e7990: f64 = if param_given[670] { 1.0 } else { 0.0 };
        let assign8640_e7992: f64 = if assign8640_e7990 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard118 = assign8640_e7992;
        locals.var_guard118_rv = 0.0;

        let (assign8650_e8000,) = {
    if (((locals.var_guard41 != 0.0) && (locals.var_guard114 != 0.0)) && (locals.var_guard118 != 0.0)) {
        (p.p670,)
    } else {
        (locals.var_plwparam_i,)
    }
};
        locals.var_plwparam_i = assign8650_e8000;
        locals.var_plwparam_i_rv = 0.0;

        let (assign8660_e8020,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard114 != 0.0)) {
        let assign8660_e8008: f64 = (locals.var_plparam_i * locals.var_ile);
        let assign8660_e8009: f64 = (locals.var_poparam_i + assign8660_e8008);
        let assign8660_e8012: f64 = (locals.var_pwparam_i * locals.var_iwe);
        let assign8660_e8013: f64 = (assign8660_e8009 + assign8660_e8012);
        let assign8660_e8016: f64 = (locals.var_plwparam_i * locals.var_iae);
        let assign8660_e8017: f64 = (assign8660_e8013 + assign8660_e8016);
        let assign8660_e8018: f64 = assign8660_e8017;
        (assign8660_e8018,)
    } else {
        (locals.var_axac_p,)
    }
};
        locals.var_axac_p = assign8660_e8020;
        locals.var_axac_p_rv = 0.0;

        let assign8670_e8039: f64 = if (((param_given[671] || param_given[672]) || param_given[673]) || param_given[674]) { 1.0 } else { 0.0 };
        locals.var_guard119 = assign8670_e8039;
        locals.var_guard119_rv = 0.0;

        let (assign8680_e8059,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard119 != 0.0)) {
        let assign8680_e8047: f64 = (p.p672 * locals.var_ile);
        let assign8680_e8048: f64 = (p.p671 + assign8680_e8047);
        let assign8680_e8051: f64 = (p.p673 * locals.var_iwe);
        let assign8680_e8052: f64 = (assign8680_e8048 + assign8680_e8051);
        let assign8680_e8055: f64 = (p.p674 * locals.var_iae);
        let assign8680_e8056: f64 = (assign8680_e8052 + assign8680_e8055);
        let assign8680_e8057: f64 = (locals.var_ile * assign8680_e8056);
        (assign8680_e8057,)
    } else {
        (locals.var_alpac_p,)
    }
};
        locals.var_alpac_p = assign8680_e8059;
        locals.var_alpac_p_rv = 0.0;

        let assign8690_e8078: f64 = if (((param_given[675] || param_given[676]) || param_given[677]) || param_given[678]) { 1.0 } else { 0.0 };
        locals.var_guard120 = assign8690_e8078;
        locals.var_guard120_rv = 0.0;

        let (assign8700_e8098,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard120 != 0.0)) {
        let assign8700_e8086: f64 = (p.p676 * locals.var_ile);
        let assign8700_e8087: f64 = (p.p675 + assign8700_e8086);
        let assign8700_e8090: f64 = (p.p677 * locals.var_iwe);
        let assign8700_e8091: f64 = (assign8700_e8087 + assign8700_e8090);
        let assign8700_e8094: f64 = (p.p678 * locals.var_iae);
        let assign8700_e8095: f64 = (assign8700_e8091 + assign8700_e8094);
        let assign8700_e8096: f64 = (locals.var_ile * assign8700_e8095);
        (assign8700_e8096,)
    } else {
        (locals.var_alp1ac_p,)
    }
};
        locals.var_alp1ac_p = assign8700_e8098;
        locals.var_alp1ac_p_rv = 0.0;

        let assign8710_e8117: f64 = if (((param_given[679] || param_given[680]) || param_given[681]) || param_given[682]) { 1.0 } else { 0.0 };
        locals.var_guard121 = assign8710_e8117;
        locals.var_guard121_rv = 0.0;

        let (assign8720_e8137,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard121 != 0.0)) {
        let assign8720_e8125: f64 = (p.p680 * locals.var_ile);
        let assign8720_e8126: f64 = (p.p679 + assign8720_e8125);
        let assign8720_e8129: f64 = (p.p681 * locals.var_iwe);
        let assign8720_e8130: f64 = (assign8720_e8126 + assign8720_e8129);
        let assign8720_e8133: f64 = (p.p682 * locals.var_iae);
        let assign8720_e8134: f64 = (assign8720_e8130 + assign8720_e8133);
        let assign8720_e8135: f64 = (locals.var_iiwecv * assign8720_e8134);
        (assign8720_e8135,)
    } else {
        (locals.var_cgov_p,)
    }
};
        locals.var_cgov_p = assign8720_e8137;
        locals.var_cgov_p_rv = 0.0;

        let assign8730_e8156: f64 = if (((param_given[683] || param_given[684]) || param_given[685]) || param_given[686]) { 1.0 } else { 0.0 };
        locals.var_guard122 = assign8730_e8156;
        locals.var_guard122_rv = 0.0;

        let (assign8740_e8176,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard122 != 0.0)) {
        let assign8740_e8164: f64 = (p.p684 * locals.var_ile);
        let assign8740_e8165: f64 = (p.p683 + assign8740_e8164);
        let assign8740_e8168: f64 = (p.p685 * locals.var_iwe);
        let assign8740_e8169: f64 = (assign8740_e8165 + assign8740_e8168);
        let assign8740_e8172: f64 = (p.p686 * locals.var_iae);
        let assign8740_e8173: f64 = (assign8740_e8169 + assign8740_e8172);
        let assign8740_e8174: f64 = (locals.var_iiwecv * assign8740_e8173);
        (assign8740_e8174,)
    } else {
        (locals.var_cgovd_p,)
    }
};
        locals.var_cgovd_p = assign8740_e8176;
        locals.var_cgovd_p_rv = 0.0;

        let assign8750_e8195: f64 = if (((param_given[687] || param_given[688]) || param_given[689]) || param_given[690]) { 1.0 } else { 0.0 };
        locals.var_guard123 = assign8750_e8195;
        locals.var_guard123_rv = 0.0;

        let (assign8760_e8215,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard123 != 0.0)) {
        let assign8760_e8203: f64 = (p.p688 * locals.var_ile);
        let assign8760_e8204: f64 = (p.p687 + assign8760_e8203);
        let assign8760_e8207: f64 = (p.p689 * locals.var_iwe);
        let assign8760_e8208: f64 = (assign8760_e8204 + assign8760_e8207);
        let assign8760_e8211: f64 = (p.p690 * locals.var_iae);
        let assign8760_e8212: f64 = (assign8760_e8208 + assign8760_e8211);
        let assign8760_e8213: f64 = (locals.var_iilcv * assign8760_e8212);
        (assign8760_e8213,)
    } else {
        (locals.var_cgbov_p,)
    }
};
        locals.var_cgbov_p = assign8760_e8215;
        locals.var_cgbov_p_rv = 0.0;

        let assign8770_e8234: f64 = if (((param_given[691] || param_given[692]) || param_given[693]) || param_given[694]) { 1.0 } else { 0.0 };
        locals.var_guard124 = assign8770_e8234;
        locals.var_guard124_rv = 0.0;

        let (assign8780_e8254,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard124 != 0.0)) {
        let assign8780_e8242: f64 = (p.p692 * locals.var_ile);
        let assign8780_e8243: f64 = (p.p691 + assign8780_e8242);
        let assign8780_e8246: f64 = (p.p693 * locals.var_iwe);
        let assign8780_e8247: f64 = (assign8780_e8243 + assign8780_e8246);
        let assign8780_e8250: f64 = (p.p694 * locals.var_iae);
        let assign8780_e8251: f64 = (assign8780_e8247 + assign8780_e8250);
        let assign8780_e8252: f64 = (locals.var_iiwecv * assign8780_e8251);
        (assign8780_e8252,)
    } else {
        (locals.var_cinr_p,)
    }
};
        locals.var_cinr_p = assign8780_e8254;
        locals.var_cinr_p_rv = 0.0;

        let assign8790_e8273: f64 = if (((param_given[695] || param_given[696]) || param_given[697]) || param_given[698]) { 1.0 } else { 0.0 };
        locals.var_guard125 = assign8790_e8273;
        locals.var_guard125_rv = 0.0;

        let (assign8800_e8293,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard125 != 0.0)) {
        let assign8800_e8281: f64 = (p.p696 * locals.var_ile);
        let assign8800_e8282: f64 = (p.p695 + assign8800_e8281);
        let assign8800_e8285: f64 = (p.p697 * locals.var_iwe);
        let assign8800_e8286: f64 = (assign8800_e8282 + assign8800_e8285);
        let assign8800_e8289: f64 = (p.p698 * locals.var_iae);
        let assign8800_e8290: f64 = (assign8800_e8286 + assign8800_e8289);
        let assign8800_e8291: f64 = (locals.var_iiwecv * assign8800_e8290);
        (assign8800_e8291,)
    } else {
        (locals.var_cinrd_p,)
    }
};
        locals.var_cinrd_p = assign8800_e8293;
        locals.var_cinrd_p_rv = 0.0;

        let assign8930_e8546: f64 = if (((param_given[723] || param_given[724]) || param_given[725]) || param_given[726]) { 1.0 } else { 0.0 };
        locals.var_guard132 = assign8930_e8546;
        locals.var_guard132_rv = 0.0;

        let (assign8940_e8564,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard132 != 0.0)) {
        let assign8940_e8553: f64 = (p.p724 * locals.var_ile);
        let assign8940_e8554: f64 = (p.p723 + assign8940_e8553);
        let assign8940_e8557: f64 = (p.p725 * locals.var_iwe);
        let assign8940_e8558: f64 = (assign8940_e8554 + assign8940_e8557);
        let assign8940_e8561: f64 = (p.p726 * locals.var_iae);
        let assign8940_e8562: f64 = (assign8940_e8558 + assign8940_e8561);
        (assign8940_e8562,)
    } else {
        (locals.var_vfbedge_p,)
    }
};
        locals.var_vfbedge_p = assign8940_e8564;
        locals.var_vfbedge_p_rv = 0.0;

        let assign8950_e8583: f64 = if (((param_given[727] || param_given[728]) || param_given[729]) || param_given[730]) { 1.0 } else { 0.0 };
        locals.var_guard133 = assign8950_e8583;
        locals.var_guard133_rv = 0.0;

        let (assign8960_e8601,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard133 != 0.0)) {
        let assign8960_e8590: f64 = (p.p728 * locals.var_ile);
        let assign8960_e8591: f64 = (p.p727 + assign8960_e8590);
        let assign8960_e8594: f64 = (p.p729 * locals.var_iwe);
        let assign8960_e8595: f64 = (assign8960_e8591 + assign8960_e8594);
        let assign8960_e8598: f64 = (p.p730 * locals.var_iae);
        let assign8960_e8599: f64 = (assign8960_e8595 + assign8960_e8598);
        (assign8960_e8599,)
    } else {
        (locals.var_stvfbedge_p,)
    }
};
        locals.var_stvfbedge_p = assign8960_e8601;
        locals.var_stvfbedge_p_rv = 0.0;

        let assign8970_e8620: f64 = if (((param_given[731] || param_given[732]) || param_given[733]) || param_given[734]) { 1.0 } else { 0.0 };
        locals.var_guard134 = assign8970_e8620;
        locals.var_guard134_rv = 0.0;

        let (assign8980_e8638,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard134 != 0.0)) {
        let assign8980_e8627: f64 = (p.p732 * locals.var_ile);
        let assign8980_e8628: f64 = (p.p731 + assign8980_e8627);
        let assign8980_e8631: f64 = (p.p733 * locals.var_iwe);
        let assign8980_e8632: f64 = (assign8980_e8628 + assign8980_e8631);
        let assign8980_e8635: f64 = (p.p734 * locals.var_iae);
        let assign8980_e8636: f64 = (assign8980_e8632 + assign8980_e8635);
        (assign8980_e8636,)
    } else {
        (locals.var_dphibedge_p,)
    }
};
        locals.var_dphibedge_p = assign8980_e8638;
        locals.var_dphibedge_p_rv = 0.0;

        let assign8990_e8657: f64 = if (((param_given[735] || param_given[736]) || param_given[737]) || param_given[738]) { 1.0 } else { 0.0 };
        locals.var_guard135 = assign8990_e8657;
        locals.var_guard135_rv = 0.0;

        let (assign9000_e8675,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard135 != 0.0)) {
        let assign9000_e8664: f64 = (p.p736 * locals.var_ile);
        let assign9000_e8665: f64 = (p.p735 + assign9000_e8664);
        let assign9000_e8668: f64 = (p.p737 * locals.var_iwe);
        let assign9000_e8669: f64 = (assign9000_e8665 + assign9000_e8668);
        let assign9000_e8672: f64 = (p.p738 * locals.var_iae);
        let assign9000_e8673: f64 = (assign9000_e8669 + assign9000_e8672);
        (assign9000_e8673,)
    } else {
        (locals.var_neffedge_p,)
    }
};
        locals.var_neffedge_p = assign9000_e8675;
        locals.var_neffedge_p_rv = 0.0;

        let assign9010_e8694: f64 = if (((param_given[739] || param_given[740]) || param_given[741]) || param_given[742]) { 1.0 } else { 0.0 };
        locals.var_guard136 = assign9010_e8694;
        locals.var_guard136_rv = 0.0;

        let (assign9020_e8712,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard136 != 0.0)) {
        let assign9020_e8701: f64 = (p.p740 * locals.var_ile);
        let assign9020_e8702: f64 = (p.p739 + assign9020_e8701);
        let assign9020_e8705: f64 = (p.p741 * locals.var_iwe);
        let assign9020_e8706: f64 = (assign9020_e8702 + assign9020_e8705);
        let assign9020_e8709: f64 = (p.p742 * locals.var_iae);
        let assign9020_e8710: f64 = (assign9020_e8706 + assign9020_e8709);
        (assign9020_e8710,)
    } else {
        (locals.var_ctedge_p,)
    }
};
        locals.var_ctedge_p = assign9020_e8712;
        locals.var_ctedge_p_rv = 0.0;

        let assign9030_e8731: f64 = if (((param_given[743] || param_given[744]) || param_given[745]) || param_given[746]) { 1.0 } else { 0.0 };
        locals.var_guard137 = assign9030_e8731;
        locals.var_guard137_rv = 0.0;

        let (assign9040_e8753,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard137 != 0.0)) {
        let assign9040_e8737: f64 = (locals.var_we_edge / locals.var_le);
        let assign9040_e8741: f64 = (p.p744 * locals.var_ile);
        let assign9040_e8742: f64 = (p.p743 + assign9040_e8741);
        let assign9040_e8745: f64 = (p.p745 * locals.var_iwe);
        let assign9040_e8746: f64 = (assign9040_e8742 + assign9040_e8745);
        let assign9040_e8749: f64 = (p.p746 * locals.var_iae);
        let assign9040_e8750: f64 = (assign9040_e8746 + assign9040_e8749);
        let assign9040_e8751: f64 = (assign9040_e8737 * assign9040_e8750);
        (assign9040_e8751,)
    } else {
        (locals.var_betnedge_p,)
    }
};
        locals.var_betnedge_p = assign9040_e8753;
        locals.var_betnedge_p_rv = 0.0;

        let assign9050_e8772: f64 = if (((param_given[747] || param_given[748]) || param_given[749]) || param_given[750]) { 1.0 } else { 0.0 };
        locals.var_guard138 = assign9050_e8772;
        locals.var_guard138_rv = 0.0;

        let (assign9060_e8790,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard138 != 0.0)) {
        let assign9060_e8779: f64 = (p.p748 * locals.var_ile);
        let assign9060_e8780: f64 = (p.p747 + assign9060_e8779);
        let assign9060_e8783: f64 = (p.p749 * locals.var_iwe);
        let assign9060_e8784: f64 = (assign9060_e8780 + assign9060_e8783);
        let assign9060_e8787: f64 = (p.p750 * locals.var_iae);
        let assign9060_e8788: f64 = (assign9060_e8784 + assign9060_e8787);
        (assign9060_e8788,)
    } else {
        (locals.var_stbetedge_p,)
    }
};
        locals.var_stbetedge_p = assign9060_e8790;
        locals.var_stbetedge_p_rv = 0.0;

        let assign9070_e8809: f64 = if (((param_given[751] || param_given[752]) || param_given[753]) || param_given[754]) { 1.0 } else { 0.0 };
        locals.var_guard139 = assign9070_e8809;
        locals.var_guard139_rv = 0.0;

        let (assign9080_e8829,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard139 != 0.0)) {
        let assign9080_e8817: f64 = (p.p752 * locals.var_ile);
        let assign9080_e8818: f64 = (p.p751 + assign9080_e8817);
        let assign9080_e8821: f64 = (p.p753 * locals.var_iwe);
        let assign9080_e8822: f64 = (assign9080_e8818 + assign9080_e8821);
        let assign9080_e8825: f64 = (p.p754 * locals.var_iae);
        let assign9080_e8826: f64 = (assign9080_e8822 + assign9080_e8825);
        let assign9080_e8827: f64 = (locals.var_ile2 * assign9080_e8826);
        (assign9080_e8827,)
    } else {
        (locals.var_psceedge_p,)
    }
};
        locals.var_psceedge_p = assign9080_e8829;
        locals.var_psceedge_p_rv = 0.0;

        let assign9090_e8848: f64 = if (((param_given[755] || param_given[756]) || param_given[757]) || param_given[758]) { 1.0 } else { 0.0 };
        locals.var_guard140 = assign9090_e8848;
        locals.var_guard140_rv = 0.0;

        let (assign9100_e8866,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard140 != 0.0)) {
        let assign9100_e8855: f64 = (p.p756 * locals.var_ile);
        let assign9100_e8856: f64 = (p.p755 + assign9100_e8855);
        let assign9100_e8859: f64 = (p.p757 * locals.var_iwe);
        let assign9100_e8860: f64 = (assign9100_e8856 + assign9100_e8859);
        let assign9100_e8863: f64 = (p.p758 * locals.var_iae);
        let assign9100_e8864: f64 = (assign9100_e8860 + assign9100_e8863);
        (assign9100_e8864,)
    } else {
        (locals.var_pscebedge_p,)
    }
};
        locals.var_pscebedge_p = assign9100_e8866;
        locals.var_pscebedge_p_rv = 0.0;

        let assign9110_e8885: f64 = if (((param_given[759] || param_given[760]) || param_given[761]) || param_given[762]) { 1.0 } else { 0.0 };
        locals.var_guard141 = assign9110_e8885;
        locals.var_guard141_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_9(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        locals: &mut StampLocals,
    ) {
        let (assign9120_e8903,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard141 != 0.0)) {
        let assign9120_e8892: f64 = (p.p760 * locals.var_ile);
        let assign9120_e8893: f64 = (p.p759 + assign9120_e8892);
        let assign9120_e8896: f64 = (p.p761 * locals.var_iwe);
        let assign9120_e8897: f64 = (assign9120_e8893 + assign9120_e8896);
        let assign9120_e8900: f64 = (p.p762 * locals.var_iae);
        let assign9120_e8901: f64 = (assign9120_e8897 + assign9120_e8900);
        (assign9120_e8901,)
    } else {
        (locals.var_pscededge_p,)
    }
};
        locals.var_pscededge_p = assign9120_e8903;
        locals.var_pscededge_p_rv = 0.0;

        let assign9130_e8922: f64 = if (((param_given[763] || param_given[764]) || param_given[765]) || param_given[766]) { 1.0 } else { 0.0 };
        locals.var_guard142 = assign9130_e8922;
        locals.var_guard142_rv = 0.0;

        let (assign9140_e8942,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard142 != 0.0)) {
        let assign9140_e8930: f64 = (p.p764 * locals.var_ile);
        let assign9140_e8931: f64 = (p.p763 + assign9140_e8930);
        let assign9140_e8934: f64 = (p.p765 * locals.var_iwe);
        let assign9140_e8935: f64 = (assign9140_e8931 + assign9140_e8934);
        let assign9140_e8938: f64 = (p.p766 * locals.var_iae);
        let assign9140_e8939: f64 = (assign9140_e8935 + assign9140_e8938);
        let assign9140_e8940: f64 = (locals.var_ile2 * assign9140_e8939);
        (assign9140_e8940,)
    } else {
        (locals.var_cfedge_p,)
    }
};
        locals.var_cfedge_p = assign9140_e8942;
        locals.var_cfedge_p_rv = 0.0;

        let assign9150_e8961: f64 = if (((param_given[771] || param_given[772]) || param_given[773]) || param_given[774]) { 1.0 } else { 0.0 };
        locals.var_guard143 = assign9150_e8961;
        locals.var_guard143_rv = 0.0;

        let (assign9160_e8979,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard143 != 0.0)) {
        let assign9160_e8968: f64 = (p.p772 * locals.var_ile);
        let assign9160_e8969: f64 = (p.p771 + assign9160_e8968);
        let assign9160_e8972: f64 = (p.p773 * locals.var_iwe);
        let assign9160_e8973: f64 = (assign9160_e8969 + assign9160_e8972);
        let assign9160_e8976: f64 = (p.p774 * locals.var_iae);
        let assign9160_e8977: f64 = (assign9160_e8973 + assign9160_e8976);
        (assign9160_e8977,)
    } else {
        (locals.var_cfdedge_p,)
    }
};
        locals.var_cfdedge_p = assign9160_e8979;
        locals.var_cfdedge_p_rv = 0.0;

        let assign9170_e8998: f64 = if (((param_given[767] || param_given[768]) || param_given[769]) || param_given[770]) { 1.0 } else { 0.0 };
        locals.var_guard144 = assign9170_e8998;
        locals.var_guard144_rv = 0.0;

        let (assign9180_e9016,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard144 != 0.0)) {
        let assign9180_e9005: f64 = (p.p768 * locals.var_ile);
        let assign9180_e9006: f64 = (p.p767 + assign9180_e9005);
        let assign9180_e9009: f64 = (p.p769 * locals.var_iwe);
        let assign9180_e9010: f64 = (assign9180_e9006 + assign9180_e9009);
        let assign9180_e9013: f64 = (p.p770 * locals.var_iae);
        let assign9180_e9014: f64 = (assign9180_e9010 + assign9180_e9013);
        (assign9180_e9014,)
    } else {
        (locals.var_cfbedge_p,)
    }
};
        locals.var_cfbedge_p = assign9180_e9016;
        locals.var_cfbedge_p_rv = 0.0;

        let assign9250_e9152: f64 = if (((param_given[787] || param_given[788]) || param_given[789]) || param_given[790]) { 1.0 } else { 0.0 };
        locals.var_guard148 = assign9250_e9152;
        locals.var_guard148_rv = 0.0;

        let (assign9260_e9170,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard148 != 0.0)) {
        let assign9260_e9159: f64 = (p.p788 * locals.var_ile);
        let assign9260_e9160: f64 = (p.p787 + assign9260_e9159);
        let assign9260_e9163: f64 = (p.p789 * locals.var_iwe);
        let assign9260_e9164: f64 = (assign9260_e9160 + assign9260_e9163);
        let assign9260_e9167: f64 = (p.p790 * locals.var_iae);
        let assign9260_e9168: f64 = (assign9260_e9164 + assign9260_e9167);
        (assign9260_e9168,)
    } else {
        (locals.var_munqs_p,)
    }
};
        locals.var_munqs_p = assign9260_e9170;
        locals.var_munqs_p_rv = 0.0;

        let (assign9270_e9174,) = {
    if (locals.var_guard41 != 0.0) {
        (0.0,)
    } else {
        (locals.var_tmpa,)
    }
};
        locals.var_tmpa = assign9270_e9174;
        locals.var_tmpa_rv = 0.0;

        let (assign9280_e9178,) = {
    if (locals.var_guard41 != 0.0) {
        (0.0,)
    } else {
        (locals.var_tmpb,)
    }
};
        locals.var_tmpb = assign9280_e9178;
        locals.var_tmpb_rv = 0.0;

        let (assign9290_e9182,) = {
    if (locals.var_guard41 != 0.0) {
        (0.0,)
    } else {
        (locals.var_loop_,)
    }
};
        locals.var_loop_ = assign9290_e9182;
        locals.var_loop__rv = 0.0;

        let (assign9300_e9186,) = {
    if (locals.var_guard41 != 0.0) {
        (p.p795,)
    } else {
        (locals.var_kvsatac_i,)
    }
};
        locals.var_kvsatac_i = assign9300_e9186;
        locals.var_kvsatac_i_rv = 0.0;

        let assign9310_e9188: f64 = if param_given[796] { 1.0 } else { 0.0 };
        let assign9310_e9190: f64 = if assign9310_e9188 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard149 = assign9310_e9190;
        locals.var_guard149_rv = 0.0;

        let (assign9320_e9196,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard149 != 0.0)) {
        (p.p796,)
    } else {
        (locals.var_kvsatac_i,)
    }
};
        locals.var_kvsatac_i = assign9320_e9196;
        locals.var_kvsatac_i_rv = 0.0;

        let assign9330_e9215: f64 = if (((locals.var_sa_i > 0.0) && (locals.var_sb_i > 0.0)) && ((locals.var_nf_i == 1.0) || ((locals.var_nf_i > 1.0) && (locals.var_sd_i > 0.0)))) { 1.0 } else { 0.0 };
        locals.var_guard150 = assign9330_e9215;
        locals.var_guard150_rv = 0.0;

        let mut assign9340_loop_guard: usize = 0;
        while {
            let assign9340_cond_e9222: f64 = (locals.var_nf_i - 0.5);
            let assign9340_cond_e9224: f64 = if (((locals.var_guard41 != 0.0) && (locals.var_guard150 != 0.0)) && (locals.var_loop_ < assign9340_cond_e9222)) { 1.0 } else { 0.0 };
            assign9340_cond_e9224 != 0.0
        } {
            assign9340_loop_guard += 1;
            assert!(assign9340_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign9340_body0_e9244,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard150 != 0.0)) {
        let assign9340_body0_e9233: f64 = (0.5 * locals.var_l_i);
        let assign9340_body0_e9234: f64 = (locals.var_sa_i + assign9340_body0_e9233);
        let assign9340_body0_e9238: f64 = (locals.var_sd_i + locals.var_l_i);
        let assign9340_body0_e9239: f64 = (locals.var_loop_ * assign9340_body0_e9238);
        let assign9340_body0_e9240: f64 = (assign9340_body0_e9234 + assign9340_body0_e9239);
        let assign9340_body0_e9241: f64 = (1.0 / assign9340_body0_e9240);
        let assign9340_body0_e9242: f64 = (locals.var_tmpa + assign9340_body0_e9241);
        (assign9340_body0_e9242,)
    } else {
        (locals.var_tmpa,)
    }
};
            locals.var_tmpa = assign9340_body0_e9244;
            locals.var_tmpa_rv = 0.0;
            let (assign9340_body1_e9264,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard150 != 0.0)) {
        let assign9340_body1_e9253: f64 = (0.5 * locals.var_l_i);
        let assign9340_body1_e9254: f64 = (locals.var_sb_i + assign9340_body1_e9253);
        let assign9340_body1_e9258: f64 = (locals.var_sd_i + locals.var_l_i);
        let assign9340_body1_e9259: f64 = (locals.var_loop_ * assign9340_body1_e9258);
        let assign9340_body1_e9260: f64 = (assign9340_body1_e9254 + assign9340_body1_e9259);
        let assign9340_body1_e9261: f64 = (1.0 / assign9340_body1_e9260);
        let assign9340_body1_e9262: f64 = (locals.var_tmpb + assign9340_body1_e9261);
        (assign9340_body1_e9262,)
    } else {
        (locals.var_tmpb,)
    }
};
            locals.var_tmpb = assign9340_body1_e9264;
            locals.var_tmpb_rv = 0.0;
            let (assign9340_body2_e9272,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard150 != 0.0)) {
        let assign9340_body2_e9270: f64 = (locals.var_loop_ + 1.0);
        (assign9340_body2_e9270,)
    } else {
        (locals.var_loop_,)
    }
};
            locals.var_loop_ = assign9340_body2_e9272;
            locals.var_loop__rv = 0.0;
        }

        let (assign9350_e9280,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard150 != 0.0)) {
        let assign9350_e9278: f64 = (locals.var_tmpa * locals.var_invnf);
        (assign9350_e9278,)
    } else {
        (locals.var_invsa,)
    }
};
        locals.var_invsa = assign9350_e9280;
        locals.var_invsa_rv = 0.0;

        let (assign9360_e9288,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard150 != 0.0)) {
        let assign9360_e9286: f64 = (locals.var_tmpb * locals.var_invnf);
        (assign9360_e9286,)
    } else {
        (locals.var_invsb,)
    }
};
        locals.var_invsb = assign9360_e9288;
        locals.var_invsb_rv = 0.0;

        let (assign9370_e9300,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard150 != 0.0)) {
        let assign9370_e9296: f64 = (0.5 * locals.var_l_i);
        let assign9370_e9297: f64 = (p.p791 + assign9370_e9296);
        let assign9370_e9298: f64 = (1.0 / assign9370_e9297);
        (assign9370_e9298,)
    } else {
        (locals.var_invsaref,)
    }
};
        locals.var_invsaref = assign9370_e9300;
        locals.var_invsaref_rv = 0.0;

        let (assign9380_e9312,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard150 != 0.0)) {
        let assign9380_e9308: f64 = (0.5 * locals.var_l_i);
        let assign9380_e9309: f64 = (p.p792 + assign9380_e9308);
        let assign9380_e9310: f64 = (1.0 / assign9380_e9309);
        (assign9380_e9310,)
    } else {
        (locals.var_invsbref,)
    }
};
        locals.var_invsbref = assign9380_e9312;
        locals.var_invsbref_rv = 0.0;

        let (assign9390_e9327,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard150 != 0.0)) {
        let assign9390_e9318: f64 = (locals.var_l_i + locals.var_dellps);
        let (assign9390_e9325,) = {
            if (assign9390_e9318 > 1e-9) {
                let assign9390_e9323: f64 = (locals.var_l_i + locals.var_dellps);
                (assign9390_e9323,)
            } else {
                (1e-9,)
            }
        };
        (assign9390_e9325,)
    } else {
        (locals.var_lx,)
    }
};
        locals.var_lx = assign9390_e9327;
        locals.var_lx_rv = 0.0;

        let (assign9400_e9346,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard150 != 0.0)) {
        let assign9400_e9333: f64 = (locals.var_w_i + locals.var_delwod);
        let assign9400_e9335: f64 = (assign9400_e9333 + p.p793);
        let (assign9400_e9344,) = {
            if (assign9400_e9335 > 1e-9) {
                let assign9400_e9340: f64 = (locals.var_w_i + locals.var_delwod);
                let assign9400_e9342: f64 = (assign9400_e9340 + p.p793);
                (assign9400_e9342,)
            } else {
                (1e-9,)
            }
        };
        (assign9400_e9344,)
    } else {
        (locals.var_wx,)
    }
};
        locals.var_wx = assign9400_e9346;
        locals.var_wx_rv = 0.0;

        let (assign9410_e9356,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard150 != 0.0)) {
        let assign9410_e9353: f64 = (locals.var_lx).powf(p.p801);
        let assign9410_e9354: f64 = (1.0 / assign9410_e9353);
        (assign9410_e9354,)
    } else {
        (locals.var_templ,)
    }
};
        locals.var_templ = assign9410_e9356;
        locals.var_templ_rv = 0.0;

        let (assign9420_e9366,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard150 != 0.0)) {
        let assign9420_e9363: f64 = (locals.var_wx).powf(p.p802);
        let assign9420_e9364: f64 = (1.0 / assign9420_e9363);
        (assign9420_e9364,)
    } else {
        (locals.var_tempw,)
    }
};
        locals.var_tempw = assign9420_e9366;
        locals.var_tempw_rv = 0.0;

        let (assign9430_e9394,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard150 != 0.0)) {
        let assign9430_e9373: f64 = (p.p798 * locals.var_templ);
        let assign9430_e9374: f64 = (1.0 + assign9430_e9373);
        let assign9430_e9377: f64 = (p.p799 * locals.var_tempw);
        let assign9430_e9378: f64 = (assign9430_e9374 + assign9430_e9377);
        let assign9430_e9381: f64 = (p.p800 * locals.var_templ);
        let assign9430_e9383: f64 = (assign9430_e9381 * locals.var_tempw);
        let assign9430_e9384: f64 = (assign9430_e9378 + assign9430_e9383);
        let assign9430_e9389: f64 = (locals.var_rta - 1.0);
        let assign9430_e9390: f64 = (p.p797 * assign9430_e9389);
        let assign9430_e9391: f64 = (1.0 + assign9430_e9390);
        let assign9430_e9392: f64 = (assign9430_e9384 * assign9430_e9391);
        (assign9430_e9392,)
    } else {
        (locals.var_kstressu0,)
    }
};
        locals.var_kstressu0 = assign9430_e9394;
        locals.var_kstressu0_rv = 0.0;

        let (assign9440_e9406,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard150 != 0.0)) {
        let assign9440_e9401: f64 = (locals.var_invsa + locals.var_invsb);
        let assign9440_e9402: f64 = (p.p794 * assign9440_e9401);
        let assign9440_e9404: f64 = (assign9440_e9402 / locals.var_kstressu0);
        (assign9440_e9404,)
    } else {
        (locals.var_rhobeta,)
    }
};
        locals.var_rhobeta = assign9440_e9406;
        locals.var_rhobeta_rv = 0.0;

        let (assign9450_e9418,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard150 != 0.0)) {
        let assign9450_e9413: f64 = (locals.var_invsaref + locals.var_invsbref);
        let assign9450_e9414: f64 = (p.p794 * assign9450_e9413);
        let assign9450_e9416: f64 = (assign9450_e9414 / locals.var_kstressu0);
        (assign9450_e9416,)
    } else {
        (locals.var_rhobetaref,)
    }
};
        locals.var_rhobetaref = assign9450_e9418;
        locals.var_rhobetaref_rv = 0.0;

        let (assign9460_e9428,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard150 != 0.0)) {
        let assign9460_e9425: f64 = (locals.var_lx).powf(p.p807);
        let assign9460_e9426: f64 = (1.0 / assign9460_e9425);
        (assign9460_e9426,)
    } else {
        (locals.var_templ,)
    }
};
        locals.var_templ = assign9460_e9428;
        locals.var_templ_rv = 0.0;

        let (assign9470_e9438,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard150 != 0.0)) {
        let assign9470_e9435: f64 = (locals.var_wx).powf(p.p808);
        let assign9470_e9436: f64 = (1.0 / assign9470_e9435);
        (assign9470_e9436,)
    } else {
        (locals.var_tempw,)
    }
};
        locals.var_tempw = assign9470_e9438;
        locals.var_tempw_rv = 0.0;

        let (assign9480_e9458,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard150 != 0.0)) {
        let assign9480_e9445: f64 = (p.p804 * locals.var_templ);
        let assign9480_e9446: f64 = (1.0 + assign9480_e9445);
        let assign9480_e9449: f64 = (p.p805 * locals.var_tempw);
        let assign9480_e9450: f64 = (assign9480_e9446 + assign9480_e9449);
        let assign9480_e9453: f64 = (p.p806 * locals.var_templ);
        let assign9480_e9455: f64 = (assign9480_e9453 * locals.var_tempw);
        let assign9480_e9456: f64 = (assign9480_e9450 + assign9480_e9455);
        (assign9480_e9456,)
    } else {
        (locals.var_kstressvth0,)
    }
};
        locals.var_kstressvth0 = assign9480_e9458;
        locals.var_kstressvth0_rv = 0.0;

        let (assign9490_e9470,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard150 != 0.0)) {
        let assign9490_e9464: f64 = (locals.var_invsa + locals.var_invsb);
        let assign9490_e9466: f64 = (assign9490_e9464 - locals.var_invsaref);
        let assign9490_e9468: f64 = (assign9490_e9466 - locals.var_invsbref);
        (assign9490_e9468,)
    } else {
        (locals.var_temp0,)
    }
};
        locals.var_temp0 = assign9490_e9470;
        locals.var_temp0_rv = 0.0;

        let (assign9500_e9482,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard150 != 0.0)) {
        let assign9500_e9476: f64 = (1.0 + locals.var_rhobeta);
        let assign9500_e9479: f64 = (1.0 + locals.var_rhobetaref);
        let assign9500_e9480: f64 = (assign9500_e9476 / assign9500_e9479);
        (assign9500_e9480,)
    } else {
        (locals.var_temp00,)
    }
};
        locals.var_temp00 = assign9500_e9482;
        locals.var_temp00_rv = 0.0;

        let (assign9510_e9490,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard150 != 0.0)) {
        let assign9510_e9488: f64 = (locals.var_betn_p * locals.var_temp00);
        (assign9510_e9488,)
    } else {
        (locals.var_betn_p,)
    }
};
        locals.var_betn_p = assign9510_e9490;
        locals.var_betn_p_rv = 0.0;

        let (assign9520_e9510,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard150 != 0.0)) {
        let assign9520_e9496: f64 = (locals.var_thesat_p * locals.var_temp00);
        let assign9520_e9500: f64 = (p.p795 * locals.var_rhobetaref);
        let assign9520_e9501: f64 = (1.0 + assign9520_e9500);
        let assign9520_e9502: f64 = (assign9520_e9496 * assign9520_e9501);
        let assign9520_e9506: f64 = (p.p795 * locals.var_rhobeta);
        let assign9520_e9507: f64 = (1.0 + assign9520_e9506);
        let assign9520_e9508: f64 = (assign9520_e9502 / assign9520_e9507);
        (assign9520_e9508,)
    } else {
        (locals.var_thesat_p,)
    }
};
        locals.var_thesat_p = assign9520_e9510;
        locals.var_thesat_p_rv = 0.0;

        let (assign9530_e9530,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard150 != 0.0)) {
        let assign9530_e9516: f64 = (locals.var_thesatac_p * locals.var_temp00);
        let assign9530_e9520: f64 = (locals.var_kvsatac_i * locals.var_rhobetaref);
        let assign9530_e9521: f64 = (1.0 + assign9530_e9520);
        let assign9530_e9522: f64 = (assign9530_e9516 * assign9530_e9521);
        let assign9530_e9526: f64 = (locals.var_kvsatac_i * locals.var_rhobeta);
        let assign9530_e9527: f64 = (1.0 + assign9530_e9526);
        let assign9530_e9528: f64 = (assign9530_e9522 / assign9530_e9527);
        (assign9530_e9528,)
    } else {
        (locals.var_thesatac_p,)
    }
};
        locals.var_thesatac_p = assign9530_e9530;
        locals.var_thesatac_p_rv = 0.0;

        let (assign9540_e9538,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard150 != 0.0)) {
        let assign9540_e9536: f64 = (locals.var_betnedge_p * locals.var_temp00);
        (assign9540_e9536,)
    } else {
        (locals.var_betnedge_p,)
    }
};
        locals.var_betnedge_p = assign9540_e9538;
        locals.var_betnedge_p_rv = 0.0;

        let (assign9550_e9548,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard150 != 0.0)) {
        let assign9550_e9544: f64 = (p.p803 * locals.var_temp0);
        let assign9550_e9546: f64 = (assign9550_e9544 / locals.var_kstressvth0);
        (assign9550_e9546,)
    } else {
        (locals.var_temp00,)
    }
};
        locals.var_temp00 = assign9550_e9548;
        locals.var_temp00_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_10(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign9560_e9556,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard150 != 0.0)) {
        let assign9560_e9554: f64 = (locals.var_vfb_p + locals.var_temp00);
        (assign9560_e9554,)
    } else {
        (locals.var_vfb_p,)
    }
};
        locals.var_vfb_p = assign9560_e9556;
        locals.var_vfb_p_rv = 0.0;

        let (assign9570_e9564,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard150 != 0.0)) {
        let assign9570_e9562: f64 = (locals.var_vfbedge_p + locals.var_temp00);
        (assign9570_e9562,)
    } else {
        (locals.var_vfbedge_p,)
    }
};
        locals.var_vfbedge_p = assign9570_e9564;
        locals.var_vfbedge_p_rv = 0.0;

        let (assign9580_e9576,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard150 != 0.0)) {
        let assign9580_e9570: f64 = (p.p809 * locals.var_temp0);
        let assign9580_e9573: f64 = (locals.var_kstressvth0).powf(p.p810);
        let assign9580_e9574: f64 = (assign9580_e9570 / assign9580_e9573);
        (assign9580_e9574,)
    } else {
        (locals.var_temp00,)
    }
};
        locals.var_temp00 = assign9580_e9576;
        locals.var_temp00_rv = 0.0;

        let (assign9590_e9584,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard150 != 0.0)) {
        let assign9590_e9582: f64 = (locals.var_cf_p + locals.var_temp00);
        (assign9590_e9582,)
    } else {
        (locals.var_cf_p,)
    }
};
        locals.var_cf_p = assign9590_e9584;
        locals.var_cf_p_rv = 0.0;

        let (assign9600_e9592,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard150 != 0.0)) {
        let assign9600_e9590: f64 = (locals.var_cfedge_p + locals.var_temp00);
        (assign9600_e9590,)
    } else {
        (locals.var_cfedge_p,)
    }
};
        locals.var_cfedge_p = assign9600_e9592;
        locals.var_cfedge_p_rv = 0.0;

        let assign9610_e9607: f64 = if ((((locals.var_sca_i > 0.0) || (locals.var_scb_i > 0.0)) || (locals.var_scc_i > 0.0)) || (locals.var_sc_i > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard151 = assign9610_e9607;
        locals.var_guard151_rv = 0.0;

        let assign9620_e9618: f64 = if (((locals.var_sca_i == 0.0) && (locals.var_scb_i == 0.0)) && (locals.var_scc_i == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard152 = assign9620_e9618;
        locals.var_guard152_rv = 0.0;

        let (assign9630_e9628,) = {
    if (((locals.var_guard41 != 0.0) && (locals.var_guard151 != 0.0)) && (locals.var_guard152 != 0.0)) {
        let assign9630_e9626: f64 = (locals.var_sc_i + locals.var_w_i);
        (assign9630_e9626,)
    } else {
        (locals.var_temp0,)
    }
};
        locals.var_temp0 = assign9630_e9628;
        locals.var_temp0_rv = 0.0;

        let (assign9640_e9638,) = {
    if (((locals.var_guard41 != 0.0) && (locals.var_guard151 != 0.0)) && (locals.var_guard152 != 0.0)) {
        let assign9640_e9636: f64 = (1.0 / p.p811);
        (assign9640_e9636,)
    } else {
        (locals.var_temp00,)
    }
};
        locals.var_temp00 = assign9640_e9638;
        locals.var_temp00_rv = 0.0;

        let (assign9650_e9652,) = {
    if (((locals.var_guard41 != 0.0) && (locals.var_guard151 != 0.0)) && (locals.var_guard152 != 0.0)) {
        let assign9650_e9646: f64 = (p.p811 * p.p811);
        let assign9650_e9649: f64 = (locals.var_sc_i * locals.var_temp0);
        let assign9650_e9650: f64 = (assign9650_e9646 / assign9650_e9649);
        (assign9650_e9650,)
    } else {
        (locals.var_sca_i,)
    }
};
        locals.var_sca_i = assign9650_e9652;
        locals.var_sca_i_rv = 0.0;

        let (assign9660_e9692,) = {
    if (((locals.var_guard41 != 0.0) && (locals.var_guard151 != 0.0)) && (locals.var_guard152 != 0.0)) {
        let assign9660_e9660: f64 = (0.1 * locals.var_sc_i);
        let assign9660_e9663: f64 = (0.01 * p.p811);
        let assign9660_e9664: f64 = (assign9660_e9660 + assign9660_e9663);
        let assign9660_e9666: f64 = (-10.0);
        let assign9660_e9668: f64 = (assign9660_e9666 * locals.var_sc_i);
        let assign9660_e9670: f64 = (assign9660_e9668 * locals.var_temp00);
        let assign9660_e9671: f64 = (assign9660_e9670).exp();
        let assign9660_e9672: f64 = (assign9660_e9664 * assign9660_e9671);
        let assign9660_e9675: f64 = (0.1 * locals.var_temp0);
        let assign9660_e9678: f64 = (0.01 * p.p811);
        let assign9660_e9679: f64 = (assign9660_e9675 + assign9660_e9678);
        let assign9660_e9681: f64 = (-10.0);
        let assign9660_e9683: f64 = (assign9660_e9681 * locals.var_temp0);
        let assign9660_e9685: f64 = (assign9660_e9683 * locals.var_temp00);
        let assign9660_e9686: f64 = (assign9660_e9685).exp();
        let assign9660_e9687: f64 = (assign9660_e9679 * assign9660_e9686);
        let assign9660_e9688: f64 = (assign9660_e9672 - assign9660_e9687);
        let assign9660_e9690: f64 = (assign9660_e9688 / locals.var_w_i);
        (assign9660_e9690,)
    } else {
        (locals.var_scb_i,)
    }
};
        locals.var_scb_i = assign9660_e9692;
        locals.var_scb_i_rv = 0.0;

        let (assign9670_e9732,) = {
    if (((locals.var_guard41 != 0.0) && (locals.var_guard151 != 0.0)) && (locals.var_guard152 != 0.0)) {
        let assign9670_e9700: f64 = (0.05 * locals.var_sc_i);
        let assign9670_e9703: f64 = (0.0025 * p.p811);
        let assign9670_e9704: f64 = (assign9670_e9700 + assign9670_e9703);
        let assign9670_e9706: f64 = (-20.0);
        let assign9670_e9708: f64 = (assign9670_e9706 * locals.var_sc_i);
        let assign9670_e9710: f64 = (assign9670_e9708 * locals.var_temp00);
        let assign9670_e9711: f64 = (assign9670_e9710).exp();
        let assign9670_e9712: f64 = (assign9670_e9704 * assign9670_e9711);
        let assign9670_e9715: f64 = (0.05 * locals.var_temp0);
        let assign9670_e9718: f64 = (0.0025 * p.p811);
        let assign9670_e9719: f64 = (assign9670_e9715 + assign9670_e9718);
        let assign9670_e9721: f64 = (-20.0);
        let assign9670_e9723: f64 = (assign9670_e9721 * locals.var_temp0);
        let assign9670_e9725: f64 = (assign9670_e9723 * locals.var_temp00);
        let assign9670_e9726: f64 = (assign9670_e9725).exp();
        let assign9670_e9727: f64 = (assign9670_e9719 * assign9670_e9726);
        let assign9670_e9728: f64 = (assign9670_e9712 - assign9670_e9727);
        let assign9670_e9730: f64 = (assign9670_e9728 / locals.var_w_i);
        (assign9670_e9730,)
    } else {
        (locals.var_scc_i,)
    }
};
        locals.var_scc_i = assign9670_e9732;
        locals.var_scc_i_rv = 0.0;

        let (assign9680_e9746,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard151 != 0.0)) {
        let assign9680_e9739: f64 = (p.p812 * locals.var_scb_i);
        let assign9680_e9740: f64 = (locals.var_sca_i + assign9680_e9739);
        let assign9680_e9743: f64 = (p.p813 * locals.var_scc_i);
        let assign9680_e9744: f64 = (assign9680_e9740 + assign9680_e9743);
        (assign9680_e9744,)
    } else {
        (locals.var_temp0,)
    }
};
        locals.var_temp0 = assign9680_e9746;
        locals.var_temp0_rv = 0.0;

        let (assign9690_e9756,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard151 != 0.0)) {
        let assign9690_e9753: f64 = (locals.var_kvthowe * locals.var_temp0);
        let assign9690_e9754: f64 = (locals.var_vfb_p + assign9690_e9753);
        (assign9690_e9754,)
    } else {
        (locals.var_vfb_p,)
    }
};
        locals.var_vfb_p = assign9690_e9756;
        locals.var_vfb_p_rv = 0.0;

        let (assign9700_e9768,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard151 != 0.0)) {
        let assign9700_e9764: f64 = (locals.var_kuowe * locals.var_temp0);
        let assign9700_e9765: f64 = (1.0 + assign9700_e9764);
        let assign9700_e9766: f64 = (locals.var_betn_p * assign9700_e9765);
        (assign9700_e9766,)
    } else {
        (locals.var_betn_p,)
    }
};
        locals.var_betn_p = assign9700_e9768;
        locals.var_betn_p_rv = 0.0;

        let (assign9710_e9778,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard151 != 0.0)) {
        let assign9710_e9775: f64 = (locals.var_kvthowe * locals.var_temp0);
        let assign9710_e9776: f64 = (locals.var_vfbedge_p + assign9710_e9775);
        (assign9710_e9776,)
    } else {
        (locals.var_vfbedge_p,)
    }
};
        locals.var_vfbedge_p = assign9710_e9778;
        locals.var_vfbedge_p_rv = 0.0;

        let (assign9720_e9790,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard151 != 0.0)) {
        let assign9720_e9786: f64 = (locals.var_kuowe * locals.var_temp0);
        let assign9720_e9787: f64 = (1.0 + assign9720_e9786);
        let assign9720_e9788: f64 = (locals.var_betnedge_p * assign9720_e9787);
        (assign9720_e9788,)
    } else {
        (locals.var_betnedge_p,)
    }
};
        locals.var_betnedge_p = assign9720_e9790;
        locals.var_betnedge_p_rv = 0.0;

        locals.var_vfb_i = locals.var_vfb_p;
        locals.var_vfb_i_rv = 0.0;

        locals.var_stvfb_i = locals.var_stvfb_p;
        locals.var_stvfb_i_rv = 0.0;

        locals.var_st2vfb_i = locals.var_st2vfb_p;
        locals.var_st2vfb_i_rv = 0.0;

        locals.var_tox_i = locals.var_tox_p;
        locals.var_tox_i_rv = 0.0;

        locals.var_epsrox_i = locals.var_epsrox_p;
        locals.var_epsrox_i_rv = 0.0;

        let (assign9780_e9806,) = {
    if (locals.var_neff_p > 1e20) {
        let (assign9780_e9804,) = {
            if (locals.var_neff_p < 1e26) {
                (locals.var_neff_p,)
            } else {
                (1e26,)
            }
        };
        (assign9780_e9804,)
    } else {
        (1e20,)
    }
};
        locals.var_neff_i = assign9780_e9806;
        locals.var_neff_i_rv = 0.0;

        let (assign9790_e9812,) = {
    if (locals.var_gfacnud_p > 0.01) {
        (locals.var_gfacnud_p,)
    } else {
        (0.01,)
    }
};
        locals.var_gfacnud_i = assign9790_e9812;
        locals.var_gfacnud_i_rv = 0.0;

        let (assign9800_e9818,) = {
    if (locals.var_vsbnud_p > 0.0) {
        (locals.var_vsbnud_p,)
    } else {
        (0.0,)
    }
};
        locals.var_vsbnud_i = assign9800_e9818;
        locals.var_vsbnud_i_rv = 0.0;

        locals.var_dvsbnud_i = locals.var_dvsbnud_p;
        locals.var_dvsbnud_i_rv = 0.0;

        locals.var_dphib_i = locals.var_dphib_p;
        locals.var_dphib_i_rv = 0.0;

        let (assign9830_e9826,) = {
    if (locals.var_np_p > 0.0) {
        (locals.var_np_p,)
    } else {
        (0.0,)
    }
};
        locals.var_np_i = assign9830_e9826;
        locals.var_np_i_rv = 0.0;

        locals.var_toxov_i = locals.var_toxov_p;
        locals.var_toxov_i_rv = 0.0;

        locals.var_toxovd_i = locals.var_toxovd_p;
        locals.var_toxovd_i_rv = 0.0;

        let (assign9860_e9839,) = {
    if (locals.var_nov_p > 1e23) {
        let (assign9860_e9837,) = {
            if (locals.var_nov_p < 1e27) {
                (locals.var_nov_p,)
            } else {
                (1e27,)
            }
        };
        (assign9860_e9837,)
    } else {
        (1e23,)
    }
};
        locals.var_nov_i = assign9860_e9839;
        locals.var_nov_i_rv = 0.0;

        let (assign9870_e9850,) = {
    if (locals.var_novd_p > 1e23) {
        let (assign9870_e9848,) = {
            if (locals.var_novd_p < 1e27) {
                (locals.var_novd_p,)
            } else {
                (1e27,)
            }
        };
        (assign9870_e9848,)
    } else {
        (1e23,)
    }
};
        locals.var_novd_i = assign9870_e9850;
        locals.var_novd_i_rv = 0.0;

        let (assign9880_e9856,) = {
    if (locals.var_ct_p > 0.0) {
        (locals.var_ct_p,)
    } else {
        (0.0,)
    }
};
        locals.var_ct_i = assign9880_e9856;
        locals.var_ct_i_rv = 0.0;

        let (assign9890_e9867,) = {
    if (locals.var_ctb_p > 0.0) {
        let (assign9890_e9865,) = {
            if (locals.var_ctb_p < 0.5) {
                (locals.var_ctb_p,)
            } else {
                (0.5,)
            }
        };
        (assign9890_e9865,)
    } else {
        (0.0,)
    }
};
        locals.var_ctb_i = assign9890_e9867;
        locals.var_ctb_i_rv = 0.0;

        let (assign9900_e9878,) = {
    if (locals.var_ctg_p > 0.0) {
        let (assign9900_e9876,) = {
            if (locals.var_ctg_p < 1.0) {
                (locals.var_ctg_p,)
            } else {
                (1.0,)
            }
        };
        (assign9900_e9876,)
    } else {
        (0.0,)
    }
};
        locals.var_ctg_i = assign9900_e9878;
        locals.var_ctg_i_rv = 0.0;

        locals.var_stct_i = locals.var_stct_p;
        locals.var_stct_i_rv = 0.0;

        let (assign9920_e9885,) = {
    if (locals.var_cf_p > 0.0) {
        (locals.var_cf_p,)
    } else {
        (0.0,)
    }
};
        locals.var_cf_i = assign9920_e9885;
        locals.var_cf_i_rv = 0.0;

        let (assign9930_e9896,) = {
    if (locals.var_cfb_p > 0.0) {
        let (assign9930_e9894,) = {
            if (locals.var_cfb_p < 1.0) {
                (locals.var_cfb_p,)
            } else {
                (1.0,)
            }
        };
        (assign9930_e9894,)
    } else {
        (0.0,)
    }
};
        locals.var_cfb_i = assign9930_e9896;
        locals.var_cfb_i_rv = 0.0;

        let (assign9940_e9902,) = {
    if (locals.var_cfd_p > 0.0) {
        (locals.var_cfd_p,)
    } else {
        (0.0,)
    }
};
        locals.var_cfd_i = assign9940_e9902;
        locals.var_cfd_i_rv = 0.0;

        let (assign9950_e9908,) = {
    if (locals.var_psce_p > 0.0) {
        (locals.var_psce_p,)
    } else {
        (0.0,)
    }
};
        locals.var_psce_i = assign9950_e9908;
        locals.var_psce_i_rv = 0.0;

        let (assign9960_e9919,) = {
    if (locals.var_psceb_p > 0.0) {
        let (assign9960_e9917,) = {
            if (locals.var_psceb_p < 1.0) {
                (locals.var_psceb_p,)
            } else {
                (1.0,)
            }
        };
        (assign9960_e9917,)
    } else {
        (0.0,)
    }
};
        locals.var_psceb_i = assign9960_e9919;
        locals.var_psceb_i_rv = 0.0;

        let (assign9970_e9925,) = {
    if (locals.var_psced_p > 0.0) {
        (locals.var_psced_p,)
    } else {
        (0.0,)
    }
};
        locals.var_psced_i = assign9970_e9925;
        locals.var_psced_i_rv = 0.0;

        let (assign9980_e9931,) = {
    if (locals.var_betn_p > 0.0) {
        (locals.var_betn_p,)
    } else {
        (0.0,)
    }
};
        locals.var_betn_i = assign9980_e9931;
        locals.var_betn_i_rv = 0.0;

        locals.var_stbet_i = locals.var_stbet_p;
        locals.var_stbet_i_rv = 0.0;

        let (assign10000_e9938,) = {
    if (locals.var_mue_p > 0.0) {
        (locals.var_mue_p,)
    } else {
        (0.0,)
    }
};
        locals.var_mue_i = assign10000_e9938;
        locals.var_mue_i_rv = 0.0;

        locals.var_stmue_i = locals.var_stmue_p;
        locals.var_stmue_i_rv = 0.0;

        let (assign10020_e9945,) = {
    if (locals.var_themu_p > 0.0) {
        (locals.var_themu_p,)
    } else {
        (0.0,)
    }
};
        locals.var_themu_i = assign10020_e9945;
        locals.var_themu_i_rv = 0.0;

        locals.var_stthemu_i = locals.var_stthemu_p;
        locals.var_stthemu_i_rv = 0.0;

        let (assign10040_e9952,) = {
    if (locals.var_cs_p > 0.0) {
        (locals.var_cs_p,)
    } else {
        (0.0,)
    }
};
        locals.var_cs_i = assign10040_e9952;
        locals.var_cs_i_rv = 0.0;

        locals.var_stcs_i = locals.var_stcs_p;
        locals.var_stcs_i_rv = 0.0;

        let (assign10060_e9959,) = {
    if (locals.var_thecs_p > 0.0) {
        (locals.var_thecs_p,)
    } else {
        (0.0,)
    }
};
        locals.var_thecs_i = assign10060_e9959;
        locals.var_thecs_i_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_11(
        locals: &mut StampLocals,
    ) {
        locals.var_stthecs_i = locals.var_stthecs_p;
        locals.var_stthecs_i_rv = 0.0;

        let (assign10080_e9966,) = {
    if (locals.var_xcor_p > 0.0) {
        (locals.var_xcor_p,)
    } else {
        (0.0,)
    }
};
        locals.var_xcor_i = assign10080_e9966;
        locals.var_xcor_i_rv = 0.0;

        locals.var_stxcor_i = locals.var_stxcor_p;
        locals.var_stxcor_i_rv = 0.0;

        locals.var_feta_i = locals.var_feta_p;
        locals.var_feta_i_rv = 0.0;

        let (assign10110_e9974,) = {
    if (locals.var_rs_p > 0.0) {
        (locals.var_rs_p,)
    } else {
        (0.0,)
    }
};
        locals.var_rs_i = assign10110_e9974;
        locals.var_rs_i_rv = 0.0;

        locals.var_strs_i = locals.var_strs_p;
        locals.var_strs_i_rv = 0.0;

        let assign10130_e9978: f64 = (-0.5);
        let (assign10130_e9988,) = {
    if (locals.var_rsb_p > assign10130_e9978) {
        let (assign10130_e9985,) = {
            if (locals.var_rsb_p < 1.0) {
                (locals.var_rsb_p,)
            } else {
                (1.0,)
            }
        };
        (assign10130_e9985,)
    } else {
        let assign10130_e9987: f64 = (-0.5);
        (assign10130_e9987,)
    }
};
        locals.var_rsb_i = assign10130_e9988;
        locals.var_rsb_i_rv = 0.0;

        let assign10140_e9991: f64 = (-0.5);
        let (assign10140_e9996,) = {
    if (locals.var_rsg_p > assign10140_e9991) {
        (locals.var_rsg_p,)
    } else {
        let assign10140_e9995: f64 = (-0.5);
        (assign10140_e9995,)
    }
};
        locals.var_rsg_i = assign10140_e9996;
        locals.var_rsg_i_rv = 0.0;

        let (assign10150_e10002,) = {
    if (locals.var_thesat_p > 0.0) {
        (locals.var_thesat_p,)
    } else {
        (0.0,)
    }
};
        locals.var_thesat_i = assign10150_e10002;
        locals.var_thesat_i_rv = 0.0;

        locals.var_stthesat_i = locals.var_stthesat_p;
        locals.var_stthesat_i_rv = 0.0;

        let assign10170_e10006: f64 = (-0.5);
        let (assign10170_e10016,) = {
    if (locals.var_thesatb_p > assign10170_e10006) {
        let (assign10170_e10013,) = {
            if (locals.var_thesatb_p < 1.0) {
                (locals.var_thesatb_p,)
            } else {
                (1.0,)
            }
        };
        (assign10170_e10013,)
    } else {
        let assign10170_e10015: f64 = (-0.5);
        (assign10170_e10015,)
    }
};
        locals.var_thesatb_i = assign10170_e10016;
        locals.var_thesatb_i_rv = 0.0;

        let assign10180_e10019: f64 = (-0.5);
        let (assign10180_e10024,) = {
    if (locals.var_thesatg_p > assign10180_e10019) {
        (locals.var_thesatg_p,)
    } else {
        let assign10180_e10023: f64 = (-0.5);
        (assign10180_e10023,)
    }
};
        locals.var_thesatg_i = assign10180_e10024;
        locals.var_thesatg_i_rv = 0.0;

        let (assign10190_e10030,) = {
    if (locals.var_thesatt_p > 0.01) {
        (locals.var_thesatt_p,)
    } else {
        (0.01,)
    }
};
        locals.var_thesatt_i = assign10190_e10030;
        locals.var_thesatt_i_rv = 0.0;

        let (assign10200_e10036,) = {
    if (locals.var_ax_p > 2.0) {
        (locals.var_ax_p,)
    } else {
        (2.0,)
    }
};
        locals.var_ax_i = assign10200_e10036;
        locals.var_ax_i_rv = 0.0;

        let (assign10210_e10042,) = {
    if (locals.var_alp_p > 0.0) {
        (locals.var_alp_p,)
    } else {
        (0.0,)
    }
};
        locals.var_alp_i = assign10210_e10042;
        locals.var_alp_i_rv = 0.0;

        let (assign10220_e10048,) = {
    if (locals.var_alp1_p > 0.0) {
        (locals.var_alp1_p,)
    } else {
        (0.0,)
    }
};
        locals.var_alp1_i = assign10220_e10048;
        locals.var_alp1_i_rv = 0.0;

        let (assign10230_e10054,) = {
    if (locals.var_alp2_p > 0.0) {
        (locals.var_alp2_p,)
    } else {
        (0.0,)
    }
};
        locals.var_alp2_i = assign10230_e10054;
        locals.var_alp2_i_rv = 0.0;

        locals.var_vp_i = locals.var_vp_p;
        locals.var_vp_i_rv = 0.0;

        let (assign10250_e10061,) = {
    if (locals.var_a1_p > 0.0) {
        (locals.var_a1_p,)
    } else {
        (0.0,)
    }
};
        locals.var_a1_i = assign10250_e10061;
        locals.var_a1_i_rv = 0.0;

        locals.var_a2_i = locals.var_a2_p;
        locals.var_a2_i_rv = 0.0;

        locals.var_sta2_i = locals.var_sta2_p;
        locals.var_sta2_i_rv = 0.0;

        let (assign10280_e10069,) = {
    if (locals.var_a3_p > 0.0) {
        (locals.var_a3_p,)
    } else {
        (0.0,)
    }
};
        locals.var_a3_i = assign10280_e10069;
        locals.var_a3_i_rv = 0.0;

        let (assign10290_e10075,) = {
    if (locals.var_a4_p > 0.0) {
        (locals.var_a4_p,)
    } else {
        (0.0,)
    }
};
        locals.var_a4_i = assign10290_e10075;
        locals.var_a4_i_rv = 0.0;

        let (assign10300_e10081,) = {
    if (locals.var_imaxii_p > 1e-12) {
        (locals.var_imaxii_p,)
    } else {
        (1e-12,)
    }
};
        locals.var_imaxii_i = assign10300_e10081;
        locals.var_imaxii_i_rv = 0.0;

        locals.var_gco_i = locals.var_gco_p;
        locals.var_gco_i_rv = 0.0;

        let (assign10320_e10088,) = {
    if (locals.var_iginv_p > 0.0) {
        (locals.var_iginv_p,)
    } else {
        (0.0,)
    }
};
        locals.var_iginv_i = assign10320_e10088;
        locals.var_iginv_i_rv = 0.0;

        let (assign10330_e10094,) = {
    if (locals.var_igov_p > 0.0) {
        (locals.var_igov_p,)
    } else {
        (0.0,)
    }
};
        locals.var_igov_i = assign10330_e10094;
        locals.var_igov_i_rv = 0.0;

        let (assign10340_e10100,) = {
    if (locals.var_igovd_p > 0.0) {
        (locals.var_igovd_p,)
    } else {
        (0.0,)
    }
};
        locals.var_igovd_i = assign10340_e10100;
        locals.var_igovd_i_rv = 0.0;

        locals.var_stig_i = locals.var_stig_p;
        locals.var_stig_i_rv = 0.0;

        locals.var_gc2_i = locals.var_gc2_p;
        locals.var_gc2_i_rv = 0.0;

        locals.var_gc3_i = locals.var_gc3_p;
        locals.var_gc3_i_rv = 0.0;

        locals.var_gc2ov_i = locals.var_gc2ov_p;
        locals.var_gc2ov_i_rv = 0.0;

        locals.var_gc3ov_i = locals.var_gc3ov_p;
        locals.var_gc3ov_i_rv = 0.0;

        locals.var_gc2ovd_i = locals.var_gc2ovd_p;
        locals.var_gc2ovd_i_rv = 0.0;

        locals.var_gc3ovd_i = locals.var_gc3ovd_p;
        locals.var_gc3ovd_i_rv = 0.0;

        locals.var_chib_i = locals.var_chib_p;
        locals.var_chib_i_rv = 0.0;

        let (assign10430_e10114,) = {
    if (locals.var_agidl_p > 0.0) {
        (locals.var_agidl_p,)
    } else {
        (0.0,)
    }
};
        locals.var_agidl_i = assign10430_e10114;
        locals.var_agidl_i_rv = 0.0;

        let (assign10440_e10120,) = {
    if (locals.var_agidld_p > 0.0) {
        (locals.var_agidld_p,)
    } else {
        (0.0,)
    }
};
        locals.var_agidld_i = assign10440_e10120;
        locals.var_agidld_i_rv = 0.0;

        locals.var_bgidl_i = locals.var_bgidl_p;
        locals.var_bgidl_i_rv = 0.0;

        locals.var_bgidld_i = locals.var_bgidld_p;
        locals.var_bgidld_i_rv = 0.0;

        locals.var_stbgidl_i = locals.var_stbgidl_p;
        locals.var_stbgidl_i_rv = 0.0;

        locals.var_stbgidld_i = locals.var_stbgidld_p;
        locals.var_stbgidld_i_rv = 0.0;

        locals.var_cgidl_i = locals.var_cgidl_p;
        locals.var_cgidl_i_rv = 0.0;

        locals.var_cgidld_i = locals.var_cgidld_p;
        locals.var_cgidld_i_rv = 0.0;

        let (assign10510_e10132,) = {
    if (locals.var_cox_p > 0.0) {
        (locals.var_cox_p,)
    } else {
        (0.0,)
    }
};
        locals.var_cox_i = assign10510_e10132;
        locals.var_cox_i_rv = 0.0;

        locals.var_delvtac_i = locals.var_delvtac_p;
        locals.var_delvtac_i_rv = 0.0;

        let (assign10530_e10139,) = {
    if (locals.var_facneffac_p > 0.0) {
        (locals.var_facneffac_p,)
    } else {
        (0.0,)
    }
};
        locals.var_facneffac_i = assign10530_e10139;
        locals.var_facneffac_i_rv = 0.0;

        let (assign10540_e10145,) = {
    if (locals.var_thesatac_p > 0.0) {
        (locals.var_thesatac_p,)
    } else {
        (0.0,)
    }
};
        locals.var_thesatac_i = assign10540_e10145;
        locals.var_thesatac_i_rv = 0.0;

        let (assign10550_e10151,) = {
    if (locals.var_axac_p > 2.0) {
        (locals.var_axac_p,)
    } else {
        (2.0,)
    }
};
        locals.var_axac_i = assign10550_e10151;
        locals.var_axac_i_rv = 0.0;

        locals.var_alpac_i = locals.var_alpac_p;
        locals.var_alpac_i_rv = 0.0;

        let (assign10570_e10158,) = {
    if (locals.var_alp1ac_p > 0.0) {
        (locals.var_alp1ac_p,)
    } else {
        (0.0,)
    }
};
        locals.var_alp1ac_i = assign10570_e10158;
        locals.var_alp1ac_i_rv = 0.0;

        let (assign10580_e10164,) = {
    if (locals.var_cgov_p > 0.0) {
        (locals.var_cgov_p,)
    } else {
        (0.0,)
    }
};
        locals.var_cgov_i = assign10580_e10164;
        locals.var_cgov_i_rv = 0.0;

        let (assign10590_e10170,) = {
    if (locals.var_cgovd_p > 0.0) {
        (locals.var_cgovd_p,)
    } else {
        (0.0,)
    }
};
        locals.var_cgovd_i = assign10590_e10170;
        locals.var_cgovd_i_rv = 0.0;

        locals.var_fcgovacc_i = locals.var_fcgovacc_p;
        locals.var_fcgovacc_i_rv = 0.0;

        locals.var_fcgovaccd_i = locals.var_fcgovaccd_p;
        locals.var_fcgovaccd_i_rv = 0.0;

        locals.var_cgovaccg_i = locals.var_cgovaccg_p;
        locals.var_cgovaccg_i_rv = 0.0;

        let (assign10630_e10179,) = {
    if (locals.var_cgbov_p > 0.0) {
        (locals.var_cgbov_p,)
    } else {
        (0.0,)
    }
};
        locals.var_cgbov_i = assign10630_e10179;
        locals.var_cgbov_i_rv = 0.0;

        let (assign10640_e10185,) = {
    if (locals.var_cinr_p > 0.0) {
        (locals.var_cinr_p,)
    } else {
        (0.0,)
    }
};
        locals.var_cinr_i = assign10640_e10185;
        locals.var_cinr_i_rv = 0.0;

        let (assign10650_e10191,) = {
    if (locals.var_cinrd_p > 0.0) {
        (locals.var_cinrd_p,)
    } else {
        (0.0,)
    }
};
        locals.var_cinrd_i = assign10650_e10191;
        locals.var_cinrd_i_rv = 0.0;

        locals.var_dvfbinr_i = locals.var_dvfbinr_p;
        locals.var_dvfbinr_i_rv = 0.0;

        locals.var_fcinrdep_i = locals.var_fcinrdep_p;
        locals.var_fcinrdep_i_rv = 0.0;

        locals.var_fcinracc_i = locals.var_fcinracc_p;
        locals.var_fcinracc_i_rv = 0.0;

        locals.var_axinr_i = locals.var_axinr_p;
        locals.var_axinr_i_rv = 0.0;

        locals.var_fnt_i = locals.var_fnt_p;
        locals.var_fnt_i_rv = 0.0;

        locals.var_vfbedge_i = locals.var_vfbedge_p;
        locals.var_vfbedge_i_rv = 0.0;

        locals.var_stvfbedge_i = locals.var_stvfbedge_p;
        locals.var_stvfbedge_i_rv = 0.0;

        locals.var_dphibedge_i = locals.var_dphibedge_p;
        locals.var_dphibedge_i_rv = 0.0;

        let (assign10810_e10247,) = {
    if (locals.var_neffedge_p > 1e20) {
        let (assign10810_e10245,) = {
            if (locals.var_neffedge_p < 1e26) {
                (locals.var_neffedge_p,)
            } else {
                (1e26,)
            }
        };
        (assign10810_e10245,)
    } else {
        (1e20,)
    }
};
        locals.var_neffedge_i = assign10810_e10247;
        locals.var_neffedge_i_rv = 0.0;

        let (assign10820_e10253,) = {
    if (locals.var_ctedge_p > 0.0) {
        (locals.var_ctedge_p,)
    } else {
        (0.0,)
    }
};
        locals.var_ctedge_i = assign10820_e10253;
        locals.var_ctedge_i_rv = 0.0;

        let (assign10830_e10259,) = {
    if (locals.var_betnedge_p > 0.0) {
        (locals.var_betnedge_p,)
    } else {
        (0.0,)
    }
};
        locals.var_betnedge_i = assign10830_e10259;
        locals.var_betnedge_i_rv = 0.0;

        locals.var_stbetedge_i = locals.var_stbetedge_p;
        locals.var_stbetedge_i_rv = 0.0;

        let (assign10850_e10266,) = {
    if (locals.var_psceedge_p > 0.0) {
        (locals.var_psceedge_p,)
    } else {
        (0.0,)
    }
};
        locals.var_psceedge_i = assign10850_e10266;
        locals.var_psceedge_i_rv = 0.0;

        let (assign10860_e10277,) = {
    if (locals.var_pscebedge_p > 0.0) {
        let (assign10860_e10275,) = {
            if (locals.var_pscebedge_p < 1.0) {
                (locals.var_pscebedge_p,)
            } else {
                (1.0,)
            }
        };
        (assign10860_e10275,)
    } else {
        (0.0,)
    }
};
        locals.var_pscebedge_i = assign10860_e10277;
        locals.var_pscebedge_i_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_12(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign10870_e10283,) = {
    if (locals.var_pscededge_p > 0.0) {
        (locals.var_pscededge_p,)
    } else {
        (0.0,)
    }
};
        locals.var_pscededge_i = assign10870_e10283;
        locals.var_pscededge_i_rv = 0.0;

        let (assign10880_e10289,) = {
    if (locals.var_cfedge_p > 0.0) {
        (locals.var_cfedge_p,)
    } else {
        (0.0,)
    }
};
        locals.var_cfedge_i = assign10880_e10289;
        locals.var_cfedge_i_rv = 0.0;

        let (assign10890_e10300,) = {
    if (locals.var_cfbedge_p > 0.0) {
        let (assign10890_e10298,) = {
            if (locals.var_cfbedge_p < 1.0) {
                (locals.var_cfbedge_p,)
            } else {
                (1.0,)
            }
        };
        (assign10890_e10298,)
    } else {
        (0.0,)
    }
};
        locals.var_cfbedge_i = assign10890_e10300;
        locals.var_cfbedge_i_rv = 0.0;

        let (assign10900_e10306,) = {
    if (locals.var_cfdedge_p > 0.0) {
        (locals.var_cfdedge_p,)
    } else {
        (0.0,)
    }
};
        locals.var_cfdedge_i = assign10900_e10306;
        locals.var_cfdedge_i_rv = 0.0;

        let assign11030_e10341: f64 = (p.p31 * locals.var_nf_i);
        let (assign11030_e10348,) = {
    if (assign11030_e10341 > 0.0) {
        let assign11030_e10346: f64 = (p.p31 * locals.var_nf_i);
        (assign11030_e10346,)
    } else {
        (0.0,)
    }
};
        locals.var_mult_inst = assign11030_e10348;
        locals.var_mult_inst_rv = 0.0;

        locals.var_factuo_i = p.p16;
        locals.var_factuo_i_rv = 0.0;

        locals.var_delvto_i = p.p15;
        locals.var_delvto_i_rv = 0.0;

        locals.var_factuoedge_i = p.p18;
        locals.var_factuoedge_i_rv = 0.0;

        locals.var_delvtoedge_i = p.p17;
        locals.var_delvtoedge_i_rv = 0.0;

        let (assign11080_e10358,) = {
    if (locals.var_munqs_p > 0.0) {
        (locals.var_munqs_p,)
    } else {
        (0.0,)
    }
};
        locals.var_munqs_i = assign11080_e10358;
        locals.var_munqs_i_rv = 0.0;

        let assign11090_e10361: f64 = if p.p44 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard153 = assign11090_e10361;
        locals.var_guard153_rv = 0.0;

        let (assign11100_e10365,) = {
    if (locals.var_guard153 != 0.0) {
        (locals.var_toxov_i,)
    } else {
        (locals.var_toxovd_i,)
    }
};
        locals.var_toxovd_i = assign11100_e10365;
        locals.var_toxovd_i_rv = 0.0;

        let (assign11110_e10369,) = {
    if (locals.var_guard153 != 0.0) {
        (locals.var_nov_i,)
    } else {
        (locals.var_novd_i,)
    }
};
        locals.var_novd_i = assign11110_e10369;
        locals.var_novd_i_rv = 0.0;

        let (assign11120_e10373,) = {
    if (locals.var_guard153 != 0.0) {
        (locals.var_agidl_i,)
    } else {
        (locals.var_agidld_i,)
    }
};
        locals.var_agidld_i = assign11120_e10373;
        locals.var_agidld_i_rv = 0.0;

        let (assign11130_e10377,) = {
    if (locals.var_guard153 != 0.0) {
        (locals.var_bgidl_i,)
    } else {
        (locals.var_bgidld_i,)
    }
};
        locals.var_bgidld_i = assign11130_e10377;
        locals.var_bgidld_i_rv = 0.0;

        let (assign11140_e10381,) = {
    if (locals.var_guard153 != 0.0) {
        (locals.var_stbgidl_i,)
    } else {
        (locals.var_stbgidld_i,)
    }
};
        locals.var_stbgidld_i = assign11140_e10381;
        locals.var_stbgidld_i_rv = 0.0;

        let (assign11150_e10385,) = {
    if (locals.var_guard153 != 0.0) {
        (locals.var_cgidl_i,)
    } else {
        (locals.var_cgidld_i,)
    }
};
        locals.var_cgidld_i = assign11150_e10385;
        locals.var_cgidld_i_rv = 0.0;

        let (assign11160_e10389,) = {
    if (locals.var_guard153 != 0.0) {
        (locals.var_igov_i,)
    } else {
        (locals.var_igovd_i,)
    }
};
        locals.var_igovd_i = assign11160_e10389;
        locals.var_igovd_i_rv = 0.0;

        let (assign11170_e10393,) = {
    if (locals.var_guard153 != 0.0) {
        (locals.var_gc2ov_i,)
    } else {
        (locals.var_gc2ovd_i,)
    }
};
        locals.var_gc2ovd_i = assign11170_e10393;
        locals.var_gc2ovd_i_rv = 0.0;

        let (assign11180_e10397,) = {
    if (locals.var_guard153 != 0.0) {
        (locals.var_gc3ov_i,)
    } else {
        (locals.var_gc3ovd_i,)
    }
};
        locals.var_gc3ovd_i = assign11180_e10397;
        locals.var_gc3ovd_i_rv = 0.0;

        let (assign11190_e10401,) = {
    if (locals.var_guard153 != 0.0) {
        (locals.var_cgov_i,)
    } else {
        (locals.var_cgovd_i,)
    }
};
        locals.var_cgovd_i = assign11190_e10401;
        locals.var_cgovd_i_rv = 0.0;

        let (assign11200_e10405,) = {
    if (locals.var_guard153 != 0.0) {
        (locals.var_fcgovacc_i,)
    } else {
        (locals.var_fcgovaccd_i,)
    }
};
        locals.var_fcgovaccd_i = assign11200_e10405;
        locals.var_fcgovaccd_i_rv = 0.0;

        let (assign11210_e10409,) = {
    if (locals.var_guard153 != 0.0) {
        (locals.var_cinr_i,)
    } else {
        (locals.var_cinrd_i,)
    }
};
        locals.var_cinrd_i = assign11210_e10409;
        locals.var_cinrd_i_rv = 0.0;

        let assign11230_e10416: f64 = (8.8541878176e-12 * locals.var_epsrox_i);
        locals.var_epsox = assign11230_e10416;
        locals.var_epsox_rv = 0.0;

        let assign11240_e10419: f64 = (locals.var_epsox / locals.var_tox_i);
        locals.var_coxprime = assign11240_e10419;
        locals.var_coxprime_rv = 0.0;

        let assign11250_e10422: f64 = (locals.var_tox_i * locals.var_tox_i);
        locals.var_tox_sq = assign11250_e10422;
        locals.var_tox_sq_rv = 0.0;

        let assign11260_e10425: f64 = (locals.var_coxprime / 1.6021918e-19);
        locals.var_cox_over_q = assign11260_e10425;
        locals.var_cox_over_q_rv = 0.0;

        let assign11270_e10428: f64 = (locals.var_facneffac_i * locals.var_neff_i);
        locals.var_neffac_i = assign11270_e10428;
        locals.var_neffac_i_rv = 0.0;

        let (assign11280_e10439,) = {
    if (locals.var_neffac_i > 1e20) {
        let (assign11280_e10437,) = {
            if (locals.var_neffac_i < 1e26) {
                (locals.var_neffac_i,)
            } else {
                (1e26,)
            }
        };
        (assign11280_e10437,)
    } else {
        (1e20,)
    }
};
        locals.var_neffac_i = assign11280_e10439;
        locals.var_neffac_i_rv = 0.0;

        locals.var_qq = 0.0;
        locals.var_qq_rv = 0.0;

        let assign11300_e10443: f64 = if p.p52 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard154 = assign11300_e10443;
        locals.var_guard154_rv = 0.0;

        let (assign11310_e10455,) = {
    if (locals.var_guard154 != 0.0) {
        let assign11310_e10447: f64 = (0.4 * 5.951993);
        let assign11310_e10449: f64 = (assign11310_e10447 * p.p52);
        let assign11310_e10452: f64 = (locals.var_coxprime).powf(0.6666666666666666);
        let assign11310_e10453: f64 = (assign11310_e10449 * assign11310_e10452);
        (assign11310_e10453,)
    } else {
        (locals.var_qq,)
    }
};
        locals.var_qq = assign11310_e10455;
        locals.var_qq_rv = 0.0;

        let assign11320_e10458: f64 = (-1.0);
        let assign11320_e10459: f64 = if locals.var_chnl_type == assign11320_e10458 { 1.0 } else { 0.0 };
        locals.var_guard155 = assign11320_e10459;
        locals.var_guard155_rv = 0.0;

        let (assign11330_e10469,) = {
    if ((locals.var_guard154 != 0.0) && (locals.var_guard155 != 0.0)) {
        let assign11330_e10465: f64 = (7.448711 / 5.951993);
        let assign11330_e10467: f64 = (assign11330_e10465 * locals.var_qq);
        (assign11330_e10467,)
    } else {
        (locals.var_qq,)
    }
};
        locals.var_qq = assign11330_e10469;
        locals.var_qq_rv = 0.0;

        let assign11340_e10472: f64 = (1e-8 * locals.var_coxprime);
        let assign11340_e10474: f64 = (assign11340_e10472 / locals.var_epssi);
        locals.var_e_eff0 = assign11340_e10474;
        locals.var_e_eff0_rv = 0.0;

        let assign11350_e10477: f64 = (0.5 * locals.var_feta_i);
        locals.var_eta_mu = assign11350_e10477;
        locals.var_eta_mu_rv = 0.0;

        locals.var_eta_mu1 = 0.5;
        locals.var_eta_mu1_rv = 0.0;

        let assign11370_e10481: f64 = (-1.0);
        let assign11370_e10482: f64 = if locals.var_chnl_type == assign11370_e10481 { 1.0 } else { 0.0 };
        locals.var_guard156 = assign11370_e10482;
        locals.var_guard156_rv = 0.0;

        let (assign11380_e10488,) = {
    if (locals.var_guard156 != 0.0) {
        let assign11380_e10486: f64 = (0.3333333333333333 * locals.var_feta_i);
        (assign11380_e10486,)
    } else {
        (locals.var_eta_mu,)
    }
};
        locals.var_eta_mu = assign11380_e10488;
        locals.var_eta_mu_rv = 0.0;

        let (assign11390_e10492,) = {
    if (locals.var_guard156 != 0.0) {
        (0.3333333333333333,)
    } else {
        (locals.var_eta_mu1,)
    }
};
        locals.var_eta_mu1 = assign11390_e10492;
        locals.var_eta_mu1_rv = 0.0;

        let assign11400_e10495: f64 = (-2.0);
        let assign11400_e10497: f64 = (assign11400_e10495 / locals.var_ax_i);
        let assign11400_e10499: f64 = (assign11400_e10497 + 1.0);
        let assign11400_e10500: f64 = (2.0_f64).powf(assign11400_e10499);
        let assign11400_e10502: f64 = (assign11400_e10500 - 1.0);
        locals.var_temp = assign11400_e10502;
        locals.var_temp_rv = 0.0;

        let assign11410_e10505: f64 = (locals.var_temp - 1.0);
        let assign11410_e10508: f64 = (locals.var_temp - 1.0);
        let assign11410_e10509: f64 = (assign11410_e10505 * assign11410_e10508);
        let assign11410_e10512: f64 = (4.0 * locals.var_temp);
        let (assign11410_e10519,) = {
    if (assign11410_e10512 > 0.0001) {
        let assign11410_e10517: f64 = (4.0 * locals.var_temp);
        (assign11410_e10517,)
    } else {
        (0.0001,)
    }
};
        let assign11410_e10520: f64 = (assign11410_e10509 / assign11410_e10519);
        locals.var_ar = assign11410_e10520;
        locals.var_ar_rv = 0.0;

        let assign11420_e10523: f64 = (-2.0);
        let assign11420_e10525: f64 = (assign11420_e10523 / locals.var_axac_i);
        let assign11420_e10527: f64 = (assign11420_e10525 + 1.0);
        let assign11420_e10528: f64 = (2.0_f64).powf(assign11420_e10527);
        let assign11420_e10530: f64 = (assign11420_e10528 - 1.0);
        locals.var_temp = assign11420_e10530;
        locals.var_temp_rv = 0.0;

        let assign11430_e10533: f64 = (locals.var_temp - 1.0);
        let assign11430_e10536: f64 = (locals.var_temp - 1.0);
        let assign11430_e10537: f64 = (assign11430_e10533 * assign11430_e10536);
        let assign11430_e10540: f64 = (4.0 * locals.var_temp);
        let (assign11430_e10547,) = {
    if (assign11430_e10540 > 0.0001) {
        let assign11430_e10545: f64 = (4.0 * locals.var_temp);
        (assign11430_e10545,)
    } else {
        (0.0001,)
    }
};
        let assign11430_e10548: f64 = (assign11430_e10537 / assign11430_e10547);
        locals.var_arac = assign11430_e10548;
        locals.var_arac_rv = 0.0;

        let assign11440_e10551: f64 = (1.0 / locals.var_vp_i);
        locals.var_inv_vp = assign11440_e10551;
        locals.var_inv_vp_rv = 0.0;

        let assign11450_e10554: f64 = (locals.var_epsox / locals.var_toxov_i);
        locals.var_coxovprime = assign11450_e10554;
        locals.var_coxovprime_rv = 0.0;

        let assign11460_e10557: f64 = (locals.var_epsox / locals.var_toxovd_i);
        locals.var_coxovprime_d = assign11460_e10557;
        locals.var_coxovprime_d_rv = 0.0;

        let assign11470_e10560: f64 = (2.0 * 1.6021918e-19);
        let assign11470_e10562: f64 = (assign11470_e10560 * locals.var_nov_i);
        let assign11470_e10564: f64 = (assign11470_e10562 * locals.var_epssi);
        let assign11470_e10566: f64 = (assign11470_e10564 * locals.var_inv_phita);
        let assign11470_e10567: f64 = (assign11470_e10566).sqrt();
        let assign11470_e10569: f64 = (assign11470_e10567 / locals.var_coxovprime);
        locals.var_gov_s = assign11470_e10569;
        locals.var_gov_s_rv = 0.0;

        let assign11480_e10572: f64 = (2.0 * 1.6021918e-19);
        let assign11480_e10574: f64 = (assign11480_e10572 * locals.var_novd_i);
        let assign11480_e10576: f64 = (assign11480_e10574 * locals.var_epssi);
        let assign11480_e10578: f64 = (assign11480_e10576 * locals.var_inv_phita);
        let assign11480_e10579: f64 = (assign11480_e10578).sqrt();
        let assign11480_e10581: f64 = (assign11480_e10579 / locals.var_coxovprime_d);
        locals.var_gov_d = assign11480_e10581;
        locals.var_gov_d_rv = 0.0;

        let assign11490_e10584: f64 = (locals.var_gov_s * locals.var_gov_s);
        locals.var_gov2_s = assign11490_e10584;
        locals.var_gov2_s_rv = 0.0;

        let assign11500_e10587: f64 = (locals.var_gov_d * locals.var_gov_d);
        locals.var_gov2_d = assign11500_e10587;
        locals.var_gov2_d_rv = 0.0;

        let assign11510_e10590: f64 = (locals.var_cgovaccg_i * 0.005);
        let assign11510_e10592: f64 = (assign11510_e10590 * locals.var_inv_phita);
        let assign11510_e10593: f64 = (assign11510_e10592).exp();
        let assign11510_e10595: f64 = (assign11510_e10593 - 1.0);
        let assign11510_e10596: f64 = (assign11510_e10595).ln();
        let assign11510_e10598: f64 = (assign11510_e10596 / locals.var_cgovaccg_i);
        let assign11510_e10601: f64 = (0.005 * locals.var_inv_phita);
        let assign11510_e10602: f64 = (assign11510_e10601).exp();
        let assign11510_e10604: f64 = (assign11510_e10602 - 1.0);
        let assign11510_e10605: f64 = (assign11510_e10604).ln();
        let assign11510_e10606: f64 = (assign11510_e10598 - assign11510_e10605);
        locals.var_dxgb_ov_th = assign11510_e10606;
        locals.var_dxgb_ov_th_rv = 0.0;

        let assign11520_e10609: f64 = (0.5 * locals.var_gov_s);
        let assign11520_e10610: f64 = (assign11520_e10609).ln();
        let assign11520_e10612: f64 = (assign11520_e10610 + locals.var_dxgb_ov_th);
        locals.var_dxgb_ov_s = assign11520_e10612;
        locals.var_dxgb_ov_s_rv = 0.0;

        let assign11530_e10615: f64 = (0.5 * locals.var_gov_d);
        let assign11530_e10616: f64 = (assign11530_e10615).ln();
        let assign11530_e10618: f64 = (assign11530_e10616 + locals.var_dxgb_ov_th);
        locals.var_dxgb_ov_d = assign11530_e10618;
        locals.var_dxgb_ov_d_rv = 0.0;

        let assign11540_e10621: f64 = (1.0 / locals.var_gov_s);
        locals.var_inv_gov = assign11540_e10621;
        locals.var_inv_gov_rv = 0.0;

        let assign11550_e10624: f64 = (3.1 * locals.var_gov_s);
        let assign11550_e10626: f64 = (assign11550_e10624 + 8.5);
        locals.var_sp_ov_eps = assign11550_e10626;
        locals.var_sp_ov_eps_rv = 0.0;

        let assign11560_e10629: f64 = (locals.var_sp_ov_eps * locals.var_sp_ov_eps);
        locals.var_sp_ov_eps2_s = assign11560_e10629;
        locals.var_sp_ov_eps2_s_rv = 0.0;

        let assign11570_e10632: f64 = (0.5 * locals.var_sp_ov_eps);
        locals.var_sp_ov_delta = assign11570_e10632;
        locals.var_sp_ov_delta_rv = 0.0;

        let assign11580_e10635: f64 = if locals.var_inv_gov < 0.06 { 1.0 } else { 0.0 };
        locals.var_guard157 = assign11580_e10635;
        locals.var_guard157_rv = 0.0;

        let (assign11590_e10641,) = {
    if (locals.var_guard157 != 0.0) {
        let assign11590_e10639: f64 = (64.0 * locals.var_inv_gov);
        (assign11590_e10639,)
    } else {
        (locals.var_sp_ov_a_s,)
    }
};
        locals.var_sp_ov_a_s = assign11590_e10641;
        locals.var_sp_ov_a_s_rv = 0.0;

        let assign11600_e10644: f64 = if locals.var_inv_gov <= 0.45 { 1.0 } else { 0.0 };
        locals.var_guard158 = assign11600_e10644;
        locals.var_guard158_rv = 0.0;

        let (assign11610_e10655,) = {
    if ((locals.var_guard157 == 0.0) && (locals.var_guard158 != 0.0)) {
        let assign11610_e10651: f64 = (22.0 * locals.var_inv_gov);
        let assign11610_e10653: f64 = (assign11610_e10651 + 3.0);
        (assign11610_e10653,)
    } else {
        (locals.var_sp_ov_a_s,)
    }
};
        locals.var_sp_ov_a_s = assign11610_e10655;
        locals.var_sp_ov_a_s_rv = 0.0;

        let assign11620_e10658: f64 = if locals.var_inv_gov <= 1.6 { 1.0 } else { 0.0 };
        locals.var_guard159 = assign11620_e10658;
        locals.var_guard159_rv = 0.0;

        let (assign11630_e10673,) = {
    if (((locals.var_guard157 == 0.0) && (locals.var_guard158 == 0.0)) && (locals.var_guard159 != 0.0)) {
        let assign11630_e10667: f64 = (-7.2);
        let assign11630_e10669: f64 = (assign11630_e10667 * locals.var_inv_gov);
        let assign11630_e10671: f64 = (assign11630_e10669 + 15.5);
        (assign11630_e10671,)
    } else {
        (locals.var_sp_ov_a_s,)
    }
};
        locals.var_sp_ov_a_s = assign11630_e10673;
        locals.var_sp_ov_a_s_rv = 0.0;

        let (assign11640_e10684,) = {
    if (((locals.var_guard157 == 0.0) && (locals.var_guard158 == 0.0)) && (locals.var_guard159 == 0.0)) {
        (locals.var_gov_s,)
    } else {
        (locals.var_sp_ov_a_s,)
    }
};
        locals.var_sp_ov_a_s = assign11640_e10684;
        locals.var_sp_ov_a_s_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_13(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign11650_e10688: f64 = (locals.var_gov2_s * 0.5);
        let assign11650_e10689: f64 = (locals.var_sp_ov_delta + assign11650_e10688);
        let assign11650_e10694: f64 = (locals.var_gov2_s * 0.25);
        let assign11650_e10695: f64 = (locals.var_sp_ov_delta + assign11650_e10694);
        let assign11650_e10697: f64 = (assign11650_e10695 + locals.var_sp_ov_a_s);
        let assign11650_e10698: f64 = (assign11650_e10697).sqrt();
        let assign11650_e10699: f64 = (locals.var_gov_s * assign11650_e10698);
        let assign11650_e10700: f64 = (assign11650_e10689 - assign11650_e10699);
        locals.var_sp_ov_delta1_s = assign11650_e10700;
        locals.var_sp_ov_delta1_s_rv = 0.0;

        let assign11660_e10703: f64 = (1.0 / locals.var_gov_d);
        locals.var_inv_gov = assign11660_e10703;
        locals.var_inv_gov_rv = 0.0;

        let assign11670_e10706: f64 = (3.1 * locals.var_gov_d);
        let assign11670_e10708: f64 = (assign11670_e10706 + 8.5);
        locals.var_sp_ov_eps = assign11670_e10708;
        locals.var_sp_ov_eps_rv = 0.0;

        let assign11680_e10711: f64 = (locals.var_sp_ov_eps * locals.var_sp_ov_eps);
        locals.var_sp_ov_eps2_d = assign11680_e10711;
        locals.var_sp_ov_eps2_d_rv = 0.0;

        let assign11690_e10714: f64 = (0.5 * locals.var_sp_ov_eps);
        locals.var_sp_ov_delta = assign11690_e10714;
        locals.var_sp_ov_delta_rv = 0.0;

        let assign11700_e10717: f64 = if locals.var_inv_gov < 0.06 { 1.0 } else { 0.0 };
        locals.var_guard160 = assign11700_e10717;
        locals.var_guard160_rv = 0.0;

        let (assign11710_e10723,) = {
    if (locals.var_guard160 != 0.0) {
        let assign11710_e10721: f64 = (64.0 * locals.var_inv_gov);
        (assign11710_e10721,)
    } else {
        (locals.var_sp_ov_a_d,)
    }
};
        locals.var_sp_ov_a_d = assign11710_e10723;
        locals.var_sp_ov_a_d_rv = 0.0;

        let assign11720_e10726: f64 = if locals.var_inv_gov <= 0.45 { 1.0 } else { 0.0 };
        locals.var_guard161 = assign11720_e10726;
        locals.var_guard161_rv = 0.0;

        let (assign11730_e10737,) = {
    if ((locals.var_guard160 == 0.0) && (locals.var_guard161 != 0.0)) {
        let assign11730_e10733: f64 = (22.0 * locals.var_inv_gov);
        let assign11730_e10735: f64 = (assign11730_e10733 + 3.0);
        (assign11730_e10735,)
    } else {
        (locals.var_sp_ov_a_d,)
    }
};
        locals.var_sp_ov_a_d = assign11730_e10737;
        locals.var_sp_ov_a_d_rv = 0.0;

        let assign11740_e10740: f64 = if locals.var_inv_gov <= 1.6 { 1.0 } else { 0.0 };
        locals.var_guard162 = assign11740_e10740;
        locals.var_guard162_rv = 0.0;

        let (assign11750_e10755,) = {
    if (((locals.var_guard160 == 0.0) && (locals.var_guard161 == 0.0)) && (locals.var_guard162 != 0.0)) {
        let assign11750_e10749: f64 = (-7.2);
        let assign11750_e10751: f64 = (assign11750_e10749 * locals.var_inv_gov);
        let assign11750_e10753: f64 = (assign11750_e10751 + 15.5);
        (assign11750_e10753,)
    } else {
        (locals.var_sp_ov_a_d,)
    }
};
        locals.var_sp_ov_a_d = assign11750_e10755;
        locals.var_sp_ov_a_d_rv = 0.0;

        let (assign11760_e10766,) = {
    if (((locals.var_guard160 == 0.0) && (locals.var_guard161 == 0.0)) && (locals.var_guard162 == 0.0)) {
        (locals.var_gov_d,)
    } else {
        (locals.var_sp_ov_a_d,)
    }
};
        locals.var_sp_ov_a_d = assign11760_e10766;
        locals.var_sp_ov_a_d_rv = 0.0;

        let assign11770_e10770: f64 = (locals.var_gov2_d * 0.5);
        let assign11770_e10771: f64 = (locals.var_sp_ov_delta + assign11770_e10770);
        let assign11770_e10776: f64 = (locals.var_gov2_d * 0.25);
        let assign11770_e10777: f64 = (locals.var_sp_ov_delta + assign11770_e10776);
        let assign11770_e10779: f64 = (assign11770_e10777 + locals.var_sp_ov_a_d);
        let assign11770_e10780: f64 = (assign11770_e10779).sqrt();
        let assign11770_e10781: f64 = (locals.var_gov_d * assign11770_e10780);
        let assign11770_e10782: f64 = (assign11770_e10771 - assign11770_e10781);
        locals.var_sp_ov_delta1_d = assign11770_e10782;
        locals.var_sp_ov_delta1_d_rv = 0.0;

        let assign11780_e10785: f64 = (locals.var_eg + locals.var_dphib_i);
        let assign11780_e10788: f64 = (2.0 * locals.var_phit);
        let assign11780_e10792: f64 = (-0.75);
        let assign11780_e10793: f64 = (locals.var_phibfac).powf(assign11780_e10792);
        let assign11780_e10794: f64 = (locals.var_neff_i * assign11780_e10793);
        let assign11780_e10796: f64 = (assign11780_e10794 * 4e-26);
        let assign11780_e10797: f64 = (assign11780_e10796).ln();
        let assign11780_e10798: f64 = (assign11780_e10788 * assign11780_e10797);
        let assign11780_e10799: f64 = (assign11780_e10785 + assign11780_e10798);
        locals.var_phib_dc = assign11780_e10799;
        locals.var_phib_dc_rv = 0.0;

        let (assign11790_e10805,) = {
    if (locals.var_phib_dc > 0.05) {
        (locals.var_phib_dc,)
    } else {
        (0.05,)
    }
};
        locals.var_phib_dc = assign11790_e10805;
        locals.var_phib_dc_rv = 0.0;

        let assign11800_e10808: f64 = (2.0 * 1.6021918e-19);
        let assign11800_e10810: f64 = (assign11800_e10808 * locals.var_neff_i);
        let assign11800_e10812: f64 = (assign11800_e10810 * locals.var_epssi);
        let assign11800_e10814: f64 = (assign11800_e10812 * locals.var_inv_phit);
        let assign11800_e10815: f64 = (assign11800_e10814).sqrt();
        let assign11800_e10817: f64 = (assign11800_e10815 / locals.var_coxprime);
        locals.var_g_0_dc = assign11800_e10817;
        locals.var_g_0_dc_rv = 0.0;

        locals.var_kp = 0.0;
        locals.var_kp_rv = 0.0;

        locals.var_np = 0.0;
        locals.var_np_rv = 0.0;

        let assign11830_e10822: f64 = if locals.var_np_i > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard163 = assign11830_e10822;
        locals.var_guard163_rv = 0.0;

        let (assign11840_e10828,) = {
    if (locals.var_guard163 != 0.0) {
        let assign11840_e10826: f64 = (80000000.0 / locals.var_tox_sq);
        (assign11840_e10826,)
    } else {
        (locals.var_arg2max,)
    }
};
        locals.var_arg2max = assign11840_e10828;
        locals.var_arg2max_rv = 0.0;

        let (assign11850_e10837,) = {
    if (locals.var_guard163 != 0.0) {
        let (assign11850_e10835,) = {
            if (locals.var_np_i > locals.var_arg2max) {
                (locals.var_np_i,)
            } else {
                (locals.var_arg2max,)
            }
        };
        (assign11850_e10835,)
    } else {
        (locals.var_np,)
    }
};
        locals.var_np = assign11850_e10837;
        locals.var_np_rv = 0.0;

        let (assign11860_e10846,) = {
    if (locals.var_guard163 != 0.0) {
        let (assign11860_e10844,) = {
            if (5e24 > locals.var_np) {
                (5e24,)
            } else {
                (locals.var_np,)
            }
        };
        (assign11860_e10844,)
    } else {
        (locals.var_np,)
    }
};
        locals.var_np = assign11860_e10846;
        locals.var_np_rv = 0.0;

        let (assign11870_e10862,) = {
    if (locals.var_guard163 != 0.0) {
        let assign11870_e10850: f64 = (2.0 * locals.var_coxprime);
        let assign11870_e10852: f64 = (assign11870_e10850 * locals.var_coxprime);
        let assign11870_e10854: f64 = (assign11870_e10852 * locals.var_phit);
        let assign11870_e10857: f64 = (1.6021918e-19 * locals.var_np);
        let assign11870_e10859: f64 = (assign11870_e10857 * locals.var_epssi);
        let assign11870_e10860: f64 = (assign11870_e10854 / assign11870_e10859);
        (assign11870_e10860,)
    } else {
        (locals.var_kp,)
    }
};
        locals.var_kp = assign11870_e10862;
        locals.var_kp_rv = 0.0;

        let assign11880_e10865: f64 = (100.0 * locals.var_phit);
        let assign11880_e10867: f64 = (assign11880_e10865 * locals.var_phit);
        locals.var_qlim2 = assign11880_e10867;
        locals.var_qlim2_rv = 0.0;

        let assign11890_e10870: f64 = if p.p52 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard164 = assign11890_e10870;
        locals.var_guard164_rv = 0.0;

        let (assign11900_e10881,) = {
    if (locals.var_guard164 != 0.0) {
        let assign11900_e10874: f64 = (locals.var_phit * locals.var_g_0_dc);
        let assign11900_e10876: f64 = (assign11900_e10874 * locals.var_g_0_dc);
        let assign11900_e10878: f64 = (assign11900_e10876 * locals.var_phib_dc);
        let assign11900_e10879: f64 = (assign11900_e10878).sqrt();
        (assign11900_e10879,)
    } else {
        (locals.var_qb0,)
    }
};
        locals.var_qb0 = assign11900_e10881;
        locals.var_qb0_rv = 0.0;

        let (assign11910_e10891,) = {
    if (locals.var_guard164 != 0.0) {
        let assign11910_e10885: f64 = (0.75 * locals.var_qq);
        let assign11910_e10888: f64 = (locals.var_qb0).powf(0.6666666666666666);
        let assign11910_e10889: f64 = (assign11910_e10885 * assign11910_e10888);
        (assign11910_e10889,)
    } else {
        (locals.var_dphibq,)
    }
};
        locals.var_dphibq = assign11910_e10891;
        locals.var_dphibq_rv = 0.0;

        let (assign11920_e10897,) = {
    if (locals.var_guard164 != 0.0) {
        let assign11920_e10895: f64 = (locals.var_phib_dc + locals.var_dphibq);
        (assign11920_e10895,)
    } else {
        (locals.var_phib_dc,)
    }
};
        locals.var_phib_dc = assign11920_e10897;
        locals.var_phib_dc_rv = 0.0;

        let (assign11930_e10911,) = {
    if (locals.var_guard164 != 0.0) {
        let assign11930_e10903: f64 = (2.0 * 0.6666666666666666);
        let assign11930_e10905: f64 = (assign11930_e10903 * locals.var_dphibq);
        let assign11930_e10907: f64 = (assign11930_e10905 / locals.var_qb0);
        let assign11930_e10908: f64 = (1.0 + assign11930_e10907);
        let assign11930_e10909: f64 = (locals.var_g_0_dc * assign11930_e10908);
        (assign11930_e10909,)
    } else {
        (locals.var_g_0_dc,)
    }
};
        locals.var_g_0_dc = assign11930_e10911;
        locals.var_g_0_dc_rv = 0.0;

        let assign11940_e10913: f64 = (locals.var_phib_dc).sqrt();
        locals.var_sqrt_phib_dc = assign11940_e10913;
        locals.var_sqrt_phib_dc_rv = 0.0;

        let assign11950_e10916: f64 = (0.95 * locals.var_phib_dc);
        locals.var_phix_dc = assign11950_e10916;
        locals.var_phix_dc_rv = 0.0;

        let assign11960_e10919: f64 = (0.0025 * locals.var_phib_dc);
        let assign11960_e10921: f64 = (assign11960_e10919 * locals.var_phib_dc);
        locals.var_aphi_dc = assign11960_e10921;
        locals.var_aphi_dc_rv = 0.0;

        locals.var_bphi_dc = locals.var_aphi_dc;
        locals.var_bphi_dc_rv = 0.0;

        let assign11980_e10925: f64 = (locals.var_bphi_dc).sqrt();
        let assign11980_e10926: f64 = (0.5 * assign11980_e10925);
        locals.var_phix2 = assign11980_e10926;
        locals.var_phix2_rv = 0.0;

        let assign11990_e10930: f64 = (locals.var_phix_dc - locals.var_phix2);
        let assign11990_e10932: f64 = assign11990_e10930;
        let assign11990_e10935: f64 = (locals.var_phix_dc - locals.var_phix2);
        let assign11990_e10937: f64 = assign11990_e10935;
        let assign11990_e10940: f64 = (locals.var_phix_dc - locals.var_phix2);
        let assign11990_e10942: f64 = assign11990_e10940;
        let assign11990_e10943: f64 = (assign11990_e10937 * assign11990_e10942);
        let assign11990_e10945: f64 = (assign11990_e10943 + locals.var_aphi_dc);
        let assign11990_e10946: f64 = (assign11990_e10945).sqrt();
        let assign11990_e10947: f64 = (assign11990_e10932 - assign11990_e10946);
        let assign11990_e10948: f64 = (0.5 * assign11990_e10947);
        locals.var_phix1_dc = assign11990_e10948;
        locals.var_phix1_dc_rv = 0.0;

        let assign12000_e10952: f64 = (locals.var_phib_dc + locals.var_eg);
        let assign12000_e10953: f64 = (0.5 * assign12000_e10952);
        locals.var_alpha_b = assign12000_e10953;
        locals.var_alpha_b_rv = 0.0;

        let assign12010_e10956: f64 = (locals.var_vsbnud_i + locals.var_phib_dc);
        let assign12010_e10957: f64 = (assign12010_e10956).sqrt();
        let assign12010_e10959: f64 = (assign12010_e10957 - locals.var_sqrt_phib_dc);
        locals.var_us1 = assign12010_e10959;
        locals.var_us1_rv = 0.0;

        let assign12020_e10962: f64 = (locals.var_vsbnud_i + locals.var_dvsbnud_i);
        let assign12020_e10964: f64 = (assign12020_e10962 + locals.var_phib_dc);
        let assign12020_e10965: f64 = (assign12020_e10964).sqrt();
        let assign12020_e10967: f64 = (assign12020_e10965 - locals.var_sqrt_phib_dc);
        let assign12020_e10969: f64 = (assign12020_e10967 - locals.var_us1);
        locals.var_us21 = assign12020_e10969;
        locals.var_us21_rv = 0.0;

        let assign12030_e10972: f64 = (locals.var_eg + locals.var_dphib_i);
        let assign12030_e10974: f64 = (assign12030_e10972 + locals.var_delvtac_i);
        let assign12030_e10977: f64 = (2.0 * locals.var_phit);
        let assign12030_e10981: f64 = (-0.75);
        let assign12030_e10982: f64 = (locals.var_phibfac).powf(assign12030_e10981);
        let assign12030_e10983: f64 = (locals.var_neffac_i * assign12030_e10982);
        let assign12030_e10985: f64 = (assign12030_e10983 * 4e-26);
        let assign12030_e10986: f64 = (assign12030_e10985).ln();
        let assign12030_e10987: f64 = (assign12030_e10977 * assign12030_e10986);
        let assign12030_e10988: f64 = (assign12030_e10974 + assign12030_e10987);
        locals.var_phib_ac = assign12030_e10988;
        locals.var_phib_ac_rv = 0.0;

        let (assign12040_e10994,) = {
    if (locals.var_phib_ac > 0.05) {
        (locals.var_phib_ac,)
    } else {
        (0.05,)
    }
};
        locals.var_phib_ac = assign12040_e10994;
        locals.var_phib_ac_rv = 0.0;

        let assign12050_e10997: f64 = (2.0 * 1.6021918e-19);
        let assign12050_e10999: f64 = (assign12050_e10997 * locals.var_neffac_i);
        let assign12050_e11001: f64 = (assign12050_e10999 * locals.var_epssi);
        let assign12050_e11003: f64 = (assign12050_e11001 * locals.var_inv_phit);
        let assign12050_e11004: f64 = (assign12050_e11003).sqrt();
        let assign12050_e11006: f64 = (assign12050_e11004 / locals.var_coxprime);
        locals.var_g_0_ac = assign12050_e11006;
        locals.var_g_0_ac_rv = 0.0;

        let assign12060_e11009: f64 = if p.p52 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard165 = assign12060_e11009;
        locals.var_guard165_rv = 0.0;

        let (assign12070_e11020,) = {
    if (locals.var_guard165 != 0.0) {
        let assign12070_e11013: f64 = (locals.var_phit * locals.var_g_0_ac);
        let assign12070_e11015: f64 = (assign12070_e11013 * locals.var_g_0_ac);
        let assign12070_e11017: f64 = (assign12070_e11015 * locals.var_phib_ac);
        let assign12070_e11018: f64 = (assign12070_e11017).sqrt();
        (assign12070_e11018,)
    } else {
        (locals.var_qb0,)
    }
};
        locals.var_qb0 = assign12070_e11020;
        locals.var_qb0_rv = 0.0;

        let (assign12080_e11030,) = {
    if (locals.var_guard165 != 0.0) {
        let assign12080_e11024: f64 = (0.75 * locals.var_qq);
        let assign12080_e11027: f64 = (locals.var_qb0).powf(0.6666666666666666);
        let assign12080_e11028: f64 = (assign12080_e11024 * assign12080_e11027);
        (assign12080_e11028,)
    } else {
        (locals.var_dphibq,)
    }
};
        locals.var_dphibq = assign12080_e11030;
        locals.var_dphibq_rv = 0.0;

        let (assign12090_e11036,) = {
    if (locals.var_guard165 != 0.0) {
        let assign12090_e11034: f64 = (locals.var_phib_ac + locals.var_dphibq);
        (assign12090_e11034,)
    } else {
        (locals.var_phib_ac,)
    }
};
        locals.var_phib_ac = assign12090_e11036;
        locals.var_phib_ac_rv = 0.0;

        let (assign12100_e11050,) = {
    if (locals.var_guard165 != 0.0) {
        let assign12100_e11042: f64 = (2.0 * 0.6666666666666666);
        let assign12100_e11044: f64 = (assign12100_e11042 * locals.var_dphibq);
        let assign12100_e11046: f64 = (assign12100_e11044 / locals.var_qb0);
        let assign12100_e11047: f64 = (1.0 + assign12100_e11046);
        let assign12100_e11048: f64 = (locals.var_g_0_ac * assign12100_e11047);
        (assign12100_e11048,)
    } else {
        (locals.var_g_0_ac,)
    }
};
        locals.var_g_0_ac = assign12100_e11050;
        locals.var_g_0_ac_rv = 0.0;

        let assign12110_e11053: f64 = (0.95 * locals.var_phib_ac);
        locals.var_phix_ac = assign12110_e11053;
        locals.var_phix_ac_rv = 0.0;

        let assign12120_e11056: f64 = (0.0025 * locals.var_phib_ac);
        let assign12120_e11058: f64 = (assign12120_e11056 * locals.var_phib_ac);
        locals.var_aphi_ac = assign12120_e11058;
        locals.var_aphi_ac_rv = 0.0;

        locals.var_bphi_ac = locals.var_aphi_ac;
        locals.var_bphi_ac_rv = 0.0;

        let assign12140_e11062: f64 = (locals.var_bphi_ac).sqrt();
        let assign12140_e11063: f64 = (0.5 * assign12140_e11062);
        locals.var_phix2 = assign12140_e11063;
        locals.var_phix2_rv = 0.0;

        let assign12150_e11067: f64 = (locals.var_phix_ac - locals.var_phix2);
        let assign12150_e11069: f64 = assign12150_e11067;
        let assign12150_e11072: f64 = (locals.var_phix_ac - locals.var_phix2);
        let assign12150_e11074: f64 = assign12150_e11072;
        let assign12150_e11077: f64 = (locals.var_phix_ac - locals.var_phix2);
        let assign12150_e11079: f64 = assign12150_e11077;
        let assign12150_e11080: f64 = (assign12150_e11074 * assign12150_e11079);
        let assign12150_e11082: f64 = (assign12150_e11080 + locals.var_aphi_ac);
        let assign12150_e11083: f64 = (assign12150_e11082).sqrt();
        let assign12150_e11084: f64 = (assign12150_e11069 - assign12150_e11083);
        let assign12150_e11085: f64 = (0.5 * assign12150_e11084);
        locals.var_phix1_ac = assign12150_e11085;
        locals.var_phix1_ac_rv = 0.0;

        let assign12160_e11089: f64 = (locals.var_stvfb_i * locals.var_delt);
        let assign12160_e11093: f64 = (locals.var_st2vfb_i * locals.var_delt);
        let assign12160_e11094: f64 = (1.0 + assign12160_e11093);
        let assign12160_e11095: f64 = (assign12160_e11089 * assign12160_e11094);
        let assign12160_e11096: f64 = (locals.var_vfb_i + assign12160_e11095);
        let assign12160_e11098: f64 = (assign12160_e11096 + locals.var_delvto_i);
        locals.var_vfb_t = assign12160_e11098;
        locals.var_vfb_t_rv = 0.0;

        let assign12170_e11101: f64 = (locals.var_stct_i * locals.var_ln_rtn);
        let assign12170_e11102: f64 = (assign12170_e11101).exp();
        locals.var_tf_ct = assign12170_e11102;
        locals.var_tf_ct_rv = 0.0;

        let assign12180_e11105: f64 = (locals.var_ct_i * locals.var_tf_ct);
        locals.var_ct_t = assign12180_e11105;
        locals.var_ct_t_rv = 0.0;

        let assign12190_e11108: f64 = (locals.var_ctg_i / locals.var_rtn);
        locals.var_ctg_t = assign12190_e11108;
        locals.var_ctg_t_rv = 0.0;

        let assign12200_e11111: f64 = (locals.var_stbet_i * locals.var_ln_rtn);
        let assign12200_e11112: f64 = (assign12200_e11111).exp();
        locals.var_tf_bet = assign12200_e11112;
        locals.var_tf_bet_rv = 0.0;

        let assign12210_e11115: f64 = (locals.var_betn_i * locals.var_tf_bet);
        locals.var_betn_t = assign12210_e11115;
        locals.var_betn_t_rv = 0.0;

        let assign12220_e11118: f64 = (locals.var_factuo_i * locals.var_betn_t);
        let assign12220_e11120: f64 = (assign12220_e11118 * locals.var_coxprime);
        locals.var_bet_i = assign12220_e11120;
        locals.var_bet_i_rv = 0.0;

        let assign12230_e11124: f64 = (locals.var_stthemu_i * locals.var_ln_rtn);
        let assign12230_e11125: f64 = (assign12230_e11124).exp();
        let assign12230_e11126: f64 = (locals.var_themu_i * assign12230_e11125);
        locals.var_themu_t = assign12230_e11126;
        locals.var_themu_t_rv = 0.0;

        let assign12240_e11129: f64 = (locals.var_stmue_i * locals.var_ln_rtn);
        let assign12240_e11130: f64 = (assign12240_e11129).exp();
        locals.var_tf_mue = assign12240_e11130;
        locals.var_tf_mue_rv = 0.0;

        let assign12250_e11133: f64 = (locals.var_mue_i * locals.var_tf_mue);
        locals.var_mue_t = assign12250_e11133;
        locals.var_mue_t_rv = 0.0;

        let assign12260_e11137: f64 = (locals.var_stthecs_i * locals.var_ln_rtn);
        let assign12260_e11138: f64 = (assign12260_e11137).exp();
        let assign12260_e11139: f64 = (locals.var_thecs_i * assign12260_e11138);
        locals.var_thecs_t = assign12260_e11139;
        locals.var_thecs_t_rv = 0.0;

        let assign12270_e11142: f64 = (locals.var_stcs_i * locals.var_ln_rtn);
        let assign12270_e11143: f64 = (assign12270_e11142).exp();
        locals.var_tf_cs = assign12270_e11143;
        locals.var_tf_cs_rv = 0.0;

        let assign12280_e11146: f64 = (locals.var_cs_i * locals.var_tf_cs);
        locals.var_cs_t = assign12280_e11146;
        locals.var_cs_t_rv = 0.0;

        let assign12290_e11149: f64 = (locals.var_stxcor_i * locals.var_ln_rtn);
        let assign12290_e11150: f64 = (assign12290_e11149).exp();
        locals.var_tf_xcor = assign12290_e11150;
        locals.var_tf_xcor_rv = 0.0;

        let assign12300_e11153: f64 = (locals.var_xcor_i * locals.var_tf_xcor);
        locals.var_xcor_t = assign12300_e11153;
        locals.var_xcor_t_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_14(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign12310_e11156: f64 = (locals.var_strs_i * locals.var_ln_rtn);
        let assign12310_e11157: f64 = (assign12310_e11156).exp();
        locals.var_tf_ther = assign12310_e11157;
        locals.var_tf_ther_rv = 0.0;

        let assign12320_e11160: f64 = (locals.var_rs_i * locals.var_tf_ther);
        locals.var_rs_t = assign12320_e11160;
        locals.var_rs_t_rv = 0.0;

        let assign12330_e11163: f64 = (2.0 * locals.var_bet_i);
        let assign12330_e11165: f64 = (assign12330_e11163 * locals.var_rs_t);
        locals.var_ther_i = assign12330_e11165;
        locals.var_ther_i_rv = 0.0;

        let assign12340_e11168: f64 = (locals.var_stthesat_i * locals.var_ln_rtn);
        let assign12340_e11169: f64 = (assign12340_e11168).exp();
        locals.var_tf_thesat = assign12340_e11169;
        locals.var_tf_thesat_rv = 0.0;

        let assign12350_e11172: f64 = (locals.var_thesat_i * locals.var_tf_thesat);
        locals.var_thesat_t = assign12350_e11172;
        locals.var_thesat_t_rv = 0.0;

        let assign12360_e11175: f64 = (locals.var_thesatac_i * locals.var_tf_thesat);
        locals.var_thesatac_t = assign12360_e11175;
        locals.var_thesatac_t_rv = 0.0;

        let assign12370_e11178: f64 = (-locals.var_sta2_i);
        let assign12370_e11180: f64 = (assign12370_e11178 * locals.var_ln_rtn);
        let assign12370_e11181: f64 = (assign12370_e11180).exp();
        let assign12370_e11182: f64 = (locals.var_a2_i * assign12370_e11181);
        locals.var_a2_t = assign12370_e11182;
        locals.var_a2_t_rv = 0.0;

        let assign12380_e11185: f64 = (locals.var_fnt_i * 4.0);
        let assign12380_e11187: f64 = (assign12380_e11185 * 1.3806505e-23);
        let assign12380_e11189: f64 = (assign12380_e11187 * locals.var_tkd);
        locals.var_nt = assign12380_e11189;
        locals.var_nt_rv = 0.0;

        let assign12400_e11203: f64 = if ((p.p46 != 0.0) && (locals.var_betnedge_i > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard166 = assign12400_e11203;
        locals.var_guard166_rv = 0.0;

        let (assign12410_e11213,) = {
    if (locals.var_guard166 != 0.0) {
        let assign12410_e11208: f64 = (locals.var_stvfbedge_i * locals.var_delt);
        let assign12410_e11209: f64 = (locals.var_vfbedge_i + assign12410_e11208);
        let assign12410_e11211: f64 = (assign12410_e11209 + locals.var_delvtoedge_i);
        (assign12410_e11211,)
    } else {
        (locals.var_vfbedge_t,)
    }
};
        locals.var_vfbedge_t = assign12410_e11213;
        locals.var_vfbedge_t_rv = 0.0;

        let (assign12420_e11220,) = {
    if (locals.var_guard166 != 0.0) {
        let assign12420_e11217: f64 = (locals.var_stbetedge_i * locals.var_ln_rtn);
        let assign12420_e11218: f64 = (assign12420_e11217).exp();
        (assign12420_e11218,)
    } else {
        (locals.var_tf_betedge,)
    }
};
        locals.var_tf_betedge = assign12420_e11220;
        locals.var_tf_betedge_rv = 0.0;

        let (assign12430_e11226,) = {
    if (locals.var_guard166 != 0.0) {
        let assign12430_e11224: f64 = (locals.var_betnedge_i * locals.var_tf_betedge);
        (assign12430_e11224,)
    } else {
        (locals.var_betnedge_t,)
    }
};
        locals.var_betnedge_t = assign12430_e11226;
        locals.var_betnedge_t_rv = 0.0;

        let (assign12440_e11234,) = {
    if (locals.var_guard166 != 0.0) {
        let assign12440_e11230: f64 = (locals.var_factuoedge_i * locals.var_betnedge_t);
        let assign12440_e11232: f64 = (assign12440_e11230 * locals.var_coxprime);
        (assign12440_e11232,)
    } else {
        (locals.var_betedge_i,)
    }
};
        locals.var_betedge_i = assign12440_e11234;
        locals.var_betedge_i_rv = 0.0;

        let (assign12450_e11244,) = {
    if (locals.var_guard166 != 0.0) {
        let assign12450_e11240: f64 = (locals.var_ctedge_i * locals.var_rtn);
        let assign12450_e11241: f64 = (1.0 + assign12450_e11240);
        let assign12450_e11242: f64 = (locals.var_phit * assign12450_e11241);
        (assign12450_e11242,)
    } else {
        (locals.var_phit0edge,)
    }
};
        locals.var_phit0edge = assign12450_e11244;
        locals.var_phit0edge_rv = 0.0;

        let (assign12460_e11264,) = {
    if (locals.var_guard166 != 0.0) {
        let assign12460_e11248: f64 = (locals.var_eg + locals.var_dphibedge_i);
        let assign12460_e11251: f64 = (2.0 * locals.var_phit0edge);
        let assign12460_e11255: f64 = (-0.75);
        let assign12460_e11256: f64 = (locals.var_phibfac).powf(assign12460_e11255);
        let assign12460_e11257: f64 = (locals.var_neffedge_i * assign12460_e11256);
        let assign12460_e11259: f64 = (assign12460_e11257 * 4e-26);
        let assign12460_e11260: f64 = (assign12460_e11259).ln();
        let assign12460_e11261: f64 = (assign12460_e11251 * assign12460_e11260);
        let assign12460_e11262: f64 = (assign12460_e11248 + assign12460_e11261);
        (assign12460_e11262,)
    } else {
        (locals.var_phibedge,)
    }
};
        locals.var_phibedge = assign12460_e11264;
        locals.var_phibedge_rv = 0.0;

        let (assign12470_e11273,) = {
    if (locals.var_guard166 != 0.0) {
        let (assign12470_e11271,) = {
            if (locals.var_phibedge > 0.05) {
                (locals.var_phibedge,)
            } else {
                (0.05,)
            }
        };
        (assign12470_e11271,)
    } else {
        (locals.var_phibedge,)
    }
};
        locals.var_phibedge = assign12470_e11273;
        locals.var_phibedge_rv = 0.0;

        let (assign12480_e11288,) = {
    if (locals.var_guard166 != 0.0) {
        let assign12480_e11277: f64 = (2.0 * 1.6021918e-19);
        let assign12480_e11279: f64 = (assign12480_e11277 * locals.var_neffedge_i);
        let assign12480_e11281: f64 = (assign12480_e11279 * locals.var_epssi);
        let assign12480_e11283: f64 = (assign12480_e11281 * locals.var_inv_phit);
        let assign12480_e11284: f64 = (assign12480_e11283).sqrt();
        let assign12480_e11286: f64 = (assign12480_e11284 / locals.var_coxprime);
        (assign12480_e11286,)
    } else {
        (locals.var_gfedge,)
    }
};
        locals.var_gfedge = assign12480_e11288;
        locals.var_gfedge_rv = 0.0;

        let (assign12490_e11294,) = {
    if (locals.var_guard166 != 0.0) {
        let assign12490_e11292: f64 = (locals.var_gfedge * locals.var_gfedge);
        (assign12490_e11292,)
    } else {
        (locals.var_gfedge2,)
    }
};
        locals.var_gfedge2 = assign12490_e11294;
        locals.var_gfedge2_rv = 0.0;

        let (assign12500_e11299,) = {
    if (locals.var_guard166 != 0.0) {
        let assign12500_e11297: f64 = (locals.var_gfedge2).ln();
        (assign12500_e11297,)
    } else {
        (locals.var_lngfedge2,)
    }
};
        locals.var_lngfedge2 = assign12500_e11299;
        locals.var_lngfedge2_rv = 0.0;

        let (assign12510_e11305,) = {
    if (locals.var_guard166 != 0.0) {
        let assign12510_e11303: f64 = (0.95 * locals.var_phibedge);
        (assign12510_e11303,)
    } else {
        (locals.var_phixedge,)
    }
};
        locals.var_phixedge = assign12510_e11305;
        locals.var_phixedge_rv = 0.0;

        let (assign12520_e11313,) = {
    if (locals.var_guard166 != 0.0) {
        let assign12520_e11309: f64 = (0.0025 * locals.var_phibedge);
        let assign12520_e11311: f64 = (assign12520_e11309 * locals.var_phibedge);
        (assign12520_e11311,)
    } else {
        (locals.var_aphiedge,)
    }
};
        locals.var_aphiedge = assign12520_e11313;
        locals.var_aphiedge_rv = 0.0;

        let (assign12530_e11317,) = {
    if (locals.var_guard166 != 0.0) {
        (locals.var_aphiedge,)
    } else {
        (locals.var_bphiedge,)
    }
};
        locals.var_bphiedge = assign12530_e11317;
        locals.var_bphiedge_rv = 0.0;

        let (assign12540_e11324,) = {
    if (locals.var_guard166 != 0.0) {
        let assign12540_e11321: f64 = (locals.var_bphiedge).sqrt();
        let assign12540_e11322: f64 = (0.5 * assign12540_e11321);
        (assign12540_e11322,)
    } else {
        (locals.var_phix2edge,)
    }
};
        locals.var_phix2edge = assign12540_e11324;
        locals.var_phix2edge_rv = 0.0;

        let (assign12550_e11349,) = {
    if (locals.var_guard166 != 0.0) {
        let assign12550_e11329: f64 = (locals.var_phixedge - locals.var_phix2edge);
        let assign12550_e11331: f64 = assign12550_e11329;
        let assign12550_e11334: f64 = (locals.var_phixedge - locals.var_phix2edge);
        let assign12550_e11336: f64 = assign12550_e11334;
        let assign12550_e11339: f64 = (locals.var_phixedge - locals.var_phix2edge);
        let assign12550_e11341: f64 = assign12550_e11339;
        let assign12550_e11342: f64 = (assign12550_e11336 * assign12550_e11341);
        let assign12550_e11344: f64 = (assign12550_e11342 + locals.var_aphiedge);
        let assign12550_e11345: f64 = (assign12550_e11344).sqrt();
        let assign12550_e11346: f64 = (assign12550_e11331 - assign12550_e11345);
        let assign12550_e11347: f64 = (0.5 * assign12550_e11346);
        (assign12550_e11347,)
    } else {
        (locals.var_phix1edge,)
    }
};
        locals.var_phix1edge = assign12550_e11349;
        locals.var_phix1edge_rv = 0.0;

        let (assign12580_e11374,) = {
    if (locals.var_guard166 == 0.0) {
        (0.0,)
    } else {
        (locals.var_vfbedge_t,)
    }
};
        locals.var_vfbedge_t = assign12580_e11374;
        locals.var_vfbedge_t_rv = 0.0;

        let (assign12590_e11379,) = {
    if (locals.var_guard166 == 0.0) {
        (1.0,)
    } else {
        (locals.var_tf_betedge,)
    }
};
        locals.var_tf_betedge = assign12590_e11379;
        locals.var_tf_betedge_rv = 0.0;

        let (assign12600_e11384,) = {
    if (locals.var_guard166 == 0.0) {
        (0.0,)
    } else {
        (locals.var_betnedge_t,)
    }
};
        locals.var_betnedge_t = assign12600_e11384;
        locals.var_betnedge_t_rv = 0.0;

        let (assign12610_e11389,) = {
    if (locals.var_guard166 == 0.0) {
        (0.0,)
    } else {
        (locals.var_betedge_i,)
    }
};
        locals.var_betedge_i = assign12610_e11389;
        locals.var_betedge_i_rv = 0.0;

        let (assign12620_e11394,) = {
    if (locals.var_guard166 == 0.0) {
        (locals.var_phit,)
    } else {
        (locals.var_phit0edge,)
    }
};
        locals.var_phit0edge = assign12620_e11394;
        locals.var_phit0edge_rv = 0.0;

        let (assign12630_e11399,) = {
    if (locals.var_guard166 == 0.0) {
        (0.0,)
    } else {
        (locals.var_phibedge,)
    }
};
        locals.var_phibedge = assign12630_e11399;
        locals.var_phibedge_rv = 0.0;

        let (assign12640_e11404,) = {
    if (locals.var_guard166 == 0.0) {
        (1.0,)
    } else {
        (locals.var_gfedge,)
    }
};
        locals.var_gfedge = assign12640_e11404;
        locals.var_gfedge_rv = 0.0;

        let (assign12650_e11409,) = {
    if (locals.var_guard166 == 0.0) {
        (1.0,)
    } else {
        (locals.var_gfedge2,)
    }
};
        locals.var_gfedge2 = assign12650_e11409;
        locals.var_gfedge2_rv = 0.0;

        let (assign12660_e11414,) = {
    if (locals.var_guard166 == 0.0) {
        (0.0,)
    } else {
        (locals.var_lngfedge2,)
    }
};
        locals.var_lngfedge2 = assign12660_e11414;
        locals.var_lngfedge2_rv = 0.0;

        let (assign12670_e11419,) = {
    if (locals.var_guard166 == 0.0) {
        (0.0,)
    } else {
        (locals.var_phixedge,)
    }
};
        locals.var_phixedge = assign12670_e11419;
        locals.var_phixedge_rv = 0.0;

        let (assign12680_e11424,) = {
    if (locals.var_guard166 == 0.0) {
        (0.0,)
    } else {
        (locals.var_aphiedge,)
    }
};
        locals.var_aphiedge = assign12680_e11424;
        locals.var_aphiedge_rv = 0.0;

        let (assign12690_e11429,) = {
    if (locals.var_guard166 == 0.0) {
        (0.0,)
    } else {
        (locals.var_bphiedge,)
    }
};
        locals.var_bphiedge = assign12690_e11429;
        locals.var_bphiedge_rv = 0.0;

        let (assign12700_e11434,) = {
    if (locals.var_guard166 == 0.0) {
        (0.0,)
    } else {
        (locals.var_phix2edge,)
    }
};
        locals.var_phix2edge = assign12700_e11434;
        locals.var_phix2edge_rv = 0.0;

        let (assign12710_e11439,) = {
    if (locals.var_guard166 == 0.0) {
        (0.0,)
    } else {
        (locals.var_phix1edge,)
    }
};
        locals.var_phix1edge = assign12710_e11439;
        locals.var_phix1edge_rv = 0.0;

        let assign12740_e11452: f64 = (1.0 / locals.var_chib_i);
        locals.var_inv_chib = assign12740_e11452;
        locals.var_inv_chib_rv = 0.0;

        let assign12750_e11455: f64 = (4.0 * 0.3333333333333333);
        let assign12750_e11458: f64 = (2.0 * 1.6021918e-19);
        let assign12750_e11460: f64 = (assign12750_e11458 * 9.1093826e-31);
        let assign12750_e11462: f64 = (assign12750_e11460 * locals.var_chib_i);
        let assign12750_e11463: f64 = (assign12750_e11462).sqrt();
        let assign12750_e11464: f64 = (assign12750_e11455 * assign12750_e11463);
        let assign12750_e11466: f64 = (assign12750_e11464 / 1.05457168e-34);
        locals.var_b_fact = assign12750_e11466;
        locals.var_b_fact_rv = 0.0;

        let assign12760_e11469: f64 = (locals.var_b_fact * locals.var_tox_i);
        locals.var_bch = assign12760_e11469;
        locals.var_bch_rv = 0.0;

        let assign12770_e11472: f64 = (locals.var_b_fact * locals.var_toxov_i);
        locals.var_bov = assign12770_e11472;
        locals.var_bov_rv = 0.0;

        let assign12780_e11475: f64 = (locals.var_b_fact * locals.var_toxovd_i);
        locals.var_bov_d = assign12780_e11475;
        locals.var_bov_d_rv = 0.0;

        locals.var_gcq = 0.0;
        locals.var_gcq_rv = 0.0;

        let assign12800_e11479: f64 = if locals.var_gc3_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard167 = assign12800_e11479;
        locals.var_guard167_rv = 0.0;

        let (assign12810_e11488,) = {
    if (locals.var_guard167 != 0.0) {
        let assign12810_e11482: f64 = (-0.495);
        let assign12810_e11484: f64 = (assign12810_e11482 * locals.var_gc2_i);
        let assign12810_e11486: f64 = (assign12810_e11484 / locals.var_gc3_i);
        (assign12810_e11486,)
    } else {
        (locals.var_gcq,)
    }
};
        locals.var_gcq = assign12810_e11488;
        locals.var_gcq_rv = 0.0;

        locals.var_gcqov = 0.0;
        locals.var_gcqov_rv = 0.0;

        let assign12830_e11492: f64 = if locals.var_gc3ov_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard168 = assign12830_e11492;
        locals.var_guard168_rv = 0.0;

        let (assign12840_e11501,) = {
    if (locals.var_guard168 != 0.0) {
        let assign12840_e11495: f64 = (-0.495);
        let assign12840_e11497: f64 = (assign12840_e11495 * locals.var_gc2ov_i);
        let assign12840_e11499: f64 = (assign12840_e11497 / locals.var_gc3ov_i);
        (assign12840_e11499,)
    } else {
        (locals.var_gcqov,)
    }
};
        locals.var_gcqov = assign12840_e11501;
        locals.var_gcqov_rv = 0.0;

        let assign12850_e11504: f64 = if locals.var_gc3ovd_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard169 = assign12850_e11504;
        locals.var_guard169_rv = 0.0;

        let (assign12860_e11513,) = {
    if (locals.var_guard169 != 0.0) {
        let assign12860_e11507: f64 = (-0.495);
        let assign12860_e11509: f64 = (assign12860_e11507 * locals.var_gc2ovd_i);
        let assign12860_e11511: f64 = (assign12860_e11509 / locals.var_gc3ovd_i);
        (assign12860_e11511,)
    } else {
        (locals.var_gcqovd,)
    }
};
        locals.var_gcqovd = assign12860_e11513;
        locals.var_gcqovd_rv = 0.0;

        let assign12870_e11516: f64 = (locals.var_rta).powf(locals.var_stig_i);
        locals.var_tf_ig = assign12870_e11516;
        locals.var_tf_ig_rv = 0.0;

        let assign12880_e11519: f64 = (locals.var_iginv_i * locals.var_tf_ig);
        locals.var_iginv_i = assign12880_e11519;
        locals.var_iginv_i_rv = 0.0;

        let assign12890_e11522: f64 = (locals.var_igov_i * locals.var_tf_ig);
        locals.var_igov_i = assign12890_e11522;
        locals.var_igov_i_rv = 0.0;

        let assign12900_e11525: f64 = (locals.var_igovd_i * locals.var_tf_ig);
        locals.var_igovd_i = assign12900_e11525;
        locals.var_igovd_i_rv = 0.0;

        let assign12930_e11543: f64 = (locals.var_stbgidl_i * locals.var_delta);
        let assign12930_e11544: f64 = (1.0 + assign12930_e11543);
        let (assign12930_e11553,) = {
    if (assign12930_e11544 > 0.0) {
        let assign12930_e11550: f64 = (locals.var_stbgidl_i * locals.var_delta);
        let assign12930_e11551: f64 = (1.0 + assign12930_e11550);
        (assign12930_e11551,)
    } else {
        (0.0,)
    }
};
        locals.var_b_fact = assign12930_e11553;
        locals.var_b_fact_rv = 0.0;

        let assign12940_e11556: f64 = (locals.var_bgidl_i * locals.var_b_fact);
        locals.var_bgidl_t = assign12940_e11556;
        locals.var_bgidl_t_rv = 0.0;

        let assign12950_e11559: f64 = (locals.var_bgidl_t * locals.var_toxov_i);
        let assign12950_e11561: f64 = (assign12950_e11559 * 500000000.0);
        locals.var_bgidls = assign12950_e11561;
        locals.var_bgidls_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_15(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        locals: &mut StampLocals,
    ) {
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let assign12960_e11565: f64 = (locals.var_stbgidld_i * locals.var_delta);
        let assign12960_e11566: f64 = (1.0 + assign12960_e11565);
        let (assign12960_e11575,) = {
    if (assign12960_e11566 > 0.0) {
        let assign12960_e11572: f64 = (locals.var_stbgidld_i * locals.var_delta);
        let assign12960_e11573: f64 = (1.0 + assign12960_e11572);
        (assign12960_e11573,)
    } else {
        (0.0,)
    }
};
        locals.var_b_fact = assign12960_e11575;
        locals.var_b_fact_rv = 0.0;

        let assign12970_e11578: f64 = (locals.var_bgidld_i * locals.var_b_fact);
        locals.var_bgidld_t = assign12970_e11578;
        locals.var_bgidld_t_rv = 0.0;

        let assign12980_e11581: f64 = (locals.var_bgidld_t * locals.var_toxovd_i);
        let assign12980_e11583: f64 = (assign12980_e11581 * 500000000.0);
        locals.var_bgidlds = assign12980_e11583;
        locals.var_bgidlds_rv = 0.0;

        locals.var_vinr_max = 0.0;
        locals.var_vinr_max_rv = 0.0;

        let assign13000_e11587: f64 = if locals.var_fcinracc_i > 1e-10 { 1.0 } else { 0.0 };
        locals.var_guard170 = assign13000_e11587;
        locals.var_guard170_rv = 0.0;

        let (assign13010_e11593,) = {
    if (locals.var_guard170 != 0.0) {
        let assign13010_e11591: f64 = (0.75 / locals.var_fcinracc_i);
        (assign13010_e11591,)
    } else {
        (locals.var_vinr_max,)
    }
};
        locals.var_vinr_max = assign13010_e11593;
        locals.var_vinr_max_rv = 0.0;

        let assign13020_e11596: f64 = (locals.var_axinr_i * locals.var_axinr_i);
        locals.var_ainr = assign13020_e11596;
        locals.var_ainr_rv = 0.0;

        locals.var_temp__blk1038 = 0.0;
        locals.var_temp__blk1038_dn5 = 0.0;
        locals.var_temp__blk1038_dn6 = 0.0;
        locals.var_temp__blk1038_dn7 = 0.0;
        locals.var_temp__blk1038_dn8 = 0.0;
        locals.var_temp__blk1038_dn12 = 0.0;
        locals.var_temp__blk1038_dn13 = 0.0;
        locals.var_temp__blk1038_dn14 = 0.0;
        locals.var_temp__blk1038_dn15 = 0.0;
        locals.var_temp__blk1038_dn16 = 0.0;
        locals.var_temp__blk1038_dn17 = 0.0;
        locals.var_temp__blk1038_dn18 = 0.0;
        locals.var_temp__blk1038_dn19 = 0.0;
        locals.var_temp__blk1038_dn20 = 0.0;
        locals.var_temp__blk1038_rv = 0.0;

        locals.var_temp1 = 0.0;
        locals.var_temp1_dn5 = 0.0;
        locals.var_temp1_dn6 = 0.0;
        locals.var_temp1_dn7 = 0.0;
        locals.var_temp1_dn8 = 0.0;
        locals.var_temp1_dn12 = 0.0;
        locals.var_temp1_dn13 = 0.0;
        locals.var_temp1_dn14 = 0.0;
        locals.var_temp1_dn15 = 0.0;
        locals.var_temp1_dn16 = 0.0;
        locals.var_temp1_dn17 = 0.0;
        locals.var_temp1_dn18 = 0.0;
        locals.var_temp1_dn19 = 0.0;
        locals.var_temp1_dn20 = 0.0;
        locals.var_temp1_rv = 0.0;

        locals.var_temp2 = 0.0;
        locals.var_temp2_dn5 = 0.0;
        locals.var_temp2_dn6 = 0.0;
        locals.var_temp2_dn7 = 0.0;
        locals.var_temp2_dn8 = 0.0;
        locals.var_temp2_dn12 = 0.0;
        locals.var_temp2_dn13 = 0.0;
        locals.var_temp2_dn14 = 0.0;
        locals.var_temp2_dn15 = 0.0;
        locals.var_temp2_dn16 = 0.0;
        locals.var_temp2_dn17 = 0.0;
        locals.var_temp2_dn18 = 0.0;
        locals.var_temp2_dn19 = 0.0;
        locals.var_temp2_dn20 = 0.0;
        locals.var_temp2_rv = 0.0;

        locals.var_pd = 1.0;
        locals.var_pd_dn5 = 0.0;
        locals.var_pd_dn6 = 0.0;
        locals.var_pd_dn7 = 0.0;
        locals.var_pd_dn8 = 0.0;
        locals.var_pd_dn12 = 0.0;
        locals.var_pd_dn13 = 0.0;
        locals.var_pd_dn14 = 0.0;
        locals.var_pd_dn15 = 0.0;
        locals.var_pd_dn16 = 0.0;
        locals.var_pd_dn17 = 0.0;
        locals.var_pd_dn18 = 0.0;
        locals.var_pd_dn19 = 0.0;
        locals.var_pd_dn20 = 0.0;
        locals.var_pd_rv = 0.0;

        locals.var_ym = 0.0;
        locals.var_ym_dn5 = 0.0;
        locals.var_ym_dn6 = 0.0;
        locals.var_ym_dn7 = 0.0;
        locals.var_ym_dn8 = 0.0;
        locals.var_ym_dn12 = 0.0;
        locals.var_ym_dn13 = 0.0;
        locals.var_ym_dn14 = 0.0;
        locals.var_ym_dn15 = 0.0;
        locals.var_ym_dn16 = 0.0;
        locals.var_ym_dn17 = 0.0;
        locals.var_ym_dn18 = 0.0;
        locals.var_ym_dn19 = 0.0;
        locals.var_ym_dn20 = 0.0;
        locals.var_ym_rv = 0.0;

        let assign40530_e53716: f64 = 1.0;
        let assign40530_e53717: f64 = if locals.var_chnl_type == assign40530_e53716 { 1.0 } else { 0.0 };
        locals.var_guard1113 = assign40530_e53717;
        locals.var_guard1113_rv = 0.0;

        let (assign40540_e53721, assign40540_e53721_d_n5, assign40540_e53721_d_n6, assign40540_e53721_d_n7,) = {
    if (locals.var_guard1113 != 0.0) {
        ((nv5 - nv6), 1.0, -1.0, 0.0,)
    } else {
        (locals.var_v_gs, locals.var_v_gs_dn5, locals.var_v_gs_dn6, locals.var_v_gs_dn7,)
    }
};
        locals.var_v_gs = assign40540_e53721;
        locals.var_v_gs_dn5 = assign40540_e53721_d_n5;
        locals.var_v_gs_dn6 = assign40540_e53721_d_n6;
        locals.var_v_gs_dn7 = assign40540_e53721_d_n7;
        locals.var_v_gs_rv = 0.0;

        let (assign40550_e53725, assign40550_e53725_d_n6, assign40550_e53725_d_n7,) = {
    if (locals.var_guard1113 != 0.0) {
        ((nv7 - nv6), -1.0, 1.0,)
    } else {
        (locals.var_v_ds, locals.var_v_ds_dn6, locals.var_v_ds_dn7,)
    }
};
        locals.var_v_ds = assign40550_e53725;
        locals.var_v_ds_dn6 = assign40550_e53725_d_n6;
        locals.var_v_ds_dn7 = assign40550_e53725_d_n7;
        locals.var_v_ds_rv = 0.0;

        let (assign40560_e53729, assign40560_e53729_d_n6, assign40560_e53729_d_n7, assign40560_e53729_d_n8,) = {
    if (locals.var_guard1113 != 0.0) {
        ((nv6 - nv8), 1.0, 0.0, -1.0,)
    } else {
        (locals.var_v_sb, locals.var_v_sb_dn6, locals.var_v_sb_dn7, locals.var_v_sb_dn8,)
    }
};
        locals.var_v_sb = assign40560_e53729;
        locals.var_v_sb_dn6 = assign40560_e53729_d_n6;
        locals.var_v_sb_dn7 = assign40560_e53729_d_n7;
        locals.var_v_sb_dn8 = assign40560_e53729_d_n8;
        locals.var_v_sb_rv = 0.0;

        let (assign40590_e53745, assign40590_e53745_d_n5, assign40590_e53745_d_n6, assign40590_e53745_d_n7,) = {
    if (locals.var_guard1113 == 0.0) {
        let assign40590_e53743: f64 = (-(nv5 - nv6));
        (assign40590_e53743, (-1.0), 1.0, 0.0,)
    } else {
        (locals.var_v_gs, locals.var_v_gs_dn5, locals.var_v_gs_dn6, locals.var_v_gs_dn7,)
    }
};
        locals.var_v_gs = assign40590_e53745;
        locals.var_v_gs_dn5 = assign40590_e53745_d_n5;
        locals.var_v_gs_dn6 = assign40590_e53745_d_n6;
        locals.var_v_gs_dn7 = assign40590_e53745_d_n7;
        locals.var_v_gs_rv = 0.0;

        let (assign40600_e53751, assign40600_e53751_d_n6, assign40600_e53751_d_n7,) = {
    if (locals.var_guard1113 == 0.0) {
        let assign40600_e53749: f64 = (-(nv7 - nv6));
        (assign40600_e53749, 1.0, (-1.0),)
    } else {
        (locals.var_v_ds, locals.var_v_ds_dn6, locals.var_v_ds_dn7,)
    }
};
        locals.var_v_ds = assign40600_e53751;
        locals.var_v_ds_dn6 = assign40600_e53751_d_n6;
        locals.var_v_ds_dn7 = assign40600_e53751_d_n7;
        locals.var_v_ds_rv = 0.0;

        let (assign40610_e53757, assign40610_e53757_d_n6, assign40610_e53757_d_n7, assign40610_e53757_d_n8,) = {
    if (locals.var_guard1113 == 0.0) {
        let assign40610_e53755: f64 = (-(nv6 - nv8));
        (assign40610_e53755, (-1.0), 0.0, 1.0,)
    } else {
        (locals.var_v_sb, locals.var_v_sb_dn6, locals.var_v_sb_dn7, locals.var_v_sb_dn8,)
    }
};
        locals.var_v_sb = assign40610_e53757;
        locals.var_v_sb_dn6 = assign40610_e53757_d_n6;
        locals.var_v_sb_dn7 = assign40610_e53757_d_n7;
        locals.var_v_sb_dn8 = assign40610_e53757_d_n8;
        locals.var_v_sb_rv = 0.0;

        let assign40640_e53770: f64 = (locals.var_v_gs + locals.var_v_sb);
        locals.var_vgb = assign40640_e53770;
        locals.var_vgb_dn5 = locals.var_v_gs_dn5;
        locals.var_vgb_dn6 = (locals.var_v_gs_dn6 + locals.var_v_sb_dn6);
        locals.var_vgb_dn7 = (locals.var_v_gs_dn7 + locals.var_v_sb_dn7);
        locals.var_vgb_dn8 = locals.var_v_sb_dn8;
        locals.var_vgb_rv = 0.0;

        locals.var_vgsprime = locals.var_v_gs;
        locals.var_vgsprime_dn5 = locals.var_v_gs_dn5;
        locals.var_vgsprime_dn6 = locals.var_v_gs_dn6;
        locals.var_vgsprime_dn7 = locals.var_v_gs_dn7;
        locals.var_vgsprime_rv = 0.0;

        locals.var_vsbprime = locals.var_v_sb;
        locals.var_vsbprime_dn6 = locals.var_v_sb_dn6;
        locals.var_vsbprime_dn7 = locals.var_v_sb_dn7;
        locals.var_vsbprime_dn8 = locals.var_v_sb_dn8;
        locals.var_vsbprime_rv = 0.0;

        let assign40670_e53775: f64 = (locals.var_v_ds + locals.var_v_sb);
        locals.var_vdbprime = assign40670_e53775;
        locals.var_vdbprime_dn6 = (locals.var_v_ds_dn6 + locals.var_v_sb_dn6);
        locals.var_vdbprime_dn7 = (locals.var_v_ds_dn7 + locals.var_v_sb_dn7);
        locals.var_vdbprime_dn8 = locals.var_v_sb_dn8;
        locals.var_vdbprime_rv = 0.0;

        let assign40680_e53778: f64 = (locals.var_v_gs - locals.var_v_ds);
        locals.var_vgdprime = assign40680_e53778;
        locals.var_vgdprime_dn5 = locals.var_v_gs_dn5;
        locals.var_vgdprime_dn6 = (locals.var_v_gs_dn6 - locals.var_v_ds_dn6);
        locals.var_vgdprime_dn7 = (locals.var_v_gs_dn7 - locals.var_v_ds_dn7);
        locals.var_vgdprime_rv = 0.0;

        let assign40690_e53780: f64 = (-locals.var_vgsprime);
        let assign40690_e53782: f64 = (assign40690_e53780 * locals.var_inv_phita);
        locals.var_xgs_ov = assign40690_e53782;
        locals.var_xgs_ov_dn5 = ((-locals.var_vgsprime_dn5) * locals.var_inv_phita);
        locals.var_xgs_ov_dn6 = ((-locals.var_vgsprime_dn6) * locals.var_inv_phita);
        locals.var_xgs_ov_dn7 = ((-locals.var_vgsprime_dn7) * locals.var_inv_phita);
        locals.var_xgs_ov_rv = 0.0;

        let assign40700_e53784: f64 = (-locals.var_vgdprime);
        let assign40700_e53786: f64 = (assign40700_e53784 * locals.var_inv_phita);
        locals.var_xgd_ov = assign40700_e53786;
        locals.var_xgd_ov_dn5 = ((-locals.var_vgdprime_dn5) * locals.var_inv_phita);
        locals.var_xgd_ov_dn6 = ((-locals.var_vgdprime_dn6) * locals.var_inv_phita);
        locals.var_xgd_ov_dn7 = ((-locals.var_vgdprime_dn7) * locals.var_inv_phita);
        locals.var_xgd_ov_rv = 0.0;

        let assign40710_e53789: f64 = (locals.var_vgb - locals.var_vfb_t);
        let assign40710_e53790: f64 = (-assign40710_e53789);
        let assign40710_e53792: f64 = (assign40710_e53790 * locals.var_inv_phita);
        locals.var_xgb_ov = assign40710_e53792;
        locals.var_xgb_ov_dn5 = ((-locals.var_vgb_dn5) * locals.var_inv_phita);
        locals.var_xgb_ov_dn6 = ((-locals.var_vgb_dn6) * locals.var_inv_phita);
        locals.var_xgb_ov_dn7 = ((-locals.var_vgb_dn7) * locals.var_inv_phita);
        locals.var_xgb_ov_dn8 = ((-locals.var_vgb_dn8) * locals.var_inv_phita);
        locals.var_xgb_ov_rv = 0.0;

        locals.var_sigvds = 1.0;
        locals.var_sigvds_rv = 0.0;

        let assign40730_e53796: f64 = if locals.var_v_ds < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1114 = assign40730_e53796;
        locals.var_guard1114_rv = 0.0;

        let (assign40740_e53801,) = {
    if (locals.var_guard1114 != 0.0) {
        let assign40740_e53799: f64 = (-1.0);
        (assign40740_e53799,)
    } else {
        (locals.var_sigvds,)
    }
};
        locals.var_sigvds = assign40740_e53801;
        locals.var_sigvds_rv = 0.0;

        let (assign40750_e53807, assign40750_e53807_d_n5, assign40750_e53807_d_n6, assign40750_e53807_d_n7,) = {
    if (locals.var_guard1114 != 0.0) {
        let assign40750_e53805: f64 = (locals.var_v_gs - locals.var_v_ds);
        (assign40750_e53805, locals.var_v_gs_dn5, (locals.var_v_gs_dn6 - locals.var_v_ds_dn6), (locals.var_v_gs_dn7 - locals.var_v_ds_dn7),)
    } else {
        (locals.var_v_gs, locals.var_v_gs_dn5, locals.var_v_gs_dn6, locals.var_v_gs_dn7,)
    }
};
        locals.var_v_gs = assign40750_e53807;
        locals.var_v_gs_dn5 = assign40750_e53807_d_n5;
        locals.var_v_gs_dn6 = assign40750_e53807_d_n6;
        locals.var_v_gs_dn7 = assign40750_e53807_d_n7;
        locals.var_v_gs_rv = 0.0;

        let (assign40760_e53813, assign40760_e53813_d_n6, assign40760_e53813_d_n7, assign40760_e53813_d_n8,) = {
    if (locals.var_guard1114 != 0.0) {
        let assign40760_e53811: f64 = (locals.var_v_sb + locals.var_v_ds);
        (assign40760_e53811, (locals.var_v_sb_dn6 + locals.var_v_ds_dn6), (locals.var_v_sb_dn7 + locals.var_v_ds_dn7), locals.var_v_sb_dn8,)
    } else {
        (locals.var_v_sb, locals.var_v_sb_dn6, locals.var_v_sb_dn7, locals.var_v_sb_dn8,)
    }
};
        locals.var_v_sb = assign40760_e53813;
        locals.var_v_sb_dn6 = assign40760_e53813_d_n6;
        locals.var_v_sb_dn7 = assign40760_e53813_d_n7;
        locals.var_v_sb_dn8 = assign40760_e53813_d_n8;
        locals.var_v_sb_rv = 0.0;

        let (assign40770_e53818, assign40770_e53818_d_n6, assign40770_e53818_d_n7,) = {
    if (locals.var_guard1114 != 0.0) {
        let assign40770_e53816: f64 = (-locals.var_v_ds);
        (assign40770_e53816, (-locals.var_v_ds_dn6), (-locals.var_v_ds_dn7),)
    } else {
        (locals.var_v_ds, locals.var_v_ds_dn6, locals.var_v_ds_dn7,)
    }
};
        locals.var_v_ds = assign40770_e53818;
        locals.var_v_ds_dn6 = assign40770_e53818_d_n6;
        locals.var_v_ds_dn7 = assign40770_e53818_d_n7;
        locals.var_v_ds_rv = 0.0;

        let assign40780_e53821: f64 = (locals.var_v_ds + locals.var_v_sb);
        locals.var_v_db = assign40780_e53821;
        locals.var_v_db_dn6 = (locals.var_v_ds_dn6 + locals.var_v_sb_dn6);
        locals.var_v_db_dn7 = (locals.var_v_ds_dn7 + locals.var_v_sb_dn7);
        locals.var_v_db_dn8 = locals.var_v_sb_dn8;
        locals.var_v_db_rv = 0.0;

        let assign40790_e53824: f64 = (locals.var_v_ds * locals.var_v_ds);
        let assign40790_e53827: f64 = (locals.var_v_ds * locals.var_v_ds);
        let assign40790_e53829: f64 = (assign40790_e53827 + 0.01);
        let assign40790_e53830: f64 = (assign40790_e53829).sqrt();
        let assign40790_e53832: f64 = (assign40790_e53830 + 0.1);
        let assign40790_e53833: f64 = (assign40790_e53824 / assign40790_e53832);
        locals.var_vdsx = assign40790_e53833;
        locals.var_vdsx_dn6 = (((((locals.var_v_ds_dn6 * locals.var_v_ds) + (locals.var_v_ds * locals.var_v_ds_dn6)) * assign40790_e53832) - (assign40790_e53824 * (((locals.var_v_ds_dn6 * locals.var_v_ds) + (locals.var_v_ds * locals.var_v_ds_dn6)) / (2.0 * assign40790_e53830)))) / (assign40790_e53832 * assign40790_e53832));
        locals.var_vdsx_dn7 = (((((locals.var_v_ds_dn7 * locals.var_v_ds) + (locals.var_v_ds * locals.var_v_ds_dn7)) * assign40790_e53832) - (assign40790_e53824 * (((locals.var_v_ds_dn7 * locals.var_v_ds) + (locals.var_v_ds * locals.var_v_ds_dn7)) / (2.0 * assign40790_e53830)))) / (assign40790_e53832 * assign40790_e53832));
        locals.var_vdsx_rv = 0.0;

        let assign40800_e53837: f64 = (locals.var_v_db + locals.var_v_sb);
        let assign40800_e53840: f64 = (locals.var_v_db - locals.var_v_sb);
        let assign40800_e53843: f64 = (locals.var_v_db - locals.var_v_sb);
        let assign40800_e53844: f64 = (assign40800_e53840 * assign40800_e53843);
        let assign40800_e53846: f64 = (assign40800_e53844 + locals.var_bphi_dc);
        let assign40800_e53847: f64 = (assign40800_e53846).sqrt();
        let assign40800_e53848: f64 = (assign40800_e53837 - assign40800_e53847);
        let assign40800_e53849: f64 = (0.5 * assign40800_e53848);
        let assign40800_e53851: f64 = (assign40800_e53849 + locals.var_phix_dc);
        locals.var_v_xb = assign40800_e53851;
        locals.var_v_xb_dn6 = (0.5 * ((locals.var_v_db_dn6 + locals.var_v_sb_dn6) - ((((locals.var_v_db_dn6 - locals.var_v_sb_dn6) * assign40800_e53843) + (assign40800_e53840 * (locals.var_v_db_dn6 - locals.var_v_sb_dn6))) / (2.0 * assign40800_e53847))));
        locals.var_v_xb_dn7 = (0.5 * ((locals.var_v_db_dn7 + locals.var_v_sb_dn7) - ((((locals.var_v_db_dn7 - locals.var_v_sb_dn7) * assign40800_e53843) + (assign40800_e53840 * (locals.var_v_db_dn7 - locals.var_v_sb_dn7))) / (2.0 * assign40800_e53847))));
        locals.var_v_xb_dn8 = (0.5 * ((locals.var_v_db_dn8 + locals.var_v_sb_dn8) - ((((locals.var_v_db_dn8 - locals.var_v_sb_dn8) * assign40800_e53843) + (assign40800_e53840 * (locals.var_v_db_dn8 - locals.var_v_sb_dn8))) / (2.0 * assign40800_e53847))));
        locals.var_v_xb_rv = 0.0;

        locals.var_v_xb_dc_tmp = locals.var_v_xb;
        locals.var_v_xb_dc_tmp_dn6 = locals.var_v_xb_dn6;
        locals.var_v_xb_dc_tmp_dn7 = locals.var_v_xb_dn7;
        locals.var_v_xb_dc_tmp_dn8 = locals.var_v_xb_dn8;
        locals.var_v_xb_dc_tmp_rv = 0.0;

        let assign40820_e53857: f64 = locals.var_v_xb;
        let assign40820_e53860: f64 = locals.var_v_xb;
        let assign40820_e53863: f64 = locals.var_v_xb;
        let assign40820_e53864: f64 = (assign40820_e53860 * assign40820_e53863);
        let assign40820_e53866: f64 = (assign40820_e53864 + locals.var_aphi_dc);
        let assign40820_e53867: f64 = (assign40820_e53866).sqrt();
        let assign40820_e53868: f64 = (assign40820_e53857 - assign40820_e53867);
        let assign40820_e53869: f64 = (0.5 * assign40820_e53868);
        let assign40820_e53870: f64 = (locals.var_v_sb - assign40820_e53869);
        let assign40820_e53872: f64 = (assign40820_e53870 + locals.var_phix1_dc);
        locals.var_vsbstar_dc = assign40820_e53872;
        locals.var_vsbstar_dc_dn5 = 0.0;
        locals.var_vsbstar_dc_dn6 = (locals.var_v_sb_dn6 - (0.5 * (locals.var_v_xb_dn6 - (((locals.var_v_xb_dn6 * assign40820_e53863) + (assign40820_e53860 * locals.var_v_xb_dn6)) / (2.0 * assign40820_e53867)))));
        locals.var_vsbstar_dc_dn7 = (locals.var_v_sb_dn7 - (0.5 * (locals.var_v_xb_dn7 - (((locals.var_v_xb_dn7 * assign40820_e53863) + (assign40820_e53860 * locals.var_v_xb_dn7)) / (2.0 * assign40820_e53867)))));
        locals.var_vsbstar_dc_dn8 = (locals.var_v_sb_dn8 - (0.5 * (locals.var_v_xb_dn8 - (((locals.var_v_xb_dn8 * assign40820_e53863) + (assign40820_e53860 * locals.var_v_xb_dn8)) / (2.0 * assign40820_e53867)))));
        locals.var_vsbstar_dc_dn12 = 0.0;
        locals.var_vsbstar_dc_dn13 = 0.0;
        locals.var_vsbstar_dc_dn14 = 0.0;
        locals.var_vsbstar_dc_dn15 = 0.0;
        locals.var_vsbstar_dc_dn16 = 0.0;
        locals.var_vsbstar_dc_dn17 = 0.0;
        locals.var_vsbstar_dc_dn18 = 0.0;
        locals.var_vsbstar_dc_dn19 = 0.0;
        locals.var_vsbstar_dc_dn20 = 0.0;
        locals.var_vsbstar_dc_rv = 0.0;

        locals.var_vsbstar_dc_tmp = locals.var_vsbstar_dc;
        locals.var_vsbstar_dc_tmp_dn5 = locals.var_vsbstar_dc_dn5;
        locals.var_vsbstar_dc_tmp_dn6 = locals.var_vsbstar_dc_dn6;
        locals.var_vsbstar_dc_tmp_dn7 = locals.var_vsbstar_dc_dn7;
        locals.var_vsbstar_dc_tmp_dn8 = locals.var_vsbstar_dc_dn8;
        locals.var_vsbstar_dc_tmp_dn12 = locals.var_vsbstar_dc_dn12;
        locals.var_vsbstar_dc_tmp_dn13 = locals.var_vsbstar_dc_dn13;
        locals.var_vsbstar_dc_tmp_dn14 = locals.var_vsbstar_dc_dn14;
        locals.var_vsbstar_dc_tmp_dn15 = locals.var_vsbstar_dc_dn15;
        locals.var_vsbstar_dc_tmp_dn16 = locals.var_vsbstar_dc_dn16;
        locals.var_vsbstar_dc_tmp_dn17 = locals.var_vsbstar_dc_dn17;
        locals.var_vsbstar_dc_tmp_dn18 = locals.var_vsbstar_dc_dn18;
        locals.var_vsbstar_dc_tmp_dn19 = locals.var_vsbstar_dc_dn19;
        locals.var_vsbstar_dc_tmp_dn20 = locals.var_vsbstar_dc_dn20;
        locals.var_vsbstar_dc_tmp_rv = 0.0;

        locals.var_dvbstar_dc = 0.0;
        locals.var_dvbstar_dc_dn5 = 0.0;
        locals.var_dvbstar_dc_dn6 = 0.0;
        locals.var_dvbstar_dc_dn7 = 0.0;
        locals.var_dvbstar_dc_dn8 = 0.0;
        locals.var_dvbstar_dc_dn12 = 0.0;
        locals.var_dvbstar_dc_dn13 = 0.0;
        locals.var_dvbstar_dc_dn14 = 0.0;
        locals.var_dvbstar_dc_dn15 = 0.0;
        locals.var_dvbstar_dc_dn16 = 0.0;
        locals.var_dvbstar_dc_dn17 = 0.0;
        locals.var_dvbstar_dc_dn18 = 0.0;
        locals.var_dvbstar_dc_dn19 = 0.0;
        locals.var_dvbstar_dc_dn20 = 0.0;
        locals.var_dvbstar_dc_rv = 0.0;

        let assign40850_e53881: f64 = if ((p.p45 != 0.0) && (locals.var_gfacnud_i != 1.0)) { 1.0 } else { 0.0 };
        locals.var_guard1274 = assign40850_e53881;
        locals.var_guard1274_rv = 0.0;

        let (assign40860_e53891, assign40860_e53891_d_n5, assign40860_e53891_d_n6, assign40860_e53891_d_n7, assign40860_e53891_d_n8, assign40860_e53891_d_n12, assign40860_e53891_d_n13, assign40860_e53891_d_n14, assign40860_e53891_d_n15, assign40860_e53891_d_n16, assign40860_e53891_d_n17, assign40860_e53891_d_n18, assign40860_e53891_d_n19, assign40860_e53891_d_n20,) = {
    if (locals.var_guard1274 != 0.0) {
        let assign40860_e53887: f64 = (locals.var_v_ds - locals.var_vdsx);
        let assign40860_e53888: f64 = (0.5 * assign40860_e53887);
        let assign40860_e53889: f64 = (locals.var_vsbstar_dc + assign40860_e53888);
        (assign40860_e53889, locals.var_vsbstar_dc_dn5, (locals.var_vsbstar_dc_dn6 + (0.5 * (locals.var_v_ds_dn6 - locals.var_vdsx_dn6))), (locals.var_vsbstar_dc_dn7 + (0.5 * (locals.var_v_ds_dn7 - locals.var_vdsx_dn7))), locals.var_vsbstar_dc_dn8, locals.var_vsbstar_dc_dn12, locals.var_vsbstar_dc_dn13, locals.var_vsbstar_dc_dn14, locals.var_vsbstar_dc_dn15, locals.var_vsbstar_dc_dn16, locals.var_vsbstar_dc_dn17, locals.var_vsbstar_dc_dn18, locals.var_vsbstar_dc_dn19, locals.var_vsbstar_dc_dn20,)
    } else {
        (locals.var_vmb, locals.var_vmb_dn5, locals.var_vmb_dn6, locals.var_vmb_dn7, locals.var_vmb_dn8, locals.var_vmb_dn12, locals.var_vmb_dn13, locals.var_vmb_dn14, locals.var_vmb_dn15, locals.var_vmb_dn16, locals.var_vmb_dn17, locals.var_vmb_dn18, locals.var_vmb_dn19, locals.var_vmb_dn20,)
    }
};
        locals.var_vmb = assign40860_e53891;
        locals.var_vmb_dn5 = assign40860_e53891_d_n5;
        locals.var_vmb_dn6 = assign40860_e53891_d_n6;
        locals.var_vmb_dn7 = assign40860_e53891_d_n7;
        locals.var_vmb_dn8 = assign40860_e53891_d_n8;
        locals.var_vmb_dn12 = assign40860_e53891_d_n12;
        locals.var_vmb_dn13 = assign40860_e53891_d_n13;
        locals.var_vmb_dn14 = assign40860_e53891_d_n14;
        locals.var_vmb_dn15 = assign40860_e53891_d_n15;
        locals.var_vmb_dn16 = assign40860_e53891_d_n16;
        locals.var_vmb_dn17 = assign40860_e53891_d_n17;
        locals.var_vmb_dn18 = assign40860_e53891_d_n18;
        locals.var_vmb_dn19 = assign40860_e53891_d_n19;
        locals.var_vmb_dn20 = assign40860_e53891_d_n20;
        locals.var_vmb_rv = 0.0;

        let (assign40870_e53900, assign40870_e53900_d_n5, assign40870_e53900_d_n6, assign40870_e53900_d_n7, assign40870_e53900_d_n8, assign40870_e53900_d_n12, assign40870_e53900_d_n13, assign40870_e53900_d_n14, assign40870_e53900_d_n15, assign40870_e53900_d_n16, assign40870_e53900_d_n17, assign40870_e53900_d_n18, assign40870_e53900_d_n19, assign40870_e53900_d_n20,) = {
    if (locals.var_guard1274 != 0.0) {
        let assign40870_e53895: f64 = (locals.var_vmb + locals.var_phib_dc);
        let assign40870_e53896: f64 = (assign40870_e53895).sqrt();
        let assign40870_e53898: f64 = (assign40870_e53896 - locals.var_sqrt_phib_dc);
        (assign40870_e53898, (locals.var_vmb_dn5 / (2.0 * assign40870_e53896)), (locals.var_vmb_dn6 / (2.0 * assign40870_e53896)), (locals.var_vmb_dn7 / (2.0 * assign40870_e53896)), (locals.var_vmb_dn8 / (2.0 * assign40870_e53896)), (locals.var_vmb_dn12 / (2.0 * assign40870_e53896)), (locals.var_vmb_dn13 / (2.0 * assign40870_e53896)), (locals.var_vmb_dn14 / (2.0 * assign40870_e53896)), (locals.var_vmb_dn15 / (2.0 * assign40870_e53896)), (locals.var_vmb_dn16 / (2.0 * assign40870_e53896)), (locals.var_vmb_dn17 / (2.0 * assign40870_e53896)), (locals.var_vmb_dn18 / (2.0 * assign40870_e53896)), (locals.var_vmb_dn19 / (2.0 * assign40870_e53896)), (locals.var_vmb_dn20 / (2.0 * assign40870_e53896)),)
    } else {
        (locals.var_us, locals.var_us_dn5, locals.var_us_dn6, locals.var_us_dn7, locals.var_us_dn8, locals.var_us_dn12, locals.var_us_dn13, locals.var_us_dn14, locals.var_us_dn15, locals.var_us_dn16, locals.var_us_dn17, locals.var_us_dn18, locals.var_us_dn19, locals.var_us_dn20,)
    }
};
        locals.var_us = assign40870_e53900;
        locals.var_us_dn5 = assign40870_e53900_d_n5;
        locals.var_us_dn6 = assign40870_e53900_d_n6;
        locals.var_us_dn7 = assign40870_e53900_d_n7;
        locals.var_us_dn8 = assign40870_e53900_d_n8;
        locals.var_us_dn12 = assign40870_e53900_d_n12;
        locals.var_us_dn13 = assign40870_e53900_d_n13;
        locals.var_us_dn14 = assign40870_e53900_d_n14;
        locals.var_us_dn15 = assign40870_e53900_d_n15;
        locals.var_us_dn16 = assign40870_e53900_d_n16;
        locals.var_us_dn17 = assign40870_e53900_d_n17;
        locals.var_us_dn18 = assign40870_e53900_d_n18;
        locals.var_us_dn19 = assign40870_e53900_d_n19;
        locals.var_us_dn20 = assign40870_e53900_d_n20;
        locals.var_us_rv = 0.0;

        let (assign40880_e53912, assign40880_e53912_d_n5, assign40880_e53912_d_n6, assign40880_e53912_d_n7, assign40880_e53912_d_n8, assign40880_e53912_d_n12, assign40880_e53912_d_n13, assign40880_e53912_d_n14, assign40880_e53912_d_n15, assign40880_e53912_d_n16, assign40880_e53912_d_n17, assign40880_e53912_d_n18, assign40880_e53912_d_n19, assign40880_e53912_d_n20,) = {
    if (locals.var_guard1274 != 0.0) {
        let assign40880_e53905: f64 = (locals.var_us - locals.var_us1);
        let assign40880_e53906: f64 = (2.0 * assign40880_e53905);
        let assign40880_e53908: f64 = (assign40880_e53906 / locals.var_us21);
        let assign40880_e53910: f64 = (assign40880_e53908 - 1.0);
        (assign40880_e53910, ((2.0 * locals.var_us_dn5) / locals.var_us21), ((2.0 * locals.var_us_dn6) / locals.var_us21), ((2.0 * locals.var_us_dn7) / locals.var_us21), ((2.0 * locals.var_us_dn8) / locals.var_us21), ((2.0 * locals.var_us_dn12) / locals.var_us21), ((2.0 * locals.var_us_dn13) / locals.var_us21), ((2.0 * locals.var_us_dn14) / locals.var_us21), ((2.0 * locals.var_us_dn15) / locals.var_us21), ((2.0 * locals.var_us_dn16) / locals.var_us21), ((2.0 * locals.var_us_dn17) / locals.var_us21), ((2.0 * locals.var_us_dn18) / locals.var_us21), ((2.0 * locals.var_us_dn19) / locals.var_us21), ((2.0 * locals.var_us_dn20) / locals.var_us21),)
    } else {
        (locals.var_temp__blk1038, locals.var_temp__blk1038_dn5, locals.var_temp__blk1038_dn6, locals.var_temp__blk1038_dn7, locals.var_temp__blk1038_dn8, locals.var_temp__blk1038_dn12, locals.var_temp__blk1038_dn13, locals.var_temp__blk1038_dn14, locals.var_temp__blk1038_dn15, locals.var_temp__blk1038_dn16, locals.var_temp__blk1038_dn17, locals.var_temp__blk1038_dn18, locals.var_temp__blk1038_dn19, locals.var_temp__blk1038_dn20,)
    }
};
        locals.var_temp__blk1038 = assign40880_e53912;
        locals.var_temp__blk1038_dn5 = assign40880_e53912_d_n5;
        locals.var_temp__blk1038_dn6 = assign40880_e53912_d_n6;
        locals.var_temp__blk1038_dn7 = assign40880_e53912_d_n7;
        locals.var_temp__blk1038_dn8 = assign40880_e53912_d_n8;
        locals.var_temp__blk1038_dn12 = assign40880_e53912_d_n12;
        locals.var_temp__blk1038_dn13 = assign40880_e53912_d_n13;
        locals.var_temp__blk1038_dn14 = assign40880_e53912_d_n14;
        locals.var_temp__blk1038_dn15 = assign40880_e53912_d_n15;
        locals.var_temp__blk1038_dn16 = assign40880_e53912_d_n16;
        locals.var_temp__blk1038_dn17 = assign40880_e53912_d_n17;
        locals.var_temp__blk1038_dn18 = assign40880_e53912_d_n18;
        locals.var_temp__blk1038_dn19 = assign40880_e53912_d_n19;
        locals.var_temp__blk1038_dn20 = assign40880_e53912_d_n20;
        locals.var_temp__blk1038_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_16(
        locals: &mut StampLocals,
    ) {
        let (assign40890_e53933, assign40890_e53933_d_n5, assign40890_e53933_d_n6, assign40890_e53933_d_n7, assign40890_e53933_d_n8, assign40890_e53933_d_n12, assign40890_e53933_d_n13, assign40890_e53933_d_n14, assign40890_e53933_d_n15, assign40890_e53933_d_n16, assign40890_e53933_d_n17, assign40890_e53933_d_n18, assign40890_e53933_d_n19, assign40890_e53933_d_n20,) = {
    if (locals.var_guard1274 != 0.0) {
        let assign40890_e53918: f64 = (1.0 - locals.var_gfacnud_i);
        let assign40890_e53919: f64 = (0.25 * assign40890_e53918);
        let assign40890_e53921: f64 = (assign40890_e53919 * locals.var_us21);
        let assign40890_e53925: f64 = (locals.var_temp__blk1038 * locals.var_temp__blk1038);
        let assign40890_e53927: f64 = (assign40890_e53925 + 0.4804530139182);
        let assign40890_e53928: f64 = (assign40890_e53927).sqrt();
        let assign40890_e53929: f64 = (locals.var_temp__blk1038 + assign40890_e53928);
        let assign40890_e53930: f64 = (assign40890_e53921 * assign40890_e53929);
        let assign40890_e53931: f64 = (locals.var_us - assign40890_e53930);
        (assign40890_e53931, (locals.var_us_dn5 - (assign40890_e53921 * (locals.var_temp__blk1038_dn5 + (((locals.var_temp__blk1038_dn5 * locals.var_temp__blk1038) + (locals.var_temp__blk1038 * locals.var_temp__blk1038_dn5)) / (2.0 * assign40890_e53928))))), (locals.var_us_dn6 - (assign40890_e53921 * (locals.var_temp__blk1038_dn6 + (((locals.var_temp__blk1038_dn6 * locals.var_temp__blk1038) + (locals.var_temp__blk1038 * locals.var_temp__blk1038_dn6)) / (2.0 * assign40890_e53928))))), (locals.var_us_dn7 - (assign40890_e53921 * (locals.var_temp__blk1038_dn7 + (((locals.var_temp__blk1038_dn7 * locals.var_temp__blk1038) + (locals.var_temp__blk1038 * locals.var_temp__blk1038_dn7)) / (2.0 * assign40890_e53928))))), (locals.var_us_dn8 - (assign40890_e53921 * (locals.var_temp__blk1038_dn8 + (((locals.var_temp__blk1038_dn8 * locals.var_temp__blk1038) + (locals.var_temp__blk1038 * locals.var_temp__blk1038_dn8)) / (2.0 * assign40890_e53928))))), (locals.var_us_dn12 - (assign40890_e53921 * (locals.var_temp__blk1038_dn12 + (((locals.var_temp__blk1038_dn12 * locals.var_temp__blk1038) + (locals.var_temp__blk1038 * locals.var_temp__blk1038_dn12)) / (2.0 * assign40890_e53928))))), (locals.var_us_dn13 - (assign40890_e53921 * (locals.var_temp__blk1038_dn13 + (((locals.var_temp__blk1038_dn13 * locals.var_temp__blk1038) + (locals.var_temp__blk1038 * locals.var_temp__blk1038_dn13)) / (2.0 * assign40890_e53928))))), (locals.var_us_dn14 - (assign40890_e53921 * (locals.var_temp__blk1038_dn14 + (((locals.var_temp__blk1038_dn14 * locals.var_temp__blk1038) + (locals.var_temp__blk1038 * locals.var_temp__blk1038_dn14)) / (2.0 * assign40890_e53928))))), (locals.var_us_dn15 - (assign40890_e53921 * (locals.var_temp__blk1038_dn15 + (((locals.var_temp__blk1038_dn15 * locals.var_temp__blk1038) + (locals.var_temp__blk1038 * locals.var_temp__blk1038_dn15)) / (2.0 * assign40890_e53928))))), (locals.var_us_dn16 - (assign40890_e53921 * (locals.var_temp__blk1038_dn16 + (((locals.var_temp__blk1038_dn16 * locals.var_temp__blk1038) + (locals.var_temp__blk1038 * locals.var_temp__blk1038_dn16)) / (2.0 * assign40890_e53928))))), (locals.var_us_dn17 - (assign40890_e53921 * (locals.var_temp__blk1038_dn17 + (((locals.var_temp__blk1038_dn17 * locals.var_temp__blk1038) + (locals.var_temp__blk1038 * locals.var_temp__blk1038_dn17)) / (2.0 * assign40890_e53928))))), (locals.var_us_dn18 - (assign40890_e53921 * (locals.var_temp__blk1038_dn18 + (((locals.var_temp__blk1038_dn18 * locals.var_temp__blk1038) + (locals.var_temp__blk1038 * locals.var_temp__blk1038_dn18)) / (2.0 * assign40890_e53928))))), (locals.var_us_dn19 - (assign40890_e53921 * (locals.var_temp__blk1038_dn19 + (((locals.var_temp__blk1038_dn19 * locals.var_temp__blk1038) + (locals.var_temp__blk1038 * locals.var_temp__blk1038_dn19)) / (2.0 * assign40890_e53928))))), (locals.var_us_dn20 - (assign40890_e53921 * (locals.var_temp__blk1038_dn20 + (((locals.var_temp__blk1038_dn20 * locals.var_temp__blk1038) + (locals.var_temp__blk1038 * locals.var_temp__blk1038_dn20)) / (2.0 * assign40890_e53928))))),)
    } else {
        (locals.var_usnew, locals.var_usnew_dn5, locals.var_usnew_dn6, locals.var_usnew_dn7, locals.var_usnew_dn8, locals.var_usnew_dn12, locals.var_usnew_dn13, locals.var_usnew_dn14, locals.var_usnew_dn15, locals.var_usnew_dn16, locals.var_usnew_dn17, locals.var_usnew_dn18, locals.var_usnew_dn19, locals.var_usnew_dn20,)
    }
};
        locals.var_usnew = assign40890_e53933;
        locals.var_usnew_dn5 = assign40890_e53933_d_n5;
        locals.var_usnew_dn6 = assign40890_e53933_d_n6;
        locals.var_usnew_dn7 = assign40890_e53933_d_n7;
        locals.var_usnew_dn8 = assign40890_e53933_d_n8;
        locals.var_usnew_dn12 = assign40890_e53933_d_n12;
        locals.var_usnew_dn13 = assign40890_e53933_d_n13;
        locals.var_usnew_dn14 = assign40890_e53933_d_n14;
        locals.var_usnew_dn15 = assign40890_e53933_d_n15;
        locals.var_usnew_dn16 = assign40890_e53933_d_n16;
        locals.var_usnew_dn17 = assign40890_e53933_d_n17;
        locals.var_usnew_dn18 = assign40890_e53933_d_n18;
        locals.var_usnew_dn19 = assign40890_e53933_d_n19;
        locals.var_usnew_dn20 = assign40890_e53933_d_n20;
        locals.var_usnew_rv = 0.0;

        let (assign40900_e53945, assign40900_e53945_d_n5, assign40900_e53945_d_n6, assign40900_e53945_d_n7, assign40900_e53945_d_n8, assign40900_e53945_d_n12, assign40900_e53945_d_n13, assign40900_e53945_d_n14, assign40900_e53945_d_n15, assign40900_e53945_d_n16, assign40900_e53945_d_n17, assign40900_e53945_d_n18, assign40900_e53945_d_n19, assign40900_e53945_d_n20,) = {
    if (locals.var_guard1274 != 0.0) {
        let assign40900_e53937: f64 = (locals.var_usnew * locals.var_usnew);
        let assign40900_e53940: f64 = (2.0 * locals.var_sqrt_phib_dc);
        let assign40900_e53942: f64 = (assign40900_e53940 * locals.var_usnew);
        let assign40900_e53943: f64 = (assign40900_e53937 + assign40900_e53942);
        (assign40900_e53943, (((locals.var_usnew_dn5 * locals.var_usnew) + (locals.var_usnew * locals.var_usnew_dn5)) + (assign40900_e53940 * locals.var_usnew_dn5)), (((locals.var_usnew_dn6 * locals.var_usnew) + (locals.var_usnew * locals.var_usnew_dn6)) + (assign40900_e53940 * locals.var_usnew_dn6)), (((locals.var_usnew_dn7 * locals.var_usnew) + (locals.var_usnew * locals.var_usnew_dn7)) + (assign40900_e53940 * locals.var_usnew_dn7)), (((locals.var_usnew_dn8 * locals.var_usnew) + (locals.var_usnew * locals.var_usnew_dn8)) + (assign40900_e53940 * locals.var_usnew_dn8)), (((locals.var_usnew_dn12 * locals.var_usnew) + (locals.var_usnew * locals.var_usnew_dn12)) + (assign40900_e53940 * locals.var_usnew_dn12)), (((locals.var_usnew_dn13 * locals.var_usnew) + (locals.var_usnew * locals.var_usnew_dn13)) + (assign40900_e53940 * locals.var_usnew_dn13)), (((locals.var_usnew_dn14 * locals.var_usnew) + (locals.var_usnew * locals.var_usnew_dn14)) + (assign40900_e53940 * locals.var_usnew_dn14)), (((locals.var_usnew_dn15 * locals.var_usnew) + (locals.var_usnew * locals.var_usnew_dn15)) + (assign40900_e53940 * locals.var_usnew_dn15)), (((locals.var_usnew_dn16 * locals.var_usnew) + (locals.var_usnew * locals.var_usnew_dn16)) + (assign40900_e53940 * locals.var_usnew_dn16)), (((locals.var_usnew_dn17 * locals.var_usnew) + (locals.var_usnew * locals.var_usnew_dn17)) + (assign40900_e53940 * locals.var_usnew_dn17)), (((locals.var_usnew_dn18 * locals.var_usnew) + (locals.var_usnew * locals.var_usnew_dn18)) + (assign40900_e53940 * locals.var_usnew_dn18)), (((locals.var_usnew_dn19 * locals.var_usnew) + (locals.var_usnew * locals.var_usnew_dn19)) + (assign40900_e53940 * locals.var_usnew_dn19)), (((locals.var_usnew_dn20 * locals.var_usnew) + (locals.var_usnew * locals.var_usnew_dn20)) + (assign40900_e53940 * locals.var_usnew_dn20)),)
    } else {
        (locals.var_vmbnew, locals.var_vmbnew_dn5, locals.var_vmbnew_dn6, locals.var_vmbnew_dn7, locals.var_vmbnew_dn8, locals.var_vmbnew_dn12, locals.var_vmbnew_dn13, locals.var_vmbnew_dn14, locals.var_vmbnew_dn15, locals.var_vmbnew_dn16, locals.var_vmbnew_dn17, locals.var_vmbnew_dn18, locals.var_vmbnew_dn19, locals.var_vmbnew_dn20,)
    }
};
        locals.var_vmbnew = assign40900_e53945;
        locals.var_vmbnew_dn5 = assign40900_e53945_d_n5;
        locals.var_vmbnew_dn6 = assign40900_e53945_d_n6;
        locals.var_vmbnew_dn7 = assign40900_e53945_d_n7;
        locals.var_vmbnew_dn8 = assign40900_e53945_d_n8;
        locals.var_vmbnew_dn12 = assign40900_e53945_d_n12;
        locals.var_vmbnew_dn13 = assign40900_e53945_d_n13;
        locals.var_vmbnew_dn14 = assign40900_e53945_d_n14;
        locals.var_vmbnew_dn15 = assign40900_e53945_d_n15;
        locals.var_vmbnew_dn16 = assign40900_e53945_d_n16;
        locals.var_vmbnew_dn17 = assign40900_e53945_d_n17;
        locals.var_vmbnew_dn18 = assign40900_e53945_d_n18;
        locals.var_vmbnew_dn19 = assign40900_e53945_d_n19;
        locals.var_vmbnew_dn20 = assign40900_e53945_d_n20;
        locals.var_vmbnew_rv = 0.0;

        let (assign40910_e53955, assign40910_e53955_d_n5, assign40910_e53955_d_n6, assign40910_e53955_d_n7, assign40910_e53955_d_n8, assign40910_e53955_d_n12, assign40910_e53955_d_n13, assign40910_e53955_d_n14, assign40910_e53955_d_n15, assign40910_e53955_d_n16, assign40910_e53955_d_n17, assign40910_e53955_d_n18, assign40910_e53955_d_n19, assign40910_e53955_d_n20,) = {
    if (locals.var_guard1274 != 0.0) {
        let assign40910_e53951: f64 = (locals.var_v_ds - locals.var_vdsx);
        let assign40910_e53952: f64 = (0.5 * assign40910_e53951);
        let assign40910_e53953: f64 = (locals.var_vmbnew - assign40910_e53952);
        (assign40910_e53953, locals.var_vmbnew_dn5, (locals.var_vmbnew_dn6 - (0.5 * (locals.var_v_ds_dn6 - locals.var_vdsx_dn6))), (locals.var_vmbnew_dn7 - (0.5 * (locals.var_v_ds_dn7 - locals.var_vdsx_dn7))), locals.var_vmbnew_dn8, locals.var_vmbnew_dn12, locals.var_vmbnew_dn13, locals.var_vmbnew_dn14, locals.var_vmbnew_dn15, locals.var_vmbnew_dn16, locals.var_vmbnew_dn17, locals.var_vmbnew_dn18, locals.var_vmbnew_dn19, locals.var_vmbnew_dn20,)
    } else {
        (locals.var_vsbstar_dc, locals.var_vsbstar_dc_dn5, locals.var_vsbstar_dc_dn6, locals.var_vsbstar_dc_dn7, locals.var_vsbstar_dc_dn8, locals.var_vsbstar_dc_dn12, locals.var_vsbstar_dc_dn13, locals.var_vsbstar_dc_dn14, locals.var_vsbstar_dc_dn15, locals.var_vsbstar_dc_dn16, locals.var_vsbstar_dc_dn17, locals.var_vsbstar_dc_dn18, locals.var_vsbstar_dc_dn19, locals.var_vsbstar_dc_dn20,)
    }
};
        locals.var_vsbstar_dc = assign40910_e53955;
        locals.var_vsbstar_dc_dn5 = assign40910_e53955_d_n5;
        locals.var_vsbstar_dc_dn6 = assign40910_e53955_d_n6;
        locals.var_vsbstar_dc_dn7 = assign40910_e53955_d_n7;
        locals.var_vsbstar_dc_dn8 = assign40910_e53955_d_n8;
        locals.var_vsbstar_dc_dn12 = assign40910_e53955_d_n12;
        locals.var_vsbstar_dc_dn13 = assign40910_e53955_d_n13;
        locals.var_vsbstar_dc_dn14 = assign40910_e53955_d_n14;
        locals.var_vsbstar_dc_dn15 = assign40910_e53955_d_n15;
        locals.var_vsbstar_dc_dn16 = assign40910_e53955_d_n16;
        locals.var_vsbstar_dc_dn17 = assign40910_e53955_d_n17;
        locals.var_vsbstar_dc_dn18 = assign40910_e53955_d_n18;
        locals.var_vsbstar_dc_dn19 = assign40910_e53955_d_n19;
        locals.var_vsbstar_dc_dn20 = assign40910_e53955_d_n20;
        locals.var_vsbstar_dc_rv = 0.0;

        let (assign40920_e53961, assign40920_e53961_d_n5, assign40920_e53961_d_n6, assign40920_e53961_d_n7, assign40920_e53961_d_n8, assign40920_e53961_d_n12, assign40920_e53961_d_n13, assign40920_e53961_d_n14, assign40920_e53961_d_n15, assign40920_e53961_d_n16, assign40920_e53961_d_n17, assign40920_e53961_d_n18, assign40920_e53961_d_n19, assign40920_e53961_d_n20,) = {
    if (locals.var_guard1274 != 0.0) {
        let assign40920_e53959: f64 = (locals.var_vsbstar_dc_tmp - locals.var_vsbstar_dc);
        (assign40920_e53959, (locals.var_vsbstar_dc_tmp_dn5 - locals.var_vsbstar_dc_dn5), (locals.var_vsbstar_dc_tmp_dn6 - locals.var_vsbstar_dc_dn6), (locals.var_vsbstar_dc_tmp_dn7 - locals.var_vsbstar_dc_dn7), (locals.var_vsbstar_dc_tmp_dn8 - locals.var_vsbstar_dc_dn8), (locals.var_vsbstar_dc_tmp_dn12 - locals.var_vsbstar_dc_dn12), (locals.var_vsbstar_dc_tmp_dn13 - locals.var_vsbstar_dc_dn13), (locals.var_vsbstar_dc_tmp_dn14 - locals.var_vsbstar_dc_dn14), (locals.var_vsbstar_dc_tmp_dn15 - locals.var_vsbstar_dc_dn15), (locals.var_vsbstar_dc_tmp_dn16 - locals.var_vsbstar_dc_dn16), (locals.var_vsbstar_dc_tmp_dn17 - locals.var_vsbstar_dc_dn17), (locals.var_vsbstar_dc_tmp_dn18 - locals.var_vsbstar_dc_dn18), (locals.var_vsbstar_dc_tmp_dn19 - locals.var_vsbstar_dc_dn19), (locals.var_vsbstar_dc_tmp_dn20 - locals.var_vsbstar_dc_dn20),)
    } else {
        (locals.var_dvbstar_dc, locals.var_dvbstar_dc_dn5, locals.var_dvbstar_dc_dn6, locals.var_dvbstar_dc_dn7, locals.var_dvbstar_dc_dn8, locals.var_dvbstar_dc_dn12, locals.var_dvbstar_dc_dn13, locals.var_dvbstar_dc_dn14, locals.var_dvbstar_dc_dn15, locals.var_dvbstar_dc_dn16, locals.var_dvbstar_dc_dn17, locals.var_dvbstar_dc_dn18, locals.var_dvbstar_dc_dn19, locals.var_dvbstar_dc_dn20,)
    }
};
        locals.var_dvbstar_dc = assign40920_e53961;
        locals.var_dvbstar_dc_dn5 = assign40920_e53961_d_n5;
        locals.var_dvbstar_dc_dn6 = assign40920_e53961_d_n6;
        locals.var_dvbstar_dc_dn7 = assign40920_e53961_d_n7;
        locals.var_dvbstar_dc_dn8 = assign40920_e53961_d_n8;
        locals.var_dvbstar_dc_dn12 = assign40920_e53961_d_n12;
        locals.var_dvbstar_dc_dn13 = assign40920_e53961_d_n13;
        locals.var_dvbstar_dc_dn14 = assign40920_e53961_d_n14;
        locals.var_dvbstar_dc_dn15 = assign40920_e53961_d_n15;
        locals.var_dvbstar_dc_dn16 = assign40920_e53961_d_n16;
        locals.var_dvbstar_dc_dn17 = assign40920_e53961_d_n17;
        locals.var_dvbstar_dc_dn18 = assign40920_e53961_d_n18;
        locals.var_dvbstar_dc_dn19 = assign40920_e53961_d_n19;
        locals.var_dvbstar_dc_dn20 = assign40920_e53961_d_n20;
        locals.var_dvbstar_dc_rv = 0.0;

        locals.var_phib = locals.var_phib_dc;
        locals.var_phib_rv = 0.0;

        locals.var_aphi = locals.var_aphi_dc;
        locals.var_aphi_rv = 0.0;

        locals.var_g_0 = locals.var_g_0_dc;
        locals.var_g_0_rv = 0.0;

        locals.var_vsbstar = locals.var_vsbstar_dc;
        locals.var_vsbstar_dn5 = locals.var_vsbstar_dc_dn5;
        locals.var_vsbstar_dn6 = locals.var_vsbstar_dc_dn6;
        locals.var_vsbstar_dn7 = locals.var_vsbstar_dc_dn7;
        locals.var_vsbstar_dn8 = locals.var_vsbstar_dc_dn8;
        locals.var_vsbstar_dn12 = locals.var_vsbstar_dc_dn12;
        locals.var_vsbstar_dn13 = locals.var_vsbstar_dc_dn13;
        locals.var_vsbstar_dn14 = locals.var_vsbstar_dc_dn14;
        locals.var_vsbstar_dn15 = locals.var_vsbstar_dc_dn15;
        locals.var_vsbstar_dn16 = locals.var_vsbstar_dc_dn16;
        locals.var_vsbstar_dn17 = locals.var_vsbstar_dc_dn17;
        locals.var_vsbstar_dn18 = locals.var_vsbstar_dc_dn18;
        locals.var_vsbstar_dn19 = locals.var_vsbstar_dc_dn19;
        locals.var_vsbstar_dn20 = locals.var_vsbstar_dc_dn20;
        locals.var_vsbstar_rv = 0.0;

        locals.var_dvbstar = locals.var_dvbstar_dc;
        locals.var_dvbstar_dn5 = locals.var_dvbstar_dc_dn5;
        locals.var_dvbstar_dn6 = locals.var_dvbstar_dc_dn6;
        locals.var_dvbstar_dn7 = locals.var_dvbstar_dc_dn7;
        locals.var_dvbstar_dn8 = locals.var_dvbstar_dc_dn8;
        locals.var_dvbstar_dn12 = locals.var_dvbstar_dc_dn12;
        locals.var_dvbstar_dn13 = locals.var_dvbstar_dc_dn13;
        locals.var_dvbstar_dn14 = locals.var_dvbstar_dc_dn14;
        locals.var_dvbstar_dn15 = locals.var_dvbstar_dc_dn15;
        locals.var_dvbstar_dn16 = locals.var_dvbstar_dc_dn16;
        locals.var_dvbstar_dn17 = locals.var_dvbstar_dc_dn17;
        locals.var_dvbstar_dn18 = locals.var_dvbstar_dc_dn18;
        locals.var_dvbstar_dn19 = locals.var_dvbstar_dc_dn19;
        locals.var_dvbstar_dn20 = locals.var_dvbstar_dc_dn20;
        locals.var_dvbstar_rv = 0.0;

        locals.var_thesatloc = locals.var_thesat_t;
        locals.var_thesatloc_rv = 0.0;

        locals.var_arloc = locals.var_ar;
        locals.var_arloc_rv = 0.0;

        let assign41000_e53971: f64 = (locals.var_vgb - locals.var_dvbstar);
        let assign41000_e53973: f64 = (assign41000_e53971 - locals.var_vfb_t);
        locals.var_vgb1 = assign41000_e53973;
        locals.var_vgb1_dn5 = (locals.var_vgb_dn5 - locals.var_dvbstar_dn5);
        locals.var_vgb1_dn6 = (locals.var_vgb_dn6 - locals.var_dvbstar_dn6);
        locals.var_vgb1_dn7 = (locals.var_vgb_dn7 - locals.var_dvbstar_dn7);
        locals.var_vgb1_dn8 = (locals.var_vgb_dn8 - locals.var_dvbstar_dn8);
        locals.var_vgb1_dn12 = (-locals.var_dvbstar_dn12);
        locals.var_vgb1_dn13 = (-locals.var_dvbstar_dn13);
        locals.var_vgb1_dn14 = (-locals.var_dvbstar_dn14);
        locals.var_vgb1_dn15 = (-locals.var_dvbstar_dn15);
        locals.var_vgb1_dn16 = (-locals.var_dvbstar_dn16);
        locals.var_vgb1_dn17 = (-locals.var_dvbstar_dn17);
        locals.var_vgb1_dn18 = (-locals.var_dvbstar_dn18);
        locals.var_vgb1_dn19 = (-locals.var_dvbstar_dn19);
        locals.var_vgb1_dn20 = (-locals.var_dvbstar_dn20);
        locals.var_vgb1_rv = 0.0;

        let assign41010_e53978: f64 = (locals.var_v_ds - locals.var_vdsx);
        let assign41010_e53979: f64 = (0.5 * assign41010_e53978);
        let assign41010_e53980: f64 = (locals.var_vsbstar + assign41010_e53979);
        locals.var_vsbx = assign41010_e53980;
        locals.var_vsbx_dn5 = locals.var_vsbstar_dn5;
        locals.var_vsbx_dn6 = (locals.var_vsbstar_dn6 + (0.5 * (locals.var_v_ds_dn6 - locals.var_vdsx_dn6)));
        locals.var_vsbx_dn7 = (locals.var_vsbstar_dn7 + (0.5 * (locals.var_v_ds_dn7 - locals.var_vdsx_dn7)));
        locals.var_vsbx_dn8 = locals.var_vsbstar_dn8;
        locals.var_vsbx_dn12 = locals.var_vsbstar_dn12;
        locals.var_vsbx_dn13 = locals.var_vsbstar_dn13;
        locals.var_vsbx_dn14 = locals.var_vsbstar_dn14;
        locals.var_vsbx_dn15 = locals.var_vsbstar_dn15;
        locals.var_vsbx_dn16 = locals.var_vsbstar_dn16;
        locals.var_vsbx_dn17 = locals.var_vsbstar_dn17;
        locals.var_vsbx_dn18 = locals.var_vsbstar_dn18;
        locals.var_vsbx_dn19 = locals.var_vsbstar_dn19;
        locals.var_vsbx_dn20 = locals.var_vsbstar_dn20;
        locals.var_vsbx_rv = 0.0;

        locals.var_dctg = 1.0;
        locals.var_dctg_dn5 = 0.0;
        locals.var_dctg_dn6 = 0.0;
        locals.var_dctg_dn7 = 0.0;
        locals.var_dctg_dn8 = 0.0;
        locals.var_dctg_dn12 = 0.0;
        locals.var_dctg_dn13 = 0.0;
        locals.var_dctg_dn14 = 0.0;
        locals.var_dctg_dn15 = 0.0;
        locals.var_dctg_dn16 = 0.0;
        locals.var_dctg_dn17 = 0.0;
        locals.var_dctg_dn18 = 0.0;
        locals.var_dctg_dn19 = 0.0;
        locals.var_dctg_dn20 = 0.0;
        locals.var_dctg_rv = 0.0;

        let assign41030_e53984: f64 = if locals.var_ctg_i > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1275 = assign41030_e53984;
        locals.var_guard1275_rv = 0.0;

        let (assign41040_e53990,) = {
    if (locals.var_guard1275 != 0.0) {
        let assign41040_e53988: f64 = (locals.var_phib * locals.var_inv_phit);
        (assign41040_e53988,)
    } else {
        (locals.var_xbct,)
    }
};
        locals.var_xbct = assign41040_e53990;
        locals.var_xbct_rv = 0.0;

        let (assign41050_e53996, assign41050_e53996_d_n5, assign41050_e53996_d_n6, assign41050_e53996_d_n7, assign41050_e53996_d_n8, assign41050_e53996_d_n12, assign41050_e53996_d_n13, assign41050_e53996_d_n14, assign41050_e53996_d_n15, assign41050_e53996_d_n16, assign41050_e53996_d_n17, assign41050_e53996_d_n18, assign41050_e53996_d_n19, assign41050_e53996_d_n20,) = {
    if (locals.var_guard1275 != 0.0) {
        let assign41050_e53994: f64 = (locals.var_vsbx * locals.var_inv_phit);
        (assign41050_e53994, (locals.var_vsbx_dn5 * locals.var_inv_phit), (locals.var_vsbx_dn6 * locals.var_inv_phit), (locals.var_vsbx_dn7 * locals.var_inv_phit), (locals.var_vsbx_dn8 * locals.var_inv_phit), (locals.var_vsbx_dn12 * locals.var_inv_phit), (locals.var_vsbx_dn13 * locals.var_inv_phit), (locals.var_vsbx_dn14 * locals.var_inv_phit), (locals.var_vsbx_dn15 * locals.var_inv_phit), (locals.var_vsbx_dn16 * locals.var_inv_phit), (locals.var_vsbx_dn17 * locals.var_inv_phit), (locals.var_vsbx_dn18 * locals.var_inv_phit), (locals.var_vsbx_dn19 * locals.var_inv_phit), (locals.var_vsbx_dn20 * locals.var_inv_phit),)
    } else {
        (locals.var_xsbstar, locals.var_xsbstar_dn5, locals.var_xsbstar_dn6, locals.var_xsbstar_dn7, locals.var_xsbstar_dn8, locals.var_xsbstar_dn12, locals.var_xsbstar_dn13, locals.var_xsbstar_dn14, locals.var_xsbstar_dn15, locals.var_xsbstar_dn16, locals.var_xsbstar_dn17, locals.var_xsbstar_dn18, locals.var_xsbstar_dn19, locals.var_xsbstar_dn20,)
    }
};
        locals.var_xsbstar = assign41050_e53996;
        locals.var_xsbstar_dn5 = assign41050_e53996_d_n5;
        locals.var_xsbstar_dn6 = assign41050_e53996_d_n6;
        locals.var_xsbstar_dn7 = assign41050_e53996_d_n7;
        locals.var_xsbstar_dn8 = assign41050_e53996_d_n8;
        locals.var_xsbstar_dn12 = assign41050_e53996_d_n12;
        locals.var_xsbstar_dn13 = assign41050_e53996_d_n13;
        locals.var_xsbstar_dn14 = assign41050_e53996_d_n14;
        locals.var_xsbstar_dn15 = assign41050_e53996_d_n15;
        locals.var_xsbstar_dn16 = assign41050_e53996_d_n16;
        locals.var_xsbstar_dn17 = assign41050_e53996_d_n17;
        locals.var_xsbstar_dn18 = assign41050_e53996_d_n18;
        locals.var_xsbstar_dn19 = assign41050_e53996_d_n19;
        locals.var_xsbstar_dn20 = assign41050_e53996_d_n20;
        locals.var_xsbstar_rv = 0.0;

        let (assign41060_e54002, assign41060_e54002_d_n5, assign41060_e54002_d_n6, assign41060_e54002_d_n7, assign41060_e54002_d_n8, assign41060_e54002_d_n12, assign41060_e54002_d_n13, assign41060_e54002_d_n14, assign41060_e54002_d_n15, assign41060_e54002_d_n16, assign41060_e54002_d_n17, assign41060_e54002_d_n18, assign41060_e54002_d_n19, assign41060_e54002_d_n20,) = {
    if (locals.var_guard1275 != 0.0) {
        let assign41060_e54000: f64 = (locals.var_vgb1 * locals.var_inv_phit);
        (assign41060_e54000, (locals.var_vgb1_dn5 * locals.var_inv_phit), (locals.var_vgb1_dn6 * locals.var_inv_phit), (locals.var_vgb1_dn7 * locals.var_inv_phit), (locals.var_vgb1_dn8 * locals.var_inv_phit), (locals.var_vgb1_dn12 * locals.var_inv_phit), (locals.var_vgb1_dn13 * locals.var_inv_phit), (locals.var_vgb1_dn14 * locals.var_inv_phit), (locals.var_vgb1_dn15 * locals.var_inv_phit), (locals.var_vgb1_dn16 * locals.var_inv_phit), (locals.var_vgb1_dn17 * locals.var_inv_phit), (locals.var_vgb1_dn18 * locals.var_inv_phit), (locals.var_vgb1_dn19 * locals.var_inv_phit), (locals.var_vgb1_dn20 * locals.var_inv_phit),)
    } else {
        (locals.var_xgct, locals.var_xgct_dn5, locals.var_xgct_dn6, locals.var_xgct_dn7, locals.var_xgct_dn8, locals.var_xgct_dn12, locals.var_xgct_dn13, locals.var_xgct_dn14, locals.var_xgct_dn15, locals.var_xgct_dn16, locals.var_xgct_dn17, locals.var_xgct_dn18, locals.var_xgct_dn19, locals.var_xgct_dn20,)
    }
};
        locals.var_xgct = assign41060_e54002;
        locals.var_xgct_dn5 = assign41060_e54002_d_n5;
        locals.var_xgct_dn6 = assign41060_e54002_d_n6;
        locals.var_xgct_dn7 = assign41060_e54002_d_n7;
        locals.var_xgct_dn8 = assign41060_e54002_d_n8;
        locals.var_xgct_dn12 = assign41060_e54002_d_n12;
        locals.var_xgct_dn13 = assign41060_e54002_d_n13;
        locals.var_xgct_dn14 = assign41060_e54002_d_n14;
        locals.var_xgct_dn15 = assign41060_e54002_d_n15;
        locals.var_xgct_dn16 = assign41060_e54002_d_n16;
        locals.var_xgct_dn17 = assign41060_e54002_d_n17;
        locals.var_xgct_dn18 = assign41060_e54002_d_n18;
        locals.var_xgct_dn19 = assign41060_e54002_d_n19;
        locals.var_xgct_dn20 = assign41060_e54002_d_n20;
        locals.var_xgct_rv = 0.0;

        let (assign41070_e54013, assign41070_e54013_d_n5, assign41070_e54013_d_n6, assign41070_e54013_d_n7, assign41070_e54013_d_n8, assign41070_e54013_d_n12, assign41070_e54013_d_n13, assign41070_e54013_d_n14, assign41070_e54013_d_n15, assign41070_e54013_d_n16, assign41070_e54013_d_n17, assign41070_e54013_d_n18, assign41070_e54013_d_n19, assign41070_e54013_d_n20,) = {
    if (locals.var_guard1275 != 0.0) {
        let assign41070_e54007: f64 = (0.5 * locals.var_g_0);
        let assign41070_e54009: f64 = (locals.var_xbct).sqrt();
        let assign41070_e54010: f64 = (assign41070_e54007 / assign41070_e54009);
        let assign41070_e54011: f64 = (1.0 + assign41070_e54010);
        (assign41070_e54011, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp1, locals.var_temp1_dn5, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn12, locals.var_temp1_dn13, locals.var_temp1_dn14, locals.var_temp1_dn15, locals.var_temp1_dn16, locals.var_temp1_dn17, locals.var_temp1_dn18, locals.var_temp1_dn19, locals.var_temp1_dn20,)
    }
};
        locals.var_temp1 = assign41070_e54013;
        locals.var_temp1_dn5 = assign41070_e54013_d_n5;
        locals.var_temp1_dn6 = assign41070_e54013_d_n6;
        locals.var_temp1_dn7 = assign41070_e54013_d_n7;
        locals.var_temp1_dn8 = assign41070_e54013_d_n8;
        locals.var_temp1_dn12 = assign41070_e54013_d_n12;
        locals.var_temp1_dn13 = assign41070_e54013_d_n13;
        locals.var_temp1_dn14 = assign41070_e54013_d_n14;
        locals.var_temp1_dn15 = assign41070_e54013_d_n15;
        locals.var_temp1_dn16 = assign41070_e54013_d_n16;
        locals.var_temp1_dn17 = assign41070_e54013_d_n17;
        locals.var_temp1_dn18 = assign41070_e54013_d_n18;
        locals.var_temp1_dn19 = assign41070_e54013_d_n19;
        locals.var_temp1_dn20 = assign41070_e54013_d_n20;
        locals.var_temp1_rv = 0.0;

        let (assign41080_e54022, assign41080_e54022_d_n5, assign41080_e54022_d_n6, assign41080_e54022_d_n7, assign41080_e54022_d_n8, assign41080_e54022_d_n12, assign41080_e54022_d_n13, assign41080_e54022_d_n14, assign41080_e54022_d_n15, assign41080_e54022_d_n16, assign41080_e54022_d_n17, assign41080_e54022_d_n18, assign41080_e54022_d_n19, assign41080_e54022_d_n20,) = {
    if (locals.var_guard1275 != 0.0) {
        let assign41080_e54018: f64 = (locals.var_xbct).sqrt();
        let assign41080_e54019: f64 = (locals.var_g_0 * assign41080_e54018);
        let assign41080_e54020: f64 = (locals.var_xbct + assign41080_e54019);
        (assign41080_e54020, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp2, locals.var_temp2_dn5, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn12, locals.var_temp2_dn13, locals.var_temp2_dn14, locals.var_temp2_dn15, locals.var_temp2_dn16, locals.var_temp2_dn17, locals.var_temp2_dn18, locals.var_temp2_dn19, locals.var_temp2_dn20,)
    }
};
        locals.var_temp2 = assign41080_e54022;
        locals.var_temp2_dn5 = assign41080_e54022_d_n5;
        locals.var_temp2_dn6 = assign41080_e54022_d_n6;
        locals.var_temp2_dn7 = assign41080_e54022_d_n7;
        locals.var_temp2_dn8 = assign41080_e54022_d_n8;
        locals.var_temp2_dn12 = assign41080_e54022_d_n12;
        locals.var_temp2_dn13 = assign41080_e54022_d_n13;
        locals.var_temp2_dn14 = assign41080_e54022_d_n14;
        locals.var_temp2_dn15 = assign41080_e54022_d_n15;
        locals.var_temp2_dn16 = assign41080_e54022_d_n16;
        locals.var_temp2_dn17 = assign41080_e54022_d_n17;
        locals.var_temp2_dn18 = assign41080_e54022_d_n18;
        locals.var_temp2_dn19 = assign41080_e54022_d_n19;
        locals.var_temp2_dn20 = assign41080_e54022_d_n20;
        locals.var_temp2_rv = 0.0;

        let (assign41090_e54040, assign41090_e54040_d_n5, assign41090_e54040_d_n6, assign41090_e54040_d_n7, assign41090_e54040_d_n8, assign41090_e54040_d_n12, assign41090_e54040_d_n13, assign41090_e54040_d_n14, assign41090_e54040_d_n15, assign41090_e54040_d_n16, assign41090_e54040_d_n17, assign41090_e54040_d_n18, assign41090_e54040_d_n19, assign41090_e54040_d_n20,) = {
    if (locals.var_guard1275 != 0.0) {
        let assign41090_e54026: f64 = (locals.var_xgct - locals.var_temp2);
        let assign41090_e54028: f64 = (assign41090_e54026 / locals.var_temp1);
        let assign41090_e54031: f64 = (0.5 * locals.var_xbct);
        let assign41090_e54032: f64 = (assign41090_e54028 + assign41090_e54031);
        let assign41090_e54035: f64 = (1.0 + locals.var_ctb_i);
        let assign41090_e54037: f64 = (assign41090_e54035 * locals.var_xsbstar);
        let assign41090_e54038: f64 = (assign41090_e54032 - assign41090_e54037);
        (assign41090_e54038, (((((locals.var_xgct_dn5 - locals.var_temp2_dn5) * locals.var_temp1) - (assign41090_e54026 * locals.var_temp1_dn5)) / (locals.var_temp1 * locals.var_temp1)) - (assign41090_e54035 * locals.var_xsbstar_dn5)), (((((locals.var_xgct_dn6 - locals.var_temp2_dn6) * locals.var_temp1) - (assign41090_e54026 * locals.var_temp1_dn6)) / (locals.var_temp1 * locals.var_temp1)) - (assign41090_e54035 * locals.var_xsbstar_dn6)), (((((locals.var_xgct_dn7 - locals.var_temp2_dn7) * locals.var_temp1) - (assign41090_e54026 * locals.var_temp1_dn7)) / (locals.var_temp1 * locals.var_temp1)) - (assign41090_e54035 * locals.var_xsbstar_dn7)), (((((locals.var_xgct_dn8 - locals.var_temp2_dn8) * locals.var_temp1) - (assign41090_e54026 * locals.var_temp1_dn8)) / (locals.var_temp1 * locals.var_temp1)) - (assign41090_e54035 * locals.var_xsbstar_dn8)), (((((locals.var_xgct_dn12 - locals.var_temp2_dn12) * locals.var_temp1) - (assign41090_e54026 * locals.var_temp1_dn12)) / (locals.var_temp1 * locals.var_temp1)) - (assign41090_e54035 * locals.var_xsbstar_dn12)), (((((locals.var_xgct_dn13 - locals.var_temp2_dn13) * locals.var_temp1) - (assign41090_e54026 * locals.var_temp1_dn13)) / (locals.var_temp1 * locals.var_temp1)) - (assign41090_e54035 * locals.var_xsbstar_dn13)), (((((locals.var_xgct_dn14 - locals.var_temp2_dn14) * locals.var_temp1) - (assign41090_e54026 * locals.var_temp1_dn14)) / (locals.var_temp1 * locals.var_temp1)) - (assign41090_e54035 * locals.var_xsbstar_dn14)), (((((locals.var_xgct_dn15 - locals.var_temp2_dn15) * locals.var_temp1) - (assign41090_e54026 * locals.var_temp1_dn15)) / (locals.var_temp1 * locals.var_temp1)) - (assign41090_e54035 * locals.var_xsbstar_dn15)), (((((locals.var_xgct_dn16 - locals.var_temp2_dn16) * locals.var_temp1) - (assign41090_e54026 * locals.var_temp1_dn16)) / (locals.var_temp1 * locals.var_temp1)) - (assign41090_e54035 * locals.var_xsbstar_dn16)), (((((locals.var_xgct_dn17 - locals.var_temp2_dn17) * locals.var_temp1) - (assign41090_e54026 * locals.var_temp1_dn17)) / (locals.var_temp1 * locals.var_temp1)) - (assign41090_e54035 * locals.var_xsbstar_dn17)), (((((locals.var_xgct_dn18 - locals.var_temp2_dn18) * locals.var_temp1) - (assign41090_e54026 * locals.var_temp1_dn18)) / (locals.var_temp1 * locals.var_temp1)) - (assign41090_e54035 * locals.var_xsbstar_dn18)), (((((locals.var_xgct_dn19 - locals.var_temp2_dn19) * locals.var_temp1) - (assign41090_e54026 * locals.var_temp1_dn19)) / (locals.var_temp1 * locals.var_temp1)) - (assign41090_e54035 * locals.var_xsbstar_dn19)), (((((locals.var_xgct_dn20 - locals.var_temp2_dn20) * locals.var_temp1) - (assign41090_e54026 * locals.var_temp1_dn20)) / (locals.var_temp1 * locals.var_temp1)) - (assign41090_e54035 * locals.var_xsbstar_dn20)),)
    } else {
        (locals.var_xwict, locals.var_xwict_dn5, locals.var_xwict_dn6, locals.var_xwict_dn7, locals.var_xwict_dn8, locals.var_xwict_dn12, locals.var_xwict_dn13, locals.var_xwict_dn14, locals.var_xwict_dn15, locals.var_xwict_dn16, locals.var_xwict_dn17, locals.var_xwict_dn18, locals.var_xwict_dn19, locals.var_xwict_dn20,)
    }
};
        locals.var_xwict = assign41090_e54040;
        locals.var_xwict_dn5 = assign41090_e54040_d_n5;
        locals.var_xwict_dn6 = assign41090_e54040_d_n6;
        locals.var_xwict_dn7 = assign41090_e54040_d_n7;
        locals.var_xwict_dn8 = assign41090_e54040_d_n8;
        locals.var_xwict_dn12 = assign41090_e54040_d_n12;
        locals.var_xwict_dn13 = assign41090_e54040_d_n13;
        locals.var_xwict_dn14 = assign41090_e54040_d_n14;
        locals.var_xwict_dn15 = assign41090_e54040_d_n15;
        locals.var_xwict_dn16 = assign41090_e54040_d_n16;
        locals.var_xwict_dn17 = assign41090_e54040_d_n17;
        locals.var_xwict_dn18 = assign41090_e54040_d_n18;
        locals.var_xwict_dn19 = assign41090_e54040_d_n19;
        locals.var_xwict_dn20 = assign41090_e54040_d_n20;
        locals.var_xwict_rv = 0.0;

        let (assign41100_e54048,) = {
    if (locals.var_guard1275 != 0.0) {
        let assign41100_e54044: f64 = (0.5 * locals.var_xbct);
        let assign41100_e54046: f64 = (assign41100_e54044 + 2.0);
        (assign41100_e54046,)
    } else {
        (locals.var_xctmax,)
    }
};
        locals.var_xctmax = assign41100_e54048;
        locals.var_xctmax_rv = 0.0;

        let (assign41110_e54054, assign41110_e54054_d_n5, assign41110_e54054_d_n6, assign41110_e54054_d_n7, assign41110_e54054_d_n8, assign41110_e54054_d_n12, assign41110_e54054_d_n13, assign41110_e54054_d_n14, assign41110_e54054_d_n15, assign41110_e54054_d_n16, assign41110_e54054_d_n17, assign41110_e54054_d_n18, assign41110_e54054_d_n19, assign41110_e54054_d_n20,) = {
    if (locals.var_guard1275 != 0.0) {
        let assign41110_e54052: f64 = (locals.var_xbct + locals.var_xsbstar);
        (assign41110_e54052, locals.var_xsbstar_dn5, locals.var_xsbstar_dn6, locals.var_xsbstar_dn7, locals.var_xsbstar_dn8, locals.var_xsbstar_dn12, locals.var_xsbstar_dn13, locals.var_xsbstar_dn14, locals.var_xsbstar_dn15, locals.var_xsbstar_dn16, locals.var_xsbstar_dn17, locals.var_xsbstar_dn18, locals.var_xsbstar_dn19, locals.var_xsbstar_dn20,)
    } else {
        (locals.var_xnct, locals.var_xnct_dn5, locals.var_xnct_dn6, locals.var_xnct_dn7, locals.var_xnct_dn8, locals.var_xnct_dn12, locals.var_xnct_dn13, locals.var_xnct_dn14, locals.var_xnct_dn15, locals.var_xnct_dn16, locals.var_xnct_dn17, locals.var_xnct_dn18, locals.var_xnct_dn19, locals.var_xnct_dn20,)
    }
};
        locals.var_xnct = assign41110_e54054;
        locals.var_xnct_dn5 = assign41110_e54054_d_n5;
        locals.var_xnct_dn6 = assign41110_e54054_d_n6;
        locals.var_xnct_dn7 = assign41110_e54054_d_n7;
        locals.var_xnct_dn8 = assign41110_e54054_d_n8;
        locals.var_xnct_dn12 = assign41110_e54054_d_n12;
        locals.var_xnct_dn13 = assign41110_e54054_d_n13;
        locals.var_xnct_dn14 = assign41110_e54054_d_n14;
        locals.var_xnct_dn15 = assign41110_e54054_d_n15;
        locals.var_xnct_dn16 = assign41110_e54054_d_n16;
        locals.var_xnct_dn17 = assign41110_e54054_d_n17;
        locals.var_xnct_dn18 = assign41110_e54054_d_n18;
        locals.var_xnct_dn19 = assign41110_e54054_d_n19;
        locals.var_xnct_dn20 = assign41110_e54054_d_n20;
        locals.var_xnct_rv = 0.0;

        let (assign41120_e54075, assign41120_e54075_d_n5, assign41120_e54075_d_n6, assign41120_e54075_d_n7, assign41120_e54075_d_n8, assign41120_e54075_d_n12, assign41120_e54075_d_n13, assign41120_e54075_d_n14, assign41120_e54075_d_n15, assign41120_e54075_d_n16, assign41120_e54075_d_n17, assign41120_e54075_d_n18, assign41120_e54075_d_n19, assign41120_e54075_d_n20,) = {
    if (locals.var_guard1275 != 0.0) {
        let assign41120_e54058: f64 = (locals.var_xgct - locals.var_xnct);
        let assign41120_e54061: f64 = (locals.var_xnct).sqrt();
        let assign41120_e54062: f64 = (locals.var_g_0 * assign41120_e54061);
        let assign41120_e54063: f64 = (assign41120_e54058 - assign41120_e54062);
        let assign41120_e54067: f64 = (locals.var_xbct / locals.var_g_0);
        let assign41120_e54069: f64 = (locals.var_xbct).sqrt();
        let assign41120_e54070: f64 = (assign41120_e54067 + assign41120_e54069);
        let assign41120_e54071: f64 = (assign41120_e54070).ln();
        let assign41120_e54072: f64 = (2.0 * assign41120_e54071);
        let assign41120_e54073: f64 = (assign41120_e54063 - assign41120_e54072);
        (assign41120_e54073, ((locals.var_xgct_dn5 - locals.var_xnct_dn5) - (locals.var_g_0 * (locals.var_xnct_dn5 / (2.0 * assign41120_e54061)))), ((locals.var_xgct_dn6 - locals.var_xnct_dn6) - (locals.var_g_0 * (locals.var_xnct_dn6 / (2.0 * assign41120_e54061)))), ((locals.var_xgct_dn7 - locals.var_xnct_dn7) - (locals.var_g_0 * (locals.var_xnct_dn7 / (2.0 * assign41120_e54061)))), ((locals.var_xgct_dn8 - locals.var_xnct_dn8) - (locals.var_g_0 * (locals.var_xnct_dn8 / (2.0 * assign41120_e54061)))), ((locals.var_xgct_dn12 - locals.var_xnct_dn12) - (locals.var_g_0 * (locals.var_xnct_dn12 / (2.0 * assign41120_e54061)))), ((locals.var_xgct_dn13 - locals.var_xnct_dn13) - (locals.var_g_0 * (locals.var_xnct_dn13 / (2.0 * assign41120_e54061)))), ((locals.var_xgct_dn14 - locals.var_xnct_dn14) - (locals.var_g_0 * (locals.var_xnct_dn14 / (2.0 * assign41120_e54061)))), ((locals.var_xgct_dn15 - locals.var_xnct_dn15) - (locals.var_g_0 * (locals.var_xnct_dn15 / (2.0 * assign41120_e54061)))), ((locals.var_xgct_dn16 - locals.var_xnct_dn16) - (locals.var_g_0 * (locals.var_xnct_dn16 / (2.0 * assign41120_e54061)))), ((locals.var_xgct_dn17 - locals.var_xnct_dn17) - (locals.var_g_0 * (locals.var_xnct_dn17 / (2.0 * assign41120_e54061)))), ((locals.var_xgct_dn18 - locals.var_xnct_dn18) - (locals.var_g_0 * (locals.var_xnct_dn18 / (2.0 * assign41120_e54061)))), ((locals.var_xgct_dn19 - locals.var_xnct_dn19) - (locals.var_g_0 * (locals.var_xnct_dn19 / (2.0 * assign41120_e54061)))), ((locals.var_xgct_dn20 - locals.var_xnct_dn20) - (locals.var_g_0 * (locals.var_xnct_dn20 / (2.0 * assign41120_e54061)))),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn5, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn12, locals.var_temp1_dn13, locals.var_temp1_dn14, locals.var_temp1_dn15, locals.var_temp1_dn16, locals.var_temp1_dn17, locals.var_temp1_dn18, locals.var_temp1_dn19, locals.var_temp1_dn20,)
    }
};
        locals.var_temp1 = assign41120_e54075;
        locals.var_temp1_dn5 = assign41120_e54075_d_n5;
        locals.var_temp1_dn6 = assign41120_e54075_d_n6;
        locals.var_temp1_dn7 = assign41120_e54075_d_n7;
        locals.var_temp1_dn8 = assign41120_e54075_d_n8;
        locals.var_temp1_dn12 = assign41120_e54075_d_n12;
        locals.var_temp1_dn13 = assign41120_e54075_d_n13;
        locals.var_temp1_dn14 = assign41120_e54075_d_n14;
        locals.var_temp1_dn15 = assign41120_e54075_d_n15;
        locals.var_temp1_dn16 = assign41120_e54075_d_n16;
        locals.var_temp1_dn17 = assign41120_e54075_d_n17;
        locals.var_temp1_dn18 = assign41120_e54075_d_n18;
        locals.var_temp1_dn19 = assign41120_e54075_d_n19;
        locals.var_temp1_dn20 = assign41120_e54075_d_n20;
        locals.var_temp1_rv = 0.0;

        let (assign41130_e54083, assign41130_e54083_d_n5, assign41130_e54083_d_n6, assign41130_e54083_d_n7, assign41130_e54083_d_n8, assign41130_e54083_d_n12, assign41130_e54083_d_n13, assign41130_e54083_d_n14, assign41130_e54083_d_n15, assign41130_e54083_d_n16, assign41130_e54083_d_n17, assign41130_e54083_d_n18, assign41130_e54083_d_n19, assign41130_e54083_d_n20,) = {
    if (locals.var_guard1275 != 0.0) {
        let assign41130_e54079: f64 = (2.0 * locals.var_temp1);
        let assign41130_e54081: f64 = (assign41130_e54079 + locals.var_xctmax);
        (assign41130_e54081, (2.0 * locals.var_temp1_dn5), (2.0 * locals.var_temp1_dn6), (2.0 * locals.var_temp1_dn7), (2.0 * locals.var_temp1_dn8), (2.0 * locals.var_temp1_dn12), (2.0 * locals.var_temp1_dn13), (2.0 * locals.var_temp1_dn14), (2.0 * locals.var_temp1_dn15), (2.0 * locals.var_temp1_dn16), (2.0 * locals.var_temp1_dn17), (2.0 * locals.var_temp1_dn18), (2.0 * locals.var_temp1_dn19), (2.0 * locals.var_temp1_dn20),)
    } else {
        (locals.var_xmict, locals.var_xmict_dn5, locals.var_xmict_dn6, locals.var_xmict_dn7, locals.var_xmict_dn8, locals.var_xmict_dn12, locals.var_xmict_dn13, locals.var_xmict_dn14, locals.var_xmict_dn15, locals.var_xmict_dn16, locals.var_xmict_dn17, locals.var_xmict_dn18, locals.var_xmict_dn19, locals.var_xmict_dn20,)
    }
};
        locals.var_xmict = assign41130_e54083;
        locals.var_xmict_dn5 = assign41130_e54083_d_n5;
        locals.var_xmict_dn6 = assign41130_e54083_d_n6;
        locals.var_xmict_dn7 = assign41130_e54083_d_n7;
        locals.var_xmict_dn8 = assign41130_e54083_d_n8;
        locals.var_xmict_dn12 = assign41130_e54083_d_n12;
        locals.var_xmict_dn13 = assign41130_e54083_d_n13;
        locals.var_xmict_dn14 = assign41130_e54083_d_n14;
        locals.var_xmict_dn15 = assign41130_e54083_d_n15;
        locals.var_xmict_dn16 = assign41130_e54083_d_n16;
        locals.var_xmict_dn17 = assign41130_e54083_d_n17;
        locals.var_xmict_dn18 = assign41130_e54083_d_n18;
        locals.var_xmict_dn19 = assign41130_e54083_d_n19;
        locals.var_xmict_dn20 = assign41130_e54083_d_n20;
        locals.var_xmict_rv = 0.0;

        let (assign41140_e54102, assign41140_e54102_d_n5, assign41140_e54102_d_n6, assign41140_e54102_d_n7, assign41140_e54102_d_n8, assign41140_e54102_d_n12, assign41140_e54102_d_n13, assign41140_e54102_d_n14, assign41140_e54102_d_n15, assign41140_e54102_d_n16, assign41140_e54102_d_n17, assign41140_e54102_d_n18, assign41140_e54102_d_n19, assign41140_e54102_d_n20,) = {
    if (locals.var_guard1275 != 0.0) {
        let assign41140_e54088: f64 = (locals.var_xwict + locals.var_xmict);
        let assign41140_e54091: f64 = (locals.var_xwict - locals.var_xmict);
        let assign41140_e54094: f64 = (locals.var_xwict - locals.var_xmict);
        let assign41140_e54095: f64 = (assign41140_e54091 * assign41140_e54094);
        let assign41140_e54097: f64 = (assign41140_e54095 + 20.0);
        let assign41140_e54098: f64 = (assign41140_e54097).sqrt();
        let assign41140_e54099: f64 = (assign41140_e54088 + assign41140_e54098);
        let assign41140_e54100: f64 = (0.5 * assign41140_e54099);
        (assign41140_e54100, (0.5 * ((locals.var_xwict_dn5 + locals.var_xmict_dn5) + ((((locals.var_xwict_dn5 - locals.var_xmict_dn5) * assign41140_e54094) + (assign41140_e54091 * (locals.var_xwict_dn5 - locals.var_xmict_dn5))) / (2.0 * assign41140_e54098)))), (0.5 * ((locals.var_xwict_dn6 + locals.var_xmict_dn6) + ((((locals.var_xwict_dn6 - locals.var_xmict_dn6) * assign41140_e54094) + (assign41140_e54091 * (locals.var_xwict_dn6 - locals.var_xmict_dn6))) / (2.0 * assign41140_e54098)))), (0.5 * ((locals.var_xwict_dn7 + locals.var_xmict_dn7) + ((((locals.var_xwict_dn7 - locals.var_xmict_dn7) * assign41140_e54094) + (assign41140_e54091 * (locals.var_xwict_dn7 - locals.var_xmict_dn7))) / (2.0 * assign41140_e54098)))), (0.5 * ((locals.var_xwict_dn8 + locals.var_xmict_dn8) + ((((locals.var_xwict_dn8 - locals.var_xmict_dn8) * assign41140_e54094) + (assign41140_e54091 * (locals.var_xwict_dn8 - locals.var_xmict_dn8))) / (2.0 * assign41140_e54098)))), (0.5 * ((locals.var_xwict_dn12 + locals.var_xmict_dn12) + ((((locals.var_xwict_dn12 - locals.var_xmict_dn12) * assign41140_e54094) + (assign41140_e54091 * (locals.var_xwict_dn12 - locals.var_xmict_dn12))) / (2.0 * assign41140_e54098)))), (0.5 * ((locals.var_xwict_dn13 + locals.var_xmict_dn13) + ((((locals.var_xwict_dn13 - locals.var_xmict_dn13) * assign41140_e54094) + (assign41140_e54091 * (locals.var_xwict_dn13 - locals.var_xmict_dn13))) / (2.0 * assign41140_e54098)))), (0.5 * ((locals.var_xwict_dn14 + locals.var_xmict_dn14) + ((((locals.var_xwict_dn14 - locals.var_xmict_dn14) * assign41140_e54094) + (assign41140_e54091 * (locals.var_xwict_dn14 - locals.var_xmict_dn14))) / (2.0 * assign41140_e54098)))), (0.5 * ((locals.var_xwict_dn15 + locals.var_xmict_dn15) + ((((locals.var_xwict_dn15 - locals.var_xmict_dn15) * assign41140_e54094) + (assign41140_e54091 * (locals.var_xwict_dn15 - locals.var_xmict_dn15))) / (2.0 * assign41140_e54098)))), (0.5 * ((locals.var_xwict_dn16 + locals.var_xmict_dn16) + ((((locals.var_xwict_dn16 - locals.var_xmict_dn16) * assign41140_e54094) + (assign41140_e54091 * (locals.var_xwict_dn16 - locals.var_xmict_dn16))) / (2.0 * assign41140_e54098)))), (0.5 * ((locals.var_xwict_dn17 + locals.var_xmict_dn17) + ((((locals.var_xwict_dn17 - locals.var_xmict_dn17) * assign41140_e54094) + (assign41140_e54091 * (locals.var_xwict_dn17 - locals.var_xmict_dn17))) / (2.0 * assign41140_e54098)))), (0.5 * ((locals.var_xwict_dn18 + locals.var_xmict_dn18) + ((((locals.var_xwict_dn18 - locals.var_xmict_dn18) * assign41140_e54094) + (assign41140_e54091 * (locals.var_xwict_dn18 - locals.var_xmict_dn18))) / (2.0 * assign41140_e54098)))), (0.5 * ((locals.var_xwict_dn19 + locals.var_xmict_dn19) + ((((locals.var_xwict_dn19 - locals.var_xmict_dn19) * assign41140_e54094) + (assign41140_e54091 * (locals.var_xwict_dn19 - locals.var_xmict_dn19))) / (2.0 * assign41140_e54098)))), (0.5 * ((locals.var_xwict_dn20 + locals.var_xmict_dn20) + ((((locals.var_xwict_dn20 - locals.var_xmict_dn20) * assign41140_e54094) + (assign41140_e54091 * (locals.var_xwict_dn20 - locals.var_xmict_dn20))) / (2.0 * assign41140_e54098)))),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn5, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn12, locals.var_temp1_dn13, locals.var_temp1_dn14, locals.var_temp1_dn15, locals.var_temp1_dn16, locals.var_temp1_dn17, locals.var_temp1_dn18, locals.var_temp1_dn19, locals.var_temp1_dn20,)
    }
};
        locals.var_temp1 = assign41140_e54102;
        locals.var_temp1_dn5 = assign41140_e54102_d_n5;
        locals.var_temp1_dn6 = assign41140_e54102_d_n6;
        locals.var_temp1_dn7 = assign41140_e54102_d_n7;
        locals.var_temp1_dn8 = assign41140_e54102_d_n8;
        locals.var_temp1_dn12 = assign41140_e54102_d_n12;
        locals.var_temp1_dn13 = assign41140_e54102_d_n13;
        locals.var_temp1_dn14 = assign41140_e54102_d_n14;
        locals.var_temp1_dn15 = assign41140_e54102_d_n15;
        locals.var_temp1_dn16 = assign41140_e54102_d_n16;
        locals.var_temp1_dn17 = assign41140_e54102_d_n17;
        locals.var_temp1_dn18 = assign41140_e54102_d_n18;
        locals.var_temp1_dn19 = assign41140_e54102_d_n19;
        locals.var_temp1_dn20 = assign41140_e54102_d_n20;
        locals.var_temp1_rv = 0.0;

        let (assign41150_e54112, assign41150_e54112_d_n5, assign41150_e54112_d_n6, assign41150_e54112_d_n7, assign41150_e54112_d_n8, assign41150_e54112_d_n12, assign41150_e54112_d_n13, assign41150_e54112_d_n14, assign41150_e54112_d_n15, assign41150_e54112_d_n16, assign41150_e54112_d_n17, assign41150_e54112_d_n18, assign41150_e54112_d_n19, assign41150_e54112_d_n20,) = {
    if (locals.var_guard1275 != 0.0) {
        let assign41150_e54107: f64 = (locals.var_xgct - locals.var_xsbstar);
        let assign41150_e54108: f64 = (2.0 * assign41150_e54107);
        let assign41150_e54110: f64 = (assign41150_e54108 - locals.var_xctmax);
        (assign41150_e54110, (2.0 * (locals.var_xgct_dn5 - locals.var_xsbstar_dn5)), (2.0 * (locals.var_xgct_dn6 - locals.var_xsbstar_dn6)), (2.0 * (locals.var_xgct_dn7 - locals.var_xsbstar_dn7)), (2.0 * (locals.var_xgct_dn8 - locals.var_xsbstar_dn8)), (2.0 * (locals.var_xgct_dn12 - locals.var_xsbstar_dn12)), (2.0 * (locals.var_xgct_dn13 - locals.var_xsbstar_dn13)), (2.0 * (locals.var_xgct_dn14 - locals.var_xsbstar_dn14)), (2.0 * (locals.var_xgct_dn15 - locals.var_xsbstar_dn15)), (2.0 * (locals.var_xgct_dn16 - locals.var_xsbstar_dn16)), (2.0 * (locals.var_xgct_dn17 - locals.var_xsbstar_dn17)), (2.0 * (locals.var_xgct_dn18 - locals.var_xsbstar_dn18)), (2.0 * (locals.var_xgct_dn19 - locals.var_xsbstar_dn19)), (2.0 * (locals.var_xgct_dn20 - locals.var_xsbstar_dn20)),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn5, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn12, locals.var_temp2_dn13, locals.var_temp2_dn14, locals.var_temp2_dn15, locals.var_temp2_dn16, locals.var_temp2_dn17, locals.var_temp2_dn18, locals.var_temp2_dn19, locals.var_temp2_dn20,)
    }
};
        locals.var_temp2 = assign41150_e54112;
        locals.var_temp2_dn5 = assign41150_e54112_d_n5;
        locals.var_temp2_dn6 = assign41150_e54112_d_n6;
        locals.var_temp2_dn7 = assign41150_e54112_d_n7;
        locals.var_temp2_dn8 = assign41150_e54112_d_n8;
        locals.var_temp2_dn12 = assign41150_e54112_d_n12;
        locals.var_temp2_dn13 = assign41150_e54112_d_n13;
        locals.var_temp2_dn14 = assign41150_e54112_d_n14;
        locals.var_temp2_dn15 = assign41150_e54112_d_n15;
        locals.var_temp2_dn16 = assign41150_e54112_d_n16;
        locals.var_temp2_dn17 = assign41150_e54112_d_n17;
        locals.var_temp2_dn18 = assign41150_e54112_d_n18;
        locals.var_temp2_dn19 = assign41150_e54112_d_n19;
        locals.var_temp2_dn20 = assign41150_e54112_d_n20;
        locals.var_temp2_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_17(
        locals: &mut StampLocals,
    ) {
        let (assign41160_e54131, assign41160_e54131_d_n5, assign41160_e54131_d_n6, assign41160_e54131_d_n7, assign41160_e54131_d_n8, assign41160_e54131_d_n12, assign41160_e54131_d_n13, assign41160_e54131_d_n14, assign41160_e54131_d_n15, assign41160_e54131_d_n16, assign41160_e54131_d_n17, assign41160_e54131_d_n18, assign41160_e54131_d_n19, assign41160_e54131_d_n20,) = {
    if (locals.var_guard1275 != 0.0) {
        let assign41160_e54117: f64 = (locals.var_temp1 + locals.var_temp2);
        let assign41160_e54120: f64 = (locals.var_temp1 - locals.var_temp2);
        let assign41160_e54123: f64 = (locals.var_temp1 - locals.var_temp2);
        let assign41160_e54124: f64 = (assign41160_e54120 * assign41160_e54123);
        let assign41160_e54126: f64 = (assign41160_e54124 + 20.0);
        let assign41160_e54127: f64 = (assign41160_e54126).sqrt();
        let assign41160_e54128: f64 = (assign41160_e54117 - assign41160_e54127);
        let assign41160_e54129: f64 = (0.5 * assign41160_e54128);
        (assign41160_e54129, (0.5 * ((locals.var_temp1_dn5 + locals.var_temp2_dn5) - ((((locals.var_temp1_dn5 - locals.var_temp2_dn5) * assign41160_e54123) + (assign41160_e54120 * (locals.var_temp1_dn5 - locals.var_temp2_dn5))) / (2.0 * assign41160_e54127)))), (0.5 * ((locals.var_temp1_dn6 + locals.var_temp2_dn6) - ((((locals.var_temp1_dn6 - locals.var_temp2_dn6) * assign41160_e54123) + (assign41160_e54120 * (locals.var_temp1_dn6 - locals.var_temp2_dn6))) / (2.0 * assign41160_e54127)))), (0.5 * ((locals.var_temp1_dn7 + locals.var_temp2_dn7) - ((((locals.var_temp1_dn7 - locals.var_temp2_dn7) * assign41160_e54123) + (assign41160_e54120 * (locals.var_temp1_dn7 - locals.var_temp2_dn7))) / (2.0 * assign41160_e54127)))), (0.5 * ((locals.var_temp1_dn8 + locals.var_temp2_dn8) - ((((locals.var_temp1_dn8 - locals.var_temp2_dn8) * assign41160_e54123) + (assign41160_e54120 * (locals.var_temp1_dn8 - locals.var_temp2_dn8))) / (2.0 * assign41160_e54127)))), (0.5 * ((locals.var_temp1_dn12 + locals.var_temp2_dn12) - ((((locals.var_temp1_dn12 - locals.var_temp2_dn12) * assign41160_e54123) + (assign41160_e54120 * (locals.var_temp1_dn12 - locals.var_temp2_dn12))) / (2.0 * assign41160_e54127)))), (0.5 * ((locals.var_temp1_dn13 + locals.var_temp2_dn13) - ((((locals.var_temp1_dn13 - locals.var_temp2_dn13) * assign41160_e54123) + (assign41160_e54120 * (locals.var_temp1_dn13 - locals.var_temp2_dn13))) / (2.0 * assign41160_e54127)))), (0.5 * ((locals.var_temp1_dn14 + locals.var_temp2_dn14) - ((((locals.var_temp1_dn14 - locals.var_temp2_dn14) * assign41160_e54123) + (assign41160_e54120 * (locals.var_temp1_dn14 - locals.var_temp2_dn14))) / (2.0 * assign41160_e54127)))), (0.5 * ((locals.var_temp1_dn15 + locals.var_temp2_dn15) - ((((locals.var_temp1_dn15 - locals.var_temp2_dn15) * assign41160_e54123) + (assign41160_e54120 * (locals.var_temp1_dn15 - locals.var_temp2_dn15))) / (2.0 * assign41160_e54127)))), (0.5 * ((locals.var_temp1_dn16 + locals.var_temp2_dn16) - ((((locals.var_temp1_dn16 - locals.var_temp2_dn16) * assign41160_e54123) + (assign41160_e54120 * (locals.var_temp1_dn16 - locals.var_temp2_dn16))) / (2.0 * assign41160_e54127)))), (0.5 * ((locals.var_temp1_dn17 + locals.var_temp2_dn17) - ((((locals.var_temp1_dn17 - locals.var_temp2_dn17) * assign41160_e54123) + (assign41160_e54120 * (locals.var_temp1_dn17 - locals.var_temp2_dn17))) / (2.0 * assign41160_e54127)))), (0.5 * ((locals.var_temp1_dn18 + locals.var_temp2_dn18) - ((((locals.var_temp1_dn18 - locals.var_temp2_dn18) * assign41160_e54123) + (assign41160_e54120 * (locals.var_temp1_dn18 - locals.var_temp2_dn18))) / (2.0 * assign41160_e54127)))), (0.5 * ((locals.var_temp1_dn19 + locals.var_temp2_dn19) - ((((locals.var_temp1_dn19 - locals.var_temp2_dn19) * assign41160_e54123) + (assign41160_e54120 * (locals.var_temp1_dn19 - locals.var_temp2_dn19))) / (2.0 * assign41160_e54127)))), (0.5 * ((locals.var_temp1_dn20 + locals.var_temp2_dn20) - ((((locals.var_temp1_dn20 - locals.var_temp2_dn20) * assign41160_e54123) + (assign41160_e54120 * (locals.var_temp1_dn20 - locals.var_temp2_dn20))) / (2.0 * assign41160_e54127)))),)
    } else {
        (locals.var_xsubct, locals.var_xsubct_dn5, locals.var_xsubct_dn6, locals.var_xsubct_dn7, locals.var_xsubct_dn8, locals.var_xsubct_dn12, locals.var_xsubct_dn13, locals.var_xsubct_dn14, locals.var_xsubct_dn15, locals.var_xsubct_dn16, locals.var_xsubct_dn17, locals.var_xsubct_dn18, locals.var_xsubct_dn19, locals.var_xsubct_dn20,)
    }
};
        locals.var_xsubct = assign41160_e54131;
        locals.var_xsubct_dn5 = assign41160_e54131_d_n5;
        locals.var_xsubct_dn6 = assign41160_e54131_d_n6;
        locals.var_xsubct_dn7 = assign41160_e54131_d_n7;
        locals.var_xsubct_dn8 = assign41160_e54131_d_n8;
        locals.var_xsubct_dn12 = assign41160_e54131_d_n12;
        locals.var_xsubct_dn13 = assign41160_e54131_d_n13;
        locals.var_xsubct_dn14 = assign41160_e54131_d_n14;
        locals.var_xsubct_dn15 = assign41160_e54131_d_n15;
        locals.var_xsubct_dn16 = assign41160_e54131_d_n16;
        locals.var_xsubct_dn17 = assign41160_e54131_d_n17;
        locals.var_xsubct_dn18 = assign41160_e54131_d_n18;
        locals.var_xsubct_dn19 = assign41160_e54131_d_n19;
        locals.var_xsubct_dn20 = assign41160_e54131_d_n20;
        locals.var_xsubct_rv = 0.0;

        let (assign41170_e54150, assign41170_e54150_d_n5, assign41170_e54150_d_n6, assign41170_e54150_d_n7, assign41170_e54150_d_n8, assign41170_e54150_d_n12, assign41170_e54150_d_n13, assign41170_e54150_d_n14, assign41170_e54150_d_n15, assign41170_e54150_d_n16, assign41170_e54150_d_n17, assign41170_e54150_d_n18, assign41170_e54150_d_n19, assign41170_e54150_d_n20,) = {
    if (locals.var_guard1275 != 0.0) {
        let assign41170_e54136: f64 = (locals.var_xsubct + locals.var_xctmax);
        let assign41170_e54139: f64 = (locals.var_xsubct - locals.var_xctmax);
        let assign41170_e54142: f64 = (locals.var_xsubct - locals.var_xctmax);
        let assign41170_e54143: f64 = (assign41170_e54139 * assign41170_e54142);
        let assign41170_e54145: f64 = (assign41170_e54143 + 5.0);
        let assign41170_e54146: f64 = (assign41170_e54145).sqrt();
        let assign41170_e54147: f64 = (assign41170_e54136 - assign41170_e54146);
        let assign41170_e54148: f64 = (0.5 * assign41170_e54147);
        (assign41170_e54148, (0.5 * (locals.var_xsubct_dn5 - (((locals.var_xsubct_dn5 * assign41170_e54142) + (assign41170_e54139 * locals.var_xsubct_dn5)) / (2.0 * assign41170_e54146)))), (0.5 * (locals.var_xsubct_dn6 - (((locals.var_xsubct_dn6 * assign41170_e54142) + (assign41170_e54139 * locals.var_xsubct_dn6)) / (2.0 * assign41170_e54146)))), (0.5 * (locals.var_xsubct_dn7 - (((locals.var_xsubct_dn7 * assign41170_e54142) + (assign41170_e54139 * locals.var_xsubct_dn7)) / (2.0 * assign41170_e54146)))), (0.5 * (locals.var_xsubct_dn8 - (((locals.var_xsubct_dn8 * assign41170_e54142) + (assign41170_e54139 * locals.var_xsubct_dn8)) / (2.0 * assign41170_e54146)))), (0.5 * (locals.var_xsubct_dn12 - (((locals.var_xsubct_dn12 * assign41170_e54142) + (assign41170_e54139 * locals.var_xsubct_dn12)) / (2.0 * assign41170_e54146)))), (0.5 * (locals.var_xsubct_dn13 - (((locals.var_xsubct_dn13 * assign41170_e54142) + (assign41170_e54139 * locals.var_xsubct_dn13)) / (2.0 * assign41170_e54146)))), (0.5 * (locals.var_xsubct_dn14 - (((locals.var_xsubct_dn14 * assign41170_e54142) + (assign41170_e54139 * locals.var_xsubct_dn14)) / (2.0 * assign41170_e54146)))), (0.5 * (locals.var_xsubct_dn15 - (((locals.var_xsubct_dn15 * assign41170_e54142) + (assign41170_e54139 * locals.var_xsubct_dn15)) / (2.0 * assign41170_e54146)))), (0.5 * (locals.var_xsubct_dn16 - (((locals.var_xsubct_dn16 * assign41170_e54142) + (assign41170_e54139 * locals.var_xsubct_dn16)) / (2.0 * assign41170_e54146)))), (0.5 * (locals.var_xsubct_dn17 - (((locals.var_xsubct_dn17 * assign41170_e54142) + (assign41170_e54139 * locals.var_xsubct_dn17)) / (2.0 * assign41170_e54146)))), (0.5 * (locals.var_xsubct_dn18 - (((locals.var_xsubct_dn18 * assign41170_e54142) + (assign41170_e54139 * locals.var_xsubct_dn18)) / (2.0 * assign41170_e54146)))), (0.5 * (locals.var_xsubct_dn19 - (((locals.var_xsubct_dn19 * assign41170_e54142) + (assign41170_e54139 * locals.var_xsubct_dn19)) / (2.0 * assign41170_e54146)))), (0.5 * (locals.var_xsubct_dn20 - (((locals.var_xsubct_dn20 * assign41170_e54142) + (assign41170_e54139 * locals.var_xsubct_dn20)) / (2.0 * assign41170_e54146)))),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn5, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn12, locals.var_temp1_dn13, locals.var_temp1_dn14, locals.var_temp1_dn15, locals.var_temp1_dn16, locals.var_temp1_dn17, locals.var_temp1_dn18, locals.var_temp1_dn19, locals.var_temp1_dn20,)
    }
};
        locals.var_temp1 = assign41170_e54150;
        locals.var_temp1_dn5 = assign41170_e54150_d_n5;
        locals.var_temp1_dn6 = assign41170_e54150_d_n6;
        locals.var_temp1_dn7 = assign41170_e54150_d_n7;
        locals.var_temp1_dn8 = assign41170_e54150_d_n8;
        locals.var_temp1_dn12 = assign41170_e54150_d_n12;
        locals.var_temp1_dn13 = assign41170_e54150_d_n13;
        locals.var_temp1_dn14 = assign41170_e54150_d_n14;
        locals.var_temp1_dn15 = assign41170_e54150_d_n15;
        locals.var_temp1_dn16 = assign41170_e54150_d_n16;
        locals.var_temp1_dn17 = assign41170_e54150_d_n17;
        locals.var_temp1_dn18 = assign41170_e54150_d_n18;
        locals.var_temp1_dn19 = assign41170_e54150_d_n19;
        locals.var_temp1_dn20 = assign41170_e54150_d_n20;
        locals.var_temp1_rv = 0.0;

        let (assign41180_e54172, assign41180_e54172_d_n5, assign41180_e54172_d_n6, assign41180_e54172_d_n7, assign41180_e54172_d_n8, assign41180_e54172_d_n12, assign41180_e54172_d_n13, assign41180_e54172_d_n14, assign41180_e54172_d_n15, assign41180_e54172_d_n16, assign41180_e54172_d_n17, assign41180_e54172_d_n18, assign41180_e54172_d_n19, assign41180_e54172_d_n20,) = {
    if (locals.var_guard1275 != 0.0) {
        let assign41180_e54155: f64 = (-locals.var_xctmax);
        let assign41180_e54156: f64 = (locals.var_temp1 + assign41180_e54155);
        let assign41180_e54159: f64 = (-locals.var_xctmax);
        let assign41180_e54160: f64 = (locals.var_temp1 - assign41180_e54159);
        let assign41180_e54163: f64 = (-locals.var_xctmax);
        let assign41180_e54164: f64 = (locals.var_temp1 - assign41180_e54163);
        let assign41180_e54165: f64 = (assign41180_e54160 * assign41180_e54164);
        let assign41180_e54167: f64 = (assign41180_e54165 + 20.0);
        let assign41180_e54168: f64 = (assign41180_e54167).sqrt();
        let assign41180_e54169: f64 = (assign41180_e54156 + assign41180_e54168);
        let assign41180_e54170: f64 = (0.5 * assign41180_e54169);
        (assign41180_e54170, (0.5 * (locals.var_temp1_dn5 + (((locals.var_temp1_dn5 * assign41180_e54164) + (assign41180_e54160 * locals.var_temp1_dn5)) / (2.0 * assign41180_e54168)))), (0.5 * (locals.var_temp1_dn6 + (((locals.var_temp1_dn6 * assign41180_e54164) + (assign41180_e54160 * locals.var_temp1_dn6)) / (2.0 * assign41180_e54168)))), (0.5 * (locals.var_temp1_dn7 + (((locals.var_temp1_dn7 * assign41180_e54164) + (assign41180_e54160 * locals.var_temp1_dn7)) / (2.0 * assign41180_e54168)))), (0.5 * (locals.var_temp1_dn8 + (((locals.var_temp1_dn8 * assign41180_e54164) + (assign41180_e54160 * locals.var_temp1_dn8)) / (2.0 * assign41180_e54168)))), (0.5 * (locals.var_temp1_dn12 + (((locals.var_temp1_dn12 * assign41180_e54164) + (assign41180_e54160 * locals.var_temp1_dn12)) / (2.0 * assign41180_e54168)))), (0.5 * (locals.var_temp1_dn13 + (((locals.var_temp1_dn13 * assign41180_e54164) + (assign41180_e54160 * locals.var_temp1_dn13)) / (2.0 * assign41180_e54168)))), (0.5 * (locals.var_temp1_dn14 + (((locals.var_temp1_dn14 * assign41180_e54164) + (assign41180_e54160 * locals.var_temp1_dn14)) / (2.0 * assign41180_e54168)))), (0.5 * (locals.var_temp1_dn15 + (((locals.var_temp1_dn15 * assign41180_e54164) + (assign41180_e54160 * locals.var_temp1_dn15)) / (2.0 * assign41180_e54168)))), (0.5 * (locals.var_temp1_dn16 + (((locals.var_temp1_dn16 * assign41180_e54164) + (assign41180_e54160 * locals.var_temp1_dn16)) / (2.0 * assign41180_e54168)))), (0.5 * (locals.var_temp1_dn17 + (((locals.var_temp1_dn17 * assign41180_e54164) + (assign41180_e54160 * locals.var_temp1_dn17)) / (2.0 * assign41180_e54168)))), (0.5 * (locals.var_temp1_dn18 + (((locals.var_temp1_dn18 * assign41180_e54164) + (assign41180_e54160 * locals.var_temp1_dn18)) / (2.0 * assign41180_e54168)))), (0.5 * (locals.var_temp1_dn19 + (((locals.var_temp1_dn19 * assign41180_e54164) + (assign41180_e54160 * locals.var_temp1_dn19)) / (2.0 * assign41180_e54168)))), (0.5 * (locals.var_temp1_dn20 + (((locals.var_temp1_dn20 * assign41180_e54164) + (assign41180_e54160 * locals.var_temp1_dn20)) / (2.0 * assign41180_e54168)))),)
    } else {
        (locals.var_xct, locals.var_xct_dn5, locals.var_xct_dn6, locals.var_xct_dn7, locals.var_xct_dn8, locals.var_xct_dn12, locals.var_xct_dn13, locals.var_xct_dn14, locals.var_xct_dn15, locals.var_xct_dn16, locals.var_xct_dn17, locals.var_xct_dn18, locals.var_xct_dn19, locals.var_xct_dn20,)
    }
};
        locals.var_xct = assign41180_e54172;
        locals.var_xct_dn5 = assign41180_e54172_d_n5;
        locals.var_xct_dn6 = assign41180_e54172_d_n6;
        locals.var_xct_dn7 = assign41180_e54172_d_n7;
        locals.var_xct_dn8 = assign41180_e54172_d_n8;
        locals.var_xct_dn12 = assign41180_e54172_d_n12;
        locals.var_xct_dn13 = assign41180_e54172_d_n13;
        locals.var_xct_dn14 = assign41180_e54172_d_n14;
        locals.var_xct_dn15 = assign41180_e54172_d_n15;
        locals.var_xct_dn16 = assign41180_e54172_d_n16;
        locals.var_xct_dn17 = assign41180_e54172_d_n17;
        locals.var_xct_dn18 = assign41180_e54172_d_n18;
        locals.var_xct_dn19 = assign41180_e54172_d_n19;
        locals.var_xct_dn20 = assign41180_e54172_d_n20;
        locals.var_xct_rv = 0.0;

        let (assign41190_e54182, assign41190_e54182_d_n5, assign41190_e54182_d_n6, assign41190_e54182_d_n7, assign41190_e54182_d_n8, assign41190_e54182_d_n12, assign41190_e54182_d_n13, assign41190_e54182_d_n14, assign41190_e54182_d_n15, assign41190_e54182_d_n16, assign41190_e54182_d_n17, assign41190_e54182_d_n18, assign41190_e54182_d_n19, assign41190_e54182_d_n20,) = {
    if (locals.var_guard1275 != 0.0) {
        let assign41190_e54177: f64 = (locals.var_xct / locals.var_xctmax);
        let assign41190_e54179: f64 = (assign41190_e54177 + 1.0);
        let assign41190_e54180: f64 = (locals.var_ctg_t * assign41190_e54179);
        (assign41190_e54180, (locals.var_ctg_t * (locals.var_xct_dn5 / locals.var_xctmax)), (locals.var_ctg_t * (locals.var_xct_dn6 / locals.var_xctmax)), (locals.var_ctg_t * (locals.var_xct_dn7 / locals.var_xctmax)), (locals.var_ctg_t * (locals.var_xct_dn8 / locals.var_xctmax)), (locals.var_ctg_t * (locals.var_xct_dn12 / locals.var_xctmax)), (locals.var_ctg_t * (locals.var_xct_dn13 / locals.var_xctmax)), (locals.var_ctg_t * (locals.var_xct_dn14 / locals.var_xctmax)), (locals.var_ctg_t * (locals.var_xct_dn15 / locals.var_xctmax)), (locals.var_ctg_t * (locals.var_xct_dn16 / locals.var_xctmax)), (locals.var_ctg_t * (locals.var_xct_dn17 / locals.var_xctmax)), (locals.var_ctg_t * (locals.var_xct_dn18 / locals.var_xctmax)), (locals.var_ctg_t * (locals.var_xct_dn19 / locals.var_xctmax)), (locals.var_ctg_t * (locals.var_xct_dn20 / locals.var_xctmax)),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn5, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn12, locals.var_temp2_dn13, locals.var_temp2_dn14, locals.var_temp2_dn15, locals.var_temp2_dn16, locals.var_temp2_dn17, locals.var_temp2_dn18, locals.var_temp2_dn19, locals.var_temp2_dn20,)
    }
};
        locals.var_temp2 = assign41190_e54182;
        locals.var_temp2_dn5 = assign41190_e54182_d_n5;
        locals.var_temp2_dn6 = assign41190_e54182_d_n6;
        locals.var_temp2_dn7 = assign41190_e54182_d_n7;
        locals.var_temp2_dn8 = assign41190_e54182_d_n8;
        locals.var_temp2_dn12 = assign41190_e54182_d_n12;
        locals.var_temp2_dn13 = assign41190_e54182_d_n13;
        locals.var_temp2_dn14 = assign41190_e54182_d_n14;
        locals.var_temp2_dn15 = assign41190_e54182_d_n15;
        locals.var_temp2_dn16 = assign41190_e54182_d_n16;
        locals.var_temp2_dn17 = assign41190_e54182_d_n17;
        locals.var_temp2_dn18 = assign41190_e54182_d_n18;
        locals.var_temp2_dn19 = assign41190_e54182_d_n19;
        locals.var_temp2_dn20 = assign41190_e54182_d_n20;
        locals.var_temp2_rv = 0.0;

        let assign41200_e54185: f64 = (-230.25850929940458);
        let assign41200_e54186: f64 = if locals.var_temp2 > assign41200_e54185 { 1.0 } else { 0.0 };
        locals.var_guard1276 = assign41200_e54186;
        locals.var_guard1276_rv = 0.0;

        let (assign41210_e54193, assign41210_e54193_d_n5, assign41210_e54193_d_n6, assign41210_e54193_d_n7, assign41210_e54193_d_n8, assign41210_e54193_d_n12, assign41210_e54193_d_n13, assign41210_e54193_d_n14, assign41210_e54193_d_n15, assign41210_e54193_d_n16, assign41210_e54193_d_n17, assign41210_e54193_d_n18, assign41210_e54193_d_n19, assign41210_e54193_d_n20,) = {
    if ((locals.var_guard1275 != 0.0) && (locals.var_guard1276 != 0.0)) {
        let assign41210_e54191: f64 = (locals.var_temp2).exp();
        (assign41210_e54191, (assign41210_e54191 * locals.var_temp2_dn5), (assign41210_e54191 * locals.var_temp2_dn6), (assign41210_e54191 * locals.var_temp2_dn7), (assign41210_e54191 * locals.var_temp2_dn8), (assign41210_e54191 * locals.var_temp2_dn12), (assign41210_e54191 * locals.var_temp2_dn13), (assign41210_e54191 * locals.var_temp2_dn14), (assign41210_e54191 * locals.var_temp2_dn15), (assign41210_e54191 * locals.var_temp2_dn16), (assign41210_e54191 * locals.var_temp2_dn17), (assign41210_e54191 * locals.var_temp2_dn18), (assign41210_e54191 * locals.var_temp2_dn19), (assign41210_e54191 * locals.var_temp2_dn20),)
    } else {
        (locals.var_dctg, locals.var_dctg_dn5, locals.var_dctg_dn6, locals.var_dctg_dn7, locals.var_dctg_dn8, locals.var_dctg_dn12, locals.var_dctg_dn13, locals.var_dctg_dn14, locals.var_dctg_dn15, locals.var_dctg_dn16, locals.var_dctg_dn17, locals.var_dctg_dn18, locals.var_dctg_dn19, locals.var_dctg_dn20,)
    }
};
        locals.var_dctg = assign41210_e54193;
        locals.var_dctg_dn5 = assign41210_e54193_d_n5;
        locals.var_dctg_dn6 = assign41210_e54193_d_n6;
        locals.var_dctg_dn7 = assign41210_e54193_d_n7;
        locals.var_dctg_dn8 = assign41210_e54193_d_n8;
        locals.var_dctg_dn12 = assign41210_e54193_d_n12;
        locals.var_dctg_dn13 = assign41210_e54193_d_n13;
        locals.var_dctg_dn14 = assign41210_e54193_d_n14;
        locals.var_dctg_dn15 = assign41210_e54193_d_n15;
        locals.var_dctg_dn16 = assign41210_e54193_d_n16;
        locals.var_dctg_dn17 = assign41210_e54193_d_n17;
        locals.var_dctg_dn18 = assign41210_e54193_d_n18;
        locals.var_dctg_dn19 = assign41210_e54193_d_n19;
        locals.var_dctg_dn20 = assign41210_e54193_d_n20;
        locals.var_dctg_rv = 0.0;

        let (assign41220_e54225, assign41220_e54225_d_n5, assign41220_e54225_d_n6, assign41220_e54225_d_n7, assign41220_e54225_d_n8, assign41220_e54225_d_n12, assign41220_e54225_d_n13, assign41220_e54225_d_n14, assign41220_e54225_d_n15, assign41220_e54225_d_n16, assign41220_e54225_d_n17, assign41220_e54225_d_n18, assign41220_e54225_d_n19, assign41220_e54225_d_n20,) = {
    if ((locals.var_guard1275 != 0.0) && (locals.var_guard1276 == 0.0)) {
        let assign41220_e54201: f64 = (-230.25850929940458);
        let assign41220_e54203: f64 = (assign41220_e54201 - locals.var_temp2);
        let assign41220_e54207: f64 = (-230.25850929940458);
        let assign41220_e54209: f64 = (assign41220_e54207 - locals.var_temp2);
        let assign41220_e54212: f64 = (-230.25850929940458);
        let assign41220_e54214: f64 = (assign41220_e54212 - locals.var_temp2);
        let assign41220_e54216: f64 = (assign41220_e54214 * 0.3333333333333333);
        let assign41220_e54217: f64 = (1.0 + assign41220_e54216);
        let assign41220_e54218: f64 = (assign41220_e54209 * assign41220_e54217);
        let assign41220_e54219: f64 = (0.5 * assign41220_e54218);
        let assign41220_e54220: f64 = (1.0 + assign41220_e54219);
        let assign41220_e54221: f64 = (assign41220_e54203 * assign41220_e54220);
        let assign41220_e54222: f64 = (1.0 + assign41220_e54221);
        let assign41220_e54223: f64 = (1e-100 / assign41220_e54222);
        (assign41220_e54223, (-((1e-100 * (((-locals.var_temp2_dn5) * assign41220_e54220) + (assign41220_e54203 * (0.5 * (((-locals.var_temp2_dn5) * assign41220_e54217) + (assign41220_e54209 * ((-locals.var_temp2_dn5) * 0.3333333333333333))))))) / (assign41220_e54222 * assign41220_e54222))), (-((1e-100 * (((-locals.var_temp2_dn6) * assign41220_e54220) + (assign41220_e54203 * (0.5 * (((-locals.var_temp2_dn6) * assign41220_e54217) + (assign41220_e54209 * ((-locals.var_temp2_dn6) * 0.3333333333333333))))))) / (assign41220_e54222 * assign41220_e54222))), (-((1e-100 * (((-locals.var_temp2_dn7) * assign41220_e54220) + (assign41220_e54203 * (0.5 * (((-locals.var_temp2_dn7) * assign41220_e54217) + (assign41220_e54209 * ((-locals.var_temp2_dn7) * 0.3333333333333333))))))) / (assign41220_e54222 * assign41220_e54222))), (-((1e-100 * (((-locals.var_temp2_dn8) * assign41220_e54220) + (assign41220_e54203 * (0.5 * (((-locals.var_temp2_dn8) * assign41220_e54217) + (assign41220_e54209 * ((-locals.var_temp2_dn8) * 0.3333333333333333))))))) / (assign41220_e54222 * assign41220_e54222))), (-((1e-100 * (((-locals.var_temp2_dn12) * assign41220_e54220) + (assign41220_e54203 * (0.5 * (((-locals.var_temp2_dn12) * assign41220_e54217) + (assign41220_e54209 * ((-locals.var_temp2_dn12) * 0.3333333333333333))))))) / (assign41220_e54222 * assign41220_e54222))), (-((1e-100 * (((-locals.var_temp2_dn13) * assign41220_e54220) + (assign41220_e54203 * (0.5 * (((-locals.var_temp2_dn13) * assign41220_e54217) + (assign41220_e54209 * ((-locals.var_temp2_dn13) * 0.3333333333333333))))))) / (assign41220_e54222 * assign41220_e54222))), (-((1e-100 * (((-locals.var_temp2_dn14) * assign41220_e54220) + (assign41220_e54203 * (0.5 * (((-locals.var_temp2_dn14) * assign41220_e54217) + (assign41220_e54209 * ((-locals.var_temp2_dn14) * 0.3333333333333333))))))) / (assign41220_e54222 * assign41220_e54222))), (-((1e-100 * (((-locals.var_temp2_dn15) * assign41220_e54220) + (assign41220_e54203 * (0.5 * (((-locals.var_temp2_dn15) * assign41220_e54217) + (assign41220_e54209 * ((-locals.var_temp2_dn15) * 0.3333333333333333))))))) / (assign41220_e54222 * assign41220_e54222))), (-((1e-100 * (((-locals.var_temp2_dn16) * assign41220_e54220) + (assign41220_e54203 * (0.5 * (((-locals.var_temp2_dn16) * assign41220_e54217) + (assign41220_e54209 * ((-locals.var_temp2_dn16) * 0.3333333333333333))))))) / (assign41220_e54222 * assign41220_e54222))), (-((1e-100 * (((-locals.var_temp2_dn17) * assign41220_e54220) + (assign41220_e54203 * (0.5 * (((-locals.var_temp2_dn17) * assign41220_e54217) + (assign41220_e54209 * ((-locals.var_temp2_dn17) * 0.3333333333333333))))))) / (assign41220_e54222 * assign41220_e54222))), (-((1e-100 * (((-locals.var_temp2_dn18) * assign41220_e54220) + (assign41220_e54203 * (0.5 * (((-locals.var_temp2_dn18) * assign41220_e54217) + (assign41220_e54209 * ((-locals.var_temp2_dn18) * 0.3333333333333333))))))) / (assign41220_e54222 * assign41220_e54222))), (-((1e-100 * (((-locals.var_temp2_dn19) * assign41220_e54220) + (assign41220_e54203 * (0.5 * (((-locals.var_temp2_dn19) * assign41220_e54217) + (assign41220_e54209 * ((-locals.var_temp2_dn19) * 0.3333333333333333))))))) / (assign41220_e54222 * assign41220_e54222))), (-((1e-100 * (((-locals.var_temp2_dn20) * assign41220_e54220) + (assign41220_e54203 * (0.5 * (((-locals.var_temp2_dn20) * assign41220_e54217) + (assign41220_e54209 * ((-locals.var_temp2_dn20) * 0.3333333333333333))))))) / (assign41220_e54222 * assign41220_e54222))),)
    } else {
        (locals.var_dctg, locals.var_dctg_dn5, locals.var_dctg_dn6, locals.var_dctg_dn7, locals.var_dctg_dn8, locals.var_dctg_dn12, locals.var_dctg_dn13, locals.var_dctg_dn14, locals.var_dctg_dn15, locals.var_dctg_dn16, locals.var_dctg_dn17, locals.var_dctg_dn18, locals.var_dctg_dn19, locals.var_dctg_dn20,)
    }
};
        locals.var_dctg = assign41220_e54225;
        locals.var_dctg_dn5 = assign41220_e54225_d_n5;
        locals.var_dctg_dn6 = assign41220_e54225_d_n6;
        locals.var_dctg_dn7 = assign41220_e54225_d_n7;
        locals.var_dctg_dn8 = assign41220_e54225_d_n8;
        locals.var_dctg_dn12 = assign41220_e54225_d_n12;
        locals.var_dctg_dn13 = assign41220_e54225_d_n13;
        locals.var_dctg_dn14 = assign41220_e54225_d_n14;
        locals.var_dctg_dn15 = assign41220_e54225_d_n15;
        locals.var_dctg_dn16 = assign41220_e54225_d_n16;
        locals.var_dctg_dn17 = assign41220_e54225_d_n17;
        locals.var_dctg_dn18 = assign41220_e54225_d_n18;
        locals.var_dctg_dn19 = assign41220_e54225_d_n19;
        locals.var_dctg_dn20 = assign41220_e54225_d_n20;
        locals.var_dctg_rv = 0.0;

        let assign41230_e54229: f64 = (locals.var_ct_t * locals.var_dctg);
        let assign41230_e54230: f64 = (1.0 + assign41230_e54229);
        locals.var_ct_fact = assign41230_e54230;
        locals.var_ct_fact_dn5 = (locals.var_ct_t * locals.var_dctg_dn5);
        locals.var_ct_fact_dn6 = (locals.var_ct_t * locals.var_dctg_dn6);
        locals.var_ct_fact_dn7 = (locals.var_ct_t * locals.var_dctg_dn7);
        locals.var_ct_fact_dn8 = (locals.var_ct_t * locals.var_dctg_dn8);
        locals.var_ct_fact_dn12 = (locals.var_ct_t * locals.var_dctg_dn12);
        locals.var_ct_fact_dn13 = (locals.var_ct_t * locals.var_dctg_dn13);
        locals.var_ct_fact_dn14 = (locals.var_ct_t * locals.var_dctg_dn14);
        locals.var_ct_fact_dn15 = (locals.var_ct_t * locals.var_dctg_dn15);
        locals.var_ct_fact_dn16 = (locals.var_ct_t * locals.var_dctg_dn16);
        locals.var_ct_fact_dn17 = (locals.var_ct_t * locals.var_dctg_dn17);
        locals.var_ct_fact_dn18 = (locals.var_ct_t * locals.var_dctg_dn18);
        locals.var_ct_fact_dn19 = (locals.var_ct_t * locals.var_dctg_dn19);
        locals.var_ct_fact_dn20 = (locals.var_ct_t * locals.var_dctg_dn20);
        locals.var_ct_fact_rv = 0.0;

        let assign41240_e54233: f64 = (locals.var_phit * locals.var_ct_fact);
        locals.var_phitct = assign41240_e54233;
        locals.var_phitct_dn5 = (locals.var_phit * locals.var_ct_fact_dn5);
        locals.var_phitct_dn6 = (locals.var_phit * locals.var_ct_fact_dn6);
        locals.var_phitct_dn7 = (locals.var_phit * locals.var_ct_fact_dn7);
        locals.var_phitct_dn8 = (locals.var_phit * locals.var_ct_fact_dn8);
        locals.var_phitct_dn12 = (locals.var_phit * locals.var_ct_fact_dn12);
        locals.var_phitct_dn13 = (locals.var_phit * locals.var_ct_fact_dn13);
        locals.var_phitct_dn14 = (locals.var_phit * locals.var_ct_fact_dn14);
        locals.var_phitct_dn15 = (locals.var_phit * locals.var_ct_fact_dn15);
        locals.var_phitct_dn16 = (locals.var_phit * locals.var_ct_fact_dn16);
        locals.var_phitct_dn17 = (locals.var_phit * locals.var_ct_fact_dn17);
        locals.var_phitct_dn18 = (locals.var_phit * locals.var_ct_fact_dn18);
        locals.var_phitct_dn19 = (locals.var_phit * locals.var_ct_fact_dn19);
        locals.var_phitct_dn20 = (locals.var_phit * locals.var_ct_fact_dn20);
        locals.var_phitct_rv = 0.0;

        let assign41250_e54238: f64 = (locals.var_psced_i * locals.var_vdsx);
        let assign41250_e54239: f64 = (1.0 + assign41250_e54238);
        let assign41250_e54240: f64 = (locals.var_psce_i * assign41250_e54239);
        let assign41250_e54244: f64 = (locals.var_psceb_i * locals.var_vsbx);
        let assign41250_e54245: f64 = (1.0 + assign41250_e54244);
        let assign41250_e54246: f64 = (assign41250_e54240 * assign41250_e54245);
        locals.var_dphit1 = assign41250_e54246;
        locals.var_dphit1_dn5 = (assign41250_e54240 * (locals.var_psceb_i * locals.var_vsbx_dn5));
        locals.var_dphit1_dn6 = (((locals.var_psce_i * (locals.var_psced_i * locals.var_vdsx_dn6)) * assign41250_e54245) + (assign41250_e54240 * (locals.var_psceb_i * locals.var_vsbx_dn6)));
        locals.var_dphit1_dn7 = (((locals.var_psce_i * (locals.var_psced_i * locals.var_vdsx_dn7)) * assign41250_e54245) + (assign41250_e54240 * (locals.var_psceb_i * locals.var_vsbx_dn7)));
        locals.var_dphit1_dn8 = (assign41250_e54240 * (locals.var_psceb_i * locals.var_vsbx_dn8));
        locals.var_dphit1_dn12 = (assign41250_e54240 * (locals.var_psceb_i * locals.var_vsbx_dn12));
        locals.var_dphit1_dn13 = (assign41250_e54240 * (locals.var_psceb_i * locals.var_vsbx_dn13));
        locals.var_dphit1_dn14 = (assign41250_e54240 * (locals.var_psceb_i * locals.var_vsbx_dn14));
        locals.var_dphit1_dn15 = (assign41250_e54240 * (locals.var_psceb_i * locals.var_vsbx_dn15));
        locals.var_dphit1_dn16 = (assign41250_e54240 * (locals.var_psceb_i * locals.var_vsbx_dn16));
        locals.var_dphit1_dn17 = (assign41250_e54240 * (locals.var_psceb_i * locals.var_vsbx_dn17));
        locals.var_dphit1_dn18 = (assign41250_e54240 * (locals.var_psceb_i * locals.var_vsbx_dn18));
        locals.var_dphit1_dn19 = (assign41250_e54240 * (locals.var_psceb_i * locals.var_vsbx_dn19));
        locals.var_dphit1_dn20 = (assign41250_e54240 * (locals.var_psceb_i * locals.var_vsbx_dn20));
        locals.var_dphit1_rv = 0.0;

        let assign41260_e54250: f64 = (1.0 + locals.var_dphit1);
        let assign41260_e54251: f64 = (locals.var_phitct * assign41260_e54250);
        locals.var_phit1 = assign41260_e54251;
        locals.var_phit1_dn5 = ((locals.var_phitct_dn5 * assign41260_e54250) + (locals.var_phitct * locals.var_dphit1_dn5));
        locals.var_phit1_dn6 = ((locals.var_phitct_dn6 * assign41260_e54250) + (locals.var_phitct * locals.var_dphit1_dn6));
        locals.var_phit1_dn7 = ((locals.var_phitct_dn7 * assign41260_e54250) + (locals.var_phitct * locals.var_dphit1_dn7));
        locals.var_phit1_dn8 = ((locals.var_phitct_dn8 * assign41260_e54250) + (locals.var_phitct * locals.var_dphit1_dn8));
        locals.var_phit1_dn12 = ((locals.var_phitct_dn12 * assign41260_e54250) + (locals.var_phitct * locals.var_dphit1_dn12));
        locals.var_phit1_dn13 = ((locals.var_phitct_dn13 * assign41260_e54250) + (locals.var_phitct * locals.var_dphit1_dn13));
        locals.var_phit1_dn14 = ((locals.var_phitct_dn14 * assign41260_e54250) + (locals.var_phitct * locals.var_dphit1_dn14));
        locals.var_phit1_dn15 = ((locals.var_phitct_dn15 * assign41260_e54250) + (locals.var_phitct * locals.var_dphit1_dn15));
        locals.var_phit1_dn16 = ((locals.var_phitct_dn16 * assign41260_e54250) + (locals.var_phitct * locals.var_dphit1_dn16));
        locals.var_phit1_dn17 = ((locals.var_phitct_dn17 * assign41260_e54250) + (locals.var_phitct * locals.var_dphit1_dn17));
        locals.var_phit1_dn18 = ((locals.var_phitct_dn18 * assign41260_e54250) + (locals.var_phitct * locals.var_dphit1_dn18));
        locals.var_phit1_dn19 = ((locals.var_phitct_dn19 * assign41260_e54250) + (locals.var_phitct * locals.var_dphit1_dn19));
        locals.var_phit1_dn20 = ((locals.var_phitct_dn20 * assign41260_e54250) + (locals.var_phitct * locals.var_dphit1_dn20));
        locals.var_phit1_rv = 0.0;

        let assign41270_e54254: f64 = (1.0 / locals.var_phit1);
        locals.var_inv_phit1 = assign41270_e54254;
        locals.var_inv_phit1_dn5 = (-(locals.var_phit1_dn5 / (locals.var_phit1 * locals.var_phit1)));
        locals.var_inv_phit1_dn6 = (-(locals.var_phit1_dn6 / (locals.var_phit1 * locals.var_phit1)));
        locals.var_inv_phit1_dn7 = (-(locals.var_phit1_dn7 / (locals.var_phit1 * locals.var_phit1)));
        locals.var_inv_phit1_dn8 = (-(locals.var_phit1_dn8 / (locals.var_phit1 * locals.var_phit1)));
        locals.var_inv_phit1_dn12 = (-(locals.var_phit1_dn12 / (locals.var_phit1 * locals.var_phit1)));
        locals.var_inv_phit1_dn13 = (-(locals.var_phit1_dn13 / (locals.var_phit1 * locals.var_phit1)));
        locals.var_inv_phit1_dn14 = (-(locals.var_phit1_dn14 / (locals.var_phit1 * locals.var_phit1)));
        locals.var_inv_phit1_dn15 = (-(locals.var_phit1_dn15 / (locals.var_phit1 * locals.var_phit1)));
        locals.var_inv_phit1_dn16 = (-(locals.var_phit1_dn16 / (locals.var_phit1 * locals.var_phit1)));
        locals.var_inv_phit1_dn17 = (-(locals.var_phit1_dn17 / (locals.var_phit1 * locals.var_phit1)));
        locals.var_inv_phit1_dn18 = (-(locals.var_phit1_dn18 / (locals.var_phit1 * locals.var_phit1)));
        locals.var_inv_phit1_dn19 = (-(locals.var_phit1_dn19 / (locals.var_phit1 * locals.var_phit1)));
        locals.var_inv_phit1_dn20 = (-(locals.var_phit1_dn20 / (locals.var_phit1 * locals.var_phit1)));
        locals.var_inv_phit1_rv = 0.0;

        let assign41280_e54258: f64 = (locals.var_phit * locals.var_inv_phit1);
        let assign41280_e54259: f64 = (assign41280_e54258).sqrt();
        let assign41280_e54260: f64 = (locals.var_g_0 * assign41280_e54259);
        locals.var_gf = assign41280_e54260;
        locals.var_gf_dn5 = (locals.var_g_0 * ((locals.var_phit * locals.var_inv_phit1_dn5) / (2.0 * assign41280_e54259)));
        locals.var_gf_dn6 = (locals.var_g_0 * ((locals.var_phit * locals.var_inv_phit1_dn6) / (2.0 * assign41280_e54259)));
        locals.var_gf_dn7 = (locals.var_g_0 * ((locals.var_phit * locals.var_inv_phit1_dn7) / (2.0 * assign41280_e54259)));
        locals.var_gf_dn8 = (locals.var_g_0 * ((locals.var_phit * locals.var_inv_phit1_dn8) / (2.0 * assign41280_e54259)));
        locals.var_gf_dn12 = (locals.var_g_0 * ((locals.var_phit * locals.var_inv_phit1_dn12) / (2.0 * assign41280_e54259)));
        locals.var_gf_dn13 = (locals.var_g_0 * ((locals.var_phit * locals.var_inv_phit1_dn13) / (2.0 * assign41280_e54259)));
        locals.var_gf_dn14 = (locals.var_g_0 * ((locals.var_phit * locals.var_inv_phit1_dn14) / (2.0 * assign41280_e54259)));
        locals.var_gf_dn15 = (locals.var_g_0 * ((locals.var_phit * locals.var_inv_phit1_dn15) / (2.0 * assign41280_e54259)));
        locals.var_gf_dn16 = (locals.var_g_0 * ((locals.var_phit * locals.var_inv_phit1_dn16) / (2.0 * assign41280_e54259)));
        locals.var_gf_dn17 = (locals.var_g_0 * ((locals.var_phit * locals.var_inv_phit1_dn17) / (2.0 * assign41280_e54259)));
        locals.var_gf_dn18 = (locals.var_g_0 * ((locals.var_phit * locals.var_inv_phit1_dn18) / (2.0 * assign41280_e54259)));
        locals.var_gf_dn19 = (locals.var_g_0 * ((locals.var_phit * locals.var_inv_phit1_dn19) / (2.0 * assign41280_e54259)));
        locals.var_gf_dn20 = (locals.var_g_0 * ((locals.var_phit * locals.var_inv_phit1_dn20) / (2.0 * assign41280_e54259)));
        locals.var_gf_rv = 0.0;

        let assign41290_e54263: f64 = (locals.var_gf * locals.var_gf);
        locals.var_gf2 = assign41290_e54263;
        locals.var_gf2_dn5 = ((locals.var_gf_dn5 * locals.var_gf) + (locals.var_gf * locals.var_gf_dn5));
        locals.var_gf2_dn6 = ((locals.var_gf_dn6 * locals.var_gf) + (locals.var_gf * locals.var_gf_dn6));
        locals.var_gf2_dn7 = ((locals.var_gf_dn7 * locals.var_gf) + (locals.var_gf * locals.var_gf_dn7));
        locals.var_gf2_dn8 = ((locals.var_gf_dn8 * locals.var_gf) + (locals.var_gf * locals.var_gf_dn8));
        locals.var_gf2_dn12 = ((locals.var_gf_dn12 * locals.var_gf) + (locals.var_gf * locals.var_gf_dn12));
        locals.var_gf2_dn13 = ((locals.var_gf_dn13 * locals.var_gf) + (locals.var_gf * locals.var_gf_dn13));
        locals.var_gf2_dn14 = ((locals.var_gf_dn14 * locals.var_gf) + (locals.var_gf * locals.var_gf_dn14));
        locals.var_gf2_dn15 = ((locals.var_gf_dn15 * locals.var_gf) + (locals.var_gf * locals.var_gf_dn15));
        locals.var_gf2_dn16 = ((locals.var_gf_dn16 * locals.var_gf) + (locals.var_gf * locals.var_gf_dn16));
        locals.var_gf2_dn17 = ((locals.var_gf_dn17 * locals.var_gf) + (locals.var_gf * locals.var_gf_dn17));
        locals.var_gf2_dn18 = ((locals.var_gf_dn18 * locals.var_gf) + (locals.var_gf * locals.var_gf_dn18));
        locals.var_gf2_dn19 = ((locals.var_gf_dn19 * locals.var_gf) + (locals.var_gf * locals.var_gf_dn19));
        locals.var_gf2_dn20 = ((locals.var_gf_dn20 * locals.var_gf) + (locals.var_gf * locals.var_gf_dn20));
        locals.var_gf2_rv = 0.0;

        let assign41300_e54266: f64 = (1.0 / locals.var_gf2);
        locals.var_inv_gf2 = assign41300_e54266;
        locals.var_inv_gf2_dn5 = (-(locals.var_gf2_dn5 / (locals.var_gf2 * locals.var_gf2)));
        locals.var_inv_gf2_dn6 = (-(locals.var_gf2_dn6 / (locals.var_gf2 * locals.var_gf2)));
        locals.var_inv_gf2_dn7 = (-(locals.var_gf2_dn7 / (locals.var_gf2 * locals.var_gf2)));
        locals.var_inv_gf2_dn8 = (-(locals.var_gf2_dn8 / (locals.var_gf2 * locals.var_gf2)));
        locals.var_inv_gf2_dn12 = (-(locals.var_gf2_dn12 / (locals.var_gf2 * locals.var_gf2)));
        locals.var_inv_gf2_dn13 = (-(locals.var_gf2_dn13 / (locals.var_gf2 * locals.var_gf2)));
        locals.var_inv_gf2_dn14 = (-(locals.var_gf2_dn14 / (locals.var_gf2 * locals.var_gf2)));
        locals.var_inv_gf2_dn15 = (-(locals.var_gf2_dn15 / (locals.var_gf2 * locals.var_gf2)));
        locals.var_inv_gf2_dn16 = (-(locals.var_gf2_dn16 / (locals.var_gf2 * locals.var_gf2)));
        locals.var_inv_gf2_dn17 = (-(locals.var_gf2_dn17 / (locals.var_gf2 * locals.var_gf2)));
        locals.var_inv_gf2_dn18 = (-(locals.var_gf2_dn18 / (locals.var_gf2 * locals.var_gf2)));
        locals.var_inv_gf2_dn19 = (-(locals.var_gf2_dn19 / (locals.var_gf2 * locals.var_gf2)));
        locals.var_inv_gf2_dn20 = (-(locals.var_gf2_dn20 / (locals.var_gf2 * locals.var_gf2)));
        locals.var_inv_gf2_rv = 0.0;

        let assign41310_e54269: f64 = (locals.var_vsbstar * locals.var_inv_phit1);
        locals.var_ux = assign41310_e54269;
        locals.var_ux_dn5 = ((locals.var_vsbstar_dn5 * locals.var_inv_phit1) + (locals.var_vsbstar * locals.var_inv_phit1_dn5));
        locals.var_ux_dn6 = ((locals.var_vsbstar_dn6 * locals.var_inv_phit1) + (locals.var_vsbstar * locals.var_inv_phit1_dn6));
        locals.var_ux_dn7 = ((locals.var_vsbstar_dn7 * locals.var_inv_phit1) + (locals.var_vsbstar * locals.var_inv_phit1_dn7));
        locals.var_ux_dn8 = ((locals.var_vsbstar_dn8 * locals.var_inv_phit1) + (locals.var_vsbstar * locals.var_inv_phit1_dn8));
        locals.var_ux_dn12 = ((locals.var_vsbstar_dn12 * locals.var_inv_phit1) + (locals.var_vsbstar * locals.var_inv_phit1_dn12));
        locals.var_ux_dn13 = ((locals.var_vsbstar_dn13 * locals.var_inv_phit1) + (locals.var_vsbstar * locals.var_inv_phit1_dn13));
        locals.var_ux_dn14 = ((locals.var_vsbstar_dn14 * locals.var_inv_phit1) + (locals.var_vsbstar * locals.var_inv_phit1_dn14));
        locals.var_ux_dn15 = ((locals.var_vsbstar_dn15 * locals.var_inv_phit1) + (locals.var_vsbstar * locals.var_inv_phit1_dn15));
        locals.var_ux_dn16 = ((locals.var_vsbstar_dn16 * locals.var_inv_phit1) + (locals.var_vsbstar * locals.var_inv_phit1_dn16));
        locals.var_ux_dn17 = ((locals.var_vsbstar_dn17 * locals.var_inv_phit1) + (locals.var_vsbstar * locals.var_inv_phit1_dn17));
        locals.var_ux_dn18 = ((locals.var_vsbstar_dn18 * locals.var_inv_phit1) + (locals.var_vsbstar * locals.var_inv_phit1_dn18));
        locals.var_ux_dn19 = ((locals.var_vsbstar_dn19 * locals.var_inv_phit1) + (locals.var_vsbstar * locals.var_inv_phit1_dn19));
        locals.var_ux_dn20 = ((locals.var_vsbstar_dn20 * locals.var_inv_phit1) + (locals.var_vsbstar * locals.var_inv_phit1_dn20));
        locals.var_ux_rv = 0.0;

        let assign41320_e54272: f64 = (locals.var_vgb1 * locals.var_inv_phit1);
        locals.var_xg = assign41320_e54272;
        locals.var_xg_dn5 = ((locals.var_vgb1_dn5 * locals.var_inv_phit1) + (locals.var_vgb1 * locals.var_inv_phit1_dn5));
        locals.var_xg_dn6 = ((locals.var_vgb1_dn6 * locals.var_inv_phit1) + (locals.var_vgb1 * locals.var_inv_phit1_dn6));
        locals.var_xg_dn7 = ((locals.var_vgb1_dn7 * locals.var_inv_phit1) + (locals.var_vgb1 * locals.var_inv_phit1_dn7));
        locals.var_xg_dn8 = ((locals.var_vgb1_dn8 * locals.var_inv_phit1) + (locals.var_vgb1 * locals.var_inv_phit1_dn8));
        locals.var_xg_dn12 = ((locals.var_vgb1_dn12 * locals.var_inv_phit1) + (locals.var_vgb1 * locals.var_inv_phit1_dn12));
        locals.var_xg_dn13 = ((locals.var_vgb1_dn13 * locals.var_inv_phit1) + (locals.var_vgb1 * locals.var_inv_phit1_dn13));
        locals.var_xg_dn14 = ((locals.var_vgb1_dn14 * locals.var_inv_phit1) + (locals.var_vgb1 * locals.var_inv_phit1_dn14));
        locals.var_xg_dn15 = ((locals.var_vgb1_dn15 * locals.var_inv_phit1) + (locals.var_vgb1 * locals.var_inv_phit1_dn15));
        locals.var_xg_dn16 = ((locals.var_vgb1_dn16 * locals.var_inv_phit1) + (locals.var_vgb1 * locals.var_inv_phit1_dn16));
        locals.var_xg_dn17 = ((locals.var_vgb1_dn17 * locals.var_inv_phit1) + (locals.var_vgb1 * locals.var_inv_phit1_dn17));
        locals.var_xg_dn18 = ((locals.var_vgb1_dn18 * locals.var_inv_phit1) + (locals.var_vgb1 * locals.var_inv_phit1_dn18));
        locals.var_xg_dn19 = ((locals.var_vgb1_dn19 * locals.var_inv_phit1) + (locals.var_vgb1 * locals.var_inv_phit1_dn19));
        locals.var_xg_dn20 = ((locals.var_vgb1_dn20 * locals.var_inv_phit1) + (locals.var_vgb1 * locals.var_inv_phit1_dn20));
        locals.var_xg_rv = 0.0;

        let assign41330_e54275: f64 = (2.0 * locals.var_vdsx);
        let assign41330_e54280: f64 = (locals.var_cfd_i * locals.var_vdsx);
        let assign41330_e54281: f64 = (1.0 + assign41330_e54280);
        let assign41330_e54282: f64 = (assign41330_e54281).sqrt();
        let assign41330_e54283: f64 = (1.0 + assign41330_e54282);
        let assign41330_e54284: f64 = (assign41330_e54275 / assign41330_e54283);
        locals.var_vdsp = assign41330_e54284;
        locals.var_vdsp_dn6 = ((((2.0 * locals.var_vdsx_dn6) * assign41330_e54283) - (assign41330_e54275 * ((locals.var_cfd_i * locals.var_vdsx_dn6) / (2.0 * assign41330_e54282)))) / (assign41330_e54283 * assign41330_e54283));
        locals.var_vdsp_dn7 = ((((2.0 * locals.var_vdsx_dn7) * assign41330_e54283) - (assign41330_e54275 * ((locals.var_cfd_i * locals.var_vdsx_dn7) / (2.0 * assign41330_e54282)))) / (assign41330_e54283 * assign41330_e54283));
        locals.var_vdsp_rv = 0.0;

        let assign41340_e54287: f64 = (locals.var_cf_i * locals.var_vdsp);
        let assign41340_e54291: f64 = (locals.var_cfb_i * locals.var_vsbx);
        let assign41340_e54292: f64 = (1.0 + assign41340_e54291);
        let assign41340_e54293: f64 = (assign41340_e54287 * assign41340_e54292);
        locals.var_delphib = assign41340_e54293;
        locals.var_delphib_dn5 = (assign41340_e54287 * (locals.var_cfb_i * locals.var_vsbx_dn5));
        locals.var_delphib_dn6 = (((locals.var_cf_i * locals.var_vdsp_dn6) * assign41340_e54292) + (assign41340_e54287 * (locals.var_cfb_i * locals.var_vsbx_dn6)));
        locals.var_delphib_dn7 = (((locals.var_cf_i * locals.var_vdsp_dn7) * assign41340_e54292) + (assign41340_e54287 * (locals.var_cfb_i * locals.var_vsbx_dn7)));
        locals.var_delphib_dn8 = (assign41340_e54287 * (locals.var_cfb_i * locals.var_vsbx_dn8));
        locals.var_delphib_dn12 = (assign41340_e54287 * (locals.var_cfb_i * locals.var_vsbx_dn12));
        locals.var_delphib_dn13 = (assign41340_e54287 * (locals.var_cfb_i * locals.var_vsbx_dn13));
        locals.var_delphib_dn14 = (assign41340_e54287 * (locals.var_cfb_i * locals.var_vsbx_dn14));
        locals.var_delphib_dn15 = (assign41340_e54287 * (locals.var_cfb_i * locals.var_vsbx_dn15));
        locals.var_delphib_dn16 = (assign41340_e54287 * (locals.var_cfb_i * locals.var_vsbx_dn16));
        locals.var_delphib_dn17 = (assign41340_e54287 * (locals.var_cfb_i * locals.var_vsbx_dn17));
        locals.var_delphib_dn18 = (assign41340_e54287 * (locals.var_cfb_i * locals.var_vsbx_dn18));
        locals.var_delphib_dn19 = (assign41340_e54287 * (locals.var_cfb_i * locals.var_vsbx_dn19));
        locals.var_delphib_dn20 = (assign41340_e54287 * (locals.var_cfb_i * locals.var_vsbx_dn20));
        locals.var_delphib_rv = 0.0;

        let assign41350_e54296: f64 = (locals.var_phib * locals.var_inv_phit1);
        locals.var_xb = assign41350_e54296;
        locals.var_xb_dn5 = (locals.var_phib * locals.var_inv_phit1_dn5);
        locals.var_xb_dn6 = (locals.var_phib * locals.var_inv_phit1_dn6);
        locals.var_xb_dn7 = (locals.var_phib * locals.var_inv_phit1_dn7);
        locals.var_xb_dn8 = (locals.var_phib * locals.var_inv_phit1_dn8);
        locals.var_xb_dn12 = (locals.var_phib * locals.var_inv_phit1_dn12);
        locals.var_xb_dn13 = (locals.var_phib * locals.var_inv_phit1_dn13);
        locals.var_xb_dn14 = (locals.var_phib * locals.var_inv_phit1_dn14);
        locals.var_xb_dn15 = (locals.var_phib * locals.var_inv_phit1_dn15);
        locals.var_xb_dn16 = (locals.var_phib * locals.var_inv_phit1_dn16);
        locals.var_xb_dn17 = (locals.var_phib * locals.var_inv_phit1_dn17);
        locals.var_xb_dn18 = (locals.var_phib * locals.var_inv_phit1_dn18);
        locals.var_xb_dn19 = (locals.var_phib * locals.var_inv_phit1_dn19);
        locals.var_xb_dn20 = (locals.var_phib * locals.var_inv_phit1_dn20);
        locals.var_xb_rv = 0.0;

        let assign41360_e54299: f64 = (locals.var_v_xb * locals.var_v_xb);
        let assign41360_e54301: f64 = (assign41360_e54299 + locals.var_aphi);
        let assign41360_e54302: f64 = (assign41360_e54301).sqrt();
        locals.var_temp1 = assign41360_e54302;
        locals.var_temp1_dn5 = 0.0;
        locals.var_temp1_dn6 = (((locals.var_v_xb_dn6 * locals.var_v_xb) + (locals.var_v_xb * locals.var_v_xb_dn6)) / (2.0 * assign41360_e54302));
        locals.var_temp1_dn7 = (((locals.var_v_xb_dn7 * locals.var_v_xb) + (locals.var_v_xb * locals.var_v_xb_dn7)) / (2.0 * assign41360_e54302));
        locals.var_temp1_dn8 = (((locals.var_v_xb_dn8 * locals.var_v_xb) + (locals.var_v_xb * locals.var_v_xb_dn8)) / (2.0 * assign41360_e54302));
        locals.var_temp1_dn12 = 0.0;
        locals.var_temp1_dn13 = 0.0;
        locals.var_temp1_dn14 = 0.0;
        locals.var_temp1_dn15 = 0.0;
        locals.var_temp1_dn16 = 0.0;
        locals.var_temp1_dn17 = 0.0;
        locals.var_temp1_dn18 = 0.0;
        locals.var_temp1_dn19 = 0.0;
        locals.var_temp1_dn20 = 0.0;
        locals.var_temp1_rv = 0.0;

        let assign41370_e54305: f64 = (locals.var_v_xb - locals.var_delphib);
        let assign41370_e54308: f64 = (locals.var_v_xb - locals.var_delphib);
        let assign41370_e54309: f64 = (assign41370_e54305 * assign41370_e54308);
        let assign41370_e54311: f64 = (assign41370_e54309 + locals.var_aphi);
        let assign41370_e54312: f64 = (assign41370_e54311).sqrt();
        locals.var_temp2 = assign41370_e54312;
        locals.var_temp2_dn5 = ((((-locals.var_delphib_dn5) * assign41370_e54308) + (assign41370_e54305 * (-locals.var_delphib_dn5))) / (2.0 * assign41370_e54312));
        locals.var_temp2_dn6 = ((((locals.var_v_xb_dn6 - locals.var_delphib_dn6) * assign41370_e54308) + (assign41370_e54305 * (locals.var_v_xb_dn6 - locals.var_delphib_dn6))) / (2.0 * assign41370_e54312));
        locals.var_temp2_dn7 = ((((locals.var_v_xb_dn7 - locals.var_delphib_dn7) * assign41370_e54308) + (assign41370_e54305 * (locals.var_v_xb_dn7 - locals.var_delphib_dn7))) / (2.0 * assign41370_e54312));
        locals.var_temp2_dn8 = ((((locals.var_v_xb_dn8 - locals.var_delphib_dn8) * assign41370_e54308) + (assign41370_e54305 * (locals.var_v_xb_dn8 - locals.var_delphib_dn8))) / (2.0 * assign41370_e54312));
        locals.var_temp2_dn12 = ((((-locals.var_delphib_dn12) * assign41370_e54308) + (assign41370_e54305 * (-locals.var_delphib_dn12))) / (2.0 * assign41370_e54312));
        locals.var_temp2_dn13 = ((((-locals.var_delphib_dn13) * assign41370_e54308) + (assign41370_e54305 * (-locals.var_delphib_dn13))) / (2.0 * assign41370_e54312));
        locals.var_temp2_dn14 = ((((-locals.var_delphib_dn14) * assign41370_e54308) + (assign41370_e54305 * (-locals.var_delphib_dn14))) / (2.0 * assign41370_e54312));
        locals.var_temp2_dn15 = ((((-locals.var_delphib_dn15) * assign41370_e54308) + (assign41370_e54305 * (-locals.var_delphib_dn15))) / (2.0 * assign41370_e54312));
        locals.var_temp2_dn16 = ((((-locals.var_delphib_dn16) * assign41370_e54308) + (assign41370_e54305 * (-locals.var_delphib_dn16))) / (2.0 * assign41370_e54312));
        locals.var_temp2_dn17 = ((((-locals.var_delphib_dn17) * assign41370_e54308) + (assign41370_e54305 * (-locals.var_delphib_dn17))) / (2.0 * assign41370_e54312));
        locals.var_temp2_dn18 = ((((-locals.var_delphib_dn18) * assign41370_e54308) + (assign41370_e54305 * (-locals.var_delphib_dn18))) / (2.0 * assign41370_e54312));
        locals.var_temp2_dn19 = ((((-locals.var_delphib_dn19) * assign41370_e54308) + (assign41370_e54305 * (-locals.var_delphib_dn19))) / (2.0 * assign41370_e54312));
        locals.var_temp2_dn20 = ((((-locals.var_delphib_dn20) * assign41370_e54308) + (assign41370_e54305 * (-locals.var_delphib_dn20))) / (2.0 * assign41370_e54312));
        locals.var_temp2_rv = 0.0;

        let assign41380_e54315: f64 = (0.5 * locals.var_inv_phit1);
        let assign41380_e54318: f64 = (locals.var_delphib + locals.var_temp1);
        let assign41380_e54320: f64 = (assign41380_e54318 - locals.var_temp2);
        let assign41380_e54321: f64 = (assign41380_e54315 * assign41380_e54320);
        locals.var_delxb = assign41380_e54321;
        locals.var_delxb_dn5 = (((0.5 * locals.var_inv_phit1_dn5) * assign41380_e54320) + (assign41380_e54315 * ((locals.var_delphib_dn5 + locals.var_temp1_dn5) - locals.var_temp2_dn5)));
        locals.var_delxb_dn6 = (((0.5 * locals.var_inv_phit1_dn6) * assign41380_e54320) + (assign41380_e54315 * ((locals.var_delphib_dn6 + locals.var_temp1_dn6) - locals.var_temp2_dn6)));
        locals.var_delxb_dn7 = (((0.5 * locals.var_inv_phit1_dn7) * assign41380_e54320) + (assign41380_e54315 * ((locals.var_delphib_dn7 + locals.var_temp1_dn7) - locals.var_temp2_dn7)));
        locals.var_delxb_dn8 = (((0.5 * locals.var_inv_phit1_dn8) * assign41380_e54320) + (assign41380_e54315 * ((locals.var_delphib_dn8 + locals.var_temp1_dn8) - locals.var_temp2_dn8)));
        locals.var_delxb_dn12 = (((0.5 * locals.var_inv_phit1_dn12) * assign41380_e54320) + (assign41380_e54315 * ((locals.var_delphib_dn12 + locals.var_temp1_dn12) - locals.var_temp2_dn12)));
        locals.var_delxb_dn13 = (((0.5 * locals.var_inv_phit1_dn13) * assign41380_e54320) + (assign41380_e54315 * ((locals.var_delphib_dn13 + locals.var_temp1_dn13) - locals.var_temp2_dn13)));
        locals.var_delxb_dn14 = (((0.5 * locals.var_inv_phit1_dn14) * assign41380_e54320) + (assign41380_e54315 * ((locals.var_delphib_dn14 + locals.var_temp1_dn14) - locals.var_temp2_dn14)));
        locals.var_delxb_dn15 = (((0.5 * locals.var_inv_phit1_dn15) * assign41380_e54320) + (assign41380_e54315 * ((locals.var_delphib_dn15 + locals.var_temp1_dn15) - locals.var_temp2_dn15)));
        locals.var_delxb_dn16 = (((0.5 * locals.var_inv_phit1_dn16) * assign41380_e54320) + (assign41380_e54315 * ((locals.var_delphib_dn16 + locals.var_temp1_dn16) - locals.var_temp2_dn16)));
        locals.var_delxb_dn17 = (((0.5 * locals.var_inv_phit1_dn17) * assign41380_e54320) + (assign41380_e54315 * ((locals.var_delphib_dn17 + locals.var_temp1_dn17) - locals.var_temp2_dn17)));
        locals.var_delxb_dn18 = (((0.5 * locals.var_inv_phit1_dn18) * assign41380_e54320) + (assign41380_e54315 * ((locals.var_delphib_dn18 + locals.var_temp1_dn18) - locals.var_temp2_dn18)));
        locals.var_delxb_dn19 = (((0.5 * locals.var_inv_phit1_dn19) * assign41380_e54320) + (assign41380_e54315 * ((locals.var_delphib_dn19 + locals.var_temp1_dn19) - locals.var_temp2_dn19)));
        locals.var_delxb_dn20 = (((0.5 * locals.var_inv_phit1_dn20) * assign41380_e54320) + (assign41380_e54315 * ((locals.var_delphib_dn20 + locals.var_temp1_dn20) - locals.var_temp2_dn20)));
        locals.var_delxb_rv = 0.0;

        let assign41390_e54324: f64 = (locals.var_xb + locals.var_ux);
        locals.var_xno_s = assign41390_e54324;
        locals.var_xno_s_dn5 = (locals.var_xb_dn5 + locals.var_ux_dn5);
        locals.var_xno_s_dn6 = (locals.var_xb_dn6 + locals.var_ux_dn6);
        locals.var_xno_s_dn7 = (locals.var_xb_dn7 + locals.var_ux_dn7);
        locals.var_xno_s_dn8 = (locals.var_xb_dn8 + locals.var_ux_dn8);
        locals.var_xno_s_dn12 = (locals.var_xb_dn12 + locals.var_ux_dn12);
        locals.var_xno_s_dn13 = (locals.var_xb_dn13 + locals.var_ux_dn13);
        locals.var_xno_s_dn14 = (locals.var_xb_dn14 + locals.var_ux_dn14);
        locals.var_xno_s_dn15 = (locals.var_xb_dn15 + locals.var_ux_dn15);
        locals.var_xno_s_dn16 = (locals.var_xb_dn16 + locals.var_ux_dn16);
        locals.var_xno_s_dn17 = (locals.var_xb_dn17 + locals.var_ux_dn17);
        locals.var_xno_s_dn18 = (locals.var_xb_dn18 + locals.var_ux_dn18);
        locals.var_xno_s_dn19 = (locals.var_xb_dn19 + locals.var_ux_dn19);
        locals.var_xno_s_dn20 = (locals.var_xb_dn20 + locals.var_ux_dn20);
        locals.var_xno_s_rv = 0.0;

        let assign41400_e54327: f64 = (locals.var_xno_s - locals.var_delxb);
        locals.var_xn_s = assign41400_e54327;
        locals.var_xn_s_dn5 = (locals.var_xno_s_dn5 - locals.var_delxb_dn5);
        locals.var_xn_s_dn6 = (locals.var_xno_s_dn6 - locals.var_delxb_dn6);
        locals.var_xn_s_dn7 = (locals.var_xno_s_dn7 - locals.var_delxb_dn7);
        locals.var_xn_s_dn8 = (locals.var_xno_s_dn8 - locals.var_delxb_dn8);
        locals.var_xn_s_dn12 = (locals.var_xno_s_dn12 - locals.var_delxb_dn12);
        locals.var_xn_s_dn13 = (locals.var_xno_s_dn13 - locals.var_delxb_dn13);
        locals.var_xn_s_dn14 = (locals.var_xno_s_dn14 - locals.var_delxb_dn14);
        locals.var_xn_s_dn15 = (locals.var_xno_s_dn15 - locals.var_delxb_dn15);
        locals.var_xn_s_dn16 = (locals.var_xno_s_dn16 - locals.var_delxb_dn16);
        locals.var_xn_s_dn17 = (locals.var_xno_s_dn17 - locals.var_delxb_dn17);
        locals.var_xn_s_dn18 = (locals.var_xno_s_dn18 - locals.var_delxb_dn18);
        locals.var_xn_s_dn19 = (locals.var_xno_s_dn19 - locals.var_delxb_dn19);
        locals.var_xn_s_dn20 = (locals.var_xno_s_dn20 - locals.var_delxb_dn20);
        locals.var_xn_s_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_18(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign41410_e54330: f64 = if p.p45 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1277 = assign41410_e54330;
        locals.var_guard1277_rv = 0.0;

        let assign41420_e54332: f64 = (locals.var_xn_s).abs();
        let assign41420_e54334: f64 = if assign41420_e54332 < 1e-5 { 1.0 } else { 0.0 };
        locals.var_guard1278 = assign41420_e54334;
        locals.var_guard1278_rv = 0.0;

        let (assign41430_e54354, assign41430_e54354_d_n5, assign41430_e54354_d_n6, assign41430_e54354_d_n7, assign41430_e54354_d_n8, assign41430_e54354_d_n12, assign41430_e54354_d_n13, assign41430_e54354_d_n14, assign41430_e54354_d_n15, assign41430_e54354_d_n16, assign41430_e54354_d_n17, assign41430_e54354_d_n18, assign41430_e54354_d_n19, assign41430_e54354_d_n20,) = {
    if ((locals.var_guard1277 != 0.0) && (locals.var_guard1278 != 0.0)) {
        let assign41430_e54343: f64 = (0.5 * locals.var_xn_s);
        let assign41430_e54347: f64 = (0.3125 * locals.var_xn_s);
        let assign41430_e54348: f64 = (1.0 - assign41430_e54347);
        let assign41430_e54349: f64 = (assign41430_e54343 * assign41430_e54348);
        let assign41430_e54350: f64 = (1.0 - assign41430_e54349);
        let assign41430_e54351: f64 = (locals.var_gf * assign41430_e54350);
        let assign41430_e54352: f64 = (1.0 + assign41430_e54351);
        (assign41430_e54352, ((locals.var_gf_dn5 * assign41430_e54350) + (locals.var_gf * (-(((0.5 * locals.var_xn_s_dn5) * assign41430_e54348) + (assign41430_e54343 * (-(0.3125 * locals.var_xn_s_dn5))))))), ((locals.var_gf_dn6 * assign41430_e54350) + (locals.var_gf * (-(((0.5 * locals.var_xn_s_dn6) * assign41430_e54348) + (assign41430_e54343 * (-(0.3125 * locals.var_xn_s_dn6))))))), ((locals.var_gf_dn7 * assign41430_e54350) + (locals.var_gf * (-(((0.5 * locals.var_xn_s_dn7) * assign41430_e54348) + (assign41430_e54343 * (-(0.3125 * locals.var_xn_s_dn7))))))), ((locals.var_gf_dn8 * assign41430_e54350) + (locals.var_gf * (-(((0.5 * locals.var_xn_s_dn8) * assign41430_e54348) + (assign41430_e54343 * (-(0.3125 * locals.var_xn_s_dn8))))))), ((locals.var_gf_dn12 * assign41430_e54350) + (locals.var_gf * (-(((0.5 * locals.var_xn_s_dn12) * assign41430_e54348) + (assign41430_e54343 * (-(0.3125 * locals.var_xn_s_dn12))))))), ((locals.var_gf_dn13 * assign41430_e54350) + (locals.var_gf * (-(((0.5 * locals.var_xn_s_dn13) * assign41430_e54348) + (assign41430_e54343 * (-(0.3125 * locals.var_xn_s_dn13))))))), ((locals.var_gf_dn14 * assign41430_e54350) + (locals.var_gf * (-(((0.5 * locals.var_xn_s_dn14) * assign41430_e54348) + (assign41430_e54343 * (-(0.3125 * locals.var_xn_s_dn14))))))), ((locals.var_gf_dn15 * assign41430_e54350) + (locals.var_gf * (-(((0.5 * locals.var_xn_s_dn15) * assign41430_e54348) + (assign41430_e54343 * (-(0.3125 * locals.var_xn_s_dn15))))))), ((locals.var_gf_dn16 * assign41430_e54350) + (locals.var_gf * (-(((0.5 * locals.var_xn_s_dn16) * assign41430_e54348) + (assign41430_e54343 * (-(0.3125 * locals.var_xn_s_dn16))))))), ((locals.var_gf_dn17 * assign41430_e54350) + (locals.var_gf * (-(((0.5 * locals.var_xn_s_dn17) * assign41430_e54348) + (assign41430_e54343 * (-(0.3125 * locals.var_xn_s_dn17))))))), ((locals.var_gf_dn18 * assign41430_e54350) + (locals.var_gf * (-(((0.5 * locals.var_xn_s_dn18) * assign41430_e54348) + (assign41430_e54343 * (-(0.3125 * locals.var_xn_s_dn18))))))), ((locals.var_gf_dn19 * assign41430_e54350) + (locals.var_gf * (-(((0.5 * locals.var_xn_s_dn19) * assign41430_e54348) + (assign41430_e54343 * (-(0.3125 * locals.var_xn_s_dn19))))))), ((locals.var_gf_dn20 * assign41430_e54350) + (locals.var_gf * (-(((0.5 * locals.var_xn_s_dn20) * assign41430_e54348) + (assign41430_e54343 * (-(0.3125 * locals.var_xn_s_dn20))))))),)
    } else {
        (locals.var_nscr, locals.var_nscr_dn5, locals.var_nscr_dn6, locals.var_nscr_dn7, locals.var_nscr_dn8, locals.var_nscr_dn12, locals.var_nscr_dn13, locals.var_nscr_dn14, locals.var_nscr_dn15, locals.var_nscr_dn16, locals.var_nscr_dn17, locals.var_nscr_dn18, locals.var_nscr_dn19, locals.var_nscr_dn20,)
    }
};
        locals.var_nscr = assign41430_e54354;
        locals.var_nscr_dn5 = assign41430_e54354_d_n5;
        locals.var_nscr_dn6 = assign41430_e54354_d_n6;
        locals.var_nscr_dn7 = assign41430_e54354_d_n7;
        locals.var_nscr_dn8 = assign41430_e54354_d_n8;
        locals.var_nscr_dn12 = assign41430_e54354_d_n12;
        locals.var_nscr_dn13 = assign41430_e54354_d_n13;
        locals.var_nscr_dn14 = assign41430_e54354_d_n14;
        locals.var_nscr_dn15 = assign41430_e54354_d_n15;
        locals.var_nscr_dn16 = assign41430_e54354_d_n16;
        locals.var_nscr_dn17 = assign41430_e54354_d_n17;
        locals.var_nscr_dn18 = assign41430_e54354_d_n18;
        locals.var_nscr_dn19 = assign41430_e54354_d_n19;
        locals.var_nscr_dn20 = assign41430_e54354_d_n20;
        locals.var_nscr_rv = 0.0;

        let assign41440_e54357: f64 = if locals.var_xn_s < 460.51701859880916 { 1.0 } else { 0.0 };
        locals.var_guard1279 = assign41440_e54357;
        locals.var_guard1279_rv = 0.0;

        let (assign41450_e54368, assign41450_e54368_d_n5, assign41450_e54368_d_n6, assign41450_e54368_d_n7, assign41450_e54368_d_n8, assign41450_e54368_d_n12, assign41450_e54368_d_n13, assign41450_e54368_d_n14, assign41450_e54368_d_n15, assign41450_e54368_d_n16, assign41450_e54368_d_n17, assign41450_e54368_d_n18, assign41450_e54368_d_n19, assign41450_e54368_d_n20,) = {
    if (((locals.var_guard1277 != 0.0) && (locals.var_guard1278 == 0.0)) && (locals.var_guard1279 != 0.0)) {
        let assign41450_e54365: f64 = (-locals.var_xn_s);
        let assign41450_e54366: f64 = (assign41450_e54365).exp();
        (assign41450_e54366, (assign41450_e54366 * (-locals.var_xn_s_dn5)), (assign41450_e54366 * (-locals.var_xn_s_dn6)), (assign41450_e54366 * (-locals.var_xn_s_dn7)), (assign41450_e54366 * (-locals.var_xn_s_dn8)), (assign41450_e54366 * (-locals.var_xn_s_dn12)), (assign41450_e54366 * (-locals.var_xn_s_dn13)), (assign41450_e54366 * (-locals.var_xn_s_dn14)), (assign41450_e54366 * (-locals.var_xn_s_dn15)), (assign41450_e54366 * (-locals.var_xn_s_dn16)), (assign41450_e54366 * (-locals.var_xn_s_dn17)), (assign41450_e54366 * (-locals.var_xn_s_dn18)), (assign41450_e54366 * (-locals.var_xn_s_dn19)), (assign41450_e54366 * (-locals.var_xn_s_dn20)),)
    } else {
        (locals.var_delta_ns, locals.var_delta_ns_dn5, locals.var_delta_ns_dn6, locals.var_delta_ns_dn7, locals.var_delta_ns_dn8, locals.var_delta_ns_dn12, locals.var_delta_ns_dn13, locals.var_delta_ns_dn14, locals.var_delta_ns_dn15, locals.var_delta_ns_dn16, locals.var_delta_ns_dn17, locals.var_delta_ns_dn18, locals.var_delta_ns_dn19, locals.var_delta_ns_dn20,)
    }
};
        locals.var_delta_ns = assign41450_e54368;
        locals.var_delta_ns_dn5 = assign41450_e54368_d_n5;
        locals.var_delta_ns_dn6 = assign41450_e54368_d_n6;
        locals.var_delta_ns_dn7 = assign41450_e54368_d_n7;
        locals.var_delta_ns_dn8 = assign41450_e54368_d_n8;
        locals.var_delta_ns_dn12 = assign41450_e54368_d_n12;
        locals.var_delta_ns_dn13 = assign41450_e54368_d_n13;
        locals.var_delta_ns_dn14 = assign41450_e54368_d_n14;
        locals.var_delta_ns_dn15 = assign41450_e54368_d_n15;
        locals.var_delta_ns_dn16 = assign41450_e54368_d_n16;
        locals.var_delta_ns_dn17 = assign41450_e54368_d_n17;
        locals.var_delta_ns_dn18 = assign41450_e54368_d_n18;
        locals.var_delta_ns_dn19 = assign41450_e54368_d_n19;
        locals.var_delta_ns_dn20 = assign41450_e54368_d_n20;
        locals.var_delta_ns_rv = 0.0;

        let (assign41460_e54400, assign41460_e54400_d_n5, assign41460_e54400_d_n6, assign41460_e54400_d_n7, assign41460_e54400_d_n8, assign41460_e54400_d_n12, assign41460_e54400_d_n13, assign41460_e54400_d_n14, assign41460_e54400_d_n15, assign41460_e54400_d_n16, assign41460_e54400_d_n17, assign41460_e54400_d_n18, assign41460_e54400_d_n19, assign41460_e54400_d_n20,) = {
    if (((locals.var_guard1277 != 0.0) && (locals.var_guard1278 == 0.0)) && (locals.var_guard1279 == 0.0)) {
        let assign41460_e54380: f64 = (locals.var_xn_s - 460.51701859880916);
        let assign41460_e54385: f64 = (locals.var_xn_s - 460.51701859880916);
        let assign41460_e54389: f64 = (locals.var_xn_s - 460.51701859880916);
        let assign41460_e54391: f64 = (assign41460_e54389 * 0.3333333333333333);
        let assign41460_e54392: f64 = (1.0 + assign41460_e54391);
        let assign41460_e54393: f64 = (assign41460_e54385 * assign41460_e54392);
        let assign41460_e54394: f64 = (0.5 * assign41460_e54393);
        let assign41460_e54395: f64 = (1.0 + assign41460_e54394);
        let assign41460_e54396: f64 = (assign41460_e54380 * assign41460_e54395);
        let assign41460_e54397: f64 = (1.0 + assign41460_e54396);
        let assign41460_e54398: f64 = (1e-200 / assign41460_e54397);
        (assign41460_e54398, (-((1e-200 * ((locals.var_xn_s_dn5 * assign41460_e54395) + (assign41460_e54380 * (0.5 * ((locals.var_xn_s_dn5 * assign41460_e54392) + (assign41460_e54385 * (locals.var_xn_s_dn5 * 0.3333333333333333))))))) / (assign41460_e54397 * assign41460_e54397))), (-((1e-200 * ((locals.var_xn_s_dn6 * assign41460_e54395) + (assign41460_e54380 * (0.5 * ((locals.var_xn_s_dn6 * assign41460_e54392) + (assign41460_e54385 * (locals.var_xn_s_dn6 * 0.3333333333333333))))))) / (assign41460_e54397 * assign41460_e54397))), (-((1e-200 * ((locals.var_xn_s_dn7 * assign41460_e54395) + (assign41460_e54380 * (0.5 * ((locals.var_xn_s_dn7 * assign41460_e54392) + (assign41460_e54385 * (locals.var_xn_s_dn7 * 0.3333333333333333))))))) / (assign41460_e54397 * assign41460_e54397))), (-((1e-200 * ((locals.var_xn_s_dn8 * assign41460_e54395) + (assign41460_e54380 * (0.5 * ((locals.var_xn_s_dn8 * assign41460_e54392) + (assign41460_e54385 * (locals.var_xn_s_dn8 * 0.3333333333333333))))))) / (assign41460_e54397 * assign41460_e54397))), (-((1e-200 * ((locals.var_xn_s_dn12 * assign41460_e54395) + (assign41460_e54380 * (0.5 * ((locals.var_xn_s_dn12 * assign41460_e54392) + (assign41460_e54385 * (locals.var_xn_s_dn12 * 0.3333333333333333))))))) / (assign41460_e54397 * assign41460_e54397))), (-((1e-200 * ((locals.var_xn_s_dn13 * assign41460_e54395) + (assign41460_e54380 * (0.5 * ((locals.var_xn_s_dn13 * assign41460_e54392) + (assign41460_e54385 * (locals.var_xn_s_dn13 * 0.3333333333333333))))))) / (assign41460_e54397 * assign41460_e54397))), (-((1e-200 * ((locals.var_xn_s_dn14 * assign41460_e54395) + (assign41460_e54380 * (0.5 * ((locals.var_xn_s_dn14 * assign41460_e54392) + (assign41460_e54385 * (locals.var_xn_s_dn14 * 0.3333333333333333))))))) / (assign41460_e54397 * assign41460_e54397))), (-((1e-200 * ((locals.var_xn_s_dn15 * assign41460_e54395) + (assign41460_e54380 * (0.5 * ((locals.var_xn_s_dn15 * assign41460_e54392) + (assign41460_e54385 * (locals.var_xn_s_dn15 * 0.3333333333333333))))))) / (assign41460_e54397 * assign41460_e54397))), (-((1e-200 * ((locals.var_xn_s_dn16 * assign41460_e54395) + (assign41460_e54380 * (0.5 * ((locals.var_xn_s_dn16 * assign41460_e54392) + (assign41460_e54385 * (locals.var_xn_s_dn16 * 0.3333333333333333))))))) / (assign41460_e54397 * assign41460_e54397))), (-((1e-200 * ((locals.var_xn_s_dn17 * assign41460_e54395) + (assign41460_e54380 * (0.5 * ((locals.var_xn_s_dn17 * assign41460_e54392) + (assign41460_e54385 * (locals.var_xn_s_dn17 * 0.3333333333333333))))))) / (assign41460_e54397 * assign41460_e54397))), (-((1e-200 * ((locals.var_xn_s_dn18 * assign41460_e54395) + (assign41460_e54380 * (0.5 * ((locals.var_xn_s_dn18 * assign41460_e54392) + (assign41460_e54385 * (locals.var_xn_s_dn18 * 0.3333333333333333))))))) / (assign41460_e54397 * assign41460_e54397))), (-((1e-200 * ((locals.var_xn_s_dn19 * assign41460_e54395) + (assign41460_e54380 * (0.5 * ((locals.var_xn_s_dn19 * assign41460_e54392) + (assign41460_e54385 * (locals.var_xn_s_dn19 * 0.3333333333333333))))))) / (assign41460_e54397 * assign41460_e54397))), (-((1e-200 * ((locals.var_xn_s_dn20 * assign41460_e54395) + (assign41460_e54380 * (0.5 * ((locals.var_xn_s_dn20 * assign41460_e54392) + (assign41460_e54385 * (locals.var_xn_s_dn20 * 0.3333333333333333))))))) / (assign41460_e54397 * assign41460_e54397))),)
    } else {
        (locals.var_delta_ns, locals.var_delta_ns_dn5, locals.var_delta_ns_dn6, locals.var_delta_ns_dn7, locals.var_delta_ns_dn8, locals.var_delta_ns_dn12, locals.var_delta_ns_dn13, locals.var_delta_ns_dn14, locals.var_delta_ns_dn15, locals.var_delta_ns_dn16, locals.var_delta_ns_dn17, locals.var_delta_ns_dn18, locals.var_delta_ns_dn19, locals.var_delta_ns_dn20,)
    }
};
        locals.var_delta_ns = assign41460_e54400;
        locals.var_delta_ns_dn5 = assign41460_e54400_d_n5;
        locals.var_delta_ns_dn6 = assign41460_e54400_d_n6;
        locals.var_delta_ns_dn7 = assign41460_e54400_d_n7;
        locals.var_delta_ns_dn8 = assign41460_e54400_d_n8;
        locals.var_delta_ns_dn12 = assign41460_e54400_d_n12;
        locals.var_delta_ns_dn13 = assign41460_e54400_d_n13;
        locals.var_delta_ns_dn14 = assign41460_e54400_d_n14;
        locals.var_delta_ns_dn15 = assign41460_e54400_d_n15;
        locals.var_delta_ns_dn16 = assign41460_e54400_d_n16;
        locals.var_delta_ns_dn17 = assign41460_e54400_d_n17;
        locals.var_delta_ns_dn18 = assign41460_e54400_d_n18;
        locals.var_delta_ns_dn19 = assign41460_e54400_d_n19;
        locals.var_delta_ns_dn20 = assign41460_e54400_d_n20;
        locals.var_delta_ns_rv = 0.0;

        let (assign41470_e54413, assign41470_e54413_d_n5, assign41470_e54413_d_n6, assign41470_e54413_d_n7, assign41470_e54413_d_n8, assign41470_e54413_d_n12, assign41470_e54413_d_n13, assign41470_e54413_d_n14, assign41470_e54413_d_n15, assign41470_e54413_d_n16, assign41470_e54413_d_n17, assign41470_e54413_d_n18, assign41470_e54413_d_n19, assign41470_e54413_d_n20,) = {
    if ((locals.var_guard1277 != 0.0) && (locals.var_guard1278 == 0.0)) {
        let (assign41470_e54411,) = {
            if (locals.var_xn_s > 0.0) {
                (1.0,)
            } else {
                let assign41470_e54410: f64 = (-1.0);
                (assign41470_e54410,)
            }
        };
        (assign41470_e54411, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp__blk1038, locals.var_temp__blk1038_dn5, locals.var_temp__blk1038_dn6, locals.var_temp__blk1038_dn7, locals.var_temp__blk1038_dn8, locals.var_temp__blk1038_dn12, locals.var_temp__blk1038_dn13, locals.var_temp__blk1038_dn14, locals.var_temp__blk1038_dn15, locals.var_temp__blk1038_dn16, locals.var_temp__blk1038_dn17, locals.var_temp__blk1038_dn18, locals.var_temp__blk1038_dn19, locals.var_temp__blk1038_dn20,)
    }
};
        locals.var_temp__blk1038 = assign41470_e54413;
        locals.var_temp__blk1038_dn5 = assign41470_e54413_d_n5;
        locals.var_temp__blk1038_dn6 = assign41470_e54413_d_n6;
        locals.var_temp__blk1038_dn7 = assign41470_e54413_d_n7;
        locals.var_temp__blk1038_dn8 = assign41470_e54413_d_n8;
        locals.var_temp__blk1038_dn12 = assign41470_e54413_d_n12;
        locals.var_temp__blk1038_dn13 = assign41470_e54413_d_n13;
        locals.var_temp__blk1038_dn14 = assign41470_e54413_d_n14;
        locals.var_temp__blk1038_dn15 = assign41470_e54413_d_n15;
        locals.var_temp__blk1038_dn16 = assign41470_e54413_d_n16;
        locals.var_temp__blk1038_dn17 = assign41470_e54413_d_n17;
        locals.var_temp__blk1038_dn18 = assign41470_e54413_d_n18;
        locals.var_temp__blk1038_dn19 = assign41470_e54413_d_n19;
        locals.var_temp__blk1038_dn20 = assign41470_e54413_d_n20;
        locals.var_temp__blk1038_rv = 0.0;

        let (assign41480_e54441, assign41480_e54441_d_n5, assign41480_e54441_d_n6, assign41480_e54441_d_n7, assign41480_e54441_d_n8, assign41480_e54441_d_n12, assign41480_e54441_d_n13, assign41480_e54441_d_n14, assign41480_e54441_d_n15, assign41480_e54441_d_n16, assign41480_e54441_d_n17, assign41480_e54441_d_n18, assign41480_e54441_d_n19, assign41480_e54441_d_n20,) = {
    if ((locals.var_guard1277 != 0.0) && (locals.var_guard1278 == 0.0)) {
        let assign41480_e54421: f64 = (locals.var_temp__blk1038 * locals.var_gf);
        let assign41480_e54426: f64 = (1.0 - locals.var_xn_s);
        let assign41480_e54427: f64 = (locals.var_delta_ns * assign41480_e54426);
        let assign41480_e54428: f64 = (1.0 - assign41480_e54427);
        let assign41480_e54429: f64 = (assign41480_e54421 * assign41480_e54428);
        let assign41480_e54434: f64 = (1.0 - locals.var_delta_ns);
        let assign41480_e54435: f64 = (locals.var_xn_s * assign41480_e54434);
        let assign41480_e54436: f64 = (assign41480_e54435).sqrt();
        let assign41480_e54437: f64 = (2.0 * assign41480_e54436);
        let assign41480_e54438: f64 = (assign41480_e54429 / assign41480_e54437);
        let assign41480_e54439: f64 = (1.0 + assign41480_e54438);
        (assign41480_e54439, (((((((locals.var_temp__blk1038_dn5 * locals.var_gf) + (locals.var_temp__blk1038 * locals.var_gf_dn5)) * assign41480_e54428) + (assign41480_e54421 * (-((locals.var_delta_ns_dn5 * assign41480_e54426) + (locals.var_delta_ns * (-locals.var_xn_s_dn5)))))) * assign41480_e54437) - (assign41480_e54429 * (2.0 * (((locals.var_xn_s_dn5 * assign41480_e54434) + (locals.var_xn_s * (-locals.var_delta_ns_dn5))) / (2.0 * assign41480_e54436))))) / (assign41480_e54437 * assign41480_e54437)), (((((((locals.var_temp__blk1038_dn6 * locals.var_gf) + (locals.var_temp__blk1038 * locals.var_gf_dn6)) * assign41480_e54428) + (assign41480_e54421 * (-((locals.var_delta_ns_dn6 * assign41480_e54426) + (locals.var_delta_ns * (-locals.var_xn_s_dn6)))))) * assign41480_e54437) - (assign41480_e54429 * (2.0 * (((locals.var_xn_s_dn6 * assign41480_e54434) + (locals.var_xn_s * (-locals.var_delta_ns_dn6))) / (2.0 * assign41480_e54436))))) / (assign41480_e54437 * assign41480_e54437)), (((((((locals.var_temp__blk1038_dn7 * locals.var_gf) + (locals.var_temp__blk1038 * locals.var_gf_dn7)) * assign41480_e54428) + (assign41480_e54421 * (-((locals.var_delta_ns_dn7 * assign41480_e54426) + (locals.var_delta_ns * (-locals.var_xn_s_dn7)))))) * assign41480_e54437) - (assign41480_e54429 * (2.0 * (((locals.var_xn_s_dn7 * assign41480_e54434) + (locals.var_xn_s * (-locals.var_delta_ns_dn7))) / (2.0 * assign41480_e54436))))) / (assign41480_e54437 * assign41480_e54437)), (((((((locals.var_temp__blk1038_dn8 * locals.var_gf) + (locals.var_temp__blk1038 * locals.var_gf_dn8)) * assign41480_e54428) + (assign41480_e54421 * (-((locals.var_delta_ns_dn8 * assign41480_e54426) + (locals.var_delta_ns * (-locals.var_xn_s_dn8)))))) * assign41480_e54437) - (assign41480_e54429 * (2.0 * (((locals.var_xn_s_dn8 * assign41480_e54434) + (locals.var_xn_s * (-locals.var_delta_ns_dn8))) / (2.0 * assign41480_e54436))))) / (assign41480_e54437 * assign41480_e54437)), (((((((locals.var_temp__blk1038_dn12 * locals.var_gf) + (locals.var_temp__blk1038 * locals.var_gf_dn12)) * assign41480_e54428) + (assign41480_e54421 * (-((locals.var_delta_ns_dn12 * assign41480_e54426) + (locals.var_delta_ns * (-locals.var_xn_s_dn12)))))) * assign41480_e54437) - (assign41480_e54429 * (2.0 * (((locals.var_xn_s_dn12 * assign41480_e54434) + (locals.var_xn_s * (-locals.var_delta_ns_dn12))) / (2.0 * assign41480_e54436))))) / (assign41480_e54437 * assign41480_e54437)), (((((((locals.var_temp__blk1038_dn13 * locals.var_gf) + (locals.var_temp__blk1038 * locals.var_gf_dn13)) * assign41480_e54428) + (assign41480_e54421 * (-((locals.var_delta_ns_dn13 * assign41480_e54426) + (locals.var_delta_ns * (-locals.var_xn_s_dn13)))))) * assign41480_e54437) - (assign41480_e54429 * (2.0 * (((locals.var_xn_s_dn13 * assign41480_e54434) + (locals.var_xn_s * (-locals.var_delta_ns_dn13))) / (2.0 * assign41480_e54436))))) / (assign41480_e54437 * assign41480_e54437)), (((((((locals.var_temp__blk1038_dn14 * locals.var_gf) + (locals.var_temp__blk1038 * locals.var_gf_dn14)) * assign41480_e54428) + (assign41480_e54421 * (-((locals.var_delta_ns_dn14 * assign41480_e54426) + (locals.var_delta_ns * (-locals.var_xn_s_dn14)))))) * assign41480_e54437) - (assign41480_e54429 * (2.0 * (((locals.var_xn_s_dn14 * assign41480_e54434) + (locals.var_xn_s * (-locals.var_delta_ns_dn14))) / (2.0 * assign41480_e54436))))) / (assign41480_e54437 * assign41480_e54437)), (((((((locals.var_temp__blk1038_dn15 * locals.var_gf) + (locals.var_temp__blk1038 * locals.var_gf_dn15)) * assign41480_e54428) + (assign41480_e54421 * (-((locals.var_delta_ns_dn15 * assign41480_e54426) + (locals.var_delta_ns * (-locals.var_xn_s_dn15)))))) * assign41480_e54437) - (assign41480_e54429 * (2.0 * (((locals.var_xn_s_dn15 * assign41480_e54434) + (locals.var_xn_s * (-locals.var_delta_ns_dn15))) / (2.0 * assign41480_e54436))))) / (assign41480_e54437 * assign41480_e54437)), (((((((locals.var_temp__blk1038_dn16 * locals.var_gf) + (locals.var_temp__blk1038 * locals.var_gf_dn16)) * assign41480_e54428) + (assign41480_e54421 * (-((locals.var_delta_ns_dn16 * assign41480_e54426) + (locals.var_delta_ns * (-locals.var_xn_s_dn16)))))) * assign41480_e54437) - (assign41480_e54429 * (2.0 * (((locals.var_xn_s_dn16 * assign41480_e54434) + (locals.var_xn_s * (-locals.var_delta_ns_dn16))) / (2.0 * assign41480_e54436))))) / (assign41480_e54437 * assign41480_e54437)), (((((((locals.var_temp__blk1038_dn17 * locals.var_gf) + (locals.var_temp__blk1038 * locals.var_gf_dn17)) * assign41480_e54428) + (assign41480_e54421 * (-((locals.var_delta_ns_dn17 * assign41480_e54426) + (locals.var_delta_ns * (-locals.var_xn_s_dn17)))))) * assign41480_e54437) - (assign41480_e54429 * (2.0 * (((locals.var_xn_s_dn17 * assign41480_e54434) + (locals.var_xn_s * (-locals.var_delta_ns_dn17))) / (2.0 * assign41480_e54436))))) / (assign41480_e54437 * assign41480_e54437)), (((((((locals.var_temp__blk1038_dn18 * locals.var_gf) + (locals.var_temp__blk1038 * locals.var_gf_dn18)) * assign41480_e54428) + (assign41480_e54421 * (-((locals.var_delta_ns_dn18 * assign41480_e54426) + (locals.var_delta_ns * (-locals.var_xn_s_dn18)))))) * assign41480_e54437) - (assign41480_e54429 * (2.0 * (((locals.var_xn_s_dn18 * assign41480_e54434) + (locals.var_xn_s * (-locals.var_delta_ns_dn18))) / (2.0 * assign41480_e54436))))) / (assign41480_e54437 * assign41480_e54437)), (((((((locals.var_temp__blk1038_dn19 * locals.var_gf) + (locals.var_temp__blk1038 * locals.var_gf_dn19)) * assign41480_e54428) + (assign41480_e54421 * (-((locals.var_delta_ns_dn19 * assign41480_e54426) + (locals.var_delta_ns * (-locals.var_xn_s_dn19)))))) * assign41480_e54437) - (assign41480_e54429 * (2.0 * (((locals.var_xn_s_dn19 * assign41480_e54434) + (locals.var_xn_s * (-locals.var_delta_ns_dn19))) / (2.0 * assign41480_e54436))))) / (assign41480_e54437 * assign41480_e54437)), (((((((locals.var_temp__blk1038_dn20 * locals.var_gf) + (locals.var_temp__blk1038 * locals.var_gf_dn20)) * assign41480_e54428) + (assign41480_e54421 * (-((locals.var_delta_ns_dn20 * assign41480_e54426) + (locals.var_delta_ns * (-locals.var_xn_s_dn20)))))) * assign41480_e54437) - (assign41480_e54429 * (2.0 * (((locals.var_xn_s_dn20 * assign41480_e54434) + (locals.var_xn_s * (-locals.var_delta_ns_dn20))) / (2.0 * assign41480_e54436))))) / (assign41480_e54437 * assign41480_e54437)),)
    } else {
        (locals.var_nscr, locals.var_nscr_dn5, locals.var_nscr_dn6, locals.var_nscr_dn7, locals.var_nscr_dn8, locals.var_nscr_dn12, locals.var_nscr_dn13, locals.var_nscr_dn14, locals.var_nscr_dn15, locals.var_nscr_dn16, locals.var_nscr_dn17, locals.var_nscr_dn18, locals.var_nscr_dn19, locals.var_nscr_dn20,)
    }
};
        locals.var_nscr = assign41480_e54441;
        locals.var_nscr_dn5 = assign41480_e54441_d_n5;
        locals.var_nscr_dn6 = assign41480_e54441_d_n6;
        locals.var_nscr_dn7 = assign41480_e54441_d_n7;
        locals.var_nscr_dn8 = assign41480_e54441_d_n8;
        locals.var_nscr_dn12 = assign41480_e54441_d_n12;
        locals.var_nscr_dn13 = assign41480_e54441_d_n13;
        locals.var_nscr_dn14 = assign41480_e54441_d_n14;
        locals.var_nscr_dn15 = assign41480_e54441_d_n15;
        locals.var_nscr_dn16 = assign41480_e54441_d_n16;
        locals.var_nscr_dn17 = assign41480_e54441_d_n17;
        locals.var_nscr_dn18 = assign41480_e54441_d_n18;
        locals.var_nscr_dn19 = assign41480_e54441_d_n19;
        locals.var_nscr_dn20 = assign41480_e54441_d_n20;
        locals.var_nscr_rv = 0.0;

        let (assign41490_e54453, assign41490_e54453_d_n5, assign41490_e54453_d_n6, assign41490_e54453_d_n7, assign41490_e54453_d_n8, assign41490_e54453_d_n12, assign41490_e54453_d_n13, assign41490_e54453_d_n14, assign41490_e54453_d_n15, assign41490_e54453_d_n16, assign41490_e54453_d_n17, assign41490_e54453_d_n18, assign41490_e54453_d_n19, assign41490_e54453_d_n20,) = {
    if (locals.var_guard1277 == 0.0) {
        let assign41490_e54447: f64 = (0.5 * locals.var_gf);
        let assign41490_e54449: f64 = (locals.var_xn_s).sqrt();
        let assign41490_e54450: f64 = (assign41490_e54447 / assign41490_e54449);
        let assign41490_e54451: f64 = (1.0 + assign41490_e54450);
        (assign41490_e54451, ((((0.5 * locals.var_gf_dn5) * assign41490_e54449) - (assign41490_e54447 * (locals.var_xn_s_dn5 / (2.0 * assign41490_e54449)))) / (assign41490_e54449 * assign41490_e54449)), ((((0.5 * locals.var_gf_dn6) * assign41490_e54449) - (assign41490_e54447 * (locals.var_xn_s_dn6 / (2.0 * assign41490_e54449)))) / (assign41490_e54449 * assign41490_e54449)), ((((0.5 * locals.var_gf_dn7) * assign41490_e54449) - (assign41490_e54447 * (locals.var_xn_s_dn7 / (2.0 * assign41490_e54449)))) / (assign41490_e54449 * assign41490_e54449)), ((((0.5 * locals.var_gf_dn8) * assign41490_e54449) - (assign41490_e54447 * (locals.var_xn_s_dn8 / (2.0 * assign41490_e54449)))) / (assign41490_e54449 * assign41490_e54449)), ((((0.5 * locals.var_gf_dn12) * assign41490_e54449) - (assign41490_e54447 * (locals.var_xn_s_dn12 / (2.0 * assign41490_e54449)))) / (assign41490_e54449 * assign41490_e54449)), ((((0.5 * locals.var_gf_dn13) * assign41490_e54449) - (assign41490_e54447 * (locals.var_xn_s_dn13 / (2.0 * assign41490_e54449)))) / (assign41490_e54449 * assign41490_e54449)), ((((0.5 * locals.var_gf_dn14) * assign41490_e54449) - (assign41490_e54447 * (locals.var_xn_s_dn14 / (2.0 * assign41490_e54449)))) / (assign41490_e54449 * assign41490_e54449)), ((((0.5 * locals.var_gf_dn15) * assign41490_e54449) - (assign41490_e54447 * (locals.var_xn_s_dn15 / (2.0 * assign41490_e54449)))) / (assign41490_e54449 * assign41490_e54449)), ((((0.5 * locals.var_gf_dn16) * assign41490_e54449) - (assign41490_e54447 * (locals.var_xn_s_dn16 / (2.0 * assign41490_e54449)))) / (assign41490_e54449 * assign41490_e54449)), ((((0.5 * locals.var_gf_dn17) * assign41490_e54449) - (assign41490_e54447 * (locals.var_xn_s_dn17 / (2.0 * assign41490_e54449)))) / (assign41490_e54449 * assign41490_e54449)), ((((0.5 * locals.var_gf_dn18) * assign41490_e54449) - (assign41490_e54447 * (locals.var_xn_s_dn18 / (2.0 * assign41490_e54449)))) / (assign41490_e54449 * assign41490_e54449)), ((((0.5 * locals.var_gf_dn19) * assign41490_e54449) - (assign41490_e54447 * (locals.var_xn_s_dn19 / (2.0 * assign41490_e54449)))) / (assign41490_e54449 * assign41490_e54449)), ((((0.5 * locals.var_gf_dn20) * assign41490_e54449) - (assign41490_e54447 * (locals.var_xn_s_dn20 / (2.0 * assign41490_e54449)))) / (assign41490_e54449 * assign41490_e54449)),)
    } else {
        (locals.var_nscr, locals.var_nscr_dn5, locals.var_nscr_dn6, locals.var_nscr_dn7, locals.var_nscr_dn8, locals.var_nscr_dn12, locals.var_nscr_dn13, locals.var_nscr_dn14, locals.var_nscr_dn15, locals.var_nscr_dn16, locals.var_nscr_dn17, locals.var_nscr_dn18, locals.var_nscr_dn19, locals.var_nscr_dn20,)
    }
};
        locals.var_nscr = assign41490_e54453;
        locals.var_nscr_dn5 = assign41490_e54453_d_n5;
        locals.var_nscr_dn6 = assign41490_e54453_d_n6;
        locals.var_nscr_dn7 = assign41490_e54453_d_n7;
        locals.var_nscr_dn8 = assign41490_e54453_d_n8;
        locals.var_nscr_dn12 = assign41490_e54453_d_n12;
        locals.var_nscr_dn13 = assign41490_e54453_d_n13;
        locals.var_nscr_dn14 = assign41490_e54453_d_n14;
        locals.var_nscr_dn15 = assign41490_e54453_d_n15;
        locals.var_nscr_dn16 = assign41490_e54453_d_n16;
        locals.var_nscr_dn17 = assign41490_e54453_d_n17;
        locals.var_nscr_dn18 = assign41490_e54453_d_n18;
        locals.var_nscr_dn19 = assign41490_e54453_d_n19;
        locals.var_nscr_dn20 = assign41490_e54453_d_n20;
        locals.var_nscr_rv = 0.0;

        let assign41500_e54457: f64 = (locals.var_xn_s).sqrt();
        let assign41500_e54458: f64 = (locals.var_gf * assign41500_e54457);
        let assign41500_e54459: f64 = (locals.var_xn_s + assign41500_e54458);
        let assign41500_e54463: f64 = (locals.var_nscr - 1.0);
        let assign41500_e54464: f64 = (assign41500_e54463).ln();
        let assign41500_e54465: f64 = (locals.var_nscr * assign41500_e54464);
        let assign41500_e54466: f64 = (assign41500_e54459 - assign41500_e54465);
        locals.var_xthscr = assign41500_e54466;
        locals.var_xthscr_dn5 = ((locals.var_xn_s_dn5 + ((locals.var_gf_dn5 * assign41500_e54457) + (locals.var_gf * (locals.var_xn_s_dn5 / (2.0 * assign41500_e54457))))) - ((locals.var_nscr_dn5 * assign41500_e54464) + (locals.var_nscr * (locals.var_nscr_dn5 / assign41500_e54463))));
        locals.var_xthscr_dn6 = ((locals.var_xn_s_dn6 + ((locals.var_gf_dn6 * assign41500_e54457) + (locals.var_gf * (locals.var_xn_s_dn6 / (2.0 * assign41500_e54457))))) - ((locals.var_nscr_dn6 * assign41500_e54464) + (locals.var_nscr * (locals.var_nscr_dn6 / assign41500_e54463))));
        locals.var_xthscr_dn7 = ((locals.var_xn_s_dn7 + ((locals.var_gf_dn7 * assign41500_e54457) + (locals.var_gf * (locals.var_xn_s_dn7 / (2.0 * assign41500_e54457))))) - ((locals.var_nscr_dn7 * assign41500_e54464) + (locals.var_nscr * (locals.var_nscr_dn7 / assign41500_e54463))));
        locals.var_xthscr_dn8 = ((locals.var_xn_s_dn8 + ((locals.var_gf_dn8 * assign41500_e54457) + (locals.var_gf * (locals.var_xn_s_dn8 / (2.0 * assign41500_e54457))))) - ((locals.var_nscr_dn8 * assign41500_e54464) + (locals.var_nscr * (locals.var_nscr_dn8 / assign41500_e54463))));
        locals.var_xthscr_dn12 = ((locals.var_xn_s_dn12 + ((locals.var_gf_dn12 * assign41500_e54457) + (locals.var_gf * (locals.var_xn_s_dn12 / (2.0 * assign41500_e54457))))) - ((locals.var_nscr_dn12 * assign41500_e54464) + (locals.var_nscr * (locals.var_nscr_dn12 / assign41500_e54463))));
        locals.var_xthscr_dn13 = ((locals.var_xn_s_dn13 + ((locals.var_gf_dn13 * assign41500_e54457) + (locals.var_gf * (locals.var_xn_s_dn13 / (2.0 * assign41500_e54457))))) - ((locals.var_nscr_dn13 * assign41500_e54464) + (locals.var_nscr * (locals.var_nscr_dn13 / assign41500_e54463))));
        locals.var_xthscr_dn14 = ((locals.var_xn_s_dn14 + ((locals.var_gf_dn14 * assign41500_e54457) + (locals.var_gf * (locals.var_xn_s_dn14 / (2.0 * assign41500_e54457))))) - ((locals.var_nscr_dn14 * assign41500_e54464) + (locals.var_nscr * (locals.var_nscr_dn14 / assign41500_e54463))));
        locals.var_xthscr_dn15 = ((locals.var_xn_s_dn15 + ((locals.var_gf_dn15 * assign41500_e54457) + (locals.var_gf * (locals.var_xn_s_dn15 / (2.0 * assign41500_e54457))))) - ((locals.var_nscr_dn15 * assign41500_e54464) + (locals.var_nscr * (locals.var_nscr_dn15 / assign41500_e54463))));
        locals.var_xthscr_dn16 = ((locals.var_xn_s_dn16 + ((locals.var_gf_dn16 * assign41500_e54457) + (locals.var_gf * (locals.var_xn_s_dn16 / (2.0 * assign41500_e54457))))) - ((locals.var_nscr_dn16 * assign41500_e54464) + (locals.var_nscr * (locals.var_nscr_dn16 / assign41500_e54463))));
        locals.var_xthscr_dn17 = ((locals.var_xn_s_dn17 + ((locals.var_gf_dn17 * assign41500_e54457) + (locals.var_gf * (locals.var_xn_s_dn17 / (2.0 * assign41500_e54457))))) - ((locals.var_nscr_dn17 * assign41500_e54464) + (locals.var_nscr * (locals.var_nscr_dn17 / assign41500_e54463))));
        locals.var_xthscr_dn18 = ((locals.var_xn_s_dn18 + ((locals.var_gf_dn18 * assign41500_e54457) + (locals.var_gf * (locals.var_xn_s_dn18 / (2.0 * assign41500_e54457))))) - ((locals.var_nscr_dn18 * assign41500_e54464) + (locals.var_nscr * (locals.var_nscr_dn18 / assign41500_e54463))));
        locals.var_xthscr_dn19 = ((locals.var_xn_s_dn19 + ((locals.var_gf_dn19 * assign41500_e54457) + (locals.var_gf * (locals.var_xn_s_dn19 / (2.0 * assign41500_e54457))))) - ((locals.var_nscr_dn19 * assign41500_e54464) + (locals.var_nscr * (locals.var_nscr_dn19 / assign41500_e54463))));
        locals.var_xthscr_dn20 = ((locals.var_xn_s_dn20 + ((locals.var_gf_dn20 * assign41500_e54457) + (locals.var_gf * (locals.var_xn_s_dn20 / (2.0 * assign41500_e54457))))) - ((locals.var_nscr_dn20 * assign41500_e54464) + (locals.var_nscr * (locals.var_nscr_dn20 / assign41500_e54463))));
        locals.var_xthscr_rv = 0.0;

        let assign41510_e54469: f64 = (locals.var_xg - locals.var_xthscr);
        let assign41510_e54471: f64 = (assign41510_e54469 / locals.var_nscr);
        locals.var_xgtscr = assign41510_e54471;
        locals.var_xgtscr_dn5 = ((((locals.var_xg_dn5 - locals.var_xthscr_dn5) * locals.var_nscr) - (assign41510_e54469 * locals.var_nscr_dn5)) / (locals.var_nscr * locals.var_nscr));
        locals.var_xgtscr_dn6 = ((((locals.var_xg_dn6 - locals.var_xthscr_dn6) * locals.var_nscr) - (assign41510_e54469 * locals.var_nscr_dn6)) / (locals.var_nscr * locals.var_nscr));
        locals.var_xgtscr_dn7 = ((((locals.var_xg_dn7 - locals.var_xthscr_dn7) * locals.var_nscr) - (assign41510_e54469 * locals.var_nscr_dn7)) / (locals.var_nscr * locals.var_nscr));
        locals.var_xgtscr_dn8 = ((((locals.var_xg_dn8 - locals.var_xthscr_dn8) * locals.var_nscr) - (assign41510_e54469 * locals.var_nscr_dn8)) / (locals.var_nscr * locals.var_nscr));
        locals.var_xgtscr_dn12 = ((((locals.var_xg_dn12 - locals.var_xthscr_dn12) * locals.var_nscr) - (assign41510_e54469 * locals.var_nscr_dn12)) / (locals.var_nscr * locals.var_nscr));
        locals.var_xgtscr_dn13 = ((((locals.var_xg_dn13 - locals.var_xthscr_dn13) * locals.var_nscr) - (assign41510_e54469 * locals.var_nscr_dn13)) / (locals.var_nscr * locals.var_nscr));
        locals.var_xgtscr_dn14 = ((((locals.var_xg_dn14 - locals.var_xthscr_dn14) * locals.var_nscr) - (assign41510_e54469 * locals.var_nscr_dn14)) / (locals.var_nscr * locals.var_nscr));
        locals.var_xgtscr_dn15 = ((((locals.var_xg_dn15 - locals.var_xthscr_dn15) * locals.var_nscr) - (assign41510_e54469 * locals.var_nscr_dn15)) / (locals.var_nscr * locals.var_nscr));
        locals.var_xgtscr_dn16 = ((((locals.var_xg_dn16 - locals.var_xthscr_dn16) * locals.var_nscr) - (assign41510_e54469 * locals.var_nscr_dn16)) / (locals.var_nscr * locals.var_nscr));
        locals.var_xgtscr_dn17 = ((((locals.var_xg_dn17 - locals.var_xthscr_dn17) * locals.var_nscr) - (assign41510_e54469 * locals.var_nscr_dn17)) / (locals.var_nscr * locals.var_nscr));
        locals.var_xgtscr_dn18 = ((((locals.var_xg_dn18 - locals.var_xthscr_dn18) * locals.var_nscr) - (assign41510_e54469 * locals.var_nscr_dn18)) / (locals.var_nscr * locals.var_nscr));
        locals.var_xgtscr_dn19 = ((((locals.var_xg_dn19 - locals.var_xthscr_dn19) * locals.var_nscr) - (assign41510_e54469 * locals.var_nscr_dn19)) / (locals.var_nscr * locals.var_nscr));
        locals.var_xgtscr_dn20 = ((((locals.var_xg_dn20 - locals.var_xthscr_dn20) * locals.var_nscr) - (assign41510_e54469 * locals.var_nscr_dn20)) / (locals.var_nscr * locals.var_nscr));
        locals.var_xgtscr_rv = 0.0;

        let assign41520_e54474: f64 = (0.5 * locals.var_gf2);
        let assign41520_e54478: f64 = (8.0 / locals.var_gf2);
        let assign41520_e54479: f64 = (1.0 + assign41520_e54478);
        let assign41520_e54480: f64 = (assign41520_e54479).sqrt();
        let assign41520_e54482: f64 = (assign41520_e54480 - 1.0);
        let assign41520_e54483: f64 = (assign41520_e54474 * assign41520_e54482);
        locals.var_qbscr = assign41520_e54483;
        locals.var_qbscr_dn5 = (((0.5 * locals.var_gf2_dn5) * assign41520_e54482) + (assign41520_e54474 * ((-((8.0 * locals.var_gf2_dn5) / (locals.var_gf2 * locals.var_gf2))) / (2.0 * assign41520_e54480))));
        locals.var_qbscr_dn6 = (((0.5 * locals.var_gf2_dn6) * assign41520_e54482) + (assign41520_e54474 * ((-((8.0 * locals.var_gf2_dn6) / (locals.var_gf2 * locals.var_gf2))) / (2.0 * assign41520_e54480))));
        locals.var_qbscr_dn7 = (((0.5 * locals.var_gf2_dn7) * assign41520_e54482) + (assign41520_e54474 * ((-((8.0 * locals.var_gf2_dn7) / (locals.var_gf2 * locals.var_gf2))) / (2.0 * assign41520_e54480))));
        locals.var_qbscr_dn8 = (((0.5 * locals.var_gf2_dn8) * assign41520_e54482) + (assign41520_e54474 * ((-((8.0 * locals.var_gf2_dn8) / (locals.var_gf2 * locals.var_gf2))) / (2.0 * assign41520_e54480))));
        locals.var_qbscr_dn12 = (((0.5 * locals.var_gf2_dn12) * assign41520_e54482) + (assign41520_e54474 * ((-((8.0 * locals.var_gf2_dn12) / (locals.var_gf2 * locals.var_gf2))) / (2.0 * assign41520_e54480))));
        locals.var_qbscr_dn13 = (((0.5 * locals.var_gf2_dn13) * assign41520_e54482) + (assign41520_e54474 * ((-((8.0 * locals.var_gf2_dn13) / (locals.var_gf2 * locals.var_gf2))) / (2.0 * assign41520_e54480))));
        locals.var_qbscr_dn14 = (((0.5 * locals.var_gf2_dn14) * assign41520_e54482) + (assign41520_e54474 * ((-((8.0 * locals.var_gf2_dn14) / (locals.var_gf2 * locals.var_gf2))) / (2.0 * assign41520_e54480))));
        locals.var_qbscr_dn15 = (((0.5 * locals.var_gf2_dn15) * assign41520_e54482) + (assign41520_e54474 * ((-((8.0 * locals.var_gf2_dn15) / (locals.var_gf2 * locals.var_gf2))) / (2.0 * assign41520_e54480))));
        locals.var_qbscr_dn16 = (((0.5 * locals.var_gf2_dn16) * assign41520_e54482) + (assign41520_e54474 * ((-((8.0 * locals.var_gf2_dn16) / (locals.var_gf2 * locals.var_gf2))) / (2.0 * assign41520_e54480))));
        locals.var_qbscr_dn17 = (((0.5 * locals.var_gf2_dn17) * assign41520_e54482) + (assign41520_e54474 * ((-((8.0 * locals.var_gf2_dn17) / (locals.var_gf2 * locals.var_gf2))) / (2.0 * assign41520_e54480))));
        locals.var_qbscr_dn18 = (((0.5 * locals.var_gf2_dn18) * assign41520_e54482) + (assign41520_e54474 * ((-((8.0 * locals.var_gf2_dn18) / (locals.var_gf2 * locals.var_gf2))) / (2.0 * assign41520_e54480))));
        locals.var_qbscr_dn19 = (((0.5 * locals.var_gf2_dn19) * assign41520_e54482) + (assign41520_e54474 * ((-((8.0 * locals.var_gf2_dn19) / (locals.var_gf2 * locals.var_gf2))) / (2.0 * assign41520_e54480))));
        locals.var_qbscr_dn20 = (((0.5 * locals.var_gf2_dn20) * assign41520_e54482) + (assign41520_e54474 * ((-((8.0 * locals.var_gf2_dn20) / (locals.var_gf2 * locals.var_gf2))) / (2.0 * assign41520_e54480))));
        locals.var_qbscr_rv = 0.0;

        locals.var_qiscr = 0.0;
        locals.var_qiscr_dn5 = 0.0;
        locals.var_qiscr_dn6 = 0.0;
        locals.var_qiscr_dn7 = 0.0;
        locals.var_qiscr_dn8 = 0.0;
        locals.var_qiscr_dn12 = 0.0;
        locals.var_qiscr_dn13 = 0.0;
        locals.var_qiscr_dn14 = 0.0;
        locals.var_qiscr_dn15 = 0.0;
        locals.var_qiscr_dn16 = 0.0;
        locals.var_qiscr_dn17 = 0.0;
        locals.var_qiscr_dn18 = 0.0;
        locals.var_qiscr_dn19 = 0.0;
        locals.var_qiscr_dn20 = 0.0;
        locals.var_qiscr_rv = 0.0;

        locals.var_fscr = 1.0;
        locals.var_fscr_dn5 = 0.0;
        locals.var_fscr_dn6 = 0.0;
        locals.var_fscr_dn7 = 0.0;
        locals.var_fscr_dn8 = 0.0;
        locals.var_fscr_dn12 = 0.0;
        locals.var_fscr_dn13 = 0.0;
        locals.var_fscr_dn14 = 0.0;
        locals.var_fscr_dn15 = 0.0;
        locals.var_fscr_dn16 = 0.0;
        locals.var_fscr_dn17 = 0.0;
        locals.var_fscr_dn18 = 0.0;
        locals.var_fscr_dn19 = 0.0;
        locals.var_fscr_dn20 = 0.0;
        locals.var_fscr_rv = 0.0;

        let assign41550_e54488: f64 = (-30.0);
        let assign41550_e54489: f64 = if locals.var_xgtscr > assign41550_e54488 { 1.0 } else { 0.0 };
        locals.var_guard1280 = assign41550_e54489;
        locals.var_guard1280_rv = 0.0;

        let (assign41560_e54497, assign41560_e54497_d_n5, assign41560_e54497_d_n6, assign41560_e54497_d_n7, assign41560_e54497_d_n8, assign41560_e54497_d_n12, assign41560_e54497_d_n13, assign41560_e54497_d_n14, assign41560_e54497_d_n15, assign41560_e54497_d_n16, assign41560_e54497_d_n17, assign41560_e54497_d_n18, assign41560_e54497_d_n19, assign41560_e54497_d_n20,) = {
    if (locals.var_guard1280 != 0.0) {
        let assign41560_e54493: f64 = (locals.var_nscr * locals.var_xgtscr);
        let assign41560_e54495: f64 = (assign41560_e54493 - 1.0);
        (assign41560_e54495, ((locals.var_nscr_dn5 * locals.var_xgtscr) + (locals.var_nscr * locals.var_xgtscr_dn5)), ((locals.var_nscr_dn6 * locals.var_xgtscr) + (locals.var_nscr * locals.var_xgtscr_dn6)), ((locals.var_nscr_dn7 * locals.var_xgtscr) + (locals.var_nscr * locals.var_xgtscr_dn7)), ((locals.var_nscr_dn8 * locals.var_xgtscr) + (locals.var_nscr * locals.var_xgtscr_dn8)), ((locals.var_nscr_dn12 * locals.var_xgtscr) + (locals.var_nscr * locals.var_xgtscr_dn12)), ((locals.var_nscr_dn13 * locals.var_xgtscr) + (locals.var_nscr * locals.var_xgtscr_dn13)), ((locals.var_nscr_dn14 * locals.var_xgtscr) + (locals.var_nscr * locals.var_xgtscr_dn14)), ((locals.var_nscr_dn15 * locals.var_xgtscr) + (locals.var_nscr * locals.var_xgtscr_dn15)), ((locals.var_nscr_dn16 * locals.var_xgtscr) + (locals.var_nscr * locals.var_xgtscr_dn16)), ((locals.var_nscr_dn17 * locals.var_xgtscr) + (locals.var_nscr * locals.var_xgtscr_dn17)), ((locals.var_nscr_dn18 * locals.var_xgtscr) + (locals.var_nscr * locals.var_xgtscr_dn18)), ((locals.var_nscr_dn19 * locals.var_xgtscr) + (locals.var_nscr * locals.var_xgtscr_dn19)), ((locals.var_nscr_dn20 * locals.var_xgtscr) + (locals.var_nscr * locals.var_xgtscr_dn20)),)
    } else {
        (locals.var_xgtscr0, locals.var_xgtscr0_dn5, locals.var_xgtscr0_dn6, locals.var_xgtscr0_dn7, locals.var_xgtscr0_dn8, locals.var_xgtscr0_dn12, locals.var_xgtscr0_dn13, locals.var_xgtscr0_dn14, locals.var_xgtscr0_dn15, locals.var_xgtscr0_dn16, locals.var_xgtscr0_dn17, locals.var_xgtscr0_dn18, locals.var_xgtscr0_dn19, locals.var_xgtscr0_dn20,)
    }
};
        locals.var_xgtscr0 = assign41560_e54497;
        locals.var_xgtscr0_dn5 = assign41560_e54497_d_n5;
        locals.var_xgtscr0_dn6 = assign41560_e54497_d_n6;
        locals.var_xgtscr0_dn7 = assign41560_e54497_d_n7;
        locals.var_xgtscr0_dn8 = assign41560_e54497_d_n8;
        locals.var_xgtscr0_dn12 = assign41560_e54497_d_n12;
        locals.var_xgtscr0_dn13 = assign41560_e54497_d_n13;
        locals.var_xgtscr0_dn14 = assign41560_e54497_d_n14;
        locals.var_xgtscr0_dn15 = assign41560_e54497_d_n15;
        locals.var_xgtscr0_dn16 = assign41560_e54497_d_n16;
        locals.var_xgtscr0_dn17 = assign41560_e54497_d_n17;
        locals.var_xgtscr0_dn18 = assign41560_e54497_d_n18;
        locals.var_xgtscr0_dn19 = assign41560_e54497_d_n19;
        locals.var_xgtscr0_dn20 = assign41560_e54497_d_n20;
        locals.var_xgtscr0_rv = 0.0;

        let (assign41570_e54510, assign41570_e54510_d_n5, assign41570_e54510_d_n6, assign41570_e54510_d_n7, assign41570_e54510_d_n8, assign41570_e54510_d_n12, assign41570_e54510_d_n13, assign41570_e54510_d_n14, assign41570_e54510_d_n15, assign41570_e54510_d_n16, assign41570_e54510_d_n17, assign41570_e54510_d_n18, assign41570_e54510_d_n19, assign41570_e54510_d_n20,) = {
    if (locals.var_guard1280 != 0.0) {
        let assign41570_e54503: f64 = (locals.var_xgtscr0 * locals.var_xgtscr0);
        let assign41570_e54505: f64 = (assign41570_e54503 + 10.0);
        let assign41570_e54506: f64 = (assign41570_e54505).sqrt();
        let assign41570_e54507: f64 = (locals.var_xgtscr0 + assign41570_e54506);
        let assign41570_e54508: f64 = (0.5 * assign41570_e54507);
        (assign41570_e54508, (0.5 * (locals.var_xgtscr0_dn5 + (((locals.var_xgtscr0_dn5 * locals.var_xgtscr0) + (locals.var_xgtscr0 * locals.var_xgtscr0_dn5)) / (2.0 * assign41570_e54506)))), (0.5 * (locals.var_xgtscr0_dn6 + (((locals.var_xgtscr0_dn6 * locals.var_xgtscr0) + (locals.var_xgtscr0 * locals.var_xgtscr0_dn6)) / (2.0 * assign41570_e54506)))), (0.5 * (locals.var_xgtscr0_dn7 + (((locals.var_xgtscr0_dn7 * locals.var_xgtscr0) + (locals.var_xgtscr0 * locals.var_xgtscr0_dn7)) / (2.0 * assign41570_e54506)))), (0.5 * (locals.var_xgtscr0_dn8 + (((locals.var_xgtscr0_dn8 * locals.var_xgtscr0) + (locals.var_xgtscr0 * locals.var_xgtscr0_dn8)) / (2.0 * assign41570_e54506)))), (0.5 * (locals.var_xgtscr0_dn12 + (((locals.var_xgtscr0_dn12 * locals.var_xgtscr0) + (locals.var_xgtscr0 * locals.var_xgtscr0_dn12)) / (2.0 * assign41570_e54506)))), (0.5 * (locals.var_xgtscr0_dn13 + (((locals.var_xgtscr0_dn13 * locals.var_xgtscr0) + (locals.var_xgtscr0 * locals.var_xgtscr0_dn13)) / (2.0 * assign41570_e54506)))), (0.5 * (locals.var_xgtscr0_dn14 + (((locals.var_xgtscr0_dn14 * locals.var_xgtscr0) + (locals.var_xgtscr0 * locals.var_xgtscr0_dn14)) / (2.0 * assign41570_e54506)))), (0.5 * (locals.var_xgtscr0_dn15 + (((locals.var_xgtscr0_dn15 * locals.var_xgtscr0) + (locals.var_xgtscr0 * locals.var_xgtscr0_dn15)) / (2.0 * assign41570_e54506)))), (0.5 * (locals.var_xgtscr0_dn16 + (((locals.var_xgtscr0_dn16 * locals.var_xgtscr0) + (locals.var_xgtscr0 * locals.var_xgtscr0_dn16)) / (2.0 * assign41570_e54506)))), (0.5 * (locals.var_xgtscr0_dn17 + (((locals.var_xgtscr0_dn17 * locals.var_xgtscr0) + (locals.var_xgtscr0 * locals.var_xgtscr0_dn17)) / (2.0 * assign41570_e54506)))), (0.5 * (locals.var_xgtscr0_dn18 + (((locals.var_xgtscr0_dn18 * locals.var_xgtscr0) + (locals.var_xgtscr0 * locals.var_xgtscr0_dn18)) / (2.0 * assign41570_e54506)))), (0.5 * (locals.var_xgtscr0_dn19 + (((locals.var_xgtscr0_dn19 * locals.var_xgtscr0) + (locals.var_xgtscr0 * locals.var_xgtscr0_dn19)) / (2.0 * assign41570_e54506)))), (0.5 * (locals.var_xgtscr0_dn20 + (((locals.var_xgtscr0_dn20 * locals.var_xgtscr0) + (locals.var_xgtscr0 * locals.var_xgtscr0_dn20)) / (2.0 * assign41570_e54506)))),)
    } else {
        (locals.var_temp__blk1038, locals.var_temp__blk1038_dn5, locals.var_temp__blk1038_dn6, locals.var_temp__blk1038_dn7, locals.var_temp__blk1038_dn8, locals.var_temp__blk1038_dn12, locals.var_temp__blk1038_dn13, locals.var_temp__blk1038_dn14, locals.var_temp__blk1038_dn15, locals.var_temp__blk1038_dn16, locals.var_temp__blk1038_dn17, locals.var_temp__blk1038_dn18, locals.var_temp__blk1038_dn19, locals.var_temp__blk1038_dn20,)
    }
};
        locals.var_temp__blk1038 = assign41570_e54510;
        locals.var_temp__blk1038_dn5 = assign41570_e54510_d_n5;
        locals.var_temp__blk1038_dn6 = assign41570_e54510_d_n6;
        locals.var_temp__blk1038_dn7 = assign41570_e54510_d_n7;
        locals.var_temp__blk1038_dn8 = assign41570_e54510_d_n8;
        locals.var_temp__blk1038_dn12 = assign41570_e54510_d_n12;
        locals.var_temp__blk1038_dn13 = assign41570_e54510_d_n13;
        locals.var_temp__blk1038_dn14 = assign41570_e54510_d_n14;
        locals.var_temp__blk1038_dn15 = assign41570_e54510_d_n15;
        locals.var_temp__blk1038_dn16 = assign41570_e54510_d_n16;
        locals.var_temp__blk1038_dn17 = assign41570_e54510_d_n17;
        locals.var_temp__blk1038_dn18 = assign41570_e54510_d_n18;
        locals.var_temp__blk1038_dn19 = assign41570_e54510_d_n19;
        locals.var_temp__blk1038_dn20 = assign41570_e54510_d_n20;
        locals.var_temp__blk1038_rv = 0.0;

        let (assign41580_e54517, assign41580_e54517_d_n5, assign41580_e54517_d_n6, assign41580_e54517_d_n7, assign41580_e54517_d_n8, assign41580_e54517_d_n12, assign41580_e54517_d_n13, assign41580_e54517_d_n14, assign41580_e54517_d_n15, assign41580_e54517_d_n16, assign41580_e54517_d_n17, assign41580_e54517_d_n18, assign41580_e54517_d_n19, assign41580_e54517_d_n20,) = {
    if (locals.var_guard1280 != 0.0) {
        let assign41580_e54514: f64 = (locals.var_temp__blk1038).ln();
        let assign41580_e54515: f64 = (locals.var_xgtscr - assign41580_e54514);
        (assign41580_e54515, (locals.var_xgtscr_dn5 - (locals.var_temp__blk1038_dn5 / locals.var_temp__blk1038)), (locals.var_xgtscr_dn6 - (locals.var_temp__blk1038_dn6 / locals.var_temp__blk1038)), (locals.var_xgtscr_dn7 - (locals.var_temp__blk1038_dn7 / locals.var_temp__blk1038)), (locals.var_xgtscr_dn8 - (locals.var_temp__blk1038_dn8 / locals.var_temp__blk1038)), (locals.var_xgtscr_dn12 - (locals.var_temp__blk1038_dn12 / locals.var_temp__blk1038)), (locals.var_xgtscr_dn13 - (locals.var_temp__blk1038_dn13 / locals.var_temp__blk1038)), (locals.var_xgtscr_dn14 - (locals.var_temp__blk1038_dn14 / locals.var_temp__blk1038)), (locals.var_xgtscr_dn15 - (locals.var_temp__blk1038_dn15 / locals.var_temp__blk1038)), (locals.var_xgtscr_dn16 - (locals.var_temp__blk1038_dn16 / locals.var_temp__blk1038)), (locals.var_xgtscr_dn17 - (locals.var_temp__blk1038_dn17 / locals.var_temp__blk1038)), (locals.var_xgtscr_dn18 - (locals.var_temp__blk1038_dn18 / locals.var_temp__blk1038)), (locals.var_xgtscr_dn19 - (locals.var_temp__blk1038_dn19 / locals.var_temp__blk1038)), (locals.var_xgtscr_dn20 - (locals.var_temp__blk1038_dn20 / locals.var_temp__blk1038)),)
    } else {
        (locals.var_qiscr0si, locals.var_qiscr0si_dn5, locals.var_qiscr0si_dn6, locals.var_qiscr0si_dn7, locals.var_qiscr0si_dn8, locals.var_qiscr0si_dn12, locals.var_qiscr0si_dn13, locals.var_qiscr0si_dn14, locals.var_qiscr0si_dn15, locals.var_qiscr0si_dn16, locals.var_qiscr0si_dn17, locals.var_qiscr0si_dn18, locals.var_qiscr0si_dn19, locals.var_qiscr0si_dn20,)
    }
};
        locals.var_qiscr0si = assign41580_e54517;
        locals.var_qiscr0si_dn5 = assign41580_e54517_d_n5;
        locals.var_qiscr0si_dn6 = assign41580_e54517_d_n6;
        locals.var_qiscr0si_dn7 = assign41580_e54517_d_n7;
        locals.var_qiscr0si_dn8 = assign41580_e54517_d_n8;
        locals.var_qiscr0si_dn12 = assign41580_e54517_d_n12;
        locals.var_qiscr0si_dn13 = assign41580_e54517_d_n13;
        locals.var_qiscr0si_dn14 = assign41580_e54517_d_n14;
        locals.var_qiscr0si_dn15 = assign41580_e54517_d_n15;
        locals.var_qiscr0si_dn16 = assign41580_e54517_d_n16;
        locals.var_qiscr0si_dn17 = assign41580_e54517_d_n17;
        locals.var_qiscr0si_dn18 = assign41580_e54517_d_n18;
        locals.var_qiscr0si_dn19 = assign41580_e54517_d_n19;
        locals.var_qiscr0si_dn20 = assign41580_e54517_d_n20;
        locals.var_qiscr0si_rv = 0.0;

        let (assign41590_e54530, assign41590_e54530_d_n5, assign41590_e54530_d_n6, assign41590_e54530_d_n7, assign41590_e54530_d_n8, assign41590_e54530_d_n12, assign41590_e54530_d_n13, assign41590_e54530_d_n14, assign41590_e54530_d_n15, assign41590_e54530_d_n16, assign41590_e54530_d_n17, assign41590_e54530_d_n18, assign41590_e54530_d_n19, assign41590_e54530_d_n20,) = {
    if (locals.var_guard1280 != 0.0) {
        let assign41590_e54523: f64 = (locals.var_qiscr0si * locals.var_qiscr0si);
        let assign41590_e54525: f64 = (assign41590_e54523 + 2.0);
        let assign41590_e54526: f64 = (assign41590_e54525).sqrt();
        let assign41590_e54527: f64 = (locals.var_qiscr0si + assign41590_e54526);
        let assign41590_e54528: f64 = (0.5 * assign41590_e54527);
        (assign41590_e54528, (0.5 * (locals.var_qiscr0si_dn5 + (((locals.var_qiscr0si_dn5 * locals.var_qiscr0si) + (locals.var_qiscr0si * locals.var_qiscr0si_dn5)) / (2.0 * assign41590_e54526)))), (0.5 * (locals.var_qiscr0si_dn6 + (((locals.var_qiscr0si_dn6 * locals.var_qiscr0si) + (locals.var_qiscr0si * locals.var_qiscr0si_dn6)) / (2.0 * assign41590_e54526)))), (0.5 * (locals.var_qiscr0si_dn7 + (((locals.var_qiscr0si_dn7 * locals.var_qiscr0si) + (locals.var_qiscr0si * locals.var_qiscr0si_dn7)) / (2.0 * assign41590_e54526)))), (0.5 * (locals.var_qiscr0si_dn8 + (((locals.var_qiscr0si_dn8 * locals.var_qiscr0si) + (locals.var_qiscr0si * locals.var_qiscr0si_dn8)) / (2.0 * assign41590_e54526)))), (0.5 * (locals.var_qiscr0si_dn12 + (((locals.var_qiscr0si_dn12 * locals.var_qiscr0si) + (locals.var_qiscr0si * locals.var_qiscr0si_dn12)) / (2.0 * assign41590_e54526)))), (0.5 * (locals.var_qiscr0si_dn13 + (((locals.var_qiscr0si_dn13 * locals.var_qiscr0si) + (locals.var_qiscr0si * locals.var_qiscr0si_dn13)) / (2.0 * assign41590_e54526)))), (0.5 * (locals.var_qiscr0si_dn14 + (((locals.var_qiscr0si_dn14 * locals.var_qiscr0si) + (locals.var_qiscr0si * locals.var_qiscr0si_dn14)) / (2.0 * assign41590_e54526)))), (0.5 * (locals.var_qiscr0si_dn15 + (((locals.var_qiscr0si_dn15 * locals.var_qiscr0si) + (locals.var_qiscr0si * locals.var_qiscr0si_dn15)) / (2.0 * assign41590_e54526)))), (0.5 * (locals.var_qiscr0si_dn16 + (((locals.var_qiscr0si_dn16 * locals.var_qiscr0si) + (locals.var_qiscr0si * locals.var_qiscr0si_dn16)) / (2.0 * assign41590_e54526)))), (0.5 * (locals.var_qiscr0si_dn17 + (((locals.var_qiscr0si_dn17 * locals.var_qiscr0si) + (locals.var_qiscr0si * locals.var_qiscr0si_dn17)) / (2.0 * assign41590_e54526)))), (0.5 * (locals.var_qiscr0si_dn18 + (((locals.var_qiscr0si_dn18 * locals.var_qiscr0si) + (locals.var_qiscr0si * locals.var_qiscr0si_dn18)) / (2.0 * assign41590_e54526)))), (0.5 * (locals.var_qiscr0si_dn19 + (((locals.var_qiscr0si_dn19 * locals.var_qiscr0si) + (locals.var_qiscr0si * locals.var_qiscr0si_dn19)) / (2.0 * assign41590_e54526)))), (0.5 * (locals.var_qiscr0si_dn20 + (((locals.var_qiscr0si_dn20 * locals.var_qiscr0si) + (locals.var_qiscr0si * locals.var_qiscr0si_dn20)) / (2.0 * assign41590_e54526)))),)
    } else {
        (locals.var_qiscr0, locals.var_qiscr0_dn5, locals.var_qiscr0_dn6, locals.var_qiscr0_dn7, locals.var_qiscr0_dn8, locals.var_qiscr0_dn12, locals.var_qiscr0_dn13, locals.var_qiscr0_dn14, locals.var_qiscr0_dn15, locals.var_qiscr0_dn16, locals.var_qiscr0_dn17, locals.var_qiscr0_dn18, locals.var_qiscr0_dn19, locals.var_qiscr0_dn20,)
    }
};
        locals.var_qiscr0 = assign41590_e54530;
        locals.var_qiscr0_dn5 = assign41590_e54530_d_n5;
        locals.var_qiscr0_dn6 = assign41590_e54530_d_n6;
        locals.var_qiscr0_dn7 = assign41590_e54530_d_n7;
        locals.var_qiscr0_dn8 = assign41590_e54530_d_n8;
        locals.var_qiscr0_dn12 = assign41590_e54530_d_n12;
        locals.var_qiscr0_dn13 = assign41590_e54530_d_n13;
        locals.var_qiscr0_dn14 = assign41590_e54530_d_n14;
        locals.var_qiscr0_dn15 = assign41590_e54530_d_n15;
        locals.var_qiscr0_dn16 = assign41590_e54530_d_n16;
        locals.var_qiscr0_dn17 = assign41590_e54530_d_n17;
        locals.var_qiscr0_dn18 = assign41590_e54530_d_n18;
        locals.var_qiscr0_dn19 = assign41590_e54530_d_n19;
        locals.var_qiscr0_dn20 = assign41590_e54530_d_n20;
        locals.var_qiscr0_rv = 0.0;

        let assign41600_e54533: f64 = (locals.var_xgtscr - locals.var_qiscr0);
        let assign41600_e54535: f64 = if assign41600_e54533 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard1281 = assign41600_e54535;
        locals.var_guard1281_rv = 0.0;

        let (assign41610_e54544, assign41610_e54544_d_n5, assign41610_e54544_d_n6, assign41610_e54544_d_n7, assign41610_e54544_d_n8, assign41610_e54544_d_n12, assign41610_e54544_d_n13, assign41610_e54544_d_n14, assign41610_e54544_d_n15, assign41610_e54544_d_n16, assign41610_e54544_d_n17, assign41610_e54544_d_n18, assign41610_e54544_d_n19, assign41610_e54544_d_n20,) = {
    if ((locals.var_guard1280 != 0.0) && (locals.var_guard1281 != 0.0)) {
        let assign41610_e54541: f64 = (locals.var_xgtscr - locals.var_qiscr0);
        let assign41610_e54542: f64 = (assign41610_e54541).exp();
        (assign41610_e54542, (assign41610_e54542 * (locals.var_xgtscr_dn5 - locals.var_qiscr0_dn5)), (assign41610_e54542 * (locals.var_xgtscr_dn6 - locals.var_qiscr0_dn6)), (assign41610_e54542 * (locals.var_xgtscr_dn7 - locals.var_qiscr0_dn7)), (assign41610_e54542 * (locals.var_xgtscr_dn8 - locals.var_qiscr0_dn8)), (assign41610_e54542 * (locals.var_xgtscr_dn12 - locals.var_qiscr0_dn12)), (assign41610_e54542 * (locals.var_xgtscr_dn13 - locals.var_qiscr0_dn13)), (assign41610_e54542 * (locals.var_xgtscr_dn14 - locals.var_qiscr0_dn14)), (assign41610_e54542 * (locals.var_xgtscr_dn15 - locals.var_qiscr0_dn15)), (assign41610_e54542 * (locals.var_xgtscr_dn16 - locals.var_qiscr0_dn16)), (assign41610_e54542 * (locals.var_xgtscr_dn17 - locals.var_qiscr0_dn17)), (assign41610_e54542 * (locals.var_xgtscr_dn18 - locals.var_qiscr0_dn18)), (assign41610_e54542 * (locals.var_xgtscr_dn19 - locals.var_qiscr0_dn19)), (assign41610_e54542 * (locals.var_xgtscr_dn20 - locals.var_qiscr0_dn20)),)
    } else {
        (locals.var_temp__blk1038, locals.var_temp__blk1038_dn5, locals.var_temp__blk1038_dn6, locals.var_temp__blk1038_dn7, locals.var_temp__blk1038_dn8, locals.var_temp__blk1038_dn12, locals.var_temp__blk1038_dn13, locals.var_temp__blk1038_dn14, locals.var_temp__blk1038_dn15, locals.var_temp__blk1038_dn16, locals.var_temp__blk1038_dn17, locals.var_temp__blk1038_dn18, locals.var_temp__blk1038_dn19, locals.var_temp__blk1038_dn20,)
    }
};
        locals.var_temp__blk1038 = assign41610_e54544;
        locals.var_temp__blk1038_dn5 = assign41610_e54544_d_n5;
        locals.var_temp__blk1038_dn6 = assign41610_e54544_d_n6;
        locals.var_temp__blk1038_dn7 = assign41610_e54544_d_n7;
        locals.var_temp__blk1038_dn8 = assign41610_e54544_d_n8;
        locals.var_temp__blk1038_dn12 = assign41610_e54544_d_n12;
        locals.var_temp__blk1038_dn13 = assign41610_e54544_d_n13;
        locals.var_temp__blk1038_dn14 = assign41610_e54544_d_n14;
        locals.var_temp__blk1038_dn15 = assign41610_e54544_d_n15;
        locals.var_temp__blk1038_dn16 = assign41610_e54544_d_n16;
        locals.var_temp__blk1038_dn17 = assign41610_e54544_d_n17;
        locals.var_temp__blk1038_dn18 = assign41610_e54544_d_n18;
        locals.var_temp__blk1038_dn19 = assign41610_e54544_d_n19;
        locals.var_temp__blk1038_dn20 = assign41610_e54544_d_n20;
        locals.var_temp__blk1038_rv = 0.0;

        let (assign41620_e54579, assign41620_e54579_d_n5, assign41620_e54579_d_n6, assign41620_e54579_d_n7, assign41620_e54579_d_n8, assign41620_e54579_d_n12, assign41620_e54579_d_n13, assign41620_e54579_d_n14, assign41620_e54579_d_n15, assign41620_e54579_d_n16, assign41620_e54579_d_n17, assign41620_e54579_d_n18, assign41620_e54579_d_n19, assign41620_e54579_d_n20,) = {
    if ((locals.var_guard1280 != 0.0) && (locals.var_guard1281 == 0.0)) {
        let assign41620_e54553: f64 = (locals.var_xgtscr - locals.var_qiscr0);
        let assign41620_e54555: f64 = (assign41620_e54553 - 230.25850929940458);
        let assign41620_e54560: f64 = (locals.var_xgtscr - locals.var_qiscr0);
        let assign41620_e54562: f64 = (assign41620_e54560 - 230.25850929940458);
        let assign41620_e54566: f64 = (locals.var_xgtscr - locals.var_qiscr0);
        let assign41620_e54568: f64 = (assign41620_e54566 - 230.25850929940458);
        let assign41620_e54570: f64 = (assign41620_e54568 * 0.3333333333333333);
        let assign41620_e54571: f64 = (1.0 + assign41620_e54570);
        let assign41620_e54572: f64 = (assign41620_e54562 * assign41620_e54571);
        let assign41620_e54573: f64 = (0.5 * assign41620_e54572);
        let assign41620_e54574: f64 = (1.0 + assign41620_e54573);
        let assign41620_e54575: f64 = (assign41620_e54555 * assign41620_e54574);
        let assign41620_e54576: f64 = (1.0 + assign41620_e54575);
        let assign41620_e54577: f64 = (1e100 * assign41620_e54576);
        (assign41620_e54577, (1e100 * (((locals.var_xgtscr_dn5 - locals.var_qiscr0_dn5) * assign41620_e54574) + (assign41620_e54555 * (0.5 * (((locals.var_xgtscr_dn5 - locals.var_qiscr0_dn5) * assign41620_e54571) + (assign41620_e54562 * ((locals.var_xgtscr_dn5 - locals.var_qiscr0_dn5) * 0.3333333333333333))))))), (1e100 * (((locals.var_xgtscr_dn6 - locals.var_qiscr0_dn6) * assign41620_e54574) + (assign41620_e54555 * (0.5 * (((locals.var_xgtscr_dn6 - locals.var_qiscr0_dn6) * assign41620_e54571) + (assign41620_e54562 * ((locals.var_xgtscr_dn6 - locals.var_qiscr0_dn6) * 0.3333333333333333))))))), (1e100 * (((locals.var_xgtscr_dn7 - locals.var_qiscr0_dn7) * assign41620_e54574) + (assign41620_e54555 * (0.5 * (((locals.var_xgtscr_dn7 - locals.var_qiscr0_dn7) * assign41620_e54571) + (assign41620_e54562 * ((locals.var_xgtscr_dn7 - locals.var_qiscr0_dn7) * 0.3333333333333333))))))), (1e100 * (((locals.var_xgtscr_dn8 - locals.var_qiscr0_dn8) * assign41620_e54574) + (assign41620_e54555 * (0.5 * (((locals.var_xgtscr_dn8 - locals.var_qiscr0_dn8) * assign41620_e54571) + (assign41620_e54562 * ((locals.var_xgtscr_dn8 - locals.var_qiscr0_dn8) * 0.3333333333333333))))))), (1e100 * (((locals.var_xgtscr_dn12 - locals.var_qiscr0_dn12) * assign41620_e54574) + (assign41620_e54555 * (0.5 * (((locals.var_xgtscr_dn12 - locals.var_qiscr0_dn12) * assign41620_e54571) + (assign41620_e54562 * ((locals.var_xgtscr_dn12 - locals.var_qiscr0_dn12) * 0.3333333333333333))))))), (1e100 * (((locals.var_xgtscr_dn13 - locals.var_qiscr0_dn13) * assign41620_e54574) + (assign41620_e54555 * (0.5 * (((locals.var_xgtscr_dn13 - locals.var_qiscr0_dn13) * assign41620_e54571) + (assign41620_e54562 * ((locals.var_xgtscr_dn13 - locals.var_qiscr0_dn13) * 0.3333333333333333))))))), (1e100 * (((locals.var_xgtscr_dn14 - locals.var_qiscr0_dn14) * assign41620_e54574) + (assign41620_e54555 * (0.5 * (((locals.var_xgtscr_dn14 - locals.var_qiscr0_dn14) * assign41620_e54571) + (assign41620_e54562 * ((locals.var_xgtscr_dn14 - locals.var_qiscr0_dn14) * 0.3333333333333333))))))), (1e100 * (((locals.var_xgtscr_dn15 - locals.var_qiscr0_dn15) * assign41620_e54574) + (assign41620_e54555 * (0.5 * (((locals.var_xgtscr_dn15 - locals.var_qiscr0_dn15) * assign41620_e54571) + (assign41620_e54562 * ((locals.var_xgtscr_dn15 - locals.var_qiscr0_dn15) * 0.3333333333333333))))))), (1e100 * (((locals.var_xgtscr_dn16 - locals.var_qiscr0_dn16) * assign41620_e54574) + (assign41620_e54555 * (0.5 * (((locals.var_xgtscr_dn16 - locals.var_qiscr0_dn16) * assign41620_e54571) + (assign41620_e54562 * ((locals.var_xgtscr_dn16 - locals.var_qiscr0_dn16) * 0.3333333333333333))))))), (1e100 * (((locals.var_xgtscr_dn17 - locals.var_qiscr0_dn17) * assign41620_e54574) + (assign41620_e54555 * (0.5 * (((locals.var_xgtscr_dn17 - locals.var_qiscr0_dn17) * assign41620_e54571) + (assign41620_e54562 * ((locals.var_xgtscr_dn17 - locals.var_qiscr0_dn17) * 0.3333333333333333))))))), (1e100 * (((locals.var_xgtscr_dn18 - locals.var_qiscr0_dn18) * assign41620_e54574) + (assign41620_e54555 * (0.5 * (((locals.var_xgtscr_dn18 - locals.var_qiscr0_dn18) * assign41620_e54571) + (assign41620_e54562 * ((locals.var_xgtscr_dn18 - locals.var_qiscr0_dn18) * 0.3333333333333333))))))), (1e100 * (((locals.var_xgtscr_dn19 - locals.var_qiscr0_dn19) * assign41620_e54574) + (assign41620_e54555 * (0.5 * (((locals.var_xgtscr_dn19 - locals.var_qiscr0_dn19) * assign41620_e54571) + (assign41620_e54562 * ((locals.var_xgtscr_dn19 - locals.var_qiscr0_dn19) * 0.3333333333333333))))))), (1e100 * (((locals.var_xgtscr_dn20 - locals.var_qiscr0_dn20) * assign41620_e54574) + (assign41620_e54555 * (0.5 * (((locals.var_xgtscr_dn20 - locals.var_qiscr0_dn20) * assign41620_e54571) + (assign41620_e54562 * ((locals.var_xgtscr_dn20 - locals.var_qiscr0_dn20) * 0.3333333333333333))))))),)
    } else {
        (locals.var_temp__blk1038, locals.var_temp__blk1038_dn5, locals.var_temp__blk1038_dn6, locals.var_temp__blk1038_dn7, locals.var_temp__blk1038_dn8, locals.var_temp__blk1038_dn12, locals.var_temp__blk1038_dn13, locals.var_temp__blk1038_dn14, locals.var_temp__blk1038_dn15, locals.var_temp__blk1038_dn16, locals.var_temp__blk1038_dn17, locals.var_temp__blk1038_dn18, locals.var_temp__blk1038_dn19, locals.var_temp__blk1038_dn20,)
    }
};
        locals.var_temp__blk1038 = assign41620_e54579;
        locals.var_temp__blk1038_dn5 = assign41620_e54579_d_n5;
        locals.var_temp__blk1038_dn6 = assign41620_e54579_d_n6;
        locals.var_temp__blk1038_dn7 = assign41620_e54579_d_n7;
        locals.var_temp__blk1038_dn8 = assign41620_e54579_d_n8;
        locals.var_temp__blk1038_dn12 = assign41620_e54579_d_n12;
        locals.var_temp__blk1038_dn13 = assign41620_e54579_d_n13;
        locals.var_temp__blk1038_dn14 = assign41620_e54579_d_n14;
        locals.var_temp__blk1038_dn15 = assign41620_e54579_d_n15;
        locals.var_temp__blk1038_dn16 = assign41620_e54579_d_n16;
        locals.var_temp__blk1038_dn17 = assign41620_e54579_d_n17;
        locals.var_temp__blk1038_dn18 = assign41620_e54579_d_n18;
        locals.var_temp__blk1038_dn19 = assign41620_e54579_d_n19;
        locals.var_temp__blk1038_dn20 = assign41620_e54579_d_n20;
        locals.var_temp__blk1038_rv = 0.0;

        let (assign41630_e54585, assign41630_e54585_d_n5, assign41630_e54585_d_n6, assign41630_e54585_d_n7, assign41630_e54585_d_n8, assign41630_e54585_d_n12, assign41630_e54585_d_n13, assign41630_e54585_d_n14, assign41630_e54585_d_n15, assign41630_e54585_d_n16, assign41630_e54585_d_n17, assign41630_e54585_d_n18, assign41630_e54585_d_n19, assign41630_e54585_d_n20,) = {
    if (locals.var_guard1280 != 0.0) {
        let assign41630_e54583: f64 = (locals.var_temp__blk1038 / locals.var_nscr);
        (assign41630_e54583, (((locals.var_temp__blk1038_dn5 * locals.var_nscr) - (locals.var_temp__blk1038 * locals.var_nscr_dn5)) / (locals.var_nscr * locals.var_nscr)), (((locals.var_temp__blk1038_dn6 * locals.var_nscr) - (locals.var_temp__blk1038 * locals.var_nscr_dn6)) / (locals.var_nscr * locals.var_nscr)), (((locals.var_temp__blk1038_dn7 * locals.var_nscr) - (locals.var_temp__blk1038 * locals.var_nscr_dn7)) / (locals.var_nscr * locals.var_nscr)), (((locals.var_temp__blk1038_dn8 * locals.var_nscr) - (locals.var_temp__blk1038 * locals.var_nscr_dn8)) / (locals.var_nscr * locals.var_nscr)), (((locals.var_temp__blk1038_dn12 * locals.var_nscr) - (locals.var_temp__blk1038 * locals.var_nscr_dn12)) / (locals.var_nscr * locals.var_nscr)), (((locals.var_temp__blk1038_dn13 * locals.var_nscr) - (locals.var_temp__blk1038 * locals.var_nscr_dn13)) / (locals.var_nscr * locals.var_nscr)), (((locals.var_temp__blk1038_dn14 * locals.var_nscr) - (locals.var_temp__blk1038 * locals.var_nscr_dn14)) / (locals.var_nscr * locals.var_nscr)), (((locals.var_temp__blk1038_dn15 * locals.var_nscr) - (locals.var_temp__blk1038 * locals.var_nscr_dn15)) / (locals.var_nscr * locals.var_nscr)), (((locals.var_temp__blk1038_dn16 * locals.var_nscr) - (locals.var_temp__blk1038 * locals.var_nscr_dn16)) / (locals.var_nscr * locals.var_nscr)), (((locals.var_temp__blk1038_dn17 * locals.var_nscr) - (locals.var_temp__blk1038 * locals.var_nscr_dn17)) / (locals.var_nscr * locals.var_nscr)), (((locals.var_temp__blk1038_dn18 * locals.var_nscr) - (locals.var_temp__blk1038 * locals.var_nscr_dn18)) / (locals.var_nscr * locals.var_nscr)), (((locals.var_temp__blk1038_dn19 * locals.var_nscr) - (locals.var_temp__blk1038 * locals.var_nscr_dn19)) / (locals.var_nscr * locals.var_nscr)), (((locals.var_temp__blk1038_dn20 * locals.var_nscr) - (locals.var_temp__blk1038 * locals.var_nscr_dn20)) / (locals.var_nscr * locals.var_nscr)),)
    } else {
        (locals.var_dscr0, locals.var_dscr0_dn5, locals.var_dscr0_dn6, locals.var_dscr0_dn7, locals.var_dscr0_dn8, locals.var_dscr0_dn12, locals.var_dscr0_dn13, locals.var_dscr0_dn14, locals.var_dscr0_dn15, locals.var_dscr0_dn16, locals.var_dscr0_dn17, locals.var_dscr0_dn18, locals.var_dscr0_dn19, locals.var_dscr0_dn20,)
    }
};
        locals.var_dscr0 = assign41630_e54585;
        locals.var_dscr0_dn5 = assign41630_e54585_d_n5;
        locals.var_dscr0_dn6 = assign41630_e54585_d_n6;
        locals.var_dscr0_dn7 = assign41630_e54585_d_n7;
        locals.var_dscr0_dn8 = assign41630_e54585_d_n8;
        locals.var_dscr0_dn12 = assign41630_e54585_d_n12;
        locals.var_dscr0_dn13 = assign41630_e54585_d_n13;
        locals.var_dscr0_dn14 = assign41630_e54585_d_n14;
        locals.var_dscr0_dn15 = assign41630_e54585_d_n15;
        locals.var_dscr0_dn16 = assign41630_e54585_d_n16;
        locals.var_dscr0_dn17 = assign41630_e54585_d_n17;
        locals.var_dscr0_dn18 = assign41630_e54585_d_n18;
        locals.var_dscr0_dn19 = assign41630_e54585_d_n19;
        locals.var_dscr0_dn20 = assign41630_e54585_d_n20;
        locals.var_dscr0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_19(
        locals: &mut StampLocals,
    ) {
        let (assign41640_e54595, assign41640_e54595_d_n5, assign41640_e54595_d_n6, assign41640_e54595_d_n7, assign41640_e54595_d_n8, assign41640_e54595_d_n12, assign41640_e54595_d_n13, assign41640_e54595_d_n14, assign41640_e54595_d_n15, assign41640_e54595_d_n16, assign41640_e54595_d_n17, assign41640_e54595_d_n18, assign41640_e54595_d_n19, assign41640_e54595_d_n20,) = {
    if (locals.var_guard1280 != 0.0) {
        let assign41640_e54590: f64 = (locals.var_qiscr0 + 1.0);
        let assign41640_e54591: f64 = (2.0 * assign41640_e54590);
        let assign41640_e54593: f64 = (assign41640_e54591 - locals.var_dscr0);
        (assign41640_e54593, ((2.0 * locals.var_qiscr0_dn5) - locals.var_dscr0_dn5), ((2.0 * locals.var_qiscr0_dn6) - locals.var_dscr0_dn6), ((2.0 * locals.var_qiscr0_dn7) - locals.var_dscr0_dn7), ((2.0 * locals.var_qiscr0_dn8) - locals.var_dscr0_dn8), ((2.0 * locals.var_qiscr0_dn12) - locals.var_dscr0_dn12), ((2.0 * locals.var_qiscr0_dn13) - locals.var_dscr0_dn13), ((2.0 * locals.var_qiscr0_dn14) - locals.var_dscr0_dn14), ((2.0 * locals.var_qiscr0_dn15) - locals.var_dscr0_dn15), ((2.0 * locals.var_qiscr0_dn16) - locals.var_dscr0_dn16), ((2.0 * locals.var_qiscr0_dn17) - locals.var_dscr0_dn17), ((2.0 * locals.var_qiscr0_dn18) - locals.var_dscr0_dn18), ((2.0 * locals.var_qiscr0_dn19) - locals.var_dscr0_dn19), ((2.0 * locals.var_qiscr0_dn20) - locals.var_dscr0_dn20),)
    } else {
        (locals.var_temp__blk1038, locals.var_temp__blk1038_dn5, locals.var_temp__blk1038_dn6, locals.var_temp__blk1038_dn7, locals.var_temp__blk1038_dn8, locals.var_temp__blk1038_dn12, locals.var_temp__blk1038_dn13, locals.var_temp__blk1038_dn14, locals.var_temp__blk1038_dn15, locals.var_temp__blk1038_dn16, locals.var_temp__blk1038_dn17, locals.var_temp__blk1038_dn18, locals.var_temp__blk1038_dn19, locals.var_temp__blk1038_dn20,)
    }
};
        locals.var_temp__blk1038 = assign41640_e54595;
        locals.var_temp__blk1038_dn5 = assign41640_e54595_d_n5;
        locals.var_temp__blk1038_dn6 = assign41640_e54595_d_n6;
        locals.var_temp__blk1038_dn7 = assign41640_e54595_d_n7;
        locals.var_temp__blk1038_dn8 = assign41640_e54595_d_n8;
        locals.var_temp__blk1038_dn12 = assign41640_e54595_d_n12;
        locals.var_temp__blk1038_dn13 = assign41640_e54595_d_n13;
        locals.var_temp__blk1038_dn14 = assign41640_e54595_d_n14;
        locals.var_temp__blk1038_dn15 = assign41640_e54595_d_n15;
        locals.var_temp__blk1038_dn16 = assign41640_e54595_d_n16;
        locals.var_temp__blk1038_dn17 = assign41640_e54595_d_n17;
        locals.var_temp__blk1038_dn18 = assign41640_e54595_d_n18;
        locals.var_temp__blk1038_dn19 = assign41640_e54595_d_n19;
        locals.var_temp__blk1038_dn20 = assign41640_e54595_d_n20;
        locals.var_temp__blk1038_rv = 0.0;

        let assign41650_e54598: f64 = if locals.var_dscr0 > 1e-6 { 1.0 } else { 0.0 };
        locals.var_guard1282 = assign41650_e54598;
        locals.var_guard1282_rv = 0.0;

        let (assign41660_e54619, assign41660_e54619_d_n5, assign41660_e54619_d_n6, assign41660_e54619_d_n7, assign41660_e54619_d_n8, assign41660_e54619_d_n12, assign41660_e54619_d_n13, assign41660_e54619_d_n14, assign41660_e54619_d_n15, assign41660_e54619_d_n16, assign41660_e54619_d_n17, assign41660_e54619_d_n18, assign41660_e54619_d_n19, assign41660_e54619_d_n20,) = {
    if ((locals.var_guard1280 != 0.0) && (locals.var_guard1282 != 0.0)) {
        let assign41660_e54607: f64 = (locals.var_dscr0 * locals.var_temp__blk1038);
        let assign41660_e54608: f64 = (1.0 + assign41660_e54607);
        let assign41660_e54609: f64 = (assign41660_e54608).sqrt();
        let assign41660_e54611: f64 = (assign41660_e54609 - 1.0);
        let assign41660_e54613: f64 = (assign41660_e54611 / locals.var_dscr0);
        let assign41660_e54614: f64 = (locals.var_qiscr0 - assign41660_e54613);
        let assign41660_e54616: f64 = (assign41660_e54614 + 1.0);
        let assign41660_e54617: f64 = (locals.var_nscr * assign41660_e54616);
        (assign41660_e54617, ((locals.var_nscr_dn5 * assign41660_e54616) + (locals.var_nscr * (locals.var_qiscr0_dn5 - ((((((locals.var_dscr0_dn5 * locals.var_temp__blk1038) + (locals.var_dscr0 * locals.var_temp__blk1038_dn5)) / (2.0 * assign41660_e54609)) * locals.var_dscr0) - (assign41660_e54611 * locals.var_dscr0_dn5)) / (locals.var_dscr0 * locals.var_dscr0))))), ((locals.var_nscr_dn6 * assign41660_e54616) + (locals.var_nscr * (locals.var_qiscr0_dn6 - ((((((locals.var_dscr0_dn6 * locals.var_temp__blk1038) + (locals.var_dscr0 * locals.var_temp__blk1038_dn6)) / (2.0 * assign41660_e54609)) * locals.var_dscr0) - (assign41660_e54611 * locals.var_dscr0_dn6)) / (locals.var_dscr0 * locals.var_dscr0))))), ((locals.var_nscr_dn7 * assign41660_e54616) + (locals.var_nscr * (locals.var_qiscr0_dn7 - ((((((locals.var_dscr0_dn7 * locals.var_temp__blk1038) + (locals.var_dscr0 * locals.var_temp__blk1038_dn7)) / (2.0 * assign41660_e54609)) * locals.var_dscr0) - (assign41660_e54611 * locals.var_dscr0_dn7)) / (locals.var_dscr0 * locals.var_dscr0))))), ((locals.var_nscr_dn8 * assign41660_e54616) + (locals.var_nscr * (locals.var_qiscr0_dn8 - ((((((locals.var_dscr0_dn8 * locals.var_temp__blk1038) + (locals.var_dscr0 * locals.var_temp__blk1038_dn8)) / (2.0 * assign41660_e54609)) * locals.var_dscr0) - (assign41660_e54611 * locals.var_dscr0_dn8)) / (locals.var_dscr0 * locals.var_dscr0))))), ((locals.var_nscr_dn12 * assign41660_e54616) + (locals.var_nscr * (locals.var_qiscr0_dn12 - ((((((locals.var_dscr0_dn12 * locals.var_temp__blk1038) + (locals.var_dscr0 * locals.var_temp__blk1038_dn12)) / (2.0 * assign41660_e54609)) * locals.var_dscr0) - (assign41660_e54611 * locals.var_dscr0_dn12)) / (locals.var_dscr0 * locals.var_dscr0))))), ((locals.var_nscr_dn13 * assign41660_e54616) + (locals.var_nscr * (locals.var_qiscr0_dn13 - ((((((locals.var_dscr0_dn13 * locals.var_temp__blk1038) + (locals.var_dscr0 * locals.var_temp__blk1038_dn13)) / (2.0 * assign41660_e54609)) * locals.var_dscr0) - (assign41660_e54611 * locals.var_dscr0_dn13)) / (locals.var_dscr0 * locals.var_dscr0))))), ((locals.var_nscr_dn14 * assign41660_e54616) + (locals.var_nscr * (locals.var_qiscr0_dn14 - ((((((locals.var_dscr0_dn14 * locals.var_temp__blk1038) + (locals.var_dscr0 * locals.var_temp__blk1038_dn14)) / (2.0 * assign41660_e54609)) * locals.var_dscr0) - (assign41660_e54611 * locals.var_dscr0_dn14)) / (locals.var_dscr0 * locals.var_dscr0))))), ((locals.var_nscr_dn15 * assign41660_e54616) + (locals.var_nscr * (locals.var_qiscr0_dn15 - ((((((locals.var_dscr0_dn15 * locals.var_temp__blk1038) + (locals.var_dscr0 * locals.var_temp__blk1038_dn15)) / (2.0 * assign41660_e54609)) * locals.var_dscr0) - (assign41660_e54611 * locals.var_dscr0_dn15)) / (locals.var_dscr0 * locals.var_dscr0))))), ((locals.var_nscr_dn16 * assign41660_e54616) + (locals.var_nscr * (locals.var_qiscr0_dn16 - ((((((locals.var_dscr0_dn16 * locals.var_temp__blk1038) + (locals.var_dscr0 * locals.var_temp__blk1038_dn16)) / (2.0 * assign41660_e54609)) * locals.var_dscr0) - (assign41660_e54611 * locals.var_dscr0_dn16)) / (locals.var_dscr0 * locals.var_dscr0))))), ((locals.var_nscr_dn17 * assign41660_e54616) + (locals.var_nscr * (locals.var_qiscr0_dn17 - ((((((locals.var_dscr0_dn17 * locals.var_temp__blk1038) + (locals.var_dscr0 * locals.var_temp__blk1038_dn17)) / (2.0 * assign41660_e54609)) * locals.var_dscr0) - (assign41660_e54611 * locals.var_dscr0_dn17)) / (locals.var_dscr0 * locals.var_dscr0))))), ((locals.var_nscr_dn18 * assign41660_e54616) + (locals.var_nscr * (locals.var_qiscr0_dn18 - ((((((locals.var_dscr0_dn18 * locals.var_temp__blk1038) + (locals.var_dscr0 * locals.var_temp__blk1038_dn18)) / (2.0 * assign41660_e54609)) * locals.var_dscr0) - (assign41660_e54611 * locals.var_dscr0_dn18)) / (locals.var_dscr0 * locals.var_dscr0))))), ((locals.var_nscr_dn19 * assign41660_e54616) + (locals.var_nscr * (locals.var_qiscr0_dn19 - ((((((locals.var_dscr0_dn19 * locals.var_temp__blk1038) + (locals.var_dscr0 * locals.var_temp__blk1038_dn19)) / (2.0 * assign41660_e54609)) * locals.var_dscr0) - (assign41660_e54611 * locals.var_dscr0_dn19)) / (locals.var_dscr0 * locals.var_dscr0))))), ((locals.var_nscr_dn20 * assign41660_e54616) + (locals.var_nscr * (locals.var_qiscr0_dn20 - ((((((locals.var_dscr0_dn20 * locals.var_temp__blk1038) + (locals.var_dscr0 * locals.var_temp__blk1038_dn20)) / (2.0 * assign41660_e54609)) * locals.var_dscr0) - (assign41660_e54611 * locals.var_dscr0_dn20)) / (locals.var_dscr0 * locals.var_dscr0))))),)
    } else {
        (locals.var_qiscr, locals.var_qiscr_dn5, locals.var_qiscr_dn6, locals.var_qiscr_dn7, locals.var_qiscr_dn8, locals.var_qiscr_dn12, locals.var_qiscr_dn13, locals.var_qiscr_dn14, locals.var_qiscr_dn15, locals.var_qiscr_dn16, locals.var_qiscr_dn17, locals.var_qiscr_dn18, locals.var_qiscr_dn19, locals.var_qiscr_dn20,)
    }
};
        locals.var_qiscr = assign41660_e54619;
        locals.var_qiscr_dn5 = assign41660_e54619_d_n5;
        locals.var_qiscr_dn6 = assign41660_e54619_d_n6;
        locals.var_qiscr_dn7 = assign41660_e54619_d_n7;
        locals.var_qiscr_dn8 = assign41660_e54619_d_n8;
        locals.var_qiscr_dn12 = assign41660_e54619_d_n12;
        locals.var_qiscr_dn13 = assign41660_e54619_d_n13;
        locals.var_qiscr_dn14 = assign41660_e54619_d_n14;
        locals.var_qiscr_dn15 = assign41660_e54619_d_n15;
        locals.var_qiscr_dn16 = assign41660_e54619_d_n16;
        locals.var_qiscr_dn17 = assign41660_e54619_d_n17;
        locals.var_qiscr_dn18 = assign41660_e54619_d_n18;
        locals.var_qiscr_dn19 = assign41660_e54619_d_n19;
        locals.var_qiscr_dn20 = assign41660_e54619_d_n20;
        locals.var_qiscr_rv = 0.0;

        let (assign41670_e54638, assign41670_e54638_d_n5, assign41670_e54638_d_n6, assign41670_e54638_d_n7, assign41670_e54638_d_n8, assign41670_e54638_d_n12, assign41670_e54638_d_n13, assign41670_e54638_d_n14, assign41670_e54638_d_n15, assign41670_e54638_d_n16, assign41670_e54638_d_n17, assign41670_e54638_d_n18, assign41670_e54638_d_n19, assign41670_e54638_d_n20,) = {
    if ((locals.var_guard1280 != 0.0) && (locals.var_guard1282 == 0.0)) {
        let assign41670_e54626: f64 = (locals.var_nscr * 0.5);
        let assign41670_e54628: f64 = (assign41670_e54626 * locals.var_dscr0);
        let assign41670_e54632: f64 = (0.25 * locals.var_temp__blk1038);
        let assign41670_e54634: f64 = (assign41670_e54632 * locals.var_temp__blk1038);
        let assign41670_e54635: f64 = (1.0 + assign41670_e54634);
        let assign41670_e54636: f64 = (assign41670_e54628 * assign41670_e54635);
        (assign41670_e54636, (((((locals.var_nscr_dn5 * 0.5) * locals.var_dscr0) + (assign41670_e54626 * locals.var_dscr0_dn5)) * assign41670_e54635) + (assign41670_e54628 * (((0.25 * locals.var_temp__blk1038_dn5) * locals.var_temp__blk1038) + (assign41670_e54632 * locals.var_temp__blk1038_dn5)))), (((((locals.var_nscr_dn6 * 0.5) * locals.var_dscr0) + (assign41670_e54626 * locals.var_dscr0_dn6)) * assign41670_e54635) + (assign41670_e54628 * (((0.25 * locals.var_temp__blk1038_dn6) * locals.var_temp__blk1038) + (assign41670_e54632 * locals.var_temp__blk1038_dn6)))), (((((locals.var_nscr_dn7 * 0.5) * locals.var_dscr0) + (assign41670_e54626 * locals.var_dscr0_dn7)) * assign41670_e54635) + (assign41670_e54628 * (((0.25 * locals.var_temp__blk1038_dn7) * locals.var_temp__blk1038) + (assign41670_e54632 * locals.var_temp__blk1038_dn7)))), (((((locals.var_nscr_dn8 * 0.5) * locals.var_dscr0) + (assign41670_e54626 * locals.var_dscr0_dn8)) * assign41670_e54635) + (assign41670_e54628 * (((0.25 * locals.var_temp__blk1038_dn8) * locals.var_temp__blk1038) + (assign41670_e54632 * locals.var_temp__blk1038_dn8)))), (((((locals.var_nscr_dn12 * 0.5) * locals.var_dscr0) + (assign41670_e54626 * locals.var_dscr0_dn12)) * assign41670_e54635) + (assign41670_e54628 * (((0.25 * locals.var_temp__blk1038_dn12) * locals.var_temp__blk1038) + (assign41670_e54632 * locals.var_temp__blk1038_dn12)))), (((((locals.var_nscr_dn13 * 0.5) * locals.var_dscr0) + (assign41670_e54626 * locals.var_dscr0_dn13)) * assign41670_e54635) + (assign41670_e54628 * (((0.25 * locals.var_temp__blk1038_dn13) * locals.var_temp__blk1038) + (assign41670_e54632 * locals.var_temp__blk1038_dn13)))), (((((locals.var_nscr_dn14 * 0.5) * locals.var_dscr0) + (assign41670_e54626 * locals.var_dscr0_dn14)) * assign41670_e54635) + (assign41670_e54628 * (((0.25 * locals.var_temp__blk1038_dn14) * locals.var_temp__blk1038) + (assign41670_e54632 * locals.var_temp__blk1038_dn14)))), (((((locals.var_nscr_dn15 * 0.5) * locals.var_dscr0) + (assign41670_e54626 * locals.var_dscr0_dn15)) * assign41670_e54635) + (assign41670_e54628 * (((0.25 * locals.var_temp__blk1038_dn15) * locals.var_temp__blk1038) + (assign41670_e54632 * locals.var_temp__blk1038_dn15)))), (((((locals.var_nscr_dn16 * 0.5) * locals.var_dscr0) + (assign41670_e54626 * locals.var_dscr0_dn16)) * assign41670_e54635) + (assign41670_e54628 * (((0.25 * locals.var_temp__blk1038_dn16) * locals.var_temp__blk1038) + (assign41670_e54632 * locals.var_temp__blk1038_dn16)))), (((((locals.var_nscr_dn17 * 0.5) * locals.var_dscr0) + (assign41670_e54626 * locals.var_dscr0_dn17)) * assign41670_e54635) + (assign41670_e54628 * (((0.25 * locals.var_temp__blk1038_dn17) * locals.var_temp__blk1038) + (assign41670_e54632 * locals.var_temp__blk1038_dn17)))), (((((locals.var_nscr_dn18 * 0.5) * locals.var_dscr0) + (assign41670_e54626 * locals.var_dscr0_dn18)) * assign41670_e54635) + (assign41670_e54628 * (((0.25 * locals.var_temp__blk1038_dn18) * locals.var_temp__blk1038) + (assign41670_e54632 * locals.var_temp__blk1038_dn18)))), (((((locals.var_nscr_dn19 * 0.5) * locals.var_dscr0) + (assign41670_e54626 * locals.var_dscr0_dn19)) * assign41670_e54635) + (assign41670_e54628 * (((0.25 * locals.var_temp__blk1038_dn19) * locals.var_temp__blk1038) + (assign41670_e54632 * locals.var_temp__blk1038_dn19)))), (((((locals.var_nscr_dn20 * 0.5) * locals.var_dscr0) + (assign41670_e54626 * locals.var_dscr0_dn20)) * assign41670_e54635) + (assign41670_e54628 * (((0.25 * locals.var_temp__blk1038_dn20) * locals.var_temp__blk1038) + (assign41670_e54632 * locals.var_temp__blk1038_dn20)))),)
    } else {
        (locals.var_qiscr, locals.var_qiscr_dn5, locals.var_qiscr_dn6, locals.var_qiscr_dn7, locals.var_qiscr_dn8, locals.var_qiscr_dn12, locals.var_qiscr_dn13, locals.var_qiscr_dn14, locals.var_qiscr_dn15, locals.var_qiscr_dn16, locals.var_qiscr_dn17, locals.var_qiscr_dn18, locals.var_qiscr_dn19, locals.var_qiscr_dn20,)
    }
};
        locals.var_qiscr = assign41670_e54638;
        locals.var_qiscr_dn5 = assign41670_e54638_d_n5;
        locals.var_qiscr_dn6 = assign41670_e54638_d_n6;
        locals.var_qiscr_dn7 = assign41670_e54638_d_n7;
        locals.var_qiscr_dn8 = assign41670_e54638_d_n8;
        locals.var_qiscr_dn12 = assign41670_e54638_d_n12;
        locals.var_qiscr_dn13 = assign41670_e54638_d_n13;
        locals.var_qiscr_dn14 = assign41670_e54638_d_n14;
        locals.var_qiscr_dn15 = assign41670_e54638_d_n15;
        locals.var_qiscr_dn16 = assign41670_e54638_d_n16;
        locals.var_qiscr_dn17 = assign41670_e54638_d_n17;
        locals.var_qiscr_dn18 = assign41670_e54638_d_n18;
        locals.var_qiscr_dn19 = assign41670_e54638_d_n19;
        locals.var_qiscr_dn20 = assign41670_e54638_d_n20;
        locals.var_qiscr_rv = 0.0;

        let (assign41680_e54663, assign41680_e54663_d_n5, assign41680_e54663_d_n6, assign41680_e54663_d_n7, assign41680_e54663_d_n8, assign41680_e54663_d_n12, assign41680_e54663_d_n13, assign41680_e54663_d_n14, assign41680_e54663_d_n15, assign41680_e54663_d_n16, assign41680_e54663_d_n17, assign41680_e54663_d_n18, assign41680_e54663_d_n19, assign41680_e54663_d_n20,) = {
    if (locals.var_guard1280 != 0.0) {
        let assign41680_e54643: f64 = (locals.var_xg - locals.var_qiscr);
        let assign41680_e54645: f64 = (assign41680_e54643 + 2.0);
        let assign41680_e54648: f64 = (locals.var_xg - locals.var_qiscr);
        let assign41680_e54650: f64 = (assign41680_e54648 - 2.0);
        let assign41680_e54653: f64 = (locals.var_xg - locals.var_qiscr);
        let assign41680_e54655: f64 = (assign41680_e54653 - 2.0);
        let assign41680_e54656: f64 = (assign41680_e54650 * assign41680_e54655);
        let assign41680_e54658: f64 = (assign41680_e54656 + 1.0);
        let assign41680_e54659: f64 = (assign41680_e54658).sqrt();
        let assign41680_e54660: f64 = (assign41680_e54645 + assign41680_e54659);
        let assign41680_e54661: f64 = (0.5 * assign41680_e54660);
        (assign41680_e54661, (0.5 * ((locals.var_xg_dn5 - locals.var_qiscr_dn5) + ((((locals.var_xg_dn5 - locals.var_qiscr_dn5) * assign41680_e54655) + (assign41680_e54650 * (locals.var_xg_dn5 - locals.var_qiscr_dn5))) / (2.0 * assign41680_e54659)))), (0.5 * ((locals.var_xg_dn6 - locals.var_qiscr_dn6) + ((((locals.var_xg_dn6 - locals.var_qiscr_dn6) * assign41680_e54655) + (assign41680_e54650 * (locals.var_xg_dn6 - locals.var_qiscr_dn6))) / (2.0 * assign41680_e54659)))), (0.5 * ((locals.var_xg_dn7 - locals.var_qiscr_dn7) + ((((locals.var_xg_dn7 - locals.var_qiscr_dn7) * assign41680_e54655) + (assign41680_e54650 * (locals.var_xg_dn7 - locals.var_qiscr_dn7))) / (2.0 * assign41680_e54659)))), (0.5 * ((locals.var_xg_dn8 - locals.var_qiscr_dn8) + ((((locals.var_xg_dn8 - locals.var_qiscr_dn8) * assign41680_e54655) + (assign41680_e54650 * (locals.var_xg_dn8 - locals.var_qiscr_dn8))) / (2.0 * assign41680_e54659)))), (0.5 * ((locals.var_xg_dn12 - locals.var_qiscr_dn12) + ((((locals.var_xg_dn12 - locals.var_qiscr_dn12) * assign41680_e54655) + (assign41680_e54650 * (locals.var_xg_dn12 - locals.var_qiscr_dn12))) / (2.0 * assign41680_e54659)))), (0.5 * ((locals.var_xg_dn13 - locals.var_qiscr_dn13) + ((((locals.var_xg_dn13 - locals.var_qiscr_dn13) * assign41680_e54655) + (assign41680_e54650 * (locals.var_xg_dn13 - locals.var_qiscr_dn13))) / (2.0 * assign41680_e54659)))), (0.5 * ((locals.var_xg_dn14 - locals.var_qiscr_dn14) + ((((locals.var_xg_dn14 - locals.var_qiscr_dn14) * assign41680_e54655) + (assign41680_e54650 * (locals.var_xg_dn14 - locals.var_qiscr_dn14))) / (2.0 * assign41680_e54659)))), (0.5 * ((locals.var_xg_dn15 - locals.var_qiscr_dn15) + ((((locals.var_xg_dn15 - locals.var_qiscr_dn15) * assign41680_e54655) + (assign41680_e54650 * (locals.var_xg_dn15 - locals.var_qiscr_dn15))) / (2.0 * assign41680_e54659)))), (0.5 * ((locals.var_xg_dn16 - locals.var_qiscr_dn16) + ((((locals.var_xg_dn16 - locals.var_qiscr_dn16) * assign41680_e54655) + (assign41680_e54650 * (locals.var_xg_dn16 - locals.var_qiscr_dn16))) / (2.0 * assign41680_e54659)))), (0.5 * ((locals.var_xg_dn17 - locals.var_qiscr_dn17) + ((((locals.var_xg_dn17 - locals.var_qiscr_dn17) * assign41680_e54655) + (assign41680_e54650 * (locals.var_xg_dn17 - locals.var_qiscr_dn17))) / (2.0 * assign41680_e54659)))), (0.5 * ((locals.var_xg_dn18 - locals.var_qiscr_dn18) + ((((locals.var_xg_dn18 - locals.var_qiscr_dn18) * assign41680_e54655) + (assign41680_e54650 * (locals.var_xg_dn18 - locals.var_qiscr_dn18))) / (2.0 * assign41680_e54659)))), (0.5 * ((locals.var_xg_dn19 - locals.var_qiscr_dn19) + ((((locals.var_xg_dn19 - locals.var_qiscr_dn19) * assign41680_e54655) + (assign41680_e54650 * (locals.var_xg_dn19 - locals.var_qiscr_dn19))) / (2.0 * assign41680_e54659)))), (0.5 * ((locals.var_xg_dn20 - locals.var_qiscr_dn20) + ((((locals.var_xg_dn20 - locals.var_qiscr_dn20) * assign41680_e54655) + (assign41680_e54650 * (locals.var_xg_dn20 - locals.var_qiscr_dn20))) / (2.0 * assign41680_e54659)))),)
    } else {
        (locals.var_temp__blk1038, locals.var_temp__blk1038_dn5, locals.var_temp__blk1038_dn6, locals.var_temp__blk1038_dn7, locals.var_temp__blk1038_dn8, locals.var_temp__blk1038_dn12, locals.var_temp__blk1038_dn13, locals.var_temp__blk1038_dn14, locals.var_temp__blk1038_dn15, locals.var_temp__blk1038_dn16, locals.var_temp__blk1038_dn17, locals.var_temp__blk1038_dn18, locals.var_temp__blk1038_dn19, locals.var_temp__blk1038_dn20,)
    }
};
        locals.var_temp__blk1038 = assign41680_e54663;
        locals.var_temp__blk1038_dn5 = assign41680_e54663_d_n5;
        locals.var_temp__blk1038_dn6 = assign41680_e54663_d_n6;
        locals.var_temp__blk1038_dn7 = assign41680_e54663_d_n7;
        locals.var_temp__blk1038_dn8 = assign41680_e54663_d_n8;
        locals.var_temp__blk1038_dn12 = assign41680_e54663_d_n12;
        locals.var_temp__blk1038_dn13 = assign41680_e54663_d_n13;
        locals.var_temp__blk1038_dn14 = assign41680_e54663_d_n14;
        locals.var_temp__blk1038_dn15 = assign41680_e54663_d_n15;
        locals.var_temp__blk1038_dn16 = assign41680_e54663_d_n16;
        locals.var_temp__blk1038_dn17 = assign41680_e54663_d_n17;
        locals.var_temp__blk1038_dn18 = assign41680_e54663_d_n18;
        locals.var_temp__blk1038_dn19 = assign41680_e54663_d_n19;
        locals.var_temp__blk1038_dn20 = assign41680_e54663_d_n20;
        locals.var_temp__blk1038_rv = 0.0;

        let (assign41690_e54680, assign41690_e54680_d_n5, assign41690_e54680_d_n6, assign41690_e54680_d_n7, assign41690_e54680_d_n8, assign41690_e54680_d_n12, assign41690_e54680_d_n13, assign41690_e54680_d_n14, assign41690_e54680_d_n15, assign41690_e54680_d_n16, assign41690_e54680_d_n17, assign41690_e54680_d_n18, assign41690_e54680_d_n19, assign41690_e54680_d_n20,) = {
    if (locals.var_guard1280 != 0.0) {
        let assign41690_e54667: f64 = (0.5 * locals.var_gf2);
        let assign41690_e54671: f64 = (4.0 / locals.var_gf2);
        let assign41690_e54673: f64 = (assign41690_e54671 * locals.var_temp__blk1038);
        let assign41690_e54674: f64 = (1.0 + assign41690_e54673);
        let assign41690_e54675: f64 = (assign41690_e54674).sqrt();
        let assign41690_e54677: f64 = (assign41690_e54675 - 1.0);
        let assign41690_e54678: f64 = (assign41690_e54667 * assign41690_e54677);
        (assign41690_e54678, (((0.5 * locals.var_gf2_dn5) * assign41690_e54677) + (assign41690_e54667 * ((((-((4.0 * locals.var_gf2_dn5) / (locals.var_gf2 * locals.var_gf2))) * locals.var_temp__blk1038) + (assign41690_e54671 * locals.var_temp__blk1038_dn5)) / (2.0 * assign41690_e54675)))), (((0.5 * locals.var_gf2_dn6) * assign41690_e54677) + (assign41690_e54667 * ((((-((4.0 * locals.var_gf2_dn6) / (locals.var_gf2 * locals.var_gf2))) * locals.var_temp__blk1038) + (assign41690_e54671 * locals.var_temp__blk1038_dn6)) / (2.0 * assign41690_e54675)))), (((0.5 * locals.var_gf2_dn7) * assign41690_e54677) + (assign41690_e54667 * ((((-((4.0 * locals.var_gf2_dn7) / (locals.var_gf2 * locals.var_gf2))) * locals.var_temp__blk1038) + (assign41690_e54671 * locals.var_temp__blk1038_dn7)) / (2.0 * assign41690_e54675)))), (((0.5 * locals.var_gf2_dn8) * assign41690_e54677) + (assign41690_e54667 * ((((-((4.0 * locals.var_gf2_dn8) / (locals.var_gf2 * locals.var_gf2))) * locals.var_temp__blk1038) + (assign41690_e54671 * locals.var_temp__blk1038_dn8)) / (2.0 * assign41690_e54675)))), (((0.5 * locals.var_gf2_dn12) * assign41690_e54677) + (assign41690_e54667 * ((((-((4.0 * locals.var_gf2_dn12) / (locals.var_gf2 * locals.var_gf2))) * locals.var_temp__blk1038) + (assign41690_e54671 * locals.var_temp__blk1038_dn12)) / (2.0 * assign41690_e54675)))), (((0.5 * locals.var_gf2_dn13) * assign41690_e54677) + (assign41690_e54667 * ((((-((4.0 * locals.var_gf2_dn13) / (locals.var_gf2 * locals.var_gf2))) * locals.var_temp__blk1038) + (assign41690_e54671 * locals.var_temp__blk1038_dn13)) / (2.0 * assign41690_e54675)))), (((0.5 * locals.var_gf2_dn14) * assign41690_e54677) + (assign41690_e54667 * ((((-((4.0 * locals.var_gf2_dn14) / (locals.var_gf2 * locals.var_gf2))) * locals.var_temp__blk1038) + (assign41690_e54671 * locals.var_temp__blk1038_dn14)) / (2.0 * assign41690_e54675)))), (((0.5 * locals.var_gf2_dn15) * assign41690_e54677) + (assign41690_e54667 * ((((-((4.0 * locals.var_gf2_dn15) / (locals.var_gf2 * locals.var_gf2))) * locals.var_temp__blk1038) + (assign41690_e54671 * locals.var_temp__blk1038_dn15)) / (2.0 * assign41690_e54675)))), (((0.5 * locals.var_gf2_dn16) * assign41690_e54677) + (assign41690_e54667 * ((((-((4.0 * locals.var_gf2_dn16) / (locals.var_gf2 * locals.var_gf2))) * locals.var_temp__blk1038) + (assign41690_e54671 * locals.var_temp__blk1038_dn16)) / (2.0 * assign41690_e54675)))), (((0.5 * locals.var_gf2_dn17) * assign41690_e54677) + (assign41690_e54667 * ((((-((4.0 * locals.var_gf2_dn17) / (locals.var_gf2 * locals.var_gf2))) * locals.var_temp__blk1038) + (assign41690_e54671 * locals.var_temp__blk1038_dn17)) / (2.0 * assign41690_e54675)))), (((0.5 * locals.var_gf2_dn18) * assign41690_e54677) + (assign41690_e54667 * ((((-((4.0 * locals.var_gf2_dn18) / (locals.var_gf2 * locals.var_gf2))) * locals.var_temp__blk1038) + (assign41690_e54671 * locals.var_temp__blk1038_dn18)) / (2.0 * assign41690_e54675)))), (((0.5 * locals.var_gf2_dn19) * assign41690_e54677) + (assign41690_e54667 * ((((-((4.0 * locals.var_gf2_dn19) / (locals.var_gf2 * locals.var_gf2))) * locals.var_temp__blk1038) + (assign41690_e54671 * locals.var_temp__blk1038_dn19)) / (2.0 * assign41690_e54675)))), (((0.5 * locals.var_gf2_dn20) * assign41690_e54677) + (assign41690_e54667 * ((((-((4.0 * locals.var_gf2_dn20) / (locals.var_gf2 * locals.var_gf2))) * locals.var_temp__blk1038) + (assign41690_e54671 * locals.var_temp__blk1038_dn20)) / (2.0 * assign41690_e54675)))),)
    } else {
        (locals.var_qbscr, locals.var_qbscr_dn5, locals.var_qbscr_dn6, locals.var_qbscr_dn7, locals.var_qbscr_dn8, locals.var_qbscr_dn12, locals.var_qbscr_dn13, locals.var_qbscr_dn14, locals.var_qbscr_dn15, locals.var_qbscr_dn16, locals.var_qbscr_dn17, locals.var_qbscr_dn18, locals.var_qbscr_dn19, locals.var_qbscr_dn20,)
    }
};
        locals.var_qbscr = assign41690_e54680;
        locals.var_qbscr_dn5 = assign41690_e54680_d_n5;
        locals.var_qbscr_dn6 = assign41690_e54680_d_n6;
        locals.var_qbscr_dn7 = assign41690_e54680_d_n7;
        locals.var_qbscr_dn8 = assign41690_e54680_d_n8;
        locals.var_qbscr_dn12 = assign41690_e54680_d_n12;
        locals.var_qbscr_dn13 = assign41690_e54680_d_n13;
        locals.var_qbscr_dn14 = assign41690_e54680_d_n14;
        locals.var_qbscr_dn15 = assign41690_e54680_d_n15;
        locals.var_qbscr_dn16 = assign41690_e54680_d_n16;
        locals.var_qbscr_dn17 = assign41690_e54680_d_n17;
        locals.var_qbscr_dn18 = assign41690_e54680_d_n18;
        locals.var_qbscr_dn19 = assign41690_e54680_d_n19;
        locals.var_qbscr_dn20 = assign41690_e54680_d_n20;
        locals.var_qbscr_rv = 0.0;

        let (assign41700_e54688, assign41700_e54688_d_n5, assign41700_e54688_d_n6, assign41700_e54688_d_n7, assign41700_e54688_d_n8, assign41700_e54688_d_n12, assign41700_e54688_d_n13, assign41700_e54688_d_n14, assign41700_e54688_d_n15, assign41700_e54688_d_n16, assign41700_e54688_d_n17, assign41700_e54688_d_n18, assign41700_e54688_d_n19, assign41700_e54688_d_n20,) = {
    if (locals.var_guard1280 != 0.0) {
        let assign41700_e54685: f64 = (locals.var_qbscr + locals.var_qiscr);
        let assign41700_e54686: f64 = (locals.var_qbscr / assign41700_e54685);
        (assign41700_e54686, (((locals.var_qbscr_dn5 * assign41700_e54685) - (locals.var_qbscr * (locals.var_qbscr_dn5 + locals.var_qiscr_dn5))) / (assign41700_e54685 * assign41700_e54685)), (((locals.var_qbscr_dn6 * assign41700_e54685) - (locals.var_qbscr * (locals.var_qbscr_dn6 + locals.var_qiscr_dn6))) / (assign41700_e54685 * assign41700_e54685)), (((locals.var_qbscr_dn7 * assign41700_e54685) - (locals.var_qbscr * (locals.var_qbscr_dn7 + locals.var_qiscr_dn7))) / (assign41700_e54685 * assign41700_e54685)), (((locals.var_qbscr_dn8 * assign41700_e54685) - (locals.var_qbscr * (locals.var_qbscr_dn8 + locals.var_qiscr_dn8))) / (assign41700_e54685 * assign41700_e54685)), (((locals.var_qbscr_dn12 * assign41700_e54685) - (locals.var_qbscr * (locals.var_qbscr_dn12 + locals.var_qiscr_dn12))) / (assign41700_e54685 * assign41700_e54685)), (((locals.var_qbscr_dn13 * assign41700_e54685) - (locals.var_qbscr * (locals.var_qbscr_dn13 + locals.var_qiscr_dn13))) / (assign41700_e54685 * assign41700_e54685)), (((locals.var_qbscr_dn14 * assign41700_e54685) - (locals.var_qbscr * (locals.var_qbscr_dn14 + locals.var_qiscr_dn14))) / (assign41700_e54685 * assign41700_e54685)), (((locals.var_qbscr_dn15 * assign41700_e54685) - (locals.var_qbscr * (locals.var_qbscr_dn15 + locals.var_qiscr_dn15))) / (assign41700_e54685 * assign41700_e54685)), (((locals.var_qbscr_dn16 * assign41700_e54685) - (locals.var_qbscr * (locals.var_qbscr_dn16 + locals.var_qiscr_dn16))) / (assign41700_e54685 * assign41700_e54685)), (((locals.var_qbscr_dn17 * assign41700_e54685) - (locals.var_qbscr * (locals.var_qbscr_dn17 + locals.var_qiscr_dn17))) / (assign41700_e54685 * assign41700_e54685)), (((locals.var_qbscr_dn18 * assign41700_e54685) - (locals.var_qbscr * (locals.var_qbscr_dn18 + locals.var_qiscr_dn18))) / (assign41700_e54685 * assign41700_e54685)), (((locals.var_qbscr_dn19 * assign41700_e54685) - (locals.var_qbscr * (locals.var_qbscr_dn19 + locals.var_qiscr_dn19))) / (assign41700_e54685 * assign41700_e54685)), (((locals.var_qbscr_dn20 * assign41700_e54685) - (locals.var_qbscr * (locals.var_qbscr_dn20 + locals.var_qiscr_dn20))) / (assign41700_e54685 * assign41700_e54685)),)
    } else {
        (locals.var_fscr, locals.var_fscr_dn5, locals.var_fscr_dn6, locals.var_fscr_dn7, locals.var_fscr_dn8, locals.var_fscr_dn12, locals.var_fscr_dn13, locals.var_fscr_dn14, locals.var_fscr_dn15, locals.var_fscr_dn16, locals.var_fscr_dn17, locals.var_fscr_dn18, locals.var_fscr_dn19, locals.var_fscr_dn20,)
    }
};
        locals.var_fscr = assign41700_e54688;
        locals.var_fscr_dn5 = assign41700_e54688_d_n5;
        locals.var_fscr_dn6 = assign41700_e54688_d_n6;
        locals.var_fscr_dn7 = assign41700_e54688_d_n7;
        locals.var_fscr_dn8 = assign41700_e54688_d_n8;
        locals.var_fscr_dn12 = assign41700_e54688_d_n12;
        locals.var_fscr_dn13 = assign41700_e54688_d_n13;
        locals.var_fscr_dn14 = assign41700_e54688_d_n14;
        locals.var_fscr_dn15 = assign41700_e54688_d_n15;
        locals.var_fscr_dn16 = assign41700_e54688_d_n16;
        locals.var_fscr_dn17 = assign41700_e54688_d_n17;
        locals.var_fscr_dn18 = assign41700_e54688_d_n18;
        locals.var_fscr_dn19 = assign41700_e54688_d_n19;
        locals.var_fscr_dn20 = assign41700_e54688_d_n20;
        locals.var_fscr_rv = 0.0;

        let (assign41710_e54696, assign41710_e54696_d_n5, assign41710_e54696_d_n6, assign41710_e54696_d_n7, assign41710_e54696_d_n8, assign41710_e54696_d_n12, assign41710_e54696_d_n13, assign41710_e54696_d_n14, assign41710_e54696_d_n15, assign41710_e54696_d_n16, assign41710_e54696_d_n17, assign41710_e54696_d_n18, assign41710_e54696_d_n19, assign41710_e54696_d_n20,) = {
    if (locals.var_guard1280 != 0.0) {
        let assign41710_e54693: f64 = (locals.var_fscr * locals.var_delxb);
        let assign41710_e54694: f64 = (locals.var_xno_s - assign41710_e54693);
        (assign41710_e54694, (locals.var_xno_s_dn5 - ((locals.var_fscr_dn5 * locals.var_delxb) + (locals.var_fscr * locals.var_delxb_dn5))), (locals.var_xno_s_dn6 - ((locals.var_fscr_dn6 * locals.var_delxb) + (locals.var_fscr * locals.var_delxb_dn6))), (locals.var_xno_s_dn7 - ((locals.var_fscr_dn7 * locals.var_delxb) + (locals.var_fscr * locals.var_delxb_dn7))), (locals.var_xno_s_dn8 - ((locals.var_fscr_dn8 * locals.var_delxb) + (locals.var_fscr * locals.var_delxb_dn8))), (locals.var_xno_s_dn12 - ((locals.var_fscr_dn12 * locals.var_delxb) + (locals.var_fscr * locals.var_delxb_dn12))), (locals.var_xno_s_dn13 - ((locals.var_fscr_dn13 * locals.var_delxb) + (locals.var_fscr * locals.var_delxb_dn13))), (locals.var_xno_s_dn14 - ((locals.var_fscr_dn14 * locals.var_delxb) + (locals.var_fscr * locals.var_delxb_dn14))), (locals.var_xno_s_dn15 - ((locals.var_fscr_dn15 * locals.var_delxb) + (locals.var_fscr * locals.var_delxb_dn15))), (locals.var_xno_s_dn16 - ((locals.var_fscr_dn16 * locals.var_delxb) + (locals.var_fscr * locals.var_delxb_dn16))), (locals.var_xno_s_dn17 - ((locals.var_fscr_dn17 * locals.var_delxb) + (locals.var_fscr * locals.var_delxb_dn17))), (locals.var_xno_s_dn18 - ((locals.var_fscr_dn18 * locals.var_delxb) + (locals.var_fscr * locals.var_delxb_dn18))), (locals.var_xno_s_dn19 - ((locals.var_fscr_dn19 * locals.var_delxb) + (locals.var_fscr * locals.var_delxb_dn19))), (locals.var_xno_s_dn20 - ((locals.var_fscr_dn20 * locals.var_delxb) + (locals.var_fscr * locals.var_delxb_dn20))),)
    } else {
        (locals.var_xn_s, locals.var_xn_s_dn5, locals.var_xn_s_dn6, locals.var_xn_s_dn7, locals.var_xn_s_dn8, locals.var_xn_s_dn12, locals.var_xn_s_dn13, locals.var_xn_s_dn14, locals.var_xn_s_dn15, locals.var_xn_s_dn16, locals.var_xn_s_dn17, locals.var_xn_s_dn18, locals.var_xn_s_dn19, locals.var_xn_s_dn20,)
    }
};
        locals.var_xn_s = assign41710_e54696;
        locals.var_xn_s_dn5 = assign41710_e54696_d_n5;
        locals.var_xn_s_dn6 = assign41710_e54696_d_n6;
        locals.var_xn_s_dn7 = assign41710_e54696_d_n7;
        locals.var_xn_s_dn8 = assign41710_e54696_d_n8;
        locals.var_xn_s_dn12 = assign41710_e54696_d_n12;
        locals.var_xn_s_dn13 = assign41710_e54696_d_n13;
        locals.var_xn_s_dn14 = assign41710_e54696_d_n14;
        locals.var_xn_s_dn15 = assign41710_e54696_d_n15;
        locals.var_xn_s_dn16 = assign41710_e54696_d_n16;
        locals.var_xn_s_dn17 = assign41710_e54696_d_n17;
        locals.var_xn_s_dn18 = assign41710_e54696_d_n18;
        locals.var_xn_s_dn19 = assign41710_e54696_d_n19;
        locals.var_xn_s_dn20 = assign41710_e54696_d_n20;
        locals.var_xn_s_rv = 0.0;

        let assign41720_e54700: f64 = (locals.var_gf * 0.7071067811865475);
        let assign41720_e54701: f64 = (1.0 + assign41720_e54700);
        locals.var_xi = assign41720_e54701;
        locals.var_xi_dn5 = (locals.var_gf_dn5 * 0.7071067811865475);
        locals.var_xi_dn6 = (locals.var_gf_dn6 * 0.7071067811865475);
        locals.var_xi_dn7 = (locals.var_gf_dn7 * 0.7071067811865475);
        locals.var_xi_dn8 = (locals.var_gf_dn8 * 0.7071067811865475);
        locals.var_xi_dn12 = (locals.var_gf_dn12 * 0.7071067811865475);
        locals.var_xi_dn13 = (locals.var_gf_dn13 * 0.7071067811865475);
        locals.var_xi_dn14 = (locals.var_gf_dn14 * 0.7071067811865475);
        locals.var_xi_dn15 = (locals.var_gf_dn15 * 0.7071067811865475);
        locals.var_xi_dn16 = (locals.var_gf_dn16 * 0.7071067811865475);
        locals.var_xi_dn17 = (locals.var_gf_dn17 * 0.7071067811865475);
        locals.var_xi_dn18 = (locals.var_gf_dn18 * 0.7071067811865475);
        locals.var_xi_dn19 = (locals.var_gf_dn19 * 0.7071067811865475);
        locals.var_xi_dn20 = (locals.var_gf_dn20 * 0.7071067811865475);
        locals.var_xi_rv = 0.0;

        let assign41730_e54704: f64 = (1e-5 * locals.var_xi);
        locals.var_margin = assign41730_e54704;
        locals.var_margin_dn5 = (1e-5 * locals.var_xi_dn5);
        locals.var_margin_dn6 = (1e-5 * locals.var_xi_dn6);
        locals.var_margin_dn7 = (1e-5 * locals.var_xi_dn7);
        locals.var_margin_dn8 = (1e-5 * locals.var_xi_dn8);
        locals.var_margin_dn12 = (1e-5 * locals.var_xi_dn12);
        locals.var_margin_dn13 = (1e-5 * locals.var_xi_dn13);
        locals.var_margin_dn14 = (1e-5 * locals.var_xi_dn14);
        locals.var_margin_dn15 = (1e-5 * locals.var_xi_dn15);
        locals.var_margin_dn16 = (1e-5 * locals.var_xi_dn16);
        locals.var_margin_dn17 = (1e-5 * locals.var_xi_dn17);
        locals.var_margin_dn18 = (1e-5 * locals.var_xi_dn18);
        locals.var_margin_dn19 = (1e-5 * locals.var_xi_dn19);
        locals.var_margin_dn20 = (1e-5 * locals.var_xi_dn20);
        locals.var_margin_rv = 0.0;

        let assign41740_e54707: f64 = (1.0 / locals.var_xi);
        locals.var_inv_xi = assign41740_e54707;
        locals.var_inv_xi_dn5 = (-(locals.var_xi_dn5 / (locals.var_xi * locals.var_xi)));
        locals.var_inv_xi_dn6 = (-(locals.var_xi_dn6 / (locals.var_xi * locals.var_xi)));
        locals.var_inv_xi_dn7 = (-(locals.var_xi_dn7 / (locals.var_xi * locals.var_xi)));
        locals.var_inv_xi_dn8 = (-(locals.var_xi_dn8 / (locals.var_xi * locals.var_xi)));
        locals.var_inv_xi_dn12 = (-(locals.var_xi_dn12 / (locals.var_xi * locals.var_xi)));
        locals.var_inv_xi_dn13 = (-(locals.var_xi_dn13 / (locals.var_xi * locals.var_xi)));
        locals.var_inv_xi_dn14 = (-(locals.var_xi_dn14 / (locals.var_xi * locals.var_xi)));
        locals.var_inv_xi_dn15 = (-(locals.var_xi_dn15 / (locals.var_xi * locals.var_xi)));
        locals.var_inv_xi_dn16 = (-(locals.var_xi_dn16 / (locals.var_xi * locals.var_xi)));
        locals.var_inv_xi_dn17 = (-(locals.var_xi_dn17 / (locals.var_xi * locals.var_xi)));
        locals.var_inv_xi_dn18 = (-(locals.var_xi_dn18 / (locals.var_xi * locals.var_xi)));
        locals.var_inv_xi_dn19 = (-(locals.var_xi_dn19 / (locals.var_xi * locals.var_xi)));
        locals.var_inv_xi_dn20 = (-(locals.var_xi_dn20 / (locals.var_xi * locals.var_xi)));
        locals.var_inv_xi_rv = 0.0;

        locals.var_sp_s_x1 = 0.0;
        locals.var_sp_s_x1_dn5 = 0.0;
        locals.var_sp_s_x1_dn6 = 0.0;
        locals.var_sp_s_x1_dn7 = 0.0;
        locals.var_sp_s_x1_dn8 = 0.0;
        locals.var_sp_s_x1_dn12 = 0.0;
        locals.var_sp_s_x1_dn13 = 0.0;
        locals.var_sp_s_x1_dn14 = 0.0;
        locals.var_sp_s_x1_dn15 = 0.0;
        locals.var_sp_s_x1_dn16 = 0.0;
        locals.var_sp_s_x1_dn17 = 0.0;
        locals.var_sp_s_x1_dn18 = 0.0;
        locals.var_sp_s_x1_dn19 = 0.0;
        locals.var_sp_s_x1_dn20 = 0.0;
        locals.var_sp_s_x1_rv = 0.0;

        locals.var_x_s = 0.0;
        locals.var_x_s_dn5 = 0.0;
        locals.var_x_s_dn6 = 0.0;
        locals.var_x_s_dn7 = 0.0;
        locals.var_x_s_dn8 = 0.0;
        locals.var_x_s_dn12 = 0.0;
        locals.var_x_s_dn13 = 0.0;
        locals.var_x_s_dn14 = 0.0;
        locals.var_x_s_dn15 = 0.0;
        locals.var_x_s_dn16 = 0.0;
        locals.var_x_s_dn17 = 0.0;
        locals.var_x_s_dn18 = 0.0;
        locals.var_x_s_dn19 = 0.0;
        locals.var_x_s_dn20 = 0.0;
        locals.var_x_s_rv = 0.0;

        let assign41770_e54712: f64 = if locals.var_xn_s < 460.51701859880916 { 1.0 } else { 0.0 };
        locals.var_guard1283 = assign41770_e54712;
        locals.var_guard1283_rv = 0.0;

        let (assign41780_e54718, assign41780_e54718_d_n5, assign41780_e54718_d_n6, assign41780_e54718_d_n7, assign41780_e54718_d_n8, assign41780_e54718_d_n12, assign41780_e54718_d_n13, assign41780_e54718_d_n14, assign41780_e54718_d_n15, assign41780_e54718_d_n16, assign41780_e54718_d_n17, assign41780_e54718_d_n18, assign41780_e54718_d_n19, assign41780_e54718_d_n20,) = {
    if (locals.var_guard1283 != 0.0) {
        let assign41780_e54715: f64 = (-locals.var_xn_s);
        let assign41780_e54716: f64 = (assign41780_e54715).exp();
        (assign41780_e54716, (assign41780_e54716 * (-locals.var_xn_s_dn5)), (assign41780_e54716 * (-locals.var_xn_s_dn6)), (assign41780_e54716 * (-locals.var_xn_s_dn7)), (assign41780_e54716 * (-locals.var_xn_s_dn8)), (assign41780_e54716 * (-locals.var_xn_s_dn12)), (assign41780_e54716 * (-locals.var_xn_s_dn13)), (assign41780_e54716 * (-locals.var_xn_s_dn14)), (assign41780_e54716 * (-locals.var_xn_s_dn15)), (assign41780_e54716 * (-locals.var_xn_s_dn16)), (assign41780_e54716 * (-locals.var_xn_s_dn17)), (assign41780_e54716 * (-locals.var_xn_s_dn18)), (assign41780_e54716 * (-locals.var_xn_s_dn19)), (assign41780_e54716 * (-locals.var_xn_s_dn20)),)
    } else {
        (locals.var_delta_ns, locals.var_delta_ns_dn5, locals.var_delta_ns_dn6, locals.var_delta_ns_dn7, locals.var_delta_ns_dn8, locals.var_delta_ns_dn12, locals.var_delta_ns_dn13, locals.var_delta_ns_dn14, locals.var_delta_ns_dn15, locals.var_delta_ns_dn16, locals.var_delta_ns_dn17, locals.var_delta_ns_dn18, locals.var_delta_ns_dn19, locals.var_delta_ns_dn20,)
    }
};
        locals.var_delta_ns = assign41780_e54718;
        locals.var_delta_ns_dn5 = assign41780_e54718_d_n5;
        locals.var_delta_ns_dn6 = assign41780_e54718_d_n6;
        locals.var_delta_ns_dn7 = assign41780_e54718_d_n7;
        locals.var_delta_ns_dn8 = assign41780_e54718_d_n8;
        locals.var_delta_ns_dn12 = assign41780_e54718_d_n12;
        locals.var_delta_ns_dn13 = assign41780_e54718_d_n13;
        locals.var_delta_ns_dn14 = assign41780_e54718_d_n14;
        locals.var_delta_ns_dn15 = assign41780_e54718_d_n15;
        locals.var_delta_ns_dn16 = assign41780_e54718_d_n16;
        locals.var_delta_ns_dn17 = assign41780_e54718_d_n17;
        locals.var_delta_ns_dn18 = assign41780_e54718_d_n18;
        locals.var_delta_ns_dn19 = assign41780_e54718_d_n19;
        locals.var_delta_ns_dn20 = assign41780_e54718_d_n20;
        locals.var_delta_ns_rv = 0.0;

        let (assign41790_e54745, assign41790_e54745_d_n5, assign41790_e54745_d_n6, assign41790_e54745_d_n7, assign41790_e54745_d_n8, assign41790_e54745_d_n12, assign41790_e54745_d_n13, assign41790_e54745_d_n14, assign41790_e54745_d_n15, assign41790_e54745_d_n16, assign41790_e54745_d_n17, assign41790_e54745_d_n18, assign41790_e54745_d_n19, assign41790_e54745_d_n20,) = {
    if (locals.var_guard1283 == 0.0) {
        let assign41790_e54725: f64 = (locals.var_xn_s - 460.51701859880916);
        let assign41790_e54730: f64 = (locals.var_xn_s - 460.51701859880916);
        let assign41790_e54734: f64 = (locals.var_xn_s - 460.51701859880916);
        let assign41790_e54736: f64 = (assign41790_e54734 * 0.3333333333333333);
        let assign41790_e54737: f64 = (1.0 + assign41790_e54736);
        let assign41790_e54738: f64 = (assign41790_e54730 * assign41790_e54737);
        let assign41790_e54739: f64 = (0.5 * assign41790_e54738);
        let assign41790_e54740: f64 = (1.0 + assign41790_e54739);
        let assign41790_e54741: f64 = (assign41790_e54725 * assign41790_e54740);
        let assign41790_e54742: f64 = (1.0 + assign41790_e54741);
        let assign41790_e54743: f64 = (1e-200 / assign41790_e54742);
        (assign41790_e54743, (-((1e-200 * ((locals.var_xn_s_dn5 * assign41790_e54740) + (assign41790_e54725 * (0.5 * ((locals.var_xn_s_dn5 * assign41790_e54737) + (assign41790_e54730 * (locals.var_xn_s_dn5 * 0.3333333333333333))))))) / (assign41790_e54742 * assign41790_e54742))), (-((1e-200 * ((locals.var_xn_s_dn6 * assign41790_e54740) + (assign41790_e54725 * (0.5 * ((locals.var_xn_s_dn6 * assign41790_e54737) + (assign41790_e54730 * (locals.var_xn_s_dn6 * 0.3333333333333333))))))) / (assign41790_e54742 * assign41790_e54742))), (-((1e-200 * ((locals.var_xn_s_dn7 * assign41790_e54740) + (assign41790_e54725 * (0.5 * ((locals.var_xn_s_dn7 * assign41790_e54737) + (assign41790_e54730 * (locals.var_xn_s_dn7 * 0.3333333333333333))))))) / (assign41790_e54742 * assign41790_e54742))), (-((1e-200 * ((locals.var_xn_s_dn8 * assign41790_e54740) + (assign41790_e54725 * (0.5 * ((locals.var_xn_s_dn8 * assign41790_e54737) + (assign41790_e54730 * (locals.var_xn_s_dn8 * 0.3333333333333333))))))) / (assign41790_e54742 * assign41790_e54742))), (-((1e-200 * ((locals.var_xn_s_dn12 * assign41790_e54740) + (assign41790_e54725 * (0.5 * ((locals.var_xn_s_dn12 * assign41790_e54737) + (assign41790_e54730 * (locals.var_xn_s_dn12 * 0.3333333333333333))))))) / (assign41790_e54742 * assign41790_e54742))), (-((1e-200 * ((locals.var_xn_s_dn13 * assign41790_e54740) + (assign41790_e54725 * (0.5 * ((locals.var_xn_s_dn13 * assign41790_e54737) + (assign41790_e54730 * (locals.var_xn_s_dn13 * 0.3333333333333333))))))) / (assign41790_e54742 * assign41790_e54742))), (-((1e-200 * ((locals.var_xn_s_dn14 * assign41790_e54740) + (assign41790_e54725 * (0.5 * ((locals.var_xn_s_dn14 * assign41790_e54737) + (assign41790_e54730 * (locals.var_xn_s_dn14 * 0.3333333333333333))))))) / (assign41790_e54742 * assign41790_e54742))), (-((1e-200 * ((locals.var_xn_s_dn15 * assign41790_e54740) + (assign41790_e54725 * (0.5 * ((locals.var_xn_s_dn15 * assign41790_e54737) + (assign41790_e54730 * (locals.var_xn_s_dn15 * 0.3333333333333333))))))) / (assign41790_e54742 * assign41790_e54742))), (-((1e-200 * ((locals.var_xn_s_dn16 * assign41790_e54740) + (assign41790_e54725 * (0.5 * ((locals.var_xn_s_dn16 * assign41790_e54737) + (assign41790_e54730 * (locals.var_xn_s_dn16 * 0.3333333333333333))))))) / (assign41790_e54742 * assign41790_e54742))), (-((1e-200 * ((locals.var_xn_s_dn17 * assign41790_e54740) + (assign41790_e54725 * (0.5 * ((locals.var_xn_s_dn17 * assign41790_e54737) + (assign41790_e54730 * (locals.var_xn_s_dn17 * 0.3333333333333333))))))) / (assign41790_e54742 * assign41790_e54742))), (-((1e-200 * ((locals.var_xn_s_dn18 * assign41790_e54740) + (assign41790_e54725 * (0.5 * ((locals.var_xn_s_dn18 * assign41790_e54737) + (assign41790_e54730 * (locals.var_xn_s_dn18 * 0.3333333333333333))))))) / (assign41790_e54742 * assign41790_e54742))), (-((1e-200 * ((locals.var_xn_s_dn19 * assign41790_e54740) + (assign41790_e54725 * (0.5 * ((locals.var_xn_s_dn19 * assign41790_e54737) + (assign41790_e54730 * (locals.var_xn_s_dn19 * 0.3333333333333333))))))) / (assign41790_e54742 * assign41790_e54742))), (-((1e-200 * ((locals.var_xn_s_dn20 * assign41790_e54740) + (assign41790_e54725 * (0.5 * ((locals.var_xn_s_dn20 * assign41790_e54737) + (assign41790_e54730 * (locals.var_xn_s_dn20 * 0.3333333333333333))))))) / (assign41790_e54742 * assign41790_e54742))),)
    } else {
        (locals.var_delta_ns, locals.var_delta_ns_dn5, locals.var_delta_ns_dn6, locals.var_delta_ns_dn7, locals.var_delta_ns_dn8, locals.var_delta_ns_dn12, locals.var_delta_ns_dn13, locals.var_delta_ns_dn14, locals.var_delta_ns_dn15, locals.var_delta_ns_dn16, locals.var_delta_ns_dn17, locals.var_delta_ns_dn18, locals.var_delta_ns_dn19, locals.var_delta_ns_dn20,)
    }
};
        locals.var_delta_ns = assign41790_e54745;
        locals.var_delta_ns_dn5 = assign41790_e54745_d_n5;
        locals.var_delta_ns_dn6 = assign41790_e54745_d_n6;
        locals.var_delta_ns_dn7 = assign41790_e54745_d_n7;
        locals.var_delta_ns_dn8 = assign41790_e54745_d_n8;
        locals.var_delta_ns_dn12 = assign41790_e54745_d_n12;
        locals.var_delta_ns_dn13 = assign41790_e54745_d_n13;
        locals.var_delta_ns_dn14 = assign41790_e54745_d_n14;
        locals.var_delta_ns_dn15 = assign41790_e54745_d_n15;
        locals.var_delta_ns_dn16 = assign41790_e54745_d_n16;
        locals.var_delta_ns_dn17 = assign41790_e54745_d_n17;
        locals.var_delta_ns_dn18 = assign41790_e54745_d_n18;
        locals.var_delta_ns_dn19 = assign41790_e54745_d_n19;
        locals.var_delta_ns_dn20 = assign41790_e54745_d_n20;
        locals.var_delta_ns_rv = 0.0;

        let assign41800_e54747: f64 = (locals.var_xg).abs();
        let assign41800_e54749: f64 = if assign41800_e54747 <= locals.var_margin { 1.0 } else { 0.0 };
        locals.var_guard1284 = assign41800_e54749;
        locals.var_guard1284_rv = 0.0;

        let (assign41810_e54759, assign41810_e54759_d_n5, assign41810_e54759_d_n6, assign41810_e54759_d_n7, assign41810_e54759_d_n8, assign41810_e54759_d_n12, assign41810_e54759_d_n13, assign41810_e54759_d_n14, assign41810_e54759_d_n15, assign41810_e54759_d_n16, assign41810_e54759_d_n17, assign41810_e54759_d_n18, assign41810_e54759_d_n19, assign41810_e54759_d_n20,) = {
    if (locals.var_guard1284 != 0.0) {
        let assign41810_e54753: f64 = (locals.var_inv_xi * locals.var_inv_xi);
        let assign41810_e54755: f64 = (assign41810_e54753 * 0.16666666666666666);
        let assign41810_e54757: f64 = (assign41810_e54755 * 0.7071067811865475);
        (assign41810_e54757, ((((locals.var_inv_xi_dn5 * locals.var_inv_xi) + (locals.var_inv_xi * locals.var_inv_xi_dn5)) * 0.16666666666666666) * 0.7071067811865475), ((((locals.var_inv_xi_dn6 * locals.var_inv_xi) + (locals.var_inv_xi * locals.var_inv_xi_dn6)) * 0.16666666666666666) * 0.7071067811865475), ((((locals.var_inv_xi_dn7 * locals.var_inv_xi) + (locals.var_inv_xi * locals.var_inv_xi_dn7)) * 0.16666666666666666) * 0.7071067811865475), ((((locals.var_inv_xi_dn8 * locals.var_inv_xi) + (locals.var_inv_xi * locals.var_inv_xi_dn8)) * 0.16666666666666666) * 0.7071067811865475), ((((locals.var_inv_xi_dn12 * locals.var_inv_xi) + (locals.var_inv_xi * locals.var_inv_xi_dn12)) * 0.16666666666666666) * 0.7071067811865475), ((((locals.var_inv_xi_dn13 * locals.var_inv_xi) + (locals.var_inv_xi * locals.var_inv_xi_dn13)) * 0.16666666666666666) * 0.7071067811865475), ((((locals.var_inv_xi_dn14 * locals.var_inv_xi) + (locals.var_inv_xi * locals.var_inv_xi_dn14)) * 0.16666666666666666) * 0.7071067811865475), ((((locals.var_inv_xi_dn15 * locals.var_inv_xi) + (locals.var_inv_xi * locals.var_inv_xi_dn15)) * 0.16666666666666666) * 0.7071067811865475), ((((locals.var_inv_xi_dn16 * locals.var_inv_xi) + (locals.var_inv_xi * locals.var_inv_xi_dn16)) * 0.16666666666666666) * 0.7071067811865475), ((((locals.var_inv_xi_dn17 * locals.var_inv_xi) + (locals.var_inv_xi * locals.var_inv_xi_dn17)) * 0.16666666666666666) * 0.7071067811865475), ((((locals.var_inv_xi_dn18 * locals.var_inv_xi) + (locals.var_inv_xi * locals.var_inv_xi_dn18)) * 0.16666666666666666) * 0.7071067811865475), ((((locals.var_inv_xi_dn19 * locals.var_inv_xi) + (locals.var_inv_xi * locals.var_inv_xi_dn19)) * 0.16666666666666666) * 0.7071067811865475), ((((locals.var_inv_xi_dn20 * locals.var_inv_xi) + (locals.var_inv_xi * locals.var_inv_xi_dn20)) * 0.16666666666666666) * 0.7071067811865475),)
    } else {
        (locals.var_sp_s_temp1, locals.var_sp_s_temp1_dn5, locals.var_sp_s_temp1_dn6, locals.var_sp_s_temp1_dn7, locals.var_sp_s_temp1_dn8, locals.var_sp_s_temp1_dn12, locals.var_sp_s_temp1_dn13, locals.var_sp_s_temp1_dn14, locals.var_sp_s_temp1_dn15, locals.var_sp_s_temp1_dn16, locals.var_sp_s_temp1_dn17, locals.var_sp_s_temp1_dn18, locals.var_sp_s_temp1_dn19, locals.var_sp_s_temp1_dn20,)
    }
};
        locals.var_sp_s_temp1 = assign41810_e54759;
        locals.var_sp_s_temp1_dn5 = assign41810_e54759_d_n5;
        locals.var_sp_s_temp1_dn6 = assign41810_e54759_d_n6;
        locals.var_sp_s_temp1_dn7 = assign41810_e54759_d_n7;
        locals.var_sp_s_temp1_dn8 = assign41810_e54759_d_n8;
        locals.var_sp_s_temp1_dn12 = assign41810_e54759_d_n12;
        locals.var_sp_s_temp1_dn13 = assign41810_e54759_d_n13;
        locals.var_sp_s_temp1_dn14 = assign41810_e54759_d_n14;
        locals.var_sp_s_temp1_dn15 = assign41810_e54759_d_n15;
        locals.var_sp_s_temp1_dn16 = assign41810_e54759_d_n16;
        locals.var_sp_s_temp1_dn17 = assign41810_e54759_d_n17;
        locals.var_sp_s_temp1_dn18 = assign41810_e54759_d_n18;
        locals.var_sp_s_temp1_dn19 = assign41810_e54759_d_n19;
        locals.var_sp_s_temp1_dn20 = assign41810_e54759_d_n20;
        locals.var_sp_s_temp1_rv = 0.0;

        let (assign41820_e54777, assign41820_e54777_d_n5, assign41820_e54777_d_n6, assign41820_e54777_d_n7, assign41820_e54777_d_n8, assign41820_e54777_d_n12, assign41820_e54777_d_n13, assign41820_e54777_d_n14, assign41820_e54777_d_n15, assign41820_e54777_d_n16, assign41820_e54777_d_n17, assign41820_e54777_d_n18, assign41820_e54777_d_n19, assign41820_e54777_d_n20,) = {
    if (locals.var_guard1284 != 0.0) {
        let assign41820_e54763: f64 = (locals.var_xg * locals.var_inv_xi);
        let assign41820_e54768: f64 = (1.0 - locals.var_delta_ns);
        let assign41820_e54769: f64 = (locals.var_xg * assign41820_e54768);
        let assign41820_e54771: f64 = (assign41820_e54769 * locals.var_gf);
        let assign41820_e54773: f64 = (assign41820_e54771 * locals.var_sp_s_temp1);
        let assign41820_e54774: f64 = (1.0 + assign41820_e54773);
        let assign41820_e54775: f64 = (assign41820_e54763 * assign41820_e54774);
        (assign41820_e54775, ((((locals.var_xg_dn5 * locals.var_inv_xi) + (locals.var_xg * locals.var_inv_xi_dn5)) * assign41820_e54774) + (assign41820_e54763 * ((((((locals.var_xg_dn5 * assign41820_e54768) + (locals.var_xg * (-locals.var_delta_ns_dn5))) * locals.var_gf) + (assign41820_e54769 * locals.var_gf_dn5)) * locals.var_sp_s_temp1) + (assign41820_e54771 * locals.var_sp_s_temp1_dn5)))), ((((locals.var_xg_dn6 * locals.var_inv_xi) + (locals.var_xg * locals.var_inv_xi_dn6)) * assign41820_e54774) + (assign41820_e54763 * ((((((locals.var_xg_dn6 * assign41820_e54768) + (locals.var_xg * (-locals.var_delta_ns_dn6))) * locals.var_gf) + (assign41820_e54769 * locals.var_gf_dn6)) * locals.var_sp_s_temp1) + (assign41820_e54771 * locals.var_sp_s_temp1_dn6)))), ((((locals.var_xg_dn7 * locals.var_inv_xi) + (locals.var_xg * locals.var_inv_xi_dn7)) * assign41820_e54774) + (assign41820_e54763 * ((((((locals.var_xg_dn7 * assign41820_e54768) + (locals.var_xg * (-locals.var_delta_ns_dn7))) * locals.var_gf) + (assign41820_e54769 * locals.var_gf_dn7)) * locals.var_sp_s_temp1) + (assign41820_e54771 * locals.var_sp_s_temp1_dn7)))), ((((locals.var_xg_dn8 * locals.var_inv_xi) + (locals.var_xg * locals.var_inv_xi_dn8)) * assign41820_e54774) + (assign41820_e54763 * ((((((locals.var_xg_dn8 * assign41820_e54768) + (locals.var_xg * (-locals.var_delta_ns_dn8))) * locals.var_gf) + (assign41820_e54769 * locals.var_gf_dn8)) * locals.var_sp_s_temp1) + (assign41820_e54771 * locals.var_sp_s_temp1_dn8)))), ((((locals.var_xg_dn12 * locals.var_inv_xi) + (locals.var_xg * locals.var_inv_xi_dn12)) * assign41820_e54774) + (assign41820_e54763 * ((((((locals.var_xg_dn12 * assign41820_e54768) + (locals.var_xg * (-locals.var_delta_ns_dn12))) * locals.var_gf) + (assign41820_e54769 * locals.var_gf_dn12)) * locals.var_sp_s_temp1) + (assign41820_e54771 * locals.var_sp_s_temp1_dn12)))), ((((locals.var_xg_dn13 * locals.var_inv_xi) + (locals.var_xg * locals.var_inv_xi_dn13)) * assign41820_e54774) + (assign41820_e54763 * ((((((locals.var_xg_dn13 * assign41820_e54768) + (locals.var_xg * (-locals.var_delta_ns_dn13))) * locals.var_gf) + (assign41820_e54769 * locals.var_gf_dn13)) * locals.var_sp_s_temp1) + (assign41820_e54771 * locals.var_sp_s_temp1_dn13)))), ((((locals.var_xg_dn14 * locals.var_inv_xi) + (locals.var_xg * locals.var_inv_xi_dn14)) * assign41820_e54774) + (assign41820_e54763 * ((((((locals.var_xg_dn14 * assign41820_e54768) + (locals.var_xg * (-locals.var_delta_ns_dn14))) * locals.var_gf) + (assign41820_e54769 * locals.var_gf_dn14)) * locals.var_sp_s_temp1) + (assign41820_e54771 * locals.var_sp_s_temp1_dn14)))), ((((locals.var_xg_dn15 * locals.var_inv_xi) + (locals.var_xg * locals.var_inv_xi_dn15)) * assign41820_e54774) + (assign41820_e54763 * ((((((locals.var_xg_dn15 * assign41820_e54768) + (locals.var_xg * (-locals.var_delta_ns_dn15))) * locals.var_gf) + (assign41820_e54769 * locals.var_gf_dn15)) * locals.var_sp_s_temp1) + (assign41820_e54771 * locals.var_sp_s_temp1_dn15)))), ((((locals.var_xg_dn16 * locals.var_inv_xi) + (locals.var_xg * locals.var_inv_xi_dn16)) * assign41820_e54774) + (assign41820_e54763 * ((((((locals.var_xg_dn16 * assign41820_e54768) + (locals.var_xg * (-locals.var_delta_ns_dn16))) * locals.var_gf) + (assign41820_e54769 * locals.var_gf_dn16)) * locals.var_sp_s_temp1) + (assign41820_e54771 * locals.var_sp_s_temp1_dn16)))), ((((locals.var_xg_dn17 * locals.var_inv_xi) + (locals.var_xg * locals.var_inv_xi_dn17)) * assign41820_e54774) + (assign41820_e54763 * ((((((locals.var_xg_dn17 * assign41820_e54768) + (locals.var_xg * (-locals.var_delta_ns_dn17))) * locals.var_gf) + (assign41820_e54769 * locals.var_gf_dn17)) * locals.var_sp_s_temp1) + (assign41820_e54771 * locals.var_sp_s_temp1_dn17)))), ((((locals.var_xg_dn18 * locals.var_inv_xi) + (locals.var_xg * locals.var_inv_xi_dn18)) * assign41820_e54774) + (assign41820_e54763 * ((((((locals.var_xg_dn18 * assign41820_e54768) + (locals.var_xg * (-locals.var_delta_ns_dn18))) * locals.var_gf) + (assign41820_e54769 * locals.var_gf_dn18)) * locals.var_sp_s_temp1) + (assign41820_e54771 * locals.var_sp_s_temp1_dn18)))), ((((locals.var_xg_dn19 * locals.var_inv_xi) + (locals.var_xg * locals.var_inv_xi_dn19)) * assign41820_e54774) + (assign41820_e54763 * ((((((locals.var_xg_dn19 * assign41820_e54768) + (locals.var_xg * (-locals.var_delta_ns_dn19))) * locals.var_gf) + (assign41820_e54769 * locals.var_gf_dn19)) * locals.var_sp_s_temp1) + (assign41820_e54771 * locals.var_sp_s_temp1_dn19)))), ((((locals.var_xg_dn20 * locals.var_inv_xi) + (locals.var_xg * locals.var_inv_xi_dn20)) * assign41820_e54774) + (assign41820_e54763 * ((((((locals.var_xg_dn20 * assign41820_e54768) + (locals.var_xg * (-locals.var_delta_ns_dn20))) * locals.var_gf) + (assign41820_e54769 * locals.var_gf_dn20)) * locals.var_sp_s_temp1) + (assign41820_e54771 * locals.var_sp_s_temp1_dn20)))),)
    } else {
        (locals.var_x_s, locals.var_x_s_dn5, locals.var_x_s_dn6, locals.var_x_s_dn7, locals.var_x_s_dn8, locals.var_x_s_dn12, locals.var_x_s_dn13, locals.var_x_s_dn14, locals.var_x_s_dn15, locals.var_x_s_dn16, locals.var_x_s_dn17, locals.var_x_s_dn18, locals.var_x_s_dn19, locals.var_x_s_dn20,)
    }
};
        locals.var_x_s = assign41820_e54777;
        locals.var_x_s_dn5 = assign41820_e54777_d_n5;
        locals.var_x_s_dn6 = assign41820_e54777_d_n6;
        locals.var_x_s_dn7 = assign41820_e54777_d_n7;
        locals.var_x_s_dn8 = assign41820_e54777_d_n8;
        locals.var_x_s_dn12 = assign41820_e54777_d_n12;
        locals.var_x_s_dn13 = assign41820_e54777_d_n13;
        locals.var_x_s_dn14 = assign41820_e54777_d_n14;
        locals.var_x_s_dn15 = assign41820_e54777_d_n15;
        locals.var_x_s_dn16 = assign41820_e54777_d_n16;
        locals.var_x_s_dn17 = assign41820_e54777_d_n17;
        locals.var_x_s_dn18 = assign41820_e54777_d_n18;
        locals.var_x_s_dn19 = assign41820_e54777_d_n19;
        locals.var_x_s_dn20 = assign41820_e54777_d_n20;
        locals.var_x_s_rv = 0.0;

        let assign41830_e54780: f64 = (-locals.var_margin);
        let assign41830_e54781: f64 = if locals.var_xg < assign41830_e54780 { 1.0 } else { 0.0 };
        locals.var_guard1285 = assign41830_e54781;
        locals.var_guard1285_rv = 0.0;

        let (assign41840_e54789, assign41840_e54789_d_n5, assign41840_e54789_d_n6, assign41840_e54789_d_n7, assign41840_e54789_d_n8, assign41840_e54789_d_n12, assign41840_e54789_d_n13, assign41840_e54789_d_n14, assign41840_e54789_d_n15, assign41840_e54789_d_n16, assign41840_e54789_d_n17, assign41840_e54789_d_n18, assign41840_e54789_d_n19, assign41840_e54789_d_n20,) = {
    if ((locals.var_guard1284 == 0.0) && (locals.var_guard1285 != 0.0)) {
        let assign41840_e54787: f64 = (-locals.var_xg);
        (assign41840_e54787, (-locals.var_xg_dn5), (-locals.var_xg_dn6), (-locals.var_xg_dn7), (-locals.var_xg_dn8), (-locals.var_xg_dn12), (-locals.var_xg_dn13), (-locals.var_xg_dn14), (-locals.var_xg_dn15), (-locals.var_xg_dn16), (-locals.var_xg_dn17), (-locals.var_xg_dn18), (-locals.var_xg_dn19), (-locals.var_xg_dn20),)
    } else {
        (locals.var_sp_s_yg, locals.var_sp_s_yg_dn5, locals.var_sp_s_yg_dn6, locals.var_sp_s_yg_dn7, locals.var_sp_s_yg_dn8, locals.var_sp_s_yg_dn12, locals.var_sp_s_yg_dn13, locals.var_sp_s_yg_dn14, locals.var_sp_s_yg_dn15, locals.var_sp_s_yg_dn16, locals.var_sp_s_yg_dn17, locals.var_sp_s_yg_dn18, locals.var_sp_s_yg_dn19, locals.var_sp_s_yg_dn20,)
    }
};
        locals.var_sp_s_yg = assign41840_e54789;
        locals.var_sp_s_yg_dn5 = assign41840_e54789_d_n5;
        locals.var_sp_s_yg_dn6 = assign41840_e54789_d_n6;
        locals.var_sp_s_yg_dn7 = assign41840_e54789_d_n7;
        locals.var_sp_s_yg_dn8 = assign41840_e54789_d_n8;
        locals.var_sp_s_yg_dn12 = assign41840_e54789_d_n12;
        locals.var_sp_s_yg_dn13 = assign41840_e54789_d_n13;
        locals.var_sp_s_yg_dn14 = assign41840_e54789_d_n14;
        locals.var_sp_s_yg_dn15 = assign41840_e54789_d_n15;
        locals.var_sp_s_yg_dn16 = assign41840_e54789_d_n16;
        locals.var_sp_s_yg_dn17 = assign41840_e54789_d_n17;
        locals.var_sp_s_yg_dn18 = assign41840_e54789_d_n18;
        locals.var_sp_s_yg_dn19 = assign41840_e54789_d_n19;
        locals.var_sp_s_yg_dn20 = assign41840_e54789_d_n20;
        locals.var_sp_s_yg_rv = 0.0;

        let (assign41850_e54800, assign41850_e54800_d_n5, assign41850_e54800_d_n6, assign41850_e54800_d_n7, assign41850_e54800_d_n8, assign41850_e54800_d_n12, assign41850_e54800_d_n13, assign41850_e54800_d_n14, assign41850_e54800_d_n15, assign41850_e54800_d_n16, assign41850_e54800_d_n17, assign41850_e54800_d_n18, assign41850_e54800_d_n19, assign41850_e54800_d_n20,) = {
    if ((locals.var_guard1284 == 0.0) && (locals.var_guard1285 != 0.0)) {
        let assign41850_e54797: f64 = (locals.var_sp_s_yg * locals.var_inv_xi);
        let assign41850_e54798: f64 = (1.25 * assign41850_e54797);
        (assign41850_e54798, (1.25 * ((locals.var_sp_s_yg_dn5 * locals.var_inv_xi) + (locals.var_sp_s_yg * locals.var_inv_xi_dn5))), (1.25 * ((locals.var_sp_s_yg_dn6 * locals.var_inv_xi) + (locals.var_sp_s_yg * locals.var_inv_xi_dn6))), (1.25 * ((locals.var_sp_s_yg_dn7 * locals.var_inv_xi) + (locals.var_sp_s_yg * locals.var_inv_xi_dn7))), (1.25 * ((locals.var_sp_s_yg_dn8 * locals.var_inv_xi) + (locals.var_sp_s_yg * locals.var_inv_xi_dn8))), (1.25 * ((locals.var_sp_s_yg_dn12 * locals.var_inv_xi) + (locals.var_sp_s_yg * locals.var_inv_xi_dn12))), (1.25 * ((locals.var_sp_s_yg_dn13 * locals.var_inv_xi) + (locals.var_sp_s_yg * locals.var_inv_xi_dn13))), (1.25 * ((locals.var_sp_s_yg_dn14 * locals.var_inv_xi) + (locals.var_sp_s_yg * locals.var_inv_xi_dn14))), (1.25 * ((locals.var_sp_s_yg_dn15 * locals.var_inv_xi) + (locals.var_sp_s_yg * locals.var_inv_xi_dn15))), (1.25 * ((locals.var_sp_s_yg_dn16 * locals.var_inv_xi) + (locals.var_sp_s_yg * locals.var_inv_xi_dn16))), (1.25 * ((locals.var_sp_s_yg_dn17 * locals.var_inv_xi) + (locals.var_sp_s_yg * locals.var_inv_xi_dn17))), (1.25 * ((locals.var_sp_s_yg_dn18 * locals.var_inv_xi) + (locals.var_sp_s_yg * locals.var_inv_xi_dn18))), (1.25 * ((locals.var_sp_s_yg_dn19 * locals.var_inv_xi) + (locals.var_sp_s_yg * locals.var_inv_xi_dn19))), (1.25 * ((locals.var_sp_s_yg_dn20 * locals.var_inv_xi) + (locals.var_sp_s_yg * locals.var_inv_xi_dn20))),)
    } else {
        (locals.var_sp_s_ysub, locals.var_sp_s_ysub_dn5, locals.var_sp_s_ysub_dn6, locals.var_sp_s_ysub_dn7, locals.var_sp_s_ysub_dn8, locals.var_sp_s_ysub_dn12, locals.var_sp_s_ysub_dn13, locals.var_sp_s_ysub_dn14, locals.var_sp_s_ysub_dn15, locals.var_sp_s_ysub_dn16, locals.var_sp_s_ysub_dn17, locals.var_sp_s_ysub_dn18, locals.var_sp_s_ysub_dn19, locals.var_sp_s_ysub_dn20,)
    }
};
        locals.var_sp_s_ysub = assign41850_e54800;
        locals.var_sp_s_ysub_dn5 = assign41850_e54800_d_n5;
        locals.var_sp_s_ysub_dn6 = assign41850_e54800_d_n6;
        locals.var_sp_s_ysub_dn7 = assign41850_e54800_d_n7;
        locals.var_sp_s_ysub_dn8 = assign41850_e54800_d_n8;
        locals.var_sp_s_ysub_dn12 = assign41850_e54800_d_n12;
        locals.var_sp_s_ysub_dn13 = assign41850_e54800_d_n13;
        locals.var_sp_s_ysub_dn14 = assign41850_e54800_d_n14;
        locals.var_sp_s_ysub_dn15 = assign41850_e54800_d_n15;
        locals.var_sp_s_ysub_dn16 = assign41850_e54800_d_n16;
        locals.var_sp_s_ysub_dn17 = assign41850_e54800_d_n17;
        locals.var_sp_s_ysub_dn18 = assign41850_e54800_d_n18;
        locals.var_sp_s_ysub_dn19 = assign41850_e54800_d_n19;
        locals.var_sp_s_ysub_dn20 = assign41850_e54800_d_n20;
        locals.var_sp_s_ysub_rv = 0.0;

        let (assign41860_e54822, assign41860_e54822_d_n5, assign41860_e54822_d_n6, assign41860_e54822_d_n7, assign41860_e54822_d_n8, assign41860_e54822_d_n12, assign41860_e54822_d_n13, assign41860_e54822_d_n14, assign41860_e54822_d_n15, assign41860_e54822_d_n16, assign41860_e54822_d_n17, assign41860_e54822_d_n18, assign41860_e54822_d_n19, assign41860_e54822_d_n20,) = {
    if ((locals.var_guard1284 == 0.0) && (locals.var_guard1285 != 0.0)) {
        let assign41860_e54808: f64 = (locals.var_sp_s_ysub + 10.0);
        let assign41860_e54811: f64 = (locals.var_sp_s_ysub - 6.0);
        let assign41860_e54814: f64 = (locals.var_sp_s_ysub - 6.0);
        let assign41860_e54815: f64 = (assign41860_e54811 * assign41860_e54814);
        let assign41860_e54817: f64 = (assign41860_e54815 + 64.0);
        let assign41860_e54818: f64 = (assign41860_e54817).sqrt();
        let assign41860_e54819: f64 = (assign41860_e54808 - assign41860_e54818);
        let assign41860_e54820: f64 = (0.5 * assign41860_e54819);
        (assign41860_e54820, (0.5 * (locals.var_sp_s_ysub_dn5 - (((locals.var_sp_s_ysub_dn5 * assign41860_e54814) + (assign41860_e54811 * locals.var_sp_s_ysub_dn5)) / (2.0 * assign41860_e54818)))), (0.5 * (locals.var_sp_s_ysub_dn6 - (((locals.var_sp_s_ysub_dn6 * assign41860_e54814) + (assign41860_e54811 * locals.var_sp_s_ysub_dn6)) / (2.0 * assign41860_e54818)))), (0.5 * (locals.var_sp_s_ysub_dn7 - (((locals.var_sp_s_ysub_dn7 * assign41860_e54814) + (assign41860_e54811 * locals.var_sp_s_ysub_dn7)) / (2.0 * assign41860_e54818)))), (0.5 * (locals.var_sp_s_ysub_dn8 - (((locals.var_sp_s_ysub_dn8 * assign41860_e54814) + (assign41860_e54811 * locals.var_sp_s_ysub_dn8)) / (2.0 * assign41860_e54818)))), (0.5 * (locals.var_sp_s_ysub_dn12 - (((locals.var_sp_s_ysub_dn12 * assign41860_e54814) + (assign41860_e54811 * locals.var_sp_s_ysub_dn12)) / (2.0 * assign41860_e54818)))), (0.5 * (locals.var_sp_s_ysub_dn13 - (((locals.var_sp_s_ysub_dn13 * assign41860_e54814) + (assign41860_e54811 * locals.var_sp_s_ysub_dn13)) / (2.0 * assign41860_e54818)))), (0.5 * (locals.var_sp_s_ysub_dn14 - (((locals.var_sp_s_ysub_dn14 * assign41860_e54814) + (assign41860_e54811 * locals.var_sp_s_ysub_dn14)) / (2.0 * assign41860_e54818)))), (0.5 * (locals.var_sp_s_ysub_dn15 - (((locals.var_sp_s_ysub_dn15 * assign41860_e54814) + (assign41860_e54811 * locals.var_sp_s_ysub_dn15)) / (2.0 * assign41860_e54818)))), (0.5 * (locals.var_sp_s_ysub_dn16 - (((locals.var_sp_s_ysub_dn16 * assign41860_e54814) + (assign41860_e54811 * locals.var_sp_s_ysub_dn16)) / (2.0 * assign41860_e54818)))), (0.5 * (locals.var_sp_s_ysub_dn17 - (((locals.var_sp_s_ysub_dn17 * assign41860_e54814) + (assign41860_e54811 * locals.var_sp_s_ysub_dn17)) / (2.0 * assign41860_e54818)))), (0.5 * (locals.var_sp_s_ysub_dn18 - (((locals.var_sp_s_ysub_dn18 * assign41860_e54814) + (assign41860_e54811 * locals.var_sp_s_ysub_dn18)) / (2.0 * assign41860_e54818)))), (0.5 * (locals.var_sp_s_ysub_dn19 - (((locals.var_sp_s_ysub_dn19 * assign41860_e54814) + (assign41860_e54811 * locals.var_sp_s_ysub_dn19)) / (2.0 * assign41860_e54818)))), (0.5 * (locals.var_sp_s_ysub_dn20 - (((locals.var_sp_s_ysub_dn20 * assign41860_e54814) + (assign41860_e54811 * locals.var_sp_s_ysub_dn20)) / (2.0 * assign41860_e54818)))),)
    } else {
        (locals.var_sp_s_eta, locals.var_sp_s_eta_dn5, locals.var_sp_s_eta_dn6, locals.var_sp_s_eta_dn7, locals.var_sp_s_eta_dn8, locals.var_sp_s_eta_dn12, locals.var_sp_s_eta_dn13, locals.var_sp_s_eta_dn14, locals.var_sp_s_eta_dn15, locals.var_sp_s_eta_dn16, locals.var_sp_s_eta_dn17, locals.var_sp_s_eta_dn18, locals.var_sp_s_eta_dn19, locals.var_sp_s_eta_dn20,)
    }
};
        locals.var_sp_s_eta = assign41860_e54822;
        locals.var_sp_s_eta_dn5 = assign41860_e54822_d_n5;
        locals.var_sp_s_eta_dn6 = assign41860_e54822_d_n6;
        locals.var_sp_s_eta_dn7 = assign41860_e54822_d_n7;
        locals.var_sp_s_eta_dn8 = assign41860_e54822_d_n8;
        locals.var_sp_s_eta_dn12 = assign41860_e54822_d_n12;
        locals.var_sp_s_eta_dn13 = assign41860_e54822_d_n13;
        locals.var_sp_s_eta_dn14 = assign41860_e54822_d_n14;
        locals.var_sp_s_eta_dn15 = assign41860_e54822_d_n15;
        locals.var_sp_s_eta_dn16 = assign41860_e54822_d_n16;
        locals.var_sp_s_eta_dn17 = assign41860_e54822_d_n17;
        locals.var_sp_s_eta_dn18 = assign41860_e54822_d_n18;
        locals.var_sp_s_eta_dn19 = assign41860_e54822_d_n19;
        locals.var_sp_s_eta_dn20 = assign41860_e54822_d_n20;
        locals.var_sp_s_eta_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_20(
        locals: &mut StampLocals,
    ) {
        let (assign41870_e54831, assign41870_e54831_d_n5, assign41870_e54831_d_n6, assign41870_e54831_d_n7, assign41870_e54831_d_n8, assign41870_e54831_d_n12, assign41870_e54831_d_n13, assign41870_e54831_d_n14, assign41870_e54831_d_n15, assign41870_e54831_d_n16, assign41870_e54831_d_n17, assign41870_e54831_d_n18, assign41870_e54831_d_n19, assign41870_e54831_d_n20,) = {
    if ((locals.var_guard1284 == 0.0) && (locals.var_guard1285 != 0.0)) {
        let assign41870_e54829: f64 = (locals.var_sp_s_yg - locals.var_sp_s_eta);
        (assign41870_e54829, (locals.var_sp_s_yg_dn5 - locals.var_sp_s_eta_dn5), (locals.var_sp_s_yg_dn6 - locals.var_sp_s_eta_dn6), (locals.var_sp_s_yg_dn7 - locals.var_sp_s_eta_dn7), (locals.var_sp_s_yg_dn8 - locals.var_sp_s_eta_dn8), (locals.var_sp_s_yg_dn12 - locals.var_sp_s_eta_dn12), (locals.var_sp_s_yg_dn13 - locals.var_sp_s_eta_dn13), (locals.var_sp_s_yg_dn14 - locals.var_sp_s_eta_dn14), (locals.var_sp_s_yg_dn15 - locals.var_sp_s_eta_dn15), (locals.var_sp_s_yg_dn16 - locals.var_sp_s_eta_dn16), (locals.var_sp_s_yg_dn17 - locals.var_sp_s_eta_dn17), (locals.var_sp_s_yg_dn18 - locals.var_sp_s_eta_dn18), (locals.var_sp_s_yg_dn19 - locals.var_sp_s_eta_dn19), (locals.var_sp_s_yg_dn20 - locals.var_sp_s_eta_dn20),)
    } else {
        (locals.var_sp_s_temp, locals.var_sp_s_temp_dn5, locals.var_sp_s_temp_dn6, locals.var_sp_s_temp_dn7, locals.var_sp_s_temp_dn8, locals.var_sp_s_temp_dn12, locals.var_sp_s_temp_dn13, locals.var_sp_s_temp_dn14, locals.var_sp_s_temp_dn15, locals.var_sp_s_temp_dn16, locals.var_sp_s_temp_dn17, locals.var_sp_s_temp_dn18, locals.var_sp_s_temp_dn19, locals.var_sp_s_temp_dn20,)
    }
};
        locals.var_sp_s_temp = assign41870_e54831;
        locals.var_sp_s_temp_dn5 = assign41870_e54831_d_n5;
        locals.var_sp_s_temp_dn6 = assign41870_e54831_d_n6;
        locals.var_sp_s_temp_dn7 = assign41870_e54831_d_n7;
        locals.var_sp_s_temp_dn8 = assign41870_e54831_d_n8;
        locals.var_sp_s_temp_dn12 = assign41870_e54831_d_n12;
        locals.var_sp_s_temp_dn13 = assign41870_e54831_d_n13;
        locals.var_sp_s_temp_dn14 = assign41870_e54831_d_n14;
        locals.var_sp_s_temp_dn15 = assign41870_e54831_d_n15;
        locals.var_sp_s_temp_dn16 = assign41870_e54831_d_n16;
        locals.var_sp_s_temp_dn17 = assign41870_e54831_d_n17;
        locals.var_sp_s_temp_dn18 = assign41870_e54831_d_n18;
        locals.var_sp_s_temp_dn19 = assign41870_e54831_d_n19;
        locals.var_sp_s_temp_dn20 = assign41870_e54831_d_n20;
        locals.var_sp_s_temp_rv = 0.0;

        let (assign41880_e54846, assign41880_e54846_d_n5, assign41880_e54846_d_n6, assign41880_e54846_d_n7, assign41880_e54846_d_n8, assign41880_e54846_d_n12, assign41880_e54846_d_n13, assign41880_e54846_d_n14, assign41880_e54846_d_n15, assign41880_e54846_d_n16, assign41880_e54846_d_n17, assign41880_e54846_d_n18, assign41880_e54846_d_n19, assign41880_e54846_d_n20,) = {
    if ((locals.var_guard1284 == 0.0) && (locals.var_guard1285 != 0.0)) {
        let assign41880_e54838: f64 = (locals.var_sp_s_temp * locals.var_sp_s_temp);
        let assign41880_e54842: f64 = (locals.var_sp_s_eta + 1.0);
        let assign41880_e54843: f64 = (locals.var_gf2 * assign41880_e54842);
        let assign41880_e54844: f64 = (assign41880_e54838 + assign41880_e54843);
        (assign41880_e54844, (((locals.var_sp_s_temp_dn5 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn5)) + ((locals.var_gf2_dn5 * assign41880_e54842) + (locals.var_gf2 * locals.var_sp_s_eta_dn5))), (((locals.var_sp_s_temp_dn6 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn6)) + ((locals.var_gf2_dn6 * assign41880_e54842) + (locals.var_gf2 * locals.var_sp_s_eta_dn6))), (((locals.var_sp_s_temp_dn7 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn7)) + ((locals.var_gf2_dn7 * assign41880_e54842) + (locals.var_gf2 * locals.var_sp_s_eta_dn7))), (((locals.var_sp_s_temp_dn8 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn8)) + ((locals.var_gf2_dn8 * assign41880_e54842) + (locals.var_gf2 * locals.var_sp_s_eta_dn8))), (((locals.var_sp_s_temp_dn12 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn12)) + ((locals.var_gf2_dn12 * assign41880_e54842) + (locals.var_gf2 * locals.var_sp_s_eta_dn12))), (((locals.var_sp_s_temp_dn13 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn13)) + ((locals.var_gf2_dn13 * assign41880_e54842) + (locals.var_gf2 * locals.var_sp_s_eta_dn13))), (((locals.var_sp_s_temp_dn14 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn14)) + ((locals.var_gf2_dn14 * assign41880_e54842) + (locals.var_gf2 * locals.var_sp_s_eta_dn14))), (((locals.var_sp_s_temp_dn15 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn15)) + ((locals.var_gf2_dn15 * assign41880_e54842) + (locals.var_gf2 * locals.var_sp_s_eta_dn15))), (((locals.var_sp_s_temp_dn16 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn16)) + ((locals.var_gf2_dn16 * assign41880_e54842) + (locals.var_gf2 * locals.var_sp_s_eta_dn16))), (((locals.var_sp_s_temp_dn17 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn17)) + ((locals.var_gf2_dn17 * assign41880_e54842) + (locals.var_gf2 * locals.var_sp_s_eta_dn17))), (((locals.var_sp_s_temp_dn18 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn18)) + ((locals.var_gf2_dn18 * assign41880_e54842) + (locals.var_gf2 * locals.var_sp_s_eta_dn18))), (((locals.var_sp_s_temp_dn19 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn19)) + ((locals.var_gf2_dn19 * assign41880_e54842) + (locals.var_gf2 * locals.var_sp_s_eta_dn19))), (((locals.var_sp_s_temp_dn20 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn20)) + ((locals.var_gf2_dn20 * assign41880_e54842) + (locals.var_gf2 * locals.var_sp_s_eta_dn20))),)
    } else {
        (locals.var_sp_s_a, locals.var_sp_s_a_dn5, locals.var_sp_s_a_dn6, locals.var_sp_s_a_dn7, locals.var_sp_s_a_dn8, locals.var_sp_s_a_dn12, locals.var_sp_s_a_dn13, locals.var_sp_s_a_dn14, locals.var_sp_s_a_dn15, locals.var_sp_s_a_dn16, locals.var_sp_s_a_dn17, locals.var_sp_s_a_dn18, locals.var_sp_s_a_dn19, locals.var_sp_s_a_dn20,)
    }
};
        locals.var_sp_s_a = assign41880_e54846;
        locals.var_sp_s_a_dn5 = assign41880_e54846_d_n5;
        locals.var_sp_s_a_dn6 = assign41880_e54846_d_n6;
        locals.var_sp_s_a_dn7 = assign41880_e54846_d_n7;
        locals.var_sp_s_a_dn8 = assign41880_e54846_d_n8;
        locals.var_sp_s_a_dn12 = assign41880_e54846_d_n12;
        locals.var_sp_s_a_dn13 = assign41880_e54846_d_n13;
        locals.var_sp_s_a_dn14 = assign41880_e54846_d_n14;
        locals.var_sp_s_a_dn15 = assign41880_e54846_d_n15;
        locals.var_sp_s_a_dn16 = assign41880_e54846_d_n16;
        locals.var_sp_s_a_dn17 = assign41880_e54846_d_n17;
        locals.var_sp_s_a_dn18 = assign41880_e54846_d_n18;
        locals.var_sp_s_a_dn19 = assign41880_e54846_d_n19;
        locals.var_sp_s_a_dn20 = assign41880_e54846_d_n20;
        locals.var_sp_s_a_rv = 0.0;

        let (assign41890_e54857, assign41890_e54857_d_n5, assign41890_e54857_d_n6, assign41890_e54857_d_n7, assign41890_e54857_d_n8, assign41890_e54857_d_n12, assign41890_e54857_d_n13, assign41890_e54857_d_n14, assign41890_e54857_d_n15, assign41890_e54857_d_n16, assign41890_e54857_d_n17, assign41890_e54857_d_n18, assign41890_e54857_d_n19, assign41890_e54857_d_n20,) = {
    if ((locals.var_guard1284 == 0.0) && (locals.var_guard1285 != 0.0)) {
        let assign41890_e54853: f64 = (2.0 * locals.var_sp_s_temp);
        let assign41890_e54855: f64 = (assign41890_e54853 - locals.var_gf2);
        (assign41890_e54855, ((2.0 * locals.var_sp_s_temp_dn5) - locals.var_gf2_dn5), ((2.0 * locals.var_sp_s_temp_dn6) - locals.var_gf2_dn6), ((2.0 * locals.var_sp_s_temp_dn7) - locals.var_gf2_dn7), ((2.0 * locals.var_sp_s_temp_dn8) - locals.var_gf2_dn8), ((2.0 * locals.var_sp_s_temp_dn12) - locals.var_gf2_dn12), ((2.0 * locals.var_sp_s_temp_dn13) - locals.var_gf2_dn13), ((2.0 * locals.var_sp_s_temp_dn14) - locals.var_gf2_dn14), ((2.0 * locals.var_sp_s_temp_dn15) - locals.var_gf2_dn15), ((2.0 * locals.var_sp_s_temp_dn16) - locals.var_gf2_dn16), ((2.0 * locals.var_sp_s_temp_dn17) - locals.var_gf2_dn17), ((2.0 * locals.var_sp_s_temp_dn18) - locals.var_gf2_dn18), ((2.0 * locals.var_sp_s_temp_dn19) - locals.var_gf2_dn19), ((2.0 * locals.var_sp_s_temp_dn20) - locals.var_gf2_dn20),)
    } else {
        (locals.var_sp_s_c, locals.var_sp_s_c_dn5, locals.var_sp_s_c_dn6, locals.var_sp_s_c_dn7, locals.var_sp_s_c_dn8, locals.var_sp_s_c_dn12, locals.var_sp_s_c_dn13, locals.var_sp_s_c_dn14, locals.var_sp_s_c_dn15, locals.var_sp_s_c_dn16, locals.var_sp_s_c_dn17, locals.var_sp_s_c_dn18, locals.var_sp_s_c_dn19, locals.var_sp_s_c_dn20,)
    }
};
        locals.var_sp_s_c = assign41890_e54857;
        locals.var_sp_s_c_dn5 = assign41890_e54857_d_n5;
        locals.var_sp_s_c_dn6 = assign41890_e54857_d_n6;
        locals.var_sp_s_c_dn7 = assign41890_e54857_d_n7;
        locals.var_sp_s_c_dn8 = assign41890_e54857_d_n8;
        locals.var_sp_s_c_dn12 = assign41890_e54857_d_n12;
        locals.var_sp_s_c_dn13 = assign41890_e54857_d_n13;
        locals.var_sp_s_c_dn14 = assign41890_e54857_d_n14;
        locals.var_sp_s_c_dn15 = assign41890_e54857_d_n15;
        locals.var_sp_s_c_dn16 = assign41890_e54857_d_n16;
        locals.var_sp_s_c_dn17 = assign41890_e54857_d_n17;
        locals.var_sp_s_c_dn18 = assign41890_e54857_d_n18;
        locals.var_sp_s_c_dn19 = assign41890_e54857_d_n19;
        locals.var_sp_s_c_dn20 = assign41890_e54857_d_n20;
        locals.var_sp_s_c_rv = 0.0;

        let (assign41900_e54870, assign41900_e54870_d_n5, assign41900_e54870_d_n6, assign41900_e54870_d_n7, assign41900_e54870_d_n8, assign41900_e54870_d_n12, assign41900_e54870_d_n13, assign41900_e54870_d_n14, assign41900_e54870_d_n15, assign41900_e54870_d_n16, assign41900_e54870_d_n17, assign41900_e54870_d_n18, assign41900_e54870_d_n19, assign41900_e54870_d_n20,) = {
    if ((locals.var_guard1284 == 0.0) && (locals.var_guard1285 != 0.0)) {
        let assign41900_e54863: f64 = (-locals.var_sp_s_eta);
        let assign41900_e54866: f64 = (locals.var_sp_s_a * locals.var_inv_gf2);
        let assign41900_e54867: f64 = (assign41900_e54866).ln();
        let assign41900_e54868: f64 = (assign41900_e54863 + assign41900_e54867);
        (assign41900_e54868, ((-locals.var_sp_s_eta_dn5) + (((locals.var_sp_s_a_dn5 * locals.var_inv_gf2) + (locals.var_sp_s_a * locals.var_inv_gf2_dn5)) / assign41900_e54866)), ((-locals.var_sp_s_eta_dn6) + (((locals.var_sp_s_a_dn6 * locals.var_inv_gf2) + (locals.var_sp_s_a * locals.var_inv_gf2_dn6)) / assign41900_e54866)), ((-locals.var_sp_s_eta_dn7) + (((locals.var_sp_s_a_dn7 * locals.var_inv_gf2) + (locals.var_sp_s_a * locals.var_inv_gf2_dn7)) / assign41900_e54866)), ((-locals.var_sp_s_eta_dn8) + (((locals.var_sp_s_a_dn8 * locals.var_inv_gf2) + (locals.var_sp_s_a * locals.var_inv_gf2_dn8)) / assign41900_e54866)), ((-locals.var_sp_s_eta_dn12) + (((locals.var_sp_s_a_dn12 * locals.var_inv_gf2) + (locals.var_sp_s_a * locals.var_inv_gf2_dn12)) / assign41900_e54866)), ((-locals.var_sp_s_eta_dn13) + (((locals.var_sp_s_a_dn13 * locals.var_inv_gf2) + (locals.var_sp_s_a * locals.var_inv_gf2_dn13)) / assign41900_e54866)), ((-locals.var_sp_s_eta_dn14) + (((locals.var_sp_s_a_dn14 * locals.var_inv_gf2) + (locals.var_sp_s_a * locals.var_inv_gf2_dn14)) / assign41900_e54866)), ((-locals.var_sp_s_eta_dn15) + (((locals.var_sp_s_a_dn15 * locals.var_inv_gf2) + (locals.var_sp_s_a * locals.var_inv_gf2_dn15)) / assign41900_e54866)), ((-locals.var_sp_s_eta_dn16) + (((locals.var_sp_s_a_dn16 * locals.var_inv_gf2) + (locals.var_sp_s_a * locals.var_inv_gf2_dn16)) / assign41900_e54866)), ((-locals.var_sp_s_eta_dn17) + (((locals.var_sp_s_a_dn17 * locals.var_inv_gf2) + (locals.var_sp_s_a * locals.var_inv_gf2_dn17)) / assign41900_e54866)), ((-locals.var_sp_s_eta_dn18) + (((locals.var_sp_s_a_dn18 * locals.var_inv_gf2) + (locals.var_sp_s_a * locals.var_inv_gf2_dn18)) / assign41900_e54866)), ((-locals.var_sp_s_eta_dn19) + (((locals.var_sp_s_a_dn19 * locals.var_inv_gf2) + (locals.var_sp_s_a * locals.var_inv_gf2_dn19)) / assign41900_e54866)), ((-locals.var_sp_s_eta_dn20) + (((locals.var_sp_s_a_dn20 * locals.var_inv_gf2) + (locals.var_sp_s_a * locals.var_inv_gf2_dn20)) / assign41900_e54866)),)
    } else {
        (locals.var_sp_s_tau, locals.var_sp_s_tau_dn5, locals.var_sp_s_tau_dn6, locals.var_sp_s_tau_dn7, locals.var_sp_s_tau_dn8, locals.var_sp_s_tau_dn12, locals.var_sp_s_tau_dn13, locals.var_sp_s_tau_dn14, locals.var_sp_s_tau_dn15, locals.var_sp_s_tau_dn16, locals.var_sp_s_tau_dn17, locals.var_sp_s_tau_dn18, locals.var_sp_s_tau_dn19, locals.var_sp_s_tau_dn20,)
    }
};
        locals.var_sp_s_tau = assign41900_e54870;
        locals.var_sp_s_tau_dn5 = assign41900_e54870_d_n5;
        locals.var_sp_s_tau_dn6 = assign41900_e54870_d_n6;
        locals.var_sp_s_tau_dn7 = assign41900_e54870_d_n7;
        locals.var_sp_s_tau_dn8 = assign41900_e54870_d_n8;
        locals.var_sp_s_tau_dn12 = assign41900_e54870_d_n12;
        locals.var_sp_s_tau_dn13 = assign41900_e54870_d_n13;
        locals.var_sp_s_tau_dn14 = assign41900_e54870_d_n14;
        locals.var_sp_s_tau_dn15 = assign41900_e54870_d_n15;
        locals.var_sp_s_tau_dn16 = assign41900_e54870_d_n16;
        locals.var_sp_s_tau_dn17 = assign41900_e54870_d_n17;
        locals.var_sp_s_tau_dn18 = assign41900_e54870_d_n18;
        locals.var_sp_s_tau_dn19 = assign41900_e54870_d_n19;
        locals.var_sp_s_tau_dn20 = assign41900_e54870_d_n20;
        locals.var_sp_s_tau_rv = 0.0;

        let (assign41910_e54879, assign41910_e54879_d_n5, assign41910_e54879_d_n6, assign41910_e54879_d_n7, assign41910_e54879_d_n8, assign41910_e54879_d_n12, assign41910_e54879_d_n13, assign41910_e54879_d_n14, assign41910_e54879_d_n15, assign41910_e54879_d_n16, assign41910_e54879_d_n17, assign41910_e54879_d_n18, assign41910_e54879_d_n19, assign41910_e54879_d_n20,) = {
    if ((locals.var_guard1284 == 0.0) && (locals.var_guard1285 != 0.0)) {
        let assign41910_e54877: f64 = (locals.var_sp_s_a + locals.var_sp_s_c);
        (assign41910_e54877, (locals.var_sp_s_a_dn5 + locals.var_sp_s_c_dn5), (locals.var_sp_s_a_dn6 + locals.var_sp_s_c_dn6), (locals.var_sp_s_a_dn7 + locals.var_sp_s_c_dn7), (locals.var_sp_s_a_dn8 + locals.var_sp_s_c_dn8), (locals.var_sp_s_a_dn12 + locals.var_sp_s_c_dn12), (locals.var_sp_s_a_dn13 + locals.var_sp_s_c_dn13), (locals.var_sp_s_a_dn14 + locals.var_sp_s_c_dn14), (locals.var_sp_s_a_dn15 + locals.var_sp_s_c_dn15), (locals.var_sp_s_a_dn16 + locals.var_sp_s_c_dn16), (locals.var_sp_s_a_dn17 + locals.var_sp_s_c_dn17), (locals.var_sp_s_a_dn18 + locals.var_sp_s_c_dn18), (locals.var_sp_s_a_dn19 + locals.var_sp_s_c_dn19), (locals.var_sp_s_a_dn20 + locals.var_sp_s_c_dn20),)
    } else {
        (locals.var_nu, locals.var_nu_dn5, locals.var_nu_dn6, locals.var_nu_dn7, locals.var_nu_dn8, locals.var_nu_dn12, locals.var_nu_dn13, locals.var_nu_dn14, locals.var_nu_dn15, locals.var_nu_dn16, locals.var_nu_dn17, locals.var_nu_dn18, locals.var_nu_dn19, locals.var_nu_dn20,)
    }
};
        locals.var_nu = assign41910_e54879;
        locals.var_nu_dn5 = assign41910_e54879_d_n5;
        locals.var_nu_dn6 = assign41910_e54879_d_n6;
        locals.var_nu_dn7 = assign41910_e54879_d_n7;
        locals.var_nu_dn8 = assign41910_e54879_d_n8;
        locals.var_nu_dn12 = assign41910_e54879_d_n12;
        locals.var_nu_dn13 = assign41910_e54879_d_n13;
        locals.var_nu_dn14 = assign41910_e54879_d_n14;
        locals.var_nu_dn15 = assign41910_e54879_d_n15;
        locals.var_nu_dn16 = assign41910_e54879_d_n16;
        locals.var_nu_dn17 = assign41910_e54879_d_n17;
        locals.var_nu_dn18 = assign41910_e54879_d_n18;
        locals.var_nu_dn19 = assign41910_e54879_d_n19;
        locals.var_nu_dn20 = assign41910_e54879_d_n20;
        locals.var_nu_rv = 0.0;

        let (assign41920_e54898, assign41920_e54898_d_n5, assign41920_e54898_d_n6, assign41920_e54898_d_n7, assign41920_e54898_d_n8, assign41920_e54898_d_n12, assign41920_e54898_d_n13, assign41920_e54898_d_n14, assign41920_e54898_d_n15, assign41920_e54898_d_n16, assign41920_e54898_d_n17, assign41920_e54898_d_n18, assign41920_e54898_d_n19, assign41920_e54898_d_n20,) = {
    if ((locals.var_guard1284 == 0.0) && (locals.var_guard1285 != 0.0)) {
        let assign41920_e54886: f64 = (locals.var_nu * locals.var_nu);
        let assign41920_e54891: f64 = (locals.var_sp_s_c * locals.var_sp_s_c);
        let assign41920_e54892: f64 = (0.5 * assign41920_e54891);
        let assign41920_e54894: f64 = (assign41920_e54892 - locals.var_sp_s_a);
        let assign41920_e54895: f64 = (locals.var_sp_s_tau * assign41920_e54894);
        let assign41920_e54896: f64 = (assign41920_e54886 + assign41920_e54895);
        (assign41920_e54896, (((locals.var_nu_dn5 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn5)) + ((locals.var_sp_s_tau_dn5 * assign41920_e54894) + (locals.var_sp_s_tau * ((0.5 * ((locals.var_sp_s_c_dn5 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn5))) - locals.var_sp_s_a_dn5)))), (((locals.var_nu_dn6 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn6)) + ((locals.var_sp_s_tau_dn6 * assign41920_e54894) + (locals.var_sp_s_tau * ((0.5 * ((locals.var_sp_s_c_dn6 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn6))) - locals.var_sp_s_a_dn6)))), (((locals.var_nu_dn7 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn7)) + ((locals.var_sp_s_tau_dn7 * assign41920_e54894) + (locals.var_sp_s_tau * ((0.5 * ((locals.var_sp_s_c_dn7 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn7))) - locals.var_sp_s_a_dn7)))), (((locals.var_nu_dn8 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn8)) + ((locals.var_sp_s_tau_dn8 * assign41920_e54894) + (locals.var_sp_s_tau * ((0.5 * ((locals.var_sp_s_c_dn8 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn8))) - locals.var_sp_s_a_dn8)))), (((locals.var_nu_dn12 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn12)) + ((locals.var_sp_s_tau_dn12 * assign41920_e54894) + (locals.var_sp_s_tau * ((0.5 * ((locals.var_sp_s_c_dn12 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn12))) - locals.var_sp_s_a_dn12)))), (((locals.var_nu_dn13 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn13)) + ((locals.var_sp_s_tau_dn13 * assign41920_e54894) + (locals.var_sp_s_tau * ((0.5 * ((locals.var_sp_s_c_dn13 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn13))) - locals.var_sp_s_a_dn13)))), (((locals.var_nu_dn14 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn14)) + ((locals.var_sp_s_tau_dn14 * assign41920_e54894) + (locals.var_sp_s_tau * ((0.5 * ((locals.var_sp_s_c_dn14 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn14))) - locals.var_sp_s_a_dn14)))), (((locals.var_nu_dn15 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn15)) + ((locals.var_sp_s_tau_dn15 * assign41920_e54894) + (locals.var_sp_s_tau * ((0.5 * ((locals.var_sp_s_c_dn15 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn15))) - locals.var_sp_s_a_dn15)))), (((locals.var_nu_dn16 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn16)) + ((locals.var_sp_s_tau_dn16 * assign41920_e54894) + (locals.var_sp_s_tau * ((0.5 * ((locals.var_sp_s_c_dn16 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn16))) - locals.var_sp_s_a_dn16)))), (((locals.var_nu_dn17 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn17)) + ((locals.var_sp_s_tau_dn17 * assign41920_e54894) + (locals.var_sp_s_tau * ((0.5 * ((locals.var_sp_s_c_dn17 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn17))) - locals.var_sp_s_a_dn17)))), (((locals.var_nu_dn18 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn18)) + ((locals.var_sp_s_tau_dn18 * assign41920_e54894) + (locals.var_sp_s_tau * ((0.5 * ((locals.var_sp_s_c_dn18 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn18))) - locals.var_sp_s_a_dn18)))), (((locals.var_nu_dn19 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn19)) + ((locals.var_sp_s_tau_dn19 * assign41920_e54894) + (locals.var_sp_s_tau * ((0.5 * ((locals.var_sp_s_c_dn19 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn19))) - locals.var_sp_s_a_dn19)))), (((locals.var_nu_dn20 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn20)) + ((locals.var_sp_s_tau_dn20 * assign41920_e54894) + (locals.var_sp_s_tau * ((0.5 * ((locals.var_sp_s_c_dn20 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn20))) - locals.var_sp_s_a_dn20)))),)
    } else {
        (locals.var_mutau, locals.var_mutau_dn5, locals.var_mutau_dn6, locals.var_mutau_dn7, locals.var_mutau_dn8, locals.var_mutau_dn12, locals.var_mutau_dn13, locals.var_mutau_dn14, locals.var_mutau_dn15, locals.var_mutau_dn16, locals.var_mutau_dn17, locals.var_mutau_dn18, locals.var_mutau_dn19, locals.var_mutau_dn20,)
    }
};
        locals.var_mutau = assign41920_e54898;
        locals.var_mutau_dn5 = assign41920_e54898_d_n5;
        locals.var_mutau_dn6 = assign41920_e54898_d_n6;
        locals.var_mutau_dn7 = assign41920_e54898_d_n7;
        locals.var_mutau_dn8 = assign41920_e54898_d_n8;
        locals.var_mutau_dn12 = assign41920_e54898_d_n12;
        locals.var_mutau_dn13 = assign41920_e54898_d_n13;
        locals.var_mutau_dn14 = assign41920_e54898_d_n14;
        locals.var_mutau_dn15 = assign41920_e54898_d_n15;
        locals.var_mutau_dn16 = assign41920_e54898_d_n16;
        locals.var_mutau_dn17 = assign41920_e54898_d_n17;
        locals.var_mutau_dn18 = assign41920_e54898_d_n18;
        locals.var_mutau_dn19 = assign41920_e54898_d_n19;
        locals.var_mutau_dn20 = assign41920_e54898_d_n20;
        locals.var_mutau_rv = 0.0;

        let (assign41930_e54931, assign41930_e54931_d_n5, assign41930_e54931_d_n6, assign41930_e54931_d_n7, assign41930_e54931_d_n8, assign41930_e54931_d_n12, assign41930_e54931_d_n13, assign41930_e54931_d_n14, assign41930_e54931_d_n15, assign41930_e54931_d_n16, assign41930_e54931_d_n17, assign41930_e54931_d_n18, assign41930_e54931_d_n19, assign41930_e54931_d_n20,) = {
    if ((locals.var_guard1284 == 0.0) && (locals.var_guard1285 != 0.0)) {
        let assign41930_e54906: f64 = (locals.var_sp_s_a * locals.var_nu);
        let assign41930_e54908: f64 = (assign41930_e54906 * locals.var_sp_s_tau);
        let assign41930_e54912: f64 = (locals.var_nu / locals.var_mutau);
        let assign41930_e54914: f64 = (assign41930_e54912 * locals.var_sp_s_tau);
        let assign41930_e54916: f64 = (assign41930_e54914 * locals.var_sp_s_tau);
        let assign41930_e54918: f64 = (assign41930_e54916 * locals.var_sp_s_c);
        let assign41930_e54921: f64 = (locals.var_sp_s_c * locals.var_sp_s_c);
        let assign41930_e54923: f64 = (assign41930_e54921 * 0.3333333333333333);
        let assign41930_e54925: f64 = (assign41930_e54923 - locals.var_sp_s_a);
        let assign41930_e54926: f64 = (assign41930_e54918 * assign41930_e54925);
        let assign41930_e54927: f64 = (locals.var_mutau + assign41930_e54926);
        let assign41930_e54928: f64 = (assign41930_e54908 / assign41930_e54927);
        let assign41930_e54929: f64 = (locals.var_sp_s_eta + assign41930_e54928);
        (assign41930_e54929, (locals.var_sp_s_eta_dn5 + (((((((locals.var_sp_s_a_dn5 * locals.var_nu) + (locals.var_sp_s_a * locals.var_nu_dn5)) * locals.var_sp_s_tau) + (assign41930_e54906 * locals.var_sp_s_tau_dn5)) * assign41930_e54927) - (assign41930_e54908 * (locals.var_mutau_dn5 + (((((((((((locals.var_nu_dn5 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn5)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau) + (assign41930_e54912 * locals.var_sp_s_tau_dn5)) * locals.var_sp_s_tau) + (assign41930_e54914 * locals.var_sp_s_tau_dn5)) * locals.var_sp_s_c) + (assign41930_e54916 * locals.var_sp_s_c_dn5)) * assign41930_e54925) + (assign41930_e54918 * ((((locals.var_sp_s_c_dn5 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn5)) * 0.3333333333333333) - locals.var_sp_s_a_dn5)))))) / (assign41930_e54927 * assign41930_e54927))), (locals.var_sp_s_eta_dn6 + (((((((locals.var_sp_s_a_dn6 * locals.var_nu) + (locals.var_sp_s_a * locals.var_nu_dn6)) * locals.var_sp_s_tau) + (assign41930_e54906 * locals.var_sp_s_tau_dn6)) * assign41930_e54927) - (assign41930_e54908 * (locals.var_mutau_dn6 + (((((((((((locals.var_nu_dn6 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn6)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau) + (assign41930_e54912 * locals.var_sp_s_tau_dn6)) * locals.var_sp_s_tau) + (assign41930_e54914 * locals.var_sp_s_tau_dn6)) * locals.var_sp_s_c) + (assign41930_e54916 * locals.var_sp_s_c_dn6)) * assign41930_e54925) + (assign41930_e54918 * ((((locals.var_sp_s_c_dn6 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn6)) * 0.3333333333333333) - locals.var_sp_s_a_dn6)))))) / (assign41930_e54927 * assign41930_e54927))), (locals.var_sp_s_eta_dn7 + (((((((locals.var_sp_s_a_dn7 * locals.var_nu) + (locals.var_sp_s_a * locals.var_nu_dn7)) * locals.var_sp_s_tau) + (assign41930_e54906 * locals.var_sp_s_tau_dn7)) * assign41930_e54927) - (assign41930_e54908 * (locals.var_mutau_dn7 + (((((((((((locals.var_nu_dn7 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn7)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau) + (assign41930_e54912 * locals.var_sp_s_tau_dn7)) * locals.var_sp_s_tau) + (assign41930_e54914 * locals.var_sp_s_tau_dn7)) * locals.var_sp_s_c) + (assign41930_e54916 * locals.var_sp_s_c_dn7)) * assign41930_e54925) + (assign41930_e54918 * ((((locals.var_sp_s_c_dn7 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn7)) * 0.3333333333333333) - locals.var_sp_s_a_dn7)))))) / (assign41930_e54927 * assign41930_e54927))), (locals.var_sp_s_eta_dn8 + (((((((locals.var_sp_s_a_dn8 * locals.var_nu) + (locals.var_sp_s_a * locals.var_nu_dn8)) * locals.var_sp_s_tau) + (assign41930_e54906 * locals.var_sp_s_tau_dn8)) * assign41930_e54927) - (assign41930_e54908 * (locals.var_mutau_dn8 + (((((((((((locals.var_nu_dn8 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn8)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau) + (assign41930_e54912 * locals.var_sp_s_tau_dn8)) * locals.var_sp_s_tau) + (assign41930_e54914 * locals.var_sp_s_tau_dn8)) * locals.var_sp_s_c) + (assign41930_e54916 * locals.var_sp_s_c_dn8)) * assign41930_e54925) + (assign41930_e54918 * ((((locals.var_sp_s_c_dn8 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn8)) * 0.3333333333333333) - locals.var_sp_s_a_dn8)))))) / (assign41930_e54927 * assign41930_e54927))), (locals.var_sp_s_eta_dn12 + (((((((locals.var_sp_s_a_dn12 * locals.var_nu) + (locals.var_sp_s_a * locals.var_nu_dn12)) * locals.var_sp_s_tau) + (assign41930_e54906 * locals.var_sp_s_tau_dn12)) * assign41930_e54927) - (assign41930_e54908 * (locals.var_mutau_dn12 + (((((((((((locals.var_nu_dn12 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn12)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau) + (assign41930_e54912 * locals.var_sp_s_tau_dn12)) * locals.var_sp_s_tau) + (assign41930_e54914 * locals.var_sp_s_tau_dn12)) * locals.var_sp_s_c) + (assign41930_e54916 * locals.var_sp_s_c_dn12)) * assign41930_e54925) + (assign41930_e54918 * ((((locals.var_sp_s_c_dn12 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn12)) * 0.3333333333333333) - locals.var_sp_s_a_dn12)))))) / (assign41930_e54927 * assign41930_e54927))), (locals.var_sp_s_eta_dn13 + (((((((locals.var_sp_s_a_dn13 * locals.var_nu) + (locals.var_sp_s_a * locals.var_nu_dn13)) * locals.var_sp_s_tau) + (assign41930_e54906 * locals.var_sp_s_tau_dn13)) * assign41930_e54927) - (assign41930_e54908 * (locals.var_mutau_dn13 + (((((((((((locals.var_nu_dn13 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn13)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau) + (assign41930_e54912 * locals.var_sp_s_tau_dn13)) * locals.var_sp_s_tau) + (assign41930_e54914 * locals.var_sp_s_tau_dn13)) * locals.var_sp_s_c) + (assign41930_e54916 * locals.var_sp_s_c_dn13)) * assign41930_e54925) + (assign41930_e54918 * ((((locals.var_sp_s_c_dn13 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn13)) * 0.3333333333333333) - locals.var_sp_s_a_dn13)))))) / (assign41930_e54927 * assign41930_e54927))), (locals.var_sp_s_eta_dn14 + (((((((locals.var_sp_s_a_dn14 * locals.var_nu) + (locals.var_sp_s_a * locals.var_nu_dn14)) * locals.var_sp_s_tau) + (assign41930_e54906 * locals.var_sp_s_tau_dn14)) * assign41930_e54927) - (assign41930_e54908 * (locals.var_mutau_dn14 + (((((((((((locals.var_nu_dn14 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn14)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau) + (assign41930_e54912 * locals.var_sp_s_tau_dn14)) * locals.var_sp_s_tau) + (assign41930_e54914 * locals.var_sp_s_tau_dn14)) * locals.var_sp_s_c) + (assign41930_e54916 * locals.var_sp_s_c_dn14)) * assign41930_e54925) + (assign41930_e54918 * ((((locals.var_sp_s_c_dn14 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn14)) * 0.3333333333333333) - locals.var_sp_s_a_dn14)))))) / (assign41930_e54927 * assign41930_e54927))), (locals.var_sp_s_eta_dn15 + (((((((locals.var_sp_s_a_dn15 * locals.var_nu) + (locals.var_sp_s_a * locals.var_nu_dn15)) * locals.var_sp_s_tau) + (assign41930_e54906 * locals.var_sp_s_tau_dn15)) * assign41930_e54927) - (assign41930_e54908 * (locals.var_mutau_dn15 + (((((((((((locals.var_nu_dn15 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn15)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau) + (assign41930_e54912 * locals.var_sp_s_tau_dn15)) * locals.var_sp_s_tau) + (assign41930_e54914 * locals.var_sp_s_tau_dn15)) * locals.var_sp_s_c) + (assign41930_e54916 * locals.var_sp_s_c_dn15)) * assign41930_e54925) + (assign41930_e54918 * ((((locals.var_sp_s_c_dn15 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn15)) * 0.3333333333333333) - locals.var_sp_s_a_dn15)))))) / (assign41930_e54927 * assign41930_e54927))), (locals.var_sp_s_eta_dn16 + (((((((locals.var_sp_s_a_dn16 * locals.var_nu) + (locals.var_sp_s_a * locals.var_nu_dn16)) * locals.var_sp_s_tau) + (assign41930_e54906 * locals.var_sp_s_tau_dn16)) * assign41930_e54927) - (assign41930_e54908 * (locals.var_mutau_dn16 + (((((((((((locals.var_nu_dn16 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn16)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau) + (assign41930_e54912 * locals.var_sp_s_tau_dn16)) * locals.var_sp_s_tau) + (assign41930_e54914 * locals.var_sp_s_tau_dn16)) * locals.var_sp_s_c) + (assign41930_e54916 * locals.var_sp_s_c_dn16)) * assign41930_e54925) + (assign41930_e54918 * ((((locals.var_sp_s_c_dn16 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn16)) * 0.3333333333333333) - locals.var_sp_s_a_dn16)))))) / (assign41930_e54927 * assign41930_e54927))), (locals.var_sp_s_eta_dn17 + (((((((locals.var_sp_s_a_dn17 * locals.var_nu) + (locals.var_sp_s_a * locals.var_nu_dn17)) * locals.var_sp_s_tau) + (assign41930_e54906 * locals.var_sp_s_tau_dn17)) * assign41930_e54927) - (assign41930_e54908 * (locals.var_mutau_dn17 + (((((((((((locals.var_nu_dn17 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn17)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau) + (assign41930_e54912 * locals.var_sp_s_tau_dn17)) * locals.var_sp_s_tau) + (assign41930_e54914 * locals.var_sp_s_tau_dn17)) * locals.var_sp_s_c) + (assign41930_e54916 * locals.var_sp_s_c_dn17)) * assign41930_e54925) + (assign41930_e54918 * ((((locals.var_sp_s_c_dn17 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn17)) * 0.3333333333333333) - locals.var_sp_s_a_dn17)))))) / (assign41930_e54927 * assign41930_e54927))), (locals.var_sp_s_eta_dn18 + (((((((locals.var_sp_s_a_dn18 * locals.var_nu) + (locals.var_sp_s_a * locals.var_nu_dn18)) * locals.var_sp_s_tau) + (assign41930_e54906 * locals.var_sp_s_tau_dn18)) * assign41930_e54927) - (assign41930_e54908 * (locals.var_mutau_dn18 + (((((((((((locals.var_nu_dn18 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn18)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau) + (assign41930_e54912 * locals.var_sp_s_tau_dn18)) * locals.var_sp_s_tau) + (assign41930_e54914 * locals.var_sp_s_tau_dn18)) * locals.var_sp_s_c) + (assign41930_e54916 * locals.var_sp_s_c_dn18)) * assign41930_e54925) + (assign41930_e54918 * ((((locals.var_sp_s_c_dn18 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn18)) * 0.3333333333333333) - locals.var_sp_s_a_dn18)))))) / (assign41930_e54927 * assign41930_e54927))), (locals.var_sp_s_eta_dn19 + (((((((locals.var_sp_s_a_dn19 * locals.var_nu) + (locals.var_sp_s_a * locals.var_nu_dn19)) * locals.var_sp_s_tau) + (assign41930_e54906 * locals.var_sp_s_tau_dn19)) * assign41930_e54927) - (assign41930_e54908 * (locals.var_mutau_dn19 + (((((((((((locals.var_nu_dn19 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn19)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau) + (assign41930_e54912 * locals.var_sp_s_tau_dn19)) * locals.var_sp_s_tau) + (assign41930_e54914 * locals.var_sp_s_tau_dn19)) * locals.var_sp_s_c) + (assign41930_e54916 * locals.var_sp_s_c_dn19)) * assign41930_e54925) + (assign41930_e54918 * ((((locals.var_sp_s_c_dn19 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn19)) * 0.3333333333333333) - locals.var_sp_s_a_dn19)))))) / (assign41930_e54927 * assign41930_e54927))), (locals.var_sp_s_eta_dn20 + (((((((locals.var_sp_s_a_dn20 * locals.var_nu) + (locals.var_sp_s_a * locals.var_nu_dn20)) * locals.var_sp_s_tau) + (assign41930_e54906 * locals.var_sp_s_tau_dn20)) * assign41930_e54927) - (assign41930_e54908 * (locals.var_mutau_dn20 + (((((((((((locals.var_nu_dn20 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn20)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau) + (assign41930_e54912 * locals.var_sp_s_tau_dn20)) * locals.var_sp_s_tau) + (assign41930_e54914 * locals.var_sp_s_tau_dn20)) * locals.var_sp_s_c) + (assign41930_e54916 * locals.var_sp_s_c_dn20)) * assign41930_e54925) + (assign41930_e54918 * ((((locals.var_sp_s_c_dn20 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn20)) * 0.3333333333333333) - locals.var_sp_s_a_dn20)))))) / (assign41930_e54927 * assign41930_e54927))),)
    } else {
        (locals.var_sp_s_y0, locals.var_sp_s_y0_dn5, locals.var_sp_s_y0_dn6, locals.var_sp_s_y0_dn7, locals.var_sp_s_y0_dn8, locals.var_sp_s_y0_dn12, locals.var_sp_s_y0_dn13, locals.var_sp_s_y0_dn14, locals.var_sp_s_y0_dn15, locals.var_sp_s_y0_dn16, locals.var_sp_s_y0_dn17, locals.var_sp_s_y0_dn18, locals.var_sp_s_y0_dn19, locals.var_sp_s_y0_dn20,)
    }
};
        locals.var_sp_s_y0 = assign41930_e54931;
        locals.var_sp_s_y0_dn5 = assign41930_e54931_d_n5;
        locals.var_sp_s_y0_dn6 = assign41930_e54931_d_n6;
        locals.var_sp_s_y0_dn7 = assign41930_e54931_d_n7;
        locals.var_sp_s_y0_dn8 = assign41930_e54931_d_n8;
        locals.var_sp_s_y0_dn12 = assign41930_e54931_d_n12;
        locals.var_sp_s_y0_dn13 = assign41930_e54931_d_n13;
        locals.var_sp_s_y0_dn14 = assign41930_e54931_d_n14;
        locals.var_sp_s_y0_dn15 = assign41930_e54931_d_n15;
        locals.var_sp_s_y0_dn16 = assign41930_e54931_d_n16;
        locals.var_sp_s_y0_dn17 = assign41930_e54931_d_n17;
        locals.var_sp_s_y0_dn18 = assign41930_e54931_d_n18;
        locals.var_sp_s_y0_dn19 = assign41930_e54931_d_n19;
        locals.var_sp_s_y0_dn20 = assign41930_e54931_d_n20;
        locals.var_sp_s_y0_rv = 0.0;

        let assign41940_e54934: f64 = if locals.var_sp_s_y0 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard1286 = assign41940_e54934;
        locals.var_guard1286_rv = 0.0;

        let (assign41950_e54944, assign41950_e54944_d_n5, assign41950_e54944_d_n6, assign41950_e54944_d_n7, assign41950_e54944_d_n8, assign41950_e54944_d_n12, assign41950_e54944_d_n13, assign41950_e54944_d_n14, assign41950_e54944_d_n15, assign41950_e54944_d_n16, assign41950_e54944_d_n17, assign41950_e54944_d_n18, assign41950_e54944_d_n19, assign41950_e54944_d_n20,) = {
    if (((locals.var_guard1284 == 0.0) && (locals.var_guard1285 != 0.0)) && (locals.var_guard1286 != 0.0)) {
        let assign41950_e54942: f64 = (locals.var_sp_s_y0).exp();
        (assign41950_e54942, (assign41950_e54942 * locals.var_sp_s_y0_dn5), (assign41950_e54942 * locals.var_sp_s_y0_dn6), (assign41950_e54942 * locals.var_sp_s_y0_dn7), (assign41950_e54942 * locals.var_sp_s_y0_dn8), (assign41950_e54942 * locals.var_sp_s_y0_dn12), (assign41950_e54942 * locals.var_sp_s_y0_dn13), (assign41950_e54942 * locals.var_sp_s_y0_dn14), (assign41950_e54942 * locals.var_sp_s_y0_dn15), (assign41950_e54942 * locals.var_sp_s_y0_dn16), (assign41950_e54942 * locals.var_sp_s_y0_dn17), (assign41950_e54942 * locals.var_sp_s_y0_dn18), (assign41950_e54942 * locals.var_sp_s_y0_dn19), (assign41950_e54942 * locals.var_sp_s_y0_dn20),)
    } else {
        (locals.var_sp_s_delta0, locals.var_sp_s_delta0_dn5, locals.var_sp_s_delta0_dn6, locals.var_sp_s_delta0_dn7, locals.var_sp_s_delta0_dn8, locals.var_sp_s_delta0_dn12, locals.var_sp_s_delta0_dn13, locals.var_sp_s_delta0_dn14, locals.var_sp_s_delta0_dn15, locals.var_sp_s_delta0_dn16, locals.var_sp_s_delta0_dn17, locals.var_sp_s_delta0_dn18, locals.var_sp_s_delta0_dn19, locals.var_sp_s_delta0_dn20,)
    }
};
        locals.var_sp_s_delta0 = assign41950_e54944;
        locals.var_sp_s_delta0_dn5 = assign41950_e54944_d_n5;
        locals.var_sp_s_delta0_dn6 = assign41950_e54944_d_n6;
        locals.var_sp_s_delta0_dn7 = assign41950_e54944_d_n7;
        locals.var_sp_s_delta0_dn8 = assign41950_e54944_d_n8;
        locals.var_sp_s_delta0_dn12 = assign41950_e54944_d_n12;
        locals.var_sp_s_delta0_dn13 = assign41950_e54944_d_n13;
        locals.var_sp_s_delta0_dn14 = assign41950_e54944_d_n14;
        locals.var_sp_s_delta0_dn15 = assign41950_e54944_d_n15;
        locals.var_sp_s_delta0_dn16 = assign41950_e54944_d_n16;
        locals.var_sp_s_delta0_dn17 = assign41950_e54944_d_n17;
        locals.var_sp_s_delta0_dn18 = assign41950_e54944_d_n18;
        locals.var_sp_s_delta0_dn19 = assign41950_e54944_d_n19;
        locals.var_sp_s_delta0_dn20 = assign41950_e54944_d_n20;
        locals.var_sp_s_delta0_rv = 0.0;

        let (assign41960_e54976, assign41960_e54976_d_n5, assign41960_e54976_d_n6, assign41960_e54976_d_n7, assign41960_e54976_d_n8, assign41960_e54976_d_n12, assign41960_e54976_d_n13, assign41960_e54976_d_n14, assign41960_e54976_d_n15, assign41960_e54976_d_n16, assign41960_e54976_d_n17, assign41960_e54976_d_n18, assign41960_e54976_d_n19, assign41960_e54976_d_n20,) = {
    if (((locals.var_guard1284 == 0.0) && (locals.var_guard1285 != 0.0)) && (locals.var_guard1286 == 0.0)) {
        let assign41960_e54956: f64 = (locals.var_sp_s_y0 - 230.25850929940458);
        let assign41960_e54961: f64 = (locals.var_sp_s_y0 - 230.25850929940458);
        let assign41960_e54965: f64 = (locals.var_sp_s_y0 - 230.25850929940458);
        let assign41960_e54967: f64 = (assign41960_e54965 * 0.3333333333333333);
        let assign41960_e54968: f64 = (1.0 + assign41960_e54967);
        let assign41960_e54969: f64 = (assign41960_e54961 * assign41960_e54968);
        let assign41960_e54970: f64 = (0.5 * assign41960_e54969);
        let assign41960_e54971: f64 = (1.0 + assign41960_e54970);
        let assign41960_e54972: f64 = (assign41960_e54956 * assign41960_e54971);
        let assign41960_e54973: f64 = (1.0 + assign41960_e54972);
        let assign41960_e54974: f64 = (1e100 * assign41960_e54973);
        (assign41960_e54974, (1e100 * ((locals.var_sp_s_y0_dn5 * assign41960_e54971) + (assign41960_e54956 * (0.5 * ((locals.var_sp_s_y0_dn5 * assign41960_e54968) + (assign41960_e54961 * (locals.var_sp_s_y0_dn5 * 0.3333333333333333))))))), (1e100 * ((locals.var_sp_s_y0_dn6 * assign41960_e54971) + (assign41960_e54956 * (0.5 * ((locals.var_sp_s_y0_dn6 * assign41960_e54968) + (assign41960_e54961 * (locals.var_sp_s_y0_dn6 * 0.3333333333333333))))))), (1e100 * ((locals.var_sp_s_y0_dn7 * assign41960_e54971) + (assign41960_e54956 * (0.5 * ((locals.var_sp_s_y0_dn7 * assign41960_e54968) + (assign41960_e54961 * (locals.var_sp_s_y0_dn7 * 0.3333333333333333))))))), (1e100 * ((locals.var_sp_s_y0_dn8 * assign41960_e54971) + (assign41960_e54956 * (0.5 * ((locals.var_sp_s_y0_dn8 * assign41960_e54968) + (assign41960_e54961 * (locals.var_sp_s_y0_dn8 * 0.3333333333333333))))))), (1e100 * ((locals.var_sp_s_y0_dn12 * assign41960_e54971) + (assign41960_e54956 * (0.5 * ((locals.var_sp_s_y0_dn12 * assign41960_e54968) + (assign41960_e54961 * (locals.var_sp_s_y0_dn12 * 0.3333333333333333))))))), (1e100 * ((locals.var_sp_s_y0_dn13 * assign41960_e54971) + (assign41960_e54956 * (0.5 * ((locals.var_sp_s_y0_dn13 * assign41960_e54968) + (assign41960_e54961 * (locals.var_sp_s_y0_dn13 * 0.3333333333333333))))))), (1e100 * ((locals.var_sp_s_y0_dn14 * assign41960_e54971) + (assign41960_e54956 * (0.5 * ((locals.var_sp_s_y0_dn14 * assign41960_e54968) + (assign41960_e54961 * (locals.var_sp_s_y0_dn14 * 0.3333333333333333))))))), (1e100 * ((locals.var_sp_s_y0_dn15 * assign41960_e54971) + (assign41960_e54956 * (0.5 * ((locals.var_sp_s_y0_dn15 * assign41960_e54968) + (assign41960_e54961 * (locals.var_sp_s_y0_dn15 * 0.3333333333333333))))))), (1e100 * ((locals.var_sp_s_y0_dn16 * assign41960_e54971) + (assign41960_e54956 * (0.5 * ((locals.var_sp_s_y0_dn16 * assign41960_e54968) + (assign41960_e54961 * (locals.var_sp_s_y0_dn16 * 0.3333333333333333))))))), (1e100 * ((locals.var_sp_s_y0_dn17 * assign41960_e54971) + (assign41960_e54956 * (0.5 * ((locals.var_sp_s_y0_dn17 * assign41960_e54968) + (assign41960_e54961 * (locals.var_sp_s_y0_dn17 * 0.3333333333333333))))))), (1e100 * ((locals.var_sp_s_y0_dn18 * assign41960_e54971) + (assign41960_e54956 * (0.5 * ((locals.var_sp_s_y0_dn18 * assign41960_e54968) + (assign41960_e54961 * (locals.var_sp_s_y0_dn18 * 0.3333333333333333))))))), (1e100 * ((locals.var_sp_s_y0_dn19 * assign41960_e54971) + (assign41960_e54956 * (0.5 * ((locals.var_sp_s_y0_dn19 * assign41960_e54968) + (assign41960_e54961 * (locals.var_sp_s_y0_dn19 * 0.3333333333333333))))))), (1e100 * ((locals.var_sp_s_y0_dn20 * assign41960_e54971) + (assign41960_e54956 * (0.5 * ((locals.var_sp_s_y0_dn20 * assign41960_e54968) + (assign41960_e54961 * (locals.var_sp_s_y0_dn20 * 0.3333333333333333))))))),)
    } else {
        (locals.var_sp_s_delta0, locals.var_sp_s_delta0_dn5, locals.var_sp_s_delta0_dn6, locals.var_sp_s_delta0_dn7, locals.var_sp_s_delta0_dn8, locals.var_sp_s_delta0_dn12, locals.var_sp_s_delta0_dn13, locals.var_sp_s_delta0_dn14, locals.var_sp_s_delta0_dn15, locals.var_sp_s_delta0_dn16, locals.var_sp_s_delta0_dn17, locals.var_sp_s_delta0_dn18, locals.var_sp_s_delta0_dn19, locals.var_sp_s_delta0_dn20,)
    }
};
        locals.var_sp_s_delta0 = assign41960_e54976;
        locals.var_sp_s_delta0_dn5 = assign41960_e54976_d_n5;
        locals.var_sp_s_delta0_dn6 = assign41960_e54976_d_n6;
        locals.var_sp_s_delta0_dn7 = assign41960_e54976_d_n7;
        locals.var_sp_s_delta0_dn8 = assign41960_e54976_d_n8;
        locals.var_sp_s_delta0_dn12 = assign41960_e54976_d_n12;
        locals.var_sp_s_delta0_dn13 = assign41960_e54976_d_n13;
        locals.var_sp_s_delta0_dn14 = assign41960_e54976_d_n14;
        locals.var_sp_s_delta0_dn15 = assign41960_e54976_d_n15;
        locals.var_sp_s_delta0_dn16 = assign41960_e54976_d_n16;
        locals.var_sp_s_delta0_dn17 = assign41960_e54976_d_n17;
        locals.var_sp_s_delta0_dn18 = assign41960_e54976_d_n18;
        locals.var_sp_s_delta0_dn19 = assign41960_e54976_d_n19;
        locals.var_sp_s_delta0_dn20 = assign41960_e54976_d_n20;
        locals.var_sp_s_delta0_rv = 0.0;

        let (assign41970_e54985, assign41970_e54985_d_n5, assign41970_e54985_d_n6, assign41970_e54985_d_n7, assign41970_e54985_d_n8, assign41970_e54985_d_n12, assign41970_e54985_d_n13, assign41970_e54985_d_n14, assign41970_e54985_d_n15, assign41970_e54985_d_n16, assign41970_e54985_d_n17, assign41970_e54985_d_n18, assign41970_e54985_d_n19, assign41970_e54985_d_n20,) = {
    if ((locals.var_guard1284 == 0.0) && (locals.var_guard1285 != 0.0)) {
        let assign41970_e54983: f64 = (1.0 / locals.var_sp_s_delta0);
        (assign41970_e54983, (-(locals.var_sp_s_delta0_dn5 / (locals.var_sp_s_delta0 * locals.var_sp_s_delta0))), (-(locals.var_sp_s_delta0_dn6 / (locals.var_sp_s_delta0 * locals.var_sp_s_delta0))), (-(locals.var_sp_s_delta0_dn7 / (locals.var_sp_s_delta0 * locals.var_sp_s_delta0))), (-(locals.var_sp_s_delta0_dn8 / (locals.var_sp_s_delta0 * locals.var_sp_s_delta0))), (-(locals.var_sp_s_delta0_dn12 / (locals.var_sp_s_delta0 * locals.var_sp_s_delta0))), (-(locals.var_sp_s_delta0_dn13 / (locals.var_sp_s_delta0 * locals.var_sp_s_delta0))), (-(locals.var_sp_s_delta0_dn14 / (locals.var_sp_s_delta0 * locals.var_sp_s_delta0))), (-(locals.var_sp_s_delta0_dn15 / (locals.var_sp_s_delta0 * locals.var_sp_s_delta0))), (-(locals.var_sp_s_delta0_dn16 / (locals.var_sp_s_delta0 * locals.var_sp_s_delta0))), (-(locals.var_sp_s_delta0_dn17 / (locals.var_sp_s_delta0 * locals.var_sp_s_delta0))), (-(locals.var_sp_s_delta0_dn18 / (locals.var_sp_s_delta0 * locals.var_sp_s_delta0))), (-(locals.var_sp_s_delta0_dn19 / (locals.var_sp_s_delta0 * locals.var_sp_s_delta0))), (-(locals.var_sp_s_delta0_dn20 / (locals.var_sp_s_delta0 * locals.var_sp_s_delta0))),)
    } else {
        (locals.var_sp_s_delta1, locals.var_sp_s_delta1_dn5, locals.var_sp_s_delta1_dn6, locals.var_sp_s_delta1_dn7, locals.var_sp_s_delta1_dn8, locals.var_sp_s_delta1_dn12, locals.var_sp_s_delta1_dn13, locals.var_sp_s_delta1_dn14, locals.var_sp_s_delta1_dn15, locals.var_sp_s_delta1_dn16, locals.var_sp_s_delta1_dn17, locals.var_sp_s_delta1_dn18, locals.var_sp_s_delta1_dn19, locals.var_sp_s_delta1_dn20,)
    }
};
        locals.var_sp_s_delta1 = assign41970_e54985;
        locals.var_sp_s_delta1_dn5 = assign41970_e54985_d_n5;
        locals.var_sp_s_delta1_dn6 = assign41970_e54985_d_n6;
        locals.var_sp_s_delta1_dn7 = assign41970_e54985_d_n7;
        locals.var_sp_s_delta1_dn8 = assign41970_e54985_d_n8;
        locals.var_sp_s_delta1_dn12 = assign41970_e54985_d_n12;
        locals.var_sp_s_delta1_dn13 = assign41970_e54985_d_n13;
        locals.var_sp_s_delta1_dn14 = assign41970_e54985_d_n14;
        locals.var_sp_s_delta1_dn15 = assign41970_e54985_d_n15;
        locals.var_sp_s_delta1_dn16 = assign41970_e54985_d_n16;
        locals.var_sp_s_delta1_dn17 = assign41970_e54985_d_n17;
        locals.var_sp_s_delta1_dn18 = assign41970_e54985_d_n18;
        locals.var_sp_s_delta1_dn19 = assign41970_e54985_d_n19;
        locals.var_sp_s_delta1_dn20 = assign41970_e54985_d_n20;
        locals.var_sp_s_delta1_rv = 0.0;

        let (assign41980_e54998, assign41980_e54998_d_n5, assign41980_e54998_d_n6, assign41980_e54998_d_n7, assign41980_e54998_d_n8, assign41980_e54998_d_n12, assign41980_e54998_d_n13, assign41980_e54998_d_n14, assign41980_e54998_d_n15, assign41980_e54998_d_n16, assign41980_e54998_d_n17, assign41980_e54998_d_n18, assign41980_e54998_d_n19, assign41980_e54998_d_n20,) = {
    if ((locals.var_guard1284 == 0.0) && (locals.var_guard1285 != 0.0)) {
        let assign41980_e54994: f64 = (locals.var_sp_s_y0 * locals.var_sp_s_y0);
        let assign41980_e54995: f64 = (2.0 + assign41980_e54994);
        let assign41980_e54996: f64 = (1.0 / assign41980_e54995);
        (assign41980_e54996, (-(((locals.var_sp_s_y0_dn5 * locals.var_sp_s_y0) + (locals.var_sp_s_y0 * locals.var_sp_s_y0_dn5)) / (assign41980_e54995 * assign41980_e54995))), (-(((locals.var_sp_s_y0_dn6 * locals.var_sp_s_y0) + (locals.var_sp_s_y0 * locals.var_sp_s_y0_dn6)) / (assign41980_e54995 * assign41980_e54995))), (-(((locals.var_sp_s_y0_dn7 * locals.var_sp_s_y0) + (locals.var_sp_s_y0 * locals.var_sp_s_y0_dn7)) / (assign41980_e54995 * assign41980_e54995))), (-(((locals.var_sp_s_y0_dn8 * locals.var_sp_s_y0) + (locals.var_sp_s_y0 * locals.var_sp_s_y0_dn8)) / (assign41980_e54995 * assign41980_e54995))), (-(((locals.var_sp_s_y0_dn12 * locals.var_sp_s_y0) + (locals.var_sp_s_y0 * locals.var_sp_s_y0_dn12)) / (assign41980_e54995 * assign41980_e54995))), (-(((locals.var_sp_s_y0_dn13 * locals.var_sp_s_y0) + (locals.var_sp_s_y0 * locals.var_sp_s_y0_dn13)) / (assign41980_e54995 * assign41980_e54995))), (-(((locals.var_sp_s_y0_dn14 * locals.var_sp_s_y0) + (locals.var_sp_s_y0 * locals.var_sp_s_y0_dn14)) / (assign41980_e54995 * assign41980_e54995))), (-(((locals.var_sp_s_y0_dn15 * locals.var_sp_s_y0) + (locals.var_sp_s_y0 * locals.var_sp_s_y0_dn15)) / (assign41980_e54995 * assign41980_e54995))), (-(((locals.var_sp_s_y0_dn16 * locals.var_sp_s_y0) + (locals.var_sp_s_y0 * locals.var_sp_s_y0_dn16)) / (assign41980_e54995 * assign41980_e54995))), (-(((locals.var_sp_s_y0_dn17 * locals.var_sp_s_y0) + (locals.var_sp_s_y0 * locals.var_sp_s_y0_dn17)) / (assign41980_e54995 * assign41980_e54995))), (-(((locals.var_sp_s_y0_dn18 * locals.var_sp_s_y0) + (locals.var_sp_s_y0 * locals.var_sp_s_y0_dn18)) / (assign41980_e54995 * assign41980_e54995))), (-(((locals.var_sp_s_y0_dn19 * locals.var_sp_s_y0) + (locals.var_sp_s_y0 * locals.var_sp_s_y0_dn19)) / (assign41980_e54995 * assign41980_e54995))), (-(((locals.var_sp_s_y0_dn20 * locals.var_sp_s_y0) + (locals.var_sp_s_y0 * locals.var_sp_s_y0_dn20)) / (assign41980_e54995 * assign41980_e54995))),)
    } else {
        (locals.var_sp_s_temp, locals.var_sp_s_temp_dn5, locals.var_sp_s_temp_dn6, locals.var_sp_s_temp_dn7, locals.var_sp_s_temp_dn8, locals.var_sp_s_temp_dn12, locals.var_sp_s_temp_dn13, locals.var_sp_s_temp_dn14, locals.var_sp_s_temp_dn15, locals.var_sp_s_temp_dn16, locals.var_sp_s_temp_dn17, locals.var_sp_s_temp_dn18, locals.var_sp_s_temp_dn19, locals.var_sp_s_temp_dn20,)
    }
};
        locals.var_sp_s_temp = assign41980_e54998;
        locals.var_sp_s_temp_dn5 = assign41980_e54998_d_n5;
        locals.var_sp_s_temp_dn6 = assign41980_e54998_d_n6;
        locals.var_sp_s_temp_dn7 = assign41980_e54998_d_n7;
        locals.var_sp_s_temp_dn8 = assign41980_e54998_d_n8;
        locals.var_sp_s_temp_dn12 = assign41980_e54998_d_n12;
        locals.var_sp_s_temp_dn13 = assign41980_e54998_d_n13;
        locals.var_sp_s_temp_dn14 = assign41980_e54998_d_n14;
        locals.var_sp_s_temp_dn15 = assign41980_e54998_d_n15;
        locals.var_sp_s_temp_dn16 = assign41980_e54998_d_n16;
        locals.var_sp_s_temp_dn17 = assign41980_e54998_d_n17;
        locals.var_sp_s_temp_dn18 = assign41980_e54998_d_n18;
        locals.var_sp_s_temp_dn19 = assign41980_e54998_d_n19;
        locals.var_sp_s_temp_dn20 = assign41980_e54998_d_n20;
        locals.var_sp_s_temp_rv = 0.0;

        let (assign41990_e55009, assign41990_e55009_d_n5, assign41990_e55009_d_n6, assign41990_e55009_d_n7, assign41990_e55009_d_n8, assign41990_e55009_d_n12, assign41990_e55009_d_n13, assign41990_e55009_d_n14, assign41990_e55009_d_n15, assign41990_e55009_d_n16, assign41990_e55009_d_n17, assign41990_e55009_d_n18, assign41990_e55009_d_n19, assign41990_e55009_d_n20,) = {
    if ((locals.var_guard1284 == 0.0) && (locals.var_guard1285 != 0.0)) {
        let assign41990_e55005: f64 = (locals.var_sp_s_y0 * locals.var_sp_s_y0);
        let assign41990_e55007: f64 = (assign41990_e55005 * locals.var_sp_s_temp);
        (assign41990_e55007, ((((locals.var_sp_s_y0_dn5 * locals.var_sp_s_y0) + (locals.var_sp_s_y0 * locals.var_sp_s_y0_dn5)) * locals.var_sp_s_temp) + (assign41990_e55005 * locals.var_sp_s_temp_dn5)), ((((locals.var_sp_s_y0_dn6 * locals.var_sp_s_y0) + (locals.var_sp_s_y0 * locals.var_sp_s_y0_dn6)) * locals.var_sp_s_temp) + (assign41990_e55005 * locals.var_sp_s_temp_dn6)), ((((locals.var_sp_s_y0_dn7 * locals.var_sp_s_y0) + (locals.var_sp_s_y0 * locals.var_sp_s_y0_dn7)) * locals.var_sp_s_temp) + (assign41990_e55005 * locals.var_sp_s_temp_dn7)), ((((locals.var_sp_s_y0_dn8 * locals.var_sp_s_y0) + (locals.var_sp_s_y0 * locals.var_sp_s_y0_dn8)) * locals.var_sp_s_temp) + (assign41990_e55005 * locals.var_sp_s_temp_dn8)), ((((locals.var_sp_s_y0_dn12 * locals.var_sp_s_y0) + (locals.var_sp_s_y0 * locals.var_sp_s_y0_dn12)) * locals.var_sp_s_temp) + (assign41990_e55005 * locals.var_sp_s_temp_dn12)), ((((locals.var_sp_s_y0_dn13 * locals.var_sp_s_y0) + (locals.var_sp_s_y0 * locals.var_sp_s_y0_dn13)) * locals.var_sp_s_temp) + (assign41990_e55005 * locals.var_sp_s_temp_dn13)), ((((locals.var_sp_s_y0_dn14 * locals.var_sp_s_y0) + (locals.var_sp_s_y0 * locals.var_sp_s_y0_dn14)) * locals.var_sp_s_temp) + (assign41990_e55005 * locals.var_sp_s_temp_dn14)), ((((locals.var_sp_s_y0_dn15 * locals.var_sp_s_y0) + (locals.var_sp_s_y0 * locals.var_sp_s_y0_dn15)) * locals.var_sp_s_temp) + (assign41990_e55005 * locals.var_sp_s_temp_dn15)), ((((locals.var_sp_s_y0_dn16 * locals.var_sp_s_y0) + (locals.var_sp_s_y0 * locals.var_sp_s_y0_dn16)) * locals.var_sp_s_temp) + (assign41990_e55005 * locals.var_sp_s_temp_dn16)), ((((locals.var_sp_s_y0_dn17 * locals.var_sp_s_y0) + (locals.var_sp_s_y0 * locals.var_sp_s_y0_dn17)) * locals.var_sp_s_temp) + (assign41990_e55005 * locals.var_sp_s_temp_dn17)), ((((locals.var_sp_s_y0_dn18 * locals.var_sp_s_y0) + (locals.var_sp_s_y0 * locals.var_sp_s_y0_dn18)) * locals.var_sp_s_temp) + (assign41990_e55005 * locals.var_sp_s_temp_dn18)), ((((locals.var_sp_s_y0_dn19 * locals.var_sp_s_y0) + (locals.var_sp_s_y0 * locals.var_sp_s_y0_dn19)) * locals.var_sp_s_temp) + (assign41990_e55005 * locals.var_sp_s_temp_dn19)), ((((locals.var_sp_s_y0_dn20 * locals.var_sp_s_y0) + (locals.var_sp_s_y0 * locals.var_sp_s_y0_dn20)) * locals.var_sp_s_temp) + (assign41990_e55005 * locals.var_sp_s_temp_dn20)),)
    } else {
        (locals.var_sp_s_xi0, locals.var_sp_s_xi0_dn5, locals.var_sp_s_xi0_dn6, locals.var_sp_s_xi0_dn7, locals.var_sp_s_xi0_dn8, locals.var_sp_s_xi0_dn12, locals.var_sp_s_xi0_dn13, locals.var_sp_s_xi0_dn14, locals.var_sp_s_xi0_dn15, locals.var_sp_s_xi0_dn16, locals.var_sp_s_xi0_dn17, locals.var_sp_s_xi0_dn18, locals.var_sp_s_xi0_dn19, locals.var_sp_s_xi0_dn20,)
    }
};
        locals.var_sp_s_xi0 = assign41990_e55009;
        locals.var_sp_s_xi0_dn5 = assign41990_e55009_d_n5;
        locals.var_sp_s_xi0_dn6 = assign41990_e55009_d_n6;
        locals.var_sp_s_xi0_dn7 = assign41990_e55009_d_n7;
        locals.var_sp_s_xi0_dn8 = assign41990_e55009_d_n8;
        locals.var_sp_s_xi0_dn12 = assign41990_e55009_d_n12;
        locals.var_sp_s_xi0_dn13 = assign41990_e55009_d_n13;
        locals.var_sp_s_xi0_dn14 = assign41990_e55009_d_n14;
        locals.var_sp_s_xi0_dn15 = assign41990_e55009_d_n15;
        locals.var_sp_s_xi0_dn16 = assign41990_e55009_d_n16;
        locals.var_sp_s_xi0_dn17 = assign41990_e55009_d_n17;
        locals.var_sp_s_xi0_dn18 = assign41990_e55009_d_n18;
        locals.var_sp_s_xi0_dn19 = assign41990_e55009_d_n19;
        locals.var_sp_s_xi0_dn20 = assign41990_e55009_d_n20;
        locals.var_sp_s_xi0_rv = 0.0;

        let (assign42000_e55022, assign42000_e55022_d_n5, assign42000_e55022_d_n6, assign42000_e55022_d_n7, assign42000_e55022_d_n8, assign42000_e55022_d_n12, assign42000_e55022_d_n13, assign42000_e55022_d_n14, assign42000_e55022_d_n15, assign42000_e55022_d_n16, assign42000_e55022_d_n17, assign42000_e55022_d_n18, assign42000_e55022_d_n19, assign42000_e55022_d_n20,) = {
    if ((locals.var_guard1284 == 0.0) && (locals.var_guard1285 != 0.0)) {
        let assign42000_e55017: f64 = (locals.var_sp_s_y0 * locals.var_sp_s_temp);
        let assign42000_e55019: f64 = (assign42000_e55017 * locals.var_sp_s_temp);
        let assign42000_e55020: f64 = (4.0 * assign42000_e55019);
        (assign42000_e55020, (4.0 * ((((locals.var_sp_s_y0_dn5 * locals.var_sp_s_temp) + (locals.var_sp_s_y0 * locals.var_sp_s_temp_dn5)) * locals.var_sp_s_temp) + (assign42000_e55017 * locals.var_sp_s_temp_dn5))), (4.0 * ((((locals.var_sp_s_y0_dn6 * locals.var_sp_s_temp) + (locals.var_sp_s_y0 * locals.var_sp_s_temp_dn6)) * locals.var_sp_s_temp) + (assign42000_e55017 * locals.var_sp_s_temp_dn6))), (4.0 * ((((locals.var_sp_s_y0_dn7 * locals.var_sp_s_temp) + (locals.var_sp_s_y0 * locals.var_sp_s_temp_dn7)) * locals.var_sp_s_temp) + (assign42000_e55017 * locals.var_sp_s_temp_dn7))), (4.0 * ((((locals.var_sp_s_y0_dn8 * locals.var_sp_s_temp) + (locals.var_sp_s_y0 * locals.var_sp_s_temp_dn8)) * locals.var_sp_s_temp) + (assign42000_e55017 * locals.var_sp_s_temp_dn8))), (4.0 * ((((locals.var_sp_s_y0_dn12 * locals.var_sp_s_temp) + (locals.var_sp_s_y0 * locals.var_sp_s_temp_dn12)) * locals.var_sp_s_temp) + (assign42000_e55017 * locals.var_sp_s_temp_dn12))), (4.0 * ((((locals.var_sp_s_y0_dn13 * locals.var_sp_s_temp) + (locals.var_sp_s_y0 * locals.var_sp_s_temp_dn13)) * locals.var_sp_s_temp) + (assign42000_e55017 * locals.var_sp_s_temp_dn13))), (4.0 * ((((locals.var_sp_s_y0_dn14 * locals.var_sp_s_temp) + (locals.var_sp_s_y0 * locals.var_sp_s_temp_dn14)) * locals.var_sp_s_temp) + (assign42000_e55017 * locals.var_sp_s_temp_dn14))), (4.0 * ((((locals.var_sp_s_y0_dn15 * locals.var_sp_s_temp) + (locals.var_sp_s_y0 * locals.var_sp_s_temp_dn15)) * locals.var_sp_s_temp) + (assign42000_e55017 * locals.var_sp_s_temp_dn15))), (4.0 * ((((locals.var_sp_s_y0_dn16 * locals.var_sp_s_temp) + (locals.var_sp_s_y0 * locals.var_sp_s_temp_dn16)) * locals.var_sp_s_temp) + (assign42000_e55017 * locals.var_sp_s_temp_dn16))), (4.0 * ((((locals.var_sp_s_y0_dn17 * locals.var_sp_s_temp) + (locals.var_sp_s_y0 * locals.var_sp_s_temp_dn17)) * locals.var_sp_s_temp) + (assign42000_e55017 * locals.var_sp_s_temp_dn17))), (4.0 * ((((locals.var_sp_s_y0_dn18 * locals.var_sp_s_temp) + (locals.var_sp_s_y0 * locals.var_sp_s_temp_dn18)) * locals.var_sp_s_temp) + (assign42000_e55017 * locals.var_sp_s_temp_dn18))), (4.0 * ((((locals.var_sp_s_y0_dn19 * locals.var_sp_s_temp) + (locals.var_sp_s_y0 * locals.var_sp_s_temp_dn19)) * locals.var_sp_s_temp) + (assign42000_e55017 * locals.var_sp_s_temp_dn19))), (4.0 * ((((locals.var_sp_s_y0_dn20 * locals.var_sp_s_temp) + (locals.var_sp_s_y0 * locals.var_sp_s_temp_dn20)) * locals.var_sp_s_temp) + (assign42000_e55017 * locals.var_sp_s_temp_dn20))),)
    } else {
        (locals.var_sp_s_xi1, locals.var_sp_s_xi1_dn5, locals.var_sp_s_xi1_dn6, locals.var_sp_s_xi1_dn7, locals.var_sp_s_xi1_dn8, locals.var_sp_s_xi1_dn12, locals.var_sp_s_xi1_dn13, locals.var_sp_s_xi1_dn14, locals.var_sp_s_xi1_dn15, locals.var_sp_s_xi1_dn16, locals.var_sp_s_xi1_dn17, locals.var_sp_s_xi1_dn18, locals.var_sp_s_xi1_dn19, locals.var_sp_s_xi1_dn20,)
    }
};
        locals.var_sp_s_xi1 = assign42000_e55022;
        locals.var_sp_s_xi1_dn5 = assign42000_e55022_d_n5;
        locals.var_sp_s_xi1_dn6 = assign42000_e55022_d_n6;
        locals.var_sp_s_xi1_dn7 = assign42000_e55022_d_n7;
        locals.var_sp_s_xi1_dn8 = assign42000_e55022_d_n8;
        locals.var_sp_s_xi1_dn12 = assign42000_e55022_d_n12;
        locals.var_sp_s_xi1_dn13 = assign42000_e55022_d_n13;
        locals.var_sp_s_xi1_dn14 = assign42000_e55022_d_n14;
        locals.var_sp_s_xi1_dn15 = assign42000_e55022_d_n15;
        locals.var_sp_s_xi1_dn16 = assign42000_e55022_d_n16;
        locals.var_sp_s_xi1_dn17 = assign42000_e55022_d_n17;
        locals.var_sp_s_xi1_dn18 = assign42000_e55022_d_n18;
        locals.var_sp_s_xi1_dn19 = assign42000_e55022_d_n19;
        locals.var_sp_s_xi1_dn20 = assign42000_e55022_d_n20;
        locals.var_sp_s_xi1_rv = 0.0;

        let (assign42010_e55039, assign42010_e55039_d_n5, assign42010_e55039_d_n6, assign42010_e55039_d_n7, assign42010_e55039_d_n8, assign42010_e55039_d_n12, assign42010_e55039_d_n13, assign42010_e55039_d_n14, assign42010_e55039_d_n15, assign42010_e55039_d_n16, assign42010_e55039_d_n17, assign42010_e55039_d_n18, assign42010_e55039_d_n19, assign42010_e55039_d_n20,) = {
    if ((locals.var_guard1284 == 0.0) && (locals.var_guard1285 != 0.0)) {
        let assign42010_e55029: f64 = (8.0 * locals.var_sp_s_temp);
        let assign42010_e55032: f64 = (12.0 * locals.var_sp_s_xi0);
        let assign42010_e55033: f64 = (assign42010_e55029 - assign42010_e55032);
        let assign42010_e55035: f64 = (assign42010_e55033 * locals.var_sp_s_temp);
        let assign42010_e55037: f64 = (assign42010_e55035 * locals.var_sp_s_temp);
        (assign42010_e55037, ((((((8.0 * locals.var_sp_s_temp_dn5) - (12.0 * locals.var_sp_s_xi0_dn5)) * locals.var_sp_s_temp) + (assign42010_e55033 * locals.var_sp_s_temp_dn5)) * locals.var_sp_s_temp) + (assign42010_e55035 * locals.var_sp_s_temp_dn5)), ((((((8.0 * locals.var_sp_s_temp_dn6) - (12.0 * locals.var_sp_s_xi0_dn6)) * locals.var_sp_s_temp) + (assign42010_e55033 * locals.var_sp_s_temp_dn6)) * locals.var_sp_s_temp) + (assign42010_e55035 * locals.var_sp_s_temp_dn6)), ((((((8.0 * locals.var_sp_s_temp_dn7) - (12.0 * locals.var_sp_s_xi0_dn7)) * locals.var_sp_s_temp) + (assign42010_e55033 * locals.var_sp_s_temp_dn7)) * locals.var_sp_s_temp) + (assign42010_e55035 * locals.var_sp_s_temp_dn7)), ((((((8.0 * locals.var_sp_s_temp_dn8) - (12.0 * locals.var_sp_s_xi0_dn8)) * locals.var_sp_s_temp) + (assign42010_e55033 * locals.var_sp_s_temp_dn8)) * locals.var_sp_s_temp) + (assign42010_e55035 * locals.var_sp_s_temp_dn8)), ((((((8.0 * locals.var_sp_s_temp_dn12) - (12.0 * locals.var_sp_s_xi0_dn12)) * locals.var_sp_s_temp) + (assign42010_e55033 * locals.var_sp_s_temp_dn12)) * locals.var_sp_s_temp) + (assign42010_e55035 * locals.var_sp_s_temp_dn12)), ((((((8.0 * locals.var_sp_s_temp_dn13) - (12.0 * locals.var_sp_s_xi0_dn13)) * locals.var_sp_s_temp) + (assign42010_e55033 * locals.var_sp_s_temp_dn13)) * locals.var_sp_s_temp) + (assign42010_e55035 * locals.var_sp_s_temp_dn13)), ((((((8.0 * locals.var_sp_s_temp_dn14) - (12.0 * locals.var_sp_s_xi0_dn14)) * locals.var_sp_s_temp) + (assign42010_e55033 * locals.var_sp_s_temp_dn14)) * locals.var_sp_s_temp) + (assign42010_e55035 * locals.var_sp_s_temp_dn14)), ((((((8.0 * locals.var_sp_s_temp_dn15) - (12.0 * locals.var_sp_s_xi0_dn15)) * locals.var_sp_s_temp) + (assign42010_e55033 * locals.var_sp_s_temp_dn15)) * locals.var_sp_s_temp) + (assign42010_e55035 * locals.var_sp_s_temp_dn15)), ((((((8.0 * locals.var_sp_s_temp_dn16) - (12.0 * locals.var_sp_s_xi0_dn16)) * locals.var_sp_s_temp) + (assign42010_e55033 * locals.var_sp_s_temp_dn16)) * locals.var_sp_s_temp) + (assign42010_e55035 * locals.var_sp_s_temp_dn16)), ((((((8.0 * locals.var_sp_s_temp_dn17) - (12.0 * locals.var_sp_s_xi0_dn17)) * locals.var_sp_s_temp) + (assign42010_e55033 * locals.var_sp_s_temp_dn17)) * locals.var_sp_s_temp) + (assign42010_e55035 * locals.var_sp_s_temp_dn17)), ((((((8.0 * locals.var_sp_s_temp_dn18) - (12.0 * locals.var_sp_s_xi0_dn18)) * locals.var_sp_s_temp) + (assign42010_e55033 * locals.var_sp_s_temp_dn18)) * locals.var_sp_s_temp) + (assign42010_e55035 * locals.var_sp_s_temp_dn18)), ((((((8.0 * locals.var_sp_s_temp_dn19) - (12.0 * locals.var_sp_s_xi0_dn19)) * locals.var_sp_s_temp) + (assign42010_e55033 * locals.var_sp_s_temp_dn19)) * locals.var_sp_s_temp) + (assign42010_e55035 * locals.var_sp_s_temp_dn19)), ((((((8.0 * locals.var_sp_s_temp_dn20) - (12.0 * locals.var_sp_s_xi0_dn20)) * locals.var_sp_s_temp) + (assign42010_e55033 * locals.var_sp_s_temp_dn20)) * locals.var_sp_s_temp) + (assign42010_e55035 * locals.var_sp_s_temp_dn20)),)
    } else {
        (locals.var_sp_s_xi2, locals.var_sp_s_xi2_dn5, locals.var_sp_s_xi2_dn6, locals.var_sp_s_xi2_dn7, locals.var_sp_s_xi2_dn8, locals.var_sp_s_xi2_dn12, locals.var_sp_s_xi2_dn13, locals.var_sp_s_xi2_dn14, locals.var_sp_s_xi2_dn15, locals.var_sp_s_xi2_dn16, locals.var_sp_s_xi2_dn17, locals.var_sp_s_xi2_dn18, locals.var_sp_s_xi2_dn19, locals.var_sp_s_xi2_dn20,)
    }
};
        locals.var_sp_s_xi2 = assign42010_e55039;
        locals.var_sp_s_xi2_dn5 = assign42010_e55039_d_n5;
        locals.var_sp_s_xi2_dn6 = assign42010_e55039_d_n6;
        locals.var_sp_s_xi2_dn7 = assign42010_e55039_d_n7;
        locals.var_sp_s_xi2_dn8 = assign42010_e55039_d_n8;
        locals.var_sp_s_xi2_dn12 = assign42010_e55039_d_n12;
        locals.var_sp_s_xi2_dn13 = assign42010_e55039_d_n13;
        locals.var_sp_s_xi2_dn14 = assign42010_e55039_d_n14;
        locals.var_sp_s_xi2_dn15 = assign42010_e55039_d_n15;
        locals.var_sp_s_xi2_dn16 = assign42010_e55039_d_n16;
        locals.var_sp_s_xi2_dn17 = assign42010_e55039_d_n17;
        locals.var_sp_s_xi2_dn18 = assign42010_e55039_d_n18;
        locals.var_sp_s_xi2_dn19 = assign42010_e55039_d_n19;
        locals.var_sp_s_xi2_dn20 = assign42010_e55039_d_n20;
        locals.var_sp_s_xi2_rv = 0.0;

        let (assign42020_e55048, assign42020_e55048_d_n5, assign42020_e55048_d_n6, assign42020_e55048_d_n7, assign42020_e55048_d_n8, assign42020_e55048_d_n12, assign42020_e55048_d_n13, assign42020_e55048_d_n14, assign42020_e55048_d_n15, assign42020_e55048_d_n16, assign42020_e55048_d_n17, assign42020_e55048_d_n18, assign42020_e55048_d_n19, assign42020_e55048_d_n20,) = {
    if ((locals.var_guard1284 == 0.0) && (locals.var_guard1285 != 0.0)) {
        let assign42020_e55046: f64 = (locals.var_sp_s_yg - locals.var_sp_s_y0);
        (assign42020_e55046, (locals.var_sp_s_yg_dn5 - locals.var_sp_s_y0_dn5), (locals.var_sp_s_yg_dn6 - locals.var_sp_s_y0_dn6), (locals.var_sp_s_yg_dn7 - locals.var_sp_s_y0_dn7), (locals.var_sp_s_yg_dn8 - locals.var_sp_s_y0_dn8), (locals.var_sp_s_yg_dn12 - locals.var_sp_s_y0_dn12), (locals.var_sp_s_yg_dn13 - locals.var_sp_s_y0_dn13), (locals.var_sp_s_yg_dn14 - locals.var_sp_s_y0_dn14), (locals.var_sp_s_yg_dn15 - locals.var_sp_s_y0_dn15), (locals.var_sp_s_yg_dn16 - locals.var_sp_s_y0_dn16), (locals.var_sp_s_yg_dn17 - locals.var_sp_s_y0_dn17), (locals.var_sp_s_yg_dn18 - locals.var_sp_s_y0_dn18), (locals.var_sp_s_yg_dn19 - locals.var_sp_s_y0_dn19), (locals.var_sp_s_yg_dn20 - locals.var_sp_s_y0_dn20),)
    } else {
        (locals.var_sp_s_temp, locals.var_sp_s_temp_dn5, locals.var_sp_s_temp_dn6, locals.var_sp_s_temp_dn7, locals.var_sp_s_temp_dn8, locals.var_sp_s_temp_dn12, locals.var_sp_s_temp_dn13, locals.var_sp_s_temp_dn14, locals.var_sp_s_temp_dn15, locals.var_sp_s_temp_dn16, locals.var_sp_s_temp_dn17, locals.var_sp_s_temp_dn18, locals.var_sp_s_temp_dn19, locals.var_sp_s_temp_dn20,)
    }
};
        locals.var_sp_s_temp = assign42020_e55048;
        locals.var_sp_s_temp_dn5 = assign42020_e55048_d_n5;
        locals.var_sp_s_temp_dn6 = assign42020_e55048_d_n6;
        locals.var_sp_s_temp_dn7 = assign42020_e55048_d_n7;
        locals.var_sp_s_temp_dn8 = assign42020_e55048_d_n8;
        locals.var_sp_s_temp_dn12 = assign42020_e55048_d_n12;
        locals.var_sp_s_temp_dn13 = assign42020_e55048_d_n13;
        locals.var_sp_s_temp_dn14 = assign42020_e55048_d_n14;
        locals.var_sp_s_temp_dn15 = assign42020_e55048_d_n15;
        locals.var_sp_s_temp_dn16 = assign42020_e55048_d_n16;
        locals.var_sp_s_temp_dn17 = assign42020_e55048_d_n17;
        locals.var_sp_s_temp_dn18 = assign42020_e55048_d_n18;
        locals.var_sp_s_temp_dn19 = assign42020_e55048_d_n19;
        locals.var_sp_s_temp_dn20 = assign42020_e55048_d_n20;
        locals.var_sp_s_temp_rv = 0.0;

        let (assign42030_e55057, assign42030_e55057_d_n5, assign42030_e55057_d_n6, assign42030_e55057_d_n7, assign42030_e55057_d_n8, assign42030_e55057_d_n12, assign42030_e55057_d_n13, assign42030_e55057_d_n14, assign42030_e55057_d_n15, assign42030_e55057_d_n16, assign42030_e55057_d_n17, assign42030_e55057_d_n18, assign42030_e55057_d_n19, assign42030_e55057_d_n20,) = {
    if ((locals.var_guard1284 == 0.0) && (locals.var_guard1285 != 0.0)) {
        let assign42030_e55055: f64 = (locals.var_delta_ns * locals.var_sp_s_delta1);
        (assign42030_e55055, ((locals.var_delta_ns_dn5 * locals.var_sp_s_delta1) + (locals.var_delta_ns * locals.var_sp_s_delta1_dn5)), ((locals.var_delta_ns_dn6 * locals.var_sp_s_delta1) + (locals.var_delta_ns * locals.var_sp_s_delta1_dn6)), ((locals.var_delta_ns_dn7 * locals.var_sp_s_delta1) + (locals.var_delta_ns * locals.var_sp_s_delta1_dn7)), ((locals.var_delta_ns_dn8 * locals.var_sp_s_delta1) + (locals.var_delta_ns * locals.var_sp_s_delta1_dn8)), ((locals.var_delta_ns_dn12 * locals.var_sp_s_delta1) + (locals.var_delta_ns * locals.var_sp_s_delta1_dn12)), ((locals.var_delta_ns_dn13 * locals.var_sp_s_delta1) + (locals.var_delta_ns * locals.var_sp_s_delta1_dn13)), ((locals.var_delta_ns_dn14 * locals.var_sp_s_delta1) + (locals.var_delta_ns * locals.var_sp_s_delta1_dn14)), ((locals.var_delta_ns_dn15 * locals.var_sp_s_delta1) + (locals.var_delta_ns * locals.var_sp_s_delta1_dn15)), ((locals.var_delta_ns_dn16 * locals.var_sp_s_delta1) + (locals.var_delta_ns * locals.var_sp_s_delta1_dn16)), ((locals.var_delta_ns_dn17 * locals.var_sp_s_delta1) + (locals.var_delta_ns * locals.var_sp_s_delta1_dn17)), ((locals.var_delta_ns_dn18 * locals.var_sp_s_delta1) + (locals.var_delta_ns * locals.var_sp_s_delta1_dn18)), ((locals.var_delta_ns_dn19 * locals.var_sp_s_delta1) + (locals.var_delta_ns * locals.var_sp_s_delta1_dn19)), ((locals.var_delta_ns_dn20 * locals.var_sp_s_delta1) + (locals.var_delta_ns * locals.var_sp_s_delta1_dn20)),)
    } else {
        (locals.var_sp_s_temp1, locals.var_sp_s_temp1_dn5, locals.var_sp_s_temp1_dn6, locals.var_sp_s_temp1_dn7, locals.var_sp_s_temp1_dn8, locals.var_sp_s_temp1_dn12, locals.var_sp_s_temp1_dn13, locals.var_sp_s_temp1_dn14, locals.var_sp_s_temp1_dn15, locals.var_sp_s_temp1_dn16, locals.var_sp_s_temp1_dn17, locals.var_sp_s_temp1_dn18, locals.var_sp_s_temp1_dn19, locals.var_sp_s_temp1_dn20,)
    }
};
        locals.var_sp_s_temp1 = assign42030_e55057;
        locals.var_sp_s_temp1_dn5 = assign42030_e55057_d_n5;
        locals.var_sp_s_temp1_dn6 = assign42030_e55057_d_n6;
        locals.var_sp_s_temp1_dn7 = assign42030_e55057_d_n7;
        locals.var_sp_s_temp1_dn8 = assign42030_e55057_d_n8;
        locals.var_sp_s_temp1_dn12 = assign42030_e55057_d_n12;
        locals.var_sp_s_temp1_dn13 = assign42030_e55057_d_n13;
        locals.var_sp_s_temp1_dn14 = assign42030_e55057_d_n14;
        locals.var_sp_s_temp1_dn15 = assign42030_e55057_d_n15;
        locals.var_sp_s_temp1_dn16 = assign42030_e55057_d_n16;
        locals.var_sp_s_temp1_dn17 = assign42030_e55057_d_n17;
        locals.var_sp_s_temp1_dn18 = assign42030_e55057_d_n18;
        locals.var_sp_s_temp1_dn19 = assign42030_e55057_d_n19;
        locals.var_sp_s_temp1_dn20 = assign42030_e55057_d_n20;
        locals.var_sp_s_temp1_rv = 0.0;

        let (assign42040_e55080, assign42040_e55080_d_n5, assign42040_e55080_d_n6, assign42040_e55080_d_n7, assign42040_e55080_d_n8, assign42040_e55080_d_n12, assign42040_e55080_d_n13, assign42040_e55080_d_n14, assign42040_e55080_d_n15, assign42040_e55080_d_n16, assign42040_e55080_d_n17, assign42040_e55080_d_n18, assign42040_e55080_d_n19, assign42040_e55080_d_n20,) = {
    if ((locals.var_guard1284 == 0.0) && (locals.var_guard1285 != 0.0)) {
        let assign42040_e55064: f64 = (2.0 * locals.var_sp_s_temp);
        let assign42040_e55068: f64 = (locals.var_sp_s_delta0 - 1.0);
        let assign42040_e55070: f64 = (assign42040_e55068 - locals.var_sp_s_temp1);
        let assign42040_e55074: f64 = (1.0 - locals.var_sp_s_xi1);
        let assign42040_e55075: f64 = (locals.var_delta_ns * assign42040_e55074);
        let assign42040_e55076: f64 = (assign42040_e55070 + assign42040_e55075);
        let assign42040_e55077: f64 = (locals.var_gf2 * assign42040_e55076);
        let assign42040_e55078: f64 = (assign42040_e55064 + assign42040_e55077);
        (assign42040_e55078, ((2.0 * locals.var_sp_s_temp_dn5) + ((locals.var_gf2_dn5 * assign42040_e55076) + (locals.var_gf2 * ((locals.var_sp_s_delta0_dn5 - locals.var_sp_s_temp1_dn5) + ((locals.var_delta_ns_dn5 * assign42040_e55074) + (locals.var_delta_ns * (-locals.var_sp_s_xi1_dn5))))))), ((2.0 * locals.var_sp_s_temp_dn6) + ((locals.var_gf2_dn6 * assign42040_e55076) + (locals.var_gf2 * ((locals.var_sp_s_delta0_dn6 - locals.var_sp_s_temp1_dn6) + ((locals.var_delta_ns_dn6 * assign42040_e55074) + (locals.var_delta_ns * (-locals.var_sp_s_xi1_dn6))))))), ((2.0 * locals.var_sp_s_temp_dn7) + ((locals.var_gf2_dn7 * assign42040_e55076) + (locals.var_gf2 * ((locals.var_sp_s_delta0_dn7 - locals.var_sp_s_temp1_dn7) + ((locals.var_delta_ns_dn7 * assign42040_e55074) + (locals.var_delta_ns * (-locals.var_sp_s_xi1_dn7))))))), ((2.0 * locals.var_sp_s_temp_dn8) + ((locals.var_gf2_dn8 * assign42040_e55076) + (locals.var_gf2 * ((locals.var_sp_s_delta0_dn8 - locals.var_sp_s_temp1_dn8) + ((locals.var_delta_ns_dn8 * assign42040_e55074) + (locals.var_delta_ns * (-locals.var_sp_s_xi1_dn8))))))), ((2.0 * locals.var_sp_s_temp_dn12) + ((locals.var_gf2_dn12 * assign42040_e55076) + (locals.var_gf2 * ((locals.var_sp_s_delta0_dn12 - locals.var_sp_s_temp1_dn12) + ((locals.var_delta_ns_dn12 * assign42040_e55074) + (locals.var_delta_ns * (-locals.var_sp_s_xi1_dn12))))))), ((2.0 * locals.var_sp_s_temp_dn13) + ((locals.var_gf2_dn13 * assign42040_e55076) + (locals.var_gf2 * ((locals.var_sp_s_delta0_dn13 - locals.var_sp_s_temp1_dn13) + ((locals.var_delta_ns_dn13 * assign42040_e55074) + (locals.var_delta_ns * (-locals.var_sp_s_xi1_dn13))))))), ((2.0 * locals.var_sp_s_temp_dn14) + ((locals.var_gf2_dn14 * assign42040_e55076) + (locals.var_gf2 * ((locals.var_sp_s_delta0_dn14 - locals.var_sp_s_temp1_dn14) + ((locals.var_delta_ns_dn14 * assign42040_e55074) + (locals.var_delta_ns * (-locals.var_sp_s_xi1_dn14))))))), ((2.0 * locals.var_sp_s_temp_dn15) + ((locals.var_gf2_dn15 * assign42040_e55076) + (locals.var_gf2 * ((locals.var_sp_s_delta0_dn15 - locals.var_sp_s_temp1_dn15) + ((locals.var_delta_ns_dn15 * assign42040_e55074) + (locals.var_delta_ns * (-locals.var_sp_s_xi1_dn15))))))), ((2.0 * locals.var_sp_s_temp_dn16) + ((locals.var_gf2_dn16 * assign42040_e55076) + (locals.var_gf2 * ((locals.var_sp_s_delta0_dn16 - locals.var_sp_s_temp1_dn16) + ((locals.var_delta_ns_dn16 * assign42040_e55074) + (locals.var_delta_ns * (-locals.var_sp_s_xi1_dn16))))))), ((2.0 * locals.var_sp_s_temp_dn17) + ((locals.var_gf2_dn17 * assign42040_e55076) + (locals.var_gf2 * ((locals.var_sp_s_delta0_dn17 - locals.var_sp_s_temp1_dn17) + ((locals.var_delta_ns_dn17 * assign42040_e55074) + (locals.var_delta_ns * (-locals.var_sp_s_xi1_dn17))))))), ((2.0 * locals.var_sp_s_temp_dn18) + ((locals.var_gf2_dn18 * assign42040_e55076) + (locals.var_gf2 * ((locals.var_sp_s_delta0_dn18 - locals.var_sp_s_temp1_dn18) + ((locals.var_delta_ns_dn18 * assign42040_e55074) + (locals.var_delta_ns * (-locals.var_sp_s_xi1_dn18))))))), ((2.0 * locals.var_sp_s_temp_dn19) + ((locals.var_gf2_dn19 * assign42040_e55076) + (locals.var_gf2 * ((locals.var_sp_s_delta0_dn19 - locals.var_sp_s_temp1_dn19) + ((locals.var_delta_ns_dn19 * assign42040_e55074) + (locals.var_delta_ns * (-locals.var_sp_s_xi1_dn19))))))), ((2.0 * locals.var_sp_s_temp_dn20) + ((locals.var_gf2_dn20 * assign42040_e55076) + (locals.var_gf2 * ((locals.var_sp_s_delta0_dn20 - locals.var_sp_s_temp1_dn20) + ((locals.var_delta_ns_dn20 * assign42040_e55074) + (locals.var_delta_ns * (-locals.var_sp_s_xi1_dn20))))))),)
    } else {
        (locals.var_sp_s_pc, locals.var_sp_s_pc_dn5, locals.var_sp_s_pc_dn6, locals.var_sp_s_pc_dn7, locals.var_sp_s_pc_dn8, locals.var_sp_s_pc_dn12, locals.var_sp_s_pc_dn13, locals.var_sp_s_pc_dn14, locals.var_sp_s_pc_dn15, locals.var_sp_s_pc_dn16, locals.var_sp_s_pc_dn17, locals.var_sp_s_pc_dn18, locals.var_sp_s_pc_dn19, locals.var_sp_s_pc_dn20,)
    }
};
        locals.var_sp_s_pc = assign42040_e55080;
        locals.var_sp_s_pc_dn5 = assign42040_e55080_d_n5;
        locals.var_sp_s_pc_dn6 = assign42040_e55080_d_n6;
        locals.var_sp_s_pc_dn7 = assign42040_e55080_d_n7;
        locals.var_sp_s_pc_dn8 = assign42040_e55080_d_n8;
        locals.var_sp_s_pc_dn12 = assign42040_e55080_d_n12;
        locals.var_sp_s_pc_dn13 = assign42040_e55080_d_n13;
        locals.var_sp_s_pc_dn14 = assign42040_e55080_d_n14;
        locals.var_sp_s_pc_dn15 = assign42040_e55080_d_n15;
        locals.var_sp_s_pc_dn16 = assign42040_e55080_d_n16;
        locals.var_sp_s_pc_dn17 = assign42040_e55080_d_n17;
        locals.var_sp_s_pc_dn18 = assign42040_e55080_d_n18;
        locals.var_sp_s_pc_dn19 = assign42040_e55080_d_n19;
        locals.var_sp_s_pc_dn20 = assign42040_e55080_d_n20;
        locals.var_sp_s_pc_rv = 0.0;

        let (assign42050_e55107, assign42050_e55107_d_n5, assign42050_e55107_d_n6, assign42050_e55107_d_n7, assign42050_e55107_d_n8, assign42050_e55107_d_n12, assign42050_e55107_d_n13, assign42050_e55107_d_n14, assign42050_e55107_d_n15, assign42050_e55107_d_n16, assign42050_e55107_d_n17, assign42050_e55107_d_n18, assign42050_e55107_d_n19, assign42050_e55107_d_n20,) = {
    if ((locals.var_guard1284 == 0.0) && (locals.var_guard1285 != 0.0)) {
        let assign42050_e55087: f64 = (locals.var_sp_s_temp * locals.var_sp_s_temp);
        let assign42050_e55091: f64 = (locals.var_sp_s_delta0 - locals.var_sp_s_y0);
        let assign42050_e55093: f64 = (assign42050_e55091 - 1.0);
        let assign42050_e55095: f64 = (assign42050_e55093 + locals.var_sp_s_temp1);
        let assign42050_e55099: f64 = (locals.var_sp_s_y0 - 1.0);
        let assign42050_e55101: f64 = (assign42050_e55099 - locals.var_sp_s_xi0);
        let assign42050_e55102: f64 = (locals.var_delta_ns * assign42050_e55101);
        let assign42050_e55103: f64 = (assign42050_e55095 + assign42050_e55102);
        let assign42050_e55104: f64 = (locals.var_gf2 * assign42050_e55103);
        let assign42050_e55105: f64 = (assign42050_e55087 - assign42050_e55104);
        (assign42050_e55105, (((locals.var_sp_s_temp_dn5 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn5)) - ((locals.var_gf2_dn5 * assign42050_e55103) + (locals.var_gf2 * (((locals.var_sp_s_delta0_dn5 - locals.var_sp_s_y0_dn5) + locals.var_sp_s_temp1_dn5) + ((locals.var_delta_ns_dn5 * assign42050_e55101) + (locals.var_delta_ns * (locals.var_sp_s_y0_dn5 - locals.var_sp_s_xi0_dn5))))))), (((locals.var_sp_s_temp_dn6 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn6)) - ((locals.var_gf2_dn6 * assign42050_e55103) + (locals.var_gf2 * (((locals.var_sp_s_delta0_dn6 - locals.var_sp_s_y0_dn6) + locals.var_sp_s_temp1_dn6) + ((locals.var_delta_ns_dn6 * assign42050_e55101) + (locals.var_delta_ns * (locals.var_sp_s_y0_dn6 - locals.var_sp_s_xi0_dn6))))))), (((locals.var_sp_s_temp_dn7 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn7)) - ((locals.var_gf2_dn7 * assign42050_e55103) + (locals.var_gf2 * (((locals.var_sp_s_delta0_dn7 - locals.var_sp_s_y0_dn7) + locals.var_sp_s_temp1_dn7) + ((locals.var_delta_ns_dn7 * assign42050_e55101) + (locals.var_delta_ns * (locals.var_sp_s_y0_dn7 - locals.var_sp_s_xi0_dn7))))))), (((locals.var_sp_s_temp_dn8 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn8)) - ((locals.var_gf2_dn8 * assign42050_e55103) + (locals.var_gf2 * (((locals.var_sp_s_delta0_dn8 - locals.var_sp_s_y0_dn8) + locals.var_sp_s_temp1_dn8) + ((locals.var_delta_ns_dn8 * assign42050_e55101) + (locals.var_delta_ns * (locals.var_sp_s_y0_dn8 - locals.var_sp_s_xi0_dn8))))))), (((locals.var_sp_s_temp_dn12 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn12)) - ((locals.var_gf2_dn12 * assign42050_e55103) + (locals.var_gf2 * (((locals.var_sp_s_delta0_dn12 - locals.var_sp_s_y0_dn12) + locals.var_sp_s_temp1_dn12) + ((locals.var_delta_ns_dn12 * assign42050_e55101) + (locals.var_delta_ns * (locals.var_sp_s_y0_dn12 - locals.var_sp_s_xi0_dn12))))))), (((locals.var_sp_s_temp_dn13 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn13)) - ((locals.var_gf2_dn13 * assign42050_e55103) + (locals.var_gf2 * (((locals.var_sp_s_delta0_dn13 - locals.var_sp_s_y0_dn13) + locals.var_sp_s_temp1_dn13) + ((locals.var_delta_ns_dn13 * assign42050_e55101) + (locals.var_delta_ns * (locals.var_sp_s_y0_dn13 - locals.var_sp_s_xi0_dn13))))))), (((locals.var_sp_s_temp_dn14 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn14)) - ((locals.var_gf2_dn14 * assign42050_e55103) + (locals.var_gf2 * (((locals.var_sp_s_delta0_dn14 - locals.var_sp_s_y0_dn14) + locals.var_sp_s_temp1_dn14) + ((locals.var_delta_ns_dn14 * assign42050_e55101) + (locals.var_delta_ns * (locals.var_sp_s_y0_dn14 - locals.var_sp_s_xi0_dn14))))))), (((locals.var_sp_s_temp_dn15 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn15)) - ((locals.var_gf2_dn15 * assign42050_e55103) + (locals.var_gf2 * (((locals.var_sp_s_delta0_dn15 - locals.var_sp_s_y0_dn15) + locals.var_sp_s_temp1_dn15) + ((locals.var_delta_ns_dn15 * assign42050_e55101) + (locals.var_delta_ns * (locals.var_sp_s_y0_dn15 - locals.var_sp_s_xi0_dn15))))))), (((locals.var_sp_s_temp_dn16 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn16)) - ((locals.var_gf2_dn16 * assign42050_e55103) + (locals.var_gf2 * (((locals.var_sp_s_delta0_dn16 - locals.var_sp_s_y0_dn16) + locals.var_sp_s_temp1_dn16) + ((locals.var_delta_ns_dn16 * assign42050_e55101) + (locals.var_delta_ns * (locals.var_sp_s_y0_dn16 - locals.var_sp_s_xi0_dn16))))))), (((locals.var_sp_s_temp_dn17 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn17)) - ((locals.var_gf2_dn17 * assign42050_e55103) + (locals.var_gf2 * (((locals.var_sp_s_delta0_dn17 - locals.var_sp_s_y0_dn17) + locals.var_sp_s_temp1_dn17) + ((locals.var_delta_ns_dn17 * assign42050_e55101) + (locals.var_delta_ns * (locals.var_sp_s_y0_dn17 - locals.var_sp_s_xi0_dn17))))))), (((locals.var_sp_s_temp_dn18 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn18)) - ((locals.var_gf2_dn18 * assign42050_e55103) + (locals.var_gf2 * (((locals.var_sp_s_delta0_dn18 - locals.var_sp_s_y0_dn18) + locals.var_sp_s_temp1_dn18) + ((locals.var_delta_ns_dn18 * assign42050_e55101) + (locals.var_delta_ns * (locals.var_sp_s_y0_dn18 - locals.var_sp_s_xi0_dn18))))))), (((locals.var_sp_s_temp_dn19 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn19)) - ((locals.var_gf2_dn19 * assign42050_e55103) + (locals.var_gf2 * (((locals.var_sp_s_delta0_dn19 - locals.var_sp_s_y0_dn19) + locals.var_sp_s_temp1_dn19) + ((locals.var_delta_ns_dn19 * assign42050_e55101) + (locals.var_delta_ns * (locals.var_sp_s_y0_dn19 - locals.var_sp_s_xi0_dn19))))))), (((locals.var_sp_s_temp_dn20 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn20)) - ((locals.var_gf2_dn20 * assign42050_e55103) + (locals.var_gf2 * (((locals.var_sp_s_delta0_dn20 - locals.var_sp_s_y0_dn20) + locals.var_sp_s_temp1_dn20) + ((locals.var_delta_ns_dn20 * assign42050_e55101) + (locals.var_delta_ns * (locals.var_sp_s_y0_dn20 - locals.var_sp_s_xi0_dn20))))))),)
    } else {
        (locals.var_sp_s_qc, locals.var_sp_s_qc_dn5, locals.var_sp_s_qc_dn6, locals.var_sp_s_qc_dn7, locals.var_sp_s_qc_dn8, locals.var_sp_s_qc_dn12, locals.var_sp_s_qc_dn13, locals.var_sp_s_qc_dn14, locals.var_sp_s_qc_dn15, locals.var_sp_s_qc_dn16, locals.var_sp_s_qc_dn17, locals.var_sp_s_qc_dn18, locals.var_sp_s_qc_dn19, locals.var_sp_s_qc_dn20,)
    }
};
        locals.var_sp_s_qc = assign42050_e55107;
        locals.var_sp_s_qc_dn5 = assign42050_e55107_d_n5;
        locals.var_sp_s_qc_dn6 = assign42050_e55107_d_n6;
        locals.var_sp_s_qc_dn7 = assign42050_e55107_d_n7;
        locals.var_sp_s_qc_dn8 = assign42050_e55107_d_n8;
        locals.var_sp_s_qc_dn12 = assign42050_e55107_d_n12;
        locals.var_sp_s_qc_dn13 = assign42050_e55107_d_n13;
        locals.var_sp_s_qc_dn14 = assign42050_e55107_d_n14;
        locals.var_sp_s_qc_dn15 = assign42050_e55107_d_n15;
        locals.var_sp_s_qc_dn16 = assign42050_e55107_d_n16;
        locals.var_sp_s_qc_dn17 = assign42050_e55107_d_n17;
        locals.var_sp_s_qc_dn18 = assign42050_e55107_d_n18;
        locals.var_sp_s_qc_dn19 = assign42050_e55107_d_n19;
        locals.var_sp_s_qc_dn20 = assign42050_e55107_d_n20;
        locals.var_sp_s_qc_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_21(
        locals: &mut StampLocals,
    ) {
        let (assign42060_e55124, assign42060_e55124_d_n5, assign42060_e55124_d_n6, assign42060_e55124_d_n7, assign42060_e55124_d_n8, assign42060_e55124_d_n12, assign42060_e55124_d_n13, assign42060_e55124_d_n14, assign42060_e55124_d_n15, assign42060_e55124_d_n16, assign42060_e55124_d_n17, assign42060_e55124_d_n18, assign42060_e55124_d_n19, assign42060_e55124_d_n20,) = {
    if ((locals.var_guard1284 == 0.0) && (locals.var_guard1285 != 0.0)) {
        let assign42060_e55116: f64 = (locals.var_sp_s_delta0 + locals.var_sp_s_temp1);
        let assign42060_e55119: f64 = (locals.var_delta_ns * locals.var_sp_s_xi2);
        let assign42060_e55120: f64 = (assign42060_e55116 - assign42060_e55119);
        let assign42060_e55121: f64 = (locals.var_gf2 * assign42060_e55120);
        let assign42060_e55122: f64 = (2.0 - assign42060_e55121);
        (assign42060_e55122, (-((locals.var_gf2_dn5 * assign42060_e55120) + (locals.var_gf2 * ((locals.var_sp_s_delta0_dn5 + locals.var_sp_s_temp1_dn5) - ((locals.var_delta_ns_dn5 * locals.var_sp_s_xi2) + (locals.var_delta_ns * locals.var_sp_s_xi2_dn5)))))), (-((locals.var_gf2_dn6 * assign42060_e55120) + (locals.var_gf2 * ((locals.var_sp_s_delta0_dn6 + locals.var_sp_s_temp1_dn6) - ((locals.var_delta_ns_dn6 * locals.var_sp_s_xi2) + (locals.var_delta_ns * locals.var_sp_s_xi2_dn6)))))), (-((locals.var_gf2_dn7 * assign42060_e55120) + (locals.var_gf2 * ((locals.var_sp_s_delta0_dn7 + locals.var_sp_s_temp1_dn7) - ((locals.var_delta_ns_dn7 * locals.var_sp_s_xi2) + (locals.var_delta_ns * locals.var_sp_s_xi2_dn7)))))), (-((locals.var_gf2_dn8 * assign42060_e55120) + (locals.var_gf2 * ((locals.var_sp_s_delta0_dn8 + locals.var_sp_s_temp1_dn8) - ((locals.var_delta_ns_dn8 * locals.var_sp_s_xi2) + (locals.var_delta_ns * locals.var_sp_s_xi2_dn8)))))), (-((locals.var_gf2_dn12 * assign42060_e55120) + (locals.var_gf2 * ((locals.var_sp_s_delta0_dn12 + locals.var_sp_s_temp1_dn12) - ((locals.var_delta_ns_dn12 * locals.var_sp_s_xi2) + (locals.var_delta_ns * locals.var_sp_s_xi2_dn12)))))), (-((locals.var_gf2_dn13 * assign42060_e55120) + (locals.var_gf2 * ((locals.var_sp_s_delta0_dn13 + locals.var_sp_s_temp1_dn13) - ((locals.var_delta_ns_dn13 * locals.var_sp_s_xi2) + (locals.var_delta_ns * locals.var_sp_s_xi2_dn13)))))), (-((locals.var_gf2_dn14 * assign42060_e55120) + (locals.var_gf2 * ((locals.var_sp_s_delta0_dn14 + locals.var_sp_s_temp1_dn14) - ((locals.var_delta_ns_dn14 * locals.var_sp_s_xi2) + (locals.var_delta_ns * locals.var_sp_s_xi2_dn14)))))), (-((locals.var_gf2_dn15 * assign42060_e55120) + (locals.var_gf2 * ((locals.var_sp_s_delta0_dn15 + locals.var_sp_s_temp1_dn15) - ((locals.var_delta_ns_dn15 * locals.var_sp_s_xi2) + (locals.var_delta_ns * locals.var_sp_s_xi2_dn15)))))), (-((locals.var_gf2_dn16 * assign42060_e55120) + (locals.var_gf2 * ((locals.var_sp_s_delta0_dn16 + locals.var_sp_s_temp1_dn16) - ((locals.var_delta_ns_dn16 * locals.var_sp_s_xi2) + (locals.var_delta_ns * locals.var_sp_s_xi2_dn16)))))), (-((locals.var_gf2_dn17 * assign42060_e55120) + (locals.var_gf2 * ((locals.var_sp_s_delta0_dn17 + locals.var_sp_s_temp1_dn17) - ((locals.var_delta_ns_dn17 * locals.var_sp_s_xi2) + (locals.var_delta_ns * locals.var_sp_s_xi2_dn17)))))), (-((locals.var_gf2_dn18 * assign42060_e55120) + (locals.var_gf2 * ((locals.var_sp_s_delta0_dn18 + locals.var_sp_s_temp1_dn18) - ((locals.var_delta_ns_dn18 * locals.var_sp_s_xi2) + (locals.var_delta_ns * locals.var_sp_s_xi2_dn18)))))), (-((locals.var_gf2_dn19 * assign42060_e55120) + (locals.var_gf2 * ((locals.var_sp_s_delta0_dn19 + locals.var_sp_s_temp1_dn19) - ((locals.var_delta_ns_dn19 * locals.var_sp_s_xi2) + (locals.var_delta_ns * locals.var_sp_s_xi2_dn19)))))), (-((locals.var_gf2_dn20 * assign42060_e55120) + (locals.var_gf2 * ((locals.var_sp_s_delta0_dn20 + locals.var_sp_s_temp1_dn20) - ((locals.var_delta_ns_dn20 * locals.var_sp_s_xi2) + (locals.var_delta_ns * locals.var_sp_s_xi2_dn20)))))),)
    } else {
        (locals.var_sp_s_temp, locals.var_sp_s_temp_dn5, locals.var_sp_s_temp_dn6, locals.var_sp_s_temp_dn7, locals.var_sp_s_temp_dn8, locals.var_sp_s_temp_dn12, locals.var_sp_s_temp_dn13, locals.var_sp_s_temp_dn14, locals.var_sp_s_temp_dn15, locals.var_sp_s_temp_dn16, locals.var_sp_s_temp_dn17, locals.var_sp_s_temp_dn18, locals.var_sp_s_temp_dn19, locals.var_sp_s_temp_dn20,)
    }
};
        locals.var_sp_s_temp = assign42060_e55124;
        locals.var_sp_s_temp_dn5 = assign42060_e55124_d_n5;
        locals.var_sp_s_temp_dn6 = assign42060_e55124_d_n6;
        locals.var_sp_s_temp_dn7 = assign42060_e55124_d_n7;
        locals.var_sp_s_temp_dn8 = assign42060_e55124_d_n8;
        locals.var_sp_s_temp_dn12 = assign42060_e55124_d_n12;
        locals.var_sp_s_temp_dn13 = assign42060_e55124_d_n13;
        locals.var_sp_s_temp_dn14 = assign42060_e55124_d_n14;
        locals.var_sp_s_temp_dn15 = assign42060_e55124_d_n15;
        locals.var_sp_s_temp_dn16 = assign42060_e55124_d_n16;
        locals.var_sp_s_temp_dn17 = assign42060_e55124_d_n17;
        locals.var_sp_s_temp_dn18 = assign42060_e55124_d_n18;
        locals.var_sp_s_temp_dn19 = assign42060_e55124_d_n19;
        locals.var_sp_s_temp_dn20 = assign42060_e55124_d_n20;
        locals.var_sp_s_temp_rv = 0.0;

        let (assign42070_e55139, assign42070_e55139_d_n5, assign42070_e55139_d_n6, assign42070_e55139_d_n7, assign42070_e55139_d_n8, assign42070_e55139_d_n12, assign42070_e55139_d_n13, assign42070_e55139_d_n14, assign42070_e55139_d_n15, assign42070_e55139_d_n16, assign42070_e55139_d_n17, assign42070_e55139_d_n18, assign42070_e55139_d_n19, assign42070_e55139_d_n20,) = {
    if ((locals.var_guard1284 == 0.0) && (locals.var_guard1285 != 0.0)) {
        let assign42070_e55131: f64 = (locals.var_sp_s_pc * locals.var_sp_s_pc);
        let assign42070_e55135: f64 = (locals.var_sp_s_qc * locals.var_sp_s_temp);
        let assign42070_e55136: f64 = (2.0 * assign42070_e55135);
        let assign42070_e55137: f64 = (assign42070_e55131 - assign42070_e55136);
        (assign42070_e55137, (((locals.var_sp_s_pc_dn5 * locals.var_sp_s_pc) + (locals.var_sp_s_pc * locals.var_sp_s_pc_dn5)) - (2.0 * ((locals.var_sp_s_qc_dn5 * locals.var_sp_s_temp) + (locals.var_sp_s_qc * locals.var_sp_s_temp_dn5)))), (((locals.var_sp_s_pc_dn6 * locals.var_sp_s_pc) + (locals.var_sp_s_pc * locals.var_sp_s_pc_dn6)) - (2.0 * ((locals.var_sp_s_qc_dn6 * locals.var_sp_s_temp) + (locals.var_sp_s_qc * locals.var_sp_s_temp_dn6)))), (((locals.var_sp_s_pc_dn7 * locals.var_sp_s_pc) + (locals.var_sp_s_pc * locals.var_sp_s_pc_dn7)) - (2.0 * ((locals.var_sp_s_qc_dn7 * locals.var_sp_s_temp) + (locals.var_sp_s_qc * locals.var_sp_s_temp_dn7)))), (((locals.var_sp_s_pc_dn8 * locals.var_sp_s_pc) + (locals.var_sp_s_pc * locals.var_sp_s_pc_dn8)) - (2.0 * ((locals.var_sp_s_qc_dn8 * locals.var_sp_s_temp) + (locals.var_sp_s_qc * locals.var_sp_s_temp_dn8)))), (((locals.var_sp_s_pc_dn12 * locals.var_sp_s_pc) + (locals.var_sp_s_pc * locals.var_sp_s_pc_dn12)) - (2.0 * ((locals.var_sp_s_qc_dn12 * locals.var_sp_s_temp) + (locals.var_sp_s_qc * locals.var_sp_s_temp_dn12)))), (((locals.var_sp_s_pc_dn13 * locals.var_sp_s_pc) + (locals.var_sp_s_pc * locals.var_sp_s_pc_dn13)) - (2.0 * ((locals.var_sp_s_qc_dn13 * locals.var_sp_s_temp) + (locals.var_sp_s_qc * locals.var_sp_s_temp_dn13)))), (((locals.var_sp_s_pc_dn14 * locals.var_sp_s_pc) + (locals.var_sp_s_pc * locals.var_sp_s_pc_dn14)) - (2.0 * ((locals.var_sp_s_qc_dn14 * locals.var_sp_s_temp) + (locals.var_sp_s_qc * locals.var_sp_s_temp_dn14)))), (((locals.var_sp_s_pc_dn15 * locals.var_sp_s_pc) + (locals.var_sp_s_pc * locals.var_sp_s_pc_dn15)) - (2.0 * ((locals.var_sp_s_qc_dn15 * locals.var_sp_s_temp) + (locals.var_sp_s_qc * locals.var_sp_s_temp_dn15)))), (((locals.var_sp_s_pc_dn16 * locals.var_sp_s_pc) + (locals.var_sp_s_pc * locals.var_sp_s_pc_dn16)) - (2.0 * ((locals.var_sp_s_qc_dn16 * locals.var_sp_s_temp) + (locals.var_sp_s_qc * locals.var_sp_s_temp_dn16)))), (((locals.var_sp_s_pc_dn17 * locals.var_sp_s_pc) + (locals.var_sp_s_pc * locals.var_sp_s_pc_dn17)) - (2.0 * ((locals.var_sp_s_qc_dn17 * locals.var_sp_s_temp) + (locals.var_sp_s_qc * locals.var_sp_s_temp_dn17)))), (((locals.var_sp_s_pc_dn18 * locals.var_sp_s_pc) + (locals.var_sp_s_pc * locals.var_sp_s_pc_dn18)) - (2.0 * ((locals.var_sp_s_qc_dn18 * locals.var_sp_s_temp) + (locals.var_sp_s_qc * locals.var_sp_s_temp_dn18)))), (((locals.var_sp_s_pc_dn19 * locals.var_sp_s_pc) + (locals.var_sp_s_pc * locals.var_sp_s_pc_dn19)) - (2.0 * ((locals.var_sp_s_qc_dn19 * locals.var_sp_s_temp) + (locals.var_sp_s_qc * locals.var_sp_s_temp_dn19)))), (((locals.var_sp_s_pc_dn20 * locals.var_sp_s_pc) + (locals.var_sp_s_pc * locals.var_sp_s_pc_dn20)) - (2.0 * ((locals.var_sp_s_qc_dn20 * locals.var_sp_s_temp) + (locals.var_sp_s_qc * locals.var_sp_s_temp_dn20)))),)
    } else {
        (locals.var_sp_s_temp, locals.var_sp_s_temp_dn5, locals.var_sp_s_temp_dn6, locals.var_sp_s_temp_dn7, locals.var_sp_s_temp_dn8, locals.var_sp_s_temp_dn12, locals.var_sp_s_temp_dn13, locals.var_sp_s_temp_dn14, locals.var_sp_s_temp_dn15, locals.var_sp_s_temp_dn16, locals.var_sp_s_temp_dn17, locals.var_sp_s_temp_dn18, locals.var_sp_s_temp_dn19, locals.var_sp_s_temp_dn20,)
    }
};
        locals.var_sp_s_temp = assign42070_e55139;
        locals.var_sp_s_temp_dn5 = assign42070_e55139_d_n5;
        locals.var_sp_s_temp_dn6 = assign42070_e55139_d_n6;
        locals.var_sp_s_temp_dn7 = assign42070_e55139_d_n7;
        locals.var_sp_s_temp_dn8 = assign42070_e55139_d_n8;
        locals.var_sp_s_temp_dn12 = assign42070_e55139_d_n12;
        locals.var_sp_s_temp_dn13 = assign42070_e55139_d_n13;
        locals.var_sp_s_temp_dn14 = assign42070_e55139_d_n14;
        locals.var_sp_s_temp_dn15 = assign42070_e55139_d_n15;
        locals.var_sp_s_temp_dn16 = assign42070_e55139_d_n16;
        locals.var_sp_s_temp_dn17 = assign42070_e55139_d_n17;
        locals.var_sp_s_temp_dn18 = assign42070_e55139_d_n18;
        locals.var_sp_s_temp_dn19 = assign42070_e55139_d_n19;
        locals.var_sp_s_temp_dn20 = assign42070_e55139_d_n20;
        locals.var_sp_s_temp_rv = 0.0;

        let (assign42080_e55156, assign42080_e55156_d_n5, assign42080_e55156_d_n6, assign42080_e55156_d_n7, assign42080_e55156_d_n8, assign42080_e55156_d_n12, assign42080_e55156_d_n13, assign42080_e55156_d_n14, assign42080_e55156_d_n15, assign42080_e55156_d_n16, assign42080_e55156_d_n17, assign42080_e55156_d_n18, assign42080_e55156_d_n19, assign42080_e55156_d_n20,) = {
    if ((locals.var_guard1284 == 0.0) && (locals.var_guard1285 != 0.0)) {
        let assign42080_e55145: f64 = (-locals.var_sp_s_y0);
        let assign42080_e55150: f64 = (locals.var_sp_s_temp).sqrt();
        let assign42080_e55151: f64 = (locals.var_sp_s_pc + assign42080_e55150);
        let assign42080_e55152: f64 = (locals.var_sp_s_qc / assign42080_e55151);
        let assign42080_e55153: f64 = (2.0 * assign42080_e55152);
        let assign42080_e55154: f64 = (assign42080_e55145 - assign42080_e55153);
        (assign42080_e55154, ((-locals.var_sp_s_y0_dn5) - (2.0 * (((locals.var_sp_s_qc_dn5 * assign42080_e55151) - (locals.var_sp_s_qc * (locals.var_sp_s_pc_dn5 + (locals.var_sp_s_temp_dn5 / (2.0 * assign42080_e55150))))) / (assign42080_e55151 * assign42080_e55151)))), ((-locals.var_sp_s_y0_dn6) - (2.0 * (((locals.var_sp_s_qc_dn6 * assign42080_e55151) - (locals.var_sp_s_qc * (locals.var_sp_s_pc_dn6 + (locals.var_sp_s_temp_dn6 / (2.0 * assign42080_e55150))))) / (assign42080_e55151 * assign42080_e55151)))), ((-locals.var_sp_s_y0_dn7) - (2.0 * (((locals.var_sp_s_qc_dn7 * assign42080_e55151) - (locals.var_sp_s_qc * (locals.var_sp_s_pc_dn7 + (locals.var_sp_s_temp_dn7 / (2.0 * assign42080_e55150))))) / (assign42080_e55151 * assign42080_e55151)))), ((-locals.var_sp_s_y0_dn8) - (2.0 * (((locals.var_sp_s_qc_dn8 * assign42080_e55151) - (locals.var_sp_s_qc * (locals.var_sp_s_pc_dn8 + (locals.var_sp_s_temp_dn8 / (2.0 * assign42080_e55150))))) / (assign42080_e55151 * assign42080_e55151)))), ((-locals.var_sp_s_y0_dn12) - (2.0 * (((locals.var_sp_s_qc_dn12 * assign42080_e55151) - (locals.var_sp_s_qc * (locals.var_sp_s_pc_dn12 + (locals.var_sp_s_temp_dn12 / (2.0 * assign42080_e55150))))) / (assign42080_e55151 * assign42080_e55151)))), ((-locals.var_sp_s_y0_dn13) - (2.0 * (((locals.var_sp_s_qc_dn13 * assign42080_e55151) - (locals.var_sp_s_qc * (locals.var_sp_s_pc_dn13 + (locals.var_sp_s_temp_dn13 / (2.0 * assign42080_e55150))))) / (assign42080_e55151 * assign42080_e55151)))), ((-locals.var_sp_s_y0_dn14) - (2.0 * (((locals.var_sp_s_qc_dn14 * assign42080_e55151) - (locals.var_sp_s_qc * (locals.var_sp_s_pc_dn14 + (locals.var_sp_s_temp_dn14 / (2.0 * assign42080_e55150))))) / (assign42080_e55151 * assign42080_e55151)))), ((-locals.var_sp_s_y0_dn15) - (2.0 * (((locals.var_sp_s_qc_dn15 * assign42080_e55151) - (locals.var_sp_s_qc * (locals.var_sp_s_pc_dn15 + (locals.var_sp_s_temp_dn15 / (2.0 * assign42080_e55150))))) / (assign42080_e55151 * assign42080_e55151)))), ((-locals.var_sp_s_y0_dn16) - (2.0 * (((locals.var_sp_s_qc_dn16 * assign42080_e55151) - (locals.var_sp_s_qc * (locals.var_sp_s_pc_dn16 + (locals.var_sp_s_temp_dn16 / (2.0 * assign42080_e55150))))) / (assign42080_e55151 * assign42080_e55151)))), ((-locals.var_sp_s_y0_dn17) - (2.0 * (((locals.var_sp_s_qc_dn17 * assign42080_e55151) - (locals.var_sp_s_qc * (locals.var_sp_s_pc_dn17 + (locals.var_sp_s_temp_dn17 / (2.0 * assign42080_e55150))))) / (assign42080_e55151 * assign42080_e55151)))), ((-locals.var_sp_s_y0_dn18) - (2.0 * (((locals.var_sp_s_qc_dn18 * assign42080_e55151) - (locals.var_sp_s_qc * (locals.var_sp_s_pc_dn18 + (locals.var_sp_s_temp_dn18 / (2.0 * assign42080_e55150))))) / (assign42080_e55151 * assign42080_e55151)))), ((-locals.var_sp_s_y0_dn19) - (2.0 * (((locals.var_sp_s_qc_dn19 * assign42080_e55151) - (locals.var_sp_s_qc * (locals.var_sp_s_pc_dn19 + (locals.var_sp_s_temp_dn19 / (2.0 * assign42080_e55150))))) / (assign42080_e55151 * assign42080_e55151)))), ((-locals.var_sp_s_y0_dn20) - (2.0 * (((locals.var_sp_s_qc_dn20 * assign42080_e55151) - (locals.var_sp_s_qc * (locals.var_sp_s_pc_dn20 + (locals.var_sp_s_temp_dn20 / (2.0 * assign42080_e55150))))) / (assign42080_e55151 * assign42080_e55151)))),)
    } else {
        (locals.var_x_s, locals.var_x_s_dn5, locals.var_x_s_dn6, locals.var_x_s_dn7, locals.var_x_s_dn8, locals.var_x_s_dn12, locals.var_x_s_dn13, locals.var_x_s_dn14, locals.var_x_s_dn15, locals.var_x_s_dn16, locals.var_x_s_dn17, locals.var_x_s_dn18, locals.var_x_s_dn19, locals.var_x_s_dn20,)
    }
};
        locals.var_x_s = assign42080_e55156;
        locals.var_x_s_dn5 = assign42080_e55156_d_n5;
        locals.var_x_s_dn6 = assign42080_e55156_d_n6;
        locals.var_x_s_dn7 = assign42080_e55156_d_n7;
        locals.var_x_s_dn8 = assign42080_e55156_d_n8;
        locals.var_x_s_dn12 = assign42080_e55156_d_n12;
        locals.var_x_s_dn13 = assign42080_e55156_d_n13;
        locals.var_x_s_dn14 = assign42080_e55156_d_n14;
        locals.var_x_s_dn15 = assign42080_e55156_d_n15;
        locals.var_x_s_dn16 = assign42080_e55156_d_n16;
        locals.var_x_s_dn17 = assign42080_e55156_d_n17;
        locals.var_x_s_dn18 = assign42080_e55156_d_n18;
        locals.var_x_s_dn19 = assign42080_e55156_d_n19;
        locals.var_x_s_dn20 = assign42080_e55156_d_n20;
        locals.var_x_s_rv = 0.0;

        let (assign42090_e55170, assign42090_e55170_d_n5, assign42090_e55170_d_n6, assign42090_e55170_d_n7, assign42090_e55170_d_n8, assign42090_e55170_d_n12, assign42090_e55170_d_n13, assign42090_e55170_d_n14, assign42090_e55170_d_n15, assign42090_e55170_d_n16, assign42090_e55170_d_n17, assign42090_e55170_d_n18, assign42090_e55170_d_n19, assign42090_e55170_d_n20,) = {
    if ((locals.var_guard1284 == 0.0) && (locals.var_guard1285 == 0.0)) {
        let assign42090_e55166: f64 = (locals.var_gf * 0.7324648775608221);
        let assign42090_e55167: f64 = (1.25 + assign42090_e55166);
        let assign42090_e55168: f64 = (1.0 / assign42090_e55167);
        (assign42090_e55168, (-((locals.var_gf_dn5 * 0.7324648775608221) / (assign42090_e55167 * assign42090_e55167))), (-((locals.var_gf_dn6 * 0.7324648775608221) / (assign42090_e55167 * assign42090_e55167))), (-((locals.var_gf_dn7 * 0.7324648775608221) / (assign42090_e55167 * assign42090_e55167))), (-((locals.var_gf_dn8 * 0.7324648775608221) / (assign42090_e55167 * assign42090_e55167))), (-((locals.var_gf_dn12 * 0.7324648775608221) / (assign42090_e55167 * assign42090_e55167))), (-((locals.var_gf_dn13 * 0.7324648775608221) / (assign42090_e55167 * assign42090_e55167))), (-((locals.var_gf_dn14 * 0.7324648775608221) / (assign42090_e55167 * assign42090_e55167))), (-((locals.var_gf_dn15 * 0.7324648775608221) / (assign42090_e55167 * assign42090_e55167))), (-((locals.var_gf_dn16 * 0.7324648775608221) / (assign42090_e55167 * assign42090_e55167))), (-((locals.var_gf_dn17 * 0.7324648775608221) / (assign42090_e55167 * assign42090_e55167))), (-((locals.var_gf_dn18 * 0.7324648775608221) / (assign42090_e55167 * assign42090_e55167))), (-((locals.var_gf_dn19 * 0.7324648775608221) / (assign42090_e55167 * assign42090_e55167))), (-((locals.var_gf_dn20 * 0.7324648775608221) / (assign42090_e55167 * assign42090_e55167))),)
    } else {
        (locals.var_sp_xg1, locals.var_sp_xg1_dn5, locals.var_sp_xg1_dn6, locals.var_sp_xg1_dn7, locals.var_sp_xg1_dn8, locals.var_sp_xg1_dn12, locals.var_sp_xg1_dn13, locals.var_sp_xg1_dn14, locals.var_sp_xg1_dn15, locals.var_sp_xg1_dn16, locals.var_sp_xg1_dn17, locals.var_sp_xg1_dn18, locals.var_sp_xg1_dn19, locals.var_sp_xg1_dn20,)
    }
};
        locals.var_sp_xg1 = assign42090_e55170;
        locals.var_sp_xg1_dn5 = assign42090_e55170_d_n5;
        locals.var_sp_xg1_dn6 = assign42090_e55170_d_n6;
        locals.var_sp_xg1_dn7 = assign42090_e55170_d_n7;
        locals.var_sp_xg1_dn8 = assign42090_e55170_d_n8;
        locals.var_sp_xg1_dn12 = assign42090_e55170_d_n12;
        locals.var_sp_xg1_dn13 = assign42090_e55170_d_n13;
        locals.var_sp_xg1_dn14 = assign42090_e55170_d_n14;
        locals.var_sp_xg1_dn15 = assign42090_e55170_d_n15;
        locals.var_sp_xg1_dn16 = assign42090_e55170_d_n16;
        locals.var_sp_xg1_dn17 = assign42090_e55170_d_n17;
        locals.var_sp_xg1_dn18 = assign42090_e55170_d_n18;
        locals.var_sp_xg1_dn19 = assign42090_e55170_d_n19;
        locals.var_sp_xg1_dn20 = assign42090_e55170_d_n20;
        locals.var_sp_xg1_rv = 0.0;

        let (assign42100_e55186, assign42100_e55186_d_n5, assign42100_e55186_d_n6, assign42100_e55186_d_n7, assign42100_e55186_d_n8, assign42100_e55186_d_n12, assign42100_e55186_d_n13, assign42100_e55186_d_n14, assign42100_e55186_d_n15, assign42100_e55186_d_n16, assign42100_e55186_d_n17, assign42100_e55186_d_n18, assign42100_e55186_d_n19, assign42100_e55186_d_n20,) = {
    if ((locals.var_guard1284 == 0.0) && (locals.var_guard1285 == 0.0)) {
        let assign42100_e55178: f64 = (locals.var_xi * 1.25);
        let assign42100_e55180: f64 = (assign42100_e55178 * locals.var_sp_xg1);
        let assign42100_e55182: f64 = (assign42100_e55180 - 1.0);
        let assign42100_e55184: f64 = (assign42100_e55182 * locals.var_sp_xg1);
        (assign42100_e55184, (((((locals.var_xi_dn5 * 1.25) * locals.var_sp_xg1) + (assign42100_e55178 * locals.var_sp_xg1_dn5)) * locals.var_sp_xg1) + (assign42100_e55182 * locals.var_sp_xg1_dn5)), (((((locals.var_xi_dn6 * 1.25) * locals.var_sp_xg1) + (assign42100_e55178 * locals.var_sp_xg1_dn6)) * locals.var_sp_xg1) + (assign42100_e55182 * locals.var_sp_xg1_dn6)), (((((locals.var_xi_dn7 * 1.25) * locals.var_sp_xg1) + (assign42100_e55178 * locals.var_sp_xg1_dn7)) * locals.var_sp_xg1) + (assign42100_e55182 * locals.var_sp_xg1_dn7)), (((((locals.var_xi_dn8 * 1.25) * locals.var_sp_xg1) + (assign42100_e55178 * locals.var_sp_xg1_dn8)) * locals.var_sp_xg1) + (assign42100_e55182 * locals.var_sp_xg1_dn8)), (((((locals.var_xi_dn12 * 1.25) * locals.var_sp_xg1) + (assign42100_e55178 * locals.var_sp_xg1_dn12)) * locals.var_sp_xg1) + (assign42100_e55182 * locals.var_sp_xg1_dn12)), (((((locals.var_xi_dn13 * 1.25) * locals.var_sp_xg1) + (assign42100_e55178 * locals.var_sp_xg1_dn13)) * locals.var_sp_xg1) + (assign42100_e55182 * locals.var_sp_xg1_dn13)), (((((locals.var_xi_dn14 * 1.25) * locals.var_sp_xg1) + (assign42100_e55178 * locals.var_sp_xg1_dn14)) * locals.var_sp_xg1) + (assign42100_e55182 * locals.var_sp_xg1_dn14)), (((((locals.var_xi_dn15 * 1.25) * locals.var_sp_xg1) + (assign42100_e55178 * locals.var_sp_xg1_dn15)) * locals.var_sp_xg1) + (assign42100_e55182 * locals.var_sp_xg1_dn15)), (((((locals.var_xi_dn16 * 1.25) * locals.var_sp_xg1) + (assign42100_e55178 * locals.var_sp_xg1_dn16)) * locals.var_sp_xg1) + (assign42100_e55182 * locals.var_sp_xg1_dn16)), (((((locals.var_xi_dn17 * 1.25) * locals.var_sp_xg1) + (assign42100_e55178 * locals.var_sp_xg1_dn17)) * locals.var_sp_xg1) + (assign42100_e55182 * locals.var_sp_xg1_dn17)), (((((locals.var_xi_dn18 * 1.25) * locals.var_sp_xg1) + (assign42100_e55178 * locals.var_sp_xg1_dn18)) * locals.var_sp_xg1) + (assign42100_e55182 * locals.var_sp_xg1_dn18)), (((((locals.var_xi_dn19 * 1.25) * locals.var_sp_xg1) + (assign42100_e55178 * locals.var_sp_xg1_dn19)) * locals.var_sp_xg1) + (assign42100_e55182 * locals.var_sp_xg1_dn19)), (((((locals.var_xi_dn20 * 1.25) * locals.var_sp_xg1) + (assign42100_e55178 * locals.var_sp_xg1_dn20)) * locals.var_sp_xg1) + (assign42100_e55182 * locals.var_sp_xg1_dn20)),)
    } else {
        (locals.var_sp_s_a_fac, locals.var_sp_s_a_fac_dn5, locals.var_sp_s_a_fac_dn6, locals.var_sp_s_a_fac_dn7, locals.var_sp_s_a_fac_dn8, locals.var_sp_s_a_fac_dn12, locals.var_sp_s_a_fac_dn13, locals.var_sp_s_a_fac_dn14, locals.var_sp_s_a_fac_dn15, locals.var_sp_s_a_fac_dn16, locals.var_sp_s_a_fac_dn17, locals.var_sp_s_a_fac_dn18, locals.var_sp_s_a_fac_dn19, locals.var_sp_s_a_fac_dn20,)
    }
};
        locals.var_sp_s_a_fac = assign42100_e55186;
        locals.var_sp_s_a_fac_dn5 = assign42100_e55186_d_n5;
        locals.var_sp_s_a_fac_dn6 = assign42100_e55186_d_n6;
        locals.var_sp_s_a_fac_dn7 = assign42100_e55186_d_n7;
        locals.var_sp_s_a_fac_dn8 = assign42100_e55186_d_n8;
        locals.var_sp_s_a_fac_dn12 = assign42100_e55186_d_n12;
        locals.var_sp_s_a_fac_dn13 = assign42100_e55186_d_n13;
        locals.var_sp_s_a_fac_dn14 = assign42100_e55186_d_n14;
        locals.var_sp_s_a_fac_dn15 = assign42100_e55186_d_n15;
        locals.var_sp_s_a_fac_dn16 = assign42100_e55186_d_n16;
        locals.var_sp_s_a_fac_dn17 = assign42100_e55186_d_n17;
        locals.var_sp_s_a_fac_dn18 = assign42100_e55186_d_n18;
        locals.var_sp_s_a_fac_dn19 = assign42100_e55186_d_n19;
        locals.var_sp_s_a_fac_dn20 = assign42100_e55186_d_n20;
        locals.var_sp_s_a_fac_rv = 0.0;

        let (assign42110_e55202, assign42110_e55202_d_n5, assign42110_e55202_d_n6, assign42110_e55202_d_n7, assign42110_e55202_d_n8, assign42110_e55202_d_n12, assign42110_e55202_d_n13, assign42110_e55202_d_n14, assign42110_e55202_d_n15, assign42110_e55202_d_n16, assign42110_e55202_d_n17, assign42110_e55202_d_n18, assign42110_e55202_d_n19, assign42110_e55202_d_n20,) = {
    if ((locals.var_guard1284 == 0.0) && (locals.var_guard1285 == 0.0)) {
        let assign42110_e55194: f64 = (locals.var_xg * locals.var_inv_xi);
        let assign42110_e55198: f64 = (locals.var_sp_s_a_fac * locals.var_xg);
        let assign42110_e55199: f64 = (1.0 + assign42110_e55198);
        let assign42110_e55200: f64 = (assign42110_e55194 * assign42110_e55199);
        (assign42110_e55200, ((((locals.var_xg_dn5 * locals.var_inv_xi) + (locals.var_xg * locals.var_inv_xi_dn5)) * assign42110_e55199) + (assign42110_e55194 * ((locals.var_sp_s_a_fac_dn5 * locals.var_xg) + (locals.var_sp_s_a_fac * locals.var_xg_dn5)))), ((((locals.var_xg_dn6 * locals.var_inv_xi) + (locals.var_xg * locals.var_inv_xi_dn6)) * assign42110_e55199) + (assign42110_e55194 * ((locals.var_sp_s_a_fac_dn6 * locals.var_xg) + (locals.var_sp_s_a_fac * locals.var_xg_dn6)))), ((((locals.var_xg_dn7 * locals.var_inv_xi) + (locals.var_xg * locals.var_inv_xi_dn7)) * assign42110_e55199) + (assign42110_e55194 * ((locals.var_sp_s_a_fac_dn7 * locals.var_xg) + (locals.var_sp_s_a_fac * locals.var_xg_dn7)))), ((((locals.var_xg_dn8 * locals.var_inv_xi) + (locals.var_xg * locals.var_inv_xi_dn8)) * assign42110_e55199) + (assign42110_e55194 * ((locals.var_sp_s_a_fac_dn8 * locals.var_xg) + (locals.var_sp_s_a_fac * locals.var_xg_dn8)))), ((((locals.var_xg_dn12 * locals.var_inv_xi) + (locals.var_xg * locals.var_inv_xi_dn12)) * assign42110_e55199) + (assign42110_e55194 * ((locals.var_sp_s_a_fac_dn12 * locals.var_xg) + (locals.var_sp_s_a_fac * locals.var_xg_dn12)))), ((((locals.var_xg_dn13 * locals.var_inv_xi) + (locals.var_xg * locals.var_inv_xi_dn13)) * assign42110_e55199) + (assign42110_e55194 * ((locals.var_sp_s_a_fac_dn13 * locals.var_xg) + (locals.var_sp_s_a_fac * locals.var_xg_dn13)))), ((((locals.var_xg_dn14 * locals.var_inv_xi) + (locals.var_xg * locals.var_inv_xi_dn14)) * assign42110_e55199) + (assign42110_e55194 * ((locals.var_sp_s_a_fac_dn14 * locals.var_xg) + (locals.var_sp_s_a_fac * locals.var_xg_dn14)))), ((((locals.var_xg_dn15 * locals.var_inv_xi) + (locals.var_xg * locals.var_inv_xi_dn15)) * assign42110_e55199) + (assign42110_e55194 * ((locals.var_sp_s_a_fac_dn15 * locals.var_xg) + (locals.var_sp_s_a_fac * locals.var_xg_dn15)))), ((((locals.var_xg_dn16 * locals.var_inv_xi) + (locals.var_xg * locals.var_inv_xi_dn16)) * assign42110_e55199) + (assign42110_e55194 * ((locals.var_sp_s_a_fac_dn16 * locals.var_xg) + (locals.var_sp_s_a_fac * locals.var_xg_dn16)))), ((((locals.var_xg_dn17 * locals.var_inv_xi) + (locals.var_xg * locals.var_inv_xi_dn17)) * assign42110_e55199) + (assign42110_e55194 * ((locals.var_sp_s_a_fac_dn17 * locals.var_xg) + (locals.var_sp_s_a_fac * locals.var_xg_dn17)))), ((((locals.var_xg_dn18 * locals.var_inv_xi) + (locals.var_xg * locals.var_inv_xi_dn18)) * assign42110_e55199) + (assign42110_e55194 * ((locals.var_sp_s_a_fac_dn18 * locals.var_xg) + (locals.var_sp_s_a_fac * locals.var_xg_dn18)))), ((((locals.var_xg_dn19 * locals.var_inv_xi) + (locals.var_xg * locals.var_inv_xi_dn19)) * assign42110_e55199) + (assign42110_e55194 * ((locals.var_sp_s_a_fac_dn19 * locals.var_xg) + (locals.var_sp_s_a_fac * locals.var_xg_dn19)))), ((((locals.var_xg_dn20 * locals.var_inv_xi) + (locals.var_xg * locals.var_inv_xi_dn20)) * assign42110_e55199) + (assign42110_e55194 * ((locals.var_sp_s_a_fac_dn20 * locals.var_xg) + (locals.var_sp_s_a_fac * locals.var_xg_dn20)))),)
    } else {
        (locals.var_sp_s_xbar, locals.var_sp_s_xbar_dn5, locals.var_sp_s_xbar_dn6, locals.var_sp_s_xbar_dn7, locals.var_sp_s_xbar_dn8, locals.var_sp_s_xbar_dn12, locals.var_sp_s_xbar_dn13, locals.var_sp_s_xbar_dn14, locals.var_sp_s_xbar_dn15, locals.var_sp_s_xbar_dn16, locals.var_sp_s_xbar_dn17, locals.var_sp_s_xbar_dn18, locals.var_sp_s_xbar_dn19, locals.var_sp_s_xbar_dn20,)
    }
};
        locals.var_sp_s_xbar = assign42110_e55202;
        locals.var_sp_s_xbar_dn5 = assign42110_e55202_d_n5;
        locals.var_sp_s_xbar_dn6 = assign42110_e55202_d_n6;
        locals.var_sp_s_xbar_dn7 = assign42110_e55202_d_n7;
        locals.var_sp_s_xbar_dn8 = assign42110_e55202_d_n8;
        locals.var_sp_s_xbar_dn12 = assign42110_e55202_d_n12;
        locals.var_sp_s_xbar_dn13 = assign42110_e55202_d_n13;
        locals.var_sp_s_xbar_dn14 = assign42110_e55202_d_n14;
        locals.var_sp_s_xbar_dn15 = assign42110_e55202_d_n15;
        locals.var_sp_s_xbar_dn16 = assign42110_e55202_d_n16;
        locals.var_sp_s_xbar_dn17 = assign42110_e55202_d_n17;
        locals.var_sp_s_xbar_dn18 = assign42110_e55202_d_n18;
        locals.var_sp_s_xbar_dn19 = assign42110_e55202_d_n19;
        locals.var_sp_s_xbar_dn20 = assign42110_e55202_d_n20;
        locals.var_sp_s_xbar_rv = 0.0;

        let assign42120_e55204: f64 = (-locals.var_sp_s_xbar);
        let assign42120_e55206: f64 = (-230.25850929940458);
        let assign42120_e55207: f64 = if assign42120_e55204 > assign42120_e55206 { 1.0 } else { 0.0 };
        locals.var_guard1287 = assign42120_e55207;
        locals.var_guard1287_rv = 0.0;

        let (assign42130_e55219, assign42130_e55219_d_n5, assign42130_e55219_d_n6, assign42130_e55219_d_n7, assign42130_e55219_d_n8, assign42130_e55219_d_n12, assign42130_e55219_d_n13, assign42130_e55219_d_n14, assign42130_e55219_d_n15, assign42130_e55219_d_n16, assign42130_e55219_d_n17, assign42130_e55219_d_n18, assign42130_e55219_d_n19, assign42130_e55219_d_n20,) = {
    if (((locals.var_guard1284 == 0.0) && (locals.var_guard1285 == 0.0)) && (locals.var_guard1287 != 0.0)) {
        let assign42130_e55216: f64 = (-locals.var_sp_s_xbar);
        let assign42130_e55217: f64 = (assign42130_e55216).exp();
        (assign42130_e55217, (assign42130_e55217 * (-locals.var_sp_s_xbar_dn5)), (assign42130_e55217 * (-locals.var_sp_s_xbar_dn6)), (assign42130_e55217 * (-locals.var_sp_s_xbar_dn7)), (assign42130_e55217 * (-locals.var_sp_s_xbar_dn8)), (assign42130_e55217 * (-locals.var_sp_s_xbar_dn12)), (assign42130_e55217 * (-locals.var_sp_s_xbar_dn13)), (assign42130_e55217 * (-locals.var_sp_s_xbar_dn14)), (assign42130_e55217 * (-locals.var_sp_s_xbar_dn15)), (assign42130_e55217 * (-locals.var_sp_s_xbar_dn16)), (assign42130_e55217 * (-locals.var_sp_s_xbar_dn17)), (assign42130_e55217 * (-locals.var_sp_s_xbar_dn18)), (assign42130_e55217 * (-locals.var_sp_s_xbar_dn19)), (assign42130_e55217 * (-locals.var_sp_s_xbar_dn20)),)
    } else {
        (locals.var_sp_s_temp, locals.var_sp_s_temp_dn5, locals.var_sp_s_temp_dn6, locals.var_sp_s_temp_dn7, locals.var_sp_s_temp_dn8, locals.var_sp_s_temp_dn12, locals.var_sp_s_temp_dn13, locals.var_sp_s_temp_dn14, locals.var_sp_s_temp_dn15, locals.var_sp_s_temp_dn16, locals.var_sp_s_temp_dn17, locals.var_sp_s_temp_dn18, locals.var_sp_s_temp_dn19, locals.var_sp_s_temp_dn20,)
    }
};
        locals.var_sp_s_temp = assign42130_e55219;
        locals.var_sp_s_temp_dn5 = assign42130_e55219_d_n5;
        locals.var_sp_s_temp_dn6 = assign42130_e55219_d_n6;
        locals.var_sp_s_temp_dn7 = assign42130_e55219_d_n7;
        locals.var_sp_s_temp_dn8 = assign42130_e55219_d_n8;
        locals.var_sp_s_temp_dn12 = assign42130_e55219_d_n12;
        locals.var_sp_s_temp_dn13 = assign42130_e55219_d_n13;
        locals.var_sp_s_temp_dn14 = assign42130_e55219_d_n14;
        locals.var_sp_s_temp_dn15 = assign42130_e55219_d_n15;
        locals.var_sp_s_temp_dn16 = assign42130_e55219_d_n16;
        locals.var_sp_s_temp_dn17 = assign42130_e55219_d_n17;
        locals.var_sp_s_temp_dn18 = assign42130_e55219_d_n18;
        locals.var_sp_s_temp_dn19 = assign42130_e55219_d_n19;
        locals.var_sp_s_temp_dn20 = assign42130_e55219_d_n20;
        locals.var_sp_s_temp_rv = 0.0;

        let (assign42140_e55258, assign42140_e55258_d_n5, assign42140_e55258_d_n6, assign42140_e55258_d_n7, assign42140_e55258_d_n8, assign42140_e55258_d_n12, assign42140_e55258_d_n13, assign42140_e55258_d_n14, assign42140_e55258_d_n15, assign42140_e55258_d_n16, assign42140_e55258_d_n17, assign42140_e55258_d_n18, assign42140_e55258_d_n19, assign42140_e55258_d_n20,) = {
    if (((locals.var_guard1284 == 0.0) && (locals.var_guard1285 == 0.0)) && (locals.var_guard1287 == 0.0)) {
        let assign42140_e55231: f64 = (-230.25850929940458);
        let assign42140_e55233: f64 = (-locals.var_sp_s_xbar);
        let assign42140_e55234: f64 = (assign42140_e55231 - assign42140_e55233);
        let assign42140_e55238: f64 = (-230.25850929940458);
        let assign42140_e55240: f64 = (-locals.var_sp_s_xbar);
        let assign42140_e55241: f64 = (assign42140_e55238 - assign42140_e55240);
        let assign42140_e55244: f64 = (-230.25850929940458);
        let assign42140_e55246: f64 = (-locals.var_sp_s_xbar);
        let assign42140_e55247: f64 = (assign42140_e55244 - assign42140_e55246);
        let assign42140_e55249: f64 = (assign42140_e55247 * 0.3333333333333333);
        let assign42140_e55250: f64 = (1.0 + assign42140_e55249);
        let assign42140_e55251: f64 = (assign42140_e55241 * assign42140_e55250);
        let assign42140_e55252: f64 = (0.5 * assign42140_e55251);
        let assign42140_e55253: f64 = (1.0 + assign42140_e55252);
        let assign42140_e55254: f64 = (assign42140_e55234 * assign42140_e55253);
        let assign42140_e55255: f64 = (1.0 + assign42140_e55254);
        let assign42140_e55256: f64 = (1e-100 / assign42140_e55255);
        (assign42140_e55256, (-((1e-100 * (((-(-locals.var_sp_s_xbar_dn5)) * assign42140_e55253) + (assign42140_e55234 * (0.5 * (((-(-locals.var_sp_s_xbar_dn5)) * assign42140_e55250) + (assign42140_e55241 * ((-(-locals.var_sp_s_xbar_dn5)) * 0.3333333333333333))))))) / (assign42140_e55255 * assign42140_e55255))), (-((1e-100 * (((-(-locals.var_sp_s_xbar_dn6)) * assign42140_e55253) + (assign42140_e55234 * (0.5 * (((-(-locals.var_sp_s_xbar_dn6)) * assign42140_e55250) + (assign42140_e55241 * ((-(-locals.var_sp_s_xbar_dn6)) * 0.3333333333333333))))))) / (assign42140_e55255 * assign42140_e55255))), (-((1e-100 * (((-(-locals.var_sp_s_xbar_dn7)) * assign42140_e55253) + (assign42140_e55234 * (0.5 * (((-(-locals.var_sp_s_xbar_dn7)) * assign42140_e55250) + (assign42140_e55241 * ((-(-locals.var_sp_s_xbar_dn7)) * 0.3333333333333333))))))) / (assign42140_e55255 * assign42140_e55255))), (-((1e-100 * (((-(-locals.var_sp_s_xbar_dn8)) * assign42140_e55253) + (assign42140_e55234 * (0.5 * (((-(-locals.var_sp_s_xbar_dn8)) * assign42140_e55250) + (assign42140_e55241 * ((-(-locals.var_sp_s_xbar_dn8)) * 0.3333333333333333))))))) / (assign42140_e55255 * assign42140_e55255))), (-((1e-100 * (((-(-locals.var_sp_s_xbar_dn12)) * assign42140_e55253) + (assign42140_e55234 * (0.5 * (((-(-locals.var_sp_s_xbar_dn12)) * assign42140_e55250) + (assign42140_e55241 * ((-(-locals.var_sp_s_xbar_dn12)) * 0.3333333333333333))))))) / (assign42140_e55255 * assign42140_e55255))), (-((1e-100 * (((-(-locals.var_sp_s_xbar_dn13)) * assign42140_e55253) + (assign42140_e55234 * (0.5 * (((-(-locals.var_sp_s_xbar_dn13)) * assign42140_e55250) + (assign42140_e55241 * ((-(-locals.var_sp_s_xbar_dn13)) * 0.3333333333333333))))))) / (assign42140_e55255 * assign42140_e55255))), (-((1e-100 * (((-(-locals.var_sp_s_xbar_dn14)) * assign42140_e55253) + (assign42140_e55234 * (0.5 * (((-(-locals.var_sp_s_xbar_dn14)) * assign42140_e55250) + (assign42140_e55241 * ((-(-locals.var_sp_s_xbar_dn14)) * 0.3333333333333333))))))) / (assign42140_e55255 * assign42140_e55255))), (-((1e-100 * (((-(-locals.var_sp_s_xbar_dn15)) * assign42140_e55253) + (assign42140_e55234 * (0.5 * (((-(-locals.var_sp_s_xbar_dn15)) * assign42140_e55250) + (assign42140_e55241 * ((-(-locals.var_sp_s_xbar_dn15)) * 0.3333333333333333))))))) / (assign42140_e55255 * assign42140_e55255))), (-((1e-100 * (((-(-locals.var_sp_s_xbar_dn16)) * assign42140_e55253) + (assign42140_e55234 * (0.5 * (((-(-locals.var_sp_s_xbar_dn16)) * assign42140_e55250) + (assign42140_e55241 * ((-(-locals.var_sp_s_xbar_dn16)) * 0.3333333333333333))))))) / (assign42140_e55255 * assign42140_e55255))), (-((1e-100 * (((-(-locals.var_sp_s_xbar_dn17)) * assign42140_e55253) + (assign42140_e55234 * (0.5 * (((-(-locals.var_sp_s_xbar_dn17)) * assign42140_e55250) + (assign42140_e55241 * ((-(-locals.var_sp_s_xbar_dn17)) * 0.3333333333333333))))))) / (assign42140_e55255 * assign42140_e55255))), (-((1e-100 * (((-(-locals.var_sp_s_xbar_dn18)) * assign42140_e55253) + (assign42140_e55234 * (0.5 * (((-(-locals.var_sp_s_xbar_dn18)) * assign42140_e55250) + (assign42140_e55241 * ((-(-locals.var_sp_s_xbar_dn18)) * 0.3333333333333333))))))) / (assign42140_e55255 * assign42140_e55255))), (-((1e-100 * (((-(-locals.var_sp_s_xbar_dn19)) * assign42140_e55253) + (assign42140_e55234 * (0.5 * (((-(-locals.var_sp_s_xbar_dn19)) * assign42140_e55250) + (assign42140_e55241 * ((-(-locals.var_sp_s_xbar_dn19)) * 0.3333333333333333))))))) / (assign42140_e55255 * assign42140_e55255))), (-((1e-100 * (((-(-locals.var_sp_s_xbar_dn20)) * assign42140_e55253) + (assign42140_e55234 * (0.5 * (((-(-locals.var_sp_s_xbar_dn20)) * assign42140_e55250) + (assign42140_e55241 * ((-(-locals.var_sp_s_xbar_dn20)) * 0.3333333333333333))))))) / (assign42140_e55255 * assign42140_e55255))),)
    } else {
        (locals.var_sp_s_temp, locals.var_sp_s_temp_dn5, locals.var_sp_s_temp_dn6, locals.var_sp_s_temp_dn7, locals.var_sp_s_temp_dn8, locals.var_sp_s_temp_dn12, locals.var_sp_s_temp_dn13, locals.var_sp_s_temp_dn14, locals.var_sp_s_temp_dn15, locals.var_sp_s_temp_dn16, locals.var_sp_s_temp_dn17, locals.var_sp_s_temp_dn18, locals.var_sp_s_temp_dn19, locals.var_sp_s_temp_dn20,)
    }
};
        locals.var_sp_s_temp = assign42140_e55258;
        locals.var_sp_s_temp_dn5 = assign42140_e55258_d_n5;
        locals.var_sp_s_temp_dn6 = assign42140_e55258_d_n6;
        locals.var_sp_s_temp_dn7 = assign42140_e55258_d_n7;
        locals.var_sp_s_temp_dn8 = assign42140_e55258_d_n8;
        locals.var_sp_s_temp_dn12 = assign42140_e55258_d_n12;
        locals.var_sp_s_temp_dn13 = assign42140_e55258_d_n13;
        locals.var_sp_s_temp_dn14 = assign42140_e55258_d_n14;
        locals.var_sp_s_temp_dn15 = assign42140_e55258_d_n15;
        locals.var_sp_s_temp_dn16 = assign42140_e55258_d_n16;
        locals.var_sp_s_temp_dn17 = assign42140_e55258_d_n17;
        locals.var_sp_s_temp_dn18 = assign42140_e55258_d_n18;
        locals.var_sp_s_temp_dn19 = assign42140_e55258_d_n19;
        locals.var_sp_s_temp_dn20 = assign42140_e55258_d_n20;
        locals.var_sp_s_temp_rv = 0.0;

        let (assign42150_e55268, assign42150_e55268_d_n5, assign42150_e55268_d_n6, assign42150_e55268_d_n7, assign42150_e55268_d_n8, assign42150_e55268_d_n12, assign42150_e55268_d_n13, assign42150_e55268_d_n14, assign42150_e55268_d_n15, assign42150_e55268_d_n16, assign42150_e55268_d_n17, assign42150_e55268_d_n18, assign42150_e55268_d_n19, assign42150_e55268_d_n20,) = {
    if ((locals.var_guard1284 == 0.0) && (locals.var_guard1285 == 0.0)) {
        let assign42150_e55266: f64 = (1.0 - locals.var_sp_s_temp);
        (assign42150_e55266, (-locals.var_sp_s_temp_dn5), (-locals.var_sp_s_temp_dn6), (-locals.var_sp_s_temp_dn7), (-locals.var_sp_s_temp_dn8), (-locals.var_sp_s_temp_dn12), (-locals.var_sp_s_temp_dn13), (-locals.var_sp_s_temp_dn14), (-locals.var_sp_s_temp_dn15), (-locals.var_sp_s_temp_dn16), (-locals.var_sp_s_temp_dn17), (-locals.var_sp_s_temp_dn18), (-locals.var_sp_s_temp_dn19), (-locals.var_sp_s_temp_dn20),)
    } else {
        (locals.var_sp_s_w, locals.var_sp_s_w_dn5, locals.var_sp_s_w_dn6, locals.var_sp_s_w_dn7, locals.var_sp_s_w_dn8, locals.var_sp_s_w_dn12, locals.var_sp_s_w_dn13, locals.var_sp_s_w_dn14, locals.var_sp_s_w_dn15, locals.var_sp_s_w_dn16, locals.var_sp_s_w_dn17, locals.var_sp_s_w_dn18, locals.var_sp_s_w_dn19, locals.var_sp_s_w_dn20,)
    }
};
        locals.var_sp_s_w = assign42150_e55268;
        locals.var_sp_s_w_dn5 = assign42150_e55268_d_n5;
        locals.var_sp_s_w_dn6 = assign42150_e55268_d_n6;
        locals.var_sp_s_w_dn7 = assign42150_e55268_d_n7;
        locals.var_sp_s_w_dn8 = assign42150_e55268_d_n8;
        locals.var_sp_s_w_dn12 = assign42150_e55268_d_n12;
        locals.var_sp_s_w_dn13 = assign42150_e55268_d_n13;
        locals.var_sp_s_w_dn14 = assign42150_e55268_d_n14;
        locals.var_sp_s_w_dn15 = assign42150_e55268_d_n15;
        locals.var_sp_s_w_dn16 = assign42150_e55268_d_n16;
        locals.var_sp_s_w_dn17 = assign42150_e55268_d_n17;
        locals.var_sp_s_w_dn18 = assign42150_e55268_d_n18;
        locals.var_sp_s_w_dn19 = assign42150_e55268_d_n19;
        locals.var_sp_s_w_dn20 = assign42150_e55268_d_n20;
        locals.var_sp_s_w_rv = 0.0;

        let (assign42160_e55291, assign42160_e55291_d_n5, assign42160_e55291_d_n6, assign42160_e55291_d_n7, assign42160_e55291_d_n8, assign42160_e55291_d_n12, assign42160_e55291_d_n13, assign42160_e55291_d_n14, assign42160_e55291_d_n15, assign42160_e55291_d_n16, assign42160_e55291_d_n17, assign42160_e55291_d_n18, assign42160_e55291_d_n19, assign42160_e55291_d_n20,) = {
    if ((locals.var_guard1284 == 0.0) && (locals.var_guard1285 == 0.0)) {
        let assign42160_e55277: f64 = (locals.var_gf2 * 0.5);
        let assign42160_e55278: f64 = (locals.var_xg + assign42160_e55277);
        let assign42160_e55283: f64 = (locals.var_gf2 * 0.25);
        let assign42160_e55284: f64 = (locals.var_xg + assign42160_e55283);
        let assign42160_e55286: f64 = (assign42160_e55284 - locals.var_sp_s_w);
        let assign42160_e55287: f64 = (assign42160_e55286).sqrt();
        let assign42160_e55288: f64 = (locals.var_gf * assign42160_e55287);
        let assign42160_e55289: f64 = (assign42160_e55278 - assign42160_e55288);
        (assign42160_e55289, ((locals.var_xg_dn5 + (locals.var_gf2_dn5 * 0.5)) - ((locals.var_gf_dn5 * assign42160_e55287) + (locals.var_gf * (((locals.var_xg_dn5 + (locals.var_gf2_dn5 * 0.25)) - locals.var_sp_s_w_dn5) / (2.0 * assign42160_e55287))))), ((locals.var_xg_dn6 + (locals.var_gf2_dn6 * 0.5)) - ((locals.var_gf_dn6 * assign42160_e55287) + (locals.var_gf * (((locals.var_xg_dn6 + (locals.var_gf2_dn6 * 0.25)) - locals.var_sp_s_w_dn6) / (2.0 * assign42160_e55287))))), ((locals.var_xg_dn7 + (locals.var_gf2_dn7 * 0.5)) - ((locals.var_gf_dn7 * assign42160_e55287) + (locals.var_gf * (((locals.var_xg_dn7 + (locals.var_gf2_dn7 * 0.25)) - locals.var_sp_s_w_dn7) / (2.0 * assign42160_e55287))))), ((locals.var_xg_dn8 + (locals.var_gf2_dn8 * 0.5)) - ((locals.var_gf_dn8 * assign42160_e55287) + (locals.var_gf * (((locals.var_xg_dn8 + (locals.var_gf2_dn8 * 0.25)) - locals.var_sp_s_w_dn8) / (2.0 * assign42160_e55287))))), ((locals.var_xg_dn12 + (locals.var_gf2_dn12 * 0.5)) - ((locals.var_gf_dn12 * assign42160_e55287) + (locals.var_gf * (((locals.var_xg_dn12 + (locals.var_gf2_dn12 * 0.25)) - locals.var_sp_s_w_dn12) / (2.0 * assign42160_e55287))))), ((locals.var_xg_dn13 + (locals.var_gf2_dn13 * 0.5)) - ((locals.var_gf_dn13 * assign42160_e55287) + (locals.var_gf * (((locals.var_xg_dn13 + (locals.var_gf2_dn13 * 0.25)) - locals.var_sp_s_w_dn13) / (2.0 * assign42160_e55287))))), ((locals.var_xg_dn14 + (locals.var_gf2_dn14 * 0.5)) - ((locals.var_gf_dn14 * assign42160_e55287) + (locals.var_gf * (((locals.var_xg_dn14 + (locals.var_gf2_dn14 * 0.25)) - locals.var_sp_s_w_dn14) / (2.0 * assign42160_e55287))))), ((locals.var_xg_dn15 + (locals.var_gf2_dn15 * 0.5)) - ((locals.var_gf_dn15 * assign42160_e55287) + (locals.var_gf * (((locals.var_xg_dn15 + (locals.var_gf2_dn15 * 0.25)) - locals.var_sp_s_w_dn15) / (2.0 * assign42160_e55287))))), ((locals.var_xg_dn16 + (locals.var_gf2_dn16 * 0.5)) - ((locals.var_gf_dn16 * assign42160_e55287) + (locals.var_gf * (((locals.var_xg_dn16 + (locals.var_gf2_dn16 * 0.25)) - locals.var_sp_s_w_dn16) / (2.0 * assign42160_e55287))))), ((locals.var_xg_dn17 + (locals.var_gf2_dn17 * 0.5)) - ((locals.var_gf_dn17 * assign42160_e55287) + (locals.var_gf * (((locals.var_xg_dn17 + (locals.var_gf2_dn17 * 0.25)) - locals.var_sp_s_w_dn17) / (2.0 * assign42160_e55287))))), ((locals.var_xg_dn18 + (locals.var_gf2_dn18 * 0.5)) - ((locals.var_gf_dn18 * assign42160_e55287) + (locals.var_gf * (((locals.var_xg_dn18 + (locals.var_gf2_dn18 * 0.25)) - locals.var_sp_s_w_dn18) / (2.0 * assign42160_e55287))))), ((locals.var_xg_dn19 + (locals.var_gf2_dn19 * 0.5)) - ((locals.var_gf_dn19 * assign42160_e55287) + (locals.var_gf * (((locals.var_xg_dn19 + (locals.var_gf2_dn19 * 0.25)) - locals.var_sp_s_w_dn19) / (2.0 * assign42160_e55287))))), ((locals.var_xg_dn20 + (locals.var_gf2_dn20 * 0.5)) - ((locals.var_gf_dn20 * assign42160_e55287) + (locals.var_gf * (((locals.var_xg_dn20 + (locals.var_gf2_dn20 * 0.25)) - locals.var_sp_s_w_dn20) / (2.0 * assign42160_e55287))))),)
    } else {
        (locals.var_sp_s_x1, locals.var_sp_s_x1_dn5, locals.var_sp_s_x1_dn6, locals.var_sp_s_x1_dn7, locals.var_sp_s_x1_dn8, locals.var_sp_s_x1_dn12, locals.var_sp_s_x1_dn13, locals.var_sp_s_x1_dn14, locals.var_sp_s_x1_dn15, locals.var_sp_s_x1_dn16, locals.var_sp_s_x1_dn17, locals.var_sp_s_x1_dn18, locals.var_sp_s_x1_dn19, locals.var_sp_s_x1_dn20,)
    }
};
        locals.var_sp_s_x1 = assign42160_e55291;
        locals.var_sp_s_x1_dn5 = assign42160_e55291_d_n5;
        locals.var_sp_s_x1_dn6 = assign42160_e55291_d_n6;
        locals.var_sp_s_x1_dn7 = assign42160_e55291_d_n7;
        locals.var_sp_s_x1_dn8 = assign42160_e55291_d_n8;
        locals.var_sp_s_x1_dn12 = assign42160_e55291_d_n12;
        locals.var_sp_s_x1_dn13 = assign42160_e55291_d_n13;
        locals.var_sp_s_x1_dn14 = assign42160_e55291_d_n14;
        locals.var_sp_s_x1_dn15 = assign42160_e55291_d_n15;
        locals.var_sp_s_x1_dn16 = assign42160_e55291_d_n16;
        locals.var_sp_s_x1_dn17 = assign42160_e55291_d_n17;
        locals.var_sp_s_x1_dn18 = assign42160_e55291_d_n18;
        locals.var_sp_s_x1_dn19 = assign42160_e55291_d_n19;
        locals.var_sp_s_x1_dn20 = assign42160_e55291_d_n20;
        locals.var_sp_s_x1_rv = 0.0;

        let (assign42170_e55301, assign42170_e55301_d_n5, assign42170_e55301_d_n6, assign42170_e55301_d_n7, assign42170_e55301_d_n8, assign42170_e55301_d_n12, assign42170_e55301_d_n13, assign42170_e55301_d_n14, assign42170_e55301_d_n15, assign42170_e55301_d_n16, assign42170_e55301_d_n17, assign42170_e55301_d_n18, assign42170_e55301_d_n19, assign42170_e55301_d_n20,) = {
    if ((locals.var_guard1284 == 0.0) && (locals.var_guard1285 == 0.0)) {
        let assign42170_e55299: f64 = (locals.var_xn_s + 3.0);
        (assign42170_e55299, locals.var_xn_s_dn5, locals.var_xn_s_dn6, locals.var_xn_s_dn7, locals.var_xn_s_dn8, locals.var_xn_s_dn12, locals.var_xn_s_dn13, locals.var_xn_s_dn14, locals.var_xn_s_dn15, locals.var_xn_s_dn16, locals.var_xn_s_dn17, locals.var_xn_s_dn18, locals.var_xn_s_dn19, locals.var_xn_s_dn20,)
    } else {
        (locals.var_sp_s_bx, locals.var_sp_s_bx_dn5, locals.var_sp_s_bx_dn6, locals.var_sp_s_bx_dn7, locals.var_sp_s_bx_dn8, locals.var_sp_s_bx_dn12, locals.var_sp_s_bx_dn13, locals.var_sp_s_bx_dn14, locals.var_sp_s_bx_dn15, locals.var_sp_s_bx_dn16, locals.var_sp_s_bx_dn17, locals.var_sp_s_bx_dn18, locals.var_sp_s_bx_dn19, locals.var_sp_s_bx_dn20,)
    }
};
        locals.var_sp_s_bx = assign42170_e55301;
        locals.var_sp_s_bx_dn5 = assign42170_e55301_d_n5;
        locals.var_sp_s_bx_dn6 = assign42170_e55301_d_n6;
        locals.var_sp_s_bx_dn7 = assign42170_e55301_d_n7;
        locals.var_sp_s_bx_dn8 = assign42170_e55301_d_n8;
        locals.var_sp_s_bx_dn12 = assign42170_e55301_d_n12;
        locals.var_sp_s_bx_dn13 = assign42170_e55301_d_n13;
        locals.var_sp_s_bx_dn14 = assign42170_e55301_d_n14;
        locals.var_sp_s_bx_dn15 = assign42170_e55301_d_n15;
        locals.var_sp_s_bx_dn16 = assign42170_e55301_d_n16;
        locals.var_sp_s_bx_dn17 = assign42170_e55301_d_n17;
        locals.var_sp_s_bx_dn18 = assign42170_e55301_d_n18;
        locals.var_sp_s_bx_dn19 = assign42170_e55301_d_n19;
        locals.var_sp_s_bx_dn20 = assign42170_e55301_d_n20;
        locals.var_sp_s_bx_rv = 0.0;

        let (assign42180_e55335, assign42180_e55335_d_n5, assign42180_e55335_d_n6, assign42180_e55335_d_n7, assign42180_e55335_d_n8, assign42180_e55335_d_n12, assign42180_e55335_d_n13, assign42180_e55335_d_n14, assign42180_e55335_d_n15, assign42180_e55335_d_n16, assign42180_e55335_d_n17, assign42180_e55335_d_n18, assign42180_e55335_d_n19, assign42180_e55335_d_n20,) = {
    if ((locals.var_guard1284 == 0.0) && (locals.var_guard1285 == 0.0)) {
        let assign42180_e55310: f64 = (locals.var_sp_s_x1 + locals.var_sp_s_bx);
        let assign42180_e55313: f64 = (locals.var_sp_s_x1 - locals.var_sp_s_bx);
        let assign42180_e55316: f64 = (locals.var_sp_s_x1 - locals.var_sp_s_bx);
        let assign42180_e55317: f64 = (assign42180_e55313 * assign42180_e55316);
        let assign42180_e55319: f64 = (assign42180_e55317 + 5.0);
        let assign42180_e55320: f64 = (assign42180_e55319).sqrt();
        let assign42180_e55321: f64 = (assign42180_e55310 - assign42180_e55320);
        let assign42180_e55322: f64 = (0.5 * assign42180_e55321);
        let assign42180_e55327: f64 = (locals.var_sp_s_bx * locals.var_sp_s_bx);
        let assign42180_e55329: f64 = (assign42180_e55327 + 5.0);
        let assign42180_e55330: f64 = (assign42180_e55329).sqrt();
        let assign42180_e55331: f64 = (locals.var_sp_s_bx - assign42180_e55330);
        let assign42180_e55332: f64 = (0.5 * assign42180_e55331);
        let assign42180_e55333: f64 = (assign42180_e55322 - assign42180_e55332);
        (assign42180_e55333, ((0.5 * ((locals.var_sp_s_x1_dn5 + locals.var_sp_s_bx_dn5) - ((((locals.var_sp_s_x1_dn5 - locals.var_sp_s_bx_dn5) * assign42180_e55316) + (assign42180_e55313 * (locals.var_sp_s_x1_dn5 - locals.var_sp_s_bx_dn5))) / (2.0 * assign42180_e55320)))) - (0.5 * (locals.var_sp_s_bx_dn5 - (((locals.var_sp_s_bx_dn5 * locals.var_sp_s_bx) + (locals.var_sp_s_bx * locals.var_sp_s_bx_dn5)) / (2.0 * assign42180_e55330))))), ((0.5 * ((locals.var_sp_s_x1_dn6 + locals.var_sp_s_bx_dn6) - ((((locals.var_sp_s_x1_dn6 - locals.var_sp_s_bx_dn6) * assign42180_e55316) + (assign42180_e55313 * (locals.var_sp_s_x1_dn6 - locals.var_sp_s_bx_dn6))) / (2.0 * assign42180_e55320)))) - (0.5 * (locals.var_sp_s_bx_dn6 - (((locals.var_sp_s_bx_dn6 * locals.var_sp_s_bx) + (locals.var_sp_s_bx * locals.var_sp_s_bx_dn6)) / (2.0 * assign42180_e55330))))), ((0.5 * ((locals.var_sp_s_x1_dn7 + locals.var_sp_s_bx_dn7) - ((((locals.var_sp_s_x1_dn7 - locals.var_sp_s_bx_dn7) * assign42180_e55316) + (assign42180_e55313 * (locals.var_sp_s_x1_dn7 - locals.var_sp_s_bx_dn7))) / (2.0 * assign42180_e55320)))) - (0.5 * (locals.var_sp_s_bx_dn7 - (((locals.var_sp_s_bx_dn7 * locals.var_sp_s_bx) + (locals.var_sp_s_bx * locals.var_sp_s_bx_dn7)) / (2.0 * assign42180_e55330))))), ((0.5 * ((locals.var_sp_s_x1_dn8 + locals.var_sp_s_bx_dn8) - ((((locals.var_sp_s_x1_dn8 - locals.var_sp_s_bx_dn8) * assign42180_e55316) + (assign42180_e55313 * (locals.var_sp_s_x1_dn8 - locals.var_sp_s_bx_dn8))) / (2.0 * assign42180_e55320)))) - (0.5 * (locals.var_sp_s_bx_dn8 - (((locals.var_sp_s_bx_dn8 * locals.var_sp_s_bx) + (locals.var_sp_s_bx * locals.var_sp_s_bx_dn8)) / (2.0 * assign42180_e55330))))), ((0.5 * ((locals.var_sp_s_x1_dn12 + locals.var_sp_s_bx_dn12) - ((((locals.var_sp_s_x1_dn12 - locals.var_sp_s_bx_dn12) * assign42180_e55316) + (assign42180_e55313 * (locals.var_sp_s_x1_dn12 - locals.var_sp_s_bx_dn12))) / (2.0 * assign42180_e55320)))) - (0.5 * (locals.var_sp_s_bx_dn12 - (((locals.var_sp_s_bx_dn12 * locals.var_sp_s_bx) + (locals.var_sp_s_bx * locals.var_sp_s_bx_dn12)) / (2.0 * assign42180_e55330))))), ((0.5 * ((locals.var_sp_s_x1_dn13 + locals.var_sp_s_bx_dn13) - ((((locals.var_sp_s_x1_dn13 - locals.var_sp_s_bx_dn13) * assign42180_e55316) + (assign42180_e55313 * (locals.var_sp_s_x1_dn13 - locals.var_sp_s_bx_dn13))) / (2.0 * assign42180_e55320)))) - (0.5 * (locals.var_sp_s_bx_dn13 - (((locals.var_sp_s_bx_dn13 * locals.var_sp_s_bx) + (locals.var_sp_s_bx * locals.var_sp_s_bx_dn13)) / (2.0 * assign42180_e55330))))), ((0.5 * ((locals.var_sp_s_x1_dn14 + locals.var_sp_s_bx_dn14) - ((((locals.var_sp_s_x1_dn14 - locals.var_sp_s_bx_dn14) * assign42180_e55316) + (assign42180_e55313 * (locals.var_sp_s_x1_dn14 - locals.var_sp_s_bx_dn14))) / (2.0 * assign42180_e55320)))) - (0.5 * (locals.var_sp_s_bx_dn14 - (((locals.var_sp_s_bx_dn14 * locals.var_sp_s_bx) + (locals.var_sp_s_bx * locals.var_sp_s_bx_dn14)) / (2.0 * assign42180_e55330))))), ((0.5 * ((locals.var_sp_s_x1_dn15 + locals.var_sp_s_bx_dn15) - ((((locals.var_sp_s_x1_dn15 - locals.var_sp_s_bx_dn15) * assign42180_e55316) + (assign42180_e55313 * (locals.var_sp_s_x1_dn15 - locals.var_sp_s_bx_dn15))) / (2.0 * assign42180_e55320)))) - (0.5 * (locals.var_sp_s_bx_dn15 - (((locals.var_sp_s_bx_dn15 * locals.var_sp_s_bx) + (locals.var_sp_s_bx * locals.var_sp_s_bx_dn15)) / (2.0 * assign42180_e55330))))), ((0.5 * ((locals.var_sp_s_x1_dn16 + locals.var_sp_s_bx_dn16) - ((((locals.var_sp_s_x1_dn16 - locals.var_sp_s_bx_dn16) * assign42180_e55316) + (assign42180_e55313 * (locals.var_sp_s_x1_dn16 - locals.var_sp_s_bx_dn16))) / (2.0 * assign42180_e55320)))) - (0.5 * (locals.var_sp_s_bx_dn16 - (((locals.var_sp_s_bx_dn16 * locals.var_sp_s_bx) + (locals.var_sp_s_bx * locals.var_sp_s_bx_dn16)) / (2.0 * assign42180_e55330))))), ((0.5 * ((locals.var_sp_s_x1_dn17 + locals.var_sp_s_bx_dn17) - ((((locals.var_sp_s_x1_dn17 - locals.var_sp_s_bx_dn17) * assign42180_e55316) + (assign42180_e55313 * (locals.var_sp_s_x1_dn17 - locals.var_sp_s_bx_dn17))) / (2.0 * assign42180_e55320)))) - (0.5 * (locals.var_sp_s_bx_dn17 - (((locals.var_sp_s_bx_dn17 * locals.var_sp_s_bx) + (locals.var_sp_s_bx * locals.var_sp_s_bx_dn17)) / (2.0 * assign42180_e55330))))), ((0.5 * ((locals.var_sp_s_x1_dn18 + locals.var_sp_s_bx_dn18) - ((((locals.var_sp_s_x1_dn18 - locals.var_sp_s_bx_dn18) * assign42180_e55316) + (assign42180_e55313 * (locals.var_sp_s_x1_dn18 - locals.var_sp_s_bx_dn18))) / (2.0 * assign42180_e55320)))) - (0.5 * (locals.var_sp_s_bx_dn18 - (((locals.var_sp_s_bx_dn18 * locals.var_sp_s_bx) + (locals.var_sp_s_bx * locals.var_sp_s_bx_dn18)) / (2.0 * assign42180_e55330))))), ((0.5 * ((locals.var_sp_s_x1_dn19 + locals.var_sp_s_bx_dn19) - ((((locals.var_sp_s_x1_dn19 - locals.var_sp_s_bx_dn19) * assign42180_e55316) + (assign42180_e55313 * (locals.var_sp_s_x1_dn19 - locals.var_sp_s_bx_dn19))) / (2.0 * assign42180_e55320)))) - (0.5 * (locals.var_sp_s_bx_dn19 - (((locals.var_sp_s_bx_dn19 * locals.var_sp_s_bx) + (locals.var_sp_s_bx * locals.var_sp_s_bx_dn19)) / (2.0 * assign42180_e55330))))), ((0.5 * ((locals.var_sp_s_x1_dn20 + locals.var_sp_s_bx_dn20) - ((((locals.var_sp_s_x1_dn20 - locals.var_sp_s_bx_dn20) * assign42180_e55316) + (assign42180_e55313 * (locals.var_sp_s_x1_dn20 - locals.var_sp_s_bx_dn20))) / (2.0 * assign42180_e55320)))) - (0.5 * (locals.var_sp_s_bx_dn20 - (((locals.var_sp_s_bx_dn20 * locals.var_sp_s_bx) + (locals.var_sp_s_bx * locals.var_sp_s_bx_dn20)) / (2.0 * assign42180_e55330))))),)
    } else {
        (locals.var_sp_s_eta, locals.var_sp_s_eta_dn5, locals.var_sp_s_eta_dn6, locals.var_sp_s_eta_dn7, locals.var_sp_s_eta_dn8, locals.var_sp_s_eta_dn12, locals.var_sp_s_eta_dn13, locals.var_sp_s_eta_dn14, locals.var_sp_s_eta_dn15, locals.var_sp_s_eta_dn16, locals.var_sp_s_eta_dn17, locals.var_sp_s_eta_dn18, locals.var_sp_s_eta_dn19, locals.var_sp_s_eta_dn20,)
    }
};
        locals.var_sp_s_eta = assign42180_e55335;
        locals.var_sp_s_eta_dn5 = assign42180_e55335_d_n5;
        locals.var_sp_s_eta_dn6 = assign42180_e55335_d_n6;
        locals.var_sp_s_eta_dn7 = assign42180_e55335_d_n7;
        locals.var_sp_s_eta_dn8 = assign42180_e55335_d_n8;
        locals.var_sp_s_eta_dn12 = assign42180_e55335_d_n12;
        locals.var_sp_s_eta_dn13 = assign42180_e55335_d_n13;
        locals.var_sp_s_eta_dn14 = assign42180_e55335_d_n14;
        locals.var_sp_s_eta_dn15 = assign42180_e55335_d_n15;
        locals.var_sp_s_eta_dn16 = assign42180_e55335_d_n16;
        locals.var_sp_s_eta_dn17 = assign42180_e55335_d_n17;
        locals.var_sp_s_eta_dn18 = assign42180_e55335_d_n18;
        locals.var_sp_s_eta_dn19 = assign42180_e55335_d_n19;
        locals.var_sp_s_eta_dn20 = assign42180_e55335_d_n20;
        locals.var_sp_s_eta_rv = 0.0;

        let (assign42190_e55345, assign42190_e55345_d_n5, assign42190_e55345_d_n6, assign42190_e55345_d_n7, assign42190_e55345_d_n8, assign42190_e55345_d_n12, assign42190_e55345_d_n13, assign42190_e55345_d_n14, assign42190_e55345_d_n15, assign42190_e55345_d_n16, assign42190_e55345_d_n17, assign42190_e55345_d_n18, assign42190_e55345_d_n19, assign42190_e55345_d_n20,) = {
    if ((locals.var_guard1284 == 0.0) && (locals.var_guard1285 == 0.0)) {
        let assign42190_e55343: f64 = (locals.var_xg - locals.var_sp_s_eta);
        (assign42190_e55343, (locals.var_xg_dn5 - locals.var_sp_s_eta_dn5), (locals.var_xg_dn6 - locals.var_sp_s_eta_dn6), (locals.var_xg_dn7 - locals.var_sp_s_eta_dn7), (locals.var_xg_dn8 - locals.var_sp_s_eta_dn8), (locals.var_xg_dn12 - locals.var_sp_s_eta_dn12), (locals.var_xg_dn13 - locals.var_sp_s_eta_dn13), (locals.var_xg_dn14 - locals.var_sp_s_eta_dn14), (locals.var_xg_dn15 - locals.var_sp_s_eta_dn15), (locals.var_xg_dn16 - locals.var_sp_s_eta_dn16), (locals.var_xg_dn17 - locals.var_sp_s_eta_dn17), (locals.var_xg_dn18 - locals.var_sp_s_eta_dn18), (locals.var_xg_dn19 - locals.var_sp_s_eta_dn19), (locals.var_xg_dn20 - locals.var_sp_s_eta_dn20),)
    } else {
        (locals.var_sp_s_temp, locals.var_sp_s_temp_dn5, locals.var_sp_s_temp_dn6, locals.var_sp_s_temp_dn7, locals.var_sp_s_temp_dn8, locals.var_sp_s_temp_dn12, locals.var_sp_s_temp_dn13, locals.var_sp_s_temp_dn14, locals.var_sp_s_temp_dn15, locals.var_sp_s_temp_dn16, locals.var_sp_s_temp_dn17, locals.var_sp_s_temp_dn18, locals.var_sp_s_temp_dn19, locals.var_sp_s_temp_dn20,)
    }
};
        locals.var_sp_s_temp = assign42190_e55345;
        locals.var_sp_s_temp_dn5 = assign42190_e55345_d_n5;
        locals.var_sp_s_temp_dn6 = assign42190_e55345_d_n6;
        locals.var_sp_s_temp_dn7 = assign42190_e55345_d_n7;
        locals.var_sp_s_temp_dn8 = assign42190_e55345_d_n8;
        locals.var_sp_s_temp_dn12 = assign42190_e55345_d_n12;
        locals.var_sp_s_temp_dn13 = assign42190_e55345_d_n13;
        locals.var_sp_s_temp_dn14 = assign42190_e55345_d_n14;
        locals.var_sp_s_temp_dn15 = assign42190_e55345_d_n15;
        locals.var_sp_s_temp_dn16 = assign42190_e55345_d_n16;
        locals.var_sp_s_temp_dn17 = assign42190_e55345_d_n17;
        locals.var_sp_s_temp_dn18 = assign42190_e55345_d_n18;
        locals.var_sp_s_temp_dn19 = assign42190_e55345_d_n19;
        locals.var_sp_s_temp_dn20 = assign42190_e55345_d_n20;
        locals.var_sp_s_temp_rv = 0.0;

        let (assign42200_e55355, assign42200_e55355_d_n5, assign42200_e55355_d_n6, assign42200_e55355_d_n7, assign42200_e55355_d_n8, assign42200_e55355_d_n12, assign42200_e55355_d_n13, assign42200_e55355_d_n14, assign42200_e55355_d_n15, assign42200_e55355_d_n16, assign42200_e55355_d_n17, assign42200_e55355_d_n18, assign42200_e55355_d_n19, assign42200_e55355_d_n20,) = {
    if ((locals.var_guard1284 == 0.0) && (locals.var_guard1285 == 0.0)) {
        let assign42200_e55352: f64 = (-locals.var_sp_s_eta);
        let assign42200_e55353: f64 = (assign42200_e55352).exp();
        (assign42200_e55353, (assign42200_e55353 * (-locals.var_sp_s_eta_dn5)), (assign42200_e55353 * (-locals.var_sp_s_eta_dn6)), (assign42200_e55353 * (-locals.var_sp_s_eta_dn7)), (assign42200_e55353 * (-locals.var_sp_s_eta_dn8)), (assign42200_e55353 * (-locals.var_sp_s_eta_dn12)), (assign42200_e55353 * (-locals.var_sp_s_eta_dn13)), (assign42200_e55353 * (-locals.var_sp_s_eta_dn14)), (assign42200_e55353 * (-locals.var_sp_s_eta_dn15)), (assign42200_e55353 * (-locals.var_sp_s_eta_dn16)), (assign42200_e55353 * (-locals.var_sp_s_eta_dn17)), (assign42200_e55353 * (-locals.var_sp_s_eta_dn18)), (assign42200_e55353 * (-locals.var_sp_s_eta_dn19)), (assign42200_e55353 * (-locals.var_sp_s_eta_dn20)),)
    } else {
        (locals.var_sp_s_temp1, locals.var_sp_s_temp1_dn5, locals.var_sp_s_temp1_dn6, locals.var_sp_s_temp1_dn7, locals.var_sp_s_temp1_dn8, locals.var_sp_s_temp1_dn12, locals.var_sp_s_temp1_dn13, locals.var_sp_s_temp1_dn14, locals.var_sp_s_temp1_dn15, locals.var_sp_s_temp1_dn16, locals.var_sp_s_temp1_dn17, locals.var_sp_s_temp1_dn18, locals.var_sp_s_temp1_dn19, locals.var_sp_s_temp1_dn20,)
    }
};
        locals.var_sp_s_temp1 = assign42200_e55355;
        locals.var_sp_s_temp1_dn5 = assign42200_e55355_d_n5;
        locals.var_sp_s_temp1_dn6 = assign42200_e55355_d_n6;
        locals.var_sp_s_temp1_dn7 = assign42200_e55355_d_n7;
        locals.var_sp_s_temp1_dn8 = assign42200_e55355_d_n8;
        locals.var_sp_s_temp1_dn12 = assign42200_e55355_d_n12;
        locals.var_sp_s_temp1_dn13 = assign42200_e55355_d_n13;
        locals.var_sp_s_temp1_dn14 = assign42200_e55355_d_n14;
        locals.var_sp_s_temp1_dn15 = assign42200_e55355_d_n15;
        locals.var_sp_s_temp1_dn16 = assign42200_e55355_d_n16;
        locals.var_sp_s_temp1_dn17 = assign42200_e55355_d_n17;
        locals.var_sp_s_temp1_dn18 = assign42200_e55355_d_n18;
        locals.var_sp_s_temp1_dn19 = assign42200_e55355_d_n19;
        locals.var_sp_s_temp1_dn20 = assign42200_e55355_d_n20;
        locals.var_sp_s_temp1_rv = 0.0;

        let (assign42210_e55369, assign42210_e55369_d_n5, assign42210_e55369_d_n6, assign42210_e55369_d_n7, assign42210_e55369_d_n8, assign42210_e55369_d_n12, assign42210_e55369_d_n13, assign42210_e55369_d_n14, assign42210_e55369_d_n15, assign42210_e55369_d_n16, assign42210_e55369_d_n17, assign42210_e55369_d_n18, assign42210_e55369_d_n19, assign42210_e55369_d_n20,) = {
    if ((locals.var_guard1284 == 0.0) && (locals.var_guard1285 == 0.0)) {
        let assign42210_e55365: f64 = (locals.var_sp_s_eta * locals.var_sp_s_eta);
        let assign42210_e55366: f64 = (2.0 + assign42210_e55365);
        let assign42210_e55367: f64 = (1.0 / assign42210_e55366);
        (assign42210_e55367, (-(((locals.var_sp_s_eta_dn5 * locals.var_sp_s_eta) + (locals.var_sp_s_eta * locals.var_sp_s_eta_dn5)) / (assign42210_e55366 * assign42210_e55366))), (-(((locals.var_sp_s_eta_dn6 * locals.var_sp_s_eta) + (locals.var_sp_s_eta * locals.var_sp_s_eta_dn6)) / (assign42210_e55366 * assign42210_e55366))), (-(((locals.var_sp_s_eta_dn7 * locals.var_sp_s_eta) + (locals.var_sp_s_eta * locals.var_sp_s_eta_dn7)) / (assign42210_e55366 * assign42210_e55366))), (-(((locals.var_sp_s_eta_dn8 * locals.var_sp_s_eta) + (locals.var_sp_s_eta * locals.var_sp_s_eta_dn8)) / (assign42210_e55366 * assign42210_e55366))), (-(((locals.var_sp_s_eta_dn12 * locals.var_sp_s_eta) + (locals.var_sp_s_eta * locals.var_sp_s_eta_dn12)) / (assign42210_e55366 * assign42210_e55366))), (-(((locals.var_sp_s_eta_dn13 * locals.var_sp_s_eta) + (locals.var_sp_s_eta * locals.var_sp_s_eta_dn13)) / (assign42210_e55366 * assign42210_e55366))), (-(((locals.var_sp_s_eta_dn14 * locals.var_sp_s_eta) + (locals.var_sp_s_eta * locals.var_sp_s_eta_dn14)) / (assign42210_e55366 * assign42210_e55366))), (-(((locals.var_sp_s_eta_dn15 * locals.var_sp_s_eta) + (locals.var_sp_s_eta * locals.var_sp_s_eta_dn15)) / (assign42210_e55366 * assign42210_e55366))), (-(((locals.var_sp_s_eta_dn16 * locals.var_sp_s_eta) + (locals.var_sp_s_eta * locals.var_sp_s_eta_dn16)) / (assign42210_e55366 * assign42210_e55366))), (-(((locals.var_sp_s_eta_dn17 * locals.var_sp_s_eta) + (locals.var_sp_s_eta * locals.var_sp_s_eta_dn17)) / (assign42210_e55366 * assign42210_e55366))), (-(((locals.var_sp_s_eta_dn18 * locals.var_sp_s_eta) + (locals.var_sp_s_eta * locals.var_sp_s_eta_dn18)) / (assign42210_e55366 * assign42210_e55366))), (-(((locals.var_sp_s_eta_dn19 * locals.var_sp_s_eta) + (locals.var_sp_s_eta * locals.var_sp_s_eta_dn19)) / (assign42210_e55366 * assign42210_e55366))), (-(((locals.var_sp_s_eta_dn20 * locals.var_sp_s_eta) + (locals.var_sp_s_eta * locals.var_sp_s_eta_dn20)) / (assign42210_e55366 * assign42210_e55366))),)
    } else {
        (locals.var_sp_s_temp2, locals.var_sp_s_temp2_dn5, locals.var_sp_s_temp2_dn6, locals.var_sp_s_temp2_dn7, locals.var_sp_s_temp2_dn8, locals.var_sp_s_temp2_dn12, locals.var_sp_s_temp2_dn13, locals.var_sp_s_temp2_dn14, locals.var_sp_s_temp2_dn15, locals.var_sp_s_temp2_dn16, locals.var_sp_s_temp2_dn17, locals.var_sp_s_temp2_dn18, locals.var_sp_s_temp2_dn19, locals.var_sp_s_temp2_dn20,)
    }
};
        locals.var_sp_s_temp2 = assign42210_e55369;
        locals.var_sp_s_temp2_dn5 = assign42210_e55369_d_n5;
        locals.var_sp_s_temp2_dn6 = assign42210_e55369_d_n6;
        locals.var_sp_s_temp2_dn7 = assign42210_e55369_d_n7;
        locals.var_sp_s_temp2_dn8 = assign42210_e55369_d_n8;
        locals.var_sp_s_temp2_dn12 = assign42210_e55369_d_n12;
        locals.var_sp_s_temp2_dn13 = assign42210_e55369_d_n13;
        locals.var_sp_s_temp2_dn14 = assign42210_e55369_d_n14;
        locals.var_sp_s_temp2_dn15 = assign42210_e55369_d_n15;
        locals.var_sp_s_temp2_dn16 = assign42210_e55369_d_n16;
        locals.var_sp_s_temp2_dn17 = assign42210_e55369_d_n17;
        locals.var_sp_s_temp2_dn18 = assign42210_e55369_d_n18;
        locals.var_sp_s_temp2_dn19 = assign42210_e55369_d_n19;
        locals.var_sp_s_temp2_dn20 = assign42210_e55369_d_n20;
        locals.var_sp_s_temp2_rv = 0.0;

        let (assign42220_e55381, assign42220_e55381_d_n5, assign42220_e55381_d_n6, assign42220_e55381_d_n7, assign42220_e55381_d_n8, assign42220_e55381_d_n12, assign42220_e55381_d_n13, assign42220_e55381_d_n14, assign42220_e55381_d_n15, assign42220_e55381_d_n16, assign42220_e55381_d_n17, assign42220_e55381_d_n18, assign42220_e55381_d_n19, assign42220_e55381_d_n20,) = {
    if ((locals.var_guard1284 == 0.0) && (locals.var_guard1285 == 0.0)) {
        let assign42220_e55377: f64 = (locals.var_sp_s_eta * locals.var_sp_s_eta);
        let assign42220_e55379: f64 = (assign42220_e55377 * locals.var_sp_s_temp2);
        (assign42220_e55379, ((((locals.var_sp_s_eta_dn5 * locals.var_sp_s_eta) + (locals.var_sp_s_eta * locals.var_sp_s_eta_dn5)) * locals.var_sp_s_temp2) + (assign42220_e55377 * locals.var_sp_s_temp2_dn5)), ((((locals.var_sp_s_eta_dn6 * locals.var_sp_s_eta) + (locals.var_sp_s_eta * locals.var_sp_s_eta_dn6)) * locals.var_sp_s_temp2) + (assign42220_e55377 * locals.var_sp_s_temp2_dn6)), ((((locals.var_sp_s_eta_dn7 * locals.var_sp_s_eta) + (locals.var_sp_s_eta * locals.var_sp_s_eta_dn7)) * locals.var_sp_s_temp2) + (assign42220_e55377 * locals.var_sp_s_temp2_dn7)), ((((locals.var_sp_s_eta_dn8 * locals.var_sp_s_eta) + (locals.var_sp_s_eta * locals.var_sp_s_eta_dn8)) * locals.var_sp_s_temp2) + (assign42220_e55377 * locals.var_sp_s_temp2_dn8)), ((((locals.var_sp_s_eta_dn12 * locals.var_sp_s_eta) + (locals.var_sp_s_eta * locals.var_sp_s_eta_dn12)) * locals.var_sp_s_temp2) + (assign42220_e55377 * locals.var_sp_s_temp2_dn12)), ((((locals.var_sp_s_eta_dn13 * locals.var_sp_s_eta) + (locals.var_sp_s_eta * locals.var_sp_s_eta_dn13)) * locals.var_sp_s_temp2) + (assign42220_e55377 * locals.var_sp_s_temp2_dn13)), ((((locals.var_sp_s_eta_dn14 * locals.var_sp_s_eta) + (locals.var_sp_s_eta * locals.var_sp_s_eta_dn14)) * locals.var_sp_s_temp2) + (assign42220_e55377 * locals.var_sp_s_temp2_dn14)), ((((locals.var_sp_s_eta_dn15 * locals.var_sp_s_eta) + (locals.var_sp_s_eta * locals.var_sp_s_eta_dn15)) * locals.var_sp_s_temp2) + (assign42220_e55377 * locals.var_sp_s_temp2_dn15)), ((((locals.var_sp_s_eta_dn16 * locals.var_sp_s_eta) + (locals.var_sp_s_eta * locals.var_sp_s_eta_dn16)) * locals.var_sp_s_temp2) + (assign42220_e55377 * locals.var_sp_s_temp2_dn16)), ((((locals.var_sp_s_eta_dn17 * locals.var_sp_s_eta) + (locals.var_sp_s_eta * locals.var_sp_s_eta_dn17)) * locals.var_sp_s_temp2) + (assign42220_e55377 * locals.var_sp_s_temp2_dn17)), ((((locals.var_sp_s_eta_dn18 * locals.var_sp_s_eta) + (locals.var_sp_s_eta * locals.var_sp_s_eta_dn18)) * locals.var_sp_s_temp2) + (assign42220_e55377 * locals.var_sp_s_temp2_dn18)), ((((locals.var_sp_s_eta_dn19 * locals.var_sp_s_eta) + (locals.var_sp_s_eta * locals.var_sp_s_eta_dn19)) * locals.var_sp_s_temp2) + (assign42220_e55377 * locals.var_sp_s_temp2_dn19)), ((((locals.var_sp_s_eta_dn20 * locals.var_sp_s_eta) + (locals.var_sp_s_eta * locals.var_sp_s_eta_dn20)) * locals.var_sp_s_temp2) + (assign42220_e55377 * locals.var_sp_s_temp2_dn20)),)
    } else {
        (locals.var_sp_s_xi0, locals.var_sp_s_xi0_dn5, locals.var_sp_s_xi0_dn6, locals.var_sp_s_xi0_dn7, locals.var_sp_s_xi0_dn8, locals.var_sp_s_xi0_dn12, locals.var_sp_s_xi0_dn13, locals.var_sp_s_xi0_dn14, locals.var_sp_s_xi0_dn15, locals.var_sp_s_xi0_dn16, locals.var_sp_s_xi0_dn17, locals.var_sp_s_xi0_dn18, locals.var_sp_s_xi0_dn19, locals.var_sp_s_xi0_dn20,)
    }
};
        locals.var_sp_s_xi0 = assign42220_e55381;
        locals.var_sp_s_xi0_dn5 = assign42220_e55381_d_n5;
        locals.var_sp_s_xi0_dn6 = assign42220_e55381_d_n6;
        locals.var_sp_s_xi0_dn7 = assign42220_e55381_d_n7;
        locals.var_sp_s_xi0_dn8 = assign42220_e55381_d_n8;
        locals.var_sp_s_xi0_dn12 = assign42220_e55381_d_n12;
        locals.var_sp_s_xi0_dn13 = assign42220_e55381_d_n13;
        locals.var_sp_s_xi0_dn14 = assign42220_e55381_d_n14;
        locals.var_sp_s_xi0_dn15 = assign42220_e55381_d_n15;
        locals.var_sp_s_xi0_dn16 = assign42220_e55381_d_n16;
        locals.var_sp_s_xi0_dn17 = assign42220_e55381_d_n17;
        locals.var_sp_s_xi0_dn18 = assign42220_e55381_d_n18;
        locals.var_sp_s_xi0_dn19 = assign42220_e55381_d_n19;
        locals.var_sp_s_xi0_dn20 = assign42220_e55381_d_n20;
        locals.var_sp_s_xi0_rv = 0.0;

        let (assign42230_e55395, assign42230_e55395_d_n5, assign42230_e55395_d_n6, assign42230_e55395_d_n7, assign42230_e55395_d_n8, assign42230_e55395_d_n12, assign42230_e55395_d_n13, assign42230_e55395_d_n14, assign42230_e55395_d_n15, assign42230_e55395_d_n16, assign42230_e55395_d_n17, assign42230_e55395_d_n18, assign42230_e55395_d_n19, assign42230_e55395_d_n20,) = {
    if ((locals.var_guard1284 == 0.0) && (locals.var_guard1285 == 0.0)) {
        let assign42230_e55390: f64 = (locals.var_sp_s_eta * locals.var_sp_s_temp2);
        let assign42230_e55392: f64 = (assign42230_e55390 * locals.var_sp_s_temp2);
        let assign42230_e55393: f64 = (4.0 * assign42230_e55392);
        (assign42230_e55393, (4.0 * ((((locals.var_sp_s_eta_dn5 * locals.var_sp_s_temp2) + (locals.var_sp_s_eta * locals.var_sp_s_temp2_dn5)) * locals.var_sp_s_temp2) + (assign42230_e55390 * locals.var_sp_s_temp2_dn5))), (4.0 * ((((locals.var_sp_s_eta_dn6 * locals.var_sp_s_temp2) + (locals.var_sp_s_eta * locals.var_sp_s_temp2_dn6)) * locals.var_sp_s_temp2) + (assign42230_e55390 * locals.var_sp_s_temp2_dn6))), (4.0 * ((((locals.var_sp_s_eta_dn7 * locals.var_sp_s_temp2) + (locals.var_sp_s_eta * locals.var_sp_s_temp2_dn7)) * locals.var_sp_s_temp2) + (assign42230_e55390 * locals.var_sp_s_temp2_dn7))), (4.0 * ((((locals.var_sp_s_eta_dn8 * locals.var_sp_s_temp2) + (locals.var_sp_s_eta * locals.var_sp_s_temp2_dn8)) * locals.var_sp_s_temp2) + (assign42230_e55390 * locals.var_sp_s_temp2_dn8))), (4.0 * ((((locals.var_sp_s_eta_dn12 * locals.var_sp_s_temp2) + (locals.var_sp_s_eta * locals.var_sp_s_temp2_dn12)) * locals.var_sp_s_temp2) + (assign42230_e55390 * locals.var_sp_s_temp2_dn12))), (4.0 * ((((locals.var_sp_s_eta_dn13 * locals.var_sp_s_temp2) + (locals.var_sp_s_eta * locals.var_sp_s_temp2_dn13)) * locals.var_sp_s_temp2) + (assign42230_e55390 * locals.var_sp_s_temp2_dn13))), (4.0 * ((((locals.var_sp_s_eta_dn14 * locals.var_sp_s_temp2) + (locals.var_sp_s_eta * locals.var_sp_s_temp2_dn14)) * locals.var_sp_s_temp2) + (assign42230_e55390 * locals.var_sp_s_temp2_dn14))), (4.0 * ((((locals.var_sp_s_eta_dn15 * locals.var_sp_s_temp2) + (locals.var_sp_s_eta * locals.var_sp_s_temp2_dn15)) * locals.var_sp_s_temp2) + (assign42230_e55390 * locals.var_sp_s_temp2_dn15))), (4.0 * ((((locals.var_sp_s_eta_dn16 * locals.var_sp_s_temp2) + (locals.var_sp_s_eta * locals.var_sp_s_temp2_dn16)) * locals.var_sp_s_temp2) + (assign42230_e55390 * locals.var_sp_s_temp2_dn16))), (4.0 * ((((locals.var_sp_s_eta_dn17 * locals.var_sp_s_temp2) + (locals.var_sp_s_eta * locals.var_sp_s_temp2_dn17)) * locals.var_sp_s_temp2) + (assign42230_e55390 * locals.var_sp_s_temp2_dn17))), (4.0 * ((((locals.var_sp_s_eta_dn18 * locals.var_sp_s_temp2) + (locals.var_sp_s_eta * locals.var_sp_s_temp2_dn18)) * locals.var_sp_s_temp2) + (assign42230_e55390 * locals.var_sp_s_temp2_dn18))), (4.0 * ((((locals.var_sp_s_eta_dn19 * locals.var_sp_s_temp2) + (locals.var_sp_s_eta * locals.var_sp_s_temp2_dn19)) * locals.var_sp_s_temp2) + (assign42230_e55390 * locals.var_sp_s_temp2_dn19))), (4.0 * ((((locals.var_sp_s_eta_dn20 * locals.var_sp_s_temp2) + (locals.var_sp_s_eta * locals.var_sp_s_temp2_dn20)) * locals.var_sp_s_temp2) + (assign42230_e55390 * locals.var_sp_s_temp2_dn20))),)
    } else {
        (locals.var_sp_s_xi1, locals.var_sp_s_xi1_dn5, locals.var_sp_s_xi1_dn6, locals.var_sp_s_xi1_dn7, locals.var_sp_s_xi1_dn8, locals.var_sp_s_xi1_dn12, locals.var_sp_s_xi1_dn13, locals.var_sp_s_xi1_dn14, locals.var_sp_s_xi1_dn15, locals.var_sp_s_xi1_dn16, locals.var_sp_s_xi1_dn17, locals.var_sp_s_xi1_dn18, locals.var_sp_s_xi1_dn19, locals.var_sp_s_xi1_dn20,)
    }
};
        locals.var_sp_s_xi1 = assign42230_e55395;
        locals.var_sp_s_xi1_dn5 = assign42230_e55395_d_n5;
        locals.var_sp_s_xi1_dn6 = assign42230_e55395_d_n6;
        locals.var_sp_s_xi1_dn7 = assign42230_e55395_d_n7;
        locals.var_sp_s_xi1_dn8 = assign42230_e55395_d_n8;
        locals.var_sp_s_xi1_dn12 = assign42230_e55395_d_n12;
        locals.var_sp_s_xi1_dn13 = assign42230_e55395_d_n13;
        locals.var_sp_s_xi1_dn14 = assign42230_e55395_d_n14;
        locals.var_sp_s_xi1_dn15 = assign42230_e55395_d_n15;
        locals.var_sp_s_xi1_dn16 = assign42230_e55395_d_n16;
        locals.var_sp_s_xi1_dn17 = assign42230_e55395_d_n17;
        locals.var_sp_s_xi1_dn18 = assign42230_e55395_d_n18;
        locals.var_sp_s_xi1_dn19 = assign42230_e55395_d_n19;
        locals.var_sp_s_xi1_dn20 = assign42230_e55395_d_n20;
        locals.var_sp_s_xi1_rv = 0.0;

        let (assign42240_e55413, assign42240_e55413_d_n5, assign42240_e55413_d_n6, assign42240_e55413_d_n7, assign42240_e55413_d_n8, assign42240_e55413_d_n12, assign42240_e55413_d_n13, assign42240_e55413_d_n14, assign42240_e55413_d_n15, assign42240_e55413_d_n16, assign42240_e55413_d_n17, assign42240_e55413_d_n18, assign42240_e55413_d_n19, assign42240_e55413_d_n20,) = {
    if ((locals.var_guard1284 == 0.0) && (locals.var_guard1285 == 0.0)) {
        let assign42240_e55403: f64 = (8.0 * locals.var_sp_s_temp2);
        let assign42240_e55406: f64 = (12.0 * locals.var_sp_s_xi0);
        let assign42240_e55407: f64 = (assign42240_e55403 - assign42240_e55406);
        let assign42240_e55409: f64 = (assign42240_e55407 * locals.var_sp_s_temp2);
        let assign42240_e55411: f64 = (assign42240_e55409 * locals.var_sp_s_temp2);
        (assign42240_e55411, ((((((8.0 * locals.var_sp_s_temp2_dn5) - (12.0 * locals.var_sp_s_xi0_dn5)) * locals.var_sp_s_temp2) + (assign42240_e55407 * locals.var_sp_s_temp2_dn5)) * locals.var_sp_s_temp2) + (assign42240_e55409 * locals.var_sp_s_temp2_dn5)), ((((((8.0 * locals.var_sp_s_temp2_dn6) - (12.0 * locals.var_sp_s_xi0_dn6)) * locals.var_sp_s_temp2) + (assign42240_e55407 * locals.var_sp_s_temp2_dn6)) * locals.var_sp_s_temp2) + (assign42240_e55409 * locals.var_sp_s_temp2_dn6)), ((((((8.0 * locals.var_sp_s_temp2_dn7) - (12.0 * locals.var_sp_s_xi0_dn7)) * locals.var_sp_s_temp2) + (assign42240_e55407 * locals.var_sp_s_temp2_dn7)) * locals.var_sp_s_temp2) + (assign42240_e55409 * locals.var_sp_s_temp2_dn7)), ((((((8.0 * locals.var_sp_s_temp2_dn8) - (12.0 * locals.var_sp_s_xi0_dn8)) * locals.var_sp_s_temp2) + (assign42240_e55407 * locals.var_sp_s_temp2_dn8)) * locals.var_sp_s_temp2) + (assign42240_e55409 * locals.var_sp_s_temp2_dn8)), ((((((8.0 * locals.var_sp_s_temp2_dn12) - (12.0 * locals.var_sp_s_xi0_dn12)) * locals.var_sp_s_temp2) + (assign42240_e55407 * locals.var_sp_s_temp2_dn12)) * locals.var_sp_s_temp2) + (assign42240_e55409 * locals.var_sp_s_temp2_dn12)), ((((((8.0 * locals.var_sp_s_temp2_dn13) - (12.0 * locals.var_sp_s_xi0_dn13)) * locals.var_sp_s_temp2) + (assign42240_e55407 * locals.var_sp_s_temp2_dn13)) * locals.var_sp_s_temp2) + (assign42240_e55409 * locals.var_sp_s_temp2_dn13)), ((((((8.0 * locals.var_sp_s_temp2_dn14) - (12.0 * locals.var_sp_s_xi0_dn14)) * locals.var_sp_s_temp2) + (assign42240_e55407 * locals.var_sp_s_temp2_dn14)) * locals.var_sp_s_temp2) + (assign42240_e55409 * locals.var_sp_s_temp2_dn14)), ((((((8.0 * locals.var_sp_s_temp2_dn15) - (12.0 * locals.var_sp_s_xi0_dn15)) * locals.var_sp_s_temp2) + (assign42240_e55407 * locals.var_sp_s_temp2_dn15)) * locals.var_sp_s_temp2) + (assign42240_e55409 * locals.var_sp_s_temp2_dn15)), ((((((8.0 * locals.var_sp_s_temp2_dn16) - (12.0 * locals.var_sp_s_xi0_dn16)) * locals.var_sp_s_temp2) + (assign42240_e55407 * locals.var_sp_s_temp2_dn16)) * locals.var_sp_s_temp2) + (assign42240_e55409 * locals.var_sp_s_temp2_dn16)), ((((((8.0 * locals.var_sp_s_temp2_dn17) - (12.0 * locals.var_sp_s_xi0_dn17)) * locals.var_sp_s_temp2) + (assign42240_e55407 * locals.var_sp_s_temp2_dn17)) * locals.var_sp_s_temp2) + (assign42240_e55409 * locals.var_sp_s_temp2_dn17)), ((((((8.0 * locals.var_sp_s_temp2_dn18) - (12.0 * locals.var_sp_s_xi0_dn18)) * locals.var_sp_s_temp2) + (assign42240_e55407 * locals.var_sp_s_temp2_dn18)) * locals.var_sp_s_temp2) + (assign42240_e55409 * locals.var_sp_s_temp2_dn18)), ((((((8.0 * locals.var_sp_s_temp2_dn19) - (12.0 * locals.var_sp_s_xi0_dn19)) * locals.var_sp_s_temp2) + (assign42240_e55407 * locals.var_sp_s_temp2_dn19)) * locals.var_sp_s_temp2) + (assign42240_e55409 * locals.var_sp_s_temp2_dn19)), ((((((8.0 * locals.var_sp_s_temp2_dn20) - (12.0 * locals.var_sp_s_xi0_dn20)) * locals.var_sp_s_temp2) + (assign42240_e55407 * locals.var_sp_s_temp2_dn20)) * locals.var_sp_s_temp2) + (assign42240_e55409 * locals.var_sp_s_temp2_dn20)),)
    } else {
        (locals.var_sp_s_xi2, locals.var_sp_s_xi2_dn5, locals.var_sp_s_xi2_dn6, locals.var_sp_s_xi2_dn7, locals.var_sp_s_xi2_dn8, locals.var_sp_s_xi2_dn12, locals.var_sp_s_xi2_dn13, locals.var_sp_s_xi2_dn14, locals.var_sp_s_xi2_dn15, locals.var_sp_s_xi2_dn16, locals.var_sp_s_xi2_dn17, locals.var_sp_s_xi2_dn18, locals.var_sp_s_xi2_dn19, locals.var_sp_s_xi2_dn20,)
    }
};
        locals.var_sp_s_xi2 = assign42240_e55413;
        locals.var_sp_s_xi2_dn5 = assign42240_e55413_d_n5;
        locals.var_sp_s_xi2_dn6 = assign42240_e55413_d_n6;
        locals.var_sp_s_xi2_dn7 = assign42240_e55413_d_n7;
        locals.var_sp_s_xi2_dn8 = assign42240_e55413_d_n8;
        locals.var_sp_s_xi2_dn12 = assign42240_e55413_d_n12;
        locals.var_sp_s_xi2_dn13 = assign42240_e55413_d_n13;
        locals.var_sp_s_xi2_dn14 = assign42240_e55413_d_n14;
        locals.var_sp_s_xi2_dn15 = assign42240_e55413_d_n15;
        locals.var_sp_s_xi2_dn16 = assign42240_e55413_d_n16;
        locals.var_sp_s_xi2_dn17 = assign42240_e55413_d_n17;
        locals.var_sp_s_xi2_dn18 = assign42240_e55413_d_n18;
        locals.var_sp_s_xi2_dn19 = assign42240_e55413_d_n19;
        locals.var_sp_s_xi2_dn20 = assign42240_e55413_d_n20;
        locals.var_sp_s_xi2_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_22(
        locals: &mut StampLocals,
    ) {
        let (assign42250_e55462, assign42250_e55462_d_n5, assign42250_e55462_d_n6, assign42250_e55462_d_n7, assign42250_e55462_d_n8, assign42250_e55462_d_n12, assign42250_e55462_d_n13, assign42250_e55462_d_n14, assign42250_e55462_d_n15, assign42250_e55462_d_n16, assign42250_e55462_d_n17, assign42250_e55462_d_n18, assign42250_e55462_d_n19, assign42250_e55462_d_n20,) = {
    if ((locals.var_guard1284 == 0.0) && (locals.var_guard1285 == 0.0)) {
        let assign42250_e55422: f64 = (locals.var_sp_s_temp * locals.var_sp_s_temp);
        let assign42250_e55426: f64 = (locals.var_sp_s_temp1 + locals.var_sp_s_eta);
        let assign42250_e55428: f64 = (assign42250_e55426 - 1.0);
        let assign42250_e55432: f64 = (locals.var_sp_s_eta + 1.0);
        let assign42250_e55434: f64 = (assign42250_e55432 + locals.var_sp_s_xi0);
        let assign42250_e55435: f64 = (locals.var_delta_ns * assign42250_e55434);
        let assign42250_e55436: f64 = (assign42250_e55428 - assign42250_e55435);
        let assign42250_e55437: f64 = (locals.var_gf2 * assign42250_e55436);
        let assign42250_e55438: f64 = (assign42250_e55422 - assign42250_e55437);
        let (assign42250_e55460, assign42250_e55460_d_n5, assign42250_e55460_d_n6, assign42250_e55460_d_n7, assign42250_e55460_d_n8, assign42250_e55460_d_n12, assign42250_e55460_d_n13, assign42250_e55460_d_n14, assign42250_e55460_d_n15, assign42250_e55460_d_n16, assign42250_e55460_d_n17, assign42250_e55460_d_n18, assign42250_e55460_d_n19, assign42250_e55460_d_n20,) = {
            if (1e-40 > assign42250_e55438) {
                (1e-40, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign42250_e55443: f64 = (locals.var_sp_s_temp * locals.var_sp_s_temp);
                let assign42250_e55447: f64 = (locals.var_sp_s_temp1 + locals.var_sp_s_eta);
                let assign42250_e55449: f64 = (assign42250_e55447 - 1.0);
                let assign42250_e55453: f64 = (locals.var_sp_s_eta + 1.0);
                let assign42250_e55455: f64 = (assign42250_e55453 + locals.var_sp_s_xi0);
                let assign42250_e55456: f64 = (locals.var_delta_ns * assign42250_e55455);
                let assign42250_e55457: f64 = (assign42250_e55449 - assign42250_e55456);
                let assign42250_e55458: f64 = (locals.var_gf2 * assign42250_e55457);
                let assign42250_e55459: f64 = (assign42250_e55443 - assign42250_e55458);
                (assign42250_e55459, (((locals.var_sp_s_temp_dn5 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn5)) - ((locals.var_gf2_dn5 * assign42250_e55457) + (locals.var_gf2 * ((locals.var_sp_s_temp1_dn5 + locals.var_sp_s_eta_dn5) - ((locals.var_delta_ns_dn5 * assign42250_e55455) + (locals.var_delta_ns * (locals.var_sp_s_eta_dn5 + locals.var_sp_s_xi0_dn5))))))), (((locals.var_sp_s_temp_dn6 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn6)) - ((locals.var_gf2_dn6 * assign42250_e55457) + (locals.var_gf2 * ((locals.var_sp_s_temp1_dn6 + locals.var_sp_s_eta_dn6) - ((locals.var_delta_ns_dn6 * assign42250_e55455) + (locals.var_delta_ns * (locals.var_sp_s_eta_dn6 + locals.var_sp_s_xi0_dn6))))))), (((locals.var_sp_s_temp_dn7 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn7)) - ((locals.var_gf2_dn7 * assign42250_e55457) + (locals.var_gf2 * ((locals.var_sp_s_temp1_dn7 + locals.var_sp_s_eta_dn7) - ((locals.var_delta_ns_dn7 * assign42250_e55455) + (locals.var_delta_ns * (locals.var_sp_s_eta_dn7 + locals.var_sp_s_xi0_dn7))))))), (((locals.var_sp_s_temp_dn8 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn8)) - ((locals.var_gf2_dn8 * assign42250_e55457) + (locals.var_gf2 * ((locals.var_sp_s_temp1_dn8 + locals.var_sp_s_eta_dn8) - ((locals.var_delta_ns_dn8 * assign42250_e55455) + (locals.var_delta_ns * (locals.var_sp_s_eta_dn8 + locals.var_sp_s_xi0_dn8))))))), (((locals.var_sp_s_temp_dn12 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn12)) - ((locals.var_gf2_dn12 * assign42250_e55457) + (locals.var_gf2 * ((locals.var_sp_s_temp1_dn12 + locals.var_sp_s_eta_dn12) - ((locals.var_delta_ns_dn12 * assign42250_e55455) + (locals.var_delta_ns * (locals.var_sp_s_eta_dn12 + locals.var_sp_s_xi0_dn12))))))), (((locals.var_sp_s_temp_dn13 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn13)) - ((locals.var_gf2_dn13 * assign42250_e55457) + (locals.var_gf2 * ((locals.var_sp_s_temp1_dn13 + locals.var_sp_s_eta_dn13) - ((locals.var_delta_ns_dn13 * assign42250_e55455) + (locals.var_delta_ns * (locals.var_sp_s_eta_dn13 + locals.var_sp_s_xi0_dn13))))))), (((locals.var_sp_s_temp_dn14 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn14)) - ((locals.var_gf2_dn14 * assign42250_e55457) + (locals.var_gf2 * ((locals.var_sp_s_temp1_dn14 + locals.var_sp_s_eta_dn14) - ((locals.var_delta_ns_dn14 * assign42250_e55455) + (locals.var_delta_ns * (locals.var_sp_s_eta_dn14 + locals.var_sp_s_xi0_dn14))))))), (((locals.var_sp_s_temp_dn15 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn15)) - ((locals.var_gf2_dn15 * assign42250_e55457) + (locals.var_gf2 * ((locals.var_sp_s_temp1_dn15 + locals.var_sp_s_eta_dn15) - ((locals.var_delta_ns_dn15 * assign42250_e55455) + (locals.var_delta_ns * (locals.var_sp_s_eta_dn15 + locals.var_sp_s_xi0_dn15))))))), (((locals.var_sp_s_temp_dn16 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn16)) - ((locals.var_gf2_dn16 * assign42250_e55457) + (locals.var_gf2 * ((locals.var_sp_s_temp1_dn16 + locals.var_sp_s_eta_dn16) - ((locals.var_delta_ns_dn16 * assign42250_e55455) + (locals.var_delta_ns * (locals.var_sp_s_eta_dn16 + locals.var_sp_s_xi0_dn16))))))), (((locals.var_sp_s_temp_dn17 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn17)) - ((locals.var_gf2_dn17 * assign42250_e55457) + (locals.var_gf2 * ((locals.var_sp_s_temp1_dn17 + locals.var_sp_s_eta_dn17) - ((locals.var_delta_ns_dn17 * assign42250_e55455) + (locals.var_delta_ns * (locals.var_sp_s_eta_dn17 + locals.var_sp_s_xi0_dn17))))))), (((locals.var_sp_s_temp_dn18 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn18)) - ((locals.var_gf2_dn18 * assign42250_e55457) + (locals.var_gf2 * ((locals.var_sp_s_temp1_dn18 + locals.var_sp_s_eta_dn18) - ((locals.var_delta_ns_dn18 * assign42250_e55455) + (locals.var_delta_ns * (locals.var_sp_s_eta_dn18 + locals.var_sp_s_xi0_dn18))))))), (((locals.var_sp_s_temp_dn19 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn19)) - ((locals.var_gf2_dn19 * assign42250_e55457) + (locals.var_gf2 * ((locals.var_sp_s_temp1_dn19 + locals.var_sp_s_eta_dn19) - ((locals.var_delta_ns_dn19 * assign42250_e55455) + (locals.var_delta_ns * (locals.var_sp_s_eta_dn19 + locals.var_sp_s_xi0_dn19))))))), (((locals.var_sp_s_temp_dn20 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn20)) - ((locals.var_gf2_dn20 * assign42250_e55457) + (locals.var_gf2 * ((locals.var_sp_s_temp1_dn20 + locals.var_sp_s_eta_dn20) - ((locals.var_delta_ns_dn20 * assign42250_e55455) + (locals.var_delta_ns * (locals.var_sp_s_eta_dn20 + locals.var_sp_s_xi0_dn20))))))),)
            }
        };
        (assign42250_e55460, assign42250_e55460_d_n5, assign42250_e55460_d_n6, assign42250_e55460_d_n7, assign42250_e55460_d_n8, assign42250_e55460_d_n12, assign42250_e55460_d_n13, assign42250_e55460_d_n14, assign42250_e55460_d_n15, assign42250_e55460_d_n16, assign42250_e55460_d_n17, assign42250_e55460_d_n18, assign42250_e55460_d_n19, assign42250_e55460_d_n20,)
    } else {
        (locals.var_sp_s_a, locals.var_sp_s_a_dn5, locals.var_sp_s_a_dn6, locals.var_sp_s_a_dn7, locals.var_sp_s_a_dn8, locals.var_sp_s_a_dn12, locals.var_sp_s_a_dn13, locals.var_sp_s_a_dn14, locals.var_sp_s_a_dn15, locals.var_sp_s_a_dn16, locals.var_sp_s_a_dn17, locals.var_sp_s_a_dn18, locals.var_sp_s_a_dn19, locals.var_sp_s_a_dn20,)
    }
};
        locals.var_sp_s_a = assign42250_e55462;
        locals.var_sp_s_a_dn5 = assign42250_e55462_d_n5;
        locals.var_sp_s_a_dn6 = assign42250_e55462_d_n6;
        locals.var_sp_s_a_dn7 = assign42250_e55462_d_n7;
        locals.var_sp_s_a_dn8 = assign42250_e55462_d_n8;
        locals.var_sp_s_a_dn12 = assign42250_e55462_d_n12;
        locals.var_sp_s_a_dn13 = assign42250_e55462_d_n13;
        locals.var_sp_s_a_dn14 = assign42250_e55462_d_n14;
        locals.var_sp_s_a_dn15 = assign42250_e55462_d_n15;
        locals.var_sp_s_a_dn16 = assign42250_e55462_d_n16;
        locals.var_sp_s_a_dn17 = assign42250_e55462_d_n17;
        locals.var_sp_s_a_dn18 = assign42250_e55462_d_n18;
        locals.var_sp_s_a_dn19 = assign42250_e55462_d_n19;
        locals.var_sp_s_a_dn20 = assign42250_e55462_d_n20;
        locals.var_sp_s_a_rv = 0.0;

        let (assign42260_e55480, assign42260_e55480_d_n5, assign42260_e55480_d_n6, assign42260_e55480_d_n7, assign42260_e55480_d_n8, assign42260_e55480_d_n12, assign42260_e55480_d_n13, assign42260_e55480_d_n14, assign42260_e55480_d_n15, assign42260_e55480_d_n16, assign42260_e55480_d_n17, assign42260_e55480_d_n18, assign42260_e55480_d_n19, assign42260_e55480_d_n20,) = {
    if ((locals.var_guard1284 == 0.0) && (locals.var_guard1285 == 0.0)) {
        let assign42260_e55474: f64 = (locals.var_delta_ns * locals.var_sp_s_xi2);
        let assign42260_e55475: f64 = (locals.var_sp_s_temp1 - assign42260_e55474);
        let assign42260_e55476: f64 = (locals.var_gf2 * assign42260_e55475);
        let assign42260_e55477: f64 = (0.5 * assign42260_e55476);
        let assign42260_e55478: f64 = (1.0 - assign42260_e55477);
        (assign42260_e55478, (-(0.5 * ((locals.var_gf2_dn5 * assign42260_e55475) + (locals.var_gf2 * (locals.var_sp_s_temp1_dn5 - ((locals.var_delta_ns_dn5 * locals.var_sp_s_xi2) + (locals.var_delta_ns * locals.var_sp_s_xi2_dn5))))))), (-(0.5 * ((locals.var_gf2_dn6 * assign42260_e55475) + (locals.var_gf2 * (locals.var_sp_s_temp1_dn6 - ((locals.var_delta_ns_dn6 * locals.var_sp_s_xi2) + (locals.var_delta_ns * locals.var_sp_s_xi2_dn6))))))), (-(0.5 * ((locals.var_gf2_dn7 * assign42260_e55475) + (locals.var_gf2 * (locals.var_sp_s_temp1_dn7 - ((locals.var_delta_ns_dn7 * locals.var_sp_s_xi2) + (locals.var_delta_ns * locals.var_sp_s_xi2_dn7))))))), (-(0.5 * ((locals.var_gf2_dn8 * assign42260_e55475) + (locals.var_gf2 * (locals.var_sp_s_temp1_dn8 - ((locals.var_delta_ns_dn8 * locals.var_sp_s_xi2) + (locals.var_delta_ns * locals.var_sp_s_xi2_dn8))))))), (-(0.5 * ((locals.var_gf2_dn12 * assign42260_e55475) + (locals.var_gf2 * (locals.var_sp_s_temp1_dn12 - ((locals.var_delta_ns_dn12 * locals.var_sp_s_xi2) + (locals.var_delta_ns * locals.var_sp_s_xi2_dn12))))))), (-(0.5 * ((locals.var_gf2_dn13 * assign42260_e55475) + (locals.var_gf2 * (locals.var_sp_s_temp1_dn13 - ((locals.var_delta_ns_dn13 * locals.var_sp_s_xi2) + (locals.var_delta_ns * locals.var_sp_s_xi2_dn13))))))), (-(0.5 * ((locals.var_gf2_dn14 * assign42260_e55475) + (locals.var_gf2 * (locals.var_sp_s_temp1_dn14 - ((locals.var_delta_ns_dn14 * locals.var_sp_s_xi2) + (locals.var_delta_ns * locals.var_sp_s_xi2_dn14))))))), (-(0.5 * ((locals.var_gf2_dn15 * assign42260_e55475) + (locals.var_gf2 * (locals.var_sp_s_temp1_dn15 - ((locals.var_delta_ns_dn15 * locals.var_sp_s_xi2) + (locals.var_delta_ns * locals.var_sp_s_xi2_dn15))))))), (-(0.5 * ((locals.var_gf2_dn16 * assign42260_e55475) + (locals.var_gf2 * (locals.var_sp_s_temp1_dn16 - ((locals.var_delta_ns_dn16 * locals.var_sp_s_xi2) + (locals.var_delta_ns * locals.var_sp_s_xi2_dn16))))))), (-(0.5 * ((locals.var_gf2_dn17 * assign42260_e55475) + (locals.var_gf2 * (locals.var_sp_s_temp1_dn17 - ((locals.var_delta_ns_dn17 * locals.var_sp_s_xi2) + (locals.var_delta_ns * locals.var_sp_s_xi2_dn17))))))), (-(0.5 * ((locals.var_gf2_dn18 * assign42260_e55475) + (locals.var_gf2 * (locals.var_sp_s_temp1_dn18 - ((locals.var_delta_ns_dn18 * locals.var_sp_s_xi2) + (locals.var_delta_ns * locals.var_sp_s_xi2_dn18))))))), (-(0.5 * ((locals.var_gf2_dn19 * assign42260_e55475) + (locals.var_gf2 * (locals.var_sp_s_temp1_dn19 - ((locals.var_delta_ns_dn19 * locals.var_sp_s_xi2) + (locals.var_delta_ns * locals.var_sp_s_xi2_dn19))))))), (-(0.5 * ((locals.var_gf2_dn20 * assign42260_e55475) + (locals.var_gf2 * (locals.var_sp_s_temp1_dn20 - ((locals.var_delta_ns_dn20 * locals.var_sp_s_xi2) + (locals.var_delta_ns * locals.var_sp_s_xi2_dn20))))))),)
    } else {
        (locals.var_sp_s_b, locals.var_sp_s_b_dn5, locals.var_sp_s_b_dn6, locals.var_sp_s_b_dn7, locals.var_sp_s_b_dn8, locals.var_sp_s_b_dn12, locals.var_sp_s_b_dn13, locals.var_sp_s_b_dn14, locals.var_sp_s_b_dn15, locals.var_sp_s_b_dn16, locals.var_sp_s_b_dn17, locals.var_sp_s_b_dn18, locals.var_sp_s_b_dn19, locals.var_sp_s_b_dn20,)
    }
};
        locals.var_sp_s_b = assign42260_e55480;
        locals.var_sp_s_b_dn5 = assign42260_e55480_d_n5;
        locals.var_sp_s_b_dn6 = assign42260_e55480_d_n6;
        locals.var_sp_s_b_dn7 = assign42260_e55480_d_n7;
        locals.var_sp_s_b_dn8 = assign42260_e55480_d_n8;
        locals.var_sp_s_b_dn12 = assign42260_e55480_d_n12;
        locals.var_sp_s_b_dn13 = assign42260_e55480_d_n13;
        locals.var_sp_s_b_dn14 = assign42260_e55480_d_n14;
        locals.var_sp_s_b_dn15 = assign42260_e55480_d_n15;
        locals.var_sp_s_b_dn16 = assign42260_e55480_d_n16;
        locals.var_sp_s_b_dn17 = assign42260_e55480_d_n17;
        locals.var_sp_s_b_dn18 = assign42260_e55480_d_n18;
        locals.var_sp_s_b_dn19 = assign42260_e55480_d_n19;
        locals.var_sp_s_b_dn20 = assign42260_e55480_d_n20;
        locals.var_sp_s_b_rv = 0.0;

        let (assign42270_e55502, assign42270_e55502_d_n5, assign42270_e55502_d_n6, assign42270_e55502_d_n7, assign42270_e55502_d_n8, assign42270_e55502_d_n12, assign42270_e55502_d_n13, assign42270_e55502_d_n14, assign42270_e55502_d_n15, assign42270_e55502_d_n16, assign42270_e55502_d_n17, assign42270_e55502_d_n18, assign42270_e55502_d_n19, assign42270_e55502_d_n20,) = {
    if ((locals.var_guard1284 == 0.0) && (locals.var_guard1285 == 0.0)) {
        let assign42270_e55488: f64 = (2.0 * locals.var_sp_s_temp);
        let assign42270_e55492: f64 = (1.0 - locals.var_sp_s_temp1);
        let assign42270_e55496: f64 = (1.0 + locals.var_sp_s_xi1);
        let assign42270_e55497: f64 = (locals.var_delta_ns * assign42270_e55496);
        let assign42270_e55498: f64 = (assign42270_e55492 - assign42270_e55497);
        let assign42270_e55499: f64 = (locals.var_gf2 * assign42270_e55498);
        let assign42270_e55500: f64 = (assign42270_e55488 + assign42270_e55499);
        (assign42270_e55500, ((2.0 * locals.var_sp_s_temp_dn5) + ((locals.var_gf2_dn5 * assign42270_e55498) + (locals.var_gf2 * ((-locals.var_sp_s_temp1_dn5) - ((locals.var_delta_ns_dn5 * assign42270_e55496) + (locals.var_delta_ns * locals.var_sp_s_xi1_dn5)))))), ((2.0 * locals.var_sp_s_temp_dn6) + ((locals.var_gf2_dn6 * assign42270_e55498) + (locals.var_gf2 * ((-locals.var_sp_s_temp1_dn6) - ((locals.var_delta_ns_dn6 * assign42270_e55496) + (locals.var_delta_ns * locals.var_sp_s_xi1_dn6)))))), ((2.0 * locals.var_sp_s_temp_dn7) + ((locals.var_gf2_dn7 * assign42270_e55498) + (locals.var_gf2 * ((-locals.var_sp_s_temp1_dn7) - ((locals.var_delta_ns_dn7 * assign42270_e55496) + (locals.var_delta_ns * locals.var_sp_s_xi1_dn7)))))), ((2.0 * locals.var_sp_s_temp_dn8) + ((locals.var_gf2_dn8 * assign42270_e55498) + (locals.var_gf2 * ((-locals.var_sp_s_temp1_dn8) - ((locals.var_delta_ns_dn8 * assign42270_e55496) + (locals.var_delta_ns * locals.var_sp_s_xi1_dn8)))))), ((2.0 * locals.var_sp_s_temp_dn12) + ((locals.var_gf2_dn12 * assign42270_e55498) + (locals.var_gf2 * ((-locals.var_sp_s_temp1_dn12) - ((locals.var_delta_ns_dn12 * assign42270_e55496) + (locals.var_delta_ns * locals.var_sp_s_xi1_dn12)))))), ((2.0 * locals.var_sp_s_temp_dn13) + ((locals.var_gf2_dn13 * assign42270_e55498) + (locals.var_gf2 * ((-locals.var_sp_s_temp1_dn13) - ((locals.var_delta_ns_dn13 * assign42270_e55496) + (locals.var_delta_ns * locals.var_sp_s_xi1_dn13)))))), ((2.0 * locals.var_sp_s_temp_dn14) + ((locals.var_gf2_dn14 * assign42270_e55498) + (locals.var_gf2 * ((-locals.var_sp_s_temp1_dn14) - ((locals.var_delta_ns_dn14 * assign42270_e55496) + (locals.var_delta_ns * locals.var_sp_s_xi1_dn14)))))), ((2.0 * locals.var_sp_s_temp_dn15) + ((locals.var_gf2_dn15 * assign42270_e55498) + (locals.var_gf2 * ((-locals.var_sp_s_temp1_dn15) - ((locals.var_delta_ns_dn15 * assign42270_e55496) + (locals.var_delta_ns * locals.var_sp_s_xi1_dn15)))))), ((2.0 * locals.var_sp_s_temp_dn16) + ((locals.var_gf2_dn16 * assign42270_e55498) + (locals.var_gf2 * ((-locals.var_sp_s_temp1_dn16) - ((locals.var_delta_ns_dn16 * assign42270_e55496) + (locals.var_delta_ns * locals.var_sp_s_xi1_dn16)))))), ((2.0 * locals.var_sp_s_temp_dn17) + ((locals.var_gf2_dn17 * assign42270_e55498) + (locals.var_gf2 * ((-locals.var_sp_s_temp1_dn17) - ((locals.var_delta_ns_dn17 * assign42270_e55496) + (locals.var_delta_ns * locals.var_sp_s_xi1_dn17)))))), ((2.0 * locals.var_sp_s_temp_dn18) + ((locals.var_gf2_dn18 * assign42270_e55498) + (locals.var_gf2 * ((-locals.var_sp_s_temp1_dn18) - ((locals.var_delta_ns_dn18 * assign42270_e55496) + (locals.var_delta_ns * locals.var_sp_s_xi1_dn18)))))), ((2.0 * locals.var_sp_s_temp_dn19) + ((locals.var_gf2_dn19 * assign42270_e55498) + (locals.var_gf2 * ((-locals.var_sp_s_temp1_dn19) - ((locals.var_delta_ns_dn19 * assign42270_e55496) + (locals.var_delta_ns * locals.var_sp_s_xi1_dn19)))))), ((2.0 * locals.var_sp_s_temp_dn20) + ((locals.var_gf2_dn20 * assign42270_e55498) + (locals.var_gf2 * ((-locals.var_sp_s_temp1_dn20) - ((locals.var_delta_ns_dn20 * assign42270_e55496) + (locals.var_delta_ns * locals.var_sp_s_xi1_dn20)))))),)
    } else {
        (locals.var_sp_s_c, locals.var_sp_s_c_dn5, locals.var_sp_s_c_dn6, locals.var_sp_s_c_dn7, locals.var_sp_s_c_dn8, locals.var_sp_s_c_dn12, locals.var_sp_s_c_dn13, locals.var_sp_s_c_dn14, locals.var_sp_s_c_dn15, locals.var_sp_s_c_dn16, locals.var_sp_s_c_dn17, locals.var_sp_s_c_dn18, locals.var_sp_s_c_dn19, locals.var_sp_s_c_dn20,)
    }
};
        locals.var_sp_s_c = assign42270_e55502;
        locals.var_sp_s_c_dn5 = assign42270_e55502_d_n5;
        locals.var_sp_s_c_dn6 = assign42270_e55502_d_n6;
        locals.var_sp_s_c_dn7 = assign42270_e55502_d_n7;
        locals.var_sp_s_c_dn8 = assign42270_e55502_d_n8;
        locals.var_sp_s_c_dn12 = assign42270_e55502_d_n12;
        locals.var_sp_s_c_dn13 = assign42270_e55502_d_n13;
        locals.var_sp_s_c_dn14 = assign42270_e55502_d_n14;
        locals.var_sp_s_c_dn15 = assign42270_e55502_d_n15;
        locals.var_sp_s_c_dn16 = assign42270_e55502_d_n16;
        locals.var_sp_s_c_dn17 = assign42270_e55502_d_n17;
        locals.var_sp_s_c_dn18 = assign42270_e55502_d_n18;
        locals.var_sp_s_c_dn19 = assign42270_e55502_d_n19;
        locals.var_sp_s_c_dn20 = assign42270_e55502_d_n20;
        locals.var_sp_s_c_rv = 0.0;

        let (assign42280_e55517, assign42280_e55517_d_n5, assign42280_e55517_d_n6, assign42280_e55517_d_n7, assign42280_e55517_d_n8, assign42280_e55517_d_n12, assign42280_e55517_d_n13, assign42280_e55517_d_n14, assign42280_e55517_d_n15, assign42280_e55517_d_n16, assign42280_e55517_d_n17, assign42280_e55517_d_n18, assign42280_e55517_d_n19, assign42280_e55517_d_n20,) = {
    if ((locals.var_guard1284 == 0.0) && (locals.var_guard1285 == 0.0)) {
        let assign42280_e55510: f64 = (locals.var_xn_s - locals.var_sp_s_eta);
        let assign42280_e55513: f64 = (locals.var_sp_s_a / locals.var_gf2);
        let assign42280_e55514: f64 = (assign42280_e55513).ln();
        let assign42280_e55515: f64 = (assign42280_e55510 + assign42280_e55514);
        (assign42280_e55515, ((locals.var_xn_s_dn5 - locals.var_sp_s_eta_dn5) + ((((locals.var_sp_s_a_dn5 * locals.var_gf2) - (locals.var_sp_s_a * locals.var_gf2_dn5)) / (locals.var_gf2 * locals.var_gf2)) / assign42280_e55513)), ((locals.var_xn_s_dn6 - locals.var_sp_s_eta_dn6) + ((((locals.var_sp_s_a_dn6 * locals.var_gf2) - (locals.var_sp_s_a * locals.var_gf2_dn6)) / (locals.var_gf2 * locals.var_gf2)) / assign42280_e55513)), ((locals.var_xn_s_dn7 - locals.var_sp_s_eta_dn7) + ((((locals.var_sp_s_a_dn7 * locals.var_gf2) - (locals.var_sp_s_a * locals.var_gf2_dn7)) / (locals.var_gf2 * locals.var_gf2)) / assign42280_e55513)), ((locals.var_xn_s_dn8 - locals.var_sp_s_eta_dn8) + ((((locals.var_sp_s_a_dn8 * locals.var_gf2) - (locals.var_sp_s_a * locals.var_gf2_dn8)) / (locals.var_gf2 * locals.var_gf2)) / assign42280_e55513)), ((locals.var_xn_s_dn12 - locals.var_sp_s_eta_dn12) + ((((locals.var_sp_s_a_dn12 * locals.var_gf2) - (locals.var_sp_s_a * locals.var_gf2_dn12)) / (locals.var_gf2 * locals.var_gf2)) / assign42280_e55513)), ((locals.var_xn_s_dn13 - locals.var_sp_s_eta_dn13) + ((((locals.var_sp_s_a_dn13 * locals.var_gf2) - (locals.var_sp_s_a * locals.var_gf2_dn13)) / (locals.var_gf2 * locals.var_gf2)) / assign42280_e55513)), ((locals.var_xn_s_dn14 - locals.var_sp_s_eta_dn14) + ((((locals.var_sp_s_a_dn14 * locals.var_gf2) - (locals.var_sp_s_a * locals.var_gf2_dn14)) / (locals.var_gf2 * locals.var_gf2)) / assign42280_e55513)), ((locals.var_xn_s_dn15 - locals.var_sp_s_eta_dn15) + ((((locals.var_sp_s_a_dn15 * locals.var_gf2) - (locals.var_sp_s_a * locals.var_gf2_dn15)) / (locals.var_gf2 * locals.var_gf2)) / assign42280_e55513)), ((locals.var_xn_s_dn16 - locals.var_sp_s_eta_dn16) + ((((locals.var_sp_s_a_dn16 * locals.var_gf2) - (locals.var_sp_s_a * locals.var_gf2_dn16)) / (locals.var_gf2 * locals.var_gf2)) / assign42280_e55513)), ((locals.var_xn_s_dn17 - locals.var_sp_s_eta_dn17) + ((((locals.var_sp_s_a_dn17 * locals.var_gf2) - (locals.var_sp_s_a * locals.var_gf2_dn17)) / (locals.var_gf2 * locals.var_gf2)) / assign42280_e55513)), ((locals.var_xn_s_dn18 - locals.var_sp_s_eta_dn18) + ((((locals.var_sp_s_a_dn18 * locals.var_gf2) - (locals.var_sp_s_a * locals.var_gf2_dn18)) / (locals.var_gf2 * locals.var_gf2)) / assign42280_e55513)), ((locals.var_xn_s_dn19 - locals.var_sp_s_eta_dn19) + ((((locals.var_sp_s_a_dn19 * locals.var_gf2) - (locals.var_sp_s_a * locals.var_gf2_dn19)) / (locals.var_gf2 * locals.var_gf2)) / assign42280_e55513)), ((locals.var_xn_s_dn20 - locals.var_sp_s_eta_dn20) + ((((locals.var_sp_s_a_dn20 * locals.var_gf2) - (locals.var_sp_s_a * locals.var_gf2_dn20)) / (locals.var_gf2 * locals.var_gf2)) / assign42280_e55513)),)
    } else {
        (locals.var_sp_s_tau, locals.var_sp_s_tau_dn5, locals.var_sp_s_tau_dn6, locals.var_sp_s_tau_dn7, locals.var_sp_s_tau_dn8, locals.var_sp_s_tau_dn12, locals.var_sp_s_tau_dn13, locals.var_sp_s_tau_dn14, locals.var_sp_s_tau_dn15, locals.var_sp_s_tau_dn16, locals.var_sp_s_tau_dn17, locals.var_sp_s_tau_dn18, locals.var_sp_s_tau_dn19, locals.var_sp_s_tau_dn20,)
    }
};
        locals.var_sp_s_tau = assign42280_e55517;
        locals.var_sp_s_tau_dn5 = assign42280_e55517_d_n5;
        locals.var_sp_s_tau_dn6 = assign42280_e55517_d_n6;
        locals.var_sp_s_tau_dn7 = assign42280_e55517_d_n7;
        locals.var_sp_s_tau_dn8 = assign42280_e55517_d_n8;
        locals.var_sp_s_tau_dn12 = assign42280_e55517_d_n12;
        locals.var_sp_s_tau_dn13 = assign42280_e55517_d_n13;
        locals.var_sp_s_tau_dn14 = assign42280_e55517_d_n14;
        locals.var_sp_s_tau_dn15 = assign42280_e55517_d_n15;
        locals.var_sp_s_tau_dn16 = assign42280_e55517_d_n16;
        locals.var_sp_s_tau_dn17 = assign42280_e55517_d_n17;
        locals.var_sp_s_tau_dn18 = assign42280_e55517_d_n18;
        locals.var_sp_s_tau_dn19 = assign42280_e55517_d_n19;
        locals.var_sp_s_tau_dn20 = assign42280_e55517_d_n20;
        locals.var_sp_s_tau_rv = 0.0;

        let (assign42290_e55527, assign42290_e55527_d_n5, assign42290_e55527_d_n6, assign42290_e55527_d_n7, assign42290_e55527_d_n8, assign42290_e55527_d_n12, assign42290_e55527_d_n13, assign42290_e55527_d_n14, assign42290_e55527_d_n15, assign42290_e55527_d_n16, assign42290_e55527_d_n17, assign42290_e55527_d_n18, assign42290_e55527_d_n19, assign42290_e55527_d_n20,) = {
    if ((locals.var_guard1284 == 0.0) && (locals.var_guard1285 == 0.0)) {
        let assign42290_e55525: f64 = (locals.var_sp_s_a + locals.var_sp_s_c);
        (assign42290_e55525, (locals.var_sp_s_a_dn5 + locals.var_sp_s_c_dn5), (locals.var_sp_s_a_dn6 + locals.var_sp_s_c_dn6), (locals.var_sp_s_a_dn7 + locals.var_sp_s_c_dn7), (locals.var_sp_s_a_dn8 + locals.var_sp_s_c_dn8), (locals.var_sp_s_a_dn12 + locals.var_sp_s_c_dn12), (locals.var_sp_s_a_dn13 + locals.var_sp_s_c_dn13), (locals.var_sp_s_a_dn14 + locals.var_sp_s_c_dn14), (locals.var_sp_s_a_dn15 + locals.var_sp_s_c_dn15), (locals.var_sp_s_a_dn16 + locals.var_sp_s_c_dn16), (locals.var_sp_s_a_dn17 + locals.var_sp_s_c_dn17), (locals.var_sp_s_a_dn18 + locals.var_sp_s_c_dn18), (locals.var_sp_s_a_dn19 + locals.var_sp_s_c_dn19), (locals.var_sp_s_a_dn20 + locals.var_sp_s_c_dn20),)
    } else {
        (locals.var_nu, locals.var_nu_dn5, locals.var_nu_dn6, locals.var_nu_dn7, locals.var_nu_dn8, locals.var_nu_dn12, locals.var_nu_dn13, locals.var_nu_dn14, locals.var_nu_dn15, locals.var_nu_dn16, locals.var_nu_dn17, locals.var_nu_dn18, locals.var_nu_dn19, locals.var_nu_dn20,)
    }
};
        locals.var_nu = assign42290_e55527;
        locals.var_nu_dn5 = assign42290_e55527_d_n5;
        locals.var_nu_dn6 = assign42290_e55527_d_n6;
        locals.var_nu_dn7 = assign42290_e55527_d_n7;
        locals.var_nu_dn8 = assign42290_e55527_d_n8;
        locals.var_nu_dn12 = assign42290_e55527_d_n12;
        locals.var_nu_dn13 = assign42290_e55527_d_n13;
        locals.var_nu_dn14 = assign42290_e55527_d_n14;
        locals.var_nu_dn15 = assign42290_e55527_d_n15;
        locals.var_nu_dn16 = assign42290_e55527_d_n16;
        locals.var_nu_dn17 = assign42290_e55527_d_n17;
        locals.var_nu_dn18 = assign42290_e55527_d_n18;
        locals.var_nu_dn19 = assign42290_e55527_d_n19;
        locals.var_nu_dn20 = assign42290_e55527_d_n20;
        locals.var_nu_rv = 0.0;

        let (assign42300_e55549, assign42300_e55549_d_n5, assign42300_e55549_d_n6, assign42300_e55549_d_n7, assign42300_e55549_d_n8, assign42300_e55549_d_n12, assign42300_e55549_d_n13, assign42300_e55549_d_n14, assign42300_e55549_d_n15, assign42300_e55549_d_n16, assign42300_e55549_d_n17, assign42300_e55549_d_n18, assign42300_e55549_d_n19, assign42300_e55549_d_n20,) = {
    if ((locals.var_guard1284 == 0.0) && (locals.var_guard1285 == 0.0)) {
        let assign42300_e55535: f64 = (locals.var_nu * locals.var_nu);
        let assign42300_e55540: f64 = (locals.var_sp_s_c * locals.var_sp_s_c);
        let assign42300_e55541: f64 = (0.5 * assign42300_e55540);
        let assign42300_e55544: f64 = (locals.var_sp_s_a * locals.var_sp_s_b);
        let assign42300_e55545: f64 = (assign42300_e55541 - assign42300_e55544);
        let assign42300_e55546: f64 = (locals.var_sp_s_tau * assign42300_e55545);
        let assign42300_e55547: f64 = (assign42300_e55535 + assign42300_e55546);
        (assign42300_e55547, (((locals.var_nu_dn5 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn5)) + ((locals.var_sp_s_tau_dn5 * assign42300_e55545) + (locals.var_sp_s_tau * ((0.5 * ((locals.var_sp_s_c_dn5 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn5))) - ((locals.var_sp_s_a_dn5 * locals.var_sp_s_b) + (locals.var_sp_s_a * locals.var_sp_s_b_dn5)))))), (((locals.var_nu_dn6 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn6)) + ((locals.var_sp_s_tau_dn6 * assign42300_e55545) + (locals.var_sp_s_tau * ((0.5 * ((locals.var_sp_s_c_dn6 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn6))) - ((locals.var_sp_s_a_dn6 * locals.var_sp_s_b) + (locals.var_sp_s_a * locals.var_sp_s_b_dn6)))))), (((locals.var_nu_dn7 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn7)) + ((locals.var_sp_s_tau_dn7 * assign42300_e55545) + (locals.var_sp_s_tau * ((0.5 * ((locals.var_sp_s_c_dn7 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn7))) - ((locals.var_sp_s_a_dn7 * locals.var_sp_s_b) + (locals.var_sp_s_a * locals.var_sp_s_b_dn7)))))), (((locals.var_nu_dn8 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn8)) + ((locals.var_sp_s_tau_dn8 * assign42300_e55545) + (locals.var_sp_s_tau * ((0.5 * ((locals.var_sp_s_c_dn8 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn8))) - ((locals.var_sp_s_a_dn8 * locals.var_sp_s_b) + (locals.var_sp_s_a * locals.var_sp_s_b_dn8)))))), (((locals.var_nu_dn12 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn12)) + ((locals.var_sp_s_tau_dn12 * assign42300_e55545) + (locals.var_sp_s_tau * ((0.5 * ((locals.var_sp_s_c_dn12 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn12))) - ((locals.var_sp_s_a_dn12 * locals.var_sp_s_b) + (locals.var_sp_s_a * locals.var_sp_s_b_dn12)))))), (((locals.var_nu_dn13 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn13)) + ((locals.var_sp_s_tau_dn13 * assign42300_e55545) + (locals.var_sp_s_tau * ((0.5 * ((locals.var_sp_s_c_dn13 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn13))) - ((locals.var_sp_s_a_dn13 * locals.var_sp_s_b) + (locals.var_sp_s_a * locals.var_sp_s_b_dn13)))))), (((locals.var_nu_dn14 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn14)) + ((locals.var_sp_s_tau_dn14 * assign42300_e55545) + (locals.var_sp_s_tau * ((0.5 * ((locals.var_sp_s_c_dn14 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn14))) - ((locals.var_sp_s_a_dn14 * locals.var_sp_s_b) + (locals.var_sp_s_a * locals.var_sp_s_b_dn14)))))), (((locals.var_nu_dn15 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn15)) + ((locals.var_sp_s_tau_dn15 * assign42300_e55545) + (locals.var_sp_s_tau * ((0.5 * ((locals.var_sp_s_c_dn15 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn15))) - ((locals.var_sp_s_a_dn15 * locals.var_sp_s_b) + (locals.var_sp_s_a * locals.var_sp_s_b_dn15)))))), (((locals.var_nu_dn16 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn16)) + ((locals.var_sp_s_tau_dn16 * assign42300_e55545) + (locals.var_sp_s_tau * ((0.5 * ((locals.var_sp_s_c_dn16 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn16))) - ((locals.var_sp_s_a_dn16 * locals.var_sp_s_b) + (locals.var_sp_s_a * locals.var_sp_s_b_dn16)))))), (((locals.var_nu_dn17 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn17)) + ((locals.var_sp_s_tau_dn17 * assign42300_e55545) + (locals.var_sp_s_tau * ((0.5 * ((locals.var_sp_s_c_dn17 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn17))) - ((locals.var_sp_s_a_dn17 * locals.var_sp_s_b) + (locals.var_sp_s_a * locals.var_sp_s_b_dn17)))))), (((locals.var_nu_dn18 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn18)) + ((locals.var_sp_s_tau_dn18 * assign42300_e55545) + (locals.var_sp_s_tau * ((0.5 * ((locals.var_sp_s_c_dn18 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn18))) - ((locals.var_sp_s_a_dn18 * locals.var_sp_s_b) + (locals.var_sp_s_a * locals.var_sp_s_b_dn18)))))), (((locals.var_nu_dn19 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn19)) + ((locals.var_sp_s_tau_dn19 * assign42300_e55545) + (locals.var_sp_s_tau * ((0.5 * ((locals.var_sp_s_c_dn19 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn19))) - ((locals.var_sp_s_a_dn19 * locals.var_sp_s_b) + (locals.var_sp_s_a * locals.var_sp_s_b_dn19)))))), (((locals.var_nu_dn20 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn20)) + ((locals.var_sp_s_tau_dn20 * assign42300_e55545) + (locals.var_sp_s_tau * ((0.5 * ((locals.var_sp_s_c_dn20 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn20))) - ((locals.var_sp_s_a_dn20 * locals.var_sp_s_b) + (locals.var_sp_s_a * locals.var_sp_s_b_dn20)))))),)
    } else {
        (locals.var_mutau, locals.var_mutau_dn5, locals.var_mutau_dn6, locals.var_mutau_dn7, locals.var_mutau_dn8, locals.var_mutau_dn12, locals.var_mutau_dn13, locals.var_mutau_dn14, locals.var_mutau_dn15, locals.var_mutau_dn16, locals.var_mutau_dn17, locals.var_mutau_dn18, locals.var_mutau_dn19, locals.var_mutau_dn20,)
    }
};
        locals.var_mutau = assign42300_e55549;
        locals.var_mutau_dn5 = assign42300_e55549_d_n5;
        locals.var_mutau_dn6 = assign42300_e55549_d_n6;
        locals.var_mutau_dn7 = assign42300_e55549_d_n7;
        locals.var_mutau_dn8 = assign42300_e55549_d_n8;
        locals.var_mutau_dn12 = assign42300_e55549_d_n12;
        locals.var_mutau_dn13 = assign42300_e55549_d_n13;
        locals.var_mutau_dn14 = assign42300_e55549_d_n14;
        locals.var_mutau_dn15 = assign42300_e55549_d_n15;
        locals.var_mutau_dn16 = assign42300_e55549_d_n16;
        locals.var_mutau_dn17 = assign42300_e55549_d_n17;
        locals.var_mutau_dn18 = assign42300_e55549_d_n18;
        locals.var_mutau_dn19 = assign42300_e55549_d_n19;
        locals.var_mutau_dn20 = assign42300_e55549_d_n20;
        locals.var_mutau_rv = 0.0;

        let (assign42310_e55585, assign42310_e55585_d_n5, assign42310_e55585_d_n6, assign42310_e55585_d_n7, assign42310_e55585_d_n8, assign42310_e55585_d_n12, assign42310_e55585_d_n13, assign42310_e55585_d_n14, assign42310_e55585_d_n15, assign42310_e55585_d_n16, assign42310_e55585_d_n17, assign42310_e55585_d_n18, assign42310_e55585_d_n19, assign42310_e55585_d_n20,) = {
    if ((locals.var_guard1284 == 0.0) && (locals.var_guard1285 == 0.0)) {
        let assign42310_e55558: f64 = (locals.var_sp_s_a * locals.var_nu);
        let assign42310_e55560: f64 = (assign42310_e55558 * locals.var_sp_s_tau);
        let assign42310_e55564: f64 = (locals.var_nu / locals.var_mutau);
        let assign42310_e55566: f64 = (assign42310_e55564 * locals.var_sp_s_tau);
        let assign42310_e55568: f64 = (assign42310_e55566 * locals.var_sp_s_tau);
        let assign42310_e55570: f64 = (assign42310_e55568 * locals.var_sp_s_c);
        let assign42310_e55573: f64 = (locals.var_sp_s_c * locals.var_sp_s_c);
        let assign42310_e55575: f64 = (assign42310_e55573 * 0.3333333333333333);
        let assign42310_e55578: f64 = (locals.var_sp_s_a * locals.var_sp_s_b);
        let assign42310_e55579: f64 = (assign42310_e55575 - assign42310_e55578);
        let assign42310_e55580: f64 = (assign42310_e55570 * assign42310_e55579);
        let assign42310_e55581: f64 = (locals.var_mutau + assign42310_e55580);
        let assign42310_e55582: f64 = (assign42310_e55560 / assign42310_e55581);
        let assign42310_e55583: f64 = (locals.var_sp_s_eta + assign42310_e55582);
        (assign42310_e55583, (locals.var_sp_s_eta_dn5 + (((((((locals.var_sp_s_a_dn5 * locals.var_nu) + (locals.var_sp_s_a * locals.var_nu_dn5)) * locals.var_sp_s_tau) + (assign42310_e55558 * locals.var_sp_s_tau_dn5)) * assign42310_e55581) - (assign42310_e55560 * (locals.var_mutau_dn5 + (((((((((((locals.var_nu_dn5 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn5)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau) + (assign42310_e55564 * locals.var_sp_s_tau_dn5)) * locals.var_sp_s_tau) + (assign42310_e55566 * locals.var_sp_s_tau_dn5)) * locals.var_sp_s_c) + (assign42310_e55568 * locals.var_sp_s_c_dn5)) * assign42310_e55579) + (assign42310_e55570 * ((((locals.var_sp_s_c_dn5 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn5)) * 0.3333333333333333) - ((locals.var_sp_s_a_dn5 * locals.var_sp_s_b) + (locals.var_sp_s_a * locals.var_sp_s_b_dn5)))))))) / (assign42310_e55581 * assign42310_e55581))), (locals.var_sp_s_eta_dn6 + (((((((locals.var_sp_s_a_dn6 * locals.var_nu) + (locals.var_sp_s_a * locals.var_nu_dn6)) * locals.var_sp_s_tau) + (assign42310_e55558 * locals.var_sp_s_tau_dn6)) * assign42310_e55581) - (assign42310_e55560 * (locals.var_mutau_dn6 + (((((((((((locals.var_nu_dn6 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn6)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau) + (assign42310_e55564 * locals.var_sp_s_tau_dn6)) * locals.var_sp_s_tau) + (assign42310_e55566 * locals.var_sp_s_tau_dn6)) * locals.var_sp_s_c) + (assign42310_e55568 * locals.var_sp_s_c_dn6)) * assign42310_e55579) + (assign42310_e55570 * ((((locals.var_sp_s_c_dn6 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn6)) * 0.3333333333333333) - ((locals.var_sp_s_a_dn6 * locals.var_sp_s_b) + (locals.var_sp_s_a * locals.var_sp_s_b_dn6)))))))) / (assign42310_e55581 * assign42310_e55581))), (locals.var_sp_s_eta_dn7 + (((((((locals.var_sp_s_a_dn7 * locals.var_nu) + (locals.var_sp_s_a * locals.var_nu_dn7)) * locals.var_sp_s_tau) + (assign42310_e55558 * locals.var_sp_s_tau_dn7)) * assign42310_e55581) - (assign42310_e55560 * (locals.var_mutau_dn7 + (((((((((((locals.var_nu_dn7 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn7)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau) + (assign42310_e55564 * locals.var_sp_s_tau_dn7)) * locals.var_sp_s_tau) + (assign42310_e55566 * locals.var_sp_s_tau_dn7)) * locals.var_sp_s_c) + (assign42310_e55568 * locals.var_sp_s_c_dn7)) * assign42310_e55579) + (assign42310_e55570 * ((((locals.var_sp_s_c_dn7 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn7)) * 0.3333333333333333) - ((locals.var_sp_s_a_dn7 * locals.var_sp_s_b) + (locals.var_sp_s_a * locals.var_sp_s_b_dn7)))))))) / (assign42310_e55581 * assign42310_e55581))), (locals.var_sp_s_eta_dn8 + (((((((locals.var_sp_s_a_dn8 * locals.var_nu) + (locals.var_sp_s_a * locals.var_nu_dn8)) * locals.var_sp_s_tau) + (assign42310_e55558 * locals.var_sp_s_tau_dn8)) * assign42310_e55581) - (assign42310_e55560 * (locals.var_mutau_dn8 + (((((((((((locals.var_nu_dn8 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn8)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau) + (assign42310_e55564 * locals.var_sp_s_tau_dn8)) * locals.var_sp_s_tau) + (assign42310_e55566 * locals.var_sp_s_tau_dn8)) * locals.var_sp_s_c) + (assign42310_e55568 * locals.var_sp_s_c_dn8)) * assign42310_e55579) + (assign42310_e55570 * ((((locals.var_sp_s_c_dn8 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn8)) * 0.3333333333333333) - ((locals.var_sp_s_a_dn8 * locals.var_sp_s_b) + (locals.var_sp_s_a * locals.var_sp_s_b_dn8)))))))) / (assign42310_e55581 * assign42310_e55581))), (locals.var_sp_s_eta_dn12 + (((((((locals.var_sp_s_a_dn12 * locals.var_nu) + (locals.var_sp_s_a * locals.var_nu_dn12)) * locals.var_sp_s_tau) + (assign42310_e55558 * locals.var_sp_s_tau_dn12)) * assign42310_e55581) - (assign42310_e55560 * (locals.var_mutau_dn12 + (((((((((((locals.var_nu_dn12 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn12)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau) + (assign42310_e55564 * locals.var_sp_s_tau_dn12)) * locals.var_sp_s_tau) + (assign42310_e55566 * locals.var_sp_s_tau_dn12)) * locals.var_sp_s_c) + (assign42310_e55568 * locals.var_sp_s_c_dn12)) * assign42310_e55579) + (assign42310_e55570 * ((((locals.var_sp_s_c_dn12 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn12)) * 0.3333333333333333) - ((locals.var_sp_s_a_dn12 * locals.var_sp_s_b) + (locals.var_sp_s_a * locals.var_sp_s_b_dn12)))))))) / (assign42310_e55581 * assign42310_e55581))), (locals.var_sp_s_eta_dn13 + (((((((locals.var_sp_s_a_dn13 * locals.var_nu) + (locals.var_sp_s_a * locals.var_nu_dn13)) * locals.var_sp_s_tau) + (assign42310_e55558 * locals.var_sp_s_tau_dn13)) * assign42310_e55581) - (assign42310_e55560 * (locals.var_mutau_dn13 + (((((((((((locals.var_nu_dn13 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn13)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau) + (assign42310_e55564 * locals.var_sp_s_tau_dn13)) * locals.var_sp_s_tau) + (assign42310_e55566 * locals.var_sp_s_tau_dn13)) * locals.var_sp_s_c) + (assign42310_e55568 * locals.var_sp_s_c_dn13)) * assign42310_e55579) + (assign42310_e55570 * ((((locals.var_sp_s_c_dn13 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn13)) * 0.3333333333333333) - ((locals.var_sp_s_a_dn13 * locals.var_sp_s_b) + (locals.var_sp_s_a * locals.var_sp_s_b_dn13)))))))) / (assign42310_e55581 * assign42310_e55581))), (locals.var_sp_s_eta_dn14 + (((((((locals.var_sp_s_a_dn14 * locals.var_nu) + (locals.var_sp_s_a * locals.var_nu_dn14)) * locals.var_sp_s_tau) + (assign42310_e55558 * locals.var_sp_s_tau_dn14)) * assign42310_e55581) - (assign42310_e55560 * (locals.var_mutau_dn14 + (((((((((((locals.var_nu_dn14 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn14)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau) + (assign42310_e55564 * locals.var_sp_s_tau_dn14)) * locals.var_sp_s_tau) + (assign42310_e55566 * locals.var_sp_s_tau_dn14)) * locals.var_sp_s_c) + (assign42310_e55568 * locals.var_sp_s_c_dn14)) * assign42310_e55579) + (assign42310_e55570 * ((((locals.var_sp_s_c_dn14 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn14)) * 0.3333333333333333) - ((locals.var_sp_s_a_dn14 * locals.var_sp_s_b) + (locals.var_sp_s_a * locals.var_sp_s_b_dn14)))))))) / (assign42310_e55581 * assign42310_e55581))), (locals.var_sp_s_eta_dn15 + (((((((locals.var_sp_s_a_dn15 * locals.var_nu) + (locals.var_sp_s_a * locals.var_nu_dn15)) * locals.var_sp_s_tau) + (assign42310_e55558 * locals.var_sp_s_tau_dn15)) * assign42310_e55581) - (assign42310_e55560 * (locals.var_mutau_dn15 + (((((((((((locals.var_nu_dn15 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn15)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau) + (assign42310_e55564 * locals.var_sp_s_tau_dn15)) * locals.var_sp_s_tau) + (assign42310_e55566 * locals.var_sp_s_tau_dn15)) * locals.var_sp_s_c) + (assign42310_e55568 * locals.var_sp_s_c_dn15)) * assign42310_e55579) + (assign42310_e55570 * ((((locals.var_sp_s_c_dn15 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn15)) * 0.3333333333333333) - ((locals.var_sp_s_a_dn15 * locals.var_sp_s_b) + (locals.var_sp_s_a * locals.var_sp_s_b_dn15)))))))) / (assign42310_e55581 * assign42310_e55581))), (locals.var_sp_s_eta_dn16 + (((((((locals.var_sp_s_a_dn16 * locals.var_nu) + (locals.var_sp_s_a * locals.var_nu_dn16)) * locals.var_sp_s_tau) + (assign42310_e55558 * locals.var_sp_s_tau_dn16)) * assign42310_e55581) - (assign42310_e55560 * (locals.var_mutau_dn16 + (((((((((((locals.var_nu_dn16 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn16)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau) + (assign42310_e55564 * locals.var_sp_s_tau_dn16)) * locals.var_sp_s_tau) + (assign42310_e55566 * locals.var_sp_s_tau_dn16)) * locals.var_sp_s_c) + (assign42310_e55568 * locals.var_sp_s_c_dn16)) * assign42310_e55579) + (assign42310_e55570 * ((((locals.var_sp_s_c_dn16 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn16)) * 0.3333333333333333) - ((locals.var_sp_s_a_dn16 * locals.var_sp_s_b) + (locals.var_sp_s_a * locals.var_sp_s_b_dn16)))))))) / (assign42310_e55581 * assign42310_e55581))), (locals.var_sp_s_eta_dn17 + (((((((locals.var_sp_s_a_dn17 * locals.var_nu) + (locals.var_sp_s_a * locals.var_nu_dn17)) * locals.var_sp_s_tau) + (assign42310_e55558 * locals.var_sp_s_tau_dn17)) * assign42310_e55581) - (assign42310_e55560 * (locals.var_mutau_dn17 + (((((((((((locals.var_nu_dn17 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn17)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau) + (assign42310_e55564 * locals.var_sp_s_tau_dn17)) * locals.var_sp_s_tau) + (assign42310_e55566 * locals.var_sp_s_tau_dn17)) * locals.var_sp_s_c) + (assign42310_e55568 * locals.var_sp_s_c_dn17)) * assign42310_e55579) + (assign42310_e55570 * ((((locals.var_sp_s_c_dn17 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn17)) * 0.3333333333333333) - ((locals.var_sp_s_a_dn17 * locals.var_sp_s_b) + (locals.var_sp_s_a * locals.var_sp_s_b_dn17)))))))) / (assign42310_e55581 * assign42310_e55581))), (locals.var_sp_s_eta_dn18 + (((((((locals.var_sp_s_a_dn18 * locals.var_nu) + (locals.var_sp_s_a * locals.var_nu_dn18)) * locals.var_sp_s_tau) + (assign42310_e55558 * locals.var_sp_s_tau_dn18)) * assign42310_e55581) - (assign42310_e55560 * (locals.var_mutau_dn18 + (((((((((((locals.var_nu_dn18 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn18)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau) + (assign42310_e55564 * locals.var_sp_s_tau_dn18)) * locals.var_sp_s_tau) + (assign42310_e55566 * locals.var_sp_s_tau_dn18)) * locals.var_sp_s_c) + (assign42310_e55568 * locals.var_sp_s_c_dn18)) * assign42310_e55579) + (assign42310_e55570 * ((((locals.var_sp_s_c_dn18 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn18)) * 0.3333333333333333) - ((locals.var_sp_s_a_dn18 * locals.var_sp_s_b) + (locals.var_sp_s_a * locals.var_sp_s_b_dn18)))))))) / (assign42310_e55581 * assign42310_e55581))), (locals.var_sp_s_eta_dn19 + (((((((locals.var_sp_s_a_dn19 * locals.var_nu) + (locals.var_sp_s_a * locals.var_nu_dn19)) * locals.var_sp_s_tau) + (assign42310_e55558 * locals.var_sp_s_tau_dn19)) * assign42310_e55581) - (assign42310_e55560 * (locals.var_mutau_dn19 + (((((((((((locals.var_nu_dn19 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn19)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau) + (assign42310_e55564 * locals.var_sp_s_tau_dn19)) * locals.var_sp_s_tau) + (assign42310_e55566 * locals.var_sp_s_tau_dn19)) * locals.var_sp_s_c) + (assign42310_e55568 * locals.var_sp_s_c_dn19)) * assign42310_e55579) + (assign42310_e55570 * ((((locals.var_sp_s_c_dn19 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn19)) * 0.3333333333333333) - ((locals.var_sp_s_a_dn19 * locals.var_sp_s_b) + (locals.var_sp_s_a * locals.var_sp_s_b_dn19)))))))) / (assign42310_e55581 * assign42310_e55581))), (locals.var_sp_s_eta_dn20 + (((((((locals.var_sp_s_a_dn20 * locals.var_nu) + (locals.var_sp_s_a * locals.var_nu_dn20)) * locals.var_sp_s_tau) + (assign42310_e55558 * locals.var_sp_s_tau_dn20)) * assign42310_e55581) - (assign42310_e55560 * (locals.var_mutau_dn20 + (((((((((((locals.var_nu_dn20 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn20)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau) + (assign42310_e55564 * locals.var_sp_s_tau_dn20)) * locals.var_sp_s_tau) + (assign42310_e55566 * locals.var_sp_s_tau_dn20)) * locals.var_sp_s_c) + (assign42310_e55568 * locals.var_sp_s_c_dn20)) * assign42310_e55579) + (assign42310_e55570 * ((((locals.var_sp_s_c_dn20 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn20)) * 0.3333333333333333) - ((locals.var_sp_s_a_dn20 * locals.var_sp_s_b) + (locals.var_sp_s_a * locals.var_sp_s_b_dn20)))))))) / (assign42310_e55581 * assign42310_e55581))),)
    } else {
        (locals.var_sp_s_x0, locals.var_sp_s_x0_dn5, locals.var_sp_s_x0_dn6, locals.var_sp_s_x0_dn7, locals.var_sp_s_x0_dn8, locals.var_sp_s_x0_dn12, locals.var_sp_s_x0_dn13, locals.var_sp_s_x0_dn14, locals.var_sp_s_x0_dn15, locals.var_sp_s_x0_dn16, locals.var_sp_s_x0_dn17, locals.var_sp_s_x0_dn18, locals.var_sp_s_x0_dn19, locals.var_sp_s_x0_dn20,)
    }
};
        locals.var_sp_s_x0 = assign42310_e55585;
        locals.var_sp_s_x0_dn5 = assign42310_e55585_d_n5;
        locals.var_sp_s_x0_dn6 = assign42310_e55585_d_n6;
        locals.var_sp_s_x0_dn7 = assign42310_e55585_d_n7;
        locals.var_sp_s_x0_dn8 = assign42310_e55585_d_n8;
        locals.var_sp_s_x0_dn12 = assign42310_e55585_d_n12;
        locals.var_sp_s_x0_dn13 = assign42310_e55585_d_n13;
        locals.var_sp_s_x0_dn14 = assign42310_e55585_d_n14;
        locals.var_sp_s_x0_dn15 = assign42310_e55585_d_n15;
        locals.var_sp_s_x0_dn16 = assign42310_e55585_d_n16;
        locals.var_sp_s_x0_dn17 = assign42310_e55585_d_n17;
        locals.var_sp_s_x0_dn18 = assign42310_e55585_d_n18;
        locals.var_sp_s_x0_dn19 = assign42310_e55585_d_n19;
        locals.var_sp_s_x0_dn20 = assign42310_e55585_d_n20;
        locals.var_sp_s_x0_rv = 0.0;

        let assign42320_e55588: f64 = if locals.var_sp_s_x0 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard1288 = assign42320_e55588;
        locals.var_guard1288_rv = 0.0;

        let (assign42330_e55599, assign42330_e55599_d_n5, assign42330_e55599_d_n6, assign42330_e55599_d_n7, assign42330_e55599_d_n8, assign42330_e55599_d_n12, assign42330_e55599_d_n13, assign42330_e55599_d_n14, assign42330_e55599_d_n15, assign42330_e55599_d_n16, assign42330_e55599_d_n17, assign42330_e55599_d_n18, assign42330_e55599_d_n19, assign42330_e55599_d_n20,) = {
    if (((locals.var_guard1284 == 0.0) && (locals.var_guard1285 == 0.0)) && (locals.var_guard1288 != 0.0)) {
        let assign42330_e55597: f64 = (locals.var_sp_s_x0).exp();
        (assign42330_e55597, (assign42330_e55597 * locals.var_sp_s_x0_dn5), (assign42330_e55597 * locals.var_sp_s_x0_dn6), (assign42330_e55597 * locals.var_sp_s_x0_dn7), (assign42330_e55597 * locals.var_sp_s_x0_dn8), (assign42330_e55597 * locals.var_sp_s_x0_dn12), (assign42330_e55597 * locals.var_sp_s_x0_dn13), (assign42330_e55597 * locals.var_sp_s_x0_dn14), (assign42330_e55597 * locals.var_sp_s_x0_dn15), (assign42330_e55597 * locals.var_sp_s_x0_dn16), (assign42330_e55597 * locals.var_sp_s_x0_dn17), (assign42330_e55597 * locals.var_sp_s_x0_dn18), (assign42330_e55597 * locals.var_sp_s_x0_dn19), (assign42330_e55597 * locals.var_sp_s_x0_dn20),)
    } else {
        (locals.var_sp_s_delta0, locals.var_sp_s_delta0_dn5, locals.var_sp_s_delta0_dn6, locals.var_sp_s_delta0_dn7, locals.var_sp_s_delta0_dn8, locals.var_sp_s_delta0_dn12, locals.var_sp_s_delta0_dn13, locals.var_sp_s_delta0_dn14, locals.var_sp_s_delta0_dn15, locals.var_sp_s_delta0_dn16, locals.var_sp_s_delta0_dn17, locals.var_sp_s_delta0_dn18, locals.var_sp_s_delta0_dn19, locals.var_sp_s_delta0_dn20,)
    }
};
        locals.var_sp_s_delta0 = assign42330_e55599;
        locals.var_sp_s_delta0_dn5 = assign42330_e55599_d_n5;
        locals.var_sp_s_delta0_dn6 = assign42330_e55599_d_n6;
        locals.var_sp_s_delta0_dn7 = assign42330_e55599_d_n7;
        locals.var_sp_s_delta0_dn8 = assign42330_e55599_d_n8;
        locals.var_sp_s_delta0_dn12 = assign42330_e55599_d_n12;
        locals.var_sp_s_delta0_dn13 = assign42330_e55599_d_n13;
        locals.var_sp_s_delta0_dn14 = assign42330_e55599_d_n14;
        locals.var_sp_s_delta0_dn15 = assign42330_e55599_d_n15;
        locals.var_sp_s_delta0_dn16 = assign42330_e55599_d_n16;
        locals.var_sp_s_delta0_dn17 = assign42330_e55599_d_n17;
        locals.var_sp_s_delta0_dn18 = assign42330_e55599_d_n18;
        locals.var_sp_s_delta0_dn19 = assign42330_e55599_d_n19;
        locals.var_sp_s_delta0_dn20 = assign42330_e55599_d_n20;
        locals.var_sp_s_delta0_rv = 0.0;

        let (assign42340_e55611, assign42340_e55611_d_n5, assign42340_e55611_d_n6, assign42340_e55611_d_n7, assign42340_e55611_d_n8, assign42340_e55611_d_n12, assign42340_e55611_d_n13, assign42340_e55611_d_n14, assign42340_e55611_d_n15, assign42340_e55611_d_n16, assign42340_e55611_d_n17, assign42340_e55611_d_n18, assign42340_e55611_d_n19, assign42340_e55611_d_n20,) = {
    if (((locals.var_guard1284 == 0.0) && (locals.var_guard1285 == 0.0)) && (locals.var_guard1288 != 0.0)) {
        let assign42340_e55609: f64 = (1.0 / locals.var_sp_s_delta0);
        (assign42340_e55609, (-(locals.var_sp_s_delta0_dn5 / (locals.var_sp_s_delta0 * locals.var_sp_s_delta0))), (-(locals.var_sp_s_delta0_dn6 / (locals.var_sp_s_delta0 * locals.var_sp_s_delta0))), (-(locals.var_sp_s_delta0_dn7 / (locals.var_sp_s_delta0 * locals.var_sp_s_delta0))), (-(locals.var_sp_s_delta0_dn8 / (locals.var_sp_s_delta0 * locals.var_sp_s_delta0))), (-(locals.var_sp_s_delta0_dn12 / (locals.var_sp_s_delta0 * locals.var_sp_s_delta0))), (-(locals.var_sp_s_delta0_dn13 / (locals.var_sp_s_delta0 * locals.var_sp_s_delta0))), (-(locals.var_sp_s_delta0_dn14 / (locals.var_sp_s_delta0 * locals.var_sp_s_delta0))), (-(locals.var_sp_s_delta0_dn15 / (locals.var_sp_s_delta0 * locals.var_sp_s_delta0))), (-(locals.var_sp_s_delta0_dn16 / (locals.var_sp_s_delta0 * locals.var_sp_s_delta0))), (-(locals.var_sp_s_delta0_dn17 / (locals.var_sp_s_delta0 * locals.var_sp_s_delta0))), (-(locals.var_sp_s_delta0_dn18 / (locals.var_sp_s_delta0 * locals.var_sp_s_delta0))), (-(locals.var_sp_s_delta0_dn19 / (locals.var_sp_s_delta0 * locals.var_sp_s_delta0))), (-(locals.var_sp_s_delta0_dn20 / (locals.var_sp_s_delta0 * locals.var_sp_s_delta0))),)
    } else {
        (locals.var_sp_s_delta1, locals.var_sp_s_delta1_dn5, locals.var_sp_s_delta1_dn6, locals.var_sp_s_delta1_dn7, locals.var_sp_s_delta1_dn8, locals.var_sp_s_delta1_dn12, locals.var_sp_s_delta1_dn13, locals.var_sp_s_delta1_dn14, locals.var_sp_s_delta1_dn15, locals.var_sp_s_delta1_dn16, locals.var_sp_s_delta1_dn17, locals.var_sp_s_delta1_dn18, locals.var_sp_s_delta1_dn19, locals.var_sp_s_delta1_dn20,)
    }
};
        locals.var_sp_s_delta1 = assign42340_e55611;
        locals.var_sp_s_delta1_dn5 = assign42340_e55611_d_n5;
        locals.var_sp_s_delta1_dn6 = assign42340_e55611_d_n6;
        locals.var_sp_s_delta1_dn7 = assign42340_e55611_d_n7;
        locals.var_sp_s_delta1_dn8 = assign42340_e55611_d_n8;
        locals.var_sp_s_delta1_dn12 = assign42340_e55611_d_n12;
        locals.var_sp_s_delta1_dn13 = assign42340_e55611_d_n13;
        locals.var_sp_s_delta1_dn14 = assign42340_e55611_d_n14;
        locals.var_sp_s_delta1_dn15 = assign42340_e55611_d_n15;
        locals.var_sp_s_delta1_dn16 = assign42340_e55611_d_n16;
        locals.var_sp_s_delta1_dn17 = assign42340_e55611_d_n17;
        locals.var_sp_s_delta1_dn18 = assign42340_e55611_d_n18;
        locals.var_sp_s_delta1_dn19 = assign42340_e55611_d_n19;
        locals.var_sp_s_delta1_dn20 = assign42340_e55611_d_n20;
        locals.var_sp_s_delta1_rv = 0.0;

        let (assign42350_e55623, assign42350_e55623_d_n5, assign42350_e55623_d_n6, assign42350_e55623_d_n7, assign42350_e55623_d_n8, assign42350_e55623_d_n12, assign42350_e55623_d_n13, assign42350_e55623_d_n14, assign42350_e55623_d_n15, assign42350_e55623_d_n16, assign42350_e55623_d_n17, assign42350_e55623_d_n18, assign42350_e55623_d_n19, assign42350_e55623_d_n20,) = {
    if (((locals.var_guard1284 == 0.0) && (locals.var_guard1285 == 0.0)) && (locals.var_guard1288 != 0.0)) {
        let assign42350_e55621: f64 = (locals.var_delta_ns * locals.var_sp_s_delta0);
        (assign42350_e55621, ((locals.var_delta_ns_dn5 * locals.var_sp_s_delta0) + (locals.var_delta_ns * locals.var_sp_s_delta0_dn5)), ((locals.var_delta_ns_dn6 * locals.var_sp_s_delta0) + (locals.var_delta_ns * locals.var_sp_s_delta0_dn6)), ((locals.var_delta_ns_dn7 * locals.var_sp_s_delta0) + (locals.var_delta_ns * locals.var_sp_s_delta0_dn7)), ((locals.var_delta_ns_dn8 * locals.var_sp_s_delta0) + (locals.var_delta_ns * locals.var_sp_s_delta0_dn8)), ((locals.var_delta_ns_dn12 * locals.var_sp_s_delta0) + (locals.var_delta_ns * locals.var_sp_s_delta0_dn12)), ((locals.var_delta_ns_dn13 * locals.var_sp_s_delta0) + (locals.var_delta_ns * locals.var_sp_s_delta0_dn13)), ((locals.var_delta_ns_dn14 * locals.var_sp_s_delta0) + (locals.var_delta_ns * locals.var_sp_s_delta0_dn14)), ((locals.var_delta_ns_dn15 * locals.var_sp_s_delta0) + (locals.var_delta_ns * locals.var_sp_s_delta0_dn15)), ((locals.var_delta_ns_dn16 * locals.var_sp_s_delta0) + (locals.var_delta_ns * locals.var_sp_s_delta0_dn16)), ((locals.var_delta_ns_dn17 * locals.var_sp_s_delta0) + (locals.var_delta_ns * locals.var_sp_s_delta0_dn17)), ((locals.var_delta_ns_dn18 * locals.var_sp_s_delta0) + (locals.var_delta_ns * locals.var_sp_s_delta0_dn18)), ((locals.var_delta_ns_dn19 * locals.var_sp_s_delta0) + (locals.var_delta_ns * locals.var_sp_s_delta0_dn19)), ((locals.var_delta_ns_dn20 * locals.var_sp_s_delta0) + (locals.var_delta_ns * locals.var_sp_s_delta0_dn20)),)
    } else {
        (locals.var_sp_s_delta0, locals.var_sp_s_delta0_dn5, locals.var_sp_s_delta0_dn6, locals.var_sp_s_delta0_dn7, locals.var_sp_s_delta0_dn8, locals.var_sp_s_delta0_dn12, locals.var_sp_s_delta0_dn13, locals.var_sp_s_delta0_dn14, locals.var_sp_s_delta0_dn15, locals.var_sp_s_delta0_dn16, locals.var_sp_s_delta0_dn17, locals.var_sp_s_delta0_dn18, locals.var_sp_s_delta0_dn19, locals.var_sp_s_delta0_dn20,)
    }
};
        locals.var_sp_s_delta0 = assign42350_e55623;
        locals.var_sp_s_delta0_dn5 = assign42350_e55623_d_n5;
        locals.var_sp_s_delta0_dn6 = assign42350_e55623_d_n6;
        locals.var_sp_s_delta0_dn7 = assign42350_e55623_d_n7;
        locals.var_sp_s_delta0_dn8 = assign42350_e55623_d_n8;
        locals.var_sp_s_delta0_dn12 = assign42350_e55623_d_n12;
        locals.var_sp_s_delta0_dn13 = assign42350_e55623_d_n13;
        locals.var_sp_s_delta0_dn14 = assign42350_e55623_d_n14;
        locals.var_sp_s_delta0_dn15 = assign42350_e55623_d_n15;
        locals.var_sp_s_delta0_dn16 = assign42350_e55623_d_n16;
        locals.var_sp_s_delta0_dn17 = assign42350_e55623_d_n17;
        locals.var_sp_s_delta0_dn18 = assign42350_e55623_d_n18;
        locals.var_sp_s_delta0_dn19 = assign42350_e55623_d_n19;
        locals.var_sp_s_delta0_dn20 = assign42350_e55623_d_n20;
        locals.var_sp_s_delta0_rv = 0.0;

        let assign42360_e55627: f64 = (locals.var_xn_s - 230.25850929940458);
        let assign42360_e55628: f64 = if locals.var_sp_s_x0 > assign42360_e55627 { 1.0 } else { 0.0 };
        locals.var_guard1289 = assign42360_e55628;
        locals.var_guard1289_rv = 0.0;

        let (assign42370_e55644, assign42370_e55644_d_n5, assign42370_e55644_d_n6, assign42370_e55644_d_n7, assign42370_e55644_d_n8, assign42370_e55644_d_n12, assign42370_e55644_d_n13, assign42370_e55644_d_n14, assign42370_e55644_d_n15, assign42370_e55644_d_n16, assign42370_e55644_d_n17, assign42370_e55644_d_n18, assign42370_e55644_d_n19, assign42370_e55644_d_n20,) = {
    if ((((locals.var_guard1284 == 0.0) && (locals.var_guard1285 == 0.0)) && (locals.var_guard1288 == 0.0)) && (locals.var_guard1289 != 0.0)) {
        let assign42370_e55641: f64 = (locals.var_sp_s_x0 - locals.var_xn_s);
        let assign42370_e55642: f64 = (assign42370_e55641).exp();
        (assign42370_e55642, (assign42370_e55642 * (locals.var_sp_s_x0_dn5 - locals.var_xn_s_dn5)), (assign42370_e55642 * (locals.var_sp_s_x0_dn6 - locals.var_xn_s_dn6)), (assign42370_e55642 * (locals.var_sp_s_x0_dn7 - locals.var_xn_s_dn7)), (assign42370_e55642 * (locals.var_sp_s_x0_dn8 - locals.var_xn_s_dn8)), (assign42370_e55642 * (locals.var_sp_s_x0_dn12 - locals.var_xn_s_dn12)), (assign42370_e55642 * (locals.var_sp_s_x0_dn13 - locals.var_xn_s_dn13)), (assign42370_e55642 * (locals.var_sp_s_x0_dn14 - locals.var_xn_s_dn14)), (assign42370_e55642 * (locals.var_sp_s_x0_dn15 - locals.var_xn_s_dn15)), (assign42370_e55642 * (locals.var_sp_s_x0_dn16 - locals.var_xn_s_dn16)), (assign42370_e55642 * (locals.var_sp_s_x0_dn17 - locals.var_xn_s_dn17)), (assign42370_e55642 * (locals.var_sp_s_x0_dn18 - locals.var_xn_s_dn18)), (assign42370_e55642 * (locals.var_sp_s_x0_dn19 - locals.var_xn_s_dn19)), (assign42370_e55642 * (locals.var_sp_s_x0_dn20 - locals.var_xn_s_dn20)),)
    } else {
        (locals.var_sp_s_delta0, locals.var_sp_s_delta0_dn5, locals.var_sp_s_delta0_dn6, locals.var_sp_s_delta0_dn7, locals.var_sp_s_delta0_dn8, locals.var_sp_s_delta0_dn12, locals.var_sp_s_delta0_dn13, locals.var_sp_s_delta0_dn14, locals.var_sp_s_delta0_dn15, locals.var_sp_s_delta0_dn16, locals.var_sp_s_delta0_dn17, locals.var_sp_s_delta0_dn18, locals.var_sp_s_delta0_dn19, locals.var_sp_s_delta0_dn20,)
    }
};
        locals.var_sp_s_delta0 = assign42370_e55644;
        locals.var_sp_s_delta0_dn5 = assign42370_e55644_d_n5;
        locals.var_sp_s_delta0_dn6 = assign42370_e55644_d_n6;
        locals.var_sp_s_delta0_dn7 = assign42370_e55644_d_n7;
        locals.var_sp_s_delta0_dn8 = assign42370_e55644_d_n8;
        locals.var_sp_s_delta0_dn12 = assign42370_e55644_d_n12;
        locals.var_sp_s_delta0_dn13 = assign42370_e55644_d_n13;
        locals.var_sp_s_delta0_dn14 = assign42370_e55644_d_n14;
        locals.var_sp_s_delta0_dn15 = assign42370_e55644_d_n15;
        locals.var_sp_s_delta0_dn16 = assign42370_e55644_d_n16;
        locals.var_sp_s_delta0_dn17 = assign42370_e55644_d_n17;
        locals.var_sp_s_delta0_dn18 = assign42370_e55644_d_n18;
        locals.var_sp_s_delta0_dn19 = assign42370_e55644_d_n19;
        locals.var_sp_s_delta0_dn20 = assign42370_e55644_d_n20;
        locals.var_sp_s_delta0_rv = 0.0;

        let (assign42380_e55659, assign42380_e55659_d_n5, assign42380_e55659_d_n6, assign42380_e55659_d_n7, assign42380_e55659_d_n8, assign42380_e55659_d_n12, assign42380_e55659_d_n13, assign42380_e55659_d_n14, assign42380_e55659_d_n15, assign42380_e55659_d_n16, assign42380_e55659_d_n17, assign42380_e55659_d_n18, assign42380_e55659_d_n19, assign42380_e55659_d_n20,) = {
    if ((((locals.var_guard1284 == 0.0) && (locals.var_guard1285 == 0.0)) && (locals.var_guard1288 == 0.0)) && (locals.var_guard1289 != 0.0)) {
        let assign42380_e55657: f64 = (locals.var_delta_ns / locals.var_sp_s_delta0);
        (assign42380_e55657, (((locals.var_delta_ns_dn5 * locals.var_sp_s_delta0) - (locals.var_delta_ns * locals.var_sp_s_delta0_dn5)) / (locals.var_sp_s_delta0 * locals.var_sp_s_delta0)), (((locals.var_delta_ns_dn6 * locals.var_sp_s_delta0) - (locals.var_delta_ns * locals.var_sp_s_delta0_dn6)) / (locals.var_sp_s_delta0 * locals.var_sp_s_delta0)), (((locals.var_delta_ns_dn7 * locals.var_sp_s_delta0) - (locals.var_delta_ns * locals.var_sp_s_delta0_dn7)) / (locals.var_sp_s_delta0 * locals.var_sp_s_delta0)), (((locals.var_delta_ns_dn8 * locals.var_sp_s_delta0) - (locals.var_delta_ns * locals.var_sp_s_delta0_dn8)) / (locals.var_sp_s_delta0 * locals.var_sp_s_delta0)), (((locals.var_delta_ns_dn12 * locals.var_sp_s_delta0) - (locals.var_delta_ns * locals.var_sp_s_delta0_dn12)) / (locals.var_sp_s_delta0 * locals.var_sp_s_delta0)), (((locals.var_delta_ns_dn13 * locals.var_sp_s_delta0) - (locals.var_delta_ns * locals.var_sp_s_delta0_dn13)) / (locals.var_sp_s_delta0 * locals.var_sp_s_delta0)), (((locals.var_delta_ns_dn14 * locals.var_sp_s_delta0) - (locals.var_delta_ns * locals.var_sp_s_delta0_dn14)) / (locals.var_sp_s_delta0 * locals.var_sp_s_delta0)), (((locals.var_delta_ns_dn15 * locals.var_sp_s_delta0) - (locals.var_delta_ns * locals.var_sp_s_delta0_dn15)) / (locals.var_sp_s_delta0 * locals.var_sp_s_delta0)), (((locals.var_delta_ns_dn16 * locals.var_sp_s_delta0) - (locals.var_delta_ns * locals.var_sp_s_delta0_dn16)) / (locals.var_sp_s_delta0 * locals.var_sp_s_delta0)), (((locals.var_delta_ns_dn17 * locals.var_sp_s_delta0) - (locals.var_delta_ns * locals.var_sp_s_delta0_dn17)) / (locals.var_sp_s_delta0 * locals.var_sp_s_delta0)), (((locals.var_delta_ns_dn18 * locals.var_sp_s_delta0) - (locals.var_delta_ns * locals.var_sp_s_delta0_dn18)) / (locals.var_sp_s_delta0 * locals.var_sp_s_delta0)), (((locals.var_delta_ns_dn19 * locals.var_sp_s_delta0) - (locals.var_delta_ns * locals.var_sp_s_delta0_dn19)) / (locals.var_sp_s_delta0 * locals.var_sp_s_delta0)), (((locals.var_delta_ns_dn20 * locals.var_sp_s_delta0) - (locals.var_delta_ns * locals.var_sp_s_delta0_dn20)) / (locals.var_sp_s_delta0 * locals.var_sp_s_delta0)),)
    } else {
        (locals.var_sp_s_delta1, locals.var_sp_s_delta1_dn5, locals.var_sp_s_delta1_dn6, locals.var_sp_s_delta1_dn7, locals.var_sp_s_delta1_dn8, locals.var_sp_s_delta1_dn12, locals.var_sp_s_delta1_dn13, locals.var_sp_s_delta1_dn14, locals.var_sp_s_delta1_dn15, locals.var_sp_s_delta1_dn16, locals.var_sp_s_delta1_dn17, locals.var_sp_s_delta1_dn18, locals.var_sp_s_delta1_dn19, locals.var_sp_s_delta1_dn20,)
    }
};
        locals.var_sp_s_delta1 = assign42380_e55659;
        locals.var_sp_s_delta1_dn5 = assign42380_e55659_d_n5;
        locals.var_sp_s_delta1_dn6 = assign42380_e55659_d_n6;
        locals.var_sp_s_delta1_dn7 = assign42380_e55659_d_n7;
        locals.var_sp_s_delta1_dn8 = assign42380_e55659_d_n8;
        locals.var_sp_s_delta1_dn12 = assign42380_e55659_d_n12;
        locals.var_sp_s_delta1_dn13 = assign42380_e55659_d_n13;
        locals.var_sp_s_delta1_dn14 = assign42380_e55659_d_n14;
        locals.var_sp_s_delta1_dn15 = assign42380_e55659_d_n15;
        locals.var_sp_s_delta1_dn16 = assign42380_e55659_d_n16;
        locals.var_sp_s_delta1_dn17 = assign42380_e55659_d_n17;
        locals.var_sp_s_delta1_dn18 = assign42380_e55659_d_n18;
        locals.var_sp_s_delta1_dn19 = assign42380_e55659_d_n19;
        locals.var_sp_s_delta1_dn20 = assign42380_e55659_d_n20;
        locals.var_sp_s_delta1_rv = 0.0;

        let (assign42390_e55701, assign42390_e55701_d_n5, assign42390_e55701_d_n6, assign42390_e55701_d_n7, assign42390_e55701_d_n8, assign42390_e55701_d_n12, assign42390_e55701_d_n13, assign42390_e55701_d_n14, assign42390_e55701_d_n15, assign42390_e55701_d_n16, assign42390_e55701_d_n17, assign42390_e55701_d_n18, assign42390_e55701_d_n19, assign42390_e55701_d_n20,) = {
    if ((((locals.var_guard1284 == 0.0) && (locals.var_guard1285 == 0.0)) && (locals.var_guard1288 == 0.0)) && (locals.var_guard1289 == 0.0)) {
        let assign42390_e55675: f64 = (locals.var_xn_s - locals.var_sp_s_x0);
        let assign42390_e55677: f64 = (assign42390_e55675 - 230.25850929940458);
        let assign42390_e55682: f64 = (locals.var_xn_s - locals.var_sp_s_x0);
        let assign42390_e55684: f64 = (assign42390_e55682 - 230.25850929940458);
        let assign42390_e55688: f64 = (locals.var_xn_s - locals.var_sp_s_x0);
        let assign42390_e55690: f64 = (assign42390_e55688 - 230.25850929940458);
        let assign42390_e55692: f64 = (assign42390_e55690 * 0.3333333333333333);
        let assign42390_e55693: f64 = (1.0 + assign42390_e55692);
        let assign42390_e55694: f64 = (assign42390_e55684 * assign42390_e55693);
        let assign42390_e55695: f64 = (0.5 * assign42390_e55694);
        let assign42390_e55696: f64 = (1.0 + assign42390_e55695);
        let assign42390_e55697: f64 = (assign42390_e55677 * assign42390_e55696);
        let assign42390_e55698: f64 = (1.0 + assign42390_e55697);
        let assign42390_e55699: f64 = (1e-100 / assign42390_e55698);
        (assign42390_e55699, (-((1e-100 * (((locals.var_xn_s_dn5 - locals.var_sp_s_x0_dn5) * assign42390_e55696) + (assign42390_e55677 * (0.5 * (((locals.var_xn_s_dn5 - locals.var_sp_s_x0_dn5) * assign42390_e55693) + (assign42390_e55684 * ((locals.var_xn_s_dn5 - locals.var_sp_s_x0_dn5) * 0.3333333333333333))))))) / (assign42390_e55698 * assign42390_e55698))), (-((1e-100 * (((locals.var_xn_s_dn6 - locals.var_sp_s_x0_dn6) * assign42390_e55696) + (assign42390_e55677 * (0.5 * (((locals.var_xn_s_dn6 - locals.var_sp_s_x0_dn6) * assign42390_e55693) + (assign42390_e55684 * ((locals.var_xn_s_dn6 - locals.var_sp_s_x0_dn6) * 0.3333333333333333))))))) / (assign42390_e55698 * assign42390_e55698))), (-((1e-100 * (((locals.var_xn_s_dn7 - locals.var_sp_s_x0_dn7) * assign42390_e55696) + (assign42390_e55677 * (0.5 * (((locals.var_xn_s_dn7 - locals.var_sp_s_x0_dn7) * assign42390_e55693) + (assign42390_e55684 * ((locals.var_xn_s_dn7 - locals.var_sp_s_x0_dn7) * 0.3333333333333333))))))) / (assign42390_e55698 * assign42390_e55698))), (-((1e-100 * (((locals.var_xn_s_dn8 - locals.var_sp_s_x0_dn8) * assign42390_e55696) + (assign42390_e55677 * (0.5 * (((locals.var_xn_s_dn8 - locals.var_sp_s_x0_dn8) * assign42390_e55693) + (assign42390_e55684 * ((locals.var_xn_s_dn8 - locals.var_sp_s_x0_dn8) * 0.3333333333333333))))))) / (assign42390_e55698 * assign42390_e55698))), (-((1e-100 * (((locals.var_xn_s_dn12 - locals.var_sp_s_x0_dn12) * assign42390_e55696) + (assign42390_e55677 * (0.5 * (((locals.var_xn_s_dn12 - locals.var_sp_s_x0_dn12) * assign42390_e55693) + (assign42390_e55684 * ((locals.var_xn_s_dn12 - locals.var_sp_s_x0_dn12) * 0.3333333333333333))))))) / (assign42390_e55698 * assign42390_e55698))), (-((1e-100 * (((locals.var_xn_s_dn13 - locals.var_sp_s_x0_dn13) * assign42390_e55696) + (assign42390_e55677 * (0.5 * (((locals.var_xn_s_dn13 - locals.var_sp_s_x0_dn13) * assign42390_e55693) + (assign42390_e55684 * ((locals.var_xn_s_dn13 - locals.var_sp_s_x0_dn13) * 0.3333333333333333))))))) / (assign42390_e55698 * assign42390_e55698))), (-((1e-100 * (((locals.var_xn_s_dn14 - locals.var_sp_s_x0_dn14) * assign42390_e55696) + (assign42390_e55677 * (0.5 * (((locals.var_xn_s_dn14 - locals.var_sp_s_x0_dn14) * assign42390_e55693) + (assign42390_e55684 * ((locals.var_xn_s_dn14 - locals.var_sp_s_x0_dn14) * 0.3333333333333333))))))) / (assign42390_e55698 * assign42390_e55698))), (-((1e-100 * (((locals.var_xn_s_dn15 - locals.var_sp_s_x0_dn15) * assign42390_e55696) + (assign42390_e55677 * (0.5 * (((locals.var_xn_s_dn15 - locals.var_sp_s_x0_dn15) * assign42390_e55693) + (assign42390_e55684 * ((locals.var_xn_s_dn15 - locals.var_sp_s_x0_dn15) * 0.3333333333333333))))))) / (assign42390_e55698 * assign42390_e55698))), (-((1e-100 * (((locals.var_xn_s_dn16 - locals.var_sp_s_x0_dn16) * assign42390_e55696) + (assign42390_e55677 * (0.5 * (((locals.var_xn_s_dn16 - locals.var_sp_s_x0_dn16) * assign42390_e55693) + (assign42390_e55684 * ((locals.var_xn_s_dn16 - locals.var_sp_s_x0_dn16) * 0.3333333333333333))))))) / (assign42390_e55698 * assign42390_e55698))), (-((1e-100 * (((locals.var_xn_s_dn17 - locals.var_sp_s_x0_dn17) * assign42390_e55696) + (assign42390_e55677 * (0.5 * (((locals.var_xn_s_dn17 - locals.var_sp_s_x0_dn17) * assign42390_e55693) + (assign42390_e55684 * ((locals.var_xn_s_dn17 - locals.var_sp_s_x0_dn17) * 0.3333333333333333))))))) / (assign42390_e55698 * assign42390_e55698))), (-((1e-100 * (((locals.var_xn_s_dn18 - locals.var_sp_s_x0_dn18) * assign42390_e55696) + (assign42390_e55677 * (0.5 * (((locals.var_xn_s_dn18 - locals.var_sp_s_x0_dn18) * assign42390_e55693) + (assign42390_e55684 * ((locals.var_xn_s_dn18 - locals.var_sp_s_x0_dn18) * 0.3333333333333333))))))) / (assign42390_e55698 * assign42390_e55698))), (-((1e-100 * (((locals.var_xn_s_dn19 - locals.var_sp_s_x0_dn19) * assign42390_e55696) + (assign42390_e55677 * (0.5 * (((locals.var_xn_s_dn19 - locals.var_sp_s_x0_dn19) * assign42390_e55693) + (assign42390_e55684 * ((locals.var_xn_s_dn19 - locals.var_sp_s_x0_dn19) * 0.3333333333333333))))))) / (assign42390_e55698 * assign42390_e55698))), (-((1e-100 * (((locals.var_xn_s_dn20 - locals.var_sp_s_x0_dn20) * assign42390_e55696) + (assign42390_e55677 * (0.5 * (((locals.var_xn_s_dn20 - locals.var_sp_s_x0_dn20) * assign42390_e55693) + (assign42390_e55684 * ((locals.var_xn_s_dn20 - locals.var_sp_s_x0_dn20) * 0.3333333333333333))))))) / (assign42390_e55698 * assign42390_e55698))),)
    } else {
        (locals.var_sp_s_delta0, locals.var_sp_s_delta0_dn5, locals.var_sp_s_delta0_dn6, locals.var_sp_s_delta0_dn7, locals.var_sp_s_delta0_dn8, locals.var_sp_s_delta0_dn12, locals.var_sp_s_delta0_dn13, locals.var_sp_s_delta0_dn14, locals.var_sp_s_delta0_dn15, locals.var_sp_s_delta0_dn16, locals.var_sp_s_delta0_dn17, locals.var_sp_s_delta0_dn18, locals.var_sp_s_delta0_dn19, locals.var_sp_s_delta0_dn20,)
    }
};
        locals.var_sp_s_delta0 = assign42390_e55701;
        locals.var_sp_s_delta0_dn5 = assign42390_e55701_d_n5;
        locals.var_sp_s_delta0_dn6 = assign42390_e55701_d_n6;
        locals.var_sp_s_delta0_dn7 = assign42390_e55701_d_n7;
        locals.var_sp_s_delta0_dn8 = assign42390_e55701_d_n8;
        locals.var_sp_s_delta0_dn12 = assign42390_e55701_d_n12;
        locals.var_sp_s_delta0_dn13 = assign42390_e55701_d_n13;
        locals.var_sp_s_delta0_dn14 = assign42390_e55701_d_n14;
        locals.var_sp_s_delta0_dn15 = assign42390_e55701_d_n15;
        locals.var_sp_s_delta0_dn16 = assign42390_e55701_d_n16;
        locals.var_sp_s_delta0_dn17 = assign42390_e55701_d_n17;
        locals.var_sp_s_delta0_dn18 = assign42390_e55701_d_n18;
        locals.var_sp_s_delta0_dn19 = assign42390_e55701_d_n19;
        locals.var_sp_s_delta0_dn20 = assign42390_e55701_d_n20;
        locals.var_sp_s_delta0_rv = 0.0;

        let (assign42400_e55737, assign42400_e55737_d_n5, assign42400_e55737_d_n6, assign42400_e55737_d_n7, assign42400_e55737_d_n8, assign42400_e55737_d_n12, assign42400_e55737_d_n13, assign42400_e55737_d_n14, assign42400_e55737_d_n15, assign42400_e55737_d_n16, assign42400_e55737_d_n17, assign42400_e55737_d_n18, assign42400_e55737_d_n19, assign42400_e55737_d_n20,) = {
    if ((((locals.var_guard1284 == 0.0) && (locals.var_guard1285 == 0.0)) && (locals.var_guard1288 == 0.0)) && (locals.var_guard1289 == 0.0)) {
        let assign42400_e55717: f64 = (locals.var_sp_s_x0 - 230.25850929940458);
        let assign42400_e55722: f64 = (locals.var_sp_s_x0 - 230.25850929940458);
        let assign42400_e55726: f64 = (locals.var_sp_s_x0 - 230.25850929940458);
        let assign42400_e55728: f64 = (assign42400_e55726 * 0.3333333333333333);
        let assign42400_e55729: f64 = (1.0 + assign42400_e55728);
        let assign42400_e55730: f64 = (assign42400_e55722 * assign42400_e55729);
        let assign42400_e55731: f64 = (0.5 * assign42400_e55730);
        let assign42400_e55732: f64 = (1.0 + assign42400_e55731);
        let assign42400_e55733: f64 = (assign42400_e55717 * assign42400_e55732);
        let assign42400_e55734: f64 = (1.0 + assign42400_e55733);
        let assign42400_e55735: f64 = (1e-100 / assign42400_e55734);
        (assign42400_e55735, (-((1e-100 * ((locals.var_sp_s_x0_dn5 * assign42400_e55732) + (assign42400_e55717 * (0.5 * ((locals.var_sp_s_x0_dn5 * assign42400_e55729) + (assign42400_e55722 * (locals.var_sp_s_x0_dn5 * 0.3333333333333333))))))) / (assign42400_e55734 * assign42400_e55734))), (-((1e-100 * ((locals.var_sp_s_x0_dn6 * assign42400_e55732) + (assign42400_e55717 * (0.5 * ((locals.var_sp_s_x0_dn6 * assign42400_e55729) + (assign42400_e55722 * (locals.var_sp_s_x0_dn6 * 0.3333333333333333))))))) / (assign42400_e55734 * assign42400_e55734))), (-((1e-100 * ((locals.var_sp_s_x0_dn7 * assign42400_e55732) + (assign42400_e55717 * (0.5 * ((locals.var_sp_s_x0_dn7 * assign42400_e55729) + (assign42400_e55722 * (locals.var_sp_s_x0_dn7 * 0.3333333333333333))))))) / (assign42400_e55734 * assign42400_e55734))), (-((1e-100 * ((locals.var_sp_s_x0_dn8 * assign42400_e55732) + (assign42400_e55717 * (0.5 * ((locals.var_sp_s_x0_dn8 * assign42400_e55729) + (assign42400_e55722 * (locals.var_sp_s_x0_dn8 * 0.3333333333333333))))))) / (assign42400_e55734 * assign42400_e55734))), (-((1e-100 * ((locals.var_sp_s_x0_dn12 * assign42400_e55732) + (assign42400_e55717 * (0.5 * ((locals.var_sp_s_x0_dn12 * assign42400_e55729) + (assign42400_e55722 * (locals.var_sp_s_x0_dn12 * 0.3333333333333333))))))) / (assign42400_e55734 * assign42400_e55734))), (-((1e-100 * ((locals.var_sp_s_x0_dn13 * assign42400_e55732) + (assign42400_e55717 * (0.5 * ((locals.var_sp_s_x0_dn13 * assign42400_e55729) + (assign42400_e55722 * (locals.var_sp_s_x0_dn13 * 0.3333333333333333))))))) / (assign42400_e55734 * assign42400_e55734))), (-((1e-100 * ((locals.var_sp_s_x0_dn14 * assign42400_e55732) + (assign42400_e55717 * (0.5 * ((locals.var_sp_s_x0_dn14 * assign42400_e55729) + (assign42400_e55722 * (locals.var_sp_s_x0_dn14 * 0.3333333333333333))))))) / (assign42400_e55734 * assign42400_e55734))), (-((1e-100 * ((locals.var_sp_s_x0_dn15 * assign42400_e55732) + (assign42400_e55717 * (0.5 * ((locals.var_sp_s_x0_dn15 * assign42400_e55729) + (assign42400_e55722 * (locals.var_sp_s_x0_dn15 * 0.3333333333333333))))))) / (assign42400_e55734 * assign42400_e55734))), (-((1e-100 * ((locals.var_sp_s_x0_dn16 * assign42400_e55732) + (assign42400_e55717 * (0.5 * ((locals.var_sp_s_x0_dn16 * assign42400_e55729) + (assign42400_e55722 * (locals.var_sp_s_x0_dn16 * 0.3333333333333333))))))) / (assign42400_e55734 * assign42400_e55734))), (-((1e-100 * ((locals.var_sp_s_x0_dn17 * assign42400_e55732) + (assign42400_e55717 * (0.5 * ((locals.var_sp_s_x0_dn17 * assign42400_e55729) + (assign42400_e55722 * (locals.var_sp_s_x0_dn17 * 0.3333333333333333))))))) / (assign42400_e55734 * assign42400_e55734))), (-((1e-100 * ((locals.var_sp_s_x0_dn18 * assign42400_e55732) + (assign42400_e55717 * (0.5 * ((locals.var_sp_s_x0_dn18 * assign42400_e55729) + (assign42400_e55722 * (locals.var_sp_s_x0_dn18 * 0.3333333333333333))))))) / (assign42400_e55734 * assign42400_e55734))), (-((1e-100 * ((locals.var_sp_s_x0_dn19 * assign42400_e55732) + (assign42400_e55717 * (0.5 * ((locals.var_sp_s_x0_dn19 * assign42400_e55729) + (assign42400_e55722 * (locals.var_sp_s_x0_dn19 * 0.3333333333333333))))))) / (assign42400_e55734 * assign42400_e55734))), (-((1e-100 * ((locals.var_sp_s_x0_dn20 * assign42400_e55732) + (assign42400_e55717 * (0.5 * ((locals.var_sp_s_x0_dn20 * assign42400_e55729) + (assign42400_e55722 * (locals.var_sp_s_x0_dn20 * 0.3333333333333333))))))) / (assign42400_e55734 * assign42400_e55734))),)
    } else {
        (locals.var_sp_s_delta1, locals.var_sp_s_delta1_dn5, locals.var_sp_s_delta1_dn6, locals.var_sp_s_delta1_dn7, locals.var_sp_s_delta1_dn8, locals.var_sp_s_delta1_dn12, locals.var_sp_s_delta1_dn13, locals.var_sp_s_delta1_dn14, locals.var_sp_s_delta1_dn15, locals.var_sp_s_delta1_dn16, locals.var_sp_s_delta1_dn17, locals.var_sp_s_delta1_dn18, locals.var_sp_s_delta1_dn19, locals.var_sp_s_delta1_dn20,)
    }
};
        locals.var_sp_s_delta1 = assign42400_e55737;
        locals.var_sp_s_delta1_dn5 = assign42400_e55737_d_n5;
        locals.var_sp_s_delta1_dn6 = assign42400_e55737_d_n6;
        locals.var_sp_s_delta1_dn7 = assign42400_e55737_d_n7;
        locals.var_sp_s_delta1_dn8 = assign42400_e55737_d_n8;
        locals.var_sp_s_delta1_dn12 = assign42400_e55737_d_n12;
        locals.var_sp_s_delta1_dn13 = assign42400_e55737_d_n13;
        locals.var_sp_s_delta1_dn14 = assign42400_e55737_d_n14;
        locals.var_sp_s_delta1_dn15 = assign42400_e55737_d_n15;
        locals.var_sp_s_delta1_dn16 = assign42400_e55737_d_n16;
        locals.var_sp_s_delta1_dn17 = assign42400_e55737_d_n17;
        locals.var_sp_s_delta1_dn18 = assign42400_e55737_d_n18;
        locals.var_sp_s_delta1_dn19 = assign42400_e55737_d_n19;
        locals.var_sp_s_delta1_dn20 = assign42400_e55737_d_n20;
        locals.var_sp_s_delta1_rv = 0.0;

        let (assign42410_e55751, assign42410_e55751_d_n5, assign42410_e55751_d_n6, assign42410_e55751_d_n7, assign42410_e55751_d_n8, assign42410_e55751_d_n12, assign42410_e55751_d_n13, assign42410_e55751_d_n14, assign42410_e55751_d_n15, assign42410_e55751_d_n16, assign42410_e55751_d_n17, assign42410_e55751_d_n18, assign42410_e55751_d_n19, assign42410_e55751_d_n20,) = {
    if ((locals.var_guard1284 == 0.0) && (locals.var_guard1285 == 0.0)) {
        let assign42410_e55747: f64 = (locals.var_sp_s_x0 * locals.var_sp_s_x0);
        let assign42410_e55748: f64 = (2.0 + assign42410_e55747);
        let assign42410_e55749: f64 = (1.0 / assign42410_e55748);
        (assign42410_e55749, (-(((locals.var_sp_s_x0_dn5 * locals.var_sp_s_x0) + (locals.var_sp_s_x0 * locals.var_sp_s_x0_dn5)) / (assign42410_e55748 * assign42410_e55748))), (-(((locals.var_sp_s_x0_dn6 * locals.var_sp_s_x0) + (locals.var_sp_s_x0 * locals.var_sp_s_x0_dn6)) / (assign42410_e55748 * assign42410_e55748))), (-(((locals.var_sp_s_x0_dn7 * locals.var_sp_s_x0) + (locals.var_sp_s_x0 * locals.var_sp_s_x0_dn7)) / (assign42410_e55748 * assign42410_e55748))), (-(((locals.var_sp_s_x0_dn8 * locals.var_sp_s_x0) + (locals.var_sp_s_x0 * locals.var_sp_s_x0_dn8)) / (assign42410_e55748 * assign42410_e55748))), (-(((locals.var_sp_s_x0_dn12 * locals.var_sp_s_x0) + (locals.var_sp_s_x0 * locals.var_sp_s_x0_dn12)) / (assign42410_e55748 * assign42410_e55748))), (-(((locals.var_sp_s_x0_dn13 * locals.var_sp_s_x0) + (locals.var_sp_s_x0 * locals.var_sp_s_x0_dn13)) / (assign42410_e55748 * assign42410_e55748))), (-(((locals.var_sp_s_x0_dn14 * locals.var_sp_s_x0) + (locals.var_sp_s_x0 * locals.var_sp_s_x0_dn14)) / (assign42410_e55748 * assign42410_e55748))), (-(((locals.var_sp_s_x0_dn15 * locals.var_sp_s_x0) + (locals.var_sp_s_x0 * locals.var_sp_s_x0_dn15)) / (assign42410_e55748 * assign42410_e55748))), (-(((locals.var_sp_s_x0_dn16 * locals.var_sp_s_x0) + (locals.var_sp_s_x0 * locals.var_sp_s_x0_dn16)) / (assign42410_e55748 * assign42410_e55748))), (-(((locals.var_sp_s_x0_dn17 * locals.var_sp_s_x0) + (locals.var_sp_s_x0 * locals.var_sp_s_x0_dn17)) / (assign42410_e55748 * assign42410_e55748))), (-(((locals.var_sp_s_x0_dn18 * locals.var_sp_s_x0) + (locals.var_sp_s_x0 * locals.var_sp_s_x0_dn18)) / (assign42410_e55748 * assign42410_e55748))), (-(((locals.var_sp_s_x0_dn19 * locals.var_sp_s_x0) + (locals.var_sp_s_x0 * locals.var_sp_s_x0_dn19)) / (assign42410_e55748 * assign42410_e55748))), (-(((locals.var_sp_s_x0_dn20 * locals.var_sp_s_x0) + (locals.var_sp_s_x0 * locals.var_sp_s_x0_dn20)) / (assign42410_e55748 * assign42410_e55748))),)
    } else {
        (locals.var_sp_s_temp, locals.var_sp_s_temp_dn5, locals.var_sp_s_temp_dn6, locals.var_sp_s_temp_dn7, locals.var_sp_s_temp_dn8, locals.var_sp_s_temp_dn12, locals.var_sp_s_temp_dn13, locals.var_sp_s_temp_dn14, locals.var_sp_s_temp_dn15, locals.var_sp_s_temp_dn16, locals.var_sp_s_temp_dn17, locals.var_sp_s_temp_dn18, locals.var_sp_s_temp_dn19, locals.var_sp_s_temp_dn20,)
    }
};
        locals.var_sp_s_temp = assign42410_e55751;
        locals.var_sp_s_temp_dn5 = assign42410_e55751_d_n5;
        locals.var_sp_s_temp_dn6 = assign42410_e55751_d_n6;
        locals.var_sp_s_temp_dn7 = assign42410_e55751_d_n7;
        locals.var_sp_s_temp_dn8 = assign42410_e55751_d_n8;
        locals.var_sp_s_temp_dn12 = assign42410_e55751_d_n12;
        locals.var_sp_s_temp_dn13 = assign42410_e55751_d_n13;
        locals.var_sp_s_temp_dn14 = assign42410_e55751_d_n14;
        locals.var_sp_s_temp_dn15 = assign42410_e55751_d_n15;
        locals.var_sp_s_temp_dn16 = assign42410_e55751_d_n16;
        locals.var_sp_s_temp_dn17 = assign42410_e55751_d_n17;
        locals.var_sp_s_temp_dn18 = assign42410_e55751_d_n18;
        locals.var_sp_s_temp_dn19 = assign42410_e55751_d_n19;
        locals.var_sp_s_temp_dn20 = assign42410_e55751_d_n20;
        locals.var_sp_s_temp_rv = 0.0;

        let (assign42420_e55763, assign42420_e55763_d_n5, assign42420_e55763_d_n6, assign42420_e55763_d_n7, assign42420_e55763_d_n8, assign42420_e55763_d_n12, assign42420_e55763_d_n13, assign42420_e55763_d_n14, assign42420_e55763_d_n15, assign42420_e55763_d_n16, assign42420_e55763_d_n17, assign42420_e55763_d_n18, assign42420_e55763_d_n19, assign42420_e55763_d_n20,) = {
    if ((locals.var_guard1284 == 0.0) && (locals.var_guard1285 == 0.0)) {
        let assign42420_e55759: f64 = (locals.var_sp_s_x0 * locals.var_sp_s_x0);
        let assign42420_e55761: f64 = (assign42420_e55759 * locals.var_sp_s_temp);
        (assign42420_e55761, ((((locals.var_sp_s_x0_dn5 * locals.var_sp_s_x0) + (locals.var_sp_s_x0 * locals.var_sp_s_x0_dn5)) * locals.var_sp_s_temp) + (assign42420_e55759 * locals.var_sp_s_temp_dn5)), ((((locals.var_sp_s_x0_dn6 * locals.var_sp_s_x0) + (locals.var_sp_s_x0 * locals.var_sp_s_x0_dn6)) * locals.var_sp_s_temp) + (assign42420_e55759 * locals.var_sp_s_temp_dn6)), ((((locals.var_sp_s_x0_dn7 * locals.var_sp_s_x0) + (locals.var_sp_s_x0 * locals.var_sp_s_x0_dn7)) * locals.var_sp_s_temp) + (assign42420_e55759 * locals.var_sp_s_temp_dn7)), ((((locals.var_sp_s_x0_dn8 * locals.var_sp_s_x0) + (locals.var_sp_s_x0 * locals.var_sp_s_x0_dn8)) * locals.var_sp_s_temp) + (assign42420_e55759 * locals.var_sp_s_temp_dn8)), ((((locals.var_sp_s_x0_dn12 * locals.var_sp_s_x0) + (locals.var_sp_s_x0 * locals.var_sp_s_x0_dn12)) * locals.var_sp_s_temp) + (assign42420_e55759 * locals.var_sp_s_temp_dn12)), ((((locals.var_sp_s_x0_dn13 * locals.var_sp_s_x0) + (locals.var_sp_s_x0 * locals.var_sp_s_x0_dn13)) * locals.var_sp_s_temp) + (assign42420_e55759 * locals.var_sp_s_temp_dn13)), ((((locals.var_sp_s_x0_dn14 * locals.var_sp_s_x0) + (locals.var_sp_s_x0 * locals.var_sp_s_x0_dn14)) * locals.var_sp_s_temp) + (assign42420_e55759 * locals.var_sp_s_temp_dn14)), ((((locals.var_sp_s_x0_dn15 * locals.var_sp_s_x0) + (locals.var_sp_s_x0 * locals.var_sp_s_x0_dn15)) * locals.var_sp_s_temp) + (assign42420_e55759 * locals.var_sp_s_temp_dn15)), ((((locals.var_sp_s_x0_dn16 * locals.var_sp_s_x0) + (locals.var_sp_s_x0 * locals.var_sp_s_x0_dn16)) * locals.var_sp_s_temp) + (assign42420_e55759 * locals.var_sp_s_temp_dn16)), ((((locals.var_sp_s_x0_dn17 * locals.var_sp_s_x0) + (locals.var_sp_s_x0 * locals.var_sp_s_x0_dn17)) * locals.var_sp_s_temp) + (assign42420_e55759 * locals.var_sp_s_temp_dn17)), ((((locals.var_sp_s_x0_dn18 * locals.var_sp_s_x0) + (locals.var_sp_s_x0 * locals.var_sp_s_x0_dn18)) * locals.var_sp_s_temp) + (assign42420_e55759 * locals.var_sp_s_temp_dn18)), ((((locals.var_sp_s_x0_dn19 * locals.var_sp_s_x0) + (locals.var_sp_s_x0 * locals.var_sp_s_x0_dn19)) * locals.var_sp_s_temp) + (assign42420_e55759 * locals.var_sp_s_temp_dn19)), ((((locals.var_sp_s_x0_dn20 * locals.var_sp_s_x0) + (locals.var_sp_s_x0 * locals.var_sp_s_x0_dn20)) * locals.var_sp_s_temp) + (assign42420_e55759 * locals.var_sp_s_temp_dn20)),)
    } else {
        (locals.var_sp_s_xi0, locals.var_sp_s_xi0_dn5, locals.var_sp_s_xi0_dn6, locals.var_sp_s_xi0_dn7, locals.var_sp_s_xi0_dn8, locals.var_sp_s_xi0_dn12, locals.var_sp_s_xi0_dn13, locals.var_sp_s_xi0_dn14, locals.var_sp_s_xi0_dn15, locals.var_sp_s_xi0_dn16, locals.var_sp_s_xi0_dn17, locals.var_sp_s_xi0_dn18, locals.var_sp_s_xi0_dn19, locals.var_sp_s_xi0_dn20,)
    }
};
        locals.var_sp_s_xi0 = assign42420_e55763;
        locals.var_sp_s_xi0_dn5 = assign42420_e55763_d_n5;
        locals.var_sp_s_xi0_dn6 = assign42420_e55763_d_n6;
        locals.var_sp_s_xi0_dn7 = assign42420_e55763_d_n7;
        locals.var_sp_s_xi0_dn8 = assign42420_e55763_d_n8;
        locals.var_sp_s_xi0_dn12 = assign42420_e55763_d_n12;
        locals.var_sp_s_xi0_dn13 = assign42420_e55763_d_n13;
        locals.var_sp_s_xi0_dn14 = assign42420_e55763_d_n14;
        locals.var_sp_s_xi0_dn15 = assign42420_e55763_d_n15;
        locals.var_sp_s_xi0_dn16 = assign42420_e55763_d_n16;
        locals.var_sp_s_xi0_dn17 = assign42420_e55763_d_n17;
        locals.var_sp_s_xi0_dn18 = assign42420_e55763_d_n18;
        locals.var_sp_s_xi0_dn19 = assign42420_e55763_d_n19;
        locals.var_sp_s_xi0_dn20 = assign42420_e55763_d_n20;
        locals.var_sp_s_xi0_rv = 0.0;

        let (assign42430_e55777, assign42430_e55777_d_n5, assign42430_e55777_d_n6, assign42430_e55777_d_n7, assign42430_e55777_d_n8, assign42430_e55777_d_n12, assign42430_e55777_d_n13, assign42430_e55777_d_n14, assign42430_e55777_d_n15, assign42430_e55777_d_n16, assign42430_e55777_d_n17, assign42430_e55777_d_n18, assign42430_e55777_d_n19, assign42430_e55777_d_n20,) = {
    if ((locals.var_guard1284 == 0.0) && (locals.var_guard1285 == 0.0)) {
        let assign42430_e55772: f64 = (locals.var_sp_s_x0 * locals.var_sp_s_temp);
        let assign42430_e55774: f64 = (assign42430_e55772 * locals.var_sp_s_temp);
        let assign42430_e55775: f64 = (4.0 * assign42430_e55774);
        (assign42430_e55775, (4.0 * ((((locals.var_sp_s_x0_dn5 * locals.var_sp_s_temp) + (locals.var_sp_s_x0 * locals.var_sp_s_temp_dn5)) * locals.var_sp_s_temp) + (assign42430_e55772 * locals.var_sp_s_temp_dn5))), (4.0 * ((((locals.var_sp_s_x0_dn6 * locals.var_sp_s_temp) + (locals.var_sp_s_x0 * locals.var_sp_s_temp_dn6)) * locals.var_sp_s_temp) + (assign42430_e55772 * locals.var_sp_s_temp_dn6))), (4.0 * ((((locals.var_sp_s_x0_dn7 * locals.var_sp_s_temp) + (locals.var_sp_s_x0 * locals.var_sp_s_temp_dn7)) * locals.var_sp_s_temp) + (assign42430_e55772 * locals.var_sp_s_temp_dn7))), (4.0 * ((((locals.var_sp_s_x0_dn8 * locals.var_sp_s_temp) + (locals.var_sp_s_x0 * locals.var_sp_s_temp_dn8)) * locals.var_sp_s_temp) + (assign42430_e55772 * locals.var_sp_s_temp_dn8))), (4.0 * ((((locals.var_sp_s_x0_dn12 * locals.var_sp_s_temp) + (locals.var_sp_s_x0 * locals.var_sp_s_temp_dn12)) * locals.var_sp_s_temp) + (assign42430_e55772 * locals.var_sp_s_temp_dn12))), (4.0 * ((((locals.var_sp_s_x0_dn13 * locals.var_sp_s_temp) + (locals.var_sp_s_x0 * locals.var_sp_s_temp_dn13)) * locals.var_sp_s_temp) + (assign42430_e55772 * locals.var_sp_s_temp_dn13))), (4.0 * ((((locals.var_sp_s_x0_dn14 * locals.var_sp_s_temp) + (locals.var_sp_s_x0 * locals.var_sp_s_temp_dn14)) * locals.var_sp_s_temp) + (assign42430_e55772 * locals.var_sp_s_temp_dn14))), (4.0 * ((((locals.var_sp_s_x0_dn15 * locals.var_sp_s_temp) + (locals.var_sp_s_x0 * locals.var_sp_s_temp_dn15)) * locals.var_sp_s_temp) + (assign42430_e55772 * locals.var_sp_s_temp_dn15))), (4.0 * ((((locals.var_sp_s_x0_dn16 * locals.var_sp_s_temp) + (locals.var_sp_s_x0 * locals.var_sp_s_temp_dn16)) * locals.var_sp_s_temp) + (assign42430_e55772 * locals.var_sp_s_temp_dn16))), (4.0 * ((((locals.var_sp_s_x0_dn17 * locals.var_sp_s_temp) + (locals.var_sp_s_x0 * locals.var_sp_s_temp_dn17)) * locals.var_sp_s_temp) + (assign42430_e55772 * locals.var_sp_s_temp_dn17))), (4.0 * ((((locals.var_sp_s_x0_dn18 * locals.var_sp_s_temp) + (locals.var_sp_s_x0 * locals.var_sp_s_temp_dn18)) * locals.var_sp_s_temp) + (assign42430_e55772 * locals.var_sp_s_temp_dn18))), (4.0 * ((((locals.var_sp_s_x0_dn19 * locals.var_sp_s_temp) + (locals.var_sp_s_x0 * locals.var_sp_s_temp_dn19)) * locals.var_sp_s_temp) + (assign42430_e55772 * locals.var_sp_s_temp_dn19))), (4.0 * ((((locals.var_sp_s_x0_dn20 * locals.var_sp_s_temp) + (locals.var_sp_s_x0 * locals.var_sp_s_temp_dn20)) * locals.var_sp_s_temp) + (assign42430_e55772 * locals.var_sp_s_temp_dn20))),)
    } else {
        (locals.var_sp_s_xi1, locals.var_sp_s_xi1_dn5, locals.var_sp_s_xi1_dn6, locals.var_sp_s_xi1_dn7, locals.var_sp_s_xi1_dn8, locals.var_sp_s_xi1_dn12, locals.var_sp_s_xi1_dn13, locals.var_sp_s_xi1_dn14, locals.var_sp_s_xi1_dn15, locals.var_sp_s_xi1_dn16, locals.var_sp_s_xi1_dn17, locals.var_sp_s_xi1_dn18, locals.var_sp_s_xi1_dn19, locals.var_sp_s_xi1_dn20,)
    }
};
        locals.var_sp_s_xi1 = assign42430_e55777;
        locals.var_sp_s_xi1_dn5 = assign42430_e55777_d_n5;
        locals.var_sp_s_xi1_dn6 = assign42430_e55777_d_n6;
        locals.var_sp_s_xi1_dn7 = assign42430_e55777_d_n7;
        locals.var_sp_s_xi1_dn8 = assign42430_e55777_d_n8;
        locals.var_sp_s_xi1_dn12 = assign42430_e55777_d_n12;
        locals.var_sp_s_xi1_dn13 = assign42430_e55777_d_n13;
        locals.var_sp_s_xi1_dn14 = assign42430_e55777_d_n14;
        locals.var_sp_s_xi1_dn15 = assign42430_e55777_d_n15;
        locals.var_sp_s_xi1_dn16 = assign42430_e55777_d_n16;
        locals.var_sp_s_xi1_dn17 = assign42430_e55777_d_n17;
        locals.var_sp_s_xi1_dn18 = assign42430_e55777_d_n18;
        locals.var_sp_s_xi1_dn19 = assign42430_e55777_d_n19;
        locals.var_sp_s_xi1_dn20 = assign42430_e55777_d_n20;
        locals.var_sp_s_xi1_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_23(
        locals: &mut StampLocals,
    ) {
        let (assign42440_e55795, assign42440_e55795_d_n5, assign42440_e55795_d_n6, assign42440_e55795_d_n7, assign42440_e55795_d_n8, assign42440_e55795_d_n12, assign42440_e55795_d_n13, assign42440_e55795_d_n14, assign42440_e55795_d_n15, assign42440_e55795_d_n16, assign42440_e55795_d_n17, assign42440_e55795_d_n18, assign42440_e55795_d_n19, assign42440_e55795_d_n20,) = {
    if ((locals.var_guard1284 == 0.0) && (locals.var_guard1285 == 0.0)) {
        let assign42440_e55785: f64 = (8.0 * locals.var_sp_s_temp);
        let assign42440_e55788: f64 = (12.0 * locals.var_sp_s_xi0);
        let assign42440_e55789: f64 = (assign42440_e55785 - assign42440_e55788);
        let assign42440_e55791: f64 = (assign42440_e55789 * locals.var_sp_s_temp);
        let assign42440_e55793: f64 = (assign42440_e55791 * locals.var_sp_s_temp);
        (assign42440_e55793, ((((((8.0 * locals.var_sp_s_temp_dn5) - (12.0 * locals.var_sp_s_xi0_dn5)) * locals.var_sp_s_temp) + (assign42440_e55789 * locals.var_sp_s_temp_dn5)) * locals.var_sp_s_temp) + (assign42440_e55791 * locals.var_sp_s_temp_dn5)), ((((((8.0 * locals.var_sp_s_temp_dn6) - (12.0 * locals.var_sp_s_xi0_dn6)) * locals.var_sp_s_temp) + (assign42440_e55789 * locals.var_sp_s_temp_dn6)) * locals.var_sp_s_temp) + (assign42440_e55791 * locals.var_sp_s_temp_dn6)), ((((((8.0 * locals.var_sp_s_temp_dn7) - (12.0 * locals.var_sp_s_xi0_dn7)) * locals.var_sp_s_temp) + (assign42440_e55789 * locals.var_sp_s_temp_dn7)) * locals.var_sp_s_temp) + (assign42440_e55791 * locals.var_sp_s_temp_dn7)), ((((((8.0 * locals.var_sp_s_temp_dn8) - (12.0 * locals.var_sp_s_xi0_dn8)) * locals.var_sp_s_temp) + (assign42440_e55789 * locals.var_sp_s_temp_dn8)) * locals.var_sp_s_temp) + (assign42440_e55791 * locals.var_sp_s_temp_dn8)), ((((((8.0 * locals.var_sp_s_temp_dn12) - (12.0 * locals.var_sp_s_xi0_dn12)) * locals.var_sp_s_temp) + (assign42440_e55789 * locals.var_sp_s_temp_dn12)) * locals.var_sp_s_temp) + (assign42440_e55791 * locals.var_sp_s_temp_dn12)), ((((((8.0 * locals.var_sp_s_temp_dn13) - (12.0 * locals.var_sp_s_xi0_dn13)) * locals.var_sp_s_temp) + (assign42440_e55789 * locals.var_sp_s_temp_dn13)) * locals.var_sp_s_temp) + (assign42440_e55791 * locals.var_sp_s_temp_dn13)), ((((((8.0 * locals.var_sp_s_temp_dn14) - (12.0 * locals.var_sp_s_xi0_dn14)) * locals.var_sp_s_temp) + (assign42440_e55789 * locals.var_sp_s_temp_dn14)) * locals.var_sp_s_temp) + (assign42440_e55791 * locals.var_sp_s_temp_dn14)), ((((((8.0 * locals.var_sp_s_temp_dn15) - (12.0 * locals.var_sp_s_xi0_dn15)) * locals.var_sp_s_temp) + (assign42440_e55789 * locals.var_sp_s_temp_dn15)) * locals.var_sp_s_temp) + (assign42440_e55791 * locals.var_sp_s_temp_dn15)), ((((((8.0 * locals.var_sp_s_temp_dn16) - (12.0 * locals.var_sp_s_xi0_dn16)) * locals.var_sp_s_temp) + (assign42440_e55789 * locals.var_sp_s_temp_dn16)) * locals.var_sp_s_temp) + (assign42440_e55791 * locals.var_sp_s_temp_dn16)), ((((((8.0 * locals.var_sp_s_temp_dn17) - (12.0 * locals.var_sp_s_xi0_dn17)) * locals.var_sp_s_temp) + (assign42440_e55789 * locals.var_sp_s_temp_dn17)) * locals.var_sp_s_temp) + (assign42440_e55791 * locals.var_sp_s_temp_dn17)), ((((((8.0 * locals.var_sp_s_temp_dn18) - (12.0 * locals.var_sp_s_xi0_dn18)) * locals.var_sp_s_temp) + (assign42440_e55789 * locals.var_sp_s_temp_dn18)) * locals.var_sp_s_temp) + (assign42440_e55791 * locals.var_sp_s_temp_dn18)), ((((((8.0 * locals.var_sp_s_temp_dn19) - (12.0 * locals.var_sp_s_xi0_dn19)) * locals.var_sp_s_temp) + (assign42440_e55789 * locals.var_sp_s_temp_dn19)) * locals.var_sp_s_temp) + (assign42440_e55791 * locals.var_sp_s_temp_dn19)), ((((((8.0 * locals.var_sp_s_temp_dn20) - (12.0 * locals.var_sp_s_xi0_dn20)) * locals.var_sp_s_temp) + (assign42440_e55789 * locals.var_sp_s_temp_dn20)) * locals.var_sp_s_temp) + (assign42440_e55791 * locals.var_sp_s_temp_dn20)),)
    } else {
        (locals.var_sp_s_xi2, locals.var_sp_s_xi2_dn5, locals.var_sp_s_xi2_dn6, locals.var_sp_s_xi2_dn7, locals.var_sp_s_xi2_dn8, locals.var_sp_s_xi2_dn12, locals.var_sp_s_xi2_dn13, locals.var_sp_s_xi2_dn14, locals.var_sp_s_xi2_dn15, locals.var_sp_s_xi2_dn16, locals.var_sp_s_xi2_dn17, locals.var_sp_s_xi2_dn18, locals.var_sp_s_xi2_dn19, locals.var_sp_s_xi2_dn20,)
    }
};
        locals.var_sp_s_xi2 = assign42440_e55795;
        locals.var_sp_s_xi2_dn5 = assign42440_e55795_d_n5;
        locals.var_sp_s_xi2_dn6 = assign42440_e55795_d_n6;
        locals.var_sp_s_xi2_dn7 = assign42440_e55795_d_n7;
        locals.var_sp_s_xi2_dn8 = assign42440_e55795_d_n8;
        locals.var_sp_s_xi2_dn12 = assign42440_e55795_d_n12;
        locals.var_sp_s_xi2_dn13 = assign42440_e55795_d_n13;
        locals.var_sp_s_xi2_dn14 = assign42440_e55795_d_n14;
        locals.var_sp_s_xi2_dn15 = assign42440_e55795_d_n15;
        locals.var_sp_s_xi2_dn16 = assign42440_e55795_d_n16;
        locals.var_sp_s_xi2_dn17 = assign42440_e55795_d_n17;
        locals.var_sp_s_xi2_dn18 = assign42440_e55795_d_n18;
        locals.var_sp_s_xi2_dn19 = assign42440_e55795_d_n19;
        locals.var_sp_s_xi2_dn20 = assign42440_e55795_d_n20;
        locals.var_sp_s_xi2_rv = 0.0;

        let (assign42450_e55805, assign42450_e55805_d_n5, assign42450_e55805_d_n6, assign42450_e55805_d_n7, assign42450_e55805_d_n8, assign42450_e55805_d_n12, assign42450_e55805_d_n13, assign42450_e55805_d_n14, assign42450_e55805_d_n15, assign42450_e55805_d_n16, assign42450_e55805_d_n17, assign42450_e55805_d_n18, assign42450_e55805_d_n19, assign42450_e55805_d_n20,) = {
    if ((locals.var_guard1284 == 0.0) && (locals.var_guard1285 == 0.0)) {
        let assign42450_e55803: f64 = (locals.var_xg - locals.var_sp_s_x0);
        (assign42450_e55803, (locals.var_xg_dn5 - locals.var_sp_s_x0_dn5), (locals.var_xg_dn6 - locals.var_sp_s_x0_dn6), (locals.var_xg_dn7 - locals.var_sp_s_x0_dn7), (locals.var_xg_dn8 - locals.var_sp_s_x0_dn8), (locals.var_xg_dn12 - locals.var_sp_s_x0_dn12), (locals.var_xg_dn13 - locals.var_sp_s_x0_dn13), (locals.var_xg_dn14 - locals.var_sp_s_x0_dn14), (locals.var_xg_dn15 - locals.var_sp_s_x0_dn15), (locals.var_xg_dn16 - locals.var_sp_s_x0_dn16), (locals.var_xg_dn17 - locals.var_sp_s_x0_dn17), (locals.var_xg_dn18 - locals.var_sp_s_x0_dn18), (locals.var_xg_dn19 - locals.var_sp_s_x0_dn19), (locals.var_xg_dn20 - locals.var_sp_s_x0_dn20),)
    } else {
        (locals.var_sp_s_temp, locals.var_sp_s_temp_dn5, locals.var_sp_s_temp_dn6, locals.var_sp_s_temp_dn7, locals.var_sp_s_temp_dn8, locals.var_sp_s_temp_dn12, locals.var_sp_s_temp_dn13, locals.var_sp_s_temp_dn14, locals.var_sp_s_temp_dn15, locals.var_sp_s_temp_dn16, locals.var_sp_s_temp_dn17, locals.var_sp_s_temp_dn18, locals.var_sp_s_temp_dn19, locals.var_sp_s_temp_dn20,)
    }
};
        locals.var_sp_s_temp = assign42450_e55805;
        locals.var_sp_s_temp_dn5 = assign42450_e55805_d_n5;
        locals.var_sp_s_temp_dn6 = assign42450_e55805_d_n6;
        locals.var_sp_s_temp_dn7 = assign42450_e55805_d_n7;
        locals.var_sp_s_temp_dn8 = assign42450_e55805_d_n8;
        locals.var_sp_s_temp_dn12 = assign42450_e55805_d_n12;
        locals.var_sp_s_temp_dn13 = assign42450_e55805_d_n13;
        locals.var_sp_s_temp_dn14 = assign42450_e55805_d_n14;
        locals.var_sp_s_temp_dn15 = assign42450_e55805_d_n15;
        locals.var_sp_s_temp_dn16 = assign42450_e55805_d_n16;
        locals.var_sp_s_temp_dn17 = assign42450_e55805_d_n17;
        locals.var_sp_s_temp_dn18 = assign42450_e55805_d_n18;
        locals.var_sp_s_temp_dn19 = assign42450_e55805_d_n19;
        locals.var_sp_s_temp_dn20 = assign42450_e55805_d_n20;
        locals.var_sp_s_temp_rv = 0.0;

        let (assign42460_e55829, assign42460_e55829_d_n5, assign42460_e55829_d_n6, assign42460_e55829_d_n7, assign42460_e55829_d_n8, assign42460_e55829_d_n12, assign42460_e55829_d_n13, assign42460_e55829_d_n14, assign42460_e55829_d_n15, assign42460_e55829_d_n16, assign42460_e55829_d_n17, assign42460_e55829_d_n18, assign42460_e55829_d_n19, assign42460_e55829_d_n20,) = {
    if ((locals.var_guard1284 == 0.0) && (locals.var_guard1285 == 0.0)) {
        let assign42460_e55813: f64 = (2.0 * locals.var_sp_s_temp);
        let assign42460_e55817: f64 = (1.0 - locals.var_sp_s_delta1);
        let assign42460_e55819: f64 = (assign42460_e55817 + locals.var_sp_s_delta0);
        let assign42460_e55823: f64 = (1.0 + locals.var_sp_s_xi1);
        let assign42460_e55824: f64 = (locals.var_delta_ns * assign42460_e55823);
        let assign42460_e55825: f64 = (assign42460_e55819 - assign42460_e55824);
        let assign42460_e55826: f64 = (locals.var_gf2 * assign42460_e55825);
        let assign42460_e55827: f64 = (assign42460_e55813 + assign42460_e55826);
        (assign42460_e55827, ((2.0 * locals.var_sp_s_temp_dn5) + ((locals.var_gf2_dn5 * assign42460_e55825) + (locals.var_gf2 * (((-locals.var_sp_s_delta1_dn5) + locals.var_sp_s_delta0_dn5) - ((locals.var_delta_ns_dn5 * assign42460_e55823) + (locals.var_delta_ns * locals.var_sp_s_xi1_dn5)))))), ((2.0 * locals.var_sp_s_temp_dn6) + ((locals.var_gf2_dn6 * assign42460_e55825) + (locals.var_gf2 * (((-locals.var_sp_s_delta1_dn6) + locals.var_sp_s_delta0_dn6) - ((locals.var_delta_ns_dn6 * assign42460_e55823) + (locals.var_delta_ns * locals.var_sp_s_xi1_dn6)))))), ((2.0 * locals.var_sp_s_temp_dn7) + ((locals.var_gf2_dn7 * assign42460_e55825) + (locals.var_gf2 * (((-locals.var_sp_s_delta1_dn7) + locals.var_sp_s_delta0_dn7) - ((locals.var_delta_ns_dn7 * assign42460_e55823) + (locals.var_delta_ns * locals.var_sp_s_xi1_dn7)))))), ((2.0 * locals.var_sp_s_temp_dn8) + ((locals.var_gf2_dn8 * assign42460_e55825) + (locals.var_gf2 * (((-locals.var_sp_s_delta1_dn8) + locals.var_sp_s_delta0_dn8) - ((locals.var_delta_ns_dn8 * assign42460_e55823) + (locals.var_delta_ns * locals.var_sp_s_xi1_dn8)))))), ((2.0 * locals.var_sp_s_temp_dn12) + ((locals.var_gf2_dn12 * assign42460_e55825) + (locals.var_gf2 * (((-locals.var_sp_s_delta1_dn12) + locals.var_sp_s_delta0_dn12) - ((locals.var_delta_ns_dn12 * assign42460_e55823) + (locals.var_delta_ns * locals.var_sp_s_xi1_dn12)))))), ((2.0 * locals.var_sp_s_temp_dn13) + ((locals.var_gf2_dn13 * assign42460_e55825) + (locals.var_gf2 * (((-locals.var_sp_s_delta1_dn13) + locals.var_sp_s_delta0_dn13) - ((locals.var_delta_ns_dn13 * assign42460_e55823) + (locals.var_delta_ns * locals.var_sp_s_xi1_dn13)))))), ((2.0 * locals.var_sp_s_temp_dn14) + ((locals.var_gf2_dn14 * assign42460_e55825) + (locals.var_gf2 * (((-locals.var_sp_s_delta1_dn14) + locals.var_sp_s_delta0_dn14) - ((locals.var_delta_ns_dn14 * assign42460_e55823) + (locals.var_delta_ns * locals.var_sp_s_xi1_dn14)))))), ((2.0 * locals.var_sp_s_temp_dn15) + ((locals.var_gf2_dn15 * assign42460_e55825) + (locals.var_gf2 * (((-locals.var_sp_s_delta1_dn15) + locals.var_sp_s_delta0_dn15) - ((locals.var_delta_ns_dn15 * assign42460_e55823) + (locals.var_delta_ns * locals.var_sp_s_xi1_dn15)))))), ((2.0 * locals.var_sp_s_temp_dn16) + ((locals.var_gf2_dn16 * assign42460_e55825) + (locals.var_gf2 * (((-locals.var_sp_s_delta1_dn16) + locals.var_sp_s_delta0_dn16) - ((locals.var_delta_ns_dn16 * assign42460_e55823) + (locals.var_delta_ns * locals.var_sp_s_xi1_dn16)))))), ((2.0 * locals.var_sp_s_temp_dn17) + ((locals.var_gf2_dn17 * assign42460_e55825) + (locals.var_gf2 * (((-locals.var_sp_s_delta1_dn17) + locals.var_sp_s_delta0_dn17) - ((locals.var_delta_ns_dn17 * assign42460_e55823) + (locals.var_delta_ns * locals.var_sp_s_xi1_dn17)))))), ((2.0 * locals.var_sp_s_temp_dn18) + ((locals.var_gf2_dn18 * assign42460_e55825) + (locals.var_gf2 * (((-locals.var_sp_s_delta1_dn18) + locals.var_sp_s_delta0_dn18) - ((locals.var_delta_ns_dn18 * assign42460_e55823) + (locals.var_delta_ns * locals.var_sp_s_xi1_dn18)))))), ((2.0 * locals.var_sp_s_temp_dn19) + ((locals.var_gf2_dn19 * assign42460_e55825) + (locals.var_gf2 * (((-locals.var_sp_s_delta1_dn19) + locals.var_sp_s_delta0_dn19) - ((locals.var_delta_ns_dn19 * assign42460_e55823) + (locals.var_delta_ns * locals.var_sp_s_xi1_dn19)))))), ((2.0 * locals.var_sp_s_temp_dn20) + ((locals.var_gf2_dn20 * assign42460_e55825) + (locals.var_gf2 * (((-locals.var_sp_s_delta1_dn20) + locals.var_sp_s_delta0_dn20) - ((locals.var_delta_ns_dn20 * assign42460_e55823) + (locals.var_delta_ns * locals.var_sp_s_xi1_dn20)))))),)
    } else {
        (locals.var_sp_s_pc, locals.var_sp_s_pc_dn5, locals.var_sp_s_pc_dn6, locals.var_sp_s_pc_dn7, locals.var_sp_s_pc_dn8, locals.var_sp_s_pc_dn12, locals.var_sp_s_pc_dn13, locals.var_sp_s_pc_dn14, locals.var_sp_s_pc_dn15, locals.var_sp_s_pc_dn16, locals.var_sp_s_pc_dn17, locals.var_sp_s_pc_dn18, locals.var_sp_s_pc_dn19, locals.var_sp_s_pc_dn20,)
    }
};
        locals.var_sp_s_pc = assign42460_e55829;
        locals.var_sp_s_pc_dn5 = assign42460_e55829_d_n5;
        locals.var_sp_s_pc_dn6 = assign42460_e55829_d_n6;
        locals.var_sp_s_pc_dn7 = assign42460_e55829_d_n7;
        locals.var_sp_s_pc_dn8 = assign42460_e55829_d_n8;
        locals.var_sp_s_pc_dn12 = assign42460_e55829_d_n12;
        locals.var_sp_s_pc_dn13 = assign42460_e55829_d_n13;
        locals.var_sp_s_pc_dn14 = assign42460_e55829_d_n14;
        locals.var_sp_s_pc_dn15 = assign42460_e55829_d_n15;
        locals.var_sp_s_pc_dn16 = assign42460_e55829_d_n16;
        locals.var_sp_s_pc_dn17 = assign42460_e55829_d_n17;
        locals.var_sp_s_pc_dn18 = assign42460_e55829_d_n18;
        locals.var_sp_s_pc_dn19 = assign42460_e55829_d_n19;
        locals.var_sp_s_pc_dn20 = assign42460_e55829_d_n20;
        locals.var_sp_s_pc_rv = 0.0;

        let (assign42470_e55857, assign42470_e55857_d_n5, assign42470_e55857_d_n6, assign42470_e55857_d_n7, assign42470_e55857_d_n8, assign42470_e55857_d_n12, assign42470_e55857_d_n13, assign42470_e55857_d_n14, assign42470_e55857_d_n15, assign42470_e55857_d_n16, assign42470_e55857_d_n17, assign42470_e55857_d_n18, assign42470_e55857_d_n19, assign42470_e55857_d_n20,) = {
    if ((locals.var_guard1284 == 0.0) && (locals.var_guard1285 == 0.0)) {
        let assign42470_e55837: f64 = (locals.var_sp_s_temp * locals.var_sp_s_temp);
        let assign42470_e55841: f64 = (locals.var_sp_s_delta1 + locals.var_sp_s_x0);
        let assign42470_e55843: f64 = (assign42470_e55841 - 1.0);
        let assign42470_e55845: f64 = (assign42470_e55843 + locals.var_sp_s_delta0);
        let assign42470_e55849: f64 = (locals.var_sp_s_x0 + 1.0);
        let assign42470_e55851: f64 = (assign42470_e55849 + locals.var_sp_s_xi0);
        let assign42470_e55852: f64 = (locals.var_delta_ns * assign42470_e55851);
        let assign42470_e55853: f64 = (assign42470_e55845 - assign42470_e55852);
        let assign42470_e55854: f64 = (locals.var_gf2 * assign42470_e55853);
        let assign42470_e55855: f64 = (assign42470_e55837 - assign42470_e55854);
        (assign42470_e55855, (((locals.var_sp_s_temp_dn5 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn5)) - ((locals.var_gf2_dn5 * assign42470_e55853) + (locals.var_gf2 * (((locals.var_sp_s_delta1_dn5 + locals.var_sp_s_x0_dn5) + locals.var_sp_s_delta0_dn5) - ((locals.var_delta_ns_dn5 * assign42470_e55851) + (locals.var_delta_ns * (locals.var_sp_s_x0_dn5 + locals.var_sp_s_xi0_dn5))))))), (((locals.var_sp_s_temp_dn6 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn6)) - ((locals.var_gf2_dn6 * assign42470_e55853) + (locals.var_gf2 * (((locals.var_sp_s_delta1_dn6 + locals.var_sp_s_x0_dn6) + locals.var_sp_s_delta0_dn6) - ((locals.var_delta_ns_dn6 * assign42470_e55851) + (locals.var_delta_ns * (locals.var_sp_s_x0_dn6 + locals.var_sp_s_xi0_dn6))))))), (((locals.var_sp_s_temp_dn7 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn7)) - ((locals.var_gf2_dn7 * assign42470_e55853) + (locals.var_gf2 * (((locals.var_sp_s_delta1_dn7 + locals.var_sp_s_x0_dn7) + locals.var_sp_s_delta0_dn7) - ((locals.var_delta_ns_dn7 * assign42470_e55851) + (locals.var_delta_ns * (locals.var_sp_s_x0_dn7 + locals.var_sp_s_xi0_dn7))))))), (((locals.var_sp_s_temp_dn8 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn8)) - ((locals.var_gf2_dn8 * assign42470_e55853) + (locals.var_gf2 * (((locals.var_sp_s_delta1_dn8 + locals.var_sp_s_x0_dn8) + locals.var_sp_s_delta0_dn8) - ((locals.var_delta_ns_dn8 * assign42470_e55851) + (locals.var_delta_ns * (locals.var_sp_s_x0_dn8 + locals.var_sp_s_xi0_dn8))))))), (((locals.var_sp_s_temp_dn12 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn12)) - ((locals.var_gf2_dn12 * assign42470_e55853) + (locals.var_gf2 * (((locals.var_sp_s_delta1_dn12 + locals.var_sp_s_x0_dn12) + locals.var_sp_s_delta0_dn12) - ((locals.var_delta_ns_dn12 * assign42470_e55851) + (locals.var_delta_ns * (locals.var_sp_s_x0_dn12 + locals.var_sp_s_xi0_dn12))))))), (((locals.var_sp_s_temp_dn13 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn13)) - ((locals.var_gf2_dn13 * assign42470_e55853) + (locals.var_gf2 * (((locals.var_sp_s_delta1_dn13 + locals.var_sp_s_x0_dn13) + locals.var_sp_s_delta0_dn13) - ((locals.var_delta_ns_dn13 * assign42470_e55851) + (locals.var_delta_ns * (locals.var_sp_s_x0_dn13 + locals.var_sp_s_xi0_dn13))))))), (((locals.var_sp_s_temp_dn14 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn14)) - ((locals.var_gf2_dn14 * assign42470_e55853) + (locals.var_gf2 * (((locals.var_sp_s_delta1_dn14 + locals.var_sp_s_x0_dn14) + locals.var_sp_s_delta0_dn14) - ((locals.var_delta_ns_dn14 * assign42470_e55851) + (locals.var_delta_ns * (locals.var_sp_s_x0_dn14 + locals.var_sp_s_xi0_dn14))))))), (((locals.var_sp_s_temp_dn15 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn15)) - ((locals.var_gf2_dn15 * assign42470_e55853) + (locals.var_gf2 * (((locals.var_sp_s_delta1_dn15 + locals.var_sp_s_x0_dn15) + locals.var_sp_s_delta0_dn15) - ((locals.var_delta_ns_dn15 * assign42470_e55851) + (locals.var_delta_ns * (locals.var_sp_s_x0_dn15 + locals.var_sp_s_xi0_dn15))))))), (((locals.var_sp_s_temp_dn16 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn16)) - ((locals.var_gf2_dn16 * assign42470_e55853) + (locals.var_gf2 * (((locals.var_sp_s_delta1_dn16 + locals.var_sp_s_x0_dn16) + locals.var_sp_s_delta0_dn16) - ((locals.var_delta_ns_dn16 * assign42470_e55851) + (locals.var_delta_ns * (locals.var_sp_s_x0_dn16 + locals.var_sp_s_xi0_dn16))))))), (((locals.var_sp_s_temp_dn17 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn17)) - ((locals.var_gf2_dn17 * assign42470_e55853) + (locals.var_gf2 * (((locals.var_sp_s_delta1_dn17 + locals.var_sp_s_x0_dn17) + locals.var_sp_s_delta0_dn17) - ((locals.var_delta_ns_dn17 * assign42470_e55851) + (locals.var_delta_ns * (locals.var_sp_s_x0_dn17 + locals.var_sp_s_xi0_dn17))))))), (((locals.var_sp_s_temp_dn18 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn18)) - ((locals.var_gf2_dn18 * assign42470_e55853) + (locals.var_gf2 * (((locals.var_sp_s_delta1_dn18 + locals.var_sp_s_x0_dn18) + locals.var_sp_s_delta0_dn18) - ((locals.var_delta_ns_dn18 * assign42470_e55851) + (locals.var_delta_ns * (locals.var_sp_s_x0_dn18 + locals.var_sp_s_xi0_dn18))))))), (((locals.var_sp_s_temp_dn19 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn19)) - ((locals.var_gf2_dn19 * assign42470_e55853) + (locals.var_gf2 * (((locals.var_sp_s_delta1_dn19 + locals.var_sp_s_x0_dn19) + locals.var_sp_s_delta0_dn19) - ((locals.var_delta_ns_dn19 * assign42470_e55851) + (locals.var_delta_ns * (locals.var_sp_s_x0_dn19 + locals.var_sp_s_xi0_dn19))))))), (((locals.var_sp_s_temp_dn20 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn20)) - ((locals.var_gf2_dn20 * assign42470_e55853) + (locals.var_gf2 * (((locals.var_sp_s_delta1_dn20 + locals.var_sp_s_x0_dn20) + locals.var_sp_s_delta0_dn20) - ((locals.var_delta_ns_dn20 * assign42470_e55851) + (locals.var_delta_ns * (locals.var_sp_s_x0_dn20 + locals.var_sp_s_xi0_dn20))))))),)
    } else {
        (locals.var_sp_s_qc, locals.var_sp_s_qc_dn5, locals.var_sp_s_qc_dn6, locals.var_sp_s_qc_dn7, locals.var_sp_s_qc_dn8, locals.var_sp_s_qc_dn12, locals.var_sp_s_qc_dn13, locals.var_sp_s_qc_dn14, locals.var_sp_s_qc_dn15, locals.var_sp_s_qc_dn16, locals.var_sp_s_qc_dn17, locals.var_sp_s_qc_dn18, locals.var_sp_s_qc_dn19, locals.var_sp_s_qc_dn20,)
    }
};
        locals.var_sp_s_qc = assign42470_e55857;
        locals.var_sp_s_qc_dn5 = assign42470_e55857_d_n5;
        locals.var_sp_s_qc_dn6 = assign42470_e55857_d_n6;
        locals.var_sp_s_qc_dn7 = assign42470_e55857_d_n7;
        locals.var_sp_s_qc_dn8 = assign42470_e55857_d_n8;
        locals.var_sp_s_qc_dn12 = assign42470_e55857_d_n12;
        locals.var_sp_s_qc_dn13 = assign42470_e55857_d_n13;
        locals.var_sp_s_qc_dn14 = assign42470_e55857_d_n14;
        locals.var_sp_s_qc_dn15 = assign42470_e55857_d_n15;
        locals.var_sp_s_qc_dn16 = assign42470_e55857_d_n16;
        locals.var_sp_s_qc_dn17 = assign42470_e55857_d_n17;
        locals.var_sp_s_qc_dn18 = assign42470_e55857_d_n18;
        locals.var_sp_s_qc_dn19 = assign42470_e55857_d_n19;
        locals.var_sp_s_qc_dn20 = assign42470_e55857_d_n20;
        locals.var_sp_s_qc_rv = 0.0;

        let (assign42480_e55875, assign42480_e55875_d_n5, assign42480_e55875_d_n6, assign42480_e55875_d_n7, assign42480_e55875_d_n8, assign42480_e55875_d_n12, assign42480_e55875_d_n13, assign42480_e55875_d_n14, assign42480_e55875_d_n15, assign42480_e55875_d_n16, assign42480_e55875_d_n17, assign42480_e55875_d_n18, assign42480_e55875_d_n19, assign42480_e55875_d_n20,) = {
    if ((locals.var_guard1284 == 0.0) && (locals.var_guard1285 == 0.0)) {
        let assign42480_e55867: f64 = (locals.var_sp_s_delta1 + locals.var_sp_s_delta0);
        let assign42480_e55870: f64 = (locals.var_delta_ns * locals.var_sp_s_xi2);
        let assign42480_e55871: f64 = (assign42480_e55867 - assign42480_e55870);
        let assign42480_e55872: f64 = (locals.var_gf2 * assign42480_e55871);
        let assign42480_e55873: f64 = (2.0 - assign42480_e55872);
        (assign42480_e55873, (-((locals.var_gf2_dn5 * assign42480_e55871) + (locals.var_gf2 * ((locals.var_sp_s_delta1_dn5 + locals.var_sp_s_delta0_dn5) - ((locals.var_delta_ns_dn5 * locals.var_sp_s_xi2) + (locals.var_delta_ns * locals.var_sp_s_xi2_dn5)))))), (-((locals.var_gf2_dn6 * assign42480_e55871) + (locals.var_gf2 * ((locals.var_sp_s_delta1_dn6 + locals.var_sp_s_delta0_dn6) - ((locals.var_delta_ns_dn6 * locals.var_sp_s_xi2) + (locals.var_delta_ns * locals.var_sp_s_xi2_dn6)))))), (-((locals.var_gf2_dn7 * assign42480_e55871) + (locals.var_gf2 * ((locals.var_sp_s_delta1_dn7 + locals.var_sp_s_delta0_dn7) - ((locals.var_delta_ns_dn7 * locals.var_sp_s_xi2) + (locals.var_delta_ns * locals.var_sp_s_xi2_dn7)))))), (-((locals.var_gf2_dn8 * assign42480_e55871) + (locals.var_gf2 * ((locals.var_sp_s_delta1_dn8 + locals.var_sp_s_delta0_dn8) - ((locals.var_delta_ns_dn8 * locals.var_sp_s_xi2) + (locals.var_delta_ns * locals.var_sp_s_xi2_dn8)))))), (-((locals.var_gf2_dn12 * assign42480_e55871) + (locals.var_gf2 * ((locals.var_sp_s_delta1_dn12 + locals.var_sp_s_delta0_dn12) - ((locals.var_delta_ns_dn12 * locals.var_sp_s_xi2) + (locals.var_delta_ns * locals.var_sp_s_xi2_dn12)))))), (-((locals.var_gf2_dn13 * assign42480_e55871) + (locals.var_gf2 * ((locals.var_sp_s_delta1_dn13 + locals.var_sp_s_delta0_dn13) - ((locals.var_delta_ns_dn13 * locals.var_sp_s_xi2) + (locals.var_delta_ns * locals.var_sp_s_xi2_dn13)))))), (-((locals.var_gf2_dn14 * assign42480_e55871) + (locals.var_gf2 * ((locals.var_sp_s_delta1_dn14 + locals.var_sp_s_delta0_dn14) - ((locals.var_delta_ns_dn14 * locals.var_sp_s_xi2) + (locals.var_delta_ns * locals.var_sp_s_xi2_dn14)))))), (-((locals.var_gf2_dn15 * assign42480_e55871) + (locals.var_gf2 * ((locals.var_sp_s_delta1_dn15 + locals.var_sp_s_delta0_dn15) - ((locals.var_delta_ns_dn15 * locals.var_sp_s_xi2) + (locals.var_delta_ns * locals.var_sp_s_xi2_dn15)))))), (-((locals.var_gf2_dn16 * assign42480_e55871) + (locals.var_gf2 * ((locals.var_sp_s_delta1_dn16 + locals.var_sp_s_delta0_dn16) - ((locals.var_delta_ns_dn16 * locals.var_sp_s_xi2) + (locals.var_delta_ns * locals.var_sp_s_xi2_dn16)))))), (-((locals.var_gf2_dn17 * assign42480_e55871) + (locals.var_gf2 * ((locals.var_sp_s_delta1_dn17 + locals.var_sp_s_delta0_dn17) - ((locals.var_delta_ns_dn17 * locals.var_sp_s_xi2) + (locals.var_delta_ns * locals.var_sp_s_xi2_dn17)))))), (-((locals.var_gf2_dn18 * assign42480_e55871) + (locals.var_gf2 * ((locals.var_sp_s_delta1_dn18 + locals.var_sp_s_delta0_dn18) - ((locals.var_delta_ns_dn18 * locals.var_sp_s_xi2) + (locals.var_delta_ns * locals.var_sp_s_xi2_dn18)))))), (-((locals.var_gf2_dn19 * assign42480_e55871) + (locals.var_gf2 * ((locals.var_sp_s_delta1_dn19 + locals.var_sp_s_delta0_dn19) - ((locals.var_delta_ns_dn19 * locals.var_sp_s_xi2) + (locals.var_delta_ns * locals.var_sp_s_xi2_dn19)))))), (-((locals.var_gf2_dn20 * assign42480_e55871) + (locals.var_gf2 * ((locals.var_sp_s_delta1_dn20 + locals.var_sp_s_delta0_dn20) - ((locals.var_delta_ns_dn20 * locals.var_sp_s_xi2) + (locals.var_delta_ns * locals.var_sp_s_xi2_dn20)))))),)
    } else {
        (locals.var_sp_s_temp, locals.var_sp_s_temp_dn5, locals.var_sp_s_temp_dn6, locals.var_sp_s_temp_dn7, locals.var_sp_s_temp_dn8, locals.var_sp_s_temp_dn12, locals.var_sp_s_temp_dn13, locals.var_sp_s_temp_dn14, locals.var_sp_s_temp_dn15, locals.var_sp_s_temp_dn16, locals.var_sp_s_temp_dn17, locals.var_sp_s_temp_dn18, locals.var_sp_s_temp_dn19, locals.var_sp_s_temp_dn20,)
    }
};
        locals.var_sp_s_temp = assign42480_e55875;
        locals.var_sp_s_temp_dn5 = assign42480_e55875_d_n5;
        locals.var_sp_s_temp_dn6 = assign42480_e55875_d_n6;
        locals.var_sp_s_temp_dn7 = assign42480_e55875_d_n7;
        locals.var_sp_s_temp_dn8 = assign42480_e55875_d_n8;
        locals.var_sp_s_temp_dn12 = assign42480_e55875_d_n12;
        locals.var_sp_s_temp_dn13 = assign42480_e55875_d_n13;
        locals.var_sp_s_temp_dn14 = assign42480_e55875_d_n14;
        locals.var_sp_s_temp_dn15 = assign42480_e55875_d_n15;
        locals.var_sp_s_temp_dn16 = assign42480_e55875_d_n16;
        locals.var_sp_s_temp_dn17 = assign42480_e55875_d_n17;
        locals.var_sp_s_temp_dn18 = assign42480_e55875_d_n18;
        locals.var_sp_s_temp_dn19 = assign42480_e55875_d_n19;
        locals.var_sp_s_temp_dn20 = assign42480_e55875_d_n20;
        locals.var_sp_s_temp_rv = 0.0;

        let (assign42490_e55891, assign42490_e55891_d_n5, assign42490_e55891_d_n6, assign42490_e55891_d_n7, assign42490_e55891_d_n8, assign42490_e55891_d_n12, assign42490_e55891_d_n13, assign42490_e55891_d_n14, assign42490_e55891_d_n15, assign42490_e55891_d_n16, assign42490_e55891_d_n17, assign42490_e55891_d_n18, assign42490_e55891_d_n19, assign42490_e55891_d_n20,) = {
    if ((locals.var_guard1284 == 0.0) && (locals.var_guard1285 == 0.0)) {
        let assign42490_e55883: f64 = (locals.var_sp_s_pc * locals.var_sp_s_pc);
        let assign42490_e55887: f64 = (locals.var_sp_s_qc * locals.var_sp_s_temp);
        let assign42490_e55888: f64 = (2.0 * assign42490_e55887);
        let assign42490_e55889: f64 = (assign42490_e55883 - assign42490_e55888);
        (assign42490_e55889, (((locals.var_sp_s_pc_dn5 * locals.var_sp_s_pc) + (locals.var_sp_s_pc * locals.var_sp_s_pc_dn5)) - (2.0 * ((locals.var_sp_s_qc_dn5 * locals.var_sp_s_temp) + (locals.var_sp_s_qc * locals.var_sp_s_temp_dn5)))), (((locals.var_sp_s_pc_dn6 * locals.var_sp_s_pc) + (locals.var_sp_s_pc * locals.var_sp_s_pc_dn6)) - (2.0 * ((locals.var_sp_s_qc_dn6 * locals.var_sp_s_temp) + (locals.var_sp_s_qc * locals.var_sp_s_temp_dn6)))), (((locals.var_sp_s_pc_dn7 * locals.var_sp_s_pc) + (locals.var_sp_s_pc * locals.var_sp_s_pc_dn7)) - (2.0 * ((locals.var_sp_s_qc_dn7 * locals.var_sp_s_temp) + (locals.var_sp_s_qc * locals.var_sp_s_temp_dn7)))), (((locals.var_sp_s_pc_dn8 * locals.var_sp_s_pc) + (locals.var_sp_s_pc * locals.var_sp_s_pc_dn8)) - (2.0 * ((locals.var_sp_s_qc_dn8 * locals.var_sp_s_temp) + (locals.var_sp_s_qc * locals.var_sp_s_temp_dn8)))), (((locals.var_sp_s_pc_dn12 * locals.var_sp_s_pc) + (locals.var_sp_s_pc * locals.var_sp_s_pc_dn12)) - (2.0 * ((locals.var_sp_s_qc_dn12 * locals.var_sp_s_temp) + (locals.var_sp_s_qc * locals.var_sp_s_temp_dn12)))), (((locals.var_sp_s_pc_dn13 * locals.var_sp_s_pc) + (locals.var_sp_s_pc * locals.var_sp_s_pc_dn13)) - (2.0 * ((locals.var_sp_s_qc_dn13 * locals.var_sp_s_temp) + (locals.var_sp_s_qc * locals.var_sp_s_temp_dn13)))), (((locals.var_sp_s_pc_dn14 * locals.var_sp_s_pc) + (locals.var_sp_s_pc * locals.var_sp_s_pc_dn14)) - (2.0 * ((locals.var_sp_s_qc_dn14 * locals.var_sp_s_temp) + (locals.var_sp_s_qc * locals.var_sp_s_temp_dn14)))), (((locals.var_sp_s_pc_dn15 * locals.var_sp_s_pc) + (locals.var_sp_s_pc * locals.var_sp_s_pc_dn15)) - (2.0 * ((locals.var_sp_s_qc_dn15 * locals.var_sp_s_temp) + (locals.var_sp_s_qc * locals.var_sp_s_temp_dn15)))), (((locals.var_sp_s_pc_dn16 * locals.var_sp_s_pc) + (locals.var_sp_s_pc * locals.var_sp_s_pc_dn16)) - (2.0 * ((locals.var_sp_s_qc_dn16 * locals.var_sp_s_temp) + (locals.var_sp_s_qc * locals.var_sp_s_temp_dn16)))), (((locals.var_sp_s_pc_dn17 * locals.var_sp_s_pc) + (locals.var_sp_s_pc * locals.var_sp_s_pc_dn17)) - (2.0 * ((locals.var_sp_s_qc_dn17 * locals.var_sp_s_temp) + (locals.var_sp_s_qc * locals.var_sp_s_temp_dn17)))), (((locals.var_sp_s_pc_dn18 * locals.var_sp_s_pc) + (locals.var_sp_s_pc * locals.var_sp_s_pc_dn18)) - (2.0 * ((locals.var_sp_s_qc_dn18 * locals.var_sp_s_temp) + (locals.var_sp_s_qc * locals.var_sp_s_temp_dn18)))), (((locals.var_sp_s_pc_dn19 * locals.var_sp_s_pc) + (locals.var_sp_s_pc * locals.var_sp_s_pc_dn19)) - (2.0 * ((locals.var_sp_s_qc_dn19 * locals.var_sp_s_temp) + (locals.var_sp_s_qc * locals.var_sp_s_temp_dn19)))), (((locals.var_sp_s_pc_dn20 * locals.var_sp_s_pc) + (locals.var_sp_s_pc * locals.var_sp_s_pc_dn20)) - (2.0 * ((locals.var_sp_s_qc_dn20 * locals.var_sp_s_temp) + (locals.var_sp_s_qc * locals.var_sp_s_temp_dn20)))),)
    } else {
        (locals.var_sp_s_temp, locals.var_sp_s_temp_dn5, locals.var_sp_s_temp_dn6, locals.var_sp_s_temp_dn7, locals.var_sp_s_temp_dn8, locals.var_sp_s_temp_dn12, locals.var_sp_s_temp_dn13, locals.var_sp_s_temp_dn14, locals.var_sp_s_temp_dn15, locals.var_sp_s_temp_dn16, locals.var_sp_s_temp_dn17, locals.var_sp_s_temp_dn18, locals.var_sp_s_temp_dn19, locals.var_sp_s_temp_dn20,)
    }
};
        locals.var_sp_s_temp = assign42490_e55891;
        locals.var_sp_s_temp_dn5 = assign42490_e55891_d_n5;
        locals.var_sp_s_temp_dn6 = assign42490_e55891_d_n6;
        locals.var_sp_s_temp_dn7 = assign42490_e55891_d_n7;
        locals.var_sp_s_temp_dn8 = assign42490_e55891_d_n8;
        locals.var_sp_s_temp_dn12 = assign42490_e55891_d_n12;
        locals.var_sp_s_temp_dn13 = assign42490_e55891_d_n13;
        locals.var_sp_s_temp_dn14 = assign42490_e55891_d_n14;
        locals.var_sp_s_temp_dn15 = assign42490_e55891_d_n15;
        locals.var_sp_s_temp_dn16 = assign42490_e55891_d_n16;
        locals.var_sp_s_temp_dn17 = assign42490_e55891_d_n17;
        locals.var_sp_s_temp_dn18 = assign42490_e55891_d_n18;
        locals.var_sp_s_temp_dn19 = assign42490_e55891_d_n19;
        locals.var_sp_s_temp_dn20 = assign42490_e55891_d_n20;
        locals.var_sp_s_temp_rv = 0.0;

        let (assign42500_e55908, assign42500_e55908_d_n5, assign42500_e55908_d_n6, assign42500_e55908_d_n7, assign42500_e55908_d_n8, assign42500_e55908_d_n12, assign42500_e55908_d_n13, assign42500_e55908_d_n14, assign42500_e55908_d_n15, assign42500_e55908_d_n16, assign42500_e55908_d_n17, assign42500_e55908_d_n18, assign42500_e55908_d_n19, assign42500_e55908_d_n20,) = {
    if ((locals.var_guard1284 == 0.0) && (locals.var_guard1285 == 0.0)) {
        let assign42500_e55902: f64 = (locals.var_sp_s_temp).sqrt();
        let assign42500_e55903: f64 = (locals.var_sp_s_pc + assign42500_e55902);
        let assign42500_e55904: f64 = (locals.var_sp_s_qc / assign42500_e55903);
        let assign42500_e55905: f64 = (2.0 * assign42500_e55904);
        let assign42500_e55906: f64 = (locals.var_sp_s_x0 + assign42500_e55905);
        (assign42500_e55906, (locals.var_sp_s_x0_dn5 + (2.0 * (((locals.var_sp_s_qc_dn5 * assign42500_e55903) - (locals.var_sp_s_qc * (locals.var_sp_s_pc_dn5 + (locals.var_sp_s_temp_dn5 / (2.0 * assign42500_e55902))))) / (assign42500_e55903 * assign42500_e55903)))), (locals.var_sp_s_x0_dn6 + (2.0 * (((locals.var_sp_s_qc_dn6 * assign42500_e55903) - (locals.var_sp_s_qc * (locals.var_sp_s_pc_dn6 + (locals.var_sp_s_temp_dn6 / (2.0 * assign42500_e55902))))) / (assign42500_e55903 * assign42500_e55903)))), (locals.var_sp_s_x0_dn7 + (2.0 * (((locals.var_sp_s_qc_dn7 * assign42500_e55903) - (locals.var_sp_s_qc * (locals.var_sp_s_pc_dn7 + (locals.var_sp_s_temp_dn7 / (2.0 * assign42500_e55902))))) / (assign42500_e55903 * assign42500_e55903)))), (locals.var_sp_s_x0_dn8 + (2.0 * (((locals.var_sp_s_qc_dn8 * assign42500_e55903) - (locals.var_sp_s_qc * (locals.var_sp_s_pc_dn8 + (locals.var_sp_s_temp_dn8 / (2.0 * assign42500_e55902))))) / (assign42500_e55903 * assign42500_e55903)))), (locals.var_sp_s_x0_dn12 + (2.0 * (((locals.var_sp_s_qc_dn12 * assign42500_e55903) - (locals.var_sp_s_qc * (locals.var_sp_s_pc_dn12 + (locals.var_sp_s_temp_dn12 / (2.0 * assign42500_e55902))))) / (assign42500_e55903 * assign42500_e55903)))), (locals.var_sp_s_x0_dn13 + (2.0 * (((locals.var_sp_s_qc_dn13 * assign42500_e55903) - (locals.var_sp_s_qc * (locals.var_sp_s_pc_dn13 + (locals.var_sp_s_temp_dn13 / (2.0 * assign42500_e55902))))) / (assign42500_e55903 * assign42500_e55903)))), (locals.var_sp_s_x0_dn14 + (2.0 * (((locals.var_sp_s_qc_dn14 * assign42500_e55903) - (locals.var_sp_s_qc * (locals.var_sp_s_pc_dn14 + (locals.var_sp_s_temp_dn14 / (2.0 * assign42500_e55902))))) / (assign42500_e55903 * assign42500_e55903)))), (locals.var_sp_s_x0_dn15 + (2.0 * (((locals.var_sp_s_qc_dn15 * assign42500_e55903) - (locals.var_sp_s_qc * (locals.var_sp_s_pc_dn15 + (locals.var_sp_s_temp_dn15 / (2.0 * assign42500_e55902))))) / (assign42500_e55903 * assign42500_e55903)))), (locals.var_sp_s_x0_dn16 + (2.0 * (((locals.var_sp_s_qc_dn16 * assign42500_e55903) - (locals.var_sp_s_qc * (locals.var_sp_s_pc_dn16 + (locals.var_sp_s_temp_dn16 / (2.0 * assign42500_e55902))))) / (assign42500_e55903 * assign42500_e55903)))), (locals.var_sp_s_x0_dn17 + (2.0 * (((locals.var_sp_s_qc_dn17 * assign42500_e55903) - (locals.var_sp_s_qc * (locals.var_sp_s_pc_dn17 + (locals.var_sp_s_temp_dn17 / (2.0 * assign42500_e55902))))) / (assign42500_e55903 * assign42500_e55903)))), (locals.var_sp_s_x0_dn18 + (2.0 * (((locals.var_sp_s_qc_dn18 * assign42500_e55903) - (locals.var_sp_s_qc * (locals.var_sp_s_pc_dn18 + (locals.var_sp_s_temp_dn18 / (2.0 * assign42500_e55902))))) / (assign42500_e55903 * assign42500_e55903)))), (locals.var_sp_s_x0_dn19 + (2.0 * (((locals.var_sp_s_qc_dn19 * assign42500_e55903) - (locals.var_sp_s_qc * (locals.var_sp_s_pc_dn19 + (locals.var_sp_s_temp_dn19 / (2.0 * assign42500_e55902))))) / (assign42500_e55903 * assign42500_e55903)))), (locals.var_sp_s_x0_dn20 + (2.0 * (((locals.var_sp_s_qc_dn20 * assign42500_e55903) - (locals.var_sp_s_qc * (locals.var_sp_s_pc_dn20 + (locals.var_sp_s_temp_dn20 / (2.0 * assign42500_e55902))))) / (assign42500_e55903 * assign42500_e55903)))),)
    } else {
        (locals.var_x_s, locals.var_x_s_dn5, locals.var_x_s_dn6, locals.var_x_s_dn7, locals.var_x_s_dn8, locals.var_x_s_dn12, locals.var_x_s_dn13, locals.var_x_s_dn14, locals.var_x_s_dn15, locals.var_x_s_dn16, locals.var_x_s_dn17, locals.var_x_s_dn18, locals.var_x_s_dn19, locals.var_x_s_dn20,)
    }
};
        locals.var_x_s = assign42500_e55908;
        locals.var_x_s_dn5 = assign42500_e55908_d_n5;
        locals.var_x_s_dn6 = assign42500_e55908_d_n6;
        locals.var_x_s_dn7 = assign42500_e55908_d_n7;
        locals.var_x_s_dn8 = assign42500_e55908_d_n8;
        locals.var_x_s_dn12 = assign42500_e55908_d_n12;
        locals.var_x_s_dn13 = assign42500_e55908_d_n13;
        locals.var_x_s_dn14 = assign42500_e55908_d_n14;
        locals.var_x_s_dn15 = assign42500_e55908_d_n15;
        locals.var_x_s_dn16 = assign42500_e55908_d_n16;
        locals.var_x_s_dn17 = assign42500_e55908_d_n17;
        locals.var_x_s_dn18 = assign42500_e55908_d_n18;
        locals.var_x_s_dn19 = assign42500_e55908_d_n19;
        locals.var_x_s_dn20 = assign42500_e55908_d_n20;
        locals.var_x_s_rv = 0.0;

        locals.var_xi1s = 0.0;
        locals.var_xi1s_dn5 = 0.0;
        locals.var_xi1s_dn6 = 0.0;
        locals.var_xi1s_dn7 = 0.0;
        locals.var_xi1s_dn8 = 0.0;
        locals.var_xi1s_dn12 = 0.0;
        locals.var_xi1s_dn13 = 0.0;
        locals.var_xi1s_dn14 = 0.0;
        locals.var_xi1s_dn15 = 0.0;
        locals.var_xi1s_dn16 = 0.0;
        locals.var_xi1s_dn17 = 0.0;
        locals.var_xi1s_dn18 = 0.0;
        locals.var_xi1s_dn19 = 0.0;
        locals.var_xi1s_dn20 = 0.0;
        locals.var_xi1s_rv = 0.0;

        locals.var_xi2s = 0.0;
        locals.var_xi2s_dn5 = 0.0;
        locals.var_xi2s_dn6 = 0.0;
        locals.var_xi2s_dn7 = 0.0;
        locals.var_xi2s_dn8 = 0.0;
        locals.var_xi2s_dn12 = 0.0;
        locals.var_xi2s_dn13 = 0.0;
        locals.var_xi2s_dn14 = 0.0;
        locals.var_xi2s_dn15 = 0.0;
        locals.var_xi2s_dn16 = 0.0;
        locals.var_xi2s_dn17 = 0.0;
        locals.var_xi2s_dn18 = 0.0;
        locals.var_xi2s_dn19 = 0.0;
        locals.var_xi2s_dn20 = 0.0;
        locals.var_xi2s_rv = 0.0;

        locals.var_delta_1s = 0.0;
        locals.var_delta_1s_dn5 = 0.0;
        locals.var_delta_1s_dn6 = 0.0;
        locals.var_delta_1s_dn7 = 0.0;
        locals.var_delta_1s_dn8 = 0.0;
        locals.var_delta_1s_dn12 = 0.0;
        locals.var_delta_1s_dn13 = 0.0;
        locals.var_delta_1s_dn14 = 0.0;
        locals.var_delta_1s_dn15 = 0.0;
        locals.var_delta_1s_dn16 = 0.0;
        locals.var_delta_1s_dn17 = 0.0;
        locals.var_delta_1s_dn18 = 0.0;
        locals.var_delta_1s_dn19 = 0.0;
        locals.var_delta_1s_dn20 = 0.0;
        locals.var_delta_1s_rv = 0.0;

        locals.var_es = 0.0;
        locals.var_es_dn5 = 0.0;
        locals.var_es_dn6 = 0.0;
        locals.var_es_dn7 = 0.0;
        locals.var_es_dn8 = 0.0;
        locals.var_es_dn12 = 0.0;
        locals.var_es_dn13 = 0.0;
        locals.var_es_dn14 = 0.0;
        locals.var_es_dn15 = 0.0;
        locals.var_es_dn16 = 0.0;
        locals.var_es_dn17 = 0.0;
        locals.var_es_dn18 = 0.0;
        locals.var_es_dn19 = 0.0;
        locals.var_es_dn20 = 0.0;
        locals.var_es_rv = 0.0;

        locals.var_ds = 0.0;
        locals.var_ds_dn5 = 0.0;
        locals.var_ds_dn6 = 0.0;
        locals.var_ds_dn7 = 0.0;
        locals.var_ds_dn8 = 0.0;
        locals.var_ds_dn12 = 0.0;
        locals.var_ds_dn13 = 0.0;
        locals.var_ds_dn14 = 0.0;
        locals.var_ds_dn15 = 0.0;
        locals.var_ds_dn16 = 0.0;
        locals.var_ds_dn17 = 0.0;
        locals.var_ds_dn18 = 0.0;
        locals.var_ds_dn19 = 0.0;
        locals.var_ds_dn20 = 0.0;
        locals.var_ds_rv = 0.0;

        locals.var_ps = 0.0;
        locals.var_ps_dn5 = 0.0;
        locals.var_ps_dn6 = 0.0;
        locals.var_ps_dn7 = 0.0;
        locals.var_ps_dn8 = 0.0;
        locals.var_ps_dn12 = 0.0;
        locals.var_ps_dn13 = 0.0;
        locals.var_ps_dn14 = 0.0;
        locals.var_ps_dn15 = 0.0;
        locals.var_ps_dn16 = 0.0;
        locals.var_ps_dn17 = 0.0;
        locals.var_ps_dn18 = 0.0;
        locals.var_ps_dn19 = 0.0;
        locals.var_ps_dn20 = 0.0;
        locals.var_ps_rv = 0.0;

        locals.var_sqs = 0.0;
        locals.var_sqs_dn5 = 0.0;
        locals.var_sqs_dn6 = 0.0;
        locals.var_sqs_dn7 = 0.0;
        locals.var_sqs_dn8 = 0.0;
        locals.var_sqs_dn12 = 0.0;
        locals.var_sqs_dn13 = 0.0;
        locals.var_sqs_dn14 = 0.0;
        locals.var_sqs_dn15 = 0.0;
        locals.var_sqs_dn16 = 0.0;
        locals.var_sqs_dn17 = 0.0;
        locals.var_sqs_dn18 = 0.0;
        locals.var_sqs_dn19 = 0.0;
        locals.var_sqs_dn20 = 0.0;
        locals.var_sqs_rv = 0.0;

        locals.var_alphas = 1.0;
        locals.var_alphas_dn5 = 0.0;
        locals.var_alphas_dn6 = 0.0;
        locals.var_alphas_dn7 = 0.0;
        locals.var_alphas_dn8 = 0.0;
        locals.var_alphas_dn12 = 0.0;
        locals.var_alphas_dn13 = 0.0;
        locals.var_alphas_dn14 = 0.0;
        locals.var_alphas_dn15 = 0.0;
        locals.var_alphas_dn16 = 0.0;
        locals.var_alphas_dn17 = 0.0;
        locals.var_alphas_dn18 = 0.0;
        locals.var_alphas_dn19 = 0.0;
        locals.var_alphas_dn20 = 0.0;
        locals.var_alphas_rv = 0.0;

        locals.var_rxcor = 1.0;
        locals.var_rxcor_dn5 = 0.0;
        locals.var_rxcor_dn6 = 0.0;
        locals.var_rxcor_dn7 = 0.0;
        locals.var_rxcor_dn8 = 0.0;
        locals.var_rxcor_dn12 = 0.0;
        locals.var_rxcor_dn13 = 0.0;
        locals.var_rxcor_dn14 = 0.0;
        locals.var_rxcor_dn15 = 0.0;
        locals.var_rxcor_dn16 = 0.0;
        locals.var_rxcor_dn17 = 0.0;
        locals.var_rxcor_dn18 = 0.0;
        locals.var_rxcor_dn19 = 0.0;
        locals.var_rxcor_dn20 = 0.0;
        locals.var_rxcor_rv = 0.0;

        let assign42600_e55920: f64 = (locals.var_xg - locals.var_x_s);
        locals.var_xgs = assign42600_e55920;
        locals.var_xgs_dn5 = (locals.var_xg_dn5 - locals.var_x_s_dn5);
        locals.var_xgs_dn6 = (locals.var_xg_dn6 - locals.var_x_s_dn6);
        locals.var_xgs_dn7 = (locals.var_xg_dn7 - locals.var_x_s_dn7);
        locals.var_xgs_dn8 = (locals.var_xg_dn8 - locals.var_x_s_dn8);
        locals.var_xgs_dn12 = (locals.var_xg_dn12 - locals.var_x_s_dn12);
        locals.var_xgs_dn13 = (locals.var_xg_dn13 - locals.var_x_s_dn13);
        locals.var_xgs_dn14 = (locals.var_xg_dn14 - locals.var_x_s_dn14);
        locals.var_xgs_dn15 = (locals.var_xg_dn15 - locals.var_x_s_dn15);
        locals.var_xgs_dn16 = (locals.var_xg_dn16 - locals.var_x_s_dn16);
        locals.var_xgs_dn17 = (locals.var_xg_dn17 - locals.var_x_s_dn17);
        locals.var_xgs_dn18 = (locals.var_xg_dn18 - locals.var_x_s_dn18);
        locals.var_xgs_dn19 = (locals.var_xg_dn19 - locals.var_x_s_dn19);
        locals.var_xgs_dn20 = (locals.var_xg_dn20 - locals.var_x_s_dn20);
        locals.var_xgs_rv = 0.0;

        locals.var_qis = 0.0;
        locals.var_qis_dn5 = 0.0;
        locals.var_qis_dn6 = 0.0;
        locals.var_qis_dn7 = 0.0;
        locals.var_qis_dn8 = 0.0;
        locals.var_qis_dn12 = 0.0;
        locals.var_qis_dn13 = 0.0;
        locals.var_qis_dn14 = 0.0;
        locals.var_qis_dn15 = 0.0;
        locals.var_qis_dn16 = 0.0;
        locals.var_qis_dn17 = 0.0;
        locals.var_qis_dn18 = 0.0;
        locals.var_qis_dn19 = 0.0;
        locals.var_qis_dn20 = 0.0;
        locals.var_qis_rv = 0.0;

        let assign42620_e55924: f64 = (locals.var_phit1 * locals.var_xgs);
        locals.var_qbs = assign42620_e55924;
        locals.var_qbs_dn5 = ((locals.var_phit1_dn5 * locals.var_xgs) + (locals.var_phit1 * locals.var_xgs_dn5));
        locals.var_qbs_dn6 = ((locals.var_phit1_dn6 * locals.var_xgs) + (locals.var_phit1 * locals.var_xgs_dn6));
        locals.var_qbs_dn7 = ((locals.var_phit1_dn7 * locals.var_xgs) + (locals.var_phit1 * locals.var_xgs_dn7));
        locals.var_qbs_dn8 = ((locals.var_phit1_dn8 * locals.var_xgs) + (locals.var_phit1 * locals.var_xgs_dn8));
        locals.var_qbs_dn12 = ((locals.var_phit1_dn12 * locals.var_xgs) + (locals.var_phit1 * locals.var_xgs_dn12));
        locals.var_qbs_dn13 = ((locals.var_phit1_dn13 * locals.var_xgs) + (locals.var_phit1 * locals.var_xgs_dn13));
        locals.var_qbs_dn14 = ((locals.var_phit1_dn14 * locals.var_xgs) + (locals.var_phit1 * locals.var_xgs_dn14));
        locals.var_qbs_dn15 = ((locals.var_phit1_dn15 * locals.var_xgs) + (locals.var_phit1 * locals.var_xgs_dn15));
        locals.var_qbs_dn16 = ((locals.var_phit1_dn16 * locals.var_xgs) + (locals.var_phit1 * locals.var_xgs_dn16));
        locals.var_qbs_dn17 = ((locals.var_phit1_dn17 * locals.var_xgs) + (locals.var_phit1 * locals.var_xgs_dn17));
        locals.var_qbs_dn18 = ((locals.var_phit1_dn18 * locals.var_xgs) + (locals.var_phit1 * locals.var_xgs_dn18));
        locals.var_qbs_dn19 = ((locals.var_phit1_dn19 * locals.var_xgs) + (locals.var_phit1 * locals.var_xgs_dn19));
        locals.var_qbs_dn20 = ((locals.var_phit1_dn20 * locals.var_xgs) + (locals.var_phit1 * locals.var_xgs_dn20));
        locals.var_qbs_rv = 0.0;

        locals.var_rhob = 1.0;
        locals.var_rhob_dn5 = 0.0;
        locals.var_rhob_dn6 = 0.0;
        locals.var_rhob_dn7 = 0.0;
        locals.var_rhob_dn8 = 0.0;
        locals.var_rhob_dn12 = 0.0;
        locals.var_rhob_dn13 = 0.0;
        locals.var_rhob_dn14 = 0.0;
        locals.var_rhob_dn15 = 0.0;
        locals.var_rhob_dn16 = 0.0;
        locals.var_rhob_dn17 = 0.0;
        locals.var_rhob_dn18 = 0.0;
        locals.var_rhob_dn19 = 0.0;
        locals.var_rhob_dn20 = 0.0;
        locals.var_rhob_rv = 0.0;

        locals.var_rhog = 1.0;
        locals.var_rhog_dn5 = 0.0;
        locals.var_rhog_dn6 = 0.0;
        locals.var_rhog_dn7 = 0.0;
        locals.var_rhog_dn8 = 0.0;
        locals.var_rhog_dn12 = 0.0;
        locals.var_rhog_dn13 = 0.0;
        locals.var_rhog_dn14 = 0.0;
        locals.var_rhog_dn15 = 0.0;
        locals.var_rhog_dn16 = 0.0;
        locals.var_rhog_dn17 = 0.0;
        locals.var_rhog_dn18 = 0.0;
        locals.var_rhog_dn19 = 0.0;
        locals.var_rhog_dn20 = 0.0;
        locals.var_rhog_rv = 0.0;

        locals.var_gmobs = 1.0;
        locals.var_gmobs_dn5 = 0.0;
        locals.var_gmobs_dn6 = 0.0;
        locals.var_gmobs_dn7 = 0.0;
        locals.var_gmobs_dn8 = 0.0;
        locals.var_gmobs_dn12 = 0.0;
        locals.var_gmobs_dn13 = 0.0;
        locals.var_gmobs_dn14 = 0.0;
        locals.var_gmobs_dn15 = 0.0;
        locals.var_gmobs_dn16 = 0.0;
        locals.var_gmobs_dn17 = 0.0;
        locals.var_gmobs_dn18 = 0.0;
        locals.var_gmobs_dn19 = 0.0;
        locals.var_gmobs_dn20 = 0.0;
        locals.var_gmobs_rv = 0.0;

        locals.var_xitsb = 1.0;
        locals.var_xitsb_dn5 = 0.0;
        locals.var_xitsb_dn6 = 0.0;
        locals.var_xitsb_dn7 = 0.0;
        locals.var_xitsb_dn8 = 0.0;
        locals.var_xitsb_dn12 = 0.0;
        locals.var_xitsb_dn13 = 0.0;
        locals.var_xitsb_dn14 = 0.0;
        locals.var_xitsb_dn15 = 0.0;
        locals.var_xitsb_dn16 = 0.0;
        locals.var_xitsb_dn17 = 0.0;
        locals.var_xitsb_dn18 = 0.0;
        locals.var_xitsb_dn19 = 0.0;
        locals.var_xitsb_dn20 = 0.0;
        locals.var_xitsb_rv = 0.0;

        locals.var_factheta = 1.0;
        locals.var_factheta_dn5 = 0.0;
        locals.var_factheta_dn6 = 0.0;
        locals.var_factheta_dn7 = 0.0;
        locals.var_factheta_dn8 = 0.0;
        locals.var_factheta_dn12 = 0.0;
        locals.var_factheta_dn13 = 0.0;
        locals.var_factheta_dn14 = 0.0;
        locals.var_factheta_dn15 = 0.0;
        locals.var_factheta_dn16 = 0.0;
        locals.var_factheta_dn17 = 0.0;
        locals.var_factheta_dn18 = 0.0;
        locals.var_factheta_dn19 = 0.0;
        locals.var_factheta_dn20 = 0.0;
        locals.var_factheta_rv = 0.0;

        let assign42680_e55932: f64 = if locals.var_xg > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1290 = assign42680_e55932;
        locals.var_guard1290_rv = 0.0;

        let (assign42690_e55942, assign42690_e55942_d_n5, assign42690_e55942_d_n6, assign42690_e55942_d_n7, assign42690_e55942_d_n8, assign42690_e55942_d_n12, assign42690_e55942_d_n13, assign42690_e55942_d_n14, assign42690_e55942_d_n15, assign42690_e55942_d_n16, assign42690_e55942_d_n17, assign42690_e55942_d_n18, assign42690_e55942_d_n19, assign42690_e55942_d_n20,) = {
    if (locals.var_guard1290 != 0.0) {
        let assign42690_e55938: f64 = (locals.var_x_s * locals.var_x_s);
        let assign42690_e55939: f64 = (2.0 + assign42690_e55938);
        let assign42690_e55940: f64 = (1.0 / assign42690_e55939);
        (assign42690_e55940, (-(((locals.var_x_s_dn5 * locals.var_x_s) + (locals.var_x_s * locals.var_x_s_dn5)) / (assign42690_e55939 * assign42690_e55939))), (-(((locals.var_x_s_dn6 * locals.var_x_s) + (locals.var_x_s * locals.var_x_s_dn6)) / (assign42690_e55939 * assign42690_e55939))), (-(((locals.var_x_s_dn7 * locals.var_x_s) + (locals.var_x_s * locals.var_x_s_dn7)) / (assign42690_e55939 * assign42690_e55939))), (-(((locals.var_x_s_dn8 * locals.var_x_s) + (locals.var_x_s * locals.var_x_s_dn8)) / (assign42690_e55939 * assign42690_e55939))), (-(((locals.var_x_s_dn12 * locals.var_x_s) + (locals.var_x_s * locals.var_x_s_dn12)) / (assign42690_e55939 * assign42690_e55939))), (-(((locals.var_x_s_dn13 * locals.var_x_s) + (locals.var_x_s * locals.var_x_s_dn13)) / (assign42690_e55939 * assign42690_e55939))), (-(((locals.var_x_s_dn14 * locals.var_x_s) + (locals.var_x_s * locals.var_x_s_dn14)) / (assign42690_e55939 * assign42690_e55939))), (-(((locals.var_x_s_dn15 * locals.var_x_s) + (locals.var_x_s * locals.var_x_s_dn15)) / (assign42690_e55939 * assign42690_e55939))), (-(((locals.var_x_s_dn16 * locals.var_x_s) + (locals.var_x_s * locals.var_x_s_dn16)) / (assign42690_e55939 * assign42690_e55939))), (-(((locals.var_x_s_dn17 * locals.var_x_s) + (locals.var_x_s * locals.var_x_s_dn17)) / (assign42690_e55939 * assign42690_e55939))), (-(((locals.var_x_s_dn18 * locals.var_x_s) + (locals.var_x_s * locals.var_x_s_dn18)) / (assign42690_e55939 * assign42690_e55939))), (-(((locals.var_x_s_dn19 * locals.var_x_s) + (locals.var_x_s * locals.var_x_s_dn19)) / (assign42690_e55939 * assign42690_e55939))), (-(((locals.var_x_s_dn20 * locals.var_x_s) + (locals.var_x_s * locals.var_x_s_dn20)) / (assign42690_e55939 * assign42690_e55939))),)
    } else {
        (locals.var_temp__blk1038, locals.var_temp__blk1038_dn5, locals.var_temp__blk1038_dn6, locals.var_temp__blk1038_dn7, locals.var_temp__blk1038_dn8, locals.var_temp__blk1038_dn12, locals.var_temp__blk1038_dn13, locals.var_temp__blk1038_dn14, locals.var_temp__blk1038_dn15, locals.var_temp__blk1038_dn16, locals.var_temp__blk1038_dn17, locals.var_temp__blk1038_dn18, locals.var_temp__blk1038_dn19, locals.var_temp__blk1038_dn20,)
    }
};
        locals.var_temp__blk1038 = assign42690_e55942;
        locals.var_temp__blk1038_dn5 = assign42690_e55942_d_n5;
        locals.var_temp__blk1038_dn6 = assign42690_e55942_d_n6;
        locals.var_temp__blk1038_dn7 = assign42690_e55942_d_n7;
        locals.var_temp__blk1038_dn8 = assign42690_e55942_d_n8;
        locals.var_temp__blk1038_dn12 = assign42690_e55942_d_n12;
        locals.var_temp__blk1038_dn13 = assign42690_e55942_d_n13;
        locals.var_temp__blk1038_dn14 = assign42690_e55942_d_n14;
        locals.var_temp__blk1038_dn15 = assign42690_e55942_d_n15;
        locals.var_temp__blk1038_dn16 = assign42690_e55942_d_n16;
        locals.var_temp__blk1038_dn17 = assign42690_e55942_d_n17;
        locals.var_temp__blk1038_dn18 = assign42690_e55942_d_n18;
        locals.var_temp__blk1038_dn19 = assign42690_e55942_d_n19;
        locals.var_temp__blk1038_dn20 = assign42690_e55942_d_n20;
        locals.var_temp__blk1038_rv = 0.0;

    }
}
