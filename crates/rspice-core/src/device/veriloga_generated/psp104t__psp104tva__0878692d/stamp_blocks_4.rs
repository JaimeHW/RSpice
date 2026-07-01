#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_8(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        locals: &mut StampLocals,
    ) {
        let (assign8440_e7804,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard109 != 0.0)) {
        (p.p599,)
    } else {
        (locals.var_plwparam_i,)
    }
};
        locals.var_plwparam_i = assign8440_e7804;
        locals.var_plwparam_i_rv = 0.0;

        let assign8450_e7806: f64 = if param_given[679] { 1.0 } else { 0.0 };
        let assign8450_e7808: f64 = if assign8450_e7806 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard113 = assign8450_e7808;
        locals.var_guard113_rv = 0.0;

        let (assign8460_e7816,) = {
    if (((locals.var_guard36 != 0.0) && (locals.var_guard109 != 0.0)) && (locals.var_guard113 != 0.0)) {
        (p.p679,)
    } else {
        (locals.var_plwparam_i,)
    }
};
        locals.var_plwparam_i = assign8460_e7816;
        locals.var_plwparam_i_rv = 0.0;

        let (assign8470_e7836,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard109 != 0.0)) {
        let assign8470_e7824: f64 = (locals.var_plparam_i * locals.var_ile);
        let assign8470_e7825: f64 = (locals.var_poparam_i + assign8470_e7824);
        let assign8470_e7828: f64 = (locals.var_pwparam_i * locals.var_iwe);
        let assign8470_e7829: f64 = (assign8470_e7825 + assign8470_e7828);
        let assign8470_e7832: f64 = (locals.var_plwparam_i * locals.var_iae);
        let assign8470_e7833: f64 = (assign8470_e7829 + assign8470_e7832);
        let assign8470_e7834: f64 = assign8470_e7833;
        (assign8470_e7834,)
    } else {
        (locals.var_axac_p,)
    }
};
        locals.var_axac_p = assign8470_e7836;
        locals.var_axac_p_rv = 0.0;

        let assign8480_e7855: f64 = if (((param_given[680] || param_given[681]) || param_given[682]) || param_given[683]) { 1.0 } else { 0.0 };
        locals.var_guard114 = assign8480_e7855;
        locals.var_guard114_rv = 0.0;

        let (assign8490_e7875,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard114 != 0.0)) {
        let assign8490_e7863: f64 = (p.p681 * locals.var_ile);
        let assign8490_e7864: f64 = (p.p680 + assign8490_e7863);
        let assign8490_e7867: f64 = (p.p682 * locals.var_iwe);
        let assign8490_e7868: f64 = (assign8490_e7864 + assign8490_e7867);
        let assign8490_e7871: f64 = (p.p683 * locals.var_iae);
        let assign8490_e7872: f64 = (assign8490_e7868 + assign8490_e7871);
        let assign8490_e7873: f64 = (locals.var_ile * assign8490_e7872);
        (assign8490_e7873,)
    } else {
        (locals.var_alpac_p,)
    }
};
        locals.var_alpac_p = assign8490_e7875;
        locals.var_alpac_p_rv = 0.0;

        let assign8500_e7894: f64 = if (((param_given[684] || param_given[685]) || param_given[686]) || param_given[687]) { 1.0 } else { 0.0 };
        locals.var_guard115 = assign8500_e7894;
        locals.var_guard115_rv = 0.0;

        let (assign8510_e7914,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard115 != 0.0)) {
        let assign8510_e7902: f64 = (p.p685 * locals.var_ile);
        let assign8510_e7903: f64 = (p.p684 + assign8510_e7902);
        let assign8510_e7906: f64 = (p.p686 * locals.var_iwe);
        let assign8510_e7907: f64 = (assign8510_e7903 + assign8510_e7906);
        let assign8510_e7910: f64 = (p.p687 * locals.var_iae);
        let assign8510_e7911: f64 = (assign8510_e7907 + assign8510_e7910);
        let assign8510_e7912: f64 = (locals.var_ile * assign8510_e7911);
        (assign8510_e7912,)
    } else {
        (locals.var_alp1ac_p,)
    }
};
        locals.var_alp1ac_p = assign8510_e7914;
        locals.var_alp1ac_p_rv = 0.0;

        let assign8520_e7933: f64 = if (((param_given[688] || param_given[689]) || param_given[690]) || param_given[691]) { 1.0 } else { 0.0 };
        locals.var_guard116 = assign8520_e7933;
        locals.var_guard116_rv = 0.0;

        let (assign8530_e7953,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard116 != 0.0)) {
        let assign8530_e7941: f64 = (p.p689 * locals.var_ile);
        let assign8530_e7942: f64 = (p.p688 + assign8530_e7941);
        let assign8530_e7945: f64 = (p.p690 * locals.var_iwe);
        let assign8530_e7946: f64 = (assign8530_e7942 + assign8530_e7945);
        let assign8530_e7949: f64 = (p.p691 * locals.var_iae);
        let assign8530_e7950: f64 = (assign8530_e7946 + assign8530_e7949);
        let assign8530_e7951: f64 = (locals.var_iiwecv * assign8530_e7950);
        (assign8530_e7951,)
    } else {
        (locals.var_cgov_p,)
    }
};
        locals.var_cgov_p = assign8530_e7953;
        locals.var_cgov_p_rv = 0.0;

        let assign8540_e7972: f64 = if (((param_given[692] || param_given[693]) || param_given[694]) || param_given[695]) { 1.0 } else { 0.0 };
        locals.var_guard117 = assign8540_e7972;
        locals.var_guard117_rv = 0.0;

        let (assign8550_e7992,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard117 != 0.0)) {
        let assign8550_e7980: f64 = (p.p693 * locals.var_ile);
        let assign8550_e7981: f64 = (p.p692 + assign8550_e7980);
        let assign8550_e7984: f64 = (p.p694 * locals.var_iwe);
        let assign8550_e7985: f64 = (assign8550_e7981 + assign8550_e7984);
        let assign8550_e7988: f64 = (p.p695 * locals.var_iae);
        let assign8550_e7989: f64 = (assign8550_e7985 + assign8550_e7988);
        let assign8550_e7990: f64 = (locals.var_iiwecv * assign8550_e7989);
        (assign8550_e7990,)
    } else {
        (locals.var_cgovd_p,)
    }
};
        locals.var_cgovd_p = assign8550_e7992;
        locals.var_cgovd_p_rv = 0.0;

        let assign8560_e8011: f64 = if (((param_given[696] || param_given[697]) || param_given[698]) || param_given[699]) { 1.0 } else { 0.0 };
        locals.var_guard118 = assign8560_e8011;
        locals.var_guard118_rv = 0.0;

        let (assign8570_e8031,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard118 != 0.0)) {
        let assign8570_e8019: f64 = (p.p697 * locals.var_ile);
        let assign8570_e8020: f64 = (p.p696 + assign8570_e8019);
        let assign8570_e8023: f64 = (p.p698 * locals.var_iwe);
        let assign8570_e8024: f64 = (assign8570_e8020 + assign8570_e8023);
        let assign8570_e8027: f64 = (p.p699 * locals.var_iae);
        let assign8570_e8028: f64 = (assign8570_e8024 + assign8570_e8027);
        let assign8570_e8029: f64 = (locals.var_iilcv * assign8570_e8028);
        (assign8570_e8029,)
    } else {
        (locals.var_cgbov_p,)
    }
};
        locals.var_cgbov_p = assign8570_e8031;
        locals.var_cgbov_p_rv = 0.0;

        let assign8580_e8050: f64 = if (((param_given[700] || param_given[701]) || param_given[702]) || param_given[703]) { 1.0 } else { 0.0 };
        locals.var_guard119 = assign8580_e8050;
        locals.var_guard119_rv = 0.0;

        let (assign8590_e8070,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard119 != 0.0)) {
        let assign8590_e8058: f64 = (p.p701 * locals.var_ile);
        let assign8590_e8059: f64 = (p.p700 + assign8590_e8058);
        let assign8590_e8062: f64 = (p.p702 * locals.var_iwe);
        let assign8590_e8063: f64 = (assign8590_e8059 + assign8590_e8062);
        let assign8590_e8066: f64 = (p.p703 * locals.var_iae);
        let assign8590_e8067: f64 = (assign8590_e8063 + assign8590_e8066);
        let assign8590_e8068: f64 = (locals.var_iiwecv * assign8590_e8067);
        (assign8590_e8068,)
    } else {
        (locals.var_cinr_p,)
    }
};
        locals.var_cinr_p = assign8590_e8070;
        locals.var_cinr_p_rv = 0.0;

        let assign8600_e8089: f64 = if (((param_given[704] || param_given[705]) || param_given[706]) || param_given[707]) { 1.0 } else { 0.0 };
        locals.var_guard120 = assign8600_e8089;
        locals.var_guard120_rv = 0.0;

        let (assign8610_e8109,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard120 != 0.0)) {
        let assign8610_e8097: f64 = (p.p705 * locals.var_ile);
        let assign8610_e8098: f64 = (p.p704 + assign8610_e8097);
        let assign8610_e8101: f64 = (p.p706 * locals.var_iwe);
        let assign8610_e8102: f64 = (assign8610_e8098 + assign8610_e8101);
        let assign8610_e8105: f64 = (p.p707 * locals.var_iae);
        let assign8610_e8106: f64 = (assign8610_e8102 + assign8610_e8105);
        let assign8610_e8107: f64 = (locals.var_iiwecv * assign8610_e8106);
        (assign8610_e8107,)
    } else {
        (locals.var_cinrd_p,)
    }
};
        locals.var_cinrd_p = assign8610_e8109;
        locals.var_cinrd_p_rv = 0.0;

        let assign8740_e8362: f64 = if (((param_given[732] || param_given[733]) || param_given[734]) || param_given[735]) { 1.0 } else { 0.0 };
        locals.var_guard127 = assign8740_e8362;
        locals.var_guard127_rv = 0.0;

        let (assign8750_e8380,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard127 != 0.0)) {
        let assign8750_e8369: f64 = (p.p733 * locals.var_ile);
        let assign8750_e8370: f64 = (p.p732 + assign8750_e8369);
        let assign8750_e8373: f64 = (p.p734 * locals.var_iwe);
        let assign8750_e8374: f64 = (assign8750_e8370 + assign8750_e8373);
        let assign8750_e8377: f64 = (p.p735 * locals.var_iae);
        let assign8750_e8378: f64 = (assign8750_e8374 + assign8750_e8377);
        (assign8750_e8378,)
    } else {
        (locals.var_vfbedge_p,)
    }
};
        locals.var_vfbedge_p = assign8750_e8380;
        locals.var_vfbedge_p_rv = 0.0;

        let assign8760_e8399: f64 = if (((param_given[736] || param_given[737]) || param_given[738]) || param_given[739]) { 1.0 } else { 0.0 };
        locals.var_guard128 = assign8760_e8399;
        locals.var_guard128_rv = 0.0;

        let (assign8770_e8417,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard128 != 0.0)) {
        let assign8770_e8406: f64 = (p.p737 * locals.var_ile);
        let assign8770_e8407: f64 = (p.p736 + assign8770_e8406);
        let assign8770_e8410: f64 = (p.p738 * locals.var_iwe);
        let assign8770_e8411: f64 = (assign8770_e8407 + assign8770_e8410);
        let assign8770_e8414: f64 = (p.p739 * locals.var_iae);
        let assign8770_e8415: f64 = (assign8770_e8411 + assign8770_e8414);
        (assign8770_e8415,)
    } else {
        (locals.var_stvfbedge_p,)
    }
};
        locals.var_stvfbedge_p = assign8770_e8417;
        locals.var_stvfbedge_p_rv = 0.0;

        let assign8780_e8436: f64 = if (((param_given[740] || param_given[741]) || param_given[742]) || param_given[743]) { 1.0 } else { 0.0 };
        locals.var_guard129 = assign8780_e8436;
        locals.var_guard129_rv = 0.0;

        let (assign8790_e8454,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard129 != 0.0)) {
        let assign8790_e8443: f64 = (p.p741 * locals.var_ile);
        let assign8790_e8444: f64 = (p.p740 + assign8790_e8443);
        let assign8790_e8447: f64 = (p.p742 * locals.var_iwe);
        let assign8790_e8448: f64 = (assign8790_e8444 + assign8790_e8447);
        let assign8790_e8451: f64 = (p.p743 * locals.var_iae);
        let assign8790_e8452: f64 = (assign8790_e8448 + assign8790_e8451);
        (assign8790_e8452,)
    } else {
        (locals.var_dphibedge_p,)
    }
};
        locals.var_dphibedge_p = assign8790_e8454;
        locals.var_dphibedge_p_rv = 0.0;

        let assign8800_e8473: f64 = if (((param_given[744] || param_given[745]) || param_given[746]) || param_given[747]) { 1.0 } else { 0.0 };
        locals.var_guard130 = assign8800_e8473;
        locals.var_guard130_rv = 0.0;

        let (assign8810_e8491,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard130 != 0.0)) {
        let assign8810_e8480: f64 = (p.p745 * locals.var_ile);
        let assign8810_e8481: f64 = (p.p744 + assign8810_e8480);
        let assign8810_e8484: f64 = (p.p746 * locals.var_iwe);
        let assign8810_e8485: f64 = (assign8810_e8481 + assign8810_e8484);
        let assign8810_e8488: f64 = (p.p747 * locals.var_iae);
        let assign8810_e8489: f64 = (assign8810_e8485 + assign8810_e8488);
        (assign8810_e8489,)
    } else {
        (locals.var_neffedge_p,)
    }
};
        locals.var_neffedge_p = assign8810_e8491;
        locals.var_neffedge_p_rv = 0.0;

        let assign8820_e8510: f64 = if (((param_given[748] || param_given[749]) || param_given[750]) || param_given[751]) { 1.0 } else { 0.0 };
        locals.var_guard131 = assign8820_e8510;
        locals.var_guard131_rv = 0.0;

        let (assign8830_e8528,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard131 != 0.0)) {
        let assign8830_e8517: f64 = (p.p749 * locals.var_ile);
        let assign8830_e8518: f64 = (p.p748 + assign8830_e8517);
        let assign8830_e8521: f64 = (p.p750 * locals.var_iwe);
        let assign8830_e8522: f64 = (assign8830_e8518 + assign8830_e8521);
        let assign8830_e8525: f64 = (p.p751 * locals.var_iae);
        let assign8830_e8526: f64 = (assign8830_e8522 + assign8830_e8525);
        (assign8830_e8526,)
    } else {
        (locals.var_ctedge_p,)
    }
};
        locals.var_ctedge_p = assign8830_e8528;
        locals.var_ctedge_p_rv = 0.0;

        let assign8840_e8547: f64 = if (((param_given[752] || param_given[753]) || param_given[754]) || param_given[755]) { 1.0 } else { 0.0 };
        locals.var_guard132 = assign8840_e8547;
        locals.var_guard132_rv = 0.0;

        let (assign8850_e8569,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard132 != 0.0)) {
        let assign8850_e8553: f64 = (locals.var_we_edge / locals.var_le);
        let assign8850_e8557: f64 = (p.p753 * locals.var_ile);
        let assign8850_e8558: f64 = (p.p752 + assign8850_e8557);
        let assign8850_e8561: f64 = (p.p754 * locals.var_iwe);
        let assign8850_e8562: f64 = (assign8850_e8558 + assign8850_e8561);
        let assign8850_e8565: f64 = (p.p755 * locals.var_iae);
        let assign8850_e8566: f64 = (assign8850_e8562 + assign8850_e8565);
        let assign8850_e8567: f64 = (assign8850_e8553 * assign8850_e8566);
        (assign8850_e8567,)
    } else {
        (locals.var_betnedge_p,)
    }
};
        locals.var_betnedge_p = assign8850_e8569;
        locals.var_betnedge_p_rv = 0.0;

        let assign8860_e8588: f64 = if (((param_given[756] || param_given[757]) || param_given[758]) || param_given[759]) { 1.0 } else { 0.0 };
        locals.var_guard133 = assign8860_e8588;
        locals.var_guard133_rv = 0.0;

        let (assign8870_e8606,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard133 != 0.0)) {
        let assign8870_e8595: f64 = (p.p757 * locals.var_ile);
        let assign8870_e8596: f64 = (p.p756 + assign8870_e8595);
        let assign8870_e8599: f64 = (p.p758 * locals.var_iwe);
        let assign8870_e8600: f64 = (assign8870_e8596 + assign8870_e8599);
        let assign8870_e8603: f64 = (p.p759 * locals.var_iae);
        let assign8870_e8604: f64 = (assign8870_e8600 + assign8870_e8603);
        (assign8870_e8604,)
    } else {
        (locals.var_stbetedge_p,)
    }
};
        locals.var_stbetedge_p = assign8870_e8606;
        locals.var_stbetedge_p_rv = 0.0;

        let assign8880_e8625: f64 = if (((param_given[760] || param_given[761]) || param_given[762]) || param_given[763]) { 1.0 } else { 0.0 };
        locals.var_guard134 = assign8880_e8625;
        locals.var_guard134_rv = 0.0;

        let (assign8890_e8645,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard134 != 0.0)) {
        let assign8890_e8633: f64 = (p.p761 * locals.var_ile);
        let assign8890_e8634: f64 = (p.p760 + assign8890_e8633);
        let assign8890_e8637: f64 = (p.p762 * locals.var_iwe);
        let assign8890_e8638: f64 = (assign8890_e8634 + assign8890_e8637);
        let assign8890_e8641: f64 = (p.p763 * locals.var_iae);
        let assign8890_e8642: f64 = (assign8890_e8638 + assign8890_e8641);
        let assign8890_e8643: f64 = (locals.var_ile2 * assign8890_e8642);
        (assign8890_e8643,)
    } else {
        (locals.var_psceedge_p,)
    }
};
        locals.var_psceedge_p = assign8890_e8645;
        locals.var_psceedge_p_rv = 0.0;

        let assign8900_e8664: f64 = if (((param_given[764] || param_given[765]) || param_given[766]) || param_given[767]) { 1.0 } else { 0.0 };
        locals.var_guard135 = assign8900_e8664;
        locals.var_guard135_rv = 0.0;

        let (assign8910_e8682,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard135 != 0.0)) {
        let assign8910_e8671: f64 = (p.p765 * locals.var_ile);
        let assign8910_e8672: f64 = (p.p764 + assign8910_e8671);
        let assign8910_e8675: f64 = (p.p766 * locals.var_iwe);
        let assign8910_e8676: f64 = (assign8910_e8672 + assign8910_e8675);
        let assign8910_e8679: f64 = (p.p767 * locals.var_iae);
        let assign8910_e8680: f64 = (assign8910_e8676 + assign8910_e8679);
        (assign8910_e8680,)
    } else {
        (locals.var_pscebedge_p,)
    }
};
        locals.var_pscebedge_p = assign8910_e8682;
        locals.var_pscebedge_p_rv = 0.0;

        let assign8920_e8701: f64 = if (((param_given[768] || param_given[769]) || param_given[770]) || param_given[771]) { 1.0 } else { 0.0 };
        locals.var_guard136 = assign8920_e8701;
        locals.var_guard136_rv = 0.0;

        let (assign8930_e8719,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard136 != 0.0)) {
        let assign8930_e8708: f64 = (p.p769 * locals.var_ile);
        let assign8930_e8709: f64 = (p.p768 + assign8930_e8708);
        let assign8930_e8712: f64 = (p.p770 * locals.var_iwe);
        let assign8930_e8713: f64 = (assign8930_e8709 + assign8930_e8712);
        let assign8930_e8716: f64 = (p.p771 * locals.var_iae);
        let assign8930_e8717: f64 = (assign8930_e8713 + assign8930_e8716);
        (assign8930_e8717,)
    } else {
        (locals.var_pscededge_p,)
    }
};
        locals.var_pscededge_p = assign8930_e8719;
        locals.var_pscededge_p_rv = 0.0;

        let assign8940_e8738: f64 = if (((param_given[772] || param_given[773]) || param_given[774]) || param_given[775]) { 1.0 } else { 0.0 };
        locals.var_guard137 = assign8940_e8738;
        locals.var_guard137_rv = 0.0;

        let (assign8950_e8758,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard137 != 0.0)) {
        let assign8950_e8746: f64 = (p.p773 * locals.var_ile);
        let assign8950_e8747: f64 = (p.p772 + assign8950_e8746);
        let assign8950_e8750: f64 = (p.p774 * locals.var_iwe);
        let assign8950_e8751: f64 = (assign8950_e8747 + assign8950_e8750);
        let assign8950_e8754: f64 = (p.p775 * locals.var_iae);
        let assign8950_e8755: f64 = (assign8950_e8751 + assign8950_e8754);
        let assign8950_e8756: f64 = (locals.var_ile2 * assign8950_e8755);
        (assign8950_e8756,)
    } else {
        (locals.var_cfedge_p,)
    }
};
        locals.var_cfedge_p = assign8950_e8758;
        locals.var_cfedge_p_rv = 0.0;

        let assign8960_e8777: f64 = if (((param_given[780] || param_given[781]) || param_given[782]) || param_given[783]) { 1.0 } else { 0.0 };
        locals.var_guard138 = assign8960_e8777;
        locals.var_guard138_rv = 0.0;

        let (assign8970_e8795,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard138 != 0.0)) {
        let assign8970_e8784: f64 = (p.p781 * locals.var_ile);
        let assign8970_e8785: f64 = (p.p780 + assign8970_e8784);
        let assign8970_e8788: f64 = (p.p782 * locals.var_iwe);
        let assign8970_e8789: f64 = (assign8970_e8785 + assign8970_e8788);
        let assign8970_e8792: f64 = (p.p783 * locals.var_iae);
        let assign8970_e8793: f64 = (assign8970_e8789 + assign8970_e8792);
        (assign8970_e8793,)
    } else {
        (locals.var_cfdedge_p,)
    }
};
        locals.var_cfdedge_p = assign8970_e8795;
        locals.var_cfdedge_p_rv = 0.0;

        let assign8980_e8814: f64 = if (((param_given[776] || param_given[777]) || param_given[778]) || param_given[779]) { 1.0 } else { 0.0 };
        locals.var_guard139 = assign8980_e8814;
        locals.var_guard139_rv = 0.0;

        let (assign8990_e8832,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard139 != 0.0)) {
        let assign8990_e8821: f64 = (p.p777 * locals.var_ile);
        let assign8990_e8822: f64 = (p.p776 + assign8990_e8821);
        let assign8990_e8825: f64 = (p.p778 * locals.var_iwe);
        let assign8990_e8826: f64 = (assign8990_e8822 + assign8990_e8825);
        let assign8990_e8829: f64 = (p.p779 * locals.var_iae);
        let assign8990_e8830: f64 = (assign8990_e8826 + assign8990_e8829);
        (assign8990_e8830,)
    } else {
        (locals.var_cfbedge_p,)
    }
};
        locals.var_cfbedge_p = assign8990_e8832;
        locals.var_cfbedge_p_rv = 0.0;

        let assign9080_e9007: f64 = if (((param_given[800] || param_given[801]) || param_given[802]) || param_given[803]) { 1.0 } else { 0.0 };
        locals.var_guard144 = assign9080_e9007;
        locals.var_guard144_rv = 0.0;

        let (assign9090_e9027,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard144 != 0.0)) {
        let assign9090_e9015: f64 = (p.p801 * locals.var_ile);
        let assign9090_e9016: f64 = (p.p800 + assign9090_e9015);
        let assign9090_e9019: f64 = (p.p802 * locals.var_iwe);
        let assign9090_e9020: f64 = (assign9090_e9016 + assign9090_e9019);
        let assign9090_e9023: f64 = (p.p803 * locals.var_iae);
        let assign9090_e9024: f64 = (assign9090_e9020 + assign9090_e9023);
        let assign9090_e9025: f64 = (locals.var_iiae * assign9090_e9024);
        (assign9090_e9025,)
    } else {
        (locals.var_cth_p,)
    }
};
        locals.var_cth_p = assign9090_e9027;
        locals.var_cth_p_rv = 0.0;

        let (assign9120_e9068,) = {
    if (locals.var_guard36 != 0.0) {
        (0.0,)
    } else {
        (locals.var_tmpa,)
    }
};
        locals.var_tmpa = assign9120_e9068;
        locals.var_tmpa_rv = 0.0;

        let (assign9130_e9072,) = {
    if (locals.var_guard36 != 0.0) {
        (0.0,)
    } else {
        (locals.var_tmpb,)
    }
};
        locals.var_tmpb = assign9130_e9072;
        locals.var_tmpb_rv = 0.0;

        let (assign9140_e9076,) = {
    if (locals.var_guard36 != 0.0) {
        (0.0,)
    } else {
        (locals.var_loop_,)
    }
};
        locals.var_loop_ = assign9140_e9076;
        locals.var_loop__rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_9(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        locals: &mut StampLocals,
    ) {
        let (assign9150_e9080,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p812,)
    } else {
        (locals.var_kvsatac_i,)
    }
};
        locals.var_kvsatac_i = assign9150_e9080;
        locals.var_kvsatac_i_rv = 0.0;

        let assign9160_e9082: f64 = if param_given[813] { 1.0 } else { 0.0 };
        let assign9160_e9084: f64 = if assign9160_e9082 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard146 = assign9160_e9084;
        locals.var_guard146_rv = 0.0;

        let (assign9170_e9090,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard146 != 0.0)) {
        (p.p813,)
    } else {
        (locals.var_kvsatac_i,)
    }
};
        locals.var_kvsatac_i = assign9170_e9090;
        locals.var_kvsatac_i_rv = 0.0;

        let assign9180_e9109: f64 = if (((locals.var_sa_i > 0.0) && (locals.var_sb_i > 0.0)) && ((locals.var_nf_i == 1.0) || ((locals.var_nf_i > 1.0) && (locals.var_sd_i > 0.0)))) { 1.0 } else { 0.0 };
        locals.var_guard147 = assign9180_e9109;
        locals.var_guard147_rv = 0.0;

        let mut assign9190_loop_guard: usize = 0;
        while {
            let assign9190_cond_e9116: f64 = (locals.var_nf_i - 0.5);
            let assign9190_cond_e9118: f64 = if (((locals.var_guard36 != 0.0) && (locals.var_guard147 != 0.0)) && (locals.var_loop_ < assign9190_cond_e9116)) { 1.0 } else { 0.0 };
            assign9190_cond_e9118 != 0.0
        } {
            assign9190_loop_guard += 1;
            assert!(assign9190_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign9190_body0_e9138,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard147 != 0.0)) {
        let assign9190_body0_e9127: f64 = (0.5 * locals.var_l_i);
        let assign9190_body0_e9128: f64 = (locals.var_sa_i + assign9190_body0_e9127);
        let assign9190_body0_e9132: f64 = (locals.var_sd_i + locals.var_l_i);
        let assign9190_body0_e9133: f64 = (locals.var_loop_ * assign9190_body0_e9132);
        let assign9190_body0_e9134: f64 = (assign9190_body0_e9128 + assign9190_body0_e9133);
        let assign9190_body0_e9135: f64 = (1.0 / assign9190_body0_e9134);
        let assign9190_body0_e9136: f64 = (locals.var_tmpa + assign9190_body0_e9135);
        (assign9190_body0_e9136,)
    } else {
        (locals.var_tmpa,)
    }
};
            locals.var_tmpa = assign9190_body0_e9138;
            locals.var_tmpa_rv = 0.0;
            let (assign9190_body1_e9158,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard147 != 0.0)) {
        let assign9190_body1_e9147: f64 = (0.5 * locals.var_l_i);
        let assign9190_body1_e9148: f64 = (locals.var_sb_i + assign9190_body1_e9147);
        let assign9190_body1_e9152: f64 = (locals.var_sd_i + locals.var_l_i);
        let assign9190_body1_e9153: f64 = (locals.var_loop_ * assign9190_body1_e9152);
        let assign9190_body1_e9154: f64 = (assign9190_body1_e9148 + assign9190_body1_e9153);
        let assign9190_body1_e9155: f64 = (1.0 / assign9190_body1_e9154);
        let assign9190_body1_e9156: f64 = (locals.var_tmpb + assign9190_body1_e9155);
        (assign9190_body1_e9156,)
    } else {
        (locals.var_tmpb,)
    }
};
            locals.var_tmpb = assign9190_body1_e9158;
            locals.var_tmpb_rv = 0.0;
            let (assign9190_body2_e9166,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard147 != 0.0)) {
        let assign9190_body2_e9164: f64 = (locals.var_loop_ + 1.0);
        (assign9190_body2_e9164,)
    } else {
        (locals.var_loop_,)
    }
};
            locals.var_loop_ = assign9190_body2_e9166;
            locals.var_loop__rv = 0.0;
        }

        let (assign9200_e9174,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard147 != 0.0)) {
        let assign9200_e9172: f64 = (locals.var_tmpa * locals.var_invnf);
        (assign9200_e9172,)
    } else {
        (locals.var_invsa,)
    }
};
        locals.var_invsa = assign9200_e9174;
        locals.var_invsa_rv = 0.0;

        let (assign9210_e9182,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard147 != 0.0)) {
        let assign9210_e9180: f64 = (locals.var_tmpb * locals.var_invnf);
        (assign9210_e9180,)
    } else {
        (locals.var_invsb,)
    }
};
        locals.var_invsb = assign9210_e9182;
        locals.var_invsb_rv = 0.0;

        let (assign9220_e9194,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard147 != 0.0)) {
        let assign9220_e9190: f64 = (0.5 * locals.var_l_i);
        let assign9220_e9191: f64 = (p.p808 + assign9220_e9190);
        let assign9220_e9192: f64 = (1.0 / assign9220_e9191);
        (assign9220_e9192,)
    } else {
        (locals.var_invsaref,)
    }
};
        locals.var_invsaref = assign9220_e9194;
        locals.var_invsaref_rv = 0.0;

        let (assign9230_e9206,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard147 != 0.0)) {
        let assign9230_e9202: f64 = (0.5 * locals.var_l_i);
        let assign9230_e9203: f64 = (p.p809 + assign9230_e9202);
        let assign9230_e9204: f64 = (1.0 / assign9230_e9203);
        (assign9230_e9204,)
    } else {
        (locals.var_invsbref,)
    }
};
        locals.var_invsbref = assign9230_e9206;
        locals.var_invsbref_rv = 0.0;

        let (assign9240_e9221,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard147 != 0.0)) {
        let assign9240_e9212: f64 = (locals.var_l_i + locals.var_dellps);
        let (assign9240_e9219,) = {
            if (assign9240_e9212 > 1e-9) {
                let assign9240_e9217: f64 = (locals.var_l_i + locals.var_dellps);
                (assign9240_e9217,)
            } else {
                (1e-9,)
            }
        };
        (assign9240_e9219,)
    } else {
        (locals.var_lx,)
    }
};
        locals.var_lx = assign9240_e9221;
        locals.var_lx_rv = 0.0;

        let (assign9250_e9240,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard147 != 0.0)) {
        let assign9250_e9227: f64 = (locals.var_w_i + locals.var_delwod);
        let assign9250_e9229: f64 = (assign9250_e9227 + p.p810);
        let (assign9250_e9238,) = {
            if (assign9250_e9229 > 1e-9) {
                let assign9250_e9234: f64 = (locals.var_w_i + locals.var_delwod);
                let assign9250_e9236: f64 = (assign9250_e9234 + p.p810);
                (assign9250_e9236,)
            } else {
                (1e-9,)
            }
        };
        (assign9250_e9238,)
    } else {
        (locals.var_wx,)
    }
};
        locals.var_wx = assign9250_e9240;
        locals.var_wx_rv = 0.0;

        let (assign9260_e9250,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard147 != 0.0)) {
        let assign9260_e9247: f64 = (locals.var_lx).powf(p.p818);
        let assign9260_e9248: f64 = (1.0 / assign9260_e9247);
        (assign9260_e9248,)
    } else {
        (locals.var_templ,)
    }
};
        locals.var_templ = assign9260_e9250;
        locals.var_templ_rv = 0.0;

        let (assign9270_e9260,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard147 != 0.0)) {
        let assign9270_e9257: f64 = (locals.var_wx).powf(p.p819);
        let assign9270_e9258: f64 = (1.0 / assign9270_e9257);
        (assign9270_e9258,)
    } else {
        (locals.var_tempw,)
    }
};
        locals.var_tempw = assign9270_e9260;
        locals.var_tempw_rv = 0.0;

        let (assign9280_e9288,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard147 != 0.0)) {
        let assign9280_e9267: f64 = (p.p815 * locals.var_templ);
        let assign9280_e9268: f64 = (1.0 + assign9280_e9267);
        let assign9280_e9271: f64 = (p.p816 * locals.var_tempw);
        let assign9280_e9272: f64 = (assign9280_e9268 + assign9280_e9271);
        let assign9280_e9275: f64 = (p.p817 * locals.var_templ);
        let assign9280_e9277: f64 = (assign9280_e9275 * locals.var_tempw);
        let assign9280_e9278: f64 = (assign9280_e9272 + assign9280_e9277);
        let assign9280_e9283: f64 = (locals.var_rta - 1.0);
        let assign9280_e9284: f64 = (p.p814 * assign9280_e9283);
        let assign9280_e9285: f64 = (1.0 + assign9280_e9284);
        let assign9280_e9286: f64 = (assign9280_e9278 * assign9280_e9285);
        (assign9280_e9286,)
    } else {
        (locals.var_kstressu0,)
    }
};
        locals.var_kstressu0 = assign9280_e9288;
        locals.var_kstressu0_rv = 0.0;

        let (assign9290_e9300,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard147 != 0.0)) {
        let assign9290_e9295: f64 = (locals.var_invsa + locals.var_invsb);
        let assign9290_e9296: f64 = (p.p811 * assign9290_e9295);
        let assign9290_e9298: f64 = (assign9290_e9296 / locals.var_kstressu0);
        (assign9290_e9298,)
    } else {
        (locals.var_rhobeta,)
    }
};
        locals.var_rhobeta = assign9290_e9300;
        locals.var_rhobeta_rv = 0.0;

        let (assign9300_e9312,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard147 != 0.0)) {
        let assign9300_e9307: f64 = (locals.var_invsaref + locals.var_invsbref);
        let assign9300_e9308: f64 = (p.p811 * assign9300_e9307);
        let assign9300_e9310: f64 = (assign9300_e9308 / locals.var_kstressu0);
        (assign9300_e9310,)
    } else {
        (locals.var_rhobetaref,)
    }
};
        locals.var_rhobetaref = assign9300_e9312;
        locals.var_rhobetaref_rv = 0.0;

        let (assign9310_e9322,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard147 != 0.0)) {
        let assign9310_e9319: f64 = (locals.var_lx).powf(p.p824);
        let assign9310_e9320: f64 = (1.0 / assign9310_e9319);
        (assign9310_e9320,)
    } else {
        (locals.var_templ,)
    }
};
        locals.var_templ = assign9310_e9322;
        locals.var_templ_rv = 0.0;

        let (assign9320_e9332,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard147 != 0.0)) {
        let assign9320_e9329: f64 = (locals.var_wx).powf(p.p825);
        let assign9320_e9330: f64 = (1.0 / assign9320_e9329);
        (assign9320_e9330,)
    } else {
        (locals.var_tempw,)
    }
};
        locals.var_tempw = assign9320_e9332;
        locals.var_tempw_rv = 0.0;

        let (assign9330_e9352,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard147 != 0.0)) {
        let assign9330_e9339: f64 = (p.p821 * locals.var_templ);
        let assign9330_e9340: f64 = (1.0 + assign9330_e9339);
        let assign9330_e9343: f64 = (p.p822 * locals.var_tempw);
        let assign9330_e9344: f64 = (assign9330_e9340 + assign9330_e9343);
        let assign9330_e9347: f64 = (p.p823 * locals.var_templ);
        let assign9330_e9349: f64 = (assign9330_e9347 * locals.var_tempw);
        let assign9330_e9350: f64 = (assign9330_e9344 + assign9330_e9349);
        (assign9330_e9350,)
    } else {
        (locals.var_kstressvth0,)
    }
};
        locals.var_kstressvth0 = assign9330_e9352;
        locals.var_kstressvth0_rv = 0.0;

        let (assign9340_e9364,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard147 != 0.0)) {
        let assign9340_e9358: f64 = (locals.var_invsa + locals.var_invsb);
        let assign9340_e9360: f64 = (assign9340_e9358 - locals.var_invsaref);
        let assign9340_e9362: f64 = (assign9340_e9360 - locals.var_invsbref);
        (assign9340_e9362,)
    } else {
        (locals.var_temp0,)
    }
};
        locals.var_temp0 = assign9340_e9364;
        locals.var_temp0_rv = 0.0;

        let (assign9350_e9376,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard147 != 0.0)) {
        let assign9350_e9370: f64 = (1.0 + locals.var_rhobeta);
        let assign9350_e9373: f64 = (1.0 + locals.var_rhobetaref);
        let assign9350_e9374: f64 = (assign9350_e9370 / assign9350_e9373);
        (assign9350_e9374,)
    } else {
        (locals.var_temp00,)
    }
};
        locals.var_temp00 = assign9350_e9376;
        locals.var_temp00_rv = 0.0;

        let (assign9360_e9384,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard147 != 0.0)) {
        let assign9360_e9382: f64 = (locals.var_betn_p * locals.var_temp00);
        (assign9360_e9382,)
    } else {
        (locals.var_betn_p,)
    }
};
        locals.var_betn_p = assign9360_e9384;
        locals.var_betn_p_rv = 0.0;

        let (assign9370_e9404,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard147 != 0.0)) {
        let assign9370_e9390: f64 = (locals.var_thesat_p * locals.var_temp00);
        let assign9370_e9394: f64 = (p.p812 * locals.var_rhobetaref);
        let assign9370_e9395: f64 = (1.0 + assign9370_e9394);
        let assign9370_e9396: f64 = (assign9370_e9390 * assign9370_e9395);
        let assign9370_e9400: f64 = (p.p812 * locals.var_rhobeta);
        let assign9370_e9401: f64 = (1.0 + assign9370_e9400);
        let assign9370_e9402: f64 = (assign9370_e9396 / assign9370_e9401);
        (assign9370_e9402,)
    } else {
        (locals.var_thesat_p,)
    }
};
        locals.var_thesat_p = assign9370_e9404;
        locals.var_thesat_p_rv = 0.0;

        let (assign9380_e9424,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard147 != 0.0)) {
        let assign9380_e9410: f64 = (locals.var_thesatac_p * locals.var_temp00);
        let assign9380_e9414: f64 = (locals.var_kvsatac_i * locals.var_rhobetaref);
        let assign9380_e9415: f64 = (1.0 + assign9380_e9414);
        let assign9380_e9416: f64 = (assign9380_e9410 * assign9380_e9415);
        let assign9380_e9420: f64 = (locals.var_kvsatac_i * locals.var_rhobeta);
        let assign9380_e9421: f64 = (1.0 + assign9380_e9420);
        let assign9380_e9422: f64 = (assign9380_e9416 / assign9380_e9421);
        (assign9380_e9422,)
    } else {
        (locals.var_thesatac_p,)
    }
};
        locals.var_thesatac_p = assign9380_e9424;
        locals.var_thesatac_p_rv = 0.0;

        let (assign9390_e9432,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard147 != 0.0)) {
        let assign9390_e9430: f64 = (locals.var_betnedge_p * locals.var_temp00);
        (assign9390_e9430,)
    } else {
        (locals.var_betnedge_p,)
    }
};
        locals.var_betnedge_p = assign9390_e9432;
        locals.var_betnedge_p_rv = 0.0;

        let (assign9400_e9442,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard147 != 0.0)) {
        let assign9400_e9438: f64 = (p.p820 * locals.var_temp0);
        let assign9400_e9440: f64 = (assign9400_e9438 / locals.var_kstressvth0);
        (assign9400_e9440,)
    } else {
        (locals.var_temp00,)
    }
};
        locals.var_temp00 = assign9400_e9442;
        locals.var_temp00_rv = 0.0;

        let (assign9410_e9450,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard147 != 0.0)) {
        let assign9410_e9448: f64 = (locals.var_vfb_p + locals.var_temp00);
        (assign9410_e9448,)
    } else {
        (locals.var_vfb_p,)
    }
};
        locals.var_vfb_p = assign9410_e9450;
        locals.var_vfb_p_rv = 0.0;

        let (assign9420_e9458,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard147 != 0.0)) {
        let assign9420_e9456: f64 = (locals.var_vfbedge_p + locals.var_temp00);
        (assign9420_e9456,)
    } else {
        (locals.var_vfbedge_p,)
    }
};
        locals.var_vfbedge_p = assign9420_e9458;
        locals.var_vfbedge_p_rv = 0.0;

        let (assign9430_e9470,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard147 != 0.0)) {
        let assign9430_e9464: f64 = (p.p826 * locals.var_temp0);
        let assign9430_e9467: f64 = (locals.var_kstressvth0).powf(p.p827);
        let assign9430_e9468: f64 = (assign9430_e9464 / assign9430_e9467);
        (assign9430_e9468,)
    } else {
        (locals.var_temp00,)
    }
};
        locals.var_temp00 = assign9430_e9470;
        locals.var_temp00_rv = 0.0;

        let (assign9440_e9478,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard147 != 0.0)) {
        let assign9440_e9476: f64 = (locals.var_cf_p + locals.var_temp00);
        (assign9440_e9476,)
    } else {
        (locals.var_cf_p,)
    }
};
        locals.var_cf_p = assign9440_e9478;
        locals.var_cf_p_rv = 0.0;

        let (assign9450_e9486,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard147 != 0.0)) {
        let assign9450_e9484: f64 = (locals.var_cfedge_p + locals.var_temp00);
        (assign9450_e9484,)
    } else {
        (locals.var_cfedge_p,)
    }
};
        locals.var_cfedge_p = assign9450_e9486;
        locals.var_cfedge_p_rv = 0.0;

        let assign9460_e9501: f64 = if ((((locals.var_sca_i > 0.0) || (locals.var_scb_i > 0.0)) || (locals.var_scc_i > 0.0)) || (locals.var_sc_i > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard148 = assign9460_e9501;
        locals.var_guard148_rv = 0.0;

        let assign9470_e9512: f64 = if (((locals.var_sca_i == 0.0) && (locals.var_scb_i == 0.0)) && (locals.var_scc_i == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard149 = assign9470_e9512;
        locals.var_guard149_rv = 0.0;

        let (assign9480_e9522,) = {
    if (((locals.var_guard36 != 0.0) && (locals.var_guard148 != 0.0)) && (locals.var_guard149 != 0.0)) {
        let assign9480_e9520: f64 = (locals.var_sc_i + locals.var_w_i);
        (assign9480_e9520,)
    } else {
        (locals.var_temp0,)
    }
};
        locals.var_temp0 = assign9480_e9522;
        locals.var_temp0_rv = 0.0;

        let (assign9490_e9532,) = {
    if (((locals.var_guard36 != 0.0) && (locals.var_guard148 != 0.0)) && (locals.var_guard149 != 0.0)) {
        let assign9490_e9530: f64 = (1.0 / p.p828);
        (assign9490_e9530,)
    } else {
        (locals.var_temp00,)
    }
};
        locals.var_temp00 = assign9490_e9532;
        locals.var_temp00_rv = 0.0;

        let (assign9500_e9546,) = {
    if (((locals.var_guard36 != 0.0) && (locals.var_guard148 != 0.0)) && (locals.var_guard149 != 0.0)) {
        let assign9500_e9540: f64 = (p.p828 * p.p828);
        let assign9500_e9543: f64 = (locals.var_sc_i * locals.var_temp0);
        let assign9500_e9544: f64 = (assign9500_e9540 / assign9500_e9543);
        (assign9500_e9544,)
    } else {
        (locals.var_sca_i,)
    }
};
        locals.var_sca_i = assign9500_e9546;
        locals.var_sca_i_rv = 0.0;

        let (assign9510_e9586,) = {
    if (((locals.var_guard36 != 0.0) && (locals.var_guard148 != 0.0)) && (locals.var_guard149 != 0.0)) {
        let assign9510_e9554: f64 = (0.1 * locals.var_sc_i);
        let assign9510_e9557: f64 = (0.01 * p.p828);
        let assign9510_e9558: f64 = (assign9510_e9554 + assign9510_e9557);
        let assign9510_e9560: f64 = (-10.0);
        let assign9510_e9562: f64 = (assign9510_e9560 * locals.var_sc_i);
        let assign9510_e9564: f64 = (assign9510_e9562 * locals.var_temp00);
        let assign9510_e9565: f64 = (assign9510_e9564).exp();
        let assign9510_e9566: f64 = (assign9510_e9558 * assign9510_e9565);
        let assign9510_e9569: f64 = (0.1 * locals.var_temp0);
        let assign9510_e9572: f64 = (0.01 * p.p828);
        let assign9510_e9573: f64 = (assign9510_e9569 + assign9510_e9572);
        let assign9510_e9575: f64 = (-10.0);
        let assign9510_e9577: f64 = (assign9510_e9575 * locals.var_temp0);
        let assign9510_e9579: f64 = (assign9510_e9577 * locals.var_temp00);
        let assign9510_e9580: f64 = (assign9510_e9579).exp();
        let assign9510_e9581: f64 = (assign9510_e9573 * assign9510_e9580);
        let assign9510_e9582: f64 = (assign9510_e9566 - assign9510_e9581);
        let assign9510_e9584: f64 = (assign9510_e9582 / locals.var_w_i);
        (assign9510_e9584,)
    } else {
        (locals.var_scb_i,)
    }
};
        locals.var_scb_i = assign9510_e9586;
        locals.var_scb_i_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_10(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign9520_e9626,) = {
    if (((locals.var_guard36 != 0.0) && (locals.var_guard148 != 0.0)) && (locals.var_guard149 != 0.0)) {
        let assign9520_e9594: f64 = (0.05 * locals.var_sc_i);
        let assign9520_e9597: f64 = (0.0025 * p.p828);
        let assign9520_e9598: f64 = (assign9520_e9594 + assign9520_e9597);
        let assign9520_e9600: f64 = (-20.0);
        let assign9520_e9602: f64 = (assign9520_e9600 * locals.var_sc_i);
        let assign9520_e9604: f64 = (assign9520_e9602 * locals.var_temp00);
        let assign9520_e9605: f64 = (assign9520_e9604).exp();
        let assign9520_e9606: f64 = (assign9520_e9598 * assign9520_e9605);
        let assign9520_e9609: f64 = (0.05 * locals.var_temp0);
        let assign9520_e9612: f64 = (0.0025 * p.p828);
        let assign9520_e9613: f64 = (assign9520_e9609 + assign9520_e9612);
        let assign9520_e9615: f64 = (-20.0);
        let assign9520_e9617: f64 = (assign9520_e9615 * locals.var_temp0);
        let assign9520_e9619: f64 = (assign9520_e9617 * locals.var_temp00);
        let assign9520_e9620: f64 = (assign9520_e9619).exp();
        let assign9520_e9621: f64 = (assign9520_e9613 * assign9520_e9620);
        let assign9520_e9622: f64 = (assign9520_e9606 - assign9520_e9621);
        let assign9520_e9624: f64 = (assign9520_e9622 / locals.var_w_i);
        (assign9520_e9624,)
    } else {
        (locals.var_scc_i,)
    }
};
        locals.var_scc_i = assign9520_e9626;
        locals.var_scc_i_rv = 0.0;

        let (assign9530_e9640,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard148 != 0.0)) {
        let assign9530_e9633: f64 = (p.p829 * locals.var_scb_i);
        let assign9530_e9634: f64 = (locals.var_sca_i + assign9530_e9633);
        let assign9530_e9637: f64 = (p.p830 * locals.var_scc_i);
        let assign9530_e9638: f64 = (assign9530_e9634 + assign9530_e9637);
        (assign9530_e9638,)
    } else {
        (locals.var_temp0,)
    }
};
        locals.var_temp0 = assign9530_e9640;
        locals.var_temp0_rv = 0.0;

        let (assign9540_e9650,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard148 != 0.0)) {
        let assign9540_e9647: f64 = (locals.var_kvthowe * locals.var_temp0);
        let assign9540_e9648: f64 = (locals.var_vfb_p + assign9540_e9647);
        (assign9540_e9648,)
    } else {
        (locals.var_vfb_p,)
    }
};
        locals.var_vfb_p = assign9540_e9650;
        locals.var_vfb_p_rv = 0.0;

        let (assign9550_e9662,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard148 != 0.0)) {
        let assign9550_e9658: f64 = (locals.var_kuowe * locals.var_temp0);
        let assign9550_e9659: f64 = (1.0 + assign9550_e9658);
        let assign9550_e9660: f64 = (locals.var_betn_p * assign9550_e9659);
        (assign9550_e9660,)
    } else {
        (locals.var_betn_p,)
    }
};
        locals.var_betn_p = assign9550_e9662;
        locals.var_betn_p_rv = 0.0;

        let (assign9560_e9672,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard148 != 0.0)) {
        let assign9560_e9669: f64 = (locals.var_kvthowe * locals.var_temp0);
        let assign9560_e9670: f64 = (locals.var_vfbedge_p + assign9560_e9669);
        (assign9560_e9670,)
    } else {
        (locals.var_vfbedge_p,)
    }
};
        locals.var_vfbedge_p = assign9560_e9672;
        locals.var_vfbedge_p_rv = 0.0;

        let (assign9570_e9684,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard148 != 0.0)) {
        let assign9570_e9680: f64 = (locals.var_kuowe * locals.var_temp0);
        let assign9570_e9681: f64 = (1.0 + assign9570_e9680);
        let assign9570_e9682: f64 = (locals.var_betnedge_p * assign9570_e9681);
        (assign9570_e9682,)
    } else {
        (locals.var_betnedge_p,)
    }
};
        locals.var_betnedge_p = assign9570_e9684;
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

        let (assign9630_e9700,) = {
    if (locals.var_neff_p > 1e20) {
        let (assign9630_e9698,) = {
            if (locals.var_neff_p < 1e26) {
                (locals.var_neff_p,)
            } else {
                (1e26,)
            }
        };
        (assign9630_e9698,)
    } else {
        (1e20,)
    }
};
        locals.var_neff_i = assign9630_e9700;
        locals.var_neff_i_rv = 0.0;

        let (assign9640_e9706,) = {
    if (locals.var_gfacnud_p > 0.01) {
        (locals.var_gfacnud_p,)
    } else {
        (0.01,)
    }
};
        locals.var_gfacnud_i = assign9640_e9706;
        locals.var_gfacnud_i_rv = 0.0;

        let (assign9650_e9712,) = {
    if (locals.var_vsbnud_p > 0.0) {
        (locals.var_vsbnud_p,)
    } else {
        (0.0,)
    }
};
        locals.var_vsbnud_i = assign9650_e9712;
        locals.var_vsbnud_i_rv = 0.0;

        locals.var_dvsbnud_i = locals.var_dvsbnud_p;
        locals.var_dvsbnud_i_rv = 0.0;

        locals.var_dphib_i = locals.var_dphib_p;
        locals.var_dphib_i_rv = 0.0;

        let (assign9680_e9720,) = {
    if (locals.var_np_p > 0.0) {
        (locals.var_np_p,)
    } else {
        (0.0,)
    }
};
        locals.var_np_i = assign9680_e9720;
        locals.var_np_i_rv = 0.0;

        locals.var_toxov_i = locals.var_toxov_p;
        locals.var_toxov_i_rv = 0.0;

        locals.var_toxovd_i = locals.var_toxovd_p;
        locals.var_toxovd_i_rv = 0.0;

        let (assign9710_e9733,) = {
    if (locals.var_nov_p > 1e23) {
        let (assign9710_e9731,) = {
            if (locals.var_nov_p < 1e27) {
                (locals.var_nov_p,)
            } else {
                (1e27,)
            }
        };
        (assign9710_e9731,)
    } else {
        (1e23,)
    }
};
        locals.var_nov_i = assign9710_e9733;
        locals.var_nov_i_rv = 0.0;

        let (assign9720_e9744,) = {
    if (locals.var_novd_p > 1e23) {
        let (assign9720_e9742,) = {
            if (locals.var_novd_p < 1e27) {
                (locals.var_novd_p,)
            } else {
                (1e27,)
            }
        };
        (assign9720_e9742,)
    } else {
        (1e23,)
    }
};
        locals.var_novd_i = assign9720_e9744;
        locals.var_novd_i_rv = 0.0;

        let (assign9730_e9750,) = {
    if (locals.var_ct_p > 0.0) {
        (locals.var_ct_p,)
    } else {
        (0.0,)
    }
};
        locals.var_ct_i = assign9730_e9750;
        locals.var_ct_i_rv = 0.0;

        let (assign9740_e9761,) = {
    if (locals.var_ctb_p > 0.0) {
        let (assign9740_e9759,) = {
            if (locals.var_ctb_p < 0.5) {
                (locals.var_ctb_p,)
            } else {
                (0.5,)
            }
        };
        (assign9740_e9759,)
    } else {
        (0.0,)
    }
};
        locals.var_ctb_i = assign9740_e9761;
        locals.var_ctb_i_rv = 0.0;

        let (assign9750_e9772,) = {
    if (locals.var_ctg_p > 0.0) {
        let (assign9750_e9770,) = {
            if (locals.var_ctg_p < 1.0) {
                (locals.var_ctg_p,)
            } else {
                (1.0,)
            }
        };
        (assign9750_e9770,)
    } else {
        (0.0,)
    }
};
        locals.var_ctg_i = assign9750_e9772;
        locals.var_ctg_i_rv = 0.0;

        locals.var_stct_i = locals.var_stct_p;
        locals.var_stct_i_rv = 0.0;

        let (assign9770_e9779,) = {
    if (locals.var_cf_p > 0.0) {
        (locals.var_cf_p,)
    } else {
        (0.0,)
    }
};
        locals.var_cf_i = assign9770_e9779;
        locals.var_cf_i_rv = 0.0;

        let (assign9780_e9790,) = {
    if (locals.var_cfb_p > 0.0) {
        let (assign9780_e9788,) = {
            if (locals.var_cfb_p < 1.0) {
                (locals.var_cfb_p,)
            } else {
                (1.0,)
            }
        };
        (assign9780_e9788,)
    } else {
        (0.0,)
    }
};
        locals.var_cfb_i = assign9780_e9790;
        locals.var_cfb_i_rv = 0.0;

        let (assign9790_e9796,) = {
    if (locals.var_cfd_p > 0.0) {
        (locals.var_cfd_p,)
    } else {
        (0.0,)
    }
};
        locals.var_cfd_i = assign9790_e9796;
        locals.var_cfd_i_rv = 0.0;

        let (assign9800_e9802,) = {
    if (locals.var_psce_p > 0.0) {
        (locals.var_psce_p,)
    } else {
        (0.0,)
    }
};
        locals.var_psce_i = assign9800_e9802;
        locals.var_psce_i_rv = 0.0;

        let (assign9810_e9813,) = {
    if (locals.var_psceb_p > 0.0) {
        let (assign9810_e9811,) = {
            if (locals.var_psceb_p < 1.0) {
                (locals.var_psceb_p,)
            } else {
                (1.0,)
            }
        };
        (assign9810_e9811,)
    } else {
        (0.0,)
    }
};
        locals.var_psceb_i = assign9810_e9813;
        locals.var_psceb_i_rv = 0.0;

        let (assign9820_e9819,) = {
    if (locals.var_psced_p > 0.0) {
        (locals.var_psced_p,)
    } else {
        (0.0,)
    }
};
        locals.var_psced_i = assign9820_e9819;
        locals.var_psced_i_rv = 0.0;

        let (assign9830_e9825,) = {
    if (locals.var_betn_p > 0.0) {
        (locals.var_betn_p,)
    } else {
        (0.0,)
    }
};
        locals.var_betn_i = assign9830_e9825;
        locals.var_betn_i_rv = 0.0;

        locals.var_stbet_i = locals.var_stbet_p;
        locals.var_stbet_i_rv = 0.0;

        let (assign9850_e9832,) = {
    if (locals.var_mue_p > 0.0) {
        (locals.var_mue_p,)
    } else {
        (0.0,)
    }
};
        locals.var_mue_i = assign9850_e9832;
        locals.var_mue_i_rv = 0.0;

        locals.var_stmue_i = locals.var_stmue_p;
        locals.var_stmue_i_rv = 0.0;

        let (assign9870_e9839,) = {
    if (locals.var_themu_p > 0.0) {
        (locals.var_themu_p,)
    } else {
        (0.0,)
    }
};
        locals.var_themu_i = assign9870_e9839;
        locals.var_themu_i_rv = 0.0;

        locals.var_stthemu_i = locals.var_stthemu_p;
        locals.var_stthemu_i_rv = 0.0;

        let (assign9890_e9846,) = {
    if (locals.var_cs_p > 0.0) {
        (locals.var_cs_p,)
    } else {
        (0.0,)
    }
};
        locals.var_cs_i = assign9890_e9846;
        locals.var_cs_i_rv = 0.0;

        locals.var_stcs_i = locals.var_stcs_p;
        locals.var_stcs_i_rv = 0.0;

        let (assign9910_e9853,) = {
    if (locals.var_thecs_p > 0.0) {
        (locals.var_thecs_p,)
    } else {
        (0.0,)
    }
};
        locals.var_thecs_i = assign9910_e9853;
        locals.var_thecs_i_rv = 0.0;

        locals.var_stthecs_i = locals.var_stthecs_p;
        locals.var_stthecs_i_rv = 0.0;

        let (assign9930_e9860,) = {
    if (locals.var_xcor_p > 0.0) {
        (locals.var_xcor_p,)
    } else {
        (0.0,)
    }
};
        locals.var_xcor_i = assign9930_e9860;
        locals.var_xcor_i_rv = 0.0;

        locals.var_stxcor_i = locals.var_stxcor_p;
        locals.var_stxcor_i_rv = 0.0;

        locals.var_feta_i = locals.var_feta_p;
        locals.var_feta_i_rv = 0.0;

        let (assign9960_e9868,) = {
    if (locals.var_rs_p > 0.0) {
        (locals.var_rs_p,)
    } else {
        (0.0,)
    }
};
        locals.var_rs_i = assign9960_e9868;
        locals.var_rs_i_rv = 0.0;

        locals.var_strs_i = locals.var_strs_p;
        locals.var_strs_i_rv = 0.0;

        let assign9980_e9872: f64 = (-0.5);
        let (assign9980_e9882,) = {
    if (locals.var_rsb_p > assign9980_e9872) {
        let (assign9980_e9879,) = {
            if (locals.var_rsb_p < 1.0) {
                (locals.var_rsb_p,)
            } else {
                (1.0,)
            }
        };
        (assign9980_e9879,)
    } else {
        let assign9980_e9881: f64 = (-0.5);
        (assign9980_e9881,)
    }
};
        locals.var_rsb_i = assign9980_e9882;
        locals.var_rsb_i_rv = 0.0;

        let assign9990_e9885: f64 = (-0.5);
        let (assign9990_e9890,) = {
    if (locals.var_rsg_p > assign9990_e9885) {
        (locals.var_rsg_p,)
    } else {
        let assign9990_e9889: f64 = (-0.5);
        (assign9990_e9889,)
    }
};
        locals.var_rsg_i = assign9990_e9890;
        locals.var_rsg_i_rv = 0.0;

        let (assign10000_e9896,) = {
    if (locals.var_thesat_p > 0.0) {
        (locals.var_thesat_p,)
    } else {
        (0.0,)
    }
};
        locals.var_thesat_i = assign10000_e9896;
        locals.var_thesat_i_rv = 0.0;

        locals.var_stthesat_i = locals.var_stthesat_p;
        locals.var_stthesat_i_rv = 0.0;

        let assign10020_e9900: f64 = (-0.5);
        let (assign10020_e9910,) = {
    if (locals.var_thesatb_p > assign10020_e9900) {
        let (assign10020_e9907,) = {
            if (locals.var_thesatb_p < 1.0) {
                (locals.var_thesatb_p,)
            } else {
                (1.0,)
            }
        };
        (assign10020_e9907,)
    } else {
        let assign10020_e9909: f64 = (-0.5);
        (assign10020_e9909,)
    }
};
        locals.var_thesatb_i = assign10020_e9910;
        locals.var_thesatb_i_rv = 0.0;

        let assign10030_e9913: f64 = (-0.5);
        let (assign10030_e9918,) = {
    if (locals.var_thesatg_p > assign10030_e9913) {
        (locals.var_thesatg_p,)
    } else {
        let assign10030_e9917: f64 = (-0.5);
        (assign10030_e9917,)
    }
};
        locals.var_thesatg_i = assign10030_e9918;
        locals.var_thesatg_i_rv = 0.0;

        let (assign10040_e9924,) = {
    if (locals.var_thesatt_p > 0.01) {
        (locals.var_thesatt_p,)
    } else {
        (0.01,)
    }
};
        locals.var_thesatt_i = assign10040_e9924;
        locals.var_thesatt_i_rv = 0.0;

        let (assign10050_e9930,) = {
    if (locals.var_ax_p > 2.0) {
        (locals.var_ax_p,)
    } else {
        (2.0,)
    }
};
        locals.var_ax_i = assign10050_e9930;
        locals.var_ax_i_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_11(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign10060_e9936,) = {
    if (locals.var_alp_p > 0.0) {
        (locals.var_alp_p,)
    } else {
        (0.0,)
    }
};
        locals.var_alp_i = assign10060_e9936;
        locals.var_alp_i_rv = 0.0;

        let (assign10070_e9942,) = {
    if (locals.var_alp1_p > 0.0) {
        (locals.var_alp1_p,)
    } else {
        (0.0,)
    }
};
        locals.var_alp1_i = assign10070_e9942;
        locals.var_alp1_i_rv = 0.0;

        let (assign10080_e9948,) = {
    if (locals.var_alp2_p > 0.0) {
        (locals.var_alp2_p,)
    } else {
        (0.0,)
    }
};
        locals.var_alp2_i = assign10080_e9948;
        locals.var_alp2_i_rv = 0.0;

        locals.var_vp_i = locals.var_vp_p;
        locals.var_vp_i_rv = 0.0;

        let (assign10100_e9955,) = {
    if (locals.var_a1_p > 0.0) {
        (locals.var_a1_p,)
    } else {
        (0.0,)
    }
};
        locals.var_a1_i = assign10100_e9955;
        locals.var_a1_i_rv = 0.0;

        locals.var_a2_i = locals.var_a2_p;
        locals.var_a2_i_rv = 0.0;

        locals.var_sta2_i = locals.var_sta2_p;
        locals.var_sta2_i_rv = 0.0;

        let (assign10130_e9963,) = {
    if (locals.var_a3_p > 0.0) {
        (locals.var_a3_p,)
    } else {
        (0.0,)
    }
};
        locals.var_a3_i = assign10130_e9963;
        locals.var_a3_i_rv = 0.0;

        let (assign10140_e9969,) = {
    if (locals.var_a4_p > 0.0) {
        (locals.var_a4_p,)
    } else {
        (0.0,)
    }
};
        locals.var_a4_i = assign10140_e9969;
        locals.var_a4_i_rv = 0.0;

        let (assign10150_e9975,) = {
    if (locals.var_imaxii_p > 1e-12) {
        (locals.var_imaxii_p,)
    } else {
        (1e-12,)
    }
};
        locals.var_imaxii_i = assign10150_e9975;
        locals.var_imaxii_i_rv = 0.0;

        locals.var_gco_i = locals.var_gco_p;
        locals.var_gco_i_rv = 0.0;

        let (assign10170_e9982,) = {
    if (locals.var_iginv_p > 0.0) {
        (locals.var_iginv_p,)
    } else {
        (0.0,)
    }
};
        locals.var_iginv_i = assign10170_e9982;
        locals.var_iginv_i_rv = 0.0;

        let (assign10180_e9988,) = {
    if (locals.var_igov_p > 0.0) {
        (locals.var_igov_p,)
    } else {
        (0.0,)
    }
};
        locals.var_igov_i = assign10180_e9988;
        locals.var_igov_i_rv = 0.0;

        let (assign10190_e9994,) = {
    if (locals.var_igovd_p > 0.0) {
        (locals.var_igovd_p,)
    } else {
        (0.0,)
    }
};
        locals.var_igovd_i = assign10190_e9994;
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

        let (assign10280_e10008,) = {
    if (locals.var_agidl_p > 0.0) {
        (locals.var_agidl_p,)
    } else {
        (0.0,)
    }
};
        locals.var_agidl_i = assign10280_e10008;
        locals.var_agidl_i_rv = 0.0;

        let (assign10290_e10014,) = {
    if (locals.var_agidld_p > 0.0) {
        (locals.var_agidld_p,)
    } else {
        (0.0,)
    }
};
        locals.var_agidld_i = assign10290_e10014;
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

        let (assign10360_e10026,) = {
    if (locals.var_cox_p > 0.0) {
        (locals.var_cox_p,)
    } else {
        (0.0,)
    }
};
        locals.var_cox_i = assign10360_e10026;
        locals.var_cox_i_rv = 0.0;

        locals.var_delvtac_i = locals.var_delvtac_p;
        locals.var_delvtac_i_rv = 0.0;

        let (assign10380_e10033,) = {
    if (locals.var_facneffac_p > 0.0) {
        (locals.var_facneffac_p,)
    } else {
        (0.0,)
    }
};
        locals.var_facneffac_i = assign10380_e10033;
        locals.var_facneffac_i_rv = 0.0;

        let (assign10390_e10039,) = {
    if (locals.var_thesatac_p > 0.0) {
        (locals.var_thesatac_p,)
    } else {
        (0.0,)
    }
};
        locals.var_thesatac_i = assign10390_e10039;
        locals.var_thesatac_i_rv = 0.0;

        let (assign10400_e10045,) = {
    if (locals.var_axac_p > 2.0) {
        (locals.var_axac_p,)
    } else {
        (2.0,)
    }
};
        locals.var_axac_i = assign10400_e10045;
        locals.var_axac_i_rv = 0.0;

        locals.var_alpac_i = locals.var_alpac_p;
        locals.var_alpac_i_rv = 0.0;

        let (assign10420_e10052,) = {
    if (locals.var_alp1ac_p > 0.0) {
        (locals.var_alp1ac_p,)
    } else {
        (0.0,)
    }
};
        locals.var_alp1ac_i = assign10420_e10052;
        locals.var_alp1ac_i_rv = 0.0;

        let (assign10430_e10058,) = {
    if (locals.var_cgov_p > 0.0) {
        (locals.var_cgov_p,)
    } else {
        (0.0,)
    }
};
        locals.var_cgov_i = assign10430_e10058;
        locals.var_cgov_i_rv = 0.0;

        let (assign10440_e10064,) = {
    if (locals.var_cgovd_p > 0.0) {
        (locals.var_cgovd_p,)
    } else {
        (0.0,)
    }
};
        locals.var_cgovd_i = assign10440_e10064;
        locals.var_cgovd_i_rv = 0.0;

        locals.var_fcgovacc_i = locals.var_fcgovacc_p;
        locals.var_fcgovacc_i_rv = 0.0;

        locals.var_fcgovaccd_i = locals.var_fcgovaccd_p;
        locals.var_fcgovaccd_i_rv = 0.0;

        locals.var_cgovaccg_i = locals.var_cgovaccg_p;
        locals.var_cgovaccg_i_rv = 0.0;

        let (assign10480_e10073,) = {
    if (locals.var_cgbov_p > 0.0) {
        (locals.var_cgbov_p,)
    } else {
        (0.0,)
    }
};
        locals.var_cgbov_i = assign10480_e10073;
        locals.var_cgbov_i_rv = 0.0;

        let (assign10490_e10079,) = {
    if (locals.var_cinr_p > 0.0) {
        (locals.var_cinr_p,)
    } else {
        (0.0,)
    }
};
        locals.var_cinr_i = assign10490_e10079;
        locals.var_cinr_i_rv = 0.0;

        let (assign10500_e10085,) = {
    if (locals.var_cinrd_p > 0.0) {
        (locals.var_cinrd_p,)
    } else {
        (0.0,)
    }
};
        locals.var_cinrd_i = assign10500_e10085;
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

        let (assign10660_e10141,) = {
    if (locals.var_neffedge_p > 1e20) {
        let (assign10660_e10139,) = {
            if (locals.var_neffedge_p < 1e26) {
                (locals.var_neffedge_p,)
            } else {
                (1e26,)
            }
        };
        (assign10660_e10139,)
    } else {
        (1e20,)
    }
};
        locals.var_neffedge_i = assign10660_e10141;
        locals.var_neffedge_i_rv = 0.0;

        let (assign10670_e10147,) = {
    if (locals.var_ctedge_p > 0.0) {
        (locals.var_ctedge_p,)
    } else {
        (0.0,)
    }
};
        locals.var_ctedge_i = assign10670_e10147;
        locals.var_ctedge_i_rv = 0.0;

        let (assign10680_e10153,) = {
    if (locals.var_betnedge_p > 0.0) {
        (locals.var_betnedge_p,)
    } else {
        (0.0,)
    }
};
        locals.var_betnedge_i = assign10680_e10153;
        locals.var_betnedge_i_rv = 0.0;

        locals.var_stbetedge_i = locals.var_stbetedge_p;
        locals.var_stbetedge_i_rv = 0.0;

        let (assign10700_e10160,) = {
    if (locals.var_psceedge_p > 0.0) {
        (locals.var_psceedge_p,)
    } else {
        (0.0,)
    }
};
        locals.var_psceedge_i = assign10700_e10160;
        locals.var_psceedge_i_rv = 0.0;

        let (assign10710_e10171,) = {
    if (locals.var_pscebedge_p > 0.0) {
        let (assign10710_e10169,) = {
            if (locals.var_pscebedge_p < 1.0) {
                (locals.var_pscebedge_p,)
            } else {
                (1.0,)
            }
        };
        (assign10710_e10169,)
    } else {
        (0.0,)
    }
};
        locals.var_pscebedge_i = assign10710_e10171;
        locals.var_pscebedge_i_rv = 0.0;

        let (assign10720_e10177,) = {
    if (locals.var_pscededge_p > 0.0) {
        (locals.var_pscededge_p,)
    } else {
        (0.0,)
    }
};
        locals.var_pscededge_i = assign10720_e10177;
        locals.var_pscededge_i_rv = 0.0;

        let (assign10730_e10183,) = {
    if (locals.var_cfedge_p > 0.0) {
        (locals.var_cfedge_p,)
    } else {
        (0.0,)
    }
};
        locals.var_cfedge_i = assign10730_e10183;
        locals.var_cfedge_i_rv = 0.0;

        let (assign10740_e10194,) = {
    if (locals.var_cfbedge_p > 0.0) {
        let (assign10740_e10192,) = {
            if (locals.var_cfbedge_p < 1.0) {
                (locals.var_cfbedge_p,)
            } else {
                (1.0,)
            }
        };
        (assign10740_e10192,)
    } else {
        (0.0,)
    }
};
        locals.var_cfbedge_i = assign10740_e10194;
        locals.var_cfbedge_i_rv = 0.0;

        let (assign10750_e10200,) = {
    if (locals.var_cfdedge_p > 0.0) {
        (locals.var_cfdedge_p,)
    } else {
        (0.0,)
    }
};
        locals.var_cfdedge_i = assign10750_e10200;
        locals.var_cfdedge_i_rv = 0.0;

        let (assign10890_e10244,) = {
    if (locals.var_cth_p > 0.0) {
        (locals.var_cth_p,)
    } else {
        (0.0,)
    }
};
        locals.var_cth_i = assign10890_e10244;
        locals.var_cth_i_rv = 0.0;

        let assign10910_e10248: f64 = (p.p31 * locals.var_nf_i);
        let (assign10910_e10255,) = {
    if (assign10910_e10248 > 0.0) {
        let assign10910_e10253: f64 = (p.p31 * locals.var_nf_i);
        (assign10910_e10253,)
    } else {
        (0.0,)
    }
};
        locals.var_mult_inst = assign10910_e10255;
        locals.var_mult_inst_rv = 0.0;

        locals.var_factuo_i = p.p16;
        locals.var_factuo_i_rv = 0.0;

        locals.var_delvto_i = p.p15;
        locals.var_delvto_i_rv = 0.0;

        locals.var_factuoedge_i = p.p18;
        locals.var_factuoedge_i_rv = 0.0;

        locals.var_delvtoedge_i = p.p17;
        locals.var_delvtoedge_i_rv = 0.0;

        let assign10960_e10262: f64 = if p.p44 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard150 = assign10960_e10262;
        locals.var_guard150_rv = 0.0;

        let (assign10970_e10266,) = {
    if (locals.var_guard150 != 0.0) {
        (locals.var_toxov_i,)
    } else {
        (locals.var_toxovd_i,)
    }
};
        locals.var_toxovd_i = assign10970_e10266;
        locals.var_toxovd_i_rv = 0.0;

        let (assign10980_e10270,) = {
    if (locals.var_guard150 != 0.0) {
        (locals.var_nov_i,)
    } else {
        (locals.var_novd_i,)
    }
};
        locals.var_novd_i = assign10980_e10270;
        locals.var_novd_i_rv = 0.0;

        let (assign10990_e10274,) = {
    if (locals.var_guard150 != 0.0) {
        (locals.var_agidl_i,)
    } else {
        (locals.var_agidld_i,)
    }
};
        locals.var_agidld_i = assign10990_e10274;
        locals.var_agidld_i_rv = 0.0;

        let (assign11000_e10278,) = {
    if (locals.var_guard150 != 0.0) {
        (locals.var_bgidl_i,)
    } else {
        (locals.var_bgidld_i,)
    }
};
        locals.var_bgidld_i = assign11000_e10278;
        locals.var_bgidld_i_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_12(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign11010_e10282,) = {
    if (locals.var_guard150 != 0.0) {
        (locals.var_stbgidl_i,)
    } else {
        (locals.var_stbgidld_i,)
    }
};
        locals.var_stbgidld_i = assign11010_e10282;
        locals.var_stbgidld_i_rv = 0.0;

        let (assign11020_e10286,) = {
    if (locals.var_guard150 != 0.0) {
        (locals.var_cgidl_i,)
    } else {
        (locals.var_cgidld_i,)
    }
};
        locals.var_cgidld_i = assign11020_e10286;
        locals.var_cgidld_i_rv = 0.0;

        let (assign11030_e10290,) = {
    if (locals.var_guard150 != 0.0) {
        (locals.var_igov_i,)
    } else {
        (locals.var_igovd_i,)
    }
};
        locals.var_igovd_i = assign11030_e10290;
        locals.var_igovd_i_rv = 0.0;

        let (assign11040_e10294,) = {
    if (locals.var_guard150 != 0.0) {
        (locals.var_gc2ov_i,)
    } else {
        (locals.var_gc2ovd_i,)
    }
};
        locals.var_gc2ovd_i = assign11040_e10294;
        locals.var_gc2ovd_i_rv = 0.0;

        let (assign11050_e10298,) = {
    if (locals.var_guard150 != 0.0) {
        (locals.var_gc3ov_i,)
    } else {
        (locals.var_gc3ovd_i,)
    }
};
        locals.var_gc3ovd_i = assign11050_e10298;
        locals.var_gc3ovd_i_rv = 0.0;

        let (assign11060_e10302,) = {
    if (locals.var_guard150 != 0.0) {
        (locals.var_cgov_i,)
    } else {
        (locals.var_cgovd_i,)
    }
};
        locals.var_cgovd_i = assign11060_e10302;
        locals.var_cgovd_i_rv = 0.0;

        let (assign11070_e10306,) = {
    if (locals.var_guard150 != 0.0) {
        (locals.var_fcgovacc_i,)
    } else {
        (locals.var_fcgovaccd_i,)
    }
};
        locals.var_fcgovaccd_i = assign11070_e10306;
        locals.var_fcgovaccd_i_rv = 0.0;

        let (assign11080_e10310,) = {
    if (locals.var_guard150 != 0.0) {
        (locals.var_cinr_i,)
    } else {
        (locals.var_cinrd_i,)
    }
};
        locals.var_cinrd_i = assign11080_e10310;
        locals.var_cinrd_i_rv = 0.0;

        let assign11100_e10317: f64 = (8.8541878176e-12 * locals.var_epsrox_i);
        locals.var_epsox = assign11100_e10317;
        locals.var_epsox_rv = 0.0;

        let assign11110_e10320: f64 = (locals.var_epsox / locals.var_tox_i);
        locals.var_coxprime = assign11110_e10320;
        locals.var_coxprime_rv = 0.0;

        let assign11120_e10323: f64 = (locals.var_tox_i * locals.var_tox_i);
        locals.var_tox_sq = assign11120_e10323;
        locals.var_tox_sq_rv = 0.0;

        let assign11130_e10326: f64 = (locals.var_coxprime / 1.6021918e-19);
        locals.var_cox_over_q = assign11130_e10326;
        locals.var_cox_over_q_rv = 0.0;

        let assign11140_e10329: f64 = (locals.var_facneffac_i * locals.var_neff_i);
        locals.var_neffac_i = assign11140_e10329;
        locals.var_neffac_i_rv = 0.0;

        let (assign11150_e10340,) = {
    if (locals.var_neffac_i > 1e20) {
        let (assign11150_e10338,) = {
            if (locals.var_neffac_i < 1e26) {
                (locals.var_neffac_i,)
            } else {
                (1e26,)
            }
        };
        (assign11150_e10338,)
    } else {
        (1e20,)
    }
};
        locals.var_neffac_i = assign11150_e10340;
        locals.var_neffac_i_rv = 0.0;

        locals.var_qq = 0.0;
        locals.var_qq_rv = 0.0;

        let assign11170_e10344: f64 = if p.p51 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard151 = assign11170_e10344;
        locals.var_guard151_rv = 0.0;

        let (assign11180_e10356,) = {
    if (locals.var_guard151 != 0.0) {
        let assign11180_e10348: f64 = (0.4 * 5.951993);
        let assign11180_e10350: f64 = (assign11180_e10348 * p.p51);
        let assign11180_e10353: f64 = (locals.var_coxprime).powf(0.6666666666666666);
        let assign11180_e10354: f64 = (assign11180_e10350 * assign11180_e10353);
        (assign11180_e10354,)
    } else {
        (locals.var_qq,)
    }
};
        locals.var_qq = assign11180_e10356;
        locals.var_qq_rv = 0.0;

        let assign11190_e10359: f64 = (-1.0);
        let assign11190_e10360: f64 = if locals.var_chnl_type == assign11190_e10359 { 1.0 } else { 0.0 };
        locals.var_guard152 = assign11190_e10360;
        locals.var_guard152_rv = 0.0;

        let (assign11200_e10370,) = {
    if ((locals.var_guard151 != 0.0) && (locals.var_guard152 != 0.0)) {
        let assign11200_e10366: f64 = (7.448711 / 5.951993);
        let assign11200_e10368: f64 = (assign11200_e10366 * locals.var_qq);
        (assign11200_e10368,)
    } else {
        (locals.var_qq,)
    }
};
        locals.var_qq = assign11200_e10370;
        locals.var_qq_rv = 0.0;

        let assign11210_e10373: f64 = (1e-8 * locals.var_coxprime);
        let assign11210_e10375: f64 = (assign11210_e10373 / locals.var_epssi);
        locals.var_e_eff0 = assign11210_e10375;
        locals.var_e_eff0_rv = 0.0;

        let assign11220_e10378: f64 = (0.5 * locals.var_feta_i);
        locals.var_eta_mu = assign11220_e10378;
        locals.var_eta_mu_rv = 0.0;

        locals.var_eta_mu1 = 0.5;
        locals.var_eta_mu1_rv = 0.0;

        let assign11240_e10382: f64 = (-1.0);
        let assign11240_e10383: f64 = if locals.var_chnl_type == assign11240_e10382 { 1.0 } else { 0.0 };
        locals.var_guard153 = assign11240_e10383;
        locals.var_guard153_rv = 0.0;

        let (assign11250_e10389,) = {
    if (locals.var_guard153 != 0.0) {
        let assign11250_e10387: f64 = (0.3333333333333333 * locals.var_feta_i);
        (assign11250_e10387,)
    } else {
        (locals.var_eta_mu,)
    }
};
        locals.var_eta_mu = assign11250_e10389;
        locals.var_eta_mu_rv = 0.0;

        let (assign11260_e10393,) = {
    if (locals.var_guard153 != 0.0) {
        (0.3333333333333333,)
    } else {
        (locals.var_eta_mu1,)
    }
};
        locals.var_eta_mu1 = assign11260_e10393;
        locals.var_eta_mu1_rv = 0.0;

        let assign11270_e10396: f64 = (-2.0);
        let assign11270_e10398: f64 = (assign11270_e10396 / locals.var_ax_i);
        let assign11270_e10400: f64 = (assign11270_e10398 + 1.0);
        let assign11270_e10401: f64 = (2.0_f64).powf(assign11270_e10400);
        let assign11270_e10403: f64 = (assign11270_e10401 - 1.0);
        locals.var_temp = assign11270_e10403;
        locals.var_temp_rv = 0.0;

        let assign11280_e10406: f64 = (locals.var_temp - 1.0);
        let assign11280_e10409: f64 = (locals.var_temp - 1.0);
        let assign11280_e10410: f64 = (assign11280_e10406 * assign11280_e10409);
        let assign11280_e10413: f64 = (4.0 * locals.var_temp);
        let (assign11280_e10420,) = {
    if (assign11280_e10413 > 0.0001) {
        let assign11280_e10418: f64 = (4.0 * locals.var_temp);
        (assign11280_e10418,)
    } else {
        (0.0001,)
    }
};
        let assign11280_e10421: f64 = (assign11280_e10410 / assign11280_e10420);
        locals.var_ar = assign11280_e10421;
        locals.var_ar_rv = 0.0;

        let assign11290_e10424: f64 = (-2.0);
        let assign11290_e10426: f64 = (assign11290_e10424 / locals.var_axac_i);
        let assign11290_e10428: f64 = (assign11290_e10426 + 1.0);
        let assign11290_e10429: f64 = (2.0_f64).powf(assign11290_e10428);
        let assign11290_e10431: f64 = (assign11290_e10429 - 1.0);
        locals.var_temp = assign11290_e10431;
        locals.var_temp_rv = 0.0;

        let assign11300_e10434: f64 = (locals.var_temp - 1.0);
        let assign11300_e10437: f64 = (locals.var_temp - 1.0);
        let assign11300_e10438: f64 = (assign11300_e10434 * assign11300_e10437);
        let assign11300_e10441: f64 = (4.0 * locals.var_temp);
        let (assign11300_e10448,) = {
    if (assign11300_e10441 > 0.0001) {
        let assign11300_e10446: f64 = (4.0 * locals.var_temp);
        (assign11300_e10446,)
    } else {
        (0.0001,)
    }
};
        let assign11300_e10449: f64 = (assign11300_e10438 / assign11300_e10448);
        locals.var_arac = assign11300_e10449;
        locals.var_arac_rv = 0.0;

        let assign11310_e10452: f64 = (1.0 / locals.var_vp_i);
        locals.var_inv_vp = assign11310_e10452;
        locals.var_inv_vp_rv = 0.0;

        let assign11320_e10455: f64 = (locals.var_epsox / locals.var_toxov_i);
        locals.var_coxovprime = assign11320_e10455;
        locals.var_coxovprime_rv = 0.0;

        let assign11330_e10458: f64 = (locals.var_epsox / locals.var_toxovd_i);
        locals.var_coxovprime_d = assign11330_e10458;
        locals.var_coxovprime_d_rv = 0.0;

        let assign11340_e10461: f64 = (2.0 * 1.6021918e-19);
        let assign11340_e10463: f64 = (assign11340_e10461 * locals.var_nov_i);
        let assign11340_e10465: f64 = (assign11340_e10463 * locals.var_epssi);
        let assign11340_e10467: f64 = (assign11340_e10465 * locals.var_inv_phita);
        let assign11340_e10468: f64 = (assign11340_e10467).sqrt();
        let assign11340_e10470: f64 = (assign11340_e10468 / locals.var_coxovprime);
        locals.var_gov_s = assign11340_e10470;
        locals.var_gov_s_rv = 0.0;

        let assign11350_e10473: f64 = (2.0 * 1.6021918e-19);
        let assign11350_e10475: f64 = (assign11350_e10473 * locals.var_novd_i);
        let assign11350_e10477: f64 = (assign11350_e10475 * locals.var_epssi);
        let assign11350_e10479: f64 = (assign11350_e10477 * locals.var_inv_phita);
        let assign11350_e10480: f64 = (assign11350_e10479).sqrt();
        let assign11350_e10482: f64 = (assign11350_e10480 / locals.var_coxovprime_d);
        locals.var_gov_d = assign11350_e10482;
        locals.var_gov_d_rv = 0.0;

        let assign11360_e10485: f64 = (locals.var_gov_s * locals.var_gov_s);
        locals.var_gov2_s = assign11360_e10485;
        locals.var_gov2_s_rv = 0.0;

        let assign11370_e10488: f64 = (locals.var_gov_d * locals.var_gov_d);
        locals.var_gov2_d = assign11370_e10488;
        locals.var_gov2_d_rv = 0.0;

        let assign11380_e10491: f64 = (locals.var_cgovaccg_i * 0.005);
        let assign11380_e10493: f64 = (assign11380_e10491 * locals.var_inv_phita);
        let assign11380_e10494: f64 = (assign11380_e10493).exp();
        let assign11380_e10496: f64 = (assign11380_e10494 - 1.0);
        let assign11380_e10497: f64 = (assign11380_e10496).ln();
        let assign11380_e10499: f64 = (assign11380_e10497 / locals.var_cgovaccg_i);
        let assign11380_e10502: f64 = (0.005 * locals.var_inv_phita);
        let assign11380_e10503: f64 = (assign11380_e10502).exp();
        let assign11380_e10505: f64 = (assign11380_e10503 - 1.0);
        let assign11380_e10506: f64 = (assign11380_e10505).ln();
        let assign11380_e10507: f64 = (assign11380_e10499 - assign11380_e10506);
        locals.var_dxgb_ov_th = assign11380_e10507;
        locals.var_dxgb_ov_th_rv = 0.0;

        let assign11390_e10510: f64 = (0.5 * locals.var_gov_s);
        let assign11390_e10511: f64 = (assign11390_e10510).ln();
        let assign11390_e10513: f64 = (assign11390_e10511 + locals.var_dxgb_ov_th);
        locals.var_dxgb_ov_s = assign11390_e10513;
        locals.var_dxgb_ov_s_rv = 0.0;

        let assign11400_e10516: f64 = (0.5 * locals.var_gov_d);
        let assign11400_e10517: f64 = (assign11400_e10516).ln();
        let assign11400_e10519: f64 = (assign11400_e10517 + locals.var_dxgb_ov_th);
        locals.var_dxgb_ov_d = assign11400_e10519;
        locals.var_dxgb_ov_d_rv = 0.0;

        let assign11410_e10522: f64 = (1.0 / locals.var_gov_s);
        locals.var_inv_gov = assign11410_e10522;
        locals.var_inv_gov_rv = 0.0;

        let assign11420_e10525: f64 = (3.1 * locals.var_gov_s);
        let assign11420_e10527: f64 = (assign11420_e10525 + 8.5);
        locals.var_sp_ov_eps = assign11420_e10527;
        locals.var_sp_ov_eps_rv = 0.0;

        let assign11430_e10530: f64 = (locals.var_sp_ov_eps * locals.var_sp_ov_eps);
        locals.var_sp_ov_eps2_s = assign11430_e10530;
        locals.var_sp_ov_eps2_s_rv = 0.0;

        let assign11440_e10533: f64 = (0.5 * locals.var_sp_ov_eps);
        locals.var_sp_ov_delta = assign11440_e10533;
        locals.var_sp_ov_delta_rv = 0.0;

        let assign11450_e10536: f64 = if locals.var_inv_gov < 0.06 { 1.0 } else { 0.0 };
        locals.var_guard154 = assign11450_e10536;
        locals.var_guard154_rv = 0.0;

        let (assign11460_e10542,) = {
    if (locals.var_guard154 != 0.0) {
        let assign11460_e10540: f64 = (64.0 * locals.var_inv_gov);
        (assign11460_e10540,)
    } else {
        (locals.var_sp_ov_a_s,)
    }
};
        locals.var_sp_ov_a_s = assign11460_e10542;
        locals.var_sp_ov_a_s_rv = 0.0;

        let assign11470_e10545: f64 = if locals.var_inv_gov <= 0.45 { 1.0 } else { 0.0 };
        locals.var_guard155 = assign11470_e10545;
        locals.var_guard155_rv = 0.0;

        let (assign11480_e10556,) = {
    if ((locals.var_guard154 == 0.0) && (locals.var_guard155 != 0.0)) {
        let assign11480_e10552: f64 = (22.0 * locals.var_inv_gov);
        let assign11480_e10554: f64 = (assign11480_e10552 + 3.0);
        (assign11480_e10554,)
    } else {
        (locals.var_sp_ov_a_s,)
    }
};
        locals.var_sp_ov_a_s = assign11480_e10556;
        locals.var_sp_ov_a_s_rv = 0.0;

        let assign11490_e10559: f64 = if locals.var_inv_gov <= 1.6 { 1.0 } else { 0.0 };
        locals.var_guard156 = assign11490_e10559;
        locals.var_guard156_rv = 0.0;

        let (assign11500_e10574,) = {
    if (((locals.var_guard154 == 0.0) && (locals.var_guard155 == 0.0)) && (locals.var_guard156 != 0.0)) {
        let assign11500_e10568: f64 = (-7.2);
        let assign11500_e10570: f64 = (assign11500_e10568 * locals.var_inv_gov);
        let assign11500_e10572: f64 = (assign11500_e10570 + 15.5);
        (assign11500_e10572,)
    } else {
        (locals.var_sp_ov_a_s,)
    }
};
        locals.var_sp_ov_a_s = assign11500_e10574;
        locals.var_sp_ov_a_s_rv = 0.0;

        let (assign11510_e10585,) = {
    if (((locals.var_guard154 == 0.0) && (locals.var_guard155 == 0.0)) && (locals.var_guard156 == 0.0)) {
        (locals.var_gov_s,)
    } else {
        (locals.var_sp_ov_a_s,)
    }
};
        locals.var_sp_ov_a_s = assign11510_e10585;
        locals.var_sp_ov_a_s_rv = 0.0;

        let assign11520_e10589: f64 = (locals.var_gov2_s * 0.5);
        let assign11520_e10590: f64 = (locals.var_sp_ov_delta + assign11520_e10589);
        let assign11520_e10595: f64 = (locals.var_gov2_s * 0.25);
        let assign11520_e10596: f64 = (locals.var_sp_ov_delta + assign11520_e10595);
        let assign11520_e10598: f64 = (assign11520_e10596 + locals.var_sp_ov_a_s);
        let assign11520_e10599: f64 = (assign11520_e10598).sqrt();
        let assign11520_e10600: f64 = (locals.var_gov_s * assign11520_e10599);
        let assign11520_e10601: f64 = (assign11520_e10590 - assign11520_e10600);
        locals.var_sp_ov_delta1_s = assign11520_e10601;
        locals.var_sp_ov_delta1_s_rv = 0.0;

        let assign11530_e10604: f64 = (1.0 / locals.var_gov_d);
        locals.var_inv_gov = assign11530_e10604;
        locals.var_inv_gov_rv = 0.0;

        let assign11540_e10607: f64 = (3.1 * locals.var_gov_d);
        let assign11540_e10609: f64 = (assign11540_e10607 + 8.5);
        locals.var_sp_ov_eps = assign11540_e10609;
        locals.var_sp_ov_eps_rv = 0.0;

        let assign11550_e10612: f64 = (locals.var_sp_ov_eps * locals.var_sp_ov_eps);
        locals.var_sp_ov_eps2_d = assign11550_e10612;
        locals.var_sp_ov_eps2_d_rv = 0.0;

        let assign11560_e10615: f64 = (0.5 * locals.var_sp_ov_eps);
        locals.var_sp_ov_delta = assign11560_e10615;
        locals.var_sp_ov_delta_rv = 0.0;

        let assign11570_e10618: f64 = if locals.var_inv_gov < 0.06 { 1.0 } else { 0.0 };
        locals.var_guard157 = assign11570_e10618;
        locals.var_guard157_rv = 0.0;

        let (assign11580_e10624,) = {
    if (locals.var_guard157 != 0.0) {
        let assign11580_e10622: f64 = (64.0 * locals.var_inv_gov);
        (assign11580_e10622,)
    } else {
        (locals.var_sp_ov_a_d,)
    }
};
        locals.var_sp_ov_a_d = assign11580_e10624;
        locals.var_sp_ov_a_d_rv = 0.0;

        let assign11590_e10627: f64 = if locals.var_inv_gov <= 0.45 { 1.0 } else { 0.0 };
        locals.var_guard158 = assign11590_e10627;
        locals.var_guard158_rv = 0.0;

        let (assign11600_e10638,) = {
    if ((locals.var_guard157 == 0.0) && (locals.var_guard158 != 0.0)) {
        let assign11600_e10634: f64 = (22.0 * locals.var_inv_gov);
        let assign11600_e10636: f64 = (assign11600_e10634 + 3.0);
        (assign11600_e10636,)
    } else {
        (locals.var_sp_ov_a_d,)
    }
};
        locals.var_sp_ov_a_d = assign11600_e10638;
        locals.var_sp_ov_a_d_rv = 0.0;

        let assign11610_e10641: f64 = if locals.var_inv_gov <= 1.6 { 1.0 } else { 0.0 };
        locals.var_guard159 = assign11610_e10641;
        locals.var_guard159_rv = 0.0;

        let (assign11620_e10656,) = {
    if (((locals.var_guard157 == 0.0) && (locals.var_guard158 == 0.0)) && (locals.var_guard159 != 0.0)) {
        let assign11620_e10650: f64 = (-7.2);
        let assign11620_e10652: f64 = (assign11620_e10650 * locals.var_inv_gov);
        let assign11620_e10654: f64 = (assign11620_e10652 + 15.5);
        (assign11620_e10654,)
    } else {
        (locals.var_sp_ov_a_d,)
    }
};
        locals.var_sp_ov_a_d = assign11620_e10656;
        locals.var_sp_ov_a_d_rv = 0.0;

        let (assign11630_e10667,) = {
    if (((locals.var_guard157 == 0.0) && (locals.var_guard158 == 0.0)) && (locals.var_guard159 == 0.0)) {
        (locals.var_gov_d,)
    } else {
        (locals.var_sp_ov_a_d,)
    }
};
        locals.var_sp_ov_a_d = assign11630_e10667;
        locals.var_sp_ov_a_d_rv = 0.0;

        let assign11640_e10671: f64 = (locals.var_gov2_d * 0.5);
        let assign11640_e10672: f64 = (locals.var_sp_ov_delta + assign11640_e10671);
        let assign11640_e10677: f64 = (locals.var_gov2_d * 0.25);
        let assign11640_e10678: f64 = (locals.var_sp_ov_delta + assign11640_e10677);
        let assign11640_e10680: f64 = (assign11640_e10678 + locals.var_sp_ov_a_d);
        let assign11640_e10681: f64 = (assign11640_e10680).sqrt();
        let assign11640_e10682: f64 = (locals.var_gov_d * assign11640_e10681);
        let assign11640_e10683: f64 = (assign11640_e10672 - assign11640_e10682);
        locals.var_sp_ov_delta1_d = assign11640_e10683;
        locals.var_sp_ov_delta1_d_rv = 0.0;

        let assign11650_e10686: f64 = (1.0 / locals.var_chib_i);
        locals.var_inv_chib = assign11650_e10686;
        locals.var_inv_chib_rv = 0.0;

        let assign11660_e10689: f64 = (4.0 * 0.3333333333333333);
        let assign11660_e10692: f64 = (2.0 * 1.6021918e-19);
        let assign11660_e10694: f64 = (assign11660_e10692 * 9.1093826e-31);
        let assign11660_e10696: f64 = (assign11660_e10694 * locals.var_chib_i);
        let assign11660_e10697: f64 = (assign11660_e10696).sqrt();
        let assign11660_e10698: f64 = (assign11660_e10689 * assign11660_e10697);
        let assign11660_e10700: f64 = (assign11660_e10698 / 1.05457168e-34);
        locals.var_b_fact = assign11660_e10700;
        locals.var_b_fact_rv = 0.0;

        let assign11670_e10703: f64 = (locals.var_b_fact * locals.var_tox_i);
        locals.var_bch = assign11670_e10703;
        locals.var_bch_rv = 0.0;

        let assign11680_e10706: f64 = (locals.var_b_fact * locals.var_toxov_i);
        locals.var_bov = assign11680_e10706;
        locals.var_bov_rv = 0.0;

        let assign11690_e10709: f64 = (locals.var_b_fact * locals.var_toxovd_i);
        locals.var_bov_d = assign11690_e10709;
        locals.var_bov_d_rv = 0.0;

        locals.var_gcq = 0.0;
        locals.var_gcq_rv = 0.0;

        let assign11710_e10713: f64 = if locals.var_gc3_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard160 = assign11710_e10713;
        locals.var_guard160_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_13(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        locals: &mut StampLocals,
    ) {
        let nv4 = ctx.node_voltage(nodes[4]);
        let (assign11720_e10722,) = {
    if (locals.var_guard160 != 0.0) {
        let assign11720_e10716: f64 = (-0.495);
        let assign11720_e10718: f64 = (assign11720_e10716 * locals.var_gc2_i);
        let assign11720_e10720: f64 = (assign11720_e10718 / locals.var_gc3_i);
        (assign11720_e10720,)
    } else {
        (locals.var_gcq,)
    }
};
        locals.var_gcq = assign11720_e10722;
        locals.var_gcq_rv = 0.0;

        locals.var_gcqov = 0.0;
        locals.var_gcqov_rv = 0.0;

        let assign11740_e10726: f64 = if locals.var_gc3ov_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard161 = assign11740_e10726;
        locals.var_guard161_rv = 0.0;

        let (assign11750_e10735,) = {
    if (locals.var_guard161 != 0.0) {
        let assign11750_e10729: f64 = (-0.495);
        let assign11750_e10731: f64 = (assign11750_e10729 * locals.var_gc2ov_i);
        let assign11750_e10733: f64 = (assign11750_e10731 / locals.var_gc3ov_i);
        (assign11750_e10733,)
    } else {
        (locals.var_gcqov,)
    }
};
        locals.var_gcqov = assign11750_e10735;
        locals.var_gcqov_rv = 0.0;

        let assign11760_e10738: f64 = if locals.var_gc3ovd_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard162 = assign11760_e10738;
        locals.var_guard162_rv = 0.0;

        let (assign11770_e10747,) = {
    if (locals.var_guard162 != 0.0) {
        let assign11770_e10741: f64 = (-0.495);
        let assign11770_e10743: f64 = (assign11770_e10741 * locals.var_gc2ovd_i);
        let assign11770_e10745: f64 = (assign11770_e10743 / locals.var_gc3ovd_i);
        (assign11770_e10745,)
    } else {
        (locals.var_gcqovd,)
    }
};
        locals.var_gcqovd = assign11770_e10747;
        locals.var_gcqovd_rv = 0.0;

        let assign11780_e10750: f64 = (locals.var_rta).powf(locals.var_stig_i);
        locals.var_tf_ig = assign11780_e10750;
        locals.var_tf_ig_rv = 0.0;

        let assign11790_e10753: f64 = (locals.var_iginv_i * locals.var_tf_ig);
        locals.var_iginv_i = assign11790_e10753;
        locals.var_iginv_i_rv = 0.0;

        let assign11800_e10756: f64 = (locals.var_igov_i * locals.var_tf_ig);
        locals.var_igov_i = assign11800_e10756;
        locals.var_igov_i_rv = 0.0;

        let assign11810_e10759: f64 = (locals.var_igovd_i * locals.var_tf_ig);
        locals.var_igovd_i = assign11810_e10759;
        locals.var_igovd_i_rv = 0.0;

        let assign11840_e10777: f64 = (locals.var_stbgidl_i * locals.var_delta);
        let assign11840_e10778: f64 = (1.0 + assign11840_e10777);
        let (assign11840_e10787,) = {
    if (assign11840_e10778 > 0.0) {
        let assign11840_e10784: f64 = (locals.var_stbgidl_i * locals.var_delta);
        let assign11840_e10785: f64 = (1.0 + assign11840_e10784);
        (assign11840_e10785,)
    } else {
        (0.0,)
    }
};
        locals.var_b_fact = assign11840_e10787;
        locals.var_b_fact_rv = 0.0;

        let assign11850_e10790: f64 = (locals.var_bgidl_i * locals.var_b_fact);
        locals.var_bgidl_t = assign11850_e10790;
        locals.var_bgidl_t_rv = 0.0;

        let assign11860_e10793: f64 = (locals.var_bgidl_t * locals.var_toxov_i);
        let assign11860_e10795: f64 = (assign11860_e10793 * 500000000.0);
        locals.var_bgidls = assign11860_e10795;
        locals.var_bgidls_rv = 0.0;

        let assign11870_e10799: f64 = (locals.var_stbgidld_i * locals.var_delta);
        let assign11870_e10800: f64 = (1.0 + assign11870_e10799);
        let (assign11870_e10809,) = {
    if (assign11870_e10800 > 0.0) {
        let assign11870_e10806: f64 = (locals.var_stbgidld_i * locals.var_delta);
        let assign11870_e10807: f64 = (1.0 + assign11870_e10806);
        (assign11870_e10807,)
    } else {
        (0.0,)
    }
};
        locals.var_b_fact = assign11870_e10809;
        locals.var_b_fact_rv = 0.0;

        let assign11880_e10812: f64 = (locals.var_bgidld_i * locals.var_b_fact);
        locals.var_bgidld_t = assign11880_e10812;
        locals.var_bgidld_t_rv = 0.0;

        let assign11890_e10815: f64 = (locals.var_bgidld_t * locals.var_toxovd_i);
        let assign11890_e10817: f64 = (assign11890_e10815 * 500000000.0);
        locals.var_bgidlds = assign11890_e10817;
        locals.var_bgidlds_rv = 0.0;

        locals.var_vinr_max = 0.0;
        locals.var_vinr_max_rv = 0.0;

        let assign11910_e10821: f64 = if locals.var_fcinracc_i > 1e-10 { 1.0 } else { 0.0 };
        locals.var_guard163 = assign11910_e10821;
        locals.var_guard163_rv = 0.0;

        let (assign11920_e10827,) = {
    if (locals.var_guard163 != 0.0) {
        let assign11920_e10825: f64 = (0.75 / locals.var_fcinracc_i);
        (assign11920_e10825,)
    } else {
        (locals.var_vinr_max,)
    }
};
        locals.var_vinr_max = assign11920_e10827;
        locals.var_vinr_max_rv = 0.0;

        let assign11930_e10830: f64 = (locals.var_axinr_i * locals.var_axinr_i);
        locals.var_ainr = assign11930_e10830;
        locals.var_ainr_rv = 0.0;

        locals.var_temp__blk949 = 0.0;
        locals.var_temp__blk949_dn4 = 0.0;
        locals.var_temp__blk949_dn6 = 0.0;
        locals.var_temp__blk949_dn7 = 0.0;
        locals.var_temp__blk949_dn8 = 0.0;
        locals.var_temp__blk949_dn9 = 0.0;
        locals.var_temp__blk949_rv = 0.0;

        locals.var_temp1 = 0.0;
        locals.var_temp1_dn4 = 0.0;
        locals.var_temp1_dn6 = 0.0;
        locals.var_temp1_dn7 = 0.0;
        locals.var_temp1_dn8 = 0.0;
        locals.var_temp1_dn9 = 0.0;
        locals.var_temp1_rv = 0.0;

        locals.var_temp2 = 0.0;
        locals.var_temp2_dn4 = 0.0;
        locals.var_temp2_dn6 = 0.0;
        locals.var_temp2_dn7 = 0.0;
        locals.var_temp2_dn8 = 0.0;
        locals.var_temp2_dn9 = 0.0;
        locals.var_temp2_rv = 0.0;

        let assign39430_e52953: f64 = (locals.var_tka + (nv4 - 0.0));
        locals.var_tkd = assign39430_e52953;
        locals.var_tkd_dn4 = 1.0;
        locals.var_tkd_rv = 0.0;

        let assign39440_e52956: f64 = (locals.var_tkd * locals.var_tkd);
        locals.var_tkd_sq = assign39440_e52956;
        locals.var_tkd_sq_dn4 = ((locals.var_tkd_dn4 * locals.var_tkd) + (locals.var_tkd * locals.var_tkd_dn4));
        locals.var_tkd_sq_rv = 0.0;

        let assign39450_e52959: f64 = (locals.var_tkd - locals.var_tkr);
        locals.var_delt = assign39450_e52959;
        locals.var_delt_dn4 = locals.var_tkd_dn4;
        locals.var_delt_rv = 0.0;

        let assign39460_e52962: f64 = (locals.var_tkr / locals.var_tkd);
        locals.var_rtn = assign39460_e52962;
        locals.var_rtn_dn4 = (-((locals.var_tkr * locals.var_tkd_dn4) / (locals.var_tkd * locals.var_tkd)));
        locals.var_rtn_rv = 0.0;

        let assign39470_e52964: f64 = (locals.var_rtn).ln();
        locals.var_ln_rtn = assign39470_e52964;
        locals.var_ln_rtn_dn4 = (locals.var_rtn_dn4 / locals.var_rtn);
        locals.var_ln_rtn_rv = 0.0;

        let assign39480_e52967: f64 = (locals.var_tkd * 1.3806505e-23);
        let assign39480_e52969: f64 = (assign39480_e52967 / 1.6021918e-19);
        locals.var_phit = assign39480_e52969;
        locals.var_phit_dn4 = ((locals.var_tkd_dn4 * 1.3806505e-23) / 1.6021918e-19);
        locals.var_phit_rv = 0.0;

        let assign39490_e52972: f64 = (1.0 / locals.var_phit);
        locals.var_inv_phit = assign39490_e52972;
        locals.var_inv_phit_dn4 = (-(locals.var_phit_dn4 / (locals.var_phit * locals.var_phit)));
        locals.var_inv_phit_rv = 0.0;

        let assign39500_e52976: f64 = (9.025e-5 * locals.var_tkd);
        let assign39500_e52977: f64 = (1.179 - assign39500_e52976);
        let assign39500_e52980: f64 = (3.05e-7 * locals.var_tkd_sq);
        let assign39500_e52981: f64 = (assign39500_e52977 - assign39500_e52980);
        locals.var_eg = assign39500_e52981;
        locals.var_eg_dn4 = ((-(9.025e-5 * locals.var_tkd_dn4)) - (3.05e-7 * locals.var_tkd_sq_dn4));
        locals.var_eg_rv = 0.0;

        let assign39510_e52985: f64 = (0.00045 * locals.var_tkd);
        let assign39510_e52986: f64 = (1.045 + assign39510_e52985);
        let assign39510_e52990: f64 = (0.0014 * locals.var_tkd);
        let assign39510_e52991: f64 = (0.523 + assign39510_e52990);
        let assign39510_e52994: f64 = (1.48e-6 * locals.var_tkd_sq);
        let assign39510_e52995: f64 = (assign39510_e52991 - assign39510_e52994);
        let assign39510_e52996: f64 = (assign39510_e52986 * assign39510_e52995);
        let assign39510_e52998: f64 = (assign39510_e52996 * locals.var_tkd_sq);
        let assign39510_e53000: f64 = (assign39510_e52998 / 90000.0);
        locals.var_phibfac = assign39510_e53000;
        locals.var_phibfac_dn4 = ((((((0.00045 * locals.var_tkd_dn4) * assign39510_e52995) + (assign39510_e52986 * ((0.0014 * locals.var_tkd_dn4) - (1.48e-6 * locals.var_tkd_sq_dn4)))) * locals.var_tkd_sq) + (assign39510_e52996 * locals.var_tkd_sq_dn4)) / 90000.0);
        locals.var_phibfac_rv = 0.0;

        let (assign39520_e53006, assign39520_e53006_d_n4,) = {
    if (locals.var_phibfac > 0.001) {
        (locals.var_phibfac, locals.var_phibfac_dn4,)
    } else {
        (0.001, 0.0,)
    }
};
        locals.var_phibfac = assign39520_e53006;
        locals.var_phibfac_dn4 = assign39520_e53006_d_n4;
        locals.var_phibfac_rv = 0.0;

        let assign39540_e53014: f64 = (locals.var_eg + locals.var_dphib_i);
        let assign39540_e53017: f64 = (2.0 * locals.var_phit);
        let assign39540_e53021: f64 = (-0.75);
        let assign39540_e53022: f64 = (locals.var_phibfac).powf(assign39540_e53021);
        let assign39540_e53023: f64 = (locals.var_neff_i * assign39540_e53022);
        let assign39540_e53025: f64 = (assign39540_e53023 * 4e-26);
        let assign39540_e53026: f64 = (assign39540_e53025).ln();
        let assign39540_e53027: f64 = (assign39540_e53017 * assign39540_e53026);
        let assign39540_e53028: f64 = (assign39540_e53014 + assign39540_e53027);
        locals.var_phib_dc = assign39540_e53028;
        locals.var_phib_dc_dn4 = (locals.var_eg_dn4 + (((2.0 * locals.var_phit_dn4) * assign39540_e53026) + (assign39540_e53017 * (((locals.var_neff_i * if 0.0 == 0.0 && ((assign39540_e53021) as f64).is_finite() && ((assign39540_e53021) as f64).fract() == 0.0 { if assign39540_e53021 == 0.0 { 0.0 } else { (assign39540_e53021 * ((locals.var_phibfac).powf(assign39540_e53021 - 1.0) * locals.var_phibfac_dn4)) } } else { (assign39540_e53022 * (assign39540_e53021 * (locals.var_phibfac_dn4 / locals.var_phibfac))) }) * 4e-26) / assign39540_e53025))));
        locals.var_phib_dc_rv = 0.0;

        let (assign39550_e53034, assign39550_e53034_d_n4,) = {
    if (locals.var_phib_dc > 0.05) {
        (locals.var_phib_dc, locals.var_phib_dc_dn4,)
    } else {
        (0.05, 0.0,)
    }
};
        locals.var_phib_dc = assign39550_e53034;
        locals.var_phib_dc_dn4 = assign39550_e53034_d_n4;
        locals.var_phib_dc_rv = 0.0;

        let assign39560_e53037: f64 = (2.0 * 1.6021918e-19);
        let assign39560_e53039: f64 = (assign39560_e53037 * locals.var_neff_i);
        let assign39560_e53041: f64 = (assign39560_e53039 * locals.var_epssi);
        let assign39560_e53043: f64 = (assign39560_e53041 * locals.var_inv_phit);
        let assign39560_e53044: f64 = (assign39560_e53043).sqrt();
        let assign39560_e53046: f64 = (assign39560_e53044 / locals.var_coxprime);
        locals.var_g_0_dc = assign39560_e53046;
        locals.var_g_0_dc_dn4 = (((assign39560_e53041 * locals.var_inv_phit_dn4) / (2.0 * assign39560_e53044)) / locals.var_coxprime);
        locals.var_g_0_dc_rv = 0.0;

        locals.var_kp = 0.0;
        locals.var_kp_dn4 = 0.0;
        locals.var_kp_rv = 0.0;

        locals.var_np = 0.0;
        locals.var_np_rv = 0.0;

        let assign39590_e53051: f64 = if locals.var_np_i > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1024 = assign39590_e53051;
        locals.var_guard1024_rv = 0.0;

        let (assign39600_e53057,) = {
    if (locals.var_guard1024 != 0.0) {
        let assign39600_e53055: f64 = (80000000.0 / locals.var_tox_sq);
        (assign39600_e53055,)
    } else {
        (locals.var_arg2max,)
    }
};
        locals.var_arg2max = assign39600_e53057;
        locals.var_arg2max_rv = 0.0;

        let (assign39610_e53066,) = {
    if (locals.var_guard1024 != 0.0) {
        let (assign39610_e53064,) = {
            if (locals.var_np_i > locals.var_arg2max) {
                (locals.var_np_i,)
            } else {
                (locals.var_arg2max,)
            }
        };
        (assign39610_e53064,)
    } else {
        (locals.var_np,)
    }
};
        locals.var_np = assign39610_e53066;
        locals.var_np_rv = 0.0;

        let (assign39620_e53075,) = {
    if (locals.var_guard1024 != 0.0) {
        let (assign39620_e53073,) = {
            if (5e24 > locals.var_np) {
                (5e24,)
            } else {
                (locals.var_np,)
            }
        };
        (assign39620_e53073,)
    } else {
        (locals.var_np,)
    }
};
        locals.var_np = assign39620_e53075;
        locals.var_np_rv = 0.0;

        let (assign39630_e53091, assign39630_e53091_d_n4,) = {
    if (locals.var_guard1024 != 0.0) {
        let assign39630_e53079: f64 = (2.0 * locals.var_coxprime);
        let assign39630_e53081: f64 = (assign39630_e53079 * locals.var_coxprime);
        let assign39630_e53083: f64 = (assign39630_e53081 * locals.var_phit);
        let assign39630_e53086: f64 = (1.6021918e-19 * locals.var_np);
        let assign39630_e53088: f64 = (assign39630_e53086 * locals.var_epssi);
        let assign39630_e53089: f64 = (assign39630_e53083 / assign39630_e53088);
        (assign39630_e53089, ((assign39630_e53081 * locals.var_phit_dn4) / assign39630_e53088),)
    } else {
        (locals.var_kp, locals.var_kp_dn4,)
    }
};
        locals.var_kp = assign39630_e53091;
        locals.var_kp_dn4 = assign39630_e53091_d_n4;
        locals.var_kp_rv = 0.0;

        let assign39640_e53094: f64 = (100.0 * locals.var_phit);
        let assign39640_e53096: f64 = (assign39640_e53094 * locals.var_phit);
        locals.var_qlim2 = assign39640_e53096;
        locals.var_qlim2_dn4 = (((100.0 * locals.var_phit_dn4) * locals.var_phit) + (assign39640_e53094 * locals.var_phit_dn4));
        locals.var_qlim2_rv = 0.0;

        let assign39650_e53099: f64 = if p.p51 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1025 = assign39650_e53099;
        locals.var_guard1025_rv = 0.0;

        let (assign39660_e53110, assign39660_e53110_d_n4,) = {
    if (locals.var_guard1025 != 0.0) {
        let assign39660_e53103: f64 = (locals.var_phit * locals.var_g_0_dc);
        let assign39660_e53105: f64 = (assign39660_e53103 * locals.var_g_0_dc);
        let assign39660_e53107: f64 = (assign39660_e53105 * locals.var_phib_dc);
        let assign39660_e53108: f64 = (assign39660_e53107).sqrt();
        (assign39660_e53108, (((((((locals.var_phit_dn4 * locals.var_g_0_dc) + (locals.var_phit * locals.var_g_0_dc_dn4)) * locals.var_g_0_dc) + (assign39660_e53103 * locals.var_g_0_dc_dn4)) * locals.var_phib_dc) + (assign39660_e53105 * locals.var_phib_dc_dn4)) / (2.0 * assign39660_e53108)),)
    } else {
        (locals.var_qb0, locals.var_qb0_dn4,)
    }
};
        locals.var_qb0 = assign39660_e53110;
        locals.var_qb0_dn4 = assign39660_e53110_d_n4;
        locals.var_qb0_rv = 0.0;

        let (assign39670_e53120, assign39670_e53120_d_n4,) = {
    if (locals.var_guard1025 != 0.0) {
        let assign39670_e53114: f64 = (0.75 * locals.var_qq);
        let assign39670_e53117: f64 = (locals.var_qb0).powf(0.6666666666666666);
        let assign39670_e53118: f64 = (assign39670_e53114 * assign39670_e53117);
        (assign39670_e53118, (assign39670_e53114 * if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((locals.var_qb0).powf(0.6666666666666666 - 1.0) * locals.var_qb0_dn4)) } } else { (assign39670_e53117 * (0.6666666666666666 * (locals.var_qb0_dn4 / locals.var_qb0))) }),)
    } else {
        (locals.var_dphibq, locals.var_dphibq_dn4,)
    }
};
        locals.var_dphibq = assign39670_e53120;
        locals.var_dphibq_dn4 = assign39670_e53120_d_n4;
        locals.var_dphibq_rv = 0.0;

        let (assign39680_e53126, assign39680_e53126_d_n4,) = {
    if (locals.var_guard1025 != 0.0) {
        let assign39680_e53124: f64 = (locals.var_phib_dc + locals.var_dphibq);
        (assign39680_e53124, (locals.var_phib_dc_dn4 + locals.var_dphibq_dn4),)
    } else {
        (locals.var_phib_dc, locals.var_phib_dc_dn4,)
    }
};
        locals.var_phib_dc = assign39680_e53126;
        locals.var_phib_dc_dn4 = assign39680_e53126_d_n4;
        locals.var_phib_dc_rv = 0.0;

        let (assign39690_e53140, assign39690_e53140_d_n4,) = {
    if (locals.var_guard1025 != 0.0) {
        let assign39690_e53132: f64 = (2.0 * 0.6666666666666666);
        let assign39690_e53134: f64 = (assign39690_e53132 * locals.var_dphibq);
        let assign39690_e53136: f64 = (assign39690_e53134 / locals.var_qb0);
        let assign39690_e53137: f64 = (1.0 + assign39690_e53136);
        let assign39690_e53138: f64 = (locals.var_g_0_dc * assign39690_e53137);
        (assign39690_e53138, ((locals.var_g_0_dc_dn4 * assign39690_e53137) + (locals.var_g_0_dc * ((((assign39690_e53132 * locals.var_dphibq_dn4) * locals.var_qb0) - (assign39690_e53134 * locals.var_qb0_dn4)) / (locals.var_qb0 * locals.var_qb0)))),)
    } else {
        (locals.var_g_0_dc, locals.var_g_0_dc_dn4,)
    }
};
        locals.var_g_0_dc = assign39690_e53140;
        locals.var_g_0_dc_dn4 = assign39690_e53140_d_n4;
        locals.var_g_0_dc_rv = 0.0;

        let assign39700_e53142: f64 = (locals.var_phib_dc).sqrt();
        locals.var_sqrt_phib_dc = assign39700_e53142;
        locals.var_sqrt_phib_dc_dn4 = (locals.var_phib_dc_dn4 / (2.0 * assign39700_e53142));
        locals.var_sqrt_phib_dc_rv = 0.0;

        let assign39710_e53145: f64 = (0.95 * locals.var_phib_dc);
        locals.var_phix_dc = assign39710_e53145;
        locals.var_phix_dc_dn4 = (0.95 * locals.var_phib_dc_dn4);
        locals.var_phix_dc_rv = 0.0;

        let assign39720_e53148: f64 = (0.0025 * locals.var_phib_dc);
        let assign39720_e53150: f64 = (assign39720_e53148 * locals.var_phib_dc);
        locals.var_aphi_dc = assign39720_e53150;
        locals.var_aphi_dc_dn4 = (((0.0025 * locals.var_phib_dc_dn4) * locals.var_phib_dc) + (assign39720_e53148 * locals.var_phib_dc_dn4));
        locals.var_aphi_dc_rv = 0.0;

        locals.var_bphi_dc = locals.var_aphi_dc;
        locals.var_bphi_dc_dn4 = locals.var_aphi_dc_dn4;
        locals.var_bphi_dc_rv = 0.0;

        let assign39740_e53154: f64 = (locals.var_bphi_dc).sqrt();
        let assign39740_e53155: f64 = (0.5 * assign39740_e53154);
        locals.var_phix2 = assign39740_e53155;
        locals.var_phix2_dn4 = (0.5 * (locals.var_bphi_dc_dn4 / (2.0 * assign39740_e53154)));
        locals.var_phix2_rv = 0.0;

        let assign39750_e53159: f64 = (locals.var_phix_dc - locals.var_phix2);
        let assign39750_e53161: f64 = assign39750_e53159;
        let assign39750_e53164: f64 = (locals.var_phix_dc - locals.var_phix2);
        let assign39750_e53166: f64 = assign39750_e53164;
        let assign39750_e53169: f64 = (locals.var_phix_dc - locals.var_phix2);
        let assign39750_e53171: f64 = assign39750_e53169;
        let assign39750_e53172: f64 = (assign39750_e53166 * assign39750_e53171);
        let assign39750_e53174: f64 = (assign39750_e53172 + locals.var_aphi_dc);
        let assign39750_e53175: f64 = (assign39750_e53174).sqrt();
        let assign39750_e53176: f64 = (assign39750_e53161 - assign39750_e53175);
        let assign39750_e53177: f64 = (0.5 * assign39750_e53176);
        locals.var_phix1_dc = assign39750_e53177;
        locals.var_phix1_dc_dn4 = (0.5 * ((locals.var_phix_dc_dn4 - locals.var_phix2_dn4) - (((((locals.var_phix_dc_dn4 - locals.var_phix2_dn4) * assign39750_e53171) + (assign39750_e53166 * (locals.var_phix_dc_dn4 - locals.var_phix2_dn4))) + locals.var_aphi_dc_dn4) / (2.0 * assign39750_e53175))));
        locals.var_phix1_dc_rv = 0.0;

        let assign39760_e53181: f64 = (locals.var_phib_dc + locals.var_eg);
        let assign39760_e53182: f64 = (0.5 * assign39760_e53181);
        locals.var_alpha_b = assign39760_e53182;
        locals.var_alpha_b_dn4 = (0.5 * (locals.var_phib_dc_dn4 + locals.var_eg_dn4));
        locals.var_alpha_b_rv = 0.0;

        let assign39770_e53185: f64 = (locals.var_vsbnud_i + locals.var_phib_dc);
        let assign39770_e53186: f64 = (assign39770_e53185).sqrt();
        let assign39770_e53188: f64 = (assign39770_e53186 - locals.var_sqrt_phib_dc);
        locals.var_us1 = assign39770_e53188;
        locals.var_us1_dn4 = ((locals.var_phib_dc_dn4 / (2.0 * assign39770_e53186)) - locals.var_sqrt_phib_dc_dn4);
        locals.var_us1_rv = 0.0;

        let assign39780_e53191: f64 = (locals.var_vsbnud_i + locals.var_dvsbnud_i);
        let assign39780_e53193: f64 = (assign39780_e53191 + locals.var_phib_dc);
        let assign39780_e53194: f64 = (assign39780_e53193).sqrt();
        let assign39780_e53196: f64 = (assign39780_e53194 - locals.var_sqrt_phib_dc);
        let assign39780_e53198: f64 = (assign39780_e53196 - locals.var_us1);
        locals.var_us21 = assign39780_e53198;
        locals.var_us21_dn4 = (((locals.var_phib_dc_dn4 / (2.0 * assign39780_e53194)) - locals.var_sqrt_phib_dc_dn4) - locals.var_us1_dn4);
        locals.var_us21_rv = 0.0;

        let assign39790_e53201: f64 = (locals.var_eg + locals.var_dphib_i);
        let assign39790_e53203: f64 = (assign39790_e53201 + locals.var_delvtac_i);
        let assign39790_e53206: f64 = (2.0 * locals.var_phit);
        let assign39790_e53210: f64 = (-0.75);
        let assign39790_e53211: f64 = (locals.var_phibfac).powf(assign39790_e53210);
        let assign39790_e53212: f64 = (locals.var_neffac_i * assign39790_e53211);
        let assign39790_e53214: f64 = (assign39790_e53212 * 4e-26);
        let assign39790_e53215: f64 = (assign39790_e53214).ln();
        let assign39790_e53216: f64 = (assign39790_e53206 * assign39790_e53215);
        let assign39790_e53217: f64 = (assign39790_e53203 + assign39790_e53216);
        locals.var_phib_ac = assign39790_e53217;
        locals.var_phib_ac_dn4 = (locals.var_eg_dn4 + (((2.0 * locals.var_phit_dn4) * assign39790_e53215) + (assign39790_e53206 * (((locals.var_neffac_i * if 0.0 == 0.0 && ((assign39790_e53210) as f64).is_finite() && ((assign39790_e53210) as f64).fract() == 0.0 { if assign39790_e53210 == 0.0 { 0.0 } else { (assign39790_e53210 * ((locals.var_phibfac).powf(assign39790_e53210 - 1.0) * locals.var_phibfac_dn4)) } } else { (assign39790_e53211 * (assign39790_e53210 * (locals.var_phibfac_dn4 / locals.var_phibfac))) }) * 4e-26) / assign39790_e53214))));
        locals.var_phib_ac_rv = 0.0;

        let (assign39800_e53223, assign39800_e53223_d_n4,) = {
    if (locals.var_phib_ac > 0.05) {
        (locals.var_phib_ac, locals.var_phib_ac_dn4,)
    } else {
        (0.05, 0.0,)
    }
};
        locals.var_phib_ac = assign39800_e53223;
        locals.var_phib_ac_dn4 = assign39800_e53223_d_n4;
        locals.var_phib_ac_rv = 0.0;

        let assign39810_e53226: f64 = (2.0 * 1.6021918e-19);
        let assign39810_e53228: f64 = (assign39810_e53226 * locals.var_neffac_i);
        let assign39810_e53230: f64 = (assign39810_e53228 * locals.var_epssi);
        let assign39810_e53232: f64 = (assign39810_e53230 * locals.var_inv_phit);
        let assign39810_e53233: f64 = (assign39810_e53232).sqrt();
        let assign39810_e53235: f64 = (assign39810_e53233 / locals.var_coxprime);
        locals.var_g_0_ac = assign39810_e53235;
        locals.var_g_0_ac_dn4 = (((assign39810_e53230 * locals.var_inv_phit_dn4) / (2.0 * assign39810_e53233)) / locals.var_coxprime);
        locals.var_g_0_ac_rv = 0.0;

        let assign39820_e53238: f64 = if p.p51 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1026 = assign39820_e53238;
        locals.var_guard1026_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_14(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign39830_e53249, assign39830_e53249_d_n4,) = {
    if (locals.var_guard1026 != 0.0) {
        let assign39830_e53242: f64 = (locals.var_phit * locals.var_g_0_ac);
        let assign39830_e53244: f64 = (assign39830_e53242 * locals.var_g_0_ac);
        let assign39830_e53246: f64 = (assign39830_e53244 * locals.var_phib_ac);
        let assign39830_e53247: f64 = (assign39830_e53246).sqrt();
        (assign39830_e53247, (((((((locals.var_phit_dn4 * locals.var_g_0_ac) + (locals.var_phit * locals.var_g_0_ac_dn4)) * locals.var_g_0_ac) + (assign39830_e53242 * locals.var_g_0_ac_dn4)) * locals.var_phib_ac) + (assign39830_e53244 * locals.var_phib_ac_dn4)) / (2.0 * assign39830_e53247)),)
    } else {
        (locals.var_qb0, locals.var_qb0_dn4,)
    }
};
        locals.var_qb0 = assign39830_e53249;
        locals.var_qb0_dn4 = assign39830_e53249_d_n4;
        locals.var_qb0_rv = 0.0;

        let (assign39840_e53259, assign39840_e53259_d_n4,) = {
    if (locals.var_guard1026 != 0.0) {
        let assign39840_e53253: f64 = (0.75 * locals.var_qq);
        let assign39840_e53256: f64 = (locals.var_qb0).powf(0.6666666666666666);
        let assign39840_e53257: f64 = (assign39840_e53253 * assign39840_e53256);
        (assign39840_e53257, (assign39840_e53253 * if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((locals.var_qb0).powf(0.6666666666666666 - 1.0) * locals.var_qb0_dn4)) } } else { (assign39840_e53256 * (0.6666666666666666 * (locals.var_qb0_dn4 / locals.var_qb0))) }),)
    } else {
        (locals.var_dphibq, locals.var_dphibq_dn4,)
    }
};
        locals.var_dphibq = assign39840_e53259;
        locals.var_dphibq_dn4 = assign39840_e53259_d_n4;
        locals.var_dphibq_rv = 0.0;

        let (assign39850_e53265, assign39850_e53265_d_n4,) = {
    if (locals.var_guard1026 != 0.0) {
        let assign39850_e53263: f64 = (locals.var_phib_ac + locals.var_dphibq);
        (assign39850_e53263, (locals.var_phib_ac_dn4 + locals.var_dphibq_dn4),)
    } else {
        (locals.var_phib_ac, locals.var_phib_ac_dn4,)
    }
};
        locals.var_phib_ac = assign39850_e53265;
        locals.var_phib_ac_dn4 = assign39850_e53265_d_n4;
        locals.var_phib_ac_rv = 0.0;

        let (assign39860_e53279, assign39860_e53279_d_n4,) = {
    if (locals.var_guard1026 != 0.0) {
        let assign39860_e53271: f64 = (2.0 * 0.6666666666666666);
        let assign39860_e53273: f64 = (assign39860_e53271 * locals.var_dphibq);
        let assign39860_e53275: f64 = (assign39860_e53273 / locals.var_qb0);
        let assign39860_e53276: f64 = (1.0 + assign39860_e53275);
        let assign39860_e53277: f64 = (locals.var_g_0_ac * assign39860_e53276);
        (assign39860_e53277, ((locals.var_g_0_ac_dn4 * assign39860_e53276) + (locals.var_g_0_ac * ((((assign39860_e53271 * locals.var_dphibq_dn4) * locals.var_qb0) - (assign39860_e53273 * locals.var_qb0_dn4)) / (locals.var_qb0 * locals.var_qb0)))),)
    } else {
        (locals.var_g_0_ac, locals.var_g_0_ac_dn4,)
    }
};
        locals.var_g_0_ac = assign39860_e53279;
        locals.var_g_0_ac_dn4 = assign39860_e53279_d_n4;
        locals.var_g_0_ac_rv = 0.0;

        let assign39870_e53282: f64 = (0.95 * locals.var_phib_ac);
        locals.var_phix_ac = assign39870_e53282;
        locals.var_phix_ac_dn4 = (0.95 * locals.var_phib_ac_dn4);
        locals.var_phix_ac_rv = 0.0;

        let assign39880_e53285: f64 = (0.0025 * locals.var_phib_ac);
        let assign39880_e53287: f64 = (assign39880_e53285 * locals.var_phib_ac);
        locals.var_aphi_ac = assign39880_e53287;
        locals.var_aphi_ac_dn4 = (((0.0025 * locals.var_phib_ac_dn4) * locals.var_phib_ac) + (assign39880_e53285 * locals.var_phib_ac_dn4));
        locals.var_aphi_ac_rv = 0.0;

        locals.var_bphi_ac = locals.var_aphi_ac;
        locals.var_bphi_ac_dn4 = locals.var_aphi_ac_dn4;
        locals.var_bphi_ac_rv = 0.0;

        let assign39900_e53291: f64 = (locals.var_bphi_ac).sqrt();
        let assign39900_e53292: f64 = (0.5 * assign39900_e53291);
        locals.var_phix2 = assign39900_e53292;
        locals.var_phix2_dn4 = (0.5 * (locals.var_bphi_ac_dn4 / (2.0 * assign39900_e53291)));
        locals.var_phix2_rv = 0.0;

        let assign39910_e53296: f64 = (locals.var_phix_ac - locals.var_phix2);
        let assign39910_e53298: f64 = assign39910_e53296;
        let assign39910_e53301: f64 = (locals.var_phix_ac - locals.var_phix2);
        let assign39910_e53303: f64 = assign39910_e53301;
        let assign39910_e53306: f64 = (locals.var_phix_ac - locals.var_phix2);
        let assign39910_e53308: f64 = assign39910_e53306;
        let assign39910_e53309: f64 = (assign39910_e53303 * assign39910_e53308);
        let assign39910_e53311: f64 = (assign39910_e53309 + locals.var_aphi_ac);
        let assign39910_e53312: f64 = (assign39910_e53311).sqrt();
        let assign39910_e53313: f64 = (assign39910_e53298 - assign39910_e53312);
        let assign39910_e53314: f64 = (0.5 * assign39910_e53313);
        locals.var_phix1_ac = assign39910_e53314;
        locals.var_phix1_ac_dn4 = (0.5 * ((locals.var_phix_ac_dn4 - locals.var_phix2_dn4) - (((((locals.var_phix_ac_dn4 - locals.var_phix2_dn4) * assign39910_e53308) + (assign39910_e53303 * (locals.var_phix_ac_dn4 - locals.var_phix2_dn4))) + locals.var_aphi_ac_dn4) / (2.0 * assign39910_e53312))));
        locals.var_phix1_ac_rv = 0.0;

        let assign39920_e53318: f64 = (locals.var_stvfb_i * locals.var_delt);
        let assign39920_e53322: f64 = (locals.var_st2vfb_i * locals.var_delt);
        let assign39920_e53323: f64 = (1.0 + assign39920_e53322);
        let assign39920_e53324: f64 = (assign39920_e53318 * assign39920_e53323);
        let assign39920_e53325: f64 = (locals.var_vfb_i + assign39920_e53324);
        let assign39920_e53327: f64 = (assign39920_e53325 + locals.var_delvto_i);
        locals.var_vfb_t = assign39920_e53327;
        locals.var_vfb_t_dn4 = (((locals.var_stvfb_i * locals.var_delt_dn4) * assign39920_e53323) + (assign39920_e53318 * (locals.var_st2vfb_i * locals.var_delt_dn4)));
        locals.var_vfb_t_rv = 0.0;

        let assign39930_e53330: f64 = (locals.var_stct_i * locals.var_ln_rtn);
        let assign39930_e53331: f64 = (assign39930_e53330).exp();
        locals.var_tf_ct = assign39930_e53331;
        locals.var_tf_ct_dn4 = (assign39930_e53331 * (locals.var_stct_i * locals.var_ln_rtn_dn4));
        locals.var_tf_ct_rv = 0.0;

        let assign39940_e53334: f64 = (locals.var_ct_i * locals.var_tf_ct);
        locals.var_ct_t = assign39940_e53334;
        locals.var_ct_t_dn4 = (locals.var_ct_i * locals.var_tf_ct_dn4);
        locals.var_ct_t_rv = 0.0;

        let assign39950_e53337: f64 = (locals.var_ctg_i / locals.var_rtn);
        locals.var_ctg_t = assign39950_e53337;
        locals.var_ctg_t_dn4 = (-((locals.var_ctg_i * locals.var_rtn_dn4) / (locals.var_rtn * locals.var_rtn)));
        locals.var_ctg_t_rv = 0.0;

        let assign39960_e53340: f64 = (locals.var_stbet_i * locals.var_ln_rtn);
        let assign39960_e53341: f64 = (assign39960_e53340).exp();
        locals.var_tf_bet = assign39960_e53341;
        locals.var_tf_bet_dn4 = (assign39960_e53341 * (locals.var_stbet_i * locals.var_ln_rtn_dn4));
        locals.var_tf_bet_rv = 0.0;

        let assign39970_e53344: f64 = (locals.var_betn_i * locals.var_tf_bet);
        locals.var_betn_t = assign39970_e53344;
        locals.var_betn_t_dn4 = (locals.var_betn_i * locals.var_tf_bet_dn4);
        locals.var_betn_t_rv = 0.0;

        let assign39980_e53347: f64 = (locals.var_factuo_i * locals.var_betn_t);
        let assign39980_e53349: f64 = (assign39980_e53347 * locals.var_coxprime);
        locals.var_bet_i = assign39980_e53349;
        locals.var_bet_i_dn4 = ((locals.var_factuo_i * locals.var_betn_t_dn4) * locals.var_coxprime);
        locals.var_bet_i_rv = 0.0;

        let assign39990_e53353: f64 = (locals.var_stthemu_i * locals.var_ln_rtn);
        let assign39990_e53354: f64 = (assign39990_e53353).exp();
        let assign39990_e53355: f64 = (locals.var_themu_i * assign39990_e53354);
        locals.var_themu_t = assign39990_e53355;
        locals.var_themu_t_dn4 = (locals.var_themu_i * (assign39990_e53354 * (locals.var_stthemu_i * locals.var_ln_rtn_dn4)));
        locals.var_themu_t_rv = 0.0;

        let assign40000_e53358: f64 = (locals.var_stmue_i * locals.var_ln_rtn);
        let assign40000_e53359: f64 = (assign40000_e53358).exp();
        locals.var_tf_mue = assign40000_e53359;
        locals.var_tf_mue_dn4 = (assign40000_e53359 * (locals.var_stmue_i * locals.var_ln_rtn_dn4));
        locals.var_tf_mue_rv = 0.0;

        let assign40010_e53362: f64 = (locals.var_mue_i * locals.var_tf_mue);
        locals.var_mue_t = assign40010_e53362;
        locals.var_mue_t_dn4 = (locals.var_mue_i * locals.var_tf_mue_dn4);
        locals.var_mue_t_rv = 0.0;

        let assign40020_e53366: f64 = (locals.var_stthecs_i * locals.var_ln_rtn);
        let assign40020_e53367: f64 = (assign40020_e53366).exp();
        let assign40020_e53368: f64 = (locals.var_thecs_i * assign40020_e53367);
        locals.var_thecs_t = assign40020_e53368;
        locals.var_thecs_t_dn4 = (locals.var_thecs_i * (assign40020_e53367 * (locals.var_stthecs_i * locals.var_ln_rtn_dn4)));
        locals.var_thecs_t_rv = 0.0;

        let assign40030_e53371: f64 = (locals.var_stcs_i * locals.var_ln_rtn);
        let assign40030_e53372: f64 = (assign40030_e53371).exp();
        locals.var_tf_cs = assign40030_e53372;
        locals.var_tf_cs_dn4 = (assign40030_e53372 * (locals.var_stcs_i * locals.var_ln_rtn_dn4));
        locals.var_tf_cs_rv = 0.0;

        let assign40040_e53375: f64 = (locals.var_cs_i * locals.var_tf_cs);
        locals.var_cs_t = assign40040_e53375;
        locals.var_cs_t_dn4 = (locals.var_cs_i * locals.var_tf_cs_dn4);
        locals.var_cs_t_rv = 0.0;

        let assign40050_e53378: f64 = (locals.var_stxcor_i * locals.var_ln_rtn);
        let assign40050_e53379: f64 = (assign40050_e53378).exp();
        locals.var_tf_xcor = assign40050_e53379;
        locals.var_tf_xcor_dn4 = (assign40050_e53379 * (locals.var_stxcor_i * locals.var_ln_rtn_dn4));
        locals.var_tf_xcor_rv = 0.0;

        let assign40060_e53382: f64 = (locals.var_xcor_i * locals.var_tf_xcor);
        locals.var_xcor_t = assign40060_e53382;
        locals.var_xcor_t_dn4 = (locals.var_xcor_i * locals.var_tf_xcor_dn4);
        locals.var_xcor_t_rv = 0.0;

        let assign40070_e53385: f64 = (locals.var_strs_i * locals.var_ln_rtn);
        let assign40070_e53386: f64 = (assign40070_e53385).exp();
        locals.var_tf_ther = assign40070_e53386;
        locals.var_tf_ther_dn4 = (assign40070_e53386 * (locals.var_strs_i * locals.var_ln_rtn_dn4));
        locals.var_tf_ther_rv = 0.0;

        let assign40080_e53389: f64 = (locals.var_rs_i * locals.var_tf_ther);
        locals.var_rs_t = assign40080_e53389;
        locals.var_rs_t_dn4 = (locals.var_rs_i * locals.var_tf_ther_dn4);
        locals.var_rs_t_rv = 0.0;

        let assign40090_e53392: f64 = (2.0 * locals.var_bet_i);
        let assign40090_e53394: f64 = (assign40090_e53392 * locals.var_rs_t);
        locals.var_ther_i = assign40090_e53394;
        locals.var_ther_i_dn4 = (((2.0 * locals.var_bet_i_dn4) * locals.var_rs_t) + (assign40090_e53392 * locals.var_rs_t_dn4));
        locals.var_ther_i_rv = 0.0;

        let assign40100_e53397: f64 = (locals.var_stthesat_i * locals.var_ln_rtn);
        let assign40100_e53398: f64 = (assign40100_e53397).exp();
        locals.var_tf_thesat = assign40100_e53398;
        locals.var_tf_thesat_dn4 = (assign40100_e53398 * (locals.var_stthesat_i * locals.var_ln_rtn_dn4));
        locals.var_tf_thesat_rv = 0.0;

        let assign40110_e53401: f64 = (locals.var_thesat_i * locals.var_tf_thesat);
        locals.var_thesat_t = assign40110_e53401;
        locals.var_thesat_t_dn4 = (locals.var_thesat_i * locals.var_tf_thesat_dn4);
        locals.var_thesat_t_rv = 0.0;

        let assign40120_e53404: f64 = (locals.var_thesatac_i * locals.var_tf_thesat);
        locals.var_thesatac_t = assign40120_e53404;
        locals.var_thesatac_t_dn4 = (locals.var_thesatac_i * locals.var_tf_thesat_dn4);
        locals.var_thesatac_t_rv = 0.0;

        let assign40130_e53407: f64 = (-locals.var_sta2_i);
        let assign40130_e53409: f64 = (assign40130_e53407 * locals.var_ln_rtn);
        let assign40130_e53410: f64 = (assign40130_e53409).exp();
        let assign40130_e53411: f64 = (locals.var_a2_i * assign40130_e53410);
        locals.var_a2_t = assign40130_e53411;
        locals.var_a2_t_dn4 = (locals.var_a2_i * (assign40130_e53410 * (assign40130_e53407 * locals.var_ln_rtn_dn4)));
        locals.var_a2_t_rv = 0.0;

        let assign40140_e53414: f64 = (locals.var_fnt_i * 4.0);
        let assign40140_e53416: f64 = (assign40140_e53414 * 1.3806505e-23);
        let assign40140_e53418: f64 = (assign40140_e53416 * locals.var_tkd);
        locals.var_nt = assign40140_e53418;
        locals.var_nt_dn4 = (assign40140_e53416 * locals.var_tkd_dn4);
        locals.var_nt_rv = 0.0;

        let assign40160_e53432: f64 = if ((p.p46 != 0.0) && (locals.var_betnedge_i > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1027 = assign40160_e53432;
        locals.var_guard1027_rv = 0.0;

        let (assign40170_e53442, assign40170_e53442_d_n4,) = {
    if (locals.var_guard1027 != 0.0) {
        let assign40170_e53437: f64 = (locals.var_stvfbedge_i * locals.var_delt);
        let assign40170_e53438: f64 = (locals.var_vfbedge_i + assign40170_e53437);
        let assign40170_e53440: f64 = (assign40170_e53438 + locals.var_delvtoedge_i);
        (assign40170_e53440, (locals.var_stvfbedge_i * locals.var_delt_dn4),)
    } else {
        (locals.var_vfbedge_t, locals.var_vfbedge_t_dn4,)
    }
};
        locals.var_vfbedge_t = assign40170_e53442;
        locals.var_vfbedge_t_dn4 = assign40170_e53442_d_n4;
        locals.var_vfbedge_t_rv = 0.0;

        let (assign40180_e53449, assign40180_e53449_d_n4,) = {
    if (locals.var_guard1027 != 0.0) {
        let assign40180_e53446: f64 = (locals.var_stbetedge_i * locals.var_ln_rtn);
        let assign40180_e53447: f64 = (assign40180_e53446).exp();
        (assign40180_e53447, (assign40180_e53447 * (locals.var_stbetedge_i * locals.var_ln_rtn_dn4)),)
    } else {
        (locals.var_tf_betedge, locals.var_tf_betedge_dn4,)
    }
};
        locals.var_tf_betedge = assign40180_e53449;
        locals.var_tf_betedge_dn4 = assign40180_e53449_d_n4;
        locals.var_tf_betedge_rv = 0.0;

        let (assign40190_e53455, assign40190_e53455_d_n4,) = {
    if (locals.var_guard1027 != 0.0) {
        let assign40190_e53453: f64 = (locals.var_betnedge_i * locals.var_tf_betedge);
        (assign40190_e53453, (locals.var_betnedge_i * locals.var_tf_betedge_dn4),)
    } else {
        (locals.var_betnedge_t, locals.var_betnedge_t_dn4,)
    }
};
        locals.var_betnedge_t = assign40190_e53455;
        locals.var_betnedge_t_dn4 = assign40190_e53455_d_n4;
        locals.var_betnedge_t_rv = 0.0;

        let (assign40200_e53463, assign40200_e53463_d_n4,) = {
    if (locals.var_guard1027 != 0.0) {
        let assign40200_e53459: f64 = (locals.var_factuoedge_i * locals.var_betnedge_t);
        let assign40200_e53461: f64 = (assign40200_e53459 * locals.var_coxprime);
        (assign40200_e53461, ((locals.var_factuoedge_i * locals.var_betnedge_t_dn4) * locals.var_coxprime),)
    } else {
        (locals.var_betedge_i, locals.var_betedge_i_dn4,)
    }
};
        locals.var_betedge_i = assign40200_e53463;
        locals.var_betedge_i_dn4 = assign40200_e53463_d_n4;
        locals.var_betedge_i_rv = 0.0;

        let (assign40210_e53473, assign40210_e53473_d_n4,) = {
    if (locals.var_guard1027 != 0.0) {
        let assign40210_e53469: f64 = (locals.var_ctedge_i * locals.var_rtn);
        let assign40210_e53470: f64 = (1.0 + assign40210_e53469);
        let assign40210_e53471: f64 = (locals.var_phit * assign40210_e53470);
        (assign40210_e53471, ((locals.var_phit_dn4 * assign40210_e53470) + (locals.var_phit * (locals.var_ctedge_i * locals.var_rtn_dn4))),)
    } else {
        (locals.var_phit0edge, locals.var_phit0edge_dn4,)
    }
};
        locals.var_phit0edge = assign40210_e53473;
        locals.var_phit0edge_dn4 = assign40210_e53473_d_n4;
        locals.var_phit0edge_rv = 0.0;

        let (assign40220_e53493, assign40220_e53493_d_n4,) = {
    if (locals.var_guard1027 != 0.0) {
        let assign40220_e53477: f64 = (locals.var_eg + locals.var_dphibedge_i);
        let assign40220_e53480: f64 = (2.0 * locals.var_phit0edge);
        let assign40220_e53484: f64 = (-0.75);
        let assign40220_e53485: f64 = (locals.var_phibfac).powf(assign40220_e53484);
        let assign40220_e53486: f64 = (locals.var_neffedge_i * assign40220_e53485);
        let assign40220_e53488: f64 = (assign40220_e53486 * 4e-26);
        let assign40220_e53489: f64 = (assign40220_e53488).ln();
        let assign40220_e53490: f64 = (assign40220_e53480 * assign40220_e53489);
        let assign40220_e53491: f64 = (assign40220_e53477 + assign40220_e53490);
        (assign40220_e53491, (locals.var_eg_dn4 + (((2.0 * locals.var_phit0edge_dn4) * assign40220_e53489) + (assign40220_e53480 * (((locals.var_neffedge_i * if 0.0 == 0.0 && ((assign40220_e53484) as f64).is_finite() && ((assign40220_e53484) as f64).fract() == 0.0 { if assign40220_e53484 == 0.0 { 0.0 } else { (assign40220_e53484 * ((locals.var_phibfac).powf(assign40220_e53484 - 1.0) * locals.var_phibfac_dn4)) } } else { (assign40220_e53485 * (assign40220_e53484 * (locals.var_phibfac_dn4 / locals.var_phibfac))) }) * 4e-26) / assign40220_e53488)))),)
    } else {
        (locals.var_phibedge, locals.var_phibedge_dn4,)
    }
};
        locals.var_phibedge = assign40220_e53493;
        locals.var_phibedge_dn4 = assign40220_e53493_d_n4;
        locals.var_phibedge_rv = 0.0;

        let (assign40230_e53502, assign40230_e53502_d_n4,) = {
    if (locals.var_guard1027 != 0.0) {
        let (assign40230_e53500, assign40230_e53500_d_n4,) = {
            if (locals.var_phibedge > 0.05) {
                (locals.var_phibedge, locals.var_phibedge_dn4,)
            } else {
                (0.05, 0.0,)
            }
        };
        (assign40230_e53500, assign40230_e53500_d_n4,)
    } else {
        (locals.var_phibedge, locals.var_phibedge_dn4,)
    }
};
        locals.var_phibedge = assign40230_e53502;
        locals.var_phibedge_dn4 = assign40230_e53502_d_n4;
        locals.var_phibedge_rv = 0.0;

        let (assign40240_e53517, assign40240_e53517_d_n4,) = {
    if (locals.var_guard1027 != 0.0) {
        let assign40240_e53506: f64 = (2.0 * 1.6021918e-19);
        let assign40240_e53508: f64 = (assign40240_e53506 * locals.var_neffedge_i);
        let assign40240_e53510: f64 = (assign40240_e53508 * locals.var_epssi);
        let assign40240_e53512: f64 = (assign40240_e53510 * locals.var_inv_phit);
        let assign40240_e53513: f64 = (assign40240_e53512).sqrt();
        let assign40240_e53515: f64 = (assign40240_e53513 / locals.var_coxprime);
        (assign40240_e53515, (((assign40240_e53510 * locals.var_inv_phit_dn4) / (2.0 * assign40240_e53513)) / locals.var_coxprime),)
    } else {
        (locals.var_gfedge, locals.var_gfedge_dn4,)
    }
};
        locals.var_gfedge = assign40240_e53517;
        locals.var_gfedge_dn4 = assign40240_e53517_d_n4;
        locals.var_gfedge_rv = 0.0;

        let (assign40250_e53523, assign40250_e53523_d_n4,) = {
    if (locals.var_guard1027 != 0.0) {
        let assign40250_e53521: f64 = (locals.var_gfedge * locals.var_gfedge);
        (assign40250_e53521, ((locals.var_gfedge_dn4 * locals.var_gfedge) + (locals.var_gfedge * locals.var_gfedge_dn4)),)
    } else {
        (locals.var_gfedge2, locals.var_gfedge2_dn4,)
    }
};
        locals.var_gfedge2 = assign40250_e53523;
        locals.var_gfedge2_dn4 = assign40250_e53523_d_n4;
        locals.var_gfedge2_rv = 0.0;

        let (assign40260_e53528, assign40260_e53528_d_n4,) = {
    if (locals.var_guard1027 != 0.0) {
        let assign40260_e53526: f64 = (locals.var_gfedge2).ln();
        (assign40260_e53526, (locals.var_gfedge2_dn4 / locals.var_gfedge2),)
    } else {
        (locals.var_lngfedge2, locals.var_lngfedge2_dn4,)
    }
};
        locals.var_lngfedge2 = assign40260_e53528;
        locals.var_lngfedge2_dn4 = assign40260_e53528_d_n4;
        locals.var_lngfedge2_rv = 0.0;

        let (assign40270_e53534, assign40270_e53534_d_n4,) = {
    if (locals.var_guard1027 != 0.0) {
        let assign40270_e53532: f64 = (0.95 * locals.var_phibedge);
        (assign40270_e53532, (0.95 * locals.var_phibedge_dn4),)
    } else {
        (locals.var_phixedge, locals.var_phixedge_dn4,)
    }
};
        locals.var_phixedge = assign40270_e53534;
        locals.var_phixedge_dn4 = assign40270_e53534_d_n4;
        locals.var_phixedge_rv = 0.0;

        let (assign40280_e53542, assign40280_e53542_d_n4,) = {
    if (locals.var_guard1027 != 0.0) {
        let assign40280_e53538: f64 = (0.0025 * locals.var_phibedge);
        let assign40280_e53540: f64 = (assign40280_e53538 * locals.var_phibedge);
        (assign40280_e53540, (((0.0025 * locals.var_phibedge_dn4) * locals.var_phibedge) + (assign40280_e53538 * locals.var_phibedge_dn4)),)
    } else {
        (locals.var_aphiedge, locals.var_aphiedge_dn4,)
    }
};
        locals.var_aphiedge = assign40280_e53542;
        locals.var_aphiedge_dn4 = assign40280_e53542_d_n4;
        locals.var_aphiedge_rv = 0.0;

        let (assign40290_e53546, assign40290_e53546_d_n4,) = {
    if (locals.var_guard1027 != 0.0) {
        (locals.var_aphiedge, locals.var_aphiedge_dn4,)
    } else {
        (locals.var_bphiedge, locals.var_bphiedge_dn4,)
    }
};
        locals.var_bphiedge = assign40290_e53546;
        locals.var_bphiedge_dn4 = assign40290_e53546_d_n4;
        locals.var_bphiedge_rv = 0.0;

        let (assign40300_e53553, assign40300_e53553_d_n4,) = {
    if (locals.var_guard1027 != 0.0) {
        let assign40300_e53550: f64 = (locals.var_bphiedge).sqrt();
        let assign40300_e53551: f64 = (0.5 * assign40300_e53550);
        (assign40300_e53551, (0.5 * (locals.var_bphiedge_dn4 / (2.0 * assign40300_e53550))),)
    } else {
        (locals.var_phix2edge, locals.var_phix2edge_dn4,)
    }
};
        locals.var_phix2edge = assign40300_e53553;
        locals.var_phix2edge_dn4 = assign40300_e53553_d_n4;
        locals.var_phix2edge_rv = 0.0;

        let (assign40310_e53578, assign40310_e53578_d_n4,) = {
    if (locals.var_guard1027 != 0.0) {
        let assign40310_e53558: f64 = (locals.var_phixedge - locals.var_phix2edge);
        let assign40310_e53560: f64 = assign40310_e53558;
        let assign40310_e53563: f64 = (locals.var_phixedge - locals.var_phix2edge);
        let assign40310_e53565: f64 = assign40310_e53563;
        let assign40310_e53568: f64 = (locals.var_phixedge - locals.var_phix2edge);
        let assign40310_e53570: f64 = assign40310_e53568;
        let assign40310_e53571: f64 = (assign40310_e53565 * assign40310_e53570);
        let assign40310_e53573: f64 = (assign40310_e53571 + locals.var_aphiedge);
        let assign40310_e53574: f64 = (assign40310_e53573).sqrt();
        let assign40310_e53575: f64 = (assign40310_e53560 - assign40310_e53574);
        let assign40310_e53576: f64 = (0.5 * assign40310_e53575);
        (assign40310_e53576, (0.5 * ((locals.var_phixedge_dn4 - locals.var_phix2edge_dn4) - (((((locals.var_phixedge_dn4 - locals.var_phix2edge_dn4) * assign40310_e53570) + (assign40310_e53565 * (locals.var_phixedge_dn4 - locals.var_phix2edge_dn4))) + locals.var_aphiedge_dn4) / (2.0 * assign40310_e53574)))),)
    } else {
        (locals.var_phix1edge, locals.var_phix1edge_dn4,)
    }
};
        locals.var_phix1edge = assign40310_e53578;
        locals.var_phix1edge_dn4 = assign40310_e53578_d_n4;
        locals.var_phix1edge_rv = 0.0;

        let (assign40340_e53603, assign40340_e53603_d_n4,) = {
    if (locals.var_guard1027 == 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_vfbedge_t, locals.var_vfbedge_t_dn4,)
    }
};
        locals.var_vfbedge_t = assign40340_e53603;
        locals.var_vfbedge_t_dn4 = assign40340_e53603_d_n4;
        locals.var_vfbedge_t_rv = 0.0;

        let (assign40350_e53608, assign40350_e53608_d_n4,) = {
    if (locals.var_guard1027 == 0.0) {
        (1.0, 0.0,)
    } else {
        (locals.var_tf_betedge, locals.var_tf_betedge_dn4,)
    }
};
        locals.var_tf_betedge = assign40350_e53608;
        locals.var_tf_betedge_dn4 = assign40350_e53608_d_n4;
        locals.var_tf_betedge_rv = 0.0;

        let (assign40360_e53613, assign40360_e53613_d_n4,) = {
    if (locals.var_guard1027 == 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_betnedge_t, locals.var_betnedge_t_dn4,)
    }
};
        locals.var_betnedge_t = assign40360_e53613;
        locals.var_betnedge_t_dn4 = assign40360_e53613_d_n4;
        locals.var_betnedge_t_rv = 0.0;

        let (assign40370_e53618, assign40370_e53618_d_n4,) = {
    if (locals.var_guard1027 == 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_betedge_i, locals.var_betedge_i_dn4,)
    }
};
        locals.var_betedge_i = assign40370_e53618;
        locals.var_betedge_i_dn4 = assign40370_e53618_d_n4;
        locals.var_betedge_i_rv = 0.0;

        let (assign40380_e53623, assign40380_e53623_d_n4,) = {
    if (locals.var_guard1027 == 0.0) {
        (locals.var_phit, locals.var_phit_dn4,)
    } else {
        (locals.var_phit0edge, locals.var_phit0edge_dn4,)
    }
};
        locals.var_phit0edge = assign40380_e53623;
        locals.var_phit0edge_dn4 = assign40380_e53623_d_n4;
        locals.var_phit0edge_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_15(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        locals: &mut StampLocals,
    ) {
        let nv6 = ctx.node_voltage(nodes[6]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let (assign40390_e53628, assign40390_e53628_d_n4,) = {
    if (locals.var_guard1027 == 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_phibedge, locals.var_phibedge_dn4,)
    }
};
        locals.var_phibedge = assign40390_e53628;
        locals.var_phibedge_dn4 = assign40390_e53628_d_n4;
        locals.var_phibedge_rv = 0.0;

        let (assign40400_e53633, assign40400_e53633_d_n4,) = {
    if (locals.var_guard1027 == 0.0) {
        (1.0, 0.0,)
    } else {
        (locals.var_gfedge, locals.var_gfedge_dn4,)
    }
};
        locals.var_gfedge = assign40400_e53633;
        locals.var_gfedge_dn4 = assign40400_e53633_d_n4;
        locals.var_gfedge_rv = 0.0;

        let (assign40410_e53638, assign40410_e53638_d_n4,) = {
    if (locals.var_guard1027 == 0.0) {
        (1.0, 0.0,)
    } else {
        (locals.var_gfedge2, locals.var_gfedge2_dn4,)
    }
};
        locals.var_gfedge2 = assign40410_e53638;
        locals.var_gfedge2_dn4 = assign40410_e53638_d_n4;
        locals.var_gfedge2_rv = 0.0;

        let (assign40420_e53643, assign40420_e53643_d_n4,) = {
    if (locals.var_guard1027 == 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_lngfedge2, locals.var_lngfedge2_dn4,)
    }
};
        locals.var_lngfedge2 = assign40420_e53643;
        locals.var_lngfedge2_dn4 = assign40420_e53643_d_n4;
        locals.var_lngfedge2_rv = 0.0;

        let (assign40430_e53648, assign40430_e53648_d_n4,) = {
    if (locals.var_guard1027 == 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_phixedge, locals.var_phixedge_dn4,)
    }
};
        locals.var_phixedge = assign40430_e53648;
        locals.var_phixedge_dn4 = assign40430_e53648_d_n4;
        locals.var_phixedge_rv = 0.0;

        let (assign40440_e53653, assign40440_e53653_d_n4,) = {
    if (locals.var_guard1027 == 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_aphiedge, locals.var_aphiedge_dn4,)
    }
};
        locals.var_aphiedge = assign40440_e53653;
        locals.var_aphiedge_dn4 = assign40440_e53653_d_n4;
        locals.var_aphiedge_rv = 0.0;

        let (assign40450_e53658, assign40450_e53658_d_n4,) = {
    if (locals.var_guard1027 == 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_bphiedge, locals.var_bphiedge_dn4,)
    }
};
        locals.var_bphiedge = assign40450_e53658;
        locals.var_bphiedge_dn4 = assign40450_e53658_d_n4;
        locals.var_bphiedge_rv = 0.0;

        let (assign40460_e53663, assign40460_e53663_d_n4,) = {
    if (locals.var_guard1027 == 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_phix2edge, locals.var_phix2edge_dn4,)
    }
};
        locals.var_phix2edge = assign40460_e53663;
        locals.var_phix2edge_dn4 = assign40460_e53663_d_n4;
        locals.var_phix2edge_rv = 0.0;

        let (assign40470_e53668, assign40470_e53668_d_n4,) = {
    if (locals.var_guard1027 == 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_phix1edge, locals.var_phix1edge_dn4,)
    }
};
        locals.var_phix1edge = assign40470_e53668;
        locals.var_phix1edge_dn4 = assign40470_e53668_d_n4;
        locals.var_phix1edge_rv = 0.0;

        let assign40500_e53681: f64 = 1.0;
        let assign40500_e53682: f64 = if locals.var_chnl_type == assign40500_e53681 { 1.0 } else { 0.0 };
        locals.var_guard1028 = assign40500_e53682;
        locals.var_guard1028_rv = 0.0;

        let (assign40510_e53686, assign40510_e53686_d_n6, assign40510_e53686_d_n7, assign40510_e53686_d_n8,) = {
    if (locals.var_guard1028 != 0.0) {
        ((nv6 - nv7), 1.0, -1.0, 0.0,)
    } else {
        (locals.var_v_gs, locals.var_v_gs_dn6, locals.var_v_gs_dn7, locals.var_v_gs_dn8,)
    }
};
        locals.var_v_gs = assign40510_e53686;
        locals.var_v_gs_dn6 = assign40510_e53686_d_n6;
        locals.var_v_gs_dn7 = assign40510_e53686_d_n7;
        locals.var_v_gs_dn8 = assign40510_e53686_d_n8;
        locals.var_v_gs_rv = 0.0;

        let (assign40520_e53690, assign40520_e53690_d_n7, assign40520_e53690_d_n8,) = {
    if (locals.var_guard1028 != 0.0) {
        ((nv8 - nv7), -1.0, 1.0,)
    } else {
        (locals.var_v_ds, locals.var_v_ds_dn7, locals.var_v_ds_dn8,)
    }
};
        locals.var_v_ds = assign40520_e53690;
        locals.var_v_ds_dn7 = assign40520_e53690_d_n7;
        locals.var_v_ds_dn8 = assign40520_e53690_d_n8;
        locals.var_v_ds_rv = 0.0;

        let (assign40530_e53694, assign40530_e53694_d_n7, assign40530_e53694_d_n8, assign40530_e53694_d_n9,) = {
    if (locals.var_guard1028 != 0.0) {
        ((nv7 - nv9), 1.0, 0.0, -1.0,)
    } else {
        (locals.var_v_sb, locals.var_v_sb_dn7, locals.var_v_sb_dn8, locals.var_v_sb_dn9,)
    }
};
        locals.var_v_sb = assign40530_e53694;
        locals.var_v_sb_dn7 = assign40530_e53694_d_n7;
        locals.var_v_sb_dn8 = assign40530_e53694_d_n8;
        locals.var_v_sb_dn9 = assign40530_e53694_d_n9;
        locals.var_v_sb_rv = 0.0;

        let (assign40560_e53710, assign40560_e53710_d_n6, assign40560_e53710_d_n7, assign40560_e53710_d_n8,) = {
    if (locals.var_guard1028 == 0.0) {
        let assign40560_e53708: f64 = (-(nv6 - nv7));
        (assign40560_e53708, (-1.0), 1.0, 0.0,)
    } else {
        (locals.var_v_gs, locals.var_v_gs_dn6, locals.var_v_gs_dn7, locals.var_v_gs_dn8,)
    }
};
        locals.var_v_gs = assign40560_e53710;
        locals.var_v_gs_dn6 = assign40560_e53710_d_n6;
        locals.var_v_gs_dn7 = assign40560_e53710_d_n7;
        locals.var_v_gs_dn8 = assign40560_e53710_d_n8;
        locals.var_v_gs_rv = 0.0;

        let (assign40570_e53716, assign40570_e53716_d_n7, assign40570_e53716_d_n8,) = {
    if (locals.var_guard1028 == 0.0) {
        let assign40570_e53714: f64 = (-(nv8 - nv7));
        (assign40570_e53714, 1.0, (-1.0),)
    } else {
        (locals.var_v_ds, locals.var_v_ds_dn7, locals.var_v_ds_dn8,)
    }
};
        locals.var_v_ds = assign40570_e53716;
        locals.var_v_ds_dn7 = assign40570_e53716_d_n7;
        locals.var_v_ds_dn8 = assign40570_e53716_d_n8;
        locals.var_v_ds_rv = 0.0;

        let (assign40580_e53722, assign40580_e53722_d_n7, assign40580_e53722_d_n8, assign40580_e53722_d_n9,) = {
    if (locals.var_guard1028 == 0.0) {
        let assign40580_e53720: f64 = (-(nv7 - nv9));
        (assign40580_e53720, (-1.0), 0.0, 1.0,)
    } else {
        (locals.var_v_sb, locals.var_v_sb_dn7, locals.var_v_sb_dn8, locals.var_v_sb_dn9,)
    }
};
        locals.var_v_sb = assign40580_e53722;
        locals.var_v_sb_dn7 = assign40580_e53722_d_n7;
        locals.var_v_sb_dn8 = assign40580_e53722_d_n8;
        locals.var_v_sb_dn9 = assign40580_e53722_d_n9;
        locals.var_v_sb_rv = 0.0;

        let assign40610_e53735: f64 = (locals.var_v_gs + locals.var_v_sb);
        locals.var_vgb = assign40610_e53735;
        locals.var_vgb_dn6 = locals.var_v_gs_dn6;
        locals.var_vgb_dn7 = (locals.var_v_gs_dn7 + locals.var_v_sb_dn7);
        locals.var_vgb_dn8 = (locals.var_v_gs_dn8 + locals.var_v_sb_dn8);
        locals.var_vgb_dn9 = locals.var_v_sb_dn9;
        locals.var_vgb_rv = 0.0;

        locals.var_vgsprime = locals.var_v_gs;
        locals.var_vgsprime_dn6 = locals.var_v_gs_dn6;
        locals.var_vgsprime_dn7 = locals.var_v_gs_dn7;
        locals.var_vgsprime_dn8 = locals.var_v_gs_dn8;
        locals.var_vgsprime_rv = 0.0;

        locals.var_vsbprime = locals.var_v_sb;
        locals.var_vsbprime_dn7 = locals.var_v_sb_dn7;
        locals.var_vsbprime_dn8 = locals.var_v_sb_dn8;
        locals.var_vsbprime_dn9 = locals.var_v_sb_dn9;
        locals.var_vsbprime_rv = 0.0;

        let assign40640_e53740: f64 = (locals.var_v_ds + locals.var_v_sb);
        locals.var_vdbprime = assign40640_e53740;
        locals.var_vdbprime_dn7 = (locals.var_v_ds_dn7 + locals.var_v_sb_dn7);
        locals.var_vdbprime_dn8 = (locals.var_v_ds_dn8 + locals.var_v_sb_dn8);
        locals.var_vdbprime_dn9 = locals.var_v_sb_dn9;
        locals.var_vdbprime_rv = 0.0;

        let assign40650_e53743: f64 = (locals.var_v_gs - locals.var_v_ds);
        locals.var_vgdprime = assign40650_e53743;
        locals.var_vgdprime_dn6 = locals.var_v_gs_dn6;
        locals.var_vgdprime_dn7 = (locals.var_v_gs_dn7 - locals.var_v_ds_dn7);
        locals.var_vgdprime_dn8 = (locals.var_v_gs_dn8 - locals.var_v_ds_dn8);
        locals.var_vgdprime_rv = 0.0;

        let assign40660_e53745: f64 = (-locals.var_vgsprime);
        let assign40660_e53747: f64 = (assign40660_e53745 * locals.var_inv_phita);
        locals.var_xgs_ov = assign40660_e53747;
        locals.var_xgs_ov_dn6 = ((-locals.var_vgsprime_dn6) * locals.var_inv_phita);
        locals.var_xgs_ov_dn7 = ((-locals.var_vgsprime_dn7) * locals.var_inv_phita);
        locals.var_xgs_ov_dn8 = ((-locals.var_vgsprime_dn8) * locals.var_inv_phita);
        locals.var_xgs_ov_rv = 0.0;

        let assign40670_e53749: f64 = (-locals.var_vgdprime);
        let assign40670_e53751: f64 = (assign40670_e53749 * locals.var_inv_phita);
        locals.var_xgd_ov = assign40670_e53751;
        locals.var_xgd_ov_dn6 = ((-locals.var_vgdprime_dn6) * locals.var_inv_phita);
        locals.var_xgd_ov_dn7 = ((-locals.var_vgdprime_dn7) * locals.var_inv_phita);
        locals.var_xgd_ov_dn8 = ((-locals.var_vgdprime_dn8) * locals.var_inv_phita);
        locals.var_xgd_ov_rv = 0.0;

        let assign40680_e53754: f64 = (locals.var_vgb - locals.var_vfb_t);
        let assign40680_e53755: f64 = (-assign40680_e53754);
        let assign40680_e53757: f64 = (assign40680_e53755 * locals.var_inv_phita);
        locals.var_xgb_ov = assign40680_e53757;
        locals.var_xgb_ov_dn4 = ((-(-locals.var_vfb_t_dn4)) * locals.var_inv_phita);
        locals.var_xgb_ov_dn6 = ((-locals.var_vgb_dn6) * locals.var_inv_phita);
        locals.var_xgb_ov_dn7 = ((-locals.var_vgb_dn7) * locals.var_inv_phita);
        locals.var_xgb_ov_dn8 = ((-locals.var_vgb_dn8) * locals.var_inv_phita);
        locals.var_xgb_ov_dn9 = ((-locals.var_vgb_dn9) * locals.var_inv_phita);
        locals.var_xgb_ov_rv = 0.0;

        locals.var_sigvds = 1.0;
        locals.var_sigvds_rv = 0.0;

        let assign40700_e53761: f64 = if locals.var_v_ds < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1029 = assign40700_e53761;
        locals.var_guard1029_rv = 0.0;

        let (assign40710_e53766,) = {
    if (locals.var_guard1029 != 0.0) {
        let assign40710_e53764: f64 = (-1.0);
        (assign40710_e53764,)
    } else {
        (locals.var_sigvds,)
    }
};
        locals.var_sigvds = assign40710_e53766;
        locals.var_sigvds_rv = 0.0;

        let (assign40720_e53772, assign40720_e53772_d_n6, assign40720_e53772_d_n7, assign40720_e53772_d_n8,) = {
    if (locals.var_guard1029 != 0.0) {
        let assign40720_e53770: f64 = (locals.var_v_gs - locals.var_v_ds);
        (assign40720_e53770, locals.var_v_gs_dn6, (locals.var_v_gs_dn7 - locals.var_v_ds_dn7), (locals.var_v_gs_dn8 - locals.var_v_ds_dn8),)
    } else {
        (locals.var_v_gs, locals.var_v_gs_dn6, locals.var_v_gs_dn7, locals.var_v_gs_dn8,)
    }
};
        locals.var_v_gs = assign40720_e53772;
        locals.var_v_gs_dn6 = assign40720_e53772_d_n6;
        locals.var_v_gs_dn7 = assign40720_e53772_d_n7;
        locals.var_v_gs_dn8 = assign40720_e53772_d_n8;
        locals.var_v_gs_rv = 0.0;

        let (assign40730_e53778, assign40730_e53778_d_n7, assign40730_e53778_d_n8, assign40730_e53778_d_n9,) = {
    if (locals.var_guard1029 != 0.0) {
        let assign40730_e53776: f64 = (locals.var_v_sb + locals.var_v_ds);
        (assign40730_e53776, (locals.var_v_sb_dn7 + locals.var_v_ds_dn7), (locals.var_v_sb_dn8 + locals.var_v_ds_dn8), locals.var_v_sb_dn9,)
    } else {
        (locals.var_v_sb, locals.var_v_sb_dn7, locals.var_v_sb_dn8, locals.var_v_sb_dn9,)
    }
};
        locals.var_v_sb = assign40730_e53778;
        locals.var_v_sb_dn7 = assign40730_e53778_d_n7;
        locals.var_v_sb_dn8 = assign40730_e53778_d_n8;
        locals.var_v_sb_dn9 = assign40730_e53778_d_n9;
        locals.var_v_sb_rv = 0.0;

        let (assign40740_e53783, assign40740_e53783_d_n7, assign40740_e53783_d_n8,) = {
    if (locals.var_guard1029 != 0.0) {
        let assign40740_e53781: f64 = (-locals.var_v_ds);
        (assign40740_e53781, (-locals.var_v_ds_dn7), (-locals.var_v_ds_dn8),)
    } else {
        (locals.var_v_ds, locals.var_v_ds_dn7, locals.var_v_ds_dn8,)
    }
};
        locals.var_v_ds = assign40740_e53783;
        locals.var_v_ds_dn7 = assign40740_e53783_d_n7;
        locals.var_v_ds_dn8 = assign40740_e53783_d_n8;
        locals.var_v_ds_rv = 0.0;

        let assign40750_e53786: f64 = (locals.var_v_ds + locals.var_v_sb);
        locals.var_v_db = assign40750_e53786;
        locals.var_v_db_dn7 = (locals.var_v_ds_dn7 + locals.var_v_sb_dn7);
        locals.var_v_db_dn8 = (locals.var_v_ds_dn8 + locals.var_v_sb_dn8);
        locals.var_v_db_dn9 = locals.var_v_sb_dn9;
        locals.var_v_db_rv = 0.0;

        let assign40760_e53789: f64 = (locals.var_v_ds * locals.var_v_ds);
        let assign40760_e53792: f64 = (locals.var_v_ds * locals.var_v_ds);
        let assign40760_e53794: f64 = (assign40760_e53792 + 0.01);
        let assign40760_e53795: f64 = (assign40760_e53794).sqrt();
        let assign40760_e53797: f64 = (assign40760_e53795 + 0.1);
        let assign40760_e53798: f64 = (assign40760_e53789 / assign40760_e53797);
        locals.var_vdsx = assign40760_e53798;
        locals.var_vdsx_dn7 = (((((locals.var_v_ds_dn7 * locals.var_v_ds) + (locals.var_v_ds * locals.var_v_ds_dn7)) * assign40760_e53797) - (assign40760_e53789 * (((locals.var_v_ds_dn7 * locals.var_v_ds) + (locals.var_v_ds * locals.var_v_ds_dn7)) / (2.0 * assign40760_e53795)))) / (assign40760_e53797 * assign40760_e53797));
        locals.var_vdsx_dn8 = (((((locals.var_v_ds_dn8 * locals.var_v_ds) + (locals.var_v_ds * locals.var_v_ds_dn8)) * assign40760_e53797) - (assign40760_e53789 * (((locals.var_v_ds_dn8 * locals.var_v_ds) + (locals.var_v_ds * locals.var_v_ds_dn8)) / (2.0 * assign40760_e53795)))) / (assign40760_e53797 * assign40760_e53797));
        locals.var_vdsx_rv = 0.0;

        let assign40770_e53802: f64 = (locals.var_v_db + locals.var_v_sb);
        let assign40770_e53805: f64 = (locals.var_v_db - locals.var_v_sb);
        let assign40770_e53808: f64 = (locals.var_v_db - locals.var_v_sb);
        let assign40770_e53809: f64 = (assign40770_e53805 * assign40770_e53808);
        let assign40770_e53811: f64 = (assign40770_e53809 + locals.var_bphi_dc);
        let assign40770_e53812: f64 = (assign40770_e53811).sqrt();
        let assign40770_e53813: f64 = (assign40770_e53802 - assign40770_e53812);
        let assign40770_e53814: f64 = (0.5 * assign40770_e53813);
        let assign40770_e53816: f64 = (assign40770_e53814 + locals.var_phix_dc);
        locals.var_v_xb = assign40770_e53816;
        locals.var_v_xb_dn4 = ((0.5 * (-(locals.var_bphi_dc_dn4 / (2.0 * assign40770_e53812)))) + locals.var_phix_dc_dn4);
        locals.var_v_xb_dn7 = (0.5 * ((locals.var_v_db_dn7 + locals.var_v_sb_dn7) - ((((locals.var_v_db_dn7 - locals.var_v_sb_dn7) * assign40770_e53808) + (assign40770_e53805 * (locals.var_v_db_dn7 - locals.var_v_sb_dn7))) / (2.0 * assign40770_e53812))));
        locals.var_v_xb_dn8 = (0.5 * ((locals.var_v_db_dn8 + locals.var_v_sb_dn8) - ((((locals.var_v_db_dn8 - locals.var_v_sb_dn8) * assign40770_e53808) + (assign40770_e53805 * (locals.var_v_db_dn8 - locals.var_v_sb_dn8))) / (2.0 * assign40770_e53812))));
        locals.var_v_xb_dn9 = (0.5 * ((locals.var_v_db_dn9 + locals.var_v_sb_dn9) - ((((locals.var_v_db_dn9 - locals.var_v_sb_dn9) * assign40770_e53808) + (assign40770_e53805 * (locals.var_v_db_dn9 - locals.var_v_sb_dn9))) / (2.0 * assign40770_e53812))));
        locals.var_v_xb_rv = 0.0;

        locals.var_v_xb_dc_tmp = locals.var_v_xb;
        locals.var_v_xb_dc_tmp_dn4 = locals.var_v_xb_dn4;
        locals.var_v_xb_dc_tmp_dn7 = locals.var_v_xb_dn7;
        locals.var_v_xb_dc_tmp_dn8 = locals.var_v_xb_dn8;
        locals.var_v_xb_dc_tmp_dn9 = locals.var_v_xb_dn9;
        locals.var_v_xb_dc_tmp_rv = 0.0;

        let assign40790_e53822: f64 = locals.var_v_xb;
        let assign40790_e53825: f64 = locals.var_v_xb;
        let assign40790_e53828: f64 = locals.var_v_xb;
        let assign40790_e53829: f64 = (assign40790_e53825 * assign40790_e53828);
        let assign40790_e53831: f64 = (assign40790_e53829 + locals.var_aphi_dc);
        let assign40790_e53832: f64 = (assign40790_e53831).sqrt();
        let assign40790_e53833: f64 = (assign40790_e53822 - assign40790_e53832);
        let assign40790_e53834: f64 = (0.5 * assign40790_e53833);
        let assign40790_e53835: f64 = (locals.var_v_sb - assign40790_e53834);
        let assign40790_e53837: f64 = (assign40790_e53835 + locals.var_phix1_dc);
        locals.var_vsbstar_dc = assign40790_e53837;
        locals.var_vsbstar_dc_dn4 = ((-(0.5 * (locals.var_v_xb_dn4 - ((((locals.var_v_xb_dn4 * assign40790_e53828) + (assign40790_e53825 * locals.var_v_xb_dn4)) + locals.var_aphi_dc_dn4) / (2.0 * assign40790_e53832))))) + locals.var_phix1_dc_dn4);
        locals.var_vsbstar_dc_dn6 = 0.0;
        locals.var_vsbstar_dc_dn7 = (locals.var_v_sb_dn7 - (0.5 * (locals.var_v_xb_dn7 - (((locals.var_v_xb_dn7 * assign40790_e53828) + (assign40790_e53825 * locals.var_v_xb_dn7)) / (2.0 * assign40790_e53832)))));
        locals.var_vsbstar_dc_dn8 = (locals.var_v_sb_dn8 - (0.5 * (locals.var_v_xb_dn8 - (((locals.var_v_xb_dn8 * assign40790_e53828) + (assign40790_e53825 * locals.var_v_xb_dn8)) / (2.0 * assign40790_e53832)))));
        locals.var_vsbstar_dc_dn9 = (locals.var_v_sb_dn9 - (0.5 * (locals.var_v_xb_dn9 - (((locals.var_v_xb_dn9 * assign40790_e53828) + (assign40790_e53825 * locals.var_v_xb_dn9)) / (2.0 * assign40790_e53832)))));
        locals.var_vsbstar_dc_rv = 0.0;

        locals.var_vsbstar_dc_tmp = locals.var_vsbstar_dc;
        locals.var_vsbstar_dc_tmp_dn4 = locals.var_vsbstar_dc_dn4;
        locals.var_vsbstar_dc_tmp_dn6 = locals.var_vsbstar_dc_dn6;
        locals.var_vsbstar_dc_tmp_dn7 = locals.var_vsbstar_dc_dn7;
        locals.var_vsbstar_dc_tmp_dn8 = locals.var_vsbstar_dc_dn8;
        locals.var_vsbstar_dc_tmp_dn9 = locals.var_vsbstar_dc_dn9;
        locals.var_vsbstar_dc_tmp_rv = 0.0;

        locals.var_dvbstar_dc = 0.0;
        locals.var_dvbstar_dc_dn4 = 0.0;
        locals.var_dvbstar_dc_dn6 = 0.0;
        locals.var_dvbstar_dc_dn7 = 0.0;
        locals.var_dvbstar_dc_dn8 = 0.0;
        locals.var_dvbstar_dc_dn9 = 0.0;
        locals.var_dvbstar_dc_rv = 0.0;

        let assign40820_e53846: f64 = if ((p.p45 != 0.0) && (locals.var_gfacnud_i != 1.0)) { 1.0 } else { 0.0 };
        locals.var_guard1189 = assign40820_e53846;
        locals.var_guard1189_rv = 0.0;

        let (assign40830_e53856, assign40830_e53856_d_n4, assign40830_e53856_d_n6, assign40830_e53856_d_n7, assign40830_e53856_d_n8, assign40830_e53856_d_n9,) = {
    if (locals.var_guard1189 != 0.0) {
        let assign40830_e53852: f64 = (locals.var_v_ds - locals.var_vdsx);
        let assign40830_e53853: f64 = (0.5 * assign40830_e53852);
        let assign40830_e53854: f64 = (locals.var_vsbstar_dc + assign40830_e53853);
        (assign40830_e53854, locals.var_vsbstar_dc_dn4, locals.var_vsbstar_dc_dn6, (locals.var_vsbstar_dc_dn7 + (0.5 * (locals.var_v_ds_dn7 - locals.var_vdsx_dn7))), (locals.var_vsbstar_dc_dn8 + (0.5 * (locals.var_v_ds_dn8 - locals.var_vdsx_dn8))), locals.var_vsbstar_dc_dn9,)
    } else {
        (locals.var_vmb, locals.var_vmb_dn4, locals.var_vmb_dn6, locals.var_vmb_dn7, locals.var_vmb_dn8, locals.var_vmb_dn9,)
    }
};
        locals.var_vmb = assign40830_e53856;
        locals.var_vmb_dn4 = assign40830_e53856_d_n4;
        locals.var_vmb_dn6 = assign40830_e53856_d_n6;
        locals.var_vmb_dn7 = assign40830_e53856_d_n7;
        locals.var_vmb_dn8 = assign40830_e53856_d_n8;
        locals.var_vmb_dn9 = assign40830_e53856_d_n9;
        locals.var_vmb_rv = 0.0;

        let (assign40840_e53865, assign40840_e53865_d_n4, assign40840_e53865_d_n6, assign40840_e53865_d_n7, assign40840_e53865_d_n8, assign40840_e53865_d_n9,) = {
    if (locals.var_guard1189 != 0.0) {
        let assign40840_e53860: f64 = (locals.var_vmb + locals.var_phib_dc);
        let assign40840_e53861: f64 = (assign40840_e53860).sqrt();
        let assign40840_e53863: f64 = (assign40840_e53861 - locals.var_sqrt_phib_dc);
        (assign40840_e53863, (((locals.var_vmb_dn4 + locals.var_phib_dc_dn4) / (2.0 * assign40840_e53861)) - locals.var_sqrt_phib_dc_dn4), (locals.var_vmb_dn6 / (2.0 * assign40840_e53861)), (locals.var_vmb_dn7 / (2.0 * assign40840_e53861)), (locals.var_vmb_dn8 / (2.0 * assign40840_e53861)), (locals.var_vmb_dn9 / (2.0 * assign40840_e53861)),)
    } else {
        (locals.var_us, locals.var_us_dn4, locals.var_us_dn6, locals.var_us_dn7, locals.var_us_dn8, locals.var_us_dn9,)
    }
};
        locals.var_us = assign40840_e53865;
        locals.var_us_dn4 = assign40840_e53865_d_n4;
        locals.var_us_dn6 = assign40840_e53865_d_n6;
        locals.var_us_dn7 = assign40840_e53865_d_n7;
        locals.var_us_dn8 = assign40840_e53865_d_n8;
        locals.var_us_dn9 = assign40840_e53865_d_n9;
        locals.var_us_rv = 0.0;

        let (assign40850_e53877, assign40850_e53877_d_n4, assign40850_e53877_d_n6, assign40850_e53877_d_n7, assign40850_e53877_d_n8, assign40850_e53877_d_n9,) = {
    if (locals.var_guard1189 != 0.0) {
        let assign40850_e53870: f64 = (locals.var_us - locals.var_us1);
        let assign40850_e53871: f64 = (2.0 * assign40850_e53870);
        let assign40850_e53873: f64 = (assign40850_e53871 / locals.var_us21);
        let assign40850_e53875: f64 = (assign40850_e53873 - 1.0);
        (assign40850_e53875, ((((2.0 * (locals.var_us_dn4 - locals.var_us1_dn4)) * locals.var_us21) - (assign40850_e53871 * locals.var_us21_dn4)) / (locals.var_us21 * locals.var_us21)), ((2.0 * locals.var_us_dn6) / locals.var_us21), ((2.0 * locals.var_us_dn7) / locals.var_us21), ((2.0 * locals.var_us_dn8) / locals.var_us21), ((2.0 * locals.var_us_dn9) / locals.var_us21),)
    } else {
        (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
    }
};
        locals.var_temp__blk949 = assign40850_e53877;
        locals.var_temp__blk949_dn4 = assign40850_e53877_d_n4;
        locals.var_temp__blk949_dn6 = assign40850_e53877_d_n6;
        locals.var_temp__blk949_dn7 = assign40850_e53877_d_n7;
        locals.var_temp__blk949_dn8 = assign40850_e53877_d_n8;
        locals.var_temp__blk949_dn9 = assign40850_e53877_d_n9;
        locals.var_temp__blk949_rv = 0.0;

        let (assign40860_e53898, assign40860_e53898_d_n4, assign40860_e53898_d_n6, assign40860_e53898_d_n7, assign40860_e53898_d_n8, assign40860_e53898_d_n9,) = {
    if (locals.var_guard1189 != 0.0) {
        let assign40860_e53883: f64 = (1.0 - locals.var_gfacnud_i);
        let assign40860_e53884: f64 = (0.25 * assign40860_e53883);
        let assign40860_e53886: f64 = (assign40860_e53884 * locals.var_us21);
        let assign40860_e53890: f64 = (locals.var_temp__blk949 * locals.var_temp__blk949);
        let assign40860_e53892: f64 = (assign40860_e53890 + 0.4804530139182);
        let assign40860_e53893: f64 = (assign40860_e53892).sqrt();
        let assign40860_e53894: f64 = (locals.var_temp__blk949 + assign40860_e53893);
        let assign40860_e53895: f64 = (assign40860_e53886 * assign40860_e53894);
        let assign40860_e53896: f64 = (locals.var_us - assign40860_e53895);
        (assign40860_e53896, (locals.var_us_dn4 - (((assign40860_e53884 * locals.var_us21_dn4) * assign40860_e53894) + (assign40860_e53886 * (locals.var_temp__blk949_dn4 + (((locals.var_temp__blk949_dn4 * locals.var_temp__blk949) + (locals.var_temp__blk949 * locals.var_temp__blk949_dn4)) / (2.0 * assign40860_e53893)))))), (locals.var_us_dn6 - (assign40860_e53886 * (locals.var_temp__blk949_dn6 + (((locals.var_temp__blk949_dn6 * locals.var_temp__blk949) + (locals.var_temp__blk949 * locals.var_temp__blk949_dn6)) / (2.0 * assign40860_e53893))))), (locals.var_us_dn7 - (assign40860_e53886 * (locals.var_temp__blk949_dn7 + (((locals.var_temp__blk949_dn7 * locals.var_temp__blk949) + (locals.var_temp__blk949 * locals.var_temp__blk949_dn7)) / (2.0 * assign40860_e53893))))), (locals.var_us_dn8 - (assign40860_e53886 * (locals.var_temp__blk949_dn8 + (((locals.var_temp__blk949_dn8 * locals.var_temp__blk949) + (locals.var_temp__blk949 * locals.var_temp__blk949_dn8)) / (2.0 * assign40860_e53893))))), (locals.var_us_dn9 - (assign40860_e53886 * (locals.var_temp__blk949_dn9 + (((locals.var_temp__blk949_dn9 * locals.var_temp__blk949) + (locals.var_temp__blk949 * locals.var_temp__blk949_dn9)) / (2.0 * assign40860_e53893))))),)
    } else {
        (locals.var_usnew, locals.var_usnew_dn4, locals.var_usnew_dn6, locals.var_usnew_dn7, locals.var_usnew_dn8, locals.var_usnew_dn9,)
    }
};
        locals.var_usnew = assign40860_e53898;
        locals.var_usnew_dn4 = assign40860_e53898_d_n4;
        locals.var_usnew_dn6 = assign40860_e53898_d_n6;
        locals.var_usnew_dn7 = assign40860_e53898_d_n7;
        locals.var_usnew_dn8 = assign40860_e53898_d_n8;
        locals.var_usnew_dn9 = assign40860_e53898_d_n9;
        locals.var_usnew_rv = 0.0;

        let (assign40870_e53910, assign40870_e53910_d_n4, assign40870_e53910_d_n6, assign40870_e53910_d_n7, assign40870_e53910_d_n8, assign40870_e53910_d_n9,) = {
    if (locals.var_guard1189 != 0.0) {
        let assign40870_e53902: f64 = (locals.var_usnew * locals.var_usnew);
        let assign40870_e53905: f64 = (2.0 * locals.var_sqrt_phib_dc);
        let assign40870_e53907: f64 = (assign40870_e53905 * locals.var_usnew);
        let assign40870_e53908: f64 = (assign40870_e53902 + assign40870_e53907);
        (assign40870_e53908, (((locals.var_usnew_dn4 * locals.var_usnew) + (locals.var_usnew * locals.var_usnew_dn4)) + (((2.0 * locals.var_sqrt_phib_dc_dn4) * locals.var_usnew) + (assign40870_e53905 * locals.var_usnew_dn4))), (((locals.var_usnew_dn6 * locals.var_usnew) + (locals.var_usnew * locals.var_usnew_dn6)) + (assign40870_e53905 * locals.var_usnew_dn6)), (((locals.var_usnew_dn7 * locals.var_usnew) + (locals.var_usnew * locals.var_usnew_dn7)) + (assign40870_e53905 * locals.var_usnew_dn7)), (((locals.var_usnew_dn8 * locals.var_usnew) + (locals.var_usnew * locals.var_usnew_dn8)) + (assign40870_e53905 * locals.var_usnew_dn8)), (((locals.var_usnew_dn9 * locals.var_usnew) + (locals.var_usnew * locals.var_usnew_dn9)) + (assign40870_e53905 * locals.var_usnew_dn9)),)
    } else {
        (locals.var_vmbnew, locals.var_vmbnew_dn4, locals.var_vmbnew_dn6, locals.var_vmbnew_dn7, locals.var_vmbnew_dn8, locals.var_vmbnew_dn9,)
    }
};
        locals.var_vmbnew = assign40870_e53910;
        locals.var_vmbnew_dn4 = assign40870_e53910_d_n4;
        locals.var_vmbnew_dn6 = assign40870_e53910_d_n6;
        locals.var_vmbnew_dn7 = assign40870_e53910_d_n7;
        locals.var_vmbnew_dn8 = assign40870_e53910_d_n8;
        locals.var_vmbnew_dn9 = assign40870_e53910_d_n9;
        locals.var_vmbnew_rv = 0.0;

        let (assign40880_e53920, assign40880_e53920_d_n4, assign40880_e53920_d_n6, assign40880_e53920_d_n7, assign40880_e53920_d_n8, assign40880_e53920_d_n9,) = {
    if (locals.var_guard1189 != 0.0) {
        let assign40880_e53916: f64 = (locals.var_v_ds - locals.var_vdsx);
        let assign40880_e53917: f64 = (0.5 * assign40880_e53916);
        let assign40880_e53918: f64 = (locals.var_vmbnew - assign40880_e53917);
        (assign40880_e53918, locals.var_vmbnew_dn4, locals.var_vmbnew_dn6, (locals.var_vmbnew_dn7 - (0.5 * (locals.var_v_ds_dn7 - locals.var_vdsx_dn7))), (locals.var_vmbnew_dn8 - (0.5 * (locals.var_v_ds_dn8 - locals.var_vdsx_dn8))), locals.var_vmbnew_dn9,)
    } else {
        (locals.var_vsbstar_dc, locals.var_vsbstar_dc_dn4, locals.var_vsbstar_dc_dn6, locals.var_vsbstar_dc_dn7, locals.var_vsbstar_dc_dn8, locals.var_vsbstar_dc_dn9,)
    }
};
        locals.var_vsbstar_dc = assign40880_e53920;
        locals.var_vsbstar_dc_dn4 = assign40880_e53920_d_n4;
        locals.var_vsbstar_dc_dn6 = assign40880_e53920_d_n6;
        locals.var_vsbstar_dc_dn7 = assign40880_e53920_d_n7;
        locals.var_vsbstar_dc_dn8 = assign40880_e53920_d_n8;
        locals.var_vsbstar_dc_dn9 = assign40880_e53920_d_n9;
        locals.var_vsbstar_dc_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_16(
        locals: &mut StampLocals,
    ) {
        let (assign40890_e53926, assign40890_e53926_d_n4, assign40890_e53926_d_n6, assign40890_e53926_d_n7, assign40890_e53926_d_n8, assign40890_e53926_d_n9,) = {
    if (locals.var_guard1189 != 0.0) {
        let assign40890_e53924: f64 = (locals.var_vsbstar_dc_tmp - locals.var_vsbstar_dc);
        (assign40890_e53924, (locals.var_vsbstar_dc_tmp_dn4 - locals.var_vsbstar_dc_dn4), (locals.var_vsbstar_dc_tmp_dn6 - locals.var_vsbstar_dc_dn6), (locals.var_vsbstar_dc_tmp_dn7 - locals.var_vsbstar_dc_dn7), (locals.var_vsbstar_dc_tmp_dn8 - locals.var_vsbstar_dc_dn8), (locals.var_vsbstar_dc_tmp_dn9 - locals.var_vsbstar_dc_dn9),)
    } else {
        (locals.var_dvbstar_dc, locals.var_dvbstar_dc_dn4, locals.var_dvbstar_dc_dn6, locals.var_dvbstar_dc_dn7, locals.var_dvbstar_dc_dn8, locals.var_dvbstar_dc_dn9,)
    }
};
        locals.var_dvbstar_dc = assign40890_e53926;
        locals.var_dvbstar_dc_dn4 = assign40890_e53926_d_n4;
        locals.var_dvbstar_dc_dn6 = assign40890_e53926_d_n6;
        locals.var_dvbstar_dc_dn7 = assign40890_e53926_d_n7;
        locals.var_dvbstar_dc_dn8 = assign40890_e53926_d_n8;
        locals.var_dvbstar_dc_dn9 = assign40890_e53926_d_n9;
        locals.var_dvbstar_dc_rv = 0.0;

        locals.var_phib = locals.var_phib_dc;
        locals.var_phib_dn4 = locals.var_phib_dc_dn4;
        locals.var_phib_rv = 0.0;

        locals.var_aphi = locals.var_aphi_dc;
        locals.var_aphi_dn4 = locals.var_aphi_dc_dn4;
        locals.var_aphi_rv = 0.0;

        locals.var_g_0 = locals.var_g_0_dc;
        locals.var_g_0_dn4 = locals.var_g_0_dc_dn4;
        locals.var_g_0_rv = 0.0;

        locals.var_vsbstar = locals.var_vsbstar_dc;
        locals.var_vsbstar_dn4 = locals.var_vsbstar_dc_dn4;
        locals.var_vsbstar_dn6 = locals.var_vsbstar_dc_dn6;
        locals.var_vsbstar_dn7 = locals.var_vsbstar_dc_dn7;
        locals.var_vsbstar_dn8 = locals.var_vsbstar_dc_dn8;
        locals.var_vsbstar_dn9 = locals.var_vsbstar_dc_dn9;
        locals.var_vsbstar_rv = 0.0;

        locals.var_dvbstar = locals.var_dvbstar_dc;
        locals.var_dvbstar_dn4 = locals.var_dvbstar_dc_dn4;
        locals.var_dvbstar_dn6 = locals.var_dvbstar_dc_dn6;
        locals.var_dvbstar_dn7 = locals.var_dvbstar_dc_dn7;
        locals.var_dvbstar_dn8 = locals.var_dvbstar_dc_dn8;
        locals.var_dvbstar_dn9 = locals.var_dvbstar_dc_dn9;
        locals.var_dvbstar_rv = 0.0;

        locals.var_thesatloc = locals.var_thesat_t;
        locals.var_thesatloc_dn4 = locals.var_thesat_t_dn4;
        locals.var_thesatloc_rv = 0.0;

        locals.var_arloc = locals.var_ar;
        locals.var_arloc_rv = 0.0;

        let assign40970_e53936: f64 = (locals.var_vgb - locals.var_dvbstar);
        let assign40970_e53938: f64 = (assign40970_e53936 - locals.var_vfb_t);
        locals.var_vgb1 = assign40970_e53938;
        locals.var_vgb1_dn4 = ((-locals.var_dvbstar_dn4) - locals.var_vfb_t_dn4);
        locals.var_vgb1_dn6 = (locals.var_vgb_dn6 - locals.var_dvbstar_dn6);
        locals.var_vgb1_dn7 = (locals.var_vgb_dn7 - locals.var_dvbstar_dn7);
        locals.var_vgb1_dn8 = (locals.var_vgb_dn8 - locals.var_dvbstar_dn8);
        locals.var_vgb1_dn9 = (locals.var_vgb_dn9 - locals.var_dvbstar_dn9);
        locals.var_vgb1_rv = 0.0;

        let assign40980_e53943: f64 = (locals.var_v_ds - locals.var_vdsx);
        let assign40980_e53944: f64 = (0.5 * assign40980_e53943);
        let assign40980_e53945: f64 = (locals.var_vsbstar + assign40980_e53944);
        locals.var_vsbx = assign40980_e53945;
        locals.var_vsbx_dn4 = locals.var_vsbstar_dn4;
        locals.var_vsbx_dn6 = locals.var_vsbstar_dn6;
        locals.var_vsbx_dn7 = (locals.var_vsbstar_dn7 + (0.5 * (locals.var_v_ds_dn7 - locals.var_vdsx_dn7)));
        locals.var_vsbx_dn8 = (locals.var_vsbstar_dn8 + (0.5 * (locals.var_v_ds_dn8 - locals.var_vdsx_dn8)));
        locals.var_vsbx_dn9 = locals.var_vsbstar_dn9;
        locals.var_vsbx_rv = 0.0;

        locals.var_dctg = 1.0;
        locals.var_dctg_dn4 = 0.0;
        locals.var_dctg_dn6 = 0.0;
        locals.var_dctg_dn7 = 0.0;
        locals.var_dctg_dn8 = 0.0;
        locals.var_dctg_dn9 = 0.0;
        locals.var_dctg_rv = 0.0;

        let assign41000_e53949: f64 = if locals.var_ctg_i > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1190 = assign41000_e53949;
        locals.var_guard1190_rv = 0.0;

        let (assign41010_e53955, assign41010_e53955_d_n4,) = {
    if (locals.var_guard1190 != 0.0) {
        let assign41010_e53953: f64 = (locals.var_phib * locals.var_inv_phit);
        (assign41010_e53953, ((locals.var_phib_dn4 * locals.var_inv_phit) + (locals.var_phib * locals.var_inv_phit_dn4)),)
    } else {
        (locals.var_xbct, locals.var_xbct_dn4,)
    }
};
        locals.var_xbct = assign41010_e53955;
        locals.var_xbct_dn4 = assign41010_e53955_d_n4;
        locals.var_xbct_rv = 0.0;

        let (assign41020_e53961, assign41020_e53961_d_n4, assign41020_e53961_d_n6, assign41020_e53961_d_n7, assign41020_e53961_d_n8, assign41020_e53961_d_n9,) = {
    if (locals.var_guard1190 != 0.0) {
        let assign41020_e53959: f64 = (locals.var_vsbx * locals.var_inv_phit);
        (assign41020_e53959, ((locals.var_vsbx_dn4 * locals.var_inv_phit) + (locals.var_vsbx * locals.var_inv_phit_dn4)), (locals.var_vsbx_dn6 * locals.var_inv_phit), (locals.var_vsbx_dn7 * locals.var_inv_phit), (locals.var_vsbx_dn8 * locals.var_inv_phit), (locals.var_vsbx_dn9 * locals.var_inv_phit),)
    } else {
        (locals.var_xsbstar, locals.var_xsbstar_dn4, locals.var_xsbstar_dn6, locals.var_xsbstar_dn7, locals.var_xsbstar_dn8, locals.var_xsbstar_dn9,)
    }
};
        locals.var_xsbstar = assign41020_e53961;
        locals.var_xsbstar_dn4 = assign41020_e53961_d_n4;
        locals.var_xsbstar_dn6 = assign41020_e53961_d_n6;
        locals.var_xsbstar_dn7 = assign41020_e53961_d_n7;
        locals.var_xsbstar_dn8 = assign41020_e53961_d_n8;
        locals.var_xsbstar_dn9 = assign41020_e53961_d_n9;
        locals.var_xsbstar_rv = 0.0;

        let (assign41030_e53967, assign41030_e53967_d_n4, assign41030_e53967_d_n6, assign41030_e53967_d_n7, assign41030_e53967_d_n8, assign41030_e53967_d_n9,) = {
    if (locals.var_guard1190 != 0.0) {
        let assign41030_e53965: f64 = (locals.var_vgb1 * locals.var_inv_phit);
        (assign41030_e53965, ((locals.var_vgb1_dn4 * locals.var_inv_phit) + (locals.var_vgb1 * locals.var_inv_phit_dn4)), (locals.var_vgb1_dn6 * locals.var_inv_phit), (locals.var_vgb1_dn7 * locals.var_inv_phit), (locals.var_vgb1_dn8 * locals.var_inv_phit), (locals.var_vgb1_dn9 * locals.var_inv_phit),)
    } else {
        (locals.var_xgct, locals.var_xgct_dn4, locals.var_xgct_dn6, locals.var_xgct_dn7, locals.var_xgct_dn8, locals.var_xgct_dn9,)
    }
};
        locals.var_xgct = assign41030_e53967;
        locals.var_xgct_dn4 = assign41030_e53967_d_n4;
        locals.var_xgct_dn6 = assign41030_e53967_d_n6;
        locals.var_xgct_dn7 = assign41030_e53967_d_n7;
        locals.var_xgct_dn8 = assign41030_e53967_d_n8;
        locals.var_xgct_dn9 = assign41030_e53967_d_n9;
        locals.var_xgct_rv = 0.0;

        let (assign41040_e53978, assign41040_e53978_d_n4, assign41040_e53978_d_n6, assign41040_e53978_d_n7, assign41040_e53978_d_n8, assign41040_e53978_d_n9,) = {
    if (locals.var_guard1190 != 0.0) {
        let assign41040_e53972: f64 = (0.5 * locals.var_g_0);
        let assign41040_e53974: f64 = (locals.var_xbct).sqrt();
        let assign41040_e53975: f64 = (assign41040_e53972 / assign41040_e53974);
        let assign41040_e53976: f64 = (1.0 + assign41040_e53975);
        (assign41040_e53976, ((((0.5 * locals.var_g_0_dn4) * assign41040_e53974) - (assign41040_e53972 * (locals.var_xbct_dn4 / (2.0 * assign41040_e53974)))) / (assign41040_e53974 * assign41040_e53974)), 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign41040_e53978;
        locals.var_temp1_dn4 = assign41040_e53978_d_n4;
        locals.var_temp1_dn6 = assign41040_e53978_d_n6;
        locals.var_temp1_dn7 = assign41040_e53978_d_n7;
        locals.var_temp1_dn8 = assign41040_e53978_d_n8;
        locals.var_temp1_dn9 = assign41040_e53978_d_n9;
        locals.var_temp1_rv = 0.0;

        let (assign41050_e53987, assign41050_e53987_d_n4, assign41050_e53987_d_n6, assign41050_e53987_d_n7, assign41050_e53987_d_n8, assign41050_e53987_d_n9,) = {
    if (locals.var_guard1190 != 0.0) {
        let assign41050_e53983: f64 = (locals.var_xbct).sqrt();
        let assign41050_e53984: f64 = (locals.var_g_0 * assign41050_e53983);
        let assign41050_e53985: f64 = (locals.var_xbct + assign41050_e53984);
        (assign41050_e53985, (locals.var_xbct_dn4 + ((locals.var_g_0_dn4 * assign41050_e53983) + (locals.var_g_0 * (locals.var_xbct_dn4 / (2.0 * assign41050_e53983))))), 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9,)
    }
};
        locals.var_temp2 = assign41050_e53987;
        locals.var_temp2_dn4 = assign41050_e53987_d_n4;
        locals.var_temp2_dn6 = assign41050_e53987_d_n6;
        locals.var_temp2_dn7 = assign41050_e53987_d_n7;
        locals.var_temp2_dn8 = assign41050_e53987_d_n8;
        locals.var_temp2_dn9 = assign41050_e53987_d_n9;
        locals.var_temp2_rv = 0.0;

        let (assign41060_e54005, assign41060_e54005_d_n4, assign41060_e54005_d_n6, assign41060_e54005_d_n7, assign41060_e54005_d_n8, assign41060_e54005_d_n9,) = {
    if (locals.var_guard1190 != 0.0) {
        let assign41060_e53991: f64 = (locals.var_xgct - locals.var_temp2);
        let assign41060_e53993: f64 = (assign41060_e53991 / locals.var_temp1);
        let assign41060_e53996: f64 = (0.5 * locals.var_xbct);
        let assign41060_e53997: f64 = (assign41060_e53993 + assign41060_e53996);
        let assign41060_e54000: f64 = (1.0 + locals.var_ctb_i);
        let assign41060_e54002: f64 = (assign41060_e54000 * locals.var_xsbstar);
        let assign41060_e54003: f64 = (assign41060_e53997 - assign41060_e54002);
        (assign41060_e54003, ((((((locals.var_xgct_dn4 - locals.var_temp2_dn4) * locals.var_temp1) - (assign41060_e53991 * locals.var_temp1_dn4)) / (locals.var_temp1 * locals.var_temp1)) + (0.5 * locals.var_xbct_dn4)) - (assign41060_e54000 * locals.var_xsbstar_dn4)), (((((locals.var_xgct_dn6 - locals.var_temp2_dn6) * locals.var_temp1) - (assign41060_e53991 * locals.var_temp1_dn6)) / (locals.var_temp1 * locals.var_temp1)) - (assign41060_e54000 * locals.var_xsbstar_dn6)), (((((locals.var_xgct_dn7 - locals.var_temp2_dn7) * locals.var_temp1) - (assign41060_e53991 * locals.var_temp1_dn7)) / (locals.var_temp1 * locals.var_temp1)) - (assign41060_e54000 * locals.var_xsbstar_dn7)), (((((locals.var_xgct_dn8 - locals.var_temp2_dn8) * locals.var_temp1) - (assign41060_e53991 * locals.var_temp1_dn8)) / (locals.var_temp1 * locals.var_temp1)) - (assign41060_e54000 * locals.var_xsbstar_dn8)), (((((locals.var_xgct_dn9 - locals.var_temp2_dn9) * locals.var_temp1) - (assign41060_e53991 * locals.var_temp1_dn9)) / (locals.var_temp1 * locals.var_temp1)) - (assign41060_e54000 * locals.var_xsbstar_dn9)),)
    } else {
        (locals.var_xwict, locals.var_xwict_dn4, locals.var_xwict_dn6, locals.var_xwict_dn7, locals.var_xwict_dn8, locals.var_xwict_dn9,)
    }
};
        locals.var_xwict = assign41060_e54005;
        locals.var_xwict_dn4 = assign41060_e54005_d_n4;
        locals.var_xwict_dn6 = assign41060_e54005_d_n6;
        locals.var_xwict_dn7 = assign41060_e54005_d_n7;
        locals.var_xwict_dn8 = assign41060_e54005_d_n8;
        locals.var_xwict_dn9 = assign41060_e54005_d_n9;
        locals.var_xwict_rv = 0.0;

        let (assign41070_e54013, assign41070_e54013_d_n4,) = {
    if (locals.var_guard1190 != 0.0) {
        let assign41070_e54009: f64 = (0.5 * locals.var_xbct);
        let assign41070_e54011: f64 = (assign41070_e54009 + 2.0);
        (assign41070_e54011, (0.5 * locals.var_xbct_dn4),)
    } else {
        (locals.var_xctmax, locals.var_xctmax_dn4,)
    }
};
        locals.var_xctmax = assign41070_e54013;
        locals.var_xctmax_dn4 = assign41070_e54013_d_n4;
        locals.var_xctmax_rv = 0.0;

        let (assign41080_e54019, assign41080_e54019_d_n4, assign41080_e54019_d_n6, assign41080_e54019_d_n7, assign41080_e54019_d_n8, assign41080_e54019_d_n9,) = {
    if (locals.var_guard1190 != 0.0) {
        let assign41080_e54017: f64 = (locals.var_xbct + locals.var_xsbstar);
        (assign41080_e54017, (locals.var_xbct_dn4 + locals.var_xsbstar_dn4), locals.var_xsbstar_dn6, locals.var_xsbstar_dn7, locals.var_xsbstar_dn8, locals.var_xsbstar_dn9,)
    } else {
        (locals.var_xnct, locals.var_xnct_dn4, locals.var_xnct_dn6, locals.var_xnct_dn7, locals.var_xnct_dn8, locals.var_xnct_dn9,)
    }
};
        locals.var_xnct = assign41080_e54019;
        locals.var_xnct_dn4 = assign41080_e54019_d_n4;
        locals.var_xnct_dn6 = assign41080_e54019_d_n6;
        locals.var_xnct_dn7 = assign41080_e54019_d_n7;
        locals.var_xnct_dn8 = assign41080_e54019_d_n8;
        locals.var_xnct_dn9 = assign41080_e54019_d_n9;
        locals.var_xnct_rv = 0.0;

        let (assign41090_e54040, assign41090_e54040_d_n4, assign41090_e54040_d_n6, assign41090_e54040_d_n7, assign41090_e54040_d_n8, assign41090_e54040_d_n9,) = {
    if (locals.var_guard1190 != 0.0) {
        let assign41090_e54023: f64 = (locals.var_xgct - locals.var_xnct);
        let assign41090_e54026: f64 = (locals.var_xnct).sqrt();
        let assign41090_e54027: f64 = (locals.var_g_0 * assign41090_e54026);
        let assign41090_e54028: f64 = (assign41090_e54023 - assign41090_e54027);
        let assign41090_e54032: f64 = (locals.var_xbct / locals.var_g_0);
        let assign41090_e54034: f64 = (locals.var_xbct).sqrt();
        let assign41090_e54035: f64 = (assign41090_e54032 + assign41090_e54034);
        let assign41090_e54036: f64 = (assign41090_e54035).ln();
        let assign41090_e54037: f64 = (2.0 * assign41090_e54036);
        let assign41090_e54038: f64 = (assign41090_e54028 - assign41090_e54037);
        (assign41090_e54038, (((locals.var_xgct_dn4 - locals.var_xnct_dn4) - ((locals.var_g_0_dn4 * assign41090_e54026) + (locals.var_g_0 * (locals.var_xnct_dn4 / (2.0 * assign41090_e54026))))) - (2.0 * (((((locals.var_xbct_dn4 * locals.var_g_0) - (locals.var_xbct * locals.var_g_0_dn4)) / (locals.var_g_0 * locals.var_g_0)) + (locals.var_xbct_dn4 / (2.0 * assign41090_e54034))) / assign41090_e54035))), ((locals.var_xgct_dn6 - locals.var_xnct_dn6) - (locals.var_g_0 * (locals.var_xnct_dn6 / (2.0 * assign41090_e54026)))), ((locals.var_xgct_dn7 - locals.var_xnct_dn7) - (locals.var_g_0 * (locals.var_xnct_dn7 / (2.0 * assign41090_e54026)))), ((locals.var_xgct_dn8 - locals.var_xnct_dn8) - (locals.var_g_0 * (locals.var_xnct_dn8 / (2.0 * assign41090_e54026)))), ((locals.var_xgct_dn9 - locals.var_xnct_dn9) - (locals.var_g_0 * (locals.var_xnct_dn9 / (2.0 * assign41090_e54026)))),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign41090_e54040;
        locals.var_temp1_dn4 = assign41090_e54040_d_n4;
        locals.var_temp1_dn6 = assign41090_e54040_d_n6;
        locals.var_temp1_dn7 = assign41090_e54040_d_n7;
        locals.var_temp1_dn8 = assign41090_e54040_d_n8;
        locals.var_temp1_dn9 = assign41090_e54040_d_n9;
        locals.var_temp1_rv = 0.0;

        let (assign41100_e54048, assign41100_e54048_d_n4, assign41100_e54048_d_n6, assign41100_e54048_d_n7, assign41100_e54048_d_n8, assign41100_e54048_d_n9,) = {
    if (locals.var_guard1190 != 0.0) {
        let assign41100_e54044: f64 = (2.0 * locals.var_temp1);
        let assign41100_e54046: f64 = (assign41100_e54044 + locals.var_xctmax);
        (assign41100_e54046, ((2.0 * locals.var_temp1_dn4) + locals.var_xctmax_dn4), (2.0 * locals.var_temp1_dn6), (2.0 * locals.var_temp1_dn7), (2.0 * locals.var_temp1_dn8), (2.0 * locals.var_temp1_dn9),)
    } else {
        (locals.var_xmict, locals.var_xmict_dn4, locals.var_xmict_dn6, locals.var_xmict_dn7, locals.var_xmict_dn8, locals.var_xmict_dn9,)
    }
};
        locals.var_xmict = assign41100_e54048;
        locals.var_xmict_dn4 = assign41100_e54048_d_n4;
        locals.var_xmict_dn6 = assign41100_e54048_d_n6;
        locals.var_xmict_dn7 = assign41100_e54048_d_n7;
        locals.var_xmict_dn8 = assign41100_e54048_d_n8;
        locals.var_xmict_dn9 = assign41100_e54048_d_n9;
        locals.var_xmict_rv = 0.0;

        let (assign41110_e54067, assign41110_e54067_d_n4, assign41110_e54067_d_n6, assign41110_e54067_d_n7, assign41110_e54067_d_n8, assign41110_e54067_d_n9,) = {
    if (locals.var_guard1190 != 0.0) {
        let assign41110_e54053: f64 = (locals.var_xwict + locals.var_xmict);
        let assign41110_e54056: f64 = (locals.var_xwict - locals.var_xmict);
        let assign41110_e54059: f64 = (locals.var_xwict - locals.var_xmict);
        let assign41110_e54060: f64 = (assign41110_e54056 * assign41110_e54059);
        let assign41110_e54062: f64 = (assign41110_e54060 + 20.0);
        let assign41110_e54063: f64 = (assign41110_e54062).sqrt();
        let assign41110_e54064: f64 = (assign41110_e54053 + assign41110_e54063);
        let assign41110_e54065: f64 = (0.5 * assign41110_e54064);
        (assign41110_e54065, (0.5 * ((locals.var_xwict_dn4 + locals.var_xmict_dn4) + ((((locals.var_xwict_dn4 - locals.var_xmict_dn4) * assign41110_e54059) + (assign41110_e54056 * (locals.var_xwict_dn4 - locals.var_xmict_dn4))) / (2.0 * assign41110_e54063)))), (0.5 * ((locals.var_xwict_dn6 + locals.var_xmict_dn6) + ((((locals.var_xwict_dn6 - locals.var_xmict_dn6) * assign41110_e54059) + (assign41110_e54056 * (locals.var_xwict_dn6 - locals.var_xmict_dn6))) / (2.0 * assign41110_e54063)))), (0.5 * ((locals.var_xwict_dn7 + locals.var_xmict_dn7) + ((((locals.var_xwict_dn7 - locals.var_xmict_dn7) * assign41110_e54059) + (assign41110_e54056 * (locals.var_xwict_dn7 - locals.var_xmict_dn7))) / (2.0 * assign41110_e54063)))), (0.5 * ((locals.var_xwict_dn8 + locals.var_xmict_dn8) + ((((locals.var_xwict_dn8 - locals.var_xmict_dn8) * assign41110_e54059) + (assign41110_e54056 * (locals.var_xwict_dn8 - locals.var_xmict_dn8))) / (2.0 * assign41110_e54063)))), (0.5 * ((locals.var_xwict_dn9 + locals.var_xmict_dn9) + ((((locals.var_xwict_dn9 - locals.var_xmict_dn9) * assign41110_e54059) + (assign41110_e54056 * (locals.var_xwict_dn9 - locals.var_xmict_dn9))) / (2.0 * assign41110_e54063)))),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign41110_e54067;
        locals.var_temp1_dn4 = assign41110_e54067_d_n4;
        locals.var_temp1_dn6 = assign41110_e54067_d_n6;
        locals.var_temp1_dn7 = assign41110_e54067_d_n7;
        locals.var_temp1_dn8 = assign41110_e54067_d_n8;
        locals.var_temp1_dn9 = assign41110_e54067_d_n9;
        locals.var_temp1_rv = 0.0;

        let (assign41120_e54077, assign41120_e54077_d_n4, assign41120_e54077_d_n6, assign41120_e54077_d_n7, assign41120_e54077_d_n8, assign41120_e54077_d_n9,) = {
    if (locals.var_guard1190 != 0.0) {
        let assign41120_e54072: f64 = (locals.var_xgct - locals.var_xsbstar);
        let assign41120_e54073: f64 = (2.0 * assign41120_e54072);
        let assign41120_e54075: f64 = (assign41120_e54073 - locals.var_xctmax);
        (assign41120_e54075, ((2.0 * (locals.var_xgct_dn4 - locals.var_xsbstar_dn4)) - locals.var_xctmax_dn4), (2.0 * (locals.var_xgct_dn6 - locals.var_xsbstar_dn6)), (2.0 * (locals.var_xgct_dn7 - locals.var_xsbstar_dn7)), (2.0 * (locals.var_xgct_dn8 - locals.var_xsbstar_dn8)), (2.0 * (locals.var_xgct_dn9 - locals.var_xsbstar_dn9)),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9,)
    }
};
        locals.var_temp2 = assign41120_e54077;
        locals.var_temp2_dn4 = assign41120_e54077_d_n4;
        locals.var_temp2_dn6 = assign41120_e54077_d_n6;
        locals.var_temp2_dn7 = assign41120_e54077_d_n7;
        locals.var_temp2_dn8 = assign41120_e54077_d_n8;
        locals.var_temp2_dn9 = assign41120_e54077_d_n9;
        locals.var_temp2_rv = 0.0;

        let (assign41130_e54096, assign41130_e54096_d_n4, assign41130_e54096_d_n6, assign41130_e54096_d_n7, assign41130_e54096_d_n8, assign41130_e54096_d_n9,) = {
    if (locals.var_guard1190 != 0.0) {
        let assign41130_e54082: f64 = (locals.var_temp1 + locals.var_temp2);
        let assign41130_e54085: f64 = (locals.var_temp1 - locals.var_temp2);
        let assign41130_e54088: f64 = (locals.var_temp1 - locals.var_temp2);
        let assign41130_e54089: f64 = (assign41130_e54085 * assign41130_e54088);
        let assign41130_e54091: f64 = (assign41130_e54089 + 20.0);
        let assign41130_e54092: f64 = (assign41130_e54091).sqrt();
        let assign41130_e54093: f64 = (assign41130_e54082 - assign41130_e54092);
        let assign41130_e54094: f64 = (0.5 * assign41130_e54093);
        (assign41130_e54094, (0.5 * ((locals.var_temp1_dn4 + locals.var_temp2_dn4) - ((((locals.var_temp1_dn4 - locals.var_temp2_dn4) * assign41130_e54088) + (assign41130_e54085 * (locals.var_temp1_dn4 - locals.var_temp2_dn4))) / (2.0 * assign41130_e54092)))), (0.5 * ((locals.var_temp1_dn6 + locals.var_temp2_dn6) - ((((locals.var_temp1_dn6 - locals.var_temp2_dn6) * assign41130_e54088) + (assign41130_e54085 * (locals.var_temp1_dn6 - locals.var_temp2_dn6))) / (2.0 * assign41130_e54092)))), (0.5 * ((locals.var_temp1_dn7 + locals.var_temp2_dn7) - ((((locals.var_temp1_dn7 - locals.var_temp2_dn7) * assign41130_e54088) + (assign41130_e54085 * (locals.var_temp1_dn7 - locals.var_temp2_dn7))) / (2.0 * assign41130_e54092)))), (0.5 * ((locals.var_temp1_dn8 + locals.var_temp2_dn8) - ((((locals.var_temp1_dn8 - locals.var_temp2_dn8) * assign41130_e54088) + (assign41130_e54085 * (locals.var_temp1_dn8 - locals.var_temp2_dn8))) / (2.0 * assign41130_e54092)))), (0.5 * ((locals.var_temp1_dn9 + locals.var_temp2_dn9) - ((((locals.var_temp1_dn9 - locals.var_temp2_dn9) * assign41130_e54088) + (assign41130_e54085 * (locals.var_temp1_dn9 - locals.var_temp2_dn9))) / (2.0 * assign41130_e54092)))),)
    } else {
        (locals.var_xsubct, locals.var_xsubct_dn4, locals.var_xsubct_dn6, locals.var_xsubct_dn7, locals.var_xsubct_dn8, locals.var_xsubct_dn9,)
    }
};
        locals.var_xsubct = assign41130_e54096;
        locals.var_xsubct_dn4 = assign41130_e54096_d_n4;
        locals.var_xsubct_dn6 = assign41130_e54096_d_n6;
        locals.var_xsubct_dn7 = assign41130_e54096_d_n7;
        locals.var_xsubct_dn8 = assign41130_e54096_d_n8;
        locals.var_xsubct_dn9 = assign41130_e54096_d_n9;
        locals.var_xsubct_rv = 0.0;

        let (assign41140_e54115, assign41140_e54115_d_n4, assign41140_e54115_d_n6, assign41140_e54115_d_n7, assign41140_e54115_d_n8, assign41140_e54115_d_n9,) = {
    if (locals.var_guard1190 != 0.0) {
        let assign41140_e54101: f64 = (locals.var_xsubct + locals.var_xctmax);
        let assign41140_e54104: f64 = (locals.var_xsubct - locals.var_xctmax);
        let assign41140_e54107: f64 = (locals.var_xsubct - locals.var_xctmax);
        let assign41140_e54108: f64 = (assign41140_e54104 * assign41140_e54107);
        let assign41140_e54110: f64 = (assign41140_e54108 + 5.0);
        let assign41140_e54111: f64 = (assign41140_e54110).sqrt();
        let assign41140_e54112: f64 = (assign41140_e54101 - assign41140_e54111);
        let assign41140_e54113: f64 = (0.5 * assign41140_e54112);
        (assign41140_e54113, (0.5 * ((locals.var_xsubct_dn4 + locals.var_xctmax_dn4) - ((((locals.var_xsubct_dn4 - locals.var_xctmax_dn4) * assign41140_e54107) + (assign41140_e54104 * (locals.var_xsubct_dn4 - locals.var_xctmax_dn4))) / (2.0 * assign41140_e54111)))), (0.5 * (locals.var_xsubct_dn6 - (((locals.var_xsubct_dn6 * assign41140_e54107) + (assign41140_e54104 * locals.var_xsubct_dn6)) / (2.0 * assign41140_e54111)))), (0.5 * (locals.var_xsubct_dn7 - (((locals.var_xsubct_dn7 * assign41140_e54107) + (assign41140_e54104 * locals.var_xsubct_dn7)) / (2.0 * assign41140_e54111)))), (0.5 * (locals.var_xsubct_dn8 - (((locals.var_xsubct_dn8 * assign41140_e54107) + (assign41140_e54104 * locals.var_xsubct_dn8)) / (2.0 * assign41140_e54111)))), (0.5 * (locals.var_xsubct_dn9 - (((locals.var_xsubct_dn9 * assign41140_e54107) + (assign41140_e54104 * locals.var_xsubct_dn9)) / (2.0 * assign41140_e54111)))),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign41140_e54115;
        locals.var_temp1_dn4 = assign41140_e54115_d_n4;
        locals.var_temp1_dn6 = assign41140_e54115_d_n6;
        locals.var_temp1_dn7 = assign41140_e54115_d_n7;
        locals.var_temp1_dn8 = assign41140_e54115_d_n8;
        locals.var_temp1_dn9 = assign41140_e54115_d_n9;
        locals.var_temp1_rv = 0.0;

        let (assign41150_e54137, assign41150_e54137_d_n4, assign41150_e54137_d_n6, assign41150_e54137_d_n7, assign41150_e54137_d_n8, assign41150_e54137_d_n9,) = {
    if (locals.var_guard1190 != 0.0) {
        let assign41150_e54120: f64 = (-locals.var_xctmax);
        let assign41150_e54121: f64 = (locals.var_temp1 + assign41150_e54120);
        let assign41150_e54124: f64 = (-locals.var_xctmax);
        let assign41150_e54125: f64 = (locals.var_temp1 - assign41150_e54124);
        let assign41150_e54128: f64 = (-locals.var_xctmax);
        let assign41150_e54129: f64 = (locals.var_temp1 - assign41150_e54128);
        let assign41150_e54130: f64 = (assign41150_e54125 * assign41150_e54129);
        let assign41150_e54132: f64 = (assign41150_e54130 + 20.0);
        let assign41150_e54133: f64 = (assign41150_e54132).sqrt();
        let assign41150_e54134: f64 = (assign41150_e54121 + assign41150_e54133);
        let assign41150_e54135: f64 = (0.5 * assign41150_e54134);
        (assign41150_e54135, (0.5 * ((locals.var_temp1_dn4 + (-locals.var_xctmax_dn4)) + ((((locals.var_temp1_dn4 - (-locals.var_xctmax_dn4)) * assign41150_e54129) + (assign41150_e54125 * (locals.var_temp1_dn4 - (-locals.var_xctmax_dn4)))) / (2.0 * assign41150_e54133)))), (0.5 * (locals.var_temp1_dn6 + (((locals.var_temp1_dn6 * assign41150_e54129) + (assign41150_e54125 * locals.var_temp1_dn6)) / (2.0 * assign41150_e54133)))), (0.5 * (locals.var_temp1_dn7 + (((locals.var_temp1_dn7 * assign41150_e54129) + (assign41150_e54125 * locals.var_temp1_dn7)) / (2.0 * assign41150_e54133)))), (0.5 * (locals.var_temp1_dn8 + (((locals.var_temp1_dn8 * assign41150_e54129) + (assign41150_e54125 * locals.var_temp1_dn8)) / (2.0 * assign41150_e54133)))), (0.5 * (locals.var_temp1_dn9 + (((locals.var_temp1_dn9 * assign41150_e54129) + (assign41150_e54125 * locals.var_temp1_dn9)) / (2.0 * assign41150_e54133)))),)
    } else {
        (locals.var_xct, locals.var_xct_dn4, locals.var_xct_dn6, locals.var_xct_dn7, locals.var_xct_dn8, locals.var_xct_dn9,)
    }
};
        locals.var_xct = assign41150_e54137;
        locals.var_xct_dn4 = assign41150_e54137_d_n4;
        locals.var_xct_dn6 = assign41150_e54137_d_n6;
        locals.var_xct_dn7 = assign41150_e54137_d_n7;
        locals.var_xct_dn8 = assign41150_e54137_d_n8;
        locals.var_xct_dn9 = assign41150_e54137_d_n9;
        locals.var_xct_rv = 0.0;

        let (assign41160_e54147, assign41160_e54147_d_n4, assign41160_e54147_d_n6, assign41160_e54147_d_n7, assign41160_e54147_d_n8, assign41160_e54147_d_n9,) = {
    if (locals.var_guard1190 != 0.0) {
        let assign41160_e54142: f64 = (locals.var_xct / locals.var_xctmax);
        let assign41160_e54144: f64 = (assign41160_e54142 + 1.0);
        let assign41160_e54145: f64 = (locals.var_ctg_t * assign41160_e54144);
        (assign41160_e54145, ((locals.var_ctg_t_dn4 * assign41160_e54144) + (locals.var_ctg_t * (((locals.var_xct_dn4 * locals.var_xctmax) - (locals.var_xct * locals.var_xctmax_dn4)) / (locals.var_xctmax * locals.var_xctmax)))), (locals.var_ctg_t * (locals.var_xct_dn6 / locals.var_xctmax)), (locals.var_ctg_t * (locals.var_xct_dn7 / locals.var_xctmax)), (locals.var_ctg_t * (locals.var_xct_dn8 / locals.var_xctmax)), (locals.var_ctg_t * (locals.var_xct_dn9 / locals.var_xctmax)),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9,)
    }
};
        locals.var_temp2 = assign41160_e54147;
        locals.var_temp2_dn4 = assign41160_e54147_d_n4;
        locals.var_temp2_dn6 = assign41160_e54147_d_n6;
        locals.var_temp2_dn7 = assign41160_e54147_d_n7;
        locals.var_temp2_dn8 = assign41160_e54147_d_n8;
        locals.var_temp2_dn9 = assign41160_e54147_d_n9;
        locals.var_temp2_rv = 0.0;

        let assign41170_e54150: f64 = (-230.25850929940458);
        let assign41170_e54151: f64 = if locals.var_temp2 > assign41170_e54150 { 1.0 } else { 0.0 };
        locals.var_guard1191 = assign41170_e54151;
        locals.var_guard1191_rv = 0.0;

        let (assign41180_e54158, assign41180_e54158_d_n4, assign41180_e54158_d_n6, assign41180_e54158_d_n7, assign41180_e54158_d_n8, assign41180_e54158_d_n9,) = {
    if ((locals.var_guard1190 != 0.0) && (locals.var_guard1191 != 0.0)) {
        let assign41180_e54156: f64 = (locals.var_temp2).exp();
        (assign41180_e54156, (assign41180_e54156 * locals.var_temp2_dn4), (assign41180_e54156 * locals.var_temp2_dn6), (assign41180_e54156 * locals.var_temp2_dn7), (assign41180_e54156 * locals.var_temp2_dn8), (assign41180_e54156 * locals.var_temp2_dn9),)
    } else {
        (locals.var_dctg, locals.var_dctg_dn4, locals.var_dctg_dn6, locals.var_dctg_dn7, locals.var_dctg_dn8, locals.var_dctg_dn9,)
    }
};
        locals.var_dctg = assign41180_e54158;
        locals.var_dctg_dn4 = assign41180_e54158_d_n4;
        locals.var_dctg_dn6 = assign41180_e54158_d_n6;
        locals.var_dctg_dn7 = assign41180_e54158_d_n7;
        locals.var_dctg_dn8 = assign41180_e54158_d_n8;
        locals.var_dctg_dn9 = assign41180_e54158_d_n9;
        locals.var_dctg_rv = 0.0;

        let (assign41190_e54190, assign41190_e54190_d_n4, assign41190_e54190_d_n6, assign41190_e54190_d_n7, assign41190_e54190_d_n8, assign41190_e54190_d_n9,) = {
    if ((locals.var_guard1190 != 0.0) && (locals.var_guard1191 == 0.0)) {
        let assign41190_e54166: f64 = (-230.25850929940458);
        let assign41190_e54168: f64 = (assign41190_e54166 - locals.var_temp2);
        let assign41190_e54172: f64 = (-230.25850929940458);
        let assign41190_e54174: f64 = (assign41190_e54172 - locals.var_temp2);
        let assign41190_e54177: f64 = (-230.25850929940458);
        let assign41190_e54179: f64 = (assign41190_e54177 - locals.var_temp2);
        let assign41190_e54181: f64 = (assign41190_e54179 * 0.3333333333333333);
        let assign41190_e54182: f64 = (1.0 + assign41190_e54181);
        let assign41190_e54183: f64 = (assign41190_e54174 * assign41190_e54182);
        let assign41190_e54184: f64 = (0.5 * assign41190_e54183);
        let assign41190_e54185: f64 = (1.0 + assign41190_e54184);
        let assign41190_e54186: f64 = (assign41190_e54168 * assign41190_e54185);
        let assign41190_e54187: f64 = (1.0 + assign41190_e54186);
        let assign41190_e54188: f64 = (1e-100 / assign41190_e54187);
        (assign41190_e54188, (-((1e-100 * (((-locals.var_temp2_dn4) * assign41190_e54185) + (assign41190_e54168 * (0.5 * (((-locals.var_temp2_dn4) * assign41190_e54182) + (assign41190_e54174 * ((-locals.var_temp2_dn4) * 0.3333333333333333))))))) / (assign41190_e54187 * assign41190_e54187))), (-((1e-100 * (((-locals.var_temp2_dn6) * assign41190_e54185) + (assign41190_e54168 * (0.5 * (((-locals.var_temp2_dn6) * assign41190_e54182) + (assign41190_e54174 * ((-locals.var_temp2_dn6) * 0.3333333333333333))))))) / (assign41190_e54187 * assign41190_e54187))), (-((1e-100 * (((-locals.var_temp2_dn7) * assign41190_e54185) + (assign41190_e54168 * (0.5 * (((-locals.var_temp2_dn7) * assign41190_e54182) + (assign41190_e54174 * ((-locals.var_temp2_dn7) * 0.3333333333333333))))))) / (assign41190_e54187 * assign41190_e54187))), (-((1e-100 * (((-locals.var_temp2_dn8) * assign41190_e54185) + (assign41190_e54168 * (0.5 * (((-locals.var_temp2_dn8) * assign41190_e54182) + (assign41190_e54174 * ((-locals.var_temp2_dn8) * 0.3333333333333333))))))) / (assign41190_e54187 * assign41190_e54187))), (-((1e-100 * (((-locals.var_temp2_dn9) * assign41190_e54185) + (assign41190_e54168 * (0.5 * (((-locals.var_temp2_dn9) * assign41190_e54182) + (assign41190_e54174 * ((-locals.var_temp2_dn9) * 0.3333333333333333))))))) / (assign41190_e54187 * assign41190_e54187))),)
    } else {
        (locals.var_dctg, locals.var_dctg_dn4, locals.var_dctg_dn6, locals.var_dctg_dn7, locals.var_dctg_dn8, locals.var_dctg_dn9,)
    }
};
        locals.var_dctg = assign41190_e54190;
        locals.var_dctg_dn4 = assign41190_e54190_d_n4;
        locals.var_dctg_dn6 = assign41190_e54190_d_n6;
        locals.var_dctg_dn7 = assign41190_e54190_d_n7;
        locals.var_dctg_dn8 = assign41190_e54190_d_n8;
        locals.var_dctg_dn9 = assign41190_e54190_d_n9;
        locals.var_dctg_rv = 0.0;

        let assign41200_e54194: f64 = (locals.var_ct_t * locals.var_dctg);
        let assign41200_e54195: f64 = (1.0 + assign41200_e54194);
        locals.var_ct_fact = assign41200_e54195;
        locals.var_ct_fact_dn4 = ((locals.var_ct_t_dn4 * locals.var_dctg) + (locals.var_ct_t * locals.var_dctg_dn4));
        locals.var_ct_fact_dn6 = (locals.var_ct_t * locals.var_dctg_dn6);
        locals.var_ct_fact_dn7 = (locals.var_ct_t * locals.var_dctg_dn7);
        locals.var_ct_fact_dn8 = (locals.var_ct_t * locals.var_dctg_dn8);
        locals.var_ct_fact_dn9 = (locals.var_ct_t * locals.var_dctg_dn9);
        locals.var_ct_fact_rv = 0.0;

        let assign41210_e54198: f64 = (locals.var_phit * locals.var_ct_fact);
        locals.var_phitct = assign41210_e54198;
        locals.var_phitct_dn4 = ((locals.var_phit_dn4 * locals.var_ct_fact) + (locals.var_phit * locals.var_ct_fact_dn4));
        locals.var_phitct_dn6 = (locals.var_phit * locals.var_ct_fact_dn6);
        locals.var_phitct_dn7 = (locals.var_phit * locals.var_ct_fact_dn7);
        locals.var_phitct_dn8 = (locals.var_phit * locals.var_ct_fact_dn8);
        locals.var_phitct_dn9 = (locals.var_phit * locals.var_ct_fact_dn9);
        locals.var_phitct_rv = 0.0;

        let assign41220_e54203: f64 = (locals.var_psced_i * locals.var_vdsx);
        let assign41220_e54204: f64 = (1.0 + assign41220_e54203);
        let assign41220_e54205: f64 = (locals.var_psce_i * assign41220_e54204);
        let assign41220_e54209: f64 = (locals.var_psceb_i * locals.var_vsbx);
        let assign41220_e54210: f64 = (1.0 + assign41220_e54209);
        let assign41220_e54211: f64 = (assign41220_e54205 * assign41220_e54210);
        locals.var_dphit1 = assign41220_e54211;
        locals.var_dphit1_dn4 = (assign41220_e54205 * (locals.var_psceb_i * locals.var_vsbx_dn4));
        locals.var_dphit1_dn6 = (assign41220_e54205 * (locals.var_psceb_i * locals.var_vsbx_dn6));
        locals.var_dphit1_dn7 = (((locals.var_psce_i * (locals.var_psced_i * locals.var_vdsx_dn7)) * assign41220_e54210) + (assign41220_e54205 * (locals.var_psceb_i * locals.var_vsbx_dn7)));
        locals.var_dphit1_dn8 = (((locals.var_psce_i * (locals.var_psced_i * locals.var_vdsx_dn8)) * assign41220_e54210) + (assign41220_e54205 * (locals.var_psceb_i * locals.var_vsbx_dn8)));
        locals.var_dphit1_dn9 = (assign41220_e54205 * (locals.var_psceb_i * locals.var_vsbx_dn9));
        locals.var_dphit1_rv = 0.0;

        let assign41230_e54215: f64 = (1.0 + locals.var_dphit1);
        let assign41230_e54216: f64 = (locals.var_phitct * assign41230_e54215);
        locals.var_phit1 = assign41230_e54216;
        locals.var_phit1_dn4 = ((locals.var_phitct_dn4 * assign41230_e54215) + (locals.var_phitct * locals.var_dphit1_dn4));
        locals.var_phit1_dn6 = ((locals.var_phitct_dn6 * assign41230_e54215) + (locals.var_phitct * locals.var_dphit1_dn6));
        locals.var_phit1_dn7 = ((locals.var_phitct_dn7 * assign41230_e54215) + (locals.var_phitct * locals.var_dphit1_dn7));
        locals.var_phit1_dn8 = ((locals.var_phitct_dn8 * assign41230_e54215) + (locals.var_phitct * locals.var_dphit1_dn8));
        locals.var_phit1_dn9 = ((locals.var_phitct_dn9 * assign41230_e54215) + (locals.var_phitct * locals.var_dphit1_dn9));
        locals.var_phit1_rv = 0.0;

        let assign41240_e54219: f64 = (1.0 / locals.var_phit1);
        locals.var_inv_phit1 = assign41240_e54219;
        locals.var_inv_phit1_dn4 = (-(locals.var_phit1_dn4 / (locals.var_phit1 * locals.var_phit1)));
        locals.var_inv_phit1_dn6 = (-(locals.var_phit1_dn6 / (locals.var_phit1 * locals.var_phit1)));
        locals.var_inv_phit1_dn7 = (-(locals.var_phit1_dn7 / (locals.var_phit1 * locals.var_phit1)));
        locals.var_inv_phit1_dn8 = (-(locals.var_phit1_dn8 / (locals.var_phit1 * locals.var_phit1)));
        locals.var_inv_phit1_dn9 = (-(locals.var_phit1_dn9 / (locals.var_phit1 * locals.var_phit1)));
        locals.var_inv_phit1_rv = 0.0;

        let assign41250_e54223: f64 = (locals.var_phit * locals.var_inv_phit1);
        let assign41250_e54224: f64 = (assign41250_e54223).sqrt();
        let assign41250_e54225: f64 = (locals.var_g_0 * assign41250_e54224);
        locals.var_gf = assign41250_e54225;
        locals.var_gf_dn4 = ((locals.var_g_0_dn4 * assign41250_e54224) + (locals.var_g_0 * (((locals.var_phit_dn4 * locals.var_inv_phit1) + (locals.var_phit * locals.var_inv_phit1_dn4)) / (2.0 * assign41250_e54224))));
        locals.var_gf_dn6 = (locals.var_g_0 * ((locals.var_phit * locals.var_inv_phit1_dn6) / (2.0 * assign41250_e54224)));
        locals.var_gf_dn7 = (locals.var_g_0 * ((locals.var_phit * locals.var_inv_phit1_dn7) / (2.0 * assign41250_e54224)));
        locals.var_gf_dn8 = (locals.var_g_0 * ((locals.var_phit * locals.var_inv_phit1_dn8) / (2.0 * assign41250_e54224)));
        locals.var_gf_dn9 = (locals.var_g_0 * ((locals.var_phit * locals.var_inv_phit1_dn9) / (2.0 * assign41250_e54224)));
        locals.var_gf_rv = 0.0;

        let assign41260_e54228: f64 = (locals.var_gf * locals.var_gf);
        locals.var_gf2 = assign41260_e54228;
        locals.var_gf2_dn4 = ((locals.var_gf_dn4 * locals.var_gf) + (locals.var_gf * locals.var_gf_dn4));
        locals.var_gf2_dn6 = ((locals.var_gf_dn6 * locals.var_gf) + (locals.var_gf * locals.var_gf_dn6));
        locals.var_gf2_dn7 = ((locals.var_gf_dn7 * locals.var_gf) + (locals.var_gf * locals.var_gf_dn7));
        locals.var_gf2_dn8 = ((locals.var_gf_dn8 * locals.var_gf) + (locals.var_gf * locals.var_gf_dn8));
        locals.var_gf2_dn9 = ((locals.var_gf_dn9 * locals.var_gf) + (locals.var_gf * locals.var_gf_dn9));
        locals.var_gf2_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_17(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign41270_e54231: f64 = (1.0 / locals.var_gf2);
        locals.var_inv_gf2 = assign41270_e54231;
        locals.var_inv_gf2_dn4 = (-(locals.var_gf2_dn4 / (locals.var_gf2 * locals.var_gf2)));
        locals.var_inv_gf2_dn6 = (-(locals.var_gf2_dn6 / (locals.var_gf2 * locals.var_gf2)));
        locals.var_inv_gf2_dn7 = (-(locals.var_gf2_dn7 / (locals.var_gf2 * locals.var_gf2)));
        locals.var_inv_gf2_dn8 = (-(locals.var_gf2_dn8 / (locals.var_gf2 * locals.var_gf2)));
        locals.var_inv_gf2_dn9 = (-(locals.var_gf2_dn9 / (locals.var_gf2 * locals.var_gf2)));
        locals.var_inv_gf2_rv = 0.0;

        let assign41280_e54234: f64 = (locals.var_vsbstar * locals.var_inv_phit1);
        locals.var_ux = assign41280_e54234;
        locals.var_ux_dn4 = ((locals.var_vsbstar_dn4 * locals.var_inv_phit1) + (locals.var_vsbstar * locals.var_inv_phit1_dn4));
        locals.var_ux_dn6 = ((locals.var_vsbstar_dn6 * locals.var_inv_phit1) + (locals.var_vsbstar * locals.var_inv_phit1_dn6));
        locals.var_ux_dn7 = ((locals.var_vsbstar_dn7 * locals.var_inv_phit1) + (locals.var_vsbstar * locals.var_inv_phit1_dn7));
        locals.var_ux_dn8 = ((locals.var_vsbstar_dn8 * locals.var_inv_phit1) + (locals.var_vsbstar * locals.var_inv_phit1_dn8));
        locals.var_ux_dn9 = ((locals.var_vsbstar_dn9 * locals.var_inv_phit1) + (locals.var_vsbstar * locals.var_inv_phit1_dn9));
        locals.var_ux_rv = 0.0;

        let assign41290_e54237: f64 = (locals.var_vgb1 * locals.var_inv_phit1);
        locals.var_xg = assign41290_e54237;
        locals.var_xg_dn4 = ((locals.var_vgb1_dn4 * locals.var_inv_phit1) + (locals.var_vgb1 * locals.var_inv_phit1_dn4));
        locals.var_xg_dn6 = ((locals.var_vgb1_dn6 * locals.var_inv_phit1) + (locals.var_vgb1 * locals.var_inv_phit1_dn6));
        locals.var_xg_dn7 = ((locals.var_vgb1_dn7 * locals.var_inv_phit1) + (locals.var_vgb1 * locals.var_inv_phit1_dn7));
        locals.var_xg_dn8 = ((locals.var_vgb1_dn8 * locals.var_inv_phit1) + (locals.var_vgb1 * locals.var_inv_phit1_dn8));
        locals.var_xg_dn9 = ((locals.var_vgb1_dn9 * locals.var_inv_phit1) + (locals.var_vgb1 * locals.var_inv_phit1_dn9));
        locals.var_xg_rv = 0.0;

        let assign41300_e54240: f64 = (2.0 * locals.var_vdsx);
        let assign41300_e54245: f64 = (locals.var_cfd_i * locals.var_vdsx);
        let assign41300_e54246: f64 = (1.0 + assign41300_e54245);
        let assign41300_e54247: f64 = (assign41300_e54246).sqrt();
        let assign41300_e54248: f64 = (1.0 + assign41300_e54247);
        let assign41300_e54249: f64 = (assign41300_e54240 / assign41300_e54248);
        locals.var_vdsp = assign41300_e54249;
        locals.var_vdsp_dn7 = ((((2.0 * locals.var_vdsx_dn7) * assign41300_e54248) - (assign41300_e54240 * ((locals.var_cfd_i * locals.var_vdsx_dn7) / (2.0 * assign41300_e54247)))) / (assign41300_e54248 * assign41300_e54248));
        locals.var_vdsp_dn8 = ((((2.0 * locals.var_vdsx_dn8) * assign41300_e54248) - (assign41300_e54240 * ((locals.var_cfd_i * locals.var_vdsx_dn8) / (2.0 * assign41300_e54247)))) / (assign41300_e54248 * assign41300_e54248));
        locals.var_vdsp_rv = 0.0;

        let assign41310_e54252: f64 = (locals.var_cf_i * locals.var_vdsp);
        let assign41310_e54256: f64 = (locals.var_cfb_i * locals.var_vsbx);
        let assign41310_e54257: f64 = (1.0 + assign41310_e54256);
        let assign41310_e54258: f64 = (assign41310_e54252 * assign41310_e54257);
        locals.var_delphib = assign41310_e54258;
        locals.var_delphib_dn4 = (assign41310_e54252 * (locals.var_cfb_i * locals.var_vsbx_dn4));
        locals.var_delphib_dn6 = (assign41310_e54252 * (locals.var_cfb_i * locals.var_vsbx_dn6));
        locals.var_delphib_dn7 = (((locals.var_cf_i * locals.var_vdsp_dn7) * assign41310_e54257) + (assign41310_e54252 * (locals.var_cfb_i * locals.var_vsbx_dn7)));
        locals.var_delphib_dn8 = (((locals.var_cf_i * locals.var_vdsp_dn8) * assign41310_e54257) + (assign41310_e54252 * (locals.var_cfb_i * locals.var_vsbx_dn8)));
        locals.var_delphib_dn9 = (assign41310_e54252 * (locals.var_cfb_i * locals.var_vsbx_dn9));
        locals.var_delphib_rv = 0.0;

        let assign41320_e54261: f64 = (locals.var_phib * locals.var_inv_phit1);
        locals.var_xb = assign41320_e54261;
        locals.var_xb_dn4 = ((locals.var_phib_dn4 * locals.var_inv_phit1) + (locals.var_phib * locals.var_inv_phit1_dn4));
        locals.var_xb_dn6 = (locals.var_phib * locals.var_inv_phit1_dn6);
        locals.var_xb_dn7 = (locals.var_phib * locals.var_inv_phit1_dn7);
        locals.var_xb_dn8 = (locals.var_phib * locals.var_inv_phit1_dn8);
        locals.var_xb_dn9 = (locals.var_phib * locals.var_inv_phit1_dn9);
        locals.var_xb_rv = 0.0;

        let assign41330_e54264: f64 = (locals.var_v_xb * locals.var_v_xb);
        let assign41330_e54266: f64 = (assign41330_e54264 + locals.var_aphi);
        let assign41330_e54267: f64 = (assign41330_e54266).sqrt();
        locals.var_temp1 = assign41330_e54267;
        locals.var_temp1_dn4 = ((((locals.var_v_xb_dn4 * locals.var_v_xb) + (locals.var_v_xb * locals.var_v_xb_dn4)) + locals.var_aphi_dn4) / (2.0 * assign41330_e54267));
        locals.var_temp1_dn6 = 0.0;
        locals.var_temp1_dn7 = (((locals.var_v_xb_dn7 * locals.var_v_xb) + (locals.var_v_xb * locals.var_v_xb_dn7)) / (2.0 * assign41330_e54267));
        locals.var_temp1_dn8 = (((locals.var_v_xb_dn8 * locals.var_v_xb) + (locals.var_v_xb * locals.var_v_xb_dn8)) / (2.0 * assign41330_e54267));
        locals.var_temp1_dn9 = (((locals.var_v_xb_dn9 * locals.var_v_xb) + (locals.var_v_xb * locals.var_v_xb_dn9)) / (2.0 * assign41330_e54267));
        locals.var_temp1_rv = 0.0;

        let assign41340_e54270: f64 = (locals.var_v_xb - locals.var_delphib);
        let assign41340_e54273: f64 = (locals.var_v_xb - locals.var_delphib);
        let assign41340_e54274: f64 = (assign41340_e54270 * assign41340_e54273);
        let assign41340_e54276: f64 = (assign41340_e54274 + locals.var_aphi);
        let assign41340_e54277: f64 = (assign41340_e54276).sqrt();
        locals.var_temp2 = assign41340_e54277;
        locals.var_temp2_dn4 = (((((locals.var_v_xb_dn4 - locals.var_delphib_dn4) * assign41340_e54273) + (assign41340_e54270 * (locals.var_v_xb_dn4 - locals.var_delphib_dn4))) + locals.var_aphi_dn4) / (2.0 * assign41340_e54277));
        locals.var_temp2_dn6 = ((((-locals.var_delphib_dn6) * assign41340_e54273) + (assign41340_e54270 * (-locals.var_delphib_dn6))) / (2.0 * assign41340_e54277));
        locals.var_temp2_dn7 = ((((locals.var_v_xb_dn7 - locals.var_delphib_dn7) * assign41340_e54273) + (assign41340_e54270 * (locals.var_v_xb_dn7 - locals.var_delphib_dn7))) / (2.0 * assign41340_e54277));
        locals.var_temp2_dn8 = ((((locals.var_v_xb_dn8 - locals.var_delphib_dn8) * assign41340_e54273) + (assign41340_e54270 * (locals.var_v_xb_dn8 - locals.var_delphib_dn8))) / (2.0 * assign41340_e54277));
        locals.var_temp2_dn9 = ((((locals.var_v_xb_dn9 - locals.var_delphib_dn9) * assign41340_e54273) + (assign41340_e54270 * (locals.var_v_xb_dn9 - locals.var_delphib_dn9))) / (2.0 * assign41340_e54277));
        locals.var_temp2_rv = 0.0;

        let assign41350_e54280: f64 = (0.5 * locals.var_inv_phit1);
        let assign41350_e54283: f64 = (locals.var_delphib + locals.var_temp1);
        let assign41350_e54285: f64 = (assign41350_e54283 - locals.var_temp2);
        let assign41350_e54286: f64 = (assign41350_e54280 * assign41350_e54285);
        locals.var_delxb = assign41350_e54286;
        locals.var_delxb_dn4 = (((0.5 * locals.var_inv_phit1_dn4) * assign41350_e54285) + (assign41350_e54280 * ((locals.var_delphib_dn4 + locals.var_temp1_dn4) - locals.var_temp2_dn4)));
        locals.var_delxb_dn6 = (((0.5 * locals.var_inv_phit1_dn6) * assign41350_e54285) + (assign41350_e54280 * ((locals.var_delphib_dn6 + locals.var_temp1_dn6) - locals.var_temp2_dn6)));
        locals.var_delxb_dn7 = (((0.5 * locals.var_inv_phit1_dn7) * assign41350_e54285) + (assign41350_e54280 * ((locals.var_delphib_dn7 + locals.var_temp1_dn7) - locals.var_temp2_dn7)));
        locals.var_delxb_dn8 = (((0.5 * locals.var_inv_phit1_dn8) * assign41350_e54285) + (assign41350_e54280 * ((locals.var_delphib_dn8 + locals.var_temp1_dn8) - locals.var_temp2_dn8)));
        locals.var_delxb_dn9 = (((0.5 * locals.var_inv_phit1_dn9) * assign41350_e54285) + (assign41350_e54280 * ((locals.var_delphib_dn9 + locals.var_temp1_dn9) - locals.var_temp2_dn9)));
        locals.var_delxb_rv = 0.0;

        let assign41360_e54289: f64 = (locals.var_xb + locals.var_ux);
        locals.var_xno_s = assign41360_e54289;
        locals.var_xno_s_dn4 = (locals.var_xb_dn4 + locals.var_ux_dn4);
        locals.var_xno_s_dn6 = (locals.var_xb_dn6 + locals.var_ux_dn6);
        locals.var_xno_s_dn7 = (locals.var_xb_dn7 + locals.var_ux_dn7);
        locals.var_xno_s_dn8 = (locals.var_xb_dn8 + locals.var_ux_dn8);
        locals.var_xno_s_dn9 = (locals.var_xb_dn9 + locals.var_ux_dn9);
        locals.var_xno_s_rv = 0.0;

        let assign41370_e54292: f64 = (locals.var_xno_s - locals.var_delxb);
        locals.var_xn_s = assign41370_e54292;
        locals.var_xn_s_dn4 = (locals.var_xno_s_dn4 - locals.var_delxb_dn4);
        locals.var_xn_s_dn6 = (locals.var_xno_s_dn6 - locals.var_delxb_dn6);
        locals.var_xn_s_dn7 = (locals.var_xno_s_dn7 - locals.var_delxb_dn7);
        locals.var_xn_s_dn8 = (locals.var_xno_s_dn8 - locals.var_delxb_dn8);
        locals.var_xn_s_dn9 = (locals.var_xno_s_dn9 - locals.var_delxb_dn9);
        locals.var_xn_s_rv = 0.0;

        let assign41380_e54295: f64 = if p.p45 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1192 = assign41380_e54295;
        locals.var_guard1192_rv = 0.0;

        let assign41390_e54297: f64 = (locals.var_xn_s).abs();
        let assign41390_e54299: f64 = if assign41390_e54297 < 1e-5 { 1.0 } else { 0.0 };
        locals.var_guard1193 = assign41390_e54299;
        locals.var_guard1193_rv = 0.0;

        let (assign41400_e54319, assign41400_e54319_d_n4, assign41400_e54319_d_n6, assign41400_e54319_d_n7, assign41400_e54319_d_n8, assign41400_e54319_d_n9,) = {
    if ((locals.var_guard1192 != 0.0) && (locals.var_guard1193 != 0.0)) {
        let assign41400_e54308: f64 = (0.5 * locals.var_xn_s);
        let assign41400_e54312: f64 = (0.3125 * locals.var_xn_s);
        let assign41400_e54313: f64 = (1.0 - assign41400_e54312);
        let assign41400_e54314: f64 = (assign41400_e54308 * assign41400_e54313);
        let assign41400_e54315: f64 = (1.0 - assign41400_e54314);
        let assign41400_e54316: f64 = (locals.var_gf * assign41400_e54315);
        let assign41400_e54317: f64 = (1.0 + assign41400_e54316);
        (assign41400_e54317, ((locals.var_gf_dn4 * assign41400_e54315) + (locals.var_gf * (-(((0.5 * locals.var_xn_s_dn4) * assign41400_e54313) + (assign41400_e54308 * (-(0.3125 * locals.var_xn_s_dn4))))))), ((locals.var_gf_dn6 * assign41400_e54315) + (locals.var_gf * (-(((0.5 * locals.var_xn_s_dn6) * assign41400_e54313) + (assign41400_e54308 * (-(0.3125 * locals.var_xn_s_dn6))))))), ((locals.var_gf_dn7 * assign41400_e54315) + (locals.var_gf * (-(((0.5 * locals.var_xn_s_dn7) * assign41400_e54313) + (assign41400_e54308 * (-(0.3125 * locals.var_xn_s_dn7))))))), ((locals.var_gf_dn8 * assign41400_e54315) + (locals.var_gf * (-(((0.5 * locals.var_xn_s_dn8) * assign41400_e54313) + (assign41400_e54308 * (-(0.3125 * locals.var_xn_s_dn8))))))), ((locals.var_gf_dn9 * assign41400_e54315) + (locals.var_gf * (-(((0.5 * locals.var_xn_s_dn9) * assign41400_e54313) + (assign41400_e54308 * (-(0.3125 * locals.var_xn_s_dn9))))))),)
    } else {
        (locals.var_nscr, locals.var_nscr_dn4, locals.var_nscr_dn6, locals.var_nscr_dn7, locals.var_nscr_dn8, locals.var_nscr_dn9,)
    }
};
        locals.var_nscr = assign41400_e54319;
        locals.var_nscr_dn4 = assign41400_e54319_d_n4;
        locals.var_nscr_dn6 = assign41400_e54319_d_n6;
        locals.var_nscr_dn7 = assign41400_e54319_d_n7;
        locals.var_nscr_dn8 = assign41400_e54319_d_n8;
        locals.var_nscr_dn9 = assign41400_e54319_d_n9;
        locals.var_nscr_rv = 0.0;

        let assign41410_e54322: f64 = if locals.var_xn_s < 460.51701859880916 { 1.0 } else { 0.0 };
        locals.var_guard1194 = assign41410_e54322;
        locals.var_guard1194_rv = 0.0;

        let (assign41420_e54333, assign41420_e54333_d_n4, assign41420_e54333_d_n6, assign41420_e54333_d_n7, assign41420_e54333_d_n8, assign41420_e54333_d_n9,) = {
    if (((locals.var_guard1192 != 0.0) && (locals.var_guard1193 == 0.0)) && (locals.var_guard1194 != 0.0)) {
        let assign41420_e54330: f64 = (-locals.var_xn_s);
        let assign41420_e54331: f64 = (assign41420_e54330).exp();
        (assign41420_e54331, (assign41420_e54331 * (-locals.var_xn_s_dn4)), (assign41420_e54331 * (-locals.var_xn_s_dn6)), (assign41420_e54331 * (-locals.var_xn_s_dn7)), (assign41420_e54331 * (-locals.var_xn_s_dn8)), (assign41420_e54331 * (-locals.var_xn_s_dn9)),)
    } else {
        (locals.var_delta_ns, locals.var_delta_ns_dn4, locals.var_delta_ns_dn6, locals.var_delta_ns_dn7, locals.var_delta_ns_dn8, locals.var_delta_ns_dn9,)
    }
};
        locals.var_delta_ns = assign41420_e54333;
        locals.var_delta_ns_dn4 = assign41420_e54333_d_n4;
        locals.var_delta_ns_dn6 = assign41420_e54333_d_n6;
        locals.var_delta_ns_dn7 = assign41420_e54333_d_n7;
        locals.var_delta_ns_dn8 = assign41420_e54333_d_n8;
        locals.var_delta_ns_dn9 = assign41420_e54333_d_n9;
        locals.var_delta_ns_rv = 0.0;

        let (assign41430_e54365, assign41430_e54365_d_n4, assign41430_e54365_d_n6, assign41430_e54365_d_n7, assign41430_e54365_d_n8, assign41430_e54365_d_n9,) = {
    if (((locals.var_guard1192 != 0.0) && (locals.var_guard1193 == 0.0)) && (locals.var_guard1194 == 0.0)) {
        let assign41430_e54345: f64 = (locals.var_xn_s - 460.51701859880916);
        let assign41430_e54350: f64 = (locals.var_xn_s - 460.51701859880916);
        let assign41430_e54354: f64 = (locals.var_xn_s - 460.51701859880916);
        let assign41430_e54356: f64 = (assign41430_e54354 * 0.3333333333333333);
        let assign41430_e54357: f64 = (1.0 + assign41430_e54356);
        let assign41430_e54358: f64 = (assign41430_e54350 * assign41430_e54357);
        let assign41430_e54359: f64 = (0.5 * assign41430_e54358);
        let assign41430_e54360: f64 = (1.0 + assign41430_e54359);
        let assign41430_e54361: f64 = (assign41430_e54345 * assign41430_e54360);
        let assign41430_e54362: f64 = (1.0 + assign41430_e54361);
        let assign41430_e54363: f64 = (1e-200 / assign41430_e54362);
        (assign41430_e54363, (-((1e-200 * ((locals.var_xn_s_dn4 * assign41430_e54360) + (assign41430_e54345 * (0.5 * ((locals.var_xn_s_dn4 * assign41430_e54357) + (assign41430_e54350 * (locals.var_xn_s_dn4 * 0.3333333333333333))))))) / (assign41430_e54362 * assign41430_e54362))), (-((1e-200 * ((locals.var_xn_s_dn6 * assign41430_e54360) + (assign41430_e54345 * (0.5 * ((locals.var_xn_s_dn6 * assign41430_e54357) + (assign41430_e54350 * (locals.var_xn_s_dn6 * 0.3333333333333333))))))) / (assign41430_e54362 * assign41430_e54362))), (-((1e-200 * ((locals.var_xn_s_dn7 * assign41430_e54360) + (assign41430_e54345 * (0.5 * ((locals.var_xn_s_dn7 * assign41430_e54357) + (assign41430_e54350 * (locals.var_xn_s_dn7 * 0.3333333333333333))))))) / (assign41430_e54362 * assign41430_e54362))), (-((1e-200 * ((locals.var_xn_s_dn8 * assign41430_e54360) + (assign41430_e54345 * (0.5 * ((locals.var_xn_s_dn8 * assign41430_e54357) + (assign41430_e54350 * (locals.var_xn_s_dn8 * 0.3333333333333333))))))) / (assign41430_e54362 * assign41430_e54362))), (-((1e-200 * ((locals.var_xn_s_dn9 * assign41430_e54360) + (assign41430_e54345 * (0.5 * ((locals.var_xn_s_dn9 * assign41430_e54357) + (assign41430_e54350 * (locals.var_xn_s_dn9 * 0.3333333333333333))))))) / (assign41430_e54362 * assign41430_e54362))),)
    } else {
        (locals.var_delta_ns, locals.var_delta_ns_dn4, locals.var_delta_ns_dn6, locals.var_delta_ns_dn7, locals.var_delta_ns_dn8, locals.var_delta_ns_dn9,)
    }
};
        locals.var_delta_ns = assign41430_e54365;
        locals.var_delta_ns_dn4 = assign41430_e54365_d_n4;
        locals.var_delta_ns_dn6 = assign41430_e54365_d_n6;
        locals.var_delta_ns_dn7 = assign41430_e54365_d_n7;
        locals.var_delta_ns_dn8 = assign41430_e54365_d_n8;
        locals.var_delta_ns_dn9 = assign41430_e54365_d_n9;
        locals.var_delta_ns_rv = 0.0;

        let (assign41440_e54378, assign41440_e54378_d_n4, assign41440_e54378_d_n6, assign41440_e54378_d_n7, assign41440_e54378_d_n8, assign41440_e54378_d_n9,) = {
    if ((locals.var_guard1192 != 0.0) && (locals.var_guard1193 == 0.0)) {
        let (assign41440_e54376,) = {
            if (locals.var_xn_s > 0.0) {
                (1.0,)
            } else {
                let assign41440_e54375: f64 = (-1.0);
                (assign41440_e54375,)
            }
        };
        (assign41440_e54376, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
    }
};
        locals.var_temp__blk949 = assign41440_e54378;
        locals.var_temp__blk949_dn4 = assign41440_e54378_d_n4;
        locals.var_temp__blk949_dn6 = assign41440_e54378_d_n6;
        locals.var_temp__blk949_dn7 = assign41440_e54378_d_n7;
        locals.var_temp__blk949_dn8 = assign41440_e54378_d_n8;
        locals.var_temp__blk949_dn9 = assign41440_e54378_d_n9;
        locals.var_temp__blk949_rv = 0.0;

        let (assign41450_e54406, assign41450_e54406_d_n4, assign41450_e54406_d_n6, assign41450_e54406_d_n7, assign41450_e54406_d_n8, assign41450_e54406_d_n9,) = {
    if ((locals.var_guard1192 != 0.0) && (locals.var_guard1193 == 0.0)) {
        let assign41450_e54386: f64 = (locals.var_temp__blk949 * locals.var_gf);
        let assign41450_e54391: f64 = (1.0 - locals.var_xn_s);
        let assign41450_e54392: f64 = (locals.var_delta_ns * assign41450_e54391);
        let assign41450_e54393: f64 = (1.0 - assign41450_e54392);
        let assign41450_e54394: f64 = (assign41450_e54386 * assign41450_e54393);
        let assign41450_e54399: f64 = (1.0 - locals.var_delta_ns);
        let assign41450_e54400: f64 = (locals.var_xn_s * assign41450_e54399);
        let assign41450_e54401: f64 = (assign41450_e54400).sqrt();
        let assign41450_e54402: f64 = (2.0 * assign41450_e54401);
        let assign41450_e54403: f64 = (assign41450_e54394 / assign41450_e54402);
        let assign41450_e54404: f64 = (1.0 + assign41450_e54403);
        (assign41450_e54404, (((((((locals.var_temp__blk949_dn4 * locals.var_gf) + (locals.var_temp__blk949 * locals.var_gf_dn4)) * assign41450_e54393) + (assign41450_e54386 * (-((locals.var_delta_ns_dn4 * assign41450_e54391) + (locals.var_delta_ns * (-locals.var_xn_s_dn4)))))) * assign41450_e54402) - (assign41450_e54394 * (2.0 * (((locals.var_xn_s_dn4 * assign41450_e54399) + (locals.var_xn_s * (-locals.var_delta_ns_dn4))) / (2.0 * assign41450_e54401))))) / (assign41450_e54402 * assign41450_e54402)), (((((((locals.var_temp__blk949_dn6 * locals.var_gf) + (locals.var_temp__blk949 * locals.var_gf_dn6)) * assign41450_e54393) + (assign41450_e54386 * (-((locals.var_delta_ns_dn6 * assign41450_e54391) + (locals.var_delta_ns * (-locals.var_xn_s_dn6)))))) * assign41450_e54402) - (assign41450_e54394 * (2.0 * (((locals.var_xn_s_dn6 * assign41450_e54399) + (locals.var_xn_s * (-locals.var_delta_ns_dn6))) / (2.0 * assign41450_e54401))))) / (assign41450_e54402 * assign41450_e54402)), (((((((locals.var_temp__blk949_dn7 * locals.var_gf) + (locals.var_temp__blk949 * locals.var_gf_dn7)) * assign41450_e54393) + (assign41450_e54386 * (-((locals.var_delta_ns_dn7 * assign41450_e54391) + (locals.var_delta_ns * (-locals.var_xn_s_dn7)))))) * assign41450_e54402) - (assign41450_e54394 * (2.0 * (((locals.var_xn_s_dn7 * assign41450_e54399) + (locals.var_xn_s * (-locals.var_delta_ns_dn7))) / (2.0 * assign41450_e54401))))) / (assign41450_e54402 * assign41450_e54402)), (((((((locals.var_temp__blk949_dn8 * locals.var_gf) + (locals.var_temp__blk949 * locals.var_gf_dn8)) * assign41450_e54393) + (assign41450_e54386 * (-((locals.var_delta_ns_dn8 * assign41450_e54391) + (locals.var_delta_ns * (-locals.var_xn_s_dn8)))))) * assign41450_e54402) - (assign41450_e54394 * (2.0 * (((locals.var_xn_s_dn8 * assign41450_e54399) + (locals.var_xn_s * (-locals.var_delta_ns_dn8))) / (2.0 * assign41450_e54401))))) / (assign41450_e54402 * assign41450_e54402)), (((((((locals.var_temp__blk949_dn9 * locals.var_gf) + (locals.var_temp__blk949 * locals.var_gf_dn9)) * assign41450_e54393) + (assign41450_e54386 * (-((locals.var_delta_ns_dn9 * assign41450_e54391) + (locals.var_delta_ns * (-locals.var_xn_s_dn9)))))) * assign41450_e54402) - (assign41450_e54394 * (2.0 * (((locals.var_xn_s_dn9 * assign41450_e54399) + (locals.var_xn_s * (-locals.var_delta_ns_dn9))) / (2.0 * assign41450_e54401))))) / (assign41450_e54402 * assign41450_e54402)),)
    } else {
        (locals.var_nscr, locals.var_nscr_dn4, locals.var_nscr_dn6, locals.var_nscr_dn7, locals.var_nscr_dn8, locals.var_nscr_dn9,)
    }
};
        locals.var_nscr = assign41450_e54406;
        locals.var_nscr_dn4 = assign41450_e54406_d_n4;
        locals.var_nscr_dn6 = assign41450_e54406_d_n6;
        locals.var_nscr_dn7 = assign41450_e54406_d_n7;
        locals.var_nscr_dn8 = assign41450_e54406_d_n8;
        locals.var_nscr_dn9 = assign41450_e54406_d_n9;
        locals.var_nscr_rv = 0.0;

        let (assign41460_e54418, assign41460_e54418_d_n4, assign41460_e54418_d_n6, assign41460_e54418_d_n7, assign41460_e54418_d_n8, assign41460_e54418_d_n9,) = {
    if (locals.var_guard1192 == 0.0) {
        let assign41460_e54412: f64 = (0.5 * locals.var_gf);
        let assign41460_e54414: f64 = (locals.var_xn_s).sqrt();
        let assign41460_e54415: f64 = (assign41460_e54412 / assign41460_e54414);
        let assign41460_e54416: f64 = (1.0 + assign41460_e54415);
        (assign41460_e54416, ((((0.5 * locals.var_gf_dn4) * assign41460_e54414) - (assign41460_e54412 * (locals.var_xn_s_dn4 / (2.0 * assign41460_e54414)))) / (assign41460_e54414 * assign41460_e54414)), ((((0.5 * locals.var_gf_dn6) * assign41460_e54414) - (assign41460_e54412 * (locals.var_xn_s_dn6 / (2.0 * assign41460_e54414)))) / (assign41460_e54414 * assign41460_e54414)), ((((0.5 * locals.var_gf_dn7) * assign41460_e54414) - (assign41460_e54412 * (locals.var_xn_s_dn7 / (2.0 * assign41460_e54414)))) / (assign41460_e54414 * assign41460_e54414)), ((((0.5 * locals.var_gf_dn8) * assign41460_e54414) - (assign41460_e54412 * (locals.var_xn_s_dn8 / (2.0 * assign41460_e54414)))) / (assign41460_e54414 * assign41460_e54414)), ((((0.5 * locals.var_gf_dn9) * assign41460_e54414) - (assign41460_e54412 * (locals.var_xn_s_dn9 / (2.0 * assign41460_e54414)))) / (assign41460_e54414 * assign41460_e54414)),)
    } else {
        (locals.var_nscr, locals.var_nscr_dn4, locals.var_nscr_dn6, locals.var_nscr_dn7, locals.var_nscr_dn8, locals.var_nscr_dn9,)
    }
};
        locals.var_nscr = assign41460_e54418;
        locals.var_nscr_dn4 = assign41460_e54418_d_n4;
        locals.var_nscr_dn6 = assign41460_e54418_d_n6;
        locals.var_nscr_dn7 = assign41460_e54418_d_n7;
        locals.var_nscr_dn8 = assign41460_e54418_d_n8;
        locals.var_nscr_dn9 = assign41460_e54418_d_n9;
        locals.var_nscr_rv = 0.0;

        let assign41470_e54422: f64 = (locals.var_xn_s).sqrt();
        let assign41470_e54423: f64 = (locals.var_gf * assign41470_e54422);
        let assign41470_e54424: f64 = (locals.var_xn_s + assign41470_e54423);
        let assign41470_e54428: f64 = (locals.var_nscr - 1.0);
        let assign41470_e54429: f64 = (assign41470_e54428).ln();
        let assign41470_e54430: f64 = (locals.var_nscr * assign41470_e54429);
        let assign41470_e54431: f64 = (assign41470_e54424 - assign41470_e54430);
        locals.var_xthscr = assign41470_e54431;
        locals.var_xthscr_dn4 = ((locals.var_xn_s_dn4 + ((locals.var_gf_dn4 * assign41470_e54422) + (locals.var_gf * (locals.var_xn_s_dn4 / (2.0 * assign41470_e54422))))) - ((locals.var_nscr_dn4 * assign41470_e54429) + (locals.var_nscr * (locals.var_nscr_dn4 / assign41470_e54428))));
        locals.var_xthscr_dn6 = ((locals.var_xn_s_dn6 + ((locals.var_gf_dn6 * assign41470_e54422) + (locals.var_gf * (locals.var_xn_s_dn6 / (2.0 * assign41470_e54422))))) - ((locals.var_nscr_dn6 * assign41470_e54429) + (locals.var_nscr * (locals.var_nscr_dn6 / assign41470_e54428))));
        locals.var_xthscr_dn7 = ((locals.var_xn_s_dn7 + ((locals.var_gf_dn7 * assign41470_e54422) + (locals.var_gf * (locals.var_xn_s_dn7 / (2.0 * assign41470_e54422))))) - ((locals.var_nscr_dn7 * assign41470_e54429) + (locals.var_nscr * (locals.var_nscr_dn7 / assign41470_e54428))));
        locals.var_xthscr_dn8 = ((locals.var_xn_s_dn8 + ((locals.var_gf_dn8 * assign41470_e54422) + (locals.var_gf * (locals.var_xn_s_dn8 / (2.0 * assign41470_e54422))))) - ((locals.var_nscr_dn8 * assign41470_e54429) + (locals.var_nscr * (locals.var_nscr_dn8 / assign41470_e54428))));
        locals.var_xthscr_dn9 = ((locals.var_xn_s_dn9 + ((locals.var_gf_dn9 * assign41470_e54422) + (locals.var_gf * (locals.var_xn_s_dn9 / (2.0 * assign41470_e54422))))) - ((locals.var_nscr_dn9 * assign41470_e54429) + (locals.var_nscr * (locals.var_nscr_dn9 / assign41470_e54428))));
        locals.var_xthscr_rv = 0.0;

        let assign41480_e54434: f64 = (locals.var_xg - locals.var_xthscr);
        let assign41480_e54436: f64 = (assign41480_e54434 / locals.var_nscr);
        locals.var_xgtscr = assign41480_e54436;
        locals.var_xgtscr_dn4 = ((((locals.var_xg_dn4 - locals.var_xthscr_dn4) * locals.var_nscr) - (assign41480_e54434 * locals.var_nscr_dn4)) / (locals.var_nscr * locals.var_nscr));
        locals.var_xgtscr_dn6 = ((((locals.var_xg_dn6 - locals.var_xthscr_dn6) * locals.var_nscr) - (assign41480_e54434 * locals.var_nscr_dn6)) / (locals.var_nscr * locals.var_nscr));
        locals.var_xgtscr_dn7 = ((((locals.var_xg_dn7 - locals.var_xthscr_dn7) * locals.var_nscr) - (assign41480_e54434 * locals.var_nscr_dn7)) / (locals.var_nscr * locals.var_nscr));
        locals.var_xgtscr_dn8 = ((((locals.var_xg_dn8 - locals.var_xthscr_dn8) * locals.var_nscr) - (assign41480_e54434 * locals.var_nscr_dn8)) / (locals.var_nscr * locals.var_nscr));
        locals.var_xgtscr_dn9 = ((((locals.var_xg_dn9 - locals.var_xthscr_dn9) * locals.var_nscr) - (assign41480_e54434 * locals.var_nscr_dn9)) / (locals.var_nscr * locals.var_nscr));
        locals.var_xgtscr_rv = 0.0;

        let assign41490_e54439: f64 = (0.5 * locals.var_gf2);
        let assign41490_e54443: f64 = (8.0 / locals.var_gf2);
        let assign41490_e54444: f64 = (1.0 + assign41490_e54443);
        let assign41490_e54445: f64 = (assign41490_e54444).sqrt();
        let assign41490_e54447: f64 = (assign41490_e54445 - 1.0);
        let assign41490_e54448: f64 = (assign41490_e54439 * assign41490_e54447);
        locals.var_qbscr = assign41490_e54448;
        locals.var_qbscr_dn4 = (((0.5 * locals.var_gf2_dn4) * assign41490_e54447) + (assign41490_e54439 * ((-((8.0 * locals.var_gf2_dn4) / (locals.var_gf2 * locals.var_gf2))) / (2.0 * assign41490_e54445))));
        locals.var_qbscr_dn6 = (((0.5 * locals.var_gf2_dn6) * assign41490_e54447) + (assign41490_e54439 * ((-((8.0 * locals.var_gf2_dn6) / (locals.var_gf2 * locals.var_gf2))) / (2.0 * assign41490_e54445))));
        locals.var_qbscr_dn7 = (((0.5 * locals.var_gf2_dn7) * assign41490_e54447) + (assign41490_e54439 * ((-((8.0 * locals.var_gf2_dn7) / (locals.var_gf2 * locals.var_gf2))) / (2.0 * assign41490_e54445))));
        locals.var_qbscr_dn8 = (((0.5 * locals.var_gf2_dn8) * assign41490_e54447) + (assign41490_e54439 * ((-((8.0 * locals.var_gf2_dn8) / (locals.var_gf2 * locals.var_gf2))) / (2.0 * assign41490_e54445))));
        locals.var_qbscr_dn9 = (((0.5 * locals.var_gf2_dn9) * assign41490_e54447) + (assign41490_e54439 * ((-((8.0 * locals.var_gf2_dn9) / (locals.var_gf2 * locals.var_gf2))) / (2.0 * assign41490_e54445))));
        locals.var_qbscr_rv = 0.0;

        locals.var_qiscr = 0.0;
        locals.var_qiscr_dn4 = 0.0;
        locals.var_qiscr_dn6 = 0.0;
        locals.var_qiscr_dn7 = 0.0;
        locals.var_qiscr_dn8 = 0.0;
        locals.var_qiscr_dn9 = 0.0;
        locals.var_qiscr_rv = 0.0;

        locals.var_fscr = 1.0;
        locals.var_fscr_dn4 = 0.0;
        locals.var_fscr_dn6 = 0.0;
        locals.var_fscr_dn7 = 0.0;
        locals.var_fscr_dn8 = 0.0;
        locals.var_fscr_dn9 = 0.0;
        locals.var_fscr_rv = 0.0;

        let assign41520_e54453: f64 = (-30.0);
        let assign41520_e54454: f64 = if locals.var_xgtscr > assign41520_e54453 { 1.0 } else { 0.0 };
        locals.var_guard1195 = assign41520_e54454;
        locals.var_guard1195_rv = 0.0;

        let (assign41530_e54462, assign41530_e54462_d_n4, assign41530_e54462_d_n6, assign41530_e54462_d_n7, assign41530_e54462_d_n8, assign41530_e54462_d_n9,) = {
    if (locals.var_guard1195 != 0.0) {
        let assign41530_e54458: f64 = (locals.var_nscr * locals.var_xgtscr);
        let assign41530_e54460: f64 = (assign41530_e54458 - 1.0);
        (assign41530_e54460, ((locals.var_nscr_dn4 * locals.var_xgtscr) + (locals.var_nscr * locals.var_xgtscr_dn4)), ((locals.var_nscr_dn6 * locals.var_xgtscr) + (locals.var_nscr * locals.var_xgtscr_dn6)), ((locals.var_nscr_dn7 * locals.var_xgtscr) + (locals.var_nscr * locals.var_xgtscr_dn7)), ((locals.var_nscr_dn8 * locals.var_xgtscr) + (locals.var_nscr * locals.var_xgtscr_dn8)), ((locals.var_nscr_dn9 * locals.var_xgtscr) + (locals.var_nscr * locals.var_xgtscr_dn9)),)
    } else {
        (locals.var_xgtscr0, locals.var_xgtscr0_dn4, locals.var_xgtscr0_dn6, locals.var_xgtscr0_dn7, locals.var_xgtscr0_dn8, locals.var_xgtscr0_dn9,)
    }
};
        locals.var_xgtscr0 = assign41530_e54462;
        locals.var_xgtscr0_dn4 = assign41530_e54462_d_n4;
        locals.var_xgtscr0_dn6 = assign41530_e54462_d_n6;
        locals.var_xgtscr0_dn7 = assign41530_e54462_d_n7;
        locals.var_xgtscr0_dn8 = assign41530_e54462_d_n8;
        locals.var_xgtscr0_dn9 = assign41530_e54462_d_n9;
        locals.var_xgtscr0_rv = 0.0;

        let (assign41540_e54475, assign41540_e54475_d_n4, assign41540_e54475_d_n6, assign41540_e54475_d_n7, assign41540_e54475_d_n8, assign41540_e54475_d_n9,) = {
    if (locals.var_guard1195 != 0.0) {
        let assign41540_e54468: f64 = (locals.var_xgtscr0 * locals.var_xgtscr0);
        let assign41540_e54470: f64 = (assign41540_e54468 + 10.0);
        let assign41540_e54471: f64 = (assign41540_e54470).sqrt();
        let assign41540_e54472: f64 = (locals.var_xgtscr0 + assign41540_e54471);
        let assign41540_e54473: f64 = (0.5 * assign41540_e54472);
        (assign41540_e54473, (0.5 * (locals.var_xgtscr0_dn4 + (((locals.var_xgtscr0_dn4 * locals.var_xgtscr0) + (locals.var_xgtscr0 * locals.var_xgtscr0_dn4)) / (2.0 * assign41540_e54471)))), (0.5 * (locals.var_xgtscr0_dn6 + (((locals.var_xgtscr0_dn6 * locals.var_xgtscr0) + (locals.var_xgtscr0 * locals.var_xgtscr0_dn6)) / (2.0 * assign41540_e54471)))), (0.5 * (locals.var_xgtscr0_dn7 + (((locals.var_xgtscr0_dn7 * locals.var_xgtscr0) + (locals.var_xgtscr0 * locals.var_xgtscr0_dn7)) / (2.0 * assign41540_e54471)))), (0.5 * (locals.var_xgtscr0_dn8 + (((locals.var_xgtscr0_dn8 * locals.var_xgtscr0) + (locals.var_xgtscr0 * locals.var_xgtscr0_dn8)) / (2.0 * assign41540_e54471)))), (0.5 * (locals.var_xgtscr0_dn9 + (((locals.var_xgtscr0_dn9 * locals.var_xgtscr0) + (locals.var_xgtscr0 * locals.var_xgtscr0_dn9)) / (2.0 * assign41540_e54471)))),)
    } else {
        (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
    }
};
        locals.var_temp__blk949 = assign41540_e54475;
        locals.var_temp__blk949_dn4 = assign41540_e54475_d_n4;
        locals.var_temp__blk949_dn6 = assign41540_e54475_d_n6;
        locals.var_temp__blk949_dn7 = assign41540_e54475_d_n7;
        locals.var_temp__blk949_dn8 = assign41540_e54475_d_n8;
        locals.var_temp__blk949_dn9 = assign41540_e54475_d_n9;
        locals.var_temp__blk949_rv = 0.0;

        let (assign41550_e54482, assign41550_e54482_d_n4, assign41550_e54482_d_n6, assign41550_e54482_d_n7, assign41550_e54482_d_n8, assign41550_e54482_d_n9,) = {
    if (locals.var_guard1195 != 0.0) {
        let assign41550_e54479: f64 = (locals.var_temp__blk949).ln();
        let assign41550_e54480: f64 = (locals.var_xgtscr - assign41550_e54479);
        (assign41550_e54480, (locals.var_xgtscr_dn4 - (locals.var_temp__blk949_dn4 / locals.var_temp__blk949)), (locals.var_xgtscr_dn6 - (locals.var_temp__blk949_dn6 / locals.var_temp__blk949)), (locals.var_xgtscr_dn7 - (locals.var_temp__blk949_dn7 / locals.var_temp__blk949)), (locals.var_xgtscr_dn8 - (locals.var_temp__blk949_dn8 / locals.var_temp__blk949)), (locals.var_xgtscr_dn9 - (locals.var_temp__blk949_dn9 / locals.var_temp__blk949)),)
    } else {
        (locals.var_qiscr0si, locals.var_qiscr0si_dn4, locals.var_qiscr0si_dn6, locals.var_qiscr0si_dn7, locals.var_qiscr0si_dn8, locals.var_qiscr0si_dn9,)
    }
};
        locals.var_qiscr0si = assign41550_e54482;
        locals.var_qiscr0si_dn4 = assign41550_e54482_d_n4;
        locals.var_qiscr0si_dn6 = assign41550_e54482_d_n6;
        locals.var_qiscr0si_dn7 = assign41550_e54482_d_n7;
        locals.var_qiscr0si_dn8 = assign41550_e54482_d_n8;
        locals.var_qiscr0si_dn9 = assign41550_e54482_d_n9;
        locals.var_qiscr0si_rv = 0.0;

        let (assign41560_e54495, assign41560_e54495_d_n4, assign41560_e54495_d_n6, assign41560_e54495_d_n7, assign41560_e54495_d_n8, assign41560_e54495_d_n9,) = {
    if (locals.var_guard1195 != 0.0) {
        let assign41560_e54488: f64 = (locals.var_qiscr0si * locals.var_qiscr0si);
        let assign41560_e54490: f64 = (assign41560_e54488 + 2.0);
        let assign41560_e54491: f64 = (assign41560_e54490).sqrt();
        let assign41560_e54492: f64 = (locals.var_qiscr0si + assign41560_e54491);
        let assign41560_e54493: f64 = (0.5 * assign41560_e54492);
        (assign41560_e54493, (0.5 * (locals.var_qiscr0si_dn4 + (((locals.var_qiscr0si_dn4 * locals.var_qiscr0si) + (locals.var_qiscr0si * locals.var_qiscr0si_dn4)) / (2.0 * assign41560_e54491)))), (0.5 * (locals.var_qiscr0si_dn6 + (((locals.var_qiscr0si_dn6 * locals.var_qiscr0si) + (locals.var_qiscr0si * locals.var_qiscr0si_dn6)) / (2.0 * assign41560_e54491)))), (0.5 * (locals.var_qiscr0si_dn7 + (((locals.var_qiscr0si_dn7 * locals.var_qiscr0si) + (locals.var_qiscr0si * locals.var_qiscr0si_dn7)) / (2.0 * assign41560_e54491)))), (0.5 * (locals.var_qiscr0si_dn8 + (((locals.var_qiscr0si_dn8 * locals.var_qiscr0si) + (locals.var_qiscr0si * locals.var_qiscr0si_dn8)) / (2.0 * assign41560_e54491)))), (0.5 * (locals.var_qiscr0si_dn9 + (((locals.var_qiscr0si_dn9 * locals.var_qiscr0si) + (locals.var_qiscr0si * locals.var_qiscr0si_dn9)) / (2.0 * assign41560_e54491)))),)
    } else {
        (locals.var_qiscr0, locals.var_qiscr0_dn4, locals.var_qiscr0_dn6, locals.var_qiscr0_dn7, locals.var_qiscr0_dn8, locals.var_qiscr0_dn9,)
    }
};
        locals.var_qiscr0 = assign41560_e54495;
        locals.var_qiscr0_dn4 = assign41560_e54495_d_n4;
        locals.var_qiscr0_dn6 = assign41560_e54495_d_n6;
        locals.var_qiscr0_dn7 = assign41560_e54495_d_n7;
        locals.var_qiscr0_dn8 = assign41560_e54495_d_n8;
        locals.var_qiscr0_dn9 = assign41560_e54495_d_n9;
        locals.var_qiscr0_rv = 0.0;

        let assign41570_e54498: f64 = (locals.var_xgtscr - locals.var_qiscr0);
        let assign41570_e54500: f64 = if assign41570_e54498 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard1196 = assign41570_e54500;
        locals.var_guard1196_rv = 0.0;

        let (assign41580_e54509, assign41580_e54509_d_n4, assign41580_e54509_d_n6, assign41580_e54509_d_n7, assign41580_e54509_d_n8, assign41580_e54509_d_n9,) = {
    if ((locals.var_guard1195 != 0.0) && (locals.var_guard1196 != 0.0)) {
        let assign41580_e54506: f64 = (locals.var_xgtscr - locals.var_qiscr0);
        let assign41580_e54507: f64 = (assign41580_e54506).exp();
        (assign41580_e54507, (assign41580_e54507 * (locals.var_xgtscr_dn4 - locals.var_qiscr0_dn4)), (assign41580_e54507 * (locals.var_xgtscr_dn6 - locals.var_qiscr0_dn6)), (assign41580_e54507 * (locals.var_xgtscr_dn7 - locals.var_qiscr0_dn7)), (assign41580_e54507 * (locals.var_xgtscr_dn8 - locals.var_qiscr0_dn8)), (assign41580_e54507 * (locals.var_xgtscr_dn9 - locals.var_qiscr0_dn9)),)
    } else {
        (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
    }
};
        locals.var_temp__blk949 = assign41580_e54509;
        locals.var_temp__blk949_dn4 = assign41580_e54509_d_n4;
        locals.var_temp__blk949_dn6 = assign41580_e54509_d_n6;
        locals.var_temp__blk949_dn7 = assign41580_e54509_d_n7;
        locals.var_temp__blk949_dn8 = assign41580_e54509_d_n8;
        locals.var_temp__blk949_dn9 = assign41580_e54509_d_n9;
        locals.var_temp__blk949_rv = 0.0;

        let (assign41590_e54544, assign41590_e54544_d_n4, assign41590_e54544_d_n6, assign41590_e54544_d_n7, assign41590_e54544_d_n8, assign41590_e54544_d_n9,) = {
    if ((locals.var_guard1195 != 0.0) && (locals.var_guard1196 == 0.0)) {
        let assign41590_e54518: f64 = (locals.var_xgtscr - locals.var_qiscr0);
        let assign41590_e54520: f64 = (assign41590_e54518 - 230.25850929940458);
        let assign41590_e54525: f64 = (locals.var_xgtscr - locals.var_qiscr0);
        let assign41590_e54527: f64 = (assign41590_e54525 - 230.25850929940458);
        let assign41590_e54531: f64 = (locals.var_xgtscr - locals.var_qiscr0);
        let assign41590_e54533: f64 = (assign41590_e54531 - 230.25850929940458);
        let assign41590_e54535: f64 = (assign41590_e54533 * 0.3333333333333333);
        let assign41590_e54536: f64 = (1.0 + assign41590_e54535);
        let assign41590_e54537: f64 = (assign41590_e54527 * assign41590_e54536);
        let assign41590_e54538: f64 = (0.5 * assign41590_e54537);
        let assign41590_e54539: f64 = (1.0 + assign41590_e54538);
        let assign41590_e54540: f64 = (assign41590_e54520 * assign41590_e54539);
        let assign41590_e54541: f64 = (1.0 + assign41590_e54540);
        let assign41590_e54542: f64 = (1e100 * assign41590_e54541);
        (assign41590_e54542, (1e100 * (((locals.var_xgtscr_dn4 - locals.var_qiscr0_dn4) * assign41590_e54539) + (assign41590_e54520 * (0.5 * (((locals.var_xgtscr_dn4 - locals.var_qiscr0_dn4) * assign41590_e54536) + (assign41590_e54527 * ((locals.var_xgtscr_dn4 - locals.var_qiscr0_dn4) * 0.3333333333333333))))))), (1e100 * (((locals.var_xgtscr_dn6 - locals.var_qiscr0_dn6) * assign41590_e54539) + (assign41590_e54520 * (0.5 * (((locals.var_xgtscr_dn6 - locals.var_qiscr0_dn6) * assign41590_e54536) + (assign41590_e54527 * ((locals.var_xgtscr_dn6 - locals.var_qiscr0_dn6) * 0.3333333333333333))))))), (1e100 * (((locals.var_xgtscr_dn7 - locals.var_qiscr0_dn7) * assign41590_e54539) + (assign41590_e54520 * (0.5 * (((locals.var_xgtscr_dn7 - locals.var_qiscr0_dn7) * assign41590_e54536) + (assign41590_e54527 * ((locals.var_xgtscr_dn7 - locals.var_qiscr0_dn7) * 0.3333333333333333))))))), (1e100 * (((locals.var_xgtscr_dn8 - locals.var_qiscr0_dn8) * assign41590_e54539) + (assign41590_e54520 * (0.5 * (((locals.var_xgtscr_dn8 - locals.var_qiscr0_dn8) * assign41590_e54536) + (assign41590_e54527 * ((locals.var_xgtscr_dn8 - locals.var_qiscr0_dn8) * 0.3333333333333333))))))), (1e100 * (((locals.var_xgtscr_dn9 - locals.var_qiscr0_dn9) * assign41590_e54539) + (assign41590_e54520 * (0.5 * (((locals.var_xgtscr_dn9 - locals.var_qiscr0_dn9) * assign41590_e54536) + (assign41590_e54527 * ((locals.var_xgtscr_dn9 - locals.var_qiscr0_dn9) * 0.3333333333333333))))))),)
    } else {
        (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
    }
};
        locals.var_temp__blk949 = assign41590_e54544;
        locals.var_temp__blk949_dn4 = assign41590_e54544_d_n4;
        locals.var_temp__blk949_dn6 = assign41590_e54544_d_n6;
        locals.var_temp__blk949_dn7 = assign41590_e54544_d_n7;
        locals.var_temp__blk949_dn8 = assign41590_e54544_d_n8;
        locals.var_temp__blk949_dn9 = assign41590_e54544_d_n9;
        locals.var_temp__blk949_rv = 0.0;

        let (assign41600_e54550, assign41600_e54550_d_n4, assign41600_e54550_d_n6, assign41600_e54550_d_n7, assign41600_e54550_d_n8, assign41600_e54550_d_n9,) = {
    if (locals.var_guard1195 != 0.0) {
        let assign41600_e54548: f64 = (locals.var_temp__blk949 / locals.var_nscr);
        (assign41600_e54548, (((locals.var_temp__blk949_dn4 * locals.var_nscr) - (locals.var_temp__blk949 * locals.var_nscr_dn4)) / (locals.var_nscr * locals.var_nscr)), (((locals.var_temp__blk949_dn6 * locals.var_nscr) - (locals.var_temp__blk949 * locals.var_nscr_dn6)) / (locals.var_nscr * locals.var_nscr)), (((locals.var_temp__blk949_dn7 * locals.var_nscr) - (locals.var_temp__blk949 * locals.var_nscr_dn7)) / (locals.var_nscr * locals.var_nscr)), (((locals.var_temp__blk949_dn8 * locals.var_nscr) - (locals.var_temp__blk949 * locals.var_nscr_dn8)) / (locals.var_nscr * locals.var_nscr)), (((locals.var_temp__blk949_dn9 * locals.var_nscr) - (locals.var_temp__blk949 * locals.var_nscr_dn9)) / (locals.var_nscr * locals.var_nscr)),)
    } else {
        (locals.var_dscr0, locals.var_dscr0_dn4, locals.var_dscr0_dn6, locals.var_dscr0_dn7, locals.var_dscr0_dn8, locals.var_dscr0_dn9,)
    }
};
        locals.var_dscr0 = assign41600_e54550;
        locals.var_dscr0_dn4 = assign41600_e54550_d_n4;
        locals.var_dscr0_dn6 = assign41600_e54550_d_n6;
        locals.var_dscr0_dn7 = assign41600_e54550_d_n7;
        locals.var_dscr0_dn8 = assign41600_e54550_d_n8;
        locals.var_dscr0_dn9 = assign41600_e54550_d_n9;
        locals.var_dscr0_rv = 0.0;

        let (assign41610_e54560, assign41610_e54560_d_n4, assign41610_e54560_d_n6, assign41610_e54560_d_n7, assign41610_e54560_d_n8, assign41610_e54560_d_n9,) = {
    if (locals.var_guard1195 != 0.0) {
        let assign41610_e54555: f64 = (locals.var_qiscr0 + 1.0);
        let assign41610_e54556: f64 = (2.0 * assign41610_e54555);
        let assign41610_e54558: f64 = (assign41610_e54556 - locals.var_dscr0);
        (assign41610_e54558, ((2.0 * locals.var_qiscr0_dn4) - locals.var_dscr0_dn4), ((2.0 * locals.var_qiscr0_dn6) - locals.var_dscr0_dn6), ((2.0 * locals.var_qiscr0_dn7) - locals.var_dscr0_dn7), ((2.0 * locals.var_qiscr0_dn8) - locals.var_dscr0_dn8), ((2.0 * locals.var_qiscr0_dn9) - locals.var_dscr0_dn9),)
    } else {
        (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
    }
};
        locals.var_temp__blk949 = assign41610_e54560;
        locals.var_temp__blk949_dn4 = assign41610_e54560_d_n4;
        locals.var_temp__blk949_dn6 = assign41610_e54560_d_n6;
        locals.var_temp__blk949_dn7 = assign41610_e54560_d_n7;
        locals.var_temp__blk949_dn8 = assign41610_e54560_d_n8;
        locals.var_temp__blk949_dn9 = assign41610_e54560_d_n9;
        locals.var_temp__blk949_rv = 0.0;

        let assign41620_e54563: f64 = if locals.var_dscr0 > 1e-6 { 1.0 } else { 0.0 };
        locals.var_guard1197 = assign41620_e54563;
        locals.var_guard1197_rv = 0.0;

        let (assign41630_e54584, assign41630_e54584_d_n4, assign41630_e54584_d_n6, assign41630_e54584_d_n7, assign41630_e54584_d_n8, assign41630_e54584_d_n9,) = {
    if ((locals.var_guard1195 != 0.0) && (locals.var_guard1197 != 0.0)) {
        let assign41630_e54572: f64 = (locals.var_dscr0 * locals.var_temp__blk949);
        let assign41630_e54573: f64 = (1.0 + assign41630_e54572);
        let assign41630_e54574: f64 = (assign41630_e54573).sqrt();
        let assign41630_e54576: f64 = (assign41630_e54574 - 1.0);
        let assign41630_e54578: f64 = (assign41630_e54576 / locals.var_dscr0);
        let assign41630_e54579: f64 = (locals.var_qiscr0 - assign41630_e54578);
        let assign41630_e54581: f64 = (assign41630_e54579 + 1.0);
        let assign41630_e54582: f64 = (locals.var_nscr * assign41630_e54581);
        (assign41630_e54582, ((locals.var_nscr_dn4 * assign41630_e54581) + (locals.var_nscr * (locals.var_qiscr0_dn4 - ((((((locals.var_dscr0_dn4 * locals.var_temp__blk949) + (locals.var_dscr0 * locals.var_temp__blk949_dn4)) / (2.0 * assign41630_e54574)) * locals.var_dscr0) - (assign41630_e54576 * locals.var_dscr0_dn4)) / (locals.var_dscr0 * locals.var_dscr0))))), ((locals.var_nscr_dn6 * assign41630_e54581) + (locals.var_nscr * (locals.var_qiscr0_dn6 - ((((((locals.var_dscr0_dn6 * locals.var_temp__blk949) + (locals.var_dscr0 * locals.var_temp__blk949_dn6)) / (2.0 * assign41630_e54574)) * locals.var_dscr0) - (assign41630_e54576 * locals.var_dscr0_dn6)) / (locals.var_dscr0 * locals.var_dscr0))))), ((locals.var_nscr_dn7 * assign41630_e54581) + (locals.var_nscr * (locals.var_qiscr0_dn7 - ((((((locals.var_dscr0_dn7 * locals.var_temp__blk949) + (locals.var_dscr0 * locals.var_temp__blk949_dn7)) / (2.0 * assign41630_e54574)) * locals.var_dscr0) - (assign41630_e54576 * locals.var_dscr0_dn7)) / (locals.var_dscr0 * locals.var_dscr0))))), ((locals.var_nscr_dn8 * assign41630_e54581) + (locals.var_nscr * (locals.var_qiscr0_dn8 - ((((((locals.var_dscr0_dn8 * locals.var_temp__blk949) + (locals.var_dscr0 * locals.var_temp__blk949_dn8)) / (2.0 * assign41630_e54574)) * locals.var_dscr0) - (assign41630_e54576 * locals.var_dscr0_dn8)) / (locals.var_dscr0 * locals.var_dscr0))))), ((locals.var_nscr_dn9 * assign41630_e54581) + (locals.var_nscr * (locals.var_qiscr0_dn9 - ((((((locals.var_dscr0_dn9 * locals.var_temp__blk949) + (locals.var_dscr0 * locals.var_temp__blk949_dn9)) / (2.0 * assign41630_e54574)) * locals.var_dscr0) - (assign41630_e54576 * locals.var_dscr0_dn9)) / (locals.var_dscr0 * locals.var_dscr0))))),)
    } else {
        (locals.var_qiscr, locals.var_qiscr_dn4, locals.var_qiscr_dn6, locals.var_qiscr_dn7, locals.var_qiscr_dn8, locals.var_qiscr_dn9,)
    }
};
        locals.var_qiscr = assign41630_e54584;
        locals.var_qiscr_dn4 = assign41630_e54584_d_n4;
        locals.var_qiscr_dn6 = assign41630_e54584_d_n6;
        locals.var_qiscr_dn7 = assign41630_e54584_d_n7;
        locals.var_qiscr_dn8 = assign41630_e54584_d_n8;
        locals.var_qiscr_dn9 = assign41630_e54584_d_n9;
        locals.var_qiscr_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_18(
        locals: &mut StampLocals,
    ) {
        let (assign41640_e54603, assign41640_e54603_d_n4, assign41640_e54603_d_n6, assign41640_e54603_d_n7, assign41640_e54603_d_n8, assign41640_e54603_d_n9,) = {
    if ((locals.var_guard1195 != 0.0) && (locals.var_guard1197 == 0.0)) {
        let assign41640_e54591: f64 = (locals.var_nscr * 0.5);
        let assign41640_e54593: f64 = (assign41640_e54591 * locals.var_dscr0);
        let assign41640_e54597: f64 = (0.25 * locals.var_temp__blk949);
        let assign41640_e54599: f64 = (assign41640_e54597 * locals.var_temp__blk949);
        let assign41640_e54600: f64 = (1.0 + assign41640_e54599);
        let assign41640_e54601: f64 = (assign41640_e54593 * assign41640_e54600);
        (assign41640_e54601, (((((locals.var_nscr_dn4 * 0.5) * locals.var_dscr0) + (assign41640_e54591 * locals.var_dscr0_dn4)) * assign41640_e54600) + (assign41640_e54593 * (((0.25 * locals.var_temp__blk949_dn4) * locals.var_temp__blk949) + (assign41640_e54597 * locals.var_temp__blk949_dn4)))), (((((locals.var_nscr_dn6 * 0.5) * locals.var_dscr0) + (assign41640_e54591 * locals.var_dscr0_dn6)) * assign41640_e54600) + (assign41640_e54593 * (((0.25 * locals.var_temp__blk949_dn6) * locals.var_temp__blk949) + (assign41640_e54597 * locals.var_temp__blk949_dn6)))), (((((locals.var_nscr_dn7 * 0.5) * locals.var_dscr0) + (assign41640_e54591 * locals.var_dscr0_dn7)) * assign41640_e54600) + (assign41640_e54593 * (((0.25 * locals.var_temp__blk949_dn7) * locals.var_temp__blk949) + (assign41640_e54597 * locals.var_temp__blk949_dn7)))), (((((locals.var_nscr_dn8 * 0.5) * locals.var_dscr0) + (assign41640_e54591 * locals.var_dscr0_dn8)) * assign41640_e54600) + (assign41640_e54593 * (((0.25 * locals.var_temp__blk949_dn8) * locals.var_temp__blk949) + (assign41640_e54597 * locals.var_temp__blk949_dn8)))), (((((locals.var_nscr_dn9 * 0.5) * locals.var_dscr0) + (assign41640_e54591 * locals.var_dscr0_dn9)) * assign41640_e54600) + (assign41640_e54593 * (((0.25 * locals.var_temp__blk949_dn9) * locals.var_temp__blk949) + (assign41640_e54597 * locals.var_temp__blk949_dn9)))),)
    } else {
        (locals.var_qiscr, locals.var_qiscr_dn4, locals.var_qiscr_dn6, locals.var_qiscr_dn7, locals.var_qiscr_dn8, locals.var_qiscr_dn9,)
    }
};
        locals.var_qiscr = assign41640_e54603;
        locals.var_qiscr_dn4 = assign41640_e54603_d_n4;
        locals.var_qiscr_dn6 = assign41640_e54603_d_n6;
        locals.var_qiscr_dn7 = assign41640_e54603_d_n7;
        locals.var_qiscr_dn8 = assign41640_e54603_d_n8;
        locals.var_qiscr_dn9 = assign41640_e54603_d_n9;
        locals.var_qiscr_rv = 0.0;

        let (assign41650_e54628, assign41650_e54628_d_n4, assign41650_e54628_d_n6, assign41650_e54628_d_n7, assign41650_e54628_d_n8, assign41650_e54628_d_n9,) = {
    if (locals.var_guard1195 != 0.0) {
        let assign41650_e54608: f64 = (locals.var_xg - locals.var_qiscr);
        let assign41650_e54610: f64 = (assign41650_e54608 + 2.0);
        let assign41650_e54613: f64 = (locals.var_xg - locals.var_qiscr);
        let assign41650_e54615: f64 = (assign41650_e54613 - 2.0);
        let assign41650_e54618: f64 = (locals.var_xg - locals.var_qiscr);
        let assign41650_e54620: f64 = (assign41650_e54618 - 2.0);
        let assign41650_e54621: f64 = (assign41650_e54615 * assign41650_e54620);
        let assign41650_e54623: f64 = (assign41650_e54621 + 1.0);
        let assign41650_e54624: f64 = (assign41650_e54623).sqrt();
        let assign41650_e54625: f64 = (assign41650_e54610 + assign41650_e54624);
        let assign41650_e54626: f64 = (0.5 * assign41650_e54625);
        (assign41650_e54626, (0.5 * ((locals.var_xg_dn4 - locals.var_qiscr_dn4) + ((((locals.var_xg_dn4 - locals.var_qiscr_dn4) * assign41650_e54620) + (assign41650_e54615 * (locals.var_xg_dn4 - locals.var_qiscr_dn4))) / (2.0 * assign41650_e54624)))), (0.5 * ((locals.var_xg_dn6 - locals.var_qiscr_dn6) + ((((locals.var_xg_dn6 - locals.var_qiscr_dn6) * assign41650_e54620) + (assign41650_e54615 * (locals.var_xg_dn6 - locals.var_qiscr_dn6))) / (2.0 * assign41650_e54624)))), (0.5 * ((locals.var_xg_dn7 - locals.var_qiscr_dn7) + ((((locals.var_xg_dn7 - locals.var_qiscr_dn7) * assign41650_e54620) + (assign41650_e54615 * (locals.var_xg_dn7 - locals.var_qiscr_dn7))) / (2.0 * assign41650_e54624)))), (0.5 * ((locals.var_xg_dn8 - locals.var_qiscr_dn8) + ((((locals.var_xg_dn8 - locals.var_qiscr_dn8) * assign41650_e54620) + (assign41650_e54615 * (locals.var_xg_dn8 - locals.var_qiscr_dn8))) / (2.0 * assign41650_e54624)))), (0.5 * ((locals.var_xg_dn9 - locals.var_qiscr_dn9) + ((((locals.var_xg_dn9 - locals.var_qiscr_dn9) * assign41650_e54620) + (assign41650_e54615 * (locals.var_xg_dn9 - locals.var_qiscr_dn9))) / (2.0 * assign41650_e54624)))),)
    } else {
        (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
    }
};
        locals.var_temp__blk949 = assign41650_e54628;
        locals.var_temp__blk949_dn4 = assign41650_e54628_d_n4;
        locals.var_temp__blk949_dn6 = assign41650_e54628_d_n6;
        locals.var_temp__blk949_dn7 = assign41650_e54628_d_n7;
        locals.var_temp__blk949_dn8 = assign41650_e54628_d_n8;
        locals.var_temp__blk949_dn9 = assign41650_e54628_d_n9;
        locals.var_temp__blk949_rv = 0.0;

        let (assign41660_e54645, assign41660_e54645_d_n4, assign41660_e54645_d_n6, assign41660_e54645_d_n7, assign41660_e54645_d_n8, assign41660_e54645_d_n9,) = {
    if (locals.var_guard1195 != 0.0) {
        let assign41660_e54632: f64 = (0.5 * locals.var_gf2);
        let assign41660_e54636: f64 = (4.0 / locals.var_gf2);
        let assign41660_e54638: f64 = (assign41660_e54636 * locals.var_temp__blk949);
        let assign41660_e54639: f64 = (1.0 + assign41660_e54638);
        let assign41660_e54640: f64 = (assign41660_e54639).sqrt();
        let assign41660_e54642: f64 = (assign41660_e54640 - 1.0);
        let assign41660_e54643: f64 = (assign41660_e54632 * assign41660_e54642);
        (assign41660_e54643, (((0.5 * locals.var_gf2_dn4) * assign41660_e54642) + (assign41660_e54632 * ((((-((4.0 * locals.var_gf2_dn4) / (locals.var_gf2 * locals.var_gf2))) * locals.var_temp__blk949) + (assign41660_e54636 * locals.var_temp__blk949_dn4)) / (2.0 * assign41660_e54640)))), (((0.5 * locals.var_gf2_dn6) * assign41660_e54642) + (assign41660_e54632 * ((((-((4.0 * locals.var_gf2_dn6) / (locals.var_gf2 * locals.var_gf2))) * locals.var_temp__blk949) + (assign41660_e54636 * locals.var_temp__blk949_dn6)) / (2.0 * assign41660_e54640)))), (((0.5 * locals.var_gf2_dn7) * assign41660_e54642) + (assign41660_e54632 * ((((-((4.0 * locals.var_gf2_dn7) / (locals.var_gf2 * locals.var_gf2))) * locals.var_temp__blk949) + (assign41660_e54636 * locals.var_temp__blk949_dn7)) / (2.0 * assign41660_e54640)))), (((0.5 * locals.var_gf2_dn8) * assign41660_e54642) + (assign41660_e54632 * ((((-((4.0 * locals.var_gf2_dn8) / (locals.var_gf2 * locals.var_gf2))) * locals.var_temp__blk949) + (assign41660_e54636 * locals.var_temp__blk949_dn8)) / (2.0 * assign41660_e54640)))), (((0.5 * locals.var_gf2_dn9) * assign41660_e54642) + (assign41660_e54632 * ((((-((4.0 * locals.var_gf2_dn9) / (locals.var_gf2 * locals.var_gf2))) * locals.var_temp__blk949) + (assign41660_e54636 * locals.var_temp__blk949_dn9)) / (2.0 * assign41660_e54640)))),)
    } else {
        (locals.var_qbscr, locals.var_qbscr_dn4, locals.var_qbscr_dn6, locals.var_qbscr_dn7, locals.var_qbscr_dn8, locals.var_qbscr_dn9,)
    }
};
        locals.var_qbscr = assign41660_e54645;
        locals.var_qbscr_dn4 = assign41660_e54645_d_n4;
        locals.var_qbscr_dn6 = assign41660_e54645_d_n6;
        locals.var_qbscr_dn7 = assign41660_e54645_d_n7;
        locals.var_qbscr_dn8 = assign41660_e54645_d_n8;
        locals.var_qbscr_dn9 = assign41660_e54645_d_n9;
        locals.var_qbscr_rv = 0.0;

        let (assign41670_e54653, assign41670_e54653_d_n4, assign41670_e54653_d_n6, assign41670_e54653_d_n7, assign41670_e54653_d_n8, assign41670_e54653_d_n9,) = {
    if (locals.var_guard1195 != 0.0) {
        let assign41670_e54650: f64 = (locals.var_qbscr + locals.var_qiscr);
        let assign41670_e54651: f64 = (locals.var_qbscr / assign41670_e54650);
        (assign41670_e54651, (((locals.var_qbscr_dn4 * assign41670_e54650) - (locals.var_qbscr * (locals.var_qbscr_dn4 + locals.var_qiscr_dn4))) / (assign41670_e54650 * assign41670_e54650)), (((locals.var_qbscr_dn6 * assign41670_e54650) - (locals.var_qbscr * (locals.var_qbscr_dn6 + locals.var_qiscr_dn6))) / (assign41670_e54650 * assign41670_e54650)), (((locals.var_qbscr_dn7 * assign41670_e54650) - (locals.var_qbscr * (locals.var_qbscr_dn7 + locals.var_qiscr_dn7))) / (assign41670_e54650 * assign41670_e54650)), (((locals.var_qbscr_dn8 * assign41670_e54650) - (locals.var_qbscr * (locals.var_qbscr_dn8 + locals.var_qiscr_dn8))) / (assign41670_e54650 * assign41670_e54650)), (((locals.var_qbscr_dn9 * assign41670_e54650) - (locals.var_qbscr * (locals.var_qbscr_dn9 + locals.var_qiscr_dn9))) / (assign41670_e54650 * assign41670_e54650)),)
    } else {
        (locals.var_fscr, locals.var_fscr_dn4, locals.var_fscr_dn6, locals.var_fscr_dn7, locals.var_fscr_dn8, locals.var_fscr_dn9,)
    }
};
        locals.var_fscr = assign41670_e54653;
        locals.var_fscr_dn4 = assign41670_e54653_d_n4;
        locals.var_fscr_dn6 = assign41670_e54653_d_n6;
        locals.var_fscr_dn7 = assign41670_e54653_d_n7;
        locals.var_fscr_dn8 = assign41670_e54653_d_n8;
        locals.var_fscr_dn9 = assign41670_e54653_d_n9;
        locals.var_fscr_rv = 0.0;

        let (assign41680_e54661, assign41680_e54661_d_n4, assign41680_e54661_d_n6, assign41680_e54661_d_n7, assign41680_e54661_d_n8, assign41680_e54661_d_n9,) = {
    if (locals.var_guard1195 != 0.0) {
        let assign41680_e54658: f64 = (locals.var_fscr * locals.var_delxb);
        let assign41680_e54659: f64 = (locals.var_xno_s - assign41680_e54658);
        (assign41680_e54659, (locals.var_xno_s_dn4 - ((locals.var_fscr_dn4 * locals.var_delxb) + (locals.var_fscr * locals.var_delxb_dn4))), (locals.var_xno_s_dn6 - ((locals.var_fscr_dn6 * locals.var_delxb) + (locals.var_fscr * locals.var_delxb_dn6))), (locals.var_xno_s_dn7 - ((locals.var_fscr_dn7 * locals.var_delxb) + (locals.var_fscr * locals.var_delxb_dn7))), (locals.var_xno_s_dn8 - ((locals.var_fscr_dn8 * locals.var_delxb) + (locals.var_fscr * locals.var_delxb_dn8))), (locals.var_xno_s_dn9 - ((locals.var_fscr_dn9 * locals.var_delxb) + (locals.var_fscr * locals.var_delxb_dn9))),)
    } else {
        (locals.var_xn_s, locals.var_xn_s_dn4, locals.var_xn_s_dn6, locals.var_xn_s_dn7, locals.var_xn_s_dn8, locals.var_xn_s_dn9,)
    }
};
        locals.var_xn_s = assign41680_e54661;
        locals.var_xn_s_dn4 = assign41680_e54661_d_n4;
        locals.var_xn_s_dn6 = assign41680_e54661_d_n6;
        locals.var_xn_s_dn7 = assign41680_e54661_d_n7;
        locals.var_xn_s_dn8 = assign41680_e54661_d_n8;
        locals.var_xn_s_dn9 = assign41680_e54661_d_n9;
        locals.var_xn_s_rv = 0.0;

        let assign41690_e54665: f64 = (locals.var_gf * 0.7071067811865475);
        let assign41690_e54666: f64 = (1.0 + assign41690_e54665);
        locals.var_xi = assign41690_e54666;
        locals.var_xi_dn4 = (locals.var_gf_dn4 * 0.7071067811865475);
        locals.var_xi_dn6 = (locals.var_gf_dn6 * 0.7071067811865475);
        locals.var_xi_dn7 = (locals.var_gf_dn7 * 0.7071067811865475);
        locals.var_xi_dn8 = (locals.var_gf_dn8 * 0.7071067811865475);
        locals.var_xi_dn9 = (locals.var_gf_dn9 * 0.7071067811865475);
        locals.var_xi_rv = 0.0;

        let assign41700_e54669: f64 = (1e-5 * locals.var_xi);
        locals.var_margin = assign41700_e54669;
        locals.var_margin_dn4 = (1e-5 * locals.var_xi_dn4);
        locals.var_margin_dn6 = (1e-5 * locals.var_xi_dn6);
        locals.var_margin_dn7 = (1e-5 * locals.var_xi_dn7);
        locals.var_margin_dn8 = (1e-5 * locals.var_xi_dn8);
        locals.var_margin_dn9 = (1e-5 * locals.var_xi_dn9);
        locals.var_margin_rv = 0.0;

        let assign41710_e54672: f64 = (1.0 / locals.var_xi);
        locals.var_inv_xi = assign41710_e54672;
        locals.var_inv_xi_dn4 = (-(locals.var_xi_dn4 / (locals.var_xi * locals.var_xi)));
        locals.var_inv_xi_dn6 = (-(locals.var_xi_dn6 / (locals.var_xi * locals.var_xi)));
        locals.var_inv_xi_dn7 = (-(locals.var_xi_dn7 / (locals.var_xi * locals.var_xi)));
        locals.var_inv_xi_dn8 = (-(locals.var_xi_dn8 / (locals.var_xi * locals.var_xi)));
        locals.var_inv_xi_dn9 = (-(locals.var_xi_dn9 / (locals.var_xi * locals.var_xi)));
        locals.var_inv_xi_rv = 0.0;

        locals.var_sp_s_x1 = 0.0;
        locals.var_sp_s_x1_dn4 = 0.0;
        locals.var_sp_s_x1_dn6 = 0.0;
        locals.var_sp_s_x1_dn7 = 0.0;
        locals.var_sp_s_x1_dn8 = 0.0;
        locals.var_sp_s_x1_dn9 = 0.0;
        locals.var_sp_s_x1_rv = 0.0;

        locals.var_x_s = 0.0;
        locals.var_x_s_dn4 = 0.0;
        locals.var_x_s_dn6 = 0.0;
        locals.var_x_s_dn7 = 0.0;
        locals.var_x_s_dn8 = 0.0;
        locals.var_x_s_dn9 = 0.0;
        locals.var_x_s_rv = 0.0;

        let assign41740_e54677: f64 = if locals.var_xn_s < 460.51701859880916 { 1.0 } else { 0.0 };
        locals.var_guard1198 = assign41740_e54677;
        locals.var_guard1198_rv = 0.0;

        let (assign41750_e54683, assign41750_e54683_d_n4, assign41750_e54683_d_n6, assign41750_e54683_d_n7, assign41750_e54683_d_n8, assign41750_e54683_d_n9,) = {
    if (locals.var_guard1198 != 0.0) {
        let assign41750_e54680: f64 = (-locals.var_xn_s);
        let assign41750_e54681: f64 = (assign41750_e54680).exp();
        (assign41750_e54681, (assign41750_e54681 * (-locals.var_xn_s_dn4)), (assign41750_e54681 * (-locals.var_xn_s_dn6)), (assign41750_e54681 * (-locals.var_xn_s_dn7)), (assign41750_e54681 * (-locals.var_xn_s_dn8)), (assign41750_e54681 * (-locals.var_xn_s_dn9)),)
    } else {
        (locals.var_delta_ns, locals.var_delta_ns_dn4, locals.var_delta_ns_dn6, locals.var_delta_ns_dn7, locals.var_delta_ns_dn8, locals.var_delta_ns_dn9,)
    }
};
        locals.var_delta_ns = assign41750_e54683;
        locals.var_delta_ns_dn4 = assign41750_e54683_d_n4;
        locals.var_delta_ns_dn6 = assign41750_e54683_d_n6;
        locals.var_delta_ns_dn7 = assign41750_e54683_d_n7;
        locals.var_delta_ns_dn8 = assign41750_e54683_d_n8;
        locals.var_delta_ns_dn9 = assign41750_e54683_d_n9;
        locals.var_delta_ns_rv = 0.0;

        let (assign41760_e54710, assign41760_e54710_d_n4, assign41760_e54710_d_n6, assign41760_e54710_d_n7, assign41760_e54710_d_n8, assign41760_e54710_d_n9,) = {
    if (locals.var_guard1198 == 0.0) {
        let assign41760_e54690: f64 = (locals.var_xn_s - 460.51701859880916);
        let assign41760_e54695: f64 = (locals.var_xn_s - 460.51701859880916);
        let assign41760_e54699: f64 = (locals.var_xn_s - 460.51701859880916);
        let assign41760_e54701: f64 = (assign41760_e54699 * 0.3333333333333333);
        let assign41760_e54702: f64 = (1.0 + assign41760_e54701);
        let assign41760_e54703: f64 = (assign41760_e54695 * assign41760_e54702);
        let assign41760_e54704: f64 = (0.5 * assign41760_e54703);
        let assign41760_e54705: f64 = (1.0 + assign41760_e54704);
        let assign41760_e54706: f64 = (assign41760_e54690 * assign41760_e54705);
        let assign41760_e54707: f64 = (1.0 + assign41760_e54706);
        let assign41760_e54708: f64 = (1e-200 / assign41760_e54707);
        (assign41760_e54708, (-((1e-200 * ((locals.var_xn_s_dn4 * assign41760_e54705) + (assign41760_e54690 * (0.5 * ((locals.var_xn_s_dn4 * assign41760_e54702) + (assign41760_e54695 * (locals.var_xn_s_dn4 * 0.3333333333333333))))))) / (assign41760_e54707 * assign41760_e54707))), (-((1e-200 * ((locals.var_xn_s_dn6 * assign41760_e54705) + (assign41760_e54690 * (0.5 * ((locals.var_xn_s_dn6 * assign41760_e54702) + (assign41760_e54695 * (locals.var_xn_s_dn6 * 0.3333333333333333))))))) / (assign41760_e54707 * assign41760_e54707))), (-((1e-200 * ((locals.var_xn_s_dn7 * assign41760_e54705) + (assign41760_e54690 * (0.5 * ((locals.var_xn_s_dn7 * assign41760_e54702) + (assign41760_e54695 * (locals.var_xn_s_dn7 * 0.3333333333333333))))))) / (assign41760_e54707 * assign41760_e54707))), (-((1e-200 * ((locals.var_xn_s_dn8 * assign41760_e54705) + (assign41760_e54690 * (0.5 * ((locals.var_xn_s_dn8 * assign41760_e54702) + (assign41760_e54695 * (locals.var_xn_s_dn8 * 0.3333333333333333))))))) / (assign41760_e54707 * assign41760_e54707))), (-((1e-200 * ((locals.var_xn_s_dn9 * assign41760_e54705) + (assign41760_e54690 * (0.5 * ((locals.var_xn_s_dn9 * assign41760_e54702) + (assign41760_e54695 * (locals.var_xn_s_dn9 * 0.3333333333333333))))))) / (assign41760_e54707 * assign41760_e54707))),)
    } else {
        (locals.var_delta_ns, locals.var_delta_ns_dn4, locals.var_delta_ns_dn6, locals.var_delta_ns_dn7, locals.var_delta_ns_dn8, locals.var_delta_ns_dn9,)
    }
};
        locals.var_delta_ns = assign41760_e54710;
        locals.var_delta_ns_dn4 = assign41760_e54710_d_n4;
        locals.var_delta_ns_dn6 = assign41760_e54710_d_n6;
        locals.var_delta_ns_dn7 = assign41760_e54710_d_n7;
        locals.var_delta_ns_dn8 = assign41760_e54710_d_n8;
        locals.var_delta_ns_dn9 = assign41760_e54710_d_n9;
        locals.var_delta_ns_rv = 0.0;

        let assign41770_e54712: f64 = (locals.var_xg).abs();
        let assign41770_e54714: f64 = if assign41770_e54712 <= locals.var_margin { 1.0 } else { 0.0 };
        locals.var_guard1199 = assign41770_e54714;
        locals.var_guard1199_rv = 0.0;

        let (assign41780_e54724, assign41780_e54724_d_n4, assign41780_e54724_d_n6, assign41780_e54724_d_n7, assign41780_e54724_d_n8, assign41780_e54724_d_n9,) = {
    if (locals.var_guard1199 != 0.0) {
        let assign41780_e54718: f64 = (locals.var_inv_xi * locals.var_inv_xi);
        let assign41780_e54720: f64 = (assign41780_e54718 * 0.16666666666666666);
        let assign41780_e54722: f64 = (assign41780_e54720 * 0.7071067811865475);
        (assign41780_e54722, ((((locals.var_inv_xi_dn4 * locals.var_inv_xi) + (locals.var_inv_xi * locals.var_inv_xi_dn4)) * 0.16666666666666666) * 0.7071067811865475), ((((locals.var_inv_xi_dn6 * locals.var_inv_xi) + (locals.var_inv_xi * locals.var_inv_xi_dn6)) * 0.16666666666666666) * 0.7071067811865475), ((((locals.var_inv_xi_dn7 * locals.var_inv_xi) + (locals.var_inv_xi * locals.var_inv_xi_dn7)) * 0.16666666666666666) * 0.7071067811865475), ((((locals.var_inv_xi_dn8 * locals.var_inv_xi) + (locals.var_inv_xi * locals.var_inv_xi_dn8)) * 0.16666666666666666) * 0.7071067811865475), ((((locals.var_inv_xi_dn9 * locals.var_inv_xi) + (locals.var_inv_xi * locals.var_inv_xi_dn9)) * 0.16666666666666666) * 0.7071067811865475),)
    } else {
        (locals.var_sp_s_temp1, locals.var_sp_s_temp1_dn4, locals.var_sp_s_temp1_dn6, locals.var_sp_s_temp1_dn7, locals.var_sp_s_temp1_dn8, locals.var_sp_s_temp1_dn9,)
    }
};
        locals.var_sp_s_temp1 = assign41780_e54724;
        locals.var_sp_s_temp1_dn4 = assign41780_e54724_d_n4;
        locals.var_sp_s_temp1_dn6 = assign41780_e54724_d_n6;
        locals.var_sp_s_temp1_dn7 = assign41780_e54724_d_n7;
        locals.var_sp_s_temp1_dn8 = assign41780_e54724_d_n8;
        locals.var_sp_s_temp1_dn9 = assign41780_e54724_d_n9;
        locals.var_sp_s_temp1_rv = 0.0;

        let (assign41790_e54742, assign41790_e54742_d_n4, assign41790_e54742_d_n6, assign41790_e54742_d_n7, assign41790_e54742_d_n8, assign41790_e54742_d_n9,) = {
    if (locals.var_guard1199 != 0.0) {
        let assign41790_e54728: f64 = (locals.var_xg * locals.var_inv_xi);
        let assign41790_e54733: f64 = (1.0 - locals.var_delta_ns);
        let assign41790_e54734: f64 = (locals.var_xg * assign41790_e54733);
        let assign41790_e54736: f64 = (assign41790_e54734 * locals.var_gf);
        let assign41790_e54738: f64 = (assign41790_e54736 * locals.var_sp_s_temp1);
        let assign41790_e54739: f64 = (1.0 + assign41790_e54738);
        let assign41790_e54740: f64 = (assign41790_e54728 * assign41790_e54739);
        (assign41790_e54740, ((((locals.var_xg_dn4 * locals.var_inv_xi) + (locals.var_xg * locals.var_inv_xi_dn4)) * assign41790_e54739) + (assign41790_e54728 * ((((((locals.var_xg_dn4 * assign41790_e54733) + (locals.var_xg * (-locals.var_delta_ns_dn4))) * locals.var_gf) + (assign41790_e54734 * locals.var_gf_dn4)) * locals.var_sp_s_temp1) + (assign41790_e54736 * locals.var_sp_s_temp1_dn4)))), ((((locals.var_xg_dn6 * locals.var_inv_xi) + (locals.var_xg * locals.var_inv_xi_dn6)) * assign41790_e54739) + (assign41790_e54728 * ((((((locals.var_xg_dn6 * assign41790_e54733) + (locals.var_xg * (-locals.var_delta_ns_dn6))) * locals.var_gf) + (assign41790_e54734 * locals.var_gf_dn6)) * locals.var_sp_s_temp1) + (assign41790_e54736 * locals.var_sp_s_temp1_dn6)))), ((((locals.var_xg_dn7 * locals.var_inv_xi) + (locals.var_xg * locals.var_inv_xi_dn7)) * assign41790_e54739) + (assign41790_e54728 * ((((((locals.var_xg_dn7 * assign41790_e54733) + (locals.var_xg * (-locals.var_delta_ns_dn7))) * locals.var_gf) + (assign41790_e54734 * locals.var_gf_dn7)) * locals.var_sp_s_temp1) + (assign41790_e54736 * locals.var_sp_s_temp1_dn7)))), ((((locals.var_xg_dn8 * locals.var_inv_xi) + (locals.var_xg * locals.var_inv_xi_dn8)) * assign41790_e54739) + (assign41790_e54728 * ((((((locals.var_xg_dn8 * assign41790_e54733) + (locals.var_xg * (-locals.var_delta_ns_dn8))) * locals.var_gf) + (assign41790_e54734 * locals.var_gf_dn8)) * locals.var_sp_s_temp1) + (assign41790_e54736 * locals.var_sp_s_temp1_dn8)))), ((((locals.var_xg_dn9 * locals.var_inv_xi) + (locals.var_xg * locals.var_inv_xi_dn9)) * assign41790_e54739) + (assign41790_e54728 * ((((((locals.var_xg_dn9 * assign41790_e54733) + (locals.var_xg * (-locals.var_delta_ns_dn9))) * locals.var_gf) + (assign41790_e54734 * locals.var_gf_dn9)) * locals.var_sp_s_temp1) + (assign41790_e54736 * locals.var_sp_s_temp1_dn9)))),)
    } else {
        (locals.var_x_s, locals.var_x_s_dn4, locals.var_x_s_dn6, locals.var_x_s_dn7, locals.var_x_s_dn8, locals.var_x_s_dn9,)
    }
};
        locals.var_x_s = assign41790_e54742;
        locals.var_x_s_dn4 = assign41790_e54742_d_n4;
        locals.var_x_s_dn6 = assign41790_e54742_d_n6;
        locals.var_x_s_dn7 = assign41790_e54742_d_n7;
        locals.var_x_s_dn8 = assign41790_e54742_d_n8;
        locals.var_x_s_dn9 = assign41790_e54742_d_n9;
        locals.var_x_s_rv = 0.0;

        let assign41800_e54745: f64 = (-locals.var_margin);
        let assign41800_e54746: f64 = if locals.var_xg < assign41800_e54745 { 1.0 } else { 0.0 };
        locals.var_guard1200 = assign41800_e54746;
        locals.var_guard1200_rv = 0.0;

        let (assign41810_e54754, assign41810_e54754_d_n4, assign41810_e54754_d_n6, assign41810_e54754_d_n7, assign41810_e54754_d_n8, assign41810_e54754_d_n9,) = {
    if ((locals.var_guard1199 == 0.0) && (locals.var_guard1200 != 0.0)) {
        let assign41810_e54752: f64 = (-locals.var_xg);
        (assign41810_e54752, (-locals.var_xg_dn4), (-locals.var_xg_dn6), (-locals.var_xg_dn7), (-locals.var_xg_dn8), (-locals.var_xg_dn9),)
    } else {
        (locals.var_sp_s_yg, locals.var_sp_s_yg_dn4, locals.var_sp_s_yg_dn6, locals.var_sp_s_yg_dn7, locals.var_sp_s_yg_dn8, locals.var_sp_s_yg_dn9,)
    }
};
        locals.var_sp_s_yg = assign41810_e54754;
        locals.var_sp_s_yg_dn4 = assign41810_e54754_d_n4;
        locals.var_sp_s_yg_dn6 = assign41810_e54754_d_n6;
        locals.var_sp_s_yg_dn7 = assign41810_e54754_d_n7;
        locals.var_sp_s_yg_dn8 = assign41810_e54754_d_n8;
        locals.var_sp_s_yg_dn9 = assign41810_e54754_d_n9;
        locals.var_sp_s_yg_rv = 0.0;

        let (assign41820_e54765, assign41820_e54765_d_n4, assign41820_e54765_d_n6, assign41820_e54765_d_n7, assign41820_e54765_d_n8, assign41820_e54765_d_n9,) = {
    if ((locals.var_guard1199 == 0.0) && (locals.var_guard1200 != 0.0)) {
        let assign41820_e54762: f64 = (locals.var_sp_s_yg * locals.var_inv_xi);
        let assign41820_e54763: f64 = (1.25 * assign41820_e54762);
        (assign41820_e54763, (1.25 * ((locals.var_sp_s_yg_dn4 * locals.var_inv_xi) + (locals.var_sp_s_yg * locals.var_inv_xi_dn4))), (1.25 * ((locals.var_sp_s_yg_dn6 * locals.var_inv_xi) + (locals.var_sp_s_yg * locals.var_inv_xi_dn6))), (1.25 * ((locals.var_sp_s_yg_dn7 * locals.var_inv_xi) + (locals.var_sp_s_yg * locals.var_inv_xi_dn7))), (1.25 * ((locals.var_sp_s_yg_dn8 * locals.var_inv_xi) + (locals.var_sp_s_yg * locals.var_inv_xi_dn8))), (1.25 * ((locals.var_sp_s_yg_dn9 * locals.var_inv_xi) + (locals.var_sp_s_yg * locals.var_inv_xi_dn9))),)
    } else {
        (locals.var_sp_s_ysub, locals.var_sp_s_ysub_dn4, locals.var_sp_s_ysub_dn6, locals.var_sp_s_ysub_dn7, locals.var_sp_s_ysub_dn8, locals.var_sp_s_ysub_dn9,)
    }
};
        locals.var_sp_s_ysub = assign41820_e54765;
        locals.var_sp_s_ysub_dn4 = assign41820_e54765_d_n4;
        locals.var_sp_s_ysub_dn6 = assign41820_e54765_d_n6;
        locals.var_sp_s_ysub_dn7 = assign41820_e54765_d_n7;
        locals.var_sp_s_ysub_dn8 = assign41820_e54765_d_n8;
        locals.var_sp_s_ysub_dn9 = assign41820_e54765_d_n9;
        locals.var_sp_s_ysub_rv = 0.0;

        let (assign41830_e54787, assign41830_e54787_d_n4, assign41830_e54787_d_n6, assign41830_e54787_d_n7, assign41830_e54787_d_n8, assign41830_e54787_d_n9,) = {
    if ((locals.var_guard1199 == 0.0) && (locals.var_guard1200 != 0.0)) {
        let assign41830_e54773: f64 = (locals.var_sp_s_ysub + 10.0);
        let assign41830_e54776: f64 = (locals.var_sp_s_ysub - 6.0);
        let assign41830_e54779: f64 = (locals.var_sp_s_ysub - 6.0);
        let assign41830_e54780: f64 = (assign41830_e54776 * assign41830_e54779);
        let assign41830_e54782: f64 = (assign41830_e54780 + 64.0);
        let assign41830_e54783: f64 = (assign41830_e54782).sqrt();
        let assign41830_e54784: f64 = (assign41830_e54773 - assign41830_e54783);
        let assign41830_e54785: f64 = (0.5 * assign41830_e54784);
        (assign41830_e54785, (0.5 * (locals.var_sp_s_ysub_dn4 - (((locals.var_sp_s_ysub_dn4 * assign41830_e54779) + (assign41830_e54776 * locals.var_sp_s_ysub_dn4)) / (2.0 * assign41830_e54783)))), (0.5 * (locals.var_sp_s_ysub_dn6 - (((locals.var_sp_s_ysub_dn6 * assign41830_e54779) + (assign41830_e54776 * locals.var_sp_s_ysub_dn6)) / (2.0 * assign41830_e54783)))), (0.5 * (locals.var_sp_s_ysub_dn7 - (((locals.var_sp_s_ysub_dn7 * assign41830_e54779) + (assign41830_e54776 * locals.var_sp_s_ysub_dn7)) / (2.0 * assign41830_e54783)))), (0.5 * (locals.var_sp_s_ysub_dn8 - (((locals.var_sp_s_ysub_dn8 * assign41830_e54779) + (assign41830_e54776 * locals.var_sp_s_ysub_dn8)) / (2.0 * assign41830_e54783)))), (0.5 * (locals.var_sp_s_ysub_dn9 - (((locals.var_sp_s_ysub_dn9 * assign41830_e54779) + (assign41830_e54776 * locals.var_sp_s_ysub_dn9)) / (2.0 * assign41830_e54783)))),)
    } else {
        (locals.var_sp_s_eta, locals.var_sp_s_eta_dn4, locals.var_sp_s_eta_dn6, locals.var_sp_s_eta_dn7, locals.var_sp_s_eta_dn8, locals.var_sp_s_eta_dn9,)
    }
};
        locals.var_sp_s_eta = assign41830_e54787;
        locals.var_sp_s_eta_dn4 = assign41830_e54787_d_n4;
        locals.var_sp_s_eta_dn6 = assign41830_e54787_d_n6;
        locals.var_sp_s_eta_dn7 = assign41830_e54787_d_n7;
        locals.var_sp_s_eta_dn8 = assign41830_e54787_d_n8;
        locals.var_sp_s_eta_dn9 = assign41830_e54787_d_n9;
        locals.var_sp_s_eta_rv = 0.0;

        let (assign41840_e54796, assign41840_e54796_d_n4, assign41840_e54796_d_n6, assign41840_e54796_d_n7, assign41840_e54796_d_n8, assign41840_e54796_d_n9,) = {
    if ((locals.var_guard1199 == 0.0) && (locals.var_guard1200 != 0.0)) {
        let assign41840_e54794: f64 = (locals.var_sp_s_yg - locals.var_sp_s_eta);
        (assign41840_e54794, (locals.var_sp_s_yg_dn4 - locals.var_sp_s_eta_dn4), (locals.var_sp_s_yg_dn6 - locals.var_sp_s_eta_dn6), (locals.var_sp_s_yg_dn7 - locals.var_sp_s_eta_dn7), (locals.var_sp_s_yg_dn8 - locals.var_sp_s_eta_dn8), (locals.var_sp_s_yg_dn9 - locals.var_sp_s_eta_dn9),)
    } else {
        (locals.var_sp_s_temp, locals.var_sp_s_temp_dn4, locals.var_sp_s_temp_dn6, locals.var_sp_s_temp_dn7, locals.var_sp_s_temp_dn8, locals.var_sp_s_temp_dn9,)
    }
};
        locals.var_sp_s_temp = assign41840_e54796;
        locals.var_sp_s_temp_dn4 = assign41840_e54796_d_n4;
        locals.var_sp_s_temp_dn6 = assign41840_e54796_d_n6;
        locals.var_sp_s_temp_dn7 = assign41840_e54796_d_n7;
        locals.var_sp_s_temp_dn8 = assign41840_e54796_d_n8;
        locals.var_sp_s_temp_dn9 = assign41840_e54796_d_n9;
        locals.var_sp_s_temp_rv = 0.0;

        let (assign41850_e54811, assign41850_e54811_d_n4, assign41850_e54811_d_n6, assign41850_e54811_d_n7, assign41850_e54811_d_n8, assign41850_e54811_d_n9,) = {
    if ((locals.var_guard1199 == 0.0) && (locals.var_guard1200 != 0.0)) {
        let assign41850_e54803: f64 = (locals.var_sp_s_temp * locals.var_sp_s_temp);
        let assign41850_e54807: f64 = (locals.var_sp_s_eta + 1.0);
        let assign41850_e54808: f64 = (locals.var_gf2 * assign41850_e54807);
        let assign41850_e54809: f64 = (assign41850_e54803 + assign41850_e54808);
        (assign41850_e54809, (((locals.var_sp_s_temp_dn4 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn4)) + ((locals.var_gf2_dn4 * assign41850_e54807) + (locals.var_gf2 * locals.var_sp_s_eta_dn4))), (((locals.var_sp_s_temp_dn6 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn6)) + ((locals.var_gf2_dn6 * assign41850_e54807) + (locals.var_gf2 * locals.var_sp_s_eta_dn6))), (((locals.var_sp_s_temp_dn7 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn7)) + ((locals.var_gf2_dn7 * assign41850_e54807) + (locals.var_gf2 * locals.var_sp_s_eta_dn7))), (((locals.var_sp_s_temp_dn8 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn8)) + ((locals.var_gf2_dn8 * assign41850_e54807) + (locals.var_gf2 * locals.var_sp_s_eta_dn8))), (((locals.var_sp_s_temp_dn9 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn9)) + ((locals.var_gf2_dn9 * assign41850_e54807) + (locals.var_gf2 * locals.var_sp_s_eta_dn9))),)
    } else {
        (locals.var_sp_s_a, locals.var_sp_s_a_dn4, locals.var_sp_s_a_dn6, locals.var_sp_s_a_dn7, locals.var_sp_s_a_dn8, locals.var_sp_s_a_dn9,)
    }
};
        locals.var_sp_s_a = assign41850_e54811;
        locals.var_sp_s_a_dn4 = assign41850_e54811_d_n4;
        locals.var_sp_s_a_dn6 = assign41850_e54811_d_n6;
        locals.var_sp_s_a_dn7 = assign41850_e54811_d_n7;
        locals.var_sp_s_a_dn8 = assign41850_e54811_d_n8;
        locals.var_sp_s_a_dn9 = assign41850_e54811_d_n9;
        locals.var_sp_s_a_rv = 0.0;

        let (assign41860_e54822, assign41860_e54822_d_n4, assign41860_e54822_d_n6, assign41860_e54822_d_n7, assign41860_e54822_d_n8, assign41860_e54822_d_n9,) = {
    if ((locals.var_guard1199 == 0.0) && (locals.var_guard1200 != 0.0)) {
        let assign41860_e54818: f64 = (2.0 * locals.var_sp_s_temp);
        let assign41860_e54820: f64 = (assign41860_e54818 - locals.var_gf2);
        (assign41860_e54820, ((2.0 * locals.var_sp_s_temp_dn4) - locals.var_gf2_dn4), ((2.0 * locals.var_sp_s_temp_dn6) - locals.var_gf2_dn6), ((2.0 * locals.var_sp_s_temp_dn7) - locals.var_gf2_dn7), ((2.0 * locals.var_sp_s_temp_dn8) - locals.var_gf2_dn8), ((2.0 * locals.var_sp_s_temp_dn9) - locals.var_gf2_dn9),)
    } else {
        (locals.var_sp_s_c, locals.var_sp_s_c_dn4, locals.var_sp_s_c_dn6, locals.var_sp_s_c_dn7, locals.var_sp_s_c_dn8, locals.var_sp_s_c_dn9,)
    }
};
        locals.var_sp_s_c = assign41860_e54822;
        locals.var_sp_s_c_dn4 = assign41860_e54822_d_n4;
        locals.var_sp_s_c_dn6 = assign41860_e54822_d_n6;
        locals.var_sp_s_c_dn7 = assign41860_e54822_d_n7;
        locals.var_sp_s_c_dn8 = assign41860_e54822_d_n8;
        locals.var_sp_s_c_dn9 = assign41860_e54822_d_n9;
        locals.var_sp_s_c_rv = 0.0;

        let (assign41870_e54835, assign41870_e54835_d_n4, assign41870_e54835_d_n6, assign41870_e54835_d_n7, assign41870_e54835_d_n8, assign41870_e54835_d_n9,) = {
    if ((locals.var_guard1199 == 0.0) && (locals.var_guard1200 != 0.0)) {
        let assign41870_e54828: f64 = (-locals.var_sp_s_eta);
        let assign41870_e54831: f64 = (locals.var_sp_s_a * locals.var_inv_gf2);
        let assign41870_e54832: f64 = (assign41870_e54831).ln();
        let assign41870_e54833: f64 = (assign41870_e54828 + assign41870_e54832);
        (assign41870_e54833, ((-locals.var_sp_s_eta_dn4) + (((locals.var_sp_s_a_dn4 * locals.var_inv_gf2) + (locals.var_sp_s_a * locals.var_inv_gf2_dn4)) / assign41870_e54831)), ((-locals.var_sp_s_eta_dn6) + (((locals.var_sp_s_a_dn6 * locals.var_inv_gf2) + (locals.var_sp_s_a * locals.var_inv_gf2_dn6)) / assign41870_e54831)), ((-locals.var_sp_s_eta_dn7) + (((locals.var_sp_s_a_dn7 * locals.var_inv_gf2) + (locals.var_sp_s_a * locals.var_inv_gf2_dn7)) / assign41870_e54831)), ((-locals.var_sp_s_eta_dn8) + (((locals.var_sp_s_a_dn8 * locals.var_inv_gf2) + (locals.var_sp_s_a * locals.var_inv_gf2_dn8)) / assign41870_e54831)), ((-locals.var_sp_s_eta_dn9) + (((locals.var_sp_s_a_dn9 * locals.var_inv_gf2) + (locals.var_sp_s_a * locals.var_inv_gf2_dn9)) / assign41870_e54831)),)
    } else {
        (locals.var_sp_s_tau, locals.var_sp_s_tau_dn4, locals.var_sp_s_tau_dn6, locals.var_sp_s_tau_dn7, locals.var_sp_s_tau_dn8, locals.var_sp_s_tau_dn9,)
    }
};
        locals.var_sp_s_tau = assign41870_e54835;
        locals.var_sp_s_tau_dn4 = assign41870_e54835_d_n4;
        locals.var_sp_s_tau_dn6 = assign41870_e54835_d_n6;
        locals.var_sp_s_tau_dn7 = assign41870_e54835_d_n7;
        locals.var_sp_s_tau_dn8 = assign41870_e54835_d_n8;
        locals.var_sp_s_tau_dn9 = assign41870_e54835_d_n9;
        locals.var_sp_s_tau_rv = 0.0;

        let (assign41880_e54844, assign41880_e54844_d_n4, assign41880_e54844_d_n6, assign41880_e54844_d_n7, assign41880_e54844_d_n8, assign41880_e54844_d_n9,) = {
    if ((locals.var_guard1199 == 0.0) && (locals.var_guard1200 != 0.0)) {
        let assign41880_e54842: f64 = (locals.var_sp_s_a + locals.var_sp_s_c);
        (assign41880_e54842, (locals.var_sp_s_a_dn4 + locals.var_sp_s_c_dn4), (locals.var_sp_s_a_dn6 + locals.var_sp_s_c_dn6), (locals.var_sp_s_a_dn7 + locals.var_sp_s_c_dn7), (locals.var_sp_s_a_dn8 + locals.var_sp_s_c_dn8), (locals.var_sp_s_a_dn9 + locals.var_sp_s_c_dn9),)
    } else {
        (locals.var_nu, locals.var_nu_dn4, locals.var_nu_dn6, locals.var_nu_dn7, locals.var_nu_dn8, locals.var_nu_dn9,)
    }
};
        locals.var_nu = assign41880_e54844;
        locals.var_nu_dn4 = assign41880_e54844_d_n4;
        locals.var_nu_dn6 = assign41880_e54844_d_n6;
        locals.var_nu_dn7 = assign41880_e54844_d_n7;
        locals.var_nu_dn8 = assign41880_e54844_d_n8;
        locals.var_nu_dn9 = assign41880_e54844_d_n9;
        locals.var_nu_rv = 0.0;

        let (assign41890_e54863, assign41890_e54863_d_n4, assign41890_e54863_d_n6, assign41890_e54863_d_n7, assign41890_e54863_d_n8, assign41890_e54863_d_n9,) = {
    if ((locals.var_guard1199 == 0.0) && (locals.var_guard1200 != 0.0)) {
        let assign41890_e54851: f64 = (locals.var_nu * locals.var_nu);
        let assign41890_e54856: f64 = (locals.var_sp_s_c * locals.var_sp_s_c);
        let assign41890_e54857: f64 = (0.5 * assign41890_e54856);
        let assign41890_e54859: f64 = (assign41890_e54857 - locals.var_sp_s_a);
        let assign41890_e54860: f64 = (locals.var_sp_s_tau * assign41890_e54859);
        let assign41890_e54861: f64 = (assign41890_e54851 + assign41890_e54860);
        (assign41890_e54861, (((locals.var_nu_dn4 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn4)) + ((locals.var_sp_s_tau_dn4 * assign41890_e54859) + (locals.var_sp_s_tau * ((0.5 * ((locals.var_sp_s_c_dn4 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn4))) - locals.var_sp_s_a_dn4)))), (((locals.var_nu_dn6 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn6)) + ((locals.var_sp_s_tau_dn6 * assign41890_e54859) + (locals.var_sp_s_tau * ((0.5 * ((locals.var_sp_s_c_dn6 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn6))) - locals.var_sp_s_a_dn6)))), (((locals.var_nu_dn7 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn7)) + ((locals.var_sp_s_tau_dn7 * assign41890_e54859) + (locals.var_sp_s_tau * ((0.5 * ((locals.var_sp_s_c_dn7 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn7))) - locals.var_sp_s_a_dn7)))), (((locals.var_nu_dn8 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn8)) + ((locals.var_sp_s_tau_dn8 * assign41890_e54859) + (locals.var_sp_s_tau * ((0.5 * ((locals.var_sp_s_c_dn8 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn8))) - locals.var_sp_s_a_dn8)))), (((locals.var_nu_dn9 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn9)) + ((locals.var_sp_s_tau_dn9 * assign41890_e54859) + (locals.var_sp_s_tau * ((0.5 * ((locals.var_sp_s_c_dn9 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn9))) - locals.var_sp_s_a_dn9)))),)
    } else {
        (locals.var_mutau, locals.var_mutau_dn4, locals.var_mutau_dn6, locals.var_mutau_dn7, locals.var_mutau_dn8, locals.var_mutau_dn9,)
    }
};
        locals.var_mutau = assign41890_e54863;
        locals.var_mutau_dn4 = assign41890_e54863_d_n4;
        locals.var_mutau_dn6 = assign41890_e54863_d_n6;
        locals.var_mutau_dn7 = assign41890_e54863_d_n7;
        locals.var_mutau_dn8 = assign41890_e54863_d_n8;
        locals.var_mutau_dn9 = assign41890_e54863_d_n9;
        locals.var_mutau_rv = 0.0;

        let (assign41900_e54896, assign41900_e54896_d_n4, assign41900_e54896_d_n6, assign41900_e54896_d_n7, assign41900_e54896_d_n8, assign41900_e54896_d_n9,) = {
    if ((locals.var_guard1199 == 0.0) && (locals.var_guard1200 != 0.0)) {
        let assign41900_e54871: f64 = (locals.var_sp_s_a * locals.var_nu);
        let assign41900_e54873: f64 = (assign41900_e54871 * locals.var_sp_s_tau);
        let assign41900_e54877: f64 = (locals.var_nu / locals.var_mutau);
        let assign41900_e54879: f64 = (assign41900_e54877 * locals.var_sp_s_tau);
        let assign41900_e54881: f64 = (assign41900_e54879 * locals.var_sp_s_tau);
        let assign41900_e54883: f64 = (assign41900_e54881 * locals.var_sp_s_c);
        let assign41900_e54886: f64 = (locals.var_sp_s_c * locals.var_sp_s_c);
        let assign41900_e54888: f64 = (assign41900_e54886 * 0.3333333333333333);
        let assign41900_e54890: f64 = (assign41900_e54888 - locals.var_sp_s_a);
        let assign41900_e54891: f64 = (assign41900_e54883 * assign41900_e54890);
        let assign41900_e54892: f64 = (locals.var_mutau + assign41900_e54891);
        let assign41900_e54893: f64 = (assign41900_e54873 / assign41900_e54892);
        let assign41900_e54894: f64 = (locals.var_sp_s_eta + assign41900_e54893);
        (assign41900_e54894, (locals.var_sp_s_eta_dn4 + (((((((locals.var_sp_s_a_dn4 * locals.var_nu) + (locals.var_sp_s_a * locals.var_nu_dn4)) * locals.var_sp_s_tau) + (assign41900_e54871 * locals.var_sp_s_tau_dn4)) * assign41900_e54892) - (assign41900_e54873 * (locals.var_mutau_dn4 + (((((((((((locals.var_nu_dn4 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn4)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau) + (assign41900_e54877 * locals.var_sp_s_tau_dn4)) * locals.var_sp_s_tau) + (assign41900_e54879 * locals.var_sp_s_tau_dn4)) * locals.var_sp_s_c) + (assign41900_e54881 * locals.var_sp_s_c_dn4)) * assign41900_e54890) + (assign41900_e54883 * ((((locals.var_sp_s_c_dn4 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn4)) * 0.3333333333333333) - locals.var_sp_s_a_dn4)))))) / (assign41900_e54892 * assign41900_e54892))), (locals.var_sp_s_eta_dn6 + (((((((locals.var_sp_s_a_dn6 * locals.var_nu) + (locals.var_sp_s_a * locals.var_nu_dn6)) * locals.var_sp_s_tau) + (assign41900_e54871 * locals.var_sp_s_tau_dn6)) * assign41900_e54892) - (assign41900_e54873 * (locals.var_mutau_dn6 + (((((((((((locals.var_nu_dn6 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn6)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau) + (assign41900_e54877 * locals.var_sp_s_tau_dn6)) * locals.var_sp_s_tau) + (assign41900_e54879 * locals.var_sp_s_tau_dn6)) * locals.var_sp_s_c) + (assign41900_e54881 * locals.var_sp_s_c_dn6)) * assign41900_e54890) + (assign41900_e54883 * ((((locals.var_sp_s_c_dn6 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn6)) * 0.3333333333333333) - locals.var_sp_s_a_dn6)))))) / (assign41900_e54892 * assign41900_e54892))), (locals.var_sp_s_eta_dn7 + (((((((locals.var_sp_s_a_dn7 * locals.var_nu) + (locals.var_sp_s_a * locals.var_nu_dn7)) * locals.var_sp_s_tau) + (assign41900_e54871 * locals.var_sp_s_tau_dn7)) * assign41900_e54892) - (assign41900_e54873 * (locals.var_mutau_dn7 + (((((((((((locals.var_nu_dn7 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn7)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau) + (assign41900_e54877 * locals.var_sp_s_tau_dn7)) * locals.var_sp_s_tau) + (assign41900_e54879 * locals.var_sp_s_tau_dn7)) * locals.var_sp_s_c) + (assign41900_e54881 * locals.var_sp_s_c_dn7)) * assign41900_e54890) + (assign41900_e54883 * ((((locals.var_sp_s_c_dn7 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn7)) * 0.3333333333333333) - locals.var_sp_s_a_dn7)))))) / (assign41900_e54892 * assign41900_e54892))), (locals.var_sp_s_eta_dn8 + (((((((locals.var_sp_s_a_dn8 * locals.var_nu) + (locals.var_sp_s_a * locals.var_nu_dn8)) * locals.var_sp_s_tau) + (assign41900_e54871 * locals.var_sp_s_tau_dn8)) * assign41900_e54892) - (assign41900_e54873 * (locals.var_mutau_dn8 + (((((((((((locals.var_nu_dn8 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn8)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau) + (assign41900_e54877 * locals.var_sp_s_tau_dn8)) * locals.var_sp_s_tau) + (assign41900_e54879 * locals.var_sp_s_tau_dn8)) * locals.var_sp_s_c) + (assign41900_e54881 * locals.var_sp_s_c_dn8)) * assign41900_e54890) + (assign41900_e54883 * ((((locals.var_sp_s_c_dn8 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn8)) * 0.3333333333333333) - locals.var_sp_s_a_dn8)))))) / (assign41900_e54892 * assign41900_e54892))), (locals.var_sp_s_eta_dn9 + (((((((locals.var_sp_s_a_dn9 * locals.var_nu) + (locals.var_sp_s_a * locals.var_nu_dn9)) * locals.var_sp_s_tau) + (assign41900_e54871 * locals.var_sp_s_tau_dn9)) * assign41900_e54892) - (assign41900_e54873 * (locals.var_mutau_dn9 + (((((((((((locals.var_nu_dn9 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn9)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau) + (assign41900_e54877 * locals.var_sp_s_tau_dn9)) * locals.var_sp_s_tau) + (assign41900_e54879 * locals.var_sp_s_tau_dn9)) * locals.var_sp_s_c) + (assign41900_e54881 * locals.var_sp_s_c_dn9)) * assign41900_e54890) + (assign41900_e54883 * ((((locals.var_sp_s_c_dn9 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn9)) * 0.3333333333333333) - locals.var_sp_s_a_dn9)))))) / (assign41900_e54892 * assign41900_e54892))),)
    } else {
        (locals.var_sp_s_y0, locals.var_sp_s_y0_dn4, locals.var_sp_s_y0_dn6, locals.var_sp_s_y0_dn7, locals.var_sp_s_y0_dn8, locals.var_sp_s_y0_dn9,)
    }
};
        locals.var_sp_s_y0 = assign41900_e54896;
        locals.var_sp_s_y0_dn4 = assign41900_e54896_d_n4;
        locals.var_sp_s_y0_dn6 = assign41900_e54896_d_n6;
        locals.var_sp_s_y0_dn7 = assign41900_e54896_d_n7;
        locals.var_sp_s_y0_dn8 = assign41900_e54896_d_n8;
        locals.var_sp_s_y0_dn9 = assign41900_e54896_d_n9;
        locals.var_sp_s_y0_rv = 0.0;

        let assign41910_e54899: f64 = if locals.var_sp_s_y0 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard1201 = assign41910_e54899;
        locals.var_guard1201_rv = 0.0;

        let (assign41920_e54909, assign41920_e54909_d_n4, assign41920_e54909_d_n6, assign41920_e54909_d_n7, assign41920_e54909_d_n8, assign41920_e54909_d_n9,) = {
    if (((locals.var_guard1199 == 0.0) && (locals.var_guard1200 != 0.0)) && (locals.var_guard1201 != 0.0)) {
        let assign41920_e54907: f64 = (locals.var_sp_s_y0).exp();
        (assign41920_e54907, (assign41920_e54907 * locals.var_sp_s_y0_dn4), (assign41920_e54907 * locals.var_sp_s_y0_dn6), (assign41920_e54907 * locals.var_sp_s_y0_dn7), (assign41920_e54907 * locals.var_sp_s_y0_dn8), (assign41920_e54907 * locals.var_sp_s_y0_dn9),)
    } else {
        (locals.var_sp_s_delta0, locals.var_sp_s_delta0_dn4, locals.var_sp_s_delta0_dn6, locals.var_sp_s_delta0_dn7, locals.var_sp_s_delta0_dn8, locals.var_sp_s_delta0_dn9,)
    }
};
        locals.var_sp_s_delta0 = assign41920_e54909;
        locals.var_sp_s_delta0_dn4 = assign41920_e54909_d_n4;
        locals.var_sp_s_delta0_dn6 = assign41920_e54909_d_n6;
        locals.var_sp_s_delta0_dn7 = assign41920_e54909_d_n7;
        locals.var_sp_s_delta0_dn8 = assign41920_e54909_d_n8;
        locals.var_sp_s_delta0_dn9 = assign41920_e54909_d_n9;
        locals.var_sp_s_delta0_rv = 0.0;

        let (assign41930_e54941, assign41930_e54941_d_n4, assign41930_e54941_d_n6, assign41930_e54941_d_n7, assign41930_e54941_d_n8, assign41930_e54941_d_n9,) = {
    if (((locals.var_guard1199 == 0.0) && (locals.var_guard1200 != 0.0)) && (locals.var_guard1201 == 0.0)) {
        let assign41930_e54921: f64 = (locals.var_sp_s_y0 - 230.25850929940458);
        let assign41930_e54926: f64 = (locals.var_sp_s_y0 - 230.25850929940458);
        let assign41930_e54930: f64 = (locals.var_sp_s_y0 - 230.25850929940458);
        let assign41930_e54932: f64 = (assign41930_e54930 * 0.3333333333333333);
        let assign41930_e54933: f64 = (1.0 + assign41930_e54932);
        let assign41930_e54934: f64 = (assign41930_e54926 * assign41930_e54933);
        let assign41930_e54935: f64 = (0.5 * assign41930_e54934);
        let assign41930_e54936: f64 = (1.0 + assign41930_e54935);
        let assign41930_e54937: f64 = (assign41930_e54921 * assign41930_e54936);
        let assign41930_e54938: f64 = (1.0 + assign41930_e54937);
        let assign41930_e54939: f64 = (1e100 * assign41930_e54938);
        (assign41930_e54939, (1e100 * ((locals.var_sp_s_y0_dn4 * assign41930_e54936) + (assign41930_e54921 * (0.5 * ((locals.var_sp_s_y0_dn4 * assign41930_e54933) + (assign41930_e54926 * (locals.var_sp_s_y0_dn4 * 0.3333333333333333))))))), (1e100 * ((locals.var_sp_s_y0_dn6 * assign41930_e54936) + (assign41930_e54921 * (0.5 * ((locals.var_sp_s_y0_dn6 * assign41930_e54933) + (assign41930_e54926 * (locals.var_sp_s_y0_dn6 * 0.3333333333333333))))))), (1e100 * ((locals.var_sp_s_y0_dn7 * assign41930_e54936) + (assign41930_e54921 * (0.5 * ((locals.var_sp_s_y0_dn7 * assign41930_e54933) + (assign41930_e54926 * (locals.var_sp_s_y0_dn7 * 0.3333333333333333))))))), (1e100 * ((locals.var_sp_s_y0_dn8 * assign41930_e54936) + (assign41930_e54921 * (0.5 * ((locals.var_sp_s_y0_dn8 * assign41930_e54933) + (assign41930_e54926 * (locals.var_sp_s_y0_dn8 * 0.3333333333333333))))))), (1e100 * ((locals.var_sp_s_y0_dn9 * assign41930_e54936) + (assign41930_e54921 * (0.5 * ((locals.var_sp_s_y0_dn9 * assign41930_e54933) + (assign41930_e54926 * (locals.var_sp_s_y0_dn9 * 0.3333333333333333))))))),)
    } else {
        (locals.var_sp_s_delta0, locals.var_sp_s_delta0_dn4, locals.var_sp_s_delta0_dn6, locals.var_sp_s_delta0_dn7, locals.var_sp_s_delta0_dn8, locals.var_sp_s_delta0_dn9,)
    }
};
        locals.var_sp_s_delta0 = assign41930_e54941;
        locals.var_sp_s_delta0_dn4 = assign41930_e54941_d_n4;
        locals.var_sp_s_delta0_dn6 = assign41930_e54941_d_n6;
        locals.var_sp_s_delta0_dn7 = assign41930_e54941_d_n7;
        locals.var_sp_s_delta0_dn8 = assign41930_e54941_d_n8;
        locals.var_sp_s_delta0_dn9 = assign41930_e54941_d_n9;
        locals.var_sp_s_delta0_rv = 0.0;

        let (assign41940_e54950, assign41940_e54950_d_n4, assign41940_e54950_d_n6, assign41940_e54950_d_n7, assign41940_e54950_d_n8, assign41940_e54950_d_n9,) = {
    if ((locals.var_guard1199 == 0.0) && (locals.var_guard1200 != 0.0)) {
        let assign41940_e54948: f64 = (1.0 / locals.var_sp_s_delta0);
        (assign41940_e54948, (-(locals.var_sp_s_delta0_dn4 / (locals.var_sp_s_delta0 * locals.var_sp_s_delta0))), (-(locals.var_sp_s_delta0_dn6 / (locals.var_sp_s_delta0 * locals.var_sp_s_delta0))), (-(locals.var_sp_s_delta0_dn7 / (locals.var_sp_s_delta0 * locals.var_sp_s_delta0))), (-(locals.var_sp_s_delta0_dn8 / (locals.var_sp_s_delta0 * locals.var_sp_s_delta0))), (-(locals.var_sp_s_delta0_dn9 / (locals.var_sp_s_delta0 * locals.var_sp_s_delta0))),)
    } else {
        (locals.var_sp_s_delta1, locals.var_sp_s_delta1_dn4, locals.var_sp_s_delta1_dn6, locals.var_sp_s_delta1_dn7, locals.var_sp_s_delta1_dn8, locals.var_sp_s_delta1_dn9,)
    }
};
        locals.var_sp_s_delta1 = assign41940_e54950;
        locals.var_sp_s_delta1_dn4 = assign41940_e54950_d_n4;
        locals.var_sp_s_delta1_dn6 = assign41940_e54950_d_n6;
        locals.var_sp_s_delta1_dn7 = assign41940_e54950_d_n7;
        locals.var_sp_s_delta1_dn8 = assign41940_e54950_d_n8;
        locals.var_sp_s_delta1_dn9 = assign41940_e54950_d_n9;
        locals.var_sp_s_delta1_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_19(
        locals: &mut StampLocals,
    ) {
        let (assign41950_e54963, assign41950_e54963_d_n4, assign41950_e54963_d_n6, assign41950_e54963_d_n7, assign41950_e54963_d_n8, assign41950_e54963_d_n9,) = {
    if ((locals.var_guard1199 == 0.0) && (locals.var_guard1200 != 0.0)) {
        let assign41950_e54959: f64 = (locals.var_sp_s_y0 * locals.var_sp_s_y0);
        let assign41950_e54960: f64 = (2.0 + assign41950_e54959);
        let assign41950_e54961: f64 = (1.0 / assign41950_e54960);
        (assign41950_e54961, (-(((locals.var_sp_s_y0_dn4 * locals.var_sp_s_y0) + (locals.var_sp_s_y0 * locals.var_sp_s_y0_dn4)) / (assign41950_e54960 * assign41950_e54960))), (-(((locals.var_sp_s_y0_dn6 * locals.var_sp_s_y0) + (locals.var_sp_s_y0 * locals.var_sp_s_y0_dn6)) / (assign41950_e54960 * assign41950_e54960))), (-(((locals.var_sp_s_y0_dn7 * locals.var_sp_s_y0) + (locals.var_sp_s_y0 * locals.var_sp_s_y0_dn7)) / (assign41950_e54960 * assign41950_e54960))), (-(((locals.var_sp_s_y0_dn8 * locals.var_sp_s_y0) + (locals.var_sp_s_y0 * locals.var_sp_s_y0_dn8)) / (assign41950_e54960 * assign41950_e54960))), (-(((locals.var_sp_s_y0_dn9 * locals.var_sp_s_y0) + (locals.var_sp_s_y0 * locals.var_sp_s_y0_dn9)) / (assign41950_e54960 * assign41950_e54960))),)
    } else {
        (locals.var_sp_s_temp, locals.var_sp_s_temp_dn4, locals.var_sp_s_temp_dn6, locals.var_sp_s_temp_dn7, locals.var_sp_s_temp_dn8, locals.var_sp_s_temp_dn9,)
    }
};
        locals.var_sp_s_temp = assign41950_e54963;
        locals.var_sp_s_temp_dn4 = assign41950_e54963_d_n4;
        locals.var_sp_s_temp_dn6 = assign41950_e54963_d_n6;
        locals.var_sp_s_temp_dn7 = assign41950_e54963_d_n7;
        locals.var_sp_s_temp_dn8 = assign41950_e54963_d_n8;
        locals.var_sp_s_temp_dn9 = assign41950_e54963_d_n9;
        locals.var_sp_s_temp_rv = 0.0;

        let (assign41960_e54974, assign41960_e54974_d_n4, assign41960_e54974_d_n6, assign41960_e54974_d_n7, assign41960_e54974_d_n8, assign41960_e54974_d_n9,) = {
    if ((locals.var_guard1199 == 0.0) && (locals.var_guard1200 != 0.0)) {
        let assign41960_e54970: f64 = (locals.var_sp_s_y0 * locals.var_sp_s_y0);
        let assign41960_e54972: f64 = (assign41960_e54970 * locals.var_sp_s_temp);
        (assign41960_e54972, ((((locals.var_sp_s_y0_dn4 * locals.var_sp_s_y0) + (locals.var_sp_s_y0 * locals.var_sp_s_y0_dn4)) * locals.var_sp_s_temp) + (assign41960_e54970 * locals.var_sp_s_temp_dn4)), ((((locals.var_sp_s_y0_dn6 * locals.var_sp_s_y0) + (locals.var_sp_s_y0 * locals.var_sp_s_y0_dn6)) * locals.var_sp_s_temp) + (assign41960_e54970 * locals.var_sp_s_temp_dn6)), ((((locals.var_sp_s_y0_dn7 * locals.var_sp_s_y0) + (locals.var_sp_s_y0 * locals.var_sp_s_y0_dn7)) * locals.var_sp_s_temp) + (assign41960_e54970 * locals.var_sp_s_temp_dn7)), ((((locals.var_sp_s_y0_dn8 * locals.var_sp_s_y0) + (locals.var_sp_s_y0 * locals.var_sp_s_y0_dn8)) * locals.var_sp_s_temp) + (assign41960_e54970 * locals.var_sp_s_temp_dn8)), ((((locals.var_sp_s_y0_dn9 * locals.var_sp_s_y0) + (locals.var_sp_s_y0 * locals.var_sp_s_y0_dn9)) * locals.var_sp_s_temp) + (assign41960_e54970 * locals.var_sp_s_temp_dn9)),)
    } else {
        (locals.var_sp_s_xi0, locals.var_sp_s_xi0_dn4, locals.var_sp_s_xi0_dn6, locals.var_sp_s_xi0_dn7, locals.var_sp_s_xi0_dn8, locals.var_sp_s_xi0_dn9,)
    }
};
        locals.var_sp_s_xi0 = assign41960_e54974;
        locals.var_sp_s_xi0_dn4 = assign41960_e54974_d_n4;
        locals.var_sp_s_xi0_dn6 = assign41960_e54974_d_n6;
        locals.var_sp_s_xi0_dn7 = assign41960_e54974_d_n7;
        locals.var_sp_s_xi0_dn8 = assign41960_e54974_d_n8;
        locals.var_sp_s_xi0_dn9 = assign41960_e54974_d_n9;
        locals.var_sp_s_xi0_rv = 0.0;

        let (assign41970_e54987, assign41970_e54987_d_n4, assign41970_e54987_d_n6, assign41970_e54987_d_n7, assign41970_e54987_d_n8, assign41970_e54987_d_n9,) = {
    if ((locals.var_guard1199 == 0.0) && (locals.var_guard1200 != 0.0)) {
        let assign41970_e54982: f64 = (locals.var_sp_s_y0 * locals.var_sp_s_temp);
        let assign41970_e54984: f64 = (assign41970_e54982 * locals.var_sp_s_temp);
        let assign41970_e54985: f64 = (4.0 * assign41970_e54984);
        (assign41970_e54985, (4.0 * ((((locals.var_sp_s_y0_dn4 * locals.var_sp_s_temp) + (locals.var_sp_s_y0 * locals.var_sp_s_temp_dn4)) * locals.var_sp_s_temp) + (assign41970_e54982 * locals.var_sp_s_temp_dn4))), (4.0 * ((((locals.var_sp_s_y0_dn6 * locals.var_sp_s_temp) + (locals.var_sp_s_y0 * locals.var_sp_s_temp_dn6)) * locals.var_sp_s_temp) + (assign41970_e54982 * locals.var_sp_s_temp_dn6))), (4.0 * ((((locals.var_sp_s_y0_dn7 * locals.var_sp_s_temp) + (locals.var_sp_s_y0 * locals.var_sp_s_temp_dn7)) * locals.var_sp_s_temp) + (assign41970_e54982 * locals.var_sp_s_temp_dn7))), (4.0 * ((((locals.var_sp_s_y0_dn8 * locals.var_sp_s_temp) + (locals.var_sp_s_y0 * locals.var_sp_s_temp_dn8)) * locals.var_sp_s_temp) + (assign41970_e54982 * locals.var_sp_s_temp_dn8))), (4.0 * ((((locals.var_sp_s_y0_dn9 * locals.var_sp_s_temp) + (locals.var_sp_s_y0 * locals.var_sp_s_temp_dn9)) * locals.var_sp_s_temp) + (assign41970_e54982 * locals.var_sp_s_temp_dn9))),)
    } else {
        (locals.var_sp_s_xi1, locals.var_sp_s_xi1_dn4, locals.var_sp_s_xi1_dn6, locals.var_sp_s_xi1_dn7, locals.var_sp_s_xi1_dn8, locals.var_sp_s_xi1_dn9,)
    }
};
        locals.var_sp_s_xi1 = assign41970_e54987;
        locals.var_sp_s_xi1_dn4 = assign41970_e54987_d_n4;
        locals.var_sp_s_xi1_dn6 = assign41970_e54987_d_n6;
        locals.var_sp_s_xi1_dn7 = assign41970_e54987_d_n7;
        locals.var_sp_s_xi1_dn8 = assign41970_e54987_d_n8;
        locals.var_sp_s_xi1_dn9 = assign41970_e54987_d_n9;
        locals.var_sp_s_xi1_rv = 0.0;

        let (assign41980_e55004, assign41980_e55004_d_n4, assign41980_e55004_d_n6, assign41980_e55004_d_n7, assign41980_e55004_d_n8, assign41980_e55004_d_n9,) = {
    if ((locals.var_guard1199 == 0.0) && (locals.var_guard1200 != 0.0)) {
        let assign41980_e54994: f64 = (8.0 * locals.var_sp_s_temp);
        let assign41980_e54997: f64 = (12.0 * locals.var_sp_s_xi0);
        let assign41980_e54998: f64 = (assign41980_e54994 - assign41980_e54997);
        let assign41980_e55000: f64 = (assign41980_e54998 * locals.var_sp_s_temp);
        let assign41980_e55002: f64 = (assign41980_e55000 * locals.var_sp_s_temp);
        (assign41980_e55002, ((((((8.0 * locals.var_sp_s_temp_dn4) - (12.0 * locals.var_sp_s_xi0_dn4)) * locals.var_sp_s_temp) + (assign41980_e54998 * locals.var_sp_s_temp_dn4)) * locals.var_sp_s_temp) + (assign41980_e55000 * locals.var_sp_s_temp_dn4)), ((((((8.0 * locals.var_sp_s_temp_dn6) - (12.0 * locals.var_sp_s_xi0_dn6)) * locals.var_sp_s_temp) + (assign41980_e54998 * locals.var_sp_s_temp_dn6)) * locals.var_sp_s_temp) + (assign41980_e55000 * locals.var_sp_s_temp_dn6)), ((((((8.0 * locals.var_sp_s_temp_dn7) - (12.0 * locals.var_sp_s_xi0_dn7)) * locals.var_sp_s_temp) + (assign41980_e54998 * locals.var_sp_s_temp_dn7)) * locals.var_sp_s_temp) + (assign41980_e55000 * locals.var_sp_s_temp_dn7)), ((((((8.0 * locals.var_sp_s_temp_dn8) - (12.0 * locals.var_sp_s_xi0_dn8)) * locals.var_sp_s_temp) + (assign41980_e54998 * locals.var_sp_s_temp_dn8)) * locals.var_sp_s_temp) + (assign41980_e55000 * locals.var_sp_s_temp_dn8)), ((((((8.0 * locals.var_sp_s_temp_dn9) - (12.0 * locals.var_sp_s_xi0_dn9)) * locals.var_sp_s_temp) + (assign41980_e54998 * locals.var_sp_s_temp_dn9)) * locals.var_sp_s_temp) + (assign41980_e55000 * locals.var_sp_s_temp_dn9)),)
    } else {
        (locals.var_sp_s_xi2, locals.var_sp_s_xi2_dn4, locals.var_sp_s_xi2_dn6, locals.var_sp_s_xi2_dn7, locals.var_sp_s_xi2_dn8, locals.var_sp_s_xi2_dn9,)
    }
};
        locals.var_sp_s_xi2 = assign41980_e55004;
        locals.var_sp_s_xi2_dn4 = assign41980_e55004_d_n4;
        locals.var_sp_s_xi2_dn6 = assign41980_e55004_d_n6;
        locals.var_sp_s_xi2_dn7 = assign41980_e55004_d_n7;
        locals.var_sp_s_xi2_dn8 = assign41980_e55004_d_n8;
        locals.var_sp_s_xi2_dn9 = assign41980_e55004_d_n9;
        locals.var_sp_s_xi2_rv = 0.0;

        let (assign41990_e55013, assign41990_e55013_d_n4, assign41990_e55013_d_n6, assign41990_e55013_d_n7, assign41990_e55013_d_n8, assign41990_e55013_d_n9,) = {
    if ((locals.var_guard1199 == 0.0) && (locals.var_guard1200 != 0.0)) {
        let assign41990_e55011: f64 = (locals.var_sp_s_yg - locals.var_sp_s_y0);
        (assign41990_e55011, (locals.var_sp_s_yg_dn4 - locals.var_sp_s_y0_dn4), (locals.var_sp_s_yg_dn6 - locals.var_sp_s_y0_dn6), (locals.var_sp_s_yg_dn7 - locals.var_sp_s_y0_dn7), (locals.var_sp_s_yg_dn8 - locals.var_sp_s_y0_dn8), (locals.var_sp_s_yg_dn9 - locals.var_sp_s_y0_dn9),)
    } else {
        (locals.var_sp_s_temp, locals.var_sp_s_temp_dn4, locals.var_sp_s_temp_dn6, locals.var_sp_s_temp_dn7, locals.var_sp_s_temp_dn8, locals.var_sp_s_temp_dn9,)
    }
};
        locals.var_sp_s_temp = assign41990_e55013;
        locals.var_sp_s_temp_dn4 = assign41990_e55013_d_n4;
        locals.var_sp_s_temp_dn6 = assign41990_e55013_d_n6;
        locals.var_sp_s_temp_dn7 = assign41990_e55013_d_n7;
        locals.var_sp_s_temp_dn8 = assign41990_e55013_d_n8;
        locals.var_sp_s_temp_dn9 = assign41990_e55013_d_n9;
        locals.var_sp_s_temp_rv = 0.0;

        let (assign42000_e55022, assign42000_e55022_d_n4, assign42000_e55022_d_n6, assign42000_e55022_d_n7, assign42000_e55022_d_n8, assign42000_e55022_d_n9,) = {
    if ((locals.var_guard1199 == 0.0) && (locals.var_guard1200 != 0.0)) {
        let assign42000_e55020: f64 = (locals.var_delta_ns * locals.var_sp_s_delta1);
        (assign42000_e55020, ((locals.var_delta_ns_dn4 * locals.var_sp_s_delta1) + (locals.var_delta_ns * locals.var_sp_s_delta1_dn4)), ((locals.var_delta_ns_dn6 * locals.var_sp_s_delta1) + (locals.var_delta_ns * locals.var_sp_s_delta1_dn6)), ((locals.var_delta_ns_dn7 * locals.var_sp_s_delta1) + (locals.var_delta_ns * locals.var_sp_s_delta1_dn7)), ((locals.var_delta_ns_dn8 * locals.var_sp_s_delta1) + (locals.var_delta_ns * locals.var_sp_s_delta1_dn8)), ((locals.var_delta_ns_dn9 * locals.var_sp_s_delta1) + (locals.var_delta_ns * locals.var_sp_s_delta1_dn9)),)
    } else {
        (locals.var_sp_s_temp1, locals.var_sp_s_temp1_dn4, locals.var_sp_s_temp1_dn6, locals.var_sp_s_temp1_dn7, locals.var_sp_s_temp1_dn8, locals.var_sp_s_temp1_dn9,)
    }
};
        locals.var_sp_s_temp1 = assign42000_e55022;
        locals.var_sp_s_temp1_dn4 = assign42000_e55022_d_n4;
        locals.var_sp_s_temp1_dn6 = assign42000_e55022_d_n6;
        locals.var_sp_s_temp1_dn7 = assign42000_e55022_d_n7;
        locals.var_sp_s_temp1_dn8 = assign42000_e55022_d_n8;
        locals.var_sp_s_temp1_dn9 = assign42000_e55022_d_n9;
        locals.var_sp_s_temp1_rv = 0.0;

        let (assign42010_e55045, assign42010_e55045_d_n4, assign42010_e55045_d_n6, assign42010_e55045_d_n7, assign42010_e55045_d_n8, assign42010_e55045_d_n9,) = {
    if ((locals.var_guard1199 == 0.0) && (locals.var_guard1200 != 0.0)) {
        let assign42010_e55029: f64 = (2.0 * locals.var_sp_s_temp);
        let assign42010_e55033: f64 = (locals.var_sp_s_delta0 - 1.0);
        let assign42010_e55035: f64 = (assign42010_e55033 - locals.var_sp_s_temp1);
        let assign42010_e55039: f64 = (1.0 - locals.var_sp_s_xi1);
        let assign42010_e55040: f64 = (locals.var_delta_ns * assign42010_e55039);
        let assign42010_e55041: f64 = (assign42010_e55035 + assign42010_e55040);
        let assign42010_e55042: f64 = (locals.var_gf2 * assign42010_e55041);
        let assign42010_e55043: f64 = (assign42010_e55029 + assign42010_e55042);
        (assign42010_e55043, ((2.0 * locals.var_sp_s_temp_dn4) + ((locals.var_gf2_dn4 * assign42010_e55041) + (locals.var_gf2 * ((locals.var_sp_s_delta0_dn4 - locals.var_sp_s_temp1_dn4) + ((locals.var_delta_ns_dn4 * assign42010_e55039) + (locals.var_delta_ns * (-locals.var_sp_s_xi1_dn4))))))), ((2.0 * locals.var_sp_s_temp_dn6) + ((locals.var_gf2_dn6 * assign42010_e55041) + (locals.var_gf2 * ((locals.var_sp_s_delta0_dn6 - locals.var_sp_s_temp1_dn6) + ((locals.var_delta_ns_dn6 * assign42010_e55039) + (locals.var_delta_ns * (-locals.var_sp_s_xi1_dn6))))))), ((2.0 * locals.var_sp_s_temp_dn7) + ((locals.var_gf2_dn7 * assign42010_e55041) + (locals.var_gf2 * ((locals.var_sp_s_delta0_dn7 - locals.var_sp_s_temp1_dn7) + ((locals.var_delta_ns_dn7 * assign42010_e55039) + (locals.var_delta_ns * (-locals.var_sp_s_xi1_dn7))))))), ((2.0 * locals.var_sp_s_temp_dn8) + ((locals.var_gf2_dn8 * assign42010_e55041) + (locals.var_gf2 * ((locals.var_sp_s_delta0_dn8 - locals.var_sp_s_temp1_dn8) + ((locals.var_delta_ns_dn8 * assign42010_e55039) + (locals.var_delta_ns * (-locals.var_sp_s_xi1_dn8))))))), ((2.0 * locals.var_sp_s_temp_dn9) + ((locals.var_gf2_dn9 * assign42010_e55041) + (locals.var_gf2 * ((locals.var_sp_s_delta0_dn9 - locals.var_sp_s_temp1_dn9) + ((locals.var_delta_ns_dn9 * assign42010_e55039) + (locals.var_delta_ns * (-locals.var_sp_s_xi1_dn9))))))),)
    } else {
        (locals.var_sp_s_pc, locals.var_sp_s_pc_dn4, locals.var_sp_s_pc_dn6, locals.var_sp_s_pc_dn7, locals.var_sp_s_pc_dn8, locals.var_sp_s_pc_dn9,)
    }
};
        locals.var_sp_s_pc = assign42010_e55045;
        locals.var_sp_s_pc_dn4 = assign42010_e55045_d_n4;
        locals.var_sp_s_pc_dn6 = assign42010_e55045_d_n6;
        locals.var_sp_s_pc_dn7 = assign42010_e55045_d_n7;
        locals.var_sp_s_pc_dn8 = assign42010_e55045_d_n8;
        locals.var_sp_s_pc_dn9 = assign42010_e55045_d_n9;
        locals.var_sp_s_pc_rv = 0.0;

        let (assign42020_e55072, assign42020_e55072_d_n4, assign42020_e55072_d_n6, assign42020_e55072_d_n7, assign42020_e55072_d_n8, assign42020_e55072_d_n9,) = {
    if ((locals.var_guard1199 == 0.0) && (locals.var_guard1200 != 0.0)) {
        let assign42020_e55052: f64 = (locals.var_sp_s_temp * locals.var_sp_s_temp);
        let assign42020_e55056: f64 = (locals.var_sp_s_delta0 - locals.var_sp_s_y0);
        let assign42020_e55058: f64 = (assign42020_e55056 - 1.0);
        let assign42020_e55060: f64 = (assign42020_e55058 + locals.var_sp_s_temp1);
        let assign42020_e55064: f64 = (locals.var_sp_s_y0 - 1.0);
        let assign42020_e55066: f64 = (assign42020_e55064 - locals.var_sp_s_xi0);
        let assign42020_e55067: f64 = (locals.var_delta_ns * assign42020_e55066);
        let assign42020_e55068: f64 = (assign42020_e55060 + assign42020_e55067);
        let assign42020_e55069: f64 = (locals.var_gf2 * assign42020_e55068);
        let assign42020_e55070: f64 = (assign42020_e55052 - assign42020_e55069);
        (assign42020_e55070, (((locals.var_sp_s_temp_dn4 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn4)) - ((locals.var_gf2_dn4 * assign42020_e55068) + (locals.var_gf2 * (((locals.var_sp_s_delta0_dn4 - locals.var_sp_s_y0_dn4) + locals.var_sp_s_temp1_dn4) + ((locals.var_delta_ns_dn4 * assign42020_e55066) + (locals.var_delta_ns * (locals.var_sp_s_y0_dn4 - locals.var_sp_s_xi0_dn4))))))), (((locals.var_sp_s_temp_dn6 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn6)) - ((locals.var_gf2_dn6 * assign42020_e55068) + (locals.var_gf2 * (((locals.var_sp_s_delta0_dn6 - locals.var_sp_s_y0_dn6) + locals.var_sp_s_temp1_dn6) + ((locals.var_delta_ns_dn6 * assign42020_e55066) + (locals.var_delta_ns * (locals.var_sp_s_y0_dn6 - locals.var_sp_s_xi0_dn6))))))), (((locals.var_sp_s_temp_dn7 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn7)) - ((locals.var_gf2_dn7 * assign42020_e55068) + (locals.var_gf2 * (((locals.var_sp_s_delta0_dn7 - locals.var_sp_s_y0_dn7) + locals.var_sp_s_temp1_dn7) + ((locals.var_delta_ns_dn7 * assign42020_e55066) + (locals.var_delta_ns * (locals.var_sp_s_y0_dn7 - locals.var_sp_s_xi0_dn7))))))), (((locals.var_sp_s_temp_dn8 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn8)) - ((locals.var_gf2_dn8 * assign42020_e55068) + (locals.var_gf2 * (((locals.var_sp_s_delta0_dn8 - locals.var_sp_s_y0_dn8) + locals.var_sp_s_temp1_dn8) + ((locals.var_delta_ns_dn8 * assign42020_e55066) + (locals.var_delta_ns * (locals.var_sp_s_y0_dn8 - locals.var_sp_s_xi0_dn8))))))), (((locals.var_sp_s_temp_dn9 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn9)) - ((locals.var_gf2_dn9 * assign42020_e55068) + (locals.var_gf2 * (((locals.var_sp_s_delta0_dn9 - locals.var_sp_s_y0_dn9) + locals.var_sp_s_temp1_dn9) + ((locals.var_delta_ns_dn9 * assign42020_e55066) + (locals.var_delta_ns * (locals.var_sp_s_y0_dn9 - locals.var_sp_s_xi0_dn9))))))),)
    } else {
        (locals.var_sp_s_qc, locals.var_sp_s_qc_dn4, locals.var_sp_s_qc_dn6, locals.var_sp_s_qc_dn7, locals.var_sp_s_qc_dn8, locals.var_sp_s_qc_dn9,)
    }
};
        locals.var_sp_s_qc = assign42020_e55072;
        locals.var_sp_s_qc_dn4 = assign42020_e55072_d_n4;
        locals.var_sp_s_qc_dn6 = assign42020_e55072_d_n6;
        locals.var_sp_s_qc_dn7 = assign42020_e55072_d_n7;
        locals.var_sp_s_qc_dn8 = assign42020_e55072_d_n8;
        locals.var_sp_s_qc_dn9 = assign42020_e55072_d_n9;
        locals.var_sp_s_qc_rv = 0.0;

        let (assign42030_e55089, assign42030_e55089_d_n4, assign42030_e55089_d_n6, assign42030_e55089_d_n7, assign42030_e55089_d_n8, assign42030_e55089_d_n9,) = {
    if ((locals.var_guard1199 == 0.0) && (locals.var_guard1200 != 0.0)) {
        let assign42030_e55081: f64 = (locals.var_sp_s_delta0 + locals.var_sp_s_temp1);
        let assign42030_e55084: f64 = (locals.var_delta_ns * locals.var_sp_s_xi2);
        let assign42030_e55085: f64 = (assign42030_e55081 - assign42030_e55084);
        let assign42030_e55086: f64 = (locals.var_gf2 * assign42030_e55085);
        let assign42030_e55087: f64 = (2.0 - assign42030_e55086);
        (assign42030_e55087, (-((locals.var_gf2_dn4 * assign42030_e55085) + (locals.var_gf2 * ((locals.var_sp_s_delta0_dn4 + locals.var_sp_s_temp1_dn4) - ((locals.var_delta_ns_dn4 * locals.var_sp_s_xi2) + (locals.var_delta_ns * locals.var_sp_s_xi2_dn4)))))), (-((locals.var_gf2_dn6 * assign42030_e55085) + (locals.var_gf2 * ((locals.var_sp_s_delta0_dn6 + locals.var_sp_s_temp1_dn6) - ((locals.var_delta_ns_dn6 * locals.var_sp_s_xi2) + (locals.var_delta_ns * locals.var_sp_s_xi2_dn6)))))), (-((locals.var_gf2_dn7 * assign42030_e55085) + (locals.var_gf2 * ((locals.var_sp_s_delta0_dn7 + locals.var_sp_s_temp1_dn7) - ((locals.var_delta_ns_dn7 * locals.var_sp_s_xi2) + (locals.var_delta_ns * locals.var_sp_s_xi2_dn7)))))), (-((locals.var_gf2_dn8 * assign42030_e55085) + (locals.var_gf2 * ((locals.var_sp_s_delta0_dn8 + locals.var_sp_s_temp1_dn8) - ((locals.var_delta_ns_dn8 * locals.var_sp_s_xi2) + (locals.var_delta_ns * locals.var_sp_s_xi2_dn8)))))), (-((locals.var_gf2_dn9 * assign42030_e55085) + (locals.var_gf2 * ((locals.var_sp_s_delta0_dn9 + locals.var_sp_s_temp1_dn9) - ((locals.var_delta_ns_dn9 * locals.var_sp_s_xi2) + (locals.var_delta_ns * locals.var_sp_s_xi2_dn9)))))),)
    } else {
        (locals.var_sp_s_temp, locals.var_sp_s_temp_dn4, locals.var_sp_s_temp_dn6, locals.var_sp_s_temp_dn7, locals.var_sp_s_temp_dn8, locals.var_sp_s_temp_dn9,)
    }
};
        locals.var_sp_s_temp = assign42030_e55089;
        locals.var_sp_s_temp_dn4 = assign42030_e55089_d_n4;
        locals.var_sp_s_temp_dn6 = assign42030_e55089_d_n6;
        locals.var_sp_s_temp_dn7 = assign42030_e55089_d_n7;
        locals.var_sp_s_temp_dn8 = assign42030_e55089_d_n8;
        locals.var_sp_s_temp_dn9 = assign42030_e55089_d_n9;
        locals.var_sp_s_temp_rv = 0.0;

        let (assign42040_e55104, assign42040_e55104_d_n4, assign42040_e55104_d_n6, assign42040_e55104_d_n7, assign42040_e55104_d_n8, assign42040_e55104_d_n9,) = {
    if ((locals.var_guard1199 == 0.0) && (locals.var_guard1200 != 0.0)) {
        let assign42040_e55096: f64 = (locals.var_sp_s_pc * locals.var_sp_s_pc);
        let assign42040_e55100: f64 = (locals.var_sp_s_qc * locals.var_sp_s_temp);
        let assign42040_e55101: f64 = (2.0 * assign42040_e55100);
        let assign42040_e55102: f64 = (assign42040_e55096 - assign42040_e55101);
        (assign42040_e55102, (((locals.var_sp_s_pc_dn4 * locals.var_sp_s_pc) + (locals.var_sp_s_pc * locals.var_sp_s_pc_dn4)) - (2.0 * ((locals.var_sp_s_qc_dn4 * locals.var_sp_s_temp) + (locals.var_sp_s_qc * locals.var_sp_s_temp_dn4)))), (((locals.var_sp_s_pc_dn6 * locals.var_sp_s_pc) + (locals.var_sp_s_pc * locals.var_sp_s_pc_dn6)) - (2.0 * ((locals.var_sp_s_qc_dn6 * locals.var_sp_s_temp) + (locals.var_sp_s_qc * locals.var_sp_s_temp_dn6)))), (((locals.var_sp_s_pc_dn7 * locals.var_sp_s_pc) + (locals.var_sp_s_pc * locals.var_sp_s_pc_dn7)) - (2.0 * ((locals.var_sp_s_qc_dn7 * locals.var_sp_s_temp) + (locals.var_sp_s_qc * locals.var_sp_s_temp_dn7)))), (((locals.var_sp_s_pc_dn8 * locals.var_sp_s_pc) + (locals.var_sp_s_pc * locals.var_sp_s_pc_dn8)) - (2.0 * ((locals.var_sp_s_qc_dn8 * locals.var_sp_s_temp) + (locals.var_sp_s_qc * locals.var_sp_s_temp_dn8)))), (((locals.var_sp_s_pc_dn9 * locals.var_sp_s_pc) + (locals.var_sp_s_pc * locals.var_sp_s_pc_dn9)) - (2.0 * ((locals.var_sp_s_qc_dn9 * locals.var_sp_s_temp) + (locals.var_sp_s_qc * locals.var_sp_s_temp_dn9)))),)
    } else {
        (locals.var_sp_s_temp, locals.var_sp_s_temp_dn4, locals.var_sp_s_temp_dn6, locals.var_sp_s_temp_dn7, locals.var_sp_s_temp_dn8, locals.var_sp_s_temp_dn9,)
    }
};
        locals.var_sp_s_temp = assign42040_e55104;
        locals.var_sp_s_temp_dn4 = assign42040_e55104_d_n4;
        locals.var_sp_s_temp_dn6 = assign42040_e55104_d_n6;
        locals.var_sp_s_temp_dn7 = assign42040_e55104_d_n7;
        locals.var_sp_s_temp_dn8 = assign42040_e55104_d_n8;
        locals.var_sp_s_temp_dn9 = assign42040_e55104_d_n9;
        locals.var_sp_s_temp_rv = 0.0;

        let (assign42050_e55121, assign42050_e55121_d_n4, assign42050_e55121_d_n6, assign42050_e55121_d_n7, assign42050_e55121_d_n8, assign42050_e55121_d_n9,) = {
    if ((locals.var_guard1199 == 0.0) && (locals.var_guard1200 != 0.0)) {
        let assign42050_e55110: f64 = (-locals.var_sp_s_y0);
        let assign42050_e55115: f64 = (locals.var_sp_s_temp).sqrt();
        let assign42050_e55116: f64 = (locals.var_sp_s_pc + assign42050_e55115);
        let assign42050_e55117: f64 = (locals.var_sp_s_qc / assign42050_e55116);
        let assign42050_e55118: f64 = (2.0 * assign42050_e55117);
        let assign42050_e55119: f64 = (assign42050_e55110 - assign42050_e55118);
        (assign42050_e55119, ((-locals.var_sp_s_y0_dn4) - (2.0 * (((locals.var_sp_s_qc_dn4 * assign42050_e55116) - (locals.var_sp_s_qc * (locals.var_sp_s_pc_dn4 + (locals.var_sp_s_temp_dn4 / (2.0 * assign42050_e55115))))) / (assign42050_e55116 * assign42050_e55116)))), ((-locals.var_sp_s_y0_dn6) - (2.0 * (((locals.var_sp_s_qc_dn6 * assign42050_e55116) - (locals.var_sp_s_qc * (locals.var_sp_s_pc_dn6 + (locals.var_sp_s_temp_dn6 / (2.0 * assign42050_e55115))))) / (assign42050_e55116 * assign42050_e55116)))), ((-locals.var_sp_s_y0_dn7) - (2.0 * (((locals.var_sp_s_qc_dn7 * assign42050_e55116) - (locals.var_sp_s_qc * (locals.var_sp_s_pc_dn7 + (locals.var_sp_s_temp_dn7 / (2.0 * assign42050_e55115))))) / (assign42050_e55116 * assign42050_e55116)))), ((-locals.var_sp_s_y0_dn8) - (2.0 * (((locals.var_sp_s_qc_dn8 * assign42050_e55116) - (locals.var_sp_s_qc * (locals.var_sp_s_pc_dn8 + (locals.var_sp_s_temp_dn8 / (2.0 * assign42050_e55115))))) / (assign42050_e55116 * assign42050_e55116)))), ((-locals.var_sp_s_y0_dn9) - (2.0 * (((locals.var_sp_s_qc_dn9 * assign42050_e55116) - (locals.var_sp_s_qc * (locals.var_sp_s_pc_dn9 + (locals.var_sp_s_temp_dn9 / (2.0 * assign42050_e55115))))) / (assign42050_e55116 * assign42050_e55116)))),)
    } else {
        (locals.var_x_s, locals.var_x_s_dn4, locals.var_x_s_dn6, locals.var_x_s_dn7, locals.var_x_s_dn8, locals.var_x_s_dn9,)
    }
};
        locals.var_x_s = assign42050_e55121;
        locals.var_x_s_dn4 = assign42050_e55121_d_n4;
        locals.var_x_s_dn6 = assign42050_e55121_d_n6;
        locals.var_x_s_dn7 = assign42050_e55121_d_n7;
        locals.var_x_s_dn8 = assign42050_e55121_d_n8;
        locals.var_x_s_dn9 = assign42050_e55121_d_n9;
        locals.var_x_s_rv = 0.0;

        let (assign42060_e55135, assign42060_e55135_d_n4, assign42060_e55135_d_n6, assign42060_e55135_d_n7, assign42060_e55135_d_n8, assign42060_e55135_d_n9,) = {
    if ((locals.var_guard1199 == 0.0) && (locals.var_guard1200 == 0.0)) {
        let assign42060_e55131: f64 = (locals.var_gf * 0.7324648775608221);
        let assign42060_e55132: f64 = (1.25 + assign42060_e55131);
        let assign42060_e55133: f64 = (1.0 / assign42060_e55132);
        (assign42060_e55133, (-((locals.var_gf_dn4 * 0.7324648775608221) / (assign42060_e55132 * assign42060_e55132))), (-((locals.var_gf_dn6 * 0.7324648775608221) / (assign42060_e55132 * assign42060_e55132))), (-((locals.var_gf_dn7 * 0.7324648775608221) / (assign42060_e55132 * assign42060_e55132))), (-((locals.var_gf_dn8 * 0.7324648775608221) / (assign42060_e55132 * assign42060_e55132))), (-((locals.var_gf_dn9 * 0.7324648775608221) / (assign42060_e55132 * assign42060_e55132))),)
    } else {
        (locals.var_sp_xg1, locals.var_sp_xg1_dn4, locals.var_sp_xg1_dn6, locals.var_sp_xg1_dn7, locals.var_sp_xg1_dn8, locals.var_sp_xg1_dn9,)
    }
};
        locals.var_sp_xg1 = assign42060_e55135;
        locals.var_sp_xg1_dn4 = assign42060_e55135_d_n4;
        locals.var_sp_xg1_dn6 = assign42060_e55135_d_n6;
        locals.var_sp_xg1_dn7 = assign42060_e55135_d_n7;
        locals.var_sp_xg1_dn8 = assign42060_e55135_d_n8;
        locals.var_sp_xg1_dn9 = assign42060_e55135_d_n9;
        locals.var_sp_xg1_rv = 0.0;

        let (assign42070_e55151, assign42070_e55151_d_n4, assign42070_e55151_d_n6, assign42070_e55151_d_n7, assign42070_e55151_d_n8, assign42070_e55151_d_n9,) = {
    if ((locals.var_guard1199 == 0.0) && (locals.var_guard1200 == 0.0)) {
        let assign42070_e55143: f64 = (locals.var_xi * 1.25);
        let assign42070_e55145: f64 = (assign42070_e55143 * locals.var_sp_xg1);
        let assign42070_e55147: f64 = (assign42070_e55145 - 1.0);
        let assign42070_e55149: f64 = (assign42070_e55147 * locals.var_sp_xg1);
        (assign42070_e55149, (((((locals.var_xi_dn4 * 1.25) * locals.var_sp_xg1) + (assign42070_e55143 * locals.var_sp_xg1_dn4)) * locals.var_sp_xg1) + (assign42070_e55147 * locals.var_sp_xg1_dn4)), (((((locals.var_xi_dn6 * 1.25) * locals.var_sp_xg1) + (assign42070_e55143 * locals.var_sp_xg1_dn6)) * locals.var_sp_xg1) + (assign42070_e55147 * locals.var_sp_xg1_dn6)), (((((locals.var_xi_dn7 * 1.25) * locals.var_sp_xg1) + (assign42070_e55143 * locals.var_sp_xg1_dn7)) * locals.var_sp_xg1) + (assign42070_e55147 * locals.var_sp_xg1_dn7)), (((((locals.var_xi_dn8 * 1.25) * locals.var_sp_xg1) + (assign42070_e55143 * locals.var_sp_xg1_dn8)) * locals.var_sp_xg1) + (assign42070_e55147 * locals.var_sp_xg1_dn8)), (((((locals.var_xi_dn9 * 1.25) * locals.var_sp_xg1) + (assign42070_e55143 * locals.var_sp_xg1_dn9)) * locals.var_sp_xg1) + (assign42070_e55147 * locals.var_sp_xg1_dn9)),)
    } else {
        (locals.var_sp_s_a_fac, locals.var_sp_s_a_fac_dn4, locals.var_sp_s_a_fac_dn6, locals.var_sp_s_a_fac_dn7, locals.var_sp_s_a_fac_dn8, locals.var_sp_s_a_fac_dn9,)
    }
};
        locals.var_sp_s_a_fac = assign42070_e55151;
        locals.var_sp_s_a_fac_dn4 = assign42070_e55151_d_n4;
        locals.var_sp_s_a_fac_dn6 = assign42070_e55151_d_n6;
        locals.var_sp_s_a_fac_dn7 = assign42070_e55151_d_n7;
        locals.var_sp_s_a_fac_dn8 = assign42070_e55151_d_n8;
        locals.var_sp_s_a_fac_dn9 = assign42070_e55151_d_n9;
        locals.var_sp_s_a_fac_rv = 0.0;

        let (assign42080_e55167, assign42080_e55167_d_n4, assign42080_e55167_d_n6, assign42080_e55167_d_n7, assign42080_e55167_d_n8, assign42080_e55167_d_n9,) = {
    if ((locals.var_guard1199 == 0.0) && (locals.var_guard1200 == 0.0)) {
        let assign42080_e55159: f64 = (locals.var_xg * locals.var_inv_xi);
        let assign42080_e55163: f64 = (locals.var_sp_s_a_fac * locals.var_xg);
        let assign42080_e55164: f64 = (1.0 + assign42080_e55163);
        let assign42080_e55165: f64 = (assign42080_e55159 * assign42080_e55164);
        (assign42080_e55165, ((((locals.var_xg_dn4 * locals.var_inv_xi) + (locals.var_xg * locals.var_inv_xi_dn4)) * assign42080_e55164) + (assign42080_e55159 * ((locals.var_sp_s_a_fac_dn4 * locals.var_xg) + (locals.var_sp_s_a_fac * locals.var_xg_dn4)))), ((((locals.var_xg_dn6 * locals.var_inv_xi) + (locals.var_xg * locals.var_inv_xi_dn6)) * assign42080_e55164) + (assign42080_e55159 * ((locals.var_sp_s_a_fac_dn6 * locals.var_xg) + (locals.var_sp_s_a_fac * locals.var_xg_dn6)))), ((((locals.var_xg_dn7 * locals.var_inv_xi) + (locals.var_xg * locals.var_inv_xi_dn7)) * assign42080_e55164) + (assign42080_e55159 * ((locals.var_sp_s_a_fac_dn7 * locals.var_xg) + (locals.var_sp_s_a_fac * locals.var_xg_dn7)))), ((((locals.var_xg_dn8 * locals.var_inv_xi) + (locals.var_xg * locals.var_inv_xi_dn8)) * assign42080_e55164) + (assign42080_e55159 * ((locals.var_sp_s_a_fac_dn8 * locals.var_xg) + (locals.var_sp_s_a_fac * locals.var_xg_dn8)))), ((((locals.var_xg_dn9 * locals.var_inv_xi) + (locals.var_xg * locals.var_inv_xi_dn9)) * assign42080_e55164) + (assign42080_e55159 * ((locals.var_sp_s_a_fac_dn9 * locals.var_xg) + (locals.var_sp_s_a_fac * locals.var_xg_dn9)))),)
    } else {
        (locals.var_sp_s_xbar, locals.var_sp_s_xbar_dn4, locals.var_sp_s_xbar_dn6, locals.var_sp_s_xbar_dn7, locals.var_sp_s_xbar_dn8, locals.var_sp_s_xbar_dn9,)
    }
};
        locals.var_sp_s_xbar = assign42080_e55167;
        locals.var_sp_s_xbar_dn4 = assign42080_e55167_d_n4;
        locals.var_sp_s_xbar_dn6 = assign42080_e55167_d_n6;
        locals.var_sp_s_xbar_dn7 = assign42080_e55167_d_n7;
        locals.var_sp_s_xbar_dn8 = assign42080_e55167_d_n8;
        locals.var_sp_s_xbar_dn9 = assign42080_e55167_d_n9;
        locals.var_sp_s_xbar_rv = 0.0;

        let assign42090_e55169: f64 = (-locals.var_sp_s_xbar);
        let assign42090_e55171: f64 = (-230.25850929940458);
        let assign42090_e55172: f64 = if assign42090_e55169 > assign42090_e55171 { 1.0 } else { 0.0 };
        locals.var_guard1202 = assign42090_e55172;
        locals.var_guard1202_rv = 0.0;

        let (assign42100_e55184, assign42100_e55184_d_n4, assign42100_e55184_d_n6, assign42100_e55184_d_n7, assign42100_e55184_d_n8, assign42100_e55184_d_n9,) = {
    if (((locals.var_guard1199 == 0.0) && (locals.var_guard1200 == 0.0)) && (locals.var_guard1202 != 0.0)) {
        let assign42100_e55181: f64 = (-locals.var_sp_s_xbar);
        let assign42100_e55182: f64 = (assign42100_e55181).exp();
        (assign42100_e55182, (assign42100_e55182 * (-locals.var_sp_s_xbar_dn4)), (assign42100_e55182 * (-locals.var_sp_s_xbar_dn6)), (assign42100_e55182 * (-locals.var_sp_s_xbar_dn7)), (assign42100_e55182 * (-locals.var_sp_s_xbar_dn8)), (assign42100_e55182 * (-locals.var_sp_s_xbar_dn9)),)
    } else {
        (locals.var_sp_s_temp, locals.var_sp_s_temp_dn4, locals.var_sp_s_temp_dn6, locals.var_sp_s_temp_dn7, locals.var_sp_s_temp_dn8, locals.var_sp_s_temp_dn9,)
    }
};
        locals.var_sp_s_temp = assign42100_e55184;
        locals.var_sp_s_temp_dn4 = assign42100_e55184_d_n4;
        locals.var_sp_s_temp_dn6 = assign42100_e55184_d_n6;
        locals.var_sp_s_temp_dn7 = assign42100_e55184_d_n7;
        locals.var_sp_s_temp_dn8 = assign42100_e55184_d_n8;
        locals.var_sp_s_temp_dn9 = assign42100_e55184_d_n9;
        locals.var_sp_s_temp_rv = 0.0;

        let (assign42110_e55223, assign42110_e55223_d_n4, assign42110_e55223_d_n6, assign42110_e55223_d_n7, assign42110_e55223_d_n8, assign42110_e55223_d_n9,) = {
    if (((locals.var_guard1199 == 0.0) && (locals.var_guard1200 == 0.0)) && (locals.var_guard1202 == 0.0)) {
        let assign42110_e55196: f64 = (-230.25850929940458);
        let assign42110_e55198: f64 = (-locals.var_sp_s_xbar);
        let assign42110_e55199: f64 = (assign42110_e55196 - assign42110_e55198);
        let assign42110_e55203: f64 = (-230.25850929940458);
        let assign42110_e55205: f64 = (-locals.var_sp_s_xbar);
        let assign42110_e55206: f64 = (assign42110_e55203 - assign42110_e55205);
        let assign42110_e55209: f64 = (-230.25850929940458);
        let assign42110_e55211: f64 = (-locals.var_sp_s_xbar);
        let assign42110_e55212: f64 = (assign42110_e55209 - assign42110_e55211);
        let assign42110_e55214: f64 = (assign42110_e55212 * 0.3333333333333333);
        let assign42110_e55215: f64 = (1.0 + assign42110_e55214);
        let assign42110_e55216: f64 = (assign42110_e55206 * assign42110_e55215);
        let assign42110_e55217: f64 = (0.5 * assign42110_e55216);
        let assign42110_e55218: f64 = (1.0 + assign42110_e55217);
        let assign42110_e55219: f64 = (assign42110_e55199 * assign42110_e55218);
        let assign42110_e55220: f64 = (1.0 + assign42110_e55219);
        let assign42110_e55221: f64 = (1e-100 / assign42110_e55220);
        (assign42110_e55221, (-((1e-100 * (((-(-locals.var_sp_s_xbar_dn4)) * assign42110_e55218) + (assign42110_e55199 * (0.5 * (((-(-locals.var_sp_s_xbar_dn4)) * assign42110_e55215) + (assign42110_e55206 * ((-(-locals.var_sp_s_xbar_dn4)) * 0.3333333333333333))))))) / (assign42110_e55220 * assign42110_e55220))), (-((1e-100 * (((-(-locals.var_sp_s_xbar_dn6)) * assign42110_e55218) + (assign42110_e55199 * (0.5 * (((-(-locals.var_sp_s_xbar_dn6)) * assign42110_e55215) + (assign42110_e55206 * ((-(-locals.var_sp_s_xbar_dn6)) * 0.3333333333333333))))))) / (assign42110_e55220 * assign42110_e55220))), (-((1e-100 * (((-(-locals.var_sp_s_xbar_dn7)) * assign42110_e55218) + (assign42110_e55199 * (0.5 * (((-(-locals.var_sp_s_xbar_dn7)) * assign42110_e55215) + (assign42110_e55206 * ((-(-locals.var_sp_s_xbar_dn7)) * 0.3333333333333333))))))) / (assign42110_e55220 * assign42110_e55220))), (-((1e-100 * (((-(-locals.var_sp_s_xbar_dn8)) * assign42110_e55218) + (assign42110_e55199 * (0.5 * (((-(-locals.var_sp_s_xbar_dn8)) * assign42110_e55215) + (assign42110_e55206 * ((-(-locals.var_sp_s_xbar_dn8)) * 0.3333333333333333))))))) / (assign42110_e55220 * assign42110_e55220))), (-((1e-100 * (((-(-locals.var_sp_s_xbar_dn9)) * assign42110_e55218) + (assign42110_e55199 * (0.5 * (((-(-locals.var_sp_s_xbar_dn9)) * assign42110_e55215) + (assign42110_e55206 * ((-(-locals.var_sp_s_xbar_dn9)) * 0.3333333333333333))))))) / (assign42110_e55220 * assign42110_e55220))),)
    } else {
        (locals.var_sp_s_temp, locals.var_sp_s_temp_dn4, locals.var_sp_s_temp_dn6, locals.var_sp_s_temp_dn7, locals.var_sp_s_temp_dn8, locals.var_sp_s_temp_dn9,)
    }
};
        locals.var_sp_s_temp = assign42110_e55223;
        locals.var_sp_s_temp_dn4 = assign42110_e55223_d_n4;
        locals.var_sp_s_temp_dn6 = assign42110_e55223_d_n6;
        locals.var_sp_s_temp_dn7 = assign42110_e55223_d_n7;
        locals.var_sp_s_temp_dn8 = assign42110_e55223_d_n8;
        locals.var_sp_s_temp_dn9 = assign42110_e55223_d_n9;
        locals.var_sp_s_temp_rv = 0.0;

        let (assign42120_e55233, assign42120_e55233_d_n4, assign42120_e55233_d_n6, assign42120_e55233_d_n7, assign42120_e55233_d_n8, assign42120_e55233_d_n9,) = {
    if ((locals.var_guard1199 == 0.0) && (locals.var_guard1200 == 0.0)) {
        let assign42120_e55231: f64 = (1.0 - locals.var_sp_s_temp);
        (assign42120_e55231, (-locals.var_sp_s_temp_dn4), (-locals.var_sp_s_temp_dn6), (-locals.var_sp_s_temp_dn7), (-locals.var_sp_s_temp_dn8), (-locals.var_sp_s_temp_dn9),)
    } else {
        (locals.var_sp_s_w, locals.var_sp_s_w_dn4, locals.var_sp_s_w_dn6, locals.var_sp_s_w_dn7, locals.var_sp_s_w_dn8, locals.var_sp_s_w_dn9,)
    }
};
        locals.var_sp_s_w = assign42120_e55233;
        locals.var_sp_s_w_dn4 = assign42120_e55233_d_n4;
        locals.var_sp_s_w_dn6 = assign42120_e55233_d_n6;
        locals.var_sp_s_w_dn7 = assign42120_e55233_d_n7;
        locals.var_sp_s_w_dn8 = assign42120_e55233_d_n8;
        locals.var_sp_s_w_dn9 = assign42120_e55233_d_n9;
        locals.var_sp_s_w_rv = 0.0;

        let (assign42130_e55256, assign42130_e55256_d_n4, assign42130_e55256_d_n6, assign42130_e55256_d_n7, assign42130_e55256_d_n8, assign42130_e55256_d_n9,) = {
    if ((locals.var_guard1199 == 0.0) && (locals.var_guard1200 == 0.0)) {
        let assign42130_e55242: f64 = (locals.var_gf2 * 0.5);
        let assign42130_e55243: f64 = (locals.var_xg + assign42130_e55242);
        let assign42130_e55248: f64 = (locals.var_gf2 * 0.25);
        let assign42130_e55249: f64 = (locals.var_xg + assign42130_e55248);
        let assign42130_e55251: f64 = (assign42130_e55249 - locals.var_sp_s_w);
        let assign42130_e55252: f64 = (assign42130_e55251).sqrt();
        let assign42130_e55253: f64 = (locals.var_gf * assign42130_e55252);
        let assign42130_e55254: f64 = (assign42130_e55243 - assign42130_e55253);
        (assign42130_e55254, ((locals.var_xg_dn4 + (locals.var_gf2_dn4 * 0.5)) - ((locals.var_gf_dn4 * assign42130_e55252) + (locals.var_gf * (((locals.var_xg_dn4 + (locals.var_gf2_dn4 * 0.25)) - locals.var_sp_s_w_dn4) / (2.0 * assign42130_e55252))))), ((locals.var_xg_dn6 + (locals.var_gf2_dn6 * 0.5)) - ((locals.var_gf_dn6 * assign42130_e55252) + (locals.var_gf * (((locals.var_xg_dn6 + (locals.var_gf2_dn6 * 0.25)) - locals.var_sp_s_w_dn6) / (2.0 * assign42130_e55252))))), ((locals.var_xg_dn7 + (locals.var_gf2_dn7 * 0.5)) - ((locals.var_gf_dn7 * assign42130_e55252) + (locals.var_gf * (((locals.var_xg_dn7 + (locals.var_gf2_dn7 * 0.25)) - locals.var_sp_s_w_dn7) / (2.0 * assign42130_e55252))))), ((locals.var_xg_dn8 + (locals.var_gf2_dn8 * 0.5)) - ((locals.var_gf_dn8 * assign42130_e55252) + (locals.var_gf * (((locals.var_xg_dn8 + (locals.var_gf2_dn8 * 0.25)) - locals.var_sp_s_w_dn8) / (2.0 * assign42130_e55252))))), ((locals.var_xg_dn9 + (locals.var_gf2_dn9 * 0.5)) - ((locals.var_gf_dn9 * assign42130_e55252) + (locals.var_gf * (((locals.var_xg_dn9 + (locals.var_gf2_dn9 * 0.25)) - locals.var_sp_s_w_dn9) / (2.0 * assign42130_e55252))))),)
    } else {
        (locals.var_sp_s_x1, locals.var_sp_s_x1_dn4, locals.var_sp_s_x1_dn6, locals.var_sp_s_x1_dn7, locals.var_sp_s_x1_dn8, locals.var_sp_s_x1_dn9,)
    }
};
        locals.var_sp_s_x1 = assign42130_e55256;
        locals.var_sp_s_x1_dn4 = assign42130_e55256_d_n4;
        locals.var_sp_s_x1_dn6 = assign42130_e55256_d_n6;
        locals.var_sp_s_x1_dn7 = assign42130_e55256_d_n7;
        locals.var_sp_s_x1_dn8 = assign42130_e55256_d_n8;
        locals.var_sp_s_x1_dn9 = assign42130_e55256_d_n9;
        locals.var_sp_s_x1_rv = 0.0;

        let (assign42140_e55266, assign42140_e55266_d_n4, assign42140_e55266_d_n6, assign42140_e55266_d_n7, assign42140_e55266_d_n8, assign42140_e55266_d_n9,) = {
    if ((locals.var_guard1199 == 0.0) && (locals.var_guard1200 == 0.0)) {
        let assign42140_e55264: f64 = (locals.var_xn_s + 3.0);
        (assign42140_e55264, locals.var_xn_s_dn4, locals.var_xn_s_dn6, locals.var_xn_s_dn7, locals.var_xn_s_dn8, locals.var_xn_s_dn9,)
    } else {
        (locals.var_sp_s_bx, locals.var_sp_s_bx_dn4, locals.var_sp_s_bx_dn6, locals.var_sp_s_bx_dn7, locals.var_sp_s_bx_dn8, locals.var_sp_s_bx_dn9,)
    }
};
        locals.var_sp_s_bx = assign42140_e55266;
        locals.var_sp_s_bx_dn4 = assign42140_e55266_d_n4;
        locals.var_sp_s_bx_dn6 = assign42140_e55266_d_n6;
        locals.var_sp_s_bx_dn7 = assign42140_e55266_d_n7;
        locals.var_sp_s_bx_dn8 = assign42140_e55266_d_n8;
        locals.var_sp_s_bx_dn9 = assign42140_e55266_d_n9;
        locals.var_sp_s_bx_rv = 0.0;

        let (assign42150_e55300, assign42150_e55300_d_n4, assign42150_e55300_d_n6, assign42150_e55300_d_n7, assign42150_e55300_d_n8, assign42150_e55300_d_n9,) = {
    if ((locals.var_guard1199 == 0.0) && (locals.var_guard1200 == 0.0)) {
        let assign42150_e55275: f64 = (locals.var_sp_s_x1 + locals.var_sp_s_bx);
        let assign42150_e55278: f64 = (locals.var_sp_s_x1 - locals.var_sp_s_bx);
        let assign42150_e55281: f64 = (locals.var_sp_s_x1 - locals.var_sp_s_bx);
        let assign42150_e55282: f64 = (assign42150_e55278 * assign42150_e55281);
        let assign42150_e55284: f64 = (assign42150_e55282 + 5.0);
        let assign42150_e55285: f64 = (assign42150_e55284).sqrt();
        let assign42150_e55286: f64 = (assign42150_e55275 - assign42150_e55285);
        let assign42150_e55287: f64 = (0.5 * assign42150_e55286);
        let assign42150_e55292: f64 = (locals.var_sp_s_bx * locals.var_sp_s_bx);
        let assign42150_e55294: f64 = (assign42150_e55292 + 5.0);
        let assign42150_e55295: f64 = (assign42150_e55294).sqrt();
        let assign42150_e55296: f64 = (locals.var_sp_s_bx - assign42150_e55295);
        let assign42150_e55297: f64 = (0.5 * assign42150_e55296);
        let assign42150_e55298: f64 = (assign42150_e55287 - assign42150_e55297);
        (assign42150_e55298, ((0.5 * ((locals.var_sp_s_x1_dn4 + locals.var_sp_s_bx_dn4) - ((((locals.var_sp_s_x1_dn4 - locals.var_sp_s_bx_dn4) * assign42150_e55281) + (assign42150_e55278 * (locals.var_sp_s_x1_dn4 - locals.var_sp_s_bx_dn4))) / (2.0 * assign42150_e55285)))) - (0.5 * (locals.var_sp_s_bx_dn4 - (((locals.var_sp_s_bx_dn4 * locals.var_sp_s_bx) + (locals.var_sp_s_bx * locals.var_sp_s_bx_dn4)) / (2.0 * assign42150_e55295))))), ((0.5 * ((locals.var_sp_s_x1_dn6 + locals.var_sp_s_bx_dn6) - ((((locals.var_sp_s_x1_dn6 - locals.var_sp_s_bx_dn6) * assign42150_e55281) + (assign42150_e55278 * (locals.var_sp_s_x1_dn6 - locals.var_sp_s_bx_dn6))) / (2.0 * assign42150_e55285)))) - (0.5 * (locals.var_sp_s_bx_dn6 - (((locals.var_sp_s_bx_dn6 * locals.var_sp_s_bx) + (locals.var_sp_s_bx * locals.var_sp_s_bx_dn6)) / (2.0 * assign42150_e55295))))), ((0.5 * ((locals.var_sp_s_x1_dn7 + locals.var_sp_s_bx_dn7) - ((((locals.var_sp_s_x1_dn7 - locals.var_sp_s_bx_dn7) * assign42150_e55281) + (assign42150_e55278 * (locals.var_sp_s_x1_dn7 - locals.var_sp_s_bx_dn7))) / (2.0 * assign42150_e55285)))) - (0.5 * (locals.var_sp_s_bx_dn7 - (((locals.var_sp_s_bx_dn7 * locals.var_sp_s_bx) + (locals.var_sp_s_bx * locals.var_sp_s_bx_dn7)) / (2.0 * assign42150_e55295))))), ((0.5 * ((locals.var_sp_s_x1_dn8 + locals.var_sp_s_bx_dn8) - ((((locals.var_sp_s_x1_dn8 - locals.var_sp_s_bx_dn8) * assign42150_e55281) + (assign42150_e55278 * (locals.var_sp_s_x1_dn8 - locals.var_sp_s_bx_dn8))) / (2.0 * assign42150_e55285)))) - (0.5 * (locals.var_sp_s_bx_dn8 - (((locals.var_sp_s_bx_dn8 * locals.var_sp_s_bx) + (locals.var_sp_s_bx * locals.var_sp_s_bx_dn8)) / (2.0 * assign42150_e55295))))), ((0.5 * ((locals.var_sp_s_x1_dn9 + locals.var_sp_s_bx_dn9) - ((((locals.var_sp_s_x1_dn9 - locals.var_sp_s_bx_dn9) * assign42150_e55281) + (assign42150_e55278 * (locals.var_sp_s_x1_dn9 - locals.var_sp_s_bx_dn9))) / (2.0 * assign42150_e55285)))) - (0.5 * (locals.var_sp_s_bx_dn9 - (((locals.var_sp_s_bx_dn9 * locals.var_sp_s_bx) + (locals.var_sp_s_bx * locals.var_sp_s_bx_dn9)) / (2.0 * assign42150_e55295))))),)
    } else {
        (locals.var_sp_s_eta, locals.var_sp_s_eta_dn4, locals.var_sp_s_eta_dn6, locals.var_sp_s_eta_dn7, locals.var_sp_s_eta_dn8, locals.var_sp_s_eta_dn9,)
    }
};
        locals.var_sp_s_eta = assign42150_e55300;
        locals.var_sp_s_eta_dn4 = assign42150_e55300_d_n4;
        locals.var_sp_s_eta_dn6 = assign42150_e55300_d_n6;
        locals.var_sp_s_eta_dn7 = assign42150_e55300_d_n7;
        locals.var_sp_s_eta_dn8 = assign42150_e55300_d_n8;
        locals.var_sp_s_eta_dn9 = assign42150_e55300_d_n9;
        locals.var_sp_s_eta_rv = 0.0;

        let (assign42160_e55310, assign42160_e55310_d_n4, assign42160_e55310_d_n6, assign42160_e55310_d_n7, assign42160_e55310_d_n8, assign42160_e55310_d_n9,) = {
    if ((locals.var_guard1199 == 0.0) && (locals.var_guard1200 == 0.0)) {
        let assign42160_e55308: f64 = (locals.var_xg - locals.var_sp_s_eta);
        (assign42160_e55308, (locals.var_xg_dn4 - locals.var_sp_s_eta_dn4), (locals.var_xg_dn6 - locals.var_sp_s_eta_dn6), (locals.var_xg_dn7 - locals.var_sp_s_eta_dn7), (locals.var_xg_dn8 - locals.var_sp_s_eta_dn8), (locals.var_xg_dn9 - locals.var_sp_s_eta_dn9),)
    } else {
        (locals.var_sp_s_temp, locals.var_sp_s_temp_dn4, locals.var_sp_s_temp_dn6, locals.var_sp_s_temp_dn7, locals.var_sp_s_temp_dn8, locals.var_sp_s_temp_dn9,)
    }
};
        locals.var_sp_s_temp = assign42160_e55310;
        locals.var_sp_s_temp_dn4 = assign42160_e55310_d_n4;
        locals.var_sp_s_temp_dn6 = assign42160_e55310_d_n6;
        locals.var_sp_s_temp_dn7 = assign42160_e55310_d_n7;
        locals.var_sp_s_temp_dn8 = assign42160_e55310_d_n8;
        locals.var_sp_s_temp_dn9 = assign42160_e55310_d_n9;
        locals.var_sp_s_temp_rv = 0.0;

        let (assign42170_e55320, assign42170_e55320_d_n4, assign42170_e55320_d_n6, assign42170_e55320_d_n7, assign42170_e55320_d_n8, assign42170_e55320_d_n9,) = {
    if ((locals.var_guard1199 == 0.0) && (locals.var_guard1200 == 0.0)) {
        let assign42170_e55317: f64 = (-locals.var_sp_s_eta);
        let assign42170_e55318: f64 = (assign42170_e55317).exp();
        (assign42170_e55318, (assign42170_e55318 * (-locals.var_sp_s_eta_dn4)), (assign42170_e55318 * (-locals.var_sp_s_eta_dn6)), (assign42170_e55318 * (-locals.var_sp_s_eta_dn7)), (assign42170_e55318 * (-locals.var_sp_s_eta_dn8)), (assign42170_e55318 * (-locals.var_sp_s_eta_dn9)),)
    } else {
        (locals.var_sp_s_temp1, locals.var_sp_s_temp1_dn4, locals.var_sp_s_temp1_dn6, locals.var_sp_s_temp1_dn7, locals.var_sp_s_temp1_dn8, locals.var_sp_s_temp1_dn9,)
    }
};
        locals.var_sp_s_temp1 = assign42170_e55320;
        locals.var_sp_s_temp1_dn4 = assign42170_e55320_d_n4;
        locals.var_sp_s_temp1_dn6 = assign42170_e55320_d_n6;
        locals.var_sp_s_temp1_dn7 = assign42170_e55320_d_n7;
        locals.var_sp_s_temp1_dn8 = assign42170_e55320_d_n8;
        locals.var_sp_s_temp1_dn9 = assign42170_e55320_d_n9;
        locals.var_sp_s_temp1_rv = 0.0;

        let (assign42180_e55334, assign42180_e55334_d_n4, assign42180_e55334_d_n6, assign42180_e55334_d_n7, assign42180_e55334_d_n8, assign42180_e55334_d_n9,) = {
    if ((locals.var_guard1199 == 0.0) && (locals.var_guard1200 == 0.0)) {
        let assign42180_e55330: f64 = (locals.var_sp_s_eta * locals.var_sp_s_eta);
        let assign42180_e55331: f64 = (2.0 + assign42180_e55330);
        let assign42180_e55332: f64 = (1.0 / assign42180_e55331);
        (assign42180_e55332, (-(((locals.var_sp_s_eta_dn4 * locals.var_sp_s_eta) + (locals.var_sp_s_eta * locals.var_sp_s_eta_dn4)) / (assign42180_e55331 * assign42180_e55331))), (-(((locals.var_sp_s_eta_dn6 * locals.var_sp_s_eta) + (locals.var_sp_s_eta * locals.var_sp_s_eta_dn6)) / (assign42180_e55331 * assign42180_e55331))), (-(((locals.var_sp_s_eta_dn7 * locals.var_sp_s_eta) + (locals.var_sp_s_eta * locals.var_sp_s_eta_dn7)) / (assign42180_e55331 * assign42180_e55331))), (-(((locals.var_sp_s_eta_dn8 * locals.var_sp_s_eta) + (locals.var_sp_s_eta * locals.var_sp_s_eta_dn8)) / (assign42180_e55331 * assign42180_e55331))), (-(((locals.var_sp_s_eta_dn9 * locals.var_sp_s_eta) + (locals.var_sp_s_eta * locals.var_sp_s_eta_dn9)) / (assign42180_e55331 * assign42180_e55331))),)
    } else {
        (locals.var_sp_s_temp2, locals.var_sp_s_temp2_dn4, locals.var_sp_s_temp2_dn6, locals.var_sp_s_temp2_dn7, locals.var_sp_s_temp2_dn8, locals.var_sp_s_temp2_dn9,)
    }
};
        locals.var_sp_s_temp2 = assign42180_e55334;
        locals.var_sp_s_temp2_dn4 = assign42180_e55334_d_n4;
        locals.var_sp_s_temp2_dn6 = assign42180_e55334_d_n6;
        locals.var_sp_s_temp2_dn7 = assign42180_e55334_d_n7;
        locals.var_sp_s_temp2_dn8 = assign42180_e55334_d_n8;
        locals.var_sp_s_temp2_dn9 = assign42180_e55334_d_n9;
        locals.var_sp_s_temp2_rv = 0.0;

        let (assign42190_e55346, assign42190_e55346_d_n4, assign42190_e55346_d_n6, assign42190_e55346_d_n7, assign42190_e55346_d_n8, assign42190_e55346_d_n9,) = {
    if ((locals.var_guard1199 == 0.0) && (locals.var_guard1200 == 0.0)) {
        let assign42190_e55342: f64 = (locals.var_sp_s_eta * locals.var_sp_s_eta);
        let assign42190_e55344: f64 = (assign42190_e55342 * locals.var_sp_s_temp2);
        (assign42190_e55344, ((((locals.var_sp_s_eta_dn4 * locals.var_sp_s_eta) + (locals.var_sp_s_eta * locals.var_sp_s_eta_dn4)) * locals.var_sp_s_temp2) + (assign42190_e55342 * locals.var_sp_s_temp2_dn4)), ((((locals.var_sp_s_eta_dn6 * locals.var_sp_s_eta) + (locals.var_sp_s_eta * locals.var_sp_s_eta_dn6)) * locals.var_sp_s_temp2) + (assign42190_e55342 * locals.var_sp_s_temp2_dn6)), ((((locals.var_sp_s_eta_dn7 * locals.var_sp_s_eta) + (locals.var_sp_s_eta * locals.var_sp_s_eta_dn7)) * locals.var_sp_s_temp2) + (assign42190_e55342 * locals.var_sp_s_temp2_dn7)), ((((locals.var_sp_s_eta_dn8 * locals.var_sp_s_eta) + (locals.var_sp_s_eta * locals.var_sp_s_eta_dn8)) * locals.var_sp_s_temp2) + (assign42190_e55342 * locals.var_sp_s_temp2_dn8)), ((((locals.var_sp_s_eta_dn9 * locals.var_sp_s_eta) + (locals.var_sp_s_eta * locals.var_sp_s_eta_dn9)) * locals.var_sp_s_temp2) + (assign42190_e55342 * locals.var_sp_s_temp2_dn9)),)
    } else {
        (locals.var_sp_s_xi0, locals.var_sp_s_xi0_dn4, locals.var_sp_s_xi0_dn6, locals.var_sp_s_xi0_dn7, locals.var_sp_s_xi0_dn8, locals.var_sp_s_xi0_dn9,)
    }
};
        locals.var_sp_s_xi0 = assign42190_e55346;
        locals.var_sp_s_xi0_dn4 = assign42190_e55346_d_n4;
        locals.var_sp_s_xi0_dn6 = assign42190_e55346_d_n6;
        locals.var_sp_s_xi0_dn7 = assign42190_e55346_d_n7;
        locals.var_sp_s_xi0_dn8 = assign42190_e55346_d_n8;
        locals.var_sp_s_xi0_dn9 = assign42190_e55346_d_n9;
        locals.var_sp_s_xi0_rv = 0.0;

        let (assign42200_e55360, assign42200_e55360_d_n4, assign42200_e55360_d_n6, assign42200_e55360_d_n7, assign42200_e55360_d_n8, assign42200_e55360_d_n9,) = {
    if ((locals.var_guard1199 == 0.0) && (locals.var_guard1200 == 0.0)) {
        let assign42200_e55355: f64 = (locals.var_sp_s_eta * locals.var_sp_s_temp2);
        let assign42200_e55357: f64 = (assign42200_e55355 * locals.var_sp_s_temp2);
        let assign42200_e55358: f64 = (4.0 * assign42200_e55357);
        (assign42200_e55358, (4.0 * ((((locals.var_sp_s_eta_dn4 * locals.var_sp_s_temp2) + (locals.var_sp_s_eta * locals.var_sp_s_temp2_dn4)) * locals.var_sp_s_temp2) + (assign42200_e55355 * locals.var_sp_s_temp2_dn4))), (4.0 * ((((locals.var_sp_s_eta_dn6 * locals.var_sp_s_temp2) + (locals.var_sp_s_eta * locals.var_sp_s_temp2_dn6)) * locals.var_sp_s_temp2) + (assign42200_e55355 * locals.var_sp_s_temp2_dn6))), (4.0 * ((((locals.var_sp_s_eta_dn7 * locals.var_sp_s_temp2) + (locals.var_sp_s_eta * locals.var_sp_s_temp2_dn7)) * locals.var_sp_s_temp2) + (assign42200_e55355 * locals.var_sp_s_temp2_dn7))), (4.0 * ((((locals.var_sp_s_eta_dn8 * locals.var_sp_s_temp2) + (locals.var_sp_s_eta * locals.var_sp_s_temp2_dn8)) * locals.var_sp_s_temp2) + (assign42200_e55355 * locals.var_sp_s_temp2_dn8))), (4.0 * ((((locals.var_sp_s_eta_dn9 * locals.var_sp_s_temp2) + (locals.var_sp_s_eta * locals.var_sp_s_temp2_dn9)) * locals.var_sp_s_temp2) + (assign42200_e55355 * locals.var_sp_s_temp2_dn9))),)
    } else {
        (locals.var_sp_s_xi1, locals.var_sp_s_xi1_dn4, locals.var_sp_s_xi1_dn6, locals.var_sp_s_xi1_dn7, locals.var_sp_s_xi1_dn8, locals.var_sp_s_xi1_dn9,)
    }
};
        locals.var_sp_s_xi1 = assign42200_e55360;
        locals.var_sp_s_xi1_dn4 = assign42200_e55360_d_n4;
        locals.var_sp_s_xi1_dn6 = assign42200_e55360_d_n6;
        locals.var_sp_s_xi1_dn7 = assign42200_e55360_d_n7;
        locals.var_sp_s_xi1_dn8 = assign42200_e55360_d_n8;
        locals.var_sp_s_xi1_dn9 = assign42200_e55360_d_n9;
        locals.var_sp_s_xi1_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_20(
        locals: &mut StampLocals,
    ) {
        let (assign42210_e55378, assign42210_e55378_d_n4, assign42210_e55378_d_n6, assign42210_e55378_d_n7, assign42210_e55378_d_n8, assign42210_e55378_d_n9,) = {
    if ((locals.var_guard1199 == 0.0) && (locals.var_guard1200 == 0.0)) {
        let assign42210_e55368: f64 = (8.0 * locals.var_sp_s_temp2);
        let assign42210_e55371: f64 = (12.0 * locals.var_sp_s_xi0);
        let assign42210_e55372: f64 = (assign42210_e55368 - assign42210_e55371);
        let assign42210_e55374: f64 = (assign42210_e55372 * locals.var_sp_s_temp2);
        let assign42210_e55376: f64 = (assign42210_e55374 * locals.var_sp_s_temp2);
        (assign42210_e55376, ((((((8.0 * locals.var_sp_s_temp2_dn4) - (12.0 * locals.var_sp_s_xi0_dn4)) * locals.var_sp_s_temp2) + (assign42210_e55372 * locals.var_sp_s_temp2_dn4)) * locals.var_sp_s_temp2) + (assign42210_e55374 * locals.var_sp_s_temp2_dn4)), ((((((8.0 * locals.var_sp_s_temp2_dn6) - (12.0 * locals.var_sp_s_xi0_dn6)) * locals.var_sp_s_temp2) + (assign42210_e55372 * locals.var_sp_s_temp2_dn6)) * locals.var_sp_s_temp2) + (assign42210_e55374 * locals.var_sp_s_temp2_dn6)), ((((((8.0 * locals.var_sp_s_temp2_dn7) - (12.0 * locals.var_sp_s_xi0_dn7)) * locals.var_sp_s_temp2) + (assign42210_e55372 * locals.var_sp_s_temp2_dn7)) * locals.var_sp_s_temp2) + (assign42210_e55374 * locals.var_sp_s_temp2_dn7)), ((((((8.0 * locals.var_sp_s_temp2_dn8) - (12.0 * locals.var_sp_s_xi0_dn8)) * locals.var_sp_s_temp2) + (assign42210_e55372 * locals.var_sp_s_temp2_dn8)) * locals.var_sp_s_temp2) + (assign42210_e55374 * locals.var_sp_s_temp2_dn8)), ((((((8.0 * locals.var_sp_s_temp2_dn9) - (12.0 * locals.var_sp_s_xi0_dn9)) * locals.var_sp_s_temp2) + (assign42210_e55372 * locals.var_sp_s_temp2_dn9)) * locals.var_sp_s_temp2) + (assign42210_e55374 * locals.var_sp_s_temp2_dn9)),)
    } else {
        (locals.var_sp_s_xi2, locals.var_sp_s_xi2_dn4, locals.var_sp_s_xi2_dn6, locals.var_sp_s_xi2_dn7, locals.var_sp_s_xi2_dn8, locals.var_sp_s_xi2_dn9,)
    }
};
        locals.var_sp_s_xi2 = assign42210_e55378;
        locals.var_sp_s_xi2_dn4 = assign42210_e55378_d_n4;
        locals.var_sp_s_xi2_dn6 = assign42210_e55378_d_n6;
        locals.var_sp_s_xi2_dn7 = assign42210_e55378_d_n7;
        locals.var_sp_s_xi2_dn8 = assign42210_e55378_d_n8;
        locals.var_sp_s_xi2_dn9 = assign42210_e55378_d_n9;
        locals.var_sp_s_xi2_rv = 0.0;

        let (assign42220_e55427, assign42220_e55427_d_n4, assign42220_e55427_d_n6, assign42220_e55427_d_n7, assign42220_e55427_d_n8, assign42220_e55427_d_n9,) = {
    if ((locals.var_guard1199 == 0.0) && (locals.var_guard1200 == 0.0)) {
        let assign42220_e55387: f64 = (locals.var_sp_s_temp * locals.var_sp_s_temp);
        let assign42220_e55391: f64 = (locals.var_sp_s_temp1 + locals.var_sp_s_eta);
        let assign42220_e55393: f64 = (assign42220_e55391 - 1.0);
        let assign42220_e55397: f64 = (locals.var_sp_s_eta + 1.0);
        let assign42220_e55399: f64 = (assign42220_e55397 + locals.var_sp_s_xi0);
        let assign42220_e55400: f64 = (locals.var_delta_ns * assign42220_e55399);
        let assign42220_e55401: f64 = (assign42220_e55393 - assign42220_e55400);
        let assign42220_e55402: f64 = (locals.var_gf2 * assign42220_e55401);
        let assign42220_e55403: f64 = (assign42220_e55387 - assign42220_e55402);
        let (assign42220_e55425, assign42220_e55425_d_n4, assign42220_e55425_d_n6, assign42220_e55425_d_n7, assign42220_e55425_d_n8, assign42220_e55425_d_n9,) = {
            if (1e-40 > assign42220_e55403) {
                (1e-40, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign42220_e55408: f64 = (locals.var_sp_s_temp * locals.var_sp_s_temp);
                let assign42220_e55412: f64 = (locals.var_sp_s_temp1 + locals.var_sp_s_eta);
                let assign42220_e55414: f64 = (assign42220_e55412 - 1.0);
                let assign42220_e55418: f64 = (locals.var_sp_s_eta + 1.0);
                let assign42220_e55420: f64 = (assign42220_e55418 + locals.var_sp_s_xi0);
                let assign42220_e55421: f64 = (locals.var_delta_ns * assign42220_e55420);
                let assign42220_e55422: f64 = (assign42220_e55414 - assign42220_e55421);
                let assign42220_e55423: f64 = (locals.var_gf2 * assign42220_e55422);
                let assign42220_e55424: f64 = (assign42220_e55408 - assign42220_e55423);
                (assign42220_e55424, (((locals.var_sp_s_temp_dn4 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn4)) - ((locals.var_gf2_dn4 * assign42220_e55422) + (locals.var_gf2 * ((locals.var_sp_s_temp1_dn4 + locals.var_sp_s_eta_dn4) - ((locals.var_delta_ns_dn4 * assign42220_e55420) + (locals.var_delta_ns * (locals.var_sp_s_eta_dn4 + locals.var_sp_s_xi0_dn4))))))), (((locals.var_sp_s_temp_dn6 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn6)) - ((locals.var_gf2_dn6 * assign42220_e55422) + (locals.var_gf2 * ((locals.var_sp_s_temp1_dn6 + locals.var_sp_s_eta_dn6) - ((locals.var_delta_ns_dn6 * assign42220_e55420) + (locals.var_delta_ns * (locals.var_sp_s_eta_dn6 + locals.var_sp_s_xi0_dn6))))))), (((locals.var_sp_s_temp_dn7 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn7)) - ((locals.var_gf2_dn7 * assign42220_e55422) + (locals.var_gf2 * ((locals.var_sp_s_temp1_dn7 + locals.var_sp_s_eta_dn7) - ((locals.var_delta_ns_dn7 * assign42220_e55420) + (locals.var_delta_ns * (locals.var_sp_s_eta_dn7 + locals.var_sp_s_xi0_dn7))))))), (((locals.var_sp_s_temp_dn8 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn8)) - ((locals.var_gf2_dn8 * assign42220_e55422) + (locals.var_gf2 * ((locals.var_sp_s_temp1_dn8 + locals.var_sp_s_eta_dn8) - ((locals.var_delta_ns_dn8 * assign42220_e55420) + (locals.var_delta_ns * (locals.var_sp_s_eta_dn8 + locals.var_sp_s_xi0_dn8))))))), (((locals.var_sp_s_temp_dn9 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn9)) - ((locals.var_gf2_dn9 * assign42220_e55422) + (locals.var_gf2 * ((locals.var_sp_s_temp1_dn9 + locals.var_sp_s_eta_dn9) - ((locals.var_delta_ns_dn9 * assign42220_e55420) + (locals.var_delta_ns * (locals.var_sp_s_eta_dn9 + locals.var_sp_s_xi0_dn9))))))),)
            }
        };
        (assign42220_e55425, assign42220_e55425_d_n4, assign42220_e55425_d_n6, assign42220_e55425_d_n7, assign42220_e55425_d_n8, assign42220_e55425_d_n9,)
    } else {
        (locals.var_sp_s_a, locals.var_sp_s_a_dn4, locals.var_sp_s_a_dn6, locals.var_sp_s_a_dn7, locals.var_sp_s_a_dn8, locals.var_sp_s_a_dn9,)
    }
};
        locals.var_sp_s_a = assign42220_e55427;
        locals.var_sp_s_a_dn4 = assign42220_e55427_d_n4;
        locals.var_sp_s_a_dn6 = assign42220_e55427_d_n6;
        locals.var_sp_s_a_dn7 = assign42220_e55427_d_n7;
        locals.var_sp_s_a_dn8 = assign42220_e55427_d_n8;
        locals.var_sp_s_a_dn9 = assign42220_e55427_d_n9;
        locals.var_sp_s_a_rv = 0.0;

        let (assign42230_e55445, assign42230_e55445_d_n4, assign42230_e55445_d_n6, assign42230_e55445_d_n7, assign42230_e55445_d_n8, assign42230_e55445_d_n9,) = {
    if ((locals.var_guard1199 == 0.0) && (locals.var_guard1200 == 0.0)) {
        let assign42230_e55439: f64 = (locals.var_delta_ns * locals.var_sp_s_xi2);
        let assign42230_e55440: f64 = (locals.var_sp_s_temp1 - assign42230_e55439);
        let assign42230_e55441: f64 = (locals.var_gf2 * assign42230_e55440);
        let assign42230_e55442: f64 = (0.5 * assign42230_e55441);
        let assign42230_e55443: f64 = (1.0 - assign42230_e55442);
        (assign42230_e55443, (-(0.5 * ((locals.var_gf2_dn4 * assign42230_e55440) + (locals.var_gf2 * (locals.var_sp_s_temp1_dn4 - ((locals.var_delta_ns_dn4 * locals.var_sp_s_xi2) + (locals.var_delta_ns * locals.var_sp_s_xi2_dn4))))))), (-(0.5 * ((locals.var_gf2_dn6 * assign42230_e55440) + (locals.var_gf2 * (locals.var_sp_s_temp1_dn6 - ((locals.var_delta_ns_dn6 * locals.var_sp_s_xi2) + (locals.var_delta_ns * locals.var_sp_s_xi2_dn6))))))), (-(0.5 * ((locals.var_gf2_dn7 * assign42230_e55440) + (locals.var_gf2 * (locals.var_sp_s_temp1_dn7 - ((locals.var_delta_ns_dn7 * locals.var_sp_s_xi2) + (locals.var_delta_ns * locals.var_sp_s_xi2_dn7))))))), (-(0.5 * ((locals.var_gf2_dn8 * assign42230_e55440) + (locals.var_gf2 * (locals.var_sp_s_temp1_dn8 - ((locals.var_delta_ns_dn8 * locals.var_sp_s_xi2) + (locals.var_delta_ns * locals.var_sp_s_xi2_dn8))))))), (-(0.5 * ((locals.var_gf2_dn9 * assign42230_e55440) + (locals.var_gf2 * (locals.var_sp_s_temp1_dn9 - ((locals.var_delta_ns_dn9 * locals.var_sp_s_xi2) + (locals.var_delta_ns * locals.var_sp_s_xi2_dn9))))))),)
    } else {
        (locals.var_sp_s_b, locals.var_sp_s_b_dn4, locals.var_sp_s_b_dn6, locals.var_sp_s_b_dn7, locals.var_sp_s_b_dn8, locals.var_sp_s_b_dn9,)
    }
};
        locals.var_sp_s_b = assign42230_e55445;
        locals.var_sp_s_b_dn4 = assign42230_e55445_d_n4;
        locals.var_sp_s_b_dn6 = assign42230_e55445_d_n6;
        locals.var_sp_s_b_dn7 = assign42230_e55445_d_n7;
        locals.var_sp_s_b_dn8 = assign42230_e55445_d_n8;
        locals.var_sp_s_b_dn9 = assign42230_e55445_d_n9;
        locals.var_sp_s_b_rv = 0.0;

        let (assign42240_e55467, assign42240_e55467_d_n4, assign42240_e55467_d_n6, assign42240_e55467_d_n7, assign42240_e55467_d_n8, assign42240_e55467_d_n9,) = {
    if ((locals.var_guard1199 == 0.0) && (locals.var_guard1200 == 0.0)) {
        let assign42240_e55453: f64 = (2.0 * locals.var_sp_s_temp);
        let assign42240_e55457: f64 = (1.0 - locals.var_sp_s_temp1);
        let assign42240_e55461: f64 = (1.0 + locals.var_sp_s_xi1);
        let assign42240_e55462: f64 = (locals.var_delta_ns * assign42240_e55461);
        let assign42240_e55463: f64 = (assign42240_e55457 - assign42240_e55462);
        let assign42240_e55464: f64 = (locals.var_gf2 * assign42240_e55463);
        let assign42240_e55465: f64 = (assign42240_e55453 + assign42240_e55464);
        (assign42240_e55465, ((2.0 * locals.var_sp_s_temp_dn4) + ((locals.var_gf2_dn4 * assign42240_e55463) + (locals.var_gf2 * ((-locals.var_sp_s_temp1_dn4) - ((locals.var_delta_ns_dn4 * assign42240_e55461) + (locals.var_delta_ns * locals.var_sp_s_xi1_dn4)))))), ((2.0 * locals.var_sp_s_temp_dn6) + ((locals.var_gf2_dn6 * assign42240_e55463) + (locals.var_gf2 * ((-locals.var_sp_s_temp1_dn6) - ((locals.var_delta_ns_dn6 * assign42240_e55461) + (locals.var_delta_ns * locals.var_sp_s_xi1_dn6)))))), ((2.0 * locals.var_sp_s_temp_dn7) + ((locals.var_gf2_dn7 * assign42240_e55463) + (locals.var_gf2 * ((-locals.var_sp_s_temp1_dn7) - ((locals.var_delta_ns_dn7 * assign42240_e55461) + (locals.var_delta_ns * locals.var_sp_s_xi1_dn7)))))), ((2.0 * locals.var_sp_s_temp_dn8) + ((locals.var_gf2_dn8 * assign42240_e55463) + (locals.var_gf2 * ((-locals.var_sp_s_temp1_dn8) - ((locals.var_delta_ns_dn8 * assign42240_e55461) + (locals.var_delta_ns * locals.var_sp_s_xi1_dn8)))))), ((2.0 * locals.var_sp_s_temp_dn9) + ((locals.var_gf2_dn9 * assign42240_e55463) + (locals.var_gf2 * ((-locals.var_sp_s_temp1_dn9) - ((locals.var_delta_ns_dn9 * assign42240_e55461) + (locals.var_delta_ns * locals.var_sp_s_xi1_dn9)))))),)
    } else {
        (locals.var_sp_s_c, locals.var_sp_s_c_dn4, locals.var_sp_s_c_dn6, locals.var_sp_s_c_dn7, locals.var_sp_s_c_dn8, locals.var_sp_s_c_dn9,)
    }
};
        locals.var_sp_s_c = assign42240_e55467;
        locals.var_sp_s_c_dn4 = assign42240_e55467_d_n4;
        locals.var_sp_s_c_dn6 = assign42240_e55467_d_n6;
        locals.var_sp_s_c_dn7 = assign42240_e55467_d_n7;
        locals.var_sp_s_c_dn8 = assign42240_e55467_d_n8;
        locals.var_sp_s_c_dn9 = assign42240_e55467_d_n9;
        locals.var_sp_s_c_rv = 0.0;

        let (assign42250_e55482, assign42250_e55482_d_n4, assign42250_e55482_d_n6, assign42250_e55482_d_n7, assign42250_e55482_d_n8, assign42250_e55482_d_n9,) = {
    if ((locals.var_guard1199 == 0.0) && (locals.var_guard1200 == 0.0)) {
        let assign42250_e55475: f64 = (locals.var_xn_s - locals.var_sp_s_eta);
        let assign42250_e55478: f64 = (locals.var_sp_s_a / locals.var_gf2);
        let assign42250_e55479: f64 = (assign42250_e55478).ln();
        let assign42250_e55480: f64 = (assign42250_e55475 + assign42250_e55479);
        (assign42250_e55480, ((locals.var_xn_s_dn4 - locals.var_sp_s_eta_dn4) + ((((locals.var_sp_s_a_dn4 * locals.var_gf2) - (locals.var_sp_s_a * locals.var_gf2_dn4)) / (locals.var_gf2 * locals.var_gf2)) / assign42250_e55478)), ((locals.var_xn_s_dn6 - locals.var_sp_s_eta_dn6) + ((((locals.var_sp_s_a_dn6 * locals.var_gf2) - (locals.var_sp_s_a * locals.var_gf2_dn6)) / (locals.var_gf2 * locals.var_gf2)) / assign42250_e55478)), ((locals.var_xn_s_dn7 - locals.var_sp_s_eta_dn7) + ((((locals.var_sp_s_a_dn7 * locals.var_gf2) - (locals.var_sp_s_a * locals.var_gf2_dn7)) / (locals.var_gf2 * locals.var_gf2)) / assign42250_e55478)), ((locals.var_xn_s_dn8 - locals.var_sp_s_eta_dn8) + ((((locals.var_sp_s_a_dn8 * locals.var_gf2) - (locals.var_sp_s_a * locals.var_gf2_dn8)) / (locals.var_gf2 * locals.var_gf2)) / assign42250_e55478)), ((locals.var_xn_s_dn9 - locals.var_sp_s_eta_dn9) + ((((locals.var_sp_s_a_dn9 * locals.var_gf2) - (locals.var_sp_s_a * locals.var_gf2_dn9)) / (locals.var_gf2 * locals.var_gf2)) / assign42250_e55478)),)
    } else {
        (locals.var_sp_s_tau, locals.var_sp_s_tau_dn4, locals.var_sp_s_tau_dn6, locals.var_sp_s_tau_dn7, locals.var_sp_s_tau_dn8, locals.var_sp_s_tau_dn9,)
    }
};
        locals.var_sp_s_tau = assign42250_e55482;
        locals.var_sp_s_tau_dn4 = assign42250_e55482_d_n4;
        locals.var_sp_s_tau_dn6 = assign42250_e55482_d_n6;
        locals.var_sp_s_tau_dn7 = assign42250_e55482_d_n7;
        locals.var_sp_s_tau_dn8 = assign42250_e55482_d_n8;
        locals.var_sp_s_tau_dn9 = assign42250_e55482_d_n9;
        locals.var_sp_s_tau_rv = 0.0;

        let (assign42260_e55492, assign42260_e55492_d_n4, assign42260_e55492_d_n6, assign42260_e55492_d_n7, assign42260_e55492_d_n8, assign42260_e55492_d_n9,) = {
    if ((locals.var_guard1199 == 0.0) && (locals.var_guard1200 == 0.0)) {
        let assign42260_e55490: f64 = (locals.var_sp_s_a + locals.var_sp_s_c);
        (assign42260_e55490, (locals.var_sp_s_a_dn4 + locals.var_sp_s_c_dn4), (locals.var_sp_s_a_dn6 + locals.var_sp_s_c_dn6), (locals.var_sp_s_a_dn7 + locals.var_sp_s_c_dn7), (locals.var_sp_s_a_dn8 + locals.var_sp_s_c_dn8), (locals.var_sp_s_a_dn9 + locals.var_sp_s_c_dn9),)
    } else {
        (locals.var_nu, locals.var_nu_dn4, locals.var_nu_dn6, locals.var_nu_dn7, locals.var_nu_dn8, locals.var_nu_dn9,)
    }
};
        locals.var_nu = assign42260_e55492;
        locals.var_nu_dn4 = assign42260_e55492_d_n4;
        locals.var_nu_dn6 = assign42260_e55492_d_n6;
        locals.var_nu_dn7 = assign42260_e55492_d_n7;
        locals.var_nu_dn8 = assign42260_e55492_d_n8;
        locals.var_nu_dn9 = assign42260_e55492_d_n9;
        locals.var_nu_rv = 0.0;

        let (assign42270_e55514, assign42270_e55514_d_n4, assign42270_e55514_d_n6, assign42270_e55514_d_n7, assign42270_e55514_d_n8, assign42270_e55514_d_n9,) = {
    if ((locals.var_guard1199 == 0.0) && (locals.var_guard1200 == 0.0)) {
        let assign42270_e55500: f64 = (locals.var_nu * locals.var_nu);
        let assign42270_e55505: f64 = (locals.var_sp_s_c * locals.var_sp_s_c);
        let assign42270_e55506: f64 = (0.5 * assign42270_e55505);
        let assign42270_e55509: f64 = (locals.var_sp_s_a * locals.var_sp_s_b);
        let assign42270_e55510: f64 = (assign42270_e55506 - assign42270_e55509);
        let assign42270_e55511: f64 = (locals.var_sp_s_tau * assign42270_e55510);
        let assign42270_e55512: f64 = (assign42270_e55500 + assign42270_e55511);
        (assign42270_e55512, (((locals.var_nu_dn4 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn4)) + ((locals.var_sp_s_tau_dn4 * assign42270_e55510) + (locals.var_sp_s_tau * ((0.5 * ((locals.var_sp_s_c_dn4 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn4))) - ((locals.var_sp_s_a_dn4 * locals.var_sp_s_b) + (locals.var_sp_s_a * locals.var_sp_s_b_dn4)))))), (((locals.var_nu_dn6 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn6)) + ((locals.var_sp_s_tau_dn6 * assign42270_e55510) + (locals.var_sp_s_tau * ((0.5 * ((locals.var_sp_s_c_dn6 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn6))) - ((locals.var_sp_s_a_dn6 * locals.var_sp_s_b) + (locals.var_sp_s_a * locals.var_sp_s_b_dn6)))))), (((locals.var_nu_dn7 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn7)) + ((locals.var_sp_s_tau_dn7 * assign42270_e55510) + (locals.var_sp_s_tau * ((0.5 * ((locals.var_sp_s_c_dn7 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn7))) - ((locals.var_sp_s_a_dn7 * locals.var_sp_s_b) + (locals.var_sp_s_a * locals.var_sp_s_b_dn7)))))), (((locals.var_nu_dn8 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn8)) + ((locals.var_sp_s_tau_dn8 * assign42270_e55510) + (locals.var_sp_s_tau * ((0.5 * ((locals.var_sp_s_c_dn8 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn8))) - ((locals.var_sp_s_a_dn8 * locals.var_sp_s_b) + (locals.var_sp_s_a * locals.var_sp_s_b_dn8)))))), (((locals.var_nu_dn9 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn9)) + ((locals.var_sp_s_tau_dn9 * assign42270_e55510) + (locals.var_sp_s_tau * ((0.5 * ((locals.var_sp_s_c_dn9 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn9))) - ((locals.var_sp_s_a_dn9 * locals.var_sp_s_b) + (locals.var_sp_s_a * locals.var_sp_s_b_dn9)))))),)
    } else {
        (locals.var_mutau, locals.var_mutau_dn4, locals.var_mutau_dn6, locals.var_mutau_dn7, locals.var_mutau_dn8, locals.var_mutau_dn9,)
    }
};
        locals.var_mutau = assign42270_e55514;
        locals.var_mutau_dn4 = assign42270_e55514_d_n4;
        locals.var_mutau_dn6 = assign42270_e55514_d_n6;
        locals.var_mutau_dn7 = assign42270_e55514_d_n7;
        locals.var_mutau_dn8 = assign42270_e55514_d_n8;
        locals.var_mutau_dn9 = assign42270_e55514_d_n9;
        locals.var_mutau_rv = 0.0;

        let (assign42280_e55550, assign42280_e55550_d_n4, assign42280_e55550_d_n6, assign42280_e55550_d_n7, assign42280_e55550_d_n8, assign42280_e55550_d_n9,) = {
    if ((locals.var_guard1199 == 0.0) && (locals.var_guard1200 == 0.0)) {
        let assign42280_e55523: f64 = (locals.var_sp_s_a * locals.var_nu);
        let assign42280_e55525: f64 = (assign42280_e55523 * locals.var_sp_s_tau);
        let assign42280_e55529: f64 = (locals.var_nu / locals.var_mutau);
        let assign42280_e55531: f64 = (assign42280_e55529 * locals.var_sp_s_tau);
        let assign42280_e55533: f64 = (assign42280_e55531 * locals.var_sp_s_tau);
        let assign42280_e55535: f64 = (assign42280_e55533 * locals.var_sp_s_c);
        let assign42280_e55538: f64 = (locals.var_sp_s_c * locals.var_sp_s_c);
        let assign42280_e55540: f64 = (assign42280_e55538 * 0.3333333333333333);
        let assign42280_e55543: f64 = (locals.var_sp_s_a * locals.var_sp_s_b);
        let assign42280_e55544: f64 = (assign42280_e55540 - assign42280_e55543);
        let assign42280_e55545: f64 = (assign42280_e55535 * assign42280_e55544);
        let assign42280_e55546: f64 = (locals.var_mutau + assign42280_e55545);
        let assign42280_e55547: f64 = (assign42280_e55525 / assign42280_e55546);
        let assign42280_e55548: f64 = (locals.var_sp_s_eta + assign42280_e55547);
        (assign42280_e55548, (locals.var_sp_s_eta_dn4 + (((((((locals.var_sp_s_a_dn4 * locals.var_nu) + (locals.var_sp_s_a * locals.var_nu_dn4)) * locals.var_sp_s_tau) + (assign42280_e55523 * locals.var_sp_s_tau_dn4)) * assign42280_e55546) - (assign42280_e55525 * (locals.var_mutau_dn4 + (((((((((((locals.var_nu_dn4 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn4)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau) + (assign42280_e55529 * locals.var_sp_s_tau_dn4)) * locals.var_sp_s_tau) + (assign42280_e55531 * locals.var_sp_s_tau_dn4)) * locals.var_sp_s_c) + (assign42280_e55533 * locals.var_sp_s_c_dn4)) * assign42280_e55544) + (assign42280_e55535 * ((((locals.var_sp_s_c_dn4 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn4)) * 0.3333333333333333) - ((locals.var_sp_s_a_dn4 * locals.var_sp_s_b) + (locals.var_sp_s_a * locals.var_sp_s_b_dn4)))))))) / (assign42280_e55546 * assign42280_e55546))), (locals.var_sp_s_eta_dn6 + (((((((locals.var_sp_s_a_dn6 * locals.var_nu) + (locals.var_sp_s_a * locals.var_nu_dn6)) * locals.var_sp_s_tau) + (assign42280_e55523 * locals.var_sp_s_tau_dn6)) * assign42280_e55546) - (assign42280_e55525 * (locals.var_mutau_dn6 + (((((((((((locals.var_nu_dn6 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn6)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau) + (assign42280_e55529 * locals.var_sp_s_tau_dn6)) * locals.var_sp_s_tau) + (assign42280_e55531 * locals.var_sp_s_tau_dn6)) * locals.var_sp_s_c) + (assign42280_e55533 * locals.var_sp_s_c_dn6)) * assign42280_e55544) + (assign42280_e55535 * ((((locals.var_sp_s_c_dn6 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn6)) * 0.3333333333333333) - ((locals.var_sp_s_a_dn6 * locals.var_sp_s_b) + (locals.var_sp_s_a * locals.var_sp_s_b_dn6)))))))) / (assign42280_e55546 * assign42280_e55546))), (locals.var_sp_s_eta_dn7 + (((((((locals.var_sp_s_a_dn7 * locals.var_nu) + (locals.var_sp_s_a * locals.var_nu_dn7)) * locals.var_sp_s_tau) + (assign42280_e55523 * locals.var_sp_s_tau_dn7)) * assign42280_e55546) - (assign42280_e55525 * (locals.var_mutau_dn7 + (((((((((((locals.var_nu_dn7 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn7)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau) + (assign42280_e55529 * locals.var_sp_s_tau_dn7)) * locals.var_sp_s_tau) + (assign42280_e55531 * locals.var_sp_s_tau_dn7)) * locals.var_sp_s_c) + (assign42280_e55533 * locals.var_sp_s_c_dn7)) * assign42280_e55544) + (assign42280_e55535 * ((((locals.var_sp_s_c_dn7 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn7)) * 0.3333333333333333) - ((locals.var_sp_s_a_dn7 * locals.var_sp_s_b) + (locals.var_sp_s_a * locals.var_sp_s_b_dn7)))))))) / (assign42280_e55546 * assign42280_e55546))), (locals.var_sp_s_eta_dn8 + (((((((locals.var_sp_s_a_dn8 * locals.var_nu) + (locals.var_sp_s_a * locals.var_nu_dn8)) * locals.var_sp_s_tau) + (assign42280_e55523 * locals.var_sp_s_tau_dn8)) * assign42280_e55546) - (assign42280_e55525 * (locals.var_mutau_dn8 + (((((((((((locals.var_nu_dn8 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn8)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau) + (assign42280_e55529 * locals.var_sp_s_tau_dn8)) * locals.var_sp_s_tau) + (assign42280_e55531 * locals.var_sp_s_tau_dn8)) * locals.var_sp_s_c) + (assign42280_e55533 * locals.var_sp_s_c_dn8)) * assign42280_e55544) + (assign42280_e55535 * ((((locals.var_sp_s_c_dn8 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn8)) * 0.3333333333333333) - ((locals.var_sp_s_a_dn8 * locals.var_sp_s_b) + (locals.var_sp_s_a * locals.var_sp_s_b_dn8)))))))) / (assign42280_e55546 * assign42280_e55546))), (locals.var_sp_s_eta_dn9 + (((((((locals.var_sp_s_a_dn9 * locals.var_nu) + (locals.var_sp_s_a * locals.var_nu_dn9)) * locals.var_sp_s_tau) + (assign42280_e55523 * locals.var_sp_s_tau_dn9)) * assign42280_e55546) - (assign42280_e55525 * (locals.var_mutau_dn9 + (((((((((((locals.var_nu_dn9 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn9)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau) + (assign42280_e55529 * locals.var_sp_s_tau_dn9)) * locals.var_sp_s_tau) + (assign42280_e55531 * locals.var_sp_s_tau_dn9)) * locals.var_sp_s_c) + (assign42280_e55533 * locals.var_sp_s_c_dn9)) * assign42280_e55544) + (assign42280_e55535 * ((((locals.var_sp_s_c_dn9 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn9)) * 0.3333333333333333) - ((locals.var_sp_s_a_dn9 * locals.var_sp_s_b) + (locals.var_sp_s_a * locals.var_sp_s_b_dn9)))))))) / (assign42280_e55546 * assign42280_e55546))),)
    } else {
        (locals.var_sp_s_x0, locals.var_sp_s_x0_dn4, locals.var_sp_s_x0_dn6, locals.var_sp_s_x0_dn7, locals.var_sp_s_x0_dn8, locals.var_sp_s_x0_dn9,)
    }
};
        locals.var_sp_s_x0 = assign42280_e55550;
        locals.var_sp_s_x0_dn4 = assign42280_e55550_d_n4;
        locals.var_sp_s_x0_dn6 = assign42280_e55550_d_n6;
        locals.var_sp_s_x0_dn7 = assign42280_e55550_d_n7;
        locals.var_sp_s_x0_dn8 = assign42280_e55550_d_n8;
        locals.var_sp_s_x0_dn9 = assign42280_e55550_d_n9;
        locals.var_sp_s_x0_rv = 0.0;

        let assign42290_e55553: f64 = if locals.var_sp_s_x0 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard1203 = assign42290_e55553;
        locals.var_guard1203_rv = 0.0;

        let (assign42300_e55564, assign42300_e55564_d_n4, assign42300_e55564_d_n6, assign42300_e55564_d_n7, assign42300_e55564_d_n8, assign42300_e55564_d_n9,) = {
    if (((locals.var_guard1199 == 0.0) && (locals.var_guard1200 == 0.0)) && (locals.var_guard1203 != 0.0)) {
        let assign42300_e55562: f64 = (locals.var_sp_s_x0).exp();
        (assign42300_e55562, (assign42300_e55562 * locals.var_sp_s_x0_dn4), (assign42300_e55562 * locals.var_sp_s_x0_dn6), (assign42300_e55562 * locals.var_sp_s_x0_dn7), (assign42300_e55562 * locals.var_sp_s_x0_dn8), (assign42300_e55562 * locals.var_sp_s_x0_dn9),)
    } else {
        (locals.var_sp_s_delta0, locals.var_sp_s_delta0_dn4, locals.var_sp_s_delta0_dn6, locals.var_sp_s_delta0_dn7, locals.var_sp_s_delta0_dn8, locals.var_sp_s_delta0_dn9,)
    }
};
        locals.var_sp_s_delta0 = assign42300_e55564;
        locals.var_sp_s_delta0_dn4 = assign42300_e55564_d_n4;
        locals.var_sp_s_delta0_dn6 = assign42300_e55564_d_n6;
        locals.var_sp_s_delta0_dn7 = assign42300_e55564_d_n7;
        locals.var_sp_s_delta0_dn8 = assign42300_e55564_d_n8;
        locals.var_sp_s_delta0_dn9 = assign42300_e55564_d_n9;
        locals.var_sp_s_delta0_rv = 0.0;

        let (assign42310_e55576, assign42310_e55576_d_n4, assign42310_e55576_d_n6, assign42310_e55576_d_n7, assign42310_e55576_d_n8, assign42310_e55576_d_n9,) = {
    if (((locals.var_guard1199 == 0.0) && (locals.var_guard1200 == 0.0)) && (locals.var_guard1203 != 0.0)) {
        let assign42310_e55574: f64 = (1.0 / locals.var_sp_s_delta0);
        (assign42310_e55574, (-(locals.var_sp_s_delta0_dn4 / (locals.var_sp_s_delta0 * locals.var_sp_s_delta0))), (-(locals.var_sp_s_delta0_dn6 / (locals.var_sp_s_delta0 * locals.var_sp_s_delta0))), (-(locals.var_sp_s_delta0_dn7 / (locals.var_sp_s_delta0 * locals.var_sp_s_delta0))), (-(locals.var_sp_s_delta0_dn8 / (locals.var_sp_s_delta0 * locals.var_sp_s_delta0))), (-(locals.var_sp_s_delta0_dn9 / (locals.var_sp_s_delta0 * locals.var_sp_s_delta0))),)
    } else {
        (locals.var_sp_s_delta1, locals.var_sp_s_delta1_dn4, locals.var_sp_s_delta1_dn6, locals.var_sp_s_delta1_dn7, locals.var_sp_s_delta1_dn8, locals.var_sp_s_delta1_dn9,)
    }
};
        locals.var_sp_s_delta1 = assign42310_e55576;
        locals.var_sp_s_delta1_dn4 = assign42310_e55576_d_n4;
        locals.var_sp_s_delta1_dn6 = assign42310_e55576_d_n6;
        locals.var_sp_s_delta1_dn7 = assign42310_e55576_d_n7;
        locals.var_sp_s_delta1_dn8 = assign42310_e55576_d_n8;
        locals.var_sp_s_delta1_dn9 = assign42310_e55576_d_n9;
        locals.var_sp_s_delta1_rv = 0.0;

        let (assign42320_e55588, assign42320_e55588_d_n4, assign42320_e55588_d_n6, assign42320_e55588_d_n7, assign42320_e55588_d_n8, assign42320_e55588_d_n9,) = {
    if (((locals.var_guard1199 == 0.0) && (locals.var_guard1200 == 0.0)) && (locals.var_guard1203 != 0.0)) {
        let assign42320_e55586: f64 = (locals.var_delta_ns * locals.var_sp_s_delta0);
        (assign42320_e55586, ((locals.var_delta_ns_dn4 * locals.var_sp_s_delta0) + (locals.var_delta_ns * locals.var_sp_s_delta0_dn4)), ((locals.var_delta_ns_dn6 * locals.var_sp_s_delta0) + (locals.var_delta_ns * locals.var_sp_s_delta0_dn6)), ((locals.var_delta_ns_dn7 * locals.var_sp_s_delta0) + (locals.var_delta_ns * locals.var_sp_s_delta0_dn7)), ((locals.var_delta_ns_dn8 * locals.var_sp_s_delta0) + (locals.var_delta_ns * locals.var_sp_s_delta0_dn8)), ((locals.var_delta_ns_dn9 * locals.var_sp_s_delta0) + (locals.var_delta_ns * locals.var_sp_s_delta0_dn9)),)
    } else {
        (locals.var_sp_s_delta0, locals.var_sp_s_delta0_dn4, locals.var_sp_s_delta0_dn6, locals.var_sp_s_delta0_dn7, locals.var_sp_s_delta0_dn8, locals.var_sp_s_delta0_dn9,)
    }
};
        locals.var_sp_s_delta0 = assign42320_e55588;
        locals.var_sp_s_delta0_dn4 = assign42320_e55588_d_n4;
        locals.var_sp_s_delta0_dn6 = assign42320_e55588_d_n6;
        locals.var_sp_s_delta0_dn7 = assign42320_e55588_d_n7;
        locals.var_sp_s_delta0_dn8 = assign42320_e55588_d_n8;
        locals.var_sp_s_delta0_dn9 = assign42320_e55588_d_n9;
        locals.var_sp_s_delta0_rv = 0.0;

        let assign42330_e55592: f64 = (locals.var_xn_s - 230.25850929940458);
        let assign42330_e55593: f64 = if locals.var_sp_s_x0 > assign42330_e55592 { 1.0 } else { 0.0 };
        locals.var_guard1204 = assign42330_e55593;
        locals.var_guard1204_rv = 0.0;

        let (assign42340_e55609, assign42340_e55609_d_n4, assign42340_e55609_d_n6, assign42340_e55609_d_n7, assign42340_e55609_d_n8, assign42340_e55609_d_n9,) = {
    if ((((locals.var_guard1199 == 0.0) && (locals.var_guard1200 == 0.0)) && (locals.var_guard1203 == 0.0)) && (locals.var_guard1204 != 0.0)) {
        let assign42340_e55606: f64 = (locals.var_sp_s_x0 - locals.var_xn_s);
        let assign42340_e55607: f64 = (assign42340_e55606).exp();
        (assign42340_e55607, (assign42340_e55607 * (locals.var_sp_s_x0_dn4 - locals.var_xn_s_dn4)), (assign42340_e55607 * (locals.var_sp_s_x0_dn6 - locals.var_xn_s_dn6)), (assign42340_e55607 * (locals.var_sp_s_x0_dn7 - locals.var_xn_s_dn7)), (assign42340_e55607 * (locals.var_sp_s_x0_dn8 - locals.var_xn_s_dn8)), (assign42340_e55607 * (locals.var_sp_s_x0_dn9 - locals.var_xn_s_dn9)),)
    } else {
        (locals.var_sp_s_delta0, locals.var_sp_s_delta0_dn4, locals.var_sp_s_delta0_dn6, locals.var_sp_s_delta0_dn7, locals.var_sp_s_delta0_dn8, locals.var_sp_s_delta0_dn9,)
    }
};
        locals.var_sp_s_delta0 = assign42340_e55609;
        locals.var_sp_s_delta0_dn4 = assign42340_e55609_d_n4;
        locals.var_sp_s_delta0_dn6 = assign42340_e55609_d_n6;
        locals.var_sp_s_delta0_dn7 = assign42340_e55609_d_n7;
        locals.var_sp_s_delta0_dn8 = assign42340_e55609_d_n8;
        locals.var_sp_s_delta0_dn9 = assign42340_e55609_d_n9;
        locals.var_sp_s_delta0_rv = 0.0;

        let (assign42350_e55624, assign42350_e55624_d_n4, assign42350_e55624_d_n6, assign42350_e55624_d_n7, assign42350_e55624_d_n8, assign42350_e55624_d_n9,) = {
    if ((((locals.var_guard1199 == 0.0) && (locals.var_guard1200 == 0.0)) && (locals.var_guard1203 == 0.0)) && (locals.var_guard1204 != 0.0)) {
        let assign42350_e55622: f64 = (locals.var_delta_ns / locals.var_sp_s_delta0);
        (assign42350_e55622, (((locals.var_delta_ns_dn4 * locals.var_sp_s_delta0) - (locals.var_delta_ns * locals.var_sp_s_delta0_dn4)) / (locals.var_sp_s_delta0 * locals.var_sp_s_delta0)), (((locals.var_delta_ns_dn6 * locals.var_sp_s_delta0) - (locals.var_delta_ns * locals.var_sp_s_delta0_dn6)) / (locals.var_sp_s_delta0 * locals.var_sp_s_delta0)), (((locals.var_delta_ns_dn7 * locals.var_sp_s_delta0) - (locals.var_delta_ns * locals.var_sp_s_delta0_dn7)) / (locals.var_sp_s_delta0 * locals.var_sp_s_delta0)), (((locals.var_delta_ns_dn8 * locals.var_sp_s_delta0) - (locals.var_delta_ns * locals.var_sp_s_delta0_dn8)) / (locals.var_sp_s_delta0 * locals.var_sp_s_delta0)), (((locals.var_delta_ns_dn9 * locals.var_sp_s_delta0) - (locals.var_delta_ns * locals.var_sp_s_delta0_dn9)) / (locals.var_sp_s_delta0 * locals.var_sp_s_delta0)),)
    } else {
        (locals.var_sp_s_delta1, locals.var_sp_s_delta1_dn4, locals.var_sp_s_delta1_dn6, locals.var_sp_s_delta1_dn7, locals.var_sp_s_delta1_dn8, locals.var_sp_s_delta1_dn9,)
    }
};
        locals.var_sp_s_delta1 = assign42350_e55624;
        locals.var_sp_s_delta1_dn4 = assign42350_e55624_d_n4;
        locals.var_sp_s_delta1_dn6 = assign42350_e55624_d_n6;
        locals.var_sp_s_delta1_dn7 = assign42350_e55624_d_n7;
        locals.var_sp_s_delta1_dn8 = assign42350_e55624_d_n8;
        locals.var_sp_s_delta1_dn9 = assign42350_e55624_d_n9;
        locals.var_sp_s_delta1_rv = 0.0;

        let (assign42360_e55666, assign42360_e55666_d_n4, assign42360_e55666_d_n6, assign42360_e55666_d_n7, assign42360_e55666_d_n8, assign42360_e55666_d_n9,) = {
    if ((((locals.var_guard1199 == 0.0) && (locals.var_guard1200 == 0.0)) && (locals.var_guard1203 == 0.0)) && (locals.var_guard1204 == 0.0)) {
        let assign42360_e55640: f64 = (locals.var_xn_s - locals.var_sp_s_x0);
        let assign42360_e55642: f64 = (assign42360_e55640 - 230.25850929940458);
        let assign42360_e55647: f64 = (locals.var_xn_s - locals.var_sp_s_x0);
        let assign42360_e55649: f64 = (assign42360_e55647 - 230.25850929940458);
        let assign42360_e55653: f64 = (locals.var_xn_s - locals.var_sp_s_x0);
        let assign42360_e55655: f64 = (assign42360_e55653 - 230.25850929940458);
        let assign42360_e55657: f64 = (assign42360_e55655 * 0.3333333333333333);
        let assign42360_e55658: f64 = (1.0 + assign42360_e55657);
        let assign42360_e55659: f64 = (assign42360_e55649 * assign42360_e55658);
        let assign42360_e55660: f64 = (0.5 * assign42360_e55659);
        let assign42360_e55661: f64 = (1.0 + assign42360_e55660);
        let assign42360_e55662: f64 = (assign42360_e55642 * assign42360_e55661);
        let assign42360_e55663: f64 = (1.0 + assign42360_e55662);
        let assign42360_e55664: f64 = (1e-100 / assign42360_e55663);
        (assign42360_e55664, (-((1e-100 * (((locals.var_xn_s_dn4 - locals.var_sp_s_x0_dn4) * assign42360_e55661) + (assign42360_e55642 * (0.5 * (((locals.var_xn_s_dn4 - locals.var_sp_s_x0_dn4) * assign42360_e55658) + (assign42360_e55649 * ((locals.var_xn_s_dn4 - locals.var_sp_s_x0_dn4) * 0.3333333333333333))))))) / (assign42360_e55663 * assign42360_e55663))), (-((1e-100 * (((locals.var_xn_s_dn6 - locals.var_sp_s_x0_dn6) * assign42360_e55661) + (assign42360_e55642 * (0.5 * (((locals.var_xn_s_dn6 - locals.var_sp_s_x0_dn6) * assign42360_e55658) + (assign42360_e55649 * ((locals.var_xn_s_dn6 - locals.var_sp_s_x0_dn6) * 0.3333333333333333))))))) / (assign42360_e55663 * assign42360_e55663))), (-((1e-100 * (((locals.var_xn_s_dn7 - locals.var_sp_s_x0_dn7) * assign42360_e55661) + (assign42360_e55642 * (0.5 * (((locals.var_xn_s_dn7 - locals.var_sp_s_x0_dn7) * assign42360_e55658) + (assign42360_e55649 * ((locals.var_xn_s_dn7 - locals.var_sp_s_x0_dn7) * 0.3333333333333333))))))) / (assign42360_e55663 * assign42360_e55663))), (-((1e-100 * (((locals.var_xn_s_dn8 - locals.var_sp_s_x0_dn8) * assign42360_e55661) + (assign42360_e55642 * (0.5 * (((locals.var_xn_s_dn8 - locals.var_sp_s_x0_dn8) * assign42360_e55658) + (assign42360_e55649 * ((locals.var_xn_s_dn8 - locals.var_sp_s_x0_dn8) * 0.3333333333333333))))))) / (assign42360_e55663 * assign42360_e55663))), (-((1e-100 * (((locals.var_xn_s_dn9 - locals.var_sp_s_x0_dn9) * assign42360_e55661) + (assign42360_e55642 * (0.5 * (((locals.var_xn_s_dn9 - locals.var_sp_s_x0_dn9) * assign42360_e55658) + (assign42360_e55649 * ((locals.var_xn_s_dn9 - locals.var_sp_s_x0_dn9) * 0.3333333333333333))))))) / (assign42360_e55663 * assign42360_e55663))),)
    } else {
        (locals.var_sp_s_delta0, locals.var_sp_s_delta0_dn4, locals.var_sp_s_delta0_dn6, locals.var_sp_s_delta0_dn7, locals.var_sp_s_delta0_dn8, locals.var_sp_s_delta0_dn9,)
    }
};
        locals.var_sp_s_delta0 = assign42360_e55666;
        locals.var_sp_s_delta0_dn4 = assign42360_e55666_d_n4;
        locals.var_sp_s_delta0_dn6 = assign42360_e55666_d_n6;
        locals.var_sp_s_delta0_dn7 = assign42360_e55666_d_n7;
        locals.var_sp_s_delta0_dn8 = assign42360_e55666_d_n8;
        locals.var_sp_s_delta0_dn9 = assign42360_e55666_d_n9;
        locals.var_sp_s_delta0_rv = 0.0;

        let (assign42370_e55702, assign42370_e55702_d_n4, assign42370_e55702_d_n6, assign42370_e55702_d_n7, assign42370_e55702_d_n8, assign42370_e55702_d_n9,) = {
    if ((((locals.var_guard1199 == 0.0) && (locals.var_guard1200 == 0.0)) && (locals.var_guard1203 == 0.0)) && (locals.var_guard1204 == 0.0)) {
        let assign42370_e55682: f64 = (locals.var_sp_s_x0 - 230.25850929940458);
        let assign42370_e55687: f64 = (locals.var_sp_s_x0 - 230.25850929940458);
        let assign42370_e55691: f64 = (locals.var_sp_s_x0 - 230.25850929940458);
        let assign42370_e55693: f64 = (assign42370_e55691 * 0.3333333333333333);
        let assign42370_e55694: f64 = (1.0 + assign42370_e55693);
        let assign42370_e55695: f64 = (assign42370_e55687 * assign42370_e55694);
        let assign42370_e55696: f64 = (0.5 * assign42370_e55695);
        let assign42370_e55697: f64 = (1.0 + assign42370_e55696);
        let assign42370_e55698: f64 = (assign42370_e55682 * assign42370_e55697);
        let assign42370_e55699: f64 = (1.0 + assign42370_e55698);
        let assign42370_e55700: f64 = (1e-100 / assign42370_e55699);
        (assign42370_e55700, (-((1e-100 * ((locals.var_sp_s_x0_dn4 * assign42370_e55697) + (assign42370_e55682 * (0.5 * ((locals.var_sp_s_x0_dn4 * assign42370_e55694) + (assign42370_e55687 * (locals.var_sp_s_x0_dn4 * 0.3333333333333333))))))) / (assign42370_e55699 * assign42370_e55699))), (-((1e-100 * ((locals.var_sp_s_x0_dn6 * assign42370_e55697) + (assign42370_e55682 * (0.5 * ((locals.var_sp_s_x0_dn6 * assign42370_e55694) + (assign42370_e55687 * (locals.var_sp_s_x0_dn6 * 0.3333333333333333))))))) / (assign42370_e55699 * assign42370_e55699))), (-((1e-100 * ((locals.var_sp_s_x0_dn7 * assign42370_e55697) + (assign42370_e55682 * (0.5 * ((locals.var_sp_s_x0_dn7 * assign42370_e55694) + (assign42370_e55687 * (locals.var_sp_s_x0_dn7 * 0.3333333333333333))))))) / (assign42370_e55699 * assign42370_e55699))), (-((1e-100 * ((locals.var_sp_s_x0_dn8 * assign42370_e55697) + (assign42370_e55682 * (0.5 * ((locals.var_sp_s_x0_dn8 * assign42370_e55694) + (assign42370_e55687 * (locals.var_sp_s_x0_dn8 * 0.3333333333333333))))))) / (assign42370_e55699 * assign42370_e55699))), (-((1e-100 * ((locals.var_sp_s_x0_dn9 * assign42370_e55697) + (assign42370_e55682 * (0.5 * ((locals.var_sp_s_x0_dn9 * assign42370_e55694) + (assign42370_e55687 * (locals.var_sp_s_x0_dn9 * 0.3333333333333333))))))) / (assign42370_e55699 * assign42370_e55699))),)
    } else {
        (locals.var_sp_s_delta1, locals.var_sp_s_delta1_dn4, locals.var_sp_s_delta1_dn6, locals.var_sp_s_delta1_dn7, locals.var_sp_s_delta1_dn8, locals.var_sp_s_delta1_dn9,)
    }
};
        locals.var_sp_s_delta1 = assign42370_e55702;
        locals.var_sp_s_delta1_dn4 = assign42370_e55702_d_n4;
        locals.var_sp_s_delta1_dn6 = assign42370_e55702_d_n6;
        locals.var_sp_s_delta1_dn7 = assign42370_e55702_d_n7;
        locals.var_sp_s_delta1_dn8 = assign42370_e55702_d_n8;
        locals.var_sp_s_delta1_dn9 = assign42370_e55702_d_n9;
        locals.var_sp_s_delta1_rv = 0.0;

        let (assign42380_e55716, assign42380_e55716_d_n4, assign42380_e55716_d_n6, assign42380_e55716_d_n7, assign42380_e55716_d_n8, assign42380_e55716_d_n9,) = {
    if ((locals.var_guard1199 == 0.0) && (locals.var_guard1200 == 0.0)) {
        let assign42380_e55712: f64 = (locals.var_sp_s_x0 * locals.var_sp_s_x0);
        let assign42380_e55713: f64 = (2.0 + assign42380_e55712);
        let assign42380_e55714: f64 = (1.0 / assign42380_e55713);
        (assign42380_e55714, (-(((locals.var_sp_s_x0_dn4 * locals.var_sp_s_x0) + (locals.var_sp_s_x0 * locals.var_sp_s_x0_dn4)) / (assign42380_e55713 * assign42380_e55713))), (-(((locals.var_sp_s_x0_dn6 * locals.var_sp_s_x0) + (locals.var_sp_s_x0 * locals.var_sp_s_x0_dn6)) / (assign42380_e55713 * assign42380_e55713))), (-(((locals.var_sp_s_x0_dn7 * locals.var_sp_s_x0) + (locals.var_sp_s_x0 * locals.var_sp_s_x0_dn7)) / (assign42380_e55713 * assign42380_e55713))), (-(((locals.var_sp_s_x0_dn8 * locals.var_sp_s_x0) + (locals.var_sp_s_x0 * locals.var_sp_s_x0_dn8)) / (assign42380_e55713 * assign42380_e55713))), (-(((locals.var_sp_s_x0_dn9 * locals.var_sp_s_x0) + (locals.var_sp_s_x0 * locals.var_sp_s_x0_dn9)) / (assign42380_e55713 * assign42380_e55713))),)
    } else {
        (locals.var_sp_s_temp, locals.var_sp_s_temp_dn4, locals.var_sp_s_temp_dn6, locals.var_sp_s_temp_dn7, locals.var_sp_s_temp_dn8, locals.var_sp_s_temp_dn9,)
    }
};
        locals.var_sp_s_temp = assign42380_e55716;
        locals.var_sp_s_temp_dn4 = assign42380_e55716_d_n4;
        locals.var_sp_s_temp_dn6 = assign42380_e55716_d_n6;
        locals.var_sp_s_temp_dn7 = assign42380_e55716_d_n7;
        locals.var_sp_s_temp_dn8 = assign42380_e55716_d_n8;
        locals.var_sp_s_temp_dn9 = assign42380_e55716_d_n9;
        locals.var_sp_s_temp_rv = 0.0;

        let (assign42390_e55728, assign42390_e55728_d_n4, assign42390_e55728_d_n6, assign42390_e55728_d_n7, assign42390_e55728_d_n8, assign42390_e55728_d_n9,) = {
    if ((locals.var_guard1199 == 0.0) && (locals.var_guard1200 == 0.0)) {
        let assign42390_e55724: f64 = (locals.var_sp_s_x0 * locals.var_sp_s_x0);
        let assign42390_e55726: f64 = (assign42390_e55724 * locals.var_sp_s_temp);
        (assign42390_e55726, ((((locals.var_sp_s_x0_dn4 * locals.var_sp_s_x0) + (locals.var_sp_s_x0 * locals.var_sp_s_x0_dn4)) * locals.var_sp_s_temp) + (assign42390_e55724 * locals.var_sp_s_temp_dn4)), ((((locals.var_sp_s_x0_dn6 * locals.var_sp_s_x0) + (locals.var_sp_s_x0 * locals.var_sp_s_x0_dn6)) * locals.var_sp_s_temp) + (assign42390_e55724 * locals.var_sp_s_temp_dn6)), ((((locals.var_sp_s_x0_dn7 * locals.var_sp_s_x0) + (locals.var_sp_s_x0 * locals.var_sp_s_x0_dn7)) * locals.var_sp_s_temp) + (assign42390_e55724 * locals.var_sp_s_temp_dn7)), ((((locals.var_sp_s_x0_dn8 * locals.var_sp_s_x0) + (locals.var_sp_s_x0 * locals.var_sp_s_x0_dn8)) * locals.var_sp_s_temp) + (assign42390_e55724 * locals.var_sp_s_temp_dn8)), ((((locals.var_sp_s_x0_dn9 * locals.var_sp_s_x0) + (locals.var_sp_s_x0 * locals.var_sp_s_x0_dn9)) * locals.var_sp_s_temp) + (assign42390_e55724 * locals.var_sp_s_temp_dn9)),)
    } else {
        (locals.var_sp_s_xi0, locals.var_sp_s_xi0_dn4, locals.var_sp_s_xi0_dn6, locals.var_sp_s_xi0_dn7, locals.var_sp_s_xi0_dn8, locals.var_sp_s_xi0_dn9,)
    }
};
        locals.var_sp_s_xi0 = assign42390_e55728;
        locals.var_sp_s_xi0_dn4 = assign42390_e55728_d_n4;
        locals.var_sp_s_xi0_dn6 = assign42390_e55728_d_n6;
        locals.var_sp_s_xi0_dn7 = assign42390_e55728_d_n7;
        locals.var_sp_s_xi0_dn8 = assign42390_e55728_d_n8;
        locals.var_sp_s_xi0_dn9 = assign42390_e55728_d_n9;
        locals.var_sp_s_xi0_rv = 0.0;

        let (assign42400_e55742, assign42400_e55742_d_n4, assign42400_e55742_d_n6, assign42400_e55742_d_n7, assign42400_e55742_d_n8, assign42400_e55742_d_n9,) = {
    if ((locals.var_guard1199 == 0.0) && (locals.var_guard1200 == 0.0)) {
        let assign42400_e55737: f64 = (locals.var_sp_s_x0 * locals.var_sp_s_temp);
        let assign42400_e55739: f64 = (assign42400_e55737 * locals.var_sp_s_temp);
        let assign42400_e55740: f64 = (4.0 * assign42400_e55739);
        (assign42400_e55740, (4.0 * ((((locals.var_sp_s_x0_dn4 * locals.var_sp_s_temp) + (locals.var_sp_s_x0 * locals.var_sp_s_temp_dn4)) * locals.var_sp_s_temp) + (assign42400_e55737 * locals.var_sp_s_temp_dn4))), (4.0 * ((((locals.var_sp_s_x0_dn6 * locals.var_sp_s_temp) + (locals.var_sp_s_x0 * locals.var_sp_s_temp_dn6)) * locals.var_sp_s_temp) + (assign42400_e55737 * locals.var_sp_s_temp_dn6))), (4.0 * ((((locals.var_sp_s_x0_dn7 * locals.var_sp_s_temp) + (locals.var_sp_s_x0 * locals.var_sp_s_temp_dn7)) * locals.var_sp_s_temp) + (assign42400_e55737 * locals.var_sp_s_temp_dn7))), (4.0 * ((((locals.var_sp_s_x0_dn8 * locals.var_sp_s_temp) + (locals.var_sp_s_x0 * locals.var_sp_s_temp_dn8)) * locals.var_sp_s_temp) + (assign42400_e55737 * locals.var_sp_s_temp_dn8))), (4.0 * ((((locals.var_sp_s_x0_dn9 * locals.var_sp_s_temp) + (locals.var_sp_s_x0 * locals.var_sp_s_temp_dn9)) * locals.var_sp_s_temp) + (assign42400_e55737 * locals.var_sp_s_temp_dn9))),)
    } else {
        (locals.var_sp_s_xi1, locals.var_sp_s_xi1_dn4, locals.var_sp_s_xi1_dn6, locals.var_sp_s_xi1_dn7, locals.var_sp_s_xi1_dn8, locals.var_sp_s_xi1_dn9,)
    }
};
        locals.var_sp_s_xi1 = assign42400_e55742;
        locals.var_sp_s_xi1_dn4 = assign42400_e55742_d_n4;
        locals.var_sp_s_xi1_dn6 = assign42400_e55742_d_n6;
        locals.var_sp_s_xi1_dn7 = assign42400_e55742_d_n7;
        locals.var_sp_s_xi1_dn8 = assign42400_e55742_d_n8;
        locals.var_sp_s_xi1_dn9 = assign42400_e55742_d_n9;
        locals.var_sp_s_xi1_rv = 0.0;

        let (assign42410_e55760, assign42410_e55760_d_n4, assign42410_e55760_d_n6, assign42410_e55760_d_n7, assign42410_e55760_d_n8, assign42410_e55760_d_n9,) = {
    if ((locals.var_guard1199 == 0.0) && (locals.var_guard1200 == 0.0)) {
        let assign42410_e55750: f64 = (8.0 * locals.var_sp_s_temp);
        let assign42410_e55753: f64 = (12.0 * locals.var_sp_s_xi0);
        let assign42410_e55754: f64 = (assign42410_e55750 - assign42410_e55753);
        let assign42410_e55756: f64 = (assign42410_e55754 * locals.var_sp_s_temp);
        let assign42410_e55758: f64 = (assign42410_e55756 * locals.var_sp_s_temp);
        (assign42410_e55758, ((((((8.0 * locals.var_sp_s_temp_dn4) - (12.0 * locals.var_sp_s_xi0_dn4)) * locals.var_sp_s_temp) + (assign42410_e55754 * locals.var_sp_s_temp_dn4)) * locals.var_sp_s_temp) + (assign42410_e55756 * locals.var_sp_s_temp_dn4)), ((((((8.0 * locals.var_sp_s_temp_dn6) - (12.0 * locals.var_sp_s_xi0_dn6)) * locals.var_sp_s_temp) + (assign42410_e55754 * locals.var_sp_s_temp_dn6)) * locals.var_sp_s_temp) + (assign42410_e55756 * locals.var_sp_s_temp_dn6)), ((((((8.0 * locals.var_sp_s_temp_dn7) - (12.0 * locals.var_sp_s_xi0_dn7)) * locals.var_sp_s_temp) + (assign42410_e55754 * locals.var_sp_s_temp_dn7)) * locals.var_sp_s_temp) + (assign42410_e55756 * locals.var_sp_s_temp_dn7)), ((((((8.0 * locals.var_sp_s_temp_dn8) - (12.0 * locals.var_sp_s_xi0_dn8)) * locals.var_sp_s_temp) + (assign42410_e55754 * locals.var_sp_s_temp_dn8)) * locals.var_sp_s_temp) + (assign42410_e55756 * locals.var_sp_s_temp_dn8)), ((((((8.0 * locals.var_sp_s_temp_dn9) - (12.0 * locals.var_sp_s_xi0_dn9)) * locals.var_sp_s_temp) + (assign42410_e55754 * locals.var_sp_s_temp_dn9)) * locals.var_sp_s_temp) + (assign42410_e55756 * locals.var_sp_s_temp_dn9)),)
    } else {
        (locals.var_sp_s_xi2, locals.var_sp_s_xi2_dn4, locals.var_sp_s_xi2_dn6, locals.var_sp_s_xi2_dn7, locals.var_sp_s_xi2_dn8, locals.var_sp_s_xi2_dn9,)
    }
};
        locals.var_sp_s_xi2 = assign42410_e55760;
        locals.var_sp_s_xi2_dn4 = assign42410_e55760_d_n4;
        locals.var_sp_s_xi2_dn6 = assign42410_e55760_d_n6;
        locals.var_sp_s_xi2_dn7 = assign42410_e55760_d_n7;
        locals.var_sp_s_xi2_dn8 = assign42410_e55760_d_n8;
        locals.var_sp_s_xi2_dn9 = assign42410_e55760_d_n9;
        locals.var_sp_s_xi2_rv = 0.0;

        let (assign42420_e55770, assign42420_e55770_d_n4, assign42420_e55770_d_n6, assign42420_e55770_d_n7, assign42420_e55770_d_n8, assign42420_e55770_d_n9,) = {
    if ((locals.var_guard1199 == 0.0) && (locals.var_guard1200 == 0.0)) {
        let assign42420_e55768: f64 = (locals.var_xg - locals.var_sp_s_x0);
        (assign42420_e55768, (locals.var_xg_dn4 - locals.var_sp_s_x0_dn4), (locals.var_xg_dn6 - locals.var_sp_s_x0_dn6), (locals.var_xg_dn7 - locals.var_sp_s_x0_dn7), (locals.var_xg_dn8 - locals.var_sp_s_x0_dn8), (locals.var_xg_dn9 - locals.var_sp_s_x0_dn9),)
    } else {
        (locals.var_sp_s_temp, locals.var_sp_s_temp_dn4, locals.var_sp_s_temp_dn6, locals.var_sp_s_temp_dn7, locals.var_sp_s_temp_dn8, locals.var_sp_s_temp_dn9,)
    }
};
        locals.var_sp_s_temp = assign42420_e55770;
        locals.var_sp_s_temp_dn4 = assign42420_e55770_d_n4;
        locals.var_sp_s_temp_dn6 = assign42420_e55770_d_n6;
        locals.var_sp_s_temp_dn7 = assign42420_e55770_d_n7;
        locals.var_sp_s_temp_dn8 = assign42420_e55770_d_n8;
        locals.var_sp_s_temp_dn9 = assign42420_e55770_d_n9;
        locals.var_sp_s_temp_rv = 0.0;

        let (assign42430_e55794, assign42430_e55794_d_n4, assign42430_e55794_d_n6, assign42430_e55794_d_n7, assign42430_e55794_d_n8, assign42430_e55794_d_n9,) = {
    if ((locals.var_guard1199 == 0.0) && (locals.var_guard1200 == 0.0)) {
        let assign42430_e55778: f64 = (2.0 * locals.var_sp_s_temp);
        let assign42430_e55782: f64 = (1.0 - locals.var_sp_s_delta1);
        let assign42430_e55784: f64 = (assign42430_e55782 + locals.var_sp_s_delta0);
        let assign42430_e55788: f64 = (1.0 + locals.var_sp_s_xi1);
        let assign42430_e55789: f64 = (locals.var_delta_ns * assign42430_e55788);
        let assign42430_e55790: f64 = (assign42430_e55784 - assign42430_e55789);
        let assign42430_e55791: f64 = (locals.var_gf2 * assign42430_e55790);
        let assign42430_e55792: f64 = (assign42430_e55778 + assign42430_e55791);
        (assign42430_e55792, ((2.0 * locals.var_sp_s_temp_dn4) + ((locals.var_gf2_dn4 * assign42430_e55790) + (locals.var_gf2 * (((-locals.var_sp_s_delta1_dn4) + locals.var_sp_s_delta0_dn4) - ((locals.var_delta_ns_dn4 * assign42430_e55788) + (locals.var_delta_ns * locals.var_sp_s_xi1_dn4)))))), ((2.0 * locals.var_sp_s_temp_dn6) + ((locals.var_gf2_dn6 * assign42430_e55790) + (locals.var_gf2 * (((-locals.var_sp_s_delta1_dn6) + locals.var_sp_s_delta0_dn6) - ((locals.var_delta_ns_dn6 * assign42430_e55788) + (locals.var_delta_ns * locals.var_sp_s_xi1_dn6)))))), ((2.0 * locals.var_sp_s_temp_dn7) + ((locals.var_gf2_dn7 * assign42430_e55790) + (locals.var_gf2 * (((-locals.var_sp_s_delta1_dn7) + locals.var_sp_s_delta0_dn7) - ((locals.var_delta_ns_dn7 * assign42430_e55788) + (locals.var_delta_ns * locals.var_sp_s_xi1_dn7)))))), ((2.0 * locals.var_sp_s_temp_dn8) + ((locals.var_gf2_dn8 * assign42430_e55790) + (locals.var_gf2 * (((-locals.var_sp_s_delta1_dn8) + locals.var_sp_s_delta0_dn8) - ((locals.var_delta_ns_dn8 * assign42430_e55788) + (locals.var_delta_ns * locals.var_sp_s_xi1_dn8)))))), ((2.0 * locals.var_sp_s_temp_dn9) + ((locals.var_gf2_dn9 * assign42430_e55790) + (locals.var_gf2 * (((-locals.var_sp_s_delta1_dn9) + locals.var_sp_s_delta0_dn9) - ((locals.var_delta_ns_dn9 * assign42430_e55788) + (locals.var_delta_ns * locals.var_sp_s_xi1_dn9)))))),)
    } else {
        (locals.var_sp_s_pc, locals.var_sp_s_pc_dn4, locals.var_sp_s_pc_dn6, locals.var_sp_s_pc_dn7, locals.var_sp_s_pc_dn8, locals.var_sp_s_pc_dn9,)
    }
};
        locals.var_sp_s_pc = assign42430_e55794;
        locals.var_sp_s_pc_dn4 = assign42430_e55794_d_n4;
        locals.var_sp_s_pc_dn6 = assign42430_e55794_d_n6;
        locals.var_sp_s_pc_dn7 = assign42430_e55794_d_n7;
        locals.var_sp_s_pc_dn8 = assign42430_e55794_d_n8;
        locals.var_sp_s_pc_dn9 = assign42430_e55794_d_n9;
        locals.var_sp_s_pc_rv = 0.0;

        let (assign42440_e55822, assign42440_e55822_d_n4, assign42440_e55822_d_n6, assign42440_e55822_d_n7, assign42440_e55822_d_n8, assign42440_e55822_d_n9,) = {
    if ((locals.var_guard1199 == 0.0) && (locals.var_guard1200 == 0.0)) {
        let assign42440_e55802: f64 = (locals.var_sp_s_temp * locals.var_sp_s_temp);
        let assign42440_e55806: f64 = (locals.var_sp_s_delta1 + locals.var_sp_s_x0);
        let assign42440_e55808: f64 = (assign42440_e55806 - 1.0);
        let assign42440_e55810: f64 = (assign42440_e55808 + locals.var_sp_s_delta0);
        let assign42440_e55814: f64 = (locals.var_sp_s_x0 + 1.0);
        let assign42440_e55816: f64 = (assign42440_e55814 + locals.var_sp_s_xi0);
        let assign42440_e55817: f64 = (locals.var_delta_ns * assign42440_e55816);
        let assign42440_e55818: f64 = (assign42440_e55810 - assign42440_e55817);
        let assign42440_e55819: f64 = (locals.var_gf2 * assign42440_e55818);
        let assign42440_e55820: f64 = (assign42440_e55802 - assign42440_e55819);
        (assign42440_e55820, (((locals.var_sp_s_temp_dn4 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn4)) - ((locals.var_gf2_dn4 * assign42440_e55818) + (locals.var_gf2 * (((locals.var_sp_s_delta1_dn4 + locals.var_sp_s_x0_dn4) + locals.var_sp_s_delta0_dn4) - ((locals.var_delta_ns_dn4 * assign42440_e55816) + (locals.var_delta_ns * (locals.var_sp_s_x0_dn4 + locals.var_sp_s_xi0_dn4))))))), (((locals.var_sp_s_temp_dn6 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn6)) - ((locals.var_gf2_dn6 * assign42440_e55818) + (locals.var_gf2 * (((locals.var_sp_s_delta1_dn6 + locals.var_sp_s_x0_dn6) + locals.var_sp_s_delta0_dn6) - ((locals.var_delta_ns_dn6 * assign42440_e55816) + (locals.var_delta_ns * (locals.var_sp_s_x0_dn6 + locals.var_sp_s_xi0_dn6))))))), (((locals.var_sp_s_temp_dn7 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn7)) - ((locals.var_gf2_dn7 * assign42440_e55818) + (locals.var_gf2 * (((locals.var_sp_s_delta1_dn7 + locals.var_sp_s_x0_dn7) + locals.var_sp_s_delta0_dn7) - ((locals.var_delta_ns_dn7 * assign42440_e55816) + (locals.var_delta_ns * (locals.var_sp_s_x0_dn7 + locals.var_sp_s_xi0_dn7))))))), (((locals.var_sp_s_temp_dn8 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn8)) - ((locals.var_gf2_dn8 * assign42440_e55818) + (locals.var_gf2 * (((locals.var_sp_s_delta1_dn8 + locals.var_sp_s_x0_dn8) + locals.var_sp_s_delta0_dn8) - ((locals.var_delta_ns_dn8 * assign42440_e55816) + (locals.var_delta_ns * (locals.var_sp_s_x0_dn8 + locals.var_sp_s_xi0_dn8))))))), (((locals.var_sp_s_temp_dn9 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn9)) - ((locals.var_gf2_dn9 * assign42440_e55818) + (locals.var_gf2 * (((locals.var_sp_s_delta1_dn9 + locals.var_sp_s_x0_dn9) + locals.var_sp_s_delta0_dn9) - ((locals.var_delta_ns_dn9 * assign42440_e55816) + (locals.var_delta_ns * (locals.var_sp_s_x0_dn9 + locals.var_sp_s_xi0_dn9))))))),)
    } else {
        (locals.var_sp_s_qc, locals.var_sp_s_qc_dn4, locals.var_sp_s_qc_dn6, locals.var_sp_s_qc_dn7, locals.var_sp_s_qc_dn8, locals.var_sp_s_qc_dn9,)
    }
};
        locals.var_sp_s_qc = assign42440_e55822;
        locals.var_sp_s_qc_dn4 = assign42440_e55822_d_n4;
        locals.var_sp_s_qc_dn6 = assign42440_e55822_d_n6;
        locals.var_sp_s_qc_dn7 = assign42440_e55822_d_n7;
        locals.var_sp_s_qc_dn8 = assign42440_e55822_d_n8;
        locals.var_sp_s_qc_dn9 = assign42440_e55822_d_n9;
        locals.var_sp_s_qc_rv = 0.0;

        let (assign42450_e55840, assign42450_e55840_d_n4, assign42450_e55840_d_n6, assign42450_e55840_d_n7, assign42450_e55840_d_n8, assign42450_e55840_d_n9,) = {
    if ((locals.var_guard1199 == 0.0) && (locals.var_guard1200 == 0.0)) {
        let assign42450_e55832: f64 = (locals.var_sp_s_delta1 + locals.var_sp_s_delta0);
        let assign42450_e55835: f64 = (locals.var_delta_ns * locals.var_sp_s_xi2);
        let assign42450_e55836: f64 = (assign42450_e55832 - assign42450_e55835);
        let assign42450_e55837: f64 = (locals.var_gf2 * assign42450_e55836);
        let assign42450_e55838: f64 = (2.0 - assign42450_e55837);
        (assign42450_e55838, (-((locals.var_gf2_dn4 * assign42450_e55836) + (locals.var_gf2 * ((locals.var_sp_s_delta1_dn4 + locals.var_sp_s_delta0_dn4) - ((locals.var_delta_ns_dn4 * locals.var_sp_s_xi2) + (locals.var_delta_ns * locals.var_sp_s_xi2_dn4)))))), (-((locals.var_gf2_dn6 * assign42450_e55836) + (locals.var_gf2 * ((locals.var_sp_s_delta1_dn6 + locals.var_sp_s_delta0_dn6) - ((locals.var_delta_ns_dn6 * locals.var_sp_s_xi2) + (locals.var_delta_ns * locals.var_sp_s_xi2_dn6)))))), (-((locals.var_gf2_dn7 * assign42450_e55836) + (locals.var_gf2 * ((locals.var_sp_s_delta1_dn7 + locals.var_sp_s_delta0_dn7) - ((locals.var_delta_ns_dn7 * locals.var_sp_s_xi2) + (locals.var_delta_ns * locals.var_sp_s_xi2_dn7)))))), (-((locals.var_gf2_dn8 * assign42450_e55836) + (locals.var_gf2 * ((locals.var_sp_s_delta1_dn8 + locals.var_sp_s_delta0_dn8) - ((locals.var_delta_ns_dn8 * locals.var_sp_s_xi2) + (locals.var_delta_ns * locals.var_sp_s_xi2_dn8)))))), (-((locals.var_gf2_dn9 * assign42450_e55836) + (locals.var_gf2 * ((locals.var_sp_s_delta1_dn9 + locals.var_sp_s_delta0_dn9) - ((locals.var_delta_ns_dn9 * locals.var_sp_s_xi2) + (locals.var_delta_ns * locals.var_sp_s_xi2_dn9)))))),)
    } else {
        (locals.var_sp_s_temp, locals.var_sp_s_temp_dn4, locals.var_sp_s_temp_dn6, locals.var_sp_s_temp_dn7, locals.var_sp_s_temp_dn8, locals.var_sp_s_temp_dn9,)
    }
};
        locals.var_sp_s_temp = assign42450_e55840;
        locals.var_sp_s_temp_dn4 = assign42450_e55840_d_n4;
        locals.var_sp_s_temp_dn6 = assign42450_e55840_d_n6;
        locals.var_sp_s_temp_dn7 = assign42450_e55840_d_n7;
        locals.var_sp_s_temp_dn8 = assign42450_e55840_d_n8;
        locals.var_sp_s_temp_dn9 = assign42450_e55840_d_n9;
        locals.var_sp_s_temp_rv = 0.0;

        let (assign42460_e55856, assign42460_e55856_d_n4, assign42460_e55856_d_n6, assign42460_e55856_d_n7, assign42460_e55856_d_n8, assign42460_e55856_d_n9,) = {
    if ((locals.var_guard1199 == 0.0) && (locals.var_guard1200 == 0.0)) {
        let assign42460_e55848: f64 = (locals.var_sp_s_pc * locals.var_sp_s_pc);
        let assign42460_e55852: f64 = (locals.var_sp_s_qc * locals.var_sp_s_temp);
        let assign42460_e55853: f64 = (2.0 * assign42460_e55852);
        let assign42460_e55854: f64 = (assign42460_e55848 - assign42460_e55853);
        (assign42460_e55854, (((locals.var_sp_s_pc_dn4 * locals.var_sp_s_pc) + (locals.var_sp_s_pc * locals.var_sp_s_pc_dn4)) - (2.0 * ((locals.var_sp_s_qc_dn4 * locals.var_sp_s_temp) + (locals.var_sp_s_qc * locals.var_sp_s_temp_dn4)))), (((locals.var_sp_s_pc_dn6 * locals.var_sp_s_pc) + (locals.var_sp_s_pc * locals.var_sp_s_pc_dn6)) - (2.0 * ((locals.var_sp_s_qc_dn6 * locals.var_sp_s_temp) + (locals.var_sp_s_qc * locals.var_sp_s_temp_dn6)))), (((locals.var_sp_s_pc_dn7 * locals.var_sp_s_pc) + (locals.var_sp_s_pc * locals.var_sp_s_pc_dn7)) - (2.0 * ((locals.var_sp_s_qc_dn7 * locals.var_sp_s_temp) + (locals.var_sp_s_qc * locals.var_sp_s_temp_dn7)))), (((locals.var_sp_s_pc_dn8 * locals.var_sp_s_pc) + (locals.var_sp_s_pc * locals.var_sp_s_pc_dn8)) - (2.0 * ((locals.var_sp_s_qc_dn8 * locals.var_sp_s_temp) + (locals.var_sp_s_qc * locals.var_sp_s_temp_dn8)))), (((locals.var_sp_s_pc_dn9 * locals.var_sp_s_pc) + (locals.var_sp_s_pc * locals.var_sp_s_pc_dn9)) - (2.0 * ((locals.var_sp_s_qc_dn9 * locals.var_sp_s_temp) + (locals.var_sp_s_qc * locals.var_sp_s_temp_dn9)))),)
    } else {
        (locals.var_sp_s_temp, locals.var_sp_s_temp_dn4, locals.var_sp_s_temp_dn6, locals.var_sp_s_temp_dn7, locals.var_sp_s_temp_dn8, locals.var_sp_s_temp_dn9,)
    }
};
        locals.var_sp_s_temp = assign42460_e55856;
        locals.var_sp_s_temp_dn4 = assign42460_e55856_d_n4;
        locals.var_sp_s_temp_dn6 = assign42460_e55856_d_n6;
        locals.var_sp_s_temp_dn7 = assign42460_e55856_d_n7;
        locals.var_sp_s_temp_dn8 = assign42460_e55856_d_n8;
        locals.var_sp_s_temp_dn9 = assign42460_e55856_d_n9;
        locals.var_sp_s_temp_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_21(
        locals: &mut StampLocals,
    ) {
        let (assign42470_e55873, assign42470_e55873_d_n4, assign42470_e55873_d_n6, assign42470_e55873_d_n7, assign42470_e55873_d_n8, assign42470_e55873_d_n9,) = {
    if ((locals.var_guard1199 == 0.0) && (locals.var_guard1200 == 0.0)) {
        let assign42470_e55867: f64 = (locals.var_sp_s_temp).sqrt();
        let assign42470_e55868: f64 = (locals.var_sp_s_pc + assign42470_e55867);
        let assign42470_e55869: f64 = (locals.var_sp_s_qc / assign42470_e55868);
        let assign42470_e55870: f64 = (2.0 * assign42470_e55869);
        let assign42470_e55871: f64 = (locals.var_sp_s_x0 + assign42470_e55870);
        (assign42470_e55871, (locals.var_sp_s_x0_dn4 + (2.0 * (((locals.var_sp_s_qc_dn4 * assign42470_e55868) - (locals.var_sp_s_qc * (locals.var_sp_s_pc_dn4 + (locals.var_sp_s_temp_dn4 / (2.0 * assign42470_e55867))))) / (assign42470_e55868 * assign42470_e55868)))), (locals.var_sp_s_x0_dn6 + (2.0 * (((locals.var_sp_s_qc_dn6 * assign42470_e55868) - (locals.var_sp_s_qc * (locals.var_sp_s_pc_dn6 + (locals.var_sp_s_temp_dn6 / (2.0 * assign42470_e55867))))) / (assign42470_e55868 * assign42470_e55868)))), (locals.var_sp_s_x0_dn7 + (2.0 * (((locals.var_sp_s_qc_dn7 * assign42470_e55868) - (locals.var_sp_s_qc * (locals.var_sp_s_pc_dn7 + (locals.var_sp_s_temp_dn7 / (2.0 * assign42470_e55867))))) / (assign42470_e55868 * assign42470_e55868)))), (locals.var_sp_s_x0_dn8 + (2.0 * (((locals.var_sp_s_qc_dn8 * assign42470_e55868) - (locals.var_sp_s_qc * (locals.var_sp_s_pc_dn8 + (locals.var_sp_s_temp_dn8 / (2.0 * assign42470_e55867))))) / (assign42470_e55868 * assign42470_e55868)))), (locals.var_sp_s_x0_dn9 + (2.0 * (((locals.var_sp_s_qc_dn9 * assign42470_e55868) - (locals.var_sp_s_qc * (locals.var_sp_s_pc_dn9 + (locals.var_sp_s_temp_dn9 / (2.0 * assign42470_e55867))))) / (assign42470_e55868 * assign42470_e55868)))),)
    } else {
        (locals.var_x_s, locals.var_x_s_dn4, locals.var_x_s_dn6, locals.var_x_s_dn7, locals.var_x_s_dn8, locals.var_x_s_dn9,)
    }
};
        locals.var_x_s = assign42470_e55873;
        locals.var_x_s_dn4 = assign42470_e55873_d_n4;
        locals.var_x_s_dn6 = assign42470_e55873_d_n6;
        locals.var_x_s_dn7 = assign42470_e55873_d_n7;
        locals.var_x_s_dn8 = assign42470_e55873_d_n8;
        locals.var_x_s_dn9 = assign42470_e55873_d_n9;
        locals.var_x_s_rv = 0.0;

        locals.var_xi1s = 0.0;
        locals.var_xi1s_dn4 = 0.0;
        locals.var_xi1s_dn6 = 0.0;
        locals.var_xi1s_dn7 = 0.0;
        locals.var_xi1s_dn8 = 0.0;
        locals.var_xi1s_dn9 = 0.0;
        locals.var_xi1s_rv = 0.0;

        locals.var_xi2s = 0.0;
        locals.var_xi2s_dn4 = 0.0;
        locals.var_xi2s_dn6 = 0.0;
        locals.var_xi2s_dn7 = 0.0;
        locals.var_xi2s_dn8 = 0.0;
        locals.var_xi2s_dn9 = 0.0;
        locals.var_xi2s_rv = 0.0;

        locals.var_delta_1s = 0.0;
        locals.var_delta_1s_dn4 = 0.0;
        locals.var_delta_1s_dn6 = 0.0;
        locals.var_delta_1s_dn7 = 0.0;
        locals.var_delta_1s_dn8 = 0.0;
        locals.var_delta_1s_dn9 = 0.0;
        locals.var_delta_1s_rv = 0.0;

        locals.var_es = 0.0;
        locals.var_es_dn4 = 0.0;
        locals.var_es_dn6 = 0.0;
        locals.var_es_dn7 = 0.0;
        locals.var_es_dn8 = 0.0;
        locals.var_es_dn9 = 0.0;
        locals.var_es_rv = 0.0;

        locals.var_ds = 0.0;
        locals.var_ds_dn4 = 0.0;
        locals.var_ds_dn6 = 0.0;
        locals.var_ds_dn7 = 0.0;
        locals.var_ds_dn8 = 0.0;
        locals.var_ds_dn9 = 0.0;
        locals.var_ds_rv = 0.0;

        locals.var_ps = 0.0;
        locals.var_ps_dn4 = 0.0;
        locals.var_ps_dn6 = 0.0;
        locals.var_ps_dn7 = 0.0;
        locals.var_ps_dn8 = 0.0;
        locals.var_ps_dn9 = 0.0;
        locals.var_ps_rv = 0.0;

        locals.var_sqs = 0.0;
        locals.var_sqs_dn4 = 0.0;
        locals.var_sqs_dn6 = 0.0;
        locals.var_sqs_dn7 = 0.0;
        locals.var_sqs_dn8 = 0.0;
        locals.var_sqs_dn9 = 0.0;
        locals.var_sqs_rv = 0.0;

        locals.var_alphas = 1.0;
        locals.var_alphas_dn4 = 0.0;
        locals.var_alphas_dn6 = 0.0;
        locals.var_alphas_dn7 = 0.0;
        locals.var_alphas_dn8 = 0.0;
        locals.var_alphas_dn9 = 0.0;
        locals.var_alphas_rv = 0.0;

        locals.var_rxcor = 1.0;
        locals.var_rxcor_dn4 = 0.0;
        locals.var_rxcor_dn6 = 0.0;
        locals.var_rxcor_dn7 = 0.0;
        locals.var_rxcor_dn8 = 0.0;
        locals.var_rxcor_dn9 = 0.0;
        locals.var_rxcor_rv = 0.0;

        let assign42570_e55885: f64 = (locals.var_xg - locals.var_x_s);
        locals.var_xgs = assign42570_e55885;
        locals.var_xgs_dn4 = (locals.var_xg_dn4 - locals.var_x_s_dn4);
        locals.var_xgs_dn6 = (locals.var_xg_dn6 - locals.var_x_s_dn6);
        locals.var_xgs_dn7 = (locals.var_xg_dn7 - locals.var_x_s_dn7);
        locals.var_xgs_dn8 = (locals.var_xg_dn8 - locals.var_x_s_dn8);
        locals.var_xgs_dn9 = (locals.var_xg_dn9 - locals.var_x_s_dn9);
        locals.var_xgs_rv = 0.0;

        locals.var_qis = 0.0;
        locals.var_qis_dn4 = 0.0;
        locals.var_qis_dn6 = 0.0;
        locals.var_qis_dn7 = 0.0;
        locals.var_qis_dn8 = 0.0;
        locals.var_qis_dn9 = 0.0;
        locals.var_qis_rv = 0.0;

        let assign42590_e55889: f64 = (locals.var_phit1 * locals.var_xgs);
        locals.var_qbs = assign42590_e55889;
        locals.var_qbs_dn4 = ((locals.var_phit1_dn4 * locals.var_xgs) + (locals.var_phit1 * locals.var_xgs_dn4));
        locals.var_qbs_dn6 = ((locals.var_phit1_dn6 * locals.var_xgs) + (locals.var_phit1 * locals.var_xgs_dn6));
        locals.var_qbs_dn7 = ((locals.var_phit1_dn7 * locals.var_xgs) + (locals.var_phit1 * locals.var_xgs_dn7));
        locals.var_qbs_dn8 = ((locals.var_phit1_dn8 * locals.var_xgs) + (locals.var_phit1 * locals.var_xgs_dn8));
        locals.var_qbs_dn9 = ((locals.var_phit1_dn9 * locals.var_xgs) + (locals.var_phit1 * locals.var_xgs_dn9));
        locals.var_qbs_rv = 0.0;

        locals.var_rhob = 1.0;
        locals.var_rhob_dn4 = 0.0;
        locals.var_rhob_dn6 = 0.0;
        locals.var_rhob_dn7 = 0.0;
        locals.var_rhob_dn8 = 0.0;
        locals.var_rhob_dn9 = 0.0;
        locals.var_rhob_rv = 0.0;

        locals.var_rhog = 1.0;
        locals.var_rhog_dn4 = 0.0;
        locals.var_rhog_dn6 = 0.0;
        locals.var_rhog_dn7 = 0.0;
        locals.var_rhog_dn8 = 0.0;
        locals.var_rhog_dn9 = 0.0;
        locals.var_rhog_rv = 0.0;

        locals.var_gmobs = 1.0;
        locals.var_gmobs_dn4 = 0.0;
        locals.var_gmobs_dn6 = 0.0;
        locals.var_gmobs_dn7 = 0.0;
        locals.var_gmobs_dn8 = 0.0;
        locals.var_gmobs_dn9 = 0.0;
        locals.var_gmobs_rv = 0.0;

        locals.var_xitsb = 1.0;
        locals.var_xitsb_dn4 = 0.0;
        locals.var_xitsb_dn6 = 0.0;
        locals.var_xitsb_dn7 = 0.0;
        locals.var_xitsb_dn8 = 0.0;
        locals.var_xitsb_dn9 = 0.0;
        locals.var_xitsb_rv = 0.0;

        locals.var_factheta = 1.0;
        locals.var_factheta_dn4 = 0.0;
        locals.var_factheta_dn6 = 0.0;
        locals.var_factheta_dn7 = 0.0;
        locals.var_factheta_dn8 = 0.0;
        locals.var_factheta_dn9 = 0.0;
        locals.var_factheta_rv = 0.0;

        let assign42650_e55897: f64 = if locals.var_xg > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1205 = assign42650_e55897;
        locals.var_guard1205_rv = 0.0;

        let (assign42660_e55907, assign42660_e55907_d_n4, assign42660_e55907_d_n6, assign42660_e55907_d_n7, assign42660_e55907_d_n8, assign42660_e55907_d_n9,) = {
    if (locals.var_guard1205 != 0.0) {
        let assign42660_e55903: f64 = (locals.var_x_s * locals.var_x_s);
        let assign42660_e55904: f64 = (2.0 + assign42660_e55903);
        let assign42660_e55905: f64 = (1.0 / assign42660_e55904);
        (assign42660_e55905, (-(((locals.var_x_s_dn4 * locals.var_x_s) + (locals.var_x_s * locals.var_x_s_dn4)) / (assign42660_e55904 * assign42660_e55904))), (-(((locals.var_x_s_dn6 * locals.var_x_s) + (locals.var_x_s * locals.var_x_s_dn6)) / (assign42660_e55904 * assign42660_e55904))), (-(((locals.var_x_s_dn7 * locals.var_x_s) + (locals.var_x_s * locals.var_x_s_dn7)) / (assign42660_e55904 * assign42660_e55904))), (-(((locals.var_x_s_dn8 * locals.var_x_s) + (locals.var_x_s * locals.var_x_s_dn8)) / (assign42660_e55904 * assign42660_e55904))), (-(((locals.var_x_s_dn9 * locals.var_x_s) + (locals.var_x_s * locals.var_x_s_dn9)) / (assign42660_e55904 * assign42660_e55904))),)
    } else {
        (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
    }
};
        locals.var_temp__blk949 = assign42660_e55907;
        locals.var_temp__blk949_dn4 = assign42660_e55907_d_n4;
        locals.var_temp__blk949_dn6 = assign42660_e55907_d_n6;
        locals.var_temp__blk949_dn7 = assign42660_e55907_d_n7;
        locals.var_temp__blk949_dn8 = assign42660_e55907_d_n8;
        locals.var_temp__blk949_dn9 = assign42660_e55907_d_n9;
        locals.var_temp__blk949_rv = 0.0;

        let (assign42670_e55915, assign42670_e55915_d_n4, assign42670_e55915_d_n6, assign42670_e55915_d_n7, assign42670_e55915_d_n8, assign42670_e55915_d_n9,) = {
    if (locals.var_guard1205 != 0.0) {
        let assign42670_e55911: f64 = (locals.var_x_s * locals.var_x_s);
        let assign42670_e55913: f64 = (assign42670_e55911 * locals.var_temp__blk949);
        (assign42670_e55913, ((((locals.var_x_s_dn4 * locals.var_x_s) + (locals.var_x_s * locals.var_x_s_dn4)) * locals.var_temp__blk949) + (assign42670_e55911 * locals.var_temp__blk949_dn4)), ((((locals.var_x_s_dn6 * locals.var_x_s) + (locals.var_x_s * locals.var_x_s_dn6)) * locals.var_temp__blk949) + (assign42670_e55911 * locals.var_temp__blk949_dn6)), ((((locals.var_x_s_dn7 * locals.var_x_s) + (locals.var_x_s * locals.var_x_s_dn7)) * locals.var_temp__blk949) + (assign42670_e55911 * locals.var_temp__blk949_dn7)), ((((locals.var_x_s_dn8 * locals.var_x_s) + (locals.var_x_s * locals.var_x_s_dn8)) * locals.var_temp__blk949) + (assign42670_e55911 * locals.var_temp__blk949_dn8)), ((((locals.var_x_s_dn9 * locals.var_x_s) + (locals.var_x_s * locals.var_x_s_dn9)) * locals.var_temp__blk949) + (assign42670_e55911 * locals.var_temp__blk949_dn9)),)
    } else {
        (locals.var_xi0s, locals.var_xi0s_dn4, locals.var_xi0s_dn6, locals.var_xi0s_dn7, locals.var_xi0s_dn8, locals.var_xi0s_dn9,)
    }
};
        locals.var_xi0s = assign42670_e55915;
        locals.var_xi0s_dn4 = assign42670_e55915_d_n4;
        locals.var_xi0s_dn6 = assign42670_e55915_d_n6;
        locals.var_xi0s_dn7 = assign42670_e55915_d_n7;
        locals.var_xi0s_dn8 = assign42670_e55915_d_n8;
        locals.var_xi0s_dn9 = assign42670_e55915_d_n9;
        locals.var_xi0s_rv = 0.0;

        let (assign42680_e55925, assign42680_e55925_d_n4, assign42680_e55925_d_n6, assign42680_e55925_d_n7, assign42680_e55925_d_n8, assign42680_e55925_d_n9,) = {
    if (locals.var_guard1205 != 0.0) {
        let assign42680_e55920: f64 = (locals.var_x_s * locals.var_temp__blk949);
        let assign42680_e55922: f64 = (assign42680_e55920 * locals.var_temp__blk949);
        let assign42680_e55923: f64 = (4.0 * assign42680_e55922);
        (assign42680_e55923, (4.0 * ((((locals.var_x_s_dn4 * locals.var_temp__blk949) + (locals.var_x_s * locals.var_temp__blk949_dn4)) * locals.var_temp__blk949) + (assign42680_e55920 * locals.var_temp__blk949_dn4))), (4.0 * ((((locals.var_x_s_dn6 * locals.var_temp__blk949) + (locals.var_x_s * locals.var_temp__blk949_dn6)) * locals.var_temp__blk949) + (assign42680_e55920 * locals.var_temp__blk949_dn6))), (4.0 * ((((locals.var_x_s_dn7 * locals.var_temp__blk949) + (locals.var_x_s * locals.var_temp__blk949_dn7)) * locals.var_temp__blk949) + (assign42680_e55920 * locals.var_temp__blk949_dn7))), (4.0 * ((((locals.var_x_s_dn8 * locals.var_temp__blk949) + (locals.var_x_s * locals.var_temp__blk949_dn8)) * locals.var_temp__blk949) + (assign42680_e55920 * locals.var_temp__blk949_dn8))), (4.0 * ((((locals.var_x_s_dn9 * locals.var_temp__blk949) + (locals.var_x_s * locals.var_temp__blk949_dn9)) * locals.var_temp__blk949) + (assign42680_e55920 * locals.var_temp__blk949_dn9))),)
    } else {
        (locals.var_xi1s, locals.var_xi1s_dn4, locals.var_xi1s_dn6, locals.var_xi1s_dn7, locals.var_xi1s_dn8, locals.var_xi1s_dn9,)
    }
};
        locals.var_xi1s = assign42680_e55925;
        locals.var_xi1s_dn4 = assign42680_e55925_d_n4;
        locals.var_xi1s_dn6 = assign42680_e55925_d_n6;
        locals.var_xi1s_dn7 = assign42680_e55925_d_n7;
        locals.var_xi1s_dn8 = assign42680_e55925_d_n8;
        locals.var_xi1s_dn9 = assign42680_e55925_d_n9;
        locals.var_xi1s_rv = 0.0;

        let (assign42690_e55939, assign42690_e55939_d_n4, assign42690_e55939_d_n6, assign42690_e55939_d_n7, assign42690_e55939_d_n8, assign42690_e55939_d_n9,) = {
    if (locals.var_guard1205 != 0.0) {
        let assign42690_e55929: f64 = (8.0 * locals.var_temp__blk949);
        let assign42690_e55932: f64 = (12.0 * locals.var_xi0s);
        let assign42690_e55933: f64 = (assign42690_e55929 - assign42690_e55932);
        let assign42690_e55935: f64 = (assign42690_e55933 * locals.var_temp__blk949);
        let assign42690_e55937: f64 = (assign42690_e55935 * locals.var_temp__blk949);
        (assign42690_e55937, ((((((8.0 * locals.var_temp__blk949_dn4) - (12.0 * locals.var_xi0s_dn4)) * locals.var_temp__blk949) + (assign42690_e55933 * locals.var_temp__blk949_dn4)) * locals.var_temp__blk949) + (assign42690_e55935 * locals.var_temp__blk949_dn4)), ((((((8.0 * locals.var_temp__blk949_dn6) - (12.0 * locals.var_xi0s_dn6)) * locals.var_temp__blk949) + (assign42690_e55933 * locals.var_temp__blk949_dn6)) * locals.var_temp__blk949) + (assign42690_e55935 * locals.var_temp__blk949_dn6)), ((((((8.0 * locals.var_temp__blk949_dn7) - (12.0 * locals.var_xi0s_dn7)) * locals.var_temp__blk949) + (assign42690_e55933 * locals.var_temp__blk949_dn7)) * locals.var_temp__blk949) + (assign42690_e55935 * locals.var_temp__blk949_dn7)), ((((((8.0 * locals.var_temp__blk949_dn8) - (12.0 * locals.var_xi0s_dn8)) * locals.var_temp__blk949) + (assign42690_e55933 * locals.var_temp__blk949_dn8)) * locals.var_temp__blk949) + (assign42690_e55935 * locals.var_temp__blk949_dn8)), ((((((8.0 * locals.var_temp__blk949_dn9) - (12.0 * locals.var_xi0s_dn9)) * locals.var_temp__blk949) + (assign42690_e55933 * locals.var_temp__blk949_dn9)) * locals.var_temp__blk949) + (assign42690_e55935 * locals.var_temp__blk949_dn9)),)
    } else {
        (locals.var_xi2s, locals.var_xi2s_dn4, locals.var_xi2s_dn6, locals.var_xi2s_dn7, locals.var_xi2s_dn8, locals.var_xi2s_dn9,)
    }
};
        locals.var_xi2s = assign42690_e55939;
        locals.var_xi2s_dn4 = assign42690_e55939_d_n4;
        locals.var_xi2s_dn6 = assign42690_e55939_d_n6;
        locals.var_xi2s_dn7 = assign42690_e55939_d_n7;
        locals.var_xi2s_dn8 = assign42690_e55939_d_n8;
        locals.var_xi2s_dn9 = assign42690_e55939_d_n9;
        locals.var_xi2s_rv = 0.0;

        let (assign42700_e55943, assign42700_e55943_d_n4, assign42700_e55943_d_n6, assign42700_e55943_d_n7, assign42700_e55943_d_n8, assign42700_e55943_d_n9,) = {
    if (locals.var_guard1205 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_delta_1s, locals.var_delta_1s_dn4, locals.var_delta_1s_dn6, locals.var_delta_1s_dn7, locals.var_delta_1s_dn8, locals.var_delta_1s_dn9,)
    }
};
        locals.var_delta_1s = assign42700_e55943;
        locals.var_delta_1s_dn4 = assign42700_e55943_d_n4;
        locals.var_delta_1s_dn6 = assign42700_e55943_d_n6;
        locals.var_delta_1s_dn7 = assign42700_e55943_d_n7;
        locals.var_delta_1s_dn8 = assign42700_e55943_d_n8;
        locals.var_delta_1s_dn9 = assign42700_e55943_d_n9;
        locals.var_delta_1s_rv = 0.0;

        let assign42710_e55946: f64 = if locals.var_x_s < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard1206 = assign42710_e55946;
        locals.var_guard1206_rv = 0.0;

        let (assign42720_e55953, assign42720_e55953_d_n4, assign42720_e55953_d_n6, assign42720_e55953_d_n7, assign42720_e55953_d_n8, assign42720_e55953_d_n9,) = {
    if ((locals.var_guard1205 != 0.0) && (locals.var_guard1206 != 0.0)) {
        let assign42720_e55951: f64 = (locals.var_x_s).exp();
        (assign42720_e55951, (assign42720_e55951 * locals.var_x_s_dn4), (assign42720_e55951 * locals.var_x_s_dn6), (assign42720_e55951 * locals.var_x_s_dn7), (assign42720_e55951 * locals.var_x_s_dn8), (assign42720_e55951 * locals.var_x_s_dn9),)
    } else {
        (locals.var_delta_1s, locals.var_delta_1s_dn4, locals.var_delta_1s_dn6, locals.var_delta_1s_dn7, locals.var_delta_1s_dn8, locals.var_delta_1s_dn9,)
    }
};
        locals.var_delta_1s = assign42720_e55953;
        locals.var_delta_1s_dn4 = assign42720_e55953_d_n4;
        locals.var_delta_1s_dn6 = assign42720_e55953_d_n6;
        locals.var_delta_1s_dn7 = assign42720_e55953_d_n7;
        locals.var_delta_1s_dn8 = assign42720_e55953_d_n8;
        locals.var_delta_1s_dn9 = assign42720_e55953_d_n9;
        locals.var_delta_1s_rv = 0.0;

        let (assign42730_e55961, assign42730_e55961_d_n4, assign42730_e55961_d_n6, assign42730_e55961_d_n7, assign42730_e55961_d_n8, assign42730_e55961_d_n9,) = {
    if ((locals.var_guard1205 != 0.0) && (locals.var_guard1206 != 0.0)) {
        let assign42730_e55959: f64 = (1.0 / locals.var_delta_1s);
        (assign42730_e55959, (-(locals.var_delta_1s_dn4 / (locals.var_delta_1s * locals.var_delta_1s))), (-(locals.var_delta_1s_dn6 / (locals.var_delta_1s * locals.var_delta_1s))), (-(locals.var_delta_1s_dn7 / (locals.var_delta_1s * locals.var_delta_1s))), (-(locals.var_delta_1s_dn8 / (locals.var_delta_1s * locals.var_delta_1s))), (-(locals.var_delta_1s_dn9 / (locals.var_delta_1s * locals.var_delta_1s))),)
    } else {
        (locals.var_es, locals.var_es_dn4, locals.var_es_dn6, locals.var_es_dn7, locals.var_es_dn8, locals.var_es_dn9,)
    }
};
        locals.var_es = assign42730_e55961;
        locals.var_es_dn4 = assign42730_e55961_d_n4;
        locals.var_es_dn6 = assign42730_e55961_d_n6;
        locals.var_es_dn7 = assign42730_e55961_d_n7;
        locals.var_es_dn8 = assign42730_e55961_d_n8;
        locals.var_es_dn9 = assign42730_e55961_d_n9;
        locals.var_es_rv = 0.0;

        let (assign42740_e55969, assign42740_e55969_d_n4, assign42740_e55969_d_n6, assign42740_e55969_d_n7, assign42740_e55969_d_n8, assign42740_e55969_d_n9,) = {
    if ((locals.var_guard1205 != 0.0) && (locals.var_guard1206 != 0.0)) {
        let assign42740_e55967: f64 = (locals.var_delta_ns * locals.var_delta_1s);
        (assign42740_e55967, ((locals.var_delta_ns_dn4 * locals.var_delta_1s) + (locals.var_delta_ns * locals.var_delta_1s_dn4)), ((locals.var_delta_ns_dn6 * locals.var_delta_1s) + (locals.var_delta_ns * locals.var_delta_1s_dn6)), ((locals.var_delta_ns_dn7 * locals.var_delta_1s) + (locals.var_delta_ns * locals.var_delta_1s_dn7)), ((locals.var_delta_ns_dn8 * locals.var_delta_1s) + (locals.var_delta_ns * locals.var_delta_1s_dn8)), ((locals.var_delta_ns_dn9 * locals.var_delta_1s) + (locals.var_delta_ns * locals.var_delta_1s_dn9)),)
    } else {
        (locals.var_delta_1s, locals.var_delta_1s_dn4, locals.var_delta_1s_dn6, locals.var_delta_1s_dn7, locals.var_delta_1s_dn8, locals.var_delta_1s_dn9,)
    }
};
        locals.var_delta_1s = assign42740_e55969;
        locals.var_delta_1s_dn4 = assign42740_e55969_d_n4;
        locals.var_delta_1s_dn6 = assign42740_e55969_d_n6;
        locals.var_delta_1s_dn7 = assign42740_e55969_d_n7;
        locals.var_delta_1s_dn8 = assign42740_e55969_d_n8;
        locals.var_delta_1s_dn9 = assign42740_e55969_d_n9;
        locals.var_delta_1s_rv = 0.0;

        let assign42750_e55973: f64 = (locals.var_xn_s - 230.25850929940458);
        let assign42750_e55974: f64 = if locals.var_x_s > assign42750_e55973 { 1.0 } else { 0.0 };
        locals.var_guard1207 = assign42750_e55974;
        locals.var_guard1207_rv = 0.0;

        let (assign42760_e55986, assign42760_e55986_d_n4, assign42760_e55986_d_n6, assign42760_e55986_d_n7, assign42760_e55986_d_n8, assign42760_e55986_d_n9,) = {
    if (((locals.var_guard1205 != 0.0) && (locals.var_guard1206 == 0.0)) && (locals.var_guard1207 != 0.0)) {
        let assign42760_e55983: f64 = (locals.var_x_s - locals.var_xn_s);
        let assign42760_e55984: f64 = (assign42760_e55983).exp();
        (assign42760_e55984, (assign42760_e55984 * (locals.var_x_s_dn4 - locals.var_xn_s_dn4)), (assign42760_e55984 * (locals.var_x_s_dn6 - locals.var_xn_s_dn6)), (assign42760_e55984 * (locals.var_x_s_dn7 - locals.var_xn_s_dn7)), (assign42760_e55984 * (locals.var_x_s_dn8 - locals.var_xn_s_dn8)), (assign42760_e55984 * (locals.var_x_s_dn9 - locals.var_xn_s_dn9)),)
    } else {
        (locals.var_delta_1s, locals.var_delta_1s_dn4, locals.var_delta_1s_dn6, locals.var_delta_1s_dn7, locals.var_delta_1s_dn8, locals.var_delta_1s_dn9,)
    }
};
        locals.var_delta_1s = assign42760_e55986;
        locals.var_delta_1s_dn4 = assign42760_e55986_d_n4;
        locals.var_delta_1s_dn6 = assign42760_e55986_d_n6;
        locals.var_delta_1s_dn7 = assign42760_e55986_d_n7;
        locals.var_delta_1s_dn8 = assign42760_e55986_d_n8;
        locals.var_delta_1s_dn9 = assign42760_e55986_d_n9;
        locals.var_delta_1s_rv = 0.0;

        let (assign42770_e55997, assign42770_e55997_d_n4, assign42770_e55997_d_n6, assign42770_e55997_d_n7, assign42770_e55997_d_n8, assign42770_e55997_d_n9,) = {
    if (((locals.var_guard1205 != 0.0) && (locals.var_guard1206 == 0.0)) && (locals.var_guard1207 != 0.0)) {
        let assign42770_e55995: f64 = (locals.var_delta_ns / locals.var_delta_1s);
        (assign42770_e55995, (((locals.var_delta_ns_dn4 * locals.var_delta_1s) - (locals.var_delta_ns * locals.var_delta_1s_dn4)) / (locals.var_delta_1s * locals.var_delta_1s)), (((locals.var_delta_ns_dn6 * locals.var_delta_1s) - (locals.var_delta_ns * locals.var_delta_1s_dn6)) / (locals.var_delta_1s * locals.var_delta_1s)), (((locals.var_delta_ns_dn7 * locals.var_delta_1s) - (locals.var_delta_ns * locals.var_delta_1s_dn7)) / (locals.var_delta_1s * locals.var_delta_1s)), (((locals.var_delta_ns_dn8 * locals.var_delta_1s) - (locals.var_delta_ns * locals.var_delta_1s_dn8)) / (locals.var_delta_1s * locals.var_delta_1s)), (((locals.var_delta_ns_dn9 * locals.var_delta_1s) - (locals.var_delta_ns * locals.var_delta_1s_dn9)) / (locals.var_delta_1s * locals.var_delta_1s)),)
    } else {
        (locals.var_es, locals.var_es_dn4, locals.var_es_dn6, locals.var_es_dn7, locals.var_es_dn8, locals.var_es_dn9,)
    }
};
        locals.var_es = assign42770_e55997;
        locals.var_es_dn4 = assign42770_e55997_d_n4;
        locals.var_es_dn6 = assign42770_e55997_d_n6;
        locals.var_es_dn7 = assign42770_e55997_d_n7;
        locals.var_es_dn8 = assign42770_e55997_d_n8;
        locals.var_es_dn9 = assign42770_e55997_d_n9;
        locals.var_es_rv = 0.0;

        let (assign42780_e56035, assign42780_e56035_d_n4, assign42780_e56035_d_n6, assign42780_e56035_d_n7, assign42780_e56035_d_n8, assign42780_e56035_d_n9,) = {
    if (((locals.var_guard1205 != 0.0) && (locals.var_guard1206 == 0.0)) && (locals.var_guard1207 == 0.0)) {
        let assign42780_e56009: f64 = (locals.var_xn_s - locals.var_x_s);
        let assign42780_e56011: f64 = (assign42780_e56009 - 230.25850929940458);
        let assign42780_e56016: f64 = (locals.var_xn_s - locals.var_x_s);
        let assign42780_e56018: f64 = (assign42780_e56016 - 230.25850929940458);
        let assign42780_e56022: f64 = (locals.var_xn_s - locals.var_x_s);
        let assign42780_e56024: f64 = (assign42780_e56022 - 230.25850929940458);
        let assign42780_e56026: f64 = (assign42780_e56024 * 0.3333333333333333);
        let assign42780_e56027: f64 = (1.0 + assign42780_e56026);
        let assign42780_e56028: f64 = (assign42780_e56018 * assign42780_e56027);
        let assign42780_e56029: f64 = (0.5 * assign42780_e56028);
        let assign42780_e56030: f64 = (1.0 + assign42780_e56029);
        let assign42780_e56031: f64 = (assign42780_e56011 * assign42780_e56030);
        let assign42780_e56032: f64 = (1.0 + assign42780_e56031);
        let assign42780_e56033: f64 = (1e-100 / assign42780_e56032);
        (assign42780_e56033, (-((1e-100 * (((locals.var_xn_s_dn4 - locals.var_x_s_dn4) * assign42780_e56030) + (assign42780_e56011 * (0.5 * (((locals.var_xn_s_dn4 - locals.var_x_s_dn4) * assign42780_e56027) + (assign42780_e56018 * ((locals.var_xn_s_dn4 - locals.var_x_s_dn4) * 0.3333333333333333))))))) / (assign42780_e56032 * assign42780_e56032))), (-((1e-100 * (((locals.var_xn_s_dn6 - locals.var_x_s_dn6) * assign42780_e56030) + (assign42780_e56011 * (0.5 * (((locals.var_xn_s_dn6 - locals.var_x_s_dn6) * assign42780_e56027) + (assign42780_e56018 * ((locals.var_xn_s_dn6 - locals.var_x_s_dn6) * 0.3333333333333333))))))) / (assign42780_e56032 * assign42780_e56032))), (-((1e-100 * (((locals.var_xn_s_dn7 - locals.var_x_s_dn7) * assign42780_e56030) + (assign42780_e56011 * (0.5 * (((locals.var_xn_s_dn7 - locals.var_x_s_dn7) * assign42780_e56027) + (assign42780_e56018 * ((locals.var_xn_s_dn7 - locals.var_x_s_dn7) * 0.3333333333333333))))))) / (assign42780_e56032 * assign42780_e56032))), (-((1e-100 * (((locals.var_xn_s_dn8 - locals.var_x_s_dn8) * assign42780_e56030) + (assign42780_e56011 * (0.5 * (((locals.var_xn_s_dn8 - locals.var_x_s_dn8) * assign42780_e56027) + (assign42780_e56018 * ((locals.var_xn_s_dn8 - locals.var_x_s_dn8) * 0.3333333333333333))))))) / (assign42780_e56032 * assign42780_e56032))), (-((1e-100 * (((locals.var_xn_s_dn9 - locals.var_x_s_dn9) * assign42780_e56030) + (assign42780_e56011 * (0.5 * (((locals.var_xn_s_dn9 - locals.var_x_s_dn9) * assign42780_e56027) + (assign42780_e56018 * ((locals.var_xn_s_dn9 - locals.var_x_s_dn9) * 0.3333333333333333))))))) / (assign42780_e56032 * assign42780_e56032))),)
    } else {
        (locals.var_delta_1s, locals.var_delta_1s_dn4, locals.var_delta_1s_dn6, locals.var_delta_1s_dn7, locals.var_delta_1s_dn8, locals.var_delta_1s_dn9,)
    }
};
        locals.var_delta_1s = assign42780_e56035;
        locals.var_delta_1s_dn4 = assign42780_e56035_d_n4;
        locals.var_delta_1s_dn6 = assign42780_e56035_d_n6;
        locals.var_delta_1s_dn7 = assign42780_e56035_d_n7;
        locals.var_delta_1s_dn8 = assign42780_e56035_d_n8;
        locals.var_delta_1s_dn9 = assign42780_e56035_d_n9;
        locals.var_delta_1s_rv = 0.0;

        let (assign42790_e56067, assign42790_e56067_d_n4, assign42790_e56067_d_n6, assign42790_e56067_d_n7, assign42790_e56067_d_n8, assign42790_e56067_d_n9,) = {
    if (((locals.var_guard1205 != 0.0) && (locals.var_guard1206 == 0.0)) && (locals.var_guard1207 == 0.0)) {
        let assign42790_e56047: f64 = (locals.var_x_s - 230.25850929940458);
        let assign42790_e56052: f64 = (locals.var_x_s - 230.25850929940458);
        let assign42790_e56056: f64 = (locals.var_x_s - 230.25850929940458);
        let assign42790_e56058: f64 = (assign42790_e56056 * 0.3333333333333333);
        let assign42790_e56059: f64 = (1.0 + assign42790_e56058);
        let assign42790_e56060: f64 = (assign42790_e56052 * assign42790_e56059);
        let assign42790_e56061: f64 = (0.5 * assign42790_e56060);
        let assign42790_e56062: f64 = (1.0 + assign42790_e56061);
        let assign42790_e56063: f64 = (assign42790_e56047 * assign42790_e56062);
        let assign42790_e56064: f64 = (1.0 + assign42790_e56063);
        let assign42790_e56065: f64 = (1e-100 / assign42790_e56064);
        (assign42790_e56065, (-((1e-100 * ((locals.var_x_s_dn4 * assign42790_e56062) + (assign42790_e56047 * (0.5 * ((locals.var_x_s_dn4 * assign42790_e56059) + (assign42790_e56052 * (locals.var_x_s_dn4 * 0.3333333333333333))))))) / (assign42790_e56064 * assign42790_e56064))), (-((1e-100 * ((locals.var_x_s_dn6 * assign42790_e56062) + (assign42790_e56047 * (0.5 * ((locals.var_x_s_dn6 * assign42790_e56059) + (assign42790_e56052 * (locals.var_x_s_dn6 * 0.3333333333333333))))))) / (assign42790_e56064 * assign42790_e56064))), (-((1e-100 * ((locals.var_x_s_dn7 * assign42790_e56062) + (assign42790_e56047 * (0.5 * ((locals.var_x_s_dn7 * assign42790_e56059) + (assign42790_e56052 * (locals.var_x_s_dn7 * 0.3333333333333333))))))) / (assign42790_e56064 * assign42790_e56064))), (-((1e-100 * ((locals.var_x_s_dn8 * assign42790_e56062) + (assign42790_e56047 * (0.5 * ((locals.var_x_s_dn8 * assign42790_e56059) + (assign42790_e56052 * (locals.var_x_s_dn8 * 0.3333333333333333))))))) / (assign42790_e56064 * assign42790_e56064))), (-((1e-100 * ((locals.var_x_s_dn9 * assign42790_e56062) + (assign42790_e56047 * (0.5 * ((locals.var_x_s_dn9 * assign42790_e56059) + (assign42790_e56052 * (locals.var_x_s_dn9 * 0.3333333333333333))))))) / (assign42790_e56064 * assign42790_e56064))),)
    } else {
        (locals.var_es, locals.var_es_dn4, locals.var_es_dn6, locals.var_es_dn7, locals.var_es_dn8, locals.var_es_dn9,)
    }
};
        locals.var_es = assign42790_e56067;
        locals.var_es_dn4 = assign42790_e56067_d_n4;
        locals.var_es_dn6 = assign42790_e56067_d_n6;
        locals.var_es_dn7 = assign42790_e56067_d_n7;
        locals.var_es_dn8 = assign42790_e56067_d_n8;
        locals.var_es_dn9 = assign42790_e56067_d_n9;
        locals.var_es_rv = 0.0;

        let (assign42800_e56079, assign42800_e56079_d_n4, assign42800_e56079_d_n6, assign42800_e56079_d_n7, assign42800_e56079_d_n8, assign42800_e56079_d_n9,) = {
    if (locals.var_guard1205 != 0.0) {
        let assign42800_e56073: f64 = (locals.var_x_s + 1.0);
        let assign42800_e56075: f64 = (assign42800_e56073 + locals.var_xi0s);
        let assign42800_e56076: f64 = (locals.var_delta_ns * assign42800_e56075);
        let assign42800_e56077: f64 = (locals.var_delta_1s - assign42800_e56076);
        (assign42800_e56077, (locals.var_delta_1s_dn4 - ((locals.var_delta_ns_dn4 * assign42800_e56075) + (locals.var_delta_ns * (locals.var_x_s_dn4 + locals.var_xi0s_dn4)))), (locals.var_delta_1s_dn6 - ((locals.var_delta_ns_dn6 * assign42800_e56075) + (locals.var_delta_ns * (locals.var_x_s_dn6 + locals.var_xi0s_dn6)))), (locals.var_delta_1s_dn7 - ((locals.var_delta_ns_dn7 * assign42800_e56075) + (locals.var_delta_ns * (locals.var_x_s_dn7 + locals.var_xi0s_dn7)))), (locals.var_delta_1s_dn8 - ((locals.var_delta_ns_dn8 * assign42800_e56075) + (locals.var_delta_ns * (locals.var_x_s_dn8 + locals.var_xi0s_dn8)))), (locals.var_delta_1s_dn9 - ((locals.var_delta_ns_dn9 * assign42800_e56075) + (locals.var_delta_ns * (locals.var_x_s_dn9 + locals.var_xi0s_dn9)))),)
    } else {
        (locals.var_ds, locals.var_ds_dn4, locals.var_ds_dn6, locals.var_ds_dn7, locals.var_ds_dn8, locals.var_ds_dn9,)
    }
};
        locals.var_ds = assign42800_e56079;
        locals.var_ds_dn4 = assign42800_e56079_d_n4;
        locals.var_ds_dn6 = assign42800_e56079_d_n6;
        locals.var_ds_dn7 = assign42800_e56079_d_n7;
        locals.var_ds_dn8 = assign42800_e56079_d_n8;
        locals.var_ds_dn9 = assign42800_e56079_d_n9;
        locals.var_ds_rv = 0.0;

        let assign42810_e56082: f64 = if locals.var_x_s < 1e-5 { 1.0 } else { 0.0 };
        locals.var_guard1208 = assign42810_e56082;
        locals.var_guard1208_rv = 0.0;

        let (assign42820_e56104, assign42820_e56104_d_n4, assign42820_e56104_d_n6, assign42820_e56104_d_n7, assign42820_e56104_d_n8, assign42820_e56104_d_n9,) = {
    if ((locals.var_guard1205 != 0.0) && (locals.var_guard1208 != 0.0)) {
        let assign42820_e56089: f64 = (locals.var_x_s * locals.var_x_s);
        let assign42820_e56096: f64 = (0.25 * locals.var_x_s);
        let assign42820_e56097: f64 = (1.0 - assign42820_e56096);
        let assign42820_e56098: f64 = (locals.var_x_s * assign42820_e56097);
        let assign42820_e56099: f64 = (0.3333333333333333 * assign42820_e56098);
        let assign42820_e56100: f64 = (1.0 - assign42820_e56099);
        let assign42820_e56101: f64 = (assign42820_e56089 * assign42820_e56100);
        let assign42820_e56102: f64 = (0.5 * assign42820_e56101);
        (assign42820_e56102, (0.5 * ((((locals.var_x_s_dn4 * locals.var_x_s) + (locals.var_x_s * locals.var_x_s_dn4)) * assign42820_e56100) + (assign42820_e56089 * (-(0.3333333333333333 * ((locals.var_x_s_dn4 * assign42820_e56097) + (locals.var_x_s * (-(0.25 * locals.var_x_s_dn4))))))))), (0.5 * ((((locals.var_x_s_dn6 * locals.var_x_s) + (locals.var_x_s * locals.var_x_s_dn6)) * assign42820_e56100) + (assign42820_e56089 * (-(0.3333333333333333 * ((locals.var_x_s_dn6 * assign42820_e56097) + (locals.var_x_s * (-(0.25 * locals.var_x_s_dn6))))))))), (0.5 * ((((locals.var_x_s_dn7 * locals.var_x_s) + (locals.var_x_s * locals.var_x_s_dn7)) * assign42820_e56100) + (assign42820_e56089 * (-(0.3333333333333333 * ((locals.var_x_s_dn7 * assign42820_e56097) + (locals.var_x_s * (-(0.25 * locals.var_x_s_dn7))))))))), (0.5 * ((((locals.var_x_s_dn8 * locals.var_x_s) + (locals.var_x_s * locals.var_x_s_dn8)) * assign42820_e56100) + (assign42820_e56089 * (-(0.3333333333333333 * ((locals.var_x_s_dn8 * assign42820_e56097) + (locals.var_x_s * (-(0.25 * locals.var_x_s_dn8))))))))), (0.5 * ((((locals.var_x_s_dn9 * locals.var_x_s) + (locals.var_x_s * locals.var_x_s_dn9)) * assign42820_e56100) + (assign42820_e56089 * (-(0.3333333333333333 * ((locals.var_x_s_dn9 * assign42820_e56097) + (locals.var_x_s * (-(0.25 * locals.var_x_s_dn9))))))))),)
    } else {
        (locals.var_ps, locals.var_ps_dn4, locals.var_ps_dn6, locals.var_ps_dn7, locals.var_ps_dn8, locals.var_ps_dn9,)
    }
};
        locals.var_ps = assign42820_e56104;
        locals.var_ps_dn4 = assign42820_e56104_d_n4;
        locals.var_ps_dn6 = assign42820_e56104_d_n6;
        locals.var_ps_dn7 = assign42820_e56104_d_n7;
        locals.var_ps_dn8 = assign42820_e56104_d_n8;
        locals.var_ps_dn9 = assign42820_e56104_d_n9;
        locals.var_ps_rv = 0.0;

        let (assign42830_e56124, assign42830_e56124_d_n4, assign42830_e56124_d_n6, assign42830_e56124_d_n7, assign42830_e56124_d_n8, assign42830_e56124_d_n9,) = {
    if ((locals.var_guard1205 != 0.0) && (locals.var_guard1208 != 0.0)) {
        let assign42830_e56111: f64 = (locals.var_delta_ns * locals.var_x_s);
        let assign42830_e56113: f64 = (assign42830_e56111 * locals.var_x_s);
        let assign42830_e56115: f64 = (assign42830_e56113 * locals.var_x_s);
        let assign42830_e56119: f64 = (1.75 * locals.var_x_s);
        let assign42830_e56120: f64 = (1.0 + assign42830_e56119);
        let assign42830_e56121: f64 = (assign42830_e56115 * assign42830_e56120);
        let assign42830_e56122: f64 = (0.16666666666666666 * assign42830_e56121);
        (assign42830_e56122, (0.16666666666666666 * ((((((((locals.var_delta_ns_dn4 * locals.var_x_s) + (locals.var_delta_ns * locals.var_x_s_dn4)) * locals.var_x_s) + (assign42830_e56111 * locals.var_x_s_dn4)) * locals.var_x_s) + (assign42830_e56113 * locals.var_x_s_dn4)) * assign42830_e56120) + (assign42830_e56115 * (1.75 * locals.var_x_s_dn4)))), (0.16666666666666666 * ((((((((locals.var_delta_ns_dn6 * locals.var_x_s) + (locals.var_delta_ns * locals.var_x_s_dn6)) * locals.var_x_s) + (assign42830_e56111 * locals.var_x_s_dn6)) * locals.var_x_s) + (assign42830_e56113 * locals.var_x_s_dn6)) * assign42830_e56120) + (assign42830_e56115 * (1.75 * locals.var_x_s_dn6)))), (0.16666666666666666 * ((((((((locals.var_delta_ns_dn7 * locals.var_x_s) + (locals.var_delta_ns * locals.var_x_s_dn7)) * locals.var_x_s) + (assign42830_e56111 * locals.var_x_s_dn7)) * locals.var_x_s) + (assign42830_e56113 * locals.var_x_s_dn7)) * assign42830_e56120) + (assign42830_e56115 * (1.75 * locals.var_x_s_dn7)))), (0.16666666666666666 * ((((((((locals.var_delta_ns_dn8 * locals.var_x_s) + (locals.var_delta_ns * locals.var_x_s_dn8)) * locals.var_x_s) + (assign42830_e56111 * locals.var_x_s_dn8)) * locals.var_x_s) + (assign42830_e56113 * locals.var_x_s_dn8)) * assign42830_e56120) + (assign42830_e56115 * (1.75 * locals.var_x_s_dn8)))), (0.16666666666666666 * ((((((((locals.var_delta_ns_dn9 * locals.var_x_s) + (locals.var_delta_ns * locals.var_x_s_dn9)) * locals.var_x_s) + (assign42830_e56111 * locals.var_x_s_dn9)) * locals.var_x_s) + (assign42830_e56113 * locals.var_x_s_dn9)) * assign42830_e56120) + (assign42830_e56115 * (1.75 * locals.var_x_s_dn9)))),)
    } else {
        (locals.var_ds, locals.var_ds_dn4, locals.var_ds_dn6, locals.var_ds_dn7, locals.var_ds_dn8, locals.var_ds_dn9,)
    }
};
        locals.var_ds = assign42830_e56124;
        locals.var_ds_dn4 = assign42830_e56124_d_n4;
        locals.var_ds_dn6 = assign42830_e56124_d_n6;
        locals.var_ds_dn7 = assign42830_e56124_d_n7;
        locals.var_ds_dn8 = assign42830_e56124_d_n8;
        locals.var_ds_dn9 = assign42830_e56124_d_n9;
        locals.var_ds_rv = 0.0;

        let (assign42840_e56141, assign42840_e56141_d_n4, assign42840_e56141_d_n6, assign42840_e56141_d_n7, assign42840_e56141_d_n8, assign42840_e56141_d_n9,) = {
    if ((locals.var_guard1205 != 0.0) && (locals.var_guard1208 != 0.0)) {
        let assign42840_e56134: f64 = (0.25 * locals.var_x_s);
        let assign42840_e56135: f64 = (1.0 - assign42840_e56134);
        let assign42840_e56136: f64 = (locals.var_x_s * assign42840_e56135);
        let assign42840_e56137: f64 = (0.3333333333333333 * assign42840_e56136);
        let assign42840_e56138: f64 = (1.0 - assign42840_e56137);
        let assign42840_e56139: f64 = (assign42840_e56138).sqrt();
        (assign42840_e56139, ((-(0.3333333333333333 * ((locals.var_x_s_dn4 * assign42840_e56135) + (locals.var_x_s * (-(0.25 * locals.var_x_s_dn4)))))) / (2.0 * assign42840_e56139)), ((-(0.3333333333333333 * ((locals.var_x_s_dn6 * assign42840_e56135) + (locals.var_x_s * (-(0.25 * locals.var_x_s_dn6)))))) / (2.0 * assign42840_e56139)), ((-(0.3333333333333333 * ((locals.var_x_s_dn7 * assign42840_e56135) + (locals.var_x_s * (-(0.25 * locals.var_x_s_dn7)))))) / (2.0 * assign42840_e56139)), ((-(0.3333333333333333 * ((locals.var_x_s_dn8 * assign42840_e56135) + (locals.var_x_s * (-(0.25 * locals.var_x_s_dn8)))))) / (2.0 * assign42840_e56139)), ((-(0.3333333333333333 * ((locals.var_x_s_dn9 * assign42840_e56135) + (locals.var_x_s * (-(0.25 * locals.var_x_s_dn9)))))) / (2.0 * assign42840_e56139)),)
    } else {
        (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
    }
};
        locals.var_temp__blk949 = assign42840_e56141;
        locals.var_temp__blk949_dn4 = assign42840_e56141_d_n4;
        locals.var_temp__blk949_dn6 = assign42840_e56141_d_n6;
        locals.var_temp__blk949_dn7 = assign42840_e56141_d_n7;
        locals.var_temp__blk949_dn8 = assign42840_e56141_d_n8;
        locals.var_temp__blk949_dn9 = assign42840_e56141_d_n9;
        locals.var_temp__blk949_rv = 0.0;

        let (assign42850_e56151, assign42850_e56151_d_n4, assign42850_e56151_d_n6, assign42850_e56151_d_n7, assign42850_e56151_d_n8, assign42850_e56151_d_n9,) = {
    if ((locals.var_guard1205 != 0.0) && (locals.var_guard1208 != 0.0)) {
        let assign42850_e56148: f64 = (locals.var_x_s * locals.var_temp__blk949);
        let assign42850_e56149: f64 = (0.7071067811865475 * assign42850_e56148);
        (assign42850_e56149, (0.7071067811865475 * ((locals.var_x_s_dn4 * locals.var_temp__blk949) + (locals.var_x_s * locals.var_temp__blk949_dn4))), (0.7071067811865475 * ((locals.var_x_s_dn6 * locals.var_temp__blk949) + (locals.var_x_s * locals.var_temp__blk949_dn6))), (0.7071067811865475 * ((locals.var_x_s_dn7 * locals.var_temp__blk949) + (locals.var_x_s * locals.var_temp__blk949_dn7))), (0.7071067811865475 * ((locals.var_x_s_dn8 * locals.var_temp__blk949) + (locals.var_x_s * locals.var_temp__blk949_dn8))), (0.7071067811865475 * ((locals.var_x_s_dn9 * locals.var_temp__blk949) + (locals.var_x_s * locals.var_temp__blk949_dn9))),)
    } else {
        (locals.var_sqs, locals.var_sqs_dn4, locals.var_sqs_dn6, locals.var_sqs_dn7, locals.var_sqs_dn8, locals.var_sqs_dn9,)
    }
};
        locals.var_sqs = assign42850_e56151;
        locals.var_sqs_dn4 = assign42850_e56151_d_n4;
        locals.var_sqs_dn6 = assign42850_e56151_d_n6;
        locals.var_sqs_dn7 = assign42850_e56151_d_n7;
        locals.var_sqs_dn8 = assign42850_e56151_d_n8;
        locals.var_sqs_dn9 = assign42850_e56151_d_n9;
        locals.var_sqs_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_22(
        locals: &mut StampLocals,
    ) {
        let (assign42860_e56175, assign42860_e56175_d_n4, assign42860_e56175_d_n6, assign42860_e56175_d_n7, assign42860_e56175_d_n8, assign42860_e56175_d_n9,) = {
    if ((locals.var_guard1205 != 0.0) && (locals.var_guard1208 != 0.0)) {
        let assign42860_e56161: f64 = (0.5 * locals.var_x_s);
        let assign42860_e56162: f64 = (1.0 - assign42860_e56161);
        let assign42860_e56166: f64 = (locals.var_x_s * locals.var_x_s);
        let assign42860_e56167: f64 = (0.16666666666666666 * assign42860_e56166);
        let assign42860_e56168: f64 = (assign42860_e56162 + assign42860_e56167);
        let assign42860_e56169: f64 = (locals.var_gf * assign42860_e56168);
        let assign42860_e56171: f64 = (assign42860_e56169 / locals.var_temp__blk949);
        let assign42860_e56172: f64 = (0.7071067811865475 * assign42860_e56171);
        let assign42860_e56173: f64 = (1.0 + assign42860_e56172);
        (assign42860_e56173, (0.7071067811865475 * (((((locals.var_gf_dn4 * assign42860_e56168) + (locals.var_gf * ((-(0.5 * locals.var_x_s_dn4)) + (0.16666666666666666 * ((locals.var_x_s_dn4 * locals.var_x_s) + (locals.var_x_s * locals.var_x_s_dn4)))))) * locals.var_temp__blk949) - (assign42860_e56169 * locals.var_temp__blk949_dn4)) / (locals.var_temp__blk949 * locals.var_temp__blk949))), (0.7071067811865475 * (((((locals.var_gf_dn6 * assign42860_e56168) + (locals.var_gf * ((-(0.5 * locals.var_x_s_dn6)) + (0.16666666666666666 * ((locals.var_x_s_dn6 * locals.var_x_s) + (locals.var_x_s * locals.var_x_s_dn6)))))) * locals.var_temp__blk949) - (assign42860_e56169 * locals.var_temp__blk949_dn6)) / (locals.var_temp__blk949 * locals.var_temp__blk949))), (0.7071067811865475 * (((((locals.var_gf_dn7 * assign42860_e56168) + (locals.var_gf * ((-(0.5 * locals.var_x_s_dn7)) + (0.16666666666666666 * ((locals.var_x_s_dn7 * locals.var_x_s) + (locals.var_x_s * locals.var_x_s_dn7)))))) * locals.var_temp__blk949) - (assign42860_e56169 * locals.var_temp__blk949_dn7)) / (locals.var_temp__blk949 * locals.var_temp__blk949))), (0.7071067811865475 * (((((locals.var_gf_dn8 * assign42860_e56168) + (locals.var_gf * ((-(0.5 * locals.var_x_s_dn8)) + (0.16666666666666666 * ((locals.var_x_s_dn8 * locals.var_x_s) + (locals.var_x_s * locals.var_x_s_dn8)))))) * locals.var_temp__blk949) - (assign42860_e56169 * locals.var_temp__blk949_dn8)) / (locals.var_temp__blk949 * locals.var_temp__blk949))), (0.7071067811865475 * (((((locals.var_gf_dn9 * assign42860_e56168) + (locals.var_gf * ((-(0.5 * locals.var_x_s_dn9)) + (0.16666666666666666 * ((locals.var_x_s_dn9 * locals.var_x_s) + (locals.var_x_s * locals.var_x_s_dn9)))))) * locals.var_temp__blk949) - (assign42860_e56169 * locals.var_temp__blk949_dn9)) / (locals.var_temp__blk949 * locals.var_temp__blk949))),)
    } else {
        (locals.var_alphas, locals.var_alphas_dn4, locals.var_alphas_dn6, locals.var_alphas_dn7, locals.var_alphas_dn8, locals.var_alphas_dn9,)
    }
};
        locals.var_alphas = assign42860_e56175;
        locals.var_alphas_dn4 = assign42860_e56175_d_n4;
        locals.var_alphas_dn6 = assign42860_e56175_d_n6;
        locals.var_alphas_dn7 = assign42860_e56175_d_n7;
        locals.var_alphas_dn8 = assign42860_e56175_d_n8;
        locals.var_alphas_dn9 = assign42860_e56175_d_n9;
        locals.var_alphas_rv = 0.0;

        let (assign42870_e56186, assign42870_e56186_d_n4, assign42870_e56186_d_n6, assign42870_e56186_d_n7, assign42870_e56186_d_n8, assign42870_e56186_d_n9,) = {
    if ((locals.var_guard1205 != 0.0) && (locals.var_guard1208 == 0.0)) {
        let assign42870_e56182: f64 = (locals.var_x_s - 1.0);
        let assign42870_e56184: f64 = (assign42870_e56182 + locals.var_es);
        (assign42870_e56184, (locals.var_x_s_dn4 + locals.var_es_dn4), (locals.var_x_s_dn6 + locals.var_es_dn6), (locals.var_x_s_dn7 + locals.var_es_dn7), (locals.var_x_s_dn8 + locals.var_es_dn8), (locals.var_x_s_dn9 + locals.var_es_dn9),)
    } else {
        (locals.var_ps, locals.var_ps_dn4, locals.var_ps_dn6, locals.var_ps_dn7, locals.var_ps_dn8, locals.var_ps_dn9,)
    }
};
        locals.var_ps = assign42870_e56186;
        locals.var_ps_dn4 = assign42870_e56186_d_n4;
        locals.var_ps_dn6 = assign42870_e56186_d_n6;
        locals.var_ps_dn7 = assign42870_e56186_d_n7;
        locals.var_ps_dn8 = assign42870_e56186_d_n8;
        locals.var_ps_dn9 = assign42870_e56186_d_n9;
        locals.var_ps_rv = 0.0;

        let (assign42880_e56194, assign42880_e56194_d_n4, assign42880_e56194_d_n6, assign42880_e56194_d_n7, assign42880_e56194_d_n8, assign42880_e56194_d_n9,) = {
    if ((locals.var_guard1205 != 0.0) && (locals.var_guard1208 == 0.0)) {
        let assign42880_e56192: f64 = (locals.var_ps).sqrt();
        (assign42880_e56192, (locals.var_ps_dn4 / (2.0 * assign42880_e56192)), (locals.var_ps_dn6 / (2.0 * assign42880_e56192)), (locals.var_ps_dn7 / (2.0 * assign42880_e56192)), (locals.var_ps_dn8 / (2.0 * assign42880_e56192)), (locals.var_ps_dn9 / (2.0 * assign42880_e56192)),)
    } else {
        (locals.var_sqs, locals.var_sqs_dn4, locals.var_sqs_dn6, locals.var_sqs_dn7, locals.var_sqs_dn8, locals.var_sqs_dn9,)
    }
};
        locals.var_sqs = assign42880_e56194;
        locals.var_sqs_dn4 = assign42880_e56194_d_n4;
        locals.var_sqs_dn6 = assign42880_e56194_d_n6;
        locals.var_sqs_dn7 = assign42880_e56194_d_n7;
        locals.var_sqs_dn8 = assign42880_e56194_d_n8;
        locals.var_sqs_dn9 = assign42880_e56194_d_n9;
        locals.var_sqs_rv = 0.0;

        let (assign42890_e56211, assign42890_e56211_d_n4, assign42890_e56211_d_n6, assign42890_e56211_d_n7, assign42890_e56211_d_n8, assign42890_e56211_d_n9,) = {
    if ((locals.var_guard1205 != 0.0) && (locals.var_guard1208 == 0.0)) {
        let assign42890_e56204: f64 = (1.0 - locals.var_es);
        let assign42890_e56205: f64 = (locals.var_gf * assign42890_e56204);
        let assign42890_e56207: f64 = (assign42890_e56205 / locals.var_sqs);
        let assign42890_e56208: f64 = (0.5 * assign42890_e56207);
        let assign42890_e56209: f64 = (1.0 + assign42890_e56208);
        (assign42890_e56209, (0.5 * (((((locals.var_gf_dn4 * assign42890_e56204) + (locals.var_gf * (-locals.var_es_dn4))) * locals.var_sqs) - (assign42890_e56205 * locals.var_sqs_dn4)) / (locals.var_sqs * locals.var_sqs))), (0.5 * (((((locals.var_gf_dn6 * assign42890_e56204) + (locals.var_gf * (-locals.var_es_dn6))) * locals.var_sqs) - (assign42890_e56205 * locals.var_sqs_dn6)) / (locals.var_sqs * locals.var_sqs))), (0.5 * (((((locals.var_gf_dn7 * assign42890_e56204) + (locals.var_gf * (-locals.var_es_dn7))) * locals.var_sqs) - (assign42890_e56205 * locals.var_sqs_dn7)) / (locals.var_sqs * locals.var_sqs))), (0.5 * (((((locals.var_gf_dn8 * assign42890_e56204) + (locals.var_gf * (-locals.var_es_dn8))) * locals.var_sqs) - (assign42890_e56205 * locals.var_sqs_dn8)) / (locals.var_sqs * locals.var_sqs))), (0.5 * (((((locals.var_gf_dn9 * assign42890_e56204) + (locals.var_gf * (-locals.var_es_dn9))) * locals.var_sqs) - (assign42890_e56205 * locals.var_sqs_dn9)) / (locals.var_sqs * locals.var_sqs))),)
    } else {
        (locals.var_alphas, locals.var_alphas_dn4, locals.var_alphas_dn6, locals.var_alphas_dn7, locals.var_alphas_dn8, locals.var_alphas_dn9,)
    }
};
        locals.var_alphas = assign42890_e56211;
        locals.var_alphas_dn4 = assign42890_e56211_d_n4;
        locals.var_alphas_dn6 = assign42890_e56211_d_n6;
        locals.var_alphas_dn7 = assign42890_e56211_d_n7;
        locals.var_alphas_dn8 = assign42890_e56211_d_n8;
        locals.var_alphas_dn9 = assign42890_e56211_d_n9;
        locals.var_alphas_rv = 0.0;

        let (assign42900_e56227, assign42900_e56227_d_n4, assign42900_e56227_d_n6, assign42900_e56227_d_n7, assign42900_e56227_d_n8, assign42900_e56227_d_n9,) = {
    if (locals.var_guard1205 != 0.0) {
        let assign42900_e56216: f64 = (0.2 * locals.var_xcor_t);
        let assign42900_e56218: f64 = (assign42900_e56216 * locals.var_vsbx);
        let assign42900_e56219: f64 = (1.0 + assign42900_e56218);
        let assign42900_e56223: f64 = (locals.var_xcor_t * locals.var_vsbx);
        let assign42900_e56224: f64 = (1.0 + assign42900_e56223);
        let assign42900_e56225: f64 = (assign42900_e56219 / assign42900_e56224);
        (assign42900_e56225, ((((((0.2 * locals.var_xcor_t_dn4) * locals.var_vsbx) + (assign42900_e56216 * locals.var_vsbx_dn4)) * assign42900_e56224) - (assign42900_e56219 * ((locals.var_xcor_t_dn4 * locals.var_vsbx) + (locals.var_xcor_t * locals.var_vsbx_dn4)))) / (assign42900_e56224 * assign42900_e56224)), ((((assign42900_e56216 * locals.var_vsbx_dn6) * assign42900_e56224) - (assign42900_e56219 * (locals.var_xcor_t * locals.var_vsbx_dn6))) / (assign42900_e56224 * assign42900_e56224)), ((((assign42900_e56216 * locals.var_vsbx_dn7) * assign42900_e56224) - (assign42900_e56219 * (locals.var_xcor_t * locals.var_vsbx_dn7))) / (assign42900_e56224 * assign42900_e56224)), ((((assign42900_e56216 * locals.var_vsbx_dn8) * assign42900_e56224) - (assign42900_e56219 * (locals.var_xcor_t * locals.var_vsbx_dn8))) / (assign42900_e56224 * assign42900_e56224)), ((((assign42900_e56216 * locals.var_vsbx_dn9) * assign42900_e56224) - (assign42900_e56219 * (locals.var_xcor_t * locals.var_vsbx_dn9))) / (assign42900_e56224 * assign42900_e56224)),)
    } else {
        (locals.var_rxcor, locals.var_rxcor_dn4, locals.var_rxcor_dn6, locals.var_rxcor_dn7, locals.var_rxcor_dn8, locals.var_rxcor_dn9,)
    }
};
        locals.var_rxcor = assign42900_e56227;
        locals.var_rxcor_dn4 = assign42900_e56227_d_n4;
        locals.var_rxcor_dn6 = assign42900_e56227_d_n6;
        locals.var_rxcor_dn7 = assign42900_e56227_d_n7;
        locals.var_rxcor_dn8 = assign42900_e56227_d_n8;
        locals.var_rxcor_dn9 = assign42900_e56227_d_n9;
        locals.var_rxcor_rv = 0.0;

        let assign42910_e56230: f64 = if locals.var_ds > 1e-100 { 1.0 } else { 0.0 };
        locals.var_guard1209 = assign42910_e56230;
        locals.var_guard1209_rv = 0.0;

        let (assign42920_e56241, assign42920_e56241_d_n4, assign42920_e56241_d_n6, assign42920_e56241_d_n7, assign42920_e56241_d_n8, assign42920_e56241_d_n9,) = {
    if ((locals.var_guard1205 != 0.0) && (locals.var_guard1209 != 0.0)) {
        let assign42920_e56237: f64 = (locals.var_ps + locals.var_ds);
        let assign42920_e56238: f64 = (assign42920_e56237).sqrt();
        let assign42920_e56239: f64 = (locals.var_gf * assign42920_e56238);
        (assign42920_e56239, ((locals.var_gf_dn4 * assign42920_e56238) + (locals.var_gf * ((locals.var_ps_dn4 + locals.var_ds_dn4) / (2.0 * assign42920_e56238)))), ((locals.var_gf_dn6 * assign42920_e56238) + (locals.var_gf * ((locals.var_ps_dn6 + locals.var_ds_dn6) / (2.0 * assign42920_e56238)))), ((locals.var_gf_dn7 * assign42920_e56238) + (locals.var_gf * ((locals.var_ps_dn7 + locals.var_ds_dn7) / (2.0 * assign42920_e56238)))), ((locals.var_gf_dn8 * assign42920_e56238) + (locals.var_gf * ((locals.var_ps_dn8 + locals.var_ds_dn8) / (2.0 * assign42920_e56238)))), ((locals.var_gf_dn9 * assign42920_e56238) + (locals.var_gf * ((locals.var_ps_dn9 + locals.var_ds_dn9) / (2.0 * assign42920_e56238)))),)
    } else {
        (locals.var_xgs, locals.var_xgs_dn4, locals.var_xgs_dn6, locals.var_xgs_dn7, locals.var_xgs_dn8, locals.var_xgs_dn9,)
    }
};
        locals.var_xgs = assign42920_e56241;
        locals.var_xgs_dn4 = assign42920_e56241_d_n4;
        locals.var_xgs_dn6 = assign42920_e56241_d_n6;
        locals.var_xgs_dn7 = assign42920_e56241_d_n7;
        locals.var_xgs_dn8 = assign42920_e56241_d_n8;
        locals.var_xgs_dn9 = assign42920_e56241_d_n9;
        locals.var_xgs_rv = 0.0;

        let (assign42930_e56257, assign42930_e56257_d_n4, assign42930_e56257_d_n6, assign42930_e56257_d_n7, assign42930_e56257_d_n8, assign42930_e56257_d_n9,) = {
    if ((locals.var_guard1205 != 0.0) && (locals.var_guard1209 != 0.0)) {
        let assign42930_e56247: f64 = (locals.var_gf2 * locals.var_ds);
        let assign42930_e56249: f64 = (assign42930_e56247 * locals.var_phit1);
        let assign42930_e56253: f64 = (locals.var_gf * locals.var_sqs);
        let assign42930_e56254: f64 = (locals.var_xgs + assign42930_e56253);
        let assign42930_e56255: f64 = (assign42930_e56249 / assign42930_e56254);
        (assign42930_e56255, (((((((locals.var_gf2_dn4 * locals.var_ds) + (locals.var_gf2 * locals.var_ds_dn4)) * locals.var_phit1) + (assign42930_e56247 * locals.var_phit1_dn4)) * assign42930_e56254) - (assign42930_e56249 * (locals.var_xgs_dn4 + ((locals.var_gf_dn4 * locals.var_sqs) + (locals.var_gf * locals.var_sqs_dn4))))) / (assign42930_e56254 * assign42930_e56254)), (((((((locals.var_gf2_dn6 * locals.var_ds) + (locals.var_gf2 * locals.var_ds_dn6)) * locals.var_phit1) + (assign42930_e56247 * locals.var_phit1_dn6)) * assign42930_e56254) - (assign42930_e56249 * (locals.var_xgs_dn6 + ((locals.var_gf_dn6 * locals.var_sqs) + (locals.var_gf * locals.var_sqs_dn6))))) / (assign42930_e56254 * assign42930_e56254)), (((((((locals.var_gf2_dn7 * locals.var_ds) + (locals.var_gf2 * locals.var_ds_dn7)) * locals.var_phit1) + (assign42930_e56247 * locals.var_phit1_dn7)) * assign42930_e56254) - (assign42930_e56249 * (locals.var_xgs_dn7 + ((locals.var_gf_dn7 * locals.var_sqs) + (locals.var_gf * locals.var_sqs_dn7))))) / (assign42930_e56254 * assign42930_e56254)), (((((((locals.var_gf2_dn8 * locals.var_ds) + (locals.var_gf2 * locals.var_ds_dn8)) * locals.var_phit1) + (assign42930_e56247 * locals.var_phit1_dn8)) * assign42930_e56254) - (assign42930_e56249 * (locals.var_xgs_dn8 + ((locals.var_gf_dn8 * locals.var_sqs) + (locals.var_gf * locals.var_sqs_dn8))))) / (assign42930_e56254 * assign42930_e56254)), (((((((locals.var_gf2_dn9 * locals.var_ds) + (locals.var_gf2 * locals.var_ds_dn9)) * locals.var_phit1) + (assign42930_e56247 * locals.var_phit1_dn9)) * assign42930_e56254) - (assign42930_e56249 * (locals.var_xgs_dn9 + ((locals.var_gf_dn9 * locals.var_sqs) + (locals.var_gf * locals.var_sqs_dn9))))) / (assign42930_e56254 * assign42930_e56254)),)
    } else {
        (locals.var_qis, locals.var_qis_dn4, locals.var_qis_dn6, locals.var_qis_dn7, locals.var_qis_dn8, locals.var_qis_dn9,)
    }
};
        locals.var_qis = assign42930_e56257;
        locals.var_qis_dn4 = assign42930_e56257_d_n4;
        locals.var_qis_dn6 = assign42930_e56257_d_n6;
        locals.var_qis_dn7 = assign42930_e56257_d_n7;
        locals.var_qis_dn8 = assign42930_e56257_d_n8;
        locals.var_qis_dn9 = assign42930_e56257_d_n9;
        locals.var_qis_rv = 0.0;

        let (assign42940_e56267, assign42940_e56267_d_n4, assign42940_e56267_d_n6, assign42940_e56267_d_n7, assign42940_e56267_d_n8, assign42940_e56267_d_n9,) = {
    if ((locals.var_guard1205 != 0.0) && (locals.var_guard1209 != 0.0)) {
        let assign42940_e56263: f64 = (locals.var_sqs * locals.var_gf);
        let assign42940_e56265: f64 = (assign42940_e56263 * locals.var_phit1);
        (assign42940_e56265, ((((locals.var_sqs_dn4 * locals.var_gf) + (locals.var_sqs * locals.var_gf_dn4)) * locals.var_phit1) + (assign42940_e56263 * locals.var_phit1_dn4)), ((((locals.var_sqs_dn6 * locals.var_gf) + (locals.var_sqs * locals.var_gf_dn6)) * locals.var_phit1) + (assign42940_e56263 * locals.var_phit1_dn6)), ((((locals.var_sqs_dn7 * locals.var_gf) + (locals.var_sqs * locals.var_gf_dn7)) * locals.var_phit1) + (assign42940_e56263 * locals.var_phit1_dn7)), ((((locals.var_sqs_dn8 * locals.var_gf) + (locals.var_sqs * locals.var_gf_dn8)) * locals.var_phit1) + (assign42940_e56263 * locals.var_phit1_dn8)), ((((locals.var_sqs_dn9 * locals.var_gf) + (locals.var_sqs * locals.var_gf_dn9)) * locals.var_phit1) + (assign42940_e56263 * locals.var_phit1_dn9)),)
    } else {
        (locals.var_qbs, locals.var_qbs_dn4, locals.var_qbs_dn6, locals.var_qbs_dn7, locals.var_qbs_dn8, locals.var_qbs_dn9,)
    }
};
        locals.var_qbs = assign42940_e56267;
        locals.var_qbs_dn4 = assign42940_e56267_d_n4;
        locals.var_qbs_dn6 = assign42940_e56267_d_n6;
        locals.var_qbs_dn7 = assign42940_e56267_d_n7;
        locals.var_qbs_dn8 = assign42940_e56267_d_n8;
        locals.var_qbs_dn9 = assign42940_e56267_d_n9;
        locals.var_qbs_rv = 0.0;

        let assign42950_e56270: f64 = if locals.var_rsb_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1210 = assign42950_e56270;
        locals.var_guard1210_rv = 0.0;

        let (assign42960_e56284, assign42960_e56284_d_n4, assign42960_e56284_d_n6, assign42960_e56284_d_n7, assign42960_e56284_d_n8, assign42960_e56284_d_n9,) = {
    if (((locals.var_guard1205 != 0.0) && (locals.var_guard1209 != 0.0)) && (locals.var_guard1210 != 0.0)) {
        let assign42960_e56280: f64 = (locals.var_rsb_i * locals.var_vsbx);
        let assign42960_e56281: f64 = (1.0 - assign42960_e56280);
        let assign42960_e56282: f64 = (1.0 / assign42960_e56281);
        (assign42960_e56282, (-((-(locals.var_rsb_i * locals.var_vsbx_dn4)) / (assign42960_e56281 * assign42960_e56281))), (-((-(locals.var_rsb_i * locals.var_vsbx_dn6)) / (assign42960_e56281 * assign42960_e56281))), (-((-(locals.var_rsb_i * locals.var_vsbx_dn7)) / (assign42960_e56281 * assign42960_e56281))), (-((-(locals.var_rsb_i * locals.var_vsbx_dn8)) / (assign42960_e56281 * assign42960_e56281))), (-((-(locals.var_rsb_i * locals.var_vsbx_dn9)) / (assign42960_e56281 * assign42960_e56281))),)
    } else {
        (locals.var_rhob, locals.var_rhob_dn4, locals.var_rhob_dn6, locals.var_rhob_dn7, locals.var_rhob_dn8, locals.var_rhob_dn9,)
    }
};
        locals.var_rhob = assign42960_e56284;
        locals.var_rhob_dn4 = assign42960_e56284_d_n4;
        locals.var_rhob_dn6 = assign42960_e56284_d_n6;
        locals.var_rhob_dn7 = assign42960_e56284_d_n7;
        locals.var_rhob_dn8 = assign42960_e56284_d_n8;
        locals.var_rhob_dn9 = assign42960_e56284_d_n9;
        locals.var_rhob_rv = 0.0;

        let (assign42970_e56297, assign42970_e56297_d_n4, assign42970_e56297_d_n6, assign42970_e56297_d_n7, assign42970_e56297_d_n8, assign42970_e56297_d_n9,) = {
    if (((locals.var_guard1205 != 0.0) && (locals.var_guard1209 != 0.0)) && (locals.var_guard1210 == 0.0)) {
        let assign42970_e56294: f64 = (locals.var_rsb_i * locals.var_vsbx);
        let assign42970_e56295: f64 = (1.0 + assign42970_e56294);
        (assign42970_e56295, (locals.var_rsb_i * locals.var_vsbx_dn4), (locals.var_rsb_i * locals.var_vsbx_dn6), (locals.var_rsb_i * locals.var_vsbx_dn7), (locals.var_rsb_i * locals.var_vsbx_dn8), (locals.var_rsb_i * locals.var_vsbx_dn9),)
    } else {
        (locals.var_rhob, locals.var_rhob_dn4, locals.var_rhob_dn6, locals.var_rhob_dn7, locals.var_rhob_dn8, locals.var_rhob_dn9,)
    }
};
        locals.var_rhob = assign42970_e56297;
        locals.var_rhob_dn4 = assign42970_e56297_d_n4;
        locals.var_rhob_dn6 = assign42970_e56297_d_n6;
        locals.var_rhob_dn7 = assign42970_e56297_d_n7;
        locals.var_rhob_dn8 = assign42970_e56297_d_n8;
        locals.var_rhob_dn9 = assign42970_e56297_d_n9;
        locals.var_rhob_rv = 0.0;

        let assign42980_e56300: f64 = if locals.var_rsg_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1211 = assign42980_e56300;
        locals.var_guard1211_rv = 0.0;

        let (assign42990_e56312, assign42990_e56312_d_n4, assign42990_e56312_d_n6, assign42990_e56312_d_n7, assign42990_e56312_d_n8, assign42990_e56312_d_n9,) = {
    if (((locals.var_guard1205 != 0.0) && (locals.var_guard1209 != 0.0)) && (locals.var_guard1211 != 0.0)) {
        let assign42990_e56309: f64 = (locals.var_rsg_i * locals.var_qis);
        let assign42990_e56310: f64 = (1.0 - assign42990_e56309);
        (assign42990_e56310, (-(locals.var_rsg_i * locals.var_qis_dn4)), (-(locals.var_rsg_i * locals.var_qis_dn6)), (-(locals.var_rsg_i * locals.var_qis_dn7)), (-(locals.var_rsg_i * locals.var_qis_dn8)), (-(locals.var_rsg_i * locals.var_qis_dn9)),)
    } else {
        (locals.var_rhog, locals.var_rhog_dn4, locals.var_rhog_dn6, locals.var_rhog_dn7, locals.var_rhog_dn8, locals.var_rhog_dn9,)
    }
};
        locals.var_rhog = assign42990_e56312;
        locals.var_rhog_dn4 = assign42990_e56312_d_n4;
        locals.var_rhog_dn6 = assign42990_e56312_d_n6;
        locals.var_rhog_dn7 = assign42990_e56312_d_n7;
        locals.var_rhog_dn8 = assign42990_e56312_d_n8;
        locals.var_rhog_dn9 = assign42990_e56312_d_n9;
        locals.var_rhog_rv = 0.0;

        let (assign43000_e56327, assign43000_e56327_d_n4, assign43000_e56327_d_n6, assign43000_e56327_d_n7, assign43000_e56327_d_n8, assign43000_e56327_d_n9,) = {
    if (((locals.var_guard1205 != 0.0) && (locals.var_guard1209 != 0.0)) && (locals.var_guard1211 == 0.0)) {
        let assign43000_e56323: f64 = (locals.var_rsg_i * locals.var_qis);
        let assign43000_e56324: f64 = (1.0 + assign43000_e56323);
        let assign43000_e56325: f64 = (1.0 / assign43000_e56324);
        (assign43000_e56325, (-((locals.var_rsg_i * locals.var_qis_dn4) / (assign43000_e56324 * assign43000_e56324))), (-((locals.var_rsg_i * locals.var_qis_dn6) / (assign43000_e56324 * assign43000_e56324))), (-((locals.var_rsg_i * locals.var_qis_dn7) / (assign43000_e56324 * assign43000_e56324))), (-((locals.var_rsg_i * locals.var_qis_dn8) / (assign43000_e56324 * assign43000_e56324))), (-((locals.var_rsg_i * locals.var_qis_dn9) / (assign43000_e56324 * assign43000_e56324))),)
    } else {
        (locals.var_rhog, locals.var_rhog_dn4, locals.var_rhog_dn6, locals.var_rhog_dn7, locals.var_rhog_dn8, locals.var_rhog_dn9,)
    }
};
        locals.var_rhog = assign43000_e56327;
        locals.var_rhog_dn4 = assign43000_e56327_d_n4;
        locals.var_rhog_dn6 = assign43000_e56327_d_n6;
        locals.var_rhog_dn7 = assign43000_e56327_d_n7;
        locals.var_rhog_dn8 = assign43000_e56327_d_n8;
        locals.var_rhog_dn9 = assign43000_e56327_d_n9;
        locals.var_rhog_rv = 0.0;

        let (assign43010_e56339, assign43010_e56339_d_n4, assign43010_e56339_d_n6, assign43010_e56339_d_n7, assign43010_e56339_d_n8, assign43010_e56339_d_n9,) = {
    if ((locals.var_guard1205 != 0.0) && (locals.var_guard1209 != 0.0)) {
        let assign43010_e56333: f64 = (locals.var_ther_i * locals.var_rhob);
        let assign43010_e56335: f64 = (assign43010_e56333 * locals.var_rhog);
        let assign43010_e56337: f64 = (assign43010_e56335 * locals.var_qis);
        (assign43010_e56337, ((((((locals.var_ther_i_dn4 * locals.var_rhob) + (locals.var_ther_i * locals.var_rhob_dn4)) * locals.var_rhog) + (assign43010_e56333 * locals.var_rhog_dn4)) * locals.var_qis) + (assign43010_e56335 * locals.var_qis_dn4)), (((((locals.var_ther_i * locals.var_rhob_dn6) * locals.var_rhog) + (assign43010_e56333 * locals.var_rhog_dn6)) * locals.var_qis) + (assign43010_e56335 * locals.var_qis_dn6)), (((((locals.var_ther_i * locals.var_rhob_dn7) * locals.var_rhog) + (assign43010_e56333 * locals.var_rhog_dn7)) * locals.var_qis) + (assign43010_e56335 * locals.var_qis_dn7)), (((((locals.var_ther_i * locals.var_rhob_dn8) * locals.var_rhog) + (assign43010_e56333 * locals.var_rhog_dn8)) * locals.var_qis) + (assign43010_e56335 * locals.var_qis_dn8)), (((((locals.var_ther_i * locals.var_rhob_dn9) * locals.var_rhog) + (assign43010_e56333 * locals.var_rhog_dn9)) * locals.var_qis) + (assign43010_e56335 * locals.var_qis_dn9)),)
    } else {
        (locals.var_gr, locals.var_gr_dn4, locals.var_gr_dn6, locals.var_gr_dn7, locals.var_gr_dn8, locals.var_gr_dn9,)
    }
};
        locals.var_gr = assign43010_e56339;
        locals.var_gr_dn4 = assign43010_e56339_d_n4;
        locals.var_gr_dn6 = assign43010_e56339_d_n6;
        locals.var_gr_dn7 = assign43010_e56339_d_n7;
        locals.var_gr_dn8 = assign43010_e56339_d_n8;
        locals.var_gr_dn9 = assign43010_e56339_d_n9;
        locals.var_gr_rv = 0.0;

        let (assign43020_e56351, assign43020_e56351_d_n4, assign43020_e56351_d_n6, assign43020_e56351_d_n7, assign43020_e56351_d_n8, assign43020_e56351_d_n9,) = {
    if ((locals.var_guard1205 != 0.0) && (locals.var_guard1209 != 0.0)) {
        let assign43020_e56347: f64 = (locals.var_eta_mu * locals.var_qis);
        let assign43020_e56348: f64 = (locals.var_qbs + assign43020_e56347);
        let assign43020_e56349: f64 = (locals.var_e_eff0 * assign43020_e56348);
        (assign43020_e56349, (locals.var_e_eff0 * (locals.var_qbs_dn4 + (locals.var_eta_mu * locals.var_qis_dn4))), (locals.var_e_eff0 * (locals.var_qbs_dn6 + (locals.var_eta_mu * locals.var_qis_dn6))), (locals.var_e_eff0 * (locals.var_qbs_dn7 + (locals.var_eta_mu * locals.var_qis_dn7))), (locals.var_e_eff0 * (locals.var_qbs_dn8 + (locals.var_eta_mu * locals.var_qis_dn8))), (locals.var_e_eff0 * (locals.var_qbs_dn9 + (locals.var_eta_mu * locals.var_qis_dn9))),)
    } else {
        (locals.var_eeffs, locals.var_eeffs_dn4, locals.var_eeffs_dn6, locals.var_eeffs_dn7, locals.var_eeffs_dn8, locals.var_eeffs_dn9,)
    }
};
        locals.var_eeffs = assign43020_e56351;
        locals.var_eeffs_dn4 = assign43020_e56351_d_n4;
        locals.var_eeffs_dn6 = assign43020_e56351_d_n6;
        locals.var_eeffs_dn7 = assign43020_e56351_d_n7;
        locals.var_eeffs_dn8 = assign43020_e56351_d_n8;
        locals.var_eeffs_dn9 = assign43020_e56351_d_n9;
        locals.var_eeffs_rv = 0.0;

        let (assign43030_e56364, assign43030_e56364_d_n4, assign43030_e56364_d_n6, assign43030_e56364_d_n7, assign43030_e56364_d_n8, assign43030_e56364_d_n9,) = {
    if ((locals.var_guard1205 != 0.0) && (locals.var_guard1209 != 0.0)) {
        let assign43030_e56358: f64 = (locals.var_ps + locals.var_ds);
        let assign43030_e56360: f64 = (assign43030_e56358 + 1e-14);
        let assign43030_e56361: f64 = (locals.var_ps / assign43030_e56360);
        let assign43030_e56362: f64 = (assign43030_e56361).ln();
        (assign43030_e56362, ((((locals.var_ps_dn4 * assign43030_e56360) - (locals.var_ps * (locals.var_ps_dn4 + locals.var_ds_dn4))) / (assign43030_e56360 * assign43030_e56360)) / assign43030_e56361), ((((locals.var_ps_dn6 * assign43030_e56360) - (locals.var_ps * (locals.var_ps_dn6 + locals.var_ds_dn6))) / (assign43030_e56360 * assign43030_e56360)) / assign43030_e56361), ((((locals.var_ps_dn7 * assign43030_e56360) - (locals.var_ps * (locals.var_ps_dn7 + locals.var_ds_dn7))) / (assign43030_e56360 * assign43030_e56360)) / assign43030_e56361), ((((locals.var_ps_dn8 * assign43030_e56360) - (locals.var_ps * (locals.var_ps_dn8 + locals.var_ds_dn8))) / (assign43030_e56360 * assign43030_e56360)) / assign43030_e56361), ((((locals.var_ps_dn9 * assign43030_e56360) - (locals.var_ps * (locals.var_ps_dn9 + locals.var_ds_dn9))) / (assign43030_e56360 * assign43030_e56360)) / assign43030_e56361),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign43030_e56364;
        locals.var_temp1_dn4 = assign43030_e56364_d_n4;
        locals.var_temp1_dn6 = assign43030_e56364_d_n6;
        locals.var_temp1_dn7 = assign43030_e56364_d_n7;
        locals.var_temp1_dn8 = assign43030_e56364_d_n8;
        locals.var_temp1_dn9 = assign43030_e56364_d_n9;
        locals.var_temp1_rv = 0.0;

        let (assign43040_e56383, assign43040_e56383_d_n4, assign43040_e56383_d_n6, assign43040_e56383_d_n7, assign43040_e56383_d_n8, assign43040_e56383_d_n9,) = {
    if ((locals.var_guard1205 != 0.0) && (locals.var_guard1209 != 0.0)) {
        let assign43040_e56370: f64 = (locals.var_eeffs * locals.var_mue_t);
        let assign43040_e56372: f64 = (assign43040_e56370).powf(locals.var_themu_t);
        let assign43040_e56376: f64 = (0.5 * locals.var_thecs_t);
        let assign43040_e56378: f64 = (assign43040_e56376 * locals.var_temp1);
        let assign43040_e56379: f64 = (assign43040_e56378).exp();
        let assign43040_e56380: f64 = (locals.var_cs_t * assign43040_e56379);
        let assign43040_e56381: f64 = (assign43040_e56372 + assign43040_e56380);
        (assign43040_e56381, (if locals.var_themu_t_dn4 == 0.0 && ((locals.var_themu_t) as f64).is_finite() && ((locals.var_themu_t) as f64).fract() == 0.0 { if locals.var_themu_t == 0.0 { 0.0 } else { (locals.var_themu_t * ((assign43040_e56370).powf(locals.var_themu_t - 1.0) * ((locals.var_eeffs_dn4 * locals.var_mue_t) + (locals.var_eeffs * locals.var_mue_t_dn4)))) } } else { (assign43040_e56372 * ((locals.var_themu_t_dn4 * (assign43040_e56370).ln()) + (locals.var_themu_t * (((locals.var_eeffs_dn4 * locals.var_mue_t) + (locals.var_eeffs * locals.var_mue_t_dn4)) / assign43040_e56370)))) } + ((locals.var_cs_t_dn4 * assign43040_e56379) + (locals.var_cs_t * (assign43040_e56379 * (((0.5 * locals.var_thecs_t_dn4) * locals.var_temp1) + (assign43040_e56376 * locals.var_temp1_dn4)))))), (if 0.0 == 0.0 && ((locals.var_themu_t) as f64).is_finite() && ((locals.var_themu_t) as f64).fract() == 0.0 { if locals.var_themu_t == 0.0 { 0.0 } else { (locals.var_themu_t * ((assign43040_e56370).powf(locals.var_themu_t - 1.0) * (locals.var_eeffs_dn6 * locals.var_mue_t))) } } else { (assign43040_e56372 * (locals.var_themu_t * ((locals.var_eeffs_dn6 * locals.var_mue_t) / assign43040_e56370))) } + (locals.var_cs_t * (assign43040_e56379 * (assign43040_e56376 * locals.var_temp1_dn6)))), (if 0.0 == 0.0 && ((locals.var_themu_t) as f64).is_finite() && ((locals.var_themu_t) as f64).fract() == 0.0 { if locals.var_themu_t == 0.0 { 0.0 } else { (locals.var_themu_t * ((assign43040_e56370).powf(locals.var_themu_t - 1.0) * (locals.var_eeffs_dn7 * locals.var_mue_t))) } } else { (assign43040_e56372 * (locals.var_themu_t * ((locals.var_eeffs_dn7 * locals.var_mue_t) / assign43040_e56370))) } + (locals.var_cs_t * (assign43040_e56379 * (assign43040_e56376 * locals.var_temp1_dn7)))), (if 0.0 == 0.0 && ((locals.var_themu_t) as f64).is_finite() && ((locals.var_themu_t) as f64).fract() == 0.0 { if locals.var_themu_t == 0.0 { 0.0 } else { (locals.var_themu_t * ((assign43040_e56370).powf(locals.var_themu_t - 1.0) * (locals.var_eeffs_dn8 * locals.var_mue_t))) } } else { (assign43040_e56372 * (locals.var_themu_t * ((locals.var_eeffs_dn8 * locals.var_mue_t) / assign43040_e56370))) } + (locals.var_cs_t * (assign43040_e56379 * (assign43040_e56376 * locals.var_temp1_dn8)))), (if 0.0 == 0.0 && ((locals.var_themu_t) as f64).is_finite() && ((locals.var_themu_t) as f64).fract() == 0.0 { if locals.var_themu_t == 0.0 { 0.0 } else { (locals.var_themu_t * ((assign43040_e56370).powf(locals.var_themu_t - 1.0) * (locals.var_eeffs_dn9 * locals.var_mue_t))) } } else { (assign43040_e56372 * (locals.var_themu_t * ((locals.var_eeffs_dn9 * locals.var_mue_t) / assign43040_e56370))) } + (locals.var_cs_t * (assign43040_e56379 * (assign43040_e56376 * locals.var_temp1_dn9)))),)
    } else {
        (locals.var_mutmp, locals.var_mutmp_dn4, locals.var_mutmp_dn6, locals.var_mutmp_dn7, locals.var_mutmp_dn8, locals.var_mutmp_dn9,)
    }
};
        locals.var_mutmp = assign43040_e56383;
        locals.var_mutmp_dn4 = assign43040_e56383_d_n4;
        locals.var_mutmp_dn6 = assign43040_e56383_d_n6;
        locals.var_mutmp_dn7 = assign43040_e56383_d_n7;
        locals.var_mutmp_dn8 = assign43040_e56383_d_n8;
        locals.var_mutmp_dn9 = assign43040_e56383_d_n9;
        locals.var_mutmp_rv = 0.0;

        let (assign43050_e56395, assign43050_e56395_d_n4, assign43050_e56395_d_n6, assign43050_e56395_d_n7, assign43050_e56395_d_n8, assign43050_e56395_d_n9,) = {
    if ((locals.var_guard1205 != 0.0) && (locals.var_guard1209 != 0.0)) {
        let assign43050_e56389: f64 = (1.0 + locals.var_mutmp);
        let assign43050_e56391: f64 = (assign43050_e56389 + locals.var_gr);
        let assign43050_e56393: f64 = (assign43050_e56391 * locals.var_rxcor);
        (assign43050_e56393, (((locals.var_mutmp_dn4 + locals.var_gr_dn4) * locals.var_rxcor) + (assign43050_e56391 * locals.var_rxcor_dn4)), (((locals.var_mutmp_dn6 + locals.var_gr_dn6) * locals.var_rxcor) + (assign43050_e56391 * locals.var_rxcor_dn6)), (((locals.var_mutmp_dn7 + locals.var_gr_dn7) * locals.var_rxcor) + (assign43050_e56391 * locals.var_rxcor_dn7)), (((locals.var_mutmp_dn8 + locals.var_gr_dn8) * locals.var_rxcor) + (assign43050_e56391 * locals.var_rxcor_dn8)), (((locals.var_mutmp_dn9 + locals.var_gr_dn9) * locals.var_rxcor) + (assign43050_e56391 * locals.var_rxcor_dn9)),)
    } else {
        (locals.var_gmobs, locals.var_gmobs_dn4, locals.var_gmobs_dn6, locals.var_gmobs_dn7, locals.var_gmobs_dn8, locals.var_gmobs_dn9,)
    }
};
        locals.var_gmobs = assign43050_e56395;
        locals.var_gmobs_dn4 = assign43050_e56395_d_n4;
        locals.var_gmobs_dn6 = assign43050_e56395_d_n6;
        locals.var_gmobs_dn7 = assign43050_e56395_d_n7;
        locals.var_gmobs_dn8 = assign43050_e56395_d_n8;
        locals.var_gmobs_dn9 = assign43050_e56395_d_n9;
        locals.var_gmobs_rv = 0.0;

        let assign43060_e56398: f64 = if locals.var_thesatb_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1212 = assign43060_e56398;
        locals.var_guard1212_rv = 0.0;

        let (assign43070_e56412, assign43070_e56412_d_n4, assign43070_e56412_d_n6, assign43070_e56412_d_n7, assign43070_e56412_d_n8, assign43070_e56412_d_n9,) = {
    if (((locals.var_guard1205 != 0.0) && (locals.var_guard1209 != 0.0)) && (locals.var_guard1212 != 0.0)) {
        let assign43070_e56408: f64 = (locals.var_thesatb_i * locals.var_vsbx);
        let assign43070_e56409: f64 = (1.0 - assign43070_e56408);
        let assign43070_e56410: f64 = (1.0 / assign43070_e56409);
        (assign43070_e56410, (-((-(locals.var_thesatb_i * locals.var_vsbx_dn4)) / (assign43070_e56409 * assign43070_e56409))), (-((-(locals.var_thesatb_i * locals.var_vsbx_dn6)) / (assign43070_e56409 * assign43070_e56409))), (-((-(locals.var_thesatb_i * locals.var_vsbx_dn7)) / (assign43070_e56409 * assign43070_e56409))), (-((-(locals.var_thesatb_i * locals.var_vsbx_dn8)) / (assign43070_e56409 * assign43070_e56409))), (-((-(locals.var_thesatb_i * locals.var_vsbx_dn9)) / (assign43070_e56409 * assign43070_e56409))),)
    } else {
        (locals.var_xitsb, locals.var_xitsb_dn4, locals.var_xitsb_dn6, locals.var_xitsb_dn7, locals.var_xitsb_dn8, locals.var_xitsb_dn9,)
    }
};
        locals.var_xitsb = assign43070_e56412;
        locals.var_xitsb_dn4 = assign43070_e56412_d_n4;
        locals.var_xitsb_dn6 = assign43070_e56412_d_n6;
        locals.var_xitsb_dn7 = assign43070_e56412_d_n7;
        locals.var_xitsb_dn8 = assign43070_e56412_d_n8;
        locals.var_xitsb_dn9 = assign43070_e56412_d_n9;
        locals.var_xitsb_rv = 0.0;

        let (assign43080_e56425, assign43080_e56425_d_n4, assign43080_e56425_d_n6, assign43080_e56425_d_n7, assign43080_e56425_d_n8, assign43080_e56425_d_n9,) = {
    if (((locals.var_guard1205 != 0.0) && (locals.var_guard1209 != 0.0)) && (locals.var_guard1212 == 0.0)) {
        let assign43080_e56422: f64 = (locals.var_thesatb_i * locals.var_vsbx);
        let assign43080_e56423: f64 = (1.0 + assign43080_e56422);
        (assign43080_e56423, (locals.var_thesatb_i * locals.var_vsbx_dn4), (locals.var_thesatb_i * locals.var_vsbx_dn6), (locals.var_thesatb_i * locals.var_vsbx_dn7), (locals.var_thesatb_i * locals.var_vsbx_dn8), (locals.var_thesatb_i * locals.var_vsbx_dn9),)
    } else {
        (locals.var_xitsb, locals.var_xitsb_dn4, locals.var_xitsb_dn6, locals.var_xitsb_dn7, locals.var_xitsb_dn8, locals.var_xitsb_dn9,)
    }
};
        locals.var_xitsb = assign43080_e56425;
        locals.var_xitsb_dn4 = assign43080_e56425_d_n4;
        locals.var_xitsb_dn6 = assign43080_e56425_d_n6;
        locals.var_xitsb_dn7 = assign43080_e56425_d_n7;
        locals.var_xitsb_dn8 = assign43080_e56425_d_n8;
        locals.var_xitsb_dn9 = assign43080_e56425_d_n9;
        locals.var_xitsb_rv = 0.0;

        let (assign43090_e56433, assign43090_e56433_d_n4, assign43090_e56433_d_n6, assign43090_e56433_d_n7, assign43090_e56433_d_n8, assign43090_e56433_d_n9,) = {
    if ((locals.var_guard1205 != 0.0) && (locals.var_guard1209 != 0.0)) {
        let assign43090_e56431: f64 = (locals.var_qis * locals.var_xitsb);
        (assign43090_e56431, ((locals.var_qis_dn4 * locals.var_xitsb) + (locals.var_qis * locals.var_xitsb_dn4)), ((locals.var_qis_dn6 * locals.var_xitsb) + (locals.var_qis * locals.var_xitsb_dn6)), ((locals.var_qis_dn7 * locals.var_xitsb) + (locals.var_qis * locals.var_xitsb_dn7)), ((locals.var_qis_dn8 * locals.var_xitsb) + (locals.var_qis * locals.var_xitsb_dn8)), ((locals.var_qis_dn9 * locals.var_xitsb) + (locals.var_qis * locals.var_xitsb_dn9)),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9,)
    }
};
        locals.var_temp2 = assign43090_e56433;
        locals.var_temp2_dn4 = assign43090_e56433_d_n4;
        locals.var_temp2_dn6 = assign43090_e56433_d_n6;
        locals.var_temp2_dn7 = assign43090_e56433_d_n7;
        locals.var_temp2_dn8 = assign43090_e56433_d_n8;
        locals.var_temp2_dn9 = assign43090_e56433_d_n9;
        locals.var_temp2_rv = 0.0;

        let (assign43100_e56443, assign43100_e56443_d_n4, assign43100_e56443_d_n6, assign43100_e56443_d_n7, assign43100_e56443_d_n8, assign43100_e56443_d_n9,) = {
    if ((locals.var_guard1205 != 0.0) && (locals.var_guard1209 != 0.0)) {
        let assign43100_e56440: f64 = (locals.var_thesatt_i + locals.var_temp2);
        let assign43100_e56441: f64 = (locals.var_temp2 / assign43100_e56440);
        (assign43100_e56441, (((locals.var_temp2_dn4 * assign43100_e56440) - (locals.var_temp2 * locals.var_temp2_dn4)) / (assign43100_e56440 * assign43100_e56440)), (((locals.var_temp2_dn6 * assign43100_e56440) - (locals.var_temp2 * locals.var_temp2_dn6)) / (assign43100_e56440 * assign43100_e56440)), (((locals.var_temp2_dn7 * assign43100_e56440) - (locals.var_temp2 * locals.var_temp2_dn7)) / (assign43100_e56440 * assign43100_e56440)), (((locals.var_temp2_dn8 * assign43100_e56440) - (locals.var_temp2 * locals.var_temp2_dn8)) / (assign43100_e56440 * assign43100_e56440)), (((locals.var_temp2_dn9 * assign43100_e56440) - (locals.var_temp2 * locals.var_temp2_dn9)) / (assign43100_e56440 * assign43100_e56440)),)
    } else {
        (locals.var_wsat, locals.var_wsat_dn4, locals.var_wsat_dn6, locals.var_wsat_dn7, locals.var_wsat_dn8, locals.var_wsat_dn9,)
    }
};
        locals.var_wsat = assign43100_e56443;
        locals.var_wsat_dn4 = assign43100_e56443_d_n4;
        locals.var_wsat_dn6 = assign43100_e56443_d_n6;
        locals.var_wsat_dn7 = assign43100_e56443_d_n7;
        locals.var_wsat_dn8 = assign43100_e56443_d_n8;
        locals.var_wsat_dn9 = assign43100_e56443_d_n9;
        locals.var_wsat_rv = 0.0;

        let assign43110_e56446: f64 = if locals.var_thesatg_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1213 = assign43110_e56446;
        locals.var_guard1213_rv = 0.0;

        let (assign43120_e56460, assign43120_e56460_d_n4, assign43120_e56460_d_n6, assign43120_e56460_d_n7, assign43120_e56460_d_n8, assign43120_e56460_d_n9,) = {
    if (((locals.var_guard1205 != 0.0) && (locals.var_guard1209 != 0.0)) && (locals.var_guard1213 != 0.0)) {
        let assign43120_e56456: f64 = (locals.var_thesatg_i * locals.var_wsat);
        let assign43120_e56457: f64 = (1.0 - assign43120_e56456);
        let assign43120_e56458: f64 = (1.0 / assign43120_e56457);
        (assign43120_e56458, (-((-(locals.var_thesatg_i * locals.var_wsat_dn4)) / (assign43120_e56457 * assign43120_e56457))), (-((-(locals.var_thesatg_i * locals.var_wsat_dn6)) / (assign43120_e56457 * assign43120_e56457))), (-((-(locals.var_thesatg_i * locals.var_wsat_dn7)) / (assign43120_e56457 * assign43120_e56457))), (-((-(locals.var_thesatg_i * locals.var_wsat_dn8)) / (assign43120_e56457 * assign43120_e56457))), (-((-(locals.var_thesatg_i * locals.var_wsat_dn9)) / (assign43120_e56457 * assign43120_e56457))),)
    } else {
        (locals.var_factheta, locals.var_factheta_dn4, locals.var_factheta_dn6, locals.var_factheta_dn7, locals.var_factheta_dn8, locals.var_factheta_dn9,)
    }
};
        locals.var_factheta = assign43120_e56460;
        locals.var_factheta_dn4 = assign43120_e56460_d_n4;
        locals.var_factheta_dn6 = assign43120_e56460_d_n6;
        locals.var_factheta_dn7 = assign43120_e56460_d_n7;
        locals.var_factheta_dn8 = assign43120_e56460_d_n8;
        locals.var_factheta_dn9 = assign43120_e56460_d_n9;
        locals.var_factheta_rv = 0.0;

        let (assign43130_e56473, assign43130_e56473_d_n4, assign43130_e56473_d_n6, assign43130_e56473_d_n7, assign43130_e56473_d_n8, assign43130_e56473_d_n9,) = {
    if (((locals.var_guard1205 != 0.0) && (locals.var_guard1209 != 0.0)) && (locals.var_guard1213 == 0.0)) {
        let assign43130_e56470: f64 = (locals.var_thesatg_i * locals.var_wsat);
        let assign43130_e56471: f64 = (1.0 + assign43130_e56470);
        (assign43130_e56471, (locals.var_thesatg_i * locals.var_wsat_dn4), (locals.var_thesatg_i * locals.var_wsat_dn6), (locals.var_thesatg_i * locals.var_wsat_dn7), (locals.var_thesatg_i * locals.var_wsat_dn8), (locals.var_thesatg_i * locals.var_wsat_dn9),)
    } else {
        (locals.var_factheta, locals.var_factheta_dn4, locals.var_factheta_dn6, locals.var_factheta_dn7, locals.var_factheta_dn8, locals.var_factheta_dn9,)
    }
};
        locals.var_factheta = assign43130_e56473;
        locals.var_factheta_dn4 = assign43130_e56473_d_n4;
        locals.var_factheta_dn6 = assign43130_e56473_d_n6;
        locals.var_factheta_dn7 = assign43130_e56473_d_n7;
        locals.var_factheta_dn8 = assign43130_e56473_d_n8;
        locals.var_factheta_dn9 = assign43130_e56473_d_n9;
        locals.var_factheta_rv = 0.0;

        locals.var_vgb1_dc = locals.var_vgb1;
        locals.var_vgb1_dc_dn4 = locals.var_vgb1_dn4;
        locals.var_vgb1_dc_dn6 = locals.var_vgb1_dn6;
        locals.var_vgb1_dc_dn7 = locals.var_vgb1_dn7;
        locals.var_vgb1_dc_dn8 = locals.var_vgb1_dn8;
        locals.var_vgb1_dc_dn9 = locals.var_vgb1_dn9;
        locals.var_vgb1_dc_rv = 0.0;

        locals.var_vsbx_dc = locals.var_vsbx;
        locals.var_vsbx_dc_dn4 = locals.var_vsbx_dn4;
        locals.var_vsbx_dc_dn6 = locals.var_vsbx_dn6;
        locals.var_vsbx_dc_dn7 = locals.var_vsbx_dn7;
        locals.var_vsbx_dc_dn8 = locals.var_vsbx_dn8;
        locals.var_vsbx_dc_dn9 = locals.var_vsbx_dn9;
        locals.var_vsbx_dc_rv = 0.0;

        locals.var_phit1_dc = locals.var_phit1;
        locals.var_phit1_dc_dn4 = locals.var_phit1_dn4;
        locals.var_phit1_dc_dn6 = locals.var_phit1_dn6;
        locals.var_phit1_dc_dn7 = locals.var_phit1_dn7;
        locals.var_phit1_dc_dn8 = locals.var_phit1_dn8;
        locals.var_phit1_dc_dn9 = locals.var_phit1_dn9;
        locals.var_phit1_dc_rv = 0.0;

        locals.var_inv_phit1_dc = locals.var_inv_phit1;
        locals.var_inv_phit1_dc_dn4 = locals.var_inv_phit1_dn4;
        locals.var_inv_phit1_dc_dn6 = locals.var_inv_phit1_dn6;
        locals.var_inv_phit1_dc_dn7 = locals.var_inv_phit1_dn7;
        locals.var_inv_phit1_dc_dn8 = locals.var_inv_phit1_dn8;
        locals.var_inv_phit1_dc_dn9 = locals.var_inv_phit1_dn9;
        locals.var_inv_phit1_dc_rv = 0.0;

        locals.var_gf_dc = locals.var_gf;
        locals.var_gf_dc_dn4 = locals.var_gf_dn4;
        locals.var_gf_dc_dn6 = locals.var_gf_dn6;
        locals.var_gf_dc_dn7 = locals.var_gf_dn7;
        locals.var_gf_dc_dn8 = locals.var_gf_dn8;
        locals.var_gf_dc_dn9 = locals.var_gf_dn9;
        locals.var_gf_dc_rv = 0.0;

        locals.var_gf2_dc = locals.var_gf2;
        locals.var_gf2_dc_dn4 = locals.var_gf2_dn4;
        locals.var_gf2_dc_dn6 = locals.var_gf2_dn6;
        locals.var_gf2_dc_dn7 = locals.var_gf2_dn7;
        locals.var_gf2_dc_dn8 = locals.var_gf2_dn8;
        locals.var_gf2_dc_dn9 = locals.var_gf2_dn9;
        locals.var_gf2_dc_rv = 0.0;

        locals.var_inv_gf2_dc = locals.var_inv_gf2;
        locals.var_inv_gf2_dc_dn4 = locals.var_inv_gf2_dn4;
        locals.var_inv_gf2_dc_dn6 = locals.var_inv_gf2_dn6;
        locals.var_inv_gf2_dc_dn7 = locals.var_inv_gf2_dn7;
        locals.var_inv_gf2_dc_dn8 = locals.var_inv_gf2_dn8;
        locals.var_inv_gf2_dc_dn9 = locals.var_inv_gf2_dn9;
        locals.var_inv_gf2_dc_rv = 0.0;

        locals.var_xg_dc = locals.var_xg;
        locals.var_xg_dc_dn4 = locals.var_xg_dn4;
        locals.var_xg_dc_dn6 = locals.var_xg_dn6;
        locals.var_xg_dc_dn7 = locals.var_xg_dn7;
        locals.var_xg_dc_dn8 = locals.var_xg_dn8;
        locals.var_xg_dc_dn9 = locals.var_xg_dn9;
        locals.var_xg_dc_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_23(
        locals: &mut StampLocals,
    ) {
        locals.var_xno_s_dc = locals.var_xno_s;
        locals.var_xno_s_dc_dn4 = locals.var_xno_s_dn4;
        locals.var_xno_s_dc_dn6 = locals.var_xno_s_dn6;
        locals.var_xno_s_dc_dn7 = locals.var_xno_s_dn7;
        locals.var_xno_s_dc_dn8 = locals.var_xno_s_dn8;
        locals.var_xno_s_dc_dn9 = locals.var_xno_s_dn9;
        locals.var_xno_s_dc_rv = 0.0;

        locals.var_xn_s_dc = locals.var_xn_s;
        locals.var_xn_s_dc_dn4 = locals.var_xn_s_dn4;
        locals.var_xn_s_dc_dn6 = locals.var_xn_s_dn6;
        locals.var_xn_s_dc_dn7 = locals.var_xn_s_dn7;
        locals.var_xn_s_dc_dn8 = locals.var_xn_s_dn8;
        locals.var_xn_s_dc_dn9 = locals.var_xn_s_dn9;
        locals.var_xn_s_dc_rv = 0.0;

        locals.var_xi_dc = locals.var_xi;
        locals.var_xi_dc_dn4 = locals.var_xi_dn4;
        locals.var_xi_dc_dn6 = locals.var_xi_dn6;
        locals.var_xi_dc_dn7 = locals.var_xi_dn7;
        locals.var_xi_dc_dn8 = locals.var_xi_dn8;
        locals.var_xi_dc_dn9 = locals.var_xi_dn9;
        locals.var_xi_dc_rv = 0.0;

        locals.var_margin_dc = locals.var_margin;
        locals.var_margin_dc_dn4 = locals.var_margin_dn4;
        locals.var_margin_dc_dn6 = locals.var_margin_dn6;
        locals.var_margin_dc_dn7 = locals.var_margin_dn7;
        locals.var_margin_dc_dn8 = locals.var_margin_dn8;
        locals.var_margin_dc_dn9 = locals.var_margin_dn9;
        locals.var_margin_dc_rv = 0.0;

        locals.var_inv_xi_dc = locals.var_inv_xi;
        locals.var_inv_xi_dc_dn4 = locals.var_inv_xi_dn4;
        locals.var_inv_xi_dc_dn6 = locals.var_inv_xi_dn6;
        locals.var_inv_xi_dc_dn7 = locals.var_inv_xi_dn7;
        locals.var_inv_xi_dc_dn8 = locals.var_inv_xi_dn8;
        locals.var_inv_xi_dc_dn9 = locals.var_inv_xi_dn9;
        locals.var_inv_xi_dc_rv = 0.0;

        locals.var_sp_s_x1_dc = locals.var_sp_s_x1;
        locals.var_sp_s_x1_dc_dn4 = locals.var_sp_s_x1_dn4;
        locals.var_sp_s_x1_dc_dn6 = locals.var_sp_s_x1_dn6;
        locals.var_sp_s_x1_dc_dn7 = locals.var_sp_s_x1_dn7;
        locals.var_sp_s_x1_dc_dn8 = locals.var_sp_s_x1_dn8;
        locals.var_sp_s_x1_dc_dn9 = locals.var_sp_s_x1_dn9;
        locals.var_sp_s_x1_dc_rv = 0.0;

        locals.var_delta_ns_dc = locals.var_delta_ns;
        locals.var_delta_ns_dc_dn4 = locals.var_delta_ns_dn4;
        locals.var_delta_ns_dc_dn6 = locals.var_delta_ns_dn6;
        locals.var_delta_ns_dc_dn7 = locals.var_delta_ns_dn7;
        locals.var_delta_ns_dc_dn8 = locals.var_delta_ns_dn8;
        locals.var_delta_ns_dc_dn9 = locals.var_delta_ns_dn9;
        locals.var_delta_ns_dc_rv = 0.0;

        locals.var_x_s_dc = locals.var_x_s;
        locals.var_x_s_dc_dn4 = locals.var_x_s_dn4;
        locals.var_x_s_dc_dn6 = locals.var_x_s_dn6;
        locals.var_x_s_dc_dn7 = locals.var_x_s_dn7;
        locals.var_x_s_dc_dn8 = locals.var_x_s_dn8;
        locals.var_x_s_dc_dn9 = locals.var_x_s_dn9;
        locals.var_x_s_dc_rv = 0.0;

        locals.var_xi1s_dc = locals.var_xi1s;
        locals.var_xi1s_dc_dn4 = locals.var_xi1s_dn4;
        locals.var_xi1s_dc_dn6 = locals.var_xi1s_dn6;
        locals.var_xi1s_dc_dn7 = locals.var_xi1s_dn7;
        locals.var_xi1s_dc_dn8 = locals.var_xi1s_dn8;
        locals.var_xi1s_dc_dn9 = locals.var_xi1s_dn9;
        locals.var_xi1s_dc_rv = 0.0;

        locals.var_xi2s_dc = locals.var_xi2s;
        locals.var_xi2s_dc_dn4 = locals.var_xi2s_dn4;
        locals.var_xi2s_dc_dn6 = locals.var_xi2s_dn6;
        locals.var_xi2s_dc_dn7 = locals.var_xi2s_dn7;
        locals.var_xi2s_dc_dn8 = locals.var_xi2s_dn8;
        locals.var_xi2s_dc_dn9 = locals.var_xi2s_dn9;
        locals.var_xi2s_dc_rv = 0.0;

        locals.var_delta_1s_dc = locals.var_delta_1s;
        locals.var_delta_1s_dc_dn4 = locals.var_delta_1s_dn4;
        locals.var_delta_1s_dc_dn6 = locals.var_delta_1s_dn6;
        locals.var_delta_1s_dc_dn7 = locals.var_delta_1s_dn7;
        locals.var_delta_1s_dc_dn8 = locals.var_delta_1s_dn8;
        locals.var_delta_1s_dc_dn9 = locals.var_delta_1s_dn9;
        locals.var_delta_1s_dc_rv = 0.0;

        locals.var_es_dc = locals.var_es;
        locals.var_es_dc_dn4 = locals.var_es_dn4;
        locals.var_es_dc_dn6 = locals.var_es_dn6;
        locals.var_es_dc_dn7 = locals.var_es_dn7;
        locals.var_es_dc_dn8 = locals.var_es_dn8;
        locals.var_es_dc_dn9 = locals.var_es_dn9;
        locals.var_es_dc_rv = 0.0;

        locals.var_ps_dc = locals.var_ps;
        locals.var_ps_dc_dn4 = locals.var_ps_dn4;
        locals.var_ps_dc_dn6 = locals.var_ps_dn6;
        locals.var_ps_dc_dn7 = locals.var_ps_dn7;
        locals.var_ps_dc_dn8 = locals.var_ps_dn8;
        locals.var_ps_dc_dn9 = locals.var_ps_dn9;
        locals.var_ps_dc_rv = 0.0;

        locals.var_ds_dc = locals.var_ds;
        locals.var_ds_dc_dn4 = locals.var_ds_dn4;
        locals.var_ds_dc_dn6 = locals.var_ds_dn6;
        locals.var_ds_dc_dn7 = locals.var_ds_dn7;
        locals.var_ds_dc_dn8 = locals.var_ds_dn8;
        locals.var_ds_dc_dn9 = locals.var_ds_dn9;
        locals.var_ds_dc_rv = 0.0;

        locals.var_sqs_dc = locals.var_sqs;
        locals.var_sqs_dc_dn4 = locals.var_sqs_dn4;
        locals.var_sqs_dc_dn6 = locals.var_sqs_dn6;
        locals.var_sqs_dc_dn7 = locals.var_sqs_dn7;
        locals.var_sqs_dc_dn8 = locals.var_sqs_dn8;
        locals.var_sqs_dc_dn9 = locals.var_sqs_dn9;
        locals.var_sqs_dc_rv = 0.0;

        locals.var_alphas_dc = locals.var_alphas;
        locals.var_alphas_dc_dn4 = locals.var_alphas_dn4;
        locals.var_alphas_dc_dn6 = locals.var_alphas_dn6;
        locals.var_alphas_dc_dn7 = locals.var_alphas_dn7;
        locals.var_alphas_dc_dn8 = locals.var_alphas_dn8;
        locals.var_alphas_dc_dn9 = locals.var_alphas_dn9;
        locals.var_alphas_dc_rv = 0.0;

        locals.var_rxcor_dc = locals.var_rxcor;
        locals.var_rxcor_dc_dn4 = locals.var_rxcor_dn4;
        locals.var_rxcor_dc_dn6 = locals.var_rxcor_dn6;
        locals.var_rxcor_dc_dn7 = locals.var_rxcor_dn7;
        locals.var_rxcor_dc_dn8 = locals.var_rxcor_dn8;
        locals.var_rxcor_dc_dn9 = locals.var_rxcor_dn9;
        locals.var_rxcor_dc_rv = 0.0;

        locals.var_xgs_dc = locals.var_xgs;
        locals.var_xgs_dc_dn4 = locals.var_xgs_dn4;
        locals.var_xgs_dc_dn6 = locals.var_xgs_dn6;
        locals.var_xgs_dc_dn7 = locals.var_xgs_dn7;
        locals.var_xgs_dc_dn8 = locals.var_xgs_dn8;
        locals.var_xgs_dc_dn9 = locals.var_xgs_dn9;
        locals.var_xgs_dc_rv = 0.0;

        locals.var_qis_dc = locals.var_qis;
        locals.var_qis_dc_dn4 = locals.var_qis_dn4;
        locals.var_qis_dc_dn6 = locals.var_qis_dn6;
        locals.var_qis_dc_dn7 = locals.var_qis_dn7;
        locals.var_qis_dc_dn8 = locals.var_qis_dn8;
        locals.var_qis_dc_dn9 = locals.var_qis_dn9;
        locals.var_qis_dc_rv = 0.0;

        locals.var_qbs_dc = locals.var_qbs;
        locals.var_qbs_dc_dn4 = locals.var_qbs_dn4;
        locals.var_qbs_dc_dn6 = locals.var_qbs_dn6;
        locals.var_qbs_dc_dn7 = locals.var_qbs_dn7;
        locals.var_qbs_dc_dn8 = locals.var_qbs_dn8;
        locals.var_qbs_dc_dn9 = locals.var_qbs_dn9;
        locals.var_qbs_dc_rv = 0.0;

        locals.var_rhob_dc = locals.var_rhob;
        locals.var_rhob_dc_dn4 = locals.var_rhob_dn4;
        locals.var_rhob_dc_dn6 = locals.var_rhob_dn6;
        locals.var_rhob_dc_dn7 = locals.var_rhob_dn7;
        locals.var_rhob_dc_dn8 = locals.var_rhob_dn8;
        locals.var_rhob_dc_dn9 = locals.var_rhob_dn9;
        locals.var_rhob_dc_rv = 0.0;

        locals.var_rhog_dc = locals.var_rhog;
        locals.var_rhog_dc_dn4 = locals.var_rhog_dn4;
        locals.var_rhog_dc_dn6 = locals.var_rhog_dn6;
        locals.var_rhog_dc_dn7 = locals.var_rhog_dn7;
        locals.var_rhog_dc_dn8 = locals.var_rhog_dn8;
        locals.var_rhog_dc_dn9 = locals.var_rhog_dn9;
        locals.var_rhog_dc_rv = 0.0;

        locals.var_gmobs_dc = locals.var_gmobs;
        locals.var_gmobs_dc_dn4 = locals.var_gmobs_dn4;
        locals.var_gmobs_dc_dn6 = locals.var_gmobs_dn6;
        locals.var_gmobs_dc_dn7 = locals.var_gmobs_dn7;
        locals.var_gmobs_dc_dn8 = locals.var_gmobs_dn8;
        locals.var_gmobs_dc_dn9 = locals.var_gmobs_dn9;
        locals.var_gmobs_dc_rv = 0.0;

        locals.var_xitsb_dc = locals.var_xitsb;
        locals.var_xitsb_dc_dn4 = locals.var_xitsb_dn4;
        locals.var_xitsb_dc_dn6 = locals.var_xitsb_dn6;
        locals.var_xitsb_dc_dn7 = locals.var_xitsb_dn7;
        locals.var_xitsb_dc_dn8 = locals.var_xitsb_dn8;
        locals.var_xitsb_dc_dn9 = locals.var_xitsb_dn9;
        locals.var_xitsb_dc_rv = 0.0;

        locals.var_factheta_dc = locals.var_factheta;
        locals.var_factheta_dc_dn4 = locals.var_factheta_dn4;
        locals.var_factheta_dc_dn6 = locals.var_factheta_dn6;
        locals.var_factheta_dc_dn7 = locals.var_factheta_dn7;
        locals.var_factheta_dc_dn8 = locals.var_factheta_dn8;
        locals.var_factheta_dc_dn9 = locals.var_factheta_dn9;
        locals.var_factheta_dc_rv = 0.0;

        locals.var_thesat1 = 0.0;
        locals.var_thesat1_dn4 = 0.0;
        locals.var_thesat1_dn6 = 0.0;
        locals.var_thesat1_dn7 = 0.0;
        locals.var_thesat1_dn8 = 0.0;
        locals.var_thesat1_dn9 = 0.0;
        locals.var_thesat1_rv = 0.0;

        let assign43480_e56510: f64 = (locals.var_phit1 * 4.60517018598809);
        locals.var_vdsat_lim = assign43480_e56510;
        locals.var_vdsat_lim_dn4 = (locals.var_phit1_dn4 * 4.60517018598809);
        locals.var_vdsat_lim_dn6 = (locals.var_phit1_dn6 * 4.60517018598809);
        locals.var_vdsat_lim_dn7 = (locals.var_phit1_dn7 * 4.60517018598809);
        locals.var_vdsat_lim_dn8 = (locals.var_phit1_dn8 * 4.60517018598809);
        locals.var_vdsat_lim_dn9 = (locals.var_phit1_dn9 * 4.60517018598809);
        locals.var_vdsat_lim_rv = 0.0;

        locals.var_v_dsat = locals.var_vdsat_lim;
        locals.var_v_dsat_dn4 = locals.var_vdsat_lim_dn4;
        locals.var_v_dsat_dn6 = locals.var_vdsat_lim_dn6;
        locals.var_v_dsat_dn7 = locals.var_vdsat_lim_dn7;
        locals.var_v_dsat_dn8 = locals.var_vdsat_lim_dn8;
        locals.var_v_dsat_dn9 = locals.var_vdsat_lim_dn9;
        locals.var_v_dsat_rv = 0.0;

        locals.var_vdse = locals.var_v_ds;
        locals.var_vdse_dn4 = 0.0;
        locals.var_vdse_dn6 = 0.0;
        locals.var_vdse_dn7 = locals.var_v_ds_dn7;
        locals.var_vdse_dn8 = locals.var_v_ds_dn8;
        locals.var_vdse_dn9 = 0.0;
        locals.var_vdse_rv = 0.0;

        let assign43510_e56515: f64 = (locals.var_v_ds * locals.var_inv_phit1);
        locals.var_udse = assign43510_e56515;
        locals.var_udse_dn4 = (locals.var_v_ds * locals.var_inv_phit1_dn4);
        locals.var_udse_dn6 = (locals.var_v_ds * locals.var_inv_phit1_dn6);
        locals.var_udse_dn7 = ((locals.var_v_ds_dn7 * locals.var_inv_phit1) + (locals.var_v_ds * locals.var_inv_phit1_dn7));
        locals.var_udse_dn8 = ((locals.var_v_ds_dn8 * locals.var_inv_phit1) + (locals.var_v_ds * locals.var_inv_phit1_dn8));
        locals.var_udse_dn9 = (locals.var_v_ds * locals.var_inv_phit1_dn9);
        locals.var_udse_rv = 0.0;

        locals.var_x_d = locals.var_x_s;
        locals.var_x_d_dn4 = locals.var_x_s_dn4;
        locals.var_x_d_dn6 = locals.var_x_s_dn6;
        locals.var_x_d_dn7 = locals.var_x_s_dn7;
        locals.var_x_d_dn8 = locals.var_x_s_dn8;
        locals.var_x_d_dn9 = locals.var_x_s_dn9;
        locals.var_x_d_rv = 0.0;

        locals.var_x_ds = 0.0;
        locals.var_x_ds_dn4 = 0.0;
        locals.var_x_ds_dn6 = 0.0;
        locals.var_x_ds_dn7 = 0.0;
        locals.var_x_ds_dn8 = 0.0;
        locals.var_x_ds_dn9 = 0.0;
        locals.var_x_ds_rv = 0.0;

        locals.var_dps = 0.0;
        locals.var_dps_dn4 = 0.0;
        locals.var_dps_dn6 = 0.0;
        locals.var_dps_dn7 = 0.0;
        locals.var_dps_dn8 = 0.0;
        locals.var_dps_dn9 = 0.0;
        locals.var_dps_rv = 0.0;

        locals.var_ed = locals.var_es;
        locals.var_ed_dn4 = locals.var_es_dn4;
        locals.var_ed_dn6 = locals.var_es_dn6;
        locals.var_ed_dn7 = locals.var_es_dn7;
        locals.var_ed_dn8 = locals.var_es_dn8;
        locals.var_ed_dn9 = locals.var_es_dn9;
        locals.var_ed_rv = 0.0;

        locals.var_pd = locals.var_ps;
        locals.var_pd_dn4 = locals.var_ps_dn4;
        locals.var_pd_dn6 = locals.var_ps_dn6;
        locals.var_pd_dn7 = locals.var_ps_dn7;
        locals.var_pd_dn8 = locals.var_ps_dn8;
        locals.var_pd_dn9 = locals.var_ps_dn9;
        locals.var_pd_rv = 0.0;

        locals.var_dd = locals.var_ds;
        locals.var_dd_dn4 = locals.var_ds_dn4;
        locals.var_dd_dn6 = locals.var_ds_dn6;
        locals.var_dd_dn7 = locals.var_ds_dn7;
        locals.var_dd_dn8 = locals.var_ds_dn8;
        locals.var_dd_dn9 = locals.var_ds_dn9;
        locals.var_dd_rv = 0.0;

        locals.var_qbd = locals.var_qbs;
        locals.var_qbd_dn4 = locals.var_qbs_dn4;
        locals.var_qbd_dn6 = locals.var_qbs_dn6;
        locals.var_qbd_dn7 = locals.var_qbs_dn7;
        locals.var_qbd_dn8 = locals.var_qbs_dn8;
        locals.var_qbd_dn9 = locals.var_qbs_dn9;
        locals.var_qbd_rv = 0.0;

        locals.var_x_m = locals.var_x_s;
        locals.var_x_m_dn4 = locals.var_x_s_dn4;
        locals.var_x_m_dn6 = locals.var_x_s_dn6;
        locals.var_x_m_dn7 = locals.var_x_s_dn7;
        locals.var_x_m_dn8 = locals.var_x_s_dn8;
        locals.var_x_m_dn9 = locals.var_x_s_dn9;
        locals.var_x_m_rv = 0.0;

        locals.var_em = locals.var_es;
        locals.var_em_dn4 = locals.var_es_dn4;
        locals.var_em_dn6 = locals.var_es_dn6;
        locals.var_em_dn7 = locals.var_es_dn7;
        locals.var_em_dn8 = locals.var_es_dn8;
        locals.var_em_dn9 = locals.var_es_dn9;
        locals.var_em_rv = 0.0;

        locals.var_dm = locals.var_ds;
        locals.var_dm_dn4 = locals.var_ds_dn4;
        locals.var_dm_dn6 = locals.var_ds_dn6;
        locals.var_dm_dn7 = locals.var_ds_dn7;
        locals.var_dm_dn8 = locals.var_ds_dn8;
        locals.var_dm_dn9 = locals.var_ds_dn9;
        locals.var_dm_rv = 0.0;

        locals.var_pm = locals.var_ps;
        locals.var_pm_dn4 = locals.var_ps_dn4;
        locals.var_pm_dn6 = locals.var_ps_dn6;
        locals.var_pm_dn7 = locals.var_ps_dn7;
        locals.var_pm_dn8 = locals.var_ps_dn8;
        locals.var_pm_dn9 = locals.var_ps_dn9;
        locals.var_pm_rv = 0.0;

        let assign43630_e56529: f64 = (locals.var_xg - locals.var_x_s);
        locals.var_xgm = assign43630_e56529;
        locals.var_xgm_dn4 = (locals.var_xg_dn4 - locals.var_x_s_dn4);
        locals.var_xgm_dn6 = (locals.var_xg_dn6 - locals.var_x_s_dn6);
        locals.var_xgm_dn7 = (locals.var_xg_dn7 - locals.var_x_s_dn7);
        locals.var_xgm_dn8 = (locals.var_xg_dn8 - locals.var_x_s_dn8);
        locals.var_xgm_dn9 = (locals.var_xg_dn9 - locals.var_x_s_dn9);
        locals.var_xgm_rv = 0.0;

        locals.var_eta_p = 1.0;
        locals.var_eta_p_dn4 = 0.0;
        locals.var_eta_p_dn6 = 0.0;
        locals.var_eta_p_dn7 = 0.0;
        locals.var_eta_p_dn8 = 0.0;
        locals.var_eta_p_dn9 = 0.0;
        locals.var_eta_p_rv = 0.0;

        locals.var_alpha = 1.0;
        locals.var_alpha_dn4 = 0.0;
        locals.var_alpha_dn6 = 0.0;
        locals.var_alpha_dn7 = 0.0;
        locals.var_alpha_dn8 = 0.0;
        locals.var_alpha_dn9 = 0.0;
        locals.var_alpha_rv = 0.0;

        locals.var_sqm = 0.0;
        locals.var_sqm_dn4 = 0.0;
        locals.var_sqm_dn6 = 0.0;
        locals.var_sqm_dn7 = 0.0;
        locals.var_sqm_dn8 = 0.0;
        locals.var_sqm_dn9 = 0.0;
        locals.var_sqm_rv = 0.0;

        locals.var_qim = locals.var_qis;
        locals.var_qim_dn4 = locals.var_qis_dn4;
        locals.var_qim_dn6 = locals.var_qis_dn6;
        locals.var_qim_dn7 = locals.var_qis_dn7;
        locals.var_qim_dn8 = locals.var_qis_dn8;
        locals.var_qim_dn9 = locals.var_qis_dn9;
        locals.var_qim_rv = 0.0;

        let assign43680_e56536: f64 = (locals.var_xgm * locals.var_phit1);
        locals.var_qeff1 = assign43680_e56536;
        locals.var_qeff1_dn4 = ((locals.var_xgm_dn4 * locals.var_phit1) + (locals.var_xgm * locals.var_phit1_dn4));
        locals.var_qeff1_dn6 = ((locals.var_xgm_dn6 * locals.var_phit1) + (locals.var_xgm * locals.var_phit1_dn6));
        locals.var_qeff1_dn7 = ((locals.var_xgm_dn7 * locals.var_phit1) + (locals.var_xgm * locals.var_phit1_dn7));
        locals.var_qeff1_dn8 = ((locals.var_xgm_dn8 * locals.var_phit1) + (locals.var_xgm * locals.var_phit1_dn8));
        locals.var_qeff1_dn9 = ((locals.var_xgm_dn9 * locals.var_phit1) + (locals.var_xgm * locals.var_phit1_dn9));
        locals.var_qeff1_rv = 0.0;

        locals.var_qim1 = 0.0;
        locals.var_qim1_dn4 = 0.0;
        locals.var_qim1_dn6 = 0.0;
        locals.var_qim1_dn7 = 0.0;
        locals.var_qim1_dn8 = 0.0;
        locals.var_qim1_dn9 = 0.0;
        locals.var_qim1_rv = 0.0;

        locals.var_qbm = locals.var_qbs;
        locals.var_qbm_dn4 = locals.var_qbs_dn4;
        locals.var_qbm_dn6 = locals.var_qbs_dn6;
        locals.var_qbm_dn7 = locals.var_qbs_dn7;
        locals.var_qbm_dn8 = locals.var_qbs_dn8;
        locals.var_qbm_dn9 = locals.var_qbs_dn9;
        locals.var_qbm_rv = 0.0;

        locals.var_s1 = 0.0;
        locals.var_s1_dn4 = 0.0;
        locals.var_s1_dn6 = 0.0;
        locals.var_s1_dn7 = 0.0;
        locals.var_s1_dn8 = 0.0;
        locals.var_s1_dn9 = 0.0;
        locals.var_s1_rv = 0.0;

        locals.var_gmob = 1.0;
        locals.var_gmob_dn4 = 0.0;
        locals.var_gmob_dn6 = 0.0;
        locals.var_gmob_dn7 = 0.0;
        locals.var_gmob_dn8 = 0.0;
        locals.var_gmob_dn9 = 0.0;
        locals.var_gmob_rv = 0.0;

        locals.var_thesateff = locals.var_thesatloc;
        locals.var_thesateff_dn4 = locals.var_thesatloc_dn4;
        locals.var_thesateff_dn6 = 0.0;
        locals.var_thesateff_dn7 = 0.0;
        locals.var_thesateff_dn8 = 0.0;
        locals.var_thesateff_dn9 = 0.0;
        locals.var_thesateff_rv = 0.0;

        locals.var_voxm = locals.var_qeff1;
        locals.var_voxm_dn4 = locals.var_qeff1_dn4;
        locals.var_voxm_dn6 = locals.var_qeff1_dn6;
        locals.var_voxm_dn7 = locals.var_qeff1_dn7;
        locals.var_voxm_dn8 = locals.var_qeff1_dn8;
        locals.var_voxm_dn9 = locals.var_qeff1_dn9;
        locals.var_voxm_rv = 0.0;

        let assign43750_e56545: f64 = if locals.var_xg > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1214 = assign43750_e56545;
        locals.var_guard1214_rv = 0.0;

        let assign43760_e56548: f64 = if locals.var_ds > 1e-100 { 1.0 } else { 0.0 };
        locals.var_guard1215 = assign43760_e56548;
        locals.var_guard1215_rv = 0.0;

        let (assign43770_e56556, assign43770_e56556_d_n4, assign43770_e56556_d_n6, assign43770_e56556_d_n7, assign43770_e56556_d_n8, assign43770_e56556_d_n9,) = {
    if ((locals.var_guard1214 != 0.0) && (locals.var_guard1215 != 0.0)) {
        let assign43770_e56554: f64 = (locals.var_thesatloc * locals.var_factheta);
        (assign43770_e56554, ((locals.var_thesatloc_dn4 * locals.var_factheta) + (locals.var_thesatloc * locals.var_factheta_dn4)), (locals.var_thesatloc * locals.var_factheta_dn6), (locals.var_thesatloc * locals.var_factheta_dn7), (locals.var_thesatloc * locals.var_factheta_dn8), (locals.var_thesatloc * locals.var_factheta_dn9),)
    } else {
        (locals.var_thesateff, locals.var_thesateff_dn4, locals.var_thesateff_dn6, locals.var_thesateff_dn7, locals.var_thesateff_dn8, locals.var_thesateff_dn9,)
    }
};
        locals.var_thesateff = assign43770_e56556;
        locals.var_thesateff_dn4 = assign43770_e56556_d_n4;
        locals.var_thesateff_dn6 = assign43770_e56556_d_n6;
        locals.var_thesateff_dn7 = assign43770_e56556_d_n7;
        locals.var_thesateff_dn8 = assign43770_e56556_d_n8;
        locals.var_thesateff_dn9 = assign43770_e56556_d_n9;
        locals.var_thesateff_rv = 0.0;

        let (assign43780_e56564, assign43780_e56564_d_n4, assign43780_e56564_d_n6, assign43780_e56564_d_n7, assign43780_e56564_d_n8, assign43780_e56564_d_n9,) = {
    if ((locals.var_guard1214 != 0.0) && (locals.var_guard1215 != 0.0)) {
        let assign43780_e56562: f64 = (locals.var_thesateff / locals.var_gmobs);
        (assign43780_e56562, (((locals.var_thesateff_dn4 * locals.var_gmobs) - (locals.var_thesateff * locals.var_gmobs_dn4)) / (locals.var_gmobs * locals.var_gmobs)), (((locals.var_thesateff_dn6 * locals.var_gmobs) - (locals.var_thesateff * locals.var_gmobs_dn6)) / (locals.var_gmobs * locals.var_gmobs)), (((locals.var_thesateff_dn7 * locals.var_gmobs) - (locals.var_thesateff * locals.var_gmobs_dn7)) / (locals.var_gmobs * locals.var_gmobs)), (((locals.var_thesateff_dn8 * locals.var_gmobs) - (locals.var_thesateff * locals.var_gmobs_dn8)) / (locals.var_gmobs * locals.var_gmobs)), (((locals.var_thesateff_dn9 * locals.var_gmobs) - (locals.var_thesateff * locals.var_gmobs_dn9)) / (locals.var_gmobs * locals.var_gmobs)),)
    } else {
        (locals.var_thesat1, locals.var_thesat1_dn4, locals.var_thesat1_dn6, locals.var_thesat1_dn7, locals.var_thesat1_dn8, locals.var_thesat1_dn9,)
    }
};
        locals.var_thesat1 = assign43780_e56564;
        locals.var_thesat1_dn4 = assign43780_e56564_d_n4;
        locals.var_thesat1_dn6 = assign43780_e56564_d_n6;
        locals.var_thesat1_dn7 = assign43780_e56564_d_n7;
        locals.var_thesat1_dn8 = assign43780_e56564_d_n8;
        locals.var_thesat1_dn9 = assign43780_e56564_d_n9;
        locals.var_thesat1_rv = 0.0;

        let (assign43790_e56574, assign43790_e56574_d_n4, assign43790_e56574_d_n6, assign43790_e56574_d_n7, assign43790_e56574_d_n8, assign43790_e56574_d_n9,) = {
    if ((locals.var_guard1214 != 0.0) && (locals.var_guard1215 != 0.0)) {
        let assign43790_e56571: f64 = (0.5 * locals.var_gf2);
        let assign43790_e56572: f64 = (locals.var_xgs + assign43790_e56571);
        (assign43790_e56572, (locals.var_xgs_dn4 + (0.5 * locals.var_gf2_dn4)), (locals.var_xgs_dn6 + (0.5 * locals.var_gf2_dn6)), (locals.var_xgs_dn7 + (0.5 * locals.var_gf2_dn7)), (locals.var_xgs_dn8 + (0.5 * locals.var_gf2_dn8)), (locals.var_xgs_dn9 + (0.5 * locals.var_gf2_dn9)),)
    } else {
        (locals.var_asat, locals.var_asat_dn4, locals.var_asat_dn6, locals.var_asat_dn7, locals.var_asat_dn8, locals.var_asat_dn9,)
    }
};
        locals.var_asat = assign43790_e56574;
        locals.var_asat_dn4 = assign43790_e56574_d_n4;
        locals.var_asat_dn6 = assign43790_e56574_d_n6;
        locals.var_asat_dn7 = assign43790_e56574_d_n7;
        locals.var_asat_dn8 = assign43790_e56574_d_n8;
        locals.var_asat_dn9 = assign43790_e56574_d_n9;
        locals.var_asat_rv = 0.0;

        let (assign43800_e56586, assign43800_e56586_d_n4, assign43800_e56586_d_n6, assign43800_e56586_d_n7, assign43800_e56586_d_n8, assign43800_e56586_d_n9,) = {
    if ((locals.var_guard1214 != 0.0) && (locals.var_guard1215 != 0.0)) {
        let assign43800_e56580: f64 = (locals.var_gf2 * locals.var_delta_1s);
        let __rspice_inv_cse_0: f64 = 1.0 / locals.var_asat;
        let assign43800_e56582: f64 = (assign43800_e56580 * __rspice_inv_cse_0);
        let assign43800_e56584: f64 = (assign43800_e56582 * __rspice_inv_cse_0);
        (assign43800_e56584, ((((((((locals.var_gf2_dn4 * locals.var_delta_1s) + (locals.var_gf2 * locals.var_delta_1s_dn4)) * locals.var_asat) - (assign43800_e56580 * locals.var_asat_dn4)) / (locals.var_asat * locals.var_asat)) * locals.var_asat) - (assign43800_e56582 * locals.var_asat_dn4)) / (locals.var_asat * locals.var_asat)), ((((((((locals.var_gf2_dn6 * locals.var_delta_1s) + (locals.var_gf2 * locals.var_delta_1s_dn6)) * locals.var_asat) - (assign43800_e56580 * locals.var_asat_dn6)) / (locals.var_asat * locals.var_asat)) * locals.var_asat) - (assign43800_e56582 * locals.var_asat_dn6)) / (locals.var_asat * locals.var_asat)), ((((((((locals.var_gf2_dn7 * locals.var_delta_1s) + (locals.var_gf2 * locals.var_delta_1s_dn7)) * locals.var_asat) - (assign43800_e56580 * locals.var_asat_dn7)) / (locals.var_asat * locals.var_asat)) * locals.var_asat) - (assign43800_e56582 * locals.var_asat_dn7)) / (locals.var_asat * locals.var_asat)), ((((((((locals.var_gf2_dn8 * locals.var_delta_1s) + (locals.var_gf2 * locals.var_delta_1s_dn8)) * locals.var_asat) - (assign43800_e56580 * locals.var_asat_dn8)) / (locals.var_asat * locals.var_asat)) * locals.var_asat) - (assign43800_e56582 * locals.var_asat_dn8)) / (locals.var_asat * locals.var_asat)), ((((((((locals.var_gf2_dn9 * locals.var_delta_1s) + (locals.var_gf2 * locals.var_delta_1s_dn9)) * locals.var_asat) - (assign43800_e56580 * locals.var_asat_dn9)) / (locals.var_asat * locals.var_asat)) * locals.var_asat) - (assign43800_e56582 * locals.var_asat_dn9)) / (locals.var_asat * locals.var_asat)),)
    } else {
        (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
    }
};
        locals.var_temp__blk949 = assign43800_e56586;
        locals.var_temp__blk949_dn4 = assign43800_e56586_d_n4;
        locals.var_temp__blk949_dn6 = assign43800_e56586_d_n6;
        locals.var_temp__blk949_dn7 = assign43800_e56586_d_n7;
        locals.var_temp__blk949_dn8 = assign43800_e56586_d_n8;
        locals.var_temp__blk949_dn9 = assign43800_e56586_d_n9;
        locals.var_temp__blk949_rv = 0.0;

        let assign43810_e56589: f64 = if locals.var_temp__blk949 > 0.0001 { 1.0 } else { 0.0 };
        locals.var_guard1216 = assign43810_e56589;
        locals.var_guard1216_rv = 0.0;

    }
}
